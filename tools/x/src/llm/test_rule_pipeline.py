import json
from pathlib import Path
from tempfile import TemporaryDirectory
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

from cli.cli import build_parser
from cli.cmd.extract import _repo_root, _write_json
from cli.cmd.rule import _write_csv
from llm import rule_jobs as jobs, rule_pipeline as pipeline


def response(text):
    return {'response': {'status_code': 200, 'body': {'status': 'completed', 'output': [
        {'type': 'message', 'content': [{'type': 'output_text', 'text': text}]}]}}}


def job(status='completed', output='output', errors=None):
    return SimpleNamespace(id='batch-1', status=status, output_file_id=output,
        error_file_id=errors, model_dump=lambda mode: {'status': status})


class DurableJobs(unittest.TestCase):
    def args(self, *extra):
        return build_parser().parse_args(['rule', '--output', 'out', 'core', *extra])

    def test_resume_step1_uses_saved_transport_model_and_prompt(self):
        from cli.cmd.extract import _source_items
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            document = {"files": {"rust/library/core/src/test.rs": [{"name": "f", "line_number_start": 1,
                "declaration_text": "unsafe fn f()", "parent_struct_or_trait": None, "comment": "Must be valid."}]}}
            _write_json(output / "step0.json", document)
            item = _source_items(document)[0]
            key = 'api-' + jobs.fingerprint(item)[:24]
            request = {"model": "saved-model", "messages": [{"role": "system", "content": "saved prompt"}]}
            cache = output / "requests/saved"
            _write_json(output / "step1-input.json", {"mode": "batch", "model": "saved-model", "items": [item],
                "source_sha256": jobs.fingerprint(document), "requests_directory": str(cache)})
            _write_json(cache / "round-00/input.json", {"mode": "batch", "endpoint": "/v1/chat/completions", "requests": {key: request}})
            record = {"response": {"status_code": 200, "body": {"choices": [{"finish_reason": "stop",
                "message": {"content": '{"rules":["Must be valid."]}'}}]}}}
            args = self.args('--model', 'new-model', '--resume-step1-input')
            with patch.object(jobs, 'execute', return_value={key: record}) as execute, patch('llm.rule_context.index_for') as prepare:
                pipeline.extract(output, args)
            prepare.assert_not_called()
            self.assertEqual(execute.call_args.args[1], {key: request})
            self.assertEqual(execute.call_args.args[3], 'batch')
            result = json.loads((output / 'step1.json').read_text())[0]
            self.assertEqual((result['model'], result['mode']), ('saved-model', 'batch'))

    def test_resume_step1_rejects_source_changes_before_requests(self):
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            _write_json(output / 'step0.json', {'files': {}})
            _write_json(output / 'step1-input.json', {'source_sha256': 'different'})
            with patch.object(jobs, 'execute') as execute, self.assertRaisesRegex(RuntimeError, 'source changed'):
                pipeline.extract(output, self.args('--resume-step1-input'))
            execute.assert_not_called()

    def test_frozen_resume_preserves_per_api_repairs_without_retrying_old_errors(self):
        from cli.cmd.extract import _source_items
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            document = {"files": {"rust/library/core/src/test.rs": [{"name": "f", "line_number_start": 1,
                "declaration_text": "unsafe fn f()", "parent_struct_or_trait": None, "comment": "Must be valid."}]}}
            _write_json(output / 'step0.json', document)
            item = _source_items(document)[0]
            identifier = jobs.fingerprint(item)
            key = 'api-' + identifier[:24]
            cache = output / 'requests/base'
            request = {'model': 'base-model'}
            _write_json(output / 'step1-input.json', {'mode': 'batch', 'model': 'base-model', 'items': [item], 'requests_directory': str(cache)})
            _write_json(cache / 'round-00/input.json', {'mode': 'batch', 'requests': {key: request}})
            repair = {**item, 'model': 'repair-model', 'mode': 'llm', 'status': 'extracted', 'rules': ['Corrected contract.']}
            _write_json(output / 'step1-repairs.json', {'source_sha256': jobs.fingerprint(document),
                'repairs': {identifier: {'result': repair}}})
            with patch.object(jobs, 'execute', return_value={key: {'error': {'message': 'old failure'}}}) as execute:
                pipeline.extract(output, self.args('--resume-step1-input'))
            self.assertEqual(execute.call_count, 1)
            self.assertEqual(json.loads((output / 'step1.json').read_text()), [repair])

    def test_source_context_keeps_const_generic_default_braces(self):
        from dslapi import ApiInterface, split_body
        from dsltypes import TypeCatalog
        schema = _repo_root() / "tools/x/generated/dsl/schema.json"
        header = "trait TransmuteFrom<Src, const ASSUME: Assume = { Assume::NOTHING }>"
        declaration = "unsafe fn transmute(src: Src) -> Self"
        document = {"files": {"rust/library/core/src/mem/transmutability.rs": [
            {"name": "transmute", "line_number_start": 94, "declaration_text": declaration,
             "enclosing_declaration": header}]}}
        api, _ = pipeline.api_for({"id": "rule-1", "path": "rust/library/core/src/mem/transmutability.rs:94", "name": declaration}, document)
        self.assertEqual(split_body(api)[0].strip(), header)
        interface = ApiInterface(TypeCatalog(json.loads(schema.read_text())), api)
        self.assertEqual([v.source_name for v in interface.variables], ["src", "Self", "Src", "ASSUME", "result"])

    def test_resume_pending_job_and_match_unordered_results(self):
        with TemporaryDirectory() as temporary:
            api = Mock()
            api.batches.list.return_value = []
            api.files.create.return_value.id = 'file-input'
            api.batches.create.return_value = job('validating', None)
            api.batches.retrieve.side_effect = [job('in_progress', None), job()]
            api.files.content.return_value.text = '\n'.join(json.dumps({'custom_id': key, **response(key)}) for key in ('b', 'a'))
            args = self.args('--batch-no-wait')
            with patch.object(jobs, 'client', return_value=api):
                with self.assertRaises(jobs.BatchPending):
                    jobs.execute(Path(temporary), {'a': {}, 'b': {}}, '/v1/responses', 'batch', args)
                state = json.loads((Path(temporary)/'batch-0000/job.json').read_text())
                self.assertEqual(state['job_id'], 'batch-1')
                result = jobs.execute(Path(temporary), {'a': {}, 'b': {}}, '/v1/responses', 'batch', args)
                self.assertEqual(jobs.response_text(jobs.body(result['a']), '/v1/responses'), 'a')
                jobs.execute(Path(temporary), {'a': {}, 'b': {}}, '/v1/responses', 'batch', args)
            self.assertEqual(api.batches.create.call_count, 1)
            self.assertEqual(api.files.create.call_count, 1)
            self.assertEqual(api.batches.retrieve.call_count, 2)

    def test_expired_partial_results_preserve_success_and_missing_errors(self):
        with TemporaryDirectory() as temporary:
            api = Mock()
            api.batches.list.return_value = []
            api.files.create.return_value.id = 'input'
            api.batches.create.return_value = job('expired', 'output', 'error')
            api.batches.retrieve.return_value = job('expired', 'output', 'error')
            api.files.content.side_effect = [SimpleNamespace(text=json.dumps({'custom_id':'a', **response('True')})),
                SimpleNamespace(text=json.dumps({'custom_id':'b','error':{'code':'batch_expired'}}))]
            with patch.object(jobs, 'client', return_value=api):
                result = jobs.execute(Path(temporary), {'a':{},'b':{},'c':{}}, '/v1/responses', 'batch', self.args())
            self.assertEqual(jobs.response_text(jobs.body(result['a']), '/v1/responses'), 'True')
            for key in ('b','c'):
                with self.assertRaises(RuntimeError):
                    jobs.body(result[key])

    def test_changed_inputs_rejected_before_remote_call(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            _write_json(root/'input.json', {'mode':'batch','endpoint':'/v1/responses','requests':{'a':{}}})
            with patch.object(jobs, 'client') as api, self.assertRaisesRegex(RuntimeError,'inputs changed'):
                jobs.execute(root, {'b':{}}, '/v1/responses', 'batch', self.args())
            api.assert_not_called()

    def test_translation_repairs_only_failed_rules_and_replays_without_paid_calls(self):
        root = _repo_root()
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            rows = [{'id':f'rule-{i}', 'path':f'rust/library/core/src/test.rs:{i}',
                     'name':'unsafe fn f(b: u8)', 'rule':'b must be below 128.'} for i in (1,2,3)]
            _write_csv(output/'rules.csv', rows)
            _write_json(output/'step0.json', {'files':{'tools/rust/library/core/src/test.rs':[
                {'name':'f','line_number_start':i,'declaration_text':'unsafe fn f(b: u8)'} for i in (1,2,3)]}})
            api = Mock()
            texts = iter(['$1 < 128', '$9 < 128', 'UNEXPRESSIBLE', '$1 <= 127', '$1 < 128'])
            api.responses.create.side_effect = lambda **kw: SimpleNamespace(model_dump=lambda mode: jobs.body(response(next(texts))))
            args = self.args('--workers','1')
            with patch.object(jobs, 'client', return_value=api):
                pipeline.translate(root, output, args)
                pipeline.translate(root, output, args)
            self.assertEqual(api.responses.create.call_count, 5)
            results=json.loads((output/'step2.json').read_text())
            self.assertEqual([r['status'] for r in results], ['translated','translated','translated'])
            self.assertEqual([r['attempt'] for r in results], [0,1,1])
            cache=next((output/'translations').iterdir())
            self.assertEqual(json.loads((cache/'round-00/retry-queue.json').read_text()),['rule-2','rule-3'])

    def test_retry_bound_and_resume_with_larger_budget(self):
        root = _repo_root()
        with TemporaryDirectory() as temporary:
            output=Path(temporary)
            _write_csv(output/'rules.csv',[{'id':'rule-1','path':'rust/library/core/src/test.rs:1','name':'fn f(b: u8)','rule':'b < 128'}])
            _write_json(output/'step0.json',{'files':{'rust/library/core/src/test.rs':[{'name':'f','line_number_start':1,'declaration_text':'fn f(b: u8)'}]}})
            api=Mock()
            api.responses.create.return_value.model_dump.return_value=jobs.body(response('$9 < 128'))
            args=self.args('--retries','1','--workers','1')
            with patch.object(jobs,'client',return_value=api):
                with self.assertRaisesRegex(RuntimeError,'incomplete translations'):
                    pipeline.translate(root,output,args)
                self.assertEqual(api.responses.create.call_count,2)
                args.retries=2
                api.responses.create.return_value.model_dump.return_value=jobs.body(response('$1 < 128'))
                pipeline.translate(root,output,args)
                self.assertEqual(api.responses.create.call_count,3)

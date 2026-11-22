"""Contract checks for the callsite and crate Claude modes; no API requests."""
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('claude', Path(__file__).resolve().parents[2]/'docker/rust-comparison/run-claude.py')
claude = importlib.util.module_from_spec(spec)
spec.loader.exec_module(claude)

class ClaudeModes(unittest.TestCase):
    def test_image_build_includes_all_three_libraries(self):
        with tempfile.TemporaryDirectory() as tmp:
            libraries=Path(tmp)
            for name in ('core','std','alloc'):
                (libraries/name).mkdir()
                (libraries/name/'Cargo.toml').write_text('[package]\n')
                (libraries/name/'lib.rs').write_text('// requirements\n')
            def inspect_build(command, **kwargs):
                context=Path(command[-1])
                for name in ('core','std','alloc'):
                    self.assertEqual((context/'libraries'/name/'lib.rs').read_text(),'// requirements\n')
                self.assertIn('COPY libraries /libraries',(context/'Dockerfile.claude').read_text())
                return subprocess.CompletedProcess(command,0)
            with patch.object(claude.subprocess,'run',side_effect=inspect_build):
                claude.build_image(libraries)

    def test_callsite_output_contract(self):
        request = dict(rule='index < len', target_function='get', unsafe_api='get_unchecked',model='sonnet')
        response = dict(subtype='success', structured_output={'violation':True})
        with patch.object(claude.sys,'stdin',io.StringIO(json.dumps(request))), patch.object(claude.sys,'stdout',new_callable=io.StringIO) as out, patch.object(claude.subprocess,'run',return_value=subprocess.CompletedProcess([],0,json.dumps(response),'')):
            claude.inside()
            self.assertEqual(json.loads(out.getvalue()),{'violation':True})

    def test_crate_prompt_and_structured_result(self):
        request = dict(mode='crate',model='sonnet')
        response = dict(subtype='success',structured_output={'violations':[]})
        with patch.object(claude.sys,'stdin',io.StringIO(json.dumps(request))), patch.object(claude.sys,'stdout',new_callable=io.StringIO) as out, patch.object(claude.subprocess,'run',return_value=subprocess.CompletedProcess([],0,json.dumps(response),'')) as run:
            claude.inside()
            self.assertEqual(json.loads(out.getvalue())['violations'],[])
            self.assertIn('/libraries',run.call_args.kwargs['input'])
            self.assertIn('--add-dir',run.call_args.args[0])
            self.assertIn('/libraries',run.call_args.args[0])
            self.assertNotIn('safety',run.call_args.kwargs['input'].lower())
            self.assertNotIn('rule',run.call_args.kwargs['input'].lower())

    def test_malformed_crate_result_is_failure(self):
        for value in ({'violation':False},{'violations':[{'file':'x.rs'}]}):
            with self.assertRaises(ValueError): claude.validate_crate_result(value)

    def test_timeout_cleans_container_and_retains_evidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            crate=Path(tmp)/'crate'; crate.mkdir(); (crate/'Cargo.toml').write_text('[package]\nname="x"\nversion="0.1.0"\n')
            output=Path(tmp)/'output'
            timeout=subprocess.TimeoutExpired('docker',1,output=b'partial',stderr=b'diagnostic')
            with patch.object(claude,'api_key',return_value='test'), patch.object(claude.subprocess,'check_output',return_value='sha256:test'), patch.object(claude.subprocess,'run',side_effect=[timeout,subprocess.CompletedProcess([],0)]) as run:
                with self.assertRaises(subprocess.TimeoutExpired): claude.judge_crate(crate,output,timeout=1)
                self.assertEqual(run.call_args.args[0][:3],['docker','rm','-f'])
                self.assertEqual((output/'stdout.json').read_text(),'partial')
                self.assertEqual(json.loads((output/'error.json').read_text())['status'],'timeout')
                self.assertNotIn('test', (output/'request.json').read_text().replace('sha256:test',''))

if __name__ == '__main__': unittest.main()

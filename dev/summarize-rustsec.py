#!/usr/bin/env python3
"""Render explicitly reviewed, advisory-level coverage decisions (no LLM judge)."""
import collections
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "eval/rustsec"
TOOLS = ["ruklee", "ruklee-instantiated", "safe4u", "rudra", "mirai", "mirchecker", "lockbud", "prusti", "semgrep"]
LABELS = ["UnSAT 原始源码", "UnSAT +实例化", "Safe4U", "Rudra", "MIRAI", "MirChecker", "LockBud", "Prusti", "Semgrep"]
SYMBOLS = {"hit": "H", "miss": "M", "gap": "G", "error": "E", "partial": "P"}


def main():
    manifest = json.loads((ROOT / "benchmark/rustsec/manifest.json").read_text())
    cases = manifest["cases"]
    decisions = json.loads((OUT / "coverage-decisions.json").read_text())["decisions"]
    matrix = {(row["crate"], row["tool"]): row for row in decisions}
    assert len(matrix) == len(decisions) == len(cases) * len(TOOLS)
    for case in cases:
        name = Path(case["directory"]).name
        for tool in TOOLS:
            row = matrix[name, tool]
            assert row["outcome"] in SYMBOLS, row
            assert row["rationale"] and row["evidence"], row
            for evidence in row["evidence"]:
                assert (ROOT / evidence).exists(), evidence
                if evidence.endswith("/run.json"):
                    assert json.loads((ROOT / evidence).read_text()).get("status"), evidence
    summary = {}
    for tool in TOOLS:
        counts = collections.Counter(row["outcome"] for row in decisions if row["tool"] == tool)
        summary[tool] = {"defects": len(cases), "hits": counts["hit"], "coverage": counts["hit"] / len(cases),
                         "counts": dict(counts)}
    (OUT / "coverage.json").write_text(json.dumps({"unit": "one RustSec advisory defect", "denominator": 10,
        "cutoff_exclusive": "2026-03-01", "summary": summary, "decisions": decisions}, indent=2, ensure_ascii=False) + "\n")
    with (OUT / "coverage.csv").open("w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["crate", "advisory", "tool", "outcome", "rationale", "evidence"])
        writer.writeheader()
        for row in decisions:
            writer.writerow({k: (";".join(row[k]) if k == "evidence" else row[k]) for k in writer.fieldnames})
    lines = ["# RustSec unsafe API coverage", "", "10 个 crate、10 个公告缺陷；Issued 严格早于 2026-03-01。分母始终为 10，失败和覆盖缺口不从分母删除。", "",
             "H = 复核命中公告中的缺陷；M = 运行结束但未命中该缺陷；G = 提取、DSL、实例化或契约覆盖缺口；E = 崩溃/不支持/超时/错误输入；P = 部分分析。无关告警不算 H。", "",
             "| crate | RustSec | " + " | ".join(LABELS) + " |", "|---|---|" + "---|" * len(TOOLS)]
    for case in cases:
        name = Path(case["directory"]).name
        cells = [SYMBOLS[matrix[name, tool]["outcome"]] for tool in TOOLS]
        lines.append(f'| {name} | [{case["advisory"]}]({case["advisory_url"]}) | ' + " | ".join(cells) + " |")
    lines.extend(["", "| 工具/配置 | 命中 / 10 | 未命中 M | 缺口 G | 失败 E | 部分 P |", "|---|---:|---:|---:|---:|---:|"])
    for tool, label in zip(TOOLS, LABELS):
        s = summary[tool]; counts = s["counts"]
        lines.append(f'| {label} | {s["hits"]}/10 ({s["hits"]*10}%) | {counts.get("miss",0)} | {counts.get("gap",0)} | {counts.get("error",0)} | {counts.get("partial",0)} |')
    lines.extend(["", "统计口径：", "",
        "- 原始源码列使用已记录尝试中达到的覆盖并集，包括 library-only 运行，以及 smallvec 原有单元测试的单独复核；没有新增 API 实例化调用。它不代表仅运行一次默认命令的结果。",
        "- +实例化列另纳入隔离副本中的最小安全泛型调用。调用内容及命令保留在 run.json、源副本和 __rustsec_instantiations.rs；KLEE 回放成功后才记为本工具 H。",
        "- 本工具只定向验证 targets/ 中的公告相关危险操作，沿用原有 DSL。未新增 Vec::from_raw_parts 规则或 set_len 初始化规则。Safe4U/Rudra/MIRAI/LockBud/Prusti/Semgrep 扫描 library；MirChecker 设置公告入口。",
        "- Safe4U 使用 gpt-4o-mini、原有 Safe4U prompt 和既有 decomposition cache。原始输出在 6/10 个 crate 上有 unsound；严格匹配指定公告后为 2/10。maligned 的两空格缩进在 prettify_code_indent 中被截坏，虽然输出 unsound，仍记作 E。rendered-sample.txt 保留实际输入证据。",
        "- Safe4U 的 smallvec/lzf 解析到了 uninitialized，但该 API 文档没有 # Safety 标题，因此没有生成对应契约；其他 copy/offset/set_len 告警不能替代公告中的 uninitialized 缺陷。",
        "- Rudra 的 byte_struct 宏展开告警按一个公告去重；direct_ring_buffer 的 Send/Sync、smallvec 的其他 panic-safety 告警不匹配本次指定的缺陷。",
        "- MirChecker 的零退出码不保证正确入口或完整分析；同名方法、entry missing、compiler panic、OOM、edition 不支持分别保留。Prusti 的 unsupported/raw-pointer 和未完成验证不算有效零告警。",
        "- Semgrep 固定 p/default Rust+generic 规则，LockBud 固定 deadlock 模式；二者并非通用 unsafe API 前置条件检查器。MIRAI 的无告警也不构成安全证明。",
        "- 未使用 cargo-audit/公告 ID 匹配作为源码检测结果；编译器 deprecated/invalid_value lint 单独保留在日志中，未混入 analyzer 命中数。",
        "- 这是 10 个已知缺陷的目的性样本；未计算 TN 或全量告警 precision。KLEE 的 full_rerun_passed 指工具内 LLVM IR 回放，不等于独立 native/Miri 复现。", "",
        "[数据集与原始证据](../../benchmark/rustsec/README.md) · [逐项判定](coverage-decisions.json) · [机器结果](coverage.json) · [CSV](coverage.csv) · [选定告警复核](selected-reviews.json) · [原始告警统计](raw-alerts.json) · [所有运行](runs.json) · [环境](environment.json) · [源码完整性](source-integrity.json)", "",
        "复现汇总：`python3 dev/summarize-rustsec.py`。逐次命令、版本/镜像 ID、返回码、耗时及 stdout/stderr 都位于各 crate/tool/attempt 下；失败尝试未覆盖。较大的 LLVM 文件保留在磁盘并通过本目录 .gitignore 排除版本控制。", ""])
    (OUT / "README.md").write_text("\n".join(lines))
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()

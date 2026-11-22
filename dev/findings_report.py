#!/usr/bin/env python3
"""Write findings.md: violation results of run-all-crates, with reviewed verdicts.

Usage: dev/findings_report.py
Reads every .local/run-all-crates/run-*/jobs/*/*/*/result.json whose status is
violation or violation-timeout (low-confidence-sat and all other statuses are not
reported), keeps the latest result per crate/callsite/rule, and merges the manual
review in dev/findings/verdicts.json (key "<crate>::<callsite>").
"""
from __future__ import annotations

import collections
import json
from datetime import datetime, timezone
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
RUNS = REPO / ".local" / "run-all-crates"
VERDICTS = json.loads((REPO / "dev" / "findings" / "verdicts.json").read_text())
KEEP = {"violation", "violation-timeout"}


def collect():
    latest = {}
    for path in sorted(RUNS.glob("run-*/jobs/*/*/*/result.json")):
        try:
            data = json.loads(path.read_text())
        except ValueError:
            continue
        if data["result"]["init_status"] not in KEEP:
            continue
        crate = data["crate"].rstrip("/").split("/")[-1]
        if not ("/.local/crates/" in data["crate"] or "/.local/crates-rerun/" in data["crate"]):
            continue  # synthetic benchmarks are not corpus findings
        key = (crate, data["callsite"]["id"], data["rule"]["id"])
        latest[key] = (path, data)
    return latest


def main() -> None:
    latest = collect()
    by_site = collections.defaultdict(list)
    for (crate, site, _), (path, data) in sorted(latest.items()):
        by_site[(crate, site)].append((path, data))
    counts = collections.Counter()
    per_crate = collections.defaultdict(collections.Counter)
    for (crate, site) in by_site:
        verdict = VERDICTS.get(f"{crate}::{site}", {}).get("verdict", "未复核")
        counts[verdict] += 1
        per_crate[crate][verdict] += 1

    out = [
        "# Findings",
        "",
        f"更新时间：{datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')}。",
        "",
        "只列 `violation` / `violation-timeout`（取得 SAT 后超时），不含 `low-confidence-sat`、"
        "错误、超时和未覆盖。数据来自 `.local/run-all-crates/run-*` 下全部 `result.json`，"
        "同一 crate/调用点/规则取最近一次结果；合成基准（benchmark/ablation）不计入。",
        "",
        "复核标准：只用 safe 代码、以任意参数调用任意公开函数（含用 pub 字段构造值），"
        "能否让目标 unsafe 函数的安全要求被违反。能则为真阳性，并尽量给出原生运行的证据；"
        "被调用前的检查已排除、或反例本身合法的为误报。",
        "",
        f"共 {len(by_site)} 个调用点："
        + "，".join(f"{k} {v}" for k, v in sorted(counts.items())) + "。",
        "",
        "## 按 crate 汇总",
        "",
        "| Crate | 调用点 | 真阳性 | 误报 | 未复核 |",
        "|---|---:|---:|---:|---:|",
    ]
    for crate, c in sorted(per_crate.items()):
        out.append(f"| {crate} | {sum(c.values())} | {c['真阳性']} | {c['误报']} | {c['未复核']} |")
    out += ["", "## 明细", ""]
    for (crate, site), rows in sorted(by_site.items()):
        first = rows[0][1]
        verdict = VERDICTS.get(f"{crate}::{site}", {})
        cs = first["callsite"]
        out += [
            f"### {crate} — {cs['caller']} ({cs['path']}:{cs['line']}:{cs['col']})",
            "",
            f"- 结论：**{verdict.get('verdict', '未复核')}**",
            f"- 目标 unsafe 函数：`{cs['unsafe_callee']}`",
            "- 触发的规则：",
        ]
        for path, data in rows:
            out.append(f"  - `{data['rule']['id']}`（{data['result']['init_status']}）："
                       f"{data['rule']['rule']} · [result.json](<{path}>)")
        if verdict.get("reason"):
            out.append(f"- 依据：{verdict['reason']}")
        if verdict.get("poc"):
            out.append(f"- 验证：{verdict['poc']}")
        out.append("")
    (REPO / "findings.md").write_text("\n".join(out))
    print(f"{len(by_site)} callsites -> findings.md")


if __name__ == "__main__":
    main()

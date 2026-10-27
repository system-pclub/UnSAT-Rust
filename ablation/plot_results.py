#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DISPLAY = {
    "full": "RuKLEE",
    "without-rule-extraction": "① W.O. E",
    "without-rule-translation": "② W.O. T",
    "without-mut-ref-recovery": "③ W.O. M",
    "without-symbolic-execution": "④ W.O. S",
    "without-validation": "⑤ W.O. V",
}


def main() -> int:
    parser = argparse.ArgumentParser(description="Draw the RuKLEE ablation bar chart in the SymGPT Figure 11 style.")
    parser.add_argument("--summary", type=Path, default=ROOT / "ablation" / "results" / "summary.json")
    parser.add_argument("--output", type=Path, default=ROOT / "ablation" / "results" / "ablation.pdf")
    parser.add_argument("--png", type=Path)
    args = parser.parse_args()

    from matplotlib import pyplot as plt
    import numpy as np

    document = json.loads(args.summary.read_text(encoding="utf-8"))
    variants = document.get("variants", [])
    if not variants:
        raise RuntimeError(f"summary contains no variants: {args.summary}")
    labels = [DISPLAY.get(item["label"], item["label"]) for item in variants]
    tp = [item["TP"] for item in variants]
    fp = [item["FP"] for item in variants]
    total = int(document["ground_truth_total"])
    coverage = document.get("coverage", {})
    partial = isinstance(coverage, dict) and coverage.get("status") != "complete"

    bar_width = 0.35
    x = np.arange(len(labels), dtype=float)
    if len(x) > 1:
        x[1:] += 0.45
    fig, ax = plt.subplots(figsize=(11.5, 3.5))
    tp_bars = ax.bar(
        x,
        tp,
        width=bar_width,
        edgecolor="green",
        facecolor="none",
        hatch="..",
        linewidth=1.3,
        label="TP",
    )
    fp_bars = ax.bar(
        x + bar_width,
        fp,
        width=bar_width,
        edgecolor="red",
        facecolor="none",
        hatch="//",
        linewidth=1.3,
        label="FP",
    )
    ax.axhline(total, color="green", linestyle="--", linewidth=2, label=f"Total = {total}")
    ax.set_xticks(x + bar_width / 2, labels, fontsize=14)
    upper = max([total, *tp, *fp, 1])
    ax.set_ylim(0, upper * 1.18 + 1)
    ax.tick_params(axis="y", labelsize=14)
    ax.legend(fontsize=12, ncol=3)
    if partial:
        included = ", ".join(coverage.get("included_crates", []))
        ax.set_title(f"Partial smoke result ({included})", fontsize=12)
    for bars, values, color in ((tp_bars, tp, "green"), (fp_bars, fp, "red")):
        for bar, value in zip(bars, values):
            ax.annotate(
                str(value),
                xy=(bar.get_x() + bar.get_width() / 2, bar.get_height()),
                xytext=(0, 6),
                textcoords="offset points",
                ha="center",
                va="bottom",
                color=color,
                fontsize=13,
            )
    fig.tight_layout()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(args.output, bbox_inches="tight")
    if args.png:
        args.png.parent.mkdir(parents=True, exist_ok=True)
        fig.savefig(args.png, dpi=300, bbox_inches="tight")
    print(f"wrote {args.output}")
    if args.png:
        print(f"wrote {args.png}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

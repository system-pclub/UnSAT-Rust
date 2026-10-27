#!/usr/bin/env python3
"""Draw the dissertation ablation figure using hard-coded counts.

Run: python3 ablation/graph.py [--output PATH] [--png PATH]
Replace each "xxx" in RESULTS with an adjudicated integer when available.
Until then, unknown bars use explicitly schematic heights, not measurements.
Style reference: SymGPT's ablation figure (Chapter-3/paper/images/output1.png).
"""
from __future__ import annotations

import argparse
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
from matplotlib import pyplot as plt
from matplotlib.patches import Patch
from matplotlib.lines import Line2D


ROOT = Path(__file__).resolve().parents[1]
GROUND_TRUTH_TOTAL = 44
# Full-pipeline counts: dissertation Table tab:ruklee-ground-truth.
# Ablated counts are pending; do not substitute location-only smoke scores.
RESULTS = [
    {"label": "RuKLEE", "TP": 44, "FP": 0},
    {"label": "① W.O. E", "TP": "xxx", "FP": "xxx"},
    {"label": "② W.O. T", "TP": "xxx", "FP": "xxx"},
    {"label": "③ W.O. M", "TP": "xxx", "FP": "xxx"},
    {"label": "④ W.O. S", "TP": "xxx", "FP": "xxx"},
    {"label": "⑤ W.O. V", "TP": "xxx", "FP": "xxx"},
]
# Fractions of the reference line used ONLY to lay out pending draft bars.
# They illustrate hypotheses, not estimated or measured experimental counts.
SCHEMATIC_HEIGHTS = [
    (1.00, 0.00),
    (0.65, 0.25),
    (0.45, 0.65),
    (0.70, 0.00),
    (0.55, 1.40),
    (1.00, 0.85),
]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "ablation" / "ablation.pdf")
    parser.add_argument("--png", type=Path, default=ROOT / "ablation" / "ablation.png")
    args = parser.parse_args()

    plt.rcParams.update({"font.family": "DejaVu Sans", "pdf.fonttype": 42})
    fig, ax = plt.subplots(figsize=(11.5, 2.8))
    positions = [0, 1.55, 2.65, 3.75, 4.85, 5.95]
    width = 0.35
    pending = False
    for index, (position, result) in enumerate(zip(positions, RESULTS)):
        for metric, offset, color, hatch, schematic in (
            ("TP", 0, "green", "..", SCHEMATIC_HEIGHTS[index][0]),
            ("FP", width, "red", "//", SCHEMATIC_HEIGHTS[index][1]),
        ):
            count = result[metric]
            unknown = count == "xxx"
            if not unknown and (not isinstance(count, int) or count < 0):
                raise ValueError(f"Invalid {metric} count for {result['label']}: {count}")
            if metric == "TP" and not unknown and count > GROUND_TRUTH_TOTAL:
                raise ValueError("TP exceeds the ground-truth defect total")
            pending |= unknown
            height = schematic * GROUND_TRUTH_TOTAL if unknown else count
            ax.bar(position + offset, height, width=width,
                   edgecolor=color, facecolor="none", hatch=hatch, linewidth=1)
            ax.annotate(str(count), (position + offset, height),
                        xytext=(0, 1), textcoords="offset points",
                        ha="center", va="bottom", color=color, fontsize=16)

    ax.axhline(GROUND_TRUTH_TOTAL, color="green", linestyle="--", linewidth=1.5)
    ax.set_xticks([x + width / 2 for x in positions],
                  [result["label"] for result in RESULTS], fontsize=16)
    ax.tick_params(axis="y", labelsize=14)
    heights = [
        SCHEMATIC_HEIGHTS[i][j] * GROUND_TRUTH_TOTAL if row[key] == "xxx" else row[key]
        for i, row in enumerate(RESULTS) for j, key in enumerate(("TP", "FP"))
    ]
    ax.set_ylim(0, max(GROUND_TRUTH_TOTAL, *heights) * 1.18)
    ax.set_xlim(-0.65, positions[-1] + 0.85)
    ax.legend(handles=[
        Line2D([0], [0], color="green", linestyle="--", label="Total = 44"),
        Patch(edgecolor="green", facecolor="none", hatch="..", label="TP"),
        Patch(edgecolor="red", facecolor="none", hatch="//", label="FP"),
    ], loc="upper left", fontsize=12, ncol=3)
    if pending:
        fig.text(0.995, 0.015, "Draft: xxx = pending; ablated bar heights are schematic.",
                 ha="right", va="bottom", fontsize=10, color="#555555")
    fig.tight_layout(rect=(0, 0.065 if pending else 0, 1, 1), pad=0.4)
    for path in (args.output, args.png):
        path.parent.mkdir(parents=True, exist_ok=True)
        fig.savefig(path, dpi=300, bbox_inches="tight")
        print(f"wrote {path}")
    plt.close(fig)


if __name__ == "__main__":
    main()

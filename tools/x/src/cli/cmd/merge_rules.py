import argparse
import csv
from pathlib import Path
from itertools import zip_longest


REQUIRED_COLUMNS = {"path", "name", "rule", "tp"}
OUTPUT_COLUMNS = ["id", "path", "name", "rule"]


def _normalize_column(name: str | None) -> str:
    if name is None:
        return ""
    return name.strip().lower()


def _tp_is_one(value: str | None) -> bool:
    if value is None:
        return False
    normalized = value.strip().lower()
    return normalized in {"1", "1.0", "true", "yes", "y"}


def _find_csv_files(rules_dir: Path) -> list[Path]:
    if not rules_dir.is_dir():
        raise RuntimeError(f"rules directory does not exist: {rules_dir}")
    return sorted(path for path in rules_dir.rglob("*.csv") if path.is_file())


def _validate_header(fieldnames: list[str], csv_path: Path) -> None:
    if not fieldnames:
        raise RuntimeError(f"CSV has no header: {csv_path}")

    normalized = {_normalize_column(name) for name in fieldnames}
    missing = REQUIRED_COLUMNS.difference(normalized)
    if missing:
        missing_list = ", ".join(sorted(missing))
        raise RuntimeError(f"CSV missing required columns ({missing_list}): {csv_path}")


def _prepare_headers(rows: list[list[str]], csv_path: Path) -> tuple[list[str], int]:
    if not rows:
        raise RuntimeError(f"CSV has no header: {csv_path}")

    headers = [cell.strip() for cell in rows[0]]
    data_start_index = 1

    # Some rule exports place "TP" in a second header row under blank columns.
    if len(rows) > 1:
        normalized = {_normalize_column(name) for name in headers}
        if not REQUIRED_COLUMNS.issubset(normalized):
            second_header = [cell.strip() for cell in rows[1]]
            merged = [
                (
                    second
                    if _normalize_column(second) in REQUIRED_COLUMNS
                    else (first if first else second)
                )
                for first, second in zip_longest(headers, second_header, fillvalue="")
            ]
            merged_normalized = {_normalize_column(name) for name in merged}
            if REQUIRED_COLUMNS.issubset(merged_normalized):
                headers = merged
                data_start_index = 2

    _validate_header(headers, csv_path)
    return headers, data_start_index


def _rows_as_dicts(rows: list[list[str]], headers: list[str], data_start: int) -> list[dict[str, str]]:
    header_keys = [_normalize_column(header) for header in headers]
    normalized_rows: list[dict[str, str]] = []

    for row in rows[data_start:]:
        if not any(cell.strip() for cell in row):
            continue

        record: dict[str, str] = {}
        for index, key in enumerate(header_keys):
            if not key:
                continue
            record[key] = row[index].strip() if index < len(row) else ""
        normalized_rows.append(record)

    return normalized_rows


def run(args: argparse.Namespace) -> int:
    rules_dir = Path(args.rules_dir)
    output_path = Path(args.output)

    csv_files = _find_csv_files(rules_dir)
    if not csv_files:
        raise RuntimeError(f"no CSV files found under: {rules_dir}")

    rows_to_write: list[dict[str, str]] = []
    for csv_path in csv_files:
        with csv_path.open("r", encoding="utf-8", newline="") as infile:
            raw_rows = list(csv.reader(infile))
            headers, data_start = _prepare_headers(raw_rows, csv_path)

            for row in _rows_as_dicts(raw_rows, headers, data_start):
                if _tp_is_one(row.get("tp")):
                    rows_to_write.append(
                        {
                            "path": (row.get("path") or "").strip(),
                            "name": (row.get("name") or "").strip(),
                            "rule": (row.get("rule") or "").strip(),
                        }
                    )

    rows_to_write.sort(key=lambda row: (row.get("path", ""), row.get("name", ""), row.get("rule", "")))
    for index, row in enumerate(rows_to_write, start=1):
        row["id"] = f"rule-{index}"

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with output_path.open("w", encoding="utf-8", newline="") as outfile:
        writer = csv.DictWriter(outfile, fieldnames=OUTPUT_COLUMNS)
        writer.writeheader()
        writer.writerows(rows_to_write)

    print(
        f"merged_rows={len(rows_to_write)} csv_files={len(csv_files)} output={output_path}"
    )
    return 0

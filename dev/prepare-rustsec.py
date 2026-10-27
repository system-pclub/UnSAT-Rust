#!/usr/bin/env python3
"""Download ten pinned vulnerable crates and their primary RustSec evidence."""
import concurrent.futures
import hashlib
import io
import json
from pathlib import Path
import re
import tarfile
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
CASES = [
    ("binary_vec_io", "0.1.12", "RUSTSEC-2025-0109"),
    ("borrowck_sacrifices", "0.1.0", "RUSTSEC-2025-0107"),
    ("direct_ring_buffer", "0.2.1", "RUSTSEC-2025-0105"),
    ("maligned", "0.2.1", "RUSTSEC-2023-0017"),
    ("safe-transmute", "0.10.0", "RUSTSEC-2018-0013"),
    ("byte_struct", "0.6.0", "RUSTSEC-2021-0032"),
    ("elf_rs", "0.2.0", "RUSTSEC-2022-0079"),
    ("smallvec", "0.6.12", "RUSTSEC-2018-0018"),
    ("linked-hash-map", "0.5.2", "RUSTSEC-2020-0026"),
    ("lzf", "0.3.1", "RUSTSEC-2022-0067"),
]


def fetch(url):
    request = urllib.request.Request(url, headers={"User-Agent": "UnSAT-Rust RustSec benchmark research"})
    return urllib.request.urlopen(request, timeout=60).read()


def prepare(case):
    name, version, advisory = case
    evidence = ROOT / "benchmark/rustsec/evidence" / advisory
    evidence.mkdir(parents=True, exist_ok=True)
    url = f"https://rustsec.org/advisories/{advisory}.html"
    html = fetch(url)
    (evidence / "advisory.html").write_bytes(html)
    published = re.search(r'<dt>Issued</dt>\s*<dd>(\d{4}-\d{2}-\d{2})', html.decode())
    if published is None:
        # RustSec renders the issued date as an ISO datetime on <time>.
        published = re.search(r'<dt[^>]*>Issued</dt>.*?datetime="(\d{4}-\d{2}-\d{2})', html.decode(), re.S)
    if published is None:
        raise RuntimeError(f"Cannot establish issued date: {url}")
    issued = published.group(1)
    assert issued < "2026-03-01", (advisory, issued)
    md = ROOT / f".local/rustsec-research/crates/{name}/{advisory}.md"
    if not md.exists():
        md.parent.mkdir(parents=True, exist_ok=True)
        md.write_bytes(fetch(f"https://raw.githubusercontent.com/rustsec/advisory-db/main/crates/{name}/{advisory}.md"))
    (evidence / "advisory.md").write_bytes(md.read_bytes())
    advisory_data = tomllib.loads(md.read_text().split("```toml", 1)[1].split("```", 1)[0])
    metadata = json.loads(fetch(f"https://crates.io/api/v1/crates/{name}/{version}"))["version"]
    assert metadata["created_at"][:10] < "2026-03-01"
    archive_url = f"https://static.crates.io/crates/{name}/{name}-{version}.crate"
    data = fetch(archive_url)
    checksum = hashlib.sha256(data).hexdigest()
    assert checksum == metadata["checksum"]
    archive = evidence / f"{name}-{version}.crate"
    archive.write_bytes(data)
    source = ROOT / f"benchmark/rustsec/{name}-{version}"
    if not source.exists():
        with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as tar:
            tar.extractall(source.parent, filter="data")
    row = {"crate": name, "version": version, "directory": str(source.relative_to(ROOT)),
           "advisory": advisory, "advisory_url": url, "issued": issued,
           "reported": str(advisory_data["advisory"]["date"]),
           "versions": advisory_data["versions"], "affected_functions": advisory_data.get("affected", {}).get("functions", {}),
           "crate_published": metadata["created_at"], "crate_sha256": checksum,
           "download_url": archive_url, "references": advisory_data["advisory"].get("references", [])}
    row["upstream_url"] = advisory_data["advisory"].get("url")
    row["advisory_markdown_sha256"] = hashlib.sha256(md.read_bytes()).hexdigest()
    row["advisory_html_sha256"] = hashlib.sha256(html).hexdigest()
    (evidence / "provenance.json").write_text(json.dumps(row, indent=2) + "\n")
    return row


if __name__ == "__main__":
    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as pool:
        rows = list(pool.map(prepare, CASES))
    manifest = {"cutoff_exclusive": "2026-03-01", "selection": "Purposive sample of unsafe API misuses across bounds, initialization, layout, and ownership; not a random sample.", "cases": rows}
    (ROOT / "benchmark/rustsec/manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    for row in rows:
        print(row["advisory"], row["crate"], row["version"], row["issued"])

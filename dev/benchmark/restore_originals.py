#!/usr/bin/env python3
"""Restore exact published sources, preferring local crate archives."""
import hashlib
import json
from pathlib import Path
import shutil
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[2]

def main():
    records = json.loads((ROOT / 'dev/benchmark/sources.json').read_text())
    for row in records:
        if row['original_verified']:
            continue
        name = row['directory']
        archive = ROOT / '.local/rawcrates' / (name + '.crate')
        package, version = name.rsplit('-', 1)
        url = f'https://static.crates.io/crates/{package}/{name}.crate'
        if not archive.exists():
            req = urllib.request.Request(url, headers={'User-Agent': 'UnSAT-Rust benchmark research'})
            with urllib.request.urlopen(req, timeout=60) as response:
                archive.write_bytes(response.read())
        dest = ROOT / 'benchmark/original' / name
        shutil.rmtree(dest)
        with tarfile.open(archive) as tf:
            tf.extractall(dest.parent, filter='data')
        row.update(archive=str(archive.relative_to(ROOT)), archive_url=url,
                   archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(), original_verified=True)
        row.pop('candidate', None)
        print(name, 'restored', flush=True)
        (ROOT / 'dev/benchmark/sources.json').write_text(json.dumps(records, indent=2)+'\n')

if __name__ == '__main__':
    main()

#!/usr/bin/env python3
"""Validate the TASK roster, source-only layout and original archive contents."""
import hashlib
import json
from pathlib import Path
import tarfile
import tomllib

ROOT=Path(__file__).resolve().parents[2]

def main():
    records=json.loads((ROOT/'dev/benchmark/sources.json').read_text())
    assert len(records)==32, len(records)
    for tool in ('original','culpa','ruklee','safe4u'):
        base=ROOT/'benchmark'/tool
        for child in base.iterdir():
            assert child.is_dir() and (child/'Cargo.toml').is_file(), child
        for row in records:
            source=base/row['directory']
            package=tomllib.loads((source/'Cargo.toml').read_text())['package']
            assert f"{package['name']}-{package['version']}"==row['directory'], source
    for row in records:
        archive=ROOT/row['archive']
        assert hashlib.sha256(archive.read_bytes()).hexdigest()==row['archive_sha256'],archive
        with tarfile.open(archive) as tf:
            for entry in tf:
                if entry.isfile():
                    assert tf.extractfile(entry).read()==(ROOT/'benchmark/original'/entry.name).read_bytes(), entry.name
    print('32 requested crates present in all four source-only corpora; originals match published archives.')

if __name__=='__main__': main()

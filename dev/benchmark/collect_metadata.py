#!/usr/bin/env python3
"""Refresh upstream metadata, retaining the exact responses used by the table."""
import concurrent.futures
import json
from pathlib import Path
import urllib.request

OUT = Path(__file__).resolve().parent / 'metadata'
OUT.mkdir(exist_ok=True)

def fetch(item):
    name, url = item
    req = urllib.request.Request(url, headers={'User-Agent': 'UnSAT-Rust benchmark metadata (research)', 'Accept': 'application/json'})
    try:
        with urllib.request.urlopen(req, timeout=60) as response:
            value = json.load(response)
        (OUT / (name + '.json')).write_text(json.dumps({'url': url, 'response': value}, indent=2) + '\n')
        print(name, 'ok', flush=True)
    except Exception as exc:
        print(name, str(exc), flush=True)

if __name__ == '__main__':
    urls = [(n, 'https://crates.io/api/v1/crates/' + n) for n in ('talc','redis-module','azul-css')]
    urls += [(n+'-github', 'https://api.github.com/repos/'+repo) for n,repo in
             [('talc','SFBdragon/talc'),('redis-module','RedisLabsModules/redismodule-rs'),('azul-css','fschutt/azul')]]
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
        list(pool.map(fetch, urls))

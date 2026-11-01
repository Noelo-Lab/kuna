#!/usr/bin/env python3
"""Rebuild original Octeon test instructions; not required to run the tests."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--assembler', default='mips64-linux-gnuabi64-as')
    parser.add_argument('--objcopy', default='objcopy')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    source = Path(__file__).with_name('memory.S')
    result = {'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(), 'outputs': {}}
    with tempfile.TemporaryDirectory(prefix='octeon-atomics-') as temporary:
        directory = Path(temporary)
        for abi, flag in [('n32', '-n32'), ('n64', '-64')]:
            obj, raw = directory/f'{abi}.o', directory/f'{abi}.raw'
            subprocess.run([args.assembler, '-EB', flag, '-march=octeon2', '-o', str(obj), str(source)], check=True)
            subprocess.run([args.objcopy, '-O', 'binary', '-j', '.text', str(obj), str(raw)], check=True)
            data = raw.read_bytes()
            result['outputs'][abi] = {'sha256': hashlib.sha256(data).hexdigest(), 'hex': data.hex()}
    args.output.write_text(json.dumps(result, indent=2)+'\n')


if __name__ == '__main__':
    main()

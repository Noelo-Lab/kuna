"""Rebuild the original GCC bitfield value/lifetime regressions."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

root = Path(__file__).resolve().parent
compiler = Path(shutil.which('gcc')).resolve()
rows = []
for debug, name in [('-gdwarf-4', 'review-dwarf4.o'), ('-g0', 'review-no-debug.o')]:
    argv = ['gcc', '-O1', debug, '-gno-record-gcc-switches', '-fcf-protection=none',
            '-fno-stack-protector', '-fdebug-prefix-map=' + str(root) + '=.',
            '-c', 'review.c', '-o', name]
    subprocess.run(argv, cwd=root, check=True)
    rows.append(dict(output=name, argv=[s.replace(str(root), '$PWD') for s in argv],
                     sha256=hashlib.sha256((root / name).read_bytes()).hexdigest()))
record = dict(source='review.c', source_sha256=hashlib.sha256((root / 'review.c').read_bytes()).hexdigest(),
              compiler=compiler.name, compiler_sha256=hashlib.sha256(compiler.read_bytes()).hexdigest(),
              version=subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0],
              builds=rows)
(root / 'provenance.json').write_text(json.dumps(record, indent=2) + '\n')

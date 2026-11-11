from pathlib import Path
import subprocess,json
w=Path('/tmp/kuna-rust-fmt-write')
with(w/'postreview.log').open('w')as f:
 subprocess.run(['python3','-u',str(w/'postreview.py')],stdout=f,stderr=subprocess.STDOUT,check=True)
assert not json.loads((w/'breadth/record.json').read_text())['semantic_evidence_pending']
with(w/'breadth-69fa.log').open('a')as f:f.write('REFERENCE TYPE CONTEXT REVIEW PASSED; ALL FRESH EXPRESSIONS EXECUTED\nWRITE RECOVERY READY FOR DECLARED TIMINGS\n')
print('WRITE RECOVERY READY FOR DECLARED TIMINGS',flush=True)

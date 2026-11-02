from common import *
source=(w/'callback.c').read_text().split('int main(void)')[0];harness=(w/'native-harness.c').read_text();out=w/'native-final';out.mkdir(exist_ok=False);expected=None;record={}
for arm in ['source','parent','candidate']:
 if arm=='source':body=source
 else:
  d=json.loads((w/f'{arm}.json').read_text());body='#include <stdint.h>\ntypedef int64_t int8; typedef uint64_t uint8; typedef int32_t int4;\ntypedef struct Pair { int8 first; int8 second; } Pair;\nuint64_t calls;\n'+'\n'.join(f['code']for f in d['functions'])
 file=out/f'{arm}.c';file.write_text(body+'\n'+harness);record[arm]=[]
 for opt in ['0','2']:
  binary=out/f'{arm}-O{opt}';p=subprocess.run(['cc','-O'+opt,'-fwrapv','-Werror',str(file),'-o',str(binary)],text=True,capture_output=True);(out/f'{arm}-O{opt}-build.log').write_text(p.stdout+p.stderr)
  if arm=='parent':assert p.returncode and 'aggregate value'in p.stderr;record[arm].append({'opt':opt,'compile_exit':p.returncode});continue
  assert p.returncode==0,p.stderr
  q=subprocess.run([str(binary)],capture_output=True,check=True);assert len(q.stdout.splitlines())==169
  (out/f'{arm}-O{opt}.stdout').write_bytes(q.stdout)
  if expected is None:expected=q.stdout
  else:assert q.stdout==expected
  record[arm].append({'opt':opt,'compile_exit':0,'run_exit':q.returncode,'input_pairs':169,'output_sha256':hashlib.sha256(q.stdout).hexdigest()})
(out/'record.json').write_text(json.dumps({'complete_unmodified_emitted_bodies':8,'candidate_input_comparisons':338,'single_evaluation_asserted':True,'results':record},indent=2)+'\n');print('NATIVE PASS: 338 boundary pairs, 8 full bodies, single callback evaluation; parent compile fails.',flush=True)

from common import *
from concurrent.futures import ThreadPoolExecutor

run(r, w, ['python3', str(w / 'prepare.py')], 'prepare-coordinator.log')
run(r, w, ['python3', str(w / 'native.py')], 'native-coordinator.log')
with ThreadPoolExecutor(max_workers=2) as pool:
    futures = [pool.submit(run, r, w, ['python3', str(w / script)], script.removesuffix('.py') + '-coordinator.log') for script in ['validate.py', 'breadth.py']]
    for future in futures: future.result()
run(r, w, ['/tmp/kuna-rust-integer-conversions/ast-audit/target/release/kuna-rust-conversion-audit', str(w / 'breadth/record.json'), str(w / 'breadth/ast-algebra-review.json')], 'ast-review.log')
review = json.loads((w / 'breadth/ast-algebra-review.json').read_text())
print('Rust full validation complete; AST pending:', review['pending_count'], flush=True)

from common import *
import collections

out = w / 'breadth'
data = json.loads((out / 'record.json').read_text())
audit = json.loads((out / 'ast-algebra-review-v3.json').read_text())
key = lambda row: (row['fixture'], row['address'], row['gopairreturns'])
classified = {key(row): row for row in audit['records']}
pending = {key(row): row for row in audit['pending']}
assert len(classified) == audit['verified'] and len(pending) == audit['pending_count']
assert not classified.keys() & pending.keys()
parse_states = collections.Counter((bool(row['original_parse_errors']['parent']), bool(row['original_parse_errors']['candidate'])) for row in audit['records'] + audit['pending'])
assert parse_states[(False, True)] == 0, 'New Rust parser failure'
assert parse_states == {(False, False): 12648, (True, True): 66, (True, False): 28}

all_keys = set()
for row in data['records']:
    for change in row['changes']:
        identity = (row['fixture'], change['address'], change['gopairreturns'])
        assert identity not in all_keys
        all_keys.add(identity)
        assert change['language'] == 'rust'
        for arm in ['parent', 'candidate']:
            assert '__audit_' not in change[arm]['code'], 'Audit identifier collision'

        assert set(change['fields']) <= {'code', 'line_mappings'}
assert all_keys == classified.keys() | pending.keys()
assert len(all_keys) == data['changed_rust_records']
native = [json.loads((w / directory / 'record.json').read_text()) for directory in ['native-context', 'native-conversions']]
assert all(record['status'] == 'PASS' and not record['body_rewrites'] for record in native)
summary = {'complete': not pending, 'scope': 'Complete changed records classified by syntax-tree and modular-integer transformations, alongside whole-body native probes and source guard review. This does not establish native equivalence for whole runtime images.', 'images': data['images'], 'distinct_functions': data['distinct_functions'], 'language_function_records': data['language_function_records'], 'changed_rust_records': len(all_keys), 'classified_records': len(classified), 'pending_records': len(pending), 'c_go_records_unchanged': True, 'native_complete_bodies': 64, 'native_function_comparisons': sum(record['function_comparisons'] for record in native), 'new_decompiler_errors': 0, 'new_rust_parse_errors': 0, 'existing_rust_parse_failures': parse_states[(True, True)], 'rust_parse_failures_resolved': parse_states[(True, False)], 'ast_report_sha256': hashlib.sha256((out / 'ast-algebra-review-v3.json').read_bytes()).hexdigest(), 'audit_source_sha256': hashlib.sha256((w.parent / 'ast-audit/src/main.rs').read_bytes()).hexdigest(), 'audit_binary_sha256': hashlib.sha256((w.parent / 'ast-audit/target/release/kuna-rust-conversion-audit').read_bytes()).hexdigest(), 'limitations': ['Native Rust checks use overflow-checks=off to model wrapping machine arithmetic.', 'Primitive casts and exact core identity calls are stripped for structural comparison; rendered-type correctness also relies on same-width source guards and targeted native probes.', 'Associative integer operations may regroup without reordering operands; local name/field compound assignments expand without evaluating a pointer expression twice.', 'For the 94 records with a raw parse error in either arm, the review also erases primitive casts lexically and renames invalid fn/self/underscore-path identifiers before parsing; raw output and parser status remain retained. This is review-only normalization, not a native body rewrite.', 'STORE-width and direct-STORE signedness repairs are outside this feature.'], 'pending': list(pending.values())}
(out / 'structural-review.json').write_text(json.dumps(summary, indent=2) + '\n')
print('Rust record review:', len(classified), 'classified;', len(pending), 'pending', flush=True)
assert not pending, 'Inspect unclassified changes before timing or publication.'

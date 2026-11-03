from common import *

build = json.loads((w / 'build.json').read_text())
old = '0f2eaa8314f5f5f66c1a6ece3829c381bf70360dd3977d3f94ad081dffb67585'
for source, output, old_output, new_output in [('angle-context-native.py', 'native-context-run.py', 'angle-context-native', 'native-context'), ('angle-context-49.py', 'native-conversions-run.py', 'angle-context-49', 'native-conversions')]:
    text = (w.parent / source).read_text()
    assert text.count(old) == 1
    text = text.replace(old, build['new_cli_sha256']).replace('/home/mahaloz/kwt/angr-go-pair-parent-a324', '/home/mahaloz/kwt/angr-go-store-conversions')
    text = text.replace('66e4333c4a1a8ab1be4aca046ac5efb5d71af78f0185d27ed61887e0b21d5727', build['parent_cli_sha256'])
    text = text.replace("out=w/'" + old_output + "'", "out=w/'" + new_output + "'")
    text = text.replace("source=w/'planned-angle-subtraction'", "source=w.parent/'planned-angle-subtraction'")
    text = text.replace("source=w/'planned-fixtures'", "source=w.parent/'planned-fixtures'")
    text = text.replace("initial=json.loads((w/'initial-source.json').read_text())\n", '')
    (w / output).write_text(text)
    run(r, w, ['python3', str(w / output)], new_output + '-coordinator.log')
    record = json.loads((w / new_output / 'record.json').read_text())
    assert record['status'] == 'PASS'
    assert record['candidate_cli_sha256'] == build['new_cli_sha256'] and record['parent_cli_sha256'] == build['parent_cli_sha256']
    for language in ['c', 'go', 'rust']:
        previous = json.loads((w.parent / old_output / ('candidate-' + language + '.json')).read_text())
        current = json.loads((w / new_output / ('candidate-' + language + '.json')).read_text())
        assert current['functions'] == previous['functions'], (new_output, language, 'Complete native probe output changed after integration; inspect before claiming inherited review.')
print('All 64 native bodies preserve pre-integration captures and match C references', flush=True)

#!/usr/bin/env bash
# Harness self-checks for a regenerated corpus (not needed to regenerate it):
#   1. determinism      fixture oracle re-run              == expected/
#   2. input order      fixture oracle, class files reversed == expected/
#   3. CLI equivalence  driver --save folder --plugins     == expected-defaults/ (java -jar)
#   4. plugin effect    driver --save folder (no plugins)  vs expected-defaults/ -> list of differing tests
#   5. defaults order   java -jar, class files reversed    == expected-defaults/
#   6. threads          fixture oracle with 1 visible CPU (thread-count default) == expected/
#   7. runtime need     defaults with include-runtime="" vs expected-defaults/ -> tests that need the JDK library
#   8. runtime version  java -jar on JDK 21 (its runtime image) vs expected-defaults/ (JDK 17)
# Usage: scripts/crosscheck.sh [OUT_ROOT]   (writes OUT_ROOT/reports/crosscheck.json; OUT_ROOT defaults to $CORPUS)
# Env: KUNA_JVM_ORACLE, or the overrides CORPUS, JDKS, VF_JAR; JOBS, WORK.
set -euo pipefail
TOOLS=$(cd "$(dirname "$(readlink -f "$0")")/.." && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
export CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus}
OUT=$(cd "${1:-$CORPUS}" && pwd)
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
JOBS=${JOBS:-16}
X=${WORK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/work/java-crosscheck-$(date +%Y%m%d-%H%M%S)-$$}
mkdir -p "$X"
run() { local root=$1; shift
  python3 -I "$TOOLS/scripts/run_oracle.py" --manifest "$OUT/manifest.json" --classes "$OUT/classes" \
    --corpus "$CORPUS" --jar "$VF_JAR" --java "$JDKS/jdk17/bin/java" --jobs "$JOBS" --out-root "$X/$root" \
    --work "$X/work-$root" "$@"; }
run rerun     --mode fixture
run reversed  --mode fixture --order reverse
run cliplug   --mode defaults-driver --plugins
run noplug    --mode defaults-driver
run defrev    --mode defaults --order reverse
VF_JAVA_OPTS=-XX:ActiveProcessorCount=1 run onecpu --mode fixture   # thread-count default = 1 CPU
run nojrt     --mode defaults-driver --plugins --extra-opt include-runtime=   # defaults minus the JDK library
run jdk21     --mode defaults --java "$JDKS/jdk21/bin/java"                   # defaults on a JDK 21 runtime image
python3 -I - "$OUT" "$X" <<'EOF'
import json, os, sys, filecmp
out, x = sys.argv[1], sys.argv[2]
m = json.load(open(os.path.join(out, 'manifest.json')))
ok = [t for t in m['tests'] if t['status'] == 'ok']
def cmp(key, root):
    return sorted(t['id'] for t in ok
                  if not filecmp.cmp(os.path.join(out, t[key]), os.path.join(x, root, t[key]), shallow=False))
res = dict(work=x, tests=len(ok),
           determinism_diffs=cmp('oracle', 'rerun'),
           reversed_order_diffs=cmp('oracle', 'reversed'),
           cli_vs_driver_with_plugins_diffs=cmp('defaults_oracle', 'cliplug'),
           defaults_reversed_order_diffs=cmp('defaults_oracle', 'defrev'),
           one_cpu_diffs=cmp('oracle', 'onecpu'),
           plugin_effect_on_defaults=cmp('defaults_oracle', 'noplug'),
           defaults_needing_jdk_runtime=cmp('defaults_oracle', 'nojrt'),
           defaults_changed_by_jdk21_runtime=cmp('defaults_oracle', 'jdk21'))
p = os.path.join(out, 'reports', 'crosscheck.json')
json.dump(res, open(p + '.tmp', 'w'), indent=1)
os.replace(p + '.tmp', p)
print(json.dumps({k: (v if not isinstance(v, list) else len(v)) for k, v in res.items()}))
EOF

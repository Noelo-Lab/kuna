#!/usr/bin/env bash
# Harness self-checks for a regenerated Kotlin corpus (not needed to regenerate it):
#   fixture (expected/)
#     1 determinism        fixture oracle re-run                                   == expected/
#     2 input order        class files fed in reverse order                        vs expected/
#     3 threads            1 visible CPU (thread-count default 1 instead of 80)    vs expected/
#     4 all plugins        all 3 bundled plugins instead of Kotlin-only            vs expected/
#     5 no classpath jars  include-classpath tests without the kotlin-stdlib jars  vs expected/
#     6 JDK 21             driver JVM = JDK 21 (system modules for include-classpath) vs expected/
#   plain CLI (expected-defaults/)
#     7 driver == CLI      vf-decompile-kt --save folder --plugins --opt include-runtime=current == java -jar
#     8 other plugins      the same with --kotlin-only (idea-not-null/variable-renaming off) vs expected-defaults/
#     9 input order        java -jar, class files reversed                         vs expected-defaults/
#    10 no JDK library     driver defaults with include-runtime=""                  vs expected-defaults/
#    11 JDK 21 runtime     java -jar on JDK 21 (its runtime image)                 vs expected-defaults/
#    12 stdlib library     expected-defaults-stdlib/ (-e=kotlin-stdlib-2.0.0.jar)  vs expected-defaults/
#   compiler
#    13 recompile          compile_sets.py again into a fresh dir                  == classes/
#    14 -jdk-home JDK 21   kt set compiled against JDK 21 instead of 17            vs classes/kt
# Usage: scripts/crosscheck.sh [OUT_ROOT]   (writes OUT_ROOT/reports/crosscheck.json; OUT_ROOT defaults to $CORPUS)
# Env: KUNA_JVM_ORACLE, or the overrides CORPUS, JDKS, VF_SRC, VF_JAR, KOTLINC_HOME; JOBS, WORK.
set -euo pipefail
TOOLS=$(cd "$(dirname "$(readlink -f "$0")")/.." && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
export CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus-kt}
OUT=$(cd "${1:-$CORPUS}" && pwd)
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
VF_SRC=${VF_SRC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower}
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
KOTLINC_HOME=${KOTLINC_HOME:-$JDKS/kotlin/x200/kotlinc}
JOBS=${JOBS:-16}
X=${WORK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/work/kotlin-crosscheck-$(date +%Y%m%d-%H%M%S)-$$}
mkdir -p "$X"
run() { local root=$1; shift
  python3 -I "$TOOLS/scripts/run_oracle.py" --manifest "$OUT/manifest.json" --classes "$OUT/classes" \
    --corpus "$CORPUS" --jar "$VF_JAR" --java "${JAVA_BIN:-$JDKS/jdk17/bin/java}" --jobs "$JOBS" \
    --out-root "$X/$root" --work "$X/work-$root" "$@"; }
run rerun     --mode fixture
run reversed  --mode fixture --order reverse
VF_JAVA_OPTS=-XX:ActiveProcessorCount=1 run onecpu --mode fixture
run allplug   --mode fixture-variant --plugin-mode all
run nocpjars  --mode fixture-variant --cp-extra none --only core-entire-classpath/
JAVA_BIN=$JDKS/jdk21/bin/java run jdk21fix --mode fixture
run cliplug   --mode defaults-driver --plugin-mode all
run ktonly    --mode defaults-driver --plugin-mode kotlin-only
run defrev    --mode defaults --order reverse
run nojrt     --mode defaults-driver --plugin-mode all --extra-opt include-runtime=
JAVA_BIN=$JDKS/jdk21/bin/java run jdk21def --mode defaults
python3 -I "$TOOLS/scripts/compile_sets.py" --vf-src "$VF_SRC" --kotlinc-home "$KOTLINC_HOME" \
  --jdk "$JDKS/jdk17" --m2 "$CORPUS/tools/m2" --classes "$X/recompile/classes" --logs "$X/recompile/logs" \
  --work "$X/recompile/work"
python3 -I "$TOOLS/scripts/compile_sets.py" --vf-src "$VF_SRC" --kotlinc-home "$KOTLINC_HOME" \
  --jdk "$JDKS/jdk21" --m2 "$CORPUS/tools/m2" --classes "$X/jdk21compile/classes" --logs "$X/jdk21compile/logs" \
  --work "$X/jdk21compile/work" --only kt
python3 -I - "$OUT" "$X" <<'EOF'
import json, os, sys, filecmp
out, x = sys.argv[1], sys.argv[2]
m = json.load(open(os.path.join(out, 'manifest.json')))
ok = [t for t in m['tests'] if t['status'] == 'ok']
def cmp(key, root, only=None, other=None):
    sel = [t for t in ok if only is None or only in t['id']]
    return sorted(t['id'] for t in sel
                  if not filecmp.cmp(os.path.join(out, t[key]), other(t) if other else os.path.join(x, root, t[key]),
                                     shallow=False))
def tree_diff(a, b):
    fa = {os.path.relpath(os.path.join(d, f), a) for d, _, fs in os.walk(a) for f in fs}
    fb = {os.path.relpath(os.path.join(d, f), b) for d, _, fs in os.walk(b) for f in fs}
    return sorted((fa ^ fb) | {f for f in fa & fb if not filecmp.cmp(os.path.join(a, f), os.path.join(b, f), shallow=False)})
res = dict(work=x, tests=len(ok),
           fixture_determinism_diffs=cmp('oracle', 'rerun'),
           fixture_reversed_order_diffs=cmp('oracle', 'reversed'),
           fixture_one_cpu_diffs=cmp('oracle', 'onecpu'),
           fixture_all_plugins_diffs=cmp('oracle', 'allplug'),
           fixture_without_classpath_jars_diffs=cmp('oracle', 'nocpjars', only='core-entire-classpath/'),
           fixture_jdk21_diffs=cmp('oracle', 'jdk21fix'),
           cli_vs_driver_with_plugins_diffs=cmp('defaults_oracle', 'cliplug'),
           defaults_changed_by_disabling_other_plugins=cmp('defaults_oracle', 'ktonly'),
           defaults_reversed_order_diffs=cmp('defaults_oracle', 'defrev'),
           defaults_needing_jdk_runtime=cmp('defaults_oracle', 'nojrt'),
           defaults_changed_by_jdk21_runtime=cmp('defaults_oracle', 'jdk21def'),
           defaults_changed_by_stdlib_library=cmp('defaults_oracle', None,
                                                  other=lambda t: os.path.join(out, t['defaults_stdlib_oracle'])),
           recompile_class_diffs=tree_diff(os.path.join(out, 'classes'), os.path.join(x, 'recompile', 'classes')),
           jdk21_jdk_home_kt_class_diffs=tree_diff(os.path.join(out, 'classes', 'kt'),
                                                   os.path.join(x, 'jdk21compile', 'classes', 'kt')))
p = os.path.join(out, 'reports', 'crosscheck.json')
json.dump(res, open(p + '.tmp', 'w'), indent=1)
os.replace(p + '.tmp', p)
print(json.dumps({k: (v if not isinstance(v, list) else len(v)) for k, v in res.items()}))
EOF

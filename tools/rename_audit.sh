#!/usr/bin/env bash
# rename_audit.sh — grep battery for the stage->phase / seam->context rename.
#
# Informational by default (prints counts per check). With --strict, exits 1
# if any "must be zero when the rename is complete" check is nonzero or any
# scope-guard (must NOT have been renamed) check is zero.
#
# Run from the repo root:  tools/rename_audit.sh [--strict]

set -u
cd "$(git rev-parse --show-toplevel)" || exit 2

STRICT=0
[ "${1:-}" = "--strict" ] && STRICT=1
FAIL=0

count() { # count <label> <expect:zero|nonzero> <grep args...>
    local label="$1" expect="$2"
    shift 2
    local n
    n=$(grep -rn "$@" 2>/dev/null | grep -v '/target/' | wc -l)
    if [ "$expect" = zero ]; then
        if [ "$n" -eq 0 ]; then echo "OK    (0)        $label"; else
            echo "RESID ($n)       $label"
            [ "$STRICT" = 1 ] && FAIL=1
        fi
    else
        if [ "$n" -gt 0 ]; then echo "OK    ($n)       $label [guard]"; else
            echo "GUARD-VIOLATION  $label — expected survivors, found none"
            FAIL=1
        fi
    fi
}

CRATES=decompiler/crates

echo "== rename completeness (expect 0 when the rename milestones are done) =="
count "KunaStage/KunaSubStage identifiers"      zero -E '\bKunaStage\b|\bKunaSubStage\b' --include='*.rs' "$CRATES"
count "kuna_stages module refs"                 zero -E '\bkuna_stages\b' --include='*.rs' "$CRATES"
count "SUBSTAGE_TABLE / substage API"           zero -E 'SUBSTAGE_TABLE|lookup_substage|kuna_num_substages|kuna_substage_by_index' --include='*.rs' "$CRATES"
count "SEAM markers in source"                  zero -E 'SEAM' --include='*.rs' "$CRATES"
count "ArchSeam / seams:: refs"                 zero -E 'ArchSeam|\bseams::' --include='*.rs' "$CRATES"
count "stages.toml references (code+scripts)"   zero -E 'stages\.toml' --include='*.rs' --include='*.py' --include='*.toml' "$CRATES" scripts tools
count "emitted JSON keys \"stage\"/\"substage\"" zero -E '"(sub)?stage": ' --include='*.rs' "$CRATES"

echo
echo "== scope guards (upstream vocabulary that must NOT be renamed) =="
count "jumptable multistage recovery fns"       nonzero -E 'recover_multistage|check_for_multistage' --include='*.rs' "$CRATES/kuna-decomp/src"
count "option names intact (compareform)"       nonzero -F '"compareform"' "$CRATES/kuna-decomp"
count "Action/Rule engine type names intact"    nonzero -E '\bActionDatabase\b' --include='*.rs' "$CRATES/kuna-decomp/src"

echo
echo "== residual triage (informational; allowlist = upstream C++ vocabulary) =="
grep -rniE '\bstage\b' --include='*.rs' "$CRATES" 2>/dev/null | grep -v '/target/' \
    | grep -vE 'recover_multistage|check_for_multistage|multistage|Second-stage|two-stage' \
    | head -40
echo "(showing first 40; full count: $(grep -rniE '\bstage\b' --include='*.rs' "$CRATES" 2>/dev/null | grep -v '/target/' | grep -vE 'recover_multistage|check_for_multistage|multistage|Second-stage|two-stage' | wc -l))"

exit $FAIL

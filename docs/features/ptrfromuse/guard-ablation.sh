#!/bin/bash
# Price the constant-addend refusal (analysis.md section 4).
#
# Rebuilds with `const_base_evidence` forced to `No` -- i.e. every constant
# addend forwards the walk, which is the pre-review behaviour -- re-runs the
# `byte` arm over the same 16 binaries the corpus sweep uses, then restores the
# tree and rebuilds.  The per-function diff between $OUT/<bin>.byte.c (guarded)
# and $OUT/<bin>.noguard.c is exactly the set the refusal changes.
#
# Measured 2026-09-17: 9 of 7,523 functions, all bzip2 (bzFile *, fields at
# +0x1394 / +0x13e8).  The typesweep is bit-identical either way.
set -u
WT=${WT:-$(git rev-parse --show-toplevel)}
OUT=${OUT:-$WT/.scratch/sweep}
R=${R:-/home/mahaloz/github/decbench/results/full_run_address_2026-09-11}
export SLEIGHHOME=$WT/specs KUNA_SPECS=$WT/specs
P=$WT/decompiler/crates/kuna-decomp/src/p5_types/kuna_ptrfromuse.rs
mkdir -p "$OUT"
cp "$P" /tmp/ptrfromuse-guard-keep.rs
sed -i 's/match const_base_evidence(data, op, other_vn) {/match BaseEvidence::No {/' "$P"
grep -q "match BaseEvidence::No {" "$P" || { echo "SED_FAILED"; exit 1; }
cd "$WT" && CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=12 make binaries \
  > /tmp/ptrfromuse-ablate-build.log 2>&1 || { echo BUILD_FAILED; cp /tmp/ptrfromuse-guard-keep.rs "$P"; exit 1; }
K=$WT/decompiler/target/release/kuna
for opt in O2 O0; do
  for b in coreutils/fmt coreutils/ls grep/grep gzip/gzip bzip2/bzip2 dash/dash tar/tar diffutils/diff; do
    name=$opt-$(echo "$b" | tr '/' '-')
    $K decompile-all "$R/$opt/$(dirname "$b")/stripped/$(basename "$b")" --option ptrfromuse byte \
      > "$OUT/$name.noguard.c" 2>/dev/null
    echo "$name noguard rc=$? lines=$(wc -l < "$OUT/$name.noguard.c")"
  done
done
cp /tmp/ptrfromuse-guard-keep.rs "$P"
cd "$WT" && CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=12 make binaries \
  > /tmp/ptrfromuse-restore-build.log 2>&1 || echo RESTORE_BUILD_FAILED
grep -q "match BaseEvidence::No {" "$P" && echo RESTORE_FAILED || echo RESTORED
echo ABLATE_DONE

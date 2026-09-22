#!/bin/bash
# attribution builds: 69a947074 (before #692) and eaa19ebbb (#692 merged, before #704/#705/#706)
set -u
W=/home/mahaloz/kwt/_final-bisect
git -C /home/mahaloz/github/kuna worktree add --detach $W 69a947074 >/dev/null 2>&1
cd $W
for c in 69a947074 eaa19ebbb; do
  git checkout -q --detach $c
  find specs -name '*.sla' -type l ! -exec test -e '{}' \; -delete
  for f in $(cd /home/mahaloz/github/kuna && find specs -name '*.sla'); do [ -f "${f%.sla}.slaspec" ] || continue; mkdir -p "$(dirname "$f")"; ln -sf "/home/mahaloz/github/kuna/$f" "$f"; done
  for f in $(find specs -name '*.sla' -type l); do [ -f "${f%.sla}.slaspec" ] || rm -f "$f"; done
  CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_BUILD_JOBS=16 make binaries > /home/mahaloz/kwt/_final-e/bisect-$c.build.log 2>&1
  echo "BUILD $c rc=$?"
  mkdir -p /home/mahaloz/kwt/_final-e/bis-$c
  cp decompiler/target/release/kuna decompiler/target/release/decomp_dbg /home/mahaloz/kwt/_final-e/bis-$c/
done
echo BISECT_BUILDS_DONE

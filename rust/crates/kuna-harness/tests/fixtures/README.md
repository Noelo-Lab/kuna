# B2 lift-diff fixtures

Each `<stem>.b2.txt` is a committed copy of the **C++ oracle's B2 snapshot** for one
corpus function — the `print raw` of the recovered basic-block CFG taken at
`break start heritage` (pre-SSA dataflow, post-flow). They are the reference side of
the structural differential in `../lift_diff.rs`; see `src/liftdiff.rs` for the grammar
and the comparison model.

These are small plain-text files (total < 1 MB). They are committed so the gate runs
without re-snapshotting the C++ oracle.

## Regenerating

The snapshots are produced by `kuna.goldens` (the stage-boundary harness) run against the
**main checkout's** built `decomp_dbg` + `.sla` artifacts. From the main tree
(`/home/mahaloz/github/kuna`, with `decompiler/cpp/decomp_dbg` built and `make specs`
done):

```bash
cd /home/mahaloz/github/kuna
source ~/.virtualenvs/kuna/bin/activate

# Snapshot B2 for every fixture's corpus test (the --xml flag repeats).
python -m kuna.goldens snapshot --boundary B2 --engine cpp \
  --xml lzcount --xml boolless --xml skipnext2 --xml floatprint \
  --xml promotecompare --xml readvolatile --xml condconst --xml gh6990-returnpair \
  --xml ccmp --xml convert --xml nan --xml gh1243-8051-addc \
  --xml condexesub --xml gp --xml sbyte

# Copy the first decompile's B2 dump (seq 000) into the committed fixtures.
SNAP=tests/golden/snapshots/cpp                                  # gitignored
DST=rust/crates/kuna-harness/tests/fixtures
for stem in lzcount boolless skipnext2 floatprint promotecompare readvolatile \
            condconst gh6990-returnpair ccmp convert nan gh1243-8051-addc \
            condexesub gp sbyte; do
  cp "$SNAP/$stem/000-B2.txt" "$DST/$stem.b2.txt"
done
```

The corpus XMLs live under `decompiler/datatests/` (most) and `tests/stages/`
(`gh6990-returnpair`, `gh1243-8051-addc`).  Each fixture's corpus path, language id, and
function entry are pinned in the `CASES` table in `../lift_diff.rs`.

## What the differential needs from the Rust side

`../lift_diff.rs` bootstraps the Rust `XmlArchitecture` frontend from the same corpus
`<binaryimage>` and runs `FlowInfo::generate_ops` + `generate_blocks` + `structure_reset`
(the realized body of the C++ `ActionStart` = `followFlow` + `structureReset`) to the
SAME boundary these snapshots represent, then compares the structural skeleton. It needs
the `.sla` artifacts built (`make specs` in the worktree, or the main tree's `specs/`);
when they are absent every case SKIPs and the gate is a structural pass.

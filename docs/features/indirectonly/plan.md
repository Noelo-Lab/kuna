# indirectonly — implementation plan

## 1. The change

New module `p6_variables/kuna_indirectonly.rs`, a faithful port of two upstream
`Funcdata` methods, named after their originals:

* `check_indirect_use` = `Funcdata::checkIndirectUse` (`funcdata_varnode.cc:801-844`)
* `mark_indirect_only` = `Funcdata::markIndirectOnly` (`funcdata_varnode.cc:845-858`)

One deliberate deviation: upstream uses the Varnode `mark` bit as its visited set
and clears it on the way out. The port keeps the visited set local (`HashSet`), so
an early bailout cannot leave stale marks for a later pass. The traversal order
and the accepted opcode set are unchanged.

## 2. The wiring

`ActionMarkIndirectOnly::apply` (`p6_variables/coreaction_cleanup.rs`) calls it.
The action was already in `universal_sched`, one slot ahead of
`ActionMergeAdjacent` and well ahead of `ActionNameVars`, so **no schedule change
and no `list_action_decompile_oracle.txt` re-record**.

## 3. The gate

`indirectonly` (`on|off`, default **off**), `Architecture::mark_indirect_only` ->
`ArchContext::mark_indirect_only`, read by the action. `off` takes the `if` and
the action is the inert stub it was before, byte-for-byte.

## 4. Why default-off

The first draft shipped it on, on the grounds that it is the upstream body for an
action kuna already schedules. It is — but the behaviour is wrong on a shape that
occurs in the corpus, so the default is off.

The lifted `merge_test_adjacent` refusal is sound only when the illegal input is
the copy's DESTINATION. When it is the SOURCE — the machine loads an escaped
frame slot into a register and mutates the register — the merge makes the emitted
C mutate the slot, so the C claims a store the machine never makes and a later
call handed the enclosing object reads a value that never existed. `analysis.md`
§5-§6 has the mechanism, the two hand-settled corpus instances (`bzip2 -O2
sub_3890`, `kmod -O2-noinline sub_97b0`) and the reason no local test can separate
the two directions. `counterexample.md` reduces it to 40 lines of assembly and
compiles both emitted bodies to show them disagreeing; stock Ghidra 12.1.2 emits
the same fabricated store, so this is an upstream defect the port inherits rather
than a porting error.

## 5. Tests

* `tests/stages/kuna-indirectonly.xml` — two functions in one bytechunk:
  `take` (the escaped slot's entry value reaches only call INDIRECTs) and `keep`
  (identical, except the same slot is read straight into a global first). Pass 1
  `off`, pass 2 `on`. `take` changes; `keep` must not.
* `kuna_indirectonly/tests.rs` — 8 unit tests: the accepting case, the
  direct-read counterexample, MULTIEQUAL transparency, the STORE-INDIRECT
  continuation, a plain INDIRECT's output *not* being followed, no-reads-at-all,
  the directwrite skip, and a MULTIEQUAL cycle terminating.
* `tests/stages/kuna-indirectonly-escape.xml` — the unsound direction as a
  bytechunk. Pass 1 runs the **default** and pass 2 asks for the option, so a
  future default flip turns the test red.
* `docs/features/indirectonly/corpus-escape-classification.txt` — the 34-slice
  A/B classified by the escape predicate (`classify-escape.py`).
  `corpus-hunk-classification.txt` and `skeleton-check.txt` are the superseded
  round-1 artifacts, kept with a header saying what they could not see.

## 6. Counters and docs

`counters --rederive` / `--fix` on a freshly built tree (settables 213 -> 214),
`kuna catalog --markdown > docs/options.md`, the stages corpus count in
`kuna-base/src/xml.rs`, `docs/baseline-stages.json` re-recorded, and the prose in
`docs/spec/06-variables-and-merge.md` §6.1 — where the sentence naming
`ActionMarkIndirectOnly` as an inert stub is replaced by the description of what
it now does.

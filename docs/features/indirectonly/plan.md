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

`indirectonly` (`on|off`, default **on**), `Architecture::mark_indirect_only` ->
`ArchContext::mark_indirect_only`, read by the action. `off` takes the `if` and
the action is the inert stub it was before, byte-for-byte.

## 4. Why default-on

It is the upstream body for an action kuna already schedules, and the evidence
clears the bar: 0/675 datatest assertions change, stages PARITY OK, ten changed
functions over the 16-slice corpus with every hunk a merge that removes a copy
(plus one wrong-output fix), no BUG hunk, and speed flat.

## 5. Tests

* `tests/stages/kuna-indirectonly.xml` — two functions in one bytechunk:
  `take` (the escaped slot's entry value reaches only call INDIRECTs) and `keep`
  (identical, except the same slot is read straight into a global first). Pass 1
  `off`, pass 2 `on`. `take` changes; `keep` must not.
* `kuna_indirectonly/tests.rs` — 8 unit tests: the accepting case, the
  direct-read counterexample, MULTIEQUAL transparency, the STORE-INDIRECT
  continuation, a plain INDIRECT's output *not* being followed, no-reads-at-all,
  the directwrite skip, and a MULTIEQUAL cycle terminating.
* `docs/features/indirectonly/corpus-hunk-classification.txt` and
  `skeleton-check.txt` — the whole-corpus A/B and its structural control.

## 6. Counters and docs

`counters --rederive` / `--fix` on a freshly built tree (settables 213 -> 214),
`kuna catalog --markdown > docs/options.md`, the stages corpus count in
`kuna-base/src/xml.rs`, `docs/baseline-stages.json` re-recorded, and the prose in
`docs/spec/06-variables-and-merge.md` §6.1 — where the sentence naming
`ActionMarkIndirectOnly` as an inert stub is replaced by the description of what
it now does.

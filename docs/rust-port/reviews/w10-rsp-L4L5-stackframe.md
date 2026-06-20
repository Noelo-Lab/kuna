# Review — `rport/w10-rsp-L4L5-stackframe` (RSP L4 + L5 tail, stack-frame infra)

**Verdict: ACCEPT_WITH_LOSSES** (round 1, independent verifier)

Branch `rport/w10-rsp-L4L5-stackframe`@7f1f4df, parent `rust-port`@477d6f3
(merge-base == parent HEAD, clean descendant). This is the faithful **PARTIAL**
the verdict criteria anticipate: **L4 ported + live, L5 input-effect-marking
faithfully transcribed but gated OFF** behind a printc/render seam — **+0
substrate, zero regression, keystone held**. Single loss recorded (LOSS-225).

## What was verified (independent runs, not trusting the porter)

### 1. FAITHFULNESS — line-faithful to the C++ oracle
Each ported function cross-checked against the cited C++:

| Rust | C++ oracle | Faithful? |
|---|---|---|
| `Funcdata::restrict_local` (funcdata_spacebase.rs) | `ActionRestrictLocal::apply` coreaction.cc:2003-2059 | YES — both loops transcribed: loop1 input-locked-call spacebase/JOIN param excision (`wrapOffset(spacebaseOffset+off)`), loop2 unaffected-input COPY sweep, `killedbycall` skip. Snapshot-before-mutate is borrow-safe (loop2 mark doesn't alter the descend list). |
| `ScopeLocal::mark_not_mapped` (varmap.rs) | varmap.cc:510-545 | YES — `last = first+sz-1` (uintb wrapping), wrap/clamp at `getHighest()`, parameter→min/maxParamOffset extension. |
| `Database::mark_not_mapped_core` (database.rs) | varmap.cc:527-545 symbol loop | YES — `findOverlap`/`removeSymbol` loop, typelock→return, fake_input→return, then `removeRange`. Warning channel is a documented W5 seam; the early `return` (load-bearing) is preserved. |
| `ScopeLocal::is_unaffected_storage` (varmap.rs) | varmap.hh:244 `vn->getSpace()==space` | YES — Rc::ptr_eq OR index-equality (correct identity proxy for Rc-cloned AddrSpaces; not special-casing). |
| `Funcdata::check_unaliased_return` + `alias_reaches_return_slot` (funcdata_spacebase.rs) | varmap.cc:414-428 | YES — `getFirstReturnOp`, `numInput()<2` guard, `getIn(1)`, space guard, `lower_bound(alias,off)` == `partition_point(a<off)`, `*iter <= off+size-1` inclusive end, else `markNotMapped`. |
| `FuncProto::has_effect`/`effect_list` (fspec.rs) | fspec.cc:4239-4262 | YES for the live path. The Rust `model: Option` `None` arms (UNKNOWN_EFFECT / empty slice) only protect model-less test fixtures; in the real pipeline `model` is always present so behavior is identical. Documented. |
| `apply_input_effect_marking` (funcdata_varnode.rs) | `setInputVarnode` `funcp.hasEffect` tail | YES (transcription) — but **GATED OFF** (see Losses). |

`OFFSET_UNKNOWN = 0xBADBEEF` == C++ `offset_unknown`; `num_params`/`get_param`
delegate to the same proto store as C++ `numParams`/`getParam`. All checked.

### 2. NO SPECIAL-CASING
grep over every added line (incl. the 3 non-reserved seam files) for
`rsp`/`register:0x20`/register names/offsets/function names/`get_value_byref`/
`switchhide`: **none in live code** (only in doc/seam comments). Every guard keys
on Symbol category/typelock + AddrSpace identity + spacetype — purely structural.

### 3. REAL PARITY — datatest cluster re-run, inspected vs oracle
- Full Rust datatest corpus (`decomp_test_dbg`, 83 files): **397/675** on branch
  AND **397/675** on `rust-port` base (independent rebuild of base from clean HEAD).
- Per-assertion passing-set diff (base vs branch): **regressed-set EMPTY,
  new-pass-set EMPTY — byte-identical 397-element passing set.**
- Switch Indirect: #1-7, #9-16 PASS (**15/16**); only #8 `default:` FAILs — and #8
  ALSO fails on base, so 15/16 is the keystone state, NOT a regression. **#15/#16
  GREEN** confirmed on both. Switch Multi #1 GREEN.

### 4. NO REGRESSION + KEYSTONE HELD
- `cargo test --workspace`: **3653 passed / 0 failed / 37 ignored** (independent run).
- **Full-corpus rendered-C byte comparison** (KUNA_DUMP over all 83 files, base vs
  branch, paths normalized): the ONLY real delta is `longdouble.xml` — L4
  `markNotMapped` correctly excises a spurious `xunknown2 v3; // stack - 0x10`
  local, inlining `CONCAT(SUB(...))`. Directionally TOWARD the oracle (which has no
  such local); flips no assertion. **switchind / boolless / readstruct / condconst:
  byte-identical.**
- Keystone guard tests GREEN: `w10_spacebase_typing_switchind_passing_set_unregressed`,
  `switchhide_callarg_render_delta_pinned`.
- Reserved files (`jumptable.rs`, `funcdata.rs`, `funcdata_block.rs`,
  `coreaction_casts.rs`, `printc.rs`): **UNTOUCHED**.
- C++ oracle: `python -m kuna.run_tests --all --baseline` → **207/207 unit +
  675/675 datatest, PARITY OK** (only `rust/` is touched; C++/specs/baseline/B0 unchanged).
- `cargo clippy --lib -p kuna-decomp`: clean.
- Adversarial tests: **7** (4 L4 + 3 L5), all pass, none ignored — exercise the
  actual ported functions (symbol removal, typelock preservation, space-identity,
  uintb wrap, the lower_bound/inclusive-end alias-overlap decision). Real, not fabricated.

## Findings (non-blocking)
- **F1 (scope, non-reject):** the diff touches 3 files beyond the stated
  `coreaction_protos.rs`/`varmap.rs`/`funcdata_spacebase.rs`+tests list —
  `database.rs` (markNotMapped symbol-removal core, structurally must live on the
  owned Database), `fspec.rs` (Option-safe `has_effect`/`effect_list` deref),
  `funcdata_varnode.rs` (the gated L5 input-marking). **None is a reserved file**
  and all three changes are faithful + net-safe, so this is not a reject condition;
  flagged for the record.

## Loss
- **LOSS-225**: L5 `setInputVarnode` input-effect-marking transcribed but held
  behind `INPUT_EFFECT_MARKING_ENABLED = false` (printc/render seam) — activating it
  alone is net-negative (regresses `switchhide` call-arg render) until the L5
  stack-frame typing / `annotateRawStackPtr` cast-plane chain lands. `restrict_local`
  loop 2 inert in the meantime (faithful for the current IR). +0 net. One-call flip.

## Why ACCEPT_WITH_LOSSES (not ACCEPT, not REJECT)
Faithful PARTIAL (L4 live, L5 printc-gated) with 0 regression + keystone held =
ACCEPT_WITH_LOSSES per the stated criteria. Not REJECT: keystone holds (switchind
15/16, #15/#16 green, byte-identical switch cluster), no masked regression
(passing set byte-identical, the only render delta is toward-oracle), no
special-casing, no reserved file touched, nothing fabricated. Not plain ACCEPT:
LOSS-225 (L5 marking deferred behind the render seam).

# RSP/Spacebase Keystone — Independent Verification (THE KEYSTONE LANDING)

**Verdict: ACCEPT_WITH_LOSSES** (faithful + zero regression + 4/5 fence updates are
justified pre-keystone residue; the 5th — AT4 longdouble — is borderline, noted below).

Branch `rport/w10-rsp-5layer-atomic` @ `bf4ca27`. Built and run INDEPENDENTLY in the
worktree `/tmp/kuna-rsp5` (`cargo build --release -p kuna-console -p kuna-harness` →
`decomp_dbg`/`decomp_test_dbg`, exit 0). Baseline (`rust-port`, 384) built from the main
tree. C++ oracle = the committed `decompiler/cpp/decomp_test_dbg`. Nothing trusted from
the porter; every claim re-derived from the live engines.

---

## (1) switchind — CERTIFIED 15/16

Re-ran `switchind.xml` through the worktree harness:

| # | assertion | bf4ca27 | rust-port 384 | C++ oracle |
|---|---|---|---|---|
| #1–7 | `case 0:`…`case 10:` | PASS | PASS | PASS |
| **#8** | `default:` | **FAIL** | **FAIL** | PASS |
| #9–14 | `casefunc*()` | PASS | PASS | PASS |
| **#15** | `get_value_byref(&val);` | **PASS** | FAIL | PASS |
| **#16** | `switch(val)` | **PASS** | FAIL | PASS |

- **#8 (`default:`) fails on BOTH bf4ca27 AND the rust-port-384 baseline** → it is **NOT a
  regression**. It is the separate guard-fold seam (CORRECTION-8; `jumptable.rs
  fold_in_one_guard` is ported but `#[allow(dead_code)]`/unwired). The C++ oracle is 16/16.
- **#15 / #16 flipped FAIL→PASS** (+2), exactly the keystone's headline.

**KUNA_DUMP B4 render (worktree engine), verbatim:**
```
void switchind(void)
{
  undefined8 v1; // rsp
  int4 val; // stack - 0xc        <-- slot -0xc (B4), single int4 decl
  val = 0;
  get_value_byref(&val);          <-- #15: NO (int4 *) cast
  if ((uint4)val <= 10) {
    switch(val) {                 <-- #16: symbolic, not switch(0x100058)
      ...
```
Slot is `-0xc` (the corrected B4 slot, not the pre-keystone `-0x14`); `&val` carries **no
`(int4 *)` cast**; `switch(val)` is symbolic. Single `int4 val` — the int8/int4
duplicate-HighVariable collision is gone.

---

## (2) +13 datatests — NO MASKED REGRESSION (strict superset confirmed)

Full per-assertion passing sets captured from BOTH engines and diffed:

- bf4ca27: **397 passing** / 675. rust-port baseline: **384 passing** / 675.
- **`comm -23 baseline_pass bf4ca27_pass` (REGRESSED) = EMPTY.** Literally zero assertions
  that passed on baseline now fail. 397 = 384 + 13 is a **strict superset**.

**The 13 gained:** Concat split #2,#3,#4,#5,#6 · For-loop thru special #2 · Indirect
prototype #3 · No for-loop alias #4 · Partial splitting #3 · Pointer to array #9 · Switch
Indirect #15,#16 · Union #18.

**loopcomment:** `KUNA_DUMP` render shows **single decls** for every name (`v1`,`v2`,`v3`
each declared once); **no duplicate vN** — the CORRECTION-7 `3× v2 int8/int8/int4`
pathology is cured. loopcomment's datatest assertions fail on BOTH commits (comment
placement, unrelated) → not a regression (consistent with the empty regressed set).

---

## (3) THE STALE-FENCE UPDATES — 4 justified-residue, 1 borderline

Each fence-bearing test was run under BOTH the worktree engine and the C++ oracle.

| fence (file) | old → new | oracle-faithful? | verdict |
|---|---|---|---|
| **a1b forloop1** (spacebase_render) | "dirty `// rsp` present" → "`// rsp` cleaned, `int4 v1; // ebx`, reroll still inert (no `for(`)" | Oracle decl is `int4 v1; // ebx` (matches). Oracle DOES reroll `for(...)`; Rust stays `while/break` (For-loop #1 still FAIL on both). | **JUSTIFIED** — the RSP residue is genuinely cleaned to the oracle decl; the test is honest that reroll is a still-open gap (no masked flip). |
| **vfy_r1 partialsplit** (spacebase_render) | "`PTRSUB(v1,` must survive" → "`&stackother` renders, no `PTRSUB(v1,`" | Oracle renders `&stackmy`,`&stackother` as `&name`. Rust now matches the `&name` forms. Partial splitting #3 flips green (+1 net). | **JUSTIFIED** — keystone recovered `&stackother`; oracle-faithful. |
| **nanops** (input_params) | `void nanops(float8)` → `void nanops(float8,float8)` | Oracle: `void nanops(float8 a0,float8 a1)` — both float8 params. Arity matches. | **JUSTIFIED** — keystone recovered the 2nd XMM float arg. |
| **forloop_varused** (struct_corpus) | "64-bit signal = `// rsp`/RSP token" → "also accept `// ebx` / `(uint8)`/`(int8)` cast" | Oracle: `uint4 v1; // ebx` + `printf(...,(uint8)v1)`. Rust render matches both signals exactly. Widened proxy still requires a real lift + realmode==0 + a loop keyword. | **JUSTIFIED** — `// rsp` residue genuinely cleaned to oracle `// ebx`; the `(uint8)` it accepts is genuinely present and oracle-faithful. |
| **AT4 longdouble** (callarg_piece) | "float10 arg DROPPED (`writeLongDouble(ldarr)` only)" → "arg reconstructed (`writeLongDouble(ldarr,` + `CONCAT`)" | Oracle: clean `writeLongDouble(ldarr,x)`. Rust: `writeLongDouble(ldarr,(undefined10)CONCAT(z,CONCAT(v1,x)))` — arg now PRESENT but a **messy CONCAT, not the oracle's clean `x`**. **Long double #3/#4 still FAIL on both commits** → no datatest gained. | **BORDERLINE** — directionally toward the oracle (arg appears vs absent), but NOT byte-faithful to the oracle render and gains no assertion. **NOT a masked regression** (the Long double passing set is byte-identical bf4ca27 vs baseline). |

`verify_w10_jts_chain.rs` is **purely additive** (ADV-1 no-duplicate-decl) — a stronger
guard, not a fence change. `verify_w10_spacebase_render.rs` also adds ADV-2/ADV-3
(no-spurious-pointer-cast, distinct-named-locals) — additive.

No fence was weakened to hide a regression: every flipped fence corresponds to a genuine,
oracle-verified cleanup, and the regressed set is provably empty.

---

## (4) FAITHFULNESS — every layer line-cited to C++

- **Effect-block decode (ROOT-A)** `architecture.rs::decode_default_proto` +
  `decode_effect_block` — faithful to `ProtoModel::decode` (fspec.cc:2620-2643
  UNAFFECTED/KILLEDBYCALL/RETURNADDRESS arms) + the `!sawretaddr && defaultReturnAddr`
  fallback (fspec.cc:2694-2696). Instrument-proven by the live render (RSP effecttype 4→1,
  slot -0xc). This is the true root every prior attempt missed.
- **check_call_double_use (ROOT-B #1)** `funcdata_varnode.rs` — verbatim transcription of
  `funcdata_varnode.cc:1802` (getSlot/getCallSpecs/getEntryAddress/
  getTrialForInputVarnode/isInputActive/isAlternatePathValid), each C++ line cited inline.
- **createPlaceholder / opStackLoad / createStackRef / newSpacebasePtr (ROOT-B #2)**
  `fspec.rs`/`funcdata_op.rs` — faithful to `funcdata_op.cc:459/541` (INT_ADD(spacebase,
  byteToAddress(off)); LOAD; getSpacebase(0)); SEGMENTOP arm documented as an unreached
  flat-stack seam (correct for x86-64).
- **index-based ActionActiveParam (ROOT-B #3)** `funcdata_callsite.rs` — uses
  `get_call_specs(idx)`/`get_call_specs_mut(idx)` keeping specs on `data.qlst`, NOT
  `take_call_specs` (which would break cross-call lookup); clone-then-writeback for
  ancestor paths. Matches CORRECTION-7's architecture requirement exactly.
- **L0 ActionExtraPopSetup + L1 setEffectiveExtraPop** `coreaction_protos.rs` — verbatim
  `coreaction.cc:1452-1482` (INT_ADD if known extrapop, INDIRECT otherwise; the
  `setEffectiveExtraPop` write).
- **heritage INDIRECT un-gate** `heritage.rs` — drops `persist_range &&` so the INDIRECT
  emits for every `unknown_effect||return_address` range, faithful to `heritage.cc:1514`
  (no persist restriction). Verified against the C++ source.
- **spacebase-aware propagate_add_in2_out** `coreaction_infertypes.rs` — reproduces
  `TypePointer::downChain` (type.cc:1224-1257) dispatch to `TypeSpacebase::getSubType`
  (type.cc:1248) + `getTypePointerStripArray` (the `!isArray` arm). The int8/int4 fix.
- **const-sibling decl-skip** `printc.rs` — selective dedup of an all-constant `&symbol`
  reference high that shadows a real storage-backed sibling of the same name; the
  whole-sibling guard is load-bearing (the stack-array `c` in passPtrToArray, with no
  storage sibling, still prints). A justified compensating render fix, well-guarded.

**No special-casing.** Exhaustive grep over ALL added source lines for `RDI`/`0x38`/`0xc`/
`0x14`/`value`/`address`/`switchind`/`get_value_byref`/`casefunc`/`nanops`/`passmany`/
`partialsplit`/`forloop`/`loopcomment`/`stackother`/`0x100058`/type-name string literals →
**zero hits**. The matched names all derive dynamically from `h.kuna_name()`/proto effect
records.

**The reserved fold rule (`ruleaction_3.rs RuleAddMultCollapse`) is UNTOUCHED** (empty
diff). The `ruleaction_4.rs` change is the faithful `ruleaction.cc:4316-4324`
spacebase-placeholder tail (resolveSpacebaseRelative wire), C++-cited.

---

## (5) ALL-GREEN GATES

- **cargo test --workspace (debug): 3646 passed, 0 failed.** All five fence-bearing
  verify_w10 suites green. (Release-profile run shows ONE failure —
  `constseq::tests::w6s5_form_byte_array_huge_bytepos_diverges_debug_panic`, a
  `#[should_panic(expected="add with overflow")]` test that relies on debug overflow
  checks; it ALSO fails in `--release` on the rust-port baseline and constseq.rs is
  UNTOUCHED by the keystone → a pre-existing profile artifact, not a keystone regression.)
- **cargo clippy --workspace --lib -- -D warnings: clean (exit 0, 0 warnings).**
- **C++ oracle `run_tests --all --baseline docs/baseline.json`: 207/207 unit +
  675/675 datatests, PARITY OK.** The keystone diff is **rust-only** — `git diff` over
  `decompiler/`/`specs/` is empty (C++ tree untouched).
- **boolless / condconst2 byte-identical** across bf4ca27 vs baseline (body diff = only the
  KUNA_DUMP path-header line). (No `readstruct.xml` exists in the vendored tree; the
  byte-identity intent is covered by boolless/condconst.) B0 structural shape unchanged.
- **No leftover instrumentation** (`KUNA_L0DBG`/`KUNA_RSPDBG`/`eprintln!`/`dbg!`): zero hits
  in the committed source.

---

## Verdict rationale

Every hard gate passes: switchind 15/16 (#8 a pre-existing non-regression), +13 strict
superset with an **empty regressed set**, all layers line-faithful with no special-casing,
the reserved fold rule untouched, full test/clippy/oracle-parity green, C++ tree untouched.
Four of the five fence updates are unambiguous justified pre-keystone residue (oracle-
verified cleanups). The fifth — **AT4 longdouble** — is borderline: the keystone moved the
float10 call-arg from *dropped* to *present-but-messy-CONCAT*, which is directionally toward
the oracle and masks no regression (the Long double passing set is byte-identical across
commits, and #3/#4 still fail on both), but the new render is not the oracle's clean
`writeLongDouble(ldarr,x)`. Per the protocol — faithful + 0-regression but one fence update
borderline — the verdict is **ACCEPT_WITH_LOSSES**.

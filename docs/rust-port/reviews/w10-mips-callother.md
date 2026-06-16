# w10-mips-callother
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-16
branch: rport/w10-mips-callother (substantive commit f7d3784; verifier test commit 28ab39d)
base: merge-base d852f06 (this branch is exactly 1 substantive commit ahead of it; rust-port tip c4bea8f adds only a PROGRESS.md doc commit, no code)

gate: `cd rust && cargo test --workspace` -> exit 0, 3538 passed / 0 failed
        (= 3534 base + 4 new verifier adversarial tests).
      `cargo test -p kuna-decomp --test verify_w10_mips_callother` -> 4 passed / 0 failed.
      Whole-suite datatest parity (verifier-run Rust `decomp_test_dbg`, full
        corpus, `-path decompiler/datatests -sleighpath specs`):
          base d852f06   = 675 applied / **290** passing
          this branch    = 675 applied / **293** passing
        passing-SET diff (`comm` of the `Success` sets): newly-passing = exactly
        {MIPS Bitfields #4, #5, #31}; **regressed = EMPTY**. 0 new exec-failures
        (675 applied on both; no datatest file errored/dropped).
      `cargo clippy -p kuna-decomp -p kuna-sleigh --lib` -> clean (0 warnings, 0 errors).
      C++ oracle: branch touches ONLY rust/ (`git diff --name-only rust-port...HEAD`
        has no non-rust path; `git diff -- decompiler/ specs/ docs/baseline.json`
        empty) -> decomp_test_dbg 207/207 + 675/675 PARITY OK by construction, untouched.

## What this branch is (read the diff against the C++, not the name)

The wave wires the C++ callother-fixup injection path end-to-end so an
`injected`-type user-op CALLOTHER (the MIPS `setISAMode` jr-ra-delay-slot
ISA-mode-switch syscall) is replaced by its cspec `<callotherfixup>` p-code and
then dead-code-eliminated — exactly as upstream Ghidra. The elimination is NOT
in ActionDeadCode (an output-less CALLOTHER is never swept by the consume
fixpoint); it happens at flow time (flow.cc:362 queues an `injected` CALLOTHER
onto `injectlist`; `injectPcode` -> `injectUserOp` -> `doInjection`,
flow.cc:1179/1214, weaves the `v0 = v0;` incidental-copy body in and
`opDestroyRaw`s the original; the dead COPY then folds away).

6 Rust files, +522/-8:
- `sleighbase.rs` (+67): `impl SnippetLanguage for SleighBase` (pcodeparse.cc
  PcodeSnippet::lex symbol classification + the const/unique/default-code space
  accessors) so a fixup body compiles against the loaded language.
- `inject_sleigh.rs` / `pcodeinject.rs` (+82/+18): `parse_inject_all` compiles
  every registered payload's `parsestring` into a `ConstructTpl` (parseInject)
  and stores it parallel to `injection`; `SnippetLanguageProvider` bridges the
  compile without the library struct naming the kuna-sleigh trait.
- `architecture.rs` (+137): `init_userops_and_fixups` = `userops.initialize`
  (architecture.cc:641) + the `<callotherfixup>` dispatch (architecture.cc:1294)
  + parseInject, wired into `init_post_engine` after `decode_call_fixups`; plus
  the `UseropArchitecture`/`InjectArchitecture` glue the decode path needs.
- `flow.rs` (+169): `inject_user_op` + `do_injection` (faithful flow.cc:1214/1179)
  and the three `FlowEnvironment` hooks (`is_injected_userop`,
  `is_incidental_copy_userop`, `inject_userop`) routed to the arch user-op table
  + inject library via `decompile_drive.rs::ArchFlowEnv`.

No reserved file touched. `git diff --name-only rust-port...HEAD` = exactly the
6 files above (+ the verifier test) — variable.rs, funcdata_varnode.rs,
printc.rs (no pushPartialSymbol / member-access arm), unionresolve.rs, dtype.rs
(the concurrent union-value wave) are ALL ABSENT.

## The +3 PASSES are REAL (the central parity verification, re-run from scratch)

The fixture is `decompiler/datatests/bitfields2.xml` (`MIPS:BE:32`, "MIPS
Bitfields #1..#31"). Before this wave, `dosomething`/`increment` carry a
spurious `CALLOTHER(0,(ra & 1) != 0);` that blocks the container field stores
(#4 `ptr->field5 = val + 0x14;`, #5 `ptr->field7 = 100;`) and the
single-statement return (#31 `ip->field5 = ip->field5 + 1;`).

Verifier ran the Rust `decomp_test_dbg` on `bitfields2.xml` on BOTH trees (full
script applied — `parse line` struct typing + `lo fu` prototype override):
  base d852f06: 13/31 passing; #4, #5, #31 = **FAIL**.
  this branch : 16/31 passing; #4, #5, #31 = **Success**.
Full-corpus passing-SET diff confirms these three are the ONLY deltas and that
NO base-passing test regressed. The gain is not fabricated and not a
weakened/looser assertion — it is the upstream B5 oracle text matching.

(Note: a verifier adversarial test that bootstraps the Architecture WITHOUT the
datatest `<script>` shows the CALLOTHER eliminated but the stores rendered as
raw bitmask arithmetic, not `ptr->fieldN`. That is expected — the member-access
text needs the script's struct-type import + prototype lock; the authoritative
`decomp_test_dbg` applies it, hence the harness, not the script-less bootstrap,
is the parity oracle. The adversarial tests below assert the script-independent
core — that the CALLOTHER is gone.)

## NO special-casing (the primary REJECT trigger — clean)

The elimination keys on the user-op's DECLARED `injected` type, never on the
operand. Verified:
- The CALLOTHER is queued onto `injectlist` (flow.rs:1084) iff
  `is_injected_userop(index)` -> `userops.get_op(index).get_type() == injected`
  — the exact flow.cc:364 query. `index` is the user-op TABLE index (in0's
  offset), not the `(ra & 1)` data operand.
- `setISAMode` becomes `injected` ONLY because the cspec `<callotherfixup
  targetop="setISAMode">` is decoded (`decode_call_other_fixup` ->
  `decode_injected`, userop.rs, faithful to userop.cc:86-99): it reads the
  inject id, resolves `getCallOtherTarget` -> the name, requires the base op be
  Unspecialized, takes the base index, and `register_op` REPLACES the base entry
  at that index. No name/address/opcode hardcode.
- grep over ALL touched src (flow.rs / decompile_drive.rs / inject_sleigh.rs /
  architecture.rs / sleighbase.rs / pcodeinject.rs) for `(ra & 1)` /
  `"setISAMode"` / `"main"` / `getName()=="` / `get_offset()==` / a CALLOTHER
  operand match: ZERO hits in executable code. Every `setISAMode` mention is a
  doc comment. The only integer literals on the injection path are the SLEIGH
  temp base `0x2000` (parseInject, faithful to inject_sleigh.cc:398) and flag
  bits — not data-dependent constants.
- Adversarial table-walk test (`..._non_fixed_up_userop_is_not_injected`)
  confirms: every `injected` op in the MIPS table carries an inject id (no op is
  injected without a `<callotherfixup>`), and the bulk of the table stays
  `unspecialized` — the elimination is selective on the declared type, not blanket.

## Faithfulness (line-by-line against the C++ in scope)

- `do_injection` (flow.rs) == doInjection (flow.cc:1179-1210) statement-for-
  statement: `marker = endDead(); --iter` -> `dead_tail()`; `inject`; `++iter`
  first-op (-> `dead_next(marker)`/`dead_head`) with the **Empty injection** throw
  preserved; `startbasic = op->isBlockStart()`; `xrefControlFlow`; the
  `startbasic` next-op `opMarkStartBasic`; `if isIncidentalCopy markIncidentalCopy
  (firstop,lastop)`; `moveSequenceDead(firstop,lastop,op)`; `updateTarget(op,
  firstop)`; `opDestroyRaw(op)`. Ordering identical (mark-incidental BEFORE move,
  destroy LAST). The marker is captured pre-inject and `dead_next` computed
  post-inject — faithful to the iterator `++` after the append.
- `inject_user_op` (flow.rs) == injectUserOp (flow.cc:1214-1238): in0 offset ->
  userop index; icontext baseaddr=nextaddr=op addr; `for i in 1..numInput` skips
  the inject-id annotation slot 0 (C++ `for(i=1;...)`), pushing each input's
  `{space,offset,size}`; the optional output. Faithful.
- `parse_inject` (inject_sleigh.rs) == parseInject (inject_sleigh.cc:373-416):
  input-then-output `add_operand` order; EXECUTABLEPCODE_TYPE -> `set_unique_base
  (0x2000)` and does NOT write back tempbase; else `set_unique_base(tempbase)` and
  `tempbase = get_unique_base()`. `parse_inject_all` threads the advancing
  `base.tempbase` across payloads in `injection` insertion order (Vec index) —
  matching the C++ per-payload `parseInject` thread.
- `decode_injected`/`register_op` == userop.cc:86-99 / the index-override
  registration. Verified the override replaces by index (useroplist[ind]),
  preserving the index/name crossref.

## Mandatory hunt list

- **Signedness**: clean. `index` flows `get_offset()` (uintb) -> `get_op(_ as
  uint4)`. C++ casts the same offset `(int4)` for `getOp`. For any valid user-op
  index (< 2^31) `as int4` and `as uint4` are value-identical; `get_op(i:uint4)`
  gates on `(i as usize) < len`. Benign signed/unsigned divergence that cannot
  differ on the real index space. The `1..numInput` operand loop and
  `0..injection.len()` compile loop use unsigned/usize bounds (no signed index vs
  size compare).
- **Integer widths**: clean. inject ids / userop indices / param sizes are small;
  `as int4` casts (param size, inject id) are documented and faithful to the C++
  `int4` reads. No int4*int4->intb promotion on the injection path.
- **Wrapping**: clean — none needed, none added. No `uintb`-lineage arithmetic in
  the injection bodies (the p-code values flow through the SLEIGH ConstructTpl,
  not Rust arithmetic). `tempbase` advances monotonically via
  `get_unique_base()`, no Rust add.
- **Comparator totality**: N/A — no `operator<`/comparator ported in this wave.
- **Iteration-order provenance**: clean, no HashMap/HashSet in the diff. The
  `<callotherfixup>` filter walks `root.get_children()` in XML document order
  (== C++ Decoder child order). `parse_inject_all` walks `injection` by Vec index
  (== C++ registration/insertion order). The operand loop is index `1..numInput`.
  The dead-list walks (`dead_tail`/`dead_next`/`dead_head`) are the obank list
  order. No nondeterministic container in ported logic.
- **Off-by-one / do-while / reverse-iterators**: clean. `do_injection`'s
  `--iter`/`++iter` marker dance is reproduced via `dead_tail`+`dead_next` with
  the at-least-one-op invariant (`op` itself) holding; the Empty-injection throw
  preserves the C++ guard. No do-while/rbegin idioms.
- **Erase-while-iterating**: clean. `inject_pcode` walks `injectlist` by index
  with the C++ nullify-on-visit (`injectlist[i] = 0`), re-reading `.len()` each
  iteration to absorb nested inlines pushed mid-walk (faithful to the C++
  `for(i=0;i<injectlist.size();++i)`). `do_injection` mutates the obank
  (move/destroy) but not the list it iterates.
- **Exception -> Result partial-state**: clean. `init_userops_and_fixups`
  restores `self.userops`/`self.pcodeinjectlib` on the error path before
  propagating (the split-borrow mem::take is balanced on both Ok and Err). The
  Empty-injection / non-compiled-template errors are `Err(...)` at the same points
  the C++ throws.

## Mechanical pass

- `grep todo!|unimplemented!|HashMap|HashSet|sort_unstable|partial_cmp|float-unwrap`
  over the diff: ZERO hits.
- bare `as` casts in the diff: 8, all justified — param-size/inject-id `as int4`
  carry inline `// cast:`-style comments faithful to the C++ int4 reads;
  `get_size() as u32` for VarnodeData.size; `id as usize` for slice index. No
  unjustified bare cast.
- `cargo clippy -p kuna-decomp -p kuna-sleigh --lib` clean. (A pre-existing
  clippy `--tests` error at heritage.rs:3232 `!i.is_heritaged() || true` exists
  IDENTICALLY on the base and in an untouched file — not introduced by this wave;
  the `--lib` requirement is met.)

## Adversarial tests (verifier-authored, this round; landed @ 28ab39d in tests/verify_w10_mips_callother.rs)

All 4 pass against the branch (ACCEPT — no failing test / no divergence):
- `w10_mips_callother_dosomething_eliminates_the_callother` — bootstraps the MIPS
  bitfields Architecture and decompiles `dosomething`; asserts the body no longer
  contains `setISAMode`/`CALLOTHER`/`ra & 1` (it did before this wave) and keeps
  its real assignments. The script-independent core effect.
- `w10_mips_callother_increment_eliminates_the_callother` — same for `increment`.
- `w10_mips_callother_setisamode_is_marked_injected_in_the_userop_table` — proves
  the cspec decode (not a hardcode) marks `setISAMode` `injected`, carrying an
  inject id, and its payload is incidental-copy (cspec `incidentalcopy="true"`).
- `w10_mips_callother_non_fixed_up_userop_is_not_injected` — table-wide: no op is
  `injected` without an inject id; most ops stay `unspecialized`. The
  no-special-casing / general-mechanism gate.

findings:
  - F1 (informational, non-blocking): the `parse_inject_all` skip-condition is
    `parsestring.is_empty()` rather than the C++ `payload->isDynamic()` early
    return. For the in-scope payload set (callfixup/callotherfixup, never
    dynamic) the two coincide; a hypothetical dynamic payload with a non-empty
    parsestring would be compiled instead of skipped. Not triggered by any cspec
    inject in the corpus (the InjectPayloadDynamic path is a debug-only seam,
    already noted in-source). No behavior difference observed; recorded for the
    future dynamic-payload wave.
       cpp: decompiler/cpp/inject_sleigh.cc:376 (`if (payload->isDynamic()) return;`)
       rust: rust/crates/kuna-decomp/src/inject_sleigh.rs:492 (`if parsestring.is_empty() { continue; }` in `parse_inject_all`)

adversarial tests: w10_mips_callother_dosomething_eliminates_the_callother,
  w10_mips_callother_increment_eliminates_the_callother,
  w10_mips_callother_setisamode_is_marked_injected_in_the_userop_table,
  w10_mips_callother_non_fixed_up_userop_is_not_injected

losses: none (clean ACCEPT — +3 real parity, 0 regression, 0 reserved-file edits,
  faithful general mechanism; no losses.md entry required).

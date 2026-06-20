# w10-inline-body
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-15
gate: `cd rust && ./target/debug/decomp_test_dbg -sleighpath ../specs -path
      ../decompiler/datatests datatests` -> 668 applied / **187 passing**
      (baseline rust-port 37c2450 = 668 / **182**; +5 real, applied unchanged);
      inline.xml 0/12 -> **3/12** (#10/#11/#12), inlinetarget.xml 1/4 -> **3/4**
      (#1/#2/#3).  `cargo test --workspace` -> 3375+ passing, 0 failed (incl. the
      5 new verify_w10_inline_body.rs).  `cargo clippy -p kuna-decomp --lib` /
      `-p kuna-console --lib` / `-p kuna-harness --tests` clean.  C++ oracle
      `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 +
      675/675 PARITY OK, byte-untouched.

## Scope

Un-seams the inline-clone family (LOSS-037 (c)) against the in-scope C++:

- `Funcdata::inlineFlow` (funcdata_op.cc:853) -> flow.rs `inline_flow` (the nested
  `FlowInfo` over the callee via the placeholder-swap idiom, set-range/set-flags,
  forwardRecursion, generateOps, the EZ-marker / moveSequenceDead / opDestroyRaw
  dance, the hard-model RETURN->BRANCH + newCodeRef).
- `checkEZModel` (flow.cc:1159), `testHardInlineRestrictions` (:1135),
  `inlineClone` (:1076), `inlineEZClone` (:1110), `xrefInlinedBranch` (:1055),
  `forwardRecursion` (:1045), `inlineSubFunction` (:1244), `deleteCallSpec`
  (:1308) -> flow.rs / funcdata.rs counterparts.
- `checkForFlowModification` (flow.cc:654) folded into one method (inline +
  noreturn arms) replacing the prior noreturn-only seam.
- `queryCall` (flow.cc:674) inline/inject-id flow-effect copy via the
  `FlowEnvironment` seam (`query_call_inline`/`query_call_inject_id`/
  `build_inline_funcdata`), reusing `clone_op_from`/`clone_varnode_from` from the
  switch-chain wave.
- `Funcdata::warning`/`warningHeader` (funcdata.cc:119,135) buffered on the
  Funcdata (`pending_comments`) and re-seeded by decompile_drive, and
  `PrintC::emitCommentFuncHeader` (printc.cc:3429) rendering the
  `Comment::warningheader` subset before the prototype.

Rust-only diff (no C++/specs change); the C++ oracle is byte-untouched.

## Mandatory hunt list

- **Signedness:** clean. `res < 0`/`res == 0` are `int4` vs literal-0 (C++ `int4
  res`); `n <= 1` / `n - 1` in the CALL->BRANCH input-trim loop is `int4 n =
  num_input()` with `n > 1` guaranteed before `n-1` (no underflow); `injectid >=
  0` is `int4` vs 0. No int4-vs-uint compare introduced.
- **Integer widths:** clean. `SeqNum::new(addr, u32)` takes `get_time(): u32`
  (C++ uintm); `eaddr = Address::new(space, !0u64)` == C++ `Address(space,
  ~(uintb)0)`; flow flags are the verbatim `uint4` enum (4/8/0x10/0x2000, match
  flow.hh:62-72); comment_type warning=16 / warningheader=32 (comment.hh:58-59).
- **Wrapping:** clean. No `uintb`-lineage arithmetic added; the only ops are
  `Address::new(space, 0/!0)`, clone, and index `+1`/`-1` on bounded loop vars.
- **Comparator totality:** n/a — no comparator ported. `inline_recursion` is a
  `BTreeSet<Address>` (Address `Ord` is the prior-verified total order), NOT a
  HashSet.
- **Iteration-order provenance:** clean. `inline_clone`/`inline_ez_clone` iterate
  `src.obank().iter_dead()` (C++ `beginOpDead()..endOpDead()`, insertion-order
  intrusive list) via a collect-then-process snapshot — safe because the clone
  mutates `self.data`, never `src`; `unprocessed`/`addrlist` are vector appends in
  order; `visited` copy uses `.entry().or_insert_with()` matching `std::map::insert`
  no-overwrite (flow.cc:1099); the injectlist index-walk re-reads `.len()` each
  iteration (C++ `for(i=0;i<injectlist.size();...)` — nested inlines push more).
  No HashMap/HashSet in ported logic.
- **Off-by-one / do-while / reverse iterators:** clean. The EZ-marker idiom
  (`marker = dead_tail()` before clone, `dead_next(marker)` after) faithfully
  reproduces C++ `oiter = endDead(); --oiter; ...; ++oiter` (the marker is the
  callop, always present; the `None => dead_head()` fallback is unreached but
  correct). `testHardInlineRestrictions` `++iter == endDead()` -> `dead_next(op)
  == None` (the no-fallthrough fail).
- **Erase-while-iterating:** clean. `deleteCallSpec` (`qlst.remove(idx)`) shifts
  vector positions, but the fspec handle is the call op's identity (a
  process-unique counter in the annotation varnode, re-resolved by
  `get_call_specs_index(op)` each iteration), NOT the vector index — so the shift
  does not invalidate other calls' annotations, matching C++ `qlst.erase` of a
  raw pointer. `op_destroy_raw` does not free the slotmap slot (only `deadandgone`
  + list removal), so OpIds are never reused (no ABA).
- **Exception -> Result partial-state parity:** clean for the corpus. The
  placeholder-swap (`mem::replace(inlinefd, placeholder)`) is restored to
  `*inlinefd` on BOTH the hard-model bail (`return Ok(-1)`) and the normal exit;
  a `?` mid-`inline_flow` would leave `inlinefd` holding the placeholder, but the
  callee Funcdata is a local in `inline_sub_function` that drops on the propagated
  error (matching a C++ exception aborting the whole decompile). Not reached in
  the corpus (gate green).

## Faithfulness verification (vs C++ B5 oracle)

Drove the SAME inline.xml image through the C++ `decomp_dbg` (main tree) and the
Rust console:

- `add100` (EZ-model, inlines add50): both fold to the inlined `a + 100` and emit
  `/* WARNING: Inlined function: add50 */`; the original add50 CALL is destroyed.
  Rust adds a copy-prop temp (`int4 v1; v1 = a + 100; return v1;` vs C++ `return
  a + 100;`) — a downstream W8 cosmetic seam, NOT an inline-mechanism difference.
- `twohard` (hard-model, inlines compare): both weave the `compare` body into the
  `if (x < 10)` branch and emit the header; the second `compare` call stays
  un-inlined (recursion break).
- `collatz` (self-recursion, collatz<->collatz1): both inline collatz1 and break
  the recursive re-inline; the run terminates with a real body.

The inline machinery is structurally faithful. inline.xml #1-#8 fail only on
downstream copy-prop/symbol/structure (W8) and #9 on the un-rendered placement
warning (CommentSorter, not this item) — see LOSS below; the 3 *header*
assertions are real oracle matches.

## No special-casing

Grepped the diff: no function-name / address / magic-value hardcoding. Every
string literal is a verbatim C++ warning message ("Could not inline here",
"Inlined function: ", "No fallthrough prevents inlining here", "Return address
prevents inlining here", "Subroutine does not return", "WARNING: " /
"WARNING (jumptable): "). The header name is the resolved-callee symbol name from
the fspec (`fc.get_name()`), proven distinct (add50/compare/collatz1) by the
adversarial test. No `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`.
Two bare `as` casts (`jt_idx as int4`, `index as usize`) are narrowing on
known-non-negative small indices matching the C++ `int4` indexing (no `// cast:`
note — minor protocol nit, established codebase convention).

## findings
  - F1 (minor/divergence): `inline_recursion` is an owned `BTreeSet` deep-copied
    by `forward_recursion`, not C++'s shared pointer into `inline_base`; a
    cross-branch grandchild inline is not deduped.  NOT corpus-observable.
       cpp: decompiler/cpp/flow.cc:1048 (forwardRecursion copies the POINTER)
       rust: rust/crates/kuna-decomp/src/flow.rs (forward_recursion clones the set)
       -> LOSS-174
  - F2 (accepted partial): the `injectUserOp` (CALLOTHER) and `injectSubFunction`
    (inject-id) arms of `injectPcode` are W4-deferred `continue` no-ops; only the
    in-lining arm is wired.  Marked `// SEAM(W4)` in the diff.
       cpp: decompiler/cpp/flow.cc:1336-1347 (the two payload arms)
       rust: rust/crates/kuna-decomp/src/flow.rs inject_pcode (the two continues)
       -> LOSS-175
  - F3 (cosmetic, doc-only): the inject_pcode comment "A destroyed op id is
    filtered by liveness" is imprecise — a destroyed op stays gettable (slotmap
    slot not freed); the real guard is `get_call_specs_index` returning None after
    `delete_call_spec`.  Behavior is correct; only the comment misdescribes the
    mechanism.  No code change required.
       rust: rust/crates/kuna-decomp/src/flow.rs inject_pcode (the None=>continue note)
  - F4 (cosmetic, faithful-partial): placement `Comment::warning` lines ("Could
    not inline here", "Subroutine does not return") are buffered but not rendered
    (only `warningheader` is emitted; the body CommentSorter is a separate item),
    and `emit_comment_func_header` drops the `Comment::header` half of the default
    `head_comment_type` mask.  No `header`-type comment is produced by inline
    analysis, so the rendered subset is faithful for this feature.
       cpp: decompiler/cpp/printc.cc:3429-3468 / printlanguage.cc:586
       rust: rust/crates/kuna-decomp/src/printc.rs emit_comment_func_header

## adversarial tests
rust/crates/kuna-harness/tests/verify_w10_inline_body.rs (committed on the branch,
5 tests, all green):
  - w10_inline_body_header_warnings_are_real_oracle_parity (3 distinct resolved
    names; baseline 0/12 -> 3/12)
  - w10_inline_body_ez_clone_folds_leaf_arithmetic (console-driven: `a + 100`,
    header rendered, original CALL destroyed)
  - w10_inline_body_recursion_break_terminates_and_keeps_inner_call (collatz
    self-inline terminates; inner call survives)
  - w10_inline_body_placement_warning_is_documented_partial_not_misport (#9 fails
    for CommentSorter reasons, file runs, headers pass — proves F4)
  - w10_inline_body_inlinetarget_headers_real_and_no_target_corruption (PPC,
    BRANCH-as-CALL hard inline; #1/#2/#3 real, no "Could not find op at target")

## losses
LOSS-174 (inline_recursion owned-clone vs shared-pointer), LOSS-175 (injectUserOp/
injectSubFunction payload arms W4-deferred).  Both new, appended to
docs/rust-port/losses.md by this verifier.  No prior parity regressed (182 -> 187,
applied unchanged at 668; C++ oracle 675/675 untouched; all byte-parity / fence
tests intact).

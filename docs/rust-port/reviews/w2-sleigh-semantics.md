# w2-sleigh-semantics

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_6343a4ba-016-14 (worktree `.claude/worktrees/wf_6343a4ba-016-14`, reviewed at 4d11d50; adversarial tests committed as a73bae0)
cpp blob shas: all four in-scope files match the checklist entry
(`git hash-object` verified: semantics.cc 7b06da8e, semantics.hh 2a768a7b,
pcodecompile.cc a67a3de8, pcodecompile.hh 3caa57c6)

gate: `cd rust && cargo test -p kuna-sleigh` -> ok. 114 passed (lib, incl.
the 20 porter semantics/pcodecompile tests) + 6 (`verify_w2_sleigh_context`)
+ 6 (`verify_w2_sleigh_pattern`) + 6 (`verify_w2_sleigh_symbol`) + 6 (this
review's `verify_w2_sleigh_semantics`), 0 failed.
clippy: `cargo clippy -p kuna-sleigh -- -D warnings` (protocol command) ->
clean; `--all-targets` still fails on the PRE-EXISTING slghsymbol.rs:3964
test-only `drop_non_drop` lint (w2-sleigh-symbol review F3, not this item);
the new test target itself is clippy-clean
(`cargo clippy -p kuna-sleigh --test verify_w2_sleigh_semantics -- -D warnings`).
port-audit: `cargo run -p port-audit -- --item w2-sleigh-semantics` NOT
runnable — no `port-audit` crate exists in the workspace (carried from every
prior review); blob-SHA check performed manually instead (all four match).

## Method

Re-derived every function of semantics.cc (962 lines) and pcodecompile.cc
(781 lines) plus both headers against the item diff (commit 4d11d50:
`rust/crates/kuna-sleigh/src/semantics.rs` +2553,
`rust/crates/kuna-sleigh/src/pcodecompile.rs` +2077; the branch also
carries the already-reviewed context/pattern/symbol merges, excluded from
scope).  The 5 FORMAT_SCOPE attribute ids and 17 element ids newly defined
in `semantics::sla` were checked constant-by-constant against
`decompiler/cpp/slaformat.cc` (all match name and number; ATTRIB_VAL/SPACE/
CODE and ELEM_NULL come from the symbol-wave re-export, spot-verified:
val=2, space=4, code=7, null=11).

The load-bearing re-derivation of this review is the port's re-engineering
of the C++ pointer-aliasing in the size-propagation statics: C++
`force_size`/`matchSize`/`fillinZero` alias a `VarnodeTpl *` INTO the op
vector they simultaneously mutate (e.g. createLoad's `outvn` IS the
LOAD op's output; createStore's `val->outvn` IS input slot 2 of the STORE).
The port splits this into `force_size` (target outside the vector — the
`ExprTree::outvn` copies used by assignBitRange/createBitRange, which C++
also keeps as separate copies) and `force_size_at`/`VtSlot` (target inside
the vector, by `(op_index, slot)` — the createLoad/createStore cases).  I
verified for each C++ call site which side of that split it lands on, that
the propagation loop revisiting the target's own slot is the same-size
no-op it is in C++, and that match_size's clone-the-matched-size-first is
equivalent to C++'s live reference (the only aliasing case — source and
target sharing a local-temp offset — self-assigns an equal value in both).
Pinned by adversarial T2 (multi-pass propagateSize convergence through a
shared temp, exercising cross-op propagation then re-fill on pass two).

Seam checks: `SymbolWalker` (symbol wave) against the exact ParserWalker
surface semantics.cc touches (getFixedHandle by value vs C++ const ref:
no mutation through the handle in scope, equivalent); `FixedHandle` field
widths against context.hh (uint4 size/offset_size -> u32, uintb -> u64,
all three AddrSpace* -> Option<Rc>, match); the space-as-u64 convention
against kuna-num `pcoderaw.rs::get_space_from_const` (manager index, the
LOSS-015 representation — `space_to_const` produces exactly what
`get_space_from_const` consumes).  MACROBUILD/`setOpvec`/`setNumLabels`
consumers grep-verified to be slgh_compile.cc only (LOSS-001 scope);
pcodeparse.cc confirmed as the runtime caller of newOutput/assignBitRange/
createBitRange/addressOf/getVarnode (the next item's scope).

## Hunt list

- signedness: clean. The C++ sign-compare sites (`i<input.size()-1` in
  removeInput, `i<ops.size()`/`i<check.size()` loops, `zerovec.size() <
  lastsize` with int4 lastsize) are restructured to usize iteration with
  no behavior change (all counts small and non-negative; removeInput's
  shift-down is `Vec::remove`, same kept order).  int4-indexed accessors
  (`get_in(i32)`, handle indices) carry commented `as usize` casts; a
  negative index panics where C++ indexes out of bounds (UB), per ADR 0004.
- integer widths: clean with one cosmetic gap (F3).  Every implicit C++
  truncation is a commented cast at the same site: `delayslot`
  (uintb->uint4, semantics.rs:1303), fillinBuild/changeHandleIndex BUILD
  index (uintb->int4, rs:1333/1373), decode attributes (intb->uint4/int4,
  rs:1450-1456, 611-614), `transfer`'s `(int4)getReal()` (rs:759),
  newOutput's VarnodeSymbol size (uintb->int4, pcodecompile.rs:740),
  symsize/insize (uintb->uint4, rs:1158/1265), HandleTpl::fix sizes
  (uintb->uint4, rs:1019/1023/1043).  adjustTruncation's int4->uintb
  SIGN-extension is explicit `i64::from(x) as u64` (rs:859-866), pinned
  by adversarial T4 (byteoffset == -1 encodes to all-ones / ...0007).
- wrapping: clean. All uintb-lineage arithmetic uses `Wrap` helpers:
  offset+truncation adds (semantics.rs:389-391, 511), buildTruncatedVarnode
  wadd/wsub (pcodecompile.rs:1085, 1120, 1124), both bitrange mask
  constructions wshl/wsub (rs:1153, 1279), labelcount wadd.  The C++
  shift-count UB (`val >>= 8*(value_real>>16)` for counts >= 64; mask
  shifts when numbits==0) resolves to wrapping_shl/shr's count-mod-64 —
  the x86 hardware behavior of the C++ binary — pinned by adversarial T1
  at the 56/64-bit boundary.  One bare i32 `+` in adjust_truncation's
  bounds check is the C++ signed-overflow-UB region (F4, note).
- comparator totality: clean. `ConstTpl::less_than`/`VarnodeTpl::less_than`
  are strict-weak transcriptions of the C++ operators; the spaceid arm
  replaces nondeterministic heap-pointer order with manager-index order
  (None first) — deterministic and total, the LOSS-015 representation (F2).
  PartialEq matches C++ operator== exactly, including Rc::ptr_eq for the
  spaceid arm and unnamed_flag exclusion in VarnodeTpl.  No floats, and no
  BTree in the item is keyed on these orders.
- iteration-order provenance, per loop: clean. Every loop is over a C++
  std::vector equivalent in index/insertion order: PcodeBuilder::build's
  opvec walk, fillinBuild's two passes (scan in vec order, then prepend
  per check index — each missing BUILD inserted at front, so the LAST
  missing index ends up FIRST, porter test pins [2,0,1]), buildOnly,
  changeHandleIndex, deleteOps' stable compaction, encode loops,
  appendParams' param order, createOp*'s ops splicing (vn2 after vn1),
  propagateSize's zerovec index vectors (same op order across passes).
  No HashMap/HashSet anywhere in the item (mechanical grep clean).
- off-by-one / do-while / reverse iterators: clean. No do-while/reverse
  iterators in scope.  propagateSize's `lastsize = size()+1` priming, the
  shrink-only loop condition, and the `lastsize != 0` exit are transcribed
  exactly (empty-template edge returns true in both).  assignBitRange's
  symsize checks run even at symsize==0 while createBitRange gates ALL its
  insize checks under `insize > 0` — the port preserves this asymmetry
  exactly (C++ pcodecompile.cc:624-635 vs 712-724), pinned by adversarial
  T3 (zero-size varnode: assign errors "Assigned bitrange is bad", create
  silently emits SUBPIECE).
- erase-while-iterating: n/a beyond deleteOps, whose null-then-compact
  becomes flag-then-stable-compact with identical kept order (porter test
  pins [0,2,4] from deleting {1,3}).
- exception -> Result partial-state parity: clean, pinned twice.
  PcodeBuilder::build restores labelbase only on fall-through; an error
  from dump/appendBuild leaves labelbase at the new base and labelcount
  grown, exactly the C++ no-RAII throw state (adversarial T5).
  fillinBuild's duplicate-BUILD return leaves the check marks made so far
  (adversarial T6).  force_size's mid-propagation mismatch error leaves
  earlier varnodes resized (C++ throws mid-loop identically); newOutput
  propagates setOutput's error BEFORE addSymbol (C++ throws before
  addSymbol); add_op_list stops adding at the first refused delay slot
  (C++ returns false mid-list).  ConstTpl::decode error taxonomy matches
  C++ (DecoderError for the selector, LowlevelError for the element type).

## Findings

- F1 (loss, accepted): compiler-side surface unported, and the LOSS-022(b)
  getVarnode sliver is NOT closed at this item contrary to that entry's
  restoration plan: `ConstructTpl::setOpvec`/`setNumLabels` (friend
  SleighCompile; slgh_compile.cc:3414,3422 are the only consumers) and
  MACROBUILD's consumers (slgh_compile.cc:1359,3245,3406) stay C++
  (LOSS-001); `createBitRange(SpecificSymbol *,...)` is reshaped to
  `create_bit_range(vn, sym_name, ...)` taking the caller-resolved
  varnode, moving the whole getVarnode obligation to w2-sleigh-pcodeparse
  (which needs it for four other grammar productions anyway,
  pcodeparse.cc:2365-2453).  Counterweight: LabelSymbol — LOSS-022(a)'s
  pcodeparse sliver — is ported HERE (pcodecompile.rs:122-173, Cell-based
  isplaced/refcount mirroring the C++ shared-pointer mutation).
     cpp: decompiler/cpp/semantics.hh:165-172; decompiler/cpp/
          pcodecompile.cc:676 (createBitRange signature)
     rust: rust/crates/kuna-sleigh/src/pcodecompile.rs:26-42 (module
           docs), :1224-1230; rust/crates/kuna-sleigh/src/semantics.rs:41-45
     -> LOSS-001, LOSS-022, LOSS-026 (appended by this review)
- F2 (loss, accepted): the C++ `(uintb)(uintp)AddrSpace*` encodes become
  the space's manager index (`space_to_const`, null -> 0) in
  `ConstTpl::fix` (j_curspace/spaceid/handle-v_space arms), and
  `ConstTpl::less_than`'s spaceid arm orders by manager index (None first)
  instead of heap-pointer order.  Consistent with kuna-num
  `get_space_from_const`; deterministic where C++ was not.
     cpp: decompiler/cpp/semantics.cc:139,146-147,176 (fix), :109
          (operator< spaceid)
     rust: rust/crates/kuna-sleigh/src/semantics.rs:154-164, :329-335
     -> LOSS-015
- F3 (minor, non-blocking): one bare `as` cast in the item without a
  `// cast:` justification: `self.select as i64` in ConstTpl::encode
  (enum-discriminant widening, no truncation possible; C++ passes the
  enum through integer promotion).  Protocol-style gap only.
     cpp: decompiler/cpp/semantics.cc:312
     rust: rust/crates/kuna-sleigh/src/semantics.rs:547
- F4 (note): `adjust_truncation` uses bare i32 `+` in
  `numbytes + byteoffset > sz` and the big-endian arm — a debug-build
  panic where C++ signed overflow is UB.  Reachable only via hostile
  .sla offset/size reals >= 2^31; ADR-0004-sanctioned UB resolution.
  The defined-behavior negative-byteoffset region is parity-exact
  (adversarial T4).
     cpp: decompiler/cpp/semantics.cc:504-515
     rust: rust/crates/kuna-sleigh/src/semantics.rs:851-866
- F5 (note): `OpTpl::new_for_decode` seeds `opc` with CPUI_COPY where the
  C++ default constructor leaves it uninitialized until `decode`
  overwrites it.  Observable only if a caller reads the opcode of a shell
  whose decode failed (C++ reads indeterminate memory there).  Documented
  at the site.
     cpp: decompiler/cpp/semantics.hh:145
     rust: rust/crates/kuna-sleigh/src/semantics.rs:1111-1115
- F6 (note): `getLocation(SleighSymbol *)` becomes name-keyed
  `get_location(&[u8])`.  The only runtime implementor (PcodeSnippet,
  next item) returns null unconditionally in C++; the pointer-keyed map
  belongs to the unported compiler, so a same-name collision divergence
  is unreachable in ported scope.  Documented in the module docs.
     cpp: decompiler/cpp/pcodecompile.hh:69
     rust: rust/crates/kuna-sleigh/src/pcodecompile.rs:34-37, :652

## Mechanical pass detail

- grep over the item diff (d7b824c..4d11d50, rust/ only): no `todo!`,
  `unimplemented!`, `HashMap`, `HashSet`, or `sort_unstable` anywhere in
  semantics.rs/pcodecompile.rs (shipped or test code).  Every bare `as`
  cast carries a `// cast:` justification at the line or the immediately
  preceding line, except the benign enum cast in F3.
- `cargo clippy -p kuna-sleigh -- -D warnings` clean (protocol command);
  the `--all-targets` failure is the pre-existing w2-sleigh-symbol F3
  test lint, out of this item's scope.

## Adversarial tests

`rust/crates/kuna-sleigh/tests/verify_w2_sleigh_semantics.rs` (committed
on the port branch as a73bae0; all pass, and the full gate stays green):

- verify_w2sem_offset_plus_const_shift_boundary — v_offset_plus on a
  constant handle at shift 56 (defined C++) and shift 64 (C++ UB, pinned
  to the x86/count-mod-64 resolution `wshr` provides); low-16 truncation
  add on the non-constant path.
- verify_w2sem_propagate_size_multipass_convergence — a temp chain
  unresolvable on pass one that converges on pass two through cross-op
  force_size propagation (the index-based re-engineering), plus a
  never-resolvable chain that terminates returning false.
- verify_w2sem_bitrange_zero_size_guard_asymmetry — zero-size real
  varnode: assignBitRange reports "Assigned bitrange is bad" and passes
  the rhs ops through; createBitRange emits SUBPIECE(vn,1):1 with no
  error (the insize>0 guard asymmetry).
- verify_w2sem_adjust_truncation_sign_extension — byteoffset == -1
  sign-extends through the uintb encode on both endians
  (all-ones / 0xFFFF_FFFF_FFFF_0007); out-of-bounds leaves the offset
  untouched.
- verify_w2sem_pcodebuilder_error_partial_state — a failing dump leaves
  labelbase at the new base (not restored) and labelcount grown, for two
  distinct base/count starting states.
- verify_w2sem_fillin_build_partial_marks_on_duplicate — duplicate BUILD
  returns 1 with the prior check marks retained and no ops prepended.

losses: LOSS-001, LOSS-015, LOSS-022, LOSS-026

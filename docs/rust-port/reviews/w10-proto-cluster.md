# w10-proto-cluster
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8, round 1)
date: 2026-06-16
gate: `cd rust && cargo test --workspace` -> green (0 failed); `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean

## SCOPE MISMATCH (the headline finding)

The task and the item id `w10-proto-cluster` declare the **proto cluster** as the C++
in scope: `fspec.cc` (FuncProto hidden-return-ptr, float10/joined-float-reg param
model, struct-return reconstruction, CONCAT-piece return) and `coreaction.cc`
(`ActionOutputPrototype`/`ActionInputPrototype` return+param recovery).

**The branch delivers none of that.** `git diff rust-port...rport/w10-proto-cluster --
rust/` touches exactly ONE file — `rust/crates/kuna-decomp/src/heritage.rs` (+302/-15) —
and it ports `Heritage::processJoins` and its helpers (`splitJoinLevel`,
`splitJoinRead`, `splitJoinWrite`, `floatExtensionRead`, `floatExtensionWrite`) from
`heritage.cc:2068-2313`. This is W4 join-space splitting, a different subsystem
entirely. The branch's single commit message says so verbatim: "port
Heritage::processJoins join-space splitting (W4 seam)".

The proto cluster (`fspec.rs`, `coreaction_protos.rs`, `FuncProto`,
`ActionOutputPrototype`/`ActionInputPrototype`) ALREADY EXISTS on `rust-port` and is
**untouched** by this branch. So the proto-cluster parity wins this item is nominally
about were not delivered here; they pre-exist.

This is a structural/naming mismatch, recorded as **LOSS-204**. It is NOT a REJECT
condition: nothing is special-cased, unfaithful, weakened, fabricated, or regressing.
The actual content present (the heritage port) is verified faithful and zero-regression
below, so the correct verdict is ACCEPT-WITH-LOSSES (a faithful PARTIAL with zero
regression), with the undelivered proto-cluster scope booked as the loss.

## What was actually verified (the heritage join-space port)

The diff is a statement-faithful transcription of `heritage.cc:2068-2313`, checked
function-by-function against the C++ at the in-tree blob:

- `split_join_level` (heritage.cc:2068): the `cursize == piece.size` fast path
  (push vn + `None` filler, `recnum += 1`), the else-branch size accumulation
  (`for j in recnum..numpieces { sizeaccum += piece(j).size; if ==cursize {j+=1;break} }`
  reproduces the C++ `for` + `j+=1;break` exactly — final `j == j0+1`), the
  `numinhalf = (j-recnum)/2` (truncating int4 division, positive operands, identical),
  the `numinhalf==1` -> `newVarnode` (register piece) vs `>1` -> `newUnique`, and the
  `(j-recnum)==2` -> register least-half vs `!=2` -> `newUnique` selection.
- `split_join_read` (heritage.cc:2119): the `while lastcombo.len() < numPieces` loop,
  the `leasthalf==None -> continue` skip, PIECE concat construction with
  `op=concat` earliest-op aliasing, the `isPrimitive` precisHi/Lo vs `opMarkNoCollapse`
  fork, and the `lastcombo = nextlev.flatten()` (drop `None`) rebuild.
- `split_join_write` (heritage.cc:2172): two SUBPIECE ops per split (mosthalf =
  curvn>>leastsize, leasthalf = curvn(0)), the `isInput ? bb->getStart() : op->getAddr()`
  address pick (hoisted once — `isInput` is loop-invariant), the `op==0 -> opInsertBegin`
  else `opInsertAfter` placement, `op=split` latest-op aliasing.
- `float_extension_read`/`float_extension_write` (heritage.cc:2236/2256): single-piece
  FLOAT2FLOAT truncation/extension; `Address::new(spc, vdata.offset)` == C++
  `vdata.getAddr()` (= `Address(space,offset)`, pcoderaw.hh:96).
- `process_joins` (heritage.cc:2282): replaces the prior `unimplemented_seam` stub with
  the real walk; `find_join(offset).expect(..)` matches the C++ `findJoin` throw site
  (partial-state parity preserved); the `unified.size != vn.getSize()` panic matches the
  C++ `LowlevelError` throw; the `pass != info->delay -> continue` skip is faithful; the
  call site (heritage.rs:2764) and action ordering are UNCHANGED.

hunt list:
- Signedness: clean — `size:u32 as int4` casts faithfully resolve the C++ `int4 ==
  uint4` size compares (`-Wno-sign-compare`); all values small positive.
- Integer widths: clean — `num_pieces()`/`get_size()`/`get_unified().size` map to
  i32/i32/u32 matching C++ `int4`/`int4`/`uint4`; `unified.size as int4` for the panic
  compare matches the C++ widths.
- Wrapping: clean — `sizeaccum += size` operates on small piece sizes; no `uintb`
  arithmetic in the diff (the `wadd` join-allocate arithmetic lives in already-verified
  kuna-base, not this diff).
- Comparator totality: n/a — no comparator ported in this diff.
- Iteration-order provenance: clean — `process_joins` snapshots `loc_space_ids(joinspace)`
  (VarnodeLocSet loc-order, same `getSpace()!=joinspace` break as C++ `beginLoc..endLoc`);
  the snapshot is faithful because the split helpers insert ONLY into piece spaces
  (register/unique) via newVarnode/newUnique/newConstant/newVarnodeOut, never the join
  space, so C++ "new varnodes inserted before enditer" can never add a join-space
  Varnode the snapshot would miss. `lastcombo`/`nextlev` are Vec (index order = C++).
  No HashMap/HashSet anywhere in the diff.
- Off-by-one / do-while / reverse iterators: clean — the inner accumulation `for` and
  the `while lastcombo.len() < numPieces` outer loop reproduce the C++ bounds and the
  `j+=1;break` exactly (verified by adversarial tests A/B below).
- Erase-while-iterating: clean — no erase; the snapshot + flatten-rebuild model is the
  faithful translation (the helpers append, never remove, from the live loc-set).
- Exception -> Result partial-state parity: clean — `expect`/`panic!` land at the same
  points the C++ `throw LowlevelError`s (find_join miss, size mismatch), leaving the
  same partial state.

findings:
  - F1 (cosmetic / protocol): bare `as int4` / `as i32` / `as u64` casts in the diff
    (heritage.rs split_join_level/read/write/process_joins) lack the protocol-required
    `// cast:` justification comment. Each is correct (faithful width/sign resolution of
    the C++ `uint4`/`int4` size types), so this is a style/protocol nit, not a
    correctness defect.
       cpp: decompiler/cpp/heritage.cc:2074 (`curvn->getSize() == joinrec->getPiece(recnum).size`, int4==uint4)
       rust: rust/crates/kuna-decomp/src/heritage.rs (`joinrec.get_piece(recnum).size as int4`)
  - F2 (scope, MAJOR — booked as LOSS-204): the branch does not deliver the declared
    proto-cluster item; it ports `Heritage::processJoins` instead. The proto cluster is
    untouched and pre-exists on rust-port. No fspec.cc / coreaction.cc Rust changed.
       cpp: decompiler/cpp/fspec.cc (FuncProto) + decompiler/cpp/coreaction.cc
            (ActionOutputPrototype/ActionInputPrototype) — NOT ported by this branch
       rust: rust/crates/kuna-decomp/src/heritage.rs (the actual, unrelated change)

## Mechanical + parity evidence

- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean.
- `cargo test --workspace`: green (0 failed). B5 boolless fixture: 11 passed / 1 ignored.
- Special-casing grep over added lines: NO function-name / address / magic-value /
  type-name hardcoding (only `expect(..)` panic messages and doc comments).
- Forbidden constructs grep: NO `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/`partial_cmp` in the diff.
- Datatest passing SET (rust engine, real `decomp_test_dbg`, repo specs+datatests):
  parent (de9be68) = 675 applied / 312 passing; branch (e926daf) = 675 applied / 312
  passing. `comm`/`diff -q` of sorted `Success --` sets: **BYTE-IDENTICAL** — zero
  regressions AND zero new passes (the change is observably inert on the corpus, which
  is correct: no datatest exercises a join-space Varnode on the built spec set).
- boolless / readstruct / condconst_conn: covered by the byte-identical passing set and
  the green workspace suite (these are B-fixture/integration tests, all passing).
- C++ oracle 207/207 + 675/675: structurally untouched — the branch changes ONLY
  `rust/` (no `decompiler/cpp`, `specs/`, or `Makefile` touched).
- B0 / universalAction listing: unchanged (only `process_joins`'s body was filled in;
  the call site and all action ordering are identical).

adversarial tests (committed on the branch, marked w10_proto_cluster):
  - w10_proto_cluster_split_join_level_two_register_halves — 16-byte vn vs a 2x8
    register JoinRecord: asserts the else-branch emits BOTH halves as register pieces
    (newVarnode, not newUnique) at the recorded offsets, with a None-free 2-entry nextlev
    (the numinhalf==1 / (j-recnum)==2 branch).
  - w10_proto_cluster_split_join_level_equal_piece_fast_path — 8-byte vn vs an 8+8+8
    record: asserts the `cursize==piece.size` fast path keeps the vn as the most-half and
    fills the least-half slot with None (the 2-1 mapping alignment).
  - w10_proto_cluster_process_joins_empty_join_space_noop — a function with a join space
    but no join Varnodes: asserts process_joins adds zero Varnodes / zero ops (the
    snapshot/loop-boundary no-op path).

losses: LOSS-204 (the proto-cluster scope this item declared is not delivered by this
branch; the faithful heritage join-space port that IS delivered is accepted with zero
regression). F1 (missing `// cast:` comments) is a cosmetic nit folded into LOSS-204's
restoration note, not a separate accepted divergence.

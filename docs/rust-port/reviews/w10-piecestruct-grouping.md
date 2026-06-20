# PieceNode::findRoot / linkProtoPartial root-symbol bind (w10-piecestruct-grouping) — ACCEPT, +10

Wave (2026-06-18) integrated at rust-port (merge of `rport/w11-piecestruct-grouping` @ `73259c9`,
base `94e2030`/589). Verdict: **ACCEPT**. The payoff of the propagateSpacebaseRef enabler — the
now-firing `RulePieceStructure` split finally binds field names.

## The fix

Once the propagateSpacebaseRef seed (prior commit) made `RulePieceStructure` fire+split live, the two
CONCAT-tree roots were UNIQUE-space PIECE outputs whose HighVariable was not grouped with the addr-tied
stack member, so the root got a fresh `vN` instead of `mypiece.a`. This wave closes the grouping:
- **`bind_proto_partial_piece` (coreaction_cleanup.rs:1593-1682)** — faithful
  `linkProtoPartial`/`linkSymbol(nameRep)` root naming (merge.cc:1374 `groupPartialRoot`,
  coreaction.cc:3136). When the CONCAT root's name-rep is addr-tied storage covered by a mapped
  composite Symbol (the unified `mypiece/8` stack symbol), bind the Symbol's shared `vN` name +
  in-symbol offset + whole struct type; otherwise keep the prior whole-value/register-return path. The
  addr-tied gate reproduces the C++ usepoint discrimination.
- **`link_symbol_root` (varmap.rs:1521-1563)** — the on-demand `linkSymbol` for a root (container query
  + one-shot undefined→`vN` rename).
- **`printc.rs:1567-1576`** — new `PartialEntry` enum (Member/Subscript); **printc.rs:~4540** — ported
  the TYPE_ARRAY arm of `pushPartialSymbol` via `TypeArray::getSubEntry`; **printc.rs:~4610** — reversed
  the member-op push order to match C++ so `v1.arr[i]` nests correctly.

## Effect

589 → **599** (+10). Gained: Piece Structure #1-6 (`v1.a = a`, `v1.b = b`, `v1.arr[0..3] = c..f`) +
Partial splitting #1/#12/#13/#14 (bonus — same root-symbol binding).

## Gate (at integration onto 589)

- datatests `[675, 599]`; regressed-set **EMPTY** (Concat 18/18, packstructaccess 4/4, switch,
  for-loop, Bitfields, Stack-string all unmoved).
- `cargo test --workspace --no-fail-fast` (debug): **0 failures**.
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ + docs byte-untouched.

## Residual

**Return Structure** (5) not gained — the grouping now renders the retstruct body
(`v2.a = v1; v2.b = y; return v2`), confirming the grouping is decisive, but the 5 assertions remain
blocked on the usepoint-scoped `type varnode %EAX tmp` register-Symbol binding — a
`queryProperties(addr,1,usepoint)` surface distinct from the piece-grouping. Next-locus: thread the
real usepoint into `varmap::query_container_for_link` (coreaction_cleanup.rs:1632) / `linkSymbol`
(corpus-wide naming change — own wave).

## No special-casing

Faithful merge.cc:1374 / op.cc:822 / coreaction.cc:3136 transcription; render keyed by addr-tied
storage + composite-Symbol geometry only, no name/offset/type hardcoding.

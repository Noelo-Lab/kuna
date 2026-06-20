# propagateSpacebaseRef mapped-stack seed (w10-propagate-spacebaseref) — ACCEPT (enabler, +0)

Wave (2026-06-18) integrated at rust-port (merge of `worktree-agent-a583cd01365b5c46c` @ `5e663dc`,
base `34de25e`). Verdict: **ACCEPT as a foundational enabler** — +0 datatest movement, regression-free,
on the critical path to the Piece Structure / Return Structure clusters.

## The change

`propagate_ref` (`coreaction_infertypes.rs:1433-1444`) — replaced the skip-gate `is_mapped()` with
`kuna_symbol_entry().is_some()` (the faithful `Varnode::mapentry`/`getSymbolEntry()` proxy,
coreaction.cc:5490). `isMapped()` is a distinct heritage-time bit set on *every* stack varnode, which
over-skipped the entire stack frame and killed the spacebase type seed.

## Effect (internal — verified live, no rendered-output change yet)

The whole 8-byte `mypiece` now flows from the `&v1` PTRSUB output into the stack slice; the
MapState/ScopeLocal restructure (`add_fixed_type` PARTIALSTRUCT/ARRAY → Open `mypiece`) now builds a
**unified `mypiece/8` stack Symbol at -0x18** (base had two fragmented 4-byte `undefined4` locals);
`RulePieceStructure` now **fires and splits** the struct store (base emitted raw `v1 = CONCAT22(b,a)`;
this branch emits separate `v2 = a;` / `v1[i] = …`). No datatest flips yet — the `v1.a` field *name*
render is gated on the downstream merge grouping (see Residual).

## Gate (at integration onto 589)

- datatests `[675, 589]`; GAINED empty, REGRESSED **empty** — no output change, no regression.
- `cargo test --workspace --no-fail-fast` (debug): **0 failures**.
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ + docs byte-untouched.

## Residual (the next-locus this enables)

The two CONCAT-tree roots are UNIQUE-space PIECE outputs (`u…:4`); their HighVariable is not
grouped/merged with the addr-tied stack member `s-0x18:4`. So `bind_proto_partial_piece`
(`coreaction_cleanup.rs:1538`, via `piece_find_root`→root-high) names the root with a fresh `vN`
instead of querying the stack member's container (mypiece). C++ closes this through
`PieceNode::findRoot` + `linkProtoPartial` group-establishment (merge.cc:1374 `groupPartialRoot`;
coreaction.cc:3136) so `rootHigh->getNameRepresentative()` is the addr-tied member and `linkSymbol`
binds mypiece. That merge-stage grouping is the next wave; it builds directly on this seed.

## No special-casing

Single faithful gate change matching coreaction.cc:5490; no name/offset/type hardcoding.

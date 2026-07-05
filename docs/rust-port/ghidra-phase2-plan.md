# Ghidra mode — Phase 2 plan: the engine bridge

**Status: [PROPOSAL] — awaiting go/no-go.** Phase 2 is the first phase that
touches `kuna-decomp`'s core types and can perturb the standalone engine's
output, so per the repo's standing requirement (large/output-changing features
go through a proposal for human sign-off) it is scoped here before the risky
work lands.

Phase 1 (PR #134, branch `feat/ghidra-mode`) shipped the wire protocol complete
and the engine stubbed: `decompileAt` answers the incomplete-function shape so
the Ghidra GUI degrades cleanly. **Phase 2 makes `decompileAt` return real,
marked-up C** — the first end-to-end decompilation of a live Ghidra program by
kuna, rendered in the stock GUI.

Citations: Ghidra 12.2-DEV (`f9e13846`); C++ under
`Ghidra/Features/Decompiler/src/decompile/cpp/`, Java under
`.../src/main/java/ghidra/app/decompiler/`. kuna paths are relative to
`decompiler/crates/`. The seam inventory this plan implements is
`docs/ghidra-integration.md` §9; the response contracts it must satisfy are
§6 there.

---

## 0. What "done" means for Phase 2

Open a small stripped x86-64 ELF in the stock Ghidra GUI with the kuna core
active (PR #134's extension), double-click a function, and see kuna's C in the
Decompiler window — with working click-to-address and variable highlighting.
Concretely, `decompileAt` must return a packed `<doc>` carrying, in order, the
`HighFunction` `<function>` (prototype + `<localdb>` + `<ast>`) and the Clang
markup `<function>`, with the two cross-referenced consistently
(`DecompileResults.java:215-264`, `HighFunction.java:245-293`). Rename/retype,
the switch analyzer's `<jumptablelist>`, `<parammeasures>`, and `structureGraph`
are explicitly **out of scope** — Phase 4.

## 1. The central design decision: how a provider calls back into Java

This is the crux of the whole phase and the one thing worth settling first.

Upstream, every ghidra-mode component (`LoadImageGhidra`, `GhidraTranslate`,
`ScopeGhidra`, …) holds a raw back-pointer `glb` to its `ArchitectureGhidra`,
and issues queries through it *mid-decompile*: the engine, deep inside
`decompileAt`'s action loop, calls e.g. `loadimage->loadFill`, which writes a
`getBytes` query onto the same `sout` the command response is being built on,
then blocks reading `sin` for the answer. The protocol is strictly
synchronous — exactly one query is ever in flight, and it completes before the
engine regains control (`ghidra_arch.cc:63-76`).

**Proposed kuna shape (demonstrated in Phase 1's `kuna-ghidra/src/provider.rs`):**
a `SharedClient<R,W> = Rc<RefCell<GhidraClient<R,W>>>` cloned into every
provider and into the process loop. `GhidraLoadImage::load_fill` already does
`self.client.borrow_mut().get_bytes(...)` and is unit-tested end to end. The
`RefCell` never double-borrows because the protocol is synchronous; the `Rc`
mirrors the C++ shared `glb` pointer. This keeps everything single-threaded and
allocation-cheap, and — critically — is entirely additive: it lives in
`kuna-ghidra` and touches no existing crate.

Open sub-question for review: the process loop currently (Phase 1) owns
`sin`/`sout` directly in `GhidraProcess` and writes command responses inline.
Phase 2 must move stream ownership into the `SharedClient` so the loop and the
providers share it. Two options:

- **(a) `GhidraProcess` holds the `SharedClient`** and writes responses through
  it (borrow for the duration of a response-frame write, drop before handing
  control to the engine, which re-borrows per query). Simple; one owner.
- **(b) split reader/writer** so response-writing and query-writing hold
  independent handles to the same `W`. More plumbing, no clear benefit given the
  protocol is half-duplex within a command.

**Recommendation: (a).** It matches the C++ single-owner model and the Phase-1
`SharedClient` type already expresses it.

A second concern the scaffolding flags with a `TODO(phase-2)`: a pipe death
*during* a query. Upstream `exit(1)`s from inside `readToAnyBurst`
(`ghidra_arch.cc:95-96`); kuna's trait methods can only return a `KunaError`, so
the fatal signal must ride out through the engine and be re-checked by the loop.
Proposed: a `Cell<bool> fatal` on the `SharedClient`; `wire_to_kuna` sets it on
`PipeClosed`/`Io`, the query returns a `KunaError`, and `run_command` checks it
after the action returns and propagates `WireError::PipeClosed` so `main` exits
1. This preserves the "one bad pipe ends the process" contract without unwinding
across the FFI-like boundary.

## 2. Work breakdown

Ordered by dependency. Each item notes the kuna type it touches and the risk to
the standalone engine (the thing the three gates protect).

### 2.1 The `Architecture` construction path (`registerProgram` → engine)

Today Phase 1's `Session` holds the four spec strings and a parsed
`GhidraTranslate` (space manager only). Phase 2 turns a `Session` into a live
`Architecture` (`infra/architecture.rs:218`).

- **Reuse the existing frontend recipe** (`docs/ghidra-integration.md` §3 of the
  kuna-console path): the console's `build_engine_and_init` already sets cspec/
  pspec XML (`set_cspec_xml` :2665, `set_pspec_xml` :2675) and runs
  `init_post_engine` (:3584). The wire cspec/pspec are the *same XML* those
  setters expect — so most of registerProgram is feeding the four wire documents
  into the path that already exists. `build_engine_and_init` is file-private
  (`kuna-console/src/engine.rs:542`); **promote it to `pub` or replicate the
  ~70-line tail in `kuna-ghidra`** (replication avoids a `kuna-console`
  dependency and keeps the crate lean — recommended).
- **coretypes**: decode the wire `<coretypes>` into the `TypeFactoryImpl`
  (`buildCoreTypes`, `ghidra_arch.cc:316-349`); the decode machinery is ported.
- **Risk: LOW-MEDIUM.** This constructs a *new* `Architecture` on the ghidra
  path; it does not change how the standalone path constructs one. The shared
  code (`init_post_engine`) is exercised identically by both, so a regression
  would show in `make test`. The one real risk is promoting `build_engine_and_init`
  to `pub` — mitigated by replicating instead.

### 2.2 The `Translate` enum seam (the concrete-`Sleigh` field)

`Architecture.translate` is a concrete `Sleigh` (`architecture.rs:836`), chosen
so `manage()` (the space-manager accessor) is concrete. A `GhidraTranslate`
(p-code per instruction from `getPcode`) cannot be installed without generalizing
that field.

- **Proposed: `enum EngineTranslate { Sleigh(Sleigh), Ghidra(GhidraTranslate) }`**
  with `manage()`/`base()` etc. delegating. NOT `Box<dyn Translate>` — an enum
  keeps `manage()` returning a concrete `&AddrSpaceManager` (the Sleigh-only call
  surface — `manager_rc`, `get_register_varnode`, `get_unique_start`,
  `with_context_db_mut` — audited and delegated per variant).
- The lift consumer is already trait-typed
  (`FlowEnvironment::translate() -> &dyn Translate`, `s2_lift/flow.rs:1321`), so
  the pipeline below `Architecture` is untouched — only the owner changes.
- `GhidraTranslate` itself: port `ghidra_translate.cc:45-156` — `getRegister`/
  `getRegisterName` caches (`nm2addr`/`addr2nm`), `getUserOpNames` (the probe
  loop at init), and `oneInstruction` (one `getPcode` query per instruction, the
  `<inst>`/`<op>` decode, `ELEM_UNIMPL` → `UnimplError`, empty → `BadDataError`).
  The space manager it needs already exists (Phase 1's `GhidraTranslate::decode`).
- **Risk: MEDIUM-HIGH — this is the highest-risk item.** Making
  `Architecture.translate` an enum touches ~30 call sites in `architecture.rs`
  and anything reaching `translate()`. Every one must delegate to the `Sleigh`
  variant with byte-identical behavior on the standalone path. **Mitigation: land
  it as a mechanical, behavior-preserving refactor in its own commit, gated by
  all three test suites BEFORE any `Ghidra` variant logic is added** — the enum
  with only the `Sleigh` variant wired must be a no-op for the 675 datatests.

### 2.3 `GhidraLoadImage` + `ContextGhidra` (the two "trivial" providers)

- **`GhidraLoadImage`**: **done in Phase 1's `provider.rs`** (implements the real
  `kuna_sleigh::loadimage::LoadImage`, backed by `getBytes`, tested). Phase 2 only
  hands it to the engine as the loader (`set_loader`, `architecture.rs:2336`).
- **`ContextGhidra`**: implements `kuna_sleigh::globalcontext::ContextDatabase`,
  overriding only `getTrackedSet` via a `getTrackedRegisters` query
  (`ghidra_context.cc:20-31`; everything else throws — it is Java's job).
  **Friction to resolve in review:** the trait signature is
  `get_tracked_set(&self, addr) -> &TrackedSet` (`globalcontext.rs:453`) —
  returning a borrow from `&self` while querying on demand needs interior
  mutability that can hand out a `&TrackedSet`. Options: (i) a `RefCell` cache +
  an `unsafe`-free `Ref::leak`-style pattern (rejected — fragile); (ii) query
  eagerly at function-start into an owned field the trait then borrows (the C++
  `cache` member is cleared each call, so a single-entry owned cache matches
  semantics); (iii) widen the trait to `&mut self` (touches the standalone impl).
  **Recommendation: (ii)** — a single-slot owned cache populated by an explicit
  `prime(addr)` the loop calls before the action, mirroring C++'s clear-then-fill.
- **Risk: LOW** for `GhidraLoadImage` (additive, done); **LOW-MEDIUM** for
  `ContextGhidra` (the trait-borrow friction is real but self-contained; option
  (ii) touches no standalone code).

### 2.4 `Funcdata::encode` — the `<function>` response document

kuna has **no** `Funcdata::encode` (no encode on `substrate/funcdata*.rs`). This
is the biggest *new* code in the phase: port `funcdata.cc Funcdata::encode` — the
`<function>` element with `<addr>`, `<prototype>`, `<localdb>` (the local symbol
map), and `<ast>` (the SSA varnodes + p-code ops). Minimal first cut: name/entry
+ `<ast>` + prototype; `<highlist>`/`<jumptablelist>` are Phase 4.

- The Java decoder is the normative contract (`HighFunction.java:245-293`,
  `PcodeSyntaxTree.decode`, `LocalSymbolMap.decodeScope`); the C++ encoder at
  `GHIDRA_REV` is the porting source. The `<ast>` varnode/op ids emitted here must
  be the same ids `EmitMarkup` references (see 2.5) — that consistency is the
  click-to-address contract.
- **Risk: LOW to the standalone engine (purely additive — a new method), HIGH in
  effort/correctness.** Getting the `<ast>` id scheme to match `EmitMarkup`'s
  `MarkupRef` (op `getTime()` / varnode `getCreateIndex()`) is the subtle part; a
  mismatch silently breaks click-to-address without breaking rendering.

### 2.5 PrintC → `EmitMarkup` — the marked-up C

The token-markup emitter is ported (`s9_emit/prettyprint.rs:719`, packed clang
doc) but **unreachable**: `PrintC` hardwires `EmitNoMarkup`
(`s9_emit/printc.rs:1015`, ctor). Phase 2 generalizes `PrintC`'s `emit` field and
wires `doc_function` (`printc.rs:1102`) to emit the markup document into the
`decompileAt` response after the `<function>`.

- **Proposed: an `enum` or `Box<dyn Emit>` for `PrintC.emit`**, defaulting to
  `EmitNoMarkup` (the standalone/console path, unchanged) and switched to the
  markup emitter only on the ghidra path. Also flips the console's stubbed
  `print C xml` (`kuna-console/src/ifacedecomp.rs:2095`) into a real command as a
  free side benefit.
- **Risk: MEDIUM.** `PrintC` is on the standalone hot path; generalizing its emit
  field must be zero-cost and zero-change for `EmitNoMarkup`. The 675 datatests
  render C through this exact path, so a regression is caught immediately. Keep
  the default variant a direct `EmitNoMarkup` (no dynamic dispatch) to avoid any
  standalone perf/behavior delta.

### 2.6 Wire `decompileAt` to the engine

Replace Phase 1's stub `rawAction` for `decompileAt`: `queryFunction(addr)` (which
lazily triggers `getMappedSymbols` — but Phase 2 can pre-query the one function at
`addr` and defer the general lazy scope to Phase 3), run the action, then emit the
`<doc>`: optional `<parammeasures>` (Phase 4), `Funcdata::encode` (2.4), and — when
`sendCcode` && action=="decompile" — the `docFunction` markup (2.5). The
incomplete-function empty-payload path stays as the failure shape.

- **Symbols in Phase 2:** to keep scope bounded, Phase 2 does the *minimum*
  symbol work to decompile one function: query the function's own mapped symbol
  and its immediate references eagerly. The full `ScopeGhidra` lazy cache/holes
  model (`database_ghidra.cc`) — required for correctness and speed on a real
  program — is **Phase 3**. Phase 2 output on a function that references many
  globals will be imperfect (missing names/types); that is expected and gated.

## 3. Sequencing (each step gated by `make test` + `test-stages` + `rust-test`)

1. **Translate enum seam, Sleigh-only** (2.2 refactor) — must be a no-op; full
   gates green before proceeding. *This is the go/no-go linchpin: if the enum
   refactor cannot be made behavior-preserving cleanly, the whole approach needs
   rethinking.*
2. `Architecture` construction from wire specs (2.1) + hand it `GhidraLoadImage`
   (2.3) — a registerProgram that builds a real (if symbol-starved) engine.
3. `GhidraTranslate` + `ContextGhidra` (2.2/2.3) — p-code flows from the host.
4. `Funcdata::encode` minimal `<function>` (2.4).
5. `PrintC` markup wiring (2.5) + `decompileAt` emit (2.6) — first C in the GUI.

Steps 1 and 5 are the ones that touch the standalone hot path; 2–4 are additive.

## 4. Testing

- **Unit** (in `kuna-ghidra`, MockJava-driven, as Phase 1): each provider against
  canned host answers; `GhidraTranslate::oneInstruction` against a canned
  `getPcode` document; the full `registerProgram → decompileAt → <doc>` shape.
- **Fixtures**: Ghidra's "Debug Function Decompilation" writes an `<xml_savefile>`
  capturing every callback answer (`DecompInterface.enableDebug`,
  `DebugDecompilerAction.java:38-73`) — the same document kuna's datatest corpus
  consumes. A capture gives us recorded host answers to replay `decompileAt`
  against without a live Ghidra, and a golden `<doc>` to diff. **Building a small
  capture corpus is the recommended Phase-2 regression backbone.**
- **Live smoke**: the extension + a real 12.2 release; decompile a function, eyeball
  the C, click a token, confirm it navigates.
- **The three standing gates stay green throughout** — every standalone-touching
  step (1, 5) lands behind them.

## 5. Divergences to record when they ship

- `setOptions` unknown-option tolerance (deferred from Phase 1) gets its
  `docs/divergences.md` DIV entry once `setOptions` actually decodes options.
- Any place `Funcdata::encode` or the markup emitter must filter varnodes the
  Java `PackedDecode.readSpace` rejects (fspec/iop/spacebase — it accepts only
  stack/join) is a wire constraint to honor, not a divergence, but note it.

## 6. Explicitly out of scope (Phase 3 / Phase 4)

- Lazy `ScopeGhidra` symbol cache with `<hole>` negatives + namespace resolution
  + the per-function `ArchSeam` snapshot rework (Phase 3).
- `TypeFactoryGhidra` lazy `findById`, `CommentDatabaseGhidra`, injects via the
  wire-fed cspec, `getStringData` charset fidelity (Phase 3).
- `<highlist>`/`<jumptablelist>` + DB symbol-id echo for rename/retype;
  `structureGraph`; `<parammeasures>`; the four signature commands; overlay
  spaces (Phase 4).

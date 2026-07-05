# Using kuna as Ghidra's decompiler core

How kuna replaces the native `decompile` process behind Ghidra's stock GUI — the
architecture, the wire protocol, what kuna already has, what each phase adds, and the
response contracts that make the GUI actually work. Audience: kuna developers
implementing Phases 2–4 (Phase 1 — the protocol-complete, engine-stubbed binary plus the
extension that spawns it — is on branch `feat/ghidra-mode`).

**Citation convention.** Ghidra sources are cited as `file:line` against the Ghidra
12.2-DEV checkout at commit `f9e13846` (2026-06-16). Java paths are under
`Ghidra/Features/Decompiler/src/main/java/ghidra/app/decompiler/` unless otherwise
noted; C++ paths under `Ghidra/Features/Decompiler/src/decompile/cpp/`. kuna's own
pinned port anchor is `GHIDRA_REV` `cef869af` (2026-06-01, `docs/UPSTREAM.md`); the one
protocol-relevant delta between the two revisions is called out in §8. kuna paths are
relative to `decompiler/crates/`.

The companion document **`docs/decompiler-core-interface.md`** specifies the same
interface from the *other* side — what Ghidra requires from *any* replacement core, with
no kuna knowledge assumed. This document is the kuna-specific plan; details of the
protocol that are fully specified there are only summarized here.

---

## 1. The decision: reimplement the native side, leave Ghidra alone

Ghidra's Java GUI talks to its decompiler through a **child process** named `decompile`,
spawned lazily on the first decompile request (`DecompInterface.java:267`) and driven
over stdin/stdout with a burst-framed binary protocol (`DecompileProcess.java:54-63`).
Upstream, that process is the C++ `ghidra_opt` build — the same DECCORE engine kuna
ported, linked against a `GHIDRA` glue group (`ghidra_process.cc`, `ghidra_arch.cc`,
`ghidra_translate.cc`, …) that kuna deliberately excluded from the port (LOSS-002,
`docs/rust-port/losses.md`).

**kuna reimplements exactly that glue group**: a new **`kuna-ghidra`** crate producing a
binary that speaks the full decompiler-process protocol, backed by kuna's engine. The
stock Ghidra GUI and its entire Java side stay byte-for-byte untouched; the only
Ghidra-side artifact is a tiny extension (`integrations/ghidra/KunaDecompiler/`, §7)
that points the spawn at our binary.

Rejected alternatives, for the record:

- **Fork `DecompInterface`/the Java chain.** The GUI chain is hardcoded `new` at every
  link (`DecompilePlugin.java:93` → … → `Decompiler.java:75` → `DecompInterface.java:267`)
  with ~28 `new DecompInterface()` call sites across the tree and no injection seam.
  A fork covers only the call sites you rewrite, breaks the "normal Ghidra GUI" goal,
  and is unmaintainable against upstream.
- **A shim process wrapping kuna's existing CLI.** The protocol is not
  request/response: the native side issues **callback queries mid-decompile**
  (per-instruction p-code, symbols at an address, bytes, types — §4) that a
  `kuna decompile-all` wrapper cannot answer; and the response is not C text but a
  typed dual-document (`HighFunction` + token markup with cross-referenced ids, §6)
  over live program state. There is no shim-shaped solution.

## 2. Architecture and session lifecycle

```
Ghidra GUI (untouched)                          kuna-ghidra binary
──────────────────────                          ─────────────────────
DecompilerProvider/Controller/Manager           main loop: readCommand()
  → DecompInterface                               ├─ registerProgram  → ArchitectureGhidra-equiv
    → DecompileProcessFactory.get()               ├─ setOptions/setAction (replayed state)
      → spawn kuna_ghidra  ◄──── extension        ├─ decompileAt ──┐
        (stdin/stdout bursts)                     │   queries back ─┘ getPcode/getBytes/…
                                                  ├─ flushNative   (after every function)
                                                  └─ deregisterProgram → exit
```

- **Spawn**: lazy, on first decompile; `runtime.exec(exepath)` with **no arguments and
  no environment changes** (`DecompileProcess.java:151`); Java waits 200 ms, checks
  `isAlive()`, and reads stderr into an error dialog if the process died
  (`DecompileProcess.java:154-192`). There is **no startup handshake** — the process
  silently waits for the first command.
- **Session**: `registerProgram` (four XML spec documents → a new engine session,
  answered with a decimal *archid*) → replayed session state (`setOptions`, `setAction`,
  print toggles, signature settings — `DecompInterface.java:262-352`) → any number of
  `decompileAt` + `flushNative` pairs (Java flushes the native caches after **every**
  decompiled function, `DecompInterface.java:826-832`) → `deregisterProgram`, which
  answers and then **terminates the process** (C++ sets `status=1`, ending the main
  loop, `ghidra_process.cc:242-248,533-535`; Java never reuses a deregistered process,
  `DecompileProcess.java:501-522`).
- **Timeout is process murder**: on decompile timeout Java disposes the process from a
  timer thread so the blocked read throws (`DecompileProcess.java:100-106,564-595`).
  The native side never sees a cancel message — it sees EOF, on which it must `exit(1)`
  (`ghidra_arch.cc:95-96`).
- **Respawn + replay**: any IOException marks the process bad; the next request
  respawns, re-registers the program, and replays options/action/toggles
  (`DecompInterface.java:354-361,262-352`). The native side needs no persistence — a
  fresh process must simply reach the same state from the replay.

## 3. The wire protocol, condensed

Full byte-level specification: `docs/decompiler-core-interface.md` §3–§5. Summary:

**Burst framing** (`DecompileProcess.java:54-63`; `ghidra_arch.cc:50-77`): every marker
is written as `{0x00,0x00,0x01,code}` and read tolerantly (skip garbage, one-or-more
`0x00`, expect `0x01`, then the code byte — `readToAnyBurst`, `ghidra_arch.cc:79-98`).
Even codes open, odd codes close.

| pair | meaning | direction |
|---|---|---|
| 2 / 3 | command | Java → native |
| 4 / 5 | callback query | native → Java |
| 6 / 7 | command response | native → Java |
| 8 / 9 | query response | Java → native |
| 10 / 11 | exception | both |
| 12 / 13 | byte-stream payload | both |
| 14 / 15 | string-stream payload | both |
| 16 / 17 | native message (warnings) | native → Java |

String streams (14/15) carry either raw ASCII (command names, decimal archids, the
`t`/`f` answers) or a **packed** binary document (kuna's bit-exact
`kuna-base/src/marshal.rs` `PackedEncode`/`PackedDecode`) — every packed byte is
nonzero, so the closing burst's leading `0x00` terminates ingestion. The only XML on
the wire is the four `registerProgram` spec strings.

**Commands** (dispatch `ghidra_process.cc:464-486`; every command's first parameter
after the name is the decimal archid in a string burst, *except* `registerProgram`):

| command | parameters after archid | response payload | queries legal during? |
|---|---|---|---|
| `registerProgram` | *(no archid)* pspec, cspec, tspec, coretypes — 4 XML string bursts (`ghidra_process.cc:162-173`) | new archid, ASCII decimal | **yes** |
| `deregisterProgram` | — | `1`/`0` decimal; then process exit | no |
| `flushNative` | — | `0` decimal | no |
| `decompileAt` | packed `<addr>` of the entry | packed `<doc>` (§6); empty 14/15 if incomplete | **yes** |
| `structureGraph` | packed block graph | packed restructured block graph | **yes** |
| `setAction` | actionstring, printstring (2 string bursts) | `t`/`f` | no |
| `setOptions` | packed `<optionslist>` | `t`/`f` | no |
| `generateSignatures` | packed `<addr>` | packed `<signatures>` | **yes** |
| `debugSignatures` | packed `<addr>` | packed debug-sig doc | **yes** |
| `getSignatureSettings` | — | packed `<sigsettings>` | no |
| `setSignatureSettings` | decimal settings string | `t`/`f` | no |

Choreography invariant: the response-open burst 6 is written **before** parameters are
read (`GhidraCommand::doit`, `ghidra_process.cc:125-135`), so callback queries nest
inside the open response; the result payload is followed by the 16/17 warnings frame
(always present, possibly empty — `ghidra_process.cc:108-116`), then 7, then flush.
The "queries legal" column is a **hard constraint**: Java nulls its callback
decoder/encoder for all other commands (`DecompileProcess.java:512-513,536-537,618-619,
657-658,686-687` vs `:472-473,571-572`) — a query issued during `setOptions` desyncs
the protocol.

**The 19 callback queries** (native → Java; element ids `ghidra_arch.cc:30-48` =
`ElementId.java:371-427`; framing: `4, 14 <packed command element> 15, 5, flush`, then
read `8 payload 9`, or `10 type msg 11` = a Java exception rethrown into the decompile):

| query | id | request params | response transport |
|---|---|---|---|
| `isNameUsed` | 239 | name, first (scope id), last | bool: raw `t`/`f` in 14/15 |
| `getBytes` | 240 | `<addr>` + size | byte burst 12/13, nibble-doubled (`'A'+hi,'A'+lo`); empty ⇒ DataUnavail |
| `getCallFixup` | 241 | name + `<context>` | packed `<inst>` of ops |
| `getCallMech` | 242 | name + `<context>` | packed `<inst>` |
| `getCallOtherFixup` | 243 | name + `<context>` | packed `<inst>` |
| `getCodeLabel` | 244 | `<addr>` | plain string (label, "" = none) |
| `getComments` | 245 | type-flag mask + `<addr>` | packed `<commentdb>` — **always written**, possibly empty |
| `getCPoolRef` | 246 | size n + n×`<value>` | packed cpool record |
| `getDataType` | 247 | name + id (signed) | packed `<type>`, or empty = not found |
| `getExternalRef` | 248 | `<addr>` | packed `<doc><mapsym>`, or empty |
| `getMappedSymbols` | 249 | `<addr>` | packed `<doc><mapsym>` / `<hole>`, or empty |
| `getNamespacePath` | 250 | id (unsigned) | packed `<parent>` with `<val>` per level |
| `getPcode` | 251 | `<addr>` | packed `<inst offset><addr/><op>…` or `<unimpl offset>`; empty ⇒ BadDataError, decompile **continues** |
| `getPcodeExecutable` | 252 | name + `<context>` | packed `<inst>` |
| `getRegister` | 253 | name | packed `<addr space offset size>` |
| `getRegisterName` | 254 | `<addr>` + size | plain string ("" = none) |
| `getStringData` | 255 | maxsize, type name, id + `<addr>` | byte burst: 2-byte biased length of len+1, raw trunc flag, nibble-doubled UTF-8 + doubled NUL |
| `getTrackedRegisters` | 256 | `<addr>` | packed `<tracked_pointset>` — **always written** |
| `getUserOpName` | 257 | index (signed) | plain string ("" = end of table) |

Registration-time traffic the client expects: the `getUserOpName` probe loop (index
0,1,2,… until "") fires during `registerProgram` init (`ghidra_translate.cc:107-117`),
establishing the CALLOTHER index table.

## 4. P-code comes from Java, per instruction (decided)

kuna ships a full SLEIGH engine and vendors every upstream spec, so the tempting
shortcut is to disassemble locally and use the wire only for bytes/symbols. **Rejected**
— Phase 2 ports `GhidraTranslate` faithfully (`ghidra_translate.cc:120-156`: one
`getPcode` query per instruction, register-name caches, no p-code cache), because:

1. **There is no wire query for low-level disassembly context.** The upstream
   `ContextGhidra` throws on every context-variable method
   (`ghidra_context.hh:36-47,60-74`) — disassembly context (ARM/Thumb, MIPS16,
   context-register state per address) lives only in Java's program database. A local
   SLEIGH engine would disassemble Thumb code as ARM with no way to know better.
2. **Listing parity.** What Java disassembled — including user overrides, manual
   context, length-override instructions — is exactly what gets decompiled. That
   invariant is the point of the product: the decompiler view always matches the
   listing view.
3. **It is what the wire contract assumes.** The spawn passes no arguments to identify
   a language; the tspec carries only address spaces + endianness + uniqbase, not a
   language id (`SleighLanguage.encodeTranslator`); and `flushNative`'s cache semantics
   (Java flushes after every function) are designed around Java being the source of
   truth for all program facts. A local-SLEIGH kuna would need out-of-band language
   resolution the protocol simply does not carry.

Consequence for kuna: `Architecture.translate` is today a **concrete `Sleigh`** field
(`kuna-decomp/src/infra/architecture.rs:836`). Phase 2 introduces a `Translate` seam on
`Architecture` — an **enum over `{Sleigh, GhidraTranslate}`**, not a `Box<dyn>`, to keep
`manage()` (the space-manager accessor) concrete and the standalone path untouched. The
lift stage is already trait-typed at the consumption point
(`FlowEnvironment::translate() -> &dyn Translate`, called at
`kuna-decomp/src/s2_lift/flow.rs:1321`), and the trait itself
(`kuna-sleigh/src/translate.rs:386`) is a faithful port of the C++ virtual. The
space-manager-without-`.sla` problem is already solved: `insert_space`
supports the sparse tspec indices (`kuna-base/src/space.rs:2545-2566`) and a unit test
builds a manager purely from decoded `<space>` elements
(`kuna-sleigh/src/translate.rs:1086-1124`). Phase 1 already ships the
`GhidraTranslate::decode` equivalent — `kuna-ghidra/src/translate.rs` parses the tspec
`<sleigh>` element (endianness, `uniqbase`, the `<space>`/`<space_unique>`/`<space_other>`/
`<space_overlay>` list, `<truncate_space>`) into a real `AddrSpaceManager`, which is why
`decompileAt` already decodes real `<addr>` parameters (`ghidra_translate.cc:161-176`).
Phase 2 only wires that manager into an `Architecture`.

## 5. Symbols, types, comments, strings, injects: pull-only ⇒ lazy caches (decided)

The wire has **no enumerate-the-program query**. Symbols arrive one address at a time
(`getMappedSymbols`), types one id at a time (`getDataType`), comments per function
(`getComments`), strings per address (`getStringData`), inject bodies per call site
(`getPcodeInject` family). Eager pre-population — the pattern kuna's analysis tier uses
(`ConsoleProgram::commit_pending_analysis`) — is therefore **impossible**, not merely
inferior: there is nothing to enumerate. kuna needs the upstream lazy model:

- **`ScopeGhidra`** (`database_ghidra.cc`): every lookup checks a local cache, then
  queries, then materializes the answer — including negative answers as `<hole>` ranges
  so the same miss is never re-queried, readonly/volatile flags folded into the
  property map with dirty-tracking, and namespaces rebuilt on demand via
  `getNamespacePath`.
- **`TypeFactoryGhidra`** (`typegrp_ghidra.cc:20-36`): one override — `findById` miss →
  `getDataType` → decode into the local factory.
- **`CommentDatabaseGhidra`** (`comment_ghidra.cc:30-49`): fill-once-per-function from
  `getComments`, filtered by the printer's current comment settings.
- All of it flushed by `flushNative` after every function
  (`ghidra_process.cc:262-273`): global scope, sub-scopes, non-core types, comments,
  string decodings, cpool.

This is what forces the Phase-3 seam work on kuna's concrete types: the C++ virtuals
these classes override were deliberately collapsed in the port (the `Scope`/
`ScopeInternal` merge, `kuna-decomp/src/p0_knowledge/database.rs:904-922`), and the
per-function `ArchSeam` snapshot (`substrate/seams.rs:411`, built by
`build_arch_handle`, `architecture.rs:1529`) assumes all facts are known before the
decompile starts — mid-decompile discovery must either rebuild those snapshots or route
queries through them. See the seam inventory (§9) and the Phase 3 checklist (§11).

## 6. Response contracts that make the GUI work

Printing correct C is not enough. The `decompileAt` response is a packed `<doc>`
(ELEM_DOC=229) whose Java consumption (`DecompileResults.java:215-264`) imposes:

- **Dual `<function>` elements, order load-bearing**: the *first* decodes as the
  `HighFunction` (prototype, `<localdb>` symbols, `<ast>` varnodes+ops, `<highlist>`,
  `<jumptablelist>`), the *second* is the Clang token markup — an explicitly-commented
  "ugly kludge" around duplicate tag names. The GUI renders only if **both** decode
  (`DecompileData.java:56`); headless consumers survive on the first alone.
- **Name + entry echo**: `HighFunction.decode` throws on a function-name or
  entry-address mismatch with the Java-side `Function`
  (`Framework/SoftwareModeling/.../pcode/HighFunction.java:245-293`).
- **ast ↔ markup refid consistency**: markup tokens carry `ATTRIB_VARREF`/`ATTRIB_OPREF`
  that Java resolves *against the first `<function>`'s decoded `<ast>`*
  (`ClangVariableToken.java:147-163`, `PcodeSyntaxTree.java:309,365`). Click-to-address,
  hover, highlight, and rename-target resolution all ride on these ids — kuna's
  `EmitMarkup` `MarkupRef` fields (op time / varnode create-index) must match what
  `Funcdata::encode` emits into the `<ast>`.
- **DB symbol-id echo**: `<mapsym>` symbols must carry the *real Ghidra database symbol
  ids* they were delivered with; ids in the internal `0x4000000000000000` range are
  never round-tripped (`HighSymbol.java:39,386-387`). Rename/retype is a DB write keyed
  by that id followed by an event-driven re-decompile
  (`RenameVariableTask.java:51-57`, `DecompilerProgramListener.java:60,82`) — a wrong id
  silently breaks rename.
- **`<jumptablelist>`** feeds the switch analyzer (`DecompilerSwitchAnalysisCmd.java:100`,
  configured with C-code *off* and jumpload *on* — the list must be emitted even when
  the markup isn't).
- **`<parammeasures>`** (action `paramid` + the parammeasures toggle) feeds the
  Decompiler Parameter ID and calling-convention analyzers
  (`DecompilerParameterIdCmd.java:325-345`).

## 7. The extension seam (decided)

Primary mechanism — **reflection swap of `DecompileProcessFactory.exepath` from a
plugin**. The factory caches the resolved path in a private static with an early-return
(`DecompileProcessFactory.java:28,52-55`); the native process spawns lazily on the first
decompile (`DecompInterface.java:267`), so any plugin constructor in the tool runs
inside the pre-spawn window. Ghidra's flat classpath (all classes in the unnamed module,
`GhidraClassLoader.java:34`) means `setAccessible(true)` works with no `--add-opens`.
The extension (`integrations/ghidra/KunaDecompiler/`) resolves the binary in two steps:
first the `-Dkuna.decompiler.exe=<path>` dev override (a JVM system property, for
pointing at a fresh `cargo build` without reinstalling the extension), then — the normal
path — the kuna binary shipped in the extension's **own** module `os/` dir under the
**distinct name `kuna_ghidra`**, resolved with the two-arg
`Application.getOSFile(moduleName, filename)` (`Application.java:1000-1003`). Naming it
`decompile` could never work, because the single-arg lookup searches the *calling*
class's module first and `DecompileProcessFactory` lives in the Decompiler module,
which ships its own (`Application.java:1013-1026`).

Documented fallbacks (no code, release installs): **build/os file-drop** — a binary at
`<install>/Ghidra/Features/Decompiler/build/os/<platform>/decompile` shadows the stock
`os/` copy because `getModuleOSFile` checks `build/os/` first unconditionally; and the
**patch-dir class shadow** — `<install>/Ghidra/patch` jars precede module jars in
release mode only (`GhidraLauncher.java:182-183`). Full ranking and the why-nots:
`docs/decompiler-core-interface.md` §2.

## 8. Version policy (decided)

`kuna-ghidra` targets a **pinned Ghidra release** (the 12.2 vintage). There is no
protocol handshake — the interface version (major=6/minor=1,
`cpp/architecture.cc:35-36`) is exposed only through `getSignatureSettings` and only
BSim reads it. The real skew risk is **option drift**, already live between kuna's
`GHIDRA_REV` (`cef869af`) and the 12.2-DEV head (`f9e13846`): upstream added a
`baddatacount` option (ELEM_BADDATACOUNT=290, moving ELEM_UNKNOWN to 291,
`ElementId.java:293,464`), and an older core receiving that unknown `<optionslist>`
element throws `ParseError` → `setOptions` answers `f` → Java fails the whole
program-open with "Did not accept decompiler options" (`DecompInterface.java:301-303`).
kuna therefore **deliberately diverges from upstream: unknown option elements are
skipped with a warning instead of failing the command** — one stale option must not
brick the decompiler view. This is output-invariant for known options; it gets a DIV
entry in `docs/divergences.md` when the behavior ships (Phase 2, when `setOptions`
stops being a stub).

A second, smaller **deliberate divergence** hardens the process against a malformed
archid. Upstream reads the id with `sin >> dec >> id` (`ghidra_process.cc:89-96`); on a
non-numeric or overflowing payload C++11 leaves the stream in a failed state, and the
next `readToAnyBurst` sees the failbit and `exit(1)`s the whole process
(`ghidra_arch.cc:95-96`) — one bad command kills the session. kuna's `parse_arch_id`
instead returns `-1`, which the caller turns into the ordinary "No architecture
registered with decompiler" `JavaError`: the client sees a clean exception on that one
command and the process stays alive for the next.

## 9. What kuna already has vs. the seam inventory

Already in the tree (verified):

| asset | where | state |
|---|---|---|
| Packed marshaling, bit-exact | `kuna-base/src/marshal.rs` (`PackedDecode` :1424, `PackedEncode` :2008) | done — the payload codec is the ported one |
| `KunaError::Java` | `kuna-base/src/error.rs:145-153` | done — the C++ `JavaError` carrier |
| Token-markup emitter | `kuna-decomp/src/s9_emit/prettyprint.rs:719` (`EmitMarkup`, packed clang doc) | ported, **unreachable** — PrintC hardwires `EmitNoMarkup` (`printc.rs:1015`) |
| Signature engine | `kuna-decomp/src/infra/signature.rs` + `analyzesigs.rs` | ported; the four signature commands are wire-glue, not engine work |
| Signature element ids | `signature.rs:73-87` — 258, 259, 260, 263, 265, 266, 267, 269, upstream-numbered | done; the gaps 261/262/264/268 are ours to add |
| tspec-driven space manager, no `.sla` | `kuna-base/src/space.rs:2545-2566`; `kuna-ghidra/src/translate.rs` (`GhidraTranslate::decode`) | **done in kuna-ghidra** — the tspec `<sleigh>` parse builds a real `AddrSpaceManager`; Phase 2 wires it into an `Architecture` |
| `LoadImage` trait | `kuna-sleigh/src/loadimage.rs:101`, consumed as `Box<dyn>` everywhere | ready |
| `ContextDatabase` trait | `kuna-sleigh/src/globalcontext.rs:340` | ready |
| `Translate` trait + trait-typed lift | `kuna-sleigh/src/translate.rs:386`; `flow.rs:1321` | ready at the consumer; owner is concrete |

Protocol element ids to add (upstream numbers, wire compat — kuna's own 4000+ range is
for kuna-invented ids only, and none of these are taken): **229** (`doc`), **239–257**
(the query commands, §3 — note 241=`getcallfixup`, 242=`getcallmech`,
243=`getcallotherfixup`, verified against `ghidra_arch.cc:30-48` and
`ElementId.java:377-385`), **261/262/264/268** (`major`/`minor`/`settings`/`sigsettings`).

The engine-seam inventory, with honest difficulty:

| seam | kuna today | work | phase |
|---|---|---|---|
| `LoadImage` | trait ready | **trivial** — `GhidraLoadImage::load_fill` = `getBytes` | 2 |
| `ContextDatabase` | trait ready | **trivial** — `ContextGhidra` implements `getTrackedSet` only (upstream throws on the rest, `ghidra_context.hh:36-47`) | 2 |
| `Translate` | trait exists; `Architecture.translate: Sleigh` concrete (`architecture.rs:836`) | **enum seam** `{Sleigh, GhidraTranslate}` + the Sleigh-only call surface audit | 2 |
| `Funcdata::encode` | **missing** (no encode on `substrate/funcdata*.rs`) | port from upstream `funcdata.cc` — minimal `<function>` first | 2 |
| PrintC → `EmitMarkup` | back-end ported, front-end hardwired (`printc.rs:1015`) | generalize PrintC's `emit` field; wire `doc_function` (`printc.rs:1102`) to the markup path | 2 |
| Scope / symbol table | polymorphism deliberately collapsed (`database.rs:904-922`) | **redesign** — lazy query-through cache + `<hole>` negatives + namespace path resolution (the `ScopeGhidra` model) | 3 |
| `TypeFactory` | trait exists; owners name `TypeFactoryImpl` (`architecture.rs:765`, `seams.rs:630`) | trait-plumbing for lazy `findById` | 3 |
| `CommentDatabase` | full trait ported (`s9_emit/comment.rs:280`) but `Architecture.commentdb` is a minimal stand-in (`architecture.rs:143`) | rewiring, small | 3 |
| `StringManager` | concrete, no trait (`stringmanage.rs:83`) | one-method extraction for Java-side charset-faithful decode | 3→4 |
| Inject library | traits exist; owner concrete; SLEIGH inject engine compiles cspec snippets | reuse the SLEIGH engine over the **wire-fed cspec** (the snippets travel inside it) | 3 |
| `ConstantPool` | trait ready (`infra/cpool.rs:470`) but **unwired** into `Architecture` | wiring + `CPOOLREF` path; JVM/Dalvik only | 4 |
| ArchSeam snapshot | per-function plain-data snapshot (`seams.rs:411`, `build_arch_handle` `architecture.rs:1529`) | rework so mid-decompile discovery (new symbols/types from queries) reaches the pipeline | 3 |

## 10. Graceful degradation

What breaks when a partial core omits pieces — all failure modes are clean:

| omitted | Java-side behavior | blast radius |
|---|---|---|
| signature commands | answered "Bad command" (the `6, 16 "Bad command: <name>" 17, 7` pattern, `ghidra_process.cc:476-484`); Java shows "not built with signature module" (`DecompInterface.java:341-347`) | **BSim only** — GUI unaffected |
| `structureGraph` | same "Bad command" path | FunctionGraph **nested-layout** view only |
| empty `decompileAt` payload (function failed) | `decompileCompleted()` false; the 16/17 warnings text becomes the error message | that one function — clean GUI error |
| second `<function>` (markup) | HighFunction decodes; GUI panel refuses to render (`DecompileData.java:56`) | GUI blank for that function; analyzers/scripts on HighFunction still work |
| `<parammeasures>` | param-ID / convention analyzers get nothing | those analyzers no-op |
| `<jumptablelist>` | switch analyzer finds no tables | unrecovered switches in the listing |

## 11. Testing strategy

- **Phase 1 (now): in-crate mock-Java e2e.** A `MockJava` test double owns the other
  end of the pipe: sends command bursts, answers queries from canned tables, asserts
  the byte-exact response framing (including the response-open-before-params ordering,
  the always-present 16/17 frame, and the self-alignment/exception paths). This is the
  same differential discipline as the port — the protocol spec in
  `docs/decompiler-core-interface.md` is the oracle.
- **Phase 2+: DecompileDebug captures as fixtures.** Ghidra's "Debug Function
  Decompilation" action (`DecompInterface.enableDebug`, `DebugDecompilerAction.java:38-73`)
  records every callback answer into an `<xml_savefile>` — exactly the document kuna's
  datatest corpus already consumes. Captures give us *recorded Java-side query answers*
  to replay against `kuna-ghidra` without a live Ghidra, and `DecompileDebugXmlLoader`
  (Features/Base) can import them as Programs for the reverse direction.
- **Live smoke procedure** (manual, per phase): build the extension, install into a
  Ghidra 12.2 release (or `-Dghidra.external.modules=` in a dev checkout), enable the
  plugin, open a known binary, decompile; verify the spawned PID is `kuna_ghidra` and —
  from Phase 2 — that the C, click-to-address, and rename round-trip behave.
- **Regression floors**: the three standing gates (`make test`, `make test-stages`,
  `make rust-test`) must stay green — `kuna-ghidra` is additive and must not perturb
  the standalone engine.

## 12. Phase breakdown

**Phase 1 — wire-protocol-complete, engine-stubbed (this branch).**
- [x] `kuna-ghidra` crate: burst framer, command registry + `doit()` lifecycle,
      archlist session model, the 19 typed query clients, exception channels — engine
      calls stubbed.
- [x] Protocol element ids 229, 239–257, 261, 262, 264, 268 (upstream numbers).
- [x] tspec `<sleigh>` parse → real `AddrSpaceManager` (`GhidraTranslate::decode`), so
      `decompileAt` decodes real `<addr>` parameters.
- [x] Ghidra extension (`integrations/ghidra/KunaDecompiler/`): plugin + reflection
      exepath swap, binary shipped as `os/<platform>/kuna_ghidra`.
- [x] These two documents.

**Phase 2 — engine bridge (first real C in the GUI).**
- [ ] `ArchitectureGhidra`-equivalent construction path: Architecture from the four
      wire spec documents (wire the Phase-1 tspec space manager into the engine;
      cspec/pspec via the existing `set_cspec_xml`/`set_pspec_xml` + `init_post_engine`).
- [ ] `Translate` enum seam on `Architecture` (`{Sleigh, GhidraTranslate}`);
      `GhidraTranslate` port (getPcode per instruction, register caches, user-op probe
      loop).
- [ ] `GhidraLoadImage` (`load_fill` = `getBytes`); `ContextGhidra::getTrackedSet`.
- [ ] `Funcdata::encode` — minimal `<function>` (name/addr + `<ast>` + prototype).
- [ ] PrintC → `EmitMarkup` wiring; `doc_function` markup path; dual-`<function>`
      `decompileAt` response.

**Phase 3 — lazy providers (correct symbols/types at scale).**
- [ ] `ScopeGhidra`-equivalent lazy symbol cache: query-through + `<hole>` negatives +
      property-range side effects + namespace resolution + DB symbol-id fidelity.
- [ ] `TypeFactory` lazy `findById` trait-plumbing; comments rewiring
      (`CommentDatabaseGhidra`); injects via the wire-fed cspec snippets.
- [ ] ArchSeam snapshot rework for mid-decompile discovery; `flushNative` clearing
      semantics end-to-end.

**Phase 4 — parity.**
- [ ] `<highlist>`/`<jumptablelist>` fidelity incl. DB symbol-id echo for
      rename/retype; `structureGraph`; `<parammeasures>`.
- [ ] The four signature commands (engine already ported) for BSim.
- [ ] Overlay spaces (Java swaps in overlay codecs transparently —
      `DecompInterface.java:84-127,896-909`); `getStringData` charset fidelity
      (Java-side decode instead of `GhidraLoadImage` bytes).

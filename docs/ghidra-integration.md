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
pinned port anchor is `GHIDRA_REV` `cef869af` (2026-06-01, `docs/history.md`); the one
protocol-relevant delta between the two revisions is called out in §8. kuna paths are
relative to `decompiler/crates/`.

---

## 1. The decision: reimplement the native side, leave Ghidra alone

Ghidra's Java GUI talks to its decompiler through a **child process** named `decompile`,
spawned lazily on the first decompile request (`DecompInterface.java:267`) and driven
over stdin/stdout with a burst-framed binary protocol (`DecompileProcess.java:54-63`).
Upstream, that process is the C++ `ghidra_opt` build — the same DECCORE engine kuna
ported, linked against a `GHIDRA` glue group (`ghidra_process.cc`, `ghidra_arch.cc`,
`ghidra_translate.cc`, …) that kuna deliberately excluded from the port (LOSS-002,
see `docs/history.md`).

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

The byte-level ground truth is the upstream implementation (`DecompileProcess.java`,
`ghidra_arch.cc`). Summary:

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

The C++ virtuals these classes override were deliberately collapsed in the port (the
`Scope`/`ScopeInternal` merge, `kuna-decomp/src/p0_knowledge/database.rs`), so Phase 3
relocated the lazy model to the seams the kuna pipeline actually reads
(**shipped** — `kuna-decomp/src/infra/remote_provider.rs`):

- **`RemoteScope`** (the `ScopeGhidra` port at the GlobalQuery boundary): installed on
  `Architecture` at registerProgram and threaded into every per-function `ArchContext`
  by `build_arch_handle`; every global-scope read (`query_global_properties`,
  `name_for_global_varnode*`, `query_container_global`, `query_callee_proto`,
  `query_function`, `callee_proto_pieces`) and the flow environment's callee
  name/no-return queries resolve through `effective_global_query`/`function_at` —
  query-through with `<hole>` negative caching, decoded-entry positive caching,
  readonly/volatile property paints over a `lockDefaultProperties` snapshot, and
  namespace paths via getNamespacePath.  The wire is reached through the
  `RemoteProviderFetch` trait (implemented in `kuna-ghidra/src/provider.rs` over the
  shared client); the standalone path installs nothing and stays byte-identical.
- **`TypeFactoryGhidra`**: the wire `<coretypes>` decode replaces the default core
  types at registerProgram (so kuna's type ids match the host's), and
  `TypeFactoryImpl::find_by_id_or_remote` fetches unknown types with getDataType
  through the `RemoteTypeFetch` trait (`substrate/dtype.rs` `decode_type`/
  `decode_type_no_ref`/`decode_core_types`); `clear_noncore` evicts on flush.
- **`CommentDatabaseGhidra`**: `RemoteScope::fill_comments` fills the comment
  database once per flush cycle from getComments, filtered by the printer's comment
  settings (empty filter ⇒ no query).
- **External references** resolve through the upstream two-step
  (`resolveExternalRefFunction`): an `<externrefsymbol>` answer keeps its resolve
  address and fires getExternalRef at the POINTER address; the returned function
  materializes at its own entry (name/prototype/noreturn), and the pointer symbol
  itself types as pointer-to-code.
- **Tracked registers**: the pspec `<tracked_set>` decodes as the static default,
  and `ContextGhidra` is wired for real — decompileAt issues getTrackedRegisters
  at the entry (cached until flushNative) and merges the host's values OVER the
  pspec defaults, so per-address host context (MIPS `gp`, PPC TOC, a user 'Set
  Register Value') reaches `ActionConstbase`.
- **setOptions** follows the upstream reset-then-apply contract: Java
  delta-encodes its option list, so every `setOptions` first restores the
  registerProgram baseline (`Architecture::reset_wire_defaults` + the DIV-77
  ghidra-mode preset layer) and then applies the deltas — a previously-sent
  non-default option reverts when the user sets it back to default.
- **The `Kuna v…` banner**: ghidra-mode prints a one-line plate comment
  (`/* Kuna v<MAJOR.MINOR> */`, the release `KUNA_VERSION` bake) at the top of
  every decompiled function so it is visible in the GUI that kuna is the active
  core.  Cache-only (never written back to the host); standalone output has no
  banner.
- **flushNative** clears everything in the upstream order
  (`Architecture::flush_remote_caches`): the lazy symbol cache + property rollback
  + the per-address tracked-register cache, the non-core types, the comment
  database, the decoded strings.

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
release mode only (`GhidraLauncher.java:182-183`).

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
brick the decompiler view. This is output-invariant for known options; it shipped
with Phase 3 (`OptionDatabase::decode_lenient` behind the real `setOptions`) and is
recorded as **DIV-76** in `docs/history.md`.

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
| `Funcdata::encode` | **DONE (Phase 4)** — the FULL `<function>` in upstream child order: `<addr>` + `<localdb>` + `<ast>` + `<highlist>` + `<jumptablelist>` + `<prototype>` (`substrate/funcdata_encode.rs`), over the `Datatype::encodeRef` / `FuncProto::encode` / `Symbol`/`SymbolEntry`/scope encode ports and the encode-time symbol-link pass | `<override>`/child-`<scope>` statics deferred (Java skips both) | 4 ✔ |
| PrintC → `EmitMarkup` | back-end ported, front-end hardwired (`printc.rs:1015`) | generalize PrintC's `emit` field; wire `doc_function` (`printc.rs:1102`) to the markup path | 2 |
| Scope / symbol table | **DONE (Phase 3)** — `RemoteScope` (`infra/remote_provider.rs`): query-through + `<hole>` negatives + property paints + namespace paths, at the GlobalQuery/flow seams; **Phase 4** echoes the delivered DB symbol ids back (`GlobalEntry::symbol_id` → `<high symref>`) | — | 3 ✔ / 4 ✔ |
| `TypeFactory` | **DONE (Phase 3)** — wire `<coretypes>` decode + `decode_type`/`<typeref>` + `find_by_id_or_remote` getDataType miss-hook + `clear_noncore` (`substrate/dtype.rs`); **Phase 4** adds the marshal-OUT (`Datatype::encode_ref`/`encode`) | function-definition `<prototype>` bodies inside `<type metatype="code">` are skipped (FuncProto::decode unported) | 3 ✔ / 4 ✔ |
| `CommentDatabase` | **DONE (Phase 3)** — `RemoteScope::fill_comments` (getComments, printer-filtered, fill-once-per-flush) into the `Architecture.commentdb` sink | the printer renders PRE/warning + header comments; EOL/POST placement is a printer gap, not a wire gap | 3 ✔ |
| `ParamIDAnalysis` | **DONE (Phase 4)** — `<parammeasures>` encode + the paramid-action doc shape (`infra/paramid.rs`; the justproto constructor reads the real recovered `FuncProto`) | — | 4 ✔ |
| `StringManager` | concrete, no trait (`stringmanage.rs:83`); cleared by flushNative | one-method extraction for Java-side charset-faithful decode | deferred |
| Inject library | **DONE** — payloads register from the wire-fed cspec, but nothing compiles them without a local `.sla`, so each injection fetches already-lifted p-code for that call site via getPcodeInject (`EngineTranslate::fetch_inject_pcode`, `kuna-ghidra/src/translate.rs`); never cached, since the answer is bound to the site | `CALLMECHANISM`/`EXECUTABLEPCODE` payloads are unreachable from the live engine and stay unwired. The call-fixup arm is wired but also unreachable: `remote_provider.rs` decodes every remote function with `func_inject_id: -1` and the remote `<prototype>` has no `<inject>` arm, so a Ghidra-side `setCallFixup` never arrives | ✔ |
| `ConstantPool` | trait ready (`infra/cpool.rs:470`) but **unwired** into `Architecture` | wiring + `CPOOLREF` path; JVM/Dalvik only | deferred |
| ArchSeam snapshot | **DONE (Phase 3)** — the three frozen tables read through live seams when a `RemoteScope` is installed (`ArchContext::effective_global_query`/`callee_proto_pieces`; tracked sets decode from the wire pspec `<tracked_set>`) | — | 3 ✔ |

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

- **In-crate mock-Java e2e (Phase 1, shipped).** A `MockJava` test double owns the
  other end of the pipe: sends command bursts, answers queries from canned tables,
  asserts the byte-exact response framing (including the response-open-before-params
  ordering, the always-present 16/17 frame, and the self-alignment/exception paths) —
  `kuna-ghidra/tests/protocol_e2e.rs` (canned streams) and `tests/decompile_at_e2e.rs`
  (the interactive loopback, one canned RETURN function). This is the same
  differential discipline as the port — the upstream implementation
  (`DecompileProcess.java`, `ghidra_arch.cc`) is the oracle.
- **ghidra-sim: the real-program differential harness (shipped with Phase 2).**
  `kuna-ghidra/tests/ghidra_sim_e2e.rs` + the shared machinery in
  `tests/ghidra_sim/` drive the FULL wire lifecycle in-process (registerProgram →
  setAction → decompileAt×N → flushNative → deregister) against vendored ELFs
  (`tests/bug-repro/faillog` fast; `sort`/`grep` breadth), with the mock-Java end
  answered from **kuna's own analysis** (`ghidra_sim/oracle.rs`:
  `bootstrap_from_object` for bytes/labels, the real Sleigh re-encoded as wire
  `<inst>` documents for getPcode, a tspec *generated* from the Sleigh's space
  manager so packed space indices agree by construction; getMappedSymbols answers
  empty — the Phase-2 reality). It asserts the §6 response contracts (dual
  `<function>` decode, name/entry echo, markup opref/varref ⊆ ast, the 19-query
  legality and query-legal-command placement), flattens the Clang markup to C with
  Java's `getC()` token cleaning applied (`IllegalCharCppTransformer` — the reason
  over-wide type tokens read back as `unsigned_long__a1`), and **pins today's
  GUI-path quality numbers**: raw-register identifier leaks, `Unique<hex>` tokens,
  `sub_`/`dat_` placeholder rate vs what the loader knows, `getC()`-mangled token
  counts, query traffic (getPcode totals, getMappedSymbols == 0), and the
  normalized line-diff ratio against the SAME functions through the in-process CLI
  path. The pins are inline consts in `ghidra_sim_e2e.rs`; they move only with the
  provider/emitter change that earns the move — Phase 3/4 land by flipping them
  downward. Run: `make test-ghidra` (release; also part of the CI `gates` job on
  every PR, which matters because the workspace suite is skipped on internal PRs).
- **Phase 3+: DecompileDebug captures as fixtures.** Ghidra's "Debug Function
  Decompilation" action (`DecompInterface.enableDebug`, `DebugDecompilerAction.java:38-73`)
  records every callback answer into an `<xml_savefile>` — exactly the document kuna's
  datatest corpus already consumes. Captures give us *recorded Java-side query answers*
  to replay against `kuna-ghidra` without a live Ghidra (de-risking the sim's
  synthesized `<mapsym>` shapes), and `DecompileDebugXmlLoader` (Features/Base) can
  import them as Programs for the reverse direction.
- **Live smoke** (manual, per phase): `integrations/ghidra/live-smoke/` — a
  pyghidra rig that swaps `DecompileProcessFactory.exepath` to `kuna_ghidra` inside
  a real Ghidra, decompiles the same functions with both cores, and writes a
  side-by-side report with the same scanner counts the sim pins (see its README
  for the offline-pyghidra setup). For the full extension path: build the
  extension, install into a Ghidra 12.2 release (or `-Dghidra.external.modules=`
  in a dev checkout), enable the plugin, verify the spawned PID is `kuna_ghidra`
  and that the C, click-to-address, and rename round-trip behave.
- **Regression floors**: the standing gates (`make test`, `make test-stages`,
  `make rust-test`, now plus the `gates`-job ghidra-sim step) must stay green —
  `kuna-ghidra` is additive and must not perturb the standalone engine.

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

**Phase 2 — engine bridge (first real C in the GUI). DONE** (PR #135, branch
`feat/ghidra-mode-phase2`). Full plan + risk assessment + sequencing: the retired
`docs/rust-port/ghidra-phase2-plan.md` (git history). Each step landed as its own commit
keeping `make test` at 675/675 PARITY OK. One design refinement vs the plan: the
translator seam is a `Box<dyn EngineTranslate>` **trait**, not an enum — an enum
variant naming `kuna-ghidra`'s `GhidraTranslate` would be a `kuna-decomp`↔`kuna-ghidra`
dependency cycle; a trait object owned in `kuna-decomp` is not.
- [x] `SharedClient` design (the re-entrant provider→query pattern) +
      `GhidraLoadImage` (`load_fill` = `getBytes`) — `kuna-ghidra/src/provider.rs`.
- [x] `EngineTranslate` trait seam on `Architecture` (`translate: Box<dyn EngineTranslate>`,
      `infra/engine_translate.rs`) — behavior-preserving; only `Sleigh` implements it today.
- [x] `GhidraTranslate` port (getPcode per instruction, register caches, user-op probe
      loop) — `kuna-ghidra/src/translate.rs`. `ContextGhidra` is a `ContextInternal`
      stub (empty tracked sets; the query-backed `getTrackedRegisters` is Phase 3).
- [x] `Architecture::from_engine_translate` construction path: `registerProgram` builds
      a live engine from the wire specs (tspec→manager, cspec/pspec via
      `set_cspec_xml`/`set_pspec_xml`, `init_post_engine`; wire `<coretypes>` decode
      deferred — uses the default `build_core_types`, Phase 3).
- [x] `Funcdata::encode` — minimal `<function>`/`<ast>` (name/addr + ast;
      `<prototype>`/`<localdb>`/`<highlist>`/`<jumptablelist>` deferred).
- [x] PrintC → `EmitMarkup` wiring (`PrintEmit` enum, `doc_function_markup`, `MarkupRef`
      population); the dual-`<function>` `decompileAt` response, driven by `decompile_func`
      (name from `getCodeLabel`, flow-discovered extent). Proven end to end
      (`kuna-ghidra/tests/decompile_at_e2e.rs`: the response decodes to a `<function>`
      + a markup `<function>` whose refs resolve against the `<ast>`).

**Phase-2 scope boundary (historical):** Phase 2's `decompileAt` ran against an
*empty* `ScopeInternal` global scope — placeholders (`sub_ADDR`/`DAT_ADDR`/default
types) everywhere, no `getMappedSymbols` traffic. Phase 3 (below) replaced that with
the lazy providers; the ghidra-sim harness pinned the Phase-2 gap numerically and its
pins now hold the Phase-3 level.

**Phase 3 — lazy providers (correct symbols/types at scale). DONE** (branch
`feat/ghidra-phase3-providers`; DIV-76/DIV-77 in `docs/history.md`).
- [x] `ScopeGhidra`-equivalent lazy symbol cache (`RemoteScope`,
      `kuna-decomp/src/infra/remote_provider.rs`): getMappedSymbols query-through at
      the GlobalQuery + flow seams, the `<mapsym>`/`<hole>`/symbol-family decoder,
      `<hole>` negative caching, readonly/volatile property paints over the
      `lockDefaultProperties` snapshot, namespace paths via getNamespacePath, and
      wire symbol-id capture (`RemoteSymbolRecord::symbol_id`, the Phase-4 echo seed).
      decompileAt resolves the current function's identity (name + locked prototype
      pieces) through it, getCodeLabel demoted to fallback.
- [x] `TypeFactory` wire decode + lazy `findById`
      (`decode_core_types`/`decode_type`/`find_by_id_or_remote` + `RemoteTypeFetch`,
      `clear_noncore`); comments rewiring (`fill_comments`, printer-filtered);
      tracked registers from the wire pspec `<tracked_set>` (the `DF = 0` seed).
      Injects via the wire-fed cspec snippets shipped with Phase 2.
- [x] ArchSeam snapshot rework (the frozen tables read through the live provider
      when installed); `flushNative` clearing end-to-end in the upstream order
      (`Architecture::flush_remote_caches`), proven by the ghidra-sim
      label-override cache-clearing test.
- [x] ghidra-mode defaults: the `aggressive` engine-tier preset at registerProgram +
      `FUN_`/`DAT_`/`LAB_` fallback naming (DIV-77); `setOptions` decodes and applies
      for real with per-element skip-unknown (DIV-76).

**Phase 4 — the full response encode (branch `feat/ghidra-phase4-encode`).**
- [x] `Datatype::encodeRef` port (`substrate/dtype.rs` `encode_ref`/`encode`/
      `encode_basic`/`encode_typedef` + `TypeField`/`TypeBitField` encode) —
      the cross-cutting type marshal-out every other element needed.
- [x] `<prototype>` (`FuncProto::encode`, `p4_calls/fspec.rs`): model +
      extrapop (+"unknown"), the boolean flags, `<returnsym>` (sized storage +
      type), effect/likely-trash model-diffs.  Input params travel as localdb
      cat-0 symbols (the upstream symbol-backed store shape).
- [x] `<localdb>` (`ScopeLocal::encode`, `varmap.rs` →
      `Database::encode_scope`/`Symbol::encode`/`SymbolEntry::encode`,
      `p0_knowledge/database.rs`): nonzero ids everywhere, ≥1 entry per
      mapsym, positional `<parent>`+`<rangelist>`, params cat=0+index+exact
      storage.
- [x] `<highlist>` (`Funcdata::encode_high`, `substrate/funcdata_encode.rs`)
      + the encode-time symbol-link pass (`kuna_link_high_symbols` — the
      C++ `ActionNameVars::linkSymbols` stand-in) attaching/creating localmap
      symbols for named highs so `<high symref>` resolves in the just-encoded
      `<localdb>`; global `<high>`s echo the REAL host DB id
      (`GlobalEntry::symbol_id` from the getMappedSymbols record) and omit
      `symref` rather than fabricate one.
- [x] `<jumptablelist>` (the ported `JumpTable::encode` wired into
      `Funcdata::encode`, emitted independently of savetree) + the jumpload
      toggle plumbed to `FlowInfo::record_jumploads` per decompile — the
      switch-analyzer contract (noc+notree+jumpload), loadtables included.
- [x] `<parammeasures>` (`ParamIDAnalysis::encode`, `infra/paramid.rs`,
      `<rank>` always on) + the `paramid`-action doc shape (parammeasures is
      the ONLY doc child) — the param-ID / convention analyzers.
- [x] Markup type-token fidelity: `EmitMarkup::tag_type` splits a rendered
      declarator into base-word `<type>` tokens + `<syntax>` separators, so
      Java's `getC()` (`IllegalCharCppTransformer`) no longer mangles
      `unsigned long *` to `unsigned_long__` (ghidra-sim mangled pins → 0).
- [x] The rename/retype PERSISTENCE loop: the function `<localdb>` answer's
      non-parameter locals (what Java sends after
      `HighFunctionDBUtil.updateDBVariable` committed a user edit) seed the
      fresh Funcdata — typelocked locals as mapped/usepoint symbols, plain
      renames (typelock=false) as `ScopeLocal::nameRecommend` records (the
      C++ mechanism; applied by the `ActionNameVars` port) — so a GUI
      rename/retype SURVIVES the event-driven re-decompile.
- [x] Every `<vardecl symref>` in the corpus resolves against its own
      `<localdb>` — `PIN_FAILLOG_VARDECL_UNRESOLVED = [0,0,0]`, plus a
      per-target 0 on the sort/grep breadth fixtures, so declaration-line
      rename/retype is live everywhere.  The last residue was a stack aggregate
      reached only through `&sym`, whose whole HighVariable is the constant
      `PTRSUB` operand: `link_symbol_reference` now records the referenced
      Symbol's identity (C++ `Varnode::setSymbolReference`).
- [x] Harness: the ghidra-sim decode now validates every Java hard-throw trap
      (r5 §3 — nonzero symbol ids, entry pairs, positional scope children,
      localdb/ast-before-highlist order, symref/repref resolution, prototype
      completeness, `<rank>` presence) plus switch-analyzer- and
      paramid-shaped configuration tests.

**Phase-4 deferred (follow-ups, graceful degradation per §10):**
- [ ] `structureGraph` (FunctionGraph nested layout only).
- [ ] The four signature commands (engine already ported) for BSim.
- [ ] Overlay spaces (Java swaps in overlay codecs transparently —
      `DecompInterface.java:84-127,896-909`); `getStringData` charset fidelity
      (Java-side decode instead of `GhidraLoadImage` bytes).
- [ ] `<override>` / child-`<scope>` statics (Java skips both); the C++
      `collectNameRecs` harvest (standalone symbols → recommendations).
- [ ] `kuna_apply_dynamic_recommendations` runs BEFORE the naming walk rather
      than after linking (kuna fuses the two upstream stages into one pass), so
      it creates a dynamic Symbol where upstream renames an existing one.  The
      upstream guards are reproduced against the scope instead of the high —
      see `docs/spec/06-variables-and-merge.md` §6.4.

# What Ghidra requires from a replacement decompiler core

Ghidra's GUI does not contain a decompiler. It spawns a child process (the "decompiler
core", upstream binary name `decompile`), drives it over stdin/stdout, answers its
questions about the program mid-decompile, and renders the typed documents it returns.
This document specifies that interface completely, from the host's (Java's) side: what
it takes to substitute **any** decompiler core — not just kuna — behind an untouched
Ghidra GUI. No kuna knowledge is assumed; kuna's own plan is
`docs/ghidra-integration.md`.

**Citation convention.** All citations are `file:line` against Ghidra 12.2-DEV at
commit `f9e13846` (2026-06-16). Java paths are under
`Ghidra/Features/Decompiler/src/main/java/ghidra/app/decompiler/` unless prefixed;
C++ paths (the upstream reference implementation of the native side) under
`Ghidra/Features/Decompiler/src/decompile/cpp/`. `Framework/...` paths are under
`Ghidra/Framework/`. The Java side is the **normative** contract — the C++ side is
cited where it is the only precise statement of native-side behavior.

---

## 1. The shape of the interface

One session = one spawned process per open Program. The host sends *commands*; while
servicing a command the core may send *callback queries* (each answered by the host
before the command continues); the command ends with a *response*. Everything — specs,
addresses, symbols, p-code, types, the decompiled output — travels through this one
pipe. There is no shared memory, no filesystem contract, no network.

The interface has four layers, specified bottom-up in §3–§6:

1. **Burst framing** — 4-byte markers delimiting streams on the raw pipe.
2. **Packed document encoding** — a nonzero-byte binary element/attribute format
   carried inside string streams.
3. **Command / query choreography** — who may talk when, and the exception channels.
4. **Document schemas** — what the host sends, what it answers, and what it demands
   back (the response contracts, §8).

## 2. Getting your binary spawned (the substitution seams)

The spawn path is hardcoded and seamless-by-design: `DecompInterface` (the only
consumer, instantiated with `new` at 29 call sites) calls the static
`DecompileProcessFactory.get()` (`DecompInterface.java:267`), which resolves the
executable named `decompile` (`decompile.exe` on Windows) **once** via
`Application.getOSFile(exeName)` and caches the absolute path in a private static
field with an early-return guard (`DecompileProcessFactory.java:28,52-55`). No system
property, environment variable, ExtensionPoint, or service can redirect it (grep
confirms zero config reads in the package; the only wrapper precedent upstream is a
commented-out valgrind line, `DecompileProcess.java:98`).

Substitution options, ranked:

1. **Reflection from an extension plugin (recommended).** The process spawns *lazily*
   on the first decompile, so any plugin constructor in the tool runs before it. Set
   the private static `exepath` by reflection; `getExePath()` early-returns forever
   after (`DecompileProcessFactory.java:52-55`). Ghidra runs a flat classpath — every
   class in the unnamed module (`Framework/Utility/.../GhidraClassLoader.java:34`) — so
   `setAccessible(true)` needs no `--add-opens`. Ship your binary inside your
   extension's own `os/<platform>/` under a **distinct filename** and resolve it with
   the two-arg `Application.getOSFile(moduleName, filename)`
   (`Framework/Generic/.../Application.java:1000-1003`). Covers all 29 consumers,
   survives Ghidra upgrades that don't touch the 76-line factory.
2. **File drop at `build/os/` (release installs, zero code).**
   `getModuleOSFile` checks `build/os/<platform>/` *before* `os/<platform>/`
   unconditionally, so a binary at
   `<install>/Ghidra/Features/Decompiler/build/os/<platform>/decompile` shadows the
   stock one without deleting it. A distribution patch, not an extension.
3. **Patch-directory class shadow (release installs only).** Jars in
   `<install>/Ghidra/patch` precede module jars on the release classpath
   (`Framework/Utility/.../GhidraLauncher.java:182-183`); ship a rebuilt
   `DecompileProcessFactory` that consults a property. Version-coupled to the exact
   release; the dev-mode classpath never adds the patch dir.

**Why an extension shipping `os/<platform>/decompile` can never shadow the stock
binary**: the single-arg `Application.getOSFile(exactFilename)` searches the *calling
class's own module first* and only falls back to an all-module scan
(`Application.java:1013-1026`). The caller is `DecompileProcessFactory`, which lives in
the Features/Decompiler module — and that module ships its own `decompile`, so the
short-circuit always wins and the scan (where extensions would be visible) is never
reached. Extension *jars* cannot class-shadow either: module classpath ordering puts
Features before Extensions on the flat classloader.

## 3. The process contract

- **Spawn**: `runtime.exec(exepath)` — **zero arguments, inherited environment,
  nothing else** (`DecompileProcess.java:151`). The process cannot learn anything
  (language, program, paths) from its invocation; everything arrives on stdin.
- **Liveness**: the host waits 200 ms, then requires `isAlive()`
  (`DecompileProcess.java:154-167`). A process that exits during startup gets its
  stderr shown in an error dialog once (`:168-192`). **Do not print anything to
  stdout at startup** — there is no greeting, no handshake; the first bytes on the
  pipe are the host's first command.
- **Windows**: stdin/stdout must be put in binary mode
  (`_setmode(_fileno(stdin), _O_BINARY)`, `ghidra_process.cc:524-528` in the reference
  implementation) — the protocol is 8-bit-clean binary.
- **Exit rules**: exit(1) immediately on EOF/pipe-close anywhere (the host disposed
  you; a lingering process is a runaway — `ghidra_arch.cc:95-96`). Install a SIGSEGV
  handler that `_Exit(1)`s (`ghidra_process.cc:510-527`) so a crash surfaces as a dead
  process, not a hang. Exit normally after answering `deregisterProgram` (§5) — the
  host never reuses a deregistered process (`DecompileProcess.java:501-522`).
- **Timeout is murder, not a message**: on decompile timeout the host disposes the
  process from a timer thread so its own blocked read throws
  (`DecompileProcess.java:100-106,564-595`). The core sees only EOF. A user cancel is
  the same. `timeoutSecs=0` disables the timer (`:597-602`).
- **Respawn + replay**: after any I/O failure the host respawns and *replays* the
  session — `registerProgram`, then `setOptions`, `setAction`, the print toggles, and
  signature settings (`DecompInterface.java:262-361`). The core needs no persistence;
  it must merely reach equivalent state from the replay.
- **Threading**: the host serializes all commands (synchronized senders); the core can
  be single-threaded over the pipe.

## 4. The burst framing

Every protocol marker ("burst") is written as the 4 bytes `{0x00, 0x00, 0x01, code}`
and must be *read* tolerantly: skip any nonzero garbage, then one-or-more `0x00`
bytes, then expect `0x01`, then the code byte (`DecompileProcess.java:196-228`;
`ghidra_arch.cc:79-98`). Both sides' scanners are self-aligning in this way. Codes are
paired even/odd = open/close, and the host's reader skips stray odd (close) codes
while hunting a response (`DecompileProcess.java:230-244`).

| pair | meaning | direction |
|---|---|---|
| 2 / 3 | command | host → core |
| 4 / 5 | callback query | core → host |
| 6 / 7 | command response | core → host |
| 8 / 9 | query response | host → core |
| 10 / 11 | exception | both |
| 12 / 13 | byte-stream payload | both |
| 14 / 15 | string-stream payload | both |
| 16 / 17 | native message (warnings) | core → host |

A *string stream* (14/15) carries either raw ASCII text (command names, decimal ids,
`t`/`f` answers) or a packed document (§6). A *byte stream* (12/13) carries the
nibble-doubled binary formats of `getBytes`/`getStringData` (§7). Payload bytes
between 14…15 must never contain `0x00` — that is the packed encoding's job to
guarantee (§6).

**Size limits** (host-enforced): a query parameter from the core ≤ **64 KiB**
(`DecompileProcess.java:272`); the main command response ≤ **50 MiB** default
(option-settable, `:80,414`); a native message ≤ **1 MiB** (`:428`).

## 5. Command choreography

**Dispatch loop** (reference: `ghidra_process.cc:464-486`): scan bursts until a 2;
read the command name as a string stream; look it up. Unknown command → emit a
complete, *payload-less* response: `6, 16, "Bad command: <name>", 17, 7`, flush, keep
looping (`:476-484`) — this exact shape is what makes partial cores degrade gracefully
(§9).

**Per-command lifecycle** (reference: `GhidraCommand::doit`, `ghidra_process.cc:125-160`):

1. Write the response-open burst **6 first, before reading parameters** — this is why
   callback queries are legal mid-command: they nest inside the open response.
2. Read parameters. Every command's first parameter is the session's **archid** as
   ASCII decimal inside a string stream — *except* `registerProgram`, which has no
   archid (it creates one) (`ghidra_process.cc:86-103,162-173`).
3. Consume the command-close burst 3; anything else is an alignment error.
4. Do the work (queries may flow here, for the commands that allow them — see table).
5. Write the result payload (see table), then **always** the message frame:
   `16, <accumulated warning text, possibly empty>, 17` (`ghidra_process.cc:108-116`),
   then 7, then flush.

Internal errors during step 4 must not kill the response: the reference implementation
converts recoverable exceptions into warning text and still runs step 5 with an
empty/absent payload (`ghidra_process.cc:137-155`). If a 16 arrives while the host's
14/15 payload is still open, the host *abandons the partial payload* and keeps the
message (`DecompileProcess.java:423-429`) — the designed salvage path for an exception
thrown mid-encode. Hard protocol errors instead send the exception frame
`10, <type string>, <message string>, 11` **in place of** the result
(`ghidra_arch.cc:241-248`; `ghidra_process.cc:142-145`); the type `"alignment"` is
special-cased by the host into an IOException.

**The command set** (host senders: `DecompileProcess.java:464-700`; the four styles of
result payload are: decimal string, `t`/`f` string, packed document, or nothing):

| command | params after archid | result payload | callback queries legal? |
|---|---|---|---|
| `registerProgram` | *(no archid)* 4 XML string streams: pspec, cspec, tspec, coretypes | new archid, ASCII decimal | **yes** |
| `deregisterProgram` | — | `1` (or `0` if nothing registered); then the process must exit | no |
| `flushNative` | — | `0` | no |
| `decompileAt` | packed `<addr>` (function entry) | packed `<doc>` (§8); **empty 14/15** if the function did not complete | **yes** |
| `structureGraph` | packed block graph | packed restructured block graph | **yes** |
| `setAction` | 2 string streams: action name (`decompile`/`normalize`/`jumptable`/`paramid`/`register`/`firstpass` or ""), print config (`tree`/`notree`/`c`/`noc`/`parammeasures`/`noparammeasures`/`jumpload`/`nojumpload` or "") | `t` / `f` | no |
| `setOptions` | packed `<optionslist>` | `t` / `f` | no |
| `generateSignatures` | packed `<addr>` | packed `<signatures>` | **yes** |
| `debugSignatures` | packed `<addr>` | packed debug-signature doc | **yes** |
| `getSignatureSettings` | — | packed `<sigsettings><major/><minor/><settings/>` | no |
| `setSignatureSettings` | decimal settings string | `t` / `f` | no |

**The archid session model**: `registerProgram` allocates a slot in a global session
list and answers its index; every later command names it. Slots are nulled by
`deregisterProgram` and may be reused (`ghidra_process.cc:176-210,231-251`). In
practice the host runs one program per process, but the protocol is multi-session.
An invalid archid is a hard exception (`"No architecture registered with decompiler"`).

**The query legality window is a hard constraint**: the host services callback queries
only during `registerProgram` and the four timed commands (`decompileAt`,
`structureGraph`, `generateSignatures`, `debugSignatures`) — for every other command it
nulls its query decoder/encoder (`DecompileProcess.java:472-473,571-572` vs
`:512-513,536-537,618-619,657-658,686-687`). A core that lazily queries during
`setOptions` desyncs the protocol unrecoverably.

**Query choreography** (core side): write `4, 14, <packed command element>, 15, 5`,
flush; then read either `8, [one optional payload], 9` — or `10, <exception type>,
<message>, 11`, which means the host's callback threw; the core must abort the current
command and propagate the exception back out as its own 10/11 frame
(`ghidra_arch.cc:192-232`; host side `DecompileProcess.java:382-397`). An **empty
response** (8 immediately followed by 9) is the in-band "not found / no data" signal —
each query's row in §7 says what that means for it.

**Session lifecycle the host actually drives** (`DecompInterface.java:262-361,774-839`):
`registerProgram` → `setOptions` → `setAction` + toggles (only non-defaults) →
[`setSignatureSettings`] → *n* × (`decompileAt` → `flushNative`) → `deregisterProgram`.
`flushNative` after **every** decompiled function is the cache-coherence mechanism: the
core must drop every program fact it cached — symbols, sub-scopes, non-core types,
comments, string decodings, constant-pool records (`ghidra_process.cc:262-273`) —
because the user may have renamed/retyped anything between requests. Caches live
*within* one decompilation only.

## 6. The packed document encoding

All non-XML payloads are "packed" documents — a binary element/attribute serialization
whose defining property is that **every encoded byte is nonzero**
(`Framework/SoftwareModeling/.../pcode/PackedDecode.java:24-51`; `cpp/marshal.hh:454-504`),
which is what lets it sit inside `0x00`-delimited string streams: the host ingests
payload bytes until the first `0x00` (the opening zero of the closing burst).

- **Header byte**: top two bits — `0x40` element-start, `0x80` element-end, `0xC0`
  attribute; bit `0x20` = extend (one more byte `1iiiiiii` with 7 more id bits → 12-bit
  ids); low 5 bits = first id bits.
- **Attribute record**: header, then a type byte `ttttllll` — types 1=boolean
  (length nibble is the value), 2=signed-positive, 3=signed-negative (magnitude),
  4=unsigned, 5=address-space (integer = space index), 6=special-space (length nibble:
  0=stack, 1=join, 2=fspec, 3=iop, 4=spacebase), 7=string (an integer byte-length,
  then raw UTF-8).
- **Integers**: big-endian, 7 bits per byte, each byte OR'd with `0x80`; the length
  nibble is the byte count; length 0 encodes value 0.
- **Indexed attributes** (join pieces): the index is folded into the attribute id
  itself (`attribId + index` — `PackedEncode.java:169-174` = `marshal.cc:1188-1195`);
  decoders compare raw ids against the base. The host **requires join pieces in
  order** (`Varnode.java:550-556`, the `"piece" attributes must be in order`
  `DecoderException`).
- **The space-index binding — the deepest interop invariant**: the wire integer for an
  address space is the host's `AddressSpace.getUnique()` index, and the core learns
  the same numbering from the tspec, which writes each space's `index` attribute from
  `getUnique()` (`SleighLanguage.encodeTranslator`,
  `Framework/SoftwareModeling/.../sleigh/SleighLanguage.java`). Indices may be
  **sparse** — the core's space table must tolerate holes. Of the special codes, the
  host's decoder accepts only stack and join, maps spacebase to null, and rejects
  fspec/iop (`PackedDecode.readSpace`) — never encode fspec/iop varnodes into response
  documents.
- Element ids run 1–291 upstream; the protocol-specific ones a core needs: `doc`=229,
  the 19 query commands 239–257 (§7), and `major`=261/`minor`=262/`settings`=264/
  `sigsettings`=268 (`ElementId.java:371-440`; `ghidra_arch.cc:30-48`,
  `ghidra_process.cc:74`, `cpp/signature.cc:29-40`).

## 7. What `registerProgram` delivers — and the query catalog (the full data API)

`registerProgram` sends exactly four XML documents (the only XML on the wire), in
order: **pspec** (the raw `.pspec` file text with all newlines stripped —
`DecompInterface.java:241-254`; safe for all shipped specs), **cspec** (the compiler
spec, re-encoded single-line, *including the p-code snippet sources for call fixups
and injects*), **tspec** (a `<sleigh>` element: endianness, `uniqbase`, and the
address-space list with names/indices/sizes/wordsizes — **not** a `.sla` file), and
**coretypes** (`DecompileProcess.java:479-485`; `DecompInterface.java:262-302`).

What it does **not** deliver: a language id, processor context, memory contents,
symbols, or any disassembly capability. **P-code-from-host is therefore structurally
implied**: with no language identity and no context-register model on the wire, a core
cannot correctly disassemble raw bytes itself (ARM/Thumb, MIPS16, user overrides); the
listing the *host* disassembled is fetched instruction-by-instruction via `getPcode`.
The reference core contains no SLEIGH engine at all (the `ghidra_opt` link excludes
it, `cpp/Makefile:99-131`).

The 19 callback queries are the complete data API a core ever gets. Framing per §5;
"empty ⇒" describes the empty-8/9-response meaning. Two queries (`getComments`,
`getTrackedRegisters`) **always** return a document, even when empty
(`DecompileProcess.java:727-734,844-853`); for all others an empty response is the
negative answer.

| query | id | request params | response | notes |
|---|---|---|---|---|
| `isNameUsed` | 239 | `name`, `first`/`last` scope ids | raw `t`/`f` byte in 14/15 | host caps the scan at 16 symbols and reports "used" past it |
| `getBytes` | 240 | `<addr>` + `size` | 12/13 byte stream, each byte nibble-doubled: `('A'+(b>>4)), ('A'+(b&0xF))` | empty ⇒ no backing memory (DataUnavail) |
| `getCallFixup` | 241 | `name` + `<context>` (baseaddr, calladdr, in/out varnodes) | packed `<inst>` of p-code ops | inject bodies live host-side; core sends context, gets ops |
| `getCallMech` | 242 | `name` + `<context>` | packed `<inst>` | call-mechanism (uponentry/uponreturn) injects |
| `getCallOtherFixup` | 243 | `name` + `<context>` | packed `<inst>` | CALLOTHER fixups |
| `getCodeLabel` | 244 | `<addr>` | plain string | "" = no label |
| `getComments` | 245 | `type` flag mask + `<addr>` (function entry) | packed `<commentdb>` | **always written**; flags: 1=EOL, 2=PRE, 4=POST, 8=PLATE/header |
| `getCPoolRef` | 246 | `size` n + n × `<value>` | packed `<cpoolrec>` | JVM/Dalvik constant pool |
| `getDataType` | 247 | `name` + `id` (signed) | packed `<type>` | empty ⇒ unknown type |
| `getExternalRef` | 248 | `<addr>` | packed `<doc id><mapsym>` | external/thunk resolution |
| `getMappedSymbols` | 249 | `<addr>` | packed `<doc id><mapsym>` (function/data/label/external) or `<hole [readonly] [volatile] space first last/>` | THE symbol query; `<hole>` = cache-this-negative-range; empty ⇒ no address |
| `getNamespacePath` | 250 | `id` (unsigned) | packed `<parent>` of `<val>` per namespace level | lazily rebuilds scope paths |
| `getPcode` | 251 | `<addr>` | packed `<inst offset=len><addr/>` + `<op>` per p-code op, or `<unimpl offset=len/>` | **one query per instruction**; empty ⇒ undecodable (BadData) — the decompile *continues*, counting toward the bad-data limit |
| `getPcodeExecutable` | 252 | `name` + `<context>` | packed `<inst>` | executable p-code scripts (e.g. segment resolution) |
| `getRegister` | 253 | `name` | packed `<addr space offset size>` | unknown name ⇒ host throws (exception frame) |
| `getRegisterName` | 254 | `<addr>` + size | plain string | "" = not a register |
| `getStringData` | 255 | `maxsize`, `type` name, `id` + `<addr>` | 12/13 byte stream: 2 bytes encoding sz=len+1 (`byte1=(sz&0x3f)+0x20`, `byte2=((sz>>6)&0x3f)+0x20`), 1 raw truncated-flag byte, then sz nibble-doubled UTF-8 bytes ending in the doubled NUL `"AA"` | host decodes/validates the string in *its* charset settings; empty ⇒ not a string |
| `getTrackedRegisters` | 256 | `<addr>` | packed `<tracked_pointset>` of known register values | **always written** |
| `getUserOpName` | 257 | `index` (signed) | plain string | "" = end; the core probes 0,1,2,… at registration to build the CALLOTHER table |

(Verified against both ends: `ghidra_arch.cc:30-48,445-896` and
`ElementId.java:371-427` + the dispatch at `DecompileProcess.java:315-380,704-916`.
Note 241=`getcallfixup`, 242=`getcallmech`, 243=`getcallotherfixup`.)

Exceptions from a query (frame 10/11 instead of 8/9) abort the current command; the
mid-command query that failed is never retried within it.

## 8. The response document contracts

The wire will move any well-formed packed document; the *decoders* are the real
contract. What each response must contain:

**`decompileAt` → `<doc>` (element id 229).** Decoded by
`DecompileResults.decodeStream` (`DecompileResults.java:215-264`), which accepts, in
order:

1. A first `<function>` → decoded as the **HighFunction**
   (`Framework/SoftwareModeling/.../pcode/HighFunction.java:245-293`). Requirements:
   - `name` attribute **equal to the host-side function name** (mismatch throws);
     an `<addr>` child matching the entry point (mismatch throws).
   - Recognized children — all individually optional: `<prototype>`, `<localdb>`
     (the local symbol scope: `<mapsym>` entries), `<ast>` (varnodes + p-code ops),
     `<highlist>` (HighVariables), `<jumptablelist>`, `<override>`/`<scope>`
     (skipped). **Any other child throws** — emit nothing extra.
   - **Symbol-id echo**: `<mapsym>` symbols must carry the host database's real symbol
     ids as delivered by `getMappedSymbols`; ids in the internal
     `0x4000000000000000` range are reserved and never round-tripped
     (`HighSymbol.java:39,386-387`). Rename/retype in the GUI is a database write
     keyed by this id followed by a re-decompile — a wrong id silently breaks it.
2. A second `<function>` → the **token markup** (the rendered C as a Clang token
   tree, `ClangMarkup.buildClangTree`). Every token needs `color` (syntax class) and
   text content; variable/op/statement tokens carry `varref`/`opref` ids that the host
   resolves **against the first `<function>`'s decoded `<ast>`**
   (`ClangVariableToken.java:147-163`;
   `Framework/SoftwareModeling/.../pcode/PcodeSyntaxTree.java:309,365`). Click-to-address,
   hover, highlight, and rename targeting all ride these ids — markup and ast must be
   emitted from the same numbering.
3. Alternatively `<parammeasures>` (action `paramid`) → `HighParamID`.

Anything else at `<doc>` level is "Unknown decompiler tag". The ordering kludge is
explicit and load-bearing: HighFunction first, markup second. `decompileCompleted()`
requires hfunc *or* hparamid; the **GUI panel** additionally requires the markup
(`component/DecompileData.java:56`) — a core that omits the second `<function>` keeps
scripts and analyzers alive but renders nothing. An **empty** result payload (allowed
for a failed function) plus warning text in the 16/17 frame produces a clean GUI error.

**`structureGraph`** → the input is a packed block graph (nested `<block>`/`<bhead>`/
`<edge>`); the output is the structured graph, decoded by `BlockGraph.decode` and
re-associated with the caller's objects (`DecompInterface.java:732-764`). Pure
control-flow structuring — no dataflow, no program state; always uses the non-overlay
codec set.

**`generateSignatures`** → `<signatures>` with `<sig val=…/>` children (+ `unimpl`/
`baddata` attributes), decoded by `SignatureResult.decode`. `getSignatureSettings` →
`<sigsettings>` with `<major>`, `<minor>`, `<settings>` integer children.

**Overlay spaces**: for functions in overlay spaces the host transparently swaps in
overlay-translating codecs on *both* the command and callback channels
(`DecompInterface.java:84-127,896-909`); the core just sees the underlying space via
the translated index, plus `<space_overlay>` entries in the tspec.

## 9. Consumer matrix — what breaks when a core is partial

| consumer | what it sends / expects | if missing |
|---|---|---|
| **Decompiler panel** (the GUI) | `decompileAt`, style `decompile`, tree+C on; needs *both* `<function>` elements with consistent refids | no render (clean error) |
| **Rename/retype actions** | HighFunction symbols with real DB ids; re-decompile picks up changes via fresh queries | renames silently fail to stick |
| **Switch analyzer** (auto-analysis) | `decompileAt` with C *off*, `jumpload` *on*; consumes `<jumptablelist>` | computed jumps stay unrecovered in the listing |
| **Parameter-ID / call-convention analyzers** | action `paramid` + `parammeasures`; consume `<parammeasures>` | those analyzers no-op |
| **BSim** | `generateSignatures`, `debugSignatures`, `get`/`setSignatureSettings` | disabled with a friendly "not built with signature module" message (`DecompInterface.java:341-347`) |
| **FunctionGraph nested layout** | `structureGraph` | that one layout unavailable |
| **CppExporter, scripts, FlatDecompilerAPI, Debugger unwind** | `decompileAt`; HighFunction and/or the pretty-printed text | per-feature |

The graceful-degradation mechanism is uniform: answer unimplemented commands with the
exact "Bad command" response shape (§5) and the corresponding feature degrades in
isolation; everything else keeps working.

## 10. Versioning reality

There is **no handshake**. The interface version (major=6, minor=1,
`cpp/architecture.cc:35-36`) is exposed only through `getSignatureSettings`, fetched
lazily, and consumed only by BSim (`Features/BSim/.../FunctionDatabase.java:213-214`).
Compatibility in practice rests on shipping the core with the matching Ghidra release.

The concrete skew failure mode is **option drift**: `setOptions` sends an
`<optionslist>` whose elements are only emitted when they differ from the Java-side
defaults. When Ghidra adds an option (real example: `baddatacount`,
ELEM_BADDATACOUNT=290, added between 2026-06-01 and the 12.2-DEV head —
`ElementId.java:293`, `DecompileOptions.java:988-989`), an older reference core throws
`ParseError("Unknown option")`, `setOptions` answers `f`, and the host fails the whole
program-open with "Did not accept decompiler options" (`DecompInterface.java:301-303`)
— one stale non-default option bricks the decompiler view.

**Recommendation for any replacement core**: pin the Ghidra release you target, and —
diverging from the reference implementation — *tolerate unknown option elements*
(skip + warn) so option skew degrades to a warning instead of a hard failure.
Element-id renumbering across releases is the other watch item: protocol ids must
match the pinned release exactly.

## 11. Minimum viable core

The smallest surface that yields working GUI decompilation:

- **Commands**: `registerProgram` (parse the tspec's space list — sparse indices — and
  hold the cspec/pspec/coretypes; answer an archid), `setOptions`/`setAction` (may
  accept-and-ignore; answer `t`), `decompileAt`, `flushNative` (clear caches; answer
  `0`), `deregisterProgram` (answer `1`; exit). Answer everything else "Bad command".
- **Queries issued**: `getPcode` (per instruction — the instruction stream *is* this),
  `getMappedSymbols` at the entry (the function's name/extent; honor `<hole>`
  semantics for anything else you probe), `getRegister`/`getRegisterName` as your IR
  needs them, `getUserOpName` 0…n at registration, `getTrackedRegisters` at the entry.
  Everything else — bytes, strings, types, comments, injects, cpool — is quality, not
  viability (skipping call fixups will visibly hurt output on real binaries; add
  `getCallFixup`/`getCallMech` next).
- **Response**: a `<doc>` with the dual `<function>` contract of §8 — matching
  name/entry, a decodable `<ast>`, `<localdb>` echoing the delivered symbol ids, and a
  markup tree whose `varref`/`opref` ids resolve into that ast.
- **Behavior**: the exact framing choreography of §5 (response-open before params, the
  always-present 16/17 frame, exception frames), `exit(1)` on EOF, and full state
  reconstruction from the replay sequence after a respawn.

Everything beyond that — jump tables, param measures, structureGraph, signatures,
overlay spaces, charset-faithful strings — is incremental parity, feature by feature,
per the consumer matrix.

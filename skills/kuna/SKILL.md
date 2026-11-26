---
name: kuna-decompiler
description: Use the Kuna CLI to recover searchable C and assembly, trace program logic, compare functions with BSim, and identify library code with FunctionID. Covers using Kuna, not developing the decompiler.
---

# Kuna for reversing binaries

Kuna decompiles locally through a CLI on PATH, not an MCP server. Decompilation needs
no connection and saves no analysis state between invocations.
In a checkout, the built CLI is `decompiler/target/release/kuna`.

Decompile once for the chosen scope, then reuse the output with `rg`, file reads, and
byte offsets. These examples focus on CTF reversing.

## Choose the scope, then decompile

Default to exporting the entire binary to disk:

```bash
kuna decompile-project ./challenge --stream --jobs auto -o ./challenge.kuna
```

After initial loading, `--stream` makes completed functions readable while the export
continues. Start reading then; the scheduler prioritizes the entry point, `main`, and
their callees when discovered, but parallel completion can interleave other functions.

`--jobs auto` sizes parallel processing using CPU and memory availability. Use an
explicit count when resources are known, or `--jobs 1` for small workloads where
worker startup outweighs the benefit. A sub-1 MB file is only a rough size guide.

When scale is unknown, `kuna functions ./challenge --summary --json` reports counts,
entry point, reachability, and code bytes. It still loads and analyzes the image, so
skip this extra call if scope is clear. Its `summary.runtime` array names a PyInstaller
or Nuitka bundle, a .NET assembly, a UPX stub, AutoIt, VB6 or twinBASIC with the tool to
use instead; other subcommands print the same hint as a `note:` line on stderr.

For a few dozen short functions, `kuna decompile-all ./challenge` is manageable to
read directly; use a project when assembly or repeated navigation is needed.

Select known targets before decompiling: project export accepts `--functions name1,name2`
or repeatable `--addr 0x401000`. `--filter REGEX` and `--reachable-from main` are only
for `functions` and `decompile-all`. Static reachability can miss runtime dispatch.

Use `decompile` for a function needed immediately, a failed function, or an option comparison:

```bash
kuna decompile ./challenge main
kuna decompile ./challenge 0x401230 --addr --json
```

Batch multiple functions instead of looping over `decompile`. Use `decompile-all --json`
for structured batch output; project export does not accept `--json`. When only the code
is needed, `--omit aliases,ranges,line_mappings,variables,types` on either `--json` drops those
collections (named in the document's `omitted`), keeping it near the size of the C.

## Navigate the export

| Artifact | How to use it |
|---|---|
| `index.jsonl` | Search this streaming index first: `name`, `addr`, `size`, `error`, and the C block's byte span, `c_offset`/`c_len`. |
| `<name>.c` | Function bodies under `// Function: <name> @ <addr>` headers. |
| `<name>.h` | Prototypes and recompile prelude; recovered type definitions arrive at finalization. |
| `<name>.asm` | Disassembly and data bytes; often much larger than the C. Search by symbol or address. |
| `README.md` | Image details, progress, and success/failure counts. |
| `.streaming` | Live status and operational failures while exporting. |

Read only newline-terminated index records: each announces a complete C block. Defer
an unfinished last record until the next read. Streamed C follows completion order,
so locate functions by the index rather than position.

Check per-function errors even after a successful exit or removal of `.streaming`.
Retry relevant failures individually; adjust `--max-fn-seconds` for budget overruns.

Keep notes, solver scripts, symbols, addresses, and useful settings outside generated
artifacts, which another export can replace. Summarize findings instead of pasting
large C or assembly regions.

## Use the embedded documentation

`kuna docs` lists topics and sizes. Read or search only the relevant topic: `cli` for
syntax, supported combinations, exit codes, and JSON schemas; `modes` for presets and
size thresholds; `options` when tuning as below. Use `kuna --help` or
`kuna <subcommand> --help` for quick syntax checks. Avoid loading `kuna docs --all`
or the full option catalog into context for routine analysis.

## Find similar functions with BSim

Use BSim to find candidates across builds. Query a specific symbol (`--name`) or
loaded address (`--address`). Read both `similarity` and `significance`: even a
similarity of 1.0 does not prove identity. A `truncated` result is not exhaustive.
Record the reference build and function address. Distinguish exact instruction-byte
matches, matches after accounting for known relocations, and similarity candidates.
Review candidates before renaming. `bsim query` does not accept `--raw-image`;
use FID for raw-image candidate lookup.

```bash
kuna bsim build ./known-build -o ./known.kbsm
kuna bsim info ./known.kbsm --json
kuna bsim query ./target --corpus ./known.kbsm --address 0x401230 --json
kuna bsim compare ./target --address 0x401230 \
  --reference ./known-build --reference-address 0x401000 --json
```

For an existing native Kuna server, set `BSIM_URL` to its URL and take `MATCH_ID`
from the query. The query sends feature hashes, not the executable. Compare downloads
and verifies the original reference bytes, then decompiles both sides locally.

```bash
kuna bsim query ./target --server "$BSIM_URL" --address 0x401230 --json
kuna bsim compare ./target --server "$BSIM_URL" \
  --address 0x401230 --match-id "$MATCH_ID" --json
```

To populate an authorized native store, import binaries directly or import a closed
or consistently copied Ghidra 12.1.2 snapshot (`.gpr` with its `.rep`, `.rep` alone,
or `.gzf`). The native server does not require Ghidra or Java.
For an older Program DB, upgrade and save a copy in 12.1.2. Project import
keeps saved names for supported linked images and backed relocatable ELF sections,
but skips default names and placeholders such as `FUN_00401000`.

```bash
kuna-bsim-server import ./module.o ./app.exe --store ./library
kuna-bsim-server import-ghidra ./saved.gpr --store ./library
kuna-bsim-server --store ./library --bind 127.0.0.1:8787
```

To publish one reviewed name, run `kuna bsim push` in the client shell. It uploads
the whole executable; use it only with permission and a separate
`KUNA_BSIM_SERVER_WRITE_TOKEN` on the client and server. See `kuna docs cli` for
read tokens and TLS.

```bash
kuna bsim push ./target --server "$BSIM_URL" \
  --address 0x401230 --rename parse_record --json
```

For stock PostgreSQL BSim, configure `KUNA_BSIM_POSTGRES` through a credential
manager and replace `--server "$BSIM_URL"` on **query** with `--postgres`. For
**compare**, use the returned `--match-id` with a local `--reference` instead of
`--reference-address`. Kuna checks its MD5; it does not fetch from Ghidra Server.
If compare returns `diff: null`, check both `code` results before drawing conclusions.

## Identify library functions with FunctionID

Use FID to propose names from known library fingerprints, not to rank similar code.
Query loaded addresses in a linked or raw image without renaming. Repeat `--addr`
for a batch. Relocatable objects are database inputs, not lookup targets.

```bash
kuna fid build ./lib.o -o ./lib.fid --lang x86:LE:64:default --cspec gcc
kuna fid query ./target --db ./lib.fid --addr 0x401230 --addr 0x4012a0 --json > fid.json
kuna_fid_db="$PWD/lib.fid" kuna functions ./target --option listing on --option fid on --json
```

Use the library's actual language and compiler spec when building. Query and automatic
naming also accept stock Ghidra `.fidbf` and `.fidb` files. Query verified entries in
batches. Keep candidates grouped by address, and count each status separately:
`no_match` means a fingerprint was searched, while `unavailable` or `error` means
the lookup could not complete. Keep the reason with those entries. For a batch:

```bash
jq '.queries | group_by([.status, .reason]) | map({status: .[0].status, reason: .[0].reason, count: length})' fid.json
```

`single_candidate` is not a verified identity. Check database provenance and compare
instructions, callers, and known reference objects before applying a name, especially
for short functions or when `specific_hash_evidence` is `no_candidate_matches`.
Leave ambiguous identities unresolved. For automatic naming, check
`fid_databases[].status`: a missing or incompatible database does not fail the listing.

For a raw image, give FID the verified mapping and, on ARM32, the decode mode:

```bash
kuna fid query ./rom.bin --db ./library.fidb --addr 0x329 --raw-image \
  --target ARM:LE:32:v7 --base 0 --isa thumb --json
```

## Explore options only when the task warrants it

Explore `--option NAME VALUE` only when an error blocks progress or changes the
solution, or the requested operating mode calls for exploration. Skip cosmetic
issues; even a small error matters if it changes the checker's comparison.

1. Identify the concrete discrepancy using assembly, bytes, or observed execution.
2. Search `kuna docs options` by symptom, or `kuna catalog --json` for structured
   discovery. Read `kuna docs phases` only if phase context is needed.
3. Change one relevant option for the affected function, keeping other settings fixed.
   Compare against the original result and evidence; stop once the uncertainty is resolved.

For a custom processor with fixed instruction slots, keep the executable section
headers when comparing function inventories. AIF counts slots from each section
start when the processor spec declares `kuna.aif.sectionInstructionStride`.
A segment-only ELF or raw mapping uses the ordinary cursor for the selected mode.

Supply known prototypes and types with `--assert` and function entries with
`--define-function`. If assembly or execution establishes every taken destination
of a computed jump, pass `--assert 'jumptable SITE TARGET [TARGET ...]'` using
hexadecimal code addresses. Keep the guard's fall-through out of the list.
Destinations must stay in the jumping function; state one tail-call target
with `callee`. A symbol without a size, such as a PE export, holds no jump,
so declare the function's extent with `--define-function START-END=NAME` when
a destination is its entry or another symbol inside it. Kuna rejects the set where case labels would be false, such as a PIE, shared
object or relocatable PE, TE or MZ image, a segmented jump, or a destination
wider than the jump's value, and the function then decompiles as without it. Use `--assert-strict` to catch rejected facts. Save reusable directives one per
line and pass `--assert @FILE`. For `decompile-all --jobs N` with multiple
workers, every directive must be a `jumptable`. Use `--jobs 1` for a mixed
file. For project exports with assertions, use serial, non-streaming mode.
See `kuna docs cli` for a runnable example and address conventions.

## Follow challenge clues and verify critical operations

Search the export for input handling, prompts, and success/failure paths. If needed,
query string owners and references:

```bash
kuna strings ./challenge --filter '(?i)password|flag|correct|wrong' --json
kuna crypto ./challenge --json          # AES/SHA/MD5/CRC/TEA/Base64 constants + the functions using them
kuna blobs ./challenge --json           # embedded PNG/JPEG/GIF/BMP/ICO/zip/gzip/xz/zstd files, exact lengths (--extract DIR)
kuna xrefs ./challenge --from main --json
```

Treat generated C as an estimate. Verify answer-critical branches, widths, signedness,
and indirect calls against assembly or observed execution. Use `kuna disassemble` for
targeted instructions and `kuna read` for encoded data and lookup-table bytes.

## Recover from command failures

Read the actual error and apply a relevant suggested correction before retrying.

| Error or input | Next step |
|---|---|
| Packed image (UPX or NEOLite) | Run `kuna unpack ./challenge -o ./challenge.unpacked`, then analyze the output. |
| Entry section not flagged executable | If the bytes are code, retry with `--define-function 0xADDR` using the reported address. Unpack packed images first. |
| Known headerless image | Use `--raw-image` with the correct target, base, and entry, as below. Do not infer these from a damaged header alone. |
| Archive (`.tgz`, `.zip`) | Extract it and analyze the executable inside. |

For a DOS `.COM` loaded at offset `0x100`:

```bash
kuna decompile-all ./challenge.com --raw-image \
  --target 'x86:LE:16:Real Mode' --base 0x100 --entry 0x100 --jobs 1
```

Verify the architecture, byte order, and file-offset-to-address mapping from evidence
such as a memory map, image metadata, or a known load address. Do not infer the base
by rounding an apparent entry address. In a Cortex-M vector table, the first word is
the initial stack pointer and the second is the reset vector; first locate the table,
which may not start at file offset zero. Use reset vector bit 0 as the Thumb indicator,
not as part of the instruction address.

For a verified function-pointer table, record each slot and decoded target.
An odd in-range value alone does not prove it points to code. Seed only reviewed
targets with repeatable `--entry`, or use `--addr` to seed and select output.
Use `--define-function @FILE` to declare entries or bounds after discovery.
Raw-image decompilation requires explicit entries and serial execution;
`--summary`, `--reachable-from`, `strings`, `xrefs`, `disassemble`, and `read` require
object metadata.

If corrections fail, record the command and error, then use another decompiler or
dynamic analysis when available. Suggest an issue report after the original task;
if fixing Kuna is in scope, consult the [repository](https://github.com/Noelo-Lab/kuna).

## Share files when delegating

When delegating, give each analyst the binary path, shared project directory, one
bounded question, and relevant symbols or addresses. Request concise findings with
exact symbols, addresses, and evidence; share the running export rather than duplicating it.

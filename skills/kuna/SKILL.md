---
name: kuna-decompiler
description: Use the Kuna CLI to recover searchable C and assembly, trace program logic, and validate findings in CTF solving, vulnerability hunting, malware identification, and program repair. Covers using Kuna, not developing the decompiler.
---

# Kuna for reversing binaries

Kuna provides fast, flexible, source-like decompilation through a CLI on PATH, not an
MCP server. It needs no connection and saves no analysis state between invocations.
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
skip this extra call if scope is clear.

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
for structured batch output; project export does not accept `--json`.

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

## Explore options only when the task warrants it

Explore `--option NAME VALUE` only when an error blocks progress or changes the
solution, or the requested operating mode calls for exploration. Skip cosmetic
issues; even a small error matters if it changes the checker's comparison.

1. Identify the concrete discrepancy using assembly, bytes, or observed execution.
2. Search `kuna docs options` by symptom, or `kuna catalog --json` for structured
   discovery. Read `kuna docs phases` only if phase context is needed.
3. Change one relevant option for the affected function, keeping other settings fixed.
   Compare against the original result and evidence; stop once the uncertainty is resolved.

Supply known prototypes/types with `--assert`, or function entries with
`--define-function`; consult `kuna docs cli` for syntax. Save reusable assertions in
an `@file` and check acceptance. `--assert` requires decompilation without `--stream`
and without a worker pool (`--jobs 1`).

## Follow challenge clues and verify critical operations

Search the export for input handling, prompts, and success/failure paths. If needed,
query string owners and references:

```bash
kuna strings ./challenge --filter '(?i)password|flag|correct|wrong' --json
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

Adjust target/base/entry to the image; a bare-metal firmware image usually keeps its
entry in its first word, so `entry = *(uint32_t *)&file[0]` and the base is that value
rounded down to the region start. `--entry` only seeds the load: discovery sweeps the
executable bytes for call targets from there, so an unfiltered `decompile-all` emits
what the image calls, not just the seed (`--option rawdiscover off` to disable, `--addr`
to narrow). Raw-image decompilation requires explicit entries and serial execution;
`--summary`, `--reachable-from`, `strings`, `xrefs`, `disassemble`, and `read` require
object metadata.

If corrections fail, record the command and error, then use another decompiler or
dynamic analysis when available. Suggest an issue report after the original task;
if fixing Kuna is in scope, consult the [repository](https://github.com/Noelo-Lab/kuna).

## Share files when delegating

When delegating, give each analyst the binary path, shared project directory, one
bounded question, and relevant symbols or addresses. Request concise findings with
exact symbols, addresses, and evidence; share the running export rather than duplicating it.

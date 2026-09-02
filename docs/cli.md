# The `kuna` CLI reference

The user-facing commands are the single Rust binary `kuna`
(`decompiler/crates/kuna-cli`, built to `decompiler/target/release/kuna` by
`make binaries`). This is the full reference; the one-screen version is in
`docs/agents.md`.

All command output goes through a fallible stdout boundary. A downstream reader that closes
the pipe early is a normal terminal condition, not the `println!` panic (exit `101`) it used
to be: no panic text, no broken-pipe diagnostic. It suppresses the *diagnostic*, not the
*verdict* — the command still exits with the code its own work earned, so `kuna test | head`
on a regressed baseline exits `1` and the DIV-45 failure contract below holds with or without
a reader. Other stdout write failures are real errors: reported, and exit `1`.

## `kuna test` — the parity gates

```bash
kuna test --all --baseline docs/baseline.json          # expect: PARITY OK
kuna test --datatests --json                           # machine-readable
kuna test --datatests --datatests-dir tests/stages \
    --baseline docs/baseline-stages.json               # the stage-issue corpus (= make test-stages)
```

`kuna test` parses the harness's two streams separately (unit results on **stderr**,
datatest results on **stdout**) and exits nonzero on any failure or baseline regression.
`--save-baseline PATH` re-records a baseline. Routine use: `docs/baseline-stages.json`
when adding stage tests. `docs/baseline.json` is re-pinned only for sanctioned intentional
changes (an upstream sync per `docs/history.md`, a DIV-recorded default flip) — never to
absorb a regression.

## `kuna decompile` — one function

```bash
kuna decompile ./a.out main
kuna decompile ./stripped.bin 0x401040 --addr
kuna decompile ./a.out main --option compareform canonical
kuna decompile ./sparc.elf main --option returnpair single
kuna decompile ./a.out main --language rust
```

Drives `decomp_dbg` as a subprocess and captures `print C` via `openfile write`, so
interactive prompts never pollute the output. `--option NAME VALUE` (repeatable) and
`--kassert "<args>"` flip phase-model sub-phase assertions per run; `--mode
auto|reliable|aggressive|fast` applies an option preset (`docs/modes.md`).

**Paths containing spaces work (DIV-100).** This is the one surface that reaches the engine
through a console *script* rather than an in-process call, and the console reads a
filename with `s >> filename` — whitespace-delimited. An unquoted path with a space
therefore split into two arguments: `load file` took the head as a BFD target and
loaded the tail, and `openfile write` truncated the redirect at the split, writing
the C to a file named after the first component. The CLI now quotes a path that
needs it, and the console's `read_filename` accepts a double-quoted argument
(`\"` and `\\` are escapes inside quotes; any other backslash is literal, so a
Windows path survives either spelling). Unquoted paths parse exactly as before.
Hand-written console scripts and interactive `decomp_dbg` sessions get the same
grammar — quote the path when it contains a space:

```
load file "/home/u/test dir/a.out"
openfile write "/tmp/out dir/main.c"
```

**`--language auto|c|rust`** selects the output language. **`auto` is the
default and follows the binary**: a Rust binary renders as Rust, because kuna
already detects one (`kuna-analysis`'s `sourcelang` pass, the port of Ghidra's
`SourceLanguageAnalyzer`) and rendering it as C is worse in a way the reader has
to undo by hand (DIV-80). Detection is high-precision, not heuristic; an
unreadable file leaves C in place; and `--language c` always wins, so the policy
can only ever add a language. It lowers to the upstream `option setlanguage`, so
`--option setlanguage rust-language` is equivalent; an unknown name is an error
rather than a silent fall back to C. `decompile-all --json` reports the resolved
choice in a top-level `"language"` key. The same recovered function is rendered
through a different profile — types, structuring and analysis are identical —
producing `unsafe fn n(mut a0: i64) -> u32`, `let mut v: T;` declarations, Rust
primitive spelling, `x as T` casts, `loop`/`while c {}`, and `match v { A | B =>
{ … } _ => {} }`. The contract is `syn::parse_file` validity, not `rustc`
compilation: the output calls functions that have no definition and does no type
checking. Constructs Rust cannot express — an unstructured `goto` the structurer
could not remove, a C switch fall-through — render as a comment plus a diverging
`panic!("kuna: …")` so a lossy site is never mistaken for a translation; grep
that marker to measure them, and `--option gotoreduce on --option taildup on
--option ifelseflatten on` to reduce them. `--language` also works on
`decompile-all`; `decompile-project` is C-only -- it never auto-selects, and errors on an
explicit non-C language -- and the Ghidra front-end pins its markup document to
C. The browser decompiler carries the same three choices in its **Language**
control. See `docs/spec/09-emission.md` §9.6.
Omitting `--mode` selects `auto`: files below 500 KiB use `aggressive`, files
from 500 KiB up to 2 MiB use `reliable`, and files at least 2 MiB use `fast`.
The raw on-disk byte length is used, with exact cutovers at 512,000 and
2,097,152 bytes. A later explicit `--option` wins over the resolved preset.
Address-selected single-function decompilation suppresses a preset-provided
`fast_funcdisc` whole-image walk because the requested entry is already known.
Name selection keeps it enabled so generated `sub_<addr>` names can resolve;
explicitly spelling `--option fast_funcdisc on` opts an address run back into
that analysis.

**Failure contract (DIV-45).** A function whose decompile pipeline aborts is
*loud*:

- **exit code `1`** — the same code as a run-level error (no such function, no
  architecture, no C at all). Exit `0` means the pipeline completed.
- **stderr** carries `error: decompilation failed for <fn> in <binary>:
  <reason>`, followed by `note: decomp_dbg stderr:` and the console's own
  stderr (the panic line and its source location), truncated at 2000 chars.
- **stdout still carries the recovered shell**, whose body comment names the
  same reason: `/* WARNING: decompilation failed: <reason> */`. A shell with
  the generic `/* WARNING: structured blocks unavailable (structuring
  declined) */` means the pipeline *ran* and produced no structured blocks —
  a different failure.

**Load and analysis failures (DIV-90).** `kuna decompile` runs the engine in a
subprocess, so it recovers *why* a run failed from the console transcript — and
reports it in the same words `decompile-all` / `functions` / `decompile-project`
use, so one failure reads identically from all four commands:

- **the binary could not be loaded** — `error: could not build an architecture
  for <binary>: <reason>` (e.g. `Non-global scope has empty name`, `No sleigh
  specification for x86:LE:64:default`, `not in recognized object file format`),
  exit `1`. The older `could not build an architecture for <binary>
  (unsupported/!recognized binary)` is now only the fallback for a transcript
  that carried no reason at all.
- **the analysis commit failed** — `error: read symbols (analysis commit)
  failed: <reason>`, exit `1`, **and no C**. The console keeps its session alive
  after a failed `read symbols`, so C *can* still be rendered, but from a program
  whose debug facts were applied only up to the failing step and cannot be
  re-committed; that C used to be printed with exit `0`, indistinguishable from a
  binary with no symbols at all. `--option datasyms off` (or naming whichever
  analysis pass is implicated) is the way to get a run through.

The abort itself is not fatal to the console session (`decomp_dbg` prints
`Skipping <fn>: <reason>` and keeps going, so datatest `<stringmatch>` rules
still evaluate); the CLI is what turns it into a non-zero exit.
`decompile-all` / `decompile-project` / the WASM front-end are unaffected: a
failed function stays a per-function `error` record and never aborts the batch
(its text now carries the real panic message instead of `panic with non-string
payload`).

## `kuna decompile-all` / `kuna functions` — whole binary, machine-readable

```bash
kuna decompile-all ./a.out --json                      # every CODE-backed function
kuna decompile-all ./a.out --functions main,parse --json
kuna functions ./a.out --json                          # full callable-symbol inventory
```

The whole-binary surface (the benchmark + LLM path). Runs **in-process**
(`kuna_console::engine::bootstrap_from_object` → `commit_pending_analysis` → loop
`decompile_func` + `print_c`), loading + analyzing the binary **once** instead of
`kuna decompile`'s subprocess-per-function (≈10×+ faster on a many-function binary).

`--json` emits
`{binary,count,functions:[{name,address,address_hex,aliases,size,code,error,
line_mappings:[{line_number,addresses}],variables:[{name,type,kind,arg_index,
stack_offset,size,line_numbers,addresses}]}]}` (`kuna functions --json` emits
`name`/`address`/`address_hex`/`aliases`/`size` per function). `line_mappings` maps 1-based
lines in `code` to sorted, unique machine-instruction VMAs. Variable `line_numbers`
come from the printer's `varref` tokens; variable `addresses` are the union of the
mapped instruction addresses on those lines. Both are empty when no backed use is
emitted. The references are captured from Kuna's markup emitter and resolved against
the live p-code IR, rather than inferred from the rendered text. The ordinary
plain-text renderer still produces `code`, so its bytes are unchanged.
Reported variables are joined to native varrefs by ABI or stack storage and recovered
high-variable identity. Multiple high-variable fragments are combined only when they
name the same exact stack location and size; ambiguous name-only matches stay empty.

Per-function `size` is the entry's byte extent, and both surfaces report the same
number with the same meaning — it is an **inventory** fact, measured without
decompiling, so `kuna functions --json` alone is enough to rank a binary's functions
by weight (the "decompile the three biggest functions" first move costs one call, not
a whole-binary run). It is an **upper bound**: the address-contiguous clip from the
entry to the next entry, or to the end of the containing CODE section, whichever comes
first — so inter-function alignment padding is counted in. Against ELF `st_size` over
the 1428 symbolized-fixture functions with ground truth it is never short, exact for
231, and overshoots by a median of 8 bytes (worst 52). An entry in no CODE section — an
import pointer slot, an undefined external — reports `0`, as does a function whose
extent could not be measured. A caller needing the exact body must still decompile.

Per-function `code` matches `kuna decompile ... --option listing on` byte-for-byte on
x86-64 (elsewhere, see the injected defaults below), `error` isolates a single failed
function, and `variables` (params in ABI order + DWARF/stack locals) feed type-recovery
scoring. `--no-vars` leaves `variables` empty but still emits function line mappings.

Behaviors specific to `decompile-all`:

- **Executable default targets** — an unfiltered run decompiles canonical entries
  contained by loader sections marked `CODE`. Callable import pointer slots in PE
  IATs, Mach-O symbol-pointer sections, and similar data areas remain in `kuna
  functions`, remain installed for named calls and prototypes, and remain
  reachable through explicit `--addr`; they are not automatically decoded as
  function bodies. Analysis-discovered entries inside executable sections join
  this default set. `--functions` retains its normal first-match behavior when a
  stub and slot share a name. Loaders without section metadata retain the
  complete inventory.

- **One record per function entry** — a whole-binary run reports (and decompiles) each
  entry address exactly once. A function can carry several names: a `.symtab` symbol
  plus a debug-info one (`macho_dwarf.o` has `_l0` and `first_byte` at `0x0`), a
  decorated/undecorated PE pair, or the generated `sub_<addr>` placeholder an analysis
  pass registers over an already-named entry. `name` reports the most informative of
  them — a real symbol beats a synthesized `_INIT_<i>`/`_FINI_<i>`/`_DT_INIT`/`_DT_FINI`
  table name, which beats a generated `sub_`/`func_`/`FUN_`/`LAB_` placeholder; ties
  prefer the unprefixed spelling (`main` over `_main`), then the shorter name — and
  `aliases` carries the rest (`[]` when there is only one). `--functions <name>` matches
  aliases too, so any name that used to select a function still does. On ARM the Thumb
  mode bit is folded out of symbol addresses, so a function whose ELF `st_value` is odd
  (`compute` at `0x100b9`) is reported once, at its real even entry — and `--addr` accepts
  either spelling, resolving an odd ARM address to the entry it belongs to instead of
  decompiling mid-instruction. The fold is ARM-only: an odd address on a byte-aligned ISA
  is a genuine entry and is left alone.

- **Injected default options**: under the concrete `reliable` preset it injects
  `option listing on` unless the caller names
  `listing` (DIV-15), so the default-on `noreturn_propagate` call-graph fixpoint fires and
  a stripped binary's unnamed exit/fatal wrappers no longer swallow the functions after
  them; on non-x86-64 binaries it likewise injects `funcstart_patterns on` and `aif on`
  unless the caller names them (see `docs/history.md`). `--option listing off` opts
  out; single-function `kuna decompile` also injects Listing.
  `kuna functions` shares the **discovery** half of that policy (DIV-68): on a
  non-x86-64 binary it injects `funcstart_patterns on`, `aif on`, and the
  `listing on` those two are gated behind, so the inventory always contains every
  entry `decompile-all` would decompile (stripped betaflight STM32F405 under
  `--mode reliable`: 1 entry listed before, 5,798 after, against the 5,797
  `decompile-all` decompiles). That costs a whole-program decode there — 0.08 s to
  5.27 s on that firmware — which is the price of a correct answer. On x86-64
  `kuna functions` injects nothing and is unchanged: the Listing is measured
  entry-neutral on that architecture, so it stays the decompiling surfaces'
  default. The interactive console keeps the engine default off; an auto-selected
  `aggressive` preset names all three itself, on either surface.
  Omitted `--mode` first resolves the size-based `auto` policy. `--mode fast`
  names and disables the three exhaustive program-wide decode/discovery options
  (`listing`, `funcstart_patterns`, `aif`), suppressing those injections, and
  enables `fast_funcdisc`. That bounded pass recursively promotes direct CALL
  targets from loader-backed roots and adds conservatively validated
  pointer-table targets, so a stripped project does not collapse to imports plus
  its entry point. An explicit `--addr` selector suppresses the preset-provided
  pass because the entry is already known; `--functions` keeps discovery active
  so generated names can resolve. Explicitly spelling `--option fast_funcdisc
  on` opts an address run back in. A later explicit `--option` always wins.
- **Per-function watchdog** — `--max-fn-seconds N` (`0` disables): an
  unfiltered `decompile-all`/`decompile-project` run in the resolved `fast`
  preset defaults to 10 seconds per function. On native, selected-function runs
  and the other presets retain 120 seconds; an explicit value always wins. WASM
  arms only the fast whole-binary 10-second policy and leaves its other commands
  unbudgeted. A function whose decompile drive exceeds the budget is cut off
  cooperatively (deadline probes at the action/rule-pool/heritage loop
  boundaries) and recorded as that function's `error` (`"per-function
  decompile budget exceeded (N s)"`), the batch continuing. This is not a hard
  process timer: it does not bound discovery, unprobed decoder work, C/variable
  rendering, artifact construction, total export time, or memory. Driver
  policy, not a stage-model settable — zero output change for a function whose
  drive completes before expiry; the console / `decomp_dbg` parity path never
  arms it.

The decbench backend (`decbench/decompilers/raw/kuna_raw.py`) shells out to
`kuna decompile-all --json`.

## `kuna decompile-project` — recompile-oriented project export

```bash
kuna decompile-project ./a.out                         # writes ./a.out.kuna/
kuna decompile-project ./a.out -o proj --functions main,parse
```

The project-export face of the same in-process core
(`decompiler/crates/kuna-cli/src/decompile_project.rs`, a thin wrapper over the shared
`kuna_console::project` module — the decompile loop + artifact builders also behind the
web UI's Download-Binary-Source zip and `kuna_wasm project`). Identical
load-once/decompile-many path and flags —
`--functions`/`--addr`/`--max-fn-seconds`/`--mode`/`--option`/`--slice`/`--target`/
`--sleighpath`; no `--json`. Omitted mode is the same size-based `auto` policy
as the other file front-ends. In particular, a project input at least 2 MiB
automatically suppresses the exhaustive Listing consumers, prologue scan, and
AIF gap walk through the `fast` preset, while substituting rooted direct-call
and bounded pointer-table discovery. Its unfiltered per-function watchdog also
defaults to 10 seconds instead of 120; `--max-fn-seconds` overrides it,
including `0` to disable. Explicit `--addr` selections remain exact and
suppress that whole-image walk by default; named selections keep it so
generated names can resolve. Explicit `--option fast_funcdisc on` can restore
its program facts for an address-selected run, but does not add definitions
outside the selection.

Writes a project folder — default `<binary-filename>.kuna/` next to the binary,
`-o/--output DIR` overrides — of four artifacts designed so a human or LLM can study the
binary and attempt recompilation:

- `<name>.c` — every decompiled function, address-ordered, under
  `// Function: <name> @ <addr>` headers, failures as comments, `#include "<name>.h"`.
  One definition per loader- or analysis-discovered executable entry address:
  the export shares
  `decompile-all`'s CODE-backed target policy and one-record-per-entry
  enumeration above, so data import slots are not rendered as functions and a
  function carrying several names cannot produce several identical definitions.
- `<name>.h` — include guard + a generated recompile prelude (core scalar and
  `undefined`-family typedefs), the recovered user-defined type definitions, and one
  prototype per decompiled function, token-identical to the `.c` definition line.
- `<name>.asm` — labeled linear disassembly of every CODE section: labels match the `.c`
  function names, per-function `; arg:`/`; stack:` comments map decompiled variables to
  storage, undecodable bytes as `db` lines, and a `; --- data ---` tail labeling named
  globals plus every `dat_<hex>` the `.c` references, with raw bytes.
- `README.md` — size, arch id, entry point, function counts, sections table, file
  inventory.

The artifact format is purely additive and has no exporter-specific transform
(spec §9.7); the set of emitted definitions follows the selected P1 discovery
options, including `fast_funcdisc`.

## `kuna catalog` — option discovery (the LLM control API)

```bash
kuna catalog --json              # the flippable assertion list, for an agent
kuna catalog --markdown          # regenerate docs/options.md
kuna catalog --check             # fail on catalog/registration drift (CI)
kuna catalog --tier transform    # filter to the transform-tier control surface
```

Parses the decompiler's `phase catalog` JSON (single source of truth: `settableTable`,
generated from `decompiler/crates/kuna-decomp/phases.toml`) into the documented, flippable
assertion list. `--markdown` output is tier-grouped and symptom-indexed; `--check`
cross-checks the catalog against `kuna_decomp::options::KUNA_OPTION_NAMES` in-process.
The rendered catalog is `docs/options.md`; the model behind it is `docs/phases.md` /
`docs/spec/`; the defaults are recorded in `docs/history.md`.

## `kuna specs` — the SLEIGH compiler

```bash
kuna specs -a specs/             # compile every .slaspec under a dir (slacomp's -a mode)
kuna specs <file.slaspec>        # compile one
```

A thin alias for `slacomp` (same CLI as upstream's `sleigh_opt`).

## Everything else

`kuna modes` (list the option presets) and `kuna fid` (function identification) also
exist, plus minor flags not covered here (`--no-vars`, `--raw`, `--regions`, `--timeout`,
…) — see the usage block in `decompiler/crates/kuna-cli/src/main.rs`.

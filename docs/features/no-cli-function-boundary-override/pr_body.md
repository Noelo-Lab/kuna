## What was broken

`docs/re-needs/no-cli-function-boundary-override.md` (severity **blocker**, track `tooling`, scope `large`):

> The console has `map function <addr> [name] [nocode]` and `load addr <addr> [name]`, both
> functional. The `kuna` binary can emit neither […] Worse, `function F spans [start,end)`
> does not exist ANYWHERE — extent is derived in `kuna-console/src/funcextent.rs` as
> `[entry, min(next_entry, section_end))` with no override […] On an obfuscated or packed
> image, where discovery is exactly what fails, the agent has no lever.

**1 instance**, captain-seeded, with an independent round-2 witness recorded in T_TRIAGE
(`overlapping-anti-disassembly-sequence`: `kuna decompile … 0x80489e6 --addr` exiting 1 at an
internal target). Round 1 hit the same wall three times.

## The hypothesis was half wrong

Filed hypothesis: *"the cheap half is exposure, the expensive half is the stubs."*

- **START — upheld.** Declaring an entry really was pure exposure; `map function` works.
- **END — overturned. No stub was involved.** It was inert *plumbing*. `Funcdata::size` is
  threaded from `map function` / `load addr` / `decompile_one` all the way to
  `Architecture::new_funcdata`, and `FlowInfo` carries a fully ported
  `set_range` / `new_address` / `fallthru` / `handle_out_of_bounds` range machinery. The two
  ends were simply never joined: every call site passed `UNBOUNDED_SIZE`, and
  `follow_flow_on_fd` never called `set_range`. Joining them is ~10 lines in
  `decompile_drive.rs`, not an engine port.

## The mechanism

`--define-function <start[-end][=name] | @file>`, repeatable, on `decompile`,
`decompile-all`, `functions`, `decompile-project` and `disassemble`.

```bash
# two functions merged into one: say where the first really ends
kuna decompile-all ./packed.bin --json --addr 0x13c9 --define-function 0x13c9-0x1420=stage1

# keep what you worked out, and pass it to every later command
kuna functions ./packed.bin --json --define-function @bounds.txt
```

- **`start`** declares the entry the way `map function` does — function symbol, name→address
  registration — so it enumerates, resolves by name and names its call sites.
- **`end` is exclusive** and records a *declared extent* in a new per-program store,
  `ConsoleProgram::declared_extents`. That store is consulted by **every later load of that
  entry** (`load function`, `load addr`, the whole-binary loop) and by `funcextent` when the
  inventory reports a size. A declaration outlives the one command that made it — which is
  what separates an interface from a one-shot flag. Cross-*process* durability is
  caller-carried by design: the `@file` is the artifact.
- Declarations are applied **after** the analysis commit, so they override discovery rather
  than compete with it.
- The console spelling is one new kuna-only command, `function bounds <start> [<end>] [as
  <name>]`. It takes plain integers rather than the `parse_machaddr` grammar, whose
  `[space,offset,size]` size is indistinguishable from the address width for a small size.

### The failure mode this feature introduces, and the stub that hid it

Refuting the mechanism against *wrongness* rather than against no-op turned up the real
defect: a declared end that cuts real control flow produced a **silently empty body** with
`error: null`. `FlowInfo::handle_out_of_bounds` computed the C++ `Function flow out of
bounds` message and dropped it on the floor (a W4 stub). Un-stubbed to the two
`Funcdata::warning` / `warning_header` calls the C++ makes:

```c
void chopped(void) // warn: Function flows out of bounds
{ // warn: Function flow out of bounds: r0x000013d4 flows to r0x000013d4
}
```

A correctly declared function ends in a return and never leaves its range, so that comment is
the signal to widen the bound. The flow range is the whole entry-point space unless an extent
is declared, so this fires only under a declared boundary.

## The acceptance probe now passes

The need had **no probe** in its front matter — one of the records the acceptance suite
reports `unrunnable`. Both arms are now authored against `scripts/repipe/probe.py`'s schema
with a pinned `target` block (in-repo fixture `aif_gap_x86_64`, sha256
`1a592a85…fceeef`, 14408 bytes), so neither is a `{{BIN}}` substitution with `target: null`.

| arm | on main `5fec5ff9` | on this branch |
|---|---|---|
| reproduction `p-83ce32ba278c` | **PASS** — exit 2, `error: unknown option --define-function` | n/a |
| acceptance `a-88c4db106ade` | **FAIL** (same exit 2) | **PASS** |

```
python -m scripts.repipe.verify --need no-cli-function-boundary-override --json
  counts {'total': 1, 'pass': 1, 'fail': 0, 'closed': 1, 'regressed': 0, 'indeterminate': 0}
  passed True  flaky False  unrunnable False  transition closed
```

Promoted verbatim to `tests/cli/no-cli-function-boundary-override.json`.

The A/B on one build, same address:

| | name | size | body |
|---|---|---|---|
| no flag | `sub_13c9` | 682 | 54 lines, swallows 25 leaf callees through `sub_1393` |
| `--define-function 0x13c9-0x1420=stage1` | `stage1` | 87 | the 1 call inside the declared extent |

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK** — 675/675 assertions |
| `make test-stages` | **PARITY OK** — 594/594 assertions |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | OK |
| acceptance probe | **PASS** |

**Inertness, measured rather than asserted.** Both parity corpora are symbol-less bytechunks
and never reach the real-ELF loader path, so they cannot see this change. Sweeping
`decompile-all --json` over **all 73 real-ELF fixtures** in
`kuna-analysis/tests/fixtures/`, main's release `kuna` vs this branch's: **0 differ**. Every
byte of output is unchanged for a run that declares nothing.

> **Note for whoever runs `make rust-test` next in a repipe worktree:** `verify_w10_proto_unlock`
> fails spuriously unless `KUNA_DECOMP_TEST` is unset. The harness points that variable — which
> the test reads as the *C++* oracle path — at a Rust `decomp_test_dbg`, so the test compares
> Rust against Rust and reports "oracle signature drifted". CI does not set it.

## Not closed (stated, not hidden)

- `xrefs` and `strings` load through the same `Args` but do not accept the flag.
- kuna never writes boundaries back into the image; the `@file` is the durable artifact.
- Renaming is supported only as the `=NAME` half of a declaration — general rename/prototype
  override is the separate open need `no-cli-rename-or-prototype-override`, and
  `no-cli-data-code-override` / `no-cli-structuring-override` are likewise untouched.
- **`analysis-generated-function-name` is NOT affected** — checked, since this branch touches
  `kuna-cli/src/decompile.rs`. A *declared* name resolves as a selector here
  (`kuna decompile <bin> stage1` works for an entry the image never named), but an
  *analysis-generated* `sub_<addr>` selector already resolved on main for the same fixture,
  and resolves identically with and without this change. Nothing here closes or moves that
  need.

## No option row, by design

Per `docs/agents.md`, an option gates *anything that can change emitted C*. A CLI flag that
only carries caller assertions cannot change output for a run that does not pass it — proven
by the 73-fixture sweep above — so there is no `phases.toml` row, no `options.rs`
registration, no catalog counter and no stages XML. Behaviour changes are described in prose
in `docs/spec/00-overview.md` (the declared-boundary plane) and `docs/spec/02-lift-and-flow.md`
(declared flow bounds and their diagnostics).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

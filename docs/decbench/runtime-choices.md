# Runtime-choice registry

Some decompiler behaviors are **not universally better** — they win on the majority of
functions but lose on a specific *source shape*. kuna keeps the majority-winning behavior
as the default and exposes the alternative as a per-run `--option`, so an agent (or a human)
can flip it **per function** when the source shape calls for it. This registry documents
each such lever: the default, when to flip it, and the source-shape signal that tells you to.

Discovered by the decbench ghidra-beats-kuna analysis (`docs/decbench/triage-ghidra/`, the
`--option X off` divergence-revert experiment) and the angr campaign. See `docs/decbench-loop.md`
for the loop; `kuna catalog --json` is the machine-readable option surface an agent reads.

A behavior belongs here **only if both renderings are valid** and the choice is a genuine
tradeoff. A behavior whose non-default output is *invalid or wrong* is a bug, not a choice —
those are listed at the bottom as "not runtime choices" so they don't get mis-filed.

## The levers

### `taildup` — tail duplication (default: ON)

- **What ON does (the majority win):** duplicates a shared bare-return/epilogue block into
  each predecessor, eliminating a `goto` and linearizing the code. Goto-free output reads
  cleaner on the bulk of functions — why it's default-on (DIV-14).
- **When to flip OFF:** the function has a **shared cleanup / free-chain epilogue** reached
  from many predecessors — several early-exit error checks that all free/dispose the *same*
  locals and `return` (the C `goto out;` idiom). There, tail duplication forks the single
  cleanup into N source-absent copies; keeping it merged (`taildup off`) matches the source
  and Ghidra.
- **Source-shape signal:** multiple early-exit error checks → one `free(a); free(b); return r;`
  cleanup block with many predecessors.
- **Evidence:** `kex_choose_conf` (openssh ssh, O2-noinline) — `taildup off` recovers Ghidra's
  merged epilogue (GED 54 → ~0). `--option taildup off`.
- Related: the `returndup` angr feature (below) auto-detects this shape from the other
  direction (splitting a merged return); here the existing `taildup` lever already closes it.

### `iteregion` — recover `?:` ternaries from assignment diamonds (default: ON, DIV-17)

- **What ON does:** rewrites an `if/else { x = a } { x = b }` two-arm assignment diamond into
  `x = cond ? a : b`. Ghidra's printer has no ternary, so this is a kuna-added S9 form.
- **Default ON (DIV-17):** the ternary form matches the common format/print/flag source where
  compilers turn `cond ? "%s," : "%s"` into exactly this diamond (`print_link_flags`, coreutils
  output helpers). 0/675 datatests change (byte-identical corpus even default-on), so the flip is
  clean. Still a runtime choice — the object code is identical for an explicit `if/else`.
- **When to flip OFF (`--option iteregion off`, per function):** the source **likely used an
  explicit `if/else`** — then the diamond is what the author wrote and a ternary would invent a
  form they never used. Off is byte-identical to upstream.
- **Source-shape signal:** many two-arm same-destination assignment diamonds converging on a
  call argument (print/format/flag builders) ⇒ leave ON. A lone diamond guarding distinct
  branch bodies ⇒ consider OFF. Evidence: `print_link_flags` (iproute2 ip), GED 140 → 11.
- Shipped by decbench feature F5 (PR #125), default-on per user directive. `--option iteregion off` to revert.

### `cstyle-null-cmp` — terse null/bool comparisons (default: OFF)

- **What ON does:** renders pointer/bool comparisons as `if (x)` / `if (!x)` instead of the
  explicit `if (x != 0)` / `if (x == 0)`. Matches the angr/IDA/source terse convention.
- **When to flip ON:** you want output that matches the terse majority convention and the
  Joern-based GED metric (the explicit `!= 0` form expands into extra CFG nodes and is
  penalized even when the control flow is identical — see `shell_initialize`,
  `compspec_dispose`). It is a **pure rendering style**, no control-flow change.
- **When to leave OFF (the default):** you want explicit comparisons for review clarity
  (kuna/Ghidra house style). Default-off keeps the explicit form.
- Shipped by decbench feature F6 (PR TBD). `--option cstyle-null-cmp on` *(name TBD)*.

### Speed-gated opt-ins (default: OFF for cost, not correctness)

These win on quality where they trigger but carry heavy per-function cost, so they stay
off by default and an agent flips them on when it needs the recovery on a specific function
(see `docs/divergences.md`, `[[kuna-default-on-sweep]]`):

- **`switchguardbound`** — extend jump-table bound analysis across a guard. "Incredibly slow"
  (PR #60); flip on for a specific "Too many branches" computed-call failure.
- **`unrolledguard`** — recover MSVC unrolled-memcpy jump tables. **~3.9×** on the recovered
  function; flip on for that specific optimized-memcpy switch shape.

## How an agent uses this

1. `kuna catalog --json` lists every option with its `default`, `use_when`, and `summary`.
2. When the default output looks wrong for a *known source shape* above, re-decompile that one
   function with the lever flipped: `kuna decompile <bin> <fn> --option <lever> <value>`
   (or `kuna decompile-all <bin> --addr 0x… --option <lever> <value>`).
3. The choice is **per function** — never flip a lever globally to chase one function; the
   default is the default because it wins on the rest.

## Not runtime choices (bugs — fix on-default, do not flip)

Recorded here so the divergence-revert experiment doesn't mis-file them as choices. In each
the non-default output is *invalid or strictly wrong*, so there is no valid tradeoff to expose.

- **`regionstructure` nested-`if`-in-`while`-condition** (`overwrite_database`, cronie crond):
  the SAILR region structurer folds a loop body containing a nested `if` into the `while`
  condition, emitting **uncompilable C** — the same defect class PR #122 fixed for
  while-conditions, surviving when the body has a nested `if`. `regionstructure off` is only a
  workaround (it disables SAILR entirely, regressing the majority). The real fix is on-default:
  fall back to `while(true){ …; if(cond) break; }` when the refined body can't reduce to a pure
  expression condition. Tracked as a follow-up correctness PR (F7).

## Benchmark caveat (not a kuna behavior at all)

Some ghidra-beats-kuna GED gaps are **decbench scoring artifacts**, not kuna choices:
`compspec_dispose` / `dispose_variable` (bash) use **K&R old-style** function definitions
(`f(x) TYPE x; { … }`), which degenerate Joern's source-CFG parse to ~1 node at benchmark time
— so Ghidra's *also*-degenerate stub matched (GED 0) while kuna's correct ~19-node CFG was
charged its full size. The decbench `GED_MIN_SOURCE_NODES` fix (decbench PR #6) excludes these
degenerate-source cases on the next benchmark run.

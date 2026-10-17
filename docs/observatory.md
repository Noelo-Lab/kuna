# The codebase observatory

Local, generated views of the codebase — run `make observatory` from the repo
root and open `observatory/index.html` in a browser (the pages are
self-contained and work over `file://`). Nothing under `observatory/` is
committed; regenerate at will. Generators live in `scripts/observatory/`
(unit-tested by `make test-tools`).

| Page | Shows | Regenerate |
|---|---|---|
| `observatory/atlas.html` | The architecture atlas: the P0–P9 pipeline with per-phase cards (owned artifact, modules by size, sub-phases, options, console surfaces, owning `docs/spec/` chapter), the shared layers (`substrate/`, `infra/`), the workspace crate graph, and the real pass order. | `make observatory` |
| `observatory/churn.html` | Churn & ownership: a squarified treemap of every tracked file (area = live size or windowed churn; color = churn intensity, agent share, or recency), with scope/window controls and a per-group rollup table. | `make observatory` |
| `observatory/coverage.html` | Merged line coverage: workspace suite + datatest/stages corpora + tests/cli probes through instrumented binaries, per crate / per phase folder / per file, with the llvm-cov HTML report under `observatory/coverage/`. The atlas phase cards pick the numbers up too. | `make coverage` |

Each page inlines its data; the same data is written as raw JSON under
`observatory/data/` for agents and scripts:

- **`data/atlas.json`** — `{schemaVersion, meta{sha,generated}, phases[],
  extraFolders[], crates[], schedule{passes[],unmapped[],legend,source,note},
  counts{group,subphase,surface,settable}}`. Per-phase: `{id, name, folder,
  artifact, readsAs, band, orthogonal, specChapter, files, loc, modules[],
  groups[], subphases[], options[], surfaces[]}`. Per-pass:
  `{index, name, kind(action|rule|container), depth, repeat, once, disabled,
  group, phase, definedIn, folderPhase}` — `phase` is the action-group's
  phase (semantic, via `phases.toml` `[[group]]`), `folderPhase` the folder
  the defining module lives in (physical); they legitimately differ.
- **`data/coverage.json`** — `{schemaVersion, meta{sha,generated,runsFailed},
  totals, crates[], phases[], files[]}`, each with
  `lines{count,covered,percent}` (+ functions on the rollups).
- **`data/churn.json`** — `{schemaVersion, meta, windowsDays, areas, files[]}`;
  per file: `{path, area, crate, phase, loc, last, authors[{name,commits}],
  windows{all,d30,d90,d365}{added,removed,commits,agentCommits}}`. Full
  no-merge history with `--find-renames`; agent commits are `[AUTOMATED]`
  or Co-authored-by claude/codex/chatgpt/copilot; only currently-tracked
  files under decompiler/scripts/tools/integrations/docs/tests are kept
  (vendored `specs/` is excluded).

## Where the data comes from

Everything is read from committed sources, so the atlas can never drift
silently from the tree:

- `docs/phases.md` (the two pipe tables: phase names/artifacts/Band B and the
  folder map) — parsed, not duplicated.
- `decompiler/crates/kuna-decomp/phases.toml` (groups, sub-phases, surfaces,
  settable options).
- `docs/spec/*.md` `Anchors:` blocks (folder → owning chapter).
- `decompiler/crates/kuna-decomp/tests/fixtures/list_action_decompile_snapshot.txt`
  — the byte-enforced listing of the default `decompile` schedule — joined
  against the declarative registry in `infra/universalaction.rs` (`rrow!`
  rows, `act!` group literals, `ActionBase::new` definition sites) for each
  pass's group and defining module. Passes that cannot be attributed are
  listed in `schedule.unmapped` rather than guessed.
- `cargo metadata --no-deps` + `git ls-files` for the crate inventory and
  line counts.

Options:
`python -m scripts.observatory.generate --out DIR [--only atlas] [--render-only]`
(`--render-only` rebuilds the pages from whatever JSON already exists).

## Coverage: prerequisites and caveats

`make coverage` needs `rustup component add llvm-tools-preview` and
`cargo install cargo-llvm-cov` (the script checks and says so). It always
re-measures from scratch: a full instrumented build into
`decompiler/target/llvm-cov-target` (a second build tree; debug info is
forced off to keep it small) plus every suite — expect 30–60 minutes. Suite
failures do not abort the measurement; they are reported as
`meta.runsFailed` and the report covers what ran. Two sharp edges it handles
for you: `KUNA_DECOMP_TEST` is pinned so the datatest runs cannot silently
fall back to an uninstrumented `decomp_test_dbg`, and profile output uses a
pool-merged `%16m` pattern so the thousands of spawned processes do not
leave one `.profraw` each. A process killed by a watchdog/timeout loses its
counters; if that matters for a run, llvm's continuous mode (`%c` in
`LLVM_PROFILE_FILE` plus `-C llvm-args=-runtime-counter-relocation` in
`RUSTFLAGS`) is the escape hatch.

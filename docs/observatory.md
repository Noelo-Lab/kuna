# The codebase observatory

Local, generated views of the codebase — run `make observatory` from the repo
root and open `observatory/index.html` in a browser (the pages are
self-contained and work over `file://`). Nothing under `observatory/` is
committed; regenerate at will. Generators live in `scripts/observatory/`
(unit-tested by `make test-tools`).

| Page | Shows | Regenerate |
|---|---|---|
| `observatory/atlas.html` | The architecture atlas: the P0–P9 pipeline with per-phase cards (owned artifact, modules by size, sub-phases, options, console surfaces, owning `docs/spec/` chapter), the shared layers (`substrate/`, `infra/`), the workspace crate graph, and the real pass order. | `make observatory` |

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

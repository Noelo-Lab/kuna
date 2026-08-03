---
case_id: audit-uncatalogued-options
pool: novel
group_id: kuna::option-catalog
status: feature-candidate
tier: A
margin: 0
fresh_verdict: "`readonly` is not the exception — ALL 38 upstream-inherited ArchOptions are absent from `kuna catalog`, and at least 8 of them measurably change emitted C on today's build via `--option`."
option_closing: null
feature_slug: catalog-upstream-options
scope: small
confidence: high
---

## What was filed

Round 1's loose thread (`docs/decbench/features.md`): *"`readonly` is registered without a
`settableTable` row, so it is invisible to `kuna catalog` and therefore to any agent
sweeping options by symptom. Check whether other Ghidra-inherited options share that gap."*

They all do. `readonly` is not an oversight; it is one member of an entire uncatalogued
family.

## Verify-first: what today's build actually does

### 1. The kuna-owned half is perfectly in sync — the gap is elsewhere

```
$ kuna catalog --json | (count) -> 83
KUNA_OPTION_NAMES                  -> 83
in KUNA_OPTION_NAMES not in phases.toml settableTable: []
in phases.toml settableTable not in KUNA_OPTION_NAMES: []
```

`kuna catalog --check` (`kuna-cli/src/catalog.rs:221`) enforces **bidirectional** equality
between the emitted catalog and `KUNA_OPTION_NAMES`, and it is green. So the contract that
exists is being honoured; the problem is what the contract does not cover.

### 2. All 38 upstream ArchOptions are invisible

`decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs` contains 38 `impl ArchOption`
blocks whose names are the ported Ghidra `OptionDatabase` set. Probing each against the
catalog:

```
$ for o in <38 names>; do kuna catalog --option $o; done
aliasblock               invisible      inplaceops               invisible
allowcontextset          invisible      integerformat            invisible
analyzeforloops          invisible      jumpload                 invisible
braceformat              invisible      jumptablemax             invisible
commentheader            invisible      maxinstruction           invisible
commentindent            invisible      maxlinewidth             invisible
commentinstruction       invisible      namespacestrategy        invisible
commentstyle             invisible      nanignore                invisible
conventionprinting       invisible      nocastprinting           invisible
currentaction            invisible      noreturn                 invisible
defaultprototype         invisible      nullprinting             invisible
errorreinterpreted       invisible      protoeval                invisible
errortoomanyinstructions invisible      readonly                 invisible
errorunimplemented       invisible      setaction                invisible
extrapop                 invisible      setlanguage              invisible
hideextensions           invisible      splitdatatype            invisible
ignoreunimplemented      invisible      togglerule               invisible
indentincrement          invisible      warning                  invisible
inferconstptr            invisible
```

**38 / 38 invisible.** Zero of them appear in `kuna catalog`, `kuna catalog --json`,
`docs/options.md`, or the `symptoms` index an agent greps.

### 3. They are nonetheless a live `--option` surface that changes emitted C

`kuna-cli/src/decompile_all.rs:455 apply_runtime_options` routes any name **not** in
`KUNA_OPTION_NAMES` through `registry().find_element(name, 0)` into `OptionDatabase::set`.
So `kuna decompile-all <bin> --option readonly on` works today. A/B on three stripped
corpus binaries (changed lines in the full `decompile-all` output vs a no-flag run):

| option | value | O0 coreutils/factor | O0 coreutils/ptx | O0 cronie/crontab | O2-noinline iproute2/ip |
|---|---|---|---|---|---|
| `integerformat` | `hex` | **2616** | — | — | — |
| `nocastprinting` | `on` | **1126** | **1096** | **576** | — |
| `jumptablemax` | `32` | **668** | **1214** | **212** | — |
| `analyzeforloops` | `off` | **321** | **183** | **43** | — |
| `inferconstptr` | `off` | **220** | **296** | **339** | — |
| `inplaceops` | `off` | **240** | **122** | **58** | — |
| `nullprinting` | `off` | **46** | **78** | **26** | — |
| `readonly` | `on` | 0 | 0 | **87** | **159** (63168 → 63107 lines) |
| `aliasblock` | `all` | 0 | 0 | 0 | — |
| `nanignore` | `all` | 0 | 0 | 0 | — |
| `conventionprinting` | `off` | 0 | — | — | — |
| `maxlinewidth` | `40` | 0 | — | — | — |

`readonly on` is the one round 1 already named — it is what closes
`O2-noinline-iproute2-ip-netns_add` — and it is reproduced here on today's build: 159
changed lines on `ip`, 87 on `crontab`. Seven more upstream options change output on
*every* binary tried.

### 4. Three of the 38 are not even reachable

```
$ kuna decompile-all factor --option hideextensions on
error: option hideextensions: Unknown option
$ kuna decompile-all factor --option splitdatatype off
error: option splitdatatype: Unknown data-type split option: off
$ kuna decompile-all factor --option warning off
error: option warning: Bad action/rule specifier: off
```

`hideextensions` is genuinely dead: its `ELEM_HIDEEXTENSIONS` (kuna-local id 4090) is
deliberately **not** in `UPSTREAM_OPTION_ELEMENTS`, so `find_element` returns 0 from both
the CLI (`decompile_all.rs:465`) and the console (`ifacedecomp.rs:761`). It is an
`ArchOption` impl with no caller. `splitdatatype`/`warning` are reachable but take
multi-token values the single-`p1` `--option NAME VALUE` form cannot express.

## Analysis

**Symptom (one, named):** the option catalog — kuna's entire discovery surface for the LLM
control API — documents 83 of the 121 options the binary actually accepts, and the 38 it
omits include the ones that flip read-only constant propagation, constant-pointer
inference, for-loop recovery, jump-table bounds, cast printing, and integer radix. An agent
sweeping `use_when`/`symptoms` for "stores into .rodata that the binary never performs"
cannot find `readonly`, because as far as `kuna catalog --json` is concerned it does not
exist. Round 1 hit this exact wall on `netns_add`.

This is the `docs/improvement-pipeline.md` *provenance contract* being satisfied only on the
half of the surface that was born after the contract. CLAUDE.md's rule — *"anything that can
change emitted C ships behind a named option ... `kuna catalog --check` must stay green"* —
is enforced by a check that structurally cannot see the upstream family, because
`cmd_check` compares the catalog to `KUNA_OPTION_NAMES` and the upstream options are not in
that list by construction.

**Owning phase: P0** (knowledge/configuration plane). Owning files:
`decompiler/crates/kuna-decomp/phases.toml`,
`decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs`,
`decompiler/crates/kuna-cli/src/catalog.rs`.

**Not a metric artifact, not covered by an option, not already fixed.** It has zero GED
value and zero direct correctness value — it is a *discoverability* defect, which is exactly
the class the campaign says to rank on: round 1 lost a case to it.

## Classification of the 38

Not all 38 deserve a row. The settableTable models a **per-run decision point with a closed
value set**; several upstream options are console verbs or per-function facts.

### Tier A — 16 rows to add. Per-run knobs, closed enum values, drop straight into today's row model

| option | values | current default | phase | evidence |
|---|---|---|---|---|
| `readonly` | `on\|off` | `off` (`architecture.rs:1208`) | P3 | 159 lines on `ip`, 87 on `crontab`; the `netns_add` case |
| `inferconstptr` | `on\|off` | on | P5 | 220 / 296 / 339 |
| `analyzeforloops` | `on\|off` | `true` (`architecture.rs:1206`) | P8 | 321 / 183 / 43 |
| `aliasblock` | `none\|struct\|array\|all` | `array` (level 2, `:1211`) | P6 | 0 observed (default already blocks) |
| `nanignore` | `none\|compare\|all` | `compare` (`:1210`) | P3 | 0 observed |
| `jumpload` | `on\|off` | off | P2 | unmeasured |
| `ignoreunimplemented` | `on\|off` | off | P2 | unmeasured |
| `errorunimplemented` | `on\|off` | off | P2 | unmeasured |
| `errorreinterpreted` | `on\|off` | on | P2 | unmeasured |
| `errortoomanyinstructions` | `on\|off` | off | P2 | unmeasured |
| `allowcontextset` | `on\|off` | on | P0 | unmeasured |
| `namespacestrategy` | `minimal\|all\|none` | `minimal` | P9 | 0 observed |
| `nullprinting` | `on\|off` | on | P9 | 46 / 78 / 26 |
| `inplaceops` | `on\|off` | — | P9 | 240 / 122 / 58 |
| `nocastprinting` | `on\|off` | off | P9 | 1126 / 1096 / 576 |
| `conventionprinting` | `on\|off` | on | P9 | 0 observed |
| `integerformat` | `hex\|dec\|best` | `best` (`printlanguage.rs:606`) | P9 | 2616 |

(17 lines listed; `allowcontextset` is the judgement call — drop it and it is 16.)

The four P9 printing toggles are the **same class** as the already-catalogued
`truthycond` / `braceelide` / `arraynotation` / `warnstyle` / `compareform`, which carry
`tier = "core"`, `change_kind = "presentation-default"`. There is no principled reason
`truthycond` has a row and `nullprinting` does not.

### Tier B — 6 options that need a `values` grammar extension first

`jumptablemax`, `maxinstruction`, `maxlinewidth`, `indentincrement`, `commentindent`,
`extrapop`. All take an integer. `build.rs:399` validates with
`if !st.values.split('|').any(|v| v == value) { return false; }` and `gen_option_values`
emits one enum-typed field per settable, so an open integer domain does not fit. All 83
existing rows are closed enums (`on|off` ×78, plus `pair|single`, `off|on|wide`,
`inline|banner`, `canonical|original`, `angr|ghidra`).

`jumptablemax` is the one that matters — 668/1214/212 changed lines, and it is the direct
sibling of the already-catalogued `switchmodbound` / `switchguardbound`. It should get a row
once a `values = "<int>"` (or `"<int:1..65536>"`) form exists.

### Tier C — 15 correctly excluded

`inline`, `noreturn`, `extrapop`(also B), `defaultprototype`, `protoeval`, `setaction`,
`currentaction`, `setlanguage`, `togglerule`, `warning`, `commentheader`,
`commentinstruction`, `commentstyle`, `braceformat`, `splitdatatype`, `hideextensions`.
These are console verbs (`setaction`, `togglerule`), per-function facts requiring a name
argument (`inline`, `noreturn`), or multi-token configurations — not per-run decision
points with a closed value set. `hideextensions` is dead code and should be either wired
into `UPSTREAM_OPTION_ELEMENTS` or deleted; that is a separate one-line judgement.

## Proposed fix

### The blocker nobody has noticed

Adding a `settableTable` row alone **fails `kuna catalog --check`**: `cmd_check`
(`catalog.rs:233-246`) reports `catalog row "readonly" matches no registered kuna option
(stale)`. Adding the name to `KUNA_OPTION_NAMES` instead **breaks dispatch**, because both
`apply_runtime_options` (`decompile_all.rs:457`) and the console `IfcOption`
(`ifacedecomp.rs:748`) check `KUNA_OPTION_NAMES.contains()` **first** and route to
`Architecture::set_kuna_option`, whose `match name` (`architecture.rs:1283`) has no arm for
upstream names and would return a parse error. So the naive "just add the rows" PR
regresses `--option readonly on` from working to erroring.

### Mechanism (fits in one module of code)

Two viable shapes; recommend the second.

1. **Passthrough arm.** Add the 16 names to `KUNA_OPTION_NAMES` and give
   `Architecture::set_kuna_option` a fallback arm that forwards to
   `OptionDatabase::new().set(self, find_element(name), p1, "", "")`. One new match arm +
   one helper in `infra/architecture.rs`. Keeps `cmd_check` unchanged.
2. **Teach `cmd_check` the upstream class** (recommended, smaller and truthful). Leave
   `KUNA_OPTION_NAMES` alone; relax `cmd_check`'s `cat.difference(&registered)` arm to also
   accept a name that resolves in `UPSTREAM_OPTION_ELEMENTS`. ~6 lines in
   `kuna-cli/src/catalog.rs`, no dispatch change, and the two dispatch sites keep working
   unmodified because upstream names still fall through to `OptionDatabase`. Rows omit
   `live_field`/`live_true`/`live_false` — an established pattern (18 analysis gates and
   `loweredswitch`/`stackguard`/`namestyle` already do).

### True size of the fix

`docs/improvement-pipeline.md` §3.4 names three count sites; measured against the actual
files, adding 16 rows costs:

| site | what | 83 → 99 |
|---|---|---|
| `src/p0_knowledge/kuna_phases/tests.rs:48,49` | `kuna_num_settables()`, `SETTABLE_TABLE.len()` | 2 edits |
| `tests/catalog_bytecompat.rs:82,84,86` | `"option":` / `"tier":` / `"symptoms":` counts | 3 edits |
| `tests/fixtures/phase_catalog.json` | the **byte-for-byte** catalog fixture (`assert_eq!(rust, FIXTURE)`) | regenerate |
| `tests/stages/kuna-catalog.xml:40,44` | `#5 use_when` / `#9 change_kind` are `min="10" max="160"` | **no bump** (99 < 160) |
| `tests/stages/kuna-catalog.xml:41,42,43` | `#6/#7/#8` exact buckets keyed on `source_decompiler: "angr"` / `change_kind: "structure-recovery"` / `"opt-in-tool"` | **no bump** if every new row uses `source_decompiler = "ghidra-upstream"` and `change_kind = "presentation-default"` or `"analysis-enablement"` |
| `docs/options.md` | `kuna catalog --markdown >` | regenerate |
| `docs/spec/` | the P0 chapter | prose |

So: ~5 hand-edited count asserts, one regenerated fixture, one regenerated doc, 16 TOML rows
of ~18 fields each, and ~6 lines of Rust. The genuinely expensive part is **writing 16
honest `use_when` + `symptoms` strings** — the whole point of the exercise is that they are
greppable by symptom, and a lazy row is worse than no row.

Standing requirement 5 (a two-pass `tests/stages/` testcase per option) does **not** apply:
no behavior is being added, and the underlying `apply` bodies are already covered by
`p0_knowledge/options/tests.rs`. `tests/stages/kuna-catalog.xml` is the right regression
fence — extend it with a `min="1" max="1"` stringmatch on the `readonly` row.

### Recommendation: ONE PR for Tier A, with a per-option judgement pass inside it

The blocker (`cmd_check` / dispatch) is shared, the count-bump surface is shared, and the
fixture must be regenerated exactly once — splitting this into 16 PRs would collide on
`phases.toml`, the fixture, and every count assert (the known two-PRs-in-flight conflict
from CLAUDE.md). One PR, `catalog-upstream-options`.

But each **row** is a judgement call and must be written as one: `readonly`'s `use_when`
has to say *"upstream Ghidra keeps this off for RELRO reasons; turning it on folds
read-only loads to constants and hides — not fixes — the `rodata-phantom-store` defect"*,
which is the opposite advice from `nocastprinting`'s. Do not template them.

Tier B (`jumptablemax` + 5 integer options) is a **follow-up PR** and needs the `values`
grammar extension in `build.rs` first; it is not blocked on Tier A. Tier C stays out, with
one line in the P0 spec chapter recording *why*, so the next auditor does not re-open this.

## Risks

- Regenerating `tests/fixtures/phase_catalog.json` is a byte-exact recapture (the header of
  `catalog_bytecompat.rs` documents the console `openfile write` / `phase catalog`
  procedure); getting it wrong fails opaquely.
- `docs/options.md` grows by 16 rows and is checked by `make rust-test` freshness.
- Zero emitted-C risk: no default changes, so `make test` / `make test-stages` stay
  byte-identical. No DIV row needed.

## What was broken

RE-friction need `pe-function-inventory-labels` (round 2, challenge `6547b4d50f4238b24302b588`,
1 instance, severity major). The tester wanted *"a trustworthy function inventory for triaging
the PE"* and reported:

> The inventory includes executable ranges named `std::bad_alloc::vftable_1` and numerous
> `std::basic_stringbuf::vftable_N` entries as functions, including ranges hundreds of bytes
> long. These names are data-symbol aliases and misleadingly classify routines as vtable
> objects.

10 entries on that PE, from `0x140001040` (32 B) to `0x140001de0` (496 B).

## The filed hypothesis is overturned; the symptom stands

The hypothesis was *"public data aliases near code addresses are winning the canonical-name
selection"*. No data symbol is involved. `<Class>::vftable_<i>` is a name **kuna synthesises**:
the MSVC RTTI pass (R3, `analyzers/rtti/mod.rs::emit_vftable_methods`) walks each recovered
vftable and emits a `SymFact{Function}` at the address each slot **points at** — the virtual
method, not the table. All 10 entries are genuine `.text` routines and classifying them as
functions is correct.

The name is what is wrong, and not merely cosmetically: an MSVC class under multiple
inheritance genuinely owns several vftables, so `<Class>::vftable_<i>` collides with the
natural reading *"this class's i-th vftable"*. The need's STOP clause — stop if the fix changes
function DISCOVERY rather than labelling — does not fire: which addresses the pass creates is
untouched.

## Mechanism

**1. The slot function is named after the function.** `<Class>::vftable_<i>` →
`<Class>::vfunc_<i>`. Only the table itself keeps a `vftable` name, and it keeps the unindexed
`<Class>::vftable` Data label it always had.

**2. A slot-index name is structural, so a real method name outranks it.** The rename alone was
a *net regression* on one function, and only the whole-corpus sweep found it.
`entry_name_rank`'s tiers are `(placeholder, structural, leading-underscore, LENGTH)`, so
between a recovered method name and a synthetic slot name the reported name was decided by a
one-character length accident:

| address | before this PR | after rename only | after both changes |
|---|---|---|---|
| `0x140002c2c` | `std::basic_streambuf::showmanyc` (30) | **`std::basic_stringbuf::vfunc_5`** (29) | `std::basic_streambuf::showmanyc` |
| `0x140002c32` | `std::basic_streambuf::uflow` (27) | `std::basic_streambuf::uflow` | `std::basic_streambuf::uflow` |

The fix is not to pick a longer word. A slot-index name is structural in exactly the sense the
existing `_FINI_<i>` tier already documents — *it says which slot the function occupies, not
what the function is called* — so `<Class>::vfunc_<i>` (MSVC) and `<Class>::vtable_<i>`
(Itanium) now sort into that tier, and a real method name outranks them at any length. They
still beat `sub_`/`FUN_` placeholders, which lose a tier earlier.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need pe-function-inventory-labels
acceptance suite  sha 74118fd
  PASS   closed         open           pe-function-inventory-labels
total=1 pass=1 fail=0 closed=1 regressed=0 indeterminate=0
```

`"name": "[^"]*::vftable_[0-9]+"` count on the recorded PE: **10 → 0**.

Promoted to `tests/cli/pe-function-inventory-labels.json`, re-pointed off the dataset PE onto
the in-repo `msvc_rtti_x64.exe`, which carried the identical defect (`Box::vftable_0`) — CI has
no dataset. The acceptance is a single `stdout_absent` clause, which a dead `rtti` pass would
also satisfy, so the promoted probe adds a positive `"name": "Box::vfunc_0"` clause and pins
`--mode aggressive` instead of relying on auto's 500 KiB threshold to turn the gate on.

## Sweep

`kuna functions --json --mode aggressive` over **19 targets** — all 9 in-repo PE fixtures, all 8
C++/ELF fixtures including the stripped Itanium-RTTI `.so`, the `fauxware`/`aif_gap` controls,
and the recorded dataset PE — plus `kuna decompile-all --json` over the 9 PE fixtures.

- 3 files changed, 50 lines; **residual after normalising `::vftable_<i>` → `::vfunc_<i>`: 0**.
  Every changed line anywhere is exactly that one name.
- The ELF half of the rank predicate is **measured-inert**: `itaniumrtti_x86_64.so` is stripped,
  so its 26 `<Class>::vtable_<i>` entries only ever face `sub_<addr>` placeholders, which
  already lose at tier 1. It is carried by unit tests, not by a fixture.

No speed measurement: no pass, decode or analysis step was added or removed. The rtti change is
one format string; the ranking change adds one `rsplit_once` + prefix test per `(name, entry)`
pair inside an inventory sort that already compares those strings.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK**, 675/675 assertions |
| `make test-stages` | **PARITY OK**, 600/600 assertions |
| `make rust-test` | green — 5,317 tests over 343 suites |
| `make check-spec` | `check-spec OK` |
| `kuna catalog --check` | `catalog OK: documents exactly the registered kuna options` |
| `python -m scripts.repipe.clitests` | 17/17 |
| new unit tests | 3 in `kuna-console::engine`, 1 in `kuna-analysis::rtti`, `verify_rtti.rs` updated |

Tooling track: no option, no `phases.toml` row, no catalog counter, no `tests/stages` case — so
no counter or file lease was taken.

## Left for a follow-up (recorded in the need)

The MSVC pass has no defining-class attribution, so a slot shared by a base and its derivatives
is named once per class and the canonical pick among them is arbitrary
(`std::bad_alloc::vfunc_0` canonical over `std::exception::vfunc_0` on lexicographic order
alone). The Itanium sibling already attributes an inherited slot to the class that *defines*
it. The Itanium pass's own `vtable_<i>` stem carries the same naming defect and was left alone
deliberately: renaming it edits the `itaniumrtti` row in `phases.toml`, which is leased to
another builder this round.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

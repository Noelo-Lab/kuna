## incorrect-duplication-chcon: name-match known-no-return externs in ET_REL `.o` files

Closes the gap angr's `test_decompiling_incorrect_duplication_chcon_main` exposes on coreutils
`chcon.o` (`main`): kuna emitted **~90 lines of garbage** after the no-return `__stack_chk_fail()`
call — the inter-function alignment padding (`00 00 …`, decoded as `add byte ptr [rax], al`)
decompiled as live code. angr emits none.

See [`docs/features/incorrect-duplication-chcon-a0e113/analysis.md`](docs/features/incorrect-duplication-chcon-a0e113/analysis.md)
for the full analysis.

### Root cause

`stack_chk_fail` is in kuna's vendored ELF known-no-return list (`option noreturn_known`,
default-on), so on a normal dynamically-linked ELF the call is marked no-return and the dead
fall-through never decoded. But in an **ET_REL `.o`**, `__stack_chk_fail` is an *undefined extern*:
its `.symtab` entry has `address()==0` and there is no PLT stub, so the **address-keyed**
`noreturn_known` scan emits no fact for it. The `relocobjects` loader installs a `FunctionSymbol`
named `__stack_chk_fail` at a synthetic target (the call *prints* the name) but no no-return flag is
set — so flow runs off the end into the padding.

### Mechanism

New `option noreturn_externmatch` (S2 flow-follow, ElementId 4104). When set, the
`FlowEnvironment::query_call_no_return` seam (`infra/decompile_drive.rs`) **also** reports no-return
for a callee whose *name* matches the same vendored ELF list — `flow.rs` ORs this with the proto
flag at the artificial-halt site, so a name match plants the halt and the trailing padding is never
decoded. The matcher (`s2_lift/kuna_noreturn_externmatch.rs`) `include_str!`s the **same** list
`noreturn_known` uses (`kuna-analysis/data/ElfFunctionsThatDoNotReturn`) and applies the **same**
leading-`_` strip + global/`std` namespace guard, so it adds no risk class beyond the already
default-on `noreturn_known`; it merely reaches the ET_REL extern the address scan structurally
misses. A no-op on a normal ELF (the proto flag is already set).

### Option & default

- `option noreturn_externmatch on|off` — flip via `kuna decompile … --option noreturn_externmatch off`.
- **Default-ON (DIV-13).** The full ablation is clean: **0 / 675** upstream datatest assertions
  change with it default-ON, and it is **~23% faster** on the target (`chcon.o::main` median
  449.8 ms → 344.3 ms, n=5 — there is less dead padding to decompile). Set OFF to restore the prior
  byte-identical rendering.
- One interaction: `option noreturn_known off` no longer alone restores the post-`exit`/`__stack_chk_fail`
  dead code (this feature is a second name-based gate); disable both to see the raw fall-through.

### Verification

- `kuna catalog --check` → catalog OK
- `kuna test --datatests --baseline docs/baseline.json` → **PARITY OK** (675/675)
- `make test-stages` → **PARITY OK** (193/193; `tests/stages/ghangr-incorrect-duplication-chcon-a0e113.xml`, +3)
- `make rust-test` → green (3 new matcher unit tests; count/golden fixtures bumped)

🤖 Generated with [Claude Code](https://claude.com/claude-code)

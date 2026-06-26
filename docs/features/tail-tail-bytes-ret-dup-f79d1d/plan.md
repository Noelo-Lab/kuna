# Plan — option `noreturn_extern`

**Feature**: at decompile flow time, treat a direct CALL whose resolved callee name
matches the known ELF no-return list as no-return, even when the address-keyed
no-return flag is unset (the undefined-extern `.o` case). Closes the `tail_bytes`
boundary blow-up (326→~87 loc).

## Mechanism

Gate a name-based fallback in the existing flow seam
`FlowEnvironment::query_call_no_return`. Modeled exactly on `tailcalljump`
(`is_tail_call_branch`): a bool flag on `Architecture`, an `option` arm, read once
at flow time on the main arch (no ArchSeam copy needed — flow runs on the main arch,
like `is_tail_call_branch`).

## Files

New module:
- `decompiler/crates/kuna-decomp/src/s2_lift/kuna_noreturnextern.rs`
  - `ELEM_NORETURNEXTERN` = `ElementId::new("noreturn_extern", 4103)` (next free above 4102).
  - `pub fn matches_noreturn_extern_name(name: &str) -> bool` — base-name match
    (leading `_` stripped, `std`-only namespace guard) against a static list
    mirroring `kuna-analysis` `ElfFunctionsThatDoNotReturn`
    (exit, cexit, c_exit, abort, reboot, longjmp, longjmp_chk, siglongjmp, panic,
    stack_chk_fail, cxa_throw, cxa_terminate, cxa_call_unexpected, cxa_bad_cast,
    Unwind_Resume, assert_fail, assert_rtn, fortify_fail, ZSt9terminatev,
    ZN10__cxxabiv111__terminateEPFvvE, pthread_exit).
  - unit tests.

Anchor edits (each `// (kuna)`):
1. `infra/architecture.rs`
   - field `pub noreturn_extern_calls: bool` (near `tail_call_jumps`)
   - constructor `noreturn_extern_calls: false`
   - reset `self.noreturn_extern_calls = false;` (default-off opt-in)
   - option arm `"noreturn_extern" => on_off!(noreturn_extern_calls, "Name-based extern no-return")`
2. `infra/decompile_drive.rs` — in `ArchFlowEnv::query_call_no_return`: when
   `arch.noreturn_extern_calls` and the address-keyed check is false, fall back to
   `kuna_noreturnextern::matches_noreturn_extern_name(query_call(entry))`.
3. `p0_knowledge/options.rs` — add `"noreturn_extern"` to `KUNA_OPTION_NAMES`.
4. `s2_lift/mod.rs` — `pub mod kuna_noreturnextern;`.

Registration / discovery:
- `stages.toml` — `settableTable` row (source_decompiler=angr, change_kind, summary,
  use_when, example, all provenance fields).

## Test

`tests/stages/ghangr-noreturn_extern.xml` — two passes over a `tail.o` byte chunk /
the binary: pass 1 `option noreturn_extern off` asserts the bug (falls through into
the next function — e.g. `fstatfs`/`die_pipe` appears, or the spurious outer loop);
pass 2 default asserts the fix (function bounded, `__stack_chk_fail()` is the last
call before the final return, none of the neighbour-function symbols present).

## Default decision

Default-OFF while developing. Run the 675-datatest ablation with it ON; if 0 change
and speed within budget, flip default-ON + DIV-N. Otherwise stays opt-in.

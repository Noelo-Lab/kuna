# Plan — tail-call jump recovery (option `tailcalljump`)

Scope: **small** (decider-ratified). Modeled on `kuna_v850indbranch.rs`.

Option name: `tailcalljump` (the slug `tee-o2-tail-jumps-4a1f49` is awkward / not a
valid Rust ident). ElementId: **4100** (next free above 4099).

## Pieces

1. **New module** `decompiler/crates/kuna-decomp/src/s2_lift/kuna_tailcalljump.rs`
   - `ELEM_TAILCALLJUMP = ElementId::new("tailcalljump", 4100)`
   - `kuna_is_tail_call_branch(data, op, dest_is_known_fn, dest_is_self) -> bool`:
     fires iff gate on AND `op.code()==CPUI_BRANCH` AND `dest_is_known_fn` AND
     `!dest_is_self`. (The name-resolution / self-entry comparison are resolved by the
     caller in `decompile_drive.rs`, the v850 register-name convention.)
   - `TailCallJumpOption` (default off) + `apply`/`is_enabled` (the v850 option shape).

2. **flow.rs `xref_control_flow`** — `CPUI_BRANCH` arm, the non-constant `else`:
   when `self.env.is_tail_call_branch(&self.data, curop, &destaddr)` fires, instead of
   `new_address`: `op_set_opcode_code(curop, CPUI_CALL)`, `let nr = setup_call_specs(curop)?`
   (which, for a noreturn callee, already plants the noreturn halt via
   `check_for_flow_modification`), and **only if `!nr`** plant `artificial_halt(addr, 0)` +
   `op_dead_insert_after`. Re-derive `cursor = dead_next(curop)` (so the planted RETURN is
   picked up next iteration), set `*startbasic = true`. Mark with `// (kuna)`.

3. **flow.rs `FlowEnvironment`** — add `fn is_tail_call_branch(&self, _data, _op, _dest) -> bool { false }`
   (default, like `is_v850_indirect_jmp`).

4. **decompile_drive.rs `ArchFlowEnv`** — implement `is_tail_call_branch`: fast-path
   `if !arch.tail_call_jumps { return false }`; else resolve `dest_is_known_fn =
   self.query_call(dest).is_some()` and `dest_is_self = dest == fd.get_address()`, call the
   predicate.

5. **architecture.rs** — `pub tail_call_jumps: bool` field, `false` in ctor + reset path,
   `"tailcalljump" => on_off!(tail_call_jumps, "Tail-call jump recovery")` arm.

6. **options.rs** — add `"tailcalljump"` to `KUNA_OPTION_NAMES`.

7. **stages.toml** — settableTable row (all provenance fields), `change_kind =
   structure-recovery`, `source_decompiler = angr`.

8. **lib.rs** — `mod kuna_tailcalljump;` (under s2_lift).

## Test

`tests/stages/ghangr-tee-o2-tail-jumps-4a1f49.xml`: bytechunk of `setlocale_null_androidfix`
(`31 f6 e9 .. .. .. ..`) + a minimal PLT thunk + a named `setlocale` external (or map the
real tee_O2 bytes). Two passes: `option tailcalljump off` asserts the bug
(`Treating indirect jump as call` / `dat_` indirect call); default-... actually ship
default-OFF, so pass 2 sets `option tailcalljump on` and asserts the fix
(`setlocale(` present, warning absent).

## Default decision

Ship **default-OFF opt-in** unless the ablation is clean (0/675) AND the speed gate passes,
in which case flip default-ON + DIV entry. Decide after measuring.

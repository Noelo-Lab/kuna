# varargstackargs — design

**Slug / option:** `varargstackargs` (on|off) · **Phase:** P4 ·
**Sub-stage:** active-input-trial-scoring · **ElementId:** 4132 ·
**Arch flag:** `vararg_stack_args`

## The one-line change

`ParamListStandard::fillin_map_standard` scores each resource section of a call's
trials with two positional rules (`force_no_use`, `force_inactive_chain`). When the
callee's prototype is variadic, cut a section in two at its first stack trial and
score the halves independently.

## Where it hooks

```
FuncCallSpecs::resolve_and_derive_input_map        (called from ActionActiveParam)
  FuncProto::derive_input_map
    ParamListStandard::fillin_map
      fillin_map_standard
        build_trial_map
        force_exclusion_group
        separate_sections            -> trial_start[]
   +    kuna_varargstackargs::stack_section_split   <- the new decision
        force_no_use            (per section)
        force_inactive_chain    (per section)
        mark everything still active as used
```

`stack_section_split(active, start, stop)` returns the index of the first
spacebase trial in the section, and only when

* `ParamActive::is_vararg_stack_split()` is set (the option, AND the callee's
  prototype is `dotdotdot`), and
* at least one non-spacebase trial precedes it in the section — an all-stack
  section (i386 `__cdecl`) has no boundary to draw, and
* that trial carries a `ParamEntry`, because it becomes the new section's
  `groupstart` via `ParamTrial::slot_group`.

The caller inserts the cut into `trial_start` and a parallel `group_start`, then runs
both existing loops over the finer partition. With the option off, `group_start` is
`resource_start` element-for-element and the partition is unchanged, so the default
output is byte-identical.

## Why the flag travels on `ParamActive` and not through `fillin_map`

`fillin_map` is a `ParamList` method; it has no `Funcdata`, no `Architecture` and no
`FuncProto`, so it can read neither the option nor `is_dotdotdot()`. `ParamActive` is
the per-call object that is already threaded through the whole recovery and is
mutable at exactly the place where both facts are in scope: `ActionActiveParam`, one
statement before `resolve_and_derive_input_map`. It is set there and nowhere else,
and `ParamActive::clear()` deliberately leaves it alone — like `recoversubcall` it is
a property of the call site, not of one pass.

## Why `dotdotdot` gates it

The register hole between the last fixed parameter and the first stack argument is a
property of the *calling convention* only when the callee is variadic. For a fully
known prototype a hole in the register file is real evidence that the argument list
ended, and upstream's rules should keep firing. `dotdotdot` is set from a real
prototype declaration (`libproto`, DWARF, a PE import, `parse line`), so it is never a
guess.

## Blast radius, predicted before measuring

Only an ABI that (a) puts registers and the outgoing stack area in one resource
section and (b) passes variadic arguments on the stack can produce the cut at all.
AArch64/Apple is the case. On x86-64 SysV and Win64 the varargs are in registers, so
no active stack trial exists to rescue; on i386 `__cdecl` the section is all stack, so
`stack_section_split` declines. Prediction: near-zero movement outside arm64 Mach-O.
Measured in `sweep.md`.

## What it cannot do

Nothing about trial *scoring* changes. A stack trial still has to arrive at
`fillin_map` ACTIVE on its own evidence — `check_input_trial_use`'s local-alias test,
caller-local-range test and `ancestor_op_use` all run first and unchanged — so the
option can only keep an argument the recovery already believed in. It cannot recover
a second or third vararg whose trial was never scored active, and it does not touch
the *callee*-side prototype of a variadic function (that is the `sub_40c81e` case in
`analysis.md`, which stays open).

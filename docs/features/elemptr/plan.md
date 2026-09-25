# elemptr — implementation plan

## 1. Where the rule lives: a vote in P5, not a rewrite in P9

The casts this removes are not a spelling problem. `*(char *)((long)v6 + (long)v8)`
casts twice because `v6` is declared `void *` and the add is priced as integer
arithmetic; `*(char *)(a0 + v7)` casts once because `a0` is declared `long`. A P9
rewrite (`castindex`, another lane) can respell the sum, but the declarations stay
wrong, and decbench scores the declarations. So the rule supplies the element pointer
as one more vote in `ActionInferTypes::buildLocaltypes`'s `getLocalType` fold
(`p5_types/coreaction_infertypes.rs`), after `calleevote`, `ptrfromuse` and `charptr`
and before `boolbyte`. From there upstream does the rest: propagation carries `T *`
to the copies, `RulePtrArith` turns `INT_ADD(p, i)` into `PTRADD(p, i, #W)`, and the
printer prints a subscript. No new `Action` is scheduled.

## 2. The rule (`p5_types/kuna_elemptr.rs`)

- Candidates: a function input the prototype model could place a parameter in; a
  `CALL` output whose callee left the pointee open, alone in its variable (no phi but
  its own copies and nulls, and not sharing the return register with another
  returned value); an unnamed global in a data section, decided from every Varnode
  holding it; a constant address in a data section that is only ever an add's base.
- The vote replaces an integer, unknown or pointer-at-nothing vote only.
- The walk: BFS over COPY/CAST/MULTIEQUAL/INDIRECT and literal offsets, ten hops.
  An add with a non-constant addend is an indexed access when the addend is an
  index by construction (scaled, extended, small mask, shift, division, loop counter,
  used as a number) or the candidate is already a pointer; never when the addend is
  itself a pointer by type or by a pointer-only use, through its copies and small
  literal offsets. The right-hand side of a pointer difference (`b + buf * -1`,
  the sum not dereferenced) is not a number use.
- A later `ActionInferTypes` pass that finds a parameter or call return an earlier
  pass typed indexing another base disputes it (`Funcdata`, surviving `clear`) and
  restarts the function without it.
- Commit: one access width W, whole elements, at least one computed index.
- Element: integer of width W, sign from extensions/orderings of loads, a declared
  integer pointee wins, typed stored values only break a tie; pointer-width
  elements need a named pointee or arithmetic-only uses.
- Propagation: a value stored through a pointer the rule typed keeps its own sign
  (`keeps_stored_sign`, in `propagate_type_edge`).
- Refusals: second width, scale mismatch or record stride, non-zero PTRSUB, pointer
  arithmetic, floats, literal compares, declared scalar or other-width pointees.

## 3. Globals: the batch ledger

`Ledger` on the `Architecture` (like `calleevote`'s): `record` files each function's
verdicts after its decompile; `disagreements` blocks a global for the functions that
typed it when another function refused it or typed another width; the batch drivers
(`project::converge_element_globals`, `decompile_all::converge_element_globals_callee_first`)
decompile those functions again. `seed` hands a function its blocked set, plus
every global some earlier function of the batch already disagreed about
(`disputed`), so only the functions decompiled before the first disagreement are
redone. Each storage (a global's address, a constant's value) is decided once per
`ActionInferTypes` pass, before it is classified, because a global has a Varnode
per call it lives across.

## 4. Printing

- `globalref`: an address that is the base of a `PTRADD` with a computed index is an
  array (`Plan::indexed`), printed without `&` and declared `T dat_X[]`; the header
  prefers the array declaration when no function reads the name directly.
- The string probe declines a literal the index can provably run past, and an empty
  literal under any non-zero index.
- A held global the rule typed (`dat_5068`) is declared in the header
  (`GlobalInfo::elem`), and `castimplied` reads a subscript of it at the declared
  element (`Plan::declared_type`), so `castternary` drops the `(int)` a `?:` arm
  of `dat_5068[i]` carries.

## 5. Tests

Unit tests (`kuna_elemptr/tests.rs`), the two-pass stage test
`tests/stages/kuna-elemptr.xml` on the compiled fixture, two `tests/cli` probes, and
the compiled round trip over three builds, two arms and two compilers
(`decompile_all_cli.rs`, `an_element_pointer_round_trips_through_the_printed_c`).

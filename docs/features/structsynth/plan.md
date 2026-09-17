# structsynth — implementation plan

## 1. The change

A new P5 action, `p5_types/kuna_structsynth.rs (ActionStructSynth)`, inserted into
`mainloop` immediately after `ActionInferTypes` in the `typerecovery` group, behind
`--option structsynth off|param` (default `off`).

**The default is `off` on measured evidence**, not by assumption. Of the four
criteria the user set — speed within +5%, the 444-slice `type_match` perfect
count within 0.1% of the off arm, a clean datatest/stage corpus under the flip,
and no bug hunk in the whole-corpus sweep — speed passes (+1.34% worst, inert
control +2.60%) and the corpora pass (675/675 datatests unmoved; 4 stage
assertions move and all four are the intended field rendering). Accuracy misses
by four-hundredths of a function (959 → 958 perfect, −0.104%), and the sweep
turns up a bug hunk: `find` O2 `sub_f620` emits `goto label_f752;` with no such
label. The whole story is `docs/features/structsynth/analysis.md` §6.6. The
option is likewise **not** in `AGGRESSIVE_OVERRIDES`: `auto` picks `aggressive`
under 500 KiB, which is most of the benchmark, so preset membership would
realize both costs on the scored path.

```
collect   one read-only walk of the live LOAD/STORE ops
          -> per base: {offset -> widest access, its value type}
          -> per base: negative evidence (non-constant term, integer use, phi)
decide    decline list below; widest access wins an offset
install   complete struct_N, then type-lock the base varnode to struct_N *
```

## 2. Why it must wait for a settled lattice

`ActionInferTypes::apply` returns 0 unconditionally — a type change is deliberately
not counted as a data-flow change — so nothing in the schedule can observe that
propagation has stopped moving, and `has_type_recovery_started` is armed at the end
of the *first* `fullloop`. An action gated on it reads the first of up to seven
passes. `run_infer_types` returning "no change" therefore sets a plateau flag on the
`Funcdata` (`funcdata.rs (kuna_infertypes_settled)`) and the synthesizer fires only
then, once per function, and never on a function that hit the 7-pass ceiling.

Measured on `fmt` O2 with `KUNA_ACTION_PROF`: `put_word` @0x3000 settles after 3 of
its 6 `infertypes` passes; installing the type costs one more `mainloop` and one more
`fullloop` (6/6/3 → 8/8/4). `main` @0x26a0, which has no accepted candidate, is
9/9/3 in both arms.

## 3. The install channel

`funcdata.rs (vn_update_type_locked)` — a new wrapper pairing
`Varnode::update_type_locked` with `HighVariable::type_dirty()`, the same pairing
`vn_update_type` already does for the unlocked case. Without the notification
`HighVariable::update_type` returns at its first line and `high_get_type` keeps
handing back the stale type, so `ActionInputPrototype` would print the old
signature. The change is signalled by bumping the action's `count`; `Action::perform`
reads the return value only for `res < 0`.

There was no in-tree precedent for this: the only existing lock of an *input*
varnode runs under `is_input_locked()`, the regime where `update_input_types`
returns early. The proof is the stage test's assertion #2 (`struct_0 *a0` on a
bytechunk function with no declared prototype) and the `fmt` witness. The
decbench-scored JSON surface still reports the parameter at pointer width:
`{"type": "struct_0 *", "kind": "arg", "size": 8}`.

The structure is **completed before** its pointer is taken: `set_fields_struct_raw`
mints a fresh `Rc`, and `merge_test_adjacent` compares high types with `Rc::ptr_eq`.
`assign_raw_fields_struct` is never used — it re-derives offsets with C packing rules
and sizes the structure at 0.

## 4. The decline list

1. not a function input (`param` is the only mode this version ships);
2. the base is type-locked;
3. the base is spacebase-rooted, stack-spaced or persistent — the frame is
   `varmap.rs`'s to lay out and a global's type is program-wide;
4. the base is not already `TYPE_PTR` — pointer-ness is never invented;
5. the pointee is a *named* composite (DWARF, `parse line`, a libc shell, an earlier
   synthesized structure);
6. a non-constant offset term anywhere on the base (an array, TRex §3.3.3),
    whether or not this function dereferences the result — `ls`'s `mpsort`
    passes `&base[n]` to its callee and reads `base[0]`/`base[1]` itself;
7. the base is used as an integer;
8. the base reaches a `MULTIEQUAL`/`INDIRECT` (an induction variable);
9. fewer than two distinct offsets, or no access at offset 0;
10. an offset that is negative or at least `0x8000`;
11. a uniform-width run on a regular grid (Howard §4.5): three or more offsets
    outright, two when the element is narrower than a pointer; a gap does not
    break the run below pointer width (one untouched byte of a buffer used to
    make it a structure) but does at pointer width;
12. the function hit the type-recovery-exceeded flag, or its lattice never settled;
13. the prune (§5) left fewer than two fields — which is how an optimized
    `int fd[2]` declines, its two elements sitting inside one 8-byte
    save/restore (`sort` O2 `rpl_pipe2` @0xe990).

Phis are not peeled at all. Only `COPY`, `CAST` and constant
`INT_ADD`/`PTRADD`/`PTRSUB` are, with a total budget of 8 steps.

## 5. Layout

Widest access wins an offset; no `variable_length` (it makes
`propagate_from_pointer` refuse the pointee and gives `AddTreeState` size 0,
which invalidates every `TypePointerRel`); a field takes the value type when the
widths agree and the type's C spelling is its own width, and `undefined<N>`
otherwise; names are program-wide `struct_N` probed with `find_by_name` and
reused on an exact layout-signature match.

The layout is then pruned to one a C compiler reproduces byte for byte, because
`kuna decompile-project` exports the declaration as source: a width the exported
prelude respells (`undefined3` is a 4-byte `unsigned int` there), an offset its
own alignment does not divide, and an access inside a wider access's bytes are
each dropped. What survives is naturally aligned and 1/2/4/8 wide, and two such
ranges either nest or are disjoint, so the declaration provably cannot overlap.

What survives is then made **dense and C-sized**, which is about the body rather
than the declaration:

* every byte from 0 to the end belongs to a member, a gap becoming an
  `undefined1 field_0x<hex>[N]` of alignment 1 at its own offset. `printc`
  invents the member name `field_0x<hex>` for an in-range offset no field
  covers, while the exported header renders the same gap as
  `undefined1 _pad<hex>[N]`, so the body would name a member the header does not
  declare — and address arithmetic reaches those offsets without a dereference.
* the size is the end of the last surviving field rounded up to the widest
  surviving field's width, with the bytes that rounding adds covered by
  `undefined1` filler like any other gap, because a structure the decompiler
  sizes 0x15 and the compiler sizes 0x18 puts `a0[1]` at two different addresses. Taking the size from the accesses instead — pruned ones
  included — also let one misaligned far access declare a 64 KB type for a
  16-byte object. Past the end the printer spells the address as an element of
  the structure array plus a byte offset, which is still the right address.

Two checks, because the declaration check cannot see the body:
`_Static_assert(offsetof(struct_N, field_0xK) == K)` over every member of every
synthesized structure (0 of 352 fail over 14 exported projects, 2,174 members;
10 of 154 did without the prune), and `gcc -fsyntax-only` over the whole exported
project (0 `has no member named 'field_0x…'` in 14 binaries; 44 before the layout
was made dense).

## 6. Tests and gates

- `tests/stages/structsynth-param-struct.xml` — two-pass, hand-assembled x86-64
  bytechunk, six assertions in the console vocabulary (`int4`/`uint1`).
- `tests/stages/structsynth-overlap-layout.xml` — two-pass, the interior access
  that must not become a field; its assertion #5 (`field_0x4` must not appear)
  fails with `Evidence::prune` ablated.
- `tests/stages/structsynth-hole-member.xml` — two-pass, an offset nobody
  dereferences but the body names anyway; with the hole filler ablated,
  assertions #4 and #5 fail while #3 (the body naming it) still passes, which is
  the bug.
- 16 unit tests in `p5_types/kuna_structsynth/tests.rs` — option parsing, the
  widest-wins rule, the array-shape rule and its gap tolerance, constant sign
  extension, each prune clause, the dense layout, the rounded size, the saved
  `int[2]` pair, and the plateau latch.
- Counters, re-derived on the rebased tree (`counters --rederive`, no drift):
  +1 settable (**216**), tier `transform` **85**, stages corpus **308**, catalog
  rows 215, `source_decompiler: "angr"` **36**,
  `change_kind: "structure-recovery"` **31**, `change_kind: "opt-in-tool"` 21,
  `phase_catalog.json` recaptured, `docs/options.md` regenerated,
  `docs/baseline-stages.json` re-recorded to **1088**.
- `tests/fixtures/list_action_decompile_oracle.txt` re-recorded (two consuming
  tests: `universalaction_listing.rs` and `verify_w8x_allowlist.rs`).

## 7. What this PR deliberately does not do

Nesting (a field that is itself a synthesis base), recursion, subsumptive dedup,
locals and globals (`structsynth all`) and arrays (`structarray`) are each their
own change. Printing the recovered definitions above the function is already its
own option, `structdefs`, and the two compose: `--option structdefs on --option
structsynth param` prints `struct struct_0 { ... };` above the function whose
parameter this pass retyped.

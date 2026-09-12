# Mach-O import-slot binding design

## Compatibility surface

Keep the public option id `peimportcall`, its default, tier, and architecture
field. The option already means "turn a loader-resolved import pointer into an
external reference so `ActionDeindirect` can bind it"; Mach-O direct GOT calls
are the same decision point. `off` must restore the old anonymous slot calls on
both PE and Mach-O.

## Loader fact

Refactor the existing Mach-O indirect-symbol walk to return two additive products:

1. the unchanged import/export inventory used for naming and selection; and
2. exact `[slot, slot + pointer_width)` ranges for valid imported indirect-symbol
   entries in only `S_LAZY_SYMBOL_POINTERS` and
   `S_NON_LAZY_SYMBOL_POINTERS`.

The range is emitted only after the existing validation has rejected LOCAL/ABS
indices, invalid symbol-table indices, empty names, and arithmetic overflow.
Stubs and exports remain inventory entries but never ranges. Lazy-dylib and TLS
pointer sections retain their existing names without being promoted by this
change. Ordinary sections are never entered by the walker.

`ObjectFormat::import_slots` exposes those ranges for Mach-O. The existing
`PeImportCallPass` is registered for PE, COFF, and Mach-O; its PE/COFF arm alone
runs the Win32 no-return list, while all three formats commit their own loader's
slot ranges.

## Verification

- Fact-level unit: exact sole GOT range, explicit non-overlap with
  `__objc_msgrefs`.
- Registration unit: present on PE/COFF/Mach-O and absent on ELF/Wasm.
- Console two-pass test: default binds only the typed slot; option off leaves both
  calls anonymous.
- Stage two-pass extension: the established `ghdec-peimportcall.xml` owns both PE
  and Mach-O arms under one option.
- Promoted CLI probe: hermetic fixture with the ordinary-data negative control.
- Filed dataset acceptance: three-pass replay of the original command.
- Whole-function sweeps: all twelve vendored Mach-O fixtures and every function in
  the 50-function dataset witness, with every changed hunk classified.

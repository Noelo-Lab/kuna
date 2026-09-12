## What

Mach-O direct calls through a typed lazy or non-lazy import-pointer slot now bind
to the import name the loader already registered there. The existing
`peimportcall` option owns the behavior: disabling it restores the anonymous
`(*dat_<slot>)(...)` rendering.

The loader reports only exact pointer-width entries backed by the Mach-O indirect
symbol table. Stubs, exports, LOCAL/ABS entries, lazy-dylib/TLS pointer sections,
and ordinary data such as `__objc_msgrefs` are excluded. The Win32 no-return name
list remains PE/COFF-only.

## Evidence

The filed dataset command now emits seven `objc_msgSend(...)` calls and no
`dat_100004038`, while its two unrelated message-reference calls remain raw.
Three consecutive acceptance replays pass. The hermetic fixture and stage test
pin both option arms and the ordinary-data negative control.

Whole-binary A/B on the 50-function witness changes 27 bodies: 20 application
functions contain 40 exact GOT-slot name bindings and seven import veneers bind
their typed lazy slots. Function and error sets are identical. Across all twelve
vendored Mach-O fixtures, five are byte-identical and every hunk in the other
seven is the corresponding imported symbol binding.

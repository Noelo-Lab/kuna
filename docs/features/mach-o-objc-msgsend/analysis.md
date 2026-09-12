# Mach-O direct import-pointer calls

RE-friction need `mach-o-objc-msgsend`, challenge
`5ab77f5633c5d40ad448c29b`, function `-[CrackMeManager checkCode:]` at
`0x100001d2b`.

## Symptom and discriminator

The loader already names `0x100004038` as `objc_msgSend`, but the instruction
`callq *0x100004038(%rip)` decompiled as `(*dat_100004038)(...)`. The same
function's calls through `__TEXT,__stubs` already rendered by name. Its call
through `0x100004d60`, inside `__DATA,__objc_msgrefs`, correctly remained an
anonymous data-pointer call. The missing fact is therefore neither import naming
nor generic indirect-call recovery: it is the external-reference property on a
typed import-pointer slot.

The hermetic `macho_import_slots` fixture holds both shapes in one function. Its
first call reads the sole eight-byte `S_NON_LAZY_SYMBOL_POINTERS` indirect-symbol
entry (`__DATA_CONST,__got` at `0x100001000`, `_objc_msgSend`); its second reads an
eight-byte word in ordinary `__DATA,__objc_msgrefs` at `0x100002000`. That is the
negative control against painting arbitrary pointer-sized Mach-O data.

## Owning phase and reference behavior

This is P1 program preparation / external refinement. `ActionDeindirect` already
has the downstream behavior: a persistent global target carrying
`Varnode::externref` is resolved against the `FunctionSymbol` registered at the
slot address. Ghidra supplies that property through an external-reference symbol.
kuna's PE `peimportcall` pass supplies the equivalent property-map range over IAT
slots, but `ObjectFormat::import_slots` returned nothing for Mach-O.

The smallest compatible fix extends that existing option and loader interface.
The Mach-O indirect-symbol walk now reports exact pointer-width ranges only for
valid imported entries in `S_LAZY_SYMBOL_POINTERS` and
`S_NON_LAZY_SYMBOL_POINTERS`. It does not report `S_SYMBOL_STUBS`, exports,
`INDIRECT_SYMBOL_LOCAL`/`INDIRECT_SYMBOL_ABS`, lazy-dylib/TLS pointer sections, or
ordinary data. Existing import naming is preserved for every one of those
categories.

The PE-specific no-return-name scan remains structurally inside the PE/COFF match
arm. Mach-O receives only the format-neutral `externref` fact; no Windows API
name is ever applied to it.

## Measured result and wrongness check

The filed acceptance passes three consecutive replays. In the target function,
seven occurrences of `(*dat_100004038)(...)` become seven `objc_msgSend(...)`
calls, while both calls through `dat_100004d60` remain unchanged. With
`peimportcall off`, all seven raw GOT calls return and no `objc_msgSend(` remains.

Whole-binary comparison on the 50-function witness changes 27 bodies: 20
application functions contain 40 exact raw-slot-to-`objc_msgSend` replacements,
and seven named import veneers bind the typed lazy slot they jump through. No
function is added or removed, no decompilation error appears, and line counts are
unchanged except the `memcpy` veneer, whose named return collapses a temporary.

Across all twelve vendored Mach-O fixtures, both arms have identical function and
error sets. Seven images change: six existing import fixtures bind their `printf`
pointer in the import veneer and the new fixture binds `objc_msgSend`; the five
remaining images, including all three Objective-C metadata fixtures and both
Mach-O object files, are byte-identical. Every changed hunk was read and is an
import-slot name binding; no arbitrary data call, export, stub range, LOCAL/ABS
entry, or non-Mach-O image is admitted by the producer.

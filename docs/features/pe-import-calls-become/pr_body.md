## The problem

A PE may place its Import Address Table in an executable/read-only section. In
that shape kuna marked each slot both `readonly` and `externref`, but P3 folded
the read to the bytes stored in the file before `ActionDeindirect` could use the
import identity. Those bytes are a hint/name RVA, not the function address the
loader writes at run time, so `GetDlgItemTextA(...)` became a call through raw
`0x173dc`.

## The fix

`ActionVarnodeProps` now gives the loader's external-reference fact precedence:
an `externref` Varnode skips the whole read-only folding branch, whether folding
was enabled by `readonly`, `dynrelocs`, or `litpoolconst`. The PE loader ranges
and option boundaries are unchanged, so `peimportcall off` still returns the
prior raw indirect call and ARM literal pools still fold normally.

The deterministic 1 KB PE32 fixture places code, three IAT slots, and import
metadata in one RX section. Its console gate proves the raw off arm, the exact
nested `VirtualAlloc(0x40,GetModuleHandleA(0))` default/on result, and named calls
with program-wide `readonly` enabled. The promoted CLI acceptance separately
requires both distinct `GetDlgItemTextA` calls with all arguments and forbids the
raw target RVAs.

## Evidence

On the original `crypto4.exe` witness, a before/after `decompile-all --json
--no-vars` sweep keeps the same 93 functions and zero errors. Forty-three
functions change, all of which previously contained raw folded import calls;
those calls fall from 115 to zero and 78 distinct import names become visible.
The ordinary writable-IAT `pe_imports.exe` control is byte-identical across all
150 functions. One pre-existing imported-call no-return misassociation remains
visible in `sub_4039b2`; unpatched main with `litpoolconst off` has the exact same
truncation, so it is recorded as follow-up rather than folded into this repair.

Acceptance passes 3/3 on the rebased build. Full gate and performance results
are in [record.json](record.json).

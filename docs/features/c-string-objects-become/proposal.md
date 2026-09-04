# ptrdepthcap — the pointer-nesting cap (RE-need `c-string-objects-become`)

Notes kept for the record. The shipped mechanism and its measurements are in
`pr_body.md`; this file records what was *measured* about the cause, because the
need's filed hypothesis was wrong and the difference matters for the next reader.

## The filed hypothesis, refuted

> MSVC STL object layouts and constructor prototypes are not propagated into
> stack locals.

That is not what happens. The defect has nothing to do with the STL, with MSVC,
with layouts, or with constructor prototypes: it reproduces on a 0x131-byte
hand-built x86-64 ELF bytechunk with three unnamed extern stubs and no C++ at
all (`tests/stages/kuna-ptrdepthcap.xml`). What the STL contributes is only the
*idiom*, and any small-string-optimized container has it.

## What was measured

`ActionInferTypes` is capped at seven passes
(`coreaction_render.rs (ActionInferTypes::apply)`). Making that ceiling
settable and sweeping it 1..12 on the witness (`trappy attack.exe`,
`0x1400011c0`) gives the maximum pointer depth in the emitted C:

| ceiling | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 12 |
|---|---|---|---|---|---|---|---|---|
| max `*` depth | 2 | 2 | 3 | 4 | 5 | 5 | 5 | 5 |

One pointer level per pass, exactly. Tracing every candidate type of depth >= 3
that `propagate_type_edge` adopted named the producing op: **1,368 INDIRECT,
958 MULTIEQUAL, 84 PTRSUB, 44 COPY, 42 PTRADD, 4 STORE, 4 INT_EQUAL** — and the
PTRSUBs are all `in0 -> out-1` off `register:0x20` (RSP), i.e. `&stack_local`,
each pass one level deeper than the last.

That is the cycle. The spacebase arm of `propagate_add_in2_out` types
`PTRSUB(spacebase, -0xN)` as a pointer to whatever the mapped local currently
is; the SSO idiom puts that value and a LOAD from the same address on the two
inputs of one MULTIEQUAL; so the local must be both `T` and `ptr(T)`. The
identity-propagating markers (COPY/MULTIEQUAL/INDIRECT) then carry the deeper
type everywhere the object is touched, which is why 2,326 of the 2,504 traced
adoptions are markers rather than the PTRSUB that produces the depth.

It settles at 5 rather than 7 only because the last two passes report no
`write_back` change; there is no principle keeping it at 5.

## Why the fix is one rule and not a C++ type-recovery project

Upstream already has the rule. `TypeFactory::getTypePointerNoDepth`
(`type.cc:1509`) refuses to build a pointer whose target is a pointer-to-pointer
and returns `ptr(undefined<N>)` instead, and it returns an existing
`ptr(undefined<N>)` unchanged — i.e. it is a fixed point. Ghidra calls it from
`TypeOpLoad`/`TypeOpStore::propagateType` only, and kuna's port of that
(`propagate_to_pointer`) is faithful. The escalation here does not go through
LOAD/STORE, so the rule never runs.

`ptrdepthcap` calls the same rule at the one funnel every adopted type passes
through. No new pass, no new pass type, one new module, one gated call site.

## What was NOT done, and why

Recovering `std::string` as a *struct* (a 32-byte record with a union, a size
and a capacity) would be strictly better output and is what the need's title
imagines. It is also a different, much larger feature: it needs an MSVC STL
layout catalogue, a way to recognise an unnamed constructor as
`basic_string::assign`, and a place to attach the recovered record type. That is
the `cppproto`/`cppsig`/`cspecprotos` family's territory. The cap is orthogonal
to it: with the cap on, the object reads `unsigned long long *v21[2]` — the
right *width and shape* for a later struct recovery to replace, rather than a
five-level pointer chain that has to be undone first.

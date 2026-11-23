# Bind Go AMD64 closure context with compiler and instruction evidence

A closure leaf can emit an uninitialized RDX variable in place of its
capture. The [reproduction](analysis.md) shows a signed-word multiply
whose ordinary input and result both use RAX. Bind a typed context at
the named leaf entry when the compiler's debug information and the
instructions agree.

## Named leaf scope

- Refine the existing `godwarfsig` importer for a unique concrete
  little-endian AMD64 Go ABIInternal function. Require one direct
  signed-word capture with compiler offset eight, a matching load,
  multiply and return sequence, the sole post-load RCX location range,
  and complete matching signed-word input and result formals. A closure
  name is a filter. The metadata and instructions establish the layout.
- Use the existing deferred prototype installation and honor explicit
  prototypes and storage assertions. Materialize a context view with the
  entry word and named capture only after establishing the full contract.
  Reserve RDX for the synthetic body-entry parameter and verify the
  ordinary input and result stay in RAX. Extending any source-level call
  signature requires independent evidence of its context at the call.
- Add a source-backed stage with the signed signature, named capture,
  missing metadata, wrong location or displacement, multiple or aliased
  captures, and off-mode controls. Verify the positive against the
  Go 1.27.2 authored leaf and the named reference leaf. Check refusal on
  older, stripped, unoptimized, unrelated and conflicting explicit
  bindings. Compile the emitted bound body against the signed cases and
  measure the default on Go and unrelated inputs.

## Separate invocation scope

Prove both the entry PC and closure context before rendering an indirect
call as a Go function-value call. For the initial zero-argument or unary
shape, require a typed Go function value and an exact pointer-width load
from byte zero of its record. The same reaching record definition must
be in RDX at the call, and the source argument and result storage must
agree on every supported path. Record the validated relationship before
cleanup can discard the RDX copy.

Emission can then evaluate the same typed function value once and pass
only the verified ordinary arguments. Preserve evaluation order, nil
behavior and any independent use of the entry load. Preserve the
existing fallback for unknown arity, user-supplied call contracts,
foreign pointers, ambiguous records, clobbered RDX and different entry
offsets. The reference `applyAll` wrapper also needs an independently
verified slice-input contract. Closure construction, escaping or
mutable environments, and stripped inference require their own proofs.

## Open questions from review

The named-leaf admission requires "a matching load, multiply and return
sequence", which is the body of the `main.scaled.func1` witness (`MOV
RCX,[RDX+0x8]; IMUL RAX,RCX; RET`) rather than part of the closure
contract. Keyed on that sequence, the gate admits the witness and refuses
every closure that does anything else with its capture. Can admission rest
on the compiler's `DW_AT_go_closure_offset`, the ABIInternal closure
register and a load from `[RDX+offset]` alone, with the body left free?

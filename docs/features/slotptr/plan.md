# slotptr — implementation plan

## 1. What is recorded, and when

`restructure_varnode` already folds each pass's stack symbols into the
`framelayout` union (`Funcdata::record_frame_slots`). Right after it, and only
when the per-function seam flag `ArchContext::slot_ptr` is set,
`kuna_slotptr::record_pass` walks the live stack-space Varnodes once and notes
into `Funcdata::slot_evidence`:

- every `(offset, size)` any live stack Varnode occupies (read or written);
- for each written one, the value stored there, as a durable identity:
  - `Null` for the constant 0 (neutral: agrees with any pointer);
  - `Input(addr, size)` for a function input (a parameter's home-slot spill);
  - `Op(seqnum)` for the root op that computed the value, walked back through
    COPY chains (at most 16 hops); an indirect-creation (`INDIRECT(#0, iop)`,
    what a call's killed output register is before return recovery attaches
    it) resolves to the call's SeqNum;
  - `Opaque` for anything else (a non-zero constant: at -O0 the slot below the
    frame is where every call pushes its return address).

INDIRECT and MULTIEQUAL writers are not stores of a new value and are skipped.

The pass that matters is the second `mainloop` iteration: the first one's
`oppool2` turns each spill STORE into a sized stack COPY, the second one's
heritage links it, `restructure_varnode` sees it, and the following `oppool1`
copy-propagates every read away. SeqNums are reissued when the op bank is
rebuilt, so `Funcdata::clear` clears the record.

## 2. How a row is re-spelled

`extract_variables` section 3 appends the filler rows, then
`type_filler_slots_from_stores` visits each one whose byte range overlaps no
other exported row and calls `kuna_slotptr::slot_pointer_type`:

1. No store at exactly `(offset, size)` → keep the width.
2. Any access at another offset or width inside the slot → keep the width
   (a partial store, a narrower read, a wider aggregate copy).
3. Resolve every non-null store in the FINAL function: `Op` by
   `PcodeOpBank::find_op` (must be alive and have an output), `Input` by
   `find_varnode_input`. When the output is an implied temporary whose only
   reader is a CAST (what `ActionSetCasts::castOutput` leaves), take the CAST's
   output. The type is the declaration type of that Varnode's HighVariable
   (`printc::decl_type_representative`), i.e. what the C body declares.
4. Any unresolved store, or two stores whose types print differently → keep
   the width.
5. `admit`: a `TYPE_PTR` of the slot's size whose pointee is neither unknown
   bytes nor code.

## 3. Alternatives measured and rejected

Three ways of reading "the stored value's type" were swept on all 444 slices
(same build, option on vs off):

| variant | perfect | aggregate | improved / worse | ptr_char stack TP |
|---|---|---|---|---|
| A: declared type of the value's HighVariable (shipped rule, `void *` still excluded) | 1,467 | +100.90 | 412 / 5 | 2,133 |
| B: the producing op's own output type | 1,440 | +72.60 | 295 / 4 | 1,969 |
| C: both must agree | 1,439 | +71.46 | 291 / 4 | 1,948 |
| shipped: A, plus `void *` admitted and a pointer to code declined | 1,472 | +103.23 | 418 / 5 | 2,133 |

The base (option off) is 1,353 perfect of 10,748 scored functions.

B and C lose a third of the gain (a call to an unprototyped function returns
`unsigned long` and is cast to the variable's `char *`) and still keep four of
the five worse rows, which come from the body's own pointer-arithmetic typing
rather than from merging.

`void *` was measured as its own value first: it added 5 perfect functions and
6 `ptr_void` true positives, and its only two losses were pointers to code
(`_DT_INIT`, a constant 0x2000 resolved to the ELF init address). The shipped
rule admits `void *` and always declines a pointer to code.

## 4. Why the recording is gated

`framelayout`'s union is recorded unconditionally. This record costs a second
walk of the stack Varnodes per pass plus a COPY-chain walk per store, so it is
gated on `ArchContext::slot_ptr`, copied from `Architecture::slot_ptr` with the
other per-function flags; `slotptr off` does no extra work at all.

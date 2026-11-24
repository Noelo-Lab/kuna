# Infineon Carmel DSP

`Carmel:BE:24:default:default` supports canonical CODE listings and an
executable subset. CODE addresses are 24-bit instruction slots, three bytes
each. A six-byte fetch is either a parallel packet, two independent
sequential instructions, or one wide issue.
Only a sequential right slot is an instruction start. DATA, IO and EXT are
separate 16-bit-word spaces; a CLIW template occupies four 24-bit slots.

```sh
decoder=decompiler/target/release/kuna
export KUNA_SPECS="$PWD/specs"
xxd -r -p tests/fixtures/carmel/compiler-o2.hex /tmp/carmel-o2.bin
"$decoder" disassemble /tmp/carmel-o2.bin 0 --addr --raw-image \
  --target Carmel:BE:24:default:default --base 0 --count 4
"$decoder" decompile /tmp/carmel-o2.bin 0x2e --addr --raw-image \
  --target Carmel:BE:24:default:default --base 0
```

The raw image must use canonical CODE byte order, an even CODE word base and
complete fetches. XML `<binaryimage>` chunks and symbols instead carry byte
offsets: `3 * code_slot`, `2 * data_word`, `12 * template_index`. XML CODE must
be read-only. Set CLIW `readonly="true"` only for an independently established
immutable bank, normalized to template index 0–1023. This explicit mapping uses
the simulator's one-bank index alias; the encoded high index bit remains in
disassembly. A writable, absent, partial or non-normalized bank cannot provide
p-code. Missing memory is not initialized by this language.

Implemented compact effects: exact NOP; INC with low-half accumulator operands;
zero and register-half MOVE; half-register CMP/CMPU and signed full-accumulator
CMP with an inactive peer; signed register-only MUL, MAC and MSU; logical
and arithmetic low-half shifts with an inactive peer; ordinary signed conditional
GOTO with no delay; and unconditional relative GOTO/CALL and RETURN with zero
or one delay issue. Immediate REPEAT handles one register-only following issue,
including zero repeats. It does not lift a transfer or DATA access in that issue.
The explicit post-CALL branch past an unexecuted sequential slot is part of
the caller's control flow, even if analysis found a function entry there.
Unconditional wide relative CALL without delay uses the same two-word DATA
stack link. Marked wide MOVE covers signed literals to accumulator halves,
whole accumulators, and address registers; it also supports short DATA
through `R0` indirect/postincrement and absolute DATA loads into address
registers. Marked MOVU supports a subset of
register and literal sources to whole accumulators or the arithmetic result
sink. Parallel lanes sample old registers and publish in lane order;
competing accumulator writes and two transfers stop lift. An unrecognized
right opcode family stops lift at the start of a sequential fetch without
changing its canonical listing boundaries.

CLIW specialization admits the all-NOP template and a signed MAC1 register
product with the remaining lanes inactive. The multiply uses standard-width
C intermediates while preserving the physical accumulator's guard. Other
forms can be listed but do not lift. In particular the subset has no
general hardware loops, indirect calls, PowerPlug, or cross-issue port
hazard model. An adjacent same-address store/load can read
the old word in the native pipeline but the new word in emitted C.

`carmel.slaspec` is generated from the recovered decoder field descriptions
and independently assembled/native-tested semantic fixtures in the Carmel
reference repository. No vendor tool or reference simulator is used at Kuna
build or runtime. From that checkout, regenerate the source with:

```sh
python tools/carmel_ghidra.py --target kuna isa/carmel-v3.1r2.json \
  ../kuna/specs/Ghidra/Processors/Carmel/data/languages/carmel.slaspec
```

`kuna disassemble` prints CrossView-style expressions, including parallel
lane separators, conditional qualifiers and reserved issues (`???;`).
The listing uses decoded operand fields, not p-code, so an instruction
can have a complete listing while the lifter conservatively returns `unimpl`.
Regenerate the checked-in display projections separately:

```sh
python tools/carmel_kuna_listing.py \
  ../kuna/decompiler/crates/kuna-console/src/engine/kuna_carmel_listing/decode/tables.rs
```

The ignored Rust `verify_carmel_disasm_parity::sweep` test consumes private
JSONL produced by `tests/carmel_kuna_disasm_corpus.py` in the reference
checkout. It compares exact text, original CODE addresses and consumed
words against the original CrossView and the Python formatter.

The reference checkout's `carmel_kuna_sim_inventory.py` and
`carmel_kuna_sim_merge.py` index the native simulator observations by pytest
item, oracle call, and case occurrence. The emitted-C runners verify a native
return boundary and compare the compiled C with the DLL at `-O0` and `-O2`.
The current run matched 195 of 3,585 observed case occurrences. Two additional
cases match the original static decode rejection. Nineteen more truncate at
the same address but are only unimplemented lifts. The full simulator suite
does not pass; neither a matching listing nor a truncated lift is a C pass.

The [fixture corpus](../../../../tests/fixtures/carmel/README.md) records the
compiler and locator identities, fixed entry addresses, ABI evidence and
bounded inputs. Compiler support is limited to the observed simple scalar
argument and result convention; the compiler vectors exercise deliberately
unimplemented instruction families as well.

# Infineon Carmel DSP

`Carmel:BE:24:default:default` supports canonical CODE listings and an
executable subset. CODE addresses are 24-bit instruction slots, three bytes
each. A six-byte fetch is either a parallel packet, two independent
sequential instructions, or one wide issue.
Only a sequential right slot is an instruction start. DATA, IO and EXT are
separate 16-bit-word spaces; a CLIW template occupies four 24-bit slots.

Build Kuna and compile the specification as shown below before running this
example from the repository root:

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
immutable bank, normalized to template index 0–1023. In this mapping the
encoded high index bit does not affect the template lookup; it remains in
disassembly. A writable, absent, partial or non-normalized bank cannot provide
p-code. Missing memory is not initialized by this language.

TASKING's IEEE-695 `.obj` and unlocated `.out`, and its IEEE-695 and
TIOF absolute outputs, load without `--raw-image`. For example:

```sh
"$decoder" decompile tests/fixtures/carmel/containers/caller-linked.ieee695 _call_addone
```

Absolute modules retain locator addresses. Relocatable modules use a synthetic
layout and report unsupported relocations rather than guessing. Motorola
S-record and Intel HEX exports can be loaded with `KUNA_TASKING_CARMEL_CODE=on`
only when they contain instruction words in CODE, with no auxiliary banks or
ROM initialization data. See the [CLI reference](../../../../docs/cli.md#tasking-carmel-images).

Implemented compact effects: exact NOP; INC with low-half accumulator operands;
zero and register-half MOVE; half-register CMP/CMPU and signed full-accumulator
CMP with an inactive peer; signed register-only MUL, MAC, MSU, and DIV;
logical and arithmetic low-half shifts with an inactive peer; assigned
ADDRUPDATE pointer pairs; and ordinary signed conditional GOTO with no delay.
Selected compact long ADD, SUB, and multiply forms preserve status and
two-word DATA results with an inactive peer. Qualified CLRBTR clears the
selected backtrace port only when its predicate allows it.
Compact unconditional and wide relative GOTO execute up to three following
issues, counting sequential slots individually. For the qualified wide form,
the address-enable bit can admit a transfer even when the predicate is false.
Compact CALL and RETURN admit zero or one delay issue. Immediate REPEAT handles
one register-only following issue, including zero repeats. REPEAT does not lift a
transfer or DATA access in that issue.
The explicit post-CALL branch past an unexecuted sequential slot is part of
the caller's control flow, even if analysis found a function entry there.
Unconditional wide relative CALL without delay uses the same two-word DATA
stack link. Marked wide MOVE covers signed literals to accumulator halves,
whole accumulators, and address registers; it also supports short DATA
through `R0` indirect/postincrement and absolute DATA loads into address
registers. Wide LOAD covers selected accumulator, pointer and configuration
destinations, including immediate pointer setup that leaves a bound intact.
Selected wide address transfers add the immediate displacement to a word
source, including saved CODE addresses and the `R3` address unit.
Selected CE2 logical shifts use configuration-word sources and preserve
the destination on a veto. An unconnected configuration input stops before issue.
Other wide effects include selected EXTRACTS, EXTRACTZ, INSERT, DIVIDE,
divide-with-trace, logic and unary arithmetic routes. Unqualified
BANKXCHG exchanges selected pointer or accumulator banks. Predicated logic
uses incoming CE feedback; the language does not model its general publication.
Marked MOVU supports some register and literal sources to whole accumulators
or the arithmetic result sink. Parallel lanes sample old registers and publish in
lane order. Competing accumulator writes and unproved dual transfers stop
lift. Compact dual direct transfers with fixed-result selectors can lift when
both destinations are backed and their potential delay peers have bounded
register effects.
Unrecognized sequential right families and reserved compact operands and
wide EXTRACT descriptors reject lift without changing canonical listing boundaries.

CLIW specialization admits the all-NOP template, a signed MAC1 register
product, and covered register ALU operations with all other lanes inactive.
The arithmetic uses standard-width C intermediates while preserving the
physical accumulator's guard. Other forms can be listed but do not lift.
The subset has no general hardware loops, indirect calls, PowerPlug, or a general cross-issue
port model. A directly adjacent wide immediate store and absolute load
are lifted as a pair so the load reads before the store writes, while the
load remains an independently enterable issue. A delayed transfer still
executes only its prescribed issues. A separated same-port collision is not
lifted faithfully by this model.

`kuna disassemble` prints CrossView-style expressions, including parallel
lane separators, conditional qualifiers and reserved issues (`???;`).
The listing uses decoded operand fields, not p-code, so an instruction
can have a complete listing while the lifter conservatively returns `unimpl`.

Run the checked-in p-code and raw CODE fixtures from the repository root:

```sh
cargo build --manifest-path decompiler/Cargo.toml --release -p kuna-slacomp -p kuna-cli
decompiler/target/release/slacomp specs/Ghidra/Processors/Carmel/data/languages/carmel.slaspec
cargo test --manifest-path decompiler/Cargo.toml --release -p kuna-sleigh --test integration carmel_pcode
cargo test --manifest-path decompiler/Cargo.toml --release -p kuna-console --test integration verify_carmel_raw
```

The [compiler and assembler fixtures](../../../../tests/fixtures/carmel/README.md)
include their source, linked CODE, entry points, and bounded inputs.
These tests cover the implemented subset, not the full instruction set.

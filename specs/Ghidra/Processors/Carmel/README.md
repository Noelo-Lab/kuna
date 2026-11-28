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
Compact half-register INC and MOVE publish the predicate rails sampled before
the issue updates either lane's status flags. The following issue's feedback
uses the new flags.
Compact signed literal MOVE writes `R` or `RN` when its lane predicate or
address enable is set. A following `MOVE A2,*R1++RN1` reads DATA at the old
`R1` and uses the new `RN1` to postmodify the pointer.
Emitted C keeps the returned pointer register in DATA word units even
when that same address has a named DATA object.

Qualified compact `MOVU` to a modulo register uses early address admission:
the parallel left and sequential right forms use CE1, and the parallel
right form uses CE2. The modulo value and alignment update together,
independently of the late execution predicate.
Selected compact long ADD, SUB, and multiply forms preserve status and
two-word DATA results with an inactive peer. Qualified CLRBTR clears the
selected backtrace port only when its predicate allows it.
Compact unconditional and wide relative GOTO execute up to three following
issues, counting sequential slots individually. For the qualified wide form,
the address-enable bit can admit a transfer even when the predicate is false.
Unqualified wide relative GOTO and compact relative GOTO with no arithmetic
peer publish condition feedback across all three delay clocks, including
idle clocks that do not execute a CODE issue.
The undelayed wide `GOTO NC1` tests the incoming carry flag.
Unconditional wide `GOABS A0` samples the accumulator's low 32 bits before
retiring up to five delay issues. The guard byte does not enter the target.
Compact unconditional `GOABS (R0)` with an inactive right lane reads a
32-bit target from two DATA words and admits up to three delay issues.
The unconditional wide `CALLABS (R0)` reads its target from two DATA words.
It retires at most five delay issues but saves the link from the encoded
delay, including when the encoding requests six or seven.
Compact unconditional `CALLABS` with an inactive right lane takes its target
from `(R0)` or the low 32 bits of `A0`, with up to three delay issues.
Undelayed `CALLABS A0 CE1` can transfer with address enable alone, but
only execution enable writes the two stack link words.
Compact `RETI` with an inactive right lane admits up to three delay issues
and has an undelayed CE1 form. It pops the active trap source if present;
otherwise it clears bit four of `PSTATUS`.
Guarded `RETURN EQ1` and `RETI EQ1` retire up to six delay issues on both
paths; only a taken return reads the link or retires a trap source.
Unconditional compact CALL admits up to three delay issues in sequential
form or with an inactive parallel peer. Compact `CALL NC1` with an inactive
right lane retires at most two delay issues. The sequential compact and
marked wide `CALL NC1` with encoded delay three also retire two issues.
These forms test carry before the delayed issues, write the encoded link
on both paths, and decrement `SP` only when they take the call. Taken calls
return to the encoded link; vetoed calls continue after the retired delays.
Undelayed wide relative `CALL CE1` and `CALL C1 CE1` require proved early
address admission. With admission but a late CE veto, both redirect
without writing the link; only the unconditional form decrements `SP`.
With late CE enabled, the conditional form writes the link even if carry
vetoes the transfer and `SP` update.
Compact RETURN admits two or three delay issues with an inactive right lane,
and an undelayed CE1 form whose address enable can override a veto.
Immediate REPEAT handles one register-only following issue, including zero
repeats. For wide CE1
REPEAT, a veto with address enable clear executes that issue once; otherwise
the immediate count applies. REPEAT does not lift a transfer or DATA access
in the repeated issue.
Unqualified immediate REP with no arithmetic peer also publishes condition
feedback during an annulled body's idle clock.
Selected dynamic REPEAT forms sample `A0L`, a postmodified `R1`, an absolute
DATA word, or a literal before three setup issues. A zero count skips only
the first body issue. The direct DATA and literal forms publish condition
feedback on that idle clock and may replay a range on a positive count.
Setup and body must be register-only and contain no transfers.
An explicit post-CALL branch to the encoded resume word remains part of
the caller's control flow, even if analysis found a function entry there.
Unconditional wide relative CALL retires at most three following issues.
Its two-word DATA stack link uses the encoded delay, even for delays four
through seven. Marked wide MOVE covers signed literals to accumulator halves,
whole accumulators, and address registers; it also supports short DATA
through `R0` indirect/postincrement and absolute DATA loads into address
registers. Wide LOAD covers selected accumulator, pointer and configuration
destinations, including immediate pointer setup that leaves a bound intact.
Marked wide `SWAP` exchanges one register word with absolute or indirect
DATA. An unbound first address selector uses DATA word `0xffff`, not the
literal field. An indirect long selector sets the pointer stride but
still exchanges only one word at the original address.
A DATA selector in the second operand can advance its own pointer; it
does not read a second DATA word. Two updates to the same pointer are
not lifted.
Selected wide address transfers add the immediate displacement to a word
source, including saved CODE addresses and the `R3` address unit.
Selected qualified `TRANSFER` and `NOT LC1,*R3++` lifts require a proved
earlier address-admission sample on every incoming path. The later result
enable does not reopen an earlier veto. A direct entry or opaque transfer
does not establish the missing history.
Selected CE2 logical shifts use configuration-word sources and preserve
the destination on a veto. An unconnected configuration input stops before issue.
Covered wide signed/unsigned multiply-subtract forms select the addend
from the result bank and keep the first multiplier's `X1` latch.
The full-width wide MAC flag sink accumulates from a constant -1 without
reading DATA. Selected wide MAC, MACSU, MSU and MACR forms instead
read and write the single fallback DATA word at `0xffff`; the encoded
literal does not address that word.
Other wide effects include selected EXTRACTS, EXTRACTZ, INSERT, DIVIDE,
divide-with-trace, logic and unary arithmetic routes. Unqualified
BANKXCHG exchanges selected pointer or accumulator banks. Predicated logic
uses incoming CE feedback; the language does not model its general publication.
Wide `DIVTRACE` routes distinguish signed short and long DATA inputs
and select the trace bank independently of where the remainder is written.
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

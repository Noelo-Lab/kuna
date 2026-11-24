# Infineon Carmel DSP

`Carmel:BE:24:default:default` is an initial processor subset. CODE addresses
are 24-bit instruction slots, three bytes each. A six-byte fetch is either a
parallel packet, two independent sequential instructions, or one wide issue.
Only a sequential right slot is an instruction start. DATA, IO and EXT are
separate 16-bit-word spaces; a CLIW template occupies four 24-bit slots.

```sh
decoder=decompiler/target/release/kuna
export KUNA_SPECS="$PWD/specs"
xxd -r -p tests/fixtures/carmel/compiler-o2.hex /tmp/carmel-o2.bin
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
zero and register-half MOVE; unconditional relative GOTO/CALL and RETURN with
zero or one delay issue. Marked wide MOVE covers signed literals to accumulator
halves or short DATA through `R0` indirect/postincrement. Parallel lanes sample
the old register state and publish in lane order; competing accumulator writes
and two transfers stop lift. CLIW specialization admits the all-NOP template
and a signed MAC1 register product with the remaining lanes inactive. Other
forms retain framing and raw operands, but do not lift. In particular there is
no hardware-loop, general DATA, indirect call, wide direct call or PowerPlug
support, and the subset does not model general cross-issue port hazards.

`carmel.slaspec` is generated from the recovered decoder field descriptions
and independently assembled/native-tested semantic fixtures in the Carmel
reference repository. No vendor tool or reference simulator is used at Kuna
build or runtime. From that checkout, regenerate the source with:

```sh
python tools/carmel_ghidra.py --target kuna isa/carmel-v3.1r2.json \
  ../kuna/specs/Ghidra/Processors/Carmel/data/languages/carmel.slaspec
```

The [fixture corpus](../../../../tests/fixtures/carmel/README.md) records the
compiler and locator identities, fixed entry addresses, ABI evidence and
bounded inputs. Compiler support is limited to the observed simple scalar
argument and result convention; the compiler vectors exercise deliberately
unimplemented instruction families as well.

# Indexed byte stores and constant stack reads

The authored `scan.s` zeros a stack word, fills its first three bytes through
an incrementing pointer and directly reads the first byte. A nonzero byte
must reach `helper`. The load-only heritage policy lets the initializer flow
across the STORE unchanged, so the call disappears. The original emitted C
fails an executable regression; broad store guards preserve the call.

`stackstoreguard` adds the missing possible-write fact for this bounded case:
a stack range with a constant write, and byte STOREs already recorded by
indexed stack-pointer discovery. The initializer check follows COPY,
SUBPIECE and integer extensions for at most 16 links. It runs before partial
reads are widened to the heritage range. Existing INDIRECT construction
preserves the store ordering; no stored value is guessed.

The option defaults on. `stackstoreguard off` restores the previous load-only
policy. `indexaliasguard off` suppresses it, and `full` retains broad store
guards irrespective of this option. Wider stores, uninitialized locals,
unknown pointers and global aliases are outside the new policy. Conservative
guards do not establish disjointness or solve general memory aliasing.

Broadening the default to every stack STORE changed parameter and aggregate
recovery in existing stages. This patch limits the fix to initialized byte
fills. The public tests cover direct byte and word reads, a second overlapping
byte, disjoint slots, direct stores, a non-stack pointer and a later overwrite.
GCC and Clang execute the printed C at O0/O2 over 260 inputs per case and
compare helper-call counts with independent fixture semantics. The explicit
full policy is also executed on the three positive cases. This checks control
flow; it is not a claim of complete type or alias recovery for arbitrary code.

The feature uses only authored synthetic bytes. `scan.s`, the stage and the
Rust integration fixture are covered by the repository license. No external
binary or decompiler oracle is required. See `comparison.txt`, `corpus.diff`
and `record.json` for captured output and validation measurements.

## Output review

All 83 original and 287 existing stage files were captured before and after.
Their decompiled C is identical; the catalog stage changes only by adding the
new option. The new stage adds three assertions. Existing assertion records
are unchanged. An additional off/on `decompile-all` comparison covers 59
functions in `arm_thumb_linked_le32`, `splitstorekeep_x86_64` and
`castwiden_gcc_O0_x86_64`, with no missing bodies or changed functions.
These existing authored fixtures retain their provenance in the analysis
fixture README.

Captures omit trailing whitespace on blank lines; the diff uses zero context.
The changed synthetic body in `corpus.diff` retains the four initializer
bytes and adds the byte-dependent return guard and helper call. Its copy
loop is unchanged. Each change follows the assembly: zero the word, copy
three bytes, then branch according to the first byte. There is no comparison
against a proprietary or external decompiler oracle.

## Validation and cost

Original parity: 675/675. Stage parity: 1,598/1,598, including three new
assertions. Catalog, strict spec, generated options, shared counters, lint,
42 tooling tests, 274 CLI cases, 56 Ghidra tests (including the normally
ignored breadth case), and the wasm32-wasip1 release check pass.

The full workspace reports 7,562 passed, one failed and 38 ignored. The failure
is `an_implied_widening_round_trips_through_the_printed_c`: GCC rejects scalar
pointers passed to an emitted byte-pointer parameter. The same public fixture,
driver and baseline binaries independently reproduce that compile failure.
The workspace is therefore not fully green; this change does not fix that
separate emitter defect.

An interleaved timing run after compilation, with four warmups and 61 samples
per setting, measures 18.5658 ms off versus 18.9740 ms on (+2.198%, within the
5% budget). Each run checks whether the helper call is present as expected.
This is CLI wall time for the authored witness, including startup/loading;
it is not an isolated engine-cost or general throughput claim. Raw samples
are retained in `record.json`.

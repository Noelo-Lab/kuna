# Source-authored callwide fixtures

These source-authored inputs exercise sealed MIPS32-BE `callwide` admission.
ARM inputs are refusal controls: enabling `callwide` must preserve their complete
function records, text, project C and header. Native ARM validation does not
establish caller-closure proof or extend the option's supported architectures.
The generator emits reproducible commands without executing a toolchain.

`arm32_be.S` and `mips32_be.S` define one untyped `produce_sum`. The same producer
has a full-width typed-forwarding caller, high-word32/16/8/boolean callers, and
an ignored-output caller. The boolean result means high32 != 0. The narrow
callers are not separate narrow producers. High16/high8 preserve the low16/8
bits of the big-endian high word; vectors distinguish them from the low return
word and from nonzero-boolean normalization. The ignored caller saves its own
input pair in its private frame before the call, then overwrites both producer
return words with that unrelated pair before any read. Its result is its own
left input's high word, which the vectors distinguish from the producer's sum.

The ARM control uses A32 ARMv5TE with no Thumb, floating point or conditional
control. Boolean normalization is CLZ/LSR/EOR. The MIPS
fixture uses fixed-width MIPS32-BE o32 without PIC/abicalls: low-word addition
feeds the carry, high-word completion occupies the producer's RETURN delay
slot, and the low sink-argument copy occupies the sink CALL delay slot. Every
nonleaf assembly caller has a 24-byte private frame with the entry RA saved at
SP+20 and SP restored in the RETURN delay slot. The ARM frame is 16 bytes with
LR at SP+12. Positive assembly stores touch only these private frames.

`consumers.c` is a separate translation unit. It defines the one-formal
eight-byte sink, a two-four-byte-formal negative control and a sink with the
wide formal at the next ABI-aligned ordinal. The positive sink XORs its two
words with distinct immediate constants; `identity-sink` is a separate control
that returns its argument unchanged. It never declares or references
`produce_sum`. In `consumer-dwarf` mode, additional straight-line contract
functions reference typed extern declarations of the assembly callers, so
their narrow return contracts can be retained in consumer-TU DWARF without
address tables. Actual declaration DIEs, ABI storage and provenance MUST be
checked after building: an unretained declaration is missing proof, not a pass.
These C helpers also need full image closure/return-token validation; their
compiler-generated saves must not be silently exempted from the proof.
The positive consumer-DWARF commands explicitly select `narrowext compiler`
on both sides of the `callwide` ablation. MIPS's default `narrowext abi` and
`off` do not establish extension of newly declared narrow results; those arms
must refuse without changing the complete records or exports.

The generator only emits source copies, a linker script, assertions and an
argument-array command manifest. It does not discover or invoke compilers,
linkers, emulators or kuna. For example:

```sh
python3 decompiler/crates/kuna-cli/tests/fixtures/callwide/generate.py \
  --out /tmp/callwide-mips --arch mips32-be --case positive --mode declared \
  --cc clang-14 --ld ld.lld-14
```

`declared` mode builds all target assembly and consumers without DWARF;
its `assertions.txt` locks the sink and caller contracts but leaves the producer
untyped. `consumer-dwarf` mode still builds target assembly with `-g0`, compiling
only consumers/contracts with `-g -gdwarf-4`; use that image for supported
parallel/streamed runs without assertions. Do not use `assertions.txt` to mask
missing DWARF in that acceptance arm. A separate declared-producer control may
add its correct eight-byte prototype. `declared-narrow` instead deliberately
locks its narrow output and must refuse promotion.

## Independent execution and actual-C replay

The proof executable contains only target assembly and the consumer TU. MIPS
appends the 16-byte `callwide_entry` after all existing consumer/helper code:
`jal caller64; nop; j callwide_entry+8; nop`. Its ELF entry is that sealed root;
it calls once and loops at the return continuation. Existing function addresses
and bytes are preserved. `missing-root` retains the original returnable entry
as an explicit refusal control. ARM remains outside `callwide` admission.
`oracle.c` and `start_*.S` are linked ONLY into a separate runtime executable:
Linux exit syscalls, oracle branches and its data must never enter the admitted
image. The runtime link reuses the exact fixture/consumer objects at the same
fixed addresses. Before QEMU execution, run the generated mandatory check:

```sh
python3 decompiler/crates/kuna-cli/tests/fixtures/callwide/generate.py \
  --verify /tmp/callwide-mips/proof.elf /tmp/callwide-mips/runtime.elf
```

This requires identical `.text.callwide` bytes/address, including padding and
literal pools, and every target function's address/size/hash. A mismatch or
unsupported ELF invalidates native evidence. Run only the positive/control
runtime image; malformed-RA/opaque/indirect refusal images are not safe runtime
oracles. Exit codes 1–8 identify full64/high32/high16/high8/bool/ignored and
the DWARF-only wide/narrow composition helpers; 9 identifies a direct producer
call with its four raw word arguments. Eight source-defined vectors
include distinct halves, low-word carry, full overflow and nonzero upper
padding. A separate harness function fills stack storage before the calls so
uninitialized padding cannot pass merely because stack pages start at zero.

The primary ABI replay cross-compiles untouched whole-output C and project C
with their actual generated header for MIPS, using the generator's compiler,
linker and separate runtime harness settings, then executes both at `-O0` and
`-O2` under QEMU. This checks the generated formal grouping and return layout
against the target ABI. The original native image remains a separate oracle.
The CLI test packages the vendored target section verbatim with `.incbin` and
its original function symbols, then links only the separate harness. The full
RX and per-function identity check above runs before each native execution;
the target code is never recompiled or rewritten for this check.

Selected-function replay keeps those same target bytes and gives every original
function a `callwide_native_` symbol at its original address and size. Unselected
functions also retain their public aliases; the selected public symbol belongs
to the untouched emitted C in a separate executable segment. The oracle uses
the actual selected header, declaring additional native functions only when
that header omits them. The selected C receives no supplemental declarations.
Before each `-O0`/`-O2` QEMU run, `generate.py --verify-selected PROOF RUNTIME NAME`
checks the complete target RX bytes, every prefixed function's address/size/hash,
unselected aliases, and separate ownership of the recompiled selected function.
The oracle calls each selected public symbol directly, including the producer;
native callers' encoded calls to the original producer cannot stand in for that
direct check.

Host replay also compiles the actual C AND its actual generated header
with the fixture oracle at both `-O0` and `-O2`, GCC and Clang when installed:
define `CALLWIDE_HOST_REPLAY` and `CALLWIDE_EMITTED_HEADER` to the quoted generated
header path. That mode uses the generated caller declarations instead of this
fixture's hand-written externs. Supply only established byte/CONCAT helpers;
do not replace producer/caller bodies, declarations or return types to repair
the result. Preserve observed target-byte hashes with those replay results.

## Refusal matrix and integration plan

Every negative requires complete off/on equality of generated code, header,
inventory and relevant metadata for the affected producer and callers. A
successful MIPS positive needs a durable verdict/firing witness, producer eight-byte
output in ABI order, and all six same-producer callers passing native and
actual-C/header replay; mere absence of an error is insufficient.

| Cases | Required refusal evidence |
|---|---|
| ARM inputs in both metadata modes | Complete OFF/ON function records, text, project C and header remain identical. |
| `mixed-refusal` | The original producer promotes while appended `produce_rejected`, its swapped forwarding caller and its declaration-retention helper keep exact OFF/ON records and header prototypes. |
| `swapped`, `partial`, `arithmetic`, `spill` | Typed sink does not receive the exact unchanged ordered register pair. |
| `two-formals`, `wrong-slot`, `unrelated` | Two inferred/declared words, wrong formal ordinal or an unrelated constant pair cannot supply positive proof. |
| `intervening-call`, `branch`, `opaque` | The local walk crosses a call, branch or opaque operation. |
| `stale-word` | The declared typed sink cannot justify a producer entry-leftover word. |
| `low-only`, `late-low-only` | An unsupported low-only caller vetoes the SAME producer globally; the latter puts it at producer site 33 after 32 otherwise allowed sites. |
| `site-limit` | All 257 sites are considered; exceeding 256 refuses globally, without sampling. Extra sites receive their own narrow contracts in both modes. |
| `address-table`, `exported`, `declared-narrow` | Stored producer pointer, public/exported candidate or declared narrow output overrides inferred width. |
| MIPS `missing-root`, `ignored-zero` | The returnable ELF entry lacks the sealed nonreturning root; `ignored-zero` also retains the original literal-zero dead-result caller. |
| MIPS `jalr`, `pointer-ra`, `constant-ra` | Indirect calls or a JR-ra whose effective token is not entry RA refuse the image. |
| MIPS `wrong-ra-slot`, `alias-ra-slot`, `bad-sp-delay` | Wrong token cell, overwritten saved token or incomplete SP restoration after the return bundle refuses. |
| MIPS `unknown-store`, `pic`, `reserved-gap` | Nonprivate store, PIC/abicalls, or a reserved executable-gap word prevents image admission. Confirm the selected reserved word is actually unsupported by the decoder before claiming that witness. |

Inspect compiled instructions/ELF flags, exact site counts,
delay bundles, consumer DWARF/ABI storage and the complete code ownership model.
Do not weaken closure to accommodate compiler output. Unsupported instructions,
unexpected external helpers, missing definitions or absent debug declarations
mean the positive fixture is not yet admitted and needs correction, not a skip.

`tests/call_wide.rs` covers MIPS serial text/JSON, whole and selected orders,
actual project header/body, parallel/streamed consumer-DWARF runs, default-OFF
and legacy-mode identity, missing-root refusal, and ARM refusal. Its compiler
checks use actual header typedefs to require four four-byte producer formals
and two eight-byte caller formals, along with the exact return widths. The
consumer-DWARF arm supplies no prototype assertions. Epoch and lifetime
adversaries belong in the P4 and frontend unit controls.
Existing OFF output can infer different raw 32-bit argument signs or omit
arguments across rendering orders and worker shards. Positive checks therefore
keep the producer join, ABI widths and generated header contracts fixed while
executing each actual order/parallel/stream variant. Emitted C and headers are
preserved verbatim. Refusal records and exports still require exact identity.

The CLI suite also patches exact, asserted instruction words within the vendored
MIPS function symbols to exercise swapped/partial/stale/unrelated pairs, two
formal arguments, the wrong wide ordinal, a low-only caller, a wrong saved-RA
cell and incomplete SP restoration. Both metadata modes require complete
OFF/ON function-record identity after each mutation, with an unchanged positive
firing beside them. An undecodable OFF baseline fails the test; it is not counted
as refusal evidence. These controls require no cross toolchain.

The mixed fixture appends its second producer, caller and consumer-TU helper
after the sealed root in `.text.callwide.mixed`. Existing target bytes and
function addresses stay unchanged. The new caller's declaration is retained
without a producer declaration; it shares the original typed sink with the
accepted path. Native checks exercise the original eight-vector paths and
helpers, without treating the intentionally swapped path as a valid wide proof.

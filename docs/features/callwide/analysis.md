# Callwide: caller evidence for a big-endian register-pair return

Implementation admission note: the approved first MIPS route now requires a
sealed process entry, as specified in [the proposal](proposal.md). The baseline
returnable ELF below remains a refusal control. The independent hand-authored
acceptance fixture preserves its own returnable version's producer, callers and
XOR consumer byte-for-byte when adding the sealed entry. This
proves the executed caller set, not absence of integer values equal to code
addresses. Initial admission is MIPS-only; ARM is an unchanged refusal control
because its existing caller scan does not prove this complete transfer set.
Explicit-declaration and strict-DWARF MIPS replays pass unchanged generated
C/header checks, including target emulation and selected-project declarations.
Validation on merged base `74d0bab7238ceacccedc6d1d888d56762b9f71f2`
passes all four gates, all seven CLI acceptance tests and seven firing stage
assertions. The option remains off in every mode, and #904 remains open.

On `6d7ab7fff8092b6f154f5aed3cdf2a98701d5d6a`, an untyped MIPS32 big-endian
producer loses its low return word even when a caller forwards both words to
one declared 64-bit parameter. The generated project header declares the
producer as `int`; its caller treats the same call as an eight-byte value.
Adding the producer's own 64-bit prototype repairs the output. Merely enabling
`bejoin on` does not retain the missing word.

This is one remaining positive-evidence family from
[issue #904](https://github.com/Noelo-Lab/kuna/issues/904).
Declared and linked-DWARF eight-byte outputs already work with `bejoin off`.
Ambiguous instructions alone cannot prove the width: PPC and ARM compile a
64-bit sum and its high-word-only truncation to identical producer bytes.

## Self-contained reproduction

Run Bash from the repository root with built kuna/specs, Clang 14 and LLD 14. The
source contains both the positive producer and a narrow negative control; no
producer prototype is supplied to kuna in the failing arm.

```sh
mkdir -p /tmp/kuna-callwide-repro
cat > /tmp/kuna-callwide-repro/caller.c <<'C'
typedef unsigned long long u64;
typedef unsigned int u32;
__attribute__((noinline)) u64 produce_sum(u64 a,u64 b) { return a+b; }
__attribute__((noinline)) u64 consume64(u64 x) { return x^0x1122334455667788ULL; }
__attribute__((noinline)) u64 caller(u64 a,u64 b) { return consume64(produce_sum(a,b)); }
__attribute__((noinline)) u32 produce_high(u64 a,u64 b) { return (a+b)>>32; }
__attribute__((noinline)) u32 consume32(u32 x) { return x^0x11223344U; }
__attribute__((noinline)) u32 caller32(u64 a,u64 b) { return consume32(produce_high(a,b)); }
C
CALLWIDE_LD="$(command -v ld.lld-14 || command -v ld.lld)"
clang --target=mips-linux-gnu "-fuse-ld=$CALLWIDE_LD" \
  -march=mips32 -mabi=32 -fno-pic -mno-abicalls \
  -O2 -fno-inline -fno-optimize-sibling-calls -nostdlib -no-pie \
  -Wl,-Ttext=0x100000,-e,caller /tmp/kuna-callwide-repro/caller.c \
  -o /tmp/kuna-callwide-repro/mips.elf
CALLWIDE_ARGS=(--sleighpath "$PWD/specs" --option dwarf off \
  --option bejoin evidence \
  --assert 'prototype consume64 unsigned long long consume64(unsigned long long x)' \
  --assert 'prototype consume32 unsigned int consume32(unsigned int x)' \
  --assert 'prototype caller unsigned long long caller(unsigned long long a,unsigned long long b)' \
  --assert 'prototype caller32 unsigned int caller32(unsigned long long a,unsigned long long b)')
decompiler/target/release/kuna decompile-project \
  /tmp/kuna-callwide-repro/mips.elf -o /tmp/kuna-callwide-repro/before \
  "${CALLWIDE_ARGS[@]}"
```

The producer at `0x100000` computes both words:

```asm
addu $1,$6,$4
addu $3,$7,$5
sltu $2,$3,$7
jr   $ra
addu $2,$1,$2       # return delay slot
```

The caller copies `$2:$3` to the declared sink's `$4:$5`, including the second
copy in the sink CALL's delay slot. Default C instead contains:

```c
int produce_sum(int a0,int a1,int a2,unsigned int a3)
{
  return a2 + a0 + (unsigned int)(a3 + a1 < a3);
}
/* caller: */
v1 = produce_sum((int)(a >> 0x20),(int)a,(int)(b >> 0x20),(unsigned int)b);
return consume64(CONCAT44((int)v1,(int)((unsigned long long)v1 >> 0x20)));
```

Replay the actual generated C and header. The helper only supplies kuna's
concatenation operation; it does not repair the output.

```sh
cat > /tmp/kuna-callwide-repro/support.h <<'C'
#include <stdint.h>
#define CONCAT44(a,b) (((uint64_t)(uint32_t)(a)<<32)|(uint32_t)(b))
C
cat > /tmp/kuna-callwide-repro/replay.c <<'C'
#include <stdio.h>
#include "mips.elf.h"
int main(void) {
  unsigned long long a=0x100000002ULL,b=0x300000004ULL;
  unsigned long long expected=(a+b)^0x1122334455667788ULL;
  unsigned long long got=caller(a,b);
  printf("expected=%016llx got=%016llx\n",expected,got);
  return got!=expected;
}
C
cc -O2 -fwrapv -include /tmp/kuna-callwide-repro/support.h \
  -I/tmp/kuna-callwide-repro/before \
  /tmp/kuna-callwide-repro/before/mips.elf.c \
  /tmp/kuna-callwide-repro/replay.c -o /tmp/kuna-callwide-repro/replay
/tmp/kuna-callwide-repro/replay
# expected=112233405566778e got=1122334055667788; exit 1
```

A supplied producer prototype is a positive control, not the proposed
implementation:

```sh
decompiler/target/release/kuna decompile-project \
  /tmp/kuna-callwide-repro/mips.elf -o /tmp/kuna-callwide-repro/typed \
  "${CALLWIDE_ARGS[@]}" \
  --assert 'prototype produce_sum unsigned long long produce_sum(unsigned long long a,unsigned long long b)' \
  --assert 'prototype produce_high unsigned int produce_high(unsigned long long a,unsigned long long b)'
cc -O2 -fwrapv -include /tmp/kuna-callwide-repro/support.h \
  -I/tmp/kuna-callwide-repro/typed \
  /tmp/kuna-callwide-repro/typed/mips.elf.c \
  /tmp/kuna-callwide-repro/replay.c -o /tmp/kuna-callwide-repro/replay-typed
/tmp/kuna-callwide-repro/replay-typed
# expected=112233405566778e got=112233405566778e; exit 0.
```

## Diagnosis and evidence limits

P4's `kuna_bejoin/evidence.rs::seed` caches producer-body proofs only.
`kuna_callerreads::read` handles recovered-void outputs and one register; it
does not establish this pair. `ActionReturnRecovery::apply` scores output
trials before selecting their join order. On this MIPS witness the low word
also feeds the carry, so ordinary use scoring rejects it; changing the join
order cannot restore it. A new proof must justify both retaining the word and
using the ABI order at every affected call.

The preserved baseline measurements are pinned in `record.json`:

| Check | Result |
|---|---|
| 117 caller decompile cells, three BE ABIs | Default caller evidence does not improve the producer; single JSON and whole producer agree |
| Six emulated target-ELF runs, O0/O2 runners | All five wide and five high-word-only vectors pass per run; the 36 inspected function bodies match those decompiled |
| 36 generated whole-C replays, O0/O2 | Default MIPS loses low bits on three of five wide vectors; typed producers pass every ABI/mode |
| Six project exports, 12 C/header replays | Same MIPS failure; all high-word-only controls pass under default |
| PPC/ARM default wide witness | Reversed producer output is compensated by the caller; whole-C replay passes |
| PPC/ARM `bejoin on` narrow controls | Wrong result; global activation is unsafe |
| 198 separate declared/linked-DWARF cells | All 33 declared functions agree across bejoin modes and linked DWARF |

Target execution used QEMU user emulation, not physical target hardware.
These are historical baseline measurements. ET_REL debug relocations are
excluded. The implemented feature remains partial; #904 remains open until
sound default behavior is validated.

## Implemented-feature validation

The final release snapshot uses merged base `74d0bab7`, source tree
`824561aa9da5cc542c775e00ebc146f867a2ee28`, and 401 frozen processor-spec
inputs. Both declared and consumer-only DWARF routes promote the untyped MIPS
producer from four to eight bytes. Whole, selected, reordered, worker and stream
exports replay their actual C and headers at O0/O2 under MIPS QEMU. Eight input
vectors exercise carry, sign bits, the direct producer, all six callers and both
DWARF composition helpers; stack poisoning detects undefined narrow padding.
Canonical and raw `xunknown8` output projections also pass. Original executable
bytes, addresses and every function's ownership are checked before target replay.

All 18 exact-word refusal mutations preserve complete off/on function records.
A mixed image promotes its accepted producer while leaving the rejected producer
and its declaration dependencies unchanged. ARM, missing sealed entry and
incompatible narrow-extension policy controls preserve ordinary C and headers.
An independent 144-query panel compares the merged baseline, default-off and
explicit-on arms: 132 JSON documents, 375 function records and 24 exported C/header
files are byte-identical for its default/refusal controls.

The mixed-sign stream deferral path is tested with actual engine results in
caller64, ignored-caller, producer order. Its unchanged three-function C and
header pass MIPS O0/O2 replay. Ordinary serial and parallel stream runs are
separate evidence; their observed orders delivered the producer before the
ignored caller.

The final gates pass 675 upstream assertions, 3,158 stage assertions and 9,317
Rust tests (38 ignored), plus tooling, lint, strict spec and catalog checks.
The upstream baseline is unchanged; the stage baseline adds the seven firing
assertions and updates the catalog row count.

The fixed CPU24 timing run retains all samples and external-load observations
(`quiet=false`). Values below are medians of paired percentage changes; enabling
the proof is compared with the candidate's default-off arm.

| Workload | Repeats per arm | Default-off CPU vs baseline | Enabled CPU vs off | Enabled wall vs off |
|---|---:|---:|---:|---:|
| Healthy 154-function executable | 7 | -0.34% | +1.35% | +1.36% |
| Sealed 11-function MIPS image | 21 | +2.44% | +1.90% | +1.94% |
| Sealed 256-callsite MIPS image | 7 | -0.40% | +9.52% | +9.42% |

Default-off stays within the 5% CPU budget; no follow-up run was required.
These opt-in costs support keeping the default off. `record.json` pins the final
release, raw acceptance, gate, source-identity and timing receipts, including
preserved failures corrected before the final snapshot.

# Function-level Unicorn oracle

An explicit function case records its original executable bytes, register
state, launch address and mapped memory. Unicorn executes the case without
loading SLEIGH or interpreting p-code. The raw p-code and final IR interpreters
start from the same declared case. The three observations can be examined
independently of the sampled Tier C verdicts.

## Reproduce

Use an installed Clang and LLD with the x86-64 ELF target, built SLEIGH specs,
and Python 3.11 or later with `unicorn==2.1.2`.

```sh
(cd decompiler && cargo build --release -p kuna-verify)
python3 scripts/diffverify/test_function_oracle.py
python3 scripts/diffverify/test_function_oracle_integration.py
```

To save a case and run it separately:

```sh
clang --target=x86_64-linux-gnu -O2 -g -ffreestanding -fno-pic \
  -fno-stack-protector -c scripts/diffverify/fixtures/function_oracle_x86_64.c -o fixture.o
ld.lld -static -z separate-loadable-segments -e add fixture.o -o fixture.elf
decompiler/target/release/kuna-verify oracle-case fixture.elf store_loop \
  --sleighpath specs --seed 0x6b756e61 > case.json
python3 scripts/diffverify/function_oracle.py case.json > unicorn.json
```

The JSON envelope holds the reusable `case`, the explicit decompiler options
and the two Kuna `observations`. The Unicorn runner also accepts the case on
its own. The fixtures were checked on macOS arm64 with Clang 22.1.8, LLD
22.1.6 and Unicorn 2.1.2.

| Fixture | Observations |
|---|---|
| `add`, `add_double` | All three give the expected integer or scalar double return bits. |
| `store_loop` | All three give the expected return and writes to initialized and zero-initialized globals. A low raw p-code budget makes the run incomplete. |
| `pointer_update` | All three write the expected bytes through a pointer passed in RDI into the synthetic heap. |
| `call_inner` | The internal call returns with the expected result and memory. Kuna's observations disclose the raw callee effect that the IR can replay. |
| `clobber_rbx` | Unicorn and raw p-code both register an RBX preservation violation. The final IR reports the return value. |
| `caller_write` | Unicorn and raw p-code write a caller-owned stack slot. The final IR leaves its bytes unchanged. A separate Tier C run reports `AGREES(32)`. |
| `system_boundary` | All three reach the system call and preserve the preceding store. The syscall's effects are outside this case model. |
| `unmapped_read` | All three stop on an unmapped load. Unicorn reports a memory fault and Kuna reports a trap. |

The Tier C control runs matched the pinned reference's 5,079 corpus verdicts
and 1,654 real-image verdicts. The corpus file error also matched. At 32
input runs per function, all 19,047 individual mutation records matched
after removing timings.

## Compare observations

```sh
python3 scripts/diffverify/oracle_compare.py case.json unicorn.json --return RAX:8
```

Declare the observed return storage explicitly as `REGISTER:BYTES` or
`void`. For the `add_double` fixture use `XMM0:8`. This scope comes from
the caller or a known signature. When the final IR reports a different
physical return location or width, the comparison records that limitation.
The Unicorn observation includes a SHA-256 digest of the literal case,
covering the input registers, bytes, memory map and declared machine limits.
The evidence also records the ELF and loaded SLEIGH image digests.

```sh
python3 scripts/diffverify/oracle_replay.py case.json fixture.elf --return RAX:8
```

Replay takes the same ELF and spec root, regenerates the input and both
Kuna observations, and then executes the archived case in Unicorn. It
checks the image digests, regenerated case and archived Kuna observations.

- `CROSS_BACKEND_AGREEMENT` records a single completed sample under the
  selected return and memory scope. Final IR exposes no full physical
  register file. The ABI checks use the two physical executions.
- `COUNTEREXAMPLE` records a disagreement involving Unicorn. The JSON
  reports the differing return or termination, a representative byte
  address per memory disagreement pattern, or a mismatch at a shared
  system-call boundary. At that boundary it compares the call number and
  the observed memory prefix. For declared external calls it compares the
  target, declared integer inputs and observed memory prefix. Candidate
  causes are diagnostic hypotheses.
- `INCONCLUSIVE` retains the evidence from available observations.
  A matching trap, a shared ABI violation, an incomplete execution,
  replayed callee effects, or missing return coverage limits the result.
  The comparator also retains disagreements found only within the two
  Kuna executions so they can be inspected.

The fixture tests use an isolated SLEIGH build that changes a register
`ADD` into subtraction. Raw p-code and final IR agree on the same wrong
return and writes. Unicorn runs the unchanged ELF and the three-way
report identifies both differences. Build `slacomp` for this test
(`cargo build --release -p kuna-slacomp` from `decompiler`).

Use `--external-entry ADDR ARITY` on `oracle-case` to treat the first
declared external call reached by the selected function as an observation
boundary. `ARITY` counts full-width SysV integer arguments in registers.
The comparison requires all three runs to stop at the same call site before
any of them models the call's effects. It reports the prefix separately
from returned runs. Floating-point and stack arguments, nested external
boundaries and the call's effects remain outside this projection.

## Input model and coverage

- The exporter handles fixed-address, little-endian x86-64 ELF executables
  with nonoverlapping load pages. It reads the `PT_LOAD` bytes directly via
  the ELF headers and zero-fills the rest of each declared region. It
  rejects interpreters, dynamic segments, allocated relocations and TLS.
  The runner can also execute a hand-authored map with pre-applied
  relocations. It has no process loader or OS environment.
- The version 1 case records SysV AMD64 and the Unicorn `qemu64` CPU model.
  Entry state includes general-purpose and XMM registers, arithmetic flags,
  segment bases and initial SSE/x87 environment. The exporter supplies
  seeded words, an aligned stack, a synthetic heap and an unmapped return
  sentinel. It observes writable ELF data, the heap and the modeled caller
  stack page above the synthetic return slot. Use `--input-reg RDI 0x60000100`
  to pass a heap pointer, and supply other integer or vector register arguments
  the same way.
- `memory` entries have a page-aligned hexadecimal `address`, byte `size`,
  `permissions` (`r`, `rw`, `rx` or `x`), optional initial `bytes` in hex,
  and optional `observe: true` for writable pages. The total is capped at
  16 MiB. Only the declared map is available for guest accesses. Checkpoints
  use named registers with hexadecimal values, including up to 128 bits in
  XMM registers. The runner checks the return slot contents and stack
  alignment at entry.
- Unicorn reports the reached return, trap, fault or incomplete boundary,
  captures registers and watched memory, and checks the ABI-preserved
  integer registers and RSP on return. Declared external entry addresses
  stop execution and record physical argument registers as well as the
  predecessor PC. System calls and unsupported environmental dependencies
  such as timestamp reads stop before any modeled effect. Limits default
  to 10,000 Unicorn instructions and a 1 s emulator timeout. Kuna reports
  a separate p-code operation budget.
- The Kuna executor records raw calls and any concrete effects available
  for graph replay. Calls limit coverage since graph replay can share the
  raw execution's effects and stubs can supply hypothetical outcomes.
  Final IR reports its claimed return storage, width and value. Physical
  registers absent from the IR are available only from the raw and Unicorn
  sides. Failed interpreter runs can lack a final memory snapshot.
- CPU features and the syscall ABI are limited to the declared model.
  Comparison support for floating point beyond the scalar fixture, system
  ABI variants, runtime relocations and external interactions requires
  further validation. Unicorn shares QEMU's execution lineage. A recorded
  run provides evidence only for its particular map, entry state and
  observed effects.

See the [verification roadmap](verification-roadmap.md) for the reference
measurements, dependency audit and subsequent equivalence checks.

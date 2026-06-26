# FID fixture

The fingerprint fixtures for kuna's FID (Function-ID) port — a tiny, hermetic,
**regenerable** set used by the `.fid` database + `kuna fid build` generator tests
(FID PR3). See `docs/fid-design.md` §7 for the design.

## Files

| File | What |
|---|---|
| `lib.c` | Three distinctive, self-contained functions (`kuna_crc32`, `kuna_strlen`, `kuna_memset`), each ≥ 4 code units, **no external libc call** — so each body is fully self-contained and its FID full hash is stable across link. `kuna_crc32` is the star: a CRC-32 loop with the magic `0xEDB88320` immediate. |
| `main.c` | Calls each `kuna_*` (so the linker retains them in the stripped `prog` PR4 will build + vendor). Not used by PR3. |
| `lib.o` | **Vendored.** The compiled `lib.c` (see *Build* below). |
| `lib.fid` | **Vendored.** The generated fingerprint DB — the committed output of `kuna fid build` over `lib.o`. |

The same `lib.o`/`lib.c`/`lib.fid` are mirrored into
`decompiler/crates/kuna-analysis/tests/fixtures/` as `fid_lib_x86_32.{o,c,fid}`
(the `cargo test` fixtures home — the consistency gate `verify_fid_build.rs` reads
them there).

## Build (in the `kuna-dev` container)

The current vendored `lib.o` is **32-bit x86** (`x86:LE:32:default`) — see the
*Architecture note* below. From this directory:

```bash
# 1. compile the library object (32-bit x86)
gcc -m32 -O2 -ffreestanding -fno-stack-protector -c lib.c -o lib.o

# 2. fingerprint it with kuna's own generator → the vendored .fid
#    (SLEIGHHOME must point at the repo `specs/` with the x86 .sla built:
#     `make specs`, or `slacomp specs/.../x86.slaspec`)
SLEIGHHOME="$PWD/../../../specs" \
  kuna fid build lib.o -o lib.fid --lang x86:LE:32:default --cspec gcc

# 3. mirror into the cargo-test fixtures dir
cp lib.{o,c,fid} ../../../decompiler/crates/kuna-analysis/tests/fixtures/
#   …renaming to fid_lib_x86_32.{o,c,fid}
```

`-O2 -ffreestanding -fno-stack-protector` keeps the bodies self-contained and
deterministic (no stack-guard prologue, no libc dependency). Regenerate `lib.fid`
whenever the FID DB `version` bumps or the hasher/skipper changes.

## The vendored records

`kuna fid build lib.o` produces (x86:LE:32:default, gcc):

| function | full_hash | specific_hash | code_unit_size | specific_addl |
|---|---|---|---|---|
| `kuna_crc32`  | `0x0c802846cd3b75ef` | `0x270017388f7ad28e` | 33 | 24 |
| `kuna_strlen` | `0x6457464202e9c843` | `0x0dc29556fc68a8f2` | 15 | 13 |
| `kuna_memset` | `0x1461d890c55eb740` | `0x8706041ac5102d0b` | 18 | 19 |

(`verify_fid_build.rs` asserts a fresh in-process generate reproduces these and
that they round-trip through the `.fid` format.)

## Architecture note (honest fidelity accounting)

The FID design doc (`docs/fid-design.md` §7.2) targets **x86-64**
(`x86:LE:64:default`). The vendored fixture is **32-bit x86** because the FID PR1
`Sleigh::instruction_mask` accessor does **not yet decode REX-prefixed (x86-64)
instructions**: its post-decode decision-tree re-walk
(`DecisionNode::resolve_matched`) reads the *final* multi-phase decode context
(`instrPhase` advanced past the REX-prefix phase) and so fails to re-resolve the
constructor — `instruction_mask` returns "Unable to resolve constructor" for e.g.
`48 01 fe` (`add rsi, rdi`). 32-bit x86 has no REX prefixes and masks cleanly, so
it exercises the **entire PR3 mechanism** (the byte-exact PR2 hasher, the `.fid`
format, the generator, the loader rebasing) end-to-end with verifiable hashes.

To regenerate the x86-64 fixture **once PR1's `instruction_mask` handles
multi-phase decode** (a tracked follow-up):

```bash
gcc -O2 -ffreestanding -fno-stack-protector -c lib.c -o lib.o
kuna fid build lib.o -o lib.fid --lang x86:LE:64:default --cspec gcc
```

## Cross-validation (PR2 fidelity follow-up)

The PR2 increment log flagged the Ghidra-validated golden-vector gate (§7.1) as a
deferred follow-up: the `~/github/ghidra` checkout has the FID sources but no built
distribution, so dumping `(function → FidHashQuad)` vectors needs a multi-GB gradle
build out of env scope. The hashes above are validated by self-consistency (a fresh
generate reproduces the vendored `.fid`) and by the byte-exact PR2 unit gate against
the published FNV-1a64 vectors; a real Ghidra dump over this `lib.o` remains the
target for full cross-validation.

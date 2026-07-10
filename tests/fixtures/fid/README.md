# FID fixture

The fingerprint fixtures for kuna's FID (Function-ID) port — a tiny, hermetic,
**regenerable** set used by the `.fid` database + `kuna fid build` generator tests
(FID PR3). See `docs/history/fid-design.md` §7 for the design.

## Files

| File | What |
|---|---|
| `lib.c` | Three distinctive, self-contained functions (`kuna_crc32`, `kuna_strlen`, `kuna_memset`), each ≥ 4 code units, **no external libc call** — so each body is fully self-contained and its FID full hash is stable across link. `kuna_crc32` is the star: a CRC-32 loop with the magic `0xEDB88320` immediate. |
| `main.c` | Calls each `kuna_*` (so the linker retains them in the stripped `prog`). |
| `lib.o` | **Vendored.** The compiled `lib.c` (see *Build* below). |
| `lib.fid` | **Vendored.** The generated fingerprint DB — the committed output of `kuna fid build` over `lib.o`. |
| `prog` | **Vendored.** The statically-linked, `-no-pie`, **stripped** x86-64 ELF (`lib.o` + `main.o`), the FID PR4 re-identification target. No symbol table; the FID e2e (`kuna-console/tests/verify_fid.rs`) renames its `sub_<addr>` placeholder back to `kuna_crc32` by full-hash match against `lib.fid`. The pinned VMA of `kuna_crc32` in `prog` is **`0x4017c0`** (`-static -no-pie` keeps it stable; recorded by the e2e as `KUNA_CRC32_VMA`). |

The same `lib.o`/`lib.c`/`lib.fid` are mirrored into
`decompiler/crates/kuna-analysis/tests/fixtures/` as `fid_lib_x86_64.{o,c,fid}`
(the `cargo test` fixtures home — the consistency gate `verify_fid_build.rs` reads
them there).

## Build (in the `kuna-dev` container)

The vendored `lib.o` is **x86-64** (`x86:LE:64:default`), the architecture the FID
design doc targets — see the *Architecture note* below. From this directory:

```bash
# 1. compile the library object (x86-64, the gcc default)
gcc -O2 -ffreestanding -fno-stack-protector -c lib.c -o lib.o

# 2. fingerprint it with kuna's own generator → the vendored .fid
#    (SLEIGHHOME must point at the repo `specs/` with the x86 .sla built:
#     `make specs`, or `slacomp specs/.../x86-64.slaspec`)
SLEIGHHOME="$PWD/../../../specs" \
  kuna fid build lib.o -o lib.fid --lang x86:LE:64:default --cspec gcc

# 3. mirror into the cargo-test fixtures dir
cp lib.{o,c,fid} ../../../decompiler/crates/kuna-analysis/tests/fixtures/
#   …renaming to fid_lib_x86_64.{o,c,fid}

# 4. (PR4) build the stripped re-identification target `prog`.
#    `-static -no-pie` pins the addresses; `strip --strip-all` removes every
#    symbol, so the FID e2e must recover `kuna_crc32` purely by fingerprint.
gcc -O2 -c lib.c main.c                       # plain -O2 (the prog/lib build need
                                              # not match — the FULL hash is
                                              # operand-masked / position-independent)
gcc lib.o main.o -o prog -static -no-pie
strip --strip-all prog                        # kuna_crc32 lands at VMA 0x4017c0
```

After `strip --strip-all`, `kuna_crc32` is at **`0x4017c0`** (verify with
`objdump -d prog --start-address=0x4017c0` before the strip, or `nm` the un-stripped
`prog`). The FID e2e (`kuna-console/tests/verify_fid.rs`) hardcodes that VMA — bump
`KUNA_CRC32_VMA` there if a toolchain change moves it. Note step 4 overwrites the
`-ffreestanding` `lib.o` from step 1 with a plain `-O2` one; `git checkout
tests/fixtures/fid/lib.o` to restore the vendored (`.fid`-matching) object after
building `prog`.

`-O2 -ffreestanding -fno-stack-protector` keeps the bodies self-contained and
deterministic (no stack-guard prologue, no libc dependency). Regenerate `lib.fid`
whenever the FID DB `version` bumps or the hasher/skipper changes.

## The vendored records

`kuna fid build lib.o` produces (x86:LE:64:default, gcc):

| function | full_hash | specific_hash | code_unit_size | specific_addl |
|---|---|---|---|---|
| `kuna_crc32`  | `0x2603a009d9e0776f` | `0xc05eead857c9304a` | 25 | 30 |
| `kuna_strlen` | `0x03a308fb37c24cf0` | `0xea7136fa1ab9ce93` | 12 | 22 |
| `kuna_memset` | `0x67c5bd4207a978b3` | `0xe74cd6a00ca603e6` | 11 | 31 |

(`verify_fid_build.rs` asserts a fresh in-process generate reproduces these and
that they round-trip through the `.fid` format.)

## Architecture note

The vendored fixture is **x86-64** (`x86:LE:64:default`), the architecture the FID
design doc (`docs/history/fid-design.md` §7.2) targets. The byte-exact PR2 hasher, the
`.fid` format, the generator, and the loader rebasing are architecture-agnostic
and the x86-64 `lib.o` exercises the whole PR3 mechanism end-to-end with verifiable
hashes.

This fixture was briefly 32-bit x86 because the FID PR1 `Sleigh::instruction_mask`
accessor could not decode REX-prefixed (x86-64) instructions — its post-decode
decision-tree re-walk read the *final* multi-phase decode context (`instrPhase`
advanced past the REX-prefix phase) and failed to re-resolve the constructor.
The PR1-fix captures each constructor node's matched pattern DURING decode (under
the correct per-phase context), so REX-prefixed instructions like `48 01 fe`
(`add rsi, rdi`) now mask correctly and the fixture is re-homed to x86-64.

## Cross-validation (PR2 fidelity follow-up)

The PR2 increment log flagged the Ghidra-validated golden-vector gate (§7.1) as a
deferred follow-up: the `~/github/ghidra` checkout has the FID sources but no built
distribution, so dumping `(function → FidHashQuad)` vectors needs a multi-GB gradle
build out of env scope. The hashes above are validated by self-consistency (a fresh
generate reproduces the vendored `.fid`) and by the byte-exact PR2 unit gate against
the published FNV-1a64 vectors; a real Ghidra dump over this `lib.o` remains the
target for full cross-validation.

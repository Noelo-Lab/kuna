# kuna bug repro binaries

Binaries that reproduce open output bugs, kept here so an issue can be acted on
without reconstructing a corpus first (same idea as `tests/hang-repro/`, which
holds the binaries for the fixed decompile hang).

These are **not** a test gate. Nothing under `make test` / `make rust-test` /
`make test-stages` reads this directory — the pinned regression for a fixed bug
belongs in `tests/stages/` or the cargo suite. This is evidence for an issue.

## Provenance

Every binary is a **stripped**, optimized build produced by the
[DecBench](https://decbench.com) corpus (public dataset:
`noelo-lab/decbench-dataset`) and copied verbatim from that run's results tree at
`<opt>/<project>/stripped/<name>`. They are builds of public upstream sources
(GPL/BSD, per project); DecBench's `projects/*.toml` records the exact source
revision and compiler flags for each. Nothing here is kuna-authored.

| file | sha256 (first 16) | project · build | DecBench path |
|---|---|---|---|
| `grep` | `f71085ac24299246` | GNU grep · x86-64, gcc -O2, PIE | `O2/grep/stripped/grep` |
| `libselinux.so.1` | `477a2fd5dd3f23cb` | libselinux · x86-64, gcc -O2, shared | `O2/libselinux/stripped/libselinux.so.1` |
| `betaflight_STM32F405.elf` | `af14f018ac689b91` | Betaflight · ARM Cortex-M (Thumb), -O2, static | `O2/betaflight/stripped/betaflight_STM32F405.elf` |
| `sort` | `8e01fe2367a880f4` | GNU coreutils · x86-64, gcc -O2 -fno-inline, PIE | `O2-noinline/coreutils/stripped/sort` |
| `faillog` | `d3b414256e4878f8` | shadow · x86-64, gcc -O2, PIE | `O2/shadow/stripped/faillog` |

## What reproduces what

Every command is a single function out of a whole-binary load. Use
`decompile-all --addr`, not `decompile <addr> --addr`: it is the surface DecBench
drives, the Listing analysis tier is on by default there, and stripped ARM
firmware needs that tier to find the function at all.

```bash
K=decompiler/target/release/kuna

# stack-protector load kept after the check is dropped (dead read of an
# undefined `// fs_offset` local as the first body statement)
$K decompile-all tests/bug-repro/sort    --addr 0x6e80    # coreutils sort  zaptemp
$K decompile-all tests/bug-repro/faillog --addr 0x3320    # shadow faillog  setmax_one

# libc data imported by a COPY relocation stays `dat_<addr>` (stderr, stdout,
# optind, optarg …) where every other decompiler prints the name
$K decompile-all tests/bug-repro/faillog --addr 0x3320    # two fprintf(stderr, …)

# overflow guard rendered against the post-add value
$K decompile-all tests/bug-repro/grep --addr 0x7c90       # grep nlscan

# a stack load that happens after a call is hoisted above it
$K decompile-all tests/bug-repro/libselinux.so.1 --addr 0x17370   # lsetfilecon_raw

# stack array declared too small for the stores emitted into it; the slots past
# the end become separate scalars that are read but never assigned
$K decompile-all tests/bug-repro/betaflight_STM32F405.elf --addr 0x801b4a4  # cliServo
```

Each open issue links the binary and address it needs; see the repository's
issue list for the current status of each. `faillog` carries two distinct bugs,
which is why it is here once and referenced twice.

## Adding to this directory

Keep it small and evidential: one binary per distinct bug, preferring the
smallest binary that shows it, and only when the address alone would not let a
maintainer reproduce. Record the sha256 and the DecBench path in the table above
so the copy is traceable, and delete the entry when the bug is fixed and pinned
by a real regression test.

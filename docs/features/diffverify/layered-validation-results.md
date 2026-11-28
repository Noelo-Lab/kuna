# Layered validation fixtures

These measurements cover the linked x86-64 Linux ELF fixture compiled
with Clang 22.1.8 at `-O2` and the assembled RV64I ELF fixture. Both ran
on macOS arm64. The x86-64 checks use Z3 4.13.0. The RV64I checks add
Isla and the pinned Sail snapshot described in the formal ISA guide.
Wall time includes process startup and the solver. Peak RSS is the
maximum resident set size reported by macOS `/usr/bin/time -lp` for one
invocation. [x86-64 fixture](function-oracle.md) · [RV64I fixture](formal-rv64.md)

| Target | Case | Verdict | Wall time | Peak RSS |
|---|---|---|---:|---:|
| x86-64 | Integer add | `MODEL_EQUIVALENCE` | 0.24 s | 64.3 MiB |
| x86-64 | Pointer aliases inside the memory window | `MODEL_EQUIVALENCE` | 0.17 s | 65.3 MiB |
| x86-64 | Pointer range extended past the window | `INCONCLUSIVE` | 0.17 s | 65.1 MiB |
| x86-64 | Countdown with a four-visit bound | `BOUNDED_EQUIVALENCE` | 0.12 s | 64.5 MiB |
| x86-64 | Countdown with no jointly completed input | `INCONCLUSIVE` | 0.13 s | 65.9 MiB |
| x86-64 | Countdown with an inductive ranking term | `MODEL_EQUIVALENCE` | 0.14 s | 64.3 MiB |
| RV64I | Add | `MODEL_EQUIVALENCE` | 0.79 s | 131.8 MiB |
| RV64I | Add with final IR changed to subtraction | `COUNTEREXAMPLE` | 0.64 s | 144.6 MiB |
| RV64I | Add with negative immediate | `MODEL_EQUIVALENCE` | 0.65 s | 144.8 MiB |
| RV64I | Arithmetic right shift | `MODEL_EQUIVALENCE` | 0.64 s | 144.8 MiB |
| RV64I | Load outside the ALU model | `INCONCLUSIVE` | 0.07 s | 24.9 MiB |

The same pointer-aliasing proof was also run with an additional 72-byte
private-stack window on each x86-64 build. The proof completed on all
four builds with the same observable memory and return projection.

| Compiler | Optimization | Wall time | Peak RSS |
|---|---|---:|---:|
| Clang 22.1.8 | `-O0` | 1.14 s | 88.3 MiB |
| Clang 22.1.8 | `-O2` | 0.23 s | 68.5 MiB |
| Apple Clang 17.0.0 | `-O0` | 0.94 s | 88.5 MiB |
| Apple Clang 17.0.0 | `-O2` | 0.23 s | 70.1 MiB |

The RV64I integration suite completed proofs for four equivalent ALU
fixtures, including an immediate addition that reads the zero register.
The load fixture remained outside coverage. Changing the SLEIGH ADD lift
makes raw p-code and final IR produce the same incorrect result. The suite
detected both deliberate RV64I mutations and Unicorn reproduced both
witnesses. The x86-64 suite compared exact floating-point bits for a rounding tie,
negative zero, infinity, quiet NaN payload propagation, signaling NaN
quieting, and an operation that produces NaN. A separate loop fixture
branches to ADD or LEA in the machine bytes. Kuna folds both paths into
one IR loop body. The inductive proof checked the different control-flow
graphs and Unicorn exercised both machine paths. These results apply to
the listed fixtures and test mutations.

Ethos checked refutations from the integer add, scoped aliasing,
bounded countdown, inductive countdown and Sail RV64I ADD examples.
The switch-table obligation was proved by Z3 under its explicit target
set. Its CPC proof generation exceeded the 15-second certificate
budget. [Certificate workflow](proof-certificates.md)

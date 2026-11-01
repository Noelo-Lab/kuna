# Rust keyword machine operations

Rust output now spells a keyword-named functional machine operation as a raw
identifier. For example, x86 `in(0x31)` becomes `r#in(0x31)`. The prefix preserves
the underlying identifier. AVR `break` and MIPS `yield` likewise remain calls.
The correction changes only Rust functional CALLOTHER names; ordinary names,
nonfunctional displays, and C/Go output retain their previous spelling.

Three XML fixtures contain actual x86 IN/OUT, AVR BREAK, and MIPS YIELD bytes.
Their five unchanged emitted bodies execute against recording Rust stubs,
checking 42 calls per run: port truncation, returned byte extraction, output
arguments, break/yield dispatch and single evaluation. This checks the emission
contract, not native hardware I/O or AVR/MIPS execution. All four Rust editions
compile and execute under Rust 1.90, 1.93.1 and 1.99.1: 12 successful runs and
504 checked calls. The before program fails keyword parsing in all 12 cases.
The workspace integration test independently exercises editions 2015 and 2024.

The eighteen-image comparison covers 25,339 distinct
functions and 50,733 function records. It changes 8
Rust bodies across 15 hunks, inserting exactly 15 raw `in` prefixes.
Every other token, delimiter, punctuation mark, literal and comment is retained.
The independent token verifier rejects mutations to arguments, declarations,
operators, strings and comments, and refuses prefix insertion outside a callee.
Existing unrelated syntax defects can be tokenized without repairing them.
Function metadata, variable declarations and instruction address sets match.
All 41,461 C/Go records are exact; all
16,122 Go bodies parse. Baseline: `1689dc9e01c23d50c96a15b4ff2d4cb4a08f895c`.

All 10,063 workspace tests pass, with 39 ignored
across 71 suites. Original assertions remain 675/675; stages
pass 3,494/3,494. The new stage improves from 1/2 to 2/2. Tooling, lint, spec,
catalog and counter checks pass. The original baseline is unchanged.

Three timing cases cover default C, an affected Rust runtime function, and Go.
Each uses 31 alternating pairs after three warmups per arm, after local builds
and scans finish. Maximum measured changes are +0.56% CPU and
+0.56% wall, within the 5% budget. Raw samples are retained.

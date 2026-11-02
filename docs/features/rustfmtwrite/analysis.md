# Opt-in verified Rust String writes

`rustfmtwrite`, off in every preset, recovers `write!` and `writeln!` for exact String writer
tables from stable Rust 1.93.0/1.93.1 x86-64 little-endian ELF images.
The direct `core::fmt::write` call must have four runtime arguments and a
one-byte bool, u8 or i8 result. The original writer must come from an entry
value through copies or casts. A complete immutable, aligned 48-byte table
must identify String drop, size 24, alignment 8, String write_str/write_char
and the exact write_fmt method. Every alias must agree. ELF RELRO supplies
private immutability evidence, rounded down at the upper page boundary;
volatile annotations override it.

The existing literal, integer and string proofs verify the format descriptor.
Recovery borrows the same String pointer and scopes the formatting-trait import
inside the expression. `.is_err()` preserves the native result, including its
byte conversion where required. Setup, branches, declarations, prototypes and
IR remain unchanged. Custom writers, dynamic width, unsupported callbacks and
unknown tables remain explicit runtime calls. Candidate symbol maps index only
possible formatting identities and then reject conflicting or unknown aliases.

## Native evidence

Seven integration tests cover seven positive writes plus refusal boundaries,
metadata conflicts, producer/layout/language checks, volatile tables and result
shapes. A zero-instruction compiler barrier in the literal fixture preserves the
initialized Arguments words while preventing LLVM from replacing the test call.
The stage case moves from 0/10 to 10/10. Twenty-two functions from unsupported
1.90.0 and 1.99.1 producers remain identical with recovery on and off.

Three native ABI probes confirm the one-byte fmt::Result representation,
Ok=0, Err=1, String size/alignment and original-buffer mutation. The emitted
expressions execute unchanged with explicit typed inputs under Rust 1.90.0,
1.93.1 and 1.99.1. Each executor matches 7,056 original native calls, totaling
21,168 comparisons. Inputs include 156 UTF-8 strings, 263 unsigned and 264 signed
values, nine mixed unsigned values and three preexisting buffer prefixes.
Output bytes, appending, allocation-capacity variation and result bytes match.

## Complete output review

The 27-image comparison covers 29,156
distinct functions and 62,184 language records.
All 49,095 C/Go records remain identical; all
19,939 Go records parse. Disabling recovery restores all
13,089 Rust records exactly. Eight
functions change in 8 hunks: the seven native examples and
angr's modern `write_str` example. Full AST and comment comparisons preserve
surrounding statements. Independent image reads and call-linked operands prove
every table, callback, literal and argument binding. Seven native complete function records match the earlier review. Whole-image
analysis gives the reference writer a pointer type where the earlier single
function analysis used an integer. This difference already exists on the parent.
The review admits only that exact parameter/metadata change, its original-call
pointer casts and the corresponding recovered-writer cast. Every other field
and instruction mapping is identical. The fresh emitted expression matches
7,128 additional native comparisons: 264 signed values, three existing prefixes
and three initial capacities under each of three execution compilers.

All eight unchanged recovered expressions compile and execute under the three
execution compilers. The partial compiler audit reduces selected diagnostics
from eight to zero, introduces no diagnostic multiset and leaves complete-body
standalone compilation at zero before and after. Six omitted instruction
annotations are stack-descriptor LEAs into RCX, confined to replaced call lines;
they have no load, store or flag effect. Call addresses and function metadata
remain. Variable-use mapping changes stay within those calls.

The retained initial review failures concern a proof helper's copy-chain
handling and an execution harness's writer-pointer binding. The corrections
preserve the emitted expressions. Initial stale catalog-count tests were also
corrected; the complete repository checks below ran afterward.

## Default decision and fresh validation

The enabled implementation did not meet its predeclared speed criterion. In the
768-round three-arm study, nine cases passed but the small C control had upper
mean wall-time bounds of +5.603% against the parent and +6.125% against the
disabled option, above the 5% budget. Its median wall deltas were +0.084% and
+0.151%. This is an unresolved confidence bound, not a measured 6% median
slowdown. The failed study is retained and was not repeated unchanged.

The option consequently ships off in every preset. A fresh release build and
all four required checks pass: 675 original assertions, 3,541 stage assertions,
10,109 workspace tests with zero failures and 39 ignored across 71 suites.
Tooling passes 114 tests with one optional skip. Lint, spec, catalog, generated
documentation and shared counts pass. The initial preset-policy test failure is
retained with the test-only correction documenting the intentional exclusion.

Fresh captures of all 27 images compare every complete function record. Default
and explicit-off Rust output equal the previously reviewed parent output;
explicit-on Rust and default C/Go equal their previously reviewed output. This
binds the native execution evidence above to byte-identical enabled expressions
from the new binary. Native execution was not repeated for this default-only
rebuild. `fresh-breadth.json` records both generations of capture hashes.

A separately declared default-off comparison uses four cases, 192 balanced
two-arm rounds, three warmups and one final 20,000-resample block bootstrap,
with family alpha 0.05 across eight CPU/wall comparisons. Every median ratio
and upper mean-ratio bound meets 5%. Maximum parent-to-default median CPU/wall
deltas are +0.371%/+0.183%. This verifies the
configuration being shipped and cannot promote recovery to default-on. Both
studies retain observations, CPU assignments, telemetry and binary identities.

The enabled proof artifacts name their original binary and capture state. The
parent source manifest and reanchor receipt bind that state to committed parent
`e29a0ef52489142062669b7b8b41d0db09120fcb` without changing its executable. The fresh binary hash is
`ebff674185bf38266bea0adc10afda33a0bba10a699f09be32a1ab9b09e9a923`. Full Go/Rust parity remains separate work.

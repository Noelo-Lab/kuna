# Verified Rust string formatting

`rustfmtstr` exposes string Display arguments inside existing print, allocating
format and panic recovery. It accepts stable Rust 1.93.0/1.93.1 on x86-64
little-endian ELF. Full symbol identities must establish a reference Display
callback and `core::fmt::Formatter::pad`, with the exact 18-byte forwarding body
that loads string data and length and directly jumps to that pad implementation.
Generic names alone never establish a concrete string type. Conflicting aliases,
other wrappers, derived Debug and custom formatting remain explicit calls.

Both eight-byte aligned string words must be complete call-linked stack inputs.
The renderer constructs a string view from those same values. Valid UTF-8 and
storage follow the original `pad(&str)` contract. It changes no setup statements,
IR, prototypes or return statements; the existing format, destination and panic
checks still apply. Recovered panic source has its own file, line and backtrace.

## Native and refusal evidence

The Rust 1.93.1 native fixture covers nine positive functions: stdout, stderr,
mixed primitive/string arguments, two strings, Unicode width/precision/fill,
owned String results and panic payloads. A separate oracle exercises 156 valid
UTF-8 strings, including every ASCII character, embedded NUL, Unicode boundaries,
combining and zero-width joiners, Arabic, Japanese and strings through 4096 bytes.
Nine unsigned boundary values are included. Each of three execution compilers
(1.90.0, 1.93.1 and 1.99.1) matches 3,744 native stream calls and 156 panic payloads:
11,700 comparisons. The emitted macro expressions are unchanged; explicit typed
bindings and uninitialized String destinations supply their inputs.

Six integration tests cover the positive fixture and refusal boundaries: option,
source/output language, exact producer identity, missing and conflicting symbols,
altered wrapper bytes/targets, incomplete or misaligned words, scalar size/type
constraints and unsupported callbacks. Five native unsupported forms and six
other-producer controls remain unchanged. A non-LTO wrapper with a different
instruction sequence also remains explicit. These results do not establish
complete-function recompilation or equivalence of original panic locations.

## Complete output comparison

The 26-image comparison covers 28,720 distinct functions and 60,876 language
records. All 48,223 C/Go records are identical, and all 19,503 Go records parse.
Disabling the option restores all 12,653 Rust records exactly. Twelve functions
change in twelve hunks: ten in the new fixture and two in the modern angr format
fixture, including `make_string`. All surrounding parsed statements and comments
remain identical. Independent ELF reads, full callback names, exact forwarding
bytes, template decoding and call-linked descriptor operands verify every new
literal and all thirteen string argument bindings. Eight AST mutations and five
descriptor-evidence mutations are rejected by the review tools.

All twelve unchanged recovered macro expressions compile and execute under the
three execution compilers with explicit bindings. A partial compiler audit finds
one selected diagnostic in one function both before and after, with no new
diagnostic multiset; none of these complete bodies compiles standalone. Source
mappings omit twelve stack-descriptor LEAs into RSI or RDX from replaced call
lines. Disassembly verifies each address computation. Call instruction mappings,
descriptor stores, declarations and function metadata remain. Variable-use
mapping changes are confined to the recovered inputs and hidden destination.

The initial review tool did not account for the existing `param_1` metadata name
corresponding to signature `a0` in String-result functions. Its rejected capture
is retained. The correction verifies that mapping against the actual signature,
and all complete captures were reviewed again without changing production code.
The original stripped angr fixture remains unchanged; symbol recovery and older
consumer layouts remain separate gaps.

## Checks and speed

The original corpus passes 675/675 without changing its baseline; stages pass
3,531/3,531, with the new case moving from 0/8 to 8/8. The parent capture omits
the new option commands unknown to its catalog. The workspace passes 10,102 tests
with zero failures and 39 ignored across 71 suites. Tooling passes 114 tests with
one optional skip. Lint, spec, catalog, documentation and shared counts pass.

Eight cases compare the parent, the new binary with recovery disabled and the
default using 768 balanced randomized rounds after three warmups per arm.
All six arm orders occur in each six-round block. Independent case workers use
disjoint physical CPU pairs, with each case's arms pinned to the same CPU.
No owned build, test or corpus jobs run during measurement. The fixed protocol
and CPU selection were declared before sampling; all raw observations remain.
Maximum parent-to-default median CPU delta is +0.466%
and wall delta is +0.532%; maximum
off-to-default CPU delta is +0.278% and wall delta
is +0.511%. All median comparisons and upper
mean-ratio bootstrap bounds fit five percent. Twenty thousand bootstrap resamples
preserve six-round blocks. One final look allocates family alpha 0.05 across
32 CPU/wall comparisons, with upper quantile 1 - 0.05/32. No interim acceptance
or automatic repetition is allowed. Main commit
`69fad3ce4cbdbb3232680938252bd75c2bec5a68` is included in all final captures.

The earlier implementation's fixed confirmation did not clear its bounds:
the large Rust control's parent wall-time upper bound was 6.135351%, and its
off-to-default bound was 5.261491%. The observations remain under their original
attempt IDs. Callback discovery now indexes only forwarding/pad candidates,
then checks every symbol for conflicting or unknown aliases. A new fixed study
tests that changed implementation; observations from different binaries are not
combined. Fresh captures preserve every previously reviewed complete function
record, so the native and manual mapping evidence still applies.

Two overlapping capture processes were detected during the final integration
review. Both were stopped before timing, and their ambiguous files were retained
locally. An exclusive lock now prevents duplicate capture or review jobs. The
complete 26-image capture was rerun, file hashes were checked again, and every
complete function record matched the previously reviewed output. Timing used
only this clean capture.

This closes the verified string argument portion of modern format-macro coverage.
It does not establish full Go/Rust parity.

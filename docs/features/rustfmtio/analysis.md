# Rust Stdout formatting recovery

`rustfmtio` recovers supported `Stdout::write_fmt` calls as `write!` or
`writeln!`, with the original writer and complete result word preserved. It is
off in every preset because the enabled study did not clear the 5% speed
budget. Use `--option rustfmtio on` to recover the supported expressions. This
does not cover arbitrary `io::Write` implementations or establish full Rust/Go
parity. The default-off wall-time study also remains unresolved, as detailed below.

## Identity and ABI

Recovery requires the supported x86-64 ELF Rust 1.93.1 producer profile,
complete matching method/callback bodies, unambiguous symbol aliases, validated
read-only descriptors and relocation/import roles. Generic callback spelling
alone does not prove the sink. Both promoted inner-handle and reference-cell
receivers are covered. The writer is evaluated once and borrowed with the trait
scoped inside the expression. The `match` preserves zero on success and the
owned error's complete raw word on failure; it does not collapse the result to
a Boolean or drop the error before returning that word.

All 11 changed sites have independent parent operands, template/formatter bytes,
writer identity and result-width checks. The flags case uses two distinct
formatter descriptors for one repeated input. An initial extraction requested
one; the failed review and corrected two-descriptor extraction are retained.

## Native and refusal evidence

The 11 unchanged emitted expressions compile with explicit typed input bindings.
There are 91,881 comparisons against original fixture-source functions and
7,128 against the independently reconstructed reference descriptor oracle.
The latter is not execution of the downloaded reference executable. The checks
use Rust 1.90.0, 1.93.1 and 1.99.1 executors, ordinary pipes, broken pipes and
`/dev/full`, comparing bytes and the full error representation. Integer limits,
Unicode strings, formatting flags and buffered sequences are included.

Empty literals stay explicit. The first cross-version sequence failed because
Rust 1.90 and 1.93/1.99 differ on empty buffered writes after an error. Follow-up
probes show the original source and macro agree within each version; this is a
standard-library-version boundary, not a demonstrated same-version rewrite bug.
The recovered-expression tests exclude this refused call from both sequences.
Its exact on/off output is checked separately. Unsupported Rust 1.90/1.99
producer controls cover 28 functions and remain unchanged. Dynamic/unknown
writers, incompatible relocations, changed profile bytes, aliases, call shapes
and result widths have refusal tests.

## Complete output and tests

The 29-image corpus contains 30,028 distinct functions and 64,800 language
records. With explicit opt-in, eleven functions change in eleven hunks. All
50,839 C/Go records remain identical, 20,811 Go outputs parse, and 13,961
disabled Rust records equal the parent. Function and variable metadata stay
unchanged; the ten lost instruction
annotations are descriptor-address LEAs into RDX on replaced call lines.

A second complete corpus comparison after integrating current main uses fresh
parent records for all 29 images. Shared switchrunonguard output changes are
counted separately: disabled Rust and all C/Go records match the current parent.
All eleven I/O-only changes retain exact previous before/after whole records;
that equality supports reuse of the earlier native and AST evidence. Explicit
String recovery controls also match the new parent. After changing the default
to off, all 64,800 fresh default language records equal that parent, all 13,961
explicitly disabled Rust records equal it, and all 64,800 fresh enabled language
records equal the prior enabled build. These checks bind the earlier semantic
evidence to the final opt-in build. Old whole-corpus equality
is not claimed, and original binary identities are kept.
The partial type audit stays at two selected diagnostics and improves complete
body compilation from zero to one. It does not establish recompilation of all
bodies. The stage changes from 1/12 (the refusal control) to 12/12.

All correctness and repository checks pass: 675 original assertions, 3,563 stage assertions and
10,132 workspace tests, zero failures and 39 ignored across 71 suites,
plus tooling, lint, catalog and spec checks.

## Speed and provenance

Before sampling, the nine-case protocol fixed 384 rounds per arm, three
warmups, balanced six-round order blocks, 20,000 block-bootstrap resamples
and one final look. Family alpha 0.05 covers all 36 parent/enabled and
disabled/enabled CPU/wall comparisons. Every median ratio and simultaneous
upper mean-ratio bound must meet the 5% budget. The Go control failed: parent
to enabled CPU and wall medians rose 1.28% and 1.22%, with upper bounds 5.12%
and 5.39%. All other cases passed. This study remains failed; there was no
unchanged enabled retest, and recovery stays off in every preset.

The revised default-off build has a separate, predeclared four-case comparison
against its committed parent: 768 balanced two-arm rounds, three warmups,
20,000 block-bootstrap resamples, family alpha 0.05 across eight CPU/wall
bounds, and one final look. This study also remains failed: the small-C
wall-time upper bound is 5.72%, despite a 0.63% median increase. Both binaries
had large wall-time stalls near the end of that case; all observations are
retained. The other three cases pass. Across all four cases, median increases
stay below 1.82% and CPU upper bounds below 2.06%.

The repository requires the speed budget for default-on features. This change
ships only as explicit opt-in, with all default outputs equal to the parent
and the unresolved default-off wall-time bound documented here. Neither
study is reclassified as passed, no observations are discarded, and there is
no unchanged-code repeat. These measurements do not approve enabling recovery.
Both studies use disjoint physical CPU pairs with no owned builds, tests or
corpus work overlapping sampling. All 154 compiled specs and both source and
binary manifests are checked before and after each study.

[record.json](record.json) contains the exact counts, binary identities and
per-case bounds. [implementation/](implementation/) retains proof scripts,
failure diagnoses, logs, source manifests and raw timing observations. The
original design proposal and its earlier documentation-only validation are
kept separately in `proposal.md` and `proposal-validation/`.

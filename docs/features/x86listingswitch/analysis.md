# x86listingswitch: the discovery walk leaves i386 switch cases undefined

Status: approved design implemented behind the default-off `x86listingswitch`
option. This is a partial underlying improvement for
[#299](https://github.com/Noelo-Lab/kuna/issues/299); its original private PE and
crackme remain unavailable, so this feature does not establish that issue's
acceptance bar.

## Implemented discovery behavior

The durable fixture `decompiler/crates/kuna-analysis/tests/fixtures/x86listingswitch_i386`
contains 24 directly called leaves, a four-case dispatcher at `0x100400`, an
independent function at `0x100480` between the cases, and a real helper reachable
only from case two. A callback word immediately follows the table. This command
isolates AIF from the separate `fast_funcdisc` pointer-table entry heuristic:

```sh
decompiler/target/release/kuna functions \
  decompiler/crates/kuna-analysis/tests/fixtures/x86listingswitch_i386 \
  --mode aggressive --option fast_funcdisc off --option listing on --json
```

With the option off, discovery lists 31 functions, including false entries at
`0x100440`, `0x100460`, `0x1004b0`, and `0x1004d0`, and misses the helper at
`0x100510`. Adding `--option x86listingswitch on` gives 28: those four cases are
decoded without becoming entries, the helper becomes an entry, and the
independent function remains. The gap at `0x100490` remains undefined. The
native assembly and actual emitted dispatcher plus helper agree on all four
cases, out-of-range inputs, and the helper's marker store at O0 and O2.

The implementation uses shared post-walk bounded reconciliation. It leaves
the xrefs reader unchanged; the historical false fifth xrefs edge below still
reproduces and is not a discovery successor. Full, partition-only, serial,
forced-lane, and resumed models agree. Later direct/computed guard bypasses,
new case-entry claims, a seventeenth expansion round, and initial or later
collection-budget exhaustion discard the whole candidate.

## Corpus recall and default

The first implementation sweep compared immutable base `6d7ab7fff` with the
candidate across 107 non-x86-64 decbench images, 17 Wine DLLs, 16 MinGW crackmes,
and a CMSIS twin, keeping shipped, candidate, AIF-off, and candidate/AIF-off
arms. A separate `fast_funcdisc off`/`listing on` arm on the PE corpus produced
the same changes. All 107 decbench images and CMSIS were unchanged. Twenty-one
PEs changed: 399 old entries disappeared (392 symbol mid-body addresses and
seven exact function starts), and 44 appeared (30 exact starts and 14 new AIF
mid-body addresses). No unknown-address inventory changed.

The preserved abace sweep repeats all 141 images against merged main `abace009d9` and
the frozen default-off implementation, with the option explicitly enabled in
the candidate arm. All queries complete, and both entry inventories for every
image exactly match the first sweep. The 21 changed images, 399 removals,
44 additions, and seven exact-start losses therefore remain the preserved discovery scope.

Those seven lost starts are real functions. Newly decoded cases restore their
caller's frame and tail-jump to them; the ordinary walk decodes the targets as
instructions before AIF can recognize their entries. They are not table-case
labels or improvements to count as false positives:

| Image | Lost function entry | Tail jump in newly reached case |
|---|---|---|
| crypt32 | `_url_matches` `0x6e8d0210` | `0x6e8d0a25` |
| crypt32 | `_CRYPT_CopyAttr` `0x6e8fd540` | `0x6e8fedd5` |
| jscript | `_disp_cmp.part.0` `0x6e3e0dc0` | `0x6e3e99b9` |
| gdi32 | `_copy_bitmap` `0x6c9cb6c0` | `0x6c9cbbe9` |
| msi | `_InstallerImpl_Version.constprop.0` `0x650a6240` | `0x650a96bf` |
| msi | `_STREAMS_delete_row` `0x6511a9e0` | `0x6511ac4a` |
| user32 | `_WDML_ServerHandleTerminate.constprop.0` `0x6ed232a0` | `0x6ed247c9` |

This blocks a default-on recommendation. The option stays off; successful table
proofs do not imply that every ordinary successor improves function-entry
recall. The existing `tailcallentry on` option did not recover any of the five
jscript/gdi32/msi/user32 losses and left their complete entry sets unchanged;
that existing recovery is ARM-gated. Adding a new tail-call policy or a
count-based suppression is outside this feature. The preserved older association of 466 Wine and 49 MinGW entries
with unresolved computed jumps was not a prediction of improvement.

## Emitted C limitations

The whole-image comparison retains both natural function inventories and
explicit-address supplements for every removed or gained body. It also
compares all returned function metadata, including variables, types, stack
objects, line mappings, and unstructured gotos. The original five-field
metadata projection is not the final review surface.

Enabling the option can make emitted C worse even when the local table proof
is valid. In crypt32, the newly discovered entry `0x6e8eea21` makes
`funcboundflow` truncate `0x6e8ee740`: branches for values five and eight lose
their `memcpy` calls. At `0x6e904a20`, case five loses its zero/nonzero special
path and the `0x2000003` call, leaving an empty branch and an unconditional
`0xa000003` path. An explicit request at the removed fragment `0x6e8fedaf`
also loses its tail-return to `0x6e8fd540`, although the natural parent recovers
that call. These are unsafe opt-in regressions, not harmless formatting or
successful native translations.

Kernelbase's `GetComputerNameExW` at `0x7b05af90` also regresses: a former
`short[260]` buffer becomes a four-byte scalar passed with a count of 255, then
scanned and copied beyond that emitted object. Its returned variable metadata
splits the 520-byte frame span into separate objects; those objects do not
make the emitted scalar access safe. New coverage can therefore worsen both
boundary handling and inferred C objects.

The review also retains adverse GDI and user32 results. GDI's six-argument
wrapper at `0x6ca62180` becomes a void, zero-argument forwarding body with
uninitialized temporaries. User32's `0x6ed38460` changes a one-argument
`IsWindow` call into a two-argument call and loses an existing conditional
notification log at a new boundary. At `0x6ed50680`, a 28-byte buffer shrinks
to 12 emitted bytes while its `GetObjectW` call still requests 24. These
examples are part of the opt-in risk, not ignored differences in the review.

Other expanded bodies expose existing imperfect call and input inference.
For example, MinGW FirstCrackMe's recovered `___gdtoa` uses the result of
`sub_408f00(1)`, while that callee's unchanged baseline explicit-address
translation has a void signature. Newly reached Wine paths can retain
uninitialized register-input temporaries. The runnable fixture's native
agreement does not establish whole-corpus C correctness. These limitations,
in addition to the seven lost true starts, keep the option off in every mode;
this feature adds no new function-boundary or tail-call policy.

## Preserved abace validation and timing

Validation uses merged main `abace009d9` and the immutable default-off runtime
built from source tree `c62b54fb`. The ordinary datatests retain 675/675 and the
fresh stages retain 3,145/3,145; a real default-on rebuild also changes zero of
675 assertions. The workspace passes 9,197 tests with 38 ignored and no
failures. Focused checks pass 27 proof/integration tests and nine console
fixture tests. Independent checks cover 20 ELF and six PE controls, 16 resume
mutations, initial and later budget rollback, and 44 supported native/emitted
comparisons. Preexisting wrong-C controls remain separately classified.

The 21 changed images supply 41,275 address comparisons: 41,231 natural
baseline records plus 44 forced addresses, and 40,876 natural enabled records
plus 399 forced addresses. Every removed address and all seven true-start
losses remain covered. All 726 changed comparisons and 1,482 hunks (762 code,
720 full semantic metadata) have individual classifications; the strict
offline verifier reconstructs their exact identities from saved raw results.
Seventeen addresses fail the 120-second watchdog in both arms: fifteen in MSI,
one in GDI, and one in kernelbase. Thirteen report a different stop action and
four report identical errors. None is a success-to-failure transition or an
error repair. Those errors and the unsafe C above remain in the result.

Timing compares the actual-main trio, default-off candidate trio, and explicit
on candidate on the same eight physical cores, with one warmup and five
interleaved samples per arm. Each row retains all output, input/spec/trio hashes,
CPU topology, external compute/load, selected-core activity, child CPU time,
and wall-time ranges. Unrelated campaigns remained active on the shared host:
these are contention-logged measurements (`quiet=false`), not a controlled
default-on budget certificate. Whole-C queries use four jobs and `protoorder
off`. The causal fixture additionally uses aggressive mode, `fast_funcdisc
off`, and `listing on`; other images use auto mode.

| Image / surface | Main / off / on median wall ms | Off vs main wall / CPU | On vs off wall / CPU |
|---|---:|---:|---:|
| Causal i386 / discovery | 128.75 / 129.25 / 128.02 | +0.38% / +0.45% | -0.95% / -0.99% |
| Causal i386 / whole-c | 302.47 / 328.42 / 343.23 | +8.58% / +1.28% | +4.51% / +1.57% |
| MinGW FirstCrackMe / discovery | 229.65 / 229.23 / 227.00 | -0.18% / -0.13% | -0.98% / -0.97% |
| MinGW FirstCrackMe / whole-c | 1711.11 / 1635.55 / 1344.52 | -4.42% / -7.28% | -17.79% / -13.23% |
| Wine advapi32 / discovery | 349.57 / 351.75 / 434.87 | +0.62% / +0.64% | +23.63% / +23.68% |
| Wine advapi32 / whole-c | 2380.38 / 2217.40 / 2391.14 | -6.85% / -8.16% | +7.84% / +9.57% |
| ARM control / discovery | 72.61 / 77.29 / 75.40 | +6.44% / +3.93% | -2.44% / -0.20% |
| ARM control / whole-c | 211.90 / 237.20 / 228.97 | +11.94% / +14.46% | -3.47% / -5.09% |
| x64 control / discovery | 146.34 / 149.34 / 141.45 | +2.04% / +2.29% | -5.28% / -5.48% |
| x64 control / whole-c | 308.79 / 307.53 / 358.60 | -0.41% / +7.91% | +16.61% / +5.21% |

Predeclared fifteen-sample follow-ups retain every sample separately rather
than pooling away the initial outliers:

| Follow-up / surface | Main / off / on median wall ms | Off vs main wall / CPU |
|---|---:|---:|
| ARM control / discovery | 88.74 / 77.80 / 86.73 | -12.32% / -12.13% |
| x64 control / whole-c | 382.67 / 381.90 / 383.12 | -0.20% / -0.27% |
| ARM control / whole-c | 227.80 / 223.67 / 229.22 | -1.81% / -4.60% |

The ARM and x64 follow-ups do not reproduce the initial positive default-off
CPU outliers; their wall ranges overlap broadly. The causal whole-C row still
records +8.58% wall and +1.28% CPU for off versus main. The disabled code path
skips constructing the table reader, but these shared-host samples do not
establish a tight global overhead bound. Enabled Wine discovery is +23.63%
wall and +23.68% CPU, beyond the five-percent budget; its whole-C row is also
slower. The machine record uses that conservative Wine discovery result as
its speed headline. Recall losses, observed C regressions, and this timing
result all support leaving the feature off in every mode.

The first pinned attempt remains preserved but is excluded from the primary
wall comparison: Python's timed subprocess wait quantized short timings. A
small sleep control demonstrated that distortion; the primary and follow-ups
use blocking waits under the same external timeout wrapper. No samples from
that attempt are substituted into the tables above.

## Revalidation after the project-export change

That revalidation used base `7f69becc`, source tree `f5733318`, which adds function
linkage names to project exports. The 25 implementation, fixture, metadata,
and stage paths of this feature remain byte-identical to the reviewed
`c62b54fb` snapshot. Its rebased default-off runtime is frozen from tree
`70326c69`; only three evidence documents change after that freeze.

Fresh checks pass 9,202 workspace tests with zero failures and 38 ignored,
675/675 ordinary assertions, 3,145/3,145 stage assertions, 27 focused tests,
and nine console tests. A real default-on rebuild again changes zero of 675
assertions and activates the 28-function fixture. The complete 141-image
survey has zero errors; all 744 arm inventories and classifications match
the preserved abace results, including the seven lost true starts.

The nine complete raw JSON documents for causal i386, MinGW FirstCrackMe,
and Wine advapi32 across main/off/on match their old counterparts with no
fields removed. Twenty separate controls pass 120 fresh CLI queries, including
all function metadata and preexisting unsupported errors. The entire
`result_json` writer is unchanged by the project-export commit; its new
name/reservation logic is guarded by `header_carries_types`, which the ordinary
whole-C query does not enable. The full 21-image, 726-comparison, 1,482-hunk
manual audit above remains explicitly an abace measurement. No new full
21-image query or whole-corpus native equivalence is claimed.

The additional project-mode interaction test selects the discovered dispatcher
and helper by name. Serial and two-worker project artifacts are byte-identical,
and complete serial/parallel JSON records agree. Streaming preserves exact C
function blocks, prototypes except their order, index spans, and comments under
the documented relocation. Untouched generated C and headers from all three
exports compile at O0 and O2: six native executables pass all seven inputs and
the helper marker checks. This exercises the new project-header path without
changing the earlier wrong-C classifications.

Fresh actual-base timing again uses five interleaved samples and one warmup
for every arm, one preselected set of eight physical cores, matching complete
trios, blocking waits, and complete pre/post input and spec hashes. External
work remains active, so `quiet=false`; ranges, CPU/load/contention evidence,
and every sample are retained separately from the earlier base.

| Image / surface | Main / off / on median wall ms | Off vs main wall / CPU | On vs off wall / CPU |
|---|---:|---:|---:|
| Causal i386 / discovery | 177.66 / 174.47 / 115.74 | -1.80% / -2.53% | -33.66% / -32.67% |
| Causal i386 / whole-c | 371.67 / 334.42 / 369.65 | -10.02% / +2.54% | +10.53% / -7.45% |
| MinGW FirstCrackMe / discovery | 193.57 / 194.53 / 190.35 | +0.50% / +0.57% | -2.15% / -2.09% |
| MinGW FirstCrackMe / whole-c | 1290.40 / 1294.44 / 1287.06 | +0.31% / -1.44% | -0.57% / -1.77% |
| Wine advapi32 / discovery | 473.81 / 470.35 / 508.13 | -0.73% / -0.76% | +8.03% / +8.06% |
| Wine advapi32 / whole-c | 2394.81 / 2480.62 / 2341.11 | +3.58% / +3.76% | -5.62% / -4.66% |
| ARM control / discovery | 79.35 / 79.73 / 82.58 | +0.48% / +0.81% | +3.57% / -2.06% |
| ARM control / whole-c | 232.10 / 229.09 / 203.97 | -1.30% / -0.58% | -10.96% / -8.30% |
| x64 control / discovery | 165.19 / 158.10 / 147.48 | -4.30% / -1.86% | -6.71% / -7.38% |
| x64 control / whole-c | 547.08 / 600.80 / 631.02 | +9.82% / +9.28% | +5.03% / -0.74% |

The predeclared Wine discovery headline is +8.03% wall and +8.06%
child CPU for on versus off. These shared-host results do not certify a tight
global overhead bound. The option remains off in every mode for the observed
recall and C risks as well as its enabled cost. #299 remains open.

The predeclared fifteen-pair x64 whole-C follow-up keeps all three arms and
all samples separately: main/off/on medians 468.24/493.14/574.08 ms,
off versus main +5.32% wall and +2.33% CPU. The initial
+9.82% wall/+9.28% CPU row remains in the primary table. Broad overlapping
ranges and contention prevent a tight global performance claim.

## Original fixed-byte evidence

The public fixture `decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386`
has a four-entry absolute-pointer table. `dispatch` starts at `0x100000`, checks
`eax <= 3`, and dispatches at `0x100009`. Its case starts are `0x100010`,
`0x100017`, `0x10001e`, and `0x100025`; the default starts at `0x10002c`.

The existing reference reader reaches each case:

```sh
decompiler/target/release/kuna xrefs \
  decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386 \
  --to 0x100010 --kind jump --json
```

A native API comparison, with both walks seeded only at `0x100000`, measured:

| Fact | `Listing::build` | `xrefs::build` |
|---|---:|---:|
| Decoded instructions | 11 | 19 |
| Four table case starts decoded / reached | 0 | 4 |
| Statically reached default decoded | yes | yes |

The Listing instruction query was `is_instruction_start(case)`. The xrefs query
was a `Jump` reference from `0x100009` to each case. This measures missing code
coverage, not an AIF count reduction: this tiny fixture does not supply AIF's
minimum twenty-function fingerprint corpus. Both this API check and the
negative witness below were re-run on base `5fe62be4`, using an own release
build of its exact source tree `d5fb1cb3`.

## Why copying the reference reader is insufficient

The direct-table arm in `listing/xrefs.rs::build_with_focus` calls
`kuna_switchtable::targets` with `cases=None`. That reader scans consecutive
read-only pointers until a word fails its section test or reaches `jumptablemax`.
A neighbouring callback array can contain perfectly valid code pointers.

This runnable negative witness adds such an array after the fixture's four
entries. It builds an isolated temporary ELF; repository files are unchanged:

```sh
python3 - <<'PY'
from pathlib import Path
import subprocess, tempfile
kuna = Path('decompiler/target/release/kuna').resolve()
source = Path('decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386.s').read_text()
source = source.replace('\t.section\t.rodata', '''\t.globl independent
\t.type independent, @function
independent:
\tpushl %ebp
\tmovl %esp,%ebp
\txorl %eax,%eax
\tleave
\tret
\t.size independent, .-independent
\t.section\t.rodata''')
source = source.replace('msg_alpha:\n', 'adjacent_callback_array:\n\t.long independent\nmsg_alpha:\n')
directory = Path(tempfile.mkdtemp(prefix='kuna-x86listingswitch-'))
assembly, binary = directory / 'adjacent.s', directory / 'adjacent'
assembly.write_text(source)
subprocess.run(['gcc', '-m32', '-nostdlib', '-no-pie', '-Wl,-Ttext=0x100000',
                '-Wl,-e,dispatch', str(assembly), '-o', str(binary)], check=True)
subprocess.run([str(kuna), 'xrefs', str(binary), '--from', 'dispatch',
                '--kind', 'jump', '--json'], check=True)
PY
```

Current output includes a fifth jump from `0x100009` to the independent function
at `0x100042`. The guard permits indices zero through three, so that edge is
false. If discovery copied this unbounded scan, it could decode a genuine
unreferenced function as an intra-function successor and stop entry discovery
from admitting it. Lower entry counts would then conceal lost recall.

The bounded register-table path already has `TableRead::stated`, guard decoding,
and pointer-table reads. Reuse those facilities where their proof applies;
their existence does not establish that an arbitrary nearest comparison checks
the actual table index. The new admission rule must establish that connection.

## Cause and ownership

`listing/walk.rs::step` follows static flow and has special table readers for
Thumb and A32. It has no x86 table reader. `StepCtx` and the constructors in
`listing/mod.rs::Listing::build_resuming` therefore cannot supply one.
`listing/kuna_pdecode.rs` builds its own `StepCtx` for each lane and must make
the same decision as the serial walk. A predecessor-dependent reader added only
to the serial loop would violate that invariant.

The xrefs walk has richer p-code capture, register provenance, predecessor
availability, and a retry list for missing predecessor decodes. The minimal
Listing capture retains only each op's first input. Consequently this is an
analysis feature requiring bounded proof and scheduling design, rather than
the omission of one universally safe successor call. P1 owns it; its normative
chapter is `docs/spec/01-program-prep.md`.

## What #299 still requires

The [latest measured issue update](https://github.com/Noelo-Lab/kuna/issues/299#issuecomment-6162124373)
reports AIF-marginal corpus mid-body entries reduced from 11,010 to 1,504 after
already-merged improvements. The original bar included a sixty-percent corpus
reduction and at most one hundred mid-body entries on the original crackme.
The latter remains unmet; reported PE totals remain 1,335 for Wine and 442 for
MinGW crackmes. These are historical measurements, not new measurements of this
proposal.

The preserved earlier classification associates 466 Wine and 49 MinGW mid-body
entries with unresolved computed jumps in their enclosing symbol extents.
Association is not reachability proof or a predicted 515-entry improvement.
Every recovered dispatch and changed function must be checked afresh.

The original 3.4 MB private i386 PE, its IDA 9.2 entry/extent inventory, and the
exact 590 KB i386 crackme plus unstripped twin are not attached or identified by
retrievable path, public URL, or hash in the issue. They are required to verify
those original witnesses. Public fixtures and cached Wine/MinGW twins permit
implementation and corpus validation, but cannot certify the original crackme
bar or close #299. ARM conditional returns, typed-data Listing infrastructure,
and x86-64 delta-table discovery are separate features.

## Latest base and fresh whole-C review

The latest tested base is `dd4f9fde`, with the default-off runtime frozen from
tree `df260df3`. All four switch-recognition files retain their reviewed hashes.
Fresh checks pass 9,243 workspace tests with zero failures and 38 ignored,
675/675 ordinary assertions, 3,151/3,151 stage assertions, 27 focused tests,
and nine console tests. The 141-image discovery survey has zero errors; all
744 arm inventories and classifications exactly match the preserved results.
Native fixture and three project-export checks pass, including six untouched
i386 generated-C O0/O2 executions and their helper-marker checks. The real
global-default-on 675-assertion rebuild remains explicitly labeled with its
`b580cb2f` predecessor; current DD4 also checks the explicit-on fixture/stages.

The fresh 21-image C capture uses predecessor `e265bf23`. It retains all
41,275 address comparisons, 399 removed and 44 gained addresses, and all
seven true-start losses. All 726 changed comparisons and 1,480 hunks
(760 code and 720 complete semantic-metadata hunks) pass strict
reconstruction from saved raw JSON. Prior manual annotations carry only when
the complete comparison or full hunk content is exact; 18 changed contexts
and all 27 of their hunks were manually read again. Every semantic API field
is retained. The boundary and type regressions and other unsafe
opt-in results described above remain limitations.

The same 17 addresses still fail in both arms, with no success-to-error
transition or error repair. The fresh raw results retain 11 `new-error` labels from different stopping
diagnostics and six identical errors; the older 13/4 labels remain
in their historical evidence. All 34 actual failed records report 120 seconds.
Natural queries omit the budget flag; forced-address supplements explicitly
set 120 seconds. Their original commands and diagnostics are preserved. The
13 failed forced MSI interior models are counterfactual requests, not newly
accepted independent functions or repaired natural failures.

The full C capture remains labeled `e265bf23`. Current DD4 validation adds
120 full-record queries on 20 controls and 24 queries/78 records on 13 known
unsafe anchors across crypt32, GDI, kernelbase and user32. Every complete
record matches its predecessor in all three arms, and base/off match. The
21 stripped PE images contain no DWARF sections; the intervening optional
DWARF extent path cannot affect that capture. This bounded composition, full
discovery survey and native checks do not claim a new 21-image DD4 C capture
or whole-corpus native equivalence.

Current-base timing keeps every sample separately: one warmup and five
interleaved samples per arm/surface on the same preselected eight physical
cores, with all six trio artifacts, manifests, fixture/spec bytes and outputs
pinned. External campaigns and an allowed sequential offline verifier remain
observed, so `quiet=false`; these measurements do not certify a tight global
overhead bound.

| Image / surface | Main / off / on median wall ms | Off vs main wall / CPU | On vs off wall / CPU |
|---|---:|---:|---:|
| Causal i386 / discovery | 109.07 / 107.90 / 125.60 | -1.07% / -0.65% | +16.40% / +15.71% |
| Causal i386 / whole-c | 315.44 / 310.62 / 311.46 | -1.53% / -3.36% | +0.27% / +3.45% |
| MinGW FirstCrackMe / discovery | 225.76 / 226.46 / 171.00 | +0.31% / +0.31% | -24.49% / -24.69% |
| MinGW FirstCrackMe / whole-c | 1270.59 / 1256.51 / 1317.12 | -1.11% / +3.99% | +4.82% / -3.63% |
| Wine advapi32 / discovery | 467.49 / 469.14 / 502.80 | +0.35% / +0.38% | +7.17% / +7.17% |
| Wine advapi32 / whole-c | 2208.09 / 2172.00 / 2228.29 | -1.63% / -3.58% | +2.59% / +0.63% |
| ARM control / discovery | 72.34 / 74.82 / 75.43 | +3.43% / +3.43% | +0.82% / +0.26% |
| ARM control / whole-c | 222.62 / 224.83 / 223.94 | +0.99% / +1.02% | -0.39% / +0.36% |
| x64 control / discovery | 144.53 / 141.09 / 143.77 | -2.38% / -0.90% | +1.90% / +1.48% |
| x64 control / whole-c | 370.82 / 352.28 / 370.42 | -5.00% / +3.67% | +5.15% / -8.14% |

The predeclared Wine discovery headline is +7.17% wall and
+7.17% child CPU for on versus off. All ranges and contention
records remain in the machine record. The feature remains off in every mode;
local switch admission does not establish correct whole-function C, and the
original private PE/crackme acceptance for #299 remains unverified.

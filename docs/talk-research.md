# Kuna development stories for a reverse engineering talk

Kuna's strongest story is a decompiler whose behavior agents can investigate, change, and replay. The most useful examples combine a difficult native-code case with a visible improvement to emitted C and an executable check that the improvement preserved the program. For a technical reverse-engineering audience, lead with SIMD reductions and reused stack storage, then show how benchmark mining and agents solving crackmes generated the work.

Evidence checked on 2026-10-18 against repository commit `99d4b7f1e` and GitHub PR state. Measurements retain their original corpus and build; the fresh local replays below use an existing release build, identified separately under Reproduction evidence. Ranking is editorial: technical interest, practical reach, visible before/after, and strength of the supporting evidence.

## PRs worth showing

| Priority | Story and PRs | Where the work came from | Evidence worth putting on a slide |
|---|---|---|---|
| 1 | Recover a scalar loop from a vectorized integer reduction — [#1010](https://github.com/Noelo-Lab/kuna/pull/1010) | Contributor `ReversingWithMe`; commit records name an agent coauthor, followed by an automated maintainer correction | A vector arm, per-lane accumulators, horizontal fold, and remainder loop become one byte-sum loop. The merge review records generated-C execution against 26 reductions under nine compiler configurations. |
| 2 | Give reused storage separate logical names and types while retaining aliases — [#977](https://github.com/Noelo-Lab/kuna/pull/977) | Contributor `mgostIH`, from independently authored reproductions in [#809](https://github.com/Noelo-Lab/kuna/issues/809) and [#810](https://github.com/Noelo-Lab/kuna/issues/810) | The same eight stack bytes hold two integers and later a pointer. The old output assigns a pointer to a struct and cannot compile. Typed views over one union preserve the address and produce the native result, 67. |
| 3 | Recover types and structures across real stripped programs — [#655](https://github.com/Noelo-Lab/kuna/pull/655), [#682](https://github.com/Noelo-Lab/kuna/pull/682), [#669](https://github.com/Noelo-Lab/kuna/pull/669), and the campaign reported in [#771](https://github.com/Noelo-Lab/kuna/pull/771) | A directed type-recovery campaign measured with decbench and additional instruments | Across 10,748 fixed functions, perfect `type_match` results rose from 848 to 1,674; 3,539 functions improved and 17 worsened. This is a campaign result, not the effect of structsynth alone. |
| 4 | Turn a packed PE into an analyzable image — [#488](https://github.com/Noelo-Lab/kuna/pull/488) | RE-friction round 6: an agent solving a crackme could not reach the payload | Original witness: 1 discovered function before unpacking, 167 after, with 79 imports and three resource leaves restored. An additional corpus image went from 7 to 423 functions. Fresh replay: 2 to 168 on the original fixture. |
| 5 | Stop an unresolved Windows API call from swallowing the next function — [#254](https://github.com/Noelo-Lab/kuna/pull/254) | decbench case `O0-mydoom-mydoom-mmsender_th`, with rival panes exposing a structural discrepancy | Unnamed import calls fell from 927 to 8 across two measured PEs. Two functions reached GED 0, from 37 and 39. The essential fix was restoring no-return information, not merely printing API names. |
| 6 | Find firmware functions without cutting existing functions apart — [#255](https://github.com/Noelo-Lab/kuna/pull/255), following [#239](https://github.com/Noelo-Lab/kuna/pull/239) | decbench ARM entry-recall measurements, using matching DWARF twins | On 48 stripped Cortex-M images: 1,613 additional ground-truth entries; 95.2% of new entries matched ground truth; zero measured mid-body splits and zero lost entries. |
| 7 | Let an analyst supply recovered code bytes as a persistent fact — [#538](https://github.com/Noelo-Lab/kuna/pull/538) | RE-friction round 12: a loader rewrites 2,967 bytes above itself | `--assert 'bytes ADDRESS @FILE'` overlays plaintext in the load image before analysis. A fresh regression replay changes the selected function to `return 0x2a;`. The original executable is unchanged. |
| 8 | Repair invalid variable identities that a CFG score cannot see — [#257](https://github.com/Noelo-Lab/kuna/pull/257) | decbench NOVEL-pool inspection of Kuna's own output | 2,142 `$$undef` occurrences disappeared over 116 binaries and 90,808 functions. The witness's GED remained 8.0. The defect included reading an unwritten alias of a variable written under another name. |

## Recovering a source loop from SIMD

[#1010](https://github.com/Noelo-Lab/kuna/pull/1010), merged 2026-10-12, is the most visually compelling new transformation. Compilers turn an ordinary byte sum into a length guard, vector reduction, horizontal addition, and scalar remainder. Kuna's `wideslice` first simplifies artificial 9–15-byte SSA values left by vector-register heritage; `devectorize` can then recognize a restricted integer reduction and discard the vector path in favor of the already present scalar loop, starting at zero.

The interesting technical point is the proof required before removing that edge. The matcher checks the initial values, stride, load slices, complete nonoverlapping lane coverage, accumulator fold, exits, and uses escaping the arm. Integer addition is associative modulo the accumulator width. Floating-point reassociation would need a different argument and is outside this matcher; saturating and sign-extended reductions also decline. This is recovery of a particular compiler transformation, rather than general SIMD understanding. The current algorithm and guards are in [kuna_devectorize.rs](../decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_devectorize.rs) and [the P3 specification](spec/03-ssa-and-simplification.md).

The fresh Clang 14 reproduction has a stride-four vector arm. With the option off, it contains:

```c
if (4 <= a1) {
  /* initialize four accumulators */
  do {
    v1 = *(uint4 *)(a0 + v3);
    v2 += v1 & 0xff;
    v5 += v1 >> 8 & 0xff;
    v6 += v1 >> 0x10 & 0xff;
    v7 += v1 >> 0x18;
    v3 += 4;
  } while (v4 != v3);
  v2 = v7 + v5 + v6 + v2;
  /* then handle the remaining bytes */
}
```

With `--option devectorize on`, the same native function becomes:

```c
if (a1) {
  v2 = 0;
  v1 = 0;
  do {
    v1 += (uint4)a0[v2];
    v2 += 1;
  } while (a1 != v2);
  return v1;
}
return 0;
```

The contributor's implementation commit records a Claude Fable coauthor. The subsequent maintainer correction records a Claude Opus coauthor and an automated review comment: the first implementation could lose global stores when the vector and scalar exits had different effects. The merged guard declines those cases. That makes this an unusually good agent-development story: a substantive transform, an adversarial semantic counterexample, a repair, and a permanent regression. The review also records recompiling the output of 26 reductions under nine Clang/GCC configurations and comparing their behavior on random inputs. Those are PR measurements; the fresh replay here checks the visible transformation and its 12 focused stage assertions.

A fresh comparison on the identical Clang 14 ELF gives a concrete result against Ghidra 12.1 through DecLib with its default adapter settings:

| Output | Artificial 13 byte arrays | Loops | Lines of emitted C |
|---|---:|---:|---:|
| Ghidra 12.1 | 4 | 2 | 65 |
| Kuna with both rules off | 3 | 2 | 59 |
| Kuna with wideslice only | 0 | 2 | 41 |
| Kuna with wideslice and devectorize | 0 | 1 | 16 |

The [full output capture](talk/simd-evidence.json) includes the input hash, executable hashes, commands, and all four bodies. Recompiling the exact final body with GCC and Clang at O0 and O2 passed 1,569 comparisons per build, 6,276 total, against the source compiled with Clang O3. Inputs include vector handoff lengths, zero and maximum byte values, deterministic pseudorandom data, and lengths crossing the signed-sum boundary and unsigned wrap. Run `bash docs/talk/verify-simd.sh` to repeat these behavioral checks on the captured body. This establishes a useful witness where Kuna recovers a form this Ghidra configuration does not; it does not establish superiority across SIMD code generally.

`wideslice` is default-on; `devectorize` is default-off and enabled by the aggressive preset. A live demo should specify both `--mode reliable` and `--option devectorize off|on`, so the preset does not decide the experiment.

## Reused stack storage is a problem of identity and aliasing

The contributor reports distinguish two defects. In [#809](https://github.com/Noelo-Lab/kuna/issues/809), two disjoint register lifetimes already have separate printed variables, but name assertions reject both because the physical storage is shared. In [#810](https://github.com/Noelo-Lab/kuna/issues/810), one eight-byte frame range first holds `Pair {5,7}`, then a pointer to the integer 55. An assertion supplies the two helper prototypes, yet the old output still declares a whole-function `Pair` and emits:

```c
v2[0].x = 5;
v2[0].y = 7;
v1 = read_pair(v2);
v2[0] = &datum;
return read_handle((LifetimeHandle *)v2) + v1;
```

That assignment is invalid C. Simply allocating separate independent locals would introduce a different error when an escaped pointer can still observe writes to the original bytes. [#977](https://github.com/Noelo-Lab/kuna/pull/977), merged 2026-10-13, instead gives names and types logical identities while retaining shared physical backing for overlapping stack views. On the fresh replay, the declaration becomes a union and the calls use its `LifetimePair` and `LifetimeHandle` members. The native and generated-C regressions cited in the PR return 67 under GCC and Clang at O0 and O2. The [self-contained reproduction](features/lifetimes/README.md) includes the assembly and assertions.

This is also a useful bounded comparison with another decompiler: the contributor tested a pinned angr build, which handled the register-rename case but emitted invalid aggregate C for the stack case. Those are two named witnesses against one recorded angr version, not an overall tool ranking. IDA already has [Split variable](https://docs.hex-rays.com/8.4/user-guide/decompiler/interactive/cmd_split_lvar), and Binary Ninja documents [automatic and manual variable splitting](https://docs.binary.ninja/guide/index.html#merging-and-splitting-variables). The contribution to emphasize is the integration of lifetime-aware assertions, typed aggregate views, and alias preservation, demonstrated on this case.

`stackviews` remains opt-in. Its published performance measurement was +7.6% CPU on a seven-function workload, predating the final refresh; it is not a general cost bound. Related work continued in [#1022](https://github.com/Noelo-Lab/kuna/pull/1022) and [#1151](https://github.com/Noelo-Lab/kuna/pull/1151). Treat this as an important capability with conservative boundaries, not completed recovery of all source-language lifetimes.

## The type campaign offers the strongest aggregate result

The [campaign results](decbench/typecampaign/results.md), [raw summary](decbench/typecampaign/final-l/analysis.json), and [#771](https://github.com/Noelo-Lab/kuna/pull/771) support a useful same-tool result. Between baseline `809712e9` and measured build `632437155`, the pinned 444-slice instrument covered the same 10,748 functions:

| Measurement | Baseline | Campaign final |
|---|---:|---:|
| Functions perfect under `type_match` | 848, or 7.89% | 1,674, or 15.57% |
| Mean `type_match` | 0.2645 | 0.3752 |
| Ground-truth variables matched | 16,676 of 65,715 | 22,373 of 65,715 |
| Casts on the fixed 4,815-function comparison set | 40,583 | 32,073 |
| Claimed struct-field precision and recall | No synthesized records | 0.8713 precision, 0.0932 recall |

The perfect-function count nearly doubled, and mean score rose 41.9%; 3,539 functions improved and 17 worsened. This prices a sequence of changes: structure synthesis, library types, callee-first type propagation, pointer and boolean recovery, and cast rules. Attributing the whole improvement to any single PR would be wrong.

The feature-level example is [#655](https://github.com/Noelo-Lab/kuna/pull/655): constant-offset dereferences of a pointer become members of a synthesized structure. Its `ls -O2` witness recovered nine of the ten members of gnulib's `hash_table` at their real offsets. [#682](https://github.com/Noelo-Lab/kuna/pull/682) subsequently enabled parameter synthesis by default, and later changes added shared layouts and local objects. These initial PRs illustrate why the dataflow/type fixed point matters: synthesizing too early, or failing to notify the high-variable type cache, leaves the output unchanged or commits the wrong shape.

For the cast portion, the public base64-decoder fixture is a clear slide: [the recorded witness](decbench/typecampaign/final-l/b64-witness.md) falls from 33 casts to 7 and changes raw pointer arithmetic into `a0[a1 - 1]` and `table[(unsigned char)a0[v7]]`. The fresh local run still shows those subscripts. Ghidra already documents [creating structures from decompiler uses](https://ghidra.re/ghidra_docs/GhidraClass/Advanced/improvingDisassemblyAndDecompilation.pdf); the interesting Kuna result is automatic integration, explicit policies, and measured corpus effects.

The limitations are part of the result. Claimed field precision is high, but recall is only 9.32% and nesting F1 is 0.0036. The final measurement also exceeded the campaign's original +5% speed budget on four measured binaries, reaching +16.3% on `dpkg-divert -O2-noinline`. Small acceptable costs accumulated across rounds. Rival columns were replayed from stored outputs, rather than produced by a fresh simultaneous run of all tools. Present the fixed Kuna baseline-to-final delta as the main aggregate claim.

## Agents solving crackmes generated different features

The [RE-friction loop](re-pipeline.md) asks agents to use Kuna as their primary tool, record a failing workflow, and attach both a predicate reproducing the bad behavior and a predicate expressing the desired behavior. A report is admitted when the former passes and the latter fails on the pinned build. Builders implement the change, and acceptance probes become regression tests. The runbook describes a weaker solver role and stronger builder role; it also records changes to the configured models, so do not imply one model pairing produced every historical PR.

This channel found missing ways to work, as well as wrong C. In [round 1](https://github.com/Noelo-Lab/kuna/pull/367), independent reports converged on a string inventory and disassembly. The resulting string view includes virtual addresses and owning functions, and function triage reduced one measured PE's JSON from 174 KB to 2.8 KB. This is an agent-usefulness result: the reverse engineer gets a manageable starting point and can cross-check decompilation against bytes.

The stronger engine story is [#488](https://github.com/Noelo-Lab/kuna/pull/488), merged 2026-09-07. The tester suspected missing packer metadata. Refutation found an intact UPX header at offset `0x3e0`: Kuna had only searched the ELF-shaped tail window and had no PE unpacking implementation. The repair reconstructed the PE image, imports, and resources, including a witness whose import directory was a decoy. Recovering decompressed bytes alone would have produced a plausible but incomplete image. The PR records 79 imports, three resource leaves, and an additional image improving from 7 to 423 discoverable functions. Fresh replay on the checked-in witness gives 2 packed functions and 168 unpacked functions; the original PR recorded 1 and 167.

[#538](https://github.com/Noelo-Lab/kuna/pull/538), merged 2026-09-09, arose from a different loader: an agent had recovered a 2,967-byte plaintext layer but needed a patched executable to analyze it. The new byte assertion makes the recovered contents a replayable input fact and applies them before lifting. It enables analyst-assisted unpacking; it does not evaluate arbitrary runtime decryption automatically. The open [runtime-decryption need](re-needs/runtime-decrypted-code-opaque.md) asks for the latter on a separate witness.

Built-in unpacking, overlays, strings, and cross-references are familiar capabilities elsewhere. Their value here is the measured removal of actual agent blockers and their integration into one command and assertion vocabulary. The evidence does not support calling these individual features unique inventions.

## Benchmark mining found both structural and semantic defects

[#254](https://github.com/Noelo-Lab/kuna/pull/254) is the best compact illustration of a benchmark case leading to a root-cause investigation. A Windows `call [IAT slot]` stayed indirect because the deindirection gate read a flag that no code set. A second missing link dropped the resolved callee's no-return property. Restoring both lets `ExitThread(0)` terminate the function, instead of incorporating the following function's body. The [feature record](features/peimportcall/record.json) records the 927-to-8 unnamed-call reduction and both exact GED improvements. This is repair of missing Ghidra-derived behavior, with broad practical impact.

[#255](https://github.com/Noelo-Lab/kuna/pull/255) shows why the campaign needed recall measurements in addition to GED. A missed function has no emitted CFG to improve. Its [DWARF-grounded record](features/ptrentry/record.json) measures 38,238 to 39,851 recovered entries out of 41,378. The accepted heuristic uses containment evidence; an explored reseeding design introduced 974 mid-body splits and lost 15 ground-truth entries. The option makes the precision/recall decision inspectable. This is a measured firmware-discovery improvement, not evidence that other tools lack code-pointer discovery.

[#257](https://github.com/Noelo-Lab/kuna/pull/257) is the counterexample to optimizing only structure. One stack symbol acquired two cached names: assignments wrote `v12`, while a branch read an unwritten `$$undef...` alias. Restoring the catch-all naming path and repairing the cache removed 2,142 placeholder occurrences, with the measured function count unchanged. Its witness's GED was 8.0 in both arms. The [record](features/undefname/record.json) distinguishes corpus breadth from the smaller full-diff audit.

The round-2 [feature ledger](decbench/features.md) records nine merged items: three moved the mined-case GED, four corrected problems GED could not see, and two improved recall. That is a stronger methodological finding than another small leaderboard gain.

## Failures that deserve a slide

These are documented failure modes with technical consequences, rather than a claim that every historical defect remains in current main.

| Failure | Concrete evidence | What it means for the development approach |
|---|---|---|
| A proposed mechanism can fire and still be wrong | During [#250](https://github.com/Noelo-Lab/kuna/pull/250), three designs lost an assignment, an early return, or the correct return value; the witness alone did not expose them. [Campaign account](decbench-loop.md). | Refutation must test semantics and every changed function, not just prove that the proposed hook affects output. |
| A new simplification can erase effects | The correction merged in [#1010](https://github.com/Noelo-Lab/kuna/pull/1010) guards reductions whose exits write global storage. | Pretty scalar C is insufficient evidence; preserve observable stores before removing a vector path. |
| A useful-looking structural change can hurt the aggregate metric | [#136](https://github.com/Noelo-Lab/kuna/pull/136) reverted return duplication to default-off after about 976 fewer GED-perfect results. | Source structure and a preferred early-return style disagree. A per-function choice is valuable even when a global default loses. GED loss alone does not establish semantic wrongness. |
| An executable probe can report a false fix | Round 1's exact `_secret_function(v2)` probe stopped matching when the local became `v3`, while the defect remained. [Recorded account](re-pipeline.md#round-1--what-actually-happened-measured-against-those-criteria). | Stable identities and semantic properties belong in probes; variable numbering should not decide closure. |
| Correctness costs accumulate | The type campaign's final speed was +8.9% to +16.3% on four measured inputs despite acceptable individual rounds. [Final result](decbench/typecampaign/results.md). | Budget against the original campaign baseline as well as the previous PR. |
| Structural recovery remains incomplete | Final type-campaign field recall 0.0932; nesting F1 0.0036. | A strong demonstration on one record is not comprehensive recovery of program data structures. |
| A usable intervention is not automatic recovery | Byte overlays accept supplied plaintext; [runtime-decrypted stack code](re-needs/runtime-decrypted-code-opaque.md) remains a separate open ledger item. | Distinguish exposing a way for an agent to state a fact from the engine inferring that fact. |

The RE loop also documents weak solve-rate ground truth: only 22 of its 250 challenges were both machine-checkable and uncontaminated in the recorded assessment, and some verifier scripts were unvalidated reconstructions. Its strongest evidence is replayed tool behavior, not a headline crackme solve rate. The committed index records 196 needs with 167 closed, but individual Markdown frontmatter can disagree with that index, and some referenced records are absent. Do not turn that snapshot into a current verified closure percentage.

## What to claim about contributions

The best candidates for deeper technical contributions are the restricted SSA proof for undoing integer vectorization; logical identities and typed shared storage that survive assertions and replay; and an improvement process that turns real RE friction into executable contracts, implementation work, and regressions. These combine existing compiler and decompiler ideas in a product where an agent can inspect and select the policy.

The evidence supports specific wins over recorded outputs and strong improvements over Kuna's own baseline. It does not establish that Kuna is the first tool with variable splitting, structure creation, byte patching, semantic local names, or programmatic decompiler controls. For example, [#1102](https://github.com/Noelo-Lab/kuna/pull/1102) adds useful library-derived local names and records 968 named locals in a 14-binary sweep, but its [proposal](features/libcnames/proposal.md) explicitly identifies existing behavior in other tools. Use it as usability progress, not a novelty claim.

For the talk, the sequence can follow six screens: the SIMD before/after and equivalence conditions; the invalid reused-stack assignment and shared union; the two work-generation loops; the packed-PE and Windows no-return examples; the fixed type-campaign delta; and the failures that changed the methodology. This gives the audience both emitted-code evidence and an account of how the development process found it.

## Reproduction evidence

Fresh replays on 2026-10-18 used the existing executables, not a new build of the inspected HEAD. Their SHA-256 values are:

```text
kuna        23f6db4d8909d99aa8292f913de1e4b46e78edbe3e6df8609b3e38d453727aca
decomp_dbg  96a51dbc4252f55fe3f2634075cc669fca1fe6609184a36b4807c60ef0bd4a19
```

The executable predates the inspected HEAD and exposes 315 catalog entries, while that HEAD's TOML contains 318 option rows. These replays establish behavior of the identified build, not full current-HEAD validation.

The SIMD reproduction source is [talk/byte_sum.c](talk/byte_sum.c). On x86-64 Linux with Clang 14:

```sh
clang -O3 -msse2 -mno-avx -fno-unroll-loops -no-pie \
  docs/talk/byte_sum.c -o /tmp/kuna-talk-byte-sum
decompiler/target/release/kuna decompile /tmp/kuna-talk-byte-sum byte_sum \
  --mode reliable --option devectorize off
decompiler/target/release/kuna decompile /tmp/kuna-talk-byte-sum byte_sum \
  --mode reliable --option devectorize on
```

Focused SIMD regressions passed 12/12:

```sh
decompiler/target/release/kuna test --datatests --datatests-dir tests/stages \
  --name kuna-devectorize --name kuna-devectorize-effects --name kuna-wideslice --json
```

The lifetime replay used the checked-in fixture and assertions; changing `on` to `off` reproduces the incompatible aggregate assignment:

```sh
decompiler/target/release/kuna decompile \
  decompiler/crates/kuna-analysis/tests/fixtures/lifetimes_stack_x86_64 stack_reuse \
  --mode reliable --assert-strict \
  --assert @decompiler/crates/kuna-analysis/tests/fixtures/lifetimes_stack_x86_64.kuna \
  --option stackviews on --option structdefs on --json
```

The byte-overlay replay reached the lifter and returned `0x2a`:

```sh
decompiler/target/release/kuna decompile \
  decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64 0x401140 \
  --addr --define-function '0x401140-0x401146=decrypted_stage' \
  --assert 'bytes 0x401140 b82a000000c3' --assert-strict --json
```

The unpacking replay used [the fixture named by #488](../decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe). Run `kuna functions FILE --json` before and after `kuna unpack FILE -o OUTPUT`; the fresh counts were 2 and 168. Unpacking is static; the packed program was not executed.

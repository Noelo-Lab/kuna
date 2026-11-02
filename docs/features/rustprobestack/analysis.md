# Verified Rust stack-probe effects

The pinned angr general test `test_decompiling_rust_binary_rust_probestack`
requires `linux_encryptor::files::create_note` to retain its filename and lose
the compiler probe. Kuna previously kept the helper and an authored caller that
used R11 returned an undefined local. The decision belongs to P2, before ordinary
call setup and SSA recovery. No source text or helper name identifies the fold.

The static reference helper at normalized address `0x52603` contains exactly
57 bytes, independently disassembled with Ghidra. It probes pages, restores RAX,
RBP and SP, leaves R11 at the final 0..4096-byte residual, writes the saved RBP
below its return slot, and changes arithmetic flags. The immediately following
`sub rsp,rax` overwrites those flags. The supplied executable is never run.

The P2 action requires the exact mapped bytes, the AMD64 register layout, the
canonical direct-call push/store/call sequence, the immediate subtraction, and
a bounded same-block constant RAX setup. It retains the return-address store
and materializes saved RBP, restored SP and R11. The option deliberately
abstracts page-touch reads and faults. Unsupported contexts retain the call.

A counterexample rejected an earlier candidate: materializing the RBP save made
P6 treat its bytes as permanently reserved caller storage. A later 4096-byte
buffer shrank to 4088 bytes. The authored buffer fixture reproduced actual
AddressSanitizer stack-buffer-overflows with GCC and Clang at O0/O2; the parent
passed all 32 value comparisons. The final store carries transient callee-spill
provenance, which excludes it from full-register caller spill
restriction without weakening its memory effects. The complete emitted buffer
body now retains all 4096 bytes and passes those checks.

The final native proof runs twelve complete emitted C bodies unchanged against
the authored executables: 352 register/stack-value comparisons plus 32 ASan
buffer comparisons. Negative controls cover unknown sizes, flag observations,
a different effectful body, altered same-named helper bytes, interior targets,
custom call facts and missing mapping evidence. Selected C, Rust and Go records
also check off/auto/always behavior. The pinned general C witness passes all
four textual assertions, plus the stronger absence of `__rust_probestack`.
The earlier filename failure used Rust output; it remains a separate Rust
literal-rendering limitation. The second-switch case at `0xbcf0` is separate.

The validation and timing results below bound this feature. No full angr parity claim is made.

A fixture audit corrected the assembly-mutated counter's Rust declaration to
`static mut`, as required by the [foreign-static rules](https://doc.rust-lang.org/reference/items/external-blocks.html#statics).
The corrected executable preserves every helper address and all 112 native
observations. Native equivalence and full validation are rerun against it; the
earlier immutable-declaration captures are excluded from final evidence.

A second buffer counterexample covered explicit `stackviews` and `stackalias`.
Their retained physical spill formed a separate scalar at the buffer tail even
without reserving the slot. The summary now declines both policies; the two
option combinations preserve their parent records exactly. This boundary is
part of the proof, rather than a claim that those memory policies were repaired.

The first nine-case study, with 384 paired rounds per arm, failed the default-on budget. The native page, live-R11 and buffer cases had upper wall-time bounds of 5.283%, 6.343% and 5.598%, despite negative median deltas. Both enabled auto/off ablations passed. All observations are retained in `auto-study/`; the option ships off in every preset. No enabled ablation was repeated.

Final validation: the original corpus remains 675/675; stages 3606/3606; the workspace has 10,209 passed, zero failed and 39 ignored across 71 suites. Tooling, lint, catalog and spec checks pass. Fresh captures cover 41 images, 33,695 distinct functions and 75,801 language records. Every default record equals the parent. Explicit auto captures of the three affected images reproduce all 84 previously reviewed records exactly. Twelve inherited Go parser failures retain identical function records. Captures enable the existing godwarfsig option equally in both arms; its off default is unchanged.

The separate changed-OFF-default study used seven cases, 384 paired rounds per arm, three warmups, balanced randomized order and disjoint physical CPU pairs. One final look uses 20,000 block-bootstrap resamples with family alpha 0.05 across 14 CPU/wall bounds. Maximum median CPU/wall deltas are +0.638%/+0.869%; maximum upper mean-ratio bounds are +1.426%/+7.348%. This changed-default study also fails to establish the 5% budget. The unresolved cases are native-page. All results are retained and no unchanged-code repeat is performed; the feature remains explicitly opt-in. Source, binary and all 154 compiled-spec hashes agree before and after sampling. The OFF-study parent is independently built from main c5fcb033 after integrating ARM xrefs and MIPS/PowerPC loader changes. The retained AUTO study used the earlier source-identical PR1437 release at main8332; its failed result is not replaced.

Integration with main `887325c0` adds the PPC32 import option and its 17 stage files. The combined tree has 371 options and 670 XML files. Fresh validation passes 675 original assertions, 3658 stage assertions and 10,276 workspace tests, with 39 ignored and no failures. Fresh parent/current captures again cover 41 images and 75,801 language records with no default changes. Explicit auto reproduces the same 84 fully reviewed records; all 384 native comparisons, including 32 ASan buffer checks, pass again. The [integration record](integration-8873/record.json) binds these checks to the new source and binaries.

The earlier speed studies remain failed results for their recorded builds and parents. They were not repeated after this integration, and do not establish a timing bound for the new binaries. The Rust probe implementation and its off default are unchanged.

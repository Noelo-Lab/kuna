# Plan: bounded function discovery for fast projects

## Goal

Make `fast` whole-binary commands export real internal function bodies on
stripped binaries while retaining a materially cheaper policy than the reliable
Listing, prologue-pattern, and exhaustive AIF combination.

## Design

1. Build one SLEIGH Listing from loader-backed and format-oracle roots.
2. Follow direct calls recursively and register every reached call target.
3. Scan only initialized, allocated, non-executable data for absolute
   code-pointer tables.
4. On non-ARM images, reject tables above 256 slots. If more than 512 unique
   candidates remain, require references from two distinct tables. Validate at
   most 4,096 candidates in independent-table rank order.
5. Require a pointer candidate's two-instruction mnemonic/length fingerprint to
   occur in at least four rooted functions, then require a bounded strict
   subroutine probe. Do not recursively expand pointer-derived roots.
6. Preserve the established Thumb-pointer oracle on ARM.
7. Keep ordinary Listing consumers, full prologue patterns, and exhaustive AIF
   disabled in fast mode.
8. Suppress preset-provided discovery for explicit address selection, because
   that command already has its exact target. Preserve an explicit
   `--option fast_funcdisc on`.

## Verification

- A true two-pass stage test loads public `pdb_prog.exe`, inventories functions
  with the option off and on, and decompiles the newly discovered leaf.
- CLI project tests cover direct-call discovery, pointer-only discovery, the
  option-off controls, exact address scope, and switch-table false positives.
- Analysis unit tests cover distinct-table corroboration, the global candidate
  cap, and oversized-table rejection.
- Native and WASM selector tests pin the address/name discovery policy.
- The private-binary acceptance check requires `0x402d80` in the inventory and
  a nonempty body in the project.

This crosses analysis and front-end policy boundaries, so it is larger than the
usual single-pass increment. The reporter explicitly requested the corrective
implementation after establishing that the shipped fast preset was unusable;
the option remains independently flippable for review and ablation.

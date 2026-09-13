# Implementation plan

1. Vendor and hash the exact KataVM fixture; reproduce default, reliable, and
   correctly spelled `loweredswitch off` behavior.
2. Derive label signedness from required lowered-switch range opcodes and
   extension paths, carry effective width across canonicalization and restart,
   and decline conflicting evidence only while the new option is enabled.
3. Add signed, unsigned, widened, ambiguous, shrinking-projection,
   exact-fixture, full-legacy ablation, and original-chain controls without
   changing the generic printer.
4. Register the default-on option, regenerate catalog/options and stage baseline,
   measure speed, sweep the affected corpus, and run all repository gates.
5. Rebase the reviewed candidate onto serialized `origin/main`; do not open or
   merge a PR while the captain's lease is occupied.

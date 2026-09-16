# F4 elfmain — state

- slug: elfmain / branch: feat/elfmain / worktree /home/mahaloz/kwt/elfmain
- PR: NOT OPENED YET (open with gh pr create ... --body-file .scratch/pr_body.md, then add full-ci label)

## Done
- kuna_elfmain.rs (ElfMainPass) + full gate wiring + PASS_GATES + counters + options.md
- tests/stages/kuna-elfmain.xml (4 asserts), xml.rs 293->294, baseline-stages re-recorded (additive)
- tests/cli/elf-libc-start-main-unnamed.json
- declared_names precedence fix in kuna-console/src/engine.rs (--define-function name wins)
- docs/spec/01-program-prep.md section; docs/features/elfmain/{analysis,plan,record}
- evidence: .scratch/sweep/{summary.txt,diff/,oracle.txt}
- GATES: make test 675/675 PARITY OK; check-spec OK + strict OK; test-cli 171/171;
  catalog --check OK; counters --check no drift
- PENDING: make test-stages (running), make rust-test (lane running, .scratch/rust-test.log)

## Next
1. confirm test-stages + rust-test green, paste lines into record.json gates
2. commit, push, open PR, add full-ci label. DO NOT MERGE.

# Pushan port — progress log

Branch `pushan`, worktree `~/github/kuna-pushan` (main tree is shared with another agent).

## Milestones
- [ ] M1 expression arena + simplifier (`kuna_pushanexpr.rs`) with unit tests
- [ ] M2 abstract state + symbolic p-code emulator (`kuna_pushanstate.rs`, `kuna_pushanemu.rs`)
- [ ] M3 VPC-sensitive exploration + symbolization fixpoint (`kuna_pushanflow.rs`, `kuna_pushanvm.rs`)
- [ ] M4 synthetic p-code emission + SLEIGH overlay (`kuna_pushanemit.rs`, `kuna_sleigh::kuna_pcodeoverlay`)
- [ ] M5 option `pushan` wiring (phases.toml, dispatch, counts), decompile_drive hook
- [ ] M6 eval: just_if / loop / smt_no_solve VMProtect-ultra, then Themida, then the rest
- [ ] M7 stage test + spec chapter + options.md + gates + PR

# w9-con-ifacedecomp
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-console` -> ok, 85 passed (81 pre-existing + 4 verifier adversarial); 0 failed. `cargo clippy -p kuna-console --tests -- -D warnings` -> clean.

## Scope and blob integrity

C++ in scope: `decompiler/cpp/ifacedecomp.{cc,hh}` (3687 + 681 LOC). Blob SHAs at
HEAD match the checklist `cpp_blob_sha` exactly:
`ifacedecomp.cc e62ab6ab...`, `ifacedecomp.hh 5958f67c...`. The item commit is
`758d900` (`rport/w9-con-ifacedecomp`); `interface.rs`/`ifaceterm.rs` in the
branch diff belong to the prior w9-con-interface item (separately verified) and
are out of scope here. In-scope Rust: `ifacedecomp.rs` (1847 LOC) +
`ifacedecomp/tests.rs`.

## Shape of the port

`ifacedecomp.rs` ports the 96 `Ifc*` decompiler commands + the base-module
wiring as a registration-table + parse-order + diagnostic-string port. The
engine-touching tail of essentially every command (decompile drive, print
stack, the `parse_machaddr`/`parse_C` grammars, the `Architecture`
loader/types/context/`ArchOptionContext` surface) routes through
`engine_unavailable` — a genuine cross-item dependency, ledgered as **LOSS-127**
(see verification below that the surface is truly absent from the merged tree).

## Hunt list

- Signedness: clean — no signed/unsigned comparisons in scope; the file has no
  arithmetic. (The one int-extraction hazard family, `read_int`, lives in the
  out-of-scope `interface.rs` and is covered by LOSS-125.)
- Integer widths: clean — no `int4`/`uintb` arithmetic in scope.
- Wrapping: clean — no arithmetic ops (grep: no `wrapping_*`/`checked_*` because
  none are needed).
- Comparator totality: N/A — no comparators ported here (command sort lives in
  `interface.rs`).
- Iteration-order provenance: the one ordered loop in scope is
  `register_decomp_commands` (a fixed statement sequence). Verified
  **byte-identical in order and content** to C++ `registerCommands`
  (ifacedecomp.cc:37-147) via positional `diff` of the extracted token lists
  (105 == 105, zero diff), modulo the documented `source` omission (console-only,
  consolemain.cc) and the correctly-skipped `#ifdef CPUI_RULECOMPILE`/
  `OPACTION_DEBUG`/`TYPEPROP_DEBUG` blocks. No HashMap/HashSet anywhere (grep).
- Off-by-one / do-while / reverse iterators: clean — `mainloop` is a faithful
  `loop { while !finished {...}; break-conditions; pop }` transcription of the
  C++ `for(;;){ while(...){...}; if(done)break; if(size==0)break; popScript; }`.
- Erase-while-iterating: N/A in scope.
- Exception -> Result partial-state parity: the `execute()` exception->prefix
  grammar is faithful (IfaceParseError->"Command parsing error: ",
  IfaceExecutionError->"Execution error: ", base IfaceError->"ERROR: ", in C++
  inheritance-catch order), and the ParseError/RecovError/LowlevelError/
  DecoderError arms (with the `abortFunction` side effect on the low-level/decoder
  arms) are transcribed in `render_engine_error`, currently unreachable because
  command bodies fail-closed through `engine_unavailable` (part of LOSS-127). The
  catch placement is preserved.

## Mechanical pass

- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare-`as`:
  all (none) in `ifacedecomp.rs` and the original tests.
- clippy `-D warnings`: clean (lib and tests).
- `port-audit` per protocol: the `port-audit` crate is **not present** in this
  workspace (`cargo run -p port-audit` -> "package not found") — a tooling gap,
  not an item defect; the blob-sha drift check was done directly (matches).

## Faithfulness re-derivation (in-scope, non-engine surface)

Each spot-checked command's pre-engine guard/parse order + exact error text was
compared against the C++ body and found faithful: `IfcOption` (conf guard,
optname, up-to-3 params, "Too many option parameters"); `IfcDecompile`
("No function selected" / "No code for <name>" early-return / "Clearing old
decompilation" / "Decompiling <name>"); `IfcBreakaction`/`IfcBreakstart`
(read-specify-then empty-arg as **IfaceExecutionError** "No action/rule
specified", then conf "Decompile action not loaded"); `IfcMapconvert`
(fd-guard-then-format, "Bad convert format"); `IfcMaplabel` ("Need label name
and address", no conf guard — matches); `IfcRename`/`IfcRetype` (read-then-check,
"Missing old symbol name"/"Missing new name"/"Must specify name of symbol");
`IfcListOverride` ("No function selected" then "Function: <name>");
`IfcSetcontextrange`/`IfcSettrackedrange` ("Missing context variable name"/
"Missing tracked register name"); `IfcContinue`/`IfcProducePrototypes`/
`IfcCommentInstr` (conf-then-fd / conf-then-cgraph guard order). `read_token`
skips leading whitespace, matching C++ `operator>>(string&)`, so the
no-`skip_ws`-before-`read_token` sites (`IfcMapconvert`) are faithful.

## Findings

- F1 (loss, accepted as LOSS-127): every command's post-guard engine call is
  routed through `engine_unavailable`; the engine-error catch arms in
  `render_engine_error` are unreachable; a handful of *pre-engine* errors sitting
  behind an int/grammar read are dropped (`IfcSetcontextrange` "Missing context
  value", `IfcParseFile` "Unable to open file: ", `IfcMapconvert`/
  `IfcMapunionfacet` post-token validation). Consequence: no datatest runs
  end-to-end through this Rust console.
     cpp: decompiler/cpp/ifacedecomp.cc:304,347,889,923,1087 (et al.)
     rust: rust/crates/kuna-console/src/ifacedecomp.rs:175 (engine_unavailable), ~80 call sites
  Verified the missing `Architecture` surface (print/types/loader/context +
  `ArchOptionContext`, `OptionDatabase::set` needing `&mut dyn ArchOptionContext`
  at options.rs:1930, the parse grammars) is genuinely absent from the merged
  rust-port tree — accurate cross-item dependency, single-site wireable later.
- No blockers, no majors. The load-bearing, in-scope surface (registration
  tokens, parse/guard order, exact diagnostic text, console driver) is faithful.

## Adversarial tests added (committed to branch, `2badc70`)

- `w9_con_ifacedecomp_v1_register_token_sequence_byte_identical` — a hand-
  transcribed 105-entry C++ register order; each token line must resolve to
  itself through the real registered prefix tree (catches a single-token
  swap/rename/reorder the `num_commands()==105` count test cannot see).
- `w9_con_ifacedecomp_v2_break_empty_specify_is_execution_not_parse` — the
  unusual empty-arg => IfaceExecutionError (not parse) class on
  `break action`/`break start`.
- `w9_con_ifacedecomp_v3_execute_exception_prefix_grammar` — a synthetic
  command raising each reachable IfaceError kind, asserting the exact
  `execute()` console prefix the harness greps.
- `w9_con_ifacedecomp_v4_deliberate_omissions_absent` — `source` and the
  CPUI_RULECOMPILE-gated `parse rule`/`experimental rules` must not resolve,
  while `parse file`/`parse line` still do (no over-pruning).

All 4 pass; full crate gate stays green (85 passed).

## losses

LOSS-127 (engine-integration deferral; the `engine_unavailable` family). Filed
by this verifier (appended to losses.md, not edited).

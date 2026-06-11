# w4-fw-architecture

## Round 1

verdict: REJECT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_a6e29a1b-137-16 (worktree `.claude/worktrees/wf_a6e29a1b-137-16`, reviewed at 04dde3c; adversarial tests committed as 1d75d3e)
cpp blob shas: all 6 in-scope files match the checklist entry exactly
(`git hash-object` verified: architecture.cc 5bece5e7, architecture.hh
4237825d, capability.cc eaecfcda, capability.hh 2c0701d1, libdecomp.cc
869c131f, libdecomp.hh 1c1dcdd8).

gate (porter submission, my test excluded): `cd rust && cargo test -p
kuna-decomp` -> ok, 309 lib + verifier tests pass; `cargo test -p kuna-sleigh`
-> ok, 200+ pass. The porter's own suite is green.
gate (with adversarial tests): `cargo test -p kuna-decomp --test
verify_w4_fw_architecture` -> 7 passed, 1 FAILED
(`func_default_name_matches_cpp_printraw_oracle`) — the failure is the F1
divergence proof.
clippy: `cargo clippy -p kuna-decomp -- -D warnings` (after touching the 3
item files to force a fresh build) -> clean.
port-audit: `cargo run -p port-audit -- --item w4-fw-architecture` NOT
runnable — no `port-audit` crate exists in the workspace (carried note from
W1/W2 reviews); blob-SHA check performed manually instead (all 6 match).

## Method

Re-derived the semantics of every ported function against the C++ in scope
and diffed `git diff rust-port...worktree-wf_a6e29a1b-137-16 -- rust/`
restricted to the item files: `architecture.rs` (+~600), `capability.rs`
(+256), `libdecomp.rs` (+108) plus their in-module test files. The C++
`Architecture` (1595 LOC) is a god object that reaches W6/W8 subsystems; the
port wires the dependency-available parts (`resetDefaultsInternal` verbatim,
the kuna anchor flags, `buildDatabase` global-scope attach, the injection
library tempbase, `getMinimumLanedRegisterSize`, `nameFunction`, the W3
`new_funcdata` boot seam) and `// SEAM(...)`-notes the rest (`init`/
`restoreFromSpec`, type factory, proto models, print language, loader,
register-data decode). The capability registry (`ArchitectureCapability`
`thelist` + `sortCapabilities`/`findCapability`/`getCapability`) and
`startDecompilerLibrary` are ported in full.

## Hunt list

- **Signedness.** clean. The only signed/unsigned C++ comparisons in scope
  are the capability-registry loops (`int4 i < thelist.size()` in
  `getCapability`/`initializeAll`, `uint4 i/j` in `sortCapabilities`/
  `findCapability`); the Rust replaces all with iterator/`while i <
  self.thelist.len()` walks over a `Vec`, no signed index. No arithmetic on
  the index that could underflow.
- **Integer widths.** clean. `majorversion`/`minorversion` are `uint4` (=6/=1)
  in both. The config fields map C++ `int4`/`uint4` to `int4`/`uint4` 1:1
  (`min_funcsymbol_size: int4`, `max_jumptable_size: uint4`, etc.) matching
  `architecture.hh:170-208`.
- **Wrapping.** clean. No arithmetic on `uintb`-lineage values in the ported
  surface; `reset_defaults_internal` is pure constant assignment.
- **Comparator totality.** n/a — no `operator<`/sort in scope.
  `sortCapabilities` is a find-first-and-rotate, not a comparison sort.
- **Iteration-order provenance.** clean. `thelist` is a `Vec` in registration
  order (C++ `vector<ArchitectureCapability*>` push_back order);
  `initialize_all`, `find_capability_by_file/_by_xml`, `get_capability`, and
  `sort_capabilities` all walk it in index order, matching the C++ `for(i=0;
  i<thelist.size();++i)`. `spec_paths` preserves sleighhome-then-extras order
  (C++ scan-then-`addDir2Path` loop). No `HashMap`/`HashSet` anywhere in scope.
- **Off-by-one / do-while / reverse iterators.** clean. `sortCapabilities`'s
  C++ shift loop `for(j=i+1;...) thelist[j-1]=thelist[j]; thelist[last]=capa`
  is replaced by `remove(i)`+`push`, a provably equivalent stable left-rotation
  (verified by `sort_capabilities_preserves_order_of_many_trailing` and
  `sort_capabilities_raw_at_front_rotates_left`). The degenerate two-"raw"
  case moves only the first (C++ `break` on first match) — verified by
  `sort_capabilities_first_raw_only_moves`.
- **Erase-while-iterating.** n/a — `remove(i)` happens once after the search
  loop terminates, not during traversal.
- **Exception -> Result partial-state parity.** clean. The ported surface has
  no mid-mutation throw site; `new_funcdata` returns `KunaResult` and the only
  `?` is the `Funcdata::new` boundary (W3). The `expect()` calls in `new()`
  (`buildDatabase` scope attach, IR-manager space inserts) are on
  bootstrap-impossible paths, matching the C++ ctor's non-recoverable
  `attachScope`.
- **Aliasing.** clean. `Architecture` owns `Sleigh` by value and forwards
  `manage()` to `translate.base().manager()`; the IR-boundary manager is built
  fresh (cloned spaces) per `build_arch_handle`, documented as the W3 seam.

## Findings

- **F1 (major): the non-angr `func_` default function name diverges from C++
  `Address::printRaw`.** When `name_style_angr` is `false` (the per-function
  upstream-naming option), `Architecture::nameFunction` must emit `"func_" +
  addr.printRaw()`. C++ `Address::printRaw` -> `AddrSpace::printRaw`
  (`space.cc:206`) prints `"0x" << setw(2*sz) << hex <<
  byteToAddress(offset,wordsize)` with **no space-name prefix**, zero-padded to
  the address width — e.g. `func_0x00401000` for `ram:0x401000`. The Rust port
  instead calls a *new, divergent* module-local `print_raw` helper
  (`architecture.rs:550`) that renders `"<space-name>:0x<offset>"`, yielding
  `func_ram:0x401000`: wrong prefix (space name added), no zero-padding, no
  `byteToAddress` word-size division. A faithful `Address::print_raw` already
  exists in kuna-base (`crates/kuna-base/src/address.rs:196` ->
  `space.rs:1516`, a correct transcription including the `+cut` wordsize tail)
  and was not used. The porter additionally **pinned the divergent format into
  a test** (`architecture::tests::name_function_angr_vs_upstream`,
  `architecture/tests.rs:145` asserts `func_ram:0x401000`), so the porter's own
  suite is green on the wrong output. Reachable whenever the upstream naming
  option is selected (default is angr=true, so dormant in the default config,
  but the option is a real control surface).
       cpp: decompiler/cpp/architecture.cc:539 (nameFunction) ->
            decompiler/cpp/address.hh:336 / decompiler/cpp/space.cc:206
            (Address::printRaw / AddrSpace::printRaw)
       rust: rust/crates/kuna-decomp/src/architecture.rs:498-505
             (name_function) + :550-555 (the divergent print_raw helper);
             pinned-wrong test at
             rust/crates/kuna-decomp/src/architecture/tests.rs:145
       fix: replace the local `print_raw` with `addr.print_raw(&mut s)` from
            kuna-base (already a faithful transcription) and correct the
            `name_function_angr_vs_upstream` assertion to the C++ form
            `func_0x00401000`.

No other findings. capability.rs and libdecomp.rs are faithful (registration
order, raw-last sort, first-match lookups, version constants 6/1, init order,
spec-path accumulation all verified). The seamed `Architecture` subsystems
(`init`/`restoreFromSpec`, type factory, proto models, print language, loader,
register-data decode, `scanForSleighDirectories` recursion) are correctly
out-of-scope deferrals with `// SEAM(...)` anchors and dependency-wave
ownership; not findings for this item.

## Adversarial tests

Committed (branch 1d75d3e) at
`rust/crates/kuna-decomp/tests/verify_w4_fw_architecture.rs`:

- `func_default_name_matches_cpp_printraw_oracle` — **FAILS** (F1 proof):
  `func_ram:0x401000` (port) != `func_0x00401000` (C++ oracle computed via the
  faithful kuna-base `Address::print_raw`).
- `sort_capabilities_preserves_order_of_many_trailing` — pass.
- `sort_capabilities_raw_at_front_rotates_left` — pass.
- `sort_capabilities_first_raw_only_moves` — pass (degenerate two-"raw" case).
- `find_capability_by_file_first_match_after_sort` — pass (raw-last sort
  changes routing as intended).
- `start_library_spec_paths_sleighhome_then_extras_in_order` — pass.
- `start_library_no_home_just_extras` — pass.
- `start_library_initializes_and_sorts_raw_last` — pass.

## Verdict rationale

REJECT: F1 is a confirmed, mechanically demonstrated fidelity divergence in an
in-scope ported function (`Architecture::nameFunction`, upstream path),
avoidable (a correct `Address::print_raw` already exists in kuna-base) and
silently masked by a porter test that pins the wrong output. It is a behavioral
output difference, not an accepted loss, so it is a blocker rather than a
losses.md entry. The fix is small (use the existing kuna-base `print_raw` and
correct the in-module test). Item returns to `todo` with attempts incremented;
F1 is the work order.

## Round 2

verdict: ACCEPT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_a6e29a1b-137-16 (worktree
`.claude/worktrees/wf_a6e29a1b-137-16`); porter repair reviewed at ff45d18,
my round-2 adversarial tests committed as 89905fe.
cpp blob shas: all 6 in-scope files unchanged from round 1 and still match the
checklist entry (`git hash-object`: architecture.cc 5bece5e7, architecture.hh
4237825d, capability.cc eaecfcda, capability.hh 2c0701d1, libdecomp.cc
869c131f, libdecomp.hh 1c1dcdd8).

gate (item gate, fresh): `cd rust && cargo test -p kuna-decomp` -> ok, 309 lib
+ all integration suites pass, 0 failed; `cargo test -p kuna-sleigh` -> ok, 0
failed. The previously-failing
`verify_w4_fw_architecture::func_default_name_matches_cpp_printraw_oracle` now
PASSES (suite is 8/8), and the in-module pinned test
`architecture::tests::name_function_angr_vs_upstream` now asserts the correct
C++ form.
gate (my round-2 tests): `cargo test -p kuna-decomp --test
verify_w4_fw_architecture_r2` -> 6 passed, 0 failed.
clippy: `cargo clippy -p kuna-decomp -- -D warnings` (after touching the 3 item
files to force a fresh build) -> clean.
port-audit: still no `port-audit` crate in the workspace (carried note);
blob-SHA drift checked manually (all 6 match, unchanged).

### F1 disposition: RESOLVED

The round-1 blocker was the non-angr `func_` default name using a divergent
module-local `print_raw` (`<space>:0x<offset>` — wrong prefix, no zero-pad, no
`byteToAddress`). The repair (ff45d18) deletes that helper and calls the
faithful kuna-base `Address::print_raw`:

```
let mut s = String::from("func_");
addr.print_raw(&mut s).expect("... C++ cannot fail here");
s
```

Re-derived against the C++ chain:
- `Architecture::nameFunction` (architecture.cc:539) = angr branch first, else
  `"func_" + addr.printRaw()` — Rust matches exactly (the angr branch routes to
  `database::kuna_function_name`, the kuna anchor).
- `Address::printRaw` (address.hh:336) = `"invalid_addr"` if null space else
  `base->printRaw(s,offset)` — kuna-base `Address::print_raw` matches, incl. the
  null arm.
- `AddrSpace::printRaw` (space.cc:206) = `sz=getAddrSize(); if(sz>4){ ... }; "0x"
  + setfill('0') setw(2*sz) hex byteToAddress(offset,wordsize); if(wordsize>1)
  "+cut"`. kuna-base generic-space `print_raw` (space.rs:1560) is a faithful
  transcription: the `>>32`/`>>48` shrink, the `0x{:0width$x}` with
  `width=2*sz`, `byte_to_address = val/ws`, and the `wordsize>1` `+cut` tail all
  match.
- The `.expect()` on `print_raw` is sound: only the fspec/iop space arms return
  `Err`, and a function address is always a processor space (const/other/join
  do not error), matching the `void` C++ `nameFunction` with no throw site. Not
  a partial-state concern.

The pinned-wrong round-1 test was corrected: `name_function_angr_vs_upstream`
now `assert_eq!(n_up, "func_0x00401000")`, and the oracle test computes
`"func_" + addr.print_raw()` independently and additionally pins the literal
zero-padded form. No weakening.

### Hunt list (re-checked for round 2)

Round 2 changed only `architecture.rs` (name_function repair) + its in-module
test; `capability.rs` and `libdecomp.rs` are byte-identical to round 1 (verified
clean there). Re-derived against the worktree this round:

- **Signedness.** clean. Capability loops (`sortCapabilities` `uint4 i`,
  `getCapability` `int4 i` vs `thelist.size()`) -> Rust `usize` index / iterator
  walks; no signed index, no underflow. The repair adds no comparisons.
- **Integer widths.** clean. `MAJOR/MINOR_VERSION` = 6/1 (`uint4`). The repair's
  width logic (`2*sz`, `>>32`/`>>48`, `offset % wordsize as i32`) lives in
  kuna-base and was re-derived to match C++ `int4 sz` / `int4 cut` truncation.
- **Wrapping.** clean. `func_` path is pure formatting; `byte_to_address` is
  division (no wrap); the only `wadd`/`wmul` in the helper are in the Join arm
  (out of the `func_` path), carrying `// cast:` notes.
- **Comparator totality.** n/a — no sort/`operator<` in scope; `sortCapabilities`
  is a find-first-and-rotate.
- **Iteration-order provenance.** clean. `thelist` is a registration-order `Vec`;
  `initialize_all`/`find_capability_*`/`get_capability`/`sort_capabilities` all
  walk it in index order, matching C++ `for(i=0;i<thelist.size();++i)`. No
  HashMap/HashSet anywhere in the 3 files (grep clean).
- **Off-by-one / do-while / reverse iterators.** clean. `sortCapabilities`'s C++
  shift loop (`thelist[j-1]=thelist[j]; thelist[last]=capa`) == Rust
  `remove(i)+push` left-rotation (re-verified vs the C++ source this round). The
  repair's format-width boundaries (`>>32`/`>>48` shrink, `setw(2*sz)`) are
  pinned by my new tests at exactly those boundaries.
- **Erase-while-iterating.** n/a — `remove(i)` happens once after the search
  loop terminates.
- **Exception -> Result partial-state parity.** clean. The repair's `.expect()`
  is on a C++-cannot-fail path (processor address); no mid-mutation throw.
- **Aliasing.** clean. Unchanged from round 1.

### Mechanical pass

- grep of the 3 item files: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`; no numeric `as` casts (the only `as` in scope is in kuna-base,
  out of item).
- clippy `-D warnings` clean.

### Adversarial tests (round 2)

Committed (branch 89905fe) at
`rust/crates/kuna-decomp/tests/verify_w4_fw_architecture_r2.rs`, all PASS,
each comparing the port against an independently-computed C++ `AddrSpace::printRaw`
oracle (so a regression in the port wiring OR the kuna-base helper is caught):

- `r2_func_name_zero_pads_4byte_no_space_prefix` — the exact F1 case;
  `func_0x00401000`, asserts no `ram` prefix (the old bug would fail both
  assertions).
- `r2_func_name_8byte_low_offset_shrinks_to_4` — `(offset>>32)==0` shrink to 8
  hex digits.
- `r2_func_name_8byte_mid_offset_width_6` — `>>32!=0 && >>48==0` -> 12 hex
  digits (middle boundary of the C++ shrink ladder).
- `r2_func_name_8byte_high_offset_full_width` — `>>48!=0` -> full 16 hex digits.
- `r2_func_name_wordsize_division_and_cut` — `byteToAddress` division + `+cut`
  tail (the part the original divergent helper dropped entirely); on-cut case
  has no tail.
- `r2_func_name_zero_offset_1byte_space` — `setw(2*sz)` at the smallest sz.

The round-1 suite (`verify_w4_fw_architecture.rs`, 8 tests) is retained and now
fully green.

### Verdict rationale

ACCEPT: the sole round-1 blocker (F1) is fixed by routing through the existing
faithful kuna-base `Address::print_raw`, verified against the C++ chain and
pinned across the full format edge-case space by independent oracle tests. No
new findings; the unchanged `capability.rs`/`libdecomp.rs` remain faithful. No
losses (F1 was a fixed output divergence, not an accepted divergence). Both
item-gate commands are green.

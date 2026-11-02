# Rust out-of-line stack probes

The pinned angr general test `test_decompiling_rust_binary_rust_probestack`
expects the Rust stack-probe helper to disappear from `create_note`. Kuna
currently retains that call and recovers a live R11 result as an undefined local.
The supplied executable is inspected statically; only authored harmless fixtures
are executed.

Add one P2 action before ExtraPopSetup and FuncLink, behind
`rustprobestack off|auto|always`. Auto requires Rust source-language evidence.
Match the complete 57-byte AMD64 helper, a direct relative CALL followed immediately
by SUB RSP,RAX, its canonical raw push/store/call p-code, and a local full-RAX
constant write. Bound the backward scan and frame size (16 MiB). Decline modified
bodies, interior targets, custom injected/inline/nonreturning call facts, missing
bytes, other register layouts, unknown sizes, and intervening flag observations.

Replace the call with its register and writable-memory summary: preserve the
caller's return-address store, add the saved-RBP store, restore the pushed return
slot, and write R11's exact residual (zero for zero; otherwise 1..4096).
The immediately following SUB overwrites arithmetic flags. Page-touch reads and
fault behavior are intentionally abstracted by this destructive instrumentation
option; ordinary SSA/dead-store analysis decides whether stack writes survive.
Never hide a helper by its name or at emission time.

Prove with raw-action integration tests, negative body/context/prototype cases,
whole unchanged emitted-C execution for harmless source fixtures including live
R11 and stack writes, and a two-pass stage case. Inspect all changed functions in
whole-binary parent/current and option-off/on captures, including the five calls
in the upstream binary. Run all repository gates. Predeclare a paired CPU/wall
speed experiment before sampling; auto can ship only with zero original corpus
regressions and the 5% speed budget satisfied, otherwise ship off.

# Declared-void `/GS` checks must not erase a floating return

## Witness

RE-friction need `declaring-double-score-return` points at PolyMLP.exe
`score` (`0x140003150`). The machine accumulates a `double` in nonvolatile XMM6,
copies XMM6 to ABI output XMM0, calls the security-cookie checker, restores XMM6,
and returns. With `score`, `log`, and the checker all declared, kuna discarded
the `log` result and returned undefined XMM fragments.

The exact dataset image is SHA-256
`01652e8b03e2bec127f5726320db4c9925d495d75583a8bb4534ac640f6ec230`
(54,272 bytes). IDA and Ghidra server startup timed out during this run, so the
instruction claim was checked with kuna's listing: XMM6 is accumulated, moved
to XMM0 immediately before the call at `0x140008140`, and restored afterward.

## Root cause

The enclosing `double score(...)` declaration is not the trigger. Declaring the
later checker `void` removes its concrete output, but the Windows x64 convention
still describes XMM0 as killed by the call. Heritage therefore creates a new,
undefined post-call XMM0 value. XMM6 itself remains correctly nonvolatile.

The generic decoded-callee proof cannot safely close this case. The checker's
syntactic failure tail contains an ordinary nested call before fast-fail, so its
body summary is intentionally incomplete. Relaxing nested-call handling would
make unrelated callees evidence-free register-preservation claims.

## Chosen seam

Use the existing exact MSVC cookie algebra recognizer as caller-side evidence.
When `calleeretpreserves` is enabled and an exact `(cookie ^ SP) ^ SP` direct
call has a locked-void output, P7 collects all such sites, records the full set,
and requests one restart. This stays complete even when a multi-exit function
has more checkers than the outer reflow limit. P3 then preserves only the exact
logical storage in the prototype model's ABI output list. If that 8-byte output
is contained in a 16-byte XMM heritage range, P3 extracts the pre-call output
slice, leaves the overlapping upper scratch lane killed, and rejoins the pieces
after the call; it does not downgrade the whole XMM range to unaffected.
The checker calls and cookie algebra remain visible; only `msvcstackguard on`
authorizes their existing deletion.

The generic return-preservation helper also learns the locked-void ABI-output
classification, so a complete decoded body can use it. Locked non-void outputs
remain authoritative, explicit effect overrides win, and killed scratch
registers outside ABI output are never preserved globally. The generic arm
requires a complete body with no ABI-return write. The exact-cookie arm may use
an incomplete failure-tail summary, but the production probe retains positive
writes and STORE spaces recovered before the unresolved edge. Any ABI-return
write among those facts still vetoes preservation. A STORE into a processor
space used by an ABI output entry is a possible write to that output even when
the runtime address cannot be resolved, so it vetoes the exact arm as well;
only conclusions from an absent write require completeness.

## Result

On the exact PolyMLP target, default output now contains
`v3 = v5 + log(v3 + v6);`, retains `sub_140008140(...)`, and returns
`v3 / (double)v21`. With `--option calleeretpreserves off`, the log call is again
discarded and the function returns undefined concatenated fragments. The
in-repo PE twin makes the same discriminator durable in stage and CLI suites.
Its negative checker writes XMM0 before a nested call, exercising the real
incomplete-probe path, and its 16-byte XMM control proves that the low
`XMM0_Qa` value survives while the upper `XMM0_Qb` scratch lane does not.
The focused predicate test separately pins the other production summary shape:
an incomplete probe with no direct ranges and a retained output-space STORE.

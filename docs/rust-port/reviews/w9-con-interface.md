# w9-con-interface
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4-8-1m (independent verifier, round 2)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-console` -> ok, 55 passed; 0 failed (incl. 8 round-1 + 4 round-2 verifier adversarial tests)

## Round-2 disposition

Round 1 REJECTed on a single **major** blocker (F1): the `IfaceBaseCommand`
base class and its six concrete commands (`IfcQuit`, `IfcHistory`, `IfcOpenfile`,
`IfcOpenfileAppend`, `IfcClosefile`, `IfcEcho`), `interface.hh:261-298` +
`interface.cc:518-622`, were unported and unaccounted — including the
`openfile write`/`closefile` redirect the Python harness uses to capture
`print C` (CLAUDE.md) and `quit` which ends `mainloop`.

The repair commit (`dd1b56f`) **resolves F1 in full**: all six commands are now
ported as `IfaceCommandAction` impls in `interface.rs` (the C++ abstract base's
`getModule()=="base"`/`createData()==null` supplied by a private `BaseModule`
marker each command embeds), all six are `pub`, and `register_com(&[&str])`
accepts the exact token shapes `ifacedecomp.cc:40-45` registers — crucially the
**two-token** `openfile write` / `openfile append`. The module docstring now
lists them and documents why they live in this item (same-file ownership). A new
verifier test drives the full harness path (`openfile write` -> `echo` ->
`closefile` -> `quit`) end-to-end through `run_command` and confirms the redirect
routing and `done` flag.

Re-deriving the six `execute` bodies against C++ byte-for-byte: `IfcQuit` (eof
check + "Too many parameters to quit" + `done=true`), `IfcOpenfile`/`Append`
("Output file already opened" / `s>>filename` / "No filename specified" + the
`ios_base::app` append flag), `IfcClosefile` ("No file open" + return-to-`optr`),
`IfcEcho` (`while(s.get(c)) fileoptr->put(c); << endl`) — all faithful. Echo
content position is correct: `expand_com`'s per-iteration `skip_ws` consumes the
space before the echo body exactly as C++ `s>>ws` does, so `echo hello` echoes
`hello\n` (no leading space) — verified against a C++ trace.

What remains are **minor, unreachable** integer-extraction gaps in the
newly-added `read_int`/`IfcHistory` path (F2 below), none of which is exercised
by any datatest or the harness (`history` appears in zero datatests, grep-
verified, and is not driven by `kuna/decompile.py`). They are recorded as a loss
(LOSS-125), so the verdict is **ACCEPT-WITH-LOSSES**, not a third attempt.

## Hunt list (round 2 — focus on the repair surface)

- **Signedness**: CLEAN for the framework (round-1 re-derivation of
  `getHistory` `i >= history.size()` cc:222 still holds — every caller passes
  `i >= 0`). New `read_int` returns `int4` (`i32`); the `IfcHistory` clamp
  `if num > history_size()` and the `for i in num-1..=0` loop are signed
  throughout, matching C++ `int4`.
- **Integer widths**: see F2. `read_int` overflow returns 0 instead of C++
  `INT_MAX` saturation; `IfcHistory`'s `num - 1` is a checked subtraction that
  debug-panics on `INT_MIN` where C++ wraps. Both unreachable.
- **Wrapping**: F2 — `num - 1` (interface.rs:575) should be `wrapping_sub(1)` to
  match C++'s silent INT_MIN->INT_MAX wrap; in a debug build it panics. The
  round-1 `restrict_com` temp bump `wrapping_add(1)` (cc:398) is still correct.
- **Comparator totality**: CLEAN, unchanged from round 1. `compare`
  (interface.rs:416) is a total lexicographic order; `sort_by` derives a
  consistent `Ordering`; no float cmp. Round 2 added no comparator.
- **Iteration-order provenance**: CLEAN, unchanged. `datamap` BTreeMap (==
  std::map key order), `comlist` Vec+`sort_by` (== vector+std::sort over unique
  keys). Round 2 added NO new container iteration. Grep: 0 `HashMap`/`HashSet`.
- **Off-by-one / do-while / reverse iterators**: CLEAN. `IfcHistory`'s
  `for(int4 i=num-1;i>=0;--i)` -> `let mut i = num-1; while i >= 0 { ...; i -= 1 }`
  keeps the descending oldest->newest order and the empty-when-`num<=0` semantics
  (`i -= 1` at i=0 reaches -1 and exits, no underflow at the bottom; only the
  initial `num-1` is the F2 panic locus). `readLine` do-while parity unchanged.
- **Erase-while-iterating**: N/A — no in-loop container mutation added.
- **Exception -> Result partial-state parity**: CLEAN. `run_command` re-installs
  the swapped action BEFORE propagating the command error (round-1 finding,
  unchanged). The new base commands `?`-propagate `IfaceError` cleanly:
  `IfcQuit`'s parse-error path leaves `done` untouched (matches C++ — the throw
  is before `status->done=true`); `IfcOpenfile`'s "already opened"/"No filename"
  errors leave `fileoptr` untouched (matches C++ where the throw precedes the
  `new ofstream`). The one C++ partial-state site the port does NOT reproduce is
  the `ofstream::open` *failure* inside `IfcOpenfile`/`Append` (F5): C++ raises
  `IfaceExecutionError("Unable to open file")` and resets `fileoptr=optr`
  synchronously, while the port defers the actual filesystem open to the binary
  (like `push_script_lines` defers script reads), so that error moves to flush
  time. Binary integration (decomp_dbg) is out of scope.

## Exact-console-output re-derivation (round 2 additions)

- `IfcHistory` listing: each line `*optr << historyline << endl` -> `out(line)`
  + `out("\n")`, byte-exact; "Too many parameters to history" byte-exact.
- `IfcQuit`: "Too many parameters to quit" byte-exact.
- `IfcOpenfile`/`Append`: "Output file already opened" / "No filename specified"
  / (deferred) "Unable to open file" byte-exact where reachable.
- `IfcClosefile`: "No file open" byte-exact.
- `IfcEcho`: rest-of-line bytes via `get()` (no ws skip) + `\n`, byte-exact;
  echo body position after `expand_com` matches C++ (`hello`, not ` hello`),
  confirmed against a C++ istringstream trace.
- `read_int`: re-derived against `g++ -std=c++11 istringstream >> int >> ws`.
  Clean small counts agree (`5`->5, `-3`->-3, `5 junk`->throw, `junk`->throw,
  `+x`->throw); the three divergent classes are F2 (all unreachable).

## Mechanical pass

- port-audit: tool still absent from this worktree (`port-audit` not in the
  workspace) — could not run; blob-sha drift not machine-checked (same as round
  1).
- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable: 0 hits.
- bare `as` casts: still present without `// cast:` comments (carried F4 process
  nit); all `usize`<->`int4` index/length interconversions over small bounded
  values, each semantically correct, no correctness hazard.
- clippy -p kuna-console --tests -- -D warnings: clean.

## Findings

  - F1 (major) — **RESOLVED in round 2** (`dd1b56f`). The six base commands +
       the `IfaceBaseCommand` shape are now ported (interface.rs:524-666), public,
       and registrable with the exact `ifacedecomp.cc:40-45` token shapes
       (incl. the two-token `openfile write`). Docstring updated. Verified
       end-to-end (verify_w9_r2_base_commands_registered_like_ifacedecomp).
       cpp: decompiler/cpp/interface.hh:261-298, decompiler/cpp/interface.cc:518-622
       rust: rust/crates/kuna-console/src/interface.rs:524-666
  - F2 (minor, LOSS-125): `CommandStream::read_int` / `IfcHistory` diverge from
       `std::istream >> int4` on three inputs — overflow (`999999999999`:
       cpp 2147483647 prints all history, port returns 0 prints nothing),
       sign-only-at-eof (`+`/`-`: cpp latches eof -> no throw, port rewinds the
       sign -> spurious "Too many parameters to history"), and INT_MIN
       (`-2147483648`: cpp `num-1` wraps to INT_MAX, port `num - 1` debug-panics).
       All UNREACHABLE — `history` is in zero datatests and not in the harness.
       Recorded as LOSS-125. The interface.rs:293-294 doc-comment's "saturates the
       same input the same way" over-claims for overflow (scoped to small counts).
       cpp: decompiler/cpp/interface.cc:531-552 (IfcHistory `s >> num >> ws`, `num-1`)
       rust: rust/crates/kuna-console/src/interface.rs:264-299, 555-584
  - F3 (minor, carried from round 1): non-ASCII input bytes become U+FFFD in
       `out_byte`/`file_out_byte`/`read_token` (e.g. a non-UTF-8 filename), vs C++
       byte-exact `std::string`. Unreachable in the ASCII corpus.
       cpp: decompiler/cpp/ifaceterm.cc:228, interface.cc:620
       rust: rust/crates/kuna-console/src/interface.rs:851-876, ifaceterm.rs:354-384
  - F4 (minor/process, carried): bare `as` casts lack `// cast:` justifications
       (protocol mechanical-pass requirement). All semantically correct.
       rust: rust/crates/kuna-console/src/{interface.rs,ifaceterm.rs} (multiple)
  - F5 (minor): the `ofstream::open`-failure path of `IfcOpenfile`/`Append`
       (C++ raises `IfaceExecutionError("Unable to open file")` and resets
       `fileoptr=optr` synchronously) is deferred to the binary along with the
       actual filesystem open — consistent with the `push_script_lines` deferral
       pattern; decomp_dbg binary integration is out of this item's scope. Not a
       loss because the redirect state machine and every other error message are
       faithful; only the deferred open's error timing differs.
       cpp: decompiler/cpp/interface.cc:567-573, 589-595
       rust: rust/crates/kuna-console/src/interface.rs:891-894 (open_file_redirect, infallible)

## Adversarial tests added (committed on branch, 6b6a7d9)

interface.rs (round 2, in addition to the 8 round-1 tests, all still passing):
  - verify_w9_r2_read_int_overflow_returns_zero_not_intmax   (overflow -> 0, not INT_MAX)
  - verify_w9_r2_read_int_sign_only_no_eof_latch             (sign rewind suppresses eof latch)
  - verify_w9_r2_history_intmin_arg_subtraction_overflows    (INT_MIN num-1 wrap/panic locus)
  - verify_w9_r2_base_commands_registered_like_ifacedecomp   (F1 end-to-end: openfile write/echo/closefile/quit)

All 55 lib tests pass (12 verifier adversarial tests total across both rounds).

## losses cited / appended

- LOSS-122, LOSS-123 (round 1, verifier-appended): RemoteSocket
  (`__REMOTE_SOCKET__`) and IfaceTerm raw-terminal (`__TERMINAL__`) — legitimate
  build-flag-gated exclusions named in the docstrings.
- LOSS-125 (round 2, verifier-appended): the `read_int`/`IfcHistory`
  integer-extraction divergences (F2). This is the accepted loss that makes the
  verdict ACCEPT-WITH-LOSSES. All three sub-divergences are unreachable via the
  datatest/harness corpus.

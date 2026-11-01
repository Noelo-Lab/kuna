# Go data pointers for internal byte strings

`fatstr` converts a constant pointer/length pair into an internal string value.
The value still has a pointer type, but the `DISPLAY_STRING` printer emitted a
bare quoted literal in Go. A caller whose formal is `unsafe.Pointer` therefore
received `unsafe.Pointer("hello bytes")`, which Go rejects.

The fix shares the existing ordinary-string data-pointer conversion with the
internal-string path. A complete one-byte literal passes through
`unsafe.StringData`, `unsafe.Pointer` and its recovered pointer type. Ordinary
character strings retain the terminating zero; the separate length argument is
unchanged. The fix does not infer a Go string aggregate or change the call ABI.
Wide and truncated internal strings are outside this fix.

The hermetic stage contains three byte-assembled calls with bounded, read-only
strings: plain ASCII, quotes/backslashes/control characters, and UTF-8. Three of
its five assertions fail on baseline
`a3363656e81149f5db0d57258fecfa4c0b340768`; all five pass with the fix. The console
integration test compiles the untouched emitted bodies and checks each call's
bytes, length and terminating zero. Existing ordinary C-string execution checks
continue to cover the shared helper's other path.

An additional native Go 1.22.5 witness is `runtime.badmorestackgsignal` at
`0x4323c0` in the Go output fixture (SHA256
`243073101d99260f354c13a6a06640d53d9eef64e4c4a4fdf8d9a5d8c6465c09`). With
`godwarfsig on`, its emitted body now compiles and calls a checking sink with
descriptor 2 and the exact 28-byte runtime message. `native-runtime.go` records
that executable witness. A small Go source repro also produces the invalid
conversion on the baseline without depending on a fixed runtime address.

The change is a strict Go emission correction under the existing `outlang go`
selection. C and Rust output retain their existing spelling. No new option is
introduced.

The twelve-image comparison covers 11,814 distinct functions and 23,683 C/Rust/Go
records, with `godwarfsig on` to exercise typed Go callers as well as the
stripped-image path. All 12,773 C/Rust records are identical. The 2,309 changed
Go bodies contain 7,037 added byte-pointer bridges across 3,405 textual hunks.
The comparison normalizes only those exact bridges and requires identical
remaining text and preservation of every pre-existing bridge. Function metadata
is identical. All 10,910 Go bodies parse; no decompilation errors occur.
`breadth.json` lists every changed body and classifies its hunks; the raw Go
diffs are included.

Validation: 10,006 workspace tests pass, none fail, and 39 are ignored across
71 suites. Original datatests remain 675/675; stages are 3,469/3,469. Tooling
checks pass 114 tests with one optional skip. Lint, spec links, the option
catalog and shared counters pass. The suite and native witness execution use Go
1.27.3. The original baseline is unchanged.

Speed uses 31 alternating baseline/patched pairs after three warmups per arm,
on CPU 79, after local builds and the workspace suite completed. Median CPU
changes are -0.49% for default C, +0.03%
for the Go string witness and -0.41% for the Rust control;
maximum wall change is +0.04%. Every timing file
records raw samples, ratios of medians and median paired changes. All remain
within the 5% budget.

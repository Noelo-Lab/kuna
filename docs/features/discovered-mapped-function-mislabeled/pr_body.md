## The problem

A function whose bytes are all in the image is reported as living in another
module, and emits no C, if any branch inside it targets an address the image
does not map. Build the 346-byte reduction and ask for it:

```
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/unmapped_branch_x86_64.py
$ kuna disassemble decompiler/crates/kuna-analysis/tests/fixtures/unmapped_branch_x86_64 --addr 0x401000 --count 6
# 6 instructions at sub_401000 @ 0x401000 (0x401000..0x401011, 17 bytes)
0x401000      31c0                  XOR EAX,EAX
0x401002      85ff                  TEST EDI,EDI
0x401004      7406                  JZ 0x40100c
0x401006      b839050000            MOV EAX,0x539
0x40100b      c3                    RET
0x40100c      e9efefbfff            JMP 0x0

$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/unmapped_branch_x86_64 sub_401000
// sub_401000: external symbol -- no code at this address in this module
```

Six clean instructions, 17 mapped bytes, and no body. It happens wherever a
displacement lands outside the image, which is ordinary in obfuscated, packed
and statically linked binaries: on the reported case (a stripped 807 KB static
ELF) it cost `sub_496920`, 160 bytes and 33 instructions, and it also cost the
UPX unpacking stub of the vendored `upx_packed_pe_i386.exe` fixture.

## The fix

- `FlowInfo::new_address` probes the load image for one byte at a branch target
  before queueing it, and ends the path if there is nothing there — the same
  treatment a target outside a caller-declared extent already got: a halt stub
  for the edge to land on, a warning at the branch site, and a
  `Function flows into unmapped memory` header on the prototype.
- Previously that target was queued, the walk popped it, and `load_fill`'s
  `Unable to load N bytes` raise unwound the whole flow follow. The CLI then read
  that failure out of the console transcript and attributed it to the *selected*
  entry, which is where the "external symbol" claim came from.
- The probe sits after the already-decoded short-circuit, so it is paid once per
  new target rather than once per branch, and it cannot cut flow that used to
  decode: the only address that fails it is one with no bytes behind it, which
  previously ended the function outright.
- No option. There is no judgement call between "lose the function" and "clip the
  one dead edge under a warning kuna already emits" — the same call as #422 and
  the declared-extent stub fix.

## The tests

`tests/cli/discovered-mapped-function-mislabeled.json` is the promoted acceptance
probe, retargeted to the vendored reduction because CI has no dataset; it asserts
the body's own `0x539`, not just a brace, so an empty stub cannot pass it. Three
cargo tests in `kuna-console/tests/verify_unmappedbranch.rs` cover the body, the
warning at its branch site, and the premise (the entry is mapped, the target is
not). All four fail on the unpatched tree.

Sweep: `decompile-all` over all 167 binary fixtures under
`kuna-analysis/tests/fixtures`, before and after — no exit code moved, and two
documents differ, both from an error line to a full body (the new fixture, and
`upx_packed_pe_i386.exe`'s `sub_4100a0`: 1 error line -> 185 lines of C).
Gates: `make test` 675/675 PARITY OK, `make test-stages` 687/687 PARITY OK,
`make rust-test` 6069 passed / 0 failed, `make check-spec` OK, `make test-cli`
85/85, `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

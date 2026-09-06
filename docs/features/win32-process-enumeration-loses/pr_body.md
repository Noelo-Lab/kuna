## The problem

`calleedeadarg` drops an argument when the callee's body proves the callee
overwrites that register before reading it. On a PE import the "callee body" is
the IAT slot, so the walk decodes a pointer as instructions — and when that
decode never reaches a path terminator, the proof it hands back is vacuous and
the argument disappears.

Build the fixture this PR adds and decompile its entry:

```
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/calleedeadarg_noterm_x86_64.py
$ kuna decompile calleedeadarg_noterm_x86_64.exe 0x140001000 --addr

void sub_140001000(void)
{
  CreateToolhelp32Snapshot(2);
  if (!CONCAT44(dat_4,v1)) {
    Process32NextW();
    return;
  }
  if (!CloseHandle())
    return;
  abort(); // no-return
}
```

The snapshot handle is in RCX at both calls and in the disassembly, and
`--option calleedeadarg off` prints it. Same shape on the crackme this came
from (crackmes.one `640a526833c5d447bc761899`, `sub_1400015c0`), where
`Process32FirstW` keeps both its arguments in the same run.

## The fix

- A callee-body walk that recorded **no path terminator** now proves nothing.
  The pass's test is "the register is written before *every* terminator", which
  is a conjunction — over an empty list it holds for every register at once.
- The walk ends that way when every path closes back onto an address it has
  already visited: a body that is one endless loop, and an IAT slot whose first
  byte decodes to `HLT`, whose p-code branches to itself.
- The guard is in `proves_dead`, not in the walk: the walk really did cover
  every path with nothing abandoned, so `complete` stays honest and only the
  claim built on it is withheld.

## The tests

`tests/cli/win32-process-enumeration-loses.json` (promoted acceptance probe) plus
a unit test pinning both directions — no terminator proves nothing, one
terminator that wrote RCX still proves it dead. Gates: `make test` 675/675
PARITY OK, `make test-stages` 635/635 PARITY OK, `make test-cli` 40/40,
`make rust-test` green, `make check-spec` green, `catalog OK`. A pre/post
`decompile-all` sweep over 160 binaries and 15,048 functions (107 in-repo
fixtures plus 53 RE crackmes) moved exactly one: the witness itself.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## The problem

On an x86 Windows PE, every stack slot after the first API call is at the wrong
offset, and the error grows with each call. In this bitmap decoder the buffer
handed to `ReadFile` and the buffer read back afterwards are the *same* four
bytes, but kuna models them as two independent variables:

```
$ kuna decompile ./Imagination.exe sub_401290 \
    --assert 'prototype LocalAlloc void *LocalAlloc(unsigned int flags,unsigned int size)'
unsigned int sub_401290(void)
{
  unsigned int v3; // stack - 0x20
  unsigned int v5; // stack - 0x34
  ...
    v5 = 0;
    ReadFile(v7,&Stackffffffe0,4,&Stackffffffe4,0);
    if ((GetLastError()) || (!v6)) break;
    wsprintfA(v1,0x406030,v1,(v5 >> 8 & 0xff) + (v5 >> 0x10 & 0xff) + (v5 & 0xff));
```

`v5` is the checksum source and nothing ever writes it, so it is assigned `0`
and the emitted program checksums a constant. Traced by hand from the
disassembly, `lpBuffer` and the checksum load are both `[entry_esp - 8]`, and
the function takes an `hFile` parameter it is not shown to have.

## The fix

- `StackSolver` guesses `extrapop = 4` — "the callee pops none of its
  arguments" — for every call whose model says `extrapop="unknown"`, and
  `x86win.cspec`'s default `__stdcall` says exactly that for every call. Each
  `__stdcall` API therefore leaves the modeled stack pointer `4*nargs` too low,
  cumulatively.
- New option `calleepop` (default on) raises that guess for the one family
  whose convention the platform fixes: an **imported** callee. `peimportcall`
  already paints `Varnode::externref` over the Import Address Table, so the
  imports are identifiable; internal callees keep upstream's guess, because
  MSVC compiles those `__cdecl`.
- For an import the argument bytes are counted off the caller's own push run,
  stopping at the first push that stores a register's *input* Varnode — a
  prologue callee-save, not an argument.
- A caller that raises the stack pointer past the whole run is doing `__cdecl`
  cleanup, so `wsprintfA` and friends keep `4`. That is measured as the highest
  stack-pointer value reachable from the call rather than by looking for an
  `add esp,#k`: the additive normalization re-bases a following push run onto
  the call's own result and leaves the cleanup Varnode dead.

Output on the same command becomes the hand-traced truth — one slot, and the
parameter:

```
void * sub_401290(unsigned int a0)
{
  int v3; // stack - 0x4
  unsigned int v4; // stack - 0x8
  ...
    ReadFile(v1,&v4,4,&v3,0);
    if ((GetLastError()) || (!v3)) break;
    wsprintfA(v2,0x406030,v2,(v4 >> 8 & 0xff) + (v4 >> 0x10 & 0xff) + (v4 & 0xff));
```

## The tests

`tests/cli/pixel-reader-uses-undefined.json` runs the command above against a
vendored 40 KB PE fixture and requires the checksum operand to be the same
variable `lpBuffer` points at, so a cosmetic rename cannot pass it; it fails
with `--option calleepop off`. Gates: `make test` PARITY OK 675/675 (the
default-on flip moves no assertion), `make test-stages` PARITY OK,
`make check-spec` OK, `catalog --check` OK, `make test-cli` 68/68. Whole-binary
sweep of the fixture: 16 of 87 functions change, `off` is byte-identical to the
unpatched tree, and three deep-frame CRT helpers lose call arguments they had
recovered by accident of a too-shallow frame — recorded in
`docs/features/pixel-reader-uses-undefined/record.json` and in the spec.
`decompile-all` median 926 ms on vs 942 ms off over 7 reps.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## The problem

An i386 PIE reaches every global through a get-PC thunk, and the argument the
caller loaded before it disappears. `collide` from crackmes.one
(`5ab77f5833c5d40ad448c399`), `sub_8049f20`:

```
$ kuna decompile ./collide 0x8049f20 --addr \
    --assert 'prototype sub_8049f20 int4 sub_8049f20(char *path,void *buf);'
int sub_8049f20(char *path,void *buf)
{
  unsigned int v1; // edx

  sub_8049f55();
  return __lxstat(3,path,v1);
}
```

`v1` is declared, read once, and assigned nowhere. The disassembly says what it
is, and the controlled comparison is inside the one function:

```
8049f29  mov edx,[ebp+0xc]     ; buf
8049f2c  call 8049f55          ; 8049f55: mov ebx,[esp]; ret
8049f3e  mov [esp+0x8],edx     ; third argument
8049f42  mov edx,[ebp+0x8]     ; path
```

`path` is loaded after the call and is recovered; `buf` is loaded before it and
is lost. The only difference is that `EDX` crosses the `CALL`. Two testers hit
this on two architectures in round 3; the second was a Win64 helper preserving
`RAX`.

## The fix

- `x86gcc.cspec` lists `ECX`/`EDX` in `<killedbycall>` because cdecl allows a
  callee to clobber them, and `Heritage::guardCalls` applies that to every call.
  Under `option calleepreserves` (new, default on) the guard consults the
  callee's own instructions instead: a bounded body walk that proves the callee
  never writes the register downgrades *killed by call* to *unaffected* for that
  one call.
- The walk is the one `rustabi` already takes for the call-output seam, sharing
  its per-image cache, so a body is decoded at most once per run. It declares
  itself incomplete — proving nothing — at a nested call, an unresolved
  `BRANCHIND`, an undecodable instruction, or its budget, so a PLT stub and a
  recursive callee are never narrowed.
- The output-active arm is skipped for the same range. Downgrading alone is
  inert here: `EDX` characterizes as return storage on i386 (the `EAX:EDX` join
  pentry), so that branch promotes the range straight back.
- Absence of a write is not enough on its own. A complete walk over a one-byte
  `ret` records nothing, and believing it deletes the return value of every stub
  and placeholder in the image. The callee must also have written a register the
  model marks `<unaffected>` other than the stack pointer — the signature of the
  hand-rolled helper (the thunk's `EBX`), and a positive finding rather than an
  absence. Without that half, 24 stage assertions regress.

## The tests

`tests/stages/kuna-calleepreserves.xml` is two-pass on the witness's own bytes:
`off` pins `unsigned int v1; // edx`, `on` pins the recovered parameter.
`tests/cli/get-pc-helper-loses.json` is the promoted acceptance, re-pointed at a
vendored fixture because CI has no dataset.

`ghidra_sim_faillog_pins` moves: whole-session `getPcode` 1613 -> 1862, with
the distinct-decoded count unmoved at 1044. Nothing new is read — the seam takes
the same bounded walk without `calleedeadarg`'s "fewer than two calls" skip, so
it re-asks for bytes the session already decoded, and ghidra mode has no p-code
cache. Both arms measured: `off` gives 1613/1044, `on` gives 1862/1044.

`make test` 675/675 PARITY OK with `docs/baseline.json` untouched,
`make test-stages` 650/650 PARITY OK (+6, all mine; no existing assertion moved),
`make test-cli` 52/52, `make rust-test` green, `make check-spec` OK,
`catalog OK`. Sweep, both arms of `decompile-all`: 128 decbench ELFs (x86-64 PIE
and ARM, O0/O2), 15,987 functions, 0 changed; 47 x86-32 images, 7,064 functions,
15 changed and none of them starts reading a never-assigned local.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

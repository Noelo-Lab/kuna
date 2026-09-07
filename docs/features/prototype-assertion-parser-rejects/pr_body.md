## The problem

A prototype assertion that names a calling convention is rejected as bad C
syntax. Every Windows API declaration worth pasting carries one, and the
`__stdcall` spelling is exactly what an agent repairing stack arguments reaches
for:

```bash
kuna decompile ./Cube.exe sub_401ba0 --assert-strict \
  --assert 'prototype 0x4050a6 void * __stdcall LoadLibraryExW(unsigned short *n,void *f,unsigned int g)'
```

```text
warning: --assert "prototype 0x4050a6 void * __stdcall LoadLibraryExW(...)" rejected: Bad C syntax
$ echo $?
1
```

Dropping `__stdcall` from the same declaration is accepted, so the convention is
the only thing in the way — and nothing in `kuna docs`, the assertion reference
or the option catalog offers another way to state one.

## The fix

- `CParse::lookup_identifier` classifies an identifier the loaded compiler spec
  registered as a prototype model into a function specifier, which is upstream's
  `glb->hasModel` arm — a stub since the port, for want of a model registry the
  `Architecture` has had for a while. The registry decides: a spelling the spec
  does not declare stays an identifier and is still rejected, so a misremembered
  convention cannot be silently dropped.
- The C-standard specifier run ends at the `*`, so a pointer-returning function
  has nowhere to put its convention. The declarator accepts one on either side
  of the pointer run, which also covers the Win32 callback shape
  `int (__stdcall *cb)(int)`.
- The named convention is resolved against the registry and recorded against the
  function. It drives storage in both directions: the function's own prototype
  is seeded under it, and the prototype-bearing `TypeCode` locked onto the symbol
  is built under it — which is the copy a *caller* reads, so declaring a callee
  `__fastcall` moves where the caller's arguments come from. Accepting the
  keyword without that second half would be a directive that parses and does
  nothing.

## The tests

Seven parser cases in `kuna-console` (both specifier positions, the callback
shape, an unregistered convention still rejected, two conventions rejected, and
the no-registry parse rejected exactly as before). Two promoted probes on the
in-repo `fauxware` fixture: `prototype-assertion-parser-rejects.json` is the
acceptance, and `prototype-convention-is-honored.json` is the anti-swallow guard
— declaring `accepted()` `MSABI` renders `accepted(a3)` instead of
`accepted((int)v4)`, which a fix that lexed the keyword and dropped it would
fail. Gates: `make test` 675/675 PARITY OK, `make test-stages` 677/677 PARITY
OK, `make test-cli` 70/70, `make check-spec` OK, `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

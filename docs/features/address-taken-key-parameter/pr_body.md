## The problem

A function whose parameter is only ever used *by address* declares that parameter a
second time inside its own body, under the same name and a different type — which is
not valid C, so the function cannot be recompiled from `kuna decompile-project` output.

```sh
cat > d.s <<'EOF'
.globl _start
_start:      push $2; push $1; call derive_key; add $8,%esp; hlt
.type derive_key,@function
derive_key:  push %ebp; mov %esp,%ebp
             lea 0x8(%ebp),%eax          # &a0 -- the parameter's own slot
             push 0xc(%ebp); push %eax; call consume
             add $8,%esp; pop %ebp; ret
.type consume,@function
consume:     mov 0x4(%esp),%eax; mov (%eax),%eax; add 0x8(%esp),%eax; ret
EOF
as --32 d.s -o d.o && ld -m elf_i386 d.o -o d
kuna decompile ./d derive_key
```

```c
void derive_key(unsigned int a0,unsigned int a1)
{
  int a0;

  consume(&a0,a1);
}
```

`--json` disagrees with that in the other direction: it reports `param_1` with no
`line_numbers` at all, an unused parameter.

## The fix

- Upstream declares body locals by walking the ScopeLocal symbol table for
  `no_category` symbols (`PrintC::emitScopeVarDecls`). kuna walks HighVariables
  instead and recognises a parameter by testing whether a `function_parameter`
  symbol contains the *storage* of one of the high's Varnodes. An `&parameter`
  reference has no such Varnode — it is the offset constant of a
  `PTRSUB(spacebase, off)` — so it escapes the test.
- New option `paramrefdecl` (P9, default on, DIV-143) makes the same decision on
  symbol *identity*: `linkSpacebaseSymbol` already binds the reference to the symbol
  owning the frame slot, so a high whose bound symbol is `function_parameter` is that
  parameter and is not declared again.
- Not a rename. The duplicate object stops existing; `&a0` in the body resolves to
  the prototype's own `a0`.
- The skip keeps the existing guard that the high's name is one of the prototype's
  parameter names, so it can only ever remove a declaration that duplicates a
  signature parameter — never leave one undeclared.
- The same symbol identity is given to the `variables` JSON surface, and only where
  no storage-backed or name-backed evidence was found, so `param_1` now carries the
  `&a0` line instead of reading as unused.

## The tests

`tests/stages/kuna-paramrefdecl.xml` (two-pass: off reproduces the duplicate
declaration, on removes it, both passes keep `consume(&a0,a1)` and the signature),
unit tests for the symbol-identity predicate, and `tests/cli/address-taken-key-parameter.json`
carrying the need's acceptance regex verbatim against a newly vendored 4,532-byte i386
ELF. `make test` PARITY OK 675/675 (byte-identical — the default-on evidence);
`make test-stages` PARITY OK 712/712 (3 new assertions, none moved); `make test-cli`
101/101; `make check-spec` green (strict too); `kuna catalog --check` OK; workspace
suite green (379 suites, 6,186 tests, 0 failures).

Sweep: `decompile-all` in both arms over 66 crackme binaries, 14,474 functions —
5 changed, 0 added, 0 removed, and every hunk is the removal of the duplicate
parameter declaration. Speed: not timed -- the change is one `Option` read plus one symbol-category lookup per declared HighVariable, on a path already walked once per declaration.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

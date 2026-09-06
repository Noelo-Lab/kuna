## The problem

An MSVC `/GS` frame cookie makes every stack-passed call argument in the
function disappear. On a Win64 crackme, a call whose prototype is declared
variadic renders with one value after `"%s%s"`, though the disassembly stores
the second pointer to `[RSP+0x20]` immediately before the call:

```
$ kuna decompile ObfuscationFiesta.exe sub_140002530 \
    --assert 'prototype sub_140001180 int format(char *out,unsigned long long size,char *fmt,...)'
    sub_140001180(v85,v103,"%s%s",v38);
```

It is not about varargs. The same binary loses the `va_list` of an ordinary
non-variadic call for the same reason:

```
$ kuna decompile ObfuscationFiesta.exe sub_140001030
    v8 = __stdio_common_vfprintf(*v1,v11,v10,0);      /* the 5th argument is gone */
```

Reproducible without the crackme — assemble the two ingredients (a cookie mixed
with `xor %rsp`, and a call with stack-passed arguments) and the call truncates
at the register budget:

```
$ cat >gs.s <<'EOF'
callee: mov 8(%rsp),%rax; add 16(%rsp),%rax; add %rdi,%rax; add %rsi,%rax
        add %rdx,%rax; add %rcx,%rax; add %r8,%rax; add %r9,%rax; ret
caller: sub $0x38,%rsp; movabs $0x123456789abcdef0,%rax; xor %rsp,%rax
        mov %rax,0x28(%rsp); mov %rdi,(%rsp); mov %rsi,0x8(%rsp)
        mov $1,%edi; mov $2,%esi; mov $3,%edx; mov $4,%ecx; mov $6,%r9d; mov $5,%r8d
        call callee
        mov 0x28(%rsp),%rcx; xor %rsp,%rcx; add %rcx,%rax; add $0x38,%rsp; ret
EOF
$ gcc -nostdlib -no-pie -o gs gs.s -Wl,-e,caller && kuna decompile ./gs caller
  v2 = a0;                       /* argument 7, stored and then dropped */
  v3 = a1;                       /* argument 8 */
  v1 = callee(1,2,3,4,5,6);
```

## The fix

- `AliasChecker::gatherAdditiveBase` records a local-alias escape site at the
  frame offset of every *non-additive* use of a stack-pointer-derived Varnode.
  `xor rax,rsp` is not an address computation, but read as an escape it plants a
  site at the bottom of the frame — the shallowest offset there is — so
  `hasLocalAlias` answers yes for every stack location in the function and
  `checkInputTrialUse` scores every outgoing-argument slot no-use.
- New option `cookiescramble` (`on|off`, default on, DIV-126): an `INT_XOR` no
  longer records an escape site. Not conditioned on the second operand — whether
  the cookie is loaded or folded to an immediate is an optimizer detail, not an
  aliasing fact.
- Scoped to the checker the call-site recovery builds, not to the local-layout
  gather. Flipping both also drops the covering array `ScopeLocal` builds at the
  alias offset, which moved 17/159 functions on the witness binary instead of
  6/159 and turned five unnamed `&Stack<hex>` addresses into 38. Stack-variable
  layout is byte-identical here.
- GCC/Clang read the cookie from `%fs:0x28` and never touch the stack pointer,
  so the change is structurally inert on ELF corpora.

## The tests

`tests/stages/kuna-cookiescramble.xml` — two-pass on the assembly above: option
off truncates the call and leaves the two arguments as dead stores; the default
recovers `callee(1,2,3,4,5,6,a0,a1)`. Promoted probe
`tests/cli/variadic-prototype-still-drops.json` against the vendored
`gscookie_x86_64` fixture (the variadic form of the same shape) fails without
the fix. `make test` 675/675 byte-identical, `make test-stages` 657/657 (+3, no
pre-existing assertion moved), `make test-cli` 58/58, `make check-spec` OK,
`kuna catalog --check` OK. Sweep over 3111 functions: 17 changed, every one an
argument added at a call; none lost.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

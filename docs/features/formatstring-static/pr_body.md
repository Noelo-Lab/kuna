## The problem

A `printf`/`scanf` call states the types of its variadic arguments in its
format string, and the default output ignores it. Reading the string was
already possible (`--option formatstring on`), but it cost a second decompile of
every caller, +43% to +77% on a whole binary, so it stayed off.

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fmt_x86_64 main
unsigned long main(unsigned int a0,unsigned long *a1)
{
  printf("%d %s\n",(unsigned long)a0,*a1);
```

With this change the default prints `main(int a0,unsigned long *a1)` and
`printf("%d %s\n",a0,(char *)*a1);`.

## The fix

- `formatstring` becomes `off|static|full`, default `static` (`on` still means
  `full`). `static` reads the format constant out of the image at load, from the
  Listing's call and tail-jump edges and a short constant fold before each call,
  and hands the per-call prototype to the first decompile. `full` keeps the old
  second-decompile loop for the sites `static` cannot answer.
- A callee takes a format when its built-in signature has a `char *` just
  before the `...`, so `err`/`warn`/`error`/`syslog` count too. A format that
  comes out of `dcgettext` is read through its msgid.
- The first decompile has the last word. An override is dropped, and the
  function decompiled once more, when the call passes a different number of
  arguments than it declares (an `alloca` frame) or its format is not the
  resolved string (a join reached from a jump table). That happens at 130 of
  11,743 sites in 325 binaries.
- An override is built only where the target passes a vararg exactly like a
  named argument: every conversion on x86 and standard AArch64, integer and
  pointer conversions on ARM32, RISC-V, MIPS, PowerPC and Windows AArch64,
  nothing on Apple AArch64 (which puts every vararg on the stack) or an
  unchecked processor. This binds `full` too.
- The format string must sit in a read-only section, and a site whose call
  instruction writes the format register itself (a delay slot) is declined.
- Fixes in the existing typing: override parameter names no longer leak into
  the caller, `%c` is a char, `%lc` is a `wint_t` rather than a `char`, `%lf`
  is a `double` (and `double *` for scanf) rather than an `unsigned long`,
  `%Lf` declines the site.

## The tests

`tests/stages/kuna-formatstring-static.xml` has 20 passes: off, default and
`full` on `fmt_x86_64` and on the new `fmtjoin_x86_64` (the jump-table and
`alloca` sites). Passes 7-14 require default == off on the Apple arm64
`macho_imports_arm64`, on a new ARM hard-float `fmtabi_armhf` and on a new
`fmtedge_x86_64` (a format in `.data`, a `%lc`). Passes 15-20 pin `%lf` as a
`double` on new `fmtlf_x86_64`/`fmtlf_armhf` fixtures. The previous build fails
8 of those 9. Also a CLI probe and unit tests. Over 325 binaries (every
coreutils build plus 31 others), off vs static: no format call gains or loses a
vararg, or passes one in the wrong class (float vs integer), against its
conversions (313 counts and 4 classes are fixed), and 8 functions change arity,
all to what their callers pass. With `formatstring off` the output is
byte-identical to main on 13 other binaries.
decbench `type_match` over 444 slices: 987 → 1025 perfect, 176 better, 2 worse.
Speed, `decompile-all` interleaved min-of-15 against `off`: SPEEDLINE.
Evidence: `docs/features/formatstring-static/`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01YFCA715QE6ZBSnTn7bwabG

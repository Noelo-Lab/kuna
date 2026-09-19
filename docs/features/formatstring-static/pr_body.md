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
  resolved string (a join reached from a jump table).
- An override is built only where the target passes a vararg exactly like a
  named argument: every conversion on x86 and standard AArch64, integer and
  pointer conversions on ARM32, RISC-V, MIPS, PowerPC and Windows AArch64,
  nothing on Apple AArch64 or an unchecked processor. This binds `full` too.
- The format must sit in a read-only section, and a site whose call
  instruction writes the format register itself (a delay slot) is declined.
- Fixes in the existing typing: override parameter names no longer leak into
  the caller; `%c` is a char and `%lc` a `wint_t`; `%lf` is a `double` (a
  `double *` for scanf); `%zu`/`%td` are pointer-width, so 8 bytes on Win64
  where `long` is 4; `%Lf` declines the site.

## The tests

- `tests/stages/kuna-formatstring-static.xml`, 23 passes: off, default and
  `full` on `fmt_x86_64`, the jump-table/`alloca` sites, `%lf` on x86-64 and
  ARM hard-float, a Win64 PE with `%zu`/`%td`, and default == off on Apple
  arm64, ARM hard-float `%f` and a format in `.data`. Without the `%zu` fix
  the 3 Win64 assertions fail; without the `%lf` fix 8 more do.
- decbench `type_match` over 444 slices: 986 → 1024 perfect, 176 functions
  better, 2 worse.
- Over 325 binaries, off vs static, no format call gains or loses a vararg or
  gets one of the wrong class. With `formatstring off` the output is
  byte-identical to main. `decompile-all` min-of-15 against main: `fmt`
  +1.2%, `ls` +0.7%, `sort` +1.1%.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01YFCA715QE6ZBSnTn7bwabG

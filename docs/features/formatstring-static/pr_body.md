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
- The resolved prototype is installed with an open tail: the call is scored
  exactly like the untyped call and drops what it took past its declared
  arguments only once every call in the function has its arguments, so the
  calls around it keep what `off` gives them. Installed closed, it freed the
  stack slots it used to claim, and calls whose format is not resolved took
  them (three `__fprintf_chk` calls in Ubuntu m4's `version_etc` went from 9
  to 12 arguments); it also let a clang `sscanf("%d", &a)` destination in the
  first outgoing stack slot fold to the value stored before the call.
- The first decompile has the last word. An override is dropped, and the
  function decompiled once more, when the call's format is not the resolved
  string (a join reached from a jump table) or its argument count disagrees.
- The default types x86 only. A declared argument, now certain, can veto the
  same value at a neighbouring call. On x86-64 the 8 calls this moved all lost
  a phantom; on AArch64 and ARM32 firmware (measured with the prototype
  closed) it gave calls an `x8` of `0` or leftover registers and cost iproute2
  `ip`'s `parse_rtattr` its `len`. `full` still types those targets.
- An override is built only where the target passes a vararg exactly like a
  named argument: every conversion on x86 and standard AArch64, integer and
  pointer conversions on ARM32, RISC-V, MIPS, PowerPC and Windows AArch64,
  nothing on Apple AArch64 or an unchecked processor.
- The format must sit in a read-only section, and a site whose call
  instruction writes the format register itself (a delay slot) is declined.
- Fixes in the existing typing: override parameter names no longer leak into
  the caller; `%c` is a char and `%lc` a `wint_t`; `%lf` is a `double` (a
  `double *` for scanf); `%zu`/`%td` are pointer-width, so 8 bytes on Win64
  where `long` is 4; `%Lf` declines the site.

## The tests

- `tests/stages/kuna-formatstring-static.xml`, 32 passes and 38 assertions:
  off, default and `full` on `fmt_x86_64`, the jump-table/`alloca` sites, `%lf`
  on x86-64 and ARM hard-float, a Win64 PE with `%zu`/`%td`, `fmt_aarch64`,
  gnulib's `version_etc` shape, clang `sscanf` destinations in the first
  outgoing stack slot and a ninth `double` on the stack, and default == off on
  Apple arm64, AArch64, ARM and a format in `.data`. A build that installs the
  prototype closed fails 6 of the last 10.
- decbench `type_match` over 444 slices (x86-64): 988 → 1026 perfect, 176
  functions better, none worse.
- Every call in every function the default changes, off vs default, over 325
  decbench and 25 Ubuntu `/usr/bin` x86-64 binaries (81,955 functions): no
  format call newly disagrees with its format (428 stop disagreeing), no call
  whose format is not resolved gains an argument, and the 8 other calls whose
  argument count moved all lost a phantom. With `formatstring off`, and on
  every non-x86 target by default, the output is byte-identical to main.
  `decompile-all` min-of-15 against main: `fmt` -4.6%, `ls` +0.8%, `sort`
  -0.4%, `m4` +0.7% (against `off`: +1.2%, +1.2%, -0.7%, +1.1%).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01YFCA715QE6ZBSnTn7bwabG

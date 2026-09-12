## The problem

`linuxsyscall on` named Linux i386 `int 0x80` calls in a 32-bit image, but the
same compatibility entry in an x86-64 image stayed opaque:

```c
(*(code *)swi(0x80))();
```

The minimal witness is 17 bytes: `endbr64; mov eax,1; mov ebx,0; int 0x80; ret`.
Linux interprets it with the i386 table and ABI, so its source-level meaning is
the standalone call `sys_exit(0);`.

## The fix

The existing pre-SSA `linuxsyscall` repair now accepts x86 languages with a
four- or eight-byte default code address size. In long mode it resolves the full
RAX/RBX/RCX/RDX/RSI/RDI/RBP records, then models their low four-byte lanes with
the existing i386 syscall table and argument order.

There is one deliberately narrow accommodation for SLEIGH: before SSA,
`mov eax,imm32` is normalized into an eight-byte zero-extending COPY to RAX. The
number scan accepts that exact constant full-RAX COPY and takes its low 32 bits.
It still refuses a computed value, a partial or mismatched-width COPY, an
overlapping write, a barrier, or a non-x86 language. The interrupt matcher and
rewrite are unchanged.

Native `SYSCALL` is still handled only by `x64syscall`. The stage gate includes
both forms in one x86-64 image and proves that turning `linuxsyscall` on changes
the compatibility interrupt once while leaving the native call byte-identical.

## Tests

- Four focused unit tests pin the syscall table and exact constant-width rules.
- `kuna-linuxsyscall-x64-compat.xml` adds five two-pass assertions; the existing
  i386 and native-x64 stages pass alongside it (16/16 focused assertions).
- The promoted CLI probe over the deterministic 17-byte raw fixture requires a
  standalone `sys_exit(0);` and forbids `swi`, `syscall()`, `sys_write`, and
  extra arguments.
- Datatest expectations are unchanged. The stage baseline moves 783→788 only
  for the five new assertions; the corpus-count guard moves 274→275.

Catalog, strict spec, and derived-counter checks pass.

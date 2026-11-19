# (kuna) Option `tailcallentry` on AArch64. `helper` is reached only by the
# tail `b` at the end of `f`, and `g` sits between them, so the branch leaves
# f's entry-ordered region. Build: clang --target=aarch64-linux-gnu -nostdlib
# -static -fuse-ld=lld -Wl,--build-id=none -o tailcall_aarch64 tailcall_aarch64.s
# && llvm-strip -s tailcall_aarch64
    .text
    .globl _start
_start:
    bl   f
    bl   g
    mov  x8, #93
    svc  #0
    brk  #0
f:
    mov  w0, #1
    b    helper
g:
    mov  w0, #2
    ret
helper:
    add  w0, w0, #1
    ret

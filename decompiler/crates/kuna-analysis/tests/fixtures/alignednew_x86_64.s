# (kuna) RE-friction round 2, need `argument-recovery-knobs-still` / option
# `calleearityfwd`.  Minimal SysV x86-64 stand-in for MSVC's aligned
# `operator new`: one callee reached from both arms of a size test, where the
# small arm passes its argument register live-in and the guard branches on it.
# The small arm is laid out AFTER the large one and reached by a forward branch,
# which is what makes its call spec finalize FIRST -- so the site that loses its
# argument has no already-final witness and only the deferred retry can rescue
# it.  See decompiler/crates/kuna-analysis/tests/fixtures/README.md.
    .text

    .globl callee
    .type callee, @function
callee:
    mov  %rdi,%rax
    add  $1,%rax
    ret
    .size callee, .-callee

    .align 16
    .globl caller
    .type caller, @function
caller:
    sub  $0x28,%rsp
    cmp  $0x1000,%rdi
    jb   .Lsmall
    lea  0x27(%rdi),%rax
    cmp  %rdi,%rax
    jbe  .Lbail
    mov  %rax,%rdi
    call callee                # large arm: fresh rdi, argument recovered
    add  $0x28,%rsp
    ret
.Lsmall:
    test %rdi,%rdi
    jz   .Lzero
    call callee                # small arm: rdi live-in, and the guard reads it
    add  $0x28,%rsp
    ret
.Lzero:
    xor  %eax,%eax
    add  $0x28,%rsp
    ret
.Lbail:
    xor  %eax,%eax
    add  $0x28,%rsp
    ret
    .size caller, .-caller

    .align 16
    .globl _start
    .type _start, @function
_start:
    mov  $0x40,%edi
    call caller
    mov  $60,%eax
    xor  %edi,%edi
    syscall
    .size _start, .-_start
    .section .note.GNU-stack,"",@progbits

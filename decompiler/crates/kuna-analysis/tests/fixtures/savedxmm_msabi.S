/* Compile as ELF on Linux; the tested function and its helper use Win64 ABI.
   The probe target can be routed to the native body or emitted C independently. */
#ifndef SAVED_XMM_FUNCTION
#define SAVED_XMM_FUNCTION fixture_spills_rect
#endif
#ifndef SAVED_XMM_PROBE_TARGET
#define SAVED_XMM_PROBE_TARGET SAVED_XMM_FUNCTION
#endif
.text
.globl SAVED_XMM_FUNCTION
.type SAVED_XMM_FUNCTION,@function
SAVED_XMM_FUNCTION:
    subq $0x88, %rsp
    movaps %xmm6, 0x40(%rsp)
    movaps %xmm7, 0x50(%rsp)
    movaps %xmm8, 0x60(%rsp)
    movaps %xmm9, 0x70(%rsp)
#ifdef SAVED_XMM_ESCAPE_HOME
    leaq 0(%rsp), %rax
    movq %rax, fixture_escaped_home(%rip)
#endif
    leaq 0x20(%rsp), %rcx
    leaq rect_source(%rip), %rdx
    call fixture_copy_rect@PLT
#ifdef SAVED_XMM_READ_HOME
    movq 0(%rsp), %rax
    movss 0(%rax), %xmm0
#else
    movss 0x20(%rsp), %xmm0
#endif
    addss 0x24(%rsp), %xmm0
    movaps 0x40(%rsp), %xmm6
    movaps 0x50(%rsp), %xmm7
    movaps 0x60(%rsp), %xmm8
    movaps 0x70(%rsp), %xmm9
    addq $0x88, %rsp
    ret
.size SAVED_XMM_FUNCTION, .-SAVED_XMM_FUNCTION

#ifdef SAVED_XMM_READ_HOME
.globl fixture_copy_rect
.type fixture_copy_rect,@function
fixture_copy_rect:
    movq %rcx, 8(%rsp)
    movq %rdx, 16(%rsp)
    movss 0(%rdx), %xmm0
    movss %xmm0, 0(%rcx)
    movq 8(%rsp), %rcx
    movq 16(%rsp), %rdx
    movss 4(%rdx), %xmm0
    movss %xmm0, 4(%rcx)
    movq 8(%rsp), %rcx
    movq 16(%rsp), %rdx
    movss 8(%rdx), %xmm0
    movss %xmm0, 8(%rcx)
    movq 8(%rsp), %rcx
    movq 16(%rsp), %rdx
    movss 12(%rdx), %xmm0
    movss %xmm0, 12(%rcx)
    ret
.size fixture_copy_rect, .-fixture_copy_rect
#endif

.globl fixture_probe
.type fixture_probe,@function
fixture_probe:
    pushq %r12
    subq $0x20, %rsp
    movq %rdi, %r12
    movdqa .Lseed6(%rip), %xmm6
    movdqa .Lseed7(%rip), %xmm7
    movdqa .Lseed8(%rip), %xmm8
    movdqa .Lseed9(%rip), %xmm9
    call SAVED_XMM_PROBE_TARGET
    movdqu %xmm6, 0(%r12)
    movdqu %xmm7, 16(%r12)
    movdqu %xmm8, 32(%r12)
    movdqu %xmm9, 48(%r12)
    movd %xmm0, 64(%r12)
    addq $0x20, %rsp
    popq %r12
    ret
.size fixture_probe, .-fixture_probe

.section .rodata
.p2align 4
.Lseed6:
    .long 0x06060606, 0x06060607, 0x06060608, 0x06060609
.Lseed7:
    .long 0x07070706, 0x07070707, 0x07070708, 0x07070709
.Lseed8:
    .long 0x08080806, 0x08080807, 0x08080808, 0x08080809
.Lseed9:
    .long 0x09090906, 0x09090907, 0x09090908, 0x09090909
.section .note.GNU-stack,"",@progbits

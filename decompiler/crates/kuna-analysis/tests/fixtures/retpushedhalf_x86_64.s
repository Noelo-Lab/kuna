# (kuna) `retpushedhalf` fixture -- RE-friction round 12, need
# `alignment-push-pop-invents` (crackmes.one 67f9bdc38f555589f3530a85
# WeeperVM--Level_1, sub_10e27, a four-argument XOR decryptor returning its
# allocated buffer), reduced to the prologue/epilogue idiom that causes it.
#
# `xordec` is the witness, byte-for-byte the target's shape: the prologue pushes
# four callee-saved registers and then R8 purely to realign the stack, and the
# epilogue pops that fifth slot into RDX before restoring the four.  RDX at the
# RETURN therefore holds R8 as it arrived from the caller, which `retinputhalf`
# reads as "an input parameter the function placed in the return register" --
# inventing a fifth argument and a 128-bit return for a function that returns a
# pointer.  With `option retpushedhalf off` it renders
# `undefined16 xordec(long,int,long,int,unsigned long)` with `v._8_8_ = a4`;
# by default, `void * xordec(long,int,long,int)`.
#
# `regmove` is the control: the same `RDX = R8` at the RETURN, written by an
# ordinary `mov %r8,%rdx`.  Nothing was pushed, so the half is a real returned
# argument in both passes and the rendering must not move.
#
# Built with:  as -o f.o f.s && ld -o retpushedhalf_x86_64 f.o
        .text
        .globl xordec
        .type xordec, @function
xordec:
        push %r13
        mov  %rdx,%r13
        push %r12
        mov  %rdi,%r12
        push %rbp
        mov  %ecx,%ebp
        push %rbx
        movslq %esi,%rbx
        push %r8                # stack realignment, not a value
        lea  0x1(%rbx),%rax
        xor  %ecx,%ecx
        mov  %rax,%rsi
1:      cmp  %ecx,%ebx
        jle  2f
        mov  %ecx,%eax
        cltd
        idiv %ebp
        movslq %edx,%rdx
        mov  0(%r13,%rdx,1),%al
        xor  (%r12,%rcx,1),%al
        mov  %al,(%rsi,%rcx,1)
        inc  %rcx
        jmp  1b
2:      movb $0x0,(%rsi,%rbx,1)
        mov  %rsi,%rax
        pop  %rdx               # restores the stack, not a result
        pop  %rbx
        pop  %rbp
        pop  %r12
        pop  %r13
        ret
        .size xordec, .-xordec
        .balign 16, 0x90

        .globl regmove
        .type regmove, @function
regmove:
        lea  0x1(%rdi),%rax
        mov  %r8,%rdx
        ret
        .size regmove, .-regmove
        .balign 16, 0x90

        .globl _start
        .type _start, @function
_start:
        xor %edi,%edi
        xor %esi,%esi
        xor %edx,%edx
        xor %ecx,%ecx
        xor %r8d,%r8d
        call xordec
        call regmove
        ret
        .size _start, .-_start

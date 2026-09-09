# (kuna) `zeroidiomuse` fixture -- RE-friction round 12, need
# `argument-recovery-drops-both` (challenge 67f9bdc38f555589f3530a85, stripped
# ELF x86-64 `WeeperVM--Level_1`, sub_113d9 -> sub_110ff @0x11477), reduced to
# one loop.
#
# The loop head clears the two argument registers with the x86 self-XOR idiom
# before calling `reader`, and the loop body passes two fresh constants to
# `watchdog` and branches back.  The value handed to `watchdog` therefore
# reaches the `xor esi,esi` / `xor edi,edi` of the NEXT iteration -- through the
# killed-by-call INDIRECT, the loop-head MULTIEQUAL and the SUBPIECE that slices
# esi out of rsi -- and from there `call reader`, whose own trial for that
# register is active.  onlyOpUse sees a competing call use and sinks both of
# `watchdog`'s trials, so the call renders `watchdog()`.
#
# `INT_XOR(v,v)` is 0 whatever v is, so that competing use does not exist.  With
# `option zeroidiomuse off` the call renders `watchdog()`; with it on (the
# default), `watchdog(&timers,2)`.
#
# Built with:  as -o f.o f.s && ld -o zeroidiomuse_x86_64 f.o
        .text
        .globl reader
        .type reader, @function
reader:
        mov  %edi,%eax
        add  %esi,%eax
        ret
        .size reader, .-reader
        .balign 16, 0x90

        .globl watchdog
        .type watchdog, @function
watchdog:
        push %rbp
        mov  %rdi,%rbp
        xor  %edi,%edi
        push %rbx
        mov  %esi,%ebx
        push %rcx
        call now
        xor  %edx,%edx
        mov  %rax,%rcx
1:      cmp  %edx,%ebx
        jle  2f
        mov  (%rbp,%rdx,8),%rax
        mov  %rcx,%rsi
        inc  %rdx
        sub  (%rax),%rsi
        cmp  $2,%rsi
        jle  1b
        jmp  3f
2:      xor  %eax,%eax
3:      pop  %rdx
        pop  %rbx
        pop  %rbp
        ret
        .size watchdog, .-watchdog
        .balign 16, 0x90

        .globl now
        .type now, @function
now:
        mov  $201,%eax
        ret
        .size now, .-now
        .balign 16, 0x90

        .globl feed
        .type feed, @function
feed:
        push %r12
        push %rbx
        mov  %edi,%ebx
        xor  %r12d,%r12d
3:      cmp  %r12d,%ebx
        jle  4f
        xor  %esi,%esi
        xor  %edi,%edi
        call reader
        mov  $2,%esi
        mov  $timers,%edi
        inc  %r12d
        call watchdog
        test %rax,%rax
        jz   3b
4:      mov  %r12d,%eax
        pop  %rbx
        pop  %r12
        ret
        .size feed, .-feed
        .balign 16, 0x90

        .globl _start
        .type _start, @function
_start:
        mov  $3,%edi
        call feed
        mov  %eax,%edi
        mov  $60,%eax
        syscall
        .size _start, .-_start

        .data
        .globl timers
timers: .quad 0, 0

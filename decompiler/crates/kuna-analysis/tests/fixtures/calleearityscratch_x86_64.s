# (kuna) `calleearityscratch` fixture -- RE-friction round 9, need
# `hint-printing-call-loses` (challenge 5e51836e33c5d4439bb2dc24, ELF x86-64
# `cr4ckm3_0x`, 0x400f30 -> sub_417f80 @0x401007), reduced to four caller/callee
# pairs.
#
# Every callee reads rdi at its first instruction and then calls `sink`, so the
# callee-body walk `calleearitybody` takes is cut there: rdi proves input, rsi
# proves nothing either way, and `calleearitycut` is asked to bound the run
# `{rdi}` by the register it stops at.  Every caller passes its own incoming rdi
# straight through (`AncestorRealistic::execute` refuses an input Varnode, so it
# scores inactive and the site recovers nothing at all) and differs only in what
# it puts in rsi:
#
#   `scratch_wide`   loads rsi from memory and ADDS it into rbx -- the caller
#                    consumes the value, so rsi's trial scores inactive.  With
#                    `option calleearityscratch off` the call renders `cal1()`;
#                    with it on, `cal1(a0)`.
#   `scratch_byte`   the witness's own shape: `movzbl` into esi and an `xor` of
#                    sil into memory.  Same flip.
#   `boundary_live`  writes rsi and uses it for NOTHING but the call, so its
#                    trial scores active and the site recovers rsi on its own;
#                    unchanged in both passes.
#   `boundary_const` materializes a constant into rsi and adds it into rbx too --
#                    inactive, but a constant is never scratch, so the run is
#                    declined in both passes.
#
# Built with:  as -o f.o f.s && ld -o calleearityscratch_x86_64 f.o
        .text
        .globl sink
        .type sink, @function
sink:
        ret
        .size sink, .-sink

        .globl cal1
        .type cal1, @function
cal1:
        mov  %rdi,%rax
        call sink
        ret
        .size cal1, .-cal1

        .globl cal2
        .type cal2, @function
cal2:
        mov  %rdi,%rax
        call sink
        ret
        .size cal2, .-cal2

        .globl cal3
        .type cal3, @function
cal3:
        mov  %rdi,%rax
        call sink
        ret
        .size cal3, .-cal3

        .globl cal4
        .type cal4, @function
cal4:
        mov  %rdi,%rax
        call sink
        ret
        .size cal4, .-cal4

        .globl scratch_wide
        .type scratch_wide, @function
scratch_wide:
        push %rbx
        mov  %rdi,%rbx
        mov  (%rdi),%rsi
        add  %rsi,%rbx
        call cal1
        add  %ebx,%eax
        pop  %rbx
        ret
        .size scratch_wide, .-scratch_wide

        .globl scratch_byte
        .type scratch_byte, @function
scratch_byte:
        push %rbx
        mov  %rdi,%rbx
        movzbl (%rdi),%esi
        xor  %sil,0x4(%rdi)
        call cal2
        add  %ebx,%eax
        pop  %rbx
        ret
        .size scratch_byte, .-scratch_byte

        .globl boundary_live
        .type boundary_live, @function
boundary_live:
        push %rbx
        mov  %rdi,%rbx
        mov  (%rdi),%rsi
        call cal3
        add  %ebx,%eax
        pop  %rbx
        ret
        .size boundary_live, .-boundary_live

        .globl boundary_const
        .type boundary_const, @function
boundary_const:
        push %rbx
        mov  %rdi,%rbx
        mov  $2,%esi
        add  %rsi,%rbx
        call cal4
        add  %ebx,%eax
        pop  %rbx
        ret
        .size boundary_const, .-boundary_const

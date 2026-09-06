# (kuna) `calleearitylive` fixture -- RE-friction round 4, need
# `argument-recovery-options-still` (challenge 69761b7a39e9c4d85c2f9fc1, ELF
# x86-64 `graphy`), reduced to two callee/caller pairs.
#
# `caller` and `caller2` are identical: they compute the 4th argument in ecx,
# BRANCH on it, and call the same callee twice -- once with that branched-on
# value and once with a freshly computed one.  onlyOpUse rejects the trial with
# the competing CBRANCH use, so the first site renders with three arguments and
# the second with four.
#
# They differ only in the callee.  `callee` reads exactly rdi/esi/edx/ecx, so
# the four-argument witness IS its whole register argument list and the short
# site is extended.  `callee2` also reads r8d -- a register the witness does not
# claim, the same shape a variadic register-save prologue has -- so the callee's
# own body contradicts the witness and the short site is left alone.
#
# Built with:  as -o f.o f.s && ld -o calleearitylive_x86_64 f.o
        .text
        .globl callee
        .type callee, @function
callee:
        mov  %rdi,%rax
        add  %esi,%eax
        add  %edx,%eax
        add  %ecx,%eax
        ret
        .size callee, .-callee
        .balign 16, 0x90

        .globl callee2
        .type callee2, @function
callee2:
        mov  %rdi,%rax
        add  %esi,%eax
        add  %edx,%eax
        add  %ecx,%eax
        add  %r8d,%eax
        ret
        .size callee2, .-callee2
        .balign 16, 0x90

        .globl caller
        .type caller, @function
caller:
        mov  %rsi,%r8
        mov  %rdx,%r9
        lea  0x8000(%r9),%ecx
        mov  $4,%esi
        mov  %r8,%rdx
        cmp  $0xffff,%ecx
        ja   1f
        call callee
        ret
1:      shr  $0x10,%r9
        lea  0x8000(%r9),%ecx
        mov  $4,%esi
        mov  %r8,%rdx
        call callee
        ret
        .size caller, .-caller
        .balign 16, 0x90

        .globl caller2
        .type caller2, @function
caller2:
        mov  %rsi,%r10
        mov  %rdx,%r11
        lea  0x8000(%r11),%ecx
        mov  $4,%esi
        mov  %r10,%rdx
        cmp  $0xffff,%ecx
        ja   2f
        call callee2
        ret
2:      shr  $0x10,%r11
        lea  0x8000(%r11),%ecx
        mov  $4,%esi
        mov  %r10,%rdx
        call callee2
        ret
        .size caller2, .-caller2
        .balign 16, 0x90

        .globl _start
        .type _start, @function
_start:
        xor  %edi,%edi
        xor  %esi,%esi
        xor  %edx,%edx
        call caller
        call caller2
        ret
        .size _start, .-_start

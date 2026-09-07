# (kuna) `calleearitybody` fixture -- RE-friction round 7, need
# `defined-payload-function-loses` (challenge 68bffbe9224c0ec5dcedbe7a, PE
# x86-64 `runtime.exe`, sub_14000b62c -> payload @0x14000ca0c), reduced to one
# caller/callee pair.
#
# `callee` is called exactly ONCE, which is the whole point: every sibling rule
# in the calleearity family needs a second call to the same entry address to
# compare against, and there is not one.  It reads rdi before writing it (the
# dereference) and overwrites rsi before ever reading it, so its own body says
# the argument list is exactly one register long.
#
# `caller` passes rdi straight through, and also branches on it -- the
# `test rdi,rdi; je; call` shape that makes onlyOpUse reject the trial on its
# CBRANCH descendant.  With `option calleearitybody off` the call renders
# `callee()`; with it on, `callee(a0)`.
#
# Built with:  as -o f.o f.s && ld -o calleearitybody_x86_64 f.o
        .text
        .globl callee
        .type callee, @function
callee:
        movq $0,%rsi
        mov  (%rdi),%eax
        ret
        .size callee, .-callee
        .balign 16, 0x90

        .globl caller
        .type caller, @function
caller:
        test %rdi,%rdi
        je   1f
        call callee
        ret
1:      xor  %eax,%eax
        ret
        .size caller, .-caller
        .balign 16, 0x90

        .globl _start
        .type _start, @function
_start:
        mov  %rsp,%rdi
        call caller
        mov  %eax,%edi
        mov  $60,%eax
        syscall
        .size _start, .-_start

# (kuna) `calleearitycut` fixture -- RE-friction round 8, need
# `bytecode-reader-call-site` (challenge 673da52e9b533b4c22bd2eeb, ELF x86-64
# `Layers`, sub_9d9c0 -> sub_875e0 @0x9d9d8), reduced to three caller/callee
# pairs.
#
# `callee` reads all four SysV argument registers and only THEN calls `sink`.
# The callee-body walk `calleearitybody` takes ends every path at that call, so
# r8 -- which a real callee would clobber later -- is never proven dead and the
# read run has no boundary.  `calleearitycut` supplies one from the run's own
# shape: it stops short of the last argument register, and the register it stops
# at is quiet.
#
# `caller` passes rdi/rsi/rdx straight through from its own incoming parameters
# (`AncestorRealistic::execute` refuses an input Varnode outright, so all three
# score inactive) and writes ecx=1 right before the CALL; the `cmp rsi,rdx`
# gives rsi/rdx the competing use that keeps them live into the call.  With
# `option calleearitycut off` the call renders `callee()`; with it on,
# `callee(a0,a1,a2,1)`.
#
# The other two are the negative controls, declined in BOTH passes:
#   `caller2` calls `callee2`, which reads EVERY argument register, so the run
#            reaches the last one and has no boundary at all;
#   `caller3` writes `mov $2,%r8d` before the CALL, so the register the run
#            stops at is carrying a further argument.
#
# Built with:  as -o f.o f.s && ld -o calleearitycut_x86_64 f.o
        .text
        .globl sink
        .type sink, @function
sink:
        ret
        .size sink, .-sink

        .globl callee
        .type callee, @function
callee:
        mov  %rdi,%rax
        add  %esi,%eax
        add  %edx,%eax
        add  %ecx,%eax
        call sink
        ret
        .size callee, .-callee

        .globl callee2
        .type callee2, @function
callee2:
        mov  %rdi,%rax
        add  %esi,%eax
        add  %edx,%eax
        add  %ecx,%eax
        add  %r8d,%eax
        add  %r9d,%eax
        call sink
        ret
        .size callee2, .-callee2

        .globl caller
        .type caller, @function
caller:
        push %rbx
        mov  %rdi,%rbx
        cmp  %rsi,%rdx
        je   1f
        mov  $1,%ecx
        call callee
        add  %ebx,%eax
        pop  %rbx
        ret
1:      xor  %eax,%eax
        pop  %rbx
        ret
        .size caller, .-caller

        .globl caller2
        .type caller2, @function
caller2:
        push %rbx
        mov  %rdi,%rbx
        cmp  %rsi,%rdx
        je   1f
        mov  $1,%ecx
        call callee2
        add  %ebx,%eax
        pop  %rbx
        ret
1:      xor  %eax,%eax
        pop  %rbx
        ret
        .size caller2, .-caller2

        .globl caller3
        .type caller3, @function
caller3:
        push %rbx
        mov  %rdi,%rbx
        cmp  %rsi,%rdx
        je   1f
        mov  $1,%ecx
        mov  $2,%r8d
        call callee
        add  %ebx,%eax
        pop  %rbx
        ret
1:      xor  %eax,%eax
        pop  %rbx
        ret
        .size caller3, .-caller3

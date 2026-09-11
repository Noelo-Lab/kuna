# (kuna) `exclusivearguse` fixture -- RE-friction round 12, need
# `bytecode-append-call-loses` (challenge 69b7492049fa49a2a260218f, stripped
# MSVC x86-64 PE `vm.exe`, sub_140004540 -> sub_140007be0 @0x1400046cc),
# reduced to one branch and re-spelled in the SysV ABI.
#
# `append` sets both of the slow path's arguments up BEFORE the capacity test,
# because the inlined one-element fast path needs them too: it LOADs through
# `rdx` and STOREs through `rsi` on the arm the call is not on.  Both accesses
# are descendants of the Varnodes the call is passed, so onlyOpUse
# (funcdata_varnode.cc:1851) rejects both trials and `append_slow(c,end,src)`
# renders `append_slow(c)`.  No execution path runs both arms, so neither
# access can be competing with the call.
#
# `append_loop` is the control: the same two arms, inside a loop.  The back edge
# makes each arm reachable from the other, so the rule declines it and the call
# renders with one argument in BOTH passes.
#
# Built with:  as -o f.o f.s && ld -o exclusivearguse_x86_64 f.o
        .text
        .globl append_slow
        .type append_slow, @function
append_slow:                    # (container, end, src)
        mov  (%rdi),%rax
        mov  %rsi,8(%rdi)
        movzbl (%rdx),%r10d
        mov  %r10b,(%rax)
        mov  %rax,%rdx
        ret
        .size append_slow, .-append_slow
        .balign 16, 0x90

        .globl append
        .type append, @function
append:                         # (container, byte)
        movzbl %sil,%eax
        lea  table(%rip),%rdx
        add  %rax,%rdx          # rdx = &table[byte]      -- the slow path's 3rd argument
        mov  8(%rdi),%rsi       # rsi = container->end    -- the slow path's 2nd argument
        cmp  16(%rdi),%rsi
        je   .Lslow
        movzbl (%rdx),%eax      # fast path: LOAD through rdx
        mov  %al,(%rsi)         # fast path: STORE through rsi
        addq $1,8(%rdi)
        jmp  .Ldone
.Lslow:
        call append_slow
.Ldone:
        ret
        .size append, .-append
        .balign 16, 0x90

        .globl append_loop
        .type append_loop, @function
append_loop:                    # (container, src) -- the control
        push %rbx
        push %rbp
        mov  %rdi,%rbx
        mov  %rsi,%rbp
.Ltop:
        mov  %rbx,%rdi          # the container, used only by the call
        mov  %rbp,%rdx          # the source byte
        mov  8(%rbx),%rsi       # the write cursor
        cmp  16(%rbx),%rsi
        je   .Lslow2
        movzbl (%rdx),%eax      # same fast path as `append` ...
        mov  %al,(%rsi)
        addq $1,8(%rbx)
        jmp  .Lnext
.Lslow2:
        call append_slow        # ... but the back edge below makes the two
.Lnext:                         #     arms reachable from each other
        subl $1,24(%rbx)
        jne  .Ltop
        pop  %rbp
        pop  %rbx
        ret
        .size append_loop, .-append_loop

        .data
        .globl table
table:
        .byte 0,1,2,3,4,5,6,7

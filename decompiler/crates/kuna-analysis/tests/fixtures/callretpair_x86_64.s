# (kuna) `callretpair` fixture -- RE-friction round 12, need
# `bytecode-reader-return-discarded` (challenge 673da52e9b533b4c22bd2eeb, a
# stripped gcc 9.4 QuickJS interpreter, caller sub_9d9c0 -> sub_875e0 @0x9d9d8),
# reduced to one call and re-spelled with explicit symbols.
#
# `read_value` returns a QuickJS-shaped `JSValue` -- `{ void *u; long tag; }`,
# sixteen bytes -- which the System V x86-64 ABI hands back in RAX:RDX.  The
# x86-64 cspec's <join_dual_class/> output rule describes exactly that storage,
# so `read_tag` puts two active output trials on its call; the arm of
# buildOutputFromTrials that turns two trials into an output was a stub, so the
# call got no output at all and both halves rendered as locals the function
# never assigns.
#
# `read_tag_clobber` is the control: `no_payload` provably never writes RDX, so
# the caller's read of RDX is a clobber and the bounded callee decode refuses the
# pair in BOTH passes.
#
# Built with:  as -o f.o f.s && ld -o callretpair_x86_64 f.o
        .text
        .globl read_value
        .type read_value, @function
read_value:                     # (ctx, buf, len, flags) -> JSValue in RAX:RDX
        mov  (%rsi),%rax        # u   = *buf
        mov  8(%rsi),%rdx       # tag = buf[1]
        test %ecx,%ecx
        jne  .Lrv_done
        mov  $6,%edx            # tag = JS_TAG_EXCEPTION
        xor  %eax,%eax
.Lrv_done:
        ret
        .size read_value, .-read_value
        .balign 16, 0x90

        .globl read_tag
        .type read_tag, @function
read_tag:                       # (ctx, buf) -> long
        push %rbx
        mov  $1,%ecx
        mov  $16,%edx
        call read_value
        mov  %rax,%rbx          # keep the payload half
        cmp  $6,%edx            # test the tag half
        je   .Lrt_bad
        mov  (%rbx),%rax
        add  %rdx,%rax
        pop  %rbx
        ret
.Lrt_bad:
        mov  $-1,%rax
        pop  %rbx
        ret
        .size read_tag, .-read_tag
        .balign 16, 0x90

        .globl no_payload
        .type no_payload, @function
no_payload:                     # (ctx) -> long in RAX only; never writes RDX
        mov  (%rdi),%rax
        add  $1,%rax
        ret
        .size no_payload, .-no_payload
        .balign 16, 0x90

        .globl read_tag_clobber
        .type read_tag_clobber, @function
read_tag_clobber:               # (ctx) -- the control
        push %rbx
        mov  $7,%edx
        call no_payload
        mov  %rax,%rbx
        cmp  $6,%edx
        je   .Lrc_bad
        mov  (%rbx),%rax
        add  %rdx,%rax
        pop  %rbx
        ret
.Lrc_bad:
        mov  $-1,%rax
        pop  %rbx
        ret
        .size read_tag_clobber, .-read_tag_clobber

        .globl _start
        .type _start, @function
_start:
        lea  buf(%rip),%rsi
        xor  %edi,%edi
        call read_tag
        mov  %rax,%rdi
        call read_tag_clobber
        mov  $60,%eax
        syscall
        .size _start, .-_start

        .data
        .balign 16
buf:    .quad 0, 3

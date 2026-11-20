@ One function per condition of the literal-pool fall-through bound (see
@ fixtures/README.md). `die` is an undefined import, so its PLT stub ends in a
@ computed jump and nothing can prove whether it returns; `sink` returns, and is
@ hidden so a call to it binds directly rather than through a PLT stub.
@ LDFLAGS='-shared -nostdlib -Wl,--build-id=none -Wl,-z,max-page-size=0x1000
@   -Wl,-z,noseparate-code'
@ arm-linux-gnueabi-as -march=armv5te -mfpu=vfp poolguards.s -o poolguards.o
@ arm-linux-gnueabi-gcc $LDFLAGS poolguards.o -o poolguards_arm_le32
@ arm-linux-gnueabi-as -march=armv5te -mfpu=vfp -mbig-endian poolguards.s -o poolguards.o
@ arm-linux-gnueabi-gcc $LDFLAGS -mbig-endian -Wl,-EB poolguards.o -o poolguards_arm_be32
        .text
        .arm
        .fpu    vfp
        .globl  sink
        .hidden sink
        .type   sink, %function
sink:   bx      lr

@ The call is conditional, so its fall-through is live code.
        .globl  cond_call
        .type   cond_call, %function
cond_call:
        ldr     r1, .Lcond_pool
        cmp     r0, #0
        blne    die
.Lcond_pool:
        .word   0x0007943c

@ The word after the call is a branch target, so it is code.
        .globl  labelled_word
        .type   labelled_word, %function
labelled_word:
        ldr     r1, .Llab_word
        cmp     r0, #0
        beq     .Llab_word
        bl      die
.Llab_word:
        mov     r0, #1
        bx      lr

@ The word after the call is read a halfword at a time, not whole.
        .globl  halfword_read
        .type   halfword_read, %function
halfword_read:
        ldrh    r1, .Lhalf_pool
        bl      die
.Lhalf_pool:
        mov     r0, #2
        bx      lr

@ The call's fall-through is the function's pool: the bound applies here.
        .globl  bounded_call
        .type   bounded_call, %function
bounded_call:
        ldr     r1, .Lbound_pool
        bl      die
        .align  2
.Lbound_pool:
        .word   0x0002cbb2

@ One instruction of alignment padding between the call and the pool: bounded.
        .globl  pad_call
        .type   pad_call, %function
pad_call:
        ldr     r1, .Lpad_pool
        bl      die
        nop
.Lpad_pool:
        .word   0x0005f9fb

@ The gap is live code, not padding, so the fall-through stands.
        .globl  live_gap
        .type   live_gap, %function
live_gap:
        ldr     r1, .Llive_pool
        bl      die
        mov     r0, #7
.Llive_pool:
        .word   0x00079198

@ The gap is wider than one instruction, so the fall-through stands.
        .globl  wide_gap
        .type   wide_gap, %function
wide_gap:
        ldr     r1, .Lwide_pool
        bl      die
        nop
        nop
.Lwide_pool:
        .word   0x00021ba1

@ The word after the call is in writable bytes, not the instruction stream.
        .section .wtext,"awx",%progbits
        .globl  data_word
        .type   data_word, %function
data_word:
        ldr     r1, .Ldata_pool
        bl      die
.Ldata_pool:
        .word   0x00079198
        .text

@ An indirect call is bounded at its pool too.
        .globl  indirect_call
        .type   indirect_call, %function
indirect_call:
        ldr     r3, .Lind_pool
        blx     r3
        .align  2
.Lind_pool:
        .word   0x0002cea6

@ The callee provably returns and the pool's only load sits past a call, so
@ the word after the second call is reached code, whatever reads it.
        .globl  returning_callee
        .type   returning_callee, %function
returning_callee:
        bl      sink
        ldr     r1, .Lret_pool
        bl      sink
.Lret_pool:
        .word   0x0007943c

@ The pool's only reader sits past a call nothing proves returns, so it may
@ itself be data and proves nothing.
        .globl  unproven_reader
        .type   unproven_reader, %function
unproven_reader:
        bl      die
        ldr     r1, .Lunp_pool
        bl      die
.Lunp_pool:
        .word   0x00021ba1

@ A non-returning `svc` falls through into a data word that decodes as a load
@ of the returning call's fall-through (`ldr r2,[pc,#-24]` names `.Lsj_live`).
@ The word is past a user operation, so its load witnesses nothing and the
@ live `return 5` stands.
        .globl  svc_junk
        .type   svc_junk, %function
svc_junk:
        push    {r4, lr}
        cmp     r0, #0
        beq     .Lsj_err
        bl      sink
.Lsj_live:
        mov     r0, #5
        pop     {r4, pc}
.Lsj_err:
        mov     r7, #1
        svc     0
        .word   0xe51f2018

@ The pool word spells a PC-relative load of the live code after a returning
@ call. It is still the pool, read by the trusted load at the entry, so the
@ final call is bounded and the live code before it is kept.
        .globl  junk_pointer
        .type   junk_pointer, %function
junk_pointer:
        ldr     r1, .Ljp_pool
        bl      sink
.Ljp_live:
        add     r0, r0, #1
        mov     r4, r0
        bl      die
.Ljp_pool:
        .word   0xe51f2000 + (.Ljp_pool + 8 - .Ljp_live)

@ Two call-and-pool sites in one function: both are bounded.
        .globl  two_pools
        .type   two_pools, %function
two_pools:
        cmp     r0, #0
        beq     .Ltp_b
        ldr     r1, .Ltp_p1
        bl      die
.Ltp_p1:
        .word   0x0002cbb2
.Ltp_b:
        ldr     r1, .Ltp_p2
        bl      die
.Ltp_p2:
        .word   0x0002cea6

@ A VFP doubleword literal, loaded through `vldr`, is a pool too.
        .globl  wide_literal
        .type   wide_literal, %function
wide_literal:
        vldr    d0, .Lwl_pool
        bl      die
        .align  3
.Lwl_pool:
        .double 1.5

@ The pool's loads sit in a block after the call in address order.
        .globl  ordered_loads
        .type   ordered_loads, %function
ordered_loads:
        cmp     r0, #0
        bne     .Lol_x
        bl      die
.Lol_pool:
        .word   0x0002cbb2
.Lol_x:
        ldr     r0, .Lol_pool
        bx      lr

@ A terminator, so the last case above falls through to a function entry rather
@ than off the end of the image.
        .globl  tail_stub
        .type   tail_stub, %function
tail_stub:
        mov     r0, #0
        bx      lr

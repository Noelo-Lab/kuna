! sparc64-linux-gnu-as -32 -o /tmp/kuna-sparc32-call-gap.o kuna-sparc32-call-gap.S
! sparc64-linux-gnu-ld -m elf32_sparc -Ttext 0x10000 -e seed0 -o kuna-sparc32-call-gap /tmp/kuna-sparc32-call-gap.o
        .text
        .irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19
        .globl seed\n
        .type seed\n,@function
seed\n:
        save %sp, -96, %sp
        mov 42, %i0
        ret
         restore
        .size seed\n, .-seed\n
        .endr

        ! A gap entry ends with a call that cannot return.
        ! AIF must not consume the independently callable frame after it.
        save %sp, -96, %sp
        mov 42, %i0
        call abort
         nop
.Lcallee:
        save %sp, -96, %sp
        mov 42, %i0
        ret
         restore

        .globl abort
        .type abort,@function
abort:
        ba abort
         nop
        .size abort, .-abort

        ! A separate hidden caller names the second frame in the original code.
        save %sp, -96, %sp
        call .Lcallee
         nop
        ret
         restore

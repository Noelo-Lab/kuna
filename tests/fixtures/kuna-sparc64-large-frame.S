! sparc64-linux-gnu-as -64 -o /tmp/kuna-sparc64-large-frame.o kuna-sparc64-large-frame.S
! sparc64-linux-gnu-ld -Ttext 0x10000 -e seed0 -o kuna-sparc64-large-frame /tmp/kuna-sparc64-large-frame.o
        .text
        .irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19
        .globl seed\n
        .type seed\n,@function
seed\n:
        mov 42, %o0
        add %o0, 1, %o0
        retl
         nop
        .size seed\n, .-seed\n
        .endr

        ! 0x10140: not in the symbol table and no seed calls into it.
        sethi %hi(0x2000), %g1
        xor %g1, -0xd0, %g1
        save %sp, %g1, %sp
        call seed0
         nop
        ret
         restore

        ! 0x1015c: the XOR writes the wrong register for the frame setup.
        sethi %hi(0x2000), %g1
        xor %g1, -0xd0, %g2
        save %sp, %g1, %sp
        call seed0
         nop
        ret
         restore

        ! 0x10178: a matching prefix without a corroborating flow edge.
        sethi %hi(0x2000), %g1
        xor %g1, -0xd0, %g1
        save %sp, %g1, %sp
        ret
         restore

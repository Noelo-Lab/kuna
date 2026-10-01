        .section ".text"
        .align 4

        .global counter_bump
        .type counter_bump, #function
counter_bump:
        sethi   %hi(counter), %g1
        ld      [%g1 + %lo(counter)], %o1
        add     %o1, %o0, %o0
        retl
         st     %o0, [%g1 + %lo(counter)]
        .size counter_bump, .-counter_bump

        .global report_total
        .type report_total, #function
report_total:
        save    %sp, -96, %sp
        call    counter_bump
         mov    %i0, %o0
        mov     %o0, %o1
        sethi   %hi(.Lformat), %o0
        call    printf
         or     %o0, %lo(.Lformat), %o0
        ret
         restore %g0, 0, %o0
        .size report_total, .-report_total

        .section ".rodata"
        .align 8
.Lformat:
        .asciz  "total %d\n"

        .section ".data"
        .align 4
counter:
        .word   0

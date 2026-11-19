// Source of a64.o (tests/stages/kuna-reloc-families.xml):
//   zig cc -target aarch64-freestanding-none -c a64.s -o a64.o

    .text
    .globl probe
    .type probe,%function
probe:
    adr x8, ext_tab
    ldr w9, ext_word
    cbz w0, zero
    tbnz w0, #3, odd
    movz x10, #:abs_g1:ext_tab
    movk x10, #:abs_g0_nc:ext_tab
    ldr w0, [x8, #4]
    add w0, w0, w9
    ldr w11, [x10, #8]
    add w0, w0, w11
    ret
    .globl odd
    .type odd,%function
odd:
    cmp w1, #5
    b.ne zero
    b ext_fn
    .globl zero
    .type zero,%function
zero:
    mov w0, #0
    ret

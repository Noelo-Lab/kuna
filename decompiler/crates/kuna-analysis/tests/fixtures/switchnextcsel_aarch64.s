// A byte-masked dispatch that is its function's last table, followed by
// the table of a function whose index a `csel` with a constant operand
// selects. `name_of` indexes tab_names (3 bytes) with `x0 & 0xff`, so the
// basic model reads 256 rows; the rows past its own three are pick's table
// and the bytes after it: 1 and 3 land inside name_of, 13 in pick, 0 on
// its first case, so the run-on cut keeps every row. pick's index comes
// from a csel/csinc chain over constants (0, 1, 2 or 4) and is added to the
// table's address before the load; each csel/csinc lifts to a p-code
// branch inside the instruction, so a linear fold that takes the last copy
// as the value reads the chain as the constant 1, the address as one
// constant and pick's table as a constant load. pick's table, right after
// tab_names, ends name_of's.
    .text
    .globl name_of
    .type name_of, %function
    .p2align 2
name_of:
    and     x8, x0, #0xff
    adrp    x9, tab_names
    add     x9, x9, :lo12:tab_names
    adr     x10, .Ln_base
    ldrb    w11, [x9, x8]
    add     x10, x10, x11, lsl #2
    br      x10
.Ln_base:
    mov     w0, #30
    ret
.Ln_c1:
    mov     w0, #31
    ret
.Ln_c2:
    mov     w0, #32
    ret
    .size name_of, . - name_of

    .globl pick
    .type pick, %function
    .p2align 2
pick:
    mov     w9, #4
    mov     w10, #2
    cmp     w0, #3
    csel    w9, w10, w9, lo
    cmp     w1, #0xff
    csinc   w8, w9, wzr, hs
    cmp     w2, #7
    csel    w8, wzr, w8, lo
    adrp    x9, tab_pick
    add     x9, x9, :lo12:tab_pick
    adr     x10, .Lp_base
    add     x9, x9, x8
    ldrb    w11, [x9]
    add     x10, x10, x11, lsl #2
    br      x10
.Lp_base:
    nop
.Lp_c0:
    mov     w0, #40
    ret
.Lp_c1:
    mov     w0, #41
    ret
    .size pick, . - pick

    .section .rodata
tab_names:
    .byte (.Ln_base - .Ln_base) / 4, (.Ln_c1 - .Ln_base) / 4, (.Ln_c2 - .Ln_base) / 4
tab_pick:
    .byte (.Lp_c0 - .Lp_base) / 4, (.Lp_c1 - .Lp_base) / 4, (.Lp_c0 - .Lp_base) / 4
    .byte (.Lp_c1 - .Lp_base) / 4, (.Lp_c0 - .Lp_base) / 4
    .byte 13
    .space 256

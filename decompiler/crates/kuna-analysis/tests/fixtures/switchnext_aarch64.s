// A byte-masked dispatch that is its function's last table: `name_of`
// indexes tab_names (3 halfwords) with `x0 & 0xff`, so the basic model reads
// 256 rows. The rows past its own three are next_kind's table and the zero
// bytes after it, measured from name_of's base: they land past the
// function and back inside it. next_kind's table, right after tab_names,
// ends name_of's.
    .text
    .globl name_of
    .type name_of, %function
    .p2align 2
name_of:
    and     x8, x0, #0xff
    adrp    x9, tab_names
    add     x9, x9, :lo12:tab_names
    adr     x10, .Ln_base
    ldrh    w11, [x9, x8, lsl #1]
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

    .globl next_kind
    .type next_kind, %function
    .p2align 2
next_kind:
    cmp     w0, #3
    b.hi    .Lk_default
    adrp    x8, tab_next
    add     x8, x8, :lo12:tab_next
    adr     x10, .Lk_base
    ldrh    w9, [x8, w0, uxtw #1]
    add     x10, x10, x9, lsl #2
    br      x10
.Lk_base:
    .rept 24
    nop
    .endr
    mov     w0, #40
    ret
.Lk_c1:
    mov     w0, #41
    ret
.Lk_c2:
    mov     w0, #42
    ret
.Lk_c3:
    mov     w0, #43
    ret
.Lk_default:
    mov     w0, #-1
    ret
    .size next_kind, . - next_kind

    .section .rodata
    .p2align 1
tab_names:
    .hword (.Ln_base - .Ln_base) / 4, (.Ln_c1 - .Ln_base) / 4, (.Ln_c2 - .Ln_base) / 4
tab_next:
    .hword (.Lk_base - .Lk_base) / 4 + 24, (.Lk_c1 - .Lk_base) / 4, (.Lk_c2 - .Lk_base) / 4, (.Lk_c3 - .Lk_base) / 4
    .space 512

// AArch64 compact jump tables, assembled into `switchextent_aarch64`
// (see README.md). `guarded` is the clang shape behind a `cmp`/`b.hi`;
// `entering` has its range check branch into the dispatch block;
// `unchecked` indexes two tables straight with a call's result, the first
// bounded only by the second; `last` indexes one unchecked table followed by
// a byte array it reads, which must not bound it.
    .text
    .globl guarded
    .type guarded, %function
    .p2align 2
guarded:
    cmp     w0, #3
    b.hi    .Lg_default
    adrp    x8, tab_guarded
    add     x8, x8, :lo12:tab_guarded
    adr     x10, .Lg_base
    ldrb    w9, [x8, w0, uxtw]
    add     x10, x10, x9, lsl #2
    br      x10
.Lg_base:
    mov     w0, #10
    ret
.Lg_c1:
    mov     w0, #11
    ret
.Lg_c2:
    mov     w0, #12
    ret
.Lg_c3:
    mov     w0, #13
    ret
.Lg_default:
    mov     w0, #-1
    ret
    .size guarded, . - guarded

    .globl entering
    .type entering, %function
    .p2align 2
entering:
    cmp     w0, #2
    b.ls    .Le_dispatch
    mov     w0, #-1
    ret
.Le_dispatch:
    adrp    x8, tab_entering
    add     x8, x8, :lo12:tab_entering
    adr     x10, .Le_base
    ldrb    w9, [x8, w0, uxtw]
    add     x10, x10, x9, lsl #2
    br      x10
.Le_base:
    mov     w0, #20
    ret
.Le_c1:
    mov     w0, #21
    ret
.Le_c2:
    mov     w0, #22
    ret
    .size entering, . - entering

    .globl pick
    .type pick, %function
    .p2align 2
pick:
    and     w0, w0, #7
    ret
    .size pick, . - pick

    .globl unchecked
    .type unchecked, %function
    .p2align 2
unchecked:
    stp     x29, x30, [sp, #-32]!
    stp     x19, x20, [sp, #16]
    mov     x29, sp
    mov     w19, w1
    bl      pick
    adrp    x8, tab_first
    add     x8, x8, :lo12:tab_first
    adr     x10, .Lu_base
    ldrh    w9, [x8, w0, uxtw #1]
    add     x10, x10, x9, lsl #2
    br      x10
.Lu_base:
    mov     w20, #30
    b       .Lu_join
.Lu_c1:
    mov     w20, #31
    b       .Lu_join
.Lu_c2:
    mov     w20, #32
.Lu_join:
    mov     w0, w19
    bl      pick
    adrp    x8, tab_second
    add     x8, x8, :lo12:tab_second
    adr     x10, .Lv_base
    ldrh    w9, [x8, w0, uxtw #1]
    add     x10, x10, x9, lsl #2
    br      x10
.Lv_base:
    add     w0, w20, #1
    b       .Lu_ret
.Lv_c1:
    add     w0, w20, #2
    b       .Lu_ret
.Lv_c2:
    add     w0, w20, #3
    b       .Lu_ret
.Lv_c3:
    add     w0, w20, #4
.Lu_ret:
    ldp     x19, x20, [sp, #16]
    ldp     x29, x30, [sp], #32
    ret
    .size unchecked, . - unchecked

    .globl last
    .type last, %function
    .p2align 2
last:
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp
    bl      pick
    adrp    x8, tab_last
    add     x8, x8, :lo12:tab_last
    adr     x10, .Ll_base
    ldrh    w9, [x8, w0, uxtw #1]
    add     x10, x10, x9, lsl #2
    br      x10
.Ll_base:
    mov     w0, #40
    b       .Ll_ret
.Ll_c1:
    adrp    x8, bytes_last
    add     x8, x8, :lo12:bytes_last
    ldrb    w0, [x8, #2]
.Ll_ret:
    ldp     x29, x30, [sp], #16
    ret
    .size last, . - last

    .section .rodata,"a",@progbits
    .p2align 2
tab_guarded:
    .byte   (.Lg_base - .Lg_base) / 4
    .byte   (.Lg_c1 - .Lg_base) / 4
    .byte   (.Lg_c2 - .Lg_base) / 4
    .byte   (.Lg_c3 - .Lg_base) / 4
tab_entering:
    .byte   (.Le_base - .Le_base) / 4
    .byte   (.Le_c1 - .Le_base) / 4
    .byte   (.Le_c2 - .Le_base) / 4
    .byte   0
tab_first:
    .hword  (.Lu_base - .Lu_base) / 4
    .hword  (.Lu_c1 - .Lu_base) / 4
    .hword  (.Lu_c2 - .Lu_base) / 4
tab_second:
    .hword  (.Lv_base - .Lv_base) / 4
    .hword  (.Lv_c1 - .Lv_base) / 4
    .hword  (.Lv_c2 - .Lv_base) / 4
    .hword  (.Lv_c3 - .Lv_base) / 4
tab_last:
    .hword  (.Ll_base - .Ll_base) / 4
    .hword  (.Ll_c1 - .Ll_base) / 4
bytes_last:
    .byte   1, 2, 3, 4, 5, 6, 7, 8

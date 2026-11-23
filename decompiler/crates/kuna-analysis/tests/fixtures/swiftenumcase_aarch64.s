// A Swift multi-payload enum `switch`, assembled into `swiftenumcase_aarch64`
// (see README.md). `dispatch` indexes one halfword table straight with the
// case `swift_getEnumCaseMultiPayload` returns for the enum whose metadata
// `Shape_accessor` returns, with no range check and no table after it; the
// `swift5_type_metadata` record names that enum's descriptor, which gives it
// two payload cases and one empty case. `extra` sits after the table and
// must not be read as rows.
    .text
    .globl dispatch
    .type dispatch, %function
    .p2align 2
dispatch:
    stp     x29, x30, [sp, #-32]!
    stp     x19, x20, [sp, #16]
    mov     x29, sp
    mov     x19, x0
    mov     x0, #0
    bl      Shape_accessor
    mov     x20, x0
    mov     x0, x19
    bl      consume
    mov     x0, x19
    mov     x1, x20
    bl      swift_getEnumCaseMultiPayload
    mov     w8, w0
    adrp    x9, tab_shape
    add     x9, x9, :lo12:tab_shape
    adr     x10, .Ld_base
    ldrh    w11, [x9, x8, lsl #1]
    add     x10, x10, x11, lsl #2
    br      x10
.Ld_base:
    mov     w0, #30
    b       .Ld_out
.Ld_c1:
    mov     w0, #31
    b       .Ld_out
.Ld_c2:
    mov     w0, #32
.Ld_out:
    ldp     x19, x20, [sp, #16]
    ldp     x29, x30, [sp], #32
    ret
    .size dispatch, . - dispatch

    .globl consume
    .type consume, %function
    .p2align 2
consume:
    mov     x1, #0
    ret
    .size consume, . - consume

    .globl Shape_accessor
    .type Shape_accessor, %function
    .p2align 2
Shape_accessor:
    adrp    x0, shape_metadata
    add     x0, x0, :lo12:shape_metadata
    ret
    .size Shape_accessor, . - Shape_accessor

    .globl swift_getEnumCaseMultiPayload
    .type swift_getEnumCaseMultiPayload, %function
    .p2align 2
swift_getEnumCaseMultiPayload:
    mov     w0, #0
    ret
    .size swift_getEnumCaseMultiPayload, . - swift_getEnumCaseMultiPayload

    .section .rodata
    .p2align 2
tab_shape:
    .hword  (.Ld_base - .Ld_base) / 4
    .hword  (.Ld_c1 - .Ld_base) / 4
    .hword  (.Ld_c2 - .Ld_base) / 4
extra:
    .hword  0x0400, 0x0800, 0x0c00, 0x1000

    .p2align 2
shape_module:
    .word   0
    .word   0
    .word   module_name - .
shape_descriptor:
    .word   0x52
    .word   shape_module - .
    .word   shape_name - .
    .word   Shape_accessor - .
    .word   0
    .word   2
    .word   1
module_name:
    .asciz  "Demo"
shape_name:
    .asciz  "Shape"

    .data
    .p2align 3
    .quad   0
shape_metadata:
    .quad   0x201
    .quad   shape_descriptor

    .section swift5_type_metadata, "a"
    .p2align 2
    .word   shape_descriptor - .

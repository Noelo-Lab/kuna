
.section __TEXT,__text,regular,pure_instructions
.globl _main
_main:
    ret

.globl _holder_viewDidLoad
_holder_viewDidLoad:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
    stp x20, x19, [sp, #-16]!
    mov x19, x0
    bl _holder_type_accessor
    stp x19, x0, [sp, #-16]!
    bl _objc_retain_x19
    mov x20, x0
    add sp, sp, #16
    mov x0, x20
    bl _holder_vtable7
    ldp x20, x19, [sp], #16
    ldp x29, x30, [sp], #16
    ret

.globl _holder_type_accessor
_holder_type_accessor:
    adrp x0, _holder_class@PAGE
    add x0, x0, _holder_class@PAGEOFF
    mov x1, #0
    ret

.globl _holder_vtable7
_holder_vtable7:
    ldr x0, [x0, #8]
    ret

.section __DATA,__data
.globl _holder_class
_holder_class:
    .quad 0
    .quad 0

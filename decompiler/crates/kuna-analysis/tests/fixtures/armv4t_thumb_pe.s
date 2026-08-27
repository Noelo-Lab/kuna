        .syntax unified
        .thumb
        .text
        .globl entry
        .def entry
        .scl 2
        .type 32
        .endef
        .thumb_func
entry:
        movs r0, #7
        bx lr

.syntax unified
.file "aliases.s"
.thumb
.text
.globl answer
.type answer,%function
.thumb_func
answer:
    movs r0,#7
    bx lr
.size answer,.-answer
.globl caller
.type caller,%function
.thumb_func
caller:
    push {lr}
    bl answer(PLT)
    pop {pc}
.size caller,.-caller
.section .note.GNU-stack,"",%progbits

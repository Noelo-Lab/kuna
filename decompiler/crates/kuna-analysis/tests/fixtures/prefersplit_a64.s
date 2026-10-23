.text
.p2align 2
.global ccw_entry
.type ccw_entry, %function
ccw_entry:
    ldr q0, [x0]
    mov x0, x1
    b .Lhelper
.size ccw_entry, .-ccw_entry

.global ccw_helper
.type ccw_helper, %function
ccw_helper:
.Lhelper:
    stp x29, x30, [sp, #-32]!
    mov x29, sp
    str d8, [sp, #16]
    mov v8.16b, v0.16b
    fcmp d8, #0.0
    b.pl .Lpositive
    fneg d8, d8
.Lpositive:
    mov v0.16b, v8.16b
    bl .Lformatter
    ldr d8, [sp, #16]
    ldp x29, x30, [sp], #32
    ret
.size ccw_helper, .-ccw_helper

.global ccw_formatter
.type ccw_formatter, %function
ccw_formatter:
.Lformatter:
    fmov x8, d0
    lsr x9, x8, #52
    and x9, x9, #0x7ff
    and x10, x8, #0x000fffffffffffff
    eor x9, x9, x10, lsr #1
    eor x9, x9, x8
    str x9, [x0]
    ret
.size ccw_formatter, .-ccw_formatter

.global ccw_vector
.type ccw_vector, %function
ccw_vector:
    str q0, [x0]
    ret
.size ccw_vector, .-ccw_vector

.global ccw_tagged
.type ccw_tagged, %function
ccw_tagged:
    fmov x8, d0
    lsr x9, x8, #52
    and x9, x9, #0x7ff
    and x8, x8, #0x000fffffffffffff
    ldr x0, [x8, x9, lsl #3]
    ret
.size ccw_tagged, .-ccw_tagged

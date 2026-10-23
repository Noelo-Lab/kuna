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

.global ccw_mixed
.type ccw_mixed, %function
ccw_mixed:
    ldrh w8, [x0, #22]
    ldr d0, [x0]
    fcmp d0, #0.0
    cset w9, gt
    and w0, w9, w8, lsr #1
    ret
.size ccw_mixed, .-ccw_mixed

.global ccw_array
.type ccw_array, %function
ccw_array:
    ldr d0, [x0, x1, lsl #3]
    ret
.size ccw_array, .-ccw_array

.global ccw_phi
.type ccw_phi, %function
ccw_phi:
    ldr d0, [x0]
    cbz w1, .Lphi_y
    b .Lphi_call
.Lphi_y:
    ldr d0, [x0, #8]
.Lphi_call:
    mov x1, x2
    b .Lhelper
.size ccw_phi, .-ccw_phi

.global ccw_ints
.type ccw_ints, %function
ccw_ints:
    ldr q0, [x0]
    add x0, x1, #1
    b .Lintshelper
.size ccw_ints, .-ccw_ints

.global ccw_f32_entry
.type ccw_f32_entry, %function
ccw_f32_entry:
    ldr s0, [x0]
    b .Lf32helper
.size ccw_f32_entry, .-ccw_f32_entry

.global ccw_f32_helper
.type ccw_f32_helper, %function
ccw_f32_helper:
.Lf32helper:
    fneg s0, s0
    ret
.size ccw_f32_helper, .-ccw_f32_helper

.global ccw_intshelper
.type ccw_intshelper, %function
ccw_intshelper:
.Lintshelper:
    ldr w0, [x0]
    ret
.size ccw_intshelper, .-ccw_intshelper

.global ccw_cycle_entry
.type ccw_cycle_entry, %function
ccw_cycle_entry:
    stp x29, x30, [sp, #-16]!
    ldr d0, [x0]
    bl .Lcycle_a
    fneg d0, d0
    bl .Lcycle_b
    ldp x29, x30, [sp], #16
    ret
.size ccw_cycle_entry, .-ccw_cycle_entry

.global ccw_cycle_a
.type ccw_cycle_a, %function
ccw_cycle_a:
.Lcycle_a:
    fneg d0, d0
    b .Lcycle_b
.size ccw_cycle_a, .-ccw_cycle_a

.global ccw_cycle_b
.type ccw_cycle_b, %function
ccw_cycle_b:
.Lcycle_b:
    fneg d0, d0
    b .Lcycle_a
.size ccw_cycle_b, .-ccw_cycle_b

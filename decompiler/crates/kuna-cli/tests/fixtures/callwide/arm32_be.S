.syntax unified
.arch armv5te
.arm
.section .text.callwide,"ax",%progbits
.balign 4

.macro CW_BEGIN name
    .global \name
    .hidden \name
    .type \name,%function
\name:
.endm

.macro CW_END name
    .size \name,.-\name
.endm

.macro CW_FRAME
    sub sp,sp,#16
    str lr,[sp,#12]
.endm

.macro CW_RETURN
    ldr lr,[sp,#12]
    add sp,sp,#16
    bx lr
.endm

.global produce_sum
#ifndef CALLWIDE_EXPORT
.hidden produce_sum
#endif
.type produce_sum,%function
produce_sum:
#ifdef CALLWIDE_STALE
    adds r12,r1,r3
#else
    adds r1,r1,r3
#endif
    adc r0,r0,r2
    bx lr
CW_END produce_sum

CW_BEGIN caller64
    CW_FRAME
    bl produce_sum
#if defined(CALLWIDE_SWAP)
    mov r12,r0
    mov r0,r1
    mov r1,r12
#elif defined(CALLWIDE_PARTIAL)
    mov r0,r0,lsl #16
    mov r0,r0,lsr #16
#elif defined(CALLWIDE_SPILL)
    str r0,[sp]
    str r1,[sp,#4]
    ldr r0,[sp]
    ldr r1,[sp,#4]
#elif defined(CALLWIDE_UNRELATED)
    mov r0,#1
    mov r1,#2
#elif defined(CALLWIDE_ARITHMETIC)
    add r0,r0,#1
#elif defined(CALLWIDE_INTERVENING_CALL)
    bl consume_two32
#elif defined(CALLWIDE_BRANCH)
    cmp r0,#0
    beq .Lafter_branch
.Lafter_branch:
#elif defined(CALLWIDE_OPAQUE)
    svc #0
#endif
#ifndef CALLWIDE_SINK
#define CALLWIDE_SINK consume64
#endif
    bl CALLWIDE_SINK
    CW_RETURN
CW_END caller64

.macro CW_HIGH name,width
    CW_BEGIN \name
    CW_FRAME
    .if \width == 0
        str r0,[sp]
        str r1,[sp,#4]
    .endif
    bl produce_sum
    .if \width == 16
        mov r0,r0,lsl #16
        mov r0,r0,lsr #16
    .elseif \width == 8
        and r0,r0,#255
    .elseif \width == 1
        clz r0,r0
        mov r0,r0,lsr #5
        eor r0,r0,#1
    .elseif \width == 0
#ifdef CALLWIDE_IGNORED_ZERO
        mov r0,#0
        mov r1,#0
#else
        ldr r0,[sp]
        ldr r1,[sp,#4]
#endif
    .elseif \width == -1
        mov r0,r1
    .endif
    CW_RETURN
    CW_END \name
.endm

CW_HIGH caller_high32,32
CW_HIGH caller_high16,16
CW_HIGH caller_high8,8
CW_HIGH caller_bool,1
CW_HIGH caller_ignored,0

/* CALLWIDE_EXTRA_CALLERS */

#ifdef CALLWIDE_ADDRESS_TABLE
.section .rodata.callwide,"a",%progbits
.balign 4
.hidden producer_address
.type producer_address,%object
producer_address:
    .word produce_sum
.size producer_address,4
#endif

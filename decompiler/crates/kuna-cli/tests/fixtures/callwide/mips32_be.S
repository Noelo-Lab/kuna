.set noreorder
.set noat
.section .text.callwide,"ax",@progbits
.balign 4

.macro CW_BEGIN name
    .globl \name
    .hidden \name
    .type \name,@function
\name:
.endm

.macro CW_END name
    .size \name,.-\name
.endm

.macro CW_FRAME
    addiu $29,$29,-24
    sw $31,20($29)
.endm

.macro CW_RETURN
    lw $31,20($29)
    jr $31
    addiu $29,$29,24
.endm

.globl produce_sum
#ifndef CALLWIDE_EXPORT
.hidden produce_sum
#endif
.type produce_sum,@function
produce_sum:
    addu $8,$6,$4
#ifdef CALLWIDE_STALE
    addu $9,$7,$5
    sltu $2,$9,$7
#else
    addu $3,$7,$5
    sltu $2,$3,$7
#endif
    jr $31
    addu $2,$8,$2
CW_END produce_sum

CW_BEGIN caller64
    CW_FRAME
#ifdef CALLWIDE_JALR
    lui $25,%hi(produce_sum)
    addiu $25,$25,%lo(produce_sum)
    jalr $25
    nop
#else
    jal produce_sum
    nop
#endif
#if defined(CALLWIDE_SWAP)
    addu $4,$3,$0
#elif defined(CALLWIDE_PARTIAL)
    andi $4,$2,65535
#elif defined(CALLWIDE_SPILL)
    sw $2,4($29)
    sw $3,8($29)
    lw $4,4($29)
#elif defined(CALLWIDE_UNRELATED)
    addiu $4,$0,1
#elif defined(CALLWIDE_ARITHMETIC)
    addiu $4,$2,1
#else
    addu $4,$2,$0
#endif
#ifdef CALLWIDE_INTERVENING_CALL
    jal consume_two32
    addu $5,$3,$0
#endif
#ifdef CALLWIDE_BRANCH
    beq $4,$0,.Lafter_branch
    nop
.Lafter_branch:
#endif
#ifdef CALLWIDE_OPAQUE
    syscall
#endif
#ifndef CALLWIDE_SINK
#define CALLWIDE_SINK consume64
#endif
    jal CALLWIDE_SINK
#if defined(CALLWIDE_SWAP)
    addu $5,$2,$0
#elif defined(CALLWIDE_SPILL)
    lw $5,8($29)
#elif defined(CALLWIDE_UNRELATED)
    addiu $5,$0,2
#else
    addu $5,$3,$0
#endif
#ifdef CALLWIDE_ALIAS_RA
    sw $4,20($29)
#endif
#ifdef CALLWIDE_WRONG_RA_SLOT
    lw $31,16($29)
#else
    lw $31,20($29)
#endif
    jr $31
#ifdef CALLWIDE_BAD_SP_DELAY
    addiu $29,$29,20
#else
    addiu $29,$29,24
#endif
CW_END caller64

.macro CW_HIGH name,width
    CW_BEGIN \name
    CW_FRAME
    .if \width == 0
        sw $4,4($29)
        sw $5,8($29)
    .endif
    jal produce_sum
    nop
    .if \width == 16
        andi $2,$2,65535
    .elseif \width == 8
        andi $2,$2,255
    .elseif \width == 1
        sltu $2,$0,$2
    .elseif \width == 0
#ifdef CALLWIDE_IGNORED_ZERO
        addu $2,$0,$0
        addu $3,$0,$0
#else
        lw $2,4($29)
        lw $3,8($29)
#endif
    .elseif \width == -1
        addu $2,$3,$0
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

#ifdef CALLWIDE_POINTER_RA
CW_BEGIN pointer_ra
    addu $31,$4,$0
    jr $31
    nop
CW_END pointer_ra
#endif

#ifdef CALLWIDE_CONSTANT_RA
CW_BEGIN constant_ra
    lui $31,0x1234
    ori $31,$31,0x5678
    jr $31
    nop
CW_END constant_ra
#endif

#ifdef CALLWIDE_UNKNOWN_STORE
CW_BEGIN unknown_store
    sw $2,0($4)
    jr $31
    nop
CW_END unknown_store
#endif

#ifdef CALLWIDE_RESERVED_WORD
.Lreserved_gap:
    .word 0x00000005
#endif

#ifdef CALLWIDE_ADDRESS_TABLE
.section .rodata.callwide,"a",@progbits
.balign 4
.hidden producer_address
.type producer_address,@object
producer_address:
    .word produce_sum
.size producer_address,4
#endif

#ifndef CALLWIDE_MISSING_ROOT
.section .text.callwide.root,"ax",@progbits
.balign 4
CW_BEGIN callwide_entry
    jal caller64
    nop
    j callwide_entry+8
    nop
CW_END callwide_entry
#endif

#ifdef CALLWIDE_MIXED
.section .text.callwide.mixed,"ax",@progbits
.balign 4
CW_BEGIN produce_rejected
    addu $8,$6,$4
    addu $3,$7,$5
    sltu $2,$3,$7
    jr $31
    addu $2,$8,$2
CW_END produce_rejected

CW_BEGIN caller_rejected
    CW_FRAME
    jal produce_rejected
    nop
    addu $4,$3,$0
    jal consume64
    addu $5,$2,$0
    CW_RETURN
CW_END caller_rejected
#endif

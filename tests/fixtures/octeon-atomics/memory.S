/* Original regression functions. No vendor code or binary input. */
.set noreorder
.set noat
.text
.macro function name
    .align 5
    .globl \name
    .type \name,@function
\name:
.endm
.macro endfunction name
    .size \name,.-\name
.endm

function atomic_global
    ori $4,$0,0x2000
    daddiu $5,$0,7
    sd $5,0($4)
    laad $0,($4),$5
    ld $2,0($4)
    jr $31
    nop
endfunction atomic_global

function atomic_local
    daddiu $29,$29,-32
    sd $5,16($29)
    daddiu $4,$29,16
    laad $2,($4),$6
    daddiu $29,$29,32
    jr $31
    nop
endfunction atomic_local

function conditional
    ll $8,0($4)
    sc $5,0($4)
    daddu $2,$5,$0
    jr $31
    nop
endfunction conditional

function barriers
    sd $5,0($4)
    syncw
    syncw
    ld $2,0($4)
    jr $31
    nop
endfunction barriers

function wide_integer
    laad $8,($4),$5
    daddu $2,$4,$0
    jr $31
    nop
endfunction wide_integer

function partial_global
    ori $4,$0,0x2004
    sd $0,-4($4)
    daddiu $5,$0,1
    laa $0,($4),$5
    ld $2,-4($4)
    jr $31
    nop
endfunction partial_global

function pointer_snapshot
    ld $2,0($4)
    saad $5,($4)
    jr $31
    nop
endfunction pointer_snapshot

function conditional_snapshot
    ld $2,0($4)
    beq $6,$0,1f
    nop
    saad $5,($4)
1:  jr $31
    daddu $2,$2,$6
endfunction conditional_snapshot

function ordered_snapshot
    ld $2,0($4)
    syncw
    jr $31
    nop
endfunction ordered_snapshot

function zero_neighbor
    daddiu $29,$29,-32
    sd $5,8($29)
    daddiu $4,$29,16
    zcb ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,32
endfunction zero_neighbor

function zero_neighbor_transient
    daddiu $29,$29,-32
    sd $5,24($29)
    daddiu $4,$29,16
    zcbt ($4)
    ld $2,24($29)
    jr $31
    daddiu $29,$29,32
endfunction zero_neighbor_transient

function zero_elsewhere
    daddiu $29,$29,-32
    sd $5,8($29)
    zcb ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,32
endfunction zero_elsewhere

function zero_deep
    daddiu $29,$29,-256
    sd $5,8($29)
    zcb ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,256
endfunction zero_deep

    .align 5
persist_first:
    ori $9,$0,0x2100
    lw $3,0($9)
    addiu $3,$3,1
    jr $31
    sw $3,0($9)

    .align 5
persist_second:
    ori $9,$0,0x2108
    lw $3,0($9)
    addiu $3,$3,1
    jr $31
    sw $3,0($9)

function persist_atomic
    daddiu $29,$29,-32
    sd $31,24($29)
    sd $16,16($29)
    sd $17,8($29)
    move $17,$5
    andi $8,$4,1
    beqz $8,1f
    nop
    bal persist_first
    nop
    b 2f
    nop
1:  bal persist_second
    nop
2:  ori $9,$0,0x2100
    lw $16,0($9)
    lw $3,4($9)
    slti $3,$3,18
    bnez $3,3f
    nop
    bal persist_second
    nop
    saad $16,($17)
    bal persist_first
    nop
    b 4f
    nop
3:  bal persist_first
    nop
    addiu $4,$16,8
    ori $9,$0,0x2108
    sw $4,0($9)
    bal persist_first
    move $4,$16
4:  ld $31,24($29)
    ld $16,16($29)
    ld $17,8($29)
    jr $31
    daddiu $29,$29,32
endfunction persist_atomic

function zero_incoming_negative
    daddiu $29,$29,-256
    sd $5,8($29)
    daddiu $4,$4,-256
    zcb ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,256
endfunction zero_incoming_negative

function zero_incoming_negative_transient
    daddiu $29,$29,-256
    sd $5,8($29)
    daddiu $4,$4,-256
    zcbt ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,256
endfunction zero_incoming_negative_transient

function zero_incoming_variable
    daddiu $29,$29,-256
    sd $5,8($29)
    daddu $4,$4,$6
    zcb ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,256
endfunction zero_incoming_variable

function zero_incoming_masked
    daddiu $29,$29,-256
    sd $5,8($29)
    and $4,$4,$6
    zcb ($4)
    ld $2,8($29)
    jr $31
    daddiu $29,$29,256
endfunction zero_incoming_masked

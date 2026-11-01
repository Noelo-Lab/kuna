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

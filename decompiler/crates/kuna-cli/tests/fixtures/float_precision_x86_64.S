.text
.globl widened_update
.type widened_update,@function
widened_update:
    cvtss2sd %xmm0, %xmm0
    cvtss2sd %xmm1, %xmm1
    cvtss2sd %xmm2, %xmm2
    subsd %xmm2, %xmm1
    addsd %xmm1, %xmm0
    cvtsd2ss %xmm0, %xmm0
    ret
.size widened_update, .-widened_update

.globl packed_update
.type packed_update,@function
packed_update:
    movss (%rsi), %xmm0
    movss 4(%rsi), %xmm3
    unpcklps %xmm3, %xmm0
    cvtps2pd %xmm0, %xmm0
    movss (%rdx), %xmm1
    movss 4(%rdx), %xmm3
    unpcklps %xmm3, %xmm1
    cvtps2pd %xmm1, %xmm1
    movss (%rcx), %xmm2
    movss 4(%rcx), %xmm3
    unpcklps %xmm3, %xmm2
    cvtps2pd %xmm2, %xmm2
    subpd %xmm2, %xmm1
    addpd %xmm1, %xmm0
    cvtpd2ps %xmm0, %xmm0
    movss %xmm0, (%rdi)
    shufps $0x55, %xmm0, %xmm0
    movss %xmm0, 4(%rdi)
    ret
.size packed_update, .-packed_update

.globl float_update
.type float_update,@function
float_update:
    subss %xmm2, %xmm1
    addss %xmm1, %xmm0
    ret
.size float_update, .-float_update

.globl widen_roundtrip
.type widen_roundtrip,@function
widen_roundtrip:
    cvtss2sd %xmm0, %xmm0
    cvtsd2ss %xmm0, %xmm0
    ret
.size widen_roundtrip, .-widen_roundtrip

.globl widened_product
.type widened_product,@function
widened_product:
    cvtss2sd %xmm0, %xmm0
    cvtss2sd %xmm1, %xmm1
    cvtss2sd %xmm2, %xmm2
    mulsd %xmm2, %xmm1
    mulsd %xmm1, %xmm0
    cvtsd2ss %xmm0, %xmm0
    ret
.size widened_product, .-widened_product

.globl widened_quotient
.type widened_quotient,@function
widened_quotient:
    cvtss2sd %xmm0, %xmm0
    cvtss2sd %xmm1, %xmm1
    cvtss2sd %xmm2, %xmm2
    divsd %xmm2, %xmm1
    divsd %xmm1, %xmm0
    cvtsd2ss %xmm0, %xmm0
    ret
.size widened_quotient, .-widened_quotient
.section .note.GNU-stack,"",@progbits

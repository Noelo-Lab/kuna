.text
.globl scalar_min
.type scalar_min,@function
scalar_min:
    minss %xmm1, %xmm0
    ret
.size scalar_min, .-scalar_min
.globl ratio_clamp
.type ratio_clamp,@function
ratio_clamp:
    subss %xmm1, %xmm0
    subss %xmm1, %xmm2
    divss %xmm2, %xmm0
    movss clamp_one(%rip), %xmm1
    minss %xmm0, %xmm1
    xorps %xmm2, %xmm2
    ucomiss %xmm1, %xmm2
    ja .Lzero
    movaps %xmm1, %xmm0
    ret
.Lzero:
    movaps %xmm2, %xmm0
    ret
.size ratio_clamp, .-ratio_clamp
.globl ordered_above
.type ordered_above,@function
ordered_above:
    ucomiss %xmm1, %xmm0
    seta %al
    movzbl %al, %eax
    ret
.size ordered_above, .-ordered_above
.section .rodata
.p2align 2
.globl clamp_one
.type clamp_one,@object
clamp_one: .long 0x3f800000
.size clamp_one, 4

.section .note.GNU-stack,"",@progbits

    .set noreorder
    .text
    .globl disp
    .type disp,@function
    .ent disp
disp:
    beqz $a0, 1f
    nop
    jr $a0
    nop
2:  li $v0, 11
    jr $ra
    nop
1:  li $v0, 77
    jr $ra
    nop
    .end disp
    .size disp, .-disp

    .set micromips
.Lm:
    li $v0, 55
    jrc $ra
    .set nomicromips
    .globl m
    .type m,@function
    .set m, .Lm + 1

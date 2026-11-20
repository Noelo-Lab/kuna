.set noreorder
.text

.macro caller name, form, target
.globl \name
.type \name, @function
\name:
    daddiu $sp,$sp,-16
    sd $ra,8($sp)
    li $a0,7
    li $a1,11
.if \form == 0
    jal \target
.elseif \form == 1
    .reloc ., R_MIPS_64, \target
    .reloc ., R_MIPS_26, 0
    .word 0x0c000000
.elseif \form == 2
    .reloc ., R_MIPS_32, \target
    .reloc ., R_MIPS_64, 0
    .reloc ., R_MIPS_26, 0
    .word 0x0c000000
.elseif \form == 3
    .reloc ., R_MIPS_64, \target
    .reloc ., R_MIPS_64, 0
    .reloc ., R_MIPS_64, 0
    .reloc ., R_MIPS_26, 0
    .word 0x0c000000
.else
    .reloc ., R_MIPS_64, \target-4
    .reloc ., R_MIPS_PC16, 0
    .word 0x04110000
.endif
    nop
    ld $ra,8($sp)
    jr $ra
    daddiu $sp,$sp,16
.size \name, .-\name
.endm

caller direct_call, 0, direct_target
caller composed_call, 1, composed_target
caller scalar_chain_call, 2, scalar_target
caller cross_record_call, 3, cross_target
caller relative_call, 4, relative_target

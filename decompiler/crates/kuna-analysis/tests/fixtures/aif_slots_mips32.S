# LLVM 21.1.8, big-endian MIPS32. Stripped ELF SHA-256:
# 285b286f9f5c0ba19171db9893b2a3b0cf3453812c1c72704e31e4f80d8e5590
# From this directory:
# llvm-mc -triple=mips -mcpu=mips32 -filetype=obj aif_slots_mips32.S -o slots.o
# ld.lld -z max-page-size=4096 -m elf32btsmip --build-id=none -T aif_slots_mips32.ld slots.o -o slots
# llvm-strip -s slots -o aif_slots_mips32
.set noreorder
.text
.globl _start
_start:
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
    jal branch\n
    nop
    jal simple\n
    nop
    jal padded\n
    nop
.endr
    jr $ra
    nop

.macro delayed
    beq $a0,$zero,1f
    addiu $v0,$zero,11
    addiu $v0,$a0,5
1:  jr $ra
    nop
.endm
.macro ordinary
    addiu $v0,$a0,3
    addiu $v0,$v0,2
    jr $ra
    nop
.endm
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
branch\n:
    delayed
simple\n:
    ordinary
padded\n:
    nop
    nop
    ordinary
.endr

.section .candidates,"ax"
.globl delayed_entry,ordinary_entry,after_padding
delayed_entry:
    delayed
ordinary_entry:
    ordinary
    nop
    nop
after_padding:
    # This BNE fingerprint is absent from training. Only the NOP pair
    # can discover it, so it exercises the padding-adjusted width check.
    bne $a0,$zero,2f
    addiu $v0,$zero,11
    addiu $v0,$a0,5
2:  jr $ra
    nop

.abicalls
.set noreorder
.text
.macro POINTER name, byte
.globl \name
.type \name,@function
\name:
.cpsetup $25,$24,\name
.if ABI64
 ld $2,%got_page(saved)($28)
 ld $2,%got_ofst(saved+\byte)($2)
.else
 lw $2,%got_page(saved)($28)
 lw $2,%got_ofst(saved+\byte)($2)
.endif
.cpreturn
 jr $31
 nop
.size \name,.-\name
.endm
POINTER address_value,0
.if ABI64
POINTER address_offset,8
POINTER address_negative,16
.else
POINTER address_offset,4
POINTER address_negative,8
.endif
.globl unrelated_literal
.type unrelated_literal,@function
unrelated_literal:
 lui $2,0x40
 ori $2,$2,0x2000
 jr $31
 nop
.size unrelated_literal,.-unrelated_literal
.weak optional
.type optional,@function
.section .rodata
.balign 8
saved:
.if ABI64
 .dword optional,optional+0x8000,optional-0x8000
.else
 .word optional,optional+0x8000,optional-0x8000
.endif

.abicalls
.set noreorder
.text
.globl same
.type same,@function
same:
.cpsetup $25,$24,same
.if ABI64
 ld $2,%got_disp(optional)($28)
.else
 lw $2,%got_disp(optional)($28)
.endif
.if MIXED
.if ABI64
 ld $3,%got_page(saved)($28)
 ld $3,%got_ofst(saved)($3)
.else
 lw $3,%got_page(saved)($28)
 lw $3,%got_ofst(saved)($3)
.endif
 xor $2,$2,$3
 sltiu $2,$2,1
.else
 sltiu $2,$2,1
.endif
.cpreturn
 jr $31
 nop
.size same,.-same
.weak optional
.type optional,@function
.if MIXED
.section .rodata
.align 3
saved:
.if ABI64
 .dword optional
.else
 .word optional
.endif
.endif

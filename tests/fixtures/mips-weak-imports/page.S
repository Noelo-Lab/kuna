.abicalls
.set noreorder
.text
.globl call_optional
.type call_optional,@function
call_optional:
.cpsetup $25,$24,call_optional
.if ABI64
 ld $2,%got_page(optional)($28)
 daddiu $2,$2,%got_ofst(optional)
.else
 lw $2,%got_page(optional)($28)
 addiu $2,$2,%got_ofst(optional)
.endif
 beqz $2,1f
 xori $3,$4,0x42
 or $25,$2,$0
.cpreturn
 jr $25
 nop
1:
.cpreturn
 jr $31
 or $2,$3,$0
.size call_optional,.-call_optional
.weak optional
.type optional,@function

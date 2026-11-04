/* Original GOT fields; all address constants and instructions are synthetic. */
.set noreorder
.abicalls
.macro pointer reg, expr
.if ABI64
ld \reg, \expr($28)
.else
lw \reg, \expr($28)
.endif
.endm
.macro plus reg, expr
.if ABI64
daddiu \reg, \reg, \expr
.else
addiu \reg, \reg, \expr
.endif
.endm
.text
.globl _start
.type _start, @function
_start:
.if ABI32
.cpload $25
.else
.cpsetup $25, $2, _start
.endif
pointer $4, %got_disp(global_value)
pointer $5, %got_page(local_value)
plus $5, %got_ofst(local_value)
pointer $6, %got(local_value)
plus $6, %lo(local_value)
pointer $7, %got(global_value)
pointer $25, %call16(function)
.reloc ., R_MIPS_JALR, function
jalr $25
nop
lui $8, %got_hi(global_value)
daddu $8, $8, $28
.if ABI64
ld $9, %got_lo(global_value)($8)
.else
lw $9, %got_lo(global_value)($8)
.endif
lui $10, %call_hi(function)
daddu $10, $10, $28
.if ABI64
ld $25, %call_lo(function)($10)
.else
lw $25, %call_lo(function)($10)
.endif
pointer $11, %got_page(local_value+0x7ffc)
plus $11, %got_ofst(local_value+0x7ffc)
pointer $12, %got_disp(optional)
.size _start, .-_start
.globl global_value, function
.weak optional
.set global_value, 0x12348000
.set function, 0x500000
.data
.align 3
local_value:
.word 0x12348004

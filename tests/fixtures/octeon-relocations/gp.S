/* Original NewABI PIC prologue and explicit gp-relative fields. */
.set noreorder
.abicalls
.text
.globl _start
.type _start, @function
_start:
.if ABI32
.cpload $25
.else
.cpsetup $25, $2, _start
.endif
.reloc ., R_MIPS_GPREL16, near
.word 0x67830000
.reloc ., R_MIPS_LITERAL, near
.word 0x8f840000
.reloc ., R_MIPS_LITERAL, local_target
.word 0x8f850000
.if !ABI32
.reloc ., R_MIPS_LITERAL, near
.reloc ., R_MIPS_SUB, 0
.reloc ., R_MIPS_LO16, 0
.word 0x67860000
.endif
.reloc ., R_MIPS_GPREL32, local_target
.word 0xa5a5a5a5
.if ABI32
.reloc ., R_MIPS_32, __gnu_local_gp
.word 0
.else
.reloc ., R_MIPS_64, __gnu_local_gp
.dword 0xa5a5a5a5a5a5a5a5
.endif
.size _start, .-_start
local_target:
.word 0x12345678
.globl _gp, near
.set _gp, 0x408000
.set near, 0x407ff0

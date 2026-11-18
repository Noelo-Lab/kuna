/* Original PC32 fields; GNU ld supplies the expected wrapping results. */
.set noreorder
.text
.globl composed_value, standalone_value
.type composed_value, @object
composed_value:
.reloc ., R_MIPS_32, -4
.reloc ., R_MIPS_PC32, -4
.word 0
.size composed_value, .-composed_value
.type standalone_value, @object
standalone_value:
.reloc ., R_MIPS_PC32, lower
.word 0xa5a5a5a5
.size standalone_value, .-standalone_value
.if ABI64
.reloc ., R_MIPS_PC32, -4
.reloc ., R_MIPS_64, 0
.dword 0xa5a5a5a5a5a5a5a5
.reloc ., R_MIPS_PC32, upper
.word 0xa5a5a5a5
.endif
.balign 16
.globl read_composed_pc32, read_standalone_pc32
.type read_composed_pc32, @function
read_composed_pc32:
lui $8, %hi(composed_value)
lw $2, %lo(composed_value)($8)
jr $31
nop
.size read_composed_pc32, .-read_composed_pc32
.type read_standalone_pc32, @function
read_standalone_pc32:
lui $8, %hi(standalone_value)
lw $2, %lo(standalone_value)($8)
jr $31
nop
.size read_standalone_pc32, .-read_standalone_pc32
.globl lower
.set lower, -0x80000000
.if ABI64
.globl upper
.set upper, 0x123456789abcdef0
.endif

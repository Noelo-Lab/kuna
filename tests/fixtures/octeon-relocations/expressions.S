/* Original relocation fields; GNU ld supplies the expected relocated bytes. */
.set noreorder
.text
.globl _start
.type _start, @function
_start:
.reloc ., R_MIPS_HI16, value
.word 0x3c080000
.reloc ., R_MIPS_LO16, value
.word 0x65080000
.reloc ., R_MIPS_HI16, value-1
.word 0x3c090000
.reloc ., R_MIPS_LO16, value-1
.word 0x65290000
.reloc ., R_MIPS_26, dest
.word 0x0c000000
.reloc ., R_MIPS_PC16, dest-4
.word 0x10000000
.reloc ., R_MIPS_PC16, back-4
.word 0x11000000
.reloc ., R_MIPS_16, small
.word 0x24020000
.reloc ., R_MIPS_32, value+7
.word 0xa5a5a5a5
.reloc ., R_MIPS_REL32, value+9
.word 0xa5a5a5a5
.reloc ., R_MIPS_64, value+11
.dword 0xa5a5a5a5a5a5a5a5
.reloc ., R_MIPS_SUB, value
.dword 0xa5a5a5a5a5a5a5a5
.reloc ., R_MIPS_32, value
.reloc ., R_MIPS_SUB, 0
.reloc ., R_MIPS_HI16, 0
.word 0x3c0a0000
.reloc ., R_MIPS_32, value
.reloc ., R_MIPS_SUB, 0
.reloc ., R_MIPS_LO16, 0
.word 0x654a0000
.reloc ., R_MIPS_16, value
.reloc ., R_MIPS_LO16, 0
.word 0x654a0000
.reloc ., R_MIPS_32, value
.reloc ., R_MIPS_SUB, 0
.reloc ., R_MIPS_SUB, 0
.reloc ., R_MIPS_HI16, 0
.word 0x3c0a0000
.reloc ., R_MIPS_HI16, 0x12348000
.word 0x3c0a0000
.reloc ., R_MIPS_PC32, dest+12
.word 0xa5a5a5a5
.reloc ., R_MIPS_64, negative
.dword 0xa5a5a5a5a5a5a5a5
.if ABI64
.reloc ., R_MIPS_HIGHEST, wide
.word 0x3c0b0000
.reloc ., R_MIPS_HIGHER, wide
.word 0x656b0000
.word 0x000b5c38
.reloc ., R_MIPS_HI16, wide
.word 0x656b0000
.word 0x000b5c38
.reloc ., R_MIPS_LO16, wide
.word 0x656b0000
.reloc ., R_MIPS_64, wide
.dword 0xa5a5a5a5a5a5a5a5
/* Compose narrow operations with a 64-bit final write. */
.reloc ., R_MIPS_32, wide
.reloc ., R_MIPS_64, 0
.dword 0xa5a5a5a5a5a5a5a5
.reloc ., R_MIPS_REL32, wide
.reloc ., R_MIPS_64, 0
.dword 0xa5a5a5a5a5a5a5a5
.reloc ., R_MIPS_PC32, wide
.reloc ., R_MIPS_64, 0
.dword 0xa5a5a5a5a5a5a5a5
.reloc ., R_MIPS_HIGHEST, wrap
.word 0x3c0b0000
.reloc ., R_MIPS_HIGHER, wrap
.word 0x656b0000
.endif
.size _start, .-_start
.globl value, dest, back, small, negative
.set value, 0x12348000
.set dest, 0x400100
.set back, 0x3ffff0
.set small, -17
.set negative, -0x789a8000
.if ABI64
.globl wide, wrap
.set wide, 0x12347fff80008000
.set wrap, 0xffffffffffff8000
.endif

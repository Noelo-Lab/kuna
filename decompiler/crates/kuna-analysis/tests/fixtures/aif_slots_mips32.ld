ENTRY(_start)
SECTIONS {
    .MIPS.abiflags 0x4f0000 : { *(.MIPS.abiflags) }
    .reginfo : { *(.reginfo) }
    .text 0x501000 : { *(.text) }
    .candidates 0x502000 : { *(.candidates) }
    .data ALIGN(0x1000) : { *(.data) *(.got) }
}

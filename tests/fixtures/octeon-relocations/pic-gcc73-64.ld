ENTRY(read_global)
SECTIONS
{
  .text 0x400000 : { *(.text) }
  .data 0x400130 : { *(.data) }
  .MIPS.options 0x400140 : { *pic-gcc73-64.o(.MIPS.options) }
  .MIPS.abiflags 0x400168 : { *(.MIPS.abiflags) }
  .rodata 0x400180 : { *(.rodata) }
  _gp = 0x408180;
  .got 0x400190 : { *(.got) }
  .external_text 0x402000 : { *(.external_text) }
  /DISCARD/ : { *external-gcc73-64.o(.MIPS.options) }
}

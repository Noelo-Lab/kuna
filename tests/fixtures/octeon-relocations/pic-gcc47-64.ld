ENTRY(read_global)
SECTIONS
{
  .text 0x400000 : { *(.text) }
  .data 0x400130 : { *(.data) }
  .MIPS.options 0x400140 : { *pic-gcc47-64.o(.MIPS.options) }
  .rodata 0x400170 : { *(.rodata) }
  _gp = 0x408170;
  .got 0x400180 : { *(.got) }
  .external_text 0x402000 : { *(.external_text) }
  /DISCARD/ : { *external-gcc47-64.o(.MIPS.options) }
}

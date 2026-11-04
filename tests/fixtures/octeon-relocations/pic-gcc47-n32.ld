ENTRY(read_global)
SECTIONS
{
  .text 0x400000 : { *(.text) }
  .data 0x400130 : { *(.data) }
  .reginfo 0x400140 : { *(.reginfo) }
  .rodata 0x400160 : { *(.rodata) }
  _gp = 0x408160;
  .got 0x400170 : { *(.got) }
  .external_text 0x402000 : { *(.external_text) }
  /DISCARD/ : { *external-gcc47-n32.o(.MIPS.options) }
}

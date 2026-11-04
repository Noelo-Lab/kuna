ENTRY(_start)
SECTIONS
{
  . = 0x400000;
  .text : { *(.text) }
  .reginfo : { *(.reginfo) }
  .MIPS.options : { *(.MIPS.options) }
  .MIPS.abiflags : { *(.MIPS.abiflags) }
  .data : { *(.data) }
  .bss : { *(.bss) }
  _gp = 0x408000;
}

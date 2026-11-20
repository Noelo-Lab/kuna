ENTRY(_start)
SECTIONS
{
  . = 0x400000;
  .text : { *(.text) }
  .data : { *(.data) }
  .bss : { *(.bss) }
  .reginfo : { *(.reginfo) }
  .MIPS.options : { *(.MIPS.options) }
  .MIPS.abiflags : { *(.MIPS.abiflags) }
  . = ALIGN(16);
  _gp = . + 0x7ff0;
  .got : { *(.got) }
}

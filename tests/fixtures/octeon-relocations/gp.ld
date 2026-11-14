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
  . = ALIGN(16);
  _gp = . + 0x7ff0;
  .got : { *(.got) }
  .sdata : { *(.sdata) }
}

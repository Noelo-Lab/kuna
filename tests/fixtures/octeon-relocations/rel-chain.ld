ENTRY(read_combined)
SECTIONS
{
  . = 0x400000;
  .text : { *(.text) }
  .rodata : { *(.rodata) }
  .reginfo : { *(.reginfo) }
  .MIPS.abiflags : { *(.MIPS.abiflags) }
  .data : { *(.data) }
  .bss : { *(.bss) }
  . = ALIGN(16);
  _gp = . + 0x7ff0;
  .got : { *(.got) }
}

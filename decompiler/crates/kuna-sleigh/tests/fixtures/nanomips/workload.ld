SECTIONS
{
  . = 0x80001000;
  .table : { KEEP(*(.table)) }
  .text : { *(.text .text.*) }
  .rodata : { *(.rodata .rodata.*) }
  .data : { *(.data .data.*) *(.sdata .sdata.*) }
  .bss : { *(.sbss .sbss.*) *(.bss .bss.*) *(COMMON) }
  _gp = 0x80008000;
  /DISCARD/ : { *(.nanoMIPS.abiflags) *(.eh_frame) *(.comment) *(.note*) }
}

ENTRY(_start)
SECTIONS {
  . = 0x10000;
  .text : { *(.text*) *(.rodata*) }
  .got : { *(.got*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) }
}

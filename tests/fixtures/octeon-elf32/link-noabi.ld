ENTRY(crc16)
SECTIONS
{
  . = 0x10000000;
  .text : { *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) }
  .mdebug.abiO64 0 : { KEEP(*(.mdebug.abiO64)) mdebug_abi = .; }
  .mdebug.abi32 0 : { KEEP(*(.mdebug.abi32)) mdebug_abi32 = .; }
  /DISCARD/ : { *(.MIPS.abiflags) *(.gnu.attributes) *(.mdebug.*) }
}

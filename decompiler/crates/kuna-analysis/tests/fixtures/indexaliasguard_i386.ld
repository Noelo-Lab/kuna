/* Linker script for indexaliasguard_i386.s: pins .text and .rodata at the
   VMAs the vendored MSVC body hard-codes (0x401330 and 0x4060bc). */
ENTRY(transform)
SECTIONS
{
  . = 0x401330;
  .text : { *(.text) }
  . = 0x4060bc;
  .rodata : { *(.rodata) }
}

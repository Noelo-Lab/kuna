SECTIONS
{
    .text : { *(.text.start) *(.text*) }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) *(.sdata*) }
    .bss : { *(.sbss*) *(.bss*) *(COMMON) }
    /DISCARD/ : { *(.MIPS.abiflags) *(.reginfo) *(.pdr) *(.comment) *(.note*) *(.eh_frame*) *(.MIPS.options) }
}

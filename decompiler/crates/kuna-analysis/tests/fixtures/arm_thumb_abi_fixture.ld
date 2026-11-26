/* Match Kuna's nonempty SHF_ALLOC section order, alignment, and load base. */
ENTRY(thumb_abi_values)
SECTIONS
{
    . = 0x400000;
    .text : { *(.text) }
    .rodata : { *(.rodata) }
    .text.tail : { *(.text.tail) }
}

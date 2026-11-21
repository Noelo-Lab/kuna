ENTRY(_start)
PHDRS { code PT_LOAD FLAGS(5); }
SECTIONS {
    . = 0x401000;
    .text : { *(.text) } :code
    . = 0x402006;
    .padding : { *(.padding) } :code
    . = 0x403006;
    .short_slot : { *(.short_slot) } :code
    .short_tail : { *(.short_tail) } :code
    . = 0x404006;
    .full_slot : { *(.full_slot) } :code
    .full_tail : { *(.full_tail) } :code
    . = 0x405006;
    .shift_short : { *(.shift_short) } :code
    .shift_tail : { *(.shift_tail) } :code
}

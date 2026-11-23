ENTRY(_start)
PHDRS {
    boot PT_LOAD FLAGS(5);
    code PT_LOAD FLAGS(5);
}
SECTIONS {
    . = 0;
    .boot : { *(.boot) } :boot
    . = 0x401000;
    .text : { *(.text) } :code
    . = 0x402006;
    .extra : { *(.extra) } :code
    .more : { *(.more) } :code
    .fake : { *(.fake) } :code
}

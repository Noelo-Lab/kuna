PHDRS { text PT_LOAD FLAGS(5); data PT_LOAD FLAGS(6); }
ENTRY(_start)
SECTIONS {
  . = 0x400000 + SIZEOF_HEADERS;
  .text : { *(.text) } :text
  .rodata : { *(.rodata) } :text
  . = ALIGN(0x1000);
  .data : { *(.data) } :data
}

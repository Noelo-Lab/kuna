/* GH #1401: .noaccess gets its own PT_LOAD with no permissions; .bss runs the RW
   segment past its file content. */
PHDRS {
  text PT_LOAD FILEHDR PHDRS FLAGS(5);
  sealed PT_LOAD FLAGS(0);
  data PT_LOAD FLAGS(6);
}
SECTIONS {
  . = 0x400000 + SIZEOF_HEADERS;
  .text : { *(.text) } :text
  . = ALIGN(0x1000);
  .noaccess : { *(.noaccess) } :sealed
  . = ALIGN(0x1000);
  .data : { *(.data) } :data
  .bss : { *(.bss) } :data
  /DISCARD/ : { *(.note*) *(.comment) }
}

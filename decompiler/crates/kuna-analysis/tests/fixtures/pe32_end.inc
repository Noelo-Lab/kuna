; Closes the sections opened by pe32.inc and pads them to FileAlignment.

section .text
text_end:
text_vsize equ text_end - text_start
    times (0x200 - (text_vsize % 0x200)) % 0x200 db 0xcc
text_rawsize equ $ - text_start

section .rdata
rdata_end:
rdata_vsize equ rdata_end - rdata_start
    times (0x200 - (rdata_vsize % 0x200)) % 0x200 db 0
rdata_rawsize equ $ - rdata_start
image_end equ rdata_start + ((rdata_vsize + 0xfff) & ~0xfff)

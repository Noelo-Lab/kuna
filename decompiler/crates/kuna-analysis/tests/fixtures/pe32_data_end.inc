; Closes the sections opened by pe32_data.inc and pads them to FileAlignment.

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

section .data
    dd 0
data_end:
data_vsize equ data_end - data_start
    times (0x200 - (($ - data_start) % 0x200)) % 0x200 db 0
data_rawsize equ $ - data_start
image_end equ data_start + ((data_rawsize + 0xfff) & ~0xfff)

# GNU binutils 2.47.20260726. Stripped ELF SHA-256:
# 60f42216be41ed6cd008afa19641de927f4df16c051eb8032680e2c738ea4bf8
# Build from this directory:
# as --64 aif_slot_edges_x86_64.S -o edges.o
# ld --build-id=none -T aif_slot_edges_x86_64.ld edges.o -o edges
# strip -s edges -o aif_slot_edges_x86_64
.macro mov40
.byte 0x2e,0xb8,40,0,0,0
.endm
.macro add2
.byte 0x81,0xc0,2,0,0,0
.endm
.macro ret6
.byte 0x2e,0x36,0x3e,0x26,0xf3,0xc3
.endm
.macro nop6
.byte 0x66,0x0f,0x1f,0x44,0,0
.endm
.text
.globl _start
_start:
.set i, 0
.rept 64
.byte 0x2e,0xe8
.long handlers + i * 18 - . - 4
.set i, i + 1
.endr
ret6
handlers:
.rept 32
mov40
add2
ret6
.endr
.rept 32
nop6
mov40
ret6
.endr

.section .padding,"ax"
.fill 12,1,0xcc
.type padded_data,@object
padded_data:
.byte 0x90                   # one-byte NOP, then an eleven-byte MOV
.byte 0x2e,0x48,0xb8
.quad 40
ret6
.fill 6,1,0xcc
nop6
.type after_padding,@function
after_padding:
mov40
ret6

.section .short_slot,"ax"
.fill 12,1,0xcc
.type split_data,@object
split_data:
.byte 0x2e,0xb8,40,0,0      # only five bytes belong to this section
.section .short_tail,"ax"
.byte 0
add2
ret6

.section .full_slot,"ax"
.fill 12,1,0xcc
.type exact_fit,@function
exact_fit:
mov40                       # the first slot fits exactly
.section .full_tail,"ax"
add2
ret6

.section .shift_short,"ax"
.fill 12,1,0xcc
nop6
.type shifted_split_data,@object
shifted_split_data:
.byte 0x2e,0xb8,40,0,0      # adjusted entry is aligned but its slot is short
.section .shift_tail,"ax"
.byte 0
ret6

# GNU binutils 2.47.20260726. Stripped ELF SHA-256:
# d8b61345325d9fe9dc83ef6dd3d7b7918ef545290ac80b8b89ab5393f9f80505
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
call inside_slot
call following_slot
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
.rept 16
nop6
mov40
ret6
.endr
.rept 16
nop6
nop6
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

.section .overlap,"ax"
.fill 12,1,0
nop6
nop6
.type overlapping_data,@object
overlapping_data:
.byte 0x2e,0x36,0x3e,0x26,0xf3
.type inside_slot,@function
inside_slot:
.byte 0xc3                  # direct target occupies the sixth byte

.section .abutting,"ax"
.fill 12,1,0
nop6
nop6
.type abutting_function,@function
abutting_function:
mov40
.type following_slot,@function
following_slot:
.byte 0xc3                  # direct target starts just after the slot

.section .long_padding,"ax"
.rept 8500
nop6
.endr
.byte 0x90                  # every six-byte NOP leads to the same off-grid return
ret6

.section .long_insn,"ax"
.fill 12,1,0
.type straddling_data,@object
straddling_data:
.byte 0x2e,0x36,0xb8,40,0,0  # six bytes of a seven-byte MOV
.section .long_insn_tail,"ax"
.byte 0
.byte 0x05,2,0,0,0           # five-byte ADD keeps the MOV/ADD fingerprint length at 12
ret6

.section .trapped,"ax"
.fill 12,1,0
.type trapped_padding,@object
trapped_padding:
nop6
nop6
.byte 0x2e,0x36,0x3e,0x26,0xf3,0xcc  # INT3 terminates before the classified RET
ret6
.fill 6,1,0
nop6
nop6
.type reached_function,@function
reached_function:
.byte 0xb8,40,0,0,0           # five-byte MOV fits the first slot
ret6

.section .padded_wide,"ax"
.fill 12,1,0
nop6
nop6
.type wide_after_padding,@object
wide_after_padding:
.byte 0x2e,0x36,0xb8,40,0,0,0
.byte 0x05,2,0,0,0
ret6

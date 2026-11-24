# Synthetic six-byte instruction slots. All callable bodies return 42.
# ELF SHA-256: 4a9dbfd2f4510accf4c928ed645af1a6892feab03e50bd8c869802533b158471
# Rebuild with GNU binutils 2.47.20260726 (the linker script sets the entry and VMAs):
# as --64 aif_slots_x86_64.S -o slots.o
# ld --build-id=none -T aif_slots_x86_64.ld slots.o -o slots
# strip -s slots -o aif_slots_x86_64
.macro body
.byte 0x2e,0xb8,40,0,0,0        # mov $40,%eax
.byte 0x81,0xc0,2,0,0,0         # add $2,%eax
.byte 0x2e,0x36,0x3e,0x26,0xf3,0xc3 # ret
.endm

.section .boot,"ax"
.type at_zero,@function
at_zero:
body

.text
.globl _start
_start:
# Each relative call occupies one slot and seeds AIF's prologue histogram.
.set i, 0
.rept 64
.byte 0x2e,0xe8
.long handlers + i * 18 - . - 4
.set i, i + 1
.endr
body
handlers:
.rept 64
body
.endr

.section .extra,"ax"
.fill 6,1,0xcc
.type first_hidden,@function
first_hidden:
body
.fill 8,1,0xcc
.type segment_decoy,@object
segment_decoy:
body

.section .more,"ax"
.fill 6,1,0xcc
.type second_hidden,@function
second_hidden:
body
.byte 0xcc

.section .fake,"ax"
.byte 0xcc
.type off_grid,@object
off_grid:
body
.fill 5,1,0xcc

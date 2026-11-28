.text
.globl rv_add
.type rv_add,@function
rv_add:
    add a0, a0, a1
    ret
.size rv_add, .-rv_add

.globl rv_immediate
.type rv_immediate,@function
rv_immediate:
    addi a0, a0, -17
    ret
.size rv_immediate, .-rv_immediate

.globl rv_zero_source
.type rv_zero_source,@function
rv_zero_source:
    addi a0, x0, 17
    ret
.size rv_zero_source, .-rv_zero_source

.globl rv_shift
.type rv_shift,@function
rv_shift:
    sra a0, a0, a1
    ret
.size rv_shift, .-rv_shift

.globl rv_memory
.type rv_memory,@function
rv_memory:
    ld a0, 0(a0)
    ret
.size rv_memory, .-rv_memory

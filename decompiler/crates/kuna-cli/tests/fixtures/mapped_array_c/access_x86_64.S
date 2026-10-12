.text
.macro function name
.globl \name
.type \name,@function
\name:
.endm
.macro endfunction name
ret
.size \name,.-\name
.endm
function write1
movb %dil, bytes+3(%rip)
endfunction write1
function write2
movw %di, bytes+1(%rip)
endfunction write2
function write4
movl %edi, bytes+3(%rip)
endfunction write4
function write8
movq %rdi, bytes+7(%rip)
endfunction write8
function read1
movzbl bytes+3(%rip), %eax
endfunction read1
function read2
movzwl bytes+1(%rip), %eax
endfunction read2
function read4
movl bytes+3(%rip), %eax
endfunction read4
function read8
movq bytes+7(%rip), %rax
endfunction read8
function read_signed4
movl bytes+3(%rip), %eax
endfunction read_signed4
function read_const4
movl readonly_bytes+1(%rip), %eax
endfunction read_const4
function whole_write
movq %rdi, whole(%rip)
endfunction whole_write
function whole_read
movq whole(%rip), %rax
endfunction whole_read
function whole_write1
movb %dil, whole1(%rip)
endfunction whole_write1
function whole_read1
movzbl whole1(%rip), %eax
endfunction whole_read1
function whole_write2
movw %di, whole2(%rip)
endfunction whole_write2
function whole_read2
movzwl whole2(%rip), %eax
endfunction whole_read2
function whole_write4
movl %edi, whole4(%rip)
endfunction whole_write4
function whole_read4
movl whole4(%rip), %eax
endfunction whole_read4
function overlap_copy
movq bytes(%rip), %rax
movq %rax, bytes+4(%rip)
movw %di, bytes+6(%rip)
endfunction overlap_copy
.data
.globl bytes
.type bytes,@object
bytes: .zero 32
.size bytes,32
.globl whole
.type whole,@object
whole: .zero 8
.size whole,8
.globl whole1
whole1: .zero 1
.globl whole2
whole2: .zero 2
.globl whole4
whole4: .zero 4
.section .rodata
.globl readonly_bytes
.type readonly_bytes,@object
readonly_bytes: .byte 0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88
.size readonly_bytes,8
.section .note.GNU-stack,"",@progbits

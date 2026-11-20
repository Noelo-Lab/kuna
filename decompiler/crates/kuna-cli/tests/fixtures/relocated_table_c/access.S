#if defined(__x86_64__)
.text
.macro LOOKUP name, origin
.globl \name
.type \name,@function
\name:
    movabsq $sample_table, %rax
    movabsq $\origin, %rcx
    subq %rcx, %rdi
    movsbl (%rax,%rdi), %eax
    ret
.size \name,.-\name
.endm
LOOKUP lookup17,17
LOOKUP lookup53,53
LOOKUP lookup211,211
LOOKUP lookup32,0x80000035
LOOKUP lookup64,0x1000000d3
LOOKUP lookup63,0x7fffffffffffffe0
.globl lookup_half
.type lookup_half,@function
lookup_half:
    movabsq $sample_halves, %rax
    subq $53, %rdi
    movswl (%rax,%rdi,2), %eax
    ret
.size lookup_half,.-lookup_half
.globl store53
.type store53,@function
store53:
    movabsq $sample_table, %rax
    subq $53, %rdi
    movb %sil, (%rax,%rdi)
    ret
.size store53,.-store53
#elif defined(__sparc__)
.text
.macro LOOKUP name, origin
.globl \name
.type \name,#function
\name:
    sethi %hi(sample_table), %g1
    or %g1, %lo(sample_table), %g1
    sub %o0, \origin, %o0
    retl
    ldsb [%g1+%o0], %o0
.size \name,.-\name
.endm
LOOKUP lookup17,17
LOOKUP lookup53,53
LOOKUP lookup211,211
.globl lookup_half
.type lookup_half,#function
lookup_half:
    sethi %hi(sample_halves), %g1
    or %g1, %lo(sample_halves), %g1
    sub %o0, 53, %o0
    sll %o0, 1, %o0
    retl
    ldsh [%g1+%o0], %o0
.size lookup_half,.-lookup_half
.globl store53
.type store53,#function
store53:
    sethi %hi(sample_table), %g1
    or %g1, %lo(sample_table), %g1
    sub %o0, 53, %o0
    retl
    stb %o1, [%g1+%o0]
.size store53,.-store53
#else
#error unsupported test architecture
#endif
.data
.balign 8
.globl sample_prefix
sample_prefix:
    .zero 256
.globl sample_table
#if defined(__sparc__)
.type sample_prefix,#object
.type sample_table,#object
#else
.type sample_prefix,@object
.type sample_table,@object
#endif
.size sample_prefix,256
sample_table:
    .byte -128,127,-1,0,1,93,-93,42,-42,7,-7,63,-64,3,-3,99,-99,11,-11,17,-17,29,-29
.size sample_table,23
.balign 2
.globl sample_halves
#if defined(__sparc__)
.type sample_halves,#object
#else
.type sample_halves,@object
#endif
sample_halves:
    .short -32768,32767,-1,0,1,12345,-12345,999,-999,53,-211
.size sample_halves,22
#if defined(__x86_64__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.macro guard name, shift, register, zext=0, signed=0
.globl \name
.type \name,@function
\name:
    shr $\shift,%rdi
.if \zext
    mov %edi,%edi
.endif
    cmp $40,\register
.if \signed
    jg .L\name\()_default
    test %edi,%edi
    js .L\name\()_default
.else
    ja .L\name\()_default
.endif
    lea .L\name\()_table(%rip),%rax
    movslq (%rax,%rdi,4),%rdi
    add %rax,%rdi
    jmp *%rdi
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40
.L\name\()_\n:
    mov $(\n*7+3),%eax
    ret
.endr
.L\name\()_default:
    mov $1000,%eax
    ret
.size \name,.-\name
.pushsection .rodata
.balign 4
.L\name\()_table:
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40
    .long .L\name\()_\n-.L\name\()_table
.endr
.popsection
.endm

guard narrow_guard,32,%edi
guard wide_guard,32,%rdi
guard explicit_zext,32,%edi,1
guard byte_guard,56,%dil
guard word_guard,48,%di
guard signed_guard,32,%edi,0,1
.section .note.GNU-stack,"",@progbits

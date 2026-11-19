# (kuna) Option `aifdataveto`: `table` sits in .text, looks like a function
# (push rbp; mov rbp,rsp; ...; pop rbp; ret) and is only ever READ as data.
# Build: as aifdataveto_x86_64.s -o t.o && ld -o aifdataveto_x86_64 t.o && strip -s aifdataveto_x86_64
    .text
    .globl _start
_start:
    call reader
    mov  $60,%eax
    syscall
    hlt
    .p2align 4
reader:
    mov  table(%rip),%eax
    add  table+4(%rip),%eax
    ret
    .p2align 4
table:
    push %rbp
    mov  %rsp,%rbp
    mov  $1,%eax
    add  $2,%eax
    pop  %rbp
    ret

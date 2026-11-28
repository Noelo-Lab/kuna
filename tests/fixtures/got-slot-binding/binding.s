/* The relative and absolute calls intentionally have the same link-time VMA. */
.equ absolute_word, 0x402fe0
.text
.macro enter name
    .globl \name
    .type \name,@function
\name:
    sub $8,%rsp
.endm
.macro finish name, value
    mov $\value,%eax
    add $8,%rsp
    ret
    .size \name,.-\name
.endm

enter relative
    mov $17,%edi
    call *exit@GOTPCREL(%rip)
    finish relative, 41

enter absolute
    call *absolute_word
    finish absolute, 42

enter replaced
    lea callback(%rip),%rax
    mov %rax,exit@GOTPCREL(%rip)
    call *exit@GOTPCREL(%rip)
    finish replaced, 43

enter exported
    call abort@PLT
    .size exported,.-exported

enter interposed
    call *exported@GOTPCREL(%rip)
    finish interposed, 44

.globl plus_one, forward
.type plus_one,@function
plus_one:
    lea 1(%rdi),%eax
    ret
.size plus_one,.-plus_one
.type forward,@function
forward:
    sub $8,%rsp
    mov $7,%edi
    call *plus_one@GOTPCREL(%rip)
    add $8,%rsp
    add $2,%eax
    ret
.size forward,.-forward

.type callback,@function
callback:
    ret
.size callback,.-callback
.section .note.GNU-stack,"",@progbits

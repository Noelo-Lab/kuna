.text
.weak optional
.type optional,@function
.globl address_value
.type address_value,@function
address_value: mov saved(%rip),%rax
 ret
.size address_value,.-address_value
.globl address_offset
.type address_offset,@function
address_offset: mov saved+8(%rip),%rax
 ret
.size address_offset,.-address_offset
.globl address_negative
.type address_negative,@function
address_negative: mov saved+16(%rip),%rax
 ret
.size address_negative,.-address_negative
.globl unrelated_literal
.type unrelated_literal,@function
unrelated_literal: mov $0x402000,%eax
 ret
.size unrelated_literal,.-unrelated_literal
.section .rodata
.balign 8
saved:.quad optional,optional+0x8000,optional-0x8000
.section .note.GNU-stack,"",@progbits

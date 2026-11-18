.text
.weak optional
.type optional,@function
.globl address_value
.type address_value,@function
address_value:
 lea optional(%rip),%rax
 ret
.size address_value,.-address_value
.globl address_offset
.type address_offset,@function
address_offset:
 lea optional+0x8000(%rip),%rax
 ret
.size address_offset,.-address_offset
.globl address_negative
.type address_negative,@function
address_negative:
 lea optional-0x8000(%rip),%rax
 ret
.size address_negative,.-address_negative
.globl unrelated_literal
.type unrelated_literal,@function
unrelated_literal:
 mov $0x402000,%eax
 ret
.size unrelated_literal,.-unrelated_literal
.section .note.GNU-stack,"",@progbits

.text
.globl register_offset
.type register_offset,@function
register_offset:
  leaq optional(%rdi),%rax
  ret
.size register_offset,.-register_offset

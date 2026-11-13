.text
.globl load_offset, store_offset, return_offset, load_dynamic, store_dynamic, return_dynamic, return_negative, integer_address
.type load_offset,@function
load_offset: mov 8(%rdi),%rax; ret
.size load_offset,.-load_offset
.type store_offset,@function
store_offset: mov %rsi,8(%rdi); ret
.size store_offset,.-store_offset
.type return_offset,@function
return_offset: lea 8(%rdi),%rax; ret
.size return_offset,.-return_offset
.type load_dynamic,@function
load_dynamic: mov (%rdi,%rsi),%rax; ret
.size load_dynamic,.-load_dynamic
.type store_dynamic,@function
store_dynamic: mov %rdx,(%rdi,%rsi); ret
.size store_dynamic,.-store_dynamic
.type return_dynamic,@function
return_dynamic: lea (%rdi,%rsi),%rax; ret
.size return_dynamic,.-return_dynamic
.type return_negative,@function
return_negative: lea -8(%rdi),%rax; ret
.size return_negative,.-return_negative
.type integer_address,@function
integer_address: lea 8(%rdi),%rax; ret
.size integer_address,.-integer_address
.section .note.GNU-stack,"",@progbits

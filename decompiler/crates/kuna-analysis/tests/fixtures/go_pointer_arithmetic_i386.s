 .text
.globl load_offset, store_offset, return_offset, load_dynamic, store_dynamic, return_dynamic, return_negative, integer_address
.type load_offset,@function
load_offset: mov 4(%esp),%eax; mov 4(%eax),%eax; ret
.size load_offset,.-load_offset
.type store_offset,@function
store_offset: mov 4(%esp),%eax; mov 8(%esp),%edx; mov %edx,4(%eax); ret
.size store_offset,.-store_offset
.type return_offset,@function
return_offset: mov 4(%esp),%eax; add $4,%eax; ret
.size return_offset,.-return_offset
.type load_dynamic,@function
load_dynamic: mov 4(%esp),%eax; mov 8(%esp),%edx; mov (%eax,%edx),%eax; ret
.size load_dynamic,.-load_dynamic
.type store_dynamic,@function
store_dynamic: mov 4(%esp),%eax; mov 8(%esp),%edx; mov 12(%esp),%ecx; mov %ecx,(%eax,%edx); ret
.size store_dynamic,.-store_dynamic
.type return_dynamic,@function
return_dynamic: mov 4(%esp),%eax; add 8(%esp),%eax; ret
.size return_dynamic,.-return_dynamic
.type return_negative,@function
return_negative: mov 4(%esp),%eax; sub $4,%eax; ret
.size return_negative,.-return_negative
.type integer_address,@function
integer_address: mov 4(%esp),%eax; add $4,%eax; ret
.size integer_address,.-integer_address
.section .note.GNU-stack,"",@progbits

.text
.globl metadata8
.type metadata8,@function
metadata8:
 sub $24,%rsp
 lea 32(%rsp),%rax
 mov %rax,8(%rsp)
 lea 8(%rsp),%rsi
 mov $17,%edi
 mov $1,%eax
 mov $8,%edx
 syscall
 add $24,%rsp
 ret
.size metadata8,.-metadata8
.globl metadata40
.type metadata40,@function
metadata40:
 sub $24,%rsp
 lea 64(%rsp),%rax
 mov %rax,8(%rsp)
 lea 8(%rsp),%rsi
 mov $17,%edi
 mov $1,%eax
 mov $8,%edx
 syscall
 add $24,%rsp
 ret
.size metadata40,.-metadata40
.globl abi_echo
.type abi_echo,@function
abi_echo:
 mov %rcx,%rax
 ret
.size abi_echo,.-abi_echo
.section .note.GNU-stack,"",@progbits

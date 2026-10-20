.text
.globl read_float_bits
.type read_float_bits,@function
read_float_bits:
 push %rbp
 mov %rsp,%rbp
 sub $16,%rsp
 mov (%rdi),%eax
 mov %eax,-8(%rbp)
 lea -8(%rbp),%rdi
 call observe_float
 mov -8(%rbp),%eax
 leave
 ret
.size read_float_bits,.-read_float_bits
.globl read_double_bits
.type read_double_bits,@function
read_double_bits:
 push %rbp
 mov %rsp,%rbp
 sub $16,%rsp
 mov (%rdi),%rax
 mov %rax,-8(%rbp)
 lea -8(%rbp),%rdi
 call observe_double
 mov -8(%rbp),%rax
 leave
 ret
.size read_double_bits,.-read_double_bits
.globl read_float_low
.type read_float_low,@function
read_float_low:
 push %rbp
 mov %rsp,%rbp
 sub $16,%rsp
 mov (%rdi),%eax
 mov %eax,-8(%rbp)
 lea -8(%rbp),%rdi
 call observe_float
 movzwl -8(%rbp),%eax
 leave
 ret
.size read_float_low,.-read_float_low
.globl read_float_high
.type read_float_high,@function
read_float_high:
 push %rbp
 mov %rsp,%rbp
 sub $16,%rsp
 mov (%rdi),%eax
 mov %eax,-8(%rbp)
 lea -8(%rbp),%rdi
 call observe_float
 movzwl -6(%rbp),%eax
 leave
 ret
.size read_float_high,.-read_float_high
.globl observe_float
.type observe_float,@function
observe_float:
 ret
.size observe_float,.-observe_float
.globl observe_double
.type observe_double,@function
observe_double:
 ret
.size observe_double,.-observe_double
.section .note.GNU-stack,"",@progbits

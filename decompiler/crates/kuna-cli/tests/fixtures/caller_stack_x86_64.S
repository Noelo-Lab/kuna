/* Unnamed caller addresses, with a physical caller-area oracle. */
.text
.globl caller_first
.type caller_first,@function
caller_first:
lea 8(%rsp),%rax
ret
.size caller_first,.-caller_first

.globl caller_adjacent
.type caller_adjacent,@function
caller_adjacent:
lea 19(%rsp),%rax
ret
.size caller_adjacent,.-caller_adjacent

.globl caller_frame
.type caller_frame,@function
caller_frame:
push %rbp
mov %rsp,%rbp
sub $32,%rsp
lea 27(%rbp),%rax
leave
ret
.size caller_frame,.-caller_frame

.globl caller_write_first
.type caller_write_first,@function
caller_write_first:
lea 8(%rsp),%rsi
mov $1,%eax
mov $17,%edi
mov $16,%edx
syscall
ret
.size caller_write_first,.-caller_write_first

.globl caller_write_adjacent
.type caller_write_adjacent,@function
caller_write_adjacent:
lea 19(%rsp),%rsi
mov $1,%eax
mov $17,%edi
mov $16,%edx
syscall
ret
.size caller_write_adjacent,.-caller_write_adjacent

.globl caller_below
.type caller_below,@function
caller_below:
lea 7(%rsp),%rax
ret
.size caller_below,.-caller_below

.globl caller_local
.type caller_local,@function
caller_local:
lea -16(%rsp),%rax
ret
.size caller_local,.-caller_local

.globl caller_named
.type caller_named,@function
caller_named:
lea 8(%rsp),%rax
ret
.size caller_named,.-caller_named
.section .note.GNU-stack,"",@progbits

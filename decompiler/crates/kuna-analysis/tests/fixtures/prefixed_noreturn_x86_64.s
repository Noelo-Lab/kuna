.text
.macro fn name
.globl \name
.type \name,@function
\name:
.endm
.macro guarded name, callee
fn \name
 push %rbp
 mov %rsp,%rbp
 test %edi,%edi
 je 1f
 mov $7,%eax
 pop %rbp
 ret
1:
 call \callee
.size \name,.-\name
.space 4
.endm
.macro returning name, callee
fn \name
 push %rbp
 mov %rsp,%rbp
 call \callee
 mov $9,%eax
 pop %rbp
 ret
.size \name,.-\name
.endm
.macro stub name
fn \name
 ret
.size \name,.-\name
.endm
guarded probe, stub___stack_chk_fail
guarded plain, __stack_chk_fail
returning keeps_count, mystack_chk_fail_count
returning keeps_exit, handle_exit
stub stub___stack_chk_fail
stub __stack_chk_fail
stub mystack_chk_fail_count
stub handle_exit

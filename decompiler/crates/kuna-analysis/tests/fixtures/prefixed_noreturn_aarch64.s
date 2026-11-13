.text
.macro fn name
.globl \name
.type \name,%function
\name:
.endm
.macro guarded name, callee
fn \name
 stp x29,x30,[sp,#-16]!
 mov x29,sp
 cbz w0,1f
 mov w0,#7
 ldp x29,x30,[sp],#16
 ret
1:
 bl \callee
.size \name,.-\name
.space 4
.endm
.macro returning name, callee
fn \name
 stp x29,x30,[sp,#-16]!
 mov x29,sp
 bl \callee
 mov w0,#9
 ldp x29,x30,[sp],#16
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

.text
.globl through_got
.type through_got,@function
through_got:
 sub $8,%rsp
 call *callback_slot(%rip)
 add $8,%rsp
 ret
.size through_got,.-through_got
.globl through_mutable
.type through_mutable,@function
through_mutable:
 sub $8,%rsp
 call *mutable_slot(%rip)
 add $8,%rsp
 ret
.size through_mutable,.-through_mutable
.globl through_carried
.type through_carried,@function
through_carried:
 sub $8,%rsp
 mov callback_slot(%rip),%rax
 call *%rax
 add $8,%rsp
 ret
.size through_carried,.-through_carried
.section .got,"aw",@progbits
.balign 8
.globl callback_slot
.hidden callback_slot
.type callback_slot,@object
callback_slot: .quad callback
.size callback_slot,8
.data
.balign 8
.globl mutable_slot
.hidden mutable_slot
.type mutable_slot,@object
mutable_slot: .quad callback
.size mutable_slot,8
.section .note.GNU-stack,"",@progbits

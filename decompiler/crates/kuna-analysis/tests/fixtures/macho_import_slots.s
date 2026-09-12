.text
.globl _call_slots
.p2align 4, 0x90
_call_slots:
  pushq %rbp
  movq %rsp, %rbp
  callq *_objc_msgSend@GOTPCREL(%rip)
  callq *1f(%rip)
  popq %rbp
  retq

.section __DATA,__objc_msgrefs
.p2align 3
1:
  .quad _objc_msgSend
  .quad 0

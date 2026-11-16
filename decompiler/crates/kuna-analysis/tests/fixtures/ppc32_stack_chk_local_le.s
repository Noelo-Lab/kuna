 .text
 .globl r6
 .type r6,@function
r6:
 mulli 3,3,3
 cmpwi 4,0
 bne .Lcall
 blr
.Lcall:
 stwu 1,-16(1)
 mflr 0
 stw 0,20(1)
 bl __stack_chk_fail_local
 lwz 0,20(1)
 li 3,0
 addi 1,1,16
 mtlr 0
 blr
 .size r6,.-r6

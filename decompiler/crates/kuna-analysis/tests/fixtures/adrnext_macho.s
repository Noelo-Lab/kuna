; GH #1360's fixture, verbatim: `adr x0, L_next` names the next instruction.
.text
.space 0x1000
.p2align 2
.globl _audit_self
_audit_self:
 adr x0, _audit_self
 ret
.globl _audit_next
_audit_next:
 adr x0, L_next
L_next:
 ret
.globl _audit_after
_audit_after:
 adr x0, L_after
 nop
L_after:
 ret

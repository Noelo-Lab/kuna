.section __TEXT,__text,regular,pure_instructions
.p2align 2
.globl _audit_export_a
.globl _audit_export_b
_audit_export_a:
_audit_export_b:
  add w0, w0, #7
  ret
.p2align 2
.globl _audit_caller
_audit_caller:
  b _audit_export_b

.section __TEXT,__text,regular,pure_instructions
.p2align 2
_audit_shared:
  mov w0, #2
  ret
.p2align 2
_audit_second:
_audit_both:
  mov w0, #3
  ret

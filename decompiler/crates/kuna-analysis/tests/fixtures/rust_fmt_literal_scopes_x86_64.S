/* Namespace lookup for Rust print sinks reached through relocated GOT slots. */
.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
text_lf: .ascii "scoped literal\n"
.text
.macro wrapper name, sink
.globl \name
.type \name,@function
\name:
  subq $8, %rsp
  leaq text_lf(%rip), %rdi
  movq $31, %rsi
  call *\sink@GOTPCREL(%rip)
  addq $8, %rsp
  xorl %eax, %eax
  ret
.size \name,.-\name
.endm
wrapper scoped_stdout, _ZN3std2io5stdio6_print17h0123456789abcdefE
wrapper scoped_stderr, _ZN3std2io5stdio7_eprint17h0123456789abcdefE
wrapper scoped_other, _ZN5other2io5stdio6_print17h0123456789abcdefE
wrapper scoped_incomplete, _ZN3std5stdio6_print17h0123456789abcdefE
wrapper scoped_nested, _ZN3std2io5stdio6nested6_print17h0123456789abcdefE
wrapper scoped_global, _print
.macro sink name
.globl \name
.type \name,@function
\name:
  ret
.size \name,.-\name
.endm
sink _ZN3std2io5stdio6_print17h0123456789abcdefE
sink _ZN3std2io5stdio7_eprint17h0123456789abcdefE
sink _ZN5other2io5stdio6_print17h0123456789abcdefE
sink _ZN3std5stdio6_print17h0123456789abcdefE
sink _ZN3std2io5stdio6nested6_print17h0123456789abcdefE
sink _print
.section .note.GNU-stack,"",@progbits

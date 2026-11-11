/* Rust 1.93 literal fmt::Arguments encoding, with refusal controls. */
.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
text_lf: .ascii "just a literal\n"
text_plain: .ascii "plain"
text_braces: .ascii "{a}\"\\\0\n\n"
text_unicode: .ascii "caf\303\251\n"
text_invalid: .byte 0xff
.section .data
text_mutable: .ascii "mutable\n"
.text
.macro wrapper name, text, tag, sink=_ZN3std2io5stdio6_print17h0123456789abcdefE
.globl \name
.type \name,@function
\name:
  subq $8, %rsp
  leaq \text(%rip), %rdi
  movq $\tag, %rsi
  call \sink
  addq $8, %rsp
  xorl %eax, %eax
  ret
.size \name,.-\name
.endm
wrapper literal_lf, text_lf, 31
wrapper literal_plain, text_plain, 11
wrapper literal_stderr, text_plain, 11, _ZN3std2io5stdio7_eprint17h0123456789abcdefE
wrapper literal_stderr_lf, text_lf, 31, _ZN3std2io5stdio7_eprint17h0123456789abcdefE
wrapper literal_braces, text_braces, 17
wrapper literal_unicode, text_unicode, 13
wrapper reject_utf8, text_invalid, 3
wrapper reject_mutable, text_mutable, 17
wrapper reject_template, text_lf, 30
wrapper reject_large, text_lf, 8195
wrapper reject_sink, text_lf, 31, _ZN5other6_print17h0123456789abcdefE
.globl literal_empty
.type literal_empty,@function
literal_empty:
  subq $8, %rsp
  movq $1, %rdi
  movq $1, %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $8, %rsp
  xorl %eax, %eax
  ret
.size literal_empty,.-literal_empty
.globl reject_null
.type reject_null,@function
reject_null:
  subq $8, %rsp
  xorl %edi, %edi
  movq $1, %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $8, %rsp
  xorl %eax, %eax
  ret
.size reject_null,.-reject_null
wrapper reject_unmapped_span, text_lf, 4095
.globl reject_variable
.type reject_variable,@function
reject_variable:
  subq $8, %rsp
  movq $31, %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $8, %rsp
  xorl %eax, %eax
  ret
.size reject_variable,.-reject_variable
.globl reject_result
.type reject_result,@function
reject_result:
  subq $8, %rsp
  leaq text_lf(%rip), %rdi
  movq $31, %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $8, %rsp
  addq $1, %rax
  ret
.size reject_result,.-reject_result
.globl reject_extra_argument
.type reject_extra_argument,@function
reject_extra_argument:
  subq $8, %rsp
  leaq text_lf(%rip), %rdi
  movq $31, %rsi
  movq $99, %rdx
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $8, %rsp
  xorl %eax, %eax
  ret
.size reject_extra_argument,.-reject_extra_argument
.globl _ZN3std2io5stdio6_print17h0123456789abcdefE
.type _ZN3std2io5stdio6_print17h0123456789abcdefE,@function
_ZN3std2io5stdio6_print17h0123456789abcdefE:
  ret
.size _ZN3std2io5stdio6_print17h0123456789abcdefE,.-_ZN3std2io5stdio6_print17h0123456789abcdefE
.globl _ZN3std2io5stdio7_eprint17h0123456789abcdefE
.type _ZN3std2io5stdio7_eprint17h0123456789abcdefE,@function
_ZN3std2io5stdio7_eprint17h0123456789abcdefE:
  ret
.size _ZN3std2io5stdio7_eprint17h0123456789abcdefE,.-_ZN3std2io5stdio7_eprint17h0123456789abcdefE
.globl _ZN5other6_print17h0123456789abcdefE
.type _ZN5other6_print17h0123456789abcdefE,@function
_ZN5other6_print17h0123456789abcdefE:
  ret
.size _ZN5other6_print17h0123456789abcdefE,.-_ZN5other6_print17h0123456789abcdefE
.section .note.GNU-stack,"",@progbits

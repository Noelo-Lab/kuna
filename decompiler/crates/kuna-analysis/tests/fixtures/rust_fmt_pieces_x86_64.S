/* Descriptor boundary shapes. Output sinks are non-executed ABI stubs. */
.section .comment,"",@progbits
.asciz "rustc version 1.90.0 (1159e78c4 2025-09-14)"
.section .rodata
text: .ascii "{x}\n"
bad_utf8: .byte 0xff,0xfe
.balign 8
literal_table: .quad text,4
invalid_table: .quad bad_utf8,2
large_table: .quad text,4097
unmapped_table: .quad 0x7fff00000000,4
overflow_table: .quad 0xffffffffffffffff,4
null_text_table: .quad 0,0
.section .data
mutable_text: .ascii "{x}\n"
.balign 8
mutable_table: .quad text,4
mutable_text_table: .quad mutable_text,4
sink_pointer: .quad 0
.text
.macro pieces_case name, table=literal_table, pieces=1, args=0, fmt=0, target=_ZN3std2io5stdio6_print17h0123456789abcdefE, mode=0
.globl \name
.type \name,@function
\name:
  subq $56,%rsp
  leaq \table(%rip),%rax
  movq %rax,0(%rsp)
  movq $\pieces,8(%rsp)
  movq $8,16(%rsp)
  movq $\args,24(%rsp)
  movq $\fmt,32(%rsp)
  .if \mode == 1
    movq %rdi,0(%rsp)
  .endif
  .if \mode == 2
    movq %rdi,8(%rsp)
  .endif
  .if \mode == 3
    movq %rdi,32(%rsp)
  .endif
  .if \mode != 4
    leaq 0(%rsp),%rdi
  .endif
  call *\target@GOTPCREL(%rip)
  addq $56,%rsp
  ret
.size \name,.-\name
.endm
pieces_case literal_shape
pieces_case stderr_shape, target=_ZN3std2io5stdio7_eprint17h0123456789abcdefE
pieces_case reject_root, target=_ZN3app2io5stdio6_print17h0123456789abcdefE
pieces_case reject_missing_scope, target=_ZN3std6_print17h0123456789abcdefE
pieces_case reject_extra_scope, target=_ZN3std2io5stdio5extra6_print17h0123456789abcdefE
pieces_case reject_global, target=_print
pieces_case reject_mutable_table, table=mutable_table
pieces_case reject_mutable_text, table=mutable_text_table
pieces_case reject_invalid_utf8, table=invalid_table
pieces_case reject_large, table=large_table
pieces_case reject_unmapped, table=unmapped_table
pieces_case reject_overflow, table=overflow_table
pieces_case reject_null_text, table=null_text_table
pieces_case reject_piece_count, pieces=2
pieces_case reject_argument_count, args=17
pieces_case reject_format_specs, fmt=8
pieces_case reject_dynamic_table, mode=1
pieces_case reject_dynamic_count, mode=2
pieces_case reject_dynamic_format, mode=3
pieces_case reject_forwarded_descriptor, mode=4
.macro sink name
.globl \name
.type \name,@function
\name:
  movq %rdi,sink_pointer(%rip)
  ret
.size \name,.-\name
.endm
sink _ZN3std2io5stdio6_print17h0123456789abcdefE
sink _ZN3std2io5stdio7_eprint17h0123456789abcdefE
sink _ZN3app2io5stdio6_print17h0123456789abcdefE
sink _ZN3std6_print17h0123456789abcdefE
sink _ZN3std2io5stdio5extra6_print17h0123456789abcdefE
sink _print
.section .note.GNU-stack,"",@progbits

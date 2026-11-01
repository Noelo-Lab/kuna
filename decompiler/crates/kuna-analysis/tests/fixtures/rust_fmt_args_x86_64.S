/* Rust 1.93 primitive formatting descriptors and refusal controls. */
.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
plain: .byte 0xc0,0
newline: .byte 0xc0,1,10,0
repeated: .byte 0xc0,1,32,0xc0,1,32,0xc8,0,0,1,10,0
invalid_utf8: .byte 1,255,0xc0,0
invalid_head: .byte 0x81,0
unsupported_flags: .byte 0xc1,0x20,0,0,0x60,0
missing_index: .byte 0xc8,1,0,0
unterminated: .fill 4097,1,0xc0
.section .data
mutable_template: .byte 0xc0,1,10,0
global_value: .quad 7
.text
.globl decimal
.type decimal,@function
decimal:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size decimal,.-decimal
.globl decimal_lf
.type decimal_lf,@function
decimal_lf:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq newline(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size decimal_lf,.-decimal_lf
.globl decimal_stderr
.type decimal_stderr,@function
decimal_stderr:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio7_eprint17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size decimal_stderr,.-decimal_stderr
.globl signed_lf
.type signed_lf,@function
signed_lf:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h010192852a1970dfE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq newline(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio7_eprint17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size signed_lf,.-signed_lf
.globl narrow
.type narrow,@function
narrow:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size narrow,.-narrow
.globl reject_utf8
.type reject_utf8,@function
reject_utf8:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq invalid_utf8(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_utf8,.-reject_utf8
.globl reject_head
.type reject_head,@function
reject_head:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq invalid_head(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_head,.-reject_head
.globl reject_flags
.type reject_flags,@function
reject_flags:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq unsupported_flags(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_flags,.-reject_flags
.globl reject_index
.type reject_index,@function
reject_index:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq missing_index(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_index,.-reject_index
.globl reject_unterminated
.type reject_unterminated,@function
reject_unterminated:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq unterminated(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_unterminated,.-reject_unterminated
.globl reject_mutable
.type reject_mutable,@function
reject_mutable:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq mutable_template(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_mutable,.-reject_mutable
.globl reject_trait
.type reject_trait,@function
reject_trait:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp53_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_trait,.-reject_trait
.globl reject_unknown
.type reject_unknown,@function
reject_unknown:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq unknown_fmt(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_unknown,.-reject_unknown
.globl reject_callback
.type reject_callback,@function
reject_callback:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  movq %rdx, %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_callback,.-reject_callback
.globl reject_sink
.type reject_sink,@function
reject_sink:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN5other6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_sink,.-reject_sink
.globl reject_global_value
.type reject_global_value,@function
reject_global_value:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq global_value(%rip), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_global_value,.-reject_global_value
.globl reject_result
.type reject_result,@function
reject_result:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)

  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  addq $1, %rax
  ret
.size reject_result,.-reject_result
.globl reject_clobber
.type reject_clobber,@function
reject_clobber:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq 8(%rsp), %rdi
  call clobber
  leaq plain(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size reject_clobber,.-reject_clobber
.globl reorder
.type reorder,@function
reorder:
  subq $40, %rsp
  movl %edi, (%rsp)
  movl %esi, 4(%rsp)
  leaq 4(%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E(%rip), %rax
  movq %rax, 16(%rsp)
  movq %rax, 32(%rsp)
  leaq (%rsp), %rax
  movq %rax, 24(%rsp)
  leaq repeated(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $40, %rsp
  xorl %eax, %eax
  ret
.size reorder,.-reorder
.globl _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE
.type _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE,@function
_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE:
  nop
  ret
.size _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE,.-_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h970422b3c504363aE
.globl _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h010192852a1970dfE
.type _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h010192852a1970dfE,@function
_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h010192852a1970dfE:
  nop
  ret
.size _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h010192852a1970dfE,.-_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h010192852a1970dfE
.globl _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E
.type _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E,@function
_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E:
  nop
  ret
.size _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E,.-_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h54a7ce44a80f16b3E
.globl _ZN4core3fmt3num3imp53_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num3imp53_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num3imp53_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE:
  nop
  ret
.size _ZN4core3fmt3num3imp53_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num3imp53_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
.globl _ZN3std2io5stdio6_print17h0123456789abcdefE
.type _ZN3std2io5stdio6_print17h0123456789abcdefE,@function
_ZN3std2io5stdio6_print17h0123456789abcdefE:
  nop
  ret
.size _ZN3std2io5stdio6_print17h0123456789abcdefE,.-_ZN3std2io5stdio6_print17h0123456789abcdefE
.globl _ZN3std2io5stdio7_eprint17h0123456789abcdefE
.type _ZN3std2io5stdio7_eprint17h0123456789abcdefE,@function
_ZN3std2io5stdio7_eprint17h0123456789abcdefE:
  nop
  ret
.size _ZN3std2io5stdio7_eprint17h0123456789abcdefE,.-_ZN3std2io5stdio7_eprint17h0123456789abcdefE
.globl _ZN5other6_print17h0123456789abcdefE
.type _ZN5other6_print17h0123456789abcdefE,@function
_ZN5other6_print17h0123456789abcdefE:
  nop
  ret
.size _ZN5other6_print17h0123456789abcdefE,.-_ZN5other6_print17h0123456789abcdefE
.globl unknown_fmt
.type unknown_fmt,@function
unknown_fmt:
  nop
  ret
.size unknown_fmt,.-unknown_fmt
.globl clobber
.type clobber,@function
clobber:
  nop
  ret
.size clobber,.-clobber
.section .note.GNU-stack,"",@progbits

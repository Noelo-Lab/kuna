/* Verified result-call shapes; sink bodies are ABI stubs, not runtime implementations. */
.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
plain: .byte 0xc0,0
newline: .byte 0xc0,1,10,0
bad: .byte 0x81,0
.section .data
mutable: .byte 0xc0,0
global_value: .quad 7
global_result: .zero 24
heap_pointer: .quad global_result
.text
.globl result_decimal
.type result_decimal,@function
result_decimal:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size result_decimal,.-result_decimal
.globl result_lf
.type result_lf,@function
result_lf:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq newline(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size result_lf,.-result_lf
.globl result_scoped
.type result_scoped,@function
result_scoped:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call *_ZN5alloc3fmt6format12format_inner17h0123456789abcdefE@GOTPCREL(%rip)
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size result_scoped,.-result_scoped
.globl reject_root
.type reject_root,@function
reject_root:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN3app3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_root,.-reject_root
.globl reject_missing
.type reject_missing,@function
reject_missing:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_missing,.-reject_missing
.globl reject_extra
.type reject_extra,@function
reject_extra:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format5extra12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_extra,.-reject_extra
.globl reject_global
.type reject_global,@function
reject_global:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call format_inner
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_global,.-reject_global
.globl reject_null
.type reject_null,@function
reject_null:
  subq $56, %rsp
  movq %rsi, (%rsp)
  xorl %edi, %edi
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_null,.-reject_null
.globl reject_constant
.type reject_constant,@function
reject_constant:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq global_result(%rip), %rdi
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_constant,.-reject_constant
.globl reject_local
.type reject_local,@function
reject_local:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq 24(%rsp), %rdi
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_local,.-reject_local
.globl reject_unaligned
.type reject_unaligned,@function
reject_unaligned:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq 25(%rsp), %rdi
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_unaligned,.-reject_unaligned
.globl reject_computed
.type reject_computed,@function
reject_computed:
  subq $56, %rsp
  movq %rsi, (%rsp)
  call output_pointer
  movq %rax, %rdi
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_computed,.-reject_computed
.globl reject_returned
.type reject_returned,@function
reject_returned:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  movq %rax, global_value(%rip)
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_returned,.-reject_returned
.globl reject_value
.type reject_value,@function
reject_value:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq global_value(%rip), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_value,.-reject_value
.globl reject_template
.type reject_template,@function
reject_template:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq bad(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_template,.-reject_template
.globl reject_mutable
.type reject_mutable,@function
reject_mutable:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  movq %rax, 16(%rsp)
  leaq mutable(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_mutable,.-reject_mutable
.globl reject_missing_cell
.type reject_missing_cell,@function
reject_missing_cell:
  subq $56, %rsp
  movq %rsi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE(%rip), %rax
  leaq plain(%rip), %rsi
  leaq 8(%rsp), %rdx
  call _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
  addq $56, %rsp
  xorl %eax, %eax
  ret
.size reject_missing_cell,.-reject_missing_cell
.globl _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
.type _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE,@function
_ZN5alloc3fmt6format12format_inner17h0123456789abcdefE:
  movq %rsi, (%rdi)
  movq %rdx, 8(%rdi)
  movq $0, 16(%rdi)
  movq %rdi, %rax
  ret
.size _ZN5alloc3fmt6format12format_inner17h0123456789abcdefE,.-_ZN5alloc3fmt6format12format_inner17h0123456789abcdefE
.globl _ZN3app3fmt6format12format_inner17h0123456789abcdefE
.type _ZN3app3fmt6format12format_inner17h0123456789abcdefE,@function
_ZN3app3fmt6format12format_inner17h0123456789abcdefE:
  movq %rsi, (%rdi)
  movq %rdx, 8(%rdi)
  movq $0, 16(%rdi)
  movq %rdi, %rax
  ret
.size _ZN3app3fmt6format12format_inner17h0123456789abcdefE,.-_ZN3app3fmt6format12format_inner17h0123456789abcdefE
.globl _ZN5alloc3fmt12format_inner17h0123456789abcdefE
.type _ZN5alloc3fmt12format_inner17h0123456789abcdefE,@function
_ZN5alloc3fmt12format_inner17h0123456789abcdefE:
  movq %rsi, (%rdi)
  movq %rdx, 8(%rdi)
  movq $0, 16(%rdi)
  movq %rdi, %rax
  ret
.size _ZN5alloc3fmt12format_inner17h0123456789abcdefE,.-_ZN5alloc3fmt12format_inner17h0123456789abcdefE
.globl _ZN5alloc3fmt6format5extra12format_inner17h0123456789abcdefE
.type _ZN5alloc3fmt6format5extra12format_inner17h0123456789abcdefE,@function
_ZN5alloc3fmt6format5extra12format_inner17h0123456789abcdefE:
  movq %rsi, (%rdi)
  movq %rdx, 8(%rdi)
  movq $0, 16(%rdi)
  movq %rdi, %rax
  ret
.size _ZN5alloc3fmt6format5extra12format_inner17h0123456789abcdefE,.-_ZN5alloc3fmt6format5extra12format_inner17h0123456789abcdefE
.globl format_inner
.type format_inner,@function
format_inner:
  movq %rsi, (%rdi)
  movq %rdx, 8(%rdi)
  movq $0, 16(%rdi)
  movq %rdi, %rax
  ret
.size format_inner,.-format_inner
.globl _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE:
  ret
.size _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
.globl output_pointer
.type output_pointer,@function
output_pointer:
  movq heap_pointer(%rip), %rax
  ret
.size output_pointer,.-output_pointer
.section .note.GNU-stack,"",@progbits

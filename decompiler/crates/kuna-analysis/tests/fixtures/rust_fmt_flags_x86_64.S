/* Rust 1.93 static format specifications and malformed descriptor controls. */
.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
hex_width_template:
  .byte 195
  .long 1761607712
  .byte 8,0
  .byte 1,10,0
zero_precision_template:
  .byte 193
  .long 1879048224
  .byte 1,10,0
zero_width_template:
  .byte 193
  .long 1761607712
  .byte 1,10,0
debug_upper_template:
  .byte 193
  .long 1686110240
  .byte 1,10,0
reject_unicode_template:
  .byte 193
  .long 1610668032
  .byte 1,10,0
reject_large_unicode_template:
  .byte 193
  .long 1611726848
  .byte 1,10,0
reject_brace_template:
  .byte 193
  .long 123
  .byte 1,10,0
reject_fill_template:
  .byte 193
  .long 1610612782
  .byte 1,10,0
reject_reserved_template:
  .byte 193
  .long 3758096416
  .byte 1,10,0
reject_signs_template:
  .byte 193
  .long 1616904224
  .byte 1,10,0
reject_debug_modes_template:
  .byte 193
  .long 1711276064
  .byte 1,10,0
reject_debug_trait_template:
  .byte 193
  .long 1652555808
  .byte 1,10,0
reject_disabled_width_template:
  .byte 195
  .long 1610612768
  .byte 8,0
  .byte 1,10,0
reject_disabled_precision_template:
  .byte 197
  .long 1610612768
  .byte 3,0
  .byte 1,10,0
reject_zero_width_template:
  .byte 193
  .long 1744830496
  .byte 1,10,0
reject_dynamic_width_template:
  .byte 211
  .long 1761607712
  .byte 8,0
  .byte 1,10,0
reject_dynamic_precision_template:
  .byte 229
  .long 1879048224
  .byte 3,0
  .byte 1,10,0
reject_alias_template:
  .byte 195
  .long 1761607712
  .byte 8,0
  .byte 1,10,0
.text
.macro print_case name, template, formatter
.globl \name
.type \name,@function
\name:
  subq $24, %rsp
  movq %rdi, (%rsp)
  leaq (%rsp), %rax
  movq %rax, 8(%rsp)
  leaq \formatter(%rip), %rax
  movq %rax, 16(%rsp)
  leaq \template(%rip), %rdi
  leaq 8(%rsp), %rsi
  call _ZN3std2io5stdio6_print17h0123456789abcdefE
  addq $24, %rsp
  xorl %eax, %eax
  ret
.size \name,.-\name
.endm
print_case hex_width, hex_width_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case zero_precision, zero_precision_template, _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
print_case zero_width, zero_width_template, _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
print_case debug_upper, debug_upper_template, _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_unicode, reject_unicode_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_large_unicode, reject_large_unicode_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_brace, reject_brace_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_fill, reject_fill_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_reserved, reject_reserved_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_signs, reject_signs_template, _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
print_case reject_debug_modes, reject_debug_modes_template, _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_debug_trait, reject_debug_trait_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_disabled_width, reject_disabled_width_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_disabled_precision, reject_disabled_precision_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_zero_width, reject_zero_width_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_dynamic_width, reject_dynamic_width_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_dynamic_precision, reject_dynamic_precision_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
print_case reject_alias, reject_alias_template, _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE
.globl _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE:
  ret
.size _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h0123456789abcdefE
.globl _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE:
  ret
.size _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
.globl _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE:
  ret
.size _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h0123456789abcdefE
.globl _ZN3std2io5stdio6_print17h0123456789abcdefE
.type _ZN3std2io5stdio6_print17h0123456789abcdefE,@function
_ZN3std2io5stdio6_print17h0123456789abcdefE:
  ret
.size _ZN3std2io5stdio6_print17h0123456789abcdefE,.-_ZN3std2io5stdio6_print17h0123456789abcdefE
.globl _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE:
.globl _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE
.type _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE,@function
_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE:
  ret
.size _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE
.size _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE,.-_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17h0123456789abcdefE
.section .note.GNU-stack,"",@progbits

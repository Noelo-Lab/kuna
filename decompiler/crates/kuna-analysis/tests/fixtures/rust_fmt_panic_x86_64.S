/* Panic ABI shapes; sink bodies are non-executed stubs, not panic implementations. */
.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
literal: .ascii "{x}\n"
invalid: .byte 0xff,0xff,0xff,0xff
filename: .asciz "panic.rs"
.balign 8
location: .quad filename,8
          .long 1,2
.section .data
mutable: .ascii "{x}\n"
mutable_location: .quad filename,8
                  .long 1,2
sink_values: .zero 24
.text
.macro panic_case name, text=literal, tag=9, loc=location, target=_ZN4core9panicking9panic_fmt17h0123456789abcdefE, got=0
.globl \name
.type \name,@function
\name:
  subq $8,%rsp
  leaq \text(%rip),%rdi
  movq $\tag,%rsi
  .ifc \loc,dynamic
  .else
    .ifc \loc,null
      xorl %edx,%edx
    .else
      .ifc \loc,unmapped
        movq $0x7fff00000000,%rdx
      .else
        leaq \loc(%rip),%rdx
      .endif
    .endif
  .endif
  .if \got
    call *\target@GOTPCREL(%rip)
  .else
    call \target
  .endif
  ud2
.size \name,.-\name
.endm
panic_case literal_panic
panic_case scoped_panic, got=1
panic_case reject_root, target=_ZN3app9panicking9panic_fmt17h0123456789abcdefE
panic_case reject_missing, target=_ZN4core9panic_fmt17h0123456789abcdefE
panic_case reject_extra, target=_ZN4core9panicking5extra9panic_fmt17h0123456789abcdefE
panic_case reject_global, target=panic_fmt
panic_case reject_null, loc=null
panic_case static_mutable_location, loc=mutable_location
panic_case reject_dynamic_location, loc=dynamic
panic_case reject_unmapped_location, loc=unmapped
panic_case reject_mutable_text, text=mutable
panic_case reject_invalid_utf8, text=invalid
panic_case reject_even_tag, tag=8
panic_case reject_excess_length, tag=8195
.macro sink name
.globl \name
.type \name,@function
\name:
  movq %rdi,sink_values(%rip)
  movq %rsi,sink_values+8(%rip)
  movq %rdx,sink_values+16(%rip)
  ud2
.size \name,.-\name
.endm
sink _ZN4core9panicking9panic_fmt17h0123456789abcdefE
sink _ZN3app9panicking9panic_fmt17h0123456789abcdefE
sink _ZN4core9panic_fmt17h0123456789abcdefE
sink _ZN4core9panicking5extra9panic_fmt17h0123456789abcdefE
sink panic_fmt
.section .note.GNU-stack,"",@progbits

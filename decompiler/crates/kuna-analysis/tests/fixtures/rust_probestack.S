.text
.p2align 4
.globl __rust_probestack
.type __rust_probestack,@function
__rust_probestack:
  .byte 0x55,0x48,0x89,0xe5,0x49,0x89,0xc3,0x49,0x81,0xfb,0x00,0x10,0x00,0x00,0x76,0x1c
  .byte 0x48,0x81,0xec,0x00,0x10,0x00,0x00,0x48,0x85,0x64,0x24,0x08,0x49,0x81,0xeb,0x00,0x10,0x00,0x00,0x49,0x81,0xfb,0x00,0x10,0x00,0x00,0x77,0xe4
  .byte 0x4c,0x29,0xdc,0x48,0x85,0x64,0x24,0x08,0x48,0x01,0xc4,0xc9,0xc3
.size __rust_probestack,.-__rust_probestack

.macro frame name, amount, helper=__rust_probestack
.p2align 4
.globl \name
.type \name,@function
\name:
  push %rbx
  mov %edi,%ebx
  mov $\amount,%eax
  call \helper
  sub %rax,%rsp
  mov %ebx,(%rsp)
  movl $7,\amount-4(%rsp)
  mov (%rsp),%eax
  add \amount-4(%rsp),%eax
  add $\amount,%rsp
  pop %rbx
  ret
.size \name,.-\name
.endm
frame probe_page, 0x1000
frame probe_tail, 0x1008
frame probe_pages, 0x3000

.p2align 4
.globl probe_observes_r11
.type probe_observes_r11,@function
probe_observes_r11:
  push %rbx
  mov %edi,%ebx
  mov $0x1008,%eax
  call __rust_probestack
  sub %rax,%rsp
  mov %r11d,%eax
  add %ebx,%eax
  add $0x1008,%rsp
  pop %rbx
  ret
.size probe_observes_r11,.-probe_observes_r11

.p2align 4
.globl not_a_probe
.type not_a_probe,@function
not_a_probe:
  incl probe_counter(%rip)
  ret
.size not_a_probe,.-not_a_probe
frame probe_side_effect, 0x1008, not_a_probe


.macro residual name, amount
.p2align 4
.globl \name
.type \name,@function
\name:
  push %rbx
  mov %edi,%ebx
  mov $\amount,%eax
  call __rust_probestack
  sub %rax,%rsp
  mov %r11d,%eax
  add %ebx,%eax
  add $\amount,%rsp
  pop %rbx
  ret
.size \name,.-\name
.endm
residual probe_zero, 0
residual probe_one, 1
residual probe_page_r11, 4096
residual probe_plus_one, 4097
residual probe_pages_r11, 8192

.p2align 4
.globl probe_saved_rbp
.type probe_saved_rbp,@function
probe_saved_rbp:
  push %rbp
  mov %edi,%ebp
  mov $0x1008,%eax
  call __rust_probestack
  sub %rax,%rsp
  mov 0xff8(%rsp),%eax
  add $0x1008,%rsp
  pop %rbp
  ret
.size probe_saved_rbp,.-probe_saved_rbp

.p2align 4
.globl probe_return_slot
.type probe_return_slot,@function
probe_return_slot:
  mov $0x1008,%eax
  call __rust_probestack
.Lcontinuation:
  sub %rax,%rsp
  lea .Lcontinuation(%rip),%rcx
  xor %eax,%eax
  cmp %rcx,0x1000(%rsp)
  sete %al
  add $0x1008,%rsp
  ret
.size probe_return_slot,.-probe_return_slot

.p2align 4
.globl probe_observes_flags
.type probe_observes_flags,@function
probe_observes_flags:
  mov $0x1008,%eax
  call __rust_probestack
  sete %cl
  sub %rax,%rsp
  movzbl %cl,%eax
  add $0x1008,%rsp
  ret
.size probe_observes_flags,.-probe_observes_flags

.p2align 4
.globl probe_unknown
.type probe_unknown,@function
probe_unknown:
  push %rbx
  mov %edi,%ebx
  mov %edi,%eax
  call __rust_probestack
  sub %rax,%rsp
  mov %r11d,%eax
  add %rbx,%rsp
  pop %rbx
  ret
.size probe_unknown,.-probe_unknown

.data
.p2align 2
.globl probe_counter
.type probe_counter,@object
.size probe_counter,4
probe_counter: .long 0
.section .note.GNU-stack,"",@progbits

.text
.p2align 4
.globl probe_buffer
.type probe_buffer,@function
probe_buffer:
  push %rbx
  push %r12
  mov %edi,%ebx
  mov $0x1008,%eax
  call __rust_probestack
  sub %rax,%rsp
  mov %ebx,%esi
  mov %rsp,%rdi
  call fill_probe_page
  add $0x1008,%rsp
  pop %r12
  pop %rbx
  ret
.size probe_buffer,.-probe_buffer
.section .note.GNU-stack,"",@progbits

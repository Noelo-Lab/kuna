.text
.globl caller
caller:
  push %rbx
  mov %rdi,%rbx
  test %rbx,%rbx
  je 1f
  call clobber
  jmp 2f
1:
  mov $7,%edx
2:
  mov %rbx,%rdi
  mov $5,%esi
  call target
  pop %rbx
  ret
.globl clobber
clobber:
  call helper
  ret
.globl target
target:
  push %rbx
  mov %rdi,%rbx
  call helper
  mov %rbx,%rax
  pop %rbx
  ret
.globl helper
helper:
  ret
.globl _start
_start:
  mov $1,%rdi
  call caller
  ret

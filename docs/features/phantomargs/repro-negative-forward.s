.text
.globl caller2
caller2:
  push %rbx
  push %r12
  mov %rdi,%rbx
  mov %rsi,%r12
  test %rbx,%rbx
  je 1f
  call clobber
  jmp 2f
1:
  mov %r12,%rdx
2:
  mov %rbx,%rdi
  mov $5,%esi
  call target
  pop %r12
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
  mov $2,%rsi
  call caller2
  ret

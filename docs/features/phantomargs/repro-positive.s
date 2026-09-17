.globl _start
caller: push %rbx;  mov %rdi,%rbx;  test %rsi,%rsi;  je 1f
        call clobber;  jmp 2f
1:      mov %ebx,%eax;  cltd;  mov $200,%ecx;  idivl %ecx;  mov %eax,%ebx
2:      mov %rbx,%rdi;  mov $5,%esi;  call target;  pop %rbx;  ret
clobber: call helper; ret
target:  push %rbx; mov %rdi,%rbx; call helper; mov %rbx,%rax; pop %rbx; ret
helper:  ret
_start:  mov $1,%rdi;  mov $3,%rsi;  call caller;  ret

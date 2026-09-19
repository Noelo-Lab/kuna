# tests/stages/kuna-argclobber.xml's caller, as a real program so that
# `decompile-all` decompiles `target` before `caller` and `target`'s recovered
# prototype is on the table when the call site is scored.
#
# rdx at `call target` is a two-input join and neither input is a value the
# caller put there: on one path `clobber` destroyed the register, on the other
# an `idivl` left its remainder in edx while the caller went on to use only the
# quotient. `target` reads rdi and nothing else, so its recovered prototype
# takes one parameter and says rdx is free.
#
#   gcc -nostdlib -static -o argclobber_x86_64 argclobber_x86_64.s
.globl _start
caller: push %rbx;  mov %rdi,%rbx;  test %rsi,%rsi;  je 1f
        call clobber;  jmp 2f
1:      mov %ebx,%eax;  cltd;  mov $200,%ecx;  idivl %ecx;  mov %eax,%ebx
2:      mov %rbx,%rdi;  mov $5,%esi;  call target;  pop %rbx;  ret
clobber: call helper;  ret
target:  push %rbx;  mov %rdi,%rbx;  call helper;  mov %rbx,%rax;  pop %rbx;  ret
helper:  ret
_start:  mov $1,%rdi;  mov $3,%rsi;  call caller;  ret

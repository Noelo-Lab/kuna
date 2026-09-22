# The fixture caller (a clobber on one path, an idivl remainder on the other)
# calling a two-member cycle: `a` never names rdx and hands it on to `b`, and
# `b` reads rdx in its first instruction and calls `a` back.  `a`'s own
# recovery is short (the forwarded rdx is not one of its parameters), and under
# `protoorder cycles` it states that short list; the drop must still decline.
#
#   gcc -nostdlib -static -o ce-rec-pair ce-rec-pair.s
.globl _start
caller: push %rbx;  mov %rdi,%rbx;  test %rsi,%rsi;  je 1f
        call clobber;  jmp 2f
1:      mov %ebx,%eax;  cltd;  mov $200,%ecx;  idivl %ecx;  mov %eax,%ebx
2:      mov %rbx,%rdi;  mov $5,%esi;  call a;  pop %rbx;  ret
a:      test %rsi,%rsi;  je 1f
        sub $1,%rsi;  call b;  ret
1:      mov %rdi,%rax;  ret
b:      add %rdx,%rdi;  call a;  ret
clobber: call helper; ret
helper:  ret
_start:  mov $1,%rdi;  mov $3,%rsi;  call caller;  ret

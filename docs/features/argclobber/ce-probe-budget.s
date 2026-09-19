# tests/stages/kuna-argclobber-guards.xml's caller, byte for byte in
# shape (a clobber on one path, an idivl remainder on the other), with a
# callee that READS rdx in its first instruction -- but whose call-free body
# is longer than the entry probe's 192-instruction budget.
.globl _start
caller: push %rbx;  mov %rdi,%rbx;  test %rsi,%rsi;  je 1f
        call clobber;  jmp 2f
1:      mov %ebx,%eax;  cltd;  mov $200,%ecx;  idivl %ecx;  mov %eax,%ebx
2:      mov %rbx,%rdi;  mov $5,%esi;  call reader;  pop %rbx;  ret
clobber: call helper; ret
reader:  mov %rdx,%rax
        .rept 200
        add %rdi,%rax
        .endr
        ret
helper:  ret
_start:  mov $1,%rdi;  mov $3,%rsi;  call caller;  ret

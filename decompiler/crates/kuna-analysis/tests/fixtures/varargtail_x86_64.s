# `passthrough`: the register a callee reads ONLY to hand to a variadic call is
# not a parameter its callers may gain.  openssh `xcalloc`'s shape, reduced.
#
#   vsink   six register arguments and two stack ones: what a variadic callee
#           reads when the caller pushed a vararg past the register file
#   xfail   rdi and rsi are real parameters.  On the error path it sets the call
#           to vsink up the way gcc sets up a variadic one -- `push %rdx` as the
#           stack-alignment filler (rdx holds nothing; the register is loaded
#           AFTER the push, exactly as in openssh's xcalloc), `push %rax` for
#           the message, `xor %eax,%eax` for the vector count.  Recovery reports
#           rdx as a third parameter in BOTH arms, because the push is a write
#           of the caller's incoming register into an argument slot.
#   fwdx    reads its own rdi and rsi and forwards rdx untouched to xfail: the
#           caller `passthrough` would hand xfail's phantom third parameter to.
#           It must come out with TWO parameters.
#   good    the positive control: `jmp sum`, forwarding rdi untouched to a
#           callee that reads it, and must gain it.
#
#   gcc -nostdlib -static -o varargtail_x86_64 varargtail_x86_64.s && strip varargtail_x86_64
.globl _start
vsink:     mov 8(%rsp),%rax
           add 16(%rsp),%rax
           add %rdi,%rax
           add %rsi,%rax
           add %rdx,%rax
           add %rcx,%rax
           add %r8,%rax
           add %r9,%rax
           ret
xfail:     test %rdi,%rdi
           je 1f
           mov %rsi,%rax
           add %rdi,%rax
           ret
1:         push %rdx
           lea msg(%rip),%rax
           push %rax
           xor %ecx,%ecx
           mov $1,%r8d
           xor %r9d,%r9d
           mov $0x34,%edx
           lea msg(%rip),%rsi
           lea msg(%rip),%rdi
           xor %eax,%eax
           call vsink
           add $16,%rsp
           xor %eax,%eax
           ret
fwdx:      add $1,%rdi
           add $2,%rsi
           jmp xfail
sum:       movzbl (%rdi),%eax
           ret
good:      jmp sum
_start:    mov $1,%edi
           mov $2,%esi
           mov $3,%edx
           call fwdx
           lea buf(%rip),%rdi
           call good
           mov $4,%edi
           mov $5,%esi
           mov $6,%edx
           call xfail
           mov $60,%eax
           xor %edi,%edi
           syscall
           hlt
.data
buf:       .asciz "a/b"
msg:       .asciz "m"

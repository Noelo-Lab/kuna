# `passthrough`: registers a function forwards untouched to a callee that reads
# them, as a real program so that `decompile-all` decompiles each callee before
# its callers and the callee's recovered prototype is on the table.
#
#   basename   `jmp last` -- forwards rdi and hands back last's rax (gzip
#              gzip_base_name's shape)
#   dirname    `call last; test %rax,%rax` -- forwards rdi, tests the result
#              (coreutils df dir_name's shape)
#   clobbered  `call helper; jmp last` -- rdi at the tail call is whatever
#              helper left there, so it is NOT an argument, and helper must not
#              gain one either
#   callv      `xor %eax,%eax; jmp vsum` -- vsum saves every argument register
#              (a variadic's register-save prologue), so its recovered list is
#              no arity claim and callv gains nothing
#
# Controls that must keep every argument the option-off run gives them
# (gcc -O2 shapes: IPA-RA does not re-save rdi around a callee that leaves it):
#   e1         `call noop; mov $3,%esi; call twoarg` -- rdi reaches twoarg
#              through noop's call: keeps `twoarg(a0,3)`
#   d1, d3     `call vout; mov $3,%esi; jmp/call twoarg` -- vout gains the
#              forwarded rdi, twoarg keeps `(a0,3)`
#   sysint     tar sysinttostr: `mov %rcx,%rsi; cmp %rdx,%rdi; ja; jmp umax;
#              jmp imax` -- both tail calls keep `(a0,a3)`
#
#   gcc -nostdlib -static -o passthrough_x86_64 passthrough_x86_64.s && strip passthrough_x86_64
.globl _start
last:      mov %rdi,%rax
1:         movzbl (%rdi),%edx
           test %dl,%dl
           je 3f
           add $1,%rdi
           cmp $0x2f,%dl
           jne 1b
           mov %rdi,%rax
           jmp 1b
3:         ret
basename:  jmp last
dirname:   sub $8,%rsp
           call last
           test %rax,%rax
           je 4f
           add $8,%rsp
           ret
4:         call die
helper:    xor %eax,%eax
           ret
clobbered: sub $8,%rsp
           call helper
           add $8,%rsp
           jmp last
vsum:      sub $0x38,%rsp
           mov %rsi,0x8(%rsp)
           mov %rdx,0x10(%rsp)
           mov %rcx,0x18(%rsp)
           mov %r8,0x20(%rsp)
           mov %r9,0x28(%rsp)
           mov 0x8(%rsp),%rax
           add %rdi,%rax
           add $0x38,%rsp
           ret
callv:     xor %eax,%eax
           jmp vsum
_start:    lea buf(%rip),%rdi
           call basename
           lea buf(%rip),%rdi
           call dirname
           lea buf(%rip),%rdi
           call clobbered
           mov $3,%edi
           mov $4,%esi
           call callv
           mov $5,%edi
           mov $6,%esi
           xor %eax,%eax
           call vsum
           lea buf(%rip),%rdi
           call e1
           lea buf(%rip),%rdi
           call d1
           lea buf(%rip),%rdi
           call d3
           mov $1,%edi
           mov $2,%esi
           mov $3,%edx
           lea buf(%rip),%rcx
           call sysint
           mov $7,%edi
           lea buf(%rip),%rsi
           call umax
           mov $8,%edi
           lea buf(%rip),%rsi
           call imax
           mov $60,%eax
           xor %edi,%edi
           syscall
die:       mov $60,%eax
           mov $1,%edi
           syscall
           hlt
noop:      ret
twoarg:    movslq %esi,%rax
           movsbl (%rdi,%rax,1),%eax
           add %esi,%eax
           ret
vout:      movsbl (%rdi),%eax
           add %eax,glob(%rip)
           ret
e1:        call noop
           mov $3,%esi
           call twoarg
           mov %eax,glob(%rip)
           ret
d1:        call vout
           mov $3,%esi
           jmp twoarg
d3:        call vout
           mov $3,%esi
           call twoarg
           mov %eax,glob(%rip)
           xor %eax,%eax
           ret
umax:      lea 20(%rsi),%rax
           mov %rdi,(%rsi)
           ret
imax:      lea 21(%rsi),%rax
           neg %rdi
           mov %rdi,(%rsi)
           ret
sysint:    mov %rcx,%rsi
           cmp %rdx,%rdi
           ja 5f
           jmp umax
5:         jmp imax
.data
buf:       .asciz "a/b/c"
glob:      .long 0

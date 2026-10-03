.text
.global retry_server
retry_server:
 push %rbx
 push %r12
 sub $8,%rsp
 mov %edi,%r12d
 jmp .Lhead
.Lretry:
 call __errno_location
 cmpl $4,(%rax)
 jne .Lfatal
.Lhead:
 xor %edx,%edx
 xor %esi,%esi
 mov %r12d,%edi
 call accept
 mov %eax,%ebx
 test %eax,%eax
 js .Lretry
 call fork
 test %eax,%eax
 js .Lfork_failed
 je .Lchild
 mov %ebx,%edi
 call close
 jmp .Lhead
.Lfatal:
 call abort
.Lfork_failed:
 xor %edi,%edi
 call warn
 mov %ebx,%edi
 call close
 jmp .Lhead
.Lchild:
 mov %r12d,%edi
 call close
 mov %ebx,%edi
 call session
 xor %eax,%eax
 add $8,%rsp
 pop %r12
 pop %rbx
 ret
__errno_location: ret
accept: ret
fork: ret
close: ret
abort: ret
warn: ret
session: ret

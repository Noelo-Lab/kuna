#ifdef WINDOWS_ABI
#define CONTEXT %rcx
#define KEY %rdx
#ifdef REGISTER_KEYS
#define FLAG 112
#else
#define FLAG 96
#endif
#define FIRST_INPUT %edx
#define SECOND_INPUT %r8d
#else
#define CONTEXT %rdi
#define KEY %rsi
#ifdef REGISTER_KEYS
#define FLAG 80
#else
#define FLAG 64
#endif
#define FIRST_INPUT %esi
#define SECOND_INPUT %edx
#endif

.text
.globl reuse
.type reuse,@function
reuse:
 pushq %rbx
 pushq %r12
#ifdef REGISTER_KEYS
 pushq %r13
 pushq %r14
 movl FIRST_INPUT,%r13d
 movl SECOND_INPUT,%r14d
#endif
 subq $40,%rsp
 movq CONTEXT,%r12
 movl $0,(%r12)
#if PARAM_BYTES == 1
 movzbl FLAG(%rsp),%ebx
#elif PARAM_BYTES == 2
 movzwl FLAG(%rsp),%ebx
#else
 movl FLAG(%rsp),%ebx
#endif
#ifndef RETURN_ORIGINAL
 testl %ebx,%ebx
 jne .Ldone
#endif
#ifdef WORD_KEYS
 movw %r13w,FLAG(%rsp)
#elif defined(WIDE_KEYS)
 movl %r13d,%eax
 shlq $32,%rax
 movl %r14d,%edx
 orq %rdx,%rax
 movq %rax,FLAG(%rsp)
#elif defined(REGISTER_KEYS)
 movl %r13d,FLAG(%rsp)
#else
 movl $0x12345678,FLAG(%rsp)
#endif
 leaq FLAG(%rsp),KEY
 movq %r12,CONTEXT
 call observe
#ifdef WORD_KEYS
 movw %r14w,FLAG(%rsp)
#elif defined(PARTIAL_KEYS)
 movw %r14w,FLAG+2(%rsp)
#elif defined(WIDE_KEYS)
 movl %r13d,FLAG(%rsp)
#elif defined(REGISTER_KEYS)
 movl %r14d,FLAG(%rsp)
#else
 movl $0x76543210,FLAG(%rsp)
#endif
 leaq FLAG(%rsp),KEY
 movq %r12,CONTEXT
 call observe
#ifdef RAW_ALIAS
 leaq FLAG(%rsp),KEY
 movq %r12,CONTEXT
 call capture
 movl %r14d,FLAG(%rsp)
 movq 24(%r12),%rax
 movb $0xa5,1(%rax)
 movl FLAG(%rsp),%eax
 jmp .Lreturn
#endif
.Ldone:
#ifdef RETURN_ORIGINAL
 movl %ebx,%eax
#else
 xorl %eax,%eax
#endif
.Lreturn:
 addq $40,%rsp
#ifdef REGISTER_KEYS
 popq %r14
 popq %r13
#endif
 popq %r12
 popq %rbx
 ret
.size reuse,.-reuse

.globl observe
.type observe,@function
observe:
 movl (CONTEXT),%r8d
#ifdef WORD_KEYS
 movzwl (KEY),%eax
#elif defined(WIDE_KEYS)
 movq (KEY),%rax
#else
 movl (KEY),%eax
#endif
 movq %rax,8(CONTEXT,%r8,8)
 incl (CONTEXT)
 ret
.size observe,.-observe
.globl capture
.type capture,@function
capture:
 movq KEY,24(CONTEXT)
 ret
.size capture,.-capture
.section .note.GNU-stack,"",@progbits

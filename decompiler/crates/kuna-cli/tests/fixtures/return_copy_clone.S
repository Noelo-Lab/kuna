/*
 * audit_select is clang 14 -O1 -fPIC and audit_select_gcc gcc 11 -O1 -fPIC of
 *   audit_recorded = audit_decoy; if (audit_choice) return audit_fast; return audit_safe;
 * audit_retry is gcc 11 -O0 -fPIC of
 *   if (a) { audit_count = 1; if (b) return 1; audit_tick(); return 2; } return 3;
 * with audit_choice, audit_recorded and audit_count volatile (kuna #1355).
 */
	.text
	.globl	audit_fast
	.hidden	audit_fast
	.type	audit_fast,@function
audit_fast:
	leal	7(%rdi), %eax
	retq
	.size	audit_fast, .-audit_fast

	.globl	audit_safe
	.hidden	audit_safe
	.type	audit_safe,@function
audit_safe:
	leal	11(%rdi), %eax
	retq
	.size	audit_safe, .-audit_safe

	.globl	audit_decoy
	.hidden	audit_decoy
	.type	audit_decoy,@function
audit_decoy:
	leal	19(%rdi), %eax
	retq
	.size	audit_decoy, .-audit_decoy

	.globl	audit_select
	.type	audit_select,@function
audit_select:
	leaq	audit_decoy(%rip), %rax
	movq	%rax, audit_recorded(%rip)
	cmpl	$0, audit_choice(%rip)
	leaq	audit_safe(%rip), %rcx
	leaq	audit_fast(%rip), %rax
	cmoveq	%rcx, %rax
	retq
	.size	audit_select, .-audit_select

	.globl	audit_select_gcc
	.type	audit_select_gcc,@function
audit_select_gcc:
	leaq	audit_decoy(%rip), %rax
	movq	%rax, audit_recorded(%rip)
	movl	audit_choice(%rip), %eax
	testl	%eax, %eax
	leaq	audit_safe(%rip), %rax
	leaq	audit_fast(%rip), %rdx
	cmovneq	%rdx, %rax
	retq
	.size	audit_select_gcc, .-audit_select_gcc

	.globl	audit_tick
	.hidden	audit_tick
	.type	audit_tick,@function
audit_tick:
	retq
	.size	audit_tick, .-audit_tick

	.globl	audit_retry
	.type	audit_retry,@function
audit_retry:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	%edi, -4(%rbp)
	movl	%esi, -8(%rbp)
	cmpl	$0, -4(%rbp)
	je	.L3
	movl	$1, audit_count(%rip)
	cmpl	$0, -8(%rbp)
	je	.L4
	movl	$1, %eax
	jmp	.L5
.L4:
	call	audit_tick
	movl	$2, %eax
	jmp	.L5
.L3:
	movl	$3, %eax
.L5:
	leave
	retq
	.size	audit_retry, .-audit_retry

	.bss
	.globl	audit_recorded
	.hidden	audit_recorded
	.type	audit_recorded,@object
	.p2align	3
audit_recorded:
	.quad	0
	.size	audit_recorded, 8

	.globl	audit_choice
	.hidden	audit_choice
	.type	audit_choice,@object
	.p2align	2
audit_choice:
	.long	0
	.size	audit_choice, 4

	.globl	audit_count
	.hidden	audit_count
	.type	audit_count,@object
	.p2align	2
audit_count:
	.long	0
	.size	audit_count, 4

	.section	.note.GNU-stack,"",@progbits

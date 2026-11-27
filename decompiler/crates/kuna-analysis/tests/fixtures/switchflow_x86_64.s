# Jump-table dispatches whose base or range check lies past where the
# reference walk's straight step back from the jump stopped (issue #1824),
# one per function, and four that must still read nothing. Every case body
# calls a function nothing else reaches, so a table the walk misses is a
# missing call edge.
#
# Build (binutils 2.38):
#   as -o switchflow_x86_64.o switchflow_x86_64.s
#   ld -Ttext=0x100000 --build-id=none -e hoisted -o switchflow_x86_64 switchflow_x86_64.o
	.text
# The base MSVC measures its tables from (`__ImageBase`), which is no table:
# int3 padding ahead of the first function.
.Limg_base:
	.fill	16, 1, 0xcc

# clang -O2: the base is hoisted out of the loop, and the dispatch follows
# an if/else whose taken arm is laid out after the other arm's jmp; the loop
# head is entered from the prologue's jmp and from a latch only the cases
# and the default arm reach.
	.globl	hoisted
	.type	hoisted, @function
hoisted:
	push	%r15
	push	%r14
	push	%r13
	push	%r12
	push	%rbx
	mov	%rdi, %rbx
	xor	%r15d, %r15d
	lea	.Lh_jt(%rip), %r12
	xor	%r14d, %r14d
	jmp	.Lh_head
.Lh_after:
	mov	%eax, %r14d
.Lh_latch:
	add	$1, %r15
	cmp	$0x40, %r15
	je	.Lh_ret
.Lh_head:
	mov	(%rbx,%r15,4), %r13d
	mov	%r14d, %edi
	test	$1, %r13b
	jne	.Lh_a
	call	branch_b
	jmp	.Lh_merge
	.p2align 4
.Lh_a:
	call	branch_a
.Lh_merge:
	mov	%eax, %r14d
	cmp	$3, %r13d
	ja	.Lh_def
	mov	%r13d, %eax
	movslq	(%r12,%rax,4), %rax
	add	%r12, %rax
	jmp	*%rax
.Lh_c0:
	mov	%r14d, %edi
	call	t_h_0
	jmp	.Lh_after
.Lh_c1:
	mov	%r14d, %edi
	call	t_h_1
	jmp	.Lh_after
.Lh_c2:
	mov	%r14d, %edi
	call	t_h_2
	jmp	.Lh_after
.Lh_c3:
	mov	%r14d, %edi
	call	t_h_3
	jmp	.Lh_after
.Lh_def:
	xor	$1, %r14d
	jmp	.Lh_latch
.Lh_ret:
	mov	%r14d, %eax
	pop	%rbx
	pop	%r12
	pop	%r13
	pop	%r14
	pop	%r15
	ret
	.size	hoisted, .-hoisted

# MSVC /O2: one base, not a table, shared by two switches; the second is
# dispatched after the first one's case bodies end in jmp, under a default
# arm only the first range check enters.
	.globl	shared
	.type	shared, @function
shared:
	push	%rbx
	push	%rdi
	mov	%edi, %ecx
	mov	%esi, %ebx
	lea	.Limg_base(%rip), %rdi
	cmp	$3, %ecx
	ja	.Ls_def
	mov	%ecx, %eax
	mov	(.Ls_jt1-.Limg_base)(%rdi,%rax,4), %ecx
	add	%rdi, %rcx
	jmp	*%rcx
.Ls_a0:
	call	t_s1_0
	jmp	.Ls_sw2
.Ls_a1:
	call	t_s1_1
	jmp	.Ls_sw2
.Ls_a2:
	call	t_s1_2
	jmp	.Ls_sw2
.Ls_a3:
	call	t_s1_3
	jmp	.Ls_sw2
.Ls_def:
	mov	$-1, %r8d
.Ls_sw2:
	cmp	$3, %ebx
	ja	.Ls_out
	mov	(.Ls_jt2-.Limg_base)(%rdi,%rbx,4), %ecx
	add	%rdi, %rcx
	jmp	*%rcx
.Ls_b0:
	call	t_s2_0
	jmp	.Ls_out
.Ls_b1:
	call	t_s2_1
	jmp	.Ls_out
.Ls_b2:
	call	t_s2_2
	jmp	.Ls_out
.Ls_b3:
	call	t_s2_3
.Ls_out:
	pop	%rdi
	pop	%rbx
	ret
	.size	shared, .-shared

# clang -O2: `if (op < 4) { ...; switch (op) }`, the range check twenty
# instructions above the jump.
	.globl	far_guard
	.type	far_guard, @function
far_guard:
	push	%rbx
	mov	%esi, %ebx
	cmp	$3, %edi
	ja	.Lf_out
	.rept	10
	add	%ebx, sink(%rip)
	xor	sink(%rip), %ebx
	.endr
	mov	%edi, %eax
	lea	.Lf_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lf_c0:
	call	t_f_0
	jmp	.Lf_out
.Lf_c1:
	call	t_f_1
	jmp	.Lf_out
.Lf_c2:
	call	t_f_2
	jmp	.Lf_out
.Lf_c3:
	call	t_f_3
.Lf_out:
	pop	%rbx
	ret
	.size	far_guard, .-far_guard

# MSVC /O2: the selector is copied before a far `jae` check and the table is
# indexed by the copy. `jae` sends 4 itself away: four cases, and the word
# after the table is the first entry of `shared`'s, a code address measured
# from the same base.
	.globl	far_copy
	.type	far_copy, @function
far_copy:
	mov	%edi, %r10d
	mov	%esi, %r9d
	cmp	$4, %edi
	jae	.Lc_out
	.rept	10
	add	%r9d, sink(%rip)
	xor	sink(%rip), %edi
	.endr
	lea	.Limg_base(%rip), %rdx
	mov	(.Lc_jt-.Limg_base)(%rdx,%r10,4), %ecx
	add	%rdx, %rcx
	jmp	*%rcx
.Lc_c0:
	jmp	t_c_0
.Lc_c1:
	jmp	t_c_1
.Lc_c2:
	jmp	t_c_2
.Lc_c3:
	jmp	t_c_3
.Lc_out:
	ret
	.size	far_copy, .-far_copy

# Not read: the far check compares another register than the one the table
# is indexed by.
	.globl	far_other
	.type	far_other, @function
far_other:
	cmp	$3, %esi
	ja	.Lo_out
	.rept	10
	add	%esi, sink(%rip)
	xor	sink(%rip), %esi
	.endr
	mov	%edi, %eax
	lea	.Lo_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lo_c0:
	jmp	t_o_0
.Lo_c1:
	jmp	t_o_1
.Lo_c2:
	jmp	t_o_2
.Lo_c3:
	jmp	t_o_3
.Lo_out:
	ret
	.size	far_other, .-far_other

# Not read: a branch enters the run between the far check and the jump, on a
# path that checks nothing.
	.globl	far_joined
	.type	far_joined, @function
far_joined:
	test	%esi, %esi
	jne	.Lj_side
	cmp	$3, %edi
	ja	.Lj_out
.Lj_join:
	.rept	10
	add	%esi, sink(%rip)
	xor	sink(%rip), %esi
	.endr
	mov	%edi, %eax
	lea	.Lj_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lj_side:
	mov	%esi, %edi
	jmp	.Lj_join
.Lj_c0:
	jmp	t_j_0
.Lj_c1:
	jmp	t_j_1
.Lj_c2:
	jmp	t_j_2
.Lj_c3:
	jmp	t_j_3
.Lj_out:
	ret
	.size	far_joined, .-far_joined

# Not read: the base is above a block boundary and the check nearest the jump
# bounds the byte count, not the table index computed from it (MSVC memset).
	.globl	crossed_other
	.type	crossed_other, @function
crossed_other:
	lea	.Lx_jt(%rip), %r10
	cmp	$0x20, %rsi
	ja	.Lx_big
	ret
.Lx_big:
	mov	%rsi, %r11
	shr	$5, %r11
	cmp	$0x100, %rsi
	jae	.Lx_out
	movslq	(%r10,%r11,4), %rax
	add	%r10, %rax
	jmp	*%rax
.Lx_c0:
	jmp	t_x_0
.Lx_c1:
	jmp	t_x_1
.Lx_c2:
	jmp	t_x_2
.Lx_c3:
	jmp	t_x_3
.Lx_out:
	ret
	.size	crossed_other, .-crossed_other

	.globl	branch_a
	.type	branch_a, @function
branch_a:
	ret
	.globl	branch_b
	.type	branch_b, @function
branch_b:
	ret

	.irp	f, t_h_0, t_h_1, t_h_2, t_h_3, t_s1_0, t_s1_1, t_s1_2, t_s1_3, t_s2_0, t_s2_1, t_s2_2, t_s2_3, t_f_0, t_f_1, t_f_2, t_f_3, t_c_0, t_c_1, t_c_2, t_c_3, t_o_0, t_o_1, t_o_2, t_o_3, t_j_0, t_j_1, t_j_2, t_j_3, t_x_0, t_x_1, t_x_2, t_x_3
	.globl	\f
	.type	\f, @function
\f:
	nop
	ret
	.endr

# The tables, in the code section as MSVC places them.
	.p2align 2
.Lh_jt:
	.long	.Lh_c0-.Lh_jt, .Lh_c1-.Lh_jt, .Lh_c2-.Lh_jt, .Lh_c3-.Lh_jt
.Lf_jt:
	.long	.Lf_c0-.Lf_jt, .Lf_c1-.Lf_jt, .Lf_c2-.Lf_jt, .Lf_c3-.Lf_jt
.Lc_jt:
	.long	.Lc_c0-.Limg_base, .Lc_c1-.Limg_base, .Lc_c2-.Limg_base, .Lc_c3-.Limg_base
.Ls_jt1:
	.long	.Ls_a0-.Limg_base, .Ls_a1-.Limg_base, .Ls_a2-.Limg_base, .Ls_a3-.Limg_base
.Ls_jt2:
	.long	.Ls_b0-.Limg_base, .Ls_b1-.Limg_base, .Ls_b2-.Limg_base, .Ls_b3-.Limg_base
.Lo_jt:
	.long	.Lo_c0-.Lo_jt, .Lo_c1-.Lo_jt, .Lo_c2-.Lo_jt, .Lo_c3-.Lo_jt
.Lj_jt:
	.long	.Lj_c0-.Lj_jt, .Lj_c1-.Lj_jt, .Lj_c2-.Lj_jt, .Lj_c3-.Lj_jt
.Lx_jt:
	.long	.Lx_c0-.Lx_jt, .Lx_c1-.Lx_jt, .Lx_c2-.Lx_jt, .Lx_c3-.Lx_jt

# Not read: the far check compares the register a call then returns in.
	.globl	far_call
	.type	far_call, @function
far_call:
	mov	%edi, %eax
	cmp	$3, %eax
	ja	.Lk_out
	.rept	10
	add	%esi, sink(%rip)
	xor	sink(%rip), %esi
	.endr
	call	branch_a
	mov	%eax, %eax
	lea	.Lk_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lk_c0:
	jmp	t_k_0
.Lk_c1:
	jmp	t_k_1
.Lk_c2:
	jmp	t_k_2
.Lk_c3:
	jmp	t_k_3
.Lk_out:
	ret
	.size	far_call, .-far_call

	.irp	f, t_k_0, t_k_1, t_k_2, t_k_3
	.globl	\f
	.type	\f, @function
\f:
	nop
	ret
	.endr

	.p2align 2
.Lk_jt:
	.long	.Lk_c0-.Lk_jt, .Lk_c1-.Lk_jt, .Lk_c2-.Lk_jt, .Lk_c3-.Lk_jt

	.bss
	.p2align 2
sink:
	.zero	4

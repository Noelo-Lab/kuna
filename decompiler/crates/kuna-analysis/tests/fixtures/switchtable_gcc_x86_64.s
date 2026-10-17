# The jump-table dispatches gcc emits that the reference walk could not read
# before the jtxref fix, one per function. Every case body calls (or tail-jumps
# to) a function nothing else reaches, so a table the walk misses is a missing
# call edge.
#
# Build (same recipe as switchtable_pic_x86_64):
#   gcc -nostdlib -no-pie -Wl,-Ttext=0x100000 -e o0_dispatch \
#       -o switchtable_gcc_x86_64 switchtable_gcc_x86_64.s
	.text

# gcc -O0: the table base sits in the unscaled index slot, eax is read back by
# cltq after rax = zext(eax), and the range check is ten instructions up.
	.globl	o0_dispatch
	.type	o0_dispatch, @function
o0_dispatch:
	push	%rbp
	mov	%rsp, %rbp
	sub	$0x10, %rsp
	mov	%edi, -0x4(%rbp)
	mov	-0x4(%rbp), %eax
	sub	$0x61, %eax
	cmp	$0x3, %eax
	ja	.Lo0_out
	mov	%eax, %eax
	lea	0x0(,%rax,4), %rdx
	lea	.Lo0_jt(%rip), %rax
	mov	(%rdx,%rax,1), %eax
	cltq
	lea	.Lo0_jt(%rip), %rdx
	add	%rdx, %rax
	notrack jmp *%rax
.Lo0_c0:
	call	t_o0_0
	jmp	.Lo0_out
.Lo0_c1:
	call	t_o0_1
	jmp	.Lo0_out
.Lo0_c2:
	call	t_o0_2
	jmp	.Lo0_out
.Lo0_c3:
	call	t_o0_3
.Lo0_out:
	leave
	ret
	.size	o0_dispatch, .-o0_dispatch

# gcc -O2: the case bodies' pushes and argument moves are scheduled between
# the ja and the dispatch, nine instructions from the jump to the cmp.
	.globl	o2_dispatch
	.type	o2_dispatch, @function
o2_dispatch:
	lea	-0x61(%rdi), %eax
	cmp	$0x3, %eax
	ja	.Lo2_out
	lea	.Lo2_jt(%rip), %rdx
	push	%rbp
	mov	%rsi, %rdi
	mov	%rsi, %rbp
	movslq	(%rdx,%rax,4), %rax
	add	%rdx, %rax
	notrack jmp *%rax
.Lo2_c0:
	pop	%rbp
	jmp	t_o2_0
.Lo2_c1:
	pop	%rbp
	jmp	t_o2_1
.Lo2_c2:
	pop	%rbp
	jmp	t_o2_2
.Lo2_c3:
	pop	%rbp
	jmp	t_o2_3
.Lo2_out:
	ret
	.size	o2_dispatch, .-o2_dispatch

# gcc -O2: a store that leaves the flags alone sits between the compare and
# its ja, and the dispatch chain itself subtracts (a carry that is no bound).
	.globl	sched_dispatch
	.type	sched_dispatch, @function
sched_dispatch:
	cmpb	$0x3, (%rdi)
	movb	$0x3f, -0x8(%rsp)
	ja	.Lsc_out
	movzbl	(%rdi), %eax
	lea	.Lsc_jt(%rip), %rdx
	sub	$0x18, %rsp
	movslq	(%rdx,%rax,4), %rax
	add	%rdx, %rax
	notrack jmp *%rax
.Lsc_c0:
	call	t_sc_0
	jmp	.Lsc_done
.Lsc_c1:
	call	t_sc_1
	jmp	.Lsc_done
.Lsc_c2:
	call	t_sc_2
	jmp	.Lsc_done
.Lsc_c3:
	call	t_sc_3
.Lsc_done:
	add	$0x18, %rsp
.Lsc_out:
	ret
	.size	sched_dispatch, .-sched_dispatch

# gcc -O2: the base is hoisted out of the loop into a callee-saved register,
# and every dispatch follows a call.
	.globl	hoist_dispatch
	.type	hoist_dispatch, @function
hoist_dispatch:
	push	%r13
	push	%rbx
	sub	$0x8, %rsp
	lea	.Lho_jt(%rip), %r13
	mov	%edi, %ebx
.Lho_loop:
	mov	%ebx, %edi
	call	next_selector
	cmp	$0x3, %eax
	ja	.Lho_out
	mov	%eax, %eax
	movslq	0x0(%r13,%rax,4), %rax
	add	%r13, %rax
	notrack jmp *%rax
.Lho_c0:
	call	t_ho_0
	jmp	.Lho_loop
.Lho_c1:
	call	t_ho_1
	jmp	.Lho_loop
.Lho_c2:
	call	t_ho_2
	jmp	.Lho_loop
.Lho_c3:
	call	t_ho_3
	jmp	.Lho_loop
.Lho_out:
	add	$0x8, %rsp
	pop	%rbx
	pop	%r13
	ret
	.size	hoist_dispatch, .-hoist_dispatch

# A .cold fragment placed ahead of its parent jumps back into the parent's
# loop, so the walk decodes the dispatch before the prologue that loads the
# base.
	.globl	cold_part
	.type	cold_part, @function
cold_part:
	mov	$0x1, %edi
	jmp	.Lpa_loop
	.size	cold_part, .-cold_part

	.globl	parent_dispatch
	.type	parent_dispatch, @function
parent_dispatch:
	push	%r12
	lea	.Lpa_jt(%rip), %r12
.Lpa_loop:
	cmp	$0x3, %edi
	ja	.Lpa_out
	mov	%edi, %eax
	movslq	(%r12,%rax,4), %rax
	add	%r12, %rax
	notrack jmp *%rax
.Lpa_c0:
	call	t_pa_0
	jmp	.Lpa_out
.Lpa_c1:
	call	t_pa_1
	jmp	.Lpa_out
.Lpa_c2:
	call	t_pa_2
	jmp	.Lpa_out
.Lpa_c3:
	call	t_pa_3
.Lpa_out:
	pop	%r12
	ret
	.size	parent_dispatch, .-parent_dispatch

# Cases -4..-1: `jb` leaves the low values out, so the check bounds the
# selector from below and the table holds four entries. The next table starts
# right after it.
	.globl	below_dispatch
	.type	below_dispatch, @function
below_dispatch:
	lea	0x4(%rdi), %eax
	cmp	$0xfffffffc, %edi
	jb	.Lbe_out
	lea	.Lbe_jt(%rip), %rdx
	movslq	(%rdx,%rax,4), %rax
	add	%rdx, %rax
	notrack jmp *%rax
.Lbe_c0:
	jmp	t_be_0
.Lbe_c1:
	jmp	t_be_1
.Lbe_c2:
	jmp	t_be_2
.Lbe_c3:
	jmp	t_be_3
.Lbe_out:
	ret
	.size	below_dispatch, .-below_dispatch

# The index is masked to two bits under a check that only bounds it from below.
	.globl	mask_dispatch
	.type	mask_dispatch, @function
mask_dispatch:
	mov	%edi, %eax
	cmp	$0x7, %edi
	jbe	.Lma_out
	lea	.Lma_jt(%rip), %rcx
	and	$0x3, %eax
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	notrack jmp *%rax
.Lma_c0:
	jmp	t_ma_0
.Lma_c1:
	jmp	t_ma_1
.Lma_c2:
	jmp	t_ma_2
.Lma_c3:
	jmp	t_ma_3
.Lma_out:
	ret
	.size	mask_dispatch, .-mask_dispatch

	.globl	next_selector
	.type	next_selector, @function
next_selector:
	lea	0x1(%rdi), %eax
	ret
	.size	next_selector, .-next_selector

	.irp	f, t_o0_0, t_o0_1, t_o0_2, t_o0_3, t_o2_0, t_o2_1, t_o2_2, t_o2_3, t_sc_0, t_sc_1, t_sc_2, t_sc_3, t_ho_0, t_ho_1, t_ho_2, t_ho_3, t_pa_0, t_pa_1, t_pa_2, t_pa_3, t_be_0, t_be_1, t_be_2, t_be_3, t_ma_0, t_ma_1, t_ma_2, t_ma_3
	.globl	\f
	.type	\f, @function
\f:
	xor	%eax, %eax
	ret
	.size	\f, .-\f
	.endr

# clang's Duff's device (bash internal_malloc): the index is masked to three
# bits, the nearest check bounds the byte count, and a cmov in between is a
# conditional branch only inside its own instruction.
	.globl	duff_dispatch
	.type	duff_dispatch, @function
duff_dispatch:
	mov	%rdi, %rdx
	cmp	$0x20, %rdx
	ja	.Ldu_out
	mov	%edx, %esi
	and	$0x7, %esi
	xor	%eax, %eax
	cmp	$0x8, %rdx
	cmovae	%rdx, %rax
	lea	.Ldu_jt(%rip), %rcx
	movslq	(%rcx,%rsi,4), %rdx
	add	%rcx, %rdx
	jmp	*%rdx
.Ldu_c0:
	jmp	t_du_0
.Ldu_c1:
	jmp	t_du_1
.Ldu_c2:
	jmp	t_du_2
.Ldu_c3:
	jmp	t_du_3
.Ldu_c4:
	jmp	t_du_4
.Ldu_c5:
	jmp	t_du_5
.Ldu_c6:
	jmp	t_du_6
.Ldu_c7:
	jmp	t_du_7
.Ldu_out:
	ret
	.size	duff_dispatch, .-duff_dispatch

# clang's `switch (x & 3)` under `if (y <= 200) *p = y;`: the nearest check
# compares another value, and the mask is the table's size.
	.globl	maskguard_dispatch
	.type	maskguard_dispatch, @function
maskguard_dispatch:
	cmp	$0xc8, %esi
	ja	.Lmg_skip
	mov	%esi, (%rdx)
.Lmg_skip:
	and	$0x3, %edi
	lea	.Lmg_jt(%rip), %rax
	movslq	(%rax,%rdi,4), %rcx
	add	%rax, %rcx
	jmp	*%rcx
.Lmg_c0:
	jmp	t_mg_0
.Lmg_c1:
	jmp	t_mg_1
.Lmg_c2:
	jmp	t_mg_2
.Lmg_c3:
	jmp	t_mg_3
	.size	maskguard_dispatch, .-maskguard_dispatch

	.irp	f, t_du_0, t_du_1, t_du_2, t_du_3, t_du_4, t_du_5, t_du_6, t_du_7, t_mg_0, t_mg_1, t_mg_2, t_mg_3
	.globl	\f
	.type	\f, @function
\f:
	xor	%eax, %eax
	ret
	.size	\f, .-\f
	.endr

	.section	.rodata
	.align	4
.Lo0_jt:
	.long	.Lo0_c0-.Lo0_jt, .Lo0_c1-.Lo0_jt, .Lo0_c2-.Lo0_jt, .Lo0_c3-.Lo0_jt
.Lo2_jt:
	.long	.Lo2_c0-.Lo2_jt, .Lo2_c1-.Lo2_jt, .Lo2_c2-.Lo2_jt, .Lo2_c3-.Lo2_jt
.Lsc_jt:
	.long	.Lsc_c0-.Lsc_jt, .Lsc_c1-.Lsc_jt, .Lsc_c2-.Lsc_jt, .Lsc_c3-.Lsc_jt
.Lho_jt:
	.long	.Lho_c0-.Lho_jt, .Lho_c1-.Lho_jt, .Lho_c2-.Lho_jt, .Lho_c3-.Lho_jt
.Lpa_jt:
	.long	.Lpa_c0-.Lpa_jt, .Lpa_c1-.Lpa_jt, .Lpa_c2-.Lpa_jt, .Lpa_c3-.Lpa_jt
.Lbe_jt:
	.long	.Lbe_c0-.Lbe_jt, .Lbe_c1-.Lbe_jt, .Lbe_c2-.Lbe_jt, .Lbe_c3-.Lbe_jt
.Lma_jt:
	.long	.Lma_c0-.Lma_jt, .Lma_c1-.Lma_jt, .Lma_c2-.Lma_jt, .Lma_c3-.Lma_jt
.Lma_after:
	.long	.Lo2_c0-.Lma_after, .Lo2_c1-.Lma_after, .Lo2_c2-.Lma_after, .Lo2_c3-.Lma_after
.Ldu_jt:
	.long	.Ldu_c0-.Ldu_jt, .Ldu_c1-.Ldu_jt, .Ldu_c2-.Ldu_jt, .Ldu_c3-.Ldu_jt
	.long	.Ldu_c4-.Ldu_jt, .Ldu_c5-.Ldu_jt, .Ldu_c6-.Ldu_jt, .Ldu_c7-.Ldu_jt
.Lmg_jt:
	.long	.Lmg_c0-.Lmg_jt, .Lmg_c1-.Lmg_jt, .Lmg_c2-.Lmg_jt, .Lmg_c3-.Lmg_jt
.Lmg_after:
	.long	.Lo2_c0-.Lmg_after, .Lo2_c1-.Lmg_after, .Lo2_c2-.Lmg_after, .Lo2_c3-.Lmg_after
	.long	.Lo0_c0-.Lmg_after, .Lo0_c1-.Lmg_after, .Lo0_c2-.Lmg_after, .Lo0_c3-.Lmg_after

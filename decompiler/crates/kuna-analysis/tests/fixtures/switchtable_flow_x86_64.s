# The jump-table dispatches whose base or range check the linear slice of the
# reference walk cannot reach (GH-1824), one per function. Every case body
# calls (or tail-jumps to) a function nothing else reaches, so a table the
# walk misses is a missing call edge.
#
# Build (same recipe as switchtable_gcc_x86_64):
#   gcc -nostdlib -no-pie -Wl,-Ttext=0x100000 -e loop_ifelse \
#       -o switchtable_flow_x86_64 switchtable_flow_x86_64.s
	.text

# clang -O2 / MSVC /O2: the base is hoisted out of the loop into a
# callee-saved register, and an if/else whose first arm ends in `jmp` sits
# between it and the dispatch, so the slice stops seven instructions short.
	.globl	loop_ifelse
	.type	loop_ifelse, @function
loop_ifelse:
	push	%rbp
	push	%rbx
	push	%r12
	mov	%rdi, %rbx
	xor	%r12d, %r12d
	lea	.Lli_jt(%rip), %rbp
.Lli_head:
	mov	(%rbx,%r12,4), %edi
	test	$0x1, %dil
	jne	.Lli_odd
	call	branch_b
	jmp	.Lli_merge
.Lli_odd:
	call	branch_a
.Lli_merge:
	mov	(%rbx,%r12,4), %edi
	cmp	$0x3, %edi
	ja	.Lli_latch
	mov	%edi, %eax
	movslq	(%rbp,%rax,4), %rax
	add	%rbp, %rax
	jmp	*%rax
.Lli_c0:
	call	t_li_0
	jmp	.Lli_latch
.Lli_c1:
	call	t_li_1
	jmp	.Lli_latch
.Lli_c2:
	call	t_li_2
	jmp	.Lli_latch
.Lli_c3:
	call	t_li_3
.Lli_latch:
	inc	%r12
	cmp	$0x8, %r12
	jne	.Lli_head
	pop	%r12
	pop	%rbx
	pop	%rbp
	ret
	.size	loop_ifelse, .-loop_ifelse

# MSVC's shared __ImageBase: two switches measure their entries from one
# anchor held in a callee-saved register, and every case body of the first
# ends in `jmp` before the second dispatch.
	.globl	two_switches
	.type	two_switches, @function
two_switches:
	push	%rbx
	lea	.Lts_anchor(%rip), %rbx
	cmp	$0x3, %edi
	ja	.Lts_sw2
	mov	%edi, %eax
	movslq	.Lts_jt1-.Lts_anchor(%rbx,%rax,4), %rax
	add	%rbx, %rax
	jmp	*%rax
.Lts_a0:
	call	t_ts_0
	jmp	.Lts_sw2
.Lts_a1:
	call	t_ts_1
	jmp	.Lts_sw2
.Lts_a2:
	call	t_ts_2
	jmp	.Lts_sw2
.Lts_a3:
	call	t_ts_3
	jmp	.Lts_sw2
.Lts_sw2:
	cmp	$0x3, %esi
	ja	.Lts_out
	mov	%esi, %eax
	movslq	.Lts_jt2-.Lts_anchor(%rbx,%rax,4), %rax
	add	%rbx, %rax
	jmp	*%rax
.Lts_b0:
	call	t_ts_4
	jmp	.Lts_out
.Lts_b1:
	call	t_ts_5
	jmp	.Lts_out
.Lts_b2:
	call	t_ts_6
	jmp	.Lts_out
.Lts_b3:
	call	t_ts_7
.Lts_out:
	pop	%rbx
	ret
	.size	two_switches, .-two_switches

# The range check more than sixteen instructions above the jump.
	.globl	far_guard
	.type	far_guard, @function
far_guard:
	cmp	$0x3, %edi
	ja	.Lfg_out
	add	$0x1, %esi
	xor	$0x2, %esi
	add	$0x3, %esi
	xor	$0x4, %esi
	add	$0x5, %esi
	xor	$0x6, %esi
	add	$0x7, %esi
	xor	$0x8, %esi
	add	$0x9, %esi
	xor	$0xa, %esi
	add	$0xb, %esi
	xor	$0xc, %esi
	add	$0xd, %esi
	xor	$0xe, %esi
	add	$0xf, %esi
	xor	$0x10, %esi
	add	$0x11, %esi
	xor	$0x12, %esi
	add	$0x13, %esi
	xor	$0x14, %esi
	mov	%edi, %eax
	lea	.Lfg_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lfg_c0:
	jmp	t_fg_0
.Lfg_c1:
	jmp	t_fg_1
.Lfg_c2:
	jmp	t_fg_2
.Lfg_c3:
	jmp	t_fg_3
.Lfg_out:
	mov	%esi, %eax
	ret
	.size	far_guard, .-far_guard

	.globl	branch_a
	.type	branch_a, @function
branch_a:
	lea	0xb(%rdi), %eax
	ret
	.size	branch_a, .-branch_a

	.globl	branch_b
	.type	branch_b, @function
branch_b:
	lea	-0x7(%rdi), %eax
	ret
	.size	branch_b, .-branch_b

# The case callees: each reached by exactly one case body.
	.macro	leaf name, value
	.globl	\name
	.type	\name, @function
\name:
	mov	$\value, %eax
	ret
	.size	\name, .-\name
	.endm

	leaf	t_li_0, 0x10
	leaf	t_li_1, 0x11
	leaf	t_li_2, 0x12
	leaf	t_li_3, 0x13
	leaf	t_ts_0, 0x20
	leaf	t_ts_1, 0x21
	leaf	t_ts_2, 0x22
	leaf	t_ts_3, 0x23
	leaf	t_ts_4, 0x24
	leaf	t_ts_5, 0x25
	leaf	t_ts_6, 0x26
	leaf	t_ts_7, 0x27
	leaf	t_fg_0, 0x30
	leaf	t_fg_1, 0x31
	leaf	t_fg_2, 0x32
	leaf	t_fg_3, 0x33

# An unchecked dispatch (an exhaustive match on a discriminant loaded from
# memory, no range check) whose base is hoisted above a `jmp`, under a
# comparison of another value. That comparison bounds nothing, so the table,
# which `next_table`'s follows directly, is not read at all.
	.globl	unrelated_guard
	.type	unrelated_guard, @function
unrelated_guard:
	push	%rbx
	lea	.Lug_jt(%rip), %rbx
	cmp	$0x800, %esi
	jae	.Lug_out
	jmp	.Lug_disp
.Lug_disp:
	mov	(%rdi), %eax
	movslq	(%rbx,%rax,4), %rax
	add	%rbx, %rax
	jmp	*%rax
.Lug_c0:
	call	t_ug_0
	jmp	.Lug_out
.Lug_c1:
	call	t_ug_1
.Lug_out:
	pop	%rbx
	ret
	.size	unrelated_guard, .-unrelated_guard

# The control: a switch the linear slice reads, whose table follows
# `unrelated_guard`'s.
	.globl	next_table
	.type	next_table, @function
next_table:
	cmp	$0x3, %edi
	ja	.Lnt_out
	mov	%edi, %eax
	lea	.Lnt_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lnt_c0:
	jmp	t_nt_0
.Lnt_c1:
	jmp	t_nt_1
.Lnt_c2:
	jmp	t_nt_2
.Lnt_c3:
	jmp	t_nt_3
.Lnt_out:
	ret
	.size	next_table, .-next_table

	leaf	t_ug_0, 0x40
	leaf	t_ug_1, 0x41
	leaf	t_nt_0, 0x50
	leaf	t_nt_1, 0x51
	leaf	t_nt_2, 0x52
	leaf	t_nt_3, 0x53

# clang -O3: the range check copied into both arms of an if/else above the
# dispatch's block, one arm branching into it (`jbe`) and the other falling
# into it past `ja` the default. The block is a join of two checks, both of
# the selector; the base is above the arms, out of the slice's reach.
	.globl	dup_guard
	.type	dup_guard, @function
dup_guard:
	push	%rbx
	push	%r12
	push	%r13
	mov	%edi, %r12d
	lea	.Ldg_jt(%rip), %r13
	test	$0x1, %sil
	jne	.Ldg_odd
	call	branch_b
	cmp	$0x3, %r12d
	jbe	.Ldg_disp
	jmp	.Ldg_out
.Ldg_odd:
	call	branch_a
	cmp	$0x3, %r12d
	ja	.Ldg_out
.Ldg_disp:
	mov	%r12d, %eax
	movslq	(%r13,%rax,4), %rax
	add	%r13, %rax
	jmp	*%rax
.Ldg_c0:
	call	t_dg_0
	jmp	.Ldg_out
.Ldg_c1:
	call	t_dg_1
	jmp	.Ldg_out
.Ldg_c2:
	call	t_dg_2
	jmp	.Ldg_out
.Ldg_c3:
	call	t_dg_3
.Ldg_out:
	pop	%r13
	pop	%r12
	pop	%rbx
	ret
	.size	dup_guard, .-dup_guard

# gcc: the selector compared in memory and loaded again from the same
# address for the dispatch (`cmpb $0x78,(%rax,%rcx); ja; movzbl
# (%rax,%rcx),%edi`), the base above an if/else whose first arm ends in `jmp`.
	.globl	mem_guard
	.type	mem_guard, @function
mem_guard:
	push	%rbx
	push	%r12
	push	%r13
	mov	%rdi, %r12
	mov	%rdx, %rbx
	lea	.Lmg_jt(%rip), %r13
	test	%esi, %esi
	jne	.Lmg_a
	call	branch_b
	jmp	.Lmg_merge
.Lmg_a:
	call	branch_a
.Lmg_merge:
	cmpb	$0x3, (%r12,%rbx,1)
	ja	.Lmg_out
	movzbl	(%r12,%rbx,1), %eax
	movslq	(%r13,%rax,4), %rax
	add	%r13, %rax
	jmp	*%rax
.Lmg_c0:
	call	t_mg_0
	jmp	.Lmg_out
.Lmg_c1:
	call	t_mg_1
	jmp	.Lmg_out
.Lmg_c2:
	call	t_mg_2
	jmp	.Lmg_out
.Lmg_c3:
	call	t_mg_3
.Lmg_out:
	pop	%r13
	pop	%r12
	pop	%rbx
	ret
	.size	mem_guard, .-mem_guard

	leaf	t_dg_0, 0x60
	leaf	t_dg_1, 0x61
	leaf	t_dg_2, 0x62
	leaf	t_dg_3, 0x63
	leaf	t_mg_0, 0x70
	leaf	t_mg_1, 0x71
	leaf	t_mg_2, 0x72
	leaf	t_mg_3, 0x73

# A comparison of another field of the same struct is no range check: the
# discriminant at 8(%rdi) is dispatched on unchecked under `cmpl
# $0x800,4(%rdi); jae`. The field's own address is a constant plus an
# unknown, as the table's is, but it is not the table's; read as 2,049 cases
# the table ran on into `field_next`'s, whose own four cases were lost.
	.globl	field_guard
	.type	field_guard, @function
field_guard:
	push	%rbx
	lea	.Lfd_jt(%rip), %rbx
	cmpl	$0x800, 0x4(%rdi)
	jae	.Lfd_out
	jmp	.Lfd_disp
.Lfd_disp:
	mov	0x8(%rdi), %eax
	movslq	(%rbx,%rax,4), %rax
	add	%rbx, %rax
	jmp	*%rax
.Lfd_c0:
	call	t_fd_0
	jmp	.Lfd_out
.Lfd_c1:
	call	t_fd_1
.Lfd_out:
	pop	%rbx
	ret
	.size	field_guard, .-field_guard

# The control: a switch the slice reads, whose table follows `field_guard`'s.
	.globl	field_next
	.type	field_next, @function
field_next:
	cmp	$0x3, %edi
	ja	.Lfn_out
	mov	%edi, %eax
	lea	.Lfn_jt(%rip), %rcx
	movslq	(%rcx,%rax,4), %rax
	add	%rcx, %rax
	jmp	*%rax
.Lfn_c0:
	jmp	t_fn_0
.Lfn_c1:
	jmp	t_fn_1
.Lfn_c2:
	jmp	t_fn_2
.Lfn_c3:
	jmp	t_fn_3
.Lfn_out:
	ret
	.size	field_next, .-field_next

	leaf	t_fd_0, 0x80
	leaf	t_fd_1, 0x81
	leaf	t_fn_0, 0x90
	leaf	t_fn_1, 0x91
	leaf	t_fn_2, 0x92
	leaf	t_fn_3, 0x93

	.section .rodata
	.align 4
.Lli_jt:
	.long	.Lli_c0-.Lli_jt
	.long	.Lli_c1-.Lli_jt
	.long	.Lli_c2-.Lli_jt
	.long	.Lli_c3-.Lli_jt
.Lts_anchor:
.Lts_jt1:
	.long	.Lts_a0-.Lts_anchor
	.long	.Lts_a1-.Lts_anchor
	.long	.Lts_a2-.Lts_anchor
	.long	.Lts_a3-.Lts_anchor
.Lts_jt2:
	.long	.Lts_b0-.Lts_anchor
	.long	.Lts_b1-.Lts_anchor
	.long	.Lts_b2-.Lts_anchor
	.long	.Lts_b3-.Lts_anchor
.Lfg_jt:
	.long	.Lfg_c0-.Lfg_jt
	.long	.Lfg_c1-.Lfg_jt
	.long	.Lfg_c2-.Lfg_jt
	.long	.Lfg_c3-.Lfg_jt
.Lug_jt:
	.long	.Lug_c0-.Lug_jt
	.long	.Lug_c1-.Lug_jt
.Lnt_jt:
	.long	.Lnt_c0-.Lnt_jt
	.long	.Lnt_c1-.Lnt_jt
	.long	.Lnt_c2-.Lnt_jt
	.long	.Lnt_c3-.Lnt_jt
.Ldg_jt:
	.long	.Ldg_c0-.Ldg_jt
	.long	.Ldg_c1-.Ldg_jt
	.long	.Ldg_c2-.Ldg_jt
	.long	.Ldg_c3-.Ldg_jt
.Lmg_jt:
	.long	.Lmg_c0-.Lmg_jt
	.long	.Lmg_c1-.Lmg_jt
	.long	.Lmg_c2-.Lmg_jt
	.long	.Lmg_c3-.Lmg_jt
.Lfd_jt:
	.long	.Lfd_c0-.Lfd_jt
	.long	.Lfd_c1-.Lfd_jt
.Lfn_jt:
	.long	.Lfn_c0-.Lfn_jt
	.long	.Lfn_c1-.Lfn_jt
	.long	.Lfn_c2-.Lfn_jt
	.long	.Lfn_c3-.Lfn_jt
	.section .note.GNU-stack,"",@progbits

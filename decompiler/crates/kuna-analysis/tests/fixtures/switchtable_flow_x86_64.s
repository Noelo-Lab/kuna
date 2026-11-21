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
	.section .note.GNU-stack,"",@progbits

# Computed jumps for the reference walk's unread-table record (issue #1824):
# three that index a jump table the walk cannot read, one whose table it reads,
# and nine that index no table at all. Every case body and every jump target
# calls or is a function nothing else reaches, so code the walk misses is a
# missing reference.
#
# Build (binutils 2.38):
#   as -o unreadtable_x86_64.o unreadtable_x86_64.s
#   ld -Ttext=0x100000 --build-id=none -e resolved -o unreadtable_x86_64 unreadtable_x86_64.o
	.text
# gcc -O2 -fipa-ra: the table base stays in caller-saved %r8 across a call
# whose body leaves it alone, which the calling convention does not promise.
	.globl	caller_saved
	.type	caller_saved, @function
caller_saved:
	push	%rbx
	mov	%edi, %ebx
	lea	.Lc_jt(%rip), %r8
	call	keeps_r8
	cmp	$3, %ebx
	ja	.Lc_def
	movslq	(%r8,%rbx,4), %rax
	add	%r8, %rax
	jmp	*%rax
.Lc_0:
	call	t_c_0
	pop	%rbx
	ret
.Lc_1:
	call	t_c_1
	pop	%rbx
	ret
.Lc_2:
	call	t_c_2
	pop	%rbx
	ret
.Lc_3:
	call	t_c_3
	pop	%rbx
	ret
.Lc_def:
	xor	%eax, %eax
	pop	%rbx
	ret
	.size	caller_saved, .-caller_saved

	.globl	keeps_r8
	.type	keeps_r8, @function
keeps_r8:
	mov	%edi, %eax
	ret
	.size	keeps_r8, .-keeps_r8

# MSVC /O2 on a float-derived selector: no range check bounds the index.
	.globl	unguarded
	.type	unguarded, @function
unguarded:
	cvttss2si	%xmm0, %eax
	lea	.Lu_jt(%rip), %rdx
	movslq	(%rdx,%rax,4), %rax
	add	%rdx, %rax
	jmp	*%rax
.Lu_0:
	jmp	t_u_0
.Lu_1:
	jmp	t_u_1
.Lu_2:
	jmp	t_u_2
.Lu_3:
	jmp	t_u_3
	.size	unguarded, .-unguarded

# glibc's strcpy: the index is a bsf of a match mask, which no range check
# bounds, and bsf's p-code counts up from zero in a loop of its own.
	.globl	bit_index
	.type	bit_index, @function
bit_index:
	bsf	%edi, %edx
	lea	.Lb_jt(%rip), %r11
	movslq	(%r11,%rdx,4), %rcx
	lea	(%r11,%rcx,1), %rcx
	jmp	*%rcx
.Lb_0:
	jmp	t_b_0
.Lb_1:
	jmp	t_b_1
.Lb_2:
	jmp	t_b_2
.Lb_3:
	jmp	t_b_3
	.size	bit_index, .-bit_index

# gcc -O2 -fPIE: a range check, the base and the dispatch in one block.
	.globl	resolved
	.type	resolved, @function
resolved:
	cmp	$3, %edi
	ja	.Lr_def
	mov	%edi, %edi
	lea	.Lr_jt(%rip), %rdx
	movslq	(%rdx,%rdi,4), %rax
	add	%rdx, %rax
	jmp	*%rax
.Lr_0:
	jmp	t_r_0
.Lr_1:
	jmp	t_r_1
.Lr_2:
	jmp	t_r_2
.Lr_3:
	jmp	t_r_3
.Lr_def:
	ret
	.size	resolved, .-resolved

# Tail calls through a pointer: a field, an argument, an import slot read by
# the jump and by a load ahead of it, a virtual method, and a returned pointer.
	.globl	via_field
	.type	via_field, @function
via_field:
	mov	0x10(%rdi), %rax
	jmp	*%rax
	.size	via_field, .-via_field

	.globl	via_argument
	.type	via_argument, @function
via_argument:
	mov	%rdi, %rax
	jmp	*%rsi
	.size	via_argument, .-via_argument

	.globl	via_slot
	.type	via_slot, @function
via_slot:
	jmp	*.Lslot(%rip)
	.size	via_slot, .-via_slot

	.globl	via_loaded_slot
	.type	via_loaded_slot, @function
via_loaded_slot:
	mov	.Lslot(%rip), %rax
	jmp	*%rax
	.size	via_loaded_slot, .-via_loaded_slot

	.globl	via_vtable
	.type	via_vtable, @function
via_vtable:
	mov	(%rdi), %rax
	jmp	*0x18(%rax)
	.size	via_vtable, .-via_vtable

	.globl	via_return
	.type	via_return, @function
via_return:
	sub	$8, %rsp
	call	pick
	add	$8, %rsp
	jmp	*%rax
	.size	via_return, .-via_return

# Tail calls through an array of function pointers: one passed in
# (o->fn[i](x)), and a table of functions perl's PL_check[type](op) and
# glibc's svcauthsw[flavor] dispatch through, in writable data.
	.globl	via_array_argument
	.type	via_array_argument, @function
via_array_argument:
	mov	%rdi, %r8
	and	$3, %esi
	mov	%edx, %edi
	jmp	*(%r8,%rsi,8)
	.size	via_array_argument, .-via_array_argument

	.globl	via_function_table
	.type	via_function_table, @function
via_function_table:
	and	$3, %edi
	lea	.Lfn_tab(%rip), %rdx
	mov	(%rdx,%rdi,8), %rax
	jmp	*%rax
	.size	via_function_table, .-via_function_table

	.globl	via_table_after_epilogue
	.type	via_table_after_epilogue, @function
via_table_after_epilogue:
	push	%rbx
	mov	%edi, %ebx
	call	pick
	and	$3, %ebx
	lea	.Lfn_tab(%rip), %rdx
	mov	(%rdx,%rbx,8), %rax
	pop	%rbx
	jmp	*%rax
	.size	via_table_after_epilogue, .-via_table_after_epilogue

	.globl	pick
	.type	pick, @function
pick:
	lea	t_p_0(%rip), %rax
	ret
	.size	pick, .-pick

	.irp	f, t_c_0, t_c_1, t_c_2, t_c_3, t_u_0, t_u_1, t_u_2, t_u_3, t_r_0, t_r_1, t_r_2, t_r_3, t_p_0, t_b_0, t_b_1, t_b_2, t_b_3, t_f_0, t_f_1, t_f_2, t_f_3
	.globl	\f
	.type	\f, @function
\f:
	mov	%edi, %eax
	ret
	.size	\f, .-\f
	.endr

	.section .rodata
	.p2align 2
.Lc_jt:
	.long	.Lc_0-.Lc_jt
	.long	.Lc_1-.Lc_jt
	.long	.Lc_2-.Lc_jt
	.long	.Lc_3-.Lc_jt
.Lr_jt:
	.long	.Lr_0-.Lr_jt
	.long	.Lr_1-.Lr_jt
	.long	.Lr_2-.Lr_jt
	.long	.Lr_3-.Lr_jt
.Lu_jt:
	.long	.Lu_0-.Lu_jt
	.long	.Lu_1-.Lu_jt
	.long	.Lu_2-.Lu_jt
	.long	.Lu_3-.Lu_jt
.Lb_jt:
	.long	.Lb_0-.Lb_jt
	.long	.Lb_1-.Lb_jt
	.long	.Lb_2-.Lb_jt
	.long	.Lb_3-.Lb_jt
	.p2align 3
.Lslot:
	.quad	t_p_0

	.data
	.p2align 3
.Lfn_tab:
	.quad	t_f_0
	.quad	t_f_1
	.quad	t_f_2
	.quad	t_f_3

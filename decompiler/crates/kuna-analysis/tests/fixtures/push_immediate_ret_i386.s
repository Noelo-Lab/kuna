# One-store PUSH-immediate/RET tail transfer and conservative controls.
#
#   gcc -m32 -nostdlib -no-pie -Wl,-e,push_immediate_ret \
#     -Wl,--section-start=.encrypted=0x0804b000 \
#     -o push_immediate_ret_i386 push_immediate_ret_i386.s
	.text
	.globl push_immediate_ret
	.type push_immediate_ret, @function
push_immediate_ret:
	call	unpacker
	pushl	$0x0804b000
	ret
	.size push_immediate_ret, .-push_immediate_ret

	.type unpacker, @function
unpacker:
	movl	$0x1111, %eax
	ret
	.size unpacker, .-unpacker

	.globl ordinary_ret
	.type ordinary_ret, @function
ordinary_ret:
	ret
	.size ordinary_ret, .-ordinary_ret

	.globl argument_push_later_ret
	.type argument_push_later_ret, @function
argument_push_later_ret:
	pushl	$0x0804b000
	call	argument_user
	addl	$4, %esp
	ret
	.size argument_push_later_ret, .-argument_push_later_ret

	.type argument_user, @function
argument_user:
	movl	4(%esp), %eax
	ret
	.size argument_user, .-argument_user

	.globl stack_adjust_ret
	.type stack_adjust_ret, @function
stack_adjust_ret:
	pushl	$0x0804b000
	addl	$4, %esp
	ret
	.size stack_adjust_ret, .-stack_adjust_ret

	.globl stack_overwrite_ret
	.type stack_overwrite_ret, @function
stack_overwrite_ret:
	pushl	$0x0804b000
	movl	%eax, (%esp)
	ret
	.size stack_overwrite_ret, .-stack_overwrite_ret

	.globl computed_target_ret
	.type computed_target_ret, @function
computed_target_ret:
	pushl	%eax
	ret
	.size computed_target_ret, .-computed_target_ret

	.globl conditional_bypass_ret
	.type conditional_bypass_ret, @function
conditional_bypass_ret:
	testl	%eax, %eax
	jz	.Lconditional_return
	pushl	$0x0804b000
.Lconditional_return:
	ret
	.size conditional_bypass_ret, .-conditional_bypass_ret

	.globl two_push_dispatch
	.type two_push_dispatch, @function
two_push_dispatch:
	pushl	$.Ldispatch_continuation
	pushl	$dispatch_helper
	ret
.Ldispatch_continuation:
	ret
	.size two_push_dispatch, .-two_push_dispatch

	.type dispatch_helper, @function
dispatch_helper:
	movl	$0x2222, %edx
	ret
	.size dispatch_helper, .-dispatch_helper

	.section .encrypted,"aw",@progbits
	.byte 0x29, 0x60, 0x8c, 0x1d, 0x23, 0x33, 0x2b, 0x30
	.byte 0xb8, 0x4d, 0x4c, 0xc3, 0x02, 0x54, 0xc0, 0x2b

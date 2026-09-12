# entry_ret_dispatch_i386: an entry body built from stack-directed RET calls,
# plus ordinary-return controls for RE-need `entry-point-ret-dispatch`.
#
#     gcc -m32 -nostdlib -no-pie -Wl,-e,entry_dispatch \
#       -o entry_ret_dispatch_i386 entry_ret_dispatch_i386.s
	.text
	.globl entry_dispatch
	.type entry_dispatch, @function
entry_dispatch:
	pushl	$.Lcont_one
	pushl	slot_one
	ret
.Lcont_one:
	pushl	$.Lcont_two
	pushl	slot_two
	ret
.Lcont_two:
	pushl	$.Ldone
	pushl	slot_three
	ret
.Ldone:
	ret
	.size entry_dispatch, .-entry_dispatch

	.globl ordinary_ret
	.type ordinary_ret, @function
ordinary_ret:
	movl	$0x1111, %eax
	ret
	.size ordinary_ret, .-ordinary_ret

	.globl immediate_ret
	.type immediate_ret, @function
immediate_ret:
	movl	$0x2222, %eax
	ret	$8
	.size immediate_ret, .-immediate_ret

	.globl incoming_return_ret
	.type incoming_return_ret, @function
incoming_return_ret:
	popl	%eax
	pushl	%eax
	ret
	.size incoming_return_ret, .-incoming_return_ret

	.globl computed_ret
	.type computed_ret, @function
computed_ret:
	pushl	slot_one
	ret
	.size computed_ret, .-computed_ret

	.globl constant_ret
	.type constant_ret, @function
constant_ret:
	pushl	$ordinary_ret
	ret
	.size constant_ret, .-constant_ret

	# The old literal-only recognizer mistook this for a call: the pushed
	# constant equals RET's own fall-through, but ADD discards that slot before
	# RET consumes the untouched incoming return address.
	.globl adjusted_fallthrough_ret
	.type adjusted_fallthrough_ret, @function
adjusted_fallthrough_ret:
	pushl	$.Ladjusted_fallthrough
	addl	$4, %esp
	ret
.Ladjusted_fallthrough:
	ret
	.size adjusted_fallthrough_ret, .-adjusted_fallthrough_ret

	# This stores RET's fall-through to unrelated memory. RET still consumes the
	# incoming return address, so address equality alone is not dispatch proof.
	.globl unrelated_fallthrough_store_ret
	.type unrelated_fallthrough_store_ret, @function
unrelated_fallthrough_store_ret:
	movl	$.Lunrelated_fallthrough, decoy_slot
	ret
.Lunrelated_fallthrough:
	ret
	.size unrelated_fallthrough_store_ret, .-unrelated_fallthrough_store_ret

	# The continuation store is addressed through EAX-4. P-code encodes -4 as
	# an unsigned constant of the operand width; the recognizer must sign-extend
	# it and see that this overwrites the proven continuation. At runtime the
	# helper is therefore entered twice, not followed by this fall-through.
	.globl negative_displacement_ret
	.type negative_displacement_ret, @function
negative_displacement_ret:
	movl	%esp, %eax
	pushl	$.Lnegative_fallthrough
	pushl	slot_one
	movl	$first_link, -4(%eax)
	ret
.Lnegative_fallthrough:
	ret
	.size negative_displacement_ret, .-negative_displacement_ret

	# One path bypasses the store pair and reaches RET with the incoming return
	# address. A fall-through-only scan cannot prove the setup dominates RET.
	.globl conditional_bypass_ret
	.type conditional_bypass_ret, @function
conditional_bypass_ret:
	testl	%eax, %eax
	jz	.Lconditional_return
	pushl	$.Lconditional_fallthrough
	pushl	slot_one
.Lconditional_return:
	ret
.Lconditional_fallthrough:
	ret
	.size conditional_bypass_ret, .-conditional_bypass_ret

	# A partial write to SP destroys the tracked ESP relation. Register aliases
	# overlap, so the earlier stores cannot remain valid dispatch provenance.
	.globl partial_sp_ret
	.type partial_sp_ret, @function
partial_sp_ret:
	pushl	$.Lpartial_fallthrough
	pushl	slot_one
	movw	$0x1234, %sp
	ret
.Lpartial_fallthrough:
	ret
	.size partial_sp_ret, .-partial_sp_ret

	.globl first_link
	.type first_link, @function
first_link:
	movl	$0x3333, %eax
	ret
	.size first_link, .-first_link

	.globl second_link
	.type second_link, @function
second_link:
	movl	$0x4444, %edx
	ret
	.size second_link, .-second_link

	.globl third_link
	.type third_link, @function
third_link:
	movl	$0x5555, %ecx
	ret
	.size third_link, .-third_link

	.section .rodata
	.align 4
slot_one:
	.long	first_link
slot_two:
	.long	second_link
slot_three:
	.long	third_link

	.data
	.align 4
decoy_slot:
	.long	0

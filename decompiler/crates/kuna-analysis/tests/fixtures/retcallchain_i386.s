# retcallchain_i386: a body built out of `push <continuation>; push <target>;
# ret` call links (RE-need `flow-call-override-retain`, round-9 challenge
# 5ab77f5c33c5d40ad448c681).
#
# Each `ret` is a call: it pops <target> and jumps to it, and the callee returns
# to <continuation>, which is the instruction right after the `ret`.  The
# witness builds its whole self-unpacking body this way -- 22 links -- so a
# `flow <addr> call` override on the first one recovers exactly one call and the
# rest of the body decompiles to `return;`.  Three links here are enough to
# distinguish "the override was applied" from "the chain was followed".
#
#     gcc -m32 -nostdlib -no-pie -o retcallchain_i386 retcallchain_i386.s
	.text
	.globl _start
	.type _start, @function
_start:
	call	chain_entry
	movl	$1, %eax
	xorl	%ebx, %ebx
	int	$0x80
	.size	_start, .-_start

	.globl chain_entry
	.type chain_entry, @function
chain_entry:
	pushl	$.Lcont_one
	pushl	slot_one
	ret
.Lcont_one:
	pushl	$.Lcont_two
	pushl	slot_two
	ret
.Lcont_two:
	pushl	$.Lcont_three
	pushl	slot_three
	ret
.Lcont_three:
	ret
	.size	chain_entry, .-chain_entry

	.globl first_link
	.type first_link, @function
first_link:
	movl	$0x1111, %eax
	ret
	.size	first_link, .-first_link

	.globl second_link
	.type second_link, @function
second_link:
	movl	$0x2222, %edx
	ret
	.size	second_link, .-second_link

	.globl third_link
	.type third_link, @function
third_link:
	movl	$0x3333, %ecx
	ret
	.size	third_link, .-third_link

	.section .rodata
	.align 4
slot_one:
	.long	first_link
slot_two:
	.long	second_link
slot_three:
	.long	third_link

/* gcc -m32 -nostdlib -no-pie -Wl,--build-id=none,-Ttext=0x100000,-e,_start \
 *     x86listingswitch_i386.s -o /tmp/x86listingswitch_i386.full
 * strip --strip-all /tmp/x86listingswitch_i386.full -o x86listingswitch_i386
 * The ELF exits zero after checking each dispatch result, including defaults.
 */

	.text
	.globl _start
	.type _start, @function
_start:
	.irp number,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23
	pushl $7
	call leaf_\number
	addl $4, %esp
	.endr
	.macro check selector, expected
	pushl $\selector
	call dispatch
	addl $4, %esp
	cmpl $\expected, %eax
	jne .Lfailed
	.if \selector == 2
	cmpl $1, seen_case_helper
	jne .Lfailed
	.endif
	.endm
	check 0, 11
	check 1, 22
	check 2, 33
	check 3, 44
	check -1, -7
	check 4, -7
	check 0x80000000, -7
	check 0xffffffff, -7
	xorl %ebx, %ebx
	jmp .Lexit
.Lfailed:
	movl $1, %ebx
.Lexit:
	movl $1, %eax
	int $0x80
	.size _start, .-_start

	.macro leaf number
	.org 0x200 + 0x10 * \number, 0xcc
	.globl leaf_\number
	.type leaf_\number, @function
leaf_\number:
	xorl %edx, %edx
	xorl %ecx, %ecx
	movl 4(%esp), %eax
	addl $\number, %eax
	ret
	.size leaf_\number, .-leaf_\number
	.endm
	.irp number,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23
	leaf \number
	.endr

	.org 0x400, 0xcc
	.globl dispatch
	.type dispatch, @function
dispatch:
	movl 4(%esp), %eax
	cmpl $3, %eax
	ja default_case
	.globl table_jump
table_jump:
	jmp *switch_table(,%eax,4)
	.size dispatch, .-dispatch

	.org 0x440, 0xcc
	.globl case_zero
case_zero:
	xorl %edx, %edx
	xorl %ecx, %ecx
	movl $11, %eax
	ret

	.org 0x460, 0xcc
	.globl case_one
case_one:
	xorl %edx, %edx
	xorl %ecx, %ecx
	movl $22, %eax
	ret

	.org 0x480, 0xcc
	.globl independent
	.type independent, @function
independent:
	xorl %edx, %edx
	xorl %ecx, %ecx
	movl $1234, %eax
	ret
	.size independent, .-independent

	.org 0x490, 0xcc
	.globl interleaved_gap
interleaved_gap:
	.byte 0xde, 0xad, 0xbe, 0xef, 0x55, 0x23, 0x93, 0x71

	.org 0x4b0, 0xcc
	.globl case_two
case_two:
	xorl %edx, %edx
	xorl %ecx, %ecx
	call case_helper
	movl $33, %eax
	ret

	.org 0x4d0, 0xcc
	.globl case_three
case_three:
	xorl %edx, %edx
	xorl %ecx, %ecx
	movl $44, %eax
	ret

	.org 0x4f0, 0xcc
	.globl default_case
default_case:
	xorl %edx, %edx
	xorl %ecx, %ecx
	movl $-7, %eax
	ret

	.org 0x510, 0xcc
	.globl case_helper
	.type case_helper, @function
case_helper:
	movl $1, seen_case_helper
	ret
	.size case_helper, .-case_helper

	.section .rodata
	.balign 4
	.globl switch_table
switch_table:
	.long case_zero, case_one, case_two, case_three
	.globl adjacent_callback
adjacent_callback:
	.long independent

	.section .data
	.globl seen_case_helper
seen_case_helper:
	.long 0

	.section .note.GNU-stack,"",@progbits

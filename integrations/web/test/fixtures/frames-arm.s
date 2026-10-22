@ ARM prologue shapes the frame walk has to read (assembled, no libc): each
@ function leaves through a conditional call or return before it saves its
@ registers and reserves its locals; `tcondcall` is Thumb, with a vpush.
@
@ Rebuild:  clang --target=armv7a-linux-gnueabihf -c -o frames-arm.o frames-arm.s
	.syntax unified
	.arm
	.text
	.globl condcall
	.type condcall, %function
condcall:
	cmp r0, #0
	bleq abort
	push {r4, lr}
	sub sp, sp, #40
	mov r0, sp
	bl fill
	add sp, sp, #40
	pop {r4, pc}
	.size condcall, .-condcall

	.globl condret
	.type condret, %function
condret:
	cmp r0, #0
	bxeq lr
	push {r4, lr}
	sub sp, sp, #40
	mov r0, sp
	bl fill
	add sp, sp, #40
	pop {r4, pc}
	.size condret, .-condret

	.globl condpop
	.type condpop, %function
condpop:
	cmp r0, #0
	moveq pc, lr
	push {r4, r5, r6, lr}
	vpush {d8-d11}
	sub sp, sp, #16
	mov r0, sp
	bl fill
	add sp, sp, #16
	vpop {d8-d11}
	pop {r4, r5, r6, pc}
	.size condpop, .-condpop

	.thumb
	.globl tcondcall
	.type tcondcall, %function
	.thumb_func
tcondcall:
	cmp r0, #0
	it eq
	bleq abort
	push {r4, lr}
	vpush {d8-d9}
	sub sp, #40
	mov r0, sp
	bl fill
	add sp, #40
	vpop {d8-d9}

	.arm
	.globl fill
	.type fill, %function
fill:
	mov r1, #0
	str r1, [r0]
	bx lr
	.size fill, .-fill

	.globl abort
	.type abort, %function
abort:
	b abort
	.size abort, .-abort

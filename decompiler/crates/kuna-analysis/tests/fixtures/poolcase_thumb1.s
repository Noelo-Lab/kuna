@ `dispatch` is a Thumb-1 switch through libgcc's `__gnu_thumb1_case_uqi`
@ (lib1funcs.S, verbatim below), followed by an indirect call in the common
@ tail. The helper returns past the case table after its `bl` (`add lr,r1;
@ bx lr`), so the table bytes are no code, and the halfword 0x4808 at 0x10078
@ spells `ldr r0,[pc,#32]`, a load of the live `adds r0,r0,r5` after `blx r4`.
@ arm-linux-gnueabihf-as poolcase_thumb1.s -o poolcase_thumb1.o
@ arm-linux-gnueabihf-ld -e _start --build-id=none -o poolcase_thumb1_le32 poolcase_thumb1.o
	.syntax unified
	.cpu cortex-m0
	.thumb
	.text

	.p2align 2
	.global _start
	.type _start, %function
	.thumb_func
_start:
	movs	r0, #1
	ldr	r1, .Lcbaddr
	bl	dispatch
	movs	r7, #1
	svc	#0
	b	_start
	.p2align 2
.Lcbaddr:
	.word	cb + 1

	.p2align 2
	.global cb
	.type cb, %function
	.thumb_func
cb:
	adds	r0, r0, #3
	bx	lr

@ int dispatch(int sel, int (*cb)(int)): a Thumb-1 switch through libgcc's
@ __gnu_thumb1_case_uqi, then an indirect call in the common tail.
	.p2align 2
	.global dispatch
	.type dispatch, %function
	.thumb_func
dispatch:
	push	{r4, r5, lr}
	movs	r4, r1
	cmp	r0, #3
	bhi	.Ldefault
	bl	__gnu_thumb1_case_uqi
.Ltable:
	.byte	(.Lc0 - .Ltable) / 2
	.byte	(.Lc1 - .Ltable) / 2
	.byte	(.Lc2 - .Ltable) / 2
	.byte	(.Lc3 - .Ltable) / 2
.Lc2:
	movs	r0, #2
	b	.Lend
.Lc3:
	movs	r0, #3
	b	.Lend
.Ldefault:
	movs	r0, #9
	b	.Lend
.Lc0:
	movs	r0, #0
	b	.Lend
.Lend:
	adds	r0, r0, #1
	lsls	r0, r0, #1
	adds	r0, r0, #3
	movs	r5, r0
	adds	r5, r5, #1
	movs	r1, r5
	movs	r2, #0
	blx	r4
.Lafter:
	adds	r0, r0, r5
	adds	r0, r0, #7
	pop	{r4, r5, pc}
	.rept	51
	nop
	.endr
.Lc1:
	movs	r0, #1
	b	.Lend
	.size dispatch, .-dispatch

@ libgcc's lib1funcs.S __gnu_thumb1_case_uqi, verbatim.
	.p2align 2
	.global __gnu_thumb1_case_uqi
	.type __gnu_thumb1_case_uqi, %function
	.thumb_func
__gnu_thumb1_case_uqi:
	push	{r1}
	mov	r1, lr
	lsrs	r1, r1, #1
	lsls	r1, r1, #1
	ldrb	r1, [r1, r0]
	lsls	r1, r1, #1
	add	lr, lr, r1
	pop	{r1}
	bx	lr

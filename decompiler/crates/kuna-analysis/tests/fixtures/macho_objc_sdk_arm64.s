	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_style                          ; -- Begin function style
	.p2align	2
_style:                                 ; @style
	.cfi_startproc
; %bb.0:
	stp	x22, x21, [sp, #-48]!           ; 16-byte Folded Spill
	stp	x20, x19, [sp, #16]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	mov	x19, x2
	mov	x20, x1
	mov	x21, x0
	bl	"_objc_msgSend$setAlpha:"
	mov	x0, x21
	mov	x2, x19
	bl	"_objc_msgSend$setTag:"
	mov	x0, x21
	bl	"_objc_msgSend$isHidden"
	cbz	w0, LBB0_2
; %bb.1:
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	ret
LBB0_2:
	mov	x0, x21
	bl	"_objc_msgSend$frame"
	fmov	d4, #10.00000000
	fadd	d2, d2, d4
	mov	x0, x21
	bl	"_objc_msgSend$setFrame:"
	mov	x0, x21
	mov	x2, x20
	mov	x3, x19
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	b	"_objc_msgSend$insertSubview:atIndex:"
	.cfi_endproc
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:
	mov	w0, #0                          ; =0x0
	ret
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_:                  ; @OBJC_METH_VAR_NAME_
	.asciz	"setAlpha:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.1:                ; @OBJC_METH_VAR_NAME_.1
	.asciz	"setTag:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.2
_OBJC_SELECTOR_REFERENCES_.2:
	.quad	l_OBJC_METH_VAR_NAME_.1

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.3:                ; @OBJC_METH_VAR_NAME_.3
	.asciz	"isHidden"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.4
_OBJC_SELECTOR_REFERENCES_.4:
	.quad	l_OBJC_METH_VAR_NAME_.3

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.5:                ; @OBJC_METH_VAR_NAME_.5
	.asciz	"frame"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.6
_OBJC_SELECTOR_REFERENCES_.6:
	.quad	l_OBJC_METH_VAR_NAME_.5

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.7:                ; @OBJC_METH_VAR_NAME_.7
	.asciz	"setFrame:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.8
_OBJC_SELECTOR_REFERENCES_.8:
	.quad	l_OBJC_METH_VAR_NAME_.7

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.9:                ; @OBJC_METH_VAR_NAME_.9
	.asciz	"insertSubview:atIndex:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.10
_OBJC_SELECTOR_REFERENCES_.10:
	.quad	l_OBJC_METH_VAR_NAME_.9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols

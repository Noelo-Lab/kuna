	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2                               ; -- Begin function -[Ruler tag]
"-[Ruler tag]":                         ; @"\01-[Ruler tag]"
	.cfi_startproc
; %bb.0:
	fmov	d0, #2.50000000
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_measured                       ; -- Begin function measured
	.p2align	2
_measured:                              ; @measured
	.cfi_startproc
; %bb.0:
Lloh0:
	adrp	x8, __OBJC_PROTOCOL_REFERENCE_$_Measured@PAGE
Lloh1:
	ldr	x0, [x8, __OBJC_PROTOCOL_REFERENCE_$_Measured@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.globl	_validate                       ; -- Begin function validate
	.p2align	2
_validate:                              ; @validate
	.cfi_startproc
; %bb.0:
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x19, x2
	mov	x20, x1
	bl	"_objc_msgSend$length"
Lloh4:
	adrp	x3, l_.str@PAGE
Lloh5:
	add	x3, x3, l_.str@PAGEOFF
	mov	x1, x19
	mov	w2, #1                          ; =0x1
	bl	_check
	cbz	w0, LBB2_2
; %bb.1:
	mov	x0, x20
	bl	"_objc_msgSend$tag"
Lloh8:
	adrp	x3, l_.str.4@PAGE
Lloh9:
	add	x3, x3, l_.str.4@PAGEOFF
	mov	x1, x19
	mov	w2, #2                          ; =0x2
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	b	_check
LBB2_2:
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh8, Lloh9
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
	.section	__DATA,__objc_data
	.globl	_OBJC_CLASS_$_Ruler             ; @"OBJC_CLASS_$_Ruler"
	.p2align	3, 0x0
_OBJC_CLASS_$_Ruler:
	.quad	_OBJC_METACLASS_$_Ruler
	.quad	0
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_Ruler

	.globl	_OBJC_METACLASS_$_Ruler         ; @"OBJC_METACLASS_$_Ruler"
	.p2align	3, 0x0
_OBJC_METACLASS_$_Ruler:
	.quad	_OBJC_METACLASS_$_Ruler
	.quad	_OBJC_CLASS_$_Ruler
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_Ruler

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_:                     ; @OBJC_CLASS_NAME_
	.asciz	"Ruler"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_METACLASS_RO_$_Ruler"
__OBJC_METACLASS_RO_$_Ruler:
	.long	3                               ; 0x3
	.long	40                              ; 0x28
	.long	40                              ; 0x28
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_:                  ; @OBJC_METH_VAR_NAME_
	.asciz	"tag"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_:                  ; @OBJC_METH_VAR_TYPE_
	.asciz	"d16@0:8"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Ruler"
__OBJC_$_INSTANCE_METHODS_Ruler:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"-[Ruler tag]"

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Ruler"
__OBJC_CLASS_RO_$_Ruler:
	.long	2                               ; 0x2
	.long	0                               ; 0x0
	.long	0                               ; 0x0
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	__OBJC_$_INSTANCE_METHODS_Ruler
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.1:                   ; @OBJC_CLASS_NAME_.1
	.asciz	"Measured"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.2:                ; @OBJC_METH_VAR_NAME_.2
	.asciz	"length"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_Measured"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_Measured:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.2
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	0

	.p2align	3, 0x0                          ; @"_OBJC_$_PROTOCOL_METHOD_TYPES_Measured"
__OBJC_$_PROTOCOL_METHOD_TYPES_Measured:
	.quad	l_OBJC_METH_VAR_TYPE_

	.private_extern	__OBJC_PROTOCOL_$_Measured ; @"_OBJC_PROTOCOL_$_Measured"
	.section	__DATA,__data
	.globl	__OBJC_PROTOCOL_$_Measured
	.weak_definition	__OBJC_PROTOCOL_$_Measured
	.p2align	3, 0x0
__OBJC_PROTOCOL_$_Measured:
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.1
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_Measured
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.long	96                              ; 0x60
	.long	0                               ; 0x0
	.quad	__OBJC_$_PROTOCOL_METHOD_TYPES_Measured
	.quad	0
	.quad	0

	.private_extern	__OBJC_LABEL_PROTOCOL_$_Measured ; @"_OBJC_LABEL_PROTOCOL_$_Measured"
	.section	__DATA,__objc_protolist,coalesced,no_dead_strip
	.globl	__OBJC_LABEL_PROTOCOL_$_Measured
	.weak_definition	__OBJC_LABEL_PROTOCOL_$_Measured
	.p2align	3, 0x0
__OBJC_LABEL_PROTOCOL_$_Measured:
	.quad	__OBJC_PROTOCOL_$_Measured

	.private_extern	__OBJC_PROTOCOL_REFERENCE_$_Measured ; @"_OBJC_PROTOCOL_REFERENCE_$_Measured"
	.section	__DATA,__objc_protorefs,coalesced,no_dead_strip
	.globl	__OBJC_PROTOCOL_REFERENCE_$_Measured
	.weak_definition	__OBJC_PROTOCOL_REFERENCE_$_Measured
	.p2align	3, 0x0
__OBJC_PROTOCOL_REFERENCE_$_Measured:
	.quad	__OBJC_PROTOCOL_$_Measured

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_.2

	.section	__TEXT,__cstring,cstring_literals
l_.str:                                 ; @.str
	.asciz	"empty name"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.3
_OBJC_SELECTOR_REFERENCES_.3:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__TEXT,__cstring,cstring_literals
l_.str.4:                               ; @.str.4
	.asciz	"bad tag"

	.section	__DATA,__objc_classlist,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_LABEL_CLASS_$"
l_OBJC_LABEL_CLASS_$:
	.quad	_OBJC_CLASS_$_Ruler

	.no_dead_strip	__OBJC_LABEL_PROTOCOL_$_Measured
	.no_dead_strip	__OBJC_PROTOCOL_$_Measured
	.no_dead_strip	__OBJC_PROTOCOL_REFERENCE_$_Measured
	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols

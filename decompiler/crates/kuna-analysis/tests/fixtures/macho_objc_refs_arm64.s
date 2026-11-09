	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2                               ; -- Begin function +[Base tag]
"+[Base tag]":                          ; @"\01+[Base tag]"
	.cfi_startproc
; %bb.0:
	mov	w0, #11                         ; =0xb
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Base greet:]
"-[Base greet:]":                       ; @"\01-[Base greet:]"
	.cfi_startproc
; %bb.0:
	add	w8, w2, w2, lsl #1
	add	w0, w8, #7
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Base count]
"-[Base count]":                        ; @"\01-[Base count]"
	.cfi_startproc
; %bb.0:
	mov	w0, #4                          ; =0x4
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Derived greet:]
"-[Derived greet:]":                    ; @"\01-[Derived greet:]"
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh0:
	adrp	x8, l_OBJC_CLASSLIST_SUP_REFS_$_@PAGE
Lloh1:
	ldr	x8, [x8, l_OBJC_CLASSLIST_SUP_REFS_$_@PAGEOFF]
	stp	x0, x8, [sp]
Lloh2:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
Lloh3:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
	mov	x0, sp
	bl	_objc_msgSendSuper2
	add	w0, w0, #1
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdr	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.globl	_use                            ; -- Begin function use
	.p2align	2
_use:                                   ; @use
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
	mov	x19, x0
Lloh4:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
Lloh5:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
	mov	w2, #2                          ; =0x2
	bl	_objc_msgSend
	mov	x20, x0
	mov	x0, x19
	bl	"_objc_msgSend$count"
	add	w0, w0, w20
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh4, Lloh5
	.cfi_endproc
                                        ; -- End function
	.globl	_tag                            ; -- Begin function tag
	.p2align	2
_tag:                                   ; @tag
	.cfi_startproc
; %bb.0:
Lloh8:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGE
Lloh9:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGEOFF]
	b	"_objc_msgSend$tag"
	.loh AdrpLdr	Lloh8, Lloh9
	.cfi_endproc
                                        ; -- End function
	.globl	_use2                           ; -- Begin function use2
	.p2align	2
_use2:                                  ; @use2
	.cfi_startproc
; %bb.0:
	mov	w2, #5
	b	"_objc_msgSend$greet:"
	.cfi_endproc
                                        ; -- End function
	.globl	_remote                         ; -- Begin function remote
	.p2align	2
_remote:                                ; @remote
	.cfi_startproc
; %bb.0:
Lloh14:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGE
Lloh15:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGEOFF]
Lloh16:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.8@PAGE
Lloh17:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.8@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh16, Lloh17
	.loh AdrpAdrp	Lloh14, Lloh16
	.loh AdrpLdr	Lloh14, Lloh15
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
	.globl	_OBJC_CLASS_$_Base              ; @"OBJC_CLASS_$_Base"
	.p2align	3, 0x0
_OBJC_CLASS_$_Base:
	.quad	_OBJC_METACLASS_$_Base
	.quad	0
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_Base

	.globl	_OBJC_METACLASS_$_Base          ; @"OBJC_METACLASS_$_Base"
	.p2align	3, 0x0
_OBJC_METACLASS_$_Base:
	.quad	_OBJC_METACLASS_$_Base
	.quad	_OBJC_CLASS_$_Base
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_Base

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_:                     ; @OBJC_CLASS_NAME_
	.asciz	"Base"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_:                  ; @OBJC_METH_VAR_NAME_
	.asciz	"tag"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_:                  ; @OBJC_METH_VAR_TYPE_
	.asciz	"i16@0:8"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_CLASS_METHODS_Base"
__OBJC_$_CLASS_METHODS_Base:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"+[Base tag]"

	.p2align	3, 0x0                          ; @"_OBJC_METACLASS_RO_$_Base"
__OBJC_METACLASS_RO_$_Base:
	.long	3                               ; 0x3
	.long	40                              ; 0x28
	.long	40                              ; 0x28
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	__OBJC_$_CLASS_METHODS_Base
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.1:                ; @OBJC_METH_VAR_NAME_.1
	.asciz	"greet:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.2:                ; @OBJC_METH_VAR_TYPE_.2
	.asciz	"i20@0:8i16"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.3:                ; @OBJC_METH_VAR_NAME_.3
	.asciz	"count"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Base"
__OBJC_$_INSTANCE_METHODS_Base:
	.long	24                              ; 0x18
	.long	2                               ; 0x2
	.quad	l_OBJC_METH_VAR_NAME_.1
	.quad	l_OBJC_METH_VAR_TYPE_.2
	.quad	"-[Base greet:]"
	.quad	l_OBJC_METH_VAR_NAME_.3
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"-[Base count]"

	.section	__DATA,__objc_ivar
	.globl	_OBJC_IVAR_$_Base.isa           ; @"OBJC_IVAR_$_Base.isa"
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.isa:
	.long	0                               ; 0x0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.4:                ; @OBJC_METH_VAR_NAME_.4
	.asciz	"isa"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.5:                ; @OBJC_METH_VAR_TYPE_.5
	.asciz	"#"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_VARIABLES_Base"
__OBJC_$_INSTANCE_VARIABLES_Base:
	.long	32                              ; 0x20
	.long	1                               ; 0x1
	.quad	_OBJC_IVAR_$_Base.isa
	.quad	l_OBJC_METH_VAR_NAME_.4
	.quad	l_OBJC_METH_VAR_TYPE_.5
	.long	3                               ; 0x3
	.long	8                               ; 0x8

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Base"
__OBJC_CLASS_RO_$_Base:
	.long	2                               ; 0x2
	.long	0                               ; 0x0
	.long	8                               ; 0x8
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	__OBJC_$_INSTANCE_METHODS_Base
	.quad	0
	.quad	__OBJC_$_INSTANCE_VARIABLES_Base
	.quad	0
	.quad	0

	.section	__DATA,__objc_data
	.globl	_OBJC_CLASS_$_Derived           ; @"OBJC_CLASS_$_Derived"
	.p2align	3, 0x0
_OBJC_CLASS_$_Derived:
	.quad	_OBJC_METACLASS_$_Derived
	.quad	_OBJC_CLASS_$_Base
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_Derived

	.section	__DATA,__objc_superrefs,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_CLASSLIST_SUP_REFS_$_"
l_OBJC_CLASSLIST_SUP_REFS_$_:
	.quad	_OBJC_CLASS_$_Derived

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_.1

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.6:                   ; @OBJC_CLASS_NAME_.6
	.asciz	"Derived"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_METACLASS_RO_$_Derived"
__OBJC_METACLASS_RO_$_Derived:
	.long	1                               ; 0x1
	.long	40                              ; 0x28
	.long	40                              ; 0x28
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.6
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__DATA,__objc_data
	.globl	_OBJC_METACLASS_$_Derived       ; @"OBJC_METACLASS_$_Derived"
	.p2align	3, 0x0
_OBJC_METACLASS_$_Derived:
	.quad	_OBJC_METACLASS_$_Base
	.quad	_OBJC_METACLASS_$_Base
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_Derived

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Derived"
__OBJC_$_INSTANCE_METHODS_Derived:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.1
	.quad	l_OBJC_METH_VAR_TYPE_.2
	.quad	"-[Derived greet:]"

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Derived"
__OBJC_CLASS_RO_$_Derived:
	.long	0                               ; 0x0
	.long	8                               ; 0x8
	.long	8                               ; 0x8
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.6
	.quad	__OBJC_$_INSTANCE_METHODS_Derived
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.7
_OBJC_SELECTOR_REFERENCES_.7:
	.quad	l_OBJC_METH_VAR_NAME_.3

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_CLASSLIST_REFERENCES_$_"
_OBJC_CLASSLIST_REFERENCES_$_:
	.quad	_OBJC_CLASS_$_Derived

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.8
_OBJC_SELECTOR_REFERENCES_.8:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_CLASSLIST_REFERENCES_$_.9"
_OBJC_CLASSLIST_REFERENCES_$_.9:
	.quad	_OBJC_CLASS_$_Remote

	.section	__DATA,__objc_classlist,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_LABEL_CLASS_$"
l_OBJC_LABEL_CLASS_$:
	.quad	_OBJC_CLASS_$_Base
	.quad	_OBJC_CLASS_$_Derived

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols

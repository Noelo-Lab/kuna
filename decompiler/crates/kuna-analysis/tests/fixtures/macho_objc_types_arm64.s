	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2                               ; -- Begin function -[Base count]
"-[Base count]":                        ; @"\01-[Base count]"
	.cfi_startproc
; %bb.0:
	ldr	w0, [x0, #8]
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Base setCount:]
"-[Base setCount:]":                    ; @"\01-[Base setCount:]"
	.cfi_startproc
; %bb.0:
	str	w2, [x0, #8]
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Holder innerCount]
"-[Holder innerCount]":                 ; @"\01-[Holder innerCount]"
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
	mov	x19, x0
	ldr	x0, [x0, #16]
	ldr	w22, [x0, #8]
Lloh0:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
Lloh1:
	ldr	x20, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
	mov	x1, x20
	bl	_objc_msgSend
	mov	x21, x0
	mov	x0, x19
	bl	"_objc_msgSend$inner"
	mov	x1, x20
	bl	_objc_msgSend
	add	w8, w21, w0
	add	w0, w8, w22
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Holder inner]
"-[Holder inner]":                      ; @"\01-[Holder inner]"
	.cfi_startproc
; %bb.0:
	ldr	x0, [x0, #16]
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Holder setInner:]
"-[Holder setInner:]":                  ; @"\01-[Holder setInner:]"
	.cfi_startproc
; %bb.0:
	str	x2, [x0, #16]
	ret
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

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_METACLASS_RO_$_Base"
__OBJC_METACLASS_RO_$_Base:
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
	.asciz	"count"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_:                  ; @OBJC_METH_VAR_TYPE_
	.asciz	"i16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.1:                ; @OBJC_METH_VAR_NAME_.1
	.asciz	"setCount:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.2:                ; @OBJC_METH_VAR_TYPE_.2
	.asciz	"v20@0:8i16"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Base"
__OBJC_$_INSTANCE_METHODS_Base:
	.long	24                              ; 0x18
	.long	2                               ; 0x2
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"-[Base count]"
	.quad	l_OBJC_METH_VAR_NAME_.1
	.quad	l_OBJC_METH_VAR_TYPE_.2
	.quad	"-[Base setCount:]"

	.section	__DATA,__objc_ivar
	.globl	_OBJC_IVAR_$_Base.isa           ; @"OBJC_IVAR_$_Base.isa"
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.isa:
	.long	0                               ; 0x0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.3:                ; @OBJC_METH_VAR_NAME_.3
	.asciz	"isa"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.4:                ; @OBJC_METH_VAR_TYPE_.4
	.asciz	"#"

	.section	__DATA,__objc_ivar
	.globl	_OBJC_IVAR_$_Base.count         ; @"OBJC_IVAR_$_Base.count"
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.count:
	.long	8                               ; 0x8

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.5:                ; @OBJC_METH_VAR_TYPE_.5
	.asciz	"i"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_VARIABLES_Base"
__OBJC_$_INSTANCE_VARIABLES_Base:
	.long	32                              ; 0x20
	.long	2                               ; 0x2
	.quad	_OBJC_IVAR_$_Base.isa
	.quad	l_OBJC_METH_VAR_NAME_.3
	.quad	l_OBJC_METH_VAR_TYPE_.4
	.long	3                               ; 0x3
	.long	8                               ; 0x8
	.quad	_OBJC_IVAR_$_Base.count
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_.5
	.long	2                               ; 0x2
	.long	4                               ; 0x4

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_PROP_NAME_ATTR_:                 ; @OBJC_PROP_NAME_ATTR_
	.asciz	"count"

l_OBJC_PROP_NAME_ATTR_.6:               ; @OBJC_PROP_NAME_ATTR_.6
	.asciz	"Ti,N,Vcount"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_PROP_LIST_Base"
__OBJC_$_PROP_LIST_Base:
	.long	16                              ; 0x10
	.long	1                               ; 0x1
	.quad	l_OBJC_PROP_NAME_ATTR_
	.quad	l_OBJC_PROP_NAME_ATTR_.6

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Base"
__OBJC_CLASS_RO_$_Base:
	.long	2                               ; 0x2
	.long	0                               ; 0x0
	.long	12                              ; 0xc
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	__OBJC_$_INSTANCE_METHODS_Base
	.quad	0
	.quad	__OBJC_$_INSTANCE_VARIABLES_Base
	.quad	0
	.quad	__OBJC_$_PROP_LIST_Base

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.7:                ; @OBJC_METH_VAR_NAME_.7
	.asciz	"inner"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.8
_OBJC_SELECTOR_REFERENCES_.8:
	.quad	l_OBJC_METH_VAR_NAME_.7

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.9:                   ; @OBJC_CLASS_NAME_.9
	.asciz	"Holder"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_METACLASS_RO_$_Holder"
__OBJC_METACLASS_RO_$_Holder:
	.long	1                               ; 0x1
	.long	40                              ; 0x28
	.long	40                              ; 0x28
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.9
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__DATA,__objc_data
	.globl	_OBJC_METACLASS_$_Holder        ; @"OBJC_METACLASS_$_Holder"
	.p2align	3, 0x0
_OBJC_METACLASS_$_Holder:
	.quad	_OBJC_METACLASS_$_Base
	.quad	_OBJC_METACLASS_$_Base
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_Holder

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.10:               ; @OBJC_METH_VAR_NAME_.10
	.asciz	"innerCount"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.11:               ; @OBJC_METH_VAR_TYPE_.11
	.asciz	"@16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.12:               ; @OBJC_METH_VAR_NAME_.12
	.asciz	"setInner:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.13:               ; @OBJC_METH_VAR_TYPE_.13
	.asciz	"v24@0:8@16"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Holder"
__OBJC_$_INSTANCE_METHODS_Holder:
	.long	24                              ; 0x18
	.long	3                               ; 0x3
	.quad	l_OBJC_METH_VAR_NAME_.10
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"-[Holder innerCount]"
	.quad	l_OBJC_METH_VAR_NAME_.7
	.quad	l_OBJC_METH_VAR_TYPE_.11
	.quad	"-[Holder inner]"
	.quad	l_OBJC_METH_VAR_NAME_.12
	.quad	l_OBJC_METH_VAR_TYPE_.13
	.quad	"-[Holder setInner:]"

	.section	__DATA,__objc_ivar
	.globl	_OBJC_IVAR_$_Holder.inner       ; @"OBJC_IVAR_$_Holder.inner"
	.p2align	2, 0x0
_OBJC_IVAR_$_Holder.inner:
	.long	16                              ; 0x10

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.14:               ; @OBJC_METH_VAR_TYPE_.14
	.asciz	"@\"Base\""

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_VARIABLES_Holder"
__OBJC_$_INSTANCE_VARIABLES_Holder:
	.long	32                              ; 0x20
	.long	1                               ; 0x1
	.quad	_OBJC_IVAR_$_Holder.inner
	.quad	l_OBJC_METH_VAR_NAME_.7
	.quad	l_OBJC_METH_VAR_TYPE_.14
	.long	3                               ; 0x3
	.long	8                               ; 0x8

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_PROP_NAME_ATTR_.15:              ; @OBJC_PROP_NAME_ATTR_.15
	.asciz	"inner"

l_OBJC_PROP_NAME_ATTR_.16:              ; @OBJC_PROP_NAME_ATTR_.16
	.asciz	"T@\"Base\",N,Vinner"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_PROP_LIST_Holder"
__OBJC_$_PROP_LIST_Holder:
	.long	16                              ; 0x10
	.long	1                               ; 0x1
	.quad	l_OBJC_PROP_NAME_ATTR_.15
	.quad	l_OBJC_PROP_NAME_ATTR_.16

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Holder"
__OBJC_CLASS_RO_$_Holder:
	.long	0                               ; 0x0
	.long	16                              ; 0x10
	.long	24                              ; 0x18
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.9
	.quad	__OBJC_$_INSTANCE_METHODS_Holder
	.quad	0
	.quad	__OBJC_$_INSTANCE_VARIABLES_Holder
	.quad	0
	.quad	__OBJC_$_PROP_LIST_Holder

	.section	__DATA,__objc_data
	.globl	_OBJC_CLASS_$_Holder            ; @"OBJC_CLASS_$_Holder"
	.p2align	3, 0x0
_OBJC_CLASS_$_Holder:
	.quad	_OBJC_METACLASS_$_Holder
	.quad	_OBJC_CLASS_$_Base
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_Holder

	.section	__DATA,__objc_classlist,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_LABEL_CLASS_$"
l_OBJC_LABEL_CLASS_$:
	.quad	_OBJC_CLASS_$_Base
	.quad	_OBJC_CLASS_$_Holder

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols

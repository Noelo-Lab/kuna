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
	.p2align	2                               ; -- Begin function -[Base ratio]
"-[Base ratio]":                        ; @"\01-[Base ratio]"
	.cfi_startproc
; %bb.0:
	ldr	d0, [x0, #16]
	fadd	d0, d0, d0
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Base(Extra) greet:]
"-[Base(Extra) greet:]":                ; @"\01-[Base(Extra) greet:]"
	.cfi_startproc
; %bb.0:
	ldr	w8, [x0, #8]
	add	w0, w8, w2
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function +[Base(Extra) extraTag]
"+[Base(Extra) extraTag]":              ; @"\01+[Base(Extra) extraTag]"
	.cfi_startproc
; %bb.0:
	mov	w0, #3                          ; =0x3
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Remote(Extra) remoteGreet:]
"-[Remote(Extra) remoteGreet:]":        ; @"\01-[Remote(Extra) remoteGreet:]"
	.cfi_startproc
; %bb.0:
	lsl	w0, w2, #1
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function +[Remote(Extra) remoteTag]
"+[Remote(Extra) remoteTag]":           ; @"\01+[Remote(Extra) remoteTag]"
	.cfi_startproc
; %bb.0:
	mov	w0, #5                          ; =0x5
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[Sub x]
"-[Sub x]":                             ; @"\01-[Sub x]"
	.cfi_startproc
; %bb.0:
Lloh0:
	adrp	x8, _OBJC_IVAR_$_Sub.x@PAGE
Lloh1:
	ldrsw	x8, [x8, _OBJC_IVAR_$_Sub.x@PAGEOFF]
	ldr	w0, [x0, x8]
	ret
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
	bl	"_objc_msgSend$wave"
Lloh4:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.23@PAGE
Lloh5:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.23@PAGEOFF]
	mov	x0, x19
	mov	w2, #1                          ; =0x1
	bl	_objc_msgSend
	mov	x20, x0
Lloh6:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.24@PAGE
Lloh7:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.24@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
	add	w0, w0, w20
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh6, Lloh7
	.loh AdrpLdr	Lloh4, Lloh5
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

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.3:                ; @OBJC_METH_VAR_NAME_.3
	.asciz	"ratio"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.4:                ; @OBJC_METH_VAR_TYPE_.4
	.asciz	"d16@0:8"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Base"
__OBJC_$_INSTANCE_METHODS_Base:
	.long	24                              ; 0x18
	.long	3                               ; 0x3
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"-[Base count]"
	.quad	l_OBJC_METH_VAR_NAME_.1
	.quad	l_OBJC_METH_VAR_TYPE_.2
	.quad	"-[Base setCount:]"
	.quad	l_OBJC_METH_VAR_NAME_.3
	.quad	l_OBJC_METH_VAR_TYPE_.4
	.quad	"-[Base ratio]"

	.section	__DATA,__objc_ivar
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.isa:
	.long	0                               ; 0x0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.5:                ; @OBJC_METH_VAR_NAME_.5
	.asciz	"isa"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.6:                ; @OBJC_METH_VAR_TYPE_.6
	.asciz	"#"

	.section	__DATA,__objc_ivar
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.count:
	.long	8                               ; 0x8

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.7:                ; @OBJC_METH_VAR_TYPE_.7
	.asciz	"i"

	.section	__DATA,__objc_ivar
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.ratio:
	.long	16                              ; 0x10

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.8:                ; @OBJC_METH_VAR_TYPE_.8
	.asciz	"d"

	.section	__DATA,__objc_ivar
	.p2align	2, 0x0
_OBJC_IVAR_$_Base.name:
	.long	24                              ; 0x18

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.9:                ; @OBJC_METH_VAR_NAME_.9
	.asciz	"name"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.10:               ; @OBJC_METH_VAR_TYPE_.10
	.asciz	"@"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_VARIABLES_Base"
__OBJC_$_INSTANCE_VARIABLES_Base:
	.long	32                              ; 0x20
	.long	4                               ; 0x4
	.quad	_OBJC_IVAR_$_Base.isa
	.quad	l_OBJC_METH_VAR_NAME_.5
	.quad	l_OBJC_METH_VAR_TYPE_.6
	.long	3                               ; 0x3
	.long	8                               ; 0x8
	.quad	_OBJC_IVAR_$_Base.count
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_.7
	.long	2                               ; 0x2
	.long	4                               ; 0x4
	.quad	_OBJC_IVAR_$_Base.ratio
	.quad	l_OBJC_METH_VAR_NAME_.3
	.quad	l_OBJC_METH_VAR_TYPE_.8
	.long	3                               ; 0x3
	.long	8                               ; 0x8
	.quad	_OBJC_IVAR_$_Base.name
	.quad	l_OBJC_METH_VAR_NAME_.9
	.quad	l_OBJC_METH_VAR_TYPE_.10
	.long	3                               ; 0x3
	.long	8                               ; 0x8

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Base"
__OBJC_CLASS_RO_$_Base:
	.long	2                               ; 0x2
	.long	0                               ; 0x0
	.long	32                              ; 0x20
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	__OBJC_$_INSTANCE_METHODS_Base
	.quad	0
	.quad	__OBJC_$_INSTANCE_VARIABLES_Base
	.quad	0
	.quad	0

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.11:                  ; @OBJC_CLASS_NAME_.11
	.asciz	"Extra"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.12:               ; @OBJC_METH_VAR_NAME_.12
	.asciz	"greet:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.13:               ; @OBJC_METH_VAR_TYPE_.13
	.asciz	"i20@0:8i16"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_CATEGORY_INSTANCE_METHODS_Base_$_Extra"
__OBJC_$_CATEGORY_INSTANCE_METHODS_Base_$_Extra:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.12
	.quad	l_OBJC_METH_VAR_TYPE_.13
	.quad	"-[Base(Extra) greet:]"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.14:               ; @OBJC_METH_VAR_NAME_.14
	.asciz	"extraTag"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_CATEGORY_CLASS_METHODS_Base_$_Extra"
__OBJC_$_CATEGORY_CLASS_METHODS_Base_$_Extra:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.14
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"+[Base(Extra) extraTag]"

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.15:                  ; @OBJC_CLASS_NAME_.15
	.asciz	"Greeter"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_Greeter"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_Greeter:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.12
	.quad	l_OBJC_METH_VAR_TYPE_.13
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.16:               ; @OBJC_METH_VAR_NAME_.16
	.asciz	"wave"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.17:               ; @OBJC_METH_VAR_TYPE_.17
	.asciz	"v16@0:8"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_Greeter"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_Greeter:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.16
	.quad	l_OBJC_METH_VAR_TYPE_.17
	.quad	0

	.p2align	3, 0x0                          ; @"_OBJC_$_PROTOCOL_METHOD_TYPES_Greeter"
__OBJC_$_PROTOCOL_METHOD_TYPES_Greeter:
	.quad	l_OBJC_METH_VAR_TYPE_.13
	.quad	l_OBJC_METH_VAR_TYPE_.17

	.private_extern	__OBJC_PROTOCOL_$_Greeter ; @"_OBJC_PROTOCOL_$_Greeter"
	.section	__DATA,__data
	.globl	__OBJC_PROTOCOL_$_Greeter
	.weak_definition	__OBJC_PROTOCOL_$_Greeter
	.p2align	3, 0x0
__OBJC_PROTOCOL_$_Greeter:
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.15
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_Greeter
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_Greeter
	.quad	0
	.quad	0
	.long	96                              ; 0x60
	.long	0                               ; 0x0
	.quad	__OBJC_$_PROTOCOL_METHOD_TYPES_Greeter
	.quad	0
	.quad	0

	.private_extern	__OBJC_LABEL_PROTOCOL_$_Greeter ; @"_OBJC_LABEL_PROTOCOL_$_Greeter"
	.section	__DATA,__objc_protolist,coalesced,no_dead_strip
	.globl	__OBJC_LABEL_PROTOCOL_$_Greeter
	.weak_definition	__OBJC_LABEL_PROTOCOL_$_Greeter
	.p2align	3, 0x0
__OBJC_LABEL_PROTOCOL_$_Greeter:
	.quad	__OBJC_PROTOCOL_$_Greeter

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_CATEGORY_PROTOCOLS_$_Base_$_Extra"
__OBJC_CATEGORY_PROTOCOLS_$_Base_$_Extra:
	.quad	1                               ; 0x1
	.quad	__OBJC_PROTOCOL_$_Greeter
	.quad	0

	.p2align	3, 0x0                          ; @"_OBJC_$_CATEGORY_Base_$_Extra"
__OBJC_$_CATEGORY_Base_$_Extra:
	.quad	l_OBJC_CLASS_NAME_.11
	.quad	_OBJC_CLASS_$_Base
	.quad	__OBJC_$_CATEGORY_INSTANCE_METHODS_Base_$_Extra
	.quad	__OBJC_$_CATEGORY_CLASS_METHODS_Base_$_Extra
	.quad	__OBJC_CATEGORY_PROTOCOLS_$_Base_$_Extra
	.quad	0
	.quad	0
	.long	64                              ; 0x40
	.space	4

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.18:               ; @OBJC_METH_VAR_NAME_.18
	.asciz	"remoteGreet:"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_CATEGORY_INSTANCE_METHODS_Remote_$_Extra"
__OBJC_$_CATEGORY_INSTANCE_METHODS_Remote_$_Extra:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.18
	.quad	l_OBJC_METH_VAR_TYPE_.13
	.quad	"-[Remote(Extra) remoteGreet:]"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.19:               ; @OBJC_METH_VAR_NAME_.19
	.asciz	"remoteTag"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_CATEGORY_CLASS_METHODS_Remote_$_Extra"
__OBJC_$_CATEGORY_CLASS_METHODS_Remote_$_Extra:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.19
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"+[Remote(Extra) remoteTag]"

	.p2align	3, 0x0                          ; @"_OBJC_$_CATEGORY_Remote_$_Extra"
__OBJC_$_CATEGORY_Remote_$_Extra:
	.quad	l_OBJC_CLASS_NAME_.11
	.quad	_OBJC_CLASS_$_Remote
	.quad	__OBJC_$_CATEGORY_INSTANCE_METHODS_Remote_$_Extra
	.quad	__OBJC_$_CATEGORY_CLASS_METHODS_Remote_$_Extra
	.quad	0
	.quad	0
	.quad	0
	.long	64                              ; 0x40
	.space	4

	.section	__DATA,__objc_ivar
	.p2align	2, 0x0
_OBJC_IVAR_$_Sub.x:
	.long	0                               ; 0x0

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.20:                  ; @OBJC_CLASS_NAME_.20
	.asciz	"Sub"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_METACLASS_RO_$_Sub"
__OBJC_METACLASS_RO_$_Sub:
	.long	1                               ; 0x1
	.long	40                              ; 0x28
	.long	40                              ; 0x28
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.20
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__DATA,__objc_data
	.globl	_OBJC_METACLASS_$_Sub           ; @"OBJC_METACLASS_$_Sub"
	.p2align	3, 0x0
_OBJC_METACLASS_$_Sub:
	.quad	_OBJC_METACLASS_$_Remote
	.quad	_OBJC_METACLASS_$_Remote
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_Sub

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.21:               ; @OBJC_METH_VAR_NAME_.21
	.asciz	"x"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_METHODS_Sub"
__OBJC_$_INSTANCE_METHODS_Sub:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.21
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	"-[Sub x]"

	.section	__DATA,__objc_ivar
	.p2align	2, 0x0
_OBJC_IVAR_$_Sub.tag:
	.long	8                               ; 0x8

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.22:               ; @OBJC_METH_VAR_NAME_.22
	.asciz	"tag"

	.section	__DATA,__objc_const
	.p2align	3, 0x0                          ; @"_OBJC_$_INSTANCE_VARIABLES_Sub"
__OBJC_$_INSTANCE_VARIABLES_Sub:
	.long	32                              ; 0x20
	.long	2                               ; 0x2
	.quad	_OBJC_IVAR_$_Sub.x
	.quad	l_OBJC_METH_VAR_NAME_.21
	.quad	l_OBJC_METH_VAR_TYPE_.7
	.long	2                               ; 0x2
	.long	4                               ; 0x4
	.quad	_OBJC_IVAR_$_Sub.tag
	.quad	l_OBJC_METH_VAR_NAME_.22
	.quad	l_OBJC_METH_VAR_TYPE_.10
	.long	3                               ; 0x3
	.long	8                               ; 0x8

	.p2align	3, 0x0                          ; @"_OBJC_CLASS_RO_$_Sub"
__OBJC_CLASS_RO_$_Sub:
	.long	0                               ; 0x0
	.long	0                               ; 0x0
	.long	16                              ; 0x10
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.20
	.quad	__OBJC_$_INSTANCE_METHODS_Sub
	.quad	0
	.quad	__OBJC_$_INSTANCE_VARIABLES_Sub
	.quad	0
	.quad	0

	.section	__DATA,__objc_data
	.globl	_OBJC_CLASS_$_Sub               ; @"OBJC_CLASS_$_Sub"
	.p2align	3, 0x0
_OBJC_CLASS_$_Sub:
	.quad	_OBJC_METACLASS_$_Sub
	.quad	_OBJC_CLASS_$_Remote
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_Sub

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_.16

	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.23
_OBJC_SELECTOR_REFERENCES_.23:
	.quad	l_OBJC_METH_VAR_NAME_.12

	.p2align	3, 0x0                          ; @OBJC_SELECTOR_REFERENCES_.24
_OBJC_SELECTOR_REFERENCES_.24:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__DATA,__objc_classlist,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_LABEL_CLASS_$"
l_OBJC_LABEL_CLASS_$:
	.quad	_OBJC_CLASS_$_Base
	.quad	_OBJC_CLASS_$_Sub

	.section	__DATA,__objc_catlist,regular,no_dead_strip
	.p2align	3, 0x0                          ; @"OBJC_LABEL_CATEGORY_$"
l_OBJC_LABEL_CATEGORY_$:
	.quad	__OBJC_$_CATEGORY_Base_$_Extra
	.quad	__OBJC_$_CATEGORY_Remote_$_Extra

	.no_dead_strip	__OBJC_LABEL_PROTOCOL_$_Greeter
	.no_dead_strip	__OBJC_PROTOCOL_$_Greeter
	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols

	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.byte	0x90
"-[Greeter greet:]":
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	leal	(%rdx,%rdx,2), %eax
	addl	$7, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.section	__DATA,__objc_data
	.globl	_OBJC_CLASS_$_Greeter
	.p2align	3
_OBJC_CLASS_$_Greeter:
	.quad	_OBJC_METACLASS_$_Greeter
	.quad	0
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_Greeter

	.globl	_OBJC_METACLASS_$_Greeter
	.p2align	3
_OBJC_METACLASS_$_Greeter:
	.quad	_OBJC_METACLASS_$_Greeter
	.quad	_OBJC_CLASS_$_Greeter
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_Greeter

	.section	__TEXT,__objc_classname,cstring_literals
L_OBJC_CLASS_NAME_:
	.asciz	"Greeter"

	.section	__DATA,__objc_const
	.p2align	3
__OBJC_METACLASS_RO_$_Greeter:
	.long	131
	.long	40
	.long	40
	.space	4
	.quad	0
	.quad	L_OBJC_CLASS_NAME_
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
L_OBJC_METH_VAR_NAME_:
	.asciz	"greet:"

	.section	__TEXT,__objc_methtype,cstring_literals
L_OBJC_METH_VAR_TYPE_:
	.asciz	"i20@0:8i16"

	.section	__DATA,__objc_const
	.p2align	3
__OBJC_$_INSTANCE_METHODS_Greeter:
	.long	24
	.long	1
	.quad	L_OBJC_METH_VAR_NAME_
	.quad	L_OBJC_METH_VAR_TYPE_
	.quad	"-[Greeter greet:]"

	.p2align	3
__OBJC_CLASS_RO_$_Greeter:
	.long	130
	.long	0
	.long	0
	.space	4
	.quad	0
	.quad	L_OBJC_CLASS_NAME_
	.quad	__OBJC_$_INSTANCE_METHODS_Greeter
	.quad	0
	.quad	0
	.quad	0
	.quad	0

	.section	__DATA,__objc_classlist,regular,no_dead_strip
	.p2align	3
l_OBJC_LABEL_CLASS_$:
	.quad	_OBJC_CLASS_$_Greeter

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols

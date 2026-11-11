// GH #1376: the x86-64 spellings of a call through an import's GOT slot.
	.text
	.globl	_audit_call_slot
_audit_call_slot:
	pushq	%rbp
	callq	*_malloc@GOTPCREL(%rip)
	popq	%rbp
	retq

	.globl	_audit_load_call
_audit_load_call:
	pushq	%rbp
	movq	_malloc@GOTPCREL(%rip), %rax
	callq	*%rax
	popq	%rbp
	retq

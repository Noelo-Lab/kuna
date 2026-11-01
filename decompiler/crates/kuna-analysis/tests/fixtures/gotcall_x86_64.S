// GH #1376's ELF control: call *malloc@GOTPCREL(%rip), and the same slot
// loaded into a register first (an R_X86_64_GLOB_DAT slot).
	.text
	.globl	audit_call_slot
	.type	audit_call_slot, @function
audit_call_slot:
	pushq	%rbp
	call	*malloc@GOTPCREL(%rip)
	popq	%rbp
	ret
	.size	audit_call_slot, .-audit_call_slot

	.globl	audit_load_call
	.type	audit_load_call, @function
audit_load_call:
	pushq	%rbp
	movq	malloc@GOTPCREL(%rip), %rax
	call	*%rax
	popq	%rbp
	ret
	.size	audit_load_call, .-audit_load_call

// Fixture for the `noreturn_discstrict` gate (GH-312): three undecodable bytes
// forge a no-return verdict for a function that plainly returns, and the forged
// verdict then DELETES a live caller's tail.
//
// `retseven` is `mov $7,%eax ; ret` -- it returns, always.  `gap1`/`gap2`/`gap3`
// each follow their `call retseven` with one byte that is invalid in 64-bit mode
// (0x06, `push %es`, removed in long mode), so the Listing walk attempts the
// fall-through, `decode_one` fails, and the address never becomes an instruction
// start.  The legacy evidence tally reads that as "the call has no valid
// fall-through" at all three sites, hits the >=3 threshold, and concludes
// `retseven` never returns.  `usesret` -- whose `return retseven() + 3` is real,
// reachable code -- is then truncated to `retseven(); // no-return`.
//
// With `noreturn_discstrict` on (the default) only positive evidence counts: the
// successor of each `call retseven` is an ordinary undecoded byte inside .text,
// which is neither data nor another function's entry, so `retseven` collects no
// votes and `usesret` keeps its tail.
//
// Nothing is byte-patched after the link, so the fixture rebuilds reproducibly:
//
//   gcc -no-pie -nostdlib -e _start -o noreturn_discstrict_x86_64 \
//       noreturn_discstrict_x86_64.s
	.text

// The victim: a function that returns a value on every path.
	.globl	retseven
	.type	retseven, @function
retseven:
	movl	$7, %eax
	ret
	.size	retseven, .-retseven

// Vote 1: `call retseven` followed by a byte the x86-64 decoder rejects.
	.globl	gap1
	.type	gap1, @function
gap1:
	call	retseven
	.byte	0x06
	ret
	.size	gap1, .-gap1

// Vote 2.
	.globl	gap2
	.type	gap2, @function
gap2:
	call	retseven
	.byte	0x06
	ret
	.size	gap2, .-gap2

// Vote 3 -- the threshold.
	.globl	gap3
	.type	gap3, @function
gap3:
	call	retseven
	.byte	0x06
	ret
	.size	gap3, .-gap3

// The collateral: a caller with a perfectly ordinary live tail.
	.globl	usesret
	.type	usesret, @function
usesret:
	pushq	%rbx
	call	retseven
	movl	%eax, %ebx
	addl	$3, %ebx
	movl	%ebx, %eax
	popq	%rbx
	ret
	.size	usesret, .-usesret

	.globl	_start
	.type	_start, @function
_start:
	call	gap1
	call	gap2
	call	gap3
	call	usesret
	movl	%eax, %edi
	movl	$60, %eax
	syscall
	// exit(2) never comes back, but the decoder does not know that; a
	// terminator keeps the walk inside the mapped image on both arms.
	ret
	.size	_start, .-_start

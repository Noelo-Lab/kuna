// Fixture for the `symbolnamerepair` gate: a symbol whose name is degenerate as
// a `::`-qualified path, with no mangling involved.  `a::::b` splits to an EMPTY
// middle component, which Database::attach_scope rejects -- and because the
// symbol table is installed inside `load file`, that error aborts the ENTIRE
// architecture build rather than costing one symbol.
//
// The name is quoted so the assembler takes it verbatim; nothing is byte-patched
// after the link, so the fixture rebuilds reproducibly:
//
//   gcc -no-pie -nostdlib -e main -o hostile_scope_x86_64 hostile_scope_x86_64.s
	.text
	.globl	main
	.type	main, @function
main:
	xorl	%eax, %eax
	ret
	.size	main, .-main

	.globl	"a::::b"
	.type	"a::::b", @function
"a::::b":
	movl	$7, %eax
	ret
	.size	"a::::b", .-"a::::b"

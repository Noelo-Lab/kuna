// Fixture for the `symbolnamechars` gate (GH-340): a symbol table whose names
// restructure the C document they are printed into, and two names that are
// distinct in the binary but collapse onto one `String` when decoded lossily.
//
// Five function names and one data name, each a different half of the defect:
//
//   a*/b   closes the `// Function: … @ <addr>` header comment early
//   a//b   comments out the rest of the line it lands on
//   a\nb   splits the header, the .h prototype, the definition, the call site
//          and the .asm label across two lines each
//   a\x80b } two DIFFERENT symbols at two addresses that `from_utf8_lossy`
//   a\x81b } maps to the SAME string, so the export carries a C redefinition
//   d*/x   the data half of the same walk (an `STT_OBJECT`)
//
// BUILD (reproducible, two steps -- there is no one-step form):
//
//   gcc -no-pie -nostdlib -e main -o hostile_symname_x86_64.tmp \
//       hostile_symname_x86_64.s
//   python3 hostile_symname_x86_64_patch.py hostile_symname_x86_64.tmp \
//       hostile_symname_x86_64
//   chmod +x hostile_symname_x86_64 && rm hostile_symname_x86_64.tmp
//
// The quoted `"a*/b"` / `"a//b"` / `"d*/x"` names assemble verbatim, but GNU as
// will NOT put a raw 0x0a, 0x80 or 0x81 inside a symbol name -- a `\200` in a
// quoted name is taken as four literal characters (with a warning). So those
// three are declared here as same-length ASCII placeholders (`aQb`, `aWb`,
// `aXb`) and the checked-in patch script rewrites the middle byte of each in
// `.strtab`. That is a departure from the sibling `hostile_scope_x86_64.s`,
// which needs no patching; the script is checked in beside this file so the
// fixture rebuilds from source rather than being magic bytes.
	.text
	.globl	main
	.type	main, @function
main:
	call	"a*/b"
	call	"a//b"
	call	"aQb"
	call	"aWb"
	call	"aXb"
	movl	"d*/x"(%rip), %eax
	ret
	.size	main, .-main

	.globl	"a*/b"
	.type	"a*/b", @function
"a*/b":
	movl	$1, %eax
	ret
	.size	"a*/b", .-"a*/b"

	.globl	"a//b"
	.type	"a//b", @function
"a//b":
	movl	$2, %eax
	ret
	.size	"a//b", .-"a//b"

	.globl	"aQb"
	.type	"aQb", @function
"aQb":
	movl	$3, %eax
	ret
	.size	"aQb", .-"aQb"

	.globl	"aWb"
	.type	"aWb", @function
"aWb":
	movl	$4, %eax
	ret
	.size	"aWb", .-"aWb"

	.globl	"aXb"
	.type	"aXb", @function
"aXb":
	movl	$5, %eax
	ret
	.size	"aXb", .-"aXb"

	.data
	.globl	"d*/x"
	.type	"d*/x", @object
	.size	"d*/x", 4
"d*/x":
	.long	9

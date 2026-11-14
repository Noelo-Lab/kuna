# Stripped PE32 i386 fixture for kuna_calleereginputs (tests/stages/kuna-calleereginputs.xml).
#
# thiscall_pe_i386.cpp is compiled by Visual C++ 2005 SP1 (`cl /O2 /c`, output
# renamed thiscall_pe_i386_c.obj). The stub below stands in for the
# incremental-link thunk MSVC routes the call to V::len2 through.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c thiscall_pe_i386.s -o stub.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:thiscall_pe_i386.exe \
#     thiscall_pe_i386_c.obj stub.obj

	.intel_syntax noprefix
	.text
	.globl	"?len2@V@@QBEMXZ"
"?len2@V@@QBEMXZ":
	.byte	0xe9
	.long	"?len2_impl@V@@QBEMXZ" - . - 4
	.data
	.globl	__fltused
__fltused:	.long	0x9875

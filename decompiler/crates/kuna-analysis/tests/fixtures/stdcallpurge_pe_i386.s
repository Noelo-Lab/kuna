# Stripped PE32 i386 fixture for kuna_calleepurge (tests/stages/kuna-calleepurge.xml).
#
# stdcallpurge_pe_i386.c is compiled by Visual C++ 2005 SP1 (`cl /O2 /c`, output
# renamed stdcallpurge_pe_i386_c.obj). The two `jmp` stubs below stand in for the
# incremental-link thunks MSVC routes calls through, so `split` (0x401080) reaches
# both __stdcall callees only through a thunk.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c stdcallpurge_pe_i386.s -o thunks.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:stdcallpurge_pe_i386.exe \
#     stdcallpurge_pe_i386_c.obj thunks.obj

	.intel_syntax noprefix
	.text
	.globl	"_dist@20"
"_dist@20":
	jmp	"_dist_impl@20"
	.globl	"_corte@24"
"_corte@24":
	jmp	"_corte_impl@24"
	.data
	.globl	__fltused
__fltused:	.long	0x9875

# kuna fixture: a constructor run on a local through `lea ecx,[local]`, whose
# own code reads ECX and leaves it alone, so the caller (as MSVC /GL code
# does) reuses ECX after the call: the local's address then has a second use.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c ctorthis_pe_i386.s -o c.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:ctorthis_pe_i386.exe c.obj
    .intel_syntax noprefix
    .text
    .globl _place
_place:                         # void place(const float *src, Grid *out)
    sub esp, 0x40
    push edi
    mov edi, dword ptr [esp + 0x48]
    lea ecx, [esp + 0x8]
    call _grid_init             # Grid::Grid(), this in ecx, ecx preserved
    fld dword ptr [edi]
    push ecx
    mov eax, ecx
    fstp dword ptr [esp]
    push eax
    push dword ptr [esp + 0x54]
    call _grid_fill             # void grid_fill(Grid *out, Grid *g, float f)
    add esp, 0xc
    pop edi
    add esp, 0x40
    ret

    .p2align 4, 0xcc
_grid_init:
    fldz
    mov eax, ecx
    push esi
    lea edx, [eax + 4]
    mov esi, 0xf
.Lloop:
    fst dword ptr [edx]
    add edx, 4
    sub esi, 1
    jns .Lloop
    fstp st(0)
    mov dword ptr [eax], 0
    pop esi
    ret

    .p2align 4, 0xcc
_grid_fill:
    mov eax, dword ptr [esp + 4]
    mov ecx, dword ptr [esp + 8]
    fld dword ptr [esp + 12]
    fstp dword ptr [eax]
    mov dword ptr [ecx], 1
    ret

    .p2align 4, 0xcc
    .globl _start
_start:
    lea eax, [_out]
    push eax
    lea eax, [_src]
    push eax
    call _place
    add esp, 8
    ret

    .data
_src:
    .long 0x3f800000
_out:
    .space 0x44

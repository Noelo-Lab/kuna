; Clean-room: a MinGW-style i386 caller of the C++ allocation operators that
; libstdc++-6.dll exports. They are __cdecl, so the caller pops nothing and
; GCC's fixed frame (arguments written with `mov [esp+k]`) stays put.
;
;   int cmpfree(const char *a, const char *b, size_t n)
;   {
;       char *t = new char[n];
;       copy3(t, a, n);
;       int r = cmp3(t, b, n);
;       delete[] t;
;       return r;
;   }
;   void roundtrip(long x) { Node *v = new Node; fill2(v, x); delete v; }
;
; Build: nasm -f bin -I . -o cxx_alloc_pe_i386.exe cxx_alloc_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

cmpfree:                                ; 0x401000
    push edi
    push esi
    push ebx
    sub esp, 0x10
    mov ebx, [esp+0x28]                 ; n
    mov [esp], ebx
    call znaj_thunk
    mov esi, eax
    mov eax, [esp+0x20]                 ; a
    mov [esp+8], ebx
    mov [esp], esi
    mov [esp+4], eax
    call copy3
    mov eax, [esp+0x24]                 ; b
    mov [esp+8], ebx
    mov [esp], esi
    mov [esp+4], eax
    call cmp3
    mov [esp], esi
    mov edi, eax
    call zdapv_thunk
    add esp, 0x10
    mov eax, edi
    pop ebx
    pop esi
    pop edi
    ret

    align 16, db 0xcc
roundtrip:
    push ebx
    sub esp, 0x18
    mov dword [esp], 8
    call znwj_thunk
    mov ebx, eax
    mov eax, [esp+0x20]                 ; x
    mov [esp], ebx
    mov [esp+4], eax
    call fill2
    mov dword [esp+4], 8
    mov [esp], ebx
    call zdlpvj_thunk
    add esp, 0x18
    pop ebx
    ret

    align 16, db 0xcc
copy3:
    push edi
    push esi
    mov edi, [esp+0xc]
    mov esi, [esp+0x10]
    mov ecx, [esp+0x14]
    rep movsb
    pop esi
    pop edi
    ret

    align 16, db 0xcc
cmp3:
    mov eax, [esp+4]
    mov edx, [esp+8]
    mov ecx, [esp+0xc]
    movzx eax, byte [eax+ecx-1]
    movzx edx, byte [edx+ecx-1]
    sub eax, edx
    ret

    align 16, db 0xcc
fill2:
    mov eax, [esp+4]
    mov edx, [esp+8]
    mov [eax], edx
    mov dword [eax+4], 0
    ret

    align 16, db 0xcc
znwj_thunk:
    jmp [__imp__Znwj]
    align 8, db 0x90
znaj_thunk:
    jmp [__imp__Znaj]
    align 8, db 0x90
zdlpvj_thunk:
    jmp [__imp__ZdlPvj]
    align 8, db 0x90
zdapv_thunk:
    jmp [__imp__ZdaPv]

    align 16, db 0xcc
entry:
    push 4
    push str_b
    push str_a
    call cmpfree
    mov [esp], eax
    call roundtrip
    add esp, 0xc
    ret

section .rdata
str_a: db 'abcd', 0
str_b: db 'abce', 0
    align 4, db 0
import_dir:
    dd RVA(ilt_cxx), 0, 0, RVA(dll_cxx), RVA(iat_cxx)
    times 5 dd 0
import_dir_end:
iat:
iat_cxx:
__imp__Znwj:   dd RVA(hn_znwj)
__imp__Znaj:   dd RVA(hn_znaj)
__imp__ZdlPvj: dd RVA(hn_zdlpvj)
__imp__ZdaPv:  dd RVA(hn_zdapv)
    dd 0
iat_end:
ilt_cxx: dd RVA(hn_znwj), RVA(hn_znaj), RVA(hn_zdlpvj), RVA(hn_zdapv), 0
hn_znwj: dw 0
    db '_Znwj', 0
    align 2, db 0
hn_znaj: dw 0
    db '_Znaj', 0
    align 2, db 0
hn_zdlpvj: dw 0
    db '_ZdlPvj', 0
    align 2, db 0
hn_zdapv: dw 0
    db '_ZdaPv', 0
    align 2, db 0
dll_cxx: db 'libstdc++-6.dll', 0

section .data
    dd 0

%include "pe32_data_end.inc"

; Clean-room repro: a declared __stdcall import's pop, in a function whose
; other pops the stack solver only guesses.
;
;   int launch(void)                     (0x401000)
;   {
;       char path[256];
;       GetModuleFileNameA(NULL, path, sizeof path);
;       char *end = path + strlen(path);  /* MSVC's pointer-difference idiom */
;       while (end > path && *end != '.')
;           *end-- = 0;
;       *end = 0;
;       int fd = open_like(path, 0);
;       if (fd == -1)
;           report("Cannot open %s\n", path);
;       return fd;
;   }
;
; launch is the MSVC shape of setuptools' cli-32.exe launcher: the length is
; `end - (path + 1)` taken from two pointers into `path`. open_like (0x401080,
; __cdecl) reads its arguments after calling the import xopen, whose pop is
; unknown, so neither its own pop nor the stack bytes it reads can be read off
; its code; report (0x401090) is __cdecl.
;
; Build: nasm -f bin -I . -o declaredpop_pe_i386.exe declaredpop_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

launch:                                 ; 0x401000
    sub esp, 0x108
    push 0x100
    lea ecx, [esp+0x8]
    push ecx
    push 0
    call [__imp_GetModuleFileNameA]
    lea eax, [esp+0x4]
    lea edx, [eax+0x1]
.len:
    mov cl, [eax]
    inc eax
    test cl, cl
    jnz .len
    sub eax, edx
    lea eax, [esp+eax+0x4]
    lea edx, [esp+0x4]
    cmp eax, edx
    jbe .cut
.back:
    cmp byte [eax], 0x2e
    je .cut
    mov byte [eax], 0
    dec eax
    lea ecx, [esp+0x4]
    cmp eax, ecx
    ja .back
.cut:
    mov byte [eax], 0
    push esi
    lea edx, [esp+0x8]
    push 0
    push edx
    call open_like
    mov esi, eax
    add esp, 0x8
    cmp esi, -1
    jne .done
    lea eax, [esp+0x8]
    push eax
    push fmt
    call report
    add esp, 0x8
.done:
    mov eax, esi
    pop esi
    add esp, 0x108
    ret

    align 16, db 0xcc
open_like:                              ; __cdecl
    call [__imp_OpenFile]
    add eax, [esp+4]
    add eax, [esp+8]
    ret

    align 16, db 0xcc
report:                                 ; __cdecl
    mov eax, [esp+4]
    mov ecx, [esp+8]
    mov [ecx], al
    or eax, -1
    ret

    align 16, db 0xcc
entry:
    call launch
    ret

section .rdata
    align 4, db 0
import_dir:
    dd RVA(ilt_k32), 0, 0, RVA(dll_k32), RVA(iat_k32)
    dd RVA(ilt_x), 0, 0, RVA(dll_x), RVA(iat_x)
    times 5 dd 0
import_dir_end:
iat:
iat_k32:
__imp_GetModuleFileNameA: dd RVA(hn_getmodulefilenamea)
    dd 0
iat_x:
__imp_OpenFile: dd RVA(hn_openfile)
    dd 0
iat_end:
ilt_k32: dd RVA(hn_getmodulefilenamea), 0
ilt_x: dd RVA(hn_openfile), 0
hn_getmodulefilenamea: dw 0
    db 'GetModuleFileNameA', 0
    align 2, db 0
hn_openfile: dw 0
    db 'xopen', 0
    align 2, db 0
dll_k32: db 'kernel32.dll', 0
dll_x: db 'x.dll', 0
fmt: db 'Cannot open %s', 10, 0

section .data
    dd 0

%include "pe32_data_end.inc"

; Clean-room repro: a fixed-frame i386 caller of __stdcall imports
; (Noelo-Lab/kuna#1029).
;
; GCC/MinGW reserves the outgoing-argument area once and writes each call's
; arguments with `mov [esp+k]`. memset (msvcrt.dll, __cdecl) pops nothing;
; after each advapi32 call (__stdcall) the caller lowers ESP by the bytes the
; callee popped (`sub esp,0x14`), so the frame stays fixed.
;
;   int GetFolder(char *path, const char *folder)
;   {
;       HKEY key; DWORD size = MAX_PATH;
;       memset(path, 0, MAX_PATH);
;       if (RegOpenKeyExA(HKEY_CURRENT_USER, "Software\\Kuna", 0, KEY_QUERY_VALUE, &key))
;           return 0;
;       if (RegQueryValueExA(key, folder, NULL, NULL, (BYTE *)path, &size)) {
;           RegCloseKey(key);
;           return 0;
;       }
;       RegCloseKey(key);
;       return 1;
;   }
;
; Build: nasm -f bin -I . -o calleepopframe_pe_i386.exe calleepopframe_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

f:                                      ; 0x401000
    push ebx
    sub esp, 0x38
    mov ebx, [esp+0x40]                 ; path
    mov dword [esp+8], 0x104
    mov dword [esp+4], 0
    mov [esp], ebx
    mov dword [esp+0x2c], 0x104         ; size
    call memset_thunk
    lea eax, [esp+0x28]                 ; &key
    mov dword [esp+0xc], 1
    mov [esp+0x10], eax
    mov dword [esp+8], 0
    mov dword [esp+4], subkey
    mov dword [esp], 0x80000001
    call [__imp_RegOpenKeyExA]
    sub esp, 0x14
    test eax, eax
    je .opened
    xor eax, eax
.out:
    add esp, 0x38
    pop ebx
    ret
.opened:
    lea eax, [esp+0x2c]                 ; &size
    mov [esp+0x10], ebx
    mov [esp+0x14], eax
    mov eax, [esp+0x44]                 ; folder
    mov dword [esp+0xc], 0
    mov [esp+4], eax
    mov eax, [esp+0x28]                 ; key
    mov dword [esp+8], 0
    mov [esp], eax
    call [__imp_RegQueryValueExA]
    sub esp, 0x18
    test eax, eax
    mov eax, [esp+0x28]                 ; key
    mov [esp], eax
    jne .failed
    call [__imp_RegCloseKey]
    mov eax, 1
    sub esp, 4
    add esp, 0x38
    pop ebx
    ret
.failed:
    call [__imp_RegCloseKey]
    xor eax, eax
    sub esp, 4
    jmp .out

    align 16, db 0xcc
memset_thunk:
    jmp [__imp_memset]

    align 16, db 0xcc
entry:
    push folder
    push buf
    call f
    add esp, 8
    ret

section .rdata
subkey: db 'Software\Kuna', 0
folder: db 'Desktop', 0
    align 4, db 0
import_dir:
    dd RVA(ilt_advapi), 0, 0, RVA(dll_advapi), RVA(iat_advapi)
    dd RVA(ilt_msvcrt), 0, 0, RVA(dll_msvcrt), RVA(iat_msvcrt)
    times 5 dd 0
import_dir_end:
iat:
iat_advapi:
__imp_RegCloseKey:      dd RVA(hn_close)
__imp_RegOpenKeyExA:    dd RVA(hn_open)
__imp_RegQueryValueExA: dd RVA(hn_query)
    dd 0
iat_msvcrt:
__imp_memset:           dd RVA(hn_memset)
    dd 0
iat_end:
ilt_advapi: dd RVA(hn_close), RVA(hn_open), RVA(hn_query), 0
ilt_msvcrt: dd RVA(hn_memset), 0
hn_close: dw 0
    db 'RegCloseKey', 0
    align 2, db 0
hn_open: dw 0
    db 'RegOpenKeyExA', 0
    align 2, db 0
hn_query: dw 0
    db 'RegQueryValueExA', 0
    align 2, db 0
hn_memset: dw 0
    db 'memset', 0
    align 2, db 0
dll_advapi: db 'ADVAPI32.dll', 0
dll_msvcrt: db 'msvcrt.dll', 0

section .data
buf: times 0x104 db 0

%include "pe32_data_end.inc"

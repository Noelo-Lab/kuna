; Clean-room repro: an x87 intrinsic imported from the C runtime keeps its
; ST0 operand. A `msvcr*.dll` import is otherwise seeded `__cdecl`, and a locked
; `long double _CIcos(long double)` under `__cdecl` reads its operand as ten
; bytes of the caller's stack, taking the argument pushed for Sleep with it.
;
; void spin(double *v) { Sleep(1); v[1] = cos(v[0]); }   (VS2008 /MD shape)
;
; Build: nasm -f bin -I . -o x87import_pe_i386.exe x87import_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

spin:                                   ; 0x401000
    push esi
    push 1
    mov esi, [esp+0xc]
    call [__imp_Sleep]
    fld qword [esi]
    call cicos_thunk
    fstp qword [esi+8]
    pop esi
    ret

    align 16, db 0xcc
cicos_thunk:                            ; 0x401020
    jmp [__imp__CIcos]

    align 16, db 0xcc
entry:
    push 0
    call spin
    add esp, 4
    ret

section .rdata
import_dir:
    dd RVA(ilt_k32), 0, 0, RVA(dll_k32), RVA(iat_k32)
    dd RVA(ilt_crt), 0, 0, RVA(dll_crt), RVA(iat_crt)
    times 5 dd 0
import_dir_end:
iat:
iat_k32:
__imp_Sleep: dd RVA(hn_sleep)
    dd 0
iat_crt:
__imp__CIcos: dd RVA(hn_cicos)
    dd 0
iat_end:
ilt_k32: dd RVA(hn_sleep), 0
ilt_crt: dd RVA(hn_cicos), 0
hn_sleep: dw 0
    db 'Sleep', 0
    align 2, db 0
hn_cicos: dw 0
    db '_CIcos', 0
    align 2, db 0
dll_k32: db 'kernel32.dll', 0
dll_crt: db 'msvcr90.dll', 0

%include "pe32_data_end.inc"

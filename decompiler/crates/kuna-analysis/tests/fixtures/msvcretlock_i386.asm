; Clean-room repro: an imported constructor must not inherit a void return.
;
; arr.dll exports a class `Arr` with a void member and a default constructor
; that share one C signature, `(Arr *this)` under __thiscall:
;   void Arr::Reset()   ?Reset@Arr@@QAEXXZ     explicit void return
;   Arr::Arr()          ??0Arr@@QAE@XZ         no return type in the name
;   void Use(Arr *)     ?Use@@YAXPAVArr@@@Z    __cdecl
; Reset's slot comes first, so its locked void callable type is built before
; the constructor's input-only one.
;
; Build: nasm -f bin -I . -o msvcretlock_i386.exe msvcretlock_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

; Arr *Make(void *mem) { Arr *p = new (mem) Arr(); p->Reset(); Use(p); return p; }
make:                                   ; 0x401000
    push ebp
    mov ebp, esp
    push esi
    mov ecx, [ebp+8]
    call [__imp_Arr_ctor]               ; MSVC returns `this` in eax
    mov esi, eax
    mov ecx, eax
    call [__imp_Arr_Reset]
    push esi
    call [__imp_Use]
    add esp, 4
    mov eax, esi
    pop esi
    pop ebp
    ret

    align 16, db 0xcc
entry:
    push 0
    call make
    add esp, 4
    ret

section .rdata
import_dir:
    dd RVA(ilt_arr), 0, 0, RVA(dll_arr), RVA(iat_arr)
    times 5 dd 0
import_dir_end:
iat:
iat_arr:
__imp_Arr_Reset: dd RVA(hn_reset)
__imp_Arr_ctor:  dd RVA(hn_ctor)
__imp_Use:       dd RVA(hn_use)
    dd 0
iat_end:
ilt_arr: dd RVA(hn_reset), RVA(hn_ctor), RVA(hn_use), 0
hn_reset: dw 0
    db '?Reset@Arr@@QAEXXZ', 0
    align 2, db 0
hn_ctor: dw 0
    db '??0Arr@@QAE@XZ', 0
    align 2, db 0
hn_use: dw 0
    db '?Use@@YAXPAVArr@@@Z', 0
    align 2, db 0
dll_arr: db 'arr.dll', 0

%include "pe32_data_end.inc"

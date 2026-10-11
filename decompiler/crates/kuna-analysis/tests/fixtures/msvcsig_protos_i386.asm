; Clean-room repro: MSVC-mangled imports get no prototype.
;
; arr.dll exports a 4-byte handle class `Arr` (it has a destructor, so MSVC
; passes and returns it in memory):
;   int Arr::IsValid()           ?IsValid@Arr@@QAEHXZ               __thiscall
;   Arr::Arr(Arr const &)        ??0Arr@@QAE@ABV0@@Z                __thiscall
;   void Note(long)              ?Note@@YAXJ@Z                      __cdecl
;   Arr MakeStack(Arr, Arr &, Arr &)
;                                ?MakeStack@@YA?AVArr@@V1@AAV1@1@Z  __cdecl
;
; Build: nasm -f bin -I . -o msvcsig_protos_i386.exe msvcsig_protos_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

; int Check(Arr *a) { int v = a->IsValid(); Note(v); return v; }
check:                                  ; 0x401000
    push ebp
    mov ebp, esp
    push esi
    mov ecx, [ebp+8]                    ; this
    call [__imp_Arr_IsValid]
    mov esi, eax
    push eax
    call [__imp_Note]
    add esp, 4
    mov eax, esi
    pop esi
    pop ebp
    ret

    align 16, db 0xcc
; Arr Build(Arr &x, Arr &y) { return MakeStack(x, x, y); }
build:                                  ; 0x401020
    push ebp
    mov ebp, esp
    push dword [ebp+16]                 ; Arr &y
    push dword [ebp+12]                 ; Arr &x
    push ecx                            ; room for the by-value copy of x
    mov ecx, esp
    push dword [ebp+12]
    call [__imp_Arr_copy]               ; copy-construct x into the slot
    push dword [ebp+8]                  ; hidden return pointer
    call [__imp_MakeStack]
    add esp, 0x10
    mov eax, [ebp+8]
    pop ebp
    ret

    align 16, db 0xcc
entry:
    push 0
    call check
    add esp, 4
    push 0
    push 0
    push 0
    call build
    add esp, 12
    ret

section .rdata
import_dir:
    dd RVA(ilt_arr), 0, 0, RVA(dll_arr), RVA(iat_arr)
    times 5 dd 0
import_dir_end:
iat:
iat_arr:
__imp_Arr_IsValid: dd RVA(hn_isvalid)
__imp_Arr_copy:    dd RVA(hn_copy)
__imp_Note:        dd RVA(hn_note)
__imp_MakeStack:   dd RVA(hn_makestack)
    dd 0
iat_end:
ilt_arr: dd RVA(hn_isvalid), RVA(hn_copy), RVA(hn_note), RVA(hn_makestack), 0
hn_isvalid: dw 0
    db '?IsValid@Arr@@QAEHXZ', 0
    align 2, db 0
hn_copy: dw 0
    db '??0Arr@@QAE@ABV0@@Z', 0
    align 2, db 0
hn_note: dw 0
    db '?Note@@YAXJ@Z', 0
    align 2, db 0
hn_makestack: dw 0
    db '?MakeStack@@YA?AVArr@@V1@AAV1@1@Z', 0
    align 2, db 0
dll_arr: db 'arr.dll', 0

%include "pe32_data_end.inc"

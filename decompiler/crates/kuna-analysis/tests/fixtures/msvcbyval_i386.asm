; MSVC __cdecl imports that take a class by value, at call sites whose caller
; cleanup does and does not state the class's size.
;
; arr.dll exports two classes with copy constructors (so MSVC passes them in
; memory, copy-constructed into the outgoing slot): `Arr` (4 bytes) and `Pair`
; (8 bytes).
;   Arr::Arr(Arr const &)              ??0Arr@@QAE@ABV0@@Z                __thiscall
;   Pair::Pair(Pair const &)           ??0Pair@@QAE@ABV0@@Z               __thiscall
;   void Note(long)                    ?Note@@YAXJ@Z                      __cdecl
;   Arr MakeStack(Arr, Arr &, Arr &)   ?MakeStack@@YA?AVArr@@V1@AAV1@1@Z  __cdecl
;   void Join(Pair, Pair &)            ?Join@@YAXVPair@@AAV1@@Z           __cdecl
;
; Build: nasm -f bin -I . -o msvcbyval_i386.exe msvcbyval_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

; void Wide(Pair &p) { Join(p, p); }
wide:                                   ; 0x401000
    push ebp
    mov ebp, esp
    push dword [ebp+8]                  ; Pair &
    sub esp, 8                          ; room for the by-value copy of p
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Pair_copy]
    call [__imp_Join]
    add esp, 0xc
    pop ebp
    ret

    align 16, db 0xcc
; Arr Late(Arr &x, Arr &y) { Note(1); return MakeStack(x, x, y); }
; Note's pop is deferred into MakeStack's cleanup.
late:                                   ; 0x401020
    push ebp
    mov ebp, esp
    push 1
    call [__imp_Note]
    push dword [ebp+16]
    push dword [ebp+12]
    push ecx
    mov ecx, esp
    push dword [ebp+12]
    call [__imp_Arr_copy]
    push dword [ebp+8]
    call [__imp_MakeStack]
    add esp, 0x14
    mov eax, [ebp+8]
    pop ebp
    ret

    align 16, db 0xcc
; void Folded(Arr &x, Arr &y) { Arr r = MakeStack(y, x, y); }
; No frame pointer: the cleanup also releases the local `r`.
folded:                                 ; 0x401050
    push ecx                            ; r
    push dword [esp+0xc]                ; y
    push dword [esp+0xc]                ; x
    push ecx                            ; room for the by-value copy of y
    mov ecx, esp
    push dword [esp+0x18]
    call [__imp_Arr_copy]
    lea eax, [esp+0xc]                  ; &r
    push eax
    call [__imp_MakeStack]
    add esp, 0x14
    ret

    align 16, db 0xcc
entry:
    push 0
    call wide
    add esp, 4
    push 0
    push 0
    push 0
    call late
    add esp, 12
    push 0
    push 0
    call folded
    add esp, 8
    ret

section .rdata
import_dir:
    dd RVA(ilt_arr), 0, 0, RVA(dll_arr), RVA(iat_arr)
    times 5 dd 0
import_dir_end:
iat:
iat_arr:
__imp_Arr_copy:    dd RVA(hn_copy)
__imp_Pair_copy:   dd RVA(hn_pcopy)
__imp_Note:        dd RVA(hn_note)
__imp_MakeStack:   dd RVA(hn_makestack)
__imp_Join:        dd RVA(hn_join)
    dd 0
iat_end:
ilt_arr: dd RVA(hn_copy), RVA(hn_pcopy), RVA(hn_note), RVA(hn_makestack), RVA(hn_join), 0
hn_copy: dw 0
    db '??0Arr@@QAE@ABV0@@Z', 0
    align 2, db 0
hn_pcopy: dw 0
    db '??0Pair@@QAE@ABV0@@Z', 0
    align 2, db 0
hn_note: dw 0
    db '?Note@@YAXJ@Z', 0
    align 2, db 0
hn_makestack: dw 0
    db '?MakeStack@@YA?AVArr@@V1@AAV1@1@Z', 0
    align 2, db 0
hn_join: dw 0
    db '?Join@@YAXVPair@@AAV1@@Z', 0
    align 2, db 0
dll_arr: db 'arr.dll', 0

%include "pe32_data_end.inc"

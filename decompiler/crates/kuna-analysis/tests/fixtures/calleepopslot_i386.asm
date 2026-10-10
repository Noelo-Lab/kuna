; Clean-room repro: wrong stack depth after a callee pops a by-value class
; argument that was pushed before an earlier call.
;
; arr.dll exports a 4-byte handle class `Arr` with a destructor, so MSVC
; passes and returns it in memory:
;   Arr Load(long)               ?Load@@YA?AVArr@@J@Z     __cdecl
;   Arr Arr::operator=(Arr)      ??4Arr@@QAE?AV0@V0@@Z    __thiscall, ret 8
;   void Note(long)              ?Note@@YAXJ@Z            __cdecl
;
; MSVC builds the result of Load directly in the stack slot that is then
; the by-value argument of operator=, and operator= pops that slot.
;
; Control: delete the first Note call (the three lines marked [1]). The slot
; is then pushed from ECX's input value, which reads as a saved register, so
; the push run in front of Load stops below it.
;
; Build: nasm -f bin -I . -o calleepopslot_i386.exe calleepopslot_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

; void F(void) { Arr a, r; Note(1); a = Load(0x2a0017); Note(0x40000001); }
f:                                      ; 0x401000
    push ebp
    mov ebp, esp
    sub esp, 8                          ; a at [ebp-8], r at [ebp-4]
    push 1                              ; [1]
    call [__imp_Note]                   ; [1]
    add esp, 4                          ; [1]
    push ecx                            ; 4-byte slot for Load's result
    mov ecx, esp
    push 0x2a0017
    push ecx                            ; hidden return pointer
    call [__imp_Load]
    add esp, 8                          ; the slot stays on the stack
    lea eax, [ebp-4]
    push eax                            ; hidden return pointer
    lea ecx, [ebp-8]                    ; this
    call [__imp_Arr_assign]             ; pops the hidden pointer and the slot
    push 0x40000001
    call [__imp_Note]
    add esp, 4
    mov esp, ebp
    pop ebp
    ret

    align 16, db 0xcc
entry:
    call f
    ret

section .rdata
import_dir:
    dd RVA(ilt_arr), 0, 0, RVA(dll_arr), RVA(iat_arr)
    times 5 dd 0
import_dir_end:
iat:
iat_arr:
__imp_Arr_assign: dd RVA(hn_assign)
__imp_Load:       dd RVA(hn_load)
__imp_Note:       dd RVA(hn_note)
    dd 0
iat_end:
ilt_arr: dd RVA(hn_assign), RVA(hn_load), RVA(hn_note), 0
hn_assign: dw 0
    db '??4Arr@@QAE?AV0@V0@@Z', 0
    align 2, db 0
hn_load: dw 0
    db '?Load@@YA?AVArr@@J@Z', 0
    align 2, db 0
hn_note: dw 0
    db '?Note@@YAXJ@Z', 0
    align 2, db 0
dll_arr: db 'arr.dll', 0

%include "pe32_data_end.inc"

; Clean-room repro (GH-299): an AIF gap candidate that is a fragment of the
; known function around it.
;
; `dispatch` leaves a hole the recursive-descent walk cannot enter: its only
; way into `.case` is `jmp dword [ebp+12]`. `.case` opens with the same
; `mov eax,[esp+4]; add eax,imm8` prologue as the 22 leaf functions `start`
; calls and branches back into `dispatch` through `jmp .join`, so the
; Aggressive Instruction Finder accepts it as a function of its own.
;
; Control: `shared` is a real function behind a pointer in `.rdata`, after
; int3 padding, that also branches into `dispatch`'s tail. AIF finds it and
; it stays a function with `aifbracket` on.
;
; Build: nasm -f bin -I . -o aifbracket_pe_i386.exe aifbracket_pe_i386.asm

%define IMAGE_DLL 0
%define LEAVES 22
%include "pe32.inc"

section .text

entry:
start:                                  ; 0x401000
    push ebp
    mov ebp, esp
%assign i 0
%rep LEAVES
    call leaf%[i]
%assign i i+1
%endrep
    push dword [handlers]
    push 1
    call dispatch
    add esp, 8
    pop ebp
    ret

%assign i 0
%rep LEAVES
    align 16, db 0xcc
leaf%[i]:
    mov eax, [esp+4]
    add eax, i + 3
    ret
%assign i i+1
%endrep

    align 16, db 0xcc
dispatch:                               ; 0x4011f0
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    test eax, eax
    jz .join
    jmp dword [ebp+12]
.case:                                  ; 0x4011fd
    mov eax, [esp+4]
    add eax, 2
    jmp .join
.join:                                  ; 0x401206
    pop ebp
    ret

    align 16, db 0xcc
shared:                                 ; 0x401210
    mov eax, [esp+4]
    add eax, 9
    jmp dispatch.join

section .rdata
handlers:
    dd shared
import_dir:
import_dir_end:
iat:
iat_end:
    dd 0

%include "pe32_end.inc"

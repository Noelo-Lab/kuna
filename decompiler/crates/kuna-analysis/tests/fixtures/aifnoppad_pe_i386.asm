; Clean-room repro (GH-299): AIF gap candidates that start on nop padding.
;
; Five called `stub`s open with nine nops, as Wine's unimplemented-function
; stubs do, so `nop, nop` is a prologue fingerprint the discovered functions
; share. `relay` ends unaligned and eight nops pad to the hot-patchable entry
; of the called function `hot`; the Aggressive Instruction Finder accepts the
; padding as a function of its own, falling through into `hot`.
;
; `lone` is never referenced and opens with a prologue nothing else shares;
; AIF finds it only through the nops in front of it, and plants the entry on
; one of them. `dead`, also unreferenced, opens with `mov edi,edi` straight
; after a return: that is a hot-patch entry, not padding, and stays a function.
;
; Build: nasm -f bin -I . -o aifnoppad_pe_i386.exe aifnoppad_pe_i386.asm

%define IMAGE_DLL 0
%define STUBS 5
%define LEAVES 16
%include "pe32.inc"

section .text

entry:
start:                                  ; 0x401000
    push ebp
    mov ebp, esp
%assign i 0
%rep STUBS
    call stub%[i]
%assign i i+1
%endrep
%assign i 0
%rep LEAVES
    call leaf%[i]
%assign i i+1
%endrep
    push 1
    call relay
    call hot
    pop ebp
    ret

%assign i 0
%rep STUBS
    align 16, db 0xcc
stub%[i]:
    times 9 nop
    mov eax, [esp+4]
    add eax, i + 1
    ret
%assign i i+1
%endrep

%assign i 0
%rep LEAVES
    align 16, db 0xcc
leaf%[i]:
    db 0x8b, 0xff                      ; mov edi,edi (MSVC hot-patch entry)
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    add eax, i + 3
    pop ebp
    ret
%assign i i+1
%endrep

    align 16, db 0xcc
relay:
    mov eax, 7
    ret 4
    times 8 nop
hot:
    db 0x8b, 0xff                      ; mov edi,edi (MSVC hot-patch entry)
    push ebp
    mov ebp, esp
    xor eax, eax
    pop ebp
    ret
dead:
    db 0x8b, 0xff                      ; mov edi,edi (MSVC hot-patch entry)
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    add eax, 40
    pop ebp
    ret
    times 6 nop
lone:
    xor eax, eax
    inc eax
    shl eax, 3
    ret

section .rdata
import_dir:
import_dir_end:
iat:
iat_end:
    dd 0

%include "pe32_end.inc"

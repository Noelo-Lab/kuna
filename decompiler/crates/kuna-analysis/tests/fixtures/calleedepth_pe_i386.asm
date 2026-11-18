; Clean-room repro: a caller of a stack-moving helper is not itself stack-moving
; (kuna_calleepurge), at any depth the walk reaches it.
;
;   seh_prolog4      the CRT-shaped frame helper of calleeframes_pe_i386.asm: it
;                    lowers its caller's stack and sets EBP for it.
;   seh_body(x)      __cdecl, framed by seh_prolog4 and ending in the CRT's
;                    inline epilogue (`leave` on the EBP the helper set). Walked
;                    with its helper call cut off by the depth limit, its return
;                    is unplaced by its own `leave`.
;   wrap(x)          __cdecl, calls seh_body on one path; both paths meet at one
;                    `ret`. Its only imprecision comes from that call.
;   framed(x)        __stdcall, EBP frame, calls wrap; `mov esp,ebp`, `ret 4`.
;   outer(x)         __stdcall, FPO: `push x; call framed; ret 4`.
;   start(a0)        outer(0x11), then probe(a0) read past outer's pop.
;
; Walking outer reaches seh_body at the last depth. framed's pop is proven only
; if wrap is not taken for a helper that rewrites EBP.
;
; Build: nasm -f bin -I . -o calleedepth_pe_i386.exe calleedepth_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

seh_prolog4:                            ; 0x401000
    push except_handler
    push dword [fs:0]
    mov eax, [esp+0x10]
    mov [esp+0x10], ebp
    lea ebp, [esp+0x10]
    sub esp, eax
    push ebx
    push esi
    push edi
    mov eax, [cookie]
    xor [ebp-4], eax
    xor eax, ebp
    push eax
    mov [ebp-0x18], esp
    push dword [ebp-8]
    mov eax, [ebp-4]
    mov dword [ebp-4], -2
    mov [ebp-8], eax
    lea eax, [ebp-0x10]
    mov [fs:0], eax
    ret

except_handler:
    xor eax, eax
    ret

    align 16, db 0xcc
seh_body:
    push 0x10
    push scopetable
    call seh_prolog4
    mov eax, [ebp+8]
    add eax, 1
    mov [ebp-0x1c], eax
    mov dword [ebp-4], -2
    mov eax, [ebp-0x1c]
    mov ecx, [ebp-0x10]
    mov [fs:0], ecx
    pop ecx
    pop edi
    pop esi
    pop ebx
    leave
    ret

    align 16, db 0xcc
wrap:
    mov eax, [esp+4]
    test eax, eax
    jz .z
    push eax
    call seh_body
    add esp, 4
.z: xor eax, eax
    ret

    align 16, db 0xcc
framed:
    push ebp
    mov ebp, esp
    push dword [ebp+8]
    call wrap
    mov esp, ebp
    pop ebp
    ret 4

    align 16, db 0xcc
outer:
    push dword [esp+4]
    call framed
    ret 4

    align 16, db 0xcc
probe:
    mov eax, [esp+4]
    ret 4

    align 16, db 0xcc
start:
    push 0x11
    call outer
    push dword [esp+4]
    call probe
    ret

    align 16, db 0xcc
entry:
    push 0xa0
    call start
    add esp, 4
    ret

section .rdata
    align 4, db 0
scopetable: dd -2, 0, -0x2c, 0, -2, 0, 0
import_dir:
import_dir_end:
iat:
iat_end:

section .data
cookie: dd 0xbb40e64e

%include "pe32_data_end.inc"

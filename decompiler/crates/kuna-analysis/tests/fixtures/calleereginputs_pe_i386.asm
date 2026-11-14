; Clean-room repro: calls to callees that read ECX (and EDX) before writing
; them, where the caller's own data flow does not prove the register is an
; argument (kuna_calleereginputs).
;
;   obj_set(this, x)    __thiscall, stores x into *this, `ret 4`.
;   obj_init(this)      __thiscall, zeroes *this, `ret`.
;   fast2(a, b)         __fastcall, returns ECX + EDX.
;   plain(x)            __stdcall, `ret 4`; no register input.
;   cdecl_one(x)        __cdecl, `ret`; no register input.
;   two_args(a, b)      __stdcall, `ret 8`; no register input.
;
;   forward(x)          a method forwarding its own `this`: it never touches
;                       ECX before `call obj_set`.
;   inherited()         `mov ecx, esi` from an ESI it never set (a funclet's
;                       inherited register), then obj_set(this, 5).
;   twice(p)            pushes p for plain(p), then passes the same p as the
;                       `this` of obj_set(p, 1).
;   reload(p)           pushes p for cdecl_one(p), then loads the pushed slot
;                       back as the `this` of obj_set(p, 2) before cleaning up.
;   drift()             calls the import FooBar(3), then two_args(1, 2), whose
;                       pop the stack solver can only guess because the import's
;                       is a guess too, then obj_init on a local at the frame's
;                       bottom.
;   fastcaller()        stores fast2(1, 2).
;
; Build: nasm -f bin -I . -o calleereginputs_pe_i386.exe calleereginputs_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

obj_set:                                ; 0x401000
    mov eax, [esp+4]
    mov [ecx], eax
    ret 4

    align 16, db 0xcc
obj_init:                               ; 0x401010
    xor eax, eax
    mov [ecx], eax
    mov [ecx+4], eax
    ret

    align 16, db 0xcc
fast2:                                  ; 0x401020
    mov eax, ecx
    add eax, edx
    ret

    align 16, db 0xcc
plain:                                  ; 0x401030
    mov eax, [esp+4]
    add eax, 7
    ret 4

    align 16, db 0xcc
two_args:                               ; 0x401040
    mov eax, [esp+4]
    add eax, [esp+8]
    ret 8

    align 16, db 0xcc
forward:                                ; 0x401050
    push dword [esp+4]
    call obj_set
    ret 4

    align 16, db 0xcc
inherited:                              ; 0x401060
    push 5
    mov ecx, esi
    call obj_set
    ret

    align 16, db 0xcc
twice:                                  ; 0x401070
    push esi
    mov esi, [esp+8]
    push esi
    call plain
    mov ecx, esi
    push 1
    call obj_set
    pop esi
    ret 4

    align 16, db 0xcc
drift:                                  ; 0x401090
    push ebp
    mov ebp, esp
    sub esp, 0x10
    push 3
    call [__imp_FooBar]
    push 2
    push 1
    call two_args
    lea ecx, [esp]
    call obj_init
    leave
    ret

    align 16, db 0xcc
fastcaller:                             ; 0x4010c0
    mov ecx, 1
    mov edx, 2
    call fast2
    mov [buf], eax
    ret

    align 16, db 0xcc
cdecl_one:                              ; 0x4010e0
    mov eax, [esp+4]
    add eax, 9
    ret

    align 16, db 0xcc
reload:                                 ; 0x4010f0
    push esi
    mov esi, [esp+8]
    push esi
    call cdecl_one
    mov ecx, [esp]
    push 2
    call obj_set
    add esp, 4
    pop esi
    ret 4

    align 16, db 0xcc
entry:
    push 0x40
    mov ecx, buf
    call forward
    call inherited
    push buf
    call twice
    call drift
    call fastcaller
    push buf
    call reload
    ret

section .rdata
    align 4, db 0
import_dir:
    dd RVA(ilt_foo), 0, 0, RVA(dll_foo), RVA(iat_foo)
    times 5 dd 0
import_dir_end:
iat:
iat_foo:
__imp_FooBar: dd RVA(hn_foobar)
    dd 0
iat_end:
ilt_foo: dd RVA(hn_foobar), 0
hn_foobar: dw 0
    db 'FooBar', 0
    align 2, db 0
dll_foo: db 'foo.dll', 0

section .data
buf: dd 0, 0

%include "pe32_data_end.inc"

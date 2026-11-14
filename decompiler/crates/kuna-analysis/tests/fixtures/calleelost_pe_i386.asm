; Clean-room repro: callees whose stack-argument reads the callee walk cannot
; place, and a caller whose pushed local an earlier bounded call frees
; (kuna_calleestackargs, kuna_calleepurge).
;
; Each reader is __cdecl(a, b) and reads b; its caller passes (0x11, 0x22)
; through `mov [esp+k]` stores with no cleanup after the call, so only the
; callee's own reads can bound the call:
;   get_sp / read_ret   read_ret reads b through the stack address get_sp
;                       returns (`lea eax,[esp+4]; ret`).
;   read_xmm            parks the address of b in xmm0 and reads it back.
;   read_pushesp        `push esp; pop eax` below its locals, then [eax+0x10].
;
;   keep_one(x)         __stdcall, reads x, `ret 4`.
;   pass_on(x)          __stdcall, calls the import FooBar first, so its own
;                       reads cannot be placed (no extent).
;   cookie_caller(a0)   pushes a cookie-style local in its prologue, then
;                       keep_one(a0) and pass_on(7): keep_one is bounded, and
;                       the local it no longer takes must not move to pass_on.
;
; After entry, more __cdecl(a, b) readers of b, each with a caller like the
; ones above:
;   read_argslot        stores a local's address over its first argument,
;                       loads it back and reads past it.
;   read_movq           stores a local's address in a slot, copies the slot
;                       with an 8-byte movq, loads it back and reads past it.
;   get_sp_ecx / read_ecx  read_ecx reads b through the stack address
;                       get_sp_ecx leaves in ECX (`lea ecx,[esp+4]; ret`).
;   read_ret2           read_ret again, for a session that turns the option on
;                       after walking it with the option off.
;
; Build: nasm -f bin -I . -o calleelost_pe_i386.exe calleelost_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

get_sp:                                 ; 0x401000
    lea eax, [esp+4]
    ret

    align 16, db 0xcc
read_ret:
    call get_sp
    mov eax, [eax+8]
    ret

    align 16, db 0xcc
read_xmm:
    lea eax, [esp+8]
    movd xmm0, eax
    xor eax, eax
    movd eax, xmm0
    mov eax, [eax]
    ret

    align 16, db 0xcc
read_pushesp:
    sub esp, 8
    push esp
    pop eax
    mov eax, [eax+0x10]
    add esp, 8
    ret

    align 16, db 0xcc
call_ret:
    push ebp
    mov ebp, esp
    sub esp, 8
    mov dword [esp], 0x11
    mov dword [esp+4], 0x22
    call read_ret
    leave
    ret

    align 16, db 0xcc
call_xmm:
    push ebp
    mov ebp, esp
    sub esp, 8
    mov dword [esp], 0x11
    mov dword [esp+4], 0x22
    call read_xmm
    leave
    ret

    align 16, db 0xcc
call_pushesp:
    push ebp
    mov ebp, esp
    sub esp, 8
    mov dword [esp], 0x11
    mov dword [esp+4], 0x22
    call read_pushesp
    leave
    ret

    align 16, db 0xcc
keep_one:
    mov eax, [esp+4]
    add eax, 1
    ret 4

    align 16, db 0xcc
pass_on:
    push dword [esp+4]
    call [__imp_FooBar]
    pop ecx
    ret 4

    align 16, db 0xcc
cookie_caller:
    push ebp
    mov ebp, esp
    push esi
    mov eax, [cookie]
    xor eax, ebp
    push eax
    mov esi, [ebp+8]
    push esi
    call keep_one
    push 7
    call pass_on
    mov esi, [ebp-4]
    leave
    ret

    align 16, db 0xcc
entry:
    call call_ret
    call call_xmm
    call call_pushesp
    push 0xa0
    call cookie_caller
    add esp, 4
    ret

    align 16, db 0xcc
read_argslot:
    sub esp, 8
    mov eax, esp
    mov [esp+0xc], eax
    mov ecx, [esp+0xc]
    mov eax, [ecx+0x10]
    add esp, 8
    ret

    align 16, db 0xcc
read_movq:
    sub esp, 0x10
    mov eax, esp
    mov [esp+4], eax
    movq xmm0, [esp]
    movq [esp+8], xmm0
    mov ecx, [esp+0xc]
    mov eax, [ecx+0x18]
    add esp, 0x10
    ret

    align 16, db 0xcc
get_sp_ecx:
    lea ecx, [esp+4]
    ret

    align 16, db 0xcc
read_ecx:
    call get_sp_ecx
    mov eax, [ecx+8]
    ret

    align 16, db 0xcc
read_ret2:
    call get_sp
    mov eax, [eax+8]
    ret

%macro CALLER 1
    align 16, db 0xcc
call_%1:
    push ebp
    mov ebp, esp
    sub esp, 8
    mov dword [esp], 0x11
    mov dword [esp+4], 0x22
    call read_%1
    leave
    ret
%endmacro

CALLER argslot
CALLER movq
CALLER ecx
CALLER ret2

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
cookie: dd 0xbb40e64e

%include "pe32_data_end.inc"

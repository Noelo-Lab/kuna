; Clean-room repro: x86-32 callees whose returns do not, or do, state what
; they pop, in the shapes an MSVC /O2 image links them (kuna_calleepurge,
; kuna_calleestackargs).
;
;   seh_prolog4 / seh_epilog4  the CRT's frame helpers: the prolog lowers its
;                              caller's stack by a size it is passed and leaves
;                              EBP set for it; the epilog restores both from
;                              that EBP. Neither returns with the stack where
;                              it found it.
;   alloca_probe_16 / chkstk   `_alloca`: returns with ESP lowered by EAX.
;   seh_open(path, mode)       __cdecl, framed by seh_prolog4; reads both
;                              arguments through the EBP the helper set and
;                              passes them to add2.
;   map_string(l, f, s, n)     __cdecl EBP frame calling LCMapStringW twice,
;                              with an _alloca'd buffer between, then freea.
;   add2(a, b)                 __stdcall, EBP frame, `ret 8`.
;   use_add2(a, b, c)          calls add2 twice, the second after the first's
;                              pop.
;   log_msg(fmt, ...)          hands va_start's argument address to vsprintf.
;   pick(a, b)                 reads b through a copy of ESP.
;   start                      calls each of them.
;
; Build: nasm -f bin -I . -o calleeframes_pe_i386.exe calleeframes_pe_i386.asm

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

seh_epilog4:
    mov ecx, [ebp-0x10]
    mov [fs:0], ecx
    pop ecx
    pop edi
    pop edi
    pop esi
    pop ebx
    mov esp, ebp
    pop ebp
    push ecx
    ret

except_handler:
    xor eax, eax
    ret

    align 16, db 0xcc
alloca_probe_16:
    push ecx
    lea ecx, [esp+8]
    sub ecx, eax
    and ecx, 0xf
    add eax, ecx
    sbb ecx, ecx
    or eax, ecx
    pop ecx
    jmp chkstk

    align 16, db 0xcc
chkstk:
    push ecx
    lea ecx, [esp+4]
    sub ecx, eax
    sbb eax, eax
    not eax
    and ecx, eax
    mov eax, esp
    and eax, 0xfffff000
.cs10:
    cmp ecx, eax
    jb .cs20
    mov eax, ecx
    pop ecx
    xchg esp, eax
    mov eax, [eax]
    mov [esp], eax
    ret
.cs20:
    sub eax, 0x1000
    test [eax], eax
    jmp .cs10

    align 16, db 0xcc
seh_open:
    push 0x10
    push scopetable
    call seh_prolog4
    xor ebx, ebx
    mov [ebp-4], ebx
    push dword [ebp+0xc]
    push dword [ebp+8]
    call add2
    mov [ebp-0x1c], eax
    mov dword [ebp-4], -2
    mov eax, [ebp-0x1c]
    call seh_epilog4
    ret

    align 16, db 0xcc
heap_alloc:
    push dword [esp+4]
    call [__imp_malloc]
    pop ecx
    ret

    align 16, db 0xcc
freea:
    mov eax, [esp+4]
    test eax, eax
    je .done
    sub eax, 8
    cmp dword [eax], 0xdddd
    jne .done
    push eax
    call [__imp_free]
    pop ecx
.done:
    ret

    align 16, db 0xcc
map_string:
    mov edi, edi
    push ebp
    mov ebp, esp
    sub esp, 0x10
    push ebx
    push esi
    push edi
    xor ebx, ebx
    mov [ebp-4], ebx
    mov esi, [__imp_LCMapStringW]
    push ebx
    push ebx
    push dword [ebp+0x14]
    push dword [ebp+0x10]
    push dword [ebp+0xc]
    push dword [ebp+8]
    call esi
    mov ecx, eax
    mov [ebp-8], ecx
    cmp ecx, ebx
    je .out
    lea eax, [ecx+ecx+8]
    cmp eax, 0x400
    ja .heap
    call alloca_probe_16
    mov edi, esp
    cmp edi, ebx
    je .out
    mov dword [edi], 0xcccc
    add edi, 8
    jmp .have
.heap:
    push eax
    call heap_alloc
    pop ecx
    cmp eax, ebx
    je .out
    mov dword [eax], 0xdddd
    add eax, 8
    mov edi, eax
.have:
    push dword [ebp-8]
    push edi
    push dword [ebp+0x14]
    push dword [ebp+0x10]
    push dword [ebp+0xc]
    push dword [ebp+8]
    call esi
    mov [ebp-4], eax
    push edi
    call freea
    pop ecx
.out:
    mov eax, [ebp-4]
    lea esp, [ebp-0x1c]
    pop edi
    pop esi
    pop ebx
    leave
    ret

    align 16, db 0xcc
add2:
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    add eax, [ebp+0xc]
    pop ebp
    ret 8

    align 16, db 0xcc
use_add2:
    push esi
    mov esi, [esp+8]
    push dword [esp+0xc]
    push esi
    call add2
    push eax
    push dword [esp+0x14]
    call add2
    pop esi
    ret

    align 16, db 0xcc
log_msg:
    lea eax, [esp+8]
    push eax
    push dword [esp+8]
    push msgbuf
    call [__imp_vsprintf]
    add esp, 0xc
    ret

    align 16, db 0xcc
pick:
    push esi
    mov esi, esp
    mov eax, [esi+0xc]
    pop esi
    ret

    align 16, db 0xcc
entry:
    push 0x80000000
    push path
    call seh_open
    add esp, 8
    push 4
    push src
    push 0
    push 0x409
    call map_string
    add esp, 0x10
    push 3
    push 2
    push 1
    call use_add2
    add esp, 0xc
    push 2
    push 1
    push fmt
    call log_msg
    add esp, 0xc
    push 2
    push 1
    call pick
    add esp, 8
    ret

section .rdata
path: db 'kuna.txt', 0
fmt: db '%d %d', 0
    align 2, db 0
src: dw 'a', 'b', 'c', 'd', 0
    align 4, db 0
scopetable: dd -2, 0, -0x2c, 0, -2, 0, 0
import_dir:
    dd RVA(ilt_kernel32), 0, 0, RVA(dll_kernel32), RVA(iat_kernel32)
    dd RVA(ilt_msvcrt), 0, 0, RVA(dll_msvcrt), RVA(iat_msvcrt)
    times 5 dd 0
import_dir_end:
iat:
iat_kernel32:
__imp_CreateFileA:   dd RVA(hn_createfile)
__imp_LCMapStringW:  dd RVA(hn_lcmap)
    dd 0
iat_msvcrt:
__imp_malloc:        dd RVA(hn_malloc)
__imp_free:          dd RVA(hn_free)
__imp_vsprintf:      dd RVA(hn_vsprintf)
    dd 0
iat_end:
ilt_kernel32: dd RVA(hn_createfile), RVA(hn_lcmap), 0
ilt_msvcrt: dd RVA(hn_malloc), RVA(hn_free), RVA(hn_vsprintf), 0
hn_createfile: dw 0
    db 'CreateFileA', 0
    align 2, db 0
hn_lcmap: dw 0
    db 'LCMapStringW', 0
    align 2, db 0
hn_malloc: dw 0
    db 'malloc', 0
    align 2, db 0
hn_free: dw 0
    db 'free', 0
    align 2, db 0
hn_vsprintf: dw 0
    db 'vsprintf', 0
    align 2, db 0
dll_kernel32: db 'KERNEL32.dll', 0
dll_msvcrt: db 'msvcrt.dll', 0

section .data
cookie: dd 0xbb40e64e
msgbuf: times 0x40 db 0

%include "pe32_data_end.inc"

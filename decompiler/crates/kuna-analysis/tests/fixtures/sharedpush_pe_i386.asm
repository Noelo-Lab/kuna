; Clean-room repro: one push above a branch is the argument of a declared
; import on one arm and of an undeclared callee on the other (MSVC hoists
; an argument push both arms need).
;
;   pick(sel)        __cdecl (0x401000). push 0x14, then malloc through the
;                    import table or the internal wrapper e_alloc; then push
;                    the result, and free or e_free. Every call takes the
;                    pushed value.
;   cases(dst)       __cdecl (0x401040). A loop that pushes i and calls
;                    toupper through the import table or tolower through a
;                    register loaded from it; both take i.
;   rejoin(sel)      __cdecl (0x401080). push 0x14; malloc only if sel is
;                    set, then Note from note.dll on both paths: on one path
;                    Note runs after malloc on the slot malloc took.
;   guarded(fp, p)   __cdecl (0x4010a0). Pushes p, then either checks fp
;                    with the control-flow guard (target in ECX) and calls
;                    fp, or calls free. fp and free take p; the guard call
;                    runs on the slot first and does not.
;   e_alloc (0x4010d0), e_free (0x4010e0)   __cdecl wrappers of the imports.
;   guard_nop (0x4010f0)                    the guard check, `ret`.
;   deeper(sel)      __cdecl (0x401100). push 0x14, then malloc, or push 3
;                    and call Note: on that arm 0x14 sits one slot deeper
;                    than malloc's argument.
;   outer(p, sel)    __cdecl (0x401120). if (sel) free(p); else
;                    pair(guard_nop(), p): p is pushed once for free and for
;                    pair's second argument, so guard_nop runs on the slot
;                    and pair reads it; guard_nop takes nothing.
;   pair (0x401150)                          __cdecl, adds its two arguments.
;
; Build: nasm -f bin -I . -o sharedpush_pe_i386.exe sharedpush_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

pick:                                   ; 0x401000
    push esi
    cmp dword [esp+8], 0
    push 0x14
    jz .e
    call [__imp_malloc]
    jmp .j
.e:
    call e_alloc
.j:
    pop ecx
    mov esi, eax
    cmp dword [esp+8], 0
    push esi
    jz .f
    call [__imp_free]
    jmp .k
.f:
    call e_free
.k:
    pop ecx
    mov eax, esi
    pop esi
    ret

    align 16, db 0xcc
cases:                                  ; 0x401040
    push ebx
    push esi
    push edi
    mov edi, [esp+0x10]
    mov ebx, [__imp_tolower]
    xor esi, esi
.top:
    test esi, 1
    push esi
    jz .lo
    call [__imp_toupper]
    jmp .st
.lo:
    call ebx
.st:
    mov [edi+esi], al
    pop ecx
    inc esi
    cmp esi, 0x100
    jl .top
    pop edi
    pop esi
    pop ebx
    ret

    align 16, db 0xcc
rejoin:                                 ; 0x401080
    cmp dword [esp+4], 0
    push 0x14
    jz .n
    call [__imp_malloc]
.n:
    call [__imp_Note]
    pop ecx
    ret

    align 16, db 0xcc
guarded:                                ; 0x4010a0
    push esi
    mov esi, [esp+8]
    push dword [esp+0xc]
    test esi, esi
    jz .plain
    mov ecx, esi
    call [guard_fptr]
    call esi
    jmp .j
.plain:
    call [__imp_free]
.j:
    pop ecx
    pop esi
    ret

    align 16, db 0xcc
e_alloc:                                ; 0x4010d0
    push dword [esp+4]
    call [__imp_malloc]
    pop ecx
    ret

    align 16, db 0xcc
e_free:                                 ; 0x4010e0
    push dword [esp+4]
    call [__imp_free]
    pop ecx
    ret

    align 16, db 0xcc
guard_nop:                              ; 0x4010f0
    ret

    align 16, db 0xcc
deeper:                                 ; 0x401100
    cmp dword [esp+4], 0
    push 0x14
    jz .n
    call [__imp_malloc]
    jmp .j
.n:
    push 3
    call [__imp_Note]
    pop ecx
.j:
    pop ecx
    ret

    align 16, db 0xcc
outer:                                  ; 0x401120
    cmp dword [esp+8], 0
    push dword [esp+4]
    jz .k
    call [__imp_free]
    pop ecx
    ret
.k:
    call guard_nop
    push eax
    call pair
    add esp, 8
    ret

    align 16, db 0xcc
pair:                                   ; 0x401150
    mov eax, [esp+4]
    add eax, [esp+8]
    ret

    align 16, db 0xcc
entry:
    push 1
    call pick
    push table
    call cases
    push 0
    call rejoin
    push table
    push e_alloc
    call guarded
    push 1
    call deeper
    push 0
    push table
    call outer
    add esp, 32
    ret

section .rdata
    align 4, db 0
import_dir:
    dd RVA(ilt_crt), 0, 0, RVA(dll_crt), RVA(iat_crt)
    dd RVA(ilt_note), 0, 0, RVA(dll_note), RVA(iat_note)
    times 5 dd 0
import_dir_end:
iat:
iat_crt:
__imp_malloc: dd RVA(hn_malloc)
__imp_free: dd RVA(hn_free)
__imp_toupper: dd RVA(hn_toupper)
__imp_tolower: dd RVA(hn_tolower)
    dd 0
iat_note:
__imp_Note: dd RVA(hn_note)
    dd 0
iat_end:
ilt_crt: dd RVA(hn_malloc), RVA(hn_free), RVA(hn_toupper), RVA(hn_tolower), 0
ilt_note: dd RVA(hn_note), 0
hn_malloc: dw 0
    db 'malloc', 0
    align 2, db 0
hn_free: dw 0
    db 'free', 0
    align 2, db 0
hn_toupper: dw 0
    db 'toupper', 0
    align 2, db 0
hn_tolower: dw 0
    db 'tolower', 0
    align 2, db 0
hn_note: dw 0
    db 'Note', 0
    align 2, db 0
dll_crt: db 'msvcrt.dll', 0
dll_note: db 'note.dll', 0

section .data
guard_fptr: dd guard_nop
table: times 0x100 db 0

%include "pe32_data_end.inc"

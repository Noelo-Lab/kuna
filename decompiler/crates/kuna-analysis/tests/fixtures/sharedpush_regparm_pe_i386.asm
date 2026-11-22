; Clean-room repro: a declared callee whose own code takes its arguments in
; registers sits on one arm of a branch, and an undeclared call on the other
; reads a padding slot the declared list places a parameter in. The shape of
; wine's MSVCRT_longjmp as MinGW GCC builds it: msvcrt_local_unwind2 is
; static, GCC passes it frame/trylevel/ebp in EAX/EDX/ECX, and its debug
; information lists the three as ordinary parameters.
;
;   local_unwind (0x401000)  takes frame in EAX, level in EDX, ebp in ECX and
;                            stores level and ebp into *frame.
;   finish (0x401020)        __cdecl, takes jmp and retval.
;   stack_free (0x401030)    __cdecl, clears *p.
;   declared_ok(fp, p) __cdecl (0x401040). Pushes p, then fp(p) through a
;                      register or stack_free(p): both take p.
;   longjmp_like(jmp, retval)  __cdecl (0x401060). If jmp->frame is set:
;                      IsBadReadPtr(&jmp->cookie, 4) (__stdcall, pops 8),
;                      then `push edx; push edx` re-reserves the outgoing
;                      area, and either local_unwind(jmp->frame, jmp->level,
;                      jmp->ebp) in EAX, EDX, ECX, or jmp->unwind(jmp), a
;                      __stdcall callee taking one argument (`push eax`
;                      re-reserves the slot it pops). finish(jmp, retval).
;   this_free (0x4010e0)     __thiscall: this in ECX, p on the stack, `ret 4`.
;   declared_this(fp, p) __cdecl (0x4010f0). Pushes p above a branch, then
;                      fp(p) through a register or this_free(p) with ECX set;
;                      this_free's declared list also places `this` on the
;                      stack. Both take p, and nothing else: the next slot up
;                      is the saved ESI.
;
; Build: nasm -f bin -I . -o sharedpush_regparm_pe_i386.exe sharedpush_regparm_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

local_unwind:                           ; 0x401000
    push ebp
    mov ebp, esp
    push edi
    mov edi, edx
    push esi
    mov esi, eax
    mov [esi+0xc], edi
    mov [esi+4], ecx
    pop esi
    pop edi
    pop ebp
    ret

    align 16, db 0xcc
finish:                                 ; 0x401020
    mov eax, [esp+4]
    mov ecx, [esp+8]
    mov [eax+0x20], ecx
    ret

    align 16, db 0xcc
stack_free:                             ; 0x401030
    mov eax, [esp+4]
    mov dword [eax], 0
    ret

    align 16, db 0xcc
declared_ok:                            ; 0x401040
    push esi
    mov esi, [esp+8]
    push dword [esp+0xc]
    test esi, esi
    jz .d
    call esi
    jmp .j
.d:
    call stack_free
.j:
    pop ecx
    pop esi
    ret

    align 32, db 0xcc
longjmp_like:                           ; 0x401060
    push ebp
    mov ebp, esp
    push edi
    push esi
    push ebx
    sub esp, 0x34
    mov ebx, [ebp+8]
    mov esi, [ebp+0xc]
    mov ecx, [ebx+0x18]
    test ecx, ecx
    je .out
    lea eax, [ebx+0x20]
    mov dword [esp+4], 4
    mov [esp], eax
    call [__imp_IsBadReadPtr]
    push edx
    push edx
    test eax, eax
    jne .unwind
    cmp dword [ebx+0x20], 0x56433230
    je .cb
.unwind:
    mov edx, [ebx+0x1c]
    mov eax, [ebx+0x18]
    mov ecx, [ebx]
    call local_unwind
    jmp .out
.cb:
    mov eax, [ebx+0x24]
    test eax, eax
    je .out
    mov [esp], ebx
    call eax
    push eax
.out:
    test esi, esi
    mov eax, 1
    mov [esp], ebx
    cmove esi, eax
    mov [esp+4], esi
    call finish
    lea esp, [ebp-12]
    pop ebx
    pop esi
    pop edi
    pop ebp
    ret

    align 16, db 0xcc
this_free:                              ; 0x4010e0
    mov eax, [esp+4]
    mov [eax], ecx
    ret 4

    align 16, db 0xcc
declared_this:                          ; 0x4010f0
    push esi
    mov esi, [esp+8]
    push dword [esp+0xc]
    test esi, esi
    jz .d
    call esi
    jmp .j
.d:
    mov ecx, esi
    call this_free
    push eax
.j:
    pop ecx
    pop esi
    ret

    align 16, db 0xcc
entry:
    push 0
    push buf
    call longjmp_like
    push buf
    push stack_free
    call declared_ok
    push buf
    push stack_free
    call declared_this
    add esp, 24
    ret

section .rdata
    align 4, db 0
import_dir:
    dd RVA(ilt_k32), 0, 0, RVA(dll_k32), RVA(iat_k32)
    times 5 dd 0
import_dir_end:
iat:
iat_k32:
__imp_IsBadReadPtr: dd RVA(hn_isbad)
    dd 0
iat_end:
ilt_k32: dd RVA(hn_isbad), 0
hn_isbad: dw 0
    db 'IsBadReadPtr', 0
    align 2, db 0
dll_k32: db 'kernel32.dll', 0

section .data
buf: times 0x40 db 0

%include "pe32_data_end.inc"

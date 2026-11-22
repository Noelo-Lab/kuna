# kuna fixture: __stdcall callees whose stack effect the purge walk cannot
# prove, because they call through a pointer and restore no frame pointer, but
# whose own `ret 4` states what they pop.
#   only   (0x401030): int only(Rec *r, int k) { guess(r); return k; }
#   twice  (0x401050): int twice(Rec *r, int k) { guess(r); guess(r); return k; }
#   guess  (0x401070): void __stdcall guess(Rec *r), ret 4; calls sink through
#                      a pointer and removes its argument itself (add esp,4).
#
# Build (Homebrew clang 22 + lld-link):
#   clang --target=i686-pc-windows-msvc -c retpop_pe_i386.s -o r.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:retpop_pe_i386.exe r.obj
    .intel_syntax noprefix
    .text
    .globl _start
_start:
    push 5
    lea eax, [_rec]
    push eax
    call _only
    add esp, 8
    push 6
    lea eax, [_rec]
    push eax
    call _twice
    add esp, 8
    ret

    .p2align 4, 0xcc
_only:
    push esi
    push edi
    mov eax, dword ptr [esp + 0xc]
    push eax
    call _guess
    mov eax, dword ptr [esp + 0x10]
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_twice:
    push esi
    mov esi, dword ptr [esp + 8]
    push esi
    call _guess
    push esi
    call _guess
    mov eax, dword ptr [esp + 0xc]
    pop esi
    ret

    .p2align 4, 0xcc
_guess:
    push dword ptr [esp + 4]
    call dword ptr [_sink_ptr]
    add esp, 4
    ret 4

    .p2align 4, 0xcc
_sink:
    mov eax, dword ptr [esp + 4]
    inc dword ptr [eax]
    ret

    .data
_sink_ptr:
    .long _sink
_rec:
    .long 1, 2, 3, 4, 5, 6, 7, 8

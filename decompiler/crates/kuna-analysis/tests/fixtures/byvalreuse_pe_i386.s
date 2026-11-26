# kuna fixture: a 32-byte struct passed by value to a __thiscall method, copied
# with `rep movsd` into an outgoing block that reuses the 12 bytes an earlier
# __cdecl call's pushes left on the stack (MSVC defers that cleanup), as VC8
# /O2 builds it.
#   byval (0x401000): Rec t; copy(&t, r, 32); return o->take(t);
#   take  (0x401040): __thiscall, ret 0x20; hands its argument's address to
#                     a helper called through a pointer, so the purge walk
#                     cannot settle its stack effect.
# Three copies that stay loops: no callee's own returns pop their block.
#   cleanup (after start): the __cdecl callee ctake returns with a plain ret
#                     and the caller removes the block (add esp,0x20).
#   local:            a local at the bottom of the frame, then an unrelated
#                     __cdecl call whose caller's cleanup, with the local's
#                     release in the epilogue, spans the local.
#   nested:           f(g(0), t): t is copied first, but the next call is g.
#   stray:            the __cdecl callee nf ends in a call that does not
#                     return (through a pointer, so the walk cannot tell),
#                     and its walk runs on through the int3 padding into the
#                     __stdcall neighbour ng's ret 0x20.
#   quit:             a __stdcall callee qtake (ret 0x20) whose other path ends
#                     in exit: its pop is proven only while exit is known not
#                     to return (`noreturn_known`), and the walk is taken again
#                     when that option changes.
#   settle:           an earlier call to the __stdcall note (ret 4, proven)
#                     before the block for take: once take's pop is stated,
#                     every call has one, so note's proven pop is stated too.
#   unsettled:        an earlier __stdcall guess (ret 4, but it calls through a
#                     pointer, so the walk cannot place its stack pointer)
#                     before the block for take: guess's own ret 4 states its
#                     pop, so no pop is left to the solver's guess.
#
# Build (clang 17 + lld-link 17, e.g. in the swift:6.0-jammy Docker image;
# clang 14 does not reproduce the bytes):
#   clang --target=i686-pc-windows-msvc -c byvalreuse_pe_i386.s -o b.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:byvalreuse_pe_i386.exe b.obj
    .intel_syntax noprefix
    .text
    .globl _byval
_byval:                         # int byval(Obj *o, const Rec *r)
    push esi
    push edi
    sub esp, 0x20
    push 0x20
    push dword ptr [esp + 0x34]
    lea eax, [esp + 8]
    push eax
    call _copy
    mov eax, dword ptr [esp + 0x38]
    sub esp, 0x14
    mov edi, esp
    mov ecx, 8
    lea esi, [esp + 0x20]
    rep movsd
    mov ecx, eax
    call _take
    add esp, 0x20
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_take:                          # int Obj::take(Rec t) { return *sink(&t) + t.x[7]; }
    lea eax, [esp + 4]
    push eax
    call dword ptr [_sink_ptr]
    add esp, 4
    mov eax, dword ptr [eax]
    add eax, dword ptr [esp + 0x20]
    ret 0x20

    .p2align 4, 0xcc
_sink:
    mov eax, dword ptr [esp + 4]
    ret

    .p2align 4, 0xcc
_copy:
    push esi
    push edi
    mov edi, dword ptr [esp + 0xc]
    mov esi, dword ptr [esp + 0x10]
    mov ecx, dword ptr [esp + 0x14]
    shr ecx, 2
    rep movsd
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
    .globl _start
_start:
    lea eax, [_rec]
    push eax
    lea eax, [_obj]
    push eax
    call _byval
    add esp, 8
    lea eax, [_rec]
    push 5
    push eax
    call _cleanup
    add esp, 8
    lea eax, [_rec]
    push eax
    call _local
    call _nested
    add esp, 4
    ret

    .p2align 4, 0xcc
_cleanup:                       # int cleanup(const Rec *r, int k) { return ctake(*r) + k; }
    push esi
    push edi
    sub esp, 0x20
    mov edi, esp
    mov ecx, 8
    mov esi, dword ptr [esp + 0x2c]
    rep movsd
    call _ctake
    add esp, 0x20
    add eax, dword ptr [esp + 0x10]
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_local:                         # int local(const Rec *r) { Rec t = *r; return one(1) + t.x[1]; }
    push esi
    push edi
    sub esp, 0x20
    mov edi, esp
    mov ecx, 8
    mov esi, dword ptr [esp + 0x2c]
    rep movsd
    push 1
    call _one
    add esp, 4
    add eax, dword ptr [esp + 4]
    add esp, 0x20
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_nested:                        # void nested(const Rec *r) { take9(one(0), *r); }
    sub esp, 0x30
    mov dword ptr [esp + 0x10], 7
    mov edx, dword ptr [esp + 0x34]
    sub esp, 0x20
    mov eax, edi
    mov ecx, 8
    mov edi, esp
    push esi
    mov esi, edx
    rep movsd
    pop esi
    mov edi, eax
    push 0
    call _one
    add esp, 4
    push eax
    call _take9
    add esp, 0x30
    ret

    .p2align 4, 0xcc
_ctake:                         # int ctake(Rec t), __cdecl
    lea eax, [esp + 4]
    push eax
    call dword ptr [_sink_ptr]
    add esp, 4
    mov eax, dword ptr [eax]
    add eax, dword ptr [esp + 0x20]
    ret

    .p2align 4, 0xcc
_one:                           # int one(int x), __cdecl
    push dword ptr [esp + 4]
    call dword ptr [_sink_ptr]
    add esp, 4
    add eax, dword ptr [esp + 4]
    ret

    .p2align 4, 0xcc
_take9:                         # int __stdcall take9(int x, Rec t)
    lea eax, [esp + 4]
    push eax
    call dword ptr [_sink_ptr]
    add esp, 4
    mov eax, dword ptr [eax]
    add eax, dword ptr [esp + 0x24]
    ret 0x24

    .p2align 4, 0xcc
_stray:                         # int stray(const Rec *r, int k) { return nf(*r) + k; }
    push esi
    push edi
    sub esp, 0x20
    mov edi, esp
    mov ecx, 8
    mov esi, dword ptr [esp + 0x2c]
    rep movsd
    call _nf
    add esp, 0x20
    add eax, dword ptr [esp + 0x10]
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_nf:                            # int nf(Rec t), __cdecl: if (!t.x[0]) fatal(); return t.x[7];
    cmp dword ptr [esp + 4], 0
    je 1f
    mov eax, dword ptr [esp + 0x20]
    ret
1:
    push 1
    call dword ptr [_sink_ptr]

    .p2align 4, 0xcc
_ng:                            # int __stdcall ng(Rec t), the next function
    mov eax, dword ptr [esp + 4]
    ret 0x20

    .p2align 4, 0xcc
_exit:                          # the test names it exit
    ret

    .p2align 4, 0xcc
_quit:                          # int quit(const Rec *r) { return qtake(*r); }
    push esi
    push edi
    sub esp, 0x20
    mov edi, esp
    mov ecx, 8
    mov esi, dword ptr [esp + 0x2c]
    rep movsd
    call _qtake
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_qtake:                         # int __stdcall qtake(Rec t) { if (!*sink(&t)) exit(1); return t.x[7]; }
    lea eax, [esp + 4]
    push eax
    call dword ptr [_sink_ptr]
    add esp, 4
    test eax, eax
    je 1f
    mov eax, dword ptr [esp + 0x20]
    ret 0x20
1:
    push 1
    call _exit

    .p2align 4, 0xcc
_settle:                        # int *settle(Rec *r) { note(r); take(*r); return r; }
    push esi
    push edi
    mov eax, dword ptr [esp + 0xc]
    push eax
    call _note
    sub esp, 0x20
    mov edi, esp
    mov ecx, 8
    mov esi, dword ptr [esp + 0x2c]
    rep movsd
    call _take
    mov eax, dword ptr [esp + 0xc]
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_note:                          # void __stdcall note(Rec *r), ret 4
    mov eax, dword ptr [esp + 4]
    inc dword ptr [eax]
    ret 4

    .p2align 4, 0xcc
_unsettled:                     # int unsettled(Rec *r, int k) { guess(r); return take(*r) + k; }
    push esi
    push edi
    mov eax, dword ptr [esp + 0xc]
    push eax
    call _guess
    sub esp, 0x20
    mov edi, esp
    mov ecx, 8
    mov esi, dword ptr [esp + 0x2c]
    rep movsd
    call _take
    add eax, dword ptr [esp + 0x10]
    pop edi
    pop esi
    ret

    .p2align 4, 0xcc
_guess:                         # void __stdcall guess(Rec *r), ret 4, through a pointer
    push dword ptr [esp + 4]
    call dword ptr [_sink_ptr]
    add esp, 4
    ret 4

    .data
_obj:
    .long 0
_sink_ptr:
    .long _sink
_rec:
    .long 1, 2, 3, 4, 5, 6, 7, 8

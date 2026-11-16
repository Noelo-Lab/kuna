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
#
# Build:
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

    .data
_obj:
    .long 0
_sink_ptr:
    .long _sink
_rec:
    .long 1, 2, 3, 4, 5, 6, 7, 8

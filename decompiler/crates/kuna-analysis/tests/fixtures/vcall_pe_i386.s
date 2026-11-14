# kuna fixture: three calls through the vtable of the object in ECX, the MSVC
# x86-32 dispatch `mov eax,[ecx]; call [eax+k]`.
#   vcall    (0x401000): a virtual member, __thiscall: this in ECX, one push.
#   comcall  (0x401010): a COM method, __stdcall: the interface is pushed last.
#   reserve  (0x401020): a virtual member taking a float; `push ecx` only
#                        reserves the slot that `fstp [esp]` fills.
#   ctable   (0x401040): C, not C++: a __cdecl function pointer in a table
#                        reached through ECX, the caller cleans up after it.
#   comcopy  (0x401050): a COM method whose interface is pushed from the
#                        register ECX was copied from.
#   comesi   (0x401060): five COM calls through ESI, copied from ECX before the
#                        first of them; each pushes the interface from ESI.
#   crosscall(0x4010a0): a C table reached through ESI, copied from ECX before
#                        a call that clobbers ECX.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c vcall_pe_i386.s -o v.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:vcall_pe_i386.exe v.obj
    .intel_syntax noprefix
    .text
    .globl _vcall
_vcall:                         # int vcall(Obj *o) { return o->get(5); }
    mov ecx, dword ptr [esp + 4]
    mov eax, dword ptr [ecx]
    push 5
    call dword ptr [eax + 8]
    ret

    .p2align 4, 0xcc
_comcall:                       # int comcall(IUnknownish *p) { return p->SetVolume(7); }
    mov ecx, dword ptr [esp + 4]
    push 7
    push ecx
    mov eax, dword ptr [ecx]
    call dword ptr [eax + 0xc]
    ret

    .p2align 4, 0xcc
_reserve:                       # int reserve(Obj *o, float f) { return o->scale(1, f); }
    mov ecx, dword ptr [esp + 4]
    fld dword ptr [esp + 8]
    mov eax, dword ptr [ecx]
    push ecx
    fstp dword ptr [esp]
    push 1
    call dword ptr [eax + 4]
    ret

    .p2align 4, 0xcc
_ctable:                        # int ctable(Ctx *c) { return c->ops->fn(5); }
    mov ecx, dword ptr [esp + 4]
    mov eax, dword ptr [ecx]
    push 5
    call dword ptr [eax + 0x10]
    add esp, 4
    ret

    .p2align 4, 0xcc
_comcopy:                       # int comcopy(IUnknownish *p) { return p->Other(7); }
    mov eax, dword ptr [esp + 4]
    mov ecx, eax
    mov edx, dword ptr [ecx]
    push 7
    push eax
    call dword ptr [edx + 0x14]
    ret

    .p2align 4, 0xcc
_comesi:                        # void comesi(IThing *p, int a, int b)
    push esi                    # { p->m0(); p->m1(1); p->m2(2); p->m3(a); p->m4(b); }
    mov ecx, dword ptr [esp + 8]
    mov esi, ecx
    push esi
    mov eax, dword ptr [ecx]
    call dword ptr [eax + 0x34]
    mov eax, dword ptr [esi]
    push 1
    push esi
    call dword ptr [eax + 0x1c]
    mov eax, dword ptr [esi]
    push 2
    push esi
    call dword ptr [eax + 0x20]
    push dword ptr [esp + 0xc]
    mov eax, dword ptr [esi]
    push esi
    call dword ptr [eax + 0x14]
    push dword ptr [esp + 0x10]
    mov eax, dword ptr [esi]
    push esi
    call dword ptr [eax + 0x18]
    pop esi
    ret

    .p2align 4, 0xcc
_crosscall:                     # int crosscall(Ctx *c) { leaf(); return c->ops->fn(5); }
    push esi
    mov ecx, dword ptr [esp + 8]
    mov esi, ecx
    call _leaf
    mov eax, dword ptr [esi]
    push 5
    call dword ptr [eax + 8]
    pop esi
    ret

    .p2align 4, 0xcc
_leaf:
    ret

    .p2align 4, 0xcc
_get:
    mov eax, dword ptr [esp + 4]
    add eax, dword ptr [ecx + 4]
    ret 4

    .p2align 4, 0xcc
_scale:
    mov eax, dword ptr [esp + 4]
    ret 8

    .p2align 4, 0xcc
_setvolume:
    mov eax, dword ptr [esp + 8]
    ret 8

    .p2align 4, 0xcc
    .globl _start
_start:
    lea eax, [_obj]
    push eax
    call _vcall
    add esp, 4
    lea eax, [_obj]
    push eax
    call _comcall
    add esp, 4
    push 0x3f800000
    lea eax, [_obj]
    push eax
    call _reserve
    add esp, 8
    lea eax, [_obj]
    push eax
    call _ctable
    add esp, 4
    lea eax, [_obj]
    push eax
    call _comcopy
    add esp, 4
    push 2
    push 1
    lea eax, [_obj]
    push eax
    call _comesi
    add esp, 0xc
    lea eax, [_obj]
    push eax
    call _crosscall
    add esp, 4
    ret

    .p2align 4, 0xcc
_cfn:
    mov eax, dword ptr [esp + 4]
    ret

    .data
_vtable:
    .long 0, _scale, _get, _setvolume, _cfn, _setvolume
_obj:
    .long _vtable, 3

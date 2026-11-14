# kuna fixture: three calls through the vtable of the object in ECX, the MSVC
# x86-32 dispatch `mov eax,[ecx]; call [eax+k]`.
#   vcall    (0x401000): a virtual member, __thiscall: this in ECX, one push.
#   comcall  (0x401010): a COM method, __stdcall: the interface is pushed last.
#   reserve  (0x401020): a virtual member taking a float; `push ecx` only
#                        reserves the slot that `fstp [esp]` fills.
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
    ret

    .data
_vtable:
    .long 0, _scale, _get, _setvolume
_obj:
    .long _vtable, 3

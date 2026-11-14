# kuna fixture: an inner this-only call between an outer virtual call's first
# push and its call, in a function with a local frame, so the epilogue tears
# the frame down after the callee-saved registers are restored (the MSVC /O2
# shape of `return o->message(9, 0, v->angle(), extra)` with locals).
#
# Build:
#   clang --target=i686-pc-windows-msvc -c epilogcleanup_pe_i386.s -o e.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:epilogcleanup_pe_i386.exe e.obj
    .intel_syntax noprefix
    .text
    .globl _steer
_steer:                         # int __stdcall steer(Obj *o, Vec *v, int extra)
    sub esp, 0x10
    push esi
    push edi
    mov esi, dword ptr [esp + 0x1c]
    mov eax, dword ptr [esp + 0x24]
    mov ecx, dword ptr [esp + 0x20]
    mov edi, dword ptr [esi]
    push eax                    # message's fourth argument: extra
    call _angle                 # float Vec::angle() (this in ecx, no stack argument)
    mov edx, dword ptr [edi]
    push ecx
    fstp dword ptr [esp]
    push 0
    push 9
    mov ecx, esi
    call edx                    # int Obj::message(int, int, float, int), pops 16
    mov dword ptr [esp + 8], eax
    mov eax, dword ptr [esp + 8]
    pop edi
    pop esi
    add esp, 0x10
    ret 0xc

    .p2align 4, 0xcc
_angle:
    fld dword ptr [ecx]
    ret

    .p2align 4, 0xcc
_message:
    mov eax, dword ptr [esp + 4]
    add eax, dword ptr [esp + 0x10]
    ret 0x10

    .p2align 4, 0xcc
    .globl _start
_start:
    lea eax, [_vec]
    push 3
    push eax
    lea eax, [_obj]
    push eax
    call _steer
    ret

    .data
_vtable:
    .long _message
_obj:
    .long _vtable
_vec:
    .long 0x3f800000

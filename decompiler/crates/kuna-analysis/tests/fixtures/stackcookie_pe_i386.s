# kuna fixture: MSVC frame cookies (`/GS`, EH) stored on the stack below a
# call's arguments. The cookie is a global XORed with the stack or frame
# pointer; only the epilogue reads it back.
#   gsseh   (0x401000): an SEH frame with a /GS cookie pushed below it, then
#                       g = lookup("NAME") through a pointer, cleaned up after.
#   gsthis  (0x401050): the same frame around a method taking only this
#                       (`lea ecx,[esi+8]; call`) and no stack argument.
#   gsebp   (0x401090): an EBP frame storing the cookie at [ebp-4] and
#                       checking it before the return; one call in between.
#   keyarg  (0x401130): no SEH frame; g = lookup(key ^ esp) through a pointer,
#                       a real argument shaped like the cookie.
#   keyread (0x401150): an SEH frame with a cookie; g = lookup(key ^ esp) to
#                       the direct lookup, which reads that argument.
#   copyarg (0x4011a0): an SEH frame with a cookie; after a call it pushes a
#                       copy read back from the cookie's slot and calls
#                       lookup through a pointer: the copy is the argument.
#   sehkey  (0x4011f0): an SEH frame with a cookie; g = lookup2(key ^ esp, 7)
#                       through a pointer, pushed in the body: both are
#                       arguments, the cookie is not.
#   sehebp  (0x401240): an EBP SEH frame with no cookie; g = lookup(key ^ ebp)
#                       through a pointer: the argument stays.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c stackcookie_pe_i386.s -o s.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:stackcookie_pe_i386.exe s.obj
    .intel_syntax noprefix
    .text
    .globl _gsseh
_gsseh:                         # void gsseh(void) { g = lookup("NAME"); }
    push -1
    push 0
    mov eax, dword ptr fs:[0]
    push eax
    push esi
    mov eax, dword ptr [___security_cookie]
    xor eax, esp
    push eax
    lea eax, [esp + 8]
    mov dword ptr fs:[0], eax
    lea edx, [_name]
    push edx
    call dword ptr [_lookup_ptr]
    add esp, 4
    mov dword ptr [_g], eax
    mov ecx, dword ptr [esp + 8]
    mov dword ptr fs:[0], ecx
    pop ecx
    pop esi
    add esp, 0xc
    ret

    .p2align 4, 0xcc
_gsthis:                        # void gsthis(Obj *o) { o->inner.reset(); }
    push -1
    push 0
    mov eax, dword ptr fs:[0]
    push eax
    push esi
    mov eax, dword ptr [___security_cookie]
    xor eax, esp
    push eax
    lea eax, [esp + 8]
    mov dword ptr fs:[0], eax
    mov esi, dword ptr [esp + 0x18]
    lea ecx, [esi + 8]
    call _reset
    mov ecx, dword ptr [esp + 8]
    mov dword ptr fs:[0], ecx
    pop ecx
    pop esi
    add esp, 0xc
    ret

    .p2align 4, 0xcc
_gsebp:                         # int gsebp(int x) { char buf[4]; return use(x); }
    push ebp
    mov ebp, esp
    sub esp, 8
    mov eax, dword ptr [___security_cookie]
    xor eax, ebp
    mov dword ptr [ebp - 4], eax
    push dword ptr [ebp + 8]
    call dword ptr [_use_ptr]
    add esp, 4
    mov ecx, dword ptr [ebp - 4]
    xor ecx, ebp
    call _check_cookie
    mov esp, ebp
    pop ebp
    ret

    .p2align 4, 0xcc
_reset:
    mov dword ptr [ecx], 0
    ret

    .p2align 4, 0xcc
_lookup:
    mov eax, dword ptr [esp + 4]
    ret

    .p2align 4, 0xcc
_use:
    mov eax, dword ptr [esp + 4]
    ret

    .p2align 4, 0xcc
_check_cookie:
    cmp ecx, dword ptr [___security_cookie]
    jne _fail
    ret
_fail:
    int3

    .p2align 4, 0xcc
    .globl _start
_start:
    call _gsseh
    lea eax, [_obj]
    push eax
    call _gsthis
    add esp, 4
    push 3
    call _gsebp
    add esp, 4
    call _keyarg
    call _keyread
    call _copyarg
    ret

    .p2align 4, 0xcc
_keyarg:
    mov eax, dword ptr [_key]
    xor eax, esp
    push eax
    call dword ptr [_lookup_ptr]
    add esp, 4
    mov dword ptr [_g], eax
    ret

    .p2align 4, 0xcc
_keyread:
    push -1
    push 0
    mov eax, dword ptr fs:[0]
    push eax
    push esi
    mov eax, dword ptr [___security_cookie]
    xor eax, esp
    push eax
    lea eax, [esp + 8]
    mov dword ptr fs:[0], eax
    mov eax, dword ptr [_key]
    xor eax, esp
    push eax
    call _lookup
    add esp, 4
    mov dword ptr [_g], eax
    mov ecx, dword ptr [esp + 8]
    mov dword ptr fs:[0], ecx
    pop ecx
    pop esi
    add esp, 0xc
    ret

    .p2align 4, 0xcc
_copyarg:
    push -1
    push 0
    mov eax, dword ptr fs:[0]
    push eax
    push esi
    mov eax, dword ptr [___security_cookie]
    xor eax, esp
    push eax
    lea eax, [esp + 8]
    mov dword ptr fs:[0], eax
    lea ecx, [_obj]
    call _reset
    push dword ptr [esp]
    call dword ptr [_lookup_ptr]
    add esp, 4
    mov dword ptr [_g], eax
    mov ecx, dword ptr [esp + 8]
    mov dword ptr fs:[0], ecx
    pop ecx
    pop esi
    add esp, 0xc
    ret

    .p2align 4, 0xcc
_sehkey:
    push -1
    push 0
    mov eax, dword ptr fs:[0]
    push eax
    push esi
    mov eax, dword ptr [___security_cookie]
    xor eax, esp
    push eax
    lea eax, [esp + 8]
    mov dword ptr fs:[0], eax
    push 7
    mov eax, dword ptr [_key]
    xor eax, esp
    push eax
    call dword ptr [_lookup2_ptr]
    add esp, 8
    mov dword ptr [_g], eax
    mov ecx, dword ptr [esp + 8]
    mov dword ptr fs:[0], ecx
    pop ecx
    pop esi
    add esp, 0xc
    ret

    .p2align 4, 0xcc
_sehebp:
    push ebp
    mov ebp, esp
    push -1
    push 0
    push dword ptr fs:[0]
    mov dword ptr fs:[0], esp
    sub esp, 8
    mov eax, dword ptr [_key]
    xor eax, ebp
    push eax
    call dword ptr [_lookup_ptr]
    add esp, 4
    mov dword ptr [_g], eax
    mov ecx, dword ptr [ebp - 0xc]
    mov dword ptr fs:[0], ecx
    mov esp, ebp
    pop ebp
    ret

    .p2align 4, 0xcc
_lookup2:
    mov eax, dword ptr [esp + 4]
    add eax, dword ptr [esp + 8]
    ret

    .data
___security_cookie:
    .long 0xbb40e64e
_lookup_ptr:
    .long _lookup
_use_ptr:
    .long _use
_g:
    .long 0
_obj:
    .long 0, 0, 0
_name:
    .asciz "NAME"
_key:
    .long 0x5a5a1234
    .p2align 2
_lookup2_ptr:
    .long _lookup2

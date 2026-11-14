; Clean-room repro: `rep movsd` copies with a constant count into and out of
; the frame (kuna_repframecopy), and the shapes left as loops.
;
;   fill8(Rect r, int b)  __stdcall, reads all eight dwords of r and b, `ret 0x24`.
;   use1(int x)           __stdcall, reads x, `ret 4`.
;   take6(Six s)          __stdcall, reads all six dwords of s, `ret 0x18`.
;   take_big(Big s)       __stdcall, reads the first and last dword of a
;                         32-dword s, `ret 0x80`.
;
;   ebp_draw(src, b, c)   frame pointer; calls use1(c) only when c is not zero,
;                         then copies *src into a local Rect, and the local into
;                         the outgoing area for fill8(r, b): both copies are
;                         expanded, so fill8 takes src's eight dwords and b.
;   big_pass(src)         passes a 32-dword struct by value: too large to
;                         expand, so the copy stays a loop.
;   copy_global(dst)      copies four dwords from a global to dst: no frame
;                         slot, so the copy stays a loop.
;   zero_pass(a)          zeroes a six-dword local with `rep stosd`, sets its
;                         first dword to a and passes it by value to take6: the
;                         copy reads the frame while the stosd walks it, so it
;                         stays a loop.
;   forward(Rect r)       __stdcall, passes its own by-value parameter on to
;                         fill8(r, 7): the copy reads the parameter slots.
;   loop_fill()           fills a local Rect with an indexed loop (r[i] = 3*i)
;                         and passes it by value to fill8(r, 1): the copy reads
;                         slots the indexed store writes, so it stays a loop.
;   paint3(a, b, c)       __stdcall, reads all three, `ret 0xc` (after entry).
;   big_draw(a, b, src)   calls paint3(a, b, 3), then passes a 32-dword struct
;                         by value: the copy stays a loop and its bottom-of-frame
;                         pointer is what `calleeargs` keeps from costing paint3
;                         its pushed arguments (tests/stages/kuna-calleeargs.xml).
;   shift_pass(a)         fills nine slots at the stack pointer, moves the upper
;                         eight down one slot with an overlapping `rep movsd`,
;                         and calls fill8: it takes 1..8 and 8.
;
; Register-argument callers (Delphi's convention: eax, edx, ecx) that copy a
; record into a local at the bottom of their frame with `rep movsd`. The local
; is no call's stack argument, so every move stays a loop:
;   reg_sum(eax, edx)     adds the dwords two pointers point at; no stack.
;   virt_get(eax)         the method at +0x58 of obj's table; no stack.
;   d_virtual(self, r)    two virtual calls through self's table, the local
;                         read back after them.
;   d_ptr(self, r, x)     patches the local and hands its address to reg_sum
;                         in edx.
;   d_direct(r)           reads the local, then calls reg_sum on two globals.
;
; Build: nasm -f bin -I . -o repframecopy_pe_i386.exe repframecopy_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

fill8:                                  ; 0x401000
    mov eax, [esp+4]
    add eax, [esp+8]
    add eax, [esp+0xc]
    add eax, [esp+0x10]
    add eax, [esp+0x14]
    add eax, [esp+0x18]
    add eax, [esp+0x1c]
    add eax, [esp+0x20]
    imul eax, [esp+0x24]
    ret 0x24

    align 16, db 0xcc
use1:                                   ; 0x401030
    mov eax, [esp+4]
    mov [sink], eax
    ret 4

    align 16, db 0xcc
take6:                                  ; 0x401040
    mov eax, [esp+4]
    add eax, [esp+8]
    add eax, [esp+0xc]
    add eax, [esp+0x10]
    add eax, [esp+0x14]
    add eax, [esp+0x18]
    ret 0x18

    align 16, db 0xcc
take_big:                               ; 0x401060
    mov eax, [esp+4]
    add eax, [esp+0x80]
    ret 0x80

    align 16, db 0xcc
ebp_draw:                               ; 0x401070
    push ebp
    mov ebp, esp
    sub esp, 0x20
    push esi
    push edi
    mov eax, [ebp+0x10]
    test eax, eax
    jz .copy
    push eax
    call use1
.copy:
    mov esi, [ebp+8]
    lea edi, [ebp-0x20]
    mov ecx, 8
    rep movsd
    push dword [ebp+0xc]
    sub esp, 0x20
    mov edi, esp
    lea esi, [ebp-0x20]
    mov ecx, 8
    rep movsd
    call fill8
    pop edi
    pop esi
    mov esp, ebp
    pop ebp
    ret

    align 16, db 0xcc
big_pass:                               ; 0x4010b0
    push esi
    push edi
    sub esp, 0x80
    mov edi, esp
    mov esi, [esp+0x8c]
    mov ecx, 0x20
    rep movsd
    call take_big
    pop edi
    pop esi
    ret

    align 16, db 0xcc
copy_global:                            ; 0x4010d0
    push esi
    push edi
    mov edi, [esp+0xc]
    mov esi, table
    mov ecx, 4
    rep movsd
    pop edi
    pop esi
    ret

    align 16, db 0xcc
zero_pass:                              ; 0x4010f0
    push esi
    push edi
    sub esp, 0x18
    lea edi, [esp]
    xor eax, eax
    mov ecx, 6
    rep stosd
    mov eax, [esp+0x24]
    mov [esp], eax
    sub esp, 0x18
    mov edi, esp
    lea esi, [esp+0x18]
    mov ecx, 6
    rep movsd
    call take6
    add esp, 0x18
    pop edi
    pop esi
    ret

    align 16, db 0xcc
forward:                                ; 0x401130
    push esi
    push edi
    push 7
    sub esp, 0x20
    mov edi, esp
    lea esi, [esp+0x30]
    mov ecx, 8
    rep movsd
    call fill8
    pop edi
    pop esi
    ret 0x20

    align 16, db 0xcc
loop_fill:                              ; 0x401150
    push esi
    push edi
    sub esp, 0x20
    xor eax, eax
.next:
    lea edx, [eax+eax*2]
    mov [esp+eax*4], edx
    inc eax
    cmp eax, 8
    jl .next
    push 1
    sub esp, 0x20
    mov edi, esp
    lea esi, [esp+0x24]
    mov ecx, 8
    rep movsd
    call fill8
    add esp, 0x20
    pop edi
    pop esi
    ret

    align 16, db 0xcc
entry:
    push 0
    push 2
    push table
    call ebp_draw
    add esp, 0xc
    push table
    call big_pass
    add esp, 4
    push buffer
    call copy_global
    add esp, 4
    push 5
    call zero_pass
    add esp, 4
    sub esp, 0x20
    mov edi, esp
    mov esi, table
    mov ecx, 8
    rep movsd
    call forward
    call loop_fill
    push table
    push 2
    push 1
    call big_draw
    add esp, 0xc
    push 9
    call shift_pass
    add esp, 4
    mov eax, obj
    mov edx, table
    call d_virtual
    mov eax, obj
    mov ecx, table
    mov edx, 5
    call d_ptr
    mov edx, table
    call d_direct
    ret

    align 16, db 0xcc
paint3:
    mov eax, [esp+4]
    add eax, [esp+8]
    imul eax, [esp+0xc]
    mov [sink], eax
    ret 0xc

    align 16, db 0xcc
big_draw:
    push esi
    push edi
    push 3
    push dword [esp+0x14]
    push dword [esp+0x14]
    call paint3
    sub esp, 0x80
    mov edi, esp
    mov esi, [esp+0x94]
    mov ecx, 0x20
    rep movsd
    call take_big
    pop edi
    pop esi
    ret

    align 16, db 0xcc
shift_pass:
    push esi
    push edi
    sub esp, 0x24
    mov eax, [esp+0x30]
    mov [esp], eax
    mov dword [esp+4], 1
    mov dword [esp+8], 2
    mov dword [esp+0xc], 3
    mov dword [esp+0x10], 4
    mov dword [esp+0x14], 5
    mov dword [esp+0x18], 6
    mov dword [esp+0x1c], 7
    mov dword [esp+0x20], 8
    mov edi, esp
    lea esi, [esp+4]
    mov ecx, 8
    rep movsd
    call fill8
    pop edi
    pop esi
    ret

    align 16, db 0xcc
reg_sum:
    mov eax, [eax]
    add eax, [edx]
    ret

    align 16, db 0xcc
virt_get:
    mov eax, [eax+4]
    ret

    align 16, db 0xcc
d_virtual:
    push ebx
    push esi
    push edi
    add esp, -0x10
    mov esi, edx
    lea edi, [esp]
    mov ecx, 4
    rep movsd
    mov ebx, eax
    mov edx, [ebx]
    call [edx+0x58]
    test eax, eax
    jz .done
    mov eax, ebx
    mov edx, [eax]
    call [edx+0x58]
    mov edx, [esp+4]
    add edx, [esp+0xc]
    add eax, edx
.done:
    add esp, 0x10
    pop edi
    pop esi
    pop ebx
    ret

    align 16, db 0xcc
d_ptr:
    push esi
    push edi
    add esp, -0x10
    mov esi, ecx
    mov edi, esp
    mov ecx, 4
    rep movsd
    mov [esp+8], edx
    mov edx, esp
    call reg_sum
    add esp, 0x10
    pop edi
    pop esi
    ret

    align 16, db 0xcc
d_direct:
    push esi
    push edi
    add esp, -0x10
    mov esi, edx
    lea edi, [esp]
    mov ecx, 4
    rep movsd
    mov eax, [esp+4]
    mov [sink], eax
    mov eax, table
    mov edx, table
    call reg_sum
    add esp, 0x10
    pop edi
    pop esi
    ret

section .rdata
    align 4, db 0
import_dir:
import_dir_end:
iat:
iat_end:

section .data
table: dd 1, 2, 3, 4, 5, 6, 7, 8
sink: dd 0
buffer: times 0x80 db 0
obj: dd vtbl, 7
vtbl: times 0x16 dd 0
    dd virt_get

%include "pe32_data_end.inc"

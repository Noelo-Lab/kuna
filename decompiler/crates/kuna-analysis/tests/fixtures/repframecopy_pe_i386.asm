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

%include "pe32_data_end.inc"

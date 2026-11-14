; Clean-room repro for calleereginputs `custom`: callee-saved register custom
; arguments (EBX/ESI/EDI), the two-sided rule.
;
;   countsum(EDI cnt, a,b,c,d)  the issue's shape: a point count in EDI, 4 stack
;                               args; sums the first cnt of them, `ret 16`. Both
;                               call sites set EDI -> EDI is argument + parameter.
;   edisave(p)                  saves EDI (push edi), overwrites it with its own
;                               stack arg, restores it: reads no incoming EDI.
;                               A save-only callee -> no custom parameter.
;   esiarg()                    reads incoming ESI as a pointer. Called twice;
;                               one site sets ESI, the other leaves a leftover
;                               -> one site does not set it -> disproved.
;   takenfn(EDI)                reads incoming EDI; its address is also stored in
;                               a data table -> address-taken -> excluded though
;                               its one direct call site sets EDI.

; Build: nasm -f bin -I . -o calleeregcustom_pe_i386.exe calleeregcustom_pe_i386.asm
%define IMAGE_DLL 0
%include "pe32_data.inc"

section .text

countsum:                               ; 0x401000
    push ebx
    push esi
    xor eax, eax
    lea esi, [esp+0xc]
    test edi, edi
    jle .done
    mov ecx, edi
.loop:
    add eax, [esi]
    add esi, 4
    dec ecx
    jnz .loop
.done:
    pop esi
    pop ebx
    ret 16

    align 16, db 0xcc
edisave:                                ; 0x401020
    push edi
    mov edi, [esp+8]
    mov eax, [edi]
    pop edi
    ret 4

    align 16, db 0xcc
esiarg:                                 ; 0x401030
    mov eax, [esi]
    ret

    align 16, db 0xcc
takenfn:                                ; 0x401040
    mov eax, [edi]
    ret 4

    align 16, db 0xcc
helper:                                 ; 0x401050 -- a plain cdecl callee for leftover setup
    mov eax, [esp+4]
    ret

    align 16, db 0xcc
entry:                                  ; 0x401060
    ; countsum site 1: EDI set
    mov edi, 4
    push 4
    push 3
    push 2
    push 1
    call countsum
    mov [buf], eax
    ; countsum site 2: EDI set
    mov edi, 2
    push 40
    push 30
    push 20
    push 10
    call countsum
    add [buf], eax
    ; edisave: save-only callee (set EDI, but callee ignores incoming EDI)
    mov edi, buf
    push buf
    call edisave
    add esp, 4
    ; esiarg site A: ESI set
    mov esi, buf
    call esiarg
    mov [buf+4], eax
    ; leftover: call helper (returns in EAX), then esiarg site B with no ESI write
    push 7
    call helper
    add esp, 4
    call esiarg
    mov [buf+4], eax
    ; takenfn: direct call sets EDI, but its address is also stored in fnptr
    mov edi, buf
    call takenfn
    mov eax, [fnptr]
    ret

section .rdata
    align 4, db 0
import_dir:
    dd RVA(ilt_foo), 0, 0, RVA(dll_foo), RVA(iat_foo)
    times 5 dd 0
import_dir_end:
iat:
iat_foo:
__imp_FooBar: dd RVA(hn_foobar)
    dd 0
iat_end:
ilt_foo: dd RVA(hn_foobar), 0
hn_foobar: dw 0
    db 'FooBar', 0
    align 2, db 0
dll_foo: db 'foo.dll', 0

section .data
buf: dd 0, 0
fnptr: dd takenfn

%include "pe32_data_end.inc"

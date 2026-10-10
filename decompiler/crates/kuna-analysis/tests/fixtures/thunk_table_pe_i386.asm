; Clean-room repro: functions reached only through incremental-link thunks.
;
; MSVC /INCREMENTAL puts a table of 5-byte `jmp rel32` thunks at the start
; of .text. The entry point and every call go to a thunk; nothing calls a
; function body directly.
;
; Control: change `call t_sum` in `start` to `call sum` and `--mode fast`
; lists sum at 0x401010.
;
; Build: nasm -f bin -I . -o thunk_table_pe_i386.exe thunk_table_pe_i386.asm

%define IMAGE_DLL 0
%include "pe32.inc"

section .text

; The thunk table.
entry:
t_start:  jmp near start                ; 0x401000
t_sum:    jmp near sum                  ; 0x401005
t_leaf:   jmp near leaf                 ; 0x40100a

    align 16, db 0xcc
; int sum(int a, int b) { return leaf(a) + b; }
sum:                                    ; 0x401010
    push ebp
    mov ebp, esp
    push dword [ebp+8]
    call t_leaf
    add esp, 4
    add eax, [ebp+12]
    pop ebp
    ret

    align 16, db 0xcc
; int leaf(int a) { return a + 7; }  -- no stack frame
leaf:                                   ; 0x401030
    mov eax, [esp+4]
    add eax, 7
    ret

    align 16, db 0xcc
; entry: return sum(3, 200);
start:                                  ; 0x401040
    push ebp
    mov ebp, esp
    push 200
    push 3
    call t_sum
    add esp, 8
    pop ebp
    ret

section .rdata
import_dir:
import_dir_end:
iat:
iat_end:
    dd 0

%include "pe32_end.inc"

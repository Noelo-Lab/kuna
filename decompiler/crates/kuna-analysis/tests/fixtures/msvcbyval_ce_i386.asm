; Counterexamples for `option msvcbyval` (GH Noelo-Lab/kuna#990 review): cleanups and
; reservations that must NOT be read as the size of an MSVC __cdecl import's by-value
; class argument. Built like msvcbyval_i386.exe (arr.dll imports, classes with copy
; constructors so they pass in memory). Each function is a shape the sizing must either
; size correctly or decline -- never size wrong.
;   0x401000 popcx   void Sink(Pair)       pop ecx;pop ecx cleanup (not add esp,k)      -> decline
;   0x401020 pad     void Take3(int,Arr)   4 bytes of sub-esp padding above the args    -> decline
;   0x401040 ebploc  Arr Ebp(Arr&,Arr&)    a [ebp-4] local released by the cleanup      -> decline
;   0x401070 vlog    void Log(Arr,...)     varargs                                       -> not sized
;   0x401090 thisuse Arr Obj::Build(Arr&)  __thiscall, ecx (this) really used           -> size + keep this
;   0x4010c0 loop    MakeStack in a loop   correct per-iteration cleanup                 -> size
;   0x4010f0 six     void Put(Six,int)     6-byte class padded to 8 by sub esp,8         -> size (8)
;   0x401110 pad0    void Take3(int,Arr)   no padding                                    -> size (4)
;   0x401130 pad8    void Take3(int,Arr)   8 bytes of sub-esp padding above the args     -> decline
;   0x401150 ebploc8 MakeStack             8-byte ebp locals released by the cleanup     -> decline
;   0x401190 take4   void Take4(int,Pair)  8-byte class right after a 4-byte slot        -> size (8)
;   0x4011e0 shift8  frameless MakeStack   8 bytes of dead push-ecx locals above args    -> decline
;   0x401210 podfold void Take(Pod,int)    frameless push-ecx local whose address goes to
;                                          Fill, released by the cleanup                 -> decline
;   0x401240 split   void Take(Pod,int)    cleanup split as add esp,8; add esp,4         -> decline
;   0x401260 podok   void Take(Pod,int)    plain add esp,0xc (control)                   -> size (8)
; Build: nasm -f bin -I . -o msvcbyval_ce_i386.exe msvcbyval_ce_i386.asm
%define IMAGE_DLL 0
%include "pe32_data.inc"
section .text
; Counterexamples for msvcbyval: cleanups and reservations that must NOT be read as a
popcx:                                  ; 0x401000
    push ebp
    mov ebp, esp
    push ecx
    push ecx
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Pair_copy]
    call [__imp_Sink]
    pop ecx
    pop ecx
    pop ebp
    ret
    align 16, db 0xcc
; CE2 void Pad(Arr &x) { Take3(5, x); } with 4 bytes of padding above the args, released by the cleanup
pad:                                    ; 0x401020
    push ebp
    mov ebp, esp
    sub esp, 4
    push ecx
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Arr_copy]
    push 5
    call [__imp_Take3]
    add esp, 0xc
    pop ebp
    ret
    align 16, db 0xcc
; CE3 Arr Ebp(Arr &x, Arr &y) { int r = 7; return MakeStack(x, x, y); }: local r at [ebp-4], released by the cleanup
ebploc:                                 ; 0x401040
    push ebp
    mov ebp, esp
    push ecx
    mov dword [ebp-4], 7
    push dword [ebp+16]
    push dword [ebp+12]
    push ecx
    mov ecx, esp
    push dword [ebp+12]
    call [__imp_Arr_copy]
    push dword [ebp+8]
    call [__imp_MakeStack]
    add esp, 0x14
    mov eax, [ebp+8]
    pop ebp
    ret
    align 16, db 0xcc
; CE4 varargs: void Log(Arr, ...)  Log(x, 1, 2)
vlog:                                   ; 0x401070
    push ebp
    mov ebp, esp
    push 2
    push 1
    push ecx
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Arr_copy]
    call [__imp_Log]
    add esp, 0xc
    pop ebp
    ret
    align 16, db 0xcc
; CE5 __thiscall Arr Obj::Build(Arr &x) { return MakeStack(x, x, *(Arr*)this); }: ecx (this) really used
thisuse:                                ; 0x401090
    push ebp
    mov ebp, esp
    push esi
    mov esi, ecx
    push esi
    push dword [ebp+12]
    push ecx
    mov ecx, esp
    push dword [ebp+12]
    call [__imp_Arr_copy]
    push dword [ebp+8]
    call [__imp_MakeStack]
    add esp, 0x10
    mov eax, [ebp+8]
    pop esi
    pop ebp
    ret 8
    align 16, db 0xcc
; CE6 call in a loop with correct cleanup: for (i=0;i<n;i++) MakeStack(x,x,y)
loop:                                   ; 0x4010c0
    push ebp
    mov ebp, esp
    push esi
    mov esi, [ebp+20]
.top:
    push dword [ebp+16]
    push dword [ebp+12]
    push ecx
    mov ecx, esp
    push dword [ebp+12]
    call [__imp_Arr_copy]
    push dword [ebp+8]
    call [__imp_MakeStack]
    add esp, 0x10
    dec esi
    jnz .top
    mov eax, [ebp+8]
    pop esi
    pop ebp
    ret
    align 16, db 0xcc
; CE7 6-byte class Six padded to 8 by sub esp,8: void Put(Six, int)
six:                                    ; 0x401100
    push ebp
    mov ebp, esp
    push 9
    sub esp, 8
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Six_copy]
    call [__imp_Put]
    add esp, 0xc
    pop ebp
    ret
    align 16, db 0xcc
; P1 Take3(5, x) without padding
pad0:
    push ebp
    mov ebp, esp
    push ecx
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Arr_copy]
    push 5
    call [__imp_Take3]
    add esp, 8
    pop ebp
    ret
    align 16, db 0xcc
; V1 Take3(5,x) with 8 bytes padding (slot would read 12)
pad8:                                   ; +0x00
    push ebp
    mov ebp, esp
    sub esp, 8
    push ecx
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Arr_copy]
    push 5
    call [__imp_Take3]
    add esp, 0x10
    pop ebp
    ret
    align 16, db 0xcc
; V2 ebp local of 8 bytes folded into MakeStack's cleanup
ebploc8:
    push ebp
    mov ebp, esp
    sub esp, 8
    mov dword [ebp-4], 7
    mov dword [ebp-8], 8
    push dword [ebp+16]
    push dword [ebp+12]
    push ecx
    mov ecx, esp
    push dword [ebp+12]
    call [__imp_Arr_copy]
    push dword [ebp+8]
    call [__imp_MakeStack]
    add esp, 0x18
    mov eax, [ebp+8]
    pop ebp
    ret
    align 16, db 0xcc
; V3 correct: void Take4(int, Pair)  Take4(5, p)
take4:
    push ebp
    mov ebp, esp
    sub esp, 8
    mov ecx, esp
    push dword [ebp+8]
    call [__imp_Pair_copy]
    push 5
    call [__imp_Take4]
    add esp, 0xc
    pop ebp
    ret
    align 16, db 0xcc
; V4 frameless dead push-ecx local + MakeStack: shift
shift:
    push ecx
    push dword [esp+0x10]
    push dword [esp+0x10]
    push ecx
    mov ecx, esp
    push dword [esp+0x18]
    call [__imp_Arr_copy]
    push dword [esp+0x14]
    call [__imp_MakeStack]
    add esp, 0x14
    ret
    align 16, db 0xcc
; V5 frameless, 8 bytes of dead push-ecx locals + MakeStack: shift by 8
shift8:
    push ecx
    push ecx
    push dword [esp+0x14]
    push dword [esp+0x14]
    push ecx
    mov ecx, esp
    push dword [esp+0x1c]
    call [__imp_Arr_copy]
    push dword [esp+0x18]
    call [__imp_MakeStack]
    add esp, 0x18
    ret
    align 16, db 0xcc
; N1 void F(Pod *p) { int r; Fill(&r); Take(*p, r); }  frameless local r, addressed, released by the cleanup
podfold:                                ; 0x401210
    push ecx
    mov eax, esp
    push eax
    call [__imp_Fill]
    add esp, 4
    mov eax, [esp+8]
    push dword [esp]
    push dword [eax+4]
    push dword [eax]
    call [__imp_Take]
    add esp, 0x10
    ret
    align 16, db 0xcc
; N5 split cleanup: Take(*p, 5) cleaned by add esp,8 then add esp,4
split:                                  ; 0x401240
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    push 5
    push dword [eax+4]
    push dword [eax]
    call [__imp_Take]
    add esp, 8
    add esp, 4
    pop ebp
    ret
    align 16, db 0xcc
; N6 control: Take(*p, 5) with add esp,0xc
podok:                                  ; 0x401260
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    push 5
    push dword [eax+4]
    push dword [eax]
    call [__imp_Take]
    add esp, 0xc
    pop ebp
    ret
    align 16, db 0xcc
entry:
    ret
section .rdata
import_dir:
    dd RVA(ilt_arr), 0, 0, RVA(dll_arr), RVA(iat_arr)
    times 5 dd 0
import_dir_end:
iat:
iat_arr:
__imp_Arr_copy:    dd RVA(hn_copy)
__imp_Pair_copy:   dd RVA(hn_pcopy)
__imp_MakeStack:   dd RVA(hn_makestack)
__imp_Sink:        dd RVA(hn_sink)
__imp_Take3:       dd RVA(hn_take3)
__imp_Log:         dd RVA(hn_log)
__imp_Six_copy:    dd RVA(hn_scopy)
__imp_Put:         dd RVA(hn_put)
__imp_Take4:       dd RVA(hn_take4)
__imp_Take:        dd RVA(hn_take)
__imp_Fill:        dd RVA(hn_fill)
    dd 0
iat_end:
ilt_arr: dd RVA(hn_copy), RVA(hn_pcopy), RVA(hn_makestack), RVA(hn_sink), RVA(hn_take3), RVA(hn_log), RVA(hn_scopy), RVA(hn_put), RVA(hn_take4), RVA(hn_take), RVA(hn_fill), 0
hn_copy: dw 0
    db '??0Arr@@QAE@ABV0@@Z', 0
    align 2, db 0
hn_pcopy: dw 0
    db '??0Pair@@QAE@ABV0@@Z', 0
    align 2, db 0
hn_makestack: dw 0
    db '?MakeStack@@YA?AVArr@@V1@AAV1@1@Z', 0
    align 2, db 0
hn_sink: dw 0
    db '?Sink@@YAXVPair@@@Z', 0
    align 2, db 0
hn_take3: dw 0
    db '?Take3@@YAXHVArr@@@Z', 0
    align 2, db 0
hn_log: dw 0
    db '?Log@@YAXVArr@@ZZ', 0
    align 2, db 0
hn_scopy: dw 0
    db '??0Six@@QAE@ABV0@@Z', 0
    align 2, db 0
hn_put: dw 0
    db '?Put@@YAXVSix@@H@Z', 0
    align 2, db 0
hn_take4: dw 0
    db '?Take4@@YAXHVPair@@@Z', 0
    align 2, db 0
hn_take: dw 0
    db '?Take@@YAXUPod@@H@Z', 0
    align 2, db 0
hn_fill: dw 0
    db '?Fill@@YAXPAH@Z', 0
    align 2, db 0
dll_arr: db 'arr.dll', 0
%include "pe32_data_end.inc"

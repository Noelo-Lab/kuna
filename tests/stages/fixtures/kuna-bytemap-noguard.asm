; MSVC-style two-stage jump tables (a byte map naming a row of 4-byte
; image-base-relative offsets) in three x64 functions, for #1157.
; nasm -f bin kuna-bytemap-noguard.asm -o kuna-bytemap-noguard.bin (loaded at 0x1820e0000)
bits 64
org 0x1820e0000
%define IMGB ($$ - 0x20e0000)
cmp_nodes:
  push rbx
  push r12
  push r13
  push r14
  sub rsp, 0x20
  mov rbx, rcx
  mov r14, rdx
.top:
  test rbx, rbx
  jz .ret0
  mov eax, [rbx]
  lea r12d, [rax - 2]
  cmp r12d, 199
  ja .dflt1
  cmp dword [rbx + 4], 1
  jle .a0
  add r13d, 1
  cmp r12d, 199
  ja .gd
  jmp .b0
.a0:
  sub r13d, 2
  cmp r12d, 199
  ja .gd
.b0:
  cmp dword [rbx + 4], 4
  jle .a1
  add r13d, 2
  cmp r12d, 199
  ja .gd
  jmp .b1
.a1:
  sub r13d, 3
  cmp r12d, 198
  ja .gd
.b1:
  lea r8, [rel IMGB]
  movsxd rax, r12d
  movzx eax, byte [r8 + rax + (cmp_nodes.map1 - IMGB)]
  mov ecx, [r8 + rax*4 + (cmp_nodes.tbl1 - IMGB)]
  add rcx, r8
  jmp rcx
.gd:
  jmp .dflt1
.case_flag:
  lea r8, [rel IMGB]
  movsxd rax, r12d
  movzx eax, byte [r8 + rax + (cmp_nodes.map2 - IMGB)]
  mov ecx, [r8 + rax*4 + (cmp_nodes.tbl2 - IMGB)]
  add rcx, r8
  jmp rcx
.case_rec:
  mov rcx, [rbx + 8]
  mov rdx, [r14 + 8]
  call cmp_nodes
  test eax, eax
  jnz .exit
  mov rbx, [rbx + 16]
  mov r14, [r14 + 16]
  jmp .top
.dflt1:
  mov eax, -1
  jmp .exit
.ret0:
  xor eax, eax
  jmp .exit
.c1:
  mov eax, [rbx + 4]
  sub eax, [r14 + 4]
  jmp .exit
.c2:
  mov eax, 2
  jmp .exit
.c4:
  mov eax, 4
  jmp .exit
.dflt2:
  mov eax, -2
.exit:
  add rsp, 0x20
  pop r14
  pop r13
  pop r12
  pop rbx
  ret
  align 16, db 0xcc
cmp_nodes.tbl1:
  dd cmp_nodes.case_flag - IMGB
  dd cmp_nodes.case_rec - IMGB
  dd cmp_nodes.dflt1 - IMGB
cmp_nodes.map1:
  ; tag 1..128: flags 1,2,4,8 -> case_flag; 16,32,64,128 -> case_rec; others -> dflt1
  %assign t 2
  %rep 200
    %if t = 2 || t = 4 || t = 8
      db 0
    %elif t = 9 || t = 16 || t = 32 || t = 64 || t = 128
      db 1
    %else
      db 2
    %endif
    %assign t t+1
  %endrep
  align 4, db 0
cmp_nodes.tbl2:
  dd cmp_nodes.c1 - IMGB
  dd cmp_nodes.c2 - IMGB
  dd cmp_nodes.c4 - IMGB
  dd cmp_nodes.dflt2 - IMGB
cmp_nodes.map2:
  %assign t 2
  %rep 200
    %if t = 2
      db 0
    %elif t = 3
      db 1
    %elif t = 4 || t = 8
      db 2
    %else
      db 3
    %endif
    %assign t t+1
  %endrep
  align 16, db 0xcc
cmp_folded:
  push rbx
  push r12
  push r13
  push r14
  sub rsp, 0x20
  mov rbx, rcx
  mov r14, rdx
.top:
  test rbx, rbx
  jz .ret0
  mov eax, [rbx]
  lea r12d, [rax - 2]
  cmp r12d, 199
  ja .dflt1
  cmp dword [rbx + 4], 1
  jle .a0
  add r13d, 1
  cmp r12d, 199
  ja .gd
  jmp .b0
.a0:
  sub r13d, 2
  cmp r12d, 199
  ja .gd
.b0:
  cmp dword [rbx + 4], 4
  jle .a1
  add r13d, 2
  cmp r12d, 199
  ja .gd
  jmp .b1
.a1:
  sub r13d, 3
  cmp r12d, 198
  ja .gd
.b1:
  lea r8, [rel IMGB]
  movsxd rax, dword [rbx]
  movzx eax, byte [r8 + rax + (cmp_folded.map1 - 2 - IMGB)]
  mov ecx, [r8 + rax*4 + (cmp_folded.tbl1 - IMGB)]
  add rcx, r8
  jmp rcx
.gd:
  jmp .dflt1
.case_flag:
  lea r8, [rel IMGB]
  movsxd rax, r12d
  movzx eax, byte [r8 + rax + (cmp_folded.map2 - IMGB)]
  mov ecx, [r8 + rax*4 + (cmp_folded.tbl2 - IMGB)]
  add rcx, r8
  jmp rcx
.case_rec:
  mov rcx, [rbx + 8]
  mov rdx, [r14 + 8]
  call cmp_folded
  test eax, eax
  jnz .exit
  mov rbx, [rbx + 16]
  mov r14, [r14 + 16]
  jmp .top
.dflt1:
  mov eax, -1
  jmp .exit
.last:
  mov eax, 0x33
  jmp .exit
.ret0:
  xor eax, eax
  jmp .exit
.c1:
  mov eax, [rbx + 4]
  sub eax, [r14 + 4]
  jmp .exit
.c2:
  mov eax, 2
  jmp .exit
.c4:
  mov eax, 4
  jmp .exit
.dflt2:
  mov eax, -2
.exit:
  add rsp, 0x20
  pop r14
  pop r13
  pop r12
  pop rbx
  ret
  align 16, db 0xcc
cmp_folded.tbl1:
  dd cmp_folded.case_flag - IMGB
  dd cmp_folded.case_rec - IMGB
  dd cmp_folded.last - IMGB
cmp_folded.map1:
  ; tag 1..128: flags 1,2,4,8 -> case_flag; 16,32,64,128 -> case_rec; others -> dflt1
  %assign t 2
  %rep 200
    %if t = 2 || t = 4 || t = 8
      db 0
    %elif t = 9 || t = 16 || t = 32 || t = 64 || t = 128
      db 1
    %else
      db 2
    %endif
    %assign t t+1
  %endrep
  align 4, db 0
cmp_folded.tbl2:
  dd cmp_folded.c1 - IMGB
  dd cmp_folded.c2 - IMGB
  dd cmp_folded.c4 - IMGB
  dd cmp_folded.dflt2 - IMGB
cmp_folded.map2:
  %assign t 2
  %rep 200
    %if t = 2
      db 0
    %elif t = 3
      db 1
    %elif t = 4 || t = 8
      db 2
    %else
      db 3
    %endif
    %assign t t+1
  %endrep
  align 16, db 0xcc
cmp_bytetag:
  movzx edx, byte [rcx]
  cmp edx, 15
  ja .dflt
  cmp dword [rcx + 4], 0
  jle .a
  cmp edx, 15
  ja .dflt
  jmp .go
.a:
  cmp edx, 14
  ja .dflt
.go:
  lea r8, [rel IMGB]
  movzx eax, byte [r8 + rdx + (cmp_bytetag.map - IMGB)]
  mov ecx, [r8 + rax*4 + (cmp_bytetag.tbl - IMGB)]
  add rcx, r8
  jmp rcx
.c0:
  mov eax, 20
  ret
.c1:
  mov eax, 21
  ret
.dflt:
  mov eax, -3
  ret
  align 16, db 0xcc
cmp_bytetag.tbl:
  dd cmp_bytetag.c0 - IMGB, cmp_bytetag.c1 - IMGB, cmp_bytetag.dflt - IMGB
cmp_bytetag.map:
  db 0, 2, 0, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2
  db 3
  align 16, db 0
  times 1024 db 0

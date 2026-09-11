.intel_syntax noprefix
.globl sub_140002930
.type sub_140002930,@function
sub_140002930:
  push rbx
  push r13
  sub rsp, 32
  xor r13d, r13d
  mov ebx, r13d
.Louter:
  cmp byte ptr [rip+initialized], r13b
  jne .Lgenerate
  xor ecx, ecx
  call init_one
  mov rcx, rax
  call init_two
  mov byte ptr [rip+initialized], 1
.Lgenerate:
  call helper
  and eax, 0x8000ffff
  jge .Lnonneg
  dec eax
  or eax, 0xffff0000
  inc eax
.Lnonneg:
  xor ecx, ecx
.Linner:
  cmp word ptr [rsp+rcx*2], ax
  je .Louter
  inc rcx
  cmp rcx, 16
  jl .Linner
  mov word ptr [rsp+rbx*2], ax
  inc rbx
  cmp rbx, 16
  jl .Louter
  add rsp, 32
  pop r13
  pop rbx
  ret
.size sub_140002930,.-sub_140002930

.globl helper
.type helper,@function
helper:
  mov eax, 7
  ret
.size helper,.-helper

.type init_one,@function
init_one:
  xor eax, eax
  ret
.size init_one,.-init_one

.type init_two,@function
init_two:
  ret
.size init_two,.-init_two

.data
.type initialized,@object
.size initialized,1
initialized:
  .byte 0

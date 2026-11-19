
.intel_syntax noprefix
.text
.globl f
.type f,@function
f:
  .cfi_startproc
  test edi, edi
  je 1f
  mov eax, 1
  ret
1:
  call mydie@PLT
  .cfi_endproc
.size f, .-f
  mov eax, 7
  ret

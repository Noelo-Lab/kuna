# (kuna) Option `coldentry`.  GCC hot/cold splitting in miniature: `check`'s two
# unlikely paths are split into ONE `.cold` fragment with ONE FDE, laid back to
# back, each entered by its own `jcc rel32` from the hot body.  The FDE names only
# the first path; the second (`cold_two`) sits strictly inside the FDE body after
# a `ud2`.  See decompiler/crates/kuna-analysis/tests/fixtures/README.md.
    .text

    .type check.cold, @function
check.cold:
    .cfi_startproc
cold_one:
    mov  $1,%edi
    call die
    ud2
cold_two:
    mov  $2,%edi
    call die
    ud2
    .cfi_endproc
    .size check.cold, .-check.cold

    .align 16
    .type die, @function
die:
    .cfi_startproc
    mov  $60,%eax
    syscall
    hlt
    .cfi_endproc
    .size die, .-die

    .align 16
    .globl check
    .type check, @function
check:
    .cfi_startproc
    test %rdi,%rdi
    {disp32} je cold_one
    cmp  $1,%rdi
    {disp32} je cold_two
    lea  1(%rdi),%rax
    ret
    .cfi_endproc
    .size check, .-check

    .align 16
    .globl _start
    .type _start, @function
_start:
    .cfi_startproc
    mov  (%rsp),%rdi
    call check
    mov  %eax,%edi
    mov  $60,%eax
    syscall
    .cfi_endproc
    .size _start, .-_start

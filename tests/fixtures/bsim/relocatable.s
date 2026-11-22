# Assemble from the repo root with GNU as 2.46:
# as --64 -mx86-used-note=no -o tests/fixtures/bsim/relocatable.o tests/fixtures/bsim/relocatable.s
.section .text.rel_a,"ax",@progbits
.globl original_rel_a
.type original_rel_a,@function
original_rel_a:
    sub $8, %rsp
    call original_rel_b
    add $8, %rsp
    add $17, %eax
    ret
.size original_rel_a, .-original_rel_a

.section .text.rel_b,"ax",@progbits
.globl original_rel_b
.type original_rel_b,@function
original_rel_b:
    mov $23, %eax
    ret
.size original_rel_b, .-original_rel_b

.section .note.GNU-stack,"",@progbits

    .section .text.selector_b,"ax",@progbits
    .local duplicate_local
    .type duplicate_local,@function
duplicate_local:
    mov $2, %eax
    ret
    .size duplicate_local, .-duplicate_local

    .section .text.selector_a,"ax",@progbits
    .local duplicate_local
    .type duplicate_local,@function
duplicate_local:
    mov $1, %eax
    ret
    .size duplicate_local, .-duplicate_local

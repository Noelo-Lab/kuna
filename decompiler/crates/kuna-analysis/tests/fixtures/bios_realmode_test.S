
.code16
.section .text
.org 0xe000
post_code:
    mov $0x80, %dx
    out %al, %dx
    ret
.org 0xe05b
reset:
    cli
    cld
    mov $0x55, %al
    call post_code
    mov $0x1000, %ax
    mov %ax, %ds
    movw $0x1234, (0x10)
    mov $0xaa, %al
    call post_code
halt:
    hlt
    jmp halt
.org 0xfff0
    ljmp $0xf000, $0xe05b
.org 0x10000

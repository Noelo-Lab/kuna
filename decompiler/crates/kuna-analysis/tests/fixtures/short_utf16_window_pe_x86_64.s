# RE-friction need `short-utf-16-window`: Windows uses two-byte wchar_t, so a
# PE fixture is required to distinguish wchar_t[3] from the char[2] mapping
# operand_refs derives from the low byte and high-byte NUL of UTF-16 "ID".
# Build from the repository root:
#   clang -target x86_64-windows-gnu -c THIS.s -o /tmp/short_utf16.o
#   ld -mi386pep --image-base 0x400000 --section-alignment 0x1000 \
#     --file-alignment 0x200 --subsystem console -e entry \
#     --section-start .text=0x401000 --section-start .rdata=0x402000 \
#     /tmp/short_utf16.o -o THIS.exe
.section .text,"xr"
.global entry
entry:
    sub $40, %rsp
    mov $window_class, %rax
    mov %rax, 32(%rsp)
    mov 32(%rsp), %rcx
    xor %edx, %edx
    call find_window
    mov $adjacent_wide, %rcx
    xor %edx, %edx
    call find_window
    mov $ascii_text, %rcx
    call use_ascii
    add $40, %rsp
    ret

.p2align 4
.global find_window
find_window:
    xor %eax, %eax
    ret

.global use_ascii
use_ascii:
    ret

.section .rdata,"dr"
.global window_class
window_class:
    .byte 0x49, 0x00, 0x44, 0x00, 0x00, 0x00
    .byte 0xff, 0xff

.global adjacent_wide
adjacent_wide:
    .byte 0x4f, 0x00, 0x4c, 0x00, 0x4c, 0x00, 0x59, 0x00
    .byte 0x44, 0x00, 0x42, 0x00, 0x47, 0x00, 0x00, 0x00

.global ascii_text
ascii_text:
    .asciz "ASCII"

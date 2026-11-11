.section .comment,"",@progbits
.asciz "rustc version 1.93.1 (01f6ddf75 2026-02-11)"
.section .rodata
msg: .ascii "hello\n"
.text
.globl show
.type show,@function
show:
 sub $8,%rsp; lea msg(%rip),%rdi; mov $13,%rsi
 call *"std::io::stdio::_print"@GOTPCREL(%rip)
 add $8,%rsp; xor %eax,%eax; ret
.size show,.-show
.globl "std::io::stdio::_print"
.type "std::io::stdio::_print",@function
"std::io::stdio::_print": ret
.size "std::io::stdio::_print",.-"std::io::stdio::_print"
.section .note.GNU-stack,"",@progbits

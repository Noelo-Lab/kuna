.text
.Ltext:
.global _start
_start:
    call .Lyield
    call .Lstop
    call .Lthrow

.p2align 4,0xcc
.Lyield:
    lea .Lresume_descriptor(%rip),%rax
    call .Lswitch
    movb $0x5a,.Lsentinel(%rip)
    ret

.p2align 4,0xcc
.Lswitch:
    mov (%rsp),%rcx
    mov %rcx,.Lsaved_pc(%rip)
    lea 8(%rsp),%rcx
    mov %rcx,.Lsaved_sp(%rip)
    lea .Lscheduler_stack_end(%rip),%rsp
    call *(%rax)
    jmp .Lthrow

.p2align 4,0xcc
.Lresume:
    mov .Lsaved_sp(%rip),%rsp
    jmp *.Lsaved_pc(%rip)

.p2align 4,0xcc
.Lstop:
    call .Lterminal_wrapper
    movb $0x5b,.Lsentinel(%rip)
    ret

.p2align 4,0xcc
.Lterminal_wrapper:
    call .Lexit
    ud2

.p2align 4,0xcc
.Lexit:
    jmp .Lthrow

.p2align 4,0xcc
.Lthrow:
    ud2
.Lend:

.section .rodata,"a"
.p2align 3
.Lresume_descriptor: .quad .Lresume

.data
.p2align 3
.Lsaved_pc: .quad 0
.Lsaved_sp: .quad 0
.Lsentinel: .byte 0
.p2align 4
.zero 64
.Lscheduler_stack_end:

.section .go.buildinfo,"aw"
.p2align 4
.ascii "\377 Go buildinf:"
.byte 8,2
.zero 16
.byte 8
.ascii "go1.27.2"
.byte 0

.section .gopclntab,"a"
.p2align 3
.Lheader:
.long 0xfffffff1
.byte 0,0,1,8
.quad 8,0,.Ltext,.Lnames-.Lheader,0,0,0,.Lfunctions-.Lheader
.Lnames:
.Ln_start: .asciz "main.entry"
.Ln_yield: .asciz "main.yield"
.Ln_switch: .asciz "runtime.mcall"
.Ln_resume: .asciz "runtime.gosched_m"
.Ln_stop: .asciz "main.stop"
.Ln_terminal_wrapper: .asciz "main.mcall"
.Ln_exit: .asciz "runtime.goexit0"
.Ln_throw: .asciz "runtime.throw"
.p2align 3
.Lfunctions:
.long .Ltext-.Ltext,.Lf_start-.Lfunctions
.long .Lyield-.Ltext,.Lf_yield-.Lfunctions
.long .Lswitch-.Ltext,.Lf_switch-.Lfunctions
.long .Lresume-.Ltext,.Lf_resume-.Lfunctions
.long .Lstop-.Ltext,.Lf_stop-.Lfunctions
.long .Lterminal_wrapper-.Ltext,.Lf_terminal_wrapper-.Lfunctions
.long .Lexit-.Ltext,.Lf_exit-.Lfunctions
.long .Lthrow-.Ltext,.Lf_throw-.Lfunctions
.long .Lend-.Ltext

.macro record entry, name
.long \entry-.Ltext,\name-.Lnames
.zero 36
.endm
.Lf_start: record .Ltext,.Ln_start
.Lf_yield: record .Lyield,.Ln_yield
.Lf_switch: record .Lswitch,.Ln_switch
.Lf_resume: record .Lresume,.Ln_resume
.Lf_stop: record .Lstop,.Ln_stop
.Lf_terminal_wrapper: record .Lterminal_wrapper,.Ln_terminal_wrapper
.Lf_exit: record .Lexit,.Ln_exit
.Lf_throw: record .Lthrow,.Ln_throw

.section .note.GNU-stack,"",@progbits

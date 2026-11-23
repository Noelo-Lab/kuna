.text
.Ltext:
.global _start
_start:
    ret
.p2align 4,0xcc
.Lcheck:
    test %rax,%rax
    jne .Lthrow_path
    ret
.Lthrow_path:
    call .Lthrow
    ud2
.p2align 4,0xcc
.Lshift:
    call .Lcheck
    call .Lgopanic
    movb $0x5a,.Lafter_panic(%rip)
    ret
.p2align 4,0xcc
.Lgopanic:
    ud2
.p2align 4,0xcc
.Lthrow:
    ud2
.Lend:

.data
.Lafter_panic: .byte 0

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
.quad 5,0,.Ltext,.Lnames-.Lheader,0,0,0,.Lfunctions-.Lheader
.Lnames:
.Ln_start: .asciz "main.entry"
.Ln_check: .asciz "runtime.panicCheck1"
.Ln_shift: .asciz "runtime.panicshift"
.Ln_panic: .asciz "runtime.gopanic"
.Ln_throw: .asciz "runtime.throw"
.p2align 3
.Lfunctions:
.long .Ltext-.Ltext,.Lf_start-.Lfunctions
.long .Lcheck-.Ltext,.Lf_check-.Lfunctions
.long .Lshift-.Ltext,.Lf_shift-.Lfunctions
.long .Lgopanic-.Ltext,.Lf_panic-.Lfunctions
.long .Lthrow-.Ltext,.Lf_throw-.Lfunctions
.long .Lend-.Ltext

.macro record entry, name
.long \entry-.Ltext,\name-.Lnames
.zero 36
.endm
.Lf_start: record .Ltext,.Ln_start
.Lf_check: record .Lcheck,.Ln_check
.Lf_shift: record .Lshift,.Ln_shift
.Lf_panic: record .Lgopanic,.Ln_panic
.Lf_throw: record .Lthrow,.Ln_throw

.section .note.GNU-stack,"",@progbits

/* One real pointer method topology and a receiver-looking ordinary function. */
.text
.globl runtime.text,_start
.type runtime.text,@object
runtime.text:
_start:
    ret

.macro method name
.globl "\name"
.type "\name",@function
"\name":
    mov 8(%rax),%rcx
    add (%rax),%rcx
    add %rbx,%rcx
    mov %rcx,8(%rax)
    mov %rcx,%rax
    ret
.size "\name",.-"\name"
.endm
method "main.(*Meter).Apply"
method "main.(*Meter).Pretend"
method "main.Apply"
.globl main.Forward
.type main.Forward,@function
main.Forward:
    call "main.(*Meter).Apply"
    ret
.size main.Forward,.-main.Forward
.globl main.ForwardShadow
.type main.ForwardShadow,@function
main.ForwardShadow:
    call "main.(*Meter).Apply"
    ret
.size main.ForwardShadow,.-main.ForwardShadow
.LtextEnd:

.section .gopclntab,"a",@progbits
.Lheader:
    .long 0xfffffff1
    .byte 0,0,1,8
    .quad 5,0,runtime.text
    .quad .LfnNames-.Lheader,0,0,0,.Lftab-.Lheader
.LfnNames:
.LapplyName: .asciz "main.(*Meter).Apply"
.LpretendName: .asciz "main.(*Meter).Pretend"
.LfreeName: .asciz "main.Apply"
.LforwardName: .asciz "main.Forward"
.LshadowName: .asciz "main.ForwardShadow"
.LfnNamesEnd:
.Lftab:
    .long "main.(*Meter).Apply"-runtime.text,.Lapply-.Lftab
    .long "main.(*Meter).Pretend"-runtime.text,.Lpretend-.Lftab
    .long main.Apply-runtime.text,.Lfree-.Lftab
    .long main.Forward-runtime.text,.Lforward-.Lftab
    .long main.ForwardShadow-runtime.text,.Lshadow-.Lftab
    .long .LtextEnd-runtime.text,0
.macro function_entry label, symbol, name
\label:
    .long "\symbol"-runtime.text,\name-.LfnNames
    .zero 36
.endm
function_entry .Lapply,"main.(*Meter).Apply",.LapplyName
function_entry .Lpretend,"main.(*Meter).Pretend",.LpretendName
function_entry .Lfree,main.Apply,.LfreeName
function_entry .Lforward,main.Forward,.LforwardName
function_entry .Lshadow,main.ForwardShadow,.LshadowName

.section .rodata,"a",@progbits
.p2align 3
.globl runtime.types,runtime.etypes
.type runtime.types,@object
.type runtime.etypes,@object
runtime.types:
    .quad 0
.macro goname label, text
\label:
    .byte 1,1f-0f
0:  .ascii "\text"
1:
.endm
goname .Lpackage,main
goname .LmeterName,main.Meter
goname .LpointerName,*main.Meter
goname .Lu64Name,uint64
goname .LsigName,"func(uint64) uint64"
goname .LbaseName,Base
goname .LtotalName,Total
goname .LmethodName,Apply
.p2align 3
.macro type_base size, flags, kind, name
    .quad \size,0
    .long 0
    .byte \flags,8,8,\kind
    .quad 0,0
    .long \name-runtime.types,0
.endm
.Lu64:
    type_base 8,0,11,.Lu64Name
.Lsig:
    type_base 8,0,19,.LsigName
    .short 1,1
    .long 0
    .quad .Lu64,.Lu64
.Lmeter:
    type_base 16,5,25,.LmeterName
    .quad 0,.Lfields,2,2
    .long .Lpackage-runtime.types
    .short 0,0
    .long 0,0
.Lfields:
    .quad .LbaseName,.Lu64,0
    .quad .LtotalName,.Lu64,8
.Lpointer:
    type_base 8,1,22,.LpointerName
    .quad .Lmeter
    .long .Lpackage-runtime.types
    .short 1,1
    .long 16,0
    .long .LmethodName-runtime.types,.Lsig-runtime.types
    .long "main.(*Meter).Apply"-runtime.text,"main.(*Meter).Apply"-runtime.text
runtime.etypes:

.section .typelink,"a",@progbits
.Ltypelink:
    .long .Lpointer-runtime.types,.Lmeter-runtime.types,.Lu64-runtime.types,.Lsig-runtime.types

.section .noptrdata,"aw",@progbits
.p2align 3
.globl runtime.firstmoduledata
.type runtime.firstmoduledata,@object
runtime.firstmoduledata:
    .quad .Lheader,.LfnNames,.LfnNamesEnd-.LfnNames,.LfnNamesEnd-.LfnNames
    .zero (17-4)*8
    .quad 6,6,0,runtime.text,.LtextEnd,runtime.text,.LtextEnd
    .zero (37-24)*8
    .quad runtime.types,runtime.etypes
    .zero (44-39)*8
    .quad .Ltypelink,4,4
    .zero (96-47)*8
.size runtime.firstmoduledata,.-runtime.firstmoduledata

.section .go.buildinfo,"aw",@progbits
.p2align 4
    .byte 0xff
    .ascii " Go buildinf:"
    .byte 8,2
    .zero 16
    .byte 9
    .ascii "go1.22.10"
    .byte 0

.section .debug_abbrev
    .uleb128 1,0x11
    .byte 1
    .uleb128 0x03,0x08,0x25,0x08,0x13,0x05,0,0
    .uleb128 2,0x24
    .byte 0
    .uleb128 0x03,0x08,0x3e,0x0b,0x0b,0x0b,0x2900,0x0b,0x2904,0x01,0,0
    .uleb128 3,0x0f
    .byte 0
    .uleb128 0x03,0x08,0x0b,0x0b,0x49,0x13,0x2900,0x0b,0x2904,0x01,0,0
    .uleb128 4,0x13
    .byte 1
    .uleb128 0x03,0x08,0x0b,0x0b,0x2900,0x0b,0x2904,0x01,0,0
    .uleb128 5,0x0d
    .byte 0
    .uleb128 0x03,0x08,0x49,0x13,0x38,0x0b,0,0
    .uleb128 6,0x2e
    .byte 1
    .uleb128 0x03,0x08,0x11,0x01,0x12,0x06,0,0
    .uleb128 7,0x05
    .byte 0
    .uleb128 0x03,0x08,0x4b,0x0c,0x49,0x13,0x02,0x18,0,0
    .uleb128 0

.section .debug_info
.Lunit:
    .long .LunitEnd-.LunitBody
.LunitBody:
    .short 4
    .long 0
    .byte 8
    .uleb128 1
    .asciz "main"
    .asciz "Go cmd/compile go1.22.10; regabi"
    .short 0x16
.LdU64:
    .uleb128 2
    .asciz "uint64"
    .byte 7,8,11
    .quad .Lu64-runtime.types
.LdMeter:
    .uleb128 4
    .asciz "main.Meter"
    .byte 16,25
    .quad .Lmeter-runtime.types
    .uleb128 5
    .asciz "Base"
    .long .LdU64-.Lunit
    .byte 0
    .uleb128 5
    .asciz "Total"
    .long .LdU64-.Lunit
    .byte 8
    .byte 0
.LdPtr:
    .uleb128 3
    .asciz "*main.Meter"
    .byte 8
    .long .LdMeter-.Lunit
    .byte 22
    .quad .Lpointer-runtime.types
.macro prototype symbol, size, receiver=m
    .uleb128 6
    .asciz "\symbol"
    .quad "\symbol"
    .long \size
    .uleb128 7
    .asciz "\receiver"
    .byte 0
    .long .LdPtr-.Lunit
    .byte 1,0x50
    .uleb128 7
    .asciz "delta"
    .byte 0
    .long .LdU64-.Lunit
    .byte 1,0x53
    .uleb128 7
    .asciz "r"
    .byte 1
    .long .LdU64-.Lunit
    .byte 0
    .byte 0
.endm
prototype "main.(*Meter).Apply",18
prototype "main.(*Meter).Pretend",18
prototype main.Apply,18
prototype main.Forward,6
prototype main.ForwardShadow,6,Meter
    .byte 0
.LunitEnd:
.section .note.GNU-stack,"",@progbits

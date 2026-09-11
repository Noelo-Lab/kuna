# Fixture for the `callpopret` option (RE-friction need
# call-pop-pointer-helper).  `dllname` hands back a pointer to a string that
# sits inline immediately after its own `call`, the oldest position-independent
# idiom on x86 and the one a packer uses to carry its import names:
#
#     dllname: call    getptr         # the return address is the pad byte below
#              .byte   0x90           # control never arrives here
#     .Lstr:   .asciz  "kernel32.dll" # ... nor here: this is data
#              ...
#     getptr:  popl    %eax           # EAX = the pushed return address
#              incl    %eax           # step past the pad byte -> .Lstr
#              ret                    # return to dllname's OWN caller
#
# `CALL` is a fall-through op, so decoded as one the name table lifts as
# instructions: `6c` is INSB, and the emitted C fills with `in(...)` port reads
# and stores through registers that were never set, while the pointer the helper
# exists to return appears nowhere.  Flowed through, `dllname` is the one-liner
# it is.
#
# Build:
#   gcc -m32 -nostdlib -static -Wl,-e,_start -o callpopret_i386 callpopret_i386.s
    .text
    .globl _start
_start:
    call    dllname
    pushl   %eax
    hlt

    .globl  dllname
    .type   dllname, @function
dllname:
    call    getptr
    .byte   0x90
.Lnames:
    .asciz  "kernel32.dll"
    .asciz  "GetProcAddress"
    .asciz  "LoadLibraryA"
    .asciz  "VirtualAlloc"
    .asciz  "VirtualFree"
    .asciz  "VirtualProtect"
    .size   dllname, .-dllname

    .globl  getptr
    .type   getptr, @function
getptr:
    popl    %eax
    incl    %eax
    ret
    .size   getptr, .-getptr

# Landing pad.  With `callpopret` OFF the name table is decoded as instructions,
# and the conditional branches that junk contains aim past the end of the
# section; without somewhere to land the decode fails outright instead of
# showing the defect.  Every byte here is a `ret`.
    .fill   256, 1, 0xc3

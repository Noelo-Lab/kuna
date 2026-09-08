# Fixture for the `calltrampoline` option (RE-friction need
# beria-call-fragments-produce).  `protected` reaches its continuation through a
# return-address-discarding fragment, the shape a Beria-family PE protector puts
# in front of every protected function:
#
#     call    tramp          # the return address is the junk byte below
#     .byte   0xa3           # control never arrives here
# .Lresume:
#     add     $0x5c, %eax    # the real continuation
#
#     tramp:  lea 0x4(%esp), %esp   # throw the pushed return address away
#             add $0x12, %eax
#             jmp .Lresume          # resume in the caller, one byte past
#
# Decoded as a fall-through, the junk lead byte swallows the continuation:
# `a3 83 c0 5c 5d` is `MOV [0x5d5cc083],EAX`, a store to a global that does not
# exist, and the two adds never appear.  Flowed through, the body is `a0 + 0x6e`.
#
# Build:
#   gcc -m32 -nostdlib -static -Wl,-e,_start -o calltrampoline_i386 calltrampoline_i386.s
    .text
    .globl _start
_start:
    pushl   $7
    call    protected
    addl    $4, %esp
    hlt

    .globl  protected
    .type   protected, @function
protected:
    pushl   %ebp
    movl    %esp, %ebp
    movl    0x8(%ebp), %eax
    call    tramp
    .byte   0xa3
.Lresume:
    addl    $0x5c, %eax
    popl    %ebp
    ret
    .size   protected, .-protected

    .globl  tramp
    .type   tramp, @function
tramp:
    leal    0x4(%esp), %esp
    addl    $0x12, %eax
    jmp     .Lresume
    .size   tramp, .-tramp

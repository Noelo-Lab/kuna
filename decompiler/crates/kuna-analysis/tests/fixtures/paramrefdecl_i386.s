# Fixture for the `paramrefdecl` option (RE-friction need
# address-taken-key-parameter).  `derive_key` is an i386 cdecl function whose
# first parameter is a four-byte key it passes BY ADDRESS (`lea 0x8(%ebp)`), so
# nothing in the body ever reads that parameter's frame slot as a value.  With
# `paramrefdecl` off the `&a0` reference is declared as a body local under the
# signature parameter's own name -- two objects of one name in one scope.
    .text
    .globl _start
_start:
    pushl   $2
    pushl   $1
    call    derive_key
    addl    $8, %esp
    hlt

    .globl  derive_key
    .type   derive_key, @function
derive_key:
    pushl   %ebp
    movl    %esp, %ebp
    leal    0x8(%ebp), %eax
    pushl   0xc(%ebp)
    pushl   %eax
    call    consume
    addl    $8, %esp
    popl    %ebp
    ret
    .size   derive_key, .-derive_key

    .globl  consume
    .type   consume, @function
consume:
    movl    0x4(%esp), %eax
    movl    (%eax), %eax
    addl    0x8(%esp), %eax
    ret
    .size   consume, .-consume

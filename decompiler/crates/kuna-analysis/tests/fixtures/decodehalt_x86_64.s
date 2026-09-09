        .text
        .globl  stub
        .type   stub, @function
stub:
        movabs  $payload, %rsi
        movl    $0x20, %ecx
1:      xorb    $0x5a, (%rsi)
        incq    %rsi
        decl    %ecx
        jnz     1b
        pushq   $payload
        popq    %rbx
        jmp     *%rbx
        .size   stub, .-stub

        .data
        .globl  payload
payload:
        .byte 0x06
        .fill 0x3ff, 1, 0x00

/* Original Linux user-mode entry; supply EXIT_NR=6058 (N32) or 5058 (N64). */
.set noreorder
.option pic0
.text
.globl _start
.type _start,@function
_start:
    dla $25,main
    jalr $25
    nop
    move $4,$2
    li $2,EXIT_NR
    syscall
.size _start,.-_start

# (kuna) `protoorder` fixture -- two functions whose own recovered prototypes
# are the only evidence their callers have, and one whose recovery UNDER-counts.
#
# `callee` reads rdi as a byte pointer and esi as a number, so decompiling it
# alone recovers `(unsigned char *, int)`.  `caller` passes both but holds
# neither type: with the option off it renders `unsigned long caller(unsigned
# long a0, int a1)` and `_start` passes it a bare `0x402000`.  With the option
# on, `callee`'s recovered prototype is parked before `caller` is decompiled,
# so the pointer and the `int` return reach `caller`'s own signature and
# `_start`'s call site.  (The `mov $0,%edx` before the call is real caller
# code, so the third argument `callee(a0,a1,0)` stands in BOTH arms: a park
# states what the callee recovered, it never deletes what a caller passes.)
#
# `thunk` is the under-recovery case: it loads a function pointer out of its
# first argument and tail-jumps to it, so its own recovery sees ONE parameter
# while `thunkcaller` really passes five.  Parking a one-parameter prototype as
# a closed list would delete four live arguments from the call; the parked list
# is an open floor, so all five survive.
#
# `mixed` is the deletion pin, the shape that made `findutils/find -O2` drop an
# argument the machine code passes.  It calls `sink` with two arguments and,
# later in the same body, `overrec`.  `overrec` really takes one parameter, but
# a nested call reads `rdx` it never writes, so its OWN recovery reports three:
# parking that list makes every caller materialise two register slots that are
# nothing, and those fabricated reads join the caller's dataflow and take `rsi`
# away from the EARLIER `sink` call.  The tail carries the burden of proof, so
# `overrec` is parked with the one parameter its body proves it reads and
# `sink(a0,a1)` keeps both arguments in both arms.
#
# `vfmt` is the register-file-saturating shape: a SysV variadic that calls
# `va_start` opens with the register-save prologue, which READS every argument
# register the convention has, so recovery reports six parameters for a function
# its callers hand one.  Stating that list gives every call site five arguments
# the machine code never writes -- and nothing in the callee's body can say
# otherwise, because the seventh slot is a stack offset the entry walk cannot
# see.  A recovered list that ends on the last argument register is declined.
#
# Built with:  as -o f.o f.s && ld -o protoorder_x86_64 f.o
        .text
        .globl callee
        .type callee, @function
callee:
        movzbl  (%rdi), %eax
        addl    %esi, %eax
        ret
        .size callee, .-callee

        .globl caller
        .type caller, @function
caller:
        pushq   %rbx
        movl    %esi, %ebx
        movq    %rdi, %rdi
        testl   %ebx, %ebx
        je      .Lout
        movl    $0, %edx
        movl    %ebx, %esi
        call    callee
        popq    %rbx
        ret
.Lout:
        xorl    %eax, %eax
        popq    %rbx
        ret
        .size caller, .-caller

        .globl thunk
        .type thunk, @function
thunk:
        movq    80(%rdi), %rax
        jmp     *%rax
        .size thunk, .-thunk

        .globl thunkcaller
        .type thunkcaller, @function
thunkcaller:
        movl    $4, %r8d
        movl    $3, %ecx
        movl    $2, %edx
        movl    $1, %esi
        jmp     thunk
        .size thunkcaller, .-thunkcaller


# Reads every argument register it is given: the register-save shape, which is
# why `overrec`'s `rdx` looks like a parameter it must have been handed.
        .globl vararg
        .type vararg, @function
vararg:
        movq    %rdx, %rax
        addq    %rsi, %rax
        addq    %rdi, %rax
        ret
        .size vararg, .-vararg

# One real parameter (`rdi`, read straight away), one register written before it
# is ever read (`rsi`), and one that only a nested call touches (`rdx`).
        .globl overrec
        .type overrec, @function
overrec:
        pushq   %rbx
        movq    %rdi, %rbx
        movl    $16, %esi
        leaq    msg(%rip), %rdi
        call    vararg
        movq    %rax, (%rbx)
        popq    %rbx
        ret
        .size overrec, .-overrec

# Recovers nothing about itself, so its call sites are typed by the CALLER
# alone -- the position `setfilecon@plt` is in.
        .globl sink
        .type sink, @function
sink:
        ret
        .size sink, .-sink

        .globl mixed
        .type mixed, @function
mixed:
        pushq   %rbx
        subq    $16, %rsp
        movq    %rsi, %rbx
        testq   %rdi, %rdi
        je      .Lmx
        movq    %rbx, %rsi
        call    sink
.Lmx:
        movq    %rsp, %rdi
        call    overrec
        movq    (%rsp), %rax
        addq    $16, %rsp
        popq    %rbx
        ret
        .size mixed, .-mixed

        .globl _start
        .type _start, @function
_start:
        leaq    buf(%rip), %rdi
        movl    $3, %esi
        call    caller
        leaq    tab(%rip), %rdi
        call    thunkcaller
        leaq    buf(%rip), %rdi
        movl    $7, %esi
        call    mixed
        call    vfmtcaller
        movl    $60, %eax
        xorl    %edi, %edi
        syscall
        .size _start, .-_start

# The SysV register-save prologue, byte for byte what gcc emits for
# `static void vfmt(const char *fmt, ...)` at -O2: the five argument registers
# past the named one are spilled to the save area, so every one of them is READ
# before it is written and recovery reports six parameters.
        .globl vfmt
        .type vfmt, @function
vfmt:
        subq    $216, %rsp
        movq    %rdi, %r10
        movq    %rsi, 40(%rsp)
        movq    %rdx, 48(%rsp)
        movq    %rcx, 56(%rsp)
        movq    %r8, 64(%rsp)
        movq    %r9, 72(%rsp)
        movzbl  (%r10), %eax
        andl    $7, %eax
        movq    40(%rsp,%rax,8), %rax
        addq    $216, %rsp
        ret
        .size vfmt, .-vfmt

# Passes exactly one argument, the way `xlog("start\n")` does.
        .globl vfmtcaller
        .type vfmtcaller, @function
vfmtcaller:
        leaq    msg(%rip), %rdi
        xorl    %eax, %eax
        call    vfmt
        ret
        .size vfmtcaller, .-vfmtcaller

        .data
buf:    .asciz "abc"
msg:    .asciz "m"
        .align 8
tab:    .quad 0,0,0,0,0,0,0,0,0,0
        .quad callee

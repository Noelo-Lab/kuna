.text

/* uppercase final component */
.balign 16
.globl "_ZN4demo17hABCDEF0123456789E"
.type "_ZN4demo17hABCDEF0123456789E", @function
"_ZN4demo17hABCDEF0123456789E":
    mov $1, %eax
    ret
.size "_ZN4demo17hABCDEF0123456789E", .-"_ZN4demo17hABCDEF0123456789E"

/* mixed-case final component */
.balign 16
.globl "_ZN5mixed17hAbCdE0123456789fE"
.type "_ZN5mixed17hAbCdE0123456789fE", @function
"_ZN5mixed17hAbCdE0123456789fE":
    mov $2, %eax
    ret
.size "_ZN5mixed17hAbCdE0123456789fE", .-"_ZN5mixed17hAbCdE0123456789fE"

/* uppercase member plus true compiler hash */
.balign 16
.globl "_ZN6member17hABCDEF012345678917h0123456789abcdefE"
.type "_ZN6member17hABCDEF012345678917h0123456789abcdefE", @function
"_ZN6member17hABCDEF012345678917h0123456789abcdefE":
    mov $3, %eax
    ret
.size "_ZN6member17hABCDEF012345678917h0123456789abcdefE", .-"_ZN6member17hABCDEF012345678917h0123456789abcdefE"

/* ordinary legacy compiler hash */
.balign 16
.globl "_ZN5lower6actual17h0123456789abcdefE"
.type "_ZN5lower6actual17h0123456789abcdefE", @function
"_ZN5lower6actual17h0123456789abcdefE":
    mov $4, %eax
    ret
.size "_ZN5lower6actual17h0123456789abcdefE", .-"_ZN5lower6actual17h0123456789abcdefE"

/* true Itanium function with uppercase identifier */
.balign 16
.globl "_ZN8cpp_demo17hABCDEF0123456789Ev"
.type "_ZN8cpp_demo17hABCDEF0123456789Ev", @function
"_ZN8cpp_demo17hABCDEF0123456789Ev":
    mov $5, %eax
    ret
.size "_ZN8cpp_demo17hABCDEF0123456789Ev", .-"_ZN8cpp_demo17hABCDEF0123456789Ev"

/* ordinary Itanium function */
.balign 16
.globl "_ZN6cpp_ok2goEv"
.type "_ZN6cpp_ok2goEv", @function
"_ZN6cpp_ok2goEv":
    mov $6, %eax
    ret
.size "_ZN6cpp_ok2goEv", .-"_ZN6cpp_ok2goEv"

/* uppercase member after valid LLVM renaming */
.balign 16
.globl "_ZN4llvm17hABCDEF0123456789E.llvm.9D1C9369"
.type "_ZN4llvm17hABCDEF0123456789E.llvm.9D1C9369", @function
"_ZN4llvm17hABCDEF0123456789E.llvm.9D1C9369":
    mov $7, %eax
    ret
.size "_ZN4llvm17hABCDEF0123456789E.llvm.9D1C9369", .-"_ZN4llvm17hABCDEF0123456789E.llvm.9D1C9369"

/* ordinary Rust v0 function */
.balign 16
.globl "_RNvC6simple4main"
.type "_RNvC6simple4main", @function
"_RNvC6simple4main":
    mov $8, %eax
    ret
.size "_RNvC6simple4main", .-"_RNvC6simple4main"

/* MSVC name-only demangling */
.balign 16
.globl "?foo@Bar@@QEAAXXZ"
.type "?foo@Bar@@QEAAXXZ", @function
"?foo@Bar@@QEAAXXZ":
    mov $9, %eax
    ret
.size "?foo@Bar@@QEAAXXZ", .-"?foo@Bar@@QEAAXXZ"

/* true lowercase hash in LLVM spelling */
.balign 16
.globl "_ZN9lowerllvm6actual17h0123456789abcdefE.llvm.9D1C9369"
.type "_ZN9lowerllvm6actual17h0123456789abcdefE.llvm.9D1C9369", @function
"_ZN9lowerllvm6actual17h0123456789abcdefE.llvm.9D1C9369":
    mov $10, %eax
    ret
.size "_ZN9lowerllvm6actual17h0123456789abcdefE.llvm.9D1C9369", .-"_ZN9lowerllvm6actual17h0123456789abcdefE.llvm.9D1C9369"

.section .note.GNU-stack,"",@progbits

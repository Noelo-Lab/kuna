.text
.globl caller_integer
.type caller_integer,@function
caller_integer:
    subq $40,%rsp
    movl %ecx,%edx
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_integer,.-caller_integer
.globl caller_stored_integer
.type caller_stored_integer,@function
caller_stored_integer:
    subq $40,%rsp
    movl (%rcx),%edx
    addl $1,%edx
    movl %edx,(%rcx)
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_stored_integer,.-caller_stored_integer
.globl render_value
.type render_value,@function
render_value:
    movq %rdx,%rax
    ret
.size render_value,.-render_value
.section .rodata
format_integer: .asciz "%i"
.section .note.GNU-stack,"",@progbits
.text
.globl caller_computed_integer
.type caller_computed_integer,@function
caller_computed_integer:
    subq $40,%rsp
    movl (%rcx),%edx
    addl $1,%edx
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_computed_integer,.-caller_computed_integer

.macro stored name,format
.globl \name
.type \name,@function
\name:
    subq $40,%rsp
    movl (%rcx),%edx
    addl $1,%edx
    movl %edx,(%rcx)
    leaq \format(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size \name,.-\name
.endm
stored caller_stored_unused,format_plain
stored caller_stored_malformed,format_malformed
stored caller_stored_float_format,format_double
stored caller_stored_writable,format_writable

.globl caller_stored_extra
.type caller_stored_extra,@function
caller_stored_extra:
    subq $40,%rsp
    movl (%rcx),%edx
    addl $1,%edx
    movl %edx,(%rcx)
    movl 4(%rcx),%r8d
    addl $2,%r8d
    movl %r8d,4(%rcx)
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_stored_extra,.-caller_stored_extra

.globl caller_stored_dynamic
.type caller_stored_dynamic,@function
caller_stored_dynamic:
    subq $40,%rsp
    movq %rdx,%r10
    movl (%r10),%edx
    addl $1,%edx
    movl %edx,(%r10)
    call render_value
    addq $40,%rsp
    ret
.size caller_stored_dynamic,.-caller_stored_dynamic

.section .rodata
format_plain: .asciz "plain"
format_malformed: .asciz "%q"
format_double: .asciz "%f"
.data
format_writable: .asciz "%i"
.text
.globl caller_undeclared_stored
.type caller_undeclared_stored,@function
caller_undeclared_stored:
    subq $40,%rsp
    movl %edx,(%rcx)
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_undeclared_stored,.-caller_undeclared_stored

.globl caller_clobbered
.type caller_clobbered,@function
caller_clobbered:
    subq $40,%rsp
    call clobber
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_clobbered,.-caller_clobbered
.globl clobber
.type clobber,@function
clobber:
    movl $42,%edx
    ret
.size clobber,.-clobber
.globl caller_narrow_stored
.type caller_narrow_stored,@function
caller_narrow_stored:
    subq $40,%rsp
    movb %dl,(%rcx)
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_narrow_stored,.-caller_narrow_stored

.globl caller_scanf_control
.type caller_scanf_control,@function
caller_scanf_control:
    subq $40,%rsp
    movq %rcx,%rdx
    leaq format_integer(%rip),%rcx
    call scanflike
    addq $40,%rsp
    ret
.size caller_scanf_control,.-caller_scanf_control
.globl scanflike
.type scanflike,@function
scanflike:
    movq %rdx,%rax
    ret
.size scanflike,.-scanflike

.globl caller_partial_unknown
.type caller_partial_unknown,@function
caller_partial_unknown:
    subq $40,%rsp
    cmpl $0,(%rcx)
    je .Lpartial_call
    movl (%rcx),%edx
    addl $1,%edx
    movl %edx,(%rcx)
.Lpartial_call:
    leaq format_integer(%rip),%rcx
    call render_value
    addq $40,%rsp
    ret
.size caller_partial_unknown,.-caller_partial_unknown

.globl caller_moved_narrow
.type caller_moved_narrow,@function
caller_moved_narrow:
    subq $40,%rsp
    movb %dl,(%rcx)
    movq %rdx,%r8
    movl $0,%ecx
    leaq format_integer(%rip),%rdx
    call render_with_flag
    addq $40,%rsp
    ret
.size caller_moved_narrow,.-caller_moved_narrow
.globl render_with_flag
.type render_with_flag,@function
render_with_flag:
    movq %r8,%rax
    ret
.size render_with_flag,.-render_with_flag

.section .rodata
format_two: .asciz "%i %i"
.text
.macro gap name,clobbered=0
.globl \name
.type \name,@function
\name:
    subq $40,%rsp
    .if \clobbered
    call clobber
    .endif
    movl (%rcx),%r8d
    addl $1,%r8d
    movl %r8d,(%rcx)
    leaq format_two(%rip),%rcx
    call render_two
    addq $40,%rsp
    ret
.size \name,.-\name
.endm
gap caller_gap
gap caller_gap_clobbered,1

.globl caller_stored_two
.type caller_stored_two,@function
caller_stored_two:
    subq $40,%rsp
    movl (%rcx),%edx
    addl $1,%edx
    movl %edx,(%rcx)
    movl 4(%rcx),%r8d
    addl $2,%r8d
    movl %r8d,4(%rcx)
    leaq format_two(%rip),%rcx
    call render_two
    addq $40,%rsp
    ret
.size caller_stored_two,.-caller_stored_two
.globl render_two
.type render_two,@function
render_two:
    movl %r8d,%eax
    shlq $32,%rax
    movl %edx,%edx
    orq %rdx,%rax
    ret
.size render_two,.-render_two

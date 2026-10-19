.text
.globl sample_fistp16
.type sample_fistp16,@function
sample_fistp16:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movw %si,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistps 8(%rsp)
 fldcw 0(%rsp)
 movswl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_fistp16,.-sample_fistp16

.text
.globl sample_fistp32
.type sample_fistp32,@function
sample_fistp32:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movw %si,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistpl 8(%rsp)
 fldcw 0(%rsp)
 movl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_fistp32,.-sample_fistp32

.text
.globl sample_fistp64
.type sample_fistp64,@function
sample_fistp64:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movw %si,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistpll 8(%rsp)
 fldcw 0(%rsp)
 movq 8(%rsp),%rax
 add $24,%rsp
 ret
.size sample_fistp64,.-sample_fistp64

.text
.globl sample_fist16
.type sample_fist16,@function
sample_fist16:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movw %si,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fists 8(%rsp)
 fstpt (%rdx)
 fldcw 0(%rsp)
 movswl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_fist16,.-sample_fist16

.text
.globl sample_fist32
.type sample_fist32,@function
sample_fist32:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movw %si,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistl 8(%rsp)
 fstpt (%rdx)
 fldcw 0(%rsp)
 movl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_fist32,.-sample_fist32

.section .note.GNU-stack,"",@progbits

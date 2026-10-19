.text
.globl sample_cast16
.type sample_cast16,@function
sample_cast16:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movzwl 0(%rsp),%eax
 orb $12,%ah
 movw %ax,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistps 8(%rsp)
 fldcw 0(%rsp)
 movswl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_cast16,.-sample_cast16

.text
.globl sample_cast32
.type sample_cast32,@function
sample_cast32:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movzwl 0(%rsp),%eax
 orl $3072,%eax
 movw %ax,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistpl 8(%rsp)
 fldcw 0(%rsp)
 movl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_cast32,.-sample_cast32

.text
.globl sample_cast64
.type sample_cast64,@function
sample_cast64:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movzwl 0(%rsp),%eax
 andl $62463,%eax
 orw %si,%ax
 orl $3072,%eax
 movw %ax,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistpll 8(%rsp)
 fldcw 0(%rsp)
 movq 8(%rsp),%rax
 add $24,%rsp
 ret
.size sample_cast64,.-sample_cast64

.text
.globl sample_floor32
.type sample_floor32,@function
sample_floor32:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movzwl 0(%rsp),%eax
 andb $243,%ah
 orb $4,%ah
 movw %ax,2(%rsp)
 fldcw 2(%rsp)
 fldt (%rdi)
 fistpl 8(%rsp)
 fldcw 0(%rsp)
 movl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_floor32,.-sample_floor32

.text
.globl sample_restored32
.type sample_restored32,@function
sample_restored32:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movzwl 0(%rsp),%eax
 orl $3072,%eax
 movw %ax,2(%rsp)
 fldcw 2(%rsp)
 fxrstor (%rsi)
 fldt (%rdi)
 fistpl 8(%rsp)
 fldcw 0(%rsp)
 movl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_restored32,.-sample_restored32

.text
.globl sample_xrestored32
.type sample_xrestored32,@function
sample_xrestored32:
 sub $24,%rsp
 fnstcw 0(%rsp)
 movzwl 0(%rsp),%eax
 orl $3072,%eax
 movw %ax,2(%rsp)
 fldcw 2(%rsp)
 movl $1,%eax
 xorl %edx,%edx
 xrstor (%rsi)
 fldt (%rdi)
 fistpl 8(%rsp)
 fldcw 0(%rsp)
 movl 8(%rsp),%eax
 add $24,%rsp
 ret
.size sample_xrestored32,.-sample_xrestored32

.section .note.GNU-stack,"",@progbits

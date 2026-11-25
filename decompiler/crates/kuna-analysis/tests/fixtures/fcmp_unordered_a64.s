/* Every AArch64 condition read after fcmp (tests/stages/kuna-fcmp-unordered.xml).
   clang --target=aarch64-linux-gnu -c fcmp_unordered_a64.s -o fcmp_unordered_a64.o */
.text
.p2align 2
.macro CSET name, cond
.global \name
.type \name, %function
\name:
    fcmp d0, d1
    cset w0, \cond
    ret
.size \name, .-\name
.endm
CSET a64_eq, eq
CSET a64_ne, ne
CSET a64_mi, mi
CSET a64_pl, pl
CSET a64_ge, ge
CSET a64_lt, lt
CSET a64_gt, gt
CSET a64_le, le
CSET a64_hi, hi
CSET a64_ls, ls
CSET a64_hs, hs
CSET a64_lo, lo

.global a64_gt_zero
.type a64_gt_zero, %function
a64_gt_zero:
    fcmp s0, #0.0
    cset w0, gt
    ret
.size a64_gt_zero, .-a64_gt_zero

.global a64_le_branch
.type a64_le_branch, %function
a64_le_branch:
    fcmp d0, d1
    b.le 1f
    mov w0, #3
    ret
1:  mov w0, #5
    ret
.size a64_le_branch, .-a64_le_branch

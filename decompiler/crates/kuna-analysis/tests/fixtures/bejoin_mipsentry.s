# option bejoin callers: a proven pair kept whole only where every path writes both words (GH-904).
.set noreorder
.set noat
.text
.macro FN name
  .globl \name
  .type \name,@function
\name:
.endm

# u64 sum_both(u32 hi, u32 lo, int c, u32 k): both paths compute $3, which also feeds $2.
FN sum_both
  beqz $6,1f
  nop
  addu $3,$5,$7
  sltu $2,$3,$5
  jr $31
  addu $2,$2,$4
1:
  addiu $3,$5,1
  sltiu $2,$3,1
  jr $31
  addu $2,$2,$4
.size sum_both,.-sum_both

# Like sum_both, but when c is zero $3 reaches the RETURN as it came in.
FN sum_or_entry
  beqz $6,1f
  move $2,$4
  addu $3,$5,$7
  sltu $2,$3,$5
  addu $2,$2,$4
1:
  jr $31
  nop
.size sum_or_entry,.-sum_or_entry

.macro CALLER name,callee
  FN \name
  addiu $29,$29,-24
  sw $31,20($29)
  jal \callee
  nop
  lui $8,%hi(sink)
  addiu $8,$8,%lo(sink)
  sw $2,0($8)
  sw $3,4($8)
  lw $31,20($29)
  jr $31
  addiu $29,$29,24
  .size \name,.-\name
.endm

CALLER c_sum_both,sum_both
CALLER c_sum_or_entry,sum_or_entry

FN __start
  jal c_sum_both
  nop
  jal c_sum_or_entry
  nop
1:
  b 1b
  nop
.size __start,.-__start

.data
.globl sink
.type sink,@object
sink: .word 0,0
.size sink,8

@ (kuna) ARM32 shared object whose exported `answer` shares its address with a
@ local ARM veneer name (`__answer_from_arm`, GNU ld's spelling for its
@ ARM-to-Thumb glue); the veneer branches to the Thumb body, and `caller` reaches
@ `answer` through the PLT, so the name also labels a PLT stub. Built with
@   clang --target=armv7a-linux-gnueabihf -c armveneeralias_le32.s -o a.o
@   ld.lld -shared -o armveneeralias_le32 a.o
@ See decompiler/crates/kuna-analysis/tests/fixtures/README.md.
.syntax unified
.text
.arm
.type __answer_from_arm,%function
__answer_from_arm:
.global answer
.type answer,%function
answer:
  ldr ip, [pc, #4]
  add ip, ip, pc
  bx ip
  .word (real_answer + 1) - .
.thumb
.type real_answer,%function
.thumb_func
real_answer:
  movs r0, #7
  bx lr
.global caller
.type caller,%function
.thumb_func
caller:
  push {lr}
  bl answer(PLT)
  pop {pc}
.section .note.GNU-stack,"",%progbits

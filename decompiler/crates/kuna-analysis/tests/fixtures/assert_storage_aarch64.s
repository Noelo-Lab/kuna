// A pointer-returning callee and a caller that loads through its result, for the
// storage names `--assert` accepts in `return` and `param` directives.
//
//   struct sample { unsigned char prefix[24]; unsigned int word; };
//   struct sample *pick(struct sample *p, unsigned int v) { p->word = v; return p; }
//   unsigned int read_back(struct sample *p, unsigned int v) { return pick(p, v)->word; }

 .text

 .globl pick
 .type pick,%function
pick:
 str w1, [x0, #24]
 ret
 .size pick, .-pick

 .globl read_back
 .type read_back,%function
read_back:
 stp x29, x30, [sp, #-16]!
 mov x29, sp
 bl pick
 ldr w0, [x0, #24]
 ldp x29, x30, [sp], #16
 ret
 .size read_back, .-read_back

// (kuna) AArch64 shared object whose `timer_wrapper` calls its own exports
// through the PLT: `bl timer_helper@plt` forwards the caller's x0, x1, w2 and
// w3 untouched, and `b send_helper@plt` tail-calls after reloading x0/x1.
// GH-1342's standalone reproducer, verbatim. Built with
//   clang --target=aarch64-linux-gnu -c pltlocal_aarch64.s -o p.o
//   ld.lld -shared -o pltlocal_aarch64.so p.o
// See pltlocal_aarch64.provenance.
.text
.p2align 2
.global timer_wrapper
.type timer_wrapper,%function
timer_wrapper:
 ldrb w6,[x1,#5]
 tst x6,#7
 b.eq 1f
 ret
1:
 stp x29,x30,[sp,#-32]!
 mov x29,sp
 stp x19,x20,[sp,#16]
 mov x19,x1
 mov x20,x0
 bl timer_helper
 mov x1,x19
 mov x0,x20
 ldp x19,x20,[sp,#16]
 ldp x29,x30,[sp],#32
 b send_helper
.size timer_wrapper,.-timer_wrapper
.global timer_helper
.type timer_helper,%function
timer_helper:
 str w2,[x0,#4]
 str w3,[x0,#8]
 str x1,[x0,#16]
 ret
.size timer_helper,.-timer_helper
.global send_helper
.type send_helper,%function
send_helper:
 str x1,[x0,#24]
 ret
.size send_helper,.-send_helper

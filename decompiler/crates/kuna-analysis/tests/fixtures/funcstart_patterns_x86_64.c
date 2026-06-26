#include <stdint.h>
static int sink;
extern long ext(long);

// Force "push rbx; sub rsp, C" at the entry: save the (callee-saved) rbx FIRST by
// reading the first arg into a callee-saved reg before any other setup, plus a
// stack array so a frame (sub rsp) is allocated. -O2.
__attribute__((noinline, used))
static long widget(long a) {
    register long keep __asm__("rbx") = a;          // rbx live from entry
    volatile long buf[3];
    __asm__ volatile("" : "+r"(keep));               // pin rbx at entry
    buf[0] = ext(keep);
    buf[1] = ext(keep + 1);
    buf[2] = ext(keep + 2);
    __asm__ volatile("" :: "r"(keep));
    return buf[0] + buf[1] + buf[2] + keep;
}

int main(int argc, char **argv) {
    long r = widget(argc + argv[0][0]);
    sink = (int)r;
    return (int)r;
}
long ext(long v) { return v * 2 + 1; }

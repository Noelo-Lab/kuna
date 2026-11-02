#include <stdint.h>
typedef struct Pair { int64_t first, second; } Pair;
uint64_t calls;
__attribute__((noinline,noclone)) Pair make_pair(int64_t a, int64_t b) { ++calls; return (Pair){a+b,a-b}; }
__attribute__((noinline,noclone)) int64_t first(Pair (*fn)(int64_t,int64_t), int64_t a, int64_t b) { return fn(a,b).first; }
__attribute__((noinline,noclone)) int64_t second(Pair (*fn)(int64_t,int64_t), int64_t a, int64_t b) { return fn(a,b).second; }
__attribute__((noinline,noclone)) int64_t both(Pair (*fn)(int64_t,int64_t), int64_t a, int64_t b) { Pair x=fn(a,b); return x.first^x.second; }
__attribute__((noinline,noclone)) Pair forward(Pair (*fn)(int64_t,int64_t), int64_t a, int64_t b) { return fn(a,b); }
__attribute__((noinline,noclone)) int32_t narrow_second(Pair (*fn)(int64_t,int64_t), int64_t a, int64_t b) { return (int32_t)fn(a,b).second; }
__attribute__((noinline,noclone)) int64_t scalar(int64_t (*fn)(int64_t,int64_t), int64_t a, int64_t b) { return fn(a,b); }
__attribute__((noinline,noclone)) void invoke_void(void (*fn)(int64_t,int64_t), int64_t a, int64_t b) { fn(a,b); }
int main(void) { return second(make_pair,13,7)!=6; }

/* stackguard_global_aarch64 (tests/stages/kuna-stackguard-global.xml):
 *   zig cc -target aarch64-linux-gnu -O2 -fstack-protector-all stackguard_global_aarch64.c
 * The canary is the global __stack_chk_guard, read through its .got slot. */
__attribute__((noinline)) int fill(char *p, int n) { for (int i=0;i<n;i++) p[i]=i; return p[n/2]; }
int guarded(int n) { char buf[64]; return fill(buf, n & 63) + 1; }
int main(int c,char**v){return guarded(c);}

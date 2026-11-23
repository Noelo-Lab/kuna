/*
 * passthrough_loopfwd_x86_64 -- a function that hands its own rdi to a callee
 * inside a loop keeps the argument.
 *
 *     gcc -O2 -fno-inline -fno-pie -no-pie -fcf-protection=none \
 *         passthrough_loopfwd_x86_64.c -o passthrough_loopfwd_x86_64
 *
 * bzip2 compress.c `add_pair_to_block(s)` inside `generateMTFValues`' loops,
 * reduced. gcc knows `add_pair` leaves rdi alone, so `walk` keeps `s` in rdi
 * across the call and never reloads it. At the call, heritage joins the
 * incoming rdi with the rdi the loop carries back past the call, and the
 * call printed with no argument while `add_pair` was recovered taking one.
 * `once` is the same call outside a loop, which already kept its argument.
 */
typedef struct { int n, a, b, c; long x, y; } S;
__attribute__((noinline)) static void add_pair(S *s) { s->x += s->y; s->c++; }
__attribute__((noinline)) long once(S *s) { if (s->a & 1) add_pair(s); return s->x + s->b; }
__attribute__((noinline)) long walk(S *s) {
  long t = 0;
  while (s->n-- > 0) {
    if (s->a & 1) add_pair(s);
    else t += s->b;
    s->b++;
  }
  return t + s->x;
}
int main(void) { static S s = {3, 1, 0, 0, 1, 2}; return (int)(walk(&s) + once(&s)); }

/*
 * passthrough_widthfwd_x86_64 -- a forwarder that calls its target keeps the
 * width it reads a register at.
 *
 *     gcc -O2 -fno-inline -fno-pie -no-pie -fcf-protection=none \
 *         passthrough_widthfwd_x86_64.c -o passthrough_widthfwd_x86_64
 *
 * openssh's channel_lookup/channel_by_id shape. `by_id` passes `id` to a
 * variadic logger with `push %rsi`, which reads all of rsi, so its recovered
 * `id` is 64 bits. `lookup` and `send_open` read `esi` themselves and forward
 * by a call. `passthrough` follows the call from `use_send` into `send_open`,
 * `lookup` and `by_id` to find `rdi`, but `esi` is already read by each
 * forwarder, so the deeper 64-bit read must not widen it: `send_open` and
 * `use_send` take `int id`, not `unsigned long`.
 */
#include <stdarg.h>
#include <stdio.h>
struct ch { int type; int id; };
struct ssh { struct ch **chans; unsigned n; };
__attribute__((noinline)) void logit(const char *file, const char *func, int line, int a, int b, int c, const char *fmt, ...) {
  va_list ap; va_start(ap, fmt); vfprintf(stderr, fmt, ap); va_end(ap); (void)file; (void)func; (void)line; (void)a; (void)b; (void)c;
}
__attribute__((noinline)) struct ch *by_id(struct ssh *s, int id) {
  if (id < 0 || (unsigned)id >= s->n) { logit("c.c","by_id",1,0,3,0,"%d: bad id", id); return 0; }
  return s->chans[id];
}
__attribute__((noinline)) struct ch *lookup(struct ssh *s, int id) {
  struct ch *c = by_id(s, id);
  if (c && c->type > 22) { logit("c.c","lookup",2,0,3,0,"Non-public %d %d", id, c->type); return 0;}
  return c;
}
__attribute__((noinline)) int send_open(struct ssh *s, int id) {
  struct ch *c = lookup(s, id);
  if (!c) { logit("c.c","send_open",3,0,3,0,"%d: bad id", id); return 0; }
  return c->id + 7;
}
__attribute__((noinline)) int use_send(struct ssh *s, int id) {
  return send_open(s, id) * 3;
}
int main(int argc, char **argv) {
  static struct ch c0 = {1, 5}; static struct ch *arr[1] = {&c0}; static struct ssh s = {arr, 1};
  return use_send(&s, argc - 1) + (argv == 0);
}

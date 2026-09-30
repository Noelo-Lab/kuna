/* fieldtype: record fields the program holds as pointers, whose first access
 * at -O0 is an integer add or compare; and two integer fields some access
 * carries as a pointer that must stay integers.
 *
 *   gcc   -O0 -fno-stack-protector -o fieldtype_gcc_O0_x86_64   fieldtype_x86_64.c
 *   clang -O0 -fno-stack-protector -o fieldtype_clang_O0_x86_64 fieldtype_x86_64.c
 *   gcc   -O2 -fno-stack-protector -o fieldtype_gcc_O2_x86_64   fieldtype_x86_64.c
 *   clang -O2 -fno-stack-protector -o fieldtype_clang_O2_x86_64 fieldtype_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

struct buf { char *data; long used; int left; int flags; };

/* sort's fillbuf: the buffer is added to before it is handed to memmove. */
__attribute__((noinline)) long buf_shift(struct buf *b) {
  if (b->used != b->left) {
    memmove(b->data, b->data + (b->used - b->left), b->left);
    b->used = b->left;
    b->flags |= 1;
  }
  char *end = b->data + b->used;
  long n = 0;
  for (char *p = b->data; p < end; p++)
    n = n * 31 + *p;
  return n;
}

static char unknown_ctx[] = "?";
struct ent { char *name; int size; char *ctx; int flags; };

/* ls's free_ent: a sentinel compare, then strlen and free. */
__attribute__((noinline)) long ent_release(struct ent *e) {
  if (!e->name)
    return -2;
  if (e->ctx == unknown_ctx)
    return -1;
  long n = (long)strlen(e->ctx) + e->size;
  if (e->flags)
    free(e->ctx);
  return n;
}

struct ops { int state; long (*fn)(long); long arg; };

/* A function-pointer field compared with zero before it is called. */
__attribute__((noinline)) long ops_run(struct ops *o) {
  if (o->fn == 0)
    return o->state;
  return o->fn(o->arg) + o->state;
}

struct range { long *lo; long *hi; long *end_lo; long *end_hi; int nlo; int nhi; };

/* sort's mergelines_node: two fields known only by being compared with the
 * pointers walked down to them, and pointer differences. */
__attribute__((noinline)) long range_walk(struct range *r) {
  long sum = 0;
  while (r->lo != r->end_lo && r->hi != r->end_hi) {
    sum = sum * 7 + (*r->lo - *r->hi);
    r->lo++;
    r->hi--;
  }
  r->nlo = r->end_lo - r->lo;
  r->nhi = r->hi - r->end_hi;
  return sum;
}

struct cell { long tag; union { double d; char *s; long l; } u; };

/* A field read as a double and as a pointer stays raw bytes. */
__attribute__((noinline)) double cell_val(struct cell *c) {
  if (c->tag == 1)
    return (double)strlen(c->u.s);
  if (c->tag == 2)
    return (double)c->u.l;
  return c->u.d;
}

struct pctx { unsigned long num; char *str; unsigned int ino; unsigned long blk; };

/* e2fsck's expand_percent_expression: the number is compared first, and at
 * -O2 clang sinks every case into one fprintf, so the number and the string
 * reach it through one register. The number stays a number. */
__attribute__((noinline)) void pctx_print(FILE *f, char ch, int width, struct pctx *c) {
  if (c->num == 0)
    fputs("(none) ", f);
  switch (ch) {
  case 'n': fprintf(f, "%*lu", width, c->num); break;
  case 'b': fprintf(f, "%*lu", width, c->blk); break;
  case 'i': fprintf(f, "%*u", width, c->ino); break;
  case 's': fprintf(f, "%*s", width, c->str ? c->str : "NULL"); break;
  case 'x': fprintf(f, "%*lx", width, c->num); break;
  default: fputc(ch, f);
  }
}

struct slot { int kind; long idx; char *name; };

/* An index a table is read at, then handed to write as its buffer. The index
 * stays a number. */
__attribute__((noinline)) long slot_post(const long *tab, struct slot *s) {
  long v = tab[s->idx] + s->kind;
  return v + write(-1, (void *)s->idx, 0);
}

static long twice(long x) { return 2 * x + 1; }

int main(void) {
  char text[] = "hello, fieldtype world";
  struct buf b = { text, 10, 4, 0 };
  long s1 = buf_shift(&b);
  printf("buf %ld %ld %d %.*s\n", s1, b.used, b.flags, (int)b.used, b.data);

  char *heap = malloc(8);
  strcpy(heap, "context");
  struct ent e1 = { "a", 3, heap, 1 }, e2 = { "b", 5, unknown_ctx, 0 }, e3 = { "c", 7, "static", 0 }, e4 = { 0, 1, "x", 0 };
  printf("ent %ld %ld %ld %ld\n", ent_release(&e1), ent_release(&e2), ent_release(&e3), ent_release(&e4));

  struct ops o1 = { 40, twice, 5 }, o2 = { -3, 0, 9 };
  printf("ops %ld %ld\n", ops_run(&o1), ops_run(&o2));

  long lo[] = { 9, 8, 7, 6, 5 }, hi[] = { 1, 2, 3, 4, 5 };
  struct range r = { lo, hi + 4, lo + 5, hi, 0, 0 };
  long w = range_walk(&r);
  printf("range %ld %d %d\n", w, r.nlo, r.nhi);

  struct cell c1 = { 1, { .s = "four" } }, c2 = { 2, { .l = -12 } }, c3 = { 3, { .d = 2.5 } };
  printf("cell %g %g %g\n", cell_val(&c1), cell_val(&c2), cell_val(&c3));

  struct pctx pc = { 42, "path", 7, 99 };
  for (const char *p = "nbisxz"; *p; p++) {
    pctx_print(stdout, *p, 3, &pc);
    putchar('|');
  }
  pc.str = 0;
  pc.num = 0;
  pctx_print(stdout, 's', 1, &pc);
  pctx_print(stdout, 'n', 1, &pc);
  putchar('\n');

  long tab[] = { 3, 1, 4, 1, 5, 9, 2, 6 };
  struct slot sl = { 2, 5, "five" };
  printf("slot %ld\n", slot_post(tab, &sl));
  return 0;
}

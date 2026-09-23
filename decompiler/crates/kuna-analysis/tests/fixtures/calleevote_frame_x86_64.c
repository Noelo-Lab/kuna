/* calleevote fixture: a caller's frame record and a declared pointer (x86-64).
 *
 * `main` keeps a `struct cfg` in its frame whose first member is a `char *`,
 * so it passes `&cfg` to `add` and `drop` as a `char **`. `add` stores a new
 * node through the record, and the node's word stores would print one
 * character at a time through a `char *`; `drop` reads the member after the
 * first one, past the `char *` the caller typed. Neither takes `char **`.
 * `advance` is handed `&cursor`, a real `char **`, and reads only that one
 * pointer, so it takes it. `mkpipe` reads one field of the `int *` it hands to
 * `pipe`, which keeps `int *`.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o calleevote_frame_x86_64 calleevote_frame_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

struct node { struct node *next; unsigned int kind; unsigned int flags; unsigned long len; };
struct cfg { const char *name; char dir[200]; struct node *head; };

__attribute__((noinline)) void add(struct cfg *cfg, const char *path) {
  struct node *s;
  if (strcmp(path, "builtin") == 0) {
    s = malloc(sizeof *s);
    if (!s)
      return;
    s->kind = 0x01020304;
    s->flags = 0x05060708;
    s->len = 0x1122334455667788UL;
  } else {
    size_t n = strlen(path);
    s = malloc(sizeof *s + n + 1);
    if (!s)
      return;
    s->kind = 0xa0b0c0d0;
    s->flags = 7;
    s->len = n;
    memcpy(s + 1, path, n + 1);
  }
  s->next = cfg->head;
  cfg->head = s;
}

__attribute__((noinline)) unsigned long drop(struct cfg *cfg) {
  unsigned long t = 0;
  struct node *s = cfg->head;
  while (s) {
    struct node *n = s->next;
    t += s->len;
    free(s);
    s = n;
  }
  return t;
}

__attribute__((noinline)) int advance(char **cursor) {
  char *p = *cursor;
  while (*p == ' ')
    p++;
  *cursor = p;
  return *p;
}

__attribute__((noinline)) int mkpipe(int *p) {
  int r = pipe(p);
  return r + p[1];
}

int main(int c, char **v) {
  struct cfg cfg;
  int fds[2];
  char *cursor = v[0];
  memset(&cfg, 0, sizeof cfg);
  cfg.name = v[0];
  add(&cfg, "builtin");
  add(&cfg, c > 1 ? v[1] : "x");
  printf("%s %lu %d %d\n", cfg.name, drop(&cfg), advance(&cursor), mkpipe(fds));
  return 0;
}

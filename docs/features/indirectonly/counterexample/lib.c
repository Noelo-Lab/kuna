#include <stdio.h>
#include <stdlib.h>
struct arr { void **p; int n; };
void a_init(struct arr *a, int cap){ a->p = calloc(cap,8); a->n = 0; }
void a_push(struct arr *a, void *v){ a->p[a->n++] = v; }
void a_done(struct arr *a){ printf("a_done sees n=%d\n", a->n); free(a->p); }

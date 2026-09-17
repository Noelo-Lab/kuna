#include <stdio.h>
struct arr { void **p; int n; };
void a_init(struct arr *, int);
void a_push(struct arr *, void *);
void a_done(struct arr *);
void sink(void *);

/* transcription of kuna's emitted walk(): v1 = stack-0x38 (the void** p),
   v2 = stack-0x30 (the int n), 8 bytes apart -> exactly struct arr. */
void walk_off(unsigned long *a0)
{
  struct arr s;
  long *v1 = (long *)&s.p;
  int v2;                         /* // r14d  : a register local */
  int *v3 = &s.n;                 /* // stack - 0x30 */
  a_init((struct arr *)v1,3);
  a_push((struct arr *)v1,(void *)*a0);
  a_push((struct arr *)v1,(void *)a0[1]);
  a_push((struct arr *)v1,(void *)a0[2]);
  v2 = *v3 + -1;
  if (0 <= v2) {
    do {
      sink(*(void **)(*v1 + (long)v2 * 8));
      v2 -= 1;
    } while (v2 != -1);
  }
  a_done((struct arr *)v1);
}

void walk_on(unsigned long *a0)
{
  struct arr s;
  long *v1 = (long *)&s.p;
  int *v2 = &s.n;                 /* // stack - 0x30  (the merged variable) */
  a_init((struct arr *)v1,3);
  a_push((struct arr *)v1,(void *)*a0);
  a_push((struct arr *)v1,(void *)a0[1]);
  a_push((struct arr *)v1,(void *)a0[2]);
  *v2 -= 1;
  if (0 <= *v2) {
    do {
      sink(*(void **)(*v1 + (long)*v2 * 8));
      *v2 -= 1;
    } while (*v2 != -1);
  }
  a_done((struct arr *)v1);
}

int main(void)
{
  unsigned long it[3] = {1,2,3};
  printf("-- walk_off (option off / origin-main output) --\n"); walk_off(it);
  printf("-- walk_on  (option on  / default output)      --\n"); walk_on(it);
  return 0;
}

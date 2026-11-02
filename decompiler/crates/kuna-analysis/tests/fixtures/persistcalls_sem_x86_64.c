/* GH-1034 `persistcalls` correctness fixture: functions whose sparse call
   guards once changed what the C means. Rebuild:
     gcc -O1 -fno-pie -no-pie -fno-inline -fcf-protection=none -o persistcalls_sem_x86_64 persistcalls_sem_x86_64.c
   Run with no arguments it prints 0 0 21 42. */
#include <stdio.h>
union U { long l; int i[2]; short s[4]; };
union U u0, u1;
int arr[8];
int bumps;
__attribute__((noinline)) void f1(void) { __asm__ volatile("" ::: "memory"); u0.s[0] += 1; arr[2] += 1; bumps++; }
__attribute__((noinline)) void f2(void) { __asm__ volatile("" ::: "memory"); }
__attribute__((noinline)) int fi(int x) { __asm__ volatile("" : "+r"(x) : : "memory"); return x + 20; }
/* A later pass reads u0 at four bytes through p: the two-byte reads either
   side of f1 must stay two reads. */
__attribute__((noinline, optimize("O0"))) int revisit(void) {
  int *p = &u0.i[0];
  short x = u0.s[0];
  f1();
  short y = u0.s[0];
  f2();
  *p = *p * 3;
  return x == y;
}
/* The value stored to arr[2] after the loop is computed before it: the store
   must stay after the calls in the loop. */
__attribute__((noinline)) int loopstore(int a) {
  int *p = &arr[2];
  int t = fi(*p);
  for (int i = 0; i < a; i++) f1();
  *p = t;
  return u1.i[0];
}
int main(void) {
  int r = revisit();
  u1.i[0] = 0;
  int s = loopstore(3);
  printf("%d %d %d %d\n", r, s, arr[2], bumps * 0 + 42);
  return 0;
}

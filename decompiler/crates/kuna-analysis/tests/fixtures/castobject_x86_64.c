/* castobject: a frame object whose address fills a declared int * parameter. */
#define _GNU_SOURCE
#include <grp.h>
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>

/* WIFEXITED / WEXITSTATUS: bit tests and a shift. */
__attribute__((noinline)) int exit_code(int pid) {
  int status;
  if (waitpid(pid, &status, 0) < 0)
    return -1;
  if (WIFEXITED(status))
    return WEXITSTATUS(status);
  return -2;
}

/* The status compared and divided signed. */
__attribute__((noinline)) long status_order(int pid) {
  int status;
  if (waitpid(pid, &status, 0) < 0)
    return -1;
  if (status < 256)
    return status;
  return status / 256;
}

/* wait(): the same out-parameter, compared signed. */
__attribute__((noinline)) long reaped(void) {
  int status;
  if (wait(&status) < 0)
    return -1;
  return status >= 0x100 ? status >> 8 : -status;
}

/* The address is kept in a pointer before the call: another reading. */
__attribute__((noinline)) int escaped(int pid) {
  int status;
  int *volatile p = &status;
  if (waitpid(pid, p, 0) < 0)
    return -1;
  return (*p >> 8) & 0xff;
}

/* One byte of the status is read on its own: the object is read at two widths. */
__attribute__((noinline)) int two_widths(int pid) {
  int status;
  if (waitpid(pid, &status, 0) < 0)
    return -1;
  return ((unsigned char *)&status)[1] | (status & 0x7f) << 8;
}

/* Wrapping arithmetic on the value: it stays unsigned. */
__attribute__((noinline)) unsigned long plus_one(int pid) {
  int status;
  if (waitpid(pid, &status, 0) < 0)
    return 0;
  return (unsigned int)status + 1u;
}

/* Two calls fill the same object; it is compared signed. */
__attribute__((noinline)) int cancel_state(void) {
  int old;
  pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, &old);
  pthread_setcancelstate(old, &old);
  return old < 1 ? old : 7;
}

/* The objects below start with a parameter's value; waitpid finds no such child
   and getgroups with a size of 0 writes nothing, so each keeps that value. */
#define NOPID 0x7ffffff0

/* Every reader is signed. */
__attribute__((noinline)) int init_signed(unsigned init) {
  int st = init;
  waitpid(NOPID, &st, WNOHANG);
  if (st & 0x7f)
    return (st >> 8) & 0xff;
  return st >> 20;
}

/* A logical shift beside a signed one. */
__attribute__((noinline)) unsigned init_ushr(unsigned init) {
  int st = init;
  waitpid(NOPID, &st, WNOHANG);
  if (st & 0x7f)
    return (st >> 8) & 0xff;
  return ((unsigned)st >> 20) ^ ((st >> 9) & 3);
}

/* An unsigned compare beside signed shifts. */
__attribute__((noinline)) int init_ult(unsigned init) {
  int st = init;
  waitpid(NOPID, &st, WNOHANG);
  if ((st & 0x7f) == 0)
    return (st >> 8) & 0xff;
  if ((unsigned)st < 0x100u)
    return (st >> 4) & 0xf;
  return (st >> 16) & 0xff;
}

/* A zero-extension beside a sign-extension. */
__attribute__((noinline)) long init_zext(unsigned init) {
  int st = init;
  waitpid(NOPID, &st, WNOHANG);
  if (st & 0x7f)
    return (st >> 8) & 0xff;
  return (long)((unsigned long)(unsigned)st ^ (unsigned long)(long)st);
}

/* The unsigned direction: a gid_t also compared signed. */
__attribute__((noinline)) long gid_mixed(int init) {
  gid_t g = init;
  getgroups(0, &g);
  if ((int)g < 0)
    return -1;
  return (long)(g / 3u) + (g >> 4);
}

/* A value stored into the object is also shifted logically on its own. */
__attribute__((noinline)) unsigned stored_value(unsigned k) {
  int st;
  unsigned x = (k >> 1) ^ 0x80000001u;
  st = x;
  if (x & 0x10)
    return x >> 28;
  waitpid(NOPID, &st, WNOHANG);
  return st >> 8;
}

static int child(int code) {
  int pid = fork();
  if (pid == 0) {
    if (code < 0)
      raise(-code);
    _exit(code);
  }
  return pid;
}

int main(void) {
  int a = exit_code(child(0)), b = exit_code(child(7)), c = exit_code(child(255)), d = exit_code(child(-SIGTERM));
  printf("exit_code    %d %d %d %d\n", a, b, c, d);
  long e = status_order(child(0)), f = status_order(child(3)), g = status_order(child(-SIGKILL));
  printf("status_order %ld %ld %ld\n", e, f, g);
  child(5);
  long h = reaped();
  child(-SIGTERM);
  long i = reaped();
  printf("reaped       %ld %ld\n", h, i);
  int j = escaped(child(9)), k = escaped(child(-SIGTERM));
  printf("escaped      %d %d\n", j, k);
  int l = two_widths(child(9)), m = two_widths(child(-SIGTERM));
  printf("two_widths   %d %d\n", l, m);
  unsigned long n = plus_one(child(1)), o = plus_one(child(-SIGTERM));
  printf("plus_one     %lu %lu\n", n, o);
  int q = cancel_state();
  pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, NULL);
  int r = cancel_state();
  printf("cancel_state %d %d\n", q, r);
  unsigned vals[] = {0, 0x20, 0x100, 0x7f00, 0x80000000u, 0x80000001u, 0xfffffffeu, 0xffffffffu, 0x87654321u, 0xffff0000u, 0x800000ffu};
  for (unsigned t = 0; t < sizeof vals / sizeof *vals; t++)
    printf("init %08x %d %x %d %lx %ld %x\n", vals[t], init_signed(vals[t]), init_ushr(vals[t]), init_ult(vals[t]),
           init_zext(vals[t]), gid_mixed((int)vals[t]), stored_value(vals[t]));
  return 0;
}

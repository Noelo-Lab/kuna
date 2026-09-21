/* Each function hands &sa.sa_mask of a struct sigaction local to one sigset_t slot.
   Built gcc -O0 and -O2; decompile-all on both arms shows which slots split the object. */
#include <signal.h>
#include <string.h>
#include <pthread.h>
static void h(int s) { (void)s; }
#define PRE struct sigaction sa; memset(&sa, 0, sizeof sa); sa.sa_handler = h;
#define POST if (sig != 14) sa.sa_flags = 0x10000000; return sigaction(sig, &sa, old);
__attribute__((noinline)) int f_empty(int sig, struct sigaction *old) { PRE sigemptyset(&sa.sa_mask); POST }
__attribute__((noinline)) int f_del(int sig, struct sigaction *old) { PRE sigemptyset(&sa.sa_mask); sigdelset(&sa.sa_mask, 2); POST }
__attribute__((noinline)) int f_ismember(int sig, struct sigaction *old) { PRE if (sigismember(&sa.sa_mask, 2)) sa.sa_handler = SIG_IGN; POST }
__attribute__((noinline)) int f_suspend(int sig, struct sigaction *old) { PRE sigsuspend(&sa.sa_mask); POST }
__attribute__((noinline)) int f_pmask(int sig, struct sigaction *old) { PRE pthread_sigmask(0, &sa.sa_mask, 0); POST }
__attribute__((noinline)) int f_wait(int sig, struct sigaction *old) { PRE int s; sigwait(&sa.sa_mask, &s); if (s) sa.sa_handler = SIG_IGN; POST }
__attribute__((noinline)) int f_pmask_old(int sig, struct sigaction *old) { struct sigaction sa; sigaction(sig, 0, &sa); pthread_sigmask(1, &sa.sa_mask, 0); if (sa.sa_flags & 4) return 1; return sa.sa_flags; }
int main(int c, char **v) { struct sigaction o; (void)v;
  return f_empty(c,&o)+f_del(c,&o)+f_ismember(c,&o)+f_suspend(c,&o)+f_pmask(c,&o)+f_wait(c,&o)+f_pmask_old(c,&o); }

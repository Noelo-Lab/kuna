/* callbacktype negative controls: the two shapes a declared slot does NOT
 * describe (x86-64).
 *
 * `cmp3` takes three arguments and is cast into `qsort`'s two-argument slot,
 * which is legal C and does happen. Its body forwards the third argument to
 * `memcmp` without ever reading it, so nothing in its entry facts says the
 * register is live; only the arity its own recovery found does. Parking the
 * slot's two-parameter declaration on it would drop the third argument at the
 * direct call site below and leave an uninitialized read in its body.
 *
 * `cleanup` is registered twice through ONE materialized address: gcc hoists
 * the `lea` into a callee-saved register and feeds it both to `signal` and to
 * the program's own registrar `set_hook`, which stores it. The image shows one
 * address-taking instruction for two uses, so a count of instructions cannot
 * tell them apart; the second use is what refuses the `void (*)(int)` the
 * `signal` slot would otherwise declare on a routine that takes nothing.
 * (glibc's `atexit` is the same shape and would do as well, but gcc emits a
 * local `atexit` wrapper whose name IS a slot, and the two slots disagreeing
 * refuses it one step earlier.)
 *
 * `quit_either` and `quit_kept` are the same question asked past a carrier.
 * `run_either` registers `quit_either` and then calls it through a phi that may
 * also hold `other`, so the second use reaches an indirect call only through
 * the phi. `keep_and_register` registers `quit_kept` and writes the same
 * register into `KEPT`, a global nothing in that body reads back. Both hold the
 * address in ONE callee-saved register, so the image again shows one `lea`.
 *
 * `wrap_up` and `score1` are declarations longer than the body. `wrap_up`
 * takes nothing and is cast into `signal`'s `void (*)(int)` slot, `score1`
 * takes one pointer and is cast into `qsort`'s two-pointer slot, and `main`
 * ALSO calls each directly: parking the slot closes a list one longer than the
 * body's, and each direct call site then materializes the extra argument out
 * of whatever register was live -- inventing a statement to set it.
 *
 * `start_puts` and `warn_count` are slots that return a value the body does
 * not compute. `start_puts` is a `void` routine cast into `pthread_create`'s
 * `void *(*)(void *)` slot; it ends in a tail-called `puts`, which leaves only
 * the low four of the eight bytes a `void *` return reads. `warn_count` is a
 * `void` routine cast into `glob`'s `int (*)(const char *, int)` errfunc slot
 * that never writes the return register at all. Parking either invents the
 * return expression out of the register's leftover.
 *
 * `noipa` rather than `noinline`: gcc's interprocedural register allocation
 * otherwise hoists an argument register past a call to a function it knows
 * does not clobber it, and kuna's call-site recovery -- which has only the
 * convention to go on -- then reads the argument as the wrong call's.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none -pthread \
 *       -o callbacktype_refused_x86_64 callbacktype_refused_x86_64.c
 */
#include <glob.h>
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NI __attribute__((noipa))

NI int cmp3(const void *a, const void *b, size_t n)
{
    return memcmp(a, b, n);
}

NI void cleanup(void)
{
    fputs("bye\n", stderr);
}

static void (*HOOK)(void);

NI void set_hook(void (*f)(void))
{
    HOOK = f;
}

NI void quit_either(void)
{
    fputs("either\n", stderr);
}

NI void other(void)
{
    fputs("other\n", stderr);
}

NI void run_either(int n)
{
    void (*f)(void) = quit_either;

    signal(SIGTERM, (void (*)(int))quit_either);
    if (n > 2)
        f = other;
    f();
    fputs("ran\n", stderr);
}

NI void quit_kept(void)
{
    fputs("kept\n", stderr);
}

static void (*KEPT)(void);

NI void keep_and_register(void)
{
    signal(SIGHUP, (void (*)(int))quit_kept);
    KEPT = quit_kept;
}

NI void wrap_up(void)
{
    fputs("wrapped\n", stderr);
}

NI int score1(const void *a)
{
    return *(const int *)a * 3;
}

NI void start_puts(void *p)
{
    puts((const char *)p);
}

static int HITS;

NI void warn_count(const char *p, int e)
{
    (void)p;
    HITS += e;
}

int main(int argc, char **argv)
{
    int v[4] = { argc, 3, 2, 1 };
    glob_t g;
    pthread_t t;

    qsort(v, 4, sizeof v[0], (int (*)(const void *, const void *))cmp3);
    signal(SIGINT, (void (*)(int))cleanup);
    set_hook(cleanup);
    printf("%d %d\n", v[0], cmp3(argv[0], "x", (size_t)argc));
    HOOK();
    run_either(argc);
    keep_and_register();
    signal(SIGUSR1, (void (*)(int))wrap_up);
    if (argc > 3)
        wrap_up();
    qsort(v, 4, sizeof v[0], (int (*)(const void *, const void *))score1);
    if (argc > 4)
        printf("%d %d\n", score1(v), score1(v + 1));
    pthread_create(&t, 0, (void *(*)(void *))start_puts, argv[0]);
    pthread_join(t, 0);
    glob("/tmp/*", 0, (int (*)(const char *, int))warn_count, &g);
    printf("%zu %d\n", g.gl_pathc, HITS);
    if (argc > 5)
        KEPT();
    return 0;
}

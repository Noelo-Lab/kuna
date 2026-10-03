/* A hardware watchpoint records actual accesses, including duplicates
 * that a comparison of final values cannot observe. */
#include <sys/ptrace.h>
#include <sys/user.h>
#include <sys/wait.h>
#include <signal.h>
#include <stddef.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>

extern volatile int sink;
extern int pick(unsigned);
#ifdef GLOBALORDER_MULTIGLOBAL
extern volatile unsigned char flag_a, flag_b;
extern volatile unsigned long uid_a, uid_b;
extern volatile long scaled;
extern unsigned long uid_sequence(unsigned *);
extern long scaled_sequence(long *);
#endif

int main(int argc, char **argv) {
    unsigned mode = argc > 1 ? strtoul(argv[1], 0, 0) : 0;
    unsigned long control = argc > 2 ? 0xf0001UL : 0xd0001UL;
    int pipefd[2];
    if (pipe(pipefd)) return 2;
    pid_t child = fork();
    if (child < 0) return 3;
    if (child == 0) {
        close(pipefd[0]);
        sink = 0;
        if (ptrace(PTRACE_TRACEME, 0, 0, 0) < 0) _exit(4);
        raise(SIGSTOP);
#ifdef GLOBALORDER_MULTIGLOBAL
        unsigned uid = argc > 2 ? strtoul(argv[2], 0, 0) : 17;
        long value = (long)uid;
        int result = mode ? (int)scaled_sequence(&value) : (int)uid_sequence(&uid);
#else
        int result = pick(mode);
#endif
        if (write(pipefd[1], &result, sizeof(result)) != sizeof(result)) _exit(11);
        _exit(0);
    }
    close(pipefd[1]);
    int status;
    if (waitpid(child, &status, 0) != child || !WIFSTOPPED(status)) return 4;
#ifdef GLOBALORDER_MULTIGLOBAL
    void *watched[] = {(void *)&flag_a, (void *)&flag_b,
        mode ? (void *)&scaled : (void *)&uid_a, (void *)&uid_b};
    control = 0x99110055UL;
    for (int i = 0; i < 4; i++) {
        if (ptrace(PTRACE_POKEUSER, child, offsetof(struct user, u_debugreg[0]) + i * sizeof(long), watched[i]) < 0) return 5;
    }
    if (ptrace(PTRACE_POKEUSER, child, offsetof(struct user, u_debugreg[7]), control) < 0) {
#else
    if (ptrace(PTRACE_POKEUSER, child, offsetof(struct user, u_debugreg[0]), (unsigned long)&sink) < 0
        || ptrace(PTRACE_POKEUSER, child, offsetof(struct user, u_debugreg[7]), control) < 0) {
#endif
        perror("watchpoint");
        return 5;
    }
    printf("accesses=");
    for (;;) {
        if (ptrace(PTRACE_CONT, child, 0, 0) < 0) return 6;
        if (waitpid(child, &status, 0) != child) return 7;
        if (WIFEXITED(status)) {
            if (WEXITSTATUS(status)) return 8;
            break;
        }
        if (!WIFSTOPPED(status) || WSTOPSIG(status) != SIGTRAP) return 9;
#ifdef GLOBALORDER_MULTIGLOBAL
        long fired = ptrace(PTRACE_PEEKUSER, child, offsetof(struct user, u_debugreg[6]), 0);
        if (fired < 0) return 10;
        for (int i = 0; i < 4; i++) if (fired & (1L << i)) {
            errno = 0;
            long value = ptrace(PTRACE_PEEKDATA, child, watched[i], 0);
            if (value == -1 && errno) return 10;
            if (i < 2) value &= 0xff;
            printf("%d:%ld,", i, value);
        }
        if (ptrace(PTRACE_POKEUSER, child, offsetof(struct user, u_debugreg[6]), 0) < 0) return 10;
#else
        errno = 0;
        long value = ptrace(PTRACE_PEEKDATA, child, (void *)&sink, 0);
        if (value == -1 && errno) return 10;
        printf("%d,", (int)value);
#endif
    }
    int result;
    if (read(pipefd[0], &result, sizeof(result)) != sizeof(result)) return 11;
    printf(" return=%d\n", result);
    close(pipefd[0]);
    return 0;
}

/* Two locals given one name whose storage overlaps (eax within rax).
   gcc -O2 -fno-inline -o sharednames_x86_64 sharednames_x86_64.c */
#define _GNU_SOURCE
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/wait.h>

int sprint_status(char *os, int status, int sigonly)
{
    char *s = os;
    int st = WEXITSTATUS(status);
    if (!WIFEXITED(status)) {
        st = WSTOPSIG(status);
        if (!WIFSTOPPED(status))
            st = WTERMSIG(status);
        if (sigonly) {
            if (st == SIGINT || st == SIGPIPE)
                goto out;
            if (WIFSTOPPED(status))
                goto out;
        }
        s = stpncpy(s, strsignal(st), 32);
        if (WCOREDUMP(status))
            s = stpcpy(s, " (core dumped)");
    } else if (!sigonly) {
        if (st)
            s += snprintf(s, 16, "Done(%d)", st);
        else
            s = stpcpy(s, "Done");
    }
out:
    return s - os;
}

int main(int argc, char **argv)
{
    char buf[64];
    int n = sprint_status(buf, argc << 8, argc > 2);
    puts(buf);
    return n;
}

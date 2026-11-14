/* libcnames fixture: stack objects whose address a library call takes.
 * gcc 11.4 -O2 -fno-inline -fno-stack-protector -U_FORTIFY_SOURCE -o libcnames_addr_x86_64 libcnames_addr.c */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/wait.h>
#include <unistd.h>

struct hdr {
    int magic;
    short ver;
    short n;
    long size;
};

long echo(int fd)
{
    char block[256];
    long got = read(fd, block, sizeof block);
    if (got > 0)
        got = write(1, block, got);
    return got;
}

long file_size(const char *path)
{
    struct stat st;
    if (stat(path, &st) < 0)
        return -1;
    return st.st_size;
}

int child(void)
{
    int status;
    pid_t pid = fork();
    if (pid == 0)
        _exit(3);
    waitpid(pid, &status, 0);
    return WEXITSTATUS(status);
}

long parse(const char *s)
{
    char *end;
    long v = strtol(s, &end, 10);
    if (*end)
        return -1;
    return v;
}

int read_hdr(FILE *fp)
{
    struct hdr hdr;
    if (fread(&hdr, sizeof hdr, 1, fp) != 1)
        return -1;
    return hdr.magic + hdr.n;
}

long when(void)
{
    struct timeval tv;
    explicit_bzero(&tv, sizeof tv);
    gettimeofday(&tv, NULL);
    return tv.tv_sec;
}

int keep(char *buf)
{
    return buf[0] == 'k';
}

int stash(void)
{
    char scratch[32];
    scratch[0] = 'k';
    return keep(scratch);
}

int main(int argc, char **argv)
{
    if (argc < 2)
        return 2;
    printf("%ld %ld %d %ld %d %d %d\n", echo(0), file_size(argv[1]), child(), parse(argv[1]), read_hdr(stdin),
           (int)when(), stash());
    return 0;
}

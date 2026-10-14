/* libcnames fixture: locals that meet library calls.
 * gcc -O2 -fno-inline -fno-stack-protector -o libcnames_x86_64 libcnames_x86_64.c
 * gcc -O0 -fno-stack-protector -o libcnames_O0_x86_64 libcnames_x86_64.c */
#include <fcntl.h>
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

long copy_file(const char *from, const char *to)
{
    int in = open(from, O_RDONLY);
    int out = open(to, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    char block[512];
    long total = 0;
    ssize_t got;
    while ((got = read(in, block, sizeof block)) > 0)
        total += write(out, block, got);
    close(in);
    close(out);
    return total;
}

char *joined(const char *head, const char *tail)
{
    size_t head_len = strlen(head);
    size_t tail_len = strlen(tail);
    char *both = malloc(head_len + tail_len + 1);
    if (both == NULL)
        return NULL;
    memcpy(both, head, head_len);
    memcpy(both + head_len, tail, tail_len + 1);
    return both;
}

int first_line(const char *path, char *line, int cap)
{
    FILE *stream = fopen(path, "r");
    if (stream == NULL)
        return -1;
    char *got = fgets(line, cap, stream);
    fclose(stream);
    return got != NULL;
}

char *pick(char *a, char *b, int which)
{
    return which ? a : b;
}

int ordered(char *a, char *b, int which, const char *lo, const char *hi)
{
    char *mid = pick(a, b, which);
    return strcmp(lo, mid) <= 0 && strcmp(mid, hi) <= 0;
}

size_t measure(const char *msg)
{
    size_t width = 0;
    while (msg[width] != '\0' && msg[width] != '\n')
        width++;
    return width;
}

size_t emit(int fd, const char *msg)
{
    size_t width = measure(msg);
    if (write(fd, msg, width) < 0)
        return 0;
    return width;
}

int pid;

int note_pid(void)
{
    int self = getpid();
    if (self > pid)
        pid = self;
    return self * 2 + pid;
}

int self_pid(void)
{
    int me = getpid();
    return me * 3 + (me > 7);
}

int stamp(time_t a, time_t b)
{
    struct tm *x = localtime(&a);
    int y = x->tm_year;
    int d = x->tm_mday;
    struct tm *z = localtime(&b);
    return y * 1000 + z->tm_yday + d + z->tm_hour;
}

int can(const char *path, const char *how)
{
    int mode = atoi(how) & 7;
    int r = access(path, mode);
    printf("%d %d\n", mode, r);
    return r + mode;
}

int main(int argc, char **argv)
{
    char line[64], scratch[32];
    if (argc < 3)
        return 2;
    char *both = joined(argv[1], argv[2]);
    if (ordered(scratch, line, argc & 1, argv[1], argv[2]))
        emit(1, argv[2]);
    puts(both);
    free(both);
    printf("%ld\n", copy_file(argv[1], argv[2]));
    printf("%d %d %d %d\n", note_pid() - pid, self_pid(), stamp(0, 100000), can(argv[1], argv[2]));
    return first_line(argv[1], line, sizeof line);
}

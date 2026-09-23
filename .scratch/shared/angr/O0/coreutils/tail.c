// Function: usage @ 0x4115
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern FILE *stderr;

void usage(int a0)
{
    unsigned long long v2;  // rbx
    char *v3;  // rax
    char *v12;  // rax
    char *v13;  // rax
    char *v15;  // rax
    char *v16;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    char *v21;  // rax
    long long v6;  // rdi
    long long v7;  // rsi
    long long v8;  // rdx
    long long v9;  // rcx
    long long v10;  // r8
    long long v11;  // r9
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    if (a0)
    {
        v3 = gettext("Try '%s --help' for more information.\n");
        fprintf(stderr, v3);
    }
    else
    {
        printf(gettext("Usage: %s [OPTION]... [FILE]...\n"));
        printf(gettext("Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n"));
        sub_402a69(v6, v7, v8, v9, v10, v11);
        sub_402a9a(v6, v7, v8, v9, v10, v11);
        v12 = gettext("  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n");
        fputs_unlocked(v12, stdout);
        v13 = gettext("  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means 'descriptor'\n  -F                       same as --follow=name --retry\n");
        fputs_unlocked(v13, stdout);
        printf(gettext("  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n"));
        v15 = gettext("      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n");
        fputs_unlocked(v15, stdout);
        v16 = gettext("  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n");
        fputs_unlocked(v16, stdout);
        v17 = gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n");
        fputs_unlocked(v17, stdout);
        v18 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v18, stdout);
        v19 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v19, stdout);
        v20 = gettext("\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n");
        fputs_unlocked(v20, stdout);
        v21 = gettext("With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n");
        fputs_unlocked(v21, stdout);
        sub_402acb("tail");
    }
    exit(a0); /* do not return */
}



// Function: die_pipe @ 0x431c
void die_pipe(void)
{
    raise(13);
    exit(1); /* do not return */
}



// Function: check_output_alive @ 0x4338
extern char g_41812f;

void check_output_alive(void)
{
    unsigned int v0;  // [bp-0x18]
    unsigned short v1;  // [bp-0x14]
    unsigned short v2;  // [bp-0x12]

    if (g_41812f != 1)
        return;
    v0 = 1;
    v2 = 0;
    v1 = v2;
    if (poll(&v0, 1, 0) < 0)
    {
        return;
    }
    else if ((char)v2 & 24)
    {
        sub_40431c(); /* do not return */
    }
    else
    {
        return;
    }
}



// Function: valid_file_spec @ 0x43ba
long long valid_file_spec(unsigned int *a0)
{
    unsigned long long v1;  // rax

    v1 = !a0[15] ^ a0[14] == 0xffffffff;
    return _INSERT(v1, 0, (unsigned int)v1);
}



// Function: pretty_name @ 0x43ee
char * pretty_name(char **a0)
{
    return (!strcmp(*(a0), "-") ? gettext("standard input") : *(a0));
}



// Function: record_open_fd @ 0x4435
typedef struct struct_1 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[8];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned int field_30;
    char field_34;
    char padding_35[3];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
    char padding_44[20];
    unsigned long long field_58;
} struct_0;

struct_0 * record_open_fd(struct_0 *idx, unsigned int a1, unsigned long a2, struct_1 *index, unsigned int a4)
{
    unsigned long v1;  // rdx

    idx->field_38 = a1;
    idx->field_8 = a2;
    idx->field_10 = sub_40ee31(index);
    idx->field_18 = v1;
    idx->field_20 = index->field_0;
    idx->field_28 = index->field_8;
    idx->field_30 = index->field_18;
    idx->field_40 = a4;
    idx->field_58 = 0;
    idx->field_34 = 0;
    return idx;
}



// Function: close_fd @ 0x44d5
int close_fd(int a0, long long a1)
{
    unsigned int v1;  // eax
    char *v2;  // rax
    unsigned int v3;  // eax
    unsigned int v4;  // eax

    if (a0 == -0x1)
    {
        return v4;
    }
    else if (a0)
    {
        v1 = close(a0);
        if (!v1)
            return v1;
        sub_40e8ca(4, a1);
        v2 = gettext("closing %s (fd=%d)");
        return (unsigned long long)error(0, *(__errno_location()), v2);
    }
    else
    {
        return v3;
    }
}



// Function: write_header @ 0x4559
extern char g_4121c3;
extern char g_418020;

int write_header(char *a0)
{
    int v1;  // eax

    v1 = printf("%s==> %s <==\n", (!g_418020 ? "\n" : &g_4121c3), a0);
    g_418020 = 0;
    return v1;
}



// Function: xwrite_stdout @ 0x45a9
typedef struct FILE {
} FILE;

extern FILE *stdout;

long long xwrite_stdout(void* a0, unsigned long a1)
{
    unsigned long count;  // rax
    char *v2;  // rax
    int *err;  // rax
    unsigned long v4;  // rax

    if (!a1)
        return v4;
    count = fwrite_unlocked(a0, 1, a1, stdout);
    if (a1 <= count)
        return count;
    clearerr_unlocked(stdout);
    sub_40e8ca(4, "standard output");
    v2 = gettext("error writing %s");
    err = __errno_location();
    error(1, *(err), v2);
}



// Function: dump_remainder @ 0x4649
unsigned long long dump_remainder(char a0, unsigned long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v8;  // rax
    char *v9;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x2050]
    char v1;  // [bp-0x204c]
    unsigned long long v2;  // [bp-0x2048]
    unsigned long long v3;  // [bp-0x2040]
    unsigned long long v4;  // [bp-0x2038]
    unsigned long long v5;  // [bp-0x2030]
    char v6;  // [bp-0x2028]

    v0 = a2;
    v1 = a0;
    v3 = a3;
    v2 = 0;
    do
    {
        if (v3 > 0x2000)
            v8 = 0x2000;
        else
            v8 = v3;
        v4 = v8;
        v5 = sub_40ed34();
        if (v5 == 0xffffffffffffffff)
        {
            if (*(__errno_location()) != 11)
            {
                sub_40e8ca(4, a1);
                v9 = gettext("error reading %s");
                err = __errno_location();
                error(1, *(err), v9);
            }
            return v2;
        }
        if (!v5)
            return v2;
        if (v1)
        {
            sub_404559(a1);
            v1 = 0;
        }
        sub_4045a9(&v6, v5);
        v2 += v5;
    } while (a3 == 0xffffffffffffffff || !(v3 -= v5, !v3 || a3 == 0xfffffffffffffffe));
    return v2;
}



// Function: xlseek @ 0x4804
unsigned long long xlseek(int a0, long long a1, int a2, long long a3)
{
    unsigned long v5;  // fs
    char *v6;  // rax
    int *err;  // rax
    char *v8;  // rax
    int *err1;  // rax
    char *v10;  // rax
    int *err2;  // rax
    unsigned long long v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x40]
    char v2;  // [bp-0x38]
    unsigned long v3;  // [bp-0x20]

    v3 = *((long long *)(40 + v5));
    v0 = lseek(a0, a1, a2);
    if (v0 - 0 >> 63 & 1)
    {
        v1 = sub_40cade(a1, &v2, &v2);
        if (a2 != 2)
        {
            if (a2 <= 2)
            {
                if (a2)
                {
                    if (a2 == 1)
                    {
                        sub_40ea5c(0, 3, a3);
                        v6 = gettext("%s: cannot seek to relative offset %s");
                        err = __errno_location();
                        error(0, *(err), v6);
                    }
                    else
                    {
                        abort(); /* do not return */
                    }
                }
                else
                {
                    sub_40ea5c(0, 3, a3);
                    v8 = gettext("%s: cannot seek to offset %s");
                    err1 = __errno_location();
                    error(0, *(err1), v8);
                }
            }
            else
            {
                abort(); /* do not return */
            }
        }
        else
        {
            sub_40ea5c(0, 3, a3);
            v10 = gettext("%s: cannot seek to end-relative offset %s");
            err2 = __errno_location();
            error(0, *(err2), v10);
        }
        exit(1); /* do not return */
    }
    else if (v3 != *((long long *)(40 + v5)))
    {
        __stack_chk_fail(); /* do not return */
    }
    else
    {
        return v0;
    }
}



// Function: file_lines @ 0x49bd
extern char g_418132;

unsigned int file_lines(long long a0, unsigned int a1, unsigned long long a2, long long a3, unsigned long long a4, unsigned long long *a5)
{
    char *v9;  // rax
    int *err;  // rax
    unsigned long long v11;  // rax
    char *v12;  // rax
    int *err1;  // rax
    unsigned long long v0;  // [bp-0x2060]
    unsigned long long v1;  // [bp-0x2060]
    long long v2;  // [bp-0x2048], Other Possible Types: unsigned long
    long long iter;  // [bp-0x2040], Other Possible Types: unsigned long long
    unsigned long v4;  // [bp-0x2038]
    void* v5;  // [bp-0x2030]
    char v6;  // [bp-0x2029]
    char v7;  // [bp-0x2028]

    v0 = a2;
    iter = a4;
    if (!v0)
        return 1;
    v2 = ((unsigned int)(iter - a3) + (unsigned int)((long long)(iter - a3) >> 63 >> 0x33) & 0x1fff) - ((long long)(iter - a3) >> 63 >> 0x33);
    if (!v2)
        v2 = 0x2000;
    iter -= v2;
    sub_404804(a1, iter, 0, a0);
    v2 = sub_40ed34(a1, &v7, v2, &v7);
    if (v2 == 0xffffffffffffffff)
    {
        sub_40e8ca(4, a0);
        v9 = gettext("error reading %s");
        err = __errno_location();
        error(0, *(err), v9);
        return 0;
    }
    *(a5) = iter + v2;
    if (v2 && (&v6)[v2] != g_418132)
        v0 -= 1;
    while (1)
    {
        v4 = v2;
        v1 = v0;
        while (1)
        {
            v0 = v1;
            if (!v4 || !(v5 = memrchr(&v7, (int)(char)g_418132, v4), v5))
                break;
            v4 = v5 - &v7;
            v1 = v0 - 1;
            if (!v0)
            {
                sub_4045a9(v5 + 1, v2 - v4 - 1);
                v11 = sub_404649(0, a0, a1, a4 - (v2 + iter));
                *(a5) = v11 + *(a5);
                return 1;
            }
        }
        if (iter == a3)
        {
            sub_404804(a1, a3, 0, a0);
            *(a5) = a3 + sub_404649(0, a0, a1, a4);
            return 1;
        }
        iter -= 0x2000;
        sub_404804(a1, iter, 0, a0);
        v2 = sub_40ed34(a1, &v7, 0x2000, &v7);
        if (v2 == 0xffffffffffffffff)
        {
            sub_40e8ca(4, a0);
            v12 = gettext("error reading %s");
            err1 = __errno_location();
            error(0, *(err1), v12);
            return 0;
        }
        *(a5) = iter + v2;
        if (!v2)
            return 1;
    }
}



// Function: pipe_lines @ 0x4df7
typedef struct struct_0 {
    char field_0;
    char padding_1[8191];
    char field_2000;
    char padding_2001[7];
    unsigned long long field_2008;
    struct struct_0 *field_2010;
} struct_0;

extern char g_418132;

int pipe_lines(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    char *v12;  // rax
    int *err;  // rax
    char v0;  // [bp-0x69]
    struct_0 *node;  // [bp-0x68]
    struct_0 *iter2;  // [bp-0x60]
    struct_0 *iter1;  // [bp-0x58]
    unsigned long long iter;  // [bp-0x50]
    struct_0 *ptr1;  // [bp-0x48]
    struct_0 *ptr;  // [bp-0x40]
    unsigned long long j;  // [bp-0x38]
    unsigned long long v8;  // [bp-0x30]
    struct_0 *v9;  // [bp-0x28]
    struct_0 *v10;  // [bp-0x20]

    iter = 0;
    v0 = 1;
    iter2 = sub_40f96c(8216);
    node = iter2;
    node->field_2008 = 0;
    *((unsigned long long *)&node->field_2000) = node->field_2008;
    node->field_2010 = NULL;
    iter1 = sub_40f96c(8216);
    while (1)
    {
        v8 = sub_40ed34(a1, iter1, 0x2000, iter1);
        if (!v8 || v8 == 0xffffffffffffffff)
            break;
        *((unsigned long long *)&iter1->field_2000) = v8;
        *(a3) = *(a3) + v8;
        iter1->field_2008 = 0;
        iter1->field_2010 = 0;
        v9 = &(&iter1->field_0)[v8];
        ptr1 = iter1;
        while (1)
        {
            ptr1 = memchr(ptr1, g_418132, v9 - ptr1);
            if (!ptr1)
                break;
            ptr1 = ptr1->padding_1;
            iter1->field_2008 = iter1->field_2008 + 1;
        }
        iter += iter1->field_2008;
        if (*((long long *)&iter2->field_2000) + *((long long *)&iter1->field_2000) <= 0x1fff)
        {
            memcpy(&(&iter2->field_0)[*((long long *)&iter2->field_2000)], iter1, *((long long *)&iter1->field_2000));
            *((long long *)&iter2->field_2000) = *((long long *)&iter2->field_2000) + *((long long *)&iter1->field_2000);
            iter2->field_2008 = iter2->field_2008 + iter1->field_2008;
        }
        else
        {
            iter2->field_2010 = iter1;
            iter2 = iter2->field_2010;
            if (a2 < iter - node->field_2008)
            {
                iter1 = node;
                iter -= node->field_2008;
                node = node->field_2010;
            }
            else
            {
                iter1 = sub_40f96c(8216);
            }
        }
    }
    free(iter1);
    if (v8 == 0xffffffffffffffff)
    {
        sub_40e8ca(4, a0);
        v12 = gettext("error reading %s");
        err = __errno_location();
        error(0, *(err), v12);
        v0 = 0;
    }
    else if (a2)
    {
        if (*(&(&iter2->field_0)[*((long long *)&iter2->field_2000)] - 1) != g_418132)
        {
            iter2->field_2008 = iter2->field_2008 + 1;
            iter += 1;
        }
        for (iter1 = node; a2 < iter - iter1->field_2008; iter1 = iter1->field_2010)
        {
            iter -= iter1->field_2008;
        }
        ptr = iter1;
        v10 = &(&iter1->field_0)[*((long long *)&iter1->field_2000)];
        if (iter > a2)
        {
            for (j = iter - a2; j; j -= 1)
            {
                ptr = rawmemchr(ptr, g_418132);
                ptr = ptr->padding_1;
            }
        }
        sub_4045a9(ptr, v10 - ptr);
        for (iter1 = iter1->field_2010; iter1; iter1 = iter1->field_2010)
        {
            sub_4045a9(iter1, *((long long *)&iter1->field_2000));
        }
    }
    for (; node; node = iter1)
    {
        iter1 = node->field_2010;
        free(node);
    }
    return v0;
}



// Function: pipe_bytes @ 0x5286
typedef struct struct_0 {
    char padding_0[8192];
    char field_2000;
    char padding_2001[7];
    struct struct_0 *field_2008;
} struct_0;

int pipe_bytes(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    char *v8;  // rax
    int *err;  // rax
    char v0;  // [bp-0x49]
    struct_0 *node;  // [bp-0x48]
    struct_0 *iter2;  // [bp-0x40]
    struct_0 *iter1;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x30]
    unsigned long long iter;  // [bp-0x28]
    unsigned long long v6;  // [bp-0x20]

    iter = 0;
    v0 = 1;
    iter2 = sub_40f96c(8208);
    node = iter2;
    *((unsigned long long *)&node->field_2000) = 0;
    node->field_2008 = NULL;
    iter1 = sub_40f96c(8208);
    while (1)
    {
        v6 = sub_40ed34(a1, iter1, 0x2000, iter1);
        if (!v6 || v6 == 0xffffffffffffffff)
            break;
        *(a3) = *(a3) + v6;
        *((unsigned long long *)&iter1->field_2000) = v6;
        iter1->field_2008 = NULL;
        iter += *((long long *)&iter1->field_2000);
        if (*((long long *)&iter2->field_2000) + *((long long *)&iter1->field_2000) <= 0x1fff)
        {
            memcpy(&iter2->padding_0[*((long long *)&iter2->field_2000)], iter1, *((long long *)&iter1->field_2000));
            *((long long *)&iter2->field_2000) = *((long long *)&iter2->field_2000) + *((long long *)&iter1->field_2000);
        }
        else
        {
            iter2->field_2008 = iter1;
            iter2 = iter2->field_2008;
            if (a2 < iter - *((long long *)&node->field_2000))
            {
                iter1 = node;
                iter -= *((long long *)&node->field_2000);
                node = node->field_2008;
            }
            else
            {
                iter1 = sub_40f96c(8208);
            }
        }
    }
    free(iter1);
    if (v6 == 0xffffffffffffffff)
    {
        sub_40e8ca(4, a0);
        v8 = gettext("error reading %s");
        err = __errno_location();
        error(0, *(err), v8);
        v0 = 0;
    }
    else
    {
        for (iter1 = node; a2 < iter - *((long long *)&iter1->field_2000); iter1 = iter1->field_2008)
        {
            iter -= *((long long *)&iter1->field_2000);
        }
        v4 = (iter <= a2 ? 0 : iter - a2);
        sub_4045a9(&iter1->padding_0[v4], *((long long *)&iter1->field_2000) - v4);
        for (iter1 = iter1->field_2008; iter1; iter1 = iter1->field_2008)
        {
            sub_4045a9(iter1, *((long long *)&iter1->field_2000));
        }
    }
    for (; node; node = iter1)
    {
        iter1 = node->field_2008;
        free(node);
    }
    return v0;
}



// Function: start_bytes @ 0x55ae
unsigned int start_bytes(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    char *v5;  // rax
    int *err;  // rax
    unsigned long long i;  // [bp-0x2050]
    unsigned long long v1;  // [bp-0x2038]
    unsigned long long v2;  // [bp-0x2030]
    char v3;  // [bp-0x2028]

    for (i = a2; i; i -= v1)
    {
        v1 = sub_40ed34(a1, &v3, 0x2000, &v3);
        if (!v1)
            return 0xffffffff;
        if (v1 == 0xffffffffffffffff)
        {
            sub_40e8ca(4, a0);
            v5 = gettext("error reading %s");
            err = __errno_location();
            error(0, *(err), v5);
            return 1;
        }
        *(a3) = *(a3) + v1;
        if (v1 > i)
        {
            v2 = v1 - i;
            sub_4045a9(&(&v3)[i], v2);
            break;
        }
    }
    return 0;
}



// Function: start_lines @ 0x5741
extern char g_418132;

unsigned int start_lines(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    char *v6;  // rax
    int *err;  // rax
    unsigned long long v0;  // [bp-0x2060]
    char *ptr;  // [bp-0x2040]
    unsigned long long v2;  // [bp-0x2038]
    char *v3;  // [bp-0x2030]
    char v4;  // [bp-0x2028]

    v0 = a2;
    if (!v0)
        return 0;
    while (1)
    {
        v2 = sub_40ed34(a1, &v4, 0x2000, &v4);
        if (!v2)
        {
            return 0xffffffff;
        }
        else if (v2 == 0xffffffffffffffff)
        {
            sub_40e8ca(4, a0);
            v6 = gettext("error reading %s");
            err = __errno_location();
            error(0, *(err), v6);
            return 1;
        }
        else
        {
            v3 = &(&v4)[v2];
            *(a3) = *(a3) + v2;
            ptr = &v4;
            while (1)
            {
                ptr = memchr(ptr, g_418132, v3 - ptr);
                if (!ptr)
                    break;
                ptr += 1;
                v0 -= 1;
                if (!v0)
                {
                    if (ptr >= v3)
                        return 0;
                    sub_4045a9(ptr, v3 - ptr);
                    return 0;
                }
            }
        }
    }
}



// Function: fremote @ 0x5938
long long fremote(unsigned int a0, long long a1)
{
    char *v4;  // rax
    int *err;  // rax
    unsigned int v6;  // eax
    char v0;  // [bp-0x9d]
    unsigned int v1;  // [bp-0x9c]
    char v2;  // [bp-0x98]

    v0 = 1;
    v1 = fstatfs(a0, &v2, &v2);
    if (!v1)
    {
        v6 = sub_402da1(v2);
        if (v6 == 1)
        {
            v0 = 0;
            return v0;
        }
        if (v6 <= 1)
        {
            if (v6 != 0xffffffff)
            {
                if (!v6)
                    return v0;
            }
            else
            {
                return v0;
            }
        }
        __assert_fail(); /* do not return */
    }
    else if (*(__errno_location()) != 38)
    {
        sub_40e8ca(4, a1);
        v4 = gettext("cannot determine location of %s. reverting to polling");
        err = __errno_location();
        error(0, *(err), v4);
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: recheck @ 0x5a69
typedef struct struct_1 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[8];
    unsigned int field_18;
} struct_1;

typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

extern char g_418010;
extern char g_41812c;
extern char g_41813a;

void recheck(void* idx, char a1)
{
    unsigned int v8;  // eax
    char v9;  // al
    char *v20;  // rax
    int *err;  // rax
    unsigned long v24;  // rax
    unsigned int v10;  // eax
    unsigned long v32;  // rax
    unsigned int v13;  // eax
    char v14;  // al
    char v0;  // [bp-0xc4]
    char v1;  // [bp-0xc3]
    char v2;  // [bp-0xc2]
    char v3;  // [bp-0xc1]
    unsigned int v4;  // [bp-0xc0]
    unsigned int v5;  // [bp-0xbc]
    struct_1 v6;  // [bp-0xb8], Other Possible Types: stat

    v0 = 1;
    v2 = !strcmp(*((long long *)idx), "-");
    v3 = (char)idx[54];
    v4 = (int)idx[60];
    if (!v2)
        v8 = sub_40af09(*((long long *)idx), (!a1 ? 0x800 : 0));
    else
        v8 = 0;
    v5 = v8;
    if (!(char)sub_4043ba(idx))
        __assert_fail(); /* do not return */
    if (g_41812c != 1 || v5 != -0x1)
        v9 = 1;
    else
        v9 = 0;
    *((char *)&idx[54]) = v9 & 1;
    if (g_41813a != 1 && !(v10 = (unsigned int)(int)(long long)lstat(*((long long *)idx), &v6), v10 || ((unsigned short)v6.st_nlink & 0xf000) != 0xa000))
    {
        v0 = 0;
        *((unsigned int *)&idx[60]) = 0xffffffff;
        *((char *)&idx[52]) = 1;
        sub_40e8ca(4, sub_4043ee(idx));
        error(0, 0, gettext("%s has been replaced with an untailable symbolic link"));
    }
    else
    {
        if (v5 == -0x1 || (v13 = (unsigned int)(int)(long long)fstat(v5, &v6), (int)(long long)fstat(v5, &v6) < 0))
        {
            v0 = 0;
            *((int *)&idx[60]) = *(__errno_location());
            if ((char)idx[54] != 1)
            {
                if (v3)
                {
                    sub_40e8ca(4, sub_4043ee(idx));
                    v20 = gettext("%s has become inaccessible");
                    error(0, (int)idx[60], v20);
                }
            }
            else
            {
                if (v4 != *(__errno_location()))
                {
                    sub_40ea5c(0, 3, sub_4043ee(idx));
                    err = __errno_location();
                    error(0, *(err), "%s");
                }
            }
        }
        else if (((unsigned short)v6.st_nlink & 0xf000) != 0x8000 && ((unsigned short)v6.st_nlink & 0xf000) != 0x1000 && ((unsigned short)v6.st_nlink & 0xf000) != 0xc000 && ((unsigned short)v6.st_nlink & 0xf000) != 0x2000)
        {
            v0 = 0;
            *((unsigned int *)&idx[60]) = 0xffffffff;
            *((char *)&idx[54]) = 0;
            if (g_41812c != 1 || *((int *)&g_418010) != 1)
                v14 = 1;
            else
                v14 = 0;
            *((char *)&idx[52]) = v14 & 1;
            if (v3 || v4 != (int)idx[60])
            {
                if ((char)idx[52])
                    gettext("; giving up on this name");
                sub_40e8ca(4, sub_4043ee(idx));
                error(0, 0, gettext("%s has been replaced with an untailable file%s"));
            }
        }
        else
        {
            *((char *)&idx[53]) = sub_405938(v5, sub_4043ee(idx));
            if ((char)idx[53] && g_41813a != 1)
            {
                v0 = 0;
                *((unsigned int *)&idx[60]) = 0xffffffff;
                sub_40e8ca(4, sub_4043ee(idx));
                error(0, 0, gettext("%s has been replaced with an untailable remote file"));
                *((char *)&idx[52]) = 1;
                *((char *)&idx[53]) = 1;
            }
            else
            {
                *((unsigned int *)&idx[60]) = 0;
            }
        }
    }
    v1 = 0;
    if (v0 != 1)
    {
        sub_4044d5(v5, sub_4043ee(idx));
        v24 = sub_4043ee(idx);
        sub_4044d5((int)idx[56], v24);
        *((int *)&idx[56]) = -0x1;
    }
    else if (v4 && v4 != 2)
    {
        v1 = 1;
        if ((int)idx[56] != -0x1)
            __assert_fail(); /* do not return */
        sub_40e8ca(4, sub_4043ee(idx));
        error(0, 0, gettext("%s has become accessible"));
    }
    else if ((int)idx[56] == -0x1)
    {
        v1 = 1;
        sub_40e8ca(4, sub_4043ee(idx));
        error(0, 0, gettext("%s has appeared;  following new file"));
    }
    else if ((long long)idx[40] != v6.field_8 || (long long)idx[32] != v6.field_0)
    {
        v1 = 1;
        sub_40e8ca(4, sub_4043ee(idx));
        error(0, 0, gettext("%s has been replaced;  following new file"));
        v32 = sub_4043ee(idx);
        sub_4044d5((int)idx[56], v32);
    }
    else
    {
        sub_4044d5(v5, sub_4043ee(idx));
    }
    if (!v1)
        return;
    sub_404435(idx, v5, 0, &v6, (!v2 ? a1 : 0xffffffff));
    sub_404804(v5, 0, 0, sub_4043ee(idx));
    return;
}



// Function: any_live_files @ 0x6245
extern unsigned int g_418010;
extern char g_41812c;

unsigned int any_live_files(unsigned long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x10]

    if (g_41812c && g_418010 == 1)
        return 1;
    for (i = 0; i < a1; i += 1)
    {
        if (*((int *)(a0 + i * 96 + 56)) >= 0)
            return 1;
        if (*((char *)(a0 + i * 96 + 52)) != 1 && g_41812c)
            return 1;
    }
    return 0;
}



// Function: tail_forever @ 0x62f3
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    char padding_20[16];
    unsigned int field_30;
    char padding_34[4];
    unsigned int field_38;
    unsigned int field_3c;
    char padding_40[24];
    unsigned long long field_58;
} struct_0;

typedef struct FILE {
} FILE;

extern unsigned int g_418010;
extern char g_418018;
extern FILE *stdout;
extern char g_418131;
extern int g_418134;

void tail_forever(struct_0 *a0, unsigned long long a1)
{
    unsigned long long v15;  // xmm0lq
    char v16;  // al
    char v26;  // al
    char *v27;  // rax
    int *err;  // rax
    char *v30;  // rax
    int *err1;  // rax
    char *v32;  // rax
    int *err2;  // rax
    struct_0 *v18;  // rcx
    int *v19;  // rax
    long long v20;  // rax
    long long v21;  // rdx
    struct_0 *idx;  // rax
    unsigned long v23;  // rdx
    struct_0 *index;  // rbx
    unsigned long long v0;  // [bp-0x110]
    char v1;  // [bp-0xf3]
    char v2;  // [bp-0xf2]
    char v3;  // [bp-0xf1]
    unsigned int v4;  // [bp-0xf0]
    unsigned int v5;  // [bp-0xec]
    unsigned int v6;  // [bp-0xe8]
    unsigned int v7;  // [bp-0xe4]
    unsigned long long v8;  // [bp-0xe0]
    unsigned long long v9;  // [bp-0xd8]
    unsigned long long v10;  // [bp-0xd0]
    unsigned long v11;  // [bp-0xc8]
    unsigned long long v12;  // [bp-0xc0]
    int v13;  // [bp-0xb8]

    v0 = v15;
    if (!g_418134 && g_418010 == 2 && a1 == 1 && a0->field_38 != 0xffffffff && ((unsigned short)a0->field_30 & 0xf000) != 0x8000)
        v16 = 1;
    else
        v16 = 0;
    v3 = v16;
    v3 &= 1;
    v1 = 0;
    v8 = a1 - 1;
    while (1)
    {
        v2 = 0;
        v9 = 0;
        while (1)
        {
            if (v9 >= a1)
            {
                if ((char)sub_406245(a0, a1) != 1)
                {
                    error(0, 0, gettext("no files remaining"));
                    return;
                }
                if ((v2 != 1 || v3) && fflush_unlocked(stdout))
                {
                    v30 = gettext("write error");
                    err1 = __errno_location();
                    error(1, *(err1), v30);
                }
                sub_404338();
                if (v2 != 1)
                {
                    if (!v1)
                    {
                        if (g_418134 && kill(g_418134, 0) && *(__errno_location()) != 1)
                            v26 = 1;
                        else
                            v26 = 0;
                        v1 = v26;
                        v1 &= 1;
                        if (v1 != 1)
                        {
                            if (sub_4101af())
                            {
                                v32 = gettext("cannot read realtime clock");
                                err2 = __errno_location();
                                error(1, *(err2), v32);
                            }
                            break;
                        }
                    }
                    else
                    {
                        return;
                    }
                }
            }
            if (!a0[v9].padding_34[0])
            {
                if (a0[v9].field_38 < 0)
                {
                    sub_405a69(&a0[v9], v3);
                }
                else
                {
                    v4 = a0[v9].field_38;
                    v11 = sub_4043ee(&a0[v9]);
                    v5 = a0[v9].field_30;
                    if (*((int *)&a0[v9].padding_40[0]) != v3)
                    {
                        v6 = sub_40aa63(v4, 3);
                        v7 = (!v3 ? 0x800 : 0) | v6;
                        if (!(v6 < 0 || v7 != v6 && sub_40aa63(v4, 4, v7) == 0xffffffff))
                        {
                            *((unsigned int *)&a0[v9].padding_40[0]) = v3;
                        }
                        else if (((unsigned short)a0[v9].field_30 & 0xf000) != 0x8000 || *(__errno_location()) != 1)
                        {
                            sub_40ea5c(0, 3, v11);
                            v27 = gettext("%s: cannot change nonblocking mode");
                            err = __errno_location();
                            error(1, *(err), v27);
                        }
                    }
                    if (*((int *)&a0[v9].padding_40[0]))
                    {
LABEL_406a58:
                        if (*((int *)&a0[v9].padding_40[0]))
                        {
                            v10 = 0xfffffffffffffffe;
                        }
                        else
                        {
                            if (((unsigned short)v5 & 0xf000) == 0x8000 && a0[v9].padding_34[1])
                                v10 = *((unsigned long long *)(&v13 + 48)) - *((long long *)&a0[v9].padding_0[8]);
                            else
                                v10 = 0xffffffffffffffff;
                        }
                        v12 = sub_404649(0, v11, v4, v10);
                        v2 = v12 || v2;
                        *((unsigned long long *)&a0[v9].padding_0[8]) = *((long long *)&a0[v9].padding_0[8]) + v12;
                    }
                    else if (fstat(v4, &v13))
                    {
                        a0[v9].field_38 = 0xffffffff;
                        v18 = &a0[v9];
                        v18->field_3c = *(__errno_location());
                        sub_40ea5c(0, 3, v11, v18);
                        v19 = __errno_location();
                        error(0, *(v19), "%s");
                        close(v4);
                    }
                    else if (a0[v9].field_30 == *((unsigned int *)(&v13 + 24)) && (((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) != 0x8000 || *((long long *)&a0[v9].padding_0[8]) == *((unsigned long long *)(&v13 + 48))) && !(v20 = (long long)(unsigned long long)sub_40ee31(&v13), (unsigned int)(unsigned long long)sub_40eeaf(a0[v9].field_10, a0[v9].field_18, (unsigned long long)sub_40ee31(&v13), v21)))
                    {
                        idx = &a0[v9];
                        v23 = idx->field_58;
                        idx->field_58 = v23 + 1;
                        if (v23 >= *((long long *)&g_418018) && g_418010 == 1)
                        {
                            sub_405a69(&a0[v9], *((int *)&a0[v9].padding_40[0]));
                            a0[v9].field_58 = 0;
                        }
                    }
                    else
                    {
                        index = &a0[v9];
                        index->field_10 = sub_40ee31(&v13);
                        index->field_18 = v21;
                        a0[v9].field_30 = *((unsigned int *)(&v13 + 24));
                        a0[v9].field_58 = 0;
                        if (((unsigned short)v5 & 0xf000) == 0x8000 && *((unsigned long long *)(&v13 + 48)) < *((long long *)&a0[v9].padding_0[8]))
                        {
                            sub_40ea5c(0, 3, v11);
                            error(0, 0, gettext("%s: file truncated"));
                            sub_404804(v4, 0, 0, v11);
                            *((unsigned long *)&a0[v9].padding_0[8]) = 0;
                        }
                        if (v9 != v8)
                        {
                            if (g_418131)
                                sub_404559(v11);
                            v8 = v9;
                            goto LABEL_406a58;
                        }
                    }
                }
            }
            v9 += 1;
        }
    }
}



// Function: any_remote_file @ 0x6d57
unsigned int any_remote_file(unsigned long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a1; i += 1)
    {
        if (*((int *)(a0 + i * 96 + 56)) >= 0 && *((char *)(a0 + i * 96 + 53)))
            return 1;
    }
    return 0;
}



// Function: any_non_remote_file @ 0x6dd3
unsigned int any_non_remote_file(unsigned long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a1; i += 1)
    {
        if (*((int *)(a0 + i * 96 + 56)) >= 0 && *((char *)(a0 + i * 96 + 53)) != 1)
            return 1;
    }
    return 0;
}



// Function: any_symlinks @ 0x6e52
typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

unsigned int any_symlinks(unsigned long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0xb0]
    stat v1;  // [bp-0xa8]

    for (i = 0; i < a1; i += 1)
    {
        if (!lstat(*((long long *)(a0 + i * 96)), &v1) && ((unsigned short)v1.st_nlink & 0xf000) == 0xa000)
            return 1;
    }
    return 0;
}



// Function: any_non_regular_fifo @ 0x6f11
unsigned int any_non_regular_fifo(unsigned long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a1; i += 1)
    {
        if (*((int *)(a0 + i * 96 + 56)) >= 0 && ((short)*((int *)(a0 + i * 96 + 48)) & 0xf000) != 0x8000 && ((short)*((int *)(a0 + i * 96 + 48)) & 0xf000) != 0x1000)
            return 1;
    }
    return 0;
}



// Function: tailable_stdin @ 0x6fc5
unsigned int tailable_stdin(unsigned long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a1; i += 1)
    {
        if (*((char *)(a0 + i * 96 + 52)) != 1 && !strcmp(*((long long *)(a0 + i * 96)), "-"))
            return 1;
    }
    return 0;
}



// Function: wd_hasher @ 0x705a
typedef struct struct_0 {
    char padding_0[68];
    int field_44;
} struct_0;

long long wd_hasher(struct_0 *a0, unsigned long long a1)
{
    return a0->field_44 % a1;
}



// Function: wd_comparator @ 0x7089
typedef struct struct_0 {
    char padding_0[68];
    unsigned int field_44;
} struct_0;

long long wd_comparator(unsigned long a0, struct_0 *a1)
{
    unsigned long v1;  // rax

    v1 = a1->field_44;
    return _INSERT(v1, 0, *((int *)(a0 + 0x44)) == (unsigned int)v1);
}



// Function: check_fspec @ 0x70be
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[16];
    unsigned int field_30;
    char padding_34[4];
    unsigned int field_38;
    unsigned int field_3c;
} struct_0;

typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_418131;

void check_fspec(void* idx, struct_0 **a1)
{
    long long v7;  // rax
    long long v8;  // rdx
    char v9;  // al
    char *v10;  // rax
    int *err;  // rax
    char v0;  // [bp-0xc9]
    unsigned long v1;  // [bp-0xc8]
    unsigned long long v2;  // [bp-0xc0]
    stat v3;  // [bp-0xb8]

    if ((int)idx[56] == 0xffffffff)
        return;
    v1 = sub_4043ee(idx);
    if (fstat((int)idx[56], &v3))
    {
        *((int *)&idx[60]) = *(__errno_location());
        sub_4044d5((int)idx[56], v1);
        *((unsigned int *)&idx[56]) = 0xffffffff;
        return;
    }
    if (((short)(int)idx[48] & 0xf000) == 0x8000 && v3.st_atime < (long long)idx[8])
    {
        sub_40ea5c(0, 3, v1);
        error(0, 0, gettext("%s: file truncated"));
        sub_404804((int)idx[56], 0, 0, v1);
        *((long long *)&idx[8]) = 0;
    }
    else if (((short)(int)idx[48] & 0xf000) == 0x8000 && v3.st_atime == (long long)idx[8])
    {
        v7 = sub_40ee31(&v3);
        if (!sub_40eeaf((long long)idx[16], (long long)idx[24], v7, v8))
            return;
    }
    if (g_418131 && idx != *(a1))
        v9 = 1;
    else
        v9 = 0;
    v0 = v9;
    v0 &= 1;
    v2 = sub_404649(v0, v1, (int)idx[56], 0xffffffffffffffff);
    *((unsigned long long *)&idx[8]) = v2 + (long long)idx[8];
    if (!v2)
        return;
    *(a1) = idx;
    if (!fflush_unlocked(stdout))
        return;
    v10 = gettext("write error");
    err = __errno_location();
    error(1, *(err), v10);
}



// Function: tail_forever_inotify @ 0x7379
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[16];
    unsigned int field_30;
    char padding_34[4];
    unsigned int field_38;
    unsigned int field_3c;
} struct_0;

typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

extern char g_418010;
extern char g_41812c;
extern char g_41812f;
extern int g_418134;

long long tail_forever_inotify(unsigned int a0, unsigned long a1, long long a2, long long *a3)
{
    unsigned long long v37;  // rax
    unsigned int *err;  // rax
    char *v47;  // rax
    int *err1;  // rax
    char v49;  // al
    unsigned long long v50;  // xmm0lq
    char *v51;  // rax
    int *err2;  // rax
    char v53;  // al
    unsigned long v54;  // rdx
    unsigned long v55;  // rax
    unsigned long v56;  // rdx
    char *v39;  // rax
    char *v58;  // rax
    int *v59;  // rax
    char *v60;  // rax
    int *v61;  // rax
    unsigned long v64;  // fs
    int *v40;  // rax
    char *v41;  // rax
    int *v42;  // rax
    unsigned int v45;  // eax
    char v0;  // [bp-0x1ac]
    char v1;  // [bp-0x1ab]
    char v2;  // [bp-0x1aa]
    char v3;  // [bp-0x1a9]
    char v4;  // [bp-0x1a8]
    char v5;  // [bp-0x1a7]
    char v6;  // [bp-0x1a6]
    char v7;  // [bp-0x1a5]
    unsigned int v8;  // [bp-0x1a4]
    unsigned int v9;  // [bp-0x1a0]
    unsigned int v10;  // [bp-0x19c]
    unsigned int v11;  // [bp-0x198]
    unsigned int j;  // [bp-0x194]
    struct_0 *v13;  // [bp-0x190]
    unsigned long long v14;  // [bp-0x188]
    long long v15;  // [bp-0x180]
    unsigned long long v16;  // [bp-0x178]
    unsigned long long v17;  // [bp-0x170]
    long long iter;  // [bp-0x168]
    void* v19;  // [bp-0x160]
    long long k;  // [bp-0x158]
    long long v21;  // [bp-0x150]
    unsigned long long len;  // [bp-0x148]
    unsigned long long v23;  // [bp-0x140]
    unsigned long long v24;  // [bp-0x138]
    struct_0 *v25;  // [bp-0x130]
    struct_0 *v26;  // [bp-0x128]
    void* v27;  // [bp-0x120]
    char v28;  // [bp-0x118]
    unsigned int v29;  // [bp-0xd4]
    stat v30;  // [bp-0xb8], Other Possible Types: unsigned int
    unsigned short v31;  // [bp-0xb4]
    unsigned int v32;  // [bp-0xb0]
    unsigned short v33;  // [bp-0xac]
    unsigned short v34;  // [bp-0xaa]
    unsigned long v35;  // [bp-0x20]

    v8 = 3;
    v0 = 0;
    v1 = 0;
    v2 = 0;
    v3 = 0;
    v4 = 0;
    v14 = 0;
    v16 = 0;
    v17 = 0;
    v21 = sub_40b9fe(a2, 0, sub_40705a, sub_407089, 0);
    if (!v21)
        sub_40ffa4(); /* do not return */
    *(a3) = v21;
    v9 = 2;
    if (*((int *)&g_418010) == 1)
        v9 |= 3076;
    for (iter = 0; iter < a2; iter += 1)
    {
        if (*((char *)(a1 + iter * 96 + 52)) != 1)
        {
            len = strlen(*((long long *)(a1 + iter * 96)));
            if (v14 < len)
                v14 = len;
            *((unsigned int *)(a1 + iter * 96 + 0x44)) = 0xffffffff;
            if (*((int *)&g_418010) == 1)
            {
                v23 = sub_40a943(*((long long *)(a1 + iter * 96)));
                v5 = *((char *)(v23 + *((long long *)(a1 + iter * 96))));
                v37 = sub_40a685(*((long long *)(a1 + iter * 96)));
                *((unsigned long long *)(a1 + iter * 96 + 80)) = v37 - *((long long *)(a1 + iter * 96));
                *((char *)(v23 + *((long long *)(a1 + iter * 96)))) = 0;
                *((unsigned int *)(iter * 96 + a1 + 72)) = inotify_add_watch(a0, (!v23 ? "." : *((long long *)(a1 + iter * 96))), 1924);
                *((char *)(*((long long *)(a1 + iter * 96)) + v23)) = v5;
                if (*((int *)(a1 + iter * 96 + 72)) < 0)
                {
                    if (*(__errno_location()) != 28)
                    {
                        sub_40e8ca(4, *((long long *)(a1 + iter * 96)), iter * 96);
                        v41 = gettext("cannot watch parent directory of %s");
                        v42 = __errno_location();
                        error(0, *(v42), v41);
                    }
                    else
                    {
                        error(0, 0, gettext("inotify resources exhausted"));
                    }
                    v2 = 1;
                    break;
                }
            }
            *((unsigned int *)(iter * 96 + a1 + 0x44)) = inotify_add_watch(a0, *((long long *)(a1 + iter * 96)), v9, *((long long *)(a1 + iter * 96)));
            if (*((int *)(a1 + iter * 96 + 0x44)) < 0)
            {
                if (*((int *)(a1 + iter * 96 + 56)) != 0xffffffff)
                    v1 = 1;
                if (*(__errno_location()) == 28 || *(__errno_location()) == 12)
                {
                    v3 = 1;
                    error(0, 0, gettext("inotify resources exhausted"));
                    break;
                }
                else
                {
                    err = __errno_location();
                    if (*(err) != *((int *)(a1 + iter * 96 + 60)))
                    {
                        sub_40e8ca(4, *((long long *)(a1 + iter * 96)), iter * 96);
                        v39 = gettext("cannot watch %s");
                        v40 = __errno_location();
                        error(0, *(v40), v39);
                    }
                }
            }
            else
            {
                if (!sub_40c73f(v21, iter * 96 + a1, iter * 96 + a1))
                    sub_40ffa4(); /* do not return */
                v0 = 1;
            }
        }
    }
    if (v3 || v2 || *((int *)&g_418010) == 2 && v1)
        return v35 - *((long long *)(40 + v64));
    if (*((int *)&g_418010) == 2 && v0 != 1)
        exit(1); /* do not return */
    v13 = a1 + a2 * 96 - 96;
    iter = 0;
    while (1)
    {
        if (iter < a2)
        {
            if (*((char *)(a1 + iter * 96 + 52)) != 1)
            {
                if (*((int *)&g_418010) == 1)
                {
                    sub_405a69(a1 + iter * 96, 0);
                }
                else if (*((int *)(a1 + iter * 96 + 56)) != 0xffffffff && !(v45 = (unsigned int)(int)(long long)stat(*((long long *)(a1 + iter * 96)), &v30), (int)(long long)stat(*((long long *)(a1 + iter * 96)), &v30) || *((long long *)(a1 + iter * 96 + 32)) == *((unsigned long long *)(void*)&v30) && *((long long *)(a1 + iter * 96 + 40)) == v30.st_ino))
                {
                    sub_40e8ca(4, sub_4043ee(a1 + iter * 96));
                    v47 = gettext("%s was replaced");
                    err1 = __errno_location();
                    error(0, *(err1), v47);
                    break;
                }
                sub_4070be(iter * 96 + a1, &v13);
            }
            iter += 1;
        }
        else
        {
            v14 += 0x11;
            v15 = sub_40f96c(v14);
            while (1)
            {
                while (1)
                {
                    if (*((int *)&g_418010) == 1 && g_41812c != 1 && !sub_40b071(v21))
                        error(1, 0, gettext("no files remaining"));
                    if (v17 <= v16)
                    {
                        do
                        {
                            v10 = 0xffffffff;
                            if (g_418134)
                            {
                                if (v4)
                                    exit(0); /* do not return */
                                if (kill(g_418134, 0) && *(__errno_location()) != 1)
                                    v49 = 1;
                                else
                                    v49 = 0;
                                v4 = v49;
                                v4 &= 1;
                                if (v4 || !(CmpF(0, v50) & 1))
                                {
                                    v10 = 0;
                                }
                                else if ((((char)(CmpF(0x4140624d00000000, v50) & 69) | (char)((CmpF(0x4140624d00000000, v50) & 69) >> 6)) & 1) != 1)
                                {
                                    v24 = MulV(0x408f400000000000, v50);
                                    v10 = v24;
                                    v10 += !((CmpF(v24, v10) & 69 | (CmpF(v24, v10) & 69) >> 6) & 1);
                                }
                            }
                            v30 = a0;
                            v31 = 1;
                            v32 = 1;
                            v34 = 0;
                            v33 = v34;
                            j = poll(&v30, g_41812f + 1, v10, g_41812f + 1);
                        } while (!j);
                        if (j < 0)
                        {
                            v58 = gettext("error waiting for inotify and output events");
                            v59 = __errno_location();
                            error(1, *(v59), v58);
                        }
                        if (v34)
                            sub_40431c(); /* do not return */
                        v17 = sub_40ed34(a0, v15, v14, v15);
                        v16 = 0;
                        if ((!v17 || (v8 = v8, v17 == 0xffffffffffffffff && !(v8 = v8, *(__errno_location()) != 22))) && !(v8 -= 1, !v8))
                        {
                            v17 = 0;
                            v14 *= 2;
                            v15 = sub_40f9d6(v15, v14, v14);
                            continue;
                        }
                        else
                        {
                            if (!v17 || !(v8 = v8, v17 != 0xffffffffffffffff))
                            {
                                v60 = gettext("error reading inotify event");
                                v61 = __errno_location();
                                error(1, *(v61), v60);
                            }
                        }
                    }
                    v25 = v16 + v15;
                    v26 = v25;
                    v16 = v16 + *((int *)((char *)&v26->field_8 + 4)) + 16;
                    if ((short)*((int *)&v26->padding_0[4]) & 0x400 && !*((int *)((char *)&v26->field_8 + 4)))
                    {
                        for (iter = 0; iter < a2; iter += 1)
                        {
                            if (*((int *)&v26->padding_0[0]) == *((int *)(a1 + iter * 96 + 72)))
                            {
                                error(0, 0, gettext("directory containing watched file was removed"));
                                return v35 - *((long long *)(40 + v64));
                            }
                        }
                    }
                    if (*((int *)((char *)&v26->field_8 + 4)))
                    {
                        for (k = 0; k < a2 && (*((int *)(a1 + k * 96 + 72)) != *((int *)&v26->padding_0[0]) || strcmp(&v26->field_10, *((long long *)(a1 + k * 96)) + *((long long *)(a1 + k * 96 + 80)))); k += 1);
                        if (k == a2)
                            continue;
                        v19 = a1 + k * 96;
                        v11 = 0xffffffff;
                        v6 = *((int *)&v26->padding_0[4]) & 0x200;
                        if (v6 != 1)
                        {
                            v11 = inotify_add_watch(a0, *((long long *)(a1 + k * 96)), v9, *((long long *)(a1 + k * 96)));
                            break;
                        }
                    }
                    else
                    {
                        v29 = *((int *)&v26->padding_0[0]);
                        v19 = sub_40b349(v21, &v28, &v28);
LABEL_408453:
                        if (v19)
                        {
                            if ((short)*((int *)&v26->padding_0[4]) & 3588)
                            {
                                if ((short)*((int *)&v26->padding_0[4]) & 0x400)
                                {
                                    v56 = (int)v19[0x44];
                                    inotify_rm_watch(a0, v56 & 0xffffffff, v56);
                                    sub_40c7af(v21, v19, v19);
                                }
                                sub_405a69(v19, 0);
                            }
                            else
                            {
                                sub_4070be(v19, &v13);
                                continue;
                            }
                        }
                    }
                }
                if (v6 != 1 && v11 < 0)
                {
                    if (*(__errno_location()) == 28 || *(__errno_location()) == 12)
                    {
                        error(0, 0, gettext("inotify resources exhausted"));
                        break;
                    }
                    else
                    {
                        sub_40e8ca(4, *((long long *)(a1 + k * 96)), k * 96);
                        v51 = gettext("cannot watch %s");
                        err2 = __errno_location();
                        error(0, *(err2), v51);
                    }
                }
                if (v6 != 1 && ((int)v19[0x44] < 0 || v11 != (int)v19[0x44]))
                    v53 = 1;
                else
                    v53 = 0;
                v7 = v53;
                v7 &= 1;
                if (v7)
                {
                    if ((int)v19[0x44] >= 0)
                    {
                        v54 = (int)v19[0x44];
                        inotify_rm_watch(a0, v54 & 0xffffffff, v54);
                        sub_40c7af(v21, v19, v19);
                    }
                    *((unsigned int *)&v19[0x44]) = v11;
                    if (v11 == 0xffffffff)
                        continue;
                    v27 = sub_40c7af(v21, v19, v19);
                    if (v27 && v27 != v19)
                    {
                        if (*((int *)&g_418010) == 1)
                            sub_405a69(v27, 0);
                        *((unsigned int *)&v27[0x44]) = 0xffffffff;
                        v55 = sub_4043ee(v27);
                        sub_4044d5((int)v27[56], v55);
                    }
                    if (!sub_40c73f(v21, v19, v19))
                        sub_40ffa4(); /* do not return */
                }
                if (*((int *)&g_418010) == 1)
                {
                    sub_405a69(v19, 0);
                    goto LABEL_408453;
                }
            }
        }
    }
    return v35 - *((long long *)(40 + v64));
}



// Function: tail_bytes @ 0x8520
typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

extern char g_418130;
extern char g_418139;

unsigned int tail_bytes(long long a0, unsigned int a1, unsigned long a2, void* a3)
{
    char *v8;  // rax
    int *err;  // rax
    unsigned long v10;  // rax
    unsigned long long v11;  // rax
    unsigned long v0;  // [bp-0xf0]
    unsigned int v1;  // [bp-0xd4]
    unsigned long v2;  // [bp-0xd0]
    unsigned long v3;  // [bp-0xc8]
    unsigned long v4;  // [bp-0xc0]
    stat v5;  // [bp-0xb8]

    v0 = a2;
    if (fstat(a1, &v5))
    {
        sub_40e8ca(4, a0);
        v8 = gettext("cannot fstat %s");
        err = __errno_location();
        error(0, *(err), v8);
        return 0;
    }
    if (g_418130)
    {
        if (g_418139 != 1 && v0 >= 0 && (((unsigned short)v5.st_nlink & 0xf000) == 0x8000 && sub_404804(a1, v0, 1, a0) >= 0 || lseek(a1, v0, 1) != 0xffffffffffffffff))
        {
            *((unsigned long *)a3) = *((long long *)a3) + v0;
        }
        else
        {
            v1 = sub_4055ae(a0, a1, v0, a3);
            if (v1)
                return v1 >> 31;
        }
        v0 = 0xffffffffffffffff;
    }
    else
    {
        v2 = 0xffffffffffffffff;
        v3 = 0xffffffffffffffff;
        if (g_418139 != 1 && v0 >= 0)
        {
            if ((char)sub_402d62(&v5))
            {
                v2 = v5.st_atime;
            }
            else
            {
                v3 = lseek(a1, -(v0), 2);
                if (v3 != 0xffffffffffffffff)
                    v2 = v0 + v3;
            }
        }
        if (v5.st_mtime > 0 && v5.st_mtime <= 0x2000000000000000)
            v10 = v5.st_mtime;
        else
            v10 = 0x200;
        if (v10 >= v2)
            return sub_405286(a0, a1, v0, a3);
        if (v3 == 0xffffffffffffffff)
            v3 = sub_404804(a1, 0, 1, a0);
        if (v3 < v2)
        {
            v4 = v2 - v3;
            if (v0 < v4)
            {
                v3 = v2 - v0;
                sub_404804(a1, v3, 0, a0);
            }
        }
        *((unsigned long *)a3) = v3;
    }
    v11 = sub_404649(0, a0, a1, v0);
    *((unsigned long long *)a3) = v11 + *((long long *)a3);
    return 1;
}



// Function: tail_lines @ 0x88c7
typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

extern char g_418130;
extern char g_418139;

unsigned int tail_lines(long long a0, unsigned int a1, unsigned long long a2, void* a3)
{
    char *v6;  // rax
    int *err;  // rax
    unsigned long long v8;  // rax
    unsigned int v0;  // [bp-0xcc]
    unsigned long v1;  // [bp-0xc8]
    unsigned long v2;  // [bp-0xc0]
    stat v3;  // [bp-0xb8]

    if (fstat(a1, &v3))
    {
        sub_40e8ca(4, a0);
        v6 = gettext("cannot fstat %s");
        err = __errno_location();
        error(0, *(err), v6);
        return 0;
    }
    if (g_418130)
    {
        v0 = sub_405741(a0, a1, a2, a3);
        if (v0)
            return v0 >> 31;
        v8 = sub_404649(0, a0, a1, 0xffffffffffffffff);
        *((unsigned long long *)a3) = v8 + *((long long *)a3);
    }
    else
    {
        v1 = 0xffffffffffffffff;
        if (g_418139 != 1 && ((unsigned short)v3.st_nlink & 0xf000) == 0x8000 && (v1 = (unsigned long)lseek(a1, 0, 1), v1 != 0xffffffffffffffff && (v2 = (unsigned long)lseek(a1, 0, 2), v2 > v1)))
        {
            *((unsigned long *)a3) = v2;
            if (v2 && (char)sub_4049bd(a0, a1, a2, v1, v2, a3) != 1)
                return 0;
        }
        else
        {
            if (v1 == 0xffffffffffffffff)
                return sub_404df7(a0, a1, a2, a3);
            sub_404804(a1, v1, 0, a0);
            return sub_404df7(a0, a1, a2, a3);
        }
    }
    return 1;
}



// Function: tail @ 0x8b67
extern char g_41812d;

unsigned int tail(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    *(a3) = 0;
    return (!g_41812d ? sub_408520(a0, a1, a2, a3) : sub_4088c7(a0, a1, a2, a3));
}



// Function: tail_file @ 0x8bca
extern char g_41812c;
extern char g_41812e;
extern char g_418131;
extern char g_418138;

long long tail_file(void* idx, unsigned long long a1)
{
    char v6;  // al
    char *v19;  // rax
    int *err;  // rax
    char *v8;  // rax
    int *err1;  // rax
    char *v13;  // rax
    int *err2;  // rax
    char v0;  // [bp-0xc6]
    char v1;  // [bp-0xc5]
    unsigned int v2;  // [bp-0xc4]
    unsigned int v3;  // [bp-0xc0]
    int v4;  // [bp-0xb8]

    v1 = !strcmp(*((long long *)idx), "-");
    if (v1)
    {
        g_418138 = 1;
        v2 = 0;
        sub_40ffef(0, 0);
    }
    else
    {
        v2 = sub_40af09(*((long long *)idx), 0);
    }
    if (g_41812c != 1 || v2 != 0xffffffff)
        v6 = 1;
    else
        v6 = 0;
    *((char *)&idx[54]) = v6 & 1;
    if (v2 == 0xffffffff)
    {
        if (g_41812e)
        {
            *((unsigned int *)&idx[56]) = 0xffffffff;
            *((int *)&idx[60]) = *(__errno_location());
            *((char *)&idx[52]) = !g_41812c;
            *((unsigned long long *)&idx[40]) = 0;
            *((unsigned long long *)&idx[32]) = 0;
        }
        sub_40e8ca(4, sub_4043ee(idx));
        v8 = gettext("cannot open %s for reading");
        err1 = __errno_location();
        error(0, *(err1), v8);
        v0 = 0;
        return v0;
    }
    else
    {
        if (g_418131)
            sub_404559(sub_4043ee(idx));
        v0 = sub_408b67(sub_4043ee(idx), v2, a1, &v3);
        if (g_41812e)
        {
            *((unsigned int *)&idx[60]) = v0 - 1;
            if (fstat(v2, &v4) < 0)
            {
                v0 = 0;
                *((int *)&idx[60]) = *(__errno_location());
                sub_40e8ca(4, sub_4043ee(idx));
                v13 = gettext("error reading %s");
                err2 = __errno_location();
                error(0, *(err2), v13);
            }
            else
            {
                if (((unsigned short)*((unsigned int *)(&v4 + 24)) & 0xf000) != 0x8000 && ((unsigned short)*((unsigned int *)(&v4 + 24)) & 0xf000) != 0x1000 && ((unsigned short)*((unsigned int *)(&v4 + 24)) & 0xf000) != 0xc000 && ((unsigned short)*((unsigned int *)(&v4 + 24)) & 0xf000) != 0x2000)
                {
                    v0 = 0;
                    *((unsigned int *)&idx[60]) = 0xffffffff;
                    *((char *)&idx[54]) = 0;
                    *((char *)&idx[52]) = !g_41812c;
                    if ((char)idx[52])
                        gettext("; giving up on this name");
                    sub_40ea5c(0, 3, sub_4043ee(idx));
                    error(0, 0, gettext("%s: cannot follow end of this type of file%s"));
                }
            }
            if (v0 == 1)
            {
                sub_404435(idx, v2, v3, &v4, (!v1 ? 1 : 0xffffffff));
                *((char *)&idx[53]) = sub_405938(v2, sub_4043ee(idx));
                return v0;
            }
            *((char *)&idx[52]) = !g_41812c;
            sub_4044d5(v2, sub_4043ee(idx));
            *((unsigned int *)&idx[56]) = 0xffffffff;
            return v0;
        }
        else if (v1 == 1)
        {
            return v0;
        }
        else if (close(v2))
        {
            sub_40e8ca(4, sub_4043ee(idx));
            v19 = gettext("error reading %s");
            err = __errno_location();
            error(0, *(err), v19);
            v0 = 0;
            return v0;
        }
        else
        {
            return v0;
        }
    }
}



// Function: parse_obsolete_option @ 0x90f5
typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_1 {
    char padding_0[8];
    struct struct_0 *field_8;
    struct struct_2 *field_10;
} struct_1;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern char g_41812d;
extern char g_41812e;
extern char g_418130;

unsigned int parse_obsolete_option(int a0, struct_1 *a1, unsigned long long *a2)
{
    char v11;  // al
    struct_0 *v12;  // rax
    unsigned int v13;  // eax
    unsigned int v14;  // eax
    int *err;  // rax
    char v0;  // [bp-0x3d]
    char v1;  // [bp-0x3c]
    char v2;  // [bp-0x3b]
    char v3;  // [bp-0x3a]
    char v4;  // [bp-0x39]
    unsigned int v5;  // [bp-0x38]
    unsigned int v6;  // [bp-0x34]
    struct_0 *iter;  // [bp-0x30], Other Possible Types: unsigned long
    unsigned long v8;  // [bp-0x28]
    unsigned long v9;  // [bp-0x20]

    v5 = 10;
    v1 = 1;
    v2 = 0;
    if (a0 != 2 && (a0 != 3 || a1->field_10->field_0 == 45 && a1->field_10->field_1) && (a0 <= 2 || a0 > 4 || strcmp(a1->field_10, "--")))
        return 0;
    v6 = sub_40cc1c();
    v3 = v6 <= 200111;
    if (v3 || v6 > 200808)
        v11 = 1;
    else
        v11 = 0;
    v4 = v11;
    v4 &= 1;
    iter = a1->field_8;
    v12 = iter;
    iter = &v12->field_1;
    v13 = v12->field_0;
    switch (v13)
    {
    case 43:
        if (v4 != 1)
            return 0;
        v0 = 1;
        break;
    case 45:
        if (v3 != 1 && !*((char *)(iter + (*((char *)iter) == 99))))
            return 0;
        v0 = 0;
        break;
    default:
        return 0;
    }
    for (v8 = iter; *((char *)iter) - 48 <= 9; iter += 1);
    v9 = iter;
    v14 = *((char *)iter);
    switch (v14)
    {
    case 98:
        v5 *= 0x200;
    case 99:
        v1 = 0;
        goto LABEL_409295;
    case 108:
LABEL_409295:
        iter += 1;
        break;
    default:
        goto LABEL_40929b;
    }
LABEL_40929b:
    if (*((char *)iter) == 0x66)
    {
        v2 = 1;
        iter += 1;
    }
    if (*((char *)iter))
        return 0;
    if (v8 == v9)
    {
        *(a2) = v5;
    }
    else if (sub_410410(v8, 0, 10, a2, "b") & 0xfffffffd)
    {
        sub_40ed11(a1->field_8);
        gettext("invalid number");
        err = __errno_location();
        error(1, *(err), "%s: %s");
    }
    g_418130 = v0;
    g_41812d = v1;
    g_41812e = v2;
    return 1;
}



// Function: parse_options @ 0x9381
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern long long g_4121c3;
extern unsigned int g_412210[4];
extern long long g_417900;
extern option g_417920;
extern unsigned int g_418010;
extern unsigned long long g_418018;
extern long long g_418030;
extern long long g_418038;
extern long long stdout;
extern unsigned long long optarg;
extern char g_41812c;
extern char g_41812d;
extern char g_41812e;
extern char g_418130;
extern char g_418132;
extern int g_418134;
extern char g_418139;
extern char g_41813a;

void parse_options(int a0, char **a1, unsigned long long *a2, unsigned int *a3, unsigned long long *a4)
{
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    unsigned long v5;  // rax
    unsigned long v7;  // cc_ndep
    unsigned long long v8;  // 4135
    unsigned int v0;  // [bp-0x2c]
    char v1;  // [bp-0x28]

    while (1)
    {
        v0 = getopt_long(a0, a1, "c:n:fFqs:vz0123456789", &g_417920.name, NULL);
        if (v0 == 0xffffffff)
            break;
        if (v0 > 133)
            goto LABEL_40975f;
        if (v0 < 48)
        {
            if (v0 != 0xffffff7d)
            {
                if (v0 != 0xffffff7e)
                    goto LABEL_40975f;
                sub_404115(0); /* do not return */
            }
            sub_40f78c(stdout, "tail", "GNU coreutils", g_418030, "Paul Rubin", "David MacKenzie", "Ian Lance Taylor", "Jim Meyering", 0);
            exit(0); /* do not return */
        }
        switch (v0)
        {
        case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
            error(1, 0, gettext("option used in invalid context -- %c"));
        case 70:
            g_41812e = 1;
            g_418010 = 1;
            g_41812c = 1;
            break;
        case 99: case 110:
            g_41812d = v0 == 110;
            if (*((char *)optarg) == 43)
            {
                g_418130 = 1;
            }
            else if (*((char *)optarg) == 45)
            {
                optarg = optarg + 1;
            }
            v3 = (!g_41812d ? gettext("invalid number of bytes") : gettext("invalid number of lines"));
            *(a2) = sub_410156(optarg, 0, -0x1, "bkKmMGTPEZY0", v3, 0);
            break;
        case 102: case 132:
            g_41812e = 1;
            if (!optarg)
            {
                g_418010 = 2;
                break;
            }
            else
            {
                g_418010 = g_412210[sub_40a546("--follow", optarg, &g_417900, &g_412210[0], 4, g_418038, 1)];
                break;
            }
        case 113:
            *(a3) = 2;
            break;
        case 115:
            if ((char)sub_410261(optarg, 0, &v1, sub_40a745) ^ 1 || !(v8 = (unsigned long long)_ccall(3, 0, (unsigned long long)(CmpF(v1, 0) & 69), 0, v7), ((char)v8 & 1) == 1))
            {
                sub_40ed11(optarg);
                error(1, 0, gettext("invalid number of seconds: %s"));
            }
            *(a4) = v1;
            break;
        case 118:
            *(a3) = 1;
            break;
        case 122:
            g_418132 = 0;
            break;
        case 128:
            g_41812c = 1;
            break;
        case 129:
            v4 = gettext("invalid maximum number of unchanged stats between opens");
            g_418018 = sub_410156(optarg, 0, -0x1, &g_4121c3, v4, 0);
            break;
        case 130:
            v5 = gettext("invalid PID");
            g_418134 = sub_410156(optarg, 0, 0x7fffffff, &g_4121c3, v5, 0);
            break;
        case 131:
            g_418139 = 1;
            break;
        case 133:
            g_41813a = 1;
            break;
        default:
LABEL_40975f:
            sub_404115(1); /* do not return */
        }
    }
    if (g_41812c)
    {
        if (g_41812e != 1)
        {
            g_41812c = 0;
            error(0, 0, gettext("warning: --retry ignored; --retry is useful only when following"));
        }
        else if (g_418010 == 2)
        {
            error(0, 0, gettext("warning: --retry only effective for the initial open"));
        }
    }
    if (g_418134 && g_41812e != 1)
    {
        error(0, 0, gettext("warning: PID ignored; --pid=PID is useful only when following"));
        return;
    }
    if (!g_418134)
    {
        return;
    }
    else if (!kill(g_418134, 0))
    {
        return;
    }
    else if (*(__errno_location()) == 38)
    {
        error(0, 0, gettext("warning: --pid=PID is not supported on this system"));
        g_418134 = 0;
        return;
    }
    else
    {
        return;
    }
}



// Function: ignore_fifo_and_pipe @ 0x98cb
unsigned long long ignore_fifo_and_pipe(unsigned long a0, unsigned long long a1)
{
    char v4;  // al
    char v0;  // [bp-0x19]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long i;  // [bp-0x10]

    v1 = 0;
    for (i = 0; i < a1; i += 1)
    {
        if (!strcmp(*((long long *)(a0 + i * 96)), "-") && *((char *)(a0 + i * 96 + 52)) != 1 && *((int *)(a0 + i * 96 + 56)) >= 0 && ((short)*((int *)(a0 + i * 96 + 48)) & 0xf000) == 0x1000)
            v4 = 1;
        else
            v4 = 0;
        v0 = v4;
        v0 &= 1;
        if (v0)
        {
            *((unsigned int *)(a0 + i * 96 + 56)) = 0xffffffff;
            *((char *)(a0 + i * 96 + 52)) = 1;
        }
        else
        {
            v1 += 1;
        }
    }
    return v1;
}



// Function: main @ 0x9a14
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

typedef struct stat {
    unsigned int st_mode;
    unsigned long st_ino;
    uint64_t st_dev;
    unsigned int st_nlink;
    unsigned int st_uid;
    unsigned int st_gid;
    long st_size;
    long st_atime;
    long st_mtime;
    long st_ctime;
} stat;

typedef struct FILE {
} FILE;

extern char g_4121c3;
extern unsigned int g_418010;
extern unsigned long long g_418028;
extern FILE *stdout;
extern char optind;
extern char g_41812d;
extern char g_41812e;
extern char g_41812f;
extern char g_418130;
extern char g_418131;
extern char g_418132;
extern unsigned int g_418134;
extern char g_418138;
extern char g_41813a;

unsigned int main(int a0, long long *a1)
{
    unsigned int v18;  // eax
    int *err;  // rax
    int *err1;  // rax
    char v19;  // al
    char *v22;  // rax
    int *err2;  // rax
    char *v24;  // rax
    int *v25;  // rax
    char *v26;  // rax
    long long *v0;  // [bp-0x118]
    int v1;  // [bp-0x10c]
    char v2;  // [bp-0xfc]
    char v3;  // [bp-0xfb]
    char v4;  // [bp-0xfa]
    char v5;  // [bp-0xf9]
    unsigned int v6;  // [bp-0xf8]
    unsigned int v7;  // [bp-0xf4]
    unsigned long long v8;  // [bp-0xf0]
    unsigned long long v9;  // [bp-0xe8]
    long long v10;  // [bp-0xe0]
    unsigned long long v11;  // [bp-0xd8]
    unsigned long long v12;  // [bp-0xd0]
    unsigned long long iter;  // [bp-0xc8]
    struct_0 *v14;  // [bp-0xc0]
    stat v15;  // [bp-0xb8]

    v1 = a0;
    v0 = a1;
    v6 = 0;
    v2 = 1;
    v8 = 10;
    v9 = 0x3ff0000000000000;
    sub_40ccd0(*(v0));
    setlocale(6, &g_4121c3);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_411700(sub_40a84b);
    g_418138 = 0;
    g_41812d = 1;
    g_418131 = 0;
    g_418130 = g_418131;
    g_41812e = g_418130;
    g_418132 = 10;
    v4 = sub_4090f5(v1, v0, &v8);
    v1 -= v4;
    v0 = &v0[v4];
    sub_409381(v1, v0, &v8, &v6, &v9);
    if (g_418130 && v8)
        v8 -= 1;
    if (v1 > *((int *)&optind))
    {
        v11 = v1 - *((int *)&optind);
        v12 = &v0[*((int *)&optind)];
    }
    else
    {
        v11 = 1;
        v12 = &g_418028;
    }
    v3 = 0;
    for (iter = 0; iter < v11; iter += 1)
    {
        if (!strcmp(*((long long *)(v12 + iter * 8)), "-"))
            v3 = 1;
    }
    if (v3 && g_418010 == 1)
    {
        sub_40e8ca(4, "-");
        error(1, 0, gettext("cannot follow %s by name"));
    }
    if (g_41812e && v3)
    {
        if (!g_418134 && g_418010 == 2 && v11 == 1 && !(v18 = (unsigned int)(int)(long long)fstat(0, &v15), (int)(long long)fstat(0, &v15) || ((unsigned short)v15.st_nlink & 0xf000) == 0x8000))
            v19 = 1;
        else
            v19 = 0;
        v5 = v19;
        v5 &= 1;
        if (v5 != 1 && isatty(0))
            error(0, 0, gettext("warning: following standard input indefinitely is ineffective"));
    }
    if (!v8 && g_41812e != 1 && g_418130 != 1)
        return 0;
    v14 = sub_40fae5(v11, 96);
    for (iter = 0; iter < v11; iter += 1)
    {
        *((long long *)&v14[32 * iter]) = *((long long *)(iter * 8 + v12));
    }
    if (v6 == 1 || !v6 && v11 > 1)
        g_418131 = 1;
    sub_40ffef(1, 0);
    for (iter = 0; iter < v11; iter += 1)
    {
        v2 = (char)sub_408bca(&v14[32 * iter], v8) & v2;
    }
    if (g_41812e && sub_4098cb(v14, v11))
    {
        if (fstat(1, &v15) < 0)
        {
            v22 = gettext("standard output");
            err2 = __errno_location();
            error(1, *(err2), v22);
        }
        g_41812f = (v15.st_nlink & 0xf000) == 0x1000;
        if (g_41813a != 1 && ((char)sub_406fc5(v14, v11) || (char)sub_406d57(v14, v11) || (char)sub_406dd3(v14, v11) != 1 || (char)sub_406e52(v14, v11) || (char)sub_406f11(v14, v11) || v2 != 1 && g_418010 == 2))
            g_41813a = 1;
        if (g_41813a != 1)
        {
            v7 = inotify_init();
            if (v7 >= 0)
            {
                if (fflush_unlocked(stdout))
                {
                    v24 = gettext("write error");
                    v25 = __errno_location();
                    error(1, *(v25), v24);
                }
                sub_407379(v7, v14, v11, &v10);
                sub_40bc7e(v10);
                close(v7);
                *(__errno_location()) = 0;
            }
            v26 = gettext("inotify cannot be used, reverting to polling");
            err = __errno_location();
            error(0, *(err), v26);
        }
        g_41813a = 1;
        sub_4062f3(v14, v11);
    }
    if (g_418138 && close(0) < 0)
    {
        err1 = __errno_location();
        error(1, *(err1), "-");
    }
    return v2 ^ 1;
}



// Function: hash_get_n_buckets @ 0xb045
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0xb05b
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0xb071
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0xb087
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_get_max_bucket_length(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *iter;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v1 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v3 = 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                v3 += 1;
            }
            if (v3 > v1)
                v1 = v3;
        }
    }
    return v1;
}



// Function: hash_table_ok @ 0xb109
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned int hash_table_ok(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long node;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v1 = 0;
    node = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v1 += 1;
            node += 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                node += 1;
            }
        }
    }
    if (v1 == a0[3] && node == a0[4])
        return 1;
    return 0;
}



// Function: hash_print_statistics @ 0xb1a7
typedef struct FILE {
} FILE;

int hash_print_statistics(void* a0, FILE *ptr)
{
    uint128_t v5;  // xmm0
    long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = sub_40b071(a0);
    v1 = sub_40b045(a0);
    v2 = sub_40b05b(a0);
    v3 = sub_40b087(a0);
    fprintf(ptr, "# entries:         %lu\n", v0);
    fprintf(ptr, "# buckets:         %lu\n", v1);
    v5 = (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1);
    fprintf(ptr, "# buckets used:    %lu (%.2f%%)\n", v2, (unsigned long long)(DivV(MulV(0x4059000000000000, (v2 < 0 ? AddV(v2 >> 1 | (unsigned int)v2 & 1, v2 >> 1 | (unsigned int)v2 & 1) : v2)), (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1))));
    return fprintf(ptr, "max bucket length: %lu\n", v3);
}



// Function: safe_hasher @ 0xb2ee
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

long long safe_hasher(struct_0 *a0, unsigned long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0->field_30(a1, a0->field_10, a0->field_10, a0->field_30);
    if (v0 < a0->field_10)
        return v0 * 16 + a0->field_0;
    abort(); /* do not return */
}



// Function: hash_lookup @ 0xb349
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_lookup(struct_0 *a0, unsigned long long a1)
{
    struct_0 *iter;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v1 = sub_40b2ee(a0, a1);
    if (!v1->field_0)
        return 0;
    iter = v1;
    while (1)
    {
        if (!iter)
            return 0;
        if (a1 == iter->field_0 || (char)a0[1].field_0(a1, iter->field_0, iter->field_0, a0[1].field_0))
            break;
        iter = (struct_0 *)iter->padding_8;
    }
    return iter->field_0;
}



// Function: hash_get_first @ 0xb3e0
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[24];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_get_first(struct_0 *a0)
{
    unsigned long long *v0;  // [bp-0x10]

    if (!a0->field_20)
        return 0;
    v0 = &a0->field_0->field_0;
    while (1)
    {
        if (v0 >= *((long long *)&a0->padding_8[0]))
            abort(); /* do not return */
        if (*(v0))
            return *(v0);
        v0 += 2;
    }
}



// Function: hash_get_next @ 0xb440
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_get_next(struct_0 *a0, unsigned int a1)
{
    struct_0 *v0;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = sub_40b2ee(a0, a1);
    iter = v0;
    while (1)
    {
        if (a1 == iter->field_0 && iter->padding_8)
            return *((long long *)iter->padding_8);
        iter = (struct_0 *)iter->padding_8;
        if (!iter)
        {
            while (1)
            {
                v0 = &v0->field_10;
                if (v0 >= a0->padding_8)
                {
                    return 0;
                }
                else if (v0->field_0)
                {
                    return v0->field_0;
                }
            }
        }
    }
}



// Function: hash_get_entries @ 0xb4de
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_get_entries(struct_0 **a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long idx;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    idx = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; idx = v1)
            {
                if (idx >= a2)
                    return idx;
                v1 = idx + 1;
                *((unsigned long long *)(idx * 8 + a1)) = iter->field_0;
                iter = iter->field_8;
            }
        }
    }
    return idx;
}



// Function: hash_do_for_each @ 0xb57e
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_do_for_each(struct_0 **a0, unsigned long long *a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; iter = iter->field_8)
            {
                if ((char)a1(iter->field_0, a2, a2, a1) != 1)
                    return v0;
                v0 += 1;
            }
        }
    }
    return v0;
}



// Function: hash_string @ 0xb616
unsigned long long hash_string(char *a0, unsigned long long a1)
{
    char *iter;  // [bp-0x20]
    char v1;  // [bp-0x11]
    unsigned long long v2;  // [bp-0x10]

    iter = a0;
    v2 = 0;
    while (1)
    {
        v1 = *(iter);
        if (!v1)
            break;
        v2 = (v1 + v2 * 31) % a1;
        iter += 1;
    }
    return v2;
}



// Function: is_prime @ 0xb670
long long is_prime(unsigned long long a0)
{
    unsigned long long iter;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    iter = 3;
    for (v1 = iter * iter; v1 < a0 && a0 % iter; iter += 1)
    {
        iter += 1;
        v1 += iter * 4;
    }
    return _INSERT(a0 % iter, 0, a0 % iter);
}



// Function: next_prime @ 0xb6df
unsigned long long next_prime(unsigned long long a0)
{
    unsigned long long i;  // [bp-0x10]

    i = a0;
    if (i <= 9)
        i = 10;
    for (i |= 1; i != 0xffffffffffffffff; i += 2)
    {
        if ((char)sub_40b670(i) == 1)
            return i;
    }
    return i;
}



// Function: hash_reset_tuning @ 0xb72a
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
    char field_10;
} struct_0;

struct_0 * hash_reset_tuning(struct_0 *idx)
{
    idx->field_0 = 0;
    idx->field_4 = 0x3f800000;
    idx->field_8 = 1061997773;
    idx->field_c = 1068826100;
    idx->field_10 = 0;
    return idx;
}



// Function: raw_hasher @ 0xb780
long long raw_hasher(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_410957(a0, 3);
    return v0 % a1;
}



// Function: raw_comparator @ 0xb7bb
long long raw_comparator(unsigned long a0, unsigned long a1)
{
    return _INSERT(a0, 0, a0 == a1);
}



// Function: check_tuning @ 0xb7d8
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

extern unsigned int g_4135f0[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int v3;  // ymm1
    unsigned int v4;  // ymm1
    unsigned int v0;  // [bp-0x14]
    unsigned int *v1;  // [bp-0x10]

    v1 = &idx->field_28->field_0;
    if (v1 == &g_4135f0[0])
        return 1;
    v0 = 1036831949;
    if (((CmpF(v1[2], v0) & 69 | (char)((CmpF(v1[2], v0) & 69) >> 6)) & 1) != 1 && ((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69 | (char)((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69) >> 6)) & 1) != 1 && ((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69 | (char)((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69) >> 6)) & 1) != 1 && !(CmpF(*(v1), 0) & 1))
    {
        v3 = AddV(*(v1), v0);
        if (((CmpF(v1[1], v3) & 69 | (char)((CmpF(v1[1], v3) & 69) >> 6)) & 1) != 1 && !(CmpF(0x3f800000, v1[1]) & 1))
        {
            v4 = AddV(*(v1), v0);
            if (((CmpF(v1[2], v4) & 69 | (char)((CmpF(v1[2], v4) & 69) >> 6)) & 1) != 1)
                return 1;
        }
    }
    idx->field_28 = &g_4135f0[0];
    return 0;
}



// Function: compute_bucket_size @ 0xb8e5
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long compute_bucket_size(unsigned long a0, struct_0 *a1)
{
    char v3;  // al
    unsigned long v0;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xc]

    v0 = a0;
    if (a1->field_10 != 1)
    {
        v1 = DivV((v0 < 0 ? AddV((unsigned int)(v0 >> 1 | (unsigned int)v0 & 1), (unsigned int)(v0 >> 1 | (unsigned int)v0 & 1)) : (unsigned int)v0), a1->field_8);
        if (!(CmpF(v1, 0x5f800000) & 1))
            return 0;
        if (CmpF(v1, 0x5f000000) & 1)
        {
            v0 = v1;
        }
        else
        {
            v0 = (unsigned int)(SubV(v1, 0x5f000000));
            v0 ^= 0x8000000000000000;
        }
    }
    v0 = sub_40b6df(v0);
    v3 = 0;
    if (v0 & 0xe000000000000000)
        v3 = 1;
    if (v0 * 8 < 0)
        v3 = 1;
    if (!(v3 & 1))
        return v0;
    return 0;
}



// Function: hash_initialize @ 0xb9fe
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

extern struct_0 g_4135f0;

struct_0 * hash_initialize(unsigned int a0, struct_0 *a1, unsigned long long a2, unsigned long long a3, unsigned long a4)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    struct_0 *v2;  // [bp-0x28]
    struct_0 *ptr;  // [bp-0x10]

    v2 = a1;
    v1 = a2;
    v0 = a3;
    if (!v1)
        v1 = sub_40b780;
    if (!v0)
        v0 = sub_40b7bb;
    ptr = malloc(80);
    if (!ptr)
        return NULL;
    if (!v2)
        v2 = &g_4135f0.padding_0[0];
    ptr->field_28 = v2;
    if ((char)sub_40b7d8(ptr) == 1)
    {
        *((unsigned long long *)&ptr->field_10) = sub_40b8e5(a0, v2);
        if (*((long long *)&ptr->field_10))
        {
            ptr->field_0 = calloc(*((long long *)&ptr->field_10), 16);
            if (ptr->field_0)
            {
                ptr->field_8 = ptr->field_0 + *((long long *)&ptr->field_10) * 16;
                ptr->field_18 = 0;
                ptr->field_20 = 0;
                ptr->field_30 = v1;
                ptr->field_38 = v0;
                ptr->field_40 = a4;
                ptr->field_48 = 0;
                return ptr;
            }
        }
    }
    free(ptr);
    return NULL;
}



// Function: hash_clear @ 0xbb73
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[24];
    struct struct_3 *field_40;
    struct struct_0 *field_48;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

struct_2 * hash_clear(struct_2 *idx)
{
    struct_1 *iter;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    struct_0 *v2;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx1 = iter->field_8; idx1; idx1 = v2)
            {
                if (idx->field_40)
                    idx->field_40(idx1->field_0);
                idx1->field_0 = 0;
                v2 = idx1->field_8;
                idx1->field_8 = idx->field_48;
                idx->field_48 = idx1;
            }
            if (idx->field_40)
                idx->field_40(iter->field_0);
            iter->field_0 = 0;
            iter->field_8 = NULL;
        }
    }
    idx->field_18 = 0;
    idx->field_20 = 0;
    return idx;
}



// Function: hash_free @ 0xbc7e
typedef struct struct_1 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
    char padding_8[24];
    unsigned long long field_20;
    char padding_28[24];
    struct struct_3 *field_40;
    struct struct_1 *field_48;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

void hash_free(struct_2 *ptr)
{
    struct_0 *node;  // [bp-0x20]
    struct_1 *iter;  // [bp-0x18], Other Possible Types: struct_0 *
    struct_0 *v2;  // [bp-0x10], Other Possible Types: struct_1 *

    if (ptr->field_40 && ptr->field_20)
    {
        for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
        {
            if (node->field_0)
            {
                for (iter = node; iter; iter = iter->field_8)
                {
                    ptr->field_40(iter->field_0);
                }
            }
        }
    }
    for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
    {
        for (iter = node->field_8; iter; iter = v2)
        {
            v2 = iter->field_8;
            free(iter);
        }
    }
    for (iter = ptr->field_48; iter; iter = v2)
    {
        v2 = iter->field_8;
        free(iter);
    }
    free(ptr->field_0);
    free(ptr);
    return;
}



// Function: allocate_entry @ 0xbdad
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_1;

struct_0 * allocate_entry(struct_1 *idx)
{
    struct_0 *ptr;  // [bp-0x10]

    if (!idx->field_48)
    {
        ptr = malloc(16);
        return ptr;
    }
    ptr = idx->field_48;
    idx->field_48 = ptr->field_8;
    return ptr;
}



// Function: free_entry @ 0xbdfc
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_2;

struct_2 * free_entry(struct_2 *idx, struct_0 *a1)
{
    a1->field_0 = 0;
    a1->field_8 = idx->field_48;
    idx->field_48 = a1;
    return idx;
}



// Function: hash_find_entry @ 0xbe36
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[56];
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_find_entry(struct_0 *a0, unsigned long long a1, struct_2 **a2, char a3)
{
    struct_2 *v7;  // rdx
    struct_2 *iter;  // [bp-0x38]
    struct_2 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    struct_2 *v5;  // [bp-0x10]

    index = sub_40b2ee(a0, a1);
    *(a2) = index;
    if (!index->field_0)
        return 0;
    if (a1 != index->field_0 && !(char)a0->field_38(a1, index->field_0, index->field_0, a0->field_38))
    {
        iter = index;
        while (1)
        {
            if (!iter->field_8)
                return 0;
            if (a1 == iter->field_8->field_0 || (char)a0->field_38(a1, iter->field_8->field_0, iter->field_8->field_0, a0->field_38))
                break;
            iter = iter->field_8;
        }
        v2 = iter->field_8->field_0;
        if (!a3)
            return v2;
        v3 = iter->field_8;
        iter->field_8 = v3->field_8;
        sub_40bdfc(a0, v3);
        return v2;
    }
    v4 = index->field_0;
    if (!a3)
        return v4;
    if (index->field_8)
    {
        v5 = index->field_8;
        v7 = v5->field_8;
        index->field_0 = v5->field_0;
        index->field_8 = v7;
        sub_40bdfc(a0, v5);
    }
    else
    {
        index->field_0 = 0;
    }
    return v4;
}



// Function: transfer_entries @ 0xbfcf
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
} struct_2;

unsigned int transfer_entries(struct_2 *index, struct_2 *idx, char a2)
{
    struct_1 *iter;  // [bp-0x38]
    struct_0 *idx2;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28], Other Possible Types: unsigned long long
    struct_0 *idx1;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]
    struct_0 *v5;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx2 = iter->field_8; idx2; idx2 = v5)
            {
                v2 = idx2->field_0;
                idx1 = sub_40b2ee(index, v2);
                v5 = idx2->field_8;
                if (idx1->field_0)
                {
                    idx2->field_8 = idx1->field_8;
                    idx1->field_8 = idx2;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                    sub_40bdfc(index, idx2);
                }
            }
            v2 = iter->field_0;
            iter->field_8 = NULL;
            if (!a2)
            {
                idx1 = sub_40b2ee(index, v2);
                if (idx1->field_0)
                {
                    v4 = sub_40bdad(index);
                    if (!v4)
                        return 0;
                    v4->field_0 = v2;
                    v4->field_8 = idx1->field_8;
                    idx1->field_8 = v4;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                }
                iter->field_0 = 0;
                idx->field_18 = idx->field_18 - 1;
            }
        }
    }
    return 1;
}



// Function: hash_rehash @ 0xc1a3
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
} struct_2;

unsigned int hash_rehash(struct_2 *index, unsigned int a1)
{
    unsigned long v0;  // [bp-0x78]
    struct_2 *idx;  // [bp-0x70]
    char v2;  // [bp-0x68], Other Possible Types: unsigned long
    unsigned long v3;  // [bp-0x60]
    unsigned long v4;  // [bp-0x58]
    unsigned long long v5;  // [bp-0x50]
    unsigned long long v6;  // [bp-0x48]
    unsigned long v7;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    unsigned long v10;  // [bp-0x28]
    unsigned long v11;  // [bp-0x20]

    v0 = sub_40b8e5(a1, *((long long *)&index[1].padding_8[0]));
    if (!v0)
    {
        return 0;
    }
    else if (v0 == *((long long *)&index->padding_8[8]))
    {
        return 1;
    }
    else
    {
        idx = &v2;
        v2 = calloc(v0, 16);
        if (!v2)
            return 0;
        v4 = v0;
        v3 = v2 + v0 * 16;
        v5 = 0;
        v6 = 0;
        v7 = *((long long *)&index[1].padding_8[0]);
        v8 = *((long long *)&index[1].padding_8[8]);
        v9 = index[1].field_18;
        v10 = index[2].field_0;
        v11 = *((long long *)&index[2].padding_8[0]);
        if ((char)sub_40bfcf(idx, index, 0))
        {
            free(index->field_0);
            index->field_0 = v2;
            *((unsigned long *)&index->padding_8[0]) = v3;
            *((unsigned long *)&index->padding_8[8]) = v4;
            index->field_18 = v5;
            *((unsigned long *)&index[2].padding_8[0]) = v11;
            return 1;
        }
        *((unsigned long *)&index[2].padding_8[0]) = v11;
        if ((char)sub_40bfcf(index, idx, 1) == 1 && (char)sub_40bfcf(index, idx, 0) == 1)
        {
            free(v2);
            return 0;
        }
        abort(); /* do not return */
    }
}



// Function: hash_insert_if_absent @ 0xc3ce
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned int hash_insert_if_absent(void* idx, unsigned long long a1, unsigned long long *a2)
{
    unsigned long v6;  // rax
    unsigned int v7;  // ymm0
    unsigned long v16;  // rax
    unsigned int v17;  // ymm0
    unsigned long v18;  // rax
    int v8;  // xmm2
    unsigned long v9;  // rax
    unsigned long v11;  // rax
    unsigned int v12;  // ymm0
    int v13;  // xmm2
    unsigned long v14;  // rax
    unsigned int v0;  // [bp-0x34]
    struct_0 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]

    if (!a1)
        abort(); /* do not return */
    v2 = sub_40be36(idx, a1, &index, 0);
    if (v2)
    {
        if (!a2)
            return 0;
        *(a2) = v2;
        return 0;
    }
    else
    {
        v6 = (long long)idx[24];
        v7 = (v6 < 0 ? (unsigned int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1))) : (int)v6);
        v8 = (int)*((int *)((long long)idx[40] + 8));
        v9 = (long long)idx[16];
        if (...)
        {
            sub_40b7d8(idx);
            v11 = (long long)idx[24];
            v12 = (v11 < 0 ? (unsigned int)(AddV((unsigned int)(v11 >> 1 | (unsigned int)v11 & 1), (unsigned int)(v11 >> 1 | (unsigned int)v11 & 1))) : (int)v11);
            v13 = (int)*((int *)((long long)idx[40] + 8));
            v14 = (long long)idx[16];
            if (...)
            {
                v3 = (long long)idx[40];
                if ((char)v3[1].field_0)
                {
                    v16 = (long long)idx[16];
                    v17 = MulV((v16 < 0 ? AddV((unsigned int)(v16 >> 1 | (unsigned int)v16 & 1), (unsigned int)(v16 >> 1 | (unsigned int)v16 & 1)) : (unsigned int)v16), *((int *)((char *)&v3->field_8 + 4)));
                }
                else
                {
                    v18 = (long long)idx[16];
                    v17 = MulV((int)v3->field_8, MulV(*((int *)((char *)&v3->field_8 + 4)), (v18 < 0 ? AddV((unsigned int)(v18 >> 1 | (unsigned int)v18 & 1), (unsigned int)(v18 >> 1 | (unsigned int)v18 & 1)) : (unsigned int)v18)));
                }
                v0 = v17;
                if (!(CmpF(v0, 0x5f800000) & 1))
                {
                    return 0xffffffff;
                }
                else if ((char)sub_40c1a3(idx, (CmpF(v0, 0x5f000000) & 69 & 1 ? (unsigned int)v0 : (unsigned int)(SubV(v0, 0x5f000000)) ^ 0)) != 1)
                {
                    return 0xffffffff;
                }
                else if (sub_40be36(idx, a1, &index, 0))
                {
                    abort(); /* do not return */
                }
            }
        }
        if (!index->field_0)
        {
            index->field_0 = a1;
            *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
            *((unsigned long long *)&idx[24]) = (long long)idx[24] + 1;
            return 1;
        }
        v4 = sub_40bdad(idx);
        if (v4)
        {
            v4->field_0 = a1;
            v4->field_8 = index->field_8;
            index->field_8 = v4;
            *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
            return 1;
        }
        return 0xffffffff;
    }
}



// Function: hash_insert @ 0xc73f
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]

    v0 = sub_40c3ce(a0, a1, &v1);
    if (v0 == 0xffffffff)
    {
        return 0;
    }
    else if (!v0)
    {
        return v1;
    }
    else
    {
        return a1;
    }
}



// Function: hash_remove @ 0xc7af
typedef struct struct_1 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long hash_remove(struct_2 *idx, unsigned long long a1)
{
    unsigned long v7;  // rax
    unsigned int v8;  // ymm0
    unsigned long v17;  // rax
    int v18;  // xmm0
    unsigned long long v19;  // rax
    unsigned long v20;  // rax
    int v21;  // xmm0
    int v9;  // xmm2
    unsigned long v10;  // rax
    unsigned long v12;  // rax
    unsigned int v13;  // ymm0
    int v14;  // xmm2
    unsigned long v15;  // rax
    struct_2 *v0;  // [bp-0x40]
    struct_1 *ptr;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    struct_0 *v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    struct_1 *v5;  // [bp-0x18]

    v2 = sub_40be36(idx, a1, &v0, 1);
    if (!v2)
        return 0;
    idx[2].field_0 = idx[2].field_0 - 1;
    if (v0->field_0)
        return v2;
    idx[1].field_8 = (char *)idx[1].field_8 - 1;
    v7 = idx[1].field_8;
    v8 = (v7 < 0 ? (unsigned int)(AddV((unsigned int)(v7 >> 1 | (unsigned int)v7 & 1), (unsigned int)(v7 >> 1 | (unsigned int)v7 & 1))) : (int)v7);
    v9 = (int)(unsigned int)idx[2].field_8->field_0;
    v10 = idx[1].field_0;
    if (...)
    {
        sub_40b7d8(idx);
        v12 = idx[1].field_8;
        v13 = (v12 < 0 ? (unsigned int)(AddV((unsigned int)(v12 >> 1 | (unsigned int)v12 & 1), (unsigned int)(v12 >> 1 | (unsigned int)v12 & 1))) : (int)v12);
        v14 = (int)(unsigned int)idx[2].field_8->field_0;
        v15 = idx[1].field_0;
        if (...)
        {
            v3 = idx[2].field_8;
            if (v3->field_10)
            {
                v17 = idx[1].field_0;
                v18 = (int)(MulV((v17 < 0 ? AddV((unsigned int)(v17 >> 1 | (unsigned int)v17 & 1), (unsigned int)(v17 >> 1 | (unsigned int)v17 & 1)) : (unsigned int)v17), v3->field_4));
                v19 = (CmpF(*((unsigned int *)&v18), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v18) : *((unsigned int *)&SubV(v18, 0x5f000000)) ^ 0x8000000000000000);
            }
            else
            {
                v20 = idx[1].field_0;
                v21 = (int)(MulV(v3->field_8, MulV(v3->field_4, (v20 < 0 ? AddV((unsigned int)(v20 >> 1 | (unsigned int)v20 & 1), (unsigned int)(v20 >> 1 | (unsigned int)v20 & 1)) : (unsigned int)v20))));
                v19 = (CmpF(*((unsigned int *)&v21), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v21) : *((unsigned int *)&SubV(v21, 0x5f000000)) ^ 0x8000000000000000);
            }
            v4 = v19;
            if ((char)sub_40c1a3(idx, v4) != 1)
            {
                for (ptr = idx[4].field_8; ptr; ptr = v5)
                {
                    v5 = ptr->field_8;
                    free(ptr);
                }
                idx[4].field_8 = NULL;
            }
        }
    }
    return v2;
}



// Function: hash_delete @ 0xcab5
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned long long hash_delete(struct_2 *a0, unsigned long long a1)
{
    return sub_40c7af(a0, a1);
}



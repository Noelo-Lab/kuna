// Function: async_safe_die @ 0x439b
void async_safe_die(unsigned int a0, void* a1)
{
    unsigned long v8;  // fs
    unsigned long count;  // [bp-0x48]
    void* v1;  // [bp-0x40]
    unsigned long count1;  // [bp-0x38]
    unsigned long count2;  // [bp-0x30]
    unsigned long v4;  // [bp-0x28]
    char v5;  // [bp-0x1c]
    unsigned long v6;  // [bp-0x10]

    v6 = *((long long *)(40 + v8));
    count = write(2, a1, strlen(a1));
    if (a0)
    {
        v1 = sub_4126fe(a0, &v5, &v5);
        count1 = write(2, ": errno ", 8);
        count2 = write(2, v1, strlen(v1));
    }
    v4 = write(2, "\n", 1);
    _exit(2); /* do not return */
}



// Function: sort_die @ 0x4464
void sort_die(unsigned long long a0, unsigned long a1)
{
    unsigned long long v4;  // rbx
    int *err;  // rax
    unsigned long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x10]

    v2 = v4;
    v1 = a0;
    v0 = a1;
    if (!v0)
        v0 = gettext("standard output");
    sub_4151eb(0, 3, v0);
    err = __errno_location();
    error(2, *(err), "%s: %s");
}



// Function: usage @ 0x44d3
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern FILE *stdout;

void usage(int a0)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    long long v13;  // r9
    char *v14;  // rax
    char *v15;  // rax
    char *v16;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    char *v21;  // rax
    char *v22;  // rax
    char *v5;  // rax
    char *v23;  // rax
    char *v24;  // rax
    char *v25;  // rax
    char *v27;  // rax
    char *v28;  // rax
    char *v29;  // rax
    char *v30;  // rax
    char *v31;  // rax
    char *v7;  // rax
    long long v8;  // rdi
    long long v9;  // rsi
    long long v10;  // rdx
    long long v11;  // rcx
    long long v12;  // r8
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v1 = v3;
    v0 = v4;
    if (a0)
    {
        v5 = gettext("Try '%s --help' for more information.\n");
        fprintf(stderr, v5);
    }
    else
    {
        printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"));
        v7 = gettext("Write sorted concatenation of all FILE(s) to standard output.\n");
        fputs_unlocked(v7, stdout);
        sub_4040a2(v8, v9, v10, v11, v12, v13);
        sub_4040d3(v8, v9, v10, v11, v12, v13);
        v14 = gettext("Ordering options:\n\n");
        fputs_unlocked(v14, stdout);
        v15 = gettext("  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n");
        fputs_unlocked(v15, stdout);
        v16 = gettext("  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < 'JAN' < ... < 'DEC'\n");
        fputs_unlocked(v16, stdout);
        v17 = gettext("  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n");
        fputs_unlocked(v17, stdout);
        v18 = gettext("  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n");
        fputs_unlocked(v18, stdout);
        v19 = gettext("      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n");
        fputs_unlocked(v19, stdout);
        v20 = gettext("Other options:\n\n");
        fputs_unlocked(v20, stdout);
        v21 = gettext("      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n");
        fputs_unlocked(v21, stdout);
        v22 = gettext("  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n");
        fputs_unlocked(v22, stdout);
        v23 = gettext("      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n");
        fputs_unlocked(v23, stdout);
        v24 = gettext("  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n");
        fputs_unlocked(v24, stdout);
        v25 = gettext("  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n");
        fputs_unlocked(v25, stdout);
        printf(gettext("  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n"));
        v27 = gettext("  -z, --zero-terminated     line delimiter is NUL, not newline\n");
        fputs_unlocked(v27, stdout);
        v28 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v28, stdout);
        v29 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v29, stdout);
        v30 = gettext("\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line's end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n");
        fputs_unlocked(v30, stdout);
        v31 = gettext("% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n");
        fputs_unlocked(v31, stdout);
        sub_404104("sort");
    }
    exit(a0); /* do not return */
}



// Function: cs_enter @ 0x47f1
extern long long g_4236a0;

char * cs_enter(char *a0)
{
    unsigned int v0;  // [bp-0xc]

    v0 = pthread_sigmask(0, &g_4236a0, a0 + 8);
    *(a0) = !v0;
    return a0;
}



// Function: cs_leave @ 0x4833
long long cs_leave(char *a0)
{
    unsigned long count;  // rax

    count = *(a0);
    if ((char)count)
        count = pthread_sigmask(2, a0 + 8, 0);
    return count;
}



// Function: proctab_hasher @ 0x486b
typedef struct struct_0 {
    char padding_0[8];
    int field_8;
} struct_0;

long long proctab_hasher(struct_0 *a0, unsigned long long a1)
{
    return a0->field_8 % a1;
}



// Function: proctab_comparator @ 0x489a
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
} struct_0;

long long proctab_comparator(unsigned long a0, struct_0 *a1)
{
    unsigned long v1;  // rax

    v1 = a1->field_8;
    return _INSERT(v1, 0, *((int *)(a0 + 8)) == (unsigned int)v1);
}



// Function: reap @ 0x48cf
extern long long g_423690;
extern unsigned int g_423730;

long long reap(int a0)
{
    int v4;  // eax
    char *v5;  // rax
    int *err;  // rax
    int v0;  // [bp-0x2c]
    int v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x24]

    v0 = a0;
    if (v0)
        v4 = v0;
    else
        v4 = -0x1;
    v2 = waitpid(v4, &v1, !v0);
    if (v2 < 0)
    {
        sub_415059(4, g_423690);
        v5 = gettext("waiting for %s [-d]");
        err = __errno_location();
        error(2, *(err), v5);
    }
    else if (v2 > 0)
    {
        if (v0 <= 0 && !(char)sub_404a90(v2))
            return v2;
        if (!((char)v1 & 127) && !(char)(v1 >> 8))
        {
            g_423730 = g_423730 - 1;
            return v2;
        }
        sub_415059(4, g_423690);
        error(2, 0, gettext("%s [-d] terminated abnormally"));
    }
    else
    {
        return v2;
    }
}



// Function: register_proc @ 0x4a08
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

extern long long g_423728;

unsigned long long register_proc(struct_0 *a0)
{
    unsigned long long v1;  // rax

    if (!g_423728)
    {
        g_423728 = sub_41120d(47, 0, sub_40486b, sub_40489a, 0);
        if (!g_423728)
            sub_41865c(); /* do not return */
    }
    a0->field_c = 1;
    v1 = sub_411f4e(g_423728, a0, a0);
    if (v1)
        return v1;
    sub_41865c(); /* do not return */
}



// Function: delete_proc @ 0x4a90
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

extern long long g_423728;

unsigned int delete_proc(unsigned int a0)
{
    struct_0 *v0;  // [bp-0x30]
    char v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x20]

    v2 = a0;
    v0 = sub_411fbe(g_423728, &v1, &v1);
    if (v0)
    {
        v0->field_c = 2;
        return 1;
    }
    return 0;
}



// Function: wait_proc @ 0x4aff
long long wait_proc(unsigned int a0)
{
    long long v1;  // rax

    v1 = sub_404a90(a0);
    if (!(char)v1)
        return v1;
    return sub_4048cf(a0);
}



// Function: reap_exited @ 0x4b29
extern void g_423730;

long long reap_exited(void)
{
    unsigned long v1;  // rax

    while (*((int *)&g_423730) > 0)
    {
        v1 = sub_4048cf(0);
        if (!(unsigned int)v1)
            return v1;
    }
    return *((int *)&g_423730);
}



// Function: reap_some @ 0x4b4f
long long reap_some(void)
{
    sub_4048cf(-0x1);
    return sub_404b29();
}



// Function: reap_all @ 0x4b69
extern void g_423730;

long long reap_all(void)
{
    while (*((int *)&g_423730) > 0)
    {
        sub_4048cf(-0x1);
    }
    return *((int *)&g_423730);
}



// Function: cleanup @ 0x4b8b
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_423720;

struct_0 ** cleanup(void)
{
    struct_0 **v2;  // rax
    struct_0 **i;  // [bp-0x10]

    v2 = g_423720;
    for (i = g_423720; i; i = *(i))
    {
        unlink((char *)&i[1] + 5);
        v2 = *(i);
    }
    g_423720 = 0;
    return v2;
}



// Function: exit_cleanup @ 0x4bd4
extern unsigned long long g_423720;

void exit_cleanup(void)
{
    char v0[136];  // [bp-0x98]

    if (!g_423720)
    {
        sub_40fa34();
        return;
    }
    sub_4047f1(v0);
    sub_404b8b();
    sub_404833(v0);
    sub_40fa34();
    return;
}



// Function: create_temp_file @ 0x4c3d
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_1 *g_423100;
extern unsigned long long g_423668;
extern char g_423670;
extern unsigned long long g_4237a0;

unsigned long long * create_temp_file(unsigned int *a0, char a1)
{
    char *v8;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0xd0]
    unsigned int v1;  // [bp-0xcc]
    unsigned long long *ptr1;  // [bp-0xc8]
    unsigned long ptr;  // [bp-0xc0]
    unsigned long len;  // [bp-0xb8]
    unsigned long long *v5;  // [bp-0xb0]
    char v6[136];  // [bp-0xa8]

    ptr = *((long long *)(g_4237a0 * 8 + g_423668));
    len = strlen(ptr);
    ptr1 = sub_418024(len + 32 & 0xfffffffffffffff8);
    v5 = (char *)&ptr1[1] + 5;
    memcpy(v5, ptr, len);
    memcpy(len + (char *)v5, "/sortXXXXXX", 12);
    *(ptr1) = 0;
    g_4237a0 = g_4237a0 + 1;
    if (g_4237a0 == *((long long *)&g_423670))
        g_4237a0 = 0;
    sub_4047f1(v6);
    v0 = sub_41704d(v5, 0x80000);
    if (v0 >= 0)
    {
        g_423100->field_0 = ptr1;
        g_423100 = ptr1;
    }
    v1 = *(__errno_location());
    sub_404833(v6);
    *(__errno_location()) = v1;
    if (v0 < 0)
    {
        if (a1 != 1 || *(__errno_location()) != 24)
        {
            sub_415059(4, ptr);
            v8 = gettext("cannot create temporary file in %s");
            err = __errno_location();
            error(2, *(err), v8);
        }
        free(ptr1);
        ptr1 = NULL;
    }
    *(a0) = v0;
    return ptr1;
}



// Function: get_outstatus @ 0x4e8c
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

extern int g_4237a8;
extern stat g_4237c0;

stat * get_outstatus(void)
{
    if (!g_4237a8)
        g_4237a8 = (!fstat(1, &g_4237c0.st_mode) ? 0xffffffff : *(__errno_location()));
    return (g_4237a8 < 0 ? &g_4237c0.st_mode : NULL);
}



// Function: stream_open @ 0x4ee4
typedef struct FILE {
} FILE;

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

extern unsigned long long stdin;
extern unsigned long long stdout;
extern char g_423683;

FILE * stream_open(unsigned long a0, char *a1)
{
    unsigned int v5;  // edx
    unsigned int v0;  // [bp-0x30]
    unsigned int v1;  // [bp-0x2c]
    unsigned long v2;  // [bp-0x28]
    stat *v3;  // [bp-0x20]

    if (*(a1) == 114)
    {
        if (!strcmp(a0, "-"))
        {
            g_423683 = 1;
            v2 = stdin;
        }
        else
        {
            v1 = open(a0, 0x80000, v5);
            v2 = (v1 < 0 ? NULL : fdopen(v1, a1));
        }
        sub_40fed8(v2, 2);
        return v2;
    }
    else if (*(a1) == 0x77)
    {
        if (a0 && ftruncate(1, 0))
        {
            v0 = *(__errno_location());
            v3 = sub_404e8c();
            if (!v3 || ((unsigned short)v3->st_nlink & 0xf000) == 0x8000)
            {
                sub_4151eb(0, 3, a0);
                error(2, v0, gettext("%s: error truncating"));
            }
        }
        v2 = stdout;
        return v2;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: xfopen @ 0x505e
typedef struct FILE {
} FILE;

FILE * xfopen(unsigned int a0, char *a1)
{
    FILE *v0;  // [bp-0x10]

    v0 = sub_404ee4(a0, a1);
    if (v0)
        return v0;
    sub_404464(gettext("open failed"), a0); /* do not return */
}



// Function: xfclose @ 0x50b7
long long xfclose(unsigned long a0, unsigned int a1)
{
    unsigned int v1;  // eax
    unsigned long v2;  // rax
    unsigned long v4;  // rax

    v1 = fileno(a0);
    if (!v1)
    {
        return (unsigned long long)clearerr_unlocked(a0);
    }
    else if (v1 == 1)
    {
        v2 = fflush_unlocked(a0);
        if (!(unsigned int)v2)
            return v2;
        sub_404464(gettext("fflush failed"), a1); /* do not return */
    }
    else
    {
        v4 = sub_40ff17(a0);
        if (!(unsigned int)v4)
            return v4;
        sub_404464(gettext("close failed"), a1); /* do not return */
    }
}



// Function: move_fd @ 0x515d
int move_fd(unsigned int a0, unsigned int a1)
{
    if (a0 == a1)
        return a0;
    dup2(a0, a1);
    return close(a0);
}



// Function: pipe_fork @ 0x5193
extern unsigned int g_423024;
extern unsigned long long g_423720;
extern unsigned int g_423730;

int pipe_fork(int *a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0xc8]
    unsigned long long v1;  // [bp-0xc8]
    int v2;  // [bp-0xb0]
    unsigned int v3;  // [bp-0xac]
    unsigned long long v4;  // [bp-0xa8]
    unsigned long v5;  // [bp-0xa0]
    char v6[136];  // [bp-0x98]

    v0 = a1;
    v4 = 0x3fd0000000000000;
    if (sub_41311d(a0, 0x80000) < 0)
        return -0x1;
    if (g_423024 + 1 < g_423730)
        sub_404b4f();
    while (1)
    {
        v1 = v0 - 1;
        if (!v0)
            break;
        sub_4047f1(v6);
        v5 = g_423720;
        g_423720 = 0;
        v2 = fork();
        v3 = *(__errno_location());
        if (v2)
            g_423720 = v5;
        sub_404833(v6);
        *(__errno_location()) = v3;
        if (v2 >= 0 || *(__errno_location()) != 11)
            break;
        sub_418863();
        v4 = AddV(v4, v4);
        sub_404b29();
        v0 = v1;
    }
    if (v2 < 0)
    {
        v3 = *(__errno_location());
        close(*(a0));
        close(a0[1]);
        *(__errno_location()) = v3;
    }
    else if (!v2)
    {
        close(0);
        close(1);
    }
    else
    {
        g_423730 = g_423730 + 1;
    }
    return v2;
}



// Function: maybe_create_temp @ 0x5372
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    int field_8;
    char field_c;
} struct_0;

extern char *g_423690;

struct_0 * maybe_create_temp(FILE **a0, char a1)
{
    unsigned int *err;  // rax
    unsigned int v0;  // [bp-0x34]
    struct_0 *idx;  // [bp-0x30]
    int v2;  // [bp-0x28]
    unsigned int v3;  // [bp-0x24]

    idx = sub_404c3d(&v0, a1);
    if (!idx)
        return NULL;
    idx->field_c = 0;
    if (g_423690)
    {
        idx->field_8 = sub_405193(&v2, 4);
        if (idx->field_8 > 0)
        {
            close(v0);
            close(v2);
            v0 = v3;
            sub_404a08(idx);
        }
        else if (!idx->field_8)
        {
            close(v3);
            sub_40515d(v0, 1);
            sub_40515d(v2, 0);
            execlp(g_423690, g_423690);
            err = __errno_location();
            sub_40439b(*(err), "couldn't execute compress program"); /* do not return */
        }
    }
    *(a0) = fdopen(v0, "w");
    if (*(a0))
        return idx;
    sub_404464(gettext("couldn't create temporary file"), idx + 1); /* do not return */
}



// Function: create_temp @ 0x54f9
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    int field_8;
    char field_c;
} struct_0;

struct_0 * create_temp(FILE **a0)
{
    return sub_405372(a0, 0);
}



// Function: open_temp @ 0x551c
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
} struct_0;

extern void g_423690;

FILE * open_temp(struct_0 *a0, unsigned long a1, unsigned int a2)
{
    unsigned int v7;  // edx
    unsigned int *err;  // rax
    char *v9;  // rax
    int *err1;  // rax
    unsigned int v0;  // [bp-0x3c]
    unsigned int v1;  // [bp-0x38]
    unsigned int v2;  // [bp-0x34]
    FILE *fp;  // [bp-0x30]
    int v4;  // [bp-0x28]
    unsigned int v5;  // [bp-0x24]

    fp = NULL;
    if (a0->field_c == 1)
        sub_404aff(a0->field_8);
    v0 = open(a0 + 1, 0, v7);
    if (v0 < 0)
        return NULL;
    v1 = sub_405193(&v4, 9);
    if (v1 != 0xffffffff)
    {
        if (!v1)
        {
            close(v4);
            sub_40515d(v0, 0);
            sub_40515d(v5, 1);
            execlp(*((long long *)&g_423690), *((long long *)&g_423690));
            err = __errno_location();
            sub_40439b(*(err), "couldn't execute compress program (with -d)"); /* do not return */
        }
        a0->field_8 = v1;
        sub_404a08(a0);
        close(v0);
        close(v5);
        fp = fdopen(v4, "r");
        if (!fp)
        {
            v2 = *(__errno_location());
            close(v4);
            *(__errno_location()) = v2;
        }
    }
    else
    {
        if (*(__errno_location()) != 24)
        {
            sub_415059(4, *((long long *)&g_423690));
            v9 = gettext("couldn't create process for %s -d");
            err1 = __errno_location();
            error(2, *(err1), v9);
        }
        close(v0);
        *(__errno_location()) = 24;
    }
    return fp;
}



// Function: add_temp_dir @ 0x570f
extern long long g_423668;
extern unsigned long long g_423670;
extern long long g_423678;

long long add_temp_dir(unsigned long a0)
{
    unsigned long long idx;  // rax

    if (g_423670 == g_423678)
        g_423668 = sub_418221(g_423668, &g_423678, 8, &g_423678);
    idx = g_423670;
    g_423670 = idx + 1;
    *((unsigned long *)(g_423668 + idx * 8)) = a0;
    return a0;
}



// Function: zaptemp @ 0x5782
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
    char field_c;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[4];
    char field_c;
    char field_d;
} struct_1;

extern unsigned long long g_423100;
extern struct_0 g_423720;

void zaptemp(unsigned long a0)
{
    unsigned int v0;  // [bp-0xc8]
    unsigned int v1;  // [bp-0xc4]
    struct_0 *v2;  // [bp-0xc0]
    struct_1 *ptr;  // [bp-0xb8]
    unsigned long v4;  // [bp-0xb0]
    char v5[136];  // [bp-0xa8]

    v0 = 0;
    v2 = &g_423720.field_0;
    while (1)
    {
        ptr = v2->field_0;
        if (a0 == &ptr->field_d)
            break;
        v2 = ptr;
    }
    if (ptr->field_c == 1)
        sub_404aff(ptr->padding_8);
    v4 = ptr->field_0;
    sub_4047f1(v5);
    v1 = unlink(a0);
    v0 = *(__errno_location());
    v2->field_0 = v4;
    sub_404833(v5);
    if (v1)
    {
        sub_4151eb(0, 3, a0);
        error(0, v0, gettext("warning: cannot remove: %s"));
    }
    if (!v4)
        g_423100 = v2;
    free(ptr);
    return;
}



// Function: struct_month_cmp @ 0x590b
int struct_month_cmp(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: inittables @ 0x594a
extern unsigned long long g_423040;
extern unsigned int g_423048;
extern char g_42322a;
extern long long g_423260;
extern long long g_423360;
extern char g_423460;
extern char g_423560;

long long inittables(void)
{
    unsigned long long v8;  // rdx
    long long v9;  // rdi
    unsigned long long *v18;  // rax
    char v19;  // al
    long long v10;  // rsi
    long long v11;  // r8
    long long v12;  // r9
    unsigned long long *v13;  // rax
    unsigned int v14;  // eax
    long long v15;  // rdx
    unsigned long long *v16;  // rax
    char v17;  // al
    unsigned long long iter;  // [bp-0x48]
    unsigned long j;  // [bp-0x40]
    unsigned long v2;  // [bp-0x38]
    unsigned long v3;  // [bp-0x38]
    char *v4;  // [bp-0x30]
    unsigned long len;  // [bp-0x28]
    unsigned long v6;  // [bp-0x20]

    for (iter = 0; iter <= 0xff; iter += 1)
    {
        v8 = iter + (char *)&g_423260;
        *((char *)v8) = sub_404051((char)iter);
        v13 = __ctype_b_loc(v9, v10, v8, &g_423260, v11, v12);
        v14 = *((short *)((int)iter * 2 + *(v13))) & 0x4000;
        v15 = _INSERT(*(v13), 0, !v14);
        *(iter + (char *)&g_423360) = !v14;
        v16 = __ctype_b_loc(v9, v10, v15, &g_423360, v11, v12);
        if (!((char)*((short *)((int)iter * 2 + *(v16))) & 8) && (char)sub_404051((char)iter) != 1)
            v17 = 1;
        else
            v17 = 0;
        (&g_423460)[iter] = v17 & 1;
        (&g_423560)[iter] = toupper(iter) & 0xffffffff;
    }
    if (!g_42322a)
        return g_42322a;
    for (iter = 0; iter <= 11; iter += 1)
    {
        v4 = nl_langinfo((unsigned int)iter + 131086);
        len = strlen(v4);
        v6 = sub_418024(len + 1);
        (&g_423040)[2 * iter] = v6;
        (&g_423048)[4 * iter] = (unsigned int)iter + 1;
        v2 = 0;
        for (j = v2; j < len; j += 1)
        {
            v18 = __ctype_b_loc();
            v19 = sub_404042(v4[j]);
            if (!((char)*((short *)(v19 * 2 + *(v18))) & 1))
            {
                v3 = v2 + 1;
                *((char *)(v6 + v2)) = *((char *)(unsigned int)&(&g_423560)[sub_404042(v4[j])]);
                v2 = v3;
            }
        }
        *((char *)(v2 + v6)) = 0;
    }
    return (unsigned long long)qsort(&g_423040, 12, 16, sub_40590b);
}



// Function: specify_nmerge @ 0x5ba8
extern long long g_41b1db;
extern long long g_422560;
extern unsigned int g_423024;

int specify_nmerge(unsigned int a0, char a1, long long a2)
{
    unsigned long v10;  // fs
    unsigned int v0;  // [bp-0x58]
    unsigned int v1;  // [bp-0x54]
    char v2;  // [bp-0x50]
    unsigned int v3[3];  // [bp-0x48]
    char v4;  // [bp-0x2b]
    unsigned long v5;  // [bp-0x20]

    v0 = sub_418b54(a2, 0, 10, &v2, &g_41b1db);
    v1 = (!getrlimit(7, v3) ? (unsigned int)v3 - 3 : 0x11);
    if (!v0)
    {
        g_423024 = v2;
        if (g_423024 != v2)
        {
            v0 = 1;
        }
        else if (g_423024 <= 1)
        {
            sub_4154a0(a2);
            error(0, 0, gettext("invalid --%s argument %s"));
            sub_4154a0("2");
            error(2, 0, gettext("minimum --%s argument is %s"));
        }
        else if (v1 < g_423024)
        {
            v0 = 1;
        }
        else
        {
            return v5 - *((long long *)(40 + v10));
        }
    }
    if (v0 != 1)
        sub_418a33(v0, a0, a1, &g_422560, a2); /* do not return */
    sub_4154a0(a2);
    error(0, 0, gettext("--%s argument %s too large"));
    sub_412802(v1, &v4, &v4);
    error(2, 0, gettext("maximum --%s argument with current rlimit is %s"));
}



// Function: specify_sort_size @ 0x5e0c
extern long long g_422560;
extern unsigned int g_423024;
extern unsigned long long g_423660;

int specify_sort_size(unsigned int a0, char a1, long long a2)
{
    unsigned int v6;  // eax
    int v7;  // xmm0
    unsigned long long v8;  // rdx
    unsigned long long v9;  // rax
    unsigned long v10;  // fs
    unsigned int v0;  // [bp-0x2c]
    char v1;  // [bp-0x28], Other Possible Types: unsigned long long
    char v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    v0 = sub_418b54(a2, &v2, 10, &v1, "EgGkKmMPtTYZ");
    if (!v0 && *((char *)(v2 - 1)) - 48 <= 9)
    {
        if (v1 <= 0x3fffffffffffff)
            v1 *= 0x400;
        else
            v0 = 1;
    }
    if (v0 == 2 && *((char *)(v2 - 1)) - 48 <= 9 && !*((char *)(v2 + 1)))
    {
        switch (v6)
        {
        case 37:
            sub_412e84();
            v3 = *((unsigned long long *)&DivV(MulV((v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1), v7), 0x4059000000000000));
            if (((CmpF(0x43f0000000000000, v3) & 69 | (char)((CmpF(0x43f0000000000000, v3) & 69) >> 6)) & 1) != 1)
            {
                v1 = (CmpF(v3, 0x43e0000000000000) & 69 & 1 ? v3 : (unsigned long long)(SubV(v3, 0x43e0000000000000)) ^ 0x8000000000000000);
                v0 = 0;
            }
            else
            {
                v0 = 1;
            }
            break;
        case 98:
            v0 = 0;
            break;
        default:
            goto LABEL_405fa5;
        }
    }
LABEL_405fa5:
    if (v0)
        sub_418a33(v0, a0, a1, &g_422560, a2); /* do not return */
    if (v1 < g_423660)
        return v4 - *((long long *)(40 + v10));
    g_423660 = v1;
    if (g_423660 != v1)
    {
        v0 = 1;
        sub_418a33(v0, a0, a1, &g_422560, a2); /* do not return */
    }
    v8 = g_423024 * 0x22;
    v9 = g_423660;
    if (g_423660 <= v8)
        v9 = v8;
    g_423660 = v9;
    return v4 - *((long long *)(40 + v10));
}



// Function: specify_nthreads @ 0x6042
extern long long g_41b1db;
extern long long g_422560;

unsigned long long specify_nthreads(unsigned int a0, char a1, long long a2)
{
    unsigned int v0;  // [bp-0x1c]
    char v1;  // [bp-0x18], Other Possible Types: unsigned long long

    v0 = sub_418b54(a2, 0, 10, &v1, &g_41b1db);
    if (v0 == 1)
        return 0xffffffffffffffff;
    if (v0)
        sub_418a33(v0, a0, a1, &g_422560, a2); /* do not return */
    if (v1)
        return v1;
    error(2, 0, gettext("number in parallel must be nonzero"));
}



// Function: default_sort_size @ 0x610d
extern unsigned int g_423024;

unsigned long long default_sort_size(void)
{
    long long v4;  // rdi
    long long v5;  // rsi
    unsigned long long v15;  // rdx
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9
    unsigned long long v10;  // xmm0lq
    unsigned long long v11;  // xmm0lq
    int v13;  // xmm0
    unsigned int v0[3];  // [bp-0x48], Other Possible Types: unsigned long long
    unsigned long long v1;  // [bp-0x30]
    unsigned int v2[3];  // [bp-0x28]

    v0 = 0xffffffffffffffff;
    if (!getrlimit(2, v2) && v0 > v2)
        v0 = v2;
    if (!getrlimit(9, v2) && v0 > v2)
        v0 = v2;
    v0 = (unsigned long long)(v0 >> 1);
    if (!getrlimit(5, v2) && v0 > (v2 >> 4) * 15)
        v0 = (unsigned long long)((v2 >> 4) * 15);
    sub_412fab();
    sub_412e84(v4, v5, v6, v7, v8, v9);
    if (((CmpF(v10, (unsigned long long)(DivV(v10, 0x4020000000000000))) & 69 | (char)((CmpF(v10, (unsigned long long)(DivV(v10, 0x4020000000000000))) & 69) >> 6)) & 1) != 1)
        v11 = v10;
    else
        v11 = DivV(v10, 0x4020000000000000);
    v1 = v11;
    if (((CmpF((v0 < 0 ? (unsigned long long)(AddV(v0 >> 1 | (unsigned int)v0 & 1, v0 >> 1 | (unsigned int)v0 & 1)) : v0), (unsigned long long)(MulV(v10, 0x3fe8000000000000))) & 69 | (char)((CmpF((v0 < 0 ? (unsigned long long)(AddV(v0 >> 1 | (unsigned int)v0 & 1, v0 >> 1 | (unsigned int)v0 & 1)) : v0), (unsigned long long)(MulV(v10, 0x3fe8000000000000))) & 69) >> 6)) & 1) != 1)
    {
        v13 = (int)(MulV(0x3fe8000000000000, v10));
        if (CmpF(*((unsigned long long *)&v13), 0x43e0000000000000) & 1)
        {
            v0 = *((unsigned long long *)&v13);
        }
        else
        {
            v0 = *((unsigned long long *)&SubV(v13, 0x43e0000000000000));
            v0 ^= 0x8000000000000000;
        }
    }
    if (((CmpF((v0 < 0 ? (unsigned long long)(AddV(v0 >> 1 | (unsigned int)v0 & 1, v0 >> 1 | (unsigned int)v0 & 1)) : v0), v1) & 69 | (char)((CmpF((v0 < 0 ? (unsigned long long)(AddV(v0 >> 1 | (unsigned int)v0 & 1, v0 >> 1 | (unsigned int)v0 & 1)) : v0), v1) & 69) >> 6)) & 1) != 1)
    {
        if (CmpF(v1, 0x43e0000000000000) & 1)
        {
            v0 = v1;
        }
        else
        {
            v0 = SubV(v1, 0x43e0000000000000);
            v0 ^= 0x8000000000000000;
        }
    }
    v15 = g_423024 * 0x22;
    if (v0 > v15)
        return v0;
    return v15;
}



// Function: sort_buffer_size @ 0x637c
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

extern char g_423660;
extern unsigned long long g_423850;

unsigned long long sort_buffer_size(unsigned long a0, unsigned long long a1, unsigned long a2, unsigned long long a3, unsigned long long a4)
{
    char v8;  // al
    unsigned long v11;  // rax
    unsigned long long v0;  // [bp-0xe0]
    unsigned long long v1;  // [bp-0xd8]
    unsigned long v2;  // [bp-0xd0]
    unsigned long long v3;  // [bp-0xc8]
    unsigned long long v4;  // [bp-0xc0]
    stat v5;  // [bp-0xb8]

    v3 = a4 + 1;
    v0 = v3 + 1;
    v1 = 0;
    while (1)
    {
        if (v1 >= a3)
            return v0;
        if (v1 < a1)
        {
            v8 = fstat(fileno(*((long long *)(a0 + v1 * 8))), &v5);
        }
        else if (!strcmp(*((long long *)(a2 + v1 * 8)), "-"))
        {
            v8 = fstat(0, &v5);
        }
        else
        {
            v8 = stat(*((long long *)(a2 + v1 * 8)), &v5);
        }
        if (v8)
        {
            v11 = gettext("stat failed");
            sub_404464(v11, *((long long *)(a2 + v1 * 8))); /* do not return */
        }
        if (((unsigned short)v5.st_nlink & 0xf000) == 0x8000)
        {
            v2 = v5.st_atime;
        }
        else
        {
            if (*((long long *)&g_423660))
                return *((long long *)&g_423660);
            v2 = 0x20000;
        }
        if (!g_423850)
        {
            g_423850 = *((long long *)&g_423660);
            if (!g_423850)
                g_423850 = sub_40610d();
        }
        v4 = v3 * v2 + 1;
        if (v4 / v3 != v2 || v4 >= g_423850 - v0)
            break;
        v0 += v4;
        v1 += 1;
    }
    return g_423850;
}



// Function: initbuf @ 0x6618
typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
} struct_0;

struct_0 * initbuf(struct_0 *idx, unsigned long long a1, unsigned long long a2)
{
    unsigned long v0;  // [bp-0x20], Other Possible Types: unsigned long long

    v0 = a2;
    while (1)
    {
        v0 = (v0 & 0xffffffffffffffe0) + 32;
        idx->field_0 = malloc(v0);
        if (idx->field_0)
        {
            idx->field_28 = a1;
            *((unsigned long *)&idx->field_18) = v0;
            idx->field_10 = 0;
            idx->field_20 = idx->field_10;
            idx->field_8 = idx->field_20;
            idx->field_30 = 0;
            return idx;
        }
        v0 >>= 1;
        if (v0 <= a1 + 1)
            sub_41865c(); /* do not return */
    }
}



// Function: buffer_linelim @ 0x66d0
unsigned long long buffer_linelim(unsigned long long *a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0[3] + *(a0);
    return v0;
}



// Function: begfield @ 0x66f4
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[32];
    char field_30;
} struct_2;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_423020;
extern char g_423260;

char * begfield(struct_0 *a0, struct_2 *a1)
{
    char *v5;  // rdx
    char *iter;  // [bp-0x28]
    unsigned long long node;  // [bp-0x20], Other Possible Types: unsigned long
    char *v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    iter = &a0->field_0->field_0;
    if (*((int *)&g_423020) != 128)
    {
        for (node = a1->field_0; iter < v2 && (node = (unsigned long)(node - 1), node); node = node)
        {
            for (v3 = a1->field_8; iter < v2 && *(iter) != *((int *)&g_423020); iter += 1);
            node = node;
            if (iter >= v2)
                continue;
            iter += 1;
        }
    }
    else
    {
        while (iter < v2 && (node = (unsigned long)(node - 1), node))
        {
            for (v2 = iter + a0->field_8 - 1; iter < v2 && *((char *)(unsigned int)&(&g_423260)[sub_404042(*(iter))]); iter += 1);
            while (1)
            {
                if (iter >= v2 || !(node = (unsigned long long)node, *((char *)(long long)(int)(unsigned int)((char)(unsigned long long)sub_404042((unsigned long long)(unsigned int)(int)*(iter)) + &g_423260)) != 1))
                    break;
                iter += 1;
            }
        }
    }
    if (a1->field_30)
    {
        for (; iter < v2 && *((char *)(unsigned int)&(&g_423260)[sub_404042(*(iter))]); iter += 1);
    }
    v5 = &iter[v3];
    if (v2 <= v5)
        v5 = v2;
    iter = v5;
    return iter;
}



// Function: limfield @ 0x6898
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[17];
    char field_31;
} struct_2;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_423020;
extern char g_423260;

char * limfield(struct_0 *a0, struct_2 *idx)
{
    char *v5;  // rdx
    char *node;  // [bp-0x28]
    unsigned long long iter;  // [bp-0x20], Other Possible Types: unsigned long
    char *v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    node = &a0->field_0->field_0;
    v2 = node + a0->field_8 - 1;
    iter = idx->field_10;
    v3 = idx->field_18;
    if (!v3)
        iter += 1;
    if (*((int *)&g_423020) != 128)
    {
        for (; node < v2 && (iter = (unsigned long)(iter - 1), iter); iter = iter)
        {
            for (; node < v2 && *(node) != *((int *)&g_423020); node += 1);
            iter = iter;
            if (node >= v2 || !iter && !(iter = (unsigned long long)iter, v3))
                continue;
            node += 1;
        }
    }
    else
    {
        while (node < v2 && (iter = (unsigned long)(iter - 1), iter))
        {
            for (; node < v2 && *((char *)(unsigned int)&(&g_423260)[sub_404042(*(node))]); node += 1);
            while (1)
            {
                if (node >= v2 || !(iter = (unsigned long long)iter, *((char *)(long long)(int)(unsigned int)((char)(unsigned long long)sub_404042((unsigned long long)(unsigned int)(int)*(node)) + &g_423260)) != 1))
                    break;
                node += 1;
            }
        }
    }
    if (!v3)
        return node;
    if (idx->field_31)
    {
        for (; node < v2 && *((char *)(unsigned int)&(&g_423260)[sub_404042(*(node))]); node += 1);
    }
    v5 = &node[v3];
    if (v2 <= v5)
        v5 = v2;
    node = v5;
    return node;
}



// Function: fillbuf @ 0x6a5e
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    struct struct_2 *field_0;
    char field_8;
    char padding_9[7];
    long long field_10;
    unsigned long long field_18;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char field_30;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    char field_30;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_423010;
extern unsigned long long g_423018;
extern char g_423260;
extern struct_0 *g_423688;

unsigned int fillbuf(struct_1 *idx, FILE *a1, unsigned int a2)
{
    unsigned long v16;  // rax
    char v0;  // [bp-0x79]
    unsigned long v1;  // [bp-0x78]
    unsigned long v2;  // [bp-0x70]
    char *v3;  // [bp-0x68]
    struct_0 *iter;  // [bp-0x60]
    unsigned long node;  // [bp-0x58]
    char *iter1;  // [bp-0x50]
    char *v7;  // [bp-0x48]
    char *v8;  // [bp-0x48]
    struct_0 *v9;  // [bp-0x40]
    unsigned long v10;  // [bp-0x38]
    unsigned long long v11;  // [bp-0x30]
    unsigned long v12;  // [bp-0x28]
    unsigned long count;  // [bp-0x20]
    char *ptr;  // [bp-0x18]

    v9 = g_423688;
    v0 = g_423010;
    v10 = idx->field_28;
    v2 = g_423018 - 0x22;
    if (idx->field_30)
        return 0;
    if (*((long long *)&idx->field_8) != *((long long *)&idx->field_20))
    {
        memmove(idx->field_0, &idx->field_0[*((long long *)&idx->field_8) + -1 * *((long long *)&idx->field_20)], *((long long *)&idx->field_20));
        *((long long *)&idx->field_8) = *((long long *)&idx->field_20);
        idx->field_10 = 0;
    }
    while (1)
    {
        v3 = &idx->field_0[*((long long *)&idx->field_8)].field_0;
        v11 = sub_4066d0(idx);
        iter = v11 + -(idx->field_10 * 32);
        node = v11 + -(v10 * idx->field_10) - v3;
        iter1 = &(!idx->field_10 ? idx->field_0 : iter->padding_8 + iter->field_0)->field_0;
        do
        {
            if (node <= v10 + 1)
                break;
            v12 = (node - 1) / (v10 + 1);
            count = fread_unlocked(v3, 1, v12, a1);
            v7 = &v3[count];
            node -= count;
            if (count != v12)
            {
                if (ferror_unlocked(a1))
                {
                    sub_404464(gettext("read failed"), a2); /* do not return */
                }
                else if (feof_unlocked(a1))
                {
                    idx->field_30 = 1;
                    if (v7 == idx->field_0)
                    {
                        return 0;
                    }
                    else if (iter1 != v7 && v0 != *(v7 - 1))
                    {
                        v8 = v7 + 1;
                        *(v7) = v0;
                        v7 = v8;
                    }
                }
            }
            while (1)
            {
                ptr = memchr(v3, v0, v7 - v3);
                if (!ptr)
                    break;
                *(ptr) = 0;
                v3 = ptr + 1;
                iter = (char *)iter - 32;
                iter->field_0 = iter1;
                *((unsigned long *)&iter->padding_8[0]) = v3 - iter1;
                v16 = (unsigned long)iter->padding_8;
                if (iter->padding_8 <= v2)
                    v16 = v2;
                v2 = v16;
                node -= v10;
                if (v9)
                {
                    if (v9->field_0 != 0xffffffffffffffff)
                    {
                        iter->field_10 = sub_4066f4(iter, v9);
                    }
                    else
                    {
                        if (v9->field_30)
                        {
                            for (*((char **)&iter->padding_18[0]) = (v9->field_10 == 0xffffffffffffffff ? ptr : sub_406898(iter, v9)); *((char *)(unsigned int)&(&g_423260)[sub_404042(*(iter1))]); iter1 += 1);
                        }
                        iter->field_10 = iter1;
                    }
                }
                iter1 = v3;
            }
            v3 = v7;
        } while (!idx->field_30);
        *((unsigned long *)&idx->field_8) = v3 - idx->field_0;
        idx->field_10 = sub_4066d0(idx) - (char *)iter >> 5;
        if (idx->field_10)
            break;
        v1 = idx->field_18 >> 5;
        idx->field_0 = sub_418221(idx->field_0, &v1, 32, &v1);
        idx->field_18 = v1 * 32;
    }
    *((unsigned long *)&idx->field_20) = v3 - iter1;
    g_423018 = v2 + 0x22;
    return 1;
}



// Function: traverse_raw_number @ 0x6f64
extern char g_423220;
extern char g_423224;

int traverse_raw_number(void* *a0)
{
    void* v5;  // rax
    void* v6;  // rax
    char v0;  // [bp-0x13]
    char v1;  // [bp-0x12]
    char v2;  // [bp-0x11]
    void* v3;  // [bp-0x10]

    v3 = *(a0);
    v0 = 0;
    v1 = 0;
    while (1)
    {
        v5 = v3;
        v3 = v5 + 1;
        v2 = *((char *)v5);
        if (v2 - 48 > 9)
            break;
        if (v0 < v2)
            v0 = v2;
        v1 = *((char *)v3) == *((int *)&g_423224);
        if (!v1)
            continue;
        v3 += 1;
    }
    if (v1)
    {
        *(a0) = v3 - 2;
        return v0;
    }
    if (v2 == g_423220)
    {
        while (1)
        {
            v6 = v3;
            v3 = v6 + 1;
            v2 = *((char *)v6);
            if (v2 - 48 > 9)
                break;
            if (v0 >= v2)
                continue;
            v0 = v2;
        }
    }
    *(a0) = v3 - 1;
    return v0;
}



// Function: find_unit_order @ 0x7042
extern char g_41c5c0;

unsigned int find_unit_order(char *a0)
{
    char v0;  // [bp-0x1f]
    char v1;  // [bp-0x1e]
    char v2;  // [bp-0x1d]
    unsigned int v3;  // [bp-0x1c]
    char *v4;  // [bp-0x18]

    v0 = *(a0) == 45;
    v4 = &a0[v0];
    v1 = sub_406f64(&v4);
    if (v1 <= 48)
        return 0;
    v2 = *(v4);
    v3 = *((char *)(unsigned int)&(&g_41c5c0)[v2]);
    if (!v0)
        return v3;
    return -(v3);
}



// Function: human_numcompare @ 0x70e2
extern char g_423220;
extern unsigned int g_423224;
extern char g_423260;

int human_numcompare(char *a0, char *a1)
{
    unsigned int v4;  // eax
    char *j;  // [bp-0x38]
    char *i;  // [bp-0x30]
    unsigned int v2;  // [bp-0x1c]

    i = a0;
    for (j = a1; *((char *)(unsigned int)&(&g_423260)[sub_404042(*(i))]); i += 1);
    for (; *((char *)(unsigned int)&(&g_423260)[sub_404042(*(j))]); j += 1);
    v4 = sub_407042(i);
    v2 = v4 - sub_407042(j);
    if (v2)
        return v2;
    return sub_41764c(i, j, g_423220, g_423224);
}



// Function: numcompare @ 0x71a3
extern char g_423220;
extern unsigned int g_423224;
extern char g_423260;

int numcompare(char *a0, char *a1)
{
    char *j;  // [bp-0x18]
    char *i;  // [bp-0x10]

    i = a0;
    for (j = a1; *((char *)(unsigned int)&(&g_423260)[sub_404042(*(i))]); i += 1);
    for (; *((char *)(unsigned int)&(&g_423260)[sub_404042(*(j))]); j += 1);
    return sub_41764c(i, j, g_423220, g_423224);
}



// Function: nan_compare @ 0x7231
int nan_compare(char *a0, char *a1)
{
    char v0;  // [bp-0x38]
    char v1;  // [bp-0x28]

    memset(&v0, 0, 16);
    strtold(a0, NULL);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_78{s-56|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_78{s-56|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    memset(&v1, 0, 16);
    strtold(a1, NULL);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_79{s-40|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_79{s-40|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    return memcmp(&v0, &v1, 16);
}



// Function: general_numcompare @ 0x72d6
unsigned int general_numcompare(char *a0, char *a1)
{
    unsigned int v3;  // eax
    char *v0;  // [bp-0x48]
    char *v1;  // [bp-0x40], Other Possible Types: char*[6]

    strtold(a0, &v0);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_221{s-56|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_221{s-56|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    strtold(a1, v1);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_222{s-40|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_222{s-40|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    if (a0 == v0)
    {
        return (a1 == v1 ? 0 : 0xffffffff);
    }
    else if (a1 == v1)
    {
        return 1;
    }
    else
    {
        if (/* unsupported instruction */)
        {
            /* unsupported instruction */
            /* unsupported instruction */
        }
        else
        {
            /* unsupported instruction */
            /* unsupported instruction */
        }
        if (/* unsupported instruction */)
        {
            /* unsupported instruction */
            /* unsupported instruction */
        }
        else
        {
            /* unsupported instruction */
            /* unsupported instruction */
        }
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        if (((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69 | (char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 6)) & 1) != 1)
        {
            v3 = 0xffffffff;
        }
        else
        {
            if (/* unsupported instruction */)
            {
                /* unsupported instruction */
                /* unsupported instruction */
            }
            else
            {
                /* unsupported instruction */
                /* unsupported instruction */
            }
            if (/* unsupported instruction */)
            {
                /* unsupported instruction */
                /* unsupported instruction */
            }
            else
            {
                /* unsupported instruction */
                /* unsupported instruction */
            }
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            if (((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69 | (char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 6)) & 1) != 1)
            {
                v3 = 1;
            }
            else
            {
                if (/* unsupported instruction */)
                {
                    /* unsupported instruction */
                    /* unsupported instruction */
                }
                else
                {
                    /* unsupported instruction */
                    /* unsupported instruction */
                }
                if (/* unsupported instruction */)
                {
                    /* unsupported instruction */
                    /* unsupported instruction */
                }
                else
                {
                    /* unsupported instruction */
                    /* unsupported instruction */
                }
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                if (!((char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2) & 1))
                {
                    if (/* unsupported instruction */)
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    else
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    if (/* unsupported instruction */)
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    else
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    if (((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 6 & 1) != 1)
                        goto LABEL_40739c;
                    v3 = 0;
                }
                else
                {
LABEL_40739c:
                    if (/* unsupported instruction */)
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    else
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    if (/* unsupported instruction */)
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    else
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    if ((char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2) & 1)
                    {
                        if (/* unsupported instruction */)
                        {
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
                        else
                        {
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
                        if (/* unsupported instruction */)
                        {
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
                        else
                        {
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        v3 = ((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2 & 1 ? (int)sub_407231(a0, a1) : 1);
                    }
                    else
                    {
                        v3 = 0xffffffff;
                    }
                }
            }
        }
        return v3;
    }
}



// Function: getmonth @ 0x73f3
typedef struct struct_0 {
    char field_0;
} struct_0;

extern void g_423040;
extern unsigned int g_423048[4];
extern char g_423260;
extern char g_423560;

unsigned int * getmonth(char *a0, struct_0 **a1)
{
    char v7;  // al
    char v8;  // al
    char *i;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    char *iter;  // [bp-0x28]
    void* v4;  // [bp-0x20]
    unsigned long long idx;  // [bp-0x18]

    i = a0;
    v1 = 0;
    for (v2 = 12; *((char *)(unsigned int)&(&g_423260)[sub_404042(*(i))]); i += 1);
    while (1)
    {
        idx = v2 + v1 >> 1;
        iter = i;
        v4 = *((long long *)&(&g_423040)[16 * idx]);
        while (1)
        {
            if (!*((char *)v4))
            {
                if (!a1)
                    return g_423048[4 * idx];
                *(a1) = iter;
                return g_423048[4 * idx];
            }
            v7 = sub_404042(*((char *)(unsigned int)&(&g_423560)[sub_404042(*(iter))]));
            if (v7 < (sub_404042(*((char *)v4)) & 0xff))
            {
                v2 = idx;
                goto LABEL_40755a;
            }
            else
            {
                v8 = sub_404042(*((char *)(unsigned int)&(&g_423560)[sub_404042(*(iter))]));
                if (v8 > (sub_404042(*((char *)v4)) & 0xff))
                {
                    v1 = idx + 1;
LABEL_40755a:
                    if (v1 >= v2)
                        return NULL;
                    break;
                }
                else
                {
                    iter += 1;
                    v4 += 1;
                }
            }
        }
    }
}



// Function: random_md5_state_init @ 0x7573
extern long long g_423740;

void random_md5_state_init(long long a0)
{
    long long v0;  // [bp-0x50]
    long long v1;  // [bp-0x40]
    char v2;  // [bp-0x38]

    v0 = a0;
    v1 = sub_415668(v0, 16);
    if (!v1)
    {
        if (!v0)
            v0 = "getrandom";
        sub_404464(gettext("open failed"), v0); /* do not return */
    }
    else
    {
        sub_4159b4(v1, &v2, 16, &v2);
        if (!sub_415a0f(v1))
        {
            sub_40fb2c(&g_423740);
            sub_40fb4b(&v2, 16, &g_423740);
            return;
        }
        sub_404464(gettext("close failed"), v0); /* do not return */
    }
}



// Function: xstrxfrm @ 0x7668
long long xstrxfrm(char *a0, unsigned long a1, unsigned long a2)
{
    char *v2;  // rax
    int *err;  // rax
    unsigned long v0;  // [bp-0x20]

    *(__errno_location()) = 0;
    v0 = strxfrm(a0, a1, a2);
    if (!*(__errno_location()))
        return v0;
    v2 = gettext("string transformation failed");
    err = __errno_location();
    error(0, *(err), v2);
    error(0, 0, gettext("set LC_ALL='C' to work around the problem"));
    sub_414f8b(0, 8, a1);
    error(2, 0, gettext("the untransformed string was %s"));
}



// Function: compare_random @ 0x7755
extern char g_423229;
extern unsigned long long g_423740;
extern unsigned long long g_423748;
extern unsigned long long g_423750;
extern unsigned long long g_423758;
extern unsigned long long g_423760;
extern unsigned long long g_423768;
extern unsigned long long g_423770;
extern unsigned long long g_423778;
extern unsigned long long g_423780;
extern unsigned long long g_423788;
extern unsigned long long g_423790;
extern unsigned int g_423798;

long long compare_random(char *a0, unsigned long long a1, unsigned long a2, unsigned long long a3)
{
    unsigned long long v43;  // rdx
    unsigned long long v45;  // rax
    unsigned long long v46;  // rdx
    unsigned long long v47;  // rdx
    unsigned long long v0;  // [bp-0x1108]
    unsigned long v1;  // [bp-0x1100]
    unsigned long long v2;  // [bp-0x10f8]
    char *v3;  // [bp-0x10f0]
    char v4;  // [bp-0x10e1]
    unsigned int v5;  // [bp-0x10e0]
    unsigned int v6;  // [bp-0x10dc]
    char *v7;  // [bp-0x10d8]
    unsigned long long v8;  // [bp-0x10d0]
    char *ptr;  // [bp-0x10c8]
    char *v10;  // [bp-0x10c0]
    unsigned long v11;  // [bp-0x10b8]
    unsigned long long v12;  // [bp-0x10b0]
    unsigned long long v13;  // [bp-0x10a8]
    unsigned long long v14;  // [bp-0x10a0]
    char v15;  // [bp-0x1098]
    char v16;  // [bp-0x1088]
    unsigned long v17;  // [bp-0x1078]
    unsigned long v18;  // [bp-0x1070]
    unsigned long v19;  // [bp-0x1068]
    unsigned long v20;  // [bp-0x1060]
    unsigned long v21;  // [bp-0x1058]
    unsigned long v22;  // [bp-0x1050]
    unsigned long v23;  // [bp-0x1048]
    unsigned long v24;  // [bp-0x1040]
    unsigned long v25;  // [bp-0x1038]
    unsigned long v26;  // [bp-0x1030]
    unsigned long v27;  // [bp-0x1028]
    unsigned int v28;  // [bp-0x1020]
    unsigned long v29;  // [bp-0x101c]
    unsigned long v30;  // [bp-0x1014]
    unsigned long v31;  // [bp-0x100c]
    unsigned long v32;  // [bp-0x1004]
    unsigned long v33;  // [bp-0xffc]
    unsigned long v34;  // [bp-0xff4]
    unsigned long v35;  // [bp-0xfec]
    unsigned long v36;  // [bp-0xfe4]
    unsigned long v37;  // [bp-0xfdc]
    unsigned long v38;  // [bp-0xfd4]
    unsigned long v39;  // [bp-0xfcc]
    unsigned int v40;  // [bp-0xfc4]
    char v41;  // [bp-0xfb8]

    v3 = a0;
    v2 = a1;
    v1 = a2;
    v0 = a3;
    v5 = 0;
    v7 = &v41;
    v8 = 4000;
    ptr = NULL;
    v29 = g_423740;
    v30 = g_423748;
    v31 = g_423750;
    v32 = g_423758;
    v33 = g_423760;
    v34 = g_423768;
    v35 = g_423770;
    v36 = g_423778;
    v37 = g_423780;
    v38 = g_423788;
    v39 = g_423790;
    v40 = g_423798;
    v17 = v29;
    v18 = v30;
    v19 = v31;
    v20 = v32;
    v21 = v33;
    v22 = v34;
    v23 = v35;
    v24 = v36;
    v25 = v37;
    v26 = v38;
    v27 = v39;
    v28 = v40;
    if (g_423229)
    {
        v10 = &v3[v2];
        v11 = v0 + v1;
        while (1)
        {
            v12 = (v2 + v0) * 3 + 2;
            if (v8 < v12)
            {
                v43 = v8 * 3 >> 1;
                if (v12 >= v43)
                    v43 = v12;
                v8 = v43;
                free(ptr);
                ptr = malloc(v8);
                v7 = ptr;
                if (!v7)
                {
                    v7 = &v41;
                    v8 = 4000;
                }
            }
            v13 = (v3 < v10 ? sub_407668(v7, v3, v8) + 1 : 0);
            v4 = v13 <= v8;
            if (v1 < v11)
                v45 = sub_407668((!v4 ? 0 : &v7[v13]), v1, (!v4 ? 0 : v8 - v13)) + 1;
            else
                v45 = 0;
            v14 = v45;
            if (v4 != 1 || v8 < v14 + v13)
            {
                v8 = v14 + v13;
                if (v8 <= 6148914691236517204)
                    v8 = v8 * 3 >> 1;
                free(ptr);
                ptr = sub_418024(v8);
                v7 = ptr;
                if (v3 < v10)
                    strxfrm(v7, v3, v13);
                if (v1 < v11)
                    strxfrm(&v7[v13], v1, v14);
            }
            if (v3 < v10)
                v3 = &v3[1 + strlen(v3)];
            if (v1 < v11)
                v1 += strlen(v1) + 1;
            if (v3 >= v10 && v1 >= v11)
                break;
            sub_40fb4b(v7, v13, &v17, v13);
            sub_40fb4b(&v7[v13], v14, &v29, &v7[v13]);
            if (!v5)
            {
                if (v14 > v13)
                    v46 = v13;
                else
                    v46 = v14;
                v5 = memcmp(v7, &v7[v13], v46);
                if (!v5)
                    v5 = (v13 > v14) - (v13 < v14);
            }
        }
        v2 = v13;
        v3 = v7;
        v0 = v14;
        v1 = &v7[v13];
    }
    sub_40fb4b(v3, v2, &v17, v2);
    sub_40fbaf(&v17, &v15, &v15);
    sub_40fb4b(v1, v0, &v29, v0);
    sub_40fbaf(&v29, &v16, &v16);
    v6 = memcmp(&v15, &v16, 16);
    if (v6)
    {
        free(ptr);
        return v6;
    }
    if (!v5)
    {
        if (v0 > v2)
            v47 = v2;
        else
            v47 = v0;
        v5 = memcmp(v3, v1, v47);
        if (!v5)
            v5 = (v2 > v0) - (v2 < v0);
    }
    v6 = v5;
    free(ptr);
    return v6;
}



// Function: debug_width @ 0x7ee4
long long debug_width(char *a0, char *a1)
{
    char *v3;  // rax
    char *v0;  // [bp-0x20]
    unsigned long i;  // [bp-0x10]

    v0 = a0;
    for (i = sub_41293d(v0, a1 - v0, 0, a1 - v0); v0 < a1; i += *(v3) == 9)
    {
        v3 = v0;
        v0 = v3 + 1;
    }
    return i;
}



// Function: mark_key @ 0x7f4c
int mark_key(unsigned long long a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x10]

    v1 = a0;
    i = a1;
    while (1)
    {
        v2 = v1 - 1;
        if (!v1)
            break;
        putchar_unlocked(32);
        v1 = v2;
    }
    if (!i)
        return printf(gettext("^ no match for key\n"));
    do
    {
        putchar_unlocked(95);
        i -= 1;
    } while (i);
    return putchar_unlocked(10);
}



// Function: key_numeric @ 0x7fc5
unsigned int key_numeric(char *a0)
{
    unsigned int v1;  // eax

    if (a0[50] || a0[52] || a0[53])
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: debug_key @ 0x8002
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_41c5c0;
extern char g_423260;

void debug_key(struct_0 *a0, struct_0 *a1)
{
    int v11;  // eax
    int v12;  // eax
    char v0;  // [bp-0x63]
    char v1;  // [bp-0x62]
    char v2;  // [bp-0x61]
    char *v3;  // [bp-0x60]
    char *v4;  // [bp-0x58]
    char *i;  // [bp-0x50]
    char *v6;  // [bp-0x48]
    char *v7;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]

    v7 = &a0->field_0->field_0;
    i = v7;
    v6 = v7 + a0->field_8 - 1;
    if (a1)
    {
        if (a1->field_0 != 0xffffffffffffffff)
            i = sub_4066f4(a0, a1);
        if (a1[1].field_0 != 0xffffffffffffffff)
            v6 = sub_406898(a0, a1);
        if ((char)a1[3].field_0 && a1->field_0 == 0xffffffffffffffff || *((char *)&a1[3].field_0 + 6) || (char)sub_407fc5(a1))
        {
            v0 = *(v6);
            for (*(v6) = 0; *((char *)(unsigned int)&(&g_423260)[sub_404042(*(i))]); i += 1);
            v3 = i;
            if (v6 < i)
            {
                v3 = v6;
            }
            else if (*((char *)&a1[3].field_0 + 6))
            {
                sub_4073f3(i, &v3);
            }
            else if (*((char *)&a1[3].field_0 + 4))
            {
                strtold(i, &v3);
                if (/* unsupported instruction */)
                {
                    amd64g_dirtyhelper_storeF80le((Reference vvar_395{s-40|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                    /* unsupported instruction */
                }
                else
                {
                    amd64g_dirtyhelper_storeF80le((Reference vvar_395{s-40|1b}), Reinterpret(F64->I64, nan<64>))
                    /* unsupported instruction */
                }
            }
            else if (*((char *)&a1[3].field_0 + 2) || *((char *)&a1[3].field_0 + 5))
            {
                if (i < v6 && *(i) == 45)
                    v11 = 1;
                else
                    v11 = 0;
                v4 = &i[v11];
                v1 = sub_406f64(&v4);
                if (v1 > 47)
                {
                    v2 = *(v4);
                    if (*((char *)&a1[3].field_0 + 5) && *((char *)(unsigned int)&(&g_41c5c0)[v2]))
                        v12 = 1;
                    else
                        v12 = 0;
                    v3 = &v4[v12];
                }
            }
            else
            {
                v3 = v6;
            }
            *(v6) = v0;
            v6 = v3;
        }
    }
    v8 = sub_407ee4(v7, i);
    v9 = sub_407ee4(i, v6);
    sub_407f4c(v8, v9);
    return;
}



// Function: debug_line @ 0x829e
typedef struct struct_0 {
    char padding_0[64];
    struct struct_0 *field_40;
} struct_0;

extern char g_423681;
extern char g_423682;
extern struct_0 *g_423688;

long long debug_line(struct_0 *a0)
{
    unsigned long v2;  // rax
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    struct_0 *iter;  // [bp-0x10]

    iter = g_423688;
    while (1)
    {
        do
        {
            v2 = (unsigned long long)sub_408002(a0, iter);
            if (!iter)
                return v2;
        } while ((iter = (struct_0 *)iter->field_40, iter));
        v3 = g_423682 ^ 1;
        if (!(char)v3)
            return v3;
        v4 = g_423681 ^ 1;
        if (!(char)v4)
            return v4;
    }
}



// Function: default_key_compare @ 0x8305
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char field_31;
    char padding_32[1];
    char field_33;
    char padding_34[2];
    char field_36;
    char padding_37[1];
    char field_38;
} struct_0;

unsigned int default_key_compare(struct_0 *a0)
{
    unsigned int v1;  // eax

    if (!a0->field_20 && !a0->field_28 && a0->field_30 != 1 && a0->field_31 != 1 && (char)sub_407fc5(a0) != 1 && a0->field_36 != 1 && a0->field_38 != 1 && a0->field_33 != 1)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: key_to_opts @ 0x839e
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char field_31;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
} struct_0;

extern char g_423360;
extern char g_423460;

char * key_to_opts(struct_0 *a0, char *a1)
{
    char *v0;  // [bp-0x18]
    char *v1;  // [bp-0x18]

    v0 = a1;
    if (a0->field_30 || a0->field_31)
    {
        v1 = v0 + 1;
        *(v0) = 98;
        v0 = v1;
    }
    if (a0->field_20 == &g_423460)
    {
        v1 = v0 + 1;
        *(v0) = 100;
        v0 = v1;
    }
    if (a0->field_28)
    {
        v1 = v0 + 1;
        *(v0) = 0x66;
        v0 = v1;
    }
    if (a0->field_34)
    {
        v1 = v0 + 1;
        *(v0) = 103;
        v0 = v1;
    }
    if (a0->field_35)
    {
        v1 = v0 + 1;
        *(v0) = 104;
        v0 = v1;
    }
    if (a0->field_20 == &g_423360)
    {
        v1 = v0 + 1;
        *(v0) = 105;
        v0 = v1;
    }
    if (a0->field_36)
    {
        v1 = v0 + 1;
        *(v0) = 77;
        v0 = v1;
    }
    if (a0->field_32)
    {
        v1 = v0 + 1;
        *(v0) = 110;
        v0 = v1;
    }
    if (a0->field_33)
    {
        v1 = v0 + 1;
        *(v0) = 82;
        v0 = v1;
    }
    if (a0->field_37)
    {
        v1 = v0 + 1;
        *(v0) = 114;
        v0 = v1;
    }
    if (a0->field_38)
    {
        v1 = v0 + 1;
        *(v0) = 86;
        v0 = v1;
    }
    *(v0) = 0;
    return v0;
}



// Function: key_warnings @ 0x84fe
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[40];
    char field_30;
    char field_31;
    char field_32;
    char field_33;
    char padding_34[1];
    char field_35;
    char padding_36[1];
    char field_37;
    char field_38;
    char field_39;
    char padding_3a[6];
    struct struct_0 *field_40;
} struct_0;

extern unsigned int g_423020;
extern char g_423220;
extern unsigned int g_423224;
extern char g_423228;
extern char g_423681;
extern char g_423682;
extern struct_0 *g_423688;

void key_warnings(unsigned long long *idx, char a1)
{
    char *v31;  // rax
    char *v32;  // rax
    char v41;  // al
    char v42;  // al
    unsigned long long *v45;  // rax
    char v46;  // al
    char v47;  // al
    char v48;  // al
    unsigned long long *v50;  // rax
    char *v33;  // rax
    char v51;  // al
    char v52;  // al
    char v53;  // al
    unsigned long v35;  // rdx
    char *v36;  // rax
    char v39;  // al
    char v0;  // [bp-0x141]
    char v1;  // [bp-0x140]
    char v2;  // [bp-0x13f]
    char v3;  // [bp-0x13e]
    char v4;  // [bp-0x13d]
    char v5;  // [bp-0x13c]
    char v6;  // [bp-0x13b]
    char v7;  // [bp-0x13a]
    char v8;  // [bp-0x139]
    struct_0 *iter;  // [bp-0x138]
    unsigned long long i;  // [bp-0x130]
    unsigned long v11;  // [bp-0x128]
    unsigned long long v12;  // [bp-0x120]
    unsigned long long v13;  // [bp-0x118]
    char *v14;  // [bp-0x110]
    char *v15;  // [bp-0x108]
    unsigned long long v16;  // [bp-0x100]
    unsigned long v17;  // [bp-0xf8]
    unsigned long v18;  // [bp-0xf0]
    unsigned long v19;  // [bp-0xe8]
    unsigned long v20;  // [bp-0xe0]
    unsigned long long v21;  // [bp-0xd8]
    unsigned long long v22;  // [bp-0xd0]
    unsigned long v23;  // [bp-0xc8]
    unsigned long v24;  // [bp-0xc0]
    unsigned long v25;  // [bp-0xb8]
    char v26;  // [bp-0xa8]
    char v27;  // [bp-0x88]
    char v28;  // [bp-0x58]
    char v29;  // [bp-0x57]

    v17 = *(idx);
    v18 = idx[1];
    v19 = idx[2];
    v20 = idx[3];
    v21 = idx[4];
    v22 = idx[5];
    v23 = idx[6];
    v24 = idx[7];
    v25 = idx[8];
    i = 1;
    v0 = 0;
    v1 = 0;
    v2 = 0;
    v3 = 0;
    for (iter = g_423688; iter; i += 1)
    {
        if ((char)sub_407fc5(iter))
        {
            if (iter->padding_34)
                v1 = 1;
            else
                v0 = 1;
        }
        if (iter->field_39)
        {
            v11 = iter->field_0;
            v13 = *((long long *)&iter->padding_8[8]);
            v14 = &v27;
            v15 = &v28;
            if (v11 == 0xffffffffffffffff)
                v11 += 1;
            v31 = sub_41287e(v11, &v26, &v26);
            v14 = stpcpy(stpcpy(v14, "+"), v31);
            v32 = sub_41287e(v11 + 1, &v26, v11 + 1);
            v15 = stpcpy(stpcpy(v15, "-k "), v32);
            if (*((long long *)&iter->padding_8[8]) != 0xffffffffffffffff)
            {
                v33 = sub_41287e(v13 + 1, &v26, v13 + 1);
                stpcpy(stpcpy(v14, " -"), v33);
                v35 = *((long long *)&iter->padding_8[16]) == 0xffffffffffffffff;
                v36 = sub_41287e(v13 + v35 + 1, &v26, v13 + v35 + 1);
                stpcpy(stpcpy(v15, ","), v36);
            }
            sub_415473(1, &v28);
            sub_415473(0, &v27);
            error(0, 0, gettext("obsolescent key %s used; consider %s instead"));
        }
        if (iter->field_0 != 0xffffffffffffffff && *((long long *)&iter->padding_8[8]) < iter->field_0)
            v39 = 1;
        else
            v39 = 0;
        v6 = v39;
        v6 &= 1;
        if (v6)
            error(0, 0, gettext("key %lu has zero width and will be ignored"));
        if ((char)sub_407fc5(iter) || iter->padding_36)
            v41 = 1;
        else
            v41 = 0;
        v7 = v41;
        v7 &= 1;
        if (!*((long long *)&iter->padding_8[8]) && *((long long *)&iter->padding_8[16]))
            v42 = 1;
        else
            v42 = 0;
        v8 = v42;
        v8 &= 1;
        if (v6 != 1 && a1 != 1 && g_423020 == 128 && v8 != 1 && (iter->field_30 != 1 && v7 != 1 || iter->field_30 != 1 && *((long long *)&iter->padding_8[0]) || iter->field_31 != 1 && *((long long *)&iter->padding_8[16])))
            error(0, 0, gettext("leading blanks are significant in key %lu; consider also specifying 'b'"));
        if (a1 != 1 && (char)sub_407fc5(iter))
        {
            v12 = iter->field_0 + 1;
            v16 = *((long long *)&iter->padding_8[8]) + 1;
            if (!v12)
                v12 += 1;
            if (!v16 || v12 < v16)
            {
                error(0, 0, gettext("key %lu is numeric and spans multiple fields"));
                if (iter->padding_34)
                    v3 = 1;
                else
                    v2 = 1;
            }
        }
        if (v21 && v21 == *((long long *)&iter->padding_8[24]))
            v21 = 0;
        if (v22 && v22 == *((long long *)&iter->padding_8[32]))
            v22 = 0;
        *((char *)&v23) = (iter->field_30 ^ 1) & (char)v23;
        *((char *)&v23 + 1) = (iter->field_31 ^ 1) & *((char *)((void*)&v23 + 1));
        *((char *)&v23 + 6) = (unsigned int)(iter->padding_36 ^ 1) & *((char *)((void*)&v23 + 6));
        *((char *)&v23 + 2) = (iter->field_32 ^ 1) & *((char *)((void*)&v23 + 2));
        *((char *)&v23 + 4) = (unsigned int)(iter->padding_34 ^ 1) & *((char *)((void*)&v23 + 4));
        *((char *)&v23 + 5) = (iter->field_35 ^ 1) & *((char *)((void*)&v23 + 5));
        *((char *)&v23 + 3) = (iter->field_33 ^ 1) & *((char *)((void*)&v23 + 3));
        *((char *)&v24) = (iter->field_38 ^ 1) & (char)v24;
        *((char *)&v23 + 7) = (iter->field_37 ^ 1) & *((char *)((void*)&v23 + 7));
        iter = iter->field_40;
    }
    v4 = 0;
    if (v2)
    {
        if (g_423020 == 128)
        {
            if (g_423224 != 128)
            {
                v45 = __ctype_b_loc();
                v46 = sub_404042((char)g_423224);
                if (!(*((short *)(v46 * 2 + *(v45))) & 1))
                    goto LABEL_408ce5;
                v47 = 1;
            }
            else
            {
LABEL_408ce5:
                v47 = 0;
            }
            v48 = v47 & 1;
        }
        else
        {
            v48 = g_423020 == g_423224;
        }
        if (v48)
        {
            v28 = g_423224;
            v29 = 0;
            sub_4154a0(&v28);
            error(0, 0, gettext("field separator %s is treated as a group separator in numbers"));
            v4 = 1;
        }
    }
    if (v2 || v3)
    {
        if (g_423020 == 128)
        {
            if (g_423224 != 128)
            {
                v50 = __ctype_b_loc();
                v51 = sub_404042(g_423220);
                if (!(*((short *)(v51 * 2 + *(v50))) & 1))
                    goto LABEL_408db6;
                v52 = 1;
            }
            else
            {
LABEL_408db6:
                v52 = 0;
            }
            v53 = v52 & 1;
        }
        else
        {
            v53 = g_423220 == g_423020;
        }
        if (v53)
        {
            v28 = g_423220;
            v29 = 0;
            sub_4154a0(&v28);
            error(0, 0, gettext("field separator %s is treated as a decimal point in numbers"));
            v4 = 1;
        }
        else if (g_423020 == 45)
        {
            v28 = g_423020;
            v29 = 0;
            sub_4154a0(&v28);
            error(0, 0, gettext("field separator %s is treated as a minus sign in numbers"));
        }
        else if (v3 && g_423020 == 43)
        {
            v28 = g_423020;
            v29 = 0;
            sub_4154a0(&v28);
            error(0, 0, gettext("field separator %s is treated as a plus sign in numbers"));
        }
    }
    if ((v0 || v1) && v4 != 1)
    {
        v28 = g_423220;
        v29 = 0;
        sub_4154a0(&v28);
        if (g_423220 != g_423020)
            gettext("note ");
        error(0, 0, gettext("%snumbers use %s as a decimal point in this locale"));
    }
    if (v0 && g_423228)
        error(0, 0, gettext("the multi-byte number group separator in this locale is not supported"));
    if ((char)sub_408305(&v17) != 1 || *((char *)((void*)&v23 + 7)) && (g_423681 || g_423682) && g_423688)
    {
        v5 = *((char *)((void*)&v23 + 7));
        if (g_423681 != 1 && g_423682 != 1)
            *((char *)&v23 + 7) = 0;
        sub_40839e(&v17, &v28);
        error(0, 0, ngettext("option '-%s' is ignored", "options '-%s' are ignored", sub_404094(strlen(&v28))));
        *((char *)&v23 + 7) = v5;
    }
    if (!*((char *)((void*)&v23 + 7)))
    {
        return;
    }
    else if (g_423681 == 1)
    {
        return;
    }
    else if (g_423682 == 1)
    {
        return;
    }
    else if (g_423688)
    {
        error(0, 0, gettext("option '-r' only applies to last-resort comparison"));
        return;
    }
    else
    {
        return;
    }
}



// Function: keycompare @ 0x9116
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[8];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char padding_31[1];
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_0 *field_40;
} struct_0;

extern char g_423229;
extern char g_423260;
extern struct_0 *g_423688;

unsigned int keycompare(struct_0 *index, struct_0 *idx)
{
    char *v24;  // rax
    char *v25;  // rax
    unsigned int v34;  // eax
    char v26;  // al
    char v27;  // al
    char *v28;  // rax
    char v29;  // al
    char *v30;  // rax
    unsigned long v31;  // rdx
    char v0;  // [bp-0x104e]
    char v1;  // [bp-0x104d]
    unsigned int j;  // [bp-0x104c]
    struct_0 *iter;  // [bp-0x1048]
    char *node;  // [bp-0x1040]
    char *iter2;  // [bp-0x1038]
    char *v6;  // [bp-0x1030]
    char *v7;  // [bp-0x1028]
    char *v8;  // [bp-0x1020]
    char *v9;  // [bp-0x1018]
    unsigned long v10;  // [bp-0x1010]
    unsigned long v11;  // [bp-0x1010]
    unsigned long v12;  // [bp-0x1008]
    unsigned long v13;  // [bp-0x1008]
    char *v14;  // [bp-0x1000]
    char *v15;  // [bp-0x1000]
    unsigned long iter1;  // [bp-0xff8]
    unsigned long long v17;  // [bp-0xff0]
    unsigned long long v18;  // [bp-0xfe8]
    unsigned long v19;  // [bp-0xfe0]
    unsigned long v20;  // [bp-0xfd8]
    unsigned long long v21;  // [bp-0xfd0]
    char v22;  // [bp-0xfc8]

    iter = g_423688;
    node = index->field_10;
    iter2 = idx->field_10;
    v6 = index->padding_18;
    v7 = idx->padding_18;
    while (1)
    {
        v17 = iter->field_28;
        v18 = iter->field_20;
        if (node <= v6)
            v24 = v6;
        else
            v24 = node;
        v6 = v24;
        if (iter2 <= v7)
            v25 = v7;
        else
            v25 = iter2;
        v7 = v25;
        if (g_423229 || (char)sub_407fc5(iter) || iter->field_36 || iter->field_33 || iter->field_38)
        {
            if (v18 || v17)
            {
                v21 = v20 + v19 + 2;
                if (v21 <= 4000)
                {
                    v8 = &v22;
                    v15 = NULL;
                }
                else
                {
                    v15 = sub_418024(v21);
                    v8 = v15;
                }
                v9 = &v8[1 + v19];
                iter1 = 0;
                for (v10 = iter1; iter1 < v19; iter1 += 1)
                {
                    if (!v18 || *((char *)(v18 + (sub_404042(node[iter1]) & 0xff))) != 1)
                    {
                        v11 = v10 + 1;
                        v8[v10] = (!v17 ? node[iter1] : *((char *)(v17 + (sub_404042(node[iter1]) & 0xff))));
                        v10 = v11;
                    }
                }
                v8[v10] = 0;
                iter1 = 0;
                for (v12 = iter1; iter1 < v20; iter1 += 1)
                {
                    if (!v18 || *((char *)(v18 + (sub_404042(iter2[iter1]) & 0xff))) != 1)
                    {
                        v13 = v12 + 1;
                        v9[v12] = (!v17 ? iter2[iter1] : *((char *)(v17 + (sub_404042(iter2[iter1]) & 0xff))));
                        v12 = v13;
                    }
                }
                v9[v12] = 0;
            }
            else
            {
                v8 = node;
                v10 = v19;
                v0 = v8[v10];
                v8[v10] = 0;
                v9 = iter2;
                v12 = v20;
                v1 = v9[v12];
                v9[v12] = 0;
                v15 = v14;
            }
            if (iter->field_32)
            {
                j = sub_4071a3(v8, v9);
            }
            else
            {
                if (iter->field_34)
                {
                    j = sub_4072d6(v8, v9);
                }
                else
                {
                    if (iter->field_35)
                    {
                        j = sub_4070e2(v8, v9);
                    }
                    else
                    {
                        if (iter->field_36)
                        {
                            v34 = (unsigned int)sub_4073f3(v8, NULL);
                            j = (unsigned int)(unsigned int *)(v34 - (char *)sub_4073f3(v9, NULL));
                        }
                        else
                        {
                            j = (!iter->field_33 ? (!iter->field_38 ? (!v10 ? -(v12) : (!v12 ? 1 : (unsigned int)sub_4187ec(v8, v10 + 1, v9, v12 + 1))) : (unsigned int)sub_410461(v8, v10, v9, v12)) : (int)sub_407755(v8, v10, v9, v12));
                        }
                    }
                }
            }
            if (v18 || v17)
            {
                free(v15);
            }
            else
            {
                v8[v10] = v0;
                v9[v12] = v1;
            }
        }
        else if (v18)
        {
            if (v17)
            {
                while (1)
                {
                    for (v20 = v7 - iter2; node < v6 && *((char *)(v18 + (sub_404042(*(node)) & 0xff))); node += 1);
                    for (; iter2 < v7 && *((char *)(v18 + (sub_404042(*(iter2)) & 0xff))); iter2 += 1);
                    if (node >= v6 || iter2 >= v7)
                        break;
                    v26 = sub_404042(*((char *)(v17 + (sub_404042(*(node)) & 0xff))));
                    j = v26 - (sub_404042(*((char *)(v17 + (sub_404042(*(iter2)) & 0xff)))) & 0xff);
                    if (j)
                    {
                        j = j;
                        return (!iter->field_37 ? j : -(j));
                    }
                    node += 1;
                    iter2 += 1;
                }
                j = (node < v6) - (iter2 < v7);
            }
            else
            {
                while (1)
                {
                    for (v19 = v6 - node; node < v6 && *((char *)(v18 + (sub_404042(*(node)) & 0xff))); node += 1);
                    for (; iter2 < v7 && *((char *)(v18 + (sub_404042(*(iter2)) & 0xff))); iter2 += 1);
                    if (node >= v6 || iter2 >= v7)
                        break;
                    v27 = sub_404042(*(node));
                    j = v27 - (sub_404042(*(iter2)) & 0xff);
                    if (j)
                    {
                        j = j;
                        return (!iter->field_37 ? j : -(j));
                    }
                    node += 1;
                    iter2 += 1;
                }
                j = (node < v6) - (iter2 < v7);
            }
        }
        else
        {
            if (!v19)
            {
                j = -(v20);
            }
            else if (v20)
            {
                if (v17)
                {
                    do
                    {
                        if (node < v6 && iter2 < v7)
                            goto LABEL_409acb;
                        else
                            goto LABEL_409bc1;
LABEL_409acb:
                        v28 = node;
                        node = v28 + 1;
                        v29 = sub_404042(*((char *)(v17 + (sub_404042(*(v28)) & 0xff))));
                        v30 = iter2;
                        iter2 = v30 + 1;
                        j = v29 - (sub_404042(*((char *)(v17 + (sub_404042(*(v30)) & 0xff)))) & 0xff);
                    } while (!j);
                }
                else
                {
                    if (v20 > v19)
                        v31 = v19;
                    else
                        v31 = v20;
                    j = memcmp(node, iter2, v31);
                    if (j)
                        break;
                }
LABEL_409bc1:
                j = (v19 < v20 ? 0xffffffff : v19 != v20);
            }
            else
            {
                j = 1;
                break;
            }
        }
        if (j)
            break;
        iter = iter->field_40;
        if (!iter)
            return 0;
        if (iter->field_10 != 0xffffffffffffffff)
        {
            v6 = sub_406898(index, iter);
            v7 = sub_406898(idx, iter);
        }
        else
        {
            v6 = (char *)(index->padding_8 - 1 + index->field_0);
            v7 = (char *)(idx->padding_8 - 1 + idx->field_0);
        }
        if (iter->field_0 != 0xffffffffffffffff)
        {
            node = sub_4066f4(index, iter);
            iter2 = sub_4066f4(idx, iter);
        }
        else
        {
            node = index->field_0;
            if (iter->field_30)
            {
                for (iter2 = idx->field_0; node < v6 && *((char *)(unsigned int)&(&g_423260)[sub_404042(*(node))]); node += 1);
                for (; iter2 < v7 && *((char *)(unsigned int)&(&g_423260)[sub_404042(*(iter2))]); iter2 += 1);
            }
        }
    }
    return (!iter->field_37 ? j : -(j));
}



// Function: compare @ 0x9e2a
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[8];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char padding_31[1];
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_0 *field_40;
} struct_0;

extern char g_423229;
extern char g_423680;
extern char g_423681;
extern char g_423682;
extern unsigned long long g_423688;

unsigned int compare(struct_0 *a0, struct_0 *a1)
{
    unsigned long v4;  // rdx
    unsigned int v0;  // [bp-0x1c]
    unsigned long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10]

    if (g_423688)
    {
        v0 = sub_409116(a0, a1);
        if (v0 || g_423682 || g_423681)
            return v0;
    }
    v1 = (unsigned long)(a0->padding_8 - 1);
    v2 = (unsigned long)(a1->padding_8 - 1);
    if (!v1)
    {
        v0 = -(v2);
    }
    else if (!v2)
    {
        v0 = 1;
    }
    else if (g_423229)
    {
        v0 = sub_4187ec(a0->field_0, v1 + 1, a1->field_0, v2 + 1);
    }
    else
    {
        if (v2 > v1)
            v4 = v1;
        else
            v4 = v2;
        v0 = memcmp(a0->field_0, a1->field_0, v4);
        if (!v0)
            v0 = (v1 < v2 ? 0xffffffff : v1 != v2);
    }
    if (!g_423680)
        return v0;
    return -(v0);
}



// Function: write_line @ 0x9f73
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[64];
    struct struct_0 *field_40;
} struct_0;

extern char g_423010;
extern char g_423698;

void* write_line(struct_0 *a0, FILE *a1, unsigned long long a2)
{
    void* v6;  // rax
    char v0;  // [bp-0x29]
    void* i;  // [bp-0x28]
    void* v2;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]
    void* v4;  // [bp-0x10]

    v2 = *((long long *)&a0->padding_0[0]);
    v3 = *((long long *)&a0->padding_0[8]);
    v4 = v3 + v2;
    if (!a2 && g_423698)
    {
        i = v2;
        while (i < v4)
        {
            v6 = i;
            i = v6 + 1;
            v0 = *((char *)v6);
            if (v0 == 9)
            {
                v0 = 62;
            }
            else if (i == v4)
            {
                v0 = 10;
            }
            if (fputc_unlocked(v0, a1) == 0xffffffff)
                sub_404464(gettext("write failed"), a2); /* do not return */
        }
        return sub_40829e(a0);
    }
    *((char *)v4 - 1) = g_423010;
    if (v3 == fwrite_unlocked(v2, 1, v3, a1))
    {
        *((char *)v4 - 1) = 0;
        return v4 - 1;
    }
    sub_404464(gettext("write failed"), a2); /* do not return */
}



// Function: check @ 0xa0b1
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    struct struct_2 *field_0;
    char field_8;
    char padding_9[7];
    long long field_10;
    unsigned long long field_18;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char field_30;
} struct_1;

typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_423018;
extern FILE *stderr;
extern unsigned long long g_423660;
extern char g_423682;
extern unsigned long long g_423688;

long long check(unsigned int a0, char a1)
{
    unsigned long long v18;  // rdx
    char *v19;  // rax
    unsigned long v20;  // rax
    char v0;  // [bp-0xda]
    char v1;  // [bp-0xd9]
    unsigned long long i;  // [bp-0xd8]
    unsigned long long v3;  // [bp-0xd0]
    struct_0 *iter;  // [bp-0xc8]
    FILE *v5;  // [bp-0xc0]
    unsigned long long v6;  // [bp-0xb8]
    struct_0 *v7;  // [bp-0xb0]
    struct_0 *v8;  // [bp-0xa8]
    long long v9;  // [bp-0xa0]
    struct_0 ptr;  // [bp-0x98]
    unsigned long long v11;  // [bp-0x90]
    unsigned long long v12;  // [bp-0x88]
    unsigned long long v13;  // [bp-0x80]
    struct_1 v14;  // [bp-0x78]
    unsigned long v15;  // [bp-0x68]
    char v16;  // [bp-0x38]

    v5 = sub_40505e(a0, "r");
    i = 0;
    v3 = 0;
    v6 = g_423688;
    v1 = !g_423682;
    v1 &= 1;
    v0 = 1;
    v18 = g_423660;
    if (g_423660 < *((long long *)&g_423018))
        v18 = *((long long *)&g_423018);
    sub_406618(&v14, 32, v18);
    ptr = (struct_0)0;
    while ((char)sub_406a5e(&v14, v5, a0))
    {
        iter = sub_4066d0(&v14);
        v7 = (char *)iter + -(v15 * 32);
        if (i && v1 <= sub_409e2a(&ptr, (char *)iter - 32))
        {
            if (a1 == 99)
            {
                v8 = (char *)iter - 32;
                v9 = v3 + (sub_4066d0(&v14) - (char *)v8 >> 5);
                sub_41287e(v9, &v16, &v16);
                v19 = gettext("%s: %s:%s: disorder: ");
                fprintf(stderr, v19);
                v20 = gettext("standard error");
                sub_409f73(v8, stderr, v20);
            }
            v0 = 0;
            break;
        }
        else
        {
            do
            {
                iter = (char *)iter - 32;
                if (iter <= v7)
                {
                    v3 += v15;
                    if (i < iter->field_8)
                    {
                        do
                        {
                            i *= 2;
                            if (!i)
                            {
                                i = iter->field_8;
                                break;
                            }
                        } while (i < iter->field_8);
                        free(ptr);
                        ptr = (struct_0)sub_418024(i);
                    }
                    memcpy(ptr, iter->field_0, iter->field_8);
                    v11 = iter->field_8;
                    if (v6)
                    {
                        v12 = iter->field_10 - iter->field_0 + ptr;
                        v13 = *((long long *)&iter->field_18) - iter->field_0 + ptr;
                    }
                }
            } while (v1 > sub_409e2a(iter, (char *)iter - 32));
        }
    }
    sub_4050b7(v5, a0);
    free(v14);
    free(ptr);
    return v0;
}



// Function: open_input_files @ 0xa4a1
typedef struct FILE {
} FILE;

int open_input_files(unsigned long a0, long long a1, unsigned long long *a2)
{
    unsigned int v3;  // rsi
    FILE *v4;  // rax
    int i;  // [bp-0x14]
    unsigned long v1;  // [bp-0x10]

    *(a2) = sub_41819d(a1, 8);
    v1 = *(a2);
    for (i = 0; a1 > i; i += 1)
    {
        if (*((long long *)(a0 + i * 16 + 8)) && *((char *)(*((long long *)(a0 + i * 16 + 8)) + 12)))
            v4 = sub_40551c(*((long long *)(a0 + i * 16 + 8)), v3, i * 16);
        else
            v4 = sub_404ee4(*((long long *)(a0 + i * 16)), "r");
        *((FILE **)(v1 + i * 8)) = v4;
        if (!*((long long *)(v1 + i * 8)))
            return i;
    }
    return i;
}



// Function: mergefps @ 0xa5c0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
} struct_0;

extern char g_423018;
extern unsigned long long g_423660;
extern char g_423682;
extern unsigned long long g_423688;

void mergefps(unsigned long a0, unsigned long long a1, unsigned long long a2, FILE *a3, unsigned long long a4, void* ptr)
{
    unsigned long long v26;  // rdx
    unsigned long long *v27;  // rax
    unsigned long v36;  // rbx
    unsigned long v37;  // rbx
    unsigned long long *v28;  // rcx
    unsigned long v29;  // rdx
    unsigned long long *v30;  // rax
    unsigned long long *v31;  // rcx
    unsigned long v32;  // rdx
    unsigned long long *index;  // rdx
    unsigned long long *idx;  // rax
    unsigned long v35;  // rbx
    unsigned long long iter2;  // [bp-0x100]
    unsigned long long iter;  // [bp-0xf8]
    unsigned int v2;  // [bp-0xdc]
    struct_0 *v3;  // [bp-0xd8]
    unsigned long long v4;  // [bp-0xd0]
    unsigned long long iter1;  // [bp-0xc8]
    unsigned long long node;  // [bp-0xc0]
    unsigned long long v7;  // [bp-0xb8]
    unsigned long long v8;  // [bp-0xb0]
    unsigned long long l;  // [bp-0xa8]
    void* v10;  // [bp-0xa0]
    void* ptr2;  // [bp-0x98]
    void* v12;  // [bp-0x90]
    void* v13;  // [bp-0x88]
    unsigned long long v14;  // [bp-0x80]
    char *idx1;  // [bp-0x78]
    unsigned long long v16;  // [bp-0x70]
    unsigned long long v17;  // [bp-0x68]
    unsigned long long idx2;  // [bp-0x60]
    unsigned long v19;  // [bp-0x58]
    unsigned long long v20;  // [bp-0x50]
    struct_0 ptr1;  // [bp-0x48]
    unsigned long long n;  // [bp-0x40]
    unsigned long long v23;  // [bp-0x38]
    unsigned long long v24;  // [bp-0x30]

    iter = a1;
    iter2 = a2;
    v10 = sub_41819d(iter2, 56);
    v3 = NULL;
    v4 = 0;
    ptr2 = sub_41819d(iter2, 8);
    v12 = sub_41819d(iter2, 8);
    v13 = sub_41819d(iter2, 8);
    v14 = g_423688;
    ptr1 = (struct_0)0;
    for (iter1 = 0; iter1 < iter2; iter1 += 1)
    {
        v26 = g_423660 / iter2;
        if (v26 < *((long long *)&g_423018))
            v26 = *((long long *)&g_423018);
        sub_406618(v10 + iter1 * 56, 32, v26);
        if ((char)sub_406a5e(v10 + iter1 * 56, *((long long *)((char *)ptr + 8 * iter1)), *((long long *)(a0 + iter1 * 16))))
        {
            v20 = sub_4066d0(v10 + iter1 * 56);
            *((unsigned long long *)((char *)ptr2 + 8 * iter1)) = v20 - 32;
            *((unsigned long long *)((char *)v12 + 8 * iter1)) = v20 + -(*((long long *)((char *)v10 + 56 * iter1 + 16)) * 32);
        }
        else
        {
            sub_4050b7(*((long long *)((char *)ptr + 8 * iter1)), *((long long *)(a0 + iter1 * 16)));
            if (iter1 < iter)
            {
                iter -= 1;
                sub_405782(*((long long *)(a0 + iter1 * 16)));
            }
            free(*((long long *)((char *)v10 + 56 * iter1)));
            iter2 -= 1;
            for (node = iter1; node < iter2; node += 1)
            {
                v27 = a0 + (node + 1) * 16;
                v28 = node * 16 + a0;
                v29 = v27[1];
                *(v28) = *(v27);
                v28[1] = v29;
                *((long long *)((char *)ptr + 8 * node)) = *((long long *)((char *)ptr + 8 * node + 8));
            }
        }
    }
    for (iter1 = 0; iter1 < iter2; iter1 += 1)
    {
        *((unsigned long long *)(8 * iter1 + (char *)v13)) = iter1;
    }
    for (iter1 = 1; iter1 < iter2; iter1 += 1)
    {
        if (sub_409e2a(*((long long *)((char *)ptr2 + 8 * *((long long *)((char *)v13 + 8 * iter1 - 8)))), *((long long *)((char *)ptr2 + 8 * *((long long *)((char *)v13 + 8 * iter1))))) > 0)
        {
            v19 = *((long long *)((char *)v13 + 8 * iter1 - 8));
            *((long long *)((char *)v13 + 8 * iter1 - 8)) = *((long long *)((char *)v13 + 8 * iter1));
            *((unsigned long *)(8 * iter1 + (char *)v13)) = v19;
            iter1 = 0;
        }
    }
    while (iter2)
    {
        idx1 = *((long long *)((char *)ptr2 + 8 * *((long long *)v13)));
        if (g_423682)
        {
            if (v3 && sub_409e2a(v3, idx1))
            {
                v3 = NULL;
                sub_409f73(&ptr1, a3, a4);
            }
            if (!v3)
            {
                v3 = &ptr1;
                if (v4 < *((long long *)&idx1[8]))
                {
                    do
                    {
                        if (!v4)
                        {
                            v4 = *((long long *)&idx1[8]);
                            break;
                        }
                    } while ((v4 *= 2, v4 < *((long long *)&idx1[8])));
                    free(ptr1);
                    ptr1 = (struct_0)sub_418024(v4);
                }
                n = *((long long *)&idx1[8]);
                memcpy(ptr1, *((long long *)&idx1), n);
                if (v14)
                {
                    v23 = (unsigned long long)(*((long long *)&idx1[16]) - *((long long *)&idx1) + ptr1);
                    v24 = (unsigned long long)(*((long long *)&idx1[24]) - *((long long *)&idx1) + ptr1);
                }
            }
        }
        else
        {
            sub_409f73(idx1, a3, a4);
        }
        if (idx1 > *((long long *)((char *)v12 + 8 * *((long long *)v13))))
        {
            *((char **)((char *)ptr2 + 8 * *((long long *)v13))) = idx1 - 32;
            goto LABEL_40b0ea;
        }
        else if ((char)sub_406a5e(v10 + *((long long *)v13) * 56, *((long long *)((char *)ptr + 8 * *((long long *)v13))), *((long long *)(a0 + *((long long *)v13) * 16))))
        {
            v16 = sub_4066d0(v10 + *((long long *)v13) * 56);
            *((unsigned long long *)((char *)ptr2 + 8 * *((long long *)v13))) = v16 - 32;
            *((unsigned long long *)((char *)v12 + 8 * *((long long *)v13))) = v16 + -(*((long long *)((char *)v10 + 56 * *((long long *)v13) + 16)) * 32);
LABEL_40b0ea:
            v7 = 1;
            v8 = iter2;
            l = v7;
            for (v17 = *((long long *)v13); v7 < v8; l = v8 + v7 >> 1)
            {
                v2 = sub_409e2a(*((long long *)((char *)ptr2 + 8 * v17)), *((long long *)((char *)ptr2 + 8 * *((long long *)((char *)v13 + 8 * l)))));
                if (v2 < 0 || !v2 && v17 < *((long long *)((char *)v13 + 8 * l)))
                    v8 = l;
                else
                    v7 = l + 1;
            }
            idx2 = v7 - 1;
            for (node = 0; node < idx2; node += 1)
            {
                *((long long *)((char *)v13 + 8 * node)) = *((long long *)((char *)v13 + 8 * node + 8));
            }
            *((unsigned long long *)(8 * idx2 + (char *)v13)) = v17;
        }
        else
        {
            for (iter1 = 1; iter1 < iter2; iter1 += 1)
            {
                if (*((long long *)((char *)v13 + 8 * iter1)) > *((long long *)v13))
                    *((unsigned long long *)((char *)v13 + 8 * iter1)) = *((long long *)((char *)v13 + 8 * iter1)) - 1;
            }
            iter2 -= 1;
            sub_4050b7(*((long long *)((char *)ptr + 8 * *((long long *)v13))), *((long long *)(a0 + *((long long *)v13) * 16)));
            if (iter > *((long long *)v13))
            {
                iter -= 1;
                sub_405782(*((long long *)(a0 + *((long long *)v13) * 16)));
            }
            free(*((long long *)((char *)v10 + 56 * *((long long *)v13))));
            for (iter1 = *((long long *)v13); iter1 < iter2; iter1 += 1)
            {
                *((long long *)((char *)ptr + 8 * iter1)) = *((long long *)((char *)ptr + 8 * iter1 + 8));
                v30 = a0 + (iter1 + 1) * 16;
                v31 = iter1 * 16 + a0;
                v32 = v30[1];
                *(v31) = *(v30);
                v31[1] = v32;
                index = (iter1 + 1) * 56 + v10;
                idx = v10 + iter1 * 56;
                v35 = index[1];
                *(idx) = *(index);
                idx[1] = v35;
                v36 = index[3];
                idx[2] = index[2];
                idx[3] = v36;
                v37 = index[5];
                idx[4] = index[4];
                idx[5] = v37;
                idx[6] = index[6];
                *((long long *)((char *)ptr2 + 8 * iter1)) = *((long long *)((char *)ptr2 + 8 * iter1 + 8));
                *((long long *)((char *)v12 + 8 * iter1)) = *((long long *)((char *)v12 + 8 * iter1 + 8));
            }
            for (iter1 = 0; iter1 < iter2; iter1 += 1)
            {
                *((long long *)((char *)v13 + 8 * iter1)) = *((long long *)((char *)v13 + 8 * iter1 + 8));
            }
        }
    }
    if (g_423682 && v3)
    {
        sub_409f73(&ptr1, a3, a4);
        free(ptr1);
    }
    sub_4050b7(a3, a4);
    free(ptr);
    free(v10);
    free(v13);
    free(v12);
    free(ptr2);
    return;
}



// Function: mergefiles @ 0xb344
typedef struct FILE {
} FILE;

long long mergefiles(unsigned int a0, unsigned long long a1, unsigned long a2, FILE *a3, unsigned long long a4)
{
    unsigned long v3;  // rax
    void* v0;  // [bp-0x30], Other Possible Types: unsigned long long
    unsigned long v1;  // [bp-0x28]

    v1 = sub_40a4a1(a0, a2, &v0);
    if (v1 >= a2)
    {
        sub_40a5c0(a0, a1, v1, a3, a4, v0);
        return v1;
    }
    else if (v1 <= 1)
    {
        v3 = gettext("open failed");
        sub_404464(v3, *((long long *)(a0 + v1 * 16))); /* do not return */
    }
    else
    {
        sub_40a5c0(a0, a1, v1, a3, a4, v0);
        return v1;
    }
}



// Function: mergelines @ 0xb413
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[8];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char padding_31[1];
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_0 *field_40;
} struct_0;

long long mergelines(struct_0 *a0, unsigned long long a1, struct_0 *a2)
{
    struct_0 *index;  // rsi
    unsigned long v7;  // rdx
    unsigned long v16;  // rdx
    unsigned long v8;  // rax
    unsigned long v9;  // rdx
    struct_0 *idx;  // rsi
    unsigned long v11;  // rdx
    unsigned long v12;  // rdx
    struct_0 *idx1;  // rsi
    unsigned long v14;  // rdx
    unsigned long v15;  // rax
    struct_0 *node;  // [bp-0x40]
    struct_0 *iter;  // [bp-0x30]
    unsigned long long iter1;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    struct_0 *v4;  // [bp-0x10]

    iter = a0;
    node = a2;
    iter1 = a1 >> 1;
    v3 = a1 - iter1;
    v4 = (char *)iter + -(iter1 * 32);
    while (1)
    {
        if (sub_409e2a((char *)node - 32, (char *)v4 - 32) <= 0)
        {
            node = (char *)node - 32;
            iter = (char *)iter - 32;
            index = node;
            v7 = (unsigned long)index->padding_8;
            iter->field_0 = index->field_0;
            *((unsigned long *)&iter->padding_8[0]) = v7;
            v8 = index->field_10;
            v9 = (unsigned long)index->padding_18;
            iter->field_10 = v8;
            *((unsigned long *)&iter->padding_18[0]) = v9;
            iter1 -= 1;
            if (!iter1)
                return v8;
        }
        else
        {
            v4 = (char *)v4 - 32;
            iter = (char *)iter - 32;
            idx = v4;
            v11 = (unsigned long)idx->padding_8;
            iter->field_0 = idx->field_0;
            *((unsigned long *)&iter->padding_8[0]) = v11;
            v12 = (unsigned long)idx->padding_18;
            iter->field_10 = idx->field_10;
            *((unsigned long *)&iter->padding_18[0]) = v12;
            v3 -= 1;
            if (!v3)
            {
                do
                {
                    node = (char *)node - 32;
                    iter = (char *)iter - 32;
                    idx1 = node;
                    v14 = (unsigned long)idx1->padding_8;
                    iter->field_0 = idx1->field_0;
                    *((unsigned long *)&iter->padding_8[0]) = v14;
                    v15 = idx1->field_10;
                    v16 = (unsigned long)idx1->padding_18;
                    iter->field_10 = v15;
                    *((unsigned long *)&iter->padding_18[0]) = v16;
                    iter1 -= 1;
                } while (iter1);
                return v15;
            }
        }
    }
}



// Function: sequential_sort @ 0xb536
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[8];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char padding_31[1];
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_0 *field_40;
} struct_0;

long long sequential_sort(struct_0 *a0, unsigned long long a1, struct_0 *a2, char a3)
{
    struct_0 *idx;  // rsi
    struct_0 *idx1;  // rsi
    struct_0 *v19;  // rcx
    unsigned long v20;  // rdx
    unsigned long v21;  // rdx
    struct_0 *index;  // rsi
    struct_0 *v23;  // rcx
    unsigned long v24;  // rdx
    unsigned long v25;  // rdx
    struct_0 *idx2;  // rsi
    struct_0 *v27;  // rcx
    struct_0 *v10;  // rcx
    unsigned long v28;  // rdx
    long long v29;  // rax
    unsigned long v30;  // rdx
    struct_0 *v31;  // rsi
    struct_0 *v32;  // rcx
    unsigned long v33;  // rdx
    unsigned long v34;  // rdx
    unsigned long v11;  // rdx
    unsigned long v12;  // rdx
    struct_0 *v13;  // rsi
    struct_0 *v14;  // rcx
    unsigned long v15;  // rdx
    long long v16;  // rax
    unsigned long v17;  // rdx
    int v0;  // [bp-0x3c]
    struct_0 *v1;  // [bp-0x38]
    struct_0 *v2;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x28]
    long long v4;  // [bp-0x20]
    struct_0 *v5;  // [bp-0x18]
    struct_0 *v6;  // [bp-0x10]

    if (a1 == 2)
    {
        v0 = sub_409e2a((char *)a0 - 32, (char *)a0 - 64) > 0;
        if (a3)
        {
            idx = &(&a0->field_0)[4 * ~(v0)];
            v10 = (char *)a2 - 32;
            v11 = (unsigned long)idx->padding_8;
            v10->field_0 = idx->field_0;
            *((unsigned long *)&v10->padding_8[0]) = v11;
            v12 = (unsigned long)idx->padding_18;
            v10->field_10 = idx->field_10;
            *((unsigned long *)&v10->padding_18[0]) = v12;
            v13 = 32 * v0 + (char *)a0 - 64;
            v14 = (char *)a2 - 64;
            v15 = (unsigned long)v13->padding_8;
            v14->field_0 = v13->field_0;
            *((unsigned long *)&v14->padding_8[0]) = v15;
            v16 = v13->field_10;
            v17 = (unsigned long)v13->padding_18;
            v14->field_10 = v16;
            *((unsigned long *)&v14->padding_18[0]) = v17;
            return v16;
        }
        if (!v0)
            return v29;
        idx1 = (char *)a2 - 32;
        v19 = a0;
        v20 = *((long long *)((char *)v19 - 24));
        idx1->field_0 = *((long long *)((char *)v19 - 32));
        *((unsigned long *)&idx1->padding_8[0]) = v20;
        v21 = *((long long *)((char *)v19 - 8));
        idx1->field_10 = *((long long *)((char *)v19 - 16));
        *((unsigned long *)&idx1->padding_18[0]) = v21;
        index = (char *)a0 - 32;
        v23 = a0;
        v24 = *((long long *)((char *)v23 - 56));
        index->field_0 = *((long long *)((char *)v23 - 64));
        *((unsigned long *)&index->padding_8[0]) = v24;
        v25 = *((long long *)((char *)v23 - 40));
        index->field_10 = *((long long *)((char *)v23 - 48));
        *((unsigned long *)&index->padding_18[0]) = v25;
        idx2 = (char *)a0 - 64;
        v27 = a2;
        v28 = *((long long *)((char *)v27 - 24));
        idx2->field_0 = *((long long *)((char *)v27 - 32));
        *((unsigned long *)&idx2->padding_8[0]) = v28;
        v29 = *((long long *)((char *)v27 - 16));
        v30 = *((long long *)((char *)v27 - 8));
        idx2->field_10 = v29;
        *((unsigned long *)&idx2->padding_18[0]) = v30;
        return v29;
    }
    else
    {
        v3 = a1 >> 1;
        v4 = a1 - v3;
        v5 = a0;
        v6 = (char *)a0 + -(v3 * 32);
        sequential_sort(v6, v4, (!a3 ? 0 : -(v3 * 32)) + (char *)a2, a3);
        if (v3 > 1)
        {
            sequential_sort(v5, v3, a2, !a3);
        }
        else if (a3 != 1)
        {
            v31 = (char *)a2 - 32;
            v32 = v5;
            v33 = *((long long *)((char *)v32 - 24));
            v31->field_0 = *((long long *)((char *)v32 - 32));
            *((unsigned long *)&v31->padding_8[0]) = v33;
            v34 = *((long long *)((char *)v32 - 8));
            v31->field_10 = *((long long *)((char *)v32 - 16));
            *((unsigned long *)&v31->padding_18[0]) = v34;
        }
        if (a3)
        {
            v1 = a2;
            v2 = a0;
        }
        else
        {
            v1 = a0;
            v2 = a2;
        }
        return sub_40b413(v1, a1, v2);
    }
}



// Function: merge_tree_init @ 0xb7b6
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    long long field_28;
    long long field_30;
    unsigned long long field_38;
    char padding_40[16];
    unsigned int field_50;
    char field_54;
} struct_0;

struct_0 * merge_tree_init(long long a0, long long a1, long long a2)
{
    long long count;  // rdx
    struct_0 *v0;  // [bp-0x18]
    struct_0 *idx;  // [bp-0x10]

    v0 = sub_418024(a0 * 0x100);
    idx = v0;
    idx->field_18 = 0;
    idx->field_10 = idx->field_18;
    idx->field_8 = idx->field_10;
    idx->field_0 = idx->field_8;
    idx->field_20 = 0;
    idx->field_30 = a1;
    count = idx->field_30;
    idx->field_28 = count;
    idx->field_38 = 0;
    idx->field_50 = 0;
    idx->field_54 = 0;
    pthread_mutex_init((char *)&idx[1].field_0 + 3, 0, count);
    sub_40b911(idx, (char *)&idx[1].field_28 + 3, a2, a0, a1, 0);
    return v0;
}



// Function: merge_tree_destroy @ 0xb8b3
void merge_tree_destroy(unsigned long a0, unsigned long a1)
{
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10]

    v0 = a0 * 2;
    v2 = a1;
    while (1)
    {
        v1 = v0 - 1;
        if (!v0)
            break;
        pthread_mutex_destroy(v2 + 88);
        v2 += 128;
        v0 = v1;
    }
    free(a1);
    return;
}



// Function: init_node @ 0xb911
typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[24];
    unsigned int field_50;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    struct struct_0 *field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    struct struct_0 *field_38;
    struct struct_2 *field_40;
    void* field_48;
    unsigned int field_50;
    char field_54;
} struct_2;

struct_2 * init_node(struct_0 *a0, struct_2 *a1, unsigned long a2, unsigned long long a3, long long a4, char a5)
{
    struct_2 *v11;  // rax
    unsigned long count;  // rdx
    void* v0;  // [bp-0x68], Other Possible Types: struct_2 *
    unsigned long long v1;  // [bp-0x50]
    unsigned long long v2;  // [bp-0x48]
    unsigned long long v3;  // [bp-0x40]
    long long v4;  // [bp-0x38]
    long long v5;  // [bp-0x30]
    struct_0 *v6;  // [bp-0x28]
    struct_2 *idx;  // [bp-0x20]
    unsigned long long v8;  // [bp-0x18]
    long long v9;  // [bp-0x10]

    v0 = a1;
    v1 = (!a5 ? a0->field_30 : a0->field_28);
    v2 = v1 >> 1;
    v3 = v1 - v2;
    v4 = a2 + -(a4 * 32);
    v5 = v4 + -(v2 * 32);
    v6 = (!a5 ? (struct struct_0 *)&a0->padding_0[24] : (struct struct_0 *)&a0->padding_0[16]);
    v11 = v0;
    v0 = (char *)&v11[1].field_28 + 3;
    idx = v11;
    idx->field_10 = v4;
    idx->field_0 = idx->field_10;
    idx->field_18 = v5;
    idx->field_8 = idx->field_18;
    idx->field_20 = v6;
    idx->field_28 = v2;
    idx->field_30 = v3;
    idx->field_38 = a0;
    count = a0->field_50 + 1;
    idx->field_50 = count;
    idx->field_54 = 0;
    pthread_mutex_init((char *)&idx[1].field_0 + 3, 0, count);
    if (a3 <= 1)
    {
        idx->field_40 = NULL;
        idx->field_48 = NULL;
        return v0;
    }
    v8 = a3 >> 1;
    v9 = a3 - v8;
    idx->field_40 = v0;
    v0 = init_node(idx, v0, v4, v8, a4, 1);
    idx->field_48 = v0;
    v0 = init_node(idx, v0, v5, v9, a4, 0);
    return v0;
}



// Function: compare_nodes @ 0xbb0c
typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[24];
    unsigned int field_50;
} struct_0;

char compare_nodes(struct_0 *a0, unsigned long long *a1)
{
    return (a0->field_50 == (int)a1[10] ? a0->field_28 + a0->field_30 < a1[6] + a1[5] : a0->field_50 < (int)a1[10]);
}



// Function: lock_node @ 0xbb87
long long lock_node(long long a0)
{
    return pthread_mutex_lock(a0 + 88);
}



// Function: unlock_node @ 0xbba6
long long unlock_node(long long a0)
{
    return pthread_mutex_unlock(a0 + 88);
}



// Function: queue_destroy @ 0xbbc5
long long queue_destroy(long long *a0)
{
    sub_412398(*(a0));
    pthread_cond_destroy(a0 + 6);
    return pthread_mutex_destroy(a0 + 1);
}



// Function: queue_init @ 0xbc07
long long queue_init(long long *a0, unsigned long a1)
{
    long long count;  // rdx

    count = sub_4122ed(sub_40bb0c, a1 * 2);
    *(a0) = count;
    pthread_mutex_init(a0 + 1, 0, count);
    return pthread_cond_init(a0 + 6, 0);
}



// Function: queue_insert @ 0xbc6b
typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

long long queue_insert(long long *a0, struct_0 *a1)
{
    pthread_mutex_lock(a0 + 1);
    sub_4123c6(*(a0), a1, a1);
    a1->field_54 = 1;
    pthread_cond_signal(a0 + 6);
    return pthread_mutex_unlock(a0 + 1);
}



// Function: queue_pop @ 0xbcd0
typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

struct_0 * queue_pop(long long *a0)
{
    struct_0 *v0;  // [bp-0x10]

    pthread_mutex_lock(a0 + 1);
    while (1)
    {
        v0 = sub_412474(*(a0));
        if (v0)
            break;
        pthread_cond_wait(a0 + 6, a0 + 1, a0 + 1);
    }
    pthread_mutex_unlock(a0 + 1);
    sub_40bb87(v0);
    v0->field_54 = 0;
    return v0;
}



// Function: write_unique @ 0xbd51
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 g_423240;
extern unsigned long long g_423248;
extern unsigned long long g_423250;
extern unsigned long long g_423258;
extern char g_423682;

long long write_unique(unsigned long long *idx, FILE *a1, unsigned long long a2)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rdx
    unsigned long v3;  // rdx

    if (g_423682)
    {
        if (g_423240.field_0)
        {
            v1 = sub_409e2a(idx, &g_423240.field_0);
            if (!(unsigned int)v1)
                return v1;
        }
        v2 = idx[1];
        g_423240.field_0 = *(idx);
        g_423248 = v2;
        v3 = idx[3];
        g_423250 = idx[2];
        g_423258 = v3;
    }
    return sub_409f73(idx, a1, a2);
}



// Function: mergelines_node @ 0xbde5
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char *field_0;
} struct_1;

typedef struct struct_0 {
    char *field_0;
    char *field_8;
    char padding_10[8];
    unsigned long long field_18;
    struct struct_1 *field_20;
    long long field_28;
    long long field_30;
    char padding_38[24];
    unsigned int field_50;
} struct_0;

struct_0 * mergelines_node(struct_0 *idx, unsigned long long a1, FILE *a2, unsigned long long a3)
{
    char *idx1;  // rsi
    unsigned long v8;  // rdx
    unsigned long v17;  // rdx
    unsigned long v18;  // rdx
    unsigned long v9;  // rdx
    char *idx2;  // rsi
    unsigned long v11;  // rdx
    unsigned long v12;  // rdx
    char *v13;  // rsi
    unsigned long v14;  // rdx
    unsigned long v15;  // rdx
    char *v16;  // rsi
    unsigned long long node;  // [bp-0x38]
    char *iter;  // [bp-0x30]
    char *v2;  // [bp-0x28]
    char *v3;  // [bp-0x20]
    long long v4;  // [bp-0x18]
    long long v5;  // [bp-0x10]

    v2 = idx->field_0;
    v3 = idx->field_8;
    node = (a1 >> (((char)idx->field_50 + 1) * 2 & 63)) + 1;
    if (idx->field_50 > 1)
    {
        iter = idx->field_20->field_0;
        while (1)
        {
            if (idx->field_0 == idx->padding_10 || !(node = node, idx->field_8 != idx->field_18 && (node -= 1, node)))
                break;
            if (sub_409e2a(idx->field_0 - 32, idx->field_8 - 32) <= 0)
            {
                idx->field_0 = idx->field_0 - 32;
                idx1 = idx->field_0;
                iter -= 32;
                v8 = *((long long *)&idx1[8]);
                *((long long *)&iter) = *((long long *)&idx1);
                *((unsigned long *)&iter[8]) = v8;
                v9 = *((long long *)&idx1[24]);
                *((long long *)&iter[16]) = *((long long *)&idx1[16]);
                *((unsigned long *)&iter[24]) = v9;
            }
            else
            {
                idx->field_8 = idx->field_8 - 32;
                idx2 = idx->field_8;
                iter -= 32;
                v11 = *((long long *)&idx2[8]);
                *((long long *)&iter) = *((long long *)&idx2);
                *((unsigned long *)&iter[8]) = v11;
                v12 = *((long long *)&idx2[24]);
                *((long long *)&iter[16]) = *((long long *)&idx2[16]);
                *((unsigned long *)&iter[24]) = v12;
            }
        }
        if (v5 == idx->field_30)
        {
            for (v5 = (long long)(v3 - idx->field_8) >> 5; idx->field_0 != idx->padding_10 && (node -= 1, node); node = node)
            {
                idx->field_0 = idx->field_0 - 32;
                v13 = idx->field_0;
                iter -= 32;
                v14 = *((long long *)&v13[8]);
                *((long long *)&iter) = *((long long *)&v13);
                *((unsigned long *)&iter[8]) = v14;
                v15 = *((long long *)&v13[24]);
                *((long long *)&iter[16]) = *((long long *)&v13[16]);
                *((unsigned long *)&iter[24]) = v15;
            }
        }
        else if (v4 == idx->field_28)
        {
            for (v4 = (long long)(v2 - idx->field_0) >> 5; idx->field_8 != idx->field_18 && (node -= 1, node); node = node)
            {
                idx->field_8 = idx->field_8 - 32;
                v16 = idx->field_8;
                iter -= 32;
                v17 = *((long long *)&v16[8]);
                *((long long *)&iter) = *((long long *)&v16);
                *((unsigned long *)&iter[8]) = v17;
                v18 = *((long long *)&v16[24]);
                *((long long *)&iter[16]) = *((long long *)&v16[16]);
                *((unsigned long *)&iter[24]) = v18;
            }
        }
        idx->field_20->field_0 = iter;
    }
    else
    {
        while (1)
        {
            if (idx->field_0 == idx->padding_10 || !(node = node, idx->field_8 != idx->field_18 && (node -= 1, node)))
                break;
            if (sub_409e2a(idx->field_0 - 32, idx->field_8 - 32) <= 0)
            {
                idx->field_0 = idx->field_0 - 32;
                sub_40bd51(idx->field_0, a2, a3);
                node = node;
            }
            else
            {
                idx->field_8 = idx->field_8 - 32;
                sub_40bd51(idx->field_8, a2, a3);
            }
        }
        if (v5 == idx->field_30)
        {
            for (v5 = (long long)(v3 - idx->field_8) >> 5; idx->field_0 != idx->padding_10 && (node -= 1, node); node = node)
            {
                idx->field_0 = idx->field_0 - 32;
                sub_40bd51(idx->field_0, a2, a3);
            }
        }
        else if (v4 == idx->field_28)
        {
            for (v4 = (long long)(v2 - idx->field_0) >> 5; idx->field_8 != idx->field_18 && (node -= 1, node); node = node)
            {
                idx->field_8 = idx->field_8 - 32;
                sub_40bd51(idx->field_8, a2, a3);
            }
        }
    }
    v4 = (long long)(v2 - idx->field_0) >> 5;
    v5 = (long long)(v3 - idx->field_8) >> 5;
    idx->field_28 = idx->field_28 - v4;
    idx->field_30 = idx->field_30 - v5;
    return idx;
}



// Function: queue_check_insert @ 0xc28e
typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

long long queue_check_insert(long long *a0, struct_0 *a1)
{
    long long v3;  // rax
    char v4;  // al
    char v5;  // al
    char v6;  // al
    char v0;  // [bp-0xa]
    char v1;  // [bp-0x9]

    v3 = a1->field_54 ^ 1;
    if (!(char)v3)
        return v3;
    v0 = *((long long *)&a1->padding_0[0]) != *((long long *)&a1->padding_0[16]);
    v1 = *((long long *)&a1->padding_0[8]) != *((long long *)&a1->padding_0[24]);
    if (v0)
    {
        if (v1 || !*((long long *)&a1->padding_0[48]))
            v4 = 1;
        else
            v4 = 0;
        v5 = v4 & 1;
    }
    else
    {
        if (v1 && !*((long long *)&a1->padding_0[40]))
            v6 = 1;
        else
            v6 = 0;
        v5 = v6 & 1;
    }
    return sub_40bc6b(a0, a1);
}



// Function: queue_check_insert_parent @ 0xc34c
long long queue_check_insert_parent(long long *a0, void* idx)
{
    long long v1;  // rax

    if ((int)idx[80] > 1)
    {
        sub_40bb87((long long)idx[56]);
        sub_40c28e(a0, (long long)idx[56]);
        return sub_40bba6((long long)idx[56]);
    }
    v1 = (long long)idx[48] + (long long)idx[40];
    if (v1)
        return v1;
    sub_40bc6b(a0, (long long)idx[56]);
    return v1;
}



// Function: merge_loop @ 0xc3d7
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[80];
    unsigned int field_50;
} struct_0;

long long merge_loop(long long *a0, unsigned long long a1, FILE *a2, unsigned long long a3)
{
    struct_0 *v0;  // [bp-0x10]

    while (1)
    {
        v0 = sub_40bcd0(a0);
        if (!v0->field_50)
            break;
        sub_40bde5(v0, a1, a2, a3);
        sub_40c28e(a0, v0);
        sub_40c34c(a0, v0);
        sub_40bba6(v0);
    }
    sub_40bba6(v0);
    return sub_40bc6b(a0, v0);
}



// Function: sortlines_thread @ 0xc481
unsigned int sortlines_thread(long long *a0)
{
    sub_40c4e8(*(a0), a0[1], a0[2], a0[3], a0[4], a0[5], a0[6]);
    return 0;
}



// Function: sortlines @ 0xc4e8
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[8];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char padding_31[1];
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_0 *field_40;
} struct_0;

void sortlines(unsigned long a0, unsigned long long a1, unsigned long a2, struct_0 *idx, long long *a4, FILE *a5, unsigned long a6)
{
    char v0;  // [bp-0x80]
    unsigned long long v1;  // [bp-0x78]
    unsigned long long v2;  // [bp-0x70]
    long long v3;  // [bp-0x68]
    unsigned long v4;  // [bp-0x60]
    unsigned long v5;  // [bp-0x58]
    struct_0 *v6;  // [bp-0x50]
    unsigned long v7;  // [bp-0x48]
    unsigned long long v8;  // [bp-0x40]
    unsigned long v9;  // [bp-0x38]
    unsigned long v10;  // [bp-0x30]
    long long *v11;  // [bp-0x28]
    FILE *v12;  // [bp-0x20]
    unsigned long v13;  // [bp-0x18]

    v1 = *((long long *)&idx->field_30) + idx->field_28;
    v2 = a1 >> 1;
    v3 = a1 - v2;
    v7 = a0;
    v8 = v2;
    v9 = a2;
    v10 = idx->field_40;
    v11 = a4;
    v12 = a5;
    v13 = a6;
    if (a1 > 1 && v1 > 0x1ffff && !pthread_create(&v0, 0, thread_entry, &v7))
    {
        sortlines(-(idx->field_28 * 32) + a0, v3, a2, idx[1].field_0, a4, a5, a6);
        pthread_join(v0, 0);
        return;
    }
    v4 = idx->field_28;
    v5 = *((long long *)&idx->field_30);
    v6 = a0 + -(a2 * 32);
    if (v5 > 1)
        sub_40b536(-(v4 * 32) + a0, v5, -((v4 >> 1) * 32) + (char *)v6, 0);
    if (v4 > 1)
        sub_40b536(a0, v4, v6, 0);
    idx->field_0 = a0;
    *((unsigned long *)&idx->padding_8[0]) = -(v4 * 32) + a0;
    idx->field_10 = -(v4 * 32) + a0;
    *((unsigned long *)&idx->padding_18[0]) = -(v4 * 32) - v5 * 32 + a0;
    sub_40bc6b(a4, idx);
    sub_40c3d7(a4, a2, a5, a6);
    return;
}



// Function: avoid_trashing_input @ 0xc7fa
typedef struct FILE {
} FILE;

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

void avoid_trashing_input(unsigned long a0, unsigned long long a1, unsigned long long a2, char *a3)
{
    char v10;  // al
    char v0;  // [bp-0xca]
    char v1;  // [bp-0xc9]
    FILE *v2;  // [bp-0xc8]
    unsigned long v3;  // [bp-0xc0]
    unsigned long long idx;  // [bp-0xb8]
    stat *v5;  // [bp-0xb0]
    int v6;  // [bp-0xa8], Other Possible Types: stat

    v3 = 0;
    idx = a1;
    while (1)
    {
        if (idx >= a2)
            return;
        v1 = !strcmp(*((long long *)(a0 + idx * 16)), "-");
        if (a3 && !strcmp(a3, *((long long *)(a0 + idx * 16))) && v1 != 1)
        {
            v0 = 1;
            goto LABEL_40c993;
        }
        v5 = sub_404e8c();
        if (!v5)
            return;
        if (v1)
        {
            if (fstat(0, &v6))
                goto LABEL_40c981;
        }
        else
        {
            if (stat(*((long long *)(a0 + idx * 16)), &v6))
                goto LABEL_40c981;
        }
        if (*((unsigned long long *)(&v6 + 8)) == v5->st_ino && *((unsigned long long *)&v6) == *((long long *)&v5->st_mode))
        {
            v10 = 1;
            goto LABEL_40c986;
        }
LABEL_40c981:
        v10 = 0;
LABEL_40c986:
        v0 = v10;
        v0 &= 1;
LABEL_40c993:
        if (v0)
        {
            if (!v3)
            {
                v3 = sub_4054f9(&v2);
                sub_40b344(idx * 16 + a0, 0, 1, v2, v3 + 13);
            }
            *((unsigned long long *)(a0 + idx * 16)) = v3 + 13;
            *((unsigned long *)(idx * 16 + a0 + 8)) = v3;
        }
        idx += 1;
    }
}



// Function: check_inputs @ 0xca81
unsigned long long check_inputs(unsigned long a0, unsigned long long a1)
{
    unsigned long v2;  // rax
    unsigned long long i;  // [bp-0x20]

    for (i = 0; i < a1; i += 1)
    {
        if (strcmp(*((long long *)(a0 + i * 8)), "-") && euidaccess(*((long long *)(a0 + i * 8)), 4, i * 8))
        {
            v2 = gettext("cannot read");
            sub_404464(v2, *((long long *)(a0 + i * 8))); /* do not return */
        }
    }
    return i;
}



// Function: check_output @ 0xcb44
int check_output(char *a0)
{
    int v4;  // eax
    int v0;  // [bp-0x10]
    unsigned int v1;  // [bp-0xc]

    if (!a0)
        return v4;
    v0 = 524353;
    v1 = open(a0, v0, 438);
    if (v1 >= 0)
        return sub_40515d(v1, 1);
    sub_404464(gettext("open failed"), a0); /* do not return */
}



// Function: merge @ 0xcbb9
typedef struct FILE {
} FILE;

extern unsigned int g_423024;

long long merge(void* a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long v19;  // fs
    unsigned long v20;  // rax
    unsigned long v21;  // rax
    unsigned long v22;  // rax
    unsigned long v23;  // rax
    unsigned long v24;  // rax
    unsigned long v25;  // rax
    unsigned long v27;  // rax
    unsigned long i;  // [bp-0xb0]
    unsigned long iter;  // [bp-0xa8]
    unsigned long long v2;  // [bp-0x90]
    FILE *v3;  // [bp-0x88]
    unsigned long long node;  // [bp-0x80]
    unsigned long long v5;  // [bp-0x78]
    unsigned long long idx;  // [bp-0x78]
    unsigned long v7;  // [bp-0x70]
    FILE *v8;  // [bp-0x68]
    unsigned long k;  // [bp-0x60]
    unsigned long long v10;  // [bp-0x58]
    unsigned long long v11;  // [bp-0x50]
    unsigned long v12;  // [bp-0x48]
    unsigned long v13;  // [bp-0x40]
    unsigned long v14;  // [bp-0x38]
    unsigned long v15;  // [bp-0x30]
    unsigned long v16;  // [bp-0x28]
    unsigned long v17;  // [bp-0x20]

    iter = a1;
    i = a2;
    for (v17 = *((long long *)(40 + v19)); i > g_423024; i += idx - node)
    {
        node = 0;
        for (idx = node; g_423024 <= i - node; idx += 1)
        {
            v15 = sub_4054f9(&v3);
            if (g_423024 <= iter)
                v20 = g_423024;
            else
                v20 = iter;
            v16 = sub_40b344(*((unsigned int *)&node * 16 + a0), v20, g_423024, v3, v15 + 13);
            if (v16 <= iter)
                v21 = v16;
            else
                v21 = iter;
            iter -= v21;
            *((unsigned long long *)((char *)a0 + 16 * idx)) = v15 + 13;
            *((unsigned long *)(16 * idx + (char *)a0 + 8)) = v15;
            node += v16;
        }
        v10 = i - node;
        v11 = g_423024 - idx % g_423024;
        if (v11 < v10)
        {
            v12 = v10 - v11 + 1;
            v13 = sub_4054f9(&v3);
            if (v12 <= iter)
                v22 = v12;
            else
                v22 = iter;
            v14 = sub_40b344(*((unsigned int *)&node * 16 + a0), v22, v12, v3, v13 + 13);
            if (v14 <= iter)
                v23 = v14;
            else
                v23 = iter;
            iter -= v23;
            *((unsigned long long *)((char *)a0 + 16 * idx)) = v13 + 13;
            v5 = idx + 1;
            *((unsigned long *)(16 * idx + (char *)a0 + 8)) = v13;
            node += v14;
            idx = v5;
        }
        memmove(a0 + idx * 16, node * 16 + a0, (i - node) * 16);
        iter += idx;
    }
    sub_40c7fa(a0, iter, i, a3);
    while (1)
    {
        v7 = sub_40a4a1(a0, i, &v2);
        if (v7 == i)
        {
            v8 = sub_404ee4(a3, "w");
            if (v8)
                break;
            if (*(__errno_location()) != 24 || v7 <= 2)
                sub_404464(gettext("open failed"), a3); /* do not return */
        }
        else if (v7 <= 2)
        {
            v27 = gettext("open failed");
            sub_404464(v27, *((long long *)((char *)a0 + 16 * v7))); /* do not return */
        }
        do
        {
            v7 -= 1;
            sub_4050b7(*((long long *)(v7 * 8 + v2)), *((long long *)((char *)a0 + 16 * v7)));
            k = sub_405372(&v3, v7 > 2);
        } while (!k);
        if (v7 <= iter)
            v24 = v7;
        else
            v24 = iter;
        sub_40a5c0(a0, v24, v7, v3, k + 13, v2);
        if (v7 <= iter)
            v25 = v7;
        else
            v25 = iter;
        iter -= v25;
        *((unsigned long long *)a0) = k + 13;
        *((unsigned long *)&a0[8]) = k;
        memmove(a0 + 16, v7 * 16 + a0, (i - v7) * 16);
        iter += 1;
        i = i - v7 + 1;
    }
    sub_40a5c0(a0, iter, i, v8, a3, v2);
    if (v17 == *((long long *)(40 + v19)))
        return v17 - *((long long *)(40 + v19));
    __stack_chk_fail(); /* do not return */
}



// Function: sort @ 0xd0f9
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern unsigned long long g_423240;
extern struct_0 *g_423720;

void sort(unsigned long long *a0, unsigned long long a1, unsigned long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x138]
    unsigned long long *iter;  // [bp-0x130]
    char j;  // [bp-0x121]
    unsigned long v3;  // [bp-0x120]
    FILE *v4;  // [bp-0x118]
    FILE *v5;  // [bp-0x118]
    long long v6;  // [bp-0x110]
    unsigned long v7;  // [bp-0x108]
    unsigned long long v8;  // [bp-0x100]
    unsigned long long v9;  // [bp-0xf8]
    unsigned long long i;  // [bp-0xf0]
    struct_0 **v11;  // [bp-0xe8]
    unsigned long long idx;  // [bp-0xe0]
    unsigned long v13;  // [bp-0xd8]
    unsigned long v14;  // [bp-0xd0]
    void* v15;  // [bp-0xc8]
    void* ptr;  // [bp-0xc0]
    void* v17;  // [bp-0xb8]
    unsigned long long v18;  // [bp-0xb0]
    long long v19;  // [bp-0xa8]
    unsigned long long v20;  // [bp-0xa0]
    unsigned long long v21;  // [bp-0x98]
    char v22;  // [bp-0x88]
    long long v23[13];  // [bp-0x78]

    iter = a0;
    v0 = a1;
    v6 = 0;
    j = 0;
    v20 = 0;
LABEL_40d4b5:
    if (v0)
    {
        v13 = *(iter);
        v3 = sub_40505e(v13, "r");
        if (a3 > 1)
        {
            v9 = 1;
            for (i = 1; v9 < a3; i += 1)
            {
                v9 *= 2;
            }
            v8 = i * 32;
        }
        else
        {
            v8 = 48;
        }
        if (!v20)
            sub_406618(&v17, v8, sub_40637c(&v3, 1, iter, v0, v8));
        v22 = 0;
        iter += 1;
        v0 -= 1;
        v5 = v4;
        do
        {
            if (!(char)sub_406a5e(&v17, v3, v13))
            {
LABEL_40d49c:
                sub_4050b7(v3, v13);
                goto LABEL_40d4b5;
            }
            else if (v22 && v0 && v8 + 1 < v20 - v18 - v8 * v19)
            {
                v21 = v18;
                goto LABEL_40d49c;
            }
            g_423240 = 0;
            v14 = sub_4066d0(&v17);
            if (v22 && !v0 && !v6 && !v21)
            {
                sub_4050b7(v3, v13);
                v5 = sub_40505e(a2, "w");
                v7 = a2;
                j = 1;
            }
            else
            {
                v6 += 1;
                v7 = sub_4054f9(&v5) + 1;
            }
            if (v19 > 1)
            {
                sub_40bc07(v23, a3);
                v15 = sub_40b7b6(a3, v19, v14);
                sub_40c4e8(v14, a3, v19, v15 + 128, v23, v5, v7);
                sub_40b8b3(a3, v15);
                sub_40bbc5(v23);
            }
            else
            {
                sub_40bd51(v14 - 32, v5, v7);
            }
            sub_4050b7(v5, v7);
        } while (!j);
    }
    free(v17);
    if (j == 1)
    {
        sub_404b69();
        return;
    }
    v11 = g_423720;
    ptr = sub_41819d(v6, 16);
    for (idx = 0; v11; idx += 1)
    {
        *((struct_0 ***)((char *)ptr + 16 * idx)) = (char *)&v11[1] + 5;
        *((struct_0 ***)(16 * idx + (char *)ptr + 8)) = v11;
        v11 = *(v11);
    }
    sub_40cbb9(ptr, v6, v6, a2);
    free(ptr);
    sub_404b69();
    return;
}



// Function: insertkey @ 0xd5d8
typedef struct struct_0 {
    char padding_0[64];
    unsigned long long field_40;
} struct_0;

extern struct_0 *g_423688;

struct_0 * insertkey(long long a0)
{
    struct_0 **i;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v1 = sub_418564(a0, 72);
    for (i = &g_423688; *(i); i = &*(i)->field_40);
    *(i) = v1;
    v1->field_40 = 0;
    return v1;
}



// Function: badfieldspec @ 0xd640
void badfieldspec(long long a0, char *a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v1 = v3;
    v0 = v4;
    sub_4154a0(a0);
    gettext(a1);
    error(2, 0, gettext("%s: invalid field specification %s"));
}



// Function: incompatible_options @ 0xd6a1
void incompatible_options(unsigned long long a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    error(2, 0, gettext("options '-%s' are incompatible"));
}



// Function: check_ordering_compatibility @ 0xd6de
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    char padding_28[8];
    char field_30;
    char field_31;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_0 *field_40;
} struct_0;

extern struct_0 *g_423688;

void check_ordering_compatibility(void)
{
    struct_0 *iter;  // [bp-0x40]
    char v1[40];  // [bp-0x38]

    for (iter = g_423688; iter; iter = iter->field_40)
    {
        if ((iter->field_20 | iter->field_33 | iter->field_38) + iter->field_32 + iter->field_34 + iter->field_35 + iter->field_36 > 1)
        {
            iter->field_37 = 0;
            iter->field_31 = iter->field_37;
            iter->field_30 = iter->field_31;
            sub_40839e(iter, v1);
            sub_40d6a1(v1); /* do not return */
        }
    }
    return;
}



// Function: parse_field_count @ 0xd7de
extern long long g_41b1db;

unsigned long long parse_field_count(long long a0, unsigned long long *a1, char *a2)
{
    char v0;  // [bp-0x30], Other Possible Types: unsigned long long
    char v1;  // [bp-0x28]

    sub_418b54(a0, &v0, 10, &v1, &g_41b1db);
    switch ((unsigned int)sub_418b54(a0, &v0, 10, &v1, &g_41b1db))
    {
    case 4:
        if (a2)
        {
            sub_4154a0(a0);
            gettext(a2);
            error(2, 0, gettext("%s: invalid count at start of %s"));
        }
        return 0;
    case 0: case 2:
        *(a1) = v1;
        if (*(a1) == v1)
            return v0;
    case 1: case 3:
        *(a1) = 0xffffffffffffffff;
        return v0;
    default:
        return v0;
    }
}



// Function: sighandler @ 0xd8f9
int sighandler(int a0)
{
    sub_404b8b();
    signal(a0, NULL);
    return raise(a0);
}



// Function: set_ordering @ 0xd929
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    char padding_28[8];
    char field_30;
    char field_31;
    char padding_32[2];
    char field_34;
    char field_35;
} struct_0;

extern unsigned long long g_423360;
extern unsigned long long g_423460;
extern char g_423560;

char * set_ordering(char *a0, struct_0 *idx, unsigned int a2)
{
    char *i;  // [bp-0x10]

    for (i = a0; *(i); i += 1)
    {
        switch (*(i))
        {
        case 77:
            idx[1].padding_0[0] = 1;
            break;
        case 82:
            idx->padding_32[1] = 1;
            break;
        case 86:
            idx[1].padding_0[2] = 1;
            break;
        case 98:
            if (!a2 || a2 == 2)
                idx->field_30 = 1;
            if (a2 == 1 || a2 == 2)
                idx->field_31 = 1;
            break;
        case 100:
            idx->field_20 = &g_423460;
            break;
        case 102:
            *((char **)&idx->padding_28[0]) = &g_423560;
            break;
        case 103:
            idx->field_34 = 1;
            break;
        case 104:
            idx->field_35 = 1;
            break;
        case 105:
            if (!idx->field_20)
                idx->field_20 = &g_423360;
            break;
        case 110:
            idx->padding_32[0] = 1;
            break;
        case 114:
            idx[1].padding_0[1] = 1;
            break;
        default:
            return i;
        }
    }
    return i;
}



// Function: key_init @ 0xda55
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

struct_0 * key_init(struct_0 *ptr)
{
    memset(ptr, 0, 72);
    ptr->field_10 = 0xffffffffffffffff;
    return ptr;
}



// Function: main @ 0xda8d
typedef struct sigaction {
} sigaction;

typedef struct struct_2 {
    struct struct_0 *field_0;
    long long field_8;
} struct_2;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

typedef struct struct_4 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char field_31;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    struct struct_4 *field_40;
} struct_4;

typedef struct lconv {
    char * decimal_point;
    char * thousands_sep;
    char * grouping;
    char * int_curr_symbol;
    char * currency_symbol;
    char * mon_decimal_point;
    char * mon_thousands_sep;
    char * mon_grouping;
    char * positive_sign;
    char * negative_sign;
    char int_frac_digits;
    char frac_digits;
    char p_cs_precedes;
    char p_sep_by_space;
    char n_cs_precedes;
    char n_sep_by_space;
    char p_sign_posn;
    char n_sign_posn;
    char int_p_cs_precedes;
    char int_p_sep_by_space;
    char int_n_cs_precedes;
    char int_n_sep_by_space;
    char int_p_sign_posn;
    char int_n_sign_posn;
} lconv;

typedef struct FILE {
} FILE;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_41b1db;
extern unsigned int g_41d100[4];
extern option g_422560;
extern long long g_422940;
extern long long g_422960;
extern char g_423010;
extern unsigned int g_423020;
extern unsigned int g_423024;
extern char g_423032;
extern long long g_423108;
extern long long g_423110;
extern unsigned int optind;
extern long long stdin;
extern struct_0 *optarg;
extern FILE *stderr;
extern long long stdout;
extern char g_423220;
extern unsigned int g_423224;
extern char g_423228;
extern char g_423229;
extern char g_42322a;
extern unsigned long long g_423660;
extern unsigned long long g_423670;
extern char g_423680;
extern char g_423681;
extern char g_423682;
extern char g_423683;
extern struct_4 *g_423688;
extern char *g_423690;
extern char g_423698;
extern unsigned long long g_4236a0;
extern unsigned long long g_4236a8;
extern unsigned long long g_4236b0;
extern unsigned long long g_4236b8;
extern unsigned long long g_4236c0;
extern unsigned long long g_4236c8;
extern unsigned long long g_4236d0;
extern unsigned long long g_4236d8;
extern unsigned long long g_4236e0;
extern unsigned long long g_4236e8;
extern unsigned long long g_4236f0;
extern unsigned long long g_4236f8;
extern unsigned long long g_423700;
extern unsigned long long g_423708;
extern unsigned long long g_423710;
extern unsigned long long g_423718;

unsigned int main(int a0, struct_0 **a1)
{
    long long v67;  // rdi
    long long v68;  // rsi
    unsigned long idx;  // rax
    unsigned long long v79;  // rax
    unsigned long long v80;  // rax
    unsigned long long v81;  // rax
    int v82;  // eax
    unsigned long idx1;  // rax
    char *v85;  // rax
    long long v69;  // rdx
    unsigned long long v92;  // rdx
    unsigned long long v93;  // rax
    long long v70;  // rcx
    long long v71;  // r8
    long long v72;  // r9
    char v73;  // al
    char v74;  // al
    unsigned int v75;  // eax
    int v76;  // eax
    char v0;  // [bp-0x339]
    char v1;  // [bp-0x338]
    char v2;  // [bp-0x337]
    char v3;  // [bp-0x336]
    char v4;  // [bp-0x335]
    char v5;  // [bp-0x334]
    char v6;  // [bp-0x333]
    char v7;  // [bp-0x332]
    char v8;  // [bp-0x331]
    unsigned int v9;  // [bp-0x330]
    unsigned int v10;  // [bp-0x32c]
    struct_4 *iter;  // [bp-0x328], Other Possible Types: unsigned long long *
    char *v12;  // [bp-0x320]
    char *v13;  // [bp-0x318]
    unsigned long long v14;  // [bp-0x310]
    unsigned long v15;  // [bp-0x308]
    struct_2 *ptr;  // [bp-0x300]
    char *v17;  // [bp-0x2f8]
    char *v18;  // [bp-0x2f0]
    unsigned long long node;  // [bp-0x2e8]
    char *v20;  // [bp-0x2e0]
    unsigned long l;  // [bp-0x2d8]
    unsigned long index;  // [bp-0x2d0]
    lconv *v23;  // [bp-0x2c8]
    unsigned long v24;  // [bp-0x2c0]
    unsigned long v25;  // [bp-0x2b8]
    unsigned long v26;  // [bp-0x2b0]
    char *v27;  // [bp-0x2a8]
    unsigned long long v28;  // [bp-0x2a0]
    unsigned long long v29;  // [bp-0x298]
    void* v30;  // [bp-0x290]
    struct_0 v31;  // [bp-0x288]
    struct_0 v32;  // [bp-0x238]
    unsigned long v33;  // [bp-0x218]
    unsigned long v34;  // [bp-0x210]
    char v35;  // [bp-0x208]
    char v36;  // [bp-0x207]
    char v37;  // [bp-0x206]
    char v38;  // [bp-0x205]
    char v39;  // [bp-0x204]
    char v40;  // [bp-0x203]
    char v41;  // [bp-0x202]
    char v42;  // [bp-0x201]
    char v43;  // [bp-0x200]
    unsigned long v44;  // [bp-0x1e8]
    struct_2 *v45;  // [bp-0x1e0]
    sigaction v46;  // [bp-0xc8]
    unsigned long v47;  // [bp-0xc0]
    unsigned long v48;  // [bp-0xb8]
    unsigned long v49;  // [bp-0xb0]
    unsigned long v50;  // [bp-0xa8]
    unsigned long v51;  // [bp-0xa0]
    unsigned long v52;  // [bp-0x98]
    unsigned long v53;  // [bp-0x90]
    unsigned long v54;  // [bp-0x88]
    unsigned long v55;  // [bp-0x80]
    unsigned long v56;  // [bp-0x78]
    unsigned long v57;  // [bp-0x70]
    unsigned long v58;  // [bp-0x68]
    unsigned long v59;  // [bp-0x60]
    unsigned long v60;  // [bp-0x58]
    unsigned long v61;  // [bp-0x50]
    unsigned long v62;  // [bp-0x48]
    unsigned int v63;  // [bp-0x40]
    char v64;  // [bp-0x22]
    char v65;  // [bp-0x21]

    v0 = 0;
    v9 = 0;
    v1 = 0;
    v2 = 0;
    v13 = NULL;
    v3 = 0;
    v14 = 0;
    v15 = 0;
    v7 = getenv("POSIXLY_CORRECT");
    v10 = sub_4133ab(v67, v68, v69, v70, v71, v72, a1);
    if (v10 <= 200111 || v10 > 200808)
        v73 = 1;
    else
        v73 = 0;
    v4 = v73;
    v4 &= 1;
    v17 = NULL;
    v18 = NULL;
    sub_41345f(*(a1));
    v5 = setlocale(6, &g_41b1db);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_404029(2);
    g_423229 = sub_4107aa(3);
    g_42322a = sub_4107aa(2);
    v23 = localeconv();
    g_423220 = *(v23->decimal_point);
    if (!g_423220 || v23->decimal_point[1])
        g_423220 = 46;
    g_423224 = *(v23->thousands_sep);
    if (g_423224 && v23->thousands_sep[1])
        g_423228 = 1;
    if (!g_423224 || v23->thousands_sep[1])
        g_423224 = 128;
    g_423683 = 0;
    sub_40594a();
    sigemptyset(&g_4236a0);
    for (node = 0; node <= 10; node += 1)
    {
        sigaction(g_41d100[node], NULL, &v46);
        if (v46 != 1)
            sigaddset(&g_4236a0, g_41d100[node]);
    }
    v46 = (sigaction)sub_40d8f9;
    v47 = g_4236a0;
    v48 = g_4236a8;
    v49 = g_4236b0;
    v50 = g_4236b8;
    v51 = g_4236c0;
    v52 = g_4236c8;
    v53 = g_4236d0;
    v54 = g_4236d8;
    v55 = g_4236e0;
    v56 = g_4236e8;
    v57 = g_4236f0;
    v58 = g_4236f8;
    v59 = g_423700;
    v60 = g_423708;
    v61 = g_423710;
    v62 = g_423718;
    v63 = 0;
    for (node = 0; node <= 10; node += 1)
    {
        if (sigismember(&g_4236a0, g_41d100[node]))
            sigaction(g_41d100[node], &v46, NULL);
    }
    signal(0x11, NULL);
    sub_41a960(sub_404bd4);
    sub_40da55(&v32);
    v32 = (struct_0)0xffffffffffffffff;
    ptr = sub_41819d(a0, 8);
    while (1)
    {
        *((unsigned int *)&v44) = 0xffffffff;
        if (v9 == 0xffffffff || v7 && v15 && (v4 != 1 || v1 || a0 == optind || a1[optind]->field_0 != 45 || a1[optind]->field_1 != 111 || !a1[optind]->field_2 && a0 == optind + 1) || !(v9 = (unsigned int)(int)(long long)getopt_long(a0, a1, "-bcCdfghik:mMno:rRsS:t:T:uVy:z", &g_422560.name, &v44), v9 != 0xffffffff))
        {
            if (a0 > optind)
            {
                v82 = optind;
                optind = v82 + 1;
                idx1 = v15;
                v15 = idx1 + 1;
                (&ptr->field_0)[idx1] = a1[v82];
            }
            else if (v17)
            {
                if (v15)
                {
                    sub_415059(4, ptr->field_0);
                    error(0, 0, gettext("extra operand %s"));
                    v85 = gettext("file operands cannot be combined with --files0-from");
                    fprintf(stderr, "%s\n", v85);
                    sub_4044d3(2); /* do not return */
                }
                v25 = sub_40505e(*((unsigned int *)&v17), "r");
                sub_41676c(&v44);
                if ((char)sub_416b8b(v25, &v44, &v44) != 1)
                {
                    sub_415059(4, v17);
                    error(2, 0, gettext("cannot read file names from %s"));
                }
                sub_4050b7(v25, *((unsigned int *)&v17));
                if (!v44)
                {
                    sub_415059(4, v17);
                    error(2, 0, gettext("no input from %s"));
                }
                free(ptr);
                ptr = v45;
                v15 = v44;
                l = 0;
                while (l < v15)
                {
                    if (!strcmp((&ptr->field_0)[l], "-"))
                    {
                        sub_415059(4, (&ptr->field_0)[l], l * 8);
                        error(2, 0, gettext("when reading file names from stdin, no file name of %s allowed"));
                    }
                    else if (!(&ptr->field_0)[l]->field_0)
                    {
                        v26 = l + 1;
                        sub_4151eb(0, 3, v17);
                        error(2, 0, gettext("%s:%lu: invalid zero-length file name"));
                    }
                    else
                    {
                        l += 1;
                    }
                }
            }
        }
        else
        {
            if (v9 > 135)
                goto LABEL_40ebdc;
            if (v9 > 0)
            {
                switch (v9)
                {
                case 1:
                    iter = NULL;
                    if ((char)optarg->field_0 == 43)
                    {
                        if (a0 != optind && a1[optind]->field_0 == 45 && a1[optind]->field_1 - 48 <= 9)
                            v74 = 1;
                        else
                            v74 = 0;
                        v8 = v74;
                        v8 &= 1;
                        if (v8 && v7 != 1)
                            v75 = 1;
                        else
                            v75 = 0;
                        v4 = v75 || v4;
                        if (v4)
                        {
                            iter = sub_40da55(&v31);
                            v12 = sub_40d7de((char *)&optarg->field_0 + 1, iter, NULL);
                            if (v12 && *(v12) == 46)
                                v12 = sub_40d7de(v12 + 1, iter + 1, NULL);
                            if (!*(iter) && !iter[1])
                                *(iter) = 0xffffffffffffffff;
                            if (!v12 || *(sub_40d929(v12, iter, 0)))
                            {
                                iter = NULL;
                            }
                            else
                            {
                                if (v8)
                                {
                                    v76 = optind;
                                    optind = v76 + 1;
                                    v24 = a1[v76];
                                    v12 = sub_40d7de(v24 + 1, iter + 2, "invalid number after '-'");
                                    if (!v12)
                                        __assert_fail(); /* do not return */
                                    if (*(v12) == 46)
                                        v12 = sub_40d7de(v12 + 1, iter + 3, "invalid number after '.'");
                                    if (!iter[3] && iter[2])
                                        iter[2] = iter[2] - 1;
                                    if (*(sub_40d929(v12, iter, 1)))
                                        sub_40d640(v24, "stray character in field spec"); /* do not return */
                                }
                                *((char *)&iter[7] + 1) = 1;
                                sub_40d5d8(iter);
                            }
                        }
                    }
                    if (!iter)
                    {
                        idx = v15;
                        v15 = idx + 1;
                        (&ptr->field_0)[idx] = optarg;
                    }
                    continue;
                case 83:
                    sub_405e0c(v44, v9, optarg);
                    break;
                case 84:
                    sub_40570f(optarg);
                    break;
                case 107:
                    iter = sub_40da55(&v31);
                    v12 = sub_40d7de(optarg, iter, "invalid number at field start");
                    v79 = *(iter);
                    *(iter) = v79 - 1;
                    if (!v79)
                        sub_40d640(optarg, "field number is zero"); /* do not return */
                    if (*(v12) == 46)
                    {
                        v12 = sub_40d7de(v12 + 1, iter + 1, "invalid number after '.'");
                        v80 = iter[1];
                        iter[1] = v80 - 1;
                        if (!v80)
                            sub_40d640(optarg, "character offset is zero"); /* do not return */
                    }
                    if (!*(iter) && !iter[1])
                        *(iter) = 0xffffffffffffffff;
                    v12 = sub_40d929(v12, iter, 0);
                    if (*(v12) != 44)
                    {
                        iter[2] = 0xffffffffffffffff;
                        iter[3] = 0;
                    }
                    else
                    {
                        v12 = sub_40d7de(v12 + 1, iter + 2, "invalid number after ','");
                        v81 = iter[2];
                        iter[2] = v81 - 1;
                        if (!v81)
                            sub_40d640(optarg, "field number is zero"); /* do not return */
                        if (*(v12) == 46)
                            v12 = sub_40d7de(v12 + 1, iter + 3, "invalid number after '.'");
                        v12 = sub_40d929(v12, iter, 1);
                    }
                    if (*(v12))
                        sub_40d640(optarg, "stray character in field spec"); /* do not return */
                    sub_40d5d8(iter);
                    break;
                case 109:
                    v2 = 1;
                    break;
                case 111:
                    if (v18 && strcmp(v18, optarg))
                        error(2, 0, gettext("multiple output files specified"));
                    v18 = optarg;
                    break;
                case 115:
                    g_423681 = 1;
                    break;
                case 116:
                    v6 = (char)optarg->field_0;
                    if (!v6)
                        error(2, 0, gettext("empty tab"));
                    if (*((char *)&optarg->field_0 + 1))
                    {
                        if (!strcmp(optarg, "\\0"))
                        {
                            v6 = 0;
                        }
                        else
                        {
                            sub_4154a0(optarg);
                            error(2, 0, gettext("multi-character tab %s"));
                        }
                    }
                    if (g_423020 != 128 && v6 != g_423020)
                        error(2, 0, gettext("incompatible tabs"));
                    g_423020 = v6;
                    break;
                case 117:
                    g_423682 = 1;
                    break;
                case 121:
                    if (*((long long *)((char *)&a1[optind] - 8)) == optarg)
                    {
                        for (v20 = optarg; *(v20) - 48 <= 9; v20 += 1);
                        optind = optind - (*(v20));
                    }
                    break;
                case 122:
                    g_423010 = 0;
                    break;
                case 129:
                    if (g_423690 && strcmp(g_423690, optarg))
                        error(2, 0, gettext("multiple compress programs specified"));
                    g_423690 = optarg;
                    break;
                case 130:
                    g_423698 = 1;
                    break;
                case 131:
                    v17 = optarg;
                    break;
                case 132:
                    sub_405ba8(v44, v9, optarg);
                    break;
                case 133:
                    if (v13 && strcmp(v13, optarg))
                        error(2, 0, gettext("multiple random sources specified"));
                    v13 = optarg;
                    break;
                case 135:
                    v14 = sub_406042(v44, v9, optarg);
                    break;
                case 128:
                    v9 = (!optarg ? 99 : *((char *)(sub_40f8c1("--check", optarg, &g_422940, "CCcgeneral-numeric", 1, g_423110, 1) + 4309801)));
                case 67: case 99:
                    if (v1 && v9 != v1)
                        sub_40d6a1("cC"); /* do not return */
                    v1 = v9;
                    break;
                case 134:
                    v9 = *((char *)(sub_40f8c1("--sort", optarg, &g_422960, "ghMnRVwaiting for %s [-d]", 1, g_423110, 1) + 4309855));
                case 77: case 82: case 86: case 98: case 100: case 102: case 103: case 104: case 105: case 110: case 114:
                    v64 = v9;
                    v65 = 0;
                    sub_40d929(&v64, &v32, 2);
                    break;
                default:
LABEL_40ebdc:
                    sub_4044d3(2); /* do not return */
                }
            }
            else if (v9 == 0xffffff7d)
            {
                sub_417e44(stdout, "sort", "GNU coreutils", g_423108, "Mike Haertel", "Paul Eggert", 0);
                exit(0); /* do not return */
            }
            else if (v9 == 0xffffff7e)
            {
                sub_4044d3(0); /* do not return */
            }
        }
    }
    for (iter = g_423688; iter; iter = iter->field_40)
    {
        if ((char)sub_408305(iter) && iter->field_37 != 1)
        {
            iter->field_20 = v33;
            iter->field_28 = v34;
            iter->field_30 = v35;
            iter->field_31 = v36;
            iter->field_36 = v41;
            iter->field_32 = v37;
            iter->field_34 = v39;
            iter->field_35 = v40;
            iter->field_38 = v43;
            iter->field_33 = v38;
            iter->field_37 = v42;
        }
        v3 = iter->field_33 || v3;
    }
    if (!g_423688 && (char)sub_408305(&v32) != 1)
    {
        v0 = 1;
        sub_40d5d8(&v32);
        v3 = v38 || v3;
    }
    sub_40d6de();
    if (g_423698)
    {
        if (v1 || v18)
        {
            g_423028 = (!v1 ? 111 : v1);
            sub_40d6a1("X --debug"); /* do not return */
        }
        if (v5)
            v5 = setlocale(3, &g_41b1db);
        if (v5 != 1)
        {
            gettext("failed to set locale");
            error(0, 0, "%s");
        }
        if (g_423229)
        {
            sub_4154a0(setlocale(3, NULL));
            error(0, 0, gettext("text ordering performed using %s sorting rules"));
        }
        else
        {
            gettext("text ordering performed using simple byte comparison");
            error(0, 0, "%s");
        }
        sub_4084fe(&v32, v0);
    }
    g_423680 = v42;
    if (v3)
        sub_407573(v13);
    if (!g_423670)
    {
        v27 = getenv("TMPDIR");
        sub_40570f((!v27 ? "/tmp" : v27));
    }
    if (!v15)
    {
        v15 = 1;
        free(ptr);
        ptr = sub_418024(8);
        ptr->field_0 = "-";
    }
    if (g_423660)
    {
        v92 = g_423024 * 0x22;
        v93 = g_423660;
        if (g_423660 <= v92)
            v93 = v92;
        g_423660 = v93;
    }
    if (!v1)
    {
        sub_40ca81(ptr, v15);
        sub_40cb44(v18);
        if (v2)
        {
            v30 = sub_418502(v15, 16);
            for (index = 0; index < v15; index += 1)
            {
                *((struct struct_0 **)((char *)v30 + 16 * index)) = (&ptr->field_0)[index];
            }
            sub_40cbb9(v30, 0, v15, v18);
        }
        else
        {
            if (!v14)
            {
                v28 = sub_412de4(2);
                if (v28 > 8)
                    v28 = 8;
                v14 = v28;
            }
            v29 = 0xffffffffffffff;
            if (v14 <= v29)
                v29 = v14;
            v14 = v29;
            sub_40d0f9(ptr, v15, v18, v14);
        }
        if (!g_423683)
            return 0;
        if (sub_40ff17(stdin) == 0xffffffff)
            sub_404464(gettext("close failed"), "-"); /* do not return */
        return 0;
    }
    else if (v15 > 1)
    {
        sub_415059(4, ptr->field_8);
        error(2, 0, gettext("extra operand %s not allowed with -%c"));
    }
    else if (v18)
    {
        g_423032 = v1;
        sub_40d6a1(&g_423032); /* do not return */
    }
    else
    {
        exit((char)sub_40a0b1(*((unsigned int *)&ptr->field_0), v1) ^ 1); /* do not return */
    }
}



// Function: hash_get_n_buckets @ 0x10854
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0x1086a
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0x10880
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0x10896
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



// Function: hash_table_ok @ 0x10918
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



// Function: hash_print_statistics @ 0x109b6
typedef struct FILE {
} FILE;

int hash_print_statistics(void* a0, FILE *ptr)
{
    uint128_t v5;  // xmm0
    long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = sub_410880(a0);
    v1 = sub_410854(a0);
    v2 = sub_41086a(a0);
    v3 = sub_410896(a0);
    fprintf(ptr, "# entries:         %lu\n", v0);
    fprintf(ptr, "# buckets:         %lu\n", v1);
    v5 = (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1);
    fprintf(ptr, "# buckets used:    %lu (%.2f%%)\n", v2, (unsigned long long)(DivV(MulV(0x4059000000000000, (v2 < 0 ? AddV(v2 >> 1 | (unsigned int)v2 & 1, v2 >> 1 | (unsigned int)v2 & 1) : v2)), (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1))));
    return fprintf(ptr, "max bucket length: %lu\n", v3);
}



// Function: safe_hasher @ 0x10afd
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



// Function: hash_lookup @ 0x10b58
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

    v1 = sub_410afd(a0, a1);
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



// Function: hash_get_first @ 0x10bef
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



// Function: hash_get_next @ 0x10c4f
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

    v0 = sub_410afd(a0, a1);
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



// Function: hash_get_entries @ 0x10ced
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



// Function: hash_do_for_each @ 0x10d8d
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



// Function: hash_string @ 0x10e25
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



// Function: is_prime @ 0x10e7f
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



// Function: next_prime @ 0x10eee
unsigned long long next_prime(unsigned long long a0)
{
    unsigned long long i;  // [bp-0x10]

    i = a0;
    if (i <= 9)
        i = 10;
    for (i |= 1; i != 0xffffffffffffffff; i += 2)
    {
        if ((char)sub_410e7f(i) == 1)
            return i;
    }
    return i;
}



// Function: hash_reset_tuning @ 0x10f39
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



// Function: raw_hasher @ 0x10f8f
long long raw_hasher(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_419063(a0, 3);
    return v0 % a1;
}



// Function: raw_comparator @ 0x10fca
long long raw_comparator(unsigned long a0, unsigned long a1)
{
    return _INSERT(a0, 0, a0 == a1);
}



// Function: check_tuning @ 0x10fe7
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

extern unsigned int g_41d1f0[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int v3;  // ymm1
    unsigned int v4;  // ymm1
    unsigned int v0;  // [bp-0x14]
    unsigned int *v1;  // [bp-0x10]

    v1 = &idx->field_28->field_0;
    if (v1 == &g_41d1f0[0])
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
    idx->field_28 = &g_41d1f0[0];
    return 0;
}



// Function: compute_bucket_size @ 0x110f4
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
    v0 = sub_410eee(v0);
    v3 = 0;
    if (v0 & 0xe000000000000000)
        v3 = 1;
    if (v0 * 8 < 0)
        v3 = 1;
    if (!(v3 & 1))
        return v0;
    return 0;
}



// Function: hash_initialize @ 0x1120d
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

extern struct_0 g_41d1f0;

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
        v1 = sub_410f8f;
    if (!v0)
        v0 = sub_410fca;
    ptr = malloc(80);
    if (!ptr)
        return NULL;
    if (!v2)
        v2 = &g_41d1f0.field_0;
    ptr->field_28 = v2;
    if ((char)sub_410fe7(ptr) == 1)
    {
        *((unsigned long long *)&ptr->field_10) = sub_4110f4(a0, v2);
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



// Function: hash_clear @ 0x11382
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



// Function: hash_free @ 0x1148d
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



// Function: allocate_entry @ 0x115bc
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



// Function: free_entry @ 0x1160b
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



// Function: hash_find_entry @ 0x11645
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

    index = sub_410afd(a0, a1);
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
        sub_41160b(a0, v3);
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
        sub_41160b(a0, v5);
    }
    else
    {
        index->field_0 = 0;
    }
    return v4;
}



// Function: transfer_entries @ 0x117de
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
                idx1 = sub_410afd(index, v2);
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
                    sub_41160b(index, idx2);
                }
            }
            v2 = iter->field_0;
            iter->field_8 = NULL;
            if (!a2)
            {
                idx1 = sub_410afd(index, v2);
                if (idx1->field_0)
                {
                    v4 = sub_4115bc(index);
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



// Function: hash_rehash @ 0x119b2
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

    v0 = sub_4110f4(a1, *((long long *)&index[1].padding_8[0]));
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
        if ((char)sub_4117de(idx, index, 0))
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
        if ((char)sub_4117de(index, idx, 1) == 1 && (char)sub_4117de(index, idx, 0) == 1)
        {
            free(v2);
            return 0;
        }
        abort(); /* do not return */
    }
}



// Function: hash_insert_if_absent @ 0x11bdd
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
    v2 = sub_411645(idx, a1, &index, 0);
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
            sub_410fe7(idx);
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
                else if ((char)sub_4119b2(idx, (CmpF(v0, 0x5f000000) & 69 & 1 ? (unsigned int)v0 : (unsigned int)(SubV(v0, 0x5f000000)) ^ 0)) != 1)
                {
                    return 0xffffffff;
                }
                else if (sub_411645(idx, a1, &index, 0))
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
        v4 = sub_4115bc(idx);
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



// Function: hash_insert @ 0x11f4e
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]

    v0 = sub_411bdd(a0, a1, &v1);
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



// Function: hash_remove @ 0x11fbe
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

    v2 = sub_411645(idx, a1, &v0, 1);
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
        sub_410fe7(idx);
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
            if ((char)sub_4119b2(idx, v4) != 1)
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



// Function: hash_delete @ 0x122c4
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned long long hash_delete(struct_2 *a0, unsigned long long a1)
{
    return sub_411fbe(a0, a1);
}



// Function: num_processors_via_affinity_mask @ 0x12bc5
unsigned long long num_processors_via_affinity_mask(void)
{
    unsigned long long v0;  // [bp-0xa0]
    char v1;  // [bp-0x98]

    if (!sched_getaffinity(0, 128, &v1))
    {
        v0 = __sched_cpucount(128, &v1);
        if (v0)
            return v0;
    }
    return 0;
}



// Function: num_processors_ignoring_omp @ 0x12c4b
long long num_processors_ignoring_omp(unsigned int a0)
{
    unsigned long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    if (a0 == 1)
    {
        v2 = sub_412bc5();
        if (v2)
            return v2;
        v3 = sysconf(84);
        if (v3 > 0)
            return v3;
    }
    else
    {
        v0 = sysconf(83);
        if (v0 == 1 || v0 == 2)
        {
            v1 = sub_412bc5();
            if (v1 > v0)
                v0 = v1;
        }
        if (v0 > 0)
            return v0;
    }
    return 1;
}



// Function: parse_omp_threads @ 0x12cdc
unsigned long long parse_omp_threads(char *a0)
{
    char *i;  // [bp-0x30]
    char *j;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]
    unsigned long long num;  // [bp-0x18]

    i = a0;
    if (!i)
        return v2;
    for (v2 = 0; *(i) && (char)sub_419332(*(i)); i += 1);
    if (!(char)sub_419258(*(i)))
        return v2;
    j = NULL;
    if (j)
    {
        for (num = strtoul(i, &j, 10, &j); *(j) && (char)sub_419332(*(j)); j += 1);
        if (!*(j))
        {
            return num;
        }
        else if (*(j) == 44)
        {
            return num;
        }
    }
    return v2;
}



// Function: num_processors @ 0x12de4
long long num_processors(unsigned int a0)
{
    unsigned int v0;  // [bp-0x2c]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    v0 = a0;
    v1 = 0xffffffffffffffff;
    if (v0 == 2)
    {
        v2 = sub_412cdc(getenv("OMP_NUM_THREADS"));
        v1 = sub_412cdc(getenv("OMP_THREAD_LIMIT"));
        if (!v1)
            v1 = 0xffffffffffffffff;
        if (v2)
        {
            if (v1 > v2)
                v1 = v2;
            return v1;
        }
        v0 = 1;
    }
    v3 = sub_412c4b(v0);
    if (v1 > v3)
        return v3;
    return v1;
}



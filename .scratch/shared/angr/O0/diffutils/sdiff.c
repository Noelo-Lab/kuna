// Function: try_help @ 0x3c09
void try_help(char *a0, unsigned long long a1)
{
    unsigned long long v3;  // rbx
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x10]

    v1 = v3;
    v0 = a1;
    if (a0)
        error(0, 0, gettext(a0));
    error(2, 0, gettext("Try '%s --help' for more information."));
}



// Function: check_stdout @ 0x3c7f
typedef struct FILE {
} FILE;

extern FILE *stdout;

unsigned long long check_stdout(void)
{
    unsigned long long v1;  // rax

    if (ferror_unlocked(stdout))
        sub_403e69("write failed"); /* do not return */
    v1 = fclose(stdout);
    if (!(unsigned int)v1)
        return v1;
    sub_403eab(gettext("standard output")); /* do not return */
}



// Function: usage @ 0x3cd6
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern struct_0 *g_411a40;
extern FILE *stdout;

int usage(void)
{
    long long v12;  // r8
    long long v13;  // r9
    char *v4;  // rax
    char *v6;  // rax
    long long v8;  // rdi
    long long v9;  // rsi
    long long v10;  // rdx
    long long v11;  // rcx
    struct_0 **i;  // [bp-0x20]

    printf(gettext("Usage: %s [OPTION]... FILE1 FILE2\n"));
    printf("%s\n\n", gettext("Side-by-side merge of differences between FILE1 and FILE2."));
    v4 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
    fputs_unlocked(v4, stdout);
    for (i = &g_411a40; *(i); i += 1)
    {
        if (*(i)->field_0)
            printf("  %s\n", gettext(*(i)));
        else
            putchar_unlocked(10);
    }
    v6 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
    printf("\n%s\n%s\n", gettext("If a FILE is '-', read standard input."), v6);
    return sub_408c7b(v8, v9, v10, v11, v12, v13);
}



// Function: cleanup @ 0x3df5
extern char *g_4120a8;
extern int g_4120b8;

char * cleanup(unsigned int a0)
{
    char *v2;  // rax
    unsigned int v0;  // [bp-0xc]

    v0 = a0;
    if (g_4120b8 > 0)
        kill(g_4120b8, 13);
    v2 = g_4120a8;
    if (g_4120a8)
        v2 = unlink(g_4120a8);
    return v2;
}



// Function: exiterr @ 0x3e3e
void exiterr(void)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    long long v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9

    sub_403df5(0);
    sub_40510d(0);
    sub_4051e3(v1, v2, v3, v4, v5, v6);
    exit(2); /* do not return */
}



// Function: fatal @ 0x3e69
void fatal(char *a0)
{
    gettext(a0);
    error(0, 0, "%s");
    sub_403e3e(); /* do not return */
}



// Function: perror_fatal @ 0x3eab
void perror_fatal(unsigned long long a0)
{
    long long v3;  // rdi
    long long v4;  // rsi
    long long v5;  // rdx
    long long v6;  // rcx
    long long v7;  // r8
    long long v8;  // r9
    unsigned long long v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0xc]

    v0 = a0;
    v1 = *(__errno_location());
    sub_4051e3(v3, v4, v5, v6, v7, v8);
    error(0, v1, "%s");
    sub_403e3e(); /* do not return */
}



// Function: check_child_status @ 0x3ef1
long long check_child_status(int a0, int a1, unsigned int a2, unsigned long long a3)
{
    unsigned int v3;  // eax
    unsigned long long v0;  // [bp-0x30]
    unsigned int v1;  // [bp-0xc]

    v0 = a3;
    if (!a0 && !((char)a1 & 127))
        v3 = a1 >> 8 & 0xff;
    else
        v3 = 0x7fffffff;
    v1 = v3;
    if (a2 >= v1)
        return a2;
    error(0, a0, gettext((v1 == 126 ? "subsidiary program '%s' could not be invoked" : (v1 == 127 ? "subsidiary program '%s' not found" : (v1 == 0x7fffffff ? "subsidiary program '%s' failed" : "subsidiary program '%s' failed (exit status %d)")))));
    sub_403e3e(); /* do not return */
}



// Function: ck_fopen @ 0x3fa3
typedef struct FILE {
} FILE;

FILE * ck_fopen(unsigned long a0, char *a1)
{
    FILE *fp;  // [bp-0x10]

    fp = fopen(a0, a1);
    if (fp)
        return fp;
    sub_403eab(a0); /* do not return */
}



// Function: ck_fclose @ 0x3fe7
typedef struct FILE {
} FILE;

int ck_fclose(FILE *a0)
{
    unsigned int v1;  // eax

    v1 = fclose(a0);
    if (!v1)
        return v1;
    sub_403eab("fclose"); /* do not return */
}



// Function: ck_fread @ 0x4019
typedef struct FILE {
} FILE;

long long ck_fread(void* a0, unsigned long a1, FILE *a2)
{
    unsigned long count;  // [bp-0x10]

    count = fread_unlocked(a0, 1, a1, a2);
    if (count)
    {
        return count;
    }
    else if (ferror_unlocked(a2))
    {
        sub_403eab(gettext("read failed")); /* do not return */
    }
    else
    {
        return count;
    }
}



// Function: ck_fwrite @ 0x4082
typedef struct FILE {
} FILE;

long long ck_fwrite(void* a0, unsigned long a1, FILE *a2)
{
    unsigned long count;  // rax

    count = fwrite_unlocked(a0, 1, a1, a2);
    if (a1 == count)
        return count;
    sub_403eab(gettext("write failed")); /* do not return */
}



// Function: ck_fflush @ 0x40d3
typedef struct FILE {
} FILE;

int ck_fflush(FILE *a0)
{
    unsigned int v1;  // eax

    v1 = fflush_unlocked(a0);
    if (!v1)
        return v1;
    sub_403eab(gettext("write failed")); /* do not return */
}



// Function: expand_name @ 0x410d
long long expand_name(unsigned long ptr, char a1, long long a2)
{
    char v6;  // al
    char v0;  // [bp-0x29]
    unsigned long v1;  // [bp-0x28]
    unsigned long len;  // [bp-0x20]
    unsigned long n;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    if (!strcmp(ptr, "-"))
    {
        sub_403e69("cannot interactively merge standard input"); /* do not return */
    }
    else if (a1 != 1)
    {
        return ptr;
    }
    else
    {
        v1 = sub_405f32(a2);
        len = strlen(ptr);
        n = sub_405f9f(v1);
        if (*((char *)sub_405f32(ptr)) && *((char *)(ptr + len - 1)) != 47)
            v6 = 1;
        else
            v6 = 0;
        v0 = v6;
        v0 &= 1;
        v4 = sub_408dc0(n + v0 + len + 1);
        memcpy(v4, ptr, len);
        *((char *)(len + v4)) = 47;
        memcpy(v0 + len + v4, v1, n);
        *((char *)(v4 + v0 + len + n)) = 0;
        return v4;
    }
}



// Function: lf_init @ 0x425e
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_1 *field_8;
    struct struct_1 *field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

char * lf_init(struct_0 *idx, unsigned long a1)
{
    char *v1;  // rax

    idx->field_0 = a1;
    idx->field_18 = sub_408dc0(65537);
    idx->field_10 = idx->field_18;
    idx->field_8 = idx->field_10;
    v1 = &idx->field_18->field_0;
    *(v1) = 10;
    return v1;
}



// Function: lf_refill @ 0x42c0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct FILE *field_0;
    void* field_8;
    void* field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

long long lf_refill(struct_0 *idx)
{
    char *v2;  // rdx
    long long v3;  // rdi
    long long v4;  // rsi
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    long long v0;  // [bp-0x10]

    v0 = sub_404019(idx->field_10, 0x10000, idx->field_0);
    idx->field_8 = idx->field_10;
    v2 = idx->field_10 + v0;
    idx->field_18 = v2;
    idx->field_18->field_0 = 10;
    sub_4051e3(v3, v4, v2, v5, v6, v7);
    return v0;
}



// Function: lf_copy @ 0x432d
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct FILE *field_0;
    void* field_8;
    void* field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

long long lf_copy(struct_0 *idx, unsigned long long a1, FILE *a2)
{
    long long v3;  // rax
    unsigned long long i;  // [bp-0x28]
    unsigned long v1;  // [bp-0x10]

    i = a1;
    v1 = idx->field_8;
    while (i)
    {
        idx->field_8 = rawmemchr(idx->field_8, 10);
        if (idx->field_8 == idx->field_18)
        {
            sub_404082(v1, &idx->field_18[-1 * v1], a2);
            v3 = sub_4042c0(idx);
            if (!v3)
                return v3;
            v1 = idx->field_8;
        }
        else
        {
            i -= 1;
            idx->field_8 = idx->field_8 + 1;
        }
    }
    return sub_404082(v1, idx->field_8 - v1, a2);
}



// Function: lf_skip @ 0x4414
void* * lf_skip(void* *idx, unsigned long long a1)
{
    void* *v2;  // rax
    unsigned long long i;  // [bp-0x18]

    for (i = a1; i; v2[1] = idx[1] + 1)
    {
        idx[1] = rawmemchr(idx[1], 10);
        if (idx[1] == idx[3])
        {
            v2 = sub_4042c0(idx);
            if (!v2)
                return v2;
        }
        else
        {
            i -= 1;
            v2 = idx;
        }
    }
    return v2;
}



// Function: lf_snarf @ 0x4493
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct FILE *field_0;
    void* field_8;
    void* field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

unsigned int lf_snarf(struct_0 *idx, void* a1, unsigned long a2)
{
    unsigned long v0;  // [bp-0x40]
    void* iter;  // [bp-0x38]
    unsigned long v2;  // [bp-0x20]
    unsigned long ptr;  // [bp-0x18]
    unsigned long n;  // [bp-0x10]

    iter = a1;
    v0 = a2;
    while (1)
    {
        v2 = idx->field_8;
        ptr = rawmemchr(v2, 10);
        n = ptr - v2;
        if (v0 <= n)
            return 0;
        memcpy(iter, v2, n);
        if (ptr < idx->field_18)
        {
            *(n + (char *)iter) = 0;
            idx->field_8 = ptr + 1;
            return 1;
        }
        if (!sub_4042c0(idx))
            return (!n ? 0xffffffff : 0);
        iter += n;
        v0 -= n;
    }
}



// Function: main @ 0x4573
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_1 *field_8;
    struct struct_1 *field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_40d1e3;
extern option g_4117a0;
extern unsigned long long g_412010;
extern long long g_412018;
extern unsigned int g_412020;
extern long long stdout;
extern int optind;
extern unsigned long long optarg;
extern void* g_4120a0;
extern char *g_4120a8;
extern int g_4120b8;
extern unsigned long long g_412450;
extern unsigned long long g_4124e8;
extern char g_4124f0;

unsigned int main(int a0, void* a1)
{
    long long v19;  // rdi
    long long v20;  // rsi
    long long v21;  // rdx
    long long v22;  // rcx
    long long v23;  // r8
    long long v24;  // r9
    long long v25;  // rax
    char v0;  // [bp-0xbf]
    char v1;  // [bp-0xbe]
    char v2;  // [bp-0xbd]
    int v3;  // [bp-0xbc]
    unsigned int v4;  // [bp-0xb8]
    int v5;  // [bp-0xb4]
    char *v6;  // [bp-0xb0]
    long long v7;  // [bp-0xa8]
    FILE *v8;  // [bp-0xa0]
    long long v9;  // [bp-0x98]
    FILE *v10;  // [bp-0x90]
    unsigned long v11;  // [bp-0x88]
    FILE *fp;  // [bp-0x80]
    struct_0 v13;  // [bp-0x78]
    struct_0 v14;  // [bp-0x58]
    struct_0 v15;  // [bp-0x38]
    char v16;  // [bp-0x18]
    int v17;  // [bp-0x14]

    g_412020 = 2;
    sub_40632f(*((long long *)a1));
    setlocale(6, &g_40d1e3);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_40626b(sub_403df5);
    sub_4093c4(v19, v20, v21, v22, v23, v24);
    v6 = getenv("EDITOR");
    if (v6)
        g_412010 = v6;
    sub_404ebd("diff");
    while (1)
    {
        v4 = getopt_long(a0, a1, "abBdEHiI:lo:stvw:WZ", &g_4117a0.name, NULL);
        if (v4 == 0xffffffff)
            break;
        switch (v4)
        {
        case 66:
            sub_404ebd("-B");
            break;
        case 69:
            sub_404ebd("-E");
            break;
        case 72:
            sub_404ebd("-H");
            break;
        case 73:
            sub_404ebd("-I");
            sub_404ebd(optarg);
            break;
        case 87:
            sub_404ebd("-w");
            break;
        case 90:
            sub_404ebd("-Z");
            break;
        case 97:
            sub_404ebd("-a");
            break;
        case 98:
            sub_404ebd("-b");
            break;
        case 100:
            sub_404ebd("-d");
            break;
        case 105:
            sub_404ebd("-i");
            break;
        case 108:
            sub_404ebd("--left-column");
            break;
        case 111:
            g_4124e8 = optarg;
            break;
        case 115:
            g_4124f0 = 1;
            break;
        case 116:
            sub_404ebd("-t");
            break;
        case 118:
            v25 = sub_4068f0("Thomas Lord");
            sub_408b97(stdout, "sdiff", "GNU diffutils", g_412018, v25, 0);
            sub_403c7f();
            return 0;
        case 119:
            sub_404ebd("-W");
            sub_404ebd(optarg);
            break;
        case 128:
            *((unsigned long long *)g_4120a0) = optarg;
            break;
        case 129:
            sub_403cd6();
            sub_403c7f();
            return 0;
        case 130:
            sub_404ebd("--strip-trailing-cr");
            break;
        case 131:
            sub_404ebd("--tabsize");
            sub_404ebd(optarg);
            break;
        default:
            sub_403c09(NULL, 0); /* do not return */
        }
    }
    if (a0 - optind != 2)
    {
        if (a0 - optind <= 1)
            sub_403c09("missing operand after '%s'", *((long long *)((char *)a1 + 8 * a0 - 8))); /* do not return */
        sub_403c09("extra operand '%s'", *((long long *)((char *)a1 + 8 * optind + 16))); /* do not return */
    }
    else
    {
        if (!g_4124e8)
        {
            if (g_4124f0)
                sub_404ebd("--suppress-common-lines");
            sub_404ebd("-y");
            sub_404ebd("--");
            sub_404ebd(*((long long *)((char *)a1 + 8 * optind)));
            sub_404ebd(*((long long *)((char *)a1 + 8 * optind + 8)));
            sub_404ebd(0);
            execvp(*((long long *)g_4120a0), g_4120a0);
            sub_403eab(*((long long *)g_4120a0)); /* do not return */
        }
        else
        {
            v0 = sub_405e22(*((long long *)((char *)a1 + 8 * optind)));
            v1 = sub_405e22(*((long long *)((char *)a1 + 8 * optind + 8)));
            if (v1 & v0)
                sub_403e69("both files to be compared are directories"); /* do not return */
            v7 = sub_40410d(*((long long *)((char *)a1 + 8 * optind)), v0, *((long long *)((char *)a1 + 8 * optind + 8)));
            v8 = sub_403fa3(v7, "r");
            v9 = sub_40410d(*((long long *)((char *)a1 + 8 * optind + 8)), v1, *((long long *)((char *)a1 + 8 * optind)));
            v10 = sub_403fa3(v9, "r");
            v11 = sub_403fa3(g_4124e8, "w");
            sub_404ebd("--sdiff-merge-assist");
            sub_404ebd("--");
            sub_404ebd(*((long long *)((char *)a1 + 8 * optind)));
            sub_404ebd(*((long long *)((char *)a1 + 8 * optind + 8)));
            sub_404ebd(0);
            sub_404fdd(v19, v20, v21, v22, v23, v24);
            if (pipe(&v16))
                sub_403eab("pipe"); /* do not return */
            g_4120b8 = fork();
            if (g_4120b8 < 0)
            {
                sub_403eab("fork"); /* do not return */
            }
            else if (!g_4120b8)
            {
                if (g_412450 != 1)
                    sub_404fa3(2, 1);
                sub_404fa3(13, 0);
                close(v16);
                if (v17 != 1)
                {
                    dup2(v17, 1);
                    close(v17);
                }
                execvp(*((long long *)g_4120a0), g_4120a0);
                _exit((*(__errno_location()) == 2 ? 127 : 126)); /* do not return */
            }
            else
            {
                close(v17);
                fp = fdopen(v16, "r");
                if (!fp)
                    sub_403eab("fdopen"); /* do not return */
                sub_40425e(&v15, fp);
                sub_40425e(&v13, v8);
                sub_40425e(&v14, v10);
                v2 = sub_405b03(&v15, &v13, v7, &v14, v9, v11);
                sub_403fe7(v8);
                sub_403fe7(v10);
                sub_403fe7(v11);
                v5 = 0;
                sub_403fe7(fp);
                while (waitpid(g_4120b8, &v3, 0) < 0)
                {
                    if (*(__errno_location()) == 4)
                        sub_4051e3();
                    else
                        sub_403eab("waitpid"); /* do not return */
                }
                g_4120b8 = 0;
                if (g_4120a8)
                {
                    unlink(g_4120a8);
                    g_4120a8 = 0;
                }
                if (v2 == 1)
                {
                    sub_403ef1(v5, v3, 1, *((long long *)g_4120a0));
                    sub_40510d(0);
                    sub_4051e3(v19, v20, v21, v22, v23, v24);
                    exit((char)(v3 >> 8)); /* do not return */
                }
                sub_403e3e(); /* do not return */
            }
        }
    }
}



// Function: diffarg @ 0x4ebd
extern long long g_4120a0;
extern unsigned long long g_412598;
extern unsigned long long g_4125a0;

long long diffarg(unsigned long a0)
{
    unsigned long long v1;  // rdx
    unsigned long long idx;  // rax

    if (g_412598 == g_4125a0)
    {
        if (!g_4125a0)
        {
            g_4125a0 = 16;
        }
        else
        {
            if (g_4125a0 > 0x7fffffffffffffe)
                sub_409384(); /* do not return */
            g_4125a0 = g_4125a0 * 2;
        }
        v1 = g_4125a0 * 8;
        g_4120a0 = sub_408e0c(g_4120a0, v1, v1);
    }
    idx = g_412598;
    g_412598 = idx + 1;
    *((unsigned long *)(g_4120a0 + idx * 8)) = a0;
    return a0;
}



// Function: catchsig @ 0x4f78
extern char g_4124f1;
extern unsigned int g_4124f4;

long long catchsig(unsigned int a0)
{
    unsigned long v1;  // rax

    if (a0 == 2)
    {
        v1 = g_4124f1 ^ 1;
        if (!(char)v1)
            return v1;
    }
    g_4124f4 = a0;
    return a0;
}



// Function: signal_handler @ 0x4fa3
typedef struct sigaction {
} sigaction;

extern sigaction g_412500;

int signal_handler(int a0, unsigned long a1)
{
    *((unsigned long *)&g_412500) = a1;
    return sigaction(a0, &g_412500, NULL);
}



// Function: trapsigs @ 0x4fdd
extern unsigned int g_40d020[4];
extern unsigned long long g_4120c0[4];
extern char g_4124f8;
extern int g_412508;
extern unsigned int g_412588;

sighandler_t trapsigs(void)
{
    sighandler_t v2;  // rax
    int iter;  // [bp-0xc]

    g_412588 = 0x10000000;
    sigemptyset(&g_412508);
    for (iter = 0; iter <= 6; iter += 1)
    {
        sigaddset(&g_412508, g_40d020[iter]);
    }
    for (iter = 0; iter <= 6; iter += 1)
    {
        sigaction(g_40d020[iter], NULL, &g_4120c0[19 * iter]);
        if (g_4120c0[19 * iter] != 0x1)
            sub_404fa3(g_40d020[iter], sub_404f78);
    }
    v2 = signal(0x11, NULL);
    g_4124f8 = 1;
    return v2;
}



// Function: untrapsig @ 0x510d
extern unsigned int g_40d020[4];
extern unsigned long long g_4120c0[4];
extern char g_4124f8;

long long untrapsig(unsigned int *a0)
{
    unsigned long i;  // rax
    unsigned long v3;  // rax
    int v0;  // [bp-0xc]

    i = g_4124f8;
    if (!g_4124f8)
        return g_4124f8;
    for (v0 = 0; v0 <= 6; i = v3)
    {
        if ((!a0 || a0 == g_40d020[v0]) && g_4120c0[19 * v0] != 0x1)
            sigaction(g_40d020[v0], &g_4120c0[19 * v0], NULL);
        v0 += 1;
    }
    return i;
}



// Function: checksigs @ 0x51e3
extern unsigned int g_4124f4;

long long checksigs(void)
{
    unsigned int v0;  // [bp-0xc]

    v0 = g_4124f4;
    if (!v0)
        return g_4124f4;
    sub_403df5(0);
    sub_40510d(v0);
    raise(v0);
    exit(2); /* do not return */
}



// Function: give_help @ 0x5229
typedef struct FILE {
} FILE;

extern FILE *stderr;

int give_help(void)
{
    char *v1;  // rax

    v1 = gettext("ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n");
    return fprintf(stderr, "%s", v1);
}



// Function: skip_white @ 0x5264
typedef struct FILE {
} FILE;

extern FILE *stdin;

int skip_white(void)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned long long *v8;  // rax
    unsigned int v0;  // [bp-0xc]

    while (1)
    {
        v0 = getchar_unlocked();
        v8 = __ctype_b_loc(v2, v3, v4, v5, v6, v7);
        if (!(*((short *)(v0 * 2 + *(v8))) & 0x2000) || v0 == 10)
            break;
        sub_4051e3();
    }
    if (!ferror_unlocked(stdin))
        return v0;
    sub_403eab(gettext("read failed")); /* do not return */
}



// Function: flush_line @ 0x52d6
typedef struct FILE {
} FILE;

extern FILE *stdin;

int flush_line(void)
{
    unsigned int v2;  // eax
    unsigned int v0;  // [bp-0xc]

    do
    {
        v0 = getchar_unlocked();
    } while (v0 != 10 && v0 != 0xffffffff);
    v2 = ferror_unlocked(stdin);
    if (!v2)
        return v2;
    sub_403eab(gettext("read failed")); /* do not return */
}



// Function: edit @ 0x5326
typedef struct FILE {
} FILE;

extern unsigned long long g_412010;
extern FILE *stdout;
extern FILE *stdin;
extern unsigned long long g_4120a8;
extern FILE *g_4120b0;
extern char g_4124f0;
extern char g_4124f1;

unsigned int edit(void* a0, char *a1, int a2, unsigned long long a3, void* a4, char *a5, int a6, unsigned long long a7, FILE *a8)
{
    unsigned long v18;  // fs
    unsigned int v20;  // eax
    char i;  // [bp-0x10079]
    int v1;  // [bp-0x10078]
    unsigned int v2;  // [bp-0x10074]
    unsigned int v3;  // [bp-0x10070]
    unsigned int v4;  // [bp-0x10070]
    unsigned int v5;  // [bp-0x1006c]
    int v6;  // [bp-0x10068]
    unsigned int v7;  // [bp-0x10064]
    unsigned long v8;  // [bp-0x10058]
    unsigned long v9;  // [bp-0x10048]
    long long v10;  // [bp-0x10040]
    unsigned long v11;  // [bp-0x10038]
    unsigned long long v12;  // [bp-0x10030]
    unsigned long long v13;  // [bp-0x10028]
    char v14;  // [bp-0x10018]
    char v15;  // [bp-0x10008]
    unsigned long long v16;  // [bp-0x2008]
    unsigned long v17;  // [bp-0x10]

    do
    {
    } while (&v16 != &v15);
    v17 = *((long long *)(40 + v18));
    while (1)
    {
        i = 0;
        v4 = v3;
        while (i != 1)
        {
            if (putchar_unlocked(37) != 37)
                sub_403eab(gettext("write failed")); /* do not return */
            sub_4040d3(stdout);
            v2 = sub_405264();
            if (v2 != 118)
            {
                if (v2 > 118 || v2 > 115)
                {
LABEL_405552:
                    sub_4052d6();
LABEL_405557:
                    sub_405229();
                    continue;
                }
                if (v2 >= 113 || v2 == 108)
                    goto LABEL_40546f;
                if (v2 > 108)
                    goto LABEL_405552;
                if (v2 != 101 && v2 <= 101 && v2 <= 50)
                {
                    if (v2 >= 49)
                        goto LABEL_40546f;
                    if (v2 != 0xffffffff)
                    {
                        if (v2 == 10)
                            goto LABEL_405557;
                        goto LABEL_405552;
                    }
                    else if (feof_unlocked(stdin))
                    {
                        i = 1;
                        v2 = 113;
                        continue;
                    }
                }
                v4 = sub_405264();
                if (v4 > 114)
                {
LABEL_40551e:
                    sub_405229();
                    sub_4052d6();
                    continue;
                }
                if (v4 < 98)
                {
                    if (v4 == 10)
                    {
                        i = 1;
LABEL_40552a:
                        continue;
                    }
                    else if (v4 < 10 || v4 - 49 > 1)
                    {
                        goto LABEL_40551e;
                    }
                }
                else
                {
                    if (!((unsigned int)(66565 >> ((char)(v4 - 98) & 63)) & 1))
                        goto LABEL_40551e;
                }
                if (sub_405264() != 10)
                {
                    sub_405229();
                    sub_4052d6();
                }
                else
                {
                    i = 1;
                    goto LABEL_40552a;
                }
            }
            else
            {
LABEL_40546f:
                if (sub_405264() != 10)
                {
                    sub_405229();
                    sub_4052d6();
                }
                else
                {
                    i = 1;
                }
            }
        }
        if (v2 > 118)
            goto LABEL_405ae2;
        if (v2 < 101)
        {
            if (v2 == 49)
                goto LABEL_4055d1;
            if (v2 == 50)
                goto LABEL_405611;
            else
                goto LABEL_405ae2;
        }
        else
        {
            switch (v2)
            {
            case 101:
                if (g_4120a8)
                {
                    g_4120b0 = fopen(g_4120a8, "w");
                }
                else
                {
                    v5 = sub_405e9b();
                    if (v5 < 0)
                        sub_403eab("mkstemp"); /* do not return */
                    g_4120b0 = fdopen(v5, "w");
                }
                if (!g_4120b0)
                    sub_403eab(g_4120a8); /* do not return */
                switch (v4)
                {
                case 100:
                    if (a3)
                    {
                        v8 = a3 + a2 - 1;
                        if (a3 == 1)
                        {
                            fprintf(g_4120b0, "--- %s %ld\n", a1, a2);
                            goto LABEL_4057ec;
                        }
                        else
                        {
                            fprintf(g_4120b0, "--- %s %ld,%ld\n", a1, a2, v8);
                        }
                    }
                    break;
                case 49: case 98: case 108:
LABEL_4057ec:
                    sub_40432d(a0, a3, g_4120b0);
                    break;
                default:
                    sub_404414(a0, a3);
                    break;
                }
                switch (v4)
                {
                case 100:
                    if (a7)
                    {
                        v9 = a7 + a6 - 1;
                        if (a7 == 1)
                        {
                            fprintf(g_4120b0, "+++ %s %ld\n", a5, a6);
                            goto LABEL_405908;
                        }
                        else
                        {
                            fprintf(g_4120b0, "+++ %s %ld,%ld\n", a5, a6, v9);
                        }
                    }
                    break;
                case 50: case 98: case 114:
LABEL_405908:
                    sub_40432d(a4, a7, g_4120b0);
                    break;
                default:
                    sub_404414(a4, a7);
                    break;
                }
                sub_403fe7(g_4120b0);
                v6 = 0;
                g_4124f1 = 1;
                sub_4051e3();
                v11 = g_412010;
                v12 = g_4120a8;
                v13 = 0;
                v7 = fork();
                if (!v7)
                {
                    execvp(g_412010, &v11);
                    _exit((*(__errno_location()) == 2 ? 127 : 126)); /* do not return */
                }
                if (v7 < 0)
                    sub_403eab("fork"); /* do not return */
                while (waitpid(v7, &v1, 0) < 0)
                {
                    if (*(__errno_location()) == 4)
                        sub_4051e3();
                    else
                        sub_403eab("waitpid"); /* do not return */
                }
                g_4124f1 = 0;
                sub_403ef1(v6, v1, 0, g_412010);
                g_4120b0 = sub_403fa3(g_4120a8, "r");
                while (1)
                {
                    v10 = sub_404019(&v14, 0x10000, g_4120b0);
                    if (!v10)
                        break;
                    sub_4051e3();
                    sub_404082(&v14, v10, a8);
                }
                sub_403fe7(g_4120b0);
                v20 = 1;
                break;
            case 108:
LABEL_4055d1:
                sub_40432d(a0, a3, a8);
                sub_404414(a4, a7);
                v20 = 1;
                break;
            case 113:
                v20 = 0;
                break;
            case 114:
LABEL_405611:
                sub_40432d(a4, a7, a8);
                sub_404414(a0, a3);
                v20 = 1;
                break;
            case 115:
                g_4124f0 = 1;
                goto LABEL_405ae8;
            case 118:
                g_4124f0 = 0;
                goto LABEL_405ae8;
            default:
LABEL_405ae2:
                sub_405229();
                goto LABEL_405ae8;
            }
LABEL_405ae8:
        }
    }
    if (v17 == *((long long *)(40 + v18)))
        return v20;
    __stack_chk_fail(); /* do not return */
}



// Function: interact @ 0x5b03
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_4124f0;

unsigned int interact(void* a0, void* a1, char *a2, void* a3, char *a4, FILE *a5)
{
    unsigned long v11;  // rax
    unsigned int v0;  // [bp-0x154]
    unsigned long v1;  // [bp-0x150]
    unsigned long long v2;  // [bp-0x148]
    unsigned long long v3;  // [bp-0x140]
    unsigned long v4;  // [bp-0x138]
    unsigned long v5;  // [bp-0x130]
    unsigned long v6;  // [bp-0x128]
    unsigned long v7;  // [bp-0x120]
    char v8;  // [bp-0x118]
    char v9[263];  // [bp-0x117]

    v2 = 1;
    v3 = 1;
    while (1)
    {
        v0 = sub_404493(a0, &v8, 0x100);
        if (v0 <= 0)
            return _INSERT(v0, 0, v0);
        sub_4051e3();
        if (v8 == 32)
        {
            puts(v9);
        }
        else
        {
            *(__errno_location()) = 0;
            v4 = strtoimax(v9, &v1, 10, &v1);
            if (v4 - 0 >> 63 & 1 || *(__errno_location()) || *((char *)v1) != 44)
                sub_403e69(&v8); /* do not return */
            v5 = v4;
            v4 = strtoimax(v1 + 1, &v1, 10, v1 + 1);
            if (v4 - 0 >> 63 & 1 || *(__errno_location()) || *((char *)v1))
                sub_403e69(&v8); /* do not return */
            v6 = v4;
            if (v6 <= v5)
                v11 = v5;
            else
                v11 = v6;
            v7 = v11;
            switch (v8)
            {
            case 99:
                sub_40432d(a0, v7, stdout);
                if ((char)sub_405326(a1, a2, v2, v5, a3, a4, v3, v6, a5) != 1)
                    return 0;
                break;
            case 105:
                if (g_4124f0)
                    sub_404414(a0, v7);
                else
                    sub_40432d(a0, v7, stdout);
                sub_40432d(a1, v5, a5);
                sub_404414(a3, v6);
                break;
            default:
                sub_403e69(&v8); /* do not return */
            }
            v2 += v5;
            v3 += v6;
        }
    }
}



// Function: diraccess @ 0x5e22
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

long long diraccess(char *a0)
{
    unsigned int v3;  // eax
    stat v0;  // [bp-0xa8]

    if (!stat(a0, &v0) && ((unsigned short)v0.st_nlink & 0xf000) == 0x4000)
        v3 = 1;
    else
        v3 = 0;
    return v3 & 1;
}



// Function: temporary_file @ 0x5e9b
extern unsigned long long g_4120a8;

int temporary_file(void)
{
    unsigned int v0;  // [bp-0x24]
    char *v1;  // [bp-0x20]
    char *v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    v1 = getenv("TMPDIR");
    if (!v1)
        v1 = "/tmp";
    v2 = v1;
    v3 = sub_408dc0(strlen(v2) + 13);
    sprintf(v3, "%s/sdiffXXXXXX", v2);
    v0 = mkstemp(v3);
    if (v0 >= 0)
        g_4120a8 = v3;
    return v0;
}



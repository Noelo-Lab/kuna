// Function: main @ 0x3b60
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_40a94d;
extern option g_40e860;
extern unsigned long long g_40f010;
extern long long g_40f018;
extern unsigned int g_40f020;
extern long long stdout;
extern int optind;
extern unsigned long long optarg;
extern char g_40f161;
extern unsigned long long g_40f168;
extern unsigned long long g_40f510;
extern int g_40f5a8;
extern char *g_40f5b8;
extern void* g_40f5c0;

unsigned int main(int a0, long long *a1)
{
    unsigned long v10;  // rbp
    long long v11;  // rdi
    long long v20;  // rax
    unsigned int v21;  // edx
    char v22;  // al
    char v23;  // al
    long long v24;  // rax
    long long v25;  // r12
    long long v26;  // rax
    long long v27;  // rbp
    unsigned long fp;  // rax
    char v29;  // al
    long long v12;  // rdi
    unsigned int *err;  // rax
    long long v13;  // rsi
    long long v14;  // rdx
    long long v15;  // rcx
    long long v16;  // r8
    long long v17;  // r9
    char *v18;  // rax
    unsigned int v19;  // eax
    long long v0;  // [bp-0xd0]
    char *v1;  // [bp-0xc8]
    char *v2;  // [bp-0xc0]
    int v3;  // [bp-0xac]
    char v4;  // [bp-0xa8]
    char v5;  // [bp-0x88]
    char v6;  // [bp-0x68]
    char v7;  // [bp-0x48]
    int v8;  // [bp-0x44]

    v10 = a0;
    v11 = *(a1);
    g_40f020 = 2;
    sub_405960(v11);
    setlocale(6, &g_40a94d);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_4058a0(sub_404300);
    sub_407bf0(v12, v13, v14, v15, v16, v17);
    v18 = getenv("EDITOR");
    if (v18)
        g_40f010 = v18;
    sub_4045b0("diff");
    while (1)
    {
        v19 = getopt_long(v10, a1, "abBdEHiI:lo:stvw:WZ", &g_40e860.name, NULL);
        if (v19 == 0xffffffff)
            break;
        switch (v19)
        {
        case 66:
            sub_4045b0("-B");
            break;
        case 69:
            sub_4045b0("-E");
            break;
        case 72:
            sub_4045b0("-H");
            break;
        case 73:
            sub_4045b0("-I");
            sub_4045b0(optarg);
            break;
        case 87:
            sub_4045b0("-w");
            break;
        case 90:
            sub_4045b0("-Z");
            break;
        case 97:
            sub_4045b0("-a");
            break;
        case 98:
            sub_4045b0("-b");
            break;
        case 100:
            sub_4045b0("-d");
            break;
        case 105:
            sub_4045b0("-i");
            break;
        case 108:
            sub_4045b0("--left-column");
            break;
        case 111:
            g_40f168 = optarg;
            break;
        case 115:
            g_40f161 = 1;
            break;
        case 116:
            sub_4045b0("-t");
            break;
        case 118:
            v20 = sub_405c70("Thomas Lord");
            sub_407690(stdout, "sdiff", "GNU diffutils", g_40f018, v20, 0);
            sub_404a10(v12, v13, v14, v15, v16, v17);
            return 0;
        case 119:
            sub_4045b0("-W");
            sub_4045b0(optarg);
            break;
        case 128:
            *((unsigned long long *)g_40f5c0) = optarg;
            break;
        case 129:
            sub_404380();
            sub_404a10(v12, v13, v14, v15, v16, v17);
            return 0;
        case 130:
            sub_4045b0("--strip-trailing-cr");
            break;
        case 131:
            sub_4045b0("--tabsize");
            sub_4045b0(optarg);
            break;
        default:
            sub_4044c0(0, 0); /* do not return */
        }
    }
    v21 = (v10 & 0xffffffff) - optind;
    if (v21 != 2)
    {
        if (v21 > 1)
            sub_4044c0("extra operand '%s'", a1[2 + optind]); /* do not return */
        sub_4044c0("missing operand after '%s'", *((long long *)((char *)&a1[v10] - 8))); /* do not return */
    }
    else
    {
        if (g_40f168)
        {
            v22 = sub_404520(a1[optind]);
            v23 = sub_404520(a1[1 + optind]);
            if (v22 && v23)
                sub_4048d0("both files to be compared are directories"); /* do not return */
            v24 = sub_404900(a1[optind]);
            v25 = sub_404a60(v24, "r");
            v26 = sub_404900(a1[1 + optind], v23, a1[optind]);
            v27 = sub_404a60(v26, "r");
            v0 = sub_404a60(g_40f168, "w");
            sub_4045b0("--sdiff-merge-assist");
            sub_4045b0("--");
            sub_4045b0(a1[optind]);
            sub_4045b0(a1[1 + optind]);
            sub_4045b0(0);
            sub_4046f0(v12, v13, v14, v15, v16, v17);
            if (pipe(&v7))
                sub_4049e0("pipe"); /* do not return */
            g_40f5a8 = fork();
            if (g_40f5a8 < 0)
            {
                sub_4049e0("fork"); /* do not return */
            }
            else if (g_40f5a8)
            {
                close(v8);
                fp = fdopen(v7, "r");
                if (!fp)
                    sub_4049e0("fdopen"); /* do not return */
                sub_404580(&v6, fp);
                v2 = &v4;
                sub_404580(&v4, v25);
                v1 = &v5;
                sub_404580(&v5, v27);
                v29 = sub_405480(&v6, v2, v24, v1, v26, v0);
                sub_404b40(v25);
                sub_404b40(v27);
                sub_404b40(v0);
                sub_404b40(fp);
                while (waitpid(g_40f5a8, &v3, 0) < 0)
                {
                    if (*(__errno_location()) == 4)
                        sub_404870();
                    else
                        sub_4049e0("waitpid"); /* do not return */
                }
                g_40f5a8 = 0;
                if (g_40f5b8)
                {
                    unlink(g_40f5b8);
                    g_40f5b8 = 0;
                }
                if (v29)
                {
                    sub_404e40(v3, 1, *((long long *)g_40f5c0));
                    sub_404660(0);
                    sub_404870(v12, v13, v14, v15, v16, v17);
                    exit((char)v3); /* do not return */
                }
                sub_4048a0(); /* do not return */
            }
            else
            {
                if (g_40f510 != 1)
                    sub_404640(2, 1);
                sub_404640(13, 0);
                close(v7);
                if (v8 != 1)
                {
                    dup2(v8, 1);
                    close(v8);
                }
                execvp(*((long long *)g_40f5c0), g_40f5c0);
                err = __errno_location();
                _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
            }
        }
        else
        {
            if (g_40f161)
                sub_4045b0("--suppress-common-lines");
            sub_4045b0("-y");
            sub_4045b0("--");
            sub_4045b0(a1[optind]);
            sub_4045b0(a1[1 + optind]);
            sub_4045b0(0);
            execvp(*((long long *)g_40f5c0), g_40f5c0);
            sub_4049e0(*((long long *)g_40f5c0)); /* do not return */
        }
    }
}



// Function: catchsig @ 0x42e0
extern unsigned int g_40f15c;
extern char g_40f160;

long long catchsig(unsigned int a0)
{
    unsigned long v1;  // rax

    if (a0 == 2)
    {
        v1 = g_40f160;
        if (g_40f160)
            return g_40f160;
    }
    g_40f15c = a0;
    return v1;
}



// Function: cleanup @ 0x4300
extern int g_40f5a8;
extern char *g_40f5b8;

unsigned long long cleanup(void)
{
    if (g_40f5a8 > 0)
    {
        kill(g_40f5a8, 13);
        if (!g_40f5b8)
            return 0;
        return unlink(g_40f5b8);
    }
    else if (g_40f5b8)
    {
        return unlink(g_40f5b8);
    }
    else
    {
        return 0;
    }
}



// Function: usage @ 0x4380
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern struct_0 *g_40e780;
extern FILE *stdout;

int usage(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    struct_0 **iter;  // rbx
    char *v8;  // rax
    char v9;  // al
    char *v10;  // rsi
    char *v11;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    iter = &g_40e780;
    __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... FILE1 FILE2\n", 5));
    __printf_chk(1, "%s\n\n", dcgettext(NULL, "Side-by-side merge of differences between FILE1 and FILE2.", 5));
    v8 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v8, stdout);
    v9 = 45;
    v10 = "-o, --output=FILE            operate interactively, sending output to FILE";
    while (1)
    {
        if (v9)
        {
            iter += 1;
            dcgettext(NULL, v10, 5);
            __printf_chk(1, "  %s\n");
            v10 = &*(iter)->field_0;
            if (!*(iter))
                break;
        }
        else
        {
            iter += 1;
            putchar_unlocked(10);
            v10 = &*(iter)->field_0;
            if (!*(iter))
                break;
        }
        v9 = *(v10);
    }
    v11 = dcgettext(NULL, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
    __printf_chk(1, "\n%s\n%s\n", dcgettext(NULL, "If a FILE is '-', read standard input.", 5), v11);
    fputs_unlocked("\n", stdout);
    __printf_chk(1, dcgettext(NULL, "Report bugs to: %s\n", 5));
    __printf_chk(1, dcgettext(NULL, "%s home page: <%s>\n", 5));
    return __printf_chk(1, dcgettext(NULL, "General help using GNU software: <%s>\n", 5));
}



// Function: try_help @ 0x44c0
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: diraccess @ 0x4520
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

unsigned long long diraccess(char *a0)
{
    stat v0;  // [bp-0xa8]

    if (stat(a0, &v0))
        return 0;
    return _INSERT(v0.st_nlink & 0xf000, 0, (v0.st_nlink & 0xf000) == 0x4000);
}



// Function: lf_init @ 0x4580
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_1 *field_8;
    struct struct_1 *field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

void lf_init(struct_0 *idx, unsigned long a1)
{
    char *v1;  // rax

    idx->field_0 = a1;
    v1 = sub_407800(65537);
    idx->field_18 = v1;
    idx->field_10 = v1;
    idx->field_8 = v1;
    *(v1) = 10;
    return;
}



// Function: diffarg @ 0x45b0
extern unsigned long long g_40f0a0;
extern unsigned long long g_40f0a8;
extern unsigned long long g_40f5c0;

long long diffarg(unsigned long a0)
{
    unsigned long long idx;  // rsi
    unsigned long v2;  // rax
    unsigned long long v3;  // rax

    idx = g_40f0a8;
    v2 = g_40f5c0;
    if (g_40f0a8 == g_40f0a0)
    {
        if (!g_40f0a8)
        {
            v3 = 16;
        }
        else if (g_40f0a8 <= 0x7fffffffffffffe)
        {
            v3 = g_40f0a8 * 2;
        }
        else
        {
            sub_407bb0(); /* do not return */
        }
        g_40f0a0 = v3;
        v2 = sub_407850(g_40f5c0);
        idx = g_40f0a8;
        g_40f5c0 = v2;
    }
    *((unsigned long *)(v2 + idx * 8)) = a0;
    g_40f0a8 = idx + 1;
    return v2;
}



// Function: signal_handler @ 0x4640
typedef struct sigaction {
} sigaction;

extern sigaction g_40f0c0;

int signal_handler(int a0, unsigned long a1)
{
    *((unsigned long *)&g_40f0c0) = a1;
    return sigaction(a0, &g_40f0c0, NULL);
}



// Function: untrapsig @ 0x4660
extern int g_40ad30;
extern char g_40f158;
extern unsigned long long g_40f180;
extern unsigned long long g_40f5a8;

int untrapsig(unsigned int a0)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    int *v6;  // rbp
    unsigned long long v7;  // rbx
    unsigned long long *v8;  // rbx
    int v9;  // eax
    unsigned long long *v10;  // rbx
    int *v11;  // rbp
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    if (!g_40f158)
        return v9;
    v2 = v4;
    v1 = v5;
    v6 = &g_40ad30;
    v0 = v7;
    v8 = &g_40f180;
    while (1)
    {
        if (a0 && *(v6) != a0 || *(v8) == 1)
        {
            v8 += 19;
            v6 += 1;
            if (v8 == &g_40f5a8)
                return v9;
        }
        else
        {
            v10 = v8 + 19;
            v11 = v6 + 1;
            v9 = sigaction(*(v6), v8, NULL);
            v6 = v11;
            v8 = v10;
            if (v8 == &g_40f5a8)
                return v9;
        }
    }
}



// Function: trapsigs @ 0x46f0
typedef struct sigaction {
} sigaction;

extern int g_40ad34;
extern int g_40f0c8;
extern unsigned int g_40f148;
extern char g_40f158;
extern sigaction g_40f180;
extern sigaction g_40f5a8;

sighandler_t trapsigs(void)
{
    int *iter;  // rbp
    int *node;  // rbx
    int v3;  // esi
    sigaction *v4;  // rbx
    int v5;  // r12d
    sighandler_t v6;  // rax

    iter = &g_40ad34;
    node = &g_40ad34;
    g_40f148 = 0x10000000;
    sigemptyset(&g_40f0c8);
    v3 = 1;
    while (1)
    {
        sigaddset(&g_40f0c8, v3);
        if (node == "3.8")
            break;
        v3 = *(node);
        node += 1;
    }
    v4 = &g_40f180;
    v5 = 1;
    while (1)
    {
        sigaction(v5, NULL, v4);
        if (*((long long *)&v4) != 1)
            sub_404640(v5, sub_4042e0);
        v4 += 152;
        if (v4 == &g_40f5a8)
            break;
        v5 = *(iter);
        iter += 1;
    }
    v6 = signal(0x11, NULL);
    g_40f158 = 1;
    return v6;
}



// Function: temporary_file @ 0x47b0
extern unsigned long long g_40f5b8;

int temporary_file(void)
{
    char *v1;  // rax
    char *v2;  // rax
    unsigned long v3;  // rbp
    unsigned int v4;  // eax

    v1 = getenv("TMPDIR");
    v2 = "/tmp";
    if (!v1)
        v1 = v2;
    v3 = sub_407800(strlen(v1) + 13);
    __sprintf_chk(v3, 0x1);
    v4 = mkstemp(v3);
    if (v4 >= 0)
        g_40f5b8 = v3;
    return v4;
}



// Function: give_help @ 0x4830
extern unsigned long long stderr;

int give_help(void)
{
    char *v1;  // rax

    v1 = dcgettext(NULL, "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n", 5);
    return __fprintf_chk(stderr, 0x1, "%s", v1);
}



// Function: checksigs @ 0x4870
extern void g_40f15c;

void checksigs(void)
{
    if (*((int *)&g_40f15c))
    {
        sub_404300();
        sub_404660(*((int *)&g_40f15c));
        raise(*((int *)&g_40f15c));
        exit(2); /* do not return */
    }
    return;
}



// Function: exiterr @ 0x48a0
void exiterr(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_404300();
    sub_404660(0);
    sub_404870();
    exit(2); /* do not return */
}



// Function: fatal @ 0x48d0
void fatal(char *a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    dcgettext(NULL, a0, 5);
    error(0, 0, "%s");
    sub_4048a0(); /* do not return */
}



// Function: expand_name @ 0x4900
long long expand_name(unsigned long ptr, char a1, long long a2)
{
    unsigned long long v2;  // r15
    unsigned long v3;  // r13
    unsigned long len;  // rbx
    unsigned long n;  // r14
    unsigned long long v6;  // r15
    unsigned long v7;  // r12
    long long v0;  // [bp-0x40]

    if (!strcmp(ptr, "-"))
    {
        sub_4048d0("cannot interactively merge standard input"); /* do not return */
    }
    else if (a1)
    {
        v2 = 0;
        v3 = sub_405690(a2);
        len = strlen(ptr);
        n = sub_4056f0(v3);
        if (*((char *)sub_405690(ptr)))
            v2 = *((char *)(ptr + len - 1)) != 47;
        v6 = v2 + len;
        v0 = v6 + n;
        v7 = sub_407800(v0 + 1);
        memcpy(v7, ptr, len);
        *((char *)(v7 + len)) = 47;
        memcpy(v7 + v6, v3, n);
        *((char *)(v7 + v0)) = 0;
        return v7;
    }
    else
    {
        return ptr;
    }
}



// Function: perror_fatal @ 0x49e0
void perror_fatal(unsigned long a0)
{
    unsigned long long v2;  // r12
    int *err;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    err = __errno_location();
    sub_404870();
    error(0, *(err), "%s");
    sub_4048a0(); /* do not return */
}



// Function: check_stdout @ 0x4a10
typedef struct FILE {
} FILE;

extern FILE *stdout;

int check_stdout(void)
{
    unsigned int v1;  // eax

    if (ferror_unlocked(stdout))
        sub_4048d0("write failed"); /* do not return */
    v1 = fclose(stdout);
    if (v1)
        sub_4049e0(dcgettext(NULL, "standard output", 5)); /* do not return */
    return v1;
}



// Function: ck_fopen @ 0x4a60
typedef struct FILE {
} FILE;

FILE * ck_fopen(char *a0, char *a1)
{
    FILE *fp;  // rax

    fp = fopen(a0, a1);
    if (!fp)
        sub_4049e0(a0); /* do not return */
    return fp;
}



// Function: ck_fwrite @ 0x4a80
typedef struct FILE {
} FILE;

long long ck_fwrite(void* a0, unsigned long a1, FILE *a2)
{
    unsigned long count;  // rax

    count = fwrite_unlocked(a0, 1, a1, a2);
    if (count != a1)
        sub_4049e0(dcgettext(NULL, "write failed", 5)); /* do not return */
    return count;
}



// Function: ck_fflush @ 0x4ac0
typedef struct FILE {
} FILE;

int ck_fflush(FILE *a0)
{
    unsigned int v1;  // eax

    v1 = fflush_unlocked(a0);
    if (v1)
        sub_4049e0(dcgettext(NULL, "write failed", 5)); /* do not return */
    return v1;
}



// Function: flush_line @ 0x4af0
typedef struct FILE {
} FILE;

extern FILE *stdin;

int flush_line(void)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax

    do
    {
        v1 = getchar_unlocked();
    } while (v1 != 0xffffffff && v1 != 10);
    v2 = ferror_unlocked(stdin);
    if (v2)
        sub_4049e0(dcgettext(NULL, "read failed", 5)); /* do not return */
    return v2;
}



// Function: ck_fclose @ 0x4b40
typedef struct FILE {
} FILE;

int ck_fclose(FILE *a0)
{
    unsigned int v1;  // eax

    v1 = fclose(a0);
    if (v1)
        sub_4049e0("fclose"); /* do not return */
    return v1;
}



// Function: skip_white @ 0x4b60
typedef struct FILE {
} FILE;

extern FILE *stdin;

int skip_white(void)
{
    unsigned long long *v1;  // rax
    unsigned long v2;  // rdx

    v1 = __ctype_b_loc();
    while (1)
    {
        v2 = (int)getchar_unlocked();
        if (!(*((char *)(*(v1) + v2 * 2 + 1)) & 32) || (unsigned int)v2 == 10)
            break;
        sub_404870();
    }
    if (ferror_unlocked(stdin))
        sub_4049e0(dcgettext(NULL, "read failed", 5)); /* do not return */
    return v2;
}



// Function: lf_refill @ 0x4c30
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

unsigned long long lf_refill(struct_0 *idx)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // rdx
    char *v3;  // rdx

    v1 = sub_404bd0(idx->field_10, idx->field_0);
    v2 = idx->field_10;
    idx->field_8 = v2;
    v3 = v2 + v1;
    idx->field_18 = v3;
    *(v3) = 10;
    sub_404870();
    return v1;
}



// Function: lf_skip @ 0x4c70
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

void* lf_skip(struct_0 *idx, unsigned long long a1)
{
    unsigned long long v1;  // rbp
    void* ptr;  // rax
    void* v3;  // rax

    v1 = a1;
    if (!a1)
        return v3;
    while (1)
    {
        ptr = rawmemchr(idx->field_8, 10);
        idx->field_8 = ptr;
        if (ptr != idx->field_18)
        {
            idx->field_8 = ptr + 1;
            v1 -= 1;
            if (v1 == 1)
                return ptr + 1;
        }
        else if (!sub_404c30(idx))
        {
            return NULL;
        }
    }
}



// Function: lf_copy @ 0x4cd0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

long long lf_copy(struct_0 *idx, unsigned long long a1, FILE *a2)
{
    unsigned long v1;  // rdi
    unsigned long v2;  // r12
    unsigned long long v3;  // rbx
    unsigned long v4;  // r13
    unsigned long ptr;  // rax
    unsigned long long v6;  // rsi

    v1 = idx->field_8;
    if (a1)
    {
        v2 = idx->field_18;
        v3 = a1;
        v4 = v1;
        while (1)
        {
            ptr = rawmemchr(v1, 10);
            idx->field_8 = ptr;
            if (ptr != v2)
            {
                v1 = ptr + 1;
                idx->field_8 = v1;
                v3 -= 1;
                if (v3 != 1)
                    continue;
                v6 = v1 - v4;
                break;
            }
            else
            {
                sub_404a80(v4, v2 - v4, a2);
                if (!sub_404c30(idx))
                    return 0;
                v4 = idx->field_8;
                v2 = idx->field_18;
                v1 = v4;
            }
        }
    }
    else
    {
        v4 = v1;
        v6 = 0;
    }
    return sub_404a80(v4, v6, a2);
}



// Function: edit @ 0x4ec0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_40f010;
extern FILE *stdout;
extern FILE *stdin;
extern char g_40f160;
extern char g_40f161;
extern FILE *g_40f5b0;
extern unsigned long long g_40f5b8;

int edit(void)
{
    unsigned long v13;  // fs
    struct_0 *v20;  // rdi
    unsigned long long v21;  // rcx
    struct_0 *v22;  // r8
    char *v23;  // rsi
    int v24;  // rdx
    unsigned int v14;  // ebx
    char *v25;  // r9
    unsigned int v15;  // eax
    unsigned int v26;  // eax
    unsigned long long v27;  // rax
    unsigned int *err;  // rax
    unsigned long v16;  // rax
    FILE *fp;  // rax
    unsigned int v19;  // eax
    unsigned long v0;  // [bp-0x101c8]
    unsigned int v1;  // [bp-0x1009c]
    int v2;  // [bp-0x1006c], Other Possible Types: unsigned int
    unsigned long v3;  // [bp-0x10068]
    unsigned long long v4;  // [bp-0x10060]
    unsigned long long v5;  // [bp-0x10058]
    char v6;  // [bp-0x10048]
    char v7;  // [bp-0x10030]
    unsigned long long v8;  // [bp-0x2030]
    unsigned long v9;  // [bp-0x40]
    int v10;  // [bp+0x8]
    unsigned long long v11;  // [bp+0x10]
    FILE *v12;  // [bp+0x18]

    do
    {
    } while (&v8 != &v7);
    v9 = *((long long *)(40 + v13));
LABEL_404f30:
    if (putchar_unlocked(37) == 37)
    {
        while (1)
        {
            sub_404ac0(stdout);
            v15 = sub_404b60();
            if (v15 > 50)
            {
                if (v15 == 101)
                {
                    v14 = sub_404b60();
                    if (v14 <= 50)
                    {
                        if (v14 <= 48)
                        {
                            if (v14 == 10)
                                goto LABEL_405092;
                            goto LABEL_404fd5;
                        }
                    }
                    else
                    {
                        v16 = v14 - 98;
                        if ((unsigned int)v16 > 16 || (v0 = 66565, !(((char)v16 & 254 | *((char *)((char *)&v0 + ((long long)(v16 & 63) >> 3))) >> (unsigned long long)((char)v16 & 63 & 7) & 1) & 1)))
                            goto LABEL_404fd5;
                    }
                    if (sub_404b60() == 10)
                        goto LABEL_405092;
LABEL_404fd5:
                    sub_404830();
                    sub_404af0();
                    break;
                }
                if (v15 <= 100)
                {
LABEL_404fae:
                    sub_404af0();
                    goto LABEL_404f6e;
                }
                else
                {
                    if (v15 <= 115)
                    {
                        if (v15 <= 112 && v15 == 108)
                        {
                            if (sub_404b60() != 10)
                                goto LABEL_404fd5;
                            goto LABEL_4052a0;
                        }
                    }
                    else
                    {
                        if (v15 == 118)
                            goto LABEL_404fed;
                        goto LABEL_404fae;
                    }
                }
            }
            else if (v15 <= 48)
            {
                if (v15 != 0xffffffff)
                    if (v15 == 10)
                        goto LABEL_404f6e;
                    else
                        goto LABEL_404fae;
                else
                    if (!feof_unlocked(stdin))
        goto LABEL_404fae;
    else
        goto LABEL_405044;
                if (v15 != 10)
                    goto LABEL_404fae;
                goto LABEL_404f6e;
            }
LABEL_404fed:
            v1 = v15;
            if (sub_404b60() != 10)
                goto LABEL_404fd5;
            if (v1 > 118)
                goto LABEL_404f6e;
            if (v1 > 100)
            {
                switch (v1)
                {
                case 101:
LABEL_405092:
                    if (g_40f5b8)
                    {
                        fp = fopen(g_40f5b8, "w");
                    }
                    else
                    {
                        v19 = sub_4047b0();
                        if (v19 < 0)
                            sub_4049e0("mkstemp"); /* do not return */
                        fp = fdopen(v19, "w");
                    }
                    g_40f5b0 = fp;
                    if (!fp)
                        sub_4049e0(g_40f5b8); /* do not return */
                    switch (v14)
                    {
                    case 100:
                        if (v21)
                        {
                            if (v21 != 1)
                            {
                                __fprintf_chk(*((unsigned int *)&fp), 0x1, "--- %s %ld,%ld\n", v23, v24, v24 + v21 - 1);
                                fp = g_40f5b0;
                            }
                            else
                            {
                                __fprintf_chk(*((unsigned int *)&fp), 0x1, "--- %s %ld\n", v23, v24);
                                fp = g_40f5b0;
                            }
                        }
                        sub_404cd0(v20, v21, fp);
                        if (v11)
                        {
                            if (v11 != 1)
                            {
                                __fprintf_chk(*((unsigned int *)&g_40f5b0), 0x1, "+++ %s %ld,%ld\n", v25, v10, v10 + v11 - 1);
                                break;
                            }
                            else
                            {
                                __fprintf_chk(*((unsigned int *)&g_40f5b0), 0x1, "+++ %s %ld\n", v25, v10);
                                break;
                            }
                        }
LABEL_405109:
                        sub_404cd0(v22, v11, g_40f5b0);
                        goto LABEL_405120;
                    case 49: case 98: case 108:
                        sub_404cd0(v20, v21, fp);
                    default:
                        sub_404c70(v20, v21);
                        goto LABEL_4050f2;
                    }
LABEL_4050f2:
                    if (v14 <= 100)
                    {
                        if (v14 == 50 || v14 == 98)
                            goto LABEL_405109;
                    }
                    else
                    {
                        if (v14 == 114)
                            goto LABEL_405109;
                    }
                    sub_404c70(v22, v11);
LABEL_405120:
                    sub_404b40(g_40f5b0);
                    g_40f160 = 1;
                    sub_404870();
                    v5 = 0;
                    v3 = g_40f010;
                    v4 = g_40f5b8;
                    v26 = fork();
                    if (!v26)
                    {
                        execvp(g_40f010, &v3);
                        err = __errno_location();
                        _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
                    }
                    else if (v26 >= 0)
                    {
                        while (waitpid(v26, &v2, 0) < 0)
                        {
                            if (*(__errno_location()) == 4)
                                sub_404870();
                            else
                                sub_4049e0("waitpid"); /* do not return */
                        }
                        g_40f160 = 0;
                        sub_404e40(v2, 0, g_40f010);
                        g_40f5b0 = sub_404a60(g_40f5b8, "r");
                        while (1)
                        {
                            v27 = sub_404bd0(&v6);
                            if (!v27)
                                break;
                            sub_404870();
                            sub_404a80(&v6, v27, v12);
                        }
                        sub_404b40(g_40f5b0);
                        break;
                    }
                    else
                    {
                        sub_4049e0("fork"); /* do not return */
                    }
                case 108:
LABEL_4052a0:
                    sub_404cd0(v20, v21, v12);
                    sub_404c70(v22, v11);
                    if (v9 != *((long long *)(40 + v13)))
                        __stack_chk_fail(); /* do not return */
                    return;
                case 113:
LABEL_405044:
                    break;
                case 114:
LABEL_40525e:
                    sub_404cd0(v22, v11, v12);
                    sub_404c70(v20, v21);
                    break;
                case 115:
                    g_40f161 = 1;
                    goto LABEL_404f30;
                case 118:
                    g_40f161 = 0;
                    goto LABEL_404f30;
                default:
LABEL_404f6e:
                    sub_404830();
                    if (putchar_unlocked(37) != 37)
                        sub_4049e0(dcgettext(NULL, "write failed", 5)); /* do not return */
                    continue;
                }
            }
            else
            {
                if (v1 == 49)
                    goto LABEL_4052a0;
                if (v1 == 50)
                    goto LABEL_40525e;
                else
                    goto LABEL_404f6e;
            }
        }
    }
    sub_4049e0(dcgettext(NULL, "write failed", 5)); /* do not return */
}



// Function: interact @ 0x5480
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_40f161;

char interact(unsigned long a0, unsigned long a1, long long a2, unsigned long a3, long long a4, unsigned long a5)
{
    unsigned int v6;  // eax
    unsigned int *err;  // rax
    unsigned int *v8;  // rbx
    unsigned long v9;  // r12
    unsigned long v10;  // r13
    unsigned long v11;  // rsi
    char v12;  // al
    int v13;  // eax
    long long v0;  // [bp-0x190]
    long long v1;  // [bp-0x188]
    unsigned long v2;  // [bp-0x150]
    char v3;  // [bp-0x148]
    char v4[263];  // [bp-0x147]

    v1 = 1;
    v0 = 1;
    while (1)
    {
        if (sub_404d80(a0, &v3) <= 0)
            break;
        while (1)
        {
            sub_404870();
            if (v3 == 32)
                break;
            err = __errno_location();
            *(err) = 0;
            v8 = err;
            v9 = strtoimax(v4, &v2, 10);
            if (v9 < 0)
            {
                sub_4048d0(&v3); /* do not return */
            }
            else if (*(v8))
            {
                sub_4048d0(&v3); /* do not return */
            }
            else if (*((char *)v2) == 44)
            {
                v10 = strtoimax(v2 + 1, &v2, 10);
                if (v10 < 0)
                {
                    sub_4048d0(&v3); /* do not return */
                }
                else if (*(v8))
                {
                    sub_4048d0(&v3); /* do not return */
                }
                else if (!*((char *)v2))
                {
                    if (v10 <= v9)
                        v11 = v9;
                    else
                        v11 = v10;
                    switch (v3)
                    {
                    case 99:
                        sub_404cd0(a0, v11, stdout);
                        v12 = sub_404ec0(a1, a2, v0, v9, a3, a4, v1, v10, a5);
                        if (!v12)
                            return v12;
                        break;
                    case 105:
                        if (g_40f161)
                            sub_404c70(a0, v11);
                        else
                            sub_404cd0(a0, v11, stdout);
                        sub_404cd0(a1, v9, a5);
                        sub_404c70(a3, v10);
                        break;
                    default:
                        sub_4048d0(&v3); /* do not return */
                    }
                    v0 += v9;
                    v1 += v10;
                    v13 = sub_404d80(a0, &v3);
                    if (v13 <= 0)
                        return v13;
                }
                else
                {
                    sub_4048d0(&v3); /* do not return */
                }
            }
            else
            {
                sub_4048d0(&v3); /* do not return */
            }
        }
        puts(v4);
    }
    return v6;
}



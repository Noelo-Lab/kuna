// Function: main @ 0x3b00
typedef struct sigaction {
} sigaction;

typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_40ba59;
extern int g_40bd30;
extern struct struct_0 *g_40f7a0[2];
extern option g_40f880;
extern unsigned long long g_410010;
extern long long g_410018;
extern unsigned int g_410020;
extern struct_1 *stdout;
extern int optind;
extern unsigned long long optarg;
extern sigaction g_4100c0;
extern int g_4100c8;
extern unsigned int g_410148;
extern char g_410158;
extern unsigned int g_41015c;
extern char g_410161;
extern unsigned long long g_410168;
extern sigaction g_410180;
extern unsigned long long g_410510;
extern sigaction g_4105a8;
extern char *g_4105b8;
extern void* g_4105c0;

unsigned int main(int a0, long long *a1)
{
    unsigned long v28;  // rbp
    long long v29;  // rdi
    struct_0 **iter;  // rbx
    char *v41;  // rax
    char *v42;  // rsi
    char *v43;  // rax
    char *v44;  // rax
    long long v46;  // rax
    unsigned int v47;  // edx
    long long v30;  // rdi
    char v48;  // bpl
    char v49;  // al
    int *node;  // rbx
    int *i;  // r15
    int *v52;  // r15
    sigaction *v53;  // r14
    unsigned int v54;  // r15d
    char *v55;  // rax
    char *ptr;  // r15
    char *v57;  // rax
    long long v31;  // rsi
    char *v58;  // rax
    char *iter1;  // rbx
    unsigned long v60;  // rbp
    char *ptr1;  // r13
    unsigned long n;  // r14
    unsigned long long v63;  // rax
    unsigned int v64;  // r9d
    unsigned int *err;  // rax
    unsigned int *v66;  // r14
    long long v67;  // rbx
    long long v32;  // rdx
    long long v68;  // r15
    char v69;  // al
    char v70;  // bl
    unsigned int *err1;  // rax
    long long v33;  // rcx
    long long v34;  // r8
    long long v35;  // r9
    char *v36;  // rax
    unsigned int v37;  // eax
    char *v0;  // [bp-0x218], Other Possible Types: unsigned long
    long long v1;  // [bp-0x210]
    long long v2;  // [bp-0x208]
    long long v3;  // [bp-0x200]
    char v4;  // [bp-0x1f1]
    long long v5;  // [bp-0x1f0]
    long long v6;  // [bp-0x1e8]
    unsigned long v7;  // [bp-0x1e0]
    unsigned long v8;  // [bp-0x1d8]
    unsigned long fp;  // [bp-0x1d0]
    char *v10;  // [bp-0x1c0]
    unsigned long v11;  // [bp-0x1b8]
    char *v12;  // [bp-0x1b0]
    char *v13;  // [bp-0x1a8]
    char *v14;  // [bp-0x1a0]
    unsigned long v15;  // [bp-0x198]
    char *v16;  // [bp-0x190]
    char *v17;  // [bp-0x188]
    char *v18;  // [bp-0x180]
    unsigned long v19;  // [bp-0x178]
    char *v20;  // [bp-0x170]
    char *v21;  // [bp-0x168]
    char *v22;  // [bp-0x160]
    int v23;  // [bp-0x150]
    int v24;  // [bp-0x14c]
    char v25;  // [bp-0x148]
    char v26[263];  // [bp-0x147]

    v28 = a0;
    v29 = *(a1);
    g_410020 = 2;
    sub_405b80(v29);
    setlocale(6, &g_40ba59);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_405ac0(sub_404990);
    sub_4082b0(v30, v31, v32, v33, v34, v35);
    v36 = getenv("EDITOR");
    if (v36)
        g_410010 = v36;
    sub_404900("diff");
    while (1)
    {
        v37 = getopt_long(v28, a1, "abBdEHiI:lo:stvw:WZ", &g_40f880.name, NULL);
        if (v37 == 0xffffffff)
            break;
        switch (v37)
        {
        case 66:
            sub_404900("-B");
            break;
        case 69:
            sub_404900("-E");
            break;
        case 72:
            sub_404900("-H");
            break;
        case 73:
            sub_404900("-I");
            sub_404900(optarg);
            break;
        case 87:
            sub_404900("-w");
            break;
        case 90:
            sub_404900("-Z");
            break;
        case 97:
            sub_404900("-a");
            break;
        case 98:
            sub_404900("-b");
            break;
        case 100:
            sub_404900("-d");
            break;
        case 105:
            sub_404900("-i");
            break;
        case 108:
            sub_404900("--left-column");
            break;
        case 111:
            g_410168 = optarg;
            break;
        case 115:
            g_410161 = 1;
            break;
        case 116:
            sub_404900("-t");
            break;
        case 118:
            v46 = sub_406020("Thomas Lord");
            sub_407bf0(stdout, "sdiff", "GNU diffutils", g_410018, v46, 0);
            sub_404cc0(v30, v31, v32, v33, v34, v35);
            return 0;
        case 119:
            sub_404900("-W");
            sub_404900(optarg);
            break;
        case 128:
            *((unsigned long long *)g_4105c0) = optarg;
            break;
        case 129:
            iter = &g_40f7a0[0];
            __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... FILE1 FILE2\n", 5));
            __printf_chk(1, "%s\n\n", dcgettext(NULL, "Side-by-side merge of differences between FILE1 and FILE2.", 5));
            v41 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
            fputs_unlocked(v41, stdout);
            v42 = "-o, --output=FILE            operate interactively, sending output to FILE";
            while (1)
            {
                if (!*(v42))
                {
                    v43 = &stdout->field_28->field_0;
                    if (v43 < *((long long *)&stdout[1].padding_0[0]))
                    {
                        stdout->field_28 = v43 + 1;
                        *(v43) = 10;
                    }
                    else
                    {
                        __overflow(stdout, 10);
                    }
                    v42 = &iter[1]->field_0;
                    iter += 1;
                    if (!v42)
                        break;
                }
                else
                {
                    iter += 1;
                    dcgettext(NULL, v42, 5);
                    __printf_chk(1, "  %s\n");
                    v42 = &*(iter)->field_0;
                    if (!v42)
                        break;
                }
            }
            v44 = dcgettext(NULL, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
            __printf_chk(1, "\n%s\n%s\n", dcgettext(NULL, "If a FILE is '-', read standard input.", 5), v44);
            sub_407cd0(v30, v31, v32, v33, v34, v35);
            sub_404cc0(v30, v31, v32, v33, v34, v35);
            return 0;
        case 130:
            sub_404900("--strip-trailing-cr");
            break;
        case 131:
            sub_404900("--tabsize");
            sub_404900(optarg);
            break;
        default:
            sub_404840(0, 0); /* do not return */
        }
    }
    v47 = (v28 & 0xffffffff) - optind;
    if (v47 != 2)
    {
        if (v47 > 1)
            sub_404840("extra operand '%s'", a1[2 + optind]); /* do not return */
        sub_404840("missing operand after '%s'", *((long long *)((char *)&a1[v28] - 8))); /* do not return */
    }
    if (g_410168)
    {
        v48 = sub_4048a0(a1[optind]);
        v49 = sub_4048a0(a1[1 + optind]);
        v4 = v48 & v49;
        if (v48 & v49)
            sub_404b00("both files to be compared are directories"); /* do not return */
        v5 = sub_404be0(a1[optind]);
        v7 = sub_404b60(v5, "r");
        v6 = sub_404be0(a1[1 + optind], v49, a1[optind]);
        v8 = sub_404b60(v6, "r");
        v3 = sub_404b60(g_410168, "w");
        sub_404900("--sdiff-merge-assist");
        sub_404900("--");
        sub_404900(a1[optind]);
        node = &g_40bd30;
        i = &g_40bd30;
        sub_404900(a1[1 + optind]);
        sub_404900(0);
        g_410148 = 0x10000000;
        sigemptyset(&g_4100c8);
        do
        {
            v52 = i + 1;
            sigaddset(&g_4100c8, *(i));
            i = v52;
        } while (i != "3.8");
        v53 = &g_410180;
        do
        {
            v54 = *(node);
            sigaction(v54, NULL, v53);
            if (*((long long *)&v53) != 1)
            {
                *((void* *)&g_4100c0) = sub_404820;
                sigaction(v54, &g_4100c0, NULL);
            }
        } while ((v53 += 152, node += 4, v53 != &g_4105a8));
        signal(0x11, NULL);
        g_410158 = 1;
        if (pipe(&v23))
            sub_404b30("pipe"); /* do not return */
        *((int *)&g_4105a8) = fork();
        if (*((int *)&g_4105a8) < 0)
        {
            sub_404b30("fork"); /* do not return */
        }
        else if (*((int *)&g_4105a8))
        {
            close(v24);
            fp = fdopen(v23, "r");
            if (!fp)
                sub_404b30("fdopen"); /* do not return */
            v19 = fp;
            v55 = sub_407df0(65537);
            *(v55) = 10;
            ptr = v55;
            v22 = v55;
            v21 = v55;
            v20 = v55;
            v11 = v7;
            v57 = sub_407df0(65537);
            *(v57) = 10;
            v14 = v57;
            v13 = v57;
            v12 = v57;
            v15 = v8;
            v58 = sub_407df0(65537);
            v2 = 1;
            v18 = v58;
            v17 = v58;
            v16 = v58;
            *(v58) = 10;
            v1 = 1;
            v0 = &v25;
            while (1)
            {
                iter1 = v0;
                v60 = 0x100;
                while (1)
                {
                    ptr1 = rawmemchr(ptr, 10);
                    n = ptr1 - ptr;
                    if (n < v60)
                    {
                        memcpy(iter1, ptr, n);
                        if (ptr1 < v22)
                        {
                            ptr = ptr1 + 1;
                            iter1[n] = 0;
                            v20 = ptr;
                            if (g_41015c)
                            {
                                if (*((int *)&g_4105a8) > 0)
                                    kill(*((int *)&g_4105a8), 13);
                                if (g_4105b8)
                                    unlink(g_4105b8);
                                if (g_410158)
                                    sub_404a10(g_41015c);
                                raise(g_41015c);
                                exit(2); /* do not return */
                            }
                            if (v25 != 32)
                            {
                                err = __errno_location();
                                *(err) = 0;
                                v66 = err;
                                v67 = strtoimax(v26, &v10, 10);
                                if (!(v67 >= 0) || !(!*(v66) && *(v10) == 44 && (v68 = (long long)(unsigned long long)strtoimax(v10 + 1, &v10, 10), v68 >= 0 && !*(v66) && !*(v10))))
                                    goto LABEL_40457d;
                                switch (v25)
                                {
                                case 99:
                                    sub_405000(&v19);
                                    v69 = sub_405120(&v11, v5, v1, v67, &v15, v6, v2, v68, v3);
                                    if (!v69)
                                    {
                                        v70 = v69;
LABEL_40441a:
                                        sub_404bc0(v7);
                                        sub_404bc0(v8);
                                        sub_404bc0(v3);
                                        sub_404bc0(fp);
                                        while (waitpid(*((int *)&g_4105a8), &v10, 0) < 0)
                                        {
                                            if (*(__errno_location()) == 4)
                                                sub_404a90();
                                            else
                                                sub_404b30("waitpid"); /* do not return */
                                        }
                                        *((unsigned int *)&g_4105a8) = 0;
                                        if (g_4105b8)
                                        {
                                            unlink(g_4105b8);
                                            g_4105b8 = 0;
                                        }
                                        if (!v70)
                                            sub_404ad0(); /* do not return */
                                        sub_404dc0(*((unsigned int *)&v10), 1, *((long long *)g_4105c0));
                                        if (g_410158)
                                            sub_404a10(0);
                                        sub_404a90();
                                        exit((char)*((unsigned int *)&v10)); /* do not return */
                                    }
                                    break;
                                case 105:
                                    if (g_410161)
                                        sub_404f30(&v19);
                                    else
                                        sub_405000(&v19);
                                    sub_405000(&v11, v67, v3);
                                    sub_404f30(&v15, v68);
                                    break;
                                default:
LABEL_40457d:
                                    sub_404b00(v0); /* do not return */
                                }
                                v2 += v68;
                                ptr = v20;
                                v1 += v67;
                                break;
                            }
                            else
                            {
                                puts(v26);
                                break;
                            }
                        }
                        ptr = v21;
                        v63 = sub_404d80(ptr, v19);
                        v64 = g_41015c;
                        v20 = ptr;
                        v22 = &ptr[v63];
                        *(v22) = 10;
                        if (g_41015c)
                        {
                            if (*((int *)&g_4105a8) > 0)
                            {
                                *((unsigned int *)&v0) = g_41015c;
                                kill(*((int *)&g_4105a8), 13);
                                v64 = *((unsigned int *)&v0);
                            }
                            if (g_4105b8)
                            {
                                *((unsigned int *)&v0) = v64;
                                unlink(g_4105b8);
                                v64 = *((unsigned int *)&v0);
                            }
                            if (g_410158)
                            {
                                *((unsigned int *)&v0) = v64;
                                sub_404a10(v64);
                                v64 = v0;
                            }
                            raise(v64);
                            exit(2); /* do not return */
                        }
                        if (v63)
                        {
                            iter1 = &iter1[n];
                            v60 -= n;
                        }
                        else
                        {
                            v4 = !n;
                            goto LABEL_404410;
                        }
                    }
                    else
                    {
LABEL_404410:
                        v70 = v4;
                        goto LABEL_40441a;
                    }
                }
            }
        }
        else
        {
            if (g_410510 != 1)
            {
                *((unsigned long *)&g_4100c0) = 1;
                sigaction(2, &g_4100c0, NULL);
            }
            *((unsigned long *)&g_4100c0) = 0;
            sigaction(13, &g_4100c0, NULL);
            close(v23);
            if (v24 != 1)
            {
                dup2(v24, 1);
                close(v24);
            }
            execvp(*((long long *)g_4105c0), g_4105c0);
            err1 = __errno_location();
            _exit((unsigned int)((*(err1) == 2) + 126)); /* do not return */
        }
    }
    else
    {
        if (g_410161)
            sub_404900("--suppress-common-lines");
        sub_404900("-y");
        sub_404900("--");
        sub_404900(a1[optind]);
        sub_404900(a1[1 + optind]);
        sub_404900(0);
        execvp(*((long long *)g_4105c0), g_4105c0);
        sub_404b30(*((long long *)g_4105c0)); /* do not return */
    }
}



// Function: catchsig @ 0x4820
extern unsigned int g_41015c;
extern char g_410160;

long long catchsig(unsigned int a0)
{
    unsigned long v1;  // rax

    if (a0 == 2)
    {
        v1 = g_410160;
        if (g_410160)
            return g_410160;
    }
    g_41015c = a0;
    return v1;
}



// Function: try_help @ 0x4840
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: diraccess @ 0x48a0
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



// Function: diffarg @ 0x4900
extern unsigned long long g_4100a0;
extern unsigned long long g_4100a8;
extern unsigned long long g_4105c0;

long long diffarg(unsigned long a0)
{
    unsigned long long idx;  // rsi
    unsigned long v2;  // rax
    unsigned long long v3;  // rax

    idx = g_4100a8;
    v2 = g_4105c0;
    if (g_4100a8 == g_4100a0)
    {
        if (!g_4100a8)
        {
            v3 = 16;
        }
        else if (g_4100a8 <= 0x7fffffffffffffe)
        {
            v3 = g_4100a8 * 2;
        }
        else
        {
            sub_408270(); /* do not return */
        }
        g_4100a0 = v3;
        v2 = sub_407e40(g_4105c0);
        idx = g_4100a8;
        g_4105c0 = v2;
    }
    *((unsigned long *)(v2 + idx * 8)) = a0;
    g_4100a8 = idx + 1;
    return v2;
}



// Function: exiterr @ 0x4ad0
extern char g_410158;

void exiterr(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_404990(0);
    if (g_410158)
        sub_404a10(0);
    sub_404a90();
    exit(2); /* do not return */
}



// Function: fatal @ 0x4b00
void fatal(char *a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    dcgettext(NULL, a0, 5);
    error(0, 0, "%s");
    sub_404ad0(); /* do not return */
}



// Function: perror_fatal @ 0x4b30
void perror_fatal(unsigned long a0)
{
    unsigned long long v2;  // r12
    int *err;  // rax
    long long v4;  // rdi
    long long v5;  // rsi
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    err = __errno_location();
    sub_404a90(v4, v5, v6, v7, v8, v9);
    error(0, *(err), "%s");
    sub_404ad0(); /* do not return */
}



// Function: ck_fwrite @ 0x4b80
typedef struct FILE {
} FILE;

long long ck_fwrite(void* a0, unsigned long a1, FILE *a2)
{
    unsigned long count;  // rax

    count = fwrite_unlocked(a0, 1, a1, a2);
    if (a1 != count)
        sub_404b30(dcgettext(NULL, "write failed", 5)); /* do not return */
    return count;
}



// Function: skip_white @ 0x4e40
typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    char field_0;
    char padding_1[7];
    struct struct_1 *field_8;
    unsigned long long field_10;
} struct_0;

extern struct_0 *stdin;
extern char g_410158;
extern void g_41015c;
extern int g_4105a8;
extern char *g_4105b8;

long long skip_white(void)
{
    unsigned long long *v1;  // rbx
    char *v2;  // rax
    unsigned long v3;  // rcx
    unsigned long v4;  // rax

    v1 = __ctype_b_loc();
    do
    {
        v2 = &stdin->field_8->field_0;
        if (v2 < stdin->field_10)
        {
            v3 = *(v1);
            stdin->field_8 = v2 + 1;
            v4 = *(v2);
            if (!(*((char *)(v3 + (int)v4 * 2 + 1)) & 32))
                goto LABEL_404ea0;
        }
        else
        {
            v4 = __uflow(stdin);
            if (!(*((char *)(*(v1) + (int)v4 * 2 + 1)) & 32))
                goto LABEL_404ea0;
        }
        if ((unsigned int)v4 == 10)
        {
LABEL_404ea0:
            if (stdin->field_0 & 32)
                sub_404b30(dcgettext(NULL, "read failed", 5)); /* do not return */
            return v4;
        }
    } while (!*((int *)&g_41015c));
    if (g_4105a8 > 0)
        kill(g_4105a8, 13);
    if (g_4105b8)
        unlink(g_4105b8);
    if (g_410158)
        sub_404a10(*((int *)&g_41015c));
    raise(*((int *)&g_41015c));
    exit(2); /* do not return */
}



// Function: lf_skip @ 0x4f30
typedef struct struct_0 {
    long long field_0;
    struct struct_1 *field_8;
    struct struct_1 *field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_410158;
extern void g_41015c;
extern int g_4105a8;
extern char *g_4105b8;

char * lf_skip(struct_0 *idx, unsigned long long a1)
{
    unsigned long long v1;  // rbp
    char *ptr;  // rax
    unsigned long long v3;  // rax
    char *v4;  // rdx
    char *v5;  // rdx
    char *v6;  // rax

    v1 = a1;
    if (!a1)
        return v6;
    while (1)
    {
        while (1)
        {
            ptr = rawmemchr(idx->field_8, 10);
            idx->field_8 = ptr;
            if (ptr == idx->field_18)
                break;
            idx->field_8 = ptr + 1;
            v1 -= 1;
            if (v1 == 1)
                return ptr + 1;
        }
        v3 = sub_404d80(idx->field_10, idx->field_0);
        v4 = &idx->field_10->field_0;
        idx->field_8 = v4;
        v5 = &v4[v3];
        idx->field_18 = v5;
        *(v5) = 10;
        if (*((int *)&g_41015c))
            break;
        if (!v3)
            return 0;
    }
    if (g_4105a8 > 0)
        kill(g_4105a8, 13);
    if (g_4105b8)
        unlink(g_4105b8);
    if (g_410158)
        sub_404a10(*((int *)&g_41015c));
    raise(*((int *)&g_41015c));
    exit(2); /* do not return */
}



// Function: lf_copy @ 0x5000
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    long long field_0;
    struct struct_1 *field_8;
    struct struct_1 *field_10;
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_410158;
extern void g_41015c;
extern int g_4105a8;
extern char *g_4105b8;

long long lf_copy(struct_0 *idx, unsigned long long a1, FILE *a2)
{
    char *v1;  // rdi
    char *v2;  // rbp
    unsigned long long v3;  // rbx
    char *v4;  // r12
    char *ptr;  // rax
    unsigned long long v6;  // rax
    char *v7;  // rdx
    char *v8;  // rdx
    unsigned long long v9;  // rsi

    v1 = &idx->field_8->field_0;
    if (a1)
    {
        v2 = &idx->field_18->field_0;
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
                v9 = v1 - v4;
                break;
            }
            else
            {
                sub_404b80(v4, v2 - v4, a2);
                v6 = sub_404d80(idx->field_10, idx->field_0);
                v7 = &idx->field_10->field_0;
                idx->field_8 = v7;
                v8 = &v7[v6];
                idx->field_18 = v8;
                *(v8) = 10;
                if (*((int *)&g_41015c))
                {
                    if (g_4105a8 > 0)
                        kill(g_4105a8, 13);
                    if (g_4105b8)
                        unlink(g_4105b8);
                    if (g_410158)
                        sub_404a10(*((int *)&g_41015c));
                    raise(*((int *)&g_41015c));
                    exit(2); /* do not return */
                }
                else if (v6)
                {
                    v4 = &idx->field_8->field_0;
                    v2 = &idx->field_18->field_0;
                    v1 = v4;
                }
                else
                {
                    return 0;
                }
            }
        }
    }
    else
    {
        v4 = v1;
        v9 = 0;
    }
    return sub_404b80(v4, v9, a2);
}



// Function: edit @ 0x5120
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_410010;
extern struct_0 *stdout;
extern struct_1 *stdin;
extern unsigned long long stderr;
extern char g_410158;
extern unsigned int g_41015c;
extern char g_410160;
extern char g_410161;
extern int g_4105a8;
extern FILE *g_4105b0;
extern unsigned long long g_4105b8;

int edit(void)
{
    unsigned long v13;  // fs
    unsigned long v22;  // r8
    unsigned long v23;  // r9
    FILE *fp;  // rax
    char *v25;  // rax
    unsigned long v26;  // r12
    unsigned int v27;  // eax
    unsigned int v28;  // ebp
    unsigned int v29;  // eax
    FILE *fp1;  // rax
    unsigned int v14;  // ebx
    unsigned int *err;  // rax
    unsigned long v15;  // rax
    int v16;  // eax
    unsigned long v17;  // rax
    unsigned long v18;  // rdi
    unsigned long v19;  // rsi
    long long v20;  // rdx
    unsigned long v21;  // rcx
    unsigned long v0;  // [bp-0x101d8]
    int v1;  // [bp-0x100ac]
    int v2;  // [bp-0x1006c], Other Possible Types: unsigned int
    unsigned long v3;  // [bp-0x10068]
    unsigned long long v4;  // [bp-0x10060]
    unsigned long long v5;  // [bp-0x10058]
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
LABEL_405198:
    while (1)
    {
        v15 = stdout[1].field_10;
        if (v15 < *((long long *)&stdout[2].field_0))
        {
            stdout[1].field_10 = v15 + 1;
            *((char *)v15) = 37;
        }
        else if (!(__overflow(stdout, 37) == 37))
        {
            goto LABEL_40557b;
        }
        if (fflush_unlocked(stdout))
        {
LABEL_40557b:
            sub_404b30(dcgettext(NULL, "write failed", 5)); /* do not return */
        }
        v16 = sub_404e40();
        if (v16 <= 50)
        {
            if (v16 > 48)
                goto LABEL_405295;
            if (v16 != -0x1)
            {
                if (v16 == 10)
                    goto LABEL_4051f7;
                goto LABEL_405236;
            }
            else
            {
                if (!(stdin->padding_0[0] & 16))
                    goto LABEL_405236;
                else
                    goto LABEL_4054c8;
            }
        }
        else if (v16 != 101)
        {
            if (v16 > 100)
            {
                if (v16 <= 115)
                {
                    if (v16 > 112)
                        goto LABEL_405295;
                    if (v16 == 108)
                    {
                        if ((int)sub_404e40() != 10)
                            goto LABEL_40525d;
                        goto LABEL_4055e8;
                    }
                }
                else if (v16 == 118)
                {
LABEL_405295:
                    v1 = v16;
                    if ((int)sub_404e40() == 10)
                    {
                        if (v1 > 118)
                            goto LABEL_4055a0;
                        if (v1 > 100)
                        {
                            switch (v1)
                            {
                            case 101:
LABEL_4052f2:
                                if (g_4105b8)
                                {
                                    fp = fopen(g_4105b8, "w");
                                }
                                else
                                {
                                    v25 = getenv("TMPDIR");
                                    if (!v25)
                                        v25 = "/tmp";
                                    v26 = sub_407df0(strlen(v25) + 13);
                                    __sprintf_chk(v26, 0x1);
                                    v27 = mkstemp(v26);
                                    if (v27 < 0)
                                        sub_404b30("mkstemp"); /* do not return */
                                    g_4105b8 = v26;
                                    fp = fdopen(v27, "w");
                                }
                                g_4105b0 = fp;
                                if (!fp)
                                    sub_404b30(g_4105b8); /* do not return */
                                switch (v14)
                                {
                                case 100:
                                    if (v21)
                                    {
                                        if (v21 != 1)
                                        {
                                            __fprintf_chk(*((unsigned int *)&fp), 0x1, "--- %s %ld,%ld\n", v19, v20, v20 + v21 - 1);
                                            fp = g_4105b0;
                                        }
                                        else
                                        {
                                            __fprintf_chk(*((unsigned int *)&fp), 0x1, "--- %s %ld\n", v19, v20);
                                            fp = g_4105b0;
                                        }
                                    }
                                    sub_405000(v18, v21, fp);
                                    if (v11)
                                    {
                                        if (v11 != 1)
                                        {
                                            __fprintf_chk(*((unsigned int *)&g_4105b0), 0x1, "+++ %s %ld,%ld\n", v23, v10, v10 + v11 - 1);
                                            break;
                                        }
                                        else
                                        {
                                            __fprintf_chk(*((unsigned int *)&g_4105b0), 0x1, "+++ %s %ld\n", v23, v10);
                                            break;
                                        }
                                    }
LABEL_40536b:
                                    sub_405000(v22, v11, g_4105b0);
                                    goto LABEL_405384;
                                case 49: case 98: case 108:
                                    sub_405000(v18, v21, fp);
                                default:
                                    sub_404f30(v18, v21);
                                    goto LABEL_405354;
                                }
LABEL_405354:
                                if (v14 <= 100)
                                {
                                    if (v14 != 50 && v14 != 98)
                                        goto LABEL_40569a;
                                    goto LABEL_40536b;
                                }
                                else if (v14 != 114)
                                {
LABEL_40569a:
                                    sub_404f30(v22, v11);
LABEL_405384:
                                    if (fclose(g_4105b0))
                                        sub_404b30("fclose"); /* do not return */
                                    g_410160 = 1;
                                    v28 = g_41015c;
                                    if (!g_41015c)
                                    {
                                        v5 = 0;
                                        v3 = g_410010;
                                        v4 = g_4105b8;
                                        v29 = fork();
                                        if (!v29)
                                        {
                                            execvp(g_410010, &v3);
                                            err = __errno_location();
                                            _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
                                        }
                                        else if (v29 >= 0)
                                        {
                                            do
                                            {
                                                if (waitpid(v29, &v2, 0) >= 0)
                                                {
                                                    g_410160 = 0;
                                                    sub_404dc0(v2, 0, g_410010);
                                                    fp1 = fopen(g_4105b8, "r");
                                                    if (!fp1)
                                                        sub_404b30(g_4105b8); /* do not return */
                                                    g_4105b0 = fp1;
                                                }
                                                if (*(__errno_location()) != 4)
                                                    sub_404b30("waitpid"); /* do not return */
                                                v28 = g_41015c;
                                            } while (!g_41015c);
                                            if (g_4105a8 > 0)
                                                kill(g_4105a8, 13);
                                            if (g_4105b8)
                                                unlink(g_4105b8);
                                        }
                                        else
                                        {
                                            sub_404b30("fork"); /* do not return */
                                        }
                                    }
                                    else
                                    {
                                        sub_404990(0);
                                    }
                                    if (g_410158)
                                        sub_404a10(v28);
                                    raise(v28);
                                    exit(2); /* do not return */
                                }
                            case 108:
LABEL_4055e8:
                                sub_405000(v18, v21, v12);
                                sub_404f30(v22, v11);
                                break;
                            case 113:
LABEL_4054c8:
                                break;
                            case 114:
LABEL_405536:
                                sub_405000(v22, v11, v12);
                                sub_404f30(v18, v21);
                                break;
                            case 115:
                                g_410161 = 1;
                                goto LABEL_405198;
                            case 118:
                                g_410161 = 0;
                                goto LABEL_405198;
                            default:
LABEL_4055a0:
                                dcgettext(NULL, "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n", 5);
                                __fprintf_chk(stderr, 0x1, "%s");
                                goto LABEL_405198;
                            }
                            if (v9 != *((long long *)(40 + v13)))
                                __stack_chk_fail(); /* do not return */
                            return;
                        }
                        if (v1 == 49)
                            goto LABEL_4055e8;
                        if (v1 == 50)
                            goto LABEL_405536;
                        else
                            goto LABEL_4055a0;
                    }
                    goto LABEL_40525d;
                }
            }
LABEL_405236:
            sub_404d10();
LABEL_4051f7:
            dcgettext(NULL, "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n", 5);
            __fprintf_chk(stderr, 0x1, "%s");
        }
        else
        {
            v14 = sub_404e40();
            if (v14 <= 50)
            {
                if (v14 > 48)
                    goto LABEL_40550e;
                if (v14 == 10)
                    goto LABEL_4052f2;
                else
                    goto LABEL_40525d;
            }
            else
            {
                v17 = v14 - 98;
                if ((unsigned int)v17 <= 16 && !(v0 = 66565, !(((char)v17 & 254 | *((char *)((char *)&v0 + ((long long)(v17 & 63) >> 3))) >> (unsigned long long)((char)v17 & 63 & 7) & 1) & 1)))
                {
LABEL_40550e:
                    if ((int)sub_404e40() == 10)
                        goto LABEL_4052f2;
                    else
                        goto LABEL_40525d;
                }
            }
LABEL_40525d:
            dcgettext(NULL, "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n", 5);
            __fprintf_chk(stderr, 0x1, "%s");
            sub_404d10(v18, v19, v20, v21, v22, v23);
        }
    }
}



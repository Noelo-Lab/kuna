// Function: main @ 0x3ac0
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

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_40da64;
extern option g_412920;
extern unsigned long long g_413010;
extern long long g_413018;
extern unsigned int g_413020;
extern long long stdout;
extern void stdin;
extern int optind;
extern unsigned long long optarg;
extern char g_4130e9;
extern char g_4130ea;
extern char g_4130eb;
extern char g_4130ec;
extern char g_4130ed;
extern char g_4130ee;
extern char g_4130ef;
extern char g_4130f0;
extern char g_4130f1;
extern char g_4130f2;

unsigned int main(int a0, void* a1)
{
    unsigned int v15;  // r15d
    unsigned long v16;  // rbp
    long long v25;  // rax
    char v26;  // cl
    char v27;  // al
    char v28;  // al
    unsigned int v29;  // esi
    void* v30;  // rbp
    unsigned long idx;  // rax
    unsigned int v32;  // ebx
    unsigned int v33;  // r12d
    unsigned long index;  // r14
    unsigned long v17;  // rdi
    void* v35;  // rbx
    long long v37;  // rax
    long long v38;  // rax
    char v39;  // bl
    long long v18;  // rdi
    long long v19;  // rsi
    long long v20;  // rdx
    long long v21;  // rcx
    long long v22;  // r8
    long long v23;  // r9
    unsigned int v24;  // eax
    int v0;  // [bp-0x12c]
    char v1;  // [bp-0x128]
    void* v2;  // [bp-0x120]
    char v3;  // [bp-0x118]
    unsigned int v4;  // [bp-0x110]
    unsigned int v5;  // [bp-0x10c]
    unsigned int v6;  // [bp-0x108]
    unsigned int v7;  // [bp-0x104]
    int v8;  // [bp-0x100]
    int v9;  // [bp-0xfc]
    int <0x403ac0[is_2]|Stack bp-0xf8, 1 B>;  // [bp-0xf8]
    long long v11;  // [bp-0xf0]
    long long v12;  // [bp-0xe8]
    stat v13;  // [bp-0xd8]

    v15 = 0;
    v16 = a0;
    v17 = *((long long *)a1);
    g_413020 = 2;
    sub_4062e0(v17);
    setlocale(6, &g_40da64);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_406220(0);
    sub_4086a0(v18, v19, v20, v21, v22, v23);
    v0 = 0;
    while (1)
    {
        v24 = getopt_long(v16, a1, "aeimvx3AEL:TX", &g_412920.name, NULL);
        if (v24 == 0xffffffff)
            break;
        switch (v24)
        {
        case 51:
            g_4130ed = 1;
            v15 |= 1;
            break;
        case 65:
            g_4130eb = 1;
            v15 |= 2;
            g_4130ef = 1;
            break;
        case 69:
            g_4130ef = 1;
            v15 |= 4;
            break;
        case 76:
            if (v0 <= 2)
            {
                (&<0x403ac0[is_2]|Stack bp-0xf8, 1 B>)[v0] = optarg;
                v0 += 1;
                break;
            }
            else
            {
                sub_4041b0("too many file label options", 0); /* do not return */
            }
        case 84:
            g_4130ee = 1;
            break;
        case 88:
            g_4130ec = 1;
            v15 |= 8;
            break;
        case 97:
            g_4130f2 = 1;
            break;
        case 101:
            v15 |= 16;
            break;
        case 105:
            g_4130ea = 1;
            break;
        case 109:
            g_4130e9 = 1;
            break;
        case 118:
            v25 = sub_4065f0("Randy Smith");
            sub_408010(stdout, "diff3", "GNU diffutils", g_413018, v25, 0);
            sub_405c30(v18, v19, v20, v21, v22, v23);
            return 0;
        case 120:
            g_4130ec = 1;
            v15 |= 32;
            break;
        case 128:
            g_413010 = optarg;
            break;
        case 129:
            sub_404240();
            sub_405c30(v18, v19, v20, v21, v22, v23);
            return 0;
        case 130:
            g_4130f1 = 1;
            break;
        default:
            sub_4041b0(0, 0); /* do not return */
        }
    }
    v26 = v15;
    g_4130f0 = (g_4130e9 ^ 1) & v26;
    v27 = !v15 & g_4130e9;
    g_4130eb = g_4130eb | v27;
    v28 = v27 | g_4130ef;
    g_4130ef = v27 | g_4130ef;
    if (!(v15 & v15 - 1) && (!g_4130ea || !g_4130e9) && (!v0 || v28))
    {
        v29 = (v16 & 0xffffffff) - optind;
        if (v29 == 3)
        {
            v30 = a1 + optind * 8;
            for (idx = v0; (unsigned int)idx <= 2; idx += 1)
            {
                (&<0x403ac0[is_2]|Stack bp-0xf8, 1 B>)[idx] = *((long long *)((char *)v30 + 8 * idx));
            }
            v32 = v26 | g_4130e9;
            v33 = 2 - v32;
            index = v33;
            if (!strcmp(*((long long *)((char *)v30 + 8 * index)), "-"))
            {
                v33 = v32 + 1;
                if (!strcmp(*((long long *)v30), "-") || !(index = (unsigned long)(long long)(int)v33, (int)(long long)strcmp(*((long long *)((char *)v30 + 8 * index)), "-")))
                    sub_404210("'-' specified for more than one input file"); /* do not return */
            }
            v6 = v33;
            v35 = v30;
            v4 = 0;
            v7 = 0;
            v5 = 3 - v33;
            (&v7)[v5] = 1;
            (&v7)[index] = 2;
            do
            {
                if (strcmp(*((long long *)v35), "-"))
                {
                    if (stat(*((long long *)v35), &v13) >= 0)
                    {
                        if (((unsigned short)v13.st_nlink & 0xf000) == 0x4000)
                            error(2, 21, "%s");
                    }
                    else
                    {
                        sub_4043a0(*((long long *)v35)); /* do not return */
                    }
                }
            } while ((v35 += 8, v30 + 24 != v35));
            signal(0x11, NULL);
            v37 = sub_405d70(*((long long *)((char *)v30 + 8 * v8)), *((long long *)((char *)v30 + 8 * v9)), &v1, &v3);
            v38 = sub_404e40(sub_405d70(*((long long *)((char *)v30 + 8 * v7)), *((long long *)((char *)v30 + 8 * v9)), &v1, &v2), v37);
            if (!g_4130f0)
            {
                v39 = g_4130e9;
                if (g_4130e9)
                {
                    sub_408570(*((long long *)((char *)v30 + 8 * v7)), "r", *((long long *)&stdin));
                    v39 = sub_4057d0(*((long long *)&stdin), stdout, v38, &v4, &v7, <0x403ac0[is_2]|Stack bp-0xf8, 1 B>, v11, v12);
                    if (ferror_unlocked(*((long long *)&stdin)))
                        sub_404210("read failed"); /* do not return */
                }
                else
                {
                    sub_405520(stdout, v38, &v4, &v7);
                }
            }
            else
            {
                v39 = sub_4050f0(stdout, v38, &v4, &v7, <0x403ac0[is_2]|Stack bp-0xf8, 1 B>, v11, v12, v19);
            }
            free(v2);
            free(v3);
            sub_405c30(v18, v19, v20, v21, v22, v23);
            exit(v39); /* do not return */
        }
        else if (v29 <= 2)
        {
            sub_4041b0("missing operand after '%s'", *((long long *)((char *)a1 + 8 * v16 - 8))); /* do not return */
        }
        else
        {
            sub_4041b0("extra operand '%s'", *((long long *)((char *)a1 + 8 * optind + 24))); /* do not return */
        }
    }
    sub_4041b0("incompatible options", 0); /* do not return */
}



// Function: skipwhite @ 0x4110
char * skipwhite(char *a0)
{
    char v1;  // dl
    char *v2;  // rax
    char *v3;  // rax
    char *v4;  // rax

    v1 = *(a0);
    v2 = a0;
    v3 = v2;
    if (*(a0) != 32)
        goto LABEL_40412d;
    while (1)
    {
        v4 = v2;
        v1 = v4[1];
        v3 = v4 + 1;
        v2 = v3;
        if (v4[1] == 32)
            continue;
LABEL_40412d:
        v2 = v3;
        if (v1 != 9)
            break;
    }
    return v2;
}



// Function: readnum @ 0x4140
char * readnum(char *a0, unsigned long long *a1)
{
    unsigned int v1;  // edx
    unsigned long long v2;  // rax

    v1 = *(a0);
    v2 = 0;
    if (v1 - 48 > 9)
        return NULL;
    do
    {
        a0 += 1;
        v2 = v1 - 48 + v2 * 10;
        v1 = *(a0);
    } while (v1 - 48 <= 9);
    *(a1) = v2;
    return a0;
}



// Function: reverse_diff3_blocklist @ 0x4180
typedef struct struct_0 {
    char padding_0[104];
    struct struct_0 *field_68;
} struct_0;

struct_0 * reverse_diff3_blocklist(struct_0 *idx)
{
    struct_0 *v1;  // rcx
    struct_0 *v2;  // rdx

    if (!idx)
        return idx;
    v1 = NULL;
    while (1)
    {
        v2 = idx->field_68;
        idx->field_68 = v1;
        if (!v2)
            break;
        idx = v2;
        v1 = idx;
    }
    return idx;
}



// Function: try_help @ 0x41b0
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: fatal @ 0x4210
void fatal(char *a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    dcgettext(NULL, a0, 5);
    error(2, 0, "%s");
}



// Function: usage @ 0x4240
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern struct_0 *g_412880;
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
    char *v12;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    iter = &g_412880;
    __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n", 5));
    __printf_chk(1, "%s\n\n", dcgettext(NULL, "Compare three files line by line.", 5));
    v8 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v8, stdout);
    v9 = 45;
    v10 = "-A, --show-all              output all changes, bracketing conflicts";
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
    v11 = dcgettext(NULL, "\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n", 5);
    fputs_unlocked(v11, stdout);
    v12 = dcgettext(NULL, "Exit status is 0 if successful, 1 if conflicts, 2 if trouble.", 5);
    __printf_chk(1, "\n%s\n%s\n", dcgettext(NULL, "If a FILE is '-', read standard input.", 5), v12);
    fputs_unlocked("\n", stdout);
    __printf_chk(1, dcgettext(NULL, "Report bugs to: %s\n", 5));
    __printf_chk(1, dcgettext(NULL, "%s home page: <%s>\n", 5));
    return __printf_chk(1, dcgettext(NULL, "General help using GNU software: <%s>\n", 5));
}



// Function: perror_with_exit @ 0x43a0
void perror_with_exit(unsigned long a0)
{
    unsigned long long v2;  // r12
    int *err;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    err = __errno_location();
    error(2, *(err), "%s");
}



// Function: read_diff @ 0x43d0
typedef struct struct_0 {
    char field_0;
    char padding_1[15];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
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

extern unsigned long long g_413010;
extern char g_4130f1;
extern char g_4130f2;

long long read_diff(unsigned long a0, unsigned long a1, long long *a2)
{
    struct_0 *iter;  // rax
    unsigned int v9;  // r12d
    unsigned int *err;  // rax
    unsigned long long v11;  // rax
    unsigned long long v12;  // rbx
    long long v13;  // rax
    long long v14;  // r14
    long long v15;  // rax
    char v16;  // al
    char *v17;  // rsi
    unsigned int v0;  // [bp-0x144]
    unsigned int v1;  // [bp-0x134]
    char v2;  // [bp-0x130]
    int v3;  // [bp-0x12c]
    unsigned long v4;  // [bp-0x128]
    int v5;  // [bp-0x120], Other Possible Types: char
    stat v6;  // [bp-0xd8]

    v4 = g_413010;
    iter = &v5;
    if (g_4130f2)
    {
        *((char **)&v5) = "-a";
        iter = &v5 - 8;
    }
    if (g_4130f1)
    {
        iter = &iter->padding_1[7];
        *((char **)(&iter->field_0 - 8)) = "--strip-trailing-cr";
    }
    iter->field_10 = a0;
    *((char **)&iter->field_0) = "--horizon-lines=100";
    *((char **)&iter->padding_1[7]) = "--";
    iter->field_18 = a1;
    iter->field_20 = 0;
    if (pipe(&v2))
        sub_4043a0("pipe"); /* do not return */
    v0 = fork();
    if (!v0)
    {
        close(v2);
        if (v3 != 1)
        {
            dup2(v3, 1);
            close(v3);
        }
        execvp(g_413010, &v4);
        err = __errno_location();
        _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
    }
    else if (v0 != 0xffffffff)
    {
        close(v3);
        v9 = v2;
        if (fstat(v9, &v6))
            sub_4043a0("fstat"); /* do not return */
        v11 = 1;
        if (v6.st_mtime > 0)
            v11 = v6.st_mtime;
        v12 = 0;
        v13 = sub_408180();
        while (1)
        {
            v14 = v11 - v12;
            v15 = sub_408770(v9, v13 + v12, v14);
            v12 += v15;
            if (v14 != v15)
                break;
            if (v11 > 0x3ffffffffffffffe)
                sub_408530(); /* do not return */
            v11 *= 2;
            v13 = sub_4081d0(v13, v11);
        }
        if (v15 == -0x1)
            sub_4043a0(dcgettext(NULL, "read failed", 5)); /* do not return */
        if (v12 && *((char *)(v13 + v12 - 1)) != 10)
            sub_404210("invalid diff format; incomplete last line"); /* do not return */
        *(a2) = v13;
        if (close(v9))
        {
            sub_4043a0("close"); /* do not return */
        }
        else if (waitpid(v0, &v1, 0) >= 0)
        {
            if (!((char)v1 & 127))
            {
                v16 = (int)(v1) >> 8;
                if (!(v16 & 254))
                    return v13 + v12;
                if (v16 == 126)
                {
                    v17 = "subsidiary program '%s' could not be invoked";
                }
                else
                {
                    v17 = "subsidiary program '%s' failed (exit status %d)";
                    if (v16 == 127)
                        v17 = "subsidiary program '%s' not found";
                }
            }
            else
            {
                v17 = "subsidiary program '%s' failed";
            }
            error(2, 0, dcgettext(NULL, v17, 5));
        }
        else
        {
            sub_4043a0("waitpid"); /* do not return */
        }
    }
    else
    {
        sub_4043a0("fork"); /* do not return */
    }
}



// Function: undotlines @ 0x46d0
int undotlines(unsigned long a0, char a1, int a2, unsigned long long a3)
{
    unsigned int v1;  // ebp
    int v2;  // eax

    v1 = a0;
    v2 = fputs_unlocked(".\n", a0);
    if (!a1)
    {
        return v2;
    }
    else if (a3 != 1)
    {
        return __fprintf_chk(v1, 0x1, "%ld,%lds/^\\.//\n", a2, a2 + a3 - 1);
    }
    else
    {
        return __fprintf_chk(v1, 0x1, "%lds/^\\.//\n", a2);
    }
}



// Function: scan_diff_line @ 0x4770
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_0;

extern FILE *stderr;
extern char g_4130f0;
extern char *g_423140;

struct_0 * scan_diff_line(struct_0 *a0, struct_0 **a1, unsigned long long *a2, struct_0 *a3, char a4)
{
    struct_0 *v1;  // rdi
    struct_0 *i;  // rbx
    struct_0 *v3;  // rbx
    unsigned long long v4;  // rsi
    char v5;  // al
    struct_0 *v6;  // r12

    if (a0->field_0 == a4 && a0->field_1 == 32)
    {
        v1 = &a0->field_2;
        *(a1) = v1;
        for (i = v1; i->field_0 != 10; i = &i->field_1);
        v4 = v3 - v1;
        *(a2) = v4;
        if (v3 >= a3)
        {
            return v3;
        }
        else if (v3->field_0 != 92)
        {
            return v3;
        }
        else
        {
            v5 = g_4130f0;
            if (!g_4130f0)
            {
                *(a2) = v4 - 1;
            }
            else
            {
                __fprintf_chk(*((unsigned int *)&stderr), 0x1, "%s:", g_423140);
                v5 = g_4130f0;
            }
            v6 = &i->field_2;
            while (1)
            {
                if (v5)
                    putc_unlocked(v6->field_0, stderr);
                v6 = &v6->field_1;
                if (*(&v6->field_0 - 1) == 10)
                    break;
                v5 = g_4130f0;
            }
            return v6;
        }
    }
    sub_404210("invalid diff format; incorrect leading line chars"); /* do not return */
}



// Function: create_diff3_block @ 0x4850
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
    unsigned long long field_50;
    unsigned long long field_58;
    unsigned long long field_60;
    unsigned long long field_68;
} struct_0;

struct_0 * create_diff3_block(unsigned long a0, unsigned long a1, unsigned long long a2, unsigned long long a3, unsigned long a4, unsigned long a5)
{
    struct_0 *v1;  // rax
    long long v2;  // rbp
    unsigned long long v11;  // rax
    long long v12;  // rdi
    long long v13;  // rdi
    unsigned long long v14;  // rax
    struct_0 *idx;  // r12
    long long v4;  // rbp
    long long v5;  // rbx
    long long v6;  // rbx
    unsigned long long v7;  // rax
    unsigned long long v8;  // rbx
    unsigned long long v9;  // r13
    long long v10;  // rbx

    v1 = sub_408180(112);
    v1->field_10 = a1;
    v2 = a1 - a0;
    idx = v1;
    idx->field_0 = 0;
    idx->field_68 = 0;
    idx->field_8 = a0;
    idx->field_18 = a2;
    idx->field_20 = a3;
    idx->field_28 = a4;
    idx->field_30 = a5;
    v4 = v2 + 1;
    if (v2 == -0x1)
    {
        v5 = a3 - a2;
        idx->field_38 = 0;
        idx->field_50 = 0;
        v6 = v5 + 1;
        if (v5 != -0x1)
            goto LABEL_40495b;
LABEL_4048d0:
        idx->field_40 = 0;
        v11 = 0;
    }
    else
    {
        idx->field_38 = sub_408410(v4, 8);
        v7 = sub_408410(v4, 8);
        v8 = idx->field_20;
        v9 = idx->field_18;
        idx->field_50 = v7;
        v10 = v8 - v9;
        v6 = v10 + 1;
        if (v10 == -0x1)
            goto LABEL_4048d0;
LABEL_40495b:
        idx->field_40 = sub_408410(v6, 8);
        v11 = sub_408410(v6, 8);
    }
    v12 = idx->field_30 - idx->field_28;
    idx->field_58 = v11;
    v13 = v12 + 1;
    if (v12 == -0x1)
    {
        idx->field_48 = 0;
        v14 = 0;
    }
    else
    {
        idx->field_48 = sub_408410(v13, 8);
        v14 = sub_408410(v13, 8);
    }
    idx->field_60 = v14;
    return idx;
}



// Function: copy_stringlist @ 0x49b0
unsigned int copy_stringlist(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long long a4)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long iter;  // rbx
    unsigned long v12;  // rdx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!a4)
        return 1;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    iter = 0;
    while (1)
    {
        v12 = *((long long *)(a1 + iter * 8));
        if (!*((long long *)(a2 + iter * 8)))
        {
            *((long long *)(a2 + iter * 8)) = *((long long *)(a0 + iter * 8));
            *((unsigned long *)(a3 + iter * 8)) = v12;
            iter += 1;
            if (iter == a4)
                return 1;
        }
        else if (*((long long *)(a3 + iter * 8)) != v12)
        {
            return 0;
        }
        else if (!memcmp(*((long long *)(a0 + iter * 8)), *((long long *)(a2 + iter * 8)), v12))
        {
            iter += 1;
            if (iter == a4)
                return 1;
        }
        else
        {
            return 0;
        }
    }
}



// Function: compare_line_list @ 0x4a50
unsigned int compare_line_list(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long long a4)
{
    unsigned long long v1;  // rbx
    unsigned long long v2;  // rbx

    v1 = 0;
    while (1)
    {
        v2 = v1;
        if (a4 == v2)
        {
            return 1;
        }
        else if (!*((long long *)(a0 + v2 * 8)))
        {
            return 0;
        }
        else if (!*((long long *)(a2 + v2 * 8)))
        {
            return 0;
        }
        else if (*((long long *)(a1 + v2 * 8)) == *((long long *)(a3 + v2 * 8)))
        {
            v1 = v2 + 1;
            if (memcmp(*((long long *)(a0 + v2 * 8)), *((long long *)(a2 + v2 * 8)), *((long long *)(a1 + v2 * 8))))
                return 0;
        }
        else
        {
            return 0;
        }
    }
}



// Function: using_to_diff3_block @ 0x4ae0
typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
} struct_2;

typedef struct struct_4 {
    char padding_0[64];
    unsigned long long field_40;
} struct_4;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned int field_20;
    char padding_24[4];
    unsigned int field_28;
    char padding_2c[4];
    unsigned int field_30;
    char padding_34[4];
    unsigned int field_38;
    char padding_3c[4];
    struct struct_0 *field_40;
} struct_0;

struct_0 * using_to_diff3_block(struct_1 *a0, unsigned long a1, int a2, int a3, struct_2 *idx)
{
    unsigned long v6;  // r12
    unsigned long v7;  // r9
    unsigned long v16;  // rdx
    struct_4 *index;  // rbx
    unsigned long long v19;  // rax
    unsigned long v20;  // rcx
    unsigned long v21;  // r15
    unsigned long v22;  // r13
    unsigned long v23;  // r12
    unsigned long v24;  // rbx
    unsigned long long i;  // rdx
    unsigned long long v8;  // rax
    long long idx1;  // rax
    unsigned long long v28;  // rax
    unsigned long v9;  // rdx
    unsigned long long v10;  // rdx
    unsigned long v11;  // rbp
    struct_0 *idx2;  // r14
    unsigned long iter;  // r15
    unsigned long v14;  // rdi
    unsigned long long v15;  // r12
    unsigned long v0;  // [bp-0x90], Other Possible Types: unsigned long long
    unsigned long v1;  // [bp-0x80], Other Possible Types: unsigned long long
    char *v2;  // [bp-0x78]
    char *v3;  // [bp-0x70]
    int v4;  // [bp-0x68], Other Possible Types: char

    v2 = &v4;
    v3 = &v4 - 16;
    v6 = (&a0->field_0)[a2]->field_10;
    v7 = *((long long *)(*((long long *)(a1 + a3 * 8)) + 24));
    v8 = 0;
    do
    {
        v9 = *((long long *)((char *)a0 + v8));
        if (v9)
        {
            *((unsigned long *)((char *)&v4 + v8)) = v6 - *((long long *)(v9 + 16)) + *((long long *)v9);
            v10 = v7 - *((long long *)(*((long long *)(a1 + v8)) + 24)) + *((long long *)(*((long long *)(a1 + v8)) + 8));
        }
        else
        {
            *((unsigned long long *)((char *)&v4 + v8)) = v6 - idx->field_30 + *((long long *)&idx->padding_0[16 + 2 * v8]);
            v10 = v7 - idx->field_30 + *((long long *)&idx->padding_0[16 + 2 * v8]);
        }
    } while ((*((unsigned long long *)((char *)&v4 + v8 - 16)) = v10, v8 += 8, v8 != 16));
    v11 = *((unsigned long long *)&v4);
    v1 = *((unsigned long long *)(&v4 + 16));
    v0 = 0;
    idx2 = sub_404850(v11, *((unsigned long long *)(&v4 + 16)), *((unsigned long long *)(&v4 + 8)), *((unsigned long long *)(&v4 + 24)), v6, v7);
    while (1)
    {
        iter = (&a0->field_0)[v0];
        if ((&a0->field_0)[v0])
        {
            do
            {
                if (!(char)sub_4049b0(*((long long *)(iter + 40)), *((long long *)(iter + 56)), (*((long long *)(iter + 16)) - v6) * 8 + idx2->field_48, idx2->field_60 + (*((long long *)(iter + 16)) - v6) * 8, *((long long *)(iter + 24)) - *((long long *)(iter + 16)) + 1))
                    return NULL;
            } while ((iter = (unsigned long)*((long long *)(iter + 64)), iter));
        }
        if (v0 == 1)
            break;
        v0 = 1;
    }
    v14 = v1;
    v0 = v6;
    v15 = 0;
    while (1)
    {
        v16 = v11;
        index = *((long long *)((char *)a0 + v15));
        v19 = 0;
        while (1)
        {
            v20 = v14 + 1;
            if (index)
                v20 = *((long long *)&index->padding_0[0]);
            if (v20 <= v16)
                break;
            v16 += 1;
            *((long long *)(*((long long *)((char *)idx2 + v15 + 56)) + v19)) = *((long long *)(idx2->field_48 + v19));
            *((long long *)(*((long long *)((char *)idx2 + v15 + 80)) + v19)) = *((long long *)(idx2->field_60 + v19));
            v19 += 8;
        }
        if (index)
        {
            v1 = v15;
            v21 = *((long long *)((char *)idx2 + v15 + 56));
            v22 = *((long long *)((char *)idx2 + v15 + 80));
            v23 = v11;
            while (1)
            {
                if (!(char)sub_4049b0(*((long long *)&index->padding_0[32]), *((long long *)&index->padding_0[48]), (*((long long *)&index->padding_0[0]) - v23) * 8 + v21, v22 + (*((long long *)&index->padding_0[0]) - v23) * 8, *((long long *)&index->padding_0[8]) - *((long long *)&index->padding_0[0]) + 1))
                    return NULL;
                v24 = index->field_40;
                i = (*((long long *)&index->padding_0[24]) - v0) * 8 + 8;
                for (idx1 = *((long long *)&index->padding_0[8]) + 1 - v23; !v24; i += 8)
                {
                    if (v14 + 1 - v11 <= idx1)
                    {
                        v15 = v1;
                        goto LABEL_404d8b;
                    }
LABEL_404d5b:
                    *((long long *)(v21 + idx1 * 8)) = *((long long *)(idx2->field_48 + i));
                    *((long long *)(v22 + idx1 * 8)) = *((long long *)(idx2->field_60 + i));
                    idx1 += 1;
                }
                index = v24;
                goto LABEL_404d5b;
            }
        }
LABEL_404d8b:
        v15 += 8;
        if (v15 == 16)
            break;
        v11 = *((long long *)&v2[v15]);
        v14 = *((long long *)&v3[v15]);
    }
    if (!a0->field_0)
    {
        idx2->field_0 = 6;
        return idx2;
    }
    else if (a0->field_8)
    {
        v28 = idx2->field_10 - idx2->field_8;
        if (v28 == idx2->field_20 - idx2->field_18 && (char)sub_404a50(idx2->field_38, idx2->field_50, idx2->field_40, idx2->field_58, v28 + 1))
        {
            idx2->field_0 = 7;
            return idx2;
        }
        idx2->field_0 = 4;
        return idx2;
    }
    else
    {
        idx2->field_0 = 5;
        return idx2;
    }
}



// Function: make_3way_diff @ 0x4e40
typedef struct struct_0 {
    char padding_0[16];
    long long field_10;
    unsigned long long field_18;
    char padding_20[32];
    unsigned long long field_40;
} struct_0;

typedef struct struct_2 {
    char padding_0[64];
    unsigned long long field_40;
} struct_2;

typedef struct struct_1 {
    struct struct_1 *field_0;
} struct_1;

extern struct_1 *g_40dd60;

unsigned long long make_3way_diff(struct_0 *a0, struct_0 *a1)
{
    struct_0 *index;  // rdx
    struct_0 *v8;  // rax
    unsigned long long idx;  // rdx
    struct_2 *v18;  // rsi
    unsigned long v19;  // rsi
    unsigned long v20;  // rdx
    unsigned int v21;  // eax
    long long v23;  // rax
    long long v24;  // 4102
    struct_1 **v9;  // r8
    struct_1 **v10;  // rbx
    unsigned long long idx1;  // rsi
    unsigned long long idx2;  // rax
    unsigned int v13;  // r10d
    unsigned long long v14;  // rdi
    unsigned long v15;  // rcx
    unsigned int v16;  // r9d
    char v0;  // [bp-0x50], Other Possible Types: unsigned long long
    struct_1 v1;  // [bp-0x48]
    unsigned long long v2;  // [bp-0x40]
    unsigned long v3;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x30]
    struct_0 *v5;  // [bp-0x28]

    index = a0;
    v8 = a1;
    v9 = &g_40dd60;
    v10 = &v0;
    v5 = a0;
    v0 = 0;
    if (!index)
        goto LABEL_404fab;
    while (1)
    {
        v4 = 0;
        v3 = 0;
        v2 = 0;
        v1 = (struct_1)0;
        if (v8)
        {
            v23 = v8->field_10;
            v24 = index->field_10;
            v13 = v23 < v24;
            idx1 = v23 < v24;
            idx2 = v24 <= v23;
            index = (&v5)[idx1];
        }
        else
        {
            idx2 = 1;
            v13 = 0;
            idx1 = 0;
        }
        while (1)
        {
            v14 = index->field_40;
            v15 = index->field_18;
            (&v3)[idx1] = index;
            v16 = v13;
            (&v1)[idx1].field_0 = index;
            (&v5)[idx1] = v14;
            index->field_40 = 0;
            idx = (&v5)[idx2];
            if (idx)
            {
                do
                {
                    if (*((long long *)(idx + 16)) > v15 + 1)
                        break;
                    if ((&v1)[idx2].field_0)
                        *((unsigned long long *)((&v3)[idx2] + 64)) = idx;
                    else
                        (&v1)[idx2].field_0 = idx;
                    v18 = (&v5)[idx2];
                    (&v3)[idx2] = idx;
                    v19 = v18->field_40;
                    *((unsigned long long *)(idx + 64)) = 0;
                    v20 = *((long long *)(idx + 24));
                    (&v5)[idx2] = v19;
                    v21 = v16 ^ 1;
                    if (v20 > v15)
                    {
                        v21 = v16;
                        v15 = v20;
                        v16 = v21;
                    }
                    idx2 = v21;
                    idx = (&v5)[idx2];
                } while (idx);
            }
            v9 = sub_404ae0(&v1, &v3, v13, v16, v9);
            if (!v9)
                sub_404210("internal error: screwup in format of diff blocks"); /* do not return */
            index = v5;
            *(v10) = v9;
            v10 = v9 + 13;
            v8 = a1;
            if (index)
                break;
LABEL_404fab:
            if (!v8)
                return v0;
            index = v8;
            idx1 = 1;
            idx2 = 0;
            v4 = 0;
            v3 = 0;
            v13 = 1;
            v2 = 0;
            v1 = (struct_1)0;
        }
    }
}



// Function: dotlines @ 0x5030
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[16];
    long long field_10;
} struct_0;

int dotlines(FILE *a0, unsigned long a1, int a2)
{
    unsigned long v2;  // r13
    struct_0 *v3;  // r14
    long long v4;  // rbx
    unsigned long long v5;  // rdx
    long long v6;  // rbx
    char v0;  // [bp-0x41]

    v2 = a2;
    v3 = v2 * 16 + a1;
    if (v3->field_10 >= *((long long *)&v3->padding_0[8]))
    {
        v0 = 0;
        v4 = 0;
        do
        {
            v5 = v4 * 8;
            if (*((char *)*((long long *)(*((long long *)(a1 + v2 * 8 + 56)) + v4 * 8))) == 46)
            {
                fputc_unlocked(46, a0);
                v0 = 1;
            }
        } while ((v6 = (long long)(v4 + 1), fwrite_unlocked(*((long long *)(*((long long *)(a1 + v2 * 8 + 56)) + v4 * 8)), 1, *((long long *)(*((long long *)(a1 + v2 * 8 + 80)) + v5)), a0), v4 = (long long)(v4 + 1), v3->field_10 - *((long long *)&v3->padding_0[8]) >= v4));
    }
    else
    {
        v0 = 0;
    }
    return v0;
}



// Function: output_diff3_edscript @ 0x50f0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[104];
    struct struct_0 *field_68;
} struct_0;

extern char g_4130ea;
extern char g_4130eb;
extern char g_4130ec;
extern char g_4130ed;
extern char g_4130ef;

unsigned int output_diff3_edscript(FILE *a0, struct_0 *a1, int *a2, unsigned long a3, char *a4, char *a5, char *a6)
{
    struct_0 *v1;  // rax
    unsigned int v2;  // r14d
    struct_0 *index;  // rax
    unsigned long long v12;  // rax
    unsigned long long v13;  // rax
    unsigned long v14;  // rax
    unsigned long v16;  // rcx
    char v17;  // r14b
    unsigned long v18;  // r8
    unsigned long long v19;  // rax
    struct_0 *idx;  // r15
    unsigned long long v20;  // rax
    unsigned long v21;  // rcx
    char v22;  // sil
    unsigned long long v23;  // rax
    unsigned int v4;  // eax
    struct_0 *idx1;  // rax
    unsigned long v6;  // r12
    unsigned long v7;  // r8
    unsigned int v8;  // r14d
    struct_0 *idx2;  // rax
    unsigned int v10;  // r10d
    unsigned int v0;  // [bp-0x4c]

    v1 = sub_404180(a1);
    if (v1)
    {
        v2 = 0;
        while (1)
        {
            idx = v1;
            if (*((int *)&idx->padding_0[0]) != 4)
            {
                v4 = *((int *)(a3 + (*((int *)&idx->padding_0[0]) - 5) * 4)) + 5;
                if (*((int *)(a3 + (*((int *)&idx->padding_0[0]) - 5) * 4)) != 1)
                {
                    if (v4 != 7)
                    {
                        if (v4 == 4)
                            goto LABEL_405167;
                    }
                    else
                    {
                        if (!g_4130ec)
                        {
                            idx1 = &idx->padding_0[16 * *(a2)];
                            v6 = *((long long *)&idx1->padding_0[8]);
                            v7 = *((long long *)&idx1->padding_0[16]);
LABEL_40518f:
                            v12 = a2[2] * 16;
                            if (*((long long *)&idx->padding_0[16 + v12]) - *((long long *)&idx->padding_0[8 + v12]) != 0xffffffffffffffff)
                            {
                                v13 = v7 - v6;
                                if (v13 == 0xffffffffffffffff)
                                {
                                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "%lda\n", v7);
                                }
                                else if (v13)
                                {
                                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "%ld,%ldc\n", v6, v7);
                                }
                                else
                                {
                                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "%ldc\n", v7);
                                }
                                v14 = a2[2];
                                v18 = *((long long *)&idx->padding_0[16 + 16 * v14]) - *((long long *)&idx->padding_0[8 + 16 * v14]) + 1;
                                sub_4046d0(a0, sub_405030(a0, idx, v14), v6, v18);
                                v1 = idx->field_68;
                                if (!idx->field_68)
                                    break;
                                else
                                    continue;
                            }
                            else
                            {
                                v16 = v6;
                                if (v6 != v7)
                                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "%ld,%ldd\n", v16, v7);
                                else
                                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "%ldd\n", v16);
                            }
                        }
                    }
                }
                else
                {
                    if (g_4130eb)
                    {
                        v8 = 6;
                        idx2 = &idx->padding_0[16 * *(a2)];
                        v6 = *((long long *)&idx2->padding_0[8]);
                        v18 = *((long long *)&idx2->padding_0[16]);
                        __fprintf_chk(*((unsigned int *)&a0), 0x1, "%lda\n", *((long long *)&idx2->padding_0[16]));
                        v10 = 0;
                        goto LABEL_4052b9;
                    }
                }
            }
            else
            {
LABEL_405167:
                if (!g_4130ed)
                {
                    index = &idx->padding_0[16 * *(a2)];
                    v6 = *((long long *)&index->padding_0[8]);
                    v7 = *((long long *)&index->padding_0[16]);
                    if (!g_4130ef)
                        goto LABEL_40518f;
                    v18 = v7;
                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "%lda\n", v7);
                    v17 = g_4130eb;
                    if (g_4130eb)
                    {
                        __fprintf_chk(*((unsigned int *)&a0), 0x1, "||||||| %s\n", a5);
                        v17 = sub_405030(a0, idx, a2[1]);
                    }
                    fputs_unlocked("=======\n", a0);
                    v10 = v17 | (char)sub_405030(a0, idx, a2[2]);
                    v8 = 4;
LABEL_4052b9:
                    v0 = v10;
                    __fprintf_chk(*((unsigned int *)&a0), 0x1, ">>>>>>> %s\n", a6);
                    v19 = a2[1] * 16;
                    v20 = a2[2] * 16;
                    sub_4046d0(a0, v0, v18 + 2, *((long long *)&idx->padding_0[16 + v19]) - *((long long *)&idx->padding_0[8 + v19]) + *((long long *)&idx->padding_0[16 + v20]) - *((long long *)&idx->padding_0[8 + v20]) + 3);
                    v21 = v6 - 1;
                    if (v8 != 4)
                    {
                        __fprintf_chk(*((unsigned int *)&a0), 0x1, "%lda\n<<<<<<< %s\n", v21, a5);
                        v22 = 0;
                        if (v8 == 6)
                        {
                            v22 = sub_405030(a0, idx, a2[1]);
                            fputs_unlocked("=======\n", a0);
                        }
                    }
                    else
                    {
                        __fprintf_chk(*((unsigned int *)&a0), 0x1, "%lda\n<<<<<<< %s\n", v21, a4);
                        v22 = 0;
                    }
                    v2 = 1;
                    v23 = a2[1] * 16;
                    sub_4046d0(a0, v22, v6 + 1, *((long long *)&idx->padding_0[16 + v23]) - *((long long *)&idx->padding_0[8 + v23]) + 1);
                }
            }
            v1 = idx->field_68;
            if (!idx->field_68)
                break;
        }
    }
    else
    {
        v2 = 0;
    }
    if (g_4130ea)
    {
        fputs_unlocked("w\nq\n", a0);
        return v2;
    }
    return v2;
}



// Function: output_diff3 @ 0x5520
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
} struct_1;

extern struct_1 g_40dd40;
extern char g_4130ee;

void output_diff3(FILE *a0, struct_0 *a1, unsigned long a2, unsigned long a3)
{
    char *v10;  // rax
    unsigned long idx;  // rax
    unsigned long v20;  // r14
    long long v21;  // r12
    unsigned int v12;  // ecx
    int i;  // r15d
    struct_0 *v14;  // r14
    unsigned long v15;  // rbp
    struct_0 *idx1;  // rax
    unsigned long v17;  // rbx
    unsigned long v18;  // rax
    long long j;  // r12
    long long v0;  // [bp-0x88]
    unsigned long v1;  // [bp-0x80]
    int v2;  // [bp-0x74]
    char *v3;  // [bp-0x70]
    unsigned long index;  // [bp-0x68]
    unsigned int v5;  // [bp-0x58]
    int v6;  // [bp-0x54]
    char v7;  // [bp-0x42]
    char v8;  // [bp-0x41]

    v10 = "  ";
    if (g_4130ee)
        v10 = "\t";
    v3 = v10;
    if (!a1)
        return;
    while (1)
    {
        if (a1->field_0 == 4)
        {
            v7 = 0;
            v6 = 3;
            v5 = 3;
        }
        else
        {
            idx = a1->field_0 - 5;
            if ((unsigned int)idx > 2)
                sub_404210("internal error: invalid diff type passed to output"); /* do not return */
            v8 = 0;
            v12 = *((int *)(a3 + idx * 4));
            v5 = v12;
            v7 = (char)v12 + 49;
            v6 = !v12;
        }
        i = 0;
        __fprintf_chk(*((unsigned int *)&a0), 0x1, "====%s\n", &v7);
        v14 = a1;
        do
        {
            index = i;
            v15 = *((int *)(a2 + index * 4));
            idx1 = &(&v14->field_0)[4 * v15];
            v17 = idx1->field_10;
            v2 = i + 1;
            __fprintf_chk(*((unsigned int *)&a0), 0x1, "%d:", v2);
            v18 = idx1->field_8;
            if (v18 == v17)
            {
                __fprintf_chk(*((unsigned int *)&a0), 0x1, "%ldc\n", v18);
            }
            else if (v18 - v17 == 1)
            {
                __fprintf_chk(*((unsigned int *)&a0), 0x1, "%lda\n", v18 - 1);
            }
            else
            {
                __fprintf_chk(*((unsigned int *)&a0), 0x1, "%ld,%ldc\n", v18, v17);
            }
            if (v18 <= v17 && v6 != i)
            {
                j = 0;
                v0 = v17 - v18;
                v1 = v15 + 6;
                v20 = v15;
                do
                {
                    v21 = j;
                    fputs_unlocked(v3, a0);
                    j = v21 + 1;
                    fwrite_unlocked(*((long long *)((&v14->field_8)[v1] + v21 * 8)), 1, *((long long *)((&v14[3].field_8)[v20] + v21 * 8)), a0);
                } while (j <= v0);
                if (*((char *)(*((long long *)((&v14->field_8)[v1] + v21 * 8)) + *((long long *)((&v14[3].field_8)[v20] + v21 * 8)) - 1)) != 10)
                    __fprintf_chk(*((unsigned int *)&a0), 0x1, "\n\\ %s\n", dcgettext(NULL, "No newline at end of file", 5));
            }
            if (v5 == 1)
                v2 = *((int *)((char *)&g_40dd40.field_0 + 4 * index));
            i = v2;
        } while (i <= 2);
        a1 = v14[4].field_8;
        if (!v14[4].field_8)
            return;
    }
}



// Function: output_diff3_merge @ 0x57d0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned long long field_10;
    char padding_18[80];
    struct struct_0 *field_68;
} struct_0;

extern char g_4130eb;
extern char g_4130ec;
extern char g_4130ed;
extern char g_4130ef;

int output_diff3_merge(FILE *a0, FILE *a1, struct_0 *a2, int *a3, unsigned long a4, char *a5, unsigned long a6, char *a7)
{
    struct_0 *iter;  // r15
    unsigned int v6;  // eax
    unsigned long v16;  // rax
    long long i;  // r12
    unsigned long long node;  // r14
    unsigned int v20;  // eax
    unsigned int v21;  // eax
    unsigned long long v22;  // rax
    unsigned long long v23;  // rax
    unsigned int v24;  // eax
    unsigned long long v7;  // r12
    unsigned int v26;  // eax
    unsigned int v27;  // eax
    unsigned long long v8;  // rax
    unsigned long long iter1;  // r12
    long long v10;  // r12
    long long v11;  // r12
    unsigned long v12;  // rdx
    unsigned long v13;  // rax
    long long j;  // r12
    char *v0;  // [bp-0x60]
    unsigned long v1;  // [bp-0x58]
    unsigned int v2;  // [bp-0x50]
    char v3;  // [bp-0x4a]
    char v4;  // [bp-0x49]

    if (a2)
    {
        iter = a2;
        v4 = 0;
        v6 = iter->field_0;
        v7 = 0;
        if (iter->field_0 == 4)
            goto LABEL_40584a;
        else
            goto LABEL_405810;
        while (1)
        {
            iter = iter->field_68;
            if (!iter)
                break;
            v6 = iter->field_0;
            if (iter->field_0 != 4)
            {
LABEL_405810:
                v21 = *((int *)(a4 + (v6 - 5) * 4)) + 5;
                if (*((int *)(a4 + (v6 - 5) * 4)) == 1)
                {
                    v3 = g_4130eb;
                    if (!g_4130eb)
                        continue;
                    v2 = 6;
                    v23 = iter->field_8 - v7;
                    v1 = iter->field_8 - 1;
                    v0 = "<<<<<<< %s\n";
                    iter1 = v23 - 2;
                    if (v23 - 2 >> 63 & 1)
                        goto LABEL_405aef;
                }
                else if (v21 != 7)
                {
                    if (v21 != 4)
                        continue;
                    goto LABEL_40584a;
                }
                else
                {
                    v3 = g_4130ec;
                    if (g_4130ec)
                        continue;
                    v2 = 7;
                    v22 = iter->field_8 - v7;
                    v1 = iter->field_8 - 1;
                    v0 = "<<<<<<< %s\n";
                    iter1 = v22 - 2;
                    if (v22 - 2 >> 63 & 1)
                        goto LABEL_4058fa;
                }
                do
                {
                    while (1)
                    {
LABEL_405898:
                        v24 = getc_unlocked(a0);
                        if (v24 != 0xffffffff)
                        {
                            putc_unlocked(v24, a1);
                            if (v24 == 10)
                                break;
                        }
                        else if (ferror_unlocked(a0))
                        {
                            sub_4043a0(dcgettext(NULL, "read failed", 5)); /* do not return */
                        }
                        else if (!feof_unlocked(a0))
                        {
                            putc_unlocked(-0x1, a1);
                        }
                        else
                        {
                            sub_404210("input file shrank"); /* do not return */
                        }
                    }
                } while ((iter1 -= 1, iter1 >= 1));
                if (!v3)
                    goto LABEL_4058fa;
                if (v2 != 4)
                    goto LABEL_4058d4;
                goto LABEL_405b9f;
            }
            else
            {
LABEL_40584a:
                if (g_4130ed)
                    continue;
                v3 = g_4130ef;
                v8 = iter->field_8 - v7;
                v1 = iter->field_8 - 1;
                iter1 = v8 - 2;
                if (!(v8 - 2 >> 63 & 1))
                {
                    v2 = 4;
                    v0 = "||||||| %s\n";
                    goto LABEL_405898;
                }
                else if (!v3)
                {
LABEL_4058fa:
                    v16 = a3[2];
                    if ((&iter->field_10)[2 * v16] >= (&iter->field_8)[2 * v16])
                    {
                        i = 0;
                        do
                        {
                            fwrite_unlocked(*((long long *)(*((long long *)&iter->padding_18[32 + 8 * v16]) + i * 8)), 1, *((long long *)(*((long long *)&iter->padding_18[56 + 8 * v16]) + i * 8)), a1);
                            v16 = a3[2];
                            i += 1;
                        } while ((&iter->field_10)[2 * v16] - (&iter->field_8)[2 * v16] >= i);
                    }
                    if (v3)
                        __fprintf_chk(*((unsigned int *)&a1), 0x1, ">>>>>>> %s\n", a7);
                    node = iter->field_10 - iter->field_8;
                    v7 = v1 + node + 1;
                    if (iter->field_10 - iter->field_8 >> 63 & 1)
                        continue;
                    while (1)
                    {
                        v20 = getc_unlocked(a0);
                        if (v20 == 10)
                        {
                            node -= 1;
                            if (node < 1)
                                break;
                        }
                        else if (v20 == 0xffffffff)
                        {
                            if (ferror_unlocked(a0))
                            {
                                sub_4043a0(dcgettext(NULL, "read failed", 5)); /* do not return */
                            }
                            else if (feof_unlocked(a0))
                            {
                                if (node)
                                {
                                    sub_404210("input file shrank"); /* do not return */
                                }
                                else if (!iter->field_68)
                                {
                                    return v4;
                                }
                                else
                                {
                                    sub_404210("input file shrank"); /* do not return */
                                }
                            }
                        }
                    }
                }
                else
                {
                    v0 = "||||||| %s\n";
LABEL_405b9f:
                    v10 = 0;
                    __fprintf_chk(*((unsigned int *)&a1), 0x1, "<<<<<<< %s\n", a5);
                    while (1)
                    {
                        v11 = v10;
                        v12 = *(a3);
                        if ((&iter->field_10)[2 * v12] - (&iter->field_8)[2 * v12] < v11)
                            break;
                        v10 = v11 + 1;
                        fwrite_unlocked(*((long long *)(*((long long *)&iter->padding_18[32 + 8 * v12]) + v11 * 8)), 1, *((long long *)(*((long long *)&iter->padding_18[56 + 8 * v12]) + v11 * 8)), a1);
                    }
LABEL_4058d4:
                    if (!g_4130eb)
                        goto LABEL_4058e1;
LABEL_405aef:
                    __fprintf_chk(*((unsigned int *)&a1), 0x1, v0);
                    v13 = a3[1];
                    if ((&iter->field_10)[2 * v13] >= (&iter->field_8)[2 * v13])
                    {
                        j = 0;
                        do
                        {
                            fwrite_unlocked(*((long long *)(*((long long *)&iter->padding_18[32 + 8 * v13]) + j * 8)), 1, *((long long *)(*((long long *)&iter->padding_18[56 + 8 * v13]) + j * 8)), a1);
                            v13 = a3[1];
                            j += 1;
                        } while ((&iter->field_10)[2 * v13] - (&iter->field_8)[2 * v13] >= j);
                    }
LABEL_4058e1:
                    fputs_unlocked("=======\n", a1);
                    v3 = 1;
                    v4 = 1;
                    goto LABEL_4058fa;
                }
            }
        }
    }
    else
    {
        v4 = 0;
    }
    while (1)
    {
        v26 = getc_unlocked(a0);
        if (v26 == 0xffffffff && !(v27 = (unsigned int)(int)(long long)ferror_unlocked(a0), !(int)(long long)ferror_unlocked(a0) && !(int)(long long)feof_unlocked(a0)))
            break;
        putc_unlocked(v26, a1);
    }
    return v4;
}



// Function: check_stdout @ 0x5c30
typedef struct FILE {
} FILE;

extern FILE *stdout;

int check_stdout(void)
{
    unsigned int v1;  // eax

    if (ferror_unlocked(stdout))
        sub_404210("write failed"); /* do not return */
    v1 = fclose(stdout);
    if (v1)
        sub_4043a0(dcgettext(NULL, "standard output", 5)); /* do not return */
    return v1;
}



// Function: process_diff @ 0x5d70
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern FILE *stderr;

long long process_diff(unsigned int a0, unsigned int a1, struct_0 **a2, struct_2 **a3)
{
    struct_0 *v8;  // r13
    char *v9;  // rbp
    unsigned long long v18;  // r12
    unsigned long long v19;  // rax
    char *v20;  // rdi
    unsigned long long v21;  // r12
    char *v22;  // rax
    struct_0 *v10;  // rax
    struct_0 *idx;  // r15
    unsigned int v12;  // ebx
    unsigned long long v13;  // r12
    unsigned long long v14;  // r14
    unsigned long long v15;  // rax
    unsigned long long v16;  // r14
    unsigned long long v17;  // rbx
    char *v0;  // [bp-0x78]
    unsigned long long v1;  // [bp-0x70]
    char *v2;  // [bp-0x68]
    char *v3;  // [bp-0x58]
    char *v4;  // [bp-0x50]
    char *iter;  // [bp-0x50]
    char v6;  // [bp-0x48]

    v8 = &v6;
    v9 = sub_4043d0(a0, a1, &v3);
    *(a3) = v3;
    iter = v3;
    if (v3 < v9)
    {
        v0 = &iter;
        while (1)
        {
            v10 = sub_408180(72);
            v10->field_20 = 0;
            idx = v10;
            idx->field_18 = 0;
            idx->field_30 = 0;
            idx->field_28 = 0;
            v12 = sub_405c80(v0, idx);
            if (!v12 || *(iter) != 10)
            {
                v22 = dcgettext(NULL, "%s: diff failed: ", 5);
                __fprintf_chk(*((unsigned int *)&stderr), 0x1, v22);
                do
                {
                    v4 = iter;
                    putc_unlocked(*(v4), stderr);
                    iter = v4 + 1;
                } while (*(v4) != 10);
                exit(2); /* do not return */
            }
            iter += 1;
            if (v12 != 2)
            {
                if (v12 == 3)
                {
                    idx->field_8 = idx->field_8 + 1;
                }
                else
                {
                    if (v12 != 1)
                        sub_404210("internal error: invalid diff type in process_diff"); /* do not return */
                    idx->field_0 = (char *)&idx->field_0->field_0 + 1;
                    goto LABEL_405f0b;
                }
            }
            v13 = *((long long *)&idx->field_1) - (char *)idx->field_0;
            v14 = v13 + 1;
            if (v14 > 0xffffffffffffffe)
                sub_408530(); /* do not return */
            v2 = iter;
            v1 = v14 * 8;
            idx->field_18 = sub_408180(v1);
            v15 = sub_408180(v1);
            idx->field_28 = v15;
            if (v14)
            {
                v16 = 0;
                while (1)
                {
                    iter = &sub_404770(v2, v16 * 8 + idx->field_18, v15 + v16 * 8, v9, 60)->field_0;
                    v2 = iter;
                    if (v13 == v16)
                        break;
                    v15 = idx->field_28;
                    v16 += 1;
                }
            }
            if (v12 == 2)
            {
                if (!strncmp(iter, "---\n", 4))
                {
                    iter += 4;
                    goto LABEL_405f0b;
                }
                else
                {
                    sub_404210("invalid diff format; invalid change separator"); /* do not return */
                }
            }
            else
            {
                if (v12 != 3)
                {
LABEL_405f0b:
                    v17 = idx->field_10 - idx->field_8;
                    v18 = v17 + 1;
                    if (v18 > 0xffffffffffffffe)
                        sub_408530(); /* do not return */
                    v1 = v18 * 8;
                    idx->field_20 = sub_408180(v1);
                    v19 = sub_408180(v1);
                    v20 = iter;
                    idx->field_30 = v19;
                    if (v18)
                    {
                        v21 = 0;
                        while (1)
                        {
                            iter = &sub_404770(v20, v21 * 8 + idx->field_20, v19 + v21 * 8, v9, 62)->field_0;
                            v20 = iter;
                            if (v17 == v21)
                                break;
                            v19 = idx->field_30;
                            v21 += 1;
                        }
                    }
                    v8->field_0 = idx;
                    v8 = &idx->field_38;
                    if (v9 <= v20)
                        break;
                }
                else
                {
                    v8->field_0 = idx;
                    v8 = &idx->field_38;
                    if (v9 <= iter)
                        break;
                }
            }
        }
    }
    v8->field_0 = NULL;
    *(a2) = idx;
    return v6;
}



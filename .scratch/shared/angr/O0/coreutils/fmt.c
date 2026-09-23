// Function: usage @ 0x2aa2
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
    char *v14;  // rax
    char *v15;  // rax
    char *v5;  // rax
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
        printf(gettext("Usage: %s [-WIDTH] [OPTION]... [FILE]...\n"));
        v5 = gettext("Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n");
        fputs_unlocked(v5, stdout);
        sub_4027a9(v6, v7, v8, v9, v10, v11);
        sub_4027da(v6, v7, v8, v9, v10, v11);
        v12 = gettext("  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n");
        fputs_unlocked(v12, stdout);
        v13 = gettext("  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n");
        fputs_unlocked(v13, stdout);
        v14 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v14, stdout);
        v15 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v15, stdout);
        sub_40280b("fmt");
    }
    exit(a0); /* do not return */
}



// Function: main @ 0x2bde
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_4091c3;
extern option g_40ca80;
extern long long g_40d010;
extern long long stdout;
extern long long stdin;
extern unsigned int optind;
extern long long optarg;
extern char g_40d100;
extern char g_40d101;
extern char g_40d102;
extern char g_40d103;
extern unsigned long long g_40d108;
extern int g_40d110;
extern unsigned int g_40d114;
extern unsigned int g_40d118;
extern unsigned int g_40d11c;
extern unsigned int g_40d120;

int main(int a0, struct_0 **a1)
{
    unsigned long v11;  // rax
    char *v12;  // rax
    int *err;  // rax
    int *err1;  // rax
    struct_0 **iter;  // [bp-0x58]
    int v1;  // [bp-0x4c]
    char v2;  // [bp-0x3e]
    char i;  // [bp-0x3d]
    int v4;  // [bp-0x3c]
    long long v5;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    unsigned long v7;  // [bp-0x28]
    unsigned long fp;  // [bp-0x20]

    v1 = a0;
    iter = a1;
    v2 = 1;
    v5 = 0;
    v6 = 0;
    sub_40498f(*(iter));
    setlocale(6, &g_4091c3);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_408540(sub_404625);
    g_40d103 = 0;
    g_40d102 = g_40d103;
    g_40d101 = g_40d102;
    g_40d100 = g_40d101;
    g_40d110 = 75;
    g_40d108 = &g_4091c3;
    g_40d114 = 0;
    g_40d118 = g_40d114;
    g_40d11c = g_40d118;
    if (v1 > 1 && iter[1]->field_0 == 45 && iter[1]->field_1 - 48 <= 9)
    {
        v5 = &iter[1]->field_1;
        iter[1] = *(iter);
        iter += 1;
        v1 -= 1;
    }
    while (1)
    {
        v4 = getopt_long(v1, iter, "0123456789cstuw:p:g:", &g_40ca80.name, NULL);
        if (v4 == -0x1)
            break;
        if (v4 > 0x77)
            goto LABEL_402d94;
        if (v4 < 99)
        {
            if (v4 != -0x83)
            {
                if (v4 != -0x82)
                    goto LABEL_402d94;
                sub_402aa2(0); /* do not return */
            }
            sub_407168(stdout, "fmt", "GNU coreutils", g_40d010, "Ross Paterson", 0);
            exit(0); /* do not return */
        }
        switch (v4)
        {
        case 99:
            g_40d100 = 1;
            break;
        case 103:
            v6 = optarg;
            break;
        case 112:
            sub_40312d(optarg);
            break;
        case 115:
            g_40d102 = 1;
            break;
        case 116:
            g_40d101 = 1;
            break;
        case 117:
            g_40d103 = 1;
            break;
        case 119:
            v5 = optarg;
            break;
        default:
LABEL_402d94:
            if (v4 - 48 <= 9)
                error(0, 0, gettext("invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead"));
            sub_402aa2(1); /* do not return */
        }
    }
    if (v5)
        g_40d110 = sub_407afa(v5, 0, 2500, &g_4091c3, gettext("invalid width"), 0);
    if (v6)
    {
        v11 = gettext("invalid width");
        g_40d120 = sub_407afa(v6, 0, g_40d110, &g_4091c3, v11, 0);
        if (!v5)
            g_40d110 = g_40d120 + 10;
    }
    else
    {
        g_40d120 = g_40d110 * 0xbb / 200;
    }
    if (v1 == optind)
    {
        i = 1;
        v2 = sub_4031d1(stdin, "-", "-");
    }
    else
    {
        for (i = 0; v1 > optind; optind = optind + 1)
        {
            v7 = iter[optind];
            if (!strcmp(v7, "-"))
            {
                v2 = (char)sub_4031d1(stdin, v7, v7) & v2;
                i = 1;
            }
            else
            {
                fp = fopen(v7, "r");
                if (fp)
                {
                    v2 = (char)sub_4031d1(fp, v7, v7) & v2;
                }
                else
                {
                    sub_406589(4, v7);
                    v12 = gettext("cannot open %s for reading");
                    err = __errno_location();
                    error(0, *(err), v12);
                    v2 = 0;
                }
            }
        }
    }
    if (i && sub_404791(stdin))
    {
        gettext("closing standard input");
        err1 = __errno_location();
        error(1, *(err1), "%s");
    }
    return v2 ^ 1;
}



// Function: set_prefix @ 0x312d
extern unsigned long long g_40d108;
extern int g_40d114;
extern unsigned int g_40d118;
extern unsigned int g_40d11c;

void* set_prefix(void* a0)
{
    void* i;  // [bp-0x20]
    void* iter;  // [bp-0x10]

    i = a0;
    for (g_40d118 = 0; *((char *)i) == 32; i += 1)
    {
        g_40d118 = g_40d118 + 1;
    }
    g_40d108 = i;
    g_40d114 = strlen(i);
    for (iter = i + g_40d114; iter > i && *((char *)iter - 1) == 32; iter -= 1);
    *((char *)iter) = 0;
    g_40d11c = iter - i;
    return iter - i;
}



// Function: fmt @ 0x31d1
extern char stdin;
extern long long g_418120;
extern char g_418128;
extern unsigned int g_418134;
extern unsigned int g_418138;

unsigned int fmt(unsigned long a0, long long a1)
{
    unsigned int v0;  // [bp-0x1c]

    sub_404752(a0, 2);
    g_418128 = 0;
    g_418134 = 0;
    g_418138 = sub_403978(a0);
    while ((char)sub_4033b7(a0))
    {
        sub_403db9();
        sub_40411d(g_418120);
    }
    v0 = (!ferror_unlocked(a0) ? 0xffffffff : 0);
    if (a0 == *((long long *)&stdin))
    {
        clearerr_unlocked(a0);
    }
    else if (sub_404791(a0) && v0 < 0)
    {
        v0 = *(__errno_location());
    }
    if (v0 >= 0)
    {
        sub_40671b(0, 3, a1);
        error(0, v0, (!v0 ? gettext("read error") : "%s"));
    }
    return v0 >> 31;
}



// Function: set_other_indent @ 0x32fb
extern char g_40d100;
extern char g_40d101;
extern char g_40d102;
extern unsigned int g_40d124;
extern unsigned int g_418130;
extern unsigned int g_418134;

long long set_other_indent(char a0)
{
    if (g_40d102)
    {
        g_418134 = g_418130;
        return g_418130;
    }
    else if (g_40d100)
    {
        g_418134 = (!a0 ? g_418130 : g_40d124);
        return g_418134;
    }
    else if (g_40d101)
    {
        if (a0 && g_40d124 != g_418130)
        {
            g_418134 = g_40d124;
            return g_40d124;
        }
        if (g_418134 != g_418130)
            return g_418130;
        g_418134 = (!g_418130 ? 3 : 0);
        return g_418134;
    }
    else
    {
        g_418134 = g_418130;
        return g_418130;
    }
}



// Function: get_paragraph @ 0x33b7
extern char g_40d100;
extern char g_40d101;
extern char g_40d102;
extern unsigned int g_40d114;
extern char g_40d118;
extern char g_40d124;
extern char g_40d140;
extern unsigned long long g_40e4c8;
extern void g_40e4e0;
extern void* g_418120;
extern unsigned int g_41812c;
extern unsigned int g_418130;
extern char g_418134;
extern unsigned int g_418138;
extern int g_41813c;
extern unsigned int g_418140;

unsigned int get_paragraph(long long a0)
{
    unsigned long idx;  // rax
    unsigned long index;  // rdx
    unsigned int i;  // [bp-0xc]

    g_418140 = 0;
    for (i = g_418138; i == 10 || i == 0xffffffff || g_41813c < *((int *)&g_40d118) || g_41813c + g_40d114 > *((int *)&g_40d124); i = sub_403978(a0))
    {
        i = sub_403630(a0, i, i);
        if (i == 0xffffffff)
        {
            g_418138 = 0xffffffff;
            return 0;
        }
        putchar_unlocked(10);
    }
    g_41812c = g_41813c;
    g_418130 = *((int *)&g_40d124);
    g_40e4c8 = &g_40d140;
    g_418120 = &g_40e4e0;
    i = sub_403793(a0, i, i);
    sub_4032fb(sub_403743(i));
    if (!g_40d102)
    {
        if (g_40d100)
        {
            if ((char)sub_403743(i))
            {
                do
                {
                    i = sub_403793(a0, i, i);
                } while ((char)sub_403743(i) && *((int *)&g_40d124) == *((int *)&g_418134));
            }
        }
        else
        {
            if (!g_40d101)
            {
                for (; (char)sub_403743(i) && *((int *)&g_40d124) == *((int *)&g_418134); i = sub_403793(a0, i, i));
            }
            else if ((char)sub_403743(i) && *((int *)&g_40d124) != g_418130)
            {
                do
                {
                    i = sub_403793(a0, i, i);
                } while ((char)sub_403743(i) && *((int *)&g_40d124) == *((int *)&g_418134));
            }
        }
    }
    if (g_418120 > &g_40e4e0)
    {
        idx = g_418120 - 40;
        *((char *)(idx + 16)) = *((char *)(idx + 16)) | 8;
        index = g_418120 - 40;
        *((char *)(index + 16)) = *((char *)(index + 16)) & 0xfffffffd | (*((char *)(idx + 16)) >> 3 & 1) * 2;
        g_418138 = i;
        return 1;
    }
    __assert_fail(); /* do not return */
}



// Function: copy_rest @ 0x3630
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern struct_0 *g_40d108;
extern unsigned int g_40d11c;
extern char g_40d124;
extern void g_40d128;
extern char g_41813c;

int copy_rest(FILE *a0, int a1)
{
    int j;  // [bp-0x24]
    char *i;  // [bp-0x10]
    char *v2;  // [bp-0x10]

    j = a1;
    *((unsigned int *)&g_40d128) = 0;
    if (*((int *)&g_40d124) > *((int *)&g_41813c) || j != 10 && j != -0x1)
    {
        sub_4042aa(*((int *)&g_41813c));
        for (i = &g_40d108->field_0; *((int *)&g_40d128) != *((int *)&g_40d124) && *(i); i = v2)
        {
            v2 = i + 1;
            putchar_unlocked(*(i));
            *((unsigned int *)&g_40d128) = *((int *)&g_40d128) + 1;
        }
        if (j != -0x1 && j != 10)
            sub_4042aa(*((int *)&g_40d124) - *((int *)&g_40d128) & 0xffffffff);
        if (j == -0x1 && *((int *)&g_41813c) + g_40d11c <= *((int *)&g_40d124))
            putchar_unlocked(10);
    }
    for (; j != 10; j = getc_unlocked(a0))
    {
        if (j == -0x1)
            return j;
        putchar_unlocked(j);
    }
    return j;
}



// Function: same_para @ 0x3743
extern unsigned int g_40d114;
extern char g_40d124;
extern char g_41812c;
extern unsigned int g_41813c;

unsigned int same_para(unsigned int a0)
{
    unsigned int v1;  // eax

    if (g_41813c == *((int *)&g_41812c) && g_41813c + g_40d114 <= *((int *)&g_40d124) && a0 != 10 && a0 != 0xffffffff)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: get_line @ 0x3793
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_3 {
    struct struct_0 *field_0;
} struct_3;

typedef struct struct_2 {
    struct struct_3 *field_0;
    unsigned int field_8;
    int field_c;
    char field_10;
    char padding_11[23];
    char field_28;
} struct_2;

extern char g_40d103;
extern unsigned int g_40d124;
extern struct_0 *g_40e4c8;
extern long long g_4180d0;
extern struct_2 *g_418120;

long long get_line(unsigned long a0, unsigned int a1)
{
    long long v5;  // rdi
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    char *v11;  // rax
    char v12;  // al
    unsigned int v0;  // [bp-0x34]
    unsigned int v1;  // [bp-0x1c]
    struct_0 **v2;  // [bp-0x18]
    long long v3;  // [bp-0x10]

    v0 = a1;
    v2 = &g_40e4c8;
    v3 = &g_4180d0;
    do
    {
        g_418120->field_0 = g_40e4c8;
        do
        {
            if (v2 == g_40e4c8)
            {
                sub_4032fb(1);
                sub_403bd2(v5, v6, v7, v8, v9, v10);
            }
        } while ((v11 = (char *)g_40e4c8, g_40e4c8 = (struct_0 *)(v11 + 1), *(v11) = (char)v0, v0 = (unsigned int)(int)(long long)getc_unlocked(a0), v0 != 0xffffffff && (char)(unsigned long long)sub_404512((unsigned long long)v0) != 1));
        g_418120->field_8 = g_40e4c8 - g_418120->field_0;
        g_40d124 = g_40d124 + g_418120->field_8;
        sub_403aba(g_418120);
        v1 = g_40d124;
        v0 = sub_403a4c(a0, v0, v0);
        g_418120->field_c = g_40d124 - v1;
        if (v0 == 0xffffffff || g_418120->field_10 & 2 && (v0 == 10 || g_418120->field_c > 1))
            v12 = 1;
        else
            v12 = 0;
        g_418120->field_10 = g_418120->field_10 & 247 | (v12 & 1) * 8;
        if (v0 == 10 || v0 == 0xffffffff || g_40d103)
            g_418120->field_c = (!(g_418120->field_10 & 8) ? 1 : 2);
        if (v3 == g_418120)
        {
            sub_4032fb(1);
            sub_403bd2(v5, v6, v7, v8, v9, v10);
        }
        g_418120 = &g_418120->field_28;
    } while (v0 != 10 && v0 != 0xffffffff);
    return sub_403978(a0);
}



// Function: get_prefix @ 0x3978
typedef struct struct_0 {
    char field_0;
} struct_0;

extern struct_0 *g_40d108;
extern char g_40d118;
extern unsigned int g_40d11c;
extern int g_40d124;
extern unsigned int g_41813c;

int get_prefix(unsigned long a0)
{
    unsigned int v4;  // eax
    char v0;  // [bp-0x15]
    unsigned int v1;  // [bp-0x14]
    char *iter;  // [bp-0x10]

    g_40d124 = 0;
    v4 = getc_unlocked(a0);
    v1 = sub_403a4c(a0, v4, v4);
    if (!g_40d11c)
    {
        g_41813c = (g_40d124 <= *((int *)&g_40d118) ? g_40d124 : *((int *)&g_40d118));
    }
    else
    {
        g_41813c = g_40d124;
        for (iter = &g_40d108->field_0; *(iter); iter += 1)
        {
            v0 = *(iter);
            if (v1 != v0)
                return v1;
            g_40d124 = g_40d124 + 1;
            v1 = getc_unlocked(a0);
        }
        v1 = sub_403a4c(a0, v1, v1);
    }
    return v1;
}



// Function: get_space @ 0x3a4c
typedef struct FILE {
} FILE;

extern int g_40d124;
extern char g_418128;

unsigned int get_space(FILE *a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x14]

    v0 = a1;
    while (1)
    {
        switch (v0)
        {
        case 32:
            g_40d124 = g_40d124 + 1;
            break;
        case 9:
            g_418128 = 1;
            g_40d124 = (((int)((g_40d124 < 0 ? g_40d124 + 7 : g_40d124)) >> 3) + 1) * 8;
            break;
        default:
            return v0;
        }
        v0 = getc_unlocked(a0);
    }
}



// Function: check_punctuation @ 0x3aba
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

struct_0 * check_punctuation(struct_0 *idx)
{
    unsigned long v4;  // rcx
    long long v5;  // rdx
    long long v6;  // rdi
    long long v7;  // rsi
    long long v8;  // r8
    long long v9;  // r9
    unsigned long long *v10;  // rax
    char *ptr;  // rax
    char v0;  // [bp-0x19]
    char *iter;  // [bp-0x18]
    char *v2;  // [bp-0x10]

    v2 = &idx->field_0->field_0;
    iter = v2 + idx->field_8 - 1;
    v0 = *(iter);
    v4 = _INSERT(0, 0, strchr("([\'`\"", *(v2))) & 1;
    v5 = idx->field_10 & 0xfffffffe | (unsigned int)v4;
    idx->field_10 = v5;
    v10 = __ctype_b_loc(v6, v7, v5, v4, v8, v9);
    for (idx->field_10 = idx->field_10 & 0xfffffffb | (*((short *)(v0 * 2 + *(v10))) & 4) * 4; v2 < iter && strchr(")]\'\"", *(iter)); iter -= 1);
    ptr = strchr(".?!", *(iter));
    idx->field_10 = idx->field_10 & 0xfffffffd | (ptr) * 2;
    return idx;
}



// Function: flush_paragraph @ 0x3bd2
typedef struct struct_3 {
    char padding_0[24];
    unsigned long long field_18;
    struct struct_1 *field_20;
} struct_3;

typedef struct struct_0 {
    void* field_0;
    char padding_8[16];
    unsigned long long field_18;
    char padding_20[8];
    char field_28;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[16];
    unsigned long long field_18;
    char padding_20[8];
    char field_28;
} struct_1;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern struct_1 g_40d140;
extern unsigned long long g_40e4c8;
extern struct_0 g_40e4e0;
extern struct_3 *g_40e500;
extern struct_0 *g_418120;

struct_1 * flush_paragraph(void)
{
    unsigned int v0;  // [bp-0x24]
    struct_0 *ptr;  // [bp-0x20]
    struct_0 *iter;  // [bp-0x18], Other Possible Types: struct_3 *
    long long v3;  // [bp-0x10]

    if (g_418120 == &g_40e4e0.field_0)
    {
        fwrite_unlocked(&g_40d140.field_0, 1, g_40e4c8 - (char *)&g_40d140.field_0, stdout);
        g_40e4c8 = &g_40d140.field_0;
        return &g_40d140.field_0;
    }
    sub_403db9();
    ptr = g_418120;
    v3 = 0x7fffffffffffffff;
    for (iter = g_40e500; iter != g_418120; iter = iter->field_20)
    {
        if (v3 > iter->field_18 - iter->field_20->field_18)
        {
            ptr = iter;
            v3 = iter->field_18 - iter->field_20->field_18;
        }
        if (v3 <= 0x7ffffffffffffff6)
            v3 += 9;
    }
    sub_40411d(ptr);
    memmove(&g_40d140.field_0, ptr->field_0, g_40e4c8 - ptr->field_0);
    v0 = ptr->field_0 - &g_40d140.field_0;
    g_40e4c8 = -(v0) + g_40e4c8;
    for (iter = ptr; iter <= g_418120; iter = &iter->field_28)
    {
        iter->field_0 = iter->field_0 + -(v0);
    }
    memmove(&g_40e4e0.field_0, ptr, g_418120 - ptr + 40);
    g_418120 = &g_418120[-(ptr - &g_40e4e0.field_0)];
    return g_418120;
}



// Function: fmt_paragraph @ 0x3db9
extern void g_40d110;
extern void g_40e4e0;
extern void* g_418120;
extern unsigned int g_418130;
extern unsigned int g_418134;
extern int g_418140;

void* fmt_paragraph(void)
{
    unsigned long long v7;  // rax
    unsigned long long v8;  // rax
    unsigned int iter;  // [bp-0x30]
    unsigned int v1;  // [bp-0x2c]
    void* node;  // [bp-0x28]
    void* iter1;  // [bp-0x20]
    long long v4;  // [bp-0x18]
    long long v5;  // [bp-0x10]

    *((unsigned long long *)&g_418120[24]) = 0;
    v1 = (int)g_418120[8];
    *((int *)&g_418120[8]) = *((int *)&g_40d110);
    for (node = g_418120 - 40; node >= &g_40e4e0; node -= 40)
    {
        v5 = 0x7fffffffffffffff;
        iter = (node == &g_40e4e0 ? g_418130 : g_418134);
        iter1 = node;
        iter += (int)iter1[8];
        do
        {
            iter1 += 40;
            v7 = sub_40406b(iter1, iter, iter);
            v4 = (long long)iter1[24] + v7;
            if (node == &g_40e4e0 && g_418140 > 0)
            {
                v8 = (iter - g_418140) * 10 * (iter - g_418140) * 10;
                v4 += (long long)(v8 + (v8 >> 63)) >> 1;
            }
            if (v4 < v5)
            {
                v5 = v4;
                *((void* *)&node[32]) = iter1;
                *((unsigned int *)&node[20]) = iter;
            }
        } while (iter1 != g_418120 && (iter += (int)iter1[8] + *((int *)((char *)iter1 - 28)), iter < *((int *)&g_40d110)));
        *((unsigned long long *)&node[24]) = v5 + sub_403f67(node);
    }
    *((unsigned int *)&g_418120[8]) = v1;
    return g_418120;
}



// Function: base_cost @ 0x3f67
extern void g_40e4e0;
extern void g_40e508;

unsigned long long base_cost(void* a0)
{
    unsigned long long iter;  // [bp-0x10]

    iter = 4900;
    if (a0 > &g_40e4e0)
    {
        if (*((char *)a0 - 24) & 2)
        {
            iter = (!(*((char *)a0 - 24) & 8) ? iter + 360000 : iter - 2500);
        }
        else if (*((char *)a0 - 24) & 4)
        {
            iter -= 1600;
        }
        else if (a0 > &g_40e508 && *((char *)a0 - 64) & 8)
        {
            iter += (long long)(40000 / (*((int *)((char *)a0 - 32)) + 2));
        }
    }
    if ((char)a0[16] & 1)
    {
        iter -= 1600;
        return iter;
    }
    if (!((char)a0[16] & 8))
        return iter;
    iter += (long long)(22500 / ((int)a0[8] + 2));
    return iter;
}



// Function: line_cost @ 0x406b
typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
} struct_0;

extern unsigned int g_40d120;
extern char g_418120;

unsigned long long line_cost(struct_0 *a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x14]
    unsigned long long v1;  // [bp-0x10]

    if (a0 == *((long long *)&g_418120))
        return 0;
    v0 = g_40d120 - a1;
    v1 = v0 * 10 * v0 * 10;
    if (*((long long *)&a0[1].padding_0[8]) != *((long long *)&g_418120))
    {
        v0 = a1 - a0->field_14;
        v1 += v0 * 10 * v0 * 10 + (v0 * 10 * v0 * 10 >> 63) >> 1;
    }
    return v1;
}



// Function: put_paragraph @ 0x411d
typedef struct struct_0 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_0;

extern long long g_40e4e0;
extern struct_0 *g_40e500;
extern unsigned int g_418130;
extern unsigned int g_418134;

struct_0 * put_paragraph(struct_0 *a0)
{
    struct_0 *iter;  // [bp-0x10]

    sub_40417f(&g_40e4e0, g_418130);
    for (iter = g_40e500; iter != a0; iter = iter->field_20)
    {
        sub_40417f(iter, g_418134, g_418134);
    }
    return iter;
}



// Function: put_line @ 0x417f
typedef struct struct_0 {
    char padding_0[12];
    unsigned int field_c;
    char padding_10[16];
    unsigned long long field_20;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char *g_40d108;
extern unsigned int g_40d11c;
extern unsigned int g_40d128;
extern unsigned int g_41812c;
extern unsigned int g_418140;

int put_line(struct_0 *a0, unsigned int a1)
{
    struct_0 *iter;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x10]

    iter = a0;
    g_40d128 = 0;
    sub_4042aa(g_41812c);
    fputs_unlocked(g_40d108, stdout);
    g_40d128 = g_40d11c + g_40d128;
    sub_4042aa(a1 - g_40d128 & 0xffffffff);
    for (v1 = iter->field_20 - 40; iter != v1; iter += 1)
    {
        sub_404248(iter);
        sub_4042aa(iter->field_c);
    }
    sub_404248(iter);
    g_418140 = g_40d128;
    return putchar_unlocked(10);
}



// Function: put_word @ 0x4248
typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
} struct_0;

extern unsigned int g_40d128;

int put_word(struct_0 *idx)
{
    unsigned int v4;  // edx
    unsigned int v5;  // eax
    unsigned int v0;  // [bp-0x14]
    char *i;  // [bp-0x10]
    char *v2;  // [bp-0x10]

    i = &idx->field_0->field_0;
    for (v0 = idx->field_8; v0; i = v2)
    {
        v2 = i + 1;
        putchar_unlocked(*(i));
        v0 -= 1;
    }
    v4 = idx->field_8;
    v5 = g_40d128;
    g_40d128 = v5 + v4;
    return v5 + v4;
}



// Function: put_space @ 0x42aa
extern unsigned int g_40d128;
extern char g_418128;

long long put_space(unsigned int a0)
{
    unsigned long v3;  // rax
    unsigned int v0;  // [bp-0x10]
    unsigned int i;  // [bp-0xc]

    v0 = a0 + g_40d128;
    if (g_418128)
    {
        if (i > g_40d128 + 1)
        {
            for (i = ((int)((v0 < 0 ? v0 + 7 : v0)) >> 3) * 8; i > g_40d128; g_40d128 = (((int)((g_40d128 < 0 ? g_40d128 + 7 : g_40d128)) >> 3) + 1) * 8)
            {
                putchar_unlocked(9);
            }
        }
    }
    while (1)
    {
        if (v0 <= g_40d128)
            break;
        putchar_unlocked(32);
        g_40d128 = g_40d128 + 1;
    }
    return v3;
}



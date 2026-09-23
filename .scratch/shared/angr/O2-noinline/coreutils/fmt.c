// Function: main @ 0x26e0
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

extern char g_408be1;
extern option g_40ca80;
extern long long g_40d010;
extern long long stdout;
extern long long stdin;
extern unsigned int optind;
extern long long optarg;
extern unsigned int g_418110;
extern unsigned int g_418114;
extern unsigned int g_418118;
extern unsigned int g_41811c;
extern int g_418120;
extern unsigned long long g_418128;
extern char g_418130;
extern char g_418131;
extern char g_418132;
extern char g_418133;

int main(int a0, struct_0 **a1)
{
    long long v3;  // r15
    struct_0 **v4;  // r12
    char *v13;  // rax
    int *err;  // rax
    int *err1;  // rax
    int v5;  // ebp
    char *v6;  // rax
    struct_0 **v7;  // r12
    int v8;  // ecx
    unsigned int v9;  // eax
    unsigned int v10;  // ebx
    unsigned long v11;  // r14
    unsigned long fp;  // rax
    long long v0;  // [bp-0x48], Other Possible Types: unsigned long, unsigned long long
    unsigned long long v1;  // [bp-0x40]

    v3 = 0;
    v4 = a1;
    v5 = a0;
    sub_404350(*(a1));
    setlocale(6, &g_408be1);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_407c40(sub_4040f0);
    g_418130 = 0;
    g_418131 = 0;
    g_418132 = 0;
    g_418133 = 0;
    g_418120 = 75;
    g_418128 = &g_408be1;
    g_41811c = 0;
    g_418118 = 0;
    g_418114 = 0;
    if (v5 > 1)
    {
        v6 = &v4[1]->field_0;
        if (*(v6) == 45 && v6[1] - 48 <= 9)
        {
            v3 = v6 + 1;
            v5 -= 1;
            v7 = v4 + 1;
            *(v7) = *(v4);
            v4 = v7;
        }
    }
    v0 = 0;
    v8 = getopt_long(v5, v4, "0123456789cstuw:p:g:", &g_40ca80.name, NULL);
    if (v8 != -0x1)
    {
        if (v8 > 0x77)
            goto LABEL_402a9d;
        if (v8 > 98)
        {
            switch (v8)
            {
            case 99:
                g_418133 = 1;
                break;
            case 103:
                v0 = optarg;
                break;
            case 112:
                sub_402fd0(optarg);
                break;
            case 115:
                g_418131 = 1;
                break;
            case 116:
                g_418132 = 1;
                break;
            case 117:
                g_418130 = 1;
                break;
            case 119:
                v3 = optarg;
                break;
            default:
LABEL_402a9d:
                if (v8 - 48 <= 9)
                {
                    *((int *)&v0) = v8;
                    error(0, 0, dcgettext(NULL, "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead", 5));
                }
                sub_403dc0(1); /* do not return */
            }
        }
        else if (v8 == -0x83)
        {
            sub_406dd0(stdout, "fmt", "GNU coreutils", g_40d010, "Ross Paterson", 0);
            exit(0); /* do not return */
        }
        else if (v8 == -0x82)
        {
            sub_403dc0(0); /* do not return */
        }
    }
    if (v3)
    {
        g_418120 = sub_407450(v3, 0, 2500, &g_408be1, dcgettext(NULL, "invalid width", 5), 0);
        if (!v0)
            goto LABEL_4029f3;
        g_418110 = sub_407450(v0, 0, g_418120, &g_408be1, dcgettext(NULL, "invalid width", 5), 0);
    }
    else if (!v0)
    {
LABEL_4029f3:
        g_418110 = g_418120 * 0xbb / 200;
    }
    else
    {
        g_418110 = sub_407450(v0, 0, 75, &g_408be1, dcgettext(NULL, "invalid width", 5), 0);
        g_418120 = g_418110 + 10;
    }
    v9 = optind;
    if (optind != v5)
    {
        *((char *)&v0) = 0;
        v10 = 1;
        if (optind < v5)
        {
            do
            {
                v11 = v4[v9];
                if (!strcmp(v11, "-"))
                {
                    *((char *)&v0) = 1;
                    v10 &= sub_403a30(stdin, v11);
                }
                else
                {
                    fp = fopen(v11, "r");
                    if (fp)
                    {
                        v10 &= sub_403a30(fp, v11);
                    }
                    else
                    {
                        v1 = sub_4065c0(4);
                        v13 = dcgettext(NULL, "cannot open %s for reading", 5);
                        err = __errno_location();
                        v10 = 0;
                        error(0, *(err), v13);
                    }
                }
            } while ((v9 = optind + 1, optind = v9, v9 < v5));
            if ((char)v0)
                goto LABEL_402a16;
        }
    }
    else
    {
        v10 = sub_403a30(stdin, "-");
LABEL_402a16:
        if (sub_4041e0(stdin))
        {
            dcgettext(NULL, "closing standard input", 5);
            err1 = __errno_location();
            error(1, *(err1), "%s");
        }
    }
    return (char)v10 ^ 1;
}



// Function: set_other_indent @ 0x2c80
extern unsigned int g_40d0ec;
extern unsigned int g_40d0f0;
extern char g_41810c;
extern char g_418131;
extern char g_418132;
extern char g_418133;

long long set_other_indent(char a0)
{
    unsigned int v1;  // eax

    if (g_418131)
    {
        g_40d0ec = g_40d0f0;
        return g_40d0f0;
    }
    else if (g_418133)
    {
        g_40d0ec = (!a0 ? g_40d0f0 : *((int *)&g_41810c));
        return g_40d0ec;
    }
    else
    {
        if (g_418132)
        {
            if (a0 && *((int *)&g_41810c) != g_40d0f0)
            {
                g_40d0ec = *((int *)&g_41810c);
                return g_40d0f0;
            }
            if (g_40d0ec == g_40d0f0)
            {
                v1 = -(g_40d0f0 < 1);
                g_40d0ec = v1 & 3;
                return v1 & 3;
            }
        }
        else
        {
            g_40d0ec = g_40d0f0;
        }
        return g_40d0f0;
    }
}



// Function: same_para @ 0x2d10
extern unsigned int g_40d0e4;
extern char g_40d0f4;
extern char g_41810c;
extern unsigned int g_41811c;

unsigned int same_para(unsigned int a0)
{
    unsigned long v1;  // rdx

    if (g_40d0e4 != *((int *)&g_40d0f4))
        return 0;
    v1 = g_40d0e4 + g_41811c;
    if ((unsigned int)v1 <= *((int *)&g_41810c))
        return a0 != 10 & _INSERT(v1, 0, a0 != 0xffffffff);
    return 0;
}



// Function: base_cost @ 0x2d50
extern void g_40d120;
extern void g_40d148;

unsigned long long base_cost(void* a0)
{
    unsigned long long v1;  // r8
    char v2;  // al

    v1 = 4900;
    if (a0 > &g_40d120)
    {
        v2 = *((char *)a0 - 24);
        if (v2 & 2)
        {
            v1 = ((unsigned int)-((v2 & 8) < 1) & 362500) + 2400;
        }
        else
        {
            v1 = 3300;
            if (!(v2 & 4))
            {
                v1 = 4900;
                if (a0 > &g_40d148 && *((char *)a0 - 64) & 8)
                    v1 = (long long)(40000 / (*((int *)((char *)a0 - 32)) + 2)) + 4900;
            }
        }
    }
    if ((char)a0[16] & 1)
    {
        return v1 - 1600;
    }
    else if (!((char)a0[16] & 8))
    {
        return v1;
    }
    else
    {
        return v1 + (long long)(22500 / ((int)a0[8] + 2));
    }
}



// Function: line_cost @ 0x2e00
typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
} struct_0;

extern struct_0 *g_40d100;
extern unsigned int g_418110;

unsigned long long line_cost(struct_0 *a0, unsigned int a1)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    long long v3;  // rdx

    if (g_40d100 == a0)
        return 0;
    v1 = g_418110 - a1;
    v2 = (g_418110 - a1) * 10;
    if (g_40d100 == *((long long *)&a0[1].padding_0[8]))
        return v1 * 10 * v1 * 10;
    v3 = (a1 - a0->field_14) * 10;
    return v2 * v2 + (v3 * v3 >> 1);
}



// Function: fmt_paragraph @ 0x2e50
typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
} struct_0;

extern int g_40d0e0;
extern unsigned int g_40d0ec;
extern unsigned int g_40d0f0;
extern struct_0 *g_40d100;
extern void g_40d120;
extern void g_418120;

int fmt_paragraph(void)
{
    void* v2;  // r10
    unsigned int v3;  // eax
    long long v12;  // rsi
    long long v13;  // r9
    void* v14;  // r11
    void* v15;  // r15
    void* idx;  // r15
    void* v5;  // rcx
    unsigned int v6;  // r8d
    unsigned long long v7;  // rax
    void* index;  // rcx
    long long v9;  // rax
    void* v10;  // r10
    unsigned int v11;  // r8d
    unsigned int v0;  // [bp-0x34]

    v2 = &g_40d120;
    v3 = *((int *)&g_40d100->padding_0[8]);
    idx = (char *)g_40d100 - 40;
    *((unsigned long *)&g_40d100[1].padding_0[0]) = 0;
    *((int *)&g_40d100->padding_0[8]) = *((int *)&g_418120);
    v0 = v3;
    if (idx >= &g_40d120)
    {
        do
        {
            v5 = idx;
            v6 = (idx == v2 ? g_40d0f0 : g_40d0ec) + (int)idx[8];
            do
            {
                v7 = sub_402e00(v5 + 40, v6);
                v9 = v7 + (long long)index[24];
                if (idx == v10 && g_40d0e0 > 0)
                {
                    v12 = (v11 - g_40d0e0) * 10;
                    v9 += v12 * v12 >> 1;
                }
                if (v9 < v13)
                {
                    *((void* *)&idx[32]) = index;
                    *((unsigned int *)&idx[20]) = v11;
                }
            } while (v14 != index && (v6 = v11 + (int)index[8] + *((int *)((char *)index - 28)), v5 = index, *((int *)&g_418120) > v6));
            v15 = idx - 40;
            *((unsigned long long *)&v15[64]) = sub_402d50(idx) + v13;
            idx = v15;
            v2 = v10;
        } while (idx >= v2);
    }
    *((unsigned int *)&v14[8]) = v0;
    return v0;
}



// Function: set_prefix @ 0x2fd0
extern unsigned int g_418114;
extern unsigned int g_418118;
extern int g_41811c;
extern unsigned long long g_418128;

void* set_prefix(void* a0)
{
    unsigned long v1;  // 4115
    void* v2;  // rbx
    void* v3;  // rbx
    void* v4;  // rbx
    void* iter;  // rax

    v1 = *((char *)a0);
    v2 = a0;
    g_418118 = 0;
    if ((char)v1 == 32)
    {
        do
        {
            v4 = v2 + 1;
            v2 = v4;
        } while (*((char *)v2) == 32);
        g_418118 = (unsigned int)(1 - a0) + (unsigned int)v3;
        v2 = v4;
    }
    g_418128 = v2;
    g_41811c = strlen(v2);
    iter = g_41811c + v2;
    if (iter > v2)
    {
        do
        {
        } while (*((char *)iter - 1) == 32 && (iter -= 1, iter != v2));
    }
    *((char *)iter) = 0;
    g_418114 = iter - v2;
    return iter - v2;
}



// Function: get_space @ 0x3050
typedef struct FILE {
} FILE;

extern char g_40d0f8;
extern int g_41810c;

unsigned int get_space(FILE *a0, unsigned int a1)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax

    v1 = a1;
    while (1)
    {
        switch (v1)
        {
        case 32:
            v2 = g_41810c + 1;
            break;
        case 9:
            g_40d0f8 = 1;
            v2 = g_41810c / 8 * 8 + 8;
            break;
        default:
            return v1;
        }
        g_41810c = v2;
        v1 = getc_unlocked(a0);
    }
}



// Function: get_prefix @ 0x30b0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
} struct_0;

extern unsigned int g_40d0e4;
extern unsigned int g_41810c;
extern unsigned int g_418114;
extern char g_418118;
extern struct_0 *g_418128;

unsigned int get_prefix(FILE *a0)
{
    unsigned int v1;  // eax
    unsigned int v2;  // edx
    char *v3;  // rbx
    unsigned int v4;  // edx

    g_41810c = 0;
    v1 = sub_403050(a0, getc_unlocked(a0));
    v2 = g_41810c;
    if (!g_418114)
    {
        if (*((int *)&g_418118) < v2)
            v2 = *((int *)&g_418118);
        g_40d0e4 = v2;
        return v1;
    }
    else
    {
        v3 = &g_418128->padding_0[0];
        g_40d0e4 = v2;
        v4 = g_418128->padding_0[0];
        if ((char)v4)
        {
            do
            {
                if (v4 != v1)
                    return v1;
            } while ((v3 += 1, g_41810c = g_41810c + 1, v1 = (unsigned int)(int)(long long)getc_unlocked(a0), v4 = (unsigned int)*(v3), (char)v4));
        }
        return sub_403050(a0, v1);
    }
}



// Function: put_space @ 0x3150
extern char g_40d0f8;
extern int g_418108;

unsigned long long put_space(unsigned int a0)
{
    int v1;  // edx
    int i;  // ebx
    int v3;  // eax
    unsigned long long v4;  // rax

    v1 = g_418108;
    i = g_418108 + a0;
    if (g_40d0f8)
    {
        if (i >= 0)
            v3 = i;
        else
            v3 = i + 7;
        v4 = v3 & 0xfffffff8;
        if (g_418108 + 1 < (unsigned int)v4 && g_418108 < (unsigned int)v4)
        {
            do
            {
                putchar_unlocked(9);
                v1 = (g_418108 / 8 + 1) * 8;
                g_418108 = v1;
            } while (v3 >> 3 > (unsigned int)v4);
        }
    }
    if (i <= v1)
        return v4;
    do
    {
        putchar_unlocked(32);
        g_418108 = g_418108 + 1;
    } while (g_418108 < i);
    return g_418108;
}



// Function: put_word @ 0x31f0
typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
} struct_0;

extern unsigned int g_418108;

int put_word(struct_0 *a0)
{
    unsigned long v1;  // rbp
    char *i;  // rbx
    char *v3;  // rbx
    int v4;  // eax

    v1 = a0->field_8;
    i = &a0->field_0->field_0;
    if ((unsigned int)v1)
    {
        do
        {
            v3 = i + 1;
            v4 = putchar_unlocked(*(i));
            i = v3;
        } while (&a0->field_0[v1] != i);
        v1 = a0->field_8;
    }
    g_418108 = g_418108 + (unsigned int)v1;
    return v4;
}



// Function: put_line @ 0x3230
typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_40d0e0;
extern unsigned int g_40d0f4;
extern unsigned int g_418108;
extern unsigned int g_418114;
extern char *g_418128;

int put_line(struct_0 *a0, unsigned int a1)
{
    struct_0 *v1;  // rbp
    struct_0 *i;  // rbx
    struct_0 *v3;  // rbp

    v1 = a0;
    g_418108 = 0;
    sub_403150(g_40d0f4);
    fputs_unlocked(g_418128, stdout);
    g_418108 = g_418114 + g_418108;
    sub_403150(a1 - g_418108);
    i = *((long long *)&v1[2].field_8) - 40;
    v3 = v1;
    if (v3 != i)
    {
        do
        {
            v3 = (char *)&v1[3].field_0 + 4;
            sub_4031f0(v1);
            sub_403150(*((int *)((char *)v3 - 28)));
            v1 = v3;
        } while (i != v1);
    }
    sub_4031f0(v3);
    g_40d0e0 = g_418108;
    return putchar_unlocked(10);
}



// Function: put_paragraph @ 0x32d0
typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
} struct_0;

extern unsigned int g_40d0ec;
extern unsigned int g_40d0f0;
extern struct_0 g_40d120;
extern struct_0 *g_40d140;

int put_paragraph(struct_0 *i)
{
    int v1;  // eax
    struct_0 *v2;  // rbx
    struct_0 *v3;  // rbx
    int v4;  // eax

    v1 = sub_403230(&g_40d120.padding_0[0], g_40d0f0);
    v2 = g_40d140;
    if (g_40d140 == i)
        return v1;
    do
    {
        v3 = v2;
        v4 = sub_403230(v3, g_40d0ec);
        v2 = *((long long *)&v3[2].field_8);
    } while (i != *((long long *)&v3[2].field_8));
    return v4;
}



// Function: copy_rest @ 0x3320
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
} struct_0;

extern char g_40d0e4;
extern void g_418108;
extern char g_41810c;
extern unsigned int g_418114;
extern struct_0 *g_418128;

int copy_rest(FILE *a0, int a1)
{
    int v1;  // ebp
    char v2;  // r12b
    unsigned int v3;  // eax
    unsigned int v4;  // edx
    unsigned int v5;  // edi
    char *v6;  // r13

    v1 = a1;
    *((unsigned int *)&g_418108) = 0;
    v2 = a1 != 10 & a1 != -0x1;
    if (*((int *)&g_41810c) <= *((int *)&g_40d0e4))
    {
        if (!v2)
            return v1;
        sub_403150(*((int *)&g_40d0e4));
        v3 = *((int *)&g_418108);
        v4 = *((int *)&g_41810c);
        v5 = 0;
        v6 = &g_418128->padding_0[0];
        if (*((int *)&g_418108) == *((int *)&g_41810c))
            goto LABEL_4033e4;
    }
    else
    {
        sub_403150(*((int *)&g_40d0e4));
        v3 = *((int *)&g_418108);
        v4 = *((int *)&g_41810c);
        v6 = &g_418128->padding_0[0];
        if (*((int *)&g_418108) == *((int *)&g_41810c))
            goto LABEL_4033ac;
    }
    do
    {
        if (!*(v6))
            break;
    } while ((putchar_unlocked((unsigned int)(unsigned long long)*(v6)), v4 = (unsigned int)*((int *)&g_41810c), v6 += 1, v3 = *((int *)&g_418108) + 1, *((unsigned int *)&g_418108) = v3, v3 != *((int *)&g_41810c)));
LABEL_4033ac:
    if (!v2)
    {
        if (v1 != -0x1)
        {
            return v1;
        }
        else if (g_418114 + *((int *)&g_40d0e4) > *((int *)&g_41810c))
        {
            return v1;
        }
        else
        {
            putchar_unlocked(10);
            return v1;
        }
    }
    v5 = v4 - v3;
LABEL_4033e4:
    sub_403150(v5);
    while (1)
    {
        putchar_unlocked(v1);
        v1 = getc_unlocked(a0);
        if (v1 == 10)
        {
            return v1;
        }
        else if (v1 == -0x1)
        {
            return v1;
        }
    }
}



// Function: check_punctuation @ 0x3460
typedef struct struct_0 {
    void* field_0;
    int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

long long check_punctuation(struct_0 *idx)
{
    void* v6;  // r12
    void* v7;  // rbx
    unsigned int v16;  // ebp
    void* v17;  // rax
    char *ptr;  // rax
    unsigned int v19;  // eax
    char *ptr1;  // rax
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    long long v12;  // rcx
    long long v13;  // r8
    long long v14;  // r9
    unsigned long long *v15;  // rax
    long long v0;  // [bp-0x28]
    long long v1;  // [bp-0x20]
    long long v2;  // [bp-0x18]
    long long v3;  // [bp-0x10]
    long long v4;  // [bp-0x8]

    v6 = idx->field_0;
    v7 = v6 + idx->field_8 - 1;
    ptr1 = strchr("([\'`\"", *((char *)v6));
    v15 = __ctype_b_loc(v9, v10, v11, v12, v13, v14, v0, v1, v2, v3, v4);
    idx->field_10 = 4 & *((char *)(*(v15) + *((char *)v7) * 2)) | ptr1 | idx->field_10 & 250;
    if (v6 < v7)
    {
        while (1)
        {
            v16 = *((char *)v7);
            if (!strchr(")]\'\"", v16))
                break;
            v17 = v7 - 1;
            if (v6 != v17)
            {
                v7 = v17;
            }
            else
            {
                v16 = *((char *)v7 - 1);
                break;
            }
        }
    }
    else
    {
        v16 = *((char *)v7);
    }
    ptr = strchr(".?!", v16);
    v19 = idx->field_10 & 0xfffffffd;
    idx->field_10 = (char)v19 | (ptr) * 2;
    return v19 | _INSERT(0, 0, ptr) * 2;
}



// Function: flush_paragraph @ 0x3530
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
    void* field_20;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern void* g_40d100;
extern void g_40d120;
extern struct_0 *g_40d140;
extern unsigned long long g_416d60;
extern char g_416d80;

long long flush_paragraph(void)
{
    void* v1;  // r12
    struct_0 *i;  // rdx
    unsigned long v11;  // rax
    void* v12;  // r12
    void* iter;  // rax
    unsigned long dst;  // rax
    unsigned long count;  // rax
    unsigned long long v3;  // rsi
    long long v4;  // rax
    void* idx;  // rdx
    unsigned long v6;  // rsi
    long long v7;  // rcx
    long long v8;  // rcx
    unsigned int v9;  // eax
    unsigned long v10;  // rdx

    v1 = g_40d100;
    if (g_40d100 == &g_40d120)
    {
        count = fwrite_unlocked(&g_416d80, 1, g_416d60 - &g_416d80, stdout);
        g_416d60 = &g_416d80;
        return count;
    }
    sub_402e50();
    i = g_40d140;
    if (g_40d100 != g_40d140)
    {
        v3 = *((long long *)&g_40d140[1].padding_0[0]);
        v4 = 0x7fffffffffffffff;
        do
        {
            idx = i->field_20;
            v6 = (long long)idx[24];
            v7 = v3 - (long long)idx[24];
            if (v7 < v4)
            {
                v1 = i;
                v4 = v7;
            }
            v8 = v4 + 9;
            if (v4 <= 0x7ffffffffffffff6)
                v4 = v8;
            i = idx;
            v3 = v6;
        } while (g_40d100 != i);
    }
    sub_4032d0(v1);
    v9 = __memmove_chk(&g_416d80, *((long long *)v1), g_416d60 - *((long long *)v1), 5000);
    v10 = *((long long *)v1);
    v11 = (int)(v10 - v9);
    g_416d60 = g_416d60 - v11;
    v12 = g_40d100;
    if (v12 >= v1)
    {
        iter = v1;
        while (1)
        {
            iter += 40;
            *((unsigned long *)((char *)iter - 40)) = v10 + -(v11);
            if (iter > v12)
                break;
            v10 = *((long long *)iter);
        }
    }
    dst = __memmove_chk(&g_40d120, v1, v12 - v1 + 40, 40000);
    g_40d100 = v12 - (v1 - &g_40d120);
    return dst;
}



// Function: get_line @ 0x3690
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
    int field_c;
    char field_10;
    char padding_11[23];
    char field_28;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern struct_0 *g_40d100;
extern struct_0 g_416d10;
extern struct_1 *g_416d60;
extern char g_418108;
extern unsigned int g_41810c;
extern char g_418130;

unsigned int get_line(FILE *a0, unsigned int a1)
{
    struct_0 *v1;  // rdx
    unsigned int v2;  // ebp
    char *v3;  // rax
    char v4;  // al
    unsigned int v5;  // edx
    unsigned int v6;  // edx
    struct_0 *idx;  // rax
    int v8;  // edx
    char v9;  // dl

    v1 = g_40d100;
    v2 = a1;
    while (1)
    {
        v3 = &g_416d60->field_0;
        v1->field_0 = g_416d60;
        do
        {
            if (v3 == &g_418108)
            {
                sub_402c80(1);
                sub_403530();
                v3 = &g_416d60->field_0;
            }
            *(v3) = v2;
            g_416d60 = v3 + 1;
            v2 = getc_unlocked(a0);
            if (v2 == 0xffffffff)
            {
                v5 = (unsigned int)g_416d60;
                break;
            }
        } while ((v4 = (char)(unsigned long long)sub_404030((unsigned long long)v2), v3 = (char *)g_416d60, v5 = (unsigned int)g_416d60, !(char)(unsigned long long)sub_404030((unsigned long long)v2)));
        v6 = v5 - *((long long *)&g_40d100->padding_0[0]);
        *((unsigned int *)&g_40d100->padding_0[8]) = v6;
        g_41810c = g_41810c + v6;
        sub_403460(g_40d100);
        v2 = sub_403050(a0, v2);
        idx = g_40d100;
        v8 = g_41810c - g_41810c;
        idx->field_c = v8;
        if (v2 == 0xffffffff)
        {
            idx->field_10 = idx->field_10 | 8;
        }
        else if (idx->field_10 & 2)
        {
            v9 = 1 < v8 | v2 == 10;
            if (1 < v8 || v2 == 10)
                goto LABEL_403791;
            idx->field_10 = idx->field_10 & 247;
LABEL_4037a9:
            if (!g_418130)
            {
                if (idx != &g_416d10.padding_0[0])
                {
                    v1 = &idx->field_28;
                    g_40d100 = v1;
                    continue;
                }
                goto LABEL_403820;
            }
        }
        else
        {
            v9 = 0;
LABEL_403791:
            idx->field_10 = idx->field_10 & 247 | v9 * 8;
            if (v2 != 10)
                goto LABEL_4037a9;
        }
        idx->field_c = (unsigned int)((idx->field_10 & 8) + 1);
        if (idx != &g_416d10.padding_0[0])
            goto LABEL_4037e8;
LABEL_403820:
        sub_402c80(1);
        sub_403530();
        idx = g_40d100;
LABEL_4037e8:
        v1 = &idx->field_28;
        g_40d100 = v1;
        if (v2 == 10 || v2 == 0xffffffff)
            break;
    }
    return sub_4030b0(a0);
}



// Function: get_paragraph @ 0x3850
typedef struct FILE {
} FILE;

extern unsigned int g_40d0e0;
extern int g_40d0e4;
extern void g_40d0e8;
extern char g_40d0ec;
extern unsigned int g_40d0f0;
extern unsigned int g_40d0f4;
extern void* g_40d100;
extern void g_40d120;
extern unsigned long long g_416d60;
extern char g_416d80;
extern char g_41810c;
extern char g_418118;
extern unsigned int g_41811c;
extern char g_418131;
extern char g_418132;
extern char g_418133;

unsigned int get_paragraph(FILE *a0)
{
    unsigned int i;  // esi
    unsigned int v2;  // eax
    unsigned int v4;  // esi
    char v5;  // al

    i = *((int *)&g_40d0e8);
    for (g_40d0e0 = 0; i == 10 || i == 0xffffffff || g_40d0e4 < *((int *)&g_418118) || g_41811c + g_40d0e4 > *((int *)&g_41810c); i = sub_4030b0(a0))
    {
        v2 = sub_403320(a0, i);
        if (v2 == 0xffffffff)
        {
            *((unsigned int *)&g_40d0e8) = v2;
            return 0;
        }
        putchar_unlocked(10);
    }
    g_40d0f4 = g_40d0e4;
    g_40d0f0 = *((int *)&g_41810c);
    g_416d60 = &g_416d80;
    g_40d100 = &g_40d120;
    sub_402c80(sub_402d10(sub_403690(a0, i)));
    if (!g_418131)
    {
        v5 = sub_402d10(v4);
        if (g_418133)
        {
            if (v5)
            {
                do
                { } while ((char)sub_402d10(sub_403690(a0, v4)) && *((int *)&g_41810c) == *((int *)&g_40d0ec));
            }
        }
        else
        {
            if (!g_418132)
            {
                while ((char)sub_402d10(v4) && *((int *)&g_41810c) == *((int *)&g_40d0ec))
                {
                    sub_403690(a0, v4);
                }
            }
            else if (v5 && *((int *)&g_41810c) != g_40d0f0)
            {
                do
                { } while ((char)sub_402d10(sub_403690(a0, v4)) && *((int *)&g_41810c) == *((int *)&g_40d0ec));
            }
        }
    }
    if (g_40d100 <= &g_40d120)
        __assert_fail(); /* do not return */
    *((char *)g_40d100 - 24) = *((char *)g_40d100 - 24) | 10;
    *((unsigned int *)&g_40d0e8) = v4;
    return 1;
}



// Function: fmt @ 0x3a30
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
} struct_0;

extern char stdin;
extern unsigned int g_40d0e8;
extern unsigned int g_40d0ec;
extern char g_40d0f8;
extern struct_0 *g_40d100;

unsigned int fmt(FILE *a0, long long a1)
{
    unsigned int v1;  // eax
    unsigned int v2;  // ebp

    sub_4041b0();
    g_40d0f8 = 0;
    g_40d0ec = 0;
    g_40d0e8 = sub_4030b0(a0);
    while (1)
    {
        if (!(char)sub_403850(a0))
            break;
        sub_402e50();
        sub_4032d0(g_40d100);
    }
    if (ferror_unlocked(a0))
    {
        if (a0 != *((long long *)&stdin))
            sub_4041e0();
        else
            clearerr_unlocked(a0);
        sub_4066c0(0, 3, a1);
    }
    else if (a0 == *((long long *)&stdin))
    {
        clearerr_unlocked(a0);
        return 1;
    }
    else if (!sub_4041e0())
    {
        return 1;
    }
    else
    {
        v2 = *(__errno_location());
        if (v2 < 0)
            return 1;
        sub_4066c0(0, 3, a1);
        if (v2)
        {
            error(0, v2, "%s");
            return v1;
        }
    }
    error(0, 0, dcgettext(NULL, "read error", 5));
    return v1;
}



// Function: usage @ 0x3dc0
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned long long stderr;

void usage(int a0)
{
    unsigned long long v2;  // r12
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
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
    {
        v3 = dcgettext(NULL, "Try '%s --help' for more information.\n", 5);
        __fprintf_chk(stderr, 0x1, v3);
    }
    else
    {
        __printf_chk(1, dcgettext(NULL, "Usage: %s [-WIDTH] [OPTION]... [FILE]...\n", 5));
        v5 = dcgettext(NULL, "Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n", 5);
        fputs_unlocked(v5, stdout);
        sub_402f70(v6, v7, v8, v9, v10, v11);
        sub_402fa0(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n", 5);
        fputs_unlocked(v13, stdout);
        v14 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v14, stdout);
        v15 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v15, stdout);
        sub_403b90(v6, v7, v8, v9, v10, v11);
    }
    exit(a0); /* do not return */
}



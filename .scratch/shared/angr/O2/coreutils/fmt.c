// Function: main @ 0x26a0
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_408bc1;
extern option g_40baa0;
extern long long g_40c010;
extern long long stdout;
extern long long stdin;
extern unsigned int optind;
extern unsigned long long optarg;
extern unsigned int g_417110;
extern unsigned int g_417114;
extern unsigned int g_417118;
extern int g_41711c;
extern int g_417120;
extern unsigned long long g_417128;
extern char g_417130;
extern char g_417131;
extern char g_417132;
extern char g_417133;

int main(int a0, void* a1)
{
    void* iter;  // r12
    int v4;  // ebp
    unsigned long v13;  // r14
    unsigned long fp;  // rax
    char *v15;  // rax
    int *err;  // rax
    int *err1;  // rax
    char *v5;  // rax
    int v6;  // ecx
    unsigned long v7;  // r15
    unsigned long v8;  // r15
    void* v9;  // r15
    void* node;  // rax
    unsigned int v11;  // eax
    unsigned int v12;  // ebx
    void* v0;  // [bp-0x48], Other Possible Types: unsigned long, unsigned long long
    void* v1;  // [bp-0x40], Other Possible Types: unsigned long long

    iter = a1;
    v4 = a0;
    sub_404350(*((long long *)a1));
    setlocale(6, &g_408bc1);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_407b90(sub_404100);
    g_417130 = 0;
    g_417131 = 0;
    g_417132 = 0;
    g_417133 = 0;
    g_417120 = 75;
    g_417128 = &g_408bc1;
    g_41711c = 0;
    g_417118 = 0;
    g_417114 = 0;
    v1 = NULL;
    if (v4 > 1)
    {
        v5 = (long long)iter[8];
        if (*(v5) == 45 && v5[1] - 48 <= 9)
        {
            iter += 8;
            v4 -= 1;
            v1 = v5 + 1;
            *((long long *)iter) = *((long long *)((char *)iter - 8));
        }
    }
    v0 = NULL;
    v6 = getopt_long(v4, iter, "0123456789cstuw:p:g:", &g_40baa0.name, NULL);
    if (v6 != -0x1)
    {
        if (v6 > 0x77)
            goto LABEL_402acd;
        if (v6 > 98)
        {
            switch (v6)
            {
            case 99:
                g_417133 = 1;
                break;
            case 103:
                v0 = optarg;
                break;
            case 112:
                g_417118 = 0;
                v7 = optarg;
                if (*((char *)optarg) == 32)
                {
                    do
                    {
                        v9 = v7 + 1;
                        v7 = v9;
                    } while (*((char *)v7) == 32);
                    g_417118 = (unsigned int)(1 - optarg) + (unsigned int)v8;
                    v7 = v9;
                }
                g_417128 = v7;
                g_41711c = strlen(v7);
                node = g_41711c + v7;
                if (v7 < node)
                {
                    do
                    {
                    } while (*((char *)node - 1) == 32 && (node -= 1, v7 != node));
                }
                *((char *)node) = 0;
                g_417114 = (unsigned int)(node - v7);
                break;
            case 115:
                g_417131 = 1;
                break;
            case 116:
                g_417132 = 1;
                break;
            case 117:
                g_417130 = 1;
                break;
            case 119:
                v1 = optarg;
                break;
            default:
LABEL_402acd:
                if (v6 - 48 <= 9)
                {
                    *((int *)&v0) = v6;
                    error(0, 0, dcgettext(NULL, "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead", 5));
                }
                sub_403d70(1); /* do not return */
            }
        }
        else if (v6 == -0x83)
        {
            sub_406c80(stdout, "fmt", "GNU coreutils", g_40c010, "Ross Paterson", 0);
            exit(0); /* do not return */
        }
        else if (v6 == -0x82)
        {
            sub_403d70(0); /* do not return */
        }
    }
    if (v1)
    {
        g_417120 = sub_4074a0(v1, 0, 2500, &g_408bc1, dcgettext(NULL, "invalid width", 5), 0);
        if (!v0)
            goto LABEL_402a19;
        g_417110 = sub_4074a0(v0, 0, g_417120, &g_408bc1, dcgettext(NULL, "invalid width", 5), 0);
    }
    else if (!v0)
    {
LABEL_402a19:
        g_417110 = g_417120 * 0xbb / 200;
    }
    else
    {
        g_417110 = sub_4074a0(v0, 0, 75, &g_408bc1, dcgettext(NULL, "invalid width", 5), 0);
        g_417120 = g_417110 + 10;
    }
    v11 = optind;
    if (optind != v4)
    {
        *((char *)&v0) = 0;
        v12 = 1;
        if (optind < v4)
        {
            do
            {
                v13 = *((long long *)((char *)iter + 8 * v11));
                if (!strcmp(v13, "-"))
                {
                    *((char *)&v0) = 1;
                    v12 &= sub_403700(stdin, v13);
                }
                else
                {
                    fp = fopen(v13, "r");
                    if (fp)
                    {
                        v12 &= sub_403700(fp, v13);
                    }
                    else
                    {
                        v1 = sub_406030(4);
                        v15 = dcgettext(NULL, "cannot open %s for reading", 5);
                        err = __errno_location();
                        v12 = 0;
                        error(0, *(err), v15);
                    }
                }
            } while ((v11 = optind + 1, optind = v11, v11 < v4));
            if ((char)v0)
                goto LABEL_402a46;
        }
    }
    else
    {
        v12 = sub_403700(stdin, "-");
LABEL_402a46:
        if (sub_4041f0(stdin))
        {
            dcgettext(NULL, "closing standard input", 5);
            err1 = __errno_location();
            error(1, *(err1), "%s");
        }
    }
    return (char)v12 ^ 1;
}



// Function: fmt_paragraph @ 0x2cb0
extern char g_40c0e0;
extern unsigned int g_40c0ec;
extern unsigned int g_40c0f0;
extern void* g_40c100;
extern void g_40c120;
extern void g_40c148;
extern unsigned int g_417110;
extern int g_417120;

int fmt_paragraph(void)
{
    void* iter;  // rcx
    unsigned int v3;  // eax
    long long v12;  // r14
    unsigned long long iter1;  // r14
    char v14;  // al
    unsigned long v4;  // rbx
    void* node;  // rdx
    long long v6;  // rdi
    unsigned long long v7;  // r15
    unsigned int v8;  // eax
    unsigned long long v9;  // rax
    long long v10;  // r14
    long long v11;  // rax
    unsigned int v0;  // [bp-0x34]

    iter = g_40c100 - 40;
    v3 = (int)g_40c100[8];
    *((unsigned long long *)&g_40c100[24]) = 0;
    *((int *)&g_40c100[8]) = g_417120;
    v0 = v3;
    if (iter >= &g_40c120)
    {
        do
        {
            v4 = (int)iter[8];
            node = iter;
            v6 = 0x7fffffffffffffff;
            v7 = (iter == &g_40c120 ? g_40c0f0 : g_40c0ec) + v4;
            do
            {
                node += 40;
                if (g_40c100 != node)
                {
                    v8 = g_417110 - v7;
                    v9 = v8 * 10 * v8 * 10;
                    if (g_40c100 != (long long)node[32])
                    {
                        v10 = ((unsigned int)v7 - (int)node[20]) * 10;
                        v9 += v10 * v10 >> 1;
                    }
                }
                else
                {
                    v9 = 0;
                }
                v11 = v9 + (long long)node[24];
                if (iter == &g_40c120 && *((int *)&g_40c0e0) > 0)
                {
                    v12 = (unsigned int)((v7 & 0xffffffff) - *((int *)&g_40c0e0)) * 10;
                    v11 += v12 * v12 >> 1;
                }
                if (v11 < v6)
                {
                    *((void* *)&iter[32]) = node;
                    v6 = v11;
                    *((unsigned int *)&iter[20]) = v7;
                }
            } while (g_40c100 != node && (v7 += (unsigned long long)(unsigned int)((int)node[8] + *((int *)((char *)node - 28))), g_417120 > (unsigned int)v7));
            iter1 = 4900;
            if (iter > &g_40c120)
            {
                v14 = *((char *)iter - 24);
                if (v14 & 2)
                {
                    iter1 = ((unsigned int)-((v14 & 8) < 1) & 362500) + 2400;
                }
                else
                {
                    iter1 = 3300;
                    if (!(v14 & 4))
                    {
                        iter1 = 4900;
                        if (iter > &g_40c148 && *((char *)iter - 64) & 8)
                            iter1 = (long long)(40000 / (*((int *)((char *)iter - 32)) + 2)) + 4900;
                    }
                }
            }
            if ((char)iter[16] & 1)
            {
                iter1 -= 1600;
            }
            else if ((char)iter[16] & 8)
            {
                iter1 += (long long)(22500 / ((unsigned int)v4 + 2));
            }
            iter -= 40;
            *((unsigned long long *)&iter[64]) = v6 + iter1;
        } while (iter >= &g_40c120);
    }
    *((unsigned int *)&g_40c100[8]) = v0;
    return v0;
}



// Function: get_space @ 0x2ea0
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_40c0f8;
extern int g_41710c;

unsigned int get_space(struct_0 *idx, unsigned int a1)
{
    unsigned int v1;  // eax
    char *v2;  // rax

    v1 = a1;
    while (1)
    {
        switch (v1)
        {
        case 32:
            g_41710c = g_41710c + 1;
            v2 = &idx->field_8->field_0;
            if (v2 < idx->field_10)
            {
LABEL_402ee4:
                idx->field_8 = v2 + 1;
                v1 = *(v2);
                continue;
            }
            else
            {
LABEL_402f0d:
                v1 = __uflow(idx);
                break;
            }
        case 9:
            g_40c0f8 = 1;
            g_41710c = g_41710c / 8 * 8 + 8;
            v2 = &idx->field_8->field_0;
            if (v2 >= idx->field_10)
                goto LABEL_402f0d;
            goto LABEL_402ee4;
        default:
            return v1;
        }
    }
}



// Function: get_prefix @ 0x2f30
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

extern unsigned int g_40c0e4;
extern unsigned int g_41710c;
extern unsigned int g_417114;
extern char g_417118;
extern struct_2 *g_417128;

unsigned int get_prefix(struct_0 *idx)
{
    char *v1;  // rax
    unsigned int v2;  // esi
    unsigned int v3;  // eax
    unsigned int v4;  // edx
    char *v5;  // rbx
    unsigned int v6;  // edx
    char *v7;  // rax

    v1 = &idx->field_8->field_0;
    g_41710c = 0;
    if (v1 < idx->field_10)
    {
        idx->field_8 = v1 + 1;
        v2 = *(v1);
    }
    else
    {
        v2 = __uflow();
    }
    v3 = sub_402ea0(idx, v2);
    v4 = g_41710c;
    if (!g_417114)
    {
        if (*((int *)&g_417118) < v4)
            v4 = *((int *)&g_417118);
        g_40c0e4 = v4;
        return v3;
    }
    else
    {
        v5 = &g_417128->field_0;
        g_40c0e4 = v4;
        v6 = g_417128->field_0;
        if ((char)v6)
        {
            do
            {
                if (v6 != v3)
                    return v3;
                g_41710c = g_41710c + 1;
                v7 = &idx->field_8->field_0;
                if (v7 < idx->field_10)
                {
                    idx->field_8 = v7 + 1;
                    v3 = *(v7);
                }
                else
                {
                    v3 = __uflow(idx);
                }
                v6 = v5[1];
                v5 += 1;
            } while ((char)v6);
        }
        return sub_402ea0(idx, v3);
    }
}



// Function: put_word @ 0x3000
typedef struct struct_0 {
    void* field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

extern struct_1 *stdout;
extern unsigned int g_417108;

void put_word(struct_0 *a0)
{
    unsigned long v1;  // rbp
    void* v2;  // rbx
    void* i;  // rbx
    unsigned long v4;  // rsi
    char *v5;  // rax

    v1 = a0->field_8;
    v2 = a0->field_0;
    if ((unsigned int)v1)
    {
        i = v2;
        do
        {
            i += 1;
            v4 = *((char *)i - 1);
            v5 = &stdout->field_28->field_0;
            if (v5 < *((long long *)&stdout[1].padding_0[0]))
            {
                stdout->field_28 = v5 + 1;
                *(v5) = v4;
            }
            else
            {
                __overflow(stdout, v4);
            }
        } while (i != v1 + v2);
        v1 = a0->field_8;
    }
    g_417108 = g_417108 + (unsigned int)v1;
    return;
}



// Function: put_space @ 0x3060
typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
} struct_0;

extern struct_0 *stdout;
extern char g_40c0f8;
extern int g_417108;

unsigned long long put_space(unsigned int a0)
{
    int v1;  // edx
    int v2;  // ebx
    int v3;  // eax
    unsigned long long v4;  // rax
    unsigned long v5;  // rax
    unsigned long v6;  // rax

    v1 = g_417108;
    v2 = g_417108 + a0;
    if (g_40c0f8)
    {
        if (v2 >= 0)
            v3 = v2;
        else
            v3 = v2 + 7;
        v4 = v3 & 0xfffffff8;
        if (g_417108 + 1 < (unsigned int)v4 && g_417108 < (unsigned int)v4)
        {
            do
            {
                v5 = stdout->field_28;
                if (v5 < *((long long *)&stdout[1].padding_0[0]))
                {
                    stdout->field_28 = v5 + 1;
                    *((char *)v5) = 9;
                }
                else
                {
                    __overflow(stdout, 9);
                }
            } while ((v4 = (unsigned long long)(g_417108 / 8 + 1), v1 = (int)((unsigned int)v4 * 8), g_417108 = v1, v3 >> 3 > (unsigned int)v4));
        }
    }
    if (v2 <= v1)
        return v4;
    do
    {
        v6 = stdout->field_28;
        if (v6 < *((long long *)&stdout[1].padding_0[0]))
        {
            stdout->field_28 = v6 + 1;
            *((char *)v6) = 32;
        }
        else
        {
            __overflow(stdout, 32);
        }
    } while ((g_417108 = (int)(g_417108 + 1), g_417108 < v2));
    return g_417108;
}



// Function: put_line @ 0x3140
typedef struct struct_0 {
    void* field_0;
    unsigned int field_8;
} struct_0;

extern struct_0 *stdout;
extern unsigned int g_40c0e0;
extern unsigned int g_40c0f4;
extern unsigned int g_417108;
extern unsigned int g_417114;
extern char *g_417128;

char * put_line(struct_0 *a0, unsigned int a1)
{
    struct_0 *v1;  // rbp
    struct_0 *i;  // rbx
    struct_0 *v3;  // rbp
    char *v4;  // rax

    v1 = a0;
    g_417108 = 0;
    sub_403060(g_40c0f4);
    fputs_unlocked(g_417128, stdout);
    g_417108 = g_417114 + g_417108;
    sub_403060(a1 - g_417108);
    i = *((long long *)&v1[2].field_8) - 40;
    v3 = v1;
    if (v3 != i)
    {
        do
        {
            v3 = (char *)&v1[3].field_0 + 4;
            sub_403000(v1);
            sub_403060(*((int *)((char *)v3 - 28)));
            v1 = v3;
        } while (i != v1);
    }
    sub_403000(v3);
    g_40c0e0 = g_417108;
    v4 = stdout->field_28;
    if (v4 >= *((long long *)&stdout[1].padding_0[0]))
        return __overflow(stdout, 10);
    stdout->field_28 = v4 + 1;
    *(v4) = 10;
    return v4;
}



// Function: flush_paragraph @ 0x3200
typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
    struct struct_1 *field_20;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_40c0ec;
extern unsigned int g_40c0f0;
extern void* g_40c100;
extern void g_40c120;
extern struct_0 *g_40c140;
extern unsigned long long g_415d60;
extern char g_415d80;

long long flush_paragraph(void)
{
    void* j;  // r12
    struct_0 *i;  // rdx
    unsigned int v11;  // eax
    unsigned long v12;  // rdx
    unsigned long v13;  // rax
    void* iter;  // rax
    void* v15;  // r12
    unsigned long dst;  // rax
    unsigned long count;  // rax
    unsigned long long v3;  // rsi
    long long v4;  // rax
    struct_1 *idx;  // rdx
    unsigned long v6;  // rsi
    long long v7;  // rcx
    long long v8;  // rcx
    struct_0 *v9;  // r12
    struct_0 *v10;  // r12

    j = g_40c100;
    if (g_40c100 == &g_40c120)
    {
        count = fwrite_unlocked(&g_415d80, 1, g_415d60 - &g_415d80, stdout);
        g_415d60 = &g_415d80;
        return count;
    }
    sub_402cb0();
    i = g_40c140;
    if (g_40c100 != g_40c140)
    {
        v3 = *((long long *)&g_40c140->padding_0[24]);
        v4 = 0x7fffffffffffffff;
        do
        {
            idx = i->field_20;
            v6 = idx->field_18;
            v7 = v3 - idx->field_18;
            if (v7 < v4)
            {
                j = i;
                v4 = v7;
            }
            v8 = v4 + 9;
            if (v4 <= 0x7ffffffffffffff6)
                v4 = v8;
            i = idx;
            v3 = v6;
        } while (g_40c100 != i);
    }
    sub_403140(&g_40c120, g_40c0f0);
    v9 = g_40c140;
    if (j != g_40c140)
    {
        do
        {
            v10 = v9;
            sub_403140(v10, g_40c0ec);
            v9 = v10->field_20;
        } while (j != v10->field_20);
    }
    v11 = __memmove_chk(&g_415d80, *((long long *)j), g_415d60 - *((long long *)j), 5000);
    v12 = *((long long *)j);
    v13 = (int)(v12 - v11);
    iter = j;
    g_415d60 = g_415d60 - v13;
    v15 = g_40c100;
    if (v15 >= j)
    {
        while (1)
        {
            iter += 40;
            *((unsigned long *)((char *)iter - 40)) = v12 + -(v13);
            if (v15 < iter)
                break;
            v12 = *((long long *)iter);
        }
    }
    dst = __memmove_chk(&g_40c120, j, v15 - j + 40, 40000);
    g_40c100 = v15 - (j - &g_40c120);
    return dst;
}



// Function: get_line @ 0x3420
typedef struct struct_1 {
    char padding_0[8];
    struct struct_2 *field_8;
    unsigned long long field_10;
} struct_1;

typedef struct struct_0 {
    void* field_0;
    unsigned int field_8;
    int field_c;
    char field_10;
    char padding_11[23];
    char field_28;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern struct_0 *g_40c100;
extern struct_0 g_415d10;
extern void* g_415d60;
extern void g_417108;
extern unsigned int g_41710c;
extern char g_417130;

unsigned int get_line(struct_1 *index, unsigned int a1)
{
    unsigned int v3;  // ebp
    unsigned long long *v4;  // rax
    unsigned int v13;  // r15d
    void* v14;  // rax
    char *ptr;  // rax
    unsigned int v16;  // eax
    struct_0 *idx;  // rdx
    int v18;  // eax
    char v19;  // al
    struct_0 *v5;  // rdx
    void* i;  // rax
    char *v7;  // rax
    void* v8;  // rbx
    unsigned int v9;  // eax
    unsigned int v10;  // esi
    void* v11;  // r14
    char *ptr1;  // rax
    unsigned int v0;  // [bp-0x44]
    unsigned long long *v1;  // [bp-0x40]

    v3 = a1;
    v4 = __ctype_b_loc();
    v5 = g_40c100;
    v1 = v4;
LABEL_403450:
    i = g_415d60;
    v5->field_0 = g_415d60;
    if (i != &g_417108)
    {
        do
        {
LABEL_40346f:
            *((char *)i) = v3;
            v7 = &index->field_8->field_0;
            g_415d60 = i + 1;
            if (v7 < index->field_10)
            {
                index->field_8 = v7 + 1;
                v3 = *(v7);
                i = g_415d60;
            }
            else
            {
                v3 = __uflow(index);
                i = g_415d60;
                if (v3 == 0xffffffff)
                    goto LABEL_4034a7;
            }
            if (v3 <= 13)
                goto LABEL_4034a7;
            if (v3 == 32)
            {
LABEL_4034a7:
                v8 = *((long long *)&g_40c100->padding_0[0]);
                v9 = i - v8;
                *((unsigned int *)&g_40c100->padding_0[8]) = v9;
                v10 = *((char *)v8);
                v0 = g_41710c + v9;
                v11 = v8 + v9 - 1;
                g_41710c = v0;
                ptr1 = strchr("([\'`\"", v10);
                g_40c100->padding_0[16] = 4 & *((char *)(*(v1) + *((char *)v11) * 2)) | ptr1 | g_40c100->padding_0[16] & 250;
                if (v8 < v11)
                {
                    while (1)
                    {
                        v13 = *((char *)v11);
                        if (!strchr(")]\'\"", v13))
                            break;
                        v14 = v11 - 1;
                        if (v8 != v14)
                        {
                            v11 = v14;
                        }
                        else
                        {
                            v13 = *((char *)v11 - 1);
                            break;
                        }
                    }
                }
                else
                {
                    v13 = *((char *)v11);
                }
                ptr = strchr(".?!", v13);
                g_40c100->padding_0[16] = g_40c100->padding_0[16] & 0xfffffffd | (ptr) * 2;
                v16 = sub_402ea0(index, v3);
                idx = g_40c100;
                v3 = v16;
                v18 = g_41710c - v0;
                idx->field_c = v18;
                if (v3 == 0xffffffff)
                {
                    idx->field_10 = idx->field_10 | 8;
LABEL_40363c:
                    idx->field_c = (unsigned int)((idx->field_10 & 8) + 1);
                    if (idx != &g_415d10.padding_0[0])
                        goto LABEL_403657;
LABEL_4036d0:
                    sub_403390(1);
                    sub_403200();
                    idx = g_40c100;
LABEL_403657:
                    g_40c100 = &idx->field_28;
                    if (v3 == 10 || v3 == 0xffffffff)
                        return sub_402f30(index);
                }
                else if (idx->field_10 & 2)
                {
                    v19 = 1 < v18 | v3 == 10;
                    if (1 < v18 || v3 == 10)
                        goto LABEL_4035b7;
                    idx->field_10 = idx->field_10 & 247;
LABEL_4035cf:
                    if (g_417130)
                        goto LABEL_40363c;
                    if (idx == &g_415d10.padding_0[0])
                        goto LABEL_4036d0;
                    g_40c100 = &idx->field_28;
                    goto LABEL_403450;
                }
                else
                {
                    v19 = 0;
LABEL_4035b7:
                    idx->field_10 = idx->field_10 & 247 | v19 * 8;
                    if (v3 == 10)
                        goto LABEL_40363c;
                    goto LABEL_4035cf;
                }
            }
        } while (i != &g_417108);
    }
    sub_403390(1);
    sub_403200();
    i = g_415d60;
    goto LABEL_40346f;
}



// Function: fmt @ 0x3700
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_2 {
    char field_0;
    char padding_1[7];
    struct struct_3 *field_8;
    unsigned long long field_10;
} struct_2;

typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_3 {
    char field_0;
} struct_3;

extern struct_0 *stdout;
extern char stdin;
extern unsigned int g_40c0e0;
extern char g_40c0e4;
extern unsigned int g_40c0e8;
extern void g_40c0ec;
extern unsigned int g_40c0f0;
extern unsigned int g_40c0f4;
extern char g_40c0f8;
extern void* g_40c100;
extern void g_40c120;
extern struct_1 *g_40c140;
extern unsigned long long g_415d60;
extern char g_415d80;
extern unsigned int g_417108;
extern char g_41710c;
extern unsigned int g_417114;
extern char g_417118;
extern unsigned int g_41711c;
extern unsigned long long g_417128;
extern char g_417131;
extern char g_417132;
extern char g_417133;

unsigned int fmt(struct_2 *idx, long long a1)
{
    unsigned int v1;  // ebp
    char v2;  // r13b
    unsigned long v11;  // rax
    long long v12;  // rdi
    unsigned long long v13;  // rsi
    unsigned long long v14;  // rax
    unsigned int v15;  // ecx
    unsigned int v16;  // r8d
    unsigned int v17;  // esi
    unsigned int v18;  // esi
    unsigned int j;  // eax
    unsigned int v19;  // esi
    unsigned int v20;  // ecx
    struct_1 *iter;  // rbp
    unsigned int v4;  // r8d
    unsigned long v22;  // rax
    struct_2 *v23;  // rdi
    struct_2 *v24;  // rdi
    unsigned int v25;  // ebp
    unsigned long node;  // r15
    char v6;  // sil
    unsigned long v7;  // rax
    unsigned long v8;  // rax
    unsigned long v9;  // rax
    char v10;  // al

    sub_4041c0();
    g_40c0f8 = 0;
    *((unsigned int *)&g_40c0ec) = 0;
    g_40c0e8 = sub_402f30(idx);
    v1 = g_40c0e8;
    while (1)
    {
        g_40c0e0 = 0;
        while (1)
        {
            if (v1 != 10 && v1 != 0xffffffff)
            {
                if (*((int *)&g_417118) <= *((int *)&g_40c0e4) && g_41711c + *((int *)&g_40c0e4) <= *((int *)&g_41710c))
                {
                    g_40c0f4 = *((int *)&g_40c0e4);
                    g_40c0f0 = *((int *)&g_41710c);
                    g_415d60 = &g_415d80;
                    g_40c100 = &g_40c120;
                    v12 = 0;
                    v13 = sub_403420(idx, v1) & 0xffffffff;
                    if (*((int *)&g_40c0e4) == g_40c0f4)
                    {
                        v14 = g_41711c + *((int *)&g_40c0e4);
                        if ((unsigned int)v14 <= *((int *)&g_41710c))
                            v12 = (unsigned int)v13 != 0xffffffff & _INSERT(v14, 0, (unsigned int)v13 != 10);
                    }
                    sub_403390(v12, v13);
                    if (!g_417131)
                    {
                        if (g_417133)
                        {
                            if (v15 == v16 && v15 + g_41711c <= *((int *)&g_41710c) && v17 != 10)
                            {
                                v18 = v17;
                                if (v18 != 0xffffffff)
                                {
                                    do
                                    {
                                        v18 = sub_403420(idx, v18);
                                    } while (*((int *)&g_40c0e4) == g_40c0f4 && *((int *)&g_40c0e4) + g_41711c <= *((int *)&g_41710c) && v18 != 10 && v18 != 0xffffffff && *((int *)&g_40c0ec) == *((int *)&g_41710c));
                                }
                            }
                        }
                        else
                        {
                            if (!g_417132)
                            {
                                v20 = v15;
                                if (v20 == v16)
                                {
                                    do
                                    {
                                    } while (v20 + g_41711c <= *((int *)&g_41710c) && v17 != 10 && v17 != 0xffffffff && *((int *)&g_40c0ec) == *((int *)&g_41710c) && (sub_403420(idx, v17), v20 = (unsigned int)*((int *)&g_40c0e4), *((int *)&g_40c0e4) == g_40c0f4));
                                }
                            }
                            else if (v15 == v16 && v15 + g_41711c <= *((int *)&g_41710c) && v17 != 10 && v17 != 0xffffffff)
                            {
                                v19 = v17;
                                if (g_40c0f0 != *((int *)&g_41710c))
                                {
                                    do
                                    {
                                        v19 = sub_403420(idx, v19);
                                    } while (*((int *)&g_40c0e4) == g_40c0f4 && *((int *)&g_40c0e4) + g_41711c <= *((int *)&g_41710c) && v19 != 10 && v19 != 0xffffffff && *((int *)&g_40c0ec) == *((int *)&g_41710c));
                                }
                            }
                        }
                    }
                }
                g_417108 = 0;
                if (*((int *)&g_41710c) > *((int *)&g_40c0e4))
                {
                    v2 = 1;
                    goto LABEL_40379e;
                }
                sub_403060(*((int *)&g_40c0e4));
                j = g_417108;
                v4 = *((int *)&g_41710c);
                node = g_417128;
                if (j == *((int *)&g_41710c))
                {
                    sub_403060(0);
                    do
                    {
LABEL_403981:
                        v8 = stdout->field_28;
                        if (v8 < *((long long *)&stdout[1].padding_0[0]))
                        {
                            stdout->field_28 = v8 + 1;
                            *((char *)v8) = v1;
                        }
                        else
                        {
                            __overflow(stdout, (char)v1);
                        }
                        v9 = idx->field_8;
                        if (v9 < idx->field_10)
                        {
                            idx->field_8 = v9 + 1;
                            v1 = *((char *)v9);
                            v10 = 1;
                        }
                        else
                        {
                            v1 = __uflow(idx);
                            v10 = v1 != 0xffffffff;
                        }
                    } while (v1 != 10 && v10);
LABEL_4038db:
                    if (v1 != 0xffffffff)
                        goto LABEL_4038e4;
LABEL_40383a:
                    g_40c0e8 = 0xffffffff;
                    if (idx->field_0 & 32)
                    {
                        v23 = idx;
                        if (idx != *((long long *)&stdin))
                            sub_4041f0(v23);
                        else
                            clearerr_unlocked(v23);
                        sub_4063b0(0, 3, a1);
                    }
                    else
                    {
                        v24 = idx;
                        if (idx == *((long long *)&stdin))
                        {
                            clearerr_unlocked(v24);
                            return 1;
                        }
                        else if (!sub_4041f0(v24))
                        {
                            return 1;
                        }
                        else
                        {
                            v25 = *(__errno_location());
                            if (v25 < 0)
                                return 1;
                            sub_4063b0(0, 3, a1);
                            if (v25)
                            {
                                error(0, v25, "%s");
                                return 0;
                            }
                        }
                    }
                    error(0, 0, dcgettext(NULL, "read error", 5));
                    return 0;
                }
LABEL_40393a:
                v2 = 1;
                do
                {
LABEL_4037df:
                    v6 = *((char *)node);
                    if (!v6)
                        break;
                    node += 1;
                    v7 = stdout->field_28;
                    if (v7 < *((long long *)&stdout[1].padding_0[0]))
                    {
                        stdout->field_28 = v7 + 1;
                        *((char *)v7) = v6;
                    }
                    else
                    {
                        __overflow(stdout);
                        v4 = *((int *)&g_41710c);
                    }
                    j = g_417108 + 1;
                    g_417108 = j;
                } while (j != v4);
LABEL_403810:
                if (!v2)
                {
                    if (v1 == 0xffffffff)
                        goto LABEL_403822;
LABEL_4038e4:
                    v11 = stdout->field_28;
                    if (v11 < *((long long *)&stdout[1].padding_0[0]))
                    {
                        stdout->field_28 = v11 + 1;
                        *((char *)v11) = 10;
                    }
                    else
                    {
                        __overflow(stdout, 10);
                    }
                    v1 = sub_402f30(idx);
                }
                else
                {
                    sub_403060(v4 - j);
                    if (v1 != 0xffffffff)
                        goto LABEL_403981;
LABEL_403822:
                    if (g_417114 + *((int *)&g_40c0e4) <= *((int *)&g_41710c))
                    {
                        v22 = stdout->field_28;
                        if (v22 < *((long long *)&stdout[1].padding_0[0]))
                        {
                            stdout->field_28 = v22 + 1;
                            *((char *)v22) = 10;
                            goto LABEL_40383a;
                        }
                        else
                        {
                            __overflow(stdout, 10);
                            goto LABEL_40383a;
                        }
                    }
                }
            }
            else
            {
                g_417108 = 0;
                v2 = v1 != 10 & v1 != 0xffffffff;
                if (*((int *)&g_41710c) > *((int *)&g_40c0e4))
                {
LABEL_40379e:
                    sub_403060(*((int *)&g_40c0e4));
                    j = g_417108;
                    v4 = *((int *)&g_41710c);
                    node = g_417128;
                    if (*((int *)&g_41710c) == g_417108)
                        goto LABEL_403810;
                    goto LABEL_4037df;
                }
                if (!v2)
                    goto LABEL_4038db;
                sub_403060(*((int *)&g_40c0e4));
                j = g_417108;
                v4 = *((int *)&g_41710c);
                node = g_417128;
                if (j != *((int *)&g_41710c))
                    goto LABEL_40393a;
                sub_403060(0);
                if (v1 != 0xffffffff)
                    goto LABEL_403981;
                goto LABEL_403822;
            }
        }
        if (g_40c100 <= &g_40c120)
            break;
        *((char *)g_40c100 - 24) = *((char *)g_40c100 - 24) | 10;
        g_40c0e8 = v17;
        sub_402cb0();
        sub_403140(&g_40c120, g_40c0f0);
        iter = g_40c140;
        if (g_40c140 != g_40c100)
        {
            do
            {
                sub_403140(iter, *((int *)&g_40c0ec));
                iter = iter->field_20;
            } while (iter != g_40c100);
        }
        v1 = g_40c0e8;
    }
    __assert_fail(); /* do not return */
}



// Function: usage @ 0x3d70
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
    char padding_1[15];
    char *field_10;
} struct_0;

extern FILE *stdout;
extern unsigned long long stderr;

void usage(int a0)
{
    unsigned long long v16;  // r15
    unsigned long long v17;  // r14
    char *v26;  // rax
    char *v27;  // rax
    char *v28;  // rax
    char *v29;  // rax
    char *v30;  // rax
    char *v31;  // rax
    char *v32;  // rsi
    struct_0 *v33;  // rbx
    char *v34;  // r14
    unsigned long long v18;  // r13
    char *v36;  // rax
    char *v37;  // rax
    unsigned long long v19;  // r12
    unsigned long long v20;  // rbx
    unsigned long v21;  // fs
    char *v22;  // rax
    struct_0 *v23;  // rbx
    char *v25;  // rax
    int v0;  // [bp-0xb8], Other Possible Types: char
    unsigned long long v1;  // [bp-0x88]
    unsigned long long v2;  // [bp-0x80]
    unsigned long long v3;  // [bp-0x78]
    unsigned long long v4;  // [bp-0x70]
    unsigned long long v5;  // [bp-0x68]
    unsigned long long v6;  // [bp-0x60]
    unsigned long long v7;  // [bp-0x58]
    unsigned long long v8;  // [bp-0x50]
    unsigned long v9;  // [bp-0x40]
    unsigned long long v10;  // [bp-0x30]
    unsigned long long v11;  // [bp-0x20]
    unsigned long long v12;  // [bp-0x18]
    unsigned long long v13;  // [bp-0x10]
    unsigned long long v14;  // [bp-0x8]

    v14 = v16;
    v13 = v17;
    v12 = v18;
    v11 = v19;
    v10 = v20;
    v9 = *((long long *)(40 + v21));
    if (a0)
    {
        v22 = dcgettext(NULL, "Try '%s --help' for more information.\n", 5);
        __fprintf_chk(stderr, 0x1, v22);
    }
    else
    {
        v23 = &v0;
        __printf_chk(1, dcgettext(NULL, "Usage: %s [-WIDTH] [OPTION]... [FILE]...\n", 5));
        v25 = dcgettext(NULL, "Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "\nWith no FILE, or when FILE is -, read standard input.\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n", 5);
        fputs_unlocked(v29, stdout);
        v30 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v30, stdout);
        v31 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v31, stdout);
        v0 = (int)_INSERT(v0 CONCAT 0, 0, "[");
        v32 = "[";
        *((char **)&(&v0)[8]) = "test invocation";
        *((char **)&(&v0)[16]) = "coreutils";
        *((char **)&(&v0)[24]) = "Multi-call invocation";
        v1 = "sha256sum";
        *((char **)&(&v0)[32]) = "sha224sum";
        v3 = "sha384sum";
        *((char **)&(&v0)[40]) = "sha2 utilities";
        v2 = "sha2 utilities";
        v4 = "sha2 utilities";
        v5 = "sha512sum";
        v6 = "sha2 utilities";
        v7 = 0;
        v8 = 0;
        do
        {
            v23 = v23;
        } while (strcmp("fmt", v32) && (v32 = v33->field_10, v23 = v33 + 16, v33->field_10));
        v34 = *((long long *)&v23->padding_1[7]);
        if (!*((long long *)&v23->padding_1[7]))
            v34 = "fmt";
        __printf_chk(1, dcgettext(NULL, "\n%s online help: <%s>\n", 5));
        v36 = setlocale(5, NULL);
        if (v36 && strncmp(v36, "en_", 3))
        {
            v37 = dcgettext(NULL, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
            fputs_unlocked(v37, stdout);
        }
        __printf_chk(1, dcgettext(NULL, "Full documentation <%s%s>\n", 5));
        __printf_chk(1, dcgettext(NULL, "or available locally via: info '(coreutils) %s%s'\n", 5));
    }
    exit(a0); /* do not return */
}



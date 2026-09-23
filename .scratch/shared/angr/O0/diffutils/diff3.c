// Function: free_diff_block @ 0x2b49
void free_diff_block(unsigned long a0)
{
    return;
}



// Function: next_to_n2 @ 0x2b58
void next_to_n2(unsigned long a0)
{
    return;
}



// Function: main @ 0x2b67
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

extern char g_40f09e;
extern option g_413880;
extern unsigned long long g_414020;
extern long long g_414028;
extern unsigned int g_414030;
extern long long stdout;
extern void stdin;
extern int optind;
extern unsigned long long optarg;
extern char g_414109;
extern char g_41410a;
extern char g_41410b;
extern char g_41410c;
extern char g_41410d;
extern char g_41410e;
extern char g_41410f;
extern char g_414110;
extern char g_414111;
extern char g_414112;

unsigned int main(int a0, void* a1)
{
    long long v26;  // rdi
    long long v27;  // rsi
    long long v28;  // rdx
    long long v29;  // rcx
    long long v30;  // r8
    long long v31;  // r9
    long long v32;  // rax
    unsigned long long v33;  // rdx
    char v0;  // [bp-0x135]
    int iter;  // [bp-0x134]
    unsigned int v2;  // [bp-0x130]
    unsigned int v3;  // [bp-0x12c]
    int v4;  // [bp-0x128]
    int v5;  // [bp-0x128]
    unsigned int v6;  // [bp-0x124]
    char v7;  // [bp-0x120]
    void* v8;  // [bp-0x118]
    char v9;  // [bp-0x110]
    void* v10;  // [bp-0x108]
    unsigned long v11;  // [bp-0x100]
    long long v12;  // [bp-0xf8]
    long long v13;  // [bp-0xf0]
    long long v14;  // [bp-0xe8]
    unsigned int v15;  // [bp-0xe0]
    unsigned int v16;  // [bp-0xdc]
    unsigned int v17;  // [bp-0xd8]
    char v18;  // [bp-0xd4]
    int v19;  // [bp-0xd0]
    int v20;  // [bp-0xcc]
    long long v21;  // [bp-0xc8]
    int <0x402b67[is_5]|Stack bp-0xc8, 1 B>;  // [bp-0xc8]
    long long v22;  // [bp-0xc0]
    long long v23;  // [bp-0xb8]
    stat v24;  // [bp-0xa8]

    v3 = 0;
    v4 = 0;
    g_414030 = 2;
    sub_40637f(*((long long *)a1));
    setlocale(6, &g_40f09e);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_4062bb(0);
    sub_409533(v26, v27, v28, v29, v30, v31);
    while (1)
    {
        v6 = getopt_long(a0, a1, "aeimvx3AEL:TX", &g_413880.name, NULL);
        if (v6 == 0xffffffff)
            break;
        switch (v6)
        {
        case 51:
            g_41410e = 1;
            v3 |= 1;
            break;
        case 65:
            g_414110 = 1;
            g_41410c = 1;
            v3 |= 2;
            break;
        case 69:
            g_41410c = 1;
            v3 |= 4;
            break;
        case 76:
            if (v4 <= 2)
            {
                v5 = v4 + 1;
                (&<0x402b67[is_5]|Stack bp-0xc8, 1 B>)[v4] = optarg;
                v4 = v5;
                break;
            }
            else
            {
                sub_403489("too many file label options", 0); /* do not return */
            }
        case 84:
            g_41410d = 1;
            break;
        case 88:
            g_41410f = 1;
            v3 |= 8;
            break;
        case 97:
            g_414109 = 1;
            break;
        case 101:
            v3 |= 16;
            break;
        case 105:
            g_414111 = 1;
            break;
        case 109:
            g_414112 = 1;
            break;
        case 118:
            v32 = sub_406940("Randy Smith");
            sub_408be7(stdout, "diff3", "GNU diffutils", g_414028, v32, 0);
            sub_4034ff(v26, v27, v28, v29, v30, v31);
            return 0;
        case 120:
            g_41410f = 1;
            v3 |= 32;
            break;
        case 128:
            g_414020 = optarg;
            break;
        case 129:
            sub_403556();
            sub_4034ff(v26, v27, v28, v29, v30, v31);
            return 0;
        case 130:
            g_41410a = 1;
            break;
        default:
            sub_403489(0, 0); /* do not return */
        }
    }
    g_41410b = (g_414112 ^ 1) & v3;
    g_414110 = g_414112 & !v3 || g_414110;
    g_41410c = g_414112 & !v3 || g_41410c;
    if (!(v3 - 1 & v3) && !(g_414112 & g_414111) && (!v4 || g_41410c == 1))
    {
        if (a0 - optind == 3)
        {
            v10 = a1 + optind * 8;
            for (iter = v4; iter <= 2; iter += 1)
            {
                (&<0x402b67[is_5]|Stack bp-0xc8, 1 B>)[iter] = *((long long *)((char *)v10 + 8 * iter));
            }
            v2 = 2 - (g_414112 | g_41410b);
            if (!strcmp(*((long long *)((char *)v10 + 8 * v2)), "-"))
            {
                v2 = 3 - v2;
                if (!strcmp(*((long long *)v10), "-") || !strcmp(*((long long *)((char *)v10 + 8 * v2)), "-"))
                    sub_405fcf("'-' specified for more than one input file"); /* do not return */
            }
            v15 = 0;
            v16 = 3 - v2;
            v17 = v2;
            for (iter = 0; iter <= 2; iter += 1)
            {
                *((int *)&(&v18)[4 * (&v15)[iter]]) = iter;
            }
            for (iter = 0; iter <= 2; iter += 1)
            {
                if (strcmp(*((long long *)((char *)v10 + 8 * iter)), "-"))
                {
                    if (stat(*((long long *)((char *)v10 + 8 * iter)), &v24) < 0)
                        sub_40600c(*((long long *)((char *)v10 + 8 * iter))); /* do not return */
                    if (((unsigned short)v24.st_nlink & 0xf000) == 0x4000)
                        error(2, 21, "%s");
                }
            }
            signal(0x11, NULL);
            v11 = *((long long *)((char *)v10 + 8 * v20));
            v12 = sub_4043e5(*((long long *)((char *)v10 + 8 * v19)), v11, &v7, &v9);
            v13 = sub_4043e5(*((long long *)((char *)v10 + 8 * v18)), v11, &v7, &v8);
            sub_402b58(v13);
            sub_402b58(v12);
            v14 = sub_403696(v13, v12, v12);
            sub_402b49(v13);
            sub_402b49(v12);
            if (g_41410b)
            {
                v0 = sub_4054de(stdout, v14, &v15, &v18, v21, v22, v23);
            }
            else
            {
                if (g_414112)
                {
                    sub_409414(*((long long *)((char *)v10 + 8 * v18)), "r", *((long long *)&stdin), "r");
                    v0 = sub_405a29(*((long long *)&stdin), stdout, v14, &v15, &v18, v21, v22, v23);
                    if (ferror_unlocked(*((long long *)&stdin)))
                        sub_405fcf("read failed"); /* do not return */
                }
                else
                {
                    sub_405012(stdout, v14, &v15, &v18);
                    v0 = 0;
                }
            }
            free(v8);
            free(v9);
            sub_4034ff(v26, v27, v28, v29, v30, v31);
            exit(v0); /* do not return */
        }
        else if (a0 - optind <= 2)
        {
            sub_403489("missing operand after '%s'", *((long long *)((char *)a1 + 8 * a0 - 8)), a0 * 8 - 8); /* do not return */
        }
        else
        {
            v33 = (optind + 3) * 8;
            sub_403489("extra operand '%s'", *((long long *)((char *)a1 + v33)), v33); /* do not return */
        }
    }
    sub_403489("incompatible options", 0); /* do not return */
}



// Function: try_help @ 0x3489
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



// Function: check_stdout @ 0x34ff
typedef struct FILE {
} FILE;

extern FILE *stdout;

unsigned long long check_stdout(void)
{
    unsigned long long v1;  // rax

    if (ferror_unlocked(stdout))
        sub_405fcf("write failed"); /* do not return */
    v1 = fclose(stdout);
    if (!(unsigned int)v1)
        return v1;
    sub_40600c(gettext("standard output")); /* do not return */
}



// Function: usage @ 0x3556
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern struct_0 *g_413a40;
extern FILE *stdout;

int usage(void)
{
    long long v12;  // rcx
    long long v13;  // r8
    long long v14;  // r9
    char *v4;  // rax
    char *v6;  // rax
    char *v7;  // rax
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    struct_0 **i;  // [bp-0x20]

    printf(gettext("Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n"));
    printf("%s\n\n", gettext("Compare three files line by line."));
    v4 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
    fputs_unlocked(v4, stdout);
    for (i = &g_413a40; *(i); i += 1)
    {
        if (*(i)->field_0)
            printf("  %s\n", gettext(*(i)));
        else
            putchar_unlocked(10);
    }
    v6 = gettext("\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n");
    fputs_unlocked(v6, stdout);
    v7 = gettext("Exit status is 0 if successful, 1 if conflicts, 2 if trouble.");
    printf("\n%s\n%s\n", gettext("If a FILE is '-', read standard input."), v7);
    return sub_408ccb(v9, v10, v11, v12, v13, v14);
}



// Function: make_3way_diff @ 0x3696
typedef struct struct_1 {
    char padding_0[16];
    long long field_10;
} struct_1;

typedef struct struct_2 {
    char padding_0[24];
    unsigned long long field_18;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_40fd80;

long long make_3way_diff(struct_1 *a0, unsigned long long a1)
{
    unsigned int v0;  // [bp-0x8c]
    unsigned int v1;  // [bp-0x88]
    int v2;  // [bp-0x84]
    unsigned long v3;  // [bp-0x80]
    unsigned long v4;  // [bp-0x78]
    unsigned long long idx;  // [bp-0x70]
    struct_0 **v6;  // [bp-0x68]
    struct_0 **v7;  // [bp-0x60]
    struct_2 *index;  // [bp-0x58]
    struct_0 **v9;  // [bp-0x50]
    unsigned long v10;  // [bp-0x48]
    unsigned long v11;  // [bp-0x40]
    unsigned long v12;  // [bp-0x38]
    unsigned long v13;  // [bp-0x30]
    struct_1 *v14;  // [bp-0x28]

    v3 = 0;
    v6 = &v3;
    v14 = a0;
    v7 = &g_40fd80;
    while (1)
    {
        if (!v14 && !a1)
            return v3;
        v13 = 0;
        v12 = v13;
        v11 = v12;
        v10 = v11;
        v1 = (!v14 ? 1 : (!a1 ? 0 : *((long long *)(a1 + 16)) < v14->field_10));
        v0 = v1;
        index = (&v14)[v0];
        v4 = index->field_18;
        (&v12)[v0] = index;
        (&v10)[v0] = (&v12)[v0];
        (&v14)[v0] = *((long long *)&index[2].padding_0[0]);
        *((unsigned long *)((&v12)[v0] + 64)) = 0;
        v2 = v0 ^ 1;
        for (idx = (&v14)[v2]; idx && *((long long *)(idx + 16)) <= v4 + 1; idx = (&v14)[v2])
        {
            if ((&v10)[v2])
                *((unsigned long long *)((&v12)[v2] + 64)) = idx;
            else
                (&v10)[v2] = idx;
            (&v12)[v2] = idx;
            (&v14)[v2] = ((&v14)[v2])[2].field_10;
            *((unsigned long long *)(idx + 64)) = 0;
            if (v4 < *((long long *)(idx + 24)))
            {
                v0 ^= 1;
                v4 = *((long long *)(idx + 24));
            }
            v2 = v0 ^ 1;
        }
        v9 = sub_40393d(&v10, &v12, v1, v0, v7);
        if (!v9)
            sub_405fcf("internal error: screwup in format of diff blocks"); /* do not return */
        *(v6) = v9;
        v6 = v9 + 13;
        v7 = v9;
    }
}



// Function: using_to_diff3_block @ 0x393d
typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
} struct_2;

typedef struct struct_3 {
    char padding_0[64];
    unsigned long long field_40;
} struct_3;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    long long field_20;
    long long field_28;
    long long field_30;
    long long field_38;
    struct struct_0 *field_40;
} struct_0;

void* using_to_diff3_block(struct_1 *a0, unsigned long a1, int a2, int a3, struct_2 *idx)
{
    int node;  // [bp-0xa4]
    struct_3 *iter;  // [bp-0xa0], Other Possible Types: unsigned long
    long long iter1;  // [bp-0x98], Other Possible Types: unsigned long long
    unsigned long long idx2;  // [bp-0x90]
    unsigned long v4;  // [bp-0x88]
    unsigned long v5;  // [bp-0x80]
    void* idx1;  // [bp-0x78]
    unsigned long v7;  // [bp-0x70]
    unsigned long v8;  // [bp-0x68]
    struct_3 *v9;  // [bp-0x60]
    unsigned long v10;  // [bp-0x58]
    unsigned long v11;  // [bp-0x50]
    unsigned long long v12;  // [bp-0x48]
    unsigned long long v13;  // [bp-0x40]
    unsigned long v14;  // [bp-0x38]
    long long v15;  // [bp-0x30]
    unsigned long v16;  // [bp-0x28]
    long long v17;  // [bp-0x20]

    v4 = (&a0->field_0)[a2]->field_10;
    v5 = *((long long *)(*((long long *)(a1 + a3 * 8)) + 24));
    for (node = 0; node <= 1; node += 1)
    {
        if ((&a0->field_0)[node])
        {
            (&v14)[node] = v4 - (&a0->field_0)[node]->field_10 + (&a0->field_0)[node]->field_0;
            (&v16)[node] = v5 - *((long long *)(*((long long *)(a1 + node * 8)) + 24)) + *((long long *)(*((long long *)(a1 + node * 8)) + 8));
        }
        else
        {
            (&v14)[node] = v4 - idx->field_30 + *((long long *)&idx->padding_0[16 + 16 * node]);
            (&v16)[node] = v5 - idx->field_30 + *((long long *)&idx->padding_0[16 + 16 * node]);
        }
    }
    idx1 = sub_404116(v14, v16, v15, v17, v4, v5);
    for (node = 0; node <= 1; node += 1)
    {
        for (iter = (&a0->field_0)[node]; iter; iter = *((long long *)(iter + 64)))
        {
            v13 = *((long long *)(iter + 16)) - v4;
            if ((char)sub_404067(*((long long *)(iter + 40)), *((long long *)(iter + 56)), (long long)idx1[72] + v13 * 8, (long long)idx1[96] + v13 * 8, *((long long *)(iter + 24)) - *((long long *)(iter + 16)) + 1) != 1)
                return NULL;
        }
    }
    for (node = 0; node <= 1; node += 1)
    {
        v9 = (&a0->field_0)[node];
        v10 = (&v14)[node];
        v11 = (&v16)[node];
        for (iter1 = 0; (!v9 ? v11 + 1 : *((long long *)&v9->padding_0[0])) > iter1 + v10; iter1 += 1)
        {
            *((long long *)(*((long long *)((char *)idx1 + 8 * node + 56)) + iter1 * 8)) = *((long long *)((long long)idx1[72] + iter1 * 8));
            *((long long *)(*((long long *)((char *)idx1 + 8 * node + 80)) + iter1 * 8)) = *((long long *)((long long)idx1[96] + iter1 * 8));
        }
        for (iter = v9; iter; iter = iter->field_40)
        {
            v12 = *((long long *)&iter->padding_0[0]) - v10;
            if ((char)sub_404067(*((long long *)&iter->padding_0[32]), *((long long *)&iter->padding_0[48]), *((long long *)((char *)idx1 + 8 * node + 56)) + v12 * 8, *((long long *)((char *)idx1 + 8 * node + 80)) + v12 * 8, *((long long *)&iter->padding_0[8]) - *((long long *)&iter->padding_0[0]) + 1) != 1)
                return NULL;
            idx2 = *((long long *)&iter->padding_0[24]) + 1 - v4;
            for (iter1 = *((long long *)&iter->padding_0[8]) + 1 - v10; iter1 < (!iter->field_40 ? v11 + 1 : *((long long *)iter->field_40)) - v10; iter1 += 1)
            {
                *((long long *)(*((long long *)((char *)idx1 + 8 * node + 56)) + iter1 * 8)) = *((long long *)((long long)idx1[72] + idx2 * 8));
                *((long long *)(*((long long *)((char *)idx1 + 8 * node + 80)) + iter1 * 8)) = *((long long *)((long long)idx1[96] + idx2 * 8));
                idx2 += 1;
            }
        }
    }
    if (!a0->field_0)
    {
        *((unsigned int *)idx1) = 6;
    }
    else if (!a0->field_8)
    {
        *((unsigned int *)idx1) = 5;
    }
    else
    {
        v7 = (long long)idx1[16] - (long long)idx1[8] + 1;
        v8 = (long long)idx1[32] - (long long)idx1[24] + 1;
        if (v7 != v8 || (char)sub_404311((long long)idx1[56], (long long)idx1[80], (long long)idx1[64], (long long)idx1[88], v7) != 1)
            *((unsigned int *)idx1) = 4;
        else
            *((unsigned int *)idx1) = 7;
    }
    return idx1;
}



// Function: copy_stringlist @ 0x4067
unsigned int copy_stringlist(void* *a0, void* a1, void* *a2, void* a3, unsigned long long a4)
{
    void* *v3;  // r13
    void* *iter;  // rbx
    void* v5;  // r12
    void* node;  // r14
    unsigned long long v0;  // [bp-0x50]
    unsigned long long v1;  // [bp-0x50]

    v0 = a4;
    v3 = a0;
    iter = a2;
    v5 = a1;
    node = a3;
    while (1)
    {
        v1 = v0 - 1;
        if (!v0)
            return 1;
        if (!*(iter))
        {
            *(iter) = *(v3);
            *((long long *)node) = *((long long *)v5);
        }
        else if (*((long long *)v5) != *((long long *)node) || memcmp(*(v3), *(iter), *((long long *)v5)))
        {
            break;
        }
        iter += 1;
        v3 += 1;
        node += 8;
        v5 += 8;
        v0 = v1;
    }
    return 0;
}



// Function: create_diff3_block @ 0x4116
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    long long field_38;
    long long field_40;
    long long field_48;
    unsigned long long field_50;
    unsigned long long field_58;
    unsigned long long field_60;
    unsigned long long field_68;
} struct_0;

struct_0 * create_diff3_block(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    long long v3;  // rdx
    long long v4;  // rdx
    long long v5;  // rdx
    struct_0 *idx;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    idx = sub_408e10(112);
    idx->field_0 = 0;
    idx->field_68 = 0;
    idx->field_8 = a0;
    idx->field_10 = a1;
    idx->field_18 = a2;
    idx->field_20 = a3;
    idx->field_28 = a4;
    idx->field_30 = a5;
    v1 = idx->field_10 - idx->field_8 + 1;
    if (v1)
    {
        v3 = sub_40927a(v1, 8);
        idx->field_38 = v3;
        idx->field_50 = sub_40927a(v1, 8, v3);
    }
    else
    {
        idx->field_38 = 0;
        idx->field_50 = 0;
    }
    v1 = idx->field_20 - idx->field_18 + 1;
    if (v1)
    {
        v4 = sub_40927a(v1, 8);
        idx->field_40 = v4;
        idx->field_58 = sub_40927a(v1, 8, v4);
    }
    else
    {
        idx->field_40 = 0;
        idx->field_58 = 0;
    }
    v1 = idx->field_30 - idx->field_28 + 1;
    if (!v1)
    {
        idx->field_48 = 0;
        idx->field_60 = 0;
        return idx;
    }
    v5 = sub_40927a(v1, 8);
    idx->field_48 = v5;
    idx->field_60 = sub_40927a(v1, 8, v5);
    return idx;
}



// Function: compare_line_list @ 0x4311
unsigned int compare_line_list(void* *a0, unsigned long long a1, void* *a2, void* a3, unsigned long long a4)
{
    unsigned long long iter;  // [bp-0x50]
    void* *v1;  // [bp-0x28]
    void* *v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    void* node;  // [bp-0x10]

    iter = a4;
    v1 = a0;
    v2 = a2;
    v3 = a1;
    node = a3;
    do
    {
        iter -= 1;
        if (!iter)
            return 1;
        if (!*(v1) || !*(v2) || !(node += 8, *((long long *)v3) == *((long long *)node)))
            break;
    } while ((v3 += 8, v2 += 8, v1 += 8, iter = iter, node = node, !(int)(long long)memcmp(*(v1), *(v2), *((long long *)v3))));
    return 0;
}



// Function: process_diff @ 0x43e5
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

typedef struct FILE {
} FILE;

extern FILE *stderr;

long long process_diff(long long a0, long long a1, struct_0 **a2, struct_2 **a3)
{
    char *v13;  // rax
    unsigned int v0;  // [bp-0x74]
    char *v1;  // [bp-0x70]
    char *v2;  // [bp-0x68]
    char *node;  // [bp-0x68]
    char v4;  // [bp-0x60]
    long long iter;  // [bp-0x58]
    struct_0 *v6;  // [bp-0x50]
    struct_0 *idx;  // [bp-0x48]
    long long v8;  // [bp-0x40]
    char *v9;  // [bp-0x38]
    long long v10;  // [bp-0x30]
    long long v11;  // [bp-0x28]

    v6 = &v4;
    v8 = 0xfffffffffffffff;
    v9 = sub_404a3a(a0, a1, &v1, a1);
    *(a3) = v1;
    node = v1;
    while (1)
    {
        if (v9 <= node)
        {
            v6->field_0 = NULL;
            *(a2) = idx;
            return v4;
        }
        idx = sub_408e10(72);
        idx->field_20 = 0;
        idx->field_18 = idx->field_20;
        idx->field_30 = 0;
        idx->field_28 = idx->field_30;
        v0 = sub_404887(&node, idx, idx);
        if (!v0 || *(node) != 10)
            break;
        node += 1;
        if (v0 == 3)
        {
            idx->field_8 = idx->field_8 + 1;
        }
        else if (v0 > 3)
        {
            sub_405fcf("internal error: invalid diff type in process_diff"); /* do not return */
        }
        else if (v0 == 1)
        {
            idx->field_0 = (char *)&idx->field_0->field_0 + 1;
        }
        else if (v0 != 2)
        {
            sub_405fcf("internal error: invalid diff type in process_diff"); /* do not return */
        }
        if (v0 != 1)
        {
            v10 = *((long long *)&idx->field_1) - (char *)idx->field_0 + 1;
            if (v8 <= v10)
                sub_4093d4(); /* do not return */
            idx->field_18 = sub_408e10(v10 * 8);
            idx->field_28 = sub_408e10(v10 * 8);
            for (iter = 0; iter < v10; iter += 1)
            {
                node = sub_404edc(node, idx->field_18 + iter * 8, idx->field_28 + iter * 8, v9, 60);
            }
        }
        if (v0 == 2)
        {
            if (strncmp(node, "---\n", 4))
                sub_405fcf("invalid diff format; invalid change separator"); /* do not return */
            node += 4;
        }
        if (v0 != 3)
        {
            v11 = idx->field_10 - idx->field_8 + 1;
            if (v8 <= v11)
                sub_4093d4(); /* do not return */
            idx->field_20 = sub_408e10(v11 * 8);
            idx->field_30 = sub_408e10(v11 * 8);
            for (iter = 0; iter < v11; iter += 1)
            {
                node = sub_404edc(node, idx->field_20 + iter * 8, idx->field_30 + iter * 8, v9, 62);
            }
        }
        v6->field_0 = idx;
        v6 = &idx->field_38;
    }
    v13 = gettext("%s: diff failed: ");
    fprintf(stderr, v13);
    do
    {
        v2 = node;
        putc_unlocked(*(v2), stderr);
        node = v2 + 1;
    } while (*(v2) != 10);
    exit(2); /* do not return */
}



// Function: skipwhite @ 0x47d5
char * skipwhite(char *a0)
{
    char *i;  // [bp-0x10]

    for (i = a0; *(i) == 32 || *(i) == 9; i += 1);
    return i;
}



// Function: readnum @ 0x4804
char * readnum(char *a0, unsigned long long *a1)
{
    char *iter;  // [bp-0x20]
    char v1;  // [bp-0x11]
    unsigned long long v2;  // [bp-0x10]

    iter = a0;
    v1 = *(iter);
    v2 = 0;
    if (v1 - 48 > 9)
        return NULL;
    do
    {
        v2 = v2 * 10 + v1 - 48;
        iter += 1;
        v1 = *(iter);
    } while (v1 - 48 <= 9);
    *(a1) = v2;
    return iter;
}



// Function: process_diff_control @ 0x4887
typedef struct struct_0 {
    char field_0;
} struct_0;

unsigned int process_diff_control(struct_0 **a0, unsigned long long *idx)
{
    unsigned int v3;  // eax
    unsigned int v0;  // [bp-0x1c]
    char *v1;  // [bp-0x18]

    v1 = &*(a0)->field_0;
    v1 = sub_404804(sub_4047d5(v1), idx);
    if (!v1)
        return 0;
    v1 = sub_4047d5(v1);
    if (*(v1) == 44)
    {
        v1 = sub_404804(v1 + 1, idx + 1);
        if (!v1)
            return 0;
    }
    else
    {
        idx[1] = *(idx);
    }
    v1 = sub_4047d5(v1);
    v3 = *(v1);
    switch (v3)
    {
    case 100:
        v0 = 3;
        break;
    case 97:
        v0 = 1;
        break;
    case 99:
        v0 = 2;
        break;
    default:
        return 0;
    }
    v1 += 1;
    v1 = sub_404804(sub_4047d5(v1), idx + 2);
    if (!v1)
        return 0;
    v1 = sub_4047d5(v1);
    if (*(v1) == 44)
    {
        v1 = sub_404804(v1 + 1, idx + 3);
        if (!v1)
            return 0;
        v1 = sub_4047d5(v1);
    }
    else
    {
        idx[3] = idx[2];
    }
    *(a0) = v1;
    return v0;
}



// Function: read_diff @ 0x4a3a
typedef struct struct_0 {
    char field_0;
    char padding_1[23];
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
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

extern char *g_414020;
extern char g_414109;
extern char g_41410a;

long long read_diff(unsigned long a0, unsigned long a1, long long *a2)
{
    unsigned long long v20;  // rax
    unsigned int v22;  // eax
    int v0;  // [bp-0x154]
    int v1;  // [bp-0x150]
    unsigned int v2;  // [bp-0x14c]
    unsigned int v3;  // [bp-0x148]
    unsigned int v4;  // [bp-0x144]
    long long v5;  // [bp-0x140]
    unsigned long long v6;  // [bp-0x138]
    unsigned long long v7;  // [bp-0x130]
    struct_0 *iter;  // [bp-0x128]
    long long v9;  // [bp-0x120]
    long long v10;  // [bp-0x118]
    char v11;  // [bp-0x110]
    int v12;  // [bp-0x10c]
    char *v13;  // [bp-0x108], Other Possible Types: char
    unsigned long v14;  // [bp-0x100]
    stat v16;  // [bp-0xb8]

    v1 = 0;
    iter = &v13;
    iter = &iter->padding_1[7];
    v13 = g_414020;
    if (g_414109)
    {
        iter = &iter->padding_1[7];
        v14 = "-a";
    }
    if (g_41410a)
    {
        iter = &iter->padding_1[7];
        *((char **)&iter->field_0) = "--strip-trailing-cr";
    }
    iter = &iter->padding_1[7];
    *((char **)&iter->field_0) = "--horizon-lines=100";
    iter = &iter->padding_1[7];
    *((char **)&iter->field_0) = "--";
    iter = &iter->padding_1[7];
    *((unsigned long *)&iter->field_0) = a0;
    iter = &iter->padding_1[7];
    *((unsigned long *)&iter->field_0) = a1;
    *((unsigned long long *)&iter->field_0) = 0;
    if (pipe(&v11))
        sub_40600c("pipe"); /* do not return */
    v2 = fork();
    if (!v2)
    {
        close(v11);
        if (v12 != 1)
        {
            dup2(v12, 1);
            close(v12);
        }
        execvp(g_414020, &v13);
        _exit((*(__errno_location()) == 2 ? 127 : 126)); /* do not return */
    }
    if (v2 == 0xffffffff)
        sub_40600c("fork"); /* do not return */
    close(v12);
    v3 = v11;
    if (fstat(v3, &v16))
        sub_40600c("fstat"); /* do not return */
    v20 = v16.st_mtime;
    if (v16.st_mtime <= 0)
        v20 = 1;
    v6 = v20;
    v5 = sub_408e10();
    v7 = 0;
    while (1)
    {
        v9 = v6 - v7;
        v10 = sub_409671(v3, v5 + v7, v9, v5 + v7);
        v7 += v10;
        if (v10 != v9)
            break;
        if (v6 > 0x3ffffffffffffffe)
            sub_4093d4(); /* do not return */
        v6 *= 2;
        v5 = sub_408e5c(v5, v6, v6);
    }
    if (v10 == -0x1)
        sub_40600c(gettext("read failed")); /* do not return */
    if (v7 && *((char *)(v5 + v7 - 1)) != 10)
        sub_405fcf("invalid diff format; incomplete last line"); /* do not return */
    *(a2) = v5;
    if (close(v3))
        sub_40600c("close"); /* do not return */
    if (waitpid(v2, &v0, 0) < 0)
        sub_40600c("waitpid"); /* do not return */
    if (!v1 && !((char)v0 & 127))
        v22 = v0 >> 8 & 0xff;
    else
        v22 = 0x7fffffff;
    v4 = v22;
    if (v4 <= 1)
        return v7 + v5;
    error(2, v1, gettext((v4 == 126 ? "subsidiary program '%s' could not be invoked" : (v4 == 127 ? "subsidiary program '%s' not found" : (v4 == 0x7fffffff ? "subsidiary program '%s' failed" : "subsidiary program '%s' failed (exit status %d)")))));
}



// Function: scan_diff_line @ 0x4edc
typedef struct struct_0 {
    char padding_0[1];
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char g_41410b;
extern char *g_424148;

struct_0 * scan_diff_line(struct_0 *a0, struct_0 **a1, unsigned long long *a2, struct_0 *a3, char a4)
{
    struct_0 *iter;  // [bp-0x10]

    if (a4 == a0->padding_0 && a0->field_1 == 32)
    {
        iter = &a0->field_2;
        *(a1) = iter;
        do
        {
            iter = &iter->field_1;
        } while (iter->padding_0 != 10);
        *(a2) = iter - *(a1);
        if (iter >= a3)
        {
            return iter;
        }
        else if (iter->padding_0 == 92)
        {
            if (g_41410b)
                fprintf(stderr, "%s:", g_424148);
            else
                *(a2) = *(a2) - 1;
            iter = &iter->field_1;
            do
            {
                if (g_41410b)
                    putc_unlocked((char)iter->padding_0, stderr);
            } while ((iter += 1, iter->padding_0 != 10));
            return iter;
        }
        else
        {
            return iter;
        }
    }
    sub_405fcf("invalid diff format; incorrect leading line chars"); /* do not return */
}



// Function: output_diff3 @ 0x5012
typedef struct FILE {
} FILE;

extern unsigned int g_414010[4];
extern char g_41410d;

int output_diff3(FILE *ptr, void* a1, unsigned long a2, unsigned long a3)
{
    unsigned int v17;  // eax
    unsigned long long v18;  // rax
    unsigned long v20;  // fs
    unsigned int *i;  // [bp-0x70]
    unsigned int v1;  // [bp-0x6c]
    unsigned int *v2;  // [bp-0x68]
    unsigned int v3;  // [bp-0x64]
    void* iter;  // [bp-0x60]
    long long idx;  // [bp-0x58]
    char *v6;  // [bp-0x50]
    unsigned long v7;  // [bp-0x48]
    unsigned long v8;  // [bp-0x40]
    unsigned long v9;  // [bp-0x38]
    unsigned long v10;  // [bp-0x30]
    unsigned long v11;  // [bp-0x28]
    unsigned long count;  // [bp-0x20]
    char v13;  // [bp-0x12]
    char v14;  // [bp-0x11]
    unsigned long v15;  // [bp-0x10]

    v6 = (!g_41410d ? "  " : "\t");
    iter = a1;
    while (1)
    {
        if (!iter)
            return v15 - *((long long *)(40 + v20));
        v17 = *((int *)iter);
        if (v17 != 4)
        {
            if (v17 < 4 || v17 - 5 > 2)
                break;
            v1 = *((int *)(a3 + (*((int *)iter) - 5) * 4));
            v13 = (char)v1 + 49;
            v14 = 0;
            v2 = (unsigned int *)(!v1);
        }
        else
        {
            v13 = 0;
            v2 = 0x3;
            v1 = 3;
        }
        fprintf(ptr, "====%s\n", &v13);
        for (i = NULL; i <= 2; i = (unsigned int *)(v1 == 1 ? *((int *)(0x4 * i + (char *)&g_414010[0])) : (unsigned int *)((char *)i + 1)))
        {
            v3 = *((int *)(a2 + (long long)i * 4));
            v7 = *((long long *)(16 * v3 + (char *)iter + 8));
            v8 = *((long long *)(16 * v3 + (char *)iter + 16));
            v9 = v7;
            v10 = v8;
            fprintf(ptr, "%d:", (unsigned int *)((char *)i + 1));
            v18 = v7 - v8;
            if (!v18)
            {
                fprintf(ptr, "%ldc\n", v9);
            }
            else if (v18 == 1)
            {
                fprintf(ptr, "%lda\n", v9 - 1);
            }
            else
            {
                fprintf(ptr, "%ld,%ldc\n", v9, v10);
            }
            if (i != v2 && v7 <= v8)
            {
                idx = 0;
                do
                {
                    fputs_unlocked(v6, ptr);
                    v11 = *((long long *)(idx * 8 + *((long long *)((char *)iter + 8 * v3 + 56))));
                    count = *((long long *)(idx * 8 + *((long long *)((char *)iter + 8 * v3 + 80))));
                    fwrite_unlocked(v11, 1, count, ptr);
                    idx += 1;
                } while (idx <= v8 - v7);
                if (*((char *)(v11 + count - 1)) != 10)
                    fprintf(ptr, "\n\\ %s\n", gettext("No newline at end of file"));
            }
        }
        iter = (long long)iter[104];
    }
    sub_405fcf("internal error: invalid diff type passed to output"); /* do not return */
}



// Function: dotlines @ 0x5369
typedef struct FILE {
} FILE;

int dotlines(FILE *a0, unsigned long a1, int a2)
{
    char v0;  // [bp-0x19]
    long long idx;  // [bp-0x18]
    char *v2;  // [bp-0x10]

    v0 = 0;
    for (idx = 0; idx <= *((long long *)(a2 * 16 + a1 + 16)) - *((long long *)(a2 * 16 + a1 + 8)); idx += 1)
    {
        v2 = *((long long *)(idx * 8 + *((long long *)(a1 + (a2 + 6) * 8 + 8))));
        if (*(v2) == 46)
        {
            v0 = 1;
            fputc_unlocked(46, a0);
        }
        fwrite_unlocked(v2, 1, *((long long *)(idx * 8 + *((long long *)(a1 + (a2 + 10) * 8)))), a0);
    }
    return v0;
}



// Function: undotlines @ 0x544f
typedef struct FILE {
} FILE;

int undotlines(FILE *a0, char a1, int a2, unsigned long long a3)
{
    int v1;  // eax

    v1 = fputs_unlocked(".\n", a0);
    if (!a1)
        return v1;
    return (a3 == 1 ? fprintf(a0, "%lds/^\\.//\n", a2) : fprintf(a0, "%ld,%lds/^\\.//\n", a2, a3 + a2 - 1));
}



// Function: output_diff3_edscript @ 0x54de
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[100];
    struct struct_0 *field_68;
} struct_0;

extern char g_41410c;
extern char g_41410e;
extern char g_41410f;
extern char g_414110;
extern char g_414111;

int output_diff3_edscript(FILE *ptr, long long a1, int *a2, unsigned long a3, char *a4, char *a5, char *a6)
{
    char *v9;  // rax
    unsigned long long v10;  // rax
    char v11;  // al
    char *v0;  // [bp-0x68]
    char *v1;  // [bp-0x60]
    char v2;  // [bp-0x37]
    char v3;  // [bp-0x36]
    char v4;  // [bp-0x35]
    unsigned int v5;  // [bp-0x34]
    struct_0 *iter;  // [bp-0x30]
    unsigned long v7;  // [bp-0x28]
    unsigned long v8;  // [bp-0x20]

    v1 = a4;
    v0 = a5;
    v3 = 0;
    iter = sub_405f95(a1);
    while (iter)
    {
        v5 = (iter->field_0 == 4 ? 4 : *((int *)(a3 + (iter->field_0 - 5) * 4)) + 5);
        switch (v5)
        {
        case 7:
            if (g_41410f)
                break;
            v4 = 0;
            goto LABEL_4055b7;
        case 4:
            if (g_41410e)
                break;
            v4 = g_41410c;
LABEL_4055b7:
            v7 = *((long long *)&iter->padding_4[4 + 16 * *(a2)]);
            v8 = *((long long *)&iter->padding_4[12 + 16 * *(a2)]);
            if (v4)
            {
                v3 = 1;
                fprintf(ptr, "%lda\n", v8);
                v2 = 0;
                if (v5 == 4)
                {
                    if (g_414110)
                    {
                        fprintf(ptr, "||||||| %s\n", v0);
                        v2 = sub_405369(ptr, iter, a2[1]);
                    }
                    fputs_unlocked("=======\n", ptr);
                    v2 = (char)sub_405369(ptr, iter, a2[2]) || v2;
                }
                fprintf(ptr, ">>>>>>> %s\n", a6);
                sub_40544f(ptr, v2, v8 + 2, *((long long *)&iter->padding_4[12 + 16 * a2[2]]) - *((long long *)&iter->padding_4[4 + 16 * a2[2]]) + 1 + *((long long *)&iter->padding_4[12 + 16 * a2[1]]) - *((long long *)&iter->padding_4[4 + 16 * a2[1]]) + 1 + 1);
                if (v5 == 4)
                    v9 = v1;
                else
                    v9 = v0;
                fprintf(ptr, "%lda\n<<<<<<< %s\n", v7 - 1, v9);
                v2 = 0;
                if (v5 == 6)
                {
                    v2 = sub_405369(ptr, iter, a2[1]);
                    fputs_unlocked("=======\n", ptr);
                }
                sub_40544f(ptr, v2, v7 + 1, *((long long *)&iter->padding_4[12 + 16 * a2[1]]) - *((long long *)&iter->padding_4[4 + 16 * a2[1]]) + 1);
                break;
            }
            else if (*((long long *)&iter->padding_4[12 + 16 * a2[2]]) - *((long long *)&iter->padding_4[4 + 16 * a2[2]]) != 0xffffffffffffffff)
            {
                v10 = v8 - v7;
                if (v10 != 0xffffffffffffffff)
                {
                    if (!v10)
                        fprintf(ptr, "%ldc\n", v8);
                    else
                        fprintf(ptr, "%ld,%ldc\n", v7, v8);
                }
                else
                {
                    fprintf(ptr, "%lda\n", v8);
                }
                v11 = sub_405369(ptr, iter, a2[2]);
                sub_40544f(ptr, v11, v7, *((long long *)&iter->padding_4[12 + 16 * a2[2]]) - *((long long *)&iter->padding_4[4 + 16 * a2[2]]) + 1);
                break;
            }
            else if (v7 == v8)
            {
                fprintf(ptr, "%ldd\n", v7);
                break;
            }
            else
            {
                fprintf(ptr, "%ld,%ldd\n", v7, v8);
                break;
            }
        case 6:
            if (g_414110 ^ 1)
                break;
            v4 = 1;
            goto LABEL_4055b7;
        default:
            iter = iter->field_68;
            continue;
        }
    }
    if (g_414111)
        fputs_unlocked("w\nq\n", ptr);
    return v3;
}



// Function: output_diff3_merge @ 0x5a29
typedef struct FILE {
} FILE;

extern char g_41410c;
extern char g_41410e;
extern char g_41410f;
extern char g_414110;

long long output_diff3_merge(FILE *a0, FILE *ptr, void* a2, int *a3, unsigned long a4, char *a5, unsigned long a6, char *a7)
{
    unsigned int v10;  // eax
    char v0;  // [bp-0x42]
    char v1;  // [bp-0x41]
    unsigned int v2;  // [bp-0x40]
    unsigned int v3;  // [bp-0x3c]
    unsigned long node;  // [bp-0x38], Other Possible Types: unsigned long long, long long
    void* iter1;  // [bp-0x30]
    unsigned long long iter;  // [bp-0x28]
    char *v7;  // [bp-0x20]

    v0 = 0;
    iter = 0;
    iter1 = a2;
    while (iter1)
    {
        v3 = (*((int *)iter1) == 4 ? 4 : *((int *)(a4 + (*((int *)iter1) - 5) * 4)) + 5);
        v7 = "<<<<<<< %s\n";
        switch (v3)
        {
        case 7:
            if (g_41410f)
                break;
            v1 = 0;
            goto LABEL_405b18;
        case 4:
            if (g_41410e)
                break;
            v1 = g_41410c;
            v7 = "||||||| %s\n";
LABEL_405b18:
            node = (long long)iter1[8] - iter - 1;
            iter += node;
            while (1)
            {
                node -= 1;
                if (node - 0 >> 63 & 1)
                    break;
                do
                {
                    v2 = getc_unlocked(a0);
                    if (v2 == 0xffffffff)
                    {
                        if (ferror_unlocked(a0))
                        {
                            sub_40600c(gettext("read failed")); /* do not return */
                        }
                        else if (feof_unlocked(a0))
                        {
                            sub_405fcf("input file shrank"); /* do not return */
                        }
                    }
                } while ((putc_unlocked(v2, ptr), v2 != 10));
            }
            if (v1)
            {
                v0 = 1;
                if (v3 == 4)
                {
                    fprintf(ptr, "<<<<<<< %s\n", a5);
                    for (node = 0; node <= *((long long *)(16 * *(a3) + (char *)iter1 + 16)) - *((long long *)(16 * *(a3) + (char *)iter1 + 8)); node += 1)
                    {
                        fwrite_unlocked(*((long long *)(node * 8 + *((long long *)((char *)iter1 + 8 * *(a3) + 56)))), 1, *((long long *)(node * 8 + *((long long *)((char *)iter1 + 8 * *(a3) + 80)))), ptr);
                    }
                }
                if (g_414110)
                {
                    fprintf(ptr, v7);
                    for (node = 0; node <= *((long long *)(16 * a3[1] + (char *)iter1 + 16)) - *((long long *)(16 * a3[1] + (char *)iter1 + 8)); node += 1)
                    {
                        fwrite_unlocked(*((long long *)(node * 8 + *((long long *)((char *)iter1 + 8 * a3[1] + 56)))), 1, *((long long *)(node * 8 + *((long long *)((char *)iter1 + 8 * a3[1] + 80)))), ptr);
                    }
                }
                fputs_unlocked("=======\n", ptr);
            }
            for (node = 0; node <= *((long long *)(16 * a3[2] + (char *)iter1 + 16)) - *((long long *)(16 * a3[2] + (char *)iter1 + 8)); node += 1)
            {
                fwrite_unlocked(*((long long *)(node * 8 + *((long long *)((char *)iter1 + 8 * a3[2] + 56)))), 1, *((long long *)(node * 8 + *((long long *)((char *)iter1 + 8 * a3[2] + 80)))), ptr);
            }
            if (v1)
                fprintf(ptr, ">>>>>>> %s\n", a7);
            node = (long long)iter1[16] - (long long)iter1[8] + 1;
            iter += node;
            while (1)
            {
                node -= 1;
                if (node - 0 >> 63 & 1)
                    break;
                while (1)
                {
                    v2 = getc_unlocked(a0);
                    if (v2 == 10)
                        break;
                    if (v2 == 0xffffffff)
                    {
                        if (ferror_unlocked(a0))
                        {
                            sub_40600c(gettext("read failed")); /* do not return */
                        }
                        else if (feof_unlocked(a0))
                        {
                            if (!node && !(long long)iter1[104])
                                return v0;
                            sub_405fcf("input file shrank"); /* do not return */
                        }
                    }
                }
            }
            break;
        case 6:
            if (g_414110 ^ 1)
                break;
            v1 = 1;
            goto LABEL_405b18;
        default:
            iter1 = (long long)iter1[104];
            continue;
        }
    }
    while (1)
    {
        v2 = getc_unlocked(a0);
        if (v2 == 0xffffffff && !(v10 = (unsigned int)(int)(long long)ferror_unlocked(a0), !(int)(long long)feof_unlocked(a0) && !(int)(long long)ferror_unlocked(a0)))
            break;
        putc_unlocked(v2, ptr);
    }
    return v0;
}



// Function: reverse_diff3_blocklist @ 0x5f95
typedef struct struct_0 {
    char padding_0[104];
    struct struct_0 *field_68;
} struct_0;

struct_0 * reverse_diff3_blocklist(struct_0 *a0)
{
    struct_0 *v1;  // rbx
    struct_0 *v2;  // r12
    struct_0 *idx;  // rbx

    v1 = a0;
    v2 = NULL;
    while (1)
    {
        idx = v1;
        if (!idx)
            break;
        v1 = idx->field_68;
        idx->field_68 = v2;
        v2 = idx;
    }
    return v2;
}



// Function: fatal @ 0x5fcf
void fatal(char *a0)
{
    gettext(a0);
    error(2, 0, "%s");
}



// Function: perror_with_exit @ 0x600c
void perror_with_exit(unsigned long long a0)
{
    int *err;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    err = __errno_location();
    error(2, *(err), "%s");
}



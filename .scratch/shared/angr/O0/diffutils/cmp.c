// Function: try_help @ 0x29e9
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



// Function: specify_ignore_initial @ 0x2a5f
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_410200[4];

void specify_ignore_initial(int a0, struct_0 **a1, char a2)
{
    unsigned int v0;  // [bp-0x24]
    unsigned long long *v1;  // [bp-0x20]
    char *v2;  // [bp-0x18]

    v2 = &*(a1)->field_0;
    v0 = sub_407e2e(v2, a1, 0, &v1, "kKMGTPEZY0");
    switch (v0)
    {
    case 0:
        if (v1 < NULL)
            sub_4029e9("invalid --ignore-initial value '%s'", v2); /* do not return */
        if (g_410200[a0] >= v1)
            return;
        g_410200[a0] = v1;
        break;
        return;
    case 2:
        if (a2 != *(a1)->field_0)
            sub_4029e9("invalid --ignore-initial value '%s'", v2); /* do not return */
        break;
    default:
        sub_4029e9("invalid --ignore-initial value '%s'", v2); /* do not return */
    }
}



// Function: specify_comparison_type @ 0x2b46
extern unsigned int g_410210;

unsigned int specify_comparison_type(unsigned int a0)
{
    if (g_410210 && a0 != g_410210)
        sub_4029e9("options -l and -s are incompatible", 0); /* do not return */
    g_410210 = a0;
    return a0;
}



// Function: check_stdout @ 0x2b8a
typedef struct FILE {
} FILE;

extern FILE *stdout;

int check_stdout(void)
{
    unsigned int v1;  // eax
    int *err;  // rax

    if (ferror_unlocked(stdout))
    {
        gettext("write failed");
        error(2, 0, "%s");
    }
    v1 = fclose(stdout);
    if (!v1)
        return v1;
    gettext("standard output");
    err = __errno_location();
    error(2, *(err), "%s");
}



// Function: usage @ 0x2c28
typedef struct FILE {
} FILE;

extern char *g_40fb40;
extern FILE *stdout;

int usage(void)
{
    long long v12;  // rdx
    long long v13;  // rcx
    long long v14;  // r8
    long long v15;  // r9
    char *v5;  // rax
    char *v7;  // rax
    char *v8;  // rax
    long long v10;  // rdi
    long long v11;  // rsi
    char **i;  // [bp-0x20]

    printf(gettext("Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n"));
    printf("%s\n", gettext("Compare two files byte by byte."));
    printf("\n%s\n\n", gettext("The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default)."));
    v5 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
    fputs_unlocked(v5, stdout);
    for (i = &g_40fb40; *(i); i += 1)
    {
        printf("  %s\n", gettext(*(i)));
    }
    v7 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
    v8 = gettext("If a FILE is '-' or missing, read standard input.");
    printf("\n%s\n\n%s\n%s\n", gettext("SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y."), v8, v7);
    return sub_4074c0(v10, v11, v12, v13, v14, v15);
}



// Function: main @ 0x2d6d
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

typedef struct struct_0 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_40c5be;
extern long long g_40c60c;
extern option g_40fa00;
extern unsigned long long g_410010;
extern long long g_410018;
extern unsigned int g_410020;
extern long long stdout;
extern int optind;
extern unsigned long long optarg;
extern struct_0 g_4100a0;
extern char *g_4100a8;
extern unsigned int g_4100b0[4];
extern unsigned long long g_4100c0;
extern unsigned long long g_4100c8;
extern unsigned long long g_4100d0;
extern unsigned int g_4100d8;
extern unsigned int g_4100dc;
extern unsigned int g_4100e0;
extern unsigned long long g_4100e8;
extern unsigned long long g_4100f0;
extern long long g_4100f8;
extern unsigned long long g_410118;
extern unsigned long long g_410128;
extern char g_410150;
extern char g_410158;
extern char g_410160;
extern char g_410168;
extern char g_41016c;
extern char g_410170;
extern char g_410178;
extern char g_410180;
extern long long g_410188;
extern char g_4101a8;
extern char g_4101b8;
extern long long g_4101e0;
extern unsigned long long g_4101e8;
extern unsigned long long g_4101f0;
extern char g_410200;
extern long long g_410208;
extern unsigned int g_410210;
extern char g_410214;

unsigned int main(int a0, void* a1)
{
    long long v12;  // rdi
    long long v13;  // rsi
    int v22;  // eax
    unsigned long v23;  // rax
    int v24;  // eax
    int *err;  // rax
    int v26;  // eax
    unsigned long long v27;  // rax
    int v30;  // eax
    unsigned long long v31;  // rax
    long long v14;  // rdx
    unsigned long long v32;  // rax
    int *err1;  // rax
    long long v15;  // rcx
    long long v16;  // r8
    long long v17;  // r9
    unsigned long v18;  // rax
    long long v19;  // rax
    long long v20;  // rax
    int v21;  // eax
    int i;  // [bp-0x17c]
    int j;  // [bp-0x178]
    int k;  // [bp-0x174]
    unsigned int v3;  // [bp-0x170]
    unsigned int v4;  // [bp-0x16c]
    char v5;  // [bp-0x168], Other Possible Types: unsigned long
    unsigned long v6;  // [bp-0x160]
    unsigned long v7;  // [bp-0x158]
    unsigned long long v8;  // [bp-0x150]
    stat v9;  // [bp-0x148]
    stat v10;  // [bp-0xb8]

    g_410020 = 2;
    sub_404a4b(*((long long *)a1));
    setlocale(6, &g_40c5be);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_40479f(0);
    sub_407c09(v12, v13, v14, v15, v16, v17);
    while (1)
    {
        v3 = getopt_long(a0, a1, "bci:ln:sv", &g_40fa00.name, NULL);
        if (v3 == 0xffffffff)
            break;
        switch (v3)
        {
        case 98: case 99:
            g_410214 = 1;
            break;
        case 105:
            sub_402a5f(0, &optarg, 58);
            v18 = optarg;
            optarg = v18 + 1;
            if (*((char *)v18) == 58)
            {
                sub_402a5f(1, &optarg, 0);
                break;
            }
            else if (g_410208 < *((long long *)&g_410200))
            {
                g_410208 = *((long long *)&g_410200);
                break;
            }
        case 108:
            sub_402b46(1);
            break;
        case 110:
            if (sub_407e2e(optarg, 0, 0, &v5, "kKMGTPEZY0") || v5 < 0)
            {
                sub_4029e9("invalid --bytes value '%s'", optarg); /* do not return */
            }
            else if (g_410010 < 0 || g_410010 >= v5)
            {
                g_410010 = v5;
                break;
            }
        case 115:
            sub_402b46(3);
            break;
        case 118:
            v19 = sub_40500c("David MacKenzie");
            v20 = sub_4050b2("Torbjorn Granlund", &g_40c60c);
            sub_4073dc(stdout, "cmp", "GNU diffutils", g_410018, v20, v19, 0);
            sub_402b8a();
            return 0;
        case 128:
            sub_402c28();
            sub_402b8a();
            return 0;
        default:
            sub_4029e9(NULL, 0); /* do not return */
        }
    }
    if (a0 == optind)
        sub_4029e9("missing operand after '%s'", *((long long *)((char *)a1 + 8 * a0 - 8))); /* do not return */
    v21 = optind;
    optind = v21 + 1;
    g_4100a0.field_0 = *((long long *)((char *)a1 + 8 * v21));
    if (a0 > optind)
    {
        v22 = optind;
        optind = v22 + 1;
        v23 = *((long long *)((char *)a1 + 8 * v22));
    }
    else
    {
        v23 = "-";
    }
    g_4100a8 = v23;
    for (i = 0; i <= 1 && a0 > optind; i += 1)
    {
        v24 = optind;
        optind = v24 + 1;
        v5 = *((long long *)((char *)a1 + 8 * v24));
        sub_402a5f(i, &v5, 0);
    }
    if (a0 > optind)
        sub_4029e9("extra operand '%s'", *((long long *)((char *)a1 + 8 * optind))); /* do not return */
    j = 0;
    while (j <= 1)
    {
        if (j && *((long long *)&g_410200) == g_410208 && !strcmp(g_4100a0.field_0, g_4100a8))
            return 0;
        if (!strcmp((&g_4100a0.field_0)[j], "-"))
            g_4100b0[j] = 0;
        else
            g_4100b0[j] = open((&g_4100a0.field_0)[j], 0, 0);
        if (g_4100b0[j] < 0 || fstat(g_4100b0[j], &(&g_4100c0)[18 * j]))
        {
            if (g_4100b0[j] < 0 && g_410210 == 3)
                exit(2); /* do not return */
            err = __errno_location();
            error(2, *(err), "%s");
        }
        else
        {
            j += 1;
        }
    }
    if (g_4100c8 == *((long long *)&g_410158) && g_4100c0 == *((long long *)&g_410150) || (((unsigned short)g_4100d8 & 0xf000) == 0x6000 && ((short)*((int *)&g_410168) & 0xf000) == 0x6000 || ((unsigned short)g_4100d8 & 0xf000) == 0x2000 && ((short)*((int *)&g_410168) & 0xf000) == 0x2000) && g_4100e8 == *((long long *)&g_410178))
        v26 = 1;
    else
        v26 = 0;
    if (v26 > 0 && g_4100d8 == *((int *)&g_410168) && g_4100d0 == *((long long *)&g_410160) && g_4100dc == *((int *)&g_41016c) && g_4100e0 == *((int *)&g_410170) && g_4100f0 == *((long long *)&g_410180) && g_410118 == *((long long *)&g_4101a8) && g_410128 == *((long long *)&g_4101b8))
    {
        v27 = sub_40447f(0);
        if (v27 == sub_40447f(1))
            return 0;
    }
    if (g_410210 != 3 && !fstat(1, &v9) && !stat("/dev/null", &v10))
    {
        if (v9.st_ino == v10.st_ino && *((unsigned long long *)&v9) == *((unsigned long long *)&v10) || (((unsigned short)v9.st_nlink & 0xf000) == 0x6000 && ((unsigned short)v10.st_nlink & 0xf000) == 0x6000 || ((unsigned short)v9.st_nlink & 0xf000) == 0x2000 && ((unsigned short)v10.st_nlink & 0xf000) == 0x2000) && v9.st_size == v10.st_size)
            v30 = 1;
        else
            v30 = 0;
        if (v30 > 0)
            g_410210 = 2;
    }
    if (g_410210 == 3 && ((unsigned short)g_4100d8 & 0xf000) == 0x8000 && ((short)*((int *)&g_410168) & 0xf000) == 0x8000)
    {
        v31 = sub_40447f(0);
        v6 = g_4100f0 - v31;
        v32 = sub_40447f(1);
        v7 = *((long long *)&g_410180) - v32;
        if (v6 - 0 >> 63 & 1)
            v6 = 0;
        if (v7 - 0 >> 63 & 1)
            v7 = 0;
        if (v6 != v7)
        {
            if (g_410010 >= 0)
            {
                if (v7 < v6)
                    v6 = v7;
                if (v6 < g_410010)
                    exit(1); /* do not return */
            }
            else
            {
                exit(1); /* do not return */
            }
        }
    }
    g_4101f0 = sub_408307(g_4100f8, g_410188, 0x7ffffffffffffff7, g_4100f8);
    v8 = g_4101f0 + 15 >> 3;
    g_4101e0 = sub_407605(v8 * 16);
    g_4101e8 = v8 * 8 + g_4101e0;
    v4 = sub_4037d2(v12, v13, g_4101e0, v15, v16, v17);
    k = 0;
    while (k <= 1)
    {
        if (close(g_4100b0[k]))
        {
            err1 = __errno_location();
            error(2, *(err1), "%s");
        }
        k += 1;
    }
    if (v4 && g_410210 <= 1)
        sub_402b8a();
    exit(v4); /* do not return */
}



// Function: cmp @ 0x37d2
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

extern struct_0 *g_410010;
extern FILE *stderr;
extern unsigned long long g_4100a0[4];
extern unsigned int g_4100b0;
extern unsigned int g_4100b4;
extern unsigned int g_4100d8[4];
extern unsigned long long g_4100f0[4];
extern unsigned long long g_4101e0;
extern unsigned long long g_4101e8;
extern char g_4101f0;
extern unsigned long long g_410200[4];
extern unsigned int g_410210;
extern char g_410214;

unsigned int cmp(void)
{
    unsigned long long v41;  // rax
    unsigned long long *v42;  // rax
    char *v51;  // rax
    int *err;  // rax
    unsigned long long *v44;  // rax
    int *err1;  // rax
    int *err2;  // rax
    char v47;  // al
    char *v49;  // rax
    char *v50;  // rax
    char *v0;  // [bp-0x178]
    unsigned long v1;  // [bp-0x170]
    char *v2;  // [bp-0x168]
    char v3;  // [bp-0x14a]
    char v4;  // [bp-0x149]
    char v5;  // [bp-0x148]
    char v6;  // [bp-0x147]
    char v7;  // [bp-0x146]
    char v8;  // [bp-0x145]
    int v9;  // [bp-0x144]
    unsigned int iter;  // [bp-0x140]
    unsigned int v11;  // [bp-0x13c]
    long long v12;  // [bp-0x138]
    long long v13;  // [bp-0x130]
    unsigned long long *v14;  // [bp-0x128]
    unsigned long long *v15;  // [bp-0x120]
    unsigned long long *v16;  // [bp-0x118]
    unsigned long long *v17;  // [bp-0x110]
    unsigned long long *v18;  // [bp-0x108]
    unsigned long v19;  // [bp-0x100]
    unsigned long v20;  // [bp-0xf8]
    unsigned long v21;  // [bp-0xf0]
    unsigned long v22;  // [bp-0xe8]
    unsigned long long *v23;  // [bp-0xe0]
    unsigned long long *v24;  // [bp-0xd8]
    unsigned long long *v25;  // [bp-0xd0]
    unsigned long long *v26;  // [bp-0xc8]
    unsigned long long v27;  // [bp-0xc0]
    unsigned long long v28;  // [bp-0xb8]
    unsigned long long v29;  // [bp-0xb0]
    char *v30;  // [bp-0xa8]
    unsigned long long *v31;  // [bp-0xa0]
    unsigned long long v32;  // [bp-0x98]
    unsigned long long v33;  // [bp-0x90]
    unsigned long long *v34;  // [bp-0x88]
    unsigned long long *v35;  // [bp-0x80]
    char v36;  // [bp-0x72]
    char v37;  // [bp-0x6d]
    char v38;  // [bp-0x68]
    char v39;  // [bp-0x48]

    v3 = 1;
    v12 = 1;
    v13 = 1;
    v14 = g_410010;
    v19 = g_4101e0;
    v20 = g_4101e8;
    v21 = v19;
    v22 = v20;
    v9 = 0;
    if (g_410210 == 1)
    {
        v16 = (g_410010 < NULL ? 0x7fffffffffffffff : g_410010);
        for (iter = 0; iter <= 1; iter += 1)
        {
            if (((unsigned short)g_4100d8[36 * iter] & 0xf000) == 0x8000)
            {
                v41 = sub_40447f(iter);
                v23 = g_4100f0[18 * iter] - v41;
                if (v23 < v16)
                    v16 = v23;
            }
        }
        v11 = 1;
        while (1)
        {
            v16 = ((long long)(v16 * 7378697629483820647) >> 66) - (v16 >> 63);
            if (!v16)
                break;
            v11 += 1;
        }
    }
    for (iter = 0; iter <= 1; iter += 1)
    {
        v17 = g_410200[iter];
        if (v17 && sub_40447f(iter) == 0xffffffffffffffff)
        {
            do
            {
                v42 = *((long long *)&g_4101f0);
                if (v17 <= *((long long *)&g_4101f0))
                    v42 = v17;
                v34 = v42;
                v35 = sub_408238();
                if (v35 != v34)
                {
                    if (v35 == 0xffffffffffffffff)
                    {
                        err = __errno_location();
                        error(2, *(err), "%s");
                    }
                    break;
                }
            } while ((v17 = (unsigned long long *)(v17 - v35), v17));
        }
    }
    while (1)
    {
        v18 = *((long long *)&g_4101f0);
        if (!(v14 >> 63 & 1))
        {
            if (v18 > v14)
                v18 = v14;
            v14 -= v18;
        }
        v24 = sub_408238(g_4100b0, v21, v18, v21);
        if (v24 == 0xffffffffffffffff)
        {
            err1 = __errno_location();
            error(2, *(err1), "%s");
        }
        v25 = sub_408238(g_4100b4, v22, v18, v22);
        if (v25 == 0xffffffffffffffff)
        {
            err2 = __errno_location();
            error(2, *(err2), "%s");
        }
        if (v24 <= v25)
            v44 = v24;
        else
            v44 = v25;
        v26 = v44;
        if (!memcmp(v21, v22, v26))
        {
            v15 = v26;
        }
        else
        {
            if (v24 >= v25)
                *((char *)v24 + v22) = 0x55;
            if (v25 >= v24)
                *((char *)v25 + v21) = 121;
            *((char *)v24 + v21) = ~(*((char *)v24 + v22));
            *((char *)v25 + v22) = ~(*((char *)v25 + v21));
            memset(v21 + (char *)v24 + 1, 0, (unsigned int)~(v24) & 7);
            memset(v22 + (char *)v25 + 1, 0, (unsigned int)~(v25) & 7);
            v15 = sub_4042d7(v19, v20, v20);
        }
        v13 += (char *)v15;
        if (!g_410210 && v15)
        {
            v12 += sub_404350(v21, v15, v15);
            v3 = *(v21 + (char *)v15 - 1) == 10;
        }
        if (v15 < v26)
        {
            if (g_410210 == 3)
                return 1;
            if (g_410210 <= 3)
            {
                if (g_410210 == 2)
                {
                    v9 = 1;
                }
                else if (g_410210 <= 2)
                {
                    if (!g_410210)
                        break;
                    if (g_410210 == 1)
                    {
                        do
                        {
                            v4 = *((char *)v15 + v21);
                            v5 = *((char *)v15 + v22);
                            if (v4 != v5)
                            {
                                v27 = sub_40490d(v13, &v39, &v39);
                                if (g_410214 != 1)
                                {
                                    printf("%*s %3o %3o\n", v11, (unsigned int)v27);
                                }
                                else
                                {
                                    sub_4043cf(&v37, v4, v4);
                                    sub_4043cf(&v38, v5, v5);
                                    v2 = &v38;
                                    printf("%*s %3o %-4s %3o %s\n", v11, (unsigned int)v27, v4);
                                }
                            }
                        } while ((v13 = (long long)(v13 + 1), v15 += 1, v15 < v26));
                        v9 = -0x1;
                    }
                }
            }
        }
        if (v24 != v25)
        {
            if (v9 > 0 || g_410210 == 3)
                return 1;
            v31 = g_4100a0[v25 < v24];
            if (v13 == 1)
            {
                v49 = gettext("cmp: EOF on %s which is empty\n");
                fprintf(stderr, v49);
            }
            else
            {
                v32 = sub_40490d(v13 - 1, &v38, v13 - 1);
                if (!g_410210)
                {
                    v33 = sub_40490d(v12 - v3, &v39, v12 - v3, v3);
                    v50 = (!v3 ? gettext("cmp: EOF on %s after byte %s, in line %s\n") : gettext("cmp: EOF on %s after byte %s, line %s\n"));
                    fprintf(stderr, v50);
                }
                else
                {
                    v51 = gettext("cmp: EOF on %s after byte %s\n");
                    fprintf(stderr, v51);
                }
            }
            return 1;
        }
        else if (v9 > 0 || v24 != *((long long *)&g_4101f0))
        {
            return v9;
        }
    }
    v28 = sub_40490d(v13, &v38, &v38);
    v29 = sub_40490d(v12, &v39, &v39);
    if (g_410214 == 1)
    {
        v6 = *((char *)v15 + v21);
        v7 = *((char *)v15 + v22);
        sub_4043cf(&v36, v6, v6);
        sub_4043cf(&v37, v7, v7);
        v2 = &v37;
        v1 = v7;
        v0 = &v36;
        printf(gettext("%s %s differ: byte %s, line %s is %3o %s %3o %s\n"));
        return 1;
    }
    v30 = gettext("%s %s differ: byte %s, line %s\n");
    if (v30 != "%s %s differ: byte %s, line %s\n" || (char)sub_404863(5))
        v47 = 1;
    else
        v47 = 0;
    v8 = v47;
    v8 &= 1;
    printf((!v8 ? "%s %s differ: char %s, line %s\n" : v30));
    return 1;
}



// Function: block_compare @ 0x42d7
void* block_compare(void* a0, unsigned long long a1)
{
    void* v0;  // [bp-0x28]
    unsigned long long i;  // [bp-0x20]
    void* iter;  // [bp-0x18]
    unsigned long long node;  // [bp-0x10]

    v0 = a0;
    for (i = a1; *((long long *)v0) == *((long long *)i); i += 8)
    {
        v0 += 8;
    }
    iter = v0;
    for (node = i; *((char *)iter) == *((char *)node); node += 1)
    {
        iter += 1;
    }
    return iter - a0;
}



// Function: count_newlines @ 0x4350
unsigned long long count_newlines(char *a0, unsigned long a1)
{
    char v0;  // [bp-0x21]
    unsigned long long v1;  // [bp-0x20]
    char *ptr;  // [bp-0x18]
    char *v3;  // [bp-0x10]

    v1 = 0;
    v3 = &a0[a1];
    v0 = *(v3);
    *(v3) = 10;
    ptr = a0;
    while (1)
    {
        ptr = rawmemchr(ptr, 10);
        if (ptr == v3)
            break;
        v1 += 1;
        ptr += 1;
    }
    *(v3) = v0;
    return v1;
}



// Function: sprintc @ 0x43cf
void* sprintc(void* a0, char a1)
{
    unsigned long long *v4;  // rax
    void* v5;  // rax
    void* v6;  // rax
    char iter;  // [bp-0x14]
    void* v1;  // [bp-0x10]
    void* v2;  // [bp-0x10]

    v1 = a0;
    iter = a1;
    v4 = __ctype_b_loc();
    if (!(*((short *)(iter * 2 + *(v4))) & 0x4000))
    {
        if (iter < 0)
        {
            v2 = v1 + 1;
            *((char *)v1) = 77;
            v1 = v2 + 1;
            *((char *)v2) = 45;
            iter += 128;
        }
        if (iter <= 31)
        {
            v5 = v1;
            v1 = v5 + 1;
            *((char *)v5) = 94;
            iter += 64;
        }
        else if (iter == 127)
        {
            v6 = v1;
            v1 = v6 + 1;
            *((char *)v6) = 94;
            iter = 63;
        }
    }
    v2 = v1 + 1;
    *((char *)v1) = iter;
    *((char *)v2) = 0;
    return v2;
}



// Function: file_position @ 0x447f
extern unsigned int g_4100b0[4];
extern unsigned long long g_410200[4];
extern char g_410215;
extern unsigned long long g_410220[4];

unsigned long long * file_position(int a0)
{
    if (*(&(&g_410215)[a0]) != 1)
    {
        *(&(&g_410215)[a0]) = 1;
        g_410220[a0] = lseek(g_4100b0[a0], g_410200[a0], 1);
    }
    return g_410220[a0];
}



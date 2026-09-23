// Function: avx2_supported @ 0x2daf
extern char g_40f110;

unsigned int avx2_supported(void)
{
    char v0;  // [bp-0x22]
    char v1;  // [bp-0x21]
    unsigned int v2;  // [bp-0x20]
    unsigned int v3;  // [bp-0x1c]
    unsigned int v4;  // [bp-0x18]
    unsigned int v5;  // [bp-0x14]

    v2 = 0;
    v3 = 0;
    v4 = 0;
    v5 = 0;
    v0 = 0;
    v1 = 0;
    if (sub_402cb7(1, &v2, &v3, &v4, &v5))
    {
        v0 = 1;
        if (v4 & 0x8000000)
            v1 = 1;
    }
    if (v1)
    {
        v5 = 0;
        v4 = v5;
        v3 = v4;
        v2 = v3;
        if (!sub_402d2f(7, 0, &v2, &v3, &v4, &v5))
        {
            v0 = 0;
        }
        else if (!((char)v3 & 32))
        {
            v1 = 0;
        }
    }
    if (v0 != 1)
    {
        if (!g_40f110)
            return 0;
        gettext("failed to get cpuid");
        error(0, 0, "%s");
        return 0;
    }
    else if (v1 != 1)
    {
        if (!g_40f110)
            return 0;
        gettext("avx2 support not detected");
        error(0, 0, "%s");
        return 0;
    }
    else
    {
        if (!g_40f110)
            return 1;
        gettext("using avx2 hardware support");
        error(0, 0, "%s");
        return 1;
    }
}



// Function: usage @ 0x2f7b
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern FILE *stderr;

void usage(int a0)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    long long v13;  // r9
    char *v14;  // rax
    char *v15;  // rax
    char *v16;  // rax
    char *v17;  // rax
    char *v5;  // rax
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
        v7 = gettext("Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n");
        fputs_unlocked(v7, stdout);
        sub_402978(v8, v9, v10, v11, v12, v13);
        v14 = gettext("\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n");
        fputs_unlocked(v14, stdout);
        v15 = gettext("      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n");
        fputs_unlocked(v15, stdout);
        v16 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v16, stdout);
        v17 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v17, stdout);
        sub_4029a9("wc");
    }
    exit(a0); /* do not return */
}



// Function: iswnbspace @ 0x30be
extern char g_40f158;

unsigned int iswnbspace(unsigned int a0)
{
    if (g_40f158 != 1 && (a0 == 160 || a0 == 8199 || a0 == 8239 || a0 == 8288))
        return 1;
    return 0;
}



// Function: isnbspace @ 0x3109
unsigned int isnbspace(int a0)
{
    return sub_4030be(btowc(a0));
}



// Function: write_counts @ 0x312b
extern char g_40f140;
extern char g_40f141;
extern char g_40f142;
extern char g_40f143;
extern char g_40f144;

void write_counts(long long a0, long long a1, long long a2, long long a3, long long a4, unsigned long a5)
{
    unsigned long v0;  // [bp-0x68]
    char *v1;  // [bp-0x30]
    char v2;  // [bp-0x28]

    v0 = a5;
    v1 = "%*s";
    if (g_40f140)
    {
        sub_405729(a0, &v2, &v2);
        printf(v1);
        v1 = " %*s";
    }
    if (g_40f141)
    {
        sub_405729(a1, &v2, &v2);
        printf(v1);
        v1 = " %*s";
    }
    if (g_40f142)
    {
        sub_405729(a2, &v2, &v2);
        printf(v1);
        v1 = " %*s";
    }
    if (g_40f143)
    {
        sub_405729(a3, &v2, &v2);
        printf(v1);
        v1 = " %*s";
    }
    if (g_40f144)
    {
        sub_405729(a4, &v2, &v2);
        printf(v1);
    }
    if (!v0)
    {
        putchar_unlocked(10);
        return;
    }
    if (strchr(v0, 10))
        v0 = sub_4079a7(0, 3, v0);
    printf(" %s", v0);
    putchar_unlocked(10);
    return;
}



// Function: wc_lines @ 0x3328
unsigned int wc_lines(long long a0, unsigned int a1, unsigned long long *a2, unsigned long long *a3)
{
    unsigned long v12;  // fs
    char *v13;  // rax
    int *err;  // rax
    unsigned int v15;  // eax
    char v0;  // [bp-0x4059]
    unsigned long long iter;  // [bp-0x4058]
    unsigned long long v2;  // [bp-0x4050]
    char *ptr;  // [bp-0x4048]
    unsigned long long v4;  // [bp-0x4040]
    char *v5;  // [bp-0x4038]
    unsigned long long v6;  // [bp-0x4030]
    char v7;  // [bp-0x4028]
    char v8;  // [bp-0x4010]
    unsigned long long v9;  // [bp-0x2010]
    unsigned long v10;  // [bp-0x20]

    do
    {
    } while (&v9 != &v8);
    v10 = *((long long *)(40 + v12));
    v0 = 0;
    if (!a2 || !a3)
    {
        v15 = 0;
    }
    else
    {
        v2 = 0;
        iter = v2;
        while (1)
        {
            v4 = sub_408412(a1, &v7, 0x4000, &v7);
            if (!v4)
            {
                *(a3) = v2;
                *(a2) = iter;
                v15 = 1;
                break;
            }
            else if (v4 == 0xffffffffffffffff)
            {
                sub_4079a7(0, 3, a0);
                err = __errno_location();
                error(0, *(err), "%s");
                v15 = 0;
                break;
            }
            else
            {
                v2 += v4;
                ptr = &v7;
                v5 = &(&v7)[v4];
                if (v0 != 1)
                {
                    for (v6 = iter; ptr != v5; iter += *(v13) == 10)
                    {
                        v13 = ptr;
                        ptr = v13 + 1;
                    }
                }
                else
                {
                    *(v5) = 10;
                    while (1)
                    {
                        ptr = rawmemchr(ptr, 10);
                        if (ptr >= v5)
                            break;
                        ptr += 1;
                        iter += 1;
                    }
                }
                v0 = (iter - v6 <= v4 / 15 ? 1 : 0);
            }
        }
    }
    if (v10 == *((long long *)(40 + v12)))
        return v15;
    __stack_chk_fail(); /* do not return */
}



// Function: wc @ 0x3591
extern unsigned long long g_40f010;
extern unsigned long long g_40f118;
extern unsigned long long g_40f120;
extern unsigned long long g_40f128;
extern unsigned long long g_40f130;
extern unsigned long long g_40f138;
extern char g_40f140;
extern char g_40f141;
extern char g_40f142;
extern char g_40f143;
extern char g_40f144;
extern unsigned long long g_40f150;

long long wc(unsigned int a0, unsigned long a1, int *idx, unsigned long a3)
{
    unsigned long v36;  // fs
    unsigned long v37;  // rax
    int *err;  // rax
    unsigned long long *v47;  // rax
    char v48;  // al
    unsigned long long *v49;  // rax
    char v50;  // al
    int *err1;  // rax
    char v38;  // al
    char v39;  // al
    unsigned long long v40;  // rcx
    int *err2;  // rax
    unsigned long long *v42;  // rax
    char v43;  // al
    unsigned long long *v44;  // rax
    char v45;  // al
    unsigned long v0;  // [bp-0x40e8]
    unsigned long v1;  // [bp-0x40d8]
    char v2;  // [bp-0x40c1]
    char v3;  // [bp-0x40c0]
    char v4;  // [bp-0x40bf]
    char v5;  // [bp-0x40be]
    char v6;  // [bp-0x40bd]
    char v7;  // [bp-0x40bc]
    char v8;  // [bp-0x40bb]
    char v9;  // [bp-0x40ba]
    char v10;  // [bp-0x40b9]
    unsigned int v11;  // [bp-0x40b8]
    unsigned int v12;  // [bp-0x40b4]
    long long iter1;  // [bp-0x40b0]
    long long v14;  // [bp-0x40a8]
    unsigned long iter2;  // [bp-0x40a0], Other Possible Types: unsigned long long
    long long node;  // [bp-0x4098]
    long long iter;  // [bp-0x4090]
    long long v18;  // [bp-0x4088]
    long long v19;  // [bp-0x4080]
    unsigned long v20;  // [bp-0x4078]
    char *v21;  // [bp-0x4070]
    unsigned long long v22;  // [bp-0x4068]
    long long v23;  // [bp-0x4060]
    char *v24;  // [bp-0x4058]
    char *v25;  // [bp-0x4058]
    unsigned long v26;  // [bp-0x4050]
    unsigned long v27;  // [bp-0x4048]
    unsigned long v28;  // [bp-0x4040]
    unsigned long v29;  // [bp-0x4038]
    unsigned long v30;  // [bp-0x4030]
    int v31;  // [bp-0x4028], Other Possible Types: char
    char v32;  // [bp-0x4010]
    unsigned long long v33;  // [bp-0x2010]
    unsigned long v34;  // [bp-0x20]

    do
    {
    } while (&v33 != &v32);
    v1 = a1;
    v0 = a3;
    v34 = *((long long *)(40 + v36));
    v2 = 1;
    if (!v1)
        v37 = gettext("standard input");
    else
        v37 = v1;
    v26 = v37;
    v18 = 0;
    v14 = v18;
    iter = v14;
    node = iter;
    iter1 = node;
    if (__ctype_get_mb_cur_max() > 1)
    {
        v3 = g_40f143;
        v4 = g_40f142;
    }
    else
    {
        if (g_40f143 || g_40f142)
            v38 = 1;
        else
            v38 = 0;
        v3 = v38;
        v3 &= 1;
        v4 = 0;
    }
    if (g_40f141 || g_40f144)
        v39 = 1;
    else
        v39 = 0;
    v10 = v39;
    v10 &= 1;
    if (v3 != 1 || v4 || g_40f140 || v10)
        sub_4054b7(a0, 0, 0, 2);
    if (v3 && v4 != 1 && g_40f140 != 1 && v10 != 1)
    {
        v5 = 0;
        if (*(idx) > 0)
            *(idx) = fstat(a0, idx + 2);
        if (!*(idx) && (char)sub_402c40(idx + 2) && *((long long *)&idx[14]) >= 0)
        {
            v27 = *((long long *)&idx[14]);
            if (v0 - 0 >> 63 & 1)
                v0 = lseek(a0, 0, 1);
            if (v27 % g_40f150)
            {
                v14 = (v27 < v0 ? 0 : v27 - v0);
                v5 = 1;
            }
            else
            {
                if (*((long long *)&idx[16]) > 0 && *((long long *)&idx[16]) <= 0x2000000000000000)
                    v40 = *((long long *)&idx[16]) + 1;
                else
                    v40 = 513;
                v28 = v27 - v27 % v40;
                if (!(v0 - 0 >> 63 & 1) && v0 < v28 && lseek(a0, v28, 1) >= 0)
                    v14 = v28 - v0;
            }
        }
        if (v5 != 1)
        {
            sub_4054b7(a0, 0, 0, 2);
            while (1)
            {
                iter2 = sub_408412(a0, &v31, 0x4000, &v31);
                if (!iter2)
                    break;
                if (iter2 == 0xffffffffffffffff)
                {
                    sub_4079a7(0, 3, v26);
                    err2 = __errno_location();
                    error(0, *(err2), "%s");
                    v2 = 0;
                    break;
                }
                else
                {
                    v14 += iter2;
                }
            }
        }
    }
    else if (v4 != 1 && v10 != 1)
    {
        if ((char)sub_402daf())
            g_40f010 = sub_40a1b1;
        v2 = sub_403328(v26, a0, &iter1, &v14);
    }
    else if (__ctype_get_mb_cur_max() > 1)
    {
        v6 = 0;
        v19 = 0;
        v29 = 0;
        v7 = 0;
        v20 = 0;
        while (1)
        {
            iter2 = sub_408412(a0, &v31 + v20, 0x4000 - v20, &v31 + v20);
            if (!iter2)
                break;
            if (iter2 == 0xffffffffffffffff)
            {
                sub_4079a7(0, 3, v26);
                err = __errno_location();
                error(0, *(err), "%s");
                v2 = 0;
                break;
            }
            else
            {
                v14 += iter2;
                v21 = &v31;
                iter2 += v20;
                do
                {
                    v8 = 1;
                    if (v7 != 1 && (char)sub_405893(*(v21)))
                    {
                        v22 = 1;
                        v11 = *(v21);
                        v8 = 0;
                    }
                    else
                    {
                        v7 = 1;
                        v30 = v29;
                        v22 = sub_4058d8(&v11, v21, iter2, &v29);
                        if (v22 == 0xfffffffffffffffe)
                        {
                            v29 = v30;
                            break;
                        }
                        else if (v22 == 0xffffffffffffffff)
                        {
                            v21 += 1;
                            iter2 -= 1;
                            continue;
                        }
                        else
                        {
                            if (mbsinit(&v29))
                                v7 = 0;
                            if (!v22)
                            {
                                v11 = 0;
                                v22 = 1;
                            }
                        }
                    }
                    switch (v11)
                    {
                    case 32:
                        v19 += 1;
                        break;
                    case 9:
                        v19 = (v19 & 0xfffffffffffffff8) + 8;
                    case 11:
                        node += v6;
                        v6 = 0;
                        goto LABEL_403dfc;
                    case 10:
                        iter1 += 1;
                    case 12: case 13:
                        if (v19 > v18)
                            v18 = v19;
                        v19 = 0;
                        break;
                    default:
                        if (v8 && iswprint(v11))
                        {
                            if (g_40f144)
                            {
                                v12 = wcwidth(v11);
                                if (v12 > 0)
                                    v19 += v12;
                            }
                            if (iswspace(v11) || sub_4030be(v11))
                            {
                                break;
                            }
                            else
                            {
                                v6 = 1;
                                goto LABEL_403dfb;
                            }
                        }
                        if (v8 != 1)
                        {
                            v42 = __ctype_b_loc();
                            v43 = sub_402969(*(v21));
                            if (!(*((short *)(v43 * 2 + *(v42))) & 0x4000))
                                goto LABEL_403dfb;
                            v19 += 1;
                            v44 = __ctype_b_loc();
                            v45 = sub_402969(*(v21));
                            if (!(*((short *)(v45 * 2 + *(v44))) & 0x2000))
                            {
                                v6 = 1;
                                goto LABEL_403dfb;
                            }
                        }
                        else
                        {
LABEL_403dfb:
LABEL_403dfc:
                            v21 = &v21[v22];
                            iter2 -= v22;
                            iter += 1;
                            goto LABEL_403e20;
                        }
                    }
LABEL_403e20:
                } while (iter2);
                switch (iter2)
                {
                case 0:
                    break;
                case 16384:
                    v21 += 1;
                    iter2 -= 1;
                    goto LABEL_403e55;
                default:
LABEL_403e55:
                    memmove(&v31, v21, iter2);
                }
                v20 = iter2;
            }
        }
        if (v19 > v18)
            v18 = v19;
        node += v6;
    }
    else
    {
        v9 = 0;
        v23 = 0;
        while (1)
        {
            iter2 = sub_408412(a0, &v31, 0x4000, &v31);
            if (!iter2)
                break;
            v24 = &v31;
            if (iter2 == 0xffffffffffffffff)
            {
                sub_4079a7(0, 3, v26);
                err1 = __errno_location();
                error(0, *(err1), "%s");
                v2 = 0;
                break;
            }
            else
            {
                v14 += iter2;
                do
                {
                    v25 = v24 + 1;
                    switch (*(v24))
                    {
                    case 32:
                        v23 += 1;
                        break;
                    case 9:
                        v23 = (v23 & 0xfffffffffffffff8) + 8;
                    case 11:
                        node += v9;
                        v9 = 0;
                        continue;
                    case 10:
                        iter1 += 1;
                    case 12: case 13:
                        if (v23 > v18)
                            v18 = v23;
                        v23 = 0;
                        break;
                    default:
                        v47 = __ctype_b_loc();
                        v48 = sub_402969(*(v25 - 1));
                        if (*((short *)(v48 * 2 + *(v47))) & 0x4000)
                        {
                            v23 += 1;
                            v49 = __ctype_b_loc();
                            v50 = sub_402969(*(v25 - 1));
                            if (*((short *)(v50 * 2 + *(v49))) & 0x2000 || sub_403109(sub_402969(*(v25 - 1)) & 0xff))
                            {
                                break;
                            }
                            else
                            {
                                v9 = 1;
                                goto LABEL_4040f8;
                            }
                        }
                        else
                        {
LABEL_4040f8:
                            goto LABEL_4040f9;
                        }
                    }
LABEL_4040f9:
                    iter2 -= 1;
                    v24 = v25;
                } while (iter2);
            }
        }
        if (v23 > v18)
            v18 = v23;
        node += v9;
    }
    if (v4 < g_40f142)
        iter = v14;
    sub_40312b(iter1, node, iter, v14, v18, v1);
    g_40f118 = iter1 + g_40f118;
    g_40f120 = node + g_40f120;
    g_40f128 = iter + g_40f128;
    g_40f130 = v14 + g_40f130;
    if (v18 > g_40f138)
        g_40f138 = v18;
    if (v34 == *((long long *)(40 + v36)))
        return v2;
    __stack_chk_fail(); /* do not return */
}



// Function: wc_file @ 0x4263
extern char g_40f14c;

long long wc_file(unsigned long a0, int *a1)
{
    unsigned int v3;  // edx
    int *err;  // rax
    int *err1;  // rax
    char v0;  // [bp-0x1d]
    unsigned int v1;  // [bp-0x1c]

    if (a0 && strcmp(a0, "-"))
    {
        v1 = open(a0, 0, v3);
        if (v1 == 0xffffffff)
        {
            sub_4079a7(0, 3, a0);
            err = __errno_location();
            error(0, *(err), "%s");
            return 0;
        }
        v0 = sub_403591(v1, a0, a1, 0);
        if (!close(v1))
            return v0;
        sub_4079a7(0, 3, a0);
        err1 = __errno_location();
        error(0, *(err1), "%s");
        return 0;
    }
    g_40f14c = 1;
    sub_40945e(0, 0);
    return sub_403591(0, a0, a1, 0xffffffffffffffff);
}



// Function: get_input_fstatus @ 0x43a5
extern char g_40f140;
extern char g_40f141;
extern char g_40f142;
extern char g_40f143;
extern char g_40f144;

unsigned int * get_input_fstatus(long long a0, unsigned long a1)
{
    long long v4;  // rax
    unsigned int v5;  // edx
    long long v0;  // [bp-0x20]
    long long idx;  // [bp-0x18]
    unsigned int *v2;  // [bp-0x10]

    v0 = a0;
    if (v0)
        v4 = v0;
    else
        v4 = 1;
    v2 = sub_408f54(v4, 152);
    if (v0 && (v0 != 1 || g_40f144 + g_40f140 + g_40f141 + g_40f142 + g_40f143 != 1))
    {
        for (idx = 0; idx < v0; idx += 1)
        {
            if (!*((long long *)(a1 + idx * 8)) || !strcmp(*((long long *)(a1 + idx * 8)), "-"))
                v5 = fstat(0, &v2[38 * idx + 2]);
            else
                v5 = stat(*((long long *)(a1 + idx * 8)), &v2[38 * idx + 2]);
            v2[38 * idx] = v5;
        }
        return v2;
    }
    *(v2) = 1;
    return v2;
}



// Function: compute_number_width @ 0x4549
int compute_number_width(unsigned long long a0, int *a1)
{
    int v0;  // [bp-0x20]
    int v1;  // [bp-0x1c]
    unsigned long long j;  // [bp-0x18]
    unsigned long long i;  // [bp-0x10]

    v0 = 1;
    if (!a0)
    {
        return v0;
    }
    else if (*(a1) <= 0)
    {
        v1 = 1;
        j = 0;
        for (i = 0; i < a0; i += 1)
        {
            if (!a1[38 * i])
            {
                if (((short)a1[8 + 38 * i] & 0xf000) == 0x8000)
                    j += *((long long *)&a1[14 + 38 * i]);
                else
                    v1 = 7;
            }
        }
        for (; j > 9; j /= 10)
        {
            v0 += 1;
        }
        if (v0 >= v1)
            return v0;
        v0 = v1;
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: main @ 0x4670
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

typedef struct FILE {
} FILE;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_40b173;
extern option g_40ea20;
extern long long g_40f018;
extern void stdout;
extern unsigned long long stdin;
extern int optind;
extern void optarg;
extern FILE *stderr;
extern char g_40f110;
extern long long g_40f118;
extern unsigned long long g_40f120;
extern unsigned long long g_40f128;
extern unsigned long long g_40f130;
extern unsigned long long g_40f138;
extern char g_40f140;
extern char g_40f141;
extern char g_40f142;
extern char g_40f143;
extern char g_40f144;
extern unsigned int g_40f148;
extern char g_40f14c;
extern unsigned long long g_40f150;
extern char g_40f158;
extern long long g_40f160;

int main(int a0, long long *a1)
{
    unsigned long long v19;  // rdx
    char *v30;  // rax
    int *err;  // rax
    char *v32;  // rax
    int *err1;  // rax
    char *v21;  // rax
    char *v22;  // rax
    int *err2;  // rax
    unsigned long long v25;  // xmm0lq
    unsigned long long v26;  // xmm0lq
    unsigned long long v27;  // xmm0lq
    unsigned long v0;  // [bp-0x240]
    char v1;  // [bp-0x227]
    char v2;  // [bp-0x226]
    char v3;  // [bp-0x225]
    unsigned int v4;  // [bp-0x224]
    int v5;  // [bp-0x220]
    int v6;  // [bp-0x21c]
    unsigned long v7;  // [bp-0x218]
    long long v8;  // [bp-0x210]
    unsigned long v9;  // [bp-0x208]
    long long v10;  // [bp-0x200]
    unsigned long long fp;  // [bp-0x1f8]
    unsigned int *ptr;  // [bp-0x1f0]
    char *v13;  // [bp-0x1e8]
    unsigned long long v14;  // [bp-0x1e0]
    unsigned long v15;  // [bp-0x1d8]
    long long v16;  // [bp-0x1d0]
    stat v17;  // [bp-0xb8]

    v9 = 0;
    sub_405c1b(*(a1));
    setlocale(6, &g_40b173);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_40ac00(sub_4053bf);
    g_40f150 = (int)getpagesize();
    setvbuf(*((long long *)&stdout), NULL, 1, 0);
    g_40f158 = getenv("POSIXLY_CORRECT");
    g_40f143 = 0;
    g_40f142 = g_40f143;
    g_40f141 = g_40f142;
    g_40f140 = g_40f141;
    g_40f144 = 0;
    g_40f138 = 0;
    g_40f130 = g_40f138;
    g_40f128 = g_40f130;
    g_40f120 = g_40f128;
    g_40f118 = g_40f120;
    while (1)
    {
        v6 = getopt_long(a0, a1, "clLmw", &g_40ea20.name, NULL);
        if (v6 == -0x1)
            break;
        if (v6 > 129)
            goto LABEL_4048e7;
        if (v6 < 76)
        {
            if (v6 != -0x83)
            {
                if (v6 != -0x82)
                    goto LABEL_4048e7;
                sub_402f7b(0); /* do not return */
            }
            sub_408bfb(*((long long *)&stdout), "wc", "GNU coreutils", g_40f018, "Paul Rubin", "David MacKenzie", 0);
            exit(0); /* do not return */
        }
        switch (v6)
        {
        case 76:
            g_40f144 = 1;
            break;
        case 99:
            g_40f143 = 1;
            break;
        case 108:
            g_40f140 = 1;
            break;
        case 109:
            g_40f142 = 1;
            break;
        case 119:
            g_40f141 = 1;
            break;
        case 128:
            g_40f110 = 1;
            break;
        case 129:
            v9 = *((long long *)&optarg);
            break;
        default:
LABEL_4048e7:
            sub_402f7b(1); /* do not return */
        }
    }
    if (g_40f140 != 1 && g_40f141 != 1 && g_40f142 != 1 && g_40f143 != 1 && g_40f144 != 1)
    {
        g_40f143 = 1;
        g_40f141 = g_40f143;
        g_40f140 = g_40f141;
    }
    v2 = 0;
    if (v9)
    {
        if (a0 > optind)
        {
            v19 = optind * 8;
            sub_407815(4, *((long long *)((char *)a1 + v19)), v19);
            error(0, 0, gettext("extra operand %s"));
            v21 = gettext("file operands cannot be combined with --files0-from");
            fprintf(stderr, "%s\n", v21);
            sub_402f7b(1); /* do not return */
        }
        if (!strcmp(v9, "-"))
        {
            fp = stdin;
        }
        else
        {
            fp = fopen(v9, "r");
            if (!fp)
            {
                sub_407815(4, v9);
                v22 = gettext("cannot open %s for reading");
                err2 = __errno_location();
                error(1, *(err2), v22);
            }
        }
        if (!fstat(fileno(fp), &v17) && ((unsigned short)v17.st_nlink & 0xf000) == 0x8000)
        {
            v0 = v17.st_atime;
            sub_405aa9();
            v26 = DivV(v25, 0x4000000000000000);
            if (((CmpF(v26, 0x4164000000000000) & 69 | (char)((CmpF(v26, 0x4164000000000000) & 69) >> 6)) & 1) != 1)
            {
                v27 = 0x4164000000000000;
            }
            else
            {
                sub_405aa9();
                v27 = DivV(v26, 0x4000000000000000);
            }
            if (CmpF(v27, v0) & 69 & 1)
                goto LABEL_404c58;
            v2 = 1;
            sub_407c7f(&v15);
            if ((char)sub_40809e(fp, &v15, &v15) != 1 || sub_40552b(fp))
            {
                sub_407815(4, v9);
                error(1, 0, gettext("cannot read file names from %s"));
            }
            v8 = v16;
            v7 = v15;
            v10 = sub_405165(v8);
        }
        else
        {
LABEL_404c58:
            v8 = 0;
            v7 = 0;
            v10 = sub_4051ba(fp);
        }
    }
    else
    {
        v8 = (a0 <= optind ? &g_40f160 : &a1[optind]);
        v7 = (a0 <= optind ? 1 : a0 - optind);
        v10 = sub_405165(v8);
    }
    if (!v10)
        sub_409413(); /* do not return */
    ptr = sub_4043a5(v7, v8);
    g_40f148 = sub_404549(v7, ptr);
    v1 = 1;
    v5 = 0;
    while (1)
    {
        v3 = 0;
        v13 = sub_405227(v10, &v4, &v4);
        if (!v13)
            break;
        if (v9 && !strcmp(v9, "-") && !strcmp(v13, "-"))
        {
            sub_407815(4, v13);
            error(0, 0, gettext("when reading file names from stdin, no file name of %s allowed"));
            v3 = 1;
        }
        if (!*(v13))
        {
            if (!v9)
            {
                gettext("invalid zero-length file name");
                error(0, 0, "%s");
            }
            else
            {
                v14 = sub_405312(v10);
                gettext("invalid zero-length file name");
                sub_4079a7(0, 3, v9);
                error(0, 0, "%s:%lu: %s");
            }
            v3 = 1;
        }
        if (v3)
            v1 = 0;
        else
            v1 = (char)sub_404263(v13, (!v7 ? 0 : v5 * 152) + (char *)ptr) & v1;
        if (!v7)
            *(ptr) = 1;
        v5 += 1;
    }
    if (v4 != 4)
    {
        if (v4 <= 4)
        {
            if (v4 == 2)
                goto LABEL_405026;
            if (v4 == 3)
                sub_409413(); /* do not return */
        }
        __assert_fail(); /* do not return */
    }
    else
    {
        sub_4079a7(0, 3, v9);
        v30 = gettext("%s: read error");
        err = __errno_location();
        error(0, *(err), v30);
        v1 = 0;
LABEL_405026:
        if (v1 && !v9 && !sub_405312(v10))
            v1 = (char)sub_404263(0, ptr) & v1;
        if (v2)
            sub_407d41(&v15);
        if (sub_405312(v10) > 1)
        {
            v32 = gettext("total");
            sub_40312b(g_40f118, g_40f120, g_40f128, g_40f130, g_40f138, v32);
        }
        sub_405350(v10);
        free(ptr);
        if (!g_40f14c || !close(0))
            return v1 ^ 1;
        err1 = __errno_location();
        error(1, *(err1), "-");
        return v1 ^ 1;
    }
}



// Function: wc_lines_avx2 @ 0xa1b1
unsigned int wc_lines_avx2(long long a0, unsigned int a1, unsigned long long *a2, unsigned long long *a3)
{
    unsigned long v91;  // fs
    uint256_t v101;  // ymm1
    uint256_t v102;  // ymm0
    unsigned long v103;  // 4300
    unsigned long v104;  // 4301
    unsigned long v105;  // 4307
    unsigned long v106;  // 4308
    int v107;  // xmm0
    uint256_t v109;  // ymm0
    uint256_t v110;  // ymm1
    uint256_t v93;  // ymm0
    uint256_t v111;  // ymm0
    unsigned long v112;  // 4472
    unsigned long v113;  // 4473
    unsigned long v114;  // 4479
    unsigned long v115;  // 4480
    int v116;  // xmm0
    uint256_t v118;  // ymm0
    void* v119;  // rax
    int *err;  // rax
    unsigned int v121;  // eax
    uint256_t v95;  // ymm0
    uint256_t v97;  // ymm0
    int v98;  // xmm0
    uint256_t v100;  // ymm0
    unsigned int v0;  // [bp-0x43e4]
    unsigned int v1;  // [bp-0x43e0]
    unsigned int v2;  // [bp-0x43dc]
    unsigned int v3;  // [bp-0x43d8]
    unsigned int v4;  // [bp-0x43d4]
    unsigned int v5;  // [bp-0x43d0]
    unsigned int v6;  // [bp-0x43cc]
    unsigned int v7;  // [bp-0x43c8]
    unsigned int v8;  // [bp-0x43c4]
    unsigned int v9;  // [bp-0x43c0]
    unsigned int v10;  // [bp-0x43bc]
    unsigned int v11;  // [bp-0x43b8]
    unsigned int v12;  // [bp-0x43b4]
    unsigned int v13;  // [bp-0x43b0]
    unsigned int v14;  // [bp-0x43ac]
    unsigned int v15;  // [bp-0x43a8]
    unsigned int v16;  // [bp-0x43a4]
    unsigned int v17;  // [bp-0x4390]
    char v18;  // [bp-0x4371]
    char v19;  // [bp-0x4370]
    char v20;  // [bp-0x436f]
    char v21;  // [bp-0x436e]
    char v22;  // [bp-0x436d]
    char v23;  // [bp-0x436c]
    char v24;  // [bp-0x436b]
    char v25;  // [bp-0x436a]
    char v26;  // [bp-0x4369]
    char v27;  // [bp-0x4368]
    char v28;  // [bp-0x4367]
    char v29;  // [bp-0x4366]
    char v30;  // [bp-0x4365]
    char v31;  // [bp-0x4364]
    char v32;  // [bp-0x4363]
    char v33;  // [bp-0x4362]
    char v34;  // [bp-0x4361]
    char v35;  // [bp-0x4360]
    char v36;  // [bp-0x435f]
    char v37;  // [bp-0x435e]
    char v38;  // [bp-0x435d]
    char v39;  // [bp-0x435c]
    char v40;  // [bp-0x435b]
    char v41;  // [bp-0x435a]
    char v42;  // [bp-0x4359]
    char v43;  // [bp-0x4358]
    char v44;  // [bp-0x4357]
    char v45;  // [bp-0x4356]
    char v46;  // [bp-0x4355]
    char v47;  // [bp-0x4354]
    char v48;  // [bp-0x4353]
    char v49;  // [bp-0x4352]
    char v50;  // [bp-0x4351]
    void* v51;  // [bp-0x4350]
    unsigned long long iter;  // [bp-0x4348]
    unsigned long long v53;  // [bp-0x4340]
    unsigned long long i;  // [bp-0x4338]
    void* v55;  // [bp-0x4330]
    void* v56;  // [bp-0x4328]
    void* v57;  // [bp-0x4320]
    void* v58;  // [bp-0x4318]
    int v59;  // [bp-0x4310]
    int v60;  // [bp-0x4300]
    int v61;  // [bp-0x42f0]
    int v62;  // [bp-0x42e0]
    int v63;  // [bp-0x42d0]
    int v64;  // [bp-0x42c0]
    int v65;  // [bp-0x42b0]
    int v66;  // [bp-0x42a0]
    int v67;  // [bp-0x4290], Other Possible Types: uint256_t
    int v68;  // [bp-0x4270], Other Possible Types: uint256_t
    uint256_t v69;  // [bp-0x4250]
    uint256_t v70;  // [bp-0x4230]
    int v71;  // [bp-0x4210]
    int v72;  // [bp-0x41f0]
    uint256_t v73;  // [bp-0x41d0]
    uint256_t v74;  // [bp-0x41b0]
    uint256_t v75;  // [bp-0x4190]
    uint256_t v76;  // [bp-0x4170]
    uint256_t v77;  // [bp-0x4150]
    uint256_t v78;  // [bp-0x4130]
    int v79;  // [bp-0x4110]
    uint256_t v80;  // [bp-0x40f0]
    int v81;  // [bp-0x40d0]
    uint256_t v82;  // [bp-0x40b0]
    uint256_t v83;  // [bp-0x4090]
    uint256_t v84;  // [bp-0x4070]
    uint256_t v85;  // [bp-0x4050]
    int <0x40a1b1[is_2]|Stack bp-0x4030, 1 B>;  // [bp-0x4030]
    uint256_t v86;  // [bp-0x4030]
    char v87;  // [bp-0x4010]
    unsigned long long v88;  // [bp-0x2030]
    unsigned long v89;  // [bp-0x38]

    do
    {
    } while (&v88 != &<0x40a1b1[is_2]|Stack bp-0x4030, 1 B>);
    v89 = *((long long *)(40 + v91));
    iter = 0;
    v53 = 0;
    i = 0;
    if (!a2 || !a3)
    {
        v121 = 0;
    }
    else
    {
        v93 = _INSERT(_INSERT(0, 0, 0), 16, 0);
        v67 = v93;
        v95 = _INSERT(_INSERT(v67, 0, 0), 16, 0);
        v68 = v95;
        v97 = _INSERT(_INSERT(v68, 0, 0), 16, 0);
        v69 = v97;
        v18 = 10;
        v16 = v18;
        v15 = v18;
        v14 = v18;
        v13 = v18;
        v17 = v18;
        v12 = v18;
        v11 = v18;
        v10 = v18;
        v9 = v18;
        v8 = v18;
        v7 = v18;
        v6 = v18;
        v5 = v18;
        v4 = v18;
        v3 = v18;
        v2 = v18;
        v1 = v18;
        v0 = v18;
        v19 = v18;
        v20 = v18;
        v21 = v18;
        v22 = v18;
        v23 = v18;
        v24 = v18;
        v25 = v18;
        v26 = v18;
        v27 = v18;
        v28 = v18;
        v29 = v18;
        v30 = v18;
        v31 = v18;
        v32 = v18;
        v33 = v0;
        v34 = v1;
        v35 = v2;
        v36 = v3;
        v37 = v4;
        v38 = v5;
        v39 = v6;
        v40 = v7;
        v41 = v8;
        v42 = v9;
        v43 = v10;
        v44 = v11;
        v45 = v12;
        v46 = v17;
        v47 = v13;
        v48 = v14;
        v49 = v15;
        v50 = v16;
        v98 = (int)(InterleaveLOV(InterleaveLOV(InterleaveLOV(v19 * 0x100 | v20 & 0xffff00ff, v21 * 0x100 | v22 & 0xffff00ff), InterleaveLOV(v23 * 0x100 | v24 & 0xffff00ff, v25 * 0x100 | v26 & 0xffff00ff)), InterleaveLOV(InterleaveLOV(v27 * 0x100 | v28, v29 * 0x100 | v30), InterleaveLOV(v31 * 0x100 | v32, v33 * 0x100 | v34))));
        v100 = _INSERT(_INSERT(v98 CONCAT 0, 0, InterleaveLOV(InterleaveLOV(InterleaveLOV(v35 * 0x100 | v36 & 0xffffffffffffffffffffffffffff00ff, v37 * 0x100 | v38 & 0xffffffffffffffffffffffffffff00ff), InterleaveLOV(v39 * 0x100 | v40 & 0xffffffffffffffffffffffffffff00ff, v41 * 0x100 | v42 & 0xffffffffffffffffffffffffffff00ff)), InterleaveLOV(InterleaveLOV(v43 * 0x100 | v44, v45 * 0x100 | v46), InterleaveLOV(v47 * 0x100 | v48, v49 * 0x100 | v50)))), 16, v98);
        v70 = v100;
        while (1)
        {
            i = sub_408412(a1, &v87, 0x3fc0, &v87);
            if (!i)
            {
                *(a2) = iter;
                *(a3) = v53;
                v121 = 1;
                break;
            }
            else if (i == 0xffffffffffffffff)
            {
                sub_4079a7(0, 3, a0);
                err = __errno_location();
                error(0, *(err), "%s");
                v121 = 0;
                break;
            }
            else
            {
                v53 += i;
                v51 = &v87;
                for (v56 = &(&v87)[i]; i > 63; i -= 64)
                {
                    v58 = v51;
                    v71 = (int)*((int256_t *)v58);
                    v57 = v51 + 32;
                    v72 = (int)*((int256_t *)v57);
                    v81 = v71;
                    v82 = v70;
                    v73 = CmpEQV(v81, v82);
                    v79 = v72;
                    v80 = v70;
                    v74 = CmpEQV(v79, v80);
                    v77 = v67;
                    v78 = v73;
                    v67 = SubV(v77, v78);
                    v75 = v68;
                    v76 = v74;
                    v68 = SubV(v75, v76);
                    v51 += 64;
                }
                v85 = v67;
                v86 = v69;
                v101 = v86;
                v102 = v85;
                v103 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
                v104 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
                v105 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
                v106 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
                v67 = (int)(v106 CONCAT v105 CONCAT v104 CONCAT v103);
                v59 = (int)*((uint128_t *)&v67);
                v60 = (int)*((uint128_t *)&v67);
                v61 = (int)*((uint128_t *)(&v67 + 16));
                v62 = (int)*((uint128_t *)(&v67 + 16));
                v107 = v62;
                iter += (unsigned short)(v107 >> 64) + (unsigned short)v59 + (unsigned short)(v60 >> 64) + (unsigned short)v61;
                v109 = _INSERT(_INSERT(v107 CONCAT 0, 0, 0), 16, 0);
                v67 = v109;
                v83 = v68;
                v84 = v69;
                v110 = v84;
                v111 = v83;
                v112 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
                v113 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
                v114 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
                v115 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
                v68 = (int)(v115 CONCAT v114 CONCAT v113 CONCAT v112);
                v63 = (int)*((uint128_t *)&v68);
                v64 = (int)*((uint128_t *)&v68);
                v65 = (int)*((uint128_t *)(&v68 + 16));
                v66 = (int)*((uint128_t *)(&v68 + 16));
                v116 = v66;
                iter += (unsigned short)(v116 >> 64) + (unsigned short)v63 + (unsigned short)(v64 >> 64) + (unsigned short)v65;
                v118 = _INSERT(_INSERT(v116 CONCAT 0, 0, 0), 16, 0);
                v68 = v118;
                for (v55 = v51; v55 != v56; iter += *((char *)v119) == 10)
                {
                    v119 = v55;
                    v55 = v119 + 1;
                }
            }
        }
    }
    if (v89 == *((long long *)(40 + v91)))
        return v121;
    __stack_chk_fail(); /* do not return */
}



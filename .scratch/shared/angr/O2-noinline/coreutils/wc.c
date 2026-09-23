// Function: main @ 0x28a0
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

extern char g_409ec1;
extern option g_40da20;
extern long long g_40e018;
extern void stdout;
extern void stdin;
extern int optind;
extern void optarg;
extern unsigned long long stderr;
extern long long g_40e110;
extern char g_40e118;
extern unsigned long long g_40e120;
extern char g_40e128;
extern unsigned int g_40e12c;
extern char g_40e130;
extern char g_40e131;
extern char g_40e132;
extern char g_40e133;
extern char g_40e134;
extern long long g_40e138;
extern long long g_40e140;
extern long long g_40e148;
extern long long g_40e150;
extern long long g_40e158;
extern char g_40e160;

int main(int a0, long long *a1)
{
    char *v9;  // rax
    unsigned long v10;  // r12
    char *v20;  // rax
    int *err;  // rax
    char *v23;  // rax
    char *v24;  // r13
    char *v26;  // rax
    int *err1;  // rax
    unsigned long v28;  // rax
    unsigned long v11;  // rax
    int *err2;  // rax
    unsigned long fp;  // rbp
    int v14;  // xmm0
    int v15;  // xmm0
    unsigned long long v16;  // xmm1lq
    long long *v17;  // r13
    long long v18;  // rbp
    unsigned long v0;  // [bp-0x228]
    char *v1;  // [bp-0x220], Other Possible Types: unsigned long long
    unsigned int *ptr;  // [bp-0x218]
    char v3;  // [bp-0x209]
    unsigned int v4;  // [bp-0x1fc]
    char v5;  // [bp-0x1f8]
    long long *v6;  // [bp-0x1f0]
    stat v7;  // [bp-0xd8]

    sub_404ea0(*(a1));
    setlocale(6, &g_409ec1);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_408cd0(sub_404870);
    g_40e120 = (int)getpagesize();
    setvbuf(*((long long *)&stdout), NULL, 1, 0);
    v9 = getenv("POSIXLY_CORRECT");
    g_40e131 = 0;
    g_40e132 = 0;
    g_40e133 = 0;
    g_40e134 = 0;
    g_40e130 = 0;
    g_40e138 = 0;
    g_40e140 = 0;
    g_40e148 = 0;
    g_40e150 = 0;
    g_40e158 = 0;
    g_40e118 = v9;
    v10 = 0;
    while (1)
    {
        v11 = getopt_long(a0, a1, "clLmw", &g_40da20.name, NULL);
        if ((unsigned int)v11 == -0x1)
            break;
        if ((unsigned int)v11 > 129)
            goto LABEL_403048;
        if ((unsigned int)v11 > 75)
        {
            switch ((unsigned int)v11)
            {
            case 76:
                g_40e130 = 1;
                break;
            case 99:
                g_40e131 = 1;
                break;
            case 108:
                g_40e134 = 1;
                break;
            case 109:
                g_40e132 = 1;
                break;
            case 119:
                g_40e133 = 1;
                break;
            case 128:
                g_40e160 = 1;
                break;
            case 129:
                v10 = *((long long *)&optarg);
                break;
            default:
LABEL_403048:
                sub_4045c0(1); /* do not return */
            }
        }
        else if ((unsigned int)v11 == -0x83)
        {
            sub_407d20(*((long long *)&stdout), "wc", "GNU coreutils", g_40e018, "Paul Rubin", "David MacKenzie", 0, v11);
            exit(0); /* do not return */
        }
        else if ((unsigned int)v11 == -0x82)
        {
            sub_4045c0(0); /* do not return */
        }
    }
    if (!g_40e134 && !g_40e133 && !g_40e132 && !g_40e131 && !g_40e130)
    {
        g_40e131 = 1;
        g_40e133 = 1;
        g_40e134 = 1;
    }
    if (v10)
    {
        if (a0 > optind)
        {
            sub_407110(4, a1[optind]);
            error(0, 0, dcgettext(NULL, "extra operand %s", 5));
            v23 = dcgettext(NULL, "file operands cannot be combined with --files0-from", 5);
            __fprintf_chk(stderr, 0x1, "%s\n", v23);
        }
        if (!strcmp(v10, "-"))
        {
            fp = *((long long *)&stdin);
        }
        else
        {
            fp = fopen(v10, "r");
            if (!fp)
            {
                sub_407110(4, v10);
                v20 = dcgettext(NULL, "cannot open %s for reading", 5);
                err = __errno_location();
                error(1, *(err), v20);
            }
        }
        if (fstat(fileno(fp), &v7) || ((unsigned short)v7.st_nlink & 0xf000) != 0x8000)
        {
LABEL_402b15:
            v17 = NULL;
            v3 = 0;
            v0 = 0;
            v18 = sub_404730(fp);
        }
        else
        {
            v0 = v7.st_atime;
            sub_404d60();
            v15 = MulV(v14, 0x3fe0000000000000);
            v16 = 0x4164000000000000;
            if (((CmpF(*((unsigned long long *)&v15), 0x4164000000000000) & 69 | (char)((CmpF(*((unsigned long long *)&v15), 0x4164000000000000) & 69) >> 6)) & 1) == 1)
            {
                sub_404d60();
                v16 = (unsigned long long)(MulV(0x3fe0000000000000, v15));
            }
            if (CmpF(v16, v0) & 69 & 1)
                goto LABEL_402b15;
            sub_4074d0(&v5);
            v3 = sub_407580(fp, &v5);
            if (!v3 || sub_404960(fp))
            {
                sub_407110(4, v10);
                error(1, 0, dcgettext(NULL, "cannot read file names from %s", 5));
            }
            v17 = v6;
            v0 = v5;
            v18 = sub_404700(v17);
        }
    }
    else
    {
        if (a0 <= optind)
        {
            v0 = 1;
            v17 = &g_40e110;
        }
        else
        {
            v17 = &a1[optind];
            v0 = a0 - optind;
        }
        v3 = 0;
        v18 = sub_404700(v17);
    }
    if (v18)
    {
        ptr = sub_4032f0(v0, v17);
        *((char *)&v1) = 1;
        g_40e12c = sub_403230(v0, ptr);
        while (1)
        {
            v24 = sub_404770(v18, &v4);
            if (!v24)
                break;
            if (v10)
            {
                if (!(strcmp(v10, "-") || strcmp(v24, "-")))
                {
                    v1 = sub_407110(4, v24);
                    error(0, 0, dcgettext(NULL, "when reading file names from stdin, no file name of %s allowed", 5));
                    if (*(v24))
                        goto LABEL_402c21;
                }
                else if (!(!*(v24)))
                {
                    goto LABEL_402ba5;
                }
                sub_404810(v18);
                v1 = dcgettext(NULL, "invalid zero-length file name", 5);
                sub_407210(0, 3, v10);
                error(0, 0, "%s:%lu: %s");
                *((char *)&v1) = 0;
LABEL_402bbf:
                if (!v0)
                    *(ptr) = 1;
            }
            else
            {
                if (!*(v24))
                {
                    dcgettext(NULL, "invalid zero-length file name", 5);
                    error(0, 0, "%s");
LABEL_402c21:
                    *((char *)&v1) = 0;
                    goto LABEL_402bbf;
                }
LABEL_402ba5:
                *((char *)&v1) = (char)v1 & (char)sub_404500();
                goto LABEL_402bbf;
            }
        }
        if (v4 != 3)
        {
            if (v4 == 4)
            {
                sub_407210(0, 3, v10);
                v26 = dcgettext(NULL, "%s: read error", 5);
                err1 = __errno_location();
                error(0, *(err1), v26);
                *((char *)&v1) = 0;
            }
            else if (v4 != 2)
            {
                __assert_fail(); /* do not return */
            }
            else if (!v10 & (char)v1)
            {
                *((char *)&v1) = !v10 & (char)v1;
                if (!sub_404810(v18))
                    *((char *)&v1) = sub_404500(0, ptr);
            }
            if (v3)
                sub_407550(&v5);
            if (sub_404810(v18) > 1)
            {
                v28 = dcgettext(NULL, "total", 5);
                sub_4035d0(g_40e158, g_40e150, g_40e148, g_40e140, g_40e138, v28);
            }
            sub_404830(v18);
            free(ptr);
            if (!g_40e128 || !close(0))
                return (char)v1 ^ 1;
            err2 = __errno_location();
            error(1, *(err2), "-");
            return (char)v1 ^ 1;
        }
    }
    sub_408250(); /* do not return */
}



// Function: iswnbspace @ 0x31f0
extern char g_40e118;

unsigned int iswnbspace(unsigned int a0)
{
    if (g_40e118)
        return 0;
    if (a0 == 160 || a0 == 8199)
        return 1;
    return a0 == 8239 | a0 == 8288;
    return 1;
}



// Function: compute_number_width @ 0x3230
typedef struct struct_0 {
    int field_0;
    char padding_4[28];
    unsigned int field_20;
    char padding_24[20];
    unsigned long long field_38;
} struct_0;

unsigned long long compute_number_width(unsigned long long a0, struct_0 *a1)
{
    unsigned long long v1;  // rdx
    unsigned int v2;  // r8d
    struct_0 *iter;  // rsi
    unsigned int v4;  // ecx
    unsigned long long i;  // rdx
    unsigned long long v6;  // rdx

    if (!a0)
    {
        return 1;
    }
    else if (a1->field_0 > 0)
    {
        return 1;
    }
    else
    {
        v1 = 0;
        v2 = 1;
        iter = a1;
        do
        {
            if (!iter->field_0)
            {
                if (((unsigned short)iter->field_20 & 0xf000) != 0x8000)
                    v2 = 7;
                else
                    v1 += iter->field_38;
            }
        } while ((iter += 152, iter != a0 * 152 + a1));
        v4 = 1;
        if (v1 > 9)
        {
            do
            {
                v4 += 1;
                v6 = v1 / 10;
                v1 = v6;
            } while (i > 99);
        }
        return (v2 <= v4 ? v4 : v2);
    }
}



// Function: get_input_fstatus @ 0x32f0
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

extern char g_40e130;
extern char g_40e131;
extern char g_40e132;
extern char g_40e133;
extern char g_40e134;

stat * get_input_fstatus(unsigned long long a0, unsigned long a1)
{
    unsigned long long idx;  // rbx
    stat *iter;  // rbp
    unsigned long v4;  // r14
    unsigned int v5;  // eax
    stat *v0;  // [bp-0x40]

    v0 = sub_407fb0();
    if (a0 && (a0 != 1 || g_40e134 + g_40e133 + g_40e132 + g_40e131 + g_40e130 != 1))
    {
        idx = 0;
        iter = &v0->st_ino;
        do
        {
            v4 = *((long long *)(a1 + idx * 8));
            if (!v4 || !strcmp(v4, "-"))
                v5 = fstat(0, iter);
            else
                v5 = stat(v4, iter);
        } while ((idx += 1, *((unsigned int *)((char *)iter - 8)) = v5, iter += 152, a0 != idx));
        return v0;
    }
    v0->st_mode = 1;
    return v0;
}



// Function: wc_lines @ 0x33e0
int wc_lines(void)
{
    unsigned long v8;  // fs
    unsigned long long *v9;  // rdx
    void* v18;  // rax
    unsigned long long v19;  // rbp
    void* v20;  // rdi
    void* ptr;  // rax
    long long v22;  // rdi
    int *err;  // rax
    unsigned long long *v10;  // rcx
    unsigned long long v11;  // r14
    void* v12;  // r12
    unsigned long long v13;  // r13
    unsigned int v14;  // esi
    unsigned long long v15;  // rax
    void* i;  // rbx
    unsigned long long v17;  // rdx
    unsigned long v0;  // [bp-0x4078]
    unsigned long long v1;  // [bp-0x4070]
    char v2;  // [bp-0x4061]
    char v3;  // [bp-0x4048]
    char v4;  // [bp-0x4030]
    unsigned long long v5;  // [bp-0x2030]
    unsigned long v6;  // [bp-0x40]

    do
    {
    } while (&v5 != &v4);
    v6 = *((long long *)(40 + v8));
    v2 = !v9 | !v10;
    if (v9 && v10)
    {
        v11 = 0;
        v12 = &v3;
        v13 = 0;
        v1 = 0;
        while (1)
        {
            v15 = sub_407770(v14, &v3, 0x4000);
            if (!v15)
            {
                v2 = 1;
                *(v10) = v1;
                *(v9) = v13;
                break;
            }
            else if (v15 != 0xffffffffffffffff)
            {
                v1 += v15;
                i = &(&v3)[v15];
                v0 = v15 / 15;
                if ((char)v11)
                {
                    *((char *)i) = 10;
                    v19 = v13;
                    v20 = v12;
                    while (1)
                    {
                        v13 = v19;
                        ptr = rawmemchr(v20, 10);
                        if (i <= ptr)
                            break;
                        v20 = ptr + 1;
                        v19 = v13 + 1;
                    }
                    v11 = _INSERT(v11, 0, v13 - v13 <= v0);
                }
                else if (i != &v3)
                {
                    v17 = v13;
                    v18 = v12;
                    do
                    {
                        v18 += 1;
                        v17 += *((char *)v18 - 1) == 10;
                    } while (i != v18);
                    v13 = v17;
                    v11 = _INSERT(v11, 0, v17 - v13 <= v0);
                }
                else
                {
                    v11 = 1;
                }
            }
            else
            {
                sub_407210(0, 3, v22);
                err = __errno_location();
                error(0, *(err), "%s");
                break;
            }
        }
    }
    else
    {
        v2 = 0;
    }
    if (v6 != *((long long *)(40 + v8)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: isnbspace @ 0x35b0
unsigned int isnbspace(int a0)
{
    return sub_4031f0(btowc(a0));
}



// Function: write_counts @ 0x35d0
extern char g_40e130;
extern char g_40e131;
extern char g_40e132;
extern char g_40e133;
extern char g_40e134;

void write_counts(unsigned long a0, long long a1, long long a2, long long a3, long long a4, unsigned long a5)
{
    unsigned long v2;  // r12
    char *v3;  // rbp
    char *v4;  // rbp
    char *v5;  // rbp
    char *v6;  // rbp
    char v0;  // [bp-0x58]

    v2 = a5;
    v3 = "%*s";
    if (g_40e134)
    {
        sub_404ad0();
        v3 = " %*s";
        __printf_chk(1, "%*s");
    }
    if (g_40e133)
    {
        sub_404ad0(a1, &v0);
        v4 = " %*s";
        __printf_chk(1, v3);
        v3 = v4;
    }
    if (g_40e132)
    {
        sub_404ad0(a2, &v0);
        v5 = " %*s";
        __printf_chk(1, v3);
        v3 = v5;
    }
    if (g_40e131)
    {
        sub_404ad0(a3, &v0);
        v6 = " %*s";
        __printf_chk(1, v3);
        v3 = v6;
    }
    if (g_40e130)
    {
        sub_404ad0(a4, &v0);
        __printf_chk(1, v3);
    }
    if (!v2)
    {
        putchar_unlocked(10);
        return;
    }
    if (strchr(v2, 10))
        v2 = sub_407210(0, 3, v2);
    __printf_chk(1, " %s", v2);
    putchar_unlocked(10);
    return;
}



// Function: avx2_supported @ 0x3830
extern char g_40e160;

unsigned int avx2_supported(void)
{
    long long v5;  // r11
    char *v6;  // rsi
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]
    unsigned int v2;  // [bp-0x38]
    unsigned int v3;  // [bp-0x34]

    v0 = 0;
    v1 = 0;
    v2 = 0;
    v3 = 0;
    if (sub_4037b0(&v0, &v1, &v2, &v3))
    {
        if (*((char *)((void*)&v2 + 3)) & 8)
        {
            v3 = 0;
            v2 = 0;
            v1 = 0;
            v0 = 0;
            if (!sub_4037f0(v5, &v1, &v2, &v3))
                goto LABEL_4038d8;
            if ((char)v1 & 32)
            {
                if (!g_40e160)
                    return 1;
                dcgettext(NULL, "using avx2 hardware support", 5);
                error(0, 0, "%s");
                return g_40e160;
            }
        }
        if (g_40e160)
        {
            v6 = "avx2 support not detected";
LABEL_4038e9:
            dcgettext(NULL, v6, 5);
            error(0, 0, "%s");
        }
    }
    else
    {
LABEL_4038d8:
        if (g_40e160)
        {
            v6 = "failed to get cpuid";
            goto LABEL_4038e9;
        }
    }
    return 0;
}



// Function: wc @ 0x3bf0
typedef struct mbstate_t {
} mbstate_t;

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
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_40e010;
extern unsigned long long g_40e120;
extern char g_40e130;
extern char g_40e131;
extern char g_40e132;
extern char g_40e133;
extern char g_40e134;
extern unsigned long long g_40e138;
extern unsigned long long g_40e140;
extern unsigned long long g_40e148;
extern unsigned long long g_40e150;
extern unsigned long long g_40e158;

int wc(void)
{
    unsigned long v16;  // fs
    unsigned long long *v17;  // rsi
    unsigned long v22;  // r14
    long long v23;  // rcx, Other Possible Types: unsigned long long
    unsigned long v24;  // rbp
    char *iter;  // rbp, Other Possible Types: unsigned long, unsigned long long
    unsigned long v26;  // r13
    long long v27;  // rbx
    unsigned long long v28;  // rcx
    unsigned long v29;  // r14
    unsigned long long v30;  // rax
    int *err;  // rax
    unsigned long long v18;  // rax
    unsigned int v32;  // edi
    unsigned long long *v33;  // rax
    long long v34;  // rdi
    stat *idx;  // rdx, Other Possible Types: long long
    long long v36;  // r8
    long long v37;  // r9
    char v38;  // r14b
    char v19;  // r13b
    unsigned long long v39;  // rbx
    long long iter1;  // r15
    unsigned long long v41;  // rax
    char *iter2;  // r12
    char v43;  // al
    unsigned long v44;  // rdi
    unsigned long long v45;  // rdx
    char v46;  // al
    unsigned long long v47;  // rax
    unsigned int v48;  // edi
    unsigned long long v49;  // rsi
    unsigned long long v50;  // rax
    unsigned int v51;  // eax
    unsigned long long v52;  // rsi
    int *err1;  // rax
    unsigned long long v54;  // rbp
    unsigned long long v55;  // rbx
    unsigned long long v56;  // r13
    unsigned long long v57;  // rax
    char *v58;  // r12
    char *i;  // rdx
    char v60;  // r14b
    char v61;  // al
    unsigned long long v62;  // rbx
    int *err2;  // rax
    unsigned int v20;  // eax
    char v21;  // al
    unsigned int v0;  // [bp-0x40a0], Other Possible Types: unsigned long long
    unsigned long long v1;  // [bp-0x4098]
    unsigned long long v2;  // [bp-0x4090]
    char *v3;  // [bp-0x4088]
    char v4;  // [bp-0x407a]
    char v5;  // [bp-0x4079]
    unsigned long long *v6;  // [bp-0x4070]
    unsigned int v7;  // [bp-0x4064]
    unsigned long long v8;  // [bp-0x4060]
    unsigned long node;  // [bp-0x4058]
    mbstate_t v10;  // [bp-0x4050]
    char v11;  // [bp-0x4048]
    char v12;  // [bp-0x4030]
    unsigned long long v13;  // [bp-0x2030]
    unsigned long v14;  // [bp-0x40]

    do
    {
    } while (&v13 != &v12);
    v14 = *((long long *)(40 + v16));
    v6 = v17;
    if (!v17)
        v6 = dcgettext(NULL, "standard input", 5);
    node = 0;
    v8 = 0;
    v18 = __ctype_get_mb_cur_max();
    v19 = g_40e131;
    if (v18 > 1)
    {
        v5 = g_40e132;
    }
    else
    {
        v5 = 0;
        v19 = (!g_40e131 ? g_40e132 : g_40e131);
    }
    if (!g_40e133 && !g_40e130)
    {
        if (v19 == 1 && !v5)
        {
            if (!g_40e134)
                goto LABEL_403cbb;
            sub_404920(v32, 0, 0, 2);
            goto LABEL_40446e;
        }
        sub_404920(v32, 0, 0, 2);
        if (v5 == 1)
            goto LABEL_403e5a;
        if (v19)
        {
LABEL_40446e:
            if (g_40e134)
                goto LABEL_404480;
LABEL_403cbb:
            v20 = idx->st_mode;
            if (v20 > 0)
            {
                v20 = fstat(v32, &idx->st_ino);
                idx->st_mode = v20;
            }
            if (!v20 && !(v21 = (char)(unsigned long long)sub_403be0((unsigned long long)idx->st_gid), !v21 || (v22 = (unsigned long)idx->st_mtime, v22 < 0)))
            {
                if (v23 != 0xffffffffffffffff)
                {
                    v24 = 0;
                    if (!(v22 % g_40e120))
                        goto LABEL_403d10;
                    goto LABEL_40442f;
                }
                iter = v22;
                v24 = lseek(v32, 0, 1);
                if (iter % g_40e120)
                {
                    v22 = (v24 <= iter ? iter - v24 : 0);
LABEL_40442f:
                    node = v22;
                    iter = 0;
                    v26 = 0;
                    v27 = 0;
                    goto LABEL_403db8;
                }
                else
                {
                    v21 = ~(v24) >> 63;
LABEL_403d10:
                    v28 = 513;
                    if (idx->st_ctime - 1 <= 0x1fffffffffffffff)
                        v28 = idx->st_ctime + 1;
                    v29 = v22 - v22 % v28;
                    if (v29 > v24 && v21 && lseek(v32, v29, 1) >= 0)
                    {
                        node = v29 - v24;
                        goto LABEL_403d65;
                    }
                }
            }
            else
            {
LABEL_403d65:
                sub_404920(v32, 0, 0, 2);
                while (1)
                {
                    v30 = sub_407770(v32, &v11, 0x4000);
                    if (!v30)
                    {
                        v22 = node;
                        v26 = 0;
                        iter = 0;
                        v27 = 0;
                        break;
                    }
                    else if (v30 != 0xffffffffffffffff)
                    {
                        node += v30;
                    }
                    else
                    {
                        iter = 0;
                        v27 = 0;
                        sub_407210(0, 3, v6);
                        err = __errno_location();
                        v26 = 0;
                        error(0, *(err), "%s");
                        v22 = node;
                        break;
                    }
                }
                goto LABEL_403db8;
            }
        }
        if (v5)
            goto LABEL_403e5a;
LABEL_404480:
        if (!(char)sub_403830())
        {
            v33 = &g_40e010->field_0;
        }
        else
        {
            v33 = sub_408a90;
            g_40e010 = sub_408a90;
        }
        iter = 0;
        v26 = 0;
        v27 = 0;
        v33(v6, v32, &v8, &node);
        v22 = node;
    }
    else
    {
        sub_404920(v32, 0, 0, 2);
LABEL_403e5a:
        if (__ctype_get_mb_cur_max(v34, v17, idx, v23, v36, v37) > 1)
        {
            v38 = 0;
            v39 = 0;
            v10 = (mbstate_t)0;
            v26 = 0;
            iter1 = 0;
            iter = 0;
            v2 = 0;
            v1 = 0;
            v3 = &v11;
            while (1)
            {
                v41 = sub_407770(v32, &v3[iter1], 0x4000 - iter1);
                if (!v41)
                    goto LABEL_4042e0;
                if (v41 == 0xffffffffffffffff)
                    break;
                node += v41;
                iter2 = v3;
                iter1 += v41;
                if (!v38)
                {
LABEL_403ef0:
                    v0 = *(iter2);
                    v43 = sub_404bc0(v0);
                    if (!v43)
                        goto LABEL_403f9c;
                    v44 = v0;
                    v7 = v44;
                    switch ((char)v0)
                    {
                    case 9:
                        v45 = 1;
                        goto LABEL_4040b5;
                    case 10:
                        v45 = 1;
                        goto LABEL_4040a5;
                    case 11:
                        v45 = 1;
                        goto LABEL_404072;
                    case 12: case 13:
                        v45 = 1;
                        goto LABEL_404055;
                    case 32:
                        v45 = 1;
                        goto LABEL_404095;
                    default:
                        v4 = v43;
                        v0 = v44;
                        v46 = sub_4031e0(v0, __ctype_b_loc(v44));
                        if ((char)*((short *)(*(v17) + v46 * 2)) & 64)
                        {
                            iter += 1;
                            if (!((char)*((short *)(*(v17) + v46 * 2)) & 32))
                            {
                                v39 = v4;
                                goto LABEL_403f76;
                            }
                        }
                        else
                        {
LABEL_403f76:
                            v45 = 1;
                            break;
                        }
                    }
                }
                while (1)
                {
LABEL_403f9c:
                    v47 = sub_404be0(&v7, iter2, iter1, &v10);
                    if (v47 == 0xfffffffffffffffe)
                        goto LABEL_0x404110;
                    if (v47 != 0xffffffffffffffff)
                    {
                        v0 = v47;
                        v45 = v0;
                        v38 = !mbsinit(&v10);
                        if (!v45)
                        {
                            v7 = 0;
                            v48 = 0;
                            v45 = 1;
                            goto LABEL_403ff8;
                        }
                        else
                        {
                            v48 = v7;
                            switch (v48)
                            {
                            case 9:
LABEL_4040b5:
                                iter = (iter & 0xfffffffffffffff8) + 8;
                            case 11:
LABEL_404072:
                                v1 += v39;
                                v39 = 0;
                                break;
                            case 32:
LABEL_404095:
                                iter += 1;
                            case 10:
LABEL_4040a5:
                                v8 += 1;
                            case 12: case 13:
LABEL_404055:
                                v49 = v2;
                                v50 = iter;
                                iter = 0;
                                if (v50 > v49)
                                    v49 = v50;
                                v2 = v49;
                            default:
LABEL_403ff8:
                                v0 = v45;
                                v45 = v0;
                                if (!iswprint(v48))
                                {
                                    iter2 = &iter2[v45];
                                    iter1 -= v45;
                                    v26 += 1;
                                    continue;
                                }
                                else
                                {
                                    if (g_40e130)
                                    {
                                        v51 = wcwidth(v7);
                                        v52 = v51 + iter;
                                        if (v51 <= 0)
                                            v52 = iter;
                                        iter = v52;
                                        v45 = v0;
                                    }
                                    if (!iswspace(v7) && !(v45 = v45, (unsigned int)(unsigned long long)sub_4031f0(v7)))
                                    {
                                        v39 = 1;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    iter2 += 1;
                    iter1 -= 1;
                    v38 = 1;
                    if (!iter1)
                        break;
                    if (!v38)
                        goto LABEL_403ef0;
                }
            }
            sub_407210(0, 3, v6);
            err1 = __errno_location();
            error(0, *(err1), "%s");
LABEL_4042e0:
            v22 = node;
            if (iter <= v2)
                iter = v2;
            v27 = v39 + v1;
        }
        else
        {
            v54 = 0;
            v3 = &v11;
            iter = 0;
            v55 = 0;
            v56 = 0;
            while (1)
            {
                v57 = sub_407770(v32, v3, 0x4000);
                iter = iter;
                if (!v57)
                {
                    v62 = (char)(v56 & 0xffffffff);
                    break;
                }
                else if (v57 != 0xffffffffffffffff)
                {
                    node += v57;
                    v58 = v3;
                    i = &v58[v57];
                    do
                    {
                        v60 = *(v58);
                        v58 += 1;
                        switch (v60)
                        {
                        case 32:
                            iter += 1;
                            break;
                        case 9:
                            iter = (iter & 0xfffffffffffffff8) + 8;
                        case 11:
                            v54 += v56;
                            v56 = 0;
                            goto LABEL_404250;
                        case 10:
                            v8 += 1;
                        case 12: case 13:
                            if (v55 < iter)
                                v55 = iter;
                            iter = 0;
                            break;
                        default:
                            v0 = iter;
                            iter = i;
                            v61 = sub_4031e0(v60, __ctype_b_loc());
                            i = iter;
                            iter = v0;
                            if (!(*((short *)(*(v17) + v61 * 2)) & 0x4000))
                                continue;
                            iter += 1;
                            if (!(*((short *)(*(v17) + v61 * 2)) & 0x2000) && !(v0 = iter, i = iter, iter = v0, (unsigned int)(unsigned long long)sub_4035b0((unsigned int)(unsigned long long)v61)))
                            {
                                v56 = 1;
                                continue;
                            }
                        }
LABEL_404250:
                    } while (i != v58);
                }
                else
                {
                    v62 = (char)(v56 & 0xffffffff);
                    sub_407210(0, 3, v6);
                    err2 = __errno_location();
                    error(0, *(err2), "%s");
                    break;
                }
            }
            v22 = node;
            if (v55 >= iter)
                iter = v55;
            v27 = v62 + v54;
            v26 = 0;
        }
    }
LABEL_403db8:
    if (g_40e132 > v5)
        v26 = v22;
    sub_4035d0(v8, v27, v26, v22, iter, v17);
    g_40e150 = g_40e150 + v27;
    g_40e158 = g_40e158 + v8;
    g_40e148 = g_40e148 + v26;
    g_40e140 = g_40e140 + node;
    if (g_40e138 < iter)
        g_40e138 = iter;
    if (v14 != *((long long *)(40 + v16)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: wc_file @ 0x4500
extern char g_40e128;

unsigned int wc_file(unsigned long a0, long long a1)
{
    unsigned int v1;  // edx
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    int *err;  // rax

    if (a0 && strcmp(a0, "-"))
    {
        v2 = open(a0, 0, v1);
        if (v2 != 0xffffffff)
        {
            v3 = sub_403bf0();
            if (!close(v2))
                return v3;
        }
        sub_407210(0, 3, a0);
        err = __errno_location();
        error(0, *(err), "%s");
        return 0;
    }
    g_40e128 = 1;
    return sub_403bf0(0, a0, a1, -0x1);
}



// Function: usage @ 0x45c0
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5));
        v5 = dcgettext(NULL, "Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n", 5);
        fputs_unlocked(v5, stdout);
        sub_4032c0(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n", 5);
        fputs_unlocked(v13, stdout);
        v14 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v14, stdout);
        v15 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v15, stdout);
        sub_4039b0(v6, v7, v8, v9, v10, v11);
    }
    exit(a0); /* do not return */
}



// Function: wc_lines_avx2 @ 0x8a90
typedef struct struct_0 {
    char field_0[32];
    uint256_t field_20;
} struct_0;

unsigned int wc_lines_avx2(long long a0, unsigned int a1, unsigned long long *a2, unsigned long long *a3)
{
    unsigned long long v2;  // rbx
    struct_0 *v3;  // r15
    struct_0 *j;  // rdi
    uint256_t v11;  // ymm3
    struct_0 *v12;  // rax
    struct_0 *i;  // rdx
    uint256_t v14;  // ymm3
    struct_0 *v15;  // rdx
    int v18;  // ymm2
    unsigned long long iter;  // r14
    unsigned long v19;  // 4399
    unsigned long v20;  // 4400
    unsigned long v21;  // 4405
    unsigned long v22;  // 4406
    int v23;  // ymm1
    unsigned long v24;  // 4415
    unsigned long v25;  // 4416
    unsigned long v26;  // 4421
    unsigned long v27;  // 4422
    int v28;  // ymm0
    int *err;  // rax
    unsigned long v30;  // r13
    uint128_t v5;  // xmm3
    uint128_t v6;  // xmm2
    uint128_t v7;  // xmm0
    unsigned long long v8;  // rax
    char v0;  // [bp-0x4010]

    if (!a2 || !a3)
        return 0;
    v2 = 0;
    v3 = &v0;
    iter = 0;
    while (1)
    {
        v8 = sub_407770(a1, &v0, 0x3fc0);
        if (!v8)
        {
            *(a2) = iter;
            *(a3) = v2;
            return 1;
        }
        else if (v8 != 0xffffffffffffffff)
        {
            v2 += v8;
            j = &(&v0)[v8];
            if (v8 > 63)
            {
                v11 = _INSERT(_INSERT(v5 CONCAT 0, 0, 0), 16, 0);
                v12 = &v0 + (v8 - 64 & 0xffffffffffffffc0) - 64;
                i = v3;
                v14 = v11;
                do
                {
                    v15 = i + 1;
                    v11 = SubV(v11, CmpEQV(0xa0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a, i->field_0));
                    v14 = SubV(v14, CmpEQV(0xa0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a, i->field_20));
                    i = v15;
                } while (i != v12);
            }
            else
            {
                v11 = _INSERT(_INSERT(v7 CONCAT 0, 0, 0), 16, 0);
                v12 = &v0;
                v14 = v11;
            }
            v18 = (int)_INSERT(_INSERT(v6 CONCAT 0, 0, 0), 16, 0);
            v19 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
            v20 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
            v21 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
            v22 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
            v23 = (int)(v22 CONCAT v21 CONCAT v20 CONCAT v19);
            v24 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
            v25 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
            v26 = _ccall((unsigned long long)UnaryOp unpack, (unsigned long long)UnaryOp unpack);
            v27 = _ccall((unsigned long long)(UnaryOp unpack >> 64), (unsigned long long)(UnaryOp unpack >> 64));
            v28 = (int)(v27 CONCAT v26 CONCAT v25 CONCAT v24);
            v5 = *((uint128_t *)&v23);
            v6 = *((uint128_t *)&v28);
            v7 = *((uint128_t *)(&v28 + 16));
            iter += ((unsigned int)(v5 >> 64) & 0xffff) + ((unsigned int)v5 & 0xffff) + ((unsigned int)*((uint128_t *)(&v23 + 16)) & 0xffff) + ((unsigned int)(*((uint128_t *)(&v23 + 16)) >> 64) & 0xffff) + ((unsigned int)(v6 >> 64) & 0xffff) + ((unsigned int)v6 & 0xffff) + ((unsigned int)v7 & 0xffff) + ((unsigned int)(v7 >> 64) & 0xffff);
            if (j != v12)
            {
                do
                {
                    v12 = &v12->field_0[1];
                    iter += *(&v12->field_0[0] - 1) == 10;
                } while (j != v12);
            }
        }
        else
        {
            sub_407210(0, 3, a0);
            err = __errno_location();
            error(0, *(err), "%s");
            return _INSERT(v30, 0, !a2 | !a3);
        }
    }
}



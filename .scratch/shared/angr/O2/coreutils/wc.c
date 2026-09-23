// Function: main @ 0x2860
typedef struct struct_0 {
    int field_0;
    char padding_4[28];
    unsigned int field_20;
    char padding_24[20];
    unsigned long long field_38;
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

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_409e81;
extern option g_40ca20;
extern long long g_40d018;
extern void stdout;
extern void stdin;
extern int optind;
extern void optarg;
extern unsigned long long stderr;
extern long long g_40d110;
extern char g_40d118;
extern unsigned long long g_40d120;
extern char g_40d128;
extern unsigned int g_40d12c;
extern char g_40d130;
extern char g_40d131;
extern char g_40d132;
extern char g_40d133;
extern char g_40d134;
extern long long g_40d138;
extern long long g_40d140;
extern long long g_40d148;
extern long long g_40d150;
extern long long g_40d158;
extern char g_40d160;

int main(int a0, long long *a1)
{
    char *v10;  // rax
    unsigned long v11;  // r12
    long long *v20;  // r15
    long long iter;  // rbp, Other Possible Types: unsigned long, unsigned long long
    int v23;  // eax
    char *v24;  // rax
    int *err;  // rax
    char *v27;  // rax
    unsigned long idx;  // r13
    struct_0 *node;  // r14
    int v12;  // eax
    unsigned long v30;  // rbp
    unsigned int v31;  // eax
    int v32;  // esi
    unsigned long long v33;  // rdx
    unsigned long v34;  // rcx
    struct_0 *iter1;  // rax
    int v36;  // edi
    unsigned long long i;  // rdx
    long long v13;  // r8
    char *v39;  // r13
    struct_0 *v40;  // rdx
    unsigned long long v41;  // rdx
    unsigned int v43;  // eax
    char v44;  // dl
    int *err1;  // rax
    char *v46;  // rax
    int *err2;  // rax
    unsigned long v48;  // rax
    unsigned long fp;  // rbp
    int *v49;  // rax
    int v16;  // xmm0
    int v17;  // xmm0
    unsigned long long v18;  // xmm1lq
    unsigned long v19;  // rbx
    char *v0;  // [bp-0x228], Other Possible Types: unsigned long long
    struct_0 *ptr;  // [bp-0x220]
    struct_0 *v2;  // [bp-0x218], Other Possible Types: char
    char v3;  // [bp-0x20d]
    unsigned int v4;  // [bp-0x20c]
    unsigned int v5;  // [bp-0x1fc]
    char v6;  // [bp-0x1f8]
    long long *v7;  // [bp-0x1f0]
    stat v8;  // [bp-0xd8]

    sub_404d10(*(a1));
    setlocale(6, &g_409e81);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_4088e0(sub_4046f0);
    g_40d120 = (int)getpagesize();
    setvbuf(*((long long *)&stdout), NULL, 1, 0);
    v10 = getenv("POSIXLY_CORRECT");
    g_40d131 = 0;
    g_40d132 = 0;
    g_40d133 = 0;
    g_40d134 = 0;
    g_40d130 = 0;
    g_40d138 = 0;
    g_40d140 = 0;
    g_40d148 = 0;
    g_40d150 = 0;
    g_40d158 = 0;
    g_40d118 = v10;
    v11 = 0;
    while (1)
    {
        v12 = getopt_long(a0, a1, "clLmw", &g_40ca20.name, NULL);
        if (v12 == 0xffffffff)
            break;
        if (v12 > 129)
            goto LABEL_403299;
        if (v12 > 75)
        {
            switch (v12)
            {
            case 76:
                g_40d130 = 1;
                break;
            case 99:
                g_40d131 = 1;
                break;
            case 108:
                g_40d134 = 1;
                break;
            case 109:
                g_40d132 = 1;
                break;
            case 119:
                g_40d133 = 1;
                break;
            case 128:
                g_40d160 = 1;
                break;
            case 129:
                v11 = *((long long *)&optarg);
                break;
            default:
LABEL_403299:
                sub_404230(1); /* do not return */
            }
        }
        else if (v12 == -0x83)
        {
            sub_407a40(*((long long *)&stdout), "wc", "GNU coreutils", g_40d018, "Paul Rubin", "David MacKenzie", 0, v13);
            exit(0); /* do not return */
        }
        else if (v12 == -0x82)
        {
            sub_404230(0); /* do not return */
        }
    }
    if (!g_40d134 && !g_40d133 && !g_40d132 && !g_40d131 && !g_40d130)
    {
        g_40d131 = 1;
        g_40d133 = 1;
        g_40d134 = 1;
    }
    if (v11)
    {
        if (a0 > optind)
        {
            sub_4069f0(4, a1[optind]);
            error(0, 0, dcgettext(NULL, "extra operand %s", 5));
            v27 = dcgettext(NULL, "file operands cannot be combined with --files0-from", 5);
            __fprintf_chk(stderr, 0x1, "%s\n", v27);
        }
        if (!strcmp(v11, "-"))
        {
            fp = *((long long *)&stdin);
        }
        else
        {
            fp = fopen(v11, "r");
            if (!fp)
            {
                sub_4069f0(4, v11);
                v24 = dcgettext(NULL, "cannot open %s for reading", 5);
                err = __errno_location();
                error(1, *(err), v24);
            }
        }
        if (fstat(fileno(fp), &v8) || ((unsigned short)v8.st_nlink & 0xf000) != 0x8000)
        {
LABEL_402ad5:
            iter = sub_4045b0(fp);
            if (!iter)
                sub_408110(); /* do not return */
            v3 = 0;
            ptr = sub_407d50(1, 152);
        }
        else
        {
            iter = v8.st_atime;
            sub_404bd0();
            v17 = MulV(v16, 0x3fe0000000000000);
            v18 = 0x4164000000000000;
            if (((CmpF(*((unsigned long long *)&v17), 0x4164000000000000) & 69 | (char)((CmpF(*((unsigned long long *)&v17), 0x4164000000000000) & 69) >> 6)) & 1) == 1)
            {
                sub_404bd0();
                v18 = (unsigned long long)(MulV(0x3fe0000000000000, v17));
            }
            if (CmpF(v18, iter) & 69 & 1)
                goto LABEL_402ad5;
            sub_4071f0(&v6);
            v3 = sub_4072a0(fp, &v6);
            if (!v3 || sub_4047e0(fp))
            {
                sub_4069f0(4, v11);
                error(1, 0, dcgettext(NULL, "cannot read file names from %s", 5));
            }
            v19 = v6;
            v20 = v7;
            iter = sub_404580(v7);
            if (!iter)
                sub_408110(); /* do not return */
            ptr = sub_407d50();
            if (v19)
                goto LABEL_403024;
        }
        v19 = 0;
        ptr->field_0 = 1;
        v23 = 1;
    }
    else
    {
        if (a0 <= optind)
        {
            v19 = 1;
            v20 = &g_40d110;
        }
        else
        {
            v19 = a0 - optind;
            v20 = &a1[optind];
        }
        iter = sub_404580(v20);
        if (!iter)
            sub_408110(); /* do not return */
        v3 = 0;
        ptr = sub_407d50(v19, 152);
LABEL_403024:
        if (v19 != 1 || (v23 = (int)((unsigned int)g_40d134 + (unsigned int)g_40d133 + (unsigned int)g_40d132 + (unsigned int)g_40d131 + (unsigned int)g_40d130), v23 != 1))
        {
            idx = 0;
            node = &ptr->padding_4[4];
            do
            {
                v30 = v20[idx];
                if (!v30 || !strcmp(v30, "-"))
                    v31 = fstat(0, node);
                else
                    v31 = stat(v30, node);
            } while ((idx = (unsigned long)(idx + 1), *((unsigned int *)((char *)node - 8)) = v31, node += 152, idx < v19));
            v23 = 1;
            if (ptr->field_0 <= 0)
            {
                v32 = 1;
                v33 = 0;
                v34 = 0;
                iter1 = ptr;
                do
                {
                    if (!iter1->field_0)
                    {
                        if (((unsigned short)iter1->field_20 & 0xf000) != 0x8000)
                            v32 = 7;
                        else
                            v33 += iter1->field_38;
                    }
                } while ((v34 = (unsigned long)(v34 + 1), iter1 += 152, v34 < v19));
                v36 = 1;
                if (v33 > 9)
                {
                    do
                    {
                        v36 += 1;
                        v33 /= 10;
                    } while (i > 99);
                }
                v23 = (v36 <= v32 ? v32 : v36);
            }
        }
        else
        {
            ptr->field_0 = 1;
        }
    }
    g_40d12c = v23;
    iter = 0;
    *((char *)&v0) = 1;
    while (1)
    {
        v39 = sub_4045f0(iter, &v5);
        if (!v39)
            break;
        if (v11)
        {
            if (!strcmp(v11, "-"))
            {
                if (strcmp(v39, "-"))
                {
                    if (*(v39))
                    {
                        if (!v19)
                        {
                            v40 = ptr;
                            goto LABEL_402b6c;
                        }
                        else
                        {
                            v41 = iter;
                            goto LABEL_402c1b;
                        }
                    }
                }
                else
                {
                    v0 = sub_4069f0(4, v39);
                    error(0, 0, dcgettext(NULL, "when reading file names from stdin, no file name of %s allowed", 5));
                    if (*(v39))
                        goto LABEL_402de1;
                }
            }
            else
            {
                if (*(v39))
                    goto LABEL_402c10;
            }
            sub_404690(iter);
            v0 = dcgettext(NULL, "invalid zero-length file name", 5);
            sub_406d70(0, 3, v11);
            error(0, 0, "%s:%lu: %s");
            *((char *)&v0) = 0;
            goto LABEL_402bb9;
        }
        else if (*(v39))
        {
LABEL_402c10:
            v41 = 0;
            if (v19)
                v41 = iter;
LABEL_402c1b:
            v2 = v41 + (char *)ptr;
            v40 = v2;
            if (!strcmp(v39, "-"))
            {
                g_40d128 = 1;
                v44 = sub_403820(0, v39);
LABEL_402bb5:
                *((char *)&v0) = (char)v0 & v44;
                goto LABEL_402bb9;
            }
LABEL_402b6c:
            v2 = v40;
            v43 = open(v39, 0, *((unsigned int *)&v40));
            if (v43 == 0xffffffff || (v4 = v43, v2 = (char)(unsigned long long)sub_403820((unsigned long long)v43, v39, v2, 0), v44 = v2, (int)(long long)close(v4)))
            {
                sub_406d70(0, 3, v39);
                err1 = __errno_location();
                error(0, *(err1), "%s");
                v44 = 0;
                goto LABEL_402bb5;
            }
        }
        else
        {
            dcgettext(NULL, "invalid zero-length file name", 5);
            error(0, 0, "%s");
LABEL_402de1:
            *((char *)&v0) = 0;
LABEL_402bb9:
            if (!v19)
                ptr->field_0 = 1;
            iter += 152;
        }
    }
    if (v5 == 3)
        sub_408110(); /* do not return */
    if (v5 == 4)
    {
        sub_406d70(0, 3, v11);
        v46 = dcgettext(NULL, "%s: read error", 5);
        err2 = __errno_location();
        error(0, *(err2), v46);
        *((char *)&v0) = 0;
    }
    else if (v5 != 2)
    {
        __assert_fail(); /* do not return */
    }
    else if (!v11 & (char)v0)
    {
        *((char *)&v0) = !v11 & (char)v0;
        if (!sub_404690(iter))
        {
            g_40d128 = 1;
            *((char *)&v0) = sub_403820(0, 0, ptr, -0x1);
        }
    }
    if (v3)
        sub_407270(&v6);
    if (sub_404690(iter) > 1)
    {
        v48 = dcgettext(NULL, "total", 5);
        sub_403430(g_40d158, g_40d150, g_40d148, g_40d140, g_40d138, v48);
    }
    sub_4046b0(iter);
    free(ptr);
    if (!g_40d128 || !close(0))
        return (char)v0 ^ 1;
    v49 = __errno_location();
    error(1, *(v49), "-");
    return (char)v0 ^ 1;
}



// Function: write_counts @ 0x3430
typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

extern struct_0 *stdout;
extern char g_40d130;
extern char g_40d131;
extern char g_40d132;
extern char g_40d133;
extern char g_40d134;

void write_counts(unsigned long a0, long long a1, long long a2, long long a3, long long a4, unsigned long a5)
{
    unsigned long v2;  // r12
    char *v3;  // rbp
    char *v4;  // rbp
    char *v5;  // rbp
    char *v6;  // rbp
    char *v7;  // rax
    char v0;  // [bp-0x58]

    v2 = a5;
    v3 = "%*s";
    if (g_40d134)
    {
        sub_404940();
        v3 = " %*s";
        __printf_chk(1, "%*s");
    }
    if (g_40d133)
    {
        sub_404940(a1, &v0);
        v4 = " %*s";
        __printf_chk(1, v3);
        v3 = v4;
    }
    if (g_40d132)
    {
        sub_404940(a2, &v0);
        v5 = " %*s";
        __printf_chk(1, v3);
        v3 = v5;
    }
    if (g_40d131)
    {
        sub_404940(a3, &v0);
        v6 = " %*s";
        __printf_chk(1, v3);
        v3 = v6;
    }
    if (g_40d130)
    {
        sub_404940(a4, &v0);
        __printf_chk(1, v3);
    }
    if (v2)
    {
        if (strchr(v2, 10))
            v2 = sub_406d70(0, 3, v2);
        __printf_chk(1, " %s", v2);
    }
    v7 = &stdout->field_28->field_0;
    if (v7 >= *((long long *)&stdout[1].padding_0[0]))
    {
        __overflow(stdout, 10);
        return;
    }
    stdout->field_28 = v7 + 1;
    *(v7) = 10;
    return;
}



// Function: wc @ 0x3820
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

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

extern struct_1 g_4099e0;
extern struct_0 *g_40d010;
extern char g_40d118;
extern unsigned long long g_40d120;
extern char g_40d130;
extern char g_40d131;
extern char g_40d132;
extern char g_40d133;
extern char g_40d134;
extern unsigned long long g_40d138;
extern unsigned long long g_40d140;
extern unsigned long long g_40d148;
extern unsigned long long g_40d150;
extern unsigned long long g_40d158;
extern char g_40d160;

int wc(void)
{
    unsigned long v16;  // fs
    unsigned long v17;  // rsi
    unsigned long long v22;  // rcx
    char v23;  // sil
    unsigned long v24;  // rbp
    unsigned long long iter;  // rbp
    unsigned long v26;  // r12
    long long v27;  // rbx
    unsigned long long v28;  // rdx
    unsigned long v29;  // r12
    unsigned long long v30;  // rax
    int *err;  // rax
    unsigned long long v18;  // rax
    stat *idx;  // rdx, Other Possible Types: long long
    unsigned long long *v35;  // rax
    char *v36;  // rsi
    long long v37;  // rdi
    long long v38;  // rcx
    char v19;  // r12b
    long long v39;  // r8
    long long v40;  // r9
    long long n;  // r14
    char v42;  // r15b
    unsigned long long v43;  // rbx
    unsigned long long v44;  // rax
    long long iter1;  // r15
    char *iter2;  // r13
    char v47;  // cl
    unsigned int v48;  // r14d
    unsigned long long v49;  // r14
    unsigned long long *v50;  // rax
    unsigned int v51;  // edi
    unsigned int v52;  // eax
    int *err1;  // rax
    unsigned long long v54;  // rbp
    unsigned long long v55;  // r13
    unsigned long long v56;  // r12
    unsigned long long v57;  // rax
    char *v58;  // r14
    char *j;  // rdx
    unsigned long v60;  // rdi
    unsigned long long *v61;  // rax
    unsigned long v62;  // rax
    unsigned int v63;  // eax
    unsigned long long v64;  // rbx
    int *err2;  // rax
    unsigned int v20;  // eax
    unsigned long v21;  // r12
    char *v0;  // [bp-0x40a8], Other Possible Types: mbstate_t *, unsigned long, char
    char *v1;  // [bp-0x40a0], Other Possible Types: unsigned long long
    mbstate_t v2;  // [bp-0x4098], Other Possible Types: char
    unsigned long long v3;  // [bp-0x4090]
    char *v4;  // [bp-0x4088]
    char v5;  // [bp-0x4079]
    unsigned long v6;  // [bp-0x4070]
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
    v19 = g_40d131;
    if (v18 > 1)
    {
        v5 = g_40d132;
    }
    else
    {
        v5 = 0;
        v19 = (!g_40d131 ? g_40d132 : g_40d131);
    }
    if (!(!g_40d133 && !g_40d130))
    {
        sub_4047a0((unsigned int)v37, 0, 0, 2);
LABEL_403a9a:
        if (__ctype_get_mb_cur_max(v37, v17, idx, v38, v39, v40) > 1)
        {
            n = 0;
            v42 = 0;
            iter = 0;
            v10 = (mbstate_t)0;
            v43 = 0;
            v26 = 0;
            v3 = 0;
            v1 = 0;
            v4 = &v11;
            while (1)
            {
LABEL_403ae0:
                v44 = sub_407490((unsigned int)v37, &v4[n], 0x4000 - n);
                if (v44)
                {
                    if (v44 != 0xffffffffffffffff)
                    {
                        node += v44;
                        iter1 = n + v44;
                        iter2 = v4;
                        do
                        {
                            if (!v42 && (v47 = *(iter2), v48 = (&g_4099e0.field_0)[(unsigned int)_INSERT((unsigned long long)v47, 0, v47 >> 5) & 7] >> (v47 & 31), (char)v48 & 1))
                            {
                                v7 = v47;
                                switch (v47)
                                {
                                case 9:
                                    v49 = 1;
                                    goto LABEL_403c26;
                                case 10:
                                    v49 = 1;
                                    goto LABEL_403c16;
                                case 11:
                                    v49 = 1;
                                    goto LABEL_403be9;
                                case 12: case 13:
                                    v49 = 1;
                                    goto LABEL_403bd6;
                                case 32:
                                    v49 = 1;
                                    goto LABEL_403c06;
                                default:
                                    v2 = v42;
                                    v0 = v47;
                                    v50 = __ctype_b_loc();
                                    v42 = v2;
                                    if (!((char)*((short *)(*(v50) + v0 * 2)) & 64))
                                    {
                                        v49 = 1;
                                        break;
                                    }
                                    iter += 1;
                                    if (!((char)*((short *)(*(v50) + v0 * 2)) & 32))
                                    {
                                        v43 = (char)v48 & 1;
                                        v49 = 1;
LABEL_403ba8:
                                        iter2 = &iter2[v49];
                                        iter1 -= v49;
                                        v26 += 1;
                                        continue;
                                    }
                                }
                            }
                            else
                            {
                                v0 = &v10;
                                v2 = v10;
                                v49 = sub_404a50(&v7, iter2, iter1, &v10);
                                if (v49 == 0xfffffffffffffffe)
                                {
                                    n = iter1;
                                    v10 = v2;
                                    switch (iter1)
                                    {
                                    case 0:
                                        break;
                                    case 16384:
                                        iter2 += 1;
                                        n = 0x3fff;
                                        goto LABEL_403d85;
                                    default:
LABEL_403d85:
                                        __memmove_chk(v4, iter2, n, 16385);
                                    }
                                    v42 = 1;
                                    goto LABEL_403ae0;
                                }
                                if (v49 != 0xffffffffffffffff)
                                {
                                    v42 = !mbsinit(v0);
                                    if (!v49)
                                    {
                                        v7 = 0;
                                        v51 = 0;
                                        v49 = 1;
                                    }
                                    else
                                    {
                                        v51 = v7;
                                        switch (v51)
                                        {
                                        case 9:
LABEL_403c26:
                                            iter = (iter & 0xfffffffffffffff8) + 8;
                                        case 10:
LABEL_403c16:
                                            v8 += 1;
                                        case 11:
LABEL_403be9:
                                            v1 += v43;
                                            v43 = 0;
                                            goto LABEL_403ba8;
                                        case 12: case 13:
LABEL_403bd6:
                                            if (iter <= v3)
                                            {
                                                iter = v3;
                                                break;
                                            }
                                            else
                                            {
                                                break;
                                            }
                                            v3 = iter;
                                            iter = 0;
                                        case 32:
LABEL_403c06:
                                            iter += 1;
                                        default:
                                            goto LABEL_403c98;
                                        }
                                        goto LABEL_403ba8;
                                    }
LABEL_403c98:
                                    v0 = v42;
                                    v42 = v0;
                                    if (iswprint(v51))
                                    {
                                        if (g_40d130)
                                        {
                                            v52 = wcwidth(v7);
                                            if (v52 > 0)
                                                iter += v52;
                                            v42 = v0;
                                        }
                                        v0 = v42;
                                        v42 = v0;
                                        if (iswspace(v7))
                                            goto LABEL_403be9;
                                        if (!g_40d118)
                                        {
                                            if (v7 == 160 || v7 == 8199 || v7 == 8239 || v7 == 8288)
                                                goto LABEL_403be9;
                                            v43 = 1;
                                            goto LABEL_403ba8;
                                        }
                                        else
                                        {
                                            v43 = g_40d118;
                                            goto LABEL_403ba8;
                                        }
                                    }
                                }
                                else
                                {
                                    iter2 += 1;
                                    iter1 -= 1;
                                    v42 = 1;
                                }
                            }
                        } while (iter1);
                        n = 0;
                    }
                    else
                    {
                        sub_406d70(0, 3, v6);
                        err1 = __errno_location();
                        error(0, *(err1), "%s");
                        break;
                    }
                }
                else
                {
                    break;
                }
            }
            v21 = node;
            if (iter <= v3)
                iter = v3;
            v27 = v43 + v1;
        }
        else
        {
            v54 = 0;
            v55 = 0;
            v4 = &v11;
            iter = 0;
            v56 = 0;
            while (1)
            {
                v57 = sub_407490((unsigned int)v37, v4, 0x4000);
                if (v57)
                {
                    if (v57 != 0xffffffffffffffff)
                    {
                        node += v57;
                        v58 = v4;
                        j = &v58[v57];
                        do
                        {
                            v60 = *(v58);
                            v58 += 1;
                            switch ((char)(v60 - 9 & 0xffffffff))
                            {
                            case 23:
                                v55 += 1;
                                break;
                            case 0:
                                v55 = (v55 & 0xfffffffffffffff8) + 8;
                            case 2:
                                v54 += v56;
                                v56 = 0;
                                goto LABEL_403ed0;
                            case 1:
                                v8 += 1;
                            case 3: case 4:
                                if (iter < v55)
                                    iter = v55;
                                v55 = 0;
                                break;
                            default:
                                v1 = j;
                                v0 = v60;
                                v61 = __ctype_b_loc(v60);
                                j = v1;
                                v62 = v0;
                                if (!((char)*((short *)(*(v61) + v62 * 2)) & 64))
                                    continue;
                                v55 += 1;
                                if (!((char)*((short *)(*(v61) + v62 * 2)) & 32))
                                {
                                    v0 = j;
                                    v63 = btowc(v62);
                                    j = v0;
                                    if (!g_40d118)
                                    {
                                        if (v63 != 160 && v63 != 8199 && v63 != 8239 && v63 != 8288)
                                        {
                                            v56 = 1;
                                            continue;
                                        }
                                    }
                                    else
                                    {
                                        v56 = g_40d118;
                                        continue;
                                    }
                                }
                            }
LABEL_403ed0:
                        } while (j != v58);
                    }
                    else
                    {
                        v64 = (char)(v56 & 0xffffffff);
                        sub_406d70(0, 3, v6);
                        err2 = __errno_location();
                        error(0, *(err2), "%s");
                        break;
                    }
                }
                else
                {
                    v64 = (char)(v56 & 0xffffffff);
                    break;
                }
            }
            v21 = node;
            if (iter < v55)
                iter = v55;
            v27 = v64 + v54;
            v26 = 0;
        }
    }
    else if (v19 == 1 && !v5)
    {
        if (!g_40d134)
            goto LABEL_4038eb;
        sub_4047a0((unsigned int)v37, 0, 0, 2);
LABEL_4040e0:
        if (g_40d134)
            goto LABEL_4040f0;
LABEL_4038eb:
        v20 = idx->st_mode;
        if (v20 > 0)
        {
            v20 = fstat(v37, &idx->st_ino);
            idx->st_mode = v20;
        }
        if (!v20 && ((unsigned short)idx->st_gid & 0xd000) == 0x8000 && !(v21 = (unsigned long)idx->st_mtime, v21 < 0))
        {
            if (v22 != 0xffffffffffffffff)
            {
                if (v21 % g_40d120)
                    goto LABEL_4040b7;
                v23 = 1;
                v24 = 0;
                goto LABEL_403949;
            }
            v0 = v21;
            v24 = lseek(v37, 0, 1);
            if (v0 % g_40d120)
            {
                v21 = (v24 <= v0 ? v0 - v24 : 0);
LABEL_4040b7:
                node = v21;
                iter = 0;
                v26 = 0;
                v27 = 0;
            }
            else
            {
                v23 = ~(v24) >> 63;
LABEL_403949:
                v28 = 513;
                if (idx->st_ctime - 1 <= 0x1fffffffffffffff)
                    v28 = idx->st_ctime + 1;
                v29 = v21 - v21 % v28;
                if (v29 > v24 && v23 && lseek(v37, v29, 1) >= 0)
                {
                    node = v29 - v24;
                    goto LABEL_4039a1;
                }
            }
        }
        else
        {
LABEL_4039a1:
            sub_4047a0((unsigned int)v37, 0, 0, 2);
            while (1)
            {
                v30 = sub_407490((unsigned int)v37, &v11, 0x4000);
                if (!v30)
                {
                    v21 = node;
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
                    sub_406d70(0, 3, v6);
                    err = __errno_location();
                    v26 = 0;
                    error(0, *(err), "%s");
                    v21 = node;
                    break;
                }
            }
        }
    }
    else
    {
        sub_4047a0((unsigned int)v37, 0, 0, 2);
        if (v5 == 1)
            goto LABEL_403a9a;
        if (v19)
            goto LABEL_4040e0;
        if (v5)
            goto LABEL_403a9a;
LABEL_4040f0:
        amd64g_dirtyhelper_CPUID_avx2(unsupported_<class 'pyvex.expr.GSPTR'>())
        /* unsupported instruction */
        if (g_40d160)
        {
            v36 = "failed to get cpuid";
            dcgettext(NULL, v36, 5);
            error(0, 0, "%s");
        }
        v35 = g_40d010;
        iter = 0;
        v26 = 0;
        v27 = 0;
        v35(v6, (unsigned int)v37, &v8, &node);
        v21 = node;
    }
    if (g_40d132 > v5)
        v26 = v21;
    sub_403430(v8, v27, v26, v21, iter, v17);
    g_40d150 = g_40d150 + v27;
    g_40d158 = g_40d158 + v8;
    g_40d148 = g_40d148 + v26;
    g_40d140 = g_40d140 + node;
    if (g_40d138 < iter)
        g_40d138 = iter;
    if (v14 != *((long long *)(40 + v16)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: usage @ 0x4230
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
    char *v31;  // rsi
    struct_0 *v32;  // rbx
    char *v33;  // r14
    char *v35;  // rax
    unsigned long long v18;  // r13
    char *v36;  // rax
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5));
        v25 = dcgettext(NULL, "Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "\nWith no FILE, or when FILE is -, read standard input.\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v29, stdout);
        v30 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v30, stdout);
        v0 = (int)_INSERT(v0 CONCAT 0, 0, "[");
        v31 = "[";
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
        } while (strcmp("wc", v31) && (v31 = v32->field_10, v23 = v32 + 16, v32->field_10));
        v33 = *((long long *)&v23->padding_1[7]);
        if (!*((long long *)&v23->padding_1[7]))
            v33 = "wc";
        __printf_chk(1, dcgettext(NULL, "\n%s online help: <%s>\n", 5));
        v35 = setlocale(5, NULL);
        if (v35 && strncmp(v35, "en_", 3))
        {
            v36 = dcgettext(NULL, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
            fputs_unlocked(v36, stdout);
        }
        __printf_chk(1, dcgettext(NULL, "Full documentation <%s%s>\n", 5));
        __printf_chk(1, dcgettext(NULL, "or available locally via: info '(coreutils) %s%s'\n", 5));
    }
    exit(a0); /* do not return */
}



// Function: wc_lines_avx2 @ 0x86a0
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
        v8 = sub_407490(a1, &v0, 0x3fc0);
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
            sub_406d70(0, 3, a0);
            err = __errno_location();
            error(0, *(err), "%s");
            return _INSERT(v30, 0, !a2 | !a3);
        }
    }
}



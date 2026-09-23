// Function: main @ 0x7240
extern char g_42962f;
extern unsigned long long stderr;
extern unsigned int g_4385f8;
extern unsigned int g_438604;
extern char g_438608;
extern unsigned long long g_438610;
extern char g_438618;
extern long long g_438620;
extern int g_438648;
extern unsigned long long g_438660;
extern unsigned long long g_438670;

int main(unsigned int a0, long long *a1)
{
    long long v1;  // rdi
    long long v2;  // rdi
    char *v11;  // rax
    int *err;  // rax
    char *v13;  // rax
    int *err1;  // rax
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned int v8;  // r12d
    char *t;  // rax
    long long v10;  // rax

    v1 = *(a1);
    if (!*(a1))
        v1 = "find";
    sub_421e40(v1);
    sub_40da20(v2, v3, v4, v5, v6, v7);
    g_438618 = 0;
    g_438604 = 0;
    g_438608 = 0;
    g_4385f8 = 0xffffff9c;
    if ((char)sub_416220(v2, v3, v4, v5, v6, v7))
        sub_415fd0();
    g_438610 = sub_40ea90("w");
    if (!g_438610)
    {
        v13 = dcgettext(NULL, "Failed to initialize shared-file hash table", 5);
        err1 = __errno_location();
        error(1, *(err1), v13);
    }
    sub_40e630(&g_438620);
    setlocale(6, &g_42962f);
    bindtextdomain("findutils", "/usr/local/share/locale");
    textdomain("findutils");
    if (!sub_427a00(sub_417a40))
    {
        v8 = sub_40e180(a0, a1);
        if ((char)g_438660 & 2)
            g_438670 = sub_40dec0;
        if ((char)g_438660 & 128)
        {
            t = ctime(&g_438648);
            __fprintf_chk(stderr, 0x1, "cur_day_start = %s", t);
        }
        v10 = sub_40c9f0(a0, a1, v8);
        if (!(char)sub_408070(a0 - v8 & 0xffffffff, &a1[v8]))
            return g_438604;
        sub_409e00(v10);
        sub_40dab0(v2, v3, v4, v5, v6, v7);
        return g_438604;
    }
    v11 = dcgettext(NULL, "The atexit library function failed", 5);
    err = __errno_location();
    error(1, *(err), v11);
}



// Function: get_fts_info_name @ 0x7510
extern char g_438598;

char * get_fts_info_name(unsigned int a0)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x8]

    switch (a0)
    {
    case 1:
        return "FTS_D";
    case 2:
        return "FTS_DC";
    case 3:
        return "FTS_DEFAULT";
    case 4:
        return "FTS_DNR";
    case 5:
        return "FTS_DOT";
    case 6:
        return "FTS_DP";
    case 7:
        return "FTS_ERR";
    case 8:
        return "FTS_F";
    case 9:
        return "FTS_INIT";
    case 10:
        return "FTS_NS";
    case 11:
        return "FTS_NSOK";
    case 12:
        return "FTS_SL";
    case 13:
        return "FTS_SLNONE";
    case 14:
        return "FTS_W";
    default:
        v0 = v2;
        __sprintf_chk(&g_438598, 0x1);
        return &g_438598;
    }
}



// Function: consider_visiting @ 0x7670
typedef struct struct_0 {
    char padding_0[44];
    int field_2c;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern int g_438010;
extern void g_438014;
extern unsigned int g_438018;
extern unsigned long long stderr;
extern unsigned int g_4385e0;
extern void g_4385e4;
extern char g_4385e5;
extern unsigned int g_4385e8;
extern unsigned long long g_4385f0;
extern unsigned int g_4385f8;
extern char g_438600;
extern unsigned int g_438604;
extern char g_438620;
extern int g_438624;
extern int g_438628;
extern void g_438660;
extern struct_1 *g_438670;
extern unsigned int g_43868c;

void consider_visiting(struct_0 *index, void* idx)
{
    unsigned long long v15;  // rax
    char *v16;  // r14
    char *v25;  // rbx
    char v26;  // r12b
    unsigned int v29;  // edx
    int v30;  // xmm0
    int v31;  // xmm1
    int v32;  // xmm4
    int v33;  // xmm5
    int v34;  // xmm6
    unsigned int v17;  // eax
    int v35;  // xmm7
    int v36;  // xmm0
    unsigned short v38;  // ax
    unsigned long v39;  // rdx
    int v40;  // r13d
    int v41;  // ebx
    unsigned int v42;  // edx
    unsigned int v43;  // eax
    char *v44;  // rax
    char *v18;  // rax
    unsigned long v45;  // rax
    long long v46;  // rax
    char v19;  // cl
    unsigned short v20;  // ax
    unsigned int v21;  // esi
    unsigned long v22;  // rdx
    int v23;  // edi
    unsigned long v24;  // rcx
    unsigned long v0;  // [bp-0x198], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0x180], Other Possible Types: unsigned long
    unsigned int v2;  // [bp-0x16c]
    int v3;  // [bp-0x168]
    unsigned long v4;  // [bp-0x160]
    int v5;  // [bp-0x158]
    int v6;  // [bp-0x148]
    int v7;  // [bp-0x138]
    int v8;  // [bp-0x128]
    int v9;  // [bp-0x118]
    int v10;  // [bp-0x108]
    int v11;  // [bp-0xf8]
    int v12;  // [bp-0xe8]
    char v13;  // [bp-0xd8]

    if (g_438660 & 4)
    {
        v15 = sub_4239f0(2, g_43868c, (long long)idx[48]);
        v16 = sub_4239f0(1, g_43868c, (long long)idx[56]);
        v1 = (long long)idx[88];
        v17 = (unsigned int)sub_407510((short)idx[104]);
        v18 = sub_4239f0(0, g_43868c, (long long)idx[56]);
        v0 = v15;
        __fprintf_chk(stderr, 0x1, "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n", v18, v17, (unsigned int)v1, *((int *)&g_438014), v16);
    }
    v19 = g_438018;
    v20 = (short)idx[104];
    v21 = g_438018 & 0x200;
    if (v20 != 6)
    {
        v22 = (long long)idx[88];
        if (v22 <= *((int *)&g_438014) && v22)
            goto LABEL_4076e0;
        if (v21)
            goto LABEL_407908;
    }
    else
    {
        if (!v21)
        {
            *((int *)&g_438014) = (long long)idx[88];
            v30 = (int)(int128_t)idx[112];
            v31 = (int)(int128_t)idx[128];
            *((unsigned short *)&g_4385e4) = 0x101;
            v32 = (int)(int128_t)idx[176];
            v3 = v30;
            v33 = (int)(int128_t)idx[192];
            v34 = (int)(int128_t)idx[208];
            v5 = v31;
            v35 = (int)(int128_t)idx[224];
            v6 = (int)(int128_t)idx[144];
            v36 = (int)(int128_t)idx[240];
            v7 = (int)(int128_t)idx[160];
            v2 = *((unsigned int *)(&v5 + 8));
            g_4385e8 = v2;
            v8 = v32;
            v9 = v33;
            v10 = v34;
            v11 = v35;
            v12 = v36;
            if (v2)
            {
                g_4385e0 = (long long)idx[88];
                goto LABEL_407808;
            }
            else
            {
                sub_4239f0(0, g_43868c, (long long)idx[56]);
                error(0, 0, dcgettext(NULL, "WARNING: file %s appears to have mode 0000", 5));
                v29 = v2;
                goto LABEL_407a35;
            }
        }
LABEL_407908:
        if (g_438010 < 0)
        {
            v23 = index->field_2c;
LABEL_4076ea:
            if (v23 != -0x64 && v23 < 0)
                sub_407640(); /* do not return */
            g_4385f8 = v23;
            if (g_438010 >= 0)
                goto LABEL_40770b;
            if (v23 == -0x64)
            {
                g_438010 = 0xffffff9c;
                v22 = (long long)idx[88];
                v20 = (short)idx[104];
            }
            else
            {
                g_438010 = sub_417a00();
LABEL_40770b:
                v22 = (long long)idx[88];
                v20 = (short)idx[104];
            }
        }
        else
        {
            close(g_438010);
            v19 = g_438018;
            g_438010 = 0xffffffff;
LABEL_4076e0:
            v23 = index->field_2c;
            if (v19 & 2)
                goto LABEL_4076ea;
            goto LABEL_40770b;
        }
    }
    v24 = (long long)idx[120];
    *((unsigned int *)&g_438014) = v22;
    v4 = v24;
    if (v20 == 7)
    {
        sub_40e8c0((int)idx[64], (long long)idx[56]);
        return;
    }
    if (v20 != 4)
    {
        if (v20 == 2)
        {
            if (((short)(int)idx[0x88] & 0xf000) != 0xa000)
            {
                if (*((long long *)(*((long long *)idx) + 72)))
                {
                    v25 = *((long long *)(*((long long *)idx) + 56)) + *((long long *)(*((long long *)idx) + 72));
                    v26 = *(v25);
                    *(v25) = 0;
                    sub_4239f0(1);
                    *(v25) = v26;
                }
                else
                {
                    sub_4239f0(1);
                }
                sub_40e8a0(0, (long long)idx[56]);
                error(0, 0, dcgettext(NULL, "File system loop detected; %s is part of the same file system loop as %s.", 5));
            }
            else
            {
                sub_40e8a0(0, (long long)idx[56]);
                error(0, 0, dcgettext(NULL, "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.", 5));
            }
            g_438604 = 1;
            return;
        }
        if (v20 == 13)
        {
            if (g_438670((long long)idx[48], &v13) && *(__errno_location()) == 40)
                goto LABEL_407b65;
            v20 = (short)idx[104];
        }
        else if (v20 == 10)
        {
            if (!v22)
            {
                sub_40e8c0((int)idx[64], (long long)idx[56]);
                return;
            }
            if (g_438670((long long)idx[48], &v13) && *(__errno_location()) == 40)
            {
LABEL_407b65:
                sub_40e8c0(40, (long long)idx[56]);
                return;
            }
            sub_40e8c0((int)idx[64], (long long)idx[56]);
            v20 = (short)idx[104];
        }
    }
    else
    {
        sub_40e8c0((int)idx[64], (long long)idx[56]);
        if (g_438620)
            return;
        v20 = (short)idx[104];
    }
    if (v20 - 10 <= 1)
    {
        if (g_4385e4)
            __assert_fail(); /* do not return */
        v29 = g_4385e8;
        if (v20 != 11 && g_4385e8)
            __assert_fail(); /* do not return */
        v2 = g_4385e8;
LABEL_407a35:
        g_4385e0 = (long long)idx[88];
        if (!v29)
        {
            v38 = (short)idx[104];
LABEL_407a4b:
            if (v38 - 1 > 1 && v38 != 6)
            {
                v39 = g_438624;
                v40 = 0;
                v41 = 0;
                if (g_438624 < 0)
                    goto LABEL_407882;
LABEL_407862:
                if ((long long)idx[88] >= v39)
                {
                    sub_41a750(index, idx, 4);
                    v41 = g_438624 < (long long)idx[88];
                    v38 = (short)idx[104];
LABEL_40786e:
                    switch (v38)
                    {
                    case 1:
                        if (!g_438620)
                            goto LABEL_40792d;
                        else
                            goto LABEL_407882;
                    case 6:
                        if (!g_438620)
                            goto LABEL_407882;
                        else
                            goto LABEL_40792d;
                    default:
LABEL_407882:
                        v42 = *((long long *)&g_438660);
                        if ((long long)idx[88] >= g_438628)
                        {
                            if (!((int)*((long long *)&g_438660) & 4))
                                break;
                            goto LABEL_407bf5;
                        }
                    }
LABEL_40792d:
                    v42 = *((long long *)&g_438660);
                    if (!((char)v42 & 4))
                    {
LABEL_40793d:
                        if ((short)idx[104] != 6)
                            return;
                        g_438600 = 0;
                        return;
                    }
                    v41 = 1;
LABEL_407bf5:
                    v1 = g_4385e5;
                    v43 = (unsigned int)sub_407510(v38);
                    v44 = sub_4239f0(0, g_43868c, (long long)idx[56]);
                    v0 = v1;
                    __fprintf_chk(stderr, 0x1, "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n", v44, v43, v40, v41, (unsigned int)g_4385e4);
                    v38 = (short)idx[104];
                    if (!v41)
                    {
                        v45 = (long long)idx[48];
                        g_4385e4 = v38 - 10 > 1;
                        g_4385f0 = v45;
                        g_4385f8 = index->field_2c;
                        v46 = sub_40b580();
                        sub_40e7b0((long long)idx[56], &v3, v46);
                        if (g_438600)
                        {
                            sub_41a750(index, idx, 4);
                            goto LABEL_40793d;
                        }
                    }
                }
LABEL_40786c:
                v41 = 0;
                goto LABEL_40786e;
            }
LABEL_407847:
            if (v38 == 11)
            {
                sub_41a750(index, idx, 1);
                return;
            }
            v39 = g_438624;
            v40 = 1;
            if (g_438624 < 0)
                goto LABEL_40786c;
            goto LABEL_407862;
        }
LABEL_407808:
        if (!(char)sub_40dfe0(&v2, (long long)idx[56], idx + 0x100, &v3, 0))
            return;
        v38 = (short)idx[104];
        if ((v2 & 0xf000) == 0x4000)
            goto LABEL_407847;
        goto LABEL_407a4b;
    }
}



// Function: find @ 0x7e10
extern int g_438010;
extern unsigned int g_438018;
extern char g_4385e4;
extern char g_4385e5;
extern unsigned int g_4385e8;
extern unsigned int g_4385f8;
extern unsigned int g_4385fc;
extern unsigned int g_438604;
extern char g_438608;
extern char g_438618;
extern char g_43862d;
extern unsigned int g_438668;

unsigned int find(unsigned long a0)
{
    unsigned int v4;  // eax
    unsigned int v5;  // esi
    unsigned int v6;  // r13d
    unsigned long v7;  // rbp
    unsigned int *err;  // rbx
    void* idx;  // rsi
    unsigned int v10;  // r14d
    unsigned int v11;  // eax
    char *v12;  // rdx
    char *v13;  // rax
    void* v0;  // [bp-0x50]
    unsigned long v1;  // [bp-0x48]
    unsigned long long v2;  // [bp-0x40]

    v4 = strlen(a0);
    v5 = g_438018;
    g_4385fc = v4;
    if ((unsigned short)v5 & 0x200)
    {
        g_4385f8 = 0xffffff9c;
        if (g_438010 < 0)
            g_438010 = 0xffffff9c;
    }
    v1 = a0;
    v2 = 0;
    if (g_438668 == 1)
    {
        v5 |= 3;
        g_438018 = v5;
    }
    else if (g_438668 == 2)
    {
        v5 |= 0x11;
        g_438018 = v5;
    }
    else if (!g_438668)
    {
        v5 |= 16;
        g_438018 = v5;
    }
    if (g_43862d)
        g_438018 = v5 | 64;
    v6 = 0x80000000;
    v7 = sub_4199d0(&v1);
    err = __errno_location();
    if (!v7)
    {
        sub_40e8a0(0, a0);
        v13 = dcgettext(NULL, "cannot search %s", 5);
        error(0, *(err), v13);
        g_438604 = 1;
        return 1;
    }
    while (1)
    {
        *(err) = 0;
        idx = sub_419ff0(v7);
        if (!idx)
            break;
        v10 = (long long)idx[88];
        if (g_438608 && v6 != (int)(long long)idx[88])
        {
            v0 = idx;
            sub_40d9f0();
            idx = v0;
            v10 = (int)idx[88];
        }
        v11 = (int)idx[0x88];
        v6 = v10;
        g_438618 = 0;
        g_4385e4 = 0;
        g_4385e8 = v11;
        g_4385e5 = g_4385e8;
        sub_407670(v7, idx);
    }
    if (!*(err))
    {
        if (!sub_419e00(v7))
            return 1;
        v12 = dcgettext(NULL, "failed to restore working directory after searching %s", 5);
    }
    else
    {
        sub_40e8a0(0, a0);
        v12 = "failed to read file names from file system at or below %s";
    }
    error(0, *(err), v12);
    g_438604 = 1;
    return 0;
}



// Function: process_all_startpoints @ 0x8070
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

extern unsigned long long stdin;
extern unsigned int g_4385fc;
extern unsigned int g_438604;
extern unsigned long long g_438690;
extern char g_438698;

unsigned long long process_all_startpoints(int a0, long long *a1)
{
    unsigned long long v7;  // rdi
    char *v16;  // rax
    int *err;  // rax
    unsigned int v8;  // eax
    long long v11;  // rax
    char *v12;  // rax
    int *err1;  // rax
    unsigned int v14;  // r12d
    char *v15;  // r14
    unsigned long long fp;  // [bp-0x180]
    unsigned int v1;  // [bp-0x16c]
    stat v2;  // [bp-0x168]
    stat v3;  // [bp-0xd8]
    unsigned short v4;  // [bp-0x42]

    if (!(a0 <= 0 || (char)sub_40e120(*(a1), 1)))
    {
        fp = g_438690;
        if (g_438690)
        {
            sub_40e8a0(0, *(a1));
            error(0, 0, dcgettext(NULL, "extra operand %s", 5));
            dcgettext(NULL, "file operands cannot be combined with -files0-from", 5);
            error(1, 0, "%s");
        }
        v11 = sub_417670(a1);
    }
    else if (g_438690)
    {
        if (strcmp(g_438690, "-"))
        {
            sub_40e8a0(0, g_438690);
            fp = fopen(g_438690, "r");
            v7 = fp;
            if (!fp)
            {
                v12 = dcgettext(v7, "cannot open %s for reading", 5);
                err1 = __errno_location();
                error(1, *(err1), v12);
            }
            v8 = fileno(v7);
            if (v8 < 0)
                __assert_fail(); /* do not return */
            if (g_438698 && !fstat(v8, &v2) && !fstat(0, &v3) && v2.st_ino == v3.st_ino && *((unsigned long long *)&v2) == *((unsigned long long *)&v3))
            {
                dcgettext(NULL, "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir", 5);
                error(1, 0, "%s: %s\n");
            }
            sub_417990(v8, 1);
        }
        else if (!g_438698)
        {
            sub_40e8a0(0, dcgettext(NULL, "(standard input)", 5));
            fp = stdin;
        }
        else
        {
            dcgettext(NULL, "option -files0-from reading from standard input cannot be combined with -ok, -okdir", 5);
            error(1, 0, "%s\n");
        }
        v11 = sub_4176a0(fp);
    }
    else
    {
        v4 = 46;
        return sub_407e10(&v4) & 0xffffffff;
    }
    if (v11)
    {
        v14 = 1;
        while (1)
        {
            v15 = sub_4176e0(v11, &v1);
            if (!v15)
            {
                if (v1 != 3)
                {
                    if (v1 == 4)
                    {
                        v16 = dcgettext(NULL, "%s: read error", 5);
                        err = __errno_location();
                        error(0, *(err), v16);
                        g_438604 = 1;
                        sub_4177a0(v11);
                        return 0;
                    }
                    if (v1 != 2)
                        __assert_fail(); /* do not return */
LABEL_408176:
                    sub_4177a0(v11);
                    if (!(char)v14)
                        return v14;
                    if (g_438690)
                    {
                        if (!ferror(fp) && !sub_417d40(fp))
                            return v14;
                        error(1, 0, dcgettext(NULL, "error reading %s", 5));
                    }
                    else
                    {
                        return v14;
                    }
                }
                break;
            }
            else if (*(v15))
            {
                if (!g_438690 && (char)sub_40e120(v15, 1))
                    goto LABEL_408176;
                g_4385fc = strlen(v15);
                if ((char)sub_407e10(v15))
                    continue;
                v14 = 0;
                goto LABEL_408176;
            }
            else
            {
                if (g_438690)
                {
                    sub_417780(v11);
                    dcgettext(NULL, "invalid zero-length file name", 5);
                    error(0, 0, "%s:%lu: %s");
                }
                else
                {
                    sub_40e8a0(0, v15);
                    error(0, 2, "%s");
                }
                g_438604 = 1;
                v14 = 0;
            }
        }
    }
    sub_425f50(); /* do not return */
}



// Function: is_fts_enabled @ 0x8530
extern unsigned int g_438018;

unsigned int is_fts_enabled(unsigned int *a0)
{
    *(a0) = g_438018;
    return 1;
}



// Function: is_ok @ 0x8550
typedef struct FILE {
} FILE;

extern FILE *stdin;
extern long long stderr;
extern long long stdout;

unsigned int is_ok(unsigned long a0, unsigned long a1)
{
    unsigned long long v4;  // rbx
    char *v5;  // rax
    unsigned int v6;  // r12d
    unsigned long v7;  // fs
    long long v8;  // rax
    char *v9;  // rdi
    void* v10;  // rax
    char *v11;  // rax
    int *err;  // rax
    char *v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10], Other Possible Types: unsigned long long

    v2 = v4;
    sub_4180c0(stdout);
    v5 = dcgettext(NULL, "< %s ... %s > ? ", 5);
    if (__fprintf_chk(stderr, 0x1, v5) >= 0)
    {
        sub_4180c0(stderr);
        v6 = 0;
        v2 = *((long long *)(40 + v7));
        v0 = NULL;
        v1 = 0;
        v8 = __getdelim(&v0, &v1, 10, stdin);
        if (v8 > 0)
        {
            v9 = v0;
            v10 = &v9[v8] - 1;
            if (*((char *)v10) == 10)
            {
                *((char *)v10) = 0;
                v9 = v0;
            }
            v6 = rpmatch(v9) > 0;
        }
        free(v0);
        if (v2 != *((long long *)(40 + v7)))
            __stack_chk_fail(); /* do not return */
        return v6;
    }
    v11 = dcgettext(NULL, "Failed to write prompt for -ok", 5);
    err = __errno_location();
    error(1, *(err), v11);
}



// Function: print_parenthesised @ 0x8870
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[264];
    unsigned long long field_110;
    struct struct_0 *field_118;
} struct_0;

long long print_parenthesised(long long a0, struct_0 *a1)
{
    unsigned long long v3;  // r13
    unsigned long long v4;  // r12
    struct_0 *v5;  // rbp
    unsigned long v6;  // rax
    unsigned long v7;  // rax
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    if (!a1)
        return v7;
    v1 = v3;
    v0 = v4;
    while (1)
    {
        v5 = a1;
        v6 = v5->field_0;
        if (v6 == sub_408940 || v6 == sub_408990)
        {
            if (v5->field_110)
                break;
            a1 = v5->field_118;
            if (!v5->field_118)
                return v6;
        }
        else if (!v5->field_110)
        {
            if (!v5->field_118)
                return sub_4086e0();
            break;
        }
    }
    __fprintf_chk(a0, 0x1, "%s", " ( ");
    sub_4086e0(a0, v5);
    return __fprintf_chk(a0, 0x1, "%s");
}



// Function: pred_amin @ 0x8b10
void pred_amin(unsigned long a0, unsigned long a1)
{
}



// Function: pred_anewer @ 0x8b30
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;

double pred_anewer(long long a0, void* a1, void* a2)
{
    uint128_t t;  // xmm0
    unsigned long v2;  // xmm0lq

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    if (*((long long *)(a1 + 72)) == *((long long *)&a2->field_40) && *((long long *)(a1 + 80)) == a2->field_48)
        return v2;
    t = (int128_t)difftime(*((long long *)(a1 + 72)), *((long long *)&a2->field_40));
    return (unsigned long long)(AddV(MulV(*((long long *)(a1 + 80)) - a2->field_48, 4472406533629990549), t));
}



// Function: pred_atime @ 0x8bc0
void pred_atime(unsigned long a0, unsigned long a1)
{
}



// Function: pred_cmin @ 0x8bf0
void pred_cmin(unsigned long a0, unsigned long a1)
{
}



// Function: pred_cnewer @ 0x8c10
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;

double pred_cnewer(long long a0, void* a1, void* a2)
{
    uint128_t t;  // xmm0
    unsigned long v2;  // xmm0lq

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    if (*((long long *)(a1 + 104)) == *((long long *)&a2->field_40) && *((long long *)(a1 + 112)) == a2->field_48)
        return v2;
    t = (int128_t)difftime(*((long long *)(a1 + 104)), *((long long *)&a2->field_40));
    return (unsigned long long)(AddV(MulV(*((long long *)(a1 + 112)) - a2->field_48, 4472406533629990549), t));
}



// Function: pred_comma @ 0x8ca0
typedef struct struct_0 {
    char padding_0[272];
    unsigned long long field_110;
    long long field_118;
} struct_0;

void pred_comma(long long a0, long long a1, struct_0 *a2)
{
    if (a2->field_110)
        sub_40e7b0();
    sub_40e7b0(a0, a1, a2->field_118);
    return;
}



// Function: pred_ctime @ 0x8ce0
void pred_ctime(unsigned long a0, unsigned long a1)
{
}



// Function: pred_delete @ 0x8d00
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4385e4;
extern void g_4385f0;
extern unsigned int g_4385f8;
extern unsigned int g_438604;
extern char g_43862e;

unsigned long long pred_delete(long long a0, struct_0 *a1)
{
    unsigned int v1;  // r13d
    int *err;  // rbx
    char *v3;  // rax

    if (!strcmp(*((long long *)&g_4385f0), "."))
        return 1;
    v1 = 0;
    if (g_4385e4)
        v1 = ((a1->field_18 & 0xf000) == 0x4000) * 0x200;
    if (unlinkat(g_4385f8, *((long long *)&g_4385f0), v1))
    {
        err = __errno_location();
        if (*(err) == 2)
        {
            if (g_43862e)
            {
                *(err) = 0;
                return g_43862e;
            }
LABEL_408db0:
            sub_40e8a0(0, a0);
            v3 = dcgettext(NULL, "cannot delete %s", 5);
            error(0, *(err), v3);
            g_438604 = 1;
            return 0;
        }
        else if (!(*(err) == 21 && !v1 && !unlinkat(g_4385f8, *((long long *)&g_4385f0), 0x200)))
        {
            goto LABEL_408db0;
        }
    }
    return 1;
}



// Function: pred_empty @ 0x8e20
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    char padding_1c[20];
    unsigned long long field_30;
} struct_0;

extern long long g_4385f0;
extern unsigned int g_4385f8;
extern unsigned int g_438604;

unsigned int pred_empty(long long a0, struct_0 *a1)
{
    unsigned int v1;  // eax
    int *err;  // rax
    int *v3;  // rbx
    unsigned int v4;  // eax
    DIR *fp;  // r13
    dirent *count;  // rax
    unsigned int v7;  // r12d

    switch (v1)
    {
    case 16384:
        err = __errno_location();
        *(err) = 0;
        v3 = err;
        v4 = sub_41f080(g_4385f8, g_4385f0, 0x90900);
        if (v4 >= 0)
        {
            fp = fdopendir(v4);
            if (!fp)
            {
                sub_40e8a0(0, a0);
                error(0, *(v3), "%s");
                g_438604 = 1;
                close(v4);
                return 0;
            }
            *(v3) = 0;
            do
            {
                if (!readdir(fp))
                {
                    v7 = 1;
                    break;
                }
            } while (*((char *)&count[1].d_ino + 6) == 46 && (!*((char *)&count[1].d_ino + 7) || *((short *)((char *)&count[1].d_ino + 7)) == 46));
            v7 = 0;
            if (*(v3))
            {
                sub_40e8a0(0, a0);
                error(0, *(v3), "%s");
                g_438604 = 1;
                closedir(fp);
                return 0;
            }
            else if (!closedir(fp))
            {
                return v7;
            }
        }
        sub_40e8a0(0, a0);
        error(0, *(v3), "%s");
        g_438604 = 1;
        return 0;
    case 32768:
        return !a1->field_30;
    default:
        return !a1->field_30;
    }
}



// Function: pred_execdir @ 0x8fc0
void pred_execdir(void)
{
}



// Function: pred_false @ 0x8fd0
unsigned int pred_false(void)
{
    return 0;
}



// Function: pred_fstype @ 0x9090
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_fstype(long long a0, long long a1, struct_0 *a2)
{
    char *v1;  // rax

    v1 = sub_4108e0(a1, a0);
    return !strcmp(v1, a2->field_38);
}



// Function: pred_gid @ 0x90c0
typedef struct struct_1 {
    char padding_0[32];
    unsigned int field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

unsigned long long pred_gid(unsigned long a0, struct_1 *a1, struct_0 *a2)
{
    unsigned int v1;  // eax

    v1 = a2->field_38;
    if (v1 == 1)
    {
        return _INSERT(0, 0, a1->field_20 < a2->field_40) & 0xffffffff;
    }
    else if (v1 == 2)
    {
        return _INSERT(0, 0, a1->field_20 == a2->field_40) & 0xffffffff;
    }
    else if (v1)
    {
        return 0;
    }
    else
    {
        return a1->field_20 > a2->field_40;
    }
}



// Function: pred_group @ 0x9110
void pred_group(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return;
}



// Function: pred_ilname @ 0x9120
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

unsigned long long pred_ilname(unsigned long a0, struct_0 *a1, unsigned long a2)
{
    if (((unsigned short)a1->field_18 & 0xf000) != 0xa000)
        return 0;
}



// Function: pred_iname @ 0x9150
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_iname(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long v1;  // rbp
    unsigned int v2;  // eax

    v1 = sub_417b90();
    sub_417d00(v1);
    v2 = fnmatch(a2->field_38, v1, 16);
    free(v1);
    return !v2;
}



// Function: pred_inum @ 0x9190
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

unsigned long long pred_inum(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned int v1;  // eax

    v1 = a2->field_38;
    if (v1 == 1)
    {
        return _INSERT(0, 0, *((long long *)(a1 + 8)) < a2->field_40) & 0xffffffff;
    }
    else if (v1 == 2)
    {
        return _INSERT(0, 0, *((long long *)(a1 + 8)) == a2->field_40) & 0xffffffff;
    }
    else if (v1)
    {
        return 0;
    }
    else
    {
        return *((long long *)(a1 + 8)) > a2->field_40;
    }
}



// Function: pred_ipath @ 0x91e0
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_ipath(char *a0, unsigned long a1, struct_0 *a2)
{
    return !fnmatch(a2->field_38, a0, 16);
}



// Function: pred_links @ 0x9210
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

unsigned long long pred_links(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned int v1;  // eax

    v1 = a2->field_38;
    if (v1 == 1)
    {
        return _INSERT(0, 0, *((long long *)(a1 + 16)) < a2->field_40) & 0xffffffff;
    }
    else if (v1 == 2)
    {
        return _INSERT(0, 0, *((long long *)(a1 + 16)) == a2->field_40) & 0xffffffff;
    }
    else if (v1)
    {
        return 0;
    }
    else
    {
        return *((long long *)(a1 + 16)) > a2->field_40;
    }
}



// Function: pred_lname @ 0x9260
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

unsigned long long pred_lname(unsigned long a0, struct_0 *a1, unsigned long a2)
{
    if (((unsigned short)a1->field_18 & 0xf000) != 0xa000)
        return 0;
}



// Function: pred_ls @ 0x9290
typedef struct struct_0 {
    char padding_0[40];
    char field_28;
    char padding_29[23];
    unsigned long long field_40;
} struct_0;

unsigned int pred_ls(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long v0;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    v1 = a2->field_40;
    v0 = a2->field_28;
    sub_4165e0();
    return 1;
}



// Function: pred_mmin @ 0x92d0
void pred_mmin(unsigned long a0, unsigned long a1)
{
}



// Function: pred_mtime @ 0x92f0
void pred_mtime(unsigned long a0, unsigned long a1)
{
}



// Function: pred_name @ 0x9310
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_name(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long v1;  // rbp
    unsigned int v2;  // eax

    v1 = sub_417b90();
    sub_417d00(v1);
    v2 = fnmatch(a2->field_38, v1, 0);
    free(v1);
    return !v2;
}



// Function: pred_negate @ 0x9350
int pred_negate(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return sub_40e7b0() ^ 1;
}



// Function: pred_newer @ 0x9370
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;

double pred_newer(long long a0, void* a1, void* a2)
{
    uint128_t t;  // xmm0
    unsigned long v2;  // xmm0lq

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    if (*((long long *)(a1 + 88)) == *((long long *)&a2->field_40) && *((long long *)(a1 + 96)) == a2->field_48)
        return v2;
    t = (int128_t)difftime(*((long long *)(a1 + 88)), *((long long *)&a2->field_40));
    return (unsigned long long)(AddV(MulV(*((long long *)(a1 + 96)) - a2->field_48, 4472406533629990549), t));
}



// Function: pred_newerXY @ 0x9400

unsigned int pred_newerXY(long long a0, void* idx, void* a2)
{
    unsigned long v1;  // rdi
    unsigned long long v2;  // rbx
    uint128_t t;  // xmm0
    unsigned long long v5;  // rbx

    if ((int)a2[60])
        __assert_fail(); /* do not return */
    switch ((int)a2[56])
    {
    case 0:
        v1 = (long long)idx[72];
        v2 = (long long)idx[80];
        if ((long long)a2[64] == v1 && v2 == (long long)a2[72])
            return 0;
        t = (int128_t)difftime(v1, (long long)a2[64]);
        v5 = v2 - (long long)a2[72];
        return _INSERT(0, 0, ((CmpF(0, (unsigned long long)(AddV(MulV(v5, 4472406533629990549), t))) & 69 | (CmpF(0, (unsigned long long)(AddV(MulV(v5, 4472406533629990549), t))) & 69) >> 6) & 1) == 1);
    case 1:
        sub_40e8a0(0, a0);
        error(0, 0, dcgettext(NULL, "WARNING: cannot determine birth time of file %s", 5));
        return 0;
    case 2:
        v1 = (long long)idx[104];
        v2 = (long long)idx[112];
        break;
    case 3:
        v1 = (long long)idx[88];
        v2 = (long long)idx[96];
        break;
    case 4:
        __assert_fail(); /* do not return */
    default:
        __assert_fail(); /* do not return */
    }
}



// Function: pred_nogroup @ 0x9540
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

char pred_nogroup(unsigned long a0, struct_0 *a1)
{
    return !getgrgid(a1->field_20);
}



// Function: pred_nouser @ 0x9560
typedef struct struct_0 {
    char padding_0[28];
    unsigned int field_1c;
} struct_0;

char pred_nouser(unsigned long a0, struct_0 *a1)
{
    return !getpwuid(a1->field_1c);
}



// Function: pred_ok @ 0x9580
typedef struct struct_0 {
    char padding_0[232];
    struct struct_1 *field_e8;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

unsigned int pred_ok(unsigned int a0, unsigned long a1, struct_0 *a2)
{
    if (!(char)sub_408550(*((long long *)&a2->field_e8), a0))
        return 0;
}



// Function: pred_okdir @ 0x95d0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    long long field_0;
    char padding_8[48];
    char field_38;
    char padding_39[151];
    unsigned int field_d0;
    char padding_d4[20];
    struct struct_2 *field_e8;
    int field_f0;
    char padding_f4[4];
    struct struct_0 *field_f8;
    unsigned int field_100;
} struct_1;

typedef struct struct_2 {
    unsigned int field_0;
} struct_2;

extern char g_4385c0;
extern void* g_4385f0;
extern unsigned int g_4385f8;
extern char g_438608;

int pred_okdir(void)
{
    unsigned long long v14;  // r12
    struct_1 *idx;  // rdx
    unsigned long ptr;  // r13
    struct_0 *v25;  // rax
    unsigned long v26;  // rdi
    struct_0 *v27;  // rax
    unsigned long v28;  // rdi
    char *v29;  // rax
    int *err;  // rax
    long long v31;  // rax
    struct_1 *v32;  // r14
    struct_1 *v33;  // r13
    unsigned int v16;  // rdi
    unsigned long long index;  // rbx
    unsigned long long v17;  // r15
    unsigned long long v18;  // r14
    unsigned long long v19;  // r13
    void* ptr1;  // r12
    unsigned long long v21;  // rbx
    unsigned long long v22;  // r15
    void* v23;  // r13
    unsigned long v0;  // [bp-0x60]
    unsigned long len2;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    char v3;  // [bp-0x41]
    void* v4;  // [bp-0x40]
    unsigned long long v5;  // [bp-0x30]
    char *v6;  // [bp-0x28]
    unsigned long long v7;  // [bp-0x20]
    unsigned long long v8;  // [bp-0x18]
    unsigned long long v9;  // [bp-0x10]
    unsigned long long v10;  // [bp-0x8]
    unsigned long long v11;  // [bp-0x8]
    char v12;  // [bp+0x0]

    v10 = v14;
    if (!(char)sub_408550(*((long long *)&idx->field_e8), v16))
        return;
    v11 = v17;
    v9 = v18;
    v8 = v19;
    v7 = v10;
    ptr1 = g_4385f0;
    v6 = &v12;
    v5 = v21;
    v3 = sub_40e880(idx->field_0);
    if (v3)
    {
        v23 = g_4385f0;
        if (!idx->field_d0)
        {
            if (strchr(g_4385f0, 47))
            {
                ptr = sub_417c40(g_4385f0);
                v25 = sub_425a50(16);
                idx->field_f8 = v25;
                v25->field_8 = 0;
                v25->field_0 = sub_41f080(g_4385f8, ptr, 0);
                v26 = idx->field_f8->field_0;
                if ((unsigned int)v26 < 0)
                {
                    free(ptr);
LABEL_40a33f:
                    sub_40e8a0(0, g_4385f0);
                    v29 = dcgettext(NULL, "Failed to save working directory in order to run a command on %s", 5);
                    err = __errno_location();
                    error(1, *(err), v29);
                }
                sub_417990(v26, 1);
                free(ptr);
                v23 = g_4385f0;
            }
            else
            {
                v27 = sub_425a50(16);
                idx->field_f8 = v27;
                v27->field_8 = 0;
                v27->field_0 = sub_41f080(g_4385f8, ".", 0);
                v28 = idx->field_f8->field_0;
                if ((unsigned int)v28 < 0)
                    goto LABEL_40a33f;
                sub_417990(v28, 1);
                v23 = g_4385f0;
            }
        }
        ptr1 = sub_417b90(v23);
        v4 = ptr1;
        v22 = (*((char *)ptr1) != 47) * 2;
        v31 = 0;
        if (*((char *)ptr1) != 47)
            v31 = "./";
        v2 = v31;
    }
    else if (idx->field_f8 == *((long long *)&g_4385c0))
    {
        v2 = 0;
        v22 = 0;
        v4 = NULL;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
    v32 = &idx->padding_39[95];
    v33 = &idx->padding_39[7];
    if (!idx->field_38)
    {
        index = 0;
        if (idx->field_f0 > 0)
        {
            do
            {
                len2 = strlen(ptr1);
                v0 = *((long long *)&idx->field_e8[2 * index]);
                sub_415270(v33, v32, v0, strlen(*((long long *)&idx->field_e8[2 * index])), v2, v22, ptr1, len2, 0);
                index += 1;
            } while (idx->field_f0 > (unsigned int)index);
        }
        sub_415430(v33, v32);
        if (!v3)
        {
            if (!v4)
                return;
            __assert_fail(); /* do not return */
        }
        sub_4242c0(idx->field_f8);
        if (!v4)
            return;
        v3 = !(char)idx->field_100 && !(idx->field_100 & 127);
    }
    else
    {
        sub_414fb0(v33, v32, ptr1, strlen(ptr1) + 1, v2, v22, 0);
        if (idx->field_d0)
            g_438608 = 1;
        if (!v4)
        {
            return;
        }
        else if (!v3)
        {
            __assert_fail(); /* do not return */
        }
    }
    free(v4);
    return;
}



// Function: pred_path @ 0x9640
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_path(char *a0, unsigned long a1, struct_0 *a2)
{
    return !fnmatch(a2->field_38, a0, 0);
}



// Function: pred_perm @ 0x9660
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[56];
    unsigned int field_38;
} struct_1;

unsigned long long pred_perm(unsigned long a0, struct_0 *a1, struct_1 *a2)
{
    unsigned long v2;  // rax
    unsigned int v3;  // ecx
    unsigned int v4;  // edx
    unsigned long v0;  // [bp-0x8]

    v2 = a1->field_18;
    v3 = *((int *)&a2[1].padding_0[4 * (((unsigned int)v2 & 0xf000) == 0x4000)]);
    v4 = a2->field_38;
    if (v4 != 1)
    {
        if (v4 == 2)
        {
            return _INSERT(0, 0, ((unsigned int)v2 & 0xfff) == v3) & 0xffffffff;
        }
        else if (v4)
        {
            v0 = v2;
            abort(); /* do not return */
        }
        else
        {
            return _INSERT(0, 0, ((unsigned int)v2 & v3) == v3) & 0xffffffff;
        }
    }
    else
    {
        if (!v3)
            return 1;
        return v3 & (unsigned int)v2;
    }
}



// Function: pred_executable @ 0x96e0
extern long long g_4385f0;
extern unsigned int g_4385f8;

char pred_executable(void)
{
    return !faccessat(g_4385f8, g_4385f0, 1, 0);
}



// Function: pred_readable @ 0x9710
extern long long g_4385f0;
extern unsigned int g_4385f8;

char pred_readable(void)
{
    return !faccessat(g_4385f8, g_4385f0, 4, 0);
}



// Function: pred_writable @ 0x9740
extern long long g_4385f0;
extern unsigned int g_4385f8;

char pred_writable(void)
{
    return !faccessat(g_4385f8, g_4385f0, 2, 0);
}



// Function: pred_regex @ 0x97e0
typedef struct struct_0 {
    char padding_0[56];
    long long field_38;
} struct_0;

char pred_regex(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned int v1;  // eax

    v1 = strlen(a0);
    return re_match(a2->field_38, a0, v1, 0, 0) == v1;
}



// Function: pred_size @ 0x9820
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    int field_3c;
    unsigned long long field_40;
} struct_0;

unsigned long long pred_size(unsigned long a0, void* a1, struct_0 *a2)
{
    unsigned long v1;  // rsi
    unsigned int v2;  // edx
    unsigned long long v3;  // rax

    v1 = a2->field_3c;
    v2 = a2->field_38;
    v3 = (long long)a1[48] / v1 + 1 - ((long long)a1[48] % v1 < 1);
    if (v2 == 1)
    {
        return _INSERT(a1, 0, a2->field_40 > v3) & 0xffffffff;
    }
    else if (v2 == 2)
    {
        return _INSERT(a1, 0, a2->field_40 == v3) & 0xffffffff;
    }
    else if (v2)
    {
        return 0;
    }
    else
    {
        return a2->field_40 < v3;
    }
}



// Function: pred_samefile @ 0x9890
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int pred_samefile(void)
{
    struct_0 *v1;  // rsi
    unsigned long v2;  // rdx

    if ((!v1->field_8 || v1->field_8 == *((long long *)(v2 + 56))) && !sub_40d820())
        return;
    return;
}



// Function: pred_type @ 0x9900
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4385e4;
extern char g_4385e5;
extern unsigned int g_4385e8;

unsigned int pred_type(unsigned long a0, struct_0 *a1, unsigned long a2)
{
    unsigned int v2;  // eax
    unsigned long long v4;  // rcx
    unsigned long long v5;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!g_4385e5)
    {
        v0 = v5;
        __assert_fail(); /* do not return */
    }
    v2 = g_4385e8;
    if (!g_4385e8)
        return 0;
    if (g_4385e4)
        v2 = a1->field_18;
    switch (v2 & 0xf000)
    {
    case 24576:
        v4 = 0;
        break;
    case 40960:
        v4 = 4;
        break;
    case 8192:
        v4 = 1;
        break;
    case 49152:
        v4 = 6;
        break;
    case 16384:
        v4 = 2;
        break;
    case 32768:
        v4 = 3;
        break;
    case 4096:
        v4 = 5;
        break;
    default:
        return 0;
    }
    return *((char *)(a2 + v4 + 56));
}



// Function: pred_uid @ 0x99e0
typedef struct struct_1 {
    char padding_0[28];
    unsigned int field_1c;
} struct_1;

typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

unsigned long long pred_uid(unsigned long a0, struct_1 *a1, struct_0 *a2)
{
    unsigned int v1;  // eax

    v1 = a2->field_38;
    if (v1 == 1)
    {
        return _INSERT(0, 0, a1->field_1c < a2->field_40) & 0xffffffff;
    }
    else if (v1 == 2)
    {
        return _INSERT(0, 0, a1->field_1c == a2->field_40) & 0xffffffff;
    }
    else if (v1)
    {
        return 0;
    }
    else
    {
        return a1->field_1c > a2->field_40;
    }
}



// Function: pred_used @ 0x9a30
typedef struct struct_0 {
    char padding_0[104];
    int field_68;
    char padding_6c[4];
    unsigned long long field_70;
} struct_0;

unsigned long long pred_used(unsigned long a0, struct_0 *idx, unsigned long a2)
{
    unsigned long long v1;  // r12
    unsigned long long v2;  // rbx

    v1 = *((long long *)&idx->padding_0[80]);
    v2 = idx->field_70;
    if (!(*((long long *)&idx->padding_0[72]) == *((long long *)&idx->field_68) & v1 == v2) && ((CmpF(0, (unsigned long long)(AddV(MulV(v1 - v2, 4472406533629990549), (int128_t)difftime(*((long long *)&idx->padding_0[72]), *((long long *)&idx->field_68))))) & 69 | (char)((CmpF(0, (unsigned long long)(AddV(MulV(v1 - v2, 4472406533629990549), (int128_t)difftime(*((long long *)&idx->padding_0[72]), *((long long *)&idx->field_68))))) & 69) >> 6)) & 1) != 1)
        return 0;
}



// Function: pred_user @ 0x9ae0
void pred_user(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return;
}



// Function: pred_xtype @ 0x9af0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern unsigned long long g_40ddb0;
extern unsigned long long g_4385f0;
extern unsigned int g_438604;

unsigned int pred_xtype(long long a0, struct_0 *a1, unsigned int a2)
{
    unsigned long long *v2;  // rbx
    char v3;  // al
    int *err;  // rax
    struct_0 v0;  // [bp-0xc8]

    v2 = (!(char)sub_40dfb0() ? &g_40ddb0 : sub_40de70);
    sub_40d810(&v0);
    if (!v2(g_4385f0, &v0))
        return sub_409900(a0, &v0, a2);
    v3 = sub_40dfb0();
    err = __errno_location();
    if (v3 && *(err) == 2)
        return sub_409900(a0, a1, a2);
    sub_40e8a0(0, a0);
    error(0, *(err), "%s");
    g_438604 = 1;
    return 0;
}



// Function: pred_context @ 0x9c00
typedef struct struct_1 {
    char padding_0[56];
    char *field_38;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_4385f0;
extern unsigned int g_4385f8;
extern struct_0 *g_438680;

unsigned int pred_context(long long a0, unsigned long a1, struct_1 *a2)
{
    unsigned long v2;  // r12
    unsigned int v3;  // r12d
    char *v4;  // rax
    int *err;  // rax
    unsigned long v0;  // [bp-0x38]

    if (g_438680(g_4385f8, g_4385f0, &v0) < 0)
    {
        sub_40e8a0(0, a0);
        v4 = dcgettext(NULL, "getfilecon failed: %s", 5);
        err = __errno_location();
        error(0, *(err), v4);
        return 0;
    }
    v3 = _INSERT(v2, 0, !fnmatch(a2->field_38, v0, 0));
    freecon(v0);
    return v3;
}



// Function: print_list @ 0x9cd0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char padding_10[248];
    struct struct_0 *field_108;
} struct_0;

void print_list(int a0, struct_0 *a1)
{
    struct_0 *v2;  // r13
    struct_0 *v3;  // r13
    unsigned int v4;  // edx
    unsigned int v5;  // edx
    char *v6;  // rax
    char *v7;  // rax
    char v0[264];  // [bp-0x138]

    v2 = a1;
    if (!a1)
    {
        __fprintf_chk(a0, 0x1, "\n");
        return;
    }
    do
    {
        v3 = v2;
        if (v3->field_8)
        {
            __strcpy_chk(v0, v3->field_8, 0x100);
            v4 = strlen(v0);
            v5 = v4 - 1;
            if (v4 - 1 >= 0)
            {
                v6 = &v0[v5];
                do
                {
                    v7 = v6;
                    if (*(v7) != 32 && *(v7) != 9)
                    {
                        v7 = &v0[1 + v5];
                        break;
                    }
                } while ((v5 -= 1, v6 = v7 - 1, v5 != 0xffffffff));
            }
            else
            {
                v7 = &v0[v4];
            }
            *(v7) = 0;
        }
    } while ((__fprintf_chk(a0, 0x1, "[%s] "), v2 = (struct_0 *)v3->field_108, v3->field_108));
    __fprintf_chk(a0, 0x1, "\n");
    return;
}



// Function: show_success_rates @ 0x9e00
extern void stderr;
extern char g_438660;

int show_success_rates(long long a0)
{
    int v1;  // eax
    unsigned int v2;  // edi

    if (!(g_438660 & 64))
        return v1;
    __fprintf_chk(*((long long *)&stderr), 0x1, "Predicate success rates after completion:\n");
    v2 = *((long long *)&stderr);
    if (a0)
    {
        sub_4086e0(*((long long *)&stderr), a0);
        v2 = *((long long *)&stderr);
    }
    return __fprintf_chk(v2, 0x1, "\n");
}



// Function: pred_sanity_check @ 0x9e60
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[16];
    char field_18;
    char field_19;
    char padding_1a[238];
    struct struct_0 *field_108;
    char padding_110[32];
    struct struct_1 *field_130;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;


void pred_sanity_check(struct_0 *a0)
{
    struct_0 *idx;  // rdi
    unsigned long long v2;  // rdx
    unsigned int *v3;  // rax
    unsigned int v4;  // ecx

    if (!a0)
        return;
    while (1)
    {
        idx = a0;
        v2 = idx->field_0;
        if (!v2)
            __assert_fail(); /* do not return */
        v3 = &idx->field_130->field_0;
        if (!v3)
            __assert_fail(); /* do not return */
        if (v2 != *((long long *)&v3[6]) && *((long long *)&v3[6]))
            __assert_fail(); /* do not return */
        v4 = *(v3);
        switch (v4)
        {
        case 0: case 2:
            if (v4)
                __assert_fail(); /* do not return */
            __assert_fail(); /* do not return */
        case 1: case 3: case 4: case 5:
            if (idx->field_19)
            {
                __assert_fail(); /* do not return */
            }
            else if (idx->field_18)
            {
                __assert_fail(); /* do not return */
            }
        case 6:
            if (!idx->field_18)
            {
                __assert_fail(); /* do not return */
            }
            else if (v2 != sub_408600 && v2 != sub_4085e0 && !idx->field_19)
            {
                __assert_fail(); /* do not return */
            }
        default:
            a0 = idx->field_108;
            if (!idx->field_108)
                return;
            break;
        }
    }
}



// Function: launch @ 0xa3a0
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
    char padding_1[95];
    unsigned long long field_60;
    unsigned long long field_68;
    char padding_70[76];
    char field_bc;
    char padding_bd[3];
    struct struct_0 *field_c0;
    unsigned int field_c8;
} struct_1;

extern unsigned int g_43801c;
extern long long stderr;
extern long long stdout;
extern unsigned int g_438604;
extern char g_438660;
extern unsigned int g_43868c;

int launch(void)
{
    unsigned long long i;  // rbx
    struct_1 *v3;  // rsi
    char v12;  // bl
    unsigned int v13;  // ebx
    long long *v14;  // rcx
    char *v17;  // rax
    char *v18;  // rax
    int *err;  // rax
    unsigned int v5;  // r13d
    struct_0 *v6;  // r13
    int *err1;  // rbx
    char *v8;  // rax
    char *v10;  // rax
    int *err2;  // rax
    char v0[10];  // [bp-0x3a]

    if (g_438660 & 32)
    {
        i = 0;
        __fprintf_chk(stderr, 0x1, "DebugExec: launching process (argc=%lu):", v3->field_60 - 1);
        if (v3->field_60 != 1)
        {
            do
            {
                sub_40e8a0(0, *((long long *)(v3->field_68 + i * 8)));
                __fprintf_chk(stderr, 0x1, " %s");
                i += 1;
            } while (v3->field_60 - 1 > i);
        }
        __fprintf_chk(stderr, 0x1, "\n");
    }
    sub_4180c0(stdout);
    sub_4180c0(stderr);
    if (g_43801c)
    {
        g_43801c = 0;
        signal(0x11, NULL);
    }
    v5 = fork();
    if (v5 != 0xffffffff)
    {
        if (!v5)
        {
            v6 = v3->field_c0;
            if (!v6)
                __assert_fail(); /* do not return */
            err1 = __errno_location();
            if (!v3->field_bc)
            {
LABEL_40a437:
                if (!sub_424290(v6))
                    goto LABEL_40a44e;
            }
            else
            {
                strncpy(v0, "/dev/null", 9);
                if (close(0) < 0)
                {
                    v8 = dcgettext(NULL, "Cannot close standard input", 5);
                    error(0, *(err1), v8);
                    if (!sub_424290(v6))
                        _exit(1); /* do not return */
                }
                else
                {
                    if (sub_418040(v0, 0) >= 0)
                        goto LABEL_40a437;
                    sub_40e8a0(0, v0);
                    error(0, *(err1), "%s");
                    if (!sub_424290(v6))
                    {
LABEL_40a44e:
                        if ((char)sub_416220())
                            sub_416240();
                        if ((char)sub_415970(v14))
                            *(err1) = 7;
                        else
                            execvp(*(v14), v14);
                        sub_40e8a0(0, *(v14));
                        error(0, *(err1), "%s");
                        _exit(1); /* do not return */
                    }
                }
            }
            v10 = dcgettext(NULL, "Failed to change directory%s%s", 5);
            error(0, *(err1), v10);
            _exit(1); /* do not return */
        }
        else
        {
            while (waitpid(v5, &v3->field_c8, 0) == 0xffffffff)
            {
                err2 = __errno_location();
                if (*(err2) != 4)
                {
                    sub_40e8a0(0, *(v14));
                    v17 = dcgettext(NULL, "error waiting for %s", 5);
                    error(0, *(err2), v17);
                    g_438604 = 1;
                    return;
                }
            }
            v12 = v3->field_c8;
            if ((v12 & 127) + 1 <= 1)
            {
                v13 = v12;
                if (g_438660 & 32)
                    __fprintf_chk(stderr, 0x1, "DebugExec: process (PID=%ld) terminated with exit status: %d\n", v5, v13 & 0xff);
                if (!v13)
                    return;
            }
            else
            {
                sub_4239f0(0, g_43868c, *(v14));
                error(0, 0, dcgettext(NULL, "%s terminated by signal %d", 5));
            }
            if (!v3->field_0)
                return;
            g_438604 = 1;
            return;
        }
    }
    v18 = dcgettext(NULL, "cannot fork", 5);
    err = __errno_location();
    error(1, *(err), v18);
}



// Function: check_normalization @ 0xa840
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[252];
    struct struct_0 *field_110;
    void* field_118;
} struct_1;

void check_normalization(unsigned long a0, char a1)
{
    char v2;  // sil
    struct_1 *v3;  // rdi

    if (v2 && v3->field_10 != 3)
        __assert_fail(); /* do not return */
}



// Function: get_expr @ 0xa8c0
typedef struct struct_4 {
    char padding_0[16];
    unsigned int field_10;
} struct_4;

typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    int field_14;
    char padding_18[17];
    char field_29;
    char padding_2a[230];
    struct struct_1 *field_110;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    short field_14;
    char padding_16[19];
    char field_29;
    char padding_2a[222];
    struct struct_0 *field_108;
    char padding_110[8];
    unsigned long long field_118;
} struct_1;


struct_1 * get_expr(struct_1 **a0, short a1, struct_4 *a2)
{
    struct_1 *idx;  // r12
    struct_0 *v2;  // rax
    unsigned int v11;  // r13d
    unsigned long v12;  // rsi
    unsigned long v3;  // 4113
    struct_1 *v6;  // rax
    struct_0 *index;  // rdx
    int v10;  // ebp

    idx = *(a0);
    if (!idx)
        error(1, 0, dcgettext(NULL, "invalid expression", 5));
    switch (idx->field_10)
    {
    case 0:
        error(1, 0, dcgettext(NULL, "invalid expression", 5));
    case 1:
        index = idx->field_108;
        *(a0) = index;
        if (!index)
            return idx;
        v10 = a1;
        if (index->field_14 <= v10)
            return idx;
        v11 = index->field_10;
        if (v11 != 5)
        {
            if (v11 != 4)
            {
                if (v11 <= 4)
                {
                    do
                    {
                        if (v11 != 3)
                            error(1, 0, dcgettext(NULL, "invalid expression", 5));
                        index->field_110 = idx;
                        idx = *(a0);
                        v12 = idx->field_14;
                        *(a0) = idx->field_108;
                        idx->field_118 = get_expr(a0, v12, index);
                        index = *(a0);
                        if (!index)
                            return idx;
                        if (v10 >= index->field_14)
                            return idx;
                        v11 = index->field_10;
                        if (v11 == 4)
                            error(1, 0, dcgettext(NULL, "invalid expression", 5));
                    } while (v11 <= 4);
                }
                if (v11 != 5)
                {
                    error(1, 0, dcgettext(NULL, "oops -- invalid expression type (%d)!", 5));
                }
                else if (idx)
                {
                    return idx;
                }
            }
            else
            {
                error(1, 0, dcgettext(NULL, "invalid expression", 5));
            }
        }
        error(1, 0, dcgettext(NULL, "invalid expression", 5));
    case 2:
        *(a0) = idx->field_108;
        idx->field_118 = get_expr(a0, 4, idx);
        index = *(a0);
        break;
    case 3:
        error(1, 0, dcgettext(NULL, "invalid expression; you have used a binary operator '%s' with nothing before it.", 5));
    case 4:
        v2 = idx->field_108;
        if (!v2 || v2->field_29)
            error(1, 0, dcgettext(NULL, "invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'", 5));
        v3 = v2->field_10;
        *(a0) = v2;
        if ((unsigned int)v3 != 5)
        {
            idx = get_expr(a0, 0, idx);
            v6 = *(a0);
            if (!v6 || v6->field_10 != 5)
                error(1, 0, dcgettext(NULL, "invalid expression; I was expecting to find a ')' somewhere but did not see one.", 5));
            index = v6->field_108;
            *(a0) = index;
            break;
        }
        else if (idx->field_29)
        {
            error(1, 0, dcgettext(NULL, "invalid expression: expected expression before closing parentheses '%s'.", 5));
        }
        else
        {
            error(1, 0, dcgettext(NULL, "invalid expression; empty parentheses are not allowed.", 5));
        }
        break;
    case 5:
        if (a2)
        {
            if (a2->field_10 - 2 > 1)
            {
                if (!idx->field_29)
                    error(1, 0, dcgettext(NULL, "invalid expression; you have too many ')'", 5));
            }
            else
            {
                if (!idx->field_29)
                    error(1, 0, dcgettext(NULL, "expected an expression between '%s' and ')'", 5));
            }
            error(1, 0, dcgettext(NULL, "expected an expression after '%s'", 5));
        }
        else
        {
            error(1, 0, dcgettext(NULL, "invalid expression: expected expression before closing parentheses '%s'.", 5));
        }
    default:
        error(1, 0, dcgettext(NULL, "oops -- invalid expression type!", 5));
    }
}



// Function: set_new_parent @ 0xad30
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[8];
    unsigned int field_20;
    unsigned int field_24;
    char padding_28[240];
    struct struct_0 *field_118;
} struct_1;

typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
} struct_0;

void set_new_parent(struct_0 *a0, unsigned int a1, struct_1 **a2)
{
    struct_1 *idx;  // rax
    int v2;  // xmm0
    unsigned int v3;  // ymm0

    idx = sub_425e50(312);
    idx->field_10 = 3;
    idx->field_14 = a1;
    idx->field_20 = 0;
    if (a1 != 2)
    {
        if (a1 != 3)
        {
            if (a1 != 1)
            {
                idx->field_118 = a0;
                *(a2) = idx;
            }
            else
            {
                *(a2) = idx;
                *((char **)&idx->padding_0[8]) = ",";
                *((void* *)&idx->padding_0[0]) = sub_408ca0;
                idx->field_24 = 0x3f800000;
                idx->field_118 = a0;
            }
            return;
        }
        v2 = (int)a0->field_24;
        *((char **)&idx->padding_0[8]) = "-a";
        *((void* *)&idx->padding_0[0]) = sub_408990;
        if (((CmpF(*((unsigned int *)&v2), 0x3f800000) & 69 | (CmpF(*((unsigned int *)&v2), 0x3f800000) & 69) >> 6) & 1) != 1)
            goto LABEL_40adeb;
LABEL_40ada3:
        v3 = (unsigned int)(MaxV(0, v2));
    }
    else
    {
        v2 = (int)a0->field_24;
        *((char **)&idx->padding_0[8]) = "-o";
        *((void* *)&idx->padding_0[0]) = sub_408940;
        if (((CmpF(*((unsigned int *)&v2), 0x3f800000) & 69 | (CmpF(*((unsigned int *)&v2), 0x3f800000) & 69) >> 6) & 1) == 1)
            goto LABEL_40ada3;
LABEL_40adeb:
        v3 = 0x3f800000;
    }
    idx->field_118 = a0;
    idx->field_24 = v3;
    *(a2) = idx;
    return;
}



// Function: estimate_costs @ 0xae30
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[18];
    char field_1a;
    char field_1b;
    char field_1c;
    char padding_1d[3];
    unsigned int field_20;
    char padding_24[20];
    char field_38;
    char padding_39[215];
    void* field_110;
    void* field_118;
} struct_0;

extern void g_4380e0;
extern void g_438480;
extern unsigned int g_4386b0;

void estimate_costs(struct_0 *idx)
{
    void* iter;  // r14
    unsigned long long v7;  // rbp
    unsigned long long v8;  // r14
    void* v9;  // r15
    unsigned int v10;  // eax
    unsigned int v12;  // eax
    unsigned long long v0;  // [bp-0x70]
    unsigned long v1;  // [bp-0x68]
    unsigned int v2;  // [bp-0x5c]
    unsigned long v3;  // [bp-0x50]
    unsigned long v4;  // [bp-0x48]

    if (!idx)
        return;
    estimate_costs(idx->field_118);
    estimate_costs(idx->field_110);
    v2 = 3;
    if (!idx->field_1a)
    {
        v2 = 1;
        if (!idx->field_1c)
            v2 = idx->field_1b * 2;
    }
    v1 = idx->field_0;
    switch (v1)
    {
    case 4231088: case 4231104:
        idx->field_20 = 8 - idx->field_38;
        return;
    case 4259728:
        v12 = idx->field_20;
        break;
    default:
        if (!g_4386b0)
        {
            iter = &g_4380e0;
            qsort(&g_4380e0, 59, 16, sub_40acd0);
            do
            {
                v3 = (long long)iter[16];
                v4 = *((long long *)iter);
                if (memcmp(&v3, &v4, 8) < 0)
                    error(1, 0, "failed to sort the costlookup array");
                v3 = *((long long *)iter);
                v4 = (long long)iter[16];
                if (memcmp(&v3, &v4, 8) > 0)
                    __assert_fail(); /* do not return */
                iter += 16;
            } while (iter != &g_438480);
            g_4386b0 = 1;
            v1 = idx->field_0;
        }
        v7 = 59;
        v8 = 0;
        do
        {
            v0 = v8 + v7 >> 1;
            v9 = &(&g_4380e0)[16 * v0];
            v3 = v1;
            v4 = *((long long *)v9);
            v10 = memcmp(&v3, &v4, 8);
            if (v10 >= 0)
            {
                if (v10)
                {
                    v8 = v0 + 1;
                    if (v7 > v8)
                        continue;
                }
                else
                {
                    v12 = (int)v9[8];
                    break;
                }
            }
        } while ((v7 = v0, v7 > v8));
        error(0, 0, dcgettext(NULL, "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug", 5));
        v12 = 10;
        break;
    }
    if (v12 <= v2)
        v12 = v2;
    idx->field_20 = v12;
    return;
}



// Function: matches_start_point @ 0xb4b0
extern unsigned long long g_4386b8;
extern unsigned long long g_4386c0;

unsigned int matches_start_point(char *a0, char a1)
{
    unsigned int v1;  // r12d
    unsigned long long v2;  // rbx
    unsigned long long v3;  // rax

    v1 = a1 * 16;
    if (!g_4386b8)
    {
        v3 = fnmatch(a0, ".", v1);
        return _INSERT(v3, 0, !(unsigned int)v3);
    }
    v2 = 0;
    while (fnmatch(a0, *((long long *)(g_4386c0 + v2 * 8)), v1))
    {
        v2 += 1;
        if (g_4386b8 <= v2)
            return 0;
    }
    return 1;
}



// Function: get_eval_tree @ 0xb580
extern unsigned long long g_4386d0;

long long get_eval_tree(void)
{
    return g_4386d0;
}



// Function: calculate_derived_rates @ 0xb590
typedef struct struct_2 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[16];
    unsigned int field_24;
    char padding_28[232];
    struct struct_0 *field_110;
    struct struct_0 *field_118;
} struct_2;

typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
} struct_0;


double calculate_derived_rates(void* idx)
{
    struct_0 *v1;  // r12
    struct_0 *v2;  // rbp
    unsigned int v3;  // eax
    unsigned long long v4;  // rax
    int v5;  // xmm2
    uint128_t v7;  // xmm0
    int v8;  // xmm0
    unsigned long long v9;  // xmm0lq

    if (!idx)
        __assert_fail(); /* do not return */
    v1 = idx->field_118;
    if (v1)
        calculate_derived_rates(v1);
    v2 = idx->field_110;
    if (v2)
        calculate_derived_rates(v2);
    v3 = idx->field_10;
    if (v3 == 5)
    {
        __assert_fail(); /* do not return */
    }
    else if (v3 == 4)
    {
        __assert_fail(); /* do not return */
    }
    else if (v3 != 2)
    {
        if (v3 <= 2)
        {
            if (v3)
            {
                if (!v1)
                {
                    if (v2)
                        __assert_fail(); /* do not return */
                }
                else
                {
                    __assert_fail(); /* do not return */
                }
            }
            else
            {
                if (!v1)
                {
                    if (v2)
                        __assert_fail(); /* do not return */
                }
                else
                {
                    __assert_fail(); /* do not return */
                }
            }
            return (unsigned long long)idx->field_24;
        }
        else if (v3 == 3)
        {
            switch (v4)
            {
            case 4229520:
                if (v1)
                    v8 = (int)v1->field_24;
                else
                    v8 = 0x3f800000;
                if (v2)
                {
                    v8 = MulV(v8, v2->field_24);
                    break;
                }
                break;
            case 4230304:
                idx->field_24 = 0x3f800000;
                return 0x3f800000;
            case 4229440:
                if (v1)
                    v5 = (int)v1->field_24;
                else
                    v5 = 0x3f800000;
                v7 = 0x3f800000;
                if (v2)
                    v7 = v2->field_24;
                v8 = AddV(v7, v5);
                break;
            default:
                __assert_fail(); /* do not return */
            }
            if (((CmpF(*((unsigned int *)&v8), 0x3f800000) & 69 | (char)((CmpF(*((unsigned int *)&v8), 0x3f800000) & 69) >> 6)) & 1) == 1)
            {
                idx->field_24 = *((unsigned int *)&MaxV(0, v8));
                return *((unsigned long long *)&MaxV(0, v8));
            }
            idx->field_24 = 0x3f800000;
            return 0x3f800000;
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    else
    {
        if (idx->field_0 != sub_409350)
        {
            __assert_fail(); /* do not return */
        }
        else if (!v2)
        {
            v9 = SubV(0x3f800000, v1->field_24);
            idx->field_24 = v9;
            return v9;
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
}



// Function: get_new_pred @ 0xb8a0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
    char padding_1c[4];
    unsigned long long field_20;
    char field_28;
    char padding_29[247];
    unsigned long long field_120;
    unsigned long long field_128;
    struct struct_1 *field_130;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

typedef struct struct_4 {
    char padding_0[264];
    struct struct_0 *field_108;
} struct_4;

extern char g_43862f;
extern struct_4 *g_4386c8;
extern unsigned long long g_4386d8;

void get_new_pred(unsigned int *a0)
{
    struct_0 *idx;  // rax

    if (!*(a0))
    {
        __assert_fail(); /* do not return */
    }
    else if (*(a0) != 2)
    {
        idx = sub_425e50(312);
        if (g_4386d8)
            g_4386c8->field_108 = idx;
        else
            g_4386d8 = idx;
        idx->field_130 = a0;
        idx->field_1a = 0x101;
        idx->field_10 = 0;
        *((char **)&idx->padding_29[7]) = "ThisShouldBeSetToSomethingElse";
        idx->field_28 = g_43862f;
        idx->field_20 = 4575657221408423946;
        idx->field_128 = 0;
        idx->field_120 = 0;
        g_4386c8 = idx;
        return;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: get_new_pred_chk_op @ 0xb9b0
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
} struct_0;

typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
    char padding_38[248];
    unsigned int *field_130;
} struct_2;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned int field_18;
    char field_1c;
    char padding_1d[19];
    unsigned long long field_30;
    unsigned long long field_38;
} struct_1;

extern unsigned long long g_4386a0;
extern struct_0 *g_4386c8;

void get_new_pred_chk_op(unsigned int *a0, unsigned long a1)
{
    unsigned long long v1;  // rdi
    struct_1 *idx;  // rax
    struct_2 *index;  // rax

    v1 = g_4386a0;
    if (!v1)
    {
        v1 = sub_414bc0("and");
        g_4386a0 = v1;
        if (!v1)
            __assert_fail(); /* do not return */
    }
    if (g_4386c8)
    {
        if (((int)g_4386c8[2].field_0 & 0xfffffffb) == 1)
        {
            idx = (unsigned long long)sub_40b8a0(v1);
            *((char **)&idx->padding_0[8]) = "-a";
            *((void* *)&idx->padding_0[0]) = sub_408990;
            idx->field_30 = 0;
            idx->field_38 = 0;
            idx->field_10 = 12884901891;
            idx->field_18 = 0;
            idx->field_1c = 0;
        }
        else if (!(int)g_4386c8[2].field_0)
        {
            error(1, 0, dcgettext(NULL, "oops -- invalid default insertion of and!", 5));
        }
    }
    index = (unsigned long long)sub_40b8a0(a0);
    index->field_30 = a1;
    index->field_130 = a0;
    return;
}



// Function: print_tree @ 0xbac0
typedef struct struct_1 {
    char padding_0[16];
    unsigned short field_10;
} struct_1;

typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    unsigned int field_10;
    unsigned int field_14;
    char field_18[8];
    char field_1a;
    char field_1b;
    char field_1c;
    char padding_1d[19];
    char *field_30;
    char padding_38[216];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

extern char g_42962f;
extern struct_1 g_436080;
extern struct_1 g_436100;
extern unsigned int g_438020;

int print_tree(long long a0, struct_0 *a1, unsigned int a2)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    struct_1 *iter;  // rax
    char *v17;  // rcx
    unsigned short v18;  // dx
    int v19;  // ecx
    struct_1 *node;  // rax
    char *v21;  // r8
    unsigned int *v22;  // rdx
    unsigned long long v23;  // rax
    char *v24;  // rcx
    char *v25;  // rcx
    unsigned long long v8;  // r13
    char *v26;  // rcx
    unsigned int j;  // r14d
    unsigned int k;  // r14d
    int v29;  // eax
    char *v30;  // rdx
    int v31;  // eax
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    struct_0 *v11;  // r12
    unsigned int i;  // r14d
    char *v13;  // r8
    unsigned short v14;  // dx
    int v15;  // ecx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!a1)
        return v31;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    while (1)
    {
        v11 = a1;
        i = 0;
        if (a2 > 0)
        {
            do
            {
                __fprintf_chk(a0, 0x1, "    ");
                i += 1;
            } while (i != a2);
        }
        __fprintf_chk(a0, 0x1, "pred=[");
        if (*((long long *)&v11->padding_1d[13]))
            __fprintf_chk(a0, 0x1, "%s %s", v11->field_8, *((long long *)&v11->padding_1d[13]));
        else
            __fprintf_chk(a0, 0x1, "%s", v11->field_8);
        v13 = "no";
        if ((unsigned short)v11->field_14)
        {
            v14 = 1;
            v15 = 0;
            iter = &g_436100.field_0;
            while (1)
            {
                v15 += 1;
                iter = &iter->field_10;
                if ((unsigned short)v11->field_14 == v14 || v14 == 0xffff)
                    break;
                v14 = iter->field_10;
            }
            v13 = (&g_436100.field_0)[1 + 2 * v15];
        }
        v17 = "no";
        if ((unsigned short)v11->field_10)
        {
            v18 = 1;
            v19 = 0;
            node = &g_436080.field_0;
            while (1)
            {
                v19 += 1;
                node = &node->field_10;
                if ((unsigned short)v11->field_10 == v18 || v18 == 0xffff)
                    break;
                v18 = node->field_10;
            }
            v17 = (&g_436080.field_0)[1 + 2 * v19];
        }
        __fprintf_chk(a0, 0x1, "] type=%s prec=%s", v17, v13);
        v21 = &g_42962f;
        v22 = &g_438020;
        if (!v11->field_18[0])
            v21 = "no ";
        v23 = 0;
        do
        {
            if (*((int *)&v11->field_1a) == *(v22))
            {
                v24 = *((long long *)&(&g_438020)[2 + 4 * v23]);
                goto LABEL_40bc5b;
            }
        } while ((v23 = (unsigned long long)((unsigned int)v23 + 1), v22 += 16, (unsigned int)v23 != 11));
        v24 = "unknown";
LABEL_40bc5b:
        __fprintf_chk(a0, 0x1, " cost=%s est_success_rate=%#.4g %sside effects ", v24, v21);
        if (0xffffff0000 & v11->field_18)
        {
            __fprintf_chk(a0, 0x1, "Needs ");
            if (v11->field_18[2])
            {
                __fprintf_chk(a0, 0x1, "stat");
                if (v11->field_18[4])
                {
                    v25 = ",";
                }
                else
                {
                    if (!v11->field_18[3])
                        goto LABEL_40bc89;
LABEL_40bde7:
                    v26 = ",";
                    goto LABEL_40bdee;
                }
LABEL_40bdc5:
                __fprintf_chk(a0, 0x1, "%sinode", v25);
                if (!v11->field_18[3])
                    goto LABEL_40bc89;
                goto LABEL_40bde7;
            }
            else if (v11->field_18[4])
            {
                v25 = &g_42962f;
                goto LABEL_40bdc5;
            }
            else if (v11->field_18[3])
            {
                v26 = &g_42962f;
LABEL_40bdee:
                __fprintf_chk(a0, 0x1, "%stype", v26);
            }
        }
LABEL_40bc89:
        j = 0;
        __fprintf_chk(a0, 0x1, "\n");
        if (a2 > 0)
        {
            do
            {
                __fprintf_chk(a0, 0x1, "    ");
                j += 1;
            } while (j != a2);
        }
        if (*((long long *)&v11->padding_38[208]))
        {
            __fprintf_chk(a0, 0x1, "left:\n");
            print_tree(a0, *((long long *)&v11->padding_38[208]), a2 + 1);
        }
        else if (v11->field_110)
        {
            __fprintf_chk(a0, 0x1, "no left.\n");
        }
        else
        {
            v30 = "no children.\n";
            break;
        }
        k = 0;
        if (a2 > 0)
        {
            do
            {
                __fprintf_chk(a0, 0x1, "    ");
                k += 1;
            } while (k != a2);
        }
        if (v11->field_110)
        {
            a2 += 1;
            v29 = __fprintf_chk(a0, 0x1, "right:\n");
            a1 = v11->field_110;
            if (!v11->field_110)
                return v29;
        }
        else
        {
            v30 = "no right.\n";
            break;
        }
    }
    return __fprintf_chk(a0, 0x1, v30);
}



// Function: predlist_merge_sort @ 0xbef0
typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[272];
    struct struct_0 *field_110;
    struct struct_1 *field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[32];
    unsigned int field_20;
    unsigned int field_24;
} struct_1;

extern long long stderr;
extern char g_438660;

struct_0 * predlist_merge_sort(struct_2 *a0, struct_0 **a1)
{
    struct_0 *node;  // r14
    struct_0 *v2;  // rbp
    struct_0 *v11;  // rax
    struct_0 *v3;  // r12
    struct_0 *index;  // r8
    struct_0 *iter;  // rax
    unsigned long v6;  // rdx
    unsigned int v7;  // ymm0
    unsigned int v8;  // ymm1
    struct_0 *v9;  // rdx
    struct_0 *v10;  // rax

    node = a0->field_0;
    if (!node)
        return v11;
    if (!(g_438660 & 8))
    {
        sub_40b590(node);
LABEL_40bf23:
        v2 = NULL;
        v3 = NULL;
        while (1)
        {
            index = node;
            node = node->field_110;
            a0->field_0 = node;
            index->field_110 = NULL;
            if (v2)
            {
                iter = v2;
                while (1)
                {
                    v6 = iter->field_118;
                    if (*((int *)(v6 + 32)) != index->field_118->field_20)
                        break;
                    v7 = *((int *)(v6 + 36));
                    v8 = index->field_118->field_24;
                    if (!((char)((CmpF(v7, v8) & 69) >> 2) & 1) && CmpF(v7, v8) & 64)
                        goto LABEL_40bf50;
                    if (*((int *)&iter->padding_0[20]) != 2)
                    {
                        if (((CmpF(v8, v7) & 69 | (CmpF(v8, v7) & 69) >> 6) & 1) != 1)
                            goto LABEL_40bfb1;
                        goto LABEL_40bf50;
                    }
                    if (((CmpF(v8, v7) & 69 | (CmpF(v8, v7) & 69) >> 6) & 1) != 1)
                        goto LABEL_40bf50;
LABEL_40bfb1:
                    iter = iter->field_110;
                    if (!iter)
                        goto LABEL_40bfbd;
                }
                if (*((int *)(v6 + 32)) < index->field_118->field_20)
                    goto LABEL_40bfb1;
LABEL_40bf50:
                v9 = iter->field_110;
                index->field_110 = v9;
                if (!v9)
                    v3 = index;
                iter->field_110 = index;
                if (!node)
                    break;
                else
                    continue;
            }
LABEL_40bfbd:
            index->field_110 = v2;
            v2 = index;
            if (!v3)
                v3 = index;
            if (!node)
                break;
        }
    }
    else
    {
        __fprintf_chk(stderr, 0x1, "%s:\n", "predlist before merge sort");
        sub_40bac0(stderr, a0->field_0, 2);
        node = a0->field_0;
        sub_40b590(node);
        if (node)
            goto LABEL_40bf23;
        v2 = NULL;
        v3 = NULL;
    }
    if (g_438660 & 8)
    {
        __fprintf_chk(stderr, 0x1, "%s:\n", "predlist after merge sort");
        sub_40bac0(stderr, v2, 2);
    }
    sub_40b590(v2);
    v10 = *(a1);
    v3->field_110 = v10;
    *(a1) = v2;
    a0->field_8 = 0;
    a0->field_0 = NULL;
    return v10;
}



// Function: opt_expr @ 0xc110
typedef struct struct_0 {
    unsigned long long field_0;
    char *field_8;
    unsigned int field_10;
    char padding_14[4];
    char field_18;
    char padding_19[1];
    char field_1a;
    char padding_1b[5];
    unsigned int field_20;
    char padding_24[12];
    char *field_30;
    char padding_38[224];
    char field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[248];
    struct struct_1 *field_110;
    struct struct_0 *field_118;
} struct_1;

extern unsigned long long g_438020;
extern long long stderr;
extern char g_438660;
extern unsigned short g_438688;

unsigned int opt_expr(struct_1 **a0)
{
    struct_1 *v9;  // r9
    struct_1 **v10;  // r15
    struct_0 *index;  // r14
    unsigned int v20;  // eax
    unsigned long v21;  // rcx
    unsigned long v22;  // rax
    unsigned long long v23;  // rdx
    unsigned long long v24;  // rax
    char *v25;  // rcx
    unsigned long long v26;  // rax
    struct_1 **v27;  // rdx
    struct_1 *v11;  // rcx
    void* node;  // rax
    struct_1 *iter;  // rdi
    unsigned int v14;  // esi
    struct_1 **v15;  // rdx
    struct_1 *idx;  // r13
    unsigned int k;  // ebx
    unsigned int v18;  // r12d
    struct_1 *v0;  // [bp-0x118]
    struct_1 *v1;  // [bp-0x118]
    struct_1 *v2;  // [bp-0x110]
    struct_1 *v3;  // [bp-0x108]
    struct_1 *v4;  // [bp-0x108]
    struct_1 *v5;  // [bp-0x100]
    char v6;  // [bp-0xf8]
    char v7;  // [bp-0x48]

    v9 = *(a0);
    v0 = NULL;
    v2 = NULL;
    v3 = NULL;
    v5 = NULL;
    if (!v9)
        return 0;
    v10 = a0;
    v11 = v9;
    node = &v6;
    do
    {
        *((unsigned long long *)&node[8]) = 0;
        node += 16;
        *((unsigned long long *)((char *)node - 16)) = 0;
    } while (node != &v7);
    if (v9->field_110)
    {
        for (iter = v9->field_110; iter->field_110; iter = iter->field_110)
        {
            v11 = iter;
        }
        v14 = v11->field_14;
        v15 = &v11->field_110;
    }
    else
    {
        v15 = v10;
        iter = v9;
        v14 = 3;
    }
    if (iter->field_10 == 3)
    {
        idx = v9;
        if (!(g_438660 & 9))
            goto LABEL_40c1e6;
    }
    else
    {
        sub_40ad30(iter, v14, v15);
        if (!(g_438660 & 9))
            goto LABEL_40c34a;
    }
    __fprintf_chk(stderr, 0x1, "Normalized Eval Tree:\n");
    sub_40bac0(stderr, *(v10), 0);
LABEL_40c34a:
    idx = *(v10);
    if (!idx)
    {
        sub_40c0b0(&v6, 11, &v3, &v0, v10);
        return 0;
    }
LABEL_40c1e6:
    k = 0;
    if (idx->field_10 == 3)
        k = idx->field_14;
    v18 = 0;
    if (idx->field_10 == 3)
        goto LABEL_40c405;
    while (1)
    {
        do
        {
            do
            {
                index = idx->field_118;
                v20 = index->field_10;
                v21 = index->field_0;
                if (v20 == 2)
                {
                    index->field_18 = opt_expr(&index->field_118);
                    index = idx->field_118;
                }
                else if (v20 <= 2)
                {
                    if (k != 1 && !index->field_18)
                    {
                        if (!(v21 != sub_409310 && v21 != sub_409640 && v21 != sub_409150 && v21 != sub_4091e0 && (v21 == sub_409350 || v21 == sub_408990 || !g_438688 || v21 == sub_408ca0 || v21 == sub_408940 || index->field_20)))
                        {
                            if (g_438660 & 8)
                            {
                                __fprintf_chk(stderr, 0x1, "-O%d: promoting cheap predicate ", g_438688);
                                if (idx->field_118->field_30)
                                    __fprintf_chk(stderr, 0x1, "%s %s", idx->field_118->field_8, idx->field_118->field_30);
                                else
                                    __fprintf_chk(stderr, 0x1, "%s", idx->field_118->field_8);
                                __fprintf_chk(stderr, 0x1, " into name_list\n");
                            }
                            *(v10) = idx->field_110;
                            v4 = idx;
                            idx->field_110 = v3;
                            v3 = v4;
                            if (!v5)
                            {
                                v5 = idx;
                                v3 = v4;
                                goto LABEL_40c3ec;
                            }
                        }
                        else if (v21 == sub_4097e0)
                        {
                            *(v10) = idx->field_110;
                            v1 = idx;
                            idx->field_110 = v0;
                            v0 = v1;
                            if (!v2)
                            {
                                v2 = idx;
                                v0 = v1;
                                goto LABEL_40c3ec;
                            }
                        }
                        else if (g_438688 > 1 && !(v22 = (unsigned long)(unsigned long long)index->field_20, ((unsigned int)(v22 - 1) > 1 || index->field_1a) && g_438688 == 2))
                        {
                            if (g_438660 & 8)
                            {
                                __fprintf_chk(stderr, 0x1, "-O%d: categorising predicate ", g_438688);
                                if (idx->field_118->field_30)
                                    __fprintf_chk(stderr, 0x1, "%s %s", idx->field_118->field_8, idx->field_118->field_30);
                                else
                                    __fprintf_chk(stderr, 0x1, "%s", idx->field_118->field_8);
                                v23 = &g_438020;
                                v24 = 0;
                                do
                                {
                                    if (idx->field_118->field_20 == *((int *)v23))
                                    {
                                        v25 = (&g_438020)[1 + 2 * v24];
                                        goto LABEL_40c617;
                                    }
                                } while ((v24 = (unsigned long long)((unsigned int)v24 + 1), v23 += 16, (unsigned int)v24 != 11));
                                v25 = "unknown";
LABEL_40c617:
                                __fprintf_chk(stderr, 0x1, " by cost (%s)\n", v25);
                                v22 = idx->field_118->field_20;
                            }
                            v26 = v22 * 16;
                            v27 = &(&v6)[v26];
                            *(v10) = idx->field_110;
                            idx->field_110 = *(v27);
                            *(v27) = idx;
                            if (!*((long long *)(&(&v6)[v26] - 8)))
                            {
                                *((struct_1 **)(&(&v6)[v26] - 8)) = idx;
                                goto LABEL_40c3ec;
                            }
                        }
                    }
                }
                else
                {
                    if (v20 == 3)
                    {
                        index->field_18 = opt_expr(&idx->field_118);
                        index = idx->field_118;
                    }
                    else
                    {
                        error(1, 0, dcgettext(NULL, "oops -- invalid expression type!", 5));
                    }
                }
                if (!index->field_18)
                    goto LABEL_40c3e5;
                v18 = 1;
                sub_40c0b0(&v6, 11, &v3, &v0, v10);
LABEL_40c3e5:
                v10 = &idx->field_110;
LABEL_40c3ec:
                idx = *(v10);
                if (!idx)
                {
                    sub_40c0b0(&v6, 11, &v3, &v0, v10);
                    return v18;
                }
            } while (idx->field_10 != 3);
LABEL_40c405:
        } while (idx->field_14 == k);
        idx = (unsigned long long)sub_40ad30(idx, k, v10);
    }
}



// Function: do_arm_swaps @ 0xc690
typedef struct struct_2 {
    char padding_0[24];
    char field_18;
    char padding_19[247];
    unsigned long long field_110;
} struct_2;

typedef struct struct_4 {
    char padding_0[280];
    struct struct_2 *field_118;
} struct_4;

typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    unsigned int field_10;
    unsigned int field_14;
    char field_18[8];
    char field_1a;
    char field_1b;
    char field_1c;
    char padding_1d[19];
    char *field_30;
    char padding_38[216];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
} struct_1;

extern long long stderr;
extern unsigned long long g_438660;

unsigned int do_arm_swaps(struct_0 *index)
{
    unsigned long long v8;  // r15
    unsigned long long v9;  // r14
    struct_0 *idx;  // r10
    struct_1 *idx1;  // r11
    unsigned long long v20;  // r14
    unsigned long long v21;  // r15
    long long v22;  // rdx
    unsigned int v23;  // eax
    long long v24;  // rsi
    unsigned int v25;  // r9d
    unsigned int v26;  // r9d
    unsigned int v27;  // eax
    unsigned long long v10;  // r13
    unsigned int v28;  // esi
    unsigned int v29;  // r8d
    unsigned int v30;  // ymm2
    unsigned int v31;  // ymm3
    unsigned long v32;  // rax
    unsigned int v33;  // r8d
    unsigned long v34;  // 4131
    struct_2 *v35;  // rcx
    unsigned long long v11;  // r12
    unsigned long long v12;  // rbx
    unsigned int v13;  // r8d
    char *v14;  // rcx
    unsigned long long v15;  // rdx
    struct_4 *idx2;  // rax
    struct_2 *v17;  // r10
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]
    unsigned long long v2;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    unsigned long long v5;  // [bp-0x10]
    unsigned long long v6;  // [bp-0x8]

    if (!index)
        return 0;
    v6 = v8;
    v5 = v9;
    v4 = v10;
    v3 = v11;
    v2 = v12;
    do
    {
        do
        {
            v13 = g_438660;
            v14 = "Not a binary operation";
            v15 = (unsigned int)g_438660 & 8;
            if (index->field_10 == 3)
            {
                idx2 = *((long long *)&index->padding_38[208]);
                do
                {
                    if (!idx2 || !index->field_110)
                    {
                        v14 = "Doesn't have two arms";
                        v15 = v13 & 8;
                        goto LABEL_40c6d8;
                    }
                    v17 = idx2->field_118;
                    if (!v17)
                    {
                        v14 = "Left arm has no child on RHS";
                        v15 = v13 & 8;
                        goto LABEL_40c6d8;
                    }
                    if (v17->field_18 || (char)sub_40b430(v17->field_110) || (char)sub_40b430(idx->field_110, idx->field_110))
                    {
                        v14 = "Left subtree has side-effects";
                        v15 = v29 & 8;
                        goto LABEL_40c6d8;
                    }
                    if (idx1->padding_0[24] || (v20 = (unsigned long long)*((long long *)&idx1[6].padding_0[32]), (char)(unsigned long long)sub_40b430(v20) || (v21 = (unsigned long long)*((long long *)&idx1[7].padding_0[0]), (char)(unsigned long long)sub_40b430(v21))))
                    {
                        v14 = "Right subtree has side-effects";
                        v15 = v29 & 8;
                        goto LABEL_40c6d8;
                    }
                    if (v22)
                        sub_40b0c0(v22);
                    v23 = 0;
                    if (v24)
                        v23 = sub_40b0c0(v24);
                    if (v25 < *((int *)&idx->field_1a))
                        v26 = *((int *)&idx->field_1a);
                    else
                        v26 = v25;
                    if (v20)
                        sub_40b0c0(v20);
                    v27 = 0;
                    if (v21)
                        v27 = sub_40b0c0(v21);
                    if ((unsigned int)v24 < *((int *)&idx1->padding_0[32]))
                        v28 = *((int *)&idx1->padding_0[32]);
                    else
                        v28 = v24;
                    if (v28 < v27)
                        v28 = v27;
                    v15 = v29 & 8;
                    if (v25 < v28)
                    {
                        v14 = "efficient as-is";
                        goto LABEL_40c6d8;
                    }
                    v13 = v29;
                    if (v25 != v28)
                        continue;
                    v30 = *((int *)&idx->padding_1d[1]);
                    v31 = idx1->field_24;
                    if (!v15)
                    {
                        v32 = (unsigned long)index->padding_0;
                        v13 = v29;
                        v33 = v29;
                        if (v32 == sub_408940)
                            goto LABEL_40c9be;
                        goto LABEL_40c901;
                    }
                    else
                    {
                        v1 = v30;
                        v0 = v31;
                        __fprintf_chk(stderr, 0x1, "Success rates: l=%f, r=%f\n");
                        v13 = g_438660;
                        v32 = (unsigned long)index->padding_0;
                        v30 = v1;
                        v31 = v0;
                        v15 = (unsigned int)g_438660 & 8;
                        v33 = v13;
                        if (v32 != sub_408940)
                        {
LABEL_40c901:
                            if (v32 != sub_408990)
                            {
                                v14 = "Not 'AND' or 'OR'";
                                goto LABEL_40c6d8;
                            }
                            else
                            {
                                if (((CmpF(v31, v30) & 69 | (CmpF(v31, v30) & 69) >> 6) & 1) != 1)
                                    continue;
                                v14 = "Operation is AND; right success rate <= left";
                                goto LABEL_40c6d8;
                            }
                        }
                        else
                        {
LABEL_40c9be:
                            v13 = v33;
                            if (((CmpF(v30, v31) & 69 | (char)((CmpF(v30, v31) & 69) >> 6)) & 1) == 1)
                            {
                                v14 = "Operation is OR; right success rate >= left";
                                goto LABEL_40c6d8;
                            }
                        }
                    }
                    if (v15)
                    {
                        __fprintf_chk(stderr, 0x1, "Performing arm swap on:\n");
                        sub_40bac0(stderr, index, 0);
                        v13 = g_438660;
                        v15 = (unsigned int)g_438660 & 8;
                    }
                    idx2 = *((long long *)&index->padding_38[208]);
                    v34 = index->field_10;
                    v35 = idx2->field_118;
                    idx2->field_118 = index->field_110;
                    index->field_110 = v35;
                } while ((unsigned int)v34 == 3);
                v14 = "Not a binary operation";
            }
LABEL_40c6d8:
            if (v15)
            {
                __fprintf_chk(stderr, 0x1, "Not an arm swap candidate (%s):\n", v14);
                sub_40bac0(stderr, index, 0);
            }
        } while ((char)do_arm_swaps(*((long long *)&index->padding_38[208])));
    } while ((char)do_arm_swaps(index->field_110));
    return 0;
}



// Function: build_expression_tree @ 0xc9f0
typedef struct struct_4 {
    char padding_0[16];
    unsigned int field_10;
} struct_4;

typedef struct struct_3 {
    char padding_0[16];
    struct struct_4 *field_10;
} struct_3;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[25];
    char field_29;
    char padding_2a[222];
    struct struct_0 *field_108;
} struct_0;

typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
} struct_2;

extern long long stderr;
extern char g_438618;
extern char g_438660;
extern unsigned long long g_4386b8;
extern unsigned long long g_4386c0;
extern struct_2 *g_4386c8;
extern void g_4386d0;
extern struct_0 *g_4386d8;

long long build_expression_tree(unsigned int a0, long long a1, unsigned int a2)
{
    unsigned long i;  // rax
    long long v8;  // r12
    unsigned long v22;  // rax
    long long v9;  // r13
    unsigned long idx;  // rax
    unsigned long v11;  // rdi
    struct_3 *v12;  // r14
    unsigned int v13;  // r15d
    unsigned long long v14;  // rax
    unsigned long v15;  // rdx
    long long v0;  // [bp-0x70]
    char *v1;  // [bp-0x68]
    unsigned int j;  // [bp-0x5c]
    unsigned int v3;  // [bp-0x4c]
    unsigned int v4;  // [bp-0x4c]
    struct_0 *ptr;  // [bp-0x48]

    j = a0;
    i = a2;
    v3 = a2;
    g_4386d8 = 0;
    if (a2 < a0)
    {
        for (g_4386c0 = a1 + i * 8; !(char)sub_40e120(*((long long *)(a1 + i * 8)), 1) && (g_4386b8 = g_4386b8 + 1, v3 += 1, v3 < j); i = v3);
    }
    v8 = sub_414bc0("(");
    v0 = sub_414bc0(")");
    v9 = sub_414bc0("print");
    if (!v8)
    {
        __assert_fail(); /* do not return */
    }
    else if (!v0)
    {
        __assert_fail(); /* do not return */
    }
    else if (v9)
    {
        v1 = &j;
        sub_412c60(v8, a1, &j);
        *((char **)&g_4386c8->padding_0[8]) = "(";
        *((char *)&g_4386d8[5].field_0 + 1) = 1;
        sub_414ba0(a1, j, g_4386c8, g_4386d8);
        sub_409e60(g_4386c8);
        idx = v3;
        if ((unsigned int)idx < j)
        {
            do
            {
                v11 = *((long long *)(a1 + idx * 8));
                g_438618 = 0;
                if (!(char)sub_40e120(v11, 0))
                {
                    error(0, 0, dcgettext(NULL, "paths must precede expression: `%s'", 5));
                    if (!access(*((long long *)(a1 + v3 * 8)), 0))
                        error(0, 0, dcgettext(NULL, "possible unquoted pattern after predicate `%s'?", 5));
                    exit(1); /* do not return */
                }
                v12 = sub_414bc0(*((long long *)(a1 + v3 * 8)));
                if (!v12)
                    error(1, 0, dcgettext(NULL, "unknown predicate `%s'", 5));
                v13 = v3;
                v4 = v3;
                if (*((int *)&v12->padding_0[0]) != 4)
                {
                    v13 += 1;
                    v4 = v13;
                }
                if (!(char)v12->field_10(v12, a1, &v4))
                {
                    v15 = v4;
                    if (!*((long long *)(a1 + v15 * 8)))
                        error(1, 0, dcgettext(NULL, "missing argument to `%s'", 5));
                    if (*((int *)&v12->padding_0[0]) == 4 && (unsigned int)v15 == v13)
                        error(1, 0, dcgettext(NULL, "invalid predicate `%s'", 5));
                    error(1, 0, dcgettext(NULL, "invalid argument `%s' to `%s'", 5));
                }
                v14 = 0;
                *((long long *)&g_4386c8->padding_0[8]) = *((long long *)(a1 + v3 * 8));
                if (v4 != v13)
                    v14 = *((long long *)(a1 + v13 * 8));
                g_4386c8->field_30 = v14;
                sub_409e60(g_4386c8);
                sub_409e60(g_4386d8);
                idx = v4;
                v3 = v4;
            } while ((unsigned int)idx < j);
        }
        sub_414bb0(a1);
        if (!g_4386d8[33].field_0)
        {
            ptr = g_4386d8;
            g_4386c8 = 0;
            g_4386d8 = 0;
            free(ptr);
            sub_411e20(v9, a1, v1);
            *((char **)&g_4386c8->padding_0[8]) = "-print";
            sub_409e60(g_4386c8);
            sub_409e60(g_4386d8);
            goto LABEL_40ccb7;
        }
        else if ((char)sub_40e0e0(ptr->field_108))
        {
            sub_412a60(v0, a1, v1);
            *((char **)&g_4386c8->padding_0[8]) = ")";
            g_4386c8->padding_0[41] = 1;
            sub_409e60(g_4386c8);
            sub_411e20(v9, a1, v1);
            *((char **)&g_4386c8->padding_0[8]) = "-print";
            g_4386c8->padding_0[41] = 1;
            sub_409e60(g_4386c8);
            sub_409e60(g_4386d8);
LABEL_40ccb7:
            if (g_438660 & 9)
                goto LABEL_40cd90;
        }
        else
        {
            ptr = g_4386d8;
            g_4386d8 = g_4386d8[33].field_0;
            sub_409e60(g_4386d8);
            free(ptr);
            if (g_438660 & 9)
            {
LABEL_40cd90:
                __fprintf_chk(stderr, 0x1, "Predicate List:\n");
                sub_409cd0(stderr, g_4386d8);
            }
        }
        sub_414a70(g_4386d8);
        sub_409e60(g_4386d8);
        ptr = g_4386d8;
        v22 = sub_40a8c0(&ptr, 0, NULL);
        *((unsigned long *)&g_4386d0) = v22;
        sub_40b590(v22);
        if (!ptr)
        {
            if (g_438660 & 9)
            {
                __fprintf_chk(stderr, 0x1, "Eval Tree:\n");
                sub_40bac0(stderr, *((long long *)&g_4386d0), 0);
                v22 = *((long long *)&g_4386d0);
            }
            sub_40ae30(v22);
            sub_40c110(&g_4386d0);
            sub_40a840(*((long long *)&g_4386d0), 1);
            sub_40c690(*((long long *)&g_4386d0));
            sub_40a840(*((long long *)&g_4386d0), 1);
            if (g_438660 & 9)
            {
                __fprintf_chk(stderr, 0x1, "Optimized Eval Tree:\n");
                sub_40bac0(stderr, *((long long *)&g_4386d0), 0);
                __fprintf_chk(stderr, 0x1, "Optimized command line:\n");
                sub_409de0(stderr, *((long long *)&g_4386d0));
                __fprintf_chk(stderr, 0x1, "\n");
                return *((long long *)&g_4386d0);
            }
            return *((long long *)&g_4386d0);
        }
        else if (ptr->field_0 == sub_408be0)
        {
            error(1, 0, dcgettext(NULL, "you have too many ')'", 5));
        }
        else if (ptr->field_8)
        {
            error(1, 0, dcgettext(NULL, "unexpected extra predicate '%s'", 5));
        }
        else
        {
            error(1, 0, dcgettext(NULL, "unexpected extra predicate", 5));
        }
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: do_complete_pending_execdirs @ 0xd130
extern char g_4385c0;
extern char g_438608;

long long do_complete_pending_execdirs(void* a0)
{
    unsigned long long v2;  // rbx
    void* v3;  // rbx
    void* idx;  // rbx
    unsigned long v5;  // rax
    unsigned long v6;  // rax
    unsigned long v7;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (!a0)
        return v7;
    v0 = v2;
    v3 = a0;
    while (1)
    {
        idx = v3;
        if (!g_438608)
            __assert_fail(); /* do not return */
        do_complete_pending_execdirs((long long)idx[272]);
        v5 = *((long long *)idx);
        if (v5 != sub_408fc0 && v5 != sub_4095d0 || !(char)idx[56] || (v5 = (unsigned long)(unsigned long long)(unsigned int)(int)idx[208], !(unsigned int)v5 || (v5 = (unsigned long)(unsigned long long)sub_4159f0((long long)idx[248], sub_40d220, idx + 56), (long long)idx[248] == *((long long *)&g_4385c0))))
        {
            v3 = (long long)idx[280];
            if (!v3)
                return v5;
        }
        else
        {
            sub_4242c0((long long)idx[248]);
            v6 = (unsigned long long)free((long long)idx[248]);
            *((unsigned long *)&idx[248]) = 0;
            v3 = (long long)idx[280];
            if (!(long long)idx[280])
                return v6;
        }
    }
}



// Function: exec_cb @ 0xd220
unsigned int exec_cb(long long a0)
{
    sub_415430(a0 + 8, a0 + 96);
    return 0;
}



// Function: complete_pending_execs @ 0xd240
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[48];
    char field_38;
    char padding_39[151];
    unsigned int field_d0;
    char padding_d4[60];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

unsigned long long complete_pending_execs(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    struct_0 *v3;  // rbx
    unsigned long long v4;  // rax
    unsigned long long v5;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (!a0)
        return v5;
    v0 = v2;
    do
    {
        v3 = a0;
        v4 = complete_pending_execs(v3->field_110);
        if (v3->field_0 == sub_408fb0 && v3->field_38 && v3->field_d0)
            v4 = sub_415430(&v3->padding_39[7], &v3->padding_39[95]);
    } while ((a0 = (struct_0 *)v3->field_118, v3->field_118));
    return v4;
}



// Function: insert_primary_noarg @ 0xd520
void insert_primary_noarg(unsigned long long *index)
{
    unsigned long long *idx;  // rax
    unsigned long long v2;  // rdx

    if (!index[3])
        sub_40d2b0(); /* do not return */
    idx = (unsigned long long)sub_40b9b0(index, 0);
    v2 = index[1];
    *(idx) = index[3];
    idx[1] = v2;
    idx[7] = 0;
    idx[2] = 1;
    return;
}



// Function: usage @ 0xd570
extern unsigned long long stderr;
extern void stdout;
extern long long g_438ed0;

void usage(int a0)
{
    unsigned long long v5;  // r14
    unsigned long long v6;  // r13
    char *v15;  // rax
    char *v16;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    unsigned long long v21;  // rax
    unsigned long long v22;  // rbp
    char *v23;  // rax
    unsigned long long v7;  // r12
    unsigned long long v8;  // rbx
    char *v9;  // rax
    char *v10;  // rax
    char *v11;  // rax
    char *v12;  // rax
    char *v13;  // rax
    char *v14;  // rax
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]
    unsigned long long v3;  // [bp-0x8]

    v3 = v5;
    v2 = v6;
    v1 = v7;
    v0 = v8;
    if (a0)
    {
        v9 = dcgettext(NULL, "Try '%s --help' for more information.\n", 5);
        __fprintf_chk(stderr, 0x1, v9);
        exit(a0); /* do not return */
    }
    v10 = dcgettext(NULL, "Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n", 5);
    __fprintf_chk(*((long long *)&stdout), 0x1, v10);
    v11 = dcgettext(NULL, "\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n", 5);
    fputs(v11, *((long long *)&stdout));
    v12 = dcgettext(NULL, "\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n", 5);
    fputs(v12, *((long long *)&stdout));
    v13 = dcgettext(NULL, "\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n", 5);
    fputs(v13, *((long long *)&stdout));
    v14 = dcgettext(NULL, "\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n", 5);
    fputs(v14, *((long long *)&stdout));
    v15 = dcgettext(NULL, "\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n", 5);
    fputs(v15, *((long long *)&stdout));
    v16 = dcgettext(NULL, "\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n", 5);
    fputs(v16, *((long long *)&stdout));
    v17 = dcgettext(NULL, "\nOther common options:\n", 5);
    fputs(v17, *((long long *)&stdout));
    v18 = dcgettext(NULL, "      --help                   display this help and exit\n", 5);
    fputs(v18, *((long long *)&stdout));
    v19 = dcgettext(NULL, "      --version                output version information and exit\n\n", 5);
    fputs(v19, *((long long *)&stdout));
    v20 = dcgettext(NULL, "Valid arguments for -D:\n", 5);
    fputs(v20, *((long long *)&stdout));
    v21 = 0;
    while (1)
    {
        v22 = v21 + 1;
        if (v21)
        {
            __fprintf_chk(*((long long *)&stdout), 0x1, "%s%s");
            if (v22 == 9)
                break;
        }
        else
        {
            __fprintf_chk(*((long long *)&stdout), 0x1, "%s%s");
        }
        v21 = v22;
    }
    v23 = dcgettext(NULL, "\nUse '-D help' for a description of the options, or see find(1)\n\n", 5);
    fputs(v23, *((long long *)&stdout));
    sub_4175f0(*((long long *)&stdout), g_438ed0);
    exit(0); /* do not return */
}



// Function: get_statinfo @ 0xd820
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern void g_4385e4;
extern unsigned int g_4385e8;
extern unsigned int g_438604;
extern char g_438618;
extern char g_43862e;
extern struct_1 *g_438670;
extern unsigned int g_43868c;

unsigned int get_statinfo(long long a0, long long a1, struct_0 *idx)
{
    unsigned int v1;  // eax
    unsigned int v3;  // r12d

    if (g_4385e4)
    {
        v1 = idx->field_18;
    }
    else if (!g_438670(a1, idx))
    {
        v1 = idx->field_18;
        if (!v1)
        {
            sub_4239f0(0, g_43868c, a1);
            error(0, 0, dcgettext(NULL, "WARNING: file %s appears to have mode 0000", 5));
            v1 = idx->field_18;
            g_438604 = 1;
        }
    }
    else
    {
        v3 = *(__errno_location());
        if (g_43862e && *(__errno_location()) == 2)
            return 0xffffffff;
        if (!g_438618)
        {
            sub_4239f0(0, g_43868c, a0);
            error(0, *(__errno_location()), "%s");
            g_438604 = 1;
        }
        g_438618 = 1;
        return 0xffffffff;
    }
    g_4385e8 = v1;
    *((unsigned short *)&g_4385e4) = 0x101;
    return 0;
}



// Function: check_nofollow @ 0xd920
unsigned int check_nofollow(void)
{
    unsigned int v3;  // ebx
    unsigned long long v4;  // rax
    unsigned long long v5;  // rax
    unsigned long v6;  // cc_ndep
    unsigned long long v7;  // 4114
    unsigned long v8;  // cc_ndep
    unsigned long long v9;  // 4114
    char v0[6][1024];  // [bp-0x1a8]
    char v1[262];  // [bp-0x126]

    if (!uname(v0))
    {
        v3 = (int)strtod(v1, NULL);
        v4 = strcmp("Linux", v0);
        if (!(unsigned int)v4)
        {
            v9 = _ccall(3, 0, (unsigned long long)(CmpF((unsigned long long)v3, 1074580685) & 69), 0, v8);
            return _INSERT(v4, 0, v9 & 1);
        }
        v5 = strcmp("FreeBSD", v0);
        if (!(unsigned int)v5)
        {
            v7 = _ccall(3, 0, (unsigned long long)(CmpF((unsigned long long)v3, 1078355558) & 69), 0, v6);
            return _INSERT(v5, 0, v7 & 1);
        }
    }
    return 1;
}



// Function: record_initial_cwd @ 0xda20
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_4385c0;

void record_initial_cwd(void)
{
    char *v1;  // rax
    int *err;  // rax

    g_4385c0 = sub_425a50(16);
    if (!sub_424230(g_4385c0))
        return;
    v1 = dcgettext(NULL, "Failed to save initial working directory%s%s", 5);
    err = __errno_location();
    error(1, *(err), v1);
}



// Function: cleanup @ 0xdab0
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[56];
    unsigned long long field_40;
    char padding_48[200];
    long long field_110;
    struct struct_0 *field_118;
} struct_0;

extern long long stdout;
extern void g_4385c0;
extern unsigned int g_438604;
extern char g_438608;
extern long long g_438610;
extern unsigned int g_43868c;

void cleanup(void)
{
    struct_0 *v1;  // rax
    struct_0 *iter;  // rbx
    long long v11;  // r9
    int *err;  // rax
    char *v13;  // rax
    int *err1;  // rax
    struct_0 *v3;  // rbp
    long long v6;  // rdi
    long long v7;  // rsi
    long long v8;  // rdx
    long long v9;  // rcx
    long long v10;  // r8

    v1 = sub_40b580();
    if (v1)
    {
        iter = v1;
        do
        {
            v3 = v1;
            if (v3->field_110)
                sub_40d2e0(v3->field_110, sub_40d240);
        } while ((sub_40d240(v3), v1 = (struct_0 *)v3->field_118, v3->field_118));
        if (g_438608)
        {
            sub_40d130(sub_40b580());
            g_438608 = 0;
        }
        sub_40eb20(g_438610);
        do
        {
            if (iter->field_110)
                sub_40d2e0(iter->field_110, sub_40d0f0);
            if (iter->field_0 == sub_40ff90 || iter->field_0 == sub_409020 || iter->field_0 == sub_408fe0 || iter->field_0 == sub_409060)
                iter->field_40 = 0;
            iter = iter->field_118;
        } while (iter);
    }
    else
    {
        sub_40eb20(g_438610);
    }
    if (sub_424290(*((long long *)&g_4385c0)))
    {
        v13 = dcgettext(NULL, "Failed to restore initial working directory%s%s", 5);
        err1 = __errno_location();
        error(0, *(err1), v13);
        _exit(1); /* do not return */
    }
    sub_4242c0(*((long long *)&g_4385c0));
    free(*((long long *)&g_4385c0));
    *((unsigned long *)&g_4385c0) = 0;
    if ((char)sub_416220(v6, v7, v8, v9, v10, v11))
    {
        sub_416240();
        sub_4161f0(v6, v7, v8, v9, v10, v11);
    }
    if (sub_4180c0(stdout) != 0xffffffff)
        return;
    err = __errno_location();
    sub_4239f0(0, g_43868c, "standard output");
    error(0, *(err), "%s");
    g_438604 = 1;
    return;
}



// Function: debug_stat @ 0xdec0
extern unsigned long long stderr;
extern unsigned int g_4385e0;
extern void g_4385f8;
extern char g_438660;
extern unsigned int g_438668;
extern unsigned long long g_4386e0;

long long debug_stat(char *a0, long long a1)
{
    unsigned long long v6;  // r12
    unsigned long long v7;  // r13
    unsigned int v8;  // eax
    unsigned int v9;  // eax
    unsigned long long v10;  // r13
    unsigned int v11;  // eax
    unsigned int v12;  // eax
    char *v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]
    unsigned long long v3;  // [bp-0x8]
    char v4[8];  // [bp+0x0]

    v2 = v6;
    g_4386e0 = g_4386e0 + 1;
    __fprintf_chk(stderr, 0x1, "debug_stat (%s)\n", a0);
    if (g_438668 == 1)
    {
        v3 = v10;
        v1 = v2;
        v0 = &v4[0];
        if (*((int *)&g_4385f8) < 0 && *((int *)&g_4385f8) != 0xffffff9c)
            __assert_fail(); /* do not return */
        v11 = fstatat(*((int *)&g_4385f8), v4, a1, 0);
        if (!v11)
            return v11;
        v12 = *(__errno_location());
        if (*(__errno_location()) != 2 && *(__errno_location()) != 20)
            return v11;
        if (!(g_438660 & 2))
            return fstatat();
        __fprintf_chk(stderr, 0x1, "fallback_stat(): stat(%s) failed; falling back on lstat()\n", v4);
        return fstatat();
    }
    else if (g_438668 == 2)
    {
        v3 = v7;
        v1 = v2;
        v0 = &v4[0];
        if (*((int *)&g_4385f8) < 0 && *((int *)&g_4385f8) != 0xffffff9c)
            __assert_fail(); /* do not return */
        if (g_4385e0)
            return fstatat();
        v8 = fstatat();
        if (!v8)
            return v8;
        v9 = *(__errno_location());
        if (*(__errno_location()) != 2 && *(__errno_location()) != 20)
            return v8;
        if (!(g_438660 & 2))
            return fstatat();
        __fprintf_chk(stderr, 0x1, "fallback_stat(): stat(%s) failed; falling back on lstat()\n", v4);
        return fstatat();
    }
    else if (g_438668)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        if (*((int *)&g_4385f8) >= 0 || *((int *)&g_4385f8) == 0xffffff9c)
            return fstatat();
        __assert_fail(); /* do not return */
        return fstatat();
    }
}



// Function: digest_mode @ 0xdfe0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern unsigned int g_4385e0;
extern unsigned short g_4385e4;
extern char g_4385e5;
extern unsigned int g_4385e8;
extern unsigned int g_438668;

unsigned int digest_mode(unsigned int *a0, long long a1, long long a2, struct_0 *idx, unsigned int a4)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax

    v1 = *(a0);
    if (v1)
    {
        if (((unsigned short)v1 & 0xf000) != 0xa000 || g_438668 != 1 && (g_438668 != 2 || g_4385e0))
        {
            g_4385e5 = 1;
            g_4385e8 = v1;
            idx->field_18 = v1;
            return 1;
        }
        else if (!sub_40d820(a1, a2, idx))
        {
            g_4385e8 = idx->field_18;
            *(a0) = g_4385e8;
            g_4385e5 = 1;
            return 1;
        }
    }
    else
    {
        if ((char)a4)
        {
            g_4385e8 = 0;
            g_4385e4 = 0;
            return a4;
        }
        else if (!sub_40d820(a1, a2, idx))
        {
            v2 = idx->field_18;
            *(a0) = v2;
            g_4385e8 = v2;
            g_4385e5 = 1;
            return 1;
        }
    }
    return 0;
}



// Function: default_prints @ 0xe0e0
typedef struct struct_0 {
    char padding_0[25];
    char field_19;
    char padding_1a[238];
    struct struct_0 *field_108;
} struct_0;

unsigned int default_prints(struct_0 *iter)
{
    for (; iter; iter = iter->field_108)
    {
        if (iter->field_19)
            return 0;
    }
    return 1;
}



// Function: looks_like_expression @ 0xe120

unsigned int looks_like_expression(char *a0, unsigned int a1)
{
    unsigned long v2;  // rax

    switch (*(a0))
    {
    case 33: case 40:
        return _INSERT(v2, 0, !a0[1]);
    case 41: case 44:
        if (a0[1])
            return 0;
        return a1 ^ 1;
    case 45:
        return _INSERT(v2, 0, a0[1]);
    default:
        return 0;
    }
}



// Function: process_leading_options @ 0xe180
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[2];
    char field_2;
} struct_0;

extern char g_436180;
extern char *g_436198;
extern char *g_436258;
extern FILE *stdout;
extern void g_438660;
extern unsigned short g_438688;
extern unsigned int g_43868c;

int process_leading_options(int a0, unsigned long a1)
{
    int v5;  // r12d
    unsigned long idx;  // rax
    char **iter;  // r14
    char *v17;  // r13
    unsigned long v18;  // rbx
    unsigned long long *v19;  // rax
    int *err;  // rax
    int v21;  // ebx
    int *v22;  // rbp
    unsigned long long num;  // rax
    char *v24;  // rax
    unsigned long long v7;  // rbp
    char *v25;  // rax
    char *v26;  // rsi
    char *v32;  // rax
    struct_0 *v8;  // rbx
    int v9;  // r12d
    char *tok;  // rax
    char *tok1;  // r12
    char **v12;  // r15
    char *v13;  // rdi
    unsigned long long v14;  // rbx
    unsigned long v0;  // [bp-0x68]
    int v1;  // [bp-0x5c]
    char *v2;  // [bp-0x50]
    char v3[2];  // [bp-0x42]

    v5 = 1;
    if (a0 <= 1)
        return 1;
    while (1)
    {
        idx = v5;
        v7 = idx * 8;
        v8 = *((long long *)(a1 + idx * 8));
        if (!strcmp("-H", v8))
        {
            sub_414af0(2);
        }
        else if (!strcmp("-L", v8))
        {
            sub_414af0(1);
        }
        else if (strcmp("-P", v8))
        {
            if (!strcmp("--", v8))
                return v5 + 1;
            if (!strcmp("-D", v8))
            {
                v9 = v5 + 1;
                if (v9 < a0)
                {
                    v2 = NULL;
                    v3 = 44;
                    v0 = *((long long *)(a1 + v7 + 8));
                    tok = strtok_r(*((long long *)(a1 + v7 + 8)), v3, &v2);
                    if (tok)
                    {
                        v1 = v9;
                        tok1 = tok;
                        do
                        {
                            v12 = &g_436198;
                            v13 = "exec";
                            v14 = 0;
                            iter = &g_436198;
                            while (1)
                            {
                                if (strcmp(v13, tok1))
                                {
                                    v14 += 1;
                                    if (v14 != 9)
                                    {
                                        v13 = *(iter);
                                        iter += 3;
                                    }
                                    else
                                    {
                                        sub_4239f0(0, g_43868c, v0);
                                        error(0, 0, dcgettext(NULL, "Ignoring unrecognised debug flag %s", 5));
                                        break;
                                    }
                                }
                                else
                                {
                                    *((long long *)&g_438660) = *((long long *)&g_438660) | *((int *)&(&g_436180)[8 + 24 * v14]);
                                    break;
                                }
                            }
                            tok1 = strtok_r(NULL, v3, &v2);
                        } while (tok1);
                        v5 = v1;
                        if (g_438660 & 16)
                        {
                            v32 = dcgettext(NULL, "Valid arguments for -D:\n", 5);
                            fputs(v32, stdout);
                            while (1)
                            {
                                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%-10s %s\n");
                                if (v12 == &g_436258)
                                    break;
                                v12 += 3;
                            }
                            exit(0); /* do not return */
                        }
                        goto LABEL_40e348;
                    }
                    else
                    {
                        v26 = "Empty argument to the -D option.";
                    }
                }
                else
                {
                    v26 = "Missing argument after the -D option.";
                }
                error(0, 0, dcgettext(NULL, v26, 5));
                sub_40d570(1); /* do not return */
            }
            if (strncmp("-O", v8, 2))
                return v5;
            v17 = &v8->field_2;
            v18 = v8->field_2;
            if (!(char)v18)
                error(1, 0, dcgettext(NULL, "The -O option must be immediately followed by a decimal integer", 5));
            v19 = __ctype_b_loc();
            if (!(*((char *)(*(v19) + v18 * 2 + 1)) & 8))
                error(1, 0, dcgettext(NULL, "Please specify a decimal number immediately after -O", 5));
            err = __errno_location();
            v21 = *(err);
            *(err) = 0;
            v22 = err;
            num = strtoul(v17, &v2, 10);
            if (num)
            {
                if (*(v2))
                    error(1, 0, dcgettext(NULL, "Invalid optimisation level %s", 5));
                if (num == 0xffffffffffffffff)
                {
                    if (!*(v22))
                    {
                        v25 = dcgettext(NULL, "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.", 5);
                        break;
                    }
                    else
                    {
                        v24 = dcgettext(NULL, "Invalid optimisation level %s", 5);
                        error(1, *(v22), v24);
                    }
                }
                else
                {
                    if (num > 0xffff)
                        error(1, 0, dcgettext(NULL, "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.", 5));
                }
            }
            else if (v17 == v2)
            {
                error(1, 0, dcgettext(NULL, "Please specify a decimal number immediately after -O", 5));
            }
            else if (*(v2))
            {
                error(1, 0, dcgettext(NULL, "Invalid optimisation level %s", 5));
            }
            g_438688 = num;
            *(v22) = v21;
        }
        else
        {
            sub_414af0(0);
        }
LABEL_40e348:
        v5 += 1;
        if (a0 <= v5)
            return v5;
    }
    error(1, 0, v25);
}



// Function: set_option_defaults @ 0xe630
typedef struct struct_0 {
    unsigned short field_0;
    char padding_2[2];
    unsigned long long field_4;
    char field_c;
    unsigned short field_d;
    char field_f;
    char field_10;
    char field_11;
    char padding_12[6];
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    char field_38;
    char padding_39[3];
    unsigned int field_3c;
    unsigned long long field_40;
    char padding_48[16];
    char field_58;
    char padding_59[3];
    unsigned int field_5c;
    char padding_60[8];
    unsigned short field_68;
    char padding_6a[2];
    unsigned int field_6c;
    unsigned long long field_70;
    char field_78;
} struct_0;

void set_option_defaults(struct_0 *idx)
{
    char v3;  // al
    unsigned int v4;  // eax
    unsigned long long v5;  // rdx
    unsigned long long t;  // rax
    char v7;  // 4107
    int v0[3];  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    (&idx->field_c)[1] = getenv("POSIXLY_CORRECT");
    v3 = sub_40d920();
    *((unsigned int *)&idx->padding_48[12]) = 0;
    idx->padding_48[8] = v3;
    v4 = isatty(0);
    *((char *)&idx->field_4 + 7) = 0;
    idx->field_c = v4;
    if ((&idx->field_c)[1])
        idx->field_c = 0;
    *((unsigned long long *)(idx->padding_2)[1]) = 0xffffffffffffffff;
    idx->field_0 = 1;
    if (!gettimeofday(v0, NULL))
    {
        v5 = v1 * 1000;
        t = (unsigned long long)v0;
    }
    else
    {
        t = time(NULL);
        if (t == 0xffffffffffffffff)
            __assert_fail(); /* do not return */
        v5 = 0;
    }
    *((unsigned long long *)&idx->padding_12[1]) = t;
    idx->field_20 = t - 86400;
    v7 = (&idx->field_c)[1];
    *((unsigned short *)((char *)&idx->field_4 + 5)) = 0;
    idx->field_18 = v5;
    idx->field_28 = v5;
    *((char *)&idx->field_30) = 0;
    *((unsigned int *)((char *)&idx->field_30 + 4)) = (-(v7 < 1) & 0x200) + 0x200;
    *((unsigned long long *)&idx->field_38) = 0;
    *((unsigned short *)&idx->padding_60[0]) = 2;
    if (!getenv("FIND_BLOCK_SIZE"))
    {
        *((char *)&idx->field_4 + 4) = 0;
        sub_414af0(0);
        *((unsigned int *)&idx->padding_60[4]) = 8;
        *((unsigned long long *)&idx->field_68) = 0;
        *((char *)&idx->field_70) = 0;
        return;
    }
    error(1, 0, dcgettext(NULL, "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable", 5));
}



// Function: apply_predicate @ 0xe7b0
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[16];
    char field_18[8];
    char field_1a;
    char field_1b;
    char field_1c;
    char padding_1d[259];
    unsigned long long field_120;
    unsigned long long field_128;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern char g_4385e4;
extern char g_4385e5;
extern long long g_4385f0;

unsigned long long apply_predicate(long long a0, struct_0 *a1, struct_0 *idx)
{
    unsigned long long v2;  // rax
    long long v0;  // [bp-0x20]

    *((unsigned long long *)&idx->padding_1d[253]) = *((long long *)&idx->padding_1d[253]) + 1;
    if (0xffffff0000 & idx->field_18 && (idx->field_18[2] && !g_4385e4 || idx->field_18[3] && !g_4385e5 || idx->field_18[4] && (!*((long long *)&a1->padding_8[0]) || !g_4385e5 || ((short)*((int *)&a1->field_18[0]) & 0xf000) == 0x4000)))
    {
        v0 = a0;
        if (sub_40d820(a0, g_4385f0, a1))
            return 0;
    }
    v2 = idx->field_0();
    if ((char)v2)
    {
        idx->field_120 = idx->field_120 + 1;
        return v2;
    }
    return 0;
}



// Function: is_exec_in_local_dir @ 0xe880
char is_exec_in_local_dir(unsigned long a0)
{
    return a0 == sub_4095d0 | a0 == sub_408fc0;
}



// Function: fatal_target_file_error @ 0xe920
extern char g_438618;
extern unsigned int g_43868c;

void fatal_target_file_error(int a0, long long a1)
{
    if (g_438618)
    {
        g_438618 = 1;
        abort(); /* do not return */
    }
    sub_4239f0(0, g_43868c, a1);
    error(1, a0, "%s");
}



// Function: fatal_nontarget_file_error @ 0xe9b0
extern char g_438618;
extern unsigned int g_43868c;

void fatal_nontarget_file_error(int a0, long long a1)
{
    g_438618 = 0;
    sub_4239f0(0, g_43868c, a1);
    error(1, a0, "%s");
}



// Function: entry_comparator @ 0xea00
unsigned long long entry_comparator(unsigned long long *a0, unsigned long a1)
{
    if (a0[1] == *((long long *)(a1 + 8)))
        return _INSERT(*((long long *)a1), 0, *(a0) == *((long long *)a1));
    return 0;
}



// Function: entry_hashfunc @ 0xea30
void entry_hashfunc(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: sharefile_init @ 0xea90
typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
} struct_0;

struct_0 * sharefile_init(char *a0)
{
    struct_0 *ptr;  // r12
    char *str;  // rax
    unsigned long long v3;  // rax

    ptr = malloc(16);
    if (!ptr)
        return ptr;
    str = strdup(a0);
    ptr->field_0 = str;
    if (!str)
    {
        free(ptr);
        return NULL;
    }
    v3 = sub_41b2f0(11, 0, sub_40ea30, sub_40ea00, sub_40ea50);
    ptr->field_8 = v3;
    if (!v3)
    {
        free(ptr->field_0);
        free(ptr);
        return NULL;
    }
    return ptr;
}



// Function: sharefile_destroy @ 0xeb20
typedef struct struct_0 {
    void* field_0;
    long long field_8;
} struct_0;

void sharefile_destroy(struct_0 *a0)
{
    free(a0->field_0);
    sub_41b4b0(a0->field_8);
    return;
}



// Function: sharefile_fopen @ 0xeb40
typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    void* field_10;
    void* field_18;
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

void* sharefile_fopen(long long *a0, unsigned long a1)
{
    struct_0 *ptr;  // rax
    struct_0 *ptr1;  // rbp
    unsigned int *err;  // rax
    void* str;  // rax
    void* v5;  // rax
    void* v6;  // r12
    unsigned int v7;  // eax
    long long v9;  // rdi
    struct_1 *v10;  // rax
    void* v11;  // r12
    stat v0;  // [bp-0xc8]

    ptr = malloc(32);
    if (!ptr)
        return NULL;
    ptr1 = ptr;
    str = strdup(a1);
    ptr1->field_10 = str;
    if (!str)
    {
        free(ptr1);
        return str;
    }
    v5 = sub_4182a0(a1, *(a0));
    ptr1->field_18 = v5;
    v6 = v5;
    if (!v6)
    {
        sub_40ea50(ptr1);
        return v6;
    }
    v7 = fileno(v6);
    if (v7 < 0)
        __assert_fail(); /* do not return */
    sub_417990(v7, 1);
    if (fstat(v7, &v0) < 0)
    {
        sub_40ea50(ptr1);
        return NULL;
    }
    v9 = a0[1];
    ptr1->field_0 = *((unsigned long long *)&v0);
    ptr1->field_8 = v0.st_ino;
    v10 = sub_41b060(v9, ptr1);
    if (v10)
    {
        sub_40ea50(ptr1);
        return v10->field_18;
    }
    v11 = sub_41b950(a0[1], ptr1);
    if (!v11)
    {
        err = __errno_location();
        sub_40ea50(ptr1);
        *(err) = *(err);
        return v11;
    }
    return ptr1->field_18;
}



// Function: checked_fprintf @ 0xece0
void checked_fprintf(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40ece7();
    return;
}



// Function: format_date @ 0xedc0
typedef struct tm {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
    long tm_gmtoff;
    char * tm_zone;
} tm;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern void g_438b00;
extern struct_0 *g_438b08;
extern void g_438b20;
extern void g_438e37;

void* format_date(int a0[2], unsigned long long a1, unsigned int a2)
{
    unsigned long v20;  // rax
    unsigned int v21;  // r13d
    char *v30;  // r12
    int v31;  // edx
    unsigned long t;  // r15
    unsigned long v33;  // rbx
    unsigned long v34;  // rax
    void* v35;  // r12
    char v36;  // r13b
    unsigned long long v37;  // r14
    char v38;  // bl
    unsigned long long *v39;  // rax
    unsigned int v22;  // ebp
    unsigned long long *v40;  // rax
    unsigned long long v41;  // rcx
    void* ptr;  // r13
    unsigned long v43;  // r8
    unsigned int v44;  // esi
    unsigned long long *v45;  // rdi
    unsigned int v46;  // esi
    void* v47;  // r13
    unsigned int v48;  // eax
    unsigned int i;  // edx
    tm *tm;  // rbx
    unsigned int v50;  // edx
    void* iter;  // r12
    unsigned long long v25;  // rax
    unsigned long v26;  // rbx
    char *v27;  // rax
    char *ptr1;  // rbp
    unsigned long v0;  // [bp-0x218]
    char *v1;  // [bp-0xf8]
    unsigned long long v2;  // [bp-0xf0]
    char v3;  // [bp-0xe1]
    unsigned long long v4;  // [bp-0xe0]
    unsigned long v5;  // [bp-0xd8]
    unsigned long v6;  // [bp-0xd0]
    int v7[2];  // [bp-0xc8]
    unsigned long long v8;  // [bp-0xc0]
    int v9;  // [bp-0xb8], Other Possible Types: tm
    int v10;  // [bp-0xa8]
    int v11;  // [bp-0x98]
    unsigned long v12;  // [bp-0x88]
    char v13;  // [bp-0x74]
    char v14;  // [bp-0x73]
    char v15;  // [bp-0x72]
    char v16;  // [bp-0x70]
    char v17;  // [bp-0x6c]
    char v18[40];  // [bp-0x68]

    v7 = a0;
    v8 = a1;
    if (a2 != 43)
    {
        v20 = a2 - 64;
        v13 = 37;
        v14 = a2;
        v15 = 0;
        if ((unsigned int)v20 > 24 || !(v0 = 18350081, ((char)v20 & 254 | *((char *)((char *)&v0 + ((long long)(v20 & 63) >> 3))) >> (unsigned long long)((char)v20 & 63 & 7) & 1) & 1))
        {
            *(v18) = 0;
            v21 = 0;
            v22 = 0;
            if (a2 == 64)
                goto LABEL_40ee40;
        }
        else
        {
            v21 = 1;
            v22 = __snprintf_chk(v18, 32, 1, 32, ".%09ld0");
            if (a2 == 64)
                goto LABEL_40ee40;
        }
    }
    else
    {
        strncpy(&v13, "%Y-%m-%d+%T", 11);
        v21 = 1;
        v22 = __snprintf_chk(v18, 32, 1, 32, ".%09ld0");
    }
    tm = localtime(v7);
    if (tm)
    {
        v5 = v22;
        v27 = sub_425a50(strlen(&v13) + 2);
        *(v27) = 95;
        ptr1 = v27;
        memcpy(ptr1 + 1, &v13, strlen(&v13) + 1);
        v30 = g_438b08;
        v9 = (int)*((int128_t *)&tm->tm_sec);
        v31 = *((unsigned int *)&v9);
        v10 = (int)*((int128_t *)&tm->tm_mon);
        v11 = (int)*((int128_t *)&tm->tm_isdst);
        v12 = tm->tm_zone;
        v9 = (tm)(11 <= v31 ? v31 - 11 : v31 + 11);
        if (!v30)
        {
            *((unsigned long *)&g_438b00) = 1;
            g_438b08 = sub_425a50(1);
            v30 = g_438b08;
        }
        while (1)
        {
            t = strftime(v30, *((long long *)&g_438b00), ptr1, tm);
            if (t && t < *((long long *)&g_438b00))
                break;
            g_438b08 = sub_425ca0(v30, &g_438b00, 1);
            v30 = g_438b08;
        }
        v33 = v5 + t + 1;
        v6 = v33;
        v34 = sub_425aa0(v30, v33);
        *((unsigned long *)&g_438b00) = v33;
        g_438b08 = v34;
        v35 = sub_425a50(v33);
        strftime(v35, *((long long *)&g_438b00), ptr1, &v9);
        v36 = g_438b08->field_0;
        v1 = g_438b08;
        if ((char)g_438b08->field_0)
        {
            v2 = 0;
            v37 = 0;
            v4 = 0;
            v3 = 0;
            while (1)
            {
                v38 = *((char *)v35 + v37);
                if (!v38)
                    break;
                if (v38 != v36)
                {
                    v39 = __ctype_b_loc();
                    if (!(*((char *)(*(v39) + v36 * 2 + 1)) & 8) || !(*((char *)(*(v39) + v38 * 2 + 1)) & 8))
                        break;
                    if (v3)
                    {
                        if (v2 != v37 - v4)
                            break;
                        v2 += 1;
                    }
                    else
                    {
                        v4 = v37;
                        v2 = 1;
                        v3 = 1;
                    }
                }
                v37 += 1;
                v36 = v1[v37];
                if (!v36 && !*((char *)v35 + v37) && v2 == 2)
                {
                    v40 = __ctype_b_loc();
                    v41 = v4 + 2;
                    ptr = &v1[v41];
                    if (!(*((char *)(*(v40) + *((char *)ptr) * 2 + 1)) & 8))
                    {
                        v43 = t + 1;
                        if (v6 != v43 + v5)
                            __assert_fail(); /* do not return */
                        memmove(&v1[v41 + v5], ptr, v43 - v41);
                        if ((unsigned int)v5 >= 8)
                        {
                            v45 = ptr + 8 & 0xfffffffffffffff8;
                            *((char [40])ptr) = v18;
                            v46 = v5;
                            *((long long *)((char *)ptr + v46 - 8)) = *((long long *)&(&v16)[v46]);
                            v47 = ptr - v45;
                            v48 = v46 + (unsigned int)v47 & 0xfffffff8;
                            if (v48 >= 8)
                            {
                                i = 0;
                                do
                                {
                                    v50 = i + 8;
                                    *((long long *)((char *)v45 + i)) = *((long long *)(v18 - v47 + i));
                                    i = v50;
                                } while (i < (v48 & 0xfffffff8));
                            }
                        }
                        else if (!((char)v5 & 4))
                        {
                            v44 = v5;
                            if (v44)
                            {
                                *((char *)ptr) = *(&v18[0]);
                                if ((char)v44 & 2)
                                {
                                    *((short *)((char *)ptr + v5 - 2)) = *((short *)(&(&v17)[v5] - 2));
                                    break;
                                }
                            }
                        }
                        else
                        {
                            *((unsigned int *)ptr) = *((unsigned int *)v18);
                            *((int *)((char *)ptr + v5 - 4)) = *((int *)&(&v17)[(unsigned int)v5]);
                            break;
                        }
                    }
                }
            }
        }
        free(ptr1);
        free(v35);
        return v1 + 1;
    }
LABEL_40ee40:
    iter = sub_41bb50();
    if (iter <= &g_438b20)
    {
        __assert_fail(); /* do not return */
    }
    else if (iter < &g_438e37)
    {
        if (v7 - 0 >> 63 & 1)
        {
            *((char *)iter - 1) = 45;
            iter -= 1;
        }
        if (!v21)
            return iter;
        v25 = iter - &g_438b20 + strlen(iter);
        if (v25 > 790)
            __assert_fail(); /* do not return */
        v26 = 790 - v25;
        if (strlen(v18) >= v26)
        {
            error(0, 0, "charsprinted=%ld but remaining=%lu: ns_buf=%s");
            if (strlen(v18) >= v26)
                __assert_fail(); /* do not return */
        }
        strcat(iter, v18);
        return iter;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: ctime_format @ 0xf400
typedef struct tm {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
    long tm_gmtoff;
    char * tm_zone;
} tm;

extern void g_436260;
extern void g_4362c0;
extern char g_438700;

long long ctime_format(unsigned long a0, unsigned long a1)
{
    tm *tm;  // rax
    unsigned long v3;  // rdx
    unsigned long v4;  // rcx
    unsigned long v5;  // rsi
    unsigned long v6;  // r8
    unsigned long v7;  // r9
    unsigned long v0;  // [bp-0x18]

    v0 = a0;
    tm = localtime(&v0);
    if (!tm)
        return sub_40edc0(v0, a1, 64);
    v3 = tm->tm_wday;
    if ((unsigned int)v3 < 0)
    {
        __assert_fail(); /* do not return */
    }
    else if ((unsigned int)v3 <= 6)
    {
        v4 = tm->tm_mon;
        if ((unsigned int)v4 < 0)
        {
            __assert_fail(); /* do not return */
        }
        else if ((unsigned int)v4 <= 11)
        {
            v5 = tm->tm_hour;
            if ((unsigned int)v5 < 0)
            {
                __assert_fail(); /* do not return */
            }
            else if ((unsigned int)v5 <= 23)
            {
                v6 = tm->tm_min;
                if ((unsigned int)v6 > 59)
                    __assert_fail(); /* do not return */
                v7 = tm->tm_sec;
                if ((unsigned int)v7 > 61)
                {
                    __assert_fail(); /* do not return */
                }
                else if (__snprintf_chk(&g_438700, 0x400, 1, 0x400, "%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d", *((long long *)&(&g_4362c0)[8 * v3]), *((long long *)&(&g_436260)[8 * v4]), tm->tm_mday, v5, v6, v7, a1, tm->tm_year + 1900) <= 0x3ff)
                {
                    return &g_438700;
                }
                else
                {
                    __assert_fail(); /* do not return */
                }
            }
            else
            {
                __assert_fail(); /* do not return */
            }
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: do_fprintf @ 0xf620
typedef struct struct_0 {
    unsigned int field_0;
    char field_4;
    char padding_5[3];
    unsigned long long field_8;
    int field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
    char padding_24[12];
    unsigned long long field_30;
    char padding_38[8];
    unsigned long long field_40;
    unsigned int field_48;
    char padding_4c[4];
    unsigned int field_50;
    char padding_54[4];
    unsigned int field_58;
    char padding_5c[4];
    unsigned int field_60;
    char padding_64[4];
    unsigned int field_68;
    char padding_6c[4];
    unsigned int field_70;
} struct_1;

typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
} struct_2;

extern long long g_42962f;
extern void g_4385e0;
extern long long g_4385f0;
extern unsigned int g_4385f8;
extern int g_4385fc;
extern unsigned int g_438604;
extern struct_2 *g_438680;

long long do_fprintf(void* a0, struct_0 *index, unsigned long a2, struct_1 *idx, unsigned int a4, unsigned int a5)
{
    unsigned int v7;  // eax
    unsigned long v8;  // rbp
    void* v14;  // rbp
    unsigned int v15;  // rcx
    unsigned int v16;  // r8
    unsigned int v17;  // r9
    unsigned long v18;  // fs
    long long v19;  // rdx
    unsigned int v20;  // eax
    unsigned long v21;  // rsi
    long long *v9;  // rax
    unsigned long v22;  // rsi
    unsigned long v23;  // rdx
    unsigned int *err;  // rax
    void* iter;  // rax
    char *ptr;  // rax
    unsigned int *err1;  // rax
    unsigned long v10;  // rdi
    long long v28;  // rax
    unsigned int v29;  // rcx
    unsigned int v30;  // r8
    unsigned int v31;  // r9
    long long v11;  // r8
    char *v32;  // rax
    int *err2;  // rax
    unsigned int v34;  // rcx
    unsigned int v35;  // r8
    unsigned int v36;  // r9
    int *v37;  // rax
    long long v12;  // rcx
    unsigned long v38;  // rsi
    int v39;  // eax
    unsigned long v13;  // rdx
    unsigned int v40;  // eax
    unsigned long v42;  // rbp
    unsigned int *v43;  // rax
    unsigned long v44;  // fs
    char v0;  // [bp-0x360]
    char v1;  // [bp-0x358]
    char v3;  // [bp-0x2c8]
    char v4;  // [bp-0x2be]
    unsigned long v5;  // [bp-0x30]

    v7 = index->field_0;
    if (v7 == 1)
    {
        v42 = index->field_10;
        if (v42 > fwrite(index->field_8, 1, v42, (long long)a0[8]))
        {
            v43 = __errno_location();
            sub_40e970(*(v43), (long long)a0[16]);
        }
        if (sub_4180c0((long long)a0[8]))
        {
            __errno_location();
            return sub_40e970();
        }
    }
    else if (v7 != 2)
    {
        if (!v7)
            goto LABEL_40f688;
        goto LABEL_40f65e;
    }
    else
    {
        switch (index->field_4)
        {
        case 0:
            error(1, 0, dcgettext(NULL, "error: %s at end of format string", 5));
        case 37:
LABEL_40f688:
            v8 = index->field_10;
            if (v8 > fwrite(index->field_8, 1, v8, (long long)a0[8]))
            {
                __errno_location();
                return sub_40e970();
            }
            break;
        case 68:
            v10 = idx->field_0;
            v11 = 1;
            v12 = 1;
            break;
        case 70:
            sub_4108e0(idx, a2);
            goto LABEL_40f7c4;
        case 72:
            v14 = sub_425a50(g_4385fc + 1);
            memcpy(v14, a2, g_4385fc);
            *((char *)v14 + g_4385fc) = 0;
            sub_40ece0(a0, index->field_8, v14, v15, v16, v17);
            if (v5 != *((long long *)(40 + v18)))
                __stack_chk_fail(); /* do not return */
            return (unsigned long long)free(v14);
        case 77:
            sub_418290(idx, &v3);
            v4 = 0;
            sub_40ece0(a0, index->field_8, &v3, v29, v30, v31);
            goto LABEL_40f65e;
        case 80:
LABEL_40f7c4:
            if (sub_416e80((long long)a0[8], (long long)a0[32], (char)a0[24], index->field_8) < 0)
            {
                __errno_location();
                return sub_40e970();
            }
            break;
        case 83:
            v13 = idx->field_40;
            return (unsigned long long)sub_40ece0(a0, index->field_8, v13, idx, a4, a5);
        case 89:
            v20 = idx->field_18 & 0xf000;
            if (v20 != 0xa000)
            {
                v19 = "f";
                if (v20 != 0x8000 && !(v19 = (long long)"d", v20 == 0x4000))
                {
LABEL_40f96d:
                    v19 = "s";
                    switch (v20)
                    {
                    case 49152:
                        break;
                    case 24576:
                        v19 = "b";
                        break;
                    case 8192:
                        v19 = "c";
                        break;
                    case 4096:
                        v19 = "U";
                        v19 = "p";
                        break;
                    default:
                        v19 = "U";
                        break;
                    }
                }
LABEL_40f9ae:
                if (v5 != *((long long *)(40 + v18)))
                    __stack_chk_fail(); /* do not return */
                v21 = index->field_8;
                return (unsigned long long)sub_40ece0(a0, index->field_8, v19, v34, v35, v36);
            }
            if (fstatat(g_4385f8, g_4385f0, &v1, 0))
            {
                v37 = __errno_location();
                v38 = index->field_8;
                switch (v39)
                {
                case 2: case 20:
                    sub_40ece0(a0, v38, "N", v29, v30, v31);
                    break;
                case 40:
                    sub_40ece0(a0, v38, "L", v29, v30, v31);
                    break;
                default:
                    sub_40ece0(a0, v38, "?", v29, v30, v31);
                    sub_40e8a0(0, a2);
                    error(0, *(v37), "%s");
                    break;
                }
            }
            else
            {
                v28 = "f";
                switch (v40)
                {
                case 32768:
                    break;
                case 16384:
                    v28 = "d";
                    break;
                case 40960:
                    v28 = "l";
                    break;
                case 49152:
                    v28 = "s";
                    break;
                case 24576:
                    v28 = "b";
                    break;
                case 8192:
                    v28 = "c";
                    break;
                case 4096:
                    v28 = "U";
                    v28 = "p";
                default:
                    v28 = "U";
                    v28 = "p";
                }
LABEL_40f778:
                sub_40ece0(a0, index->field_8, v28, v29, v30, v31);
                goto LABEL_40f65e;
            }
        case 90:
            if (g_438680(g_4385f8, g_4385f0, &v0) >= 0)
            {
                sub_40ece0(a0, index->field_8, v0, v29, v30, v31);
                freecon(v0);
                goto LABEL_40f65e;
            }
            else
            {
                sub_40ece0(a0, index->field_8, &g_42962f, v29, v30, v31);
                sub_40e8a0(0, a2);
                v32 = dcgettext(NULL, "getfilecon failed: %s", 5);
                err2 = __errno_location();
                error(0, *(err2), v32);
                g_438604 = 1;
                goto LABEL_40f65e;
            }
        case 97:
            v19 = sub_40f400(*((long long *)&idx->field_48), *((long long *)&idx->field_50));
            if (v5 != *((long long *)(40 + v18)))
                __stack_chk_fail(); /* do not return */
            return (unsigned long long)sub_40ece0(a0, index->field_8, v19, v34, v35, v36);
        case 98:
            v10 = idx->field_40;
            v11 = 0x200;
            v12 = 0x200;
            break;
        case 99:
            v19 = sub_40f400(*((long long *)&idx->field_68), *((long long *)&idx->field_70));
            if (v5 != *((long long *)(40 + v18)))
                __stack_chk_fail(); /* do not return */
            return (unsigned long long)sub_40ece0(a0, index->field_8, v19, v34, v35, v36);
        case 100:
            if (v5 != *((long long *)(40 + v18)))
                __stack_chk_fail(); /* do not return */
            v22 = index->field_8;
            v23 = *((int *)&g_4385e0);
            return (unsigned long long)sub_40ece0(a0, v22, v23, idx, a4, a5);
        case 102:
            v14 = sub_417b90(a2);
LABEL_40fbce:
            if (sub_416e80((long long)a0[8], (long long)a0[32], (char)a0[24], index->field_8, v14) < 0)
                goto LABEL_40fbef;
            goto LABEL_40fb18;
        case 104:
            v14 = sub_425f30(a2);
            iter = v14 + strlen(v14) - 1;
            if (v14 <= iter)
            {
                do
                {
                    if (*((char *)iter) != 47 && v14 < iter && (char)iter[1] == 47)
                    {
                        *((char *)&iter[1]) = 0;
                        break;
                    }
                } while ((iter -= 1, v14 <= iter));
            }
            ptr = strrchr(v14, 47);
            if (ptr)
            {
                *(ptr) = 0;
                goto LABEL_40fbce;
            }
            else
            {
                if (sub_416e80((long long)a0[8], (long long)a0[32], (char)a0[24], index->field_8, ".") < 0)
                {
LABEL_40fbef:
                    err1 = __errno_location();
                    sub_40e970(*(err1), (long long)a0[16]);
                }
LABEL_40fb18:
                if (v5 != *((long long *)(40 + v18)))
                    __stack_chk_fail(); /* do not return */
                return (unsigned long long)free(v14);
            }
        case 105:
            v10 = idx->field_8;
            v11 = 1;
            v12 = 1;
            break;
        case 107:
            v10 = idx->field_40;
            v11 = 0x400;
            v12 = 0x200;
            break;
        case 108:
            if (((unsigned short)idx->field_18 & 0xf000) != 0xa000)
            {
LABEL_40faf1:
                v14 = NULL;
                if (sub_416e80((long long)a0[8], (long long)a0[32], (char)a0[24], index->field_8, &g_42962f) < 0)
                    goto LABEL_40fbef;
                goto LABEL_40fb18;
            }
            else
            {
                v14 = sub_417650(g_4385f8, g_4385f0);
                if (!v14)
                {
                    err = __errno_location();
                    sub_40e8c0(*(err), a2);
                    g_438604 = 1;
                    goto LABEL_40faf1;
                }
            }
        case 109:
            if (v5 != *((long long *)(40 + v18)))
                __stack_chk_fail(); /* do not return */
            v22 = index->field_8;
            v23 = idx->field_18 & 0xfff;
            return (unsigned long long)sub_40ece0(a0, index->field_8, idx->field_18 & 0xfff, idx, a4, a5);
        case 110:
            v10 = idx->field_10;
            v11 = 1;
            v12 = 1;
            break;
        case 112:
            if (sub_416e80((long long)a0[8], (long long)a0[32], (char)a0[24], index->field_8, a2) < 0)
            {
                __errno_location();
                return sub_40e970();
            }
            break;
        case 115:
            v10 = idx->field_30;
            v11 = 1;
            v12 = 1;
            break;
        case 116:
            v19 = sub_40f400(*((long long *)&idx->field_58), *((long long *)&idx->field_60));
            if (v5 != *((long long *)(40 + v18)))
                __stack_chk_fail(); /* do not return */
            return (unsigned long long)sub_40ece0(a0, index->field_8, v19, v34, v35, v36);
        case 121:
            v19 = "f";
            v20 = idx->field_18 & 0xf000;
            if (v20 != 0x8000 && !(v19 = (long long)"d", v20 == 0x4000 || (v19 = (long long)"l", v20 == 0xa000)))
                goto LABEL_40f96d;
            goto LABEL_40f9ae;
        case 103:
            v9 = getgrgid(idx->field_20);
            if (v9)
                goto LABEL_40f9ef;
            break;
        case 71:
            v10 = idx->field_20;
            v11 = 1;
            v12 = 1;
            break;
        case 117:
            v9 = getpwuid(idx->field_1c);
            if (v9)
            {
LABEL_40f9ef:
                *((char *)(index->field_8 + index->field_10)) = 115;
                if (v5 != *((long long *)(40 + v18)))
                    __stack_chk_fail(); /* do not return */
                v21 = index->field_8;
                v19 = *(v9);
                return (unsigned long long)sub_40ece0(a0, v21, v19, v34, v35, v36);
            }
        case 85:
            v10 = idx->field_1c;
            v11 = 1;
            v12 = 1;
            break;
        default:
LABEL_40f65e:
            return v5 - *((long long *)(40 + v44));
        }
        v28 = sub_41bb50(v10, &v3, 0, v12, v11);
        goto LABEL_40f778;
    }
}



// Function: pred_fprintf @ 0xff90
typedef struct struct_0 {
    unsigned int field_0;
    char field_4;
    char padding_5[3];
    unsigned long long field_8;
    int field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[88];
    unsigned long long field_58;
    unsigned long long field_60;
} struct_1;

extern unsigned int g_42962f;

int pred_fprintf(void)
{
    struct_0 *v1;  // rdx
    struct_0 *v2;  // rbx
    struct_1 *idx;  // rsi
    void* v12;  // rax
    unsigned int v13;  // rcx
    struct_0 *v3;  // r12
    unsigned int v14;  // rdi
    unsigned int v15;  // r8d
    unsigned int v16;  // r9d
    struct_0 *index;  // rbx
    char v5;  // dl
    unsigned long long v7;  // rsi
    unsigned long v8;  // rdi

    v2 = v1->field_38;
    if (!v1->field_38)
        return;
    v3 = &v1->field_38;
    while (1)
    {
        while (1)
        {
            index = v2;
            if (*((int *)&index->padding_0[0]) == 2 && !(v5 = index->padding_0[5], !index->padding_0[5]))
                break;
            sub_40f620(v3, index, v14, idx, v15, v16);
            v2 = *((long long *)&index->padding_0[24]);
            if (!*((long long *)&index->padding_0[24]))
                return;
        }
        switch (index->padding_0[4])
        {
        case 67:
            v7 = *((long long *)&idx[1].padding_0[8]);
            v8 = *((long long *)&idx[1].padding_0[0]);
            v12 = sub_40edc0(v8, v7, v5);
            sub_40ece0(v3, *((long long *)&index->padding_0[8]), v12, v13, v15, v16);
            goto LABEL_40fffc;
        case 84:
            v7 = idx->field_60;
            v8 = idx->field_58;
            break;
        case 65:
            v7 = *((long long *)&idx->padding_0[80]);
            v8 = *((long long *)&idx->padding_0[72]);
            break;
        case 66:
            v8 = 0xffffffffffffffff;
            v7 = 0xffffffffffffffff;
            if (v5 == 64)
                break;
            sub_40ece0(v3, *((long long *)&index->padding_0[8]), &g_42962f, v13, v15, v16);
LABEL_40fffc:
            v2 = *((long long *)&index->padding_0[24]);
            if (!*((long long *)&index->padding_0[24]))
                return;
            continue;
        default:
            __assert_fail(); /* do not return */
        }
    }
}



// Function: make_segment @ 0x100b0
typedef struct struct_2 {
    char padding_0[1];
    char field_1;
} struct_2;

typedef struct struct_0 {
    unsigned int field_0;
    char field_4;
    char field_5;
    char padding_6[2];
    char *field_8;
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char field_4;
    char field_5;
    char padding_6[2];
    unsigned long long field_8;
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
} struct_1;

typedef struct struct_3 {
    char padding_0[26];
    char field_1a;
    char padding_1b[5];
    unsigned int field_20;
} struct_3;


long long make_segment(struct_1 **a0, char *a1, int a2, unsigned int a3, char a4, char a5, struct_3 *idx)
{
    unsigned long v1;  // r14
    struct_0 *index;  // rax
    char *v3;  // rax
    struct_2 *iter;  // rcx
    unsigned int v5;  // eax
    struct_2 *v6;  // rax
    struct_1 *v7;  // rax
    char v0;  // [bp-0x39]

    v0 = a4;
    switch (a4)
    {
    case 123:
        __assert_fail(); /* do not return */
    case 91:
        __assert_fail(); /* do not return */
    case 40:
        __assert_fail(); /* do not return */
    default:
        v1 = a2;
        index = sub_425a50(32);
        *(a0) = index;
        index->field_0 = a3;
        index->field_4 = a4;
        index->field_5 = a5;
        index->field_18 = 0;
        index->field_10 = v1;
        v3 = sub_425a50(v1 + 2);
        index->field_8 = v3;
        iter = &(strncpy(v3, a1, v1))[v1];
        if (a3 > 1)
        {
            if (a3 == 2)
            {
                switch (a4)
                {
                case 37: case 100:
                    v6 = &iter->field_1;
LABEL_4101f9:
                    iter->padding_0[0] = v0;
                    v6->padding_0[0] = 0;
                    break;
                case 65: case 66: case 67: case 68: case 70: case 71: case 77: case 84: case 85: case 97: case 98: case 99: case 103: case 107: case 110: case 115: case 116: case 117:
                    iter->padding_0[0] = 115;
                    iter = &iter->field_1;
                    idx->field_1a = 1;
                    v5 = 3;
                    break;
                case 72: case 80: case 102: case 104: case 112:
                    v0 = 115;
                    v6 = &iter->field_1;
                    goto LABEL_4101f9;
                case 83:
                    iter->padding_0[0] = 103;
                    iter = &iter->field_1;
                    idx->field_1a = 1;
                    v5 = 3;
                    break;
                case 89:
                    iter->padding_0[0] = 115;
                    iter = &iter->field_1;
                    idx->field_1a = 1;
                    v5 = 2;
                    break;
                case 90:
                    iter->padding_0[0] = 115;
                    v5 = 5;
                    iter = &iter->field_1;
                    break;
                case 105:
                    iter->padding_0[0] = 115;
                    iter = &iter->field_1;
                    idx->padding_1b[1] = 1;
                    v5 = 1;
                    break;
                case 108:
                    iter->padding_0[0] = 115;
                    iter = &iter->field_1;
                    idx->field_1a = 1;
                    v5 = 4;
                    break;
                case 109:
                    iter->padding_0[0] = 111;
                    iter = &iter->field_1;
                    idx->field_1a = 1;
                    v5 = 3;
                    break;
                case 121:
                    iter->padding_0[0] = 115;
                    iter = &iter->field_1;
                    idx->padding_1b[0] = 1;
                    v5 = 2;
                    break;
                default:
                    v5 = 0;
                    break;
                }
                iter->padding_0[0] = 0;
                if (idx->field_20 >= v5)
                    return &*(a0)->field_18;
                idx->field_20 = v5;
                break;
                return &*(a0)->field_18;
            }
            else
            {
                __assert_fail(); /* do not return */
            }
        }
        else
        {
            if (a4)
            {
                __assert_fail(); /* do not return */
            }
            else if (!a5)
            {
                v7 = *(a0);
                iter->padding_0[0] = 0;
                return &v7->field_18;
            }
            else
            {
                __assert_fail(); /* do not return */
            }
        }
    }
}



// Function: insert_fprintf @ 0x10380
void insert_fprintf(unsigned long a0, long long a1, unsigned long a2)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    sub_41039e(a1);
    return;
}



// Function: get_file_system_list @ 0x10870
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

extern char g_438e38;
extern struct_0 *g_438e40;

struct_0 * get_file_system_list(char a0)
{
    struct_0 *v1;  // r12
    struct_0 *v2;  // r12
    struct_0 *v3;  // r12
    unsigned long v4;  // rax

    v1 = g_438e40;
    if (g_438e40)
    {
        if (g_438e38 == 1 || !a0)
            return g_438e40;
        do
        {
            v2 = v1;
            v3 = v2->field_30;
            sub_41efb0(v2);
            v1 = v3;
        } while (v2->field_30);
        g_438e40 = 0;
    }
    v4 = sub_41e550(a0);
    g_438e38 = a0;
    g_438e40 = v4;
    return g_438e40;
}



// Function: filesystem_type @ 0x108e0
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

extern struct_0 *g_438670;
extern unsigned long long g_438e48;
extern char g_438e50;
extern void* g_438e58;

void filesystem_type(unsigned long long *a0)
{
    struct_0 *iter;  // rbx
    struct_0 *v3;  // rbp
    unsigned long long v4;  // rax
    unsigned long v5;  // rax
    char v0;  // [bp-0xc8]

    if (g_438e58)
    {
        if (g_438e50 && *(a0) == g_438e48)
            return;
        free(g_438e58);
    }
    g_438e48 = *(a0);
    iter = sub_410870(1);
    if (iter)
    {
        v3 = NULL;
        do
        {
            if (!strcmp(*((long long *)&iter->padding_0[24]), "ignore"))
                continue;
            v4 = *((long long *)&iter->padding_0[32]);
            if (v4 == 0xffffffffffffffff)
            {
                sub_40d810(&v0);
                if (g_438670(*((long long *)&iter->padding_0[8]), &v0))
                    continue;
                v4 = v0;
                *((unsigned long long *)&iter->padding_0[32]) = v4;
            }
            if (*(a0) == v4)
                v3 = iter;
        } while ((iter = (struct_0 *)iter->field_30, iter));
        if (v3)
        {
            v5 = sub_425f30(*((long long *)&v3->padding_0[24]));
            g_438e50 = 1;
        }
        else
        {
            g_438e50 = 0;
            v5 = sub_425f30(dcgettext(NULL, "unknown", 5));
        }
        g_438e58 = v5;
        return;
    }
    error(1, 0, dcgettext(NULL, "Cannot read mounted file system list", 5));
}



// Function: is_used_fs_type @ 0x10a50
void is_used_fs_type(long long a0)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_410a63("afs", a0);
    return;
}



// Function: get_mounted_devices @ 0x10ac0
typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[16];
    unsigned long long field_20;
    char padding_28[8];
    struct struct_1 *field_30;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_438670;

int get_mounted_devices(void)
{
    struct_1 *v5;  // rax
    struct_1 *v6;  // r14
    unsigned long long *v15;  // rdi
    unsigned long v7;  // r15
    struct_1 *idx;  // r14
    unsigned long v9;  // r15
    unsigned long long v10;  // r13
    unsigned long long v11;  // r12
    unsigned long v12;  // rax
    struct_1 *v13;  // rbp
    struct_1 *v14;  // rbp
    unsigned long long v0;  // [bp-0x100]
    char *v1;  // [bp-0xf8]
    unsigned long v2;  // [bp-0xe0]
    char v3;  // [bp-0xd8]

    v2 = 0;
    v5 = sub_41e550(0);
    if (!v5)
        return;
    v6 = v5;
    v7 = 0;
    v0 = 0;
    v1 = &v2;
    while (1)
    {
LABEL_410b4c:
        idx = v6;
        v9 = v7;
        v11 = (v0 + 1) * 8;
        v7 = sub_415be0(v9, v11, v1);
        if (v7)
            break;
        v7 = 0;
        free(v9);
        v6 = idx->field_30;
        if (!v6)
            goto LABEL_410b90;
    }
    v12 = idx->field_20;
    if (v12 == 0xffffffffffffffff)
    {
        sub_40d810(&v3);
        if (!g_438670(idx->field_8, &v3))
        {
            v12 = v3;
            idx->field_20 = v12;
        }
        else
        {
            v7 = v7;
            v6 = idx->field_30;
            if (!idx->field_30)
                goto LABEL_410b90;
            else
                goto LABEL_410b4c;
        }
    }
    *((unsigned long *)(v7 + v11 - 8)) = v12;
    v0 = v10;
    do
    {
LABEL_410b90:
        v13 = v5;
        v14 = v13->field_30;
        sub_41efb0(v13);
        v5 = v14;
    } while (v13->field_30);
    if (!v7)
        return;
    *(v15) = v0;
    return;
}



// Function: parse_files0_from @ 0x10c30
extern unsigned long long g_438690;

unsigned int parse_files0_from(unsigned long a0, unsigned long long a1, int *a2)
{
    unsigned long idx;  // rcx
    unsigned int v2;  // r8d
    unsigned long v3;  // rcx

    if (!a1)
        return 0;
    idx = *(a2);
    v2 = 0;
    v3 = *((long long *)(a1 + idx * 8));
    if (v3)
    {
        v2 = 1;
        *(a2) = (unsigned int)idx + 1;
        g_438690 = v3;
    }
    return v2;
}



// Function: parse_version @ 0x10c80
extern char g_438678;
extern unsigned short g_438688;

void parse_version(void)
{
    unsigned long long v4;  // rax
    unsigned long v5;  // fs
    char *v7;  // rdx
    long long v8;  // rdi
    long long v9;  // rsi
    long long v10;  // rdx
    long long v11;  // rcx
    long long v12;  // r8
    long long v13;  // r9
    unsigned int v0;  // [bp-0x14]
    unsigned long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = *((long long *)(40 + v5));
    sub_416370("find");
    __printf_chk(1, dcgettext(NULL, "Features enabled: ", 5));
    __printf_chk(1, "D_TYPE ");
    v7 = "enabled";
    if (!g_438678)
        v7 = "disabled";
    __printf_chk(1, "O_NOFOLLOW(%s) ", v7);
    __printf_chk(1, "LEAF_OPTIMISATION ");
    if (is_selinux_enabled(v8, v9, v10, v11, v12, v13) > 0)
        __printf_chk(1, "SELINUX ");
    v0 = 0;
    if ((char)sub_408530(&v0))
    {
        __printf_chk(1, "FTS(");
        if (*((char *)((void*)&v0 + 1)) & 2)
            __printf_chk(1, "FTS_CWDFD");
        __printf_chk(1, ") ");
    }
    __printf_chk(1, "CBO(level=%d) ", g_438688);
    __printf_chk(1, "\n");
    exit(0); /* do not return */
}



// Function: parse_quit @ 0x10e20
unsigned int parse_quit(unsigned long long *a0)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_40d520(a0);
    idx[6] = 1;
    idx[9] = 0x3f800000;
    return 1;
}



// Function: parse_prune @ 0x10e50
extern char g_438620;

unsigned int parse_prune(unsigned long long *a0)
{
    unsigned short *idx;  // rax

    idx = (unsigned long long)sub_40d520(a0);
    if (!g_438620)
        idx[13] = 0;
    idx[12] = 1;
    return 1;
}



// Function: parse_nouser @ 0x10e80
unsigned int parse_nouser(unsigned long long *a0)
{
    *((unsigned int *)((unsigned long long)sub_40d520(a0) + 36)) = 981668463;
    return 1;
}



// Function: parse_accesscheck @ 0x10ea0
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[16];
    unsigned int field_18;
    char padding_1c[8];
    unsigned int field_24;
} struct_0;

unsigned int parse_accesscheck(unsigned long long *a0)
{
    struct_0 *idx;  // rax
    unsigned int v2;  // ymm0

    idx = (unsigned long long)sub_40d520(a0);
    v2 = 1045220557;
    idx->field_18 = 0;
    if (idx->field_0 != sub_4096e0)
        v2 = 1063675494;
    idx->field_24 = v2;
    return 1;
}



// Function: parse_empty @ 0x10ee0
unsigned int parse_empty(unsigned long long *a0)
{
    *((unsigned int *)((unsigned long long)sub_40d520(a0) + 36)) = 1008981770;
    return 1;
}



// Function: parse_delete @ 0x10f00
extern char g_438620;

unsigned int parse_delete(unsigned long long *a0)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_40d520(a0);
    g_438620 = 0;
    idx[6] = 0x101;
    idx[9] = 0x3f800000;
    return 1;
}



// Function: parse_type @ 0x11200
void parse_type(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: fnmatch_sanitycheck @ 0x11220
extern char g_438e60;

unsigned int fnmatch_sanitycheck(void)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    if (!g_438e60)
    {
        v0 = v2;
        return sub_411242();
    }
    return 1;
}



// Function: parse_wholename @ 0x113f0
void parse_wholename(void)
{
}



// Function: parse_nogroup @ 0x115c0
unsigned int parse_nogroup(void)
{
    *((unsigned int *)(sub_40d4d0() + 36)) = 953267991;
    return 1;
}



// Function: parse_lname @ 0x115e0
void parse_lname(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    sub_4115f3();
    return;
}



// Function: parse_ilname @ 0x11690
unsigned int parse_ilname(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (!a1)
        return 0;
    v0 = v2;
    return sub_41169e();
}



// Function: parse_size @ 0x11720
typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[20];
    unsigned int field_3c;
    unsigned long long field_40;
} struct_0;

extern long long g_42962f;

unsigned int parse_size(long long a0, unsigned long long a1, int *a2)
{
    char *v5;  // r13
    unsigned int v6;  // eax
    void* v7;  // rax
    char v8;  // bl
    unsigned long long v9;  // r12
    char v10;  // al
    struct_0 *idx;  // rax
    unsigned long v13;  // rdx
    unsigned int v14;  // ymm0
    unsigned long v0;  // [bp-0x48]
    char v1;  // [bp-0x40]
    char v2;  // [bp-0x32]
    char v3;  // [bp-0x31]

    if (!a1)
        return 0;
    v5 = *((long long *)(a1 + *(a2) * 8));
    if (v5)
    {
        v6 = strlen(v5);
        if (!v6)
            error(1, 0, dcgettext(NULL, "invalid null argument to -size", 5));
        v7 = &v5[v6] - 1;
        v8 = *((char *)v7);
        if (v8 == 98)
        {
            *((char *)v7) = 0;
            v9 = 0x200;
        }
        else if (v8 <= 98)
        {
            if (v8 == 71)
            {
                *((char *)v7) = 0;
                v9 = 0x40000000;
            }
            else if (v8 <= 71)
            {
                if (v8 - 48 > 9)
                    error(1, 0, dcgettext(NULL, "invalid -size type `%c'", 5));
                v10 = *(v5);
                v9 = 0x200;
                v8 = 0;
                if (v10 != 43)
                    goto LABEL_4117c0;
LABEL_411918:
                if (sub_426490(v5 + 1, &v1, 10, &v0, &g_42962f))
                {
                    v2 = v8;
                    v3 = 0;
                    error(1, 0, dcgettext(NULL, "Invalid argument `%s%s' to -size", 5));
                }
                idx = sub_40d4d0(a0, v5);
                v13 = v0;
                v14 = 1036831949;
                idx->field_3c = v9;
                *((unsigned int *)&idx->padding_28[16]) = 0;
                if (v13 * v9 <= 0x5000)
                    v14 = 1063675494;
LABEL_41180e:
                idx->field_40 = v13;
                *((unsigned short *)&idx->padding_0[26]) = 1;
                idx->field_24 = v14;
                *(a2) = *(a2) + 1;
                return 1;
            }
            else if (v8 == 77)
            {
                *((char *)v7) = 0;
                v9 = 0x100000;
            }
            else
            {
                error(1, 0, dcgettext(NULL, "invalid -size type `%c'", 5));
            }
        }
        else
        {
            if (v8 == 107)
            {
                *((char *)v7) = 0;
                v9 = 0x400;
            }
            else if (v8 == 0x77)
            {
                *((char *)v7) = 0;
                v9 = 2;
            }
            else if (v8 == 99)
            {
                *((char *)v7) = 0;
                v9 = 1;
            }
            else
            {
                error(1, 0, dcgettext(NULL, "invalid -size type `%c'", 5));
            }
        }
        v10 = *(v5);
        if (*(v5) == 43)
            goto LABEL_411918;
LABEL_4117c0:
        if (v10 != 45)
        {
            if (!sub_426490(v5, &v1, 10, &v0, &g_42962f))
            {
                idx = sub_40d4d0(a0, v5);
                *((unsigned int *)&idx->padding_28[16]) = 2;
                idx->field_3c = v9;
                idx->field_40 = v0;
                *((unsigned short *)&idx->padding_0[26]) = 1;
                idx->field_24 = 1008981770;
                *(a2) = *(a2) + 1;
                return 1;
            }
            v2 = v8;
            v3 = 0;
            error(1, 0, dcgettext(NULL, "Invalid argument `%s%s' to -size", 5));
        }
        else if (!sub_426490(v5 + 1, &v1, 10, &v0, &g_42962f))
        {
            idx = sub_40d4d0(a0, v5);
            v13 = v0;
            v14 = 1063675494;
            idx->field_3c = v9;
            *((unsigned int *)&idx->padding_28[16]) = 1;
            if (v13 * v9 <= 0x5000)
            {
                idx->field_40 = v13;
                *((unsigned short *)&idx->padding_0[26]) = 1;
                idx->field_24 = 1036831949;
                *(a2) = *(a2) + 1;
                return 1;
            }
        }
        else
        {
            v2 = v8;
            v3 = 0;
            error(1, 0, dcgettext(NULL, "Invalid argument `%s%s' to -size", 5));
        }
        goto LABEL_41180e;
    }
    return 0;
}



// Function: parse_samefile @ 0x11a40
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
    char padding_0[36];
    unsigned int field_24;
    char padding_28[16];
    unsigned long long field_38;
    unsigned long long field_40;
    int field_48;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_438668;
extern struct_0 *g_438670;
extern char g_438678;

unsigned int parse_samefile(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rdx
    unsigned long v21;  // r14
    unsigned int v22;  // eax
    int v23;  // ebp
    struct_1 *idx;  // rax
    int *err;  // rax
    char v0;  // [bp-0x168], Other Possible Types: unsigned long long
    unsigned long v1;  // [bp-0x160]
    uint128_t v2;  // [bp-0x158]
    unsigned int v3;  // [bp-0x150]
    int v4;  // [bp-0x148]
    int v5;  // [bp-0x138]
    int v6;  // [bp-0x128]
    uint128_t v7;  // [bp-0x118]
    uint128_t v8;  // [bp-0x108]
    uint128_t v9;  // [bp-0xf8]
    uint128_t v10;  // [bp-0xe8]
    stat v11;  // [bp-0xd8]
    int v12;  // [bp-0xd0]
    uint128_t v13;  // [bp-0xc8]
    int v14;  // [bp-0xb8]
    uint128_t v15;  // [bp-0x88]
    uint128_t v16;  // [bp-0x78]
    uint128_t v17;  // [bp-0x68]
    uint128_t v18;  // [bp-0x58]

    sub_40d810(&v0);
    if (!a1)
        return 0;
    index = *(a2);
    v21 = *((long long *)(a1 + index * 8));
    if (v21)
    {
        *(a2) = (unsigned int)index + 1;
        v22 = g_438670(v21, &v0);
        if (v22)
        {
            err = __errno_location();
            sub_40e920(*(err), v21); /* do not return */
        }
        sub_40d810(&v11);
        if (!g_438668)
        {
            if (g_438678)
            {
                v22 = 0x20000;
            }
            else if (((unsigned short)v3 & 0xf000) == 0xa000)
            {
                idx = sub_40d4d0(a0, v21);
                idx->field_48 = -0x2;
                idx->field_38 = v1;
                idx->field_24 = 1008981770;
                idx->field_40 = v0;
                *((unsigned short *)&idx->padding_0[26]) = 1;
                return 1;
            }
        }
        v23 = sub_4160b0(v21, v22);
        if (v23 >= 0)
        {
            if (fstat(v23, &v11))
            {
                err = __errno_location();
                sub_40e920(*(err), v21); /* do not return */
            }
            else if (g_438670(v21, &v0))
            {
                err = __errno_location();
                sub_40e920(*(err), v21); /* do not return */
            }
            else if (!(!g_438668 && !g_438678))
            {
                memcpy(&v0, &v11, 16);
                v2 = v13;
                v4 = (int)*((uint128_t *)&v14);
                v5 = (int)*((uint128_t *)(&v14 + 16));
                v6 = (int)*((uint128_t *)(&v14 + 32));
                v7 = v15;
                v8 = v16;
                v9 = v17;
                v10 = v18;
            }
            else if (((unsigned short)v3 & 0xf000) == 0xa000 || v0 != *((long long *)&v11.st_mode) || v1 != *((unsigned long long *)&v12))
            {
                close(v23);
                v23 = -0x1;
            }
        }
        idx = sub_40d4d0(a0, v21);
        idx->field_48 = v23;
        idx->field_38 = v1;
        idx->field_24 = 1008981770;
        idx->field_40 = v0;
        *((unsigned short *)&idx->padding_0[26]) = 1;
        return 1;
    }
    return 0;
}



// Function: open_output_file @ 0x11c90
extern unsigned long long stderr;
extern unsigned long long stdout;
extern long long *g_438610;

unsigned int open_output_file(unsigned long a0, void* idx)
{
    void* v1;  // rax
    void* v2;  // rdi
    int *err;  // rax
    unsigned long v4;  // rax
    unsigned long v5;  // rax
    unsigned int v6;  // edi
    unsigned int v7;  // eax
    unsigned long long v8;  // rax

    *((unsigned long long *)idx) = 0;
    *((unsigned long long *)&idx[32]) = sub_423640(0);
    if (!strcmp(a0, "/dev/stderr"))
    {
        *((unsigned long long *)&idx[8]) = stderr;
        v5 = dcgettext(NULL, "standard error", 5);
        v2 = (long long)idx[8];
        *((unsigned long *)&idx[16]) = v5;
    }
    else if (strcmp(a0, "/dev/stdout"))
    {
        v1 = sub_40eb40(g_438610, a0);
        *((unsigned long *)&idx[16]) = a0;
        *((void* *)&idx[8]) = v1;
        v2 = v1;
        if (!v2)
        {
            err = __errno_location();
            sub_40e9b0(*(err), a0); /* do not return */
        }
    }
    else
    {
        *((unsigned long long *)&idx[8]) = stdout;
        v4 = dcgettext(NULL, "standard output", 5);
        v2 = (long long)idx[8];
        *((unsigned long *)&idx[16]) = v4;
    }
    v6 = fileno(v2);
    v7 = 0;
    if (v6 != 0xffffffff)
    {
        v8 = isatty(v6);
        v7 = _INSERT(v8, 0, (unsigned int)v8);
    }
    *((char *)&idx[24]) = v7;
    return v7;
}



// Function: parse_printf @ 0x11d80
unsigned int parse_printf(long long a0, unsigned long long a1, int *a2)
{
    unsigned long idx;  // rax
    unsigned long v3;  // r12
    unsigned int v4;  // ebp
    unsigned int v5;  // eax
    char v0;  // [bp-0x58]

    if (!a1)
        return 0;
    idx = *(a2);
    v3 = *((long long *)(a1 + idx * 8));
    v4 = idx;
    if (!v3)
        return 0;
    *(a2) = v4 + 1;
    sub_411c90("/dev/stdout", &v0);
    v5 = (unsigned long long)sub_410380(&v0, a0, v3);
    if (!(char)v5)
    {
        *(a2) = v4;
        return v5;
    }
    return v5;
}



// Function: parse_print @ 0x11e20
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

unsigned int parse_print(unsigned long long *a0)
{
    struct_0 *v1;  // rax

    v1 = (unsigned long long)sub_40d520(a0);
    v1->field_18 = 0x101;
    sub_411c90("/dev/stdout", v1 + 2);
    return 1;
}



// Function: parse_fprint0 @ 0x11e50
unsigned int parse_fprint0(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (!a1)
        return 0;
    v0 = v2;
    return sub_411e5e();
}



// Function: parse_fls @ 0x11ec0
unsigned int parse_fls(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (!a1)
        return 0;
    v0 = v2;
    return sub_411ece();
}



// Function: parse_fprintf @ 0x11f30
unsigned int parse_fprintf(long long a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v2;  // ebp
    unsigned long idx;  // rax
    unsigned long v4;  // rdi
    unsigned long v5;  // r13
    unsigned int v6;  // eax
    char v0;  // [bp-0x58]

    v2 = *(a2);
    if (a1)
    {
        idx = v2;
        v4 = *((long long *)(a1 + idx * 8));
        if (v4)
        {
            v5 = *((long long *)(a1 + idx * 8 + 8));
            if (v5)
            {
                *(a2) = v2 + 2;
                sub_411c90(v4, &v0);
                v2 = *(a2);
                v6 = (unsigned long long)sub_410380(&v0, a0, v5);
                if ((char)v6)
                    return v6;
            }
        }
    }
    *(a2) = v2;
    return 0;
}



// Function: parse_fprint @ 0x11fd0
unsigned int parse_fprint(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (!a1)
        return 0;
    v0 = v2;
    return sub_411fde();
}



// Function: parse_perm @ 0x12040
unsigned int parse_perm(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!a1)
        return 0;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    return sub_412057();
}



// Function: parse_okdir @ 0x12920
void parse_okdir(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: parse_or @ 0x129a0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_or(void)
{
    struct_0 *idx;  // rax

    idx = sub_40b990();
    idx->field_10 = 8589934595;
    *((void* *)&idx->padding_0[0]) = sub_408940;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_and @ 0x129e0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_and(void)
{
    struct_0 *idx;  // rax

    idx = sub_40b990();
    idx->field_10 = 12884901891;
    *((void* *)&idx->padding_0[0]) = sub_408990;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_comma @ 0x12a20
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
    char padding_1c[8];
    unsigned int field_24;
} struct_0;

unsigned int parse_comma(void)
{
    struct_0 *idx;  // rax

    idx = sub_40b990();
    idx->field_10 = 4294967299;
    *((void* *)&idx->padding_0[0]) = sub_408ca0;
    idx->field_1a = 0;
    idx->field_24 = 0x3f800000;
    return 1;
}



// Function: parse_closeparen @ 0x12a60
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_closeparen(void)
{
    struct_0 *idx;  // rax

    idx = sub_40b990();
    idx->field_10 = 5;
    *((void* *)&idx->padding_0[0]) = sub_408be0;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_group @ 0x12a90
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[16];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

extern unsigned int g_43868c;

unsigned int parse_group(long long a0, unsigned long long a1, int *a2)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    char *v5;  // r12
    group *v6;  // rbx
    unsigned int v7;  // ebx
    unsigned int v8;  // eax
    struct_0 *idx;  // rax
    unsigned int v10;  // ymm0
    unsigned long long v11;  // 4099
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!a1)
        return 0;
    v1 = v3;
    v0 = v4;
    v5 = *((long long *)(a1 + *(a2) * 8));
    if (!v5)
        return 0;
    *(a2) = *(a2) + 1;
    v6 = getgrnam(v5);
    endgrent();
    if (v6)
    {
        v7 = v6->gr_gid;
    }
    else
    {
        v8 = strspn(v5, "0123456789");
        if (v8)
        {
            if (!v5[v8])
            {
                v7 = sub_417410(v5);
            }
            else
            {
                sub_4239f0(1);
                sub_4239f0(0, g_43868c, v5);
                error(1, 0, dcgettext(NULL, "%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s", 5));
            }
        }
        else
        {
            if (*(v5))
            {
                sub_4239f0(0, g_43868c, v5);
                error(1, 0, dcgettext(NULL, "%s is not the name of an existing group", 5));
            }
            error(1, 0, dcgettext(NULL, "argument to -group is empty, but should be a group name", 5));
        }
    }
    idx = sub_40d4d0(a0, v5);
    v10 = 1065185444;
    v11 = idx->field_40;
    idx->field_38 = v7;
    if (v11 > 99)
        v10 = 1045220557;
    idx->field_24 = v10;
    return 1;
}



// Function: parse_negate @ 0x12c20
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_negate(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b9b0(a0, 0);
    idx->field_10 = 17179869186;
    *((void* *)&idx->padding_0[0]) = sub_409350;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_openparen @ 0x12c60
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_openparen(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b9b0(a0, 0);
    idx->field_10 = 4;
    *((void* *)&idx->padding_0[0]) = sub_409630;
    idx->field_1a = 0;
    return 1;
}



// Function: optionl_getfilecon @ 0x12dc0
unsigned int optionl_getfilecon(unsigned long a0, unsigned long a1, unsigned long a2)
{
    if (!sub_4242e0())
        return 0;
}



// Function: parse_name @ 0x12fc0
unsigned int parse_name(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rax
    unsigned long v2;  // r13
    unsigned int v3;  // r12d
    void* idx;  // rbx
    unsigned int v5;  // ymm0

    if (!a1)
        return 0;
    index = *(a2);
    v2 = *((long long *)(a1 + index * 8));
    if (!v2)
        return 0;
    *(a2) = (unsigned int)index + 1;
    sub_411220();
    v3 = sub_412f00("-name", "-wholename", v2);
    if (!(char)v3)
    {
        *(a2) = index;
        return v3;
    }
    idx = sub_40d4d0(a0, v2);
    *((unsigned short *)&idx[26]) = 0;
    *((unsigned long *)&idx[56]) = v2;
    v5 = 1061997773;
    if (!strpbrk(v2, "*?["))
        v5 = 1036831949;
    *((unsigned int *)&idx[36]) = v5;
    return v3;
}



// Function: parse_iname @ 0x13090
unsigned int parse_iname(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rdx
    unsigned long v2;  // r14
    unsigned int v3;  // eax
    void* idx;  // rbx
    unsigned int v5;  // ymm0

    sub_411220();
    if (!a1)
        return 0;
    index = *(a2);
    v2 = *((long long *)(a1 + index * 8));
    if (!v2)
        return 0;
    *(a2) = (unsigned int)index + 1;
    v3 = sub_412f00("-iname", "-iwholename", v2);
    if (!(char)v3)
        return 0;
    idx = sub_40d4d0(a0, v2);
    *((unsigned short *)&idx[26]) = 0;
    *((unsigned long *)&idx[56]) = v2;
    v5 = 1061997773;
    if (!strpbrk(v2, "*?["))
        v5 = 1036831949;
    *((unsigned int *)&idx[36]) = v5;
    return v3;
}



// Function: parse_regextype @ 0x13420
unsigned int parse_regextype(unsigned long a0, unsigned long long a1, int *a2)
{
    unsigned long v1;  // rcx

    if (!a1)
        return 0;
    v1 = *(a2);
    if (!*((long long *)(a1 + v1 * 8)))
        return 0;
    return sub_41343b(*((long long *)(a1 + v1 * 8)));
}



// Function: parse_maxdepth @ 0x13520
void parse_maxdepth(void)
{
}



// Function: parse_newerXY @ 0x13620
typedef struct struct_0 {
    char padding_0[26];
    char field_1a;
    char padding_1b[9];
    unsigned int field_24;
    char padding_28[16];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
    unsigned long long field_48;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern long long g_438638;
extern unsigned long long g_438648;
extern struct_1 *g_438670;
extern unsigned int g_43868c;

unsigned int parse_newerXY(long long a0, unsigned long long a1, int *a2)
{
    unsigned long idx;  // rax
    char *index;  // r14
    unsigned int v24;  // ymm0
    unsigned long long v25;  // rax
    unsigned int v26;  // r9d
    char v16;  // r15b
    struct_0 *idx1;  // r12
    unsigned int v18;  // eax
    unsigned long long v19;  // rax
    unsigned long long v20;  // rdx
    int *err;  // rax
    unsigned int v22;  // ymm1
    uint128_t v23;  // rcx
    unsigned long long v0;  // [bp-0xf0]
    char *v1;  // [bp-0xe8]
    char v2;  // [bp-0xde]
    char v3;  // [bp-0xdd]
    unsigned int v4;  // [bp-0xdc]
    char v5;  // [bp-0xd8]
    unsigned long long v6;  // [bp-0x90]
    unsigned long long v7;  // [bp-0x88]
    unsigned long long v8;  // [bp-0x80]
    unsigned long long v9;  // [bp-0x78]
    unsigned long long v10;  // [bp-0x70]
    unsigned long long v11;  // [bp-0x68]
    char v12[6];  // [bp-0x46]

    if (!a1)
        return 0;
    idx = *(a2);
    index = *((long long *)(a1 + idx * 8));
    v0 = idx * 8;
    if (index && strlen(index) == 8)
    {
        strncpy(v12, "aBcmt", 5);
        if (strncmp("-newer", index, 6))
            __assert_fail(); /* do not return */
        v16 = index[6];
        v2 = index[7];
        if (!(v16 != 66 && v2 != 66))
        {
            error(0, 0, dcgettext(NULL, "This system does not provide a way to find the birth time of a file.", 5));
            return 0;
        }
        else if (v16 != 116)
        {
            v3 = v16 == 66 | v2 == 66;
            v4 = v16;
            v1 = &v12[0];
            if (!strchr(v12, v4))
            {
                return v3;
            }
            else if (strchr(v1, v2))
            {
                if (*((long long *)(a1 + v0 + 8)))
                {
                    *(a2) = (unsigned int)idx + 1;
                    idx1 = sub_40d4d0(a0);
                    switch (v16)
                    {
                    case 99:
                        v18 = 2;
                        break;
                    case 109:
                        v18 = 3;
                        break;
                    case 97:
                        v18 = 0;
                        break;
                    default:
                        if (strchr(v1, v4))
                            __assert_fail(); /* do not return */
                        __assert_fail(); /* do not return */
                    }
                    idx1->field_38 = v18;
                    if (v2 != 116)
                    {
                        sub_40d810(&v5);
                        if (g_438670(*((long long *)(a1 + *(a2) * 8)), &v5))
                        {
                            err = __errno_location();
                            sub_40e920(*(err), *((long long *)(a1 + *(a2) * 8))); /* do not return */
                        }
                        switch (v2)
                        {
                        case 99:
                            v19 = v10;
                            v20 = v11;
                            break;
                        case 109:
                            v19 = v8;
                            v20 = v9;
                            break;
                        case 97:
                            v19 = v6;
                            v20 = v7;
                            break;
                        default:
                            __assert_fail(); /* do not return */
                        }
                        idx1->field_40 = v19;
                        idx1->field_48 = v20;
                    }
                    else if ((char)sub_421de0(&idx1->field_40, *((long long *)(a1 + *(a2) * 8)), &g_438638))
                    {
                        v19 = idx1->field_40;
                    }
                    else
                    {
                        sub_4239f0(0, g_43868c, *((long long *)(a1 + *(a2) * 8)));
                        error(1, 0, dcgettext(NULL, "I cannot figure out how to interpret %s as a date or time", 5));
                    }
                    *((unsigned int *)&idx1->padding_3c[0]) = 0;
                    v22 = 1008981770;
                    v23 = g_438648 - v19;
                    v24 = (unsigned long long)((int128_t)(v23 * 1749024623285053783) >> 77) - ((int128_t)(v23) >> 63);
                    if (((CmpF(1036831949, v24) & 69 | (char)((CmpF(1036831949, v24) & 69) >> 6)) & 1) == 1)
                    {
                        v22 = 1050253722;
                        if (((CmpF(0x3f800000, v24) & 69 | (char)((CmpF(0x3f800000, v24) & 69) >> 6)) & 1) == 1 && ((CmpF(v24, 0x42c80000) & 69 | (char)((CmpF(v24, 0x42c80000) & 69) >> 6)) & 1) == 1)
                            v22 = 1053273620;
                    }
                    v25 = *((long long *)&idx1->padding_0[0]);
                    idx1->field_24 = v22;
                    *(a2) = *(a2) + 1;
                    switch (v25)
                    {
                    case 0:
                        __assert_fail(); /* do not return */
                    case 4232192:
                        v26 = idx1->field_1a;
                        if (!(char)v26)
                            __assert_fail(); /* do not return */
                        return v26;
                    default:
                        __assert_fail(); /* do not return */
                    }
                }
                sub_4239f0(0, g_43868c, index);
                error(1, 0, dcgettext(NULL, "The %s test needs an argument", 5));
            }
            else
            {
                return v3;
            }
        }
        else
        {
            return _INSERT(0, 0, v16 == 66 | v2 == 66);
        }
    }
    return 0;
}



// Function: parse_ls @ 0x13b00
typedef struct struct_0 {
    char padding_0[24];
    unsigned short field_18;
    char padding_1a[10];
    unsigned int field_24;
} struct_0;

unsigned int parse_ls(unsigned long long *a0)
{
    struct_0 *idx;  // rbx

    idx = (unsigned long long)sub_40d520(a0);
    sub_411c90("/dev/stdout", &idx[1].padding_0[16]);
    idx->field_24 = 0x3f800000;
    idx->field_18 = 0x101;
    return 1;
}



// Function: parse_print0 @ 0x13b40
unsigned int parse_print0(void)
{
    unsigned int *idx;  // rbx

    idx = sub_40d4d0();
    sub_411c90("/dev/stdout", idx + 14);
    idx[6] = 0x101;
    idx[9] = 0x3f800000;
    return 1;
}



// Function: parse_follow @ 0x13b80
extern char g_40ddb0;
extern char g_43862c;
extern char g_438660;
extern unsigned int g_438668;
extern unsigned long long g_438670;
extern unsigned long long g_438680;

int parse_follow(void)
{
    g_43862c = 1;
    g_438680 = sub_412dc0;
    g_438668 = 1;
    if (!(g_438660 & 2))
        g_438670 = &g_40ddb0;
    else
        g_438670 = sub_40dec0;
    return sub_4133b0();
}



// Function: get_relative_timestamp @ 0x13be0
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    long long field_8;
    unsigned long long field_10;
} struct_0;

int get_relative_timestamp(char *a0, struct_0 *idx, long long a2, unsigned long a3, char *a4)
{
    char *iter;  // r12
    unsigned int v4;  // eax
    unsigned long long v5;  // xmm0lq
    int v6;  // xmm0
    int v7;  // xmm2
    long long v8;  // rax
    int v9;  // xmm1
    unsigned long v10;  // rdx
    char v0;  // [bp-0x50]
    unsigned long long v1;  // [bp-0x48]

    iter = a0;
    if (*(a0) == 43)
    {
        idx->field_4 = 1;
        iter += 1;
    }
    else if (*(a0) == 45)
    {
        idx->field_4 = 0;
        iter += 1;
    }
    else
    {
        idx->field_4 = 2;
    }
    v4 = sub_425f90(iter, 0, &v0, strtod);
    if (!(char)v4)
        return v4;
    modf(&v1);
    v6 = (int)(MulV(MulV(v5, v0), 0x41cdcd6500000000));
    if (((CmpF(0x41cdcd6500000000, *((unsigned long long *)&v6)) & 69 | (char)((CmpF(0x41cdcd6500000000, *((unsigned long long *)&v6)) & 69) >> 6)) & 1) == 1)
        __assert_fail(); /* do not return */
    v7 = (int)v1;
    v8 = *((unsigned long long *)&SubV(a2, v7));
    idx->field_8 = v8;
    if (a2 < v8 == !((CmpF(0, *((unsigned long long *)&v7)) & 69 | (CmpF(0, *((unsigned long long *)&v7)) & 69) >> 6) & 1))
    {
        v9 = (int)a3;
        v10 = *((unsigned long long *)&SubV(v9, v6));
        if (((CmpF(*((unsigned long long *)&v6), *((unsigned long long *)&v9)) & 69 | (char)((CmpF(*((unsigned long long *)&v6), *((unsigned long long *)&v9)) & 69) >> 6)) & 1) == 1)
        {
            idx->field_10 = v10;
            return v4;
        }
        idx->field_10 = v10 + 1000000000;
        idx->field_8 = v8 - 1;
        return v4;
    }
    error(1, 0, a4);
}



// Function: parse_used @ 0x13d80
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    long long field_8;
    unsigned long long field_10;
} struct_0;

unsigned int parse_used(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rcx
    unsigned long v4;  // rbp
    unsigned int v5;  // r12d
    struct_0 *idx;  // rcx
    unsigned int v7;  // ymm1
    unsigned int v8;  // ymm0
    struct_0 v0;  // [bp-0x38]
    unsigned long v1;  // [bp-0x28]

    if (!a1)
        return 0;
    index = *(a2);
    v4 = *((long long *)(a1 + index * 8));
    if (v4)
    {
        *(a2) = (unsigned int)index + 1;
        v5 = sub_413be0(v4, &v0, 0, 0, "arithmetic overflow while converting %s days to a number of seconds");
        if ((char)v5)
        {
            idx = sub_40d4d0(a0, v4);
            v7 = 1008981770;
            *((struct struct_0 *)&idx[2].field_8) = v0;
            *((unsigned long *)idx[3].padding_0) = v1;
            v8 = (v0.field_8 * 1749024623285053783 >> 77) - (v0.field_8 >> 63);
            if (((CmpF(1036831949, v8) & 69 | (char)((CmpF(1036831949, v8) & 69) >> 6)) & 1) == 1)
            {
                v7 = 1050253722;
                if (((CmpF(0x3f800000, v8) & 69 | (char)((CmpF(0x3f800000, v8) & 69) >> 6)) & 1) == 1 && ((CmpF(v8, 0x42c80000) & 69 | (char)((CmpF(v8, 0x42c80000) & 69) >> 6)) & 1) == 1)
                    v7 = 1053273620;
            }
            *((unsigned int *)((char *)&idx[1].field_8 + 4)) = v7;
            return v5;
        }
        error(1, 0, dcgettext(NULL, "Invalid argument %s to -used", 5));
    }
    return 0;
}



// Function: insert_num @ 0x13ed0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char padding_2[6];
    char *field_8;
    char padding_10[40];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
} struct_0;

extern long long g_42962f;
extern unsigned long long stderr;
extern char g_438660;

struct_0 * insert_num(unsigned long long a0, int *a1, long long a2)
{
    unsigned long idx;  // rdx
    struct_0 *v4;  // r12
    struct_0 *v5;  // rdi
    unsigned int v6;  // ebx
    struct_0 *v7;  // rax
    struct_0 *v8;  // r12
    char *v9;  // r8
    char *v10;  // rcx
    char v0;  // [bp-0x30]
    char v1;  // [bp-0x28]

    if (!a0)
        return NULL;
    idx = *(a1);
    v4 = *((long long *)(a0 + idx * 8));
    if (!v4)
        return v4;
    *(a1) = (unsigned int)idx + 1;
    if (v4->field_0 == 43)
    {
        v5 = &v4->field_1;
        v6 = 0;
    }
    else if (v4->field_0 != 45)
    {
        v5 = v4;
        v6 = 2;
    }
    else
    {
        v5 = &v4->field_1;
        v6 = 1;
    }
    if (!sub_426490(v5, &v1, 10, &v0, &g_42962f))
    {
        v7 = sub_40d4d0(a2, v4);
        v7->field_38 = v6;
        v8 = v7;
        *((unsigned long long *)&v8->field_40) = v0;
        if (!(g_438660 & 1))
            return v8;
        __fprintf_chk(stderr, 0x1, "inserting %s\n", v8->field_8);
        v9 = " >";
        v10 = "gt";
        if (v6)
        {
            v9 = " =";
            if (v6 == 1)
                v9 = " <";
            v10 = "eq";
            if (v6 == 1)
                v10 = "lt";
        }
        __fprintf_chk(stderr, 0x1, "    type: %s    %s  ", v10, v9);
        __fprintf_chk(stderr, 0x1, "%lu\n", *((long long *)&v8->field_40));
        return v8;
    }
    return NULL;
}



// Function: parse_links @ 0x14060
typedef struct struct_0 {
    char field_0;
    char field_1;
    char padding_2[6];
    char *field_8;
    char padding_10[40];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
} struct_0;

unsigned int parse_links(long long a0, unsigned long long a1, unsigned int *a2)
{
    struct_0 *idx;  // rax

    idx = sub_413ed0(a1, a2, a0);
    if (!idx)
    {
        *(a2) = *(a2) - 1;
        return 0;
    }
    else if (*((long long *)&idx->field_40) != 1)
    {
        if (*((long long *)&idx->field_40) != 2)
            *((unsigned int *)&idx->padding_10[20]) = 981668463;
        else
            *((unsigned int *)&idx->padding_10[20]) = 1008981770;
        return 1;
    }
    else
    {
        *((unsigned int *)&idx->padding_10[20]) = 1065185444;
        return 1;
    }
}



// Function: parse_inum @ 0x140d0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char padding_2[6];
    char *field_8;
    char padding_10[40];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
} struct_0;

unsigned int parse_inum(long long a0, unsigned long long a1, unsigned int *a2)
{
    struct_0 *idx;  // rax

    idx = sub_413ed0(a1, a2, a0);
    if (!idx)
    {
        *(a2) = *(a2) - 1;
        return 0;
    }
    *((unsigned int *)&idx->padding_10[20]) = 897988541;
    *((unsigned short *)&idx->padding_10[10]) = 0;
    idx->padding_10[12] = 1;
    return 1;
}



// Function: parse_newer @ 0x141e0
typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[16];
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_438648;
extern struct_0 *g_438670;

unsigned int parse_newer(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rdx
    unsigned long v5;  // r13
    struct_1 *idx;  // rsi
    unsigned int v7;  // ymm2
    uint128_t v8;  // rcx
    unsigned int v9;  // ymm0
    int *err;  // rax
    char v0;  // [bp-0xc8]
    unsigned long v1;  // [bp-0x70]
    unsigned long v2;  // [bp-0x68]

    sub_40d810(&v0);
    if (!a1)
        return 0;
    index = *(a2);
    v5 = *((long long *)(a1 + index * 8));
    if (v5)
    {
        *(a2) = (unsigned int)index + 1;
        if (g_438670(v5, &v0))
        {
            err = __errno_location();
            sub_40e920(*(err), v5); /* do not return */
        }
        idx = sub_40d4d0(a0, v5);
        idx->field_48 = v2;
        v7 = 1008981770;
        v8 = g_438648 - v1;
        idx->field_40 = v1;
        idx->field_38 = 3;
        v9 = (unsigned long long)((int128_t)(v8 * 1749024623285053783) >> 77) - ((int128_t)(v8) >> 63);
        if (((CmpF(1036831949, v9) & 69 | (char)((CmpF(1036831949, v9) & 69) >> 6)) & 1) == 1)
        {
            v7 = 1050253722;
            if (((CmpF(0x3f800000, v9) & 69 | (char)((CmpF(0x3f800000, v9) & 69) >> 6)) & 1) == 1 && ((CmpF(v9, 0x42c80000) & 69 | (char)((CmpF(v9, 0x42c80000) & 69) >> 6)) & 1) == 1)
                v7 = 1053273620;
        }
        idx->field_24 = v7;
        return 1;
    }
    return 0;
}



// Function: parse_anewer @ 0x14340
typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[16];
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_438648;
extern struct_0 *g_438670;

unsigned int parse_anewer(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rdx
    unsigned long v5;  // r13
    struct_1 *idx;  // rsi
    unsigned int v7;  // ymm2
    uint128_t v8;  // rcx
    unsigned int v9;  // ymm0
    int *err;  // rax
    char v0;  // [bp-0xc8]
    unsigned long v1;  // [bp-0x70]
    unsigned long v2;  // [bp-0x68]

    sub_40d810(&v0);
    if (!a1)
        return 0;
    index = *(a2);
    v5 = *((long long *)(a1 + index * 8));
    if (v5)
    {
        *(a2) = (unsigned int)index + 1;
        if (g_438670(v5, &v0))
        {
            err = __errno_location();
            sub_40e920(*(err), v5); /* do not return */
        }
        idx = sub_40d4d0(a0, v5);
        idx->field_48 = v2;
        v7 = 1008981770;
        v8 = g_438648 - v1;
        idx->field_40 = v1;
        idx->field_38 = 0;
        v9 = (unsigned long long)((int128_t)(v8 * 1749024623285053783) >> 77) - ((int128_t)(v8) >> 63);
        if (((CmpF(1036831949, v9) & 69 | (char)((CmpF(1036831949, v9) & 69) >> 6)) & 1) == 1)
        {
            v7 = 1050253722;
            if (((CmpF(0x3f800000, v9) & 69 | (char)((CmpF(0x3f800000, v9) & 69) >> 6)) & 1) == 1 && ((CmpF(v9, 0x42c80000) & 69 | (char)((CmpF(v9, 0x42c80000) & 69) >> 6)) & 1) == 1)
                v7 = 1053273620;
        }
        idx->field_24 = v7;
        return 1;
    }
    return 0;
}



// Function: parse_cnewer @ 0x144a0
typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[16];
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_438648;
extern struct_0 *g_438670;

unsigned int parse_cnewer(long long a0, unsigned long long a1, int *a2)
{
    unsigned long index;  // rdx
    unsigned long v5;  // r13
    struct_1 *idx;  // rsi
    unsigned int v7;  // ymm2
    uint128_t v8;  // rcx
    unsigned int v9;  // ymm0
    int *err;  // rax
    char v0;  // [bp-0xc8]
    unsigned long v1;  // [bp-0x70]
    unsigned long v2;  // [bp-0x68]

    sub_40d810(&v0);
    if (!a1)
        return 0;
    index = *(a2);
    v5 = *((long long *)(a1 + index * 8));
    if (v5)
    {
        *(a2) = (unsigned int)index + 1;
        if (g_438670(v5, &v0))
        {
            err = __errno_location();
            sub_40e920(*(err), v5); /* do not return */
        }
        idx = sub_40d4d0(a0, v5);
        idx->field_48 = v2;
        v7 = 1008981770;
        v8 = g_438648 - v1;
        idx->field_40 = v1;
        idx->field_38 = 2;
        v9 = (unsigned long long)((int128_t)(v8 * 1749024623285053783) >> 77) - ((int128_t)(v8) >> 63);
        if (((CmpF(1036831949, v9) & 69 | (char)((CmpF(1036831949, v9) & 69) >> 6)) & 1) == 1)
        {
            v7 = 1050253722;
            if (((CmpF(0x3f800000, v9) & 69 | (char)((CmpF(0x3f800000, v9) & 69) >> 6)) & 1) == 1 && ((CmpF(v9, 0x42c80000) & 69 | (char)((CmpF(v9, 0x42c80000) & 69) >> 6)) & 1) == 1)
                v7 = 1053273620;
        }
        idx->field_24 = v7;
        return 1;
    }
    return 0;
}



// Function: parse_mmin @ 0x14750
void parse_mmin(void)
{
}



// Function: parse_time @ 0x14780
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    long long field_8;
    unsigned long long field_10;
} struct_0;

extern unsigned long long stderr;
extern long long g_438648;
extern unsigned int g_438650;
extern char g_438660;

unsigned int parse_time(long long a0, unsigned long long a1, int *a2)
{
    char *v4;  // rax
    unsigned long index;  // rax
    unsigned int v14;  // edi
    char *t;  // rax
    char *t1;  // rax
    char *t2;  // rax
    char *v6;  // r14
    long long v7;  // rdx
    unsigned int v8;  // eax
    struct_0 *idx;  // rbx
    long long v10;  // rcx
    unsigned int v11;  // ymm1
    unsigned int v12;  // ymm0
    unsigned int v13;  // eax
    int v0[2];  // [bp-0x60], Other Possible Types: unsigned long
    int v1;  // [bp-0x58]
    unsigned long v2;  // [bp-0x48]

    v4 = dcgettext(NULL, "arithmetic overflow while converting %s days to a number of seconds", 5);
    if (!a1)
        return 0;
    index = *(a2);
    v6 = *((long long *)(a1 + index * 8));
    if (!v6)
        return 0;
    v7 = g_438648;
    *(a2) = (unsigned int)index + 1;
    if (*(v6) == 45)
        v7 = g_438648 + 86399;
    v8 = sub_413be0(v6, &v1, v7, *((long long *)&g_438650), v4);
    if (!(char)v8)
    {
        *(a2) = index;
        return v8;
    }
    idx = sub_40d4d0(a0, v6);
    v10 = g_438648 - *((unsigned long long *)(&v1 + 8));
    *((uint128_t *)&idx[2].field_8) = (uint128_t)v1;
    v11 = 1008981770;
    *((unsigned long *)idx[3].padding_0) = v2;
    v12 = (v10 * 1749024623285053783 >> 77) - (v10 >> 63);
    if (((CmpF(1036831949, v12) & 69 | (char)((CmpF(1036831949, v12) & 69) >> 6)) & 1) == 1)
    {
        v11 = 1050253722;
        if (((CmpF(0x3f800000, v12) & 69 | (char)((CmpF(0x3f800000, v12) & 69) >> 6)) & 1) == 1 && ((CmpF(v12, 0x42c80000) & 69 | (char)((CmpF(v12, 0x42c80000) & 69) >> 6)) & 1) == 1)
            v11 = 1053273620;
    }
    *((unsigned int *)((char *)&idx[1].field_8 + 4)) = v11;
    if (!(g_438660 & 1))
        return v8;
    __fprintf_chk(stderr, 0x1, "inserting %s\n", idx->field_8);
    v13 = *((unsigned int *)(&v1 + 4));
    if (v13)
    {
        v14 = stderr;
        if (v13 != 1 && v13 == 2)
        {
            __fprintf_chk(stderr, 0x1, "    type: %s    %s  ", "eq", ">=");
            v0 = (int (32 bits)[2])idx[2].field_10;
            t = ctime(v0);
            __fprintf_chk(stderr, 0x1, "%lu %s", idx[2].field_10, t);
            v0 = idx[2].field_10 + 86400;
            t1 = ctime(&v0);
            __fprintf_chk(stderr, 0x1, "                 <  %lu %s", v0, t1);
            return v8;
        }
    }
    else
    {
        v14 = stderr;
    }
    __fprintf_chk(v14, 0x1, "    type: %s    %s  ");
    v0 = (int (32 bits)[2])idx[2].field_10;
    t2 = ctime(v0);
    __fprintf_chk(stderr, 0x1, "%lu %s", idx[2].field_10, t2);
    return v8;
}



// Function: check_option_combinations @ 0x14a70
extern char g_438621;

unsigned long long check_option_combinations(unsigned long long *iter)
{
    unsigned long long v2;  // rax
    unsigned long long *v3;  // rdi
    unsigned long long v5;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!iter)
        return v5;
    v2 = 0;
    while (1)
    {
        if (*(iter) != sub_408d00)
        {
            v3 = iter[33];
            v2 = (*(iter) == sub_408600 ? (unsigned int)v2 | 2 : v2 & 0xffffffff);
            iter = v3;
            if (!iter)
                break;
        }
        else
        {
            iter = iter[33];
            v2 = (unsigned int)v2 | 1;
            if (!iter)
                break;
        }
    }
    if ((unsigned int)v2 != 3)
        return v2;
    if (g_438621)
        return v2;
    v0 = v2;
    error(1, 0, dcgettext(v3, "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.", 5));
}



// Function: parse_begin_user_args @ 0x14ba0
extern unsigned long long g_438e70;

void parse_begin_user_args(void)
{
    g_438e70 = 0;
    return;
}



// Function: parse_end_user_args @ 0x14bb0
void parse_end_user_args(void)
{
    return;
}



// Function: find_parser @ 0x14bc0
extern char *g_436d88;

unsigned long long find_parser(char *a0)
{
    char **v1;  // rbx
    char *v2;  // rdi
    char **v3;  // rbx

    if (strncmp("-newer", a0, 6) || strlen(a0) != 8)
    {
        v1 = &g_436d88;
        v2 = "!";
        while (1)
        {
            v3 = v1;
            if (!strcmp(v2, &a0[*(a0) == 45]))
                break;
            v2 = *(v3);
            v1 = v3 + 4;
            if (!*(v3))
                return 0;
        }
    }
}



// Function: parse_fstype @ 0x14c70
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

extern unsigned long long stderr;
extern char g_438660;
extern unsigned short g_438688;

unsigned int parse_fstype(long long a0, unsigned long long a1, int *a2)
{
    unsigned long idx;  // rcx
    unsigned long v3;  // rbp
    unsigned long v4;  // 4123
    unsigned int *index;  // rax
    void* idx1;  // rax
    unsigned int v8;  // ymm0
    stat v0;  // [bp-0xc8]

    if (!a1)
        return 0;
    idx = *(a2);
    v3 = *((long long *)(a1 + idx * 8));
    if (!v3)
        return 0;
    v4 = g_438688;
    *(a2) = (unsigned int)idx + 1;
    if (v4 > 1 && !(char)(unsigned long long)sub_410a50(v3))
    {
        if (g_438660 & 8)
            __fprintf_chk(stderr, 0x1, "-fstype %s can never succeed, substituting -false\n", v3);
        index = (unsigned long long)sub_40d520(sub_414bc0("false"));
        index[6] = 0;
        index[9] = 0;
        return 1;
    }
    idx1 = sub_40d4d0(a0, v3);
    *((unsigned long *)&idx1[56]) = v3;
    v8 = 0x3f800000;
    if (!stat("/", &v0))
    {
        v8 = 1060320051;
        if (strcmp(v3, (unsigned long long)sub_4108e0(&v0)))
            v8 = 1050253722;
    }
    *((unsigned int *)&idx1[36]) = v8;
    return 1;
}



// Function: parse_false @ 0x14df0
unsigned int parse_false(void)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_40d520(sub_414bc0("false"));
    idx[6] = 0;
    idx[9] = 0;
    return 1;
}



// Function: main @ 0x7370
extern char g_42f827;
extern unsigned long long stderr;
extern unsigned int g_43e5f8;
extern unsigned int g_43e604;
extern char g_43e608;
extern unsigned long long g_43e610;
extern char g_43e618;
extern long long g_43e620;
extern int g_43e648;
extern unsigned long long g_43e660;
extern unsigned long long g_43e670;

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
    sub_422cb0(v1);
    sub_40d970(v2, v3, v4, v5, v6, v7);
    g_43e618 = 0;
    g_43e604 = 0;
    g_43e608 = 0;
    g_43e5f8 = 0xffffff9c;
    if ((char)sub_415dd0(v2, v3, v4, v5, v6, v7))
        sub_415c30();
    g_43e610 = sub_40e4d0("w");
    if (!g_43e610)
    {
        v13 = dcgettext(NULL, "Failed to initialize shared-file hash table", 5);
        err1 = __errno_location();
        error(1, *(err1), v13);
    }
    sub_40de90(&g_43e620);
    setlocale(6, &g_42f827);
    bindtextdomain("findutils", "/usr/local/share/locale");
    textdomain("findutils");
    if (!sub_42a3c0(sub_4175a0))
    {
        v8 = sub_40dd00(a0, a1);
        if ((char)g_43e660 & 2)
            g_43e670 = sub_40db70;
        if ((char)g_43e660 & 128)
        {
            t = ctime(&g_43e648);
            __fprintf_chk(stderr, 0x1, "cur_day_start = %s", t);
        }
        v10 = sub_40c6f0(a0, a1, v8);
        if (!(char)sub_408210(a0 - v8 & 0xffffffff, &a1[v8]))
            return g_43e604;
        sub_409ee0(v10);
        sub_40e360(v2, v3, v4, v5, v6, v7);
        return g_43e604;
    }
    v11 = dcgettext(NULL, "The atexit library function failed", 5);
    err = __errno_location();
    error(1, *(err), v11);
}



// Function: symlink_loop @ 0x7640
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_43e670;

unsigned int symlink_loop(void)
{
    unsigned int *err;  // rax

    if (!g_43e670())
        return 0;
    err = __errno_location();
    return _INSERT(err, 0, *(err) == 40);
}



// Function: inside_dir @ 0x76a0
extern unsigned int g_43e010;
extern char g_43e019;
extern unsigned int g_43e5f8;

long long inside_dir(int a0)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax

    if (!(g_43e019 & 2))
        return v2;
    if (a0 != -0x64 && a0 < 0)
        __assert_fail(); /* do not return */
    v1 = g_43e010;
    g_43e5f8 = a0;
    if ((unsigned int)v1 >= 0)
    {
        return v1;
    }
    else if (a0 == -0x64)
    {
        g_43e010 = 0xffffff9c;
        return v1;
    }
    else
    {
        g_43e010 = sub_417560();
        return g_43e010;
    }
}



// Function: left_dir @ 0x7720
extern int g_43e010;
extern char g_43e019;

int left_dir(void)
{
    int v1;  // eax
    int v2;  // eax
    int v3;  // eax

    if (!(g_43e019 & 2))
    {
        return v3;
    }
    else if (g_43e010 < 0)
    {
        return v1;
    }
    else
    {
        v2 = close(g_43e010);
        g_43e010 = 0xffffffff;
        return v2;
    }
}



// Function: get_fts_info_name @ 0x7750
extern char g_43e598;

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
        __sprintf_chk(&g_43e598, 0x1);
        return &g_43e598;
    }
}



// Function: visit @ 0x7880
typedef struct struct_1 {
    char padding_0[44];
    unsigned int field_2c;
} struct_1;

typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
    long long field_38;
    char padding_40[40];
    unsigned short field_68;
} struct_0;

extern char g_43e5e4;
extern unsigned long long g_43e5f0;
extern unsigned int g_43e5f8;
extern char g_43e600;

int visit(struct_1 *a0, struct_0 *a1, long long a2)
{
    unsigned long v1;  // rax
    long long v2;  // rax
    unsigned int v3;  // eax

    v1 = a1->field_30;
    g_43e5e4 = a1->field_68 - 10 > 1;
    g_43e5f0 = v1;
    g_43e5f8 = a0->field_2c;
    v2 = sub_40b5e0();
    v3 = sub_40e2a0(a1->field_38, a2, v2);
    if (g_43e600)
        return sub_41a190(a0, a1, 4);
    return v3;
}



// Function: issue_loop_warning @ 0x7950
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[48];
    long long field_38;
    char padding_40[72];
    unsigned int field_88;
} struct_0;

typedef struct struct_1 {
    char padding_0[56];
    long long field_38;
    char padding_40[8];
    long long field_48;
} struct_1;

extern unsigned int g_43e68c;

void issue_loop_warning(struct_0 *a0)
{
    if (((unsigned short)a0->field_88 & 0xf000) != 0xa000)
    {
        sub_407900(a0->field_0->field_38, a0->field_0->field_48, g_43e68c);
        sub_40dfb0(0, a0->field_38);
        error(0, 0, dcgettext(NULL, "File system loop detected; %s is part of the same file system loop as %s.", 5));
    }
    else
    {
        sub_40dfb0(0, a0->field_38);
        error(0, 0, dcgettext(NULL, "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.", 5));
    }
    return;
}



// Function: consider_visiting @ 0x7a00
typedef struct struct_0 {
    char padding_0[44];
    int field_2c;
} struct_0;

extern void g_43e014;
extern unsigned long long stderr;
extern unsigned int g_43e5e0;
extern void g_43e5e4;
extern char g_43e5e5;
extern unsigned int g_43e5e8;
extern char g_43e600;
extern unsigned int g_43e604;
extern char g_43e620;
extern int g_43e624;
extern int g_43e628;
extern void g_43e660;
extern unsigned int g_43e68c;

void consider_visiting(struct_0 *a0, void* idx)
{
    unsigned long long v14;  // rax
    char *v15;  // r14
    int v24;  // xmm7
    int v25;  // xmm0
    unsigned int v27;  // edx
    unsigned short v28;  // ax
    unsigned int v29;  // edi
    unsigned long v30;  // rdx
    int v31;  // r13d
    int v32;  // ebx
    unsigned int v33;  // eax
    unsigned int v16;  // eax
    unsigned long long v34;  // rdx
    unsigned long long v35;  // rax
    unsigned int v36;  // eax
    char *v37;  // rax
    char *v17;  // rax
    unsigned short v18;  // ax
    int v19;  // xmm0
    int v20;  // xmm1
    int v21;  // xmm4
    int v22;  // xmm5
    int v23;  // xmm6
    unsigned long v0;  // [bp-0x108], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xf0], Other Possible Types: unsigned long
    unsigned int v2;  // [bp-0xdc]
    int v3;  // [bp-0xd8]
    unsigned long v4;  // [bp-0xd0]
    int v5;  // [bp-0xc8]
    int v6;  // [bp-0xb8]
    int v7;  // [bp-0xa8]
    int v8;  // [bp-0x98]
    int v9;  // [bp-0x88]
    int v10;  // [bp-0x78]
    int v11;  // [bp-0x68]
    int v12;  // [bp-0x58]

    if (g_43e660 & 4)
    {
        v14 = sub_424e50(2, g_43e68c, (long long)idx[48]);
        v15 = sub_424e50(1, g_43e68c, (long long)idx[56]);
        v1 = (long long)idx[88];
        v16 = (unsigned int)sub_407750((short)idx[104]);
        v17 = sub_424e50(0, g_43e68c, (long long)idx[56]);
        v0 = v14;
        __fprintf_chk(stderr, 0x1, "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n", v17, v16, (unsigned int)v1, *((int *)&g_43e014), v15);
    }
    if ((short)idx[104] == 6 || (long long)idx[88] > *((int *)&g_43e014) || !(long long)idx[88])
        sub_407720();
    sub_4076a0(a0->field_2c);
    v4 = (long long)idx[120];
    v18 = (short)idx[104];
    *((int *)&g_43e014) = (long long)idx[88];
    if (v18 == 7)
    {
        sub_40e060((int)idx[64], (long long)idx[56]);
        return;
    }
    if (v18 != 4)
    {
        if (v18 == 2)
        {
            sub_407950(idx);
            g_43e604 = 1;
            return;
        }
        if (v18 != 13)
        {
            if (v18 != 10)
                goto LABEL_407ab2;
            if (!*((int *)&g_43e014))
            {
                sub_40e060((int)idx[64], (long long)idx[56]);
                return;
            }
            if ((char)sub_407640())
            {
                sub_40e060(40);
                return;
            }
            sub_40e060((int)idx[64]);
        }
        else if ((char)sub_407640())
        {
            sub_40e060(40, (long long)idx[56]);
            return;
        }
    }
    else
    {
        sub_40e060((int)idx[64], (long long)idx[56]);
        if (g_43e620)
            return;
    }
    v18 = (short)idx[104];
LABEL_407ab2:
    if (v18 - 10 > 1)
    {
        v19 = (int)(int128_t)idx[112];
        v20 = (int)(int128_t)idx[128];
        *((unsigned short *)&g_43e5e4) = 0x101;
        v21 = (int)(int128_t)idx[176];
        v3 = v19;
        v22 = (int)(int128_t)idx[192];
        v23 = (int)(int128_t)idx[208];
        v5 = v20;
        v24 = (int)(int128_t)idx[224];
        v6 = (int)(int128_t)idx[144];
        v25 = (int)(int128_t)idx[240];
        v7 = (int)(int128_t)idx[160];
        v2 = *((unsigned int *)(&v5 + 8));
        g_43e5e8 = v2;
        v8 = v21;
        v9 = v22;
        v10 = v23;
        v11 = v24;
        v12 = v25;
        if (v2)
        {
            g_43e5e0 = (long long)idx[88];
LABEL_407b66:
            if (!(char)sub_40e150(&v2, (long long)idx[56], idx + 0x100, &v3, 0))
                return;
            v28 = (short)idx[104];
            v29 = v28;
            if ((v2 & 0xf000) == 0x4000)
                goto LABEL_407ba8;
            goto LABEL_407d4b;
        }
        else
        {
            sub_424e50(0, g_43e68c, (long long)idx[56]);
            error(0, 0, dcgettext(NULL, "WARNING: file %s appears to have mode 0000", 5));
            v27 = v2;
        }
    }
    else if (!g_43e5e4)
    {
        v27 = g_43e5e8;
        if (v18 != 11 && g_43e5e8)
            __assert_fail(); /* do not return */
        v2 = g_43e5e8;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
    g_43e5e0 = (long long)idx[88];
    if (v27)
        goto LABEL_407b66;
    v28 = (short)idx[104];
LABEL_407d4b:
    v29 = v28;
    if ((unsigned short)(v29 - 1) <= 1 || (unsigned short)v29 == 6)
    {
LABEL_407ba8:
        if (v28 == 11)
        {
            sub_41a190(a0, idx, 1);
            return;
        }
        v30 = g_43e624;
        v31 = 1;
        v32 = 0;
        if (g_43e624 < 0)
            goto LABEL_407bd4;
        goto LABEL_407bc5;
    }
    v30 = g_43e624;
    v31 = 0;
    if (g_43e624 >= 0)
    {
LABEL_407bc5:
        if ((long long)idx[88] < v30)
        {
            v29 = v28;
            v32 = 0;
        }
        else
        {
            sub_41a190(a0, idx, 4);
            v29 = (short)idx[104];
            v32 = g_43e624 < (long long)idx[88];
        }
LABEL_407bd4:
        v33 = *((long long *)&g_43e660);
        v34 = (int)*((long long *)&g_43e660) & 4;
        if ((unsigned short)v29 != 1)
        {
            if ((unsigned short)v29 != 6 || !g_43e620)
                goto LABEL_407bf5;
            if (!v34)
            {
                g_43e600 = 0;
                return;
            }
        }
        else
        {
            if (g_43e620)
                goto LABEL_407bf5;
            if (!v34)
                return;
        }
    }
    else
    {
        v33 = *((long long *)&g_43e660);
        v32 = 0;
LABEL_407bf5:
        v35 = v33 & 4;
        if ((long long)idx[88] >= g_43e628)
        {
            if (!v35)
                goto LABEL_407c12;
            goto LABEL_407d96;
        }
        else if (!v35)
        {
LABEL_407c26:
            if ((short)idx[104] != 6)
                return;
            g_43e600 = 0;
            return;
        }
    }
    v32 = 1;
LABEL_407d96:
    v1 = g_43e5e5;
    v36 = (unsigned int)sub_407750(v29);
    v37 = sub_424e50(0, g_43e68c, (long long)idx[56]);
    v0 = v1;
    __fprintf_chk(stderr, 0x1, "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n", v37, v36, v31, v32, (unsigned int)g_43e5e4);
LABEL_407c12:
    if (!v32)
        sub_407880(a0, idx, &v3);
    goto LABEL_407c26;
}



// Function: find @ 0x7fe0
extern unsigned int g_43e018;
extern char g_43e5e4;
extern char g_43e5e5;
extern unsigned int g_43e5e8;
extern unsigned int g_43e5fc;
extern unsigned int g_43e604;
extern char g_43e608;
extern char g_43e618;
extern char g_43e62d;
extern unsigned int g_43e668;

unsigned int find(unsigned long a0)
{
    unsigned int v4;  // esi
    unsigned int v5;  // r13d
    unsigned long v6;  // rbp
    unsigned int *err;  // rbx
    void* idx;  // rsi
    unsigned int v9;  // r14d
    unsigned int v10;  // eax
    char *v11;  // rdx
    char *v12;  // rax
    void* v0;  // [bp-0x50]
    unsigned long v1;  // [bp-0x48]
    unsigned long long v2;  // [bp-0x40]

    g_43e5fc = strlen(a0);
    sub_4076a0(-0x64);
    v1 = a0;
    v2 = 0;
    v4 = g_43e018;
    if (g_43e668 == 1)
    {
        v4 |= 3;
        g_43e018 = v4;
    }
    else if (g_43e668 == 2)
    {
        v4 |= 0x11;
        g_43e018 = v4;
    }
    else if (!g_43e668)
    {
        v4 |= 16;
        g_43e018 = v4;
    }
    if (g_43e62d)
        g_43e018 = v4 | 64;
    v5 = 0x80000000;
    v6 = sub_4195e0(&v1);
    err = __errno_location();
    if (!v6)
    {
        sub_40dfb0(0, a0);
        v12 = dcgettext(NULL, "cannot search %s", 5);
        error(0, *(err), v12);
        g_43e604 = 1;
        return 1;
    }
    while (1)
    {
        *(err) = 0;
        idx = sub_419b40(v6);
        if (!idx)
            break;
        v9 = (long long)idx[88];
        if (g_43e608 && v5 != (int)(long long)idx[88])
        {
            v0 = idx;
            sub_40d940();
            idx = v0;
            v9 = (int)idx[88];
        }
        v10 = (int)idx[0x88];
        v5 = v9;
        g_43e618 = 0;
        g_43e5e4 = 0;
        g_43e5e8 = v10;
        g_43e5e5 = g_43e5e8;
        sub_407a00(v6, idx);
    }
    if (!*(err))
    {
        if (!sub_419950(v6))
            return 1;
        v11 = dcgettext(NULL, "failed to restore working directory after searching %s", 5);
    }
    else
    {
        sub_40dfb0(0, a0);
        v11 = "failed to read file names from file system at or below %s";
    }
    error(0, *(err), v11);
    g_43e604 = 1;
    return 0;
}



// Function: process_all_startpoints @ 0x8210
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
extern unsigned int g_43e5fc;
extern unsigned int g_43e604;
extern unsigned long long g_43e690;
extern char g_43e698;

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

    if (!(a0 <= 0 || (char)sub_40dca0(*(a1), 1)))
    {
        fp = g_43e690;
        if (g_43e690)
        {
            sub_40dfb0(0, *(a1));
            error(0, 0, dcgettext(NULL, "extra operand %s", 5));
            dcgettext(NULL, "file operands cannot be combined with -files0-from", 5);
            error(1, 0, "%s");
        }
        v11 = sub_4171b0(a1);
    }
    else if (g_43e690)
    {
        if (strcmp(g_43e690, "-"))
        {
            sub_40dfb0(0, g_43e690);
            fp = fopen(g_43e690, "r");
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
            if (g_43e698 && !fstat(v8, &v2) && !fstat(0, &v3) && v2.st_ino == v3.st_ino && *((unsigned long long *)&v2) == *((unsigned long long *)&v3))
            {
                dcgettext(NULL, "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir", 5);
                error(1, 0, "%s: %s\n");
            }
            sub_4174f0(v8, 1);
        }
        else if (!g_43e698)
        {
            sub_40dfb0(0, dcgettext(NULL, "(standard input)", 5));
            fp = stdin;
        }
        else
        {
            dcgettext(NULL, "option -files0-from reading from standard input cannot be combined with -ok, -okdir", 5);
            error(1, 0, "%s\n");
        }
        v11 = sub_4171e0(fp);
    }
    else
    {
        v4 = 46;
        return sub_407fe0(&v4) & 0xffffffff;
    }
    if (v11)
    {
        v14 = 1;
        while (1)
        {
            v15 = sub_417220(v11, &v1);
            if (!v15)
            {
                if (v1 != 3)
                {
                    if (v1 == 4)
                    {
                        v16 = dcgettext(NULL, "%s: read error", 5);
                        err = __errno_location();
                        error(0, *(err), v16);
                        g_43e604 = 1;
                        sub_4172e0(v11);
                        return 0;
                    }
                    if (v1 != 2)
                        __assert_fail(); /* do not return */
LABEL_408316:
                    sub_4172e0(v11);
                    if (!(char)v14)
                        return v14;
                    if (g_43e690)
                    {
                        if (!ferror(fp) && !sub_417850(fp))
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
                if (!g_43e690 && (char)sub_40dca0(v15, 1))
                    goto LABEL_408316;
                g_43e5fc = strlen(v15);
                if ((char)sub_407fe0(v15))
                    continue;
                v14 = 0;
                goto LABEL_408316;
            }
            else
            {
                if (g_43e690)
                {
                    sub_4172c0(v11);
                    dcgettext(NULL, "invalid zero-length file name", 5);
                    error(0, 0, "%s:%lu: %s");
                }
                else
                {
                    sub_40dfb0(0, v15);
                    error(0, 2, "%s");
                }
                g_43e604 = 1;
                v14 = 0;
            }
        }
    }
    sub_426d50(); /* do not return */
}



// Function: is_fts_enabled @ 0x86d0
extern unsigned int g_43e018;

unsigned int is_fts_enabled(unsigned int *a0)
{
    *(a0) = g_43e018;
    return 1;
}



// Function: pred_prune @ 0x86f0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_43e5e4;
extern char g_43e600;
extern char g_43e620;

unsigned int pred_prune(unsigned long a0, struct_0 *a1)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!g_43e620)
    {
        return 1;
    }
    else if (!g_43e5e4)
    {
        v0 = v2;
        __assert_fail(); /* do not return */
    }
    else if (a1)
    {
        if (((unsigned short)a1->field_18 & 0xf000) != 0x4000)
            return 1;
        g_43e600 = 1;
        return 1;
    }
    else
    {
        return 1;
    }
}



// Function: ts_difference @ 0x8750
void ts_difference(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40875e();
    return;
}



// Function: compare_ts @ 0x8790
unsigned int compare_ts(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    if (a0 != a2)
    {
        return sub_40879c();
    }
    else if (a1 == a3)
    {
        return 0;
    }
    else
    {
        return sub_40879c();
    }
}



// Function: pred_and @ 0x8870
typedef struct struct_0 {
    char padding_0[272];
    unsigned long long field_110;
    long long field_118;
} struct_0;

void pred_and(long long a0, long long a1, struct_0 *a2)
{
    if (!a2->field_110)
    {
        sub_40e2a0(a0, a1, a2->field_118);
        return;
    }
    else if ((char)sub_40e2a0())
    {
        sub_40e2a0(a0, a1, a2->field_118);
        return;
    }
    else
    {
        return;
    }
}



// Function: pred_or @ 0x88c0
typedef struct struct_0 {
    char padding_0[272];
    unsigned long long field_110;
    long long field_118;
} struct_0;

void pred_or(long long a0, long long a1, struct_0 *a2)
{
    if (!a2->field_110)
    {
        sub_40e2a0(a0, a1, a2->field_118);
        return;
    }
    else if (!(char)sub_40e2a0())
    {
        sub_40e2a0(a0, a1, a2->field_118);
        return;
    }
    else
    {
        return;
    }
}



// Function: perform_delete @ 0x8910
extern long long g_43e5f0;
extern unsigned int g_43e5f8;

char perform_delete(unsigned int a0)
{
    return !unlinkat(g_43e5f8, g_43e5f0, a0);
}



// Function: is_ok @ 0x8990
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
    unsigned long v8;  // rax
    char *v9;  // rdi
    void* v10;  // rax
    char *v11;  // rax
    int *err;  // rax
    char *v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10], Other Possible Types: unsigned long long

    v2 = v4;
    sub_417bc0(stdout);
    v5 = dcgettext(NULL, "< %s ... %s > ? ", 5);
    if (__fprintf_chk(stderr, 0x1, v5) >= 0)
    {
        sub_417bc0(stderr);
        v6 = 0;
        v2 = *((long long *)(40 + v7));
        v0 = NULL;
        v1 = 0;
        v8 = getline(&v0, &v1, stdin);
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



// Function: blank_rtrim @ 0x8a40
char * blank_rtrim(char *a0, char *a1)
{
    unsigned long long v2;  // rbx
    unsigned int v3;  // ecx
    unsigned int v4;  // ecx
    char *v5;  // rdx
    char *v6;  // rdx
    unsigned long long v0;  // [bp-0x8]

    if (!a0)
        return NULL;
    v0 = v2;
    strcpy(a1, a0);
    v3 = strlen(a1);
    v4 = v3 - 1;
    if (v3 - 1 >= 0)
    {
        v5 = &a1[v4];
        do
        {
            v6 = v5;
            if (*(v6) != 32 && *(v6) != 9)
            {
                v6 = &a1[1 + v4];
                break;
            }
        } while ((v4 -= 1, v5 = v6 - 1, v4 != 0xffffffff));
    }
    else
    {
        v6 = &a1[v3];
    }
    *(v6) = 0;
    return a1;
}



// Function: pred_amin @ 0x8b70
void pred_amin(unsigned long a0, long long a1, unsigned long a2)
{
    sub_425c40(a1);
}



// Function: pred_anewer @ 0x8ba0
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

char pred_anewer(unsigned long a0, long long a1, struct_0 *a2)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // rdx

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    v1 = sub_425c40(a1);
    return sub_408790(v1, v2, *((long long *)&a2->field_40), *((long long *)&a2->field_48)) > 0;
}



// Function: pred_atime @ 0x8c00
void pred_atime(unsigned long a0, long long a1, unsigned long a2)
{
    sub_425c40(a1);
}



// Function: pred_cmin @ 0x8c40
void pred_cmin(unsigned long a0, long long a1, unsigned long a2)
{
    sub_425c50(a1);
}



// Function: pred_cnewer @ 0x8c70
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

char pred_cnewer(unsigned long a0, long long a1, struct_0 *a2)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // rdx

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    v1 = sub_425c50(a1);
    return sub_408790(v1, v2, *((long long *)&a2->field_40), *((long long *)&a2->field_48)) > 0;
}



// Function: pred_comma @ 0x8cd0
typedef struct struct_0 {
    char padding_0[272];
    unsigned long long field_110;
    long long field_118;
} struct_0;

void pred_comma(long long a0, long long a1, struct_0 *a2)
{
    if (a2->field_110)
        sub_40e2a0();
    sub_40e2a0(a0, a1, a2->field_118);
    return;
}



// Function: pred_ctime @ 0x8d10
void pred_ctime(unsigned long a0, long long a1, unsigned long a2)
{
    sub_425c50(a1);
}



// Function: pred_delete @ 0x8d40
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_43e5e4;
extern char *g_43e5f0;
extern unsigned int g_43e604;
extern char g_43e62e;

unsigned int pred_delete(long long a0, struct_0 *a1)
{
    unsigned int v1;  // ebx
    unsigned int v2;  // eax
    int *err;  // r13
    char *v4;  // rax

    if (!strcmp(g_43e5f0, "."))
        return 1;
    v1 = 0;
    if (g_43e5e4)
        v1 = ((a1->field_18 & 0xf000) == 0x4000) * 0x200;
    v2 = sub_408910(v1);
    if (!(char)v2)
    {
        err = __errno_location();
        if (*(err) == 2)
        {
            if (g_43e62e)
            {
                *(err) = 0;
                return g_43e62e;
            }
LABEL_408de0:
            sub_40dfb0(0, a0);
            v4 = dcgettext(NULL, "cannot delete %s", 5);
            error(0, *(err), v4);
            g_43e604 = 1;
            return v2;
        }
        else if (!(*(err) == 21 && !v1 && sub_408910(0x200)))
        {
            goto LABEL_408de0;
        }
    }
    return 1;
}



// Function: pred_empty @ 0x8e50
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

extern long long g_43e5f0;
extern unsigned int g_43e5f8;
extern unsigned int g_43e604;

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
        v4 = sub_41e690(g_43e5f8, g_43e5f0, 0x90900);
        if (v4 >= 0)
        {
            fp = fdopendir(v4);
            if (!fp)
            {
                sub_40dfb0(0, a0);
                error(0, *(v3), "%s");
                g_43e604 = 1;
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
                sub_40dfb0(0, a0);
                error(0, *(v3), "%s");
                g_43e604 = 1;
                closedir(fp);
                return 0;
            }
            else if (!closedir(fp))
            {
                return v7;
            }
        }
        sub_40dfb0(0, a0);
        error(0, *(v3), "%s");
        g_43e604 = 1;
        return 0;
    case 32768:
        return !a1->field_30;
    default:
        return !a1->field_30;
    }
}



// Function: pred_execdir @ 0x8ff0
void pred_execdir(void)
{
}



// Function: pred_false @ 0x9000
unsigned int pred_false(void)
{
    return 0;
}



// Function: pred_fls @ 0x9010
typedef struct struct_0 {
    char padding_0[40];
    char field_28;
    char padding_29[23];
    unsigned long long field_40;
} struct_0;

unsigned int pred_fls(unsigned long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long v0;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    v1 = a2->field_40;
    v0 = a2->field_28;
    sub_416170();
    return 1;
}



// Function: pred_fprint @ 0x9050
typedef struct struct_0 {
    char padding_0[64];
    long long field_40;
    char padding_48[8];
    char field_50;
    char padding_51[7];
    long long field_58;
} struct_0;

unsigned int pred_fprint(long long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long v1;  // r9

    v1 = a2->field_50;
    sub_416960(a2->field_40, a2->field_58, v1 & 0xff, "%s\n", a0, v1);
    return 1;
}



// Function: pred_fstype @ 0x90c0
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_fstype(long long a0, long long a1, struct_0 *a2)
{
    char *v1;  // rax

    v1 = sub_4104c0(a1, a0);
    return !strcmp(v1, a2->field_38);
}



// Function: pred_gid @ 0x90f0
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



// Function: pred_group @ 0x9140
void pred_group(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return;
}



// Function: pred_ilname @ 0x9150
void pred_ilname(unsigned long a0, unsigned long a1)
{
}



// Function: pred_iname @ 0x9170
void pred_iname(unsigned long a0, unsigned long a1, unsigned long a2)
{
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
void pred_lname(unsigned long a0, unsigned long a1)
{
}



// Function: pred_mmin @ 0x9280
void pred_mmin(unsigned long a0, long long a1, unsigned long a2)
{
    sub_425c60(a1);
}



// Function: pred_mtime @ 0x92b0
void pred_mtime(unsigned long a0, long long a1, unsigned long a2)
{
    sub_425c60(a1);
}



// Function: pred_name @ 0x92e0
void pred_name(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: pred_negate @ 0x92f0
int pred_negate(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return sub_40e2a0() ^ 1;
}



// Function: pred_newer @ 0x9310
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

char pred_newer(unsigned long a0, long long a1, struct_0 *a2)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // rdx

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    v1 = sub_425c60(a1);
    return sub_408790(v1, v2, *((long long *)&a2->field_40), *((long long *)&a2->field_48)) > 0;
}



// Function: pred_newerXY @ 0x9370
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;


unsigned int pred_newerXY(long long a0, long long a1, struct_0 *a2)
{
    unsigned long long v1;  // rax
    long long v2;  // rdx
    long long v4;  // rsi
    unsigned long long v5;  // rax

    if (a2->field_3c)
        __assert_fail(); /* do not return */
    switch (a2->field_38)
    {
    case 0:
        v1 = sub_425c40(a1);
        v5 = sub_408790(v1, v4, *((long long *)&a2->field_40), *((long long *)&a2->field_48));
        return _INSERT(v5, 0, (unsigned int)v5 > 0);
    case 1:
        v1 = sub_425c70(a1);
        if (v2 < 0)
        {
            sub_40dfb0(0, a0);
            error(0, 0, dcgettext(NULL, "WARNING: cannot determine birth time of file %s", 5));
            return 0;
        }
        break;
    case 2:
        v1 = sub_425c50(a1);
        break;
    case 3:
        v1 = sub_425c60(a1);
        break;
    case 4:
        __assert_fail(); /* do not return */
    default:
        __assert_fail(); /* do not return */
    }
}



// Function: pred_nogroup @ 0x94a0
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

char pred_nogroup(unsigned long a0, struct_0 *a1)
{
    return !getgrgid(a1->field_20);
}



// Function: pred_nouser @ 0x94c0
typedef struct struct_0 {
    char padding_0[28];
    unsigned int field_1c;
} struct_0;

char pred_nouser(unsigned long a0, struct_0 *a1)
{
    return !getpwuid(a1->field_1c);
}



// Function: pred_ok @ 0x94e0
typedef struct struct_0 {
    char padding_0[232];
    struct struct_1 *field_e8;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

unsigned int pred_ok(unsigned int a0, unsigned long a1, struct_0 *a2)
{
    if (!(char)sub_408990(*((long long *)&a2->field_e8), a0))
        return 0;
}



// Function: pred_okdir @ 0x9530
typedef struct struct_0 {
    long long field_0;
    char padding_8[48];
    char field_38;
    char padding_39[151];
    unsigned int field_d0;
    char padding_d4[20];
    struct struct_1 *field_e8;
    int field_f0;
    char padding_f4[4];
    long long field_f8;
    unsigned int field_100;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_43e5c0;
extern struct_2 *g_43e5f0;
extern char g_43e608;

int pred_okdir(void)
{
    unsigned long long v14;  // r12
    struct_0 *v15;  // rdx
    struct_0 *v24;  // r13
    struct_0 *v25;  // r14
    unsigned long long idx;  // rbx
    char *v30;  // rax
    int *err;  // rax
    unsigned int v16;  // rdi
    unsigned long long v17;  // r15
    unsigned long long v18;  // r14
    unsigned long long v19;  // r13
    char *ptr;  // r12
    unsigned long long v21;  // rbx
    unsigned long long v22;  // r15
    long long v23;  // rax
    unsigned long v0;  // [bp-0x60]
    unsigned long len2;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    char v3;  // [bp-0x41]
    char *v4;  // [bp-0x40]
    unsigned long long v5;  // [bp-0x30]
    char *v6;  // [bp-0x28]
    unsigned long long v7;  // [bp-0x20]
    unsigned long long v8;  // [bp-0x18]
    unsigned long long v9;  // [bp-0x10]
    unsigned long long v10;  // [bp-0x8]
    unsigned long long v11;  // [bp-0x8]
    char v12;  // [bp+0x0]

    v10 = v14;
    if (!(char)sub_408990(*((long long *)&v15->field_e8), v16))
        return;
    v11 = v17;
    v9 = v18;
    v8 = v19;
    v7 = v10;
    ptr = &g_43e5f0->field_0;
    v6 = &v12;
    v5 = v21;
    v3 = sub_40df90(v15->field_0);
    if (!v3)
    {
        if (v15->field_f8 == *((long long *)&g_43e5c0))
        {
            v2 = 0;
            v22 = 0;
            v4 = NULL;
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    else
    {
        if ((char)sub_40a190(&v15->field_38))
        {
            ptr = sub_4176f0(g_43e5f0);
            v4 = ptr;
            v22 = (*(ptr) != 47) * 2;
            v23 = 0;
            if (*(ptr) != 47)
                v23 = "./";
            v2 = v23;
        }
        else
        {
            sub_40dfb0(0, g_43e5f0);
            v30 = dcgettext(NULL, "Failed to save working directory in order to run a command on %s", 5);
            err = __errno_location();
            error(1, *(err), v30);
        }
    }
    v24 = &v15->padding_39[95];
    v25 = &v15->padding_39[7];
    if (!v15->field_38)
    {
        idx = 0;
        if (v15->field_f0 > 0)
        {
            do
            {
                len2 = strlen(ptr);
                v0 = *((long long *)&v15->field_e8[2 * idx]);
                sub_415290(v25, v24, v0, strlen(*((long long *)&v15->field_e8[2 * idx])), v2, v22, ptr, len2, 0);
                idx += 1;
            } while (v15->field_f0 > (unsigned int)idx);
        }
        sub_414e90(v25, v24);
        if (!v3)
        {
            if (!v4)
                return;
            __assert_fail(); /* do not return */
        }
        sub_425280(v15->field_f8);
        if (!v4)
            return;
        v3 = !(char)v15->field_100 && !(v15->field_100 & 127);
    }
    else
    {
        sub_415000(v25, v24, ptr, strlen(ptr) + 1, v2, v22, 0);
        if (v15->field_d0)
            g_43e608 = 1;
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



// Function: pred_path @ 0x95a0
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

char pred_path(char *a0, unsigned long a1, struct_0 *a2)
{
    return !fnmatch(a2->field_38, a0, 0);
}



// Function: pred_perm @ 0x95c0
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



// Function: pred_executable @ 0x9640
extern long long g_43e5f0;
extern unsigned int g_43e5f8;

char pred_executable(void)
{
    return !faccessat(g_43e5f8, g_43e5f0, 1, 0);
}



// Function: pred_readable @ 0x9670
extern long long g_43e5f0;
extern unsigned int g_43e5f8;

char pred_readable(void)
{
    return !faccessat(g_43e5f8, g_43e5f0, 4, 0);
}



// Function: pred_writable @ 0x96a0
extern long long g_43e5f0;
extern unsigned int g_43e5f8;

char pred_writable(void)
{
    return !faccessat(g_43e5f8, g_43e5f0, 2, 0);
}



// Function: pred_regex @ 0x96f0
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



// Function: pred_size @ 0x9730
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



// Function: pred_samefile @ 0x97a0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int pred_samefile(void)
{
    struct_0 *v1;  // rsi
    unsigned long v2;  // rdx

    if ((!v1->field_8 || v1->field_8 == *((long long *)(v2 + 56))) && !sub_40e080())
        return;
    return;
}



// Function: pred_type @ 0x9810
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_43e5e4;
extern char g_43e5e5;
extern unsigned int g_43e5e8;

unsigned int pred_type(unsigned long a0, struct_0 *a1, unsigned long a2)
{
    unsigned int v2;  // eax
    unsigned long long v4;  // rcx
    unsigned long long v5;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!g_43e5e5)
    {
        v0 = v5;
        __assert_fail(); /* do not return */
    }
    v2 = g_43e5e8;
    if (!g_43e5e8)
        return 0;
    if (g_43e5e4)
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



// Function: pred_uid @ 0x98f0
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



// Function: pred_used @ 0x9940
unsigned int pred_used(unsigned long a0, long long a1, unsigned long a2)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // rdx

    v1 = sub_425c40(a1);
    if (sub_408790(v1, v2, sub_425c50(a1), v2) < 0)
        return 0;
}



// Function: pred_user @ 0x99d0
void pred_user(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return;
}



// Function: pred_xtype @ 0x99e0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern unsigned long long g_40daa0;
extern unsigned long long g_40db20;
extern unsigned long long g_43e5f0;
extern unsigned int g_43e604;

unsigned int pred_xtype(long long a0, struct_0 *a1, unsigned int a2)
{
    unsigned long long *v2;  // rbx
    char v3;  // al
    int *err;  // rax
    struct_0 v0;  // [bp-0xc8]

    v2 = (!(char)sub_40dc30() ? &g_40daa0 : &g_40db20);
    sub_40d860(&v0);
    if (!v2(g_43e5f0, &v0))
        return sub_409810(a0, &v0, a2);
    v3 = sub_40dc30();
    err = __errno_location();
    if (v3 && *(err) == 2)
        return sub_409810(a0, a1, a2);
    sub_40dfb0(0, a0);
    error(0, *(err), "%s");
    g_43e604 = 1;
    return 0;
}



// Function: pred_context @ 0x9af0
typedef struct struct_1 {
    char padding_0[56];
    char *field_38;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_43e5f0;
extern unsigned int g_43e5f8;
extern struct_0 *g_43e680;

unsigned int pred_context(long long a0, unsigned long a1, struct_1 *a2)
{
    unsigned long v2;  // r12
    unsigned int v3;  // r12d
    char *v4;  // rax
    int *err;  // rax
    unsigned long v0;  // [bp-0x38]

    if (g_43e680(g_43e5f8, g_43e5f0, &v0) < 0)
    {
        sub_40dfb0(0, a0);
        v4 = dcgettext(NULL, "getfilecon failed: %s", 5);
        err = __errno_location();
        error(0, *(err), v4);
        return 0;
    }
    v3 = _INSERT(v2, 0, !fnmatch(a2->field_38, v0, 0));
    freecon(v0);
    return v3;
}



// Function: print_list @ 0x9bc0
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char padding_10[248];
    struct struct_0 *field_108;
} struct_0;

void print_list(int a0, struct_0 *a1)
{
    struct_0 *v2;  // rbx
    char v0[264];  // [bp-0x138]

    if (!a1)
    {
        __fprintf_chk(a0, 0x1, "\n");
        return;
    }
    do
    {
        v2 = a1;
        sub_408a40(v2->field_8, v0);
        __fprintf_chk(a0, 0x1, "[%s] ");
        a1 = v2->field_108;
    } while (v2->field_108);
    __fprintf_chk(a0, 0x1, "\n");
    return;
}



// Function: print_optlist @ 0x9c70
extern char g_42f827;
extern char g_43e660;

int print_optlist(long long a0, void* idx)
{
    char *v1;  // r9
    char *v2;  // r8
    char *v3;  // rcx
    unsigned long v4;  // rax
    unsigned long v5;  // rdx
    unsigned int v6;  // eax

    if (!idx)
        return v6;
    sub_409e10();
    v1 = "[need inum] ";
    if (!(char)idx[28])
        v1 = &g_42f827;
    v2 = "[need type] ";
    if (!(char)idx[27])
        v2 = &g_42f827;
    v3 = "[call stat] ";
    if (!(char)idx[26])
        v3 = &g_42f827;
    __fprintf_chk(a0, 0x1, "%s%s%s", v3, v2, v1);
    sub_40b5a0(a0, idx);
    __fprintf_chk(a0, 0x1, " [est success rate %.4g] ", (int)idx[36]);
    if (!(g_43e660 & 64))
        return sub_409e10();
    __fprintf_chk(a0, 0x1, "[real success rate %lu/%lu", (long long)idx[296], (long long)idx[288]);
    v4 = (long long)idx[288];
    if (v4)
    {
        v5 = (long long)idx[296];
        __fprintf_chk(a0, 0x1, "=%.4g] ", (unsigned long long)(DivV((v5 < 0 ? AddV(v5 >> 1 | (unsigned int)v5 & 1, v5 >> 1 | (unsigned int)v5 & 1) : v5), (v4 < 0 ? AddV(v4 >> 1 | (unsigned int)v4 & 1, v4 >> 1 | (unsigned int)v4 & 1) : v4))));
    }
    else
    {
        __fprintf_chk(a0, 0x1, "=_] ");
    }
    return sub_409e10();
}



// Function: print_parenthesised @ 0x9e10
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
        if (v6 == sub_4088c0 || v6 == sub_408870)
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
                return sub_409c70(a0, v5);
            break;
        }
    }
    __fprintf_chk(a0, 0x1, "%s", " ( ");
    sub_409c70(a0, v5);
    return __fprintf_chk(a0, 0x1, "%s");
}



// Function: show_success_rates @ 0x9ee0
extern long long stderr;
extern char g_43e660;

int show_success_rates(void* a0)
{
    int v1;  // eax

    if (g_43e660 & 64)
    {
        __fprintf_chk(stderr, 0x1, "Predicate success rates after completion:\n");
        sub_409c70(stderr, a0);
        return __fprintf_chk(stderr, 0x1, "\n");
    }
    return v1;
}



// Function: pred_sanity_check @ 0x9f40
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
            else if (v2 != sub_4086f0 && v2 != sub_408a20 && !idx->field_19)
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



// Function: initialize_wd_for_exec @ 0xa120
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[192];
    struct struct_0 *field_c0;
} struct_2;

unsigned int initialize_wd_for_exec(struct_2 *a0, unsigned int a1, long long a2)
{
    struct_0 *v1;  // rax
    unsigned long v2;  // rdi

    v1 = sub_426990(16);
    a0->field_c0 = v1;
    v1->field_8 = 0;
    v1->field_0 = sub_41e690(a1, a2, 0);
    v2 = a0->field_c0->field_0;
    if ((unsigned int)v2 < 0)
        return 0;
    sub_4174f0(v2, 1);
    return 1;
}



// Function: record_exec_dir @ 0xa190
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[192];
    struct struct_0 *field_c0;
} struct_2;

extern void g_43e5f0;
extern unsigned int g_43e5f8;

unsigned int record_exec_dir(struct_2 *a0)
{
    unsigned long v1;  // rax
    unsigned int v2;  // eax

    if (*((int *)&a0->padding_0[152]))
    {
        return 1;
    }
    else if (strchr(*((long long *)&g_43e5f0), 47))
    {
        v1 = sub_4177a0(*((long long *)&g_43e5f0));
        v2 = sub_40a120(a0, g_43e5f8, v1);
        free(v1);
        return v2;
    }
    else
    {
        return sub_40a120(a0, g_43e5f8, ".");
    }
}



// Function: prep_child_for_exec @ 0xa220
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

unsigned int prep_child_for_exec(unsigned int a0, struct_0 *a1)
{
    unsigned int v2;  // r12d
    int *err;  // rax
    char *v4;  // rax
    int *err1;  // rax
    char *v6;  // rax
    int *err2;  // rax
    char v0[10];  // [bp-0x3a]

    v2 = 1;
    if ((char)a0)
    {
        strncpy(v0, "/dev/null", 9);
        if (close(0) < 0)
        {
            a0 = 0;
            v4 = dcgettext(NULL, "Cannot close standard input", 5);
            err1 = __errno_location();
            error(0, *(err1), v4);
        }
        else if (sub_417b20(v0, 0) < 0)
        {
            sub_40dfb0(0, v0);
            err = __errno_location();
            error(0, *(err), "%s");
        }
        v2 = a0;
    }
    if (!sub_425250(a1))
        return v2;
    v6 = dcgettext(NULL, "Failed to change directory%s%s", 5);
    err2 = __errno_location();
    error(0, *(err2), v6);
    return 0;
}



// Function: launch @ 0xa610
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_43e01c;
extern long long stderr;
extern long long stdout;
extern unsigned int g_43e604;
extern char g_43e660;
extern unsigned int g_43e68c;

int launch(void)
{
    unsigned long long v1;  // rbx
    struct_0 *v2;  // rsi
    char *v14;  // rax
    char *v15;  // rax
    int *err;  // rax
    unsigned long long v3;  // rbx
    unsigned int v4;  // r13d
    void* v5;  // rcx
    char v6;  // al
    int *err1;  // rax
    int *err2;  // rax
    char v9;  // bl
    unsigned int v10;  // ebx

    if (g_43e660 & 32)
    {
        v1 = 0;
        __fprintf_chk(stderr, 0x1, "DebugExec: launching process (argc=%lu):", v2->field_60 - 1);
        if (v2->field_60 != 1)
        {
            do
            {
                v3 = v1 + 1;
                sub_40dfb0(0, *((long long *)(v2->field_68 + v1 * 8)));
            } while ((__fprintf_chk((unsigned int)stderr, 0x1, " %s"), v1 = v3, v2->field_60 - 1 > v1));
        }
        __fprintf_chk(stderr, 0x1, "\n");
    }
    sub_417bc0(stdout);
    sub_417bc0(stderr);
    if (g_43e01c)
    {
        g_43e01c = 0;
        signal(0x11, NULL);
    }
    v4 = fork();
    if (v4 != 0xffffffff)
    {
        if (v4)
        {
            while (waitpid(v4, &v2->field_c8, 0) == 0xffffffff)
            {
                err2 = __errno_location();
                if (*(err2) != 4)
                {
                    sub_40dfb0(0, *((long long *)v5));
                    v14 = dcgettext(NULL, "error waiting for %s", 5);
                    error(0, *(err2), v14);
                    g_43e604 = 1;
                    return;
                }
            }
            v9 = v2->field_c8;
            if ((v9 & 127) + 1 <= 1)
            {
                v10 = v9;
                if (g_43e660 & 32)
                    __fprintf_chk(stderr, 0x1, "DebugExec: process (PID=%ld) terminated with exit status: %d\n", v4, v10 & 0xff);
                if (!v10)
                    return;
            }
            else
            {
                sub_424e50(0, g_43e68c, *((long long *)v5));
                error(0, 0, dcgettext(NULL, "%s terminated by signal %d", 5));
            }
            if (!v2->field_0)
                return;
            g_43e604 = 1;
            return;
        }
        else if (v2->field_c0)
        {
            if ((char)sub_40a220(v2->field_bc, v2->field_c0))
            {
                if ((char)sub_415dd0())
                    sub_415df0();
                v6 = sub_415470(v5);
                err1 = __errno_location();
                if (v6)
                    *(err1) = 7;
                else
                    execvp(*((long long *)v5), v5);
                sub_40dfb0(0, *((long long *)v5));
                error(0, *(err1), "%s");
            }
            _exit(1); /* do not return */
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    v15 = dcgettext(NULL, "cannot fork", 5);
    err = __errno_location();
    error(1, *(err), v15);
}



// Function: predicate_is_cost_free @ 0xa950
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[24];
    unsigned int field_20;
} struct_0;

extern unsigned short g_43e688;

long long predicate_is_cost_free(struct_0 *a0)
{
    unsigned long long v1;  // rdx
    unsigned long long v2;  // rsi
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    unsigned long v5;  // rax

    v1 = a0->field_0;
    v2 = _INSERT(0, 0, v1 == sub_4095a0);
    v3 = _INSERT(0, 0, v1 == sub_4092e0) | (unsigned int)v2 | _INSERT(v2, 0, v1 == sub_409170);
    v4 = _INSERT(v3, 0, (char)v3 | v1 == sub_4091e0);
    if ((char)v3 || v1 == sub_4091e0)
    {
        return v4;
    }
    else if (v1 == sub_408870 || v1 == sub_4092f0 || !g_43e688 || v1 == sub_408cd0)
    {
        return v4;
    }
    else if (v1 == sub_4088c0)
    {
        return v4;
    }
    else
    {
        v5 = a0->field_20;
        return _INSERT(v5, 0, !(unsigned int)v5);
    }
}



// Function: predlist_init @ 0xa9e0
void predlist_init(unsigned long long *a0)
{
    a0[1] = 0;
    *(a0) = 0;
    return;
}



// Function: predlist_insert @ 0xa9f0
typedef struct struct_0 {
    char padding_0[272];
    struct struct_0 *field_110;
} struct_0;

struct_0 * predlist_insert(struct_0 **idx, struct_0 *index, struct_0 **a2)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // 4101

    *(a2) = index->field_110;
    v1 = *(idx);
    v2 = idx[1];
    index->field_110 = v1;
    *(idx) = index;
    if (!v2)
        idx[1] = index;
    return v1;
}



// Function: pred_cost_compare @ 0xaa20
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
    unsigned int field_24;
} struct_0;

typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
} struct_1;

unsigned long long pred_cost_compare(struct_0 *a0, struct_1 *a1, char a2)
{
    unsigned int v1;  // ymm0
    unsigned int v2;  // ymm1

    if (a0->field_20 != *((int *)&a1->padding_0[32]))
        return -(a0->field_20 < *((int *)&a1->padding_0[32])) | 1;
    v1 = a0->field_24;
    v2 = a1->field_24;
    if (!((char)((CmpF(v1, v2) & 69) >> 2) & 1) && CmpF(v1, v2) & 64)
        return 0;
    if (!a2)
    {
        return (!((CmpF(v2, v1) & 69 | (CmpF(v2, v1) & 69) >> 6) & 1)) * 2 - 1 & 0xffffffff;
    }
    else if ((((char)(CmpF(v2, v1) & 69) | (char)((CmpF(v2, v1) & 69) >> 6)) & 1) == 1)
    {
        return 1;
    }
    else
    {
        return 0xffffffff;
    }
}



// Function: subtree_has_side_effects @ 0xaa70
typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[247];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

unsigned long long subtree_has_side_effects(struct_0 *iter)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    for (; iter; iter = iter->field_118)
    {
        v1 = iter->field_18;
        if ((char)v1)
            return v1;
        v2 = subtree_has_side_effects(iter->field_110);
        if ((char)v2)
            return v2;
    }
    return 0;
}



// Function: worst_cost @ 0xaab0
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
    char padding_24[236];
    void* field_110;
    void* field_118;
} struct_0;

unsigned long long worst_cost(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    unsigned int v3;  // ebx
    unsigned int v4;  // eax
    unsigned long long v0;  // [bp-0x10]

    if (!a0)
        return 0;
    v0 = v2;
    v3 = worst_cost(a0->field_110);
    v4 = worst_cost(a0->field_118);
    if (v4 > v3)
        v3 = v4;
    return (v3 < a0->field_20 ? a0->field_20 : v3);
}



// Function: perform_arm_swap @ 0xab00
typedef struct struct_2 {
    char padding_0[272];
    struct struct_0 *field_110;
    unsigned long long field_118;
} struct_2;

typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

struct_0 * perform_arm_swap(struct_2 *idx)
{
    struct_0 *index;  // rax
    unsigned long long v2;  // rdx

    index = idx->field_110;
    v2 = index->field_118;
    index->field_118 = idx->field_118;
    idx->field_118 = v2;
    return index;
}



// Function: constrain_rate @ 0xab30
double constrain_rate(void)
{
    int v1;  // xmm0

    if (((CmpF(*((unsigned int *)&v1), 0x3f800000) & 69 | (char)((CmpF(*((unsigned int *)&v1), 0x3f800000) & 69) >> 6)) & 1) != 1)
        return 0x3f800000;
    return *((unsigned long long *)&MaxV(0, v1));
}



// Function: merge_pred @ 0xab60
typedef struct struct_0 {
    char padding_0[272];
    unsigned long long field_110;
} struct_0;

unsigned long long merge_pred(unsigned long long a0, struct_0 *a1, unsigned long long *a2)
{
    unsigned long long v1;  // rax

    v1 = *(a2);
    a1->field_110 = v1;
    *(a2) = a0;
    return v1;
}



// Function: getrate @ 0xab70
void getrate(unsigned long long a0)
{
    return;
}



// Function: cost_name @ 0xab90
extern unsigned int g_43e020;

long long cost_name(unsigned int i)
{
    unsigned long long v1;  // rax
    unsigned int *v2;  // rdx

    v1 = 0;
    v2 = &g_43e020;
    while (*(v2) != i)
    {
        v1 = (unsigned int)v1 + 1;
        v2 += 4;
        if ((unsigned int)v1 == 11)
            return "unknown";
    }
    return *((long long *)&(&g_43e020)[2 + 4 * v1]);
}



// Function: type_name @ 0xabd0
typedef struct struct_0 {
    char padding_0[16];
    unsigned short field_10;
} struct_0;

extern struct_0 g_43c040;

long long type_name(unsigned short a0)
{
    unsigned short v1;  // dx
    unsigned int v2;  // ecx
    struct_0 *iter;  // rax
    unsigned int v4;  // ecx

    if (!a0)
        return "no";
    v1 = 1;
    v2 = 0;
    iter = &g_43c040.field_0;
    while (1)
    {
        v4 = v2 + 1;
        iter = &iter->field_10;
        if (a0 == v1 || v1 == 0xffff)
            break;
        v1 = iter->field_10;
        v2 = v4;
    }
    return (&g_43c040.field_0)[3 + 2 * v2];
}



// Function: prec_name @ 0xac30
typedef struct struct_0 {
    char padding_0[16];
    unsigned short field_10;
} struct_0;

extern struct_0 g_43c0c0;

long long prec_name(unsigned short a0)
{
    unsigned short v1;  // dx
    unsigned int v2;  // ecx
    struct_0 *iter;  // rax
    unsigned int v4;  // ecx

    if (!a0)
        return "no";
    v1 = 1;
    v2 = 0;
    iter = &g_43c0c0.field_0;
    while (1)
    {
        v4 = v2 + 1;
        iter = &iter->field_10;
        if (a0 == v1 || v1 == 0xffff)
            break;
        v1 = iter->field_10;
        v2 = v4;
    }
    return (&g_43c0c0.field_0)[3 + 2 * v2];
}



// Function: check_normalization @ 0xac90
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



// Function: scan_rest @ 0xad10
void* scan_rest(void* *a0, void* a1, short a2)
{
    void* idx;  // rdx
    void* index;  // r12
    unsigned int v3;  // r13d
    unsigned long v4;  // rsi

    idx = *(a0);
    if (!idx)
        return NULL;
    index = a1;
    if ((int)idx[16] == 5)
        return NULL;
    while (1)
    {
        if ((int)idx[20] <= a2)
            return index;
        v3 = (int)idx[16];
        if (v3 == 4)
            error(1, 0, dcgettext(NULL, "invalid expression", 5));
        if (v3 > 4)
            break;
        if (v3 != 3)
            error(1, 0, dcgettext(NULL, "invalid expression", 5));
        *((void* *)&idx[272]) = index;
        index = *(a0);
        v4 = (short)index[20];
        *(a0) = (long long)index[264];
        *((unsigned long long *)&index[280]) = sub_40ae10(a0, v4);
        idx = *(a0);
        if (!*(a0))
            return index;
    }
    if (v3 == 5)
        return index;
    error(1, 0, dcgettext(NULL, "oops -- invalid expression type (%d)!", 5));
}



// Function: get_expr @ 0xae10
typedef struct struct_3 {
    char padding_0[16];
    unsigned int field_10;
} struct_3;

typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[21];
    char field_29;
    char padding_2a[222];
    void* field_108;
    char padding_110[8];
    unsigned long long field_118;
} struct_0;


struct_0 * get_expr(struct_0 **a0, short a1, struct_3 *a2)
{
    struct_0 *idx;  // r12
    void* v2;  // rax
    struct_0 *v11;  // r12
    unsigned long v3;  // 4113
    struct_0 *v6;  // rax
    void* v7;  // rax
    unsigned int v10;  // edx

    idx = *(a0);
    if (!idx)
        error(1, 0, dcgettext(NULL, "invalid expression", 5));
    switch (idx->field_10)
    {
    case 0:
        error(1, 0, dcgettext(NULL, "invalid expression", 5));
    case 1:
        v7 = idx->field_108;
        *(a0) = v7;
        if (!v7)
            return idx;
        v10 = a1;
        if ((int)v7[20] <= v10)
            return idx;
        v11 = sub_40ad10(a0, idx, v10);
        if (v11)
            return v11;
        error(1, 0, dcgettext(NULL, "invalid expression", 5));
    case 2:
        *(a0) = idx->field_108;
        idx->field_118 = get_expr(a0, 4, idx);
        v7 = *(a0);
        break;
    case 3:
        error(1, 0, dcgettext(NULL, "invalid expression; you have used a binary operator '%s' with nothing before it.", 5));
    case 4:
        v2 = idx->field_108;
        if (!v2 || (char)v2[41])
            error(1, 0, dcgettext(NULL, "invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'", 5));
        v3 = (int)v2[16];
        *(a0) = v2;
        if ((unsigned int)v3 != 5)
        {
            idx = get_expr(a0, 0, idx);
            v6 = *(a0);
            if (!v6 || v6->field_10 != 5)
                error(1, 0, dcgettext(NULL, "invalid expression; I was expecting to find a ')' somewhere but did not see one.", 5));
            v7 = v6->field_108;
            *(a0) = v7;
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



// Function: cost_table_comparison @ 0xb160
int cost_table_comparison(unsigned long long *a0, unsigned long long *a1)
{
    unsigned long v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]

    v0 = *(a0);
    v1 = *(a1);
    return memcmp(&v0, &v1, 8);
}



// Function: set_new_parent @ 0xb1c0
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[8];
    unsigned int field_20;
    unsigned int field_24;
    char padding_28[240];
    unsigned long long field_118;
} struct_0;

struct_0 * set_new_parent(unsigned long a0, unsigned int a1, struct_0 **a2)
{
    struct_0 *idx;  // rax
    unsigned int v2;  // ymm0
    struct_0 *index;  // rax
    unsigned int v4;  // ymm0
    struct_0 *idx1;  // rax

    idx = sub_426c50(312);
    idx->field_10 = 3;
    idx->field_14 = a1;
    idx->field_20 = 0;
    if (a1 == 2)
    {
        *((char **)&idx->padding_0[8]) = "-o";
        *((void* *)&idx->padding_0[0]) = sub_4088c0;
        v4 = (int)sub_40ab30();
        idx1->field_118 = a0;
        idx1->field_24 = v4;
        *(a2) = idx1;
        return idx1;
    }
    else if (a1 != 3)
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
            *((void* *)&idx->padding_0[0]) = sub_408cd0;
            idx->field_24 = 0x3f800000;
            idx->field_118 = a0;
        }
        return idx;
    }
    else
    {
        *((char **)&idx->padding_0[8]) = "-a";
        *((void* *)&idx->padding_0[0]) = sub_408870;
        v2 = (int)sub_40ab30();
        index->field_118 = a0;
        index->field_24 = v2;
        *(a2) = index;
        return index;
    }
}



// Function: get_pred_cost @ 0xb350
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
} struct_0;

typedef struct struct_1 {
    char padding_0[8];
    unsigned int field_8;
} struct_1;

extern void g_43e0e0;
extern unsigned int g_43e6b0;

long long get_pred_cost(struct_0 *a0)
{
    unsigned int v2;  // ebp
    unsigned long v3;  // rax
    long long v4;  // rdi
    long long v5;  // rsi
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9
    struct_1 *v10;  // rax
    unsigned int v11;  // eax
    unsigned long v0;  // [bp-0x38]

    v2 = 3;
    if (!a0->field_1a)
    {
        v2 = 1;
        if (!a0->field_1c)
            v2 = a0->field_1b * 2;
    }
    v3 = a0->field_0;
    switch (v3)
    {
    case 4231136: case 4231152:
        return 8 - a0->field_38;
    case 4258512:
        v11 = a0->field_20;
        break;
    default:
        if (!g_43e6b0)
        {
            qsort(&g_43e0e0, 59, 16, sub_40b160);
            if (!(char)sub_40b2d0(v4, v5, v6, v7, v8, v9))
                error(1, 0, "failed to sort the costlookup array");
            g_43e6b0 = 1;
            v3 = a0->field_0;
        }
        v0 = v3;
        v10 = bsearch(&v0, &g_43e0e0, 59, 16, sub_40b160);
        if (v10)
        {
            v11 = v10->field_8;
            break;
        }
        else
        {
            error(0, 0, dcgettext(NULL, "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug", 5));
            v11 = 10;
            break;
        }
    }
    return (v11 <= v2 ? v2 : v11);
}



// Function: estimate_costs @ 0xb4d0
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
} struct_0;

long long estimate_costs(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!a0)
        return v4;
    v0 = v2;
    estimate_costs(*((long long *)&a0[4].padding_24[16]));
    estimate_costs(*((long long *)&a0[4].padding_24[8]));
    v3 = sub_40b350(a0);
    a0->field_20 = v3;
    return v3;
}



// Function: print_predicate @ 0xb5a0
int print_predicate(int a0, char **a1)
{
    if (!a1[6])
        return __fprintf_chk(a0, 0x1, "%s", a1[1]);
    return __fprintf_chk(a0, 0x1, "%s %s", a1[1], a1[6]);
}



// Function: get_eval_tree @ 0xb5e0
extern unsigned long long g_43e6d0;

long long get_eval_tree(void)
{
    return g_43e6d0;
}



// Function: calculate_derived_rates @ 0xb5f0
typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
} struct_0;


double calculate_derived_rates(void* idx)
{
    struct_0 *v1;  // r12
    unsigned long v2;  // rbp
    unsigned int v3;  // eax
    unsigned long long v4;  // rax
    unsigned long long v5;  // xmm0lq
    unsigned long long v6;  // xmm0lq

    if (!idx)
        __assert_fail(); /* do not return */
    v1 = (long long)idx[280];
    if (v1)
        calculate_derived_rates(v1);
    v2 = (long long)idx[272];
    if (v2)
        calculate_derived_rates(v2);
    v3 = (int)idx[16];
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
            return (unsigned long long)(int)idx[36];
        }
        else if (v3 == 3)
        {
            v4 = *((long long *)idx);
            switch (v4)
            {
            case 4229232:
                sub_40ab70(v1);
                sub_40ab70(v2);
                break;
            case 4230352:
                break;
            case 4229312:
                sub_40ab70(v1);
                sub_40ab70(v2);
                break;
            default:
                __assert_fail(); /* do not return */
            }
            v5 = (unsigned long long)sub_40ab30();
            *((unsigned int *)&idx[36]) = v5;
            return v5;
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    else
    {
        if (*((long long *)idx) != sub_4092f0)
        {
            __assert_fail(); /* do not return */
        }
        else if (!v2)
        {
            v6 = SubV(0x3f800000, v1->field_24);
            *((unsigned int *)&idx[36]) = v6;
            return v6;
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
    char padding_29[7];
    long long field_30;
    char padding_38[248];
    struct struct_1 *field_130;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

typedef struct struct_4 {
    char padding_0[264];
    struct struct_0 *field_108;
} struct_4;

extern char g_43e62f;
extern struct_4 *g_43e6c8;
extern unsigned long long g_43e6d8;

void get_new_pred(unsigned int *a0)
{
    struct_0 *idx;  // rax

    if (!*(a0))
    {
        __assert_fail(); /* do not return */
    }
    else if (*(a0) != 2)
    {
        idx = sub_426c50(312);
        if (g_43e6d8)
            g_43e6c8->field_108 = idx;
        else
            g_43e6d8 = idx;
        idx->field_1a = 0x101;
        idx->field_130 = a0;
        idx->field_10 = 0;
        idx->field_30 = "ThisShouldBeSetToSomethingElse";
        idx->field_28 = g_43e62f;
        idx->field_20 = 4575657221408423946;
        g_43e6c8 = idx;
        sub_40b2b0(idx, 4575657221408423946, g_43e62f, "ThisShouldBeSetToSomethingElse");
        return;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: get_new_pred_noarg @ 0xb970
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

void get_new_pred_noarg(unsigned int *a0)
{
    struct_0 *v1;  // rax

    v1 = (unsigned long long)sub_40b8a0(a0);
    if (v1)
        v1->field_30 = 0;
    return;
}



// Function: get_new_pred_chk_op @ 0xb990
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

extern unsigned long long g_43e6a0;
extern struct_0 *g_43e6c8;

void get_new_pred_chk_op(unsigned int *a0, unsigned long a1)
{
    unsigned long long v1;  // rdi
    struct_1 *idx;  // rax
    struct_2 *index;  // rax

    v1 = g_43e6a0;
    if (!v1)
    {
        v1 = sub_4147f0("and");
        g_43e6a0 = v1;
        if (!v1)
            __assert_fail(); /* do not return */
    }
    if (g_43e6c8)
    {
        if (((int)g_43e6c8[2].field_0 & 0xfffffffb) == 1)
        {
            idx = (unsigned long long)sub_40b970(v1);
            *((char **)&idx->padding_0[8]) = "-a";
            *((void* *)&idx->padding_0[0]) = sub_408870;
            idx->field_30 = 0;
            idx->field_38 = 0;
            idx->field_10 = 12884901891;
            idx->field_18 = 0;
            idx->field_1c = 0;
        }
        else if (!(int)g_43e6c8[2].field_0)
        {
            error(1, 0, dcgettext(NULL, "oops -- invalid default insertion of and!", 5));
        }
    }
    index = (unsigned long long)sub_40b8a0(a0);
    index->field_30 = a1;
    index->field_130 = a0;
    return;
}



// Function: print_tree @ 0xbaa0
typedef struct FILE {
} FILE;

extern char g_42f827;

int print_tree(long long a0, void* a1, unsigned int i)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    FILE *v17;  // rsi
    char *v18;  // rcx
    char *v19;  // rcx
    unsigned int v20;  // r14d
    unsigned int v21;  // r14d
    int v22;  // eax
    char *v23;  // rdx
    int v24;  // eax
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    void* v11;  // r12
    unsigned int v12;  // r14d
    unsigned long v13;  // rax
    char *v15;  // r8
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!a1)
        return v24;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    while (1)
    {
        v11 = a1;
        if (i > 0)
        {
            v12 = 0;
            do
            {
                __fprintf_chk(a0, 0x1, "    ");
                v12 += 1;
            } while (i != v12);
        }
        __fprintf_chk(a0, 0x1, "pred=[");
        sub_40b5a0(a0, v11);
        v13 = sub_40ac30((short)v11[20]);
        __fprintf_chk(a0, 0x1, "] type=%s prec=%s", sub_40abd0((short)v11[16]), v13);
        v15 = "no ";
        if ((char)v11[24])
            v15 = &g_42f827;
        __fprintf_chk(a0, v17, " cost=%s est_success_rate=%#.4g %sside effects ", sub_40ab90((int)v11[32]), v15);
        if (0xffffff0000 & (long long)v11[24])
        {
            __fprintf_chk(a0, 0x1, "Needs ");
            if ((char)v11[26])
            {
                __fprintf_chk(a0, 0x1, "stat");
                if ((char)v11[28])
                {
                    v18 = ",";
                }
                else
                {
                    if (!(char)v11[27])
                        goto LABEL_40bba8;
LABEL_40bce3:
                    v19 = ",";
                    goto LABEL_40bcea;
                }
LABEL_40bcc1:
                __fprintf_chk(a0, 0x1, "%sinode", v18);
                if (!(char)v11[27])
                    goto LABEL_40bba8;
                goto LABEL_40bce3;
            }
            else if ((char)v11[28])
            {
                v18 = &g_42f827;
                goto LABEL_40bcc1;
            }
            else if ((char)v11[27])
            {
                v19 = &g_42f827;
LABEL_40bcea:
                __fprintf_chk(a0, 0x1, "%stype", v19);
            }
        }
LABEL_40bba8:
        __fprintf_chk(a0, 0x1, "\n");
        if (i > 0)
        {
            v20 = 0;
            do
            {
                __fprintf_chk(a0, 0x1, "    ");
                v20 += 1;
            } while (i != v20);
        }
        if ((long long)v11[272])
        {
            __fprintf_chk(a0, 0x1, "left:\n");
            print_tree(a0, (long long)v11[272], i + 1);
        }
        else if ((long long)v11[280])
        {
            __fprintf_chk(a0, 0x1, "no left.\n");
        }
        else
        {
            v23 = "no children.\n";
            break;
        }
        v21 = 0;
        if (i > 0)
        {
            do
            {
                __fprintf_chk(a0, 0x1, "    ");
                v21 += 1;
            } while (i != v21);
        }
        if ((long long)v11[280])
        {
            i += 1;
            v22 = __fprintf_chk(a0, 0x1, "right:\n");
            a1 = (long long)v11[280];
            if (!(long long)v11[280])
                return v22;
        }
        else
        {
            v23 = "no right.\n";
            break;
        }
    }
    return __fprintf_chk(a0, 0x1, v23);
}



// Function: predlist_merge_sort @ 0xbdc0
typedef struct struct_0 {
    char padding_0[272];
    struct struct_0 *field_110;
} struct_0;

typedef struct struct_3 {
    char padding_0[20];
    unsigned int field_14;
    char padding_18[248];
    struct struct_3 *field_110;
    struct struct_0 *field_118;
} struct_3;

typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
} struct_1;

extern long long stderr;
extern char g_43e660;

void predlist_merge_sort(struct_0 **a0, unsigned long long *a1)
{
    struct_0 *idx;  // rbx
    struct_3 *v4;  // rbp
    struct_3 *index;  // r8
    struct_0 *v5;  // rbx
    struct_3 *v6;  // rcx
    struct_1 *v7;  // rsi
    struct_0 *idx1;  // rcx
    struct_1 *v9;  // rax
    struct_3 *v0;  // [bp-0x48], Other Possible Types: unsigned long long
    struct_3 *v1;  // [bp-0x40]

    idx = *(a0);
    if (!idx)
        return;
    if (!(g_43e660 & 8))
    {
        sub_40b5f0(idx);
        sub_40a9e0(&v0);
    }
    else
    {
        __fprintf_chk(stderr, 0x1, "%s:\n", "predlist before merge sort");
        sub_40baa0(stderr, *(a0), 2);
        idx = *(a0);
        sub_40b5f0(idx);
        sub_40a9e0(&v0);
        v4 = v0;
        if (!idx)
            goto LABEL_40be98;
    }
    v4 = v0;
    do
    {
        v5 = idx->field_110;
        *(a0) = v5;
        idx->field_110 = NULL;
        if (v4)
        {
            v6 = v4;
            do
            {
                if ((unsigned int)sub_40aa20(v6->field_118, v7, v6->field_14 != 2) >= 0)
                {
                    v9 = idx1->field_110;
                    index->field_110 = v9;
                    if (!v9)
                        v1 = index;
                    idx1->field_110 = index;
                }
            } while ((v6 = (struct_3 *)idx1->field_110, v6));
        }
        index->field_110 = v4;
        v0 = index;
        if (!v1)
            v1 = index;
        v4 = index;
    } while ((idx = v5, idx));
LABEL_40be98:
    if (g_43e660 & 8)
    {
        __fprintf_chk(stderr, 0x1, "%s:\n", "predlist after merge sort");
        sub_40baa0(stderr, v4, 2);
    }
    sub_40b5f0(v4);
    sub_40ab60(v4, v1, a1);
    sub_40a9e0(a0);
    return;
}



// Function: opt_expr @ 0xc030
typedef struct FILE {
} FILE;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[8];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[248];
    struct struct_2 *field_110;
} struct_2;

typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[1];
    char field_1a;
    char padding_1b[5];
    unsigned int field_20;
    char padding_24[244];
    char field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[252];
    char field_110;
    char padding_111[7];
    struct struct_0 *field_118;
} struct_1;

extern long long stderr;
extern char g_43e660;
extern unsigned short g_43e688;

unsigned int opt_expr(struct_2 *a0)
{
    struct_2 *v6;  // rbx
    char *v7;  // rax
    unsigned int v16;  // r13d
    struct_0 *index;  // r15
    unsigned int v18;  // eax
    unsigned long v19;  // r8
    unsigned long v20;  // rax
    unsigned long v21;  // rax
    FILE *v22;  // rsi
    char *i;  // rcx
    struct_2 *v9;  // rdx
    struct_2 *iter;  // rdi
    struct_2 *j;  // rdx
    unsigned int v12;  // esi
    struct_2 *v13;  // r8
    struct_1 *idx;  // r14
    unsigned int k;  // ebp
    struct_0 *v0;  // [bp-0x118]
    unsigned long long v1;  // [bp-0x110]
    struct_0 *v2;  // [bp-0x108]
    unsigned long long v3;  // [bp-0x100]
    char v4;  // [bp-0xf8]

    v0 = NULL;
    v1 = 0;
    v2 = NULL;
    v3 = 0;
    if (!a0->field_0)
        return 0;
    v6 = a0;
    v7 = &v4;
    do
    {
        v7 = (unsigned long long)sub_40a9e0(v7);
    } while (i != v7);
    if (v9->field_110)
    {
        for (iter = v9->field_110; iter->field_110; j = iter)
        {
            iter = iter->field_110;
        }
        v12 = j->field_14;
        v13 = &j->field_110;
    }
    else
    {
        v12 = 3;
        iter = j;
        v13 = v6;
    }
    if (iter->field_10 == 3)
    {
        if (g_43e660 & 9)
            goto LABEL_40c2e0;
        idx = v6->field_0;
    }
    else
    {
        sub_40b1c0(iter, v12, v13);
        if (!(g_43e660 & 9))
            goto LABEL_40c1d5;
LABEL_40c2e0:
        __fprintf_chk(stderr, 0x1, "Normalized Eval Tree:\n");
        sub_40baa0(stderr, v6->field_0, 0);
LABEL_40c1d5:
        idx = v6->field_0;
        if (!v6->field_0)
        {
            sub_40bfd0(&v4, 11, &v2, &v0, v6);
            return 0;
        }
    }
    k = 0;
    if (idx->field_10 == 3)
        k = *((int *)&idx->padding_14[0]);
    v16 = 0;
    if (idx->field_10 == 3)
        goto LABEL_0x40c277;
    while (1)
    {
        do
        {
            do
            {
                index = idx->field_118;
                v18 = index->field_10;
                if (v18 == 2)
                {
                    index->padding_18[0] = opt_expr(&index->field_118);
                    index = idx->field_118;
                }
                else if (v18 <= 2)
                {
                    if (k != 1 && !index->padding_18[0])
                    {
                        if ((char)sub_40a950(index))
                        {
                            if (g_43e660 & 8)
                            {
                                __fprintf_chk(stderr, 0x1, "-O%d: promoting cheap predicate ", g_43e688);
                                sub_40b5a0(stderr, idx->field_118);
                                __fprintf_chk(stderr, 0x1, " into name_list\n");
                            }
                            sub_40a9f0(&v2, idx, v6);
                            continue;
                        }
                        else if (v19 == sub_4096f0)
                        {
                            sub_40a9f0(&v0, idx, v6);
                            continue;
                        }
                        else if (g_43e688 > 1 && !(v20 = (unsigned long)(unsigned long long)index->field_20, ((unsigned int)(v20 - 1) > 1 || index->padding_18[2]) && g_43e688 == 2))
                        {
                            if (g_43e660 & 8)
                            {
                                __fprintf_chk(stderr, 0x1, "-O%d: categorising predicate ", g_43e688);
                                sub_40b5a0(stderr, idx->field_118);
                                v21 = sub_40ab90(idx->field_118->field_20);
                                __fprintf_chk(stderr, v22, " by cost (%s)\n", v21);
                                v20 = idx->field_118->field_20;
                            }
                            sub_40a9f0(&(&v4)[16 * v20], idx, v6);
                            continue;
                        }
                        goto LABEL_40c257;
                    }
                }
                else
                {
                    if (v18 == 3)
                    {
                        index->padding_18[0] = opt_expr(&idx->field_118);
                        index = idx->field_118;
                    }
                    else
                    {
                        error(1, 0, dcgettext(NULL, "oops -- invalid expression type!", 5));
                    }
                }
                if (!index->padding_18[0])
                    goto LABEL_40c257;
                v16 = 1;
                sub_40bfd0(&v4, 11, &v2, &v0, v6);
LABEL_40c257:
                v6 = &idx->field_110;
                idx = v6->field_0;
                if (!idx)
                {
                    sub_40bfd0(&v4, 11, &v2, &v0, v6);
                    return v16;
                }
            } while (idx->field_10 != 3);
        } while (*((int *)&idx->padding_14[0]) == k);
        idx = sub_40b1c0(idx, k, v6);
    }
}



// Function: consider_arm_swap @ 0xc450
typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[247];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[252];
    struct struct_0 *field_110;
    unsigned long long field_118;
} struct_1;

typedef struct struct_2 {
    char padding_0[36];
    unsigned int field_24;
} struct_2;

extern long long stderr;

unsigned int consider_arm_swap(struct_1 *a0)
{
    char *v3;  // r8
    struct_2 *v4;  // r8
    unsigned int v5;  // eax
    struct_0 *v6;  // rsi
    unsigned int v7;  // eax
    unsigned int v8;  // r9d
    unsigned int v9;  // ymm2
    unsigned int v10;  // ymm3
    unsigned long long v11;  // rcx
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]

    v3 = "Not a binary operation";
    if (a0->field_10 == 3)
    {
        if (!(a0->field_110 && a0->field_118))
        {
            v3 = "Doesn't have two arms";
        }
        else if (!a0->field_110->field_118)
        {
            v3 = "Left arm has no child on RHS";
        }
        else if (!(char)sub_40aa70(a0->field_110->field_118))
        {
            v5 = sub_40aa70(v4);
            if ((char)v5)
            {
                v3 = "Right subtree has side-effects";
            }
            else
            {
                sub_40aab0(v6);
                v7 = sub_40aab0(v4);
                if (v8 < v7)
                {
                    v3 = "efficient as-is";
                }
                else if (v8 == v7)
                {
                    v9 = *((int *)&v6->padding_0[36]);
                    v10 = v4->field_24;
                    if (!v11)
                    {
                        if (a0->field_0 != sub_4088c0)
                        {
                            if (a0->field_0 != sub_408870)
                                return v5;
                        }
                        else
                        {
                            if (((CmpF(v9, v10) & 69 | (char)((CmpF(v9, v10) & 69) >> 6)) & 1) == 1)
                                return v5;
                            sub_40ab00(a0);
                            return 1;
                        }
                        goto LABEL_40c59c;
                    }
                    v1 = v9;
                    v0 = v10;
                    __fprintf_chk(stderr, 0x1, "Success rates: l=%f, r=%f\n", v9, v10);
                    v10 = v0;
                    v9 = v1;
                    if (a0->field_0 == sub_4088c0)
                    {
                        v3 = "Operation is OR; right success rate >= left";
                        if (((CmpF(v9, v10) & 69 | (CmpF(v9, v10) & 69) >> 6) & 1) != 1)
                            goto LABEL_40c5c0;
                    }
                    else
                    {
                        v3 = "Not 'AND' or 'OR'";
                        if (a0->field_0 == sub_408870)
                        {
LABEL_40c59c:
                            if (((CmpF(v10, v9) & 69 | (CmpF(v10, v9) & 69) >> 6) & 1) != 1)
                                goto LABEL_40c5c0;
                            v3 = "Operation is AND; right success rate <= left";
                        }
                    }
                }
                else
                {
LABEL_40c5c0:
                    if (v11)
                    {
                        __fprintf_chk(stderr, 0x1, "Performing arm swap on:\n");
                        sub_40baa0(stderr, a0, 0);
                        sub_40ab00(a0);
                    }
                    else
                    {
                        sub_40ab00(a0);
                    }
                    return 1;
                }
            }
        }
        else
        {
            v3 = "Left subtree has side-effects";
        }
    }
    if (v11)
    {
        __fprintf_chk(stderr, 0x1, "Not an arm swap candidate (%s):\n", v3);
        sub_40baa0(stderr, a0, 0);
    }
    return 0;
}



// Function: do_arm_swaps @ 0xc6b0
typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[247];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[252];
    struct struct_0 *field_110;
    unsigned long long field_118;
} struct_1;

unsigned int do_arm_swaps(struct_1 *a0)
{
    if (!a0)
        return 0;
    do
    {
        while (1)
        {
            if ((char)sub_40c450(a0))
                continue;
            if (!(char)do_arm_swaps(a0->field_110))
                break;
        }
    } while ((char)do_arm_swaps(a0->field_118));
    return 0;
}



// Function: build_expression_tree @ 0xc6f0
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

typedef struct struct_4 {
    unsigned long long field_0;
} struct_4;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern long long stderr;
extern char g_43e618;
extern char g_43e660;
extern unsigned long long g_43e6b8;
extern unsigned long long g_43e6c0;
extern struct_2 *g_43e6c8;
extern struct_2 g_43e6d0;
extern struct_0 *g_43e6d8;

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
    g_43e6d8 = 0;
    if (a2 < a0)
    {
        for (g_43e6c0 = a1 + i * 8; !(char)sub_40dca0(*((long long *)(a1 + i * 8)), 1) && (g_43e6b8 = g_43e6b8 + 1, v3 += 1, v3 < j); i = v3);
    }
    v8 = sub_4147f0("(");
    v0 = sub_4147f0(")");
    v9 = sub_4147f0("print");
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
        sub_413ec0(v8, a1, &j);
        *((char **)&g_43e6c8[8]) = "(";
        *((char *)&g_43e6d8[5].field_0 + 1) = 1;
        sub_4147d0(a1, j, g_43e6c8, g_43e6d8);
        sub_409f40(g_43e6c8);
        idx = v3;
        if ((unsigned int)idx < j)
        {
            do
            {
                v11 = *((long long *)(a1 + idx * 8));
                g_43e618 = 0;
                if (!(char)sub_40dca0(v11, 0))
                {
                    error(0, 0, dcgettext(NULL, "paths must precede expression: `%s'", 5));
                    if (!access(*((long long *)(a1 + v3 * 8)), 0))
                        error(0, 0, dcgettext(NULL, "possible unquoted pattern after predicate `%s'?", 5));
                    exit(1); /* do not return */
                }
                v12 = sub_4147f0(*((long long *)(a1 + v3 * 8)));
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
                *((long long *)&g_43e6c8[8]) = *((long long *)(a1 + v3 * 8));
                if (v4 != v13)
                    v14 = *((long long *)(a1 + v13 * 8));
                *((unsigned long long *)&g_43e6c8[48]) = v14;
                sub_409f40(g_43e6c8);
                sub_409f40(g_43e6d8);
                idx = v4;
                v3 = v4;
            } while ((unsigned int)idx < j);
        }
        sub_4147e0(a1);
        if (!g_43e6d8[33].field_0)
        {
            ptr = g_43e6d8;
            g_43e6c8 = 0;
            g_43e6d8 = 0;
            free(ptr);
            sub_412910(v9, a1, v1);
            *((char **)&g_43e6c8[8]) = "-print";
            sub_409f40(g_43e6c8);
            sub_409f40(g_43e6d8);
            goto LABEL_40c9b7;
        }
        else if ((char)sub_40dc60(ptr->field_108))
        {
            sub_413690(v0, a1, v1);
            *((char **)&g_43e6c8[8]) = ")";
            g_43e6c8[41].field_0 = 1;
            sub_409f40(g_43e6c8);
            sub_412910(v9, a1, v1);
            *((char **)&g_43e6c8[8]) = "-print";
            g_43e6c8[41].field_0 = 1;
            sub_409f40(g_43e6c8);
            sub_409f40(g_43e6d8);
LABEL_40c9b7:
            if (g_43e660 & 9)
                goto LABEL_40ca90;
        }
        else
        {
            ptr = g_43e6d8;
            g_43e6d8 = g_43e6d8[33].field_0;
            sub_409f40(g_43e6d8);
            free(ptr);
            if (g_43e660 & 9)
            {
LABEL_40ca90:
                __fprintf_chk(stderr, 0x1, "Predicate List:\n");
                sub_409bc0(stderr, g_43e6d8);
            }
        }
        sub_414670(g_43e6d8);
        sub_409f40(g_43e6d8);
        ptr = g_43e6d8;
        v22 = sub_40ae10(&ptr, 0, NULL);
        *((unsigned long *)&g_43e6d0) = v22;
        sub_40b5f0(v22);
        if (!ptr)
        {
            if (g_43e660 & 9)
            {
                __fprintf_chk(stderr, 0x1, "Eval Tree:\n");
                sub_40baa0(stderr, *((long long *)&g_43e6d0), 0);
                v22 = *((long long *)&g_43e6d0);
            }
            sub_40b4d0(v22);
            sub_40c030(&g_43e6d0.field_0);
            sub_40ac90(*((long long *)&g_43e6d0), 1);
            sub_40c6b0(*((long long *)&g_43e6d0));
            sub_40ac90(*((long long *)&g_43e6d0), 1);
            if (g_43e660 & 9)
            {
                __fprintf_chk(stderr, 0x1, "Optimized Eval Tree:\n");
                sub_40baa0(stderr, *((long long *)&g_43e6d0), 0);
                __fprintf_chk(stderr, 0x1, "Optimized command line:\n");
                sub_409c70(stderr, *((long long *)&g_43e6d0));
                __fprintf_chk(stderr, 0x1, "\n");
                return *((long long *)&g_43e6d0);
            }
            return *((long long *)&g_43e6d0);
        }
        else if (ptr->field_0 == sub_408c30)
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



// Function: traverse_tree @ 0xcdf0
typedef struct struct_0 {
    char padding_0[272];
    void* field_110;
    struct struct_0 *field_118;
} struct_0;

void traverse_tree(struct_0 *a0, unsigned long long *a1)
{
    struct_0 *v1;  // rbx

    do
    {
        v1 = a0;
        if (v1->field_110)
            traverse_tree(v1->field_110, a1);
    } while ((a1(v1), a0 = (struct_0 *)v1->field_118, v1->field_118));
    return;
}



// Function: undangle_file_pointers @ 0xce30
extern unsigned long long g_409090;

char undangle_file_pointers(unsigned long long *a0)
{
    unsigned long long v1;  // rdx

    v1 = *(a0);
    if (v1 == sub_409050 || v1 == sub_40fad0 || v1 == sub_409010 || v1 == &g_409090)
        a0[8] = 0;
    return v1 == sub_409050 | v1 == sub_40fad0 | v1 == sub_409010;
}



// Function: show_valid_debug_options @ 0xce70
typedef struct FILE {
} FILE;

extern unsigned long long g_43c158;
extern unsigned long long g_43c218;
extern FILE *stdout;

int show_valid_debug_options(unsigned int a0)
{
    char *v1;  // rax
    unsigned long long v2;  // rbx
    unsigned long long v3;  // rax
    unsigned long long v4;  // rbp
    int v5;  // eax
    int v6;  // eax

    v1 = dcgettext(NULL, "Valid arguments for -D:\n", 5);
    fputs(v1, stdout);
    v2 = &g_43c158;
    if (!a0)
    {
        v3 = 0;
        while (1)
        {
            v4 = v3 + 1;
            if (v3)
            {
                v5 = __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%s%s");
                if (v4 == 9)
                    break;
            }
            else
            {
                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%s%s");
            }
            v3 = v4;
        }
        return v5;
    }
    else
    {
        while (1)
        {
            v6 = __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%-10s %s\n");
            if (v2 == &g_43c218)
                break;
            v2 += 24;
        }
        return v6;
    }
}



// Function: do_exec @ 0xcf70
extern char g_43e5c0;

void do_exec(void* a0)
{
    sub_4154f0((long long)a0[192], sub_40d060, a0);
    if ((long long)a0[192] != *((long long *)&g_43e5c0))
    {
        sub_425280((long long)a0[192]);
        free((long long)a0[192]);
        *((unsigned long *)&a0[192]) = 0;
    }
    return;
}



// Function: do_complete_pending_execdirs @ 0xcfc0
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

extern char g_43e608;

long long do_complete_pending_execdirs(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    struct_0 *idx;  // rbx
    unsigned long v4;  // rax
    unsigned long v5;  // rax
    unsigned long v6;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!a0)
        return v6;
    v0 = v2;
    while (1)
    {
        idx = a0;
        if (!g_43e608)
            __assert_fail(); /* do not return */
        do_complete_pending_execdirs(idx->field_110);
        v4 = idx->field_0;
        if (v4 != sub_409530 && v4 != sub_408ff0 || !idx->field_38 || (v4 = (unsigned long)(unsigned long long)idx->field_d0, !(unsigned int)v4))
        {
            a0 = idx->field_118;
            if (!idx->field_118)
                return v4;
        }
        else
        {
            v5 = (unsigned long long)sub_40cf70(&idx->field_38);
            a0 = idx->field_118;
            if (!idx->field_118)
                return v5;
        }
    }
}



// Function: exec_cb @ 0xd060
unsigned int exec_cb(long long a0)
{
    sub_414e90(a0 + 8, a0 + 96);
    return 0;
}



// Function: complete_pending_execs @ 0xd080
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
        if (v3->field_0 == sub_408fe0 && v3->field_38 && v3->field_d0)
            v4 = sub_414e90(&v3->padding_39[7], &v3->padding_39[95]);
    } while ((a0 = (struct_0 *)v3->field_118, v3->field_118));
    return v4;
}



// Function: cleanup_initial_cwd @ 0xd0f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_43e5c0;

void cleanup_initial_cwd(void)
{
    char *v1;  // rax
    int *err;  // rax

    if (sub_425250(g_43e5c0))
    {
        v1 = dcgettext(NULL, "Failed to restore initial working directory%s%s", 5);
        err = __errno_location();
        error(0, *(err), v1);
        _exit(1); /* do not return */
    }
    sub_425280(g_43e5c0);
    free(g_43e5c0);
    g_43e5c0 = 0;
    return;
}



// Function: process_optimisation_option @ 0xd220
extern unsigned short g_43e688;

void process_optimisation_option(char *a0)
{
    unsigned long v2;  // rbx
    unsigned long long *v3;  // rax
    int *err;  // rax
    int v5;  // ebp
    int *v6;  // rbx
    unsigned long long num;  // r13
    char *v9;  // rax
    char v0;  // [bp-0x38]

    v2 = *(a0);
    if (!(char)v2)
        error(1, 0, dcgettext(NULL, "The -O option must be immediately followed by a decimal integer", 5));
    v3 = __ctype_b_loc();
    if (!(*((char *)(*(v3) + v2 * 2 + 1)) & 8))
        error(1, 0, dcgettext(NULL, "Please specify a decimal number immediately after -O", 5));
    err = __errno_location();
    v5 = *(err);
    *(err) = 0;
    v6 = err;
    num = strtoul(a0, &v0, 10);
    if (!num)
    {
        if (v0 == a0)
        {
            error(1, 0, dcgettext(NULL, "Please specify a decimal number immediately after -O", 5));
        }
        else if (!*((char *)v0))
        {
            g_43e688 = num;
            *(v6) = v5;
            return;
        }
    }
    else
    {
        if (!*((char *)v0))
        {
            if (num == 0xffffffffffffffff)
            {
                if (*(v6))
                {
                    v9 = dcgettext(NULL, "Invalid optimisation level %s", 5);
                    error(1, *(v6), v9);
                }
            }
            else
            {
                if (num <= 0xffff)
                {
                    g_43e688 = num;
                    *(v6) = v5;
                    return;
                }
            }
            error(1, 0, dcgettext(NULL, "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.", 5));
        }
    }
    error(1, 0, dcgettext(NULL, "Invalid optimisation level %s", 5));
}



// Function: now @ 0xd3d0
unsigned long long now(void)
{
    unsigned long long t;  // rax
    int v0[3];  // [bp-0x28]

    if (!gettimeofday(v0, NULL))
        return v0;
    t = time(NULL);
    if (t == 0xffffffffffffffff)
        __assert_fail(); /* do not return */
    return t;
}



// Function: insert_primary_withpred @ 0xd450
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

void insert_primary_withpred(struct_0 *a0, unsigned long a1, unsigned int a2)
{
    unsigned long long *idx;  // rax
    unsigned long long v2;  // rdx

    idx = (unsigned long long)sub_40b990(a0, a2);
    v2 = a0->field_8;
    *(idx) = a1;
    idx[1] = v2;
    idx[7] = 0;
    idx[2] = 1;
    return;
}



// Function: insert_primary @ 0xd490
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

void insert_primary(struct_0 *a0, unsigned int a1)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!a0[1].field_8)
    {
        v0 = v2;
        __assert_fail(); /* do not return */
    }
    sub_40d450(a0, a0[1].field_8, a1);
    return;
}



// Function: insert_primary_noarg @ 0xd4d0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

void insert_primary_noarg(struct_0 *a0)
{
    sub_40d490(a0, 0);
    return;
}



// Function: usage @ 0xd4e0
extern unsigned long long stderr;
extern void stdout;
extern long long g_43eed0;

void usage(int a0)
{
    unsigned long long v2;  // r12
    char *v3;  // rax
    char *v12;  // rax
    char *v13;  // rax
    char *v14;  // rax
    char *v4;  // rax
    char *v5;  // rax
    char *v6;  // rax
    char *v7;  // rax
    char *v8;  // rax
    char *v9;  // rax
    char *v10;  // rax
    char *v11;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (!a0)
    {
        v4 = dcgettext(NULL, "Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n", 5);
        __fprintf_chk(*((long long *)&stdout), 0x1, v4);
        v5 = dcgettext(NULL, "\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n", 5);
        fputs(v5, *((long long *)&stdout));
        v6 = dcgettext(NULL, "\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n", 5);
        fputs(v6, *((long long *)&stdout));
        v7 = dcgettext(NULL, "\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n", 5);
        fputs(v7, *((long long *)&stdout));
        v8 = dcgettext(NULL, "\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n", 5);
        fputs(v8, *((long long *)&stdout));
        v9 = dcgettext(NULL, "\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n", 5);
        fputs(v9, *((long long *)&stdout));
        v10 = dcgettext(NULL, "\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n", 5);
        fputs(v10, *((long long *)&stdout));
        v11 = dcgettext(NULL, "\nOther common options:\n", 5);
        fputs(v11, *((long long *)&stdout));
        v12 = dcgettext(NULL, "      --help                   display this help and exit\n", 5);
        fputs(v12, *((long long *)&stdout));
        v13 = dcgettext(NULL, "      --version                output version information and exit\n\n", 5);
        fputs(v13, *((long long *)&stdout));
        sub_40ce70(0);
        v14 = dcgettext(NULL, "\nUse '-D help' for a description of the options, or see find(1)\n\n", 5);
        fputs(v14, *((long long *)&stdout));
        sub_417130(*((long long *)&stdout), g_43eed0);
        exit(0); /* do not return */
    }
    v3 = dcgettext(NULL, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 0x1, v3);
    exit(a0); /* do not return */
}



// Function: process_debug_options @ 0xd6f0
extern char g_43c140;
extern char *g_43c158;
extern void g_43e660;
extern unsigned int g_43e68c;

void process_debug_options(unsigned long a0)
{
    char *tok;  // rax
    char *tok1;  // r14
    char *v5;  // r12
    char **iter;  // r13
    unsigned long long v7;  // r15
    char *v8;  // rdi
    char *v0;  // [bp-0x50]
    char v1[2];  // [bp-0x42]

    v0 = NULL;
    v1 = 44;
    tok = strtok_r(a0, v1, &v0);
    if (!tok)
    {
        error(0, 0, dcgettext(NULL, "Empty argument to the -D option.", 5));
        sub_40d4e0(1); /* do not return */
    }
    tok1 = tok;
    v5 = "exec";
    do
    {
        iter = &g_43c158;
        v7 = 0;
        v8 = v5;
        while (1)
        {
            if (strcmp(v8, tok1))
            {
                v7 += 1;
                if (v7 != 9)
                {
                    v8 = *(iter);
                    iter += 3;
                }
                else
                {
                    sub_424e50(0, g_43e68c, a0);
                    error(0, 0, dcgettext(NULL, "Ignoring unrecognised debug flag %s", 5));
                    break;
                }
            }
            else
            {
                *((long long *)&g_43e660) = *((long long *)&g_43e660) | *((int *)&(&g_43c140)[8 + 24 * v7]);
                break;
            }
        }
        tok1 = strtok_r(NULL, v1, &v0);
    } while (tok1);
    if (g_43e660 & 16)
    {
        sub_40ce70(1);
        exit(0); /* do not return */
    }
    return;
}



// Function: set_stat_placeholders @ 0xd860
void set_stat_placeholders(void)
{
    return;
}



// Function: check_nofollow @ 0xd870
unsigned int check_nofollow(void)
{
    unsigned int v3;  // ebx
    unsigned long v4;  // cc_ndep
    unsigned long long v5;  // 4114
    unsigned long v6;  // cc_ndep
    unsigned long long v7;  // 4114
    char v0[6][1024];  // [bp-0x1a8]
    char v1[262];  // [bp-0x126]

    if (uname(v0))
        return 1;
    v3 = (int)atof(v1);
    if (!strcmp("Linux", v0))
    {
        v7 = _ccall(3, 0, (unsigned long long)(CmpF((unsigned long long)v3, 1074580685) & 69), 0, v6);
        return (unsigned int)v7 & 1;
    }
    else if (strcmp("FreeBSD", v0))
    {
        return 1;
    }
    else
    {
        v5 = _ccall(3, 0, (unsigned long long)(CmpF((unsigned long long)v3, 1078355558) & 69), 0, v4);
        return (unsigned int)v5 & 1;
    }
}



// Function: complete_pending_execdirs @ 0xd940
extern char g_43e608;

long long complete_pending_execdirs(void)
{
    long long v1;  // rax
    long long v2;  // rax

    if (g_43e608)
    {
        v2 = sub_40cfc0(sub_40b5e0());
        g_43e608 = 0;
        return v2;
    }
    return v1;
}



// Function: record_initial_cwd @ 0xd970
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_43e5c0;

void record_initial_cwd(void)
{
    char *v1;  // rax
    int *err;  // rax

    g_43e5c0 = sub_426990(16);
    if (!sub_4251f0(g_43e5c0))
        return;
    v1 = dcgettext(NULL, "Failed to save initial working directory%s%s", 5);
    err = __errno_location();
    error(1, *(err), v1);
}



// Function: debug_stat @ 0xdb70
extern unsigned long long stderr;
extern void g_43e5f8;
extern unsigned int g_43e668;
extern unsigned long long g_43e6e0;

long long debug_stat(char *a0, long long a1)
{
    unsigned long long v4;  // r12
    char *v0;  // [bp-0x10]
    unsigned long v1;  // [bp-0x8], Other Possible Types: unsigned long long
    char v2;  // [bp+0x0]

    v1 = v4;
    g_43e6e0 = g_43e6e0 + 1;
    __fprintf_chk(stderr, 0x1, "debug_stat (%s)\n", a0);
    if (g_43e668 == 1)
    {
        v0 = &v2;
        return sub_40daad(&v2, a1);
    }
    else if (g_43e668 == 2)
    {
        v0 = &v2;
        return sub_40da0d(&v2, a1);
    }
    else if (g_43e668)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        if (*((int *)&g_43e5f8) >= 0 || *((int *)&g_43e5f8) == 0xffffff9c)
            return fstatat(*((int *)&g_43e5f8), &v2);
        v1 = g_43e668;
        __assert_fail(); /* do not return */
        return fstatat(*((int *)&g_43e5f8), &v2);
    }
}



// Function: following_links @ 0xdc30
extern unsigned int g_43e5e0;
extern unsigned int g_43e668;

unsigned long long following_links(void)
{
    unsigned long long v1;  // rax

    v1 = 1;
    if (g_43e668 != 1)
    {
        v1 = 0;
        if (g_43e668 == 2)
            v1 = _INSERT(g_43e5e0, 0, !g_43e5e0);
    }
    return v1;
}



// Function: default_prints @ 0xdc60
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



// Function: looks_like_expression @ 0xdca0

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



// Function: process_leading_options @ 0xdd00
int process_leading_options(int a0, unsigned long a1)
{
    int iter;  // r12d
    unsigned long idx;  // rax
    unsigned long v3;  // rbx

    if (a0 <= 1)
        return 1;
    iter = 1;
    while (1)
    {
        idx = iter;
        v3 = *((long long *)(a1 + idx * 8));
        if (strcmp("-H", v3))
        {
            if (!strcmp("-L", v3))
            {
                sub_4146f0(1);
            }
            else if (!strcmp("-P", v3))
            {
                sub_4146f0(0);
            }
            else if (!strcmp("--", v3))
            {
                return iter + 1;
            }
            else if (!strcmp("-D", v3))
            {
                iter += 1;
                if (iter >= a0)
                {
                    error(0, 0, dcgettext(NULL, "Missing argument after the -D option.", 5));
                    sub_40d4e0(1); /* do not return */
                }
                sub_40d6f0(*((long long *)(a1 + idx * 8 + 8)));
            }
            else
            {
                if (strncmp("-O", v3, 2))
                    return iter;
                sub_40d220(v3 + 2);
            }
            iter += 1;
            if (a0 <= iter)
                return iter;
        }
        else
        {
            iter += 1;
            sub_4146f0(2);
            if (a0 <= iter)
                return iter;
        }
    }
}



// Function: set_option_defaults @ 0xde90
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

unsigned long long set_option_defaults(struct_0 *idx)
{
    char v1;  // al
    unsigned int v2;  // eax
    unsigned long long v3;  // rax
    unsigned long v4;  // rdx
    char v5;  // 4109
    unsigned long long v6;  // rax

    (&idx->field_c)[1] = getenv("POSIXLY_CORRECT");
    v1 = sub_40d870();
    *((unsigned int *)&idx->padding_48[12]) = 0;
    idx->padding_48[8] = v1;
    v2 = isatty(0);
    *((char *)&idx->field_4 + 7) = 0;
    idx->field_c = v2;
    if ((&idx->field_c)[1])
        idx->field_c = 0;
    *((unsigned long long *)(idx->padding_2)[1]) = 0xffffffffffffffff;
    idx->field_0 = 1;
    v3 = sub_40d3d0();
    *((char *)&idx->field_30) = 0;
    *((unsigned long long *)&idx->padding_12[1]) = v3;
    idx->field_18 = v4;
    idx->field_28 = v4;
    v5 = (&idx->field_c)[1];
    idx->field_20 = v3 - 86400;
    *((unsigned short *)((char *)&idx->field_4 + 5)) = 0;
    *((unsigned long long *)&idx->field_38) = 0;
    *((unsigned int *)((char *)&idx->field_30 + 4)) = (-(v5 < 1) & 0x200) + 0x200;
    *((unsigned short *)&idx->padding_60[0]) = 2;
    if (!getenv("FIND_BLOCK_SIZE"))
    {
        *((char *)&idx->field_4 + 4) = 0;
        v6 = sub_4146f0(0);
        *((unsigned int *)&idx->padding_60[4]) = 8;
        *((unsigned long long *)&idx->field_68) = 0;
        *((char *)&idx->field_70) = 0;
        return v6;
    }
    error(1, 0, dcgettext(NULL, "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable", 5));
}



// Function: is_exec_in_local_dir @ 0xdf90
char is_exec_in_local_dir(unsigned long a0)
{
    return a0 == sub_409530 | a0 == sub_408ff0;
}



// Function: safely_quote_err_filename @ 0xdfb0
void safely_quote_err_filename(unsigned long a0, unsigned long a1)
{
    sub_424e50();
    return;
}



// Function: report_file_err @ 0xdfd0
extern unsigned int g_43e604;
extern char g_43e618;

void report_file_err(int a0, int a1, char a2, unsigned int a3)
{
    if (!a2)
    {
        sub_40dfb0(0, a3);
        error(a0, a1, "%s");
        g_43e604 = 1;
        return;
    }
    if (!g_43e618)
    {
        sub_40dfb0(0, a3);
        error(a0, a1, "%s");
        g_43e604 = 1;
    }
    g_43e618 = 1;
    return;
}



// Function: nonfatal_target_file_error @ 0xe060
void nonfatal_target_file_error(int a0, unsigned int a1)
{
    sub_40dfd0(0, a0, 1, a1);
    return;
}



// Function: get_statinfo @ 0xe080
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern void g_43e5e4;
extern unsigned int g_43e5e8;
extern unsigned int g_43e604;
extern char g_43e62e;
extern struct_1 *g_43e670;
extern unsigned int g_43e68c;

unsigned int get_statinfo(unsigned int a0, long long a1, struct_0 *idx)
{
    unsigned int v1;  // eax
    unsigned int v3;  // edi

    if (g_43e5e4)
    {
        v1 = idx->field_18;
    }
    else if (!g_43e670(a1, idx))
    {
        v1 = idx->field_18;
        if (!v1)
        {
            sub_424e50(0, g_43e68c, a1);
            error(0, 0, dcgettext(NULL, "WARNING: file %s appears to have mode 0000", 5));
            v1 = idx->field_18;
            g_43e604 = 1;
        }
    }
    else
    {
        v3 = *(__errno_location());
        if (g_43e62e && *(__errno_location()) == 2)
            return 0xffffffff;
        sub_40e060(*(__errno_location()), a0);
        return 0xffffffff;
    }
    g_43e5e8 = v1;
    *((unsigned short *)&g_43e5e4) = 0x101;
    return 0;
}



// Function: digest_mode @ 0xe150
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern unsigned short g_43e5e4;
extern char g_43e5e5;
extern unsigned int g_43e5e8;

unsigned int digest_mode(unsigned int *a0, unsigned int a1, long long a2, struct_0 *idx, unsigned int a4)
{
    unsigned int v1;  // r12d
    unsigned int v2;  // ecx
    unsigned int v3;  // edi
    long long v4;  // rsi
    unsigned int v5;  // eax

    if (*(a0))
    {
        if (((unsigned short)*(a0) & 0xf000) != 0xa000 || !(v1 = (unsigned int)sub_40dc30(), (char)v1))
        {
            idx->field_18 = v2;
            g_43e5e5 = 1;
            g_43e5e8 = v2;
            return 1;
        }
        else if (!sub_40e080(v3, v4, idx))
        {
            g_43e5e8 = idx->field_18;
            *(a0) = g_43e5e8;
            g_43e5e5 = 1;
            return v1;
        }
    }
    else
    {
        if ((char)a4)
        {
            g_43e5e8 = 0;
            g_43e5e4 = 0;
            return a4;
        }
        else if (!sub_40e080(a1, a2, idx))
        {
            v5 = idx->field_18;
            *(a0) = v5;
            g_43e5e8 = v5;
            g_43e5e5 = 1;
            return 1;
        }
    }
    return 0;
}



// Function: get_info @ 0xe230
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_43e5e4;
extern char g_43e5e5;
extern long long g_43e5f0;

unsigned long long get_info(unsigned int a0, struct_0 *a1, char *a2)
{
    if ((!a2[26] || g_43e5e4) && (!a2[27] || g_43e5e5))
    {
        if (!a2[28])
        {
            return 0;
        }
        else if (*((long long *)&a1->padding_0[8]) && g_43e5e5 && ((unsigned short)a1->field_18 & 0xf000) != 0x4000)
        {
            return 0;
        }
    }
    return -(0 < sub_40e080(a0, g_43e5f0, a1));
}



// Function: apply_predicate @ 0xe2a0
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
    char padding_20[256];
    unsigned long long field_120;
    unsigned long long field_128;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long apply_predicate(unsigned long long a0, struct_0 *a1, struct_0 *idx)
{
    unsigned long long v3;  // rax
    unsigned long long v0;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    idx->field_120 = idx->field_120 + 1;
    if (0xffffff0000 & idx->field_18)
    {
        v1 = a1;
        v0 = a0;
        if ((unsigned int)sub_40e230(a0, a1, idx))
            return 0;
    }
    v3 = idx->field_0();
    if ((char)v3)
    {
        idx->field_128 = idx->field_128 + 1;
        return v3;
    }
    return 0;
}



// Function: fatal_target_file_error @ 0xe310
void fatal_target_file_error(int a0, unsigned int a1)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40dfd0(1, a0, 1, a1);
    abort(); /* do not return */
}



// Function: nonfatal_nontarget_file_error @ 0xe340
void nonfatal_nontarget_file_error(int a0, unsigned int a1)
{
    sub_40dfd0(0, a0, 0, a1);
    return;
}



// Function: cleanup @ 0xe360
extern long long stdout;
extern long long g_43e610;

void cleanup(void)
{
    unsigned long v1;  // rax
    long long v2;  // rdi
    long long v3;  // rcx
    long long v4;  // r8
    long long v5;  // r9
    long long v6;  // rdx
    long long v7;  // rsi
    int *err;  // rax

    v1 = sub_40b5e0();
    if (v1)
    {
        sub_40cdf0(v1, sub_40d080);
        sub_40d940();
        sub_40e560(g_43e610);
        sub_40cdf0(v1, sub_40ce30);
        sub_40d0f0();
        if ((char)sub_415dd0(v2, v7, v6, v3, v4, v5))
            goto LABEL_40e3e2;
    }
    else
    {
        sub_40e560(g_43e610);
        sub_40d0f0();
        if ((char)sub_415dd0(v2, v7, v6, v3, v4, v5))
        {
LABEL_40e3e2:
            sub_415df0();
            sub_415da0(v2, v7, v6, v3, v4, v5);
        }
    }
    if (sub_417bc0(stdout) == 0xffffffff)
    {
        err = __errno_location();
        sub_40e340(*(err), 4376215);
    }
    return;
}



// Function: fatal_nontarget_file_error @ 0xe410
extern char g_43e618;

void fatal_nontarget_file_error(int a0, unsigned int a1)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    g_43e618 = 0;
    sub_40dfd0(1, a0, 0, a1);
    abort(); /* do not return */
}



// Function: entry_comparator @ 0xe440
unsigned long long entry_comparator(unsigned long long *a0, unsigned long a1)
{
    if (a0[1] == *((long long *)(a1 + 8)))
        return _INSERT(*((long long *)a1), 0, *(a0) == *((long long *)a1));
    return 0;
}



// Function: entry_hashfunc @ 0xe470
void entry_hashfunc(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: entry_free @ 0xe490
typedef struct struct_0 {
    char padding_0[16];
    void* field_10;
    long long field_18;
} struct_0;

void entry_free(struct_0 *ptr)
{
    int *err;  // rax

    if (!ptr->field_18)
    {
        free(ptr->field_10);
        free(ptr);
        return;
    }
    else if (!sub_417850(ptr->field_18))
    {
        free(ptr->field_10);
        free(ptr);
        return;
    }
    else
    {
        err = __errno_location();
        sub_40e410(*(err), *((unsigned int *)&ptr->field_10)); /* do not return */
    }
}



// Function: sharefile_init @ 0xe4d0
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
    v3 = sub_41ac70(11, 0, sub_40e470, sub_40e440, sub_40e490);
    ptr->field_8 = v3;
    if (!v3)
    {
        free(ptr->field_0);
        free(ptr);
        return NULL;
    }
    return ptr;
}



// Function: sharefile_destroy @ 0xe560
typedef struct struct_0 {
    void* field_0;
    long long field_8;
} struct_0;

void sharefile_destroy(struct_0 *a0)
{
    free(a0->field_0);
    sub_41ae30(a0->field_8);
    return;
}



// Function: sharefile_fopen @ 0xe580
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
    v5 = sub_417da0(a1, *(a0));
    ptr1->field_18 = v5;
    v6 = v5;
    if (!v6)
    {
        sub_40e490(ptr1);
        return v6;
    }
    v7 = fileno(v6);
    if (v7 < 0)
        __assert_fail(); /* do not return */
    sub_4174f0(v7, 1);
    if (fstat(v7, &v0) < 0)
    {
        sub_40e490(ptr1);
        return NULL;
    }
    v9 = a0[1];
    ptr1->field_0 = *((unsigned long long *)&v0);
    ptr1->field_8 = v0.st_ino;
    v10 = sub_41aa10(v9, ptr1);
    if (v10)
    {
        sub_40e490(ptr1);
        return v10->field_18;
    }
    v11 = sub_41b290(a0[1], ptr1);
    if (!v11)
    {
        err = __errno_location();
        sub_40e490(ptr1);
        *(err) = *(err);
        return v11;
    }
    return ptr1->field_18;
}



// Function: is_octal_char @ 0xe720
char is_octal_char(char a0)
{
    return a0 - 48 <= 7;
}



// Function: parse_escape_char @ 0xe730
unsigned int parse_escape_char(char a0)
{
    char v1;  // dil
    unsigned int v2;  // eax

    v1 = a0 - 92;
    v2 = 0;
    if (v1 <= 26)
        v2 = *((char *)(v1 + 4381200));
    return v2;
}



// Function: mode_to_filetype @ 0xe750
long long mode_to_filetype(unsigned int a0)
{
    switch (a0)
    {
    case 32768:
        return "f";
    case 16384:
        return "d";
    case 40960:
        return "l";
    case 49152:
        return "s";
    case 24576:
        return "b";
    case 8192:
        return "c";
    case 4096:
        return "p";
    default:
        return "U";
    }
}



// Function: get_format_specifer_length @ 0xe7d0
unsigned int get_format_specifer_length(char a0)
{
    if (!strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%", a0))
        return (strchr("ABCT", a0)) * 2;
    return 1;
}



// Function: get_format_flags_length @ 0xe820
unsigned long long get_format_flags_length(unsigned long a0)
{
    unsigned long long iter;  // r12
    char i;  // bl
    unsigned long long *v3;  // rax

    iter = 0;
    do
    {
        iter += 1;
        i = *((char *)(a0 + iter));
    } while (i && strchr("-+ #", i));
    for (v3 = __ctype_b_loc(); *((char *)(*(v3) + i * 2 + 1)) & 8; i = *((char *)(a0 + iter)))
    {
        iter += 1;
    }
    if (i == 46)
    {
        do
        {
            iter += 1;
        } while (*((char *)(*(v3) + *((char *)(a0 + iter)) * 2 + 1)) & 8);
    }
    return iter;
}



// Function: checked_fprintf @ 0xe8c0
void checked_fprintf(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40e8c7();
    return;
}



// Function: checked_print_quoted @ 0xe9d0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    long long field_20;
} struct_0;

void checked_print_quoted(struct_0 *a0, long long a1, long long a2)
{
    int *err;  // rax

    if (sub_416960(a0->field_8, a0->field_20, a0->field_18, a1, a2) < 0)
    {
        err = __errno_location();
        sub_40e340(*(err), a0->field_10);
    }
    return;
}



// Function: do_time_format @ 0xeb50
typedef struct struct_0 {
    char padding_0[2];
    char field_2;
} struct_0;

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

extern void g_43eb00;
extern struct_0 *g_43eb08;

long long do_time_format(void* ptr, tm *idx, void* a2, unsigned long n)
{
    char *v8;  // rax
    char *ptr1;  // r12
    unsigned long v18;  // rax
    unsigned long v19;  // r9
    unsigned long long v20;  // rax
    unsigned long n1;  // rdx
    unsigned int v11;  // edx
    struct_0 *v12;  // rbp
    unsigned long t;  // rax
    unsigned long v14;  // r13
    unsigned long v15;  // rax
    unsigned long v16;  // rbp
    unsigned long long *v17;  // rax
    unsigned long v0;  // [bp-0x90]
    unsigned long v1;  // [bp-0x88]
    unsigned long v2;  // [bp-0x80]
    int v3;  // [bp-0x78], Other Possible Types: tm
    int v4;  // [bp-0x68]
    int v5;  // [bp-0x58]
    unsigned long v6;  // [bp-0x48]

    v8 = sub_426990(strlen(ptr) + 2);
    *(v8) = 95;
    ptr1 = v8;
    memcpy(ptr1 + 1, ptr, strlen(ptr) + 1);
    v11 = idx->tm_sec;
    v6 = idx->tm_zone;
    v3 = (int)*((int128_t *)&idx->tm_sec);
    v12 = g_43eb08;
    v4 = (int)*((int128_t *)&idx->tm_mon);
    v3 = (tm)(11 <= v11 ? v11 - 11 : v11 + 11);
    v5 = (int)*((int128_t *)&idx->tm_isdst);
    if (!v12)
    {
        *((unsigned long *)&g_43eb00) = 1;
        g_43eb08 = sub_426990(1);
        v12 = g_43eb08;
    }
    while (1)
    {
        t = strftime(v12, *((long long *)&g_43eb00), ptr1, idx);
        if (t && *((long long *)&g_43eb00) > t)
            break;
        g_43eb08 = sub_426ad0(v12, &g_43eb00, 1);
        v12 = g_43eb08;
    }
    v14 = t + n + 1;
    v1 = 0;
    v2 = 0;
    v15 = sub_4269e0(v12, v14);
    *((unsigned long *)&g_43eb00) = v14;
    g_43eb08 = v15;
    v16 = sub_426990(v14);
    strftime(v16, *((long long *)&g_43eb00), ptr1, &v3);
    if ((char)sub_40ea60(g_43eb08, v16, &v1, &v2) && v2 == 2)
    {
        v17 = __ctype_b_loc();
        v18 = v1 + 2;
        v19 = (char *)g_43eb08 + v18;
        if (!(*((char *)(*(v17) + *((char *)v19) * 2 + 1)) & 8))
        {
            v20 = v18 + n;
            n1 = t - v18 + 1;
            if (v20 + n1 != v14)
                __assert_fail(); /* do not return */
            v0 = v19;
            memmove((char *)g_43eb08 + v20, v19, n1);
            memcpy(v0, a2, n);
        }
    }
    free(ptr1);
    free(v16);
    return (char *)&g_43eb08->field_0 + 1;
}



// Function: format_date @ 0xeda0
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

extern void g_43eb20;
extern void g_43ee37;

void* format_date(int a0[2], unsigned long long a1, unsigned int a2)
{
    unsigned long v8;  // rax
    unsigned int v9;  // ebp
    unsigned int v10;  // ebx
    tm *tm;  // rax
    void* v12;  // r12
    void* iter;  // r12
    unsigned long long v14;  // rax
    unsigned long v15;  // r13
    unsigned long v0;  // [bp-0x198]
    int v1[2];  // [bp-0x78]
    unsigned long long v2;  // [bp-0x70]
    char v3;  // [bp-0x64]
    char v4;  // [bp-0x63]
    char v5;  // [bp-0x62]
    char v6[40];  // [bp-0x58]

    v1 = a0;
    v2 = a1;
    if (a2 != 43)
    {
        v8 = a2 - 64;
        v3 = 37;
        v4 = a2;
        v5 = 0;
        if ((unsigned int)v8 > 24 || !(v0 = 18350081, ((char)v8 & 254 | *((char *)((char *)&v0 + ((long long)(v8 & 63) >> 3))) >> (unsigned long long)((char)v8 & 63 & 7) & 1) & 1))
        {
            v6 = (char[40])0;
            v9 = 0;
            v10 = 0;
            if (a2 == 64)
                goto LABEL_40ee0a;
        }
        else
        {
            v9 = 1;
            v10 = __snprintf_chk(v6, 32, 1, 32, ".%09ld0");
            if (a2 == 64)
                goto LABEL_40ee0a;
        }
    }
    else
    {
        strncpy(&v3, "%Y-%m-%d+%T", 11);
        v9 = 1;
        v10 = __snprintf_chk(v6, 32, 1, 32, ".%09ld0");
    }
    tm = localtime(v1);
    if (tm)
    {
        v12 = sub_40eb50(&v3, tm, v6, v10);
        if (v12)
            return v12;
    }
LABEL_40ee0a:
    iter = sub_41b800();
    if (iter <= &g_43eb20)
    {
        __assert_fail(); /* do not return */
    }
    else if (iter < &g_43ee37)
    {
        if (v1 - 0 >> 63 & 1)
        {
            *((char *)iter - 1) = 45;
            iter -= 1;
        }
        if (!v9)
            return iter;
        v14 = iter - &g_43eb20 + strlen(iter);
        if (v14 > 790)
            __assert_fail(); /* do not return */
        v15 = 790 - v14;
        if (strlen(v6) >= v15)
        {
            error(0, 0, "charsprinted=%ld but remaining=%lu: ns_buf=%s");
            if (strlen(v6) >= v15)
                __assert_fail(); /* do not return */
        }
        strcat(iter, v6);
        return iter;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: ctime_format @ 0xf050
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

extern void g_43c220;
extern void g_43c280;
extern char g_43e700;

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
        return sub_40eda0(v0, a1, 64);
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
                else if (__snprintf_chk(&g_43e700, 0x400, 1, 0x400, "%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d", *((long long *)&(&g_43c280)[8 * v3]), *((long long *)&(&g_43c220)[8 * v4]), tm->tm_mday, v5, v6, v7, a1, tm->tm_year + 1900) <= 0x3ff)
                {
                    return &g_43e700;
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



// Function: do_fprintf @ 0xf310
typedef struct struct_1 {
    char padding_0[8];
    long long field_8;
    unsigned long long field_10;
} struct_1;

typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
    char padding_24[12];
    long long field_30;
    char padding_38[8];
    long long field_40;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern long long g_42f827;
extern void g_43e5e0;
extern long long g_43e5f0;
extern unsigned int g_43e5f8;
extern int g_43e5fc;
extern unsigned int g_43e604;
extern struct_2 *g_43e680;

void do_fprintf(struct_1 *a0, void* index, unsigned long a2, struct_0 *idx, unsigned int a4, unsigned int a5)
{
    unsigned int v8;  // eax
    unsigned int *v9;  // rax
    unsigned long v17;  // fs
    unsigned int v18;  // edi
    unsigned int v19;  // rcx
    unsigned int v20;  // r8
    unsigned int v21;  // r9
    void* ptr;  // rbp
    unsigned int v23;  // rcx
    unsigned int v24;  // r8
    unsigned int v25;  // r9
    long long v10;  // rax
    unsigned int v26;  // rcx
    unsigned int v27;  // r8
    unsigned int v28;  // r9
    unsigned long long v29;  // rax
    void* v30;  // rbp
    int *err;  // rax
    long long v11;  // rax
    void* iter;  // rax
    char *ptr1;  // rax
    char *v34;  // rax
    int *err1;  // rax
    long long v36;  // rax
    int *err2;  // rax
    unsigned long v38;  // rsi
    int v39;  // eax
    unsigned int v12;  // rdx
    unsigned int v13;  // rcx
    unsigned int v40;  // rdx
    unsigned int v41;  // rcx
    unsigned int v42;  // r8
    unsigned int v43;  // r9
    unsigned int v14;  // r8
    unsigned long long v45;  // rbx
    int *v46;  // rax
    unsigned int v15;  // r9
    char *v16;  // rdx
    char v0;  // [bp-0x360]
    char v1;  // [bp-0x358]
    unsigned int v2;  // [bp-0x340]
    char v3;  // [bp-0x2c8]
    char v4;  // [bp-0x2be]
    unsigned long v5;  // [bp-0x30]
    unsigned long long v6;  // [bp-0x8]

    v8 = *((int *)index);
    if (v8 == 1)
    {
        sub_40ea10((long long)index[8], (int)index[16], a0);
        if (v5 != *((long long *)(40 + v17)))
            __stack_chk_fail(); /* do not return */
        v6 = v45;
        if (!sub_417bc0(a0->field_8))
            return;
        v46 = __errno_location();
        sub_40e340(*(v46), a0->field_10);
        return;
    }
    else if (v8 != 2)
    {
        if (!v8)
            goto LABEL_40f378;
        goto LABEL_40f34a;
    }
    else
    {
        switch ((char)index[4])
        {
        case 0:
            error(1, 0, dcgettext(NULL, "error: %s at end of format string", 5));
        case 37:
LABEL_40f378:
            sub_40ea10();
            return;
        case 68:
            v10 = sub_41b800(idx->field_0, &v3, 0, 1, 1);
            break;
        case 70:
            v11 = sub_4104c0(idx, a2);
            if (v5 != *((long long *)(40 + v17)))
                __stack_chk_fail(); /* do not return */
            sub_40e9d0(a0, (long long)index[8], v11);
            return;
        case 72:
            ptr = sub_426990(g_43e5fc + 1);
            memcpy(ptr, a2, g_43e5fc);
            *((char *)ptr + g_43e5fc) = 0;
            sub_40e8c0(a0, (long long)index[8], ptr, v23, v24, v25);
            if (v5 == *((long long *)(40 + v17)))
            {
                free(ptr);
                return;
            }
            goto LABEL_40f55e;
        case 77:
            sub_417d90(idx, &v3);
            v4 = 0;
            sub_40e8c0(a0, (long long)index[8], &v3, v19, v20, v21);
            goto LABEL_40f34a;
        case 80:
            v11 = &g_42f827;
            if (*((int *)&g_43e5e0) > 0)
            {
                v16 = g_43e5fc + a2;
                v11 = &v16[*(v16) == 47];
            }
            if (v5 == *((long long *)(40 + v17)))
            {
                sub_40e9d0(a0, (long long)index[8], v11);
                return;
            }
            goto LABEL_40f55e;
        case 83:
            sub_40f2c0(idx->field_30, idx->field_40);
            sub_40e8c0(a0, (long long)index[8], v12, v13, v14, v15);
            return;
        case 85:
LABEL_40f415:
            v10 = sub_41b800(idx->field_1c, &v3, 0, 1, 1);
            break;
        case 89:
            v18 = idx->field_18 & 0xf000;
            if (v18 != 0xa000)
            {
LABEL_40f571:
                v36 = sub_40e750(v18);
                if (v5 == *((long long *)(40 + v17)))
                {
                    sub_40e8c0(a0, (long long)index[8], v36, v41, v42, v43);
                    return;
                }
                __stack_chk_fail(); /* do not return */
            }
            if (fstatat(g_43e5f8, g_43e5f0, &v1, 0))
            {
                err2 = __errno_location();
                v38 = (long long)index[8];
                switch (v39)
                {
                case 2: case 20:
                    sub_40e8c0(a0, v38, "N", v19, v20, v21);
                    break;
                case 40:
                    sub_40e8c0(a0, v38, "L", v19, v20, v21);
                    break;
                default:
                    sub_40e8c0(a0, v38, "?", v19, v20, v21);
                    sub_40dfb0(0, a2);
                    error(0, *(err2), "%s");
                    break;
                }
                goto LABEL_40f34a;
            }
            else
            {
                v10 = sub_40e750(v2 & 0xf000);
                break;
            }
        case 90:
            if (g_43e680(g_43e5f8, g_43e5f0, &v0) >= 0)
            {
                sub_40e8c0(a0, (long long)index[8], v0, v19, v20, v21);
                freecon(v0);
                goto LABEL_40f34a;
            }
            else
            {
                sub_40e8c0(a0, (long long)index[8], &g_42f827, v19, v20, v21);
                sub_40dfb0(0, a2);
                v34 = dcgettext(NULL, "getfilecon failed: %s", 5);
                err1 = __errno_location();
                error(0, *(err1), v34);
                g_43e604 = 1;
                goto LABEL_40f34a;
            }
        case 97:
            v29 = sub_425c40(idx);
            goto LABEL_40f9a7;
        case 98:
            v10 = sub_41b800(idx->field_40, &v3, 0, 0x200, 0x200);
            break;
        case 99:
            v29 = sub_425c50(idx);
LABEL_40f9a7:
            v36 = sub_40f050(v29, v40);
            if (v5 == *((long long *)(40 + v17)))
            {
                sub_40e8c0(a0, (long long)index[8], v36, v41, v42, v43);
                return;
            }
LABEL_40f55e:
            __stack_chk_fail(); /* do not return */
        case 100:
            if (v5 != *((long long *)(40 + v17)))
                __stack_chk_fail(); /* do not return */
            sub_40e8c0(a0, (long long)index[8], *((int *)&g_43e5e0), idx, a4, a5);
            return;
        case 102:
            v30 = sub_4176f0(a2);
LABEL_40f84e:
            sub_40e9d0(a0, (long long)index[8], v30);
            goto LABEL_40fa88;
        case 104:
            v30 = sub_426d30(a2);
            iter = v30 + strlen(v30) - 1;
            if (v30 <= iter)
            {
                do
                {
                    if (*((char *)iter) != 47 && v30 < iter && (char)iter[1] == 47)
                    {
                        *((char *)&iter[1]) = 0;
                        break;
                    }
                } while ((iter -= 1, v30 <= iter));
            }
            ptr1 = strrchr(v30, 47);
            if (ptr1)
            {
                *(ptr1) = 0;
                goto LABEL_40f84e;
            }
            else
            {
                sub_40e9d0(a0, (long long)index[8], ".");
LABEL_40fa88:
                if (v5 == *((long long *)(40 + v17)))
                {
                    free(v30);
                    return;
                }
            }
        case 105:
            v10 = sub_41b800(idx->field_8, &v3, 0, 1, 1);
            break;
        case 107:
            v10 = sub_41b800(idx->field_40, &v3, 0, 0x200, 0x400);
            break;
        case 108:
            if (((unsigned short)idx->field_18 & 0xf000) != 0xa000)
            {
LABEL_40f779:
                v30 = NULL;
                sub_40e9d0(a0, (long long)index[8], &g_42f827);
                goto LABEL_40fa88;
            }
            else
            {
                v30 = sub_417190(g_43e5f8, g_43e5f0);
                if (!v30)
                {
                    err = __errno_location();
                    sub_40e060(*(err), a2);
                    g_43e604 = 1;
                    goto LABEL_40f779;
                }
            }
        case 109:
            if (v5 != *((long long *)(40 + v17)))
                __stack_chk_fail(); /* do not return */
            sub_40e8c0(a0, (long long)index[8], idx->field_18 & 0xfff, idx, a4, a5);
            return;
        case 110:
            v10 = sub_41b800(idx->field_10, &v3, 0, 1, 1);
            break;
        case 112:
            if (v5 != *((long long *)(40 + v17)))
                __stack_chk_fail(); /* do not return */
            sub_40e9d0(a0, (long long)index[8], a2);
            return;
        case 115:
            v10 = sub_41b800(idx->field_30, &v3, 0, 1, 1);
            break;
        case 116:
            v29 = sub_425c60(idx);
            goto LABEL_40f9a7;
        case 117:
            v9 = getpwuid(idx->field_1c);
            if (!v9)
                goto LABEL_40f415;
LABEL_40f659:
            *((char *)((long long)index[8] + (int)index[16])) = 115;
            if (v5 == *((long long *)(40 + v17)))
            {
                sub_40e8c0(a0, (long long)index[8], *((long long *)&v9), v26, v27, v28);
                return;
            }
            goto LABEL_40f55e;
        case 121:
            v18 = idx->field_18 & 0xf000;
            goto LABEL_40f571;
        case 103:
            v9 = getgrgid(idx->field_20);
            if (v9)
                goto LABEL_40f659;
            break;
        case 71:
            v10 = sub_41b800(idx->field_20, &v3, 0, 1, 1);
            break;
        default:
LABEL_40f34a:
            return;
        }
        sub_40e8c0(a0, (long long)index[8], v10, v19, v20, v21);
        goto LABEL_40f34a;
    }
}



// Function: pred_fprintf @ 0xfad0
typedef struct struct_1 {
    char padding_0[8];
    long long field_8;
    unsigned long long field_10;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char field_4;
    char field_5;
    char padding_6[2];
    unsigned long long field_8;
    char padding_10[8];
    struct struct_0 *field_18;
} struct_0;

extern unsigned int g_42f827;

unsigned int pred_fprintf(unsigned int a0, unsigned long a1, struct_1 *a2, unsigned long a3, unsigned int a4, unsigned int a5)
{
    struct_0 *v1;  // rbx
    struct_1 *v2;  // r13
    unsigned long v10;  // rdx
    void* v11;  // rax
    unsigned int v12;  // rcx
    struct_0 *idx;  // rbx
    unsigned int v13;  // r8d
    unsigned int v14;  // r9d
    char v4;  // bpl
    unsigned long v6;  // rax
    unsigned long v7;  // rsi

    v1 = a2[2].field_8;
    if (!a2[2].field_8)
        return 1;
    v2 = &a2[2].field_8;
    while (1)
    {
        while (1)
        {
            idx = v1;
            if (idx->field_0 == 2 && !(v4 = idx->field_5, !idx->field_5))
                break;
            sub_40f310(v2, idx, a0, a1, v13, v14);
            v1 = idx->field_18;
            if (!idx->field_18)
                return 1;
        }
        switch (idx->field_4)
        {
        case 67:
            v6 = sub_425c50(a1);
            v7 = v10;
            v11 = sub_40eda0(v6, v7, v4);
            sub_40e8c0(v2, idx->field_8, v11, v12, v13, v14);
            goto LABEL_40fb4f;
        case 84:
            v6 = sub_425c60(a1);
            v7 = v10;
            break;
        case 65:
            v6 = sub_425c40(a1);
            v7 = v10;
            break;
        case 66:
            v6 = sub_425c70(a1);
            v7 = v10;
            if (v4 == 64 || (v7 = v10, v7 >= 0))
                break;
            sub_40e8c0(v2, idx->field_8, &g_42f827, v12, v13, v14);
LABEL_40fb4f:
            v1 = idx->field_18;
            if (!idx->field_18)
                return 1;
            continue;
        default:
            __assert_fail(); /* do not return */
        }
    }
}



// Function: make_segment @ 0xfc20
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
        index = sub_426990(32);
        *(a0) = index;
        index->field_0 = a3;
        index->field_4 = a4;
        index->field_5 = a5;
        index->field_18 = 0;
        index->field_10 = v1;
        v3 = sub_426990(v1 + 2);
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
LABEL_40fd69:
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
                    goto LABEL_40fd69;
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



// Function: insert_fprintf @ 0xfef0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;


unsigned int insert_fprintf(struct_0 *idx, struct_0 *a1, char *a2)
{
    char *iter;  // r15
    void* idx1;  // rbx
    unsigned long long v5;  // rax
    void* v7;  // r13
    char *v8;  // r12

    iter = a2;
    idx1 = (unsigned long long)sub_40d450(a1, sub_40fad0, *((unsigned int *)&a2));
    *((int128_t *)&idx1[56]) = *((int128_t *)idx->padding_0);
    *((int128_t *)&idx1[72]) = *((int128_t *)idx[1].padding_0);
    v5 = (unsigned long long)idx[2].padding_0;
    *((unsigned int *)&idx1[24]) = 0x101;
    *((unsigned long long *)&idx1[88]) = v5;
    *((unsigned int *)&idx1[32]) = 0;
    *((unsigned long *)&idx1[56]) = 0;
    if (!*(iter))
        return 1;
    v7 = idx1 + 56;
    v8 = iter;
}



// Function: free_file_system_list @ 0x102e0
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

void free_file_system_list(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    struct_0 *v3;  // rbx
    struct_0 *v4;  // rbx
    unsigned long long v0;  // [bp-0x8]

    if (!a0)
        return;
    v0 = v2;
    do
    {
        v3 = a0;
        v4 = v3->field_30;
        sub_41dc40(v3);
        a0 = v4;
    } while (v3->field_30);
    return;
}



// Function: get_file_system_list @ 0x10310
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

extern char g_43ee38;
extern struct_0 *g_43ee40;

struct_0 * get_file_system_list(char a0)
{
    struct_0 *v1;  // rax
    unsigned long v2;  // rax

    v1 = g_43ee40;
    if (v1)
    {
        if (g_43ee38 != 1)
        {
            if (!a0)
                return v1;
            sub_4102e0(v1);
            g_43ee40 = 0;
        }
        else
        {
            return v1;
        }
    }
    v2 = sub_41dc90(a0);
    g_43ee38 = a0;
    g_43ee40 = v2;
    return g_43ee40;
}



// Function: set_fstype_devno @ 0x10360
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_43e670;

unsigned int set_fstype_devno(unsigned long long *a0)
{
    unsigned int v2;  // eax
    char v0;  // [bp-0xb8]

    if (a0[4] != 0xffffffffffffffff)
        return 0;
    sub_40d860();
    v2 = g_43e670(a0[1], &v0);
    if (v2)
        return 0xffffffff;
    a0[4] = v0;
    return v2;
}



// Function: filesystem_type @ 0x104c0
extern unsigned long long g_43ee48;
extern char g_43ee50;
extern void* g_43ee58;

void* filesystem_type(unsigned long long *a0)
{
    if (g_43ee58)
    {
        if (g_43ee50 && *(a0) == g_43ee48)
            return g_43ee58;
        free(g_43ee58);
    }
    g_43ee48 = *(a0);
    g_43ee58 = sub_4103e0(a0);
    return g_43ee58;
}



// Function: is_used_fs_type @ 0x10510
void is_used_fs_type(long long a0)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_410523("afs", a0);
    return;
}



// Function: get_mounted_devices @ 0x10580
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    char padding_28[8];
    struct struct_0 *field_30;
} struct_0;

long long get_mounted_devices(unsigned long long *a0)
{
    struct_0 *v4;  // r14
    unsigned long v5;  // r15
    unsigned long long v6;  // r12
    struct_0 *idx;  // r14
    unsigned long long v8;  // rbx
    unsigned long long v9;  // rbp
    unsigned long v10;  // r15
    unsigned long v0;  // [bp-0x60]
    struct_0 *v1;  // [bp-0x58]
    unsigned long v2;  // [bp-0x48]

    v2 = 0;
    v1 = sub_41dc90(0);
    if (!v1)
    {
        sub_4102e0(NULL);
        return 0;
    }
    v4 = v1;
    v5 = 0;
    v6 = 0;
    while (1)
    {
        idx = v4;
        v8 = v6 + 1;
        v9 = v8 * 8;
        v0 = sub_415710(v5, v9, &v2);
        if (v0)
        {
            v5 = v0;
            if (!sub_410360(idx))
            {
                v6 = v8;
                *((unsigned long long *)(v0 + v9 - 8)) = idx->field_20;
            }
            v4 = idx->field_30;
            v10 = v5;
            if (!idx->field_30)
                break;
        }
        else
        {
            v10 = 0;
            free(v5);
            v4 = idx->field_30;
            v5 = v10;
            if (!v4)
                break;
        }
    }
    sub_4102e0(v1);
    if (!v10)
        return v10;
    *(a0) = v6;
    return v10;
}



// Function: get_noop @ 0x10690
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

extern char g_43cd20;
extern struct_0 g_43cd40;
extern unsigned long long g_43ee68;

unsigned long long get_noop(void)
{
    struct_0 *v1;  // rax
    int v2;  // edx
    struct_0 *v3;  // rax

    v1 = &g_43cd40.field_0;
    v2 = 0;
    if (g_43ee68)
        return g_43ee68;
    while (1)
    {
        v3 = v1;
        v2 += 1;
        if (!v3->field_8)
            return v3->field_8;
        v1 = v3 + 2;
        if (v3->field_0 == 1)
        {
            g_43ee68 = &(&g_43cd20)[32 * v2];
            return &(&g_43cd20)[32 * v2];
        }
    }
}



// Function: should_issue_warnings @ 0x106f0
extern char g_43e630;
extern char g_43e631;

unsigned int should_issue_warnings(void)
{
    unsigned int v1;  // eax

    v1 = 0;
    if (!g_43e631)
        v1 = g_43e630;
    return v1;
}



// Function: estimate_file_age_success_rate @ 0x10710
double estimate_file_age_success_rate(void)
{
    unsigned int v1;  // ymm0
    unsigned long long v2;  // xmm1lq

    if (((CmpF(1036831949, v1) & 69 | (char)((CmpF(1036831949, v1) & 69) >> 6)) & 1) != 1)
        return 1008981770;
    v2 = 1050253722;
    if (((CmpF(0x3f800000, v1) & 69 | (char)((CmpF(0x3f800000, v1) & 69) >> 6)) & 1) == 1 && ((CmpF(v1, 0x42c80000) & 69 | (char)((CmpF(v1, 0x42c80000) & 69) >> 6)) & 1) == 1)
        v2 = 1053273620;
    return v2;
}



// Function: estimate_timestamp_success_rate @ 0x10760
long long estimate_timestamp_success_rate(unsigned long a0)
{
    return sub_410710();
}



// Function: collect_arg_nonconst @ 0x10790
unsigned int collect_arg_nonconst(unsigned long long a0, int *a1, unsigned long long *a2)
{
    unsigned long idx;  // rcx

    if (a0)
    {
        idx = *(a1);
        if (*((long long *)(a0 + idx * 8)))
        {
            *(a2) = *((long long *)(a0 + idx * 8));
            *(a1) = (unsigned int)idx + 1;
            return 1;
        }
    }
    *(a2) = 0;
    return 0;
}



// Function: collect_arg @ 0x107d0
unsigned int collect_arg(unsigned long long a0, int *a1, unsigned long a2)
{
    unsigned int v2;  // eax
    unsigned long long *v3;  // r8
    unsigned long long v0;  // [bp-0x18]

    v2 = sub_410790(a0, a1, &v0);
    *(v3) = v0;
    return v2;
}



// Function: parse_files0_from @ 0x10810
extern unsigned long long g_43e690;

unsigned int parse_files0_from(unsigned long a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    char v0;  // [bp-0x18]

    v2 = sub_4107d0(a1, a2, &v0);
    if ((char)v2)
        g_43e690 = v0;
    return v2;
}



// Function: get_comp_type @ 0x10860
typedef struct struct_0 {
    char field_0;
} struct_0;

unsigned int get_comp_type(struct_0 **a0, unsigned int *a1)
{
    char *v1;  // rax

    v1 = &*(a0)->field_0;
    if (*(v1) == 43)
    {
        *(a1) = 0;
        *(a0) = v1 + 1;
        return 1;
    }
    if (*(v1) != 45)
    {
        *(a1) = 2;
    }
    else
    {
        *(a1) = 1;
        *(a0) = v1 + 1;
    }
    return 1;
}



// Function: optionh_getfilecon @ 0x109e0
extern unsigned int g_43e5e0;

unsigned long long optionh_getfilecon(unsigned long a0, unsigned long a1, unsigned long a2)
{
    if (!g_43e5e0 && !sub_4252a0())
        return 0;
}



// Function: optionl_getfilecon @ 0x10a40
unsigned int optionl_getfilecon(unsigned long a0, unsigned long a1, unsigned long a2)
{
    if (!sub_4252a0())
        return 0;
}



// Function: parse_version @ 0x10a80
extern char g_43e678;
extern unsigned short g_43e688;

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
    sub_415e60("find");
    __printf_chk(1, dcgettext(NULL, "Features enabled: ", 5));
    __printf_chk(1, "D_TYPE ");
    v7 = "enabled";
    if (!g_43e678)
        v7 = "disabled";
    __printf_chk(1, "O_NOFOLLOW(%s) ", v7);
    __printf_chk(1, "LEAF_OPTIMISATION ");
    if (is_selinux_enabled(v8, v9, v10, v11, v12, v13) > 0)
        __printf_chk(1, "SELINUX ");
    v0 = 0;
    if ((char)sub_4086d0(&v0))
    {
        __printf_chk(1, "FTS(");
        if (*((char *)((void*)&v0 + 1)) & 2)
            __printf_chk(1, "FTS_CWDFD");
        __printf_chk(1, ") ");
    }
    __printf_chk(1, "CBO(level=%d) ", g_43e688);
    __printf_chk(1, "\n");
    exit(0); /* do not return */
}



// Function: parse_true @ 0x10bf0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_true(struct_0 *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40d4d0(a0);
    *((unsigned short *)((char *)&idx[1].field_8 + 2)) = 0;
    *((unsigned int *)&idx[2].padding_0[4]) = 0x3f800000;
    return 1;
}



// Function: parse_quit @ 0x10c20
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_quit(struct_0 *a0)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_40d4d0(a0);
    idx[6] = 1;
    idx[9] = 0x3f800000;
    return 1;
}



// Function: parse_prune @ 0x10c50
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern char g_43e620;

unsigned int parse_prune(struct_0 *a0)
{
    unsigned short *idx;  // rax

    idx = (unsigned long long)sub_40d4d0(a0);
    if (!g_43e620)
        idx[13] = 0;
    idx[12] = 1;
    return 1;
}



// Function: parse_nouser @ 0x10c80
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_nouser(struct_0 *a0)
{
    *((unsigned int *)((unsigned long long)sub_40d4d0(a0) + 36)) = 981668463;
    return 1;
}



// Function: parse_accesscheck @ 0x10ca0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_accesscheck(struct_0 *a0)
{
    struct_0 *idx;  // rax
    unsigned int v2;  // ymm0

    idx = (unsigned long long)sub_40d4d0(a0);
    v2 = 1045220557;
    *((unsigned int *)&idx[1].field_8) = 0;
    if (idx->padding_0 != sub_409640)
        v2 = 1063675494;
    *((unsigned int *)&idx[2].padding_0[4]) = v2;
    return 1;
}



// Function: parse_empty @ 0x10ce0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_empty(struct_0 *a0)
{
    *((unsigned int *)((unsigned long long)sub_40d4d0(a0) + 36)) = 1008981770;
    return 1;
}



// Function: parse_delete @ 0x10d00
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern char g_43e620;

unsigned int parse_delete(struct_0 *a0)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_40d4d0(a0);
    g_43e620 = 0;
    idx[6] = 0x101;
    idx[9] = 0x3f800000;
    return 1;
}



// Function: parse_xtype @ 0x10fe0
void parse_xtype(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: fnmatch_sanitycheck @ 0x11020
extern char g_43ee60;

unsigned int fnmatch_sanitycheck(void)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    if (!g_43ee60)
    {
        v0 = v2;
        return sub_411042();
    }
    return 1;
}



// Function: estimate_pattern_match_rate @ 0x110c0
void estimate_pattern_match_rate(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_4110ce();
    return;
}



// Function: is_feasible_path_argument @ 0x11130
void is_feasible_path_argument(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_41113c();
    return;
}



// Function: parse_wholename @ 0x11280
void parse_wholename(void)
{
}



// Function: parse_user @ 0x11300
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

extern unsigned int g_43e68c;

int parse_user(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    char *v3;  // r13
    passwd *v4;  // rbx
    unsigned int v5;  // ebx
    unsigned long v6;  // rax
    unsigned int *idx;  // rax
    unsigned int v8;  // ymm0
    char v0;  // [bp-0x38]

    v2 = sub_4107d0(a1, a2, &v0);
    if (!(char)v2)
        return v2;
    v3 = (char *)v0;
    v4 = getpwnam(v3);
    endpwent();
    if (v4)
    {
        v5 = v4->pw_uid;
    }
    else
    {
        v6 = strspn(v3, "0123456789");
        if (!(!v6 || v3[v6]))
        {
            v5 = sub_416f40(v3, g_43e68c);
        }
        else if (*(v3))
        {
            sub_424e50(0, g_43e68c, v3);
            error(1, 0, dcgettext(NULL, "%s is not the name of a known user", 5));
        }
        else
        {
            error(1, 0, dcgettext(NULL, "The argument to -user should not be empty", 5));
        }
    }
    idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v3));
    v8 = 1065185444;
    idx[14] = v5;
    if (v5 > 99)
        v8 = 1045220557;
    idx[9] = v8;
    return v2;
}



// Function: parse_nogroup @ 0x11460
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_nogroup(struct_0 *a0)
{
    *((unsigned int *)((unsigned long long)sub_40d490(a0, 0) + 36)) = 953267991;
    return 1;
}



// Function: parse_lname @ 0x11480
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_lname(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    struct_0 *idx;  // rax
    int v4;  // xmm0
    unsigned int v0;  // [bp-0x38]

    sub_411020();
    v2 = sub_4107d0(a1, a2, &v0);
    if ((char)v2)
    {
        idx = (unsigned long long)sub_40d490(a0, v0);
        idx[3].field_8 = v0;
        sub_4110c0(v0, 0);
        *((unsigned int *)&idx[2].padding_0[4]) = *((unsigned int *)&MulV(v4, 1036831949));
        return v2;
    }
    return v2;
}



// Function: parse_ilname @ 0x11520
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_ilname(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    struct_0 *idx;  // rax
    int v4;  // xmm0
    char v0;  // [bp-0x28]

    v2 = sub_4107d0(a1, a2, &v0);
    if ((char)v2)
    {
        idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v0));
        idx[3].field_8 = v0;
        sub_4110c0(v0, 0);
        *((unsigned int *)&idx[2].padding_0[4]) = *((unsigned int *)&MulV(v4, 1036831949));
        return v2;
    }
    return v2;
}



// Function: parse_context @ 0x115b0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_context(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    struct_0 *idx;  // rax
    unsigned long index;  // rdx
    unsigned long v7;  // rdx
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!a1)
        return 0;
    v1 = v3;
    v0 = v4;
    if (!*((long long *)(a1 + *(a2) * 8)))
        return 0;
    if (is_selinux_enabled() > 0)
    {
        idx = (unsigned long long)sub_40d490(a0, 0);
        index = *(a2);
        *((unsigned int *)&idx[2].padding_0[4]) = 1008981770;
        v7 = *((long long *)(a1 + index * 8));
        *((char *)&idx[1].field_8 + 2) = 0;
        idx[3].field_8 = v7;
        *(a2) = *(a2) + 1;
        return 1;
    }
    error(1, 0, dcgettext(NULL, "invalid predicate -context: SELinux is not enabled.", 5));
}



// Function: get_relative_timestamp @ 0x11650
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    long long field_8;
    unsigned long long field_10;
} struct_0;

extern char strtod;

unsigned int get_relative_timestamp(struct struct_0 *a0[2], struct_0 *idx, long long a2)
{
    unsigned int v4;  // eax
    unsigned long long v5;  // xmm0lq
    int v6;  // xmm0
    int v7;  // xmm2
    long long v8;  // rax
    unsigned long v9;  // rcx
    int v10;  // xmm1
    unsigned long v11;  // rdx
    char *v12;  // r8
    struct struct_0 *v0[2];  // [bp-0x60]
    char v1;  // [bp-0x50]
    unsigned long long v2;  // [bp-0x48]

    v0 = a0;
    if (!(char)sub_410860(v0, &idx->field_4))
        return 0;
    if (!idx->field_4)
    {
        idx->field_4 = 1;
    }
    else if (idx->field_4 == 1)
    {
        idx->field_4 = 0;
    }
    v4 = sub_426d90(v0, 0, &v1, &strtod);
    if ((char)v4)
    {
        modf(&v2);
        v6 = (int)(MulV(MulV(v5, v1), 0x41cdcd6500000000));
        if (((CmpF(0x41cdcd6500000000, *((unsigned long long *)&v6)) & 69 | (char)((CmpF(0x41cdcd6500000000, *((unsigned long long *)&v6)) & 69) >> 6)) & 1) == 1)
            __assert_fail(); /* do not return */
        v7 = (int)v2;
        v8 = *((unsigned long long *)&SubV(a2, v7));
        idx->field_8 = v8;
        if (a2 < v8 == !((CmpF(0, *((unsigned long long *)&v7)) & 69 | (CmpF(0, *((unsigned long long *)&v7)) & 69) >> 6) & 1))
        {
            v10 = (int)v9;
            v11 = *((unsigned long long *)&SubV(v10, v6));
            if (((CmpF(*((unsigned long long *)&v6), *((unsigned long long *)&v10)) & 69 | (char)((CmpF(*((unsigned long long *)&v6), *((unsigned long long *)&v10)) & 69) >> 6)) & 1) == 1)
            {
                idx->field_10 = v11;
                return v4;
            }
            idx->field_10 = v11 + 1000000000;
            idx->field_8 = v8 - 1;
            return v4;
        }
        error(1, 0, v12);
    }
    return 0;
}



// Function: parse_used @ 0x11800
typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

int parse_used(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v4;  // r12d
    unsigned long v5;  // r13
    unsigned int v6;  // r12d
    struct_0 *idx;  // rax
    struct_1 *v8;  // rcx
    unsigned long v0;  // [bp-0x40]
    struct_0 v1;  // [bp-0x38]
    unsigned long v2;  // [bp-0x28]

    v4 = sub_4107d0(a1, a2, &v0);
    if (!(char)v4)
        return v4;
    v5 = v0;
    v6 = sub_411650(v5, &v1, 0);
    if ((char)v6)
    {
        idx = (unsigned long long)sub_40d490(a0, v5);
        idx[56] = v1;
        *((unsigned long *)&idx[72]) = v2;
        v8->field_24 = (int)sub_410710();
        return v6;
    }
    error(1, 0, dcgettext(NULL, "Invalid argument %s to -used", 5));
}



// Function: parse_mmin @ 0x11a10
void parse_mmin(void)
{
}



// Function: get_stat_Ytime @ 0x11a40
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
} struct_0;

unsigned long long get_stat_Ytime(unsigned long a0, char a1, struct_0 *idx)
{
    unsigned long long v1;  // rdx
    unsigned long long v2;  // rdx
    unsigned long long v3;  // rdx
    unsigned long long v4;  // rdx

    switch (a1)
    {
    case 99:
        idx->field_0 = sub_425c50();
        idx->field_8 = v4;
        return 1;
    case 109:
        idx->field_0 = sub_425c60();
        idx->field_8 = v3;
        return 1;
    case 66:
        idx->field_0 = sub_425c70();
        idx->field_8 = v2;
        return ~(idx->field_8) >> 63;
    case 97:
        idx->field_0 = sub_425c40();
        idx->field_8 = v1;
        return 1;
    default:
        __assert_fail(); /* do not return */
    }
}



// Function: get_num @ 0x11ae0
unsigned int get_num(char *a0, unsigned long a1, unsigned int *a2)
{
    unsigned long long v3;  // rax
    char *v0;  // [bp-0x20]
    char v1;  // [bp-0x18]

    v0 = a0;
    if (a0 && (char)sub_410860(&v0, a2))
    {
        v3 = sub_427300(v0, &v1, 10);
        return _INSERT(v3, 0, !(unsigned int)v3);
    }
    return 0;
}



// Function: insert_num @ 0x11b60
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char padding_10[40];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
} struct_0;

extern unsigned long long stderr;
extern char g_43e660;

struct_0 * insert_num(unsigned long long a0, int *a1, struct_0 *a2)
{
    struct_0 *idx;  // r12
    char *v5;  // r8
    char *v6;  // rcx
    unsigned int v0;  // [bp-0x34]
    char *v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x28]

    if (!(char)sub_4107d0(a0, a1, &v1))
    {
        return NULL;
    }
    else if (!(char)sub_411ae0(v1, &v2, &v0))
    {
        return NULL;
    }
    else
    {
        idx = (unsigned long long)sub_40d490(a2, *((unsigned int *)&v1));
        idx->field_38 = v0;
        *((unsigned long long *)&idx->field_40) = v2;
        if (!(g_43e660 & 1))
            return idx;
        __fprintf_chk(stderr, 0x1, "inserting %s\n", idx->field_8);
        v5 = " >";
        v6 = "gt";
        if (v0)
        {
            v5 = " <";
            v6 = "lt";
            if (v0 != 1)
            {
                v5 = " ?";
                if (v0 == 2)
                    v5 = " =";
                v6 = "?";
                if (v0 == 2)
                    v6 = "eq";
            }
        }
        __fprintf_chk(stderr, 0x1, "    type: %s    %s  ", v6, v5);
        __fprintf_chk(stderr, 0x1, "%lu\n", *((long long *)&idx->field_40));
        return idx;
    }
}



// Function: parse_links @ 0x11cb0
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char padding_10[40];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
} struct_0;

unsigned int parse_links(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    struct_0 *idx;  // rax

    idx = sub_411b60(a1, a2, a0);
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



// Function: parse_inum @ 0x11d20
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char padding_10[40];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
} struct_0;

unsigned int parse_inum(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    struct_0 *idx;  // rax

    idx = sub_411b60(a1, a2, a0);
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



// Function: parse_size @ 0x11de0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_size(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned long v5;  // r14
    unsigned int v6;  // eax
    void* v7;  // rax
    char v8;  // bl
    unsigned long long v9;  // r13
    unsigned int v11;  // r15d
    struct_0 *idx;  // rax
    unsigned int v13;  // ymm0
    unsigned int v0;  // [bp-0x54]
    char v1;  // [bp-0x50]
    char v2;  // [bp-0x42]
    char v3;  // [bp-0x41]

    if (!a1)
        return 0;
    v5 = *((long long *)(a1 + *(a2) * 8));
    if (v5)
    {
        v6 = strlen(v5);
        if (!v6)
            error(1, 0, dcgettext(NULL, "invalid null argument to -size", 5));
        v7 = v5 + v6 - 1;
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
            else if (v8 > 71)
            {
                if (v8 == 77)
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
                if (v8 - 48 <= 9)
                {
                    v9 = 0x200;
                    v8 = 0;
                }
                else
                {
                    error(1, 0, dcgettext(NULL, "invalid -size type `%c'", 5));
                }
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
        v11 = sub_411ae0(v5, &v1, &v0);
        if ((char)v11)
        {
            idx = (unsigned long long)sub_40d490(a0, v5);
            *((unsigned int *)((char *)&idx[3].field_8 + 4)) = v9;
            *((unsigned int *)&idx[3].field_8) = v0;
            if (v0)
            {
                v13 = 1008981770;
                if (v0 == 1)
                {
                    v13 = 1063675494;
                    if (v1 * v9 <= 0x5000)
                        v13 = 1036831949;
                }
            }
            else
            {
                v13 = 1036831949;
                if (v1 * v9 <= 0x5000)
                    v13 = 1063675494;
            }
            *((unsigned long long *)&idx[4].padding_0[0]) = v1;
            *((unsigned short *)((char *)&idx[1].field_8 + 2)) = 1;
            *((unsigned int *)&idx[2].padding_0[4]) = v13;
            *(a2) = *(a2) + 1;
            return v11;
        }
        v2 = v8;
        v3 = 0;
        error(1, 0, dcgettext(NULL, "Invalid argument `%s%s' to -size", 5));
    }
    return 0;
}



// Function: collect_arg_stat_info @ 0x12080
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_43e670;

int collect_arg_stat_info(unsigned long long a0, int *a1, unsigned long a2, unsigned long long *a3)
{
    unsigned int v2;  // eax
    int *err;  // rax
    char v0;  // [bp-0x38]

    v2 = sub_4107d0(a0, a1, &v0);
    if (!(char)v2)
    {
        *(a3) = 0;
        return v2;
    }
    *(a3) = v0;
    if (g_43e670(v0, a2))
    {
        err = __errno_location();
        sub_40e310(*(err), *((unsigned int *)&v0)); /* do not return */
    }
    return v2;
}



// Function: parse_newer @ 0x12110
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_newer(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v4;  // eax
    struct_0 *idx;  // rbp
    unsigned long long v6;  // rax
    unsigned long v7;  // rdx
    unsigned int v8;  // ymm0
    unsigned long long v0;  // [bp-0xd0]
    char v1;  // [bp-0xc8]
    unsigned int v2;  // [bp-0x70]

    sub_40d860();
    v4 = sub_412080(a1, a2, &v1, &v0);
    if ((char)v4)
    {
        idx = (unsigned long long)sub_40d490(a0, v0);
        v6 = sub_425c60(&v1);
        idx[3].field_8 = 3;
        *((unsigned long long *)&idx[4].padding_0[0]) = v6;
        idx[4].field_8 = v7;
        sub_410760(v2);
        *((unsigned int *)&idx[2].padding_0[4]) = v8;
        return v4;
    }
    return v4;
}



// Function: parse_cnewer @ 0x121d0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_cnewer(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v4;  // eax
    struct_0 *v5;  // rax
    struct_0 *idx;  // rbp
    unsigned long long v7;  // rax
    unsigned long v8;  // rdx
    unsigned int v9;  // ymm0
    unsigned long long v0;  // [bp-0xd0]
    char v1;  // [bp-0xc8]
    unsigned int v2;  // [bp-0x70]

    sub_40d860();
    v4 = sub_412080(a1, a2, &v1, &v0);
    if ((char)v4)
    {
        v5 = (unsigned long long)sub_40d490(a0, v0);
        *((unsigned int *)&v5[3].field_8) = 2;
        idx = v5;
        v7 = sub_425c60(&v1);
        *((unsigned int *)((char *)&idx[3].field_8 + 4)) = 0;
        *((unsigned long long *)&idx[4].padding_0[0]) = v7;
        idx[4].field_8 = v8;
        sub_410760(v2);
        *((unsigned int *)&idx[2].padding_0[4]) = v9;
        return v4;
    }
    return v4;
}



// Function: parse_anewer @ 0x122a0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_anewer(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v4;  // eax
    struct_0 *v5;  // rax
    struct_0 *idx;  // rbp
    unsigned long long v7;  // rax
    unsigned long v8;  // rdx
    unsigned int v9;  // ymm0
    unsigned long long v0;  // [bp-0xd0]
    char v1;  // [bp-0xc8]
    unsigned int v2;  // [bp-0x70]

    sub_40d860();
    v4 = sub_412080(a1, a2, &v1, &v0);
    if ((char)v4)
    {
        v5 = (unsigned long long)sub_40d490(a0, v0);
        *((unsigned int *)&v5[3].field_8) = 0;
        idx = v5;
        v7 = sub_425c60(&v1);
        *((unsigned int *)((char *)&idx[3].field_8 + 4)) = 0;
        *((unsigned long long *)&idx[4].padding_0[0]) = v7;
        idx[4].field_8 = v8;
        sub_410760(v2);
        *((unsigned int *)&idx[2].padding_0[4]) = v9;
        return v4;
    }
    return v4;
}



// Function: parse_samefile @ 0x12370
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
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

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned int g_43e668;
extern struct_1 *g_43e670;
extern char g_43e678;

int parse_samefile(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v21;  // eax
    long long v22;  // r15
    int v23;  // ebp
    unsigned int v24;  // eax
    int v25;  // ebp
    struct_0 *idx;  // rax
    int *err;  // rax
    unsigned long long v0;  // [bp-0x170]
    char v1;  // [bp-0x168], Other Possible Types: unsigned long long
    unsigned long v2;  // [bp-0x160]
    uint128_t v3;  // [bp-0x158]
    unsigned int v4;  // [bp-0x150]
    int v5;  // [bp-0x148]
    int v6;  // [bp-0x138]
    int v7;  // [bp-0x128]
    uint128_t v8;  // [bp-0x118]
    uint128_t v9;  // [bp-0x108]
    uint128_t v10;  // [bp-0xf8]
    uint128_t v11;  // [bp-0xe8]
    stat v12;  // [bp-0xd8]
    int v13;  // [bp-0xd0]
    uint128_t v14;  // [bp-0xc8]
    int v15;  // [bp-0xb8]
    uint128_t v16;  // [bp-0x88]
    uint128_t v17;  // [bp-0x78]
    uint128_t v18;  // [bp-0x68]
    uint128_t v19;  // [bp-0x58]

    sub_40d860();
    v21 = sub_412080(a1, a2, &v1, &v0);
    if (!(char)v21)
        return v21;
    sub_40d860();
    v22 = v0;
    if (g_43e668)
    {
LABEL_412440:
        v24 = sub_415cc0(v22, 0);
        v23 = v24;
        if (v24 >= 0)
            goto LABEL_4124a5;
    }
    else if (!g_43e678)
    {
        if ((v4 & 0xf000) != 0xa000)
            goto LABEL_412440;
        v23 = -0x2;
    }
    else
    {
        v24 = sub_415cc0(v22, 0x20000);
        v23 = v24;
        if (v24 >= 0)
        {
LABEL_4124a5:
            if (!(!fstat(v24, &v12) && !g_43e670(v22, &v1)))
            {
                err = __errno_location();
                sub_40e310(*(err), v22); /* do not return */
            }
            else if (!(!g_43e668 && !g_43e678))
            {
                memcpy(&v1, &v12, 16);
                v3 = v14;
                v5 = (int)*((uint128_t *)&v15);
                v6 = (int)*((uint128_t *)(&v15 + 16));
                v7 = (int)*((uint128_t *)(&v15 + 32));
                v8 = v16;
                v9 = v17;
                v10 = v18;
                v11 = v19;
            }
            else if (((unsigned short)v4 & 0xf000) == 0xa000 || v1 != *((long long *)&v12.st_mode) || v2 != *((unsigned long long *)&v13))
            {
                v25 = -0x1;
                close(v23);
                v23 = v25;
            }
        }
    }
    idx = (unsigned long long)sub_40d490(a0, v22);
    *((int *)&idx[4].field_8) = v23;
    idx[3].field_8 = v2;
    *((unsigned int *)&idx[2].padding_0[4]) = 1008981770;
    *((unsigned long long *)&idx[4].padding_0[0]) = v1;
    *((unsigned short *)((char *)&idx[1].field_8 + 2)) = 1;
    return v21;
}



// Function: parse_iregex @ 0x12710
void parse_iregex(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: stream_is_tty @ 0x12740
typedef struct FILE {
} FILE;

unsigned int stream_is_tty(FILE *a0)
{
    unsigned int v1;  // edi
    unsigned long long v2;  // rax

    v1 = fileno(a0);
    if (v1 == 0xffffffff)
        return 0;
    v2 = isatty(v1);
    return _INSERT(v2, 0, (unsigned int)v2);
}



// Function: open_output_file @ 0x12770
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    struct FILE *field_8;
    char *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

extern unsigned long long stderr;
extern unsigned long long stdout;
extern long long *g_43e610;

unsigned int open_output_file(char *a0, struct_0 *idx)
{
    void* v1;  // rax
    unsigned int v2;  // eax
    int *err;  // rax
    char *v4;  // rax
    FILE *v5;  // rdi
    unsigned int v6;  // eax
    char *v7;  // rax
    FILE *v8;  // rdi
    unsigned int v9;  // eax

    idx->field_0 = 0;
    idx->field_20 = sub_424ac0(0);
    if (!strcmp(a0, "/dev/stderr"))
    {
        idx->field_8 = stderr;
        v7 = dcgettext(NULL, "standard error", 5);
        v8 = idx->field_8;
        idx->field_10 = v7;
        v9 = sub_412740(v8);
        idx->field_18 = v9;
        return v9;
    }
    else if (strcmp(a0, "/dev/stdout"))
    {
        v1 = sub_40e580(g_43e610, a0);
        idx->field_10 = a0;
        idx->field_8 = v1;
        if (!v1)
        {
            err = __errno_location();
            sub_40e410(*(err), *((unsigned int *)&a0)); /* do not return */
        }
        v2 = sub_412740(v1);
        idx->field_18 = v2;
        return v2;
    }
    else
    {
        idx->field_8 = stdout;
        v4 = dcgettext(NULL, "standard output", 5);
        v5 = idx->field_8;
        idx->field_10 = v4;
        v6 = sub_412740(v5);
        idx->field_18 = v6;
        return v6;
    }
}



// Function: open_stdout @ 0x12870
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    struct FILE *field_8;
    char *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

unsigned int open_stdout(struct_0 *a0)
{
    return sub_412770("/dev/stdout", a0);
}



// Function: parse_printf @ 0x12880
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    struct FILE *field_8;
    char *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

unsigned int parse_printf(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v3;  // eax
    unsigned int v4;  // eax
    unsigned long long v0;  // [bp-0x60]
    char v1;  // [bp-0x58]

    v3 = sub_410790(a1, a2, &v0);
    if (!(char)v3)
        return v3;
    sub_412870(&v1);
    v4 = sub_40fef0(&v1, a0, v0);
    if (!(char)v4)
    {
        *(a2) = *(a2);
        return v4;
    }
    return v4;
}



// Function: parse_print @ 0x12910
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_print(struct_0 *a0)
{
    struct_0 *v1;  // rax

    v1 = (unsigned long long)sub_40d4d0(a0);
    *((unsigned int *)&v1[1].field_8) = 0x101;
    sub_412870(&v1[3].field_8);
    return 1;
}



// Function: insert_fprint @ 0x12940
void insert_fprint(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_412945();
    return;
}



// Function: parse_print0 @ 0x12990
void parse_print0(void)
{
    unsigned int v1;  // rdi

    sub_412940(v1, 0);
    return;
}



// Function: parse_fprint0 @ 0x129a0
int parse_fprint0(unsigned int a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    char v0;  // [bp-0x28]

    v2 = sub_4107d0(a1, a2, &v0);
    if (!(char)v2)
        return v2;
    v3 = (unsigned long long)sub_412940(a0, v0);
    if (!(char)v3)
    {
        *(a2) = *(a2) - 1;
        return v3;
    }
    return v3;
}



// Function: insert_fls @ 0x12a10
void insert_fls(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_412a15();
    return;
}



// Function: parse_ls @ 0x12a60
void parse_ls(void)
{
    unsigned int v1;  // rdi

    sub_412a10(v1, 0);
    return;
}



// Function: parse_fls @ 0x12a70
int parse_fls(unsigned int a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    char v0;  // [bp-0x28]

    v2 = sub_4107d0(a1, a2, &v0);
    if (!(char)v2)
        return v2;
    v3 = (unsigned long long)sub_412a10(a0, v0);
    if (!(char)v3)
    {
        *(a2) = *(a2) - 1;
        return v3;
    }
    return v3;
}



// Function: parse_fprintf @ 0x12ae0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    struct FILE *field_8;
    char *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

unsigned int parse_fprintf(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v4;  // r13d
    unsigned int v5;  // eax
    char v0;  // [bp-0x68]
    unsigned long long v1;  // [bp-0x60]
    char v2;  // [bp-0x58]

    v4 = *(a2);
    if ((char)sub_4107d0(a1, a2, &v0) && (char)sub_410790(a1, a2, &v1))
    {
        sub_412770(v0, &v2);
        v4 = *(a2);
        v5 = sub_40fef0(&v2, a0, v1);
        if ((char)v5)
            return v5;
    }
    *(a2) = v4;
    return 0;
}



// Function: parse_fprint @ 0x12b90
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_fprint(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    unsigned int *idx;  // rbp
    char v0;  // [bp-0x28]

    v2 = sub_4107d0(a1, a2, &v0);
    if ((char)v2)
    {
        idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v0));
        sub_412770(v0, idx + 14);
        idx[6] = 0x101;
        idx[9] = 0x3f800000;
        return v2;
    }
    return v2;
}



// Function: parse_perm @ 0x12c20
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_43e68c;

int parse_perm(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v5;  // eax
    char *v6;  // r15
    unsigned int v7;  // r14d
    char *v8;  // rdi
    unsigned long v9;  // rbp
    unsigned long long v10;  // rax
    struct_0 *idx;  // rax
    unsigned int v0;  // [bp-0x54]
    unsigned int v1;  // [bp-0x50]
    unsigned int v2;  // [bp-0x4c]
    char v3;  // [bp-0x48]

    v5 = sub_4107d0(a1, a2, &v3);
    if (!(char)v5)
        return v5;
    v6 = (char *)v3;
    if (*(v6) == 45)
    {
        v8 = v6 + 1;
        v7 = 0;
        v0 = 1045220557;
    }
    else if (*(v6) != 47)
    {
        v7 = 2;
        v0 = 1008981770;
        v8 = v6;
    }
    else
    {
        v8 = v6 + 1;
        v7 = 1;
        v0 = 1050253722;
    }
    v9 = sub_41d600(v8);
    if (v9 && (*(v6) != 43 || v6[1] - 48 > 7))
    {
        v2 = sub_41d9d0(0, 0, 0, v9, 0);
        v10 = sub_41d9d0(0, 1, 0, v9, 0);
        v1 = v10;
        free(v9);
        if (*(v6) == 47 && !v1 && !v2)
        {
            v7 = 0;
            error(0, 0, dcgettext(NULL, "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.", 5));
            v0 = 0x3f7fa440;
        }
        idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v6));
        *((unsigned int *)&idx[3].field_8) = v7;
        *((unsigned long long *)((char *)&idx[3].field_8 + 4)) = v2 | v10 * 0x100000000;
        *((unsigned int *)&idx[2].padding_0[4]) = v0;
        return v5;
    }
    sub_424e50(0, g_43e68c, v6);
    error(1, 0, dcgettext(NULL, "invalid mode %s", 5));
}



// Function: check_path_safety @ 0x12e20
void check_path_safety(unsigned long a0)
{
    unsigned long v3;  // rax
    unsigned long v4;  // rbx
    char *v5;  // rdi
    char *ptr;  // rax
    unsigned long v0;  // [bp-0x40]
    unsigned long v1;  // [bp-0x38]

    v3 = getenv("PATH");
    if (!v3)
        return;
    v4 = v3;
    sub_4170d0(v3, ":", 1, &v0, &v1);
    while (1)
    {
        if (!v1 || !(v5 = (char *)(v0 + v4), v1 != 1 || *(v5) != 46))
            error(1, 0, dcgettext(NULL, "The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)", 5));
        if (*(v5) != 47)
            break;
        if (!(char)sub_4170d0(v4, ":", 0, &v0, &v1))
            return;
    }
    ptr = strndup(v5, v1);
    if (!ptr)
        ptr = v4 + v0;
    sub_40dfb0(0, ptr);
    error(1, 0, dcgettext(NULL, "The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH", 5));
}



// Function: parse_okdir @ 0x13550
void parse_okdir(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: parse_or @ 0x135d0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_or(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b970(a0);
    idx->field_10 = 8589934595;
    *((void* *)&idx->padding_0[0]) = sub_4088c0;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_and @ 0x13610
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_and(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b970(a0);
    idx->field_10 = 12884901891;
    *((void* *)&idx->padding_0[0]) = sub_408870;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_comma @ 0x13650
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
    char padding_1c[8];
    unsigned int field_24;
} struct_0;

unsigned int parse_comma(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b970(a0);
    idx->field_10 = 4294967299;
    *((void* *)&idx->padding_0[0]) = sub_408cd0;
    idx->field_1a = 0;
    idx->field_24 = 0x3f800000;
    return 1;
}



// Function: parse_closeparen @ 0x13690
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_closeparen(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b970(a0);
    idx->field_10 = 5;
    *((void* *)&idx->padding_0[0]) = sub_408c30;
    idx->field_1a = 0;
    return 1;
}



// Function: check_name_arg @ 0x136c0
unsigned int check_name_arg(void)
{
    char *v1;  // rdx
    unsigned int v2;  // rsi
    unsigned int v3;  // rdi

    if (!(char)sub_4106f0())
        return 1;
    if (!strchr(v1, 47))
        return 1;
    sub_40dfb0(2, v2);
    sub_40dfb0(1, "/");
    sub_40dfb0(0, v3);
    error(0, 0, dcgettext(NULL, "warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?", 5));
    return 1;
}



// Function: parse_name @ 0x13750
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

int parse_name(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v2;  // r12d
    unsigned int v3;  // eax
    struct_0 *idx;  // rbx
    unsigned int v5;  // ymm0
    char v0;  // [bp-0x38]

    v2 = sub_4107d0(a1, a2, &v0);
    if (!(char)v2)
        return v2;
    sub_411020();
    v3 = sub_4136c0();
    if ((char)v3)
    {
        idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v0));
        *((unsigned short *)((char *)&idx[1].field_8 + 2)) = 0;
        idx[3].field_8 = v0;
        sub_4110c0(v0, 0);
        *((unsigned int *)&idx[2].padding_0[4]) = v5;
        return v3;
    }
    *(a2) = *(a2);
    return v3;
}



// Function: parse_iname @ 0x13810
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

unsigned int parse_iname(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // r12d
    struct_0 *idx;  // r13
    unsigned int v4;  // ymm0
    unsigned int v0;  // [bp-0x28]

    sub_411020();
    if ((char)sub_4107d0(a1, a2, &v0))
    {
        v2 = sub_4136c0();
        if ((char)v2)
        {
            idx = (unsigned long long)sub_40d490(a0, v0);
            *((unsigned short *)((char *)&idx[1].field_8 + 2)) = 0;
            idx[3].field_8 = v0;
            sub_4110c0(v0, 0);
            *((unsigned int *)&idx[2].padding_0[4]) = v4;
            return v2;
        }
    }
    return 0;
}



// Function: parse_group @ 0x138d0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

extern unsigned int g_43e68c;

int parse_group(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    char *v3;  // r13
    group *v4;  // rbx
    unsigned int v5;  // ebx
    unsigned int v6;  // eax
    struct_0 *idx;  // rax
    unsigned int v8;  // ymm0
    unsigned long long v9;  // 4099
    char v0;  // [bp-0x38]

    v2 = sub_4107d0(a1, a2, &v0);
    if (!(char)v2)
        return v2;
    v3 = (char *)v0;
    v4 = getgrnam(v3);
    endgrent();
    if (v4)
    {
        v5 = v4->gr_gid;
    }
    else
    {
        v6 = strspn(v3, "0123456789");
        if (v6)
        {
            if (!v3[v6])
            {
                v5 = sub_416f40(v3);
            }
            else
            {
                sub_424e50(1);
                sub_424e50(0, g_43e68c, v3);
                error(1, 0, dcgettext(NULL, "%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s", 5));
            }
        }
        else
        {
            if (*(v3))
            {
                sub_424e50(0, g_43e68c, v3);
                error(1, 0, dcgettext(NULL, "%s is not the name of an existing group", 5));
            }
            error(1, 0, dcgettext(NULL, "argument to -group is empty, but should be a group name", 5));
        }
    }
    idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v3));
    v8 = 1065185444;
    v9 = (unsigned long long)idx[4].padding_0;
    *((unsigned int *)&idx[3].field_8) = v5;
    if (v9 > 99)
        v8 = 1045220557;
    *((unsigned int *)&idx[2].padding_0[4]) = v8;
    return v2;
}



// Function: estimate_fstype_success_rate @ 0x13a70
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

void estimate_fstype_success_rate(char *a0)
{
    stat v0;  // [bp-0xb8]

    if (stat("/", &v0))
    {
        return;
    }
    else if (!strcmp(a0, sub_4104c0(&v0)))
    {
        return;
    }
    else
    {
        return;
    }
}



// Function: parse_daystart @ 0x13b10
extern char g_43e658;

unsigned int parse_daystart(void)
{
    if (!g_43e658)
        return sub_413b28();
    return 1;
}



// Function: parse_time @ 0x13bc0
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long stderr;
extern char g_43e660;

int parse_time(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v7;  // r12d
    unsigned long v8;  // r14
    long long v9;  // r9
    unsigned int v10;  // eax
    struct_0 *idx;  // rbx
    unsigned int v12;  // eax
    unsigned int v13;  // edi
    char *t;  // rax
    char *t1;  // rax
    char *t2;  // rax
    unsigned int v0;  // [bp-0x7c]
    char v2;  // [bp-0x68], Other Possible Types: unsigned long
    int v3[2];  // [bp-0x60], Other Possible Types: unsigned long
    int v4;  // [bp-0x58]
    unsigned long v5;  // [bp-0x48]

    dcgettext(NULL, "arithmetic overflow while converting %s days to a number of seconds", 5);
    v0 = *(a2);
    v7 = sub_4107d0(a1, a2, &v2);
    if (!(char)v7)
        return v7;
    v8 = v2;
    v2 = v8;
    v10 = sub_411650(v8, &v4, v9);
    if (!(char)v10)
    {
        *(a2) = v0;
        return v10;
    }
    idx = (unsigned long long)sub_40d490(a0, v8);
    *((uint128_t *)&idx[56]) = (uint128_t)v4;
    *((unsigned long *)&idx[72]) = v5;
    sub_410760(*((unsigned long long *)(&v4 + 8)));
    *((unsigned int *)&idx[36]) = 0;
    if (!(g_43e660 & 1))
        return v10;
    __fprintf_chk(stderr, 0x1, "inserting %s\n", *((long long *)&idx[8]));
    v12 = *((unsigned int *)(&v4 + 4));
    if (v12)
    {
        v13 = stderr;
        if (v12 != 1 && v12 == 2)
        {
            __fprintf_chk(stderr, 0x1, "    type: %s    %s  ", "eq", ">=");
            v3 = (int (32 bits)[2])*((long long *)&idx[64]);
            t = ctime(v3);
            __fprintf_chk(stderr, 0x1, "%lu %s", *((long long *)&idx[64]), t);
            v3 = *((long long *)&idx[64]) + 86400;
            t1 = ctime(&v3);
            __fprintf_chk(stderr, 0x1, "                 <  %lu %s", v3, t1);
            return v10;
        }
    }
    else
    {
        v13 = stderr;
    }
    __fprintf_chk(v13, 0x1, "    type: %s    %s  ");
    v3 = (int (32 bits)[2])*((long long *)&idx[64]);
    t2 = ctime(v3);
    __fprintf_chk(stderr, 0x1, "%lu %s", *((long long *)&idx[64]), t2);
    return v10;
}



// Function: parse_negate @ 0x13e80
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_negate(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b990(a0, 0);
    idx->field_10 = 17179869186;
    *((void* *)&idx->padding_0[0]) = sub_4092f0;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_openparen @ 0x13ec0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[2];
    unsigned short field_1a;
} struct_0;

unsigned int parse_openparen(unsigned int *a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_40b990(a0, 0);
    idx->field_10 = 4;
    *((void* *)&idx->padding_0[0]) = sub_409590;
    idx->field_1a = 0;
    return 1;
}



// Function: parse_newerXY @ 0x13f00
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern long long g_43e638;
extern struct_1 *g_43e670;
extern unsigned int g_43e68c;

unsigned int parse_newerXY(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned long idx;  // rax
    char *index;  // r14
    unsigned int v19;  // r9d
    char v11;  // r15b
    unsigned long v12;  // rsi
    struct_0 *idx1;  // r12
    unsigned int v14;  // eax
    int *err;  // rax
    unsigned long long v17;  // rax
    unsigned int v18;  // ymm0
    unsigned long long v0;  // [bp-0xf8]
    char v1;  // [bp-0xea]
    char v2;  // [bp-0xe9]
    char *v3;  // [bp-0xe8]
    unsigned int v4;  // [bp-0xe0]
    unsigned int v5;  // [bp-0xdc]
    char v6;  // [bp-0xd8]
    char v7[6];  // [bp-0x46]

    if (!a1)
        return 0;
    idx = *(a2);
    index = *((long long *)(a1 + idx * 8));
    v0 = idx * 8;
    if (index && strlen(index) == 8)
    {
        strncpy(v7, "aBcmt", 5);
        if (strncmp("-newer", index, 6))
            __assert_fail(); /* do not return */
        v11 = index[6];
        v1 = index[7];
        if (!(v11 != 66 && v1 != 66))
        {
            error(0, 0, dcgettext(NULL, "This system does not provide a way to find the birth time of a file.", 5));
            return 0;
        }
        else if (v11 != 116)
        {
            v2 = v11 == 66 | v1 == 66;
            v4 = v11;
            v3 = &v7[0];
            if (!strchr(v7, v4))
                return v2;
            v5 = v1;
            if (!strchr(v3, v5))
                return v2;
            v12 = *((long long *)(a1 + v0 + 8));
            if (v12)
            {
                *(a2) = (unsigned int)idx + 1;
                idx1 = (unsigned long long)sub_40d490(a0, v12);
                switch (v11)
                {
                case 99:
                    v14 = 2;
                    break;
                case 109:
                    v14 = 3;
                    break;
                case 97:
                    v14 = 0;
                    break;
                default:
                    if (strchr(v3, v4))
                        __assert_fail(); /* do not return */
                    __assert_fail(); /* do not return */
                }
                *((unsigned int *)&idx1[3].field_8) = v14;
                if (v1 != 116)
                {
                    sub_40d860();
                    if (g_43e670(*((long long *)(a1 + *(a2) * 8)), &v6))
                    {
                        err = __errno_location();
                        sub_40e310(*(err), *((long long *)(a1 + *(a2) * 8))); /* do not return */
                    }
                    if (!(unsigned int)sub_411a40(&v6, v5, idx1 + 4))
                    {
                        sub_40dfb0(0, *((long long *)(a1 + *(a2) * 8)));
                        error(1, 0, dcgettext(NULL, "Cannot obtain birth time of file %s", 5));
                    }
                }
                else if (!(char)sub_422c40(idx1 + 4, *((long long *)(a1 + *(a2) * 8)), &g_43e638))
                {
                    sub_424e50(0, g_43e68c, *((long long *)(a1 + *(a2) * 8)));
                    error(1, 0, dcgettext(NULL, "I cannot figure out how to interpret %s as a date or time", 5));
                }
                *((unsigned int *)((char *)&idx1[3].field_8 + 4)) = 0;
                sub_410760(idx1[4].padding_0);
                v17 = (unsigned long long)idx1->padding_0;
                *((unsigned int *)&idx1[2].padding_0[4]) = v18;
                *(a2) = *(a2) + 1;
                switch (v17)
                {
                case 0:
                    __assert_fail(); /* do not return */
                case 4232048:
                    v19 = *((char *)&idx1[1].field_8 + 2);
                    if (!(char)v19)
                        __assert_fail(); /* do not return */
                    return v19;
                default:
                    __assert_fail(); /* do not return */
                }
            }
            sub_424e50(0, g_43e68c, index);
            error(1, 0, dcgettext(NULL, "The %s test needs an argument", 5));
        }
        else
        {
            return _INSERT(0, 0, v11 == 66 | v1 == 66);
        }
    }
    return 0;
}



// Function: parse_regextype @ 0x14370
extern unsigned int g_43e67c;

unsigned int parse_regextype(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    long long v0;  // [bp-0x28]
    char v1;  // [bp+0x0]

    if ((char)sub_4107d0(a1, a2, &v0))
    {
        g_43e67c = sub_416d30(v0);
        return sub_414350(&v1, a2);
    }
    return 0;
}



// Function: parse_mindepth @ 0x14510
void parse_mindepth(void)
{
}



// Function: parse_depth @ 0x14530
extern unsigned short g_43e620;

int parse_depth(unsigned long a0, long long a1, long long a2)
{
    g_43e620 = 0x100;
    return sub_414350(a1, a2);
}



// Function: parse_d @ 0x14550
int parse_d(unsigned long a0, long long a1, long long a2)
{
    char v0;  // [bp+0x0]

    if ((char)sub_4106f0())
        error(0, 0, dcgettext(NULL, "warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.", 5));
    return sub_414530(&v0, a1, a2);
}



// Function: parse_ignore_race @ 0x145b0
extern char g_43e62e;

void parse_ignore_race(unsigned long a0, long long a1, long long a2)
{
    g_43e62e = 1;
    sub_414350(a1, a2);
    return;
}



// Function: parse_xdev @ 0x145d0
extern char g_43e62d;

void parse_xdev(unsigned long a0, long long a1, long long a2)
{
    g_43e62d = 1;
    sub_414350(a1, a2);
    return;
}



// Function: parse_noleaf @ 0x145f0
extern char g_43e62c;

void parse_noleaf(unsigned long a0, long long a1, long long a2)
{
    g_43e62c = 1;
    sub_414350(a1, a2);
    return;
}



// Function: parse_noignore_race @ 0x14610
extern char g_43e62e;

void parse_noignore_race(unsigned long a0, long long a1, long long a2)
{
    g_43e62e = 0;
    sub_414350(a1, a2);
    return;
}



// Function: parse_nowarn @ 0x14630
extern char g_43e630;

void parse_nowarn(unsigned long a0, long long a1, long long a2)
{
    g_43e630 = 0;
    sub_414350(a1, a2);
    return;
}



// Function: parse_warn @ 0x14650
extern char g_43e630;

void parse_warn(unsigned long a0, long long a1, long long a2)
{
    g_43e630 = 1;
    sub_414350(a1, a2);
    return;
}



// Function: check_option_combinations @ 0x14670
extern char g_43e621;

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
        if (*(iter) != sub_408d40)
        {
            v3 = iter[33];
            v2 = (*(iter) == sub_4086f0 ? (unsigned int)v2 | 2 : v2 & 0xffffffff);
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
    if (g_43e621)
        return v2;
    v0 = v2;
    error(1, 0, dcgettext(v3, "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.", 5));
}



// Function: set_follow_state @ 0x146f0
extern char g_40da00;
extern char g_40daa0;
extern char g_40db20;
extern char g_43e62c;
extern char g_43e660;
extern unsigned int g_43e668;
extern unsigned long long g_43e670;
extern unsigned long long g_43e680;

int set_follow_state(unsigned int a0)
{
    unsigned int v1;  // eax

    if (a0 == 1)
    {
        g_43e62c = 1;
        g_43e670 = &g_40daa0;
        v1 = (unsigned int)sub_410a40;
        g_43e680 = sub_410a40;
    }
    else if (a0 == 2)
    {
        g_43e62c = 1;
        g_43e670 = &g_40da00;
        v1 = (unsigned int)sub_4109e0;
        g_43e680 = sub_4109e0;
    }
    else if (!a0)
    {
        g_43e670 = &g_40db20;
        v1 = (unsigned int)sub_410940;
        g_43e680 = sub_410940;
    }
    g_43e668 = a0;
    if (g_43e660 & 2)
    {
        v1 = (unsigned int)sub_40db70;
        g_43e670 = sub_40db70;
    }
    return v1;
}



// Function: parse_follow @ 0x147a0
int parse_follow(unsigned long a0, unsigned long a1, unsigned long a2)
{
    long long v1;  // r8

    sub_4146f0(1);
    return sub_414350(v1);
}



// Function: parse_begin_user_args @ 0x147d0
extern unsigned long long g_43ee70;

void parse_begin_user_args(void)
{
    g_43ee70 = 0;
    return;
}



// Function: parse_end_user_args @ 0x147e0
void parse_end_user_args(void)
{
    return;
}



// Function: find_parser @ 0x147f0
extern char *g_43cd48;

unsigned long long find_parser(char *a0)
{
    char **v1;  // rbx
    char *v2;  // rdi
    char **v3;  // rbx

    if (strncmp("-newer", a0, 6) || strlen(a0) != 8)
    {
        v1 = &g_43cd48;
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



// Function: insert_false @ 0x148a0
unsigned int insert_false(void)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_40d4d0(sub_4147f0("false"));
    idx[6] = 0;
    idx[9] = 0;
    return 1;
}



// Function: parse_false @ 0x148d0
unsigned int parse_false(void)
{
    return sub_4148a0();
}



// Function: parse_fstype @ 0x148e0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned long long stderr;
extern char g_43e660;
extern unsigned short g_43e688;

unsigned int parse_fstype(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v2;  // eax
    void* idx;  // rax
    unsigned int v4;  // ymm0
    char v0;  // [bp-0x28]

    v2 = sub_4107d0(a1, a2, &v0);
    if (!(char)v2)
        return v2;
    if (g_43e688 > 1 && !(char)(unsigned long long)sub_410510(v0))
    {
        if (!(g_43e660 & 8))
            return sub_4148a0();
        __fprintf_chk(stderr, 0x1, "-fstype %s can never succeed, substituting -false\n", v0);
        return sub_4148a0();
    }
    idx = (unsigned long long)sub_40d490(a0, *((unsigned int *)&v0));
    *((unsigned long long *)&idx[56]) = v0;
    sub_413a70(v0);
    *((unsigned int *)&idx[36]) = v4;
    return v2;
}



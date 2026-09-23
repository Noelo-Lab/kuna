// Function: left_dir @ 0x73b9
extern unsigned int g_449010;
extern void g_449018;

unsigned long long left_dir(void)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax
    unsigned long long v3;  // rax

    v1 = g_449010 & 0x200;
    if (!(unsigned int)v1)
        return v1;
    v2 = *((int *)&g_449018);
    if ((unsigned int)v2 < 0)
        return v2;
    v3 = close(*((int *)&g_449018));
    *((unsigned int *)&g_449018) = 0xffffffff;
    return v3;
}



// Function: inside_dir @ 0x73f4
extern unsigned int g_449010;
extern void g_449018;
extern unsigned int g_4499d8;

unsigned long long inside_dir(int a0)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    v1 = g_449010 & 0x200;
    if (!(unsigned int)v1)
        return v1;
    if (a0 != -0x64 && a0 < 0)
        __assert_fail(); /* do not return */
    g_4499d8 = a0;
    v2 = *((int *)&g_449018);
    if ((unsigned int)v2 >= 0)
    {
        return v2;
    }
    else if (a0 == -0x64)
    {
        *((unsigned int *)&g_449018) = 0xffffff9c;
        return v2;
    }
    else if (a0 >= 0)
    {
        *((unsigned int *)&g_449018) = sub_41d639(a0);
        return *((int *)&g_449018);
    }
    else if (*((int *)&g_449018) >= 0)
    {
        return *((int *)&g_449018);
    }
    else if (a0 < 0)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        return *((int *)&g_449018);
    }
}



// Function: get_fts_info_name @ 0x74c2
extern char g_43805e;
extern char g_438064;
extern char g_43806b;
extern char g_438077;
extern char g_43807f;
extern char g_438087;
extern char g_43808e;
extern char g_438096;
extern char g_43809c;
extern char g_4380a5;
extern char g_4380ac;
extern char g_4380b5;
extern char g_4380bc;
extern char g_4380c7;
extern char g_449918;

char * get_fts_info_name(unsigned int a0)
{
    switch (a0)
    {
    case 1:
        return &g_43805e;
    case 2:
        return &g_438064;
    case 3:
        return &g_43806b;
    case 4:
        return &g_438077;
    case 5:
        return &g_43807f;
    case 6:
        return &g_438087;
    case 7:
        return &g_43808e;
    case 8:
        return &g_438096;
    case 9:
        return &g_43809c;
    case 10:
        return &g_4380a5;
    case 11:
        return &g_4380ac;
    case 12:
        return &g_4380b5;
    case 13:
        return &g_4380bc;
    case 14:
        return &g_4380c7;
    default:
        sprintf(&g_449918, "[%d]", a0);
        return &g_449918;
    }
}



// Function: visit @ 0x75b5
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

extern char g_4499c4;
extern unsigned long long g_4499d0;
extern unsigned int g_4499d8;
extern char g_4499e0;

long long visit(struct_1 *a0, struct_0 *a1, long long a2)
{
    char v2;  // al
    long long v0;  // [bp-0x10]

    if (a1->field_68 != 10 && a1->field_68 != 11)
        v2 = 1;
    else
        v2 = 0;
    g_4499c4 = v2 & 1;
    g_4499d0 = a1->field_30;
    g_4499d8 = a0->field_2c;
    v0 = sub_40d7ff();
    sub_410873(a1->field_38, a2, v0, a2);
    if (!g_4499e0)
        return g_4499e0;
    return sub_41fc0f(a0, a1, 4, a1);
}



// Function: partial_quotearg_n @ 0x7664
extern long long g_438110;

int partial_quotearg_n(unsigned int a0, long long a1, unsigned long long a2, unsigned int a3)
{
    char v0;  // [bp-0x11]
    unsigned long long v1;  // [bp-0x10]

    if (a2)
    {
        v0 = *((char *)(a2 + a1));
        *((char *)(a2 + a1)) = 0;
        v1 = sub_42f861(a0, a3, a1, a3);
        *((char *)(a1 + a2)) = v0;
        return v1;
    }
    return sub_42f861(a0, a3, &g_438110, a3);
}



// Function: issue_loop_warning @ 0x76ea
typedef struct struct_1 {
    struct struct_0 *field_0;
    char padding_8[48];
    long long field_38;
    char padding_40[72];
    unsigned int field_88;
} struct_1;

typedef struct struct_0 {
    char padding_0[56];
    long long field_38;
    char padding_40[8];
    unsigned long long field_48;
} struct_0;

extern unsigned int g_4499ac;

void issue_loop_warning(struct_1 *a0)
{
    if (((unsigned short)a0->field_88 & 0xf000) != 0xa000)
    {
        sub_407664(1, a0->field_0->field_38, a0->field_0->field_48, g_4499ac);
        sub_41096a(0, a0->field_38);
        error(0, 0, gettext("File system loop detected; %s is part of the same file system loop as %s."));
        return;
    }
    sub_41096a(0, a0->field_38);
    error(0, 0, gettext("Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points."));
    return;
}



// Function: symlink_loop @ 0x77d0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_449990;

long long symlink_loop(unsigned long a0)
{
    unsigned int v3;  // eax
    unsigned int v0;  // [bp-0xac]
    char v1;  // [bp-0xa8]

    v0 = g_449990(a0, &v1, &v1, g_449990);
    if (v0 && *(__errno_location()) == 40)
        v3 = 1;
    else
        v3 = 0;
    return v3 & 1;
}



// Function: consider_visiting @ 0x7852
typedef struct struct_0 {
    char padding_0[44];
    int field_2c;
} struct_0;

typedef struct FILE {
} FILE;

extern void g_449014;
extern FILE *stderr;
extern char g_449940;
extern int g_449944;
extern int g_449948;
extern unsigned long long g_449980;
extern unsigned int g_4499ac;
extern unsigned int g_4499c0;
extern char g_4499c4;
extern char g_4499c5;
extern unsigned int g_4499c8;
extern char g_4499e0;
extern unsigned int g_4499e4;

void consider_visiting(struct_0 *a0, void* idx)
{
    unsigned long long v23;  // rax
    char *v24;  // rbx
    unsigned int v25;  // eax
    char *v26;  // rax
    int v28;  // eax
    unsigned int v29;  // eax
    char *v30;  // rax
    unsigned long v0;  // [bp-0x100], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xe4]
    int v2;  // [bp-0xe0]
    int v3;  // [bp-0xdc]
    unsigned long v4;  // [bp-0xd8]
    unsigned long v5;  // [bp-0xd0]
    unsigned long v6;  // [bp-0xc8]
    unsigned long v7;  // [bp-0xc0]
    unsigned long v8;  // [bp-0xb8]
    unsigned long v9;  // [bp-0xb0]
    unsigned long v10;  // [bp-0xa8]
    unsigned long v11;  // [bp-0xa0]
    unsigned long v12;  // [bp-0x98]
    unsigned long v13;  // [bp-0x90]
    unsigned long v14;  // [bp-0x88]
    unsigned long v15;  // [bp-0x80]
    unsigned long v16;  // [bp-0x78]
    unsigned long v17;  // [bp-0x70]
    unsigned long v18;  // [bp-0x68]
    unsigned long v19;  // [bp-0x60]
    unsigned long v20;  // [bp-0x58]
    unsigned long v21;  // [bp-0x50]

    if ((char)g_449980 & 4)
    {
        v23 = sub_42f861(2, g_4499ac, (long long)idx[48]);
        v24 = sub_42f861(1, g_4499ac, (long long)idx[56]);
        v25 = (unsigned int)sub_4074c2((short)idx[104]);
        v26 = sub_42f861(0, g_4499ac, (long long)idx[56]);
        v0 = v23;
        fprintf(stderr, "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n", v26, v25, (int)(long long)idx[88], *((int *)&g_449014), v24);
    }
    if ((short)idx[104] == 6)
    {
        sub_4073b9();
    }
    else if ((long long)idx[88] > *((int *)&g_449014) || !(long long)idx[88])
    {
        sub_4073b9();
    }
    sub_4073f4(a0->field_2c);
    *((int *)&g_449014) = (long long)idx[88];
    v5 = (long long)idx[120];
    if ((short)idx[104] == 7)
    {
        sub_410a13((int)idx[64], (long long)idx[56], (long long)idx[56]);
        return;
    }
    if ((short)idx[104] == 4)
    {
        sub_410a13((int)idx[64], (long long)idx[56], (long long)idx[56]);
        if (g_449940)
            return;
    }
    else if ((short)idx[104] == 2)
    {
        sub_4076ea(idx);
        g_4499e4 = 1;
        return;
    }
    else if ((short)idx[104] == 13)
    {
        if ((char)sub_4077d0((long long)idx[48]))
        {
            sub_410a13(40, (long long)idx[56]);
            return;
        }
    }
    else
    {
        if ((short)idx[104] == 10)
        {
            if (!(long long)idx[88])
            {
                sub_410a13((int)idx[64], (long long)idx[56], (long long)idx[56]);
                return;
            }
            if ((char)sub_4077d0((long long)idx[48]))
            {
                sub_410a13(40, (long long)idx[56]);
                return;
            }
            sub_410a13((int)idx[64], (long long)idx[56], (long long)idx[56]);
        }
    }
    if ((short)idx[104] == 11 || (short)idx[104] == 10)
    {
        if (g_4499c4 == 1)
            __assert_fail(); /* do not return */
        if ((short)idx[104] != 11 && g_4499c8)
            __assert_fail(); /* do not return */
        v1 = g_4499c8;
    }
    else
    {
        g_4499c4 = 1;
        g_4499c5 = 1;
        v4 = (long long)idx[112];
        v5 = (long long)idx[120];
        v6 = (long long)idx[128];
        v7 = (long long)idx[0x88];
        v8 = (long long)idx[144];
        v9 = (long long)idx[152];
        v10 = (long long)idx[160];
        v11 = (long long)idx[168];
        v12 = (long long)idx[176];
        v13 = (long long)idx[184];
        v14 = (long long)idx[192];
        v15 = (long long)idx[200];
        v16 = (long long)idx[208];
        v17 = (long long)idx[216];
        v18 = (long long)idx[224];
        v19 = (long long)idx[232];
        v20 = (long long)idx[240];
        v21 = (long long)idx[248];
        v1 = v7;
        g_4499c8 = v1;
        if (!v1)
        {
            sub_42f861(0, g_4499ac, (long long)idx[56]);
            error(0, 0, gettext("WARNING: file %s appears to have mode 0000"));
        }
    }
    g_4499c0 = (long long)idx[88];
    if (v1 && (char)sub_40fe38(&v1, (long long)idx[56], idx + 0x100, &v4, 0) != 1)
        return;
    v2 = 0;
    if (((unsigned short)v1 & 0xf000) == 0x4000 || (short)idx[104] == 1 || (short)idx[104] == 6 || (short)idx[104] == 2)
        v28 = 1;
    else
        v28 = 0;
    v3 = v28;
    if (v3 && (short)idx[104] == 11)
    {
        sub_41fc0f(a0, idx, 1, idx);
        return;
    }
    if (g_449944 >= 0 && (long long)idx[88] >= g_449944)
    {
        sub_41fc0f(a0, idx, 4, idx);
        if ((long long)idx[88] > g_449944)
            v2 = 1;
    }
    if ((short)idx[104] == 1 && g_449940 != 1)
    {
        v2 = 1;
    }
    else if ((short)idx[104] == 6 && g_449940)
    {
        v2 = 1;
    }
    else if ((long long)idx[88] < g_449948)
    {
        v2 = 1;
    }
    if ((char)g_449980 & 4)
    {
        v29 = (unsigned int)sub_4074c2((short)idx[104]);
        v30 = sub_42f861(0, g_4499ac, (long long)idx[56]);
        v0 = g_4499c5;
        fprintf(stderr, "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n", v30, v29, v3, v2, (unsigned int)g_4499c4);
    }
    if (!v2)
        sub_4075b5(a0, idx, &v4);
    if ((short)idx[104] != 6)
        return;
    g_4499e0 = 0;
    return;
}



// Function: find @ 0x803a
typedef struct struct_0 {
    char padding_0[88];
    unsigned long long field_58;
} struct_0;

extern unsigned int g_449010;
extern char g_44994d;
extern unsigned int g_449988;
extern char g_4499c4;
extern char g_4499c5;
extern unsigned int g_4499c8;
extern unsigned int g_4499dc;
extern unsigned int g_4499e4;
extern char g_4499e8;
extern char g_4499f8;

int find(void)
{
    unsigned long v5;  // rdi
    char *v6;  // rax
    int *err;  // rax
    int *err1;  // rax
    char *v9;  // rax
    int *err2;  // rax
    unsigned int v0;  // [bp-0x4c]
    unsigned long v1;  // [bp-0x48], Other Possible Types: unsigned long long
    struct_0 *v2;  // [bp-0x40]
    unsigned long long v3;  // [bp-0x30]

    g_4499dc = strlen(v5);
    sub_4073f4(-0x64);
    v3 = 0;
    if (g_449988 == 2)
    {
        g_449010 = g_449010 | 0x11;
    }
    else if (g_449988 <= 2)
    {
        if (!g_449988)
        {
            g_449010 = g_449010 | 16;
        }
        else if (g_449988 == 1)
        {
            g_449010 = g_449010 | 3;
        }
    }
    if (g_44994d)
        g_449010 = g_449010 | 64;
    v1 = sub_41ec55(&v5, g_449010, 0, g_449010);
    if (!v1)
    {
        sub_41096a(0, v5);
        v6 = gettext("cannot search %s");
        err = __errno_location();
        error(0, *(err), v6);
        g_4499e4 = 1;
    }
    else
    {
        v0 = 0x80000000;
        while (1)
        {
            *(__errno_location()) = 0;
            v2 = sub_41f599(v1);
            if (!v2)
                break;
            if (g_4499e8 && v0 != (unsigned int)v2->field_58)
                sub_40f6cb();
            v0 = v2->field_58;
            g_4499f8 = 0;
            g_4499c4 = 0;
            g_4499c5 = *((int *)&v2[1].padding_0[40]);
            g_4499c8 = (!g_4499c5 ? 0 : *((int *)&v2[1].padding_0[40]));
            sub_407852(v1, v2);
        }
        if (*(__errno_location()))
        {
            sub_41096a(0, v5);
            err1 = __errno_location();
            error(0, *(err1), "failed to read file names from file system at or below %s");
            g_4499e4 = 1;
            return;
        }
        if (sub_41f0df(v1))
        {
            v9 = gettext("failed to restore working directory after searching %s");
            err2 = __errno_location();
            error(0, *(err2), v9);
            g_4499e4 = 1;
            return;
        }
        v1 = 0;
    }
    return;
}



// Function: process_all_startpoints @ 0x82e7
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

extern void stdin;
extern char g_4499b0;
extern char g_4499b8;
extern unsigned int g_4499dc;
extern unsigned int g_4499e4;

long long process_all_startpoints(int a0, long long *a1)
{
    char v13;  // al
    char *v15;  // rax
    int *err;  // rax
    char *v19;  // rax
    int *err1;  // rax
    char v0;  // [bp-0x17a]
    char v1;  // [bp-0x179]
    unsigned int v2;  // [bp-0x178]
    unsigned int v3;  // [bp-0x174]
    unsigned long fp;  // [bp-0x170]
    unsigned long long v5;  // [bp-0x168]
    long long v6;  // [bp-0x160]
    char *v7;  // [bp-0x158]
    unsigned long long v8;  // [bp-0x150]
    stat v9;  // [bp-0x148]
    stat v10;  // [bp-0xb8]
    unsigned short v11;  // [bp-0x22]

    if (a0 > 0 && (char)sub_40ff96(*(a1), 1) != 1)
        v13 = 1;
    else
        v13 = 0;
    v1 = v13;
    v1 &= 1;
    fp = 0;
    v5 = 0;
    if (!*((long long *)&g_4499b0))
    {
        if (v1 != 1)
        {
            v11 = 46;
            return sub_40803a(&v11);
        }
        v6 = sub_41d0aa(a1);
    }
    else if (v1)
    {
        sub_41096a(0, *(a1));
        error(0, 0, gettext("extra operand %s"));
        gettext("file operands cannot be combined with -files0-from");
        error(1, 0, "%s");
    }
    else
    {
        if (strcmp(*((long long *)&g_4499b0), "-"))
        {
            v5 = sub_41096a(0, *((long long *)&g_4499b0));
            fp = fopen(*((long long *)&g_4499b0), "r");
            if (!fp)
            {
                v15 = gettext("cannot open %s for reading");
                err = __errno_location();
                error(1, *(err), v15);
            }
            v3 = fileno(fp);
            if (v3 < 0)
                __assert_fail(); /* do not return */
            if (g_4499b8 && !fstat(v3, &v9) && !fstat(0, &v10) && v9.st_ino == v10.st_ino && *((unsigned long long *)&v9) == *((unsigned long long *)&v10))
            {
                gettext("option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir");
                error(1, 0, "%s: %s\n");
            }
            sub_41d5ba(v3, 1);
        }
        else if (g_4499b8)
        {
            gettext("option -files0-from reading from standard input cannot be combined with -ok, -okdir");
            error(1, 0, "%s\n");
        }
        else
        {
            v5 = sub_41096a(0, gettext("(standard input)"));
            fp = *((long long *)&stdin);
        }
        v6 = sub_41d0ff(fp);
    }
    if (!v6)
        sub_4321fa(); /* do not return */
    v0 = 1;
    do
    {
        v7 = sub_41d16c(v6, &v2, &v2);
        if (!v7)
        {
            if (v2 != 4)
            {
                if (v2 <= 4)
                {
                    if (v2 == 2)
                        goto LABEL_408852;
                    if (v2 == 3)
                        sub_4321fa(); /* do not return */
                }
                __assert_fail(); /* do not return */
            }
            v19 = gettext("%s: read error");
            err1 = __errno_location();
            error(0, *(err1), v19);
            g_4499e4 = 1;
            v0 = 0;
            goto LABEL_408852;
        }
        else
        {
            if (!*(v7))
            {
                if (!*((long long *)&g_4499b0))
                {
                    sub_41096a(0, v7);
                    error(0, 2, "%s");
                }
                else
                {
                    v8 = sub_41d257(v6);
                    gettext("invalid zero-length file name");
                    error(0, 0, "%s:%lu: %s");
                }
                g_4499e4 = 1;
                v0 = 0;
                continue;
            }
            if (!*((long long *)&g_4499b0) && (char)sub_40ff96(v7, 1))
                goto LABEL_408852;
        }
    } while ((g_4499dc = (unsigned int)strlen(v7), (char)(long long)sub_40803a(v7) == 1));
    v0 = 0;
LABEL_408852:
    sub_41d295(v6);
    if (!v0 || !*((long long *)&g_4499b0) || !ferror(fp) && !sub_41dad5(fp))
        return v0;
    error(1, 0, gettext("error reading %s"));
    return v0;
}



// Function: main @ 0x88ed
typedef struct FILE {
} FILE;

extern char g_438110;
extern FILE *stderr;
extern long long g_449940;
extern int g_449968;
extern unsigned long long g_449980;
extern unsigned long long g_449990;
extern unsigned int g_4499d8;
extern unsigned int g_4499e4;
extern char g_4499e8;
extern unsigned long long g_4499f0;
extern char g_4499f8;

int main(unsigned int a0, long long *a1)
{
    long long v3;  // rdi
    long long v4;  // rsi
    char *t;  // rax
    long long v5;  // rdx
    long long v6;  // rcx
    long long v7;  // r8
    long long v8;  // r9
    char *v9;  // rax
    int *err;  // rax
    char *v11;  // rax
    int *err1;  // rax
    unsigned int v0;  // [bp-0x24]
    long long v1;  // [bp-0x20]

    v0 = 0;
    if (*(a1))
        sub_42dd35(*(a1));
    else
        sub_42dd35("find");
    sub_40f78b(v3, v4, v5, v6, v7, v8);
    g_4499f8 = 0;
    g_4499e4 = 0;
    g_4499e8 = 0;
    g_4499d8 = 0xffffff9c;
    if ((char)sub_41b5f4(v3, v4, v5, v6, v7, v8))
        sub_41b20a();
    g_4499f0 = sub_410be9("w");
    if (!g_4499f0)
    {
        v9 = gettext("Failed to initialize shared-file hash table");
        err = __errno_location();
        error(1, *(err), v9);
    }
    sub_4106bd(&g_449940);
    setlocale(6, &g_438110);
    bindtextdomain("findutils", "/usr/local/share/locale");
    textdomain("findutils");
    if (sub_437150(sub_41d697))
    {
        v11 = gettext("The atexit library function failed");
        err1 = __errno_location();
        error(1, *(err1), v11);
    }
    v0 = sub_4103e8(a0, a1, a1);
    if ((char)g_449980 & 2)
        g_449990 = sub_40fd39;
    if ((char)g_449980 & 128)
    {
        t = ctime(&g_449968);
        fprintf(stderr, "cur_day_start = %s", t);
    }
    v1 = sub_40dd20(a0, a1, v0, a1);
    if (!(char)sub_4082e7(a0 - v0, &a1[v0]))
        return g_4499e4;
    sub_40ae53(v1);
    sub_40fa38(v3, v4, v5, v6, v7, v8);
    return g_4499e4;
}



// Function: is_fts_enabled @ 0x8b12
extern unsigned int g_449010;

unsigned int is_fts_enabled(unsigned int *a0)
{
    *(a0) = g_449010;
    return 1;
}



// Function: ts_difference @ 0x8b31
long long ts_difference(int a0, unsigned long a1, int a2, unsigned long a3)
{
    unsigned long v0;  // [bp-0x10]

    v0 = AddV((int128_t)difftime(a0, a2), MulV(4472406533629990549, a1 - a3));
    return v0;
}



// Function: compare_ts @ 0x8ba9
unsigned int compare_ts(unsigned long a0, unsigned int a1, unsigned long a2, unsigned int a3)
{
    unsigned long v1;  // xmm0lq

    if (a0 == a2 && a1 == a3)
        return 0;
    sub_408b31(a0, a1, a2, a3);
    return (((CmpF(0, v1) & 69 | (CmpF(0, v1) & 69) >> 6) & 1) == 1 ? 1 : 0xffffffff);
}



// Function: pred_timewindow @ 0x8c32
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_timewindow(unsigned int a0, unsigned int a1, struct_0 *a2, int a3)
{
    unsigned int v1;  // eax
    unsigned long long v3;  // rax
    unsigned long v4;  // xmm0lq
    unsigned int v5;  // eax

    v1 = a2->field_3c;
    if (v1 != 2)
    {
        if (v1 <= 2)
        {
            if (!v1)
            {
                v3 = sub_408ba9(a0, a1, *((long long *)&a2->field_40), *((long long *)&a2->field_48));
                return _INSERT(v3, 0, (unsigned int)v3 > 0);
            }
            else if (v1 == 1)
            {
                return (sub_408ba9(a0, a1, *((long long *)&a2->field_40), *((long long *)&a2->field_48)) & 0xffffffff) >> 31;
            }
        }
        __assert_fail(); /* do not return */
    }
    else
    {
        sub_408b31(a0, a1, *((long long *)&a2->field_40), *((long long *)&a2->field_48));
        if (((CmpF(v4, 0) & 69 | (char)((CmpF(v4, 0) & 69) >> 6)) & 1) != 1 && !(CmpF(a3, v4) & 1))
            v5 = 1;
        else
            v5 = 0;
        return v5 & 1;
    }
}



// Function: pred_amin @ 0x8d51
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_amin(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned int v2;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    return sub_408c32(sub_430a4c(a1), v2, a2, 60);
}



// Function: pred_and @ 0x8d9c
unsigned long long pred_and(long long a0, long long a1, long long *a2)
{
    if (a2[0x22] && !(char)sub_410873(a0, a1, a2[0x22], a1))
        return 0;
    return sub_410873(a0, a1, a2[35], a1);
}



// Function: pred_anewer @ 0x8e0d
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;

char pred_anewer(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned long long v2;  // rax
    unsigned int v3;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    if (!a2->field_3c)
    {
        v2 = sub_430a4c(a1);
        return sub_408ba9(v2, v3, *((long long *)&a2->field_40), a2->field_48) > 0;
    }
    __assert_fail(); /* do not return */
}



// Function: pred_atime @ 0x8e96
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_atime(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned int v2;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    return sub_408c32(sub_430a4c(a1), v2, a2, 86400);
}



// Function: pred_closeparen @ 0x8ee1
unsigned int pred_closeparen(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return 1;
}



// Function: pred_cmin @ 0x8efc
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_cmin(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned int v2;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    return sub_408c32(sub_430a66(a1), v2, a2, 60);
}



// Function: pred_cnewer @ 0x8f47
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;

char pred_cnewer(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned long long v2;  // rax
    unsigned int v3;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    if (!a2->field_3c)
    {
        v2 = sub_430a66(a1);
        return sub_408ba9(v2, v3, *((long long *)&a2->field_40), a2->field_48) > 0;
    }
    __assert_fail(); /* do not return */
}



// Function: pred_comma @ 0x8fd0
long long pred_comma(long long a0, long long a1, long long *a2)
{
    if (a2[0x22])
        sub_410873(a0, a1, a2[0x22], a1);
    return sub_410873(a0, a1, a2[35], a1);
}



// Function: pred_ctime @ 0x9036
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_ctime(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned int v2;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    return sub_408c32(sub_430a66(a1), v2, a2, 86400);
}



// Function: perform_delete @ 0x9081
extern long long g_4499d0;
extern unsigned int g_4499d8;

char perform_delete(unsigned int a0)
{
    return !unlinkat(g_4499d8, g_4499d0, a0, g_4499d0);
}



// Function: pred_delete @ 0x90b1
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_44994e;
extern char g_4499c4;
extern char *g_4499d0;
extern unsigned int g_4499e4;

unsigned int pred_delete(long long a0, struct_0 *a1, unsigned long long a2)
{
    char *v3;  // rax
    int *err;  // rax
    unsigned long long v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x1c]

    v0 = a2;
    if (!strcmp(g_4499d0, "."))
        return 1;
    v1 = 0;
    if (g_4499c4 && ((unsigned short)a1->field_18 & 0xf000) == 0x4000)
        v1 |= 0x200;
    if (sub_409081(v1))
        return 1;
    if (*(__errno_location()) == 2 && g_44994e)
    {
        *(__errno_location()) = 0;
        return 1;
    }
    if (*(__errno_location()) == 21 && !((unsigned short)v1 & 0x200))
    {
        v1 |= 0x200;
        if (sub_409081(v1))
            return 1;
    }
    sub_41096a(0, a0);
    v3 = gettext("cannot delete %s");
    err = __errno_location();
    error(0, *(err), v3);
    g_4499e4 = 1;
    return 0;
}



// Function: pred_empty @ 0x91f4
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

extern long long g_4499d0;
extern unsigned int g_4499d8;
extern unsigned int g_4499e4;

unsigned long long pred_empty(long long a0, struct_0 *a1, unsigned long long a2)
{
    int *err;  // rax
    int *err1;  // rax
    int *err2;  // rax
    int *v9;  // rax
    unsigned long long v0;  // [bp-0x50]
    char v1;  // [bp-0x2d]
    unsigned int v2;  // [bp-0x2c]
    dirent *count;  // [bp-0x28]
    DIR *fp;  // [bp-0x20]

    v0 = a2;
    if (((unsigned short)a1->field_18 & 0xf000) != 0x4000)
        return ((a1->field_18 & 0xf000) == 0x8000 ? _INSERT(a1->field_30, 0, !a1->field_30) : 0);
    v1 = 1;
    *(__errno_location()) = 0;
    v2 = sub_42704e(g_4499d8, g_4499d0, 0x90900, g_4499d0);
    if (v2 < 0)
    {
        sub_41096a(0, a0);
        err = __errno_location();
        error(0, *(err), "%s");
        g_4499e4 = 1;
        return 0;
    }
    fp = fdopendir(v2);
    if (!fp)
    {
        sub_41096a(0, a0);
        err1 = __errno_location();
        error(0, *(err1), "%s");
        g_4499e4 = 1;
        close(v2);
        return 0;
    }
    *(__errno_location()) = 0;
    for (count = readdir(fp); count; count = readdir(fp))
    {
        if (*((char *)&count[1].d_ino + 6) != 46 || *((char *)&count[1].d_ino + 7) && (*((char *)&count[1].d_ino + 7) != 46 || (char)count[1].d_reclen))
        {
            v1 = 0;
            break;
        }
    }
    if (*(__errno_location()))
    {
        sub_41096a(0, a0);
        err2 = __errno_location();
        error(0, *(err2), "%s");
        g_4499e4 = 1;
        closedir(fp);
        return 0;
    }
    else if (closedir(fp))
    {
        sub_41096a(0, a0);
        v9 = __errno_location();
        error(0, *(v9), "%s");
        g_4499e4 = 1;
        return 0;
    }
    else
    {
        return v1;
    }
}



// Function: pred_exec @ 0x9466
long long pred_exec(long long a0, long long a1, long long a2)
{
    return sub_40b300(a0, a1, a2, a1);
}



// Function: pred_execdir @ 0x9497
extern long long g_4499d0;

long long pred_execdir(unsigned long long a0, long long a1, long long a2)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    return sub_40b300(g_4499d0, a1, a2, a1);
}



// Function: pred_false @ 0x94cb
unsigned int pred_false(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return 0;
}



// Function: pred_fls @ 0x94e6
typedef struct struct_0 {
    char padding_0[40];
    char field_28;
    char padding_29[23];
    unsigned long long field_40;
} struct_0;

extern long long g_449958;
extern unsigned int g_44997c;
extern long long g_4499d0;
extern unsigned int g_4499d8;

unsigned int pred_fls(long long a0, long long a1, struct_0 *a2)
{
    unsigned long v0;  // [bp-0x10]

    v0 = a2->field_40;
    sub_41b7b3(a0, g_4499d8, g_4499d0, a1, g_449958, g_44997c, a2->field_28, v0);
    return 1;
}



// Function: pred_fprint @ 0x954f
typedef struct struct_0 {
    char padding_0[64];
    long long field_40;
    char padding_48[8];
    char field_50;
    char padding_51[7];
    long long field_58;
} struct_0;

unsigned int pred_fprint(long long a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    sub_41c643(a2->field_40, a2->field_58, a2->field_50, "%s\n", a0);
    return 1;
}



// Function: pred_fprint0 @ 0x959f
typedef struct struct_0 {
    char padding_0[64];
    unsigned long long field_40;
} struct_0;

unsigned int pred_fprint0(char *a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x10]

    v0 = a1;
    v1 = a2->field_40;
    fputs(a0, v1);
    putc(0, v1);
    return 1;
}



// Function: pred_fstype @ 0x95ee
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

unsigned int pred_fstype(long long a0, long long a1, struct_0 *a2)
{
    char *v1;  // rax
    unsigned int v2;  // eax

    v1 = sub_413a18(a1, a0, a0);
    v2 = (!strcmp(v1, a2->field_38) ? 1 : 0);
    return v2;
}



// Function: pred_gid @ 0x9643
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

unsigned int pred_gid(unsigned long long a0, struct_1 *a1, struct_0 *a2)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    v2 = a2->field_38;
    if (v2 != 2)
    {
        if (v2 <= 2)
        {
            if (v2)
            {
                if (v2 == 1 && a1->field_20 < a2->field_40)
                    return 1;
            }
            else
            {
                if (a1->field_20 > a2->field_40)
                    return 1;
            }
        }
    }
    else
    {
        if (a1->field_20 == a2->field_40)
            return 1;
    }
    return 0;
}



// Function: pred_group @ 0x96d8
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
} struct_0;

unsigned int pred_group(unsigned long long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    return (a2->field_38 == *((int *)(a1 + 32)) ? 1 : 0);
}



// Function: pred_ilname @ 0x970c
long long pred_ilname(long long a0, long long a1, long long a2)
{
    return sub_409975(a0, a1, a2, 1);
}



// Function: pred_name_common @ 0x973f
int pred_name_common(long long a0, char *a1, int a2)
{
    char v0;  // [bp-0x11]
    unsigned long v1;  // [bp-0x10]

    v1 = sub_41d85c(a0);
    sub_41da6f(v1);
    v0 = !fnmatch(a1, v1, a2);
    free(v1);
    return v0;
}



// Function: pred_iname @ 0x97a2
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

int pred_iname(long long a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    return sub_40973f(a0, a2->field_38, 16);
}



// Function: pred_inum @ 0x97d8
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

unsigned int pred_inum(unsigned long long a0, unsigned long a1, struct_0 *a2)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    v2 = a2->field_38;
    if (v2 != 2)
    {
        if (v2 <= 2)
        {
            if (v2)
            {
                if (v2 == 1 && *((long long *)(a1 + 8)) < a2->field_40)
                    return 1;
            }
            else
            {
                if (*((long long *)(a1 + 8)) > a2->field_40)
                    return 1;
            }
        }
    }
    else
    {
        if (*((long long *)(a1 + 8)) == a2->field_40)
            return 1;
    }
    return 0;
}



// Function: pred_ipath @ 0x986a
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

unsigned int pred_ipath(char *a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    return (!fnmatch(a2->field_38, a0, 16) ? 1 : 0);
}



// Function: pred_links @ 0x98b0
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    char padding_3c[4];
    unsigned long long field_40;
} struct_0;

unsigned int pred_links(unsigned long long a0, unsigned long a1, struct_0 *a2)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    v2 = a2->field_38;
    if (v2 != 2)
    {
        if (v2 <= 2)
        {
            if (v2)
            {
                if (v2 == 1 && *((long long *)(a1 + 16)) < a2->field_40)
                    return 1;
            }
            else
            {
                if (*((long long *)(a1 + 16)) > a2->field_40)
                    return 1;
            }
        }
    }
    else
    {
        if (*((long long *)(a1 + 16)) == a2->field_40)
            return 1;
    }
    return 0;
}



// Function: pred_lname @ 0x9942
long long pred_lname(long long a0, long long a1, long long a2)
{
    return sub_409975(a0, a1, a2, 0);
}



// Function: match_lname @ 0x9975
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[56];
    char *field_38;
} struct_1;

extern long long g_4499d0;
extern unsigned int g_4499d8;
extern unsigned int g_4499e4;

int match_lname(long long a0, struct_0 *a1, struct_1 *a2, char a3)
{
    unsigned int *err;  // rax
    char v0;  // [bp-0x11]
    void* v1;  // [bp-0x10]

    v0 = 0;
    if (((unsigned short)a1->field_18 & 0xf000) == 0xa000)
    {
        v1 = sub_41d06d(g_4499d8, g_4499d0, g_4499d0);
        if (!v1)
        {
            err = __errno_location();
            sub_410a13(*(err), a0, a0);
            g_4499e4 = 1;
        }
        else if (!fnmatch(a2->field_38, v1, (!a3 ? 0 : 16)))
        {
            v0 = 1;
        }
        free(v1);
    }
    return v0;
}



// Function: pred_ls @ 0x9a33
typedef struct struct_0 {
    char padding_0[40];
    char field_28;
    char padding_29[23];
    unsigned long long field_40;
} struct_0;

unsigned int pred_ls(long long a0, long long a1, struct_0 *a2)
{
    return sub_4094e6(a0, a1, a2);
}



// Function: pred_mmin @ 0x9a64
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_mmin(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned int v2;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    return sub_408c32(sub_430a80(a1), v2, a2, 60);
}



// Function: pred_mtime @ 0x9aaf
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_mtime(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned int v2;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    return sub_408c32(sub_430a80(a1), v2, a2, 86400);
}



// Function: pred_name @ 0x9afa
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

int pred_name(long long a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    return sub_40973f(a0, a2->field_38, 0);
}



// Function: pred_negate @ 0x9b30
typedef struct struct_0 {
    char padding_0[280];
    long long field_118;
} struct_0;

long long pred_negate(long long a0, long long a1, struct_0 *a2)
{
    return !(char)sub_410873(a0, a1, a2->field_118, a1);
}



// Function: pred_newer @ 0x9b79
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;

char pred_newer(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned long long v2;  // rax
    unsigned int v3;  // edx
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    if (!a2->field_3c)
    {
        v2 = sub_430a80(a1);
        return sub_408ba9(v2, v3, *((long long *)&a2->field_40), a2->field_48) > 0;
    }
    __assert_fail(); /* do not return */
}



// Function: pred_newerXY @ 0x9c02
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned long long field_48;
} struct_0;


unsigned int pred_newerXY(long long a0, long long a1, struct_0 *a2)
{
    long long v4;  // rdx
    unsigned long long v6;  // rax
    char v0;  // [bp-0x29]
    unsigned long long v1;  // [bp-0x28]
    long long v2;  // [bp-0x20]

    v0 = 0;
    if (a2->field_3c)
        __assert_fail(); /* do not return */
    switch (a2->field_38)
    {
    case 0:
        v1 = sub_430a4c(a1);
        v2 = v4;
        v0 = 1;
        goto LABEL_409d7d;
    case 1:
        v1 = sub_430a9a(a1);
        v2 = v4;
        v0 = 1;
        if (v2 < 0)
        {
            sub_41096a(0, a0);
            error(0, 0, gettext("WARNING: cannot determine birth time of file %s"));
            return 0;
        }
    case 2:
        v1 = sub_430a66(a1);
        v2 = v4;
        v0 = 1;
        goto LABEL_409d7d;
    case 3:
        v1 = sub_430a80(a1);
        v2 = v4;
        v0 = 1;
        goto LABEL_409d7d;
    case 4:
        if (a2->field_38 == 4)
            __assert_fail(); /* do not return */
        return 0;
    default:
LABEL_409d7d:
        if (v0)
        {
            v6 = sub_408ba9(v1, v2, *((long long *)&a2->field_40), a2->field_48);
            return _INSERT(v6, 0, (unsigned int)v6 > 0);
        }
        __assert_fail(); /* do not return */
    }
}



// Function: pred_nogroup @ 0x9dd5
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

char pred_nogroup(unsigned long long a0, struct_0 *a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]

    v1 = a0;
    v0 = a2;
    return !getgrgid(a1->field_20);
}



// Function: pred_nouser @ 0x9e03
typedef struct struct_0 {
    char padding_0[28];
    unsigned int field_1c;
} struct_0;

char pred_nouser(unsigned long long a0, struct_0 *a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]

    v1 = a0;
    v0 = a2;
    return !getpwuid(a1->field_1c);
}



// Function: is_ok @ 0x9e31
extern void stderr;
extern long long stdout;

long long is_ok(unsigned long long a0, unsigned long long a1)
{
    char *v3;  // rax
    char *v4;  // rax
    int *err;  // rax
    long long v6;  // rdi
    long long v7;  // rsi
    long long v8;  // rdx
    long long v9;  // rcx
    long long v10;  // r8
    long long v11;  // r9
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]

    v1 = a0;
    v0 = a1;
    sub_41e1a8(stdout);
    v3 = gettext("< %s ... %s > ? ");
    if (fprintf(*((long long *)&stderr), v3) >= 0)
    {
        sub_41e1a8(*((long long *)&stderr));
        return sub_432d7c(v6, v7, v8, v9, v10, v11);
    }
    v4 = gettext("Failed to write prompt for -ok");
    err = __errno_location();
    error(1, *(err), v4);
}



// Function: pred_ok @ 0x9ece
typedef struct struct_0 {
    char padding_0[232];
    struct struct_1 *field_e8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long pred_ok(unsigned long a0, long long a1, struct_0 *a2)
{
    return (!(char)sub_409e31(a2->field_e8->field_0, a0) ? 0 : sub_40b300(a0, a1, a2, a1));
}



// Function: pred_okdir @ 0x9f27
typedef struct struct_0 {
    char padding_0[232];
    struct struct_1 *field_e8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern long long g_4499d0;

unsigned long long pred_okdir(unsigned long long a0, long long a1, struct_0 *a2)
{
    return (!(char)sub_409e31(a2->field_e8->field_0, a0) ? 0 : sub_40b300(g_4499d0, a1, a2, a1));
}



// Function: pred_openparen @ 0x9f83
unsigned int pred_openparen(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return 1;
}



// Function: pred_or @ 0x9f9e
unsigned long long pred_or(long long a0, long long a1, long long *a2)
{
    if (a2[0x22] && (char)sub_410873(a0, a1, a2[0x22], a1) == 1)
        return 1;
    return sub_410873(a0, a1, a2[35], a1);
}



// Function: pred_path @ 0xa012
typedef struct struct_0 {
    char padding_0[56];
    char *field_38;
} struct_0;

unsigned int pred_path(char *a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    return (!fnmatch(a2->field_38, a0, 0) ? 1 : 0);
}



// Function: pred_perm @ 0xa058
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[56];
    unsigned int field_38;
} struct_1;

unsigned long long pred_perm(unsigned long long a0, struct_0 *a1, struct_1 *a2)
{
    unsigned int v4;  // eax
    unsigned long long v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    v0 = a0;
    v1 = a1->field_18;
    v2 = *((int *)&a2->padding_0[12 + 4 * (unsigned int)(((v1 & 0xf000) == 0x4000) + 12)]);
    v4 = a2->field_38;
    if (v4 == 2)
        return _INSERT(v1 & 0xfff, 0, v2 == (v1 & 0xfff));
    if (v4 <= 2)
    {
        if (!v4)
        {
            return _INSERT(v1 & v2, 0, v2 == (v1 & v2));
        }
        else if (v4 == 1)
        {
            return (!v2 ? 1 : _INSERT(v1 & v2, 0, v1 & v2));
        }
    }
    abort(); /* do not return */
}



// Function: pred_executable @ 0xa0fa
extern long long g_4499d0;
extern unsigned int g_4499d8;

char pred_executable(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    return !faccessat(g_4499d8, g_4499d0, 1, 0);
}



// Function: pred_readable @ 0xa137
extern long long g_4499d0;
extern unsigned int g_4499d8;

char pred_readable(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    return !faccessat(g_4499d8, g_4499d0, 4, 0);
}



// Function: pred_writable @ 0xa174
extern long long g_4499d0;
extern unsigned int g_4499d8;

char pred_writable(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    return !faccessat(g_4499d8, g_4499d0, 2, 0);
}



// Function: pred_print @ 0xa1b1
typedef struct struct_0 {
    char padding_0[64];
    long long field_40;
    char padding_48[8];
    char field_50;
    char padding_51[7];
    long long field_58;
} struct_0;

unsigned int pred_print(long long a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    sub_41c643(a2->field_40, a2->field_58, a2->field_50, "%s\n", a0);
    return 1;
}



// Function: pred_print0 @ 0xa201
typedef struct struct_0 {
    char padding_0[64];
    unsigned long long field_40;
} struct_0;

unsigned int pred_print0(char *a0, unsigned long long a1, struct_0 *a2)
{
    return sub_40959f(a0, a1, a2);
}



// Function: pred_prune @ 0xa232
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_449940;
extern char g_4499c4;
extern char g_4499e0;

unsigned int pred_prune(unsigned long long a0, struct_0 *a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]

    v1 = a0;
    v0 = a2;
    if (!g_449940)
    {
        return 1;
    }
    else if (!g_4499c4)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        if (!a1 || ((unsigned short)a1->field_18 & 0xf000) != 0x4000)
            return 1;
        g_4499e0 = 1;
        return 1;
    }
}



// Function: pred_quit @ 0xa2b0
extern int g_4499e4;

void pred_quit(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    sub_40fa38();
    exit(g_4499e4); /* do not return */
}



// Function: pred_regex @ 0xa2da
typedef struct struct_0 {
    char padding_0[56];
    long long field_38;
} struct_0;

unsigned int pred_regex(unsigned long a0, unsigned long long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0xc]

    v0 = a1;
    v1 = strlen(a0);
    return (v1 == re_match(a2->field_38, a0, v1, 0, 0) ? 1 : 0);
}



// Function: pred_size @ 0xa336
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
    int field_3c;
    unsigned long long field_40;
} struct_0;

unsigned int pred_size(unsigned long long a0, void* a1, struct_0 *a2)
{
    unsigned int v3;  // eax
    unsigned long long v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x10]

    v0 = a0;
    v1 = ((long long)a1[48] % a2->field_3c) + (long long)a1[48] / a2->field_3c;
    v3 = a2->field_38;
    if (v3 != 2)
    {
        if (v3 <= 2)
        {
            if (v3)
            {
                if (v3 == 1 && v1 < a2->field_40)
                    return 1;
            }
            else
            {
                if (v1 > a2->field_40)
                    return 1;
            }
        }
    }
    else
    {
        if (v1 == a2->field_40)
            return 1;
    }
    return 0;
}



// Function: pred_samefile @ 0xa3f7
extern long long g_4499d0;

unsigned int pred_samefile(long long a0, unsigned long long *a1, unsigned long a2)
{
    unsigned int v1;  // eax

    if (a1[1] && a1[1] != *((long long *)(a2 + 56)))
        return 0;
    if (sub_40f27d(a0, g_4499d0, a1, g_4499d0))
        return 0;
    if (a1[1] == *((long long *)(a2 + 56)) && *(a1) == *((long long *)(a2 + 64)))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: pred_true @ 0xa497
unsigned int pred_true(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return 1;
}



// Function: pred_type @ 0xa4b2
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4499c4;
extern char g_4499c5;
extern unsigned int g_4499c8;

unsigned int pred_type(unsigned long long a0, struct_0 *a1, unsigned long a2)
{
    unsigned int v4;  // eax
    unsigned long long v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    v0 = a0;
    v2 = 7;
    if (!g_4499c5)
    {
        __assert_fail(); /* do not return */
    }
    else if (!g_4499c8)
    {
        return 0;
    }
    else
    {
        v1 = (!g_4499c4 ? g_4499c8 : a1->field_18);
        v4 = v1 & 0xf000;
        if (v4 == 0xc000)
        {
            v2 = 6;
        }
        else if (v4 <= 0xc000)
        {
            if (v4 == 0xa000)
            {
                v2 = 4;
            }
            else if (v4 <= 0xa000)
            {
                if (v4 == 0x8000)
                {
                    v2 = 3;
                }
                else if (v4 <= 0x8000)
                {
                    if (v4 == 0x6000)
                    {
                        v2 = 0;
                    }
                    else if (v4 <= 0x6000)
                    {
                        if (v4 == 0x4000)
                        {
                            v2 = 2;
                        }
                        else if (v4 <= 0x4000)
                        {
                            switch (v4)
                            {
                            case 4096:
                                v2 = 5;
                                break;
                            case 8192:
                                v2 = 1;
                                break;
                            default:
                                goto LABEL_40a5dc;
                            }
                        }
                    }
                }
            }
        }
LABEL_40a5dc:
        if (v2 == 7 || !*((char *)(a2 + v2 + 56)))
            return 0;
        return 1;
        return 0;
    }
}



// Function: pred_uid @ 0xa600
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

unsigned int pred_uid(unsigned long long a0, struct_1 *a1, struct_0 *a2)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    v2 = a2->field_38;
    if (v2 != 2)
    {
        if (v2 <= 2)
        {
            if (v2)
            {
                if (v2 == 1 && a1->field_1c < a2->field_40)
                    return 1;
            }
            else
            {
                if (a1->field_1c > a2->field_40)
                    return 1;
            }
        }
    }
    else
    {
        if (a1->field_1c == a2->field_40)
            return 1;
    }
    return 0;
}



// Function: pred_used @ 0xa695
typedef struct struct_0 {
    char padding_0[60];
    unsigned int field_3c;
    unsigned int field_40;
    char padding_44[4];
    unsigned int field_48;
} struct_0;

unsigned int pred_used(unsigned long long a0, long long a1, struct_0 *a2)
{
    unsigned long v6;  // rdx
    unsigned long long v0;  // [bp-0x40]
    unsigned long long v1;  // [bp-0x38]
    long long v2;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x18]

    v0 = a0;
    v3 = sub_430a4c(a1);
    v4 = sub_430a66(a1);
    if (sub_408ba9(v3, v6, v4, v6) < 0)
        return 0;
    v1 = v4 - v3;
    v2 = v6 - v6;
    if (v2 < 0)
    {
        v2 += 1000000000;
        v1 -= 1;
    }
    return sub_408c32(v1, v2, a2, 86400);
}



// Function: pred_user @ 0xa75a
typedef struct struct_0 {
    char padding_0[56];
    unsigned int field_38;
} struct_0;

unsigned int pred_user(unsigned long long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    return (a2->field_38 == *((int *)(a1 + 28)) ? 1 : 0);
}



// Function: pred_xtype @ 0xa78e
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern unsigned long long g_4499d0;
extern unsigned int g_4499e4;

unsigned int pred_xtype(unsigned long a0, struct_0 *a1, unsigned int a2)
{
    int *err;  // rax
    unsigned long long *v0;  // [bp-0xc0]
    struct_0 v1;  // [bp-0xb8]

    v0 = (!(char)sub_40fe03() ? sub_40fc21 : sub_40fcc0);
    sub_40f26e(&v1);
    if (!v0(g_4499d0, &v1, &v1, v0))
        return sub_40a4b2(a0, &v1, a2);
    if ((char)sub_40fe03() && *(__errno_location()) == 2)
        return sub_40a4b2(a0, a1, a2);
    sub_41096a(0, a0);
    err = __errno_location();
    error(0, *(err), "%s");
    g_4499e4 = 1;
    return 0;
}



// Function: pred_context @ 0xa8d8
typedef struct struct_1 {
    char padding_0[56];
    char *field_38;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_4499a0;
extern unsigned long long g_4499d0;
extern unsigned int g_4499d8;

unsigned int pred_context(long long a0, unsigned long long a1, struct_1 *a2)
{
    char *v4;  // rax
    int *err;  // rax
    unsigned long long v0;  // [bp-0x48]
    unsigned int v1;  // [bp-0x2c]
    unsigned long v2;  // [bp-0x28]

    v0 = a1;
    v1 = g_4499a0(g_4499d8, g_4499d0, &v2, g_4499d0, g_4499a0);
    if (v1 >= 0)
    {
        v1 = !fnmatch(a2->field_38, v2, 0);
        return _INSERT(freecon(v2), 0, v1);
    }
    sub_41096a(0, a0);
    v4 = gettext("getfilecon failed: %s");
    err = __errno_location();
    error(0, *(err), v4);
    return 0;
}



// Function: blank_rtrim @ 0xa9cd
char * blank_rtrim(char *a0, char *a1)
{
    unsigned int iter;  // [bp-0xc]

    if (!a0)
        return NULL;
    strcpy(a1, a0);
    for (iter = (unsigned int)strlen(a1) - 1; iter >= 0 && (a1[iter] == 32 || a1[iter] == 9); iter -= 1);
    iter += 1;
    a1[iter] = 0;
    return a1;
}



// Function: print_list @ 0xaa62
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char padding_10[248];
    struct struct_0 *field_108;
} struct_0;

void print_list(FILE *ptr, struct_0 *a1)
{
    struct_0 *iter;  // [bp-0x120]
    char v1[264];  // [bp-0x118]

    for (iter = a1; iter; iter = iter->field_108)
    {
        fprintf(ptr, "[%s] ", sub_40a9cd(iter->field_8, v1));
    }
    fprintf(ptr, "\n");
    return;
}



// Function: print_parenthesised @ 0xab30
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[264];
    unsigned long long field_110;
    void* field_118;
} struct_0;

unsigned long long print_parenthesised(unsigned long a0, struct_0 *a1)
{
    unsigned long long v2;  // rax
    unsigned long long v3;  // rax
    unsigned int v0;  // [bp-0xc]

    v0 = 0;
    if (!a1)
        return v3;
    if ((a1->field_0 == sub_409f9e || a1->field_0 == sub_408d9c) && !a1->field_110)
        return print_parenthesised(a0, a1->field_118);
    if (a1->field_110 || a1->field_118)
        v0 = 1;
    if (v0)
        fprintf(a0, "%s", " ( ");
    v2 = sub_40ac38(a0, a1, a1);
    if (!v0)
        return v2;
    return fprintf(a0, "%s", " ) ");
}



// Function: print_optlist @ 0xac38
extern char g_4387b2;
extern unsigned long long g_449980;

unsigned long long print_optlist(unsigned long ptr, void* idx)
{
    unsigned long v2;  // rax
    unsigned long v3;  // rax
    unsigned long long v4;  // rax
    double v0;  // [bp-0x10]

    if (!idx)
        return v4;
    sub_40ab30(ptr, (long long)idx[272]);
    fprintf(ptr, "%s%s%s", (!(char)idx[26] ? &g_4387b2 : "[call stat] "), (!(char)idx[27] ? &g_4387b2 : "[need type] "), (!(char)idx[28] ? &g_4387b2 : "[need inum] "));
    sub_40c2dc(ptr, idx, idx);
    fprintf(ptr, " [est success rate %.4g] ", (unsigned long long)(int)idx[36]);
    if (!((char)g_449980 & 64))
        return sub_40ab30(ptr, (long long)idx[280]);
    fprintf(ptr, "[real success rate %lu/%lu", (long long)idx[296], (long long)idx[288]);
    if ((long long)idx[288])
    {
        v2 = (long long)idx[296];
        v3 = (long long)idx[288];
        v0 = (double)(unsigned long long)(DivV((v2 < 0 ? AddV(v2 >> 1 | (unsigned int)v2 & 1, v2 >> 1 | (unsigned int)v2 & 1) : v2), (v3 < 0 ? AddV(v3 >> 1 | (unsigned int)v3 & 1, v3 >> 1 | (unsigned int)v3 & 1) : v3)));
        fprintf(ptr, "=%.4g] ", v0);
    }
    else
    {
        fprintf(ptr, "=_] ");
    }
    return sub_40ab30(ptr, (long long)idx[280]);
}



// Function: show_success_rates @ 0xae53
extern void stderr;
extern unsigned long long g_449980;

unsigned long long show_success_rates(void* a0)
{
    unsigned long long v1;  // rax

    v1 = (unsigned int)g_449980 & 64;
    if (!v1)
        return v1;
    fprintf(*((long long *)&stderr), "Predicate success rates after completion:\n");
    sub_40ac38(*((long long *)&stderr), a0);
    return fprintf(*((long long *)&stderr), "\n");
}



// Function: pred_sanity_check @ 0xaec7
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


struct_0 * pred_sanity_check(struct_0 *a0)
{
    struct_0 *v2;  // rax
    struct_0 *iter;  // [bp-0x10]

    v2 = a0;
    iter = v2;
    while (iter)
    {
        if (!iter->field_0)
        {
            __assert_fail(); /* do not return */
        }
        else if (!iter->field_130)
        {
            __assert_fail(); /* do not return */
        }
        else
        {
            if (*((long long *)&iter->field_130[6]) && *((long long *)&iter->field_130[6]) != iter->field_0)
                __assert_fail(); /* do not return */
            switch (iter->field_130->field_0)
            {
            case 0: case 2:
                if (!iter->field_130->field_0)
                {
                    __assert_fail(); /* do not return */
                }
                else if (iter->field_130->field_0 == 2)
                {
                    __assert_fail(); /* do not return */
                }
            case 1: case 3: case 4: case 5:
                if (iter->field_19 == 1)
                {
                    __assert_fail(); /* do not return */
                }
                else if (iter->field_18 == 1)
                {
                    __assert_fail(); /* do not return */
                }
            case 6:
                if (!iter->field_18)
                    __assert_fail(); /* do not return */
                if (iter->field_0 != sub_40a232 && iter->field_0 != sub_40a2b0 && !iter->field_19)
                    __assert_fail(); /* do not return */
                goto LABEL_40b166;
            default:
LABEL_40b166:
                v2 = iter->field_108;
                iter = iter->field_108;
                break;
            }
        }
    }
    return v2;
}



// Function: initialize_wd_for_exec @ 0xb184
typedef struct struct_0 {
    char padding_0[192];
    void* field_c0;
} struct_0;

unsigned int initialize_wd_for_exec(struct_0 *idx, unsigned int a1, long long a2)
{
    unsigned int v1;  // eax

    idx->field_c0 = sub_431bc2(16);
    *((unsigned long long *)&idx->field_c0[8]) = 0;
    v1 = sub_42704e(a1, a2, 0, a2);
    *((unsigned int *)idx->field_c0) = v1;
    if (*((int *)idx->field_c0) >= 0)
    {
        sub_41d5ba(*((int *)idx->field_c0), 1);
        return 1;
    }
    return 0;
}



// Function: record_exec_dir @ 0xb22b
typedef struct struct_0 {
    char padding_0[192];
    void* field_c0;
} struct_0;

extern void g_4499d0;
extern unsigned int g_4499d8;

unsigned int record_exec_dir(struct_0 *a0)
{
    char v0;  // [bp-0x11]
    unsigned long v1;  // [bp-0x10]

    if (*((int *)&a0->padding_0[152]))
    {
        return 1;
    }
    else if (*((int *)&a0->padding_0[152]))
    {
        __assert_fail(); /* do not return */
    }
    else if (strchr(*((long long *)&g_4499d0), 47))
    {
        v1 = sub_41d9d4(*((long long *)&g_4499d0));
        v0 = sub_40b184(a0, g_4499d8, v1);
        free(v1);
        return v0;
    }
    else
    {
        return sub_40b184(a0, g_4499d8, ".");
    }
}



// Function: impl_pred_exec @ 0xb300
typedef struct struct_0 {
    long long field_0;
    char padding_8[48];
    char field_38;
    char padding_39[151];
    unsigned int field_d0;
    char padding_d4[20];
    unsigned long long field_e8;
    int field_f0;
    char padding_f4[4];
    long long field_f8;
    int field_100;
} struct_0;

extern long long g_4499d0;
extern char g_4499e8;
extern char g_449a00;

int impl_pred_exec(char *a0, unsigned long long a1, struct_0 *a2)
{
    char *v10;  // rax
    int *err;  // rax
    unsigned long len1;  // rbx
    unsigned long len2;  // rax
    unsigned long long v0;  // [bp-0x58]
    char v1;  // [bp-0x46]
    char v2;  // [bp-0x45]
    int i;  // [bp-0x44]
    char *v4;  // [bp-0x40]
    char *ptr;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    long long v7;  // [bp-0x28]
    struct_0 *v8;  // [bp-0x20]

    v0 = a1;
    v8 = &a2->field_38;
    v4 = NULL;
    v2 = sub_410933(a2->field_0);
    if (!v2)
    {
        if (*((long long *)&v8->padding_39[135]) != *((long long *)&g_449a00))
            __assert_fail(); /* do not return */
        ptr = a0;
        v6 = 0;
        v7 = 0;
    }
    else if ((char)sub_40b22b(v8) != 1)
    {
        sub_41096a(0, a0);
        v10 = gettext("Failed to save working directory in order to run a command on %s");
        err = __errno_location();
        error(1, *(err), v10);
    }
    else
    {
        v4 = sub_41d85c(g_4499d0);
        ptr = v4;
        if (*(ptr) == 47)
        {
            v6 = 0;
            v7 = 0;
        }
        else
        {
            v6 = "./";
            v7 = 2;
        }
    }
    if ((char)v8->field_0)
    {
        sub_41a1b8(v8->padding_8, &v8->padding_39[39], ptr, strlen(ptr) + 1, v6, v7, 0);
        if (*((int *)&v8->padding_39[95]))
            g_4499e8 = 1;
        v1 = 1;
    }
    else
    {
        for (i = 0; i < *((int *)&v8->padding_39[127]); i += 1)
        {
            len1 = strlen(ptr);
            len2 = strlen(*((long long *)(i * 8 + *((long long *)&v8->padding_39[0x77]))));
            sub_419af7(v8->padding_8, &v8->padding_39[39], *((long long *)(i * 8 + *((long long *)&v8->padding_39[0x77]))), len2, v6, v7, ptr, len1, 0);
        }
        sub_419f5a(v8->padding_8, &v8->padding_39[39], &v8->padding_39[39]);
        v1 = (!(*((int *)&v8->padding_39[143]) & 127) ? (!(*((int *)&v8->padding_39[143]) >> 8 & 0xff) ? 1 : 0) : 0);
        if (v2)
            sub_42fe52(*((long long *)&v8->padding_39[135]));
    }
    if (v4)
    {
        if (!v2)
            __assert_fail(); /* do not return */
        free(v4);
    }
    return v1;
}



// Function: prep_child_for_exec @ 0xb615
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

long long prep_child_for_exec(char a0, struct_0 *a1)
{
    char *v3;  // rax
    int *err;  // rax
    int *err1;  // rax
    char *v6;  // rax
    int *err2;  // rax
    char v0;  // [bp-0x3b]
    char v1[10];  // [bp-0x3a]

    v0 = 1;
    if (a0)
    {
        strncpy(v1, "/dev/null", 9);
        if (close(0) < 0)
        {
            v3 = gettext("Cannot close standard input");
            err = __errno_location();
            error(0, *(err), v3);
            v0 = 0;
        }
        else if (sub_41e034(v1, 0) < 0)
        {
            sub_41096a(0, v1);
            err1 = __errno_location();
            error(0, *(err1), "%s");
        }
    }
    if (!sub_42fe17(a1))
        return v0;
    v6 = gettext("Failed to change directory%s%s");
    err2 = __errno_location();
    error(0, *(err2), v6);
    v0 = 0;
    return v0;
}



// Function: launch @ 0xb7ae
extern unsigned int g_44901c;
extern void stderr;
extern long long stdout;
extern unsigned long long g_449980;
extern unsigned int g_4499ac;
extern unsigned int g_4499e4;

unsigned int launch(unsigned long long a0, void* a1, unsigned int a2, void* a3)
{
    char *v6;  // rax
    char *v7;  // rax
    int *err;  // rax
    int *err1;  // rax
    char *v10;  // rax
    int *err2;  // rax
    unsigned int v0;  // [bp-0x4c]
    unsigned long long v1;  // [bp-0x40]
    int i;  // [bp-0x2c]
    unsigned int v3;  // [bp-0x28]
    unsigned int v4;  // [bp-0x24]

    v1 = a0;
    v0 = a2;
    if ((char)g_449980 & 32)
    {
        fprintf(*((long long *)&stderr), "DebugExec: launching process (argc=%lu):", (long long)a1[96] - 1);
        for (i = 0; i < (long long)a1[96] - 1; i += 1)
        {
            v6 = sub_41096a(0, *((long long *)(i * 8 + (long long)a1[104])), (long long)a1[104]);
            fprintf(*((long long *)&stderr), " %s", v6);
        }
        fprintf(*((long long *)&stderr), "\n");
    }
    sub_41e1a8(stdout);
    sub_41e1a8(*((long long *)&stderr));
    if (g_44901c)
    {
        g_44901c = 0;
        signal(0x11, NULL);
    }
    v3 = fork();
    if (v3 == 0xffffffff)
    {
        v7 = gettext("cannot fork");
        err = __errno_location();
        error(1, *(err), v7);
    }
    else if (v3)
    {
        while (waitpid(v3, a1 + 200, 0) == 0xffffffff)
        {
            if (*(__errno_location()) != 4)
            {
                sub_41096a(0, *((long long *)a3));
                v10 = gettext("error waiting for %s");
                err2 = __errno_location();
                error(0, *(err2), v10);
                g_4499e4 = 1;
                return 0;
            }
        }
        if (((char)(int)a1[200] & 127) + 1 >> 1 > 0)
        {
            sub_42f861(0, g_4499ac, *((long long *)a3));
            error(0, 0, gettext("%s terminated by signal %d"));
            if (!*((char *)a1))
                return 1;
            g_4499e4 = 1;
            return 1;
        }
        else
        {
            v4 = (int)a1[200] >> 8 & 0xff;
            if ((char)g_449980 & 32)
                fprintf(*((long long *)&stderr), "DebugExec: process (PID=%ld) terminated with exit status: %d\n", v3, v4);
            if (!v4)
                return 1;
            if (!*((char *)a1))
                return 1;
            g_4499e4 = 1;
            return 1;
        }
    }
    else if (!(long long)a1[192])
    {
        __assert_fail(); /* do not return */
    }
    else if ((char)sub_40b615((char)a1[188], (long long)a1[192]) != 1)
    {
        _exit(1); /* do not return */
    }
    else
    {
        if ((char)sub_41b5f4())
            sub_41b61e();
        if ((char)sub_41a94d(a3))
            *(__errno_location()) = 7;
        else
            execvp(*((long long *)a3), a3);
        sub_41096a(0, *((long long *)a3));
        err1 = __errno_location();
        error(0, *(err1), "%s");
        _exit(1); /* do not return */
    }
}



// Function: matches_start_point @ 0xbb9d
extern unsigned long long g_449a20;
extern unsigned long long g_449a28;

unsigned int matches_start_point(char *a0, char a1)
{
    unsigned long long v3;  // rax
    unsigned int v0;  // [bp-0x14]
    unsigned long long i;  // [bp-0x10]

    v0 = 0;
    if (a1)
        v0 |= 16;
    if (!g_449a28)
    {
        v3 = fnmatch(a0, ".", v0);
        return _INSERT(v3, 0, !(unsigned int)v3);
    }
    for (i = 0; i < g_449a28; i += 1)
    {
        if (!fnmatch(a0, *((long long *)(i * 8 + g_449a20)), v0))
            return 1;
    }
    return 0;
}



// Function: get_expr @ 0xbc44
typedef struct struct_3 {
    char padding_0[16];
    unsigned int field_10;
} struct_3;

typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[17];
    char field_29;
    char padding_2a[222];
    struct struct_0 *field_108;
    char padding_110[8];
    unsigned long long field_118;
} struct_1;

typedef struct struct_0 {
    char padding_0[41];
    char field_29;
} struct_0;


struct_1 * get_expr(void* *a0, short a1, struct_3 *a2)
{
    struct_3 *v0;  // [bp-0x40], Other Possible Types: void*
    void* v1;  // [bp-0x28], Other Possible Types: unsigned long long
    void* v2;  // [bp-0x20]

    v0 = a2;
    v1 = 0;
    v2 = *(a0);
    if (!*(a0))
        error(1, 0, gettext("invalid expression"));
    switch ((int)(*(a0))[16])
    {
    case 0:
        error(1, 0, gettext("invalid expression"));
    case 1:
        v1 = *(a0);
        *(a0) = (long long)(*(a0))[264];
        break;
    case 2:
        v1 = *(a0);
        *(a0) = (long long)(*(a0))[264];
        *((struct_1 **)&v1[280]) = get_expr(a0, 4, v1);
        break;
    case 3:
        error(1, 0, gettext("invalid expression; you have used a binary operator '%s' with nothing before it."));
    case 4:
        if (!(long long)(*(a0))[264] || *((char *)((long long)(*(a0))[264] + 41)))
            error(1, 0, gettext("invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'"));
        v0 = *(a0);
        *(a0) = (long long)(*(a0))[264];
        if ((int)(*(a0))[16] != 5)
        {
            v1 = get_expr(a0, 0, v0);
            if (!*(a0) || (int)(*(a0))[16] != 5)
                error(1, 0, gettext("invalid expression; I was expecting to find a ')' somewhere but did not see one."));
            *(a0) = (long long)(*(a0))[264];
            break;
        }
        else if ((char)v0[41])
        {
            error(1, 0, gettext("invalid expression: expected expression before closing parentheses '%s'."));
        }
        else
        {
            error(1, 0, gettext("invalid expression; empty parentheses are not allowed."));
        }
        if (!*(a0))
            return v1;
        if ((int)(*(a0))[20] > a1)
        {
            v1 = sub_40c08f(a0, v1, a1, v1);
            if (!v1)
                error(1, 0, gettext("invalid expression"));
        }
        return v1;
    case 5:
        if (!v0)
            error(1, 0, gettext("invalid expression: expected expression before closing parentheses '%s'."));
        if ((v0->field_10 == 2 || v0->field_10 == 3) && (char)v2[41] != 1)
            error(1, 0, gettext("expected an expression between '%s' and ')'"));
        if (!(char)(*(a0))[41])
            error(1, 0, gettext("invalid expression; you have too many ')'"));
        error(1, 0, gettext("expected an expression after '%s'"));
    default:
        error(1, 0, gettext("oops -- invalid expression type!"));
    }
}



// Function: scan_rest @ 0xc08f
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[17];
    char field_29;
    char padding_2a[222];
    struct struct_0 *field_108;
    char padding_110[8];
    unsigned long long field_118;
} struct_1;

typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    int field_14;
    char padding_18[240];
    unsigned long long field_108;
    struct struct_0 *field_110;
    unsigned long long field_118;
} struct_0;

struct_1 * scan_rest(struct_0 **a0, struct_1 *a1, short a2)
{
    unsigned int v3;  // eax
    struct_1 *v0;  // [bp-0x28]
    struct_0 *v1;  // [bp-0x20]

    if (!*(a0) || *(a0)->field_10 == 5)
        return NULL;
    v0 = a1;
    while (*(a0) && *(a0)->field_14 > a2)
    {
        v3 = *(a0)->field_10;
        if (v3 == 5)
        {
            return v0;
        }
        else if (v3 > 5)
        {
            error(1, 0, gettext("oops -- invalid expression type (%d)!"));
        }
        else if (v3 == 4)
        {
            error(1, 0, gettext("invalid expression"));
        }
        else if (v3 > 4)
        {
            error(1, 0, gettext("oops -- invalid expression type (%d)!"));
        }
        else if (v3 <= 2)
        {
            error(1, 0, gettext("invalid expression"));
        }
        else if (v3 == 3)
        {
            v1 = *(a0);
            *(a0)->field_110 = v0;
            v0 = *(a0);
            *(a0) = *(a0)->field_108;
            v0->field_118 = sub_40bc44(a0, v0->field_14, v1);
        }
        else
        {
            error(1, 0, gettext("oops -- invalid expression type (%d)!"));
        }
    }
    return v0;
    return NULL;
}



// Function: predicate_is_cost_free @ 0xc209
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[24];
    unsigned int field_20;
} struct_0;

extern unsigned short g_4499a8;

unsigned long long predicate_is_cost_free(struct_0 *a0)
{
    unsigned long v1;  // rax

    if (a0->field_0 != sub_409afa && a0->field_0 != sub_40a012 && a0->field_0 != sub_4097a2 && a0->field_0 != sub_40986a)
    {
        if (g_4499a8)
        {
            if (a0->field_0 == sub_408d9c || a0->field_0 == sub_409b30 || a0->field_0 == sub_408fd0 || a0->field_0 == sub_409f9e)
                return 0;
            v1 = a0->field_20;
            return _INSERT(v1, 0, !(unsigned int)v1);
            return 0;
        }
        else
        {
            return 0;
        }
    }
    return 1;
}



// Function: print_predicate @ 0xc2dc
typedef struct FILE {
} FILE;

int print_predicate(FILE *a0, char **a1)
{
    return (!a1[6] ? fprintf(a0, "%s", a1[1]) : fprintf(a0, "%s %s", a1[1], a1[6]));
}



// Function: predlist_init @ 0xc34d
unsigned long long * predlist_init(unsigned long long *idx)
{
    idx[1] = 0;
    *(idx) = idx[1];
    return idx;
}



// Function: predlist_insert @ 0xc377
typedef struct struct_1 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_1;

typedef struct struct_5 {
    struct struct_0 *field_0;
    struct struct_1 *field_8;
} struct_5;

typedef struct struct_0 {
    char padding_0[272];
    struct struct_0 *field_110;
} struct_0;

struct_1 * predlist_insert(struct_5 *a0, struct_0 *idx, struct_0 **a2)
{
    struct_1 *v2;  // rax
    struct_5 *v0;  // [bp-0x10]

    v0 = a0;
    *(a2) = idx->field_110;
    idx->field_110 = v0->field_0;
    v0->field_0 = idx;
    v2 = a0->field_8;
    if (!v2)
    {
        v2 = a0;
        v2->field_8 = a0->field_0;
    }
    return v2;
}



// Function: pred_cost_compare @ 0xc3e1
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
    unsigned int field_24;
} struct_0;

typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
} struct_1;

unsigned int pred_cost_compare(struct_0 *a0, struct_1 *a1, char a2)
{
    if (a0->field_20 != *((int *)&a1->padding_0[32]))
        return (a0->field_20 < *((int *)&a1->padding_0[32]) ? 0xffffffff : 1);
    if ((char)((CmpF(a0->field_24, a1->field_24) & 69) >> 2) & 1 || ((char)((CmpF(a0->field_24, a1->field_24) & 69) >> 6) & 1) != 1)
        return (!a2 ? (((CmpF(a1->field_24, a0->field_24) & 69 | (CmpF(a1->field_24, a0->field_24) & 69) >> 6) & 1) == 1 ? 0xffffffff : 1) : (((CmpF(a1->field_24, a0->field_24) & 69 | (CmpF(a1->field_24, a0->field_24) & 69) >> 6) & 1) == 1 ? 1 : 0xffffffff));
    return 0;
    return (!a2 ? (((CmpF(a1->field_24, a0->field_24) & 69 | (CmpF(a1->field_24, a0->field_24) & 69) >> 6) & 1) == 1 ? 0xffffffff : 1) : (((CmpF(a1->field_24, a0->field_24) & 69 | (CmpF(a1->field_24, a0->field_24) & 69) >> 6) & 1) == 1 ? 1 : 0xffffffff));
}



// Function: predlist_merge_sort @ 0xc49b
typedef struct struct_0 {
    char padding_0[20];
    unsigned int field_14;
    char padding_18[248];
    struct struct_0 *field_110;
    struct struct_1 *field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[36];
    unsigned int field_24;
} struct_1;

extern void stderr;
extern unsigned long long g_449980;

void predlist_merge_sort(struct_0 **a0, long long a1)
{
    char v0;  // [bp-0x39]
    struct_0 *iter;  // [bp-0x38]
    struct_0 *index;  // [bp-0x30]
    struct_0 *v3;  // [bp-0x28], Other Possible Types: unsigned long long
    struct_0 *v4;  // [bp-0x20]

    if (!*(a0))
        return;
    if ((char)g_449980 & 8)
    {
        fprintf(*((long long *)&stderr), "%s:\n", "predlist before merge sort");
        sub_40ea9f(*((long long *)&stderr), *(a0), 2, *(a0));
    }
    sub_40d838(*(a0));
    sub_40c34d(&v3);
    while (*(a0))
    {
        index = *(a0);
        *(a0) = *(a0)->field_110;
        index->field_110 = NULL;
        for (iter = v3; iter && (v0 = (char)(char)(iter->field_14 != 2), (unsigned int)(unsigned long long)sub_40c3e1(iter->field_118, index->field_118, v0) < 0); iter = iter->field_110);
        if (iter)
        {
            index->field_110 = iter->field_110;
            if (!index->field_110)
                v4 = index;
            iter->field_110 = index;
        }
        else
        {
            index->field_110 = v3;
            v3 = index;
            if (!v4)
                v4 = index;
        }
    }
    if ((char)g_449980 & 8)
    {
        fprintf(*((long long *)&stderr), "%s:\n", "predlist after merge sort");
        sub_40ea9f(*((long long *)&stderr), v3, 2, v3);
    }
    sub_40d838(v3);
    sub_40d3fd(v3, v4, a1, v4);
    sub_40c34d(a0);
    return;
}



// Function: merge_lists @ 0xc6e0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_449a38;

long long merge_lists(unsigned long a0, unsigned int a1, unsigned long a2, unsigned long a3, unsigned long a4)
{
    unsigned int i;  // [bp-0xc]

    g_449a38 = sub_40c49b;
    g_449a38(a2, a4, a4, g_449a38);
    g_449a38(a3, a4, a4, g_449a38);
    for (i = 0; i < a1; i += 1)
    {
        g_449a38(i * 16 + a0, a4, g_449a38, i * 16 + a0);
    }
    return i;
}



// Function: subtree_has_side_effects @ 0xc77a
typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[247];
    void* field_110;
    void* field_118;
} struct_0;

unsigned int subtree_has_side_effects(struct_0 *a0)
{
    unsigned int v1;  // eax

    if (!a0)
        return 0;
    if (a0->field_18 || (char)subtree_has_side_effects(a0->field_110) || (char)subtree_has_side_effects(a0->field_118))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: worst_cost @ 0xc7e3
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
    char padding_24[236];
    void* field_110;
    void* field_118;
} struct_0;

unsigned int worst_cost(struct_0 *a0)
{
    unsigned int v4;  // edx
    unsigned int v5;  // eax
    unsigned int v0;  // [bp-0x14]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    if (!a0)
        return 0;
    v1 = worst_cost(a0->field_110);
    v2 = worst_cost(a0->field_118);
    v4 = v2;
    v5 = v1;
    if (v5 > v4)
        v4 = v5;
    v0 = v4;
    if (v0 < a0->field_20)
        v0 = a0->field_20;
    return v0;
}



// Function: perform_arm_swap @ 0xc856
typedef struct struct_1 {
    char padding_0[272];
    struct struct_0 *field_110;
    unsigned long long field_118;
} struct_1;

typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

struct_1 * perform_arm_swap(struct_1 *idx)
{
    unsigned long v0;  // [bp-0x10]

    v0 = idx->field_110->field_118;
    idx->field_110->field_118 = idx->field_118;
    idx->field_118 = v0;
    return idx;
}



// Function: consider_arm_swap @ 0xc8a7
typedef struct struct_1 {
    char padding_0[280];
    struct struct_0 *field_118;
} struct_1;

typedef struct struct_3 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[252];
    struct struct_1 *field_110;
    struct struct_0 *field_118;
} struct_3;

typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
} struct_0;

extern void stderr;
extern unsigned long long g_449980;

unsigned int consider_arm_swap(struct_3 *a0)
{
    char v0;  // [bp-0x31]
    unsigned int v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x2c]
    unsigned int v3;  // [bp-0x28]
    unsigned int v4;  // [bp-0x24]
    char *v5;  // [bp-0x20]
    struct_3 *v6;  // [bp-0x18]
    unsigned long v7;  // [bp-0x10]

    v5 = NULL;
    if (a0->field_10 != 3)
        v5 = "Not a binary operation";
    if (!v5 && (!a0->field_110 || !a0->field_118))
        v5 = "Doesn't have two arms";
    if (!v5 && !a0->field_110->field_118)
        v5 = "Left arm has no child on RHS";
    v6 = &a0->field_118;
    v7 = &a0->field_110->field_118;
    if (!v5 && (char)sub_40c77a(*((long long *)v7)))
        v5 = "Left subtree has side-effects";
    if (!v5 && (char)sub_40c77a(v6->field_0))
        v5 = "Right subtree has side-effects";
    if (!v5)
    {
        v1 = sub_40c7e3(*((long long *)v7));
        v2 = sub_40c7e3(v6->field_0);
        if (v1 < v2)
            v5 = "efficient as-is";
    }
    if (!v5)
    {
        if (v1 == v2)
        {
            v3 = *((int *)(*((long long *)v7) + 36));
            v4 = *((int *)(v6->field_0 + 36));
            if ((char)g_449980 & 8)
                fprintf(*((long long *)&stderr), "Success rates: l=%f, r=%f\n", (unsigned long long)v3, v4);
            if (a0->field_0 == sub_409f9e)
            {
                v0 = !((CmpF(v3, v4) & 69 | (CmpF(v3, v4) & 69) >> 6) & 1);
                if (v0 != 1)
                    v5 = "Operation is OR; right success rate >= left";
            }
            else if (a0->field_0 == sub_408d9c)
            {
                v0 = !((CmpF(v4, v3) & 69 | (CmpF(v4, v3) & 69) >> 6) & 1);
                if (v0 != 1)
                    v5 = "Operation is AND; right success rate <= left";
            }
            else
            {
                v0 = 0;
                v5 = "Not 'AND' or 'OR'";
            }
        }
        else
        {
            v0 = 1;
        }
        if (v0)
        {
            if ((char)g_449980 & 8)
            {
                fprintf(*((long long *)&stderr), "Performing arm swap on:\n");
                sub_40ea9f(*((long long *)&stderr), a0, 0, a0);
            }
            sub_40c856(a0);
            return 1;
        }
    }
    if ((char)g_449980 & 8)
    {
        fprintf(*((long long *)&stderr), "Not an arm swap candidate (%s):\n", v5);
        sub_40ea9f(*((long long *)&stderr), a0, 0, a0);
    }
    return 0;
}



// Function: do_arm_swaps @ 0xcb9f
typedef struct struct_1 {
    char padding_0[280];
    struct struct_0 *field_118;
} struct_1;

typedef struct struct_3 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[252];
    struct struct_1 *field_110;
    struct struct_0 *field_118;
} struct_3;

typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
} struct_0;

unsigned int do_arm_swaps(struct_3 *a0)
{
    char i;  // [bp-0x9]

    if (!a0)
        return 0;
    do
    {
        i = 0;
        if ((char)sub_40c8a7(a0) || (char)do_arm_swaps(a0->field_110) || (char)do_arm_swaps(a0->field_118))
            i = 1;
    } while (i);
    return i;
}



// Function: opt_expr @ 0xcc0f
typedef struct struct_1 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_1;

typedef struct struct_5 {
    struct struct_0 *field_0;
    struct struct_1 *field_8;
} struct_5;

typedef struct struct_0 {
    char padding_0[272];
    struct struct_0 *field_110;
} struct_0;

extern void stderr;
extern unsigned long long g_449980;
extern unsigned short g_4499a8;

unsigned int opt_expr(struct_1 **a0)
{
    char v16;  // al
    char v17;  // al
    char *v18;  // rax
    char v19;  // al
    char v0;  // [bp-0x12a]
    char v1;  // [bp-0x129]
    int i;  // [bp-0x128]
    unsigned int v3;  // [bp-0x124]
    unsigned int v4;  // [bp-0x120]
    unsigned int v5;  // [bp-0x11c]
    struct_1 *iter;  // [bp-0x118], Other Possible Types: unsigned long long
    struct_1 **v7;  // [bp-0x110]
    struct_1 **v8;  // [bp-0x108]
    unsigned long v9;  // [bp-0x100]
    struct_5 v10;  // [bp-0xf8]
    unsigned long long v11;  // [bp-0xf0]
    struct_5 v12;  // [bp-0xe8]
    unsigned long long v13;  // [bp-0xe0]
    char v14;  // [bp-0xd8]

    v10 = (struct_5)0;
    v11 = 0;
    v12 = (struct_5)0;
    v13 = 0;
    v0 = 0;
    if (!a0 || !*(a0))
        return 0;
    for (i = 0; i <= 10; i += 1)
    {
        sub_40c34d(&(&v14)[16 * i]);
    }
    v7 = a0;
    v3 = 3;
    for (iter = *(v7); iter->field_110; iter = iter->field_110)
    {
        v7 = &iter->field_110;
        v3 = iter->field_14;
    }
    if (iter->field_10 != 3)
        sub_40d2d0(iter, v3, v7, v3);
    if ((char)g_449980 & 9)
    {
        fprintf(*((long long *)&stderr), "Normalized Eval Tree:\n");
        sub_40ea9f(*((long long *)&stderr), *(a0), 0, *(a0));
    }
    v7 = a0;
    v4 = 0;
    if (*(v7) && *(v7)->field_10 == 3)
        v4 = *(v7)->field_14;
    while (1)
    {
        iter = *(v7);
        if (!iter)
        {
            v8 = v7;
            sub_40c6e0(&v14, 11, &v12, &v10, v8);
            return v0;
        }
        if (*((int *)(iter + 16)) == 3 && v4 != *((int *)(iter + 20)))
            iter = sub_40d2d0(iter, v4, v7, v4);
        v5 = *((int *)(*((long long *)(iter + 280)) + 16));
        v9 = *((long long *)*((long long *)(iter + 280)));
        if (v5 != 3)
        {
            if (v5 > 3)
                break;
            if (v5 > 1)
            {
                if (v5 != 2)
                    break;
                v16 = opt_expr(*((long long *)(iter + 280)) + 280);
                *((char *)(*((long long *)(iter + 280)) + 24)) = v16;
            }
            else if (v4 != 1 && *((char *)(*((long long *)(iter + 280)) + 24)) != 1)
            {
                if ((char)sub_40c209(*((long long *)(iter + 280))))
                {
                    if ((char)g_449980 & 8)
                    {
                        fprintf(*((long long *)&stderr), "-O%d: promoting cheap predicate ", g_4499a8);
                        sub_40c2dc(*((long long *)&stderr), *((long long *)(iter + 280)));
                        fprintf(*((long long *)&stderr), " into name_list\n");
                    }
                    sub_40c377(&v12, iter, v7);
                    continue;
                }
                else if (v9 == sub_40a2da)
                {
                    sub_40c377(&v10, iter, v7);
                    continue;
                }
                else
                {
                    if (g_4499a8 > 1 && (*((int *)(*((long long *)(iter + 280)) + 32)) == 2 || *((int *)(*((long long *)(iter + 280)) + 32)) == 1) && *((char *)(*((long long *)(iter + 280)) + 26)) != 1 || g_4499a8 > 2)
                        v17 = 1;
                    else
                        v17 = 0;
                    v1 = v17;
                    v1 &= 1;
                    if (v1)
                    {
                        if ((char)g_449980 & 8)
                        {
                            fprintf(*((long long *)&stderr), "-O%d: categorising predicate ", g_4499a8);
                            sub_40c2dc(*((long long *)&stderr), *((long long *)(iter + 280)));
                            v18 = sub_40e95d(*((int *)(*((long long *)(iter + 280)) + 32)));
                            fprintf(*((long long *)&stderr), " by cost (%s)\n", v18);
                        }
                        sub_40c377(&(&v14)[16 * *((int *)(*((long long *)(iter + 280)) + 32))], iter, v7);
                        continue;
                    }
                }
            }
        }
        else
        {
            v19 = opt_expr(iter + 280);
            *((char *)(*((long long *)(iter + 280)) + 24)) = v19;
        }
        if (*((char *)(*((long long *)(iter + 280)) + 24)))
        {
            v8 = v7;
            sub_40c6e0(&v14, 11, &v12, &v10, v8);
            v0 = 1;
        }
        v7 = iter + 272;
    }
    error(1, 0, gettext("oops -- invalid expression type!"));
    return 0;
}



// Function: constrain_rate @ 0xd290
void constrain_rate(void)
{
    unsigned int v1;  // ymm0

    if ((((char)(CmpF(v1, 0x3f800000) & 69) | (char)((CmpF(v1, 0x3f800000) & 69) >> 6)) & 1) != 1)
    {
        return;
    }
    else if ((((char)(CmpF(0, v1) & 69) | (char)((CmpF(0, v1) & 69) >> 6)) & 1) != 1)
    {
        return;
    }
    else
    {
        return;
    }
}



// Function: set_new_parent @ 0xd2d0
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

struct_1 * set_new_parent(struct_0 *index, unsigned int a1, struct_1 **a2)
{
    struct_1 *idx;  // [bp-0x10]

    idx = sub_43205a(312);
    idx->field_10 = 3;
    idx->field_14 = a1;
    idx->field_20 = 0;
    if (a1 == 3)
    {
        *((void* *)&idx->padding_0[0]) = sub_408d9c;
        *((char **)&idx->padding_0[8]) = "-a";
        sub_40d290();
        idx->field_24 = index->field_24;
    }
    else if (a1 <= 3)
    {
        if (a1 == 1)
        {
            *((void* *)&idx->padding_0[0]) = sub_408fd0;
            *((char **)&idx->padding_0[8]) = ",";
            idx->field_24 = 0x3f800000;
        }
        else if (a1 == 2)
        {
            *((void* *)&idx->padding_0[0]) = sub_409f9e;
            *((char **)&idx->padding_0[8]) = "-o";
            sub_40d290();
            idx->field_24 = index->field_24;
        }
    }
    idx->field_118 = index;
    *(a2) = idx;
    return idx;
}



// Function: merge_pred @ 0xd3fd
typedef struct struct_0 {
    char padding_0[272];
    unsigned long long field_110;
} struct_0;

unsigned long long * merge_pred(unsigned long a0, struct_0 *a1, unsigned long long *a2)
{
    a1->field_110 = *(a2);
    *(a2) = a0;
    return a2;
}



// Function: check_sorted @ 0xd431
unsigned int check_sorted(unsigned long a0, unsigned long long a1, unsigned long a2, unsigned long long *a3)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long i;  // [bp-0x18]

    for (i = 1; i < a1; i += 1)
    {
        v0 = a3(a2 * i + a0, a2 * (i - 1) + a0, a2 * i + a0, a2 * (i - 1) + a0);
        if (v0 < 0)
            return 0;
        v0 = a3(a2 * (i - 1) + a0, a2 * i + a0, a2 * (i - 1) + a0, a2 * i + a0);
        if (v0 > 0)
            __assert_fail(); /* do not return */
    }
    return 1;
}



// Function: cost_table_comparison @ 0xd52e
int cost_table_comparison(unsigned long long *a0, unsigned long long *a1)
{
    unsigned long v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]

    v0 = *(a0);
    v1 = *(a1);
    return memcmp(&v0, &v1, 8);
}



// Function: get_pred_cost @ 0xd5a5
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

extern void g_449020;
extern unsigned int g_449a30;

unsigned int get_pred_cost(struct_0 *a0)
{
    unsigned int v0;  // [bp-0x48]
    unsigned int v1;  // [bp-0x44]
    struct_1 *v2;  // [bp-0x40]
    unsigned long v3;  // [bp-0x38]

    v0 = 0;
    v1 = 10;
    v0 = (!a0->field_1a ? (!a0->field_1c ? (!a0->field_1b ? 0 : 2) : 1) : 3);
    if (a0->field_0 == sub_409466 || a0->field_0 == sub_409497)
    {
        v1 = (!a0->field_38 ? 8 : 7);
    }
    else if (a0->field_0 == sub_41376d)
    {
        v1 = a0->field_20;
    }
    else
    {
        if (!g_449a30)
        {
            qsort(&g_449020, 59, 16, sub_40d52e);
            if ((char)sub_40d431(&g_449020, 59, 16, sub_40d52e) != 1)
                error(1, 0, "failed to sort the costlookup array");
            g_449a30 = 1;
        }
        v3 = a0->field_0;
        v2 = bsearch(&v3, &g_449020, 59, 16, sub_40d52e);
        if (v2)
        {
            v1 = v2->field_8;
        }
        else
        {
            error(0, 0, gettext("warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug"));
            v1 = 10;
        }
    }
    return (v1 <= v0 ? v0 : v1);
}



// Function: estimate_costs @ 0xd7ac
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

int estimate_costs(struct_0 *a0)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax

    if (!a0)
        return v2;
    estimate_costs(*((long long *)&a0[4].padding_24[16]));
    estimate_costs(*((long long *)&a0[4].padding_24[8]));
    v1 = sub_40d5a5(a0);
    a0->field_20 = v1;
    return v1;
}



// Function: get_eval_tree @ 0xd7ff
extern unsigned long long g_449a10;

long long get_eval_tree(void)
{
    return g_449a10;
}



// Function: getrate @ 0xd810
unsigned long long getrate(unsigned long long a0)
{
    unsigned long long v1;  // rax

    if (!a0)
        return v1;
    return a0;
}



// Function: calculate_derived_rates @ 0xd838

void* calculate_derived_rates(void* idx)
{
    int v3;  // xmm0
    unsigned int v0;  // [bp-0x24]
    unsigned int v1;  // [bp-0xc]

    if (!idx)
        __assert_fail(); /* do not return */
    if ((long long)idx[280])
        calculate_derived_rates((long long)idx[280]);
    if ((long long)idx[272])
        calculate_derived_rates((long long)idx[272]);
    if ((int)idx[16] == 5)
    {
        __assert_fail(); /* do not return */
    }
    else if ((int)idx[16] == 4)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        switch ((int)idx[16])
        {
        case 0:
            if ((long long)idx[280])
            {
                __assert_fail(); /* do not return */
            }
            else if ((long long)idx[272])
            {
                __assert_fail(); /* do not return */
            }
            else
            {
                return idx;
            }
        case 1:
            if ((long long)idx[280])
            {
                __assert_fail(); /* do not return */
            }
            else if ((long long)idx[272])
            {
                __assert_fail(); /* do not return */
            }
            else
            {
                return idx;
            }
        case 2:
            if (*((long long *)idx) != sub_409b30)
            {
                __assert_fail(); /* do not return */
            }
            else if ((long long)idx[272])
            {
                __assert_fail(); /* do not return */
            }
            else
            {
                *((unsigned int *)&idx[36]) = SubV(0x3f800000, *((int *)((long long)idx[280] + 36)));
                return idx;
            }
        case 3:
            if (*((long long *)idx) == sub_408d9c)
            {
                sub_40d810((long long)idx[280]);
                v0 = *((unsigned int *)&v3);
                sub_40d810((long long)idx[272]);
                v1 = *((unsigned int *)&MulV(v3, v0));
                break;
            }
            else if (*((long long *)idx) == sub_408fd0)
            {
                v1 = 0x3f800000;
                break;
            }
            else if (*((long long *)idx) == sub_409f9e)
            {
                sub_40d810((long long)idx[280]);
                v0 = *((unsigned int *)&v3);
                sub_40d810((long long)idx[272]);
                v1 = *((unsigned int *)&AddV(v3, v0));
                break;
            }
            else
            {
                __assert_fail(); /* do not return */
            }
            sub_40d290();
            *((unsigned int *)&idx[36]) = v1;
            return idx;
        case 4: case 5:
            *((unsigned int *)&idx[36]) = 0x3f800000;
            return idx;
        default:
            __assert_fail(); /* do not return */
        }
    }
}



// Function: check_normalization @ 0xdc43
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

void* check_normalization(struct_1 *a0, char a1)
{
    void* v1;  // rax

    if (a1 && a0->field_10 != 3)
        __assert_fail(); /* do not return */
    if (a0->field_110)
    {
        if (a0->field_110->field_10 != 3)
            __assert_fail(); /* do not return */
        check_normalization(a0->field_110, 0);
    }
    v1 = a0->field_118;
    if (a0->field_118)
        v1 = check_normalization(a0->field_118, 0);
    return v1;
}



// Function: build_expression_tree @ 0xdd20
typedef struct struct_1 {
    char padding_0[16];
    struct struct_2 *field_10;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[25];
    char field_29;
    char padding_2a[222];
    struct struct_0 *field_108;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
} struct_2;

extern void stderr;
extern unsigned long long g_449980;
extern char g_4499f8;
extern struct_0 *g_449a08;
extern void g_449a10;
extern unsigned long long g_449a18;
extern unsigned long long g_449a20;
extern unsigned long long g_449a28;

long long build_expression_tree(unsigned int a0, long long a1, unsigned int a2)
{
    unsigned int v0;  // [bp-0x5c]
    unsigned int iter;  // [bp-0x58]
    unsigned int v2;  // [bp-0x54]
    struct_0 *ptr;  // [bp-0x50]
    long long v4;  // [bp-0x48]
    long long v5;  // [bp-0x40]
    long long v6;  // [bp-0x38]
    long long v7;  // [bp-0x30]
    struct_1 *v8;  // [bp-0x28]

    v0 = a0;
    g_449a08 = 0;
    g_449a20 = a1 + a2 * 8;
    for (iter = a2; iter < v0 && (char)sub_40ff96(*((long long *)(a1 + iter * 8)), 1, iter * 8) != 1; iter += 1)
    {
        g_449a28 = g_449a28 + 1;
    }
    v4 = sub_41437a("(");
    v5 = sub_41437a(")");
    v6 = sub_41437a("print");
    if (!v4)
        __assert_fail(); /* do not return */
    if (!v5)
        __assert_fail(); /* do not return */
    if (!v6)
        __assert_fail(); /* do not return */
    sub_4168b2(v4, a1, &v0, a1);
    *((char **)(g_449a18 + 8)) = "(";
    *((char *)&g_449a08[5].field_0 + 1) = 1;
    sub_414262(a1, v0, g_449a18, g_449a08);
    sub_40aec7(g_449a18);
    while (iter < v0)
    {
        g_4499f8 = 0;
        if ((char)sub_40ff96(*((long long *)(a1 + iter * 8)), 0, iter * 8) != 1)
        {
            error(0, 0, gettext("paths must precede expression: `%s'"));
            if (!access(*((long long *)(a1 + iter * 8)), 0))
                error(0, 0, gettext("possible unquoted pattern after predicate `%s'?"));
            exit(1); /* do not return */
        }
        v7 = *((long long *)(a1 + iter * 8));
        v8 = sub_41437a(v7);
        if (!v8)
            error(1, 0, gettext("unknown predicate `%s'"));
        if (*((int *)&v8->padding_0[0]) != 4)
            iter += 1;
        v2 = iter;
        if ((char)v8->field_10(v8, a1, &iter, a1, v8->field_10) == 1)
        {
            *((long long *)(g_449a18 + 8)) = v7;
            if (v2 != iter)
                *((long long *)(g_449a18 + 48)) = *((long long *)(v2 * 8 + a1));
            else
                *((unsigned long long *)(g_449a18 + 48)) = 0;
            sub_40aec7(g_449a18);
            sub_40aec7(g_449a08);
        }
        else if (*((long long *)(a1 + iter * 8)))
        {
            if (*((int *)&v8->padding_0[0]) == 4 && v2 == iter)
                error(1, 0, gettext("invalid predicate `%s'"));
            error(1, 0, gettext("invalid argument `%s' to `%s'"));
        }
        else
        {
            error(1, 0, gettext("missing argument to `%s'"));
        }
    }
    sub_414287(a1, v0, g_449a18, g_449a08);
    if (!g_449a08[33].field_0)
    {
        ptr = g_449a08;
        g_449a18 = g_449a08[33].field_0;
        g_449a08 = g_449a18;
        free(ptr);
        sub_416eb3(v6, a1, &v0, a1);
        *((char **)(g_449a18 + 8)) = "-print";
        sub_40aec7(g_449a18);
        sub_40aec7(g_449a08);
    }
    else if ((char)sub_40ff58(g_449a08[33].field_0) != 1)
    {
        ptr = g_449a08;
        g_449a08 = g_449a08[33].field_0;
        sub_40aec7(g_449a08);
        free(ptr);
    }
    else
    {
        sub_414833(v5, a1, &v0, a1);
        *((char **)(g_449a18 + 8)) = ")";
        *((char *)(g_449a18 + 41)) = 1;
        sub_40aec7(g_449a18);
        sub_416eb3(v6, a1, &v0, a1);
        *((char **)(g_449a18 + 8)) = "-print";
        *((char *)(g_449a18 + 41)) = 1;
        sub_40aec7(g_449a18);
        sub_40aec7(g_449a08);
    }
    if ((char)g_449980 & 9)
    {
        fprintf(*((long long *)&stderr), "Predicate List:\n");
        sub_40aa62(*((long long *)&stderr), g_449a08);
    }
    sub_413f89(g_449a08);
    sub_40aec7(g_449a08);
    ptr = g_449a08;
    *((struct_1 **)&g_449a10) = sub_40bc44(&ptr, 0, NULL);
    sub_40d838(*((long long *)&g_449a10));
    if (!ptr)
    {
        if ((char)g_449980 & 9)
        {
            fprintf(*((long long *)&stderr), "Eval Tree:\n");
            sub_40ea9f(*((long long *)&stderr), *((long long *)&g_449a10), 0, *((long long *)&g_449a10));
        }
        sub_40d7ac(*((long long *)&g_449a10));
        sub_40cc0f(&g_449a10);
        sub_40dc43(*((long long *)&g_449a10), 1);
        sub_40cb9f(*((long long *)&g_449a10));
        sub_40dc43(*((long long *)&g_449a10), 1);
        if (!((char)g_449980 & 9))
            return *((long long *)&g_449a10);
        fprintf(*((long long *)&stderr), "Optimized Eval Tree:\n");
        sub_40ea9f(*((long long *)&stderr), *((long long *)&g_449a10), 0, *((long long *)&g_449a10));
        fprintf(*((long long *)&stderr), "Optimized command line:\n");
        sub_40ac38(*((long long *)&stderr), *((long long *)&g_449a10));
        fprintf(*((long long *)&stderr), "\n");
        return *((long long *)&g_449a10);
    }
    else if (ptr->field_0 == sub_408ee1)
    {
        error(1, 0, gettext("you have too many ')'"));
    }
    else if (ptr->field_8)
    {
        error(1, 0, gettext("unexpected extra predicate '%s'"));
    }
    else
    {
        error(1, 0, gettext("unexpected extra predicate"));
    }
}



// Function: init_pred_perf @ 0xe60c
unsigned long long * init_pred_perf(unsigned long long *a0)
{
    unsigned long long *idx;  // [bp-0x10]

    idx = a0 + 36;
    idx[1] = 0;
    *(idx) = idx[1];
    return idx;
}



// Function: get_new_pred_noarg @ 0xe644
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

struct_0 * get_new_pred_noarg(long long a0)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_40e67d(a0);
    if (v0)
        v0->field_30 = 0;
    return v0;
}



// Function: get_new_pred @ 0xe67d
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char padding_18[2];
    char field_1a;
    char field_1b;
    char padding_1c[4];
    unsigned int field_20;
    unsigned int field_24;
    char field_28;
    char padding_29[223];
    unsigned long long field_108;
    char padding_110[32];
    struct struct_1 *field_130;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

extern char g_44994f;
extern unsigned long long g_449a08;
extern struct_0 *g_449a18;

struct_0 * get_new_pred(unsigned int *a0)
{
    unsigned long v1;  // rbx

    if (!*(a0))
    {
        __assert_fail(); /* do not return */
    }
    else if (*(a0) == 2)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        v1 = sub_43205a(312);
        if (!g_449a08)
        {
            g_449a08 = v1;
            g_449a18 = g_449a08;
        }
        else
        {
            g_449a18[33].field_0 = v1;
            g_449a18 = v1;
        }
        g_449a18[38].field_0 = a0;
        *((unsigned int *)&g_449a18[2].field_0) = 0;
        *((unsigned int *)((char *)&g_449a18[2].field_0 + 4)) = 0;
        *((char *)&g_449a18[3].field_0 + 2) = 1;
        *((char *)&g_449a18[3].field_0 + 3) = 1;
        *((unsigned int *)&g_449a18[4].field_0) = 10;
        g_449a18[6].field_0 = "ThisShouldBeSetToSomethingElse";
        *((char *)&g_449a18[5].field_0) = g_44994f;
        *((unsigned int *)((char *)&g_449a18[4].field_0 + 4)) = 0x3f800000;
        sub_40e60c(g_449a18);
        return g_449a18;
    }
}



// Function: get_new_pred_chk_op @ 0xe7dd
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern struct_0 *g_449a18;
extern long long g_449a40;

struct_0 * get_new_pred_chk_op(unsigned int *a0, unsigned long a1)
{
    unsigned int v2;  // eax
    struct_0 *idx;  // [bp-0x10]

    if (!g_449a40)
        g_449a40 = sub_41437a("and");
    if (!g_449a40)
        __assert_fail(); /* do not return */
    if (g_449a18)
    {
        v2 = g_449a18[2].field_0;
        switch (v2)
        {
        case 1: case 5:
            idx = sub_40e644(g_449a40);
            *((void* *)&idx->padding_0[0]) = sub_408d9c;
            *((char **)&idx->padding_0[8]) = "-a";
            *((unsigned int *)&idx->padding_0[16]) = 3;
            *((unsigned int *)&idx->padding_0[20]) = 3;
            idx->padding_0[26] = 0;
            idx->padding_0[27] = 0;
            idx->padding_0[28] = 0;
            idx->field_30 = 0;
            *((unsigned long *)&idx[1].padding_0[0]) = 0;
            idx->padding_0[24] = 0;
            idx->padding_0[25] = 0;
            break;
        case 0:
            error(1, 0, gettext("oops -- invalid default insertion of and!"));
        default:
            break;
        }
    }
    idx = sub_40e67d(a0);
    idx->field_30 = a1;
    *((unsigned int **)&idx[5].padding_0[24]) = a0;
    return idx;
}



// Function: cost_name @ 0xe95d
extern unsigned int g_4493e0[4];
extern unsigned long long g_4493e8[4];

unsigned long long * cost_name(unsigned int *a0)
{
    unsigned int i;  // [bp-0x10]
    unsigned int v1;  // [bp-0xc]

    v1 = 11;
    for (i = 0; i < v1; i += 1)
    {
        if (a0 == g_4493e0[4 * i])
            return g_4493e8[2 * i];
    }
    return "unknown";
}



// Function: type_name @ 0xe9bd
extern unsigned short g_449520[4];
extern unsigned long long g_449528[4];

unsigned long long * type_name(unsigned short *a0)
{
    int i;  // [bp-0xc]

    for (i = 0; g_449520[8 * i] != 0xffff && a0 != g_449520[8 * i]; i += 1);
    return g_449528[2 * i];
}



// Function: prec_name @ 0xea2e
extern unsigned short g_4494a0[4];
extern unsigned long long g_4494a8[4];

unsigned long long * prec_name(unsigned short *a0)
{
    int i;  // [bp-0xc]

    for (i = 0; g_4494a0[8 * i] != 0xffff && a0 != g_4494a0[8 * i]; i += 1);
    return g_4494a8[2 * i];
}



// Function: print_tree @ 0xea9f
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    unsigned int field_14;
    char field_18;
    char padding_19[1];
    char field_1a;
    char field_1b;
    char field_1c;
    char padding_1d[3];
    unsigned int field_20;
    char padding_24[236];
    void* field_110;
    void* field_118;
} struct_0;

extern char g_43955c;

int print_tree(FILE *ptr, struct_0 *a1, unsigned int a2)
{
    void* v3;  // rax
    void* v5;  // rax
    int v6;  // rax
    unsigned int iter;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]

    if (!a1)
        return v6;
    for (iter = 0; iter < a2; iter += 1)
    {
        fprintf(ptr, "    ");
    }
    fprintf(ptr, "pred=[");
    sub_40c2dc(ptr, a1);
    v3 = sub_40ea2e((short)a1->field_14);
    fprintf(ptr, "] type=%s prec=%s", sub_40e9bd((short)a1->field_10), v3);
    v5 = sub_40e95d(a1->field_20);
    fprintf(ptr, " cost=%s est_success_rate=%#.4g %sside effects ", v5, (!a1->field_18 ? "no " : &g_43955c));
    if (a1->field_1a || a1->field_1b || a1->field_1c)
    {
        v1 = 0;
        fprintf(ptr, "Needs ");
        if (a1->field_1a)
        {
            fprintf(ptr, "stat");
            v1 = 1;
        }
        if (a1->field_1c)
        {
            fprintf(ptr, "%sinode", (!v1 ? &g_43955c : ","));
            v1 = 1;
        }
        if (a1->field_1b)
            fprintf(ptr, "%stype", (!v1 ? &g_43955c : ","));
    }
    fprintf(ptr, "\n");
    for (iter = 0; iter < a2; iter += 1)
    {
        fprintf(ptr, "    ");
    }
    if (!a1->field_110 && !a1->field_118)
        return fprintf(ptr, "no children.\n");
    if (a1->field_110)
    {
        fprintf(ptr, "left:\n");
        print_tree(ptr, a1->field_110, a2 + 1);
    }
    else
    {
        fprintf(ptr, "no left.\n");
    }
    for (iter = 0; iter < a2; iter += 1)
    {
        fprintf(ptr, "    ");
    }
    if (!a1->field_118)
        return fprintf(ptr, "no right.\n");
    fprintf(ptr, "right:\n");
    return print_tree(ptr, a1->field_118, a2 + 1);
}



// Function: insert_primary_withpred @ 0xee5a
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

struct_0 * insert_primary_withpred(struct_0 *a0, unsigned long a1, unsigned int a2)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_40e7dd(a0, a2);
    *((unsigned long *)&idx->padding_0[0]) = a1;
    *((long long *)&idx->padding_0[8]) = *((long long *)&a0->padding_0[8]);
    *((unsigned long *)&idx[1].padding_0[0]) = 0;
    *((unsigned int *)&idx->padding_0[16]) = 1;
    *((unsigned int *)&idx->padding_0[20]) = 0;
    return idx;
}



// Function: insert_primary @ 0xeecc
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

struct_0 * insert_primary(struct_0 *a0, unsigned int a1)
{
    if (*((long long *)&a0->padding_0[24]))
        return sub_40ee5a(a0, *((long long *)&a0->padding_0[24]), a1);
    __assert_fail(); /* do not return */
}



// Function: insert_primary_noarg @ 0xef32
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

struct_0 * insert_primary_noarg(struct_0 *a0)
{
    return sub_40eecc(a0, 0);
}



// Function: show_valid_debug_options @ 0xef55
typedef struct FILE {
} FILE;

extern char g_4398cd;
extern unsigned long long g_4495a0[4];
extern FILE *stdout;

int show_valid_debug_options(unsigned int a0)
{
    char *v2;  // rax
    int v3;  // eax
    unsigned long long iter;  // [bp-0x20]

    v2 = gettext("Valid arguments for -D:\n");
    v3 = fputs(v2, stdout);
    if (a0)
    {
        for (iter = 0; iter <= 8; iter += 1)
        {
            v3 = fprintf(stdout, "%-10s %s\n", g_4495a0[3 * iter]);
        }
        return v3;
    }
    else
    {
        for (iter = 0; iter <= 8; iter += 1)
        {
            v3 = fprintf(stdout, "%s%s", (!iter ? &g_4398cd : ", "), g_4495a0[3 * iter]);
        }
        return v3;
    }
}



// Function: usage @ 0xf071
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern void stdout;
extern long long g_44a230;

void usage(int a0)
{
    unsigned long long v2;  // rbx
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
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    if (!a0)
    {
        v4 = gettext("Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n");
        fprintf(*((long long *)&stdout), v4);
        v5 = gettext("\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n");
        fputs(v5, *((long long *)&stdout));
        v6 = gettext("\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n");
        fputs(v6, *((long long *)&stdout));
        v7 = gettext("\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n");
        fputs(v7, *((long long *)&stdout));
        v8 = gettext("\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n");
        fputs(v8, *((long long *)&stdout));
        v9 = gettext("\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n");
        fputs(v9, *((long long *)&stdout));
        v10 = gettext("\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n");
        fputs(v10, *((long long *)&stdout));
        v11 = gettext("\nOther common options:\n");
        fputs(v11, *((long long *)&stdout));
        v12 = gettext("      --help                   display this help and exit\n");
        fputs(v12, *((long long *)&stdout));
        v13 = gettext("      --version                output version information and exit\n\n");
        fputs(v13, *((long long *)&stdout));
        sub_40ef55(0);
        v14 = gettext("\nUse '-D help' for a description of the options, or see find(1)\n\n");
        fputs(v14, *((long long *)&stdout));
        sub_41d008(*((long long *)&stdout), g_44a230, g_44a230);
        exit(a0); /* do not return */
    }
    v3 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v3);
    exit(a0); /* do not return */
}



// Function: set_stat_placeholders @ 0xf26e
void set_stat_placeholders(unsigned long a0)
{
    return;
}



// Function: get_statinfo @ 0xf27d
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern char g_44994e;
extern struct_1 *g_449990;
extern unsigned int g_4499ac;
extern char g_4499c4;
extern char g_4499c5;
extern unsigned int g_4499c8;
extern unsigned int g_4499e4;

unsigned int get_statinfo(long long a0, long long a1, struct_0 *a2)
{
    unsigned int *err;  // rax

    if (g_4499c4 != 1)
    {
        sub_40f26e(a2);
        if (g_449990(a1, a2, a2, g_449990))
        {
            if (g_44994e != 1 || *(__errno_location()) != 2)
            {
                err = __errno_location();
                sub_410a13(*(err), a0, a0);
            }
            return 0xffffffff;
        }
        else if (!a2->field_18)
        {
            sub_42f861(0, g_4499ac, a1);
            error(0, 0, gettext("WARNING: file %s appears to have mode 0000"));
            g_4499e4 = 1;
        }
    }
    g_4499c4 = 1;
    g_4499c5 = 1;
    g_4499c8 = a2->field_18;
    return 0;
}



// Function: get_info @ 0xf388
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4499c4;
extern char g_4499c5;
extern long long g_4499d0;

unsigned int get_info(long long a0, struct_0 *a1, char *a2)
{
    char v0;  // [bp-0x9]

    v0 = 0;
    if (a2[26] && g_4499c4 != 1)
    {
        v0 = 1;
    }
    else if (a2[27] && g_4499c5 != 1)
    {
        v0 = 1;
    }
    else if (a2[28])
    {
        if (!*((long long *)&a1->padding_0[8]))
        {
            v0 = 1;
        }
        else if (g_4499c5 != 1 || ((unsigned short)a1->field_18 & 0xf000) == 0x4000)
        {
            v0 = 1;
        }
    }
    if (v0 && sub_40f27d(a0, g_4499d0, a1))
        return 0xffffffff;
    return 0;
}



// Function: check_nofollow @ 0xf45a
extern char atof;

unsigned int check_nofollow(void)
{
    unsigned long long v4;  // rax
    unsigned long v6;  // cc_ndep
    unsigned long long v7;  // 4115
    unsigned long long v8;  // rax
    unsigned int v9;  // ymm0
    unsigned long v10;  // cc_ndep
    unsigned long long v11;  // 4115
    unsigned long long *v0;  // [bp-0x1a0]
    char v1[6][1024];  // [bp-0x198]
    char v2;  // [bp-0x116]

    if (!uname(v1))
    {
        v0 = &atof;
        v0(&v2);
        v4 = strcmp("Linux", v1);
        if (!(unsigned int)v4)
        {
            v7 = _ccall(3, 0, (unsigned long long)(CmpF((unsigned long long)v9, 1074580685) & 69), 0, v6);
            return _INSERT(v4, 0, v7 & 1);
        }
        v8 = strcmp("FreeBSD", v1);
        if (!(unsigned int)v8)
        {
            v11 = _ccall(3, 0, (unsigned long long)(CmpF((unsigned long long)v9, 1078355558) & 69), 0, v10);
            return _INSERT(v8, 0, v11 & 1);
        }
    }
    return 1;
}



// Function: exec_cb @ 0xf540
unsigned int exec_cb(long long a0)
{
    sub_419f5a(a0 + 8, a0 + 96, a0 + 96);
    return 0;
}



// Function: do_exec @ 0xf57a
typedef struct struct_0 {
    char padding_0[192];
    struct struct_0 *field_c0;
} struct_0;

extern char g_449a00;

struct_0 * do_exec(struct_0 *a0)
{
    sub_41a9dd(a0->field_c0, sub_40f540, a0, sub_40f540);
    if (a0->field_c0 == *((long long *)&g_449a00))
        return *((long long *)&g_449a00);
    sub_42fe52(a0->field_c0);
    free(a0->field_c0);
    a0->field_c0 = NULL;
    return a0;
}



// Function: do_complete_pending_execdirs @ 0xf5fa
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[48];
    char field_38;
    char padding_39[151];
    unsigned int field_d0;
    char padding_d4[60];
    void* field_110;
    void* field_118;
} struct_0;

extern char g_4499e8;

long long do_complete_pending_execdirs(struct_0 *a0)
{
    unsigned long v2;  // rax
    struct_0 *v0;  // [bp-0x10]

    if (!a0)
    {
        return v2;
    }
    else if (!g_4499e8)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        do_complete_pending_execdirs(a0->field_110);
        if (a0->field_0 != sub_409497 && a0->field_0 != sub_409f27 || !a0->field_38)
            return do_complete_pending_execdirs(a0->field_118);
        v0 = &a0->field_38;
        if (*((int *)&v0->padding_39[95]))
            sub_40f57a(v0);
        return do_complete_pending_execdirs(a0->field_118);
    }
}



// Function: complete_pending_execdirs @ 0xf6cb
extern char g_4499e8;

long long complete_pending_execdirs(void)
{
    long long v1;  // rax
    long long v2;  // rax

    v1 = g_4499e8;
    if (!(char)v1)
        return v1;
    v2 = sub_40f5fa(sub_40d7ff());
    g_4499e8 = 0;
    return v2;
}



// Function: complete_pending_execs @ 0xf6f5
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[48];
    char field_38;
    char padding_39[151];
    unsigned int field_d0;
    char padding_d4[60];
    void* field_110;
    void* field_118;
} struct_0;

long long complete_pending_execs(struct_0 *a0)
{
    unsigned long v2;  // rax
    struct_0 *v0;  // [bp-0x10]

    if (!a0)
        return v2;
    complete_pending_execs(a0->field_110);
    if (a0->field_0 != sub_409466 || !a0->field_38)
        return complete_pending_execs(a0->field_118);
    v0 = &a0->field_38;
    if (*((int *)&v0->padding_39[95]))
        sub_419f5a(v0->padding_8, &v0->padding_39[39], &v0->padding_39[39]);
    return complete_pending_execs(a0->field_118);
}



// Function: record_initial_cwd @ 0xf78b
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_449a00;

unsigned long long record_initial_cwd(void)
{
    unsigned long long v1;  // rax
    char *v2;  // rax
    int *err;  // rax

    g_449a00 = sub_431bc2(16);
    v1 = sub_42fd99(g_449a00);
    if (!(unsigned int)v1)
        return v1;
    v2 = gettext("Failed to save initial working directory%s%s");
    err = __errno_location();
    error(1, *(err), v2);
}



// Function: cleanup_initial_cwd @ 0xf861
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_449a00;

void cleanup_initial_cwd(void)
{
    char *v1;  // rax
    int *err;  // rax

    if (sub_42fe17(g_449a00))
    {
        v1 = gettext("Failed to restore initial working directory%s%s");
        err = __errno_location();
        error(0, *(err), v1);
        _exit(1); /* do not return */
    }
    sub_42fe52(g_449a00);
    free(g_449a00);
    g_449a00 = 0;
    return;
}



// Function: traverse_tree @ 0xf959
void* traverse_tree(void* *a0, unsigned long long *a1)
{
    if (a0[0x22])
        traverse_tree(a0[0x22], a1);
    a1(a0);
    if (!a0[35])
        return a0[35];
    return traverse_tree(a0[35], a1);
}



// Function: undangle_file_pointers @ 0xf9d1
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[56];
    unsigned long long field_40;
} struct_0;

struct_0 * undangle_file_pointers(struct_0 *a0)
{
    if (a0->field_0 != sub_40954f && a0->field_0 != sub_41376d && a0->field_0 != sub_4094e6 && a0->field_0 != sub_40959f)
        return a0->field_0;
    a0->field_40 = 0;
    return a0;
}



// Function: cleanup @ 0xfa38
extern long long stdout;
extern long long g_4499f0;

unsigned long long cleanup(void)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned long long v8;  // rax
    unsigned long v0;  // [bp-0x10]

    v0 = sub_40d7ff();
    if (v0)
    {
        sub_40f959(v0, sub_40f6f5);
        sub_40f6cb();
    }
    sub_410cac(g_4499f0);
    if (v0)
        sub_40f959(v0, sub_40f9d1);
    sub_40f861();
    if ((char)sub_41b5f4(v2, v3, v4, v5, v6, v7))
    {
        sub_41b61e();
        sub_41b5c4(v2, v3, v4, v5, v6, v7);
    }
    v8 = sub_41e1a8(stdout);
    if ((unsigned int)v8 != 0xffffffff)
        return v8;
    return sub_410a77(*(__errno_location()), "standard output", "standard output");
}



// Function: fallback_stat @ 0xfae2
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned long long g_449980;
extern unsigned int g_4499d8;

int fallback_stat(unsigned long a0, long long a1, unsigned int a2)
{
    unsigned int v1;  // eax

    v1 = *(__errno_location());
    if (*(__errno_location()) != 2 && *(__errno_location()) != 20)
        return a2;
    if ((char)g_449980 & 2)
        fprintf(stderr, "fallback_stat(): stat(%s) failed; falling back on lstat()\n", a0);
    return fstatat(g_4499d8, a0, a1, 0x100);
}



// Function: optionh_stat @ 0xfb5c
extern unsigned int g_4499c0;
extern void g_4499d8;

unsigned int optionh_stat(long long a0, long long a1)
{
    unsigned int v0;  // [bp-0xc]

    if (*((int *)&g_4499d8) != 0xffffff9c && *((int *)&g_4499d8) < 0)
        __assert_fail(); /* do not return */
    sub_40f26e(a1);
    if (g_4499c0)
        return fstatat(*((int *)&g_4499d8), a0, a1, 0x100);
    v0 = fstatat(*((int *)&g_4499d8), a0, a1, 0);
    return (!v0 ? 0 : (int)sub_40fae2(a0, a1, v0));
}



// Function: optionl_stat @ 0xfc21
extern void g_4499d8;

unsigned int optionl_stat(long long a0, long long a1)
{
    unsigned int v0;  // [bp-0xc]

    if (*((int *)&g_4499d8) != 0xffffff9c && *((int *)&g_4499d8) < 0)
        __assert_fail(); /* do not return */
    sub_40f26e(a1);
    v0 = fstatat(*((int *)&g_4499d8), a0, a1, 0);
    return (!v0 ? 0 : (int)sub_40fae2(a0, a1, v0));
}



// Function: optionp_stat @ 0xfcc0
extern void g_4499d8;

int optionp_stat(long long a0, long long a1)
{
    if (*((int *)&g_4499d8) < 0 && *((int *)&g_4499d8) != 0xffffff9c)
        __assert_fail(); /* do not return */
    sub_40f26e(a1);
    return fstatat(*((int *)&g_4499d8), a0, a1, 0x100);
}



// Function: debug_stat @ 0xfd39
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned int g_449988;
extern unsigned long long g_449a48;

int debug_stat(unsigned long a0, long long a1)
{
    g_449a48 = g_449a48 + 1;
    fprintf(stderr, "debug_stat (%s)\n", a0);
    if (g_449988 == 2)
        return sub_40fb5c(a0, a1);
    if (g_449988 <= 2)
    {
        if (!g_449988)
        {
            return sub_40fcc0(a0, a1);
        }
        else if (g_449988 == 1)
        {
            return sub_40fc21(a0, a1);
        }
    }
    __assert_fail(); /* do not return */
}



// Function: following_links @ 0xfe03
extern unsigned int g_449988;
extern unsigned int g_4499c0;

unsigned long long following_links(void)
{
    return (g_449988 == 1 ? 1 : (g_449988 == 2 ? _INSERT(g_4499c0, 0, !g_4499c0) : 0));
}



// Function: digest_mode @ 0xfe38
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4499c4;
extern char g_4499c5;
extern unsigned int g_4499c8;

unsigned int digest_mode(unsigned int *a0, long long a1, long long a2, struct_0 *idx, char a4)
{
    if (*(a0))
    {
        if (((unsigned short)*(a0) & 0xf000) == 0xa000 && (char)sub_40fe03())
        {
            if (sub_40f27d(a1, a2, idx))
                return 0;
            g_4499c8 = idx->field_18;
            *(a0) = g_4499c8;
            g_4499c5 = 1;
        }
        else
        {
            g_4499c5 = 1;
            g_4499c8 = *(a0);
            idx->field_18 = g_4499c8;
        }
    }
    else
    {
        if (a4)
        {
            g_4499c4 = 0;
            g_4499c5 = 0;
            g_4499c8 = 0;
        }
        else
        {
            if (sub_40f27d(a1, a2, idx))
                return 0;
            *(a0) = idx->field_18;
            g_4499c8 = *(a0);
            g_4499c5 = 1;
        }
    }
    return 1;
}



// Function: default_prints @ 0xff58
typedef struct struct_0 {
    char padding_0[25];
    char field_19;
    char padding_1a[238];
    struct struct_0 *field_108;
} struct_0;

unsigned int default_prints(struct_0 *a0)
{
    struct_0 *iter;  // [bp-0x10]

    for (iter = a0; iter; iter = iter->field_108)
    {
        if (iter->field_19)
            return 0;
    }
    return 1;
}



// Function: looks_like_expression @ 0xff96

unsigned long long looks_like_expression(char *a0, char a1)
{
    switch (*(a0))
    {
    case 33: case 40:
        return (!a0[1] ? 1 : 0);
    case 41: case 44:
        return (!a0[1] ? !a1 : 0);
    case 45:
        return (!a0[1] ? 0 : 1);
    default:
        return 0;
    }
}



// Function: process_debug_options @ 0x1004b
extern unsigned long long g_4495a0[4];
extern unsigned long long g_449980;
extern unsigned int g_4499ac;

void process_debug_options(unsigned long a0)
{
    char v0;  // [bp-0x41]
    char *v1;  // [bp-0x40]
    char *tok;  // [bp-0x38]
    unsigned long long j;  // [bp-0x30]
    char v4[2];  // [bp-0x22]

    v1 = NULL;
    v4 = 44;
    v0 = 1;
    for (tok = strtok_r(a0, v4, &v1); tok; tok = strtok_r(NULL, v4, &v1))
    {
        v0 = 0;
        for (j = 0; j <= 8; j += 1)
        {
            if (!strcmp(g_4495a0[3 * j], tok))
            {
                g_449980 = *((int *)(24 * j + 4494760)) | g_449980;
                break;
            }
        }
        if (j > 8)
        {
            sub_42f861(0, g_4499ac, a0);
            error(0, 0, gettext("Ignoring unrecognised debug flag %s"));
        }
    }
    if (v0)
    {
        error(0, 0, gettext("Empty argument to the -D option."));
        sub_40f071(1); /* do not return */
    }
    else if ((char)g_449980 & 16)
    {
        sub_40ef55(1);
        exit(0); /* do not return */
    }
    else
    {
        return;
    }
}



// Function: process_optimisation_option @ 0x101fd
extern unsigned short g_4499a8;

void process_optimisation_option(char *a0)
{
    unsigned long long *v5;  // rax
    char *v9;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x34]
    char v1;  // [bp-0x30]
    unsigned long long num;  // [bp-0x28]

    if (!*(a0))
        error(1, 0, gettext("The -O option must be immediately followed by a decimal integer"));
    v5 = __ctype_b_loc();
    if (!(*((short *)(*(a0) * 2 + *(v5))) & 0x800))
        error(1, 0, gettext("Please specify a decimal number immediately after -O"));
    v0 = *(__errno_location());
    *(__errno_location()) = 0;
    num = strtoul(a0, &v1, 10, &v1);
    if (!num && a0 == v1)
        error(1, 0, gettext("Please specify a decimal number immediately after -O"));
    if (*((char *)v1))
        error(1, 0, gettext("Invalid optimisation level %s"));
    if (num == 0xffffffffffffffff && *(__errno_location()))
    {
        v9 = gettext("Invalid optimisation level %s");
        err = __errno_location();
        error(1, *(err), v9);
    }
    if (num <= 0xffff)
    {
        g_4499a8 = num;
        *(__errno_location()) = v0;
        return;
    }
    error(1, 0, gettext("Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate."));
}



// Function: process_leading_options @ 0x103e8
int process_leading_options(int a0, unsigned long a1)
{
    int iter;  // [bp-0x10]
    int v1;  // [bp-0xc]

    iter = 1;
    while (1)
    {
        v1 = iter;
        if (v1 >= a0)
            return v1;
        if (!strcmp("-H", *((long long *)(a1 + iter * 8))))
        {
            sub_4141b0(2);
        }
        else if (!strcmp("-L", *((long long *)(a1 + iter * 8))))
        {
            sub_4141b0(1);
        }
        else if (!strcmp("-P", *((long long *)(a1 + iter * 8))))
        {
            sub_4141b0(0);
        }
        else
        {
            if (!strcmp("--", *((long long *)(a1 + iter * 8))))
            {
                v1 = iter + 1;
                return v1;
            }
            if (!strcmp("-D", *((long long *)(a1 + iter * 8))))
            {
                if (a0 <= iter + 1)
                {
                    error(0, 0, gettext("Missing argument after the -D option."));
                    sub_40f071(1); /* do not return */
                }
                sub_41004b(*((long long *)(a1 + (iter + 1) * 8)));
                iter += 1;
            }
            else if (!strncmp("-O", *((long long *)(a1 + iter * 8)), 2))
            {
                sub_4101fd(*((long long *)(a1 + iter * 8)) + 2);
            }
            else
            {
                v1 = iter;
                return v1;
            }
        }
        iter += 1;
    }
}



// Function: now @ 0x10601
unsigned long long now(void)
{
    unsigned long long t;  // [bp-0x40]
    unsigned long long v1;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    int v3[3];  // [bp-0x28]
    unsigned long v4;  // [bp-0x20]

    if (!gettimeofday(v3, NULL))
    {
        v1 = (unsigned long long)v3;
        v2 = v4 * 1000;
        return v1;
    }
    t = time(NULL);
    if (t != 0xffffffffffffffff)
    {
        v1 = t;
        v2 = 0;
        return v1;
    }
    __assert_fail(); /* do not return */
}



// Function: set_option_defaults @ 0x106bd
typedef struct struct_0 {
    char field_0;
    char field_1;
    char padding_2[2];
    unsigned int field_4;
    unsigned int field_8;
    char field_c;
    char field_d;
    char field_e;
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

struct_0 * set_option_defaults(struct_0 *idx)
{
    unsigned long v1;  // rdx

    if (getenv("POSIXLY_CORRECT"))
        idx->field_11 = 1;
    else
        idx->field_11 = 0;
    idx->field_58 = sub_40f45a();
    idx->field_5c = 0;
    if (isatty(0))
    {
        idx->field_10 = 1;
        idx->field_f = 0;
    }
    else
    {
        idx->field_10 = 0;
        idx->field_f = 0;
    }
    if (idx->field_11)
        idx->field_10 = 0;
    idx->field_0 = 1;
    idx->field_1 = 0;
    idx->field_8 = 0xffffffff;
    idx->field_4 = idx->field_8;
    idx->field_18 = sub_410601();
    idx->field_20 = v1;
    idx->field_28 = idx->field_18 - 86400;
    idx->field_30 = idx->field_20;
    idx->field_38 = 0;
    idx->field_d = 0;
    idx->field_e = 0;
    if (idx->field_11)
        idx->field_3c = 0x200;
    else
        idx->field_3c = 0x400;
    idx->field_40 = 0;
    idx->field_68 = 2;
    if (!getenv("FIND_BLOCK_SIZE"))
    {
        idx->field_c = 0;
        sub_4141b0(0);
        idx->field_6c = 8;
        idx->field_70 = 0;
        idx->field_78 = 0;
        return idx;
    }
    error(1, 0, gettext("The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable"));
}



// Function: apply_predicate @ 0x10873
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[18];
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

unsigned int apply_predicate(long long a0, struct_0 *a1, struct_0 *idx)
{
    idx->field_120 = idx->field_120 + 1;
    if ((idx->field_1a || idx->field_1b || idx->field_1c) && sub_40f388(a0, a1, idx))
        return 0;
    if (!(char)idx->field_0(a0, a1, idx, a1, idx->field_0))
        return 0;
    idx->field_128 = idx->field_128 + 1;
    return 1;
}



// Function: is_exec_in_local_dir @ 0x10933
unsigned int is_exec_in_local_dir(unsigned long a0)
{
    unsigned int v1;  // eax

    if (a0 == sub_409497 || a0 == sub_409f27)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: safely_quote_err_filename @ 0x1096a
extern unsigned int g_4499ac;

int safely_quote_err_filename(unsigned int a0, long long a1)
{
    return sub_42f861(a0, g_4499ac, a1, g_4499ac);
}



// Function: report_file_err @ 0x10995
extern unsigned int g_4499e4;
extern char g_4499f8;

long long report_file_err(int a0, int a1, char a2, long long a3)
{
    unsigned long v1;  // rax

    if (a2 != 1 || (v1 = (unsigned long)(unsigned long long)((unsigned int)g_4499f8 ^ 1), (char)v1))
    {
        sub_41096a(0, a3);
        v1 = (unsigned long long)error(a0, a1, "%s");
        g_4499e4 = 1;
    }
    if (a2)
        g_4499f8 = 1;
    return v1;
}



// Function: nonfatal_target_file_error @ 0x10a13
long long nonfatal_target_file_error(int a0, long long a1)
{
    return sub_410995(0, a0, 1, a1);
}



// Function: fatal_target_file_error @ 0x10a44
void fatal_target_file_error(int a0, long long a1)
{
    sub_410995(1, a0, 1, a1);
    abort(); /* do not return */
}



// Function: nonfatal_nontarget_file_error @ 0x10a77
long long nonfatal_nontarget_file_error(int a0, long long a1)
{
    return sub_410995(0, a0, 0, a1);
}



// Function: fatal_nontarget_file_error @ 0x10aa8
extern char g_4499f8;

void fatal_nontarget_file_error(int a0, long long a1)
{
    g_4499f8 = 0;
    sub_410995(1, a0, 0, a1);
    abort(); /* do not return */
}



// Function: entry_comparator @ 0x10ae2
unsigned int entry_comparator(unsigned long long *a0, unsigned long a1)
{
    unsigned int v1;  // eax

    if (a0[1] == *((long long *)(a1 + 8)) && *(a0) == *((long long *)a1))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: entry_free @ 0x10b3b
void entry_free(void* ptr)
{
    int *err;  // rax

    if (!(long long)ptr[24])
    {
        free((long long)ptr[16]);
        free(ptr);
        return;
    }
    else if (sub_41dad5((long long)ptr[24]))
    {
        err = __errno_location();
        sub_410aa8(*(err), (long long)ptr[16]); /* do not return */
    }
    else
    {
        free((long long)ptr[16]);
        free(ptr);
        return;
    }
}



// Function: entry_hashfunc @ 0x10bb1
long long entry_hashfunc(unsigned long long *a0, unsigned long long a1)
{
    return (a0[1] ^ *(a0)) % a1;
}



// Function: sharefile_init @ 0x10be9
typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
} struct_0;

struct_0 * sharefile_init(char *a0)
{
    struct_0 *ptr;  // [bp-0x10]

    ptr = malloc(16);
    if (ptr)
    {
        ptr->field_0 = strdup(a0);
        if (ptr->field_0)
        {
            ptr->field_8 = sub_421bb1(11, 0, sub_410bb1, sub_410ae2, sub_410b3b);
            if (ptr->field_8)
                return ptr;
            free(ptr->field_0);
            free(ptr);
        }
        else
        {
            free(ptr);
        }
    }
    return NULL;
}



// Function: sharefile_destroy @ 0x10cac
typedef struct struct_0 {
    void* field_0;
    long long field_8;
} struct_0;

long long sharefile_destroy(struct_0 *a0)
{
    free(a0->field_0);
    return sub_421e31(a0->field_8);
}



// Function: sharefile_fopen @ 0x10ce6
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char *field_10;
    struct FILE *field_18;
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

FILE * sharefile_fopen(long long *a0, unsigned long a1)
{
    unsigned int v0;  // [bp-0xc8]
    unsigned int v1;  // [bp-0xc4]
    struct_0 *ptr;  // [bp-0xb8]
    struct_1 *v3;  // [bp-0xb0]
    stat v4;  // [bp-0xa8]

    ptr = malloc(32);
    if (!ptr)
        return NULL;
    ptr->field_10 = strdup(a1);
    if (!ptr->field_10)
    {
        free(ptr);
        return NULL;
    }
    ptr->field_18 = sub_41e49e(a1, *(a0), *(a0));
    if (!ptr->field_18)
    {
        sub_410b3b(ptr);
        return NULL;
    }
    v0 = fileno(ptr->field_18);
    if (v0 < 0)
        __assert_fail(); /* do not return */
    sub_41d5ba(v0, 1);
    if (fstat(v0, &v4) < 0)
    {
        sub_410b3b(ptr);
        return NULL;
    }
    ptr->field_0 = *((unsigned long long *)&v4);
    ptr->field_8 = v4.st_ino;
    v3 = sub_4214fc(a0[1], ptr, ptr);
    if (v3)
    {
        sub_410b3b(ptr);
        return v3->field_18;
    }
    else if (sub_4228f2(a0[1], ptr, ptr))
    {
        return ptr->field_18;
    }
    else
    {
        v1 = *(__errno_location());
        sub_410b3b(ptr);
        *(__errno_location()) = v1;
        return NULL;
    }
}



// Function: make_segment @ 0x10f4b
typedef struct struct_0 {
    unsigned int field_0;
    char field_4;
    char field_5;
    char padding_6[2];
    unsigned long long field_8;
    int field_10;
    char padding_14[4];
    unsigned long long field_18;
} struct_0;


long long make_segment(struct_0 **a0, char *a1, int a2, unsigned int a3, char a4, char a5, char *idx)
{
    struct_0 *index;  // rbx
    char *v3;  // rax
    char *v12;  // rax
    char *v13;  // rax
    char *v14;  // rax
    char *v15;  // rax
    char *v4;  // rax
    char *v5;  // rax
    char *v6;  // rax
    char *v7;  // rax
    char *v8;  // rax
    char *v9;  // rax
    char *v10;  // rax
    char *v11;  // rax
    unsigned int v0;  // [bp-0x24]
    char *iter;  // [bp-0x20], Other Possible Types: unsigned long

    v0 = 0;
    switch (a4)
    {
    case 123:
        __assert_fail(); /* do not return */
    case 91:
        __assert_fail(); /* do not return */
    case 40:
        __assert_fail(); /* do not return */
    default:
        *(a0) = sub_431bc2(32);
        *(a0)->field_0 = a3;
        *(a0)->field_4 = a4;
        *(a0)->field_5 = a5;
        *(a0)->field_18 = 0;
        *(a0)->field_10 = a2;
        index = *(a0);
        index->field_8 = sub_431bc2(a2 + 2);
        iter = index->field_8;
        strncpy(iter, a1, a2);
        iter += a2;
        if (a3 && a3 != 1)
        {
            if (a3 != 2)
                __assert_fail(); /* do not return */
            switch (a4)
            {
            case 37:
                v3 = iter;
                iter = v3 + 1;
                *(v3) = 37;
                break;
            case 65: case 66: case 67: case 70: case 77: case 84: case 97: case 99: case 103: case 115: case 116: case 117:
                idx[26] = 1;
                v0 = 3;
                v7 = iter;
                iter = v7 + 1;
                *(v7) = 115;
                break;
            case 68: case 71: case 85: case 98: case 107: case 110:
                idx[26] = 1;
                v0 = 3;
                v13 = iter;
                iter = v13 + 1;
                *(v13) = 115;
                break;
            case 72:
                v12 = iter;
                iter = v12 + 1;
                *(v12) = 115;
                break;
            case 80: case 102: case 104: case 112:
                v10 = iter;
                iter = v10 + 1;
                *(v10) = 115;
                break;
            case 83:
                idx[26] = 1;
                v0 = 3;
                v8 = iter;
                iter = v8 + 1;
                *(v8) = 103;
                break;
            case 89:
                idx[26] = 1;
                v0 = 2;
                v9 = iter;
                iter = v9 + 1;
                *(v9) = 115;
                break;
            case 90:
                v0 = 5;
                v11 = iter;
                iter = v11 + 1;
                *(v11) = 115;
                break;
            case 100:
                v14 = iter;
                iter = v14 + 1;
                *(v14) = 100;
                break;
            case 105:
                idx[28] = 1;
                v0 = 1;
                v6 = iter;
                iter = v6 + 1;
                *(v6) = 115;
                break;
            case 108:
                idx[26] = 1;
                v0 = 4;
                v4 = iter;
                iter = v4 + 1;
                *(v4) = 115;
                break;
            case 109:
                v15 = iter;
                iter = v15 + 1;
                *(v15) = 111;
                idx[26] = 1;
                v0 = 3;
                break;
            case 121:
                idx[27] = 1;
                v0 = 2;
                v5 = iter;
                iter = v5 + 1;
                *(v5) = 115;
                break;
            }
            *(iter) = 0;
            if (v0 <= *((int *)&idx[32]))
                return &*(a0)->field_18;
            *((unsigned int *)&idx[32]) = v0;
            return &*(a0)->field_18;
        }
        if (a4)
        {
            __assert_fail(); /* do not return */
        }
        else if (a5)
        {
            __assert_fail(); /* do not return */
        }
        else
        {
            *(iter) = 0;
            if (v0 <= *((int *)&idx[32]))
                return &*(a0)->field_18;
            *((unsigned int *)&idx[32]) = 0;
            break;
            return &*(a0)->field_18;
        }
    }
}



// Function: is_octal_char @ 0x1133c
unsigned int is_octal_char(char a0)
{
    unsigned int v1;  // eax

    if (a0 > 47 && a0 <= 55)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: parse_octal_escape @ 0x11366
unsigned int parse_octal_escape(unsigned long a0, unsigned long long *a1)
{
    unsigned int v2;  // ebx
    int v3;  // r12d
    unsigned long long iter;  // [bp-0x20]

    iter = 0;
    v2 = 0;
    for (v3 = 0; v3 <= 2 && (char)sub_41133c(*((char *)(iter + a0))); iter += 1)
    {
        v2 = *((char *)(iter + a0)) + v2 * 8 - 48;
        v3 += 1;
    }
    iter -= 1;
    *(a1) = iter;
    return v2;
}



// Function: parse_escape_char @ 0x113f2

int parse_escape_char(char a0)
{
    char v0;  // [bp-0x9]

    v0 = 0;
    switch (a0)
    {
    case 92:
        v0 = 92;
        break;
    case 97:
        v0 = 7;
        break;
    case 98:
        v0 = 8;
        break;
    case 102:
        v0 = 12;
        break;
    case 110:
        v0 = 10;
        break;
    case 114:
        v0 = 13;
        break;
    case 116:
        v0 = 9;
        break;
    case 118:
        v0 = 11;
        break;
    }
    return v0;
}



// Function: get_format_flags_length @ 0x11467
unsigned long long get_format_flags_length(unsigned long a0)
{
    unsigned long long *v2;  // rax
    unsigned long long *v3;  // rax
    unsigned long long iter;  // [bp-0x10]

    iter = 0;
    do
    {
        iter += 1;
    } while (*((char *)(iter + a0)) && strchr("-+ #", *((char *)(iter + a0))));
    while (1)
    {
        v2 = __ctype_b_loc();
        if (!(*((short *)(*((char *)(iter + a0)) * 2 + *(v2))) & 0x800))
            break;
        iter += 1;
    }
    if (*((char *)(iter + a0)) != 46)
        return iter;
    iter += 1;
    while (1)
    {
        v3 = __ctype_b_loc();
        if (!(*((short *)(*((char *)(iter + a0)) * 2 + *(v3))) & 0x800))
            break;
        iter += 1;
    }
    return iter;
}



// Function: get_format_specifer_length @ 0x11545
unsigned int get_format_specifer_length(char a0)
{
    return (!strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%", a0) ? (!strchr("ABCT", a0) ? 0 : 2) : 1);
}



// Function: insert_fprintf @ 0x1159f
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int insert_fprintf(unsigned long long *index, struct_0 *a1, char *a2)
{
    unsigned long long v10;  // rbx
    unsigned long long v11;  // rbx
    char v0;  // [bp-0x5a]
    char v1;  // [bp-0x59]
    unsigned long long v2;  // [bp-0x58]
    char *v3;  // [bp-0x50]
    char *iter;  // [bp-0x48]
    struct_0 *v5;  // [bp-0x40]
    unsigned long long node;  // [bp-0x38]
    unsigned long long v7;  // [bp-0x30]
    struct_0 *idx;  // [bp-0x28]

    v3 = a2;
    idx = sub_40ee5a(a1, sub_41376d, *((unsigned int *)&a2));
    idx->padding_0[25] = 1;
    idx->padding_0[24] = idx->padding_0[25];
    v10 = index[1];
    *((unsigned long long *)&idx[1].padding_0[0]) = *(index);
    *((unsigned long long *)&idx[1].padding_0[8]) = v10;
    v11 = index[3];
    *((unsigned long long *)&idx[1].padding_0[16]) = index[2];
    *((unsigned long long *)&idx[1].padding_0[24]) = v11;
    *((unsigned long long *)&idx[1].padding_0[32]) = index[4];
    idx->padding_0[27] = 0;
    idx->padding_0[26] = 0;
    *((unsigned int *)&idx->padding_0[32]) = 0;
    v5 = idx + 1;
    *((unsigned long long *)&v5->padding_0[0]) = 0;
    iter = v3;
    while (1)
    {
        if (!*(iter))
        {
            if (iter <= v3)
                return 1;
            sub_410f4b(v5, v3, iter - v3, 0, 0, 0, idx);
            return 1;
        }
        if (*(iter) == 92 && iter[1] == 99)
        {
            sub_410f4b(v5, v3, iter - v3, 1, 0, 0, idx);
            if (idx->padding_0[26] && *((int *)&idx->padding_0[32]) <= 2)
                *((unsigned int *)&idx->padding_0[32]) = 3;
            break;
        }
        if (*(iter) == 92)
        {
            node = 1;
            if (!iter[node])
            {
                error(0, 0, gettext("warning: escape `\\' followed by nothing at all"));
                node -= 1;
            }
            else
            {
                if ((char)sub_41133c(iter[node]))
                {
                    v2 = 0;
                    *(iter) = sub_411366(&iter[node], &v2);
                    node += v2;
                }
                else
                {
                    v1 = sub_4113f2(iter[node]);
                    if (v1)
                    {
                        *(iter) = v1;
                    }
                    else
                    {
                        error(0, 0, gettext("warning: unrecognized escape `\\%c'"));
                        iter = &iter[node];
                        goto LABEL_411aa1;
                    }
                }
            }
            v5 = sub_410f4b(v5, v3, (unsigned int)(iter - v3) + 1, 0, 0, 0, idx);
            v3 = &iter[1 + node];
            iter = &iter[node];
        }
        else if (*(iter) == 37)
        {
            if (!iter[1])
                error(1, 0, gettext("error: %s at end of format string"));
            v7 = (iter[1] == 37 ? 1 : sub_411467(iter));
            iter = &iter[v7];
            v7 = sub_411545(*(iter));
            if (v7 && *(&iter[v7] - 1))
            {
                v0 = (v7 == 2 ? iter[1] : 0);
                v5 = sub_410f4b(v5, v3, iter - v3, 2, *(iter), v0, idx);
                iter = iter + v7 - 1;
            }
            else if (strchr("{[(", *(iter)))
            {
                error(1, 0, gettext("error: the format directive `%%%c' is reserved for future use"));
            }
            else
            {
                if (v7 == 2 && !iter[1])
                    error(0, 0, gettext("warning: format directive `%%%c' should be followed by another character"));
                else
                    error(0, 0, gettext("warning: unrecognized format directive `%%%c'"));
                v5 = sub_410f4b(v5, v3, iter + 1 - v3, 0, 0, 0, idx);
            }
            v3 = iter + 1;
        }
LABEL_411aa1:
        iter += 1;
    }
    return 1;
}



// Function: scan_for_digit_differences @ 0x11b14
unsigned int scan_for_digit_differences(unsigned long a0, unsigned long a1, unsigned long long *a2, unsigned long long *a3)
{
    unsigned long long *v3;  // rax
    unsigned long long *v4;  // rax
    char v0;  // [bp-0x11]
    unsigned long long i;  // [bp-0x10]

    v0 = 0;
    for (i = 0; *((char *)(i + a0)) && *((char *)(i + a1)); i += 1)
    {
        if (*((char *)(i + a0)) != *((char *)(i + a1)))
        {
            v3 = __ctype_b_loc();
            if (!(*((short *)(*((char *)(i + a0)) * 2 + *(v3))) & 0x800) || !(v4 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)(char)*((char *)(i + a1)) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x800))
                return 0;
            if (v0 != 1)
            {
                *(a2) = i;
                *(a3) = 1;
                v0 = 1;
            }
            else if (i - *(a2) == *(a3))
            {
                *(a3) = *(a3) + 1;
            }
            else
            {
                return 0;
            }
        }
    }
    if (!*((char *)(i + a0)) && !*((char *)(i + a1)))
        return 1;
    return 0;
}



// Function: do_time_format @ 0x11c89
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

extern struct_0 *g_449a60;
extern void g_449a68;

long long do_time_format(void* ptr, tm *idx, void* a2, unsigned long n)
{
    unsigned long v17;  // fs
    unsigned long long *v19;  // rax
    unsigned long long v0;  // [bp-0x98]
    unsigned long long v1;  // [bp-0x90]
    char *ptr1;  // [bp-0x88], Other Possible Types: unsigned long long
    unsigned long t;  // [bp-0x80]
    unsigned long v4;  // [bp-0x78]
    void* v5;  // [bp-0x70]
    unsigned long long v6;  // [bp-0x68]
    unsigned long n1;  // [bp-0x60]
    tm v8;  // [bp-0x58], Other Possible Types: unsigned long
    unsigned long v9;  // [bp-0x50]
    unsigned long v10;  // [bp-0x48]
    unsigned long v11;  // [bp-0x40]
    unsigned long v12;  // [bp-0x38]
    unsigned long v13;  // [bp-0x30]
    unsigned long v14;  // [bp-0x28]
    unsigned long v15;  // [bp-0x20]

    v15 = *((long long *)(40 + v17));
    ptr1 = 0;
    ptr1 = sub_431bc2(strlen(ptr) + 2);
    *(ptr1) = 95;
    memcpy(ptr1 + 1, ptr, strlen(ptr) + 1);
    v8 = *((long long *)&idx->tm_sec);
    v9 = *((long long *)&idx->tm_hour);
    v10 = *((long long *)&idx->tm_mon);
    v11 = *((long long *)&idx->tm_wday);
    v12 = *((long long *)&idx->tm_isdst);
    v13 = idx->tm_gmtoff;
    v14 = idx->tm_zone;
    v8 = (tm)((unsigned int)v8 <= 10 ? (unsigned int)v8 + 11 : (unsigned int)v8 - 11);
    v8 = v8;
    if (!g_449a60)
    {
        *((unsigned long *)&g_449a68) = 1;
        g_449a60 = sub_431bc2(*((long long *)&g_449a68));
    }
    while (1)
    {
        t = strftime(g_449a60, *((long long *)&g_449a68), ptr1, idx);
        if (t && t < *((long long *)&g_449a68))
            break;
        g_449a60 = sub_431dbf(g_449a60, &g_449a68, 1, &g_449a68);
    }
    v0 = 0;
    v1 = 0;
    v4 = n + t + 1;
    g_449a60 = sub_431c2c(g_449a60, v4, v4);
    *((unsigned long *)&g_449a68) = v4;
    v5 = sub_431bc2(v4);
    strftime(v5, *((long long *)&g_449a68), ptr1, &v8);
    if ((char)sub_411b14(g_449a60, v5, &v0, &v1) && v1 == 2)
    {
        v19 = __ctype_b_loc();
        if (!(*((short *)(*((char *)g_449a60 + v1 + v0) * 2 + *(v19))) & 0x800))
        {
            v6 = v1 + v0;
            n1 = t - v6 + 1;
            if (v4 != n1 + v6 + n)
                __assert_fail(); /* do not return */
            memmove((char *)g_449a60 + n + v6, (char *)g_449a60 + v6, n1);
            memcpy((char *)g_449a60 + v1 + v0, a2, n);
        }
    }
    free(ptr1);
    free(v5);
    if (v15 == *((long long *)(40 + v17)))
        return (char *)&g_449a60->field_0 + 1;
    __stack_chk_fail(); /* do not return */
}



// Function: format_date @ 0x1201f
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

extern char g_449a80;
extern char g_449d97;

char * format_date(int a0[2], int a1, unsigned int a2)
{
    int v0[2];  // [bp-0xa8]
    unsigned int v1;  // [bp-0x98]
    unsigned int v2;  // [bp-0x94]
    char *iter;  // [bp-0x90]
    tm *tm;  // [bp-0x88]
    char *v5;  // [bp-0x80]
    int v6[2];  // [bp-0x78]
    unsigned long len;  // [bp-0x70]
    unsigned long long v8;  // [bp-0x68]
    unsigned long v9;  // [bp-0x60]
    char v10;  // [bp-0x54]
    char v11;  // [bp-0x53]
    char v12;  // [bp-0x52]
    char v13[40];  // [bp-0x48]

    v0 = a0;
    v1 = 0;
    v2 = 0;
    if (a2 == 43)
    {
        strcpy(&v10, "%Y-%m-%d+%T");
        v2 = 1;
    }
    else
    {
        v10 = 37;
        v11 = a2;
        v12 = 0;
        if (24 >= a2 - 64 && (char)(18350081 >> ((char)(a2 - 64) & 63)) & 1)
            v2 = 1;
        else
            v2 = 0;
    }
    if (v2)
    {
        v1 = snprintf(v13, 32, ".%09ld0", a1);
        if (v1 > 31)
            __assert_fail(); /* do not return */
    }
    else
    {
        v1 = 0;
        v13 = 0;
    }
    if (a2 != 64)
    {
        tm = localtime(v0);
        if (tm)
        {
            v5 = sub_411c89(&v10, tm, v13, v1);
            if (v5)
                return v5;
        }
    }
    v6 = v0;
    if (v0 < NULL)
        v6 = -(v6);
    iter = sub_422eb1(v6);
    if (iter <= &g_449a80)
    {
        __assert_fail(); /* do not return */
    }
    else if (iter >= &g_449d97)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        if (v0 < NULL)
        {
            iter -= 1;
            *(iter) = 45;
        }
        if (!v2)
            return iter;
        len = strlen(iter);
        v8 = len + iter - &g_449a80;
        if (v8 > 790)
            __assert_fail(); /* do not return */
        v9 = 790 - v8;
        if (v9 <= strlen(v13))
            error(0, 0, "charsprinted=%ld but remaining=%lu: ns_buf=%s");
        if (v9 <= strlen(v13))
            __assert_fail(); /* do not return */
        strcat(iter, v13);
        return iter;
    }
}



// Function: ctime_format @ 0x123c8
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

extern unsigned long long g_449680[4];
extern unsigned long long g_4496c0[4];
extern void g_449da0;

void* ctime_format(unsigned long a0, int a1)
{
    unsigned long v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x14]
    tm *tm;  // [bp-0x10]

    v0 = a0;
    tm = localtime(&v0);
    if (!tm)
    {
        return sub_41201f(v0, a1, 64);
    }
    else if (tm->tm_wday < 0)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_wday > 6)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_mon < 0)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_mon > 11)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_hour < 0)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_hour > 23)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_min > 59)
    {
        __assert_fail(); /* do not return */
    }
    else if (tm->tm_sec > 61)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        v1 = snprintf(&g_449da0, 0x400, "%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d", g_449680[tm->tm_wday], g_4496c0[tm->tm_mon], tm->tm_mday, tm->tm_hour, tm->tm_min, tm->tm_sec, a1, tm->tm_year + 1900);
        if (v1 <= 0x3ff)
            return &g_449da0;
        __assert_fail(); /* do not return */
    }
}



// Function: file_sparseness @ 0x12693
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
    char padding_38[8];
    long long field_40;
} struct_0;

unsigned long long file_sparseness(struct_0 *a0)
{
    unsigned long long v0;  // [bp-0x10]

    if (a0->field_30)
    {
        v0 = MulV(0x4080000000000000, a0->field_40);
        return DivV(v0, a0->field_30);
    }
    return (!a0->field_40 ? 0x3ff0000000000000 : (a0->field_40 < 0 ? 0xfff0000000000000 : 0x7ff0000000000000));
}



// Function: checked_fprintf @ 0x1272c
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    struct FILE *field_8;
    long long field_10;
} struct_0;

void checked_fprintf(struct_0 *a0, char *a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v13;  // fs
    int *err;  // rax
    unsigned int v0;  // [bp-0xec]
    unsigned int v1;  // [bp-0xe8]
    unsigned int v2;  // [bp-0xe4]
    char *v3;  // [bp-0xe0]
    char *v4;  // [bp-0xd8]
    unsigned long v5;  // [bp-0xd0]
    char v6;  // [bp-0xc8]
    unsigned long long v7;  // [bp-0xb8]
    unsigned long long v8;  // [bp-0xb0]
    unsigned long long v9;  // [bp-0xa8]
    unsigned long long v10;  // [bp-0xa0]
    unsigned long v11;  // [bp+0x8]

    v7 = a2;
    v8 = a3;
    v9 = a4;
    v10 = a5;
    v5 = *((long long *)(40 + v13));
    v1 = 16;
    v2 = 48;
    v3 = &v11;
    v4 = &v6;
    v0 = vfprintf(a0->field_8, a1, &v1);
    if (v0 < 0)
    {
        err = __errno_location();
        sub_410a77(*(err), a0->field_10);
    }
    if (v5 == *((long long *)(40 + v13)))
        return;
    __stack_chk_fail(); /* do not return */
}



// Function: checked_print_quoted @ 0x12839
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    long long field_10;
    char field_18;
    char padding_19[7];
    long long field_20;
} struct_0;

long long checked_print_quoted(struct_0 *a0, long long a1, long long a2)
{
    int *err;  // rax
    unsigned int v0;  // [bp-0x1c]

    v0 = sub_41c643(a0->field_8, a0->field_20, a0->field_18, a1, a2);
    if (v0 >= 0)
        return v0;
    err = __errno_location();
    return sub_410a77(*(err), a0->field_10);
}



// Function: checked_fwrite @ 0x128a9
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    struct FILE *field_8;
    long long field_10;
} struct_0;

long long checked_fwrite(void* a0, unsigned long a1, unsigned long a2, struct_0 *a3)
{
    int *err;  // rax
    unsigned long count;  // [bp-0x20]

    count = fwrite(a0, a1, a2, a3->field_8);
    if (count >= a2)
        return count;
    err = __errno_location();
    return sub_410a77(*(err), a3->field_10);
}



// Function: checked_fflush @ 0x12910
long long checked_fflush(long long *a0)
{
    unsigned long v1;  // rax
    int *err;  // rax

    v1 = sub_41e1a8(a0[1]);
    if (!(unsigned int)v1)
        return v1;
    err = __errno_location();
    return sub_410a77(*(err), a0[2]);
}



// Function: mode_to_filetype @ 0x12955
extern char g_43aa33;
extern char g_43aa35;
extern char g_43aa37;
extern char g_43aa39;
extern char g_43aa3b;
extern char g_43aa3d;
extern char g_43aa3f;
extern char g_43aa41;

int mode_to_filetype(unsigned int a0)
{
    return (a0 == 0x8000 ? &g_43aa33 : (a0 == 0x4000 ? &g_43aa35 : (a0 == 0xa000 ? &g_43aa37 : (a0 == 0xc000 ? &g_43aa39 : (a0 == 0x6000 ? &g_43aa3b : (a0 == 0x2000 ? &g_43aa3d : (a0 == 0x1000 ? &g_43aa3f : &g_43aa41)))))));
}



// Function: do_fprintf @ 0x129e7
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

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

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern char g_43aa45;
extern struct_1 *g_4499a0;
extern char g_4499c0;
extern long long g_4499d0;
extern unsigned int g_4499d8;
extern int g_4499dc;
extern unsigned int g_4499e4;

void do_fprintf(unsigned long a0, void* idx, unsigned long a2, struct_0 *a3, unsigned long long a4, unsigned long long a5)
{
    unsigned int v17;  // eax
    int v18;  // edx
    unsigned long long v22;  // r8
    unsigned long long v23;  // r9
    unsigned long long v24;  // rax
    long long v25;  // rax
    unsigned long long v26;  // rax
    unsigned long v19;  // rax
    unsigned long long v27;  // rax
    unsigned long long v28;  // rax
    int *err;  // rax
    unsigned long long v30;  // rax
    unsigned long long v31;  // rax
    unsigned long v32;  // rax
    unsigned long long v33;  // rax
    unsigned long long v20;  // rax
    int *err1;  // rax
    unsigned long long v35;  // rax
    unsigned long long v36;  // rax
    unsigned long long v37;  // rax
    char *v38;  // rax
    int *err2;  // rax
    unsigned long v21;  // rax
    char v0;  // [bp-0x399]
    unsigned int v1;  // [bp-0x398]
    unsigned int v2;  // [bp-0x394]
    char v3;  // [bp-0x390]
    char *v4;  // [bp-0x388]
    char *ptr;  // [bp-0x380]
    unsigned long v6;  // [bp-0x378]
    passwd *v7;  // [bp-0x370]
    char *v8;  // [bp-0x368]
    group *v9;  // [bp-0x360]
    unsigned long v10;  // [bp-0x358]
    unsigned long ptr1;  // [bp-0x350]
    char v12;  // [bp-0x348]
    unsigned int v13;  // [bp-0x330]
    char v14;  // [bp-0x2b8]
    char v15;  // [bp-0x2ae]

    v17 = *((int *)idx);
    if (v17 != 2)
    {
        if (v17 > 2)
        {
            return;
        }
        else if (!v17)
        {
            sub_4128a9((long long)idx[8], 1, (int)idx[16], a0);
            return;
        }
        else if (v17 == 1)
        {
            sub_4128a9((long long)idx[8], 1, (int)idx[16], a0);
            sub_412910(a0);
            return;
        }
        else
        {
            return;
        }
    }
    switch ((char)idx[4])
    {
    case 0:
        error(1, 0, gettext("error: %s at end of format string"));
    case 37:
        sub_4128a9((long long)idx[8], 1, (int)idx[16], a0);
        goto LABEL_41374b;
    case 68:
        v24 = sub_422eb1(a3->field_0, &v14, 0, 1, 1);
        sub_41272c(a0, (long long)idx[8], v24, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 70:
        v25 = sub_413a18(a3, a2, a2);
        sub_412839(a0, (long long)idx[8], v25);
        goto LABEL_41374b;
    case 71:
LABEL_412d65:
        v26 = sub_422eb1(a3->field_20, &v14, 0, 1, 1);
        sub_41272c(a0, (long long)idx[8], v26, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 72:
        ptr1 = sub_431bc2(g_4499dc + 1);
        memcpy(ptr1, a2, g_4499dc);
        *((char *)(ptr1 + g_4499dc)) = 0;
        sub_41272c(a0, (long long)idx[8], ptr1, (long long)idx[8], v22, v23);
        free(ptr1);
        goto LABEL_41374b;
    case 77:
        sub_41e472(a3, &v14, &v14);
        v15 = 0;
        sub_41272c(a0, (long long)idx[8], &v14, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 80:
        if (*((int *)&g_4499c0) > 0)
        {
            v4 = a2 + g_4499dc;
            if (*(v4) == 47)
            {
                v4 += 1;
                break;
            }
        }
        else
        {
            v4 = &g_43aa45;
            break;
        }
        sub_412839(a0, (long long)idx[8], v4);
        goto LABEL_41374b;
    case 83:
        sub_412693(a3);
        sub_41272c(a0, (long long)idx[8], a0, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 85:
LABEL_4133c6:
        v33 = sub_422eb1(a3->field_1c, &v14, 0, 1, 1);
        sub_41272c(a0, (long long)idx[8], v33, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 89:
        if (((unsigned short)a3->field_18 & 0xf000) != 0xa000)
        {
            v36 = sub_412955(a3->field_18 & 0xf000);
            sub_41272c(a0, (long long)idx[8], v36, (long long)idx[8], v22, v23);
            goto LABEL_41374b;
        }
        else if (!fstatat(g_4499d8, g_4499d0, &v12, 0))
        {
            v35 = sub_412955(v13 & 0xf000);
            sub_41272c(a0, (long long)idx[8], v35, (long long)idx[8], v22, v23);
            goto LABEL_41374b;
        }
        else if (*(__errno_location()) == 2 || *(__errno_location()) == 20)
        {
            sub_41272c(a0, (long long)idx[8], "N", (long long)idx[8], v22, v23);
            goto LABEL_41374b;
        }
        else
        {
            if (*(__errno_location()) == 40)
            {
                sub_41272c(a0, (long long)idx[8], "L", (long long)idx[8], v22, v23);
            }
            else
            {
                sub_41272c(a0, (long long)idx[8], "?", (long long)idx[8], v22, v23);
                sub_41096a(0, a2);
                err1 = __errno_location();
                error(0, *(err1), "%s");
            }
            goto LABEL_41374b;
        }
    case 90:
        v2 = g_4499a0(g_4499d8, g_4499d0, &v3, g_4499d0, g_4499a0);
        if (v2 < 0)
        {
            sub_41272c(a0, (long long)idx[8], &g_43aa45, (long long)idx[8], v22, v23);
            sub_41096a(0, a2);
            v38 = gettext("getfilecon failed: %s");
            err2 = __errno_location();
            error(0, *(err2), v38);
            g_4499e4 = 1;
            goto LABEL_41374b;
        }
        else
        {
            sub_41272c(a0, (long long)idx[8], v3, (long long)idx[8], v22, v23);
            freecon(v3);
            goto LABEL_41374b;
        }
    case 97:
        v19 = sub_4123c8(sub_430a4c(a3), v18);
        sub_41272c(a0, (long long)idx[8], v19, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 98:
        v20 = sub_422eb1(a3->field_40, &v14, 0, 0x200, 0x200);
        sub_41272c(a0, (long long)idx[8], v20, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 99:
        v21 = sub_4123c8(sub_430a66(a3), v18);
        sub_41272c(a0, (long long)idx[8], v21, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 100:
        sub_41272c(a0, (long long)idx[8], *((int *)&g_4499c0), (long long)idx[8], a4, a5);
        goto LABEL_41374b;
    case 102:
        v10 = sub_41d85c(a2);
        sub_412839(a0, (long long)idx[8], v10);
        free(v10);
        goto LABEL_41374b;
    case 103:
        v9 = getgrgid(a3->field_20);
        if (!v9)
            goto LABEL_412d65;
        else
            goto LABEL_412d17;
    case 104:
        v8 = sub_4321c9(a2);
        for (ptr = v8 + strlen(v8) - 1; v8 <= ptr && *(ptr) == 47; ptr -= 1);
        if (v8 < ptr && ptr[1] == 47)
            ptr[1] = 0;
        ptr = strrchr(v8, 47);
        if (!ptr)
        {
            sub_412839(a0, (long long)idx[8], ".");
        }
        else
        {
            *(ptr) = 0;
            sub_412839(a0, (long long)idx[8], v8);
        }
        free(v8);
        goto LABEL_41374b;
    case 105:
        v27 = sub_422eb1(a3->field_8, &v14, 0, 1, 1);
        sub_41272c(a0, (long long)idx[8], v27, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 107:
        v28 = sub_422eb1(a3->field_40, &v14, 0, 0x200, 0x400);
        sub_41272c(a0, (long long)idx[8], v28, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 108:
        v6 = 0;
        if (((unsigned short)a3->field_18 & 0xf000) == 0xa000)
        {
            v6 = sub_41d06d(g_4499d8, g_4499d0, g_4499d0);
            if (!v6)
            {
                err = __errno_location();
                sub_410a13(*(err), a2);
                g_4499e4 = 1;
            }
        }
        if (v6)
            sub_412839(a0, (long long)idx[8], v6);
        else
            sub_412839(a0, (long long)idx[8], &g_43aa45);
        free(v6);
        goto LABEL_41374b;
    case 109:
        v1 = a3->field_18;
        v0 = 1;
        sub_41272c(a0, (long long)idx[8], v1 & 0xfff, (long long)idx[8], a4, a5);
        goto LABEL_41374b;
    case 110:
        v30 = sub_422eb1(a3->field_10, &v14, 0, 1, 1);
        sub_41272c(a0, (long long)idx[8], v30, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 112:
        sub_412839(a0, (long long)idx[8], a2);
        goto LABEL_41374b;
    case 115:
        v31 = sub_422eb1(a3->field_30, &v14, 0, 1, 1);
        sub_41272c(a0, (long long)idx[8], v31, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 116:
        v32 = sub_4123c8(sub_430a80(a3), v18);
        sub_41272c(a0, (long long)idx[8], v32, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 117:
        v7 = getpwuid(a3->field_1c);
        if (!v7)
            goto LABEL_4133c6;
        *((char *)((int)idx[16] + (long long)idx[8])) = 115;
        sub_41272c(a0, (long long)idx[8], v7->pw_name, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    case 121:
        v37 = sub_412955(a3->field_18 & 0xf000);
        sub_41272c(a0, (long long)idx[8], v37, (long long)idx[8], v22, v23);
        goto LABEL_41374b;
    default:
LABEL_41374b:
        return;
    }
LABEL_412d17:
    *((char *)((int)idx[16] + (long long)idx[8])) = 115;
    sub_41272c(a0, (long long)idx[8], v9->gr_name, (long long)idx[8], v22, v23);
    goto LABEL_41374b;
}



// Function: pred_fprintf @ 0x1376d
typedef struct struct_0 {
    char padding_0[56];
    void* field_38;
} struct_0;

extern unsigned long long g_43aa45;

unsigned int pred_fprintf(unsigned int a0, unsigned long a1, struct_0 *a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    unsigned long long v7;  // rdx
    unsigned long long v9;  // r9
    unsigned long long v10;  // r8
    unsigned long v8;  // rax
    unsigned int v0;  // [bp-0x2c]
    void* iter;  // [bp-0x28]
    struct_0 *v2;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]
    unsigned long long v4;  // [bp-0x10]

    v2 = &a2->field_38;
    for (iter = *((long long *)&v2->padding_0[0]); iter; iter = (long long)iter[24])
    {
        if (*((int *)iter) == 2 && (char)iter[5])
        {
            v0 = 0;
            switch ((char)iter[4])
            {
            case 84:
                v3 = sub_430a80(a1);
                v4 = v7;
                v0 = 1;
                break;
            case 67:
                v3 = sub_430a66(a1);
                v4 = v7;
                v0 = 1;
                break;
            case 65:
                v3 = sub_430a4c(a1);
                v4 = v7;
                v0 = 1;
                break;
            case 66:
                v3 = sub_430a9a(a1);
                v4 = v7;
                v0 = ((char)iter[5] == 64 ? 1 : (char)(~(v4) >> 63));
                break;
            default:
                __assert_fail(); /* do not return */
            }
            if (v0)
            {
                v8 = sub_41201f(v3, v4, (char)iter[5]);
                sub_41272c(v2, (long long)iter[8], v8, (long long)iter[8], v10, v9);
            }
            else
            {
                sub_41272c(v2, (long long)iter[8], &g_43aa45, (long long)iter[8], v10, v9);
            }
        }
        else
        {
            sub_4129e7(v2, iter, a0, a1, v10, v9);
        }
    }
    return 1;
}



// Function: free_file_system_list @ 0x1395f
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

struct_0 * free_file_system_list(struct_0 *a0)
{
    struct_0 *v3;  // rax
    struct_0 *i;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x10]

    for (i = a0; i; i = v1)
    {
        v1 = i->field_30;
        sub_426f67(i);
    }
    return v3;
}



// Function: get_file_system_list @ 0x1399c
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

extern struct_0 *g_44a1a0;
extern char g_44a1a8;

struct_0 * get_file_system_list(char a0)
{
    if (g_44a1a0 && g_44a1a8 != 1 && a0)
    {
        sub_41395f(g_44a1a0);
        g_44a1a0 = 0;
    }
    if (!g_44a1a0)
    {
        g_44a1a0 = sub_4262d1(a0);
        g_44a1a8 = a0;
    }
    return g_44a1a0;
}



// Function: filesystem_type @ 0x13a18
extern void* g_44a1b0;
extern char g_44a1b8;
extern unsigned long long g_44a1c0;

void* filesystem_type(unsigned long long *a0, long long a1)
{
    if (g_44a1b0)
    {
        if (g_44a1b8 && *(a0) == g_44a1c0)
            return g_44a1b0;
        free(g_44a1b0);
    }
    g_44a1c0 = *(a0);
    g_44a1b0 = sub_413be0(a0, a1, &g_44a1b8, a1);
    return g_44a1b0;
}



// Function: is_used_fs_type @ 0x13aa6
typedef struct struct_0 {
    char padding_0[48];
    struct struct_0 *field_30;
} struct_0;

unsigned int is_used_fs_type(char *a0)
{
    struct_0 *iter;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    if (!strcmp("afs", a0))
        return 1;
    v1 = sub_41399c(0);
    if (!v1)
        return 1;
    for (iter = v1; iter; iter = iter->field_30)
    {
        if (!strcmp(a0, *((long long *)&iter->padding_0[24])))
            return 1;
    }
    return 0;
}



// Function: set_fstype_devno @ 0x13b3b
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_449990;

unsigned int set_fstype_devno(unsigned long long *a0)
{
    unsigned long long v0;  // [bp-0xa8]

    if (a0[4] != 0xffffffffffffffff)
        return 0;
    sub_40f26e(&v0);
    if (g_449990(a0[1], &v0, &v0, g_449990))
        return 0xffffffff;
    a0[4] = v0;
    return 0;
}



// Function: file_system_type_uncached @ 0x13be0
unsigned long long file_system_type_uncached(unsigned long a0, unsigned long long a1, char *a2)
{
    unsigned long long v0;  // [bp-0x38]
    void* iter;  // [bp-0x28]
    void* v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    void* v4;  // [bp-0x10]

    v0 = a1;
    v2 = NULL;
    v4 = sub_41399c(1);
    if (!v4)
        error(1, 0, gettext("Cannot read mounted file system list"));
    v3 = 0;
    for (iter = v4; iter; iter = (long long)iter[48])
    {
        if (strcmp((long long)iter[24], "ignore") && !sub_413b3b(iter) && (long long)iter[32] == *((long long *)a0))
            v2 = iter;
    }
    if (v2)
        v3 = sub_4321c9((long long)v2[24]);
    *(a2) = v3;
    return (!v3 ? sub_4321c9(gettext("unknown")) : v3);
}



// Function: get_mounted_devices @ 0x13cfc
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    char padding_28[8];
    struct struct_0 *field_30;
} struct_0;

long long get_mounted_devices(unsigned long long *a0)
{
    unsigned long v0;  // [bp-0x40]
    unsigned long long index;  // [bp-0x38]
    struct_0 *iter;  // [bp-0x30]
    unsigned long v3;  // [bp-0x28]
    struct_0 *v4;  // [bp-0x20]
    unsigned long v5;  // [bp-0x18]

    v0 = 0;
    index = 0;
    v3 = 0;
    v4 = sub_4262d1(0);
    for (iter = v4; iter; iter = iter->field_30)
    {
        v5 = sub_41ac29(v3, (index + 1) * 8, &v0, (index + 1) * 8);
        if (v5)
        {
            v3 = v5;
            if (!sub_413b3b(iter))
            {
                *((unsigned long long *)(index * 8 + v3)) = iter->field_20;
                index += 1;
            }
        }
        else
        {
            free(v3);
            v3 = 0;
        }
    }
    sub_41395f(v4);
    if (!v3)
        return v3;
    *(a0) = index;
    return v3;
}



// Function: fallback_getfilecon @ 0x13e1c
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned long long g_449980;

int fallback_getfilecon(unsigned int a0, unsigned long a1, long long a2, unsigned int a3)
{
    unsigned int v1;  // eax

    v1 = *(__errno_location());
    if (*(__errno_location()) != 2 && *(__errno_location()) != 20)
        return a3;
    if ((char)g_449980 & 2)
        fprintf(stderr, "fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n", a1);
    return sub_43011a(a0, a1, a2, a1);
}



// Function: optionh_getfilecon @ 0x13e94
extern unsigned int g_4499c0;

unsigned int optionh_getfilecon(unsigned int a0, long long a1, long long a2)
{
    unsigned int v0;  // [bp-0xc]

    if (g_4499c0)
        return sub_43011a(a0, a1, a2, a1);
    v0 = sub_42fe8c(a0, a1, a2, a1);
    return (!v0 ? 0 : (int)sub_413e1c(a0, a1, a2, v0));
}



// Function: optionl_getfilecon @ 0x13f08
unsigned int optionl_getfilecon(unsigned int a0, long long a1, long long a2)
{
    unsigned int v0;  // [bp-0xc]

    v0 = sub_42fe8c(a0, a1, a2, a1);
    return (!v0 ? 0 : (int)sub_413e1c(a0, a1, a2, v0));
}



// Function: optionp_getfilecon @ 0x13f5b
int optionp_getfilecon(unsigned int a0, long long a1, long long a2)
{
    return sub_43011a(a0, a1, a2, a1);
}



// Function: check_option_combinations @ 0x13f89
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[256];
    struct struct_0 *field_108;
} struct_0;

extern char g_449941;

long long check_option_combinations(struct_0 *a0)
{
    unsigned long v3;  // rax
    struct_0 *iter;  // [bp-0x20]
    unsigned int v1;  // [bp-0xc]

    iter = a0;
    for (v1 = 0; iter; iter = iter->field_108)
    {
        if (iter->field_0 == sub_4090b1)
        {
            v1 |= 1;
        }
        else if (iter->field_0 == sub_40a232)
        {
            v1 |= 2;
        }
    }
    if (!((char)v1 & 2))
    {
        return v1 & 2;
    }
    else if ((char)v1 & 1)
    {
        v3 = g_449941 ^ 1;
        if (!(char)v3)
            return v3;
        error(1, 0, gettext("The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option."));
    }
    else
    {
        return v1 & 1;
    }
}



// Function: get_noop @ 0x14033
extern unsigned int g_447de0[4];
extern unsigned long long g_447de8[4];
extern unsigned long long g_44a1d0;

unsigned long long get_noop(void)
{
    int i;  // [bp-0xc]

    if (!g_44a1d0)
    {
        for (i = 0; g_447de8[4 * i]; i += 1)
        {
            if (g_447de0[8 * i] == 0x1)
            {
                g_44a1d0 = &g_447de0[8 * i];
                break;
            }
        }
    }
    return g_44a1d0;
}



// Function: get_stat_Ytime @ 0x140b3
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
} struct_0;

unsigned long long get_stat_Ytime(long long a0, char a1, struct_0 *a2)
{
    struct_0 *v2;  // rcx
    unsigned long long v3;  // rdx
    struct_0 *v4;  // rcx
    unsigned long long v5;  // rdx
    struct_0 *v6;  // rcx
    unsigned long long v7;  // rdx
    struct_0 *v8;  // rcx
    unsigned long long v9;  // rdx

    switch (a1)
    {
    case 109:
        v8 = a2;
        v8->field_0 = sub_430a80(a0);
        v8->field_8 = v9;
        return 1;
    case 99:
        v6 = a2;
        v6->field_0 = sub_430a66(a0);
        v6->field_8 = v7;
        return 1;
    case 66:
        v4 = a2;
        v4->field_0 = sub_430a9a(a0);
        v4->field_8 = v5;
        return (char)(~(a2->field_8) >> 63);
    case 97:
        v2 = a2;
        v2->field_0 = sub_430a4c(a0);
        v2->field_8 = v3;
        return 1;
    default:
        __assert_fail(); /* do not return */
    }
}



// Function: set_follow_state @ 0x141b0
extern char g_44994c;
extern unsigned long long g_449980;
extern unsigned int g_449988;
extern unsigned long long g_449990;
extern unsigned long long g_4499a0;

unsigned long long set_follow_state(unsigned int a0)
{
    unsigned long long v1;  // rax

    if (a0 == 2)
    {
        g_449990 = sub_40fb5c;
        g_4499a0 = sub_413e94;
        g_44994c = 1;
    }
    else if (a0 <= 2)
    {
        if (!a0)
        {
            g_449990 = sub_40fcc0;
            g_4499a0 = sub_413f5b;
        }
        else if (a0 == 1)
        {
            g_449990 = sub_40fc21;
            g_4499a0 = sub_413f08;
            g_44994c = 1;
        }
    }
    g_449988 = a0;
    v1 = (unsigned int)g_449980 & 2;
    if (v1)
    {
        v1 = sub_40fd39;
        g_449990 = sub_40fd39;
    }
    return v1;
}



// Function: parse_begin_user_args @ 0x14262
extern unsigned long long g_44a1c8;

void parse_begin_user_args(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    g_44a1c8 = 0;
    return;
}



// Function: parse_end_user_args @ 0x14287
void parse_end_user_args(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    return;
}



// Function: should_issue_warnings @ 0x142a1
extern char g_449950;
extern char g_449951;

unsigned int should_issue_warnings(void)
{
    return (!g_449951 ? g_449950 : 0);
}



// Function: found_parser @ 0x142c4
extern unsigned long long g_44a1c8;

unsigned int * found_parser(unsigned long a0, unsigned int *a1)
{
    if (*(a1) != 2)
    {
        if (*(a1) == 1)
        {
            return NULL;
        }
        else if (!*(a1))
        {
            if (g_44a1c8 && (char)sub_4142a1())
                error(0, 0, gettext("warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments."));
        }
        else
        {
            if (!g_44a1c8)
                g_44a1c8 = a0;
        }
    }
    return a1;
}



// Function: find_parser @ 0x1437a
extern unsigned int g_447dc0;
extern char g_447de0;
extern unsigned long long g_447de8[4];

unsigned int * find_parser(char *a0)
{
    char *v0;  // [bp-0x20]
    int i;  // [bp-0x14]
    char *v2;  // [bp-0x10]

    v0 = a0;
    v2 = v0;
    if (!strncmp("-newer", v0, 6) && strlen(v0) == 8)
        return sub_4142c4(v2, &g_447dc0);
    if (*(v0) == 45)
        v0 += 1;
    for (i = 0; g_447de8[4 * i]; i += 1)
    {
        if (!strcmp(g_447de8[4 * i], v0))
            return sub_4142c4(v2, &(&g_447de0)[32 * i]);
    }
    return NULL;
}



// Function: estimate_file_age_success_rate @ 0x1446f
void estimate_file_age_success_rate(void)
{
    unsigned int v1;  // ymm0

    if ((((char)(CmpF(1036831949, v1) & 69) | (char)((CmpF(1036831949, v1) & 69) >> 6)) & 1) != 1)
    {
        return;
    }
    else if ((((char)(CmpF(0x3f800000, v1) & 69) | (char)((CmpF(0x3f800000, v1) & 69) >> 6)) & 1) != 1)
    {
        return;
    }
    else if ((((char)(CmpF(v1, 0x42c80000) & 69) | (char)((CmpF(v1, 0x42c80000) & 69) >> 6)) & 1) != 1)
    {
        return;
    }
    else
    {
        return;
    }
}



// Function: estimate_timestamp_success_rate @ 0x144ce
extern unsigned long long g_449968;

void estimate_timestamp_success_rate(unsigned long a0)
{
    uint128_t v2;  // rcx
    unsigned int v0;  // [bp-0xc]

    v2 = g_449968 - a0;
    v0 = (unsigned long long)((int128_t)(v2 * 1749024623285053783) >> 77) - ((int128_t)(v2) >> 63);
    sub_41446f();
    return;
}



// Function: collect_arg_nonconst @ 0x14528
unsigned int collect_arg_nonconst(unsigned long long a0, int *a1, unsigned long long *a2)
{
    if (a0 && *((long long *)(a0 + *(a1) * 8)))
    {
        *(a2) = *((long long *)(a0 + *(a1) * 8));
        *(a1) = *(a1) + 1;
        return 1;
    }
    *(a2) = 0;
    return 0;
}



// Function: collect_arg @ 0x145ab
long long collect_arg(unsigned long long a0, int *a1, unsigned long long *a2)
{
    char v0;  // [bp-0x19]
    unsigned long long v1;  // [bp-0x18]

    v0 = sub_414528(a0, a1, &v1);
    *(a2) = v1;
    return v0;
}



// Function: collect_arg_stat_info @ 0x14611
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_449990;

unsigned int collect_arg_stat_info(unsigned long long a0, int *a1, unsigned long a2, long long *a3)
{
    int *err;  // rax
    long long v0;  // [bp-0x28]

    if (!(char)sub_4145ab(a0, a1, &v0))
    {
        *(a3) = 0;
        return 0;
    }
    *(a3) = v0;
    if (!g_449990(v0, a2, a2, g_449990))
        return 1;
    err = __errno_location();
    sub_410a44(*(err), v0); /* do not return */
}



// Function: parse_and @ 0x146c4
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_and(long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40e644(a0);
    *((void* *)&idx->padding_0[0]) = sub_408d9c;
    *((unsigned int *)&idx->padding_0[16]) = 3;
    *((unsigned int *)&idx->padding_0[20]) = 3;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    return 1;
}



// Function: parse_anewer @ 0x1472e
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_anewer(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned long v5;  // rdx
    unsigned int v6;  // ymm0
    long long v0;  // [bp-0xc8]
    struct_0 *idx;  // [bp-0xc0]
    char v2;  // [bp-0xb8]
    unsigned int v3;  // [bp-0x60]

    sub_40f26e(&v2);
    if (!(char)sub_414611(a1, a2, &v2, &v0))
        return 0;
    idx = sub_40eecc(a0, v0);
    *((unsigned int *)&idx[1].padding_0[0]) = 0;
    *((unsigned long long *)&idx[1].padding_0[8]) = sub_430a80(&v2);
    *((unsigned long *)&idx[1].padding_0[16]) = v5;
    *((unsigned int *)&idx[1].padding_0[4]) = 0;
    sub_4144ce(v3);
    *((unsigned int *)&idx->padding_0[36]) = v6;
    return 1;
}



// Function: parse_closeparen @ 0x14833
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_closeparen(long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40e644(a0);
    *((void* *)&idx->padding_0[0]) = sub_408ee1;
    *((unsigned int *)&idx->padding_0[16]) = 5;
    *((unsigned int *)&idx->padding_0[20]) = 0;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    return 1;
}



// Function: parse_cnewer @ 0x1489d
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_cnewer(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned long v5;  // rdx
    unsigned int v6;  // ymm0
    long long v0;  // [bp-0xc8]
    struct_0 *idx;  // [bp-0xc0]
    char v2;  // [bp-0xb8]
    unsigned int v3;  // [bp-0x60]

    sub_40f26e(&v2);
    if (!(char)sub_414611(a1, a2, &v2, &v0))
        return 0;
    idx = sub_40eecc(a0, v0);
    *((unsigned int *)&idx[1].padding_0[0]) = 2;
    *((unsigned long long *)&idx[1].padding_0[8]) = sub_430a80(&v2);
    *((unsigned long *)&idx[1].padding_0[16]) = v5;
    *((unsigned int *)&idx[1].padding_0[4]) = 0;
    sub_4144ce(v3);
    *((unsigned int *)&idx->padding_0[36]) = v6;
    return 1;
}



// Function: parse_comma @ 0x149a2
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_comma(long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40e644(a0);
    *((void* *)&idx->padding_0[0]) = sub_408fd0;
    *((unsigned int *)&idx->padding_0[16]) = 3;
    *((unsigned int *)&idx->padding_0[20]) = 1;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: parse_daystart @ 0x14a1d
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

extern void g_449968;
extern unsigned long long g_449970;
extern char g_449978;

unsigned int parse_daystart(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]
    tm *tm;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    if (g_449978 == 1)
        return 1;
    *((unsigned long long *)&g_449968) = *((long long *)&g_449968) + 86400;
    g_449970 = 0;
    tm = localtime(&g_449968);
    *((unsigned long long *)&g_449968) = *((long long *)&g_449968) - (!tm ? *((long long *)&g_449968) - ((*((long long *)&g_449968) * 1749024623285053783 >> 77) - (*((long long *)&g_449968) >> 63)) * 86400 : tm->tm_hour * 3600 + tm->tm_sec + tm->tm_min * 60);
    g_449978 = 1;
    return 1;
}



// Function: parse_delete @ 0x14afc
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern char g_449940;

unsigned int parse_delete(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40ef32(a0);
    idx->padding_0[25] = 1;
    idx->padding_0[24] = idx->padding_0[25];
    g_449940 = 0;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: parse_depth @ 0x14b71
extern char g_449940;
extern char g_449941;

long long parse_depth(long long a0, long long a1, long long a2)
{
    g_449940 = 0;
    g_449941 = 1;
    return sub_417bae(a0, a1, a2, a1);
}



// Function: parse_d @ 0x14bb0
long long parse_d(long long a0, long long a1, long long a2)
{
    if ((char)sub_4142a1())
        error(0, 0, gettext("warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature."));
    return sub_414b71(a0, a1, a2);
}



// Function: parse_empty @ 0x14c10
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_empty(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *v2;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    v2 = sub_40ef32(a0);
    *((unsigned int *)&v2->padding_0[36]) = 1008981770;
    return 1;
}



// Function: parse_exec @ 0x14c50
int parse_exec(long long a0, long long a1, long long a2)
{
    return sub_41893a("-exec", a0, a1, a2);
}



// Function: parse_execdir @ 0x14c88
int parse_execdir(long long a0, long long a1, long long a2)
{
    return sub_41893a("-execdir", a0, a1, a2);
}



// Function: insert_false @ 0x14cc0
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int insert_false(void)
{
    void* v0;  // [bp-0x18]
    struct_0 *idx;  // [bp-0x10]

    v0 = sub_41437a("false");
    idx = sub_40ef32(v0);
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    idx->padding_0[25] = 0;
    idx->padding_0[24] = idx->padding_0[25];
    *((unsigned int *)&idx->padding_0[36]) = 0;
    return 1;
}



// Function: parse_false @ 0x14d31
unsigned int parse_false(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    return sub_414cc0();
}



// Function: parse_files0_from @ 0x14d50
extern unsigned long long g_4499b0;

unsigned int parse_files0_from(unsigned long long a0, unsigned long long a1, int *a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]

    v0 = a0;
    if (!(char)sub_4145ab(a1, a2, &v1))
        return 0;
    g_4499b0 = v1;
    return 1;
}



// Function: insert_fls @ 0x14dbf
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int insert_fls(struct_0 *a0, long long a1)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_40ef32(a0);
    if (a1)
        sub_419964(a1, idx + 1, idx + 1);
    else
        sub_419a85(idx + 1);
    idx->padding_0[25] = 1;
    idx->padding_0[24] = idx->padding_0[25];
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: parse_fls @ 0x14e42
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_fls(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    long long v0;  // [bp-0x18]

    if ((char)sub_4145ab(a1, a2, &v0))
    {
        if ((char)sub_414dbf(a0, v0))
            return 1;
        *(a2) = *(a2) - 1;
    }
    return 0;
}



// Function: parse_follow @ 0x14ecc
long long parse_follow(long long a0, long long a1, long long a2)
{
    sub_4141b0(1);
    return sub_417bae(a0, a1, a2, a1);
}



// Function: parse_fprint @ 0x14f07
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_fprint(struct_0 *a0, unsigned long long a1, int *a2)
{
    long long v0;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    idx = sub_40eecc(a0, v0);
    sub_419964(v0, idx + 1, idx + 1);
    idx->padding_0[25] = 1;
    idx->padding_0[24] = idx->padding_0[25];
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: insert_fprint @ 0x14fd8
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int insert_fprint(struct_0 *a0, long long a1)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_40eecc(a0, a1);
    if (a1)
        sub_419964(a1, idx + 1, idx + 1);
    else
        sub_419a85(idx + 1);
    idx->padding_0[25] = 1;
    idx->padding_0[24] = idx->padding_0[25];
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: parse_fprint0 @ 0x15079
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_fprint0(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    long long v0;  // [bp-0x18]

    if ((char)sub_4145ab(a1, a2, &v0))
    {
        if ((char)sub_414fd8(a0, v0))
            return 1;
        *(a2) = *(a2) - 1;
    }
    return 0;
}



// Function: estimate_fstype_success_rate @ 0x15103
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
    unsigned long v0;  // [bp-0xb8]
    void* v1;  // [bp-0xb0]
    stat v2;  // [bp-0xa8]

    v0 = "/";
    if (stat(v0, &v2))
        return;
    v1 = sub_413a18(&v2, v0);
    if (strcmp(a0, v1))
        return;
    return;
}



// Function: parse_fstype @ 0x151c2
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned long long g_449980;
extern unsigned short g_4499a8;

unsigned int parse_fstype(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v3;  // ymm0
    char *v0;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    if (g_4499a8 > 1 && !(char)sub_413aa6(v0))
    {
        if (!((char)g_449980 & 8))
            return sub_414cc0();
        fprintf(stderr, "-fstype %s can never succeed, substituting -false\n", v0);
        return sub_414cc0();
    }
    idx = sub_40eecc(a0, *((unsigned int *)&v0));
    *((char **)&idx[1].padding_0[0]) = v0;
    sub_415103(v0);
    *((unsigned int *)&idx->padding_0[36]) = v3;
    return 1;
}



// Function: parse_gid @ 0x152b9
typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[24];
    unsigned long long field_40;
} struct_0;

unsigned int parse_gid(long long a0, long long a1, unsigned int *a2)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_4197bf(a1, a2, a0, a2);
    if (!v0)
    {
        *(a2) = *(a2) - 1;
        return 0;
    }
    v0->field_24 = (v0->field_40 <= 99 ? 1065185444 : 1045220557);
    return 1;
}



// Function: parse_group @ 0x15339
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern unsigned int g_4499ac;

unsigned int parse_group(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v0;  // [bp-0x44]
    unsigned int v1;  // [bp-0x40]
    unsigned int v2;  // [bp-0x3c]
    char *v3;  // [bp-0x38]
    group *v4;  // [bp-0x30]
    struct_0 *idx;  // [bp-0x28]

    v1 = *(a2);
    if (!(char)sub_4145ab(a1, a2, &v3))
        return 0;
    v4 = getgrnam(v3);
    endgrent();
    if (v4)
    {
        v0 = v4->gr_gid;
    }
    else
    {
        v2 = strspn(v3, "0123456789");
        if (v2)
        {
            if (!v3[v2])
            {
                v0 = sub_41cd12(v3, g_4499ac, g_4499ac);
            }
            else
            {
                sub_42f861(1, g_4499ac, &v3[v2]);
                sub_42f861(0, g_4499ac, v3);
                error(1, 0, gettext("%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s"));
            }
        }
        else
        {
            if (*(v3))
            {
                sub_42f861(0, g_4499ac, v3);
                error(1, 0, gettext("%s is not the name of an existing group"));
            }
            error(1, 0, gettext("argument to -group is empty, but should be a group name"));
        }
    }
    idx = sub_40eecc(a0, *((unsigned int *)&v3));
    *((unsigned int *)&idx[1].padding_0[0]) = v0;
    *((unsigned int *)&idx->padding_0[36]) = (*((long long *)&idx[1].padding_0[8]) <= 99 ? 1065185444 : 1045220557);
    return 1;
}



// Function: parse_help @ 0x15554
void parse_help(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    sub_40f071(0); /* do not return */
}



// Function: estimate_pattern_match_rate @ 0x15576
char * estimate_pattern_match_rate(char *a0, unsigned int a1)
{
    char *ptr;  // rax

    ptr = strpbrk(a0, "*?[");
    if (!ptr && (!a1 || !(ptr = strpbrk(a0, "."), ptr)))
    {
        return ptr;
    }
    return ptr;
}



// Function: parse_ilname @ 0x155d9
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_ilname(struct_0 *a0, unsigned long long a1, int *a2)
{
    int v3;  // xmm0
    unsigned long long v0;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    idx = sub_40eecc(a0, v0);
    *((unsigned long long *)&idx[1].padding_0[0]) = v0;
    sub_415576(v0, 0);
    *((unsigned int *)&idx->padding_0[36]) = *((unsigned int *)&MulV(v3, 1036831949));
    return 1;
}



// Function: fnmatch_sanitycheck @ 0x15686
extern char g_44a1d8;

int fnmatch_sanitycheck(void)
{
    if (g_44a1d8 != 1)
    {
        if (fnmatch("foo", "foo", 0) || !fnmatch("Foo", "foo", 0) || fnmatch("Foo", "foo", 16))
            error(1, 0, gettext("sanity check of the fnmatch() library function failed."));
        g_44a1d8 = 1;
    }
    return g_44a1d8;
}



// Function: check_name_arg @ 0x1573c
unsigned int check_name_arg(long long a0, long long a1, char *a2)
{
    if (!(char)sub_4142a1())
        return 1;
    if (!strchr(a2, 47))
        return 1;
    sub_41096a(2, a1);
    sub_41096a(1, "/");
    sub_41096a(0, a0);
    error(0, 0, gettext("warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?"));
    return 1;
}



// Function: parse_iname @ 0x157fa
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_iname(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v3;  // ymm0
    char *v0;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    sub_415686();
    if ((char)sub_4145ab(a1, a2, &v0) && (char)sub_41573c("-iname", "-iwholename", v0))
    {
        idx = sub_40eecc(a0, *((unsigned int *)&v0));
        idx->padding_0[27] = 0;
        idx->padding_0[26] = idx->padding_0[27];
        *((char **)&idx[1].padding_0[0]) = v0;
        sub_415576(v0, 0);
        *((unsigned int *)&idx->padding_0[36]) = v3;
        return 1;
    }
    return 0;
}



// Function: parse_inum @ 0x158e1
typedef struct struct_0 {
    char padding_0[26];
    char field_1a;
    char field_1b;
    char field_1c;
    char padding_1d[7];
    unsigned int field_24;
} struct_0;

unsigned int parse_inum(long long a0, long long a1, unsigned int *a2)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_4197bf(a1, a2, a0, a2);
    if (!idx)
    {
        *(a2) = *(a2) - 1;
        return 0;
    }
    idx->field_24 = 897988541;
    idx->field_1c = 1;
    idx->field_1a = 0;
    idx->field_1b = 0;
    return 1;
}



// Function: parse_iregex @ 0x15961
extern char g_400000;
extern unsigned int g_44999c;

long long parse_iregex(long long a0, long long a1, long long a2)
{
    return sub_41723e(a1, a2, a0, g_44999c | &g_400000);
}



// Function: parse_links @ 0x1599c
typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[24];
    unsigned long long field_40;
} struct_0;

unsigned int parse_links(long long a0, long long a1, unsigned int *a2)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_4197bf(a1, a2, a0, a2);
    if (!idx)
    {
        *(a2) = *(a2) - 1;
        return 0;
    }
    if (idx->field_40 == 1)
    {
        idx->field_24 = 1065185444;
    }
    else if (idx->field_40 == 2)
    {
        idx->field_24 = 1008981770;
    }
    else
    {
        idx->field_24 = 981668463;
    }
    return 1;
}



// Function: parse_lname @ 0x15a46
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_lname(struct_0 *a0, unsigned long long a1, int *a2)
{
    int v3;  // xmm0
    unsigned long long v0;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    sub_415686();
    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    idx = sub_40eecc(a0, v0);
    *((unsigned long long *)&idx[1].padding_0[0]) = v0;
    sub_415576(v0, 0);
    *((unsigned int *)&idx->padding_0[36]) = *((unsigned int *)&MulV(v3, 1036831949));
    return 1;
}



// Function: parse_ls @ 0x15af8
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_ls(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a2;
    return sub_414dbf(a0, 0);
}



// Function: insert_depthspec @ 0x15b23
extern unsigned int g_4499ac;

unsigned long long insert_depthspec(long long a0, unsigned long a1, int *a2, int *a3)
{
    unsigned int v0;  // [bp-0x34]
    char *v1;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28]

    v2 = *((long long *)(a1 + *(a2) * 8 - 8));
    if (!(char)sub_4145ab(a1, a2, &v1))
        return 0;
    v0 = strspn(v1, "0123456789");
    if (v0 > 0 && !v1[v0])
    {
        *(a3) = sub_41cd12(v1, g_4499ac, g_4499ac);
        if (*(a3) >= 0)
            return sub_417bae(a0, a1, a2, a1);
    }
    sub_42f861(0, g_4499ac, v1);
    error(1, 0, gettext("Expected a positive decimal integer argument to %s, but got %s"));
}



// Function: parse_maxdepth @ 0x15c63
extern int g_449944;

unsigned long long parse_maxdepth(long long a0, unsigned int a1, int *a2)
{
    return sub_415b23(a0, a1, a2, &g_449944);
}



// Function: parse_mindepth @ 0x15c98
extern int g_449948;

unsigned long long parse_mindepth(long long a0, unsigned int a1, int *a2)
{
    return sub_415b23(a0, a1, a2, &g_449948);
}



// Function: do_parse_xmin @ 0x15ccd
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern unsigned long long g_449968;
extern unsigned long long g_449970;

unsigned int do_parse_xmin(struct_0 *a0, unsigned long long a1, unsigned int *a2, unsigned int a3)
{
    unsigned int v0;  // [bp-0x4c]
    long long v1;  // [bp-0x48]
    struct_0 *idx;  // [bp-0x40]
    unsigned long v3;  // [bp-0x38]
    unsigned long v4;  // [bp-0x30]
    unsigned long v5;  // [bp-0x28]
    unsigned int v6;  // [bp-0x20]
    unsigned long v7;  // [bp-0x18]

    v0 = *(a2);
    if (!(char)sub_4145ab(a1, a2, &v1))
        return 0;
    v3 = g_449968;
    v4 = g_449970;
    *((unsigned int *)&v5) = a3;
    v3 += 86400;
    if ((char)sub_41916a(v1, &v5, v3, v4, "arithmetic overflow while converting %s minutes to a number of seconds"))
    {
        idx = sub_40eecc(a0, v1);
        *((unsigned long *)&idx[1].padding_0[0]) = v5;
        *((unsigned long long *)&idx[1].padding_0[8]) = v6;
        *((unsigned long *)&idx[1].padding_0[16]) = v7;
        sub_4144ce(v6);
        *((unsigned int *)&idx->padding_0[36]) = 0;
        return 1;
    }
    *(a2) = v0;
    return 0;
}



// Function: parse_amin @ 0x15df0
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_amin(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    return sub_415ccd(a0, a1, a2, 0);
}



// Function: parse_cmin @ 0x15e23
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_cmin(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    return sub_415ccd(a0, a1, a2, 2);
}



// Function: parse_mmin @ 0x15e56
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_mmin(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    return sub_415ccd(a0, a1, a2, 3);
}



// Function: parse_name @ 0x15e89
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_name(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v4;  // ymm0
    unsigned int v0;  // [bp-0x24]
    char *v1;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    v0 = *(a2);
    if (!(char)sub_4145ab(a1, a2, &v1))
        return 0;
    sub_415686();
    if ((char)sub_41573c("-name", "-wholename", v1))
    {
        idx = sub_40eecc(a0, *((unsigned int *)&v1));
        idx->padding_0[27] = 0;
        idx->padding_0[26] = idx->padding_0[27];
        *((char **)&idx[1].padding_0[0]) = v1;
        sub_415576(v1, 0);
        *((unsigned int *)&idx->padding_0[36]) = v4;
        return 1;
    }
    *(a2) = v0;
    return 0;
}



// Function: parse_negate @ 0x15f82
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_negate(unsigned int *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40e7dd(a0, 0);
    *((void* *)&idx->padding_0[0]) = sub_409b30;
    *((unsigned int *)&idx->padding_0[16]) = 2;
    *((unsigned int *)&idx->padding_0[20]) = 4;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    return 1;
}



// Function: parse_newer @ 0x15ff1
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_newer(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned long v5;  // rdx
    unsigned int v6;  // ymm0
    long long v0;  // [bp-0xc8]
    struct_0 *idx;  // [bp-0xc0]
    char v2;  // [bp-0xb8]
    unsigned int v3;  // [bp-0x60]

    sub_40f26e(&v2);
    if (!(char)sub_414611(a1, a2, &v2, &v0))
        return 0;
    idx = sub_40eecc(a0, v0);
    *((unsigned long long *)&idx[1].padding_0[8]) = sub_430a80(&v2);
    *((unsigned long *)&idx[1].padding_0[16]) = v5;
    *((unsigned int *)&idx[1].padding_0[0]) = 3;
    *((unsigned int *)&idx[1].padding_0[4]) = 0;
    sub_4144ce(v3);
    *((unsigned int *)&idx->padding_0[36]) = v6;
    return 1;
}



// Function: parse_newerXY @ 0x160f6
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern long long g_449958;
extern struct_1 *g_449990;
extern unsigned int g_4499ac;

unsigned int parse_newerXY(struct_0 *a0, unsigned long long a1, int *a2)
{
    int *err;  // rax
    unsigned int v11;  // ymm0
    char v0;  // [bp-0xc2]
    char v1;  // [bp-0xc1]
    struct_0 *idx;  // [bp-0xc0]
    char v3;  // [bp-0xb8]
    char v4[6];  // [bp-0x26]

    if (!a1 || !*((long long *)(a1 + *(a2) * 8)))
        return 0;
    if (strlen(*((long long *)(a1 + *(a2) * 8))) != 8)
        return 0;
    strncpy(v4, "aBcmt", 5);
    if (strncmp("-newer", *((long long *)(a1 + *(a2) * 8)), 6))
        __assert_fail(); /* do not return */
    v0 = *((char *)(*((long long *)(a1 + *(a2) * 8)) + 6));
    v1 = *((char *)(*((long long *)(a1 + *(a2) * 8)) + 7));
    if (v0 != 66 && v1 != 66)
    {
        if (v0 == 116 || !strchr(v4, v0) || !strchr(v4, v1))
            return 0;
        if (!*((long long *)(a1 + (*(a2) + 1) * 8)))
        {
            sub_42f861(0, g_4499ac, *((long long *)(a1 + *(a2) * 8)));
            error(1, 0, gettext("The %s test needs an argument"));
        }
        *(a2) = *(a2) + 1;
        idx = sub_40eecc(a0, *((long long *)(a1 + *(a2) * 8)));
        switch (v0)
        {
        case 109:
            *((unsigned int *)&idx[1].padding_0[0]) = 3;
            break;
        case 99:
            *((unsigned int *)&idx[1].padding_0[0]) = 2;
            break;
        case 66:
            *((unsigned int *)&idx[1].padding_0[0]) = 1;
            break;
        case 97:
            *((unsigned int *)&idx[1].padding_0[0]) = 0;
            break;
        default:
            if (!strchr(v4, v0))
                __assert_fail(); /* do not return */
            __assert_fail(); /* do not return */
        }
        if (v1 != 116)
        {
            sub_40f26e(&v3);
            if (g_449990(*((long long *)(a1 + *(a2) * 8)), &v3, &v3, g_449990))
            {
                err = __errno_location();
                sub_410a44(*(err), *((long long *)(a1 + *(a2) * 8))); /* do not return */
            }
            if (!(unsigned int)sub_4140b3(&v3, v1, &idx[1].padding_0[8]))
            {
                sub_41096a(0, *((long long *)(a1 + *(a2) * 8)));
                error(1, 0, gettext("Cannot obtain birth time of file %s"));
            }
        }
        else if ((char)sub_42dcb0(&idx[1].padding_0[8], *((long long *)(a1 + *(a2) * 8)), &g_449958, &idx[1].padding_0[8]) != 1)
        {
            sub_42f861(0, g_4499ac, *((long long *)(a1 + *(a2) * 8)));
            error(1, 0, gettext("I cannot figure out how to interpret %s as a date or time"));
        }
        *((unsigned int *)&idx[1].padding_0[4]) = 0;
        sub_4144ce(*((long long *)&idx[1].padding_0[8]));
        *((unsigned int *)&idx->padding_0[36]) = v11;
        *(a2) = *(a2) + 1;
        if (!*((long long *)&idx->padding_0[0]))
        {
            __assert_fail(); /* do not return */
        }
        else if (*((long long *)&idx->padding_0[0]) != sub_409c02)
        {
            __assert_fail(); /* do not return */
        }
        else if (!idx->padding_0[26])
        {
            __assert_fail(); /* do not return */
        }
        else
        {
            return 1;
        }
        return 0;
    }
    error(0, 0, gettext("This system does not provide a way to find the birth time of a file."));
    return 0;
    return 0;
}



// Function: parse_noleaf @ 0x1674d
extern char g_44994c;

long long parse_noleaf(long long a0, long long a1, long long a2)
{
    g_44994c = 1;
    return sub_417bae(a0, a1, a2, a1);
}



// Function: parse_nogroup @ 0x16785
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_nogroup(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *v2;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    v2 = sub_40eecc(a0, 0);
    *((unsigned int *)&v2->padding_0[36]) = 953267991;
    return 1;
}



// Function: parse_nouser @ 0x167ca
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_nouser(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *v2;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    v2 = sub_40ef32(a0);
    *((unsigned int *)&v2->padding_0[36]) = 981668463;
    return 1;
}



// Function: parse_nowarn @ 0x1680a
extern char g_449950;

long long parse_nowarn(long long a0, long long a1, long long a2)
{
    g_449950 = 0;
    return sub_417bae(a0, a1, a2, a1);
}



// Function: parse_ok @ 0x16842
int parse_ok(long long a0, long long a1, long long a2)
{
    return sub_41893a("-ok", a0, a1, a2);
}



// Function: parse_okdir @ 0x1687a
int parse_okdir(long long a0, long long a1, long long a2)
{
    return sub_41893a("-okdir", a0, a1, a2);
}



// Function: parse_openparen @ 0x168b2
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_openparen(unsigned int *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40e7dd(a0, 0);
    *((void* *)&idx->padding_0[0]) = sub_409f83;
    *((unsigned int *)&idx->padding_0[16]) = 4;
    *((unsigned int *)&idx->padding_0[20]) = 0;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    return 1;
}



// Function: parse_or @ 0x16921
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_or(long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40e644(a0);
    *((void* *)&idx->padding_0[0]) = sub_409f9e;
    *((unsigned int *)&idx->padding_0[16]) = 3;
    *((unsigned int *)&idx->padding_0[20]) = 2;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    return 1;
}



// Function: is_feasible_path_argument @ 0x1698b
typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

unsigned int is_feasible_path_argument(char *a0, char a1)
{
    struct_0 *ptr;  // [bp-0x10]

    ptr = strrchr(a0, 47);
    if (ptr && !ptr->field_1)
        return (!(char)sub_40bb9d(a0, a1) ? 0 : 1);
    return 1;
}



// Function: insert_path_check @ 0x169f6
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern char g_449951;

unsigned int insert_path_check(struct_0 *a0, unsigned long long a1, int *a2, unsigned long long a3, unsigned int a4)
{
    unsigned int v5;  // ymm0
    unsigned long long v0;  // [bp-0x58]
    char v1;  // [bp-0x31]
    char *v2;  // [bp-0x30]
    struct_0 *idx;  // [bp-0x28]

    v0 = a3;
    v1 = 0;
    if (a4 == sub_40986a)
        v1 = 1;
    sub_415686();
    if (!(char)sub_4145ab(a1, a2, &v2))
        return 0;
    idx = sub_40ee5a(a0, a4, *((unsigned int *)&v2));
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    *((char **)&idx[1].padding_0[0]) = v2;
    sub_415576(v2, 0);
    *((unsigned int *)&idx->padding_0[36]) = v5;
    if (g_449951 == 1 || (char)sub_41698b(v2, v1) == 1)
        return 1;
    error(0, 0, gettext("warning: -%s %s will not match anything because it ends with /."));
    *((unsigned int *)&idx->padding_0[36]) = 841731191;
    return 1;
}



// Function: parse_path @ 0x16b4b
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;


unsigned int parse_path(struct_0 *a0, unsigned long long a1, int *a2)
{
    return sub_4169f6(a0, a1, a2, "path", 4235282);
}



// Function: parse_wholename @ 0x16b8a
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;


unsigned int parse_wholename(struct_0 *a0, unsigned long long a1, int *a2)
{
    return sub_4169f6(a0, a1, a2, "wholename", 4235282);
}



// Function: parse_ipath @ 0x16bc9
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;


unsigned int parse_ipath(struct_0 *a0, unsigned long long a1, int *a2)
{
    return sub_4169f6(a0, a1, a2, "ipath", 4233322);
}



// Function: parse_iwholename @ 0x16c08
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;


unsigned int parse_iwholename(struct_0 *a0, unsigned long long a1, int *a2)
{
    return sub_4169f6(a0, a1, a2, "iwholename", 4233322);
}



// Function: parse_perm @ 0x16c47
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern unsigned int g_4499ac;

unsigned int parse_perm(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v9;  // eax
    unsigned int v0;  // [bp-0x4c]
    int v1;  // [bp-0x48], Other Possible Types: unsigned int
    unsigned int v2;  // [bp-0x44]
    unsigned long long v3;  // [bp-0x40]
    unsigned long v4;  // [bp-0x38]
    unsigned int *idx;  // [bp-0x30]
    unsigned int v6;  // [bp-0x28]
    unsigned int v7;  // [bp-0x24]

    v1 = 0;
    v2 = 2;
    if ((char)sub_4145ab(a1, a2, &v3) != 1)
        return 0;
    v9 = *((char *)v3);
    switch (v9)
    {
    case 45:
        v1 = 1;
        v2 = 0;
        v0 = 1045220557;
        break;
    case 47:
        v1 = 1;
        v2 = 1;
        v0 = 1050253722;
        break;
    default:
        v1 = 0;
        v2 = 2;
        v0 = 1008981770;
        break;
    }
    v4 = sub_425a35(v1 + v3);
    if (v4 && (*((char *)v3) != 43 || *((char *)(v3 + 1)) <= 47 || *((char *)(v3 + 1)) > 55))
    {
        v6 = sub_425f2e(0, 0, 0, v4, 0);
        v7 = sub_425f2e(0, 1, 0, v4, 0);
        free(v4);
        if (*((char *)v3) == 47 && !v6 && !v7)
        {
            error(0, 0, gettext("warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files."));
            v2 = 0;
            v0 = 0x3f7fa440;
        }
        idx = sub_40eecc(a0, v3);
        idx[9] = v0;
        idx[14] = v2;
        memcpy(idx + 15, &v6, 8);
        return 1;
    }
    sub_42f861(0, g_4499ac, v3);
    error(1, 0, gettext("invalid mode %s"));
}



// Function: parse_print @ 0x16eb3
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_print(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    char *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = &sub_40ef32(a0)->padding_0[0];
    idx[25] = 1;
    idx[24] = idx[25];
    idx[27] = 0;
    idx[26] = idx[27];
    sub_419a85(idx + 56);
    return 1;
}



// Function: parse_print0 @ 0x16f20
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_print0(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a2;
    return sub_414fd8(a0, 0);
}



// Function: parse_printf @ 0x16f4b
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_printf(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v0;  // [bp-0x44]
    char *v1;  // [bp-0x40]
    unsigned long long v2[5];  // [bp-0x38]

    v0 = *(a2);
    if (!(char)sub_414528(a1, a2, &v1))
        return 0;
    sub_419a85(v2);
    if (!(char)sub_41159f(v2, a0, v1))
    {
        *(a2) = v0;
        return 0;
    }
    return 1;
}



// Function: parse_fprintf @ 0x16fef
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_fprintf(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    unsigned int v0;  // [bp-0x4c]
    unsigned long long v1;  // [bp-0x48]
    char *v2;  // [bp-0x40]
    unsigned long long v3[5];  // [bp-0x38]

    v0 = *(a2);
    if ((char)sub_4145ab(a1, a2, &v1) && (char)sub_414528(a1, a2, &v2))
    {
        sub_419964(v1, v3, v3);
        v0 = *(a2);
        if ((char)sub_41159f(v3, a0, v2))
            return 1;
    }
    *(a2) = v0;
    return 0;
}



// Function: parse_prune @ 0x170b7
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern char g_449940;

unsigned int parse_prune(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40ef32(a0);
    if (g_449940 != 1)
    {
        idx->padding_0[27] = 0;
        idx->padding_0[26] = idx->padding_0[27];
    }
    idx->padding_0[24] = 1;
    idx->padding_0[25] = 0;
    return 1;
}



// Function: parse_quit @ 0x1711b
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_quit(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40ef32(a0);
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    idx->padding_0[24] = 1;
    idx->padding_0[25] = 0;
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: parse_regextype @ 0x17182
extern unsigned int g_44999c;

unsigned long long parse_regextype(long long a0, unsigned long a1, unsigned long a2)
{
    long long v0;  // [bp-0x18]

    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    g_44999c = sub_41ca8f(v0);
    return sub_417bae(a0, a1, a2, a1);
}



// Function: parse_regex @ 0x1720a
extern unsigned int g_44999c;

long long parse_regex(long long a0, long long a1, long long a2)
{
    return sub_41723e(a1, a2, a0, g_44999c);
}



// Function: insert_regex @ 0x1723e
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int insert_regex(unsigned long long a0, int *a1, struct_0 *a2, int a3)
{
    unsigned long len;  // rax
    unsigned int v7;  // ymm0
    unsigned long long v0;  // [bp-0x40]
    struct_0 *idx;  // [bp-0x38]
    struct_0 *index;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x28]

    if (!(char)sub_4145ab(a0, a1, &v0))
        return 0;
    idx = sub_40ee5a(a2, sub_40a2da, v0);
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    index = sub_431bc2(64);
    *((struct_0 **)&idx[1].padding_0[0]) = index;
    *((long long *)&index->padding_0[8]) = 100;
    *((unsigned long long *)&index->padding_0[0]) = sub_431bc2(*((long long *)&index->padding_0[8]));
    *((unsigned long long *)&index->padding_0[32]) = 0;
    re_set_syntax(a3);
    *((long long *)&index->padding_0[24]) = a3;
    *((unsigned long long *)&index->padding_0[40]) = 0;
    len = strlen(v0);
    v3 = re_compile_pattern(v0, len, index, len);
    if (!v3)
    {
        sub_415576(v0, 1);
        *((unsigned int *)&idx->padding_0[36]) = v7;
        return 1;
    }
    error(1, 0, gettext("failed to compile regular expression '%s': %s"));
}



// Function: parse_size @ 0x173d3
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_size(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v11;  // eax
    char v0;  // [bp-0x4d]
    char v1;  // [bp-0x4c]
    int v2;  // [bp-0x48]
    unsigned int v3;  // [bp-0x44]
    char v4;  // [bp-0x40], Other Possible Types: unsigned long
    char *v5;  // [bp-0x38]
    struct_0 *idx;  // [bp-0x30]
    char v7;  // [bp-0x22]
    char v8;  // [bp-0x21]

    v2 = 0x200;
    if (!a1 || !*((long long *)(a1 + *(a2) * 8)))
        return 0;
    v5 = *((long long *)(a1 + *(a2) * 8));
    v3 = strlen(v5);
    if (!v3)
        error(1, 0, gettext("invalid null argument to -size"));
    v0 = *(&v5[v3] - 1);
    v11 = v0;
    switch (v11)
    {
    case 119:
        v2 = 2;
        *(&v5[v3] - 1) = 0;
        goto LABEL_417626;
        break;
    default:
        if (v11 <= 0x77)
        {
            if (v11 == 107)
            {
                v2 = 0x400;
                *(&v5[v3] - 1) = 0;
                goto LABEL_417626;
            }
            if (v11 <= 107)
            {
                if (v11 == 99)
                {
                    v2 = 1;
                    *(&v5[v3] - 1) = 0;
                    goto LABEL_417626;
                }
                if (v11 <= 99)
                {
                    if (v11 == 98)
                    {
                        v2 = 0x200;
                        *(&v5[v3] - 1) = 0;
                        goto LABEL_417626;
                    }
                    if (v11 <= 98)
                    {
                        if (v11 == 77)
                        {
                            v2 = 0x100000;
                            *(&v5[v3] - 1) = 0;
                            goto LABEL_417626;
                        }
                        if (v11 <= 77)
                        {
                            if (v11 <= 57)
                            {
                                if (v11 >= 48)
                                {
                                    v0 = 0;
LABEL_417626:
                                    if ((char)sub_419722(v5, &v4, &v1, &v4) != 1)
                                    {
                                        v7 = v0;
                                        v8 = 0;
                                        error(1, 0, gettext("Invalid argument `%s%s' to -size"));
                                    }
                                    idx = sub_40eecc(a0, *((unsigned int *)&v5));
                                    *((unsigned int *)&idx[1].padding_0[0]) = v1;
                                    *((int *)&idx[1].padding_0[4]) = v2;
                                    *((unsigned long long *)&idx[1].padding_0[8]) = v4;
                                    idx->padding_0[26] = 1;
                                    idx->padding_0[27] = 0;
                                    if (!v1)
                                    {
                                        *((unsigned int *)&idx->padding_0[36]) = (v2 * v4 <= 0x5000 ? 1063675494 : 1036831949);
                                    }
                                    else if (v1 == 1)
                                    {
                                        *((unsigned int *)&idx->padding_0[36]) = (v2 * v4 <= 0x5000 ? 1036831949 : 1063675494);
                                    }
                                    else
                                    {
                                        *((unsigned int *)&idx->padding_0[36]) = 1008981770;
                                    }
                                    *(a2) = *(a2) + 1;
                                    return 1;
                                }
                            }
                            else
                            {
                                if (v11 == 71)
                                {
                                    v2 = 0x40000000;
                                    *(&v5[v3] - 1) = 0;
                                    goto LABEL_417626;
                                }
                            }
                        }
                    }
                }
            }
        }
        error(1, 0, gettext("invalid -size type `%c'"));
    }
    return 0;
}



// Function: parse_samefile @ 0x1777e
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
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

extern unsigned int g_449988;
extern struct_0 *g_449990;
extern char g_449998;

unsigned int parse_samefile(struct_0 *a0, unsigned long long a1, int *a2)
{
    int *err;  // rax
    int *err1;  // rax
    int v0;  // [bp-0x160], Other Possible Types: unsigned int
    unsigned int v1;  // [bp-0x15c]
    long long v2;  // [bp-0x158]
    struct_0 *idx;  // [bp-0x150]
    char v4;  // [bp-0x148], Other Possible Types: unsigned long
    unsigned long v5;  // [bp-0x140]
    unsigned long v6;  // [bp-0x138]
    unsigned int v7;  // [bp-0x130], Other Possible Types: unsigned long
    unsigned long v8;  // [bp-0x128]
    unsigned long v9;  // [bp-0x120]
    unsigned long v10;  // [bp-0x118]
    unsigned long v11;  // [bp-0x110]
    unsigned long v12;  // [bp-0x108]
    unsigned long long v13;  // [bp-0x100]
    unsigned long long v14;  // [bp-0xf8]
    unsigned long long v15;  // [bp-0xf0]
    unsigned long long v16;  // [bp-0xe8]
    unsigned long long v17;  // [bp-0xe0]
    unsigned long long v18;  // [bp-0xd8]
    unsigned long long v19;  // [bp-0xd0]
    unsigned long long v20;  // [bp-0xc8]
    unsigned long long v21;  // [bp-0xc0]
    stat v22;  // [bp-0xb8]
    unsigned long long v23;  // [bp-0x70]
    unsigned long long v24;  // [bp-0x68]
    unsigned long long v25;  // [bp-0x60]
    unsigned long long v26;  // [bp-0x58]
    unsigned long long v27;  // [bp-0x50]
    unsigned long long v28;  // [bp-0x48]
    unsigned long long v29;  // [bp-0x40]
    unsigned long long v30;  // [bp-0x38]
    unsigned long long v31;  // [bp-0x30]

    sub_40f26e(&v4);
    if ((char)sub_414611(a1, a2, &v4, &v2) != 1)
        return 0;
    sub_40f26e(&v22);
    v0 = 0xfffffffd;
    v1 = 0;
    if (g_449988)
    {
        v0 = -0x1;
    }
    else if (g_449998)
    {
        v1 |= 0x20000;
        v0 = -0x1;
    }
    else
    {
        v0 = ((v7 & 0xf000) == 0xa000 ? -0x2 : -0x1);
    }
    if (v0 == -0x3)
        __assert_fail(); /* do not return */
    if (v0 == -0x1)
    {
        v0 = sub_41b433(v2, v1, v1);
        if (v0 >= 0)
        {
            if (fstat(v0, &v22))
            {
                err = __errno_location();
                sub_410a44(*(err), v2); /* do not return */
            }
            else if (g_449990(v2, &v4, &v4, g_449990))
            {
                err1 = __errno_location();
                sub_410a44(*(err1), v2); /* do not return */
            }
            else if (!(!g_449988 && g_449998 != 1))
            {
                v4 = *((unsigned long long *)&v22);
                v5 = v22.st_ino;
                v6 = v22.st_dev;
                v7 = *((unsigned long long *)((void*)&v22 + 24));
                v8 = *((unsigned long long *)((void*)&v22 + 32));
                v9 = v22.st_size;
                v10 = v22.st_atime;
                v11 = v22.st_mtime;
                v12 = v22.st_ctime;
                v13 = v23;
                v14 = v24;
                v15 = v25;
                v16 = v26;
                v17 = v27;
                v18 = v28;
                v19 = v29;
                v20 = v30;
                v21 = v31;
            }
            else if (((unsigned short)v7 & 0xf000) == 0xa000)
            {
                close(v0);
                v0 = -0x1;
            }
            else if (v4 != *((unsigned long long *)&v22) || v5 != v22.st_ino)
            {
                close(v0);
                v0 = -0x1;
            }
        }
    }
    idx = sub_40eecc(a0, v2);
    *((unsigned long *)&idx[1].padding_0[0]) = v5;
    *((unsigned long *)&idx[1].padding_0[8]) = v4;
    *((int *)&idx[1].padding_0[16]) = v0;
    idx->padding_0[27] = 0;
    idx->padding_0[26] = 1;
    *((unsigned int *)&idx->padding_0[36]) = 1008981770;
    return 1;
}



// Function: parse_true @ 0x17b57
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_true(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40ef32(a0);
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    *((unsigned int *)&idx->padding_0[36]) = 0x3f800000;
    return 1;
}



// Function: parse_noop @ 0x17bae
unsigned int parse_noop(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    return sub_417b57(sub_414033(), a1, a2);
}



// Function: parse_accesscheck @ 0x17be3
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_accesscheck(struct_0 *a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x10]

    v1 = a1;
    v0 = a2;
    idx = sub_40ef32(a0);
    idx->padding_0[27] = 0;
    idx->padding_0[26] = idx->padding_0[27];
    idx->padding_0[25] = 0;
    idx->padding_0[24] = idx->padding_0[25];
    if (*((long long *)&idx->padding_0[0]) != sub_40a0fa)
    {
        *((unsigned int *)&idx->padding_0[36]) = 1063675494;
        return 1;
    }
    *((unsigned int *)&idx->padding_0[36]) = 1045220557;
    return 1;
}



// Function: parse_type @ 0x17c77
long long parse_type(long long a0, long long a1, long long a2)
{
    return sub_4183bf(a1, a2, a0, sub_40a4b2);
}



// Function: parse_uid @ 0x17cac
typedef struct struct_0 {
    char padding_0[36];
    unsigned int field_24;
    char padding_28[24];
    unsigned long long field_40;
} struct_0;

unsigned int parse_uid(long long a0, long long a1, unsigned int *a2)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_4197bf(a1, a2, a0, a2);
    if (!v0)
    {
        *(a2) = *(a2) - 1;
        return 0;
    }
    v0->field_24 = (v0->field_40 <= 99 ? 1065185444 : 1045220557);
    return 1;
}



// Function: parse_used @ 0x17d2c
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_used(struct_0 *a0, unsigned long long a1, int *a2)
{
    long long v0;  // [bp-0x60]
    long long v1;  // [bp-0x58]
    struct_0 *idx;  // [bp-0x50]
    long long v3;  // [bp-0x48]
    long long v4;  // [bp-0x40]
    unsigned long v5;  // [bp-0x38]
    uint128_t v6;  // [bp-0x30]
    unsigned long v7;  // [bp-0x28]

    v1 = "arithmetic overflow while converting %s days to a number of seconds";
    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    v3 = 0;
    v4 = 0;
    if ((char)sub_41916a(v0, &v5, v3, v4, v1))
    {
        idx = sub_40eecc(a0, v0);
        *((unsigned long *)&idx[1].padding_0[0]) = v5;
        *((unsigned long long *)&idx[1].padding_0[8]) = v6;
        *((unsigned long *)&idx[1].padding_0[16]) = v7;
        sub_41446f();
        *((unsigned int *)&idx->padding_0[36]) = (unsigned long long)((int128_t)(v6 * 1749024623285053783) >> 77) - ((int128_t)(v6) >> 63);
        return 1;
    }
    error(1, 0, gettext("Invalid argument %s to -used"));
}



// Function: parse_user @ 0x17e8d
typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern unsigned int g_4499ac;

unsigned int parse_user(struct_0 *a0, unsigned long long a1, int *a2)
{
    unsigned int v0;  // [bp-0x44]
    char *v1;  // [bp-0x40]
    passwd *v2;  // [bp-0x38]
    unsigned long v3;  // [bp-0x30]
    struct_0 *idx;  // [bp-0x28]

    if (!(char)sub_4145ab(a1, a2, &v1))
        return 0;
    v2 = getpwnam(v1);
    endpwent();
    if (v2)
    {
        v0 = v2->pw_uid;
    }
    else
    {
        v3 = strspn(v1, "0123456789");
        if (v3 && !v1[v3])
        {
            v0 = sub_41cd12(v1, g_4499ac, g_4499ac);
        }
        else if (*(v1))
        {
            sub_42f861(0, g_4499ac, v1);
            error(1, 0, gettext("%s is not the name of a known user"));
        }
        else
        {
            error(1, 0, gettext("The argument to -user should not be empty"));
        }
    }
    idx = sub_40eecc(a0, *((unsigned int *)&v1));
    *((unsigned int *)&idx[1].padding_0[0]) = v0;
    *((unsigned int *)&idx->padding_0[36]) = (*((int *)&idx[1].padding_0[0]) <= 99 ? 1065185444 : 1045220557);
    return 1;
}



// Function: parse_version @ 0x1802c
extern char g_449998;
extern unsigned short g_4499a8;

void parse_version(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long v7;  // fs
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    long long v12;  // rcx
    long long v13;  // r8
    long long v14;  // r9
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]
    char v3;  // [bp-0x15]
    unsigned int v4;  // [bp-0x14]
    unsigned long v5;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a2;
    v5 = *((long long *)(40 + v7));
    v3 = 0;
    sub_41b6b6("find");
    printf(gettext("Features enabled: "));
    printf("D_TYPE ");
    v3 = 1;
    printf("O_NOFOLLOW(%s) ", (!g_449998 ? "disabled" : "enabled"));
    v3 = 1;
    printf("LEAF_OPTIMISATION ");
    v3 = 1;
    if (is_selinux_enabled(v9, v10, v11, v12, v13, v14) > 0)
    {
        printf("SELINUX ");
        v3 = 1;
    }
    v4 = 0;
    if ((char)sub_408b12(&v4))
    {
        printf("FTS(");
        v3 = 1;
        if ((unsigned short)v4 & 0x200)
            printf("FTS_CWDFD");
        printf(") ");
    }
    printf("CBO(level=%d) ", g_4499a8);
    v3 = 1;
    if (v3 != 1)
        printf("none");
    printf("\n");
    exit(0); /* do not return */
}



// Function: parse_context @ 0x181cd
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned int parse_context(struct_0 *a0, unsigned long long a1, int *a2)
{
    struct_0 *idx;  // [bp-0x10]

    if (a1 && *((long long *)(a1 + *(a2) * 8)))
    {
        if (is_selinux_enabled() <= 0)
            error(1, 0, gettext("invalid predicate -context: SELinux is not enabled."));
        idx = sub_40eecc(a0, 0);
        *((unsigned int *)&idx->padding_0[36]) = 1008981770;
        idx->padding_0[26] = 0;
        *((long long *)&idx[1].padding_0[0]) = *((long long *)(a1 + *(a2) * 8));
        *(a2) = *(a2) + 1;
        return 1;
    }
    return 0;
}



// Function: parse_xdev @ 0x182aa
extern char g_44994d;

unsigned int parse_xdev(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    g_44994d = 1;
    return sub_417bae(a0, a1, a2);
}



// Function: parse_ignore_race @ 0x182e2
extern char g_44994e;

unsigned int parse_ignore_race(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    g_44994e = 1;
    return sub_417bae(a0, a1, a2);
}



// Function: parse_noignore_race @ 0x1831a
extern char g_44994e;

unsigned int parse_noignore_race(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    g_44994e = 0;
    return sub_417bae(a0, a1, a2);
}



// Function: parse_warn @ 0x18352
extern char g_449950;

unsigned int parse_warn(unsigned long long a0, unsigned long long a1, unsigned long long a2)
{
    g_449950 = 1;
    return sub_417bae(a0, a1, a2);
}



// Function: parse_xtype @ 0x1838a
long long parse_xtype(long long a0, long long a1, long long a2)
{
    return sub_4183bf(a1, a2, a0, sub_40a78e);
}



// Function: insert_type @ 0x183bf
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;


unsigned int insert_type(unsigned long long a0, int *a1, struct_0 *a2, unsigned int a3)
{
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]
    unsigned long long iter;  // [bp-0x38]
    unsigned long long v3;  // [bp-0x30]
    struct_0 *idx;  // [bp-0x28]

    v3 = (a3 == sub_40a78e ? "-xtype" : "-type");
    if ((char)sub_4145ab(a0, a1, &iter) != 1)
    {
        return 0;
    }
    else if (!*((char *)iter))
    {
        error(1, 0, gettext("Arguments to %s should contain at least one letter"));
    }
    else
    {
        idx = sub_40ee5a(a2, a3, iter);
        *((unsigned int *)&idx->padding_0[36]) = 0;
        if (a3 == sub_40a78e)
        {
            idx->padding_0[26] = 1;
            idx->padding_0[27] = 0;
        }
        else
        {
            idx->padding_0[26] = 0;
            idx->padding_0[27] = 1;
        }
        while (1)
        {
            do
            {
                if (!*((char *)iter))
                    return 1;
                v1 = 1008981770;
                switch (*((char *)iter))
                {
                case 68:
                    v0 = 0;
                    error(1, 0, gettext("%s %c is not supported because Solaris doors are not supported on the platform find was compiled on."));
                case 98:
                    v0 = 0;
                    v1 = 979945644;
                    break;
                case 99:
                    v0 = 1;
                    v1 = 971522676;
                    break;
                case 100:
                    v0 = 2;
                    v1 = 1035785051;
                    break;
                case 102:
                    v0 = 3;
                    v1 = 0x3f600000;
                    break;
                case 108:
                    v0 = 4;
                    v1 = 1023329645;
                    break;
                case 112:
                    v0 = 5;
                    v1 = 922581086;
                    break;
                case 115:
                    v0 = 6;
                    v1 = 931487997;
                    break;
                default:
                    v0 = 0;
                    error(1, 0, gettext("Unknown argument to %s: %c"));
                }
                if (idx[1].padding_0[v0])
                    error(1, 0, gettext("Duplicate file type '%c' in the argument list to %s."));
                *((unsigned int *)&idx->padding_0[36]) = AddV(*((int *)&idx->padding_0[36]), v1);
                idx[1].padding_0[v0] = 1;
                iter += 1;
            } while (!*((char *)iter));
            if (*((char *)iter) != 44)
                error(1, 0, gettext("Must separate multiple arguments to %s using: ','"));
            iter += 1;
            if (!*((char *)iter))
                error(1, 0, gettext("Last file type in list argument to %s is missing, i.e., list is ending on: ','"));
        }
    }
}



// Function: stream_is_tty @ 0x1876f
typedef struct FILE {
} FILE;

unsigned int stream_is_tty(FILE *a0)
{
    unsigned long long v2;  // rax
    unsigned int v0;  // [bp-0xc]

    v0 = fileno(a0);
    if (v0 != 0xffffffff)
    {
        v2 = isatty(v0);
        return _INSERT(v2, 0, (unsigned int)v2);
    }
    return 0;
}



// Function: check_path_safety @ 0x187ac
void check_path_safety(unsigned long long a0)
{
    unsigned long long v0;  // [bp-0x50]
    char v1;  // [bp-0x48]
    char v2;  // [bp-0x40]
    unsigned long v3;  // [bp-0x38]
    long long v4;  // [bp-0x30]
    unsigned long ptr;  // [bp-0x28]

    v0 = a0;
    v3 = getenv("PATH");
    v4 = ":";
    if (!v3)
        return;
    sub_41cf56(v3, v4, 1, &v1, &v2);
    while (1)
    {
        if (!v2 || v2 == 1 && *((char *)(v3 + v1)) == 46)
            error(1, 0, gettext("The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)"));
        if (*((char *)(v3 + v1)) != 47)
            break;
        if (!(char)sub_41cf56(v3, v4, 0, &v1, &v2))
            return;
    }
    ptr = strndup(v3 + v1, v2);
    if (!ptr)
        ptr = v3 + v1;
    sub_41096a(0, ptr);
    error(1, 0, gettext("The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH"));
}



// Function: insert_exec_ok @ 0x1893a
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern char g_43bdf6;
extern char g_44994e;
extern unsigned int g_4499ac;
extern char g_4499b8;
extern unsigned long long g_449a00;

unsigned int insert_exec_ok(unsigned long long a0, struct_0 *a1, unsigned long long a2, int *a3)
{
    unsigned long len;  // rax
    char v0;  // [bp-0x91]
    int v1;  // [bp-0x90]
    int iter;  // [bp-0x8c], Other Possible Types: unsigned int
    unsigned int v3;  // [bp-0x88]
    int v4;  // [bp-0x84]
    int v5;  // [bp-0x80]
    unsigned int v6;  // [bp-0x7c]
    unsigned long v7;  // [bp-0x78]
    char *v8;  // [bp-0x70]
    unsigned long v9;  // [bp-0x68]
    void* ptr;  // [bp-0x60]
    void* idx;  // [bp-0x58]
    unsigned long v12;  // [bp-0x50]
    char v13[24];  // [bp-0x48]

    v9 = *((long long *)&a1->padding_0[24]);
    if (!a2 || !*((long long *)(a2 + *(a3) * 8)))
        return 0;
    ptr = sub_40ee5a(a1, v9, 4439373);
    *((char *)&ptr[25]) = 1;
    *((char *)&ptr[24]) = (char)ptr[25];
    *((char *)&ptr[26]) = 0;
    *((char *)&ptr[27]) = (char)ptr[26];
    idx = ptr + 56;
    *((unsigned long long *)&idx[192]) = 0;
    if (v9 != sub_409f27 && v9 != sub_409ece)
    {
        v0 = 1;
        *((char *)&idx[188]) = 0;
    }
    else
    {
        v0 = 0;
        g_4499b8 = 1;
        *((char *)&idx[188]) = 1;
    }
    if (v9 == sub_409497 || v9 == sub_409f27)
    {
        *((unsigned long long *)&idx[192]) = 0;
        g_44994e = 0;
        sub_4187ac(a0);
    }
    else
    {
        if (!g_449a00)
            __assert_fail(); /* do not return */
        *((unsigned long long *)&idx[192]) = g_449a00;
    }
    *((char *)&ptr[56]) = 0;
    v5 = *(a3);
    v1 = v5;
    v3 = 0;
    v4 = 0;
    v7 = 0;
    while (1)
    {
        if (!(*((long long *)(a2 + v1 * 8)) && (*((char *)*((long long *)(a2 + v1 * 8))) != 59 || *((char *)(*((long long *)(a2 + v1 * 8)) + 1)))))
        {
LABEL_418cc6:
            if (v1 == v5 || !*((long long *)(a2 + v1 * 8)))
            {
                *(a3) = v1;
                free(ptr);
                return 0;
            }
            if ((char)ptr[56])
            {
                v8 = (v9 == sub_409497 ? "dir" : &g_43bdf6);
                if (v4 > 1)
                    error(1, 0, gettext("Only one instance of {} is supported with -exec%s ... +"));
                if (strlen(v7) != 2)
                {
                    v12 = (int)snprintf(v13, 19, "-exec%s ... {} +", v8);
                    if (v12 > 19)
                        __assert_fail(); /* do not return */
                    sub_42f861(2, g_4499ac, v7);
                    sub_42f861(1, g_4499ac, "{}");
                    sub_42f861(0, g_4499ac, v13);
                    error(1, 0, gettext("In %s the %s must appear by itself, but you specified %s"));
                }
            }
            v6 = sub_41a58c(idx + 8, 0x800);
            if (v6 && v6 - 1 <= 1)
                error(1, 0, gettext("The environment is too large for exec()."));
            sub_41a6e3(idx + 8);
            *((void* *)&idx[72]) = sub_40b7ae;
            if ((char)ptr[56])
            {
                *((unsigned long long *)&idx[176]) = 0;
                *((long long *)&idx[56]) = 0;
                *((unsigned long *)&idx[48]) = 0;
                *((unsigned long long *)&idx[80]) = 0;
                *((unsigned long long *)&idx[88]) = 0;
                *((long long *)&idx[64]) = v1 - v5 - 1;
                sub_41a742(idx + 8, idx + 96, idx, idx + 8);
                for (iter = v5; iter < v1 - 1; iter += 1)
                {
                    len = strlen(*((long long *)(a2 + iter * 8)));
                    sub_41a1b8(idx + 8, idx + 96, *((long long *)(a2 + iter * 8)), len + 1, 0, 0, 1);
                }
            }
            else
            {
                *((int *)&idx[184]) = v1 - v5;
                *((char **)&idx[56]) = "{}";
                *((unsigned long *)&idx[48]) = strlen((long long)idx[56]);
                *((unsigned long long *)&idx[80]) = 0;
                *((unsigned long long *)&idx[88]) = 0;
                *((unsigned long long *)&idx[176]) = sub_431bc2((int)idx[184] * 8);
                sub_41a742(idx + 8, idx + 96, idx, idx + 8);
                for (iter = 0; iter < (int)idx[184]; iter += 1)
                {
                    *((long long *)((long long)idx[176] + iter * 8)) = *((long long *)((v5 + iter) * 8 + a2));
                }
            }
        }
        else if (v0 && *((char *)*((long long *)(a2 + v1 * 8))) == 43 && !*((char *)(*((long long *)(a2 + v1 * 8)) + 1)) && v3)
        {
            *((char *)&ptr[56]) = 1;
            goto LABEL_418cc6;
        }
        else
        {
            v3 = 0;
            if (sub_424cc3(*((long long *)(a2 + v1 * 8)), "{}", "{}"))
            {
                v3 = 1;
                v7 = *((long long *)(a2 + v1 * 8));
                v4 += 1;
                if (!v1 && (v9 == sub_409497 || v9 == sub_409f27))
                    error(1, 0, gettext("You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem."));
            }
            v1 += 1;
        }
    }
    if (!*((long long *)(a2 + v1 * 8)))
        *(a3) = v1;
    else
        *(a3) = v1 + 1;
    return 1;
    return 0;
}



// Function: get_relative_timestamp @ 0x1916a
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    long long field_8;
    unsigned long long field_10;
} struct_0;

extern char strtod;

unsigned int get_relative_timestamp(unsigned long a0, struct_0 *idx, long long a2, unsigned long a3, char *a4)
{
    unsigned int v5;  // eax
    unsigned long long v6;  // xmm0lq
    unsigned long v0;  // [bp-0x30]
    char v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]

    v0 = a0;
    if (!(char)sub_4196ab(&v0, &idx->field_4, &idx->field_4))
        return 0;
    v5 = idx->field_4;
    if (v5 != 2 && v5 <= 2)
    {
        if (!v5)
        {
            idx->field_4 = 1;
        }
        else if (v5 == 1)
        {
            idx->field_4 = 0;
        }
    }
    if (!(char)sub_43223a(v0, 0, &v1, &strtod))
        return 0;
    modf(&v2);
    v3 = MulV(v1, v6);
    v3 = MulV(0x41cdcd6500000000, v3);
    if (((CmpF(1000000000, v3) & 69 | (char)((CmpF(1000000000, v3) & 69) >> 6)) & 1) == 1)
        __assert_fail(); /* do not return */
    idx->field_8 = SubV(a2, v2);
    if (!((CmpF(0, v2) & 69 | (CmpF(0, v2) & 69) >> 6) & 1) ^ a2 < idx->field_8)
        error(1, 0, a4);
    idx->field_10 = SubV(a3, v3);
    if (((CmpF(v3, a3) & 69 | (char)((CmpF(v3, a3) & 69) >> 6)) & 1) == 1)
        return 1;
    idx->field_10 = idx->field_10 + 1000000000;
    idx->field_8 = idx->field_8 - 1;
    return 1;
}



// Function: parse_time @ 0x1938f
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    long long field_8;
    unsigned long long field_10;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned long long g_449968;
extern unsigned long long g_449970;
extern unsigned long long g_449980;

unsigned int parse_time(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    char *t;  // rax
    char *t1;  // rax
    char v0;  // [bp-0x70]
    unsigned int v1;  // [bp-0x6c]
    unsigned long long v2;  // [bp-0x68]
    unsigned int v3;  // [bp-0x60], Other Possible Types: unsigned long
    char *v4;  // [bp-0x58]
    unsigned long v5;  // [bp-0x50]
    unsigned long v6;  // [bp-0x48]
    struct_0 *idx;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    unsigned long v10;  // [bp-0x28]
    unsigned int v11;  // [bp-0x20]
    unsigned long v12;  // [bp-0x18]

    v4 = gettext("arithmetic overflow while converting %s days to a number of seconds");
    v1 = *(a2);
    if ((char)sub_4145ab(a1, a2, &v2) != 1)
        return 0;
    v5 = v2;
    v8 = g_449968;
    v9 = g_449970;
    if ((char)sub_4196ab(&v2, &v0, &v0) && v0 == 1)
    {
        v6 = v8 + 86399;
        v8 += 86399;
        if (v8 != v6)
            error(1, 0, gettext("arithmetic overflow when trying to calculate the end of today"));
    }
    v2 = v5;
    if ((char)sub_41916a(v2, &v10, v8, v9, v4) != 1)
    {
        *(a2) = v1;
        return 0;
    }
    idx = sub_40eecc(a0, v5);
    idx[2].field_8 = v10;
    idx[2].field_10 = v11;
    *((unsigned long *)idx[3].padding_0) = v12;
    sub_4144ce(v11);
    *((unsigned int *)((char *)&idx[1].field_8 + 4)) = 0;
    if (!((char)g_449980 & 1))
        return 1;
    fprintf(stderr, "inserting %s\n", idx->field_8);
    fprintf(stderr, "    type: %s    %s  ", (!*((unsigned int *)((void*)&v10 + 4)) ? "gt" : (*((unsigned int *)((void*)&v10 + 4)) == 1 ? "lt" : (*((unsigned int *)((void*)&v10 + 4)) == 2 ? "eq" : "?"))), (!*((unsigned int *)((void*)&v10 + 4)) ? " >" : (*((unsigned int *)((void*)&v10 + 4)) == 1 ? " <" : (*((unsigned int *)((void*)&v10 + 4)) == 2 ? ">=" : " ?"))));
    v3 = idx[2].field_10;
    t = ctime(&v3);
    fprintf(stderr, "%lu %s", idx[2].field_10, t);
    if (*((unsigned int *)((void*)&v10 + 4)) == 2)
    {
        v3 = idx[2].field_10 + 86400;
        t1 = ctime(&v3);
        fprintf(stderr, "                 <  %lu %s", v3, t1);
    }
    return 1;
}



// Function: get_comp_type @ 0x196ab
typedef struct struct_0 {
    char field_0;
} struct_0;

unsigned int get_comp_type(struct_0 **a0, unsigned int *a1)
{
    unsigned int v1;  // eax

    v1 = *(a0)->field_0;
    switch (v1)
    {
    case 43:
        *(a1) = 0;
        *(a0) = *(a0) + 1;
        return 1;
    case 45:
        *(a1) = 1;
        *(a0) = *(a0) + 1;
        return 1;
    default:
        *(a1) = 2;
        return 1;
    }
}



// Function: get_num @ 0x19722
extern long long g_43bdf6;

unsigned int get_num(char *a0, long long a1, unsigned int *a2)
{
    unsigned long long v3;  // rax
    char *v0;  // [bp-0x20]
    char v1;  // [bp-0x18]

    v0 = a0;
    if (!v0)
        return 0;
    if (a2 && (char)sub_4196ab(&v0, a2) != 1)
        return 0;
    v3 = sub_43291e(v0, &v1, 10, a1, &g_43bdf6);
    return _INSERT(v3, 0, !(unsigned int)v3);
}



// Function: insert_num @ 0x197bf
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned long long g_449980;

struct_0 * insert_num(unsigned long long a0, int *a1, struct_0 *a2)
{
    unsigned int v0;  // [bp-0x2c]
    char *v1;  // [bp-0x28]
    char v2;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    if ((char)sub_4145ab(a0, a1, &v1) && (char)sub_419722(v1, &v2, &v0))
    {
        idx = sub_40eecc(a2, *((unsigned int *)&v1));
        *((unsigned int *)&idx[1].padding_0[0]) = v0;
        *((unsigned long long *)&idx[1].padding_0[8]) = v2;
        if (!((char)g_449980 & 1))
            return idx;
        fprintf(stderr, "inserting %s\n", *((long long *)&idx->padding_0[8]));
        fprintf(stderr, "    type: %s    %s  ", (!v0 ? "gt" : (v0 == 1 ? "lt" : (v0 == 2 ? "eq" : "?"))), (!v0 ? " >" : (v0 == 1 ? " <" : (v0 == 2 ? " =" : " ?"))));
        fprintf(stderr, "%lu\n", *((long long *)&idx[1].padding_0[8]));
        return idx;
    }
    return NULL;
}



// Function: open_output_file @ 0x19964
extern unsigned long long stderr;
extern unsigned long long stdout;
extern long long *g_4499f0;

unsigned int open_output_file(unsigned long a0, void* idx)
{
    int *err;  // rax
    unsigned int v2;  // eax

    *((unsigned long long *)idx) = 0;
    *((unsigned long long *)&idx[32]) = sub_42de20(0);
    if (!strcmp(a0, "/dev/stderr"))
    {
        *((unsigned long long *)&idx[8]) = stderr;
        *((char **)&idx[16]) = gettext("standard error");
    }
    else if (!strcmp(a0, "/dev/stdout"))
    {
        *((unsigned long long *)&idx[8]) = stdout;
        *((char **)&idx[16]) = gettext("standard output");
    }
    else
    {
        *((FILE **)&idx[8]) = sub_410ce6(g_4499f0, a0);
        *((unsigned long *)&idx[16]) = a0;
        if (!(long long)idx[8])
        {
            err = __errno_location();
            sub_410aa8(*(err), a0); /* do not return */
        }
    }
    v2 = sub_41876f((long long)idx[8]);
    *((char *)&idx[24]) = v2;
    return v2;
}



// Function: open_stdout @ 0x19a85
unsigned int open_stdout(void* a0)
{
    return sub_419964("/dev/stdout", a0);
}



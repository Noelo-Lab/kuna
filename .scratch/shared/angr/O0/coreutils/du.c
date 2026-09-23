// Function: duinfo_init @ 0x40da
unsigned long long * duinfo_init(unsigned long long *idx)
{
    *(idx) = 0;
    idx[1] = 0;
    idx[2] = 0x8000000000000000;
    idx[3] = 0xffffffffffffffff;
    return idx;
}



// Function: duinfo_set @ 0x411a
long long duinfo_set(unsigned long long *idx, unsigned long a1, unsigned long a2, unsigned long a3)
{
    *(idx) = a1;
    idx[1] = 1;
    idx[2] = a2;
    idx[3] = a3;
    return a2;
}



// Function: duinfo_add @ 0x415c
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    long long field_10;
    long long field_18;
} struct_0;

long long duinfo_add(struct_0 *idx, struct_0 *index)
{
    unsigned long v2;  // rax
    long long v3;  // rdx
    long long v4;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = index->field_0 + idx->field_0;
    if (v0 < idx->field_0)
        v0 = 0xffffffffffffffff;
    idx->field_0 = v0;
    idx->field_8 = idx->field_8 + index->field_8;
    v2 = sub_41489d(idx->field_10, idx->field_18, index->field_10, index->field_18);
    if ((unsigned int)v2 >= 0)
        return v2;
    v3 = index->field_18;
    v4 = index->field_10;
    idx->field_10 = v4;
    idx->field_18 = v3;
    return v4;
}



// Function: usage @ 0x41ff
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
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    char *v21;  // rax
    char *v22;  // rax
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
        v7 = gettext("Summarize device usage of the set of FILEs, recursively for directories.\n");
        fputs_unlocked(v7, stdout);
        sub_403d79(v8, v9, v10, v11, v12, v13);
        v14 = gettext("  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in ('sparse') files, internal\n                          fragmentation, indirect blocks, and the like\n");
        fputs_unlocked(v14, stdout);
        v15 = gettext("  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           '-BM' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to '--apparent-size --block-size=1'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n");
        fputs_unlocked(v15, stdout);
        v16 = gettext("      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n");
        fputs_unlocked(v16, stdout);
        v17 = gettext("  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n");
        fputs_unlocked(v17, stdout);
        v18 = gettext("  -P, --no-dereference  don't follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n");
        fputs_unlocked(v18, stdout);
        v19 = gettext("  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in 'date'\n");
        fputs_unlocked(v19, stdout);
        v20 = gettext("  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n");
        fputs_unlocked(v20, stdout);
        v21 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v21, stdout);
        v22 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v22, stdout);
        sub_403ddb("DU");
        sub_403daa(v8, v9, v10, v11, v12, v13);
        sub_403e10("du");
    }
    exit(a0); /* do not return */
}



// Function: hash_ins @ 0x43fb
long long hash_ins(long long a0, long long a1, long long a2)
{
    unsigned int v0;  // [bp-0xc]

    v0 = sub_406bcf(a0, a2, a1, a2);
    if (v0 >= 0)
        return _INSERT(v0, 0, v0);
    sub_415992(); /* do not return */
}



// Function: show_date @ 0x4441
extern void stdout;

void show_date(long long a0, unsigned long a1, unsigned int a2, long long a3)
{
    unsigned long v0;  // [bp-0xa8]
    unsigned long v1;  // [bp-0x80]
    char v2;  // [bp-0x78]
    char v3;  // [bp-0x38]

    v0 = a1;
    if (!sub_414623(a3, &v0, &v2, &v0))
    {
        v1 = sub_4040a7(v0, &v3, &v3);
        sub_41401b(v1);
        error(0, 0, gettext("time %s is out of range"));
        fputs_unlocked(v1, *((long long *)&stdout));
        return;
    }
    sub_4080da(*((long long *)&stdout), a0, &v2, a3, a2);
    return;
}



// Function: print_only_size @ 0x4559
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_421148;
extern long long g_421170;

void print_only_size(long long a0)
{
    char *v2;  // rax
    char v0;  // [bp-0x2b8]

    v2 = (a0 == -0x1 ? gettext("Infinity") : sub_40f661(a0, &v0, g_421148, 1, g_421170));
    fputs_unlocked(v2, stdout);
    return;
}



// Function: print_size @ 0x45f2
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_42113c;
extern char g_42114c;
extern char g_42114d;
extern long long g_421160;
extern long long g_421168;

int print_size(struct_0 *a0, char *a1)
{
    sub_404559((!g_42114c ? a0->field_0 : a0->field_8));
    if (g_42114d)
    {
        putchar_unlocked(9);
        sub_404441(g_421160, *((long long *)&a0->field_10), a0->field_18, g_421168);
    }
    printf("\t%s%c", a1, (!g_42113c ? 10 : 0));
    return fflush_unlocked(stdout);
}



// Function: fill_mount_table @ 0x46a8
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char padding_10[24];
    char field_28;
    char padding_29[7];
    struct struct_0 *field_30;
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

extern long long g_421128;

void fill_mount_table(void)
{
    struct_0 *iter;  // [bp-0xb8]
    struct_0 *v1;  // [bp-0xb0]
    stat v2;  // [bp-0xa8]

    iter = sub_411107(0);
    while (iter)
    {
        if (!(iter->field_28 & 2) && !(iter->field_28 & 1) && !stat(iter->field_8, &v2))
            sub_4043fb(g_421128, v2.st_ino, *((unsigned long long *)&v2));
        v1 = iter;
        iter = iter->field_30;
        sub_411d9d(v1);
    }
    return;
}



// Function: mount_point_in_fts_cycle @ 0x4795
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
    char padding_10[96];
    long long field_70;
    long long field_78;
} struct_0;

extern long long g_421128;

unsigned long long mount_point_in_fts_cycle(struct_0 *a0)
{
    struct_0 *iter;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x10]

    iter = a0;
    v1 = (struct_0 *)iter->padding_0;
    if (!g_421128)
    {
        g_421128 = sub_406936();
        if (!g_421128)
            sub_415992(); /* do not return */
        sub_4046a8();
    }
    for (; iter && iter != v1; iter = iter->field_8)
    {
        if (sub_406c4b(g_421128, iter->field_70, iter->field_78, iter->field_70) > 0)
            return 1;
    }
    return 0;
}



// Function: process_file @ 0x4831
typedef struct struct_0 {
    char padding_0[72];
    unsigned int field_48;
} struct_0;

extern char g_421010;
extern long long g_421120;
extern unsigned long long g_421130;
extern char g_421138;
extern char g_421139;
extern char g_42113a;
extern char g_42113b;
extern char g_42113e;
extern char g_421140;
extern char g_42114c;
extern unsigned int g_421150;
extern long long g_421178;
extern struct_0 g_421180;
extern unsigned long long g_4211a0;
extern long long g_4211a8;

unsigned int process_file(struct_0 *a0, void* index, unsigned long a2)
{
    char *v19;  // rax
    char *v20;  // rax
    unsigned long long v22;  // rax
    unsigned long v23;  // rcx
    unsigned int v24;  // rdx
    char v0;  // [bp-0xa6]
    char v1;  // [bp-0xa5]
    unsigned int v2;  // [bp-0xa4]
    long long i;  // [bp-0xa0]
    unsigned long v4;  // [bp-0x98]
    void* idx;  // [bp-0x90]
    void* v6;  // [bp-0x88]
    void* v7;  // [bp-0x80]
    long long v8;  // [bp-0x78]
    unsigned long v9;  // [bp-0x70]
    char v10;  // [bp-0x68]
    unsigned long v11;  // [bp-0x60]
    unsigned long long v12;  // [bp-0x58]
    unsigned long long v13;  // [bp-0x50]
    unsigned long v14;  // [bp-0x48]
    unsigned long v15;  // [bp-0x40]
    unsigned long long v16;  // [bp-0x38]
    unsigned long long v17;  // [bp-0x30]

    v0 = 1;
    v4 = (long long)index[56];
    idx = index + 112;
    v2 = (short)index[104];
    switch (v2)
    {
    case 4:
        sub_413bd4(4, v4);
        v19 = gettext("cannot read directory %s");
        error(0, (int)index[64], v19);
        v0 = 0;
        break;
    case 6:
        v22 = (!g_421150 ? sub_4140c7(idx) : (g_421150 == 2 ? sub_414093(idx) : sub_4140ad(idx)));
        goto LABEL_404cb1;
    default:
        v1 = sub_4076f0(g_421178, v4, v4);
        if (v1 != 1)
        {
            if (v2 == 11)
            {
                sub_40c3cb(a0, index, 1, index);
                v6 = sub_40bd55(a0);
                if (v6 != index)
                    __assert_fail(); /* do not return */
                v2 = (short)index[104];
            }
            if (v2 == 10 || v2 == 13)
            {
                sub_413bd4(4, v4);
                v20 = gettext("cannot access %s");
                error(0, (int)index[64], v20);
                return 0;
            }
            else if ((char)a0->field_48 & 64 && (long long)index[88] > 0 && *((long long *)&a0->padding_0[24]) != *((long long *)idx))
            {
                v1 = 1;
            }
        }
        if (v1 || g_42113a != 1 && (g_42113b || ((short)(int)idx[24] & 0xf000) != 0x4000 && (long long)idx[16] > 1) && (char)sub_4043fb(g_421120, (long long)idx[8], *((long long *)idx)) != 1)
        {
            if (v2 != 1)
                return 1;
            sub_40c3cb(a0, index, 4, index);
            v7 = sub_40bd55(a0);
            if (v7 != index)
                __assert_fail(); /* do not return */
            return 1;
        }
        switch (v2)
        {
        case 7:
            sub_413d66(0, 3, v4);
            error(0, (int)index[64], "%s");
            v0 = 0;
            break;
        case 1:
            return 1;
        case 2:
            if (!(char)sub_415a4d(a0, index, index) || (char)sub_404795(index) == 1)
                return 1;
            sub_413d66(0, 3, v4);
            error(0, 0, gettext("WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n"));
            return 0;
            return 1;
        default:
            v22 = (!g_421150 ? sub_4140c7(idx) : (g_421150 == 2 ? sub_414093(idx) : sub_4140ad(idx)));
            break;
        }
    }
LABEL_404cb1:
    if (g_421139)
    {
        v23 = (long long)idx[48];
        if ((long long)idx[48] < 0)
            v23 = 0;
    }
    else
    {
        v23 = (long long)idx[64] * 0x200;
    }
    sub_40411a(&v10, v23, v22, v24);
    v8 = (long long)index[88];
    v14 = v10;
    v15 = v11;
    v16 = v12;
    v17 = v13;
    if (!g_4211a0)
    {
        g_4211a0 = v8 + 10;
        g_4211a8 = sub_415838(g_4211a0, 64);
    }
    else if (v8 != g_421130)
    {
        if (v8 > g_421130)
        {
            if (v8 >= g_4211a0)
            {
                g_4211a8 = sub_415307(g_4211a8, v8, 128, v8);
                g_4211a0 = v8 * 2;
            }
            for (i = g_421130 + 1; i <= v8; i += 1)
            {
                sub_4040da(i * 64 + g_4211a8);
                sub_4040da(i * 64 + g_4211a8 + 32);
            }
        }
        else
        {
            if (v8 != g_421130 - 1)
                __assert_fail(); /* do not return */
            sub_40415c(&v14, g_421130 * 64 + g_4211a8);
            if (g_42113e != 1)
                sub_40415c(&v14, g_421130 * 64 + g_4211a8 + 32);
            sub_40415c(v8 * 64 + g_4211a8 + 32, g_421130 * 64 + g_4211a8);
            sub_40415c(v8 * 64 + g_4211a8 + 32, g_421130 * 64 + g_4211a8 + 32);
        }
    }
    g_421130 = v8;
    if (g_42113e != 1 || v2 != 6 && v2 != 4)
        sub_40415c(v8 * 64 + g_4211a8, &v10);
    sub_40415c(&g_421180.padding_0[0], &v10);
    if ((v2 == 6 || v2 == 4) && v8 <= *((long long *)&g_421010) || g_421138 && v8 <= *((long long *)&g_421010) || !v8)
    {
        v9 = (!g_42114c ? v14 : v15);
        if ((*((long long *)&g_421140) < 0 ? v9 <= -(*((long long *)&g_421140)) : *((long long *)&g_421140) <= v9))
            sub_4045f2(&v14, v4);
    }
    return v0;
}



// Function: du_files @ 0x505b
typedef struct struct_0 {
    char padding_0[32];
    long long field_20;
} struct_0;

extern unsigned long long g_421130;

int du_files(unsigned long long *a0, unsigned int a1)
{
    char *v4;  // rax
    int *err;  // rax
    char *v6;  // rax
    int *err1;  // rax
    char v0;  // [bp-0x29]
    struct_0 *v1;  // [bp-0x28]
    void* v2;  // [bp-0x20]

    v0 = 1;
    if (*(a0))
    {
        v1 = sub_4159d2(a0, a1, 0, a1);
        while (1)
        {
            v2 = sub_40bd55(v1);
            if (!v2)
                break;
            v0 = sub_404831(v1, v2, v2) & 0xff & v0;
        }
        if (*(__errno_location()))
        {
            sub_413d66(0, 3, v1->field_20);
            v4 = gettext("fts_read failed: %s");
            err = __errno_location();
            error(0, *(err), v4);
            v0 = 0;
        }
        g_421130 = 0;
        if (sub_40b89b(v1))
        {
            v6 = gettext("fts_close failed");
            err1 = __errno_location();
            error(0, *(err1), v6);
            v0 = 0;
        }
    }
    return v0;
}



// Function: main @ 0x5197
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
    char padding_10[96];
    long long field_70;
    long long field_78;
} struct_0;

extern char g_41934b;
extern unsigned int g_4194a0[4];
extern unsigned int g_4194d0[4];
extern char g_420680;
extern long long g_4209c0;
extern long long g_420a00;
extern unsigned long long g_421010;
extern long long g_421018;
extern long long g_421020;
extern long long stdout;
extern void stdin;
extern int optind;
extern struct_0 *optarg;
extern FILE *stderr;
extern long long g_421120;
extern long long g_421128;
extern char g_421138;
extern char g_421139;
extern char g_42113a;
extern char g_42113b;
extern char g_42113c;
extern char g_42113d;
extern char g_42113e;
extern unsigned long long g_421140;
extern unsigned int g_421148;
extern char g_42114c;
extern char g_42114d;
extern unsigned int g_421150;
extern struct_0 *g_421158;
extern unsigned long long g_421160;
extern unsigned long long g_421168;
extern unsigned long long g_421170;
extern long long g_421178;
extern struct_0 g_421180;
extern unsigned long long g_4211b0;

int main(int a0, long long *a1)
{
    long long v21;  // rdi
    long long v22;  // rsi
    unsigned int *v35;  // eax
    char *v37;  // rax
    char *v38;  // rax
    int *err;  // rax
    char v40;  // al
    long long v23;  // rdx
    unsigned int v42;  // eax
    char *v43;  // rax
    int *err1;  // rax
    long long v24;  // rcx
    long long v25;  // r8
    long long v26;  // r9
    unsigned long v27;  // rax
    int *err2;  // rax
    char v0;  // [bp-0x94]
    char v1;  // [bp-0x93]
    char v2;  // [bp-0x92]
    char v3;  // [bp-0x91]
    int v4;  // [bp-0x90]
    unsigned int v5;  // [bp-0x8c]
    unsigned int v6;  // [bp-0x88]
    int v7;  // [bp-0x84]
    unsigned int v8;  // [bp-0x80]
    unsigned int v9;  // [bp-0x7c]
    char v10;  // [bp-0x78]
    char *v11;  // [bp-0x70]
    long long v12;  // [bp-0x68]
    unsigned long long v13;  // [bp-0x60]
    char *ptr;  // [bp-0x58]
    char *v15;  // [bp-0x50]
    char *v16;  // [bp-0x48]
    unsigned long long v17;  // [bp-0x40]
    unsigned long v18;  // [bp-0x38]
    unsigned long long v19;  // [bp-0x30]

    v0 = 0;
    v1 = 1;
    v11 = NULL;
    v5 = 8;
    v6 = 16;
    v2 = 0;
    v18 = sub_4040cc(".");
    v19 = 0;
    sub_411fda(*(a1));
    setlocale(6, &g_41934b);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_4186c0(sub_406762);
    g_421178 = sub_406ebe(v21, v22, v23, v24, v25, v26);
    v27 = getenv("DU_BLOCK_SIZE");
    sub_4102cd(v27, &g_421148, &g_421170, v27);
    while (1)
    {
        v4 = -0x1;
        v7 = getopt_long(a0, a1, "0abd:chHklmst:xB:DLPSX:", &g_420680, &v4);
        if (v7 == -0x1)
            break;
        if (v7 > 135)
            goto LABEL_4057be;
        if (v7 < 48)
        {
            if (v7 != -0x83)
            {
                if (v7 != -0x82)
                    goto LABEL_4057be;
                sub_4041ff(0); /* do not return */
            }
            sub_41517a(stdout, "du", "GNU coreutils", g_421018, "Torbjorn Granlund", "David MacKenzie", "Paul Eggert", "Jim Meyering", 0);
            exit(0); /* do not return */
        }
        switch (v7)
        {
        case 48:
            g_42113c = 1;
            break;
        case 66:
            v9 = sub_4102cd(optarg, &g_421148, &g_421170, &g_421148);
            if (v9)
                sub_4160c2(v9, v4, (char)v7, &g_420680, optarg); /* do not return */
            continue;
        case 68: case 72:
            v6 = 0x11;
            break;
        case 76:
            v6 = 2;
            break;
        case 80:
            v6 = 16;
            break;
        case 83:
            g_42113e = 1;
            break;
        case 88:
            if (sub_407dac(sub_4077e3, g_421178, optarg, 0x10000000, 10))
            {
                sub_413d66(0, 3, optarg);
                err2 = __errno_location();
                error(0, *(err2), "%s");
                v1 = 0;
            }
            break;
        case 97:
            g_421138 = 1;
            break;
        case 98:
            g_421139 = 1;
            g_421148 = 0;
            g_421170 = 1;
            break;
        case 99:
            g_42113d = 1;
            break;
        case 100:
            if (!sub_4161e3(optarg, 0, 0, &v10, &g_41934b))
            {
                v0 = 1;
                g_421010 = v10;
                break;
            }
            else
            {
                sub_41401b(optarg);
                error(0, 0, gettext("invalid maximum depth %s"));
                v1 = 0;
                break;
            }
        case 104:
            g_421148 = 176;
            g_421170 = 1;
            break;
        case 107:
            g_421148 = 0;
            g_421170 = 0x400;
            break;
        case 108:
            g_42113a = 1;
            break;
        case 109:
            g_421148 = 0;
            g_421170 = 0x100000;
            break;
        case 115:
            v2 = 1;
            break;
        case 116:
            v8 = sub_415b9a(optarg, 0, 0, &g_421140, "kKmMGTPEZY0");
            if (v8)
                sub_4160c2(v8, v4, (char)v7, &g_420680, optarg); /* do not return */
            if (!g_421140 && optarg->padding_0[0] == 45)
                error(1, 0, gettext("invalid --threshold argument '-0'"));
            continue;
        case 120:
            v5 |= 64;
            break;
        case 128:
            g_421139 = 1;
            break;
        case 129:
            sub_4077e3(g_421178, optarg, 0x10000000, optarg);
            break;
        case 130:
            v11 = &optarg->padding_0[0];
            break;
        case 131:
            g_421148 = 144;
            g_421170 = 1;
            break;
        case 133:
            g_42114d = 1;
            g_421150 = (unsigned int)(!optarg ? NULL : g_4194a0[sub_4063c9("--time", optarg, &g_4209c0, &g_4194a0[0], 4, g_421020, 1)]);
            g_421168 = sub_4141a3(getenv("TZ"));
            break;
        case 134:
            g_421158 = optarg;
            break;
        case 135:
            g_42114c = 1;
            break;
        default:
LABEL_4057be:
            v1 = 0;
            continue;
        }
    }
    if (v1 != 1)
        sub_4041ff(1); /* do not return */
    if (g_421138 && v2)
    {
        error(0, 0, gettext("cannot both summarize and show all entries"));
        sub_4041ff(1); /* do not return */
    }
    if (v2 && v0 && !g_421010)
        error(0, 0, gettext("warning: summarizing is the same as using --max-depth=0"));
    if (v2 && v0 && g_421010)
    {
        v13 = g_421010;
        error(0, 0, gettext("warning: summarizing conflicts with --max-depth=%lu"));
        sub_4041ff(1); /* do not return */
    }
    if (v2)
        g_421010 = 0;
    if (g_42114c)
    {
        if (g_421139)
            error(0, 0, gettext("warning: options --apparent-size and -b are ineffective with --inodes"));
        g_421170 = 1;
    }
    if (g_42114d)
    {
        if (!g_421158)
        {
            if (!g_421158 || !strcmp(g_421158, "locale"))
            {
                g_421158 = "long-iso";
            }
            else if (g_421158->padding_0[0] == 43)
            {
                ptr = strchr(g_421158, 10);
                if (ptr)
                    *(ptr) = 0;
            }
            else
            {
                for (g_421158 = getenv("TIME_STYLE"); !strncmp(g_421158, "posix-", 6); g_421158 = &g_421158->padding_0[6]);
            }
        }
        if (g_421158->padding_0[0] == 43)
        {
            g_421160 = &g_421158->padding_0[1];
        }
        else
        {
            v35 = (unsigned int *)g_4194d0[sub_4063c9("time style", g_421158, &g_420a00, &g_4194d0[0], 4, g_421020, 1)];
            if (v35 == 2)
            {
                g_421160 = "%Y-%m-%d";
            }
            else if (v35 <= 2)
            {
                if (!v35)
                {
                    g_421160 = "%Y-%m-%d %H:%M:%S.%N %z";
                }
                else if (v35 == 1)
                {
                    g_421160 = "%Y-%m-%d %H:%M";
                }
            }
        }
    }
    if (v11)
    {
        if (a0 > optind)
        {
            sub_41401b(a1[optind]);
            error(0, 0, gettext("extra operand %s"));
            v37 = gettext("file operands cannot be combined with --files0-from");
            fprintf(stderr, "%s\n", v37);
            sub_4041ff(1); /* do not return */
        }
        if (strcmp(v11, "-") && !sub_40abea(v11, "r", *((long long *)&stdin), "r"))
        {
            sub_413bd4(4, v11);
            v38 = gettext("cannot open %s for reading");
            err = __errno_location();
            error(1, *(err), v38);
        }
        v12 = sub_40655d(*((long long *)&stdin));
        g_42113b = 1;
    }
    else
    {
        v15 = (a0 <= optind ? &v18 : &a1[optind]);
        v12 = sub_406508(v15);
        if (a0 > optind + 1 || v6 == 2)
            v40 = 1;
        else
            v40 = 0;
        g_42113b = v40 & 1;
    }
    if (!v12)
        sub_415992(); /* do not return */
    g_421120 = sub_406936();
    if (!g_421120)
        sub_415992(); /* do not return */
    if (g_42113a || g_42113b != 1)
        v5 |= 0x100;
    v5 |= v6;
    while (1)
    {
        v3 = 0;
        v16 = sub_4065ca(v12, &v10, &v10);
        if (!v16)
            break;
        if (v11 && !strcmp(v11, "-") && !strcmp(v16, "-"))
        {
            sub_413bd4(4, v16);
            error(0, 0, gettext("when reading file names from stdin, no file name of %s allowed"));
            v3 = 1;
        }
        if (!*(v16))
        {
            if (!v11)
            {
                gettext("invalid zero-length file name");
                error(0, 0, "%s");
            }
            else
            {
                v17 = sub_4066b5(v12);
                gettext("invalid zero-length file name");
                sub_413d66(0, 3, v11);
                error(0, 0, "%s:%lu: %s");
            }
            v3 = 1;
        }
        if (v3)
        {
            v1 = 0;
        }
        else
        {
            g_4211b0 = v16;
            v1 = sub_40505b(&g_4211b0, v5) & 0xff & v1;
        }
    }
    v42 = *((unsigned int *)&v10);
    if (v42 != 4)
    {
        if (v42 <= 4)
        {
            if (v42 == 2)
                goto LABEL_405f1c;
            if (v42 == 3)
                sub_415992(); /* do not return */
        }
        __assert_fail(); /* do not return */
    }
    else
    {
        sub_413d66(0, 3, v11);
        v43 = gettext("%s: read error");
        err1 = __errno_location();
        error(0, *(err1), v43);
        v1 = 0;
LABEL_405f1c:
        sub_4066f3(v12);
        sub_4069c8(g_421120);
        if (g_421128)
            sub_4069c8(g_421128);
        if (v11 && (ferror_unlocked(*((long long *)&stdin)) || sub_407ea0(*((long long *)&stdin))) && v1)
        {
            sub_413bd4(4, v11);
            error(1, 0, gettext("error reading %s"));
        }
        if (!g_42113d)
            return v1 ^ 1;
        sub_4045f2(&g_421180.padding_0[0], gettext("total"));
        return v1 ^ 1;
    }
}



// Function: hash_get_n_buckets @ 0xd9a8
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0xd9be
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0xd9d4
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0xd9ea
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_get_max_bucket_length(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *iter;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v1 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v3 = 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                v3 += 1;
            }
            if (v3 > v1)
                v1 = v3;
        }
    }
    return v1;
}



// Function: hash_table_ok @ 0xda6c
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned int hash_table_ok(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long node;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v1 = 0;
    node = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v1 += 1;
            node += 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                node += 1;
            }
        }
    }
    if (v1 == a0[3] && node == a0[4])
        return 1;
    return 0;
}



// Function: hash_print_statistics @ 0xdb0a
typedef struct FILE {
} FILE;

int hash_print_statistics(void* a0, FILE *ptr)
{
    uint128_t v5;  // xmm0
    long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = sub_40d9d4(a0);
    v1 = sub_40d9a8(a0);
    v2 = sub_40d9be(a0);
    v3 = sub_40d9ea(a0);
    fprintf(ptr, "# entries:         %lu\n", v0);
    fprintf(ptr, "# buckets:         %lu\n", v1);
    v5 = (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1);
    fprintf(ptr, "# buckets used:    %lu (%.2f%%)\n", v2, (unsigned long long)(DivV(MulV(0x4059000000000000, (v2 < 0 ? AddV(v2 >> 1 | (unsigned int)v2 & 1, v2 >> 1 | (unsigned int)v2 & 1) : v2)), (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1))));
    return fprintf(ptr, "max bucket length: %lu\n", v3);
}



// Function: safe_hasher @ 0xdc51
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

long long safe_hasher(struct_0 *a0, unsigned long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0->field_30(a1, a0->field_10, a0->field_10, a0->field_30);
    if (v0 < a0->field_10)
        return v0 * 16 + a0->field_0;
    abort(); /* do not return */
}



// Function: hash_lookup @ 0xdcac
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_lookup(struct_0 *a0, unsigned long long a1)
{
    struct_0 *iter;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v1 = sub_40dc51(a0, a1);
    if (!v1->field_0)
        return 0;
    iter = v1;
    while (1)
    {
        if (!iter)
            return 0;
        if (a1 == iter->field_0 || (char)a0[1].field_0(a1, iter->field_0, iter->field_0, a0[1].field_0))
            break;
        iter = (struct_0 *)iter->padding_8;
    }
    return iter->field_0;
}



// Function: hash_get_first @ 0xdd43
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[24];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_get_first(struct_0 *a0)
{
    unsigned long long *v0;  // [bp-0x10]

    if (!a0->field_20)
        return 0;
    v0 = &a0->field_0->field_0;
    while (1)
    {
        if (v0 >= *((long long *)&a0->padding_8[0]))
            abort(); /* do not return */
        if (*(v0))
            return *(v0);
        v0 += 2;
    }
}



// Function: hash_get_next @ 0xdda3
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_get_next(struct_0 *a0, unsigned int a1)
{
    struct_0 *v0;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = sub_40dc51(a0, a1);
    iter = v0;
    while (1)
    {
        if (a1 == iter->field_0 && iter->padding_8)
            return *((long long *)iter->padding_8);
        iter = (struct_0 *)iter->padding_8;
        if (!iter)
        {
            while (1)
            {
                v0 = &v0->field_10;
                if (v0 >= a0->padding_8)
                {
                    return 0;
                }
                else if (v0->field_0)
                {
                    return v0->field_0;
                }
            }
        }
    }
}



// Function: hash_get_entries @ 0xde41
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_get_entries(struct_0 **a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long idx;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    idx = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; idx = v1)
            {
                if (idx >= a2)
                    return idx;
                v1 = idx + 1;
                *((unsigned long long *)(idx * 8 + a1)) = iter->field_0;
                iter = iter->field_8;
            }
        }
    }
    return idx;
}



// Function: hash_do_for_each @ 0xdee1
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_do_for_each(struct_0 **a0, unsigned long long *a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; iter = iter->field_8)
            {
                if ((char)a1(iter->field_0, a2, a2, a1) != 1)
                    return v0;
                v0 += 1;
            }
        }
    }
    return v0;
}



// Function: hash_string @ 0xdf79
unsigned long long hash_string(char *a0, unsigned long long a1)
{
    char *iter;  // [bp-0x20]
    char v1;  // [bp-0x11]
    unsigned long long v2;  // [bp-0x10]

    iter = a0;
    v2 = 0;
    while (1)
    {
        v1 = *(iter);
        if (!v1)
            break;
        v2 = (v1 + v2 * 31) % a1;
        iter += 1;
    }
    return v2;
}



// Function: is_prime @ 0xdfd3
long long is_prime(unsigned long long a0)
{
    unsigned long long iter;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    iter = 3;
    for (v1 = iter * iter; v1 < a0 && a0 % iter; iter += 1)
    {
        iter += 1;
        v1 += iter * 4;
    }
    return _INSERT(a0 % iter, 0, a0 % iter);
}



// Function: next_prime @ 0xe042
unsigned long long next_prime(unsigned long long a0)
{
    unsigned long long i;  // [bp-0x10]

    i = a0;
    if (i <= 9)
        i = 10;
    for (i |= 1; i != 0xffffffffffffffff; i += 2)
    {
        if ((char)sub_40dfd3(i) == 1)
            return i;
    }
    return i;
}



// Function: hash_reset_tuning @ 0xe08d
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
    char field_10;
} struct_0;

struct_0 * hash_reset_tuning(struct_0 *idx)
{
    idx->field_0 = 0;
    idx->field_4 = 0x3f800000;
    idx->field_8 = 1061997773;
    idx->field_c = 1068826100;
    idx->field_10 = 0;
    return idx;
}



// Function: raw_hasher @ 0xe0e3
long long raw_hasher(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_4175f6(a0, 3);
    return v0 % a1;
}



// Function: raw_comparator @ 0xe11e
long long raw_comparator(unsigned long a0, unsigned long a1)
{
    return _INSERT(a0, 0, a0 == a1);
}



// Function: check_tuning @ 0xe13b
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

extern unsigned int g_41aa40[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int v3;  // ymm1
    unsigned int v4;  // ymm1
    unsigned int v0;  // [bp-0x14]
    unsigned int *v1;  // [bp-0x10]

    v1 = &idx->field_28->field_0;
    if (v1 == &g_41aa40[0])
        return 1;
    v0 = 1036831949;
    if (((CmpF(v1[2], v0) & 69 | (char)((CmpF(v1[2], v0) & 69) >> 6)) & 1) != 1 && ((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69 | (char)((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69) >> 6)) & 1) != 1 && ((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69 | (char)((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69) >> 6)) & 1) != 1 && !(CmpF(*(v1), 0) & 1))
    {
        v3 = AddV(*(v1), v0);
        if (((CmpF(v1[1], v3) & 69 | (char)((CmpF(v1[1], v3) & 69) >> 6)) & 1) != 1 && !(CmpF(0x3f800000, v1[1]) & 1))
        {
            v4 = AddV(*(v1), v0);
            if (((CmpF(v1[2], v4) & 69 | (char)((CmpF(v1[2], v4) & 69) >> 6)) & 1) != 1)
                return 1;
        }
    }
    idx->field_28 = &g_41aa40[0];
    return 0;
}



// Function: compute_bucket_size @ 0xe248
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long compute_bucket_size(unsigned long a0, struct_0 *a1)
{
    char v3;  // al
    unsigned long v0;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xc]

    v0 = a0;
    if (a1->field_10 != 1)
    {
        v1 = DivV((v0 < 0 ? AddV((unsigned int)(v0 >> 1 | (unsigned int)v0 & 1), (unsigned int)(v0 >> 1 | (unsigned int)v0 & 1)) : (unsigned int)v0), a1->field_8);
        if (!(CmpF(v1, 0x5f800000) & 1))
            return 0;
        if (CmpF(v1, 0x5f000000) & 1)
        {
            v0 = v1;
        }
        else
        {
            v0 = (unsigned int)(SubV(v1, 0x5f000000));
            v0 ^= 0x8000000000000000;
        }
    }
    v0 = sub_40e042(v0);
    v3 = 0;
    if (v0 & 0xe000000000000000)
        v3 = 1;
    if (v0 * 8 < 0)
        v3 = 1;
    if (!(v3 & 1))
        return v0;
    return 0;
}



// Function: hash_initialize @ 0xe361
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

extern struct_0 g_41aa40;

struct_0 * hash_initialize(unsigned int a0, struct_0 *a1, unsigned long long a2, unsigned long long a3, unsigned long a4)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    struct_0 *v2;  // [bp-0x28]
    struct_0 *ptr;  // [bp-0x10]

    v2 = a1;
    v1 = a2;
    v0 = a3;
    if (!v1)
        v1 = sub_40e0e3;
    if (!v0)
        v0 = sub_40e11e;
    ptr = malloc(80);
    if (!ptr)
        return NULL;
    if (!v2)
        v2 = &g_41aa40.padding_0[0];
    ptr->field_28 = v2;
    if ((char)sub_40e13b(ptr) == 1)
    {
        *((unsigned long long *)&ptr->field_10) = sub_40e248(a0, v2);
        if (*((long long *)&ptr->field_10))
        {
            ptr->field_0 = calloc(*((long long *)&ptr->field_10), 16);
            if (ptr->field_0)
            {
                ptr->field_8 = ptr->field_0 + *((long long *)&ptr->field_10) * 16;
                ptr->field_18 = 0;
                ptr->field_20 = 0;
                ptr->field_30 = v1;
                ptr->field_38 = v0;
                ptr->field_40 = a4;
                ptr->field_48 = 0;
                return ptr;
            }
        }
    }
    free(ptr);
    return NULL;
}



// Function: hash_clear @ 0xe4d6
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[24];
    struct struct_3 *field_40;
    struct struct_0 *field_48;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

struct_2 * hash_clear(struct_2 *idx)
{
    struct_1 *iter;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    struct_0 *v2;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx1 = iter->field_8; idx1; idx1 = v2)
            {
                if (idx->field_40)
                    idx->field_40(idx1->field_0);
                idx1->field_0 = 0;
                v2 = idx1->field_8;
                idx1->field_8 = idx->field_48;
                idx->field_48 = idx1;
            }
            if (idx->field_40)
                idx->field_40(iter->field_0);
            iter->field_0 = 0;
            iter->field_8 = NULL;
        }
    }
    idx->field_18 = 0;
    idx->field_20 = 0;
    return idx;
}



// Function: hash_free @ 0xe5e1
typedef struct struct_1 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
    char padding_8[24];
    unsigned long long field_20;
    char padding_28[24];
    struct struct_3 *field_40;
    struct struct_1 *field_48;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

void hash_free(struct_2 *ptr)
{
    struct_0 *node;  // [bp-0x20]
    struct_1 *iter;  // [bp-0x18], Other Possible Types: struct_0 *
    struct_0 *v2;  // [bp-0x10], Other Possible Types: struct_1 *

    if (ptr->field_40 && ptr->field_20)
    {
        for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
        {
            if (node->field_0)
            {
                for (iter = node; iter; iter = iter->field_8)
                {
                    ptr->field_40(iter->field_0);
                }
            }
        }
    }
    for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
    {
        for (iter = node->field_8; iter; iter = v2)
        {
            v2 = iter->field_8;
            free(iter);
        }
    }
    for (iter = ptr->field_48; iter; iter = v2)
    {
        v2 = iter->field_8;
        free(iter);
    }
    free(ptr->field_0);
    free(ptr);
    return;
}



// Function: allocate_entry @ 0xe710
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_1;

struct_0 * allocate_entry(struct_1 *idx)
{
    struct_0 *ptr;  // [bp-0x10]

    if (!idx->field_48)
    {
        ptr = malloc(16);
        return ptr;
    }
    ptr = idx->field_48;
    idx->field_48 = ptr->field_8;
    return ptr;
}



// Function: free_entry @ 0xe75f
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_2;

struct_2 * free_entry(struct_2 *idx, struct_0 *a1)
{
    a1->field_0 = 0;
    a1->field_8 = idx->field_48;
    idx->field_48 = a1;
    return idx;
}



// Function: hash_find_entry @ 0xe799
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[56];
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_find_entry(struct_0 *a0, unsigned long long a1, struct_2 **a2, char a3)
{
    struct_2 *v7;  // rdx
    struct_2 *iter;  // [bp-0x38]
    struct_2 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    struct_2 *v5;  // [bp-0x10]

    index = sub_40dc51(a0, a1);
    *(a2) = index;
    if (!index->field_0)
        return 0;
    if (a1 != index->field_0 && !(char)a0->field_38(a1, index->field_0, index->field_0, a0->field_38))
    {
        iter = index;
        while (1)
        {
            if (!iter->field_8)
                return 0;
            if (a1 == iter->field_8->field_0 || (char)a0->field_38(a1, iter->field_8->field_0, iter->field_8->field_0, a0->field_38))
                break;
            iter = iter->field_8;
        }
        v2 = iter->field_8->field_0;
        if (!a3)
            return v2;
        v3 = iter->field_8;
        iter->field_8 = v3->field_8;
        sub_40e75f(a0, v3);
        return v2;
    }
    v4 = index->field_0;
    if (!a3)
        return v4;
    if (index->field_8)
    {
        v5 = index->field_8;
        v7 = v5->field_8;
        index->field_0 = v5->field_0;
        index->field_8 = v7;
        sub_40e75f(a0, v5);
    }
    else
    {
        index->field_0 = 0;
    }
    return v4;
}



// Function: transfer_entries @ 0xe932
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
} struct_2;

unsigned int transfer_entries(struct_2 *index, struct_2 *idx, char a2)
{
    struct_1 *iter;  // [bp-0x38]
    struct_0 *idx2;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28], Other Possible Types: unsigned long long
    struct_0 *idx1;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]
    struct_0 *v5;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx2 = iter->field_8; idx2; idx2 = v5)
            {
                v2 = idx2->field_0;
                idx1 = sub_40dc51(index, v2);
                v5 = idx2->field_8;
                if (idx1->field_0)
                {
                    idx2->field_8 = idx1->field_8;
                    idx1->field_8 = idx2;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                    sub_40e75f(index, idx2);
                }
            }
            v2 = iter->field_0;
            iter->field_8 = NULL;
            if (!a2)
            {
                idx1 = sub_40dc51(index, v2);
                if (idx1->field_0)
                {
                    v4 = sub_40e710(index);
                    if (!v4)
                        return 0;
                    v4->field_0 = v2;
                    v4->field_8 = idx1->field_8;
                    idx1->field_8 = v4;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                }
                iter->field_0 = 0;
                idx->field_18 = idx->field_18 - 1;
            }
        }
    }
    return 1;
}



// Function: hash_rehash @ 0xeb06
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[16];
    unsigned long long field_18;
} struct_2;

unsigned int hash_rehash(struct_2 *index, unsigned int a1)
{
    unsigned long v0;  // [bp-0x78]
    struct_2 *idx;  // [bp-0x70]
    char v2;  // [bp-0x68], Other Possible Types: unsigned long
    unsigned long v3;  // [bp-0x60]
    unsigned long v4;  // [bp-0x58]
    unsigned long long v5;  // [bp-0x50]
    unsigned long long v6;  // [bp-0x48]
    unsigned long v7;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    unsigned long v10;  // [bp-0x28]
    unsigned long v11;  // [bp-0x20]

    v0 = sub_40e248(a1, *((long long *)&index[1].padding_8[0]));
    if (!v0)
    {
        return 0;
    }
    else if (v0 == *((long long *)&index->padding_8[8]))
    {
        return 1;
    }
    else
    {
        idx = &v2;
        v2 = calloc(v0, 16);
        if (!v2)
            return 0;
        v4 = v0;
        v3 = v2 + v0 * 16;
        v5 = 0;
        v6 = 0;
        v7 = *((long long *)&index[1].padding_8[0]);
        v8 = *((long long *)&index[1].padding_8[8]);
        v9 = index[1].field_18;
        v10 = index[2].field_0;
        v11 = *((long long *)&index[2].padding_8[0]);
        if ((char)sub_40e932(idx, index, 0))
        {
            free(index->field_0);
            index->field_0 = v2;
            *((unsigned long *)&index->padding_8[0]) = v3;
            *((unsigned long *)&index->padding_8[8]) = v4;
            index->field_18 = v5;
            *((unsigned long *)&index[2].padding_8[0]) = v11;
            return 1;
        }
        *((unsigned long *)&index[2].padding_8[0]) = v11;
        if ((char)sub_40e932(index, idx, 1) == 1 && (char)sub_40e932(index, idx, 0) == 1)
        {
            free(v2);
            return 0;
        }
        abort(); /* do not return */
    }
}



// Function: hash_insert_if_absent @ 0xed31
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned int hash_insert_if_absent(void* idx, unsigned long long a1, unsigned long long *a2)
{
    unsigned long v6;  // rax
    unsigned int v7;  // ymm0
    unsigned long v16;  // rax
    unsigned int v17;  // ymm0
    unsigned long v18;  // rax
    int v8;  // xmm2
    unsigned long v9;  // rax
    unsigned long v11;  // rax
    unsigned int v12;  // ymm0
    int v13;  // xmm2
    unsigned long v14;  // rax
    unsigned int v0;  // [bp-0x34]
    struct_0 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]

    if (!a1)
        abort(); /* do not return */
    v2 = sub_40e799(idx, a1, &index, 0);
    if (v2)
    {
        if (!a2)
            return 0;
        *(a2) = v2;
        return 0;
    }
    else
    {
        v6 = (long long)idx[24];
        v7 = (v6 < 0 ? (unsigned int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1))) : (int)v6);
        v8 = (int)*((int *)((long long)idx[40] + 8));
        v9 = (long long)idx[16];
        if (...)
        {
            sub_40e13b(idx);
            v11 = (long long)idx[24];
            v12 = (v11 < 0 ? (unsigned int)(AddV((unsigned int)(v11 >> 1 | (unsigned int)v11 & 1), (unsigned int)(v11 >> 1 | (unsigned int)v11 & 1))) : (int)v11);
            v13 = (int)*((int *)((long long)idx[40] + 8));
            v14 = (long long)idx[16];
            if (...)
            {
                v3 = (long long)idx[40];
                if ((char)v3[1].field_0)
                {
                    v16 = (long long)idx[16];
                    v17 = MulV((v16 < 0 ? AddV((unsigned int)(v16 >> 1 | (unsigned int)v16 & 1), (unsigned int)(v16 >> 1 | (unsigned int)v16 & 1)) : (unsigned int)v16), *((int *)((char *)&v3->field_8 + 4)));
                }
                else
                {
                    v18 = (long long)idx[16];
                    v17 = MulV((int)v3->field_8, MulV(*((int *)((char *)&v3->field_8 + 4)), (v18 < 0 ? AddV((unsigned int)(v18 >> 1 | (unsigned int)v18 & 1), (unsigned int)(v18 >> 1 | (unsigned int)v18 & 1)) : (unsigned int)v18)));
                }
                v0 = v17;
                if (!(CmpF(v0, 0x5f800000) & 1))
                {
                    return 0xffffffff;
                }
                else if ((char)sub_40eb06(idx, (CmpF(v0, 0x5f000000) & 69 & 1 ? (unsigned int)v0 : (unsigned int)(SubV(v0, 0x5f000000)) ^ 0)) != 1)
                {
                    return 0xffffffff;
                }
                else if (sub_40e799(idx, a1, &index, 0))
                {
                    abort(); /* do not return */
                }
            }
        }
        if (!index->field_0)
        {
            index->field_0 = a1;
            *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
            *((unsigned long long *)&idx[24]) = (long long)idx[24] + 1;
            return 1;
        }
        v4 = sub_40e710(idx);
        if (v4)
        {
            v4->field_0 = a1;
            v4->field_8 = index->field_8;
            index->field_8 = v4;
            *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
            return 1;
        }
        return 0xffffffff;
    }
}



// Function: hash_insert @ 0xf0a2
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]

    v0 = sub_40ed31(a0, a1, &v1);
    if (v0 == 0xffffffff)
    {
        return 0;
    }
    else if (!v0)
    {
        return v1;
    }
    else
    {
        return a1;
    }
}



// Function: hash_remove @ 0xf112
typedef struct struct_1 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long hash_remove(struct_2 *idx, unsigned long long a1)
{
    unsigned long v7;  // rax
    unsigned int v8;  // ymm0
    unsigned long v17;  // rax
    int v18;  // xmm0
    unsigned long long v19;  // rax
    unsigned long v20;  // rax
    int v21;  // xmm0
    int v9;  // xmm2
    unsigned long v10;  // rax
    unsigned long v12;  // rax
    unsigned int v13;  // ymm0
    int v14;  // xmm2
    unsigned long v15;  // rax
    struct_2 *v0;  // [bp-0x40]
    struct_1 *ptr;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    struct_0 *v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    struct_1 *v5;  // [bp-0x18]

    v2 = sub_40e799(idx, a1, &v0, 1);
    if (!v2)
        return 0;
    idx[2].field_0 = idx[2].field_0 - 1;
    if (v0->field_0)
        return v2;
    idx[1].field_8 = (char *)idx[1].field_8 - 1;
    v7 = idx[1].field_8;
    v8 = (v7 < 0 ? (unsigned int)(AddV((unsigned int)(v7 >> 1 | (unsigned int)v7 & 1), (unsigned int)(v7 >> 1 | (unsigned int)v7 & 1))) : (int)v7);
    v9 = (int)(unsigned int)idx[2].field_8->field_0;
    v10 = idx[1].field_0;
    if (...)
    {
        sub_40e13b(idx);
        v12 = idx[1].field_8;
        v13 = (v12 < 0 ? (unsigned int)(AddV((unsigned int)(v12 >> 1 | (unsigned int)v12 & 1), (unsigned int)(v12 >> 1 | (unsigned int)v12 & 1))) : (int)v12);
        v14 = (int)(unsigned int)idx[2].field_8->field_0;
        v15 = idx[1].field_0;
        if (...)
        {
            v3 = idx[2].field_8;
            if (v3->field_10)
            {
                v17 = idx[1].field_0;
                v18 = (int)(MulV((v17 < 0 ? AddV((unsigned int)(v17 >> 1 | (unsigned int)v17 & 1), (unsigned int)(v17 >> 1 | (unsigned int)v17 & 1)) : (unsigned int)v17), v3->field_4));
                v19 = (CmpF(*((unsigned int *)&v18), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v18) : *((unsigned int *)&SubV(v18, 0x5f000000)) ^ 0x8000000000000000);
            }
            else
            {
                v20 = idx[1].field_0;
                v21 = (int)(MulV(v3->field_8, MulV(v3->field_4, (v20 < 0 ? AddV((unsigned int)(v20 >> 1 | (unsigned int)v20 & 1), (unsigned int)(v20 >> 1 | (unsigned int)v20 & 1)) : (unsigned int)v20))));
                v19 = (CmpF(*((unsigned int *)&v21), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v21) : *((unsigned int *)&SubV(v21, 0x5f000000)) ^ 0x8000000000000000);
            }
            v4 = v19;
            if ((char)sub_40eb06(idx, v4) != 1)
            {
                for (ptr = idx[4].field_8; ptr; ptr = v5)
                {
                    v5 = ptr->field_8;
                    free(ptr);
                }
                idx[4].field_8 = NULL;
            }
        }
    }
    return v2;
}



// Function: hash_delete @ 0xf418
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned long long hash_delete(struct_2 *a0, unsigned long long a1)
{
    return sub_40f112(a0, a1);
}



// Function: main @ 0x3c50
typedef struct struct_4 {
    char padding_0[8];
    struct struct_4 *field_8;
    char padding_10[40];
    unsigned long long field_38;
    int field_40;
    char padding_44[20];
    unsigned long long field_58;
    char padding_60[8];
    unsigned short field_68;
    char padding_6a[14];
    long long field_78;
    char padding_80[8];
    unsigned int field_88;
    char padding_8c[36];
    unsigned long long field_b0;
    long long field_b8;
    long long field_c0;
    char padding_c8[16];
    long long field_d8;
    long long field_e0;
} struct_4;

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

typedef struct struct_5 {
    char padding_0[24];
    long long field_18;
    long long field_20;
    char padding_28[32];
    char field_48;
} struct_5;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned int g_4156e0[4];
extern unsigned int g_4156f0[4];
extern char g_4160e1;
extern long long g_41a6a0;
extern char g_41a700;
extern unsigned long long g_41b010;
extern long long g_41b018;
extern long long g_41b020;
extern long long stdout;
extern struct_0 *stdin;
extern int optind;
extern char optarg;
extern unsigned long long stderr;
extern unsigned long long g_41b110;
extern unsigned long long g_41b120;
extern long long g_41b140;
extern unsigned long long g_41b148;
extern unsigned long long g_41b150;
extern unsigned long long g_41b158;
extern unsigned long long g_41b160;
extern unsigned int g_41b168;
extern char g_41b16c;
extern char g_41b16d;
extern unsigned int g_41b170;
extern long long g_41b178;
extern char g_41b180;
extern char g_41b181;
extern char g_41b182;
extern char g_41b183;
extern char g_41b184;
extern char g_41b185;
extern char g_41b186;
extern unsigned long long g_41b188;
extern unsigned long long g_41b190;
extern long long g_41b198;

int main(int a0, long long *a1)
{
    long long v20;  // rdi
    long long v21;  // rsi
    unsigned int v30;  // eax
    int *err;  // rax
    unsigned long v38;  // r13
    char *ptr;  // rax
    long long v22;  // rdx
    unsigned int v40;  // eax
    unsigned long v41;  // rbx
    unsigned int v42;  // eax
    char *v43;  // rdi
    unsigned long long v44;  // rax
    char *v45;  // rax
    int *err1;  // rax
    long long v47;  // r13
    char v48;  // al
    long long v23;  // rcx
    char *v50;  // rax
    long long *v51;  // rdi
    long long v52;  // r15
    unsigned long long v53;  // rbx
    unsigned int v55;  // r14d
    struct_4 *iter;  // r12
    unsigned int v57;  // ebp
    long long v24;  // r8
    long long v25;  // r9
    int v27;  // eax
    int *err2;  // rbx
    char *v99;  // rax
    unsigned int v28;  // eax
    char *v100;  // rax
    char *v101;  // rax
    int *v102;  // rax
    unsigned int v29;  // eax
    unsigned long long v0;  // [bp-0x178]
    char v1;  // [bp-0x160], Other Possible Types: unsigned long long
    unsigned long v2;  // [bp-0x158]
    unsigned int v3;  // [bp-0x150], Other Possible Types: unsigned long
    struct_5 *v4;  // [bp-0x148], Other Possible Types: unsigned int
    char *v5;  // [bp-0x140], Other Possible Types: char
    char v8;  // [bp-0x125]
    unsigned int v9;  // [bp-0x124]
    stat *v10;  // [bp-0x120]
    int v11;  // [bp-0x10c]
    char v12;  // [bp-0x108], Other Possible Types: unsigned long
    long long v16;  // [bp-0xe8]
    unsigned long long v17;  // [bp-0xe0]
    stat v18;  // [bp-0xd8]

    v17 = 0;
    v16 = ".";
    sub_40eb20(*(a1));
    setlocale(6, &g_4160e1);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_413be0(sub_405f80);
    g_41b140 = sub_406800(v20, v21, v22, v23, v24, v25);
    sub_40d520(getenv("DU_BLOCK_SIZE"), &g_41b170, &g_41b148);
    v5 = 0;
    v3 = 16;
    v4 = 8;
    v2 = 0;
    v1 = 1;
    v8 = 0;
    v11 = -0x1;
    v27 = getopt_long(a0, a1, "0abd:chHklmst:xB:DLPSX:", &g_41a700, &v11);
    if (v27 != -0x1)
    {
        if (v27 > 135)
            goto LABEL_403dae;
        if (v27 > 47)
        {
            switch (v27)
            {
            case 48:
                g_41b182 = 1;
                break;
            case 66:
                v30 = sub_40d520(*((long long *)&optarg), &g_41b170, &g_41b148);
                if (v30)
                    sub_412850(v30, v11, 66, &g_41a700, *((long long *)&optarg)); /* do not return */
                break;
            case 68: case 72:
                v3 = 0x11;
                break;
            case 76:
                v3 = 2;
                break;
            case 80:
                v3 = 16;
                break;
            case 83:
                g_41b180 = 1;
                break;
            case 88:
                if (sub_4070c0(sub_406b70, g_41b140, *((long long *)&optarg), 0x10000000, 10))
                {
                    v1 = sub_410b80(0, 3, *((long long *)&optarg));
                    err = __errno_location();
                    error(0, *(err), "%s");
                    v1 = 0;
                    break;
                }
                break;
            case 97:
                g_41b186 = 1;
                break;
            case 98:
                g_41b185 = 1;
                g_41b170 = 0;
                g_41b148 = 1;
                break;
            case 99:
                g_41b181 = 1;
                break;
            case 100:
                if (!sub_412910(*((long long *)&optarg), 0, 0, &v12, &g_4160e1))
                {
                    v8 = 1;
                    g_41b010 = v12;
                    break;
                }
                else
                {
                    v1 = sub_410ef0(*((long long *)&optarg));
                    error(0, 0, dcgettext(NULL, "invalid maximum depth %s", 5));
                    v1 = 0;
                    break;
                }
            case 104:
                g_41b170 = 176;
                g_41b148 = 1;
                break;
            case 107:
                g_41b170 = 0;
                g_41b148 = 0x400;
                break;
            case 108:
                g_41b184 = 1;
                break;
            case 109:
                g_41b170 = 0;
                g_41b148 = 0x100000;
                break;
            case 115:
                v5 = 1;
                break;
            case 116:
                v29 = sub_412250(*((long long *)&optarg), 0, 0, &g_41b178, "kKmMGTPEZY0");
                if (v29)
                    sub_412850(v29, v11, 116, &g_41a700, *((long long *)&optarg)); /* do not return */
                if (!g_41b178)
                {
                    if (*((char *)*((long long *)&optarg)) == 45)
                        error(1, 0, dcgettext(NULL, "invalid --threshold argument '-0'", 5));
                    break;
                }
            case 120:
                v4 = 72;
                break;
            case 128:
                g_41b185 = 1;
                break;
            case 129:
                sub_406b70(g_41b140, *((long long *)&optarg), 0x10000000);
                break;
            case 130:
                v2 = *((long long *)&optarg);
                break;
            case 131:
                g_41b170 = 144;
                g_41b148 = 1;
                break;
            case 133:
                g_41b16c = 1;
                v28 = 0;
                if (*((long long *)&optarg))
                {
                    v0 = 1;
                    v28 = g_4156f0[sub_405ce0("--time")];
                }
                g_41b168 = v28;
                g_41b150 = sub_410fe0(getenv("TZ"));
                break;
            case 134:
                g_41b160 = *((long long *)&optarg);
                break;
            case 135:
                g_41b16d = 1;
                break;
            default:
LABEL_403dae:
                v1 = 0;
                break;
            }
        }
        else if (v27 == -0x83)
        {
            sub_411ab0(stdout, "du", "GNU coreutils", g_41b018, "Torbjorn Granlund", "David MacKenzie", "Paul Eggert", "Jim Meyering", 0, v20);
            exit(0); /* do not return */
        }
        else if (v27 == -0x82)
        {
            sub_405540(0); /* do not return */
        }
    }
    if (!v1)
    {
LABEL_405137:
        sub_405540(1); /* do not return */
    }
    if (g_41b186)
    {
        if (!v5)
            goto LABEL_404163;
        error(0, 0, dcgettext(NULL, "cannot both summarize and show all entries", 5));
        goto LABEL_405137;
    }
    if (v8)
    {
        if (!v5)
            goto LABEL_404163;
        if (!g_41b010)
        {
            error(0, 0, dcgettext(NULL, "warning: summarizing is the same as using --max-depth=0", 5));
            if (!g_41b010)
                goto LABEL_404bdf;
        }
        error(0, 0, dcgettext(NULL, "warning: summarizing conflicts with --max-depth=%lu", 5));
        sub_405540(1); /* do not return */
    }
    else if (v5)
    {
LABEL_404bdf:
        g_41b010 = 0;
    }
LABEL_404163:
    if (g_41b16d)
    {
        if (g_41b185)
            error(0, 0, dcgettext(NULL, "warning: options --apparent-size and -b are ineffective with --inodes", 5));
        g_41b148 = 1;
    }
    if (g_41b16c)
    {
        if (g_41b160)
        {
            v38 = g_41b160;
            goto LABEL_4041a2;
        }
        else
        {
            g_41b160 = getenv("TIME_STYLE");
            v38 = g_41b160;
            if (!v38 || !strcmp(v38, "locale"))
            {
                v38 = "long-iso";
                g_41b160 = "long-iso";
LABEL_404d36:
                v40 = g_4156e0[sub_405ce0("time style", v38, &g_41a6a0, &g_4156e0[0], 4, g_41b020, 1, v22)];
                if (v40 == 1)
                {
                    g_41b158 = "%Y-%m-%d %H:%M";
                }
                else if (v40 == 2)
                {
                    g_41b158 = "%Y-%m-%d";
                }
                else if (!v40)
                {
                    g_41b158 = "%Y-%m-%d %H:%M:%S.%N %z";
                }
            }
            else
            {
                if (*((char *)v38) != 43)
                {
                    while (1)
                    {
                        v38 = g_41b160;
                        if (strncmp(v38, "posix-", 6))
                            break;
                        g_41b160 = v38 + 6;
                    }
                }
                else
                {
                    ptr = strchr(v38, 10);
                    if (!ptr)
                        goto LABEL_4041ad;
                    *(ptr) = 0;
                }
LABEL_4041a2:
                if (*((char *)v38) != 43)
                    goto LABEL_404d36;
LABEL_4041ad:
                g_41b158 = v38 + 1;
            }
        }
    }
    if (!v2)
    {
        v51 = &v16;
        if (a0 > optind)
            v51 = &a1[optind];
        v47 = sub_405e10(v51);
        v48 = optind + 1 < a0 | v3 == 2;
    }
    else if (a0 <= optind)
    {
        v41 = v2;
        v42 = strcmp(v41, "-");
        v43 = &stdin->field_0;
        if (v42)
        {
            v44 = sub_408e20(v41, "r", stdin);
            v43 = &stdin->field_0;
            if (!v44)
            {
                sub_410800(4, v2);
                v45 = dcgettext(NULL, "cannot open %s for reading", 5);
                err1 = __errno_location();
                error(1, *(err1), v45);
            }
        }
        v47 = sub_405e40(v43);
        v48 = v1;
    }
    else
    {
        sub_410ef0(a1[optind]);
        error(0, 0, dcgettext(NULL, "extra operand %s", 5));
        v50 = dcgettext(NULL, "file operands cannot be combined with --files0-from", 5);
        __fprintf_chk(stderr, 0x1, "%s\n", v50);
        sub_405540(1); /* do not return */
    }
    g_41b183 = v48;
    if (!v47 || !(g_41b198 = (long long)(unsigned long long)sub_406130(), g_41b198))
    {
LABEL_4050a5:
        sub_412180(); /* do not return */
    }
    if (g_41b184 || !g_41b183)
        v4 |= 0x100;
    v5 = &v11;
    v52 = v47;
    v9 = v4 | v3;
    v8 = v1;
    v10 = &v18;
    while (1)
    {
        while (1)
        {
LABEL_404290:
            v53 = sub_405e80(v52, v5);
            if (!v53)
            {
                if (v11 != 3)
                {
                    if (v11 == 4)
                    {
                        sub_410b80(0, 3, v2);
                        v101 = dcgettext(NULL, "%s: read error", 5);
                        v102 = __errno_location();
                        error(0, *(v102), v101);
                        v8 = 0;
                    }
                    else if (v11 != 2)
                    {
                        __assert_fail(); /* do not return */
                    }
                    sub_405f40(v52);
                    sub_4061a0(g_41b198);
                    if (g_41b190)
                        sub_4061a0(g_41b190);
                    if (v2 && (stdin->field_0 & 32 || sub_4071a0(stdin)) && v8)
                    {
                        sub_410800(4, v2);
                        error(1, 0, dcgettext(NULL, "error reading %s", 5));
                    }
                    if (!g_41b181)
                        return v8 ^ 1;
                    sub_4053d0(&g_41b120, dcgettext(NULL, "total", 5));
                    return v8 ^ 1;
                }
                goto LABEL_4050a5;
            }
            if (!v2)
                break;
            if (!(strcmp(v2, "-") || strcmp(v53, "-")))
            {
                sub_410800(4, v53);
                error(0, 0, dcgettext(NULL, "when reading file names from stdin, no file name of %s allowed", 5));
                if (*((char *)v53))
                    goto LABEL_4047d5;
            }
            else if (!(!*((char *)v53)))
            {
                goto LABEL_4042e9;
            }
            sub_405f20(v52);
            dcgettext(NULL, "invalid zero-length file name", 5);
            sub_410b80(0, 3, v2);
            error(0, 0, "%s:%lu: %s");
LABEL_4047d5:
            v8 = 0;
        }
        if (*((char *)v53))
            break;
        dcgettext(NULL, "invalid zero-length file name", 5);
        error(0, 0, "%s");
        v8 = 0;
    }
LABEL_4042e9:
    g_41b110 = v53;
    v55 = v1;
    v4 = sub_4121c0(&g_41b110, v9, 0);
    iter = sub_40add0(v4);
    if (!iter)
    {
        err2 = __errno_location();
        if (*(err2))
        {
            v55 = 0;
            sub_410b80(0, 3, v4->field_20);
            v99 = dcgettext(NULL, "fts_read failed: %s", 5);
            error(0, *(err2), v99);
        }
        g_41b188 = 0;
        if (sub_40abe0(v4))
        {
            v55 = 0;
            v100 = dcgettext(NULL, "fts_close failed", 5);
            error(0, *(err2), v100);
        }
        v8 &= (char)v55;
        goto LABEL_404290;
    }
    v57 = iter->field_68;
    v3 = iter->field_38;
    if (v57 == 4)
    {
        goto LABEL_0x4048a0;
    }
    else if (v57 == 6)
    {
        goto LABEL_0x40433a;
    }
    else
    {
        goto LABEL_0x40433a;
    }
}



// Function: print_only_size @ 0x5340
typedef struct FILE {
} FILE;

extern FILE *stdout;

void print_only_size(unsigned long long a0)
{
    char *v1;  // rax

    v1 = (a0 == 0xffffffffffffffff ? dcgettext(NULL, "Infinity", 5) : sub_40c920());
    fputs_unlocked(v1, stdout);
    return;
}



// Function: print_size @ 0x53d0
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    uint128_t field_10;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

extern struct_1 *stdout;
extern long long g_41b150;
extern long long g_41b158;
extern char g_41b16c;
extern char g_41b16d;
extern char g_41b182;

int print_size(struct_0 *idx, char *a1)
{
    unsigned long v4;  // rdi
    char *v5;  // rax
    unsigned long v6;  // rax
    int v0;  // [bp-0x98]
    char v1;  // [bp-0x88]
    char v2;  // [bp-0x48]

    v4 = idx->field_8;
    if (!g_41b16d)
        v4 = idx->field_0;
    sub_405340(v4);
    if (g_41b16c)
    {
        v5 = &stdout->field_28->field_0;
        if (v5 < *((long long *)&stdout[1].padding_0[0]))
        {
            stdout->field_28 = v5 + 1;
            *(v5) = 9;
        }
        else
        {
            __overflow(stdout, 9);
        }
        v0 = (int)idx->field_10;
        if (sub_4112f0(g_41b150, &v0, &v1))
        {
            sub_408e00(stdout, g_41b158, &v1, g_41b150, *((unsigned int *)(&v0 + 8)));
        }
        else
        {
            v6 = sub_40d920(*((unsigned long long *)&v0), &v2);
            sub_410ef0(v6);
            error(0, 0, dcgettext(NULL, "time %s is out of range", 5));
            fputs_unlocked(v6, stdout);
        }
    }
    __printf_chk(1, "\t%s%c", a1, (char)-(g_41b182 < 1) & 10);
    return fflush_unlocked(stdout);
}



// Function: usage @ 0x5540
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
    char *v32;  // rax
    char *v33;  // rax
    char *v34;  // rax
    char *v35;  // rax
    unsigned long long v18;  // r13
    char *v37;  // rax
    char *v38;  // rsi
    struct_0 *v39;  // rbx
    char *v40;  // r14
    char *v42;  // rax
    char *v43;  // rax
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
        v25 = dcgettext(NULL, "Summarize device usage of the set of FILEs, recursively for directories.\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in ('sparse') files, internal\n                          fragmentation, indirect blocks, and the like\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           '-BM' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to '--apparent-size --block-size=1'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n", 5);
        fputs_unlocked(v29, stdout);
        v30 = dcgettext(NULL, "  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n", 5);
        fputs_unlocked(v30, stdout);
        v31 = dcgettext(NULL, "  -P, --no-dereference  don't follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n", 5);
        fputs_unlocked(v31, stdout);
        v32 = dcgettext(NULL, "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in 'date'\n", 5);
        fputs_unlocked(v32, stdout);
        v33 = dcgettext(NULL, "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n", 5);
        fputs_unlocked(v33, stdout);
        v34 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v34, stdout);
        v35 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v35, stdout);
        __printf_chk(1, dcgettext(NULL, "\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n", 5));
        v37 = dcgettext(NULL, "\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n", 5);
        fputs_unlocked(v37, stdout);
        v0 = (int)_INSERT(v0 CONCAT 0, 0, "[");
        v38 = "[";
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
        } while (strcmp("du", v38) && (v38 = v39->field_10, v23 = v39 + 16, v39->field_10));
        v40 = *((long long *)&v23->padding_1[7]);
        if (!*((long long *)&v23->padding_1[7]))
            v40 = "du";
        __printf_chk(1, dcgettext(NULL, "\n%s online help: <%s>\n", 5));
        v42 = setlocale(5, NULL);
        if (v42 && strncmp(v42, "en_", 3))
        {
            v43 = dcgettext(NULL, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
            fputs_unlocked(v43, stdout);
        }
        __printf_chk(1, dcgettext(NULL, "Full documentation <%s%s>\n", 5));
        __printf_chk(1, dcgettext(NULL, "or available locally via: info '(coreutils) %s%s'\n", 5));
    }
    exit(a0); /* do not return */
}



// Function: raw_hasher @ 0xb750
void raw_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: raw_comparator @ 0xb770
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0xb780
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

extern unsigned int g_415aa0[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_415aa0[0])
        return 1;
    v2 = index[2];
    if (((CmpF(v2, 1036831949) & 69 | (char)((CmpF(v2, 1036831949) & 69) >> 6)) & 1) != 1 && ((CmpF(1063675494, v2) & 69 | (char)((CmpF(1063675494, v2) & 69) >> 6)) & 1) != 1 && ((CmpF(index[3], 1066192077) & 69 | (char)((CmpF(index[3], 1066192077) & 69) >> 6)) & 1) != 1)
    {
        v3 = (int)*(index);
        if (!(CmpF(*((unsigned int *)&v3), 0) & 1))
        {
            v4 = (unsigned int)(AddV(v3, 1036831949));
            v5 = index[1];
            if (((CmpF(v5, v4) & 69 | (char)((CmpF(v5, v4) & 69) >> 6)) & 1) != 1 && !(CmpF(0x3f800000, v5) & 1) && ((CmpF(v2, v4) & 69 | (char)((CmpF(v2, v4) & 69) >> 6)) & 1) != 1)
                return 1;
        }
    }
    idx->field_28 = &g_415aa0[0];
    return 0;
}



// Function: hash_find_entry @ 0xb810
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
    struct struct_1 *field_38;
    char padding_40[8];
    void* field_48;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

double hash_find_entry(void* index, long long a1, void* a2, long long a3)
{
    unsigned long long v1;  // rax
    void* iter;  // rbx
    void* idx;  // rdx
    unsigned long v5;  // xmm0lq
    void* v6;  // rdx
    int v7;  // xmm0
    unsigned long v8;  // xmm0lq
    unsigned long v9;  // xmm0lq
    unsigned long v10;  // xmm0lq

    v1 = index->field_30(a1, index->field_10);
    if (v1 >= index->field_10)
        abort(); /* do not return */
    iter = v1 * 16 + index->field_0;
    if (!*((long long *)iter))
    {
LABEL_40b8bf:
        return v10;
    }
    if (*((long long *)iter) != a1 && !(char)index->field_38(a1))
    {
        for (*(a2) = iter; (long long)iter[8]; iter = (long long)iter[8])
        {
            if (*((long long *)(long long)iter[8]) == a1 || (char)index->field_38(a1))
            {
                idx = (long long)iter[8];
                if (!a3)
                    return v5;
                *((long long *)&iter[8]) = (long long)idx[8];
                *((unsigned long *)idx) = 0;
                *((void* *)&idx[8]) = index->field_48;
                index->field_48 = idx;
                return v5;
            }
        }
        goto LABEL_40b8bf;
    }
    if (!a3)
        return v9;
    v6 = (long long)iter[8];
    if (!v6)
    {
        *((unsigned long *)iter) = 0;
        return v8;
    }
    v7 = (int)*((int128_t *)v6);
    *((uint128_t *)iter) = (uint128_t)v7;
    *((unsigned long *)v6) = 0;
    *((void* *)&v6[8]) = index->field_48;
    index->field_48 = v6;
    return *((unsigned long long *)&v7);
}



// Function: transfer_entries @ 0xba60
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

typedef struct struct_4 {
    unsigned long long field_0;
    char padding_8[16];
    unsigned long long field_18;
    char padding_20[16];
    struct struct_5 *field_30;
    char padding_38[16];
    struct struct_0 *field_48;
} struct_4;

typedef struct struct_5 {
    unsigned long long field_0;
} struct_5;

unsigned int transfer_entries(struct_4 *idx, struct_2 *idx1, char a2)
{
    struct_1 *iter;  // rbx
    unsigned long long v2;  // r15
    struct_0 *v11;  // rdx
    struct_0 *index;  // r13
    unsigned long long v4;  // rax
    struct_0 *idx2;  // rax
    struct_0 *v6;  // rdx
    struct_0 *v7;  // rax
    unsigned long long v8;  // rax
    struct_0 *v9;  // r13
    struct_0 *ptr;  // rax

    iter = idx1->field_0;
    if (iter >= *((long long *)&idx1->padding_8[0]))
        return 1;
    while (1)
    {
        v2 = iter->field_0;
        if (!v2)
        {
LABEL_40ba88:
            iter = &iter->field_10;
            if (*((long long *)&idx1->padding_8[0]) <= iter)
                return 1;
        }
        else
        {
            index = iter->field_8;
            if (index)
            {
                while (1)
                {
                    v4 = idx->field_30(index->field_0);
                    if (v4 >= *((long long *)&idx->padding_8[8]))
                        break;
                    idx2 = v4 * 16 + idx->field_0;
                    v6 = index->field_8;
                    if (idx2->field_0)
                    {
                        index->field_8 = idx2->field_8;
                        idx2->field_8 = index;
                        if (!v6)
                            goto LABEL_40bb0e;
                    }
                    else
                    {
                        idx2->field_0 = index->field_0;
                        v7 = idx->field_48;
                        idx->field_18 = idx->field_18 + 1;
                        index->field_0 = 0;
                        index->field_8 = v7;
                        idx->field_48 = index;
                        if (!v6)
                        {
LABEL_40bb0e:
                            v2 = iter->field_0;
                            goto LABEL_40bb11;
                        }
                    }
                    index = v6;
                }
            }
LABEL_40bb11:
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_40ba88;
            v8 = idx->field_30(v2, *((long long *)&idx->padding_8[8]));
            if (v8 >= *((long long *)&idx->padding_8[8]))
                break;
            v9 = v8 * 16 + idx->field_0;
            if (v9->field_0)
            {
                ptr = idx->field_48;
                if (ptr)
                {
                    idx->field_48 = ptr->field_8;
                }
                else
                {
                    ptr = malloc(16);
                    if (!ptr)
                        return 0;
                }
                v11 = v9->field_8;
                ptr->field_0 = v2;
                ptr->field_8 = v11;
                v9->field_8 = ptr;
            }
            else
            {
                v9->field_0 = v2;
                idx->field_18 = idx->field_18 + 1;
            }
            iter->field_0 = 0;
            iter = &iter->field_10;
            idx1->field_18 = idx1->field_18 - 1;
            if (*((long long *)&idx1->padding_8[0]) <= iter)
                return 1;
        }
    }
    abort(); /* do not return */
}



// Function: hash_print_statistics @ 0xbcd0
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

int hash_print_statistics(void* idx, int a1)
{
    unsigned long long v2;  // r12
    struct_1 *iter;  // rcx
    int v12;  // xmm1
    struct_1 *v4;  // rsi
    unsigned long v5;  // rbx
    unsigned long v6;  // r13
    struct_0 *v7;  // rax
    unsigned long long v8;  // rdx
    struct_0 *v9;  // rax
    struct_0 *v10;  // rax
    uint128_t v11;  // xmm0
    char v0;  // [bp+0x0]

    v2 = 0;
    iter = *((long long *)idx);
    v4 = (long long)idx[8];
    v5 = (long long)idx[16];
    v6 = (long long)idx[24];
    if (*((long long *)idx) < v4)
    {
        while (1)
        {
            if (!iter->field_0)
            {
                iter = &iter->field_10;
                if (iter >= v4)
                    break;
            }
            else
            {
                v7 = iter->field_8;
                v8 = 1;
                if (iter->field_8)
                {
                    do
                    {
                        v9 = v7;
                        v10 = v9->field_8;
                        v8 += 1;
                        v7 = v10;
                    } while (v9->field_8);
                }
                if (v2 < v8)
                    v2 = v8;
                iter = &iter->field_10;
                if (iter >= v4)
                    break;
            }
        }
    }
    __fprintf_chk(a1, 0x1, "# entries:         %lu\n", (long long)idx[32]);
    __fprintf_chk(a1, 0x1, "# buckets:         %lu\n", v5);
    if (v6 >= 0)
    {
        v11 = MulV(v6, 0x4059000000000000);
        if (v5 < 0)
            goto LABEL_40be09;
LABEL_40bd8a:
        v12 = (int)v5;
    }
    else
    {
        v11 = MulV(AddV(v6 >> 1 | (unsigned int)v6 & 1, v6 >> 1 | (unsigned int)v6 & 1), 0x4059000000000000);
        if (v5 >= 0)
            goto LABEL_40bd8a;
LABEL_40be09:
        v12 = (int)(AddV(v5 >> 1 | (unsigned int)v5 & 1, v5 >> 1 | (unsigned int)v5 & 1));
    }
    __fprintf_chk(a1, 0x1, "# buckets used:    %lu (%.2f%%)\n", v6, (unsigned long long)(DivV(v11, v12)));
    return __fprintf_chk(*((unsigned int *)&&v0), 0x1, "max bucket length: %lu\n", v2);
}



// Function: hash_lookup @ 0xbe30
typedef struct struct_3 {
    char padding_0[8];
    struct struct_3 *field_8;
} struct_3;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    struct struct_1 *field_30;
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_lookup(struct_0 *a0, unsigned long long a1)
{
    unsigned long long v1;  // rax
    struct_3 *iter;  // rbx
    unsigned long long i;  // rsi

    v1 = a0->field_30(a1, a0->field_10);
    if (v1 >= a0->field_10)
        abort(); /* do not return */
    iter = v1 * 16 + a0->field_0;
    if (!iter->padding_0)
        return 0;
    for (i = (unsigned long long)iter->padding_0; i != a1; i = (unsigned long long)iter->padding_0)
    {
        if ((char)a0->field_38(a1))
            return iter->padding_0;
        iter = iter->field_8;
        if (!iter)
            return 0;
    }
    return a1;
}



// Function: hash_get_next @ 0xbef0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40befc();
    return;
}



// Function: hash_do_for_each @ 0xbfd0
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_do_for_each(struct_0 **a0, unsigned long long *a1, unsigned long a2)
{
    struct_0 *v1;  // r14
    unsigned long long v2;  // r12
    struct_0 *v3;  // rbx
    struct_0 *v4;  // rbx

    v1 = *(a0);
    if (*(a0) >= a0[1])
        return 0;
    v2 = 0;
    do
    {
        if (v1->field_0)
        {
            v3 = v1;
            while (1)
            {
                if (!(char)a1())
                    return v2;
                v4 = v3->field_8;
                v2 += 1;
                if (!v3->field_8)
                    break;
                v3 = v4;
            }
        }
    } while ((v1 += 16, a0[1] > v1));
    return v2;
}



// Function: hash_initialize @ 0xc0c0
typedef struct struct_0 {
    char padding_0[16];
    char field_10;
} struct_0;

typedef struct struct_2 {
    void* field_0;
    unsigned long long field_8;
    char field_10;
    char padding_11[7];
    unsigned long long field_18;
    unsigned long long field_20;
    struct struct_0 *field_28;
    unsigned long long field_30;
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
} struct_2;

extern struct_0 g_415aa0;

struct_2 * hash_initialize(long long a0, struct_0 *a1, unsigned long long a2, unsigned long long a3, unsigned long a4)
{
    unsigned long long v1;  // rax
    struct_0 *v2;  // r15
    unsigned long long v3;  // rax
    struct_2 *ptr;  // r12
    struct_0 *v5;  // rax
    unsigned long v6;  // rax
    void* ptr1;  // rax
    unsigned long v8;  // rdx

    v1 = sub_40b750;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_40b770;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_415aa0.field_0;
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_40b780(ptr))
    {
        v6 = sub_40b920(a0, v2->field_10);
        *((unsigned long *)&ptr->field_10) = v6;
        if (v6)
        {
            ptr1 = calloc(v6, 16);
            ptr->field_0 = ptr1;
            if (ptr1)
            {
                v8 = *((long long *)&ptr->field_10);
                ptr->field_30 = a2;
                ptr->field_18 = 0;
                ptr->field_38 = a3;
                ptr->field_40 = a4;
                ptr->field_8 = ptr1 + v8 * 16;
                ptr->field_20 = 0;
                ptr->field_48 = 0;
                return ptr;
            }
        }
    }
    free(ptr);
    return NULL;
}



// Function: hash_clear @ 0xc1d0
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    void* field_0;
    char padding_8[16];
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[24];
    struct struct_2 *field_40;
    struct struct_0 *field_48;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
} struct_2;

int hash_clear(void)
{
    struct_1 *idx;  // rdi
    void* iter;  // r12
    struct_0 *index;  // rbx
    unsigned long long *v4;  // rdx
    struct_0 *v5;  // rax
    struct_0 *v6;  // rcx

    iter = idx->field_0;
    if (iter < *((long long *)&idx->padding_8[0]))
    {
        while (1)
        {
            if (!*((long long *)iter))
            {
                iter += 16;
                if (*((long long *)&idx->padding_8[0]) <= iter)
                    break;
            }
            else
            {
                index = (long long)iter[8];
                v4 = &idx->field_40->field_0;
                if (index)
                {
                    while (1)
                    {
                        if (v4)
                        {
                            v4(index->field_0);
                            v4 = &idx->field_40->field_0;
                        }
                        v5 = index->field_8;
                        v6 = idx->field_48;
                        index->field_0 = 0;
                        index->field_8 = v6;
                        idx->field_48 = index;
                        if (!v5)
                            break;
                        index = v5;
                    }
                }
                if (v4)
                    v4(*((long long *)iter));
                *((unsigned long long *)iter) = 0;
                iter += 16;
                *((unsigned long long *)((char *)iter - 8)) = 0;
                if (*((long long *)&idx->padding_8[0]) <= iter)
                    break;
            }
        }
    }
    idx->field_18 = 0;
    idx->field_20 = 0;
    return;
}



// Function: hash_free @ 0xc280
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
    struct_0 *iter;  // r13
    struct_0 *v2;  // rax
    struct_0 *v3;  // rbx
    struct_0 *v4;  // rbx
    struct_0 *v5;  // rbx
    struct_0 *ptr1;  // rbx
    struct_0 *v7;  // rbx
    struct_1 *v8;  // rbx
    struct_1 *ptr2;  // rbx
    struct_1 *v10;  // rbx

    iter = ptr->field_0;
    v2 = *((long long *)&ptr->padding_8[0]);
    if (ptr->field_40 && ptr->field_20)
    {
        if (iter >= v2)
            goto LABEL_40c32c;
        while (1)
        {
            if (!iter->field_0)
            {
                iter = &iter->field_10;
                if (v2 <= iter)
                    break;
            }
            else
            {
                v3 = iter;
                while (1)
                {
                    ptr->field_40();
                    v4 = v3->field_8;
                    if (!v3->field_8)
                        break;
                    v3 = v4;
                }
                v2 = *((long long *)&ptr->padding_8[0]);
                iter = &iter->field_10;
                if (v2 <= iter)
                    break;
            }
        }
        iter = ptr->field_0;
    }
    if (v2 > iter)
    {
        do
        {
            v5 = iter->field_8;
            if (iter->field_8)
            {
                do
                {
                    ptr1 = v5;
                    v7 = ptr1->field_8;
                    free(ptr1);
                    v5 = v7;
                } while (ptr1->field_8);
            }
        } while ((iter += 16, *((long long *)&ptr->padding_8[0]) > iter));
    }
LABEL_40c32c:
    v8 = ptr->field_48;
    if (!ptr->field_48)
    {
        free(ptr->field_0);
        free(ptr);
        return;
    }
    do
    {
        ptr2 = v8;
        v10 = ptr2->field_8;
        free(ptr2);
        v8 = v10;
    } while (ptr2->field_8);
    free(ptr->field_0);
    free(ptr);
    return;
}



// Function: hash_rehash @ 0xc370
typedef struct struct_0 {
    char padding_0[16];
    char field_10;
} struct_0;

typedef struct struct_1 {
    void* field_0;
    unsigned long long field_8;
    char field_10;
    char padding_11[7];
    unsigned long long field_18;
    char padding_20[8];
    struct struct_0 *field_28;
    unsigned long long field_30;
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
} struct_1;

unsigned int hash_rehash(struct_1 *idx, long long a1)
{
    unsigned long v11;  // rax
    unsigned long v12;  // rbx
    unsigned int v13;  // eax
    unsigned long ptr;  // [bp-0x88]
    unsigned long long v1;  // [bp-0x80]
    unsigned long v2;  // [bp-0x78]
    unsigned long long v3;  // [bp-0x70]
    unsigned long long v4;  // [bp-0x68]
    struct_0 *v5;  // [bp-0x60]
    unsigned long v6;  // [bp-0x58]
    unsigned long v7;  // [bp-0x50]
    unsigned long v8;  // [bp-0x48]
    unsigned long v9;  // [bp-0x40]

    v11 = sub_40b920(a1, idx->field_28->field_10);
    if (!v11)
        return 0;
    v12 = v11;
    if (*((long long *)&idx->field_10) == v11)
        return 1;
    ptr = calloc(v11, 16);
    if (ptr)
    {
        v2 = v12;
        v3 = 0;
        v1 = ptr + v12 * 16;
        v4 = 0;
        v5 = idx->field_28;
        v6 = idx->field_30;
        v7 = idx->field_38;
        v8 = idx->field_40;
        v9 = idx->field_48;
        v13 = sub_40ba60(&ptr, idx, 0);
        if ((char)v13)
        {
            free(idx->field_0);
            idx->field_0 = ptr;
            idx->field_8 = v1;
            *((unsigned long *)&idx->field_10) = v2;
            idx->field_18 = v3;
            idx->field_48 = v9;
            return v13;
        }
        idx->field_48 = v9;
        if ((char)sub_40ba60(idx, &ptr, 1) && (char)sub_40ba60(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0xc4f0
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[8];
    unsigned int field_8;
    unsigned int field_c;
    char field_10;
} struct_2;

unsigned int hash_insert_if_absent(struct_1 *idx, long long a1, unsigned long long *a2)
{
    unsigned long long v2;  // rax
    unsigned long v3;  // rax
    struct_0 *ptr;  // rax
    struct_0 *v13;  // rcx
    unsigned int v4;  // ymm5
    unsigned long v5;  // rax
    int v6;  // xmm4
    struct_2 *v8;  // rax
    int v9;  // xmm0
    uint128_t v10;  // xmm4
    struct_0 *idx1;  // rdx
    char v0;  // [bp-0x38]

    if (!a1)
        abort(); /* do not return */
    sub_40b810(idx, a1, &v0, 0);
    if (v2)
    {
        if (!a2)
            return 0;
        *(a2) = v2;
        return 0;
    }
    else
    {
        v3 = *((long long *)&idx[1].field_8);
        if (v3 >= 0)
        {
            v4 = v3;
            v5 = *((long long *)&idx[1].field_0);
            if (v5 < 0)
                goto LABEL_40c606;
LABEL_40c57f:
            v6 = (int)(unsigned int)v5;
        }
        else
        {
            v5 = *((long long *)&idx[1].field_0);
            v4 = AddV((unsigned int)(v3 >> 1) | (unsigned int)v3 & 1, (unsigned int)(v3 >> 1) | (unsigned int)v3 & 1);
            if (v5 >= 0)
                goto LABEL_40c57f;
LABEL_40c606:
            v6 = (int)(AddV((unsigned int)(v5 >> 1 | (unsigned int)v5 & 1), (unsigned int)(v5 >> 1 | (unsigned int)v5 & 1)));
        }
        if (((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69 | (char)((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69) >> 6)) & 1) != 1)
        {
            sub_40b780(idx);
            v8 = *((long long *)&idx[2].field_8);
            v9 = (int)v8->field_8;
            if (((CmpF(v4, *((unsigned int *)&MulV(v6, v9))) & 69 | (char)((CmpF(v4, *((unsigned int *)&MulV(v6, v9))) & 69) >> 6)) & 1) != 1)
            {
                v10 = (uint128_t)(MulV(v6, v8->field_c));
                if (!v8->field_10)
                    v10 = (uint128_t)(MulV(v10, v9));
                if (!(CmpF((unsigned int)v10, 0x5f800000) & 1))
                {
                    return 0xffffffff;
                }
                else if (!(char)sub_40c370(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else
                {
                    sub_40b810(idx, a1, &v0, 0);
                    if (v2)
                        abort(); /* do not return */
                }
            }
        }
        idx1 = (struct_0 *)v0;
        if (!idx1->field_0)
        {
            idx1->field_0 = a1;
            *((unsigned long long *)&idx[2].field_0) = *((long long *)&idx[2].field_0) + 1;
            *((unsigned long long *)&idx[1].field_8) = *((long long *)&idx[1].field_8) + 1;
            return 1;
        }
        ptr = *((long long *)&idx[4].field_8);
        if (ptr)
        {
            *((struct struct_0 **)&idx[4].field_8) = ptr->field_8;
        }
        else
        {
            ptr = malloc(16);
            if (!ptr)
                return 0xffffffff;
            idx1 = (struct_0 *)v0;
        }
        v13 = idx1->field_8;
        ptr->field_0 = a1;
        ptr->field_8 = v13;
        idx1->field_8 = ptr;
        *((unsigned long long *)&idx[2].field_0) = *((long long *)&idx[2].field_0) + 1;
        return 1;
    }
}



// Function: hash_insert @ 0xc720
long long hash_insert(unsigned int *a0, long long a1)
{
    unsigned int v2;  // eax
    long long v0;  // [bp-0x18], Other Possible Types: unsigned long long

    v2 = sub_40c4f0(a0, a1, &v0);
    if (v2 == 0xffffffff)
    {
        return 0;
    }
    else if (!v2)
    {
        return v0;
    }
    else
    {
        return a1;
    }
}



// Function: hash_remove @ 0xc780
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long hash_remove(struct_1 *idx, long long a1)
{
    unsigned long long v2;  // rax
    unsigned long v3;  // rax
    struct_1 *v12;  // rbp
    struct_1 *ptr;  // rbp
    struct_1 *v14;  // rbp
    unsigned long v4;  // rax
    unsigned int v5;  // ymm5
    unsigned long v6;  // rax
    int v7;  // xmm4
    struct_0 *v9;  // rax
    uint128_t v11;  // xmm4
    unsigned long long *v0;  // [bp-0x28], Other Possible Types: char

    sub_40b810(idx, a1, &v0, 1);
    if (!v2)
        return v2;
    *((unsigned long long *)&idx[2].field_0) = *((long long *)&idx[2].field_0) - 1;
    if (*(v0))
        return v2;
    v3 = *((long long *)&idx[1].field_8);
    v4 = v3 - 1;
    *((unsigned long *)&idx[1].field_8) = v4;
    if (!(v3 - 1 >> 63 & 1))
    {
        v5 = v4;
        v6 = *((long long *)&idx[1].field_0);
        if (v6 < 0)
            goto LABEL_40c8c6;
LABEL_40c810:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = *((long long *)&idx[1].field_0);
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_40c810;
LABEL_40c8c6:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_40b780(idx);
    v9 = *((long long *)&idx[2].field_8);
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_40c370(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
        return v2;
    v12 = *((long long *)&idx[4].field_8);
    if (*((long long *)&idx[4].field_8))
    {
        do
        {
            ptr = v12;
            v14 = *((long long *)&ptr->field_8);
            free(ptr);
            v12 = v14;
        } while (*((long long *)&ptr->field_8));
    }
    *((struct_1 **)&idx[4].field_8) = NULL;
    return v2;
}



// Function: main @ 0x3cd0
extern unsigned int g_4156f0[4];
extern unsigned int g_415700[4];
extern char g_4160e9;
extern long long g_41c680;
extern char g_41c6e0;
extern unsigned long long g_41d010;
extern long long g_41d018;
extern long long g_41d020;
extern long long stdout;
extern unsigned long long stdin;
extern int optind;
extern char optarg;
extern unsigned long long stderr;
extern unsigned long long g_41d110;
extern long long g_41d120;
extern long long g_41d140;
extern unsigned long long g_41d148;
extern unsigned long long g_41d150;
extern unsigned long long g_41d158;
extern unsigned long long g_41d160;
extern unsigned int g_41d168;
extern char g_41d16c;
extern char g_41d16d;
extern unsigned int g_41d170;
extern unsigned long long g_41d178;
extern char g_41d180;
extern char g_41d181;
extern char g_41d182;
extern char g_41d183;
extern char g_41d184;
extern char g_41d185;
extern char g_41d186;
extern long long g_41d190;
extern long long g_41d198;

int main(int a0, char **a1)
{
    unsigned int v11;  // r15d
    long long *v12;  // rsi
    unsigned int v21;  // eax
    unsigned int v22;  // eax
    int *err;  // rax
    unsigned long ptr;  // r12
    unsigned int v29;  // eax
    unsigned long v30;  // rbx
    long long v13;  // rdi
    unsigned int v31;  // eax
    unsigned long v32;  // rdi
    unsigned long long v33;  // rax
    char *v34;  // rax
    int *err1;  // rax
    long long v36;  // rbp
    char v37;  // al
    char *v39;  // rax
    char *v40;  // rdi
    long long v14;  // rdx
    char *v41;  // r14
    unsigned int v43;  // eax
    char *v44;  // rax
    int *err2;  // rax
    long long v15;  // rcx
    long long v16;  // r8
    long long v17;  // r9
    unsigned long ptr1;  // rax
    unsigned int v20;  // eax
    unsigned long long v0;  // [bp-0x98]
    unsigned long v1;  // [bp-0x80]
    char *v2;  // [bp-0x78], Other Possible Types: unsigned int
    unsigned int v3;  // [bp-0x70]
    char v4;  // [bp-0x6a]
    char v5;  // [bp-0x69]
    int v6;  // [bp-0x64]
    char v7;  // [bp-0x60], Other Possible Types: unsigned long
    unsigned long v8;  // [bp-0x58]
    unsigned long long v9;  // [bp-0x50]

    v11 = 1;
    v9 = 0;
    v8 = sub_405630();
    sub_40e310(*(v12));
    setlocale(6, &g_4160e9);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_4137d0(sub_406020);
    g_41d140 = sub_4068c0(v13, v12, v14, v15, v16, v17);
    sub_40d070(getenv("DU_BLOCK_SIZE"), &g_41d170, &g_41d148);
    v4 = 0;
    v2 = 16;
    v3 = 8;
    v1 = 0;
    v5 = 0;
    v6 = -0x1;
    ptr1 = getopt_long(a0, a1, "0abd:chHklmst:xB:DLPSX:", &g_41c6e0, &v6);
    if ((unsigned int)ptr1 != -0x1)
    {
        if ((unsigned int)ptr1 > 135)
            goto LABEL_403e26;
        if ((unsigned int)ptr1 > 47)
        {
            switch ((unsigned int)ptr1)
            {
            case 48:
                g_41d182 = 1;
                break;
            case 66:
                v22 = sub_40d070(*((long long *)&optarg), &g_41d170, &g_41d148);
                if (v22)
                    sub_412020(v22, v6, 66, &g_41c6e0, *((long long *)&optarg)); /* do not return */
                break;
            case 68: case 72:
                v2 = 0x11;
                break;
            case 76:
                v2 = 2;
                break;
            case 80:
                v2 = 16;
                break;
            case 83:
                g_41d180 = 1;
                break;
            case 88:
                if (sub_406fe0(sub_406b50, g_41d140, *((long long *)&optarg), 0x10000000, 10))
                {
                    sub_410680(0, 3, *((long long *)&optarg));
                    err = __errno_location();
                    v11 = 0;
                    error(0, *(err), "%s");
                    break;
                }
                break;
            case 97:
                g_41d186 = 1;
                break;
            case 98:
                g_41d185 = 1;
                g_41d170 = 0;
                g_41d148 = 1;
                break;
            case 99:
                g_41d181 = 1;
                break;
            case 100:
                if (!sub_4120a0(*((long long *)&optarg), 0, 0, &v7, &g_4160e9))
                {
                    v5 = 1;
                    g_41d010 = v7;
                    break;
                }
                else
                {
                    sub_410840(*((long long *)&optarg));
                    v11 = 0;
                    error(0, 0, dcgettext(NULL, "invalid maximum depth %s", 5));
                    break;
                }
            case 104:
                g_41d170 = 176;
                g_41d148 = 1;
                break;
            case 107:
                g_41d170 = 0;
                g_41d148 = 0x400;
                break;
            case 108:
                g_41d184 = 1;
                break;
            case 109:
                g_41d170 = 0;
                g_41d148 = 0x100000;
                break;
            case 115:
                v4 = 1;
                break;
            case 116:
                v21 = sub_411b80(*((long long *)&optarg), 0, 0, &g_41d178, "kKmMGTPEZY0");
                if (v21)
                    sub_412020(v21, v6, 116, &g_41c6e0, *((long long *)&optarg)); /* do not return */
                if (!g_41d178)
                {
                    if (*((char *)*((long long *)&optarg)) == 45)
                        error(1, 0, dcgettext(NULL, "invalid --threshold argument '-0'", 5));
                    break;
                }
            case 120:
                v3 = 72;
                break;
            case 128:
                g_41d185 = 1;
                break;
            case 129:
                sub_406b50(g_41d140, *((long long *)&optarg), 0x10000000);
                break;
            case 130:
                v1 = *((long long *)&optarg);
                break;
            case 131:
                g_41d170 = 144;
                g_41d148 = 1;
                break;
            case 133:
                g_41d16c = 1;
                v20 = 0;
                if (*((long long *)&optarg))
                {
                    v0 = 1;
                    v20 = g_415700[sub_405dc0("--time")];
                }
                g_41d168 = v20;
                g_41d150 = sub_4109c0(getenv("TZ"));
                break;
            case 134:
                g_41d160 = *((long long *)&optarg);
                break;
            case 135:
                g_41d16d = 1;
                break;
            default:
LABEL_403e26:
                v11 = 0;
                break;
            }
        }
        else if ((unsigned int)ptr1 == -0x83)
        {
            sub_411510(stdout, "du", "GNU coreutils", g_41d018, "Torbjorn Granlund", "David MacKenzie", "Paul Eggert", "Jim Meyering", 0, v12);
            exit(0); /* do not return */
        }
        else if ((unsigned int)ptr1 == -0x82)
        {
            sub_4058b0(0); /* do not return */
        }
    }
    if (!(char)v11)
        sub_4058b0(1); /* do not return */
    if (g_41d186)
    {
        if (v4)
        {
            error(0, 0, dcgettext(NULL, "cannot both summarize and show all entries", 5));
            sub_4058b0(1); /* do not return */
        }
    }
    else
    {
        if (v5)
        {
            if (!v4)
                goto LABEL_4041d1;
            if (!g_41d010)
            {
                ptr1 = (unsigned long long)error(0, 0, dcgettext(NULL, "warning: summarizing is the same as using --max-depth=0", 5));
                if (!g_41d010)
                    goto LABEL_4045ed;
            }
            error(0, 0, dcgettext(NULL, "warning: summarizing conflicts with --max-depth=%lu", 5));
            sub_4058b0(1); /* do not return */
        }
        else if (v4)
        {
LABEL_4045ed:
            g_41d010 = 0;
        }
    }
LABEL_4041d1:
    if (g_41d16d)
    {
        if (g_41d185)
            ptr1 = (unsigned long long)error(0, 0, dcgettext(NULL, "warning: options --apparent-size and -b are ineffective with --inodes", 5));
        g_41d148 = 1;
    }
    if (g_41d16c)
    {
        if (g_41d160)
        {
            ptr = g_41d160;
            goto LABEL_404210;
        }
        else
        {
            ptr1 = getenv("TIME_STYLE");
            g_41d160 = ptr1;
            ptr = ptr1;
            if (!ptr1 || (ptr1 = (unsigned long)(long long)strcmp(ptr1, "locale"), !(unsigned int)ptr1))
            {
                ptr = "long-iso";
                g_41d160 = "long-iso";
LABEL_404697:
                v29 = g_4156f0[sub_405dc0("time style", ptr, &g_41c680, &g_4156f0[0], 4, g_41d020, 1, ptr1)];
                if (v29 == 1)
                {
                    g_41d158 = "%Y-%m-%d %H:%M";
                }
                else if (v29 == 2)
                {
                    g_41d158 = "%Y-%m-%d";
                }
                else if (!v29)
                {
                    g_41d158 = "%Y-%m-%d %H:%M:%S.%N %z";
                }
            }
            else
            {
                if (*((char *)ptr) != 43)
                {
                    while (1)
                    {
                        ptr = g_41d160;
                        if (strncmp(ptr, "posix-", 6))
                            break;
                        g_41d160 = ptr + 6;
                    }
                }
                else
                {
                    ptr1 = strchr(ptr, 10);
                    if (!ptr1)
                        goto LABEL_40421b;
                    *((char *)ptr1) = 0;
                }
LABEL_404210:
                if (*((char *)ptr) != 43)
                    goto LABEL_404697;
LABEL_40421b:
                g_41d158 = ptr + 1;
            }
        }
    }
    if (!v1)
    {
        v40 = &v8;
        if (a0 > optind)
            v40 = &a1[optind];
        v36 = sub_405eb0(v40);
        v37 = optind + 1 < a0 | v2 == 2;
    }
    else if (a0 <= optind)
    {
        v30 = v1;
        v31 = strcmp(v30, "-");
        v32 = stdin;
        if (v31)
        {
            v33 = sub_408c10(v30, "r", stdin);
            v32 = stdin;
            if (!v33)
            {
                sub_410580(4, v1);
                v34 = dcgettext(NULL, "cannot open %s for reading", 5);
                err1 = __errno_location();
                error(1, *(err1), v34);
            }
        }
        v36 = sub_405ee0(v32);
        v37 = v11;
    }
    else
    {
        sub_410840(a1[optind]);
        error(0, 0, dcgettext(NULL, "extra operand %s", 5));
        v39 = dcgettext(NULL, "file operands cannot be combined with --files0-from", 5);
        __fprintf_chk(stderr, 0x1, "%s\n", v39);
        sub_4058b0(1); /* do not return */
    }
    g_41d183 = v37;
    if (v36)
    {
        g_41d198 = sub_406240();
        if (g_41d198)
        {
            if (g_41d184 || !g_41d183)
                v3 |= 0x100;
            v3 |= v2;
            while (1)
            {
                v41 = sub_405f20(v36, &v7);
                if (!v41)
                    break;
                if (v1)
                {
                    if (strcmp(v1, "-") || strcmp(v41, "-"))
                    {
                        if (*(v41))
                            goto LABEL_404341;
                    }
                    else
                    {
                        sub_410580(4, v41);
                        error(0, 0, dcgettext(NULL, "when reading file names from stdin, no file name of %s allowed", 5));
                        if (*(v41))
                            goto LABEL_404475;
                    }
                    sub_405fc0(v36);
                    v2 = dcgettext(NULL, "invalid zero-length file name", 5);
                    sub_410680(0, 3, v1);
                    error(0, 0, "%s:%lu: %s");
LABEL_404475:
                    v11 = 0;
                }
                else if (*(v41))
                {
LABEL_404341:
                    g_41d110 = v41;
                    v11 &= sub_405540(v3);
                }
                else
                {
                    v11 = 0;
                    dcgettext(NULL, "invalid zero-length file name", 5);
                    error(0, 0, "%s");
                }
            }
            v43 = *((unsigned int *)&v7);
            if (v43 != 3)
            {
                if (v43 == 4)
                {
                    v11 = 0;
                    sub_410680(0, 3, v1);
                    v44 = dcgettext(NULL, "%s: read error", 5);
                    err2 = __errno_location();
                    error(0, *(err2), v44);
                }
                else if (v43 != 2)
                {
                    __assert_fail(); /* do not return */
                }
                sub_405fe0(v36);
                sub_4062b0(g_41d198);
                if (g_41d190)
                    sub_4062b0(g_41d190);
                if (v1 && (ferror_unlocked(stdin) || sub_4070c0(stdin)) && (char)v11)
                {
                    sub_410580(4, v1);
                    error(1, 0, dcgettext(NULL, "error reading %s", 5));
                }
                if (!g_41d181)
                    return (char)v11 ^ 1;
                sub_404e30(&g_41d120, dcgettext(NULL, "total", 5));
                return (char)v11 ^ 1;
            }
        }
    }
    sub_411a40(); /* do not return */
}



// Function: duinfo_init @ 0x4a40
unsigned long long duinfo_init(unsigned long long *idx)
{
    *(idx) = 0;
    idx[1] = 0;
    idx[2] = 0x8000000000000000;
    idx[3] = 0xffffffffffffffff;
    return 0x8000000000000000;
}



// Function: duinfo_set @ 0x4a70
void duinfo_set(unsigned long long *idx, unsigned long a1, unsigned long a2, unsigned long a3)
{
    *(idx) = a1;
    idx[1] = 1;
    idx[2] = a2;
    idx[3] = a3;
    return;
}



// Function: duinfo_add @ 0x4a90
unsigned long long duinfo_add(unsigned long long *idx, unsigned long long *index)
{
    unsigned long long v1;  // rdx
    unsigned long long v2;  // rax
    unsigned long long v3;  // r12
    unsigned long long v4;  // r13
    unsigned long long v5;  // rax

    v1 = 0xffffffffffffffff;
    v2 = *(index) + *(idx);
    v3 = index[2];
    if (!(__CFADD__(*(index), *(idx)) & 1))
        v1 = v2;
    v4 = index[3];
    *(idx) = v1;
    idx[1] = idx[1] + index[1];
    v5 = sub_410ee0();
    if ((unsigned int)v5 < 0)
    {
        idx[2] = v3;
        idx[3] = v4;
    }
    return v5;
}



// Function: hash_ins @ 0x4b50
char hash_ins(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned int v1;  // eax

    v1 = sub_4062e0();
    if (v1 < 0)
        sub_411a40(); /* do not return */
    return v1;
}



// Function: fill_mount_table @ 0x4b80
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

extern unsigned int g_41d190;

void fill_mount_table(void)
{
    struct_0 *v2;  // rax
    struct_0 *v4;  // rbx
    stat v0;  // [bp-0xb8]

    v2 = sub_40d8b0(0);
    if (!v2)
        return;
    while (1)
    {
        if (!(v2->field_28 & 3) && !stat(v2->field_8, &v0))
            sub_404b50(*((long long *)&g_41d190), v0.st_ino, *((unsigned long long *)&v0));
        v4 = v2->field_30;
        sub_40d860(v2);
        if (!v2->field_30)
            break;
        v2 = v4;
    }
    return;
}



// Function: mount_point_in_fts_cycle @ 0x4c30
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
    char padding_10[96];
    long long field_70;
    long long field_78;
} struct_0;

extern long long g_41d190;

unsigned int mount_point_in_fts_cycle(struct_0 **a0)
{
    struct_0 *iter;  // rbx
    struct_0 *v2;  // rbp

    iter = a0;
    v2 = *(a0);
    if (!g_41d190)
    {
        g_41d190 = sub_406240();
        if (!g_41d190)
            sub_411a40(); /* do not return */
        sub_404b80();
    }
    if (iter == v2)
        return 0;
    do
    {
        if (sub_406330(g_41d190, iter->field_70, iter->field_78) > 0)
            return 1;
    } while ((iter = (struct_0 *)iter->field_8, iter && v2 != iter));
    return 0;
}



// Function: print_only_size @ 0x4cc0
typedef struct FILE {
} FILE;

extern FILE *stdout;

void print_only_size(unsigned long long a0)
{
    char *v1;  // rax

    v1 = (a0 == 0xffffffffffffffff ? dcgettext(NULL, "Infinity", 5) : sub_40c800());
    fputs_unlocked(v1, stdout);
    return;
}



// Function: show_date @ 0x4d60
extern void stdout;

void show_date(long long a0, unsigned long a1, unsigned int a2, long long a3)
{
    unsigned long v4;  // rax
    unsigned long v0;  // [bp-0x88]
    char v1;  // [bp-0x78]
    char v2;  // [bp-0x38]

    v0 = a1;
    if (!sub_410ce0(a3, &v0, &v1))
    {
        v4 = sub_404d50(v0, &v2);
        sub_410840(v4);
        error(0, 0, dcgettext(NULL, "time %s is out of range", 5));
        fputs_unlocked(v4, *((long long *)&stdout));
        return;
    }
    sub_408b90(*((long long *)&stdout), a0, &v1, a3, a2);
    return;
}



// Function: print_size @ 0x4e30
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
extern long long g_41d150;
extern long long g_41d158;
extern char g_41d16c;
extern char g_41d16d;
extern char g_41d182;

int print_size(struct_0 *a0, char *a1)
{
    unsigned long v1;  // rdi

    v1 = a0->field_8;
    if (!g_41d16d)
        v1 = a0->field_0;
    sub_404cc0(v1);
    if (g_41d16c)
    {
        putchar_unlocked(9);
        sub_404d60(g_41d158, *((long long *)&a0->field_10), a0->field_18, g_41d150);
    }
    __printf_chk(1, "\t%s%c", a1, (char)-(g_41d182 < 1) & 10);
    return fflush_unlocked(stdout);
}



// Function: process_file @ 0x4ec0
typedef struct struct_0 {
    char padding_0[72];
    char field_48;
} struct_0;

extern char g_41d010;
extern unsigned long long g_41d100;
extern unsigned long long g_41d108;
extern unsigned long long g_41d120;
extern long long g_41d140;
extern char g_41d168;
extern char g_41d16d;
extern char g_41d178;
extern char g_41d180;
extern char g_41d183;
extern char g_41d184;
extern char g_41d185;
extern char g_41d186;
extern unsigned long long g_41d188;
extern unsigned int g_41d198;

unsigned int process_file(struct_0 *a0, void* idx, unsigned long a2)
{
    unsigned int v6;  // ebx
    unsigned long v7;  // r14
    unsigned long long v17;  // rax
    unsigned long v18;  // rsi
    unsigned long v19;  // rdx
    unsigned long v20;  // rbp
    unsigned long v21;  // rdx
    char v22;  // r15b
    unsigned long v23;  // rax
    unsigned long long *v24;  // rcx
    unsigned long long *v25;  // r8, Other Possible Types: unsigned long
    unsigned int v8;  // r12d
    unsigned long long *v26;  // rax, Other Possible Types: unsigned long
    unsigned long long *v27;  // r9
    unsigned long v28;  // rax
    unsigned int v29;  // ebx
    struct_0 v30;  // rax
    unsigned int v9;  // r12d
    unsigned int v10;  // eax
    void* v11;  // rdi
    unsigned int v12;  // r12d
    char *v14;  // rax
    char *v15;  // rax
    char *v0;  // [bp-0xa0], Other Possible Types: unsigned long long *, unsigned long
    unsigned long long v1[2];  // [bp-0x88]
    uint128_t v2;  // [bp-0x78]
    struct_0 v3;  // [bp-0x68]
    uint128_t v4;  // [bp-0x58]

    v6 = (short)idx[104];
    v7 = (long long)idx[56];
    switch (v6)
    {
    case 4:
        sub_410580(4, v7);
        v15 = dcgettext(NULL, "cannot read directory %s", 5);
        v8 = 0;
        error(0, (int)idx[64], v15);
        v10 = *((int *)&g_41d168);
        v11 = idx + 112;
        if (!*((int *)&g_41d168))
        {
LABEL_405118:
            v17 = sub_4108b0(v11);
            if (!g_41d185)
            {
LABEL_40512a:
                v18 = (long long)idx[176] * 0x200;
LABEL_404f3e:
                sub_404a70(v1, v18, v17, v19);
                v20 = (long long)idx[88];
                memcpy(&v3, v1, 16);
                v4 = v2;
                if (g_41d108)
                {
                    v21 = g_41d188;
                    v22 = g_41d180;
                    if (g_41d188 != v20)
                    {
                        if (g_41d188 < v20)
                        {
                            if (g_41d108 <= v20)
                            {
                                v23 = sub_411770(g_41d100, v20, 128);
                                v21 = g_41d188;
                                g_41d100 = v23;
                                g_41d108 = v20 * 2;
                            }
                            v22 = g_41d180;
                            if (v20 >= v21 + 1)
                            {
                                do
                                {
                                    sub_404a40(v24);
                                    sub_404a40(v24 + 4);
                                } while (v20 >= v19);
                            }
                        }
                        else if (g_41d188 - 1 == v20)
                        {
                            v0 = &v3.padding_0[0];
                            v25 = g_41d100;
                            v26 = g_41d100 + v21 * 64;
                            sub_404a90(&v3, v26);
                            v25 = v26;
                            v22 = g_41d180;
                            v26 = v25;
                            v27 = v25 + 4;
                            if (!g_41d180)
                            {
                                v0 = v27;
                                sub_404a90(v0, v27);
                                v27 = v0;
                            }
                            v26 = v27;
                            v0 = v26 + v20 * 64 + 32;
                            sub_404a90(v0, v25);
                            sub_404a90(v0, v26);
                        }
                        else
                        {
                            __assert_fail(); /* do not return */
                        }
                    }
                }
                else
                {
                    g_41d108 = v20 + 10;
                    v28 = sub_411920(g_41d108, 64);
                    v22 = g_41d180;
                    g_41d100 = v28;
                }
                g_41d188 = v20;
                v29 = v6 & 0xfffffffd;
                if (!v22)
                {
                    sub_404a90(v20 * 64 + g_41d100, v1);
                    sub_404a90(&g_41d120, v1);
                    if (v29 != 4)
                        goto LABEL_405021;
                    break;
                }
                else if (v29 != 4)
                {
                    sub_404a90(v20 * 64 + g_41d100, v1);
                    sub_404a90(&g_41d120, v1);
LABEL_405021:
                    if (!g_41d186)
                        goto LABEL_405033;
                    break;
                }
                else
                {
                    sub_404a90(&g_41d120, v1);
                    break;
                }
                if (*((long long *)&g_41d010) >= v20)
                    goto LABEL_40503c;
LABEL_405033:
                if (v20)
                    return v8;
LABEL_40503c:
                if (!g_41d16d)
                    v30 = v3;
                if (*((long long *)&g_41d178) >= 0)
                {
                    if (*((long long *)&g_41d178) > v30)
                        return v8;
                }
                else
                {
                    if (!*((long long *)&g_41d178) < v30)
                        return v8;
                }
                sub_404e30(&v3, v7);
                return v8;
            }
LABEL_404f2e:
            v18 = (long long)idx[160];
            if ((long long)idx[160] < 0)
            {
                v18 = 0;
                goto LABEL_404f3e;
            }
            else
            {
                goto LABEL_404f3e;
            }
        }
LABEL_404f13:
        v17 = (v10 == 2 ? sub_410890() : sub_4108a0());
        if (!g_41d185)
            goto LABEL_40512a;
        goto LABEL_404f2e;
    case 6:
        v8 = 1;
        goto LABEL_404f01;
    default:
        v8 = 1;
        v9 = sub_406a90(g_41d140, v7);
        if (!(char)v9)
        {
            if (v6 == 11)
            {
                sub_40b190(a0, idx, 1);
                if (idx != sub_40ab50(a0))
                    __assert_fail(); /* do not return */
                v6 = (short)idx[104];
            }
            if (v6 == 10 || v6 == 13)
            {
                sub_410580(4, v7);
                v14 = dcgettext(NULL, "cannot access %s", 5);
                error(0, (int)idx[64], v14);
                return v9;
            }
            if (a0->field_48 & 64 && (long long)idx[88] > 0 && *((long long *)&a0->padding_0[24]) != (long long)idx[112] || !g_41d184 && (g_41d183 || ((short)(int)idx[0x88] & 0xf000) != 0x4000 && (long long)idx[128] > 1) && !sub_404b50(*((long long *)&g_41d198), (long long)idx[120], (long long)idx[112]))
                goto LABEL_4050e0;
            switch (v6)
            {
            case 2:
                if ((char)sub_411ad0(a0, idx))
                {
                    v12 = sub_404c30(idx);
                    if (!(char)v12)
                    {
                        sub_410680(0, 3, v7);
                        error(0, 0, dcgettext(NULL, "WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n", 5));
                        return v12;
                    }
                }
LABEL_4050e9:
                return 1;
            case 7:
                v8 = 0;
                sub_410680(0, 3, v7);
                error(0, (int)idx[64], "%s");
                break;
            case 1:
                v8 = 1;
                return 1;
            default:
                v8 = 1;
                return 1;
            }
        }
        else
        {
LABEL_4050e0:
            if (v6 != 1)
                return 1;
            sub_40b190(a0, idx, 4);
            if (idx != sub_40ab50(a0))
                __assert_fail(); /* do not return */
            goto LABEL_4050e9;
        }
    }
LABEL_404f01:
    v10 = *((int *)&g_41d168);
    v11 = idx + 112;
    if (!*((int *)&g_41d168))
        goto LABEL_405118;
    goto LABEL_404f13;
}



// Function: usage @ 0x58b0
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
    char *v16;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
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
        v5 = dcgettext(NULL, "Summarize device usage of the set of FILEs, recursively for directories.\n", 5);
        fputs_unlocked(v5, stdout);
        sub_404af0(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in ('sparse') files, internal\n                          fragmentation, indirect blocks, and the like\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           '-BM' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to '--apparent-size --block-size=1'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n", 5);
        fputs_unlocked(v13, stdout);
        v14 = dcgettext(NULL, "      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n", 5);
        fputs_unlocked(v14, stdout);
        v15 = dcgettext(NULL, "  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n", 5);
        fputs_unlocked(v15, stdout);
        v16 = dcgettext(NULL, "  -P, --no-dereference  don't follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n", 5);
        fputs_unlocked(v16, stdout);
        v17 = dcgettext(NULL, "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in 'date'\n", 5);
        fputs_unlocked(v17, stdout);
        v18 = dcgettext(NULL, "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n", 5);
        fputs_unlocked(v18, stdout);
        v19 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v19, stdout);
        v20 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v20, stdout);
        sub_405870(v6, v7, v8, v9, v10, v11);
        sub_404b20(v6, v7, v8, v9, v10, v11);
        sub_405640(v6, v7, v8, v9, v10, v11);
    }
    exit(a0); /* do not return */
}



// Function: is_prime @ 0xb340
char is_prime(uint128_t a0)
{
    unsigned long long v1;  // rdx
    unsigned long long v2;  // r8
    uint128_t v3;  // rsi
    unsigned long long v4;  // rcx

    v1 = a0 - (((unsigned long long)(a0 * 12297829382473034411 >> 64) & 0xfffffffffffffffe) + a0 / 3);
    if (a0 > 9 && v1)
    {
        v2 = 16;
        v3 = 9;
        v4 = 3;
        do
        {
            v4 += 2;
            v3 += v2;
            v1 = a0 % v4;
        } while (a0 > v3 && (v2 += 8, v1));
    }
    return v1;
}



// Function: next_prime @ 0xb3b0
uint128_t next_prime(unsigned long long a0, unsigned long a1)
{
    unsigned long long v1;  // rax
    uint128_t v2;  // r9
    unsigned long v3;  // rsi
    uint128_t v4;  // r9
    uint128_t v5;  // r9

    v1 = 10;
    if (10 > a0)
        a0 = v1;
    v2 = a0 | 1;
    if (v2 == 0xffffffffffffffff)
        return v5;
    while (!sub_40b340(v2, v3))
    {
        v2 = v4 + 2;
        if (v2 == 0xffffffffffffffff)
            return v4;
    }
    return v4;
}



// Function: raw_hasher @ 0xb3f0
long long raw_hasher(unsigned long a0, unsigned long long a1)
{
    return sub_412dc0() % a1;
}



// Function: raw_comparator @ 0xb410
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0xb420
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

extern unsigned int g_415a40[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_415a40[0])
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
    idx->field_28 = &g_415a40[0];
    return 0;
}



// Function: free_entry @ 0xb4b0
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_1;

struct_0 * free_entry(struct_1 *idx, struct_0 *a1)
{
    struct_0 *v1;  // rax

    v1 = idx->field_48;
    a1->field_0 = 0;
    a1->field_8 = v1;
    idx->field_48 = a1;
    return v1;
}



// Function: safe_hasher @ 0xb4d0
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
    unsigned long long v1;  // rax

    v1 = a0->field_30(a1, a0->field_10);
    if (a0->field_10 <= v1)
        abort(); /* do not return */
    return v1 * 16 + a0->field_0;
}



// Function: hash_find_entry @ 0xb500
typedef struct struct_0 {
    char padding_0[56];
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_find_entry(struct_0 *a0, unsigned long long a1, void* *a2, char a3)
{
    void* v2;  // rax
    unsigned long long v3;  // rsi
    void* iter;  // rbx
    unsigned long v5;  // rax
    void* v6;  // rsi
    void* v7;  // rsi
    unsigned long v0;  // [bp-0x30], Other Possible Types: unsigned long long

    v2 = sub_40b4d0(a0, a1);
    *(a2) = v2;
    v3 = *((long long *)v2);
    if (!v3)
    {
LABEL_40b5a1:
        return 0;
    }
    if (v3 == a1)
    {
        v5 = v3;
    }
    else if ((char)a0->field_38(a1))
    {
        v5 = *((long long *)iter);
    }
    else
    {
        for (iter = v2; (long long)iter[8]; iter = (long long)iter[8])
        {
            if (*((long long *)(long long)iter[8]) == a1 || (char)a0->field_38(a1))
            {
                v6 = (long long)iter[8];
                if (a3)
                {
                    v0 = *((long long *)v6);
                    *((long long *)&iter[8]) = (long long)v6[8];
                    sub_40b4b0(a0, v6);
                    return v0;
                }
                return *((long long *)v6);
            }
        }
        goto LABEL_40b5a1;
    }
    if (!a3)
        return v5;
    v7 = (long long)iter[8];
    if (!v7)
    {
        *((unsigned long *)iter) = 0;
        return v5;
    }
    v0 = v5;
    *((int128_t *)iter) = *((int128_t *)v7);
    sub_40b4b0(a0, v7);
    return v0;
}



// Function: allocate_entry @ 0xb600
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
    struct_0 *v1;  // rax

    v1 = idx->field_48;
    if (!v1)
        return malloc(16);
    idx->field_48 = v1->field_8;
    return v1;
}



// Function: transfer_entries @ 0xb630
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_3 {
    unsigned long long field_0;
    struct struct_3 *field_8;
} struct_3;

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

unsigned int transfer_entries(struct_1 *idx, struct_2 *idx1, char a2)
{
    struct_1 *iter;  // rbp
    unsigned long long v2;  // r15
    struct_0 *v3;  // r14
    void* idx2;  // rax
    struct_0 *v5;  // rsi
    struct_3 *v6;  // r14
    struct_3 *index;  // r14
    struct_0 *v8;  // rax
    struct_3 *v9;  // rdx

    iter = idx1->field_0;
    if (iter >= *((long long *)&idx1->padding_8[0]))
        return 1;
    while (1)
    {
        v2 = iter->field_0;
        if (!v2)
        {
LABEL_40b658:
            iter = &iter->field_10;
            if (*((long long *)&idx1->padding_8[0]) <= iter)
                return 1;
        }
        else
        {
            v3 = iter->field_8;
            if (v3)
            {
                while (1)
                {
                    idx2 = sub_40b4d0(idx, *((long long *)&v3->field_0));
                    v5 = v3;
                    v6 = v3->field_8;
                    if (*((long long *)idx2))
                    {
                        v5->field_8 = (long long)idx2[8];
                        *((struct_0 **)&idx2[8]) = v5;
                        v3 = v6;
                        if (!v3)
                            break;
                    }
                    else
                    {
                        *((long long *)idx2) = *((long long *)&v3->field_0);
                        *((unsigned long long *)((char *)&idx[1].field_0 + 7)) = *((long long *)((char *)&idx[1].field_0 + 7)) + 1;
                        sub_40b4b0(idx, v5);
                        v3 = v6;
                        if (!v3)
                            break;
                    }
                }
                v2 = iter->field_0;
            }
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_40b658;
            index = sub_40b4d0(idx, v2);
            if (index->field_0)
            {
                v8 = sub_40b600(idx);
                if (!v8)
                    return 0;
                v9 = index->field_8;
                *((unsigned long long *)&v8->field_0) = v2;
                v8->field_8 = v9;
                index->field_8 = v8;
            }
            else
            {
                index->field_0 = v2;
                *((unsigned long long *)((char *)&idx[1].field_0 + 7)) = *((long long *)((char *)&idx[1].field_0 + 7)) + 1;
            }
            iter->field_0 = 0;
            iter = &iter->field_10;
            idx1->field_18 = idx1->field_18 - 1;
            if (*((long long *)&idx1->padding_8[0]) <= iter)
                return 1;
        }
    }
}



// Function: hash_get_n_buckets @ 0xb7f0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0xb800
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0xb810
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0xb820
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_1;

unsigned long long hash_get_max_bucket_length(struct_1 **a0)
{
    struct_1 *iter;  // rcx
    struct_1 *v2;  // rsi
    unsigned long long v3;  // r8
    struct_0 *v4;  // rax
    unsigned long long v5;  // rdx
    struct_0 *v6;  // rax
    struct_0 *v7;  // rax

    iter = *(a0);
    v2 = a0[1];
    v3 = 0;
    if (*(a0) >= v2)
        return 0;
    while (1)
    {
        if (!iter->field_0)
        {
            iter = &iter->field_10;
            if (iter >= v2)
                return v3;
        }
        else
        {
            v4 = iter->field_8;
            v5 = 1;
            if (iter->field_8)
            {
                do
                {
                    v6 = v4;
                    v7 = v6->field_8;
                    v5 += 1;
                    v4 = v7;
                } while (v6->field_8);
            }
            if (v3 < v5)
                v3 = v5;
            iter = &iter->field_10;
            if (iter >= v2)
                return v3;
        }
    }
}



// Function: hash_print_statistics @ 0xb8f0
void hash_print_statistics(unsigned long a0, unsigned long a1)
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
    sub_40b8fd();
    return;
}



// Function: hash_lookup @ 0xba10
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
    struct_0 *iter;  // rax
    unsigned long long i;  // rsi

    iter = sub_40b4d0(a0, a1);
    if (!iter->field_0)
        return 0;
    for (i = iter->field_0; i != a1; i = iter->field_0)
    {
        if ((char)a0[1].field_0(a1))
            return iter->field_0;
        iter = (struct_0 *)iter->padding_8;
        if (!iter)
            return 0;
    }
    return a1;
}



// Function: hash_get_next @ 0xbac0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40bacc();
    return;
}



// Function: hash_do_for_each @ 0xbb80
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



// Function: hash_initialize @ 0xbc70
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

extern struct_0 g_415a40;

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

    v1 = sub_40b3f0;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_40b410;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_415a40.padding_0[0];
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_40b420(ptr))
    {
        v6 = sub_40b760(a0, v2->field_10);
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



// Function: hash_clear @ 0xbd80
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



// Function: hash_free @ 0xbe30
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
            goto LABEL_40bedc;
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
LABEL_40bedc:
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



// Function: hash_rehash @ 0xbf20
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

    v11 = sub_40b760(a1, idx->field_28->field_10);
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
        v13 = sub_40b630(&ptr, idx, 0);
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
        if ((char)sub_40b630(idx, &ptr, 1) && (char)sub_40b630(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0xc0a0
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

unsigned int hash_insert_if_absent(void* idx, unsigned long long a1, unsigned long long *a2)
{
    unsigned long long v2;  // rax
    unsigned long v3;  // rax
    unsigned int v4;  // ymm5
    unsigned long v5;  // rax
    uint128_t v6;  // xmm4
    struct_2 *v8;  // rax
    int v9;  // xmm0
    uint128_t v10;  // xmm4
    struct_0 *v11;  // rax
    struct_0 *index;  // [bp-0x38]

    if (!a1)
        abort(); /* do not return */
    v2 = sub_40b500(idx, a1, &index, 0);
    if (v2)
    {
        if (!a2)
            return 0;
        *(a2) = v2;
        return 0;
    }
    else
    {
        v3 = (long long)idx[24];
        v4 = (v3 < 0 ? (unsigned int)(AddV((unsigned int)(v3 >> 1 | (unsigned int)v3 & 1), (unsigned int)(v3 >> 1 | (unsigned int)v3 & 1))) : (int)v3);
        v5 = (long long)idx[16];
        v6 = (v5 < 0 ? AddV((unsigned int)(v5 >> 1 | (unsigned int)v5 & 1), (unsigned int)(v5 >> 1 | (unsigned int)v5 & 1)) : (unsigned int)v5);
        if (((CmpF(v4, (unsigned int)(MulV(*((int *)((long long)idx[40] + 8)), v6))) & 69 | (char)((CmpF(v4, (unsigned int)(MulV(*((int *)((long long)idx[40] + 8)), v6))) & 69) >> 6)) & 1) != 1)
        {
            sub_40b420(idx);
            v8 = (long long)idx[40];
            v9 = (int)v8->field_8;
            if (((CmpF(v4, *((unsigned int *)&MulV(v6, v9))) & 69 | (char)((CmpF(v4, *((unsigned int *)&MulV(v6, v9))) & 69) >> 6)) & 1) != 1)
            {
                v10 = MulV(v6, v8->field_c);
                if (!v8->field_10)
                    v10 = (uint128_t)(MulV(v10, v9));
                if (!(CmpF((unsigned int)v10, 0x5f800000) & 1))
                {
                    return 0xffffffff;
                }
                else if (!(char)sub_40bf20(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else if (sub_40b500(idx, a1, &index, 0))
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
        v11 = sub_40b600(idx);
        if (!v11)
            return 0xffffffff;
        v11->field_0 = a1;
        v11->field_8 = index->field_8;
        index->field_8 = v11;
        *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
        return 1;
    }
}



// Function: hash_insert @ 0xc290
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x18]

    v2 = sub_40c0a0(a0, a1, &v0);
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



// Function: hash_remove @ 0xc2f0
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_remove(struct_1 *idx, unsigned long long a1)
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
    unsigned long long *v0;  // [bp-0x28], Other Possible Types: void*

    v2 = sub_40b500(idx, a1, &v0, 1);
    if (!v2)
        return v2;
    idx[4].field_0 = idx[4].field_0 - 1;
    if (*(v0))
        return v2;
    v3 = idx[3].field_0;
    v4 = v3 - 1;
    idx[3].field_0 = v4;
    if (!(v3 - 1 >> 63 & 1))
    {
        v5 = v4;
        v6 = idx[2].field_0;
        if (v6 < 0)
            goto LABEL_40c436;
LABEL_40c380:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = idx[2].field_0;
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_40c380;
LABEL_40c436:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_40b420(idx);
    v9 = idx[5].field_0;
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_40bf20(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
        return v2;
    v12 = idx[9].field_0;
    if (idx[9].field_0)
    {
        do
        {
            ptr = v12;
            v14 = ptr[1].field_0;
            free(ptr);
            v12 = v14;
        } while (ptr[1].field_0);
    }
    idx[9].field_0 = NULL;
    return v2;
}



// Function: main @ 0x4ff0
typedef struct mbstate_t {
} mbstate_t;

typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_4 {
    char padding_0[8];
    struct struct_5 *field_8;
    struct struct_6 *field_10;
    char *field_18;
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
    struct struct_0 *field_0;
} struct_5;

typedef struct struct_6 {
    unsigned long long field_0;
} struct_6;

typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern unsigned int g_42255c[4];
extern unsigned int g_422788[4];
extern char g_4236a2;
extern long long g_429fa0;
extern option g_429fc0;
extern struct_4 g_42a620;
extern unsigned int g_42b010;
extern void g_42b014;
extern unsigned int g_42b018;
extern unsigned long long g_42b060;
extern unsigned long long g_42b068;
extern unsigned long long g_42b070;
extern long long g_42b078;
extern unsigned int g_42b080;
extern long long stdout;
extern struct_0 *stdin;
extern int optind;
extern unsigned long long optarg;
extern long long g_42b180;
extern char g_42b183;
extern void g_42b284;
extern void g_42b684;
extern char g_42b685;
extern char g_42b686;
extern char g_42b687;
extern char g_42b6a0;
extern char g_42b7a0;
extern char g_42b7a1;
extern char g_42b7a2;
extern unsigned long long g_42b7d0;
extern char g_42b7d8;
extern unsigned long long g_42b7e0;
extern char g_42b7e8;
extern unsigned long long g_42b7f0;
extern unsigned long long g_42b7f8;
extern char g_42b800;
extern char g_42b801;
extern char g_42b802;
extern char g_42b803;
extern unsigned int g_42b804;
extern unsigned int g_42b80c;
extern unsigned int g_42b810;
extern unsigned long long g_42b868;
extern unsigned int g_42b884;
extern unsigned long long g_42b8a0;
extern unsigned long long g_42b8a8;
extern unsigned long long g_42b8b0;
extern unsigned long long g_42b8b8;
extern long long g_42b8c8;
extern char g_42b8d0;
extern struct_0 *g_42b8d8;
extern char g_42b8fc;
extern char g_42b8fd;
extern unsigned int g_42b900;
extern char g_42b904;
extern char g_42b905;
extern unsigned int g_42b908;
extern unsigned int g_42b920;

unsigned int main(unsigned int a0, char **a1)
{
    char *v23;  // r12
    char *v24;  // rbp
    unsigned long long v33;  // rdi
    long long v127;  // rax
    int *err;  // rax
    unsigned long long v34;  // rdi
    char *fp;  // rbp
    char *v36;  // r14
    unsigned long v37;  // r12
    unsigned long v38;  // rdx
    unsigned long v39;  // r11
    unsigned long v40;  // rbx
    char *v41;  // rbp
    unsigned long count;  // rax
    char v25;  // r14b
    unsigned long v43;  // r14
    int *err1;  // rax
    unsigned int v45;  // esi
    unsigned long long v46;  // r14
    unsigned long len;  // rbx
    char *v48;  // rax
    unsigned int v49;  // eax
    int *err2;  // rax
    char *v52;  // rbx
    int v26;  // ebx
    char *v53;  // rax
    unsigned long index;  // rax
    unsigned long long v55;  // rdx
    char *v56;  // rax
    char *v57;  // rax
    char *v58;  // r15
    unsigned long len1;  // rax
    char v60;  // bpl
    char *v61;  // r12
    unsigned int v27;  // edx
    unsigned int *iter;  // rdi
    unsigned long long v66;  // rcx
    unsigned long long v67;  // 4103
    unsigned int v68;  // eax
    struct_2 *v69;  // rax
    struct_2 *v70;  // r13
    unsigned int v71;  // eax
    char *v72;  // rax
    char *v28;  // rax
    char *v73;  // rax
    char *node;  // r12
    char *v75;  // r13
    char *v76;  // rbx
    char v77;  // r14b
    struct_4 *iter1;  // rbp
    char *v79;  // rdi
    unsigned long long iter2;  // rdx
    unsigned int i;  // eax
    unsigned long long v29;  // rdi
    unsigned int v83;  // esi
    unsigned int v84;  // ecx
    void* v85;  // r13
    char *v86;  // r14
    unsigned long v87;  // rdi
    unsigned int v88;  // eax
    unsigned long long v89;  // rdx
    void* v90;  // r15
    char v91;  // al
    unsigned int v30;  // ebx
    void* v93;  // rbp
    char *v94;  // r15
    unsigned long v95;  // rbx
    unsigned long long v31;  // rdi
    char v106;  // al
    unsigned long v107;  // rbx
    char *v108;  // rdi
    unsigned long v109;  // rax
    unsigned long j;  // rbx
    char *v111;  // rdi
    unsigned long v112;  // rax
    unsigned long v114;  // rax
    int *v0;  // [bp-0x1f8], Other Possible Types: struct_2 *, char *, unsigned int
    int v1;  // [bp-0x1f0], Other Possible Types: unsigned int, unsigned long
    unsigned int v2;  // [bp-0x1e4]
    char *v3;  // [bp-0x1e0]
    char *v4;  // [bp-0x1d8], Other Possible Types: unsigned long
    int v5;  // [bp-0x1d0]
    unsigned int v6;  // [bp-0x1cc]
    char *v7;  // [bp-0x1c8], Other Possible Types: unsigned long
    unsigned long v8;  // [bp-0x1c0]
    unsigned long v9;  // [bp-0x1b0]
    unsigned long v10;  // [bp-0x1a8]
    unsigned long v11;  // [bp-0x1a0]
    mbstate_t v13;  // [bp-0x190]
    stat v14;  // [bp-0x188]
    char v15;  // [bp-0xfc]
    int v16;  // [bp-0xf8], Other Possible Types: stat, unsigned long
    char v17;  // [bp-0x68]
    char v20;  // [bp-0x53]
    char v21;  // [bp-0x50]

    g_42b684 = 10;
    v9 = 0;
    v10 = 0;
    g_42b080 = 2;
    g_42b804 = 0xffffffff;
    g_42b7e0 = 0x7fffffffffffffff;
    g_42b7f8 = 0xffffffffffffffff;
    g_42b7f0 = 0xffffffffffffffff;
    v11 = 0xffffffffffffffff;
    g_42b8fd = 0;
    setlocale(6, &g_4236a2);
    bindtextdomain("grep", "/usr/local/share/locale");
    textdomain("grep");
    sub_416210(&g_42b180);
    sub_420120(sub_407a00);
    sub_40f6a0(0);
    g_42b8c8 = sub_41a2d0(0, 0, sub_407850, sub_4078a0, 0);
    if (!g_42b8c8)
        sub_41f410(); /* do not return */
    v6 = 0;
    v4 = &v10;
    v23 = &v17;
    v5 = 0;
    v2 = 0xffffffff;
    v3 = NULL;
    while (1)
    {
        v24 = v23;
        v25 = 0;
        v1 = optind;
        v26 = optind;
        while (1)
        {
            v27 = getopt_long(a0, a1, "0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz", &g_429fc0.name, NULL);
            if (v27 - 48 > 9)
            {
                if (v24 == v23)
                    break;
                goto LABEL_4051b5;
            }
            if (g_42b010 != v26 || !v25)
            {
                v28 = v23;
            }
            else
            {
                v28 = &v24[-1 * (v17 == 48)];
                if (v28 == &v20)
                {
                    *((unsigned int *)&v28) = 0x2e2e2e;
                    v24 = &v21;
LABEL_4051b5:
                    *(v24) = 0;
                    v0 = v27;
                    sub_407d10(v23, &v11);
                    v27 = v0;
                    break;
                }
            }
            g_42b010 = v26;
            v24 = v28 + 1;
            v26 = optind;
            v25 = 1;
            *(v28) = v27;
        }
        if (v27 != 0xffffffff)
        {
            switch (v27)
            {
            case 0: case 85:
                break;
            case 65:
                sub_407d10(optarg, &g_42b7f0);
                continue;
            case 66:
                sub_407d10(optarg, &g_42b7f8);
                continue;
            case 67:
                sub_407d10(optarg, &v11);
                continue;
            case 68:
                if (!strcmp(optarg, "read"))
                {
                    g_42b884 = 1;
                    break;
                }
                else if (!strcmp(optarg, "skip"))
                {
                    g_42b884 = 2;
                    break;
                }
                else
                {
                    error(2, 0, dcgettext(NULL, "unknown devices method", 5));
                }
            case 69:
                v2 = sub_407950("egrep", v2);
                continue;
            case 70:
                v2 = sub_407950("fgrep", v2);
                continue;
            case 71:
                v2 = sub_407950("grep", v2);
                continue;
            case 72:
                v6 = 1;
                break;
            case 76:
                g_42b80c = 2;
                continue;
            case 80:
                v2 = sub_407950("perl", v2);
                continue;
            case 82:
                *((unsigned int *)&g_42b014) = 778;
            case 84:
                g_42b8fc = 1;
                continue;
            case 86:
                g_42b905 = 1;
                continue;
            case 88:
                v2 = sub_407950(optarg, v2);
                continue;
            case 90:
                g_42b804 = 0;
                continue;
            case 97:
                g_42b810 = 1;
                continue;
            case 98:
                g_42b800 = 1;
                continue;
            case 99:
                g_42b7e8 = 1;
                continue;
            case 100:
                v49 = g_422788[sub_40f3f0("--directories", optarg, &g_429fa0, &g_422788[0], 4, g_42b078, 1)];
                g_42b018 = g_422788[sub_40f3f0("--directories", optarg, &g_429fa0, &g_422788[0], 4, g_42b078, 1)];
                v5 = (g_42b018 != 3 ? v5 : v1);
                continue;
            case 101:
                v46 = optarg;
                len = strlen(optarg);
                if (len + v9 - v10 + 1 > 0)
                {
                    v48 = sub_41f1f0(v3, v4);
                    v46 = optarg;
                    v3 = v48;
                    g_42b8d8 = v3;
                }
                *((char *)mempcpy(&v3[v9], v46, len)) = 10;
                v9 = sub_407a20(v3, v9, len + v9 + 1, &g_4236a2);
                continue;
            case 102:
                if (!strcmp(optarg, "-"))
                {
                    fp = &stdin->field_0;
                }
                else
                {
                    fp = fopen(optarg, "r");
                    if (!fp)
                    {
                        err = __errno_location();
                        error(2, *(err), "%s");
                    }
                }
                v36 = v3;
                v7 = v23;
                v0 = fp;
                v1 = v9;
                v37 = v1;
                while (1)
                {
                    v38 = v10;
                    v39 = v37 - v38 + 2;
                    if (v39 > 0)
                    {
                        v38 = v10;
                        g_42b8d8 = sub_41f1f0(v36, v4, v39, -0x1, 1);
                        v36 = &g_42b8d8->field_0;
                    }
                    count = fread_unlocked(&v36[v37], 1, v38 - (v37 + 1), v0);
                    if (!count)
                        break;
                    v37 += count;
                }
                v8 = v40;
                v3 = v36;
                v43 = v37;
                v23 = v7;
                err1 = __errno_location();
                v45 = *(err1);
                if (*(v0) & 32)
                {
LABEL_405749:
                    if (v45)
                        error(2, v45, "%s");
                }
                else if (stdin != v0)
                {
                    v0 = err1;
                    if (fclose(v0))
                    {
                        v45 = *(v0);
                        goto LABEL_405749;
                    }
                }
                else
                {
                    clearerr_unlocked(v0);
                }
                if (v43 != v1 && *(&v3[v43] - 1) != 10)
                {
                    *(v41) = 10;
                    v43 = v8;
                }
                v9 = sub_407a20(v3, v1, v43, optarg);
                continue;
            case 104:
                v6 = 0xffffffff;
                continue;
            case 105: case 121:
                g_42b687 = 1;
                continue;
            case 108:
                g_42b80c = 1;
                continue;
            case 109:
                if (sub_41f450(optarg, 0, 10, &g_42b7e0, &g_4236a2) > 1)
                    error(2, 0, dcgettext(NULL, "invalid max count", 5));
                continue;
            case 110:
                g_42b801 = 1;
                continue;
            case 111:
                g_42b8fd = 1;
                continue;
            case 113:
                g_42b7a1 = 1;
                g_42b080 = 0;
                continue;
            case 114:
                g_42b018 = 3;
                v5 = v1;
                continue;
            case 115:
                g_42b904 = 1;
                continue;
            case 117:
                error(0, 0, dcgettext(NULL, "warning: --unix-byte-offsets (-u) is obsolete", 5));
                continue;
            case 118:
                g_42b802 = 1;
                continue;
            case 119:
                sub_40e570(&g_42255c[0]);
                g_42b686 = 1;
                continue;
            case 120:
                g_42b685 = 1;
                continue;
            case 122:
                g_42b684 = 0;
                continue;
            case 129:
                if (optarg)
                {
                    if (!sub_40f750(optarg, "always") || !sub_40f750(optarg, "yes") || !sub_40f750(optarg, "force"))
                    {
                        g_42b900 = 1;
                        continue;
                    }
                    else if (!(sub_40f750(optarg, "never") && sub_40f750(optarg, "no") && sub_40f750(optarg, "none")))
                    {
                        g_42b900 = 0;
                        continue;
                    }
                    else if (sub_40f750(optarg, "auto") && sub_40f750(optarg, "tty") && sub_40f750(optarg, "if-tty"))
                    {
                        g_42b908 = 1;
                        continue;
                    }
                }
                g_42b900 = 2;
                continue;
            case 130:
                sub_416480(optarg);
                <None|const 0> = g_42b8a0;
                if (!v33)
                {
                    g_42b8a0 = sub_4169d0(0);
                    v33 = g_42b8a0;
                }
                sub_416d40(v33, optarg, 0x50000000);
                <None|const 0> = g_42b8a8;
                if (!v34)
                {
                    g_42b8a8 = sub_4169d0(0);
                    v34 = g_42b8a8;
                }
                sub_416d40(v34, optarg, 0x10000000);
                continue;
            case 131: case 134:
                <None|const 0> = g_42b8b0;
                if (!v29)
                {
                    v0 = v27;
                    v27 = v0;
                    g_42b8b0 = sub_4169d0(0);
                    v29 = g_42b8b0;
                }
                v30 = (v27 == 134) * 0x20000000;
                sub_416d40(v29, optarg, v30 | 0x50000000);
                <None|const 0> = g_42b8b8;
                if (!v31)
                {
                    g_42b8b8 = sub_4169d0(0);
                    v31 = g_42b8b8;
                }
                sub_416d40(v31, optarg, v30 | 0x10000000);
                continue;
            case 132:
                if (!g_42b8b0)
                    g_42b8b0 = sub_4169d0();
                if (!sub_417290(sub_416d40))
                {
                    if (!g_42b8b8)
                        g_42b8b8 = sub_4169d0();
                    if (!sub_417290(sub_416d40))
                        continue;
                }
                err2 = __errno_location();
                error(2, *(err2), "%s");
            case 133:
                g_42b070 = optarg;
                continue;
            case 135:
                g_42b7d8 = 1;
                continue;
            case 136:
                g_42b7d0 = optarg;
                continue;
            case 137:
                g_42b687 = 0;
                continue;
            case 128:
                if (!strcmp(optarg, "binary"))
                {
                    g_42b810 = 0;
                    continue;
                }
                if (strcmp(optarg, "text"))
                {
                    if (strcmp(optarg, "without-match"))
                        error(2, 0, dcgettext(NULL, "unknown binary-files type", 5));
                    goto LABEL_4052b7;
                }
                break;
            case 73:
LABEL_4052b7:
                g_42b810 = 2;
                continue;
            default:
LABEL_4064f8:
                sub_40a170(2); /* do not return */
            }
        }
        else
        {
            v52 = v23;
            if (g_42b905)
            {
                v127 = sub_419950();
                sub_41ed40(stdout, v127, "GNU grep", "3.8", 0);
                puts(dcgettext(NULL, "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.", 5));
                return 0;
            }
            if (g_42b908)
                sub_40a170(0); /* do not return */
            v53 = v3;
            if (!v53)
            {
                index = optind;
                if ((unsigned int)index >= a0)
                    goto LABEL_4064f8;
                optind = (unsigned int)index + 1;
                v55 = 0;
                v56 = a1[index];
                if (v2 != 2 && *(v56) == 92)
                    v55 = v56[1] == 45;
                v57 = sub_41f3f0(&v56[v55]);
                v58 = v57;
                v3 = v57;
                g_42b8d8 = v57;
                len1 = strlen(v57);
                v58[len1] = 10;
                v9 = sub_407a20(v58, 0, len1 + 1, &g_4236a2);
            }
            else if (!v9)
            {
                g_42b686 = 0;
                g_42b685 = 0;
                g_42b802 = g_42b802 ^ 1;
                v9 = 1;
                *(v53) = 10;
            }
            v7 = v9;
            v4 = v7 - 1;
            v9 = v4;
            sub_41a490(g_42b8c8);
            v60 = g_42b7a1;
            if (g_42b7a1)
            {
                v60 = 0;
                goto LABEL_405965;
            }
            else
            {
                v61 = &v14;
                if (!fstat(1, &v14))
                {
                    switch (v14.st_nlink & 0xf000)
                    {
                    case 32768:
                        iter = &g_42b920;
                        for (v66 = 36; v66; v61 += 4)
                        {
                            v66 -= 1;
                            *(iter) = *((int *)&v61);
                            iter += 1;
                        }
                    case 8192:
                        if (stat("/dev/null", &v16) || v14.st_ino != v16.st_ino || *((unsigned long long *)&v14) != *((unsigned long long *)&v16))
                        {
                            v60 = 1;
                            break;
                        }
                        else
                        {
                            g_42b7a0 = 1;
                            break;
                        }
                    default:
                        goto LABEL_405f97;
                    }
                }
LABEL_405f97:
                if (g_42b7a0)
                {
LABEL_405965:
                    g_42b80c = 0;
                    goto LABEL_40596f;
                }
                else
                {
                    if (!g_42b80c)
                        break;
LABEL_40596f:
                    g_42b7e8 = 0;
                    g_42b7a2 = 1;
                    break;
                }
            }
        }
    }
    v67 = g_42b7f0;
    g_42b803 = g_42b7e8 | g_42b7a2;
    if (v67 - 0 >> 63 & 1)
        g_42b7f0 = v11;
    if (g_42b7f8 - 0 >> 63 & 1)
        g_42b7f8 = v11;
    if ((!g_42b7e0 || !v4 && g_42b802 && !g_42b685 && !g_42b686) && g_42b80c != 2)
        return 1;
    if (g_42b900 == 2)
    {
        if (!v60 || !sub_41fa50())
            v68 = 0;
        else
            v68 = isatty(1);
        g_42b900 = v68;
    }
    sub_41fa90();
    if (g_42b900)
    {
        v69 = getenv("GREP_COLOR");
        v70 = v69;
        if (v69 && v69->field_0)
        {
            while (1)
            {
                v71 = v69->field_0;
                if ((char)v71 != 59 && v71 - 48 > 9)
                    break;
                if (v69->field_1)
                {
                    v69 = &v69->field_1;
                }
                else
                {
                    g_42b060 = v70;
                    g_42b068 = v70;
                    break;
                }
            }
        }
        v72 = getenv("GREP_COLORS");
        if (v72 && *(v72))
        {
            v73 = sub_41f3f0();
            v0 = v70;
            node = v73 + 1;
            v75 = v73;
            v76 = NULL;
            while (1)
            {
                v77 = *(node - 1);
                switch (v77)
                {
                case 0: case 58:
                    *(node - 1) = 0;
                    iter1 = &g_42a620.padding_0[0];
                    v79 = "mt";
                    do
                    {
                    } while (strcmp(v79, v75) && (v79 = iter1->field_18, iter1 += 24, v79));
                    if (iter1->field_8 && v76)
                        iter1->field_8->field_0 = v76;
                    if (iter1->field_10)
                        iter1->field_10();
                    if (!v77)
                        goto LABEL_406314;
                    v75 = node;
                    v76 = NULL;
                    break;
                case 61:
                    if (v75 == node - 1 || v76)
                        goto LABEL_406314;
                    *(node - 1) = 0;
                    v76 = node;
                    break;
                default:
                    if (v76 && v77 != 59 && v77 - 48 > 9)
                        goto LABEL_406314;
                    else
                        break;
                }
                node += 1;
            }
LABEL_406314:
            v70 = v0;
        }
        if (g_42b068 == v70 || g_42b060 == v70)
            error(0, 0, dcgettext(NULL, "warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'", 5));
    }
    iter2 = 1;
    i = 1;
    v83 = 0;
    while (1)
    {
        if ((&g_42b183)[iter2] == 1 || (char)iter2 & (char)v83)
        {
            iter2 += 1;
            if (iter2 == 0x100)
                break;
        }
        else
        {
            do
            {
                v84 = i;
                i *= 2;
            } while (i <= (unsigned int)iter2);
            iter2 += 1;
            v83 |= v84;
            i = v84;
            if (iter2 == 0x100)
                break;
        }
    }
    g_42b868 = 0x101010101010101 * (char)v83;
    v1 = (v2 >= 0 ? v2 : 0);
    if (g_42b687)
    {
        v85 = &g_42b284;
        v0 = &v16;
        v86 = &g_42b6a0;
        do
        {
            v87 = *((int *)v85);
            if ((unsigned int)v87 == 0xffffffff)
                continue;
            v88 = sub_416380(v87, v0);
            if (v88 > 0)
            {
                v89 = v88 * 4;
                v90 = &v0[v89];
                do
                {
                    v13 = (mbstate_t)0;
                    if (wcrtomb(v52, *((int *)((char *)v90 - 4)), &v13) != 1)
                    {
                        v91 = 0xff;
                        goto LABEL_40619c;
                    }
                } while ((v90 -= 4, &v15 + v89 - (unsigned long long)(v88 - 1) * 4 != v90));
            }
            v91 = 1;
LABEL_40619c:
            *(v86) = v91;
        } while ((v85 += 4, v86 += 1, v85 != &g_42b684));
    }
    if (v2 != 2)
    {
        v16 = 0;
        v93 = sub_41ef30(v7);
        v94 = v3;
        v95 = v4;
    }
    else
    {
        if (!(char)g_42b180)
        {
            v106 = 0;
            if (*((long long *)&g_42b8d0) == 1)
                v106 = g_42b686;
        }
        else
        {
            v107 = 0;
            v16 = 0;
            while (1)
            {
                if (v4 <= v107)
                {
                    if (g_42b687)
                    {
                        v16 = 0;
                        for (j = 0; v4 > j; j += v114)
                        {
                            v111 = &v3[j];
                            v112 = *(v111);
                            v114 = (*((int *)((char *)&g_42b180 + 4 * v112 + 260)) == 0xffffffff ? sub_407d80(v111, v4 - j, &v16, &g_42b180) : (&g_42b6a0)[v112]);
                            if (v114 < 0)
                                goto LABEL_40663d;
                        }
                        break;
                    }
                    v106 = 0;
                    goto LABEL_4060b9;
                }
                v108 = &v3[v107];
                v109 = *((char *)&g_42b180 + *(v108) + 3);
                if (((char)v109 != 254 || (v109 = (unsigned long)(unsigned long long)sub_41ac90(v108, v4 - v107, &v16, &g_42b180), v109 > 16)) && v109 < 0)
                    break;
                v107 += v109;
            }
LABEL_40663d:
            v106 = 1;
        }
LABEL_4060b9:
        v1 = 2;
        if (v106 & 1)
        {
            sub_40a3e0(&g_42b8d8, &v9);
            v1 = 0;
            v3 = &g_42b8d8->field_0;
        }
    }
}



// Function: regex_compile @ 0x6840
typedef struct regex_t {
} regex_t;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
} struct_1;

typedef struct struct_0 {
    regex_t field_0;
    char padding_0[16];
    uint128_t field_10;
    uint128_t field_20;
    uint128_t field_30;
} struct_0;

extern char g_42b687;

unsigned long long regex_compile(struct_1 *a0, long long a1, long long a2, unsigned long a3, long long a4, long long a5, unsigned int a6)
{
    struct_0 *idx;  // rbx
    char v0;  // [bp-0x90]
    int v1;  // [bp-0x88], Other Possible Types: regex_t
    int v2;  // [bp-0x78]
    int v3;  // [bp-0x68], Other Possible Types: uint128_t
    int v4;  // [bp-0x58]

    *((unsigned long long *)&v1) = 0;
    *((unsigned long long *)&(&v1)[8]) = 0;
    if (g_42b687 || (char)a6)
    {
        *((unsigned long long *)&v3) = 0;
        *((unsigned long long *)((char *)&v3 + 8)) = 0;
        if (!(char)a6)
            goto LABEL_406950;
        re_set_syntax(a5 | 0x2000000);
        if (!re_compile_pattern(a1, a2, &v1))
        {
            regfree(&v1);
            return a6;
        }
    }
    else
    {
        *((unsigned long long *)&(&v3)[8]) = 0;
        *((unsigned long long *)&v3) = sub_41ef30(0x100);
LABEL_406950:
        re_set_syntax(a5);
        if (!re_compile_pattern(a1, a2, &v1))
        {
            idx = a3 * 64 + a0->field_10;
            idx->field_0 = v1;
            idx->field_10 = (uint128_t)v2;
            idx->field_20 = v3;
            idx->field_30 = (uint128_t)v4;
            return 1;
        }
    }
    free((unsigned long long)v3);
    if (a4 != -0x1 && *((char *)sub_409080(a4, &v0)))
    {
        error(0, 0, "%s:%td: %s");
        return 0;
    }
    error(0, 0, "%s");
    return 0;
}



// Function: dfaerror @ 0x6a10
int dfaerror(unsigned long a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_406a24(); /* do not return */
}



// Function: dfawarn @ 0x6a40
void dfawarn(unsigned long a0)
{
    error(0, 0, dcgettext(NULL, "warning: %s", 5));
    return;
}



// Function: GEAcompile @ 0x6a70
typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
} struct_1;

extern char g_400000;
extern char g_42b180;
extern char g_42b182;
extern char g_42b684;
extern char g_42b685;
extern char g_42b686;
extern char g_42b687;

struct_1 * GEAcompile(struct_0 *a0, unsigned long a1, unsigned long long a2, char a3)
{
    unsigned long long v13;  // r15
    struct_1 *idx;  // r12
    struct_0 *v23;  // r14
    struct_0 *v24;  // rbp
    char v25;  // bl
    unsigned long long v26;  // rax
    long long v27;  // r14
    struct_0 *v28;  // rbx
    struct_0 *v29;  // rdi
    struct_0 *ptr;  // rax
    unsigned int v31;  // edx
    unsigned long n;  // rbx
    long long v15;  // rdi
    char v33;  // bl
    unsigned long n1;  // r13
    unsigned long v36;  // rbx
    struct_0 *ptr1;  // rbp
    unsigned long long v38;  // rbx
    unsigned long len;  // rax
    unsigned long n2;  // r14
    unsigned long long v41;  // rbx
    char *v42;  // rsi
    long long v16;  // rsi
    unsigned long len1;  // rax
    unsigned long v44;  // r14
    unsigned long len2;  // rax
    unsigned long v46;  // r14
    unsigned long v47;  // rax
    unsigned long v48;  // r14
    char *v49;  // r13
    char *ptr2;  // rbx
    long long v51;  // rax
    unsigned long v52;  // 4114
    long long v17;  // rdx
    char *v54;  // r14
    unsigned long v55;  // rdi
    void* dst;  // rax
    long long v18;  // rcx
    long long v19;  // r8
    long long v20;  // r9
    long long v21;  // rax
    unsigned long v22;  // 4114
    long long v0;  // [bp-0xb0], Other Possible Types: unsigned int
    char v1;  // [bp-0xa8], Other Possible Types: unsigned long
    struct_0 *v2;  // [bp-0xa0], Other Possible Types: char
    unsigned long v3;  // [bp-0x98]
    struct_0 *v4;  // [bp-0x90]
    unsigned long v5;  // [bp-0x88]
    struct_0 *v6;  // [bp-0x80]
    struct_0 *i;  // [bp-0x78]
    struct_0 *v8;  // [bp-0x70]
    unsigned long v9;  // [bp-0x68]
    unsigned long v10;  // [bp-0x50]
    unsigned long v11;  // [bp-0x48]

    v13 = a2;
    v8 = a0;
    v9 = a1;
    idx = sub_41f2f0(1, 72);
    v21 = sub_415a90(v15, v16, v17, v18, v19, v20);
    v22 = g_42b687;
    *((long long *)&idx->padding_0[8]) = v21;
    if ((char)v22)
        v13 |= &g_400000;
    sub_415fa0(v21, &g_42b180, v13, (!g_42b684) * 2 | (unsigned int)v13 & 16 | 44);
    v23 = v8;
    v24 = v23;
    i = &(v23->padding_0)[v9];
    v25 = g_42b180 ^ 1 | g_42b182;
    v26 = sub_41ef10(64);
    v1 = 0;
    *((unsigned long *)&idx[1].padding_0[0]) = 0;
    idx->field_10 = v26 + 64;
    v10 = 1;
    v11 = 0;
    v5 = 0;
    v6 = NULL;
    v4 = v23;
    v27 = 0;
    do
    {
        v2 = rawmemchr(v24, 10);
        v3 = v2 - v24;
        if (!(v3 - 1 >> 63 & 1))
        {
            v28 = v3 + (char *)v24 - 1;
            v29 = v24;
            while (1)
            {
                ptr = memchr(v29, 92, v28 - v29);
                if (!ptr)
                    break;
                v31 = ptr->field_1;
                if ((char)(v31 - 49) <= 8)
                {
                    if (v24 > v4)
                    {
                        n = v24 - v4;
                        if (v5 - v11 + n > NULL)
                            v6 = sub_41f1f0(v6, &v11);
                        memcpy(&(v6->padding_0)[v5], v4, n);
                        v5 += n;
                        v33 = 1;
                        v0 = 0;
                        goto LABEL_406bfa;
                    }
                    else
                    {
                        v0 = 0;
                        v33 = 1;
                        goto LABEL_406bfa;
                    }
                }
                if (v31 == (!(v25 < 1) & 36) + 92 && (ptr += 1, ptr == v28))
                    break;
                v29 = &ptr->field_1;
            }
        }
        v0 = 1;
        v33 = 0;
LABEL_406bfa:
        if (*((long long *)&idx[1].padding_0[0]) - v10 + 2 > NULL)
            idx->field_10 = sub_41f1f0(idx->field_10 - 64, &v10) + 64;
        re_set_syntax(v13);
        v24 = &v2->field_1;
        v27 += 1;
        v1 = (!(char)sub_406840(idx, v24, v3, *((long long *)&idx[1].padding_0[0]), v27, v13, v0) ? 1 : v1);
        if (v33)
        {
            *((unsigned long long *)&idx[1].padding_0[0]) = *((long long *)&idx[1].padding_0[0]) + 1;
            v4 = v24;
        }
    } while (i >= v24);
    if (v1)
        exit(2); /* do not return */
    if (v4 <= i)
    {
        if (v4 <= v8)
        {
            v5 = v9;
            v6 = v8;
        }
        else
        {
            n1 = i - v4;
            v36 = v5 + n1;
            v6 = sub_41efa0(v6, v36);
            memcpy(&(v6->padding_0)[v5], v4, n1);
            v5 = v36;
        }
    }
    if (g_42b686 || (ptr1 = NULL, g_42b685))
    {
        v38 = (unsigned int)v13 & 0x2000;
        ptr1 = sub_41ef30(v9 + 45);
        if (g_42b685)
        {
            if (v38)
            {
                *((unsigned short *)ptr1->padding_0) = 10334;
                ptr1[1].padding_0[0] = 0;
                len = strlen(ptr1);
                n2 = v9;
                v41 = len + n2;
                memcpy(&(ptr1->padding_0)[len], v8, n2);
                v42 = ")$";
            }
            else
            {
                *((unsigned int *)&ptr1) = 2645086;
                len1 = strlen(ptr1);
                v44 = v9;
                v41 = len1 + v44;
                memcpy(&(ptr1->padding_0)[len1], v8, v44);
                v42 = "\\)$";
            }
        }
        else
        {
            if (!v38)
            {
                *((uint128_t *)&ptr1) = (uint128_t)123921576912308382598422666524479072348;
                *((unsigned int *)&ptr1[8]) = 693919071;
                *((unsigned short *)ptr1[10].padding_0) = 10332;
                ptr1[11].padding_0[0] = 0;
                len2 = strlen(ptr1);
                v46 = v9;
                v41 = len2 + v46;
                memcpy(&(ptr1->padding_0)[len2], v8, v46);
                v42 = "\\)\\([^[:alnum:]_]\\|$\\)";
            }
            else
            {
                *((uint128_t *)&ptr1) = (uint128_t)124113362702760382088434473634461670952;
                *((unsigned short *)ptr1[8].padding_0) = 10281;
                ptr1[9].padding_0[0] = 0;
                v47 = strlen(ptr1);
                v48 = v9;
                v41 = v47 + v48;
                memcpy(&(ptr1->padding_0)[v47], v8, v48);
                v42 = ")([^[:alnum:]_]|$)";
            }
        }
        v8 = ptr1;
        v9 = strlen(strcpy(&(ptr1->padding_0)[v41], v42)) + v41;
    }
    sub_414cf0(v8, v9, *((long long *)&idx->padding_0[8]));
    v49 = sub_415020(*((long long *)&idx->padding_0[8]));
    if (v49)
    {
        ptr2 = v49 + 3;
        v51 = sub_40e5c0(0);
        v52 = *(v49);
        *((long long *)&idx->padding_0[0]) = v51;
        if (!(char)v52)
        {
            sub_40c9d0(v51, ptr2, strlen(ptr2));
        }
        else
        {
            *((unsigned long long *)&idx[2].padding_0[8]) = *((long long *)&idx[2].padding_0[8]) + 1;
            v1 = strlen(ptr2);
            v0 = v49[1] + v1 + v49[2];
            v54 = sub_41ef30(v0);
            *(v54) = g_42b684;
            v55 = v49[1];
            v2 = g_42b684;
            *((char *)&idx[2].field_10) = (char)idx[2].field_10 | (char)v55;
            dst = memcpy(&v54[v55], ptr2, v1);
            if (v49[2])
                *((char *)dst + v1) = v2;
            sub_40c9d0(*((long long *)&idx->padding_0[0]), v54, v0);
            free(v54);
        }
        sub_40cec0(*((long long *)&idx->padding_0[0]));
        sub_415a80(v49);
    }
    sub_415aa0(0, 0, *((long long *)&idx->padding_0[8]), 1);
    if (!v6)
    {
        free(ptr1);
        return idx;
    }
    if (a3 || !(char)sub_414e30(*((long long *)&idx->padding_0[8])))
    {
        idx->field_10 = idx->field_10 - 64;
        *((unsigned long long *)&idx[1].padding_0[0]) = *((long long *)&idx[1].padding_0[0]) + 1;
        if (!(char)sub_406840(idx, v6, v5, 0, -0x1, v13, 0))
            abort(); /* do not return */
    }
    if (v8 != v6)
        free(v6);
    free(ptr1);
    return idx;
}



// Function: EGexecute @ 0x7110
typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

typedef struct struct_2 {
    char padding_0[56];
    char field_38;
} struct_2;

typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    char padding_20[16];
    struct struct_1 *field_30;
    long long field_38;
    char field_40;
} struct_0;

extern char g_42b180;
extern char g_42b182;
extern char g_42b684;
extern char g_42b685;
extern char g_42b686;

long long EGexecute(struct_0 *a0, unsigned long a1, unsigned long a2, long long *a3, unsigned long i)
{
    struct_0 *idx;  // r14
    unsigned long v22;  // rbx
    unsigned int v31;  // esi
    unsigned long v32;  // r11
    unsigned long v33;  // r15
    long long v34;  // rax
    long long v35;  // rax
    unsigned long v37;  // rax
    unsigned int v38;  // r13d
    unsigned long v39;  // rax
    unsigned int v40;  // r12d
    unsigned long v23;  // rbx
    unsigned long v41;  // rax
    unsigned long v42;  // rbx
    struct_0 *v43;  // r12
    unsigned long v44;  // r14
    long long iter;  // rbx
    unsigned int v46;  // r13d
    unsigned long long v47;  // rbp
    struct_2 *idx1;  // rdi
    unsigned int v49;  // eax
    unsigned long v50;  // r9
    unsigned long v24;  // r15
    long long v51;  // rcx
    unsigned long v52;  // rbx
    unsigned long v53;  // rbp
    long long v54;  // r14
    struct_2 *index;  // rdi
    unsigned int v56;  // eax
    struct_2 *idx2;  // rdi
    unsigned int v58;  // eax
    unsigned long v25;  // rax
    long long v26;  // rbx
    long long v27;  // r12
    unsigned long v28;  // rbp
    unsigned long v29;  // r13
    unsigned long v30;  // rax
    long long v0;  // [bp-0xf8], Other Possible Types: unsigned int, unsigned long
    unsigned int v1;  // [bp-0xec]
    unsigned long v2;  // [bp-0xe0]
    unsigned long v3;  // [bp-0xe0]
    long long v4;  // [bp-0xd8]
    unsigned long v5;  // [bp-0xd0]
    long long v6;  // [bp-0xc8]
    unsigned long long v7;  // [bp-0xc0]
    long long v8;  // [bp-0xb8]
    unsigned long v9;  // [bp-0xb0]
    unsigned long v10;  // [bp-0xa8]
    long long v11;  // [bp-0xa0]
    long long v12;  // [bp-0x90]
    unsigned int v13;  // [bp-0x88]
    char v14;  // [bp-0x82]
    char v15;  // [bp-0x81]
    char v16;  // [bp-0x69]
    unsigned long v17;  // [bp-0x68]
    unsigned long long v18;  // [bp-0x60]
    long long v19;  // [bp-0x58]

    idx = a0;
    v22 = a2 + a1;
    v14 = g_42b684;
    v12 = sub_414e10(a0->field_8);
    v17 = a1;
    v15 = sub_414e20(a0->field_8);
    v10 = v22;
    if (a1 >= v22)
        return -0x1;
    v3 = a1;
    while (i)
    {
        v33 = v3;
        v9 = i;
        v2 = v10;
LABEL_4073d9:
        v11 = v2 - v33;
        if (v11 > 0x80000000)
        {
            sub_41f410(); /* do not return */
        }
        else if (idx->field_18 > 0)
        {
            v41 = v2;
            v42 = v9;
            v43 = idx->padding_20;
            v4 = 0;
            v0 = v41;
            v5 = v41 - 1;
            v44 = v33;
            v8 = v41 - v42;
            v1 = (int)v8 - 1;
            iter = 0;
            v13 = (v42 & 0xffffffff) - v33;
            v46 = (unsigned int)v11 - 1;
            while (1)
            {
                v47 = iter * 64;
                idx1 = idx->field_10 + v47;
                idx1->field_38 = (g_42b684 == 10) * 128 | idx1->field_38 & 63;
                v49 = re_search(idx1, v44, v46, v13, v1, v43);
                if (v49 < 0xffffffff)
                    sub_41f410(); /* do not return */
                if (v49 != 0xffffffff && (v50 = (long long)(int)v49 + v44, v0 >= v50))
                {
                    v51 = idx->field_30->field_0 - v49;
                    if (i)
                    {
                        if (!g_42b686)
                            goto LABEL_407616;
                        if (g_42b685)
                        {
                            if (v8 - 1 != v51)
                                goto LABEL_407528;
                            v51 = v8;
                            v50 = v9;
                            if (v0 <= v50)
                            {
                                if (v0 != v50)
                                    goto LABEL_407528;
                            }
                            else
                            {
LABEL_407668:
                                v4 = v51;
                                v0 = v50;
                                goto LABEL_407528;
                            }
LABEL_407622:
                            if (v51 <= v4)
                                v51 = v4;
                            v4 = v51;
                            goto LABEL_407528;
                        }
                    }
                    else if (g_42b685)
                    {
                        if (v51 == v8 - 1)
                        {
                            *(a3) = v11;
                            return v44 - a1;
                        }
                        iter += 1;
                        if (idx->field_18 <= iter)
                            break;
                        else
                            continue;
                    }
                    else if (!g_42b686)
                    {
                        *(a3) = v11;
                        return v44 - a1;
                    }
                    v6 = iter;
                    v52 = v50;
                    v7 = v47;
                    v54 = v51;
                    while (1)
                    {
                        if (!sub_40e820(v52 + v54, v5) && !sub_40e830(v53, v52, v5))
                        {
                            v51 = v54;
                            v50 = v52;
                            iter = v6;
                            if (!i)
                            {
                                *(a3) = v11;
                                return v53 - a1;
                            }
LABEL_407616:
                            if (v0 > v50)
                                goto LABEL_407668;
                            if (v0 != v50)
                                goto LABEL_407528;
                            goto LABEL_407622;
                        }
                        if (v54 > 0)
                        {
                            index = v7 + idx->field_10;
                            index->field_38 = index->field_38 | 64;
                            v56 = re_match(index, v53, (unsigned int)(v52 + v54 - 1) - (unsigned int)v9, (v52 & 0xffffffff) - v53, v43);
                            if (v56 < 0xffffffff)
                                sub_41f410(); /* do not return */
                            v54 = v56;
                            if (v56 > 0)
                                goto LABEL_407510;
                        }
                        if (v5 == v52)
                            goto LABEL_40751a;
                        idx2 = v7 + idx->field_10;
                        idx2->field_38 = idx2->field_38 & 191;
                        v58 = re_search(idx2, v53, v46, v52 + 1 - v53, v2 - (v52 + 1) - 1 & 0xffffffff, v43);
                        if (v58 < 0)
                            break;
                        v54 = idx->field_30->field_0 - v58;
                        v52 = v53 + v58;
LABEL_407510:
                        if (v0 < v52)
                        {
LABEL_40751a:
                            iter = v6;
                            goto LABEL_407528;
                        }
                    }
                    iter = v6;
                    if (v58 != 0xffffffff)
                        sub_41f410(); /* do not return */
                }
LABEL_407528:
                iter += 1;
                if (idx->field_18 <= iter)
                    break;
            }
            if (v2 > v0)
            {
                v11 = v4;
                *(a3) = v11;
                return v0 - a1;
            }
        }
LABEL_4075e2:
        v3 = v2;
        if (v3 >= v10)
            return -0x1;
    }
    v16 = 0;
    v18 = 0;
    v23 = v10;
    if (idx->field_0)
    {
        v24 = v3;
        v25 = idx->field_40;
        v26 = v23 - v24;
        v27 = sub_40e3e0(idx->field_0, v24 - v25, v25 + v26, &v19, 1);
        if (v27 < 0)
            return v27;
        v28 = a1;
        v29 = v24 + v27;
        v0 = v14;
        v30 = memrchr(v28, v0, v29 - v28);
        v31 = v0;
        v32 = v30 + 1;
        if (!v30)
            v32 = v28;
        v33 = v32;
        if (v15 == 1 && v19 >= idx->field_38)
        {
            v34 = v29 - v32;
            v35 = (v34 < 16 ? 16 : v34);
            if (v35 < v27 >> 2)
            {
                v3 = v33;
                v23 = rawmemchr(v29, v31) + 1;
            }
            else if (v35 < v26 >> 2)
            {
                v2 = v33;
                v23 = rawmemchr(v3 + v35 * 4, v31) + 1;
                v3 = v2;
            }
            else
            {
                v3 = v32;
                v23 = v10;
            }
        }
        else
        {
            v0 = v19;
            v2 = rawmemchr(v29, v31) + 1;
            if (v0 < idx->field_38)
            {
                if (g_42b180 != 1)
                {
                    v11 = v2 - v33;
                    *(a3) = v11;
                    return v33 - a1;
                }
                else if (!g_42b182)
                {
                    if (v17 < v33)
                        v17 = v33;
                    if (!sub_40e640(&v17, 0, v29, v10))
                    {
                        v11 = v2 - v33;
                        *(a3) = v11;
                        return v33 - a1;
                    }
                }
                else
                {
                    v11 = v2 - v33;
                    *(a3) = v11;
                    return v33 - a1;
                }
LABEL_407335:
                v39 = sub_414e00(idx->field_8);
                if (!v39 || v2 == v39)
                    goto LABEL_4075e2;
                v40 = v14;
                if (v18)
                    v33 = memrchr(a1, v40, v39 - a1) + 1;
                v2 = rawmemchr(v39, v40) + 1;
                if (!v16)
                {
                    v11 = v2 - v33;
                    *(a3) = v11;
                    return v33 - a1;
                }
                v9 = v33;
                goto LABEL_4073d9;
            }
            else
            {
                v3 = v33;
                v23 = v2;
            }
        }
    }
    if (v12)
    {
        v37 = sub_414e00(v12, v3, v23, 0, &v18, 0);
        if (v37 && v23 != v37)
        {
            v38 = v14;
            if (v18)
                v3 = memrchr(a1, v38, v37 - a1) + 1;
            v18 = 0;
            v2 = rawmemchr(v37, v38) + 1;
            v33 = v3;
            goto LABEL_407335;
        }
        else
        {
            v3 = v23;
            if (v3 >= v10)
                return -0x1;
            continue;
        }
    }
    else
    {
        v2 = v23;
        v33 = v3;
        goto LABEL_407335;
    }
}



// Function: hash_pattern @ 0x7850
extern unsigned long long g_42b8d8;

long long hash_pattern(unsigned long a0, unsigned long long a1)
{
    void* v1;  // rcx
    unsigned long long v2;  // rax
    unsigned long v3;  // rdx

    v1 = g_42b8d8 + a0 - 1;
    v2 = 4123221751654370051;
    v3 = *((char *)v1);
    if ((char)v3 != 10)
    {
        do
        {
            v1 += 1;
            v2 = v2 * 33 ^ v3;
            v3 = *((char *)v1);
        } while ((char)v3 != 10);
    }
    return v2 % a1;
}



// Function: compare_patterns @ 0x78a0
extern unsigned long long g_42b8d8;

unsigned int compare_patterns(unsigned long a0, unsigned long a1)
{
    void* v1;  // rcx
    unsigned long long v2;  // rsi
    char v3;  // dl
    unsigned long long v4;  // rax
    unsigned long long v5;  // rax

    v1 = g_42b8d8 + a0 - 1;
    v2 = g_42b8d8 + a1 - 1;
    v3 = *((char *)v1);
    if (*((char *)v1) != *((char *)v2))
        return 0;
    v4 = 1;
    while (1)
    {
        v5 = v4;
        if (v3 == 10)
            return 1;
        v3 = *((char *)v1 + v5);
        v4 = v5 + 1;
        if (*((char *)v1 + v5) != *((char *)(v2 + v4 - 1)))
            return 0;
    }
}



// Function: color_cap_mt_fct @ 0x78f0
extern unsigned long long g_42b060;
extern unsigned long long g_42b068;

void color_cap_mt_fct(void)
{
    g_42b060 = g_42b068;
    return;
}



// Function: color_cap_rv_fct @ 0x7910
extern unsigned int g_42b900;

void color_cap_rv_fct(void)
{
    g_42b900 = 0xffffffff;
    return;
}



// Function: color_cap_ne_fct @ 0x7920
extern char g_42109e;
extern char g_4210a4;
extern unsigned long long g_42b020;
extern unsigned long long g_42b028;

void color_cap_ne_fct(void)
{
    g_42b028 = &g_42109e;
    g_42b020 = &g_4210a4;
    return;
}



// Function: setmatcher @ 0x7950
int setmatcher(char *a0, unsigned int a1)
{
    unsigned long long v1;  // rbx

    v1 = 0;
    while (strcmp(a0, 32 * v1 + 4368288))
    {
        if (v1 == 6)
            error(2, 0, dcgettext(NULL, "invalid matcher %s", 5));
    }
    if (a1 < 0)
        return v1;
    if ((unsigned int)v1 == a1)
        return v1;
    error(2, 0, dcgettext(NULL, "conflicting matchers specified", 5));
}



// Function: update_patterns @ 0x7a20
extern long long g_42b8c8;
extern unsigned long long g_42b8d0;
extern unsigned long long g_42b8e0;
extern long long g_42b8e8;
extern unsigned long long g_42b8f0;

long long update_patterns(unsigned long a0, long long a1, unsigned long a2, unsigned long a3)
{
    void* v3;  // r13
    void* iter;  // rbx
    unsigned long long *idx;  // rdx
    int v5;  // r14d
    unsigned long long v6;  // rbp
    void* v7;  // rsi
    unsigned long n;  // r12
    long long v9;  // r15
    unsigned long v10;  // rax
    unsigned long v11;  // rax
    unsigned long v12;  // rdi
    int v0;  // [bp-0x4c]
    void* v1;  // [bp-0x48]

    v3 = a0 + a1;
    v1 = a2 + a0;
    if (v3 >= v1)
        return a1;
    iter = v3;
    v5 = 0;
    v6 = 1;
    while (1)
    {
        v7 = v3;
        v3 = rawmemchr(v3, 10) + 1;
        n = v3 - v7;
        memmove(iter, v7, n);
        v9 = iter - a0;
        v0 = v5;
        v5 = sub_41a700(g_42b8c8, v9 + 1, 0);
        if (v5)
        {
            if (v5 >= 0)
            {
                iter += n;
                if (v0)
                {
                    v10 = g_42b8d0;
                }
                else
                {
                    v11 = g_42b8e0;
                    v12 = g_42b8f0;
                    if (g_42b8e0 == g_42b8e8)
                    {
                        g_42b8f0 = sub_41f1f0(v12, &g_42b8e8, 1, -0x1, 24);
                        v12 = g_42b8f0;
                        v11 = g_42b8e0;
                    }
                    g_42b8e0 = v11 + 1;
                    idx = v12 + v11 * 24;
                    v10 = g_42b8d0;
                    idx[1] = a3;
                    *(idx) = g_42b8d0;
                    idx[2] = v6;
                }
                v9 = iter - a0;
                g_42b8d0 = v10 + 1;
            }
            else
            {
                sub_41f410(); /* do not return */
            }
        }
        v6 += 1;
        if (v3 >= v1)
            return v9;
    }
}



// Function: printf_errno @ 0x7b80
extern long long stdout;
extern unsigned int g_42b8c0;

void printf_errno(long long a0)
{
    unsigned long long v12;  // rsi
    unsigned long long v13;  // rdx
    unsigned long long v14;  // rcx
    unsigned long long v15;  // r8
    unsigned long long v16;  // r9
    unsigned long v18;  // fs
    unsigned int v0;  // [bp-0xd8]
    unsigned int v1;  // [bp-0xd4]
    char *v2;  // [bp-0xd0]
    char *v3;  // [bp-0xc8]
    unsigned long v4;  // [bp-0xc0]
    char v5;  // [bp-0xb8]
    unsigned long long v6;  // [bp-0xb0]
    unsigned long long v7;  // [bp-0xa8]
    unsigned long long v8;  // [bp-0xa0]
    unsigned long long v9;  // [bp-0x98]
    unsigned long long v10;  // [bp-0x90]
    unsigned long v11;  // [bp+0x8]

    v6 = v12;
    v7 = v13;
    v8 = v14;
    v9 = v15;
    v10 = v16;
    v4 = *((long long *)(40 + v18));
    v0 = 8;
    v2 = &v11;
    v1 = 48;
    v3 = &v5;
    if (__vfprintf_chk(stdout, 1, a0, &v0) < 0)
        g_42b8c0 = *(__errno_location());
    if (v4 != *((long long *)(40 + v18)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: nlscan @ 0x7c90
extern char g_42b684;
extern void* g_42b7c0;
extern unsigned long long g_42b818;

void* nlscan(void* a0)
{
    void* v1;  // rdi
    unsigned long v2;  // r13
    unsigned long long v3;  // rbx
    void* ptr;  // rax

    v1 = g_42b7c0;
    v2 = g_42b818;
    if (g_42b7c0 < a0)
    {
        v3 = 0;
        do
        {
            ptr = memchr(v1, g_42b684, a0 - v1);
        } while (ptr && (v1 = ptr + 1, v3 += 1, a0 > v1));
        v2 = v3 + g_42b818;
        if (__OFADD__(v3, g_42b818) & 1)
            sub_407c60(); /* do not return */
    }
    g_42b818 = v2;
    g_42b7c0 = a0;
    return ptr;
}



// Function: print_sep @ 0x7ec0
typedef struct struct_0 {
    char field_0;
} struct_0;

extern long long g_42b020;
extern long long g_42b028;
extern struct_0 *g_42b040;
extern unsigned int g_42b900;

char * print_sep(char a0)
{
    if (g_42b900 && g_42b040->field_0)
        sub_41faa0(g_42b028);
    sub_407e70(a0);
    if (!g_42b900)
    {
        return g_42b900;
    }
    else if (!g_42b040->field_0)
    {
        return g_42b040;
    }
    else
    {
        return sub_41fac0(g_42b020);
    }
}



// Function: print_offset @ 0x7f20
extern long long g_42b020;
extern long long g_42b028;
extern unsigned int g_42b900;

long long print_offset(unsigned long long a0, char *a1)
{
    unsigned long long v0;  // [bp-0x10]

    if (g_42b900 && *(a1))
    {
        v0 = a0;
        sub_41faa0(g_42b028);
    }
    sub_407b80("%*ld");
    if (g_42b900 && *(a1))
        return sub_41fac0(g_42b020);
    return g_42b900;
}



// Function: print_filename @ 0x7fa0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern long long g_42b020;
extern long long g_42b028;
extern struct_0 *g_42b058;
extern FILE *stdout;
extern unsigned long long g_42b890;
extern unsigned int g_42b8c0;
extern unsigned int g_42b900;

char * print_filename(void)
{
    unsigned long long v1;  // rdi

    if (!g_42b900 || !g_42b058->field_0)
    {
        v1 = g_42b890;
        if (!g_42b890)
            goto LABEL_408030;
    }
    else
    {
        sub_41faa0(g_42b028);
        v1 = g_42b890;
        if (!g_42b890)
        {
LABEL_408030:
            v1 = dcgettext(v1, "(standard input)", 5);
            g_42b890 = v1;
        }
    }
    if (fputs_unlocked(v1, stdout) < 0)
        g_42b8c0 = *(__errno_location());
    if (!g_42b900)
    {
        return g_42b900;
    }
    else if (!g_42b058->field_0)
    {
        return g_42b058;
    }
    else
    {
        return sub_41fac0(g_42b020);
    }
}



// Function: print_line_head @ 0x8050
extern char *g_42b048;
extern char *g_42b050;
extern unsigned long long g_42b7c0;
extern unsigned long long g_42b7c8;
extern char g_42b800;
extern char g_42b801;
extern unsigned int g_42b804;
extern char g_42b808;
extern unsigned int g_42b810;
extern unsigned long long g_42b818;
extern unsigned long long g_42b848;
extern unsigned long long g_42b868;
extern char g_42b888;
extern char g_42b8fc;

unsigned long long print_line_head(void* a0, unsigned long long a1, unsigned long a2, char a3)
{
    void* v4;  // r15
    unsigned long long v5;  // rdx
    void* iter;  // r13
    long long v7;  // rax
    unsigned long v8;  // rdi
    unsigned long long v9;  // rbx
    unsigned int v10;  // edx
    unsigned long v0;  // [bp-0x58]
    char v1;  // [bp-0x49]
    unsigned long v2;  // [bp-0x48]

    v0 = a2;
    if (g_42b810 != 1)
    {
        v4 = a0 + a1;
        v5 = g_42b868;
        v1 = *((char *)v4);
        if (g_42b868)
        {
            *((char *)v4) = 0xff;
            v2 = 0;
            iter = a0;
            while (1)
            {
                if (!(*((char *)&iter) & 7))
                {
                    for (; !(v5 & *((long long *)iter)); iter += 8);
                    for (; !(v5 & *((char *)iter)); iter += 1);
                }
                else if (!(v5 & *((char *)iter)))
                {
                    iter += 1;
                    continue;
                }
                if (v4 > iter)
                {
                    v7 = sub_41ac90(iter, v4 - iter, &v2);
                    if (v7 > 16 && v7 < 0)
                    {
                        g_42b888 = 1;
                        *((char *)v4) = v1;
                        return 0;
                    }
                    v5 = g_42b868;
                    iter += v7;
                }
                else
                {
                    *((char *)v4) = v1;
                    break;
                }
            }
        }
    }
    if (*((int *)&g_42b808))
    {
        sub_407fa0();
        if (g_42b804)
            sub_407ec0(a3);
        else
            sub_407e70(0);
    }
    if (g_42b801)
    {
        v8 = g_42b818;
        if (g_42b7c0 < v0)
        {
            sub_407c90(a0);
            v8 = g_42b818 + 1;
            if (__OFADD__(g_42b818, 1) & 1)
                sub_407c60(v8); /* do not return */
            g_42b818 = v8;
            g_42b7c0 = v0;
        }
        sub_407f20(v8, g_42b050);
        sub_407ec0(a3);
    }
    if (g_42b800)
    {
        v9 = a0 - g_42b848;
        v8 = v9 + g_42b7c8;
        if (__OFADD__(v9, g_42b7c8) & 1)
            sub_407c60(v8); /* do not return */
        sub_407f20(v8, g_42b048);
        sub_407ec0(a3);
    }
    if (!g_42b8fc)
        return 1;
    v10 = g_42b801 | g_42b800;
    if ((v10 || *((int *)&g_42b808)) & a1)
    {
        *((char *)&v0) = (v10 || *((int *)&g_42b808)) & a1;
        sub_407e70(9);
        return (char)v0;
    }
    return g_42b8fc;
}



// Function: prline @ 0x82a0
typedef struct struct_6 {
    unsigned long long field_0;
} struct_6;

typedef struct struct_5 {
    struct struct_0 *field_0;
} struct_5;

typedef struct struct_4 {
    char padding_0[8];
    struct struct_5 *field_8;
    struct struct_6 *field_10;
    char *field_18;
} struct_4;

typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern long long g_42b020;
extern long long g_42b028;
extern struct_0 *g_42b030;
extern struct_0 *g_42b038;
extern struct_0 *g_42b060;
extern struct_0 *g_42b068;
extern FILE *stdout;
extern char g_42b684;
extern unsigned long long g_42b7b8;
extern char g_42b7d8;
extern char g_42b802;
extern unsigned long long g_42b870;
extern struct_4 *g_42b878;
extern unsigned int g_42b8c0;
extern char g_42b8fd;
extern char g_42b900;

void prline(void* a0, void* a1, char a2)
{
    void* v5;  // rbp
    void* i;  // rbx
    long long v15;  // rax
    void* v16;  // r15
    char v17;  // al
    unsigned long long v18;  // rax
    long long v19;  // r13
    unsigned int v20;  // ebp
    char v7;  // al
    char v8;  // al
    char v9;  // dil
    char v10;  // cl
    char *v11;  // r9
    char *v12;  // rdx
    void* v13;  // rbp
    void* v14;  // r13
    void* v0;  // [bp-0x60]
    char *v1;  // [bp-0x58]
    char *v2;  // [bp-0x50]
    unsigned long v3;  // [bp-0x48]

    v5 = a0;
    i = a1;
    v7 = g_42b8fd;
    if (!g_42b8fd)
    {
        v8 = sub_408050(a0, a1 - a0 - 1, i, a2);
        v7 = g_42b8fd;
        if (!v8)
            return;
    }
    v9 = a2 == 58;
    v10 = (v9 ^ g_42b802) & v7;
    if (*((int *)&g_42b900))
    {
        v11 = &g_42b068->field_0;
        v12 = &g_42b038->field_0;
        if (v9 == ((char)(*((int *)&g_42b900) >> 31) & g_42b802))
            v12 = &g_42b030->field_0;
        if (a2 != 58)
            v11 = &g_42b060->field_0;
        v2 = v12;
        v1 = v11;
        if (v10)
            goto LABEL_4084c2;
        if (!*(v12))
        {
            if (!*(v11))
                goto LABEL_4082fd;
            if (v9 != g_42b802)
                goto LABEL_4083b0;
        }
        else
        {
            if (v9 != g_42b802)
            {
                v13 = v5;
                if (!*(v1))
                    goto LABEL_4083f7;
LABEL_4083b0:
                if (i > v5)
                    goto LABEL_4084cb;
LABEL_4083c0:
                if (!v5)
                    return;
                v7 = g_42b8fd;
            }
        }
        if (v7)
        {
LABEL_408318:
            if (!g_42b7d8 || !fflush_unlocked(stdout))
            {
                v20 = g_42b8c0;
            }
            else
            {
                v20 = *(__errno_location());
                g_42b8c0 = v20;
            }
            if (!v20)
            {
                g_42b7b8 = i;
                return;
            }
            error(2, v20, dcgettext(NULL, "write error", 5));
        }
        else
        {
            v17 = *(v2);
            v13 = v5;
            goto LABEL_408650;
        }
        goto LABEL_408650;
    }
    else if (!v10)
    {
LABEL_4082fd:
        if (v5 < i && v7 != 1)
        {
            sub_407e40(v5, i - v5);
            goto LABEL_408318;
        }
    }
    else
    {
        v1 = NULL;
        v2 = NULL;
LABEL_4084c2:
        if (i <= v5)
            goto LABEL_4086b0;
LABEL_4084cb:
        v13 = NULL;
        v0 = i - v5;
        v14 = v5;
        do
        {
            v15 = g_42b878(g_42b870, v5, v0, &v3, v14);
            if (v15 < NULL || !(v16 = v5 + v15, i != v16))
                break;
            if (!v3)
            {
                v3 = 1;
                v3 = 1;
                if (!v13)
                    v13 = v14;
            }
            else
            {
                if (!g_42b8fd)
                {
                    if (*(v2))
                        sub_41faa0(g_42b028, v2);
                    v13 = NULL;
                    sub_407e40();
                }
                else if (!(char)sub_408050(v16, v3, i, ((char)-(g_42b802 < 1) & 13) + 45))
                {
                    return;
                }
                if (*((int *)&g_42b900) && *(v1))
                    sub_41faa0(g_42b028, v1);
                sub_407e40(v16, v3);
                if (*((int *)&g_42b900) && *(v1))
                    sub_41fac0(g_42b020);
                if (g_42b8fd)
                    sub_407e70(g_42b684);
                v3 = v3;
            }
            v14 = v16 + v3;
        } while (i > v14);
        v5 = v14;
        if (g_42b8fd)
        {
LABEL_4086b0:
            v5 = i;
            goto LABEL_4083c0;
        }
        else if (v13)
        {
            v17 = *(v2);
LABEL_408650:
            if (!v17)
                goto LABEL_408658;
LABEL_4083f7:
            v5 = i;
            if (i > v13)
            {
                v18 = 0;
                if (*((char *)i - 1) == g_42b684)
                {
                    v5 = i - 1;
                    if (v5 <= v13)
                        goto LABEL_408430;
                    v18 = 1;
                }
                if (*((char *)i + ~(v18)) == 13)
                    v5 = i - (v18 + 1);
            }
LABEL_408430:
            v19 = v5 - v13;
            if (v19 > NULL)
            {
                sub_41faa0(g_42b028, v2);
                sub_407e40(v13, v19);
                if (*(v2))
                    sub_41fac0(g_42b020);
                v7 = g_42b8fd;
                goto LABEL_4082fd;
            }
            else
            {
LABEL_408658:
                v5 = v13;
                v7 = g_42b8fd;
                goto LABEL_4082fd;
            }
        }
    }
}



// Function: prpending @ 0x8710
extern char g_42b684;
extern long long g_42b7a8;
extern void* g_42b7b8;
extern long long g_42b848;

long long prpending(void* i)
{
    long long v1;  // rax
    void* ptr;  // rax

    if (!g_42b7b8)
    {
        v1 = g_42b848;
        g_42b7b8 = g_42b848;
    }
    if (g_42b7a8 <= 0)
        return v1;
    while (g_42b7b8 < i)
    {
        ptr = rawmemchr(g_42b7b8, g_42b684);
        sub_4082a0(g_42b7b8, ptr + 1, 45);
        v1 = g_42b7a8 - 1;
        g_42b7a8 = v1;
        if (v1 <= 0)
            return v1;
    }
    return v1;
}



// Function: grepbuf @ 0x8790
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern long long g_42b020;
extern long long g_42b028;
extern struct_1 *g_42b040;
extern unsigned long long g_42b070;
extern int g_42b080;
extern FILE *stdout;
extern char g_42b684;
extern char g_42b688;
extern char g_42b7a1;
extern char g_42b7a2;
extern unsigned long long g_42b7a8;
extern long long g_42b7b0;
extern char g_42b7b8;
extern long long g_42b7f0;
extern char g_42b7f8;
extern char g_42b802;
extern char g_42b803;
extern unsigned long long g_42b828;
extern unsigned long long g_42b830;
extern unsigned long long g_42b840;
extern void* g_42b848;
extern unsigned long long g_42b870;
extern struct_0 *g_42b878;
extern char g_42b889;
extern unsigned int g_42b8c0;
extern unsigned int g_42b900;

unsigned long long grepbuf(void* a0, void* i)
{
    void* v6;  // r13
    long long v7;  // rax
    unsigned long long v16;  // rdi
    char v17;  // r12b
    long long v18;  // rdx
    unsigned long iter;  // r15
    unsigned long node;  // rbp
    void* v21;  // rax
    unsigned long long v22;  // rax
    int v23;  // edi
    void* v8;  // r15
    unsigned int v9;  // ebx
    long long v10;  // rax
    void* v11;  // rcx
    void* v12;  // r14
    void* v13;  // rcx
    long long v14;  // rax
    void* iter1;  // r14
    void* v0;  // [bp-0x70]
    void* v1;  // [bp-0x60]
    char *v2;  // [bp-0x58]
    long long v3;  // [bp-0x50]
    char v4;  // [bp-0x48], Other Possible Types: unsigned long long

    v3 = g_42b7b0;
    if (a0 >= i)
        return 0;
    v6 = a0;
    v2 = &v4;
    do
    {
        v7 = g_42b878(g_42b870, v6, i - v6, v2, 0);
        if (v7 >= 0)
        {
            v8 = v6 + v7;
            v0 = v4 + v8;
            if (!g_42b802)
            {
                if (i == v8)
                {
                    return v3 - iter;
                }
                v9 = g_42b684;
                if (!g_42b803)
                {
                    v6 = v0;
                    goto LABEL_408847;
                }
                else
                {
                    v10 = g_42b7b0;
                    v6 = v0;
LABEL_408a74:
                    iter = v10 - 1;
                    goto LABEL_408a78;
                }
            }
            else if (v8 <= v6)
            {
                v6 = v0;
                continue;
            }
            else
            {
                v11 = v0;
                v0 = v8;
            }
        }
        else
        {
            if (!g_42b802)
            {
                iter = g_42b7b0;
                return v3 - iter;
            }
            v4 = 0;
            if (i <= v6)
                break;
            v0 = i;
            v11 = v0;
        }
        v9 = g_42b684;
        v12 = v6;
        if (g_42b803)
            goto LABEL_4088f3;
        v8 = v6;
        v6 = v11;
LABEL_408847:
        if (g_42b7a8 > 0 && (sub_408710(v8), g_42b803))
        {
            v12 = v8;
            v11 = v6;
LABEL_4088f3:
            v10 = g_42b7b0;
            v17 = g_42b802;
            v6 = v11;
            v18 = g_42b7b0;
            if (!g_42b802)
                goto LABEL_408a74;
            goto LABEL_408911;
        }
        v13 = *((long long *)&g_42b7b8);
        if (!*((long long *)&g_42b7b8))
            v13 = g_42b848;
        v14 = 0;
        iter1 = v8;
        if (*((long long *)&g_42b7f8) > 0)
        {
            do
            {
                if (v13 < iter1)
                {
                    do
                    {
                        iter1 -= 1;
                    } while ((char)v9 != *((char *)iter1 - 1) && (iter1 -= 1, (char)v9 != *((char *)((char *)iter1 - 1))));
                }
            } while ((v14 = (long long)(v14 + 1), v14 != *((long long *)&g_42b7f8)));
            if (!g_42b688 || *((long long *)&g_42b7b8) == iter1 || (v16 = g_42b070, !g_42b070))
                goto LABEL_408a12;
            goto LABEL_408b46;
        }
        else if ((*((long long *)&g_42b7f8) & g_42b7f0) >= 0 && *((long long *)&g_42b7b8) != v8 && g_42b688 && !(v16 = g_42b070, !g_42b070))
        {
            iter1 = v8;
LABEL_408b46:
            if (g_42b900 && g_42b040->field_0)
            {
                sub_41faa0(g_42b028);
                v16 = g_42b070;
            }
            if (fputs_unlocked(v16, stdout) < 0)
                g_42b8c0 = *(__errno_location());
            if (!g_42b900 || !g_42b040->field_0)
            {
                sub_407e70(10);
            }
            else
            {
                sub_41fac0(g_42b020);
                sub_407e70(10);
            }
LABEL_408a12:
            v12 = iter1;
            if (v12 < v8)
            {
                do
                {
                    v12 = rawmemchr(iter1, (char)v9) + 1;
                    sub_4082a0(iter1, v12, 45);
                    iter1 = v12;
                } while (iter1 < v8);
            }
            goto LABEL_408a44;
        }
        else
        {
            v12 = v8;
LABEL_408a44:
            v17 = g_42b803;
            if (g_42b802)
            {
                v18 = g_42b7b0;
LABEL_408911:
                iter = v18;
                node = 0;
                if (v0 > v12)
                {
                    while (1)
                    {
                        if (node >= iter)
                        {
                            iter -= node;
                            break;
                        }
                        v21 = rawmemchr(v12, v9) + 1;
                        if (v17)
                        {
                            node += 1;
                            if (v0 <= v21)
                                goto LABEL_40897e;
                        }
                        else
                        {
                            v1 = v21;
                            sub_4082a0(v12, v21, 58);
                            node += 1;
                            v21 = v1;
                            v17 = g_42b803;
                            iter = g_42b7b0;
                            if (v0 <= v21)
                            {
LABEL_40897e:
                                iter -= node;
                                v12 = v21;
                                break;
                            }
                        }
                        v12 = v21;
                    }
                }
                goto LABEL_408984;
            }
            else if (!g_42b803)
            {
                v12 = v0;
                sub_4082a0(v8, v12, 58);
                v17 = g_42b803;
                iter = g_42b7b0 - 1;
LABEL_408984:
                v22 = 0;
                g_42b828 = v12 - g_42b840 + g_42b830;
                if (!v17)
                {
                    v22 = g_42b7f0;
                    if (g_42b7f0 < 0)
                    {
                        v22 = 0;
                        goto LABEL_4089b3;
                    }
                    else
                    {
                        goto LABEL_4089b3;
                    }
                }
            }
            else
            {
                iter = g_42b7b0 - 1;
LABEL_408a78:
                v22 = 0;
                g_42b828 = v0 - g_42b840 + g_42b830;
LABEL_4089b3:
                g_42b7a8 = v22;
                g_42b688 = 1;
                g_42b7b0 = iter;
                if (!iter || g_42b7a2)
                    goto LABEL_408be6;
            }
        }
    } while (i > v6);
    return v3 - g_42b7b0;
LABEL_408be6:
    if (!g_42b7a1)
        return v3 - iter;
    v23 = 0;
    if (g_42b889)
        v23 = g_42b080;
    exit(v23); /* do not return */
}



// Function: suppressible_error @ 0x8cb0
extern char g_42b889;
extern unsigned long long g_42b890;
extern char g_42b904;

void suppressible_error(int a0)
{
    if (g_42b904)
    {
        g_42b889 = 1;
        return;
    }
    if (!g_42b890)
        g_42b890 = dcgettext(NULL, "(standard input)", 5);
    error(0, a0, "%s");
    g_42b889 = 1;
    return;
}



// Function: fillbuf @ 0x8d20
typedef struct struct_1 {
    char padding_0[24];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_42b818;
extern char g_42b821;
extern unsigned long long g_42b830;
extern unsigned long long g_42b838;
extern struct_0 *g_42b840;
extern unsigned long long g_42b848;
extern unsigned int g_42b850;
extern unsigned long long g_42b858;
extern unsigned long long g_42b860;
extern char g_42b880;

unsigned long long fillbuf(unsigned long a0, struct_1 *a1)
{
    unsigned long ptr;  // rbp
    unsigned long v3;  // rax
    unsigned long v12;  // rax
    unsigned long long v13;  // rdx
    char *v14;  // rcx
    void* ptr1;  // rsi
    unsigned long long v16;  // rdx
    unsigned int v17;  // r15d
    unsigned long long v18;  // rcx
    char *v19;  // rsi
    long long v20;  // r14
    unsigned long v21;  // rax
    unsigned long v4;  // r14
    unsigned long long v22;  // rax
    char *v23;  // rdx
    unsigned long long v24;  // rbx
    unsigned long v25;  // 4114
    long long v26;  // rax
    unsigned long long v27;  // rdx
    char *v5;  // r13
    unsigned long v6;  // rcx
    unsigned long n;  // r9
    unsigned long v8;  // r15
    unsigned long long v10;  // rsi
    unsigned long long v11;  // rcx
    char *n1;  // [bp-0x40], Other Possible Types: unsigned long

    ptr = g_42b860;
    v3 = g_42b858;
    v4 = g_42b838;
    v5 = &g_42b840->field_0;
    v6 = g_42b838 + 8;
    if (ptr + g_42b858 - (char *)g_42b840 < v6)
    {
        n = a0 + 1;
        v8 = ~(a0);
        if (g_42b838 + a0 - g_42b858 + v6 > 0)
        {
            if (((unsigned short)a1->field_18 & 0xf000) == 0x8000)
            {
                v10 = *((long long *)&a1[1].padding_0[20]) - g_42b830;
                if (!(*((long long *)&a1[1].padding_0[20]) - g_42b830 >> 63 & 1))
                    v11 = v6 + a0;
            }
            n1 = n;
            v12 = sub_41f1f0(0, &g_42b858);
            v4 = g_42b838;
            ptr = v12;
            v5 = ptr + n1;
            v13 = v5 % g_42b838;
            if (!v13)
            {
                memmove(ptr, &g_42b840[v8], n1);
            }
            else
            {
                v5 = g_42b838 + a0 + 1 - v13 + ptr;
                memmove(&v5[v8], &g_42b840[v8], n1);
            }
            free(g_42b860);
            g_42b860 = ptr;
            v3 = g_42b858;
        }
        else
        {
            v14 = ptr + n;
            ptr1 = &g_42b840[v8];
            v16 = v14 % g_42b838;
            if (!v16)
            {
                n1 = v14;
                memmove(ptr, ptr1, n);
                v3 = g_42b858;
                v5 = n1;
            }
            else
            {
                v5 = g_42b838 + a0 + 1 - v16 + ptr;
                memmove(&v5[v8], ptr1, n);
                v3 = g_42b858;
            }
        }
    }
    v17 = g_42b850;
    g_42b848 = &v5[-1 * a0];
    while (1)
    {
        while (1)
        {
            do
            {
                v18 = sub_41da00(v17, v5, ptr + v3 - v5 - (ptr + v3 - v5 - 8) % v4 - 8);
                if (v18 == 0xffffffffffffffff)
                {
                    v19 = v5;
                    *((unsigned long *)&v19) = 0;
                    g_42b840 = v19;
                    return 0;
                }
                v19 = &v5[v18];
                v20 = g_42b830 + v18;
                g_42b830 = v20;
                v21 = g_42b821 ^ 1;
                v22 = _INSERT(v21, 0, (char)v21 | !v18);
                if ((char)v21 || !v18)
                {
                    *((unsigned long *)&v19) = 0;
                    g_42b840 = v19;
                    return v22;
                }
                if (v5 < v19)
                {
                    v23 = v5;
                    do
                    {
                        if (*(v23))
                        {
                            *((unsigned long *)&v19) = 0;
                            g_42b840 = v19;
                            return 1;
                        }
                    } while ((v23 += 1, v19 != v23));
                }
                v24 = v18;
                if (__OFADD__(v24, g_42b818) & 1)
                    error(2, 0, dcgettext(NULL, "input is too large to count", 5));
                v25 = g_42b880;
                g_42b818 = v24 + g_42b818;
                v17 = g_42b850;
            } while ((char)v25);
            v26 = lseek(g_42b850, v20, 3);
            if (v26 >= 0 || *(__errno_location()) == 6 && ((unsigned short)a1->field_18 & 0xf000) == 0x8000 && v20 < *((long long *)&a1[1].padding_0[20]) && !(v26 = (long long)lseek(g_42b850, 0, 2), lseek(g_42b850, 0, 2) < 0))
                break;
            g_42b880 = 1;
        }
        v27 = v26 - v20;
        if (__OFADD__(v27, g_42b818) & 1)
            break;
        g_42b818 = v27 + g_42b818;
        g_42b830 = v26;
    }
    error(2, 0, dcgettext(NULL, "input is too large to count", 5));
}



// Function: pattern_file_name @ 0x9080
typedef struct struct_0 {
    char field_0;
} struct_0;

extern long long g_42b8e0;
extern struct_0 *g_42b8f0;

long long pattern_file_name(unsigned long i, unsigned long long *a1)
{
    unsigned long long *v1;  // rcx
    unsigned long long v2;  // rax
    long long v3;  // rdx
    void* v4;  // rcx

    v1 = g_42b8f0;
    if (g_42b8e0 > 1)
    {
        v2 = 24;
        v3 = 1;
        while (*((long long *)&g_42b8f0[v2]) <= i)
        {
            v3 += 1;
            v2 += 24;
            if (v3 == g_42b8e0)
            {
                v4 = (char *)&g_42b8f0[24 * v3] - 24;
                *(a1) = i - *((long long *)v4) + (long long)v4[16];
                return (long long)v4[8];
            }
        }
        v1 = (char *)&g_42b8f0[v2] - 24;
    }
    *(a1) = i - *(v1) + v1[2];
    return v1[1];
}



// Function: grepdesc @ 0x9130
typedef struct struct_0 {
    unsigned long long field_0;
    long long field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[1];
    char field_1;
} struct_2;

typedef struct struct_1 {
    char padding_0[24];
    unsigned int field_18;
} struct_1;

typedef struct struct_4 {
    char padding_0[72];
    unsigned int field_48;
} struct_4;

typedef struct struct_3 {
    char padding_0[56];
    unsigned long long field_38;
    char padding_40[24];
    unsigned long long field_58;
    char padding_60[8];
    unsigned short field_68;
} struct_3;

typedef struct FILE {
} FILE;

extern unsigned int g_422520[4];
extern unsigned int g_42b014;
extern unsigned int g_42b018;
extern FILE *stdout;
extern char g_42b684;
extern char g_42b7a0;
extern char g_42b7a2;
extern unsigned long long g_42b7a8;
extern unsigned long long g_42b7b0;
extern unsigned long long g_42b7b8;
extern unsigned long long g_42b7c0;
extern unsigned long long g_42b7c8;
extern char g_42b7d8;
extern char g_42b7e0;
extern char g_42b7e8;
extern long long g_42b7f8;
extern char g_42b800;
extern char g_42b801;
extern char g_42b803;
extern unsigned int g_42b804;
extern int g_42b808;
extern char g_42b80c;
extern unsigned int g_42b810;
extern unsigned long long g_42b818;
extern char g_42b820;
extern char g_42b821;
extern unsigned long long g_42b828;
extern void g_42b830;
extern unsigned long long g_42b838;
extern void* g_42b840;
extern unsigned long long g_42b848;
extern unsigned int g_42b850;
extern long long g_42b858;
extern struct_1 *g_42b860;
extern char g_42b880;
extern char g_42b881;
extern unsigned int g_42b884;
extern char g_42b888;
extern char g_42b889;
extern char g_42b88a;
extern struct_2 *g_42b890;
extern struct_0 g_42b8a0;
extern struct_0 g_42b8b0;
extern unsigned int g_42b8c0;
extern unsigned int g_42b8f8;
extern char g_42b8fc;
extern char g_42b904;
extern char g_42b920;
extern char g_42b928;

unsigned long long grepdesc(unsigned int a0, unsigned int a1)
{
    unsigned long v16;  // rbx
    unsigned int v17;  // eax
    unsigned int v26;  // edx
    unsigned int v28;  // r14d
    unsigned long v29;  // rsi
    unsigned long long v30;  // rdx
    unsigned int v31;  // edi
    char v32;  // 4108
    unsigned long v33;  // 4110
    uint128_t v34;  // rcx
    long long v35;  // rax
    unsigned int v18;  // ebp
    long long i;  // rcx
    char v37;  // bpl
    unsigned long long v38;  // r12
    char v39;  // r15b
    void* v40;  // r13
    void* v41;  // r8
    unsigned long long v42;  // r14
    unsigned long v43;  // rsi
    unsigned long v44;  // rax
    int *err;  // rax
    unsigned int v19;  // eax
    void* v46;  // 4098
    void* v47;  // r14
    void* iter;  // r15
    unsigned long len;  // rax
    char v50;  // r12b
    void* v51;  // rax
    void* v52;  // rdi
    void* v53;  // r15
    void* v54;  // rax
    unsigned long v55;  // rax
    unsigned int v20;  // edx
    long long l;  // rax
    unsigned long long v57;  // rax
    int *err1;  // rax
    unsigned long long v59;  // 4099
    int *err2;  // rax
    long long v63;  // rax
    unsigned long long j;  // rax
    long long k;  // rax
    unsigned int v21;  // eax
    int *v66;  // rax
    struct_4 *v67;  // rbp
    struct_3 *v68;  // r12
    unsigned long v69;  // rax
    unsigned long long v70;  // rdi
    int *v71;  // rax
    unsigned long v72;  // 4115
    int *v73;  // rax
    int *v74;  // rax
    struct_0 *v23;  // rax
    unsigned int v24;  // eax
    unsigned int v25;  // eax
    char v0;  // [bp-0x1b8], Other Possible Types: unsigned long long
    char *v1;  // [bp-0x1b0], Other Possible Types: unsigned long
    unsigned long node;  // [bp-0x1a8]
    struct_1 *v3;  // [bp-0x1a0]
    void* v4;  // [bp-0x198], Other Possible Types: unsigned int, unsigned long long
    void* v5;  // [bp-0x190]
    unsigned long v6;  // [bp-0x188]
    char v7;  // [bp-0x17c]
    char v8;  // [bp-0x17b]
    char v9;  // [bp-0x17a]
    char v10;  // [bp-0x179]
    struct_2 *v11;  // [bp-0x178]
    unsigned long long v12;  // [bp-0x170]
    int v13;  // [bp-0x168]
    char v14;  // [bp-0xd8]

    v16 = a1;
    v0 = a1;
    v17 = fstat(a0, &v13);
    if (v17)
    {
        v73 = __errno_location();
        v28 = 1;
        sub_408cb0(*(v73));
LABEL_4098d2:
        if (!a0)
            return v28;
        goto LABEL_409454;
    }
    v18 = v17;
    if (!a0 && g_42b808 < 0)
    {
        g_42b808 = (*((unsigned int *)(&v13 + 24)) & 0xf000) == 0x4000;
        goto LABEL_409191;
    }
    if (g_42b884 != 2)
    {
        if (g_42b884 || (char)v16 == 1)
            goto LABEL_4094a4;
        v19 = *((unsigned int *)(&v13 + 24));
        v20 = v19 & 0xf000;
        if (((unsigned short)v19 & 0xb000) != 0x2000 && v20 != 0xc000 && v20 != 0x1000)
            goto LABEL_4094e0;
    }
    else
    {
        v21 = *((unsigned int *)(&v13 + 24));
        if (((unsigned short)v21 & 0xb000) != 0x2000 && (v21 & 0xf000) != 0xc000 && (v21 & 0xf000) != 0x1000)
        {
LABEL_4094a4:
            if ((char)v16)
            {
                if (((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) != 0x4000)
                    v23 = &g_42b8b0.field_0;
                if (g_42b018 != 4 && !(v23 = (struct_0 *)&g_42b8a0.field_0, !g_42b88a && v23->field_8 && (char)(unsigned long long)sub_416b90(v23->field_8)))
                    goto LABEL_4094e0;
            }
            else
            {
LABEL_4094e0:
                if (g_42b808 >= 0)
                {
                    v24 = g_42b018;
                    if (g_42b018 == 3)
                        goto LABEL_409a0e;
                    goto LABEL_4094fd;
                }
                g_42b808 = (*((unsigned int *)(&v13 + 24)) & 0xf000) == 0x4000;
                v24 = g_42b018;
                if (g_42b018 != 3)
                {
LABEL_4094fd:
                    if (v24 != 4 || ((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) != 0x4000)
                        goto LABEL_409506;
                }
                else
                {
LABEL_409a0e:
                    if (((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) == 0x4000)
                    {
                        if (close(a0))
                        {
                            v66 = __errno_location();
                            sub_408cb0(*(v66));
                        }
                        v12 = 0;
                        v11 = g_42b890;
                        v67 = sub_4189c0(&v11, ~((char)v16 ^ 1) & g_42b014, 0);
                        if (!v67)
                            sub_41f410(); /* do not return */
                        v1 = &v14;
                        while (1)
                        {
                            do
                            {
                                if (!sub_418fd0(v67))
                                {
                                    v71 = __errno_location();
                                    if (*(v71))
                                        sub_408cb0(*(v71));
                                    if (sub_418de0(v67))
                                    {
                                        sub_408cb0(*(v71));
                                        return 1;
                                    }
                                    return 1;
                                }
                            } while ((v69 = (unsigned long)(unsigned long long)v68->field_68, v16 = (unsigned long)_INSERT(v16, 0, (char)(char)(!v68->field_58) & v0), (unsigned short)v69 == 6));
                            if ((char)v16)
                                break;
                            if ((unsigned short)(v69 - 1 & 0xffffffff) > 1 && (v70 = (unsigned long long)*((long long *)&g_42b8b0), (unsigned short)v69 != 4))
                            {
LABEL_409f2d:
                                if (!v70)
                                    break;
                                if (!(char)sub_416b90())
                                {
                                    v69 = v68->field_68;
                                    break;
                                }
                            }
                            else if (g_42b018 != 4)
                            {
                                v70 = *((long long *)&g_42b8a0);
                                goto LABEL_409f2d;
                            }
                            else
                            {
                                sub_419720(v67, v68, 4);
                            }
                        }
                        v72 = g_42b88a;
                        g_42b890 = v68->field_38;
                        if ((char)v72 && g_42b890->field_1)
                            g_42b890 = g_42b890 + 1;
                        if ((unsigned short)v69 > 14)
                            abort(); /* do not return */
                        goto *((void *)((long long)(g_422520[v69] + (char *)&g_422520[0])));
                    }
LABEL_409506:
                    if (g_42b884 != 2 && (g_42b884 || (char)v16) || !(v25 = *((unsigned int *)((void*)&v13 + 24)), v26 = v25 & 0xf000, ((unsigned short)v25 & 0xb000) == 0x2000 || (v25 & 0xf000) == 0xc000 || (v25 & 0xf000) == 0x1000))
                    {
LABEL_409191:
                        v9 = g_42b803;
                        if (g_42b803 || *((int *)&g_42b80c) || *((long long *)&g_42b7e0) <= 1 || ((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) != 0x8000 || *((unsigned long long *)(&v13 + 8)) != *((long long *)&g_42b928) || *((unsigned long long *)&v13) != *((long long *)&g_42b920))
                        {
                            v29 = g_42b860 + 1;
                            v10 = g_42b7a2;
                            v30 = v29 % g_42b838;
                            if (v30)
                                v29 = &g_42b860[1 + g_42b838 + -1 * v30];
                            g_42b840 = v29;
                            g_42b848 = v29;
                            *((char *)(v29 - 1)) = g_42b684;
                            g_42b850 = a0;
                            if (g_42b850)
                            {
                                *((unsigned long *)&g_42b830) = 0;
                                g_42b881 = 0;
                                g_42b880 = 0;
                                goto LABEL_409228;
                            }
                            else
                            {
                                *((long *)&g_42b830) = lseek(0, 0, 1);
                                g_42b881 = *((long long *)&g_42b830) >> 63;
                                g_42b880 = g_42b881;
                                if (*((long long *)&g_42b830) >= 0)
                                    goto LABEL_409228;
                                v31 = *(__errno_location());
                                if (*(__errno_location()) == 29)
                                {
                                    *((unsigned long *)&g_42b830) = 0;
LABEL_409228:
                                    g_42b7c8 = 0;
                                    g_42b7b8 = 0;
                                    g_42b7b0 = *((long long *)&g_42b7e0);
                                    v32 = g_42b820;
                                    g_42b818 = 0;
                                    g_42b828 = 0;
                                    g_42b7a8 = 0;
                                    g_42b821 = !g_42b684 & v32;
                                    g_42b888 = 0;
                                    v8 = sub_408d20(0, &v13);
                                    if (v8)
                                    {
                                        v33 = g_42b8fc;
                                        g_42b8f8 = 0;
                                        if ((char)v33)
                                        {
                                            v34 = 0x7fffffffffffffff;
                                            if (((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) == 0x8000)
                                            {
                                                v34 = *((unsigned long long *)(&v13 + 48));
                                                if (g_42b801)
                                                    v34 += v34 != 0x7fffffffffffffff;
                                            }
                                            do
                                            {
                                                v18 += 1;
                                                v34 = (unsigned long long)((int128_t)(v34 * 7378697629483820647) >> 66) - ((int128_t)(v34) >> 63);
                                            } while (i != v35);
                                            g_42b8f8 = v18;
                                        }
                                        v1 = 0xffffffffffffffff;
                                        v0 = 0;
                                        node = 0;
                                        v3 = &v13;
                                        v7 = g_42b684;
                                        v37 = 0;
                                        v38 = 0;
                                        v39 = v8;
                                        do
                                        {
                                            v40 = g_42b840;
                                            v41 = g_42b848;
                                            v42 = v1 >> 63;
                                            v28 = _INSERT(v42, 0, (char)v42 & v7);
                                            if ((char)v42 & v7 && (v4 = g_42b810, g_42b810 != 1))
                                            {
                                                *((char *)g_42b840) = 0;
                                                v5 = g_42b848;
                                                v6 = g_42b840 - g_42b848;
                                                v43 = v6;
                                                v41 = v5;
                                                if (v43 == strlen(g_42b848))
                                                {
                                                    if (!v39 || g_42b881 || ((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) != 0x8000 || v43 >= *((unsigned long long *)(&v13 + 48)) || !a0 && !(v4 = v41, v41 = v4, v43 = (unsigned long)lseek(0, 0, 1), v43 >= 0))
                                                        goto LABEL_40939e;
                                                    v5 = v41;
                                                    v4 = v43;
                                                    v44 = lseek(a0, v43, 4);
                                                    v41 = v5;
                                                    if (v44 < 0)
                                                        goto LABEL_40939e;
                                                    if (lseek(a0, v4, 0) < 0)
                                                    {
                                                        err = __errno_location();
                                                        sub_408cb0(*(err));
                                                    }
                                                    if (v44 >= *((unsigned long long *)(&v13 + 48)))
                                                    {
                                                        v41 = g_42b848;
                                                        v40 = g_42b840;
                                                        goto LABEL_40939e;
                                                    }
                                                    else
                                                    {
                                                        v4 = g_42b810;
                                                    }
                                                }
                                                if (v4 != 2)
                                                {
                                                    if (!g_42b7e8)
                                                    {
                                                        g_42b803 = 1;
                                                        g_42b7a2 = 1;
                                                    }
                                                    v41 = g_42b848;
                                                    v37 = g_42b684;
                                                    v40 = g_42b840;
                                                    g_42b821 = g_42b820;
                                                    v1 = node;
                                                    goto LABEL_40939e;
                                                }
                                                else
                                                {
                                                    v8 = 0;
                                                    node = 0;
                                                    goto LABEL_4096ef;
                                                }
                                            }
                                            else
                                            {
LABEL_40939e:
                                                v46 = g_42b7b8;
                                                g_42b7c0 = v41;
                                                if (v46)
                                                    g_42b7b8 = v41;
                                                v47 = v41 + v38;
                                                if (v47 == v40)
                                                {
                                                    if (v0)
                                                    {
                                                        v59 = g_42b7b0;
                                                        *((char *)v40) = g_42b684;
                                                        g_42b840 = v40 + 1;
                                                        if (v59)
                                                            node += sub_408790(v41 + v38 - v0, g_42b840);
                                                        if (g_42b7a8)
                                                            sub_408710(g_42b840);
                                                    }
LABEL_4096b9:
                                                    g_42b7a2 = v10;
                                                    v28 = _INSERT(v47, 0, !node);
                                                    g_42b803 = v9;
                                                    if (!g_42b810 && !v9 && (g_42b888 || v1 >= 0 && v1 < node))
                                                    {
                                                        if (!g_42b890)
                                                            g_42b890 = dcgettext(NULL, "(standard input)", 5);
                                                        error(0, 0, dcgettext(NULL, "%s: binary file matches", 5));
                                                        goto LABEL_4096ef;
                                                    }
                                                }
                                                if (v37)
                                                {
                                                    iter = v47;
                                                    while (1)
                                                    {
                                                        *((char *)v40) = 0;
                                                        len = strlen(iter);
                                                        *((char *)v40) = v37;
                                                        iter += len;
                                                        if (iter == v40)
                                                            break;
                                                        do
                                                        {
                                                            iter += 1;
                                                            *((char *)iter - 1) = v37;
                                                        } while (!*((char *)iter));
                                                    }
                                                }
                                                v50 = *((char *)v47 - 1);
                                                *((char *)v47 - 1) = g_42b684;
                                                v51 = memrchr(v47 - 1, g_42b684, v40 - v47 + 1);
                                                *((char *)v47 - 1) = v50;
                                                v52 = v47 - v0;
                                                v53 = v51 + 1;
                                                v54 = v40;
                                                if (v47 != v53)
                                                {
                                                    v0 = v54 - v53;
                                                    if (v53 <= v52)
                                                        goto LABEL_409600;
                                                    if (g_42b7b0)
                                                    {
                                                        node += sub_408790(v52, v53);
                                                        if (!g_42b7a8)
                                                        {
                                                            if (!g_42b7b0)
                                                                goto LABEL_4097dd;
LABEL_4095de:
                                                            if (!g_42b7a2)
                                                                goto LABEL_409600;
                                                            v55 = 0;
                                                            if (v1 >= 0)
                                                                v55 = v1;
                                                            if (v55 >= node)
                                                                goto LABEL_409600;
LABEL_4097dd:
                                                            v8 = 0;
                                                            goto LABEL_4096b9;
                                                        }
                                                    }
                                                    else if (!g_42b7a8)
                                                    {
                                                        goto LABEL_4097dd;
                                                    }
                                                    sub_408710(v53);
                                                    if (!g_42b7b0 && !g_42b7a8)
                                                        goto LABEL_4097dd;
                                                    goto LABEL_4095de;
                                                }
                                                else
                                                {
                                                    v53 = v52;
                                                    v0 = v54 - v52;
LABEL_409600:
                                                    if (g_42b7f8 > 0)
                                                    {
                                                        l = 0;
                                                        do
                                                        {
                                                            if (v53 <= g_42b848)
                                                                break;
                                                            if (v53 == g_42b7b8)
                                                                goto LABEL_40964f;
                                                            l += 1;
                                                            do
                                                            {
                                                                v53 -= 1;
                                                            } while (g_42b684 != *((char *)v53 - 1));
                                                        } while (l != g_42b7f8);
                                                    }
                                                    if (v53 != g_42b7b8)
                                                        g_42b7b8 = 0;
                                                }
                                            }
LABEL_40964f:
                                            v38 = v40 - v53;
                                            if (g_42b800)
                                            {
                                                v57 = g_42b840 - g_42b848 - v38;
                                                if (__OFADD__(v57, g_42b7c8) & 1)
                                                    error(2, 0, dcgettext(NULL, "input is too large to count", 5));
                                                g_42b7c8 = v57 + g_42b7c8;
                                            }
                                            if (g_42b801)
                                                sub_407c90(v53);
                                            v39 = 0;
                                        } while ((char)sub_408d20(v38, v3));
                                        err1 = __errno_location();
                                        sub_408cb0(*(err1));
                                        v8 = 0;
                                        goto LABEL_4096b9;
                                    }
                                    else
                                    {
                                        err2 = __errno_location();
                                        v28 = 1;
                                        sub_408cb0(*(err2));
                                        node = 0;
                                    }
                                }
                                else
                                {
                                    sub_408cb0(*(__errno_location()));
                                    v28 = 1;
                                    v8 = 0;
                                    node = 0;
                                }
                            }
LABEL_4096ef:
                            if (g_42b7e8)
                            {
                                if (g_42b808)
                                {
                                    sub_407fa0();
                                    if (g_42b804)
                                        sub_407ec0(58);
                                    else
                                        sub_407e70(0);
                                }
                                sub_407b80("%ld\n");
                                if (g_42b7d8 && fflush_unlocked(stdout))
                                    g_42b8c0 = *(__errno_location());
                            }
                            if (!*((int *)&g_42b80c))
                            {
                                if (!a0)
                                {
                                    if (g_42b7b0)
                                    {
                                        if (v8)
                                            return v28;
                                        if (!g_42b881)
                                        {
                                            if (lseek(0, 0, 2) >= 0)
                                                return v28;
                                            if (*(__errno_location()) == 22)
                                                return v28;
                                        }
                                        if (((unsigned short)*((unsigned int *)(&v13 + 24)) & 0xf000) != 0x1000 || !g_42b7a0)
                                        {
                                            do
                                            {
LABEL_409b90:
                                                if (!sub_41da00(0, g_42b860, g_42b858))
                                                    return v28;
                                            } while (j != 0xffffffffffffffff);
                                        }
                                        else
                                        {
                                            v63 = splice(0, 0, 1, 0, 0x18000, 1);
                                            if (v63 < 0)
                                            {
                                                if (*(__errno_location()) == 22)
                                                    goto LABEL_409b90;
                                            }
                                            else
                                            {
                                                if (!v63)
                                                    return v28;
                                                do
                                                {
                                                    k = splice(0, 0, 1, 0, 0x18000, 1);
                                                } while (k > 0);
                                                if (!k)
                                                    return v28;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        if (*((long long *)&g_42b830) == g_42b828)
                                            return v28;
                                        if (g_42b881)
                                            return v28;
                                        if (lseek(0, g_42b828, 0) >= 0)
                                            return v28;
                                    }
                                    v74 = __errno_location();
                                    sub_408cb0(*(v74));
                                    return v28;
                                }
LABEL_409454:
                                if (!close(a0))
                                    return v28;
                                v74 = __errno_location();
                                sub_408cb0(*(v74));
                                return v28;
                            }
                            else if (*((int *)&g_42b80c) == 1 + (node < 1))
                            {
                                sub_407fa0();
                                sub_407e70(g_42b804 & 10);
                                if (g_42b7d8 && fflush_unlocked(stdout))
                                {
                                    g_42b8c0 = *(__errno_location());
                                    goto LABEL_4098d2;
                                }
                            }
                        }
                        else
                        {
                            if (!g_42b904)
                            {
                                sub_4090f0();
                                error(0, 0, dcgettext(NULL, "%s: input file is also the output", 5));
                            }
                            g_42b889 = 1;
                            v28 = 1;
                            goto LABEL_4098d2;
                        }
                    }
                }
            }
        }
    }
    v28 = 1;
    goto LABEL_409454;
}



// Function: grepfile @ 0xa0f0

unsigned long long grepfile(unsigned long a0, unsigned long a1, char a2, char a3)
{
    unsigned int v1;  // eax
    int *err;  // rax
    int v3;  // edi
    unsigned long long v4;  // rax

    v1 = sub_41b5d0();
    if (v1 >= 0)
        return sub_409130(v1, a3);
    err = __errno_location();
    v3 = *(err);
    if (!a2)
    {
        v4 = _INSERT(err, 0, v3 == 40 | v3 == 31);
        if (v3 == 40 || v3 == 31)
            return v4;
    }
    sub_408cb0(v3);
    return 1;
}



// Function: usage @ 0xa170
extern unsigned long long stderr;

void usage(int a0)
{
    unsigned long long v2;  // r12
    char *v3;  // rax
    long long v7;  // rdi
    long long v8;  // rsi
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    char *v4;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_419950();
    if (a0)
    {
        v3 = dcgettext(NULL, "Usage: %s [OPTION]... PATTERNS [FILE]...\n", 5);
        __fprintf_chk(stderr, 0x1, v3);
        sub_419950(v7, v8, v9, v10, v11, v12);
        v4 = dcgettext(NULL, "Try '%s --help' for more information.\n", 5);
        __fprintf_chk(stderr, 0x1, v4);
    }
    else
    {
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... PATTERNS [FILE]...\n", 5));
        __printf_chk(1, dcgettext(NULL, "Search for PATTERNS in each FILE.\n", 5));
        sub_419950(v7, v8, v9, v10, v11, v12);
        __printf_chk(1, dcgettext(NULL, "Example: %s -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n", 5));
        __printf_chk(1, dcgettext(NULL, "  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n", 5));
        __printf_chk(1, dcgettext(NULL, "  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n", 5));
        __printf_chk(1, dcgettext(NULL, "\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n", 5));
        __printf_chk(1, dcgettext(NULL, "\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n", 5));
        __printf_chk(1, dcgettext(NULL, "  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is 'binary', 'text', or 'without-match'\n  -a, --text                equivalent to --binary-files=text\n", 5));
        __printf_chk(1, dcgettext(NULL, "  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is 'read', 'recurse', or 'skip'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is 'read' or 'skip'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n", 5));
        __printf_chk(1, dcgettext(NULL, "      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n", 5));
        __printf_chk(1, dcgettext(NULL, "  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n", 5));
        __printf_chk(1, dcgettext(NULL, "\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n", 5));
        __printf_chk(1, dcgettext(NULL, "  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is 'always', 'never', or 'auto'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n", 5));
        __printf_chk(1, dcgettext(NULL, "When FILE is '-', read standard input.  With no FILE, read '.' if\nrecursive, '-' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n", 5));
        sub_41ee20(v7, v8, v9, v10, v11, v12);
    }
    exit(a0); /* do not return */
}



// Function: fgrep_to_grep_pattern @ 0xa3e0
typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_42b180;

void fgrep_to_grep_pattern(struct_0 **a0, struct_0 **a1)
{
    char *v4;  // r13
    char *v5;  // r12
    char *iter;  // r15
    unsigned long v7;  // rax
    char *v8;  // r14
    unsigned int v12;  // eax
    unsigned long v13;  // rdx
    unsigned long v0;  // [bp-0x188]
    char *v1;  // [bp-0x60]
    unsigned long v2;  // [bp-0x48]

    v4 = &*(a1)->field_0;
    v5 = &*(a0)->field_0;
    v2 = 0;
    v1 = sub_41f050(v4 + 1, 2);
    if (v4)
    {
        iter = v1;
        while (1)
        {
            v7 = (&g_42b180)[3 + *(v5)];
            if ((char)v7 == 254)
                v7 = sub_41ac90(v5, v4, &v2);
            if (v7 == 0xffffffffffffffff)
            {
                v2 = 0;
                goto LABEL_40a49c;
            }
            switch (v7)
            {
            case 1:
LABEL_40a49c:
                v12 = *(v5);
                v13 = v12 - 36;
                if ((char)v13 <= 58)
                {
                    v0 = 396316767208604737;
                    if (((char)v13 & 254 | *((char *)&v0 + ((long long)(v13 & 63) >> 3)) >> ((char)v13 & 63 & 7) & 1) & 1)
                    {
                        *(iter) = 92;
                        v12 = *(v5);
                        iter += 1;
                    }
                }
                v4 -= 1;
                *(iter) = v12;
                iter += 1;
                v5 += 1;
                if (!v4)
                    goto LABEL_40a4de;
                else
                    continue;
            case 18446744073709551614:
                v8 = NULL;
                break;
            default:
                v8 = NULL;
                break;
            }
        }
LABEL_40a4de:
        v4 = iter - v1;
    }
    else
    {
        iter = v1;
    }
    *(iter) = 10;
    free(*(a0));
    *(a0) = v1;
    *(a1) = v4;
    return;
}



// Function: Fcompile @ 0xa570
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_42b684;
extern char g_42b685;

void Fcompile(char *a0, unsigned long a1)
{
    char *v4;  // r15
    char *ptr;  // rbp
    struct_0 *idx;  // rax
    long long v6;  // r12
    char *iter;  // r15
    char *ptr1;  // rax
    unsigned long n;  // r13
    char v10;  // cl
    char *v11;  // rax
    char *v12;  // r15
    unsigned long long v13;  // rax
    char v0;  // [bp-0x61]
    char *v1;  // [bp-0x58]
    unsigned long v2;  // [bp-0x48]

    v4 = a0;
    ptr = NULL;
    v2 = 0;
    v6 = sub_40e5c0(1);
    v1 = &v2;
    iter = v4;
    do
    {
        ptr1 = rawmemchr(iter, 10);
        n = ptr1 - iter;
        if (g_42b685)
        {
            v10 = g_42b684;
            if (iter <= a0 || g_42b684 != 10)
            {
                if (n + 1 >= v2)
                {
                    free(ptr);
                    v2 = n;
                    v11 = sub_41f1f0(0, v1, 2, -0x1, 1);
                    v10 = g_42b684;
                    ptr = v11;
                    *(ptr) = g_42b684;
                }
                v0 = v10;
                memcpy(ptr + 1, iter, n);
                iter = ptr;
                ptr[1 + n] = v0;
            }
            else
            {
                iter -= 1;
            }
            n += 2;
        }
        v12 = ptr1 + 1;
        sub_40c9d0(v6, iter, n);
        iter = v12;
    } while (iter <= &v4[a1]);
    free(ptr);
    v13 = sub_40ceb0(v6);
    sub_40cec0(v6);
    idx = sub_41ef10(40);
    idx->field_0 = v6;
    idx->field_10 = a0;
    idx->field_8 = v13;
    idx->field_18 = a1;
    idx->field_20 = 0;
    return;
}



// Function: Fexecute @ 0xa710
typedef struct struct_0 {
    long long field_0;
    char padding_8[8];
    struct struct_0 *field_10;
    unsigned int field_18;
    char padding_1c[4];
    void* field_20;
} struct_0;

extern char g_42b180;
extern char g_42b182;
extern char g_42b684;
extern char g_42b685;
extern char g_42b686;

unsigned long long Fexecute(struct_0 *a0, unsigned long a1, unsigned long a2, long long *a3, unsigned long a4)
{
    unsigned long v14;  // rbp
    long long v15;  // r12
    unsigned long v24;  // r10
    unsigned long v26;  // r13
    long long v27;  // r15
    unsigned int v28;  // r12d
    unsigned long long v29;  // rax
    unsigned long v30;  // rax
    unsigned long v31;  // rax
    unsigned long v32;  // rdx
    unsigned long v16;  // rax
    unsigned long v17;  // rbx
    long long v18;  // rax
    unsigned long v19;  // rbx
    long long v20;  // r13
    unsigned long v21;  // r15
    unsigned long v22;  // rax
    unsigned long v23;  // r15
    unsigned int v0;  // [bp-0xbc]
    char *v1;  // [bp-0xb8]
    char v2;  // [bp-0x97]
    char v3;  // [bp-0x96]
    char v4;  // [bp-0x95]
    unsigned int v5;  // [bp-0x94]
    unsigned long v6;  // [bp-0x90]
    unsigned long v7;  // [bp-0x88]
    unsigned long v8;  // [bp-0x80]
    char v9;  // [bp-0x78]
    unsigned long v10;  // [bp-0x68]
    char v11;  // [bp-0x58]
    long long v12;  // [bp-0x48]

    v14 = a1 + a2;
    v15 = a0->field_0;
    v4 = g_42b684;
    v16 = g_42b685;
    v3 = a4;
    v2 = g_42b180 & ((g_42b182 | g_42b685) ^ 1);
    v5 = a4;
    if (a4)
        v17 = a4;
    else
        v17 = a1;
    v0 = (v2 | g_42b686 | v3) & (g_42b685 ^ 1);
    v7 = v17;
    if (v14 < v17)
        return 0xffffffffffffffff;
    v1 = &v9;
    while (1)
    {
        v18 = sub_40e3e0(v15, v17 - v16, v14 - v17 + v16, v1, v0);
        if (v18 < 0)
            break;
        v19 = v17 + v18;
        v8 = 0;
        v20 = v10 - g_42b685 * 2;
        if (!v2 || !sub_40e640(&v7, &v8, v19, v14))
        {
            if (g_42b686 != 1 && v3)
            {
                *(a3) = v20;
                return v19 - a1;
            }
            if (g_42b685)
            {
                *(a3) = v20 + (a4 < 1);
                return v19 - a1;
            }
            if (!g_42b686)
            {
                v26 = v19 + v20;
LABEL_40aa4d:
                v28 = v4;
                if (v14 > v26)
                    v14 = rawmemchr(v26, v28) + 1;
LABEL_40aa67:
                v30 = a1;
                v31 = memrchr(v30, v28, v19 - v30);
                if (v31)
                {
                    v32 = v31 + 1;
                    v29 = v32 - v30;
                }
                else
                {
                    v32 = a1;
                    v29 = 0;
                }
                v27 = v14 - v32;
                *(a3) = v27;
                return v29;
            }
            if (!v8)
            {
                v21 = v7;
                v22 = memrchr(v21, v4, v19 - v21);
                if (v22)
                {
                    v21 = v22 + 1;
                    v7 = v21;
                }
            }
            else if (v8 > 0)
            {
                if (sub_40e820(v19 - v8, v14))
                    goto LABEL_40a970;
                while (1)
                {
LABEL_40a8e2:
                    v23 = v19 + v20;
                    if (!sub_40e820(v23, v14))
                    {
                        v26 = v23;
                        if (a4)
                        {
                            *(a3) = v20;
                            return v19 - a1;
                        }
                        goto LABEL_40aa4d;
                    }
                    if (!a4 && !g_42b180)
                        break;
                    if (!v20 || sub_40e3e0(v15, v19, v20 - 1, &v11, 1))
                        goto LABEL_40a970;
                    v20 = v12;
                }
                if (!a0->field_20)
                {
                    sub_40a3e0(&a0->field_10, &a0->field_18);
                    a0->field_20 = sub_406a70(a0->field_10, *((long long *)&a0->field_18), 68166, v5);
                }
                v24 = v14;
                if (v24 > v23)
                    v24 = rawmemchr(v23, v4) + 1;
                v6 = v24;
                if (sub_407110(a0->field_20, v19, v24 - v19, a3, 0) < 0)
                {
                    v19 = v6 - 1;
                }
                else
                {
                    v28 = v4;
                    v14 = v6;
                    goto LABEL_40aa67;
                }
LABEL_40a970:
                v7 = v19 + sub_40e810(v19, v14);
                v17 = v7 + 1;
                if (v14 < v17)
                    break;
                else
                    goto LABEL_40a98c;
            }
            else
            {
                v21 = v7;
            }
            if (sub_40e830(v21, v19, v14))
                goto LABEL_40a970;
            goto LABEL_40a8e2;
        }
        else
        {
            v17 = v7;
            if (v14 < v17)
                break;
LABEL_40a98c:
            v16 = g_42b685;
        }
    }
    return 0xffffffffffffffff;
}



// Function: treefails @ 0xab10
typedef struct struct_3 {
    struct struct_3 *field_0;
    struct struct_3 *field_8;
    struct struct_0 *field_10;
} struct_3;

typedef struct struct_1 {
    void* field_0;
    void* field_8;
    struct struct_0 *field_10;
    char field_18;
} struct_1;

typedef struct struct_4 {
    char padding_0[8];
    struct struct_3 *field_8;
    char padding_10[16];
    struct struct_4 *field_20;
} struct_4;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[24];
    struct struct_0 *field_20;
} struct_0;

struct_0 * treefails(struct_1 *idx, struct_4 *a1, struct_0 *a2, char a3)
{
    unsigned long long v5;  // r14
    unsigned long long v6;  // r13
    struct_0 *v15;  // rax
    struct_0 *v16;  // rax
    unsigned long long v7;  // r12
    unsigned long long v8;  // rbx
    struct_4 *v9;  // rbx
    struct_4 *index;  // rbx
    struct_3 *idx1;  // rax
    struct_3 *v12;  // rcx
    struct_0 *v13;  // rdx
    struct_0 *v14;  // rax
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]
    unsigned long long v3;  // [bp-0x8]

    if (!idx)
        return v16;
    v3 = v5;
    v2 = v6;
    v1 = v7;
    v0 = v8;
    v9 = a1;
    treefails(idx->field_0, a1, a2, a3);
    treefails(idx->field_8, v9, a2, a3);
    if (v9)
    {
        do
        {
            index = v9;
            idx1 = index->field_8;
            if (index->field_8)
            {
                do
                {
                    if (idx->field_18 == (char)idx1[1].field_0)
                    {
                        v13 = idx->field_10;
                        v14 = idx1->field_10;
                        v13->field_20 = v14;
                        if (a3 || !v14->field_0 || v13->field_0)
                            return v14;
                        v13->field_0 = 0xffffffffffffffff;
                        return v14;
                    }
                    v12 = idx1->field_0;
                    if (idx->field_18 >= (char)idx1[1].field_0)
                        v12 = idx1->field_8;
                    idx1 = v12;
                } while (idx1);
            }
        } while ((v9 = (struct_4 *)index->field_20, index->field_20));
    }
    v15 = idx->field_10;
    v15->field_20 = a2;
    return v15;
}



// Function: hasevery @ 0xabd0
typedef struct struct_1 {
    struct struct_1 *field_0;
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_0 {
    void* field_0;
    void* field_8;
    char padding_10[8];
    char field_18;
} struct_0;

unsigned int hasevery(struct_1 *a0, struct_0 *a1)
{
    unsigned long long v2;  // rbx
    struct_1 *v3;  // rbx
    unsigned int v4;  // eax
    unsigned int v5;  // eax
    struct_1 *v6;  // rdx
    unsigned long long v0;  // [bp-0x10]

    if (!a1)
        return 1;
    v0 = v2;
    v3 = a0;
    v4 = hasevery(a0, a1->field_0);
    if (!(char)v4)
        return v4;
    v5 = hasevery(v3, a1->field_8);
    if (!(char)v5)
    {
        return v5;
    }
    else if (v3)
    {
        while (a1->field_18 != (char)v3[1].field_8)
        {
            v6 = v3->field_0;
            if (a1->field_18 >= (char)v3[1].field_8)
                v6 = v3->field_8;
            v3 = v6;
            if (!v3)
                return 0;
        }
        return v5;
    }
    else
    {
        return 0;
    }
}



// Function: memchr_kwset @ 0xac50
void* memchr_kwset(void* iter, unsigned long a1, void* a2)
{
    void* v3;  // rcx
    char v4;  // sil
    unsigned long v13;  // rcx
    unsigned long v14;  // rax
    unsigned long v15;  // rsi
    void* node;  // rax
    unsigned long long v5;  // rcx
    char v6;  // r10b
    unsigned long iter1;  // rdx
    char j;  // r9b
    char v9;  // r8b
    unsigned long i;  // rdx
    unsigned long long v11;  // rbp
    unsigned long long v12;  // rbx
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v3 = iter + a1;
    if ((int)a2[2444] >= 0)
    {
        do
        {
            if (iter >= v3)
                return NULL;
            if (*((long long *)((char *)a2 + 8 * *((char *)iter) + 368)))
                return iter;
            iter += 1;
        } while (*((char *)&iter) & 15);
        v4 = (int)a2[2440];
        v5 = v3 - iter;
        v6 = (int)a2[2444];
        iter1 = v3 - iter;
        if (v4 == v6)
            return memchr(iter, v4, iter1);
        j = v4;
        v9 = v6;
        if (iter1)
        {
            do
            {
                i = iter1;
                iter1 = i;
                if (!(*((char *)&iter) & 7))
                    break;
                if (*((char *)iter) == v9)
                    return iter;
                if (*((char *)iter) == j)
                    return iter;
                iter += 1;
                iter1 = i - 1;
            } while (i != 1);
        }
        v1 = v11;
        v0 = v12;
        v13 = v4 * 0x100 | v4;
        v14 = v9 * 0x100 | v9;
        if (iter1 > 7)
        {
            do
            {
                v15 = *((long long *)iter) ^ ((v13 * 0x10000 | v13) * 0x100000000 | v13 * 0x10000 | v13);
                if (0x8080808080808080 & (~(*((long long *)iter) ^ ((v14 | v14 * 0x10000) * 0x100000000 | v14 | v14 * 0x10000)) & (*((long long *)iter) ^ ((v14 | v14 * 0x10000) * 0x100000000 | v14 | v14 * 0x10000)) + 0xfefefefefefefeff | v15 + 0xfefefefefefefeff & ~(v15)))
                    goto LABEL_41b226;
            } while ((iter1 = (unsigned long)(iter1 - 8), iter += 8, iter1 > 7));
        }
        if (!iter1)
            return NULL;
LABEL_41b226:
        node = iter;
        while (*((char *)node) != j)
        {
            if (*((char *)node) == v9)
                return node;
            node += 1;
            if (node == iter1 + iter)
                return NULL;
        }
        return node;
    }
    else if (iter >= v3)
    {
        return NULL;
    }
    else
    {
        while (!*((long long *)((char *)a2 + 8 * *((char *)iter) + 368)))
        {
            iter += 1;
            if (v3 == iter)
                return NULL;
        }
        return iter;
    }
}



// Function: acexec @ 0xaf60
void acexec(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, char a4)
{
}



// Function: bmexec_trans @ 0xaf70
typedef struct struct_0 {
    char field_0;
    char padding_1[103];
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    unsigned long long field_978;
    struct struct_1 *field_980;
    char field_988;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

unsigned long long bmexec_trans(struct_0 *index, void* a1, unsigned long long a2)
{
    unsigned long long v5;  // r8
    char *v6;  // rbx
    void* v15;  // rax
    unsigned long v16;  // rdi
    void* v17;  // rax
    void* v18;  // rsi
    void* v19;  // rsi
    unsigned long v20;  // rax
    void* v21;  // rsi
    void* v22;  // rsi
    unsigned long v23;  // rax
    void* v24;  // rax
    struct_0 *v7;  // r12
    void* v25;  // rax
    void* v26;  // rax
    unsigned long long v27;  // r11
    unsigned long long v28;  // r10
    unsigned long v29;  // rdx
    unsigned long long v30;  // rdi
    unsigned long long node;  // rdx
    unsigned long long v32;  // r9
    unsigned long v33;  // rsi
    unsigned long v34;  // rdi
    unsigned long long v8;  // rbp
    unsigned long long v35;  // r10
    unsigned long long v36;  // r10
    unsigned long v37;  // rsi
    unsigned long v38;  // rdi
    unsigned long v39;  // rsi
    char v40;  // dil
    unsigned long i;  // r11
    unsigned long long v42;  // r10
    unsigned long long v43;  // r9
    unsigned long v44;  // rax
    void* iter;  // rdx
    unsigned long long v45;  // rsi
    unsigned long long iter1;  // rax
    unsigned long long v47;  // rdi
    unsigned long v48;  // rcx
    unsigned long v49;  // rsi
    unsigned long long v50;  // r9
    unsigned long long v51;  // r9
    unsigned long v52;  // rcx
    unsigned long v53;  // rsi
    unsigned long v54;  // rcx
    void* v10;  // rcx
    void* v55;  // rax
    void* v11;  // rdi
    struct_0 *idx;  // r12
    void* j;  // r15
    struct_0 *v14;  // r12
    struct_0 *v0;  // [bp-0x70]
    struct_0 *v1;  // [bp-0x58]
    unsigned long long v2;  // [bp-0x48]
    void* v3;  // [bp-0x40]

    v5 = index->field_68;
    if (!v5)
        return v5;
    v6 = &index->field_980->field_0;
    v7 = index;
    if (a2 < v5)
        return 0xffffffffffffffff;
    if (v5 != 1)
    {
        v0 = index->padding_70;
        v8 = index->field_970 + v5;
        iter = a1 + v5;
        if (__OFSMUL__(v5, 12) & 1 || v5 * 12 >= a2)
        {
            v11 = a2 + a1;
            idx = v7;
        }
        else
        {
            v10 = a2 - v5 * 11 + a1;
            v11 = a1 + a2;
            v3 = v11;
            idx = v7;
            if (iter <= v10)
            {
                j = v10;
                v14 = v0;
                idx = v7;
                do
                {
                    v15 = (&v14->field_0)[*((char *)iter - 1)] + iter;
                    v16 = (&v14->field_0)[*((char *)v15 - 1)];
                    v17 = v15 + v16;
                    if ((char)v16 && (v18 = (unsigned long long)(&v14->field_0)[(char)*((char *)((char *)v17 - 1))] + v17, v19 = v18 + (unsigned long long)(&v14->field_0)[(char)*((char *)((char *)v18 - 1))], v20 = (unsigned long)(unsigned long long)(&v14->field_0)[(char)*((char *)((char *)v19 - 1))], v17 = v20 + v19, (char)v20 && (v21 = (unsigned long long)(&v14->field_0)[(char)*((char *)((char *)v17 - 1))] + v17, v22 = v21 + (unsigned long long)(&v14->field_0)[(char)*((char *)((char *)v21 - 1))], v23 = (unsigned long)(unsigned long long)(&v14->field_0)[(char)*((char *)((char *)v22 - 1))], v17 = v23 + v22, (char)v23)))
                    {
                        v24 = v17 + (&v14->field_0)[*((char *)v17 - 1)];
                        v25 = v24 + (&v14->field_0)[*((char *)v24 - 1)];
                        if (v25 - iter <= 127)
                        {
                            v2 = v5;
                            v1 = idx;
                            v11 = v3;
                            v26 = sub_40ac50(v25 - 1, v11 - (v25 - 1), idx);
                            if (!v26)
                                return 0xffffffffffffffff;
                            idx = v1;
                            v5 = v2;
                            v17 = v26 + 1;
                            if (v17 >= j)
                                goto LABEL_40affe;
                        }
                        else
                        {
                            iter = v25;
                            continue;
                        }
                    }
                    v27 = 0;
                    v28 = v5;
                    while (1)
                    {
                        v29 = *((char *)v17 - 2);
                        if (v6)
                            v29 = v6[v29];
                        if (v7[1].padding_1[6] != (char)v29)
                        {
                            v30 = 0;
                            node = 2;
                        }
                        else
                        {
                            v32 = 0xfffffffffffffffd;
                            node = 2;
                            do
                            {
                                node += 1;
                                if (node > v28)
                                {
                                    v35 = v28 + v27;
                                    node = v35 + 1;
                                    if (v5 < node)
                                        return v17 - v5 - a1;
                                    v36 = ~(v35);
                                    while (1)
                                    {
                                        v37 = *((char *)v17 + v36);
                                        v38 = *((char *)(v8 + v36));
                                        if (v6)
                                        {
                                            v37 = v6[v37];
                                            v38 = v6[v38];
                                        }
                                        if ((char)v38 != (char)v37)
                                            break;
                                        node += 1;
                                        v36 -= 1;
                                        if (v5 < node)
                                            return v17 - v5 - a1;
                                    }
                                    if (v5 < node)
                                        return v17 - v5 - a1;
                                }
                                v33 = *((char *)v17 + v32);
                                v34 = *((char *)(v8 + v32));
                                if (v6)
                                {
                                    v33 = v6[v33];
                                    v34 = v6[v34];
                                }
                                v32 -= 1;
                            } while ((char)v34 == (char)v33);
                            v30 = node * 8 - 16;
                        }
                        v28 = *((long long *)(idx->field_978 + v30));
                        v17 += *((long long *)(idx->field_978 + v30));
                        v39 = *((char *)v17 - 1);
                        if (j < v17)
                        {
                            iter = v17;
                            goto LABEL_40b35b;
                        }
                        v40 = v39;
                        if (v6)
                            v40 = v6[(char)v39];
                        if (v7->field_988 != v40)
                            break;
                        v27 = node - 1;
                    }
                    iter = (&v14->field_0)[v39] + v17;
                } while (j >= iter);
LABEL_40b35b:
                v11 = a2 + a1;
            }
        }
LABEL_40affe:
        i = idx->padding_70[*((char *)iter - 1)];
        while (i <= v11 - iter)
        {
            iter += i;
            if ((&v0->field_0)[*((char *)iter - 1)])
                continue;
            v42 = 0;
            v43 = v5;
            while (1)
            {
                v44 = *((char *)iter - 2);
                if (v6)
                    v44 = v6[v44];
                if (v7[1].padding_1[6] != (char)v44)
                {
                    v45 = 0;
                    iter1 = 2;
                }
                else
                {
                    v47 = 0xfffffffffffffffd;
                    iter1 = 2;
                    do
                    {
                        iter1 += 1;
                        if (iter1 > v43)
                        {
                            v50 = v43 + v42;
                            iter1 = v50 + 1;
                            if (v5 < iter1)
                                return iter - v5 - a1;
                            v51 = ~(v50);
                            while (1)
                            {
                                v52 = *((char *)iter + v51);
                                v53 = *((char *)(v8 + v51));
                                if (v6)
                                {
                                    v52 = v6[v52];
                                    v53 = v6[v53];
                                }
                                if ((char)v53 != (char)v52)
                                    break;
                                iter1 += 1;
                                v51 -= 1;
                                if (v5 < iter1)
                                    return iter - v5 - a1;
                            }
                            if (iter1 > v5)
                                return iter - v5 - a1;
                        }
                        v48 = *((char *)iter + v47);
                        v49 = *((char *)(v8 + v47));
                        if (v6)
                        {
                            v48 = v6[v48];
                            v49 = v6[v49];
                        }
                        v47 -= 1;
                    } while ((char)v49 == (char)v48);
                    v45 = iter1 * 8 - 16;
                }
                v43 = *((long long *)(idx->field_978 + v45));
                iter += *((long long *)(idx->field_978 + v45));
                if (iter > v11)
                    break;
                v54 = *((char *)iter - 1);
                if (v6)
                    v54 = v6[v54];
                if (v7->field_988 != (char)v54)
                    break;
                v42 = iter1 - 1;
            }
        }
    }
    else
    {
        v55 = sub_40ac50(a1, a2, index);
        if (v55)
            return v55 - a1;
    }
    return 0xffffffffffffffff;
}



// Function: bmexec @ 0xb3f0
void bmexec(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40b3fc();
    return;
}



// Function: kwsalloc @ 0xc880
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
    char padding_51[7];
    unsigned long long field_58;
    unsigned long long field_60;
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
} struct_0;

struct_0 * kwsalloc(unsigned long a0)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    struct_0 *idx;  // r12
    unsigned long long v6;  // rcx
    unsigned long long v7;  // rax
    unsigned long v8;  // rdx
    unsigned long v9;  // rax
    unsigned long long v10;  // rsi
    unsigned long long v11;  // rax
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    idx = sub_41ef10(2464);
    _obstack_begin(idx, 0, 0, sub_41ef10, free);
    v6 = idx->field_20;
    v7 = idx->field_18;
    idx->field_58 = 0;
    if (v6 - v7 > 63)
    {
        v8 = idx->field_10;
        v9 = v7 + 64;
        if (v9 == v8)
            goto LABEL_40c9c1;
    }
    else
    {
        _obstack_newchunk(idx, 64);
        v8 = idx->field_10;
        v6 = idx->field_20;
        v9 = idx->field_18 + 64;
        if (v9 == v8)
        {
LABEL_40c9c1:
            idx->field_50 = idx->field_50 | 2;
        }
    }
    v10 = idx->field_30;
    idx->field_60 = v8;
    v11 = v9 + v10 & ~(v10);
    if (v11 - idx->field_8 <= v6 - idx->field_8)
        v6 = v11;
    idx->field_18 = v6;
    idx->field_10 = v6;
    memset(v8, 0, 56);
    idx->field_68 = 0x7fffffffffffffff;
    idx->field_980 = a0;
    idx[1].field_10 = sub_40af60;
    idx->field_970 = 0;
    return idx;
}



// Function: kwsincr @ 0xc9d0
typedef struct struct_1 {
} struct_1;

typedef struct struct_4 {
    char padding_0[8];
    unsigned long long field_8;
    void* field_10;
    void* field_18;
    void* field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
    char padding_51[7];
    unsigned long long field_58;
    void* field_60;
    unsigned long long field_68;
    char padding_70[2320];
    struct struct_5 *field_980;
    char padding_988[16];
    unsigned long long field_998;
} struct_4;

typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    struct struct_1 *field_10;
    char field_18;
} struct_0;

typedef struct struct_8 {
    char padding_0[25];
    char field_19;
} struct_8;

typedef struct struct_10 {
    char padding_0[8];
    struct struct_8 *field_8;
    char padding_10[9];
    char field_19;
} struct_10;

typedef struct struct_5 {
    char field_0;
} struct_5;

void kwsincr(struct_4 *idx, void* node, unsigned long long a2)
{
    struct_4 *idx1;  // r15
    unsigned long v10;  // r11
    unsigned long long idx2;  // rsi
    void* v20;  // rsi
    void* v21;  // r8
    void* v22;  // rax
    unsigned long v23;  // r9
    unsigned long v24;  // rdi
    void* v25;  // rax
    unsigned long v26;  // rdi
    void* v27;  // rax
    void* v28;  // rcx
    void* v11;  // r14
    void* v29;  // rax
    unsigned int v30;  // edi
    unsigned long v31;  // rax
    unsigned long v32;  // rax
    struct_10 *index;  // rsi
    char v34;  // al
    unsigned long long v35;  // rbx
    char v36;  // al
    struct_8 *v37;  // rax
    unsigned long v38;  // rdi
    unsigned long long v12;  // r13
    struct_8 *v39;  // rdi
    unsigned long v40;  // r8
    unsigned long v41;  // r9
    unsigned long v42;  // rdi
    unsigned long v43;  // rdi
    unsigned long v44;  // r8
    unsigned long v45;  // r9
    unsigned long long v46;  // rbx
    unsigned long long v47;  // rax
    char *v13;  // r15
    unsigned long long v14;  // r13
    unsigned long v15;  // rbp
    struct_0 *iter;  // rax
    unsigned long long v17;  // rbx
    char v18;  // cl
    struct_4 *v0;  // [bp-0xf8], Other Possible Types: unsigned long long
    unsigned long v1;  // [bp-0xf0]
    void* v2;  // [bp-0xe8]
    unsigned long v3;  // [bp-0xe0]
    char v4;  // [bp-0xdc]
    unsigned int v5;  // [bp-0xd8]
    char v6;  // [bp-0xd4]
    unsigned long v7;  // [bp-0xa8]

    idx1 = idx;
    v10 = idx->field_998;
    v11 = idx->field_60;
    if (v10 == sub_40b3f0)
        node += a2;
    if (a2)
    {
        v12 = a2 - 1;
        v13 = &idx->field_980->field_0;
        idx1 = idx;
        do
        {
            v14 = v12;
            if (v10 != sub_40b3f0)
            {
                v15 = *((char *)node);
                node += 1;
            }
            else
            {
                v15 = *((char *)node - 1);
                node -= 1;
            }
            if (v13)
                v15 = v13[v15];
            iter = (long long)v11[8];
            v17 = 1;
            v5 = 0;
            v7 = v11 + 8;
            if (!iter)
            {
                v17 = 0;
LABEL_40cabc:
                v20 = idx1->field_20;
                if (v20 - idx1->field_18 > 31)
                {
                    v21 = idx1->field_10;
                    v22 = idx1->field_18 + 32;
                    if (v22 != v21)
                        goto LABEL_40cae5;
                }
                else
                {
                    v1 = v10;
                    v0 = idx1;
                    _obstack_newchunk(idx1, 32);
                    idx1 = v0;
                    v10 = v1;
                    v21 = idx1->field_10;
                    v20 = idx1->field_20;
                    v22 = idx1->field_18 + 32;
                    if (v22 != v21)
                        goto LABEL_40cae5;
                }
                idx1->field_50 = idx1->field_50 | 2;
LABEL_40cae5:
                v23 = idx1->field_30;
                v24 = ~(v23);
                v25 = v22 + v23 & v24;
                v1 = v24;
                v26 = idx1->field_8;
                v0 = v20 - v26;
                if (v25 - v26 > v0)
                {
                    idx1->field_18 = v20;
                    idx1->field_10 = v20;
                    *((unsigned long long *)v21) = 0;
                    *((struct_0 **)&v21[8]) = NULL;
                }
                else
                {
                    idx1->field_18 = v25;
                    idx1->field_10 = v25;
                    *((unsigned long long *)v21) = 0;
                    *((struct_0 **)&v21[8]) = NULL;
                    if (v20 - v25 > 63)
                    {
                        v28 = v25;
                        v27 = v25 + 64;
                        if (v27 != v28)
                            goto LABEL_40cb95;
LABEL_40cd6f:
                        idx1->field_50 = idx1->field_50 | 2;
LABEL_40cb95:
                        v29 = v27 + v23 & v1;
                        v30 = (&v5)[v17];
                        if (v29 - v26 <= v0)
                            v20 = v29;
                        idx1->field_18 = v20;
                        v31 = (long long)v11[40];
                        idx1->field_10 = v20;
                        *((void* *)&v21[16]) = v28;
                        *((unsigned long long *)&v28[40]) = v31 + 1;
                        v32 = (&v7)[v17];
                        *((unsigned long long *)v28) = 0;
                        *((struct_0 **)&v28[8]) = NULL;
                        *((void* *)&v28[16]) = v11;
                        *((unsigned long *)&v28[24]) = 0;
                        *((unsigned long long *)&v28[32]) = 0;
                        *((unsigned long long *)&v28[48]) = 0;
                        *((char *)&v21[24]) = v15;
                        *((char *)&v21[25]) = 0;
                        if (!v30)
                            *((void* *)v32) = v21;
                        else
                            *((void* *)(v32 + 8)) = v21;
                        if (v17)
                        {
                            while (1)
                            {
                                index = (&v7)[v17];
                                v34 = index->field_19;
                                if (v34)
                                {
                                    if (!v30)
                                    {
                                        v36 = v34 - 1;
                                        index->field_19 = v36;
                                        if (!v36)
                                            break;
                                    }
                                    else if (!(v30 == 1 && (v36 = v34 + 1, index->field_19 = v36, v36)))
                                    {
                                        break;
                                    }
                                    switch (v36)
                                    {
                                    case 254:
                                        if (!*((int *)&(&v6)[4 * v17]))
                                        {
                                            v37 = (struct_8 *)index->padding_0;
                                            v42 = *((long long *)&v37->padding_0[8]);
                                            *((struct_10 **)&v37->padding_0[8]) = index;
                                            index->field_19 = 0;
                                            *((unsigned long *)&index->padding_0[0]) = v42;
                                            v37->field_19 = 0;
                                            break;
                                        }
                                        else if (*((int *)&(&v6)[4 * v17]) == 1)
                                        {
                                            v43 = (unsigned long)index->padding_0;
                                            v37 = *((long long *)(v43 + 8));
                                            v44 = *((long long *)&v37->padding_0[8]);
                                            v45 = *((long long *)&v37->padding_0[0]);
                                            *((unsigned long *)&v37->padding_0[0]) = v43;
                                            *((unsigned long *)(v43 + 8)) = v45;
                                            *((struct_10 **)&v37->padding_0[8]) = index;
                                            *((unsigned long *)&index->padding_0[0]) = v44;
                                            *((char *)(v43 + 25)) = -(_INSERT(v44, 0, v37->field_19 == 1));
                                            index->field_19 = v37->field_19 == 0xff;
                                            v37->field_19 = 0;
                                            break;
                                        }
                                        else
                                        {
                                            abort(); /* do not return */
                                        }
                                    case 2:
                                        if (!*((int *)&(&v6)[4 * v17]))
                                        {
                                            v39 = index->field_8;
                                            v37 = *((long long *)&v39->padding_0[0]);
                                            v40 = *((long long *)&v37->padding_0[8]);
                                            v41 = *((long long *)&v37->padding_0[0]);
                                            *((struct_10 **)&v37->padding_0[0]) = index;
                                            index->field_8 = v41;
                                            *((struct_8 **)&v37->padding_0[8]) = v39;
                                            *((unsigned long *)&v39->padding_0[0]) = v40;
                                            index->field_19 = -(_INSERT(v40, 0, v37->field_19 == 1));
                                            v39->field_19 = v37->field_19 == 0xff;
                                            v37->field_19 = 0;
                                            break;
                                        }
                                        else if (*((int *)&(&v6)[4 * v17]) == 1)
                                        {
                                            v37 = index->field_8;
                                            v38 = *((long long *)&v37->padding_0[0]);
                                            *((struct_10 **)&v37->padding_0[0]) = index;
                                            index->field_19 = 0;
                                            index->field_8 = v38;
                                            v37->field_19 = 0;
                                            break;
                                        }
                                        else
                                        {
                                            abort(); /* do not return */
                                        }
                                    default:
                                        abort(); /* do not return */
                                    }
                                    v46 = v17 - 1;
                                    if (!(&v5)[v46])
                                    {
                                        *((struct_8 **)(&v7)[v46]) = v37;
                                        v11 = v28;
                                    }
                                    else
                                    {
                                        *((struct_8 **)((&v7)[v46] + 8)) = v37;
                                        v11 = v28;
                                    }
                                }
                                index->field_19 = (char)-(v30 < 1) | 1;
                                v35 = v17 - 1;
                                if (v17 == 1)
                                    break;
                                v30 = (&v5)[v35];
                                v17 = v35;
                            }
                            continue;
                        }
                        v11 = v28;
                        continue;
                    }
                }
                v3 = v10;
                v2 = v21;
                v0 = idx1;
                _obstack_newchunk(idx1, 64);
                idx1 = v0;
                v21 = v2;
                v10 = v3;
                v23 = idx1->field_30;
                v20 = idx1->field_20;
                v27 = idx1->field_18 + 64;
                v1 = ~(v23);
                v26 = idx1->field_8;
                v0 = v20 - v26;
                v28 = idx1->field_10;
                if (v27 != v28)
                    goto LABEL_40cb95;
                goto LABEL_40cd6f;
            }
            while (1)
            {
                v18 = iter->field_18;
                if (v18 == (char)v15)
                    break;
                (&v7)[v17] = iter;
                idx2 = v17 + 1;
                if (v18 > (char)v15)
                {
                    iter = iter->field_0;
                    *((unsigned int *)&(&v4)[4 * idx2]) = 0;
                    if (!iter)
                        goto LABEL_40cabc;
                    goto LABEL_40ca90;
                }
                iter = iter->field_8;
                *((unsigned int *)&(&v4)[4 * idx2]) = 1;
                if (!iter)
                    goto LABEL_40cabc;
LABEL_40ca90:
                v17 = idx2;
            }
            v11 = iter->field_10;
        } while ((v12 = v14 - 1, v14 >= 1));
    }
    v47 = idx1->field_58;
    if (!*((long long *)v11))
        *((unsigned long long *)v11) = v47 * 2 + 1;
    idx1->field_58 = v47 + 1;
    if ((long long)v11[40] < idx1->field_68)
        idx1->field_68 = (long long)v11[40];
    return;
}



// Function: kwswords @ 0xceb0
typedef struct struct_0 {
    char padding_0[88];
    unsigned long long field_58;
} struct_0;

long long kwswords(struct_0 *a0)
{
    return a0->field_58;
}



// Function: kwsprep @ 0xcec0
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_288 {
    char *field_0;
    char *field_8;
    char *field_10;
    char *field_18;
    char *field_20;
    char *field_28;
    char *field_30;
    char *field_38;
    char *field_40;
    char *field_48;
    char *field_50;
    char *field_58;
    char *field_60;
    char *field_68;
    char *field_70;
    char *field_78;
    char *field_80;
    char *field_88;
    char *field_90;
    char *field_98;
    char *field_a0;
    char *field_a8;
    char *field_b0;
    char *field_b8;
    char *field_c0;
    char *field_c8;
    char *field_d0;
    char *field_d8;
    char *field_e0;
    char *field_e8;
    char *field_f0;
    char *field_f8;
    char padding_100[1784];
    unsigned long long field_7f8;
    char padding_800[368];
    char *field_970;
    char *field_978;
    struct struct_289 *field_980;
    unsigned int field_988;
    unsigned int field_98c;
    char field_990;
} struct_288;

typedef struct struct_4 {
    char padding_0[8];
    unsigned long long field_8;
    void* field_10;
    void* field_18;
    void* field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
    char padding_51[7];
    unsigned long long field_58;
    void* field_60;
    unsigned long long field_68;
    char padding_70[2320];
    struct struct_5 *field_980;
    char padding_988[16];
    unsigned long long field_998;
} struct_4;

typedef struct struct_13 {
    struct struct_7 *field_0;
    struct struct_7 *field_8;
    struct struct_1 *field_10;
    char field_18;
} struct_13;

typedef struct struct_2 {
    struct struct_5 *field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
    char field_18;
} struct_2;

typedef struct struct_14 {
    long long field_0;
    long long field_8;
    char padding_10[8];
    char field_18;
} struct_14;

typedef struct struct_7 {
    long long field_0;
    long long field_8;
    char *field_10;
    char field_18;
} struct_7;

typedef struct struct_11 {
    char padding_0[8];
    struct struct_13 *field_8;
    struct struct_1 *field_10;
    struct struct_10 *field_18;
} struct_11;

typedef struct struct_10 {
    char padding_0[16];
    struct struct_9 *field_10;
    struct struct_10 *field_18;
    char padding_20[16];
    long long field_30;
    long long field_38;
} struct_10;

typedef struct struct_9 {
    char padding_0[56];
    long long field_38;
} struct_9;

typedef struct struct_6 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[16];
    struct struct_6 *field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
} struct_6;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_2 *field_8;
    struct struct_1 *field_10;
    struct struct_1 *field_18;
    struct struct_6 *field_20;
    unsigned long long field_28;
    struct struct_1 *field_30;
    struct struct_1 *field_38;
} struct_1;

typedef struct struct_5 {
    struct struct_0 *field_0;
} struct_5;

typedef struct struct_289 {
    char field_0;
} struct_289;

void kwsprep(struct_288 *idx)
{
    struct_288 *index;  // rbp
    char *v8;  // rbx
    char *v17;  // rdx
    struct_288 *v107;  // rax
    struct_288 *v108;  // rax
    struct_288 *v109;  // rax
    struct_288 *v110;  // rax
    struct_288 *v111;  // rax
    struct_288 *v112;  // rax
    struct_288 *v113;  // rax
    unsigned long long v114;  // rbx
    struct_288 *v115;  // rax
    struct_288 *v116;  // rax
    char *idx1;  // r8
    struct_288 *v117;  // rax
    unsigned long long v118;  // rbx
    struct_288 *v119;  // rax
    struct_288 *v120;  // rax
    struct_288 *v121;  // rax
    struct_288 *v122;  // rax
    struct_288 *v123;  // rax
    char *v124;  // rax
    struct_288 *v125;  // rax
    char *v126;  // rax
    char *v19;  // rax
    struct_6 *iter;  // rbx
    unsigned long v128;  // rax
    unsigned long v129;  // rax
    struct_10 *node;  // rax
    long long v131;  // rdx
    long long v132;  // rcx
    struct_288 *v133;  // r8
    unsigned long long *iter1;  // rdi
    unsigned long long v135;  // rcx
    unsigned long long iter2;  // rax
    struct_11 *idx2;  // rcx
    unsigned int v137;  // esi
    unsigned long v138;  // rcx
    unsigned long v139;  // rdx
    char *v140;  // rdx
    char *v141;  // rsi
    char *v142;  // r13
    char *v143;  // rcx
    char *v144;  // rax
    char *v145;  // rdi
    char *v146;  // rax
    char *v21;  // rcx
    char *v147;  // rdx
    unsigned long v148;  // rax
    unsigned long v149;  // rax
    char *v150;  // rcx
    char *v151;  // rdx
    char *v152;  // rax
    long long v153;  // r13
    char *v154;  // rdi
    char *v155;  // rax
    char *v156;  // rsi
    char *v22;  // r8
    char *v157;  // rax
    char *v158;  // rax
    char *v159;  // rdx
    unsigned long m;  // rax
    char *v161;  // rdx
    unsigned long v162;  // rax
    unsigned long long j;  // rax
    char *v23;  // r14
    char *v24;  // rdx
    char *v25;  // rax
    char *v26;  // rsi
    struct_288 *v9;  // r12
    char *v27;  // rax
    char *v28;  // rax
    struct_0 *v29;  // r9
    struct_4 *v30;  // rax
    char *v31;  // rdx
    char *v32;  // rsi
    struct_4 *ptr;  // r14
    unsigned long long v34;  // rcx
    struct_288 *v35;  // rdi
    struct_4 *k;  // rsi
    char v10;  // al
    unsigned long v37;  // d
    char v38;  // al
    unsigned long v39;  // rax
    char *v40;  // rsi
    struct_2 *v41;  // r9
    struct_0 *v42;  // rbx
    unsigned long long v43;  // rbx
    char *v44;  // rdx
    char *v45;  // rax
    char *v46;  // rdx
    unsigned long v11;  // rax
    unsigned long long v47;  // rbx
    unsigned long long v48;  // rbx
    char *v49;  // rax
    char *v50;  // rax
    char *v51;  // rdx
    unsigned long long v52;  // rbx
    unsigned long long v53;  // rbx
    char *v54;  // rdx
    char *v55;  // rax
    char *v56;  // rax
    char *v12;  // rsi
    unsigned long long v57;  // rbx
    unsigned long long v58;  // rbx
    char *v59;  // rdx
    char *v60;  // rax
    unsigned long long v61;  // r10
    char *v62;  // rdx
    unsigned long v63;  // r13
    struct_13 *v64;  // r11
    unsigned long long v65;  // rbx
    unsigned long long v66;  // r13
    char *v13;  // r8
    unsigned long long v67;  // r13
    char *v68;  // rdx
    char *v69;  // rax
    char *v70;  // rdx
    unsigned long long v71;  // r13
    unsigned long long v72;  // r13
    char *v73;  // rax
    char *v74;  // rax
    char *v75;  // rax
    unsigned long long v76;  // r13
    char *v14;  // rcx
    struct_7 *v77;  // rdx
    struct_7 *v78;  // rdx
    char *v79;  // rdx
    char *v80;  // rax
    unsigned long long v81;  // r13
    char *v82;  // rdx
    unsigned long long v83;  // rdx
    unsigned long long v84;  // rdx
    char *v85;  // rsi
    char *v86;  // rdx
    struct_13 *v15;  // r11
    char *v87;  // rax
    unsigned long long v88;  // rbx
    struct_288 *v89;  // rax
    struct_288 *v90;  // rax
    struct_288 *v91;  // rax
    unsigned long long v92;  // rbx
    struct_288 *v93;  // rax
    struct_288 *v94;  // rax
    struct_288 *v95;  // rax
    struct_288 *v96;  // rax
    struct_11 *v16;  // r10
    unsigned long long v97;  // rbx
    struct_288 *v98;  // rax
    struct_288 *v99;  // rax
    struct_288 *v100;  // rax
    unsigned long long v101;  // rbx
    struct_288 *v102;  // rax
    struct_288 *v103;  // rax
    struct_288 *v104;  // rax
    struct_288 *v105;  // rax
    struct_288 *v106;  // rax
    char *v0;  // [bp-0x970]
    struct_14 *v1;  // [bp-0x968], Other Possible Types: struct_7 *, unsigned long long
    char *v2;  // [bp-0x960]
    char *v3;  // [bp-0x950]
    char *v4;  // [bp-0x948]
    char *v5;  // [bp-0x148]

    index = idx;
    v8 = &idx->field_980->field_0;
    v9 = &v5;
    if (!v8)
        v9 = &idx->field_70;
    v0 = idx->field_58;
    if (v0 != 0x1)
    {
        v10 = (char)idx->field_68;
        if (0xff < idx->field_68)
            v10 = 0xff;
        v11 = v10;
        v9->field_0 = v11 * 0x101010101010101;
        v12 = 0x101010101010101 * v11 + (v11 * 0x101010101010101 >> 64);
        v9->field_8 = v12;
        v9->field_18 = v12;
        v9->field_10 = v11 * 0x101010101010101;
        v9->field_28 = v12;
        v9->field_38 = v12;
        v9->field_48 = v12;
        v9->field_20 = v11 * 0x101010101010101;
        v9->field_58 = v12;
        v9->field_68 = v12;
        v9->field_78 = v12;
        v9->field_30 = v11 * 0x101010101010101;
        v9->field_88 = v12;
        v9->field_98 = v12;
        v9->field_a8 = v12;
        v9->field_40 = v11 * 0x101010101010101;
        v9->field_b8 = v12;
        v9->field_c8 = v12;
        v9->field_d8 = v12;
        v9->field_50 = v11 * 0x101010101010101;
        v9->field_e8 = v12;
        v9->field_60 = v11 * 0x101010101010101;
        v9->field_70 = v11 * 0x101010101010101;
        v9->field_80 = v11 * 0x101010101010101;
        v9->field_90 = v11 * 0x101010101010101;
        v9->field_a0 = v11 * 0x101010101010101;
        v9->field_b0 = v11 * 0x101010101010101;
        v9->field_c0 = v11 * 0x101010101010101;
        v9->field_d0 = v11 * 0x101010101010101;
        v9->field_e0 = v11 * 0x101010101010101;
        v9->field_f0 = v11 * 0x101010101010101;
        v9->field_f8 = v12;
        v13 = index->field_60;
        v3 = v13;
        if (v13)
            goto LABEL_40d084;
    }
    else
    {
        v3 = idx->field_60;
        v14 = v3;
        if (v3)
        {
            do
            {
                if (*((long long *)&v14[8]))
                {
                    if (*((long long *)*((long long *)&v14[8])))
                    {
                        if (*((long long *)*((long long *)*((long long *)&v14[8]))))
                            sub_40b420(*((long long *)*((long long *)*((long long *)&v14[8]))), v15);
                        if (v16->field_8)
                            sub_40b420(v16->field_8, v15);
                        v3 = v16->field_10;
                        *((struct struct_1 **)&v3[24]) = v16->field_10;
                    }
                    v17 = v3;
                    if (*((long long *)&idx1[8]))
                    {
                        if (*((long long *)*((long long *)&idx1[8])))
                            sub_40b420(*((long long *)*((long long *)&idx1[8])), v15);
                        if (v16->field_8)
                            sub_40b420(v16->field_8, v15);
                        v17 = v16->field_10;
                        *((char **)&v3[24]) = v17;
                    }
                    v19 = *((long long *)&idx1[16]);
                    *((char **)&v17[24]) = v19;
                    v3 = v19;
                }
            } while ((v14 = (char *)idx2->field_18, idx2->field_18));
        }
        v21 = index->field_20;
        v22 = index->field_68;
        v23 = v22;
        if (v22 <= v21 - index->field_18)
        {
            v24 = index->field_10;
            v25 = index->field_18 + v23;
            if (v25 == v24)
                goto LABEL_40e3d2;
        }
        else
        {
            _obstack_newchunk(index, v22);
            v24 = index->field_10;
            v21 = index->field_20;
            v25 = index->field_18 + v23;
            v22 = index->field_68;
            if (v25 == v24)
            {
LABEL_40e3d2:
                *((char *)&index->field_50) = (char)index->field_50 | 2;
            }
        }
        v26 = index->field_30;
        index->field_970 = v24;
        v27 = v25 + v26 & ~(v26);
        if (v27 - index->field_8 <= v21 - index->field_8)
            v21 = v27;
        index->field_18 = v21;
        index->field_10 = v21;
        if (v22 > NULL)
        {
            v28 = NULL;
            while (1)
            {
                *((char *)(v24 + v28)) = *((char *)(v29->field_8 + 24));
                v28 += 1;
                if (index->field_68 <= v28)
                    break;
                v24 = index->field_970;
            }
        }
        v30 = sub_40c880(index->field_980);
        v31 = index->field_68;
        v32 = index->field_970;
        ptr = v30;
        ptr->field_998 = sub_40b3f0;
        sub_40c9d0(ptr, v32, v31);
        _obstack_free(index, 0);
        v34 = 308;
        v35 = index;
        for (k = ptr; v34; k = &(k->padding_0)[v37])
        {
            v34 -= 1;
            *((char [8])&v35->field_0) = k->padding_0;
            v35 = &(&v35->field_0)[v37];
        }
        free(ptr);
        v38 = (char)index->field_68;
        if (0xff < index->field_68)
            v38 = 0xff;
        v39 = v38;
        v9->field_0 = v39 * 0x101010101010101;
        v40 = 0x101010101010101 * v39 + (v39 * 0x101010101010101 >> 64);
        v9->field_8 = v40;
        v9->field_18 = v40;
        v9->field_10 = v39 * 0x101010101010101;
        v9->field_28 = v40;
        v9->field_38 = v40;
        v9->field_48 = v40;
        v9->field_20 = v39 * 0x101010101010101;
        v9->field_58 = v40;
        v9->field_68 = v40;
        v9->field_78 = v40;
        v9->field_30 = v39 * 0x101010101010101;
        v9->field_88 = v40;
        v9->field_98 = v40;
        v9->field_a8 = v40;
        v9->field_40 = v39 * 0x101010101010101;
        v9->field_b8 = v40;
        v9->field_c8 = v40;
        v9->field_d8 = v40;
        v9->field_50 = v39 * 0x101010101010101;
        v9->field_e8 = v40;
        v9->field_60 = v39 * 0x101010101010101;
        v9->field_70 = v39 * 0x101010101010101;
        v9->field_80 = v39 * 0x101010101010101;
        v9->field_90 = v39 * 0x101010101010101;
        v9->field_a0 = v39 * 0x101010101010101;
        v9->field_b0 = v39 * 0x101010101010101;
        v9->field_c0 = v39 * 0x101010101010101;
        v9->field_d0 = v39 * 0x101010101010101;
        v9->field_e0 = v39 * 0x101010101010101;
        v9->field_f0 = v39 * 0x101010101010101;
        v9->field_f8 = v40;
        v13 = index->field_60;
        v3 = v13;
        if (v13)
        {
LABEL_40d084:
            v2 = v8;
            while (1)
            {
                v41 = *((long long *)&v13[8]);
                if (v41)
                    break;
LABEL_40dc38:
                sub_40ab10(v41, *((long long *)&v13[32]), index->field_60, v0 == 0x1);
                if (v0 != 0x1 || !(v126 = index->field_68, iter = (struct_6 *)*((long long *)&idx1[32]), *((char **)&idx1[48]) = v126, *((char **)&idx1[56]) = v126, iter))
                {
                    v13 = *((long long *)&idx1[24]);
                    if (!v13)
                        goto LABEL_40dd0f;
                }
                else
                {
                    do
                    {
                        if (!(char)sub_40abd0(iter->field_8, v29))
                        {
                            v128 = *((long long *)&idx1[40]) - iter->field_28;
                            if (v128 < iter->field_30)
                                iter->field_30 = v128;
                        }
                        if (v15)
                        {
                            v129 = *((long long *)&idx1[40]) - iter->field_28;
                            if (iter->field_38 > v129)
                                iter->field_38 = v129;
                        }
                        iter = iter->field_20;
                    } while (iter);
                    v13 = *((long long *)&idx1[24]);
                    if (!v13)
                        goto LABEL_40dd0f;
                }
            }
            if (v41->field_0)
            {
                if (v41->field_0->field_0)
                {
                    if (v41->field_0->field_0->field_0)
                    {
                        v42 = *((long long *)v41->field_0->field_0->field_0);
                        if (v42)
                        {
                            if (v42->padding_0)
                                sub_40b420(v42->padding_0, &v3);
                            if (v42->field_8)
                                sub_40b420(v42->field_8, &v3);
                            v3 = v42->field_10;
                            *((unsigned long long *)&v3[24]) = v42->field_10;
                        }
                        v43 = v16->field_8;
                        v44 = v3;
                        if (v43)
                        {
                            if (*((long long *)v43))
                                sub_40b420(*((long long *)v43), &v3);
                            if (*((long long *)(v43 + 8)))
                                sub_40b420(*((long long *)(v43 + 8)), &v3);
                            v44 = *((long long *)(v43 + 16));
                            *((char **)&v3[24]) = v44;
                        }
                        v45 = v16->field_10;
                        *((char **)&v44[24]) = v45;
                        v3 = v45;
                    }
                    v46 = v3;
                    if (idx2->field_8)
                    {
                        v47 = idx2->field_8->field_0;
                        if (v47)
                        {
                            if (*((long long *)v47))
                                sub_40b420(*((long long *)v47), &v3);
                            if (*((long long *)(v47 + 8)))
                                sub_40b420(*((long long *)(v47 + 8)), &v3);
                            v3 = *((long long *)(v47 + 16));
                            *((long long *)&v3[24]) = *((long long *)(v47 + 16));
                        }
                        v48 = v16->field_8;
                        v49 = v3;
                        if (v48)
                        {
                            if (*((long long *)v48))
                                sub_40b420(*((long long *)v48), &v3);
                            if (*((long long *)(v48 + 8)))
                                sub_40b420(*((long long *)(v48 + 8)), &v3);
                            v49 = *((long long *)(v48 + 16));
                            *((char **)&v3[24]) = v49;
                        }
                        v46 = v16->field_10;
                        *((char **)&v49[24]) = v46;
                    }
                    v50 = idx2->field_10;
                    *((char **)&v46[24]) = v50;
                    v3 = v50;
                }
                v51 = v3;
                if (v15->field_8)
                {
                    if (v15->field_8->field_0)
                    {
                        v52 = *((long long *)v15->field_8->field_0);
                        if (v52)
                        {
                            if (*((long long *)v52))
                                sub_40b420(*((long long *)v52), &v3);
                            if (*((long long *)(v52 + 8)))
                                sub_40b420(*((long long *)(v52 + 8)), &v3);
                            v3 = *((long long *)(v52 + 16));
                            *((long long *)&v3[24]) = *((long long *)(v52 + 16));
                        }
                        v53 = v16->field_8;
                        v54 = v3;
                        if (v53)
                        {
                            if (*((long long *)v53))
                                sub_40b420(*((long long *)v53), &v3);
                            if (*((long long *)(v53 + 8)))
                                sub_40b420(*((long long *)(v53 + 8)), &v3);
                            v54 = *((long long *)(v53 + 16));
                            *((char **)&v3[24]) = v54;
                        }
                        v55 = v16->field_10;
                        *((char **)&v54[24]) = v55;
                        v3 = v55;
                    }
                    v56 = v3;
                    if (idx2->field_8)
                    {
                        v57 = idx2->field_8->field_0;
                        if (v57)
                        {
                            if (*((long long *)v57))
                                sub_40b420(*((long long *)v57), &v3);
                            if (*((long long *)(v57 + 8)))
                                sub_40b420(*((long long *)(v57 + 8)), &v3);
                            v3 = *((long long *)(v57 + 16));
                            *((long long *)&v3[24]) = *((long long *)(v57 + 16));
                        }
                        v58 = v16->field_8;
                        v59 = v3;
                        if (v58)
                        {
                            if (*((long long *)v58))
                                sub_40b420(*((long long *)v58), &v3);
                            if (*((long long *)(v58 + 8)))
                                sub_40b420(*((long long *)(v58 + 8)), &v3);
                            v59 = *((long long *)(v58 + 16));
                            *((char **)&v3[24]) = v59;
                        }
                        v56 = v16->field_10;
                        *((char **)&v59[24]) = v56;
                    }
                    v51 = idx2->field_10;
                    *((char **)&v56[24]) = v51;
                }
                v60 = v15->field_10;
                v61 = v29->field_8;
                *((char **)&v51[24]) = v60;
                v3 = v60;
                if (v61)
                    goto LABEL_40d327;
                v62 = v29->field_10;
                v63 = *((long long *)&idx1[40]);
                *((char **)&v3[24]) = v62;
                v3 = v62;
                v64 = v15;
                goto LABEL_40d613;
            }
            else
            {
                v61 = v41->field_8;
                if (v61)
                {
LABEL_40d327:
                    v65 = *((long long *)v61);
                    if (v65)
                    {
                        if (*((long long *)v65))
                        {
                            v66 = *((long long *)*((long long *)v65));
                            if (v66)
                            {
                                if (*((long long *)v66))
                                    sub_40b420(*((long long *)v66), &v3);
                                if (*((long long *)(v66 + 8)))
                                    sub_40b420(*((long long *)(v66 + 8)), &v3);
                                v3 = *((long long *)(v66 + 16));
                                *((long long *)&v3[24]) = *((long long *)(v66 + 16));
                            }
                            v67 = idx2->field_8;
                            v68 = v3;
                            if (v67)
                            {
                                if (*((long long *)v67))
                                    sub_40b420(*((long long *)v67), &v3);
                                if (*((long long *)(v67 + 8)))
                                    sub_40b420(*((long long *)(v67 + 8)), &v3);
                                v68 = *((long long *)(v67 + 16));
                                *((char **)&v3[24]) = v68;
                            }
                            v69 = idx2->field_10;
                            *((char **)&v68[24]) = v69;
                            v3 = v69;
                        }
                        v70 = v3;
                        if (*((long long *)(v65 + 8)))
                        {
                            v71 = *((long long *)*((long long *)(v65 + 8)));
                            if (v71)
                            {
                                if (*((long long *)v71))
                                    sub_40b420(*((long long *)v71), &v3);
                                if (*((long long *)(v71 + 8)))
                                    sub_40b420(*((long long *)(v71 + 8)), &v3);
                                v3 = *((long long *)(v71 + 16));
                                *((long long *)&v3[24]) = *((long long *)(v71 + 16));
                            }
                            v72 = idx2->field_8;
                            v73 = v3;
                            if (v72)
                            {
                                if (*((long long *)v72))
                                    sub_40b420(*((long long *)v72), &v3);
                                if (*((long long *)(v72 + 8)))
                                    sub_40b420(*((long long *)(v72 + 8)), &v3);
                                v73 = *((long long *)(v72 + 16));
                                *((char **)&v3[24]) = v73;
                            }
                            v70 = idx2->field_10;
                            *((char **)&v73[24]) = v70;
                        }
                        v74 = *((long long *)(v65 + 16));
                        *((char **)&v70[24]) = v74;
                        v3 = v74;
                    }
                    v75 = v3;
                    if (v16->field_8)
                    {
                        v76 = v16->field_8->field_0;
                        if (v76)
                        {
                            v77 = *((long long *)v76);
                            if (v77)
                            {
                                if (v77->field_0)
                                {
                                    v1 = v77;
                                    sub_40b420(v77->field_0, &v3);
                                    v77 = v1;
                                }
                                if (v77->field_8)
                                {
                                    v1 = v77;
                                    sub_40b420(v77->field_8, &v3);
                                    v77 = v1;
                                }
                                v3 = v77->field_10;
                                *((char **)&v3[24]) = v77->field_10;
                            }
                            v78 = *((long long *)(v76 + 8));
                            if (v78)
                            {
                                if (v78->field_0)
                                {
                                    v1 = v78;
                                    sub_40b420(v78->field_0, &v3);
                                    v78 = v1;
                                }
                                if (v78->field_8)
                                {
                                    v1 = v78;
                                    sub_40b420(v78->field_8, &v3);
                                    v78 = v1;
                                }
                                v79 = v78->field_10;
                                *((char **)&v3[24]) = v79;
                            }
                            else
                            {
                                v79 = v3;
                            }
                            v80 = *((long long *)(v76 + 16));
                            *((char **)&v79[24]) = v80;
                            v3 = v80;
                        }
                        v81 = idx2->field_8;
                        v82 = v3;
                        if (v81)
                        {
                            v83 = *((long long *)v81);
                            if (v83)
                            {
                                if (*((long long *)v83))
                                {
                                    v1 = v83;
                                    sub_40b420(*((long long *)v83), &v3);
                                    v83 = v1;
                                }
                                if (*((long long *)(v83 + 8)))
                                {
                                    v1 = v83;
                                    sub_40b420(*((long long *)(v83 + 8)), &v3);
                                    v83 = v1;
                                }
                                v3 = *((long long *)(v83 + 16));
                                *((long long *)&v3[24]) = *((long long *)(v83 + 16));
                            }
                            v84 = *((long long *)(v81 + 8));
                            v85 = v3;
                            if (v84)
                            {
                                if (*((long long *)v84))
                                {
                                    v1 = v84;
                                    sub_40b420(*((long long *)v84), &v3);
                                    v84 = v1;
                                }
                                if (*((long long *)(v84 + 8)))
                                {
                                    v1 = v84;
                                    sub_40b420(*((long long *)(v84 + 8)), &v3);
                                    v84 = v1;
                                }
                                v85 = *((long long *)(v84 + 16));
                                *((char **)&v3[24]) = v85;
                            }
                            v82 = *((long long *)(v81 + 16));
                            *((char **)&v85[24]) = v82;
                        }
                        v75 = idx2->field_10;
                        *((char **)&v82[24]) = v75;
                    }
                    v86 = v16->field_10;
                    v63 = *((long long *)&idx1[40]);
                    *((char **)&v75[24]) = v86;
                    v87 = v29->field_10;
                    *((char **)&v86[24]) = v87;
                    v3 = v87;
                    v64 = v15;
                    if (!v64)
                        goto LABEL_40d923;
LABEL_40d613:
                    if (v64->field_0)
                    {
                        v88 = v64->field_0->field_0;
                        if (v88)
                        {
                            if (*((long long *)v88))
                            {
                                v1 = *((long long *)v88);
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v89 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v89->field_0 > v63)
                                    *((char *)&v89->field_0) = v63;
                            }
                            if (*((long long *)(v88 + 8)))
                            {
                                v1 = *((long long *)(v88 + 8));
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v90 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v90->field_0 > v63)
                                    *((char *)&v90->field_0) = v63;
                            }
                            v91 = *((char *)(v88 + 24)) + (char *)v9;
                            if ((char)v91->field_0 > v63)
                                *((char *)&v91->field_0) = v63;
                        }
                        v92 = v16->field_8;
                        if (v92)
                        {
                            if (*((long long *)v92))
                            {
                                v1 = *((long long *)v92);
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v93 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v93->field_0 > v63)
                                    *((char *)&v93->field_0) = v63;
                            }
                            if (*((long long *)(v92 + 8)))
                            {
                                v1 = *((long long *)(v92 + 8));
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v94 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v94->field_0 > v63)
                                    *((char *)&v94->field_0) = v63;
                            }
                            v95 = *((char *)(v92 + 24)) + (char *)v9;
                            if ((char)v95->field_0 > v63)
                                *((char *)&v95->field_0) = v63;
                        }
                        v96 = (char)v16->field_18 + (char *)v9;
                        if ((char)v96->field_0 > v63)
                            *((char *)&v96->field_0) = v63;
                    }
                    if (v15->field_8)
                    {
                        v97 = v15->field_8->field_0;
                        if (v97)
                        {
                            if (*((long long *)v97))
                            {
                                v1 = *((long long *)v97);
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v98 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v98->field_0 > v63)
                                    *((char *)&v98->field_0) = v63;
                            }
                            if (*((long long *)(v97 + 8)))
                            {
                                v1 = *((long long *)(v97 + 8));
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v99 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v99->field_0 > v63)
                                    *((char *)&v99->field_0) = v63;
                            }
                            v100 = *((char *)(v97 + 24)) + (char *)v9;
                            if ((char)v100->field_0 > v63)
                                *((char *)&v100->field_0) = v63;
                        }
                        v101 = v16->field_8;
                        if (v101)
                        {
                            if (*((long long *)v101))
                            {
                                v1 = *((long long *)v101);
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v102 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v102->field_0 > v63)
                                    *((char *)&v102->field_0) = v63;
                            }
                            if (*((long long *)(v101 + 8)))
                            {
                                v1 = *((long long *)(v101 + 8));
                                sub_40c100(*((long long *)v1), v63, v9);
                                sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                v103 = *((char *)(v1 + 24)) + (char *)v9;
                                if ((char)v103->field_0 > v63)
                                    *((char *)&v103->field_0) = v63;
                            }
                            v104 = *((char *)(v101 + 24)) + (char *)v9;
                            if ((char)v104->field_0 > v63)
                                *((char *)&v104->field_0) = v63;
                        }
                        v105 = (char)v16->field_18 + (char *)v9;
                        if ((char)v105->field_0 > v63)
                            *((char *)&v105->field_0) = v63;
                    }
                    v106 = v15->field_18 + (char *)v9;
                    if ((char)v106->field_0 > v63)
                        *((char *)&v106->field_0) = v63;
                    if (v29->field_8)
                    {
                        v65 = *((long long *)v29->field_8);
LABEL_40d923:
                        if (v65)
                        {
                            if (*((long long *)v65))
                            {
                                if (*((long long *)*((long long *)v65)))
                                {
                                    v1 = *((long long *)*((long long *)v65));
                                    sub_40c100(*((long long *)v1), v63, v9);
                                    sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                    v107 = *((char *)(v1 + 24)) + (char *)v9;
                                    if ((char)v107->field_0 > v63)
                                        *((char *)&v107->field_0) = v63;
                                }
                                if (v15->field_8)
                                {
                                    v1 = v15->field_8;
                                    sub_40c100(*((long long *)v1), v63, v9);
                                    sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                    v108 = *((char *)(v1 + 24)) + (char *)v9;
                                    if ((char)v108->field_0 > v63)
                                        *((char *)&v108->field_0) = v63;
                                }
                                v109 = v15->field_18 + (char *)v9;
                                if ((char)v109->field_0 > v63)
                                    *((char *)&v109->field_0) = v63;
                            }
                            if (*((long long *)(v65 + 8)))
                            {
                                if (*((long long *)*((long long *)(v65 + 8))))
                                {
                                    v1 = *((long long *)*((long long *)(v65 + 8)));
                                    sub_40c100(*((long long *)v1), v63, v9);
                                    sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                    v110 = *((char *)(v1 + 24)) + (char *)v9;
                                    if ((char)v110->field_0 > v63)
                                        *((char *)&v110->field_0) = v63;
                                }
                                if (v15->field_8)
                                {
                                    v1 = v15->field_8;
                                    sub_40c100(*((long long *)v1), v63, v9);
                                    sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                    v111 = *((char *)(v1 + 24)) + (char *)v9;
                                    if ((char)v111->field_0 > v63)
                                        *((char *)&v111->field_0) = v63;
                                }
                                v112 = v15->field_18 + (char *)v9;
                                if ((char)v112->field_0 > v63)
                                    *((char *)&v112->field_0) = v63;
                            }
                            v113 = *((char *)(v65 + 24)) + (char *)v9;
                            if ((char)v113->field_0 > v63)
                                *((char *)&v113->field_0) = v63;
                        }
                        if (v16->field_8)
                        {
                            v114 = v16->field_8->field_0;
                            if (v114)
                            {
                                if (*((long long *)v114))
                                {
                                    v1 = *((long long *)v114);
                                    sub_40c100(v1->field_0, v63, v9);
                                    sub_40c100(v1->field_8, v63, v9);
                                    v115 = v1->field_18 + (char *)v9;
                                    if ((char)v115->field_0 > v63)
                                        *((char *)&v115->field_0) = v63;
                                }
                                if (*((long long *)(v114 + 8)))
                                {
                                    v1 = *((long long *)(v114 + 8));
                                    sub_40c100(v1->field_0, v63, v9);
                                    sub_40c100(v1->field_8, v63, v9);
                                    v116 = v1->field_18 + (char *)v9;
                                    if ((char)v116->field_0 > v63)
                                        *((char *)&v116->field_0) = v63;
                                }
                                v117 = *((char *)(v114 + 24)) + (char *)v9;
                                if ((char)v117->field_0 > v63)
                                    *((char *)&v117->field_0) = v63;
                            }
                            v118 = v15->field_8;
                            if (v118)
                            {
                                if (*((long long *)v118))
                                {
                                    v1 = *((long long *)v118);
                                    sub_40c100(*((long long *)v1), v63, v9);
                                    sub_40c100(*((long long *)(v1 + 8)), v63, v9);
                                    v119 = *((char *)(v1 + 24)) + (char *)v9;
                                    if ((char)v119->field_0 > v63)
                                        *((char *)&v119->field_0) = v63;
                                }
                                if (*((long long *)(v118 + 8)))
                                {
                                    v1 = *((long long *)(v118 + 8));
                                    sub_40c100(v1->field_0, v63, v9);
                                    sub_40c100(v1->field_8, v63, v9);
                                    v120 = v1->field_18 + (char *)v9;
                                    if ((char)v120->field_0 > v63)
                                        *((char *)&v120->field_0) = v63;
                                }
                                v121 = *((char *)(v118 + 24)) + (char *)v9;
                                if ((char)v121->field_0 > v63)
                                    *((char *)&v121->field_0) = v63;
                            }
                            v122 = v15->field_18 + (char *)v9;
                            if ((char)v122->field_0 > v63)
                                *((char *)&v122->field_0) = v63;
                        }
                        v123 = (char)v16->field_18 + (char *)v9;
                        if ((char)v123->field_0 > v63)
                            *((char *)&v123->field_0) = v63;
                    }
                }
                else
                {
                    v124 = v41->field_10;
                    v63 = *((long long *)&v13[40]);
                    *((char **)&v3[24]) = v124;
                    v3 = v124;
                }
                v125 = (char)v29->field_18 + (char *)v9;
                if ((char)v125->field_0 > v63)
                    *((char *)&v125->field_0) = v63;
                v41 = *((long long *)&idx1[8]);
                v13 = idx1;
                goto LABEL_40dc38;
            }
LABEL_40dd0f:
            v8 = v2;
            if (v0 == 0x1)
                goto LABEL_40e1bb;
        }
        else
        {
LABEL_40e1bb:
            node = v16->field_18;
            if (node)
            {
                do
                {
                    v131 = node->field_38;
                    v132 = node->field_10->field_38;
                    if (v131 > v132)
                    {
                        node->field_38 = v132;
                        v131 = v132;
                    }
                    if (node->field_30 > v131)
                        node->field_30 = v131;
                    node = node->field_18;
                } while (node);
            }
        }
    }
    v133 = &v4;
    if (!v8)
        v133 = &index->padding_100[112];
    v133->field_0 = NULL;
    v133->field_7f8 = 0;
    iter1 = &v133->field_8 & 0xfffffffffffffff8;
    for (v135 = (unsigned int)(v133 - iter1) + 0x800 >> 3; v135; iter1 = &iter1[v37])
    {
        v135 -= 1;
        *(iter1) = 0;
    }
    sub_40ba10(*((long long *)&index->field_60[8]));
    iter2 = 0;
    v137 = 0xfffffffe;
    while (1)
    {
        if (v8)
        {
            v138 = v8[iter2];
            v139 = *((long long *)&idx1[8 * v138]);
            *((unsigned long *)&index->padding_100[112 + 8 * iter2]) = v139;
        }
        else
        {
            v139 = *((long long *)&index->padding_100[112 + 8 * iter2]);
            v138 = iter2 & 0xffffffff;
        }
        if (v139)
        {
            if (v137 == 0xfffffffe)
            {
                iter2 += 1;
                v137 = v138;
                if (iter2 == 0x100)
                    break;
                else
                    continue;
            }
            else if ((unsigned int)v138 != v137 && !((unsigned int)v138 == (unsigned int)iter2 && v137 == (unsigned int)v29))
            {
                v137 = 0xffffffff;
            }
        }
        iter2 += 1;
        if (iter2 == 0x100)
            break;
    }
    index->field_988 = v137;
    index->field_98c = (unsigned int)v29;
    if (v0 == 0x1)
    {
        v140 = index->field_20;
        v141 = index->field_68;
        v142 = v141;
        if (v141 <= v140 - index->field_18)
        {
            v143 = index->field_10;
            v144 = index->field_18 + v142;
            if (v144 == v143)
                goto LABEL_40e39e;
        }
        else
        {
            _obstack_newchunk(index);
            v143 = index->field_10;
            v140 = index->field_20;
            v141 = index->field_68;
            v144 = index->field_18 + v142;
            if (v144 == v143)
            {
LABEL_40e39e:
                *((char *)&index->field_50) = (char)index->field_50 | 2;
            }
        }
        v145 = index->field_30;
        index->field_970 = v143;
        v146 = v144 + v145 & ~(v145);
        if (v146 - index->field_8 <= v140 - index->field_8)
            v140 = v146;
        index->field_18 = v140;
        index->field_10 = v140;
        v147 = index->field_60;
        if (v141 > NULL)
        {
            v148 = v141 - 1;
            while (1)
            {
                v143[v148] = *((char *)(*((long long *)&v147[8]) + 24));
                v147 = *((long long *)&v147[24]);
                v149 = v148 - 1;
                if (v148 < 0x1)
                    break;
                v143 = index->field_970;
                v148 = v149;
            }
            v150 = index->field_68;
            if (v150 > 0x1)
            {
                v151 = index->field_20;
                v152 = index->field_18;
                v153 = v150 * 8 - 8;
                if (v153 > index->field_20 - index->field_18)
                {
                    _obstack_newchunk(index, v153);
                    v152 = index->field_18;
                    v151 = index->field_20;
                    v150 = index->field_68;
                }
                v154 = index->field_10;
                v155 = &v152[v153];
                if (v155 == v154)
                    *((char *)&index->field_50) = (char)index->field_50 | 2;
                v156 = index->field_30;
                index->field_978 = v154;
                v157 = v155 + v156 & ~(v156);
                if (v157 - index->field_8 <= v151 - index->field_8)
                    v151 = v157;
                v158 = index->field_60;
                index->field_18 = v151;
                index->field_10 = v151;
                v159 = *((long long *)&v158[24]);
                if (v150 > 0x1)
                {
                    m = 0;
                    do
                    {
                        v161 = *((long long *)&v159[24]);
                        *((long long *)&v154[8 * m]) = *((long long *)&v159[48]);
                        v150 = index->field_68;
                        m += 1;
                        v159 = v161;
                    } while (v150 - 0x1 > m);
                }
                v162 = *((char *)(index->field_970 + v150 - 2));
                if (v8)
                    v162 = v8[v162];
                index->field_990 = v162;
            }
        }
    }
    j = 0;
    if (v8)
    {
        do
        {
            *((char *)index + j + 112) = *((char *)v9 + v8[j]);
            j += 1;
        } while (j != 0x100);
    }
    return;
}



// Function: kwsexec @ 0xe3e0
typedef struct struct_0 {
    char padding_0[2456];
    unsigned long long field_998;
} struct_0;

long long kwsexec(struct_0 *a0, unsigned long a1, unsigned long a2, unsigned long a3, char a4)
{
    goto *((void *)(a0->field_998));
}



// Function: wordinit @ 0xe570
extern int g_42b284;
extern int g_42b684;
extern char g_42b9c0;

unsigned int wordinit(void)
{
    char *iter;  // rbp
    int *v2;  // rbx
    unsigned int v3;  // eax
    unsigned long long v4;  // rax

    iter = &g_42b9c0;
    v2 = &g_42b284;
    do
    {
        v3 = 1;
        if (*(v2) != 95)
        {
            v4 = iswalnum(*(v2));
            v3 = _INSERT(v4, 0, (unsigned int)v4);
        }
    } while ((v2 += 4, *(iter) = (char)v3, iter += 1, v2 != &g_42b684));
    return v3;
}



// Function: kwsinit @ 0xe5c0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
    char padding_51[7];
    unsigned long long field_58;
    unsigned long long field_60;
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
} struct_0;

extern char g_42b687;

struct_0 * kwsinit(void)
{
    unsigned long long v3;  // rbx
    char v4;  // dil
    long long v5;  // rdi
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    unsigned long long *v11;  // rax
    unsigned long long i;  // rax
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    if (!g_42b687)
        return sub_40c880(0);
    v1 = v3;
    if (__ctype_get_mb_cur_max() != 1 && !v4)
        return sub_40c880(0);
    v0 = sub_41ef30(0x100);
    v11 = __ctype_toupper_loc(v5, v6, v7, v8, v9, v10);
    i = 0;
    do
    {
        *((char *)(v0 + i)) = *((int *)(*(v11) + i * 4));
        i += 1;
    } while (i != 0x100);
    return sub_40c880(v0);
}



// Function: mb_goback @ 0xe640
extern char g_42b180;
extern char g_42b182;

void* mb_goback(void* *a0, unsigned long long *a1, void* a2, unsigned long a3)
{
    void* v2;  // rbx
    void* v3;  // rbx
    unsigned long v4;  // rdx
    unsigned long v5;  // rax
    unsigned long v6;  // rdx
    unsigned int v7;  // eax
    unsigned long long v8;  // rdx
    void* v9;  // r14
    long long v10;  // rdx
    unsigned long v0;  // [bp-0x48], Other Possible Types: unsigned long long

    v2 = *(a0);
    v3 = v2;
    if (v3 >= a2)
    {
        return a2 - v3;
    }
    if (!g_42b182)
    {
        v0 = 0;
        while (1)
        {
            v3 = v2;
            v4 = (&g_42b180)[3 + *((char *)v3)];
            if ((char)v4 == 254 && !(v5 = (unsigned long)(unsigned long long)sub_41ac90(v3, a3 - v3, &v0), v4 = v5, v5 > 16) || !(v5 = v4, v5 < 0))
            {
                v2 = v5 + v3;
                if (a2 <= v2)
                    break;
            }
            else
            {
                v0 = 0;
                v6 = 1;
                v2 = 1 + v3;
                if (a2 <= v2)
                    break;
            }
        }
        if (a1)
            *(a1) = v6;
    }
    else if ((*((char *)a2) & 192) != 128)
    {
        *(a0) = a2;
        return NULL;
    }
    else
    {
        v7 = *((char *)a2 - 1);
        if (((char)v7 & 192) == 128)
        {
            v7 = *((char *)a2 - 2);
            if (((char)v7 & 192) == 128)
            {
                v7 = *((char *)a2 - 3);
                if (((char)v7 & 192) == 128)
                {
                    *(a0) = a2;
                    return NULL;
                }
                v8 = 3;
            }
            else
            {
                v8 = 2;
            }
        }
        else
        {
            v8 = 1;
        }
        if ((int)(~(v7) & 0xff) >> ((char)(7 - (unsigned int)v8) & 31))
        {
            *(a0) = a2;
            return NULL;
        }
        v0 = 0;
        v9 = a2 - v8;
        v10 = sub_41ac90(v9, a3 - v9, &v0);
        if (v10 <= 16 || (v3 = v2, v2 = a2, v10 >= 0))
        {
            v2 = v9 + v10;
            v3 = v9;
        }
    }
    *(a0) = v2;
    if (v2 == a2)
        return NULL;
    return a2 - v3;
}



// Function: wordchars_size @ 0xe810
void wordchars_size(void)
{
}



// Function: wordchar_prev @ 0xe830
extern char g_42b180;
extern char g_42b182;
extern char g_42b9c0;

unsigned long long wordchar_prev(void* a0, void* a1, unsigned int a2)
{
    unsigned long v2;  // rdx
    void* v0;  // [bp-0x28]

    if (a0 == a1)
        return 0;
    v2 = *((char *)a1 - 1);
    if (!g_42b180 || g_42b182 & ~((char)v2 >> 7))
        return (&g_42b9c0)[v2];
    v0 = a0;
    sub_40e640(&v0, NULL, a1 - 1, a2);
    return (&g_42b9c0)[v2];
}



// Function: private_malloc @ 0xe900
void private_malloc(long long a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (a0 < 0)
    {
        v0 = v2;
        sub_41f410(); /* do not return */
    }
    sub_41ef30();
    return;
}



// Function: jit_exec @ 0xe920
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    unsigned long long field_20;
    long long field_28;
} struct_0;

int jit_exec(struct_0 *idx, long long a1, long long a2, long long a3, unsigned int a4)
{
    unsigned int v4;  // r15d
    unsigned long long v5;  // rdi
    unsigned long long v6;  // rax
    long long v7;  // r9
    unsigned long long v8;  // r15
    unsigned long long v9;  // rdi
    unsigned long long v10;  // rax
    unsigned long long v11;  // rdi
    unsigned long long v12;  // rax
    long long v0;  // [bp-0x50]
    unsigned int v1;  // [bp-0x44]
    unsigned int v2;  // [bp-0x44]

    while (1)
    {
        v4 = pcre2_match_8(idx->field_8, a1, a2, a3, a4, idx->field_18, idx->field_10);
        switch (v4)
        {
        case 4294967250:
            v7 = idx->field_28;
            if (v7 > 0x3fffffffffffffff)
                return v4;
            v8 = v7 * 2;
            v9 = idx->field_20;
            v0 = v7;
            idx->field_28 = v8;
            pcre2_jit_stack_free_8(v9);
            v10 = pcre2_jit_stack_create_8(v0, v8, idx->field_0);
            idx->field_20 = v10;
            if (!v10)
                sub_41f410(); /* do not return */
            v11 = idx->field_10;
            if (!v11)
            {
                v12 = pcre2_match_context_create_8(idx->field_0);
                idx->field_10 = v12;
                v11 = v12;
            }
            pcre2_jit_stack_assign_8(v11, 0);
            break;
            break;
        case 4294967243:
            pcre2_config_8(7, &v1);
            v2 = v1 * 2;
            if (v1 < 0)
                return v4;
            v5 = idx->field_10;
            if (!v5)
            {
                v6 = pcre2_match_context_create_8(idx->field_0);
                idx->field_10 = v6;
                v5 = v6;
            }
            pcre2_set_depth_limit_8(v5);
            v1 = v2;
            break;
        default:
            return v4;
        }
    }
}



// Function: Pcompile @ 0xea80
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    unsigned long long field_20;
    long long field_28;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    unsigned short field_8;
} struct_1;

extern long long g_4236a2;
extern char g_42b180;
extern char g_42b182;
extern char g_42b685;
extern char g_42b686;
extern char g_42b687;

struct_0 * Pcompile(struct_1 *a0, unsigned long a1)
{
    struct_1 *v7;  // r14
    struct_0 *idx;  // r12
    unsigned long long v17;  // rax
    unsigned long long v18;  // rdi
    unsigned long long v19;  // rax
    unsigned long long v20;  // rdi
    unsigned long v21;  // rax
    unsigned int v9;  // r13d
    long long v10;  // rax
    long long v11;  // rbp
    long long v12;  // r15
    struct_1 *v14;  // r10
    struct_0 *index;  // rax
    unsigned long v0;  // [bp-0x288]
    unsigned long v1;  // [bp-0x168]
    void* ptr;  // [bp-0x160], Other Possible Types: struct_1 *
    char v3;  // [bp-0x154], Other Possible Types: unsigned int
    char v4;  // [bp-0x150]
    char v5;  // [bp-0x148]

    v7 = a0;
    v1 = a1;
    ptr = (char *)a0 + a1;
    idx = sub_41ef30(56);
    v9 = (-(g_42b687 < 1) & 0xfffffff8) + 24;
    v10 = pcre2_general_context_create_8(sub_40e900, sub_40e8f0, 0);
    idx->field_0 = v10;
    v11 = v10;
    v12 = pcre2_compile_context_create_8(v11);
    if (g_42b180)
    {
        if (g_42b182)
            v9 |= 0x4080000;
        else
            error(2, 0, dcgettext(NULL, "-P supports only unibyte and UTF-8 locales", 5));
    }
    if (ptr != rawmemchr(v7, 10))
        error(2, 0, dcgettext(NULL, "the -P option only supports a single pattern", 5));
    if (!g_42b685)
    {
        v14 = NULL;
        if (g_42b686)
        {
            v1 = a1 + 0x11;
            ptr = sub_41ef30(v1);
            ptr->field_0 = 2893975473140547368;
            ptr->field_8 = 14911;
            index = mempcpy(ptr + 1, v7, a1);
            v14 = ptr;
            *((unsigned int *)&index->field_0) = 557787177;
            v7 = v14;
            *((unsigned short *)((char *)&index->field_0 + 4)) = 30556;
            *((char *)&index->field_0 + 6) = 41;
        }
    }
    else
    {
        pcre2_set_compile_extra_options_8(v12, 8);
        v14 = NULL;
    }
    ptr = v14;
    pcre2_set_character_tables_8(v12, pcre2_maketables_8(v11));
    v17 = pcre2_compile_8(v7, v1, v9, &v3, &v4, v12);
    idx->field_8 = v17;
    if (!v17)
    {
        pcre2_get_error_message_8(v3, &v5, 0x100);
        error(2, 0, "%s");
    }
    free(ptr);
    pcre2_compile_context_free_8(v12);
    v18 = idx->field_8;
    idx->field_10 = 0;
    v19 = pcre2_match_data_create_from_pattern_8(v18, v11);
    v20 = idx->field_8;
    idx->field_18 = v19;
    v3 = pcre2_jit_compile_8(v20, 1);
    v21 = v3 + 48;
    if ((unsigned int)v21 <= 48)
    {
        v0 = 0xfffefffffffffff6;
        if (!(((char)v21 & 254 | *((char *)&v0 + ((long long)(v21 & 63) >> 3)) >> ((char)v21 & 63 & 7) & 1) & 1))
        {
            idx->field_20 = 0;
            idx->field_28 = 0x8000;
            *((int *)&idx[1].field_0) = sub_40e920(idx, &g_4236a2, 0, 0, 1);
            *((int *)((char *)&idx[1].field_0 + 4)) = sub_40e920(idx, &g_4236a2, 0, 0, 0);
            return idx;
        }
    }
    error(2, 0, dcgettext(NULL, "JIT internal error: %d", 5));
}



// Function: Pexecute @ 0xeda0
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    unsigned long long field_20;
    long long field_28;
} struct_0;

extern unsigned int g_422940[4];
extern char g_42b180;
extern char g_42b684;

unsigned long long Pexecute(struct_0 *a0, void* a1, unsigned long a2, unsigned long long *a3, void* a4)
{
    void* v4;  // rbx
    unsigned int v5;  // ebp
    void* ptr;  // r14
    void* v7;  // rbx
    void* v8;  // rbx
    void* v9;  // r15
    void* v10;  // rcx
    int v11;  // r8d
    void* v12;  // r10
    void* v0;  // [bp-0x68], Other Possible Types: int
    void* *idx;  // [bp-0x50]
    char v2;  // [bp+0x0]

    if (a4)
        v4 = a4;
    else
        v4 = a1;
    v5 = _INSERT(&v2, 0, *((char *)v4 - 1) == g_42b684);
    v0 = a1;
    idx = pcre2_get_ovector_pointer_8();
    while (1)
    {
        ptr = rawmemchr(v4, g_42b684);
        if ((&g_42b180)[3 + *((char *)v4)] == 0xff)
        {
            do
            {
                v8 = v4 + 1;
                v4 = v8;
            } while ((&g_42b180)[3 + (char)v7[1]] == 0xff);
            v9 = v8;
            v10 = NULL;
            v5 = 0;
            v4 = v8;
        }
        else
        {
            v9 = v0;
            v10 = v4 - v9;
        }
        if (ptr == v4)
        {
            v11 = *((int *)((char *)a0 + 4 * (v5 & 1) + 48));
            idx[1] = v10;
            *(idx) = v10;
            if (v11 != -0x1)
                break;
        }
        else if (sub_40e920(a0, v9, ptr - v9, v10, (v5 ^ 1) & 1) != -0x1)
        {
            break;
        }
        v4 = ptr + 1;
        v5 = 1;
        if (v4 >= a1 + a2)
            return 0xffffffffffffffff;
        v0 = v4;
    }
    if (v11 > 0)
    {
        v12 = ptr + 1;
        if (a4)
        {
            v12 = v9 + idx[1];
            v0 = v9 + *(idx);
        }
        *(a3) = v12 - v0;
        return v0 - a1;
    }
    if (v11 < -0x2d && v11 >= -0x3f && v11 + 63 <= 0x11)
        goto *((void *)((long long)(g_422940[63 + v11] + (char *)&g_422940[0])));
    v0 = v11;
    sub_4090f0();
    error(2, 0, dcgettext(NULL, "%s: internal PCRE error: %d", 5));
}



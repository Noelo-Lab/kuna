// Function: main @ 0x5090
typedef struct struct_1 {
    char padding_0[12];
    int field_c;
    struct struct_2 *field_10;
    unsigned long long field_18;
} struct_1;

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

typedef struct struct_2 {
    unsigned long long field_0;
} struct_2;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned int g_420788[4];
extern char g_4216a2;
extern long long g_428f60;
extern long long g_428f70;
extern long long g_428f80;
extern void g_42a014;
extern unsigned int g_42a018;
extern unsigned long long g_42a060;
extern unsigned long long g_42a068;
extern unsigned long long g_42a070;
extern long long g_42a078;
extern unsigned int g_42a080;
extern long long stdout;
extern unsigned long long stdin;
extern unsigned int optind;
extern unsigned long long optarg;
extern long long g_42a180;
extern char g_42a684;
extern char g_42a685;
extern char g_42a686;
extern char g_42a687;
extern char g_42a7a0;
extern char g_42a7a1;
extern char g_42a7a2;
extern unsigned long long g_42a7d0;
extern char g_42a7d8;
extern unsigned long long g_42a7e0;
extern char g_42a7e8;
extern unsigned long long g_42a7f0;
extern unsigned long long g_42a7f8;
extern char g_42a800;
extern char g_42a801;
extern char g_42a802;
extern char g_42a803;
extern unsigned int g_42a804;
extern unsigned int g_42a808;
extern unsigned int g_42a80c;
extern unsigned int g_42a810;
extern char g_42a820;
extern unsigned long long g_42a838;
extern long long g_42a858;
extern unsigned long long g_42a860;
extern unsigned long long g_42a870;
extern struct_2 *g_42a878;
extern unsigned int g_42a884;
extern char g_42a889;
extern char g_42a88a;
extern unsigned long long g_42a8a0;
extern unsigned long long g_42a8a8;
extern unsigned long long g_42a8b0[4];
extern long long g_42a8c8;
extern long long g_42a8d0;
extern struct_0 *g_42a8d8;
extern char g_42a8fc;
extern char g_42a8fd;
extern unsigned int g_42a900;
extern char g_42a904;
extern char g_42a905;
extern unsigned int g_42a908;
extern unsigned int g_42a920;

unsigned int main(unsigned int a0, long long a1)
{
    long long v20;  // r14
    unsigned int v21;  // ebx
    unsigned int v30;  // eax
    long long v31;  // r8
    unsigned int v32;  // eax
    unsigned int v33;  // eax
    unsigned long long fp;  // r13
    unsigned long v35;  // r14
    char *v36;  // r12
    unsigned long v37;  // rdx
    unsigned long v38;  // r11
    char *v22;  // rbp
    unsigned long v39;  // rbx
    char *v40;  // rbp
    unsigned long count;  // rax
    unsigned long v42;  // r12
    char *v43;  // r8
    unsigned long long v44;  // rsi
    unsigned long len;  // r12
    char *v46;  // rax
    unsigned int v47;  // eax
    unsigned long long idx;  // r13
    unsigned int v23;  // r12d
    unsigned long long v49;  // rdi
    unsigned long v50;  // rsi
    unsigned long v51;  // rdi
    char *v53;  // rax
    char *v54;  // rax
    unsigned long long v55;  // rdx
    char *v56;  // rax
    unsigned int v24;  // eax
    char *v57;  // r15
    unsigned long len1;  // rax
    char v59;  // r12b
    char *v60;  // r13
    unsigned int *iter;  // rdi
    unsigned long long v65;  // rcx
    unsigned long long v66;  // 4103
    unsigned long long index;  // r12
    unsigned int v67;  // eax
    char *v68;  // r12
    char v69;  // al
    char *node;  // r13
    unsigned long v72;  // rax
    unsigned long long v73;  // r12
    unsigned long v74;  // r12
    char v75;  // al
    struct_1 *v76;  // rax
    unsigned long long v26;  // rdi
    unsigned long long v77;  // rax
    int v78;  // ebx
    unsigned int v79;  // eax
    unsigned long long v80;  // rcx
    unsigned long long v81;  // rdx
    unsigned long long v82;  // rax
    unsigned int v83;  // ebp
    long long *iter1;  // rbx
    long long i;  // rdi
    unsigned int v86;  // eax
    unsigned long v27;  // rdi
    unsigned int v87;  // eax
    long long *v88;  // rax
    long long v89;  // rax
    int *err;  // rax
    int *err1;  // rax
    unsigned int v28;  // eax
    char *v0;  // [bp-0x1c8]
    unsigned int v1;  // [bp-0x1c0]
    unsigned int v2;  // [bp-0x1bc]
    char *v3;  // [bp-0x1b8], Other Possible Types: unsigned long long, unsigned int
    unsigned long v4;  // [bp-0x1b0]
    unsigned int v5;  // [bp-0x1a8]
    unsigned int v6;  // [bp-0x1a4]
    unsigned int *err2;  // [bp-0x1a0], Other Possible Types: long long
    char *v8;  // [bp-0x198]
    unsigned long v9;  // [bp-0x190]
    unsigned long v10;  // [bp-0x188]
    unsigned long v11;  // [bp-0x180]
    unsigned long v12;  // [bp-0x178]
    char v13;  // [bp-0x170]
    stat v14;  // [bp-0x168]
    stat v15;  // [bp-0xd8]
    char v16;  // [bp-0x43]
    char v17;  // [bp-0x42]
    char v18;  // [bp-0x41]

    v20 = a1;
    v21 = a0;
    g_42a684 = 10;
    v10 = 0;
    v11 = 0;
    g_42a080 = 2;
    g_42a804 = 0xffffffff;
    g_42a7e0 = 0x7fffffffffffffff;
    g_42a7f8 = 0xffffffffffffffff;
    g_42a7f0 = 0xffffffffffffffff;
    v12 = 0xffffffffffffffff;
    g_42a8fd = 0;
    setlocale(6, &g_4216a2);
    bindtextdomain("grep", "/usr/local/share/locale");
    textdomain("grep");
    sub_4145d0(&g_42a180);
    sub_41e1d0(sub_407a80);
    sub_40dd40(0);
    g_42a8c8 = sub_418320(0, 0, sub_4073d0, sub_407420, 0);
    if (!g_42a8c8)
        sub_41d4b0(); /* do not return */
    v5 = 0;
    v22 = &v12;
    v2 = 0;
    v1 = 0xffffffff;
    v0 = NULL;
    while (1)
    {
        v23 = optind;
        v24 = sub_407aa0(v21, v20, v22);
        if (v24 != 0xffffffff)
        {
            switch (v24)
            {
            case 0: case 85:
                break;
            case 65:
                sub_407900(optarg, &g_42a7f0);
                continue;
            case 66:
                sub_407900(optarg, &g_42a7f8);
                continue;
            case 67:
                sub_407900(optarg, v22);
                continue;
            case 68:
                if (!strcmp(optarg, "read"))
                {
                    g_42a884 = 1;
                    break;
                }
                else if (!strcmp(optarg, "skip"))
                {
                    g_42a884 = 2;
                    break;
                }
                else
                {
                    error(2, 0, dcgettext(NULL, "unknown devices method", 5));
                }
            case 69:
                v1 = sub_4077b0("egrep", v1);
                continue;
            case 70:
                v1 = sub_4077b0("fgrep", v1);
                continue;
            case 71:
                v1 = sub_4077b0("grep", v1);
                continue;
            case 72:
                v5 = 1;
                break;
            case 76:
                g_42a80c = 2;
                continue;
            case 80:
                v1 = sub_4077b0("perl", v1);
                continue;
            case 82:
                *((unsigned int *)&g_42a014) = 778;
            case 84:
                g_42a8fc = 1;
                continue;
            case 86:
                g_42a905 = 1;
                continue;
            case 88:
                v1 = sub_4077b0(optarg, v1);
                continue;
            case 90:
                g_42a804 = 0;
                continue;
            case 97:
                g_42a810 = 1;
                continue;
            case 98:
                g_42a800 = 1;
                continue;
            case 99:
                g_42a7e8 = 1;
                continue;
            case 100:
                v47 = g_420788[sub_40d900("--directories", optarg, &g_428f80, &g_420788[0], 4, g_42a078, 1)];
                if (g_420788[sub_40d900("--directories", optarg, &g_428f80, &g_420788[0], 4, g_42a078, 1)] != 3)
                    v23 = v2;
                g_42a018 = g_420788[sub_40d900("--directories", optarg, &g_428f80, &g_420788[0], 4, g_42a078, 1)];
                v2 = v23;
                continue;
            case 101:
                v3 = optarg;
                v44 = v3;
                len = strlen(optarg);
                if (len + v10 - v11 + 1 > 0)
                {
                    v46 = sub_41d2b0(v0, &v11);
                    v44 = optarg;
                    v0 = v46;
                    g_42a8d8 = v0;
                }
                *((char *)mempcpy(&v0[v10], v44, len)) = 10;
                v10 = sub_407bd0(v0, v10, len + v10 + 1, &g_4216a2);
                continue;
            case 102:
                v33 = strcmp(optarg, "-");
                fp = stdin;
                if (v33)
                {
                    fp = fopen(optarg, "r");
                    if (!fp)
                    {
                        err1 = __errno_location();
                        error(2, *(err1), "%s");
                    }
                }
                err2 = v20;
                v6 = v21;
                v4 = v10;
                v35 = v4;
                v3 = &v11;
                v36 = v0;
                v8 = v22;
                while (1)
                {
                    v37 = v11;
                    v38 = v35 - v37 + 2;
                    if (v38 > 0)
                    {
                        v37 = v11;
                        g_42a8d8 = sub_41d2b0(v36, v3, v38, -0x1, 1);
                        v36 = &g_42a8d8->field_0;
                    }
                    count = fread_unlocked(&v36[v35], 1, v37 - (v35 + 1), fp);
                    if (!count)
                        break;
                    v35 += count;
                }
                v9 = v39;
                v21 = v6;
                v3 = v40;
                v22 = v8;
                v0 = v36;
                v42 = v35;
                v20 = err2;
                err2 = __errno_location();
                v6 = *(err2);
                v43 = v3;
                if (ferror_unlocked(fp))
                {
LABEL_405751:
                    if (v6)
                        error(2, v6, "%s");
                }
                else if (stdin != fp)
                {
                    v3 = v43;
                    v43 = v3;
                    if (fclose(fp))
                    {
                        v6 = *(err2);
                        goto LABEL_405751;
                    }
                }
                else
                {
                    clearerr_unlocked(fp);
                    v43 = v3;
                }
                if (v42 != v4 && *(&v0[v42] - 1) != 10)
                {
                    *(v43) = 10;
                    v42 = v9;
                }
                v10 = sub_407bd0(v0, v4, v42, optarg);
                continue;
            case 104:
                v5 = 0xffffffff;
                continue;
            case 105: case 121:
                g_42a687 = 1;
                continue;
            case 108:
                g_42a80c = 1;
                continue;
            case 109:
                if (sub_41d560(optarg, 0, 10, &g_42a7e0, &g_4216a2) > 1)
                    error(2, 0, dcgettext(NULL, "invalid max count", 5));
                continue;
            case 110:
                g_42a801 = 1;
                continue;
            case 111:
                g_42a8fd = 1;
                continue;
            case 113:
                g_42a7a1 = 1;
                g_42a080 = 0;
                continue;
            case 114:
                g_42a018 = 3;
                v2 = v23;
                continue;
            case 115:
                g_42a904 = 1;
                continue;
            case 117:
                error(0, 0, dcgettext(NULL, "warning: --unix-byte-offsets (-u) is obsolete", 5));
                continue;
            case 118:
                g_42a802 = 1;
                continue;
            case 119:
                sub_40ca30();
                g_42a686 = 1;
                continue;
            case 120:
                g_42a685 = 1;
                continue;
            case 122:
                g_42a684 = 0;
                continue;
            case 129:
                if (optarg)
                {
                    if (!sub_40ddf0(optarg, "always") || !sub_40ddf0(optarg, "yes") || !sub_40ddf0(optarg, "force"))
                    {
                        g_42a900 = 1;
                        continue;
                    }
                    else if (!(sub_40ddf0(optarg, "never") && sub_40ddf0(optarg, "no") && sub_40ddf0(optarg, "none")))
                    {
                        g_42a900 = 0;
                        continue;
                    }
                    else if (sub_40ddf0(optarg, "auto") && sub_40ddf0(optarg, "tty") && sub_40ddf0(optarg, "if-tty"))
                    {
                        g_42a908 = 1;
                        continue;
                    }
                }
                g_42a900 = 2;
                continue;
            case 130:
                sub_4147b0(optarg);
                if (!g_42a8a0)
                    g_42a8a0 = sub_414d20();
                v30 = sub_4076d0(0);
                sub_414fb0(v31, optarg, v30);
                if (!g_42a8a8)
                    g_42a8a8 = sub_414d20();
                v32 = sub_4076d0(1);
                sub_414fb0(v31, optarg, v32);
                continue;
            case 131: case 134:
                index = 0;
                while (1)
                {
                    v26 = index & 0xffffffff;
                    if (!g_42a8b0[index])
                    {
                        v3 = index;
                        v27 = v3;
                        g_42a8b0[index] = sub_414d20(v26);
                        v26 = v27;
                    }
                    v28 = sub_4076d0(v26);
                    sub_414fb0(v31, optarg, v28 | (v24 == 134) * 0x20000000);
                    if (index == 1)
                        break;
                    index = 1;
                }
            case 132:
                idx = 0;
                while (1)
                {
                    v49 = idx & 0xffffffff;
                    <None|const 0> = g_42a8b0[idx];
                    if (!v50)
                    {
                        v3 = idx;
                        v51 = v3;
                        v50 = sub_414d20(v49, 0);
                        g_42a8b0[idx] = v50;
                        v49 = v51;
                    }
                    sub_4076d0(v49, v50);
                    if (sub_415440(sub_414fb0))
                    {
                        err = __errno_location();
                        error(2, *(err), "%s");
                    }
                    if (idx == 1)
                        break;
                    idx = 1;
                }
            case 133:
                g_42a070 = optarg;
                continue;
            case 135:
                g_42a7d8 = 1;
                continue;
            case 136:
                g_42a7d0 = optarg;
                continue;
            case 137:
                g_42a687 = 0;
                continue;
            case 128:
                if (!strcmp(optarg, "binary"))
                {
                    g_42a810 = 0;
                    continue;
                }
                if (strcmp(optarg, "text"))
                {
                    if (strcmp(optarg, "without-match"))
                        error(2, 0, dcgettext(NULL, "unknown binary-files type", 5));
                    goto LABEL_4052c3;
                }
                break;
            case 73:
LABEL_4052c3:
                g_42a810 = 2;
                continue;
            default:
LABEL_40605a:
                sub_40a440(2); /* do not return */
            }
        }
        else
        {
            if (g_42a905)
            {
                v89 = sub_4179e0();
                sub_41cf80(stdout, v89, "GNU grep", "3.8", 0);
                puts(dcgettext(NULL, "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.", 5));
                return 0;
            }
            if (g_42a908)
                sub_40a440(0); /* do not return */
            v53 = v0;
            if (!v53)
            {
                if (optind >= v21)
                    goto LABEL_40605a;
                v54 = *((long long *)(v20 + optind * 8));
                optind = optind + 1;
                v55 = 0;
                if (v1 != 2 && *(v54) == 92)
                    v55 = v54[1] == 45;
                v56 = sub_41d490(&v54[v55]);
                v57 = v56;
                v0 = v56;
                g_42a8d8 = v56;
                len1 = strlen(v56);
                v57[len1] = 10;
                v10 = sub_407bd0(v57, 0, len1 + 1, &g_4216a2);
            }
            else if (!v10)
            {
                g_42a686 = 0;
                g_42a685 = 0;
                g_42a802 = g_42a802 ^ 1;
                v10 = 1;
                *(v53) = 10;
            }
            v10 -= 1;
            sub_4184e0(g_42a8c8);
            v59 = g_42a7a1;
            if (g_42a7a1)
            {
                v59 = 0;
                goto LABEL_405952;
            }
            else
            {
                v60 = &v14;
                if (!fstat(1, &v14))
                {
                    switch (v14.st_nlink & 0xf000)
                    {
                    case 32768:
                        iter = &g_42a920;
                        for (v65 = 36; v65; v60 += 4)
                        {
                            v65 -= 1;
                            *(iter) = *((int *)&v60);
                            iter += 1;
                        }
                    case 8192:
                        if (stat("/dev/null", &v15) || v14.st_ino != v15.st_ino || *((unsigned long long *)&v14) != *((unsigned long long *)&v15))
                        {
                            v59 = 1;
                            break;
                        }
                        else
                        {
                            g_42a7a0 = 1;
                            break;
                        }
                    default:
                        goto LABEL_405dac;
                    }
                }
LABEL_405dac:
                if (g_42a7a0)
                {
LABEL_405952:
                    g_42a80c = 0;
                    goto LABEL_40595c;
                }
                else
                {
                    if (!g_42a80c)
                        break;
LABEL_40595c:
                    g_42a7e8 = 0;
                    g_42a7a2 = 1;
                    break;
                }
            }
        }
    }
    v66 = g_42a7f0;
    g_42a803 = g_42a7e8 | g_42a7a2;
    if (v66 - 0 >> 63 & 1)
        g_42a7f0 = v12;
    if (g_42a7f8 - 0 >> 63 & 1)
        g_42a7f8 = v12;
    if ((!g_42a7e0 || !v10 && g_42a802 && !g_42a685 && !g_42a686) && g_42a80c != 2)
        return 1;
    if (g_42a900 == 2)
    {
        if (!v59 || !sub_41d920())
            v67 = 0;
        else
            v67 = isatty(1);
        g_42a900 = v67;
    }
    sub_41d960();
    if (g_42a900)
    {
        v68 = getenv("GREP_COLOR");
        if (v68)
        {
            v69 = *(v68);
            if (*(v68))
            {
                for (node = v68; v69 == 59 || (char)sub_40da90(v69); node += 1)
                {
                    if (!node[1])
                    {
                        g_42a060 = v68;
                        g_42a068 = v68;
                        break;
                    }
                }
            }
        }
        sub_407970();
        if (g_42a068 == v68 || g_42a060 == v68)
            error(0, 0, dcgettext(NULL, "warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'", 5));
    }
    sub_407520();
    v72 = v1;
    v73 = 0;
    if ((unsigned int)v72 >= 0)
    {
        v74 = v72;
        v73 = v74;
    }
    if ((unsigned int)v74 <= 2)
    {
        if (g_42a687)
            sub_407d30();
        if (v1 == 2)
        {
            if (!(char)g_42a180)
            {
                v75 = 0;
                if (g_42a8d0 == 1)
                    v75 = g_42a686;
            }
            else
            {
                v75 = 1;
                if (!(char)(sub_407730(v0, v10) & 0xffffffff))
                {
                    v75 = 0;
                    if (g_42a687)
                        v75 = (char)sub_407f20(v0, v10) ^ 1;
                }
            }
            v73 = 2;
            if (v75 & 1)
            {
                v73 = 0;
                sub_40a6b0(&g_42a8d8, &v10);
                v0 = &g_42a8d8->field_0;
            }
        }
        else if (g_42a8d0 > 1)
        {
            v73 = sub_407fb0(v73 & 0xffffffff, v0, &v10);
        }
    }
    v76 = 32 * v73 + 4364160;
    g_42a878 = v76->field_18;
    g_42a870 = v76->field_10(v0, v10, v76->field_c, g_42a8fd || g_42a900);
    v16 = 0;
    v17 = g_42a684;
    v18 = 0;
    v77 = g_42a878(g_42a870, &v17, 1, &v13, 0);
    g_42a820 = !v77 == g_42a802;
    v78 = v21 - optind;
    if (!v5 && v78 <= 1)
        v79 = -(g_42a018 == 3);
    else
        v79 = ~(v5) >> 31;
    g_42a808 = v79;
    v80 = sysconf(30);
    if (v80 - 1 > 0x3ffffffffffffffa)
        abort(); /* do not return */
    g_42a838 = v80;
    v81 = 0x18000 % v80;
    v82 = 0x18000;
    if (v81)
        v82 = v80 + 0x18000 - v81;
    g_42a858 = v80 + v82 + 8;
    g_42a860 = sub_41d110(g_42a858);
    if (g_42a014 & 2 && !g_42a884)
        g_42a884 = 1;
    if (v78 > 0)
    {
        v83 = 1;
        iter1 = v20 + optind * 8;
        i = *(iter1);
        do
        {
            v86 = sub_40a3f0(i);
            i = iter1[1];
            iter1 += 1;
            v83 &= v86;
        } while (i);
        v87 = v83 & 0xff;
    }
    else
    {
        if (g_42a018 != 3 || v2 <= 0)
        {
            v88 = &g_428f60;
        }
        else
        {
            g_42a88a = 1;
            v88 = &g_428f70;
        }
        v87 = sub_40a3f0(*(v88)) & 0xff;
    }
    if (g_42a889)
        v87 = 2;
    return v87;
}



// Function: possible_backrefs_in_pattern @ 0x62e0
typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

struct_0 * possible_backrefs_in_pattern(struct_0 *a0, unsigned long long a1, char a2)
{
    struct_0 *v1;  // rbx
    struct_0 *ptr;  // rax
    unsigned int v3;  // edx

    v1 = (char *)a0 + a1 - 1;
    if (a1 - 1 >> 63 & 1)
        return NULL;
    while (1)
    {
        ptr = memchr(a0, 92, v1 - a0);
        if (!ptr)
            return ptr;
        v3 = ptr->field_1;
        if ((char)(v3 - 49) <= 8)
            return 0x1;
        if (v3 == (!(a2 < 1) & 36) + 92)
        {
            ptr = &ptr->field_1;
            if (v1 == ptr)
                return NULL;
        }
        a0 = &ptr->field_1;
    }
}



// Function: kwsmusts @ 0x6350
void kwsmusts(unsigned long a0)
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
    sub_40635d();
    return;
}



// Function: regex_compile @ 0x6460
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

extern char g_42a687;

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
    if (g_42a687 || (char)a6)
    {
        *((unsigned long long *)&v3) = 0;
        *((unsigned long long *)((char *)&v3 + 8)) = 0;
        if (!(char)a6)
            goto LABEL_406570;
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
        *((unsigned long long *)&v3) = sub_41d110(0x100);
LABEL_406570:
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
    if (a4 != -0x1 && *((char *)sub_4089e0(a4, &v0)))
    {
        error(0, 0, "%s:%td: %s");
        return 0;
    }
    error(0, 0, "%s");
    return 0;
}



// Function: dfaerror @ 0x6630
int dfaerror(unsigned long a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_406644(); /* do not return */
}



// Function: dfawarn @ 0x6660
void dfawarn(unsigned long a0)
{
    error(0, 0, dcgettext(NULL, "warning: %s", 5));
    return;
}



// Function: GEAcompile @ 0x6690
typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
} struct_1;

extern char g_400000;
extern char g_42a180;
extern char g_42a182;
extern char g_42a684;
extern char g_42a685;
extern char g_42a686;
extern char g_42a687;

struct_1 * GEAcompile(unsigned long a0, unsigned long a1, unsigned long long a2, char a3)
{
    unsigned long long v14;  // r15
    struct_1 *idx;  // r12
    unsigned long v24;  // rbp
    unsigned long long v25;  // rax
    unsigned long v26;  // r14
    unsigned long long v27;  // rdx
    unsigned long long v28;  // rbx
    unsigned long v29;  // r15
    long long v30;  // r14
    unsigned long v31;  // rax
    unsigned long v32;  // r13
    unsigned long long v33;  // rbp
    long long v16;  // rdi
    unsigned int v35;  // eax
    unsigned long v36;  // rsi
    unsigned long n;  // r9
    unsigned long long v38;  // r15
    unsigned long v39;  // rbp
    unsigned long n1;  // r13
    unsigned long v41;  // rbx
    unsigned long ptr;  // rbp
    unsigned long long v43;  // rbx
    long long v17;  // rsi
    unsigned long len;  // rax
    unsigned long n2;  // r14
    unsigned long long v46;  // rbx
    char *v47;  // rsi
    unsigned long len1;  // rax
    unsigned long v49;  // r14
    unsigned long len2;  // rax
    unsigned long v51;  // r14
    unsigned long v52;  // rax
    unsigned long v53;  // r14
    long long v18;  // rdx
    long long v19;  // rcx
    long long v20;  // r8
    long long v21;  // r9
    long long v22;  // rax
    unsigned long v23;  // 4114
    char v0;  // [bp-0xa9]
    unsigned long ptr2;  // [bp-0xa8]
    unsigned long v2;  // [bp-0xa0]
    unsigned long v3;  // [bp-0x98]
    unsigned long v4;  // [bp-0x90]
    unsigned long v5;  // [bp-0x88]
    unsigned long v6;  // [bp-0x80]
    unsigned long ptr1;  // [bp-0x78]
    unsigned long v8;  // [bp-0x70]
    char *v9;  // [bp-0x68]
    unsigned int v10;  // [bp-0x60]
    unsigned long v11;  // [bp-0x50]
    unsigned long v12;  // [bp-0x48]

    v14 = a2;
    ptr1 = a0;
    v8 = a1;
    idx = sub_41d390(1, 72);
    v22 = sub_413f00(v16, v17, v18, v19, v20, v21);
    v23 = g_42a687;
    *((long long *)&idx->padding_0[8]) = v22;
    if ((char)v23)
        v14 |= &g_400000;
    sub_4142e0(v22, &g_42a180, v14, (!g_42a684) * 2 | (unsigned int)v14 & 16 | 44);
    v24 = ptr1;
    v10 = g_42a180 ^ 1 | g_42a182;
    v5 = v8 + v24;
    v25 = sub_41d0f0(64);
    *((unsigned long long *)&idx[1].padding_0[0]) = 0;
    v11 = 1;
    idx->field_10 = v25 + 64;
    v12 = 0;
    ptr2 = rawmemchr(v24, 10);
    v26 = ptr2 - v24;
    v3 = v24;
    v27 = 0;
    v9 = &v11;
    v28 = sub_4062e0(v24, v26, v10) & 0xffffffff;
    v29 = v26;
    v0 = 0;
    v30 = 0;
    v31 = 1;
    v32 = v24;
    v2 = 0;
    v33 = v14;
    v4 = 0;
    while (1)
    {
        if (v27 - v31 + 2 > 0)
            idx->field_10 = sub_41d2b0(idx->field_10 - 64, v9) + 64;
        re_set_syntax(v33);
        v32 = ptr2 + 1;
        v30 += 1;
        v0 = (!(char)sub_406460(idx, v32, v29, *((long long *)&idx[1].padding_0[0]), v30, v33, (char)v28 ^ 1) ? 1 : v0);
        if ((char)v28)
        {
            *((unsigned long long *)&idx[1].padding_0[0]) = *((long long *)&idx[1].padding_0[0]) + 1;
            if (v5 >= v32)
            {
                ptr2 = rawmemchr(v32, 10);
                v29 = ptr2 - v32;
                v3 = v32;
                v28 = sub_4062e0(v32, v29, v10) & 0xffffffff;
            }
            else
            {
                v38 = v33;
                v39 = v32;
                break;
            }
        }
        else if (v5 >= v32)
        {
            ptr2 = rawmemchr(v32, 10);
            v29 = ptr2 - v32;
            v35 = (unsigned int)sub_4062e0(v32, v29, v10);
            v36 = v3;
            v28 = _INSERT(v28, 0, v32 > v36 & (char)v35);
            if (v32 > v36 & (char)v35)
            {
                n = v32 - v36;
                if (v2 - v12 + n > 0)
                {
                    v6 = n;
                    n = v6;
                    v4 = sub_41d2b0(v4, &v12);
                }
                v6 = n;
                memcpy(v4 + v2, v3, n);
                v2 += v6;
            }
            else
            {
                v28 = v35;
            }
        }
        else
        {
            v38 = v33;
            v39 = v3;
            break;
        }
        v27 = *((long long *)&idx[1].padding_0[0]);
        v31 = v11;
    }
    if (v0)
        exit(2); /* do not return */
    if (v5 >= v39)
    {
        if (ptr1 >= v39)
        {
            v2 = v8;
            v4 = ptr1;
        }
        else
        {
            n1 = v5 - v39;
            v41 = v2 + n1;
            v4 = sub_41d180(v4, v41);
            memcpy(v2 + v4, v39, n1);
            v2 = v41;
        }
    }
    if (g_42a686 || (ptr = 0, g_42a685))
    {
        v43 = (unsigned int)v38 & 0x2000;
        ptr = sub_41d110(v8 + 45);
        if (g_42a685)
        {
            if (v43)
            {
                *((unsigned short *)ptr) = 10334;
                *((char *)(ptr + 2)) = 0;
                len = strlen(ptr);
                n2 = v8;
                v46 = len + n2;
                memcpy(ptr + len, ptr1, n2);
                v47 = ")$";
            }
            else
            {
                *((unsigned int *)ptr) = 2645086;
                len1 = strlen(ptr);
                v49 = v8;
                v46 = len1 + v49;
                memcpy(ptr + len1, ptr1, v49);
                v47 = "\\)$";
            }
        }
        else
        {
            if (!v43)
            {
                *((uint128_t *)ptr) = (uint128_t)123921576912308382598422666524479072348;
                *((unsigned int *)(ptr + 16)) = 693919071;
                *((unsigned short *)(ptr + 20)) = 10332;
                *((char *)(ptr + 22)) = 0;
                len2 = strlen(ptr);
                v51 = v8;
                v46 = len2 + v51;
                memcpy(ptr + len2, ptr1, v51);
                v47 = "\\)\\([^[:alnum:]_]\\|$\\)";
            }
            else
            {
                *((uint128_t *)ptr) = (uint128_t)124113362702760382088434473634461670952;
                *((unsigned short *)(ptr + 16)) = 10281;
                *((char *)(ptr + 18)) = 0;
                v52 = strlen(ptr);
                v53 = v8;
                v46 = v52 + v53;
                memcpy(ptr + v52, ptr1, v53);
                v47 = ")([^[:alnum:]_]|$)";
            }
        }
        ptr1 = ptr;
        v8 = strlen(strcpy(ptr + v46, v47)) + v46;
    }
    sub_413340(ptr1, v8, *((long long *)&idx->padding_0[8]));
    sub_406350(idx);
    sub_414250(0, 0, *((long long *)&idx->padding_0[8]), 1);
    if (!v4)
    {
        free(ptr);
        return idx;
    }
    if (a3 || !(char)sub_413480(*((long long *)&idx->padding_0[8])))
    {
        idx->field_10 = idx->field_10 - 64;
        *((unsigned long long *)&idx[1].padding_0[0]) = *((long long *)&idx[1].padding_0[0]) + 1;
        if (!(char)sub_406460(idx, v4, v2, 0, -0x1, v38, 0))
            abort(); /* do not return */
    }
    if (ptr1 != v4)
        free(v4);
    free(ptr);
    return idx;
}



// Function: EGexecute @ 0x6c90
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

extern char g_42a180;
extern char g_42a182;
extern char g_42a684;
extern char g_42a685;
extern char g_42a686;

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
    v14 = g_42a684;
    v12 = sub_413460(a0->field_8);
    v17 = a1;
    v15 = sub_413470(a0->field_8);
    v10 = v22;
    if (a1 >= v22)
        return -0x1;
    v3 = a1;
    while (i)
    {
        v33 = v3;
        v9 = i;
        v2 = v10;
LABEL_406f59:
        v11 = v2 - v33;
        if (v11 > 0x80000000)
        {
            sub_41d4b0(); /* do not return */
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
                idx1->field_38 = (g_42a684 == 10) * 128 | idx1->field_38 & 63;
                v49 = re_search(idx1, v44, v46, v13, v1, v43);
                if (v49 < 0xffffffff)
                    sub_41d4b0(); /* do not return */
                if (v49 != 0xffffffff && (v50 = (long long)(int)v49 + v44, v0 >= v50))
                {
                    v51 = idx->field_30->field_0 - v49;
                    if (i)
                    {
                        if (!g_42a686)
                            goto LABEL_407196;
                        if (g_42a685)
                        {
                            if (v8 - 1 != v51)
                                goto LABEL_4070a8;
                            v51 = v8;
                            v50 = v9;
                            if (v0 <= v50)
                            {
                                if (v0 != v50)
                                    goto LABEL_4070a8;
                            }
                            else
                            {
LABEL_4071e8:
                                v4 = v51;
                                v0 = v50;
                                goto LABEL_4070a8;
                            }
LABEL_4071a2:
                            if (v51 <= v4)
                                v51 = v4;
                            v4 = v51;
                            goto LABEL_4070a8;
                        }
                    }
                    else if (g_42a685)
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
                    else if (!g_42a686)
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
                        if (!sub_40cc90(v52 + v54, v5) && !sub_40cca0(v53, v52, v5))
                        {
                            v51 = v54;
                            v50 = v52;
                            iter = v6;
                            if (!i)
                            {
                                *(a3) = v11;
                                return v53 - a1;
                            }
LABEL_407196:
                            if (v0 > v50)
                                goto LABEL_4071e8;
                            if (v0 != v50)
                                goto LABEL_4070a8;
                            goto LABEL_4071a2;
                        }
                        if (v54 > 0)
                        {
                            index = v7 + idx->field_10;
                            index->field_38 = index->field_38 | 64;
                            v56 = re_match(index, v53, (unsigned int)(v52 + v54 - 1) - (unsigned int)v9, (v52 & 0xffffffff) - v53, v43);
                            if (v56 < 0xffffffff)
                                sub_41d4b0(); /* do not return */
                            v54 = v56;
                            if (v56 > 0)
                                goto LABEL_407090;
                        }
                        if (v5 == v52)
                            goto LABEL_40709a;
                        idx2 = v7 + idx->field_10;
                        idx2->field_38 = idx2->field_38 & 191;
                        v58 = re_search(idx2, v53, v46, v52 + 1 - v53, v2 - (v52 + 1) - 1 & 0xffffffff, v43);
                        if (v58 < 0)
                            break;
                        v54 = idx->field_30->field_0 - v58;
                        v52 = v53 + v58;
LABEL_407090:
                        if (v0 < v52)
                        {
LABEL_40709a:
                            iter = v6;
                            goto LABEL_4070a8;
                        }
                    }
                    iter = v6;
                    if (v58 != 0xffffffff)
                        sub_41d4b0(); /* do not return */
                }
LABEL_4070a8:
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
LABEL_407162:
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
        v27 = sub_40c870(idx->field_0, v24 - v25, v25 + v26, &v19, 1);
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
                if (g_42a180 != 1)
                {
                    v11 = v2 - v33;
                    *(a3) = v11;
                    return v33 - a1;
                }
                else if (!g_42a182)
                {
                    if (v17 < v33)
                        v17 = v33;
                    if (!sub_40caf0(&v17, 0, v29, v10))
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
LABEL_406eb5:
                v39 = sub_413450(idx->field_8);
                if (!v39 || v2 == v39)
                    goto LABEL_407162;
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
                goto LABEL_406f59;
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
        v37 = sub_413450(v12, v3, v23, 0, &v18, 0);
        if (v37 && v23 != v37)
        {
            v38 = v14;
            if (v18)
                v3 = memrchr(a1, v38, v37 - a1) + 1;
            v18 = 0;
            v2 = rawmemchr(v37, v38) + 1;
            v33 = v3;
            goto LABEL_406eb5;
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
        goto LABEL_406eb5;
    }
}



// Function: hash_pattern @ 0x73d0
extern unsigned long long g_42a8d8;

long long hash_pattern(unsigned long a0, unsigned long long a1)
{
    void* v1;  // rcx
    unsigned long long v2;  // rax
    unsigned long v3;  // rdx

    v1 = g_42a8d8 + a0 - 1;
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



// Function: compare_patterns @ 0x7420
extern unsigned long long g_42a8d8;

unsigned int compare_patterns(unsigned long a0, unsigned long a1)
{
    void* v1;  // rcx
    unsigned long long v2;  // rsi
    char v3;  // dl
    unsigned long long v4;  // rax
    unsigned long long v5;  // rax

    v1 = g_42a8d8 + a0 - 1;
    v2 = g_42a8d8 + a1 - 1;
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



// Function: color_cap_mt_fct @ 0x7470
extern unsigned long long g_42a060;
extern unsigned long long g_42a068;

void color_cap_mt_fct(void)
{
    g_42a060 = g_42a068;
    return;
}



// Function: color_cap_rv_fct @ 0x7490
extern unsigned int g_42a900;

void color_cap_rv_fct(void)
{
    g_42a900 = 0xffffffff;
    return;
}



// Function: color_cap_ne_fct @ 0x74a0
extern char g_41f09e;
extern char g_41f0a4;
extern unsigned long long g_42a020;
extern unsigned long long g_42a028;

void color_cap_ne_fct(void)
{
    g_42a028 = &g_41f09e;
    g_42a020 = &g_41f0a4;
    return;
}



// Function: is_device_mode @ 0x74d0
char is_device_mode(unsigned int a0)
{
    unsigned int v1;  // edx

    v1 = a0 & 0xf000;
    return (a0 & 0xb000) == 0x2000 | v1 == 0xc000 | v1 == 0x1000;
}



// Function: skip_devices @ 0x7500
extern unsigned int g_42a884;

unsigned int skip_devices(unsigned int a0)
{
    unsigned int v1;  // eax

    v1 = 1;
    if (g_42a884 != 2)
        v1 = !g_42a884 & (a0 ^ 1);
    return v1;
}



// Function: initialize_unibyte_mask @ 0x7520
extern char g_42a183;
extern unsigned long long g_42a868;

unsigned long long initialize_unibyte_mask(void)
{
    unsigned long long iter;  // rdx
    unsigned int v2;  // eax
    unsigned int v3;  // esi
    unsigned int i;  // eax

    iter = 1;
    v2 = 1;
    v3 = 0;
    while (1)
    {
        if ((&g_42a183)[iter] == 1 || !(i = v2, !((unsigned int)iter & v3 & 0xff)))
        {
            iter += 1;
            if (iter == 0x100)
                break;
        }
        else
        {
            do
            {
                v2 = i;
                i *= 2;
            } while (i <= (unsigned int)iter);
            iter += 1;
            v3 |= v2;
            if (iter == 0x100)
                break;
        }
    }
    g_42a868 = 0x101010101010101 * (char)v3;
    return 0x101010101010101;
}



// Function: skip_easy_bytes @ 0x7590
extern unsigned long long g_42a868;

void* skip_easy_bytes(void* a0)
{
    void* iter;  // rbx
    char v2;  // al
    char v3;  // al
    char v4;  // al
    void* v5;  // rbx
    void* v6;  // rbx
    char v7;  // al

    iter = a0;
    if ((char)a0 & 7)
    {
        do
        {
            v2 = sub_40c9d0(*((char *)iter));
            if (g_42a868 & v2)
                return iter;
        } while ((iter += 1, *((char *)(void*)&iter) & 7));
    }
    if (g_42a868 & *((long long *)iter))
    {
        v3 = sub_40c9d0(*((char *)iter));
        if (g_42a868 & v3)
            return iter;
    }
    else
    {
        do
        {
            iter += 8;
        } while (!(g_42a868 & *((long long *)iter)));
        v4 = sub_40c9d0(*((char *)iter));
        if (g_42a868 & v4)
            return iter;
    }
    do
    {
        v5 = iter;
        v6 = v5 + 1;
        v7 = sub_40c9d0((char)v5[1]);
        iter = v6;
    } while (!(g_42a868 & v7));
    return v5 + 1;
}



// Function: buf_has_encoding_errors @ 0x7630
extern unsigned long long g_42a868;

unsigned int buf_has_encoding_errors(void* a0, unsigned long a1)
{
    char *v2;  // rbp
    char *v3;  // rbx
    long long v4;  // rax
    unsigned long v0;  // [bp-0x28]

    if (g_42a868)
    {
        v2 = a0 + a1;
        v0 = 0;
        *(v2) = 0xff;
        while (1)
        {
            v3 = sub_407590(a0);
            if (v2 <= v3)
                break;
            v4 = sub_40c9e0(v3, v2 - v3, &v0);
            if (v4 < 0)
                return 1;
            a0 = &v3[v4];
        }
    }
    return 0;
}



// Function: exclude_options @ 0x76d0
long long exclude_options(char a0)
{
    return (-(a0 < 1) & 0x40000000) + 0x10000000;
}



// Function: all_zeros @ 0x76f0
unsigned int all_zeros(char *a0, unsigned long a1)
{
    char *v1;  // rsi

    v1 = &a0[a1];
    if (a0 >= v1)
        return 1;
    while (!*(a0))
    {
        a0 += 1;
        if (a0 == v1)
            return 1;
    }
    return 0;
}



// Function: open_symlink_nofollow_error @ 0x7720
char open_symlink_nofollow_error(unsigned int a0)
{
    return a0 == 40 | a0 == 31;
}



// Function: contains_encoding_error @ 0x7730
unsigned int contains_encoding_error(unsigned long a0, long long a1)
{
    long long v2;  // rbx
    long long v3;  // rax
    unsigned long v0;  // [bp-0x38]

    v0 = 0;
    if (a1 > 0)
    {
        v2 = 0;
        do
        {
            if (sub_40c9f0(a0 + v2, a1 - v2, &v0) < 0)
                return 1;
        } while ((v2 += v3, a1 > v2));
    }
    return 0;
}



// Function: setmatcher @ 0x77b0
int setmatcher(char *a0, unsigned int a1)
{
    unsigned long long v1;  // rbx

    v1 = 0;
    while (strcmp(a0, 32 * v1 + 4364160))
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



// Function: add_count @ 0x7860
long long add_count(unsigned long a0, unsigned long a1)
{
    unsigned long long v0;  // [bp-0x8]

    if (!(__OFADD__(a0, a1) & 1))
        return a0 + a1;
    v0 = a0 + a1;
    error(2, 0, dcgettext(NULL, "input is too large to count", 5));
}



// Function: buf_has_nulls @ 0x7890
char buf_has_nulls(char *a0, unsigned long a1)
{
    a0[a1] = 0;
    return a1 != strlen(a0);
}



// Function: zap_nuls @ 0x78b0
long long zap_nuls(void* a0, void* a1, char a2)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    void* iter;  // rbx
    unsigned long len;  // rax
    unsigned long v7;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!a2)
        return v7;
    v1 = v3;
    v0 = v4;
    iter = a0;
    while (1)
    {
        *((char *)a1) = 0;
        len = strlen(iter);
        *((char *)a1) = a2;
        iter += len;
        if (a1 == iter)
            break;
        do
        {
            iter += 1;
            *((char *)iter - 1) = a2;
        } while (!*((char *)iter));
    }
    return len;
}



// Function: context_length_arg @ 0x7900
void context_length_arg(unsigned long a0, unsigned long long *a1)
{
    if (sub_41d560() <= 1 && !(*(a1) - 0 >> 63 & 1))
        return;
    dcgettext(NULL, "invalid context length argument", 5);
    error(2, 0, "%s: %s");
}



// Function: parse_grep_colors @ 0x7970
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
    void* field_10;
    char *field_18;
} struct_0;

typedef struct struct_1 {
    void* field_0;
} struct_1;

extern struct_0 g_429600;

void* parse_grep_colors(void)
{
    void* v1;  // rax
    void* v2;  // r14
    char *v3;  // r15
    void* v4;  // r13
    void* iter;  // r12
    char v6;  // bpl
    void* v7;  // rax
    struct_0 *node;  // rbx
    char *v9;  // rdi
    unsigned long v10;  // rax

    v1 = getenv("GREP_COLORS");
    if (!v1)
        return v1;
    if (!*((char *)v1))
        return v1;
    v2 = NULL;
    v3 = "mt";
    v4 = sub_41d490(v1);
    iter = v4 + 1;
    while (1)
    {
        v6 = *((char *)iter - 1);
        if (!(v6 != 58 && v6))
        {
            *((char *)iter - 1) = 0;
            node = &g_429600.field_0;
            v9 = v3;
            do
            {
            } while (strcmp(v9, v4) && (v9 = node->field_18, node += 24, v9));
            if (v2 && node->field_8)
                node->field_8->field_0 = v2;
            v10 = node->field_10;
            if (v10)
                v10 = v10();
            if (!v6)
                return v10;
            v4 = iter;
            v2 = NULL;
        }
        else if (v6 == 61)
        {
            if (v4 == iter - 1)
                return iter - 1;
            if (v2)
                return iter - 1;
            *((char *)iter - 1) = 0;
            v2 = iter;
        }
        else if (v2 && v6 != 59)
        {
            v7 = sub_40da90(v6);
            if (!*((char *)&v7))
                return v7;
        }
        iter += 1;
    }
}



// Function: clean_up_stdout @ 0x7a80
extern unsigned int g_42a8c0;

long long clean_up_stdout(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    if (!g_42a8c0)
    {
        v1 = v3;
        v0 = v4;
        return sub_40de77();
    }
    return g_42a8c0;
}



// Function: get_nondigit_option @ 0x7aa0
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern option g_428fa0;
extern unsigned int g_42a010;
extern unsigned int optind;

int get_nondigit_option(int a0, char **a1, unsigned long long *a2)
{
    unsigned int v7;  // ebx
    unsigned int v8;  // eax
    char v9;  // al
    void* v10;  // rdx
    char v0;  // [bp-0x81]
    void* v1;  // [bp-0x80]
    void* v2;  // [bp-0x78]
    char v3;  // [bp-0x68]
    char v4;  // [bp-0x53]
    char v5;  // [bp-0x50]

    v7 = optind;
    v0 = 0;
    v2 = &v3;
    v1 = &v3;
    while (1)
    {
        v8 = getopt_long(a0, a1, "0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz", &g_428fa0.name, NULL);
        v9 = sub_40da90(v8);
        if (!v9)
            break;
        if (g_42a010 != v7 || !v0)
        {
            v10 = v2;
        }
        else
        {
            v10 = v1 - (v3 == 48);
            if (v10 == &v4)
            {
                *((unsigned int *)v10) = 0x2e2e2e;
                v1 = &v5;
                *((char *)v1) = 0;
                sub_407900(v2, a2);
                return v8;
            }
        }
        *((char *)v10) = v8;
        v1 = v10 + 1;
        v0 = v9;
        g_42a010 = v7;
        v7 = optind;
    }
    if (v1 != v2)
    {
        *((char *)v1) = 0;
        sub_407900(v2, a2);
    }
    return v8;
}



// Function: update_patterns @ 0x7bd0
extern long long g_42a8c8;
extern unsigned long long g_42a8d0;
extern unsigned long long g_42a8e0;
extern long long g_42a8e8;
extern unsigned long long g_42a8f0;

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
        v5 = sub_418750(g_42a8c8, v9 + 1, 0);
        if (v5)
        {
            if (v5 >= 0)
            {
                iter += n;
                if (v0)
                {
                    v10 = g_42a8d0;
                }
                else
                {
                    v11 = g_42a8e0;
                    v12 = g_42a8f0;
                    if (g_42a8e0 == g_42a8e8)
                    {
                        g_42a8f0 = sub_41d2b0(v12, &g_42a8e8, 1, -0x1, 24);
                        v12 = g_42a8f0;
                        v11 = g_42a8e0;
                    }
                    g_42a8e0 = v11 + 1;
                    idx = v12 + v11 * 24;
                    v10 = g_42a8d0;
                    idx[1] = a3;
                    *(idx) = g_42a8d0;
                    idx[2] = v6;
                }
                v9 = iter - a0;
                g_42a8d0 = v10 + 1;
            }
            else
            {
                sub_41d4b0(); /* do not return */
            }
        }
        v6 += 1;
        if (v3 >= v1)
            return v9;
    }
}



// Function: setup_ok_fold @ 0x7d30
typedef struct mbstate_t {
} mbstate_t;

extern unsigned int g_42a284;
extern unsigned int g_42a684;
extern char g_42a6a0;

void setup_ok_fold(void)
{
    char *iter;  // r14
    unsigned int *v7;  // r13
    unsigned long v8;  // rdi
    unsigned int v9;  // eax
    unsigned long long v10;  // rdx
    void* v11;  // r15
    char v12;  // al
    char *v0;  // [bp-0xf0]
    mbstate_t v1;  // [bp-0xe0]
    mbstate_t v2;  // [bp-0xe0]
    char v3;  // [bp-0xd8]
    char v4[24];  // [bp-0x58]

    iter = &g_42a6a0;
    v7 = &g_42a284;
    v0 = &v3;
    do
    {
        v8 = *(v7);
        if ((unsigned int)v8 == 0xffffffff)
            continue;
        v9 = sub_4146b0(v8, v0);
        v2 = v1;
        if (v9 > 0)
        {
            v10 = v9 * 4;
            v11 = &v0[v10];
            do
            {
                v2 = (mbstate_t)0;
                if (wcrtomb(v4, *((int *)((char *)v11 - 4)), &v2) != 1)
                {
                    v12 = 0xff;
                    v1 = v2;
                    goto LABEL_407dfc;
                }
            } while ((v11 -= 4, v11 != &v2 - 4 + v10 - (unsigned long long)(v9 - 1) * 4));
        }
        v1 = v2;
        v12 = 1;
LABEL_407dfc:
        *(iter) = v12;
    } while ((v7 += 4, iter += 1, v7 != &g_42a684));
    return;
}



// Function: fgrep_icase_charlen @ 0x7e40
extern char g_42a180;
extern char g_42a6a0;

long long fgrep_icase_charlen(char *a0, long long a1, long long a2)
{
    unsigned long v3;  // rax
    long long v4;  // r13
    long long v5;  // rbp
    unsigned long long *v6;  // rax
    unsigned long v7;  // rbx
    char v0;  // [bp-0xbc]
    char v1;  // [bp-0xb8]

    v3 = *(a0);
    if (*((int *)&(&g_42a180)[260 + 4 * v3]) != 0xffffffff)
        return (&g_42a6a0)[v3];
    v4 = sub_418c40(&v0, a0, a1, a2);
    if (v4 > 16 || sub_4146b0(v0, &v1))
        return -0x1;
    v5 = v4;
    do
    {
        v5 -= 1;
        if (v5 <= 0)
            return v4;
    } while ((v6 = (unsigned long long *)(unsigned long long)__ctype_toupper_loc(), v7 = (unsigned long)(unsigned long long)a0[v5], (unsigned int)(char)v7 == *((int *)(*((long long *)(unsigned long long)__ctype_toupper_loc()) + v7 * 4))));
    return -0x1;
}



// Function: fgrep_icase_available @ 0x7f20
unsigned int fgrep_icase_available(unsigned long a0, long long a1)
{
    long long v2;  // r13
    unsigned int v3;  // eax
    unsigned long v0;  // [bp-0x38]

    v0 = 0;
    if (a1 > 0)
    {
        v2 = 0;
        do
        {
            if ((int)sub_407e40(a0 + v2, a1 - v2, &v0) < 0)
                return 0;
        } while ((v2 += (long long)(int)v3, a1 > v2));
    }
    return 1;
}



// Function: try_fgrep_pattern @ 0x7fb0

unsigned int try_fgrep_pattern(unsigned int a0, char *a1, void* a2)
{
    unsigned long v2;  // r13
    void* ptr;  // r14
    char *iter;  // r15
    void* v5;  // r14
    void* v6;  // r14
    char v7;  // cl
    unsigned long v0;  // [bp-0x48]

    v2 = *((long long *)a2);
    v0 = 0;
    ptr = sub_41d110(v2 + 1);
    iter = a1;
    v5 = ptr;
    v6 = ptr;
    v7 = *(iter);
}



// Function: skipped_file @ 0x81e0
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    unsigned int field_c;
    long long field_10;
    char padding_18[8];
    long long field_20;
} struct_0;

typedef struct struct_7 {
    char padding_0[8];
    unsigned int field_8;
} struct_7;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

typedef struct struct_1 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern unsigned int g_42a018;
extern char g_42a88a;
extern struct_1 g_42a8a0;
extern struct_1 g_42a8b0;

unsigned int skipped_file(unsigned long a0, char a1, unsigned int a2)
{
    struct_7 **v3;  // r8
    unsigned long long v4;  // r12
    unsigned long long v5;  // rbx
    struct_7 *v6;  // rbx
    unsigned long v7;  // rbp
    unsigned int v8;  // r12d
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!(char)a2)
    {
        v3 = (&g_42a8b0.field_0)[a1];
        if (!(&g_42a8b0.field_0)[a1])
            return 0;
    }
    else if (g_42a018 != 4)
    {
        if (a1 && g_42a88a)
            return 0;
        v3 = (&g_42a8a0.field_0)[a1];
        if (!(&g_42a8a0.field_0)[a1])
            return 0;
    }
    else
    {
        return a2;
    }
    v1 = v4;
    v0 = v5;
    v6 = *(v3);
    if (!*(v3))
        return 0;
    v7 = 0;
    while (1)
    {
        if (!v6->field_8)
        {
            if (!v7)
                v7 = sub_41d0f0(strlen(a0) + 1);
            if ((char)sub_414b70(*((int *)&v6[1].padding_0[0]), *((long long *)&v6[1].padding_0[4]), a0, v7))
                goto LABEL_414f51;
        }
        else if ((char)sub_414e90(*((long long *)&v6[1].padding_0[4]), *((long long *)&v6[2].field_8), a0))
        {
LABEL_414f51:
            v8 = 0;
            break;
        }
        if (v6->padding_0)
        {
            v6 = (struct_7 *)v6->padding_0;
        }
        else
        {
            v8 = 1;
            break;
        }
    }
    free(v7);
    return (*((int *)&v6[1].padding_0[0]) >> 29 ^ 1) & 1 ^ v8;
}



// Function: nlscan @ 0x8250
extern char g_42a684;
extern void* g_42a7c0;
extern unsigned int g_42a818;

unsigned int nlscan(void* a0)
{
    void* v1;  // rdi
    unsigned long long v2;  // rbp
    void* ptr;  // rax
    long long v4;  // rax

    v1 = g_42a7c0;
    if (g_42a7c0 < a0)
    {
        v2 = 0;
        do
        {
            ptr = memchr(v1, g_42a684, a0 - v1);
        } while (ptr && (v1 = ptr + 1, v2 += 1, a0 > v1));
    }
    else
    {
        v2 = 0;
    }
    v4 = sub_407860(*((long long *)&g_42a818), v2);
    g_42a7c0 = a0;
    *((long long *)&g_42a818) = v4;
    return *((long long *)&g_42a818);
}



// Function: pr_sgr_start @ 0x82c0
extern char *g_42a028;

int pr_sgr_start(char *a0)
{
    int v1;  // eax

    if (*(a0))
        return __printf_chk(1, g_42a028);
    return v1;
}



// Function: pr_sgr_start_if @ 0x82e0
extern unsigned int g_42a900;

int pr_sgr_start_if(char *a0)
{
    if (g_42a900)
        return sub_4082c0(a0);
    return g_42a900;
}



// Function: fputs_errno @ 0x8300
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_42a8c0;

int fputs_errno(char *a0)
{
    unsigned int v1;  // eax

    v1 = fputs_unlocked(a0, stdout);
    if (v1 >= 0)
        return v1;
    g_42a8c0 = (unsigned int)__errno_location();
    return g_42a8c0;
}



// Function: printf_errno @ 0x8330
extern long long stdout;
extern unsigned int g_42a8c0;

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
        g_42a8c0 = *(__errno_location());
    if (v4 != *((long long *)(40 + v18)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: putchar_errno @ 0x8410
extern unsigned int g_42a8c0;

int putchar_errno(int a0)
{
    unsigned int v1;  // eax

    v1 = putchar_unlocked(a0);
    if (v1 >= 0)
        return v1;
    g_42a8c0 = (unsigned int)__errno_location();
    return g_42a8c0;
}



// Function: fflush_errno @ 0x8430
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_42a8c0;

int fflush_errno(void)
{
    unsigned int v1;  // eax

    v1 = fflush_unlocked(stdout);
    if (!v1)
        return v1;
    g_42a8c0 = (unsigned int)__errno_location();
    return g_42a8c0;
}



// Function: fillbuf @ 0x8560
typedef struct struct_1 {
    char padding_0[24];
    unsigned int field_18;
    char padding_1c[20];
    long long field_30;
} struct_1;

typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
} struct_2;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned int g_42a818;
extern char g_42a821;
extern unsigned long long g_42a830;
extern unsigned long long g_42a838;
extern struct_0 *g_42a840;
extern unsigned long long g_42a848;
extern unsigned int g_42a850;
extern unsigned long long g_42a858;
extern unsigned long long g_42a860;
extern char g_42a880;

unsigned long long fillbuf(unsigned long a0, struct_1 *a1)
{
    unsigned long ptr;  // r14
    unsigned long v3;  // rdx
    long long v13;  // r9
    long long v14;  // r11
    unsigned long v15;  // rax
    unsigned long long v16;  // rdx
    unsigned long long *v17;  // rcx
    unsigned long long v18;  // rdx
    void* ptr1;  // rsi
    unsigned int v20;  // r14d
    unsigned long long v21;  // r8
    unsigned long v4;  // r13
    unsigned long v22;  // rax
    unsigned long long v23;  // rax
    unsigned int v24;  // r8
    long long v25;  // rax
    unsigned long v26;  // 4112
    long long v27;  // r15
    long long v28;  // rax
    long long v29;  // rax
    unsigned long long *iter;  // rbx
    unsigned long v6;  // rcx
    unsigned long n;  // r15
    struct_2 *v8;  // rsi
    unsigned long long v9;  // rsi
    unsigned long v10;  // rcx
    long long v11;  // rax
    unsigned long long *v0;  // [bp-0x40], Other Possible Types: long long, unsigned long

    ptr = g_42a860;
    v3 = g_42a858;
    v4 = g_42a838;
    iter = g_42a840;
    v6 = g_42a838 + 8;
    if (ptr + g_42a858 - (char *)g_42a840 < v6)
    {
        n = a0 + 1;
        v0 = ~(a0);
        if (g_42a838 + a0 - g_42a858 + v6 > 0)
        {
            if ((char)sub_408550(a1->field_18))
            {
                v9 = v8->field_30 - g_42a830;
                if (!(v8->field_30 - g_42a830 >> 63 & 1))
                    v11 = v10 + a0;
            }
            v15 = sub_41d2b0(0, &g_42a858, v13, v14, 1);
            v4 = g_42a838;
            iter = v15 + n;
            ptr = v15;
            v16 = iter % g_42a838;
            if (!v16)
            {
                memmove(ptr, &g_42a840[v0], n);
            }
            else
            {
                iter = g_42a838 + a0 + 1 - v16 + ptr;
                memmove(v0 + (char *)iter, &g_42a840[v0], n);
            }
            free(g_42a860);
            g_42a860 = ptr;
            v3 = g_42a858;
        }
        else
        {
            v17 = ptr + n;
            v18 = v17 % g_42a838;
            ptr1 = &g_42a840[v0];
            if (!v18)
            {
                v0 = v17;
                memmove(ptr, ptr1, n);
                v3 = g_42a858;
                iter = v0;
            }
            else
            {
                iter = g_42a838 + a0 + 1 - v18 + ptr;
                memmove(v0 + (char *)iter, ptr1, n);
                v3 = g_42a858;
            }
        }
    }
    g_42a848 = (char *)iter - a0;
    while (1)
    {
        v20 = g_42a850;
        while (1)
        {
            v21 = sub_41baf0(v20, iter, ptr + v3 - (char *)iter - (ptr + v3 - (char *)iter - 8) % v4 - 8);
            if (v21 == 0xffffffffffffffff)
            {
                *(iter) = 0;
                g_42a840 = iter;
                return 0;
            }
            g_42a830 = g_42a830 + v21;
            v22 = g_42a821 ^ 1;
            v23 = _INSERT(v22, 0, (char)v22 | !v21);
            if ((char)v22 || !v21)
            {
                iter = (char *)iter + v21;
                *(iter) = 0;
                g_42a840 = iter;
                return v23;
            }
            if (!(char)sub_4076f0(iter, v21))
            {
                iter = (char *)iter + v24;
                *(iter) = 0;
                g_42a840 = iter;
                return 1;
            }
            v25 = sub_407860(*((long long *)&g_42a818), v24);
            v26 = g_42a880;
            *((long long *)&g_42a818) = v25;
            if ((char)v26)
                break;
            v27 = g_42a830;
            v20 = g_42a850;
            v28 = lseek(g_42a850, v27, 3);
            if (v28 >= 0 || *(__errno_location()) == 6 && (char)sub_408550(a1->field_18) && v27 < a1->field_30 && !(v28 = (long long)lseek(g_42a850, 0, 2), v28 < 0))
            {
                v0 = v28;
                v29 = sub_407860(*((long long *)&g_42a818), v28 - v27);
                v20 = g_42a850;
                *((long long *)&g_42a818) = v29;
                g_42a830 = v0;
            }
            else
            {
                g_42a880 = 1;
            }
        }
    }
}



// Function: print_line_tail @ 0x88b0
extern char g_42a684;

void* print_line_tail(void* a0, void* a1, char *a2)
{
    void* iter;  // rbx
    unsigned long long v2;  // rax
    long long v3;  // r13

    iter = a1;
    if (a1 > a0)
    {
        v2 = 0;
        if (*((char *)a1 - 1) != g_42a684)
        {
            if (*((char *)iter + ~(0)) == 13)
                goto LABEL_408920;
LABEL_4088e5:
            iter = a1;
        }
        else
        {
            a1 -= 1;
            if (a0 >= a1 || (v2 = 1, *((char *)((char *)iter + ~(1))) != 13))
                goto LABEL_4088e5;
LABEL_408920:
            iter -= v2 + 1;
        }
    }
    v3 = iter - a0;
    if (v3 <= 0)
        return a0;
    sub_4082c0(a2);
    sub_408460(a0, v3);
    sub_408890(*(a2));
    return iter;
}



// Function: pr_sgr_end_if @ 0x8950
extern unsigned int g_42a900;

long long pr_sgr_end_if(char *a0)
{
    if (g_42a900)
        return sub_408890(*(a0));
    return g_42a900;
}



// Function: print_offset @ 0x8970
long long print_offset(unsigned long a0, char *a1)
{
    sub_4082e0(a1);
    sub_408330("%*ld");
    return sub_408950(a1);
}



// Function: print_sep @ 0x89b0
extern char *g_42a040;

long long print_sep(char a0)
{
    sub_4082e0(g_42a040);
    sub_408410(a0);
    return sub_408950(g_42a040);
}



// Function: pattern_file_name @ 0x89e0
typedef struct struct_0 {
    char field_0;
} struct_0;

extern long long g_42a8e0;
extern struct_0 *g_42a8f0;

long long pattern_file_name(unsigned long i, unsigned long long *a1)
{
    unsigned long long *v1;  // rcx
    unsigned long long v2;  // rax
    long long v3;  // rdx
    void* v4;  // rcx

    v1 = g_42a8f0;
    if (g_42a8e0 > 1)
    {
        v2 = 24;
        v3 = 1;
        while (*((long long *)&g_42a8f0[v2]) <= i)
        {
            v3 += 1;
            v2 += 24;
            if (v3 == g_42a8e0)
            {
                v4 = (char *)&g_42a8f0[24 * v3] - 24;
                *(a1) = i - *((long long *)v4) + (long long)v4[16];
                return (long long)v4[8];
            }
        }
        v1 = (char *)&g_42a8f0[v2] - 24;
    }
    *(a1) = i - *(v1) + v1[2];
    return v1[1];
}



// Function: input_filename @ 0x8a50
extern unsigned long long g_42a890;

unsigned long long input_filename(void)
{
    if (!g_42a890)
        g_42a890 = dcgettext(NULL, "(standard input)", 5);
    return g_42a890;
}



// Function: print_filename @ 0x8a90
extern char *g_42a058;

long long print_filename(void)
{
    sub_4082e0(g_42a058);
    sub_408300(sub_408a50());
    return sub_408950(g_42a058);
}



// Function: print_line_head @ 0x8ac0
extern char *g_42a048;
extern char *g_42a050;
extern unsigned long long g_42a7c0;
extern unsigned int g_42a7c8;
extern char g_42a800;
extern char g_42a801;
extern unsigned int g_42a804;
extern unsigned int g_42a808;
extern unsigned int g_42a810;
extern unsigned long long g_42a818;
extern unsigned long long g_42a848;
extern char g_42a888;
extern char g_42a8fc;

unsigned long long print_line_head(void* a0, unsigned long long a1, unsigned long a2, char a3)
{
    char v2;  // al
    unsigned long v3;  // rdi
    long long v4;  // rax
    long long v5;  // rax
    unsigned int v6;  // edx
    char v0;  // [bp-0x39]

    if (g_42a810 != 1)
    {
        v2 = sub_407630(a0, a1);
        *((char *)a0 + a1) = *((char *)a0 + a1);
        if (v2)
        {
            g_42a888 = 1;
            return 0;
        }
    }
    if (g_42a808)
    {
        sub_408a90();
        if (g_42a804)
            sub_4089b0(a3);
        else
            sub_408410(0);
    }
    if (g_42a801)
    {
        v3 = g_42a818;
        if (g_42a7c0 < a2)
        {
            sub_408250(a0);
            v4 = sub_407860(g_42a818, 1);
            g_42a7c0 = a2;
            g_42a818 = v4;
            v3 = g_42a818;
        }
        sub_408970(v3, g_42a050);
        sub_4089b0(a3);
    }
    if (g_42a800)
    {
        v5 = sub_407860(*((long long *)&g_42a7c8), a0 - g_42a848);
        sub_408970(v5, g_42a048);
        sub_4089b0(a3);
    }
    if (!g_42a8fc)
        return 1;
    v6 = g_42a801 | g_42a800;
    if ((v6 || g_42a808) & a1)
    {
        v0 = (v6 || g_42a808) & a1;
        sub_408410(9);
        return v0;
    }
    return g_42a8fc;
}



// Function: print_line_middle @ 0x8c60
typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_42a684;
extern char g_42a802;
extern unsigned long long g_42a870;
extern struct_0 *g_42a878;
extern char g_42a8fd;

long long print_line_middle(unsigned long a0, unsigned long a1, char *a2, char *a3)
{
    unsigned long v3;  // rbp
    unsigned long v4;  // r13
    long long v5;  // rax
    unsigned long v6;  // r15
    unsigned long long v0;  // [bp-0x60]
    unsigned long v1;  // [bp-0x48]

    if (a0 < a1)
    {
        v3 = 0;
        v0 = a1 - a0;
        v4 = a0;
        while (1)
        {
            v5 = g_42a878(g_42a870, a0, v0, &v1, v4);
            if (v5 < 0 || !(v6 = a0 + v5, a1 != v6))
                break;
            if (!v1)
            {
                v1 = 1;
                if (!v3)
                    v3 = v4;
                v4 = v6 + 1;
                if (a1 <= v4)
                    break;
            }
            else
            {
                if (g_42a8fd)
                {
                    if (!(char)sub_408ac0(v6, v1, a1, ((char)-(g_42a802 < 1) & 13) + 45))
                        return 0;
                }
                else
                {
                    sub_4082c0(a2);
                    v3 = 0;
                    sub_408460();
                }
                sub_4082e0(a3);
                sub_408460(v6, v1);
                sub_408950(a3);
                if (g_42a8fd)
                    sub_408410(g_42a684);
                v4 = v6 + v1;
                if (a1 <= v4)
                    break;
            }
        }
        if (g_42a8fd)
            return a1;
        a0 = v4;
        if (v3)
            return v3;
    }
    else if (g_42a8fd)
    {
        return a1;
    }
    return a0;
}



// Function: prline @ 0x8e20
typedef struct struct_0 {
    char field_0;
} struct_0;

extern struct_0 *g_42a030;
extern struct_0 *g_42a038;
extern struct_0 *g_42a060;
extern struct_0 *g_42a068;
extern unsigned long long g_42a7b8;
extern char g_42a7d8;
extern char g_42a802;
extern int g_42a8c0;
extern char g_42a8fd;
extern unsigned int g_42a900;

long long prline(unsigned long a0, unsigned long a1, char a2)
{
    unsigned long v1;  // rbp
    unsigned long v2;  // rax
    char v3;  // r9b
    char v4;  // al
    char v5;  // dil
    char v6;  // cl
    char *v7;  // r8
    char *v8;  // r13
    char *v9;  // rax

    v1 = a0;
    v2 = g_42a8fd;
    v3 = g_42a8fd;
    if (!g_42a8fd)
    {
        v4 = sub_408ac0(a0, a1 - a0 - 1, a1, a2);
        v2 = g_42a8fd;
        v3 = g_42a8fd;
        if (!v4)
            return g_42a8fd;
    }
    v5 = a2 == 58;
    v6 = (v5 ^ g_42a802) & (char)v2;
    if (!g_42a900)
    {
        if (!v6)
            goto LABEL_408e6c;
        v7 = NULL;
        v8 = NULL;
    }
    else
    {
        v8 = &g_42a038->field_0;
        v7 = &g_42a068->field_0;
        if (v5 == ((char)(g_42a900 >> 31) & g_42a802))
            v8 = &g_42a030->field_0;
        if (a2 != 58)
            v7 = &g_42a060->field_0;
        if (!v6)
        {
            if (!*(v8))
            {
                if (!*(v7))
                    goto LABEL_408e6c;
                if (v5 == g_42a802)
                    goto LABEL_408f20;
            }
            else
            {
                if (v5 == g_42a802)
                    goto LABEL_408f20;
                if (!*(v7))
                {
LABEL_408f4b:
                    v2 = sub_4088b0(v1, a1, v8);
                    v3 = g_42a8fd;
                    v1 = v2;
LABEL_408e6c:
                    if (v1 < a1 && v3 != 1)
                    {
                        v2 = sub_408460(v1, a1 - v1);
                        goto LABEL_408e88;
                    }
                }
            }
        }
    }
    v1 = sub_408c60(v1, a1, v8, v7);
    if (!v1)
        return v1;
    v2 = g_42a8fd;
LABEL_408f20:
    if ((char)v2)
    {
LABEL_408e88:
        if (g_42a7d8)
            v2 = sub_408430();
        if (!g_42a8c0)
        {
            g_42a7b8 = a1;
            return v2;
        }
        v9 = dcgettext(NULL, "write error", 5);
        error(2, g_42a8c0, v9);
    }
    if (*(v8))
        goto LABEL_408f4b;
    v3 = 0;
    goto LABEL_408e6c;
}



// Function: prpending @ 0x8ff0
extern char g_42a684;
extern long long g_42a7a8;
extern void g_42a7b8;
extern long long g_42a848;

long long prpending(unsigned long i)
{
    long long v1;  // rax
    void* ptr;  // rax

    if (!*((long long *)&g_42a7b8))
    {
        v1 = g_42a848;
        *((long long *)&g_42a7b8) = g_42a848;
    }
    if (g_42a7a8 <= 0)
        return v1;
    while (*((long long *)&g_42a7b8) < i)
    {
        ptr = rawmemchr(*((long long *)&g_42a7b8), g_42a684);
        sub_408e20(*((long long *)&g_42a7b8), ptr + 1, 45);
        v1 = g_42a7a8 - 1;
        g_42a7a8 = v1;
        if (v1 <= 0)
            return v1;
    }
    return v1;
}



// Function: prtext @ 0x9070
extern char *g_42a040;
extern char *g_42a070;
extern char g_42a684;
extern char g_42a688;
extern long long g_42a7a8;
extern unsigned long long g_42a7b0;
extern char g_42a7b8;
extern char g_42a7f0;
extern char g_42a7f8;
extern char g_42a802;
extern char g_42a803;
extern unsigned long long g_42a828;
extern unsigned long long g_42a830;
extern unsigned long long g_42a840;
extern void* g_42a848;

long long prtext(void* a0, void* a1)
{
    void* i;  // rbx
    void* v2;  // rcx
    long long iter;  // rax
    void* v4;  // rbp
    void* v5;  // rbp
    char v6;  // r15b
    unsigned long node;  // r13
    long long v8;  // rax
    long long iter1;  // r14
    void* v10;  // rbx

    i = a0;
    if (!g_42a803 && (g_42a7a8 <= 0 || !(sub_408ff0(a0), g_42a803)))
    {
        v2 = *((long long *)&g_42a7b8);
        if (!*((long long *)&g_42a7b8))
            v2 = g_42a848;
        iter = 0;
        v4 = i;
        if (*((long long *)&g_42a7f8) > 0)
        {
            while (1)
            {
                if (v2 < v4)
                {
                    do
                    {
                        v4 -= 1;
                    } while (*((char *)v4 - 1) != g_42a684);
                    iter += 1;
                    if (iter == *((long long *)&g_42a7f8))
                        break;
                }
                else
                {
                    iter += 1;
                    if (iter == *((long long *)&g_42a7f8))
                        break;
                }
            }
            if (!g_42a688 || *((long long *)&g_42a7b8) == v4 || !g_42a070)
                goto LABEL_409135;
            goto LABEL_409107;
        }
        else if ((*((long long *)&g_42a7f8) & *((long long *)&g_42a7f0)) >= 0 && *((long long *)&g_42a7b8) != i && g_42a688 && g_42a070)
        {
            v4 = i;
LABEL_409107:
            sub_4082e0(g_42a040);
            sub_408300(g_42a070);
            sub_408950(g_42a040);
            sub_408410(10);
LABEL_409135:
            v5 = v4;
            if (i > v5)
            {
                do
                {
                    v5 = rawmemchr(v4, g_42a684) + 1;
                    sub_408e20(v4, v5, 45);
                    v4 = v5;
                } while (i > v4);
            }
        }
        else
        {
            v5 = i;
        }
        v6 = g_42a803;
        node = g_42a7b0;
        if (g_42a802)
            goto LABEL_4092c3;
        node = g_42a7b0 - 1;
        if (g_42a803)
            goto LABEL_409278;
        v5 = a1;
        sub_408e20(i, v5, 58);
        v6 = g_42a803;
        node = g_42a7b0 - 1;
        goto LABEL_4091bc;
    }
    else
    {
        v6 = g_42a802;
        node = g_42a7b0;
        if (!g_42a802)
        {
            node = g_42a7b0 - 1;
LABEL_409278:
            v8 = 0;
            g_42a828 = a1 - g_42a840 + g_42a830;
        }
        else
        {
            v5 = i;
LABEL_4092c3:
            iter1 = 0;
            if (a1 > v5)
            {
                while (1)
                {
                    if (iter1 >= node)
                    {
                        node -= iter1;
                        break;
                    }
                    v10 = rawmemchr(v5, g_42a684) + 1;
                    if (v6)
                    {
                        iter1 += 1;
                        if (a1 <= v10)
                            goto LABEL_409329;
                    }
                    else
                    {
                        iter1 += 1;
                        sub_408e20(v5, v10, 58);
                        v6 = g_42a803;
                        node = g_42a7b0;
                        if (a1 <= v10)
                        {
LABEL_409329:
                            node -= iter1;
                            v5 = v10;
                            break;
                        }
                    }
                    v5 = v10;
                }
            }
LABEL_4091bc:
            v8 = 0;
            g_42a828 = v5 - g_42a840 + g_42a830;
            if (!v6)
            {
                v8 = *((long long *)&g_42a7f0);
                if (*((long long *)&g_42a7f0) < 0)
                    v8 = 0;
            }
        }
    }
    g_42a7b0 = node;
    g_42a7a8 = v8;
    g_42a688 = 1;
    return v8;
}



// Function: grepbuf @ 0x9350
typedef struct struct_0 {
    char field_0;
} struct_0;

extern int g_42a080;
extern char g_42a7a1;
extern char g_42a7a2;
extern unsigned long long g_42a7b0;
extern char g_42a802;
extern unsigned long long g_42a870;
extern struct_0 *g_42a878;
extern char g_42a889;

unsigned long long grepbuf(void* a0, void* a1)
{
    void* i;  // rbx
    void* v3;  // r13
    long long v4;  // rax
    void* v5;  // rdi
    void* v6;  // rsi
    void* v7;  // rsi
    unsigned long long v8;  // rdx
    int v9;  // edi
    unsigned long long v0;  // [bp-0x38]

    if (a0 >= a1)
        return 0;
    i = a1;
    do
    {
        v3 = a0;
        v4 = g_42a878(g_42a870, v3, i - v3, &v0, 0);
        if (v4 >= 0)
        {
            v5 = v3 + v4;
            v6 = v0 + v5;
            if (!g_42a802)
            {
                if (i == v5)
                    goto LABEL_409460;
                a0 = v6;
                v3 = v5;
                v7 = v6;
            }
            else
            {
                if (v3 >= v5)
                {
                    a0 = v6;
                    continue;
                }
                else
                {
                    a0 = v6;
                    v7 = v5;
                }
            }
        }
        else if (g_42a802)
        {
            v0 = 0;
            if (v3 >= i)
                break;
            a0 = i;
            v7 = i;
        }
        else
        {
LABEL_409460:
            v8 = g_42a7b0;
LABEL_409429:
            return g_42a7b0 - v8;
        }
        sub_409070(v3, v7);
        v8 = g_42a7b0;
        if (!g_42a7b0 || g_42a7a2)
        {
            if (g_42a7a1)
            {
                v9 = 0;
                if (g_42a889)
                    v9 = g_42a080;
                exit(v9); /* do not return */
            }
            goto LABEL_409429;
        }
    } while (i > a0);
    return g_42a7b0 - g_42a7b0;
}



// Function: suppressible_error @ 0x94a0
extern char g_42a889;
extern char g_42a904;

void suppressible_error(int a0)
{
    if (g_42a904)
    {
        g_42a889 = 1;
    }
    else
    {
        sub_408a50();
        error(0, a0, "%s");
        g_42a889 = 1;
    }
    return;
}



// Function: file_must_have_nulls @ 0x94e0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    char padding_1c[20];
    long long field_30;
} struct_0;

extern char g_42a881;

unsigned int file_must_have_nulls(long long a0, unsigned long a1, struct_0 *a2)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    long long v6;  // rbp
    unsigned long long v7;  // rbx
    unsigned long v8;  // rdx
    int v9;  // esi
    long long v10;  // rax
    unsigned long v11;  // rax
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    if (g_42a881)
        return 0;
    v2 = v4;
    v1 = v5;
    v6 = a0;
    v0 = v7;
    if ((char)sub_408550(a2->field_18) && *((long long *)(v8 + 48)) > v6 && (v9 || (v6 = (long long)lseek(0, 0, 1), v6 >= 0)))
    {
        v10 = lseek(v9, v6, 4);
        if (v10 >= 0)
        {
            v11 = lseek(v9, v6, 0);
            if (v11 < 0)
                v11 = (unsigned long long)sub_4094a0(*(__errno_location()));
            return _INSERT(v11, 0, v10 < a2->field_30);
        }
    }
    return 0;
}



// Function: finalize_input @ 0x95a0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern unsigned long long g_42a7b0;
extern char g_42a828;
extern int g_42a830;
extern char g_42a881;

unsigned int * finalize_input(unsigned int a0, struct_0 *a1, char a2)
{
    unsigned int *v2;  // rax
    unsigned long long v3;  // rbx
    unsigned int *v4;  // rax
    unsigned int *err;  // rax
    unsigned int *v6;  // rax
    unsigned int *v7;  // rax
    unsigned int *v8;  // rax
    unsigned int *v9;  // rax
    unsigned int *v10;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (a0)
        return v2;
    v0 = v3;
    if (g_42a7b0)
    {
        if (!a2)
        {
            if (!g_42a881)
            {
                v4 = lseek(0, 0, 2);
                if (v4 >= 0)
                    return v4;
                err = __errno_location();
                if (*(err) == 22)
                    return err;
            }
            v6 = sub_408490(a1->field_18);
            if (*((char *)&v6))
                return v6;
        }
        else
        {
            return v7;
        }
    }
    else
    {
        if (*((long long *)&g_42a830) == *((long long *)&g_42a828))
        {
            return v8;
        }
        else if (g_42a881)
        {
            return v9;
        }
        else
        {
            v10 = lseek(0, *((long long *)&g_42a828), 0);
            if (v10 >= 0)
                return v10;
        }
    }
    return (unsigned long long)sub_4094a0(*(__errno_location()));
}



// Function: grep @ 0x9710
typedef struct struct_1 {
    char padding_0[24];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_42a684;
extern char g_42a7a2;
extern unsigned long long g_42a7a8;
extern unsigned long long g_42a7b0;
extern void* g_42a7b8;
extern unsigned long long g_42a7c0;
extern unsigned int g_42a7c8;
extern unsigned long long g_42a7e0;
extern char g_42a7e8;
extern long long g_42a7f8;
extern char g_42a800;
extern char g_42a801;
extern char g_42a803;
extern char g_42a810;
extern unsigned long long g_42a818;
extern char g_42a820;
extern char g_42a821;
extern unsigned long long g_42a828;
extern struct_0 *g_42a840;
extern char g_42a848;
extern char g_42a888;
extern unsigned int g_42a8f8;
extern char g_42a8fc;

long long grep(unsigned int a0, struct_1 *a1, char *a2)
{
    char v10;  // 4108
    unsigned long v11;  // 4110
    long long v20;  // rbp
    void* v21;  // 4098
    void* v22;  // rbx
    char v23;  // r14b
    void* v24;  // rax
    void* v25;  // rdi
    void* v26;  // rbp
    char *v27;  // rax
    unsigned long v28;  // rax
    long long i;  // rax
    unsigned int v12;  // edi
    long long v30;  // rax
    unsigned long v31;  // 4111
    int *err;  // rax
    char *v33;  // r14
    unsigned long long v34;  // 4099
    int *err1;  // rax
    uint128_t v13;  // rsi
    char v14;  // r8b
    char v15;  // dl
    unsigned long long v16;  // r14
    char *v17;  // r15
    void* v18;  // rdi
    unsigned int v19;  // ebx
    unsigned long long v0;  // [bp-0x70]
    unsigned long v1;  // [bp-0x68]
    char v2;  // [bp-0x59]
    unsigned long iter;  // [bp-0x58]
    void* v4;  // [bp-0x50]
    char v5;  // [bp-0x48]
    char v6;  // [bp-0x47]
    char v7;  // [bp-0x46]
    char v8;  // [bp-0x45]

    v7 = g_42a7a2;
    v8 = g_42a803;
    if (!(char)sub_409630())
    {
        iter = 0;
        return iter;
    }
    *((unsigned long *)&g_42a7c8) = 0;
    g_42a7b8 = 0;
    g_42a7b0 = g_42a7e0;
    v10 = g_42a820;
    g_42a818 = 0;
    g_42a828 = 0;
    g_42a7a8 = 0;
    g_42a821 = !g_42a684 & v10;
    g_42a888 = 0;
    if (!(char)sub_408560(0, a1))
    {
        err1 = __errno_location();
        sub_4094a0(*(err1));
        iter = 0;
        return iter;
    }
    v11 = g_42a8fc;
    g_42a8f8 = 0;
    if ((char)v11)
    {
        v12 = 0;
        do
        {
            v12 += 1;
        } while ((unsigned long long)((int128_t)(v13 * 7378697629483820647) >> 66) != (int128_t)(v13) >> 63);
        g_42a8f8 = v12;
    }
    v2 = 0;
    v15 = v14;
    v6 = g_42a684;
    v1 = 0xffffffffffffffff;
    v16 = 0;
    iter = 0;
    v0 = 0;
    do
    {
        v5 = v15;
        v17 = &g_42a840->field_0;
        v18 = *((long long *)&g_42a848);
        if (v1 - 0 >> 63 & 1 && v6 && (v19 = (unsigned int)*((int *)&g_42a810), *((int *)&g_42a810) != 1))
        {
            v4 = *((long long *)&g_42a848);
            v20 = &g_42a840[-1 * *((long long *)&g_42a848)];
            if (!sub_407890(*((long long *)&g_42a848), v20))
            {
                v18 = v4;
                if (!v5)
                    goto LABEL_409880;
                if ((char)sub_4094e0(v20, a0, a1))
                {
                    v19 = *((int *)&g_42a810);
                }
                else
                {
                    v18 = *((long long *)&g_42a848);
                    v17 = &g_42a840->field_0;
                    goto LABEL_409880;
                }
            }
            if (v19 == 2)
            {
                iter = 0;
                return iter;
            }
            if (!g_42a7e8)
            {
                g_42a803 = 1;
                g_42a7a2 = 1;
            }
            v2 = g_42a684;
            v18 = *((long long *)&g_42a848);
            v17 = &g_42a840->field_0;
            g_42a821 = g_42a820;
            v1 = iter;
            goto LABEL_409880;
        }
        else
        {
LABEL_409880:
            v21 = g_42a7b8;
            g_42a7c0 = v18;
            if (v21)
                g_42a7b8 = v18;
            v22 = v18 + v16;
            if (v22 == v17)
            {
                v33 = v17;
                *(a2) = 1;
                if (v0)
                {
                    v34 = g_42a7b0;
                    *(v33) = g_42a684;
                    g_42a840 = v33 + 1;
                    if (v34)
                        iter += sub_409350(v18 + v16 - v0, g_42a840);
                    if (g_42a7a8)
                    {
                        sub_408ff0(g_42a840);
                        goto LABEL_4099f0;
                    }
                }
            }
            sub_4078b0(v22, v17, v2);
            v23 = *((char *)v22 - 1);
            *((char *)v22 - 1) = g_42a684;
            v24 = memrchr(v22 - 1, g_42a684, v17 - v22 + 1);
            *((char *)v22 - 1) = v23;
            v25 = v22 - v0;
            v26 = v24 + 1;
            v27 = v17;
            if (v22 != v26)
            {
                v0 = v27 - v26;
                if (v26 <= v25)
                    goto LABEL_409958;
                if (!g_42a7b0)
                {
                    if (!g_42a7a8)
                        goto LABEL_4099f0;
                    else
                        goto LABEL_40991a;
                }
                else
                {
                    iter += sub_409350(v25, v26);
                    if (!g_42a7a8)
                    {
                        if (!g_42a7b0)
                            goto LABEL_4099f0;
LABEL_409936:
                        if (g_42a7a2)
                        {
                            v28 = 0;
                            if (v1 >= 0)
                                v28 = v1;
                        }
                        if (v28 >= iter)
                            goto LABEL_409958;
                        goto LABEL_4099f0;
                    }
                }
LABEL_40991a:
                sub_408ff0(v26);
                if (g_42a7b0 || g_42a7a8)
                    goto LABEL_409936;
                goto LABEL_4099f0;
            }
            else
            {
                v26 = v25;
                v0 = v27 - v25;
LABEL_409958:
                if (g_42a7f8 > 0)
                {
                    i = 0;
                    do
                    {
                        if (*((long long *)&g_42a848) >= v26)
                            break;
                        if (v26 == g_42a7b8)
                            goto LABEL_4099af;
                        i += 1;
                        do
                        {
                            v26 -= 1;
                        } while (*((char *)v26 - 1) != g_42a684);
                    } while (i != g_42a7f8);
                    goto LABEL_4099af;
                }
                if (v26 != g_42a7b8)
                    g_42a7b8 = 0;
LABEL_4099af:
                v16 = v17 - v26;
                if (g_42a800)
                {
                    v30 = sub_407860(*((long long *)&g_42a7c8), &g_42a840[-1 * *((long long *)&g_42a848) + -1 * v16]);
                    v31 = g_42a801;
                    *((long long *)&g_42a7c8) = v30;
                    if (!(char)v31)
                        continue;
                }
                else if (!g_42a801)
                {
                    continue;
                }
                sub_408250(v26);
            }
        }
    } while ((v15 = 0, (char)sub_408560(v16, a1)));
    err = __errno_location();
    sub_4094a0(*(err));
LABEL_4099f0:
    g_42a7a2 = v7;
    g_42a803 = v8;
    if (*((int *)&g_42a810))
    {
        return iter;
    }
    else if (!v8)
    {
        if (!g_42a888 && (v1 < 0 || iter <= v1))
            return iter;
        sub_408a50();
        error(0, 0, dcgettext(NULL, "%s: binary file matches", 5));
        return iter;
    }
    else
    {
        return iter;
    }
}



// Function: grepdirent @ 0xa180
typedef struct struct_2 {
    char padding_0[72];
    unsigned int field_48;
} struct_2;

typedef struct struct_0 {
    char padding_0[56];
    unsigned long long field_38;
    char padding_40[24];
    unsigned long long field_58;
    char padding_60[8];
    unsigned short field_68;
} struct_0;

typedef struct struct_1 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern unsigned int g_420520[4];
extern char g_42a88a;
extern struct_1 *g_42a890;

unsigned long long grepdirent(struct_2 *a0, struct_0 *a1, char a2)
{
    unsigned long v1;  // rax
    unsigned int v2;  // eax
    unsigned long v3;  // 4115

    v1 = a1->field_68;
    if ((unsigned short)v1 == 6)
        return 1;
    if (!(!a1->field_58 & a2))
    {
        v2 = sub_4081e0(&a1[2].padding_0[44], 0, (unsigned short)(v1 - 1 & 0xffffffff) <= 1 | (unsigned short)v1 == 4);
        if ((char)v2)
        {
            sub_417830(a0, a1, 4);
            return v2;
        }
        v1 = a1->field_68;
    }
    v3 = g_42a88a;
    g_42a890 = a1->field_38;
    if ((char)v3 && *((char *)&g_42a890->field_0 + 1))
        g_42a890 = (char *)&g_42a890->field_0 + 2;
    if ((unsigned short)v1 > 14)
        abort(); /* do not return */
    goto *((void *)((long long)(g_420520[v1] + (char *)&g_420520[0])));
}



// Function: grep_command_line_arg @ 0xa3f0
extern unsigned long long g_42a7d0;
extern unsigned long long g_42a890;

long long grep_command_line_arg(unsigned long a0)
{
    if (strcmp(a0, "-"))
        g_42a890 = a0;
    g_42a890 = g_42a7d0;
}



// Function: usage @ 0xa440
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
    sub_4179e0();
    if (a0)
    {
        v3 = dcgettext(NULL, "Usage: %s [OPTION]... PATTERNS [FILE]...\n", 5);
        __fprintf_chk(stderr, 0x1, v3);
        sub_4179e0(v7, v8, v9, v10, v11, v12);
        v4 = dcgettext(NULL, "Try '%s --help' for more information.\n", 5);
        __fprintf_chk(stderr, 0x1, v4);
    }
    else
    {
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... PATTERNS [FILE]...\n", 5));
        __printf_chk(1, dcgettext(NULL, "Search for PATTERNS in each FILE.\n", 5));
        sub_4179e0(v7, v8, v9, v10, v11, v12);
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
        sub_41d030(v7, v8, v9, v10, v11, v12);
    }
    exit(a0); /* do not return */
}



// Function: fgrep_to_grep_pattern @ 0xa6b0
typedef struct struct_0 {
    char field_0;
} struct_0;

void fgrep_to_grep_pattern(struct_0 **a0, struct_0 **a1)
{
    char *v4;  // r15
    char *v5;  // r12
    char *iter;  // r13
    char *v7;  // rax
    char *v8;  // r14
    char *v10;  // r15
    char *v11;  // r12
    unsigned int v12;  // eax
    unsigned long v13;  // rdx
    unsigned long v0;  // [bp-0x188]
    char *v1;  // [bp-0x60]
    unsigned long v2;  // [bp-0x48]

    v4 = &*(a1)->field_0;
    v5 = &*(a0)->field_0;
    v2 = 0;
    v1 = sub_41d210(v4 + 1, 2);
    if (v4)
    {
        iter = v1;
        while (1)
        {
            v7 = sub_40c9f0(v5, v4, &v2);
            if (v7 == 0xffffffffffffffff)
            {
                v2 = 0;
                goto LABEL_40a767;
            }
            switch (v7)
            {
            case 1:
LABEL_40a767:
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
                    goto LABEL_40a7ab;
                else
                    break;
            case 18446744073709551614:
                v8 = NULL;
                goto LABEL_40a72a;
            default:
                v8 = NULL;
                v8 = v4 - v7;
                v4 = v7;
LABEL_40a72a:
                v10 = v8;
                v11 = v5 + v4;
                iter = mempcpy(iter, v5, v4);
                v4 = v10;
                v5 = v11;
                if (!v4)
                    goto LABEL_40a7ab;
                else
                    break;
            }
        }
LABEL_40a7ab:
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



// Function: Fcompile @ 0xa820
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

extern char g_42a684;
extern char g_42a685;

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
    v6 = sub_40ca70(1);
    v1 = &v2;
    iter = v4;
    do
    {
        ptr1 = rawmemchr(iter, 10);
        n = ptr1 - iter;
        if (g_42a685)
        {
            v10 = g_42a684;
            if (iter <= a0 || g_42a684 != 10)
            {
                if (n + 1 >= v2)
                {
                    free(ptr);
                    v2 = n;
                    v11 = sub_41d2b0(0, v1, 2, -0x1, 1);
                    v10 = g_42a684;
                    ptr = v11;
                    *(ptr) = g_42a684;
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
        sub_40bab0(v6, iter, n);
        iter = v12;
    } while (iter <= &v4[a1]);
    free(ptr);
    v13 = sub_40bf90(v6);
    sub_40bfa0(v6);
    idx = sub_41d0f0(40);
    idx->field_0 = v6;
    idx->field_10 = a0;
    idx->field_8 = v13;
    idx->field_18 = a1;
    idx->field_20 = 0;
    return;
}



// Function: Fexecute @ 0xa9c0
typedef struct struct_0 {
    long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    unsigned int field_18;
    char padding_1c[4];
    void* field_20;
} struct_0;

extern char g_42a180;
extern char g_42a182;
extern char g_42a684;
extern char g_42a685;
extern char g_42a686;

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
    v4 = g_42a684;
    v16 = g_42a685;
    v3 = a4;
    v2 = g_42a180 & ((g_42a182 | g_42a685) ^ 1);
    v5 = a4;
    if (a4)
        v17 = a4;
    else
        v17 = a1;
    v0 = (v2 | g_42a686 | v3) & (g_42a685 ^ 1);
    v7 = v17;
    if (v14 < v17)
        return 0xffffffffffffffff;
    v1 = &v9;
    while (1)
    {
        v18 = sub_40c870(v15, v17 - v16, v14 - v17 + v16, v1, v0);
        if (v18 < 0)
            break;
        v19 = v17 + v18;
        v8 = 0;
        v20 = v10 - g_42a685 * 2;
        if (!v2 || !sub_40caf0(&v7, &v8, v19, v14))
        {
            if (g_42a686 != 1 && v3)
            {
                *(a3) = v20;
                return v19 - a1;
            }
            if (g_42a685)
            {
                *(a3) = v20 + (a4 < 1);
                return v19 - a1;
            }
            if (!g_42a686)
            {
                v26 = v19 + v20;
LABEL_40acfd:
                v28 = v4;
                if (v14 > v26)
                    v14 = rawmemchr(v26, v28) + 1;
LABEL_40ad17:
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
                if (sub_40cc90(v19 - v8, v14))
                    goto LABEL_40ac20;
                while (1)
                {
LABEL_40ab92:
                    v23 = v19 + v20;
                    if (!sub_40cc90(v23, v14))
                    {
                        v26 = v23;
                        if (a4)
                        {
                            *(a3) = v20;
                            return v19 - a1;
                        }
                        goto LABEL_40acfd;
                    }
                    if (!a4 && !g_42a180)
                        break;
                    if (!v20 || sub_40c870(v15, v19, v20 - 1, &v11, 1))
                        goto LABEL_40ac20;
                    v20 = v12;
                }
                if (!a0->field_20)
                {
                    sub_40a6b0(&a0->field_10, &a0->field_18);
                    a0->field_20 = sub_406690(*((long long *)&a0->field_10), *((long long *)&a0->field_18), 68166, v5);
                }
                v24 = v14;
                if (v24 > v23)
                    v24 = rawmemchr(v23, v4) + 1;
                v6 = v24;
                if (sub_406c90(a0->field_20, v19, v24 - v19, a3, 0) < 0)
                {
                    v19 = v6 - 1;
                }
                else
                {
                    v28 = v4;
                    v14 = v6;
                    goto LABEL_40ad17;
                }
LABEL_40ac20:
                v7 = v19 + sub_40cc80(v19, v14);
                v17 = v7 + 1;
                if (v14 < v17)
                    break;
                else
                    goto LABEL_40ac3c;
            }
            else
            {
                v21 = v7;
            }
            if (sub_40cca0(v21, v19, v14))
                goto LABEL_40ac20;
            goto LABEL_40ab92;
        }
        else
        {
            v17 = v7;
            if (v14 < v17)
                break;
LABEL_40ac3c:
            v16 = g_42a685;
        }
    }
    return 0xffffffffffffffff;
}



// Function: U @ 0xadc0
void U(char a0)
{
    sub_40c9d0(a0);
    return;
}



// Function: tr @ 0xadd0
unsigned int tr(unsigned long long a0, unsigned int a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x8]

    if (!a0)
        return a1;
    v0 = v2;
    return *((char *)(a0 + ((unsigned long long)sub_40adc0(a1) & 0xff)));
}



// Function: enqueue @ 0xae00
typedef struct struct_1 {
    void* field_0;
    void* field_8;
    struct struct_0 *field_10;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_0;

struct_0 * enqueue(struct_1 *a0, struct_0 **a1)
{
    unsigned long long v2;  // rbx
    struct_0 *v3;  // rax
    struct_0 *v4;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (!a0)
        return v4;
    v0 = v2;
    enqueue(a0->field_0, a1);
    enqueue(a0->field_8, a1);
    v3 = a0->field_10;
    *(a1)->field_18 = v3;
    *(a1) = v3;
    return v3;
}



// Function: treefails @ 0xae50
typedef struct struct_4 {
    struct struct_4 *field_0;
    struct struct_4 *field_8;
    struct struct_0 *field_10;
} struct_4;

typedef struct struct_1 {
    void* field_0;
    void* field_8;
    struct struct_0 *field_10;
    char field_18;
} struct_1;

typedef struct struct_5 {
    char padding_0[8];
    struct struct_4 *field_8;
    char padding_10[16];
    struct struct_5 *field_20;
} struct_5;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[24];
    struct struct_0 *field_20;
} struct_0;

struct_0 * treefails(struct_1 *idx, struct_5 *a1, struct_0 *a2, char a3)
{
    unsigned long long v5;  // r14
    unsigned long long v6;  // r13
    struct_0 *v15;  // rax
    struct_0 *v16;  // rax
    unsigned long long v7;  // r12
    unsigned long long v8;  // rbx
    struct_5 *v9;  // rbx
    struct_5 *index;  // rbx
    struct_4 *idx1;  // rax
    struct_4 *v12;  // rcx
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
        } while ((v9 = (struct_5 *)index->field_20, index->field_20));
    }
    v15 = idx->field_10;
    v15->field_20 = a2;
    return v15;
}



// Function: treedelta @ 0xaf10
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    char padding_10[8];
    char field_18;
} struct_0;

long long treedelta(struct_0 *a0, long long a1, long long a2)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    char *v5;  // rbx
    long long v6;  // rax
    long long v7;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!a0)
        return v7;
    v1 = v3;
    v0 = v4;
    treedelta(a0->field_0, a1, a2);
    treedelta(a0->field_8, a1, a2);
    v5 = a2 + a0->field_18;
    v6 = *(v5);
    if (v6 <= a1)
        return v6;
    *(v5) = a1;
    return v6;
}



// Function: hasevery @ 0xaf60
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



// Function: treenext @ 0xafe0
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    unsigned long long field_10;
    char field_18;
} struct_0;

int treenext(struct_0 *a0, long long a1)
{
    unsigned long long v2;  // rbx
    unsigned long idx;  // rax
    unsigned int v4;  // eax
    unsigned long long v0;  // [bp-0x10]

    if (!a0)
        return v4;
    v0 = v2;
    treenext(a0->field_0, a1);
    treenext(a0->field_8, a1);
    idx = a0->field_18;
    *((unsigned long long *)(a1 + idx * 8)) = a0->field_10;
    return idx;
}



// Function: bm_delta2_search @ 0xb030
typedef struct struct_0 {
    char padding_0[2424];
    unsigned long long field_978;
} struct_0;

unsigned int bm_delta2_search(void* *a0, void* a1, unsigned long a2, long long a3, unsigned long long a4, char a5, char a6, unsigned long long a7, struct_0 *idx)
{
    void* iter;  // rbx
    long long v2;  // r13
    unsigned long long v3;  // rcx
    long long node;  // r14
    long long v5;  // r15
    char v6;  // al
    long long v7;  // r13
    char v8;  // al
    unsigned int v9;  // r12d
    long long v0;  // [bp-0x58]

    iter = *(a0);
    v0 = 0;
    v2 = a3;
    while (1)
    {
        if ((char)sub_40add0(a4, *((char *)iter - 2)) != a6)
        {
            v3 = 0;
            node = 2;
        }
        else
        {
            node = 2;
            do
            {
                node += 1;
                if (v2 < node)
                {
                    node = v2 + v0 + 1;
                    if (a3 < node)
                    {
                        *(a0) = iter - a3;
                        return 1;
                    }
                    do
                    {
                        v7 = -(node);
                        v8 = sub_40add0(a4, *((char *)iter + v7));
                        if (v8 != (char)sub_40add0(a4, *((char *)(a2 + v7))))
                            goto LABEL_40b167;
                    } while ((node = (long long)(node + 1), a3 >= node));
                }
            } while ((v5 = -(node), v6 = (char)(unsigned long long)sub_40add0(a4, (int)*((char *)((char *)iter + v5))), v6 == (char)sub_40add0(a4, (int)*((char *)(a2 + v5)))));
LABEL_40b167:
            v3 = node * 8 - 16;
        }
        v2 = *((long long *)(idx->field_978 + v3));
        iter += *((long long *)(idx->field_978 + v3));
        if (iter > a1)
            break;
        v9 = *((char *)iter - 1);
        if ((char)sub_40add0(a4, v9) == a5)
        {
            v0 = node - 1;
        }
        else if (a7)
        {
            iter += *((char *)(a7 + ((unsigned long long)sub_40adc0(v9) & 0xff)));
            break;
        }
    }
    *(a0) = iter;
    return 0;
}



// Function: memchr_kwset @ 0xb1c0
void* memchr_kwset(void* a0, unsigned long a1, void* a2)
{
    void* v3;  // r12
    void* node;  // rbp
    unsigned long long v13;  // rbp
    unsigned long long v14;  // rbx
    unsigned long v15;  // rcx
    unsigned long v16;  // rax
    unsigned long v17;  // rsi
    void* iter;  // rax
    char v20;  // al
    char v5;  // sil
    unsigned long long v6;  // rcx
    void* iter1;  // rdi
    char v8;  // r10b
    unsigned long iter2;  // rdx
    char k;  // r9b
    char v11;  // r8b
    unsigned long j;  // rdx
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v3 = a0 + a1;
    node = a0;
    if ((int)a2[2444] >= 0)
    {
        while (node < v3)
        {
            if (*((long long *)((char *)a2 + 8 * ((unsigned long long)sub_40adc0(*((char *)node)) & 0xff) + 368)))
                return node;
            node += 1;
            if (!(*((char *)&node) & 15))
            {
                v5 = (int)a2[2440];
                v6 = v3 - node;
                iter1 = node;
                v8 = (int)a2[2444];
                iter2 = v3 - node;
                if (v5 == v8)
                    return memchr(iter1, v5, iter2);
                k = v5;
                v11 = v8;
                if (iter2)
                {
                    do
                    {
                        j = iter2;
                        iter2 = j;
                        if (!(*((char *)&iter1) & 7))
                            break;
                        if (*((char *)iter1) == v11)
                            return iter1;
                        if (*((char *)iter1) == k)
                            return iter1;
                        iter1 += 1;
                        iter2 = j - 1;
                    } while (j != 1);
                }
                v1 = v13;
                v0 = v14;
                v15 = v5 * 0x100 | v5;
                v16 = v11 * 0x100 | v11;
                if (iter2 > 7)
                {
                    do
                    {
                        v17 = *((long long *)iter1) ^ ((v15 * 0x10000 | v15) * 0x100000000 | v15 * 0x10000 | v15);
                        if (0x8080808080808080 & (~(*((long long *)iter1) ^ ((v16 | v16 * 0x10000) * 0x100000000 | v16 | v16 * 0x10000)) & (*((long long *)iter1) ^ ((v16 | v16 * 0x10000) * 0x100000000 | v16 | v16 * 0x10000)) + 0xfefefefefefefeff | v17 + 0xfefefefefefefeff & ~(v17)))
                            goto LABEL_4191d6;
                    } while ((iter2 = (unsigned long)(iter2 - 8), iter1 += 8, iter2 > 7));
                }
                if (!iter2)
                    return NULL;
LABEL_4191d6:
                iter = iter1;
                while (*((char *)iter) != k)
                {
                    if (*((char *)iter) == v11)
                        return iter;
                    iter += 1;
                    if (iter == iter2 + iter1)
                        return NULL;
                }
                return iter;
            }
        }
    }
    else if (a0 < v3)
    {
        while (1)
        {
            v20 = (unsigned long long)sub_40adc0(*((char *)node));
            if (*((long long *)((char *)a2 + 8 * v20 + 368)))
                return node;
            node += 1;
            if (v3 == node)
                return *((long long *)((char *)a2 + 8 * v20 + 368));
        }
    }
    return NULL;
}



// Function: acexec @ 0xb530
void acexec(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, char a4)
{
}



// Function: bmexec_trans @ 0xb540
typedef struct struct_0 {
    char padding_0[104];
    long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
    char field_988;
    char padding_989[7];
    char field_990;
} struct_0;

long long bmexec_trans(struct_0 *idx, void* a1, unsigned long long a2)
{
    long long v8;  // r12
    struct_0 *v9;  // rbx
    unsigned long v20;  // rcx
    void* v21;  // rdx
    unsigned long v24;  // rcx
    void* v25;  // rdx
    void* v27;  // rax
    void* v10;  // r15
    unsigned long v28;  // r14
    void* v29;  // rax
    unsigned long long v11;  // 4182
    void* v12;  // r14
    void* v13;  // rsi
    void* v14;  // r15
    void* v15;  // rbp
    unsigned long v16;  // rcx
    void* v17;  // rdx
    unsigned long v0;  // [bp-0x88]
    unsigned long long v1;  // [bp-0x80]
    char v2;  // [bp-0x75]
    char v3;  // [bp-0x74], Other Possible Types: unsigned int
    void* v4;  // [bp-0x70]
    void* v5;  // [bp-0x50]
    void* v6;  // [bp-0x48]

    v8 = idx->field_68;
    if (!v8)
        return v8;
    v0 = idx->field_980;
    if (a2 < v8)
    {
LABEL_40b67f:
        return -0x1;
    }
    else if (v8 != 1)
    {
        v9 = idx->padding_70;
        v10 = a1 + v8;
        v6 = v10;
        v1 = idx->field_970 + v8;
        v3 = idx->field_988;
        v2 = idx->field_990;
        v11 = __OFSMUL__(v8, 12);
        if (v8 * 12 >= a2 || (char)v11 & 1)
        {
            v13 = a2 + a1;
            goto LABEL_40b5f3;
        }
        v12 = a2 - v8 * 11 + a1;
        v13 = a1 + a2;
        v5 = v13;
        if (v10 > v12)
            goto LABEL_40b5f3;
        idx = idx;
        while (1)
        {
            v14 = v10;
            v15 = v14 + v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v14 - 1)) & 0xff];
            v16 = v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v15 - 1)) & 0xff];
            v17 = v15 + v16;
            v6 = v17;
            if (!(char)v16)
            {
LABEL_40b730:
                if (!(char)sub_40b030(&v6, v12, v1, v8, v0, v3, v2, v9, idx))
                {
                    v10 = v6;
                    goto LABEL_40b76e;
                }
                return v6 - a1;
            }
            v4 = v17;
            v4 += v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v17 - 1)) & 0xff];
            v4 += v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v4 - 1)) & 0xff];
            v20 = v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v4 - 1)) & 0xff];
            v21 = v4 + v20;
            v6 = v21;
            if (!(char)v20)
                goto LABEL_40b730;
            v4 = v21;
            v4 += v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v21 - 1)) & 0xff];
            v4 += v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v4 - 1)) & 0xff];
            v24 = v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v4 - 1)) & 0xff];
            v25 = v4 + v24;
            v6 = v25;
            if (!(char)v24)
                goto LABEL_40b730;
            v4 = v25;
            v4 += v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v25 - 1)) & 0xff];
            v10 = v4 + v9->padding_0[(unsigned long long)sub_40adc0(*((char *)v4 - 1)) & 0xff];
            v6 = v10;
            if (v10 - v14 <= 127)
                break;
LABEL_40b76e:
            if (v12 < v10)
            {
                v13 = a2 + a1;
                goto LABEL_40b5f3;
            }
        }
        v27 = sub_40b1c0(v10 - 1, v5 - (v10 - 1), idx);
        if (!v27)
            goto LABEL_40b67f;
        v10 = v27 + 1;
        v6 = v10;
        if (v10 < v12)
            goto LABEL_40b730;
        v13 = v5;
LABEL_40b5f3:
        v28 = idx->padding_70[(unsigned long long)sub_40adc0(*((char *)v10 - 1)) & 0xff];
        v3 = v3;
    }
    else
    {
        v29 = sub_40b1c0(a1, a2, idx);
        if (v29)
            return v29 - a1;
        goto LABEL_40b67f;
    }
}



// Function: bmexec @ 0xb930
void bmexec(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40b93c();
    return;
}



// Function: kwsalloc @ 0xb960
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
    idx = sub_41d0f0(2464);
    _obstack_begin(idx, 0, 0, sub_41d0f0, free);
    v6 = idx->field_20;
    v7 = idx->field_18;
    idx->field_58 = 0;
    if (v6 - v7 > 63)
    {
        v8 = idx->field_10;
        v9 = v7 + 64;
        if (v9 == v8)
            goto LABEL_40baa1;
    }
    else
    {
        _obstack_newchunk(idx, 64);
        v8 = idx->field_10;
        v6 = idx->field_20;
        v9 = idx->field_18 + 64;
        if (v9 == v8)
        {
LABEL_40baa1:
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
    idx[1].field_10 = sub_40b530;
    idx->field_970 = 0;
    return idx;
}



// Function: kwsincr @ 0xbab0
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
    if (v10 == sub_40b930)
        node += a2;
    if (a2)
    {
        v12 = a2 - 1;
        v13 = &idx->field_980->field_0;
        idx1 = idx;
        do
        {
            v14 = v12;
            if (v10 != sub_40b930)
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
LABEL_40bb9c:
                v20 = idx1->field_20;
                if (v20 - idx1->field_18 > 31)
                {
                    v21 = idx1->field_10;
                    v22 = idx1->field_18 + 32;
                    if (v22 != v21)
                        goto LABEL_40bbc5;
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
                        goto LABEL_40bbc5;
                }
                idx1->field_50 = idx1->field_50 | 2;
LABEL_40bbc5:
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
                            goto LABEL_40bc75;
LABEL_40be4f:
                        idx1->field_50 = idx1->field_50 | 2;
LABEL_40bc75:
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
                    goto LABEL_40bc75;
                goto LABEL_40be4f;
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
                        goto LABEL_40bb9c;
                    goto LABEL_40bb70;
                }
                iter = iter->field_8;
                *((unsigned int *)&(&v4)[4 * idx2]) = 1;
                if (!iter)
                    goto LABEL_40bb9c;
LABEL_40bb70:
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



// Function: kwswords @ 0xbf90
typedef struct struct_0 {
    char padding_0[88];
    unsigned long long field_58;
} struct_0;

long long kwswords(struct_0 *a0)
{
    return a0->field_58;
}



// Function: kwsprep @ 0xbfa0
typedef struct struct_6 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_6;

typedef struct struct_1 {
    void* field_0;
    void* field_8;
    struct struct_0 *field_10;
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
    char padding_0[24];
    char field_18;
} struct_0;

typedef struct struct_2 {
    char padding_0[24];
    char *field_18;
} struct_2;

typedef struct struct_8 {
    char padding_0[8];
    struct struct_0 *field_8;
    struct struct_1 *field_10;
    struct struct_8 *field_18;
    char padding_20[16];
    unsigned long long field_30;
    unsigned long long field_38;
} struct_8;

typedef struct struct_7 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char padding_10[8];
    char *field_18;
    struct struct_6 *field_20;
    long long field_28;
    struct struct_8 *field_30;
    struct struct_8 *field_38;
} struct_7;

typedef struct struct_5 {
    char field_0;
} struct_5;

void kwsprep(void* idx)
{
    void* index;  // rbp
    unsigned long v8;  // rbx
    char *v17;  // rdx
    char *v18;  // r9
    char *v19;  // r14
    char *v20;  // rcx
    char *v21;  // rax
    char *v22;  // rsi
    char *v23;  // rax
    char *v24;  // rax
    struct_7 *idx1;  // r8
    struct_4 *v26;  // rax
    void* idx2;  // r12
    char *v27;  // rdx
    char *v28;  // rsi
    struct_4 *ptr;  // r14
    unsigned long long v30;  // rcx
    void* node;  // rdi
    struct_4 *iter;  // rsi
    unsigned long v33;  // d
    char v34;  // al
    unsigned long v35;  // rax
    char *v36;  // rdi
    char v10;  // al
    char *iter1;  // rax
    struct_0 *v38;  // r9
    char *v39;  // rax
    struct_6 *m;  // r9
    unsigned long long v41;  // r14
    struct_0 *v42;  // r10
    unsigned long v43;  // rax
    unsigned long v44;  // rax
    unsigned long v45;  // rdx
    unsigned long v46;  // rcx
    unsigned long v11;  // rax
    void* v47;  // rax
    unsigned int v48;  // r15d
    unsigned long long iter2;  // r14
    unsigned long long *i;  // rdi
    unsigned long long v51;  // rcx
    unsigned int v52;  // r13d
    char v53;  // al
    unsigned int v54;  // edi
    unsigned long v55;  // rax
    char *v56;  // rdx
    char *v12;  // rdi
    char *v57;  // rsi
    char *v58;  // r13
    char *v59;  // rcx
    char *v60;  // rax
    char *v61;  // rdi
    char *v62;  // rax
    char *v63;  // rdx
    unsigned long v64;  // rax
    unsigned long v65;  // rax
    char *v66;  // rcx
    char *v13;  // r8
    char *v67;  // rdx
    char *v68;  // rax
    long long v69;  // r13
    char *v70;  // rdi
    char *v71;  // rax
    char *v72;  // rsi
    char *v73;  // rax
    char *v74;  // rax
    char *v75;  // rdx
    unsigned long k;  // rax
    char *v14;  // r8
    char *v77;  // rdx
    unsigned long long j;  // r13
    struct_0 **v15;  // r11
    struct_2 *v16;  // rcx
    unsigned int v0;  // [bp-0x96c]
    void* v1;  // [bp-0x968]
    char *v2;  // [bp-0x960]
    char *v3;  // [bp-0x950]
    char *v4;  // [bp-0x948]
    char *v5;  // [bp-0x148]

    index = idx;
    v8 = (long long)idx[2432];
    idx2 = &v5;
    if (!v8)
        idx2 = idx + 112;
    v2 = (long long)idx[88];
    if ((long long)idx[88] != 0x1)
    {
        v10 = (long long)idx[104];
        if (0xff < (long long)idx[104])
            v10 = 0xff;
        v11 = v10;
        *((unsigned long long *)idx2) = v11 * 0x101010101010101;
        v12 = 0x101010101010101 * v11 + (v11 * 0x101010101010101 >> 64);
        *((char **)&idx2[8]) = v12;
        *((char **)&idx2[24]) = v12;
        *((unsigned long long *)&idx2[16]) = v11 * 0x101010101010101;
        *((char **)&idx2[40]) = v12;
        *((char **)&idx2[56]) = v12;
        *((char **)&idx2[72]) = v12;
        *((unsigned long long *)&idx2[32]) = v11 * 0x101010101010101;
        *((char **)&idx2[88]) = v12;
        *((char **)&idx2[104]) = v12;
        *((char **)&idx2[120]) = v12;
        *((unsigned long long *)&idx2[48]) = v11 * 0x101010101010101;
        *((char **)&idx2[0x88]) = v12;
        *((char **)&idx2[152]) = v12;
        *((char **)&idx2[168]) = v12;
        *((unsigned long long *)&idx2[64]) = v11 * 0x101010101010101;
        *((char **)&idx2[184]) = v12;
        *((char **)&idx2[200]) = v12;
        *((char **)&idx2[216]) = v12;
        *((unsigned long long *)&idx2[80]) = v11 * 0x101010101010101;
        *((char **)&idx2[232]) = v12;
        *((unsigned long long *)&idx2[96]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[112]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[128]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[144]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[160]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[176]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[192]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[208]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[224]) = v11 * 0x101010101010101;
        *((unsigned long long *)&idx2[240]) = v11 * 0x101010101010101;
        *((char **)&idx2[248]) = v12;
        v13 = (long long)index[96];
        v3 = v13;
        if (!v13)
            goto LABEL_40c233;
    }
    else
    {
        v14 = (long long)idx[96];
        v3 = v14;
        if (v14)
        {
            do
            {
                sub_40ae00(*((long long *)&v14[8]), v15);
                v14 = v16->field_18;
            } while (v16->field_18);
        }
        v17 = (long long)index[32];
        v18 = (long long)index[104];
        v19 = v18;
        if (v18 <= &v17[-1 * index[24]])
        {
            v20 = (long long)index[16];
            v21 = &v19[index[24]];
            if (v21 == v20)
                goto LABEL_40c855;
        }
        else
        {
            _obstack_newchunk(index, v18);
            v20 = (long long)index[16];
            v17 = (long long)index[32];
            v21 = &v19[index[24]];
            v18 = (long long)index[104];
            if (v21 == v20)
            {
LABEL_40c855:
                *((char *)&index[80]) = (char)index[80] | 2;
            }
        }
        v22 = (long long)index[48];
        *((char **)&index[2416]) = v20;
        v23 = v21 + v22 & ~(v22);
        if (&v23[-1 * index[8]] <= &v17[-1 * index[8]])
            v17 = v23;
        *((char **)&index[24]) = v17;
        *((char **)&index[16]) = v17;
        if (v18 > 0)
        {
            v24 = NULL;
            while (1)
            {
                *((char *)(v20 + v24)) = idx1->field_8->field_18;
                v24 += 1;
                if ((long long)index[104] <= v24)
                    break;
                v20 = (long long)index[2416];
            }
        }
        v26 = sub_40b960((long long)index[2432]);
        v27 = (long long)index[104];
        v28 = (long long)index[2416];
        ptr = v26;
        ptr->field_998 = sub_40b930;
        sub_40bab0(ptr, v28, v27);
        _obstack_free(index, 0);
        v30 = 308;
        node = index;
        for (iter = ptr; v30; iter = &(iter->padding_0)[v33])
        {
            v30 -= 1;
            *((char [8])node) = iter->padding_0;
            node += v33 * 8;
        }
        free(ptr);
        v34 = (long long)index[104];
        if (0xff < (long long)index[104])
            v34 = 0xff;
        v35 = v34;
        *((unsigned long long *)idx2) = v35 * 0x101010101010101;
        v36 = 0x101010101010101 * v35 + (v35 * 0x101010101010101 >> 64);
        *((char **)&idx2[8]) = v36;
        *((char **)&idx2[24]) = v36;
        *((unsigned long long *)&idx2[16]) = v35 * 0x101010101010101;
        *((char **)&idx2[40]) = v36;
        *((char **)&idx2[56]) = v36;
        *((char **)&idx2[72]) = v36;
        *((unsigned long long *)&idx2[32]) = v35 * 0x101010101010101;
        *((char **)&idx2[88]) = v36;
        *((char **)&idx2[104]) = v36;
        *((char **)&idx2[120]) = v36;
        *((unsigned long long *)&idx2[48]) = v35 * 0x101010101010101;
        *((char **)&idx2[0x88]) = v36;
        *((char **)&idx2[152]) = v36;
        *((char **)&idx2[168]) = v36;
        *((unsigned long long *)&idx2[64]) = v35 * 0x101010101010101;
        *((char **)&idx2[184]) = v36;
        *((char **)&idx2[200]) = v36;
        *((char **)&idx2[216]) = v36;
        *((unsigned long long *)&idx2[80]) = v35 * 0x101010101010101;
        *((char **)&idx2[232]) = v36;
        *((unsigned long long *)&idx2[96]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[112]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[128]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[144]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[160]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[176]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[192]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[208]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[224]) = v35 * 0x101010101010101;
        *((unsigned long long *)&idx2[240]) = v35 * 0x101010101010101;
        *((char **)&idx2[248]) = v36;
        iter1 = (long long)index[96];
        v3 = iter1;
        v13 = iter1;
        if (!iter1)
            goto LABEL_40c3d0;
    }
    while (1)
    {
        sub_40ae00(*((long long *)&v13[8]), v15);
        sub_40af10(v38, idx1->field_28, idx2);
        sub_40ae50(idx1->field_8, idx1->field_20, (long long)index[96], (char)v16);
        if (v2 != 0x1 || !(v39 = (char *)(long long)index[104], m = (struct_6 *)idx1->field_20, idx1->field_30 = (struct struct_8 *)v39, idx1->field_38 = (struct struct_8 *)v39, m))
        {
            v13 = idx1->field_18;
            if (!v13)
                break;
        }
        else
        {
            v41 = idx1->field_0;
            do
            {
                if (!(char)sub_40af60(m->field_8, v42))
                {
                    v43 = idx1->field_28 - *((long long *)&v38[1].padding_0[8]);
                    if (v43 < *((long long *)&v38[1].padding_0[16]))
                        *((unsigned long *)&v38[1].padding_0[16]) = v43;
                }
                if (v41)
                {
                    v44 = idx1->field_28 - *((long long *)&v38[1].padding_0[8]);
                    if (v38[1].field_18 > v44)
                        v38[1].field_18 = v44;
                }
                m = *((long long *)&v38[1].padding_0[0]);
            } while (m);
            v13 = idx1->field_18;
            if (!v13)
                break;
        }
    }
    if (v2 == 0x1)
    {
        iter1 = (long long)index[96];
LABEL_40c3d0:
        iter1 = *((long long *)&iter1[24]);
        if (iter1)
        {
            do
            {
                v45 = *((long long *)&iter1[56]);
                v46 = *((long long *)(*((long long *)&iter1[16]) + 56));
                if (v45 > v46)
                {
                    *((unsigned long *)&iter1[56]) = v46;
                    v45 = v46;
                }
                if (*((long long *)&iter1[48]) <= v45)
                    goto LABEL_40c3d0;
            } while ((*((unsigned long *)&iter1[48]) = v45, iter1 = (char *)*((long long *)&iter1[24]), iter1));
        }
    }
LABEL_40c233:
    v47 = index + 368;
    v48 = 0xfffffffe;
    if (v8)
        v47 = &v4;
    iter2 = 0;
    v1 = v47;
    i = v47 + 8 & 0xfffffffffffffff8;
    *((unsigned long *)v47) = 0;
    *((unsigned long *)&v47[0x7f8]) = 0;
    for (v51 = (unsigned int)(v47 - i) + 0x800 >> 3; v51; i = &i[v33])
    {
        v51 -= 1;
        *(i) = 0;
    }
    sub_40afe0(*((long long *)((long long)index[96] + 8)), v47);
    v0 = 0xffffffff;
    while (1)
    {
        v52 = iter2;
        if (v8)
        {
            v53 = (unsigned long long)sub_40adc0(*((char *)(v8 + iter2)));
            v54 = v53;
            v55 = *((long long *)((char *)v1 + 8 * v53));
            *((unsigned long *)((char *)index + 8 * iter2 + 368)) = v55;
        }
        else
        {
            v55 = *((long long *)((char *)index + 8 * iter2 + 368));
            v54 = iter2;
        }
        if (v55)
        {
            if (v48 == 0xfffffffe)
            {
                iter2 += 1;
                v0 = v52;
                v48 = v54;
                if (iter2 == 0x100)
                    break;
                else
                    continue;
            }
            else if (v54 == v48)
            {
                if (v48 != v0)
                    v52 = 0xffffffff;
                v0 = v52;
            }
            else if (v54 == (unsigned int)iter2 && v48 == v0)
            {
                v0 = v52;
            }
            else
            {
                v48 = 0xffffffff;
            }
        }
        iter2 += 1;
        if (iter2 == 0x100)
            break;
    }
    *((unsigned int *)&index[2440]) = v48;
    *((unsigned int *)&index[2444]) = v0;
    if (v2 == 0x1)
    {
        v56 = (long long)index[32];
        v57 = (long long)index[104];
        v58 = v57;
        if (v57 <= &v56[-1 * index[24]])
        {
            v59 = (long long)index[16];
            v60 = &v58[index[24]];
            if (v60 == v59)
                goto LABEL_40c821;
        }
        else
        {
            _obstack_newchunk(index);
            v59 = (long long)index[16];
            v56 = (long long)index[32];
            v57 = (long long)index[104];
            v60 = &v58[index[24]];
            if (v60 == v59)
            {
LABEL_40c821:
                *((char *)&index[80]) = (char)index[80] | 2;
            }
        }
        v61 = (long long)index[48];
        *((char **)&index[2416]) = v59;
        v62 = v60 + v61 & ~(v61);
        if (&v62[-1 * index[8]] <= &v56[-1 * index[8]])
            v56 = v62;
        *((char **)&index[24]) = v56;
        *((char **)&index[16]) = v56;
        v63 = (long long)index[96];
        if (v57 > 0)
        {
            v64 = v57 - 1;
            while (1)
            {
                v59[v64] = *((char *)(*((long long *)&v63[8]) + 24));
                v63 = *((long long *)&v63[24]);
                v65 = v64 - 1;
                if (v64 < 0x1)
                    break;
                v59 = (long long)index[2416];
                v64 = v65;
            }
            v66 = (long long)index[104];
            if (v66 > 0x1)
            {
                v67 = (long long)index[32];
                v68 = (long long)index[24];
                v69 = v66 * 8 - 8;
                if (v69 > (long long)index[32] - (long long)index[24])
                {
                    _obstack_newchunk(index, v69);
                    v68 = (long long)index[24];
                    v67 = (long long)index[32];
                    v66 = (long long)index[104];
                }
                v70 = (long long)index[16];
                v71 = &v68[v69];
                if (v71 == v70)
                    *((char *)&index[80]) = (char)index[80] | 2;
                v72 = (long long)index[48];
                *((char **)&index[2424]) = v70;
                v73 = v71 + v72 & ~(v72);
                if (&v73[-1 * index[8]] <= &v67[-1 * index[8]])
                    v67 = v73;
                v74 = (long long)index[96];
                *((char **)&index[24]) = v67;
                *((char **)&index[16]) = v67;
                v75 = *((long long *)&v74[24]);
                if (v66 > 0x1)
                {
                    k = 0;
                    do
                    {
                        v77 = *((long long *)&v75[24]);
                        *((long long *)&v70[8 * k]) = *((long long *)&v75[48]);
                        v66 = (long long)index[104];
                        k += 1;
                        v75 = v77;
                    } while (v66 - 0x1 > k);
                }
                *((char *)&index[2448]) = sub_40add0(v8, *(&v66[index[2416]] - 2));
            }
        }
    }
    j = 0;
    if (!v8)
        return;
    do
    {
        *((char *)index + j + 112) = *((char *)idx2 + ((unsigned long long)sub_40adc0(*((char *)(v8 + j))) & 0xff));
        j += 1;
    } while (j != 0x100);
    return;
}



// Function: kwsexec @ 0xc870
typedef struct struct_0 {
    char padding_0[2456];
    unsigned long long field_998;
} struct_0;

long long kwsexec(struct_0 *a0, unsigned long a1, unsigned long a2, unsigned long a3, char a4)
{
    goto *((void *)(a0->field_998));
}



// Function: wordchar @ 0xc8b0
unsigned int wordchar(int a0)
{
    unsigned long long v1;  // rax

    if (a0 != 95)
    {
        v1 = iswalnum(a0);
        return _INSERT(v1, 0, (unsigned int)v1);
    }
    return 1;
}



// Function: wordinit @ 0xca30
extern int g_42a284;
extern int g_42a684;
extern void g_42a9c0;

unsigned int wordinit(void)
{
    void* iter;  // rbp
    int *i;  // rbx
    int *v3;  // rbx
    unsigned int v4;  // eax

    iter = &g_42a9c0;
    i = &g_42a284;
    do
    {
        v3 = i + 1;
        iter += 1;
        *((char *)iter - 1) = sub_40c8b0(*(i));
        i = v3;
    } while (i != &g_42a684);
    return v4;
}



// Function: kwsinit @ 0xca70
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

extern char g_42a687;

struct_0 * kwsinit(char a0)
{
    unsigned long long i;  // rbx
    unsigned long long v2;  // rbp

    if (g_42a687 && (__ctype_get_mb_cur_max() == 1 || a0))
    {
        i = 0;
        v2 = sub_41d110(0x100);
        do
        {
            *((char *)(v2 + i)) = toupper(i);
            i += 1;
        } while (i != 0x100);
        return sub_40b960(v2);
    }
    return sub_40b960(0);
}



// Function: mb_goback @ 0xcaf0
extern char g_42a182;

void* mb_goback(void* *a0, long long *a1, void* a2, unsigned long a3)
{
    void* v2;  // r13
    void* v3;  // r13
    long long v4;  // rax
    void* v5;  // rdx
    unsigned int v6;  // eax
    unsigned long long v7;  // rdx
    void* v8;  // r12
    long long v9;  // rax
    void* v10;  // rdx
    unsigned long v0;  // [bp-0x48], Other Possible Types: unsigned long long

    v2 = *(a0);
    v3 = v2;
    if (v3 >= a2)
    {
        return a2 - v3;
    }
    if (!g_42a182)
    {
        v0 = 0;
        while (1)
        {
            v3 = v2;
            v4 = sub_40c9f0(v3, a3 - v3, &v0);
            if (v4 >= 0)
            {
                v2 = v4 + v3;
                if (a2 <= v2)
                    break;
            }
            else
            {
                v0 = 0;
                v4 = 1;
                v2 = 1 + v3;
                if (a2 <= v2)
                    break;
            }
        }
        v5 = v2;
        if (a1)
        {
            *(a1) = v4;
            v5 = v2;
        }
    }
    else if ((*((char *)a2) & 192) != 128)
    {
        *(a0) = a2;
        return NULL;
    }
    else
    {
        v6 = *((char *)a2 - 1);
        if (((char)v6 & 192) == 128)
        {
            v6 = *((char *)a2 - 2);
            if (((char)v6 & 192) == 128)
            {
                v6 = *((char *)a2 - 3);
                if (((char)v6 & 192) == 128)
                {
                    *(a0) = a2;
                    return NULL;
                }
                v7 = 3;
            }
            else
            {
                v7 = 2;
            }
        }
        else
        {
            v7 = 1;
        }
        if ((int)(~(v6) & 0xff) >> ((char)(7 - (unsigned int)v7) & 31))
        {
            *(a0) = a2;
            return NULL;
        }
        v0 = 0;
        v8 = a2 - v7;
        v9 = sub_40c9e0(v8, a3 - v8, &v0);
        v5 = a2;
        v3 = v2;
        if (v9 >= 0)
        {
            v5 = v8 + v9;
            v3 = v8;
        }
    }
    v10 = v5;
    *(a0) = v10;
    if (v10 == a2)
        return NULL;
    return a2 - v3;
}



// Function: wordchars_size @ 0xcc80
void wordchars_size(void)
{
}



// Function: wordchar_next @ 0xcc90
void wordchar_next(void)
{
}



// Function: wordchar_prev @ 0xcca0
extern char g_42a180;
extern char g_42a182;
extern char g_42a9c0;

unsigned int wordchar_prev(void* a0, void* a1, unsigned int a2)
{
    unsigned long v2;  // rdx
    void* v0;  // [bp-0x28]

    if (a0 == a1)
        return 0;
    v2 = *((char *)a1 - 1);
    if (!g_42a180 || g_42a182 & ~((char)v2 >> 7))
        return (&g_42a9c0)[v2];
    v0 = a0;
    sub_40caf0(&v0, NULL, a1 - 1, a2);
    return (unsigned long long)sub_40cc90();
    return (&g_42a9c0)[v2];
}



// Function: private_malloc @ 0xcd70
void private_malloc(long long a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (a0 < 0)
    {
        v0 = v2;
        sub_41d4b0(); /* do not return */
    }
    sub_41d110();
    return;
}



// Function: jit_exec @ 0xcd90
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
                sub_41d4b0(); /* do not return */
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



// Function: Pcompile @ 0xcf00
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    unsigned long long field_20;
    long long field_28;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    unsigned short field_4;
    char field_6;
} struct_1;

extern long long g_4216a2;
extern char g_42a180;
extern char g_42a182;
extern char g_42a685;
extern char g_42a686;
extern char g_42a687;

struct_0 * Pcompile(struct_0 *a0, unsigned long a1)
{
    struct_0 *v7;  // r14
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
    struct_0 *v14;  // r10
    struct_1 *index;  // rax
    unsigned long v0;  // [bp-0x288]
    unsigned long v1;  // [bp-0x168]
    void* ptr;  // [bp-0x160], Other Possible Types: struct_0 *
    char v3;  // [bp-0x154], Other Possible Types: unsigned int
    char v4;  // [bp-0x150]
    char v5;  // [bp-0x148]

    v7 = a0;
    v1 = a1;
    ptr = (char *)a0 + a1;
    idx = sub_41d110(56);
    v9 = (-(g_42a687 < 1) & 0xfffffff8) + 24;
    v10 = pcre2_general_context_create_8(sub_40cd70, sub_40cd60, 0);
    idx->field_0 = v10;
    v11 = v10;
    v12 = pcre2_compile_context_create_8(v11);
    if (g_42a180)
    {
        if (g_42a182)
            v9 |= 0x4080000;
        else
            error(2, 0, dcgettext(NULL, "-P supports only unibyte and UTF-8 locales", 5));
    }
    if (ptr != rawmemchr(v7, 10))
        error(2, 0, dcgettext(NULL, "the -P option only supports a single pattern", 5));
    if (!g_42a685)
    {
        v14 = NULL;
        if (g_42a686)
        {
            v1 = a1 + 0x11;
            ptr = sub_41d110(v1);
            ptr->field_0 = 2893975473140547368;
            *((unsigned short *)&ptr->field_8) = 14911;
            index = mempcpy((char *)&ptr->field_8 + 2, v7, a1);
            v14 = ptr;
            index->field_0 = 557787177;
            v7 = v14;
            index->field_4 = 30556;
            index->field_6 = 41;
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
            *((int *)&idx[1].field_0) = sub_40cd90(idx, &g_4216a2, 0, 0, 1);
            *((int *)((char *)&idx[1].field_0 + 4)) = sub_40cd90(idx, &g_4216a2, 0, 0, 0);
            return idx;
        }
    }
    error(2, 0, dcgettext(NULL, "JIT internal error: %d", 5));
}



// Function: Pexecute @ 0xd220
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    unsigned long long field_20;
    long long field_28;
} struct_0;

extern unsigned int g_420940[4];
extern char g_42a180;
extern char g_42a684;

unsigned long long Pexecute(struct_0 *a0, void* a1, unsigned long a2, unsigned long long *a3, void* a4)
{
    void* v5;  // rbx
    unsigned long v6;  // r14
    long long v15;  // r9
    long long v16;  // rax
    long long v17;  // rdx
    void* *v18;  // rax
    int v19;  // r8d
    long long v20;  // rdx
    void* *v21;  // rax
    void* v22;  // r13
    void* v23;  // rbp
    int v24;  // r8d
    unsigned int v7;  // r14d
    void* v25;  // rax
    void* i;  // r12
    char v9;  // r13b
    long long v10;  // rdi
    long long v11;  // rsi
    long long v12;  // rdx
    long long v13;  // rcx
    void* ptr;  // [bp-0x78], Other Possible Types: int
    void* *v1;  // [bp-0x70]
    long long v2;  // [bp-0x68]
    void* v3;  // [bp-0x60]

    if (a4)
        v5 = a4;
    else
        v5 = a1;
    v7 = _INSERT(v6, 0, *((char *)v5 - 1) == g_42a684);
    v3 = a1;
    v1 = pcre2_get_ovector_pointer_8();
    while (1)
    {
        i = v3;
        while (1)
        {
            for (ptr = rawmemchr(v5, g_42a684); (&g_42a180)[3 + (sub_40c9d0(*((char *)v5)) & 0xff)] == 0xff; i = v5 + 1)
            {
                v7 = 0;
            }
            if (v5 == ptr)
            {
                v21 = v1;
                v22 = v5 - i;
                v23 = i;
                v21[1] = v22;
                *(v21) = v22;
                goto LABEL_40d41b;
            }
            v9 = (char)v7 ^ 1;
            if (!(char)sub_40cef0(v10, v11, v12, v13, sub_40cd90(a0, i, ptr - i, v22, v9) & 0xffffffff, v15))
            {
                v23 = i;
LABEL_40d41b:
                v19 = v24;
                if (v19 != -0x1)
                {
LABEL_40d39e:
                    if (v19 > 0)
                    {
                        v25 = ptr + 1;
                        if (a4)
                        {
                            v3 = v23 + *(v1);
                            v25 = v23 + v1[1];
                        }
                        *(a3) = v25 - v3;
                        return v3 - a1;
                    }
                    if (v19 < -0x2d && v19 >= -0x3f && v19 + 63 <= 0x11)
                        goto *((void *)((long long)(g_420940[63 + v19] + (char *)&g_420940[0])));
                    ptr = v19;
                    sub_408a50();
                    error(2, 0, dcgettext(NULL, "%s: internal PCRE error: %d", 5));
                }
                else
                {
                    v7 = 1;
                    v5 = ptr + 1;
                    if (v5 >= a1 + a2)
                        return 0xffffffffffffffff;
                    v3 = v5;
                    break;
                }
            }
            v16 = pcre2_get_startchar_8(a0->field_18);
            v17 = v16;
            if (v22 > v16)
            {
                i = i + v16 + 1;
                continue;
            }
            if (!v16)
            {
                v18 = v1;
                v19 = *((int *)((char *)a0 + 4 * (v7 & 1) + 48));
                *(v18) = 0;
                v18[1] = 0;
                if (v19 != -0x1)
                    goto LABEL_40d39b;
            }
            else
            {
                v2 = v16;
                v20 = v2;
                v19 = sub_40cd90(a0, i, v17, v22, v9 | 1073741826);
                v17 = v20;
                if (v19 != -0x1)
                {
LABEL_40d39b:
                    v23 = i;
                    goto LABEL_40d39e;
                }
            }
            v7 = 0;
            i = i + v17 + 1;
        }
    }
}



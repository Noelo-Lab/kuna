// Function: main @ 0x3f80
typedef struct sigaction {
} sigaction;

typedef struct struct_2 {
    struct struct_0 *field_0;
    long long field_8;
} struct_2;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

typedef struct struct_5 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[25];
    char field_39;
} struct_5;

typedef struct struct_4 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
    char field_31;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
    char padding_39[7];
    long long field_40;
} struct_4;

typedef struct lconv {
    char * decimal_point;
    char * thousands_sep;
    char * grouping;
    char * int_curr_symbol;
    char * currency_symbol;
    char * mon_decimal_point;
    char * mon_thousands_sep;
    char * mon_grouping;
    char * positive_sign;
    char * negative_sign;
    char int_frac_digits;
    char frac_digits;
    char p_cs_precedes;
    char p_sep_by_space;
    char n_cs_precedes;
    char n_sep_by_space;
    char p_sign_posn;
    char n_sign_posn;
    char int_p_cs_precedes;
    char int_p_sep_by_space;
    char int_n_cs_precedes;
    char int_n_sep_by_space;
    char int_p_sign_posn;
    char int_n_sign_posn;
} lconv;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern int g_415324;
extern int g_41534c;
extern char g_416da2;
extern long long g_41d540;
extern option g_41d5a0;
extern char g_41e010;
extern unsigned int g_41e024;
extern unsigned int g_41e028;
extern char g_41e038;
extern long long g_41e120;
extern long long g_41e128;
extern unsigned int optind;
extern long long stdin;
extern struct_0 *optarg;
extern unsigned long long stderr;
extern long long stdout;
extern uint128_t g_41e380;
extern uint128_t g_41e390;
extern uint128_t g_41e3a0;
extern uint128_t g_41e3b0;
extern uint128_t g_41e3c0;
extern uint128_t g_41e3d0;
extern uint128_t g_41e3e0;
extern uint128_t g_41e3f0;
extern char g_41e400;
extern char *g_41e408;
extern long long g_41e410;
extern char g_41e418;
extern char g_41e419;
extern char g_41e41a;
extern char g_41e41b;
extern unsigned long long g_41e428;
extern unsigned long long g_41e438;
extern char g_41e860;
extern char g_41e861;
extern char g_41e862;
extern unsigned int g_41e864;
extern char g_41e868;

unsigned int main(int a0, struct_0 **a1)
{
    long long v45;  // rdi
    long long v46;  // rsi
    int v55;  // ebp
    int v56;  // r13d
    long long node;  // r15
    unsigned int v58;  // r13d
    unsigned int v59;  // ecx
    struct_2 *ptr;  // rbp
    unsigned long idx;  // rax
    char *v62;  // rdx
    unsigned long v63;  // rax
    struct_0 *v64;  // rdx
    struct_4 *idx1;  // rdx
    struct_0 *v65;  // rdx
    unsigned int v66;  // esi
    unsigned int v67;  // eax
    struct_0 *v68;  // rdx
    struct_0 *v69;  // rdx
    unsigned long long *idx2;  // r8
    unsigned int v71;  // ecx
    char *v72;  // rdi
    unsigned long v73;  // rax
    unsigned long v74;  // rax
    long long v48;  // rcx
    char *v75;  // rax
    unsigned long long *v76;  // r8
    unsigned int v77;  // ecx
    unsigned long long *v78;  // r8
    unsigned int v79;  // ecx
    char *v80;  // rdi
    unsigned long long v81;  // rax
    struct_0 *v82;  // rdx
    unsigned int v83;  // r9d
    char v84;  // al
    long long v49;  // r8
    struct_5 *v85;  // r8
    char *v86;  // rdi
    unsigned int v87;  // r9d
    struct_5 *v88;  // r8
    struct_5 *v89;  // r8
    struct_5 *v90;  // r8
    unsigned long v91;  // rax
    char *v92;  // rax
    struct_5 *v93;  // r8
    unsigned int v94;  // ecx
    long long v50;  // r9
    unsigned long v95;  // rax
    char *v97;  // rax
    long long v98;  // r12
    char *v99;  // r14
    long long v100;  // rdx
    unsigned int v101;  // ebx
    unsigned int v102;  // eax
    char v103;  // r12b
    lconv *v51;  // rax
    unsigned long v106;  // rax
    unsigned long long v107;  // rdx
    unsigned long long v108;  // rax
    unsigned long long v111;  // r8
    unsigned long long v112;  // rax
    void* iter;  // rax
    struct_2 *i;  // rbp
    char *v52;  // rcx
    struct_2 *v115;  // rbp
    char *v53;  // rdx
    int *iter1;  // r15
    unsigned long long v0;  // [bp-0x308]
    struct_4 *v1;  // [bp-0x300]
    char *v2;  // [bp-0x2f8], Other Possible Types: sigaction *
    char *iter2;  // [bp-0x2f0], Other Possible Types: int *, struct_0 *
    char *v4;  // [bp-0x2e8]
    char *v5;  // [bp-0x2e0], Other Possible Types: struct_0 *
    unsigned long long v6;  // [bp-0x2d8]
    struct_0 *v7;  // [bp-0x2d0], Other Possible Types: struct_5 *, unsigned long long *, unsigned long, unsigned int
    char *v8;  // [bp-0x2c8]
    char *v9;  // [bp-0x2c0]
    struct_5 *v10;  // [bp-0x2b8], Other Possible Types: unsigned int
    char v11;  // [bp-0x2af]
    char v12;  // [bp-0x2ae]
    char v13;  // [bp-0x2ae]
    char v14;  // [bp-0x2ae]
    char v15;  // [bp-0x2ad]
    unsigned int v16;  // [bp-0x2ac]
    char v17;  // [bp-0x2a8]
    char v18;  // [bp-0x258], Other Possible Types: unsigned long long
    unsigned long v19;  // [bp-0x238]
    unsigned long v20;  // [bp-0x230]
    char v21;  // [bp-0x228]
    char v22;  // [bp-0x227]
    char v23;  // [bp-0x226]
    char v24;  // [bp-0x225]
    char v25;  // [bp-0x224]
    char v26;  // [bp-0x223]
    char v27;  // [bp-0x222]
    char v28;  // [bp-0x221]
    char v29;  // [bp-0x220]
    long long j;  // [bp-0x208]
    struct_2 *v31;  // [bp-0x200]
    sigaction v32;  // [bp-0xe8], Other Possible Types: unsigned long
    int v33;  // [bp-0xe0]
    int v34;  // [bp-0xd0]
    int v35;  // [bp-0xc0]
    int v36;  // [bp-0xb0]
    int v37;  // [bp-0xa0]
    int v38;  // [bp-0x90]
    int v39;  // [bp-0x80]
    int v40;  // [bp-0x70]
    unsigned int v41;  // [bp-0x60]
    char v42;  // [bp-0x42]
    char v43;  // [bp-0x41]

    v9 = getenv("POSIXLY_CORRECT");
    v11 = v9;
    v12 = 696 < sub_40ee70(v45, v46, idx1, v48, v49, v50) - 200112;
    sub_40ef00(*(a1));
    v8 = setlocale(6, &g_416da2);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_408f60(3);
    g_41e861 = sub_40cfb0(v45, v46, idx1, v48, v49, v50);
    g_41e860 = sub_40cfb0(2);
    v51 = localeconv();
    v52 = v51->decimal_point;
    g_41e868 = *(v52);
    if (!g_41e868 || v52[1])
        g_41e868 = 46;
    v53 = v51->thousands_sep;
    g_41e864 = *(v53);
    if (g_41e864)
    {
        if (!v53[1])
            goto LABEL_40407e;
        g_41e862 = 1;
    }
    g_41e864 = 128;
LABEL_40407e:
    g_41e418 = 0;
    sub_405b80();
    iter1 = &g_415324;
    sigemptyset(&g_41e380);
    iter2 = &g_415324;
    v55 = 14;
    v2 = &v32;
    while (1)
    {
        sigaction(v55, NULL, &v32);
        if (v32 != 1)
            sigaddset(&g_41e380, v55);
        if (iter1 == &g_41534c)
            break;
        v55 = *(iter1);
        iter1 += 1;
    }
    v41 = 0;
    v32 = sub_405b40;
    v56 = 14;
    v33 = (int)g_41e380;
    v34 = (int)g_41e390;
    v35 = (int)g_41e3a0;
    v36 = (int)g_41e3b0;
    v37 = (int)g_41e3c0;
    v38 = (int)g_41e3d0;
    v39 = (int)g_41e3e0;
    v40 = (int)g_41e3f0;
    while (1)
    {
        if (sigismember(&g_41e380, v56))
        {
            sigaction(v56, v2, NULL);
            if (iter2 == &g_41534c)
                break;
        }
        else if (iter2 == &g_41534c)
        {
            break;
        }
        v56 = *(iter2);
        iter2 += 1;
    }
    node = 0;
    v58 = 0;
    signal(0x11, NULL);
    sub_414980(sub_405ad0);
    v4 = &v18;
    sub_405cf0(&v18, 8);
    v18 = 0xffffffffffffffff;
    v2 = NULL;
    v59 = 0;
    iter2 = NULL;
    ptr = sub_412fd0(a0);
    v6 = 0;
    v5 = NULL;
    v15 = 0;
    while (1)
    {
        *((unsigned int *)&j) = 0xffffffff;
        if (v59 != 0xffffffff)
            break;
LABEL_4042de:
        idx = optind;
LABEL_4042b0:
        if ((unsigned int)idx < a0)
        {
            v95 = a1[idx];
            optind = (unsigned int)idx + 1;
            (&ptr->field_0)[node] = v95;
            node += 1;
        }
        else
        {
            if (v2)
            {
                if (!node)
                {
                    v98 = sub_406010(v2, "r");
                    sub_411ee0(&j);
                    if (!(char)sub_411f90(v98, &j))
                    {
                        sub_411170(4);
                        error(2, 0, dcgettext(NULL, "cannot read file names from %s", 5));
                    }
                    sub_406040(v98);
                    if (!j)
                    {
                        sub_411170(4, v2);
                        error(2, 0, dcgettext(NULL, "no input from %s", 5));
                    }
                    free(ptr);
                    ptr = v31;
                    if (j)
                    {
                        do
                        {
                            v99 = &(&ptr->field_0)[node]->field_0;
                            if (!strcmp(v99, "-"))
                            {
                                sub_411170(4, v99);
                                error(2, 0, dcgettext(NULL, "when reading file names from stdin, no file name of %s allowed", 5));
                            }
                            node += 1;
                            if (!*(v99))
                            {
                                sub_411270(0, 3, v2);
                                error(2, 0, dcgettext(NULL, "%s:%lu: invalid zero-length file name", 5));
                            }
                        } while (j != node);
                    }
                    goto LABEL_404466;
                }
                sub_411170(4, ptr->field_0);
                error(0, 0, dcgettext(NULL, "extra operand %s", 5));
                v97 = dcgettext(NULL, "file operands cannot be combined with --files0-from", 5);
                __fprintf_chk(stderr, 0x1, "%s\n", v97);
                goto LABEL_404db0;
            }
            goto LABEL_404466;
        }
    }
    if (node && v11)
    {
        idx = optind;
        if (v12 != 1 || (char)v58)
            goto LABEL_4042b0;
        if (optind == a0 && !v2)
        {
            v58 = 0;
            goto LABEL_404466;
        }
        v62 = &a1[(unsigned int)idx]->field_0;
        if (*(v62) != 45 || v62[1] != 111 || !v62[2] && optind + 1 == a0)
            goto LABEL_4042b0;
    }
    v63 = getopt_long(a0, a1, "-bcCdfghik:mMno:rRsS:t:T:uVy:z", &g_41d5a0.name, &j);
    v59 = v63;
    if ((unsigned int)v63 == 0xffffffff)
        goto LABEL_4042de;
    if ((unsigned int)v63 > 135)
        goto LABEL_404db0;
    if ((unsigned int)v63 > 0)
    {
        switch ((unsigned int)v63)
        {
        case 1:
            v82 = optarg;
            v13 = v12;
            if (optarg->field_0 == 43)
            {
                if (optind != a0)
                {
                    v83 = 0;
                    if (a1[optind]->field_0 == 45)
                        v83 = a1[optind]->field_1 - 48 <= 9;
                }
                else
                {
                    v83 = 0;
                }
                v84 = !v9 & (char)v83;
                v14 = v12 | v84;
                v13 = v14;
                if (v12 || v84)
                {
                    v16 = v59;
                    v10 = v83;
                    v7 = sub_405cf0(&v17);
                    v85 = v7;
                    v59 = v16;
                    v86 = sub_405d20(&idx1->padding_0[1], v7, 0);
                    if (v86)
                    {
                        v87 = v10;
                        if (*(v86) != 46)
                        {
                            v88 = v85;
                            if (v88->field_0 || !(v88 = v85, !v88->field_8))
                                goto LABEL_404dcb;
                        }
                        else
                        {
                            v89 = v7;
                            v87 = v10;
                            v59 = v16;
                            v86 = sub_405d20(v86 + 1, &v85->field_8, 0);
                            v88 = v89;
                            if (v88->field_0 || (v85 = v89, v88 = v89, v85->field_8))
                                goto LABEL_404be1;
                        }
                        v85->field_0 = 0xffffffffffffffff;
                        v88 = v85;
LABEL_404be1:
                        if (v86)
                        {
LABEL_404dcb:
                            v16 = v59;
                            v10 = v87;
                            v7 = v88;
                            v90 = v7;
                            v59 = v16;
                            if (!*((char *)sub_4057c0()))
                            {
                                if (v10)
                                {
                                    v16 = v59;
                                    v10 = v90;
                                    v91 = a1[optind];
                                    optind = optind + 1;
                                    v7 = v91;
                                    v92 = sub_405d20(v7 + 1, &v90->field_10, "invalid number after '-'");
                                    v93 = v10;
                                    v94 = v16;
                                    if (!v92)
                                        __assert_fail(); /* do not return */
                                    if (*(v92) == 46)
                                    {
                                        v16 = v94;
                                        v10 = v93;
                                        sub_405d20(v92 + 1, &v93->field_18, "invalid number after '.'");
                                        v94 = v16;
                                        v93 = v10;
                                    }
                                    if (!v93->field_18 && v93->field_10)
                                        v93->field_10 = v93->field_10 - 1;
                                    v16 = v94;
                                    v10 = v93;
                                    v90 = v10;
                                    v59 = v16;
                                    if (*((char *)sub_4057c0()))
                                        sub_405a10(v7, "stray character in field spec"); /* do not return */
                                }
                                v90->field_39 = 1;
                                v7 = v59;
                                sub_4060c0(v90);
                                v59 = v7;
                                v12 = v14;
                                continue;
                            }
                        }
                    }
                    else if (!v85->field_0 && !v85->field_8)
                    {
                        v85->field_0 = 0xffffffffffffffff;
                    }
                    v82 = optarg;
                    v13 = v14;
                }
            }
            v12 = v13;
            (&ptr->field_0)[node] = v82;
            node += 1;
            continue;
        case 83:
            v7 = v59;
            sub_4089b0((unsigned int)j, optarg);
            v59 = v7;
            continue;
        case 84:
            v7 = v59;
            sub_4061e0(optarg);
            v59 = v7;
            continue;
        case 107:
            v10 = v59;
            v7 = sub_405cf0(&v17);
            idx2 = v7;
            v71 = v10;
            v72 = sub_405d20(optarg, v7);
            v73 = *(idx2);
            *(idx2) = v73 - 1;
            if (!v73)
            {
LABEL_4052ce:
                sub_405a10(optarg, "field number is zero"); /* do not return */
            }
            if (*(v72) == 46)
            {
                v10 = v71;
                v7 = idx2;
                sub_405d20(v72 + 1, idx2 + 1, "invalid number after '.'");
                idx2 = v7;
                v71 = v10;
                v74 = idx2[1];
                idx2[1] = v74 - 1;
                if (!v74)
                    sub_405a10(optarg, "character offset is zero"); /* do not return */
            }
            if (!*(idx2) && !idx2[1])
                *(idx2) = 0xffffffffffffffff;
            v10 = v71;
            v7 = idx2;
            v75 = sub_4057c0();
            v76 = v7;
            v77 = v10;
            if (*(v75) != 44)
            {
                v76[2] = 0xffffffffffffffff;
                v76[3] = 0;
            }
            else
            {
                v10 = v77;
                v7 = v76;
                v78 = v7;
                v79 = v10;
                v80 = sub_405d20(v75 + 1, v76 + 2, "invalid number after ','");
                v81 = v78[2];
                v78[2] = v81 - 1;
                if (!v81)
                    goto LABEL_4052ce;
                if (*(v80) == 46)
                {
                    v10 = v79;
                    v7 = v78;
                    sub_405d20(v80 + 1, v78 + 3, "invalid number after '.'");
                    v79 = v10;
                    v78 = v7;
                }
                v10 = v79;
                v7 = v78;
                v75 = sub_4057c0();
                v77 = v10;
                v76 = v7;
            }
            if (*(v75))
                sub_405a10(optarg, "stray character in field spec"); /* do not return */
            v7 = v77;
            sub_4060c0(v76);
            v59 = v7;
            break;
        case 109:
            v15 = 1;
            continue;
        case 111:
            v68 = optarg;
            if (iter2)
            {
                v7 = v59;
                iter2 = optarg;
                v68 = iter2;
                v59 = v7;
                if (strcmp(iter2, optarg))
                    error(2, 0, dcgettext(NULL, "multiple output files specified", 5));
            }
            iter2 = &v68->field_0;
            continue;
        case 115:
            g_41e41a = 1;
            continue;
        case 116:
            v67 = optarg->field_0;
            if (!(char)v67)
                error(2, 0, dcgettext(NULL, "empty tab", 5));
            if (optarg->field_1)
            {
                v10 = v59;
                v7 = optarg;
                v59 = v10;
                if (strcmp(optarg, "\\0"))
                {
                    sub_411430(v7);
                    error(2, 0, dcgettext(NULL, "multi-character tab %s", 5));
                }
                v67 = 0;
            }
            if (g_41e028 != 128 && g_41e028 != v67)
                error(2, 0, dcgettext(NULL, "incompatible tabs", 5));
            g_41e028 = v67;
            break;
        case 117:
            g_41e419 = 1;
            continue;
        case 121:
            v65 = optarg;
            if (*((long long *)((char *)&a1[optind] - 8)) == optarg)
            {
                while (1)
                {
                    if (v65->field_0 - 48 > 9)
                        break;
                    v65 = &v65->field_1;
                }
                optind = optind - 1 + ((char)v66 < 1);
                break;
            }
            break;
        case 122:
            g_41e038 = 0;
            continue;
        case 129:
            v64 = optarg;
            if (g_41e408)
            {
                v10 = v59;
                v7 = optarg;
                v64 = v7;
                v59 = v10;
                if (strcmp(g_41e408, optarg))
                    error(2, 0, dcgettext(NULL, "multiple compress programs specified", 5));
            }
            g_41e408 = &v64->field_0;
            continue;
        case 130:
            g_41e400 = 1;
            continue;
        case 131:
            v2 = &optarg->field_0;
            continue;
        case 132:
            v7 = v59;
            sub_408b60((unsigned int)j, optarg);
            v59 = v7;
            continue;
        case 133:
            v69 = optarg;
            if (v5)
            {
                v7 = v59;
                v5 = optarg;
                v69 = v5;
                v59 = v7;
                if (strcmp(v5, optarg))
                    error(2, 0, dcgettext(NULL, "multiple random sources specified", 5));
            }
            v5 = &v69->field_0;
            continue;
        case 135:
            v7 = v59;
            v59 = v7;
            v6 = sub_408900((unsigned int)j, optarg);
            continue;
        case 128:
            v59 = 99;
            if (optarg)
            {
                v1 = idx1;
                v0 = 1;
                v59 = *((char *)(sub_40c4b0("--check") + 4281446));
                goto LABEL_404420;
            }
            break;
        case 67: case 99:
LABEL_404420:
            if ((char)v58 && v58 != v59)
                sub_405940("cC"); /* do not return */
            v58 = (char)v59;
            continue;
        case 134:
            v59 = *((char *)(sub_40c4b0("--sort", optarg, &g_41d540, "ghMnRVCCc", 1, g_41e128, 1, v49) + 4281440));
        case 77: case 82: case 86: case 98: case 100: case 102: case 103: case 104: case 105: case 110: case 114:
            v7 = v59;
            v42 = v59;
            v43 = 0;
            sub_4057c0(&v42, v4, 2);
            v59 = v7;
            continue;
        default:
LABEL_404db0:
            sub_40be50(2); /* do not return */
        }
    }
    else
    {
        if ((unsigned int)v63 == 0xffffff7d)
        {
            sub_412d40(stdout, "sort", "GNU coreutils", g_41e120, "Mike Haertel", "Paul Eggert", 0, v63);
            exit(0); /* do not return */
        }
        else if ((unsigned int)v63 == 0xffffff7e)
        {
            sub_40be50(0); /* do not return */
        }
        goto LABEL_404db0;
    }
LABEL_404466:
    v100 = g_41e410;
    if (g_41e410)
    {
        v101 = 0;
        while (1)
        {
            if ((char)sub_405680(v100) && !idx1->field_37)
            {
                idx1->field_20 = v19;
                idx1->field_28 = v20;
                idx1->field_30 = v21;
                idx1->field_31 = v22;
                idx1->field_36 = v27;
                idx1->field_32 = v23;
                idx1->field_34 = v25;
                idx1->field_35 = v26;
                idx1->field_37 = v28;
                idx1->field_38 = v29;
                v102 = v24;
                idx1->field_33 = v102;
                v100 = idx1->field_40;
                v101 |= v102;
                if (!v100)
                    break;
            }
            else
            {
                v100 = idx1->field_40;
                v101 |= idx1->field_33;
                if (!v100)
                    break;
            }
        }
        v103 = 0;
        goto LABEL_404532;
    }
    if ((char)sub_405680(v4))
    {
        sub_405970();
        if (g_41e400)
        {
            v101 = 0;
            v103 = 0;
LABEL_40461e:
            if ((char)v58)
            {
LABEL_40518b:
                g_41e018 = v58;
                sub_405940("X --debug"); /* do not return */
            }
            else if (!iter2)
            {
                if (!v8 || !setlocale(3, &g_416da2))
                {
                    dcgettext(NULL, "failed to set locale", 5);
                    error(0, 0, "%s");
                }
                if (g_41e861)
                {
                    sub_411430(setlocale(3, NULL));
                    error(0, 0, dcgettext(NULL, "text ordering performed using %s sorting rules", 5));
                }
                else
                {
                    dcgettext(NULL, "text ordering performed using simple byte comparison", 5);
                    error(0, 0, "%s");
                }
                sub_407d30(v4, v103);
LABEL_404544:
                g_41e41b = v28;
                if ((char)v101)
                    sub_406110(v5);
            }
            else if (!(char)v58)
            {
                v58 = 111;
                goto LABEL_40518b;
            }
        }
        else
        {
            g_41e41b = v28;
        }
        if (!g_41e428)
        {
            v106 = getenv("TMPDIR");
            if (!v106)
                v106 = "/tmp";
            sub_4061e0(v106);
        }
        if (!node)
        {
            node = 1;
            free(ptr);
            ptr = sub_412eb0(8);
            ptr->field_0 = "-";
        }
        v107 = g_41e438;
        if (v107)
        {
            v108 = g_41e024 * 0x22;
            if (v108 < v107)
                v108 = v107;
            g_41e438 = v108;
        }
        if ((char)v58)
        {
            if (node == 1)
            {
                if (iter2)
                {
                    g_41e010 = v58;
                    sub_405940(&g_41e010); /* do not return */
                }
                exit((char)sub_40aac0(ptr->field_0, (char)v58) ^ 1); /* do not return */
            }
            sub_411170(4, ptr->field_8);
            error(2, 0, dcgettext(NULL, "extra operand %s not allowed with -%c", 5));
        }
        else
        {
            sub_406c00(ptr, node);
            sub_406ca0(iter2);
            if (!v15)
            {
                if (!v6)
                {
                    v111 = sub_40e9d0(2);
                    v112 = 8;
                    if (v111 <= 8)
                        v112 = v111;
                    v6 = v112;
                }
                sub_40bac0();
            }
            else
            {
                iter = sub_413150(node, 16);
                i = ptr;
                do
                {
                    v115 = &i->field_8;
                    iter += 16;
                    *((struct struct_0 **)((char *)iter - 16)) = i->field_0;
                    i = v115;
                } while (i != &(&ptr->field_0)[node]);
                sub_40b720();
            }
            if (!g_41e418)
            {
                return 0;
            }
            else if (sub_40c900(stdin) != 0xffffffff)
            {
                return 0;
            }
            else
            {
                sub_405e10(dcgettext(NULL, "close failed", 5), "-"); /* do not return */
            }
        }
    }
    else
    {
        v103 = 1;
        sub_4060c0(v4);
        v101 = v24;
LABEL_404532:
        sub_405970();
        if (!g_41e400)
            goto LABEL_404544;
        goto LABEL_40461e;
    }
}



// Function: proctab_hasher @ 0x5400
void proctab_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: proctab_comparator @ 0x5420
void proctab_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: traverse_raw_number @ 0x5430
extern char g_41e864;
extern char g_41e868;

unsigned int traverse_raw_number(char **a0)
{
    char *v1;  // rax
    char *v2;  // rsi
    unsigned int v3;  // edx
    unsigned int v4;  // r8d
    char v5;  // r9b
    char *v6;  // rax
    unsigned int v7;  // ecx
    char *v8;  // rdx

    v1 = *(a0);
    v2 = v1 + 1;
    v3 = *(v1);
    if (v3 - 48 <= 9)
    {
        v4 = 0;
        do
        {
            v5 = 1;
            if ((char)v4 < (char)v3)
                v4 = v3;
            v6 = v1 + 2;
            if (v1[1] != *((int *)&g_41e864))
            {
                v5 = 0;
                v6 = v2;
            }
            v1 = v6;
            v2 = v1 + 1;
            v3 = *(v1);
        } while (v3 - 48 <= 9);
        if (v5)
        {
            *(a0) = v1 - 1;
            return v4;
        }
    }
    else
    {
        v4 = 0;
    }
    if (g_41e868 != (char)v3)
    {
        v2 -= 1;
    }
    else
    {
        v7 = *(v2);
        v8 = v2 + 1;
        if (v7 - 48 <= 9)
        {
            do
            {
                v2 = v8;
                if ((char)v4 < (char)v7)
                    v4 = v7;
                else
                    v4 = v4;
            } while ((v7 = (unsigned int)(int)(char)*(v2), v8 = v2 + 1, v7 - 48 <= 9));
        }
    }
    *(a0) = v2;
    return v4;
}



// Function: find_unit_order @ 0x54f0
extern char g_415360;

unsigned long long find_unit_order(char *a0)
{
    unsigned int v2;  // eax
    char v3;  // r11b
    char *v0;  // [bp-0x18]

    v0 = &a0[*(a0) == 45];
    if ((char)(sub_405430(&v0) & 0xffffffff) <= 48)
        return 0;
    v2 = (&g_415360)[*(v0)];
    return (v3 == 45 ? -(v2) : v2);
}



// Function: getmonth @ 0x5570
typedef struct struct_3 {
    char padding_0[1];
    char field_1;
} struct_3;

typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern struct_0 g_41e060;

unsigned int getmonth(char *i, struct_2 **a1)
{
    unsigned long v1;  // rdx
    char *v2;  // r9
    char *v11;  // rsi
    struct_3 *v12;  // rcx
    unsigned long long v13;  // r11
    unsigned long long v3;  // rbx
    unsigned long long v4;  // rbp
    char v5;  // r8b
    char v6;  // al
    char *v7;  // r10
    char v8;  // r14b
    char v9;  // al
    char *v10;  // rsi

    for (; *((char *)(v1 + (sub_4053e0(*(i)) & 0xff))); i = v2 + 1);
    v3 = 12;
    v4 = 0;
    while (*((char *)*((long long *)&(&g_41e060.field_0)[16 * (v4 + v3 >> 1)])))
    {
        v6 = v5;
        while (1)
        {
            v8 = sub_4053e0(v7[v6]);
            v9 = sub_4053e0((char)v1);
            if (v8 < v9)
            {
                v3 = v13;
                goto LABEL_40561a;
            }
            if (v8 > v9)
                break;
            v11 = v10 + 1;
            if (!v12->field_1)
                goto LABEL_405630;
            v6 = sub_4053e0(*(v11));
        }
        v4 = v13 + 1;
LABEL_40561a:
        if (v4 >= v3)
            return 0;
    }
    v11 = v2;
LABEL_405630:
    if (a1)
        *(a1) = v11;
    return *((int *)&(&g_41e060.field_0)[8 + 16 * v13]);
}



// Function: key_numeric @ 0x5660
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

long long key_numeric(struct_0 *a0)
{
    return _INSERT(0xffff00ff0000 & a0->field_30, 0, 0xffff00ff0000 & a0->field_30);
}



// Function: default_key_compare @ 0x5680
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

long long default_key_compare(struct_0 *a0)
{
    long long v1;  // rax
    long long v2;  // rax
    struct_0 *v3;  // rdi

    if (*((long long *)&a0->padding_0[32]))
    {
        return 0;
    }
    else if (*((long long *)&a0->padding_0[40]))
    {
        return 0;
    }
    else
    {
        v1 = (char)a0->field_30;
        if ((char)v1)
        {
            return 0;
        }
        else if (*((char *)&a0->field_30 + 1))
        {
            return v1;
        }
        else
        {
            v2 = sub_405660(a0);
            if ((char)v2)
            {
                return 0;
            }
            else if (*((char *)&v3->field_30 + 6))
            {
                return v2;
            }
            else if (v3[1].padding_0[0])
            {
                return v2;
            }
            else
            {
                return *((char *)&v3->field_30 + 3) ^ 1;
            }
        }
    }
}



// Function: key_to_opts @ 0x56d0
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned short field_30;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
} struct_0;

extern char g_41e540;
extern char g_41e640;

char * key_to_opts(struct_0 *a0, char *node)
{
    char *iter;  // rax

    if (a0->field_30)
    {
        *(node) = 98;
        node += 1;
    }
    if (a0->field_20 == &g_41e540)
    {
        *(node) = 100;
        node += 1;
    }
    iter = node;
    if (a0->field_28)
    {
        *(node) = 0x66;
        iter += 1;
    }
    if (a0->field_34)
    {
        *(iter) = 103;
        iter += 1;
    }
    if (a0->field_35)
    {
        *(iter) = 104;
        iter += 1;
    }
    if (a0->field_20 == &g_41e640)
    {
        *(iter) = 105;
        iter += 1;
    }
    if (a0->field_36)
    {
        *(iter) = 77;
        iter += 1;
    }
    if (a0->field_32)
    {
        *(iter) = 110;
        iter += 1;
    }
    if (a0->field_33)
    {
        *(iter) = 82;
        iter += 1;
    }
    if (a0->field_37)
    {
        *(iter) = 114;
        iter += 1;
    }
    if (a0->field_38)
    {
        *(iter) = 86;
        iter += 1;
    }
    *(iter) = 0;
    return iter;
}



// Function: compare_nodes @ 0x5780
typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[24];
    unsigned int field_50;
} struct_0;

char compare_nodes(struct_0 *a0, unsigned long long *a1)
{
    if (a0->field_50 == (int)a1[10])
        return a0->field_30 + a0->field_28 < a1[6] + a1[5];
    return a0->field_50 < (int)a1[10];
}



// Function: set_ordering @ 0x57c0
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    char padding_28[10];
    char field_32;
    char padding_33[1];
    char field_34;
    char field_35;
    char padding_36[1];
    char field_37;
} struct_0;

extern char g_41e440;
extern unsigned long long g_41e540;
extern unsigned long long g_41e640;

char * set_ordering(char *a0, struct_0 *idx, unsigned int a2)
{
    char v1;  // al
    char *v2;  // r8
    char *v3;  // r8

    v1 = *(a0);
    v2 = a0;
    if (v1)
    {
        do
        {
            switch (v1)
            {
            case 77:
                idx->padding_36[0] = 1;
                continue;
            case 82:
                idx->padding_33[0] = 1;
                continue;
            case 86:
                idx[1].padding_0[0] = 1;
                continue;
            case 98:
                if (!(a2 & 0xfffffffd))
                    idx->padding_28[8] = 1;
                if (a2 - 1 <= 1)
                {
                    idx->padding_28[9] = 1;
                    continue;
                }
                break;
            case 100:
                idx->field_20 = &g_41e540;
                continue;
            case 102:
                *((char **)&idx->padding_28[0]) = &g_41e440;
                continue;
            case 103:
                idx->field_34 = 1;
                continue;
            case 104:
                idx->field_35 = 1;
                continue;
            case 105:
                if (!idx->field_20)
                {
                    idx->field_20 = &g_41e640;
                    continue;
                }
                break;
            case 110:
                idx->field_32 = 1;
                continue;
            case 114:
                idx->field_37 = 1;
                continue;
            }
        } while ((v1 = v3[1], v2 = v3 + 1, v3[1]));
    }
    return v2;
}



// Function: struct_month_cmp @ 0x5930
int struct_month_cmp(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: incompatible_options @ 0x5940
void incompatible_options(unsigned long a0)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    error(2, 0, dcgettext(NULL, "options '-%s' are incompatible", 5));
}



// Function: check_ordering_compatibility @ 0x5970
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned short field_30;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char field_37;
    char field_38;
} struct_0;

extern struct_0 *g_41e410;

void check_ordering_compatibility(void)
{
    struct_0 *v2;  // rdi
    unsigned long long v3;  // rcx
    struct_0 *idx;  // rdi
    unsigned int v5;  // r8
    char v0[40];  // [bp-0x38]

    v2 = g_41e410;
    if (!g_41e410)
        return;
    while (1)
    {
        idx = v2;
        v3 = _INSERT(v3, 0, idx->field_20);
        if (idx->field_32 + idx->field_34 + idx->field_35 + idx->field_36 + (idx->field_38 | idx->field_33 | (char)v3) > 1)
        {
            idx->field_37 = 0;
            idx->field_30 = 0;
            sub_4056d0(idx, v0);
            sub_405940(v5); /* do not return */
        }
        v2 = *((long long *)&idx[1].padding_0[7]);
        if (!*((long long *)&idx[1].padding_0[7]))
            return;
    }
}



// Function: badfieldspec @ 0x5a10
void badfieldspec(unsigned long a0, char *a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    sub_411430();
    dcgettext(NULL, a1, 5);
    error(2, 0, dcgettext(NULL, "%s: invalid field specification %s", 5));
}



// Function: cs_enter @ 0x5a60
extern long long g_41e380;

void cs_enter(char *a0)
{
    *(a0) = !pthread_sigmask(0, &g_41e380, a0 + 8);
    return;
}



// Function: cs_leave @ 0x5a80
void cs_leave(char *a0)
{
    if (*(a0))
        pthread_sigmask(2, a0 + 8, 0);
    return;
}



// Function: cleanup @ 0x5aa0
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_41e368;

int cleanup(void)
{
    struct_0 **v1;  // rbx
    struct_0 **v2;  // rbx
    int v3;  // eax

    v1 = g_41e368;
    if (g_41e368)
    {
        do
        {
            v2 = v1;
            v3 = unlink((char *)&v2[1] + 5);
            v1 = *(v2);
        } while (*(v2));
    }
    g_41e368 = 0;
    return v3;
}



// Function: exit_cleanup @ 0x5ad0
extern int g_41e130;
extern long long stderr;
extern long long stdout;
extern unsigned long long g_41e368;
extern char g_41e870;
extern long long g_41e878;

void exit_cleanup(void)
{
    unsigned long v4;  // fs
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    int *err;  // rbx
    char v0[136];  // [bp-0x98]
    unsigned long v1;  // [bp-0x10], Other Possible Types: unsigned long long
    unsigned long long v2;  // [bp-0x8]

    v1 = *((long long *)(40 + v4));
    if (g_41e368)
    {
        sub_405a60(v0);
        sub_405aa0();
        sub_405a80(v0);
    }
    if (v1 != *((long long *)(40 + v4)))
        __stack_chk_fail(); /* do not return */
    v2 = v5;
    v1 = v6;
    if (!(!sub_413d00(stdout) || (err = __errno_location(), g_41e870 && *(err) == 32)))
    {
        dcgettext(NULL, "write error", 5);
        if (g_41e878)
        {
            sub_411250(g_41e878);
            error(0, *(err), "%s: %s");
        }
        else
        {
            error(0, *(err), "%s");
        }
    }
    else if (!sub_413d00(stderr))
    {
        return;
    }
    _exit(g_41e130); /* do not return */
}



// Function: sighandler @ 0x5b40
int sighandler(int a0)
{
    char v0;  // [bp+0x0]

    sub_405aa0();
    signal(a0, NULL);
    return raise((unsigned int)&v0);
}



// Function: inittables @ 0x5b80
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
} struct_0;

extern struct_0 g_41e060;
extern char g_41e440;
extern char g_41e540;
extern char g_41e640;
extern char g_41e740;
extern char g_41e860;

long long inittables(void)
{
    unsigned long long iter;  // rbx
    unsigned long long *v3;  // rax
    unsigned long long v12;  // rdi
    unsigned long v13;  // r8
    unsigned long v14;  // rcx
    char *v15;  // rdx
    char *v16;  // rsi
    unsigned int v4;  // edx
    char v5;  // al
    struct_0 *node;  // r14
    unsigned long long v7;  // r13
    char *v8;  // rax
    unsigned long len;  // rbx
    unsigned long long v10;  // rax
    char *v11;  // rdx
    unsigned long v0;  // [bp-0x38]

    iter = 0;
    v3 = __ctype_b_loc();
    do
    {
        (&g_41e740)[iter] = sub_405b60(iter & 0xffffffff);
        v4 = *((short *)(*(v3) + iter * 2));
        (&g_41e640)[iter] = ((char)((unsigned short)v4 >> 14) ^ 1) & 1;
        v5 = 0;
        if (!((char)v4 & 8))
            v5 = (char)sub_405b60(iter & 0xffffffff) ^ 1;
    } while (((&g_41e540)[iter] = v5, (&g_41e440)[iter] = (char)toupper((unsigned int)iter), iter += 1, iter != 0x100));
    if (!g_41e860)
        return v0;
    node = &g_41e060.field_0;
    v7 = 1;
    do
    {
        v8 = nl_langinfo((unsigned int)v7 + 131085);
        len = strlen(v8);
        v10 = sub_412eb0(len + 1);
        node->field_8 = v7;
        node->field_0 = v10;
        if (len)
        {
            v11 = v8;
            len = 0;
            do
            {
                v12 = sub_4053e0(*(v11)) & 0xff;
                if (!(*((char *)(v13 + v12 * 2)) & 1))
                {
                    *((char *)(v14 + len)) = (&g_41e440)[v12];
                    len += 1;
                }
            } while ((v11 = v15 + 1, v16 != v11));
        }
    } while ((v7 += 1, *((char *)(v14 + len)) = 0, node += 16, v7 != 13));
    return (unsigned long long)qsort(&g_41e060.field_0, 12, 16, sub_405930);
}



// Function: key_init @ 0x5cf0
void* key_init(void* idx)
{
    *((unsigned long long *)&idx[64]) = 0;
    *((uint128_t *)&idx[16]) = (uint128_t)0;
    *((uint128_t *)idx) = 0;
    *((unsigned long *)&idx[16]) = 0xffffffffffffffff;
    *((uint128_t *)&idx[32]) = 0;
    *((uint128_t *)&idx[48]) = 0;
    return idx;
}



// Function: parse_field_count @ 0x5d20

unsigned long long parse_field_count(long long a0, unsigned long long *a1, char *a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]

    sub_413650();
    switch ((unsigned int)sub_413650())
    {
    case 0: case 2:
        *(a1) = v1;
        return v0;
    case 1: case 3:
        *(a1) = 0xffffffffffffffff;
        return v0;
    case 4:
        if (!a2)
            return 0;
        sub_411430(a0);
        dcgettext(NULL, a2, 5);
        error(2, 0, dcgettext(NULL, "%s: invalid count at start of %s", 5));
    default:
        return v0;
    }
}



// Function: sort_die @ 0x5e10
void sort_die(unsigned long a0, unsigned long long a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    int *err;  // rax
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    if (!a1)
        dcgettext(NULL, "standard output", 5);
    sub_411270(0, 3);
    err = __errno_location();
    error(2, *(err), "%s: %s");
}



// Function: get_outstatus @ 0x5e70
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

extern stat g_41e260;
extern int g_41e2f0;

stat * get_outstatus(void)
{
    if (g_41e2f0)
        return NULL;
    if (!fstat(1, &g_41e260.st_mode))
    {
        g_41e2f0 = 0xffffffff;
    }
    else
    {
        g_41e2f0 = (unsigned int)__errno_location();
        if (g_41e2f0 >= 0)
            return NULL;
    }
    return &g_41e260.st_mode;
}



// Function: stream_open @ 0x5ee0
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

extern long long stdin;
extern unsigned long long stdout;
extern char g_41e418;

long long stream_open(unsigned long a0, char *a1)
{
    int *err;  // rax
    stat *v2;  // rax
    char *v3;  // rax
    unsigned int v4;  // edx
    unsigned int v5;  // eax
    unsigned long fp;  // r13

    if (*(a1) != 114)
    {
        if (*(a1) == 0x77)
        {
            if (!a0 || !ftruncate(1, 0))
                return stdout;
            err = __errno_location();
            v2 = sub_405e70();
            if (!v2 || ((unsigned short)v2->st_nlink & 0xf000) == 0x8000)
            {
                sub_411270(0, 3, a0);
                v3 = dcgettext(NULL, "%s: error truncating", 5);
                error(2, *(err), v3);
            }
            return stdout;
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    else
    {
        if (!strcmp(a0, "-"))
        {
            g_41e418 = 1;
            sub_40c8d0(stdin, 2);
            return stdin;
        }
        v5 = open(a0, 0x80000, v4);
        if (v5 >= 0)
        {
            fp = fdopen(v5, a1);
            sub_40c8d0(fp, 2);
            return fp;
        }
        sub_40c8d0(0, 2);
        return 0;
    }
}



// Function: xfopen @ 0x6010
long long xfopen(unsigned long long a0, char *a1)
{
    long long v1;  // rax

    v1 = sub_405ee0(a0, a1);
    if (!v1)
        sub_405e10(dcgettext(NULL, "open failed", 5), a0); /* do not return */
    return v1;
}



// Function: xfclose @ 0x6040
int xfclose(unsigned long a0, unsigned long long a1)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    char *v3;  // rsi
    unsigned int v4;  // eax

    v1 = fileno(a0);
    if (!v1)
        return (unsigned long long)clearerr_unlocked(a0);
    if (v1 != 1)
    {
        v2 = sub_40c900(a0);
        if (!v2)
            return v2;
        v3 = "close failed";
    }
    else
    {
        v4 = fflush_unlocked(a0);
        if (!v4)
            return v4;
        v3 = "fflush failed";
    }
    sub_405e10(dcgettext(NULL, v3, 5), a1); /* do not return */
}



// Function: insertkey @ 0x60c0
typedef struct struct_0 {
    char padding_0[64];
    struct struct_0 *field_40;
} struct_0;

extern struct_0 *g_41e410;

void insertkey(void)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // rdx
    struct_0 **v3;  // rcx
    struct_0 *v4;  // rdx
    struct_0 *v5;  // rdx

    v1 = sub_4131b0();
    v2 = g_41e410;
    v3 = &g_41e410;
    if (g_41e410)
    {
        do
        {
            v4 = v2;
            v5 = v4->field_40;
            v2 = v5;
        } while (v4->field_40);
        v3 = &v4->field_40;
    }
    *(v3) = v1;
    v1->field_40 = 0;
    return;
}



// Function: random_md5_state_init @ 0x6110
extern long long g_41e300;

void random_md5_state_init(unsigned long long a0)
{
    unsigned long long v2;  // r12
    long long v3;  // rax
    char *v4;  // rsi
    char v0;  // [bp-0x38]

    v2 = a0;
    v3 = sub_411710();
    if (v3)
    {
        sub_411830(v3, &v0, 16);
        if (!sub_411850(v3))
        {
            sub_40c680(&g_41e300);
            sub_40c690(&v0, 16, &g_41e300);
            return;
        }
        v4 = "close failed";
    }
    else
    {
        if (!v2)
            v2 = "getrandom";
        v4 = "open failed";
    }
    sub_405e10(dcgettext(NULL, v4, 5), v2); /* do not return */
}



// Function: add_temp_dir @ 0x61e0
extern long long g_41e420;
extern unsigned long long g_41e428;
extern unsigned long long g_41e430;

long long add_temp_dir(unsigned long a0)
{
    unsigned long idx;  // rdx
    unsigned long v2;  // rdi

    idx = g_41e428;
    v2 = g_41e430;
    if (g_41e428 == g_41e420)
    {
        v2 = sub_412ff0(v2, &g_41e420, 8);
        idx = g_41e428;
        g_41e430 = v2;
    }
    *((unsigned long *)(v2 + idx * 8)) = a0;
    g_41e428 = idx + 1;
    return idx + 1;
}



// Function: initbuf @ 0x6240
typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
} struct_0;

void* initbuf(struct_0 *idx, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v1;  // rbp
    unsigned long v2;  // rbp
    void* ptr;  // rax

    v1 = a2;
    while (1)
    {
        v2 = (v1 & 0xffffffffffffffe0) + 32;
        ptr = malloc(v2);
        idx->field_0 = ptr;
        if (ptr)
        {
            idx->field_28 = a1;
            *((unsigned long *)&idx->field_18) = v2;
            idx->field_10 = 0;
            idx->field_20 = 0;
            idx->field_8 = 0;
            idx->field_30 = 0;
            return ptr;
        }
        v1 = v2 >> 1;
        if (a1 + 1 >= v1)
            sub_413270(); /* do not return */
    }
}



// Function: debug_width @ 0x62b0
long long debug_width(void* a0, void* i)
{
    void* v1;  // rbx
    unsigned long v2;  // rax

    v1 = a0;
    v2 = sub_40e580();
    if (i <= v1)
        return v2;
    do
    {
        v1 += 1;
        v2 += *((char *)v1 - 1) == 9;
    } while (i != v1);
    return v2;
}



// Function: nan_compare @ 0x6370
int nan_compare(char *a0, char *a1)
{
    char v0;  // [bp-0x38]
    char v1;  // [bp-0x28]

    strtold(a0, NULL);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_58{s-56|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_58{s-56|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    strtold(a1, NULL);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_59{s-40|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_59{s-40|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    return memcmp(&v0, &v1, 16);
}



// Function: general_numcompare @ 0x63d0
unsigned int general_numcompare(char *a0, char *a1)
{
    unsigned long v3;  // cc_dep1
    char *v0;  // [bp-0x30]
    char *v1;  // [bp-0x28]

    strtold(a0, &v0);
    if (/* unsupported instruction */)
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_145{s-72|1b}), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
        /* unsupported instruction */
    }
    else
    {
        amd64g_dirtyhelper_storeF80le((Reference vvar_145{s-72|1b}), Reinterpret(F64->I64, nan<64>))
        /* unsupported instruction */
    }
    strtold(a1, &v1);
    if (/* unsupported instruction */)
    {
        /* unsupported instruction */
        /* unsupported instruction */
    }
    else
    {
        /* unsupported instruction */
        /* unsupported instruction */
    }
    if (v0 == a0)
    {
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        return -(v1 != a1);
    }
    else if (v1 != a1)
    {
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        if (((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69 | (char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 6)) & 1) == 1)
        {
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            if (((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69 | (char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 6)) & 1) != 1)
            {
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                return 1;
            }
            v3 = CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69;
            if (!((char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2) & 1))
            {
                if ((char)v3 & 64)
                {
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                    return 0;
                }
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
            }
            else
            {
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
                /* unsupported instruction */
            }
            /* unsupported instruction */
            if ((char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2) & 1)
            {
                /* unsupported instruction */
                if (!((char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2) & 1))
                    return 1;
                return sub_406370(a0, a1);
            }
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
        }
        else
        {
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
            /* unsupported instruction */
        }
        return 0xffffffff;
    }
    else
    {
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        /* unsupported instruction */
        return 1;
    }
}



// Function: numcompare @ 0x64d0
extern char g_41e864;
extern char g_41e868;

unsigned long long numcompare(void* i, unsigned long a1)
{
    unsigned long v1;  // rdx
    void* v2;  // r8
    void* v11;  // rdi
    unsigned int l;  // r9d
    void* v14;  // r8
    unsigned int v15;  // esi
    void* v16;  // r8
    unsigned int v17;  // edx
    unsigned int v18;  // r9d
    unsigned long long iter;  // rcx
    unsigned int v20;  // edx
    void* v3;  // rsi
    unsigned long long v21;  // rsi
    unsigned int v22;  // edx
    char v23;  // dl
    char v24;  // cl
    unsigned int m;  // esi
    void* v26;  // rdi
    unsigned int v28;  // esi
    void* v29;  // r8
    void* iter1;  // rdi
    char v31;  // dl
    void* v32;  // r8
    unsigned int v33;  // r10d
    void* v34;  // rdi
    unsigned int v35;  // edx
    unsigned int v36;  // edx
    unsigned long long v37;  // rsi
    unsigned int v38;  // edx
    unsigned int v39;  // ecx
    unsigned int v40;  // edx
    char v5;  // cl
    unsigned long v41;  // rax
    unsigned int v42;  // r8d
    unsigned int v43;  // r10d
    void* node;  // r8
    char v7;  // dl
    unsigned int v8;  // edx
    void* v9;  // r8
    unsigned int v10;  // r9d

    for (; *((char *)(v1 + (sub_4053e0(*((char *)i)) & 0xff))); i = v2 + 1);
    while (*((char *)(v1 + (sub_4053e0(*((char *)v3)) & 0xff))))
    ;
    iter1 = v2;
    v5 = *((char *)iter1);
    node = v3;
    v7 = *((char *)v3);
    if (v5 == 45)
    {
        while (1)
        {
            v24 = (char)iter1[1];
            iter1 += 1;
            if (v24 == 48)
                continue;
            m = v24;
            if (m != *((int *)&g_41e864))
                break;
        }
        if (v7 != 45)
        {
            if (m == g_41e868)
            {
                do
                {
                    iter1 += 1;
                } while ((char)v26[1] == 48);
                m = (char)v26[1];
            }
            if (m - 48 <= 9)
                return 0xffffffff;
            for (; v7 == 48 || (v28 = (unsigned int)v7, v28 == *((int *)&g_41e864)); node += 1)
            {
                v7 = (char)node[1];
            }
            if (v28 == g_41e868)
            {
                do
                {
                    node += 1;
                } while ((char)v29[1] == 48);
                v28 = (char)v29[1];
            }
            return -(v28 - 48 < 10);
        }
        while (1)
        {
            v31 = (char)node[1];
            v32 = node + 1;
            node = v32;
            if (v31 == 48)
                continue;
            v33 = v31;
            node = v32;
            v34 = iter1;
            if (v33 != *((int *)&g_41e864))
                break;
        }
        while (1)
        {
            iter1 = v32;
            node = v34;
            if (v24 != v31 || v33 - 48 > 9)
                break;
            do
            {
                node += 1;
                v24 = (char)node[1];
            } while (m == *((int *)&g_41e864));
            do
            {
                v33 = (char)iter1[1];
                v32 = iter1 + 1;
                v31 = v33;
                v34 = node;
                if (v33 != *((int *)&g_41e864))
                    break;
            } while ((v33 = (unsigned int)(char)(char)v32[1], v32 += 1, v31 = (char)v33, v34 = node, iter1 = v32, v33 == *((int *)&g_41e864)));
        }
        if (g_41e868 == m && v33 - 48 > 9)
        {
LABEL_41275d:
            v23 = g_41e868;
LABEL_4122a0:
            if (*((char *)iter1) != v23)
            {
                if (v23 != *((char *)node))
                    return 0;
                v39 = (char)node[1];
                node += 1;
            }
            else
            {
                if (v23 != *((char *)node))
                {
                    v40 = (char)iter1[1];
                    iter1 += 1;
                }
                else
                {
                    while (1)
                    {
                        v41 = (char)iter1[1];
                        iter1 += 1;
                        node += 1;
                        v40 = v41;
                        v42 = v41 - 48;
                        if ((char)v41 != (char)node[1])
                            break;
                        if (v42 > 9)
                            return 0;
                    }
                    v43 = (char)v39 - 48;
                    if (v42 > 9)
                    {
                        if (v43 > 9)
                            return 0;
                        for (; (char)v39 == 48; node += 1)
                        {
                            v39 = (char)node[1];
                        }
                        return -(v39 - 48 < 10);
                    }
                    else if (v43 <= 9)
                    {
                        return v41 - (char)v39;
                    }
                }
                for (; (char)v40 == 48; iter1 += 1)
                {
                    v40 = (char)iter1[1];
                }
                return v40 - 48 <= 9;
            }
        }
        v35 = m - 48;
        v18 = v33 - m;
        if (v33 != g_41e868)
        {
            if (v35 <= 9)
                goto LABEL_412661;
            iter = 0;
            if (v33 - 48 > 9)
                return 0;
        }
        else
        {
            if (v35 > 9)
                goto LABEL_41275d;
LABEL_412661:
            iter = 0;
            while (1)
            {
                v36 = (char)node[1];
                node += 1;
                if (v36 == *((int *)&g_41e864))
                    continue;
                iter += 1;
                if (v36 - 48 > 9)
                    break;
            }
            if (v33 - 48 > 9)
                return -(0 < iter);
        }
        v37 = 0;
        while (1)
        {
            v38 = (char)iter1[1];
            iter1 += 1;
            if (v38 == *((int *)&g_41e864))
                continue;
            v37 += 1;
            if (v38 - 48 > 9)
                break;
        }
        if (iter != v37)
            return (-(iter < v37) & 2) - 1;
LABEL_4126ae:
        if (iter)
            return v18;
        return 0;
    }
    if (v7 == 45)
    {
        while (1)
        {
            v8 = (char)node[1];
            node += 1;
            if ((char)v8 == 48)
                continue;
            if (v8 != *((int *)&g_41e864))
                break;
        }
        if (v8 == g_41e868)
        {
            do
            {
                v8 = (char)node[1];
                v9 = node + 1;
            } while ((char)v8 == 48 && (v8 = (unsigned int)(char)(char)v9[1], node = v9 + 1, (char)v8 == 48));
        }
        if (v8 - 48 <= 9)
            return 1;
        for (; v5 == 48 || (v10 = (unsigned int)v5, v10 == *((int *)&g_41e864)); iter1 += 1)
        {
            v5 = (char)iter1[1];
        }
        if (v10 == g_41e868)
        {
            do
            {
                iter1 += 1;
            } while ((char)v11[1] == 48);
            v10 = (char)v11[1];
        }
        return v10 - 48 <= 9;
    }
    for (; v5 == 48 || (l = (unsigned int)v5, (unsigned int)v5 == *((int *)&g_41e864)); iter1 += 1)
    {
        v5 = (char)iter1[1];
    }
    v14 = node;
    if (v7 != 48)
        goto LABEL_4123f6;
    while (1)
    {
        v16 = node;
        v7 = (char)v16[1];
        v14 = v16 + 1;
        node = v14;
        if ((char)v16[1] == 48)
            continue;
LABEL_4123f6:
        node = v14;
        v15 = v7;
        if (v15 != *((int *)&g_41e864))
            break;
    }
    while (v5 == v7 && v15 - 48 <= 9)
    {
        do
        {
            iter1 += 1;
            v5 = (char)iter1[1];
        } while (l == *((int *)&g_41e864));
        do
        {
            node += 1;
            v7 = (char)node[1];
        } while (v15 == *((int *)&g_41e864) && (v15 = (unsigned int)(char)(char)node[1], node += 1, v7 = (char)v15, v15 == *((int *)&g_41e864)));
    }
    if (g_41e868 == l && v15 - 48 > 9)
    {
LABEL_4126e2:
        v23 = g_41e868;
        goto LABEL_4122a0;
    }
    v17 = l - 48;
    v18 = l - v15;
    if (v15 != g_41e868)
    {
        if (v17 <= 9)
            goto LABEL_4125e2;
        iter = 0;
        if (v15 - 48 > 9)
            return 0;
    }
    else
    {
        if (v17 > 9)
            goto LABEL_4126e2;
LABEL_4125e2:
        iter = 0;
        while (1)
        {
            v20 = (char)iter1[1];
            iter1 += 1;
            if (v20 == *((int *)&g_41e864))
                continue;
            iter += 1;
            if (v20 - 48 > 9)
                break;
        }
        if (v15 - 48 > 9)
            return iter;
    }
    v21 = 0;
    while (1)
    {
        v22 = (char)node[1];
        node += 1;
        if (v22 == *((int *)&g_41e864))
            continue;
        v21 += 1;
        if (v22 - 48 > 9)
            break;
    }
    if (iter != v21)
        return -(iter < v21) | 1;
    goto LABEL_4126ae;
}



// Function: human_numcompare @ 0x6530
long long human_numcompare(char *i, char *j)
{
    unsigned long v1;  // rdx
    unsigned int v2;  // eax
    unsigned int v3;  // eax

    for (; *((char *)(v1 + (sub_4053e0(*(i)) & 0xff))); i += 1);
    for (; *((char *)(v1 + (sub_4053e0(*(j)) & 0xff))); j += 1);
    v2 = sub_4054f0(i);
    v3 = sub_4054f0(j);
    if (v2 != v3)
        return v2 - v3;
}



// Function: xstrxfrm @ 0x65c0
long long xstrxfrm(char *a0, unsigned long a1, unsigned long a2)
{
    int *err;  // rax
    int *v2;  // rbx
    unsigned long v3;  // rax
    char *v4;  // rax

    err = __errno_location();
    *(err) = 0;
    v2 = err;
    v3 = strxfrm(a0, a1, a2);
    if (!*(v2))
        return v3;
    v4 = dcgettext(NULL, "string transformation failed", 5);
    error(0, *(v2), v4);
    error(0, 0, dcgettext(NULL, "set LC_ALL='C' to work around the problem", 5));
    sub_4110a0(0, 8, a1);
    error(2, 0, dcgettext(NULL, "the untransformed string was %s", 5));
}



// Function: compare_random @ 0x6680
extern uint128_t g_41e300;
extern uint128_t g_41e310;
extern uint128_t g_41e320;
extern uint128_t g_41e330;
extern uint128_t g_41e340;
extern unsigned long long g_41e350;
extern unsigned int g_41e358;
extern char g_41e861;

unsigned int compare_random(char *a0, unsigned long long a1, char *a2, unsigned long long a3)
{
    char *v26;  // r14
    char *v27;  // rbp
    unsigned long long v28;  // rbx
    unsigned long long v29;  // r13
    char *ptr;  // r15
    unsigned long long v31;  // r12
    char *v0;  // [bp-0x1110]
    char *v2;  // [bp-0x1100]
    unsigned int v3;  // [bp-0x10ec]
    unsigned long long v4;  // [bp-0x10e8]
    char *v5;  // [bp-0x10e0]
    char *v6;  // [bp-0x10d8]
    char *v7;  // [bp-0x10d0]
    char v8;  // [bp-0x10c8]
    char v9;  // [bp-0x10b8]
    int v10;  // [bp-0x10a8]
    int v11;  // [bp-0x1098]
    int v12;  // [bp-0x1088]
    int v13;  // [bp-0x1078]
    int v14;  // [bp-0x1068]
    unsigned long v15;  // [bp-0x1058]
    unsigned int v16;  // [bp-0x1050]
    int v17;  // [bp-0x104c]
    int v18;  // [bp-0x103c]
    int v19;  // [bp-0x102c]
    int v20;  // [bp-0x101c]
    int v21;  // [bp-0x100c]
    unsigned long v22;  // [bp-0xffc]
    unsigned int v23;  // [bp-0xff4]
    char v24;  // [bp-0xfe8]

    v26 = a2;
    v27 = a0;
    v28 = a1;
    v29 = a3;
    v23 = g_41e358;
    v22 = g_41e350;
    v15 = g_41e350;
    v16 = g_41e358;
    v17 = (int)g_41e300;
    v18 = (int)g_41e310;
    v19 = (int)g_41e320;
    v20 = (int)g_41e330;
    v21 = (int)g_41e340;
    v10 = (int)g_41e300;
    v11 = (int)g_41e310;
    v12 = (int)g_41e320;
    v13 = (int)g_41e330;
    v14 = (int)g_41e340;
    if (g_41e861)
    {
        ptr = NULL;
        v31 = 4000;
        v0 = &a0[a1];
        v2 = &v26[a3];
        v4 = (v28 + a3) * 3 + 2;
        v7 = &v24;
        v27 = &v24;
        v6 = &v10;
        v3 = 0;
        v5 = &v17;
    }
    else
    {
        sub_40c690();
        sub_40c6c0(&v10, &v8);
        v27 = &v17;
        sub_40c690(v26, v29, &v17);
        sub_40c6c0(v27, &v9);
        ptr = NULL;
        v3 = memcmp(&v8, &v9, 16);
        if (v3)
        {
            free(NULL);
            return v3;
        }
    }
}



// Function: check_inputs @ 0x6c00
int check_inputs(void* a0, unsigned long long a1)
{
    unsigned long long v5;  // r14
    unsigned long long v6;  // r13
    unsigned long long v7;  // r12
    unsigned long long v8;  // rbp
    unsigned long long v9;  // rbx
    unsigned int v10;  // eax
    char *v11;  // rax
    unsigned int v12;  // eax
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]
    unsigned long long v3;  // [bp-0x8]

    if (!a1)
        return v12;
    v3 = v5;
    v2 = v6;
    v1 = v7;
    v8 = 0;
    v0 = v9;
    while (1)
    {
        v10 = strcmp(*((long long *)a0), "-");
        if (v10)
        {
            v10 = euidaccess(*((long long *)a0), 4);
            if (v10)
            {
                v11 = dcgettext(NULL, "cannot read", 5);
                sub_405e10(v11, *((long long *)a0)); /* do not return */
            }
        }
        v8 += 1;
        a0 += 8;
        if (a1 == v8)
            return v10;
    }
}



// Function: move_fd @ 0x6c80
int move_fd(int a0, int a1)
{
    int v2;  // eax
    char v0;  // [bp+0x0]

    if (a0 != a1)
    {
        dup2(a0, a1);
        return close((unsigned int)&v0);
    }
    return v2;
}



// Function: check_output @ 0x6ca0
int check_output(unsigned long a0)
{
    unsigned int v1;  // eax
    int v3;  // eax

    if (!a0)
        return v3;
    v1 = open(a0, 524353, 438);
    if (v1 < 0)
        sub_405e10(dcgettext(NULL, "open failed", 5), a0); /* do not return */
    return sub_406c80(v1, 1);
}



// Function: delete_proc @ 0x6cf0
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

extern long long g_41e360;

unsigned int delete_proc(unsigned int a0)
{
    struct_0 *v3;  // rax
    char v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x20]

    v1 = a0;
    v3 = sub_40dff0(g_41e360, &v0);
    if (!v3)
        return 0;
    v3->field_c = 2;
    return 1;
}



// Function: reap @ 0x6d50
extern unsigned int g_41e35c;
extern long long g_41e408;

int reap(int a0)
{
    int v2;  // ebx
    int v3;  // edi
    unsigned int v4;  // eax
    char *v6;  // rax
    int *err;  // rax
    int v0;  // [bp-0x24], Other Possible Types: unsigned int

    v2 = a0;
    if (a0)
        v3 = v2;
    else
        v3 = -0x1;
    v4 = waitpid(v3, &v0, !a0);
    if (v4 < 0)
    {
        sub_411170(4, g_41e408);
        v6 = dcgettext(NULL, "waiting for %s [-d]", 5);
        err = __errno_location();
        error(2, *(err), v6);
    }
    else if (v4)
    {
        if (v2 <= 0 && !(char)sub_406cf0(v4))
            return v4;
        if (!(char)v0 && !((char)v0 & 127))
        {
            g_41e35c = g_41e35c - 1;
            return v4;
        }
        sub_411170(4, g_41e408);
        error(2, 0, dcgettext(NULL, "%s [-d] terminated abnormally", 5));
    }
    else
    {
        return v4;
    }
}



// Function: wait_proc @ 0x6e60
int wait_proc(unsigned int a0)
{
    unsigned int v1;  // eax

    v1 = sub_406cf0(a0);
    if ((char)v1)
        return sub_406d50(a0);
    return v1;
}



// Function: zaptemp @ 0x6e80
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
    char field_c;
    char field_d;
} struct_0;

extern unsigned long long g_41e040;
extern struct_0 g_41e368;

void zaptemp(unsigned long i)
{
    struct_0 *v3;  // rbx
    struct_0 *v4;  // rbp
    struct_0 *ptr;  // rbp
    struct_0 *v6;  // r14
    unsigned int v7;  // eax
    unsigned int v8;  // eax
    unsigned int v0;  // [bp-0xcc]
    char v1[136];  // [bp-0xc8]

    v3 = &g_41e368.field_0;
    v4 = *((long long *)&g_41e368);
    ptr = v4;
    if (i != *((long long *)&g_41e368) + 13)
    {
        do
        {
            v3 = v4;
            ptr = v3->field_0;
            v4 = ptr;
        } while (i != &v4->field_d);
    }
    if (ptr->field_c == 1)
        sub_406e60(ptr->field_8);
    v6 = ptr->field_0;
    sub_405a60(v1);
    v7 = unlink(i);
    v8 = *(__errno_location());
    v3->field_0 = v6;
    v0 = v8;
    sub_405a80(v1);
    if (v7)
    {
        sub_411270(0, 3, i);
        error(0, v0, dcgettext(NULL, "warning: cannot remove: %s", 5));
        if (v6)
        {
            free(ptr);
            return;
        }
    }
    else if (v6)
    {
        free(ptr);
        return;
    }
    g_41e040 = v3;
    free(ptr);
    return;
}



// Function: reap_exited @ 0x6fb0
extern void g_41e35c;

unsigned long long reap_exited(void)
{
    unsigned long long v1;  // rax

    if (*((int *)&g_41e35c) <= 0)
        return *((int *)&g_41e35c);
    while (1)
    {
        v1 = sub_406d50(0);
        if (!(unsigned int)v1)
        {
            return v1;
        }
        else if (*((int *)&g_41e35c) <= 0)
        {
            return v1;
        }
    }
}



// Function: reap_some @ 0x6ff0
unsigned long long reap_some(void)
{
    sub_406d50(-0x1);
    return sub_406fb0();
}



// Function: pipe_fork @ 0x7040
extern unsigned int g_41e024;
extern unsigned int g_41e35c;
extern unsigned long long g_41e368;

unsigned int pipe_fork(int *a0, unsigned long long a1)
{
    unsigned long long v3;  // rbx
    unsigned int *err;  // rbp
    unsigned long long v5;  // rbx
    unsigned long v6;  // r14
    unsigned int v7;  // eax
    unsigned int v8;  // r15d
    unsigned int v9;  // r12d
    unsigned long long v0;  // [bp-0xd8]
    char v1[136];  // [bp-0xc8]

    if (sub_40ec90() < 0)
        return 0xffffffff;
    if (g_41e024 + 1 < g_41e35c)
        sub_406ff0();
    v3 = a1 - 1;
    err = __errno_location();
    v0 = 0x3fd0000000000000;
    while (1)
    {
        v5 = v3;
        sub_405a60(v1);
        v6 = g_41e368;
        g_41e368 = 0;
        v7 = fork();
        v8 = *(err);
        v9 = v7;
        if (!v9)
        {
            sub_405a80(v1);
            *(err) = v8;
            close(0);
            close(1);
            return v9;
        }
        g_41e368 = v6;
        sub_405a80(v1);
        *(err) = v8;
        if (v9 >= 0)
        {
            g_41e35c = g_41e35c + 1;
            return v9;
        }
        else if (v8 == 11)
        {
            sub_413440();
            v0 = AddV(v0, v0);
            sub_406fb0();
            v3 = v5 - 1;
            if (v5 < 1)
            {
                v8 = *(err);
                close(*(a0));
                close(a0[1]);
                *(err) = v8;
                return v9;
            }
        }
        else
        {
            close(*(a0));
            close(a0[1]);
            *(err) = v8;
            return v9;
        }
    }
}



// Function: async_safe_die @ 0x71d0
void async_safe_die(unsigned int a0, void* a1)
{
    unsigned long long v4;  // r12
    unsigned long v5;  // fs
    void* v7;  // rbp
    char v0;  // [bp-0x2c]
    unsigned long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = *((long long *)(40 + v5));
    write(2, a1, strlen(a1));
    if (a0)
    {
        v7 = sub_40e440(a0, &v0);
        write(2, ": errno ", 8);
        write(2, v7, strlen(v7));
    }
    write(2, "\n", 1);
    _exit(2); /* do not return */
}



// Function: register_proc @ 0x7270
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

extern unsigned long long g_41e360;

void register_proc(struct_0 *a0)
{
    unsigned long long v1;  // rdi
    long long v2;  // rdi

    v1 = g_41e360;
    if (v1 || (v1 = (unsigned long long)sub_40d970(47, 0, sub_405400, sub_405420, 0), g_41e360 = v1, v1))
    {
        a0->field_c = 1;
        if (sub_40df90(v1, a0))
            return;
    }
    sub_413270(v2); /* do not return */
}



// Function: open_temp @ 0x72d0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
} struct_0;

extern void g_41e408;

FILE * open_temp(struct_0 *a0, unsigned long a1, unsigned int a2)
{
    unsigned int v3;  // edx
    unsigned int v4;  // r12d
    unsigned int v5;  // eax
    FILE *fp;  // rax
    unsigned int *err;  // rax
    unsigned int *err1;  // rax
    int *err2;  // rbp
    char *v10;  // rax
    int v0;  // [bp-0x28]
    int v1;  // [bp-0x24]

    if (a0->field_c == 1)
        sub_406e60(a0->field_8);
    v4 = open(a0 + 1, 0, v3);
    if (v4 < 0)
        return NULL;
    v5 = sub_407040(&v0, 9);
    if (v5 != 0xffffffff)
    {
        if (v5)
        {
            a0->field_8 = v5;
            sub_407270(a0);
            close(v4);
            close(v1);
            fp = fdopen(v0, "r");
            if (!fp)
            {
                err = __errno_location();
                close(v0);
                *(err) = *(err);
                return fp;
            }
            return fp;
        }
        else
        {
            close(v0);
            sub_406c80(v4, 0);
            sub_406c80(v1, 1);
            execlp(*((long long *)&g_41e408), *((long long *)&g_41e408));
            err1 = __errno_location();
            sub_4071d0(*(err1), "couldn't execute compress program (with -d)"); /* do not return */
        }
    }
    err2 = __errno_location();
    if (*(err2) == 24)
    {
        close(v4);
        *(err2) = 24;
        return NULL;
    }
    sub_411170(4, *((long long *)&g_41e408));
    v10 = dcgettext(NULL, "couldn't create process for %s -d", 5);
    error(2, *(err2), v10);
}



// Function: open_input_files @ 0x7470
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_0 *field_8;
} struct_1;

long long open_input_files(struct_1 *a0, long long a1, unsigned long long *a2)
{
    struct_1 *iter;  // r14
    unsigned long long v2;  // rax
    unsigned long long v3;  // rbp
    long long idx;  // r13
    struct_0 *v5;  // rdi
    long long v6;  // rax
    unsigned int v7;  // rsi
    unsigned int v8;  // edx
    FILE *v9;  // rax

    iter = a0;
    v2 = sub_412fd0(a1, 8);
    *(a2) = v2;
    if (!a1)
        return 0;
    v3 = v2;
    idx = 0;
    while (1)
    {
        v5 = iter->field_8;
        if (!v5 || !v5->field_c)
        {
            v6 = sub_405ee0(*((long long *)&iter->field_0), "r");
            *((long long *)(v3 + idx * 8)) = v6;
            if (!v6)
                return idx;
        }
        else
        {
            v9 = sub_4072d0(v5, v7, v8);
            *((FILE **)(v3 + idx * 8)) = v9;
            if (!v9)
                return idx;
        }
        idx += 1;
        iter += 1;
        if (a1 == idx)
            return idx;
    }
}



// Function: create_temp_file @ 0x7510
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_1 *g_41e040;
extern unsigned long long g_41e248;
extern char g_41e428;
extern unsigned long long g_41e430;

unsigned long long * create_temp_file(unsigned int *a0, char a1)
{
    unsigned long ptr;  // r14
    unsigned long len;  // rbx
    unsigned long long *v13;  // r12
    char *v14;  // rax
    unsigned long long *v5;  // rax
    unsigned long long *v6;  // r15
    unsigned long long *ptr1;  // r12
    unsigned long long v8;  // 4106
    unsigned int v9;  // r15d
    unsigned int *err;  // rbx
    struct_0 **v11;  // rax
    unsigned int v12;  // r13d
    unsigned int v0;  // [bp-0xcc]
    char v1[136];  // [bp-0xc8]

    ptr = *((long long *)(g_41e430 + g_41e248 * 8));
    len = strlen(ptr);
    v5 = sub_412eb0(len + 32 & 0xfffffffffffffff8);
    v6 = (char *)&v5[1] + 5;
    ptr1 = v5;
    memcpy(v6, ptr, len);
    *((unsigned long *)((char *)ptr1 + len + 13)) = 6365935330447553327;
    *((unsigned int *)((char *)v6 + len + 8)) = 0x585858;
    *(ptr1) = 0;
    v8 = *((long long *)&g_41e428);
    g_41e248 = g_41e248 + 1;
    if (g_41e248 == v8)
        g_41e248 = 0;
    sub_405a60(v1);
    v9 = sub_412280(v6, 0x80000);
    err = __errno_location();
    if (v9 >= 0)
    {
        v11 = g_41e040;
        v12 = *(err);
        g_41e040 = ptr1;
        *(v11) = ptr1;
        sub_405a80(v1);
        *(err) = v12;
    }
    else
    {
        v0 = *(err);
        sub_405a80(v1);
        *(err) = v0;
        if (a1 != 1 || v0 != 24)
        {
            sub_411170(4, ptr);
            v14 = dcgettext(NULL, "cannot create temporary file in %s", 5);
            error(2, *(err), v14);
        }
        v13 = NULL;
        free(ptr1);
        ptr1 = v13;
    }
    *(a0) = v9;
    return ptr1;
}



// Function: maybe_create_temp @ 0x76b0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
} struct_0;

extern char *g_41e408;

struct_0 * maybe_create_temp(FILE **a0, char a1)
{
    struct_0 *idx;  // r12
    char *v5;  // 4098
    unsigned int v6;  // eax
    unsigned int *err;  // rax
    FILE *fp;  // rax
    int v0;  // [bp-0x2c], Other Possible Types: unsigned int
    int v1;  // [bp-0x28]
    int v2;  // [bp-0x24]

    idx = sub_407510(&v0, a1);
    if (!idx)
        return idx;
    v5 = g_41e408;
    idx->field_c = 0;
    if (v5)
    {
        v6 = sub_407040(&v1, 4);
        idx->field_8 = v6;
        if (v6 > 0)
        {
            close(v0);
            close(v1);
            v0 = v2;
            sub_407270(idx);
        }
        else if (!v6)
        {
            close(v2);
            sub_406c80(v0, 1);
            sub_406c80(v1, 0);
            execlp(g_41e408, g_41e408);
            err = __errno_location();
            sub_4071d0(*(err), "couldn't execute compress program"); /* do not return */
        }
    }
    fp = fdopen(v0, "w");
    *(a0) = fp;
    if (!fp)
        sub_405e10(dcgettext(NULL, "couldn't create temporary file", 5), idx + 1); /* do not return */
    return idx;
}



// Function: create_temp @ 0x77e0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
} struct_0;

struct_0 * create_temp(FILE **a0)
{
    return sub_4076b0(a0, 0);
}



// Function: default_sort_size @ 0x77f0
extern unsigned int g_41e024;

unsigned long long default_sort_size(void)
{
    unsigned int v3[3];  // rbx
    unsigned long long v4;  // rbx
    int v13;  // xmm0
    int v14;  // xmm1
    unsigned long v15;  // xmm0lq
    unsigned long long v16;  // rax
    unsigned long long v5;  // rax
    int v6;  // xmm0
    long long v7;  // rdi
    long long v8;  // rsi
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    unsigned long v0;  // [bp-0x40]
    unsigned int v1[3];  // [bp-0x38]

    v3 = 0xffffffffffffffff;
    if (!getrlimit(2, v1))
        v3 = v1;
    if (!getrlimit(9, v1) && v3 > v1)
        v3 = v1;
    v4 = (unsigned long long)(v3 >> 1);
    if (!getrlimit(5, v1))
    {
        v5 = (unsigned long long)((v1 & 0xfffffffffffffff0) - (v1 >> 4));
        if (v4 > v5)
            v4 = v5;
    }
    sub_40eb50();
    v0 = *((unsigned long long *)&v6);
    sub_40ea60(v7, v8, v9, v10, v11, v12);
    v13 = MulV(v6, 0x3fe8000000000000);
    v14 = MaxV(v0, MulV(0x3fc0000000000000, v6));
    if ((((char)(CmpF(v4, *((unsigned long long *)&v13)) & 69) | (char)((CmpF(v4, *((unsigned long long *)&v13)) & 69) >> 6)) & 1) == 1)
    {
LABEL_4078cf:
        if (v4 < 0)
            goto LABEL_40794a;
LABEL_4078d4:
        v15 = v4;
    }
    else if (!(CmpF(*((unsigned long long *)&v13), 0x43e0000000000000) & 1))
    {
        v4 = 0x8000000000000000 ^ *((unsigned long long *)&SubV(v13, 0x43e0000000000000));
        goto LABEL_4078cf;
    }
    else
    {
        v4 = *((unsigned long long *)&v13);
        if (v4 >= 0)
            goto LABEL_4078d4;
LABEL_40794a:
        v15 = AddV(v4 >> 1 | (unsigned int)v4 & 1, v4 >> 1 | (unsigned int)v4 & 1);
    }
    if (((CmpF(v15, *((unsigned long long *)&v14)) & 69 | (char)((CmpF(v15, *((unsigned long long *)&v14)) & 69) >> 6)) & 1) != 1)
        v4 = (CmpF(*((unsigned long long *)&v14), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v14) : 0x8000000000000000 ^ *((unsigned long long *)&SubV(v14, 0x43e0000000000000)));
    v16 = g_41e024 * 0x22;
    if (v16 >= v4)
        return v16;
    return v4;
}



// Function: init_node @ 0x7970
typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[24];
    unsigned int field_50;
} struct_0;

typedef struct struct_2 {
    long long field_0;
    long long field_8;
    long long field_10;
    long long field_18;
    struct struct_0 *field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    struct struct_0 *field_38;
    char padding_40[16];
    unsigned int field_50;
} struct_2;

struct_2 * init_node(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, char a5)
{
    unsigned long v6;  // rdx
    long long v7;  // r8
    struct_0 *index;  // rdi
    long long v8;  // r12
    char v9;  // r9b
    unsigned long long v10;  // rcx
    unsigned long long v11;  // rdx
    long long v12;  // r14
    struct_0 *v13;  // rsi

    v8 = v6 - v7 * 32;
    if (v9)
    {
        v10 = index->field_28 >> 1;
        v11 = index->field_28 - v10;
        v12 = v8 - v10 * 32;
        v13 = &index->padding_0[16];
    }
    else
    {
        v10 = index->field_30 >> 1;
        v11 = index->field_30 - v10;
        v12 = v8 - v10 * 32;
        v13 = &index->padding_0[24];
    }
}



// Function: merge_tree_init @ 0x7aa0
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned int field_28;
    char padding_2c[4];
    unsigned int field_30;
    char padding_34[4];
    unsigned long long field_38;
    char padding_40[16];
    unsigned int field_50;
    char field_54;
} struct_0;

struct_0 * merge_tree_init(unsigned int a0, unsigned int a1, unsigned int a2)
{
    struct_0 *idx;  // rax

    idx = sub_412eb0(a0 * 0x100);
    *((unsigned long long *)&idx->field_30) = a1;
    *((unsigned long long *)&idx->field_28) = a1;
    idx->field_18 = 0;
    idx->field_10 = 0;
    idx->field_8 = 0;
    idx->field_0 = 0;
    idx->field_20 = 0;
    idx->field_38 = 0;
    idx->field_50 = 0;
    idx->field_54 = 0;
    pthread_mutex_init((char *)&idx[1].field_0 + 3, 0);
    sub_407970(idx, (char *)&idx[1].field_28 + 3, a2, a0, a1, 0);
    return idx;
}



// Function: queue_init @ 0x7b40
void queue_init(unsigned long long *a0, unsigned long a1)
{
    *(a0) = sub_40e300(sub_405780, a1 * 2);
    pthread_mutex_init(a0 + 1, 0);
    pthread_cond_init(a0 + 6, 0);
    return;
}



// Function: lock_node @ 0x7b70
void lock_node(long long a0)
{
    pthread_mutex_lock(a0 + 88);
    return;
}



// Function: queue_insert @ 0x7b80
typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

void queue_insert(long long *a0, struct_0 *a1)
{
    long long *v1;  // r12

    v1 = a0 + 1;
    pthread_mutex_lock(v1);
    sub_40e390(*(a0), a1);
    a1->field_54 = 1;
    pthread_cond_signal(a0 + 6);
    pthread_mutex_unlock(v1);
    return;
}



// Function: queue_check_insert @ 0x7bc0
typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

long long queue_check_insert(long long *a0, struct_0 *a1)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax

    if (a1->field_54)
        return v2;
    v1 = *((long long *)&a1->padding_0[24]);
    if (*((long long *)&a1->padding_0[0]) != *((long long *)&a1->padding_0[16]))
    {
        if (*((long long *)&a1->padding_0[8]) == v1 && *((long long *)&a1->padding_0[48]))
            return v1;
    }
    else
    {
        if (*((long long *)&a1->padding_0[8]) == v1)
        {
            return v1;
        }
        else if (*((long long *)&a1->padding_0[40]))
        {
            return v1;
        }
    }
    return (unsigned long long)sub_407b80(a0, a1);
}



// Function: unlock_node @ 0x7c00
void unlock_node(long long a0)
{
    pthread_mutex_unlock(a0 + 88);
    return;
}



// Function: queue_check_insert_parent @ 0x7c10
long long queue_check_insert_parent(long long *a0, void* a1)
{
    if ((int)a1[80] > 1)
    {
        sub_407b70((long long)a1[56]);
        sub_407bc0(a0, (long long)a1[56]);
        return (unsigned long long)sub_407c00((long long)a1[56]);
    }
    else if ((long long)a1[48] + (long long)a1[40])
    {
        return (long long)a1[48] + (long long)a1[40];
    }
    else
    {
        return (unsigned long long)sub_407b80(a0, (long long)a1[56]);
    }
}



// Function: queue_pop @ 0x7c70
typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

struct_0 * queue_pop(long long *a0)
{
    long long *v1;  // rbp
    struct_0 *v2;  // r12

    v1 = a0 + 1;
    pthread_mutex_lock(v1);
    while (1)
    {
        if (sub_40e400(*(a0)))
            break;
        pthread_cond_wait(a0 + 6, v1);
    }
    pthread_mutex_unlock(v1);
    sub_407b70(v2);
    v2->field_54 = 0;
    return v2;
}



// Function: merge_tree_destroy @ 0x7cd0
void merge_tree_destroy(unsigned long a0, unsigned long ptr)
{
    unsigned long v1;  // rbx
    unsigned long i;  // rbx
    unsigned long v3;  // rbx

    if (!(a0 * 2))
    {
        free(ptr);
        return;
    }
    v1 = ptr + 88;
    i = v1;
    do
    {
        v3 = i + 128;
        pthread_mutex_destroy(i);
        i = v3;
    } while (i != a0 * 0x100 + v1);
    free(ptr);
    return;
}



// Function: queue_destroy @ 0x7d10
void queue_destroy(long long *a0)
{
    sub_40e370(*(a0));
    pthread_cond_destroy(a0 + 6);
    pthread_mutex_destroy(a0 + 1);
    return;
}



// Function: key_warnings @ 0x7d30
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern unsigned int g_41e028;
extern struct_0 *g_41e410;
extern char g_41e419;
extern char g_41e41a;
extern char g_41e862;
extern char g_41e864;
extern char g_41e868;

void key_warnings(struct_0 *index, char a1)
{
    struct_0 *v26;  // r13
    unsigned int v27;  // r14d
    char *v36;  // rax
    char *v37;  // rax
    char v40;  // bl
    unsigned int v41;  // eax
    char v42;  // dl
    unsigned int v28;  // r12d
    unsigned long long v44;  // rax
    unsigned int v45;  // edx
    unsigned int v47;  // eax
    char v48;  // al
    unsigned int v29;  // ebp
    unsigned long long *v49;  // rdx
    char *v51;  // rsi
    char v53;  // al
    char v57;  // bl
    struct_0 *v58;  // rdi
    unsigned int v30;  // r15d
    char *v59;  // rsi
    int v60;  // r8d
    struct_0 *idx;  // r13
    char v32;  // al
    char v33;  // dl
    unsigned long long v34;  // rdi
    unsigned long long v35;  // rbx
    char v0;  // [bp-0x14a]
    char v1;  // [bp-0x149]
    unsigned long long v2;  // [bp-0x148]
    char *v3;  // [bp-0x140], Other Possible Types: unsigned long long
    unsigned long v4;  // [bp-0x138], Other Possible Types: unsigned long long
    char *v5;  // [bp-0x130]
    char *v6;  // [bp-0x128]
    char v7;  // [bp-0x120]
    char v8;  // [bp-0x11f]
    char v9;  // [bp-0x11e]
    char v10;  // [bp-0x11c]
    char v11;  // [bp-0x11b]
    char v12;  // [bp-0x11a]
    char v13;  // [bp-0x119]
    struct_0 v14;  // [bp-0x118]
    int v15;  // [bp-0x108]
    int v16;  // [bp-0xf8]
    int v17;  // [bp-0xe8]
    char v18;  // [bp-0xe1]
    unsigned long v19;  // [bp-0xd8]
    char v20;  // [bp-0xc8]
    char v21;  // [bp-0xa8]
    char v22;  // [bp-0xa7]
    unsigned int v23;  // [bp-0x78], Other Possible Types: unsigned short, char
    char v24;  // [bp-0x77]

    v14 = (struct_0)*((int128_t *)&index->field_0.padding_0[0]);
    v15 = (int)*((int128_t *)&index->field_0.padding_0[16]);
    v19 = index->field_10;
    v16 = (int)*((int128_t *)&index->field_0.padding_0[32]);
    v17 = (int)*((int128_t *)&index->field_0.field_30);
    if (g_41e410)
    {
        v13 = 0;
        v26 = g_41e410;
        v12 = 0;
        v27 = *((char *)&v17);
        v0 = *((char *)(&v17 + 1));
        v28 = *((char *)(&v17 + 4));
        v29 = *((char *)(&v17 + 8));
        v11 = 0;
        v1 = *((char *)(&v17 + 6));
        v10 = 0;
        v30 = *((char *)(&v17 + 3));
        v7 = *((char *)(&v17 + 2));
        v2 = 1;
        v8 = *((char *)(&v17 + 5));
        v9 = *((char *)(&v17 + 7));
        do
        {
            idx = v26;
            v32 = sub_405660(idx);
            if (v32)
            {
                v33 = *((char *)&idx->field_0.field_30 + 4);
                v10 = (!v33 ? v32 : v10);
                v11 = (v33 ? v33 : v11);
            }
            if ((&idx->field_0)[1].padding_0[1])
            {
                v34 = *((long long *)&idx->field_0.padding_0[0]);
                v35 = v34 + 1;
                v4 = *((long long *)&idx->field_0.padding_0[16]);
                if (v34 == 0xffffffffffffffff)
                {
                    v35 = 1;
                    v34 = 0;
                }
                v3 = &v20;
                v21 = 43;
                v5 = __stpcpy_chk(&v22, sub_40e520(v34, &v20), 45);
                v6 = v3;
                v23 = 2124589;
                v3 = __stpcpy_chk((char *)&v23 - 3, sub_40e520(v35, v6), 44);
                if (*((long long *)&idx->field_0.padding_0[16]) != 0xffffffffffffffff)
                {
                    v4 += 1;
                    v36 = sub_40e520(v4, v6);
                    *((unsigned short *)&v5) = 11552;
                    strcpy(v5 + 2, v36);
                    v37 = sub_40e520((*((long long *)&idx->field_0.padding_0[24]) == 0xffffffffffffffff) + v4, v6);
                    *(v3) = 44;
                    strcpy(v3 + 1, v37);
                }
                sub_411420(1, &v23);
                v3 = sub_411420(0, &v21);
                error(0, 0, dcgettext(NULL, "obsolescent key %s used; consider %s instead", 5));
            }
            if (*((long long *)&idx->field_0.padding_0[0]) != 0xffffffffffffffff && *((long long *)&idx->field_0.padding_0[0]) > *((long long *)&idx->field_0.padding_0[16]))
                error(0, 0, dcgettext(NULL, "key %lu has zero width and will be ignored", 5));
            v40 = sub_405660(idx);
            v41 = 1;
            if (!v40)
                v41 = *((char *)&idx->field_0.field_30 + 6);
            if (!*((long long *)&idx->field_0.padding_0[16]) && *((long long *)&idx->field_0.padding_0[24]))
            {
                if (!v42)
                    goto LABEL_407e22;
                goto LABEL_407e17;
            }
            else if (v42)
            {
LABEL_407e17:
                if (a1)
                    goto LABEL_407e22;
                goto LABEL_408108;
            }
            else
            {
                if (g_41e028 == 128 && (!(char)idx->field_0.field_30 && (!v41 || *((long long *)&idx->field_0.padding_0[8])) || !*((char *)&idx->field_0.field_30 + 1) && *((long long *)&idx->field_0.padding_0[24])))
                {
                    error(0, 0, dcgettext(NULL, "leading blanks are significant in key %lu; consider also specifying 'b'", 5));
                    v40 = sub_405660(idx);
                }
LABEL_408108:
                if (!v40)
                {
LABEL_407e22:
                    v45 = *((char *)&idx->field_0.field_30 + 4);
                    continue;
                }
                else
                {
                    v44 = *((long long *)&idx->field_0.padding_0[0]) + 1;
                    if (*((long long *)&idx->field_0.padding_0[0]) == 0xffffffffffffffff)
                        v44 = 1;
                    if (*((long long *)&idx->field_0.padding_0[16]) < v44)
                        goto LABEL_407e22;
                    error(0, 0, dcgettext(NULL, "key %lu is numeric and spans multiple fields", 5));
                    v45 = *((char *)&idx->field_0.field_30 + 4);
                    v12 = (!(char)v45 ? v40 : v12);
                    v13 = ((char)v45 ? (char)v45 : v13);
                }
            }
            if (*((unsigned long long *)&v16) && *((unsigned long long *)&v16) == *((long long *)&idx->field_0.padding_0[32]))
                *((unsigned long long *)&v16) = 0;
            if (*((unsigned long long *)(&v16 + 8)) && *((unsigned long long *)(&v16 + 8)) == *((long long *)&idx->field_0.padding_0[40]))
                *((unsigned long long *)&(&v16)[8]) = 0;
            v28 &= v45 ^ 1;
            *((char *)&(&v17)[4]) = v28;
            v27 &= (char)idx->field_0.field_30 ^ 1;
            *((char *)&v17) = v27;
            v0 &= *((char *)&idx->field_0.field_30 + 1) ^ 1;
            v1 &= *((char *)&idx->field_0.field_30 + 6) ^ 1;
            *((char *)&(&v17)[1]) = v0;
            v7 &= *((char *)&idx->field_0.field_30 + 2) ^ 1;
            *((char *)&(&v17)[6]) = v1;
            v8 &= *((char *)&idx->field_0.field_30 + 5) ^ 1;
            *((char *)&(&v17)[2]) = v7;
            v30 &= *((char *)&idx->field_0.field_30 + 3) ^ 1;
            *((char *)&(&v17)[5]) = v8;
            *((char *)&(&v17)[3]) = v30;
            v29 &= (&idx->field_0)[1].padding_0[0] ^ 1;
            *((char *)&(&v17)[8]) = v29;
            v9 &= *((char *)&idx->field_0.field_30 + 7) ^ 1;
            *((char *)&(&v17)[7]) = v9;
            v26 = (long long)idx->field_10;
            v2 += 1;
        } while (idx->field_10);
        if (v12)
        {
            v47 = g_41e028;
            if (g_41e028 != 128 && g_41e028 != *((int *)&g_41e864))
            {
                v12 = 0;
                goto LABEL_40843e;
            }
            if (*((int *)&g_41e864) != 128)
            {
                __ctype_b_loc();
                v48 = sub_4053e0((char)*((int *)&g_41e864));
                if (!(*((char *)(*(v49) + v48 * 2)) & 1))
                {
                    v12 = 0;
                    goto LABEL_4081f3;
                }
                v23 = *((int *)&g_41e864);
                v24 = 0;
                sub_411430(&v23);
                error(0, 0, dcgettext(NULL, "field separator %s is treated as a group separator in numbers", 5));
LABEL_4086aa:
                v47 = g_41e028;
                if (g_41e028 == 128)
                    goto LABEL_4086b4;
LABEL_40843e:
                if (g_41e868 == v47)
                {
LABEL_408217:
                    v24 = 0;
                    v23 = g_41e868;
                    sub_411430(&v23);
                    error(0, 0, dcgettext(NULL, "field separator %s is treated as a decimal point in numbers", 5));
                }
                else
                {
                    switch (v47)
                    {
                    case 45:
                        v23 = 45;
                        sub_411430(&v23);
                        v51 = "field separator %s is treated as a minus sign in numbers";
                        goto LABEL_40862f;
                    case 43:
                        if (v13)
                        {
                            v23 = 43;
                            sub_411430(&v23);
                            v51 = "field separator %s is treated as a plus sign in numbers";
LABEL_40862f:
                            error(0, 0, dcgettext(NULL, v51, 5));
                            break;
                        }
                        break;
                    default:
                        goto LABEL_408466;
                    }
                    goto LABEL_408466;
                }
            }
            else
            {
                v12 = 0;
LABEL_4086b4:
                if (*((int *)&g_41e864) != 128)
                {
LABEL_4081f3:
                    __ctype_b_loc();
                    v53 = sub_4053e0(g_41e868);
                    if (*((char *)(*((long long *)v42) + v53 * 2)) & 1)
                        goto LABEL_408217;
                }
LABEL_408466:
                if (!v10 && !v11)
                    goto LABEL_408298;
                if (!v12)
                    goto LABEL_408483;
            }
        }
        else
        {
            if (v13)
                goto LABEL_4086aa;
            if (!v10 && !v11)
                goto LABEL_408298;
LABEL_408483:
            v24 = 0;
            v23 = g_41e868;
            sub_411430(&v23);
            if (g_41e868 != g_41e028)
                dcgettext(NULL, "note ", 5);
            error(0, 0, dcgettext(NULL, "%snumbers use %s as a decimal point in this locale", 5));
        }
        if (v10 && g_41e862)
            error(0, 0, dcgettext(NULL, "the multi-byte number group separator in this locale is not supported", 5));
    }
LABEL_408298:
    v57 = *((char *)(&v17 + 7));
    if ((char)sub_405680(&v14))
    {
        if (!v57)
            return;
        if (g_41e41a)
        {
            if (!g_41e410)
                return;
            sub_4056d0(v58, &v23);
            error(0, 0, dcngettext(NULL, v59, v42, sub_4053f0(strlen(&v23), "option '-%s' is ignored", "options '-%s' are ignored"), v60));
            v18 = 1;
        }
        else
        {
            if (!g_41e419 || !g_41e410)
                goto LABEL_4083b5;
            sub_4056d0(v58, &v23);
            error(0, 0, dcngettext(NULL, v59, v42, sub_4053f0(strlen(&v23), "option '-%s' is ignored", "options '-%s' are ignored"), v60));
            v18 = v57;
        }
    }
    else
    {
        if (!g_41e41a && !g_41e419)
            v18 = 0;
        sub_4056d0(v58, &v23);
        error(0, 0, dcngettext(NULL, v59, v42, sub_4053f0(strlen(&v23), "option '-%s' is ignored", "options '-%s' are ignored"), v60));
        v18 = v57;
        if (!v57)
            return;
    }
    if (g_41e41a)
        return;
LABEL_4083b5:
    if (g_41e419)
        return;
    if (!g_41e410)
        return;
    error(0, 0, dcgettext(NULL, "option '-r' only applies to last-resort comparison", 5));
    return;
}



// Function: write_line @ 0x9460
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    void* field_0;
    char field_8;
} struct_0;

extern char g_41e038;
extern char g_41e400;

long long write_line(struct_0 *a0, FILE *a1, unsigned long long a2)
{
    void* v1;  // rbp
    unsigned long v2;  // r15
    void* v3;  // rbx
    unsigned long count;  // rax
    unsigned int v6;  // eax
    int v7;  // edi

    v1 = a0->field_0;
    v2 = *((long long *)&a0->field_8);
    v3 = v1 + v2;
    if (!a2 && g_41e400)
    {
        while (1)
        {
            if (v1 < v3)
            {
                v6 = *((char *)v1);
                v1 += 1;
                v7 = 62;
                if ((char)v6 != 9)
                {
                    if (v3 != v1)
                        v7 = v6;
                    else
                        v7 = 10;
                }
                if (fputc_unlocked(v7, a1) == 0xffffffff)
                    sub_405e10(dcgettext(NULL, "write failed", 5), 0); /* do not return */
            }
        }
    }
    *((char *)v3 - 1) = g_41e038;
    count = fwrite_unlocked(v1, 1, v2, a1);
    if (count != v2)
        sub_405e10(dcgettext(NULL, "write failed", 5), a2); /* do not return */
    *((char *)v3 - 1) = 0;
    return count;
}



// Function: keycompare @ 0x9550
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern void* g_41e410;
extern long long g_41e740;
extern char g_41e861;

unsigned long long keycompare(struct_0 *index, struct_0 *a1)
{
    char *iter;  // r14
    void* iter1;  // r15
    char *v17;  // rsi, Other Possible Types: void*
    char v18;  // al
    char *v19;  // r9, Other Possible Types: void*
    char v20;  // r8b
    unsigned int v21;  // edx
    unsigned long v22;  // rdx, Other Possible Types: unsigned long long
    unsigned int v23;  // eax
    char *v24;  // cc_dep1
    char *v25;  // cc_dep2
    char *v26;  // rcx
    void* node;  // rbp
    unsigned long long v27;  // rax
    unsigned long long v28;  // rax
    unsigned long v29;  // rdx
    char *v30;  // r14
    char v31;  // al
    char *v11;  // r12
    char v32;  // dl
    unsigned long v33;  // rax
    unsigned long long v34;  // rdx
    char v35;  // al
    char v36;  // al
    long long v37;  // rdi
    char *v38;  // rcx
    unsigned long long v39;  // r8
    long long v40;  // rdi
    unsigned long long v12;  // rbx
    char v41;  // dl
    char *v42;  // r10
    void* v43;  // rax
    void* v44;  // rsi
    void* v13;  // r11
    unsigned int v45;  // eax
    unsigned long v46;  // r13
    void* v14;  // r11
    unsigned long v15;  // r13
    unsigned long v16;  // r12
    unsigned int v0;  // [bp-0x1020], Other Possible Types: unsigned long long
    unsigned long long v1;  // [bp-0x1010]
    char v2;  // [bp-0x1002]
    char v3;  // [bp-0x1001]
    char *ptr;  // [bp-0xff8]
    char *v5;  // [bp-0xff0]
    char v6;  // [bp-0xfe8]

    iter = &index->field_10->field_0;
    iter1 = a1->field_10;
    node = g_41e410;
    v11 = &index[1].field_0->field_0;
    v5 = &v6;
    while (1)
    {
        v12 = (long long)node[32];
        if (v11 <= iter)
            v11 = iter;
        if (v13 <= iter1)
            v14 = iter1;
        else
            v14 = v13;
        v0 = (long long)node[40];
        v15 = v11 - iter;
        v16 = v14 - iter1;
        if (g_41e861 || (char)sub_405660(node) || 0xff0000ff000000 & (long long)node[48] || (char)node[56])
        {
            v1 = v0 | v12;
            if (!v0 && !v12)
            {
                v35 = *(v26);
                *(v26) = 0;
                v2 = v35;
                v36 = *((char *)v13);
                *((char *)v13) = 0;
                v3 = v36;
                goto LABEL_409769;
            }
            v37 = v15 + v16 + 2;
            ptr = NULL;
            if (v37 <= 4000)
            {
                v38 = v5;
                if (!v15)
                    goto LABEL_409abb;
                goto LABEL_409986;
            }
            v38 = sub_412eb0(v37);
            ptr = v38;
            if (v15)
            {
LABEL_409986:
                v15 = 0;
                do
                {
                    if (v12)
                    {
                        if (*((char *)(v12 + (sub_4053e0(*(iter)) & 0xff))))
                            continue;
                        if (!v39)
                            goto LABEL_4099ab;
                    }
                    sub_4053e0(v40);
LABEL_4099ab:
                    v26[v15] = v41;
                    v15 += 1;
                } while ((iter += 1, v42 != iter));
                v26[v15] = 0;
                if (!v16)
                    goto LABEL_409ad0;
                goto LABEL_4099f0;
            }
LABEL_409abb:
            *(v38) = 0;
            if (v16)
            {
LABEL_4099f0:
                v16 = 0;
                do
                {
                    if (v12)
                    {
                        if (*((char *)(v12 + (sub_4053e0(*((char *)iter1)) & 0xff))))
                            continue;
                        if (!v39)
                            goto LABEL_409a13;
                    }
                    sub_4053e0(v40);
LABEL_409a13:
                    *((char *)v17 + v16) = v41;
                    v16 += 1;
                } while ((iter1 += 1, v19 != iter1));
                v43 = v17 + v16;
                v44 = v17;
                goto LABEL_409a44;
            }
LABEL_409ad0:
            v43 = v17;
            v44 = v17;
LABEL_409a44:
            *((char *)v43) = 0;
            iter1 = v44;
            iter = v26;
LABEL_409769:
            if ((char)node[50])
            {
                v28 = sub_4064d0(iter, iter1);
                goto LABEL_40977e;
            }
            if ((char)node[52])
            {
                v28 = sub_4063d0(iter, iter1);
LABEL_40977e:
                if (v1)
                    goto LABEL_409860;
                goto LABEL_40978a;
            }
            if (!(char)node[53])
            {
                if ((char)node[54])
                {
                    v45 = sub_405570(iter, NULL);
                    v28 = v45 - (sub_405570(iter1, NULL) & 0xffffffff);
                    goto LABEL_40977e;
                }
                else if ((char)node[0x33])
                {
                    v28 = sub_406680(iter, v15, iter1, v16);
                    goto LABEL_40977e;
                }
                else if ((char)node[56])
                {
                    v28 = sub_40cd70(iter, v15, iter1, v16);
                    goto LABEL_40977e;
                }
                else if (!v15)
                {
                    v28 = -(0 < v16);
                    goto LABEL_40977e;
                }
                else
                {
                    v28 = 1;
                    if (v16)
                    {
                        v28 = sub_4133e0(iter, v15 + 1, iter1, v16 + 1);
                        goto LABEL_40977e;
                    }
                }
            }
            v28 = sub_406530(iter, iter1);
            if (!v1)
            {
LABEL_40978a:
                iter[v15] = v2;
                *((char *)iter1 + v16) = v3;
                goto LABEL_40979c;
            }
            else
            {
LABEL_409860:
                v0 = v28;
                free(ptr);
                v28 = v0;
                if (!(unsigned int)v28)
                    goto LABEL_4097a4;
                goto LABEL_409880;
            }
        }
        if (v12)
        {
            if (v0)
            {
                for (; iter < v26 && *((char *)(v12 + (sub_4053e0(*(iter)) & 0xff))); iter = v17 + 1)
                {
LABEL_4096b6:
                }
                if (iter1 < v13)
                {
                    while (*((char *)(v12 + (sub_4053e0(*((char *)iter1)) & 0xff))))
                    {
                        if (v13 == iter1 + 1)
                            goto LABEL_409bc0;
                    }
                    if (v26 <= v17 || v13 <= iter1)
                        goto LABEL_409bc0;
                    v18 = sub_4053e0(*(v17));
                    sub_4053e0(v19[v18]);
                    v21 = v20;
                    v23 = sub_4053e0(v19[v22]) & 0xff;
                    if (v21 == v23)
                    {
                        iter1 += 1;
                        goto LABEL_4096b6;
                    }
                    else
                    {
                        v28 = v21 - v23 & 0xffffffff;
                        if (!(char)node[55])
                            return v21 - v23 & 0xffffffff;
                        goto LABEL_409b36;
                    }
                }
                else
                {
LABEL_409bc0:
                    v24 = v17;
                    v25 = v26;
                    goto LABEL_409bc3;
                }
            }
            for (; iter < v26 && *((char *)(v12 + (sub_4053e0(*(iter)) & 0xff))); iter += 1)
            {
LABEL_4096ed:
            }
            if (iter1 < v13)
            {
                while (*((char *)(v12 + (sub_4053e0(*((char *)iter1)) & 0xff))))
                {
                    if (v13 == iter1 + 1)
                        goto LABEL_409c50;
                }
                if (v26 <= iter || v13 <= iter1)
                    goto LABEL_409c50;
                v27 = sub_4053e0(*(iter)) & 0xff;
                v28 = v27 - v22;
                if ((unsigned int)v27 == (unsigned int)v22)
                {
                    iter1 += 1;
                    goto LABEL_4096ed;
                }
                goto LABEL_409880;
            }
            else
            {
LABEL_409c50:
                v24 = iter;
                v25 = v26;
LABEL_409bc3:
                v28 = (v24 < v25) - (v13 > iter1);
LABEL_40979c:
                if (!(unsigned int)v28)
                    goto LABEL_4097a4;
                goto LABEL_409880;
            }
        }
        if (!v15)
        {
            v28 = -(0 < v16);
            goto LABEL_40979c;
        }
        if (v16)
        {
            if (!v0)
            {
                if (v15 <= v16)
                    v29 = v15;
                else
                    v29 = v16;
                v28 = memcmp(iter, iter1, v29);
                if (!(unsigned int)v28)
                    goto LABEL_409ba0;
                goto LABEL_409880;
            }
            do
            {
                if (iter < v26 && iter1 < v13)
                    goto LABEL_409bfa;
                else
                    goto LABEL_409ba0;
LABEL_409bfa:
                iter1 += 1;
                v30 = iter + 1;
                v31 = sub_4053e0(*(iter));
                sub_4053e0(*((char *)v17 + v31));
                v33 = v32;
                v28 = v33 - (sub_4053e0(*((char *)v17 + (sub_4053e0(*((char *)iter1 - 1)) & 0xff))) & 0xff);
                iter = v30;
            } while ((unsigned int)v33 == (unsigned int)v34);
LABEL_409ba0:
            if (v15 >= v16)
            {
                if (v15 != v16)
                    goto LABEL_409baf;
LABEL_4097a4:
                node = (long long)node[64];
                if (!node)
                    return 0;
                iter = &index->field_0->field_0;
                v46 = index->field_8;
                if ((long long)node[16] != 0xffffffffffffffff)
                {
                    v11 = sub_408f70(iter, v46, node);
                    sub_408f70(a1->field_0, a1->field_8, node);
                }
                else
                {
                    v11 = &iter[v46] - 1;
                }
                if (*((long long *)node) != 0xffffffffffffffff)
                {
                    iter = sub_4090d0(iter, v46, node);
                    iter1 = sub_4090d0(a1->field_0, a1->field_8, node);
                }
                else
                {
                    iter1 = a1->field_0;
                    if ((char)node[48])
                    {
                        if (v11 > iter)
                        {
                            do
                            {
                            } while (v26[sub_4053e0(*(iter)) & 0xff] && (iter += 1, v11 != iter));
                        }
                        if (v13 > iter1)
                        {
                            do
                            {
                            } while (*((char *)v17 + (sub_4053e0(*((char *)iter1), &g_41e740) & 0xff)) && (iter1 += 1, v13 != iter1));
                        }
                    }
                }
            }
            else
            {
                v28 = 0xffffffff;
LABEL_409880:
                if (!(char)node[55])
                    return v28;
LABEL_409b36:
                break;
            }
        }
        else
        {
LABEL_409baf:
            v28 = 1;
            goto LABEL_409880;
        }
    }
    return -((unsigned int)v28);
}



// Function: compare @ 0x9c70
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_41e410;
extern char g_41e419;
extern char g_41e41a;
extern char g_41e861;

unsigned long long compare(struct_0 *a0, struct_0 *a1)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // r12
    unsigned long v3;  // r14
    unsigned long v4;  // r15
    unsigned long v5;  // r13
    unsigned long v6;  // r15
    unsigned long long v7;  // rax
    unsigned long v8;  // rdx

    if (g_41e410)
    {
        v1 = sub_409550(a0, a1);
        if ((unsigned int)v1)
        {
            return v1;
        }
        else if (g_41e419)
        {
            return v1;
        }
        else if (g_41e41a)
        {
            return v1;
        }
    }
    v2 = a0->field_8;
    v3 = a1->field_8;
    v4 = v2;
    v5 = v3 - 1;
    v6 = v4 - 1;
    if (v4 == 1)
    {
        v7 = -(0 < v5);
    }
    else
    {
        v7 = 1;
        if (v5)
        {
            if (!g_41e861)
            {
                if (v6 <= v5)
                    v8 = v6;
                else
                    v8 = v5;
                v7 = memcmp(a0->field_0, a1->field_0, v8);
                if (!(unsigned int)v7)
                {
                    v7 = 0xffffffff;
                    if (v6 >= v5)
                        v7 = v2 != v3;
                }
            }
            else
            {
                v7 = sub_4133e0(a0->field_0, v2, a1->field_0, v3);
            }
        }
    }
    return v7;
}



// Function: sequential_sort @ 0x9e20
void* sequential_sort(void* a0, unsigned long long a1, void* a2, char a3)
{
    unsigned long long v1;  // r14
    long long v2;  // rsi
    int v11;  // eax
    void* v12;  // rax
    void* v13;  // rdx
    int v14;  // xmm7
    int v15;  // xmm4
    int v16;  // xmm5
    unsigned long long v3;  // rax
    void* v4;  // rdi
    int v5;  // xmm3
    void* v6;  // rax
    int v7;  // xmm1
    int v8;  // xmm0
    int v9;  // xmm4
    int v10;  // xmm5

    if (a1 != 2)
    {
        v1 = a1 >> 1;
        v2 = a1 - v1;
        v3 = v1 * 32;
        v4 = a0 - v3;
        if (!a3)
        {
            sequential_sort(v4, v2, a2, 0);
            if (a1 <= 3)
            {
                v5 = (int)*((int128_t *)((char *)a0 - 16));
                *((int128_t *)((char *)a2 - 32)) = *((int128_t *)((char *)a0 - 32));
                *((uint128_t *)((char *)a2 - 16)) = (uint128_t)v5;
            }
            else
            {
                sequential_sort(a0, v1, a2, 1);
            }
        }
        else
        {
            sequential_sort(v4, v2, -(v3) + a2, 1);
            if (a1 > 3)
                sequential_sort(a0, v1, a2, 0);
        }
    }
    else
    {
        v6 = sub_409c70(a0 - 32, a0 - 64);
        if (a3)
        {
            v11 = (unsigned int)v6 > 0;
            v12 = (v11 - 2) * 32 + a0;
            v13 = ~(v11) * 32 + a0;
            v14 = (int)(int128_t)v12[16];
            v15 = (int)*((int128_t *)v13);
            v16 = (int)(int128_t)v13[16];
            *((int128_t *)((char *)a2 - 64)) = *((int128_t *)v12);
            *((uint128_t *)((char *)a2 - 32)) = (uint128_t)v15;
            *((uint128_t *)((char *)a2 - 16)) = (uint128_t)v16;
            *((uint128_t *)((char *)a2 - 48)) = (uint128_t)v14;
            return v12;
        }
        if ((unsigned int)v6 <= 0)
            return v6;
        v7 = (int)*((int128_t *)((char *)a0 - 32));
        v8 = (int)*((int128_t *)((char *)a0 - 16));
        v9 = (int)*((int128_t *)((char *)a0 - 64));
        v10 = (int)*((int128_t *)((char *)a0 - 48));
        *((uint128_t *)((char *)a2 - 32)) = (uint128_t)v7;
        *((uint128_t *)((char *)a2 - 16)) = (uint128_t)v8;
        *((uint128_t *)((char *)a0 - 32)) = (uint128_t)v9;
        *((uint128_t *)((char *)a0 - 16)) = (uint128_t)v10;
        *((uint128_t *)((char *)a0 - 64)) = (uint128_t)v7;
        *((uint128_t *)((char *)a0 - 48)) = (uint128_t)v8;
        return v6;
    }
}



// Function: write_unique @ 0x9fb0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern char g_41e419;
extern struct_0 g_41e840;
extern uint128_t g_41e850;

long long write_unique(uint128_t *a0, FILE *a1, unsigned long long a2)
{
    unsigned long v2;  // rax
    uint128_t *v0;  // [bp-0x18]

    if (g_41e419)
    {
        if (*((long long *)&g_41e840))
        {
            v0 = a0;
            v2 = sub_409c70(a0, &g_41e840.field_0);
            a0 = v0;
            if (!(unsigned int)v2)
                return v2;
        }
        *((uint128_t *)&g_41e840) = *(a0);
        g_41e850 = a0[1];
    }
    return sub_409460(a0, a1, a2);
}



// Function: mergelines_node @ 0xa020
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char *field_0;
} struct_1;

typedef struct struct_0 {
    void* field_0;
    void* field_8;
    char padding_10[8];
    void* field_18;
    struct struct_1 *field_20;
    long long field_28;
    long long field_30;
    char padding_38[24];
    unsigned int field_50;
} struct_0;

void* mergelines_node(struct_0 *idx, unsigned long long a1, FILE *a2, unsigned long long a3)
{
    void* v2;  // rbp
    void* v3;  // rdx
    char *iter;  // r14
    unsigned long long v13;  // r12
    void* v14;  // rax
    int v15;  // xmm0
    void* v16;  // rdx
    int v17;  // xmm2
    void* v18;  // rax
    long long v19;  // rdi
    long long v20;  // r9
    unsigned long long v21;  // r8
    void* v4;  // r13
    void* v22;  // rdx
    int v23;  // xmm4
    void* v24;  // r8
    unsigned long long v25;  // rcx
    void* v26;  // rdx
    void* v27;  // rdx
    int v28;  // xmm6
    long long v29;  // rax
    unsigned long long v30;  // r15
    void* v31;  // rdi
    void* v5;  // rcx
    void* v32;  // rdi
    long long v33;  // rcx
    unsigned long long v34;  // r15
    unsigned long long v35;  // r15
    void* v36;  // rdi
    unsigned long long v37;  // r15
    unsigned long long v38;  // r15
    void* v39;  // rdi
    unsigned long long v6;  // rsi
    char **v7;  // r10
    void* v8;  // rdx
    void* v9;  // rax
    long long v10;  // r11
    long long v11;  // r8
    void* v0;  // [bp-0x40]

    v2 = idx->field_0;
    v3 = idx->field_8;
    v4 = v2;
    v0 = v3;
    v5 = (void*)idx->padding_10;
    v6 = (a1 >> ((char)idx->field_50 * 2 + 2 & 63)) + 1;
    if (idx->field_50 > 1)
    {
        v7 = &idx->field_20->field_0;
        v8 = v0;
        v9 = v2;
        v10 = 0;
        v11 = 0;
        iter = *(v7);
        if (v2 != v5)
        {
            while (1)
            {
                if (idx->field_18 == v8)
                {
                    v7 = &idx->field_20->field_0;
                    v18 = idx->field_0;
                    v11 = (long long)(v4 - v9) >> 5;
                    v10 = (long long)(v0 - v8) >> 5;
                    goto LABEL_40a2dd;
                }
                v13 = v6 - 1;
                if (!v6)
                {
                    v7 = &idx->field_20->field_0;
                    v6 = 0xffffffffffffffff;
                    v18 = idx->field_0;
                    v11 = (long long)(v4 - v9) >> 5;
                    v10 = (long long)(v0 - v8) >> 5;
LABEL_40a2dd:
                    v9 = v18;
                    v19 = idx->field_30;
                    v20 = idx->field_28;
                    if (v19 != v10)
                        goto LABEL_40a12e;
                    if (v5 != v9)
                    {
                        v21 = v6 - 1;
                        if (v6)
                        {
                            v22 = v9 - 32;
                            do
                            {
                                v9 = v22;
                                v23 = (int)*((int128_t *)v9);
                                iter -= 32;
                                idx->field_0 = v9;
                                *((uint128_t *)&iter) = (uint128_t)v23;
                                *((int128_t *)&iter[16]) = (int128_t)v9[16];
                            } while (v9 != v5 && (v21 -= 1, v22 = v9 - 32, v21 != 0xffffffffffffffff));
                        }
                    }
                    goto LABEL_40a137;
                }
                iter -= 32;
                if ((unsigned int)sub_409c70(v9 - 32, v8 - 32) > 0)
                {
                    v14 = idx->field_8;
                    v5 = (void*)idx->padding_10;
                    v15 = (int)*((int128_t *)((char *)v14 - 32));
                    v8 = v14 - 32;
                    idx->field_8 = v8;
                    *((uint128_t *)&iter) = (uint128_t)v15;
                    v9 = idx->field_0;
                    *((int128_t *)&iter[16]) = *((int128_t *)((char *)v14 - 16));
                    if (v5 == v9)
                        goto LABEL_40a104;
                }
                else
                {
                    v16 = idx->field_0;
                    v5 = (void*)idx->padding_10;
                    v17 = (int)*((int128_t *)((char *)v16 - 32));
                    v9 = v16 - 32;
                    idx->field_0 = v9;
                    *((uint128_t *)&iter) = (uint128_t)v17;
                    v8 = idx->field_8;
                    *((int128_t *)&iter[16]) = *((int128_t *)((char *)v16 - 16));
                    if (v5 == v9)
                    {
LABEL_40a104:
                        v7 = &idx->field_20->field_0;
                        v11 = (long long)(v4 - v9) >> 5;
                        v10 = (long long)(v0 - v8) >> 5;
                        v6 = v13;
                        goto LABEL_40a121;
                    }
                }
                v6 = v13;
            }
            goto LABEL_40a12e;
        }
LABEL_40a121:
        v19 = idx->field_30;
        v20 = idx->field_28;
        if (v10 != v19)
        {
LABEL_40a12e:
            if (v11 == v20)
            {
                v24 = idx->field_18;
                if (v24 != v8)
                {
                    v25 = v6 - 1;
                    if (v6)
                    {
                        v26 = v8 - 32;
                        do
                        {
                            v27 = v26;
                            v28 = (int)*((int128_t *)v27);
                            iter -= 32;
                            idx->field_8 = v27;
                            *((uint128_t *)&iter) = (uint128_t)v28;
                            *((int128_t *)&iter[16]) = (int128_t)v27[16];
                            if (v27 == v24)
                            {
                                v10 = (long long)(v0 - v27) >> 5;
                                goto LABEL_40a137;
                            }
                        } while ((v25 -= 1, v26 = v27 - 32, v25 != 0xffffffffffffffff));
                        v10 = (long long)(v0 - v27) >> 5;
                    }
                }
            }
        }
LABEL_40a137:
        *(v7) = iter;
    }
    else
    {
        v19 = 0;
        v29 = 0;
        if (v2 != v5)
        {
            while (1)
            {
                if (idx->field_18 == v3)
                {
LABEL_40a217:
                    v19 = (long long)(v0 - v3) >> 5;
                    if (idx->field_30 != v19)
                    {
                        v33 = idx->field_28;
                        v29 = (long long)(v2 - v4) >> 5;
                        goto LABEL_40a237;
                    }
                    else
                    {
                        v9 = v4;
                        if (idx->padding_10 != v4)
                        {
                            v34 = v6 - 1;
                            if (v6)
                            {
                                do
                                {
                                    v35 = v34;
                                    v36 = v9 - 32;
                                    idx->field_0 = v36;
                                    sub_409fb0(v36, a2, a3);
                                    v9 = idx->field_0;
                                } while (v9 != idx->padding_10 && (v34 = v35 - 1, v35 >= 1));
                                v20 = idx->field_28;
                                idx->field_30 = idx->field_30 - ((long long)(v0 - idx->field_8) >> 5);
                                idx->field_28 = v20 - ((long long)(v2 - v9) >> 5);
                                return v9;
                            }
                        }
                        v20 = idx->field_28;
                        idx->field_30 = v19 - v19;
                        idx->field_28 = v20 - ((long long)(v2 - v9) >> 5);
                        return v9;
                    }
                }
                v30 = v6 - 1;
                if (!v6)
                {
                    v6 = 0xffffffffffffffff;
                    goto LABEL_40a217;
                }
                if ((unsigned int)sub_409c70(v4 - 32, v3 - 32) > 0)
                {
                    v31 = idx->field_8 - 32;
                    idx->field_8 = v31;
                    sub_409fb0(v31, a2, a3);
                    v4 = idx->field_0;
                    v3 = idx->field_8;
                    if (v4 == idx->padding_10)
                        goto LABEL_40a1d9;
                }
                else
                {
                    v32 = idx->field_0 - 32;
                    idx->field_0 = v32;
                    sub_409fb0(v32, a2, a3);
                    v4 = idx->field_0;
                    v3 = idx->field_8;
                    if (v4 == idx->padding_10)
                    {
LABEL_40a1d9:
                        v29 = (long long)(v2 - v4) >> 5;
                        v19 = (long long)(v0 - v3) >> 5;
                        v6 = v30;
                        goto LABEL_40a1f2;
                    }
                }
                v6 = v30;
            }
            goto LABEL_40a237;
        }
LABEL_40a1f2:
        v20 = idx->field_28;
        v33 = v20;
        if (idx->field_30 == v19)
        {
            v9 = v4;
            v10 = v19;
        }
        else
        {
LABEL_40a237:
            v20 = v33;
            if (v33 != v29 || v3 == idx->field_18 || (v37 = v6 - 1, !v6))
            {
                v9 = v4;
                v19 = idx->field_30;
                v10 = (long long)(v0 - idx->field_8) >> 5;
            }
            else
            {
                do
                {
                    v38 = v37;
                    v39 = v3 - 32;
                    idx->field_8 = v39;
                    sub_409fb0(v39, a2, a3);
                    v3 = idx->field_8;
                } while (v3 != idx->field_18 && (v37 = v38 - 1, v38 >= 1));
                v9 = idx->field_0;
                v20 = idx->field_28;
                v19 = idx->field_30;
                v10 = (long long)(v0 - v3) >> 5;
            }
        }
    }
    idx->field_30 = v19 - v10;
    idx->field_28 = v20 - ((long long)(v2 - v9) >> 5);
    return v9;
}



// Function: merge_loop @ 0xa460
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[80];
    unsigned int field_50;
} struct_0;

void merge_loop(long long *a0, unsigned long long a1, FILE *a2, unsigned long long a3)
{
    struct_0 *v1;  // rbp

    while (1)
    {
        v1 = sub_407c70(a0);
        if (!v1->field_50)
            break;
        sub_40a020(v1, a1, a2, a3);
        sub_407bc0(a0, v1);
        sub_407c10(a0, v1);
        sub_407c00(v1);
    }
    sub_407c00(v1);
    sub_407b80(a0, v1);
    return;
}



// Function: sortlines @ 0xa4e0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[84];
    char field_54;
} struct_0;

void sortlines(unsigned long a0, unsigned long long a1, unsigned long a2, struct_0 *idx, long long *a4, FILE *a5, unsigned long a6)
{
    unsigned long v8;  // r9
    unsigned long v9;  // r12
    unsigned long long v10;  // rdx
    void* v11;  // r11
    void* v12;  // r10, Other Possible Types: unsigned long long
    long long *v13;  // rax, Other Possible Types: void*, unsigned long long
    char v0;  // [bp-0x80]
    unsigned long v1;  // [bp-0x78]
    unsigned long long v2;  // [bp-0x70]
    unsigned long v3;  // [bp-0x68]
    unsigned long v4;  // [bp-0x60]
    long long *v5;  // [bp-0x58]
    FILE *v6;  // [bp-0x50]
    unsigned long v7;  // [bp-0x48]

    v8 = *((long long *)&idx->padding_0[40]);
    v9 = *((long long *)&idx->padding_0[48]);
    v1 = a0;
    v2 = a1 >> 1;
    v4 = *((long long *)&idx->padding_0[64]);
    v13 = v2;
    v3 = a2;
    v5 = a4;
    v6 = a5;
    v7 = a6;
    if (v8 + v9 > 0x1ffff)
    {
        v12 = a1;
        if (a1 > 1)
        {
            a4 = a4;
            a4 = a4;
            if (!pthread_create(&v0, 0, thread_entry, &v1))
            {
                sortlines(a0 - *((long long *)&idx->padding_0[40]) * 32, v12 - v13, a2, *((long long *)&idx->padding_0[72]), a4, a5, a6);
                pthread_join(v0, 0);
                return;
            }
            v8 = *((long long *)&idx->padding_0[40]);
            v9 = *((long long *)&idx->padding_0[48]);
        }
    }
    v10 = v8 * 32;
    v11 = a0 - a2 * 32;
    v12 = a0 - v10;
    v13 = -(v10);
    if (v9 > 1)
    {
        a4 = v8;
        v12 = v11;
        v13 = v12;
        sub_409e20(v12, v9, v11 - (v8 >> 1) * 32, 0);
        v11 = v12;
        v8 = a4;
        v12 = v13;
    }
    if (v8 > 1)
        sub_409e20(a0, v8, v11, 0);
    *((unsigned long *)&idx->padding_0[0]) = a0;
    *((void* *)&idx->padding_0[8]) = v12;
    *((void* *)&idx->padding_0[16]) = v12;
    *((unsigned long long *)&idx->padding_0[24]) = a0 + v13 - v9 * 32;
    v13 = a4;
    sub_407b80(a4, idx);
    sub_40a460(v13, a2, a5, a6);
    return;
}



// Function: sortlines_thread @ 0xa710
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned int field_10;
    char padding_14[4];
    struct struct_0 *field_18;
    long long *field_20;
    struct FILE *field_28;
    unsigned int field_30;
} struct_0;

unsigned int sortlines_thread(struct_0 *a0)
{
    sub_40a4e0(*((long long *)&a0->field_0), a0->field_8, *((long long *)&a0->field_10), a0->field_18, a0->field_20, a0->field_28, *((long long *)&a0->field_30));
    return 0;
}



// Function: fillbuf @ 0xa750
typedef struct FILE {
} FILE;

typedef struct struct_2 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
    void* field_18;
} struct_2;

typedef struct struct_5 {
    char field_0;
    char padding_1[15];
    long long field_10;
    char padding_18[8];
    unsigned long long field_20;
} struct_5;

typedef struct struct_0 {
    struct struct_1 *field_0;
    char field_8;
    char padding_9[7];
    unsigned long long field_10;
    unsigned long long field_18;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char field_30;
} struct_0;

typedef struct struct_4 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    char field_30;
} struct_4;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_41e030;
extern char g_41e038;
extern struct_4 *g_41e410;

unsigned int fillbuf(struct_0 *idx, FILE *a1, unsigned long long a2)
{
    unsigned long v7;  // r9
    unsigned long n;  // rdx
    unsigned long count;  // rax
    char *v18;  // r11
    struct_2 *iter;  // r15
    unsigned long long iter1;  // r13
    void* ptr;  // rax
    unsigned long long v22;  // rsi
    unsigned long long v23;  // r12
    unsigned long long v24;  // 4098
    struct_5 *idx1;  // rcx
    char *v26;  // r11
    struct_0 *idx2;  // rbx
    char *v27;  // r13
    struct_0 *v28;  // rcx
    char *v29;  // rdi
    unsigned long v30;  // rdx
    long long v31;  // rax
    struct_0 *v32;  // rax
    unsigned long long v10;  // r12
    char *i;  // r13
    char *v12;  // r15
    unsigned long long v13;  // rcx
    unsigned long long v14;  // rdx
    struct_2 *v15;  // rbx
    unsigned long v16;  // rax
    char *node;  // [bp-0x88]
    unsigned long v1;  // [bp-0x80]
    unsigned long long iter2;  // [bp-0x78]
    unsigned long long v3;  // [bp-0x70]
    char v4;  // [bp-0x51]
    unsigned long long v5;  // [bp-0x48]

    v4 = g_41e038;
    v1 = idx->field_28;
    if (idx->field_30)
        return 0;
    v7 = *((long long *)&idx->field_8);
    n = *((long long *)&idx->field_20);
    if (v7 != n)
    {
        idx2 = idx;
        memmove(idx2->field_0, &idx2->field_0[v7 + -1 * n], n);
        v7 = *((long long *)&idx2->field_20);
        idx2->field_10 = 0;
        *((unsigned long *)&idx2->field_8) = v7;
    }
    v10 = g_41e030 - 0x22;
    i = &idx->field_0->field_0;
    v3 = v1 + 1;
    while (1)
    {
        v12 = &i[v7];
        v13 = sub_40a740(i);
        v15 = v13 - v14 * 32;
        iter2 = v13 - v14 * v1 - v12;
        if (v14)
            i = &v15->field_0[v15->field_8].field_0;
        if (iter2 > v3)
        {
            do
            {
                v16 = (iter2 - 1) / v3;
                count = fread_unlocked(v12, 1, v16, a1);
                iter2 -= count;
                node = &v12[count];
                if (v16 != count)
                {
                    if (ferror_unlocked(a1))
                    {
                        sub_405e10(dcgettext(NULL, "read failed", 5), a2); /* do not return */
                    }
                    else if (feof_unlocked(a1))
                    {
                        idx->field_30 = 1;
                        if (idx->field_0 == node)
                        {
                            return 0;
                        }
                        else if (i != node && *(node - 1) != v4)
                        {
                            *(node) = v4;
                            node += 1;
                        }
                    }
                }
                v18 = v12;
                iter = v15;
                iter1 = iter2;
                while (1)
                {
                    ptr = memchr(v18, v4, node - v18);
                    if (!ptr)
                        break;
                    iter = (char *)iter - 32;
                    *((char *)ptr) = 0;
                    iter->field_0 = i;
                    v22 = ptr + 1 - i;
                    iter->field_8 = v22;
                    if (v10 < v22)
                    {
                        v23 = v22;
                        v10 = v23;
                    }
                    iter1 -= v1;
                    if (g_41e410)
                    {
                        if (g_41e410->field_10 != 0xffffffffffffffff)
                        {
                            iter2 = v23;
                            ptr = sub_408f70(i);
                        }
                        v24 = g_41e410->field_0;
                        if (v24 != 0xffffffffffffffff)
                        {
                            iter->field_10 = sub_4090d0(i);
                        }
                        else
                        {
                            if (g_41e410->field_30)
                            {
                                for (iter->field_18 = ptr; (&idx1->field_0)[sub_4053e0(*(i)) & 0xff]; i += 1);
                            }
                            iter->field_10 = i;
                        }
                    }
                    i = v26;
                    v18 = v26;
                }
                iter2 = iter1;
                v15 = iter;
                if (idx->field_30)
                {
                    v12 = node;
                    v27 = i;
                    goto LABEL_40aa0a;
                }
            } while ((v12 = node, iter2 > v3));
        }
        v27 = i;
LABEL_40aa0a:
        v28 = idx;
        v29 = &v28->field_0->field_0;
        v30 = v28->field_18;
        *((unsigned long *)&v28->field_8) = v12 - v29;
        v31 = sub_40a740(v29, v30, v30, v28) - (char *)v15;
        idx1->field_10 = v31 >> 5;
        if (v31 >> 5)
            break;
        v5 = v14 >> 5;
        i = sub_412ff0();
        v32 = idx;
        v7 = *((long long *)&v32->field_8);
        v32->field_0 = i;
        v32->field_18 = v5 * 32;
    }
    g_41e030 = v10 + 0x22;
    idx1->field_20 = v12 - v27;
    return 1;
}



// Function: check @ 0xaac0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
} struct_0;

extern char g_41e030;
extern FILE *stderr;
extern unsigned long long g_41e410;
extern char g_41e419;
extern unsigned long long g_41e438;

unsigned int check(unsigned long long a0, char a1)
{
    unsigned long v17;  // rax
    unsigned long long v18;  // rdx
    unsigned int v27;  // r12d
    void* v28;  // r15
    char *v29;  // rax
    unsigned long v30;  // rax
    unsigned int v19;  // ebx
    unsigned long long v20;  // r12
    void* iter;  // r14
    void* v22;  // r15
    unsigned long long v23;  // rdx
    unsigned long long v24;  // r12
    unsigned long v25;  // rax
    void* dst;  // rdi
    void* v0;  // [bp-0x100]
    unsigned long v1;  // [bp-0xf8]
    unsigned long v2;  // [bp-0xf0]
    struct_0 *v3;  // [bp-0xe8]
    unsigned long long v4;  // [bp-0xe0]
    unsigned long long v5;  // [bp-0xd8]
    unsigned long long v6;  // [bp-0xd0]
    struct_0 *v7;  // [bp-0xc0]
    unsigned long ptr;  // [bp-0xb8]
    unsigned long long v9;  // [bp-0xb0]
    unsigned long long v10;  // [bp-0xa8]
    unsigned long long v11;  // [bp-0xa0]
    char v12;  // [bp-0x98], Other Possible Types: unsigned long
    unsigned long long v13;  // [bp-0x88]
    long long v14;  // [bp-0x80]
    char v15;  // [bp-0x58]

    v17 = sub_406010(a0, "r");
    v18 = g_41e438;
    v1 = v17;
    v6 = g_41e410;
    v19 = g_41e419 ^ 1;
    if (g_41e438 < *((long long *)&g_41e030))
        v18 = *((long long *)&g_41e030);
    v20 = 0;
    v3 = &v12;
    sub_406240(&v12, 32, v18);
    ptr = 0;
    v5 = 0;
    v7 = &ptr;
    while (1)
    {
LABEL_40ab60:
        v2 = v12;
        if (!(char)sub_40a750(v3, v1, a0))
        {
            v27 = 1;
            break;
        }
        v0 = sub_40a740(v2, v14);
        v4 = v13;
        if (!v20 || v19 > (unsigned int)sub_409c70(v7, v0 - 32))
        {
            iter = v0 - 32;
            do
            {
                v22 = iter;
                if (v0 - v13 * 32 >= iter)
                {
                    v23 = (long long)iter[8];
                    v5 += v4;
                    if (v23 > v20)
                    {
                        do
                        {
                            v24 = v20 * 2;
                            if (!(v20 * 2))
                            {
                                v24 = v23;
                                break;
                            }
                        } while ((v20 = v24, v23 > v20));
                        v20 = v24;
                        free(ptr);
                        v25 = sub_412eb0(v20);
                        v23 = (long long)iter[8];
                        ptr = v25;
                    }
                    dst = memcpy(ptr, *((long long *)iter), v23);
                    v9 = (long long)iter[8];
                    if (v6)
                    {
                        v10 = (long long)iter[16] + dst - *((long long *)iter);
                        v11 = dst + (long long)iter[24] - *((long long *)iter);
                        goto LABEL_40ab60;
                    }
                }
            } while ((iter -= 32, v19 > (unsigned int)sub_409c70(v22, iter)));
        }
        else
        {
            v22 = v0;
            v27 = 0;
            break;
            if (a1 == 99)
            {
                v28 = v22 - 32;
                sub_40e520(((long long)(v0 - v28) >> 5) + v5, &v15);
                v29 = dcgettext(NULL, "%s: %s:%s: disorder: ", 5);
                __fprintf_chk(*((unsigned int *)&stderr), 0x1, v29);
                v30 = dcgettext(NULL, "standard error", 5);
                sub_409460(v28, stderr, v30);
                break;
            }
        }
    }
    sub_406040(v1, a0);
    free(v2);
    free(ptr);
    return v27;
}



// Function: mergefps @ 0xad90
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
    unsigned long long field_28;
    char field_30;
} struct_0;

extern unsigned long long g_41e030;
extern unsigned long long g_41e410;
extern char g_41e419;
extern unsigned long long g_41e438;

void mergefps(unsigned long a0, unsigned long long a1, unsigned long long a2, FILE *a3, unsigned long long a4, void* a5)
{
    void* ptr;  // r12
    void* ptr1;  // rbp
    unsigned long v25;  // rax
    unsigned long long idx;  // rax
    unsigned long long v27;  // r14
    unsigned long long v28;  // rax
    void* v29;  // rbx
    void* v30;  // r13
    unsigned long v31;  // rax
    char *idx1;  // r13
    unsigned long long v33;  // r14
    struct_0 ptr2;  // rdi
    unsigned long long idx2;  // rbx
    struct_0 v35;  // rax
    void* dst;  // rdi
    unsigned long long v37;  // rax
    struct_0 *v38;  // rdx
    unsigned long v39;  // rdx
    unsigned long long v40;  // r13
    unsigned long long v41;  // rax
    unsigned long long v42;  // rcx
    void* node;  // rax
    unsigned long long v44;  // rbx
    unsigned long long j;  // rbp
    unsigned long long v45;  // rax
    unsigned long long v46;  // rdx
    unsigned long long l;  // rax
    void* iter;  // rdx
    int v49;  // xmm0
    int v50;  // xmm1
    int v51;  // xmm2
    unsigned long v52;  // r9
    int v53;  // xmm3
    unsigned long v54;  // r9
    void* v19;  // r12
    unsigned long v55;  // r9
    void* m;  // rax
    void* v57;  // rax
    unsigned long long v58;  // r14
    unsigned long long v59;  // r13
    void* v60;  // rbx
    unsigned int v61;  // eax
    unsigned long long v62;  // rax
    void* k;  // rax
    void* v64;  // rax
    unsigned long v20;  // rax
    unsigned long v21;  // rdx
    void* v22;  // r13
    unsigned long long v23;  // rax
    unsigned long long v24;  // rsi
    struct_0 **v0;  // [bp-0xd8], Other Possible Types: void*
    void* index;  // [bp-0xd0], Other Possible Types: unsigned long long
    unsigned long long v2;  // [bp-0xc8]
    void* v3;  // [bp-0xc0]
    void* v4;  // [bp-0xb8]
    char *v5;  // [bp-0xb0], Other Possible Types: void*
    unsigned long long iter1;  // [bp-0x88]
    unsigned long long v7;  // [bp-0x80]
    char *v8;  // [bp-0x78]
    unsigned long long v9;  // [bp-0x70]
    struct_0 v10;  // [bp-0x68]
    unsigned long long v11;  // [bp-0x60]
    unsigned long long v12;  // [bp-0x58]
    void* v13;  // [bp-0x50]

    v2 = a2;
    iter1 = a1;
    v4 = sub_412fd0(a2, 56);
    ptr = sub_412fd0(a2, 8);
    v3 = sub_412fd0(a2, 8);
    v10 = (struct_0)0;
    ptr1 = sub_412fd0(a2, 8);
    v9 = g_41e410;
    if (a2)
    {
        index = ptr;
        idx2 = 0;
        while (1)
        {
            j = idx2 * 8;
            v19 = v4 + (j - idx2) * 8;
            v20 = g_41e438 / v2;
            v21 = g_41e030;
            if (g_41e030 <= v20)
                v21 = v20;
            v22 = idx2 * 16 + a0;
            sub_406240(v19, 32, v21);
            v0 = a5 + j;
            if ((char)sub_40a750(v19, *((long long *)v0), *((long long *)v22)))
            {
                v23 = sub_40a740(*((long long *)v19), (long long)v19[24]);
                *((unsigned long long *)((char *)index + 8 * idx2)) = v23 - 32;
                *((unsigned long long *)((char *)v3 + 8 * idx2)) = v23 - (long long)v19[16] * 32;
                idx2 += 1;
                if (idx2 < v2)
                    continue;
                ptr = index;
                ptr1 = v5;
                goto LABEL_40af06;
            }
            else
            {
                sub_406040(*((long long *)v0), *((long long *)v22));
                if (iter1 > idx2)
                {
                    iter1 -= 1;
                    sub_406e80(*((long long *)v22));
                }
                free(*((long long *)v19));
                v24 = v2 - 1;
                if (v24 > idx2)
                {
                    do
                    {
                        v25 = *((long long *)((char *)a5 + j + 8));
                        *((int128_t *)(a0 + j * 2)) = *((int128_t *)(a0 + j * 2 + 16));
                        *((unsigned long *)((char *)a5 + j)) = v25;
                        j += 8;
                    } while (v2 * 8 - 8 != j);
                    v2 = v24;
                }
                else
                {
                    ptr = index;
                    ptr1 = v5;
                    if (v24)
                    {
                        v2 = v24;
LABEL_40af06:
                        idx = 0;
                        do
                        {
                            *((unsigned long long *)((char *)ptr1 + 8 * idx)) = idx;
                            idx += 1;
                        } while (idx != v2);
                        if (v2 != 1)
                        {
                            v27 = 1;
                            while (1)
                            {
                                v28 = v27 * 8;
                                v29 = ptr1 + v28;
                                v30 = ptr1 + v28 - 8;
                                if ((unsigned int)sub_409c70(*((long long *)((char *)ptr + 8 * *((long long *)v30))), *((long long *)((char *)ptr + 8 * *((long long *)v29)))) > 0)
                                {
                                    v31 = *((long long *)v30);
                                    v27 = 1;
                                    *((long long *)v30) = *((long long *)v29);
                                    *((unsigned long *)v29) = v31;
                                }
                                else
                                {
                                    v27 += 1;
                                    if (v27 >= v2)
                                        break;
                                }
                            }
                        }
                        v7 = 0;
                        v5 = NULL;
                        v8 = &v10;
                        while (1)
                        {
                            idx1 = *((long long *)((char *)ptr + 8 * *((long long *)ptr1)));
                            if (g_41e419)
                            {
                                if (v5)
                                {
                                    if (!(unsigned int)sub_409c70(v5, idx1))
                                        goto LABEL_40afd6;
                                    sub_409460(v8, a3, a4);
                                }
                                v33 = *((long long *)&idx1[8]);
                                ptr2 = v10;
                                if (v33 > v7)
                                {
                                    do
                                    {
                                        if (!v7)
                                            goto LABEL_40b451;
                                    } while ((v7 *= 2, v33 > v7));
                                    v33 = v7;
LABEL_40b451:
                                    free(ptr2);
                                    v35 = (struct_0)sub_412eb0(v33);
                                    v7 = v33;
                                    v33 = *((long long *)&idx1[8]);
                                    v10 = v35;
                                    ptr2 = v10;
                                }
                                v11 = v33;
                                dst = memcpy(ptr2, *((long long *)&idx1), v33);
                                v5 = v8;
                                if (v9)
                                {
                                    v12 = *((long long *)&idx1[16]) - *((long long *)&idx1) + dst;
                                    v13 = dst + *((long long *)&idx1[24]) - *((long long *)&idx1);
                                }
                            }
                            else
                            {
                                sub_409460(idx1, a3, a4);
                            }
LABEL_40afd6:
                            index = *((long long *)ptr1);
                            if (*((long long *)((char *)v3 + 8 * index)) >= idx1)
                            {
                                if (!(char)sub_40a750(v4 + index * 56, *((long long *)((char *)a5 + 8 * index)), *((long long *)(a0 + index * 16))))
                                {
                                    v42 = *((long long *)ptr1);
                                    node = ptr1 + 8;
                                    if (v2 != 1)
                                    {
                                        do
                                        {
                                            if (*((long long *)node) > v42)
                                            {
                                                *((unsigned long long *)node) = *((long long *)node) - 1;
                                                v42 = *((long long *)ptr1);
                                            }
                                        } while ((node += 8, ptr1 + v2 * 8 != node));
                                    }
                                    v44 = v2 - 1;
                                    sub_406040(*((long long *)((char *)a5 + 8 * v42)), *((long long *)(a0 + v42 * 16)));
                                    v45 = *((long long *)ptr1);
                                    if (v45 < iter1)
                                    {
                                        iter1 -= 1;
                                        sub_406e80(*((long long *)(a0 + v45 * 16)));
                                        v45 = *((long long *)ptr1);
                                    }
                                    free(*((long long *)((char *)v4 + 56 * v45)));
                                    v46 = *((long long *)ptr1);
                                    if (v44 > v46)
                                    {
                                        l = v46 * 8;
                                        iter = v4 + (l - v46) * 8;
                                        do
                                        {
                                            v49 = (int)*((int128_t *)(a0 + l * 2 + 16));
                                            iter += 56;
                                            v50 = (int)*((int128_t *)iter);
                                            v51 = (int)(int128_t)iter[16];
                                            *((long long *)((char *)a5 + l)) = *((long long *)((char *)a5 + l + 8));
                                            v52 = (long long)iter[48];
                                            v53 = (int)(int128_t)iter[32];
                                            *((uint128_t *)(a0 + l * 2)) = (uint128_t)v49;
                                            *((unsigned long *)((char *)iter - 8)) = v52;
                                            v54 = *((long long *)((char *)ptr + l + 8));
                                            *((uint128_t *)((char *)iter - 56)) = (uint128_t)v50;
                                            *((unsigned long *)((char *)ptr + l)) = v54;
                                            v55 = *((long long *)((char *)v3 + l + 8));
                                            *((uint128_t *)((char *)iter - 40)) = (uint128_t)v51;
                                            *((unsigned long *)((char *)v3 + l)) = v55;
                                            l += 8;
                                            *((uint128_t *)((char *)iter - 24)) = (uint128_t)v53;
                                        } while (v2 * 8 - 8 != l);
                                    }
                                    if (!v44)
                                        break;
                                    m = ptr1;
                                    do
                                    {
                                        v57 = m + 8;
                                        *((long long *)((char *)v57 - 8)) = (long long)m[8];
                                        m = v57;
                                    } while (ptr1 + v2 * 8 - 8 != m);
                                    v2 = v44;
                                    continue;
                                }
                                index = *((long long *)ptr1);
                                v37 = sub_40a740(*((long long *)((char *)v4 + 56 * index)), *((long long *)((char *)v4 + 56 * index + 24)), v4 + index * 56);
                                v39 = v38->field_10;
                                v40 = v2;
                                *((unsigned long long *)((char *)ptr + 8 * index)) = v37 - 32;
                                *((unsigned long long *)((char *)v3 + 8 * index)) = v37 - v39 * 32;
                                v41 = index;
                                if (v40 == 1)
                                    goto LABEL_40b14c;
                                goto LABEL_40b086;
                            }
                            else
                            {
                                v41 = index;
                                *((char **)((char *)ptr + 8 * index)) = idx1 - 32;
                                v40 = v2;
                                if (v40 != 1)
                                {
LABEL_40b086:
                                    v58 = 1;
                                    v0 = ptr + v41 * 8;
                                    v59 = 1;
                                    while (1)
                                    {
                                        v60 = ptr1 + v58 * 8;
                                        v61 = sub_409c70(*(v0), *((long long *)((char *)ptr + 8 * *((long long *)v60))));
                                        if (v61 >= 0 && (v61 || *((long long *)v60) <= index))
                                        {
                                            v59 = v58 + 1;
                                            v58 = v59 + v40 >> 1;
                                            if (v59 >= v40)
                                                break;
                                        }
                                        else
                                        {
                                            v40 = v58;
                                            v58 = v59 + v40 >> 1;
                                            if (v59 >= v40)
                                                break;
                                        }
                                    }
                                    v62 = v59 - 1;
                                    if (v62)
                                    {
                                        k = ptr1;
                                        do
                                        {
                                            v64 = k + 8;
                                            *((long long *)((char *)v64 - 8)) = (long long)k[8];
                                            k = v64;
                                        } while (k != ptr1 + v59 * 8 - 8);
                                    }
                                    *((unsigned long long *)((char *)ptr1 + 8 * v62)) = index;
                                }
                                else
                                {
LABEL_40b14c:
                                    *((unsigned long long *)ptr1) = v41;
                                }
                            }
                        }
                        if (v5 && g_41e419)
                        {
                            sub_409460(&v10, a3, a4);
                            free(v10);
                            break;
                        }
                    }
                    break;
                }
            }
        }
    }
    sub_406040(a3, a4);
    free(a5);
    free(v4);
    free(ptr1);
    free(v3);
    free(ptr);
    return;
}



// Function: mergefiles @ 0xb500
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_0 *field_8;
} struct_1;

long long mergefiles(struct_1 *a0, unsigned long long a1, unsigned long a2, FILE *a3, unsigned long long a4)
{
    unsigned long v2;  // r12
    char *v3;  // rax
    void* v0;  // [bp-0x48], Other Possible Types: unsigned long long

    v2 = sub_407470(a0, a2, &v0);
    if (a2 <= v2)
    {
        sub_40ad90(a0, a1, v2, a3, a4, v0);
        return v2;
    }
    else if (v2 > 1)
    {
        sub_40ad90(a0, a1, v2, a3, a4, v0);
        return v2;
    }
    else
    {
        v3 = dcgettext(NULL, "open failed", 5);
        sub_405e10(v3, *((long long *)&a0[v2].field_0)); /* do not return */
    }
}



// Function: avoid_trashing_input @ 0xb5b0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_0 *field_8;
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

void avoid_trashing_input(unsigned long a0, unsigned long a1, unsigned long a2, char *a3)
{
    struct_0 *v5;  // r12
    struct_1 *iter;  // rbx
    unsigned int v7;  // ebp
    stat *v8;  // r15
    struct_0 *v9;  // rbp
    struct_1 *v0;  // [bp-0xf8]
    stat *v1;  // [bp-0xf0]
    FILE *v2;  // [bp-0xe0]
    stat v3;  // [bp-0xd8]

    if (a1 >= a2)
        return;
    v5 = NULL;
    iter = a0 + a1 * 16;
    v0 = a0 + a2 * 16;
    v1 = &v3;
    while (1)
    {
        v7 = strcmp(*((long long *)&iter->field_0), "-");
        if (!(a3 && !strcmp(a3, *((long long *)&iter->field_0))))
        {
            v8 = sub_405e70();
            if (!v8)
                return;
            if (v7)
            {
                if (stat(*((long long *)&iter->field_0), v1) || v3.st_ino != v8->st_ino)
                    goto LABEL_40b648;
LABEL_40b6a3:
                if (*((unsigned long long *)&v3) == *((long long *)&v8->st_mode))
                    goto LABEL_40b6b0;
LABEL_40b648:
                iter += 1;
                if (iter == v0)
                    return;
                continue;
            }
LABEL_40b629:
            if (fstat(0, v1) || v3.st_ino != v8->st_ino)
                goto LABEL_40b648;
            goto LABEL_40b6a3;
        }
        else if (!v7)
        {
            v8 = sub_405e70();
            if (!v8)
                return;
            goto LABEL_40b629;
        }
        else
        {
LABEL_40b6b0:
            v9 = v5 + 1;
            if (!v5)
            {
                v5 = sub_4077e0(&v2);
                v9 = v5 + 1;
                sub_40b500(iter, 0, 1, v2, v9);
            }
            *((struct_0 **)&iter->field_0) = v9;
            iter += 1;
            *((struct_0 **)((char *)iter - 8)) = v5;
            if (iter == v0)
                return;
        }
    }
}



// Function: merge @ 0xb720
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
} struct_0;

extern unsigned int g_41e024;

void merge(void* a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long iter;  // rbp
    unsigned long v8;  // rsi
    void* ptr;  // r10
    unsigned long long v18;  // rdx
    void* ptr1;  // r15
    unsigned long v20;  // rax
    unsigned long v21;  // rsi
    unsigned long v22;  // rsi
    unsigned long v23;  // rax
    unsigned long v24;  // rdx
    unsigned long v25;  // r13
    unsigned long v26;  // rax
    unsigned long long node;  // r13
    unsigned long v27;  // rax
    unsigned long v28;  // r15
    void* v29;  // rbp
    void* v30;  // rbp
    struct_0 *v31;  // rax
    unsigned long v32;  // rbp
    unsigned long v33;  // rbx
    unsigned long v34;  // r13
    char *v36;  // rax
    unsigned long long iter1;  // rbx
    struct_0 *v11;  // rax
    unsigned long v12;  // rsi
    unsigned long v13;  // rax
    unsigned long v14;  // rdx
    unsigned long long v15;  // rcx
    unsigned long j;  // rdi
    struct_0 *v0;  // [bp-0x80], Other Possible Types: unsigned long
    struct_0 *i;  // [bp-0x78], Other Possible Types: unsigned long
    char *v2;  // [bp-0x70], Other Possible Types: struct_0 *
    void* v3;  // [bp-0x60]
    void* v4;  // [bp-0x50], Other Possible Types: char
    FILE *v5;  // [bp-0x48]

    iter = a1;
    v8 = g_41e024;
    i = a2;
    if (g_41e024 < a2)
    {
        do
        {
            node = 0;
            iter1 = 0;
            if (v8 <= i)
            {
                do
                {
                    v11 = sub_4077e0(&v5);
                    v0 = v11 + 1;
                    if (g_41e024 <= iter)
                        v12 = g_41e024;
                    else
                        v12 = iter;
                    v13 = sub_40b500(iter1 * 16 + a0, v12, g_41e024, v5, v0);
                    v8 = g_41e024;
                    if (v13 <= iter)
                        v14 = v13;
                    else
                        v14 = iter;
                    iter1 += v13;
                    v15 = node * 16;
                    node += 1;
                    j = i - iter1;
                    *((struct_0 **)((char *)a0 + v15)) = v0;
                    *((struct_0 **)((char *)a0 + v15 + 8)) = v11;
                    iter -= v14;
                } while (g_41e024 <= j);
                ptr = a0 + v15 + 16;
                v18 = node % g_41e024;
                ptr1 = iter1 * 16 + a0;
                v20 = g_41e024 - v18;
            }
            else
            {
                j = i;
                v20 = v8;
                ptr = a0;
                ptr1 = a0;
                v18 = 0;
            }
            if (j > v20)
            {
                v3 = ptr;
                v0 = j + v18 + 1 - v8;
                v2 = sub_4077e0(&v5);
                v0 = v2 + 1;
                if (v0 <= iter)
                {
                    v21 = v0;
                    v22 = v21;
                }
                else
                {
                    v22 = iter;
                }
                v23 = sub_40b500(ptr1, v22, v21, v5, v0);
                if (v23 <= iter)
                    v24 = v23;
                else
                    v24 = iter;
                node += 1;
                iter1 += v23;
                *((struct_0 **)v3) = v0;
                *((struct_0 **)&v3[8]) = v2;
                iter -= v24;
                ptr1 = iter1 * 16 + a0;
                ptr = node * 16 + a0;
            }
            iter += node;
            memmove(ptr, ptr1, (i - iter1) * 16);
            v8 = g_41e024;
            i += node - iter1;
        } while (g_41e024 < i);
    }
    v25 = i;
    sub_40b5b0(a0, iter, v25, a3);
    v2 = &v4;
    while (1)
    {
        v26 = sub_407470(a0, v25, v2);
        if (v25 == v26)
        {
            v27 = sub_405ee0(a3, "w");
            if (v27)
            {
                sub_40ad90(a0, v0, v25, v27, a3, v4);
                return;
            }
            if (*(__errno_location()) != 24 || v25 <= 2)
                break;
        }
        else if (v26 <= 2)
        {
            v36 = dcgettext(NULL, "open failed", 5);
            sub_405e10(v36, *((long long *)((char *)a0 + 16 * v26))); /* do not return */
        }
        v28 = v26 - 1;
        v29 = a0 + v28 * 16;
        while (1)
        {
            sub_406040(*((long long *)(v4 + v28 * 8)), *((long long *)v29));
            v30 = v29 - 16;
            if (sub_4076b0(&v5, v28 > 2))
                break;
            v28 -= 1;
            v29 = v30;
        }
        v32 = v0;
        i = v31;
        v0 = v31 + 1;
        if (v32 > v28)
            v33 = v28;
        else
            v33 = v32;
        v34 = v25 - v28;
        sub_40ad90(a0, v33, v28, v5, v0, v4);
        v25 = v34 + 1;
        *((struct_0 **)a0) = v0;
        *((struct_0 **)&a0[8]) = i;
        memmove(a0 + 16, v29, v34 * 16);
        v0 = v32 - v33 + 1;
    }
    sub_405e10(dcgettext(NULL, "open failed", 5), a3); /* do not return */
}



// Function: sort @ 0xbac0
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_41e368;
extern unsigned long long g_41e840;

void sort(unsigned long long *a0, long long a1, unsigned long a2, unsigned long long j)
{
    unsigned long long v18;  // rbx
    unsigned long v19;  // rax
    void* ptr;  // rbp
    struct_0 **v29;  // rbx
    unsigned long long v20;  // rdx
    unsigned long long v21;  // rax
    char v23;  // bl
    uint128_t *v24;  // rbp
    unsigned long v25;  // r14
    struct_0 **i;  // rbx
    void* iter;  // rax
    long long v0;  // [bp-0x138]
    unsigned long v1;  // [bp-0x130]
    void* v2;  // [bp-0x128]
    char *v3;  // [bp-0x120]
    long long v4;  // [bp-0x118]
    unsigned long long v5;  // [bp-0x110]
    unsigned long long *node;  // [bp-0x108]
    unsigned long v7;  // [bp-0xf8]
    unsigned long v8;  // [bp-0xf0]
    unsigned long v9;  // [bp-0xf0]
    char ptr1;  // [bp-0xe8]
    unsigned long long v11;  // [bp-0xe0]
    unsigned long long v12;  // [bp-0xd8]
    unsigned long long v13;  // [bp-0xd0]
    unsigned long long v14;  // [bp-0xc8]
    char v15;  // [bp-0xb8]
    char v16;  // [bp-0xa8]

    node = a0;
    v4 = a1;
    v13 = 0;
    if (a1)
    {
        v0 = 0;
        v18 = 0;
        v3 = &ptr1;
        while (1)
        {
            v19 = sub_406010(*(node), "r");
            v5 = 48;
            v7 = v19;
            if (j > 1)
            {
                v20 = 1;
                v21 = 1;
                do
                {
                    v21 *= 2;
                    v20 += 1;
                } while (j > v21);
                v5 = v20 * 32;
                if (v18)
                    goto LABEL_40bb57;
            }
            else if (!(!v18))
            {
                goto LABEL_40bb57;
            }
            sub_406240(v3, v5, sub_408700(&v7, node, v4, v5));
            v19 = v7;
LABEL_40bb57:
            v4 -= 1;
            v15 = 0;
            v1 = *(node);
            v9 = v8;
            while (1)
            {
                if (!(char)sub_40a750(v3, v19, v1))
                {
                    sub_406040(v7, v1);
                    if (v4)
                        goto LABEL_40bd03;
                    v4 = v0;
                    goto LABEL_40bdbd;
                }
                v23 = v15;
                if (!v23)
                {
LABEL_40bc43:
                    g_41e840 = 0;
                    v24 = sub_40a740(ptr1);
                    goto LABEL_40bc5b;
                }
                if (v4)
                    break;
                g_41e840 = 0;
                v24 = sub_40a740(ptr1);
                if (v0 || v14)
                {
LABEL_40bc5b:
                    v0 += 1;
                    v23 = 0;
                    v25 = sub_4077e0(&v9) + 1;
                    goto LABEL_40bc70;
                }
                else
                {
                    sub_406040(v7, v1);
                    v25 = a2;
                    v0 = 0;
                    v9 = sub_406010(v25, "w");
LABEL_40bc70:
                    if (v12 > 1)
                    {
                        sub_407b40(&v16, j);
                        v2 = sub_407aa0(j, v12, *((unsigned int *)&v24));
                        sub_40a4e0(v24, j, v12, v2 + 128, &v16, v9, v25);
                        sub_407cd0(j, v2);
                        sub_407d10(&v16);
                    }
                    else
                    {
                        sub_409fb0((char *)v24 - 32, v9, v25);
                    }
                    sub_406040(v9, v25);
                    if (!v23)
                        v19 = v7;
                    else
                        free(ptr1);
                }
            }
            if (v5 + 1 >= v13 - v11 - v12 * v5)
                goto LABEL_40bc43;
            v14 = v11;
            sub_406040(v7, v1);
LABEL_40bd03:
            node += 1;
            v18 = v13;
        }
    }
LABEL_40bdbd:
    free(ptr1);
    i = g_41e368;
    iter = sub_412fd0(v4, 16);
    ptr = iter;
    if (g_41e368)
    {
        do
        {
            *((struct_0 ***)&iter[8]) = i;
            v29 = *(i);
            iter += 16;
            *((struct_0 ***)((char *)iter - 16)) = (char *)&i[1] + 5;
            i = v29;
        } while (i);
    }
    sub_40b720(ptr, v4, v4, a2);
    free(ptr);
}



// Function: usage @ 0xbe50
typedef struct FILE {
} FILE;

extern unsigned long long stderr;
extern FILE *stdout;

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
    char *v21;  // rax
    char *v22;  // rax
    char *v23;  // rax
    char *v25;  // rax
    char *v26;  // rax
    char *v27;  // rax
    char *v28;  // rax
    char *v29;  // rax
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
        v5 = dcgettext(NULL, "Write sorted concatenation of all FILE(s) to standard output.\n", 5);
        fputs_unlocked(v5, stdout);
        sub_4058d0(v6, v7, v8, v9, v10, v11);
        sub_405900(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "Ordering options:\n\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n", 5);
        fputs_unlocked(v13, stdout);
        v14 = dcgettext(NULL, "  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < 'JAN' < ... < 'DEC'\n", 5);
        fputs_unlocked(v14, stdout);
        v15 = dcgettext(NULL, "  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n", 5);
        fputs_unlocked(v15, stdout);
        v16 = dcgettext(NULL, "  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n", 5);
        fputs_unlocked(v16, stdout);
        v17 = dcgettext(NULL, "      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n", 5);
        fputs_unlocked(v17, stdout);
        v18 = dcgettext(NULL, "Other options:\n\n", 5);
        fputs_unlocked(v18, stdout);
        v19 = dcgettext(NULL, "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n", 5);
        fputs_unlocked(v19, stdout);
        v20 = dcgettext(NULL, "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n", 5);
        fputs_unlocked(v20, stdout);
        v21 = dcgettext(NULL, "      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n", 5);
        fputs_unlocked(v21, stdout);
        v22 = dcgettext(NULL, "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n", 5);
        fputs_unlocked(v22, stdout);
        v23 = dcgettext(NULL, "  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n", 5);
        fputs_unlocked(v23, stdout);
        __printf_chk(1, dcgettext(NULL, "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n", 5));
        v25 = dcgettext(NULL, "  -z, --zero-terminated     line delimiter is NUL, not newline\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line's end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n", 5);
        fputs_unlocked(v29, stdout);
        sub_408d30(v6, v7, v8, v9, v10, v11);
    }
    exit(a0); /* do not return */
}



// Function: is_prime @ 0xd040
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



// Function: next_prime @ 0xd0b0
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
    while (!sub_40d040(v2, v3))
    {
        v2 = v4 + 2;
        if (v2 == 0xffffffffffffffff)
            return v4;
    }
    return v4;
}



// Function: raw_hasher @ 0xd0f0
long long raw_hasher(unsigned long a0, unsigned long long a1)
{
    return sub_413a80() % a1;
}



// Function: raw_comparator @ 0xd110
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0xd120
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

extern unsigned int g_417200[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_417200[0])
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
    idx->field_28 = &g_417200[0];
    return 0;
}



// Function: free_entry @ 0xd1b0
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



// Function: safe_hasher @ 0xd1d0
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



// Function: hash_find_entry @ 0xd200
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

    v2 = sub_40d1d0(a0, a1);
    *(a2) = v2;
    v3 = *((long long *)v2);
    if (!v3)
    {
LABEL_40d2a1:
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
                    sub_40d1b0(a0, v6);
                    return v0;
                }
                return *((long long *)v6);
            }
        }
        goto LABEL_40d2a1;
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
    sub_40d1b0(a0, v7);
    return v0;
}



// Function: allocate_entry @ 0xd300
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



// Function: transfer_entries @ 0xd330
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
LABEL_40d358:
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
                    idx2 = sub_40d1d0(idx, *((long long *)&v3->field_0));
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
                        sub_40d1b0(idx, v5);
                        v3 = v6;
                        if (!v3)
                            break;
                    }
                }
                v2 = iter->field_0;
            }
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_40d358;
            index = sub_40d1d0(idx, v2);
            if (index->field_0)
            {
                v8 = sub_40d300(idx);
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



// Function: hash_get_n_buckets @ 0xd4f0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0xd500
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0xd510
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0xd520
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



// Function: hash_print_statistics @ 0xd5f0
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
    sub_40d5fd();
    return;
}



// Function: hash_lookup @ 0xd710
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

char * hash_lookup(struct_0 *a0, unsigned long long a1)
{
    struct_0 *iter;  // rax
    unsigned long long i;  // rsi
    char v0;  // [bp+0x0]

    iter = sub_40d1d0(a0, a1);
    if (!iter->field_0)
        return NULL;
    for (i = iter->field_0; i != a1; i = iter->field_0)
    {
        if (!(char)a0[1].field_0(a1))
        {
            iter = (struct_0 *)iter->padding_8;
            if (!iter)
                return NULL;
        }
        else
        {
            break;
        }
    }
    return &v0;
}



// Function: hash_get_next @ 0xd7c0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40d7cc();
    return;
}



// Function: hash_do_for_each @ 0xd880
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



// Function: hash_initialize @ 0xd970
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

extern struct_0 g_417200;

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

    v1 = sub_40d0f0;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_40d110;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_417200.field_0;
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_40d120(ptr))
    {
        v6 = sub_40d460(a0, v2->field_10);
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



// Function: hash_clear @ 0xda80
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



// Function: hash_free @ 0xdb30
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
            goto LABEL_40dbdc;
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
LABEL_40dbdc:
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



// Function: hash_rehash @ 0xdc20
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

    v11 = sub_40d460(a1, idx->field_28->field_10);
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
        v13 = sub_40d330(&ptr, idx, 0);
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
        if ((char)sub_40d330(idx, &ptr, 1) && (char)sub_40d330(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0xdda0
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
    v2 = sub_40d200(idx, a1, &index, 0);
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
            sub_40d120(idx);
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
                else if (!(char)sub_40dc20(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else if (sub_40d200(idx, a1, &index, 0))
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
        v11 = sub_40d300(idx);
        if (!v11)
            return 0xffffffff;
        v11->field_0 = a1;
        v11->field_8 = index->field_8;
        index->field_8 = v11;
        *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
        return 1;
    }
}



// Function: hash_insert @ 0xdf90
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x18]

    v2 = sub_40dda0(a0, a1, &v0);
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



// Function: hash_remove @ 0xdff0
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

    v2 = sub_40d200(idx, a1, &v0, 1);
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
            goto LABEL_40e136;
LABEL_40e080:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = idx[2].field_0;
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_40e080;
LABEL_40e136:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_40d120(idx);
    v9 = idx[5].field_0;
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_40dc20(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



// Function: parse_omp_threads @ 0xe7d0
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

unsigned long long parse_omp_threads(struct_0 *a0)
{
    struct_0 *v2;  // r12
    char v3;  // dil
    struct_0 *v4;  // r12
    unsigned int v5;  // ebp
    unsigned long long num;  // rax
    char *v7;  // rbp
    char v8;  // r12b
    char *v9;  // rbp
    char *v10;  // rbp
    char *v0;  // [bp-0x28]

    if (!a0)
        return 0;
    v2 = a0;
    v3 = a0->field_0;
    if (a0->field_0)
    {
        do
        {
            v4 = v2;
            v5 = v3;
            if (!(char)sub_413c00(v5))
            {
                if (!(char)sub_413b70(v5))
                    return 0;
                goto LABEL_40e860;
            }
        } while ((v3 = v4->field_1, v2 = v4 + 1, v4->field_1));
    }
    v4 = v2;
    if (!(char)sub_413b70(0))
        return 0;
LABEL_40e860:
    v0 = NULL;
    num = strtoul(v4, &v0, 10);
    v7 = v0;
    if (v7)
    {
        v8 = *(v7);
        if (!*(v7))
            return num;
        v9 = v7 + 1;
        while (1)
        {
            v10 = v9;
            if (!(char)sub_413c00(v8))
                break;
            v0 = v10;
            v8 = *(v10);
            v9 = v10 + 1;
            if (!*(v10))
                return num;
        }
        if (v8 == 44)
            return num;
    }
    return 0;
}



// Function: num_processors_via_affinity_mask @ 0xe8d0
unsigned long long num_processors_via_affinity_mask(void)
{
    unsigned long long v2;  // rax
    char v0;  // [bp-0x98]

    if (!sched_getaffinity(0, 128, &v0))
    {
        v2 = __sched_cpucount(128, &v0);
        if (v2)
            return v2;
    }
    return 0;
}



// Function: num_processors_ignoring_omp @ 0xe940
long long num_processors_ignoring_omp(unsigned int a0)
{
    unsigned long v1;  // rbx
    unsigned long v2;  // rax
    unsigned long v3;  // rax
    unsigned long v4;  // rax

    if (a0 != 1)
    {
        v1 = sysconf(83);
        if (v1 - 1 <= 1)
        {
            v2 = sub_40e8d0();
            if (v1 >= v2)
                return v1;
            v1 = v2;
        }
        if (v1 > 0)
            return v1;
    }
    else
    {
        v3 = sub_40e8d0();
        if (v3)
            return v3;
        v4 = sysconf(84);
        if (v4 > 0)
            return v4;
    }
    return 1;
}



// Function: num_processors @ 0xe9d0
long long num_processors(unsigned int a0)
{
    unsigned long long v2;  // rbx
    unsigned long v3;  // rbx
    unsigned long v4;  // rbp
    unsigned long v5;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    v3 = 0xffffffffffffffff;
    if (a0 == 2)
    {
        v4 = sub_40e7d0(getenv("OMP_NUM_THREADS"));
        v3 = sub_40e7d0(getenv("OMP_THREAD_LIMIT"));
        a0 = 1;
        if (!v3)
            v3 = 0xffffffffffffffff;
        if (v4)
        {
            if (v3 <= v4)
                return v3;
            return v4;
        }
    }
    v5 = sub_40e940(a0);
    if (v3 > v5)
        return v5;
    return v3;
}



// Function: main @ 0x3ec0
typedef struct sigaction {
} sigaction;

typedef struct struct_4 {
    struct struct_3 *field_0;
    long long field_8;
} struct_4;

typedef struct struct_6 {
    char padding_0[20];
    unsigned short field_14;
} struct_6;

typedef struct struct_1 {
    char padding_0[40];
    unsigned int field_28;
} struct_1;

typedef struct struct_0 {
    char padding_0[50];
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char padding_37[1];
    char field_38;
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

typedef struct struct_8 {
    struct struct_8 *field_0;
} struct_8;

typedef struct struct_7 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    long long field_28;
    long long field_30;
    unsigned long long field_38;
    char padding_40[16];
    unsigned int field_50;
    char field_54;
} struct_7;

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

typedef struct struct_3 {
    char field_0;
} struct_3;

typedef struct struct_2 {
    unsigned long long field_0;
    unsigned int field_8;
} struct_2;

extern void g_4150cc;
extern int g_415324;
extern int g_41534c;
extern char g_416c20;
extern long long g_41c580;
extern char g_41c5e0;
extern char g_41d010;
extern unsigned int g_41d024;
extern unsigned int g_41d028;
extern char g_41d038;
extern struct_2 g_41d060;
extern long long g_41d120;
extern long long g_41d128;
extern unsigned int g_41d130;
extern unsigned int optind;
extern long long stdin;
extern struct_3 *optarg;
extern long long stderr;
extern long long stdout;
extern unsigned long long g_41d240;
extern long long g_41d300;
extern int g_41d35c;
extern struct_8 *g_41d368;
extern uint128_t g_41d380;
extern uint128_t g_41d390;
extern uint128_t g_41d3a0;
extern uint128_t g_41d3b0;
extern uint128_t g_41d3c0;
extern uint128_t g_41d3d0;
extern uint128_t g_41d3e0;
extern uint128_t g_41d3f0;
extern char g_41d400;
extern unsigned long long g_41d408;
extern void* g_41d410;
extern char g_41d418;
extern char g_41d419;
extern char g_41d41a;
extern char g_41d41b;
extern unsigned long long g_41d428;
extern unsigned long long g_41d438;
extern char g_41d440;
extern char g_41d44a;
extern char g_41d540;
extern char g_41d54a;
extern char g_41d640;
extern char g_41d64a;
extern char g_41d740;
extern char g_41d74a;
extern unsigned long long g_41d840;
extern char g_41d860;
extern char g_41d861;
extern char g_41d862;
extern unsigned int g_41d864;
extern char g_41d868;

unsigned int main(unsigned int a0, long long *a1)
{
    long long *v73;  // rbp
    long long v74;  // rdi
    struct_6 **v83;  // rbx
    unsigned int v173;  // r14d
    void* iter;  // rbx
    unsigned int v175;  // r13d
    char v176;  // al
    unsigned long v177;  // r12
    char *v178;  // rax
    char *v179;  // rax
    unsigned long v181;  // rax
    char v182;  // cl
    struct_1 **v84;  // r8
    unsigned int v184;  // esi
    unsigned long long v185;  // rdx
    unsigned long v187;  // rax
    char v189;  // al
    char v190;  // cc_dep1
    char v191;  // dil
    char v192;  // cl
    unsigned long long iter1;  // rax
    struct_6 **v193;  // rbx
    unsigned int v194;  // eax
    struct_6 *v195;  // rax
    char *v197;  // rsi
    char v202;  // bl
    unsigned short v86;  // dx
    long long v207;  // rax
    unsigned long v208;  // rax
    unsigned long v209;  // rdx
    unsigned long v210;  // rax
    unsigned long v211;  // r14
    int v212;  // ebx
    char v87;  // dil
    unsigned long long v213;  // r12
    void* v214;  // rdi
    void* node;  // r13
    void* v216;  // r15
    unsigned long long v217;  // rdx
    unsigned long long v218;  // r12
    void* v219;  // rax
    void* dst;  // rdi
    char v221;  // bl
    void* v222;  // r15
    char v88;  // dl
    char *v223;  // rax
    unsigned long v224;  // rax
    void* v226;  // r12
    struct_4 *iter2;  // rbx
    char *v228;  // rax
    unsigned int v229;  // eax
    void* v230;  // r8
    void* v231;  // rax
    void* v232;  // rax
    char v89;  // sil
    unsigned long long v233;  // rdx
    unsigned long long v234;  // rax
    void* idx;  // r14
    char *v236;  // r15
    unsigned long long v237;  // rbp
    unsigned long long v238;  // r12
    char v239;  // al
    unsigned long v240;  // rsi
    unsigned long long v241;  // rdi
    unsigned long long v242;  // rdx
    unsigned short v90;  // dx
    unsigned long long v243;  // rdx
    unsigned long v244;  // rdx
    unsigned long long v245;  // rax
    int v246;  // xmm0
    int v247;  // xmm1
    unsigned long long v248;  // rcx
    char v249;  // bl
    void* v250;  // rax
    long long v251;  // r13
    long long v252;  // r14
    struct_1 *v91;  // rdx
    char *v253;  // r12
    unsigned long long v254;  // rax
    struct_7 *v255;  // rax
    struct_7 *ptr;  // rbp
    unsigned long long n;  // r14
    struct_7 *v258;  // r13
    struct_4 *v259;  // r13
    struct_8 **i0;  // rbx
    void* v261;  // rax
    void* ptr1;  // rbp
    char v92;  // dl
    struct_8 **v263;  // rbx
    unsigned long v265;  // rbp
    char *v266;  // rsi
    void* v267;  // rax
    struct_4 *k;  // rbp
    struct_4 *v269;  // rbp
    char *v272;  // rsi
    long long v75;  // rsi
    unsigned long long v93;  // r15
    struct_2 *v94;  // rbp
    char *v95;  // r14
    unsigned long len;  // r13
    unsigned long long v97;  // rax
    unsigned long v98;  // r13
    struct_6 *v99;  // r9
    char *v100;  // rdx
    unsigned long v101;  // rsi
    int *v102;  // r13
    long long v76;  // rdx
    int *v103;  // r12
    int v104;  // ebp
    int v105;  // ebp
    long long *v106;  // rbp
    void* v107;  // r15
    uint128_t v108;  // xmm0
    unsigned int v109;  // r9d
    struct_4 *ptr2;  // r13
    void* v111;  // rcx
    unsigned long idx1;  // rax
    void* index;  // rcx
    unsigned long v113;  // rdx
    unsigned long v114;  // rax
    unsigned int v115;  // eax
    unsigned int v116;  // eax
    char *v117;  // rdx
    unsigned int v118;  // r10d
    unsigned int v119;  // edx
    unsigned long v120;  // rax
    char *v121;  // rdx
    char *v122;  // rdx
    struct_0 *idx2;  // r8
    unsigned int v123;  // ecx
    unsigned int v124;  // eax
    char *v125;  // rdi
    char *v126;  // rdx
    unsigned int *v127;  // rdi
    unsigned long long v128;  // rcx
    unsigned int v129;  // r9d
    char *v130;  // rdi
    char *v131;  // rax
    unsigned int v132;  // r9d
    long long v79;  // r9
    unsigned int v133;  // r9d
    char *v134;  // rdi
    unsigned int v135;  // eax
    char *v136;  // rdi
    void* v137;  // rdx
    unsigned long v138;  // rax
    unsigned long v139;  // rax
    unsigned long v140;  // rdx
    char *v141;  // rdx
    char v142;  // al
    lconv *v80;  // rax
    unsigned int *v143;  // rdi
    unsigned long long v144;  // rcx
    char *v145;  // rdi
    unsigned long v146;  // rax
    char *v147;  // rax
    unsigned int v148;  // r9d
    char *v149;  // rax
    char *v81;  // rcx
    char *v160;  // rax
    long long v161;  // rbp
    void* m;  // rbp
    char *v82;  // rdx
    char *v163;  // r12
    unsigned int v164;  // ebp
    unsigned int v165;  // eax
    char *v168;  // rdi
    unsigned long long v169;  // rcx
    char *i;  // rsi
    unsigned int v171;  // ebp
    unsigned int v172;  // r15d
    unsigned long long v0;  // [bp-0x468]
    unsigned long long v1;  // [bp-0x460]
    struct_4 *v2;  // [bp-0x458], Other Possible Types: void*, unsigned int, long long, char
    long long v3;  // [bp-0x450], Other Possible Types: unsigned long long, char
    long long *v4;  // [bp-0x448], Other Possible Types: char, unsigned long, unsigned long long
    void* l;  // [bp-0x440], Other Possible Types: long long, char
    char *v6;  // [bp-0x438], Other Possible Types: char, unsigned long long
    void* v7;  // [bp-0x430], Other Possible Types: char *, char, unsigned long, unsigned long long
    struct_7 *v8;  // [bp-0x428], Other Possible Types: unsigned int *, char *, char, unsigned long, unsigned int
    char *v9;  // [bp-0x420], Other Possible Types: unsigned long, unsigned long long
    char *v10;  // [bp-0x418], Other Possible Types: struct_7 *, char, unsigned long, unsigned int
    char *v11;  // [bp-0x410]
    void* v12;  // [bp-0x408], Other Possible Types: unsigned int
    char *v13;  // [bp-0x400], Other Possible Types: unsigned int, unsigned long long
    char *v14;  // [bp-0x3f8]
    unsigned short *v15;  // [bp-0x3f0], Other Possible Types: unsigned long long
    unsigned long long v16;  // [bp-0x3e8], Other Possible Types: unsigned long
    char *v17;  // [bp-0x3e0], Other Possible Types: unsigned long long
    char *v18;  // [bp-0x3d8], Other Possible Types: struct_4 *
    void* v19;  // [bp-0x3d0]
    char *v20;  // [bp-0x3c8], Other Possible Types: unsigned long
    char v21;  // [bp-0x3c0], Other Possible Types: long long
    char v22;  // [bp-0x3b3]
    char v23;  // [bp-0x3b2]
    char v24;  // [bp-0x3b1]
    struct_6 **v25;  // [bp-0x3a0]
    void* v26;  // [bp-0x398], Other Possible Types: unsigned long long
    unsigned long long v27;  // [bp-0x390]
    unsigned long long v28;  // [bp-0x388]
    unsigned long long v29;  // [bp-0x380]
    unsigned long long v30;  // [bp-0x378]
    unsigned long v31;  // [bp-0x370]
    unsigned long v32;  // [bp-0x370]
    unsigned long long v33;  // [bp-0x368], Other Possible Types: unsigned long
    unsigned long long v34;  // [bp-0x360]
    char v35;  // [bp-0x33f]
    unsigned long v36;  // [bp-0x328]
    uint128_t v37;  // [bp-0x320]
    unsigned long long v38;  // [bp-0x318]
    uint128_t v39;  // [bp-0x310]
    uint128_t v40;  // [bp-0x300]
    uint128_t v41;  // [bp-0x2f0]
    unsigned long v42;  // [bp-0x2d8]
    unsigned long long v43;  // [bp-0x2d0]
    long long v44;  // [bp-0x2c8]
    void* v45;  // [bp-0x2c0], Other Possible Types: unsigned long
    unsigned long long v46;  // [bp-0x2b8]
    unsigned long long v47;  // [bp-0x2b0]
    char v48;  // [bp-0x2a7]
    char v49;  // [bp-0x2a6]
    char v50;  // [bp-0x2a5]
    char v51;  // [bp-0x2a4]
    char v52;  // [bp-0x2a3]
    char v53;  // [bp-0x2a1]
    char v54;  // [bp-0x2a0]
    void* v55;  // [bp-0x288], Other Possible Types: char
    struct_4 *v56;  // [bp-0x280]
    sigaction v57;  // [bp-0x168], Other Possible Types: stat, long long
    int v58;  // [bp-0x160]
    int v59;  // [bp-0x150]
    int v60;  // [bp-0x140]
    int v61;  // [bp-0x130]
    int v62;  // [bp-0x120]
    int v63;  // [bp-0x110]
    int v64;  // [bp-0x100]
    int v65;  // [bp-0xf0]
    unsigned int v66;  // [bp-0xe0]
    char v67;  // [bp-0xc8]
    char v68;  // [bp-0xa8]
    char v69;  // [bp-0xa7]
    unsigned int v70;  // [bp-0x78], Other Possible Types: unsigned short, char
    char v71;  // [bp-0x77]

    v73 = a1;
    *((unsigned int *)&v2) = a0;
    v9 = getenv("POSIXLY_CORRECT");
    *((char *)&v3) = v9;
    l = 696 < sub_40eb70(v74, v75, v76, index, idx2, v79, v2, v3) - 200112;
    sub_40ec00(*(v73));
    v7 = setlocale(6, &g_416c20);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    g_41d130 = 2;
    g_41d861 = sub_40cc40(3);
    g_41d860 = sub_40cc40(2);
    v80 = localeconv();
    v81 = v80->decimal_point;
    g_41d868 = *(v81);
    if (!g_41d868 || v81[1])
        g_41d868 = 46;
    v82 = v80->thousands_sep;
    g_41d864 = *(v82);
    if (g_41d864)
    {
        if (!v82[1])
            goto LABEL_403fc7;
        g_41d862 = 1;
    }
    g_41d864 = 128;
LABEL_403fc7:
    g_41d418 = 0;
    v83 = __ctype_b_loc();
    v84 = __ctype_toupper_loc(v74, v75, v76, index, idx2, v79);
    iter1 = 0;
    while (1)
    {
        v86 = *((short *)&*(v83)->padding_0[2 * iter1]);
        if ((char)v86 & 1)
        {
            v87 = 1;
            v88 = 0;
            v89 = ((char)(v86 >> 14) ^ 1) & 1;
LABEL_40402e:
            (&g_41d540)[iter1] = v88;
            v91 = *(v84);
            (&g_41d740)[iter1] = v87;
            v92 = *((int *)&v91->padding_0[4 * iter1]);
            (&g_41d640)[iter1] = v89;
            (&g_41d440)[iter1] = v92;
            if (iter1 == 0xff)
                break;
            else
                goto LABEL_40404c;
        }
        else if (iter1 != 10)
        {
            v87 = 0;
            v88 = ((char)(v86 >> 3) ^ 1) & 1;
            v89 = ((char)(v86 >> 14) ^ 1) & 1;
            goto LABEL_40402e;
        }
        else
        {
            v90 = *(v83)->field_14;
            g_41d74a = 1;
            g_41d54a = 0;
            g_41d64a = ((char)(v90 >> 14) ^ 1) & 1;
            g_41d44a = *(v84)->field_28;
LABEL_40404c:
            iter1 += 1;
        }
    }
    if (g_41d860)
    {
        v4 = v73;
        v93 = 1;
        v94 = &g_41d060.field_0;
        do
        {
            v95 = nl_langinfo((unsigned int)v93 + 131085);
            len = strlen(v95);
            v97 = sub_412ab0(len + 1);
            v94->field_8 = v93;
            v94->field_0 = v97;
            if (len)
            {
                v99 = *(v83);
                v100 = v95;
                v98 = 0;
                do
                {
                    v101 = *(v100);
                    if (!(v99->padding_0[2 * v101] & 1))
                    {
                        *((char *)(v97 + v98)) = (&g_41d440)[v101];
                        v98 += 1;
                    }
                } while ((v100 += 1, v95 + len != v100));
            }
        } while ((v93 += 1, *((char *)(v97 + v98)) = 0, v94 += 16, v93 != 13));
        v73 = v4;
        qsort(&g_41d060.field_0, 12, 16, sub_407300);
    }
    v102 = &g_415324;
    v103 = &g_415324;
    sigemptyset(&g_41d380);
    v4 = v73;
    v104 = 14;
    v11 = &v57;
    while (1)
    {
        sigaction(v104, NULL, &v57);
        if (*((unsigned long long *)&v57) != 1)
            sigaddset(&g_41d380, v104);
        if (v103 == &g_41534c)
            break;
        v104 = *(v103);
        v103 += 1;
    }
    v66 = 0;
    v57 = (stat)sub_408270;
    v105 = 14;
    v58 = (int)g_41d380;
    v59 = (int)g_41d390;
    v60 = (int)g_41d3a0;
    v61 = (int)g_41d3b0;
    v62 = (int)g_41d3c0;
    v63 = (int)g_41d3d0;
    v64 = (int)g_41d3e0;
    v65 = (int)g_41d3f0;
    while (1)
    {
        if (sigismember(&g_41d380, v105))
        {
            sigaction(v105, v11, NULL);
            if (v102 == &g_41534c)
                break;
        }
        else if (v102 == &g_41534c)
        {
            break;
        }
        v105 = *(v102);
        v102 += 1;
    }
    v106 = v4;
    v107 = NULL;
    signal(0x11, NULL);
    sub_4142c0(sub_408ed0);
    v108 = 0;
    v37 = 0;
    v38 = 0xffffffffffffffff;
    v36 = 0xffffffffffffffff;
    v39 = 0;
    v40 = 0;
    v41 = 0;
    v22 = 0;
    v109 = 0;
    v14 = NULL;
    ptr2 = sub_412bf0((int)v2, 8);
    v6 = NULL;
    v19 = NULL;
    v18 = NULL;
    *((char *)&v4) = 0;
    while (1)
    {
        *((unsigned int *)&v26) = 0xffffffff;
        if (v109 != 0xffffffff)
            break;
LABEL_4043f1:
        idx1 = optind;
LABEL_4043c0:
        if ((unsigned int)v2 > (unsigned int)idx1)
        {
            v149 = v106[idx1];
            optind = (unsigned int)idx1 + 1;
            *((char **)((char *)ptr2 + 0x8 * v107)) = v149;
            v107 += 1;
        }
        else
        {
            if (v6)
            {
                if (!v107)
                {
                    v161 = sub_4074e0(v6, "r");
                    if (!v161)
                        sub_407c60(v6); /* do not return */
                    sub_4119f0(&v55);
                    if (!(char)sub_411aa0(v161, &v55))
                    {
                        sub_4108e0(4);
                        error(2, 0, dcgettext(NULL, "cannot read file names from %s", 5));
                    }
                    sub_407660(v161);
                    if (!v55)
                    {
                        sub_4108e0(4, v6);
                        error(2, 0, dcgettext(NULL, "no input from %s", 5));
                    }
                    free(ptr2);
                    m = (void*)v55;
                    ptr2 = v56;
                    if (m)
                    {
                        do
                        {
                            v163 = *((long long *)((char *)ptr2 + 0x8 * v107));
                            if (!strcmp(v163, "-"))
                            {
                                sub_4108e0(4, v163);
                                error(2, 0, dcgettext(NULL, "when reading file names from stdin, no file name of %s allowed", 5));
                            }
                            v107 += 1;
                            if (!*(v163))
                            {
                                sub_410c60(0, 3, v6);
                                error(2, 0, dcgettext(NULL, "%s:%lu: invalid zero-length file name", 5));
                            }
                        } while (m != v107);
                    }
                    goto LABEL_404571;
                }
                sub_4108e0(4, ptr2->field_0);
                error(0, 0, dcgettext(NULL, "extra operand %s", 5));
                v160 = dcgettext(NULL, "file operands cannot be combined with --files0-from", 5);
                __fprintf_chk(stderr, 0x1, "%s\n", v160);
                goto LABEL_40661d;
            }
            goto LABEL_404571;
        }
    }
    if (v107 && (char)v3)
    {
        idx1 = optind;
        if (l != 1 || (char)v4)
            goto LABEL_4043c0;
        if ((unsigned int)v2 == optind && !v6)
        {
            *((char *)&v4) = 0;
            goto LABEL_404571;
        }
        v113 = v106[(unsigned int)idx1];
        if (*((char *)v113) != 45 || *((char *)(v113 + 1)) != 111 || !*((char *)(v113 + 2)) && optind + 1 == (unsigned int)v2)
            goto LABEL_4043c0;
    }
    v109 = getopt_long(v2, v106, "-bcCdfghik:mMno:rRsS:t:T:uVy:z", &g_41c5e0, &v26);
    if (v109 == 0xffffffff)
        goto LABEL_4043f1;
    if (v109 > 135)
        goto LABEL_40661d;
    if (v109 > 0)
    {
        switch (v109)
        {
        case 1:
            v141 = &optarg->field_0;
            if (optarg->field_0 == 43)
            {
                if (optind != (unsigned int)v2)
                {
                    v10 = 0;
                    if (*((char *)v106[optind]) == 45)
                        v10 = *((char *)(v106[optind] + 1)) - 48 <= 9;
                }
                else
                {
                    v10 = 0;
                }
                v142 = !v9 & (char)v10;
                l |= v142;
                if (l || v142)
                {
                    v143 = &v30;
                    v144 = 18;
                    v12 = v109;
                    for (v8 = &v30; v144; v143 += 1)
                    {
                        v144 -= 1;
                        *(v143) = 0;
                    }
                    v33 = 0xffffffffffffffff;
                    v109 = v12;
                    v145 = sub_407390(optarg + 1);
                    if (v145)
                    {
                        if (*(v145) == 46)
                        {
                            v109 = v12;
                            v145 = sub_407390(v145 + 1, &v31, 0);
                            if (v30 || v31)
                                goto LABEL_4061c1;
                        }
                        else if (!(!v31 && !v30))
                        {
                            goto LABEL_4061ca;
                        }
                        v30 = 0xffffffffffffffff;
LABEL_4061c1:
                        if (v145)
                        {
LABEL_4061ca:
                            v12 = v109;
                            v109 = v12;
                            if (!*((char *)sub_4071f0()))
                            {
                                if (v10)
                                {
                                    v12 = v109;
                                    v146 = v106[optind];
                                    optind = optind + 1;
                                    v10 = v146;
                                    v147 = sub_407390(v10 + 1, &v33, "invalid number after '-'");
                                    v148 = v12;
                                    if (!v147)
                                        __assert_fail(); /* do not return */
                                    if (*(v147) == 46)
                                    {
                                        v12 = v148;
                                        sub_407390(v147 + 1, &v34, "invalid number after '.'");
                                        v148 = v12;
                                    }
                                    if (!v34 && v33)
                                        v33 -= 1;
                                    v12 = v148;
                                    v109 = v12;
                                    if (*((char *)sub_4071f0()))
                                        sub_407340(v10, "stray character in field spec"); /* do not return */
                                }
                                v10 = v109;
                                v35 = 1;
                                sub_4076e0(v8);
                                v109 = v10;
                                continue;
                            }
                        }
                        goto LABEL_4067de;
                    }
                    else
                    {
                        if (!v31 && !v30)
                            v30 = 0xffffffffffffffff;
LABEL_4067de:
                        v141 = &optarg->field_0;
                    }
                }
            }
            *((char **)((char *)ptr2 + 0x8 * v107)) = v141;
            v107 += 1;
            continue;
        case 83:
            v10 = v109;
            v8 = &optarg->field_0;
            v12 = v26;
            v135 = sub_4132f0(optarg, &v55, 10, &v42, "EgGkKmMPtTYZ");
            v136 = v8;
            v109 = v10;
            switch (v135)
            {
            case 0:
                v139 = v42;
                if (*((char *)(v55 - 1)) - 48 <= 9)
                {
                    if (v139 > 0x3fffffffffffff)
                        goto LABEL_406691;
                    v139 *= 0x400;
                    v42 = v139;
                    break;
                }
                break;
            case 2:
                v137 = (void*)v55;
                v111 = *((char *)v137 - 1) - 48;
                if ((unsigned int)v111 <= 9 && !(char)v137[1])
                {
                    if (*((char *)v137) != 37)
                    {
                        if (*((char *)v137) == 98)
                            goto LABEL_4065ab;
                        goto LABEL_405195;
                    }
                    else
                    {
                        v10 = v136;
                        v8 = v109;
                        sub_40e760();
                        v109 = v8;
                        v136 = v10;
                        v138 = v42;
                        v108 = DivV(MulV((v138 < 0 ? AddV(v138 >> 1 | (unsigned int)v138 & 1, v138 >> 1 | (unsigned int)v138 & 1) : v138), v108), 0x4059000000000000);
                        if (((CmpF(0x43f0000000000000, (unsigned long long)v108) & 69 | (char)((CmpF(0x43f0000000000000, (unsigned long long)v108) & 69) >> 6)) & 1) == 1)
                        {
LABEL_406691:
                            v135 = 1;
                            goto LABEL_405195;
                        }
                        else
                        {
                            if (CmpF((unsigned long long)v108, 0x43e0000000000000) & 1)
                            {
                                v139 = v108;
                                v42 = v139;
                                break;
                            }
                            else
                            {
                                v108 = SubV(v108, 0x43e0000000000000);
                                v42 = v108;
                                v42 ^= 0x8000000000000000;
LABEL_4065ab:
                                v139 = v42;
                                break;
                            }
                            if (v139 >= g_41d438)
                            {
                                v140 = g_41d024 * 0x22;
                                if (v140 < v139)
                                    v140 = v139;
                                g_41d438 = v140;
                                break;
                            }
                        }
                    }
                }
                break;
            default:
LABEL_405195:
                sub_413230(v135, v12, 83, &g_41c5e0, v136); /* do not return */
            }
        case 84:
            v8 = v109;
            sub_407730(optarg);
            v109 = v8;
            continue;
        case 107:
            v127 = &v30;
            v128 = 18;
            v10 = v109;
            for (v8 = &v30; v128; v127 += 1)
            {
                v128 -= 1;
                *(v127) = 0;
            }
            v33 = 0xffffffffffffffff;
            v129 = v10;
            v130 = sub_407390(optarg);
            v30 -= 1;
            if (!v30)
            {
LABEL_406de0:
                sub_407340(optarg, "field number is zero"); /* do not return */
            }
            v32 = v31;
            if (*(v130) == 46)
            {
                v10 = v129;
                sub_407390(v130 + 1, &v31, "invalid number after '.'");
                v129 = v10;
                v32 = v31 - 1;
                if (!v31)
                    sub_407340(optarg, "character offset is zero"); /* do not return */
            }
            v31 = v32;
            if (!v30 && !v31)
                v30 = 0xffffffffffffffff;
            v10 = v129;
            v131 = sub_4071f0();
            v132 = v10;
            if (*(v131) != 44)
            {
                v33 = 0xffffffffffffffff;
                v34 = 0;
            }
            else
            {
                v10 = v132;
                v133 = v10;
                v134 = sub_407390(v131 + 1, &v33, "invalid number after ','");
                v33 -= 1;
                if (!v33)
                    goto LABEL_406de0;
                if (*(v134) == 46)
                {
                    v10 = v133;
                    sub_407390(v134 + 1, &v34, "invalid number after '.'");
                    v133 = v10;
                }
                v10 = v133;
                v131 = sub_4071f0();
                v132 = v10;
            }
            if (*(v131))
                sub_407340(optarg, "stray character in field spec"); /* do not return */
            v10 = v132;
            sub_4076e0(v8);
            v109 = v10;
            break;
        case 109:
            v22 = 1;
            continue;
        case 111:
            v125 = v14;
            v126 = &optarg->field_0;
            if (v125)
            {
                v10 = v109;
                v8 = &optarg->field_0;
                v126 = v8;
                v109 = v10;
                if (strcmp(v125, optarg))
                    error(2, 0, dcgettext(NULL, "multiple output files specified", 5));
            }
            v14 = v126;
            continue;
        case 115:
            g_41d41a = 1;
            continue;
        case 116:
            v124 = optarg->field_0;
            if (!(char)v124)
                error(2, 0, dcgettext(NULL, "empty tab", 5));
            if (optarg[1].field_0)
            {
                v10 = v109;
                v8 = &optarg->field_0;
                v109 = v10;
                if (strcmp(optarg, "\\0"))
                {
                    sub_410fd0(v8);
                    error(2, 0, dcgettext(NULL, "multi-character tab %s", 5));
                }
                v124 = 0;
            }
            if (g_41d028 != 128 && g_41d028 != v124)
                error(2, 0, dcgettext(NULL, "incompatible tabs", 5));
            g_41d028 = v124;
            break;
        case 117:
            g_41d419 = 1;
            continue;
        case 121:
            v111 = optind;
            v122 = &optarg->field_0;
            if (*((long long *)((char *)&v106[optind] - 8)) == optarg)
            {
                while (1)
                {
                    v111 = *(v122) - 48;
                    if ((unsigned int)v111 > 9)
                        break;
                    v122 += 1;
                }
                optind = optind - 1 + ((char)v123 < 1);
                break;
            }
            break;
        case 122:
            g_41d038 = 0;
            continue;
        case 129:
            v121 = &optarg->field_0;
            if (g_41d408)
            {
                v10 = v109;
                v8 = &optarg->field_0;
                v121 = v8;
                v109 = v10;
                if (strcmp(g_41d408, optarg))
                    error(2, 0, dcgettext(NULL, "multiple compress programs specified", 5));
            }
            g_41d408 = v121;
            continue;
        case 130:
            g_41d400 = 1;
            continue;
        case 131:
            v6 = &optarg->field_0;
            continue;
        case 132:
            v13 = v109;
            v8 = &optarg->field_0;
            v10 = v26;
            v12 = sub_4132f0(optarg, 0, 10, &v42, &g_416c20);
            v109 = v13;
            v118 = 0x11;
            if (!getrlimit(7, &v55))
                v118 = *((unsigned int *)&v55) - 3;
            if (!v12)
            {
                v119 = v42;
                v120 = v119;
                g_41d024 = v119;
                if (v120 == g_41d024)
                {
                    if ((unsigned int)v120 <= 1)
                    {
                        sub_410fd0(v8);
                        error(0, 0, dcgettext(NULL, "invalid --%s argument %s", 5));
                        sub_410fd0("2");
                        error(2, 0, dcgettext(NULL, "minimum --%s argument is %s", 5));
                    }
                    if ((unsigned int)v120 <= v118)
                        continue;
                }
            }
            else if (v12 != 1)
            {
                sub_413230(v12, v10, 0xffffff84, &g_41c5e0, v8); /* do not return */
            }
            v2 = v118;
            sub_410fd0(v8);
            error(0, 0, dcgettext(NULL, "--%s argument %s too large", 5));
            sub_40e1b0(v118, &v70);
            error(2, 0, dcgettext(NULL, "maximum --%s argument with current rlimit is %s", 5));
        case 133:
            v117 = &optarg->field_0;
            if (v18)
            {
                v10 = v109;
                v8 = &optarg->field_0;
                v117 = v8;
                v109 = v10;
                if (strcmp(v18, optarg))
                    error(2, 0, dcgettext(NULL, "multiple random sources specified", 5));
            }
            v18 = v117;
            continue;
        case 135:
            v12 = v26;
            v8 = &optarg->field_0;
            v116 = sub_4132f0(optarg, 0, 10, &v55, &g_416c20);
            if (v116 == 1)
            {
                v19 = 0xffffffffffffffff;
                break;
            }
            else if (!v116)
            {
                v19 = v55;
                if (!v19)
                    error(2, 0, dcgettext(NULL, "number in parallel must be nonzero", 5));
                break;
            }
            else
            {
                sub_413230(v116, v12, 0xffffff87, &g_41c5e0, v8); /* do not return */
            }
        case 128:
            v109 = 99;
            if (optarg)
            {
                v1 = 99;
                v0 = 1;
                v111 = "CCc";
                v109 = *((char *)(sub_40c370("--check") + 4281446));
                goto LABEL_404527;
            }
            break;
        case 67: case 99:
LABEL_404527:
            v115 = (char)v4;
            if ((char)v115 && v115 != v109)
                sub_407310("cC"); /* do not return */
            *((char *)&v4) = v109;
            continue;
        case 134:
            v109 = *((char *)(sub_40c370("--sort", optarg, &g_41c580, "ghMnRVCCc", 1, g_41d128, 1, &(&g_4150cc)[v114]) + 4281440));
        case 77: case 82: case 86: case 98: case 100: case 102: case 103: case 104: case 105: case 110: case 114:
            v70 = v109;
            v8 = v109;
            v71 = 0;
            sub_4071f0(&v70, &v36, 2);
            v109 = v8;
            continue;
        default:
LABEL_40661d:
            sub_40bab0(2); /* do not return */
        }
    }
    else
    {
        if (v109 == 0xffffff7d)
        {
            sub_4128e0(stdout, "sort", "GNU coreutils", g_41d120, "Mike Haertel", "Paul Eggert", 0, idx2);
            exit(0); /* do not return */
        }
        else if (v109 == 0xffffff7e)
        {
            sub_40bab0(0); /* do not return */
        }
        goto LABEL_40661d;
    }
LABEL_404571:
    if (g_41d410)
    {
        v111 = g_41d410;
        v164 = 0;
        while (1)
        {
            if ((char)sub_4070e0(v111) && !(char)index[55])
            {
                *((unsigned long long *)&index[32]) = *((unsigned long long *)((void*)&v39 + 8));
                *((unsigned long long *)&index[40]) = v40;
                *((char *)&index[48]) = *((char *)((void*)&v40 + 8));
                *((char *)&index[49]) = *((char *)((void*)&v40 + 9));
                *((char *)&index[54]) = *((char *)((void*)&v40 + 14));
                *((char *)&index[50]) = *((char *)((void*)&v40 + 10));
                *((char *)&index[52]) = *((char *)((void*)&v40 + 12));
                *((char *)&index[53]) = *((char *)((void*)&v40 + 13));
                *((char *)&index[55]) = *((char *)((void*)&v40 + 15));
                *((char *)&index[56]) = v41;
                v165 = *((char *)((void*)&v40 + 11));
                *((char *)&index[0x33]) = v165;
                v111 = (long long)index[64];
                v164 |= v165;
                if (!v111)
                    break;
            }
            else
            {
                v111 = (long long)index[64];
                v164 |= (char)index[0x33];
                if (!v111)
                    break;
            }
        }
        v8 = 0;
        goto LABEL_404648;
    }
    if ((char)sub_4070e0(&v36))
    {
        if (g_41d400)
        {
            v8 = 0;
            v164 = 0;
LABEL_404775:
            if (!(char)v4 && !v14)
            {
                if (!v7 || !setlocale(3, &g_416c20))
                {
                    dcgettext(NULL, "failed to set locale", 5);
                    error(0, 0, "%s");
                }
                if (!g_41d861)
                {
                    dcgettext(NULL, "text ordering performed using simple byte comparison", 5);
                    error(0, 0, "%s");
                }
                else
                {
                    sub_410fd0(setlocale(3, NULL));
                    error(0, 0, dcgettext(NULL, "text ordering performed using %s sorting rules", 5));
                }
                v168 = &v42;
                v169 = 18;
                i = &v36;
                for (v20 = &v42; v169; i += 4)
                {
                    v169 -= 1;
                    *((int *)&v168) = *((int *)&i);
                    v168 += 4;
                }
                if (g_41d410)
                {
                    v24 = v164;
                    v171 = v54;
                    l = v49;
                    v172 = v48;
                    v173 = v52;
                    v6 = v51;
                    v25 = v83;
                    iter = g_41d410;
                    v7 = v50;
                    v23 = 0;
                    v21 = 0;
                    v175 = v53;
                    *((char *)&v12) = 0;
                    v10 = 0;
                    v9 = 1;
                    do
                    {
                        if (0xffff00ff0000 & (long long)iter[48])
                        {
                            v176 = (char)iter[52];
                            v10 = (!v176 ? 1 : v10);
                            *((char *)&v12) = (v176 ? v176 : (char)v12);
                        }
                        if ((char)iter[57])
                        {
                            v177 = *((long long *)iter);
                            v13 = &v67;
                            if (*((long long *)iter) == 0xffffffffffffffff)
                                v177 = 0;
                            v16 = (long long)iter[16];
                            v68 = 43;
                            v15 = __stpcpy_chk(&v69, sub_40e200(), 45);
                            v17 = v13;
                            v70 = 2124589;
                            v13 = __stpcpy_chk((char *)&v70 - 3, sub_40e200(v177 + 1, v17), 44);
                            if ((long long)iter[16] != 0xffffffffffffffff)
                            {
                                v16 += 1;
                                v178 = sub_40e200(v16, v17);
                                *(v15) = 11552;
                                strcpy(v15 + 1, v178);
                                v179 = sub_40e200(((long long)iter[24] == 0xffffffffffffffff) + v16, v17);
                                *(v13) = 44;
                                strcpy(v13 + 1, v179);
                            }
                            v13 = sub_410fb0(1, &v70);
                            sub_410fb0(0, &v68);
                            error(0, 0, dcgettext(NULL, "obsolescent key %s used; consider %s instead", 5));
                        }
                        v181 = (long long)iter[16];
                        v182 = v8;
                        if (*((long long *)iter) != 0xffffffffffffffff)
                        {
                            v182 = v8;
                            if (*((long long *)iter) > v181)
                            {
                                error(0, 0, dcgettext(NULL, "key %lu has zero width and will be ignored", 5));
                                v181 = (long long)iter[16];
                                v182 = 1;
                            }
                        }
                        v184 = 1;
                        v185 = 0xffff00ff0000 & (long long)iter[48];
                        if (!(0xffff00ff0000 & (long long)iter[48]))
                            v184 = (char)iter[54];
                        if (!v181 && (long long)iter[24])
                        {
                            if (!v182)
                                goto LABEL_4048e3;
                        }
                        else
                        {
                            if (!v182 && g_41d028 == 128 && (!(char)iter[48] && (!v184 || (long long)iter[8]) || !(char)iter[49] && (long long)iter[24]))
                            {
                                error(0, 0, dcgettext(NULL, "leading blanks are significant in key %lu; consider also specifying 'b'", 5));
                                v185 = 0xffff00ff0000 & (long long)iter[48];
                                goto LABEL_4048e3;
                            }
                        }
                        if (v8)
                            goto LABEL_404908;
LABEL_4048e3:
                        if (v185)
                        {
                            v187 = *((long long *)iter) + 1;
                            if (*((long long *)iter) == 0xffffffffffffffff)
                                v187 = 1;
                            if ((long long)iter[16] < v187)
                                goto LABEL_404908;
                            error(0, 0, dcgettext(NULL, "key %lu is numeric and spans multiple fields", 5));
                            v189 = (char)iter[52];
                            v190 = v189;
                            if (!v189)
                            {
                                v191 = 1;
                                if (!v190)
                                    goto LABEL_405c47;
                            }
                            else
                            {
                                v191 = v21;
                                if (!v190)
                                {
LABEL_405c47:
                                    v192 = v23;
LABEL_405c48:
                                    v21 = v191;
                                    v23 = v192;
                                    continue;
                                }
                            }
                            v192 = v189;
                            goto LABEL_405c48;
                        }
                        else
                        {
LABEL_404908:
                            v189 = (char)iter[52];
                        }
                        if (v46 && v46 == (long long)iter[32])
                            v46 = 0;
                        if (v47 && v47 == (long long)iter[40])
                            v47 = 0;
                        v6 &= v189 ^ 1;
                        v2 &= (char)iter[48] ^ 1;
                        v173 &= (char)iter[53] ^ 1;
                        v172 &= (char)iter[49] ^ 1;
                        v2 &= (char)iter[48] ^ 1;
                        v7 &= (char)iter[0x33] ^ 1;
                        v3 &= (char)iter[54] ^ 1;
                        v48 = v172;
                        v171 &= (char)iter[56] ^ 1;
                        l &= (char)iter[50] ^ 1;
                        v3 &= (char)iter[54] ^ 1;
                        v52 = v173;
                        v49 = l;
                        v175 &= (char)iter[55] ^ 1;
                        v51 = v6;
                        v50 = v7;
                        v54 = v171;
                        v53 = v175;
                        iter = (long long)iter[64];
                        v9 += 1;
                    } while (iter);
                    v164 = v24;
                    v193 = v25;
                    if (!v21)
                    {
                        if (v23)
                            goto LABEL_4063c4;
                        if (v10 || (char)v12)
                        {
LABEL_405d6d:
                            v71 = 0;
                            v70 = g_41d868;
                            sub_410fd0(&v70);
                            if (g_41d868 != g_41d028)
                                dcgettext(NULL, "note ", 5);
                            error(0, 0, dcgettext(NULL, "%snumbers use %s as a decimal point in this locale", 5));
LABEL_405b28:
                            if (v10)
                                goto LABEL_405b2f;
                            goto LABEL_405b59;
                        }
                    }
                    v194 = g_41d028;
                    if (g_41d028 != 128)
                    {
                        if (g_41d028 != g_41d864)
                        {
                            v21 = 0;
                            goto LABEL_405aa7;
                        }
LABEL_406381:
                        v70 = g_41d864;
                        v71 = 0;
                        sub_410fd0(&v70);
                        error(0, 0, dcgettext(NULL, "field separator %s is treated as a group separator in numbers", 5));
LABEL_4063c4:
                        v194 = g_41d028;
                        if (g_41d028 != 128)
                        {
LABEL_405aa7:
                            if (v194 == g_41d868)
                                goto LABEL_40641f;
                            switch (v194)
                            {
                            case 45:
                                v70 = 45;
                                sub_410fd0(&v70);
                                v197 = "field separator %s is treated as a minus sign in numbers";
                                break;
                            case 43:
                                if (v23)
                                {
                                    v70 = 43;
                                    sub_410fd0(&v70);
                                    v197 = "field separator %s is treated as a plus sign in numbers";
                                    error(0, 0, dcgettext(NULL, v197, 5));
                                }
                                break;
                            default:
                                goto LABEL_405b0c;
                            }
                        }
                        else
                        {
                            if (g_41d864 != 128)
                            {
                                v195 = *(v193);
                                goto LABEL_40640e;
                            }
                        }
                    }
                    else if (g_41d864 != 128)
                    {
                        v195 = *(v193);
                        if (v195->padding_0[2 * (char)g_41d864] & 1)
                            goto LABEL_406381;
                        v21 = 0;
LABEL_40640e:
                        if (!(v195->padding_0[2 * g_41d868] & 1))
                            goto LABEL_405b0c;
LABEL_40641f:
                        v71 = 0;
                        v70 = g_41d868;
                        sub_410fd0(&v70);
                        error(0, 0, dcgettext(NULL, "field separator %s is treated as a decimal point in numbers", 5));
                        if (v10)
                        {
LABEL_405b2f:
                            if (g_41d862)
                                error(0, 0, dcgettext(NULL, "the multi-byte number group separator in this locale is not supported", 5));
                        }
                    }
                    else
                    {
                        v21 = 0;
LABEL_405b0c:
                        if (!v10 && !(char)v12)
                            goto LABEL_405b59;
                        if (v21)
                            goto LABEL_405b28;
                        goto LABEL_405d6d;
                    }
                }
LABEL_405b59:
                v202 = v53;
                if (!(char)sub_4070e0(v20))
                {
                    if (!g_41d41a && !g_41d419)
                        v53 = 0;
                    sub_407140(v20, &v70);
                    error(0, 0, dcngettext(NULL, "option '-%s' is ignored", "options '-%s' are ignored", strlen(&v70), 5));
                    v53 = v202;
                    if (v202)
                        goto LABEL_405cd4;
                }
                else if (v202)
                {
                    if (!g_41d41a)
                    {
                        if (!g_41d419 || !g_41d410)
                            goto LABEL_405ce1;
                        sub_407140(v20, &v70);
                        error(0, 0, dcngettext(NULL, "option '-%s' is ignored", "options '-%s' are ignored", strlen(&v70), 5));
                        v53 = v202;
                        goto LABEL_405cd4;
                    }
                    else if (g_41d410)
                    {
                        sub_407140(v20, &v70);
                        error(0, 0, dcngettext(NULL, "option '-%s' is ignored", "options '-%s' are ignored", strlen(&v70), 5));
                        v53 = 1;
LABEL_405cd4:
                        if (!g_41d41a)
                        {
LABEL_405ce1:
                            if (!g_41d419 && g_41d410)
                                error(0, 0, dcgettext(NULL, "option '-r' only applies to last-resort comparison", 5));
                        }
                    }
                }
            }
            else
            {
                if (!(char)v4)
                    v4 = 111;
                g_41d018 = v4;
                sub_407310("X --debug"); /* do not return */
            }
LABEL_404699:
            g_41d41b = *((char *)((void*)&v40 + 15));
            if ((char)v164)
            {
                v207 = sub_411060(v18, 16);
                if (v207)
                {
                    sub_411220(v207, &v70, 16);
                    if (sub_4113c0(v207))
                    {
                        v272 = "close failed";
LABEL_406b58:
                        sub_407480(dcgettext(NULL, v272, 5), v18); /* do not return */
                    }
                    MD5_Init(&g_41d300);
                    MD5_Update(&g_41d300, &v70, 16);
                }
                else
                {
                    if (!v18)
                        v18 = "getrandom";
                    v272 = "open failed";
                    goto LABEL_406b58;
                }
            }
        }
        else
        {
            g_41d41b = *((char *)((void*)&v40 + 15));
        }
        if (!g_41d428)
        {
            v208 = getenv("TMPDIR");
            if (!v208)
                v208 = "/tmp";
            sub_407730(v208);
        }
        if (!v107)
        {
            v107 = 0x1;
            free(ptr2);
            ptr2 = sub_412ab0(8);
            ptr2->field_0 = "-";
        }
        v209 = g_41d438;
        if (v209)
        {
            v210 = g_41d024 * 0x22;
            if (v210 < v209)
                v210 = v209;
            g_41d438 = v210;
        }
        if ((char)v4)
        {
            if (v107 == 1)
            {
                if (v14)
                {
                    g_41d010 = v4;
                    sub_407310(&g_41d010); /* do not return */
                }
                v211 = ptr2->field_0;
                v3 = sub_4074e0(v211, "r");
                if (!sub_4074e0(v211, "r"))
                    sub_407c60(v211); /* do not return */
                v7 = g_41d410;
                v212 = g_41d419 ^ 1;
                v213 = 0;
                v20 = &v42;
                sub_407790();
                v26 = NULL;
                v6 = 0;
                v8 = &v26;
                while (1)
                {
LABEL_405785:
                    if (!(char)sub_407ef0(v20, sub_4074e0(v211, "r"), v211))
                    {
                        v221 = 1;
                        break;
                    }
                    v214 = v45 + v42;
                    l = v44;
                    v2 = v214;
                    if (!v213 || v212 > sub_40a0b0(v8, v214 - 32))
                    {
                        node = v214 - 32;
                        do
                        {
                            v216 = node;
                            if (v214 - l * 32 >= node)
                            {
                                v217 = (long long)node[8];
                                v6 += l;
                                if (v213 < v217)
                                {
                                    do
                                    {
                                        v218 = v213 * 2;
                                        if (!(v213 * 2))
                                        {
                                            v218 = v217;
                                            break;
                                        }
                                    } while ((v213 = v218, v217 > v213));
                                    v213 = v218;
                                    free(v26);
                                    v219 = sub_412ab0(v213);
                                    v217 = (long long)node[8];
                                    v26 = v219;
                                }
                                dst = memcpy(v26, *((long long *)node), v217);
                                v27 = (long long)node[8];
                                if (v7)
                                {
                                    v28 = (long long)node[16] + dst - *((long long *)node);
                                    v29 = (long long)node[24] + dst - *((long long *)node);
                                    goto LABEL_405785;
                                }
                            }
                        } while ((node -= 32, v212 > (unsigned int)(unsigned long long)sub_40a0b0(v216, node)));
                    }
                    else
                    {
                        v216 = v214;
                        v221 = 0;
                        break;
                        if ((char)v4 == 99)
                        {
                            v222 = v216 - 32;
                            sub_40e200(((long long)(v214 - v222) >> 5) + v6, &v70);
                            v223 = dcgettext(NULL, "%s: %s:%s: disorder: ", 5);
                            __fprintf_chk(stderr, 0x1, v223);
                            v224 = dcgettext(NULL, "standard error", 5);
                            sub_4082d0(v222, stderr, v224);
                            break;
                        }
                    }
                }
                sub_407660(sub_4074e0(v211, "r"), v211);
                free(v42);
                free(v26);
                exit(v221 ^ 1); /* do not return */
            }
            sub_4108e0(4, ptr2->field_8);
            error(2, 0, dcgettext(NULL, "extra operand %s not allowed with -%c", 5));
        }
        else
        {
            v2 = ptr2;
            v226 = NULL;
            v20 = "-";
            iter2 = ptr2;
            do
            {
                if (strcmp(iter2->field_0, "-") && euidaccess(iter2->field_0, 4))
                {
                    v265 = iter2->field_0;
                    v266 = "cannot read";
                    goto LABEL_4069e2;
                }
            } while ((v226 += 1, iter2 += 8, v107 != v226));
            v228 = v14;
            v12 = v226;
            if (v228)
            {
                v229 = open(v228, 524353, 438);
                if (v229 < 0)
                {
LABEL_4068f4:
                    sub_407480(dcgettext(NULL, "open failed", 5), v14); /* do not return */
                }
                else if (v229 != 1)
                {
                    sub_407c90();
                }
            }
            if (!v22)
            {
                if (!v19)
                {
                    v230 = sub_40e570(2);
                    v231 = 0x8;
                    if (v230 <= 8)
                        v231 = v230;
                    v19 = v231;
                }
                v2 = 0;
                v232 = 0xffffffffffffff;
                v45 = NULL;
                if (v19 <= 0xffffffffffffff)
                    v232 = v19;
                l = v232;
                v6 = v232 * 2;
                v16 = v6 - 1;
                v15 = v232 * 0x100;
                v21 = "r";
            }
            else
            {
                v267 = sub_412e90(v12, 16);
                k = ptr2;
                do
                {
                    v269 = &k->field_8;
                    v267 += 16;
                    *((struct struct_3 **)((char *)v267 - 16)) = k->field_0;
                    k = v269;
                } while ((char *)ptr2 + 0x8 * v12 != k);
                sub_40ac90();
                goto LABEL_4055b4;
            }
            while (1)
            {
                v4 = ptr2->field_0;
                v3 = sub_4074e0(ptr2->field_0, v21);
                if (!sub_4074e0(ptr2->field_0, v21))
                    sub_407c60(v4); /* do not return */
                v17 = 48;
                if (v19 > 1)
                {
                    v233 = 1;
                    v234 = 1;
                    do
                    {
                        v234 *= 2;
                        v233 += 1;
                    } while (l > v234);
                    v17 = v233 * 32;
                }
                idx = v45;
                v9 = &v42;
                if (idx)
                {
LABEL_405385:
                    v12 -= 1;
                    v2 = 0;
                    v13 = &v26;
                    v18 = ptr2;
                }
                else
                {
                    v236 = v11;
                    v237 = v17 + 1;
                    v238 = v17 + 2;
                    while (1)
                    {
                        if (idx)
                        {
                            v7 = *((long long *)((char *)ptr2 + 0x8 * idx));
                            v239 = (!strcmp(*((long long *)((char *)ptr2 + 0x8 * idx)), v20) ? fstat(0, v236) : stat(v7, v236));
                        }
                        else
                        {
                            v239 = fstat(fileno(sub_4074e0(ptr2->field_0, v21)), v236);
                        }
                        if (v239)
                            break;
                        if (((unsigned short)*((unsigned int *)&v59) & 0xf000) != 0x8000)
                        {
                            v240 = g_41d438;
                            if (g_41d438)
                                goto LABEL_4060e2;
                            v240 = g_41d240;
                            v241 = 0x20000;
                            if (g_41d240)
                                goto LABEL_405e77;
                            v7 = 0x20000;
                        }
                        else
                        {
                            v241 = *((unsigned long long *)(&v60 + 8));
                            v240 = g_41d240;
                            v7 = v241;
                            if (v240 || (v240 = (unsigned long)g_41d438, g_41d240 = g_41d438, g_41d438))
                                goto LABEL_405e77;
                        }
                        v8 = &v26;
                        v242 = 0xffffffffffffffff;
                        if (!getrlimit(2, &v26))
                            v242 = v26;
                        v9 = v242;
                        v243 = v9;
                        if (!getrlimit(9, v8) && v243 > v26)
                            v243 = v26;
                        v8 = v243 >> 1;
                        v244 = v8;
                        if (!getrlimit(5, v8))
                        {
                            v245 = (v26 >> 4) * 15;
                            if (v244 > v245)
                                v244 = v245;
                        }
                        v9 = v244;
                        sub_40e850();
                        v8 = v108;
                        sub_40e760(v74, v75, v76, index, idx2, v79);
                        v246 = (int)(MulV(v108, 0x3fe8000000000000));
                        v247 = (int)(MaxV(v8, MulV(0x3fc0000000000000, v108)));
                        if (((CmpF(v9, *((unsigned long long *)&v246)) & 69 | (char)((CmpF(v9, *((unsigned long long *)&v246)) & 69) >> 6)) & 1) != 1)
                            v9 = (CmpF(*((unsigned long long *)&v246), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v246) : *((unsigned long long *)&SubV(v246, 0x43e0000000000000)) ^ 0x8000000000000000);
                        v108 = (v9 < 0 ? AddV(v9 >> 1 | (unsigned int)v9 & 1, v9 >> 1 | (unsigned int)v9 & 1) : v9);
                        if (((CmpF((unsigned long long)v108, *((unsigned long long *)&v247)) & 69 | (char)((CmpF((unsigned long long)v108, *((unsigned long long *)&v247)) & 69) >> 6)) & 1) != 1)
                            v9 = (CmpF(*((unsigned long long *)&v247), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v247) : *((unsigned long long *)&SubV(v247, 0x43e0000000000000)) ^ 0x8000000000000000);
                        v241 = v7;
                        v240 = g_41d024 * 0x22;
                        if (v240 < v9)
                            v240 = v9;
                        g_41d240 = v240;
LABEL_405e77:
                        v248 = v237 * v241 + 1;
                        if (v241 == v248 / v237 && v248 < v240 - v238)
                        {
                            v238 += v248;
                            idx += 1;
                            if (idx >= v12)
                                goto LABEL_4060e5;
                        }
                        else
                        {
LABEL_4060e2:
                            v238 = v240;
LABEL_4060e5:
                            v9 = &v42;
                            sub_407790(&v42, v17, v238);
                            goto LABEL_405385;
                        }
                    }
                    v265 = *((long long *)((char *)ptr2 + 0x8 * idx));
                    v266 = "stat failed";
LABEL_4069e2:
                    sub_407480(dcgettext(NULL, v266, 5), v265); /* do not return */
                }
                while (1)
                {
                    if (!(char)sub_407ef0(v9, sub_4074e0(ptr2->field_0, v21), v4))
                    {
                        v259 = v18;
                        sub_407660(sub_4074e0(ptr2->field_0, v21), v4);
                        if (v12)
                            goto LABEL_405d45;
                        free(v42);
                        i0 = g_41d368;
                        for (ptr1 = sub_412bf0(v2, 16); i0; i0 = v263)
                        {
                            *((struct_8 ***)&v261[8]) = i0;
                            v263 = *(i0);
                            *((struct_8 ***)v261) = (char *)&i0[1] + 5;
                        }
                        sub_40ac90(ptr1, v2, v2, v14);
                        free(ptr1);
LABEL_405592:
                        if (g_41d35c > 0)
                        {
                            do
                            {
                                sub_4078c0(0xffffffff);
                            } while (g_41d35c > 0);
                        }
LABEL_4055b4:
                        if (!g_41d418)
                        {
                            return 0;
                        }
                        else if (sub_40c5b0(stdin) == 0xffffffff)
                        {
                            sub_407480(dcgettext(NULL, "close failed", 5), v20); /* do not return */
                        }
                        return 0;
                    }
                    v249 = v2;
                    v250 = v45;
                    if (!v249)
                    {
                        v251 = v44;
                        g_41d840 = 0;
                        v252 = v250 + v42;
                        goto LABEL_405402;
                    }
                    if (v12)
                        break;
                    v251 = v44;
                    g_41d840 = 0;
                    v252 = v250 + v42;
                    if (v2 || v46)
                    {
LABEL_405402:
                        v2 += 1;
                        v249 = 0;
                        v253 = sub_408c20(v13, 0) + 13;
                        goto LABEL_405419;
                    }
                    sub_407660(sub_4074e0(ptr2->field_0, v21), v4);
                    v254 = sub_4074e0(v14, "w");
                    if (!v254)
                        goto LABEL_4068f4;
                    v26 = v254;
                    v251 = v44;
                    v2 = 0;
                    v253 = v14;
LABEL_405419:
                    if (v251 > 1)
                    {
                        v57 = sub_40deb0(sub_407c20, v6);
                        v7 = &v58;
                        pthread_mutex_init(&v58, 0);
                        pthread_cond_init(&v60 - 8, 0);
                        v255 = sub_412ab0(v15);
                        v255->field_18 = 0;
                        ptr = v255;
                        ptr->field_10 = 0;
                        ptr->field_8 = 0;
                        ptr->field_0 = 0;
                        ptr->field_20 = 0;
                        ptr->field_30 = v251;
                        ptr->field_28 = v251;
                        ptr->field_38 = 0;
                        ptr->field_50 = 0;
                        ptr->field_54 = 0;
                        v10 = (char *)&ptr[1].field_0 + 3;
                        pthread_mutex_init(v10, 0);
                        v8 = (char *)&ptr[1].field_28 + 3;
                        sub_407af0(ptr, v8, v252, l, v251, 0);
                        sub_40b220(v252, l, v251, v8, v11, v26, v253);
                        if (v6)
                        {
                            n = v16;
                            do
                            {
                                n -= 1;
                                v258 = (char *)&v10[1].field_28 + 3;
                                pthread_mutex_destroy(v10);
                                v10 = v258;
                            } while (n != 0xffffffffffffffff);
                        }
                        free(ptr);
                        sub_40df20(v57);
                        pthread_cond_destroy(&v60 - 8);
                        pthread_mutex_destroy(v7);
                    }
                    else
                    {
                        sub_40a3c0(v252 - 32, v26, v253);
                    }
                    sub_407660(v26, v253);
                    if (v249)
                    {
                        free(v42);
                        goto LABEL_405592;
                    }
                }
                v251 = v44;
                if (v17 + 1 >= v250 - v43 - v251 * v17)
                {
                    g_41d840 = 0;
                    v252 = v250 + v42;
                    goto LABEL_405402;
                }
                else
                {
                    v46 = v43;
                    v259 = v18;
                    sub_407660(sub_4074e0(ptr2->field_0, v21), v4);
LABEL_405d45:
                    ptr2 = &v259->field_8;
                }
            }
        }
    }
    else
    {
        sub_4076e0();
        v8 = 1;
        v164 = *((char *)((void*)&v40 + 11));
        if (!g_41d410)
            goto LABEL_40468c;
        do
        {
LABEL_404648:
            v111 = (void*)_INSERT(v111, 0, *((long long *)&idx2->padding_0[32]));
            if (idx2->field_32 + idx2->field_34 + idx2->field_35 + idx2->field_36 + (idx2->field_38 | idx2->field_33 | (char)v111) > 1)
            {
                idx2->padding_37[0] = 0;
                *((unsigned short *)&idx2->padding_0[48]) = 0;
                sub_407140(idx2, &v70);
                sub_407310(v79); /* do not return */
            }
        } while (*((long long *)&idx2[1].padding_0[7]));
LABEL_40468c:
        if (!g_41d400)
            goto LABEL_404699;
        goto LABEL_404775;
    }
}



// Function: proctab_hasher @ 0x6f40
void proctab_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: proctab_comparator @ 0x6f60
void proctab_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: traverse_raw_number @ 0x6f70
extern char g_41d864;
extern char g_41d868;

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
            if (v1[1] != *((int *)&g_41d864))
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
    if (g_41d868 != (char)v3)
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



// Function: getmonth @ 0x7030
typedef struct struct_3 {
    char padding_0[1];
    char field_1;
} struct_3;

typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern struct_1 g_41d060;
extern char g_41d440;
extern char g_41d740;

unsigned int getmonth(char *a0, struct_0 **a1)
{
    char *v1;  // rdi
    char *v2;  // rdi
    unsigned long long v3;  // r10
    unsigned long long v4;  // r11
    char *v5;  // rcx
    unsigned long long idx;  // r9
    struct_3 *v7;  // rdx
    char v8;  // al
    struct_3 *v9;  // rdx
    unsigned long v10;  // rsi

    v1 = a0;
    if ((&g_41d740)[*(v1)])
    {
        do
        {
            v1 = a0 + 1;
            a0 = v1;
        } while ((&g_41d740)[v2[1]]);
    }
    v3 = 12;
    v4 = 0;
LABEL_40706d:
    v5 = v1;
    idx = v4 + v3 >> 1;
    v7 = (&g_41d060.field_0)[2 * idx];
    v8 = (&g_41d060.field_0)[2 * idx]->field_0;
    if ((&g_41d060.field_0)[2 * idx]->field_0)
    {
        do
        {
            v9 = v7;
            v10 = *(v5);
            if ((&g_41d440)[v10] < v8)
            {
                v3 = idx;
                goto LABEL_4070ae;
            }
            else if ((&g_41d440)[v10] > v8)
            {
                v4 = idx + 1;
LABEL_4070ae:
                if (v4 >= v3)
                    return 0;
                goto LABEL_40706d;
            }
        } while ((v8 = v9->field_1, v7 = v9 + 1, v5 += 1, v9->field_1));
    }
    if (a1)
        *(a1) = v5;
    return (int)(&g_41d060.field_0)[1 + 2 * idx];
}



// Function: default_key_compare @ 0x70e0
unsigned long long default_key_compare(void* a0)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    if ((long long)a0[32])
    {
        return 0;
    }
    else if ((long long)a0[40])
    {
        return 0;
    }
    else
    {
        v1 = (char)a0[48];
        if ((char)v1)
        {
            return 0;
        }
        else if ((char)a0[49])
        {
            return v1;
        }
        else if (0xffff00ff0000 & (long long)a0[48])
        {
            return v1;
        }
        else
        {
            v2 = (char)a0[54];
            if ((char)v2)
            {
                return 0;
            }
            else if ((char)a0[56])
            {
                return v2;
            }
            else
            {
                return (char)a0[0x33] ^ 1;
            }
        }
    }
}



// Function: key_to_opts @ 0x7140
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

extern char g_41d540;
extern char g_41d640;

char * key_to_opts(struct_0 *a0, char *node)
{
    char *iter;  // rax

    if (a0->field_30)
    {
        *(node) = 98;
        node += 1;
    }
    if (a0->field_20 == &g_41d540)
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
    if (a0->field_20 == &g_41d640)
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



// Function: set_ordering @ 0x71f0
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

extern char g_41d440;
extern unsigned long long g_41d540;
extern unsigned long long g_41d640;

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
                idx->field_20 = &g_41d540;
                continue;
            case 102:
                *((char **)&idx->padding_28[0]) = &g_41d440;
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
                    idx->field_20 = &g_41d640;
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



// Function: struct_month_cmp @ 0x7300
int struct_month_cmp(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: incompatible_options @ 0x7310
void incompatible_options(unsigned long a0)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    error(2, 0, dcgettext(NULL, "options '-%s' are incompatible", 5));
}



// Function: badfieldspec @ 0x7340
void badfieldspec(unsigned long a0, char *a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    sub_410fd0();
    dcgettext(NULL, a1, 5);
    error(2, 0, dcgettext(NULL, "%s: invalid field specification %s", 5));
}



// Function: parse_field_count @ 0x7390

unsigned long long parse_field_count(long long a0, unsigned long long *a1, char *a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]

    sub_4132f0();
    switch ((unsigned int)sub_4132f0())
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
        sub_410fd0(a0);
        dcgettext(NULL, a2, 5);
        error(2, 0, dcgettext(NULL, "%s: invalid count at start of %s", 5));
    default:
        return v0;
    }
}



// Function: sort_die @ 0x7480
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
    sub_410c60(0, 3);
    err = __errno_location();
    error(2, *(err), "%s: %s");
}



// Function: stream_open @ 0x74e0
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
extern stat g_41d260;
extern unsigned int g_41d278;
extern int g_41d2f0;
extern char g_41d418;

long long stream_open(unsigned long a0, char *a1)
{
    int *err;  // rax
    int v2;  // r13d
    int v3;  // eax
    unsigned int v5;  // edx
    unsigned int v6;  // eax
    unsigned long fp;  // r13

    if (*(a1) != 114)
    {
        if (*(a1) != 0x77)
        {
            __assert_fail(); /* do not return */
        }
        else if (a0)
        {
            if (!ftruncate(1, 0))
                return stdout;
            err = __errno_location();
            v2 = *(err);
            v3 = g_41d2f0;
            if (!v3)
            {
                if (!fstat(1, &g_41d260.st_mode))
                {
                    g_41d2f0 = 0xffffffff;
LABEL_40757d:
                    if (((unsigned short)g_41d278 & 0xf000) != 0x8000)
                        return stdout;
LABEL_407593:
                    sub_410c60(0, 3, a0);
                    error(2, v2, dcgettext(NULL, "%s: error truncating", 5));
                }
                else
                {
                    v3 = *(err);
                    g_41d2f0 = v3;
                }
            }
            if (v3 >= 0)
                goto LABEL_407593;
            goto LABEL_40757d;
        }
        else
        {
            return stdout;
        }
    }
    else
    {
        if (!strcmp(a0, "-"))
        {
            g_41d418 = 1;
            sub_40c580(stdin, 2);
            return stdin;
        }
        v6 = open(a0, 0x80000, v5);
        if (v6 >= 0)
        {
            fp = fdopen(v6, a1);
            sub_40c580(fp, 2);
            return fp;
        }
        sub_40c580(0, 2);
        return 0;
    }
}



// Function: xfclose @ 0x7660
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
        v2 = sub_40c5b0(a0);
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
    sub_407480(dcgettext(NULL, v3, 5), a1); /* do not return */
}



// Function: insertkey @ 0x76e0
typedef struct struct_0 {
    char padding_0[64];
    struct struct_0 *field_40;
} struct_0;

extern struct_0 *g_41d410;

void insertkey(void)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // rdx
    struct_0 **v3;  // rcx
    struct_0 *v4;  // rdx
    struct_0 *v5;  // rdx

    v1 = sub_412ef0();
    v2 = g_41d410;
    v3 = &g_41d410;
    if (g_41d410)
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



// Function: add_temp_dir @ 0x7730
extern long long g_41d420;
extern unsigned long long g_41d428;
extern unsigned long long g_41d430;

long long add_temp_dir(unsigned long a0)
{
    unsigned long idx;  // rdx
    unsigned long v2;  // rdi

    idx = g_41d428;
    v2 = g_41d430;
    if (g_41d428 == g_41d420)
    {
        v2 = sub_412d00(v2, &g_41d420, 8);
        idx = g_41d428;
        g_41d430 = v2;
    }
    *((unsigned long *)(v2 + idx * 8)) = a0;
    g_41d428 = idx + 1;
    return idx + 1;
}



// Function: xstrxfrm @ 0x7800
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
    sub_4107b0(0, 8, a1);
    error(2, 0, dcgettext(NULL, "the untransformed string was %s", 5));
}



// Function: reap @ 0x78c0
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

extern unsigned int g_41d35c;
extern long long g_41d360;
extern long long g_41d408;

int reap(int a0)
{
    int v4;  // ebx
    int v5;  // edi
    unsigned int v6;  // eax
    struct_0 *v7;  // rax
    char *v9;  // rax
    int *err;  // rax
    int v0;  // [bp-0x3c], Other Possible Types: unsigned int
    char v1;  // [bp-0x38]
    unsigned int v2;  // [bp-0x30]

    v4 = a0;
    if (a0)
        v5 = v4;
    else
        v5 = -0x1;
    v6 = waitpid(v5, &v0, !a0);
    if (v6 < 0)
    {
        sub_4108e0(4, g_41d408);
        v9 = dcgettext(NULL, "waiting for %s [-d]", 5);
        err = __errno_location();
        error(2, *(err), v9);
    }
    else if (v6)
    {
        if (v4 <= 0)
        {
            v2 = v6;
            v7 = sub_40dd00(g_41d360, &v1);
            if (!v7)
                return v6;
            v7->field_c = 2;
        }
        if (!(char)v0 && !((char)v0 & 127))
        {
            g_41d35c = g_41d35c - 1;
            return v6;
        }
        sub_4108e0(4, g_41d408);
        error(2, 0, dcgettext(NULL, "%s [-d] terminated abnormally", 5));
    }
    else
    {
        return v6;
    }
}



// Function: async_safe_die @ 0x79f0
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
        v7 = sub_40e120(a0, &v0);
        write(2, ": errno ", 8);
        write(2, v7, strlen(v7));
    }
    write(2, "\n", 1);
    _exit(2); /* do not return */
}



// Function: register_proc @ 0x7a90
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

extern unsigned long long g_41d360;

void register_proc(struct_0 *a0)
{
    unsigned long long v1;  // rdi
    long long v2;  // rdi

    v1 = g_41d360;
    if (v1 || (v1 = (unsigned long long)sub_40d640(47, 0, sub_406f40, sub_406f60, 0), g_41d360 = v1, v1))
    {
        a0->field_c = 1;
        if (sub_40dca0(v1, a0))
            return;
    }
    sub_412fb0(v2); /* do not return */
}



// Function: init_node @ 0x7af0
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



// Function: fillbuf @ 0x7ef0
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    struct struct_2 *field_10;
    struct struct_2 *field_18;
} struct_1;

typedef struct struct_5 {
    struct struct_2 *field_0;
    char field_8;
    char padding_9[7];
    long long field_10;
    unsigned long long field_18;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char field_30;
} struct_5;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    char padding_18[24];
    char field_30;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern unsigned long long g_41d030;
extern char g_41d038;
extern struct_0 *g_41d410;
extern char g_41d740;

unsigned int fillbuf(struct_5 *idx, FILE *a1, unsigned long long a2)
{
    struct_0 *v7;  // rbx
    unsigned long v8;  // r10
    struct_1 *v17;  // r12
    char *v18;  // r15
    struct_1 *v19;  // r14
    char *v20;  // r13
    unsigned long v21;  // rax
    unsigned long count;  // rax
    char v23;  // al
    char *v24;  // r10
    struct_1 *iter;  // r15
    char *i;  // r14
    unsigned long n;  // rdx
    struct_0 *v27;  // r13
    unsigned long long iter1;  // r12
    char *ptr;  // rax
    unsigned long long v30;  // r11
    unsigned long long v31;  // rbp
    unsigned long long v32;  // 4098
    long long v33;  // r11
    char *v34;  // r10
    struct_1 *v35;  // r12
    char *v36;  // r14
    unsigned long v10;  // rax
    char *v37;  // r15
    struct_5 *idx1;  // rcx
    char *v39;  // rdi
    unsigned long long v40;  // rdx
    long long v41;  // rax
    struct_5 *idx2;  // rax
    struct_5 *v11;  // r15
    unsigned long long v12;  // rbp
    char *v13;  // r15
    unsigned long v14;  // r12
    unsigned long long v15;  // rcx
    char *v16;  // r14
    char *node;  // [bp-0x88]
    unsigned long v1;  // [bp-0x80]
    unsigned long long iter2;  // [bp-0x78]
    unsigned long long v3;  // [bp-0x70]
    char v4;  // [bp-0x51]
    unsigned long v5;  // [bp-0x48]

    v7 = g_41d410;
    v4 = g_41d038;
    v1 = idx->field_28;
    if (idx->field_30)
        return 0;
    v8 = *((long long *)&idx->field_8);
    n = *((long long *)&idx->field_20);
    if (v8 == n)
    {
        v10 = idx->field_10;
    }
    else
    {
        v11 = idx;
        memmove(v11->field_0, &v11->field_0[v8 + -1 * n], n);
        v8 = *((long long *)&v11->field_20);
        v11->field_10 = 0;
        v10 = 0;
        *((unsigned long *)&v11->field_8) = v8;
    }
    v12 = g_41d030 - 0x22;
    v13 = &idx->field_0->field_0;
    v14 = idx->field_18;
    v3 = v1 + 1;
    while (1)
    {
        v15 = &v13[v14];
        v16 = &v13[v8];
        v17 = v15 - v10 * 32;
        iter2 = v15 - v10 * v1 - v16;
        if (v10)
        {
            v18 = v16;
            v19 = v17;
            v20 = &v17->field_0[v17->field_8].field_0;
        }
        else
        {
            v20 = v13;
            v18 = v16;
            v19 = v17;
        }
        if (iter2 > v3)
        {
            do
            {
                v21 = (iter2 - 1) / v3;
                count = fread_unlocked(v18, 1, v21, a1);
                iter2 -= count;
                node = &v18[count];
                if (v21 != count)
                {
                    v23 = *((int *)&a1);
                    if (v23 & 32)
                    {
                        sub_407480(dcgettext(NULL, "read failed", 5), a2); /* do not return */
                    }
                    else if (v23 & 16)
                    {
                        idx->field_30 = 1;
                        if (idx->field_0 == node)
                        {
                            return 0;
                        }
                        else if (v20 != node && *(node - 1) != v4)
                        {
                            *(node) = v4;
                            node += 1;
                        }
                    }
                }
                v24 = v18;
                iter = v19;
                i = v20;
                v27 = v7;
                iter1 = iter2;
                while (1)
                {
                    ptr = memchr(v24, v4, node - v24);
                    if (!ptr)
                        break;
                    iter = (char *)iter - 32;
                    *(ptr) = 0;
                    iter->field_0 = i;
                    v30 = ptr + 1 - i;
                    iter->field_8 = v30;
                    if (v12 < v30)
                    {
                        v31 = v30;
                        v12 = v31;
                    }
                    iter1 -= v1;
                    if (v27)
                    {
                        if (v27->field_10 != 0xffffffffffffffff)
                            ptr = sub_407ca0(i, v31, v27);
                        v32 = v27->field_0;
                        if (v32 != 0xffffffffffffffff)
                        {
                            iter->field_10 = sub_407dd0(i, v33, v27);
                        }
                        else
                        {
                            if (v27->field_30)
                            {
                                for (iter->field_18 = ptr; (&g_41d740)[*(i)]; i += 1);
                            }
                            iter->field_10 = i;
                        }
                    }
                    i = v34;
                    v24 = v34;
                }
                v7 = v27;
                iter2 = iter1;
                v20 = i;
                v19 = iter;
                if (idx->field_30)
                {
                    v35 = iter;
                    v36 = node;
                    v37 = v20;
                    goto LABEL_4081a2;
                }
            } while ((v18 = node, iter2 > v3));
        }
        v35 = v19;
        v36 = v18;
        v37 = v20;
LABEL_4081a2:
        idx1 = idx;
        v39 = &idx1->field_0->field_0;
        v40 = idx1->field_18;
        *((unsigned long *)&idx1->field_8) = v36 - v39;
        v41 = &v39[v40] - v35;
        idx1->field_10 = v41 >> 5;
        if (v41 >> 5)
            break;
        v5 = v40 >> 5;
        v13 = sub_412d00(v39, &v5, 32, idx1);
        idx2 = idx;
        v14 = v5 * 32;
        idx2->field_0 = v13;
        v8 = *((long long *)&idx2->field_8);
        idx2->field_18 = v14;
        v10 = idx2->field_10;
    }
    g_41d030 = v12 + 0x22;
    *((unsigned long *)&idx1->field_20) = v36 - v37;
    return 1;
}



// Function: sighandler @ 0x8270
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_41d368;

int sighandler(int a0)
{
    struct_0 **v2;  // rbx
    struct_0 **v3;  // rbx
    char v0;  // [bp+0x0]

    v2 = g_41d368;
    if (g_41d368)
    {
        do
        {
            v3 = v2;
            unlink((char *)&v3[1] + 5);
            v2 = *(v3);
        } while (*(v3));
    }
    g_41d368 = 0;
    signal(a0, NULL);
    return raise((unsigned int)&v0);
}



// Function: write_line @ 0x82d0
typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_0 {
    void* field_0;
    char field_8;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_415360;
extern char g_41d038;
extern struct_1 *stdout;
extern char g_41d400;
extern void* g_41d410;
extern char g_41d419;
extern char g_41d41a;
extern char g_41d740;

void write_line(struct_0 *idx, struct_1 *index, unsigned long long a2)
{
    void* iter;  // r14
    unsigned long v4;  // r15
    void* node;  // rbx
    unsigned long v14;  // r15
    unsigned long v15;  // r14
    unsigned long v16;  // rbx
    unsigned long v17;  // rbx
    char *v18;  // rax
    unsigned long v19;  // r14
    char *v20;  // rax
    char *v21;  // rax
    void* i;  // rbx
    unsigned long long v23;  // r10
    char v24;  // r15b
    void* v25;  // rbx
    void* v26;  // rbx
    void* v27;  // rax
    void* v28;  // rax
    char v7;  // al
    char v8;  // al
    char *v9;  // rdx
    void* iter1;  // rbp
    void* k;  // r13
    void* v0;  // [bp-0x50]
    void* v1;  // [bp-0x48]

    iter = idx->field_0;
    v4 = *((long long *)&idx->field_8);
    i = iter + v4;
    if (!a2 && g_41d400)
    {
        if (iter < i)
        {
            do
            {
                v7 = *((char *)iter);
                iter += 1;
                if (v7 != 9)
                {
                    v8 = v7;
                    if (i == iter)
                    {
                        v7 = 10;
                        v8 = 10;
                    }
                    v9 = &index->field_28->field_0;
                    if (index->field_28 >= *((long long *)&index[1].padding_0[0]))
                        goto LABEL_4083be;
                    goto LABEL_40838c;
                }
                else
                {
                    v9 = &index->field_28->field_0;
                    v7 = 62;
                    v8 = 62;
                    if (index->field_28 < *((long long *)&index[1].padding_0[0]))
                    {
LABEL_40838c:
                        index->field_28 = v9 + 1;
                        *(v9) = v8;
                    }
                    else
                    {
LABEL_4083be:
                        if (__overflow(index, v7) == 0xffffffff)
                            sub_407480(dcgettext(NULL, "write failed", 5), 0); /* do not return */
                    }
                }
            } while (i != iter);
            iter = idx->field_0;
            v4 = *((long long *)&idx->field_8);
        }
        iter1 = g_41d410;
        k = iter + v4 - 1;
        if (!g_41d410)
            goto LABEL_408642;
        while (1)
        {
            if (*((long long *)iter1) != 0xffffffffffffffff)
            {
                node = sub_407dd0(iter, v4, iter1);
                if ((long long)iter1[16] == 0xffffffffffffffff)
                    goto LABEL_40846b;
            }
            else
            {
                node = iter;
                if ((long long)iter1[16] == 0xffffffffffffffff)
                    goto LABEL_40845f;
            }
            k = sub_407ca0(iter, v4, iter1);
LABEL_40845f:
            if (v23 == 0xffffffffffffffff && (char)iter1[48])
                goto LABEL_408485;
LABEL_40846b:
            if (!(char)iter1[54] && !(0xffff00ff0000 & (long long)iter1[48]))
                goto LABEL_408531;
LABEL_408485:
            v24 = *((char *)k);
            *((char *)k) = 0;
            v25 = node;
            if ((&g_41d740)[*((char *)v25)])
            {
                do
                {
                    v25 = node + 1;
                    node = v25;
                } while ((&g_41d740)[(char)v26[1]]);
            }
            v0 = v25;
            if (v25 <= k)
            {
                if ((char)iter1[54])
                {
                    sub_407030(v25, &v0);
                }
                else if ((char)iter1[52])
                {
                    strtold(v25, &v0);
                    /* unsupported instruction */
                    /* unsupported instruction */
                    /* unsupported instruction */
                }
                else if (0xff0000ff0000 & (long long)iter1[48])
                {
                    v27 = v25;
                    if (v27 < k)
                        v27 = (*((char *)v25) == 45) + v25;
                    v1 = v27;
                    if ((char)sub_406f70(&v1) > 47)
                    {
                        v28 = v1;
                        if ((char)iter1[53])
                            v28 = v28 + 1 - ((&g_415360)[*((char *)v28)] < 1);
                        v0 = v28;
                    }
                }
LABEL_408528:
                *((char *)k) = v24;
                k = v0;
                node = v25;
                while (1)
                {
                    while (1)
                    {
LABEL_408531:
                        v14 = sub_40e260(iter, node - iter, 0);
                        if (iter < node)
                        {
                            do
                            {
                                iter += 1;
                                v14 += *((char *)iter - 1) == 9;
                            } while (node != iter);
                        }
                        v15 = sub_40e260(node, k - node, 0);
                        if (node < k)
                        {
                            do
                            {
                                node += 1;
                                v15 += *((char *)node - 1) == 9;
                            } while (k != node);
                        }
                        v16 = v14 - 1;
                        if (v14)
                        {
                            do
                            {
                                v17 = v16;
                                v18 = stdout[1].field_8;
                                if (v18 < stdout[1].field_10)
                                {
                                    stdout[1].field_8 = v18 + 1;
                                    *(v18) = 32;
                                }
                                else
                                {
                                    __overflow(stdout, 32);
                                }
                            } while ((v16 = (unsigned long)(v17 - 1), v17 >= 1));
                        }
                        if (v15)
                        {
                            do
                            {
                                v19 = v15;
                                v20 = stdout[1].field_8;
                                if (v20 < stdout[1].field_10)
                                {
                                    stdout[1].field_8 = v20 + 1;
                                    *(v20) = 95;
                                }
                                else
                                {
                                    __overflow(stdout, 95);
                                }
                            } while ((v15 = (unsigned long)(v19 - 1), v19 != 1));
                            v21 = stdout[1].field_8;
                            if (v21 < stdout[1].field_10)
                            {
                                stdout[1].field_8 = v21 + 1;
                                *(v21) = 10;
                            }
                            else
                            {
                                __overflow(stdout, 10);
                            }
                        }
                        else
                        {
                            __printf_chk(1, dcgettext(NULL, "^ no match for key\n", 5));
                        }
                        if (!iter1)
                            return;
                        iter1 = (long long)iter1[64];
                        if (iter1)
                            break;
                        if (g_41d419)
                            return;
                        if (g_41d41a)
                            return;
                        iter = idx->field_0;
                        node = iter;
                        k = iter + *((long long *)&idx->field_8) - 1;
                    }
                    iter = idx->field_0;
                    k = iter + *((long long *)&idx->field_8) - 1;
                    if (iter1)
                        break;
LABEL_408642:
                    node = iter;
                }
            }
            else
            {
                v0 = k;
                goto LABEL_408528;
            }
        }
    }
    *((char *)i - 1) = g_41d038;
    if (v4 != fwrite_unlocked(iter, 1, v4, index))
        sub_407480(dcgettext(NULL, "write failed", 5), a2); /* do not return */
    *((char *)i - 1) = 0;
    return;
}



// Function: pipe_fork @ 0x8770
extern unsigned int g_41d024;
extern unsigned int g_41d35c;
extern unsigned long long g_41d368;
extern long long g_41d380;

unsigned int pipe_fork(int *a0, unsigned long long a1)
{
    unsigned long long v5;  // rbx
    unsigned int *err;  // rbp
    unsigned long long v7;  // rbx
    unsigned int v8;  // eax
    unsigned long v9;  // r14
    unsigned int v10;  // eax
    unsigned int v11;  // ecx
    unsigned int v12;  // r12d
    unsigned int v0;  // [bp-0xe0], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xd4]
    char v2;  // [bp-0xc8]
    char v3;  // [bp-0xc0]

    if (sub_40e990() < 0)
        return 0xffffffff;
    if (g_41d024 + 1 < g_41d35c)
    {
        sub_4078c0(-0x1);
        do
        { } while (g_41d35c > 0 && sub_4078c0(0));
    }
    v5 = a1 - 1;
    err = __errno_location();
    v0 = 0x3fd0000000000000;
    while (1)
    {
        v7 = v5;
        v8 = pthread_sigmask(0, &g_41d380, &v3);
        v9 = g_41d368;
        g_41d368 = 0;
        v2 = !v8;
        v10 = fork();
        v11 = *(err);
        v12 = v10;
        if (v12)
        {
            g_41d368 = v9;
            if (!v2)
                goto LABEL_408802;
        }
        else if (!v2)
        {
            close(0);
            close(1);
            return 0;
        }
        v1 = v11;
        pthread_sigmask(2, &v3, 0);
        v11 = v1;
LABEL_408802:
        *(err) = v11;
        if (v12 >= 0)
            break;
        if (v11 != 11)
        {
            v0 = v11;
            close(*(a0));
            close(a0[1]);
            *(err) = v0;
            return v12;
        }
        sub_413180();
        v0 = AddV(v0, v0);
        do
        { } while (g_41d35c > 0 && sub_4078c0(0));
        v5 = v7 - 1;
        if (v7 < 1)
        {
            v11 = *(err);
            v0 = v11;
            close(*(a0));
            close(a0[1]);
            *(err) = v0;
            return v12;
        }
    }
    if (!v12)
    {
        close(0);
        close(1);
        return 0;
    }
    g_41d35c = g_41d35c + 1;
    return v12;
}



// Function: open_input_files @ 0x8980
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
    char field_d;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_0 *field_8;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
} struct_2;

extern long long g_41d360;
extern void g_41d408;

long long open_input_files(struct_1 *a0, long long a1, struct_2 **a2)
{
    struct_1 *node;  // rbx
    void* iter;  // rbp
    int *err;  // rbx
    char *v17;  // rax
    unsigned long v18;  // rdi
    unsigned int *err1;  // rax
    unsigned int *err2;  // rax
    unsigned int v21;  // r13d
    long long index;  // r12
    struct_0 *idx;  // r15
    unsigned long v10;  // rax
    unsigned long long v11;  // rax
    unsigned int v12;  // edx
    unsigned int v13;  // eax
    int v14;  // r8d
    unsigned long fp;  // rax
    unsigned int v0;  // [bp-0x64], Other Possible Types: int
    void* v1;  // [bp-0x60]
    int v2;  // [bp-0x58]
    unsigned int v3;  // [bp-0x54]
    unsigned int v4;  // [bp-0x50]

    node = a0;
    v1 = sub_412bf0(a1, 8);
    *(a2) = v1;
    if (!a1)
        return 0;
    iter = v1;
    index = 0;
    while (1)
    {
        idx = node->field_8;
        if (!idx || !idx->field_c)
        {
            v10 = sub_4074e0(*((long long *)&node->field_0), "r");
            *((unsigned long *)iter) = v10;
            if (!v10)
                return index;
        }
        else
        {
            if (idx->field_c == 1)
            {
                v4 = idx->field_8;
                v0 = idx->field_8;
                v11 = sub_40dd00(g_41d360, &v2);
                if (v11)
                {
                    *((char *)(v11 + 12)) = 2;
                    sub_4078c0(v0);
                }
            }
            v0 = open(&idx->field_d, 0, v12);
            if (v0 < 0)
            {
                *((unsigned long *)((char *)v1 + 8 * index)) = 0;
                return index;
            }
            v13 = sub_408770(&v2, 9);
            v14 = v0;
            if (v13 == 0xffffffff)
            {
                v0 = v14;
                err = __errno_location();
                if (*(err) == 24)
                {
                    close(v0);
                    *(err) = 24;
                    *((unsigned long *)((char *)v1 + 8 * index)) = 0;
                    return index;
                }
                sub_4108e0(4, *((long long *)&g_41d408));
                v17 = dcgettext(NULL, "couldn't create process for %s -d", 5);
                error(2, *(err), v17);
            }
            if (v13)
            {
                idx->field_8 = v13;
                v0 = v14;
                sub_407a90(idx);
                close(v0);
                close(v3);
                fp = fdopen(v2, "r");
                if (!fp)
                {
                    err2 = __errno_location();
                    v21 = *(err2);
                    close(v2);
                    *((unsigned long *)iter) = 0;
                    *(err2) = v21;
                    return index;
                }
                *((unsigned long *)iter) = fp;
            }
            else
            {
                v0 = v14;
                close(v2);
                if (v0)
                    sub_407c90(v0, 0);
                v18 = v3;
                if ((unsigned int)v18 != 1)
                {
                    sub_407c90(v18, 1);
                    break;
                }
            }
        }
        index += 1;
        node += 1;
        iter += 8;
        if (a1 == index)
            return index;
    }
    execlp(*((long long *)&g_41d408), *((long long *)&g_41d408));
    err1 = __errno_location();
    sub_4079f0(*(err1), "couldn't execute compress program (with -d)"); /* do not return */
}



// Function: maybe_create_temp @ 0x8c20
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char field_c;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern struct_1 *g_41d040;
extern unsigned long long g_41d248;
extern long long g_41d380;
extern char *g_41d408;
extern char g_41d428;
extern unsigned long long g_41d430;

struct_0 * maybe_create_temp(FILE **a0, char a1)
{
    unsigned long ptr;  // r14
    unsigned long len;  // rbx
    unsigned int v14;  // eax
    unsigned long v15;  // rdi
    FILE *fp;  // rax
    unsigned int v18;  // ebp
    char *v19;  // rax
    struct_0 *v6;  // rax
    struct_0 *v7;  // r15
    struct_0 *ptr1;  // r12
    unsigned long long v9;  // 4106
    unsigned int v10;  // ebp
    void* err;  // rbx
    struct_0 **v12;  // rax
    char *v13;  // 4098
    int v0;  // [bp-0xc8]
    unsigned int v1;  // [bp-0xc4]
    char v2;  // [bp-0xc0]

    ptr = *((long long *)(g_41d430 + g_41d248 * 8));
    len = strlen(ptr);
    v6 = sub_412ab0(len + 32 & 0xfffffffffffffff8);
    v7 = v6 + 1;
    ptr1 = v6;
    memcpy(v7, ptr, len);
    *((unsigned long *)((char *)ptr1 + len + 13)) = 6365935330447553327;
    *((unsigned int *)((char *)v7 + len + 8)) = 0x585858;
    ptr1->field_0 = 0;
    v9 = *((long long *)&g_41d428);
    g_41d248 = g_41d248 + 1;
    if (g_41d248 == v9)
        g_41d248 = 0;
    *((char *)&v0) = !pthread_sigmask(0, &g_41d380, &v2);
    v10 = sub_411d70(v7, 0x80000);
    err = __errno_location();
    if (v10 >= 0)
    {
        v12 = g_41d040;
        g_41d040 = ptr1;
        *(v12) = ptr1;
        if ((char)v0)
        {
            pthread_sigmask(2, &v2, 0);
            *((int *)err) = *((int *)err);
        }
        v13 = g_41d408;
        ptr1->field_c = 0;
        if (v13)
        {
            v14 = sub_408770(&v0, 4);
            ptr1->field_8 = v14;
            if (v14 > 0)
            {
                close(v10);
                close(v0);
                v10 = v1;
                sub_407a90(ptr1);
            }
            else if (!v14)
            {
                close(v1);
                if (v10 != 1)
                    sub_407c90(v10, 1);
                v15 = v0;
                if ((unsigned int)v15)
                    sub_407c90(v15, 0);
                execlp(g_41d408, g_41d408);
                sub_4079f0(*((int *)err), "couldn't execute compress program"); /* do not return */
            }
        }
        fp = fdopen(v10, "w");
        *(a0) = fp;
        if (!fp)
            sub_407480(dcgettext(NULL, "couldn't create temporary file", 5), v7); /* do not return */
        return ptr1;
    }
    v18 = *((int *)err);
    if ((char)v0)
    {
        pthread_sigmask(2, &v2, 0);
        *((unsigned int *)err) = v18;
    }
    if (v18 == 24 && a1 == 1)
    {
        free(ptr1);
        return NULL;
    }
    sub_4108e0(4, ptr);
    v19 = dcgettext(NULL, "cannot create temporary file in %s", 5);
    error(2, *((int *)err), v19);
}



// Function: exit_cleanup @ 0x8ed0
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern int g_41d130;
extern long long stderr;
extern long long stdout;
extern struct_0 *g_41d368;
extern long long g_41d380;
extern char g_41d870;
extern long long g_41d878;

void exit_cleanup(void)
{
    unsigned long v5;  // fs
    unsigned int v6;  // eax
    struct_0 **v7;  // rbx
    char v8;  // al
    struct_0 **v9;  // rbx
    unsigned long long v10;  // r12
    int *err;  // rbx
    char v0;  // [bp-0xa8]
    char v1;  // [bp-0xa0]
    unsigned long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x8]

    v2 = *((long long *)(40 + v5));
    if (g_41d368)
    {
        v6 = pthread_sigmask(0, &g_41d380, &v1);
        v7 = g_41d368;
        v8 = !v6;
        v0 = !v6;
        if (g_41d368)
        {
            do
            {
                v9 = v7;
                unlink((char *)&v9[1] + 5);
                v7 = *(v9);
            } while (*(v9));
            v8 = v0;
        }
        g_41d368 = 0;
        if (v8)
            pthread_sigmask(2, &v1, 0);
    }
    if (v2 != *((long long *)(40 + v5)))
        __stack_chk_fail(); /* do not return */
    v3 = v10;
    if (!(!sub_413720(stdout) || (err = __errno_location(), g_41d870 && *(err) == 32)))
    {
        dcgettext(NULL, "write error", 5);
        if (g_41d878)
        {
            sub_410b40(g_41d878);
            error(0, *(err), "%s: %s");
        }
        else
        {
            error(0, *(err), "%s");
        }
    }
    else if (!sub_413720(stderr))
    {
        return;
    }
    _exit(g_41d130); /* do not return */
}



// Function: zaptemp @ 0x8f90
typedef struct struct_1 {
    char padding_0[12];
    char field_c;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
    char field_c;
    char field_d;
} struct_0;

extern unsigned long long g_41d040;
extern long long g_41d360;
extern struct_0 g_41d368;
extern long long g_41d380;

void zaptemp(unsigned long i)
{
    struct_0 *v4;  // rbx
    struct_0 *v5;  // rbp
    struct_0 *ptr;  // rbp
    struct_1 *v7;  // rax
    struct_0 *v8;  // r14
    unsigned int v9;  // r13d
    unsigned int v10;  // r8d
    unsigned int v0;  // [bp-0xcc]
    char v1;  // [bp-0xc8]
    unsigned int v2;  // [bp-0xc0]

    v4 = &g_41d368.padding_0[0];
    v5 = *((long long *)&g_41d368.padding_0[0]);
    ptr = v5;
    if (i != *((long long *)&g_41d368.padding_0[0]) + 13)
    {
        do
        {
            v4 = v5;
            ptr = v4->field_0;
            v5 = ptr;
        } while (i != &v5->field_d);
    }
    if (ptr->field_c == 1)
    {
        v2 = ptr->field_8;
        v7 = sub_40dd00(g_41d360, &v1);
        if (v7)
        {
            v7->field_c = 2;
            sub_4078c0(v2);
        }
    }
    v8 = ptr->field_0;
    v1 = !pthread_sigmask(0, &g_41d380, &v2);
    v9 = unlink(i);
    v10 = *(__errno_location());
    v4->field_0 = v8;
    if (!v1)
    {
        if (v9)
            goto LABEL_409086;
LABEL_40902b:
        if (v8)
        {
            free(ptr);
            return;
        }
    }
    else
    {
        v0 = v10;
        pthread_sigmask(2, &v2, 0);
        v10 = v0;
        if (!v9)
            goto LABEL_40902b;
LABEL_409086:
        v0 = v10;
        sub_410c60(0, 3, i);
        error(0, v0, dcgettext(NULL, "warning: cannot remove: %s", 5));
        if (v8)
        {
            free(ptr);
            return;
        }
    }
    g_41d040 = v4;
    free(ptr);
    return;
}



// Function: keycompare @ 0x9120
typedef struct struct_0 {
    void* field_0;
    long long field_8;
    void* field_10;
} struct_0;

extern char g_415360;
extern uint128_t g_41d300;
extern uint128_t g_41d310;
extern uint128_t g_41d320;
extern uint128_t g_41d330;
extern uint128_t g_41d340;
extern unsigned long long g_41d350;
extern unsigned int g_41d358;
extern void* g_41d410;
extern char g_41d740;
extern char g_41d861;
extern unsigned int g_41d864;
extern char g_41d868;

unsigned int keycompare(struct_0 *a0, struct_0 *index)
{
    void* iter;  // r14
    void* iter1;  // r13
    void* v55;  // r10
    unsigned long v56;  // rdx
    unsigned int v57;  // edx
    unsigned int v58;  // ecx
    unsigned int v59;  // ecx
    unsigned int v60;  // ecx
    unsigned int v61;  // edx
    void* v62;  // r14
    unsigned int j;  // ecx
    unsigned int v64;  // edx
    void* node;  // rbp
    char *v65;  // rcx
    char v66;  // cl
    char v67;  // bl
    long long v68;  // rdi
    void* v69;  // rcx
    void* v70;  // rsi
    void* iter2;  // r9
    void* v72;  // r14
    unsigned long v73;  // rax
    void* v74;  // r10
    char *v48;  // rbx
    void* v75;  // r13
    unsigned long v76;  // rax
    void* v77;  // rax
    void* v78;  // rdi
    void* v79;  // rdi
    void* v80;  // rdi
    void* v81;  // rsi
    void* v82;  // rsi
    void* v83;  // rsi
    void* v84;  // r9
    char *v49;  // r12
    void* v85;  // r10
    unsigned long v86;  // rax
    void* v87;  // r11
    void* v88;  // r11
    unsigned long v89;  // rax
    void* v90;  // r12
    void* v91;  // r12
    unsigned int v92;  // ecx
    unsigned int v93;  // ecx
    unsigned int v94;  // eax
    void* v50;  // r11
    void* v95;  // r12
    void* v96;  // rbx
    void* v97;  // rax
    void* v98;  // rbx
    void* v99;  // rdx
    void* v100;  // rdi
    void* v102;  // r13
    void* v103;  // r13
    void* v104;  // r9
    void* v51;  // r11
    void* v105;  // r10
    unsigned int v106;  // ecx
    unsigned int v107;  // eax
    char *v108;  // r9, Other Possible Types: void*
    char *v109;  // r10, Other Possible Types: void*
    unsigned int v110;  // ecx
    unsigned int v111;  // eax
    unsigned long v112;  // cc_dep1
    void* v52;  // r10
    void* v113;  // r10
    void* v114;  // rbx
    void* v53;  // rax
    void* v54;  // r9
    void* v0;  // [bp-0x2140], Other Possible Types: unsigned int, unsigned long long
    char v1;  // [bp-0x2132]
    char v2;  // [bp-0x2131]
    void* v3;  // [bp-0x2110]
    void* ptr1;  // [bp-0x2110]
    void* v5;  // [bp-0x2108], Other Possible Types: unsigned int
    void* ptr;  // [bp-0x2100], Other Possible Types: char
    void* v7;  // [bp-0x20f8]
    void* v8;  // [bp-0x20f0]
    void* v9;  // [bp-0x20e8], Other Possible Types: unsigned int
    void* v10;  // [bp-0x20e0], Other Possible Types: unsigned int
    char *v11;  // [bp-0x20d8]
    char *v12;  // [bp-0x20d0]
    void* v13;  // [bp-0x20c8]
    void* v14;  // [bp-0x20c0]
    void* v15;  // [bp-0x20b8]
    void* v16;  // [bp-0x20b0]
    void* v17;  // [bp-0x20a8]
    void* v18;  // [bp-0x20a0]
    char *v19;  // [bp-0x2098]
    char *v20;  // [bp-0x2090], Other Possible Types: void*
    char v21;  // [bp-0x2088]
    char v22;  // [bp-0x2078]
    char *v23;  // [bp-0x2078]
    char *v24;  // [bp-0x2078]
    char v25;  // [bp-0x2068]
    char v26;  // [bp-0x2058]
    int v27;  // [bp-0x2048]
    int v28;  // [bp-0x2038]
    int v29;  // [bp-0x2028]
    int v30;  // [bp-0x2018]
    int v31;  // [bp-0x2008]
    unsigned long v32;  // [bp-0x1ff8]
    unsigned int v33;  // [bp-0x1ff0]
    int v34;  // [bp-0x1fec]
    int v35;  // [bp-0x1fdc]
    int v36;  // [bp-0x1fcc]
    int v37;  // [bp-0x1fbc]
    int v38;  // [bp-0x1fac]
    unsigned long v39;  // [bp-0x1f9c]
    unsigned int v40;  // [bp-0x1f94]
    char v41;  // [bp-0x1f88]
    char v42;  // [bp-0x1f87]
    char v43;  // [bp-0xfe8]

    iter = a0->field_10;
    iter1 = index->field_10;
    node = g_41d410;
    v13 = &v43;
    while (1)
    {
        v48 = (long long)node[40];
        v49 = (long long)node[32];
        v51 = (v50 <= iter ? iter : v50);
        v53 = (v52 <= iter1 ? iter1 : v52);
        v54 = v51 - iter;
        v55 = v53 - iter1;
        if (g_41d861 || 0xffffffffff0000 & (long long)node[48] || (char)node[56])
            break;
        if (v49)
        {
            if (v48)
            {
                for (; iter < v51 && v49[*((char *)iter)]; iter += 1)
                {
LABEL_4097e2:
                }
                if (iter1 < v53)
                {
                    while (1)
                    {
                        v56 = *((char *)iter1);
                        if (!v49[v56])
                            break;
                        iter1 += 1;
                        if (v53 == iter1)
                            goto LABEL_409f28;
                    }
                    if (v51 > iter && v53 > iter1)
                    {
                        v57 = v48[v56];
                        v58 = v48[*((char *)iter)];
                        v59 = v58 - v57;
                        if (v58 != v57)
                            goto LABEL_4094da;
                        iter1 += 1;
                        goto LABEL_4097e2;
                    }
                }
LABEL_409f28:
                v59 = (iter < v51) - (v53 > iter1);
                goto LABEL_4092bf;
            }
            else
            {
                while (1)
                {
                    if (iter >= v51 || !v49[*((char *)iter)])
                    {
                        if (iter1 < v53)
                        {
                            while (v49[*((char *)iter1)])
                            {
                                if (v53 == iter1 + 1)
                                    goto LABEL_40a018;
                            }
                            if (v51 <= iter || v53 <= iter1)
                                goto LABEL_40a018;
                            v60 = *((char *)iter);
                            v61 = *((char *)iter1);
                            v59 = v60 - v61;
                            if (v60 != v61)
                                goto LABEL_4094da;
                            iter1 += 1;
                        }
                        else
                        {
LABEL_40a018:
                            v59 = (iter < v51) - (v53 > iter1);
                            goto LABEL_4092bf;
                        }
                    }
                    iter += 1;
                }
                goto LABEL_4092bf;
            }
        }
        if (v54)
        {
            if (!v55)
            {
LABEL_409f18:
                v114 = node;
                v59 = 1;
                goto LABEL_4094dd;
            }
            if (!v48)
            {
                v108 = v55;
                v0 = v54;
                v59 = memcmp(iter, iter1, (v54 <= v55 ? v54 : v55));
                if (v59)
                    goto LABEL_4094da;
                v54 = v0;
                v55 = v108;
LABEL_409f09:
                if (v54 >= v55)
                {
                    if (v54 == v55)
                        goto LABEL_4092c7;
                    goto LABEL_409f18;
                }
                else
                {
                    v114 = node;
                    v59 = 0xffffffff;
LABEL_4094dd:
                }
            }
            else
            {
                do
                {
                    if (iter < v51 && iter1 < v53)
                        goto LABEL_409f6d;
                    else
                        goto LABEL_409f09;
LABEL_409f6d:
                    iter1 += 1;
                    v62 = iter + 1;
                    j = v48[*((char *)iter)];
                    v59 = j - v48[*((char *)iter1 - 1)];
                    iter = v62;
                } while (j == v64);
LABEL_4094da:
                v114 = node;
                goto LABEL_4094dd;
            }
            return -(v59);
        }
        else
        {
            v59 = -(NULL < v55);
LABEL_4092bf:
            if (v59)
                goto LABEL_4094da;
LABEL_4092c7:
            node = (long long)node[64];
            if (!node)
                return 0;
            iter = a0->field_0;
            if ((long long)node[16] != 0xffffffffffffffff)
            {
                sub_407ca0(iter, a0->field_8, node);
                v113 = sub_407ca0(index->field_0, index->field_8, node);
            }
            else
            {
                v113 = index->field_0 + index->field_8 - 1;
            }
            if (*((long long *)node) != 0xffffffffffffffff)
            {
                iter = sub_407dd0(iter, a0->field_8, node);
                iter1 = sub_407dd0(index->field_0, index->field_8, node);
            }
            else
            {
                iter1 = index->field_0;
                if ((char)node[48])
                {
                    if (v50 > iter)
                    {
                        do
                        {
                        } while ((&g_41d740)[*((char *)iter)] && (iter += 1, v50 != iter));
                    }
                    if (v113 > iter1)
                    {
                        do
                        {
                        } while ((&g_41d740)[*((char *)iter1)] && (iter1 += 1, v113 != iter1));
                    }
                }
            }
        }
    }
    v65 = v48;
    v0 = v65 | v49;
    if (!v65 && !v49)
    {
        v66 = *((char *)v51);
        *((char *)v51) = 0;
        v67 = *((char *)v53);
        *((char *)v53) = 0;
        v1 = v66;
        v2 = v67;
        ptr1 = v3;
        goto LABEL_409209;
    }
    v68 = v54 + v55 + 2;
    v69 = &v41;
    ptr1 = NULL;
    if (v68 <= 4000)
    {
        v70 = &v42 + v54;
        if (!v54)
            goto LABEL_409887;
        goto LABEL_4095c1;
    }
    else
    {
        v109 = v55;
        v108 = v54;
        v54 = v108;
        v55 = v109;
        v69 = sub_412ab0(v68);
        ptr1 = v69;
        v70 = v69 + v54 + 1;
        if (v54)
        {
LABEL_4095c1:
            iter2 = NULL;
            v72 = iter;
            do
            {
                v73 = *((char *)v72);
                if (!v49 || !v49[(char)v73])
                {
                    if (v48)
                        v73 = v48[v73];
                    *((char *)(v69 + iter2)) = v73;
                    iter2 += 1;
                }
            } while ((v72 += 1, iter + v54 != v72));
            *((char *)(v69 + iter2)) = 0;
            if (!v55)
                goto LABEL_409896;
            goto LABEL_40960d;
        }
        else
        {
LABEL_409887:
            iter2 = v54;
            *((char *)v69) = 0;
            if (v55)
            {
LABEL_40960d:
                v74 = NULL;
                v75 = iter1;
                do
                {
                    v76 = *((char *)v75);
                    if (!v49 || !v49[(char)v76])
                    {
                        if (v48)
                            v76 = v48[v76];
                        *((char *)(v70 + v74)) = v76;
                        v74 += 1;
                    }
                } while ((v75 += 1, iter1 + v55 != v75));
                v77 = v70 + v74;
                v55 = v74;
                v54 = iter2;
                goto LABEL_40964a;
            }
            else
            {
LABEL_409896:
                v54 = iter2;
                v77 = v70;
LABEL_40964a:
                *((char *)v77) = 0;
                iter1 = v70;
                iter = v69;
LABEL_409209:
                if ((char)node[50])
                {
                    v78 = iter;
                    if ((&g_41d740)[*((char *)v78)])
                    {
                        do
                        {
                            v80 = v78 + 1;
                            v78 = v80;
                        } while ((&g_41d740)[(char)v79[1]]);
                    }
                    v81 = iter1;
                    if ((&g_41d740)[*((char *)v81)])
                    {
                        do
                        {
                            v83 = v81 + 1;
                            v81 = v83;
                        } while ((&g_41d740)[(char)v82[1]]);
                    }
                    v109 = v55;
                    v108 = v54;
                    v84 = v108;
                    v85 = v109;
                    v59 = sub_411d90();
                    goto LABEL_4092a0;
                }
                if ((char)node[52])
                {
                    v5 = v55;
                    v109 = v54;
                    strtold(iter, &v19);
                    amd64g_dirtyhelper_storeF80le((Reference vvar_2795{s-8488|8b}), Reinterpret(F64->I64, (((unsupported_<class 'pyvex.expr.GetI'>() CmpNE 0<8>)) ? (unsupported_<class 'pyvex.expr.GetI'>()) : (nan<64>))))
                    /* unsupported instruction */
                    strtold(iter1, &v20);
                    v84 = v109;
                    /* unsupported instruction */
                    /* unsupported instruction */
                    v85 = v5;
                    if (iter == v19)
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        v59 = -(v20 != iter1);
                        goto LABEL_4092a0;
                    }
                    if (iter1 == v20)
                    {
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
                        /* unsupported instruction */
LABEL_409748:
                        v59 = 1;
                        goto LABEL_4092a0;
                    }
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
                        v59 = 1;
                        if (((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69 | (char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 6)) & 1) != 1)
                        {
                            /* unsupported instruction */
                            /* unsupported instruction */
                            /* unsupported instruction */
                            /* unsupported instruction */
                            /* unsupported instruction */
                            /* unsupported instruction */
                            goto LABEL_4092a0;
                        }
                        v112 = CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69;
                        if (!((char)((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2) & 1))
                        {
                            v59 = 0;
                            if ((char)v112 & 64)
                            {
                                /* unsupported instruction */
                                /* unsupported instruction */
                                /* unsupported instruction */
                                /* unsupported instruction */
                                /* unsupported instruction */
                                /* unsupported instruction */
                                goto LABEL_4092a0;
                            }
                            else
                            {
                                /* unsupported instruction */
                                /* unsupported instruction */
                                /* unsupported instruction */
                                /* unsupported instruction */
                            }
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
                            if (!((CmpF((/* unsupported instruction */ ? /* unsupported instruction */ : nan), (/* unsupported instruction */ ? /* unsupported instruction */ : nan)) & 69) >> 2 & 1))
                                goto LABEL_409748;
                            v109 = v85;
                            v108 = v84;
                            strtold(iter, NULL);
                            amd64g_dirtyhelper_storeF80le((Reference vvar_3099{s-8328|1b}), Reinterpret(F64->I64, (((unsupported_<class 'pyvex.expr.GetI'>() CmpNE 0<8>)) ? (unsupported_<class 'pyvex.expr.GetI'>()) : (nan<64>))))
                            /* unsupported instruction */
                            strtold(iter1, NULL);
                            amd64g_dirtyhelper_storeF80le((Reference vvar_3100{s-8312|1b}), Reinterpret(F64->I64, (((unsupported_<class 'pyvex.expr.GetI'>() CmpNE 0<8>)) ? (unsupported_<class 'pyvex.expr.GetI'>()) : (nan<64>))))
                            /* unsupported instruction */
                            v84 = v108;
                            v85 = v109;
                            v59 = memcmp(&v21, &v22, 16);
                            goto LABEL_4092a0;
                        }
                        else
                        {
                            /* unsupported instruction */
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
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
                    v59 = 0xffffffff;
LABEL_4092a0:
                    if (v0)
                        goto LABEL_4094c0;
                    goto LABEL_4092ac;
                }
                if ((char)node[53])
                {
                    v86 = *((char *)iter);
                    v87 = iter;
                    v88 = v87;
                    if ((&g_41d740)[v86])
                    {
                        do
                        {
                            v88 = v87 + 1;
                            v86 = (char)v87[1];
                            v87 = v88;
                        } while ((&g_41d740)[v86]);
                    }
                    v89 = *((char *)iter1);
                    v90 = iter1;
                    v91 = v90;
                    if ((&g_41d740)[v89])
                    {
                        do
                        {
                            v91 = v90 + 1;
                            v89 = (char)v90[1];
                            v90 = v91;
                        } while ((&g_41d740)[v89]);
                    }
                    v5 = v55;
                    v109 = v54;
                    v108 = &v22;
                    ptr = v86;
                    v23 = ((char)v86 == 45) + v88;
                    v92 = 0;
                    if ((char)sub_406f70(&v23) > 48)
                    {
                        v93 = (&g_415360)[*(v23)];
                        v92 = (ptr == 45 ? -(v93) : v93);
                    }
                    v5 = v92;
                    v109 = v5;
                    v108 = v109;
                    v24 = ((char)v89 == 45) + v91;
                    v84 = v108;
                    v85 = v109;
                    v59 = v5;
                    if ((char)sub_406f70(v108) > 48)
                    {
                        v94 = (&g_415360)[*(v24)];
                        v59 = ((char)v89 == 45 ? v94 + v59 : v59 - v94);
                    }
                    if (!v59)
                    {
                        v109 = v85;
                        v108 = v84;
                        v84 = v108;
                        v85 = v109;
                        v59 = sub_411d90(v50, v91, g_41d868, g_41d864);
                        if (v0)
                        {
LABEL_4094c0:
                            v0 = v59;
                            free(ptr1);
                            v59 = v0;
                            if (!v59)
                                goto LABEL_4092c7;
                            goto LABEL_4094da;
                        }
LABEL_4092ac:
                        *((char *)(iter + v84)) = v1;
                        *((char *)(iter1 + v85)) = v2;
                        goto LABEL_4092bf;
                    }
                }
                if ((char)node[54])
                {
                    v109 = v55;
                    v108 = v54;
                    v111 = sub_407030(iter, NULL);
                    v84 = v108;
                    v85 = v109;
                    v59 = v111 - sub_407030(iter1, NULL);
                    goto LABEL_4092a0;
                }
                if (!(char)node[0x33])
                {
                    if ((char)node[56])
                    {
                        v109 = v55;
                        v108 = v54;
                        v84 = v108;
                        v85 = v109;
                        v59 = sub_40ca00(iter, v54, iter1, v55);
                        goto LABEL_4092a0;
                    }
                    else if (!v54)
                    {
                        v59 = -(NULL < v55);
                        v85 = v55;
                        v84 = v54;
                        goto LABEL_4092a0;
                    }
                    else
                    {
                        v59 = 1;
                        v85 = v55;
                        v84 = v54;
                        if (v85)
                        {
                            v109 = v55;
                            v108 = v54;
                            v85 = v109;
                            v84 = v108;
                            v59 = sub_413120(iter, v54 + 1, iter1, v55 + 1);
                            goto LABEL_4092a0;
                        }
                    }
                }
                v34 = (int)g_41d300;
                v35 = (int)g_41d310;
                v39 = g_41d350;
                v40 = g_41d358;
                v32 = g_41d350;
                v33 = g_41d358;
                v36 = (int)g_41d320;
                v37 = (int)g_41d330;
                v38 = (int)g_41d340;
                v27 = (int)g_41d300;
                v28 = (int)g_41d310;
                v29 = (int)g_41d320;
                v30 = (int)g_41d330;
                v31 = (int)g_41d340;
                if (g_41d861)
                {
                    ptr = NULL;
                    v108 = iter + v54;
                    v5 = iter1 + v55;
                    v14 = iter;
                    v10 = (v54 + v55) * 3 + 2;
                    v12 = &v27;
                    v109 = 0xfa0;
                    v11 = &v34;
                    v16 = iter1;
                    v17 = v54;
                    v18 = v55;
                    v9 = 0;
                    v15 = node;
                    v95 = v13;
                }
                else
                {
                    v108 = v55;
                    v109 = v54;
                    MD5_Update(&v27, iter, v54);
                    MD5_Final(&v25, &v27);
                    v11 = &v34;
                    MD5_Update(&v34, iter1, v108);
                    MD5_Final(&v26, v11);
                    v109 = v108;
                    v108 = v109;
                    v110 = memcmp(&v25, &v26, 16);
                    if (!v110)
                    {
                        v8 = v109;
                        v98 = v108;
                        v95 = iter;
                        v7 = iter1;
                        ptr = NULL;
                        goto LABEL_409d87;
                    }
                    else
                    {
                        ptr = NULL;
LABEL_409cac:
                        v5 = v110;
                        free(ptr);
                        v84 = v108;
                        v85 = v109;
                        v59 = v5;
                        goto LABEL_4092a0;
                    }
                }
            }
        }
    }
    while (1)
    {
        v96 = v109;
        if (v10 <= v96)
        {
LABEL_409a30:
            if (v108 > iter)
                goto LABEL_409b4f;
            goto LABEL_409a3b;
        }
LABEL_409b04:
        v97 = v96 * 3 >> 1;
        v109 = (v97 <= v10 ? v10 : v97);
        free(ptr);
        ptr = malloc(v109);
        if (!ptr)
        {
            v109 = 0xfa0;
            v95 = v13;
            goto LABEL_409a30;
        }
        v95 = ptr;
        if (v108 <= iter)
        {
LABEL_409a3b:
            if (v5 <= iter1)
            {
                v8 = NULL;
                v98 = NULL;
                goto LABEL_409a51;
            }
            else
            {
                v99 = v109;
                v100 = v95;
                v98 = NULL;
                goto LABEL_409b7e;
            }
        }
LABEL_409b4f:
        v98 = sub_407800(v95, iter, v109) + 1;
        if (v5 > iter1)
        {
            if (v109 < v98)
            {
                v99 = NULL;
                v100 = NULL;
            }
            else
            {
                v100 = v95 + v98;
                v99 = v109 - v98;
            }
LABEL_409b7e:
            v8 = sub_407800(v100, iter1, v99) + 1;
            v102 = v8 + v98;
            goto LABEL_409b93;
        }
        v8 = NULL;
        v102 = v98;
LABEL_409b93:
        if (v109 >= v98 && v109 >= v102)
        {
LABEL_409a51:
            v7 = v95 + v98;
            goto LABEL_409a5a;
        }
        if (v102 <= 0x5555555555555554)
            v102 = v102 * 3 >> 1;
        free(ptr);
        v95 = sub_412ab0(v102);
        if (v108 <= iter && v5 <= iter1)
        {
            ptr = v95;
            iter = v14;
            v7 = v95 + v98;
            node = v15;
            iter1 = v16;
            v104 = v17;
            v105 = v18;
            v106 = v9;
            goto LABEL_409c1a;
        }
        strxfrm(v95, iter, v98);
        if (v5 > iter1)
        {
            v7 = v95 + v98;
            strxfrm(v7, iter1, v8);
            ptr = v95;
            v109 = v102;
LABEL_409a5a:
            if (v108 > iter)
            {
                iter = iter + strlen(iter) + 1;
                if (v5 <= iter1)
                    goto LABEL_409a7d;
            }
            else if (v5 <= iter1)
            {
                goto LABEL_40a076;
            }
            iter1 = iter1 + strlen(iter1) + 1;
            goto LABEL_409a7d;
        }
        v7 = v95 + v98;
        ptr = v95;
        v109 = v102;
        iter = iter + strlen(iter) + 1;
LABEL_409a7d:
        if (iter >= v108 && v5 <= iter1)
        {
LABEL_40a076:
            iter = v14;
            node = v15;
            iter1 = v16;
            v104 = v17;
            v105 = v18;
            v106 = v9;
LABEL_409c1a:
            v10 = v106;
            v9 = v105;
            v5 = v104;
            MD5_Update(v12, v95, v98);
            v109 = &v25;
            MD5_Final(&v25, v12);
            MD5_Update(v11, v7, v8);
            v108 = &v26;
            MD5_Final(&v26, v11);
            v107 = memcmp(v109, v108, 16);
            v108 = v5;
            v109 = v9;
            v110 = v10;
            if (v107)
            {
                v110 = v107;
                goto LABEL_409cac;
            }
            if (v110)
                goto LABEL_409cac;
LABEL_409d87:
            if (!memcmp(v95, v7, (v98 <= v8 ? v98 : v8)))
            {
                v110 = (v98 > v8) - (v98 < v8);
                goto LABEL_409cac;
            }
        }
        MD5_Update(v12, v95, v98);
        v103 = v8;
        MD5_Update(v11, v7, v103);
        if (!v9 && (v9 = (unsigned int)(int)(long long)memcmp(v95, v7, (v98 <= v103 ? v98 : v103)), !v9))
        {
            v96 = v109;
            v9 = (v98 > v103) - (v98 < v103);
            if (v10 <= v96)
                goto LABEL_409a30;
            goto LABEL_409b04;
        }
    }
}



// Function: sequential_sort @ 0xa180
void* sequential_sort(void* a0, unsigned long long a1, void* a2, char a3)
{
    void* v4;  // r13
    void* v5;  // rbx
    void* v14;  // r14
    void* v15;  // rbp
    void* v16;  // rax
    int v17;  // xmm5
    unsigned long long v18;  // r12
    void* v19;  // rax
    void* v20;  // rax
    unsigned long long i;  // r12
    int v22;  // xmm7
    unsigned long long v6;  // r12
    unsigned long long v24;  // r12
    void* v25;  // rax
    int v26;  // xmm1
    int v27;  // xmm0
    int v28;  // xmm6
    int v29;  // xmm7
    int v30;  // eax
    void* v31;  // rax
    void* v32;  // rdx
    int v33;  // xmm1
    unsigned long long v7;  // rax
    int v34;  // xmm0
    int v35;  // xmm1
    int v8;  // xmm1
    void* v9;  // r15
    void* v10;  // r13
    void* iter;  // rbx
    void* v13;  // r13
    long long v0;  // [bp-0x48]
    long long v1;  // [bp-0x48]
    void* v2;  // [bp-0x40]

    v4 = a0;
    v5 = a2;
    if (a1 != 2)
    {
        v6 = a1 >> 1;
        v0 = a1 - v6;
        v7 = v6 * 32;
        if (!a3)
        {
            v2 = a0 - v7;
            sequential_sort(v2, v0, a2, 0);
            if (a1 <= 3)
            {
                v8 = (int)*((int128_t *)((char *)v4 - 16));
                v9 = v2;
                *((int128_t *)((char *)v5 - 32)) = *((int128_t *)((char *)v4 - 32));
                *((uint128_t *)((char *)v5 - 16)) = (uint128_t)v8;
                v10 = v5;
                iter = v4;
            }
            else
            {
                sequential_sort(v4, v6, v5, 1);
                v9 = v2;
                v10 = v5;
                iter = v4;
            }
        }
        else
        {
            v9 = a2 + -(v7);
            sequential_sort(v2, v0, v9, 1);
            v10 = v4;
            iter = v5;
            if (a1 > 3)
            {
                sequential_sort(v4, v6, v5, 0);
                v10 = v4;
                iter = v5;
            }
        }
        v13 = v10;
        v14 = v13 - 32;
        v15 = v9 - 32;
        while (1)
        {
            iter -= 32;
            v16 = sub_40a0b0(v14, v15);
            if ((unsigned int)v16 > 0)
            {
                v1 = v0 - 1;
                *((int128_t *)iter) = *((int128_t *)((char *)v9 - 32));
                *((int128_t *)&iter[16]) = *((int128_t *)((char *)v9 - 16));
                if (v0 != 1)
                {
                    v9 = v15;
                    v15 -= 32;
                    v0 = v1;
                }
                else
                {
                    v19 = 0xffffffffffffffe0;
                    do
                    {
                        v20 = v19;
                        i = v6;
                        v22 = (int)*((int128_t *)(v13 + v20 + 16));
                        *((int128_t *)(iter + v20)) = *((int128_t *)(v13 + v20));
                        *((uint128_t *)(iter + v20 + 16)) = (uint128_t)v22;
                        v24 = i - 1;
                        v6 = v24;
                        v19 = v20 - 32;
                    } while (i != 1);
                    return v20 - 32;
                }
            }
            else
            {
                v17 = (int)*((int128_t *)((char *)v13 - 16));
                *((int128_t *)iter) = *((int128_t *)((char *)v13 - 32));
                *((uint128_t *)&iter[16]) = (uint128_t)v17;
                v18 = v6 - 1;
                if (v6 == 1)
                    return v16;
                v13 = v14;
                v14 -= 32;
                v6 = v18;
            }
        }
    }
    else
    {
        v25 = sub_40a0b0(a0 - 32, a0 - 64);
        if (a3)
        {
            v30 = (unsigned int)v25 > 0;
            v31 = (v30 - 2) * 32 + v4;
            v32 = ~(v30 & 1) * 32 + v4;
            v33 = (int)(int128_t)v32[16];
            *((int128_t *)((char *)v5 - 32)) = *((int128_t *)v32);
            v34 = (int)*((int128_t *)v31);
            *((uint128_t *)((char *)v5 - 16)) = (uint128_t)v33;
            v35 = (int)(int128_t)v31[16];
            *((uint128_t *)((char *)v5 - 64)) = (uint128_t)v34;
            *((uint128_t *)((char *)v5 - 48)) = (uint128_t)v35;
            return v31;
        }
        if ((unsigned int)v25 <= 0)
            return v25;
        v26 = (int)*((int128_t *)((char *)v4 - 32));
        v27 = (int)*((int128_t *)((char *)v4 - 16));
        v28 = (int)*((int128_t *)((char *)v4 - 64));
        v29 = (int)*((int128_t *)((char *)v4 - 48));
        *((uint128_t *)((char *)v5 - 32)) = (uint128_t)v26;
        *((uint128_t *)((char *)v5 - 16)) = (uint128_t)v27;
        *((uint128_t *)((char *)v4 - 32)) = (uint128_t)v28;
        *((uint128_t *)((char *)v4 - 16)) = (uint128_t)v29;
        *((uint128_t *)((char *)v4 - 64)) = (uint128_t)v26;
        *((uint128_t *)((char *)v4 - 48)) = (uint128_t)v27;
        return v25;
    }
}



// Function: write_unique @ 0xa3c0
typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_0 {
    void* field_0;
    char field_8;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_41d419;
extern void g_41d840;
extern uint128_t g_41d850;

void write_unique(struct_0 *a0, struct_1 *a1, unsigned long long a2)
{
    if (!g_41d419)
    {
        sub_4082d0(a0, a1, a2);
        return;
    }
    if (*((long long *)&g_41d840))
    {
        a2 = a2;
        a0 = a0;
        if (!sub_40a0b0())
            return;
    }
    *((int128_t *)&g_41d840) = *((int128_t *)&a0);
    g_41d850 = *((int128_t *)((char *)&a0[1].field_0 + 7));
    sub_4082d0(a0, a1, a2);
    return;
}



// Function: mergefps @ 0xa430
typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_5 {
    struct struct_2 *field_0;
    char field_8;
    char padding_9[7];
    long long field_10;
    unsigned long long field_18;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char field_30;
} struct_5;

typedef struct struct_0 {
    void* field_0;
    char field_8;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_41d030;
extern unsigned long long g_41d410;
extern char g_41d419;
extern unsigned long long g_41d438;

long long mergefps(unsigned long a0, unsigned long long a1, unsigned long long a2, struct_1 *a3, unsigned long long a4, void* a5)
{
    void* v15;  // r12
    void* ptr;  // rbp
    unsigned long v25;  // rsi
    unsigned long v26;  // rdx
    void* v27;  // rcx
    unsigned long long v28;  // rax
    unsigned long long v29;  // rcx
    unsigned long v30;  // rax
    unsigned long long idx;  // rax
    unsigned long long v32;  // r14
    unsigned long long v33;  // rax
    void* v34;  // rbx
    unsigned long long idx1;  // r15
    void* v35;  // r13
    unsigned long v36;  // rax
    char *index;  // r13
    unsigned long long v38;  // r14
    struct_0 ptr1;  // rdi
    struct_0 v40;  // rax
    void* dst;  // rdi
    unsigned long long v42;  // r13
    unsigned long long *idx2;  // rdx
    unsigned long long v44;  // rax
    unsigned long long v18;  // rax
    unsigned long v45;  // rdx
    unsigned long long v46;  // rax
    unsigned long long v47;  // rcx
    void* node;  // rax
    unsigned long long v49;  // rbx
    unsigned long long v50;  // rax
    unsigned long long v51;  // rdx
    unsigned long long l;  // rax
    void* iter;  // rdx
    int v54;  // xmm0
    struct_5 *v19;  // r12
    int v55;  // xmm1
    int v56;  // xmm2
    unsigned long v57;  // r9
    int v58;  // xmm3
    unsigned long v59;  // r9
    unsigned long v60;  // r9
    void* m;  // rax
    void* v62;  // rax
    unsigned long long v63;  // r14
    unsigned long long v64;  // r13
    unsigned long v20;  // rbx
    void* v65;  // rbx
    unsigned int v66;  // eax
    unsigned long long v67;  // rax
    void* k;  // rax
    void* v69;  // rax
    void* ptr2;  // rax
    unsigned long long j;  // rbp
    void* v23;  // r10
    void* v24;  // rbx
    long long *v0;  // [bp-0xd8], Other Possible Types: void*
    void* v1;  // [bp-0xd0], Other Possible Types: unsigned long long
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
    v4 = sub_412bf0(a2, 56);
    v15 = sub_412bf0(a2, 8);
    v3 = sub_412bf0(a2, 8);
    v10 = (struct_0)0;
    ptr = sub_412bf0(a2, 8);
    v9 = g_41d410;
    if (a2)
    {
        v1 = v15;
        idx1 = 0;
        while (1)
        {
            v18 = g_41d438 / v2;
            if (v18 < *((long long *)&g_41d030))
                v18 = *((long long *)&g_41d030);
            v19 = v4 + idx1 * 56;
            while (1)
            {
                v20 = (v18 & 0xffffffffffffffe0) + 32;
                ptr2 = malloc(v20);
                v19->field_0 = ptr2;
                if (ptr2)
                    break;
                v18 = v20 >> 1;
                if (v20 <= 66)
                    sub_412fb0(); /* do not return */
            }
            v19->field_18 = v20;
            j = idx1 * 8;
            v23 = a5 + j;
            v19->field_28 = 32;
            v24 = idx1 * 16 + a0;
            v25 = *((long long *)v23);
            v19->field_10 = 0;
            *((unsigned long *)&v19->field_20) = 0;
            v26 = *((long long *)v24);
            *((unsigned long *)&v19->field_8) = 0;
            v19->field_30 = 0;
            v0 = v23;
            if ((char)sub_407ef0(v19, v25, v26))
            {
                v27 = v1;
                v28 = &v19->field_0[v19->field_18];
                *((unsigned long long *)((char *)v27 + 8 * idx1)) = v28 - 32;
                *((unsigned long long *)((char *)v3 + 8 * idx1)) = v28 - v19->field_10 * 32;
                idx1 += 1;
                if (idx1 < v2)
                    continue;
                ptr = v5;
                v15 = v27;
                goto LABEL_40a5e2;
            }
            else
            {
                sub_407660(*((long long *)v0), *((long long *)v24));
                if (idx1 < iter1)
                {
                    iter1 -= 1;
                    sub_408f90(*((long long *)v24));
                }
                free(v19->field_0);
                v29 = v2 - 1;
                if (v29 > idx1)
                {
                    do
                    {
                        v30 = *((long long *)((char *)a5 + j + 8));
                        *((int128_t *)(a0 + j * 2)) = *((int128_t *)(a0 + j * 2 + 16));
                        *((unsigned long *)((char *)a5 + j)) = v30;
                        j += 8;
                    } while (v2 * 8 - 8 != j);
                    v2 = v29;
                }
                else
                {
                    v15 = v1;
                    ptr = v5;
                    if (v29)
                    {
                        v2 = v29;
LABEL_40a5e2:
                        idx = 0;
                        do
                        {
                            *((unsigned long long *)((char *)ptr + 8 * idx)) = idx;
                            idx += 1;
                        } while (idx != v2);
                        if (v2 != 1)
                        {
                            v32 = 1;
                            while (1)
                            {
                                v33 = v32 * 8;
                                v34 = ptr + v33;
                                v35 = ptr + v33 - 8;
                                if (sub_40a0b0(*((long long *)((char *)v15 + 8 * *((long long *)v35))), *((long long *)((char *)v15 + 8 * *((long long *)v34)))) > 0)
                                {
                                    v36 = *((long long *)v35);
                                    v32 = 1;
                                    *((long long *)v35) = *((long long *)v34);
                                    *((unsigned long *)v34) = v36;
                                }
                                else
                                {
                                    v32 += 1;
                                    if (v32 >= v2)
                                        break;
                                }
                            }
                        }
                        v7 = 0;
                        v5 = NULL;
                        v8 = &v10;
                        while (1)
                        {
                            index = *((long long *)((char *)v15 + 8 * *((long long *)ptr)));
                            if (g_41d419)
                            {
                                if (v5)
                                {
                                    if (!sub_40a0b0(v5, index))
                                        goto LABEL_40a6b6;
                                    sub_4082d0(v8, a3, a4);
                                }
                                v38 = *((long long *)&index[8]);
                                ptr1 = v10;
                                if (v38 > v7)
                                {
                                    do
                                    {
                                        if (!v7)
                                            goto LABEL_40ab1d;
                                    } while ((v7 *= 2, v38 > v7));
                                    v38 = v7;
LABEL_40ab1d:
                                    free(ptr1);
                                    v40 = (struct_0)sub_412ab0(v38);
                                    v7 = v38;
                                    v38 = *((long long *)&index[8]);
                                    v10 = v40;
                                    ptr1 = v10;
                                }
                                v11 = v38;
                                dst = memcpy(ptr1, *((long long *)&index), v38);
                                v5 = v8;
                                if (v9)
                                {
                                    v12 = *((long long *)&index[16]) - *((long long *)&index) + dst;
                                    v13 = dst + *((long long *)&index[24]) - *((long long *)&index);
                                }
                            }
                            else
                            {
                                sub_4082d0(index, a3, a4);
                            }
LABEL_40a6b6:
                            v1 = *((long long *)ptr);
                            if (*((long long *)((char *)v3 + 8 * v1)) >= index)
                            {
                                if (!(char)sub_407ef0(v4 + v1 * 56, *((long long *)((char *)a5 + 8 * v1)), *((long long *)(a0 + v1 * 16))))
                                {
                                    v47 = *((long long *)ptr);
                                    node = ptr + 8;
                                    if (v2 != 1)
                                    {
                                        do
                                        {
                                            if (*((long long *)node) > v47)
                                            {
                                                *((unsigned long long *)node) = *((long long *)node) - 1;
                                                v47 = *((long long *)ptr);
                                            }
                                        } while ((node += 8, ptr + v2 * 8 != node));
                                    }
                                    v49 = v2 - 1;
                                    sub_407660(*((long long *)((char *)a5 + 8 * v47)), *((long long *)(a0 + v47 * 16)));
                                    v50 = *((long long *)ptr);
                                    if (v50 < iter1)
                                    {
                                        iter1 -= 1;
                                        sub_408f90(*((long long *)(a0 + v50 * 16)));
                                        v50 = *((long long *)ptr);
                                    }
                                    free(*((long long *)((char *)v4 + 56 * v50)));
                                    v51 = *((long long *)ptr);
                                    if (v49 > v51)
                                    {
                                        l = v51 * 8;
                                        iter = v4 + (l - v51) * 8;
                                        do
                                        {
                                            v54 = (int)*((int128_t *)(a0 + l * 2 + 16));
                                            iter += 56;
                                            v55 = (int)*((int128_t *)iter);
                                            v56 = (int)(int128_t)iter[16];
                                            *((long long *)((char *)a5 + l)) = *((long long *)((char *)a5 + l + 8));
                                            v57 = (long long)iter[48];
                                            v58 = (int)(int128_t)iter[32];
                                            *((uint128_t *)(a0 + l * 2)) = (uint128_t)v54;
                                            *((unsigned long *)((char *)iter - 8)) = v57;
                                            v59 = *((long long *)((char *)v15 + l + 8));
                                            *((uint128_t *)((char *)iter - 56)) = (uint128_t)v55;
                                            *((unsigned long *)((char *)v15 + l)) = v59;
                                            v60 = *((long long *)((char *)v3 + l + 8));
                                            *((uint128_t *)((char *)iter - 40)) = (uint128_t)v56;
                                            *((unsigned long *)((char *)v3 + l)) = v60;
                                            l += 8;
                                            *((uint128_t *)((char *)iter - 24)) = (uint128_t)v58;
                                        } while (v2 * 8 - 8 != l);
                                    }
                                    if (!v49)
                                        break;
                                    m = ptr;
                                    do
                                    {
                                        v62 = m + 8;
                                        *((long long *)((char *)v62 - 8)) = (long long)m[8];
                                        m = v62;
                                    } while (ptr + v2 * 8 - 8 != m);
                                    v2 = v49;
                                    continue;
                                }
                                v42 = v2;
                                v1 = *((long long *)ptr);
                                idx2 = v4 + v1 * 56;
                                v44 = idx2[3] + *(idx2);
                                v45 = idx2[2];
                                *((unsigned long long *)((char *)v15 + 8 * v1)) = v44 - 32;
                                *((unsigned long long *)((char *)v3 + 8 * v1)) = v44 - v45 * 32;
                                v46 = v1;
                                if (v42 == 1)
                                    goto LABEL_40a81c;
                                goto LABEL_40a761;
                            }
                            else
                            {
                                v46 = v1;
                                *((char **)((char *)v15 + 8 * v1)) = index - 32;
                                v42 = v2;
                                if (v42 != 1)
                                {
LABEL_40a761:
                                    v63 = 1;
                                    v0 = v15 + v46 * 8;
                                    v64 = 1;
                                    while (1)
                                    {
                                        v65 = ptr + v63 * 8;
                                        v66 = sub_40a0b0(*(v0), *((long long *)((char *)v15 + 8 * *((long long *)v65))));
                                        if (v66 >= 0 && (v66 || *((long long *)v65) <= v1))
                                        {
                                            v64 = v63 + 1;
                                            v63 = v64 + v42 >> 1;
                                            if (v64 >= v42)
                                                break;
                                        }
                                        else
                                        {
                                            v42 = v63;
                                            v63 = v64 + v42 >> 1;
                                            if (v64 >= v42)
                                                break;
                                        }
                                    }
                                    v67 = v64 - 1;
                                    if (v67)
                                    {
                                        k = ptr;
                                        do
                                        {
                                            v69 = k + 8;
                                            *((long long *)((char *)v69 - 8)) = (long long)k[8];
                                            k = v69;
                                        } while (k != ptr + v64 * 8 - 8);
                                    }
                                    *((unsigned long long *)((char *)ptr + 8 * v67)) = v1;
                                }
                                else
                                {
LABEL_40a81c:
                                    *((unsigned long long *)ptr) = v46;
                                }
                            }
                        }
                        if (v5 && g_41d419)
                        {
                            sub_4082d0(&v10, a3, a4);
                            free(v10);
                            break;
                        }
                    }
                    break;
                }
            }
        }
    }
    sub_407660(a3, a4);
    free(a5);
    free(v4);
    free(ptr);
    free(v3);
    return (unsigned long long)free(v15);
}



// Function: mergefiles @ 0xabe0
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char field_c;
    char field_d;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_0 *field_8;
} struct_1;

long long mergefiles(struct_1 *a0, unsigned long long a1, unsigned long a2, struct_1 *a3, unsigned long long a4)
{
    unsigned long v2;  // r12
    char *v3;  // rax
    unsigned long long *v0;  // [bp-0x48], Other Possible Types: void*

    v2 = sub_408980(a0, a2, &v0);
    if (a2 <= v2)
    {
        sub_40a430(a0, a1, v2, a3, a4, v0);
        return v2;
    }
    else if (v2 > 1)
    {
        sub_40a430(a0, a1, v2, a3, a4, v0);
        return v2;
    }
    else
    {
        v3 = dcgettext(NULL, "open failed", 5);
        sub_407480(v3, *((long long *)&a0[v2].field_0)); /* do not return */
    }
}



// Function: merge @ 0xac90
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char field_c;
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

extern unsigned int g_41d024;
extern stat g_41d260;
extern char g_41d268;
extern int g_41d2f0;

void merge(void* a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    void* v9;  // r13
    unsigned long node;  // rbp
    unsigned long j;  // rdi
    void* ptr;  // r10
    unsigned long long v21;  // rdx
    void* ptr1;  // r15
    unsigned long v23;  // rax
    unsigned long v24;  // rsi
    unsigned long v25;  // rsi
    unsigned long v26;  // rax
    unsigned long v27;  // rdx
    unsigned long iter;  // r14
    unsigned long v11;  // rsi
    struct_0 *v29;  // r15
    void* iter1;  // rbx
    unsigned int v31;  // r13d
    int v32;  // eax
    struct_0 *v33;  // r13
    struct_0 *v34;  // rax
    unsigned long v35;  // r14
    unsigned long v36;  // rax
    unsigned long v37;  // rax
    unsigned long v38;  // r15
    unsigned long long iter2;  // r12
    void* v39;  // rbp
    void* v40;  // rbp
    struct_0 *v41;  // rax
    unsigned long v42;  // rbp
    unsigned long v43;  // rbx
    unsigned long v44;  // r14
    char *v46;  // rax
    unsigned long long v13;  // rbx
    struct_0 *v14;  // rax
    unsigned long v15;  // rsi
    unsigned long v16;  // rax
    unsigned long v17;  // rdx
    unsigned long long v18;  // rcx
    stat *v0;  // [bp-0x110], Other Possible Types: struct_0 *, unsigned long
    struct_0 *i;  // [bp-0x108], Other Possible Types: unsigned long
    char *v2;  // [bp-0x100], Other Possible Types: struct_0 *, void*
    unsigned long v3;  // [bp-0xf0]
    void* v4;  // [bp-0xf0]
    void* v5;  // [bp-0xe8]
    FILE *v6;  // [bp-0xe0]
    stat v7;  // [bp-0xd8]

    v9 = a0;
    node = a1;
    v11 = g_41d024;
    i = a2;
    if (g_41d024 < a2)
    {
        do
        {
            iter2 = 0;
            v13 = 0;
            if (i >= v11)
            {
                do
                {
                    v14 = sub_408c20(&v6, 0);
                    v0 = v14 + 1;
                    if (g_41d024 <= node)
                        v15 = g_41d024;
                    else
                        v15 = node;
                    v16 = sub_40abe0(v13 * 16 + v9, v15, g_41d024, v6, v0);
                    v11 = g_41d024;
                    if (v16 <= node)
                        v17 = v16;
                    else
                        v17 = node;
                    v13 += v16;
                    v18 = iter2 * 16;
                    iter2 += 1;
                    j = i - v13;
                    *((struct_0 **)((char *)v9 + v18)) = v0;
                    *((struct_0 **)((char *)v9 + v18 + 8)) = v14;
                    node -= v17;
                } while (g_41d024 <= j);
                ptr = v9 + v18 + 16;
                v21 = iter2 % g_41d024;
                ptr1 = v13 * 16 + v9;
                v23 = g_41d024 - v21;
            }
            else
            {
                j = i;
                v23 = v11;
                ptr = v9;
                ptr1 = v9;
                v21 = 0;
            }
            if (j > v23)
            {
                v4 = ptr;
                v0 = j + v21 + 1 - v11;
                v2 = sub_408c20(&v6, 0);
                v0 = v2 + 1;
                if (v0 <= node)
                {
                    v24 = v0;
                    v25 = v24;
                }
                else
                {
                    v25 = node;
                }
                v26 = sub_40abe0(ptr1, v25, v24, v6, v0);
                if (v26 <= node)
                    v27 = v26;
                else
                    v27 = node;
                iter2 += 1;
                v13 += v26;
                *((struct_0 **)v4) = v0;
                *((struct_0 **)&v4[8]) = v2;
                node -= v27;
                ptr1 = v13 * 16 + v9;
                ptr = iter2 * 16 + v9;
            }
            node += iter2;
            memmove(ptr, ptr1, (i - v13) * 16);
            v11 = g_41d024;
            i += iter2 - v13;
        } while (g_41d024 < i);
    }
    if (node < i)
    {
        v2 = v9;
        iter = node;
        v0 = &v7;
        v29 = NULL;
        iter1 = node * 16 + v9;
        while (1)
        {
LABEL_40ae28:
            v31 = strcmp(*((long long *)iter1), "-");
            if (!a3 || strcmp(a3, *((long long *)iter1)) || !v31)
                break;
LABEL_40ae9e:
            v33 = v29 + 1;
            if (!v29)
            {
                v34 = sub_408c20(&v6, 0);
                v33 = v34 + 1;
                v29 = v34;
                sub_40abe0(iter1, 0, 1, v6, v33);
            }
            *((struct_0 **)iter1) = v33;
            iter += 1;
            iter1 += 16;
            *((struct_0 **)((char *)iter1 - 8)) = v29;
            if (iter == i)
                goto LABEL_40aec5;
        }
        v32 = g_41d2f0;
        if (!v32)
        {
            if (!fstat(1, &g_41d260.st_mode))
            {
                g_41d2f0 = 0xffffffff;
LABEL_40ae68:
                if (!v31)
                {
                    if (!fstat(0, v0) && v7.st_ino == *((long long *)&g_41d268))
                        goto LABEL_40ae8c;
                }
                else
                {
                    if (!stat(*((long long *)iter1), v0) && v7.st_ino == *((long long *)&g_41d268))
                    {
LABEL_40ae8c:
                        if (*((unsigned long long *)&v7) == *((long long *)&g_41d260.st_mode))
                            goto LABEL_40ae9e;
                    }
                }
                iter += 1;
                iter1 += 16;
                if (iter == i)
                    goto LABEL_40aec5;
                else
                    goto LABEL_40ae28;
            }
            else
            {
                v32 = *(__errno_location());
                g_41d2f0 = v32;
            }
        }
        if (v32 < 0)
            goto LABEL_40ae68;
LABEL_40aec5:
        v9 = v2;
        node = v3;
    }
    v0 = node;
    v35 = i;
    v2 = &v5;
    while (1)
    {
        v36 = sub_408980(v9, v35, v2);
        if (v35 == v36)
        {
            v37 = sub_4074e0(a3, "w");
            if (v37)
            {
                sub_40a430(v9, v0, v35, v37, a3, v5);
                return;
            }
            if (*(__errno_location()) != 24 || v35 <= 2)
                break;
        }
        else if (v36 <= 2)
        {
            v46 = dcgettext(NULL, "open failed", 5);
            sub_407480(v46, *((long long *)((char *)v9 + 16 * v36))); /* do not return */
        }
        v38 = v36 - 1;
        v39 = v9 + v38 * 16;
        while (1)
        {
            sub_407660(*((long long *)((char *)v5 + 8 * v38)), *((long long *)v39));
            v40 = v39 - 16;
            if (sub_408c20(&v6, v38 > 2))
                break;
            v38 -= 1;
            v39 = v40;
        }
        v42 = v0;
        i = v41;
        v0 = v41 + 1;
        if (v42 > v38)
            v43 = v38;
        else
            v43 = v42;
        v44 = v35 - v38;
        sub_40a430(v9, v43, v38, v6, v0, v5);
        v35 = v44 + 1;
        *((struct_0 **)v9) = v0;
        *((struct_0 **)&v9[8]) = i;
        memmove(v9 + 16, v39, v44 * 16);
        v0 = v42 - v43 + 1;
    }
    sub_407480(dcgettext(NULL, "open failed", 5), a3); /* do not return */
}



// Function: sortlines @ 0xb220
typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_3 {
    char *field_0;
} struct_3;

typedef struct struct_2 {
    void* field_0;
    void* field_8;
    void* field_10;
    void* field_18;
    struct struct_3 *field_20;
    long long field_28;
    long long field_30;
    struct struct_1 *field_38;
    char padding_40[16];
    unsigned int field_50;
    char field_54;
} struct_2;

typedef struct struct_0 {
    unsigned long long field_0;
    void* field_8;
    void* field_10;
    unsigned long long field_18;
    char padding_20[8];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[8];
    unsigned long long field_40;
    void* field_48;
    char padding_50[4];
    char field_54;
} struct_0;

void sortlines(unsigned long a0, unsigned long long a1, unsigned long a2, struct_0 *index, long long *a4, struct_1 *a5, unsigned long a6)
{
    unsigned long long v11;  // r14
    unsigned long long v12;  // r12
    struct_2 *idx;  // rbx
    unsigned int v21;  // eax
    void* v22;  // r9
    void* v23;  // rsi
    unsigned long long v24;  // rcx
    char **v25;  // rdx
    void* v26;  // rdi
    void* v27;  // rax
    char *iter;  // r15
    void* v29;  // r11
    void* v13;  // r8
    struct_2 *idx1;  // r15
    char *node;  // rbx
    unsigned long long v32;  // r12
    unsigned long v33;  // rcx
    int v34;  // xmm0
    unsigned long v35;  // rax
    int v36;  // xmm2
    char v37;  // r8b
    long long v38;  // r9
    long long v39;  // r10
    unsigned long long v14;  // rax
    long long v40;  // r12
    long long v41;  // r11
    unsigned long long v42;  // rax
    void* v43;  // rcx
    int v44;  // xmm4
    void* v45;  // r9
    unsigned long long v46;  // rsi
    void* v47;  // rax
    void* v48;  // rax
    int v49;  // xmm6
    void* v15;  // r15
    void* v50;  // rax
    struct_2 *idx2;  // rbp
    unsigned long long v52;  // r12
    unsigned long v53;  // rdi
    unsigned long v54;  // rdi
    long long v55;  // rdx
    unsigned long long v56;  // r12
    unsigned long long v57;  // rbp
    void* v58;  // rdi
    long long v59;  // rsi
    void* v16;  // r9, Other Possible Types: unsigned long long
    unsigned long long v60;  // r12
    unsigned long long v61;  // rbp
    void* v62;  // rdi
    struct_1 *v63;  // rsi
    long long *v17;  // r14
    long long *v18;  // rbp
    struct_2 *v19;  // rax
    struct_2 *v0;  // [bp-0xb0]
    void* v1;  // [bp-0xa0], Other Possible Types: long long *
    unsigned long v2;  // [bp-0x90]
    char v3;  // [bp-0x80]
    unsigned long v4;  // [bp-0x78]
    unsigned long long v5;  // [bp-0x70]
    unsigned long v6;  // [bp-0x68]
    unsigned long v7;  // [bp-0x60]
    long long *v8;  // [bp-0x58]
    struct_1 *v9;  // [bp-0x50]
    unsigned long v10;  // [bp-0x48]

    v11 = index->field_28;
    v12 = index->field_30;
    v2 = a6;
    v4 = a0;
    v5 = a1 >> 1;
    v7 = index->field_40;
    v6 = a2;
    v8 = a4;
    v9 = a5;
    v10 = a6;
    if (v11 + v12 > 0x1ffff)
    {
        v16 = a1;
        if (a1 > 1)
        {
            if (pthread_create(&v3, 0, thread_entry, &v4))
            {
                v11 = index->field_28;
                v12 = index->field_30;
            }
            else
            {
                sortlines(a0 - index->field_28 * 32, v16 - v5, a2, index->field_48, a4, a5, v2);
                pthread_join(v3, 0);
                return;
            }
        }
    }
    v13 = a0 - a2 * 32;
    v14 = v11 * 32;
    v15 = a0 - v14;
    v16 = -(v14);
    if (v12 > 1)
    {
        v16 = v13;
        sub_40a180(v15, v12, v13 - (v11 >> 1) * 32, 0);
        v13 = v16;
    }
    if (v11 > 1)
        sub_40a180(a0, v11, v13, 0);
    index->field_0 = a0;
    v17 = a4 + 6;
    index->field_8 = v15;
    index->field_10 = v15;
    index->field_18 = a0 + v16 - v12 * 32;
    v18 = a4 + 1;
    pthread_mutex_lock(v18);
    sub_40df40(*(a4), index);
    index->field_54 = 1;
    pthread_cond_signal(v17);
    pthread_mutex_unlock(v18);
    while (1)
    {
        pthread_mutex_lock(v18);
        while (1)
        {
            if (sub_40e010(*(a4)))
                break;
            pthread_cond_wait(v17, v18);
        }
        idx = v19;
        pthread_mutex_unlock(v18);
        v0 = (char *)&idx[1].field_0 + 3;
        pthread_mutex_lock(v0);
        v21 = idx->field_50;
        idx->field_54 = 0;
        if (!v21)
            break;
        v16 = idx->field_0;
        v22 = v16;
        v16 = idx->field_8;
        v23 = idx->field_10;
        v24 = (a2 >> ((char)v21 * 2 + 2 & 63)) + 1;
        if (v21 != 1)
        {
            v25 = &idx->field_20->field_0;
            v26 = v16;
            v27 = v16;
            iter = *(v25);
            v29 = v26;
            if (v26 != v23)
            {
                v1 = v22;
                idx1 = idx;
                node = iter;
                while (1)
                {
                    if (idx1->field_18 == v27)
                    {
                        idx = idx1;
                        v26 = idx->field_0;
                        iter = node;
                        v37 = idx->field_54;
                        v25 = &idx->field_20->field_0;
                        v38 = (long long)(v1 - v29) >> 5;
                        v39 = (long long)(v16 - v27) >> 5;
                        goto LABEL_40b66d;
                    }
                    v32 = v24 - 1;
                    if (!v24)
                    {
                        idx = idx1;
                        v26 = idx->field_0;
                        iter = node;
                        v37 = idx->field_54;
                        v24 = 0xffffffffffffffff;
                        v25 = &idx->field_20->field_0;
                        v38 = (long long)(v1 - v29) >> 5;
                        v39 = (long long)(v16 - v27) >> 5;
LABEL_40b66d:
                        v40 = idx->field_30;
                        v41 = idx->field_28;
                        if (v40 != v39)
                            goto LABEL_40b4f5;
                        if (v23 != v26)
                        {
                            v42 = v24 - 1;
                            if (v24)
                            {
                                v43 = v26 - 32;
                                do
                                {
                                    v26 = v43;
                                    v44 = (int)*((int128_t *)v26);
                                    iter -= 32;
                                    idx->field_0 = v26;
                                    *((uint128_t *)&iter) = (uint128_t)v44;
                                    *((int128_t *)&iter[16]) = (int128_t)v26[16];
                                } while (v26 != v23 && (v42 -= 1, v43 = v26 - 32, v42 != 0xffffffffffffffff));
                            }
                        }
                        goto LABEL_40b4fe;
                    }
                    node -= 32;
                    if (sub_40a0b0(v29 - 32, v27 - 32) > 0)
                    {
                        v33 = idx1->field_8;
                        v29 = idx1->field_0;
                        v23 = idx1->field_10;
                        v34 = (int)*((int128_t *)(v33 - 32));
                        v27 = v33 - 32;
                        idx1->field_8 = v27;
                        *((uint128_t *)&node) = (uint128_t)v34;
                        *((int128_t *)&node[16]) = *((int128_t *)(v33 - 16));
                        if (v23 == v29)
                            goto LABEL_40b4b8;
                    }
                    else
                    {
                        v35 = idx1->field_0;
                        v23 = idx1->field_10;
                        v36 = (int)*((int128_t *)(v35 - 32));
                        v29 = v35 - 32;
                        idx1->field_0 = v29;
                        *((uint128_t *)&node) = (uint128_t)v36;
                        v27 = idx1->field_8;
                        *((int128_t *)&node[16]) = *((int128_t *)(v35 - 16));
                        if (v23 == v29)
                        {
LABEL_40b4b8:
                            idx = idx1;
                            v37 = idx->field_54;
                            iter = node;
                            v25 = &idx->field_20->field_0;
                            v26 = v29;
                            v24 = v32;
                            v38 = (long long)(v1 - v29) >> 5;
                            v39 = (long long)(v16 - v27) >> 5;
                            goto LABEL_40b4e8;
                        }
                    }
                    v24 = v32;
                }
                goto LABEL_40b4fe;
            }
            else
            {
                v37 = 0;
                v39 = 0;
                v38 = 0;
            }
LABEL_40b4e8:
            v40 = idx->field_30;
            v41 = idx->field_28;
            if (v39 == v40)
                goto LABEL_40b4fe;
LABEL_40b4f5:
            if (v38 == v41)
            {
                v45 = idx->field_18;
                if (v45 != v27)
                {
                    v46 = v24 - 1;
                    if (v24)
                    {
                        v47 = v27 - 32;
                        do
                        {
                            v48 = v47;
                            v49 = (int)*((int128_t *)v48);
                            iter -= 32;
                            idx->field_8 = v48;
                            *((uint128_t *)&iter) = (uint128_t)v49;
                            *((int128_t *)&iter[16]) = (int128_t)v48[16];
                            if (v48 == v45)
                            {
                                v39 = (long long)(v16 - v48) >> 5;
                                goto LABEL_40b4fe;
                            }
                        } while ((v46 -= 1, v47 = v48 - 32, v46 != 0xffffffffffffffff));
                        v39 = (long long)(v16 - v48) >> 5;
                    }
                }
            }
LABEL_40b4fe:
            *(v25) = iter;
            goto LABEL_40b501;
        }
        else
        {
            v50 = idx->field_8;
            if (v16 != v23)
            {
                v1 = v18;
                idx2 = idx;
                while (1)
                {
                    if (idx2->field_18 == v50)
                    {
                        idx = idx2;
                        v18 = v1;
                        goto LABEL_40b808;
                    }
                    v52 = v24 - 1;
                    if (!v24)
                    {
                        idx = idx2;
                        v24 = 0xffffffffffffffff;
                        v18 = v1;
LABEL_40b808:
                        v39 = idx->field_30;
                        if ((long long)(v16 - v50) >> 5 == v39)
                        {
                            v26 = v22;
                            if (idx->field_10 != v22 && !(v56 = v24 - 1, !v24))
                            {
                                v1 = v18;
                                do
                                {
                                    v57 = v56;
                                    v58 = v26 - 32;
                                    idx->field_0 = v58;
                                    sub_40a3c0(v58, a5, v2);
                                    v26 = idx->field_0;
                                } while (v26 != idx->field_10 && (v56 = v57 - 1, v57 >= 1));
                                v18 = v1;
                                v37 = idx->field_54;
                                v39 = (long long)(v16 - idx->field_8) >> 5;
                                v41 = idx->field_28;
                                v40 = idx->field_30;
                                goto LABEL_40b501;
                            }
                            else
                            {
                                v37 = idx->field_54;
                                v41 = idx->field_28;
                                v40 = v39;
                                goto LABEL_40b501;
                            }
                        }
                        else
                        {
                            v59 = idx->field_28;
                            v55 = (long long)(v16 - v22) >> 5;
                            goto LABEL_40b8a8;
                        }
                    }
                    if (sub_40a0b0(v22 - 32, v50 - 32) > 0)
                    {
                        v53 = idx2->field_8 - 32;
                        idx2->field_8 = v53;
                        sub_40a3c0(v53, a5, v2);
                        v22 = idx2->field_0;
                        v50 = idx2->field_8;
                        if (v22 == idx2->field_10)
                            goto LABEL_40b5f5;
                    }
                    else
                    {
                        v54 = idx2->field_0 - 32;
                        idx2->field_0 = v54;
                        sub_40a3c0(v54, a5, v2);
                        v22 = idx2->field_0;
                        v50 = idx2->field_8;
                        if (v22 == idx2->field_10)
                        {
LABEL_40b5f5:
                            idx = idx2;
                            v24 = v52;
                            v18 = v1;
                            v55 = (long long)(v16 - v22) >> 5;
                            v39 = (long long)(v16 - v50) >> 5;
                            goto LABEL_40b618;
                        }
                    }
                    v24 = v52;
                }
                goto LABEL_40b501;
            }
            else
            {
                v39 = 0;
                v55 = 0;
            }
LABEL_40b618:
            v41 = idx->field_28;
            v59 = v41;
            if (idx->field_30 == v39)
            {
                v37 = idx->field_54;
                v26 = v22;
                v40 = v39;
                goto LABEL_40b501;
            }
LABEL_40b8a8:
            v41 = v59;
            if (v59 != v55 || v50 == idx->field_18 || (v60 = v24 - 1, !v24))
            {
                v26 = v22;
                v37 = idx->field_54;
                v40 = idx->field_30;
                v39 = (long long)(v16 - idx->field_8) >> 5;
            }
            else
            {
                v1 = v18;
                do
                {
                    v61 = v60;
                    v62 = v50 - 32;
                    idx->field_8 = v62;
                    sub_40a3c0(v62, a5, v2);
                    v50 = idx->field_8;
                } while (v50 != idx->field_18 && (v60 = v61 - 1, v61 >= 1));
                v18 = v1;
                v37 = idx->field_54;
                v26 = idx->field_0;
                v41 = idx->field_28;
                v40 = idx->field_30;
                v39 = (long long)(v16 - v50) >> 5;
            }
LABEL_40b501:
            idx->field_28 = v41 - ((long long)(v16 - v26) >> 5);
            idx->field_30 = v40 - v39;
            if (!v37)
                sub_40b1a0(a4, idx);
            if (idx->field_50 > 1)
            {
                pthread_mutex_lock(&idx->field_38[1].padding_0[3]);
                v63 = idx->field_38;
                if (!idx->field_38->field_54)
                {
                    sub_40b1a0(a4);
                    v63 = idx->field_38;
                }
                pthread_mutex_unlock(&v63[1].field_28);
            }
            else if (!(idx->field_30 + idx->field_28))
            {
                pthread_mutex_lock(v18);
                sub_40df40(*(a4), idx->field_38);
                idx->field_38->field_54 = 1;
                pthread_cond_signal(v17);
                pthread_mutex_unlock(v18);
            }
            pthread_mutex_unlock(v0);
        }
    }
    pthread_mutex_unlock(v0);
    pthread_mutex_lock(v18);
    sub_40df40(*(a4), idx);
    idx->field_54 = 1;
    pthread_cond_signal(v17);
    pthread_mutex_unlock(v18);
    return;
}



// Function: sortlines_thread @ 0xba80
typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned int field_10;
    char padding_14[4];
    struct struct_0 *field_18;
    long long *field_20;
    struct struct_1 *field_28;
    unsigned int field_30;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

unsigned int sortlines_thread(struct_0 *a0)
{
    sub_40b220(*((long long *)&a0->field_0), a0->field_8, *((long long *)&a0->field_10), a0->field_18, a0->field_20, a0->field_28, *((long long *)&a0->field_30));
    return 0;
}



// Function: usage @ 0xbab0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
    char padding_1[15];
    char *field_10;
} struct_0;

extern unsigned long long stderr;
extern FILE *stdout;

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
    char *v36;  // rax
    char *v37;  // rax
    char *v38;  // rax
    char *v39;  // rax
    char *v41;  // rax
    char *v42;  // rax
    char *v43;  // rax
    char *v44;  // rax
    char *v45;  // rax
    unsigned long long v19;  // r12
    char *v46;  // rsi
    struct_0 *v47;  // rbx
    char *v48;  // r14
    char *v50;  // rax
    char *v51;  // rax
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
        v25 = dcgettext(NULL, "Write sorted concatenation of all FILE(s) to standard output.\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "\nWith no FILE, or when FILE is -, read standard input.\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "Ordering options:\n\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n", 5);
        fputs_unlocked(v29, stdout);
        v30 = dcgettext(NULL, "  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < 'JAN' < ... < 'DEC'\n", 5);
        fputs_unlocked(v30, stdout);
        v31 = dcgettext(NULL, "  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n", 5);
        fputs_unlocked(v31, stdout);
        v32 = dcgettext(NULL, "  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n", 5);
        fputs_unlocked(v32, stdout);
        v33 = dcgettext(NULL, "      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n", 5);
        fputs_unlocked(v33, stdout);
        v34 = dcgettext(NULL, "Other options:\n\n", 5);
        fputs_unlocked(v34, stdout);
        v35 = dcgettext(NULL, "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n", 5);
        fputs_unlocked(v35, stdout);
        v36 = dcgettext(NULL, "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n", 5);
        fputs_unlocked(v36, stdout);
        v37 = dcgettext(NULL, "      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n", 5);
        fputs_unlocked(v37, stdout);
        v38 = dcgettext(NULL, "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n", 5);
        fputs_unlocked(v38, stdout);
        v39 = dcgettext(NULL, "  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n", 5);
        fputs_unlocked(v39, stdout);
        __printf_chk(1, dcgettext(NULL, "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n", 5));
        v41 = dcgettext(NULL, "  -z, --zero-terminated     line delimiter is NUL, not newline\n", 5);
        fputs_unlocked(v41, stdout);
        v42 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v42, stdout);
        v43 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v43, stdout);
        v44 = dcgettext(NULL, "\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line's end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n", 5);
        fputs_unlocked(v44, stdout);
        v45 = dcgettext(NULL, "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n", 5);
        fputs_unlocked(v45, stdout);
        v0 = (int)_INSERT(v0 CONCAT 0, 0, "[");
        v46 = "[";
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
        } while (strcmp("sort", v46) && (v46 = v47->field_10, v23 = v47 + 16, v47->field_10));
        v48 = *((long long *)&v23->padding_1[7]);
        if (!*((long long *)&v23->padding_1[7]))
            v48 = "sort";
        __printf_chk(1, dcgettext(NULL, "\n%s online help: <%s>\n", 5));
        v50 = setlocale(5, NULL);
        if (v50 && strncmp(v50, "en_", 3))
        {
            v51 = dcgettext(NULL, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
            fputs_unlocked(v51, stdout);
        }
        __printf_chk(1, dcgettext(NULL, "Full documentation <%s%s>\n", 5));
        __printf_chk(1, dcgettext(NULL, "or available locally via: info '(coreutils) %s%s'\n", 5));
    }
    exit(a0); /* do not return */
}



// Function: raw_hasher @ 0xccd0
void raw_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: raw_comparator @ 0xccf0
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0xcd00
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



// Function: hash_find_entry @ 0xcd90
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
LABEL_40ce3f:
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
        goto LABEL_40ce3f;
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



// Function: transfer_entries @ 0xcfe0
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
LABEL_40d008:
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
                            goto LABEL_40d08e;
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
LABEL_40d08e:
                            v2 = iter->field_0;
                            goto LABEL_40d091;
                        }
                    }
                    index = v6;
                }
            }
LABEL_40d091:
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_40d008;
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



// Function: hash_print_statistics @ 0xd250
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
            goto LABEL_40d389;
LABEL_40d30a:
        v12 = (int)v5;
    }
    else
    {
        v11 = MulV(AddV(v6 >> 1 | (unsigned int)v6 & 1, v6 >> 1 | (unsigned int)v6 & 1), 0x4059000000000000);
        if (v5 >= 0)
            goto LABEL_40d30a;
LABEL_40d389:
        v12 = (int)(AddV(v5 >> 1 | (unsigned int)v5 & 1, v5 >> 1 | (unsigned int)v5 & 1));
    }
    __fprintf_chk(a1, 0x1, "# buckets used:    %lu (%.2f%%)\n", v6, (unsigned long long)(DivV(v11, v12)));
    return __fprintf_chk(*((unsigned int *)&&v0), 0x1, "max bucket length: %lu\n", v2);
}



// Function: hash_lookup @ 0xd3b0
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

int hash_lookup(void)
{
    unsigned long long v1;  // rsi
    struct_0 *v2;  // rdi
    unsigned long long v3;  // rax
    struct_3 *iter;  // rbx
    unsigned long long i;  // rsi

    v3 = v2->field_30(v1, v2->field_10);
    if (v3 >= v2->field_10)
        abort(); /* do not return */
    iter = v3 * 16 + v2->field_0;
    if (!iter->padding_0)
        return;
    for (i = (unsigned long long)iter->padding_0; i != v1; i = (unsigned long long)iter->padding_0)
    {
        if ((char)v2->field_38(v1))
            return;
        iter = iter->field_8;
        if (!iter)
            return;
    }
    return;
}



// Function: hash_get_next @ 0xd470
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40d47c();
    return;
}



// Function: hash_do_for_each @ 0xd550
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



// Function: hash_initialize @ 0xd640
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

    v1 = sub_40ccd0;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_40ccf0;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_417200.padding_0[0];
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_40cd00(ptr))
    {
        v6 = sub_40cea0(a0, v2->field_10);
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



// Function: hash_clear @ 0xd750
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



// Function: hash_free @ 0xd800
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
            goto LABEL_40d8ac;
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
LABEL_40d8ac:
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



// Function: hash_rehash @ 0xd8f0
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

    v11 = sub_40cea0(a1, idx->field_28->field_10);
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
        v13 = sub_40cfe0(&ptr, idx, 0);
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
        if ((char)sub_40cfe0(idx, &ptr, 1) && (char)sub_40cfe0(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0xda70
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
    sub_40cd90(idx, a1, &v0, 0);
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
                goto LABEL_40db86;
LABEL_40daff:
            v6 = (int)(unsigned int)v5;
        }
        else
        {
            v5 = *((long long *)&idx[1].field_0);
            v4 = AddV((unsigned int)(v3 >> 1) | (unsigned int)v3 & 1, (unsigned int)(v3 >> 1) | (unsigned int)v3 & 1);
            if (v5 >= 0)
                goto LABEL_40daff;
LABEL_40db86:
            v6 = (int)(AddV((unsigned int)(v5 >> 1 | (unsigned int)v5 & 1), (unsigned int)(v5 >> 1 | (unsigned int)v5 & 1)));
        }
        if (((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69 | (char)((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69) >> 6)) & 1) != 1)
        {
            sub_40cd00(idx);
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
                else if (!(char)sub_40d8f0(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else
                {
                    sub_40cd90(idx, a1, &v0, 0);
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



// Function: hash_insert @ 0xdca0
long long hash_insert(unsigned int *a0, long long a1)
{
    unsigned int v2;  // eax
    long long v0;  // [bp-0x18], Other Possible Types: unsigned long long

    v2 = sub_40da70(a0, a1, &v0);
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



// Function: hash_remove @ 0xdd00
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

    sub_40cd90(idx, a1, &v0, 1);
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
            goto LABEL_40de46;
LABEL_40dd90:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = *((long long *)&idx[1].field_0);
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_40dd90;
LABEL_40de46:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_40cd00(idx);
    v9 = *((long long *)&idx[2].field_8);
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_40d8f0(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



// Function: num_processors @ 0xe570
long long num_processors(unsigned int a0)
{
    long long v2;  // rbx
    long long v3;  // rax
    unsigned long v4;  // rax
    long long v5;  // rbx
    unsigned long v6;  // rax
    long long v7;  // rax
    unsigned long v8;  // rax
    long long v9;  // rax
    long long v10;  // rax
    char v0;  // [bp-0xa8]

    if (a0 != 2)
    {
        if (a0 != 1)
        {
            v2 = sysconf(83);
            if (v2 - 1 <= 1)
            {
                if (sched_getaffinity(0, 128, &v0))
                    return v2;
                v3 = __sched_cpucount(128, &v0);
                if (!v3)
                    return v2;
                if (v2 >= v3)
                    return v2;
                v2 = v3;
            }
            if (v2 <= 0)
                return 1;
            return v2;
        }
LABEL_40e735:
        v7 = -0x1;
    }
    else
    {
        v4 = getenv("OMP_NUM_THREADS");
        if (v4)
        {
            v5 = sub_40e4b0(v4);
            v6 = getenv("OMP_THREAD_LIMIT");
            if (v6)
            {
                v7 = sub_40e4b0(v6);
                if (!v7)
                    v7 = -0x1;
            }
            else
            {
                v7 = -0x1;
            }
            if (v5)
            {
                if (v7 > v5)
                    v7 = v5;
                return v7;
            }
        }
        else
        {
            v8 = getenv("OMP_THREAD_LIMIT");
            if (!v8 || (v7 = (long long)(unsigned long long)sub_40e4b0(v8), !v7))
                goto LABEL_40e735;
        }
    }
    if (!sched_getaffinity(0, 128, &v0))
    {
        v9 = __sched_cpucount(128, &v0);
        if (v9)
        {
            if (v7 > v9)
                v7 = v9;
            return v7;
        }
    }
    v10 = sysconf(84);
    if (v10 <= v7)
        v7 = v10;
    if (v10 <= 0)
        v7 = 1;
    return v7;
}



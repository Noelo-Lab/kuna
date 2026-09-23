// Function: main @ 0x2a20
typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

typedef struct struct_4 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[8];
    long long field_28;
    char padding_30[8];
    long long field_38;
    unsigned long long field_40;
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

typedef struct struct_7 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[8];
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[8];
    struct struct_7 *field_40;
} struct_7;

typedef struct struct_3 {
    struct struct_3 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
    unsigned long long field_50;
    unsigned long long field_58;
    unsigned long long field_60;
} struct_3;

typedef struct struct_5 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[24];
    unsigned long long field_38;
    unsigned long long field_40;
    char padding_48[8];
    unsigned long long field_50;
    char padding_58[16];
    struct struct_5 *field_68;
} struct_5;

typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

typedef struct struct_6 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_6;

extern char g_40ca08;
extern struct_6 g_40cd00;
extern struct_3 g_40cd20;
extern struct struct_0 *g_4108a0[2];
extern option g_410940;
extern unsigned long long g_411010;
extern long long g_411018;
extern unsigned int g_411020;
extern struct_1 *stdout;
extern void* stdin;
extern int optind;
extern unsigned long long optarg;
extern char g_4110e9;
extern char g_4110ea;
extern char g_4110eb;
extern char g_4110ec;
extern char g_4110ed;
extern char g_4110ee;
extern char g_4110ef;
extern char g_4110f0;
extern char g_4110f1;
extern char g_4110f2;

unsigned int main(int a0, void* a1)
{
    unsigned int v35;  // r15d
    unsigned long v36;  // rbp
    int v45;  // esi
    struct_0 **iter;  // rbx
    char *v157;  // rax
    unsigned long idx1;  // rax
    unsigned int v159;  // esi
    int m;  // r15d
    unsigned long v161;  // rax
    unsigned long v162;  // r12
    struct_5 *idx2;  // rax
    unsigned long long v164;  // rbx
    unsigned long v165;  // rbp
    int v166;  // eax
    unsigned long long v167;  // rax
    long long n;  // r15
    long long v169;  // r15
    char *v170;  // rax
    struct_5 *v171;  // rax
    unsigned long long v172;  // r13
    unsigned int v173;  // eax
    unsigned long v174;  // r12
    char *v49;  // rax
    unsigned long v175;  // r14
    char v176;  // bl
    char v177;  // al
    struct_5 *v178;  // rbx
    unsigned long long v179;  // rax
    char v180;  // al
    char *v50;  // rsi
    char *v51;  // rax
    char *v52;  // rax
    char *v53;  // rax
    unsigned long v37;  // rdi
    long long v55;  // rax
    char v56;  // dl
    char v57;  // al
    char v58;  // al
    unsigned int v59;  // esi
    unsigned long i;  // rax
    void* v61;  // r14
    unsigned int v62;  // ebp
    unsigned int v63;  // eax
    unsigned int v64;  // r15d
    long long v38;  // rdi
    unsigned int v65;  // esi
    unsigned int v66;  // eax
    unsigned long v67;  // rax
    void* v68;  // r13
    void* v70;  // r14
    struct_3 *v71;  // r13
    struct_5 *v72;  // rax
    struct_5 *node;  // rbp
    unsigned long v74;  // rsi
    long long v39;  // rsi
    unsigned long v75;  // 4099
    unsigned long long v76;  // rdi
    unsigned int v77;  // esi
    unsigned long long v78;  // rax
    unsigned long long v79;  // r8
    unsigned long v80;  // rdx
    unsigned long v81;  // rcx
    unsigned long long l;  // rdx
    unsigned long v83;  // rdi
    unsigned long v84;  // rdi
    long long v40;  // rdx
    unsigned long v85;  // rdx
    unsigned int v86;  // eax
    unsigned long v88;  // rbx
    unsigned long long v89;  // rax
    unsigned long v90;  // rdx
    unsigned long long v91;  // rdx
    unsigned long long v92;  // r12
    unsigned long long v93;  // r14
    unsigned long long v94;  // rbp
    long long v41;  // rcx
    struct_3 *v95;  // rax
    unsigned long long v96;  // rax
    unsigned long long v97;  // rdi
    unsigned long long v98;  // rax
    long long v99;  // r14
    long long v100;  // r14
    unsigned long long v101;  // rax
    long long v102;  // rdi
    long long v103;  // rdi
    unsigned long long v104;  // rbp
    long long v42;  // r8
    unsigned long long iter1;  // r15
    unsigned long long v106;  // rdi
    unsigned long long v107;  // r11
    unsigned long long v108;  // rdx
    unsigned long long v109;  // r8
    struct_7 *v110;  // rbx
    unsigned long long v111;  // rax
    unsigned long long v112;  // rcx
    unsigned long v113;  // r15
    unsigned long v114;  // r14
    long long v43;  // r9
    unsigned long long v115;  // rax
    struct_7 *v116;  // rbx
    unsigned long long i0;  // rdx
    long long v118;  // rax
    unsigned long long v119;  // rdx
    unsigned int v120;  // eax
    unsigned long long v121;  // rbx
    unsigned long long v122;  // r13
    unsigned long long v123;  // r13
    struct_5 *iter2;  // r13
    unsigned int v44;  // eax
    unsigned long long v125;  // rdx
    struct_1 *v126;  // r15
    struct_3 *v0;  // [bp-0x1f0], Other Possible Types: unsigned long long, unsigned int, unsigned long
    char *v1;  // [bp-0x1e8], Other Possible Types: struct_4 **, int, long long, unsigned long long
    char *v2;  // [bp-0x1e0], Other Possible Types: unsigned long long
    char *v3;  // [bp-0x1d8], Other Possible Types: struct_4 **, unsigned long
    char *v4;  // [bp-0x1d0], Other Possible Types: struct_3 *, unsigned long
    char *v5;  // [bp-0x1c8], Other Possible Types: struct_4 **, unsigned long, unsigned long long
    char *v6;  // [bp-0x1c0], Other Possible Types: unsigned long long
    char *v7;  // [bp-0x1b8], Other Possible Types: unsigned long, unsigned long long
    unsigned long v8;  // [bp-0x1b0]
    unsigned long long v9;  // [bp-0x1a8]
    void* v10;  // [bp-0x1a0]
    unsigned long v11;  // [bp-0x198]
    char v12;  // [bp-0x18d]
    unsigned int v13;  // [bp-0x18c]
    char v14;  // [bp-0x180]
    char v15;  // [bp-0x178]
    char v16;  // [bp-0x170]
    struct_5 *v17;  // [bp-0x168]
    unsigned int v18;  // [bp-0x160]
    unsigned int v19;  // [bp-0x15c]
    unsigned int v20;  // [bp-0x158]
    unsigned int v21;  // [bp-0x154]
    int v22;  // [bp-0x150]
    int v23;  // [bp-0x14c]
    struct_4 *v24;  // [bp-0x148], Other Possible Types: int
    struct_4 *v25;  // [bp-0x138], Other Possible Types: int
    struct_5 *v26;  // [bp-0x128]
    struct_5 *v27;  // [bp-0x120]
    int <0x402a20[is_2]|Stack bp-0x118, 1 B>;  // [bp-0x118]
    int v28;  // [bp-0x118]
    char *v29;  // [bp-0xf0]
    char *v30;  // [bp-0xe8]
    stat v31;  // [bp-0xd8]
    char v32;  // [bp-0x42]
    char v33;  // [bp-0x41]

    v35 = 0;
    v36 = a0;
    v37 = *((long long *)a1);
    g_411020 = 2;
    sub_4050e0(v37);
    setlocale(6, &g_40ca08);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_405020(0);
    sub_407940(v38, v39, v40, v41, v42, v43);
    *((unsigned int *)&v0) = 0;
    while (1)
    {
        v44 = getopt_long(v36, a1, "aeimvx3AEL:TX", &g_410940.name, NULL);
        if (v44 == 0xffffffff)
            break;
        switch (v44)
        {
        case 51:
            g_4110ed = 1;
            v35 |= 1;
            break;
        case 65:
            g_4110eb = 1;
            v35 |= 2;
            g_4110ef = 1;
            break;
        case 69:
            g_4110ef = 1;
            v35 |= 4;
            break;
        case 76:
            v45 = v0;
            if (v45 > 2)
                sub_4042a0("too many file label options", 0); /* do not return */
            *((unsigned long long *)((char *)&(&<0x402a20[is_2]|Stack bp-0x118, 1 B>)[v45] - 32)) = optarg;
            v0 = v45 + 1;
            break;
        case 84:
            g_4110ee = 1;
            break;
        case 88:
            g_4110ec = 1;
            v35 |= 8;
            break;
        case 97:
            g_4110f2 = 1;
            break;
        case 101:
            v35 |= 16;
            break;
        case 105:
            g_4110ea = 1;
            break;
        case 109:
            g_4110e9 = 1;
            break;
        case 118:
            v55 = sub_405580("Randy Smith");
            sub_407150(stdout, "diff3", "GNU diffutils", g_411018, v55, 0);
            sub_404690(v38, v39, v40, v41, v42, v43);
            return 0;
        case 120:
            g_4110ec = 1;
            v35 |= 32;
            break;
        case 128:
            g_411010 = optarg;
            break;
        case 129:
            iter = &g_4108a0[0];
            __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n", 5));
            __printf_chk(1, "%s\n\n", dcgettext(NULL, "Compare three files line by line.", 5));
            v49 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
            fputs_unlocked(v49, stdout);
            v50 = "-A, --show-all              output all changes, bracketing conflicts";
            while (1)
            {
                if (!*(v50))
                {
                    v51 = &stdout->field_28->field_0;
                    if (v51 < *((long long *)&stdout[1].padding_0[0]))
                    {
                        stdout->field_28 = v51 + 1;
                        *(v51) = 10;
                    }
                    else
                    {
                        __overflow(stdout, 10);
                    }
                    v50 = &iter[1]->field_0;
                    iter += 1;
                    if (!v50)
                        break;
                }
                else
                {
                    iter += 1;
                    dcgettext(NULL, v50, 5);
                    __printf_chk(1, "  %s\n");
                    v50 = &*(iter)->field_0;
                    if (!v50)
                        break;
                }
            }
            v52 = dcgettext(NULL, "\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n", 5);
            fputs_unlocked(v52, stdout);
            v53 = dcgettext(NULL, "Exit status is 0 if successful, 1 if conflicts, 2 if trouble.", 5);
            __printf_chk(1, "\n%s\n%s\n", dcgettext(NULL, "If a FILE is '-', read standard input.", 5), v53);
            sub_407230(v38, v39, v40, v41, v42, v43);
            sub_404690(v38, v39, v40, v41, v42, v43);
            return 0;
        case 130:
            g_4110f1 = 1;
            break;
        default:
            sub_4042a0(0, 0); /* do not return */
        }
    }
    v56 = v35;
    g_4110f0 = (g_4110e9 ^ 1) & v56;
    v57 = !v35 & g_4110e9;
    g_4110eb = g_4110eb | v57;
    v58 = v57 | g_4110ef;
    g_4110ef = v57 | g_4110ef;
    if (v35 & v35 - 1 || !(v12 = g_4110e9 & g_4110ea, !(g_4110e9 & g_4110ea) && (!(unsigned int)v0 || v58)))
        sub_4042a0("incompatible options", 0); /* do not return */
    v59 = (v36 & 0xffffffff) - optind;
    if (v59 != 3)
    {
        if (v59 <= 2)
            sub_4042a0("missing operand after '%s'", *((long long *)((char *)a1 + 8 * v36 - 8))); /* do not return */
        sub_4042a0("extra operand '%s'", *((long long *)((char *)a1 + 8 * optind + 24))); /* do not return */
    }
    i = (int)v0;
    for (v10 = a1 + optind * 8; (unsigned int)i <= 2; i += 1)
    {
        *((long long *)((char *)&(&<0x402a20[is_2]|Stack bp-0x118, 1 B>)[i] - 32)) = *((long long *)((char *)v10 + 8 * i));
    }
    v61 = v10;
    v62 = v56 | g_4110e9;
    v63 = 2 - v62;
    *((unsigned int *)&v9) = v63;
    v8 = v63;
    if (!strcmp(*((long long *)((char *)v61 + 8 * v8)), "-"))
    {
        v64 = v62 + 1;
        *((unsigned int *)&v9) = v64;
        if (!strcmp(*((long long *)v61), "-") || !(v8 = (unsigned long)(long long)(int)v64, (int)(long long)strcmp(*((long long *)((char *)v61 + 8 * v8)), "-")))
            sub_404300("'-' specified for more than one input file"); /* do not return */
    }
    v65 = v9;
    v18 = 0;
    v21 = 0;
    v66 = 3 - v65;
    v20 = v65;
    v13 = v66;
    v19 = v66;
    v67 = v66;
    (&v21)[v67] = 1;
    v11 = v67;
    (&v21)[v8] = 2;
    v68 = v10;
    do
    {
        if (strcmp(*((long long *)v68), "-"))
        {
            if (stat(*((long long *)v68), &v31) < 0)
            {
                sub_404330(*((long long *)v68)); /* do not return */
            }
            else if (((unsigned short)v31.st_nlink & 0xf000) == 0x4000)
            {
                error(2, 21, "%s");
            }
        }
    } while ((v68 += 8, v10 + 24 != v68));
    signal(0x11, NULL);
    v70 = v10;
    v71 = &g_40cd20.field_0;
    v27 = sub_4046e0(*((long long *)((char *)v70 + 8 * v22)), *((long long *)((char *)v70 + 8 * v23)), &v14, &v16);
    v26 = sub_4046e0(*((long long *)v70), *((long long *)((char *)v70 + 8 * v23)), &v14, &v15);
    v17 = NULL;
    v4 = &v17;
    while (1)
    {
        v72 = v26;
        node = v27;
        if (v72)
        {
            *((unsigned long long *)&(&v25)[8]) = 0;
            v25 = 0;
            *((unsigned long long *)&(&v24)[8]) = 0;
            v24 = 0;
            if (node)
            {
                v74 = node->field_10;
                v75 = v72->field_10;
                v76 = v74 < v75;
                v77 = v74 < v75;
                v78 = v75 <= v74;
                node = (&v26)[v76];
                v79 = v76 * 8;
            }
            else
            {
                node = v72;
                v77 = 0;
                v79 = 0;
                v78 = 1;
                v76 = 0;
            }
        }
        else
        {
            if (!node)
                break;
            *((unsigned long long *)&(&v25)[8]) = 0;
            v78 = 0;
            v79 = 8;
            v77 = 1;
            v25 = 0;
            v76 = 1;
            *((unsigned long long *)&(&v24)[8]) = 0;
            v24 = 0;
        }
        v80 = node->field_40;
        v81 = node->field_18;
        (&v25)[v76] = node;
        (&v24)[v76] = node;
        (&v26)[v76] = v80;
        l = (&v26)[v78];
        node->field_40 = 0;
        if (l)
        {
            do
            {
                if (*((long long *)(l + 16)) > v81 + 1)
                    break;
                if ((&v24)[v78])
                    (&v25)[v78]->field_40 = l;
                else
                    (&v24)[v78] = l;
                v83 = (&v26)[v78];
                (&v25)[v78] = l;
                v84 = *((long long *)(v83 + 64));
                *((unsigned long *)(l + 64)) = 0;
                v85 = *((long long *)(l + 24));
                (&v26)[v78] = v84;
                v86 = v77 ^ 1;
                if (v81 < v85)
                {
                    v86 = v77;
                    v81 = v85;
                    v77 = v86;
                }
                v78 = v86;
                l = (&v26)[v78];
            } while (l);
            v76 = v77;
        }
        v6 = &v28;
        v7 = &v28 - 16;
        v3 = &v24;
        v0 = *((long long *)(*((long long *)((char *)&v24 + v79)) + 16));
        v88 = (&v25)[v76]->field_18;
        v89 = 0;
        do
        {
            v90 = *((long long *)((char *)&v24 + v89));
            if (v90)
            {
                *((unsigned long *)((char *)&v28 + v89)) = v0 - *((long long *)(v90 + 16)) + *((long long *)v90);
                v91 = v88 - *((long long *)(*((long long *)(v89 + (char *)&v25)) + 24)) + *((long long *)(*((long long *)(v89 + (char *)&v25)) + 8));
            }
            else
            {
                *((unsigned long long *)((char *)&v28 + v89)) = v0 - v71->field_30 + *((long long *)((char *)v71 + 2 * v89 + 16));
                v91 = v88 - v71->field_30 + *((long long *)((char *)v71 + 2 * v89 + 16));
            }
        } while ((*((unsigned long long *)((char *)&v28 + v89 - 16)) = v91, v89 += 8, v89 != 16));
        v92 = *((unsigned long long *)&v28);
        v93 = *((unsigned long long *)(&v28 + 24));
        v94 = *((unsigned long long *)(&v28 + 8));
        v2 = *((unsigned long long *)(&v28 + 16));
        v95 = sub_407350(112);
        *((unsigned int *)&v95->field_0) = 0;
        v71 = v95;
        v71[1].field_0 = 0;
        v71->field_8 = v92;
        v96 = v2;
        v71->field_18 = v94;
        v71->field_10 = v96;
        v97 = v96 - v92;
        v71->field_20 = v93;
        v71->field_28 = v0;
        v71->field_30 = v88;
        if (v97 == 0xffffffffffffffff)
        {
            v71->field_38 = 0;
            v71->field_50 = 0;
        }
        else
        {
            v1 = v97 + 1;
            v71->field_38 = sub_4076b0(v1, 8);
            v98 = sub_4076b0(v1, 8);
            v93 = v71->field_20;
            v94 = v71->field_18;
            v71->field_50 = v98;
        }
        v99 = v93 - v94;
        v100 = v99 + 1;
        if (v99 == 0xffffffffffffffff)
        {
            v71->field_40 = 0;
            v101 = 0;
        }
        else
        {
            v71->field_40 = sub_4076b0(v100, 8);
            v101 = sub_4076b0(v100, 8);
        }
        v102 = v71->field_30 - v71->field_28;
        v71->field_58 = v101;
        v103 = v102 + 1;
        if (v102 == 0xffffffffffffffff)
        {
            v71->field_48 = 0;
            v104 = 0;
        }
        else
        {
            v71->field_48 = sub_4076b0(v103, 8);
            v104 = sub_4076b0(v103, 8);
        }
        v71->field_60 = v104;
        v5 = &v25;
        v1 = v3;
        do
        {
            iter1 = *(v1);
            if (!*(v1))
                continue;
            do
            {
                if (!(char)sub_404460(*((long long *)(iter1 + 40)), *((long long *)(iter1 + 56)), (*((long long *)(iter1 + 16)) - v0) * 8 + v71->field_48, v104 + (*((long long *)(iter1 + 16)) - v0) * 8, *((long long *)(iter1 + 24)) - *((long long *)(iter1 + 16)) + 1))
                    goto LABEL_4032db;
            } while ((iter1 = (unsigned long long)*((long long *)(iter1 + 64)), iter1));
        } while ((v1 += 8, v5 != v1));
        v106 = v2;
        v107 = 0;
        while (1)
        {
            v108 = v92;
            v109 = v106 + 1;
            v110 = *((long long *)((char *)v3 + v107));
            v111 = 0;
            while (1)
            {
                v112 = v109;
                if (v110)
                    v112 = v110->field_0;
                if (v112 <= v108)
                    break;
                v108 += 1;
                *((long long *)(*((long long *)((char *)v71 + v107 + 56)) + v111)) = *((long long *)(v71->field_48 + v111));
                *((long long *)(*((long long *)((char *)v71 + v107 + 80)) + v111)) = *((long long *)(v104 + v111));
                v111 += 8;
            }
            if (v110)
            {
                v2 = v107;
                v113 = *((long long *)((char *)v71 + v107 + 56));
                v114 = *((long long *)((char *)v71 + v107 + 80));
                v1 = v106 + 1 - v92;
                while (1)
                {
                    v115 = v110->field_0 - v92;
                    if (!(char)sub_404460(v110->field_20, v110->field_30, v113 + v115 * 8, v114 + v115 * 8, v110->field_8 - v110->field_0 + 1))
                        break;
                    v116 = v110->field_40;
                    i0 = (v110->field_18 - v0) * 8 + 8;
                    for (v118 = v110->field_8 + 1 - v92; !v116; i0 = v119)
                    {
                        if (v118 >= v1)
                        {
                            v107 = v2;
                            goto LABEL_403447;
                        }
LABEL_40341b:
                        *((long long *)(v113 + v118 * 8)) = *((long long *)(v71->field_48 + i0));
                        v119 = i0 + 8;
                        *((long long *)(v114 + v118 * 8)) = *((long long *)(v104 + i0));
                        v118 += 1;
                    }
                    v110 = v116;
                    goto LABEL_40341b;
                }
LABEL_4032db:
                sub_404300("internal error: screwup in format of diff blocks"); /* do not return */
            }
LABEL_403447:
            v107 += 8;
            if (v107 == 16)
            {
                v120 = 6;
                if (*((unsigned long long *)&v24))
                {
                    if (*((unsigned long long *)((void*)&v24 + 8)))
                    {
                        v120 = 4;
                        v121 = v71->field_10 - v71->field_8;
                        if (v121 == v71->field_20 - v71->field_18)
                        {
                            v0 = v71;
                            v122 = 0;
                            do
                            {
                                if (v121 + 1 == v122)
                                {
                                    v71 = v0;
                                    v120 = 7;
                                    goto LABEL_403492;
                                }
                            } while (*((long long *)(v71->field_38 + v123 * 8)) && *((long long *)(v71->field_40 + v123 * 8)) && *((long long *)(v71->field_50 + v123 * 8)) == *((long long *)(v71->field_58 + v123 * 8)) && (v122 = v123 + 1, !(int)(long long)memcmp(*((long long *)(v71->field_38 + v123 * 8)), *((long long *)(v71->field_40 + v123 * 8)), *((long long *)(v71->field_50 + v123 * 8)))));
                            v71 = v0;
                            v120 = 4;
                        }
                    }
                    else
                    {
                        v120 = 5;
                    }
                }
LABEL_403492:
                *((unsigned int *)&v71->field_0) = v120;
                v4->field_0 = v71;
                v4 = v71 + 1;
                break;
            }
            else
            {
                v92 = *((long long *)&v6[v107]);
                v106 = *((long long *)&v7[v107]);
            }
        }
    }
    iter2 = v17;
    *((char *)&v4) = g_4110f0;
    if (!g_4110f0)
    {
        *((char *)&v3) = g_4110e9;
        if (g_4110e9)
        {
            sub_407810(*((long long *)v10), "r", stdin);
            v12 = 0;
            v125 = 0;
            v4 = v30;
            v126 = stdout;
            v5 = v29;
            v6 = *((unsigned long long *)(&v28 + 32));
            v7 = v8 * 16;
            v9 = v11 * 16;
        }
        else
        {
            v157 = "  ";
            if (g_4110ee)
                v157 = "\t";
            for (v3 = v157; iter2; iter2 = iter2->field_68)
            {
                if (iter2->field_0 == 4)
                {
                    v32 = 0;
                    v1 = 3;
                    v0 = 3;
                }
                else
                {
                    idx1 = iter2->field_0 - 5;
                    if ((unsigned int)idx1 > 2)
                        sub_404300("internal error: invalid diff type passed to output"); /* do not return */
                    v159 = (&v21)[idx1];
                    v33 = 0;
                    v0 = v159;
                    v32 = (char)v159 + 49;
                    v1 = !v159;
                }
                m = 0;
                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "====%s\n", &v32);
                do
                {
                    v161 = m;
                    v162 = (&v18)[v161];
                    v5 = v161;
                    idx2 = &(&iter2->field_0)[4 * v162];
                    v164 = idx2->field_8;
                    v165 = idx2->field_10;
                    v166 = m + 1;
                    *((int *)&v2) = v166;
                    __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%d:", v166);
                    v167 = v164;
                    if (v167 == v165)
                    {
                        __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%ldc\n", v164);
                    }
                    else if (v167 - v165 == 1)
                    {
                        __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%lda\n", v164 - 1);
                    }
                    else
                    {
                        __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%ld,%ldc\n", v164, v165);
                    }
                    if (v164 <= v165 && v1 != m)
                    {
                        n = 0;
                        v4 = v162 + 6;
                        do
                        {
                            v169 = n;
                            v7 = v165;
                            v6 = v164;
                            fputs_unlocked(v3, stdout);
                            n = v169 + 1;
                            fwrite_unlocked(*((long long *)((&iter2->field_8)[v4] + v169 * 8)), 1, *((long long *)((&iter2->field_50)[v162] + v169 * 8)), stdout);
                            v165 = v7;
                            v164 = v6;
                        } while (v165 - v164 >= n);
                        if (*((char *)(*((long long *)((&iter2->field_8)[v4] + v169 * 8)) + *((long long *)((&iter2->field_50)[v162] + v169 * 8)) - 1)) != 10)
                        {
                            v170 = dcgettext(NULL, "No newline at end of file", 5);
                            __fprintf_chk(*((unsigned int *)&stdout), 0x1, "\n\\ %s\n", v170);
                        }
                    }
                    m = v2;
                    if (v0 == 1)
                        m = (&g_40cd00.field_0)[v5];
                } while (m <= 2);
            }
            v12 = 0;
            goto LABEL_403918;
        }
    }
    else
    {
        v1 = v30;
        v2 = v29;
        for (v5 = *((unsigned long long *)(&v28 + 32)); iter2; iter2 = v171)
        {
            v171 = iter2->field_68;
            iter2->field_68 = node;
            node = iter2;
        }
        v172 = v8 * 16;
        v0 = v11 * 16;
        while (node)
        {
            if (node->field_0 == 4)
                goto LABEL_403da6;
            v173 = *((int *)((char *)&(&v21)[node->field_0] - 20)) + 5;
            if (*((int *)((char *)&(&v21)[node->field_0] - 20)) != 1)
            {
                switch (v173)
                {
                case 7:
                    if (g_4110ec)
                        goto LABEL_403d91;
                    v174 = node->field_8;
                    v175 = node->field_10;
LABEL_403f3d:
                    v178 = (char *)node + v172;
                    if (v178->field_10 - v178->field_8 != 0xffffffffffffffff)
                    {
                        v179 = v175 - v174;
                        if (v179 != 0xffffffffffffffff)
                        {
                            if (v179)
                            {
                                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%ld,%ldc\n", v174, v175);
                            }
                            else
                            {
                                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%ldc\n", v175);
                                break;
                            }
                        }
                        else
                        {
                            __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%lda\n", v175);
                            break;
                        }
                        v180 = sub_404500(stdout, node, (unsigned int)v9);
                        sub_4045f0(stdout, v180, v174, *((long long *)((char *)node + v172 + 16)) - v178->field_8 + 1);
                        goto LABEL_403d91;
                    }
                    else if (v174 != v175)
                    {
                        __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%ld,%ldd\n", v174, v175);
                        goto LABEL_403d91;
                    }
                    else
                    {
                        __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%ldd\n", v174);
                        goto LABEL_403d91;
                    }
                case 4:
LABEL_403da6:
                    *((char *)&v3) = g_4110ed;
                    if (g_4110ed)
                        goto LABEL_403d91;
                    v174 = node->field_8;
                    v175 = node->field_10;
                    if (!g_4110ef)
                        goto LABEL_403f3d;
                    __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%lda\n", v175);
                    v176 = g_4110eb;
                    if (g_4110eb)
                    {
                        __fprintf_chk(*((unsigned int *)&stdout), 0x1, "||||||| %s\n", v2);
                        v176 = sub_404500(stdout, node, v13);
                    }
                    fputs_unlocked("=======\n", stdout);
                    v177 = sub_404500(stdout, node, (unsigned int)v9);
                    __fprintf_chk(*((unsigned int *)&stdout), 0x1, ">>>>>>> %s\n", v1);
                    sub_4045f0(stdout, v176 | v177, v175 + 2, *((long long *)((char *)node + v0 + 16)) - *((long long *)((char *)node + v0 + 8)) + *((long long *)((char *)node + v172 + 16)) - *((long long *)((char *)node + v172 + 8)) + 3);
                    __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%lda\n<<<<<<< %s\n", v174 - 1, v5);
                    goto LABEL_404072;
                default:
LABEL_403d91:
                    node = node->field_68;
                    continue;
                }
            }
            else
            {
                if (!g_4110eb)
                    goto LABEL_403d91;
                v174 = node->field_8;
                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%lda\n", node->field_10);
                __fprintf_chk(*((unsigned int *)&stdout), 0x1, ">>>>>>> %s\n", v1);
                sub_4045f0(stdout, 0, node->field_10 + 2, *((long long *)((char *)node + v0 + 16)) - *((long long *)((char *)node + v0 + 8)) + *((long long *)((char *)node + v172 + 16)) - *((long long *)((char *)node + v172 + 8)) + 3);
                __fprintf_chk(*((unsigned int *)&stdout), 0x1, "%lda\n<<<<<<< %s\n", v174 - 1, v2);
                *((char *)&v3) = sub_404500(stdout, node, v13);
                fputs_unlocked("=======\n", stdout);
LABEL_404072:
                sub_4045f0(stdout, *((char *)&v3), v174 + 1, *((long long *)((char *)node + v0 + 16)) - *((long long *)((char *)node + v0 + 8)) + 1);
                v12 = v4;
                goto LABEL_403d91;
            }
        }
        if (g_4110ea)
            fputs_unlocked("w\nq\n", stdout);
LABEL_403918:
        free(v15);
        free(v16);
        sub_404690(v38, v39, v40, v41, v42, v43);
        exit(v12); /* do not return */
    }
}



// Function: try_help @ 0x42a0
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: fatal @ 0x4300
void fatal(char *a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    dcgettext(NULL, a0, 5);
    error(2, 0, "%s");
}



// Function: perror_with_exit @ 0x4330
void perror_with_exit(unsigned long a0)
{
    unsigned long long v2;  // r12
    int *err;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    err = __errno_location();
    error(2, *(err), "%s");
}



// Function: scan_diff_line @ 0x4360
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_0;

typedef struct struct_3 {
    char field_0;
} struct_3;

typedef struct struct_2 {
    char padding_0[40];
    struct struct_3 *field_28;
} struct_2;

extern struct_2 *stderr;
extern char g_4110f0;
extern char *g_421140;

struct_0 * scan_diff_line(struct_0 *a0, struct_0 **a1, unsigned long long *a2, struct_0 *a3, char a4)
{
    struct_0 *v1;  // rdi
    struct_0 *i;  // rbx
    struct_0 *v3;  // rbx
    unsigned long long v4;  // rsi
    char v5;  // al
    struct_0 *v6;  // r12
    unsigned long v7;  // rsi
    char *v8;  // rax

    if (a0->field_0 == a4 && a0->field_1 == 32)
    {
        v1 = &a0->field_2;
        *(a1) = v1;
        for (i = v1; i->field_0 != 10; i = &i->field_1);
        v4 = v3 - v1;
        *(a2) = v4;
        if (v3 >= a3)
        {
            return v3;
        }
        else if (v3->field_0 != 92)
        {
            return v3;
        }
        else
        {
            v5 = g_4110f0;
            if (!g_4110f0)
            {
                *(a2) = v4 - 1;
            }
            else
            {
                __fprintf_chk(*((unsigned int *)&stderr), 0x1, "%s:", g_421140);
                v5 = g_4110f0;
            }
            v6 = &i->field_2;
            while (1)
            {
                if (v5)
                {
                    v7 = v6->field_0;
                    v8 = &stderr->field_28->field_0;
                    if (v8 < *((long long *)&stderr[1].padding_0[0]))
                    {
                        stderr->field_28 = v8 + 1;
                        *(v8) = v7;
                    }
                    else
                    {
                        __overflow(stderr, v7);
                    }
                }
                v6 = &v6->field_1;
                if (*(&v6->field_0 - 1) == 10)
                    break;
                v5 = g_4110f0;
            }
            return v6;
        }
    }
    sub_404300("invalid diff format; incorrect leading line chars"); /* do not return */
}



// Function: copy_stringlist @ 0x4460
unsigned int copy_stringlist(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long long a4)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long iter;  // rbx
    unsigned long v12;  // rdx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!a4)
        return 1;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    iter = 0;
    while (1)
    {
        v12 = *((long long *)(a1 + iter * 8));
        if (!*((long long *)(a2 + iter * 8)))
        {
            *((long long *)(a2 + iter * 8)) = *((long long *)(a0 + iter * 8));
            *((unsigned long *)(a3 + iter * 8)) = v12;
            iter += 1;
            if (iter == a4)
                return 1;
        }
        else if (*((long long *)(a3 + iter * 8)) != v12)
        {
            return 0;
        }
        else if (!memcmp(*((long long *)(a0 + iter * 8)), *((long long *)(a2 + iter * 8)), v12))
        {
            iter += 1;
            if (iter == a4)
                return 1;
        }
        else
        {
            return 0;
        }
    }
}



// Function: dotlines @ 0x4500
typedef struct struct_0 {
    char padding_0[16];
    long long field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

int dotlines(struct_1 *idx, unsigned long a1, int a2)
{
    unsigned long v3;  // r15
    struct_0 *v4;  // r12
    long long index;  // rbx
    unsigned long long v6;  // rdx
    char *v7;  // rbp
    char *v8;  // rax
    char v0;  // [bp-0x41]
    unsigned long long v1;  // [bp-0x40]

    v3 = a2;
    v4 = v3 * 16 + a1;
    if (v4->field_10 >= *((long long *)&v4->padding_0[8]))
    {
        v0 = 0;
        index = 0;
        do
        {
            v6 = index * 8;
            v7 = *((long long *)(*((long long *)(a1 + v3 * 8 + 56)) + index * 8));
            if (*(v7) == 46)
            {
                v8 = &idx->field_28->field_0;
                if (v8 < *((long long *)&idx[1].padding_0[0]))
                {
                    v0 = 1;
                    idx->field_28 = v8 + 1;
                    *(v8) = 46;
                }
                else
                {
                    v1 = v6;
                    __overflow(idx, 46);
                    v0 = 1;
                    v6 = v1;
                }
            }
        } while ((index = (long long)(index + 1), fwrite_unlocked(v7, 1, *((long long *)(*((long long *)(a1 + v3 * 8 + 80)) + v6)), idx), v4->field_10 - *((long long *)&v4->padding_0[8]) >= index));
    }
    else
    {
        v0 = 0;
    }
    return v0;
}



// Function: process_diff @ 0x46e0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

typedef struct struct_3 {
    char field_0;
    char padding_1[15];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_3;

typedef struct struct_4 {
    struct struct_4 *field_0;
    struct struct_4 *field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
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

typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

extern unsigned long long g_411010;
extern struct_1 *stderr;
extern char g_4110f1;
extern char g_4110f2;

long long process_diff(unsigned long a0, unsigned long a1, struct_4 **a2, struct_0 **a3)
{
    struct_3 *iter;  // rax
    unsigned int v10;  // ebp
    char *v19;  // rbx
    struct_4 *v20;  // rax
    struct_4 *idx;  // r15
    unsigned int v22;  // edx
    char *node;  // rax
    struct_4 *v24;  // rcx
    char v25;  // dl
    char *v26;  // rsi
    unsigned int v27;  // eax
    struct_4 *v28;  // rdx
    char v29;  // dl
    unsigned int v30;  // r12d
    char *iter1;  // rdx
    unsigned int v32;  // eax
    unsigned long long v33;  // rsi
    char v34;  // al
    char v35;  // al
    unsigned int v36;  // eax
    unsigned long long v37;  // rdi
    unsigned long long v38;  // rbp
    unsigned long long v12;  // r13
    unsigned long long v39;  // r13
    unsigned long long v40;  // rax
    unsigned long long v41;  // r13
    unsigned long long v42;  // rbp
    unsigned long long v43;  // r12
    unsigned long long v44;  // rax
    unsigned long long v45;  // r12
    char *v46;  // rax
    char v47;  // al
    char *v48;  // rdx
    unsigned long long v13;  // rbx
    char *v49;  // rsi
    unsigned int *err;  // rax
    char *iter2;  // rax
    long long v15;  // r12
    long long v16;  // rax
    unsigned int v17;  // eax
    char v18;  // al
    struct_4 *v0;  // [bp-0x150], Other Possible Types: unsigned int
    struct_0 **v1;  // [bp-0x148], Other Possible Types: unsigned long long
    int v2[2];  // [bp-0x138]
    char v3;  // [bp-0x130]
    int v4;  // [bp-0x12c]
    unsigned long v5;  // [bp-0x128]
    int v6;  // [bp-0x120], Other Possible Types: char
    stat v7;  // [bp-0xd8]

    v1 = a3;
    v5 = g_411010;
    iter = &v6;
    if (g_4110f2)
    {
        *((char **)&v6) = "-a";
        iter = &v6 - 8;
    }
    if (g_4110f1)
    {
        iter = &iter->padding_1[7];
        *((char **)(&iter->field_0 - 8)) = "--strip-trailing-cr";
    }
    iter->field_10 = a0;
    *((char **)&iter->field_0) = "--horizon-lines=100";
    *((char **)&iter->padding_1[7]) = "--";
    iter->field_18 = a1;
    iter->field_20 = 0;
    if (pipe(&v3))
        sub_404330("pipe"); /* do not return */
    v0 = fork();
    if (!v0)
    {
        close(v3);
        if (v4 != 1)
        {
            dup2(v4, 1);
            close(v4);
        }
        execvp(g_411010, &v5);
        err = __errno_location();
        _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
    }
    else if (v0 != 0xffffffff)
    {
        close(v4);
        v10 = v3;
        if (fstat(v10, &v7))
            sub_404330("fstat"); /* do not return */
        v12 = v7.st_mtime;
        if (v7.st_mtime <= 0)
            v12 = 1;
        v13 = 0;
        iter2 = sub_407350();
        while (1)
        {
            v15 = v12 - v13;
            v16 = sub_407a10(v10, &iter2[v13], v15);
            v13 += v16;
            if (v15 != v16)
                break;
            if (v12 > 0x3ffffffffffffffe)
                sub_4077d0(); /* do not return */
            v12 *= 2;
            iter2 = sub_4073a0(iter2, v12);
        }
        if (v16 == -0x1)
            sub_404330(dcgettext(NULL, "read failed", 5)); /* do not return */
        if (v13 && *(&iter2[v13] - 1) != 10)
            sub_404300("invalid diff format; incomplete last line"); /* do not return */
        if (close(v10))
        {
            sub_404330("close"); /* do not return */
        }
        else if (waitpid(v0, v2, 0) >= 0)
        {
            v17 = *((unsigned int *)v2);
            if ((char)v17 & 127)
                error(2, 0, dcgettext(NULL, "subsidiary program '%s' failed", 5));
            v18 = (int)(v17) >> 8;
            if (!(v18 & 254))
            {
                v19 = &iter2[v13];
                v0 = v2;
                *(v1) = iter2;
                if (iter2 < v19)
                {
                    do
                    {
                        v20 = sub_407350(72);
                        v20->field_28 = 0;
                        idx = v20;
                        idx->field_20 = 0;
                        idx->field_38 = 0;
                        idx->field_30 = 0;
                        v22 = *(iter2);
                        node = iter2;
                        if ((char)v22 != 32)
                            goto LABEL_4048f5;
                        while (1)
                        {
                            v22 = node[1];
                            node += 1;
                            if ((char)v22 == 32)
                                continue;
LABEL_4048f5:
                            if ((char)v22 != 9)
                                break;
                        }
                        if (v22 - 48 > 9)
                            goto LABEL_404999;
                        v24 = NULL;
                        do
                        {
                            node += 1;
                            v24 = v22 - 48 + v24 * 10;
                            v22 = *(node);
                        } while (v22 - 48 <= 9);
                        idx->field_0 = v24;
                        v25 = *(node);
                        if (*(node) != 32)
                            goto LABEL_40494d;
                        while (1)
                        {
                            v25 = node[1];
                            node += 1;
                            if (v25 == 32)
                                continue;
LABEL_40494d:
                            if (v25 != 9)
                                break;
                        }
                        if (v25 != 44)
                        {
                            idx->field_8 = v24;
                        }
                        else
                        {
                            v26 = node + 1;
                            v27 = node[1];
                            if (v27 - 48 > 9)
                                goto LABEL_404999;
                            v28 = NULL;
                            do
                            {
                                v26 += 1;
                                v28 = v27 - 48 + v28 * 10;
                                v27 = *(v26);
                            } while (v27 - 48 <= 9);
                            idx->field_8 = v28;
                            node = v26;
                        }
                        if (*(node) == 9 || *(node) == 32)
                        {
                            while (1)
                            {
                                node += 1;
                                if (node[1] == 32)
                                    continue;
                                if (node[1] != 9)
                                    break;
                            }
                        }
                        switch (v29)
                        {
                        case 99:
                            v30 = 2;
                            break;
                        case 100:
                            v30 = 3;
                            break;
                        case 97:
                            v30 = 1;
                            break;
                        default:
LABEL_404999:
                            v46 = dcgettext(NULL, "%s: diff failed: ", 5);
                            __fprintf_chk(*((unsigned int *)&stderr), 0x1, v46);
                            do
                            {
                                v47 = *(iter2);
                                v48 = &stderr->field_28->field_0;
                                if (v48 < *((long long *)&stderr[1].padding_0[0]))
                                {
                                    stderr->field_28 = v48 + 1;
                                    *(v48) = v47;
                                }
                                else
                                {
                                    __overflow(stderr, v47, v48);
                                }
                            } while ((iter2 += 1, *((char *)((char *)iter2 - 1)) != 10));
                            exit(2); /* do not return */
                        }
                        iter1 = node + 1;
                        v32 = node[1];
                        if ((char)v32 != 32)
                            goto LABEL_404a2c;
                        while (1)
                        {
                            v32 = iter1[1];
                            iter1 += 1;
                            if ((char)v32 == 32)
                                continue;
LABEL_404a2c:
                            if ((char)v32 != 9)
                                break;
                        }
                        if (v32 - 48 > 9)
                            goto LABEL_404999;
                        v33 = 0;
                        do
                        {
                            iter1 += 1;
                            v33 = v32 - 48 + v33 * 10;
                            v32 = *(iter1);
                        } while (v32 - 48 <= 9);
                        idx->field_10 = v33;
                        v34 = *(iter1);
                        if (*(iter1) != 32)
                            goto LABEL_404a7c;
                        while (1)
                        {
                            v34 = iter1[1];
                            iter1 += 1;
                            if (v34 == 32)
                                continue;
LABEL_404a7c:
                            if (v34 != 9)
                                break;
                        }
                        if (v34 != 44)
                        {
                            idx->field_18 = v33;
                            v35 = *(iter1);
                        }
                        else
                        {
                            v36 = iter1[1];
                            if (v36 - 48 > 9)
                                goto LABEL_404999;
                            iter1 += 1;
                            v37 = 0;
                            do
                            {
                                iter1 += 1;
                                v37 = v36 - 48 + v37 * 10;
                                v36 = *(iter1);
                            } while (v36 - 48 <= 9);
                            idx->field_18 = v37;
                            v35 = *(iter1);
                            if (v35 == 9 || v35 == 32)
                            {
                                while (1)
                                {
                                    v35 = iter1[1];
                                    iter1 += 1;
                                    if (v35 == 32)
                                        continue;
                                    if (v35 != 9)
                                        break;
                                }
                            }
                        }
                        iter2 = iter1 + 1;
                        if (v35 != 10)
                        {
                            iter2 = iter1;
                            goto LABEL_404999;
                        }
                        if (v30 != 2)
                        {
                            if (v30 == 3)
                            {
                                idx->field_10 = v33 + 1;
                                goto LABEL_404ab7;
                            }
                            else
                            {
                                idx->field_0 = (char *)&v24->field_0 + 1;
                                if (v30 == 1)
                                    goto LABEL_404b6d;
                            }
                        }
                        v24 = idx->field_0;
LABEL_404ab7:
                        v38 = idx->field_8 - v24;
                        v39 = v38 + 1;
                        if (v39 > 0xffffffffffffffe)
                            sub_4077d0(); /* do not return */
                        v1 = v39 * 8;
                        idx->field_20 = sub_407350(v1);
                        v40 = sub_407350(v1);
                        idx->field_30 = v40;
                        if (v39)
                        {
                            v41 = 0;
                            while (1)
                            {
                                iter2 = &sub_404360(iter2, v41 * 8 + idx->field_20, v40 + v41 * 8, v19, 60)->field_0;
                                if (v38 == v41)
                                    break;
                                v40 = idx->field_30;
                                v41 += 1;
                            }
                        }
                        if (v30 == 2)
                        {
                            if (!strncmp(iter2, "---\n", 4))
                                iter2 += 4;
                            else
                                sub_404300("invalid diff format; invalid change separator"); /* do not return */
                        }
                        else
                        {
                            if (v30 == 3)
                                continue;
                        }
LABEL_404b6d:
                        v42 = idx->field_18 - idx->field_10;
                        v43 = v42 + 1;
                        if (v43 > 0xffffffffffffffe)
                            sub_4077d0(); /* do not return */
                        v1 = v43 * 8;
                        idx->field_28 = sub_407350(v1);
                        v44 = sub_407350(v1);
                        idx->field_38 = v44;
                        if (v43)
                        {
                            v45 = 0;
                            while (1)
                            {
                                iter2 = &sub_404360(iter2, v45 * 8 + idx->field_28, v44 + v45 * 8, v19, 62)->field_0;
                                if (v42 == v45)
                                    break;
                                v44 = idx->field_38;
                                v45 += 1;
                            }
                        }
                    } while ((v0->field_0 = (struct struct_4 *)idx, v0 = idx + 64, v19 > iter2));
                }
                v0->field_0 = NULL;
                *(a2) = idx;
                return v2;
            }
            if (v18 == 126)
                error(2, 0, dcgettext(NULL, "subsidiary program '%s' could not be invoked", 5));
            v49 = "subsidiary program '%s' failed (exit status %d)";
            if (v18 == 127)
                v49 = "subsidiary program '%s' not found";
            error(2, 0, dcgettext(NULL, v49, 5));
        }
        else
        {
            sub_404330("waitpid"); /* do not return */
        }
    }
    else
    {
        sub_404330("fork"); /* do not return */
    }
}



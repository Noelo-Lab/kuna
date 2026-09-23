// Function: main @ 0x2900
typedef struct struct_2 {
    char padding_0[1];
    char field_1;
} struct_2;

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

typedef struct struct_0 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
    char padding_1[7];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern long long g_40962a;
extern char g_409666;
extern char *g_40ca00;
extern option g_40ca60;
extern unsigned long long g_40d010;
extern long long g_40d018;
extern unsigned int g_40d020;
extern void stdout;
extern unsigned int optind;
extern unsigned long long optarg;
extern unsigned long long stderr;
extern char g_40d0b2;
extern unsigned int g_40d0b4;
extern struct_1 g_40d0c0;
extern long long g_40d0c8;
extern unsigned long long g_40d0d0;
extern unsigned long long g_40d0e0;
extern unsigned long long g_40d0e8;
extern unsigned long long g_40d100;
extern unsigned long long g_40d108;
extern unsigned long long g_40d110;
extern unsigned int g_40d118;
extern unsigned int g_40d11c;
extern unsigned int g_40d120;
extern unsigned long long g_40d128;
extern unsigned long long g_40d130;
extern long long g_40d138;
extern unsigned long long g_40d158;
extern unsigned long long g_40d168;
extern char g_40d190;
extern char g_40d198;
extern char g_40d1a0;
extern char g_40d1a8;
extern char g_40d1ac;
extern char g_40d1b0;
extern char g_40d1b8;
extern char g_40d1c0;
extern long long g_40d1c8;
extern char g_40d1e8;
extern char g_40d1f8;
extern void g_40d220;
extern void g_40d224;
extern struct_0 g_40d230;
extern char *g_40d238;

unsigned int main(int a0, long long *a1)
{
    unsigned long v24;  // rbp
    long long v25;  // rdi
    char **node;  // rbx
    char *v38;  // rax
    char *v39;  // rsi
    char *v40;  // rax
    char *v41;  // rax
    long long v43;  // rax
    long long v26;  // rdi
    long long v44;  // rax
    unsigned int v45;  // eax
    unsigned long idx;  // rcx
    unsigned long v47;  // rcx
    char *v48;  // rcx
    unsigned long v49;  // rax
    unsigned long v50;  // rax
    unsigned long long idx1;  // r13
    unsigned int v52;  // edi
    unsigned int v53;  // eax
    long long v27;  // rsi
    unsigned int v54;  // eax
    unsigned long long v55;  // rax
    unsigned int v56;  // eax
    unsigned int v57;  // eax
    unsigned int v58;  // eax
    unsigned long long v59;  // rax
    unsigned long v60;  // rbp
    unsigned long long v61;  // r8
    unsigned long v62;  // rax
    unsigned long v63;  // rdx
    long long v28;  // rdx
    unsigned long v64;  // rax
    unsigned long long v65;  // rax
    unsigned long long v66;  // rbx
    char *v67;  // rax
    unsigned long v68;  // 4125
    unsigned long long *v69;  // r14
    char *v70;  // rbp
    unsigned long v71;  // r12
    unsigned long long v72;  // rax
    unsigned long v73;  // rbx
    long long v29;  // rcx
    unsigned long long v74;  // rax
    unsigned long v75;  // rbx
    unsigned long long index;  // rbx
    unsigned long v77;  // r13
    unsigned long v78;  // r13
    unsigned long v79;  // r15
    unsigned long v80;  // rax
    unsigned long v81;  // rax
    unsigned long v82;  // r12
    unsigned long v83;  // rbx
    long long v30;  // r8
    unsigned long v84;  // rax
    unsigned long v85;  // r13
    unsigned int v86;  // eax
    char v87;  // sil
    char *v88;  // rax
    struct_2 *iter;  // rdi
    struct_2 *iter1;  // r8
    unsigned long long v91;  // rcx
    unsigned long long v92;  // rcx
    unsigned long long *k;  // rdi
    long long v31;  // r9
    char *v94;  // rcx
    unsigned long long v95;  // rax
    unsigned long long v96;  // rax
    unsigned long v98;  // r11
    unsigned long v99;  // r15
    long long v100;  // r15
    unsigned long v101;  // r13
    unsigned long long v102;  // rax
    char *v103;  // rax
    unsigned int v32;  // eax
    char *v104;  // rdi
    unsigned long long v105;  // rbp
    char *ptr;  // rax
    int v107;  // ebp
    unsigned long long v108;  // rax
    char *v109;  // rax
    unsigned long v110;  // rbp
    unsigned long v112;  // rax
    char *v113;  // rdx
    unsigned long v33;  // rax
    char *v114;  // rax
    char *v115;  // rax
    int *err;  // rax
    int *err1;  // rax
    int *err2;  // rax
    int *v119;  // rax
    int *v120;  // rax
    unsigned long v0;  // [bp-0x238]
    char *v1;  // [bp-0x230]
    unsigned long v2;  // [bp-0x228]
    char *v3;  // [bp-0x220]
    char *iter2;  // [bp-0x210], Other Possible Types: char, unsigned long long, long long
    unsigned int v5;  // [bp-0x208], Other Possible Types: unsigned long
    char *v6;  // [bp-0x200], Other Possible Types: unsigned long long
    char *v7;  // [bp-0x1f8]
    unsigned int v8;  // [bp-0x1ec]
    long long v9;  // [bp-0x1e8]
    char v10;  // [bp-0x1e0], Other Possible Types: unsigned long
    char *v11;  // [bp-0x1d8], Other Possible Types: unsigned long
    unsigned int v12;  // [bp-0x1d0], Other Possible Types: unsigned long
    unsigned long long v13;  // [bp-0x1c8]
    char v14;  // [bp-0x1bd]
    unsigned int v15;  // [bp-0x1bc]
    unsigned long v16;  // [bp-0x1b0]
    stat v17;  // [bp-0x1a8]
    stat v18;  // [bp-0x118]
    char v19;  // [bp-0x82]
    char v20;  // [bp-0x7d]
    char v21;  // [bp-0x78]
    char v22;  // [bp-0x58]

    v24 = a0;
    v25 = *(a1);
    g_40d020 = 2;
    sub_404080(v25);
    setlocale(6, &g_409666);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_403e90(0);
    sub_406870(v26, v27, v28, v29, v30, v31);
    iter2 = &v16;
    while (1)
    {
        v32 = getopt_long(v24, a1, "bci:ln:sv", &g_40ca60.name, NULL);
        if (v32 == 0xffffffff)
            break;
        switch (v32)
        {
        case 98: case 99:
            g_40d0b2 = 1;
            break;
        case 105:
            sub_403ae0(0, &optarg, 58);
            v33 = optarg;
            optarg = v33 + 1;
            if (*((char *)v33) == 58)
            {
                sub_403ae0(1, &optarg, 0);
                break;
            }
            else if (g_40d0c8 < *((long long *)&g_40d0c0.field_0))
            {
                g_40d0c8 = *((long long *)&g_40d0c0.field_0);
                break;
            }
        case 108:
            sub_403c60(1);
            break;
        case 110:
            if (!(!sub_406940(optarg, 0, 0, iter2, "kKMGTPEZY0") && v16 >= 0))
            {
                sub_403a80("invalid --bytes value '%s'", optarg); /* do not return */
            }
            else if (g_40d010 < 0 || v16 <= g_40d010)
            {
                g_40d010 = v16;
                break;
            }
        case 115:
            sub_403c60(3);
            break;
        case 118:
            v43 = sub_404520("David MacKenzie");
            v44 = sub_4045b0("Torbjorn Granlund", &g_40962a);
            sub_4061b0(*((long long *)&stdout), "cmp", "GNU diffutils", g_40d018, v44, v43, 0, v29);
            sub_403c90(v26, v27, v28, v29, v30, v31, 0, v29);
            return 0;
        case 128:
            node = &g_40ca00;
            __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n", 5));
            __printf_chk(1, "%s\n", dcgettext(NULL, "Compare two files byte by byte.", 5));
            __printf_chk(1, "\n%s\n\n", dcgettext(NULL, "The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).", 5));
            v38 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
            fputs_unlocked(v38, *((long long *)&stdout));
            v39 = "-b, --print-bytes          print differing bytes";
            do
            {
                node += 1;
                dcgettext(NULL, v39, 5);
                __printf_chk(1, "  %s\n");
                v39 = *(node);
            } while (v39);
            v40 = dcgettext(NULL, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
            v41 = dcgettext(NULL, "If a FILE is '-' or missing, read standard input.", 5);
            __printf_chk(1, "\n%s\n\n%s\n%s\n", dcgettext(NULL, "SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.", 5), v41, v40);
            sub_406290(v26, v27, v28, v29, v30, v31);
            sub_403c90(v26, v27, v28, v29, v30, v31);
            return 0;
        default:
            sub_403a80(0, 0); /* do not return */
        }
    }
    v45 = optind;
    if (v45 == (unsigned int)v24)
        sub_403a80("missing operand after '%s'", *((long long *)((char *)&a1[v24] - 8))); /* do not return */
    idx = v45;
    v47 = a1[idx];
    optind = v45 + 1;
    g_40d230.field_0 = v47;
    v48 = "-";
    if (optind < (unsigned int)v24)
    {
        v48 = a1[1 + idx];
        optind = v45 + 2;
    }
    g_40d238 = v48;
    if ((unsigned int)v24 > optind)
    {
        v49 = a1[optind];
        optind = optind + 1;
        v16 = v49;
        sub_403ae0(0, &v16, 0);
        if (optind < (unsigned int)v24)
        {
            v50 = a1[optind];
            optind = optind + 1;
            v16 = v50;
            sub_403ae0(1, &v16, 0);
            if (optind < (unsigned int)v24)
                sub_403a80("extra operand '%s'", a1[optind]); /* do not return */
        }
    }
    idx1 = 0;
    while (1)
    {
        if (idx1 && *((long long *)&g_40d0c0.field_0) == g_40d0c8 && !strcmp(g_40d230.field_0, g_40d238))
            return 0;
        v52 = strcmp((&g_40d230.field_0)[idx1], "-");
        if (!v52)
        {
            *((unsigned int *)&(&g_40d220)[4 * idx1]) = 0;
        }
        else
        {
            v52 = open((&g_40d230.field_0)[idx1], 0, 0);
            *((unsigned int *)&(&g_40d220)[4 * idx1]) = v52;
            if (v52 < 0)
            {
                if (g_40d0b4 == 3)
                    exit(2); /* do not return */
                v120 = __errno_location();
                error(2, *(v120), "%s");
            }
        }
        if (fstat(v52, &(&g_40d100)[18 * idx1]))
        {
            v120 = __errno_location();
            error(2, *(v120), "%s");
        }
        if (idx1 == 1)
            break;
        idx1 = 1;
    }
    v15 = v53;
    if (g_40d108 != *((long long *)&g_40d198) || g_40d100 != *((long long *)&g_40d190))
    {
        switch (v54)
        {
        case 24576:
            if ((*((int *)&g_40d1a8) & 0xf000) == 0x6000)
                goto LABEL_403531;
LABEL_40323a:
            if (g_40d0b4 == 3)
                goto LABEL_402e10;
            break;
        case 8192:
            if ((*((int *)&g_40d1a8) & 0xf000) != 0x2000)
                goto LABEL_40323a;
LABEL_403531:
            if (g_40d128 == *((long long *)&g_40d1b8))
                goto LABEL_403120;
            goto LABEL_402ddb;
        default:
LABEL_402ddb:
            if (g_40d0b4 == 3)
                goto LABEL_402de8;
            break;
        }
        v56 = fstat(1, &v17);
        if (!(!v56) || !(v57 = (unsigned int)(int)(long long)stat("/dev/null", &v18), !v57))
            goto LABEL_40325e;
        if (v17.st_ino != v18.st_ino || *((unsigned long long *)&v17) != *((unsigned long long *)&v18))
        {
            switch (v58)
            {
            case 24576:
                if (((unsigned short)v18.st_nlink & 0xf000) == 0x6000)
                {
LABEL_403649:
                    if (v17.st_size == v18.st_size)
                        goto LABEL_40365f;
                    goto LABEL_40325e;
                }
                break;
            case 8192:
                if ((v18.st_nlink & 0xf000) == 0x2000)
                    goto LABEL_403649;
                goto LABEL_40325e;
            default:
LABEL_40325e:
                if (g_40d0b4 == 3)
                {
LABEL_402de8:
                    if (((unsigned short)g_40d118 & 0xf000) == 0x8000 && ((short)*((int *)&g_40d1a8) & 0xf000) == 0x8000)
                    {
                        v59 = sub_403b80(0);
                        v60 = g_40d130 - v59;
                        v61 = sub_403b80(1);
                        v62 = 0;
                        if (v60 >= 0)
                            v62 = v60;
                        v63 = v62;
                        v64 = *((long long *)&g_40d1c0) - v61;
                        if (*((long long *)&g_40d1c0) - v61 >> 63 & 1)
                            v64 = 0;
                        if (v64 != v63)
                        {
                            if (g_40d010 >= 0)
                            {
                                if (v63 < v64)
                                    v64 = v63;
                                if (g_40d010 > v64)
                                    exit(1); /* do not return */
                            }
                            else
                            {
                                exit(1); /* do not return */
                            }
                        }
                    }
                }
LABEL_402e10:
                v65 = sub_407070(g_40d138, g_40d1c8, 0x7ffffffffffffff7);
                v66 = v65 + 15;
                g_40d0d0 = v65;
                v67 = sub_4063b0((v66 >> 3) * 16);
                v68 = g_40d0b4;
                v69 = &v67[v66 & 0xfffffffffffffff8];
                g_40d0e0 = v67;
                v70 = v67;
                g_40d0e8 = v69;
                v5 = g_40d010;
                if ((unsigned int)v68 == 1)
                {
                    v71 = 0x7fffffffffffffff;
                    if (g_40d010 >= 0)
                    {
                        v71 = g_40d010;
                        break;
                    }
                    else
                    {
                        break;
                    }
                    if (((unsigned short)g_40d118 & 0xf000) == 0x8000)
                    {
                        v72 = sub_403b80(0);
                        v73 = g_40d130 - v72;
                        if (v73 < v71)
                            v71 = v73;
                    }
                    if (((short)*((int *)&g_40d1a8) & 0xf000) == 0x8000)
                    {
                        v74 = sub_403b80(1);
                        v75 = *((long long *)&g_40d1c0) - v74;
                        if (v75 < v71)
                            v71 = v75;
                    }
                    v8 = 1;
                    while (1)
                    {
                        v71 /= 10;
                        if (!v71)
                            break;
                        v8 += 1;
                    }
                }
            }
        }
        else
        {
LABEL_40365f:
            g_40d0b4 = 2;
            goto LABEL_40325e;
        }
    }
    else
    {
LABEL_403120:
        if (g_40d118 == *((int *)&g_40d1a8) && g_40d110 == *((long long *)&g_40d1a0) && g_40d11c == *((int *)&g_40d1ac) && g_40d120 == *((int *)&g_40d1b0) && g_40d130 == *((long long *)&g_40d1c0) && g_40d158 == *((long long *)&g_40d1e8) && g_40d168 == *((long long *)&g_40d1f8))
        {
            v55 = sub_403b80(0);
            if (v55 == sub_403b80(1))
                return 0;
        }
    }
    index = 0;
    while (1)
    {
        v77 = *((long long *)&(&g_40d0c0.field_0)[8 * index]);
        if (*((long long *)&(&g_40d0c0.field_0)[8 * index]) && sub_403b80((int)index) == 0xffffffffffffffff)
        {
            do
            {
                v78 = v77;
                v79 = g_40d0d0;
                if (v78 <= g_40d0d0)
                    v79 = v78;
                v80 = sub_406fc0();
                if (v79 != v80)
                {
                    if (v80 == 0xffffffffffffffff)
                    {
                        v119 = __errno_location();
                        error(2, *(v119), "%s");
                    }
                    break;
                }
            } while ((v77 = v78 - v79, v78 != v79));
        }
        if (index == 1)
            break;
        index = 1;
    }
    *((unsigned int *)&v6) = 0;
    iter2 = 1;
    v9 = 1;
    v14 = 1;
    while (1)
    {
        v81 = v5;
        v82 = g_40d0d0;
        if (v81 >= 0)
        {
            if (g_40d0d0 > v81)
                v82 = v81;
            v5 = v81 - v82;
        }
        v83 = sub_406fc0(*((int *)&g_40d220), v70, v82);
        if (v83 == 0xffffffffffffffff)
        {
            err1 = __errno_location();
            error(2, *(err1), "%s");
        }
        v84 = sub_406fc0(*((int *)&g_40d224), v69, v82);
        v85 = v84;
        if (v84 == 0xffffffffffffffff)
        {
            err2 = __errno_location();
            error(2, *(err2), "%s");
        }
        if (v83 <= v84)
            v84 = v83;
        v86 = memcmp(v70, v69, v84);
        v87 = !g_40d0b4;
        if (v86)
        {
            v88 = (char *)v69 + v83;
            if (v83 < v85 || (*(v88) = 0x55, v83 == v85))
                v70[v85] = 121;
            iter = &v70[v83 + 1];
            iter1 = (char *)v69 + v85 + 1;
            v91 = (unsigned int)~(v83) & 7;
            v70[v83] = ~(*(v88));
            for (*((char *)v69 + v85) = ~(v70[v85]); v91; iter = &iter->field_1)
            {
                v91 -= 1;
                iter->padding_0[0] = 0;
            }
            for (v92 = (unsigned int)~(v85) & 7; v92; iter1 = &iter1->field_1)
            {
                v92 -= 1;
                iter1->padding_0[0] = 0;
            }
            k = v69;
            for (v94 = v70; *((long long *)&v94) == *(k); k += 1)
            {
                v94 += 8;
            }
            v95 = 0;
            do
            {
                v95 += 1;
            } while (v94[v96] == *((char *)k + v95 - 1));
            v98 = &v94[v96] - v70;
            iter2 += v98;
            v99 = v98;
            if (!v99 || !(iter2 = iter2, v87))
            {
                iter2 = iter2;
                if (v84 > v99)
                {
                    if (g_40d0b4 == 2)
                    {
                        if (v83 != v85)
                            goto LABEL_403380;
                        *((unsigned int *)&v6) = 1;
LABEL_4032a1:
                        v107 = (unsigned int)v6;
                        break;
                    }
                    else if (g_40d0b4 <= 2)
                    {
                        iter2 = iter2;
                        if (!g_40d0b4)
                            goto LABEL_403496;
                        v10 = v83;
                        v100 = iter2;
                        v6 = &v22;
                        v7 = &v20;
                        v11 = v85;
                        do
                        {
                            v101 = *((char *)v69 + v99);
                            iter2 = v70[v99];
                            if (v70[v99] != (char)v101)
                            {
                                v102 = sub_403fe0(v99 + v100, v6);
                                if (!g_40d0b2)
                                {
                                    __printf_chk(1, "%*s %3o %3o\n", v8, (unsigned int)v102);
                                }
                                else
                                {
                                    v12 = iter2;
                                    v13 = v102;
                                    sub_403bd0(v7, v12 & 0xff);
                                    iter2 = &v21;
                                    sub_403bd0(&v21, v101 & 0xff);
                                    v3 = iter2;
                                    v2 = v101;
                                    __printf_chk(1, "%*s %3o %-4s %3o %s\n", v8, (unsigned int)v13, v12);
                                }
                            }
                        } while ((v99 = (unsigned long)(v99 + 1), v84 != v99));
                        v83 = v10;
                        v85 = v11;
                        *((unsigned int *)&v6) = 0xffffffff;
                        iter2 = v84 + v100;
                        goto LABEL_40328b;
                    }
                    else
                    {
                        iter2 = iter2;
                        if (g_40d0b4 == 3)
                            goto LABEL_403380;
                        goto LABEL_40328b;
                    }
                }
            }
        }
        else
        {
            iter2 += v84;
            if (!v84 || !v87)
                goto LABEL_40328b;
            v99 = v84;
        }
        v103 = &v70[v99];
        v11 = v70;
        v104 = v70;
        v12 = v83;
        v7 = v103;
        v10 = *(v7);
        *(v7) = 10;
        v105 = 0;
        while (1)
        {
            ptr = rawmemchr(v104, 10);
            if (v103 == ptr)
                break;
            v105 += 1;
            v104 = ptr + 1;
        }
        v70 = v11;
        v9 += v105;
        *(v7) = v10;
        v83 = v12;
        if (v84 <= v99)
        {
            v14 = *(&v70[v99] - 1) == 10;
        }
        else
        {
LABEL_403496:
            sub_403fe0(iter2, &v21);
            v108 = sub_403fe0(v9, &v22);
            if (!g_40d0b2)
            {
                iter2 = v108;
                v109 = dcgettext(NULL, "%s %s differ: byte %s, line %s\n", 5);
                if (v109 == "%s %s differ: byte %s, line %s\n" && !(char)sub_403f50(5))
                    v109 = "%s %s differ: char %s, line %s\n";
                __printf_chk(1, v109);
                goto LABEL_403380;
            }
            else
            {
                v110 = v70[v99];
                v6 = v108;
                sub_403bd0(&v19, v110 & 0xff);
                v5 = *((char *)v69 + v99);
                iter2 = &v20;
                sub_403bd0(&v20, v5 & 0xff);
                v3 = iter2;
                v2 = v5;
                v1 = &v19;
                v0 = v110;
                __printf_chk(1, dcgettext(NULL, "%s %s differ: byte %s, line %s is %3o %s %3o %s\n", 5));
                goto LABEL_403380;
            }
        }
LABEL_40328b:
        if (v83 == v85)
        {
            if (v83 != g_40d0d0)
                goto LABEL_4032a1;
        }
        else
        {
            if (g_40d0b4 == 3)
            {
LABEL_403380:
                v107 = 1;
                break;
            }
            else if (iter2 != 1)
            {
                sub_403fe0(iter2 - 1, &v21);
                if (!g_40d0b4)
                {
                    v112 = v14;
                    sub_403fe0(v9 - v112, &v22);
                    v113 = (!(char)v112 ? dcgettext(NULL, "cmp: EOF on %s after byte %s, in line %s\n", 5) : dcgettext(NULL, "cmp: EOF on %s after byte %s, line %s\n", 5));
                    __fprintf_chk(stderr, 0x1, v113);
                    goto LABEL_403380;
                }
                else
                {
                    v114 = dcgettext(NULL, "cmp: EOF on %s after byte %s\n", 5);
                    __fprintf_chk(stderr, 0x1, v114);
                    goto LABEL_403380;
                }
            }
            else
            {
                v107 = 1;
                v115 = dcgettext(NULL, "cmp: EOF on %s which is empty\n", 5);
                __fprintf_chk(stderr, 0x1, v115);
                break;
            }
        }
    }
    if (!close(*((int *)&g_40d220)))
    {
        if (!close(*((int *)&g_40d224)))
        {
            if (v107 && g_40d0b4 <= 1)
                sub_403c90();
            exit(v107); /* do not return */
        }
        v15 = 1;
    }
    err = __errno_location();
    error(2, *(err), "%s");
}



// Function: try_help @ 0x3a80
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: specify_ignore_initial @ 0x3ae0
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_40d0c0[4];

void specify_ignore_initial(int a0, struct_0 **a1, char a2)
{
    unsigned int v2;  // eax
    unsigned long idx;  // rax
    long long v0;  // [bp-0x38]

    v2 = sub_406940(*(a1));
    switch (v2)
    {
    case 0:
        if (v0 >= 0)
        {
            idx = a0;
            if (v0 <= g_40d0c0[idx])
                return;
            g_40d0c0[idx] = v0;
            break;
            return;
        }
        else
        {
            sub_403a80("invalid --ignore-initial value '%s'", *(a1)); /* do not return */
        }
    case 2:
        if (*(a1)->field_0 != a2)
            sub_403a80("invalid --ignore-initial value '%s'", *(a1)); /* do not return */
        break;
    default:
        sub_403a80("invalid --ignore-initial value '%s'", *(a1)); /* do not return */
    }
}



// Function: file_position @ 0x3b80
typedef struct struct_0 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
    char padding_1[7];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern unsigned long long g_40d0a0[4];
extern char g_40d0b0;
extern struct_1 g_40d0c0;
extern struct_0 g_40d220;

long long file_position(int a0)
{
    unsigned long idx;  // rbx
    unsigned long v2;  // rax

    idx = a0;
    if (!(&g_40d0b0)[idx])
    {
        (&g_40d0b0)[idx] = 1;
        v2 = lseek(*((int *)((char *)&g_40d220.field_0 + 4 * idx)), *((long long *)&(&g_40d0c0.field_0)[8 * idx]), 1);
        g_40d0a0[idx] = v2;
        return v2;
    }
    return g_40d0a0[idx];
}



// Function: sprintc @ 0x3bd0
long long sprintc(void* a0, unsigned int a1)
{
    unsigned int v1;  // r12d
    void* iter;  // rbp
    unsigned long long *v3;  // rax
    unsigned long v4;  // rax

    v1 = a1;
    iter = a0;
    v3 = __ctype_b_loc();
    v4 = a1;
    if (!(*((char *)(*(v3) + (char)a1 * 2 + 1)) & 64))
    {
        if ((char)a1 >= 0)
        {
            if ((char)v1 > 31)
                goto LABEL_403c20;
LABEL_403bfd:
            *((char *)iter) = 94;
            v4 = v1 + 64;
            iter += 1;
        }
        else
        {
            v1 = a1 - 128;
            iter += 2;
            *((unsigned short *)((char *)iter - 2)) = 11597;
            if ((char)v1 <= 31)
                goto LABEL_403bfd;
LABEL_403c20:
            v4 = v1;
            if ((char)v1 == 127)
            {
                *((char *)iter) = 94;
                v4 = 63;
                iter += 1;
            }
        }
    }
    *((char *)iter) = v4;
    *((char *)&iter[1]) = 0;
    return v4;
}



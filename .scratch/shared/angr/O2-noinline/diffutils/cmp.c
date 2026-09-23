// Function: main @ 0x2940
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

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_4095cc;
extern long long g_409682;
extern option g_40ca40;
extern unsigned long long g_40d010;
extern long long g_40d018;
extern unsigned int g_40d020;
extern long long stdout;
extern unsigned int optind;
extern unsigned long long optarg;
extern char g_40d0b2;
extern unsigned int g_40d0b4;
extern char g_40d0c0;
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
extern int g_40d220;
extern int g_40d224;
extern struct_0 g_40d230;
extern char *g_40d238;

unsigned int main(int a0, long long *a1)
{
    unsigned long v5;  // rbp
    long long v6;  // rdi
    long long v15;  // rax
    long long v16;  // rax
    unsigned int v17;  // eax
    unsigned long index;  // rcx
    unsigned long v19;  // rcx
    char *v20;  // rcx
    unsigned long v21;  // rax
    unsigned long v22;  // rax
    unsigned long long idx;  // rbx
    unsigned int v24;  // edi
    long long v7;  // rdi
    unsigned int v25;  // eax
    unsigned int v26;  // eax
    unsigned long long v27;  // rax
    unsigned int v28;  // eax
    unsigned int v29;  // eax
    unsigned int v30;  // eax
    unsigned long long v31;  // rax
    long long v32;  // r12
    unsigned long long v33;  // r8
    long long v34;  // rax
    long long v8;  // rsi
    long long v35;  // rdx
    long long v36;  // rax
    unsigned long long v37;  // rax
    unsigned long long v38;  // rbx
    unsigned int v39;  // r12d
    int *err;  // rax
    int *err1;  // rax
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    unsigned int v13;  // eax
    unsigned long v14;  // rax
    char *v0;  // [bp-0x180]
    long long v1;  // [bp-0x170], Other Possible Types: unsigned long
    stat v2;  // [bp-0x168]
    stat v3;  // [bp-0xd8]

    v5 = a0;
    v6 = *(a1);
    g_40d020 = 2;
    sub_404180(v6);
    setlocale(6, &g_4095cc);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_403f90(0);
    sub_4064f0(v7, v8, v9, v10, v11, v12);
    v0 = &v1;
    while (1)
    {
        v13 = getopt_long(v5, a1, "bci:ln:sv", &g_40ca40.name, NULL);
        if (v13 == 0xffffffff)
            break;
        switch (v13)
        {
        case 98: case 99:
            g_40d0b2 = 1;
            break;
        case 105:
            sub_4032d0(0, &optarg, 58);
            v14 = optarg;
            optarg = v14 + 1;
            if (*((char *)v14) == 58)
            {
                sub_4032d0(1, &optarg, 0);
                break;
            }
            else if (g_40d0c8 < *((long long *)&g_40d0c0))
            {
                g_40d0c8 = *((long long *)&g_40d0c0);
                break;
            }
        case 108:
            sub_4032a0(1);
            break;
        case 110:
            if (!(!sub_406640(optarg, 0, 0, v0, "kKMGTPEZY0") && v1 >= 0))
            {
                sub_403240("invalid --bytes value '%s'", optarg); /* do not return */
            }
            else if (g_40d010 < 0 || v1 <= g_40d010)
            {
                g_40d010 = v1;
                break;
            }
        case 115:
            sub_4032a0(3);
            break;
        case 118:
            v15 = sub_404490("David MacKenzie");
            v16 = sub_404520("Torbjorn Granlund", &g_409682);
            sub_405f90(stdout, "cmp", "GNU diffutils", g_40d018, v16, v15, 0, v16);
            sub_403d80(v7, v8, v9, v10, v11, v12, 0, v16);
            return 0;
        case 128:
            sub_403370();
            sub_403d80(v7, v8, v9, v10, v11, v12);
            return 0;
        default:
            sub_403240(0, 0); /* do not return */
        }
    }
    v17 = optind;
    if (v17 == (unsigned int)v5)
        sub_403240("missing operand after '%s'", *((long long *)((char *)&a1[v5] - 8))); /* do not return */
    index = v17;
    v19 = a1[index];
    optind = v17 + 1;
    g_40d230.field_0 = v19;
    v20 = "-";
    if (optind < (unsigned int)v5)
    {
        v20 = a1[1 + index];
        optind = v17 + 2;
    }
    g_40d238 = v20;
    if ((unsigned int)v5 > optind)
    {
        v21 = a1[optind];
        optind = optind + 1;
        v1 = v21;
        sub_4032d0(0, &v1, 0);
        if (optind < (unsigned int)v5)
        {
            v22 = a1[optind];
            optind = optind + 1;
            v1 = v22;
            sub_4032d0(1, &v1, 0);
            if (optind < (unsigned int)v5)
                sub_403240("extra operand '%s'", a1[optind]); /* do not return */
        }
    }
    idx = 0;
    while (1)
    {
        *((unsigned int *)&v0) = idx;
        if (idx && *((long long *)&g_40d0c0) == g_40d0c8 && !strcmp(g_40d230.field_0, g_40d238))
            return 0;
        v24 = strcmp((&g_40d230.field_0)[idx], "-");
        if (!v24)
        {
            (&g_40d220)[idx] = 0;
        }
        else
        {
            v25 = open((&g_40d230.field_0)[idx], 0, 0);
            (&g_40d220)[idx] = v25;
            v24 = v25;
            if (v24 < 0)
            {
                if (g_40d0b4 == 3)
                    exit(2); /* do not return */
                err1 = __errno_location();
                error(2, *(err1), "%s");
            }
        }
        if (fstat(v24, &(&g_40d100)[18 * idx]))
        {
            err1 = __errno_location();
            error(2, *(err1), "%s");
        }
        if (idx == 1)
            break;
        idx = 1;
    }
    if (g_40d108 != *((long long *)&g_40d198) || g_40d100 != *((long long *)&g_40d190))
    {
        switch (v26)
        {
        case 24576:
            if ((*((int *)&g_40d1a8) & 0xf000) == 0x6000)
                goto LABEL_402f2c;
LABEL_402e90:
            if (g_40d0b4 == 3)
                goto LABEL_402d0c;
            break;
        case 8192:
            if ((*((int *)&g_40d1a8) & 0xf000) != 0x2000)
                goto LABEL_402e90;
LABEL_402f2c:
            if (g_40d128 == *((long long *)&g_40d1b8))
                goto LABEL_402dd8;
            goto LABEL_402cd7;
        default:
LABEL_402cd7:
            if (g_40d0b4 == 3)
                goto LABEL_402ce4;
            break;
        }
        v28 = fstat(1, &v2);
        if (!(!v28) || !(v29 = (unsigned int)(int)(long long)stat("/dev/null", &v3), !v29))
            goto LABEL_402eb4;
        if (v2.st_ino != v3.st_ino || *((unsigned long long *)&v2) != *((unsigned long long *)&v3))
        {
            switch (v30)
            {
            case 24576:
                if (((unsigned short)v3.st_nlink & 0xf000) == 0x6000)
                {
LABEL_402faa:
                    if (v2.st_size == v3.st_size)
                        goto LABEL_402fbd;
                    goto LABEL_402eb4;
                }
                break;
            case 8192:
                if ((v3.st_nlink & 0xf000) == 0x2000)
                    goto LABEL_402faa;
                goto LABEL_402eb4;
            default:
LABEL_402eb4:
                if (g_40d0b4 == 3)
                {
LABEL_402ce4:
                    if (((unsigned short)g_40d118 & 0xf000) == 0x8000 && ((short)*((int *)&g_40d1a8) & 0xf000) == 0x8000)
                    {
                        v31 = sub_4034c0(0);
                        v32 = g_40d130 - v31;
                        v33 = sub_4034c0(1);
                        v34 = 0;
                        if (v32 >= 0)
                        {
                            v34 = v32;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        v35 = v34;
                        v36 = *((long long *)&g_40d1c0) - v33;
                        if (*((long long *)&g_40d1c0) - v33 >> 63 & 1)
                            v36 = 0;
                        if (v36 != v35)
                        {
                            if (g_40d010 >= 0)
                            {
                                if (v35 < v36)
                                    v36 = v35;
                                if (g_40d010 > v36)
                                    exit(1); /* do not return */
                            }
                            else
                            {
                                exit(1); /* do not return */
                            }
                        }
                    }
                }
LABEL_402d0c:
                v37 = sub_406ac0(g_40d138, g_40d1c8, 0x7ffffffffffffff7);
                v38 = v37 + 15;
                g_40d0d0 = v37;
                g_40d0e0 = sub_406100((v38 >> 3) * 16);
                g_40d0e8 = g_40d0e0 + (v38 & 0xfffffffffffffff8);
                v39 = sub_4035e0(v7, v8, v9, v10, v11, v12);
                if (close(g_40d220))
                {
                    err = __errno_location();
                    error(2, *(err), "%s");
                }
                else if (close(g_40d224))
                {
                    err = __errno_location();
                    error(2, *(err), "%s");
                }
                else if (v39)
                {
                    if (g_40d0b4 > 1)
                        exit(v39); /* do not return */
                    sub_403d80();
                    goto LABEL_402d99__16;
                }
                else
                {
                    exit(v39); /* do not return */
                }
            }
        }
        else
        {
LABEL_402fbd:
            g_40d0b4 = 2;
            goto LABEL_402eb4;
        }
    }
    else
    {
LABEL_402dd8:
        if (g_40d118 == *((int *)&g_40d1a8) && g_40d110 == *((long long *)&g_40d1a0) && g_40d11c == *((int *)&g_40d1ac) && g_40d120 == *((int *)&g_40d1b0) && g_40d130 == *((long long *)&g_40d1c0) && g_40d158 == *((long long *)&g_40d1e8) && g_40d168 == *((long long *)&g_40d1f8))
        {
            v27 = sub_4034c0(0);
            if (v27 == sub_4034c0(1))
                return 0;
        }
    }
LABEL_402d99__16:
    exit(v39); /* do not return */
}



// Function: block_compare @ 0x31f0
void* block_compare(void* a0, unsigned long long iter)
{
    void* node;  // rax

    node = a0;
    if (*((long long *)a0) == *((long long *)iter))
    {
        do
        {
            node += 8;
            iter += 8;
        } while (*((long long *)node) == *((long long *)iter));
        if (*((char *)node) != *((char *)iter))
            return node - a0;
    }
    for (; *((char *)node) == *((char *)iter); iter += 1)
    {
        node += 1;
    }
    return node - a0;
}



// Function: try_help @ 0x3240
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: specify_comparison_type @ 0x32a0
extern unsigned int g_40d0b4;

long long specify_comparison_type(unsigned int a0)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x8]

    v2 = g_40d0b4;
    if ((unsigned int)v2 && (unsigned int)v2 != a0)
    {
        v0 = v2;
        sub_403240("options -l and -s are incompatible", 0); /* do not return */
    }
    g_40d0b4 = a0;
    return v2;
}



// Function: specify_ignore_initial @ 0x32d0
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_40d0c0[4];

void specify_ignore_initial(int a0, struct_0 **a1, char a2)
{
    unsigned int v2;  // eax
    unsigned long idx;  // rax
    long long v0;  // [bp-0x38]

    v2 = sub_406640(*(a1));
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
            sub_403240("invalid --ignore-initial value '%s'", *(a1)); /* do not return */
        }
    case 2:
        if (*(a1)->field_0 != a2)
            sub_403240("invalid --ignore-initial value '%s'", *(a1)); /* do not return */
        break;
    default:
        sub_403240("invalid --ignore-initial value '%s'", *(a1)); /* do not return */
    }
}



// Function: usage @ 0x3370
typedef struct FILE {
} FILE;

extern char *g_40c9e0;
extern FILE *stdout;

int usage(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    char **iter;  // rbx
    char *v9;  // rax
    char *v10;  // rsi
    char *v11;  // rax
    char *v12;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    iter = &g_40c9e0;
    __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n", 5));
    __printf_chk(1, "%s\n", dcgettext(NULL, "Compare two files byte by byte.", 5));
    __printf_chk(1, "\n%s\n\n", dcgettext(NULL, "The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).", 5));
    v9 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v9, stdout);
    v10 = "-b, --print-bytes          print differing bytes";
    do
    {
        iter += 1;
        dcgettext(NULL, v10, 5);
        __printf_chk(1, "  %s\n");
        v10 = *(iter);
    } while (*(iter));
    v11 = dcgettext(NULL, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
    v12 = dcgettext(NULL, "If a FILE is '-' or missing, read standard input.", 5);
    __printf_chk(1, "\n%s\n\n%s\n%s\n", dcgettext(NULL, "SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.", 5), v12, v11);
    fputs_unlocked("\n", stdout);
    __printf_chk(1, dcgettext(NULL, "Report bugs to: %s\n", 5));
    __printf_chk(1, dcgettext(NULL, "%s home page: <%s>\n", 5));
    return __printf_chk(1, dcgettext(NULL, "General help using GNU software: <%s>\n", 5));
}



// Function: file_position @ 0x34c0
typedef struct struct_0 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

typedef struct struct_1 {
    int field_0;
    char padding_4[4];
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
        v2 = lseek(*((int *)((char *)&g_40d220.field_0 + 4 * idx)), *((long long *)&(&g_40d0c0.field_0)[2 * idx]), 1);
        g_40d0a0[idx] = v2;
        return v2;
    }
    return g_40d0a0[idx];
}



// Function: count_newlines @ 0x3510
unsigned long long count_newlines(char *a0, unsigned long a1)
{
    unsigned long long v1;  // r12
    char *v2;  // rbx
    char v3;  // bpl
    char *ptr;  // rax

    v1 = 0;
    v2 = &a0[a1];
    v3 = *(v2);
    *(v2) = 10;
    while (1)
    {
        ptr = rawmemchr(a0, 10);
        if (v2 == ptr)
            break;
        v1 += 1;
        a0 = ptr + 1;
    }
    *(v2) = v3;
    return v1;
}



// Function: sprintc @ 0x3550
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
                goto LABEL_4035a0;
LABEL_40357d:
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
                goto LABEL_40357d;
LABEL_4035a0:
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



// Function: cmp @ 0x35e0
typedef struct struct_0 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

extern void g_40d010;
extern unsigned long long stderr;
extern char g_40d0b2;
extern unsigned int g_40d0b4;
extern struct_0 g_40d0c0;
extern char g_40d0d0;
extern unsigned long long g_40d0e0;
extern void g_40d0e8;
extern unsigned int g_40d118;
extern unsigned long long g_40d130;
extern unsigned int g_40d1a8;
extern unsigned long long g_40d1c0;
extern unsigned int g_40d220;
extern unsigned int g_40d224;
extern unsigned long long g_40d238;

unsigned int cmp(void)
{
    unsigned long v21;  // r12
    unsigned long v22;  // rbp
    unsigned long v31;  // rax
    long long iter;  // rbp
    unsigned long v33;  // r13
    unsigned long v34;  // rax
    unsigned long v35;  // r14
    unsigned long v36;  // rax
    unsigned long v37;  // r15
    char *v38;  // rax
    char *v39;  // rdx
    char v40;  // al
    long long v23;  // rax
    unsigned int v41;  // eax
    unsigned int i;  // eax
    unsigned int v44;  // eax
    unsigned int j;  // eax
    long long v47;  // rdx
    unsigned long v48;  // rcx
    unsigned long v49;  // rcx
    unsigned long v50;  // r15
    unsigned long v24;  // rax
    unsigned long long v51;  // rax
    long long v52;  // r13
    char *v53;  // rax
    unsigned long v54;  // r12
    unsigned long v55;  // rbx
    unsigned long v57;  // rax
    char *v58;  // rdx
    char *v59;  // rax
    char *v60;  // rax
    long long v25;  // rax
    int *err;  // rax
    int *err1;  // rax
    int *err2;  // rax
    unsigned long v26;  // rax
    unsigned long long idx;  // r15
    unsigned long v28;  // rbp
    unsigned long v29;  // rbp
    unsigned long v30;  // r14
    unsigned long v0;  // [bp-0x108]
    char *v1;  // [bp-0x100]
    unsigned long v2;  // [bp-0xf8]
    char *v3;  // [bp-0xf0]
    char *v4;  // [bp-0xe0], Other Possible Types: unsigned long
    char *v5;  // [bp-0xd8], Other Possible Types: long long, unsigned long long
    unsigned int v6;  // [bp-0xcc]
    long long v7;  // [bp-0xc8]
    char *v8;  // [bp-0xc0], Other Possible Types: unsigned long
    char *v9;  // [bp-0xb8]
    unsigned long long v10;  // [bp-0xb0]
    unsigned int v11;  // [bp-0xa8]
    char v12;  // [bp-0xa1]
    unsigned long v13;  // [bp-0xa0]
    unsigned long v14;  // [bp-0x98]
    unsigned long v15;  // [bp-0x90]
    char v16;  // [bp-0x82]
    char v17;  // [bp-0x7d]
    char v18;  // [bp-0x78]
    char v19;  // [bp-0x58]

    v21 = g_40d0e0;
    v4 = *((long long *)&g_40d010);
    if (g_40d0b4 == 1)
    {
        v22 = 0x7fffffffffffffff;
        if (*((long long *)&g_40d010) >= 0)
            v22 = *((long long *)&g_40d010);
        if (((unsigned short)g_40d118 & 0xf000) == 0x8000)
        {
            v23 = sub_4034c0(0);
            v24 = g_40d130 - v23;
            if (v24 < v22)
                v22 = v24;
        }
        if (((unsigned short)g_40d1a8 & 0xf000) == 0x8000)
        {
            v25 = sub_4034c0(1);
            v26 = g_40d1c0 - v25;
            if (v26 < v22)
                v22 = v26;
        }
        v6 = 1;
        while (1)
        {
            v22 /= 10;
            if (!v22)
                break;
            v6 += 1;
        }
    }
    idx = 0;
    while (1)
    {
        v28 = (&g_40d0c0.field_0)[idx];
        if ((&g_40d0c0.field_0)[idx] && sub_4034c0(idx) == 0xffffffffffffffff)
        {
            do
            {
                v29 = v28;
                v30 = *((long long *)&g_40d0d0);
                if (v29 <= *((long long *)&g_40d0d0))
                    v30 = v29;
                v31 = sub_406a10();
                if (v30 != v31)
                {
                    if (v31 == 0xffffffffffffffff)
                    {
                        err2 = __errno_location();
                        error(2, *(err2), "%s");
                    }
                    break;
                }
            } while ((v28 = v29 - v30, v29 != v30));
        }
        if (idx == 1)
            break;
        idx = 1;
    }
    v12 = 1;
    iter = 1;
    *((unsigned int *)&v5) = 0;
    v7 = 1;
    v33 = *((long long *)&g_40d0d0);
    while (1)
    {
        v34 = v4;
        v35 = v33;
        if (v34 >= 0)
        {
            if (v34 <= v33)
                v35 = v34;
            v4 = v34 - v35;
        }
        v33 = sub_406a10(g_40d220, v21, v35);
        if (v33 == 0xffffffffffffffff)
        {
            err1 = __errno_location();
            error(2, *(err1), "%s");
        }
        v36 = sub_406a10(g_40d224, *((long long *)&g_40d0e8), v35);
        v37 = v36;
        if (v36 == 0xffffffffffffffff)
        {
            err = __errno_location();
            error(2, *(err), "%s");
        }
        if (v33 <= v36)
            v36 = v33;
        if (memcmp(v21, *((long long *)&g_40d0e8), v36))
        {
            v38 = *((long long *)&g_40d0e8) + v33;
            v39 = v21 + v37;
            if (v33 < v37 || (*(v38) = 0x55, v33 == v37))
            {
                *(v39) = 121;
                v40 = ~(*(v38));
            }
            else
            {
                v40 = 0xaa;
            }
            *((char *)(v21 + v33)) = v40;
            *((char *)(*((long long *)&g_40d0e8) + v37)) = ~(*(v39));
            v41 = (unsigned int)~(v33) & 7;
            if (v41)
            {
                i = 0;
                do
                {
                    *((char *)(v21 + v33 + 1 + i)) = 0;
                    i += 1;
                } while (i < v41);
            }
            v44 = (unsigned int)~(v37) & 7;
            if (v44)
            {
                j = 0;
                do
                {
                    *((char *)(*((long long *)&g_40d0e8) + v37 + 1 + j)) = 0;
                    j += 1;
                } while (j < v44);
            }
            v47 = iter;
            v48 = sub_4031f0(v21, *((long long *)&g_40d0e8));
            iter += v48;
            if ((!v48 || g_40d0b4) && v36 > v48)
            {
                if (g_40d0b4 == 2)
                {
                    *((unsigned int *)&v5) = 1;
                    if (v33 == v37)
                    {
                        return (unsigned int)v5;
                    }
                    break;
                }
                if (g_40d0b4 <= 2)
                {
                    v49 = v48;
                    if (!g_40d0b4)
                        goto LABEL_403a01;
                    v14 = v33;
                    v5 = v47 + v36;
                    v13 = v21;
                    v15 = v37;
                    while (1)
                    {
                        v50 = *((char *)(*((long long *)&g_40d0e8) + -(v47) + iter));
                        if (*((char *)(v21 - v47 + iter)) == (char)v50)
                        {
LABEL_4037fa:
                            iter += 1;
                            if (v5 == iter)
                                break;
                        }
                        else
                        {
                            v51 = sub_4040e0(iter, &v19);
                            if (!g_40d0b2)
                            {
                                __printf_chk(1, "%*s %3o %3o\n", v6, (unsigned int)v51);
                                goto LABEL_4037fa;
                            }
                            else
                            {
                                v10 = v51;
                                v52 = iter + 1;
                                v11 = *((char *)(v21 - v47 + iter));
                                v9 = &v17;
                                sub_403550(&v17, v11 & 0xff);
                                v8 = &v18;
                                sub_403550(&v18, v50 & 0xff);
                                v3 = v8;
                                v2 = v50;
                                __printf_chk(1, "%*s %3o %-4s %3o %s\n", v6, (unsigned int)v10, v11);
                                iter = v52;
                                if (v5 == iter)
                                    break;
                            }
                        }
                    }
                    *((unsigned int *)&v5) = 0xffffffff;
                    iter = v52;
                    v21 = v13;
                    v33 = v14;
                    v37 = v15;
                    goto LABEL_4038d0;
                }
                else if (g_40d0b4 == 3)
                {
                    break;
                }
                else
                {
                    goto LABEL_4038d0;
                }
            }
        }
        else
        {
            iter += v36;
            if (g_40d0b4 || !v36)
                goto LABEL_4038d0;
            v48 = v36;
        }
        v8 = v48;
        v49 = v8;
        v7 += sub_403510(v21, v48);
        if (v49 >= v36)
        {
            v12 = *((char *)(v21 + v49 - 1)) == 10;
        }
        else
        {
LABEL_403a01:
            v4 = v49;
            sub_4040e0(iter, &v18);
            sub_4040e0(v7, &v19);
            if (!g_40d0b2)
            {
                v53 = dcgettext(NULL, "%s %s differ: byte %s, line %s\n", 5);
                if (v53 != "%s %s differ: byte %s, line %s\n" || (v4 = v53, v53 = v4, (char)(unsigned long long)sub_404050(5)))
                {
                }
                else
                {
                    v53 = "%s %s differ: char %s, line %s\n";
                }
                __printf_chk(1, v53);
                break;
            }
            else
            {
                v54 = *((char *)(v21 + v4));
                v55 = *((char *)(*((long long *)&g_40d0e8) + v4));
                sub_403550(&v16, v54 & 0xff);
                v5 = &v17;
                sub_403550(&v17, v55 & 0xff);
                v4 = g_40d238;
                v3 = v5;
                v2 = v55;
                v1 = &v16;
                v0 = v54;
                __printf_chk(1, dcgettext(NULL, "%s %s differ: byte %s, line %s is %3o %s %3o %s\n", 5));
                break;
            }
        }
LABEL_4038d0:
        if (v33 == v37)
        {
            if (*((long long *)&g_40d0d0) != v33)
                return (unsigned int)v5;
        }
        else
        {
            if (g_40d0b4 != 3)
            {
                if (iter != 1)
                {
                    sub_4040e0(iter - 1, &v18);
                    if (!g_40d0b4)
                    {
                        v57 = v12;
                        sub_4040e0(v7 - v57, &v19);
                        v58 = (!(char)v57 ? dcgettext(NULL, "cmp: EOF on %s after byte %s, in line %s\n", 5) : dcgettext(NULL, "cmp: EOF on %s after byte %s, line %s\n", 5));
                        __fprintf_chk(stderr, 0x1, v58);
                        break;
                    }
                    else
                    {
                        v59 = dcgettext(NULL, "cmp: EOF on %s after byte %s\n", 5);
                        __fprintf_chk(stderr, 0x1, v59);
                        break;
                    }
                }
                else
                {
                    v60 = dcgettext(NULL, "cmp: EOF on %s which is empty\n", 5);
                    __fprintf_chk(stderr, 0x1, v60);
                    return 1;
                }
            }
        }
    }
    return 1;
}



// Function: check_stdout @ 0x3d80
typedef struct FILE {
} FILE;

extern FILE *stdout;

int check_stdout(void)
{
    unsigned int v1;  // eax
    int *err;  // rax

    if (ferror_unlocked(stdout))
    {
        dcgettext(NULL, "write failed", 5);
        error(2, 0, "%s");
    }
    v1 = fclose(stdout);
    if (!v1)
        return v1;
    dcgettext(NULL, "standard output", 5);
    err = __errno_location();
    error(2, *(err), "%s");
}



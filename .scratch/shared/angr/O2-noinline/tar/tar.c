// Function: main @ 0xbb10
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern unsigned int g_466a30[4];
extern char g_467708;
extern unsigned int g_483e08;
extern unsigned long long stdout;
extern FILE *stderr;
extern char g_483f28;
extern unsigned long long g_484830;
extern void* g_484838;
extern unsigned long long g_484a48;
extern char g_484a69;
extern unsigned int g_484ba8;
extern unsigned long long g_484c18;
extern unsigned int g_484c20;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int main(unsigned int a0, long long *a1)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned long v8;  // rax

    sub_40cd20();
    sub_448760(*(a1));
    setlocale(6, &g_467708);
    bindtextdomain("tar", "/usr/local/share/locale");
    textdomain("tar");
    g_483e08 = 2;
    g_484d50 = sub_410820;
    g_484d58 = 0;
    sub_44a590(0, 7);
    sub_43e420(dcgettext(NULL, "stdout", 5));
    if (!sub_44bd80(v2, v3, v4, v5, v6, v7))
    {
        g_484c18 = 10;
        v8 = sub_44c5b0(80);
        g_484830 = 0;
        g_484838 = v8;
        signal(0x11, NULL);
        sub_42e690(a0, a1);
        sub_426c90(v2, v3, v4, v5, v6, v7);
        if (g_484a48)
            sub_40d850();
        if (g_484ba8 <= 9)
        {
            goto *((void *)((long long)(g_466a30[g_484ba8] + (char *)&g_466a30[0])));
        }
        else
        {
            sub_410910();
            if (g_484a69)
                sub_40d150();
            if (g_484c20)
                sub_414030();
            if (g_484a48)
                sub_40d960();
            free(g_484838);
            sub_431b40(v2, v3, v4, v5, v6, v7);
            sub_426cc0(v2, v3, v4, v5, v6, v7);
            if (g_484d58 == 2)
                error(0, 0, dcgettext(NULL, "Exiting with failure status due to previous errors", 5));
            if (*((long long *)&g_483f28) == stdout)
            {
                sub_43e440();
            }
            else if (ferror_unlocked(stderr) || fclose(stderr))
            {
                sub_42f7d0(2);
            }
            return g_484d58;
        }
    }
    else
    {
        if (g_484d50)
            g_484d50();
        dcgettext(NULL, "failed to assert availability of the standard file descriptors", 5);
        error(0, 0, "%s");
        sub_4163d0(v2, v3, v4, v5, v6, v7); /* do not return */
    }
}



// Function: bufmap_locate @ 0xbed0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_484148;

struct_0 * bufmap_locate(unsigned long long a0)
{
    struct_0 *i;  // r8
    struct_0 *v2;  // rax

    for (i = g_484148; i; i = v2)
    {
        v2 = i->field_0;
        if (!v2)
        {
            return i;
        }
        else if (v2->field_8 * 0x200 > a0)
        {
            return i;
        }
    }
    return i;
}



// Function: find_zip_program @ 0xbf10
extern unsigned int g_481020[7];

unsigned int * find_zip_program(unsigned int a0, int *a1)
{
    unsigned long v1;  // rdx
    unsigned int v2;  // eax
    unsigned int *v3;  // rdx
    unsigned int v4;  // ecx
    unsigned int v5;  // eax
    unsigned int *v6;  // rdx

    v1 = *(a1);
    v2 = v1;
    v3 = &g_481020[6 * v1];
    v4 = *(v3);
    v5 = v2;
    if (*(v3))
    {
        do
        {
            v6 = v3;
            v5 = v2 + 1;
            if (a0 == v4)
            {
                *(a1) = v5;
                return &g_481020[6 * v2];
            }
        } while ((v4 = v6[6], v3 = v6 + 24, v2 = v5, v6[6]));
    }
    *(a1) = v5;
    return NULL;
}



// Function: compress_option @ 0xbf70
typedef struct struct_0 {
    char padding_0[16];
    struct struct_0 *field_10;
} struct_0;

struct_0 * compress_option(unsigned int a0)
{
    struct_0 *v2;  // rax
    int v0;  // [bp-0x14]

    v0 = 0;
    v2 = sub_40bf10(a0, &v0);
    if (!v2)
        return v2;
    return v2->field_10;
}



// Function: bufmap_free @ 0xbfc0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    void* field_10;
} struct_0;

extern unsigned long long g_484140;
extern struct_0 *g_484148;

void bufmap_free(struct_0 *a0)
{
    struct_0 *v1;  // rbx
    struct_0 *ptr;  // rbp

    v1 = g_484148;
    if (g_484148 && g_484148 != a0)
    {
        do
        {
            ptr = v1;
            v1 = v1->field_0;
            free(ptr->field_10);
        } while ((free(ptr), v1 && a0 != v1));
    }
    g_484148 = v1;
    if (!v1)
        g_484140 = 0;
    return;
}



// Function: bufmap_reset @ 0xc030
void bufmap_reset(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40c038();
    return;
}



// Function: set_volume_start_time @ 0xc070
extern uint128_t g_484a00;
extern uint128_t g_484a10;

double set_volume_start_time(void)
{
    sub_4411a0(&g_484a10);
    g_484a00 = g_484a10;
    return (unsigned long long)g_484a10;
}



// Function: print_stats @ 0xc0a0
extern unsigned long long g_483f78;

long long print_stats(int a0, char *a1)
{
    unsigned long long v3;  // rbx
    int v4;  // xmm0
    int v5;  // xmm0
    int v6;  // xmm2
    char *v7;  // rax
    int v8;  // xmm1
    unsigned long v0;  // [bp-0x558]
    unsigned long long v1;  // [bp-0x550]

    v3 = 0;
    if (a1 && *(a1))
    {
        v0 = *((unsigned long long *)&v4);
        v5 = (int)v0;
        v3 = __fprintf_chk(a0, 0x1, "%s: ", dcgettext(NULL, a1, 5)) & 0xffffffff;
    }
    if (((CmpF(g_483f78, 0) & 69 | (char)((CmpF(g_483f78, 0) & 69) >> 6)) & 1) == 1 || (v6 = DivV(v5, (uint128_t)g_483f78), (((char)(CmpF(0x43f0000000000000, *((unsigned long long *)(void*)&v6))) & 69 | (char)((CmpF(0x43f0000000000000, *((unsigned long long *)(void*)&v6)) & 69) >> 6)) & 1) == 1))
    {
        v8 = 0x43e0000000000000;
        v7 = "?";
    }
    else
    {
        v1 = 0x43e0000000000000;
        v0 = *((unsigned long long *)&v5);
        v7 = sub_4426b0((CmpF(*((unsigned long long *)&v6), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v6) : 0x8000000000000000 ^ *((unsigned long long *)&SubV(v6, 0x43e0000000000000))));
        v5 = (int)v0;
        v8 = (int)v1;
    }
    v0 = *((unsigned long long *)&v5);
    return __fprintf_chk(a0, 0x1, "%.0f (%s, %s/s)", v0, sub_4426b0((CmpF(*((unsigned long long *)&v5), *((unsigned long long *)&v8)) & 69 & 1 ? *((unsigned long long *)&v5) : 0x8000000000000000 ^ *((unsigned long long *)&SubV(v5, v8)))), v7) + v3;
}



// Function: increase_volume_number @ 0xc240
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481010;
extern unsigned int g_481014;
extern struct_0 *g_484d50;

void increase_volume_number(void)
{
    unsigned int v2;  // 4098
    unsigned long v3;  // cc_ndep
    long long v12;  // r9
    unsigned long long v4;  // 4107
    unsigned long long v5;  // rax
    long long v7;  // rdi
    long long v8;  // rsi
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    unsigned long long v0;  // [bp-0x8]

    v2 = g_481010;
    g_481010 = g_481010 + 1;
    v4 = _ccall(8, 3, (unsigned long long)v2, 1, v3);
    if (!(v4 & 1))
    {
        g_481014 = g_481014 + 1;
        return;
    }
    v0 = v5;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Volume number overflow", 5));
    sub_4163d0(v7, v8, v9, v10, v11, v12); /* do not return */
}



// Function: backspace_output @ 0xc290
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_483f50;
extern void* g_483f58;
extern int g_484a34;
extern char g_484cf0;
extern struct_0 *g_484d50;

void* backspace_output(void)
{
    unsigned int *err;  // rax
    unsigned long v14;  // fs
    int v6;  // edi
    char v7;  // al
    unsigned long v9;  // rbx
    unsigned long long v10;  // rax
    unsigned long v11;  // rbx
    unsigned short v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x24]
    unsigned long v2;  // [bp-0x20]

    v1 = 1;
    v0 = 4;
    if ((g_484a34 <= 0x3fffffff ? (char)(~((int)ioctl(g_484a34, 1074294017)) >> 31) : (char)(~((unsigned int)sub_434080(g_484a34 - 0x40000000, 1074294017, &v0)) >> 31)))
        return v2 - *((long long *)(40 + v14));
    err = __errno_location();
    v6 = g_484a34;
    if (*(err) != 5)
    {
LABEL_40c301:
        v9 = (v6 <= 0x3fffffff ? lseek(v6, 0, 1) : sub_433f90(v6 - 0x40000000, 0, 1));
        v10 = g_483f50 - g_483f58;
        v11 = v9 - v10;
        if (v9 - v10 >> 63 & 1)
            v11 = 0;
        if ((g_484a34 <= 0x3fffffff ? lseek(g_484a34, v11, 0) : sub_433f90(g_484a34 - 0x40000000, v11, 0)) != v11)
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "Cannot backspace archive file; it may be unreadable without -i", 5));
            if (g_483f58 != *((long long *)&g_484cf0))
                return memset(g_483f58, 0, *((long long *)&g_484cf0) - g_483f58);
        }
    }
    else
    {
        v7 = (g_484a34 <= 0x3fffffff ? (char)(~((int)ioctl(g_484a34, 1074294017)) >> 31) : (char)(~((unsigned int)sub_434080(g_484a34 - 0x40000000, 1074294017, &v0)) >> 31));
        v6 = g_484a34;
        if (!v7)
            goto LABEL_40c301;
    }
    return v2 - *((long long *)(40 + v14));
}



// Function: _flush_write @ 0xc490
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_484138;
extern unsigned long long g_4841a0;
extern char g_484a30;
extern unsigned long long g_484a70;
extern char g_484b10;
extern unsigned long long g_484b98;

unsigned long long _flush_write(void)
{
    unsigned long long v1;  // r12
    struct_0 *idx;  // rdi
    unsigned long v3;  // rsi
    unsigned long v4;  // rdx
    unsigned long v5;  // rax
    unsigned long long v6;  // rcx

    sub_4108d0(1);
    if (((char)((CmpF(g_484a70, 0) & 69) >> 2) & 1 || !(CmpF(g_484a70, 0) & 64)) && !(CmpF(g_4841a0, g_484a70) & 1))
    {
        *(__errno_location()) = 28;
        return 0;
    }
    v1 = g_484b98;
    if (!g_484a30)
        v1 = sub_42af00();
    if (!v1)
    {
        return v1;
    }
    else if (!g_484b10)
    {
        return v1;
    }
    else if (!g_484138)
    {
        idx = sub_40bed0(v1);
        if (!idx)
            return v1;
        v3 = idx->field_8;
        v4 = v1 - v3 * 0x200;
        v5 = idx[2].field_0;
        v6 = (v4 >> 9) + idx[2].field_8;
        if (v5 <= v4)
            v4 = v5;
        idx[2].field_8 = v6;
        idx[2].field_0 = v5 - v4;
        if (v5 == v4)
        {
            idx = idx->field_0;
            v3 += v6;
        }
        sub_40c030(idx, -(v3));
        return v1;
    }
    else
    {
        return v1;
    }
}



// Function: change_tape_menu @ 0xc580
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[2];
    char field_2;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern FILE *stderr;
extern struct_1 *g_484828;
extern char g_484a58;
extern unsigned int g_484ba8;
extern struct_1 *g_484d50;

long long change_tape_menu(FILE *a0)
{
    char *v4;  // rax
    struct_0 *ptr;  // r8
    char *v14;  // rax
    char *v15;  // rax
    char *v16;  // rax
    char *v17;  // rsi
    unsigned long long v18;  // rax
    unsigned long v19;  // fs
    char v6;  // al
    char v7;  // al
    struct_0 *iter;  // rdi
    struct_0 *v9;  // rax
    struct_0 *v10;  // rax
    struct_0 *v11;  // rax
    char *v12;  // rax
    char *v13;  // rsi
    struct_0 *v0;  // [bp-0x50]
    unsigned long v1;  // [bp-0x48]
    unsigned long v2;  // [bp-0x40]

    v0 = NULL;
    v1 = 0;
    while (1)
    {
        fputc_unlocked(7, stderr);
        sub_44ac50(g_484828->field_0);
        v4 = dcgettext(NULL, "Prepare volume #%d for %s and hit return: ", 5);
        __fprintf_chk(*((unsigned int *)&stderr), 0x1, v4);
        fflush_unlocked(stderr);
        if (getline(&v0, &v1, a0) > NULL)
        {
            ptr = v0;
            if ((ptr->padding_0[0] & 223) == 89)
            {
                free(ptr);
                return v2 - *((long long *)(40 + v19));
            }
            switch (v6)
            {
            case 10:
                free(ptr);
                return v2 - *((long long *)(40 + v19));
            case 63:
                v14 = dcgettext(NULL, " n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n", 5);
                __fprintf_chk(*((unsigned int *)&stderr), 0x1, v14);
                if (!g_484a58)
                {
                    v15 = dcgettext(NULL, " !             Spawn a subshell\n", 5);
                    __fprintf_chk(*((unsigned int *)&stderr), 0x1, v15);
                }
                v13 = " ?             Print this list\n";
                break;
            case 110:
                v7 = ptr->padding_0[1];
                iter = &ptr->padding_0[1];
                if (v7 == 9 || v7 == 32)
                {
                    while (1)
                    {
                        v7 = iter->padding_0[1];
                        iter = &iter->padding_0[1];
                        if (v7 == 32)
                            continue;
                        if (v7 != 9)
                            break;
                    }
                }
                if (v7 != 10 && v7)
                {
                    v9 = iter;
                    do
                    {
                        v11 = &v9->padding_0[1];
                    } while (v10->padding_0[1] && (v9 = v11, v10->padding_0[1] != 10));
                }
                else
                {
                    v11 = iter;
                }
                v11->padding_0[0] = 0;
                if (iter->padding_0[0])
                {
                    v18 = sub_44c7a0();
                    g_484828->field_0 = v18;
                    free(v0);
                    return v2 - *((long long *)(40 + v19));
                }
                v12 = dcgettext(NULL, "File name not specified. Try again.\n", 5);
                __fprintf_chk(*((unsigned int *)&stderr), 0x1, "%s", v12);
                continue;
            case 33:
                if (!g_484a58)
                {
                    sub_42add0();
                    continue;
                }
                break;
            case 113:
                if (g_484d50)
                    g_484d50();
                v17 = "No new volume; exiting.\n";
                goto LABEL_40c772;
            default:
                v13 = "Invalid input. Type ? for help.\n";
                break;
            }
            v16 = dcgettext(NULL, v13, 5);
            __fprintf_chk(*((unsigned int *)&stderr), 0x1, v16);
        }
        else
        {
            if (g_484d50)
                g_484d50();
            v17 = "EOF where user reply was expected";
            break;
        }
    }
LABEL_40c772:
    error(0, 0, dcgettext(NULL, v17, 5));
    if (g_484ba8 - 5 > 2)
    {
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "WARNING: Archive is incomplete", 5));
    }
    sub_4163d0(); /* do not return */
}



// Function: guess_seekable_archive @ 0xc8d0
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

extern char g_4847f8;
extern unsigned int g_4847fc;
extern int g_484a34;
extern char g_484b10;
extern unsigned long long g_484b70;
extern unsigned int g_484ba8;

void guess_seekable_archive(void)
{
    char v2;  // al
    unsigned int v3;  // eax
    stat v0;  // [bp-0xa8]

    if (g_484ba8 == 4)
        g_4847f8 = 0;
    if (g_4847fc != 0xffffffff)
    {
        v2 = g_4847fc;
    }
    else if (g_484b10 || g_484b70 || (v3 = (unsigned int)(int)(long long)fstat(g_484a34, &v0), (int)(long long)fstat(g_484a34, &v0)))
    {
        v2 = 0;
    }
    else
    {
        v2 = (v0.st_nlink & 0xf000) == 0x8000;
    }
    g_4847f8 = v2;
    return;
}



// Function: init_buffer @ 0xc980
extern unsigned long long g_483f48;
extern unsigned long long g_483f50;
extern unsigned long long g_483f58;
extern int g_484178;
extern unsigned long long g_484180[4];
extern char g_484190;
extern long long g_484b98;
extern int g_484ba0;

long long init_buffer(void)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax
    unsigned long long v3;  // rdx

    v1 = g_484180[g_484178];
    if (!v1)
    {
        v2 = sub_425000(&(&g_484190)[8 * g_484178], g_484b98);
        g_484180[g_484178] = v2;
        v1 = g_484180[g_484178];
    }
    g_483f58 = v1;
    g_483f48 = v1;
    v3 = g_484ba0 * 0x200;
    g_483f50 = v1 + v3;
    return v1 + v3;
}



// Function: archive_is_dev @ 0xca00
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
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484828;
extern int g_484a34;

unsigned long long archive_is_dev(void)
{
    stat v0;  // [bp-0xa8]

    if (fstat(g_484a34, &v0))
    {
        sub_424ec0(g_484828->field_0);
        return 0;
    }
    return _INSERT(v0.st_nlink & 0xb000, 0, (v0.st_nlink & 0xb000) == 0x2000);
}



// Function: check_tty @ 0xca80
typedef struct struct_1 {
    long long field_0;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484838;
extern struct_1 *g_484d50;

int check_tty(unsigned int a0)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    long long v4;  // rdi
    long long v5;  // rsi
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9

    v1 = strcmp(g_484838->field_0, "-");
    if (v1)
        return v1;
    v2 = isatty(a0);
    if (!v2)
        return v2;
    if (g_484d50)
        g_484d50();
    error(0, 0, (!a0 ? dcgettext(NULL, "Refusing to read archive contents from terminal (missing -f option?)", 5) : dcgettext(NULL, "Refusing to write archive contents to terminal (missing -f option?)", 5)));
    sub_4163d0(v4, v5, v6, v7, v8, v9); /* do not return */
}



// Function: add_chunk_header @ 0xcb00
extern int g_481014;
extern unsigned int g_484ba4;

void add_chunk_header(unsigned long long *idx)
{
    unsigned long long v8;  // rcx
    unsigned long long *iter;  // rdi
    unsigned long long v0;  // [bp-0x1d8]
    unsigned long v1;  // [bp-0x1d0]
    unsigned int v2;  // [bp-0x168]
    unsigned int v3;  // [bp-0x164]
    unsigned int v4;  // [bp-0x160]
    unsigned long v5;  // [bp-0x150]
    unsigned long v6;  // [bp-0xc0]

    if (g_484ba4 != 4)
        return;
    v8 = 55;
    for (iter = &v0; v8; iter += 1)
    {
        v8 -= 1;
        *(iter) = 0;
    }
    v2 = 33188;
    v1 = idx[2];
    v0 = idx[2];
    v3 = getuid();
    v4 = getgid();
    v0 = sub_41c080(&v0, "%d/GNUFileParts/%f.%n", g_481014);
    v1 = v0;
    v5 = idx[4];
    v6 = idx[4];
    if (!sub_4132d0(&v0))
        abort(); /* do not return */
    sub_412f00(sub_412da0(0, &v0));
    free(v0);
    return;
}



// Function: mv_begin_write @ 0xcc00
extern char g_484b10;

void mv_begin_write(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    if (g_484b10)
    {
        v2 = v4;
        v1 = v5;
        v0 = v6;
        sub_40cc24();
    }
    return;
}



// Function: buffer_write_global_xheader @ 0xcca0
typedef struct struct_0 {
    struct struct_0 *field_0;
    long long field_8;
    long long field_10;
} struct_0;

extern unsigned long long g_4840e8;
extern struct_0 *g_484388;
extern long long g_484a20;

void buffer_write_global_xheader(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long v13;  // rax
    struct_0 *v5;  // rbx
    struct_0 *v6;  // rbx
    long long v7;  // rdi
    long long v8;  // rsi
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    if (g_484388)
    {
        sub_41a5a0();
        v5 = g_484388;
        if (g_484388)
        {
            do
            {
                v6 = v5;
                sub_41a9b0(v6->field_10, v6->field_8, &g_4840e8);
            } while ((v5 = (struct_0 *)v6->field_0, v6->field_0));
        }
    }
    if (g_4840e8)
    {
        sub_41cc10(&g_4840e8);
        v13 = sub_41c4b0(v7, v8, v9, v10, v11, v12);
        sub_41ccf0(103, v13, g_484a20, &g_4840e8);
        free(v13);
    }
    return;
}



// Function: mv_begin_read @ 0xccb0
void mv_begin_read(unsigned int *a0)
{
    sub_40cc00(*((long long *)&a0), *((long long *)&a0[0x22]), *((long long *)&a0[0x22]));
    return;
}



// Function: mv_end @ 0xccd0
extern char g_484b10;

void mv_end(void)
{
    if (g_484b10)
        sub_40bfc0(NULL);
    return;
}



// Function: mv_size_left @ 0xccf0
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_484148;

struct_0 * mv_size_left(unsigned long a0)
{
    if (g_484148)
        g_484148[4].field_0 = a0;
    return g_484148;
}



// Function: clear_read_error_count @ 0xcd10
extern unsigned int g_484164;

void clear_read_error_count(void)
{
    g_484164 = 0;
    return;
}



// Function: set_start_time @ 0xcd20
extern uint128_t g_484a00;
extern uint128_t g_484a10;
extern uint128_t g_484a20;

double set_start_time(void)
{
    sub_4411a0(&g_484a20);
    g_484a10 = g_484a20;
    g_484a00 = g_484a20;
    return (unsigned long long)g_484a20;
}



// Function: compute_duration @ 0xcd50
extern unsigned long long g_483f78;
extern unsigned long long g_484a00;
extern unsigned long long g_484a08;

void compute_duration(void)
{
    char v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    sub_4411a0(&v0);
    g_483f78 = AddV(AddV(DivV(v1 - g_484a08, 0x41cdcd6500000000), v0 - g_484a00), g_483f78);
    sub_4411a0(&g_484a00);
    return;
}



// Function: first_decompress_program @ 0xcdf0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_483f70;
extern unsigned long long g_484b70;

long long first_decompress_program(unsigned int *a0)
{
    unsigned long v1;  // r10
    unsigned long v2;  // r10
    struct_0 *v3;  // rax
    unsigned long v4;  // r10

    if (g_484b70)
    {
        return v1;
    }
    else if (!g_483f70)
    {
        return v2;
    }
    else
    {
        *(a0) = 0;
        v3 = sub_40bf10(g_483f70, a0);
        if (v3)
            return v3->field_8;
        return v4;
    }
}



// Function: next_decompress_program @ 0xce40
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_483f70;
extern unsigned long long g_484b70;

unsigned long long next_decompress_program(int *a0)
{
    struct_0 *v1;  // rax

    if (g_484b70)
        return 0;
    v1 = sub_40bf10(g_483f70, a0);
    if (!v1)
        return 0;
    return v1->field_8;
}



// Function: format_total_stats @ 0xce80
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    void* field_0;
    char *field_8;
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_483f38;
extern unsigned long long g_4841a0;
extern unsigned long long g_4841a8;
extern unsigned long long g_4842d8;
extern void g_484b98;
extern unsigned int g_484ba8;

unsigned int format_total_stats(FILE *a0, struct_0 *a1, int a2, int a3)
{
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    char *v4;  // rsi
    unsigned long v5;  // r12
    unsigned long long v6;  // rax
    int v7;  // xmm0
    unsigned int v8;  // r12d
    unsigned long v9;  // rax
    unsigned long long v0;  // [bp-0x50]

    if (g_484ba8 > 7)
    {
        if (g_484ba8 != 8)
            abort(); /* do not return */
        goto LABEL_40cff5;
    }
    if (g_484ba8 <= 4)
    {
        if (g_484ba8 == 4)
        {
            v2 = sub_40c0a0(*((unsigned int *)&a0), a1->field_0);
            fputc_unlocked(a2, a0);
            v3 = sub_40c0a0(*((unsigned int *)&a0), a1->field_8);
            fputc_unlocked(a2, a0);
            v4 = &a1->field_10->field_0;
            v5 = v2 + 1 + v3 + 1;
            if (v4 && *(v4))
                v5 += __fprintf_chk(*((unsigned int *)&a0), 0x1, "%s: ", dcgettext(NULL, v4, 5));
            v6 = *((long long *)&g_484b98) * (g_483f38 - g_4842d8);
            v7 = (int)(SubV((v6 < 0 ? AddV(v6 >> 1 | (unsigned int)v6 & 1, v6 >> 1 | (unsigned int)v6 & 1) : v6), AddV(g_4841a8, g_4841a0)));
            v8 = (unsigned int)v5 + __fprintf_chk(*((unsigned int *)&a0), 0x1, "%s", sub_443090((CmpF(*((unsigned long long *)&v7), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v7) : 0x8000000000000000 ^ *((unsigned long long *)&SubV(v7, 0x43e0000000000000)))));
LABEL_40cfbf:
            if (!a3)
                return v8;
        }
        else
        {
            if (!g_484ba8)
                abort(); /* do not return */
LABEL_40cff5:
            v8 = sub_40c0a0(*((unsigned int *)&a0), a1->field_8);
            if (!a3)
                return v8;
        }
        fputc_unlocked(a3, a0);
        return v8 + 1;
    }
    v9 = *((long long *)&g_484b98) * g_483f38;
    v0 = (v9 < 0 ? (unsigned long long)(AddV(v9 >> 1 | (unsigned int)v9 & 1, v9 >> 1 | (unsigned int)v9 & 1)) : v9);
    v8 = sub_40c0a0(*((unsigned int *)&a0), dcgettext(NULL, a1->field_0, 5));
    goto LABEL_40cfbf;
}



// Function: print_total_stats @ 0xd150
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_47f0e0;
extern FILE *stderr;

unsigned int print_total_stats(void)
{
    return sub_40ce80(stderr, &g_47f0e0.field_0, 10, 10);
}



// Function: current_block_ordinal @ 0xd180
extern unsigned long long g_483f48;
extern unsigned long long g_483f58;
extern unsigned long long g_484170;

long long current_block_ordinal(void)
{
    return ((long long)(g_483f48 - g_483f58) >> 9) + g_484170;
}



// Function: reset_eof @ 0xd1a0
extern unsigned int g_483f40;
extern unsigned long long g_483f48;
extern unsigned long long g_483f50;
extern unsigned long long g_483f58;
extern char g_484161;
extern int g_484ba0;

long long reset_eof(void)
{
    unsigned long long v1;  // rdx
    unsigned long v2;  // rax

    if (g_484161)
    {
        g_484161 = 0;
        g_483f40 = 1;
        v1 = g_484ba0 * 0x200;
        g_483f48 = g_483f58;
        v2 = g_483f58 + v1;
        g_483f50 = v2;
    }
    return v2;
}



// Function: set_next_block_after @ 0xd1f0
extern unsigned long long g_483f48;
extern char g_483f50;

long long set_next_block_after(unsigned long a0)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x8]

    v2 = g_483f48;
    if (a0 >= v2)
    {
        v2 = v2 + (a0 - v2 & 0xfffffffffffffe00) + 0x200;
        g_483f48 = v2;
    }
    if (*((long long *)&g_483f50) < v2)
    {
        v0 = v2;
        abort(); /* do not return */
    }
    return v2;
}



// Function: available_space_after @ 0xd2b0
extern unsigned long long g_483f50;

long long available_space_after(unsigned long a0)
{
    return g_483f50 - a0;
}



// Function: xclose @ 0xd2c0
unsigned long long xclose(int a0)
{
    unsigned long long v1;  // rax

    v1 = close(a0);
    if ((unsigned int)v1)
        return sub_432680(dcgettext(NULL, "(pipe)", 5));
    return v1;
}



// Function: archive_read_error @ 0xd300
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_484164;
extern unsigned long long g_484170;
extern struct_0 *g_484828;
extern struct_0 *g_484d50;

long long archive_read_error(void)
{
    unsigned long v1;  // rax
    char *v2;  // rsi
    long long v4;  // rdi
    long long v5;  // rsi
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9

    sub_432820(g_484828->field_0);
    if (g_484170)
    {
        v1 = g_484164;
        g_484164 = (unsigned int)v1 + 1;
        if ((unsigned int)v1 <= 10)
            return v1;
        if (g_484d50)
            g_484d50();
        v2 = "Too many errors, quitting";
    }
    else
    {
        if (g_484d50)
            g_484d50();
        v2 = "At beginning of tape, quitting now";
    }
    error(0, 0, dcgettext(NULL, v2, 5));
    sub_4163d0(v4, v5, v6, v7, v8, v9); /* do not return */
}



// Function: short_read @ 0xd390
void short_read(unsigned long a0)
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
    sub_40d399();
    return;
}



// Function: seek_archive @ 0xd540
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_483f38;
extern unsigned long long g_483f48;
extern unsigned long long g_483f50;
extern unsigned long long g_483f58;
extern unsigned long long g_484170;
extern int g_484a34;
extern char g_484b98;
extern int g_484ba0;
extern struct_0 *g_484d50;

long long seek_archive(unsigned long long a0)
{
    unsigned long long v2;  // rax
    unsigned long long v3;  // rbx
    long long v12;  // rcx
    long long v13;  // r8
    long long v14;  // r9
    long long v4;  // rax
    long long v5;  // rdx
    long long v6;  // rdi
    unsigned long v7;  // rsi
    unsigned long v9;  // r8
    long long v10;  // rcx
    unsigned long long v0;  // [bp-0x8]

    v2 = (g_484ba0 - ((long long)(g_483f48 - g_483f58) >> 9)) * 0x200;
    if (v2 >= a0)
    {
        return 0;
    }
    else if (a0 - v2 >= *((long long *)&g_484b98))
    {
        v0 = v3;
        v4 = sub_40d180();
        v9 = (g_484a34 <= 0x3fffffff ? lseek(g_484a34, (unsigned long long)((v5 CONCAT v6) / v7) * v7, 1) : sub_433f90(g_484a34 - 0x40000000));
        if (v9 < 0)
        {
            return v9;
        }
        else if (!(v9 % *((long long *)&g_484b98)))
        {
            v10 = (long long)(v9) >> 9;
            g_484170 = v10 - g_484ba0;
            g_483f38 = g_483f38 + (v10 - v4) / g_484ba0;
            g_483f48 = g_483f50;
            return v10 - v4;
        }
        else
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "rmtlseek not stopped at a record boundary", 5));
            sub_4163d0(v6, v7, v5, v12, v13, v14); /* do not return */
        }
    }
    else
    {
        return 0;
    }
}



// Function: write_fatal_details @ 0xd670
long long write_fatal_details(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40d676(); /* do not return */
}



// Function: archive_write_error @ 0xd6d0
extern char g_484a69;

long long archive_write_error(unsigned long a0)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned int *err;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    if (g_484a69)
    {
        err = __errno_location();
        sub_40d150();
        *(err) = *(err);
    }
    sub_40d670(); /* do not return */
}



// Function: simple_flush_write @ 0xd720
extern unsigned long long g_483f30;
extern unsigned long long g_4841a0;
extern char g_484b98;

double simple_flush_write(void)
{
    unsigned long long v1;  // rax

    v1 = sub_40c490();
    if (*((long long *)&g_484b98) != v1)
        sub_40d6d0(v1); /* do not return */
    g_483f30 = g_483f30 + 1;
    g_4841a0 = AddV(v1, g_4841a0);
    return g_4841a0;
}



// Function: init_volume_number @ 0xd850
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern int g_481010;
extern void g_484a48;
extern struct_0 *g_484d50;

unsigned int * init_volume_number(void)
{
    FILE *fp;  // rax
    FILE *v2;  // rbp
    long long v11;  // r9
    unsigned int *err;  // rax
    unsigned int *v3;  // rax
    unsigned int *v4;  // rax
    long long v6;  // rdi
    long long v7;  // rsi
    long long v8;  // rdx
    long long v9;  // rcx
    long long v10;  // r8

    fp = fopen(*((long long *)&g_484a48), "r");
    if (fp)
    {
        v2 = fp;
        if (__isoc99_fscanf(fp, "%d", &g_481010) == 1 && g_481010 >= 0)
        {
            if (!ferror_unlocked(v2))
            {
                v3 = fclose(v2);
                if (!(unsigned int)v3)
                    return v3;
            }
            else
            {
                sub_432820(*((long long *)&g_484a48));
                v4 = fclose(v2);
                if (!(unsigned int)v4)
                    return v4;
            }
            return sub_432680(*((long long *)&g_484a48));
        }
        if (g_484d50)
            g_484d50();
        sub_44aa70(*((long long *)&g_484a48));
        error(0, 0, dcgettext(NULL, "%s: contains invalid volume number", 5));
        sub_4163d0(v6, v7, v8, v9, v10, v11); /* do not return */
    }
    else
    {
        err = __errno_location();
        if (*(err) != 2)
            return sub_4327c0(*((long long *)&g_484a48));
        return err;
    }
}



// Function: closeout_volume_number @ 0xd960
typedef struct FILE {
} FILE;

extern int g_481010;
extern void g_484a48;

unsigned long long closeout_volume_number(void)
{
    FILE *fp;  // rax
    FILE *v2;  // rbp
    unsigned long long v3;  // rax
    unsigned long long v4;  // rax

    fp = fopen(*((long long *)&g_484a48), "w");
    if (!fp)
        return sub_4327c0(*((long long *)&g_484a48));
    v2 = fp;
    __fprintf_chk(*((unsigned int *)&fp), 0x1, "%d\n", g_481010);
    if (!ferror_unlocked(v2))
    {
        v3 = fclose(v2);
        if (!(unsigned int)v3)
            return v3;
    }
    else
    {
        sub_432e10(*((long long *)&g_484a48));
        v4 = fclose(v2);
        if (!(unsigned int)v4)
            return v4;
    }
    return sub_432680(*((long long *)&g_484a48));
}



// Function: new_volume @ 0xda00
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481010;
extern unsigned long long stdin;
extern unsigned long long g_483f00;
extern unsigned long long g_483f08;
extern long long g_483f10;
extern long long g_483f18;
extern unsigned long long g_483f48;
extern unsigned long long g_483f58;
extern unsigned int g_483f60;
extern FILE *g_483f68;
extern char g_484160;
extern char g_484280;
extern struct_0 *g_484828;
extern unsigned long long g_484830;
extern unsigned long long g_484838;
extern int g_484a34;
extern char g_484a48;
extern char g_484a50;
extern long long g_484ab0;
extern long long g_484b48;
extern char g_484b84;
extern struct_0 *g_484d50;
extern char g_484d70;
extern unsigned long long g_484d78;

unsigned int new_volume(unsigned int a0)
{
    unsigned long fp;  // rax
    int v2;  // edi
    long long v11;  // rcx
    long long v12;  // r8
    long long v13;  // r9
    unsigned long long *v3;  // rbx
    unsigned long long v4;  // rbp
    unsigned int v5;  // eax
    int v6;  // edx
    long long v8;  // rdi
    long long v9;  // rsi
    long long v10;  // rdx

    if (!g_483f68 && !g_484b48)
    {
        fp = stdin;
        if (!g_484a34)
            fp = fopen("/dev/tty", "r");
        g_483f68 = fp;
    }
    if (g_484280)
        return 0;
    if (g_484a50)
        sub_4118d0();
    sub_423be0(&g_483f18, 0);
    sub_423be0(&g_483f10, 0);
    v2 = g_484a34;
    g_483f00 = 0;
    g_483f08 = 0;
    g_483f48 = g_483f58;
    if (v2 <= 0x3fffffff)
    {
        if (close(v2))
            goto LABEL_40db13;
    }
    else
    {
        if (sub_433d60(v2 - 0x40000000))
        {
LABEL_40db13:
            sub_432680(g_484828->field_0);
        }
    }
    v3 = &g_484828[1].field_0;
    g_484828 = v3;
    if (v3 != g_484838 + g_484830 * 8)
    {
        if (!g_483f60)
            goto LABEL_40dac9;
        else
            goto LABEL_40db51;
    }
    else
    {
        g_484828 = g_484838;
        g_483f60 = 1;
        goto LABEL_40db51;
    }
    while (1)
    {
LABEL_40dac9:
        v4 = *(v3);
        if (!strcmp(v4, "-"))
        {
            g_484160 = 1;
            g_484a34 = 0;
            return 1;
        }
        if (!g_484a50)
        {
            if (a0 == 1)
            {
                if (g_484b84)
                {
                    sub_424710(v4, 1);
                    v3 = &g_484828->field_0;
                    v4 = g_484828->field_0;
                }
                if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v4, 58), v4 = *(v3), !g_484d78 || g_484d78 <= v4 || memchr(v4, 47, g_484d78 - v4)))
                {
                    v6 = creat(v4, 438);
                }
                else
                {
                    v6 = sub_4338d0(v4, 65, 0x40000000, g_484ab0);
                }
LABEL_40dc7c:
                g_484a34 = v6;
LABEL_40dc82:
                if (v6 >= 0)
                    return 1;
                sub_432800(g_484828->field_0);
                if (g_484a50 != 1 && a0 == 1 && g_484b84)
                    sub_4245d0();
LABEL_40db51:
                if (g_484b48)
                {
                    if (*((long long *)&g_484a48))
                        sub_40d960();
                    if (sub_42b8b0(g_484828, g_481010 + 1))
                        break;
                }
                else
                {
                    sub_40c580(g_483f68);
                }
                v3 = &g_484828->field_0;
            }
            else if (a0 != 2)
            {
                if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v4, 58), v4 = *(v3), !g_484d78 || g_484d78 <= v4 || memchr(v4, 47, g_484d78 - v4)))
                    v5 = open(v4, 0, 438);
                else
                    v5 = sub_4338d0(v4, 0, 0x40000000, g_484ab0);
                g_484a34 = v5;
                sub_40c8d0();
                v6 = g_484a34;
                goto LABEL_40dc82;
            }
        }
        else
        {
            if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v4, 58), v4 = *(v3), !g_484d78 || g_484d78 <= v4 || memchr(v4, 47, g_484d78 - v4)))
            {
                v6 = open(v4, 66, 438);
                goto LABEL_40dc7c;
            }
            else
            {
                v6 = sub_4338d0(v4, 66, 0x40000000, g_484ab0);
                goto LABEL_40dc7c;
            }
        }
    }
    if (g_484d50)
        g_484d50();
    sub_44ac50(g_484b48);
    error(0, 0, dcgettext(NULL, "%s command failed", 5));
    sub_4163d0(v8, v9, v10, v11, v12, v13); /* do not return */
}



// Function: drop_volume_label_suffix @ 0xde80
void* drop_volume_label_suffix(void* ptr)
{
    unsigned long len;  // rax
    void* v2;  // rbx
    unsigned long long *v3;  // rax
    void* v4;  // rbx
    unsigned long n;  // rbx
    void* v6;  // rax

    len = strlen(ptr);
    if (!len)
    {
        return NULL;
    }
    else if (ptr < ptr + len - 1)
    {
        v3 = __ctype_b_loc();
        while (*((char *)(*(v3) + *((char *)v2) * 2 + 1)) & 8)
        {
            if (ptr == v2 - 1)
                return NULL;
        }
        if (ptr >= v2)
            return NULL;
        v4 = v2 - 7;
        if (v4 <= ptr)
        {
            return NULL;
        }
        else if (memcmp(v4, " Volume ", 8))
        {
            return NULL;
        }
        else
        {
            n = v4 - ptr;
            v6 = sub_44c5b0(n + 1);
            memcpy(v6, ptr, n);
            *((char *)v6 + n) = 0;
            return v6;
        }
    }
    else
    {
        return NULL;
    }
}



// Function: check_label_pattern @ 0xdf40
extern char *g_484a40;
extern char g_484b10;

unsigned int check_label_pattern(void* a0)
{
    void* v2;  // rax
    char v0;  // [bp-0x9]

    if (!fnmatch(g_484a40, a0, 0))
        return 1;
    if (!g_484b10)
        return 0;
    v2 = sub_40de80(a0);
    if (v2)
    {
        v0 = !fnmatch(v2, g_484a40, 0);
        free(v2);
        return v0;
    }
    return 0;
}



// Function: flush_archive @ 0xdfd0
extern unsigned int g_483f40;
extern unsigned long long g_483f48;
extern void* g_483f50;
extern unsigned long long g_483f58;
extern unsigned long long g_484158;
extern unsigned long long g_484170;
extern int g_484ba0;
extern char g_484cf8;

long long flush_archive(void)
{
    unsigned long v1;  // rax
    long long v2;  // rdx
    unsigned long v3;  // rsi
    unsigned long v4;  // rdx
    long long v5;  // rax

    v1 = g_483f40;
    if (!(unsigned int)v1)
    {
        if (!g_484cf8)
        {
            v2 = g_483f50 - g_483f58;
            g_483f48 = g_483f58;
            g_484170 = g_484170 + (v2 >> 9);
            g_483f50 = g_483f58 + g_484ba0 * 0x200;
        }
        else
        {
            g_483f40 = 1;
            g_484cf8 = 0;
            sub_40c290();
            v3 = g_483f58;
            v4 = g_484ba0;
            v5 = g_483f50 - g_483f58 >> 9;
            if (v5 < g_484ba0)
            {
                memset(g_483f50, 0, (g_484ba0 - v5) * 0x200);
                g_483f50 = g_484ba0 * 0x200 + g_483f58;
                return g_483f50;
            }
            v1 = g_483f40;
LABEL_40e04c:
            g_484170 = g_484170 + v5;
            g_483f48 = v3;
            g_483f50 = v4 * 0x200 + v3;
            if ((unsigned int)v1 == 1)
            {
                goto *((void *)(g_484158));
            }
            else if ((unsigned int)v1 == 2)
            {
                abort(); /* do not return */
            }
            else if ((unsigned int)v1)
            {
                return v1;
            }
        }
        return (unsigned long long)sub_40dfc0();
    }
    v3 = g_483f58;
    v4 = g_484ba0;
    v5 = g_483f50 - g_483f58 >> 9;
    goto LABEL_40e04c;
}



// Function: find_next_block @ 0xe130
extern unsigned long long g_483f48;
extern char g_483f50;
extern char g_484161;

unsigned long long find_next_block(void)
{
    if (g_483f48 != *((long long *)&g_483f50))
    {
        return g_483f48;
    }
    else if (g_484161)
    {
        return 0;
    }
    else
    {
        sub_40dfd0();
        if (g_483f48 == *((long long *)&g_483f50))
        {
            g_484161 = 1;
            return 0;
        }
        return g_483f48;
    }
}



// Function: _write_volume_label @ 0xe190
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[148];
    char field_9c;
    char padding_9d[347];
    unsigned long long field_1f8;
} struct_0;

extern long long g_483f80;
extern long long g_484840;
extern long long g_484848;
extern char g_484850;
extern long long g_484a20;
extern unsigned int g_484ba4;

long long _write_volume_label(long long a0)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // rbp
    unsigned long long *iter;  // rdi
    unsigned long long v4;  // rcx
    char v5;  // al

    if (g_484ba4 == 4)
        return sub_41ca30("GNU.volume.label", &g_483f80, a0);
    v1 = sub_40e130();
    v2 = v1;
    v1->field_0 = 0;
    v1->field_1f8 = 0;
    iter = v1->padding_8 & 0xfffffffffffffff8;
    for (v4 = (unsigned int)(v1 - iter) + 0x200 >> 3; v4; iter += 1)
    {
        v4 -= 1;
        *(iter) = 0;
    }
    __strcpy_chk(v2);
    sub_423be0(&g_484848, v2);
    v5 = sub_43e630(g_484848);
    v2->field_9c = 86;
    g_484850 = v5;
    sub_412c10(g_484a20, &v2->padding_8[128], 12);
    sub_412f60(&g_484840, v2, -0x1);
    return sub_40d1f0(v2);
}



// Function: add_volume_label @ 0xe280
extern int g_481014;
extern char *g_484a40;

void add_volume_label(void)
{
    char *v3;  // rbx
    unsigned long len;  // rax
    unsigned long v5;  // rbp
    char *v0;  // [bp-0x48]
    char v1;  // [bp-0x38]

    v3 = sub_443090(g_481014, &v1);
    len = strlen(g_484a40);
    v0 = v3;
    v5 = sub_44c5b0(len + strlen(v3) + 9);
    __sprintf_chk(v5, 0x1);
    sub_40e190(v5);
    free(v5);
    return;
}



// Function: write_volume_label @ 0xe330
extern long long g_484a40;
extern char g_484b10;

long long write_volume_label(void)
{
    if (!g_484b10)
        return sub_40e190(g_484a40);
    return (unsigned long long)sub_40e280();
}



// Function: add_multi_volume_header @ 0xe4d0
typedef struct struct_0 {
    char padding_0[16];
    void* field_10;
    unsigned long long field_18;
    long long field_20;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern long long g_483f80;
extern long long g_484840;
extern unsigned int g_484a54;
extern unsigned int g_484ba4;
extern struct_1 *g_484d50;

long long add_multi_volume_header(struct_0 *idx)
{
    unsigned long v3;  // fs
    void* v4;  // rax
    void* v13;  // rsi
    unsigned long long *iter;  // rdi
    void* v15;  // rcx
    unsigned long long *j;  // rsi
    unsigned long long v17;  // rcx
    unsigned int v18;  // ebx
    unsigned long v19;  // fs
    void* v5;  // r12
    void* index;  // rbp
    unsigned long len;  // rax
    void* v9;  // r8
    unsigned long long *node;  // rdi
    unsigned long long v11;  // rcx
    unsigned long long v12;  // rcx
    unsigned long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    if (g_484ba4 == 4)
    {
        v0 = idx->field_18 - idx->field_20;
        sub_41ca30("GNU.volume.filename", &g_483f80, idx->field_10);
        sub_41ca30("GNU.volume.size", &g_483f80, &idx->field_20);
        sub_41ca30("GNU.volume.offset", &g_483f80, &v0);
        return v1 - *((long long *)(40 + v19));
    }
    else if (v1 == *((long long *)(40 + v3)))
    {
        v4 = sub_40e130();
        v5 = idx->field_10;
        index = v4;
        len = strlen(idx->field_10);
        if (len > 100)
        {
            if (g_484d50)
            {
                g_484d50();
                v5 = idx->field_10;
            }
            sub_44aa70(v5);
            error(0, 0, dcgettext(NULL, "%s: file name too long to be stored in a GNU multivolume header, truncated", 5));
            len = 100;
        }
        v9 = index + 8;
        *((unsigned long *)index) = 0;
        *((unsigned long *)&index[504]) = 0;
        node = v9 & 0xfffffffffffffff8;
        for (v11 = (unsigned int)(index - node) + 0x200 >> 3; v11; node += 1)
        {
            v11 -= 1;
            *(node) = 0;
        }
        v12 = len & 0xffffffff;
        v13 = idx->field_10;
        if ((unsigned int)len >= 8)
        {
            iter = v9 & 0xfffffffffffffff8;
            *((long long *)index) = *((long long *)v13);
            *((long long *)((char *)index + (len & 0xffffffff) - 8)) = *((long long *)((char *)v13 + (len & 0xffffffff) - 8));
            v15 = index - iter;
            j = v13 - v15;
            for (v17 = (v15 + len & 0xffffffff) >> 3 & 0xffffffff; v17; j += 1)
            {
                v17 -= 1;
                *(iter) = *(j);
                iter += 1;
            }
        }
        else if ((char)len & 4)
        {
            *((int *)index) = *((int *)v13);
            *((int *)((char *)index + v12 - 4)) = *((int *)((char *)v13 + v12 - 4));
        }
        else if ((unsigned int)v12)
        {
            *((char *)index) = *((char *)v13);
            if ((char)v12 & 2)
                *((short *)((char *)index + v12 - 2)) = *((short *)((char *)v13 + v12 - 2));
        }
        *((char *)&index[156]) = 77;
        sub_412be0(idx->field_20, index + 124, 12);
        sub_412be0(idx->field_18 - idx->field_20, index + 369, 12);
        v18 = g_484a54;
        g_484a54 = 0;
        sub_412f60(&g_484840, index, -0x1);
        g_484a54 = v18;
        return sub_40d1f0(index);
    }
    else
    {
        __stack_chk_fail(); /* do not return */
    }
}



// Function: _gnu_flush_write @ 0xe580
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_483f30;
extern unsigned long long g_483f58;
extern long long g_483f80;
extern unsigned int g_484138;
extern unsigned int g_484178;
extern unsigned long long g_4841a0;
extern unsigned long long g_4841a8;
extern unsigned long long g_484a40;
extern char g_484b10;
extern char g_484b98;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

long long _gnu_flush_write(unsigned long a0)
{
    unsigned long v1;  // r12
    unsigned long v2;  // xmm0lq
    unsigned long ptr;  // r13
    unsigned long v14;  // rax
    unsigned long n;  // rbx
    void* v16;  // r12
    unsigned long long *v3;  // rax
    unsigned int v4;  // eax
    unsigned long v6;  // rax
    unsigned long n1;  // rbp
    unsigned long v8;  // xmm0lq
    void* v9;  // r12
    unsigned long long v10;  // 4098

    v1 = sub_40c490();
    if (*((long long *)&g_484b98) != v1)
    {
        if (g_484b10)
        {
            v2 = AddV(v1, g_4841a0);
            if (!v1)
                goto LABEL_40e62a;
            g_483f30 = g_483f30 + 1;
            g_4841a0 = v2;
            v3 = sub_40bed0(v1);
            if ((unsigned short)v1 & 0x1ff)
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "write did not end on a block boundary", 5));
                g_484d58 = 2;
            }
            else if (!(v1 < 0 && (v4 = (unsigned int)*(__errno_location()), *(__errno_location()) - 5 > 1 && *(__errno_location()) != 28)))
            {
                goto LABEL_40e647;
            }
        }
        sub_40d6d0(v1); /* do not return */
    }
    else
    {
        v2 = AddV(*((long long *)&g_484b98), g_4841a0);
        if (*((long long *)&g_484b98))
        {
            g_483f30 = g_483f30 + 1;
            g_4841a0 = v2;
            return *((long long *)&g_484b98);
        }
LABEL_40e62a:
        g_4841a0 = v2;
        if (!*((long long *)&g_484b98))
            return *((long long *)&g_484b98);
        v1 = 0;
        v3 = sub_40bed0(0);
LABEL_40e647:
        v6 = sub_40da00(1);
        if (!(char)v6)
            return v6;
        n1 = a0 - v1;
        sub_42f6f0(&g_483f80);
        sub_40c240();
        v8 = AddV(g_4841a8, g_4841a0);
        g_4841a0 = 0;
        v9 = g_483f58 + v1;
        g_484178 = !g_484178;
        g_4841a8 = v8;
        sub_40c980();
        v10 = g_484a40;
        g_484138 = 1;
        if (v10)
            sub_40e280();
        if (v3)
        {
            sub_40e4d0(v3);
            sub_412da0(1, &g_483f80, sub_40e130());
            sub_42f6f0(&g_483f80);
            sub_40cb00(v3);
        }
        else
        {
            sub_412da0(1, &g_483f80, sub_40e130());
            sub_42f6f0(&g_483f80);
        }
        ptr = sub_40e130();
        sub_40c030(v3, (long long)(ptr - g_483f58) >> 9);
        v14 = sub_40d2b0(ptr);
        g_484138 = 0;
        n = v14;
        v16 = v9;
        if (n1 > n)
        {
            do
            {
                v16 = v9 + n;
                memcpy(ptr, v9, n);
                n1 -= n;
                sub_40d1f0(ptr + (n - 1 & 0xfffffffffffffe00));
                ptr = sub_40e130();
                n = sub_40d2b0(ptr);
                v9 = v16;
            } while (n1 > n);
        }
        memcpy(ptr, v16, n1);
        memset(ptr + n1, 0, n - n1);
        sub_40d1f0(ptr + (n1 - 1 & 0xfffffffffffffe00));
        return sub_40e130();
    }
}



// Function: gnu_flush_write @ 0xe850
extern unsigned long long g_484158;

int gnu_flush_write(unsigned int a0)
{
    g_484158 = sub_40d720;
    sub_40e580(a0);
    g_484158 = gnu_flush_write;
    return gnu_flush_write;
}



// Function: try_new_volume @ 0xe880
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

extern unsigned long long g_483f00;
extern long long g_483f08;
extern long long g_483f10;
extern long long g_483f18;
extern long long g_483f58;
extern long long g_483f80;
extern struct_2 *g_484148;
extern long long g_484408;
extern void g_484a34;
extern char g_484b98;
extern unsigned int g_484ba4;
extern unsigned int g_484ba8;
extern struct_3 *g_484d50;

unsigned int try_new_volume(void)
{
    unsigned int v5;  // eax
    long long v6;  // rax
    unsigned long long v15;  // rax
    struct_0 *v7;  // rax
    char v8;  // al
    unsigned int v9;  // eax
    struct_2 *v10;  // rbx
    unsigned long v11;  // r13
    unsigned long v13;  // rdi
    unsigned long long v14;  // 4137
    unsigned long long v0;  // [bp-0xa8]
    struct_0 *v1;  // [bp-0x90]
    char v2;  // [bp-0x88]
    char v3;  // [bp-0x68]

    if (g_484ba8 <= 2)
    {
        v5 = sub_40da00((g_484ba8) * 2);
        if (!(char)v5)
            return 1;
    }
    else
    {
        v5 = sub_40da00((g_484ba8 == 8) * 2);
        if (!(char)v5)
            return 1;
    }
    while (1)
    {
        if (*((int *)&g_484a34) > 0x3fffffff)
        {
            if (sub_433da0(*((int *)&g_484a34) - 0x40000000) != -0x1)
                break;
        }
        else
        {
            if (sub_44af10(*((int *)&g_484a34), g_483f58, *((long long *)&g_484b98)) != -0x1)
                break;
        }
        sub_40d300();
    }
    if (*((long long *)&g_484b98) == v6)
    {
        v7 = sub_40e130();
        v1 = v7;
        if (v7)
            goto LABEL_40e925;
        goto LABEL_40eb23;
    }
    else
    {
        sub_40d390(v6);
        v7 = sub_40e130();
        v1 = v7;
        if (!v7)
            goto LABEL_40eb23;
LABEL_40e925:
        switch (v8)
        {
        case 86:
            if ((char)sub_40d230())
            {
                sub_42f6f0(&g_483f80);
                sub_423c20(&g_483f18, g_484408, 100);
                sub_40d1f0(v1);
                v1 = sub_40e130();
                if (v1->field_9c == 77)
                    goto LABEL_40ebb8;
                goto LABEL_40e944;
            }
            break;
        case 103:
            sub_42f630(&g_483f80);
            if (sub_4219e0(&v1, &g_483f80, 2) != 2)
                goto LABEL_40eb23;
            sub_41c8c0(&g_483f80);
            sub_42f6f0(&g_483f80);
            sub_4219e0(&v1, &g_483f80, 0);
            switch (v9)
            {
            case 1:
                sub_40d1f0(v1);
                break;
            case 5:
LABEL_40e944:
                v10 = g_484148;
                if (!g_484148)
                    break;
LABEL_40e954:
                if (g_483f10)
                {
                    v11 = g_484148->field_10;
                    if (strcmp(g_483f10, v11))
                    {
                        if ((g_484ba4 & 0xfffffffb) == 2 && strlen(v11) > 99 && !strncmp(g_483f10, v11, 100))
                        {
                            if (g_484d50)
                            {
                                g_484d50();
                                v11 = g_484148->field_10;
                            }
                            sub_44ac50(v11);
                            error(0, 0, dcgettext(NULL, "%s is possibly continued on this volume: header contains truncated name", 5));
                            v10 = g_484148;
                        }
                        else
                        {
                            if (g_484d50)
                            {
                                g_484d50();
                                v11 = g_484148->field_10;
                            }
                            goto LABEL_40ecad;
                        }
                    }
                    v14 = __CFADD__(g_483f00, g_483f08);
                    if (v14 & 1 || g_483f00 + g_483f08 != v10->field_18)
                    {
                        if (g_484d50)
                            g_484d50();
                        v15 = sub_443090();
                        sub_443090(g_483f08, &v3);
                        sub_443090(g_484148->field_18, &v2);
                        sub_44ac50(g_483f10);
                        v0 = v15;
                        error(0, 0, dcgettext(NULL, "%s is the wrong size (%s != %s + %s)", 5));
                        return 0;
                    }
                    else if (v10->field_18 - v10->field_20 != g_483f00)
                    {
                        if (g_484d50)
                            g_484d50();
                        sub_443090();
                        sub_443090(g_484148->field_20, &v3);
                        sub_443090(g_484148->field_18, &v2);
                        error(0, 0, dcgettext(NULL, "This volume is out of sequence (%s - %s != %s)", 5));
                        return 0;
                    }
                    sub_40c240();
                    return v5;
                }
                if (g_484d50)
                {
                    g_484d50();
                    v10 = g_484148;
                }
                v13 = v10->field_10;
LABEL_40ecad:
                sub_44ac50(v13);
                error(0, 0, dcgettext(NULL, "%s is not continued on this volume", 5));
            default:
LABEL_40eb23:
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "This does not look like a tar archive", 5));
                return 0;
            }
        case 77:
LABEL_40ebb8:
            if ((char)sub_40d230())
            {
                sub_42f6f0(&g_483f80);
                sub_423c20(&g_483f10, g_484408, 100);
                g_483f08 = sub_422050(g_484408 + 124, 12);
                g_483f00 = sub_422050(g_484408 + 369, 12);
                goto LABEL_40e944;
            }
            break;
        default:
LABEL_40e944:
            v10 = g_484148;
            if (g_484148)
                goto LABEL_40e954;
            break;
        }
        return 0;
    }
}



// Function: check_compressed_archive @ 0xedb0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char field_8;
    char padding_9[7];
    void* field_10;
} struct_0;

extern void g_460127;
extern struct_0 g_47f130;
extern struct_0 g_47f1f0;
extern void* g_483f50;
extern unsigned long long g_483f58;
extern char g_484160;

unsigned int check_compressed_archive(char *a0)
{
    char *v2;  // rbx
    char v3;  // bpl
    unsigned long long v4;  // rax
    void* v5;  // rbp
    unsigned long v6;  // rbp
    unsigned int v7;  // eax
    struct_0 *iter;  // rbx
    void* i;  // rsi
    unsigned long v10;  // rdx
    char v0;  // [bp-0x21]

    v2 = a0;
    v3 = g_484160;
    g_484160 = 1;
    if (!a0)
        v2 = &v0;
    g_483f50 = g_483f58;
    v4 = sub_40e130();
    g_484160 = v3;
    v5 = g_483f50;
    *(v2) = !v4;
    if (g_483f58 != g_483f50)
    {
        v6 = g_483f58 + 0x101;
        if (strcmp(v6, "ustar") && strcmp(v6, "ustar  "))
        {
            v5 = g_483f58;
        }
        else
        {
            v7 = sub_4214d0(g_483f58, 1);
            if (v7 == 1)
                return v7;
            v5 = g_483f58;
        }
    }
    iter = &g_47f130.field_0;
    i = &g_460127;
    for (v10 = 2; memcmp(v5, i, v10); i = iter->field_10)
    {
        iter += 1;
        if (iter == &g_47f1f0.field_0)
            return 0;
        v10 = *((long long *)&iter->field_8);
    }
    return iter->field_0;
}



// Function: open_compressed_archive @ 0xeed0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_483f38;
extern unsigned long long g_483f50;
extern unsigned long long g_483f58;
extern unsigned int g_483f70;
extern char g_484160;
extern char g_484161;
extern unsigned int g_484168;
extern struct_0 *g_484838;
extern unsigned int g_484a34;
extern long long g_484ab0;
extern char g_484b10;
extern unsigned long long g_484b70;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;
extern char g_484d70;
extern unsigned long long g_484d78;

int open_compressed_archive(void)
{
    unsigned long long v3;  // rdi
    unsigned long long v4;  // rdi
    long long v13;  // r8
    long long v14;  // r9
    unsigned int v15;  // eax
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    long long v12;  // rcx
    unsigned long long v0;  // [bp-0x20]
    char v1;  // [bp-0x11]

    v3 = g_484838->field_0;
    if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v3, 58), v4 = g_484838->field_0, v3 = v4, !g_484d78 || (v3 = v4, g_484d78 <= v3 || (v0 = v4, v3 = v0, memchr(v4, 47, g_484d78 - v4)))))
        v5 = open(v3, 0, 438);
    else
        v5 = sub_4338d0(v3, 0, 0x40000000, g_484ab0);
    g_484a34 = v5;
    if (v5 == 0xffffffff)
    {
        return v5;
    }
    else if (!g_484b10)
    {
        if (!g_484b70)
        {
            v6 = sub_40edb0(&v1);
            if (v6)
            {
                if (v6 == 1)
                {
                    if (v1)
                    {
                        if (g_484d50)
                            g_484d50();
                        error(0, 0, dcgettext(NULL, "This does not look like a tar archive", 5));
                        g_484d58 = 2;
                        return g_484a34;
                    }
                    else
                    {
                        return g_484a34;
                    }
                }
                g_483f70 = v6;
            }
            else
            {
                if (v1)
                {
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcgettext(NULL, "This does not look like a tar archive", 5));
                    g_484d58 = 2;
                }
                sub_42a260(g_484838->field_0, 0);
                if (!g_484b70)
                    return g_484a34;
            }
            v5 = g_484a34;
        }
        if (v5 > 0x3fffffff)
            sub_433d60(v5 - 0x40000000);
        else
            close(v5);
        g_484161 = 0;
        v15 = sub_42b310(v9, v10, v11, v12, v13, v14);
        g_484160 = 1;
        g_484168 = v15;
        g_483f38 = 0;
        g_483f50 = g_483f58;
        return g_484a34;
    }
    else
    {
        g_483f38 = 0;
        g_483f50 = g_483f58;
        return v5;
    }
}



// Function: _open_archive @ 0xf0f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long stderr;
extern char g_483f20;
extern unsigned long long g_483f28;
extern unsigned long long g_483f30;
extern unsigned long long g_483f38;
extern unsigned int g_483f40;
extern unsigned long long g_483f50;
extern unsigned long long g_483f58;
extern char g_484160;
extern unsigned int g_484168;
extern unsigned int g_484178;
extern unsigned long long g_484820;
extern unsigned long long g_484830;
extern unsigned long long g_484838;
extern long long g_484840;
extern unsigned int g_484a34;
extern char g_484a50;
extern long long g_484ab0;
extern char g_484ab9;
extern char g_484b70;
extern char g_484b84;
extern unsigned long long g_484b98;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;
extern char g_484d70;
extern unsigned long long g_484d78;

unsigned long long _open_archive(unsigned int a0)
{
    unsigned long long v4;  // 4099
    unsigned int v5;  // eax
    long long v11;  // rcx
    long long v12;  // r8
    long long v13;  // r9
    unsigned int *err;  // rax
    unsigned int v15;  // r12d
    unsigned long long v16;  // 4099
    unsigned long long v17;  // rbp
    unsigned long v18;  // 4110
    unsigned int v19;  // eax
    unsigned long v6;  // r12
    long long v22;  // rdi
    long long v23;  // rsi
    long long v24;  // rdx
    long long v25;  // rcx
    long long v26;  // r8
    long long v27;  // r9
    unsigned long long v28;  // 4099
    unsigned long long v29;  // 4098
    long long v30;  // rdi
    unsigned int v7;  // ebp
    long long v31;  // rsi
    long long v32;  // rdx
    char *v33;  // rsi
    unsigned int v34;  // eax
    unsigned int v35;  // eax
    unsigned long long v36;  // 4098
    unsigned long long v8;  // rdi
    int v9;  // eax
    unsigned long long v10;  // rdi
    unsigned long v37;  // fs
    long long v39;  // rdi
    long long v40;  // rsi
    long long v41;  // rdx
    long long v42;  // rcx
    long long v43;  // r8
    long long v44;  // r9
    unsigned int *v0;  // [bp-0x30], Other Possible Types: unsigned int, unsigned long long
    char v1;  // [bp-0x21]
    unsigned long v2;  // [bp-0x20]

    if (!g_484b98)
    {
        if (g_484d50)
            g_484d50();
        v33 = "Invalid value for record_size";
    }
    else if (g_484830)
    {
        sub_42f6f0(&g_484840);
        g_484178 = 0;
        sub_40c980();
        if (a0 != 2)
        {
            g_483f40 = a0;
            sub_40ca80(a0);
            v4 = *((long long *)&g_484b70);
            g_483f38 = 0;
            g_484160 = g_484ab9;
            if (!v4)
            {
                v6 = g_484838;
                v0 = *((long long *)g_484838);
                if (!strcmp(*((long long *)g_484838), "-"))
                    goto LABEL_40f2d9;
                if (a0 != 1)
                {
                    if (!a0)
                    {
                        g_484a34 = sub_40eed0();
                        if (g_484a34 >= 0)
                        {
                            sub_40c8d0();
                            goto LABEL_0x40f1aa;
                        }
                    }
                }
                else
                {
                    v7 = 0;
                    if (g_484b84)
                    {
                        v7 = 1;
                        sub_424710();
                        v6 = g_484838;
                        v0 = *((long long *)g_484838);
                    }
                    if (g_484a50)
                    {
                        if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v0, 58), v8 = (unsigned long long)*((long long *)v6), v0 = v8, !g_484d78 || (v0 = v8, g_484d78 <= v0 || (v0 = v8, v0 = v0, memchr(v8, 47, g_484d78 - v8)))))
                            v9 = open(v0, 66, 438);
                        else
                            v9 = sub_4338d0(v0, 66, 0x40000000, g_484ab0);
                    }
                    else
                    {
                        if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v0, 58), v10 = (unsigned long long)*((long long *)v6), !g_484d78 || g_484d78 <= v10 || (v0 = v10, memchr(v10, 47, g_484d78 - v10))))
                            v9 = creat();
                        else
                            v9 = sub_4338d0(v0, 65, 0x40000000, g_484ab0);
                    }
                    g_484a34 = v9;
                    if (v9 >= 0)
                    {
                        if (v9 > 0x3fffffff)
                        {
                            sub_42abd0();
                            sub_42ab80(v30, v31, v32, v11, v12, v13);
                            g_483f30 = 0;
                            return v2 - *((long long *)(40 + v37));
                        }
                        if ((char)sub_42ab10())
                            goto LABEL_40f4a8;
                    }
                    err = __errno_location();
                    v15 = *(err);
                    if (v7)
                    {
                        v0 = err;
                        sub_4245d0();
                        err = v0;
                        goto LABEL_40f218;
                    }
                }
            }
            else if (a0 != 1 && !a0)
            {
                v5 = sub_42b310();
                g_484160 = 1;
                g_484168 = v5;
                g_483f50 = g_483f58;
                goto LABEL_0x40f1aa;
            }
        }
        else
        {
            g_483f40 = 0;
            sub_40ca80(0);
            v16 = *((long long *)&g_484b70);
            g_483f38 = 0;
            g_484160 = g_484ab9;
            if (v16)
                abort(); /* do not return */
            v17 = *((long long *)g_484838);
            if (!strcmp(v17, "-"))
            {
LABEL_40f2d9:
                v18 = g_484a50;
                g_484160 = 1;
                if (!(char)v18)
                {
                    if (a0 == 1)
                    {
                        v29 = g_484820;
                        g_484a34 = 1;
                        if (v29)
                            goto LABEL_40f49b;
LABEL_40f48d:
                        g_483f28 = stderr;
LABEL_40f49b:
                        if ((char)sub_42ab10())
                        {
LABEL_40f4a8:
                            sub_42abd0();
                            sub_42ab80(v30, v31, v32, v11, v12, v13);
LABEL_40f4b2:
                            if (a0 - 1 > 1)
                                return v2 - *((long long *)(40 + v37));
                            g_483f30 = 0;
                            return v2 - *((long long *)(40 + v37));
                        }
                    }
                    else if (a0 == 2)
                    {
                        v28 = g_484820;
                        g_484a34 = 0;
                        g_483f20 = 1;
                        g_483f50 = g_483f58;
                        if (!v28)
                            goto LABEL_40f48d;
                    }
                    else if (!a0)
                    {
                        g_484a34 = 0;
                        v19 = sub_40edb0(&v1);
                        if (v19 > 1)
                        {
                            if (g_484d50)
                            {
                                v0 = v19;
                                g_484d50();
                                v19 = v0;
                            }
                            sub_40bf70(v19);
                            error(0, 0, dcgettext(NULL, "Archive is compressed. Use %s option", 5));
                            sub_4163d0(v22, v23, v24, v25, v26, v27); /* do not return */
                        }
                        else if (v1)
                        {
                            if (g_484d50)
                                g_484d50();
                            error(0, 0, dcgettext(NULL, "This does not look like a tar archive", 5));
                            g_484d58 = 2;
                        }
                    }
                    for (; g_484a34 < 0 || g_484a34 <= 0x3fffffff && !(char)sub_42ab10(); g_483f28 = stderr)
                    {
                        err = __errno_location();
                        v15 = *(err);
LABEL_40f218:
                        *(err) = v15;
                        sub_4327e0(*((long long *)g_484838));
                        v35 = sub_42af40();
                        v36 = g_484820;
                        g_484168 = v35;
                    }
                    sub_42abd0();
                    sub_42ab80(v30, v31, v32, v11, v12, v13);
                    if (!a0)
                        return sub_40e130();
                    goto LABEL_40f4b2;
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    v33 = "Cannot verify stdin/stdout archive";
                }
            }
            else
            {
                if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v17, 58), v17 = (unsigned long long)*((long long *)g_484838), !g_484d78 || g_484d78 <= v17 || memchr(v17, 47, g_484d78 - v17)))
                    v34 = open(v17, 66, 438);
                else
                    v34 = sub_4338d0(v17, 66, 0x40000000, g_484ab0);
                g_484a34 = v34;
                if (sub_40edb0(NULL) <= 1)
                    goto LABEL_0x40f1aa;
                if (g_484d50)
                    g_484d50();
                v33 = "Cannot update compressed archives";
            }
        }
    }
    else
    {
        if (g_484d50)
            g_484d50();
        v33 = "No archive name given";
    }
    error(0, 0, dcgettext(NULL, v33, 5));
    sub_4163d0(v39, v40, v41, v42, v43, v44); /* do not return */
}



// Function: match_volume_label @ 0xf760
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned long long g_483f18;
extern long long g_484a40;
extern struct_1 *g_484d50;

void match_volume_label(void)
{
    unsigned long long v3;  // rdi
    struct_0 *v4;  // rbp
    long long v14;  // rdi
    long long v15;  // rsi
    long long v16;  // rdx
    long long v17;  // rcx
    long long v18;  // r8
    long long v19;  // r9
    long long v7;  // rdi
    long long v8;  // rsi
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    char v0;  // [bp-0x1d8]
    char v1;  // [bp-0x70]

    v3 = g_483f18;
    if (g_483f18)
    {
LABEL_40f789:
        if ((char)sub_40df40(v3))
            return;
        if (g_484d50)
            g_484d50();
        sub_44ac40(1, g_484a40);
        sub_44ac40(0, g_483f18);
        error(0, 0, dcgettext(NULL, "Volume %s does not match %s", 5));
        sub_4163d0(v7, v8, v9, v10, v11, v12); /* do not return */
    }
    else
    {
        v4 = sub_40e130();
        if (v4)
        {
            if (v4->field_9c == 86)
            {
                sub_423c20(&g_483f18, v4, 100);
            }
            else if (v4->field_9c == 103)
            {
                sub_42f630(&v0);
                sub_41cac0(&v1, v4, sub_4219b0(&v4->padding_0[124], 12));
                sub_41c8c0(&v0);
                sub_42f6f0(&v0);
            }
            v3 = g_483f18;
            if (g_483f18)
                goto LABEL_40f789;
        }
        if (g_484d50)
            g_484d50();
        sub_44ac50(g_484a40);
        error(0, 0, dcgettext(NULL, "Archive not labeled to match %s", 5));
        sub_4163d0(v14, v15, v16, v17, v18, v19); /* do not return */
    }
}



// Function: close_archive @ 0xf900
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    void* field_10;
} struct_0;

extern unsigned int g_483f40;
extern unsigned long long g_483f48;
extern char g_483f58;
extern char g_484161;
extern unsigned int g_484168;
extern void* g_484190;
extern void* g_484198;
extern struct_0 *g_484828;
extern long long g_484840;
extern int g_484a34;
extern char g_484a50;
extern char g_484cf8;

void close_archive(void)
{
    if (g_484cf8 || g_483f40 == 1)
    {
        do
        {
            sub_40dfd0();
        } while (g_483f48 > *((long long *)&g_483f58));
    }
    sub_40cd50();
    if (g_484a50)
        sub_4118d0();
    if (g_484a34 > 0x3fffffff)
    {
        if (sub_433d60(g_484a34 - 0x40000000))
            goto LABEL_40f9b9;
    }
    else
    {
        if (close(g_484a34))
        {
LABEL_40f9b9:
            sub_432680(g_484828->field_0);
        }
    }
    sub_42aca0(g_484168, g_484161);
    sub_42f6f0(&g_484840);
    free(g_484190);
    free(g_484198);
    sub_40bfc0(NULL);
    return;
}



// Function: _gnu_flush_read @ 0xf9e0
extern char g_483f20;
extern unsigned long long g_483f38;
extern unsigned long long g_483f48;
extern char g_483f50;
extern unsigned int g_484164;
extern unsigned long long g_484170;
extern void g_484a34;
extern char g_484b10;
extern char g_484b98;

long long _gnu_flush_read(void)
{
    unsigned long v1;  // 4110
    unsigned long v2;  // rax
    unsigned long v3;  // 4098
    unsigned int v4;  // edi
    unsigned long v5;  // rax
    unsigned long v6;  // rdi

    sub_4108d0(0);
    v1 = g_483f20;
    g_484164 = 0;
    if ((char)v1 && g_484170)
    {
        *((unsigned int *)&g_484a34) = 1;
        v2 = sub_42af00();
        v3 = *((long long *)&g_484b98);
        *((unsigned int *)&g_484a34) = 0;
        if (v3 != v2)
            sub_40d6d0(v2); /* do not return */
        v4 = 0;
        goto LABEL_40fa19;
    }
    while (1)
    {
        v4 = *((int *)&g_484a34);
LABEL_40fa19:
        if (v4 > 0x3fffffff)
        {
            v5 = sub_433da0(v4 - 0x40000000);
            v6 = v5;
        }
        else
        {
            v5 = sub_44af10();
            v6 = v5;
        }
        if (*((long long *)&g_484b98) == v6)
        {
            g_483f38 = g_483f38 + 1;
            return v5;
        }
        if (v6)
        {
            if (v6 != 0xffffffffffffffff)
                return (unsigned long long)sub_40d390(v6);
            if (*(__errno_location()) == 28 && g_484b10)
                goto LABEL_40fa79;
            sub_40d300();
        }
        else
        {
            if (!g_484b10)
                return (unsigned long long)sub_40d390(v6);
            do
            {
LABEL_40fa79:
            } while (!(char)sub_40e880());
        }
    }
    if (g_483f48 == *((long long *)&g_483f50))
        return sub_40dfd0();
    return *((long long *)&g_483f50);
}



// Function: gnu_flush_read @ 0xfb20
extern char g_40d770;
extern unsigned long long g_484150;

int gnu_flush_read(void)
{
    g_484150 = &g_40d770;
    sub_40f9e0();
    g_484150 = gnu_flush_read;
    return gnu_flush_read;
}



// Function: flush_write @ 0xfb50
extern unsigned long long g_484158;

void flush_write(void)
{
    goto *((void *)(g_484158));
}



// Function: open_archive @ 0xfb70
extern unsigned long long g_483f30;
extern unsigned long long g_484150;
extern unsigned long long g_484158;
extern unsigned long long g_484a40;

long long open_archive(unsigned int a0)
{
    unsigned long long v1;  // 4098

    g_484150 = sub_40fb20;
    g_484158 = sub_40e850;
    sub_40f0f0(a0);
    if (a0 == 1)
    {
        v1 = g_484a40;
        g_483f30 = 0;
        if (v1)
        {
            sub_40e330();
            return sub_40c070();
        }
    }
    else if (!(a0 & 0xfffffffd) && g_484a40)
    {
        sub_40f760();
        return sub_40c070();
    }
    return sub_40c070();
}



// Function: copy_string_unquote @ 0xfbf0
char * copy_string_unquote(void)
{
    char *v1;  // rax
    char v2;  // bl
    char *ptr;  // r12
    unsigned long len;  // rax

    v1 = sub_44c7a0();
    v2 = *(v1);
    ptr = v1;
    if (v2 != 0x22 && v2 != 39)
    {
        sub_423c70(ptr);
        return ptr;
    }
    len = strlen(ptr);
    if (*(&ptr[len] - 1) != v2)
    {
        sub_423c70(ptr);
    }
    else
    {
        memmove(ptr, ptr + 1, len - 2);
        *(&ptr[len] - 2) = 0;
        sub_423c70(ptr);
    }
    return ptr;
}



// Function: alloc_action @ 0xfc60
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern struct_2 *g_484268;
extern unsigned long long g_484270;

void alloc_action(unsigned int a0)
{
    struct_0 *idx;  // rax

    idx = sub_44c760(24);
    if (g_484268)
    {
        *((struct_0 **)&g_484268->padding_0[0]) = idx;
        idx->field_8 = a0;
        g_484268 = idx;
    }
    else
    {
        idx->field_8 = a0;
        g_484270 = idx;
        g_484268 = idx;
    }
    return;
}



// Function: getwidth @ 0xfcb0
typedef struct FILE {
} FILE;

long long getwidth(FILE *a0)
{
    char *v2;  // rax
    long long num;  // rax
    unsigned short v0;  // [bp-0x16]

    if (!ioctl(fileno(a0), 21523) && v0)
        return v0;
    v2 = getenv("COLUMNS");
    if (v2)
    {
        num = strtol(v2, NULL, 10);
        if (num > NULL)
            return num;
    }
    return 80;
}



// Function: format_checkpoint_string @ 0xfdf0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern struct_0 g_481140;
extern unsigned int g_4841d8;
extern struct_1 *g_484d50;
extern unsigned int g_484d58;

unsigned long long format_checkpoint_string(unsigned long a0, long long a1, struct_0 *a2, char a3, unsigned int a4)
{
    struct_0 *v12;  // r15
    unsigned long v13;  // r13
    char v22;  // al
    unsigned long idx;  // rax
    struct_0 *v25;  // rsi
    unsigned long index;  // rax
    long long iter;  // rbx
    char *v15;  // rax
    char v16;  // al
    char v17;  // al
    struct_0 *v18;  // rax
    double v19;  // xmm0lq
    unsigned long v20;  // r12
    long long num;  // r15
    char *v0;  // [bp-0x1a0]
    unsigned int v1;  // [bp-0x194]
    unsigned long long v2;  // [bp-0x190]
    struct_0 *v3;  // [bp-0x180]
    int v4[3];  // [bp-0x178]
    unsigned int v5;  // [bp-0x170]
    char v6;  // [bp-0x168]
    unsigned long v7;  // [bp-0x148]
    unsigned long v8;  // [bp-0x140]
    unsigned long long v9;  // [bp-0x110]
    char v10;  // [bp-0x58]

    v12 = a2;
    v13 = a4;
    iter = a1;
    if (a3)
    {
        v0 = dcgettext(NULL, "write", 5);
        v15 = sub_443090(v13, &v10);
        if (!v12)
            v12 = dcgettext(NULL, "Write checkpoint %u", 5);
    }
    else
    {
        v0 = dcgettext(NULL, "read", 5);
        v15 = sub_443090(v13, &v10);
        if (!v12)
            v12 = dcgettext(NULL, "Read checkpoint %u", 5);
    }
    v3 = v12;
    v16 = v12->field_0;
    if (v16)
    {
        v1 = v13;
        do
        {
            if (v16 == 37)
            {
                v3 = &v12->field_1;
                v17 = v12->field_1;
                if (v17 == 123)
                {
                    v20 = sub_40fd40(v3, &v3);
                    if (!v20)
                        goto LABEL_40ff80;
                    v22 = v3->field_0;
                    if (v22 != 42)
                    {
                        switch (v22)
                        {
                        case 84:
                            sub_40cd50();
                            v9 = ",";
                            idx = sub_4382c0(v20, &v7, 17988);
                            if ((unsigned int)idx)
                            {
                                if (g_484d50)
                                    g_484d50();
                                v2 = sub_4354b0(&v7);
                                error(0, 0, dcgettext(NULL, "cannot split string '%s': %s", 5));
                                v25 = &g_481140.field_0;
                                g_484d58 = 2;
                            }
                            else
                            {
                                v25 = &v6;
                                if (v7)
                                {
                                    idx = 0;
                                    v25 = &v6;
                                    do
                                    {
                                        *((long long *)&(&v6)[8 * idx]) = *((long long *)(v8 + idx * 8));
                                        idx += 1;
                                    } while (idx != v7);
                                    if ((unsigned int)idx == 3)
                                        goto LABEL_4101ab;
                                }
                                index = (int)idx;
                                do
                                {
                                    *((unsigned long *)&(&v6)[8 * index]) = 0;
                                    index += 1;
                                } while ((unsigned int)index != 3);
                            }
LABEL_4101ab:
                            iter += sub_40ce80(a0, v25, 44, 0);
                            sub_435420(&v7);
                            break;
                        case 116:
LABEL_4100a7:
                            gettimeofday(v4, NULL);
                            iter += sub_441100(a0, v20, localtime(v4), 0, v5 * 1000);
                            break;
                        case 85: case 86: case 87: case 88: case 89: case 90: case 91: case 92: case 93: case 94: case 95: case 96: case 97: case 98: case 101: case 102: case 103: case 104: case 105: case 106: case 107: case 108: case 109: case 110: case 111: case 112: case 113: case 114:
                            goto LABEL_40ff80;
                        case 99:
                            goto LABEL_410028;
                        case 100:
                            goto LABEL_40fff8;
                        case 115:
                            goto LABEL_40ffd0;
                        case 117:
                            goto LABEL_40ffb0;
                        default:
                            goto LABEL_40ff80;
                        }
                    }
                    else
                    {
                        num = strtol(v20, NULL, 10);
                        goto LABEL_4101de;
                    }
                }
                else if (v17 != 42)
                {
                    switch (v17)
                    {
                    case 84:
                        sub_40cd50();
                        iter += sub_40ce80(a0, &g_481140.field_0, 44, 0);
                        break;
                    case 85: case 86: case 87: case 88: case 89: case 90: case 91: case 92: case 93: case 94: case 95: case 96: case 97: case 98: case 101: case 102: case 103: case 104: case 105: case 106: case 107: case 108: case 109: case 110: case 111: case 112: case 113: case 114:
LABEL_40ff80:
                        iter += 2;
                        fputc_unlocked(37, a0);
                        fputc_unlocked(v3->field_0, a0);
                        break;
                    case 99:
LABEL_410028:
                        iter += (int)format_checkpoint_string(a0, iter, "%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r", a3, v1);
                        v18 = v3;
                        continue;
                    case 100:
LABEL_40fff8:
                        sub_40cd50();
                        iter += (int)__fprintf_chk(a0, 0x1, "%.0f", v19);
                        v18 = v3;
                        continue;
                    case 115:
LABEL_40ffd0:
                        fputs_unlocked(v0, a0);
                        iter += strlen(v0);
                        v18 = v3;
                        continue;
                    case 116:
                        v20 = "%c";
                        goto LABEL_4100a7;
                    case 117:
LABEL_40ffb0:
                        fputs_unlocked(v15, a0);
                        iter += strlen(v15);
                        v18 = v3;
                        continue;
                    default:
                        goto LABEL_40ff80;
                    }
                }
                else
                {
                    num = sub_40fcb0(a0);
LABEL_4101de:
                    if (num > iter)
                    {
                        do
                        {
                            iter += 1;
                            fputc_unlocked(32, a0);
                        } while (num != iter);
                    }
                    v18 = v3;
                }
            }
            else
            {
                iter += 1;
                fputc_unlocked(v16, a0);
                v18 = v3;
                if (v18->field_0 == 13)
                {
                    g_4841d8 = 1;
                    iter = 0;
                }
            }
        } while ((v12 = v18 + 1, v3 = v12, v16 = v18->field_1, v18->field_1));
    }
    fflush_unlocked(a0);
    return iter & 0xffffffff;
}



// Function: checkpoint_compile_action @ 0x10490
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    char *field_10;
} struct_2;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern int g_4841e0;
extern unsigned int g_484260;
extern struct_1 *g_484d50;

int checkpoint_compile_action(unsigned long a0)
{
    unsigned int v3;  // edi
    struct_0 *v4;  // rax
    long long v13;  // r8
    long long v14;  // r9
    struct_2 *v15;  // rax
    struct_2 *v16;  // rax
    unsigned long v17;  // fs
    unsigned int v5;  // eax
    char *v6;  // rsi
    unsigned long long num;  // rax
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    long long v12;  // rcx
    char *v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    if (!g_484260)
    {
        sigemptyset(&g_4841e0);
        g_484260 = 1;
    }
    if (!strcmp(a0, ".") || !strcmp(a0, "dot"))
        return (unsigned long long)sub_40fc60(0);
    if (!strcmp(a0, "bell"))
    {
        return (unsigned long long)sub_40fc60(1);
    }
    else if (strcmp(a0, "echo"))
    {
        v3 = 2;
        if (strncmp(a0, "echo=", 5))
        {
            if (!strncmp(a0, "exec=", 5))
            {
                v3 = 5;
                goto LABEL_41069d;
            }
            if (!strncmp(a0, "ttyout=", 7))
            {
                v15 = (unsigned long long)sub_40fc60(3);
                v15->field_10 = sub_40fbf0();
            }
            else if (strncmp(a0, "sleep=", 6))
            {
                if (!strcmp(a0, "totals"))
                    return (unsigned long long)sub_40fc60(6);
                if (!strncmp(a0, "wait=", 5))
                {
                    v4 = (unsigned long long)sub_40fc60(7);
                    v5 = sub_42c8a0(a0 + 5);
                    v4->field_10 = v5;
                    return sigaddset(&g_4841e0, v5);
                }
                if (g_484d50)
                    g_484d50();
                v6 = "%s: unknown checkpoint action";
                goto LABEL_410760;
                goto LABEL_410760;
            }
            else
            {
                num = strtoul(a0 + 6, &v0, 10);
                if (!*(v0))
                {
                    *((unsigned long long *)((unsigned long long)sub_40fc60(4) + 16)) = num;
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    v6 = "%s: not a valid timeout";
LABEL_410760:
                    error(0, 0, dcgettext(NULL, v6, 5));
                    sub_4163d0(v9, v10, v11, v12, v13, v14); /* do not return */
                }
            }
        }
        else
        {
LABEL_41069d:
            v16 = (unsigned long long)sub_40fc60(v3);
            v16->field_10 = sub_40fbf0();
        }
        return v1 - *((long long *)(40 + v17));
    }
    else
    {
        return (unsigned long long)sub_40fc60(2);
    }
    return (unsigned long long)sub_40fc60(0);
}



// Function: checkpoint_finish_compile @ 0x107a0
extern int g_4841e0;
extern unsigned int g_484260;
extern unsigned long long g_484270;
extern unsigned int g_484b78;

long long checkpoint_finish_compile(void)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax

    v1 = g_484260;
    if (!g_484260)
    {
        if (g_484b78)
        {
            if (g_484270)
                return g_484260;
            sub_410490("echo");
            v1 = g_484260;
        }
        else
        {
            return g_484260;
        }
    }
    if ((unsigned int)v1 != 1)
        return v1;
    sigprocmask(0, &g_4841e0, NULL);
    v2 = g_484b78;
    if (!(unsigned int)v2)
        g_484b78 = 10;
    g_484260 = 2;
    return v2;
}



// Function: checkpoint_flush_actions @ 0x10820
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
} struct_0;

extern unsigned long long g_4841d0;
extern unsigned int g_4841d8;
extern struct_0 *g_484270;

unsigned long long checkpoint_flush_actions(void)
{
    struct_0 *v2;  // rbx
    unsigned long long v3;  // rax
    struct_0 *v4;  // rbx
    long long v5;  // rax
    unsigned long long v6;  // rsi
    long long v7;  // rbp
    unsigned long long v0;  // [bp-0x20]

    v2 = g_484270;
    if (!g_484270)
        return v3;
    while (1)
    {
        v4 = v2;
        if (v4->field_8 != 3 || !g_4841d0 || (v3 = (unsigned long long)g_4841d8, !g_4841d8))
        {
            v2 = v4->field_0;
            if (!v4->field_0)
                return v3;
        }
        else
        {
            v0 = g_4841d0;
            v5 = sub_40fcb0(g_4841d0);
            v6 = v0;
            if (v5)
            {
                v7 = v5 - 2;
                while (1)
                {
                    fputc_unlocked(32, v6);
                    v6 = g_4841d0;
                    if (v7 == -0x1)
                        break;
                    v7 -= 1;
                }
            }
            fputc_unlocked(13, v6);
            v3 = fflush_unlocked(g_4841d0);
            v2 = v4->field_0;
            if (!v4->field_0)
                return v3;
        }
    }
}



// Function: checkpoint_run @ 0x108d0
extern unsigned int g_484278;
extern unsigned int g_484b78;

long long checkpoint_run(char a0)
{
    unsigned long v1;  // rax

    if (!g_484b78)
        return v1;
    g_484278 = g_484278 + 1;
    if (g_484278 % g_484b78)
        return g_484278 / g_484b78;
}



// Function: checkpoint_finish @ 0x10910
typedef struct FILE {
} FILE;

extern FILE *g_4841d0;
extern unsigned int g_484b78;

unsigned long long checkpoint_finish(void)
{
    unsigned long long v1;  // rax

    if (!g_484b78)
        return g_484b78;
    v1 = sub_410820();
    if (!g_4841d0)
        return v1;
    return fclose(g_4841d0);
}



// Function: process_noop @ 0x10950
void process_noop(void)
{
    return;
}



// Function: dumpdir_cmp @ 0x10960
unsigned int dumpdir_cmp(char *iter, char *a1)
{
    char i;  // cl
    unsigned int v2;  // eax
    unsigned long v3;  // rax

    while (1)
    {
        i = *(iter) - 0x44;
        v2 = 1 << (i & 63);
        do
        {
            do
            {
                if (!*(iter))
                    return *(a1);
            } while (i > 21);
            if (v2 & 0xffffff & 0x114000)
                return *(a1);
            if (v2 & 0xffffff & 0x200400)
            {
                if (!strchr("YN", *(a1)))
                {
                    return 1;
                }
                else if (strcmp(iter + 1, a1 + 1))
                {
                    return 1;
                }
            }
        } while (!((char)v2 & 1));
        if (strcmp(iter, a1))
            return 1;
        v3 = strlen(iter) + 1;
        iter = &iter[v3];
        a1 = &a1[v3];
    }
}



// Function: diff_init @ 0x10af0
extern unsigned long long g_484288;
extern unsigned long long g_484b28;
extern long long g_484b98;

void diff_init(void)
{
    unsigned long v2;  // rax
    unsigned long long v3;  // 4098
    char v0;  // [bp-0x18]

    v2 = sub_425000(&v0, g_484b98);
    v3 = g_484b28;
    g_484288 = v2;
    if (v3)
        sub_41fd80();
    return;
}



// Function: report_difference @ 0x10b50
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
} struct_0;

extern long long g_483f28;

void report_difference(struct_0 *a0, long long a1)
{
    unsigned long long v11;  // rdx
    unsigned long long v12;  // rcx
    unsigned long long v13;  // r8
    unsigned long long v14;  // r9
    unsigned long v16;  // fs
    char *v17;  // rax
    unsigned int v0;  // [bp-0xd8]
    unsigned int v1;  // [bp-0xd4]
    char *v2;  // [bp-0xd0]
    char *v3;  // [bp-0xc8]
    unsigned long v4;  // [bp-0xc0]
    char v5;  // [bp-0xb8]
    unsigned long long v6;  // [bp-0xa8]
    unsigned long long v7;  // [bp-0xa0]
    unsigned long long v8;  // [bp-0x98]
    unsigned long long v9;  // [bp-0x90]
    unsigned long v10;  // [bp+0x8]

    v6 = v11;
    v7 = v12;
    v8 = v13;
    v9 = v14;
    v4 = *((long long *)(40 + v16));
    if (a1)
    {
        v17 = sub_423bb0(1, a0->field_8);
        __fprintf_chk(g_483f28, 0x1, "%s: ", v17);
        v0 = 16;
        v2 = &v10;
        v1 = 48;
        v3 = &v5;
        __vfprintf_chk(g_483f28, 1, a1, &v0);
        __fprintf_chk(g_483f28, 0x1, "\n");
    }
    sub_42f7d0(1);
    if (v4 != *((long long *)(40 + v16)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: process_rawdata @ 0x10c80
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern void g_484288;
extern unsigned int g_484290;
extern struct_0 g_484840;
extern long long g_484848;

unsigned int process_rawdata(unsigned long a0, void* a1)
{
    unsigned long v1;  // rax

    v1 = sub_424960(g_484290, *((long long *)&g_484288), a0);
    if (a0 != v1)
    {
        if (v1 != 0xffffffffffffffff)
        {
            sub_410b50(&g_484840.field_0, dcngettext(NULL, "Could only read %lu of %lu byte", "Could only read %lu of %lu bytes", a0, 5));
        }
        else
        {
            sub_432820(g_484848);
            sub_410b50(&g_484840.field_0, 0);
        }
        return 0;
    }
    else if (!memcmp(a1, *((long long *)&g_484288), a0))
    {
        return 1;
    }
    else
    {
        sub_410b50(&g_484840.field_0, dcgettext(NULL, "Contents differ", 5));
        return 0;
    }
}



// Function: get_stat_data @ 0x10d70
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_484840;

unsigned int get_stat_data(long long a0)
{
    if (!sub_4246f0())
    {
        return 1;
    }
    else if (*(__errno_location()) != 2)
    {
        sub_432d30(a0);
        sub_410b50(&g_484840.field_0, 0);
        return 0;
    }
    else
    {
        sub_432d50(a0);
        sub_410b50(&g_484840.field_0, 0);
        return 0;
    }
}



// Function: diff_archive @ 0x11730
typedef struct struct_1 {
    long long field_0;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_483f28;
extern char g_484280;
extern struct_0 *g_484408;
extern long long g_484840;
extern long long g_484848;
extern char g_484850;
extern unsigned int g_484a54;
extern struct_1 *g_484d50;
extern unsigned int g_484d58;

long long diff_archive(void)
{
    char *v2;  // rax
    unsigned long v3;  // rax
    unsigned long v4;  // fs
    unsigned long v0;  // [bp-0x20]

    sub_40d1f0(g_484408);
    if (g_484a54)
    {
        if (g_484280)
        {
            v2 = dcgettext(NULL, "Verify ", 5);
            __fprintf_chk(g_483f28, 0x1, v2);
        }
        sub_422990(&g_484840, g_484408, -0x1);
    }
    switch (*((char *)&g_484408[19].field_0 + 4))
    {
    case 0: case 48: case 55: case 83:
LABEL_4117fd:
        if (g_484850)
            break;
    case 49:
        v0 = *((long long *)(40 + v4));
        if (!sub_410d70(g_484848))
            goto LABEL_0x4110c3;
        else
            goto LABEL_0x4110e8;
    case 53: case 68:
        if (!(char)sub_4201b0(&g_484840))
        {
            break;
        }
        else
        {
            sub_411650();
            break;
        }
    case 86:
        return v3;
    default:
        if (g_484d50)
            g_484d50();
        sub_44aa70(g_484848);
        error(0, 0, dcgettext(NULL, "%s: Unknown file type '%c', diffed as normal file", 5));
        g_484d58 = 2;
        goto LABEL_4117fd;
    }
}



// Function: verify_volume @ 0x118d0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b80;
extern unsigned int g_483f40;
extern char g_484280;
extern unsigned long long g_484288;
extern long long g_484400;
extern void g_484408;
extern struct_0 *g_484838;
extern long long g_484840;
extern unsigned int g_484a34;
extern char g_484b51;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void verify_volume(void)
{
    long long v5;  // rdi
    unsigned int *err;  // rax
    int v15;  // edi
    int v16;  // eax
    unsigned int i;  // r14d
    int v18;  // r12d
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    unsigned short v0;  // [bp-0x60]
    unsigned int v1;  // [bp-0x5c]
    char v2;  // [bp-0x58]

    if (!(char)sub_432fd0())
    {
        if ((char)sub_430840())
            goto LABEL_411a80;
        sub_41f510();
        if (!g_484288)
            goto LABEL_411af1;
    }
    else
    {
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Archive contains file names with leading prefixes removed.", 5));
        if (!(char)sub_430840(v5, v6, v7, v8, v9, v10))
            goto LABEL_411aaf;
LABEL_411a80:
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Archive contains transformed file names.", 5));
LABEL_411aaf:
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Verification may fail to locate original files.", 5));
        sub_41f510(v5, v6, v7, v8, v9, v10);
        if (!g_484288)
        {
LABEL_411af1:
            sub_410af0();
        }
    }
    fsync(g_484a34);
    ioctl(g_484a34, 587);
    v1 = 1;
    v0 = 2;
    if ((g_484a34 <= 0x3fffffff ? (int)ioctl(g_484a34, 1074294017) : (unsigned int)sub_434080(g_484a34 - 0x40000000, 1074294017, &v0)) < 0)
    {
        err = __errno_location();
        v15 = g_484a34;
        if (*(err) != 5)
        {
            if (g_484a34 <= 0x3fffffff)
                goto LABEL_411c58;
LABEL_41199c:
            if (sub_433f90(v15 - 0x40000000, 0, 0))
            {
                sub_432bd0(g_484838->field_0);
                return;
            }
        }
        else
        {
            v16 = (g_484a34 <= 0x3fffffff ? (int)ioctl(g_484a34, 1074294017) : (unsigned int)sub_434080(g_484a34 - 0x40000000, 1074294017, &v0));
            v15 = g_484a34;
            if (v16 < 0)
            {
                if (g_484a34 > 0x3fffffff)
                    goto LABEL_41199c;
LABEL_411c58:
                if (lseek(v15, 0, 0))
                {
                    sub_432bd0(g_484838->field_0);
                    return;
                }
            }
        }
    }
    g_484280 = 1;
    g_483f40 = 0;
    sub_40dfc0();
    while (1)
    {
        do
        {
            while (1)
            {
                i = sub_4219e0(&g_484408, &g_484840, 0);
                if (i == 5)
                {
                    v18 = 0;
                    do
                    {
                        v18 += 1;
                        sub_40d1f0(*((long long *)&g_484408));
                        i = sub_4219e0(&g_484408, &g_484840, 0);
                    } while (i == 5);
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcngettext(NULL, "VERIFY FAILURE: %d invalid header detected", "VERIFY FAILURE: %d invalid headers detected", v18, 5));
                    g_484d58 = 2;
                    if (i == 4)
                    {
                        g_483f40 = 1;
                        g_484280 = 0;
                        return;
                    }
                }
                else if (i == 4)
                {
                    g_483f40 = 1;
                    g_484280 = 0;
                    return;
                }
                if (i != 3)
                {
                    sub_4215c0(*((long long *)&g_484408), &g_484840, &g_484400, 1);
                    sub_411730();
                    sub_42f6f0(&g_484840);
                }
                else
                {
                    sub_40d1f0(*((long long *)&g_484408));
                    if (!g_484b51)
                        break;
                }
            }
            if (sub_4219e0(&g_484408, &g_484840, 0) == 3)
            {
                g_483f40 = 1;
                g_484280 = 0;
                return;
            }
        } while (!(g_483b80 & 1));
        if (g_484d50)
            g_484d50();
        sub_443090(sub_40d180(), &v2);
        error(0, 0, dcgettext(NULL, "A lone zero block at %s", 5));
    }
}



// Function: to_octal @ 0x11ce0
long long to_octal(unsigned int a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long i;  // rdx

    do
    {
        *((char *)(a1 + a2 - 1)) = ((char)a0 & 7) + 48;
    } while (i);
    return (a0 & 7) + 48;
}



// Function: tar_copy_str @ 0x11d00
unsigned long long tar_copy_str(unsigned long a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long v1;  // rax
    char v2;  // cl

    v1 = 0;
    do
    {
        v2 = *((char *)(a1 + v1));
        *((char *)(a0 + v1)) = v2;
    } while (v2 && (v1 += 1, a2 != v1));
    return v1;
}



// Function: to_base256 @ 0x11d20
void to_base256(unsigned int a0, unsigned long long a1, unsigned long a2, unsigned long long i)
{
    do
    {
        i -= 1;
        *((char *)(a2 + i)) = a1;
        a1 = a1 >> 8 | -(a0) * 0x100000000000000;
    } while (i);
    return;
}



// Function: file_dumpable_p @ 0x11d50
extern char g_484a30;
extern char g_484a69;
extern char g_484a88;

unsigned long long file_dumpable_p(void* idx)
{
    unsigned long v2;  // rdx
    unsigned long v3;  // rcx
    long long v4;  // rax
    long long v5;  // rdx

    switch ((int)idx[24] & 0xf000)
    {
    case 16384:
        return 1;
    case 32768:
        if (!g_484a30)
        {
            if ((long long)idx[48])
                return 1;
            return ((int)idx[24] & 292) != 292;
        }
        else
        {
            if (!g_484a69)
            {
                return g_484a69;
            }
            else if (!g_484a88)
            {
                return g_484a88;
            }
            else
            {
                v2 = (long long)idx[48];
                v3 = v2 + 0x1ff;
                v4 = (long long)((v2 < 0 ? v3 : v2)) >> 9;
                if ((unsigned short)v2 & 0x1ff)
                {
                    v5 = v4 + 1;
                    if (v3 > 0x3fe)
                    {
                        v4 = v5;
                        break;
                    }
                    else
                    {
                        break;
                    }
                }
                return _INSERT(v4, 0, (long long)idx[64] < v4);
            }
        }
    default:
        return 0;
    }
}



// Function: split_long_name @ 0x11e00
unsigned long long split_long_name(unsigned long a0, unsigned long long a1)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    if (a1 <= 156)
    {
        v1 = a1 - 1;
        if (*((char *)(a0 + a1 - 1)) == 47)
        {
            v1 = a1 - 2;
            if (!v1)
                return v1;
        }
        else if (!v1)
        {
            return v1;
        }
    }
    else
    {
        v1 = 155;
    }
    do
    {
        v2 = v1;
    } while (*((char *)(a0 + v2)) != 47 && (v1 = v2 - 1, v2 != 1));
    return v2;
}



// Function: hash_link @ 0x11e50
void hash_link(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: compare_links @ 0x11e70
void compare_links(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: to_chars_subst @ 0x11e90
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_484ba4;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int to_chars_subst(unsigned int a0, unsigned int a1, long long a2, long long a3, unsigned long long *a4, long long a5, unsigned long a6, long long a7)
{
    unsigned long v7;  // rcx
    char v8;  // cl
    long long v9;  // r15
    long long v10;  // r12
    unsigned long long v11;  // rax
    unsigned long long v12;  // r12
    unsigned long long iter;  // rbx
    unsigned long long v0;  // [bp-0xf8]
    unsigned long long v1;  // [bp-0xf0]
    char v2;  // [bp-0xbc], Other Possible Types: unsigned int
    char v3;  // [bp-0xb8]
    char v4;  // [bp-0x98]
    char v5;  // [bp-0x97]
    char v6;  // [bp-0x77]

    if (a1)
    {
        v7 = a6 - 1;
        if (v7 * 8 <= 63)
        {
            v8 = (char)v7 * 8;
            v9 = (v8 & 63 ? 1 << (v8 & 63) : 1 << (v8 & 63));
            v10 = v9 - 1;
            v11 = sub_443090();
        }
        else
        {
            v10 = -0x1;
            v9 = -0x8000000000000000;
            v11 = sub_443090(-0x1, &v3);
        }
        *((char *)(sub_443090(v9, &v6) - 1)) = 45;
        goto LABEL_411f29;
    }
    else
    {
        if (a6 * 3 - 3 <= 63)
        {
            v10 = (1 << ((char)(a6 * 3 - 3 & 0xffffffff) & 63)) - 1;
            v11 = sub_443090(v10, &v3);
            if (a0)
                goto LABEL_411f37;
LABEL_41207d:
            sub_443090(a2, &v4);
            if (!a4)
            {
LABEL_412096:
                if (g_484d50)
                    g_484d50();
                v0 = v11;
                error(0, 0, dcgettext(NULL, "value %s out of %s range %s..%s", 5));
                g_484d58 = 2;
                return 0;
            }
        }
        else
        {
            v10 = -0x1;
            v11 = sub_443090(-0x1, &v3);
LABEL_411f29:
            if (!a0)
                goto LABEL_41207d;
LABEL_411f37:
            *((char *)(sub_443090(-(a2), &v5) - 1)) = 45;
            if (!a4)
                goto LABEL_412096;
        }
        v12 = v10 & a4(&v2);
        v2 &= g_484ba4 == 6;
        if (v2)
            v12 = -(v12);
        iter = sub_443090();
        if (v2)
        {
            *((char *)(iter - 1)) = 45;
            iter -= 1;
        }
        if (g_484d50)
            g_484d50();
        v1 = iter;
        v0 = v11;
        error(0, 0, dcgettext(NULL, "value %s out of %s range %s..%s; substituting %s", 5));
        return sub_412150(v2, v12, a3, 0, a5, a6, a7);
    }
}



// Function: to_chars @ 0x12150
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_4842b8;
extern unsigned int g_484ba4;
extern struct_0 *g_484d50;

unsigned int to_chars(unsigned int a0, unsigned long long a1, unsigned long a2, unsigned long long *a3, char *a4, unsigned long long a5, long long a6)
{
    unsigned long long v2;  // r12
    unsigned int v3;  // eax
    unsigned long long v4;  // r14
    unsigned long long v6;  // r15
    char *v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]

    v2 = a1;
    v3 = g_484ba4 & 0xfffffffb;
    if (!a0)
    {
        if (a5 * 3 - 3 > 63 || (1 << ((char)(a5 * 3 - 3 & 0xffffffff) & 63)) - 1 >= v2)
        {
            *(&a4[a5] - 1) = 0;
            sub_411ce0(v2, a4, a5 - 1);
            return 1;
        }
        if (v3 != 2)
            goto LABEL_412310;
        v4 = a5 - 1;
        if (v4 * 8 > 63 || (1 << ((char)v4 * 8 & 63)) - 1 >= v2)
        {
            *(a4) = 128;
            return (unsigned long long)sub_411d20(a0, v2, a4 + 1, v4);
        }
    }
    else if (v3 == 2)
    {
        v4 = a5 - 1;
        if (v4 * 8 > 63 || ~(v2) <= (1 << ((char)v4 * 8 & 63)) - 1)
        {
            *(a4) = 0xff;
            return (unsigned long long)sub_411d20(a0, v2, a4 + 1, v4);
        }
        v6 = a2 * 8;
        if (v6 <= a5 * 3 - 3)
        {
            if (!g_4842b8)
            {
                g_4842b8 = 1;
                if (g_484d50)
                {
                    v1 = a5;
                    v0 = a4;
                    g_484d50();
                    a5 = v1;
                    a4 = v0;
                }
                error(0, 0, dcgettext(NULL, "Generating negative octal headers", 5));
            }
            *(&a4[a5] - 1) = 0;
            if (v6 <= 63)
                v2 &= ~(0xffffffffffffffff << ((char)a2 * 8 & 63));
            sub_411ce0(v2, a4, v4);
            return 1;
        }
    }
    else
    {
LABEL_412310:
        a3 = NULL;
    }
    return sub_411e90(a0, v3 == 2, v2, a2, a3, a4, a5, a6);
}



// Function: uid_substitute @ 0x12380
extern unsigned int g_4842b4;

unsigned long long uid_substitute(unsigned int *a0)
{
    unsigned long long v1;  // rax

    v1 = g_4842b4;
    if ((unsigned int)v1)
    {
        *(a0) = 0;
        return v1;
    }
    else if (!sub_4269f0("nobody", &g_4842b4))
    {
        g_4842b4 = 0xfffffffe;
        *(a0) = 0;
        return 0xfffffffe;
    }
    else
    {
        *(a0) = 0;
        return g_4842b4;
    }
}



// Function: gid_substitute @ 0x123e0
extern unsigned int g_4842b0;

unsigned long long gid_substitute(unsigned int *a0)
{
    unsigned long long v1;  // rax

    v1 = g_4842b0;
    if ((unsigned int)v1)
    {
        *(a0) = 0;
        return v1;
    }
    else if (!sub_426aa0("nobody", &g_4842b0))
    {
        g_4842b0 = 0xfffffffe;
        *(a0) = 0;
        return 0xfffffffe;
    }
    else
    {
        *(a0) = 0;
        return g_4842b0;
    }
}



// Function: open_failure_recover @ 0x125b0
typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

typedef struct struct_0 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    int field_1a8;
} struct_0;

unsigned int open_failure_recover(struct_1 *a0)
{
    unsigned int *err;  // rax
    struct_0 *v2;  // rax

    err = __errno_location();
    if (!(*(err) == 24 & a0))
        return _INSERT(err, 0, *(err) == 24 & a0);
    if (!a0->field_198)
        return 0;
    v2 = a0->field_198->field_198;
}



// Function: ensure_slash @ 0x126e0
void ensure_slash(unsigned long a0)
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
    sub_4126e9();
    return;
}



// Function: file_count_links @ 0x12760
void file_count_links(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    sub_412764();
    return;
}



// Function: exclusion_tag_warning @ 0x12ad0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b80;
extern unsigned int g_484a54;
extern struct_0 *g_484d50;

long long exclusion_tag_warning(long long a0, unsigned long long a1, unsigned long a2)
{
    unsigned long long v5;  // r13
    unsigned long long v6;  // r12
    unsigned long long v7;  // rbx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x10]
    unsigned long long v3;  // [bp-0x8]

    if (g_484a54 && g_483b80 & 4)
    {
        v3 = v5;
        v2 = v6;
        v1 = v7;
        if (g_484d50)
        {
            v0 = a1;
            g_484d50();
        }
        sub_44a870(1);
        sub_44aa70(a0);
        return (unsigned long long)error(0, 0, dcgettext(NULL, "%s: contains a cache directory tag %s; %s", 5));
    }
    return g_484a54;
}



// Function: cachedir_file_p @ 0x12b70
unsigned int cachedir_file_p(int a0)
{
    unsigned long long v2;  // rax
    char v0;  // [bp-0x48]

    if (read(a0, &v0, 43) != 43)
        return 0;
    v2 = memcmp(&v0, "Signature: 8a477f597d28d172789f06886806bc55", 43);
    return _INSERT(v2, 0, !(unsigned int)v2);
}



// Function: off_to_chars @ 0x12be0
unsigned int off_to_chars(unsigned long long a0, char *a1, unsigned long long a2)
{
    return sub_412150(a0 >> 63, a0, 8, NULL, a1, a2, "off_t");
}



// Function: time_to_chars @ 0x12c10
unsigned int time_to_chars(unsigned long long a0, char *a1, unsigned long long a2)
{
    return sub_412150(a0 >> 63, a0, 8, NULL, a1, a2, "time_t");
}



// Function: write_eot @ 0x12c40
long long write_eot(void)
{
    unsigned long long *v1;  // rax
    unsigned long long *iter;  // rdi
    unsigned long long v3;  // rcx
    unsigned long v4;  // rbp

    v1 = sub_40e130();
    *(v1) = 0;
    v1[63] = 0;
    iter = v1 + 1 & 0xfffffffffffffff8;
    for (v3 = (unsigned int)(v1 - iter) + 0x200 >> 3; v3; iter += 1)
    {
        v3 -= 1;
        *(iter) = 0;
    }
    sub_40d1f0(v1);
    v4 = sub_40e130();
    memset(v4, 0, sub_40d2b0(v4));
    return sub_40d1f0(v4);
}



// Function: start_private_header @ 0x12cb0
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[249];
    unsigned int field_101;
    unsigned short field_105;
    unsigned short field_107;
    char padding_109[239];
    unsigned long long field_1f8;
} struct_0;

int start_private_header(unsigned long a0, unsigned long long a1, unsigned long a2)
{
    struct_0 *v1;  // rax
    struct_0 *idx;  // r12
    unsigned long long *iter;  // rdi
    unsigned long long v4;  // rcx
    unsigned long v5;  // rdi

    v1 = sub_40e130();
    idx = v1;
    v1->field_0 = 0;
    *((unsigned long *)&v1->padding_109[236]) = 0;
    iter = v1->padding_8 & 0xfffffffffffffff8;
    for (v4 = (unsigned int)(v1 - iter) + 0x200 >> 3; v4; iter += 1)
    {
        v4 -= 1;
        *(iter) = 0;
    }
    sub_412970(idx);
    sub_412be0(a1, &idx->padding_8[116], 12);
    v5 = 0x1ffffffff;
    if (a2 <= 0x1ffffffff)
        v5 = a2;
    if (a2 < 0)
        v5 = 0;
    sub_412c10(v5, &idx->padding_8[128], 12);
    sub_4129e0(33188, &idx->padding_8[92]);
    sub_4128b0(0, &idx->padding_8[100]);
    sub_412940(0, &idx->padding_8[108]);
    *((unsigned int *)&(idx->padding_8)[1]) = 1635021685;
    *((unsigned short *)((char *)&idx->field_101 + 1)) = 114;
    *((unsigned short *)((char *)&idx->field_101 + 3)) = 0x3030;
    return *((unsigned int *)&idx);
}



// Function: write_extended @ 0x12da0
typedef struct struct_0 {
    char padding_0[176];
    long long field_b0;
    char padding_b8[176];
    unsigned long long field_168;
    char padding_170[8];
    unsigned long long field_178;
} struct_0;

extern long long g_484a20;
extern long long g_484ae0;
extern unsigned int g_484af0;

unsigned long long * write_extended(char a0, struct_0 *a1, unsigned long long *i)
{
    struct_0 *v3;  // r14
    char *iter;  // r13
    char *j;  // rsi
    unsigned long long v14;  // rcx
    unsigned long long v5;  // rcx
    unsigned long v6;  // rax
    long long v7;  // rdx
    long long v8;  // rdi
    unsigned long v9;  // rax
    unsigned long long *v10;  // r12
    unsigned long long *node;  // rdi
    unsigned long long *v12;  // rcx
    char v0;  // [bp-0x238]
    unsigned long v1;  // [bp-0x40]

    if (a1->field_178)
    {
        return i;
    }
    else if (!a1->field_168)
    {
        return i;
    }
    else
    {
        v3 = &a1->field_168;
        iter = &v0;
        sub_41cc10(v3);
        for (v5 = 64; v5; i += 1)
        {
            v5 -= 1;
            *((unsigned long long *)&iter) = *(i);
            iter += 8;
        }
        if (!a0)
        {
            v6 = sub_41c460(a1);
            if (g_484af0)
            {
                v7 = g_484ae0;
                v8 = 120;
            }
            else
            {
                v7 = a1->field_b0;
                v8 = 120;
            }
        }
        else
        {
            v9 = sub_41c4b0();
            v7 = g_484a20;
            v8 = 103;
            v6 = v9;
        }
        sub_41ccf0(v8, v6, v7, v3);
        free(v6);
        v10 = sub_40e130();
        node = v10 + 1 & 0xfffffffffffffff8;
        *(v10) = *((unsigned long long *)&v0);
        v12 = v10 - node;
        j = &v0 - v12;
        v10[63] = v1;
        for (v14 = (unsigned int)v12 + 0x200 >> 3; v14; j += 8)
        {
            v14 -= 1;
            *(node) = *((long long *)&j);
            node += 1;
        }
        return v10;
    }
}



// Function: simple_finish_header @ 0x12f00
typedef struct struct_0 {
    char padding_0[148];
    unsigned long long field_94;
} struct_0;

long long simple_finish_header(struct_0 *a0)
{
    struct_0 *iter;  // rax
    int v2;  // edx

    *((unsigned long long *)&(a0->padding_0)[1]) = 0x2020202020202020;
    iter = a0;
    do
    {
        iter = &iter->padding_0[1];
    } while (&a0[3].padding_0[32] != iter);
    sub_412a40(v2, (a0->padding_0)[1]);
    return sub_40d1f0(a0);
}



// Function: finish_header @ 0x12f60
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

extern unsigned int g_484400;
extern unsigned int g_484a54;
extern unsigned int g_484ba4;

long long finish_header(unsigned long a0, struct_0 *a1)
{
    unsigned int v2;  // eax
    unsigned long v3;  // rax
    unsigned long v0;  // [bp-0x138]

    if (g_484a54)
    {
        v2 = a1->field_9c;
        if ((char)v2 != 75)
        {
            v3 = v2 - 76;
            if ((char)v3 > 44 || (v0 = 17592320262145, !(((char)v3 & 254 | *((char *)((char *)&v0 + ((long long)(v3 & 63) >> 3))) >> (unsigned long long)((char)v3 & 63 & 7) & 1) & 1)))
            {
                g_484400 = g_484ba4;
                sub_422990(a0, a1);
            }
        }
    }
    return sub_412f00(sub_412da0(0, a0, a1));
}



// Function: write_gnu_long_link @ 0x12fd0
void write_gnu_long_link(unsigned long a0, unsigned long a1, unsigned long a2)
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
    sub_412fe5();
    return;
}



// Function: write_header_name @ 0x13240
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_484ba4;

long long write_header_name(struct_0 *idx)
{
    unsigned int v1;  // eax
    unsigned long v2;  // rdi

    v1 = g_484ba4;
    v2 = idx->field_8;
    if (g_484ba4 == 4)
    {
        if ((char)sub_431440(idx->field_8))
        {
            v1 = g_484ba4;
            v2 = idx->field_8;
        }
        else
        {
            sub_41ca30("path", idx, 0);
        }
    }
}



// Function: start_header @ 0x132d0
extern int g_484a98;
extern int g_484a9c;
extern int g_484aa0;
extern char g_484ad2;
extern void g_484ae0;
extern long long g_484ae8;
extern unsigned int g_484af0;
extern unsigned int g_484b14;
extern unsigned long long g_484b18;
extern char g_484b50;
extern char g_484ba4;

void* start_header(void* idx)
{
    void* v4;  // rax
    void* index;  // r12
    unsigned int v14;  // r13d
    long long v15;  // rdi
    unsigned long v16;  // rbp
    unsigned long long i;  // rax
    unsigned int v6;  // ebp
    unsigned int v7;  // edi
    unsigned long v8;  // rdi
    unsigned long v9;  // rdi
    unsigned long long v10;  // rdi
    unsigned long v11;  // rdi
    unsigned int v12;  // ebp
    unsigned int v13;  // r14d
    unsigned long v0;  // [bp-0x68]
    unsigned long v1;  // [bp-0x60]
    int v2;  // [bp-0x58], Other Possible Types: unsigned long, unsigned long long

    v0 = 0;
    v1 = 0;
    v4 = sub_413240(idx);
    if (!v4)
        return NULL;
    index = v4;
    sub_4239d0((int)idx[116], idx + 116, &v0);
    sub_423aa0((int)idx[120], idx + 120, &v1);
    if (g_484b18)
    {
        v6 = (int)idx[112];
        v7 = v6 & 0xfffff000 | sub_443970(v6, (v6 & 0xf000) == 0x4000, g_484b14);
        *((unsigned int *)&idx[112]) = v7;
    }
    else
    {
        v7 = (int)idx[112];
    }
    if ((*((int *)&g_484ba4) & 0xfffffffd) == 1)
        sub_4129e0(v7 & 0xfff);
    else
        sub_4129e0();
    v8 = (int)idx[116];
    if (*((int *)&g_484ba4) == 4 && (unsigned int)v8 > 0x1fffff)
    {
        sub_41ca30("uid", idx, 0);
        v8 = 0;
    }
    if ((char)sub_4128b0(v8, index + 108))
    {
        v9 = (int)idx[120];
        if (*((int *)&g_484ba4) == 4 && (unsigned int)v9 > 0x1fffff)
        {
            sub_41ca30("gid", idx, 0);
            v9 = 0;
        }
        if ((char)sub_412940(v9, index + 116))
        {
            v10 = (long long)idx[0x88];
            if (*((int *)&g_484ba4) == 4 && v10 & 0xfffffffe00000000)
            {
                sub_41ca30("size", idx, 0);
                v10 = 0;
            }
            if ((char)sub_412be0(v10, index + 124, 12))
            {
                if (g_484af0 == 1)
                {
                    v2 = (int)*((int128_t *)&g_484ae0);
                }
                else if (g_484af0 != 2)
                {
                    if (!g_484af0)
                        v2 = (int)(int128_t)idx[248];
                }
                else
                {
                    if (sub_44be80((long long)idx[248], (long long)idx[0x100], *((long long *)&g_484ae0), g_484ae8) > 0)
                    {
                        *((long long *)&v2) = *((long long *)&g_484ae0);
                        *((long long *)&(&v2)[8]) = g_484ae8;
                    }
                    else
                    {
                        v2 = (int)(int128_t)idx[248];
                    }
                }
                v11 = *((unsigned long long *)&v2);
                if (*((int *)&g_484ba4) == 4 && (v11 & 0xfffffffe00000000 || *((unsigned long long *)(&v2 + 8))))
                {
                    sub_41ca30("mtime", idx, &v2);
                    v11 = *((unsigned long long *)&v2);
                    if (v11 & 0xfffffffe00000000)
                    {
                        v2 = 0;
                        v11 = 0;
                    }
                }
                if ((char)sub_412c10(v11, index + 0x88, 12))
                {
                    v12 = *((int *)&g_484ba4);
                    if (((short)(int)idx[112] & 0xb000) != 0x2000)
                    {
                        if ((*((int *)&g_484ba4) & 0xfffffffb) == 2)
                            goto LABEL_41348d;
                        if ((char)sub_412910(0, index + 329) && (char)sub_4128e0(0, index + 337))
                        {
LABEL_413715:
                            v12 = *((int *)&g_484ba4);
LABEL_41348d:
                            if (v12 == 4)
                            {
                                sub_41ca30("atime", idx, 0);
                                sub_41ca30("ctime", idx, 0);
                                v12 = *((int *)&g_484ba4);
                            }
                            else if (g_484b50 && (v12 & 0xfffffffb) == 2)
                            {
                                sub_412c10((long long)idx[232], index + 345, 12);
                                sub_412c10((long long)idx[264], index + 357, 12);
                                v12 = *((int *)&g_484ba4);
                            }
                            if (v12 == 1)
                            {
                                *((char *)&index[156]) = 0;
                                return index;
                            }
                            *((char *)&index[156]) = 48;
                            if (v12 <= 4)
                            {
                                if (v12 > 2)
                                {
                                    *((unsigned int *)&index[0x101]) = 1635021685;
                                    *((unsigned short *)&index[261]) = 114;
                                    *((unsigned short *)&index[263]) = 0x3030;
                                }
                                else
                                {
                                    if (v12 != 2)
                                        abort(); /* do not return */
LABEL_413722:
                                    *((unsigned long *)&index[0x101]) = 9042874888123253;
                                }
                                if (!g_484ad2)
                                {
                                    if (v0)
                                        *((unsigned long long *)&idx[32]) = sub_44c7a0(v0);
                                    else
                                        sub_4268d0((int)idx[116], idx + 32);
                                    if (v1)
                                        *((unsigned long long *)&idx[40]) = sub_44c7a0(v1);
                                    else
                                        sub_426960((int)idx[120], idx + 40);
                                    v16 = (long long)idx[32];
                                    if (*((int *)&g_484ba4) == 4)
                                    {
                                        if (strlen(v16) <= 32 && (char)sub_431440(v16))
                                        {
                                            v16 = (long long)idx[32];
                                        }
                                        else
                                        {
                                            sub_41ca30("uname", idx, 0);
                                            v16 = (long long)idx[32];
                                        }
                                    }
                                    sub_412890(v16, index + 265);
                                    if (*((int *)&g_484ba4) == 4 && (strlen((long long)idx[40]) > 32 || !(char)sub_431440((long long)idx[40])))
                                        sub_41ca30("gname", idx, 0);
                                    sub_412890((long long)idx[40], index + 297);
                                    v12 = *((int *)&g_484ba4);
                                }
                                if (v12 != 4)
                                    return index;
                                if (g_484a9c > 0)
                                {
                                    if ((long long)idx[56])
                                        sub_41ca30("SCHILY.acl.access", idx, 0);
                                    if ((long long)idx[72])
                                        sub_41ca30("SCHILY.acl.default", idx, 0);
                                }
                                if (g_484aa0 > 0 && (long long)idx[48])
                                    sub_41ca30("RHT.security.selinux", idx, 0);
                                if (g_484a98 <= 0)
                                    return index;
                                i = 0;
                                v2 = 0;
                                if (!(long long)idx[344])
                                    return index;
                                do
                                {
                                    sub_41ca30(*((long long *)((long long)idx[352] + i * 24)), idx, &v2);
                                    i = v2 + 1;
                                    v2 = i;
                                } while (i < (long long)idx[344]);
                                return index;
                            }
                            if (v12 != 6)
                                abort(); /* do not return */
                            goto LABEL_413722;
                        }
                    }
                    else
                    {
                        v13 = gnu_dev_major((long long)idx[128]);
                        v14 = gnu_dev_minor((long long)idx[128]);
                        if (v13 > 0x1fffff && *((int *)&g_484ba4) == 4)
                        {
                            v13 = 0;
                            sub_41ca30("devmajor", idx, 0);
                        }
                        if ((char)sub_412910(v13, index + 329))
                        {
                            if (*((int *)&g_484ba4) != 4 || v14 <= 0x1fffff)
                            {
                                v15 = v14;
                            }
                            else
                            {
                                sub_41ca30("devminor", idx, 0);
                                v15 = 0;
                            }
                            if ((char)sub_4128e0(v15, index + 337))
                                goto LABEL_413715;
                        }
                    }
                }
            }
        }
    }
    return NULL;
}



// Function: write_long_link @ 0x13a40
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_484ba4;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

long long write_long_link(void* a0)
{
    char v1;  // al
    unsigned long v2;  // rax

    if (g_484ba4 <= 6)
    {
        v1 = 1 << ((char)g_484ba4 & 63);
        if (v1 & 42)
        {
            if (g_484d50)
                g_484d50();
            sub_44aa70((long long)a0[24]);
            v2 = (unsigned long long)error(0, 0, dcgettext(NULL, "%s: link name is too long; not dumped", 5));
            g_484d58 = 2;
            return v2;
        }
        else if (v1 & 0x44)
        {
            return (unsigned long long)sub_412fd0(a0, (long long)a0[24], 75);
        }
        else if (g_484ba4 == 4)
        {
            return sub_41ca30("linkpath", a0, 0);
        }
    }
    abort(); /* do not return */
}



// Function: dump_hard_link @ 0x13b00
typedef struct struct_0 {
    long long field_0;
    char padding_8[80];
    unsigned long long field_58;
    unsigned long long field_60;
    unsigned long long field_68;
    char padding_70[24];
    unsigned long long field_88;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
} struct_1;

extern unsigned long long g_4842c0;
extern char g_4842c8;
extern char g_484ab8;
extern char g_484b92;
extern unsigned int g_484ba4;

unsigned long long dump_hard_link(struct_0 *idx)
{
    struct_1 *index;  // rbp
    unsigned long v5;  // r12
    unsigned long len;  // rax
    void* v7;  // r8
    struct_0 *v8;  // r8
    char v0;  // [bp-0x59]
    unsigned long v1;  // [bp-0x58]
    unsigned long v2;  // [bp-0x50]

    if (!g_4842c0)
        return 0;
    if (idx->field_68 <= *((long long *)&g_4842c8) && !g_484ab8)
        return g_484ab8;
    v2 = idx->field_60;
    v1 = idx->field_58;
    index = sub_4418d0();
    if (!index)
        return 0;
    v5 = sub_433020(index + 1, 1, g_484b92);
    if (index->field_10)
        index->field_10 = index->field_10 - 1;
    sub_40d180();
    sub_423be0(&idx->padding_8[16], v5);
    len = strlen(v5);
    if (len > (unsigned long long)((g_484ba4 != 2) + 99))
        sub_413a40(idx);
    idx->field_88 = 0;
    v7 = sub_4132d0(idx);
    if (!v7)
        return 0;
    sub_411d00(v7 + 157, v5, 100);
    v8[1].padding_8[4] = 49;
    sub_412f60(idx, v8);
    if (g_484ab8)
    {
        v0 = g_484ab8;
        sub_430b70(idx->field_0, 0);
        return v0;
    }
    return 1;
}



// Function: pad_archive @ 0x13c80
long long pad_archive(long long a0)
{
    unsigned long long v3;  // r12
    long long v4;  // r12
    unsigned long long v5;  // rbx
    long long v6;  // r12
    unsigned long long *v7;  // rax
    unsigned long long *iter;  // rdi
    unsigned long long v9;  // rcx
    long long v10;  // rax
    long long v11;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (a0 <= 0)
        return v11;
    v1 = v3;
    v4 = a0 - 0x200;
    v0 = v5;
    v6 = v4;
    while (1)
    {
        v7 = sub_40e130();
        *(v7) = 0;
        v7[63] = 0;
        iter = v7 + 1 & 0xfffffffffffffff8;
        for (v9 = (unsigned int)(v7 - iter) + 0x200 >> 3; v9; iter += 1)
        {
            v9 -= 1;
            *(iter) = 0;
        }
        v10 = sub_40d1f0(v7);
        if (v6 == v4 - (a0 - 1 & 0xfffffffffffffe00))
            break;
        v6 -= 0x200;
    }
    return v10;
}



// Function: dump_regular_file @ 0x13d10
typedef struct struct_0 {
    long long field_0;
    unsigned int field_8;
    char padding_c[124];
    long long field_88;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern char g_483b80;
extern char g_484b52;
extern struct_1 *g_484d50;

unsigned int dump_regular_file(unsigned int a0, struct_0 *a1)
{
    long long v4;  // r14
    void* v5;  // rax
    long long v14;  // rax
    long long v6;  // r14
    unsigned long v7;  // r15
    long long v8;  // r12
    unsigned int v9;  // edx
    unsigned int v10;  // eax
    unsigned long long v11;  // rdx
    unsigned long long *iter;  // rdi
    unsigned long long v13;  // rcx
    long long i;  // [bp-0x68]
    long long v1;  // [bp-0x60]
    char v2;  // [bp-0x58]

    v4 = a1->field_88;
    sub_40d180();
    v5 = sub_4132d0(a1);
    if (!v5)
        return 2;
    sub_412f60(a1, v5);
    sub_40cc00(*((long long *)&a1->field_8), a1->field_88, a1->field_88);
    do
    {
        v6 = v4;
        if (v6 <= NULL)
        {
LABEL_413dbf:
            return 0;
        }
        v7 = sub_40e130();
        v8 = sub_40d2b0(v7);
        if (v6 < v8)
        {
            v9 = v6;
            if ((unsigned short)v9 & 0x1ff)
            {
                v10 = 0x200 - (v9 & 0x1ff);
                v11 = v7 + v6;
                if (v10 < 8)
                {
                    if (!((char)v10 & 4))
                    {
                        if (v10)
                        {
                            *((char *)v11) = 0;
                            if ((char)v10 & 2)
                                *((unsigned short *)(v11 + v10 - 2)) = 0;
                        }
                    }
                    else
                    {
                        *((unsigned int *)v11) = 0;
                        *((unsigned int *)(v11 + v10 - 4)) = 0;
                    }
                }
                else
                {
                    *((unsigned long long *)v11) = 0;
                    *((unsigned long *)(v11 + v10 - 8)) = 0;
                    iter = v11 + 8 & 0xfffffffffffffff8;
                    for (v13 = (v10 + v11 - (char *)iter & 0xffffffff) >> 3 & 0xffffffff; v13; iter += 1)
                    {
                        v13 -= 1;
                        *(iter) = 0;
                    }
                }
            }
            if (a0 <= 0)
            {
                sub_40d1f0(v7 + (v6 - 1 & 0xfffffffffffffe00));
            }
            else
            {
                v8 = v6;
                continue;
            }
            goto LABEL_413dbf;
        }
        else if (a0 <= 0)
        {
            v4 = v6 - v8;
            sub_40d1f0(v7 + (v8 - 1 & 0xfffffffffffffe00));
            continue;
        }
        v14 = sub_424960(a0, v7, v8);
        if (v14 == -0x1)
        {
            sub_424e00(a1->field_0, a1->field_88 - v6, v8);
            sub_413c80(v6);
            return 1;
        }
        v4 = v6 - v14;
        i = v14;
        v1 = v4;
        sub_40d1f0((v8 - 1 & 0xfffffffffffffe00) + v7);
    } while (i == v8);
    memset(v7 + i, 0, v8 - i);
    if (g_483b80 & 128)
    {
        if (g_484d50)
            g_484d50();
        sub_443090(v1, &v2);
        sub_44aa70(a1->field_0);
        error(0, 0, dcngettext(NULL, "%s: File shrank by %s byte; padding with zeros", "%s: File shrank by %s bytes; padding with zeros", v1, 5));
    }
    if (!g_484b52)
        sub_42f7d0(1);
    sub_413c80(v6 - v8);
    return 1;
}



// Function: get_directory_entries @ 0x13fe0
typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

typedef struct struct_0 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    int field_1a8;
} struct_0;

extern unsigned int g_4847e4;

unsigned long long get_directory_entries(struct_1 *a0)
{
    unsigned long fp;  // rax

    while (1)
    {
        fp = fdopendir(*((int *)&a0[1].padding_0[8]));
        *((unsigned long *)&a0[1].padding_0[0]) = fp;
        if (fp)
        {
            return sub_44b0d0(fp, g_4847e4);
        }
        else if (!(char)sub_4125b0(a0))
        {
            return 0;
        }
    }
}



// Function: check_links @ 0x14030
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern long long g_4842c0;
extern struct_1 *g_484d50;

void check_links(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    struct_0 *v5;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!g_4842c0)
        return;
    v1 = v3;
    v0 = v4;
    v5 = sub_441930(g_4842c0);
    if (!v5)
        return;
    do
    {
        if (v5->field_10)
        {
            if (g_484d50)
                g_484d50();
            sub_44ac50(v5 + 1);
            error(0, 0, dcgettext(NULL, "Missing links to %s.", 5));
        }
    } while ((v5 = (struct_0 *)(unsigned long long)sub_441980(g_4842c0, v5), v5));
    return;
}



// Function: subfile_open @ 0x140d0
typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

typedef struct struct_0 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    int field_1a8;
} struct_0;

extern char g_467708;
extern unsigned int g_481234;
extern char g_484298;

int subfile_open(struct_1 *a0, long long a1, unsigned int a2)
{
    unsigned long v1;  // rdi
    unsigned int v2;  // eax

    if (!g_484298)
    {
        g_484298 = 1;
        strerror(2);
        dcgettext(NULL, &g_467708, 5);
    }
    while (1)
    {
        v1 = g_481234;
        if (a0)
            v1 = *((int *)&a0[1].padding_0[8]);
        v2 = __openat_2(v1, a1, a2);
        if (v2 >= 0)
        {
            return v2;
        }
        else if (!(char)sub_4125b0(a0))
        {
            return v2;
        }
    }
}



// Function: check_exclusion_tags @ 0x14160
typedef struct struct_0 {
    long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    struct struct_1 *field_18;
    struct struct_0 *field_20;
} struct_0;

typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

extern struct_0 *g_4842d0;
extern unsigned int g_484808;

unsigned int check_exclusion_tags(struct_1 *a0, long long *a1)
{
    struct_0 *v1;  // rbx
    struct_0 *v2;  // rbx
    unsigned int v3;  // ebp

    v1 = g_4842d0;
    if (!g_4842d0)
        return 0;
    while (1)
    {
        v2 = v1;
        v3 = sub_4140d0(a0, v2->field_0, g_484808);
        if (v3 >= 0)
        {
            if (!v2->field_18 || (char)v2->field_18(v3))
                break;
            close(v3);
        }
        v1 = v2->field_20;
        if (!v2->field_20)
            return 0;
    }
    close(v3);
    if (!a1)
        return v2->field_10;
    *(a1) = v2->field_0;
    return v2->field_10;
}



// Function: restore_parent_fd @ 0x14200
typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    unsigned int field_1a8;
} struct_1;

typedef struct struct_0 {
    long long field_0;
    char padding_8[80];
    unsigned long long field_58;
    char padding_60[328];
    unsigned int field_1a8;
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

extern unsigned int g_481234;
extern unsigned int g_484804;

void restore_parent_fd(struct_1 *a0)
{
    struct_0 *idx;  // rbx
    unsigned int v3;  // ebp
    unsigned int v4;  // eax
    unsigned int v5;  // r12d
    unsigned int v6;  // eax
    unsigned int v7;  // eax
    stat v0;  // [bp-0xb8]

    idx = a0->field_198;
    if (!idx)
    {
        return;
    }
    else if (!idx->field_1a8)
    {
        v3 = __openat_2(a0->field_1a8, "..", g_484804);
        if (v3 < 0)
        {
            v4 = *(__errno_location());
            v3 = -(*(__errno_location()));
            if (*(__errno_location()) > 0)
            {
                v5 = __openat_2(g_481234, idx->field_0, g_484804);
                if (v5 >= 0)
                {
                    v6 = fstat(v3, &v0);
                    if (v6 || *((long long *)&idx->padding_60[0]) != v0.st_ino || idx->field_58 != *((unsigned long long *)&v0))
                        close(v5);
                    else
                        v3 = v5;
                }
            }
        }
        else
        {
            v7 = fstat(v3, &v0);
            if (v7 || *((long long *)&idx->padding_60[0]) != v0.st_ino || idx->field_58 != *((unsigned long long *)&v0))
            {
                close(v3);
                idx->field_1a8 = 2;
                return;
            }
        }
        idx->field_1a8 = v3;
        return;
    }
    else
    {
        return;
    }
}



// Function: dump_file @ 0x14330
extern unsigned long long g_484b28;

void dump_file(long long a0, long long a1, long long a2)
{
    char v0;  // [bp-0x1e8]
    long long v1;  // [bp-0x50]

    sub_42f630(&v0);
    v1 = a0;
    sub_414ca0(&v0, a1, a2);
    if (!a0)
    {
        sub_42f6f0(&v0);
        return;
    }
    if (g_484b28)
    {
        sub_41f580(a0);
        sub_42f6f0(&v0);
    }
    else
    {
        sub_42f6f0(&v0);
    }
    return;
}



// Function: create_archive @ 0x143d0
typedef struct struct_0 {
    char field_0;
    char padding_1[15];
    long long field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    char padding_18[32];
    long long field_38;
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

extern unsigned int g_481234;
extern unsigned long long g_4842c8;
extern void g_4843a0;
extern long long g_4843b0;
extern unsigned int g_484630;
extern unsigned int g_484804;
extern long long g_484a20;
extern long long g_484a28;
extern long long g_484b28;
extern char g_484b50;
extern char g_484b69;

void create_archive(void)
{
    unsigned long long v13;  // r13
    unsigned long long v14;  // r12
    unsigned long v24;  // r14
    char *v25;  // r14
    long long v26;  // rdi
    long long v27;  // rsi
    long long v28;  // rdx
    long long v29;  // rcx
    long long v30;  // r8
    long long v31;  // r9
    unsigned long v32;  // fs
    unsigned long long v15;  // rax
    unsigned long long v33;  // rbp
    void* v34;  // rax
    unsigned long len;  // rax
    void* v36;  // rax
    unsigned long len1;  // rax
    long long v16;  // rbp
    struct_0 *ptr;  // r15
    struct_1 *idx;  // rbx
    unsigned long v19;  // rbp
    char *j;  // rax
    unsigned long len2;  // r12
    unsigned int v22;  // eax
    unsigned long v0;  // [bp-0x218]
    char *v1;  // [bp-0x210]
    unsigned long i;  // [bp-0x200]
    unsigned long long v3;  // [bp-0x1f8]
    stat v4;  // [bp-0x1a0]
    unsigned int v5;  // [bp-0x50]
    unsigned long v6;  // [bp-0x40]
    char v7;  // [bp-0x38]
    unsigned long v8;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned long long v9;  // [bp-0x18]
    unsigned long long v10;  // [bp-0x10]
    unsigned long long v11;  // [bp-0x8]

    v9 = v13;
    v8 = v14;
    v15 = 0;
    if (g_484630 != 2)
        v15 = g_484b69 ^ 1;
    g_4842c8 = v15;
    sub_40fb70(1);
    sub_40cca0();
    if (!g_484b50)
    {
        while (1)
        {
            v16 = sub_426cd0(1);
            if (!v16)
                break;
            if ((char)sub_416b00(v16, 0))
                continue;
            sub_414330(0, v16, v16);
        }
    }
    else
    {
        i = 0;
        sub_4276a0();
        while (1)
        {
            ptr = sub_427b00();
            if (!ptr)
                break;
            if (!(char)sub_416b00(ptr->field_10, 0))
                sub_414330(0, ptr->field_10, ptr->field_10);
        }
        sub_427b80();
        while (1)
        {
            idx = sub_427b00();
            if (!idx)
                break;
            if (!(char)sub_416b00(idx->field_10, 0))
            {
                v19 = idx->field_10;
                v0 = strlen(idx->field_10);
                if (v0 >= i)
                {
                    do
                    {
                        ptr = sub_44c710(ptr, &i);
                    } while (i <= v0);
                    v19 = idx->field_10;
                }
                memcpy(ptr, v19, v0);
                if (*(&(&ptr->field_0)[v0] - 1) != 47)
                {
                    (&ptr->field_0)[v0] = 47;
                    v0 += 1;
                }
                v1 = &v3;
                sub_42f630(&v3);
                if (sub_41fa60(idx->field_38))
                {
                    for (; *(j); j = &j[len2 + 1])
                    {
                        len2 = strlen(j);
                        if (*(j) == 89)
                        {
                            if (!v3)
                            {
                                v22 = __openat_2(g_481234, idx->field_10, g_484804);
                                if (v22 >= 0)
                                {
                                    v5 = v22;
                                    if (!fstat(v22, &v4))
                                    {
                                        v3 = sub_44c7a0(idx->field_10);
                                    }
                                    else
                                    {
                                        sub_424ef0(idx->field_10, !*((long long *)&idx[1].padding_0[0]), sub_424ec0);
                                        break;
                                    }
                                }
                                else
                                {
                                    sub_424ef0(idx->field_10, !*((long long *)&idx[1].padding_0[0]), sub_424dd0);
                                    break;
                                }
                            }
                            v24 = v0 + len2;
                            if (v24 > i)
                            {
                                do
                                {
                                    ptr = sub_44c710(ptr, &i);
                                } while (i < v24);
                            }
                            v25 = j + 1;
                            strcpy(&(&ptr->field_0)[v0], v25);
                            sub_414330(v1, v25, ptr);
                        }
                    }
                }
                sub_42f6f0(v1);
            }
        }
        free(ptr);
    }
    sub_412c40();
    sub_40f900();
    sub_430b30(v26, v27, v28, v29, v30, v31);
    if (!g_484b28)
    {
        return;
    }
    else if (v6 == *((long long *)(40 + v32)))
    {
        v11 = v9;
        v10 = v8;
        v9 = v33;
        v8 = *((long long *)(40 + v32));
        if (!*((long long *)&g_4843a0))
            return;
        if (fseeko(*((long long *)&g_4843a0), 0, 0))
            sub_432b00(g_484b28);
        if (sub_42aed0(fileno(*((long long *)&g_4843a0)) & 0xffffffff))
            sub_432d70(g_484b28);
        __fprintf_chk(*((long long *)&g_4843a0), 0x1, "%s-%s-%d\n", "GNU tar", "1.34", 2);
        v34 = sub_442f50(g_484a20, &v7);
        len = strlen(v34);
        fwrite_unlocked(v34, len + 1, 1, *((long long *)&g_4843a0));
        v36 = sub_443090(g_484a28, &v7);
        len1 = strlen(v36);
        fwrite_unlocked(v36, len1 + 1, 1, *((long long *)&g_4843a0));
        if (!ferror_unlocked(*((long long *)&g_4843a0)) && g_4843b0)
            sub_441a40(g_4843b0, sub_41f1c0, *((long long *)&g_4843a0));
        if (ferror_unlocked(*((long long *)&g_4843a0)))
        {
            sub_432e10(g_484b28);
            if (!fclose(*((long long *)&g_4843a0)))
                return;
        }
        else if (!fclose(*((long long *)&g_4843a0)))
        {
            return;
        }
        sub_432680(g_484b28);
        return;
    }
    else
    {
        __stack_chk_fail(); /* do not return */
    }
}



// Function: dump_dir0 @ 0x14710
typedef struct struct_0 {
    char padding_0[88];
    unsigned long long field_58;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern char g_483b82;
extern unsigned long long g_484620;
extern unsigned int g_484a54;
extern char g_484ad1;
extern unsigned int g_484ad4;
extern char g_484b50;
extern unsigned int g_484ba4;
extern struct_1 *g_484d50;

long long dump_dir0(void* index, unsigned long a1)
{
    unsigned long ptr;  // rbx
    struct_0 *v5;  // rbp
    unsigned int v14;  // eax
    void* v15;  // rdx
    unsigned long long *node;  // rdi
    unsigned long long v17;  // rcx
    unsigned int v18;  // eax
    unsigned long v19;  // rax
    unsigned long len;  // rax
    unsigned long v21;  // r14
    unsigned long len1;  // rbp
    unsigned long v23;  // rdx
    char *v6;  // rax
    unsigned long v24;  // fs
    unsigned long v25;  // rdi
    char *v26;  // rax
    char *v27;  // rax
    unsigned long len2;  // rax
    unsigned long ptr1;  // rbp
    char *idx;  // r14
    unsigned long v31;  // fs
    long long v8;  // rdi
    unsigned long iter;  // rax
    unsigned long n;  // rbp
    unsigned long ptr2;  // r13
    unsigned long n1;  // rbx
    unsigned int v13;  // edx
    unsigned long v0;  // [bp-0x50]
    unsigned long v1;  // [bp-0x48]
    unsigned long v2;  // [bp-0x40]

    ptr = a1;
    v5 = (long long)index[408];
    sub_40d180();
    *((unsigned long long *)&index[0x88]) = 0;
    v6 = sub_4132d0(index);
    if (!v6)
        return v2 - *((long long *)(40 + v31));
    idx = v6;
    sub_416890(index);
    if (!g_484b50)
    {
        idx[156] = 53;
        sub_412f60(index, idx);
        goto LABEL_41478d;
    }
    else
    {
        v8 = *((long long *)(g_484620 + 56));
        if (g_484ba4 != 4)
        {
            idx[156] = 0x44;
            if (!v8)
                goto LABEL_41478d;
            sub_40d180();
            iter = sub_41fa90(*((long long *)(g_484620 + 56)));
            n = sub_41f4c0(iter);
            sub_412be0(n, idx + 124, 12);
            sub_412f60(index, idx);
            sub_40cc00((long long)index[8], n, n);
            if (n > NULL)
            {
                do
                {
                    ptr2 = sub_40e130();
                    n1 = sub_40d2b0(ptr2);
                    if (n < n1)
                    {
                        v13 = n;
                        if ((unsigned short)v13 & 0x1ff)
                        {
                            v14 = 0x200 - (v13 & 0x1ff);
                            v15 = ptr2 + n;
                            if (v14 >= 8)
                            {
                                *((unsigned long long *)v15) = 0;
                                *((unsigned long *)((char *)v15 + v14 - 8)) = 0;
                                node = v15 + 8 & 0xfffffffffffffff8;
                                for (v17 = (v14 + v15 - node & 0xffffffff) >> 3 & 0xffffffff; v17; node += 1)
                                {
                                    v17 -= 1;
                                    *(node) = 0;
                                }
                            }
                            else if ((char)v14 & 4)
                            {
                                *((unsigned int *)v15) = 0;
                                *((unsigned int *)((char *)v15 + v14 - 4)) = 0;
                            }
                            else if (v14)
                            {
                                *((char *)v15) = 0;
                                if ((char)v14 & 2)
                                    *((unsigned short *)((char *)v15 + v14 - 2)) = 0;
                            }
                        }
                        memcpy(ptr2, iter, n);
                        return sub_40d1f0(ptr2 + (n - 1 & 0xfffffffffffffe00));
                    }
                    n -= n1;
                    memcpy(ptr2, iter, n1);
                    iter += n1;
                    sub_40d1f0(ptr2 + (n1 - 1 & 0xfffffffffffffe00));
                } while (n > NULL);
            }
        }
        else
        {
            idx[156] = 53;
            if (v8)
            {
                sub_41ca30("GNU.dumpdir", index, sub_41fa90());
                return sub_412f60(index, idx);
            }
LABEL_41478d:
            if (g_484ad4)
            {
                if (!(g_484ad1 && v5 && *((long long *)((long long)index[408] + 88)) != (long long)index[88]))
                {
                    v18 = sub_414160(index, &v1);
                    if (v18 != 1)
                    {
                        if (v18 != 2)
                        {
                            if (!v18)
                            {
                                v19 = sub_44c7a0(*((long long *)index));
                                len = strlen(v19);
                                v21 = len;
                                while (1)
                                {
                                    len1 = strlen(ptr);
                                    if (!len1)
                                        break;
                                    v23 = v21 + len1;
                                    if (v23 > len)
                                    {
                                        v0 = v23;
                                        len = v0;
                                        v19 = sub_44c610(v19, v0 + 1);
                                    }
                                    strcpy(v19 + v21, ptr);
                                    if (!(char)sub_416b00(v19, index))
                                        sub_414330(index, ptr, v19);
                                    ptr = ptr + len1 + 1;
                                }
                                if (v2 != *((long long *)(40 + v24)))
                                    __stack_chk_fail(); /* do not return */
                                return (unsigned long long)free(v19);
                            }
                        }
                        else
                        {
                            v26 = dcgettext(NULL, "contents not dumped", 5);
                            return sub_412ad0(*((long long *)index), v1, v26);
                        }
                    }
                    else
                    {
                        v27 = dcgettext(NULL, "contents not dumped", 5);
                        sub_412ad0(*((long long *)index), v1, v27);
                        len2 = strlen(*((long long *)index));
                        ptr1 = sub_44c5b0(len2 + strlen(v1) + 1);
                        strcpy(ptr1, *((long long *)index));
                        strcat(ptr1, v1);
                        sub_414330(index, v1, ptr1);
                        if (v2 != *((long long *)(40 + v24)))
                            __stack_chk_fail(); /* do not return */
                        return (unsigned long long)free(v25);
                    }
                }
                else if (g_484a54 && g_483b82 & 4)
                {
                    if (g_484d50)
                        g_484d50();
                    sub_44aa70(*((long long *)index));
                    return (unsigned long long)error(0, 0, dcgettext(NULL, "%s: file is on a different filesystem; not dumped", 5));
                }
            }
        }
    }
    return v2 - *((long long *)(40 + v31));
}



// Function: dump_dir @ 0x14c40
typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

typedef struct struct_0 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    int field_1a8;
} struct_0;

unsigned int dump_dir(struct_1 *a0)
{
    unsigned long v1;  // rax

    v1 = sub_413fe0(a0);
    if (!v1)
    {
        sub_424e60(*((long long *)&a0->padding_0[0]));
        return 0;
    }
    sub_414710(a0, v1);
    sub_414200(a0);
    free(v1);
    return 1;
}



// Function: dump_file0 @ 0x14ca0
typedef struct struct_2 {
    char padding_0[156];
    char field_9c;
} struct_2;

typedef struct struct_0 {
    char padding_0[424];
    int field_1a8;
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

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern int g_481234;
extern char g_483b80;
extern char g_483b81;
extern unsigned int g_484800;
extern unsigned int g_484808;
extern unsigned int g_484a54;
extern char g_484a88;
extern char g_484ab8;
extern unsigned long long g_484b00;
extern unsigned long long g_484b08;
extern char g_484b40;
extern char g_484b50;
extern unsigned int g_484b88;
extern unsigned int g_484b8c;
extern char g_484b92;
extern unsigned int g_484ba4;
extern struct_1 *g_484d50;

long long dump_file0(void* idx, long long a1, long long a2)
{
    struct_0 *index;  // rbx
    int v13;  // r14d
    long long v22;  // r13
    char v23;  // al
    char *v24;  // rax
    unsigned int v25;  // eax
    long long v26;  // rbx
    char v27;  // bl
    long long v28;  // rbx
    long long v29;  // rbx
    struct_2 *v30;  // r13
    void* v14;  // r15
    unsigned long long v33;  // rax
    unsigned long len;  // rax
    long long v35;  // rbx
    void* v36;  // rax
    struct_0 *v37;  // r8
    unsigned long v38;  // fs
    long long v39;  // r15
    long long v40;  // rdx
    unsigned long v41;  // rcx
    long long v42;  // rax
    unsigned int v43;  // ebx
    char v45;  // bl
    unsigned long v47;  // fs
    void* v18;  // r13
    long long v19;  // rax
    long long v20;  // rdx
    char v0;  // [bp-0x122]
    char v1;  // [bp-0x121]
    int v2;  // [bp-0x118]
    long long v3;  // [bp-0x108]
    int v4;  // [bp-0x100]
    char v5;  // [bp-0xfc], Other Possible Types: unsigned int
    unsigned long v6;  // [bp-0xf8]
    unsigned long v7;  // [bp-0xf0]
    long long v8;  // [bp-0xe0]
    int v9;  // [bp-0xd8], Other Possible Types: stat
    unsigned long v10;  // [bp-0x40]

    index = (long long)idx[408];
    v1 = !index;
    v0 = g_484b40;
    if (index)
    {
        v13 = index->field_1a8;
        if (!v0 || sub_42ca20("add", a2))
        {
            v14 = idx + 8;
            sub_423be0(idx, a2);
            sub_423be0(v14, sub_433020(a2, 0, g_484b92));
            sub_430830(v14, 1);
            if (v13 < 0)
            {
                *(__errno_location()) = -(v13);
                goto LABEL_4150b0;
            }
            else
            {
                v0 = 1;
                goto LABEL_414e4d;
            }
        }
    }
    else
    {
        v13 = g_481234;
        if (!g_484b40)
        {
            v14 = idx + 8;
            sub_423be0();
            sub_423be0(v14, sub_433020(a2, 0, g_484b92));
            sub_430830(v14, 1);
LABEL_414e4d:
            v18 = idx + 88;
            v4 = fstatat(v13, a1, v18, g_484800);
            if (v4)
            {
LABEL_4150b0:
                if (v10 != *((long long *)(40 + v38)))
                    __stack_chk_fail(); /* do not return */
                return sub_424ef0(a2);
            }
            if ((char)sub_411d50(v18))
            {
                v4 = sub_4140d0(index, a1, g_484808);
                if (v4 >= 0)
                {
                    *((int *)&idx[424]) = v4;
                    if (fstat(v4, v18))
                        goto LABEL_4150b0;
                }
                else
                {
                    goto LABEL_4150b0;
                }
            }
            v19 = (long long)idx[0x88];
            *((long long *)&idx[280]) = v19;
            v3 = v19;
            *((unsigned long long *)&idx[232]) = sub_44bd00(v18);
            *((long long *)&idx[240]) = v20;
            *((unsigned long long *)&idx[248]) = sub_44bd20(v18);
            *((long long *)&idx[0x100]) = v20;
            *((unsigned long long *)&v2) = sub_44bd10(v18);
            *((long long *)&(&v2)[8]) = v20;
            *((uint128_t *)&idx[264]) = (uint128_t)v2;
            if (g_484b50 == 1 && !v1 || !(v5 = g_484b50, ((short)(int)idx[112] & 0xf000) != 0x4000 && (v6 = (unsigned long)g_484b00, v7 = (unsigned long)g_484b08, (unsigned int)(unsigned long long)sub_44be80((long long)idx[248], (long long)idx[0x100], g_484b00, g_484b08) < 0 && (!g_484b8c || (v5 = v5, (unsigned int)(unsigned long long)sub_44be80((long long)idx[264], (long long)idx[272], v6, v7) < 0)))))
            {
                if ((char)sub_42ab40(idx))
                {
                    if (g_483b81 & 4)
                    {
                        if (g_484d50)
                            g_484d50();
                        sub_44aa70(a2);
                        return (unsigned long long)error(0, 0, dcgettext(NULL, "%s: file is the archive; not dumped", 5));
                    }
                }
                else
                {
                    v5 = (int)idx[112] & 0xf000;
                    if (v5 == 0x4000)
                    {
                        v22 = a1;
                        sub_4319f0(v13, v22, idx, 0, 0);
                        sub_431de0(v13, v22, idx, v4);
                        sub_431b80(v13, v22, idx, v4);
                        sub_4126e0(idx);
                        sub_4126e0(v14);
                        if (sub_414160(idx, &v8) == 3)
                        {
                            v24 = dcgettext(NULL, "directory not dumped", 5);
                            return sub_412ad0(*((long long *)idx), v8, v24);
                        }
                        v23 = sub_414c40(idx);
                        v13 = g_481234;
                        v4 = (int)idx[424];
                        if (index)
                            v13 = index->field_1a8;
                        if (!v23)
                            goto LABEL_41501e;
                        if (v4 >= 0)
                            goto LABEL_4153ca;
                        *(__errno_location()) = -(v4);
                        goto LABEL_41500a;
                    }
                    if (!(char)sub_413b00(idx))
                    {
                        switch (v25)
                        {
                        case 32768:
                            v39 = a1;
                            sub_4319f0(v13, v39, idx, 0, 1);
                            sub_431de0(v13, v39, idx, v4);
                            sub_431b80(v13, v39, idx, v4);
                            if (v4 && g_484a88)
                            {
                                v40 = (long long)idx[0x88];
                                v41 = v40 + 0x1ff;
                                v42 = (long long)((v40 < 0 ? v41 : v40)) >> 9;
                                if ((short)v40 & 0x1ff && v41 > 0x3fe)
                                    v42 += 1;
                                if ((long long)idx[152] >= v42 || !(v43 = (unsigned int)(unsigned long long)sub_429cd0((unsigned long long)(unsigned int)v4, idx), v43 != 3))
                                    goto LABEL_4153a7;
                                if (v43 > 1 || (sub_412760(), v43))
                                    goto LABEL_41501e;
LABEL_4153ca:
                                if (!v4)
                                {
                                    if (v13 < 0 && v0)
                                    {
                                        *(__errno_location()) = -(v13);
                                        goto LABEL_41500a;
                                    }
                                    if (fstatat(v13, a1, &v9, g_484800))
                                        goto LABEL_41500a;
LABEL_4156b5:
                                    v45 = v5 == 0x4000;
                                    if (sub_44be80(sub_44bd10(&v9), v20, *((unsigned long long *)&v2), *((unsigned long long *)(&v2 + 8))) && (g_484ab8 != 1 || v5 != 0x4000) || *((unsigned long long *)(&v9 + 48)) > v3)
                                    {
                                        if (g_483b80 & 16)
                                        {
                                            if (g_484d50)
                                                g_484d50();
                                            sub_44aa70(a2);
                                            error(0, 0, dcgettext(NULL, "%s: file changed as we read it", 5));
                                        }
                                        sub_42f7d0(1);
                                    }
                                    else
                                    {
                                        if (g_484b88 == 1 && v4 && (v3 || v45) && sub_424a80(v4, v13, a1, (long long)idx[232], (long long)idx[240]))
                                            sub_432dd0(a2);
                                    }
                                    if ((char)sub_42f670(idx) && g_484ab8)
                                    {
                                        sub_430b70(a2, v45);
                                        break;
                                    }
                                }
                                else
                                {
                                    if (!fstat(v4, &v9))
                                        goto LABEL_4156b5;
LABEL_41500a:
                                    sub_424ef0(a2, v1, sub_424ec0);
                                    goto LABEL_41501e;
                                }
                            }
                            else
                            {
LABEL_4153a7:
                                if (sub_413d10(v4, idx) == 3)
                                    abort(); /* do not return */
LABEL_41501e:
                                sub_42f670(idx);
                                break;
                            }
                        case 40960:
                            v33 = sub_439200(v13, a1, (long long)idx[0x88]);
                            *((unsigned long long *)&idx[24]) = v33;
                            if (v33)
                            {
                                sub_430830(idx + 24, 4);
                                len = strlen((long long)idx[24]);
                                if (len > (unsigned long long)((g_484ba4 != 2) + 99))
                                    sub_413a40(idx);
                                v35 = a1;
                                sub_431de0(v13, v35, idx, 0);
                                sub_431b80(v13, v35, idx, 0);
                                sub_40d180();
                                *((long long *)&idx[0x88]) = 0;
                                v36 = sub_4132d0(idx);
                                if (v36)
                                {
                                    sub_411d00(v36 + 157, (long long)idx[24], 100);
                                    v37->padding_0[156] = 50;
                                    sub_412f60(idx, v37);
                                    if (!g_484ab8)
                                        return (unsigned long long)sub_412760();
                                    sub_430b70(a2, 0);
                                    return (unsigned long long)sub_412760();
                                }
                            }
                            else
                            {
                                if (*(__errno_location()) != 12)
                                {
                                    if (v10 == *((long long *)(40 + v38)))
                                        return sub_424ef0(a2);
                                    __stack_chk_fail(); /* do not return */
                                }
                                else
                                {
                                    sub_416420(); /* do not return */
                                }
                            }
                        case 8192:
                            v29 = a1;
                            sub_4319f0(v13, v29, idx, 0, 1);
                            sub_431de0(v13, v29, idx, 0);
                            v27 = 0x33;
                            sub_431b80(v13, v29, idx, 0);
                            goto LABEL_415520;
                        case 24576:
                            v28 = a1;
                            sub_4319f0(v13, v28, idx, 0, 1);
                            sub_431de0(v13, v28, idx, 0);
                            v27 = 52;
                            sub_431b80(v13, v28, idx, 0);
                            goto LABEL_415520;
                        case 4096:
                            v26 = a1;
                            sub_4319f0(v13, v26, idx, 0, 1);
                            sub_431de0(v13, v26, idx, 0);
                            v27 = 54;
                            sub_431b80(v13, v26, idx, 0);
LABEL_415520:
                            if (g_484ba4 != 1)
                            {
                                sub_40d180();
                                *((long long *)&idx[0x88]) = 0;
                                v30 = sub_4132d0(idx);
                                if (v30)
                                {
                                    v30->field_9c = v27;
                                    if (v27 != 54)
                                    {
                                        sub_412910(gnu_dev_major((long long)idx[128]) & 0xffffffff, &v30[2].padding_0[15]);
                                        sub_4128e0(gnu_dev_minor((long long)idx[128]) & 0xffffffff, &v30[2].padding_0[23]);
                                    }
                                    sub_412f60(idx, v30);
                                    if (g_484ab8)
                                        return sub_430b70(a2, 0);
                                }
                            }
                        case 49152:
                            if (g_483b80 & 32)
                            {
                                if (g_484d50)
                                    g_484d50();
                                sub_44aa70(a2);
                                return (unsigned long long)error(0, 0, dcgettext(NULL, "%s: socket ignored", 5));
                            }
                            break;
                        }
                    }
                }
            }
            else
            {
                if (!v5 && g_484a54 && g_483b81 & 1)
                {
                    if (g_484d50)
                        g_484d50();
                    sub_44aa70(a2);
                    return (unsigned long long)error(0, 0, dcgettext(NULL, "%s: file is unchanged; not dumped", 5));
                }
            }
        }
        else if (sub_42ca20("add", a2))
        {
            v14 = idx + 8;
            sub_423be0(idx, a2);
            sub_423be0(v14, sub_433020(a2, 0, g_484b92));
            sub_430830(v14, 1);
            v0 = 0;
            goto LABEL_414e4d;
        }
    }
    return v10 - *((long long *)(40 + v47));
}



// Function: flush_file @ 0x15860
extern unsigned long long g_483f48;
extern unsigned long long g_483f50;
extern unsigned int g_484408;
extern unsigned long long g_4848c8;

long long flush_file(void)
{
    unsigned long v1;  // rdx
    unsigned long v2;  // rbx
    unsigned long v3;  // rax
    unsigned long v4;  // cc_ndep
    unsigned long long v5;  // 4114
    long long v6;  // rax
    long long i;  // rbx

    sub_40d1f0(*((long long *)&g_484408));
    v1 = g_483f48;
    v2 = g_4848c8 + 0x3fe;
    v3 = g_4848c8 + 0x1ff;
    v5 = _ccall(9, 4, g_4848c8, 0x1ff, v4);
    if (v5 & 1)
        v2 = v3;
    v6 = g_483f50 - g_483f48;
    i = (long long)(v2) >> 9;
    if (i >= v6 >> 9)
    {
        do
        {
            i -= v6 >> 9;
            sub_40dfd0();
            v1 = g_483f48;
            v6 = g_483f50 - g_483f48;
        } while (v6 >> 9 <= i);
    }
    g_483f48 = v1 + i * 0x200;
    return v6;
}



// Function: move_archive @ 0x158f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484838;
extern int g_484a34;
extern unsigned long long g_484b98;

long long move_archive(unsigned long a0)
{
    unsigned int v5;  // eax
    unsigned short v6;  // cx
    unsigned long v15;  // fs
    char v7;  // dl
    int v8;  // edi
    unsigned int *err;  // rax
    long long v11;  // rax
    unsigned long long v12;  // rbx
    long long v13;  // rsi
    long long v14;  // rax
    long long v0;  // [bp-0x30]
    unsigned short v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x24]
    unsigned long v3;  // [bp-0x20]

    if (!a0)
        return v3 - *((long long *)(40 + v15));
    v5 = a0;
    if (a0 >= 0)
    {
        v6 = 3;
        v7 = (int)a0 == a0;
    }
    else
    {
        v5 = -(v5);
        v6 = 4;
        v7 = !(v5 + a0);
    }
    v1 = v6;
    v8 = g_484a34;
    v2 = v5;
    if (!v7)
    {
LABEL_41599d:
        v11 = (v8 <= 0x3fffffff ? lseek(v8, 0, 1) : sub_433f90(v8 - 0x40000000, 0, 1));
        v12 = g_484b98 * a0;
        v13 = v11 + v12;
        if ((char)__OFSMUL__(g_484b98, a0) & 1)
            return sub_432b20(g_484838->field_0);
        if (v13 < v11 != (char)(v12 >> 63))
            return sub_432b20(g_484838->field_0);
        if (v13 < 0)
            v13 = 0;
        if (g_484a34 <= 0x3fffffff)
        {
            v0 = v13;
            v14 = lseek(g_484a34, v13, 0);
        }
        else
        {
            v0 = v13;
            v14 = sub_433f90(g_484a34 - 0x40000000);
        }
        if (v14 != v0)
            return sub_432b20(g_484838->field_0);
    }
    else if (g_484a34 <= 0x3fffffff)
    {
        if (ioctl(g_484a34, 1074294017) < 0)
            goto LABEL_415964;
    }
    else
    {
        if (sub_434080(g_484a34 - 0x40000000, 1074294017, &v1) < 0)
        {
LABEL_415964:
            err = __errno_location();
            v8 = g_484a34;
            if (*(err) != 5)
                goto LABEL_41599d;
            if (g_484a34 <= 0x3fffffff)
            {
                if (ioctl(g_484a34, 1074294017) < 0)
                    goto LABEL_415997;
            }
            else
            {
                if (sub_434080(g_484a34 - 0x40000000, 1074294017, &v1) < 0)
                {
LABEL_415997:
                    v8 = g_484a34;
                    goto LABEL_41599d;
                }
            }
        }
    }
    return v3 - *((long long *)(40 + v15));
}



// Function: write_record @ 0x15b10
extern unsigned long long g_483f30;
extern unsigned long long g_483f38;
extern unsigned long long g_483f58;
extern unsigned long long g_4842d8;
extern char g_4842e0;
extern unsigned int g_4842e4;
extern unsigned long long g_4842e8;
extern unsigned int g_484a34;

long long write_record(unsigned int a0)
{
    unsigned long v1;  // 4131
    unsigned long v2;  // rbp
    unsigned long v3;  // rax

    v1 = g_4842e0;
    v2 = g_483f58;
    g_483f58 = g_4842e8;
    if ((char)v1)
    {
        g_484a34 = 1;
        v3 = (unsigned long long)sub_40fb50();
        g_484a34 = 0;
    }
    else
    {
        sub_4158f0(g_4842d8 + g_483f30 - g_483f38);
        v3 = (unsigned long long)sub_40fb50();
    }
    g_483f58 = v2;
    if (a0 && !g_4842e0)
        v3 = sub_4158f0(g_483f38 - (g_4842d8 + g_483f30));
    g_4842e4 = 0;
    return v3;
}



// Function: write_recent_blocks @ 0x15bc0
extern void g_4842e4;
extern unsigned long long g_4842e8;
extern char g_484ba0;

long long write_recent_blocks(unsigned long a0, unsigned long long a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    long long v13;  // rax
    long long v14;  // rax
    unsigned long long node;  // rbx
    unsigned long long *v6;  // rsi
    unsigned long long *v7;  // rcx
    unsigned long v8;  // rdx
    unsigned long long *iter;  // rdi
    unsigned long long *v10;  // rcx
    unsigned long long *i;  // rsi
    unsigned long long v12;  // rcx
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (!a1)
        return v14;
    v1 = v3;
    v0 = v4;
    node = 0;
    while (1)
    {
        v6 = node * 0x200 + a0;
        v7 = *((int *)&g_4842e4) * 0x200 + g_4842e8;
        v8 = *(v6);
        *((unsigned int *)&g_4842e4) = *((int *)&g_4842e4) + 1;
        iter = v7 + 1 & 0xfffffffffffffff8;
        *(v7) = v8;
        v7[63] = v6[63];
        v10 = v7 - iter;
        i = v6 - v10;
        for (v12 = (unsigned int)v10 + 0x200 >> 3; v12; i += 1)
        {
            v12 -= 1;
            *(iter) = *(i);
            iter += 1;
        }
        if (*((int *)&g_4842e4) != *((int *)&g_484ba0))
        {
            node += 1;
            if (a1 == node)
                return *((int *)&g_4842e4);
        }
        else
        {
            node += 1;
            v13 = sub_415b10(1);
            if (a1 == node)
                return v13;
        }
    }
}



// Function: write_recent_bytes @ 0x15c70
extern int g_4842e4;
extern unsigned long long g_4842e8;
extern unsigned int g_484ba0;

void* write_recent_bytes(unsigned int a0, unsigned long long a1)
{
    unsigned long long v1;  // r12
    void* v2;  // rsi
    void* v11;  // rax
    unsigned long long v12;  // rcx
    unsigned long v13;  // 4126
    void* v3;  // rax
    unsigned long long *iter;  // rdi
    void* v5;  // rcx
    void* i;  // rsi
    unsigned long long v7;  // rcx
    void* j;  // rax
    unsigned int v9;  // ecx
    unsigned long long *node;  // rdi

    v1 = (unsigned int)a1 & 0x1ff;
    sub_415bc0(a0, a1 >> 9);
    v2 = a0 + (a1 & 0xfffffffffffffe00);
    v3 = g_4842e4 * 0x200 + g_4842e8;
    if ((unsigned int)v1 >= 8)
    {
        iter = v3 + 8 & 0xfffffffffffffff8;
        *((long long *)v3) = *((long long *)v2);
        *((long long *)((char *)v3 + v1 - 8)) = *((long long *)(v1 + (char *)v2 - 8));
        v5 = v3 - iter;
        i = v2 - v5;
        for (v7 = (v5 + v1 & 0xffffffff) >> 3 & 0xffffffff; v7; i += 8)
        {
            v7 -= 1;
            *(iter) = *((long long *)i);
            iter += 1;
        }
    }
    else if ((char)v1 & 4)
    {
        *((int *)v3) = *((int *)v2);
        *((int *)((char *)v3 + v1 - 4)) = *((int *)(v1 + (char *)v2 - 4));
    }
    else if ((unsigned int)v1)
    {
        *((char *)v3) = *((char *)v2);
        if ((char)v1 & 2)
            *((short *)((char *)v3 + v1 - 2)) = *((short *)(v1 + (char *)v2 - 2));
    }
    j = v3 + v1;
    v9 = 0x200 - v1;
    if (v9 >= 8)
    {
        *((unsigned long long *)j) = 0;
        *((unsigned long *)((char *)j + v9 - 8)) = 0;
        node = j + 8 & 0xfffffffffffffff8;
        v11 = NULL;
        for (v12 = (v9 + j - node & 0xffffffff) >> 3 & 0xffffffff; v12; j = v11)
        {
            v12 -= 1;
            *(node) = 0;
            node += 1;
        }
    }
    else if ((char)v9 & 4)
    {
        *((unsigned int *)j) = 0;
        *((unsigned int *)((char *)j + v9 - 4)) = 0;
    }
    else if (v9)
    {
        *((char *)j) = 0;
        if ((char)v9 & 2)
            *((unsigned short *)((char *)j + v9 - 2)) = 0;
    }
    v13 = g_484ba0;
    g_4842e4 = g_4842e4 + 1;
    if (g_4842e4 == (unsigned int)v13)
        return sub_415b10(1);
    return j;
}



// Function: fatal_exit @ 0x163d0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_4842f0;

void fatal_exit(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (g_4842f0)
        g_4842f0();
    error(2, 0, dcgettext(NULL, "Error is not recoverable: exiting now", 5));
}



// Function: xalloc_die @ 0x16420
void xalloc_die(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    dcgettext(NULL, "memory exhausted", 5);
    error(0, 0, "%s");
    sub_4163d0(); /* do not return */
}



// Function: hg_initfn @ 0x16460
extern unsigned long long g_4842f8;

void hg_initfn(unsigned long long a0)
{
    unsigned long long v1;  // rdx

    v1 = &g_4842f8;
    if (!a0)
        a0 = v1;
    *((unsigned int *)a0) = 0x8000000;
    return;
}



// Function: get_vcs_ignore_file @ 0x16480
typedef struct struct_0 {
    char *field_0;
    char padding_8[32];
    unsigned long long field_28;
} struct_0;

extern struct_0 g_481160;

struct_0 * get_vcs_ignore_file(char *a0)
{
    struct_0 *v1;  // r12
    unsigned long v2;  // rdi
    struct_0 *v3;  // r12

    v1 = &g_481160.field_0;
    v2 = g_481160.field_0;
    if (!g_481160.field_0)
        return &g_481160.field_0;
    while (1)
    {
        v3 = v1;
        if (!strcmp(v2, a0))
            return v3;
        v2 = v3->field_28;
        v1 = &v3->field_28;
        if (!v3->field_28)
            return &v3->field_28;
    }
}



// Function: bzr_addfn @ 0x164d0
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

int bzr_addfn(void)
{
    unsigned int v2;  // edx
    unsigned int v3;  // r13d
    unsigned long long *v4;  // rax
    struct_0 *iter;  // rbp
    unsigned long v6;  // rdx
    long long v7;  // rdi
    long long v8;  // rdi
    char v0;  // [bp+0x0]

    v3 = v2;
    v4 = __ctype_b_loc();
    while (1)
    {
        v6 = iter->field_0;
        if (!(*((char *)(*(v4) + v6 * 2 + 1)) & 32))
            break;
        iter = &iter->field_1;
    }
    if (!(char)v6)
    {
        return;
    }
    else if ((char)v6 != 35)
    {
        if ((char)v6 == 33)
        {
            if (iter->field_1 != 33)
            {
                iter = &iter->field_1;
                v3 |= 0x20000000;
            }
            else
            {
                iter += 1;
            }
        }
        if (strncmp(iter, "RE:", 3))
        {
            sub_43ee30(v8, &v0, v3);
            return;
        }
        sub_43ee30(v7, &v0, v3 & 0xefffffff | 0x8000000);
        return;
    }
    else
    {
        return;
    }
}



// Function: git_addfn @ 0x16580
int git_addfn(long long a0, char *a1, unsigned int a2)
{
    unsigned long long *v2;  // rax
    unsigned long v3;  // rcx
    char v0;  // [bp+0x0]

    v2 = __ctype_b_loc();
    while (1)
    {
        v3 = *(a1);
        if (!(*((char *)(*(v2) + v3 * 2 + 1)) & 32))
            break;
        a1 += 1;
    }
    if ((char)v3 && (char)v3 != 35)
        return sub_43ee30(a0, &v0, a2);
    return *((unsigned int *)&v2);
}



// Function: hg_addfn @ 0x16600
void hg_addfn(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
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
    sub_416619();
    return;
}



// Function: cvs_addfn @ 0x16780
void cvs_addfn(long long a0, long long a1, unsigned int a2)
{
    unsigned long v3;  // rbx
    unsigned long v4;  // rbx
    char i;  // [bp-0x118]
    unsigned long v1;  // [bp-0x110]

    if (sub_4382c0(a1, &i, 2116))
        return;
    if (i)
    {
        v3 = 0;
        do
        {
            v4 = v3 + 1;
            sub_43ee30(a0, *((long long *)(v1 + v3 * 8)), a2);
            v3 = v4;
        } while (i > v3);
    }
    sub_435420(&i);
    return;
}



// Function: excfile_add @ 0x16820
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern struct_2 *g_484300;
extern unsigned long long g_484308;

struct_0 ** excfile_add(char *a0, unsigned int a1)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // rbx
    struct_0 **v3;  // rax

    v1 = sub_44c5b0(strlen(a0) + 16);
    v1->field_0 = 0;
    v2 = v1;
    v2->field_8 = a1;
    strcpy(v2 + 1, a0);
    <None|const 0> = g_484300;
    if (!v3)
    {
        g_484308 = v2;
        g_484300 = v2;
        return NULL;
    }
    *(v3) = v2;
    g_484300 = v2;
    return <None|const 0>;
}



// Function: info_attach_exclist @ 0x16890
typedef struct struct_1 {
    struct struct_1 *field_0;
    struct struct_1 *field_8;
    unsigned int field_10;
    char padding_14[4];
    long long field_18;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern struct_0 *g_484308;
extern struct_2 *g_484d50;
extern unsigned int g_484d58;

struct_1 * info_attach_exclist(struct_1 *idx)
{
    struct_0 *v3;  // rbx
    struct_1 *v4;  // r15
    int *err;  // rax
    unsigned long v5;  // rbp
    int v6;  // eax
    unsigned long fp;  // r14
    struct_0 *index;  // r13
    struct_1 *idx1;  // rax
    unsigned int v10;  // edx
    char *v11;  // rax
    int *err1;  // rax
    long long v0;  // [bp-0x48]
    struct_1 *v1;  // [bp-0x40]

    v1 = *((long long *)&idx[13].field_10);
    if (v1)
        return v1;
    v3 = g_484308;
    if (g_484308)
    {
        v4 = NULL;
        while (1)
        {
            v5 = v3 + 1;
            if (!faccessat((int)idx[13].field_8, v5, 0, 0))
            {
                v6 = sub_4140d0(idx, v5, 0);
                if (v6 != -0x1)
                {
                    fp = fdopen(v6, "r");
                    if (fp)
                    {
                        v0 = sub_43eba0();
                        index = sub_416480(v5);
                        if (index[2].field_0)
                            *((unsigned long long *)&index[2].field_8) = index[2].field_0(*((long long *)&index[2].field_8));
                        if (sub_43f0c0(*((long long *)((char *)&index[1].field_0 + 4)), v0, fp, 1342177281, 10))
                        {
                            err = __errno_location();
                            if (g_484d50)
                                g_484d50();
                            sub_44aa70(v5);
                            error(0, *(err), "%s");
                            sub_4163d0(); /* do not return */
                        }
                        fclose(fp);
                        idx1 = sub_44c5b0(32);
                        v10 = v3->field_8;
                        idx1->field_18 = v0;
                        if (v10)
                            v10 = index->field_8;
                        idx1->field_10 = v10;
                        idx1->field_8 = v4;
                        idx1->field_0 = 0;
                        if (v4)
                        {
                            v3 = v3->field_0;
                            v4->field_0 = idx1;
                            v4 = idx1;
                            if (!v3)
                                break;
                            else
                                continue;
                        }
                        else
                        {
                            v1 = idx1;
                            v4 = idx1;
                        }
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        v11 = dcgettext(NULL, "%s: fdopen failed", 5);
                        err1 = __errno_location();
                        error(0, *(err1), v11);
                        g_484d58 = 2;
                        close(v6);
                    }
                }
                else
                {
                    sub_4327c0(v5);
                }
            }
            v3 = v3->field_0;
            if (!v3)
                break;
        }
    }
    *((struct_1 **)&idx[13].field_10) = v1;
    return v1;
}



// Function: info_free_exclist @ 0x16ab0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[16];
    long long field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[432];
    struct struct_0 *field_1b0;
} struct_1;

void info_free_exclist(struct_1 *idx)
{
    struct_0 *v1;  // rbx
    struct_0 *v2;  // rbx
    struct_0 *ptr;  // rbp
    struct_0 *v4;  // rbx

    v1 = idx->field_1b0;
    if (idx->field_1b0)
    {
        do
        {
            v2 = v1;
            ptr = v2;
            v4 = v2->field_0;
            sub_43ebb0(ptr->field_18);
            free(ptr);
            v1 = v4;
        } while (v2->field_0);
    }
    idx->field_1b0 = NULL;
    return;
}



// Function: excluded_name @ 0x16b00
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    long long field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[432];
    struct struct_1 *field_1b0;
} struct_0;

extern long long g_484b60;

int excluded_name(char *a0, struct_0 *a1)
{
    struct_0 *v2;  // r14
    unsigned int v3;  // r12d
    unsigned long v4;  // rbp
    char *v5;  // r15
    struct_0 *idx;  // r14
    unsigned long long v7;  // rbx
    unsigned long long v8;  // rbx
    char v0;  // [bp-0x39]

    v2 = a1;
    v0 = sub_43ed70(g_484b60, a0);
    if (!v0 && v2)
    {
        v3 = 0;
        v4 = 0;
        v5 = NULL;
        do
        {
            idx = v2;
            v7 = idx->field_1b0;
            if (!idx->field_1b0)
                continue;
            do
            {
                v8 = v7;
                if (v3 & *((int *)(v8 + 16)))
                    continue;
                if (!(char)sub_43ed70(*((long long *)(v8 + 24)), a0))
                {
                    if (!v5)
                    {
                        v5 = a0;
                        if (*(a0) == 46)
                        {
                            do
                            {
                            } while (v5[1] == 47 && (v5 += 2, *(v5) == 46));
                        }
                    }
                    if ((char)sub_43ed70(*((long long *)(v8 + 24)), v5))
                        goto LABEL_416bd0;
                    if (v4)
                    {
                        if (!(char)sub_43ed70(*((long long *)(v8 + 24)), v4))
                            continue;
                        goto LABEL_416bd0;
                    }
                    v4 = sub_43e510(a0);
                    if ((char)sub_43ed70(*((long long *)(v8 + 24)), v4))
                        goto LABEL_416bd0;
                }
                else
                {
LABEL_416bd0:
                    v0 = 1;
                    goto LABEL_416bd5;
                }
            } while ((v7 = (unsigned long long)*((long long *)v8), *((long long *)v8)));
        } while ((v2 = (struct_0 *)*((long long *)&idx->padding_0[408]), v3 = 2, *((long long *)&idx->padding_0[408])));
LABEL_416bd5:
        free(v4);
    }
    return v0;
}



// Function: implemented @ 0x16c80
char implemented(unsigned int a0)
{
    return a0 != 38 & a0 != 95;
}



// Function: free_delayed_set_stat @ 0x16c90
typedef struct struct_0 {
    char padding_0[96];
    void* field_60;
    void* field_68;
    char padding_70[8];
    void* field_78;
    char padding_80[8];
    long long field_88;
    long long field_90;
    char padding_98[8];
    void* field_a0;
} struct_0;

void free_delayed_set_stat(struct_0 *a0)
{
    free(a0->field_a0);
    sub_41c600(a0->field_90, a0->field_88);
    free(a0->field_60);
    free(a0->field_68);
    free(a0->field_78);
    free(a0);
    return;
}



// Function: is_directory_link @ 0x16ce0
extern unsigned int g_481234;

unsigned long long is_directory_link(long long a0)
{
    unsigned int *err;  // rax
    unsigned long long v4;  // rax
    char v0;  // [bp-0xc8]
    unsigned int v1;  // [bp-0xb0]

    err = __errno_location();
    if (fstatat(g_481234, a0, &v0, 0x100) || ((unsigned short)v1 & 0xf000) != 0xa000 || fstatat(g_481234, a0, &v0, 0))
        v4 = 0;
    else
        v4 = _INSERT(v1 & 0xf000, 0, (v1 & 0xf000) == 0x4000);
    *(err) = *(err);
    return v4;
}



// Function: find_delayed_link_source @ 0x16da0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern unsigned int g_481234;
extern struct_0 *g_484318;

struct_0 * find_delayed_link_source(long long a0)
{
    struct_0 *v3;  // r12
    struct_0 *v4;  // r12
    char v0;  // [bp-0xb8]
    unsigned long v1;  // [bp-0xb0]

    v3 = g_484318;
    if (g_484318)
    {
        if (!fstatat(g_481234, a0, &v0, 0x100))
        {
            do
            {
                v4 = v3;
                if (v4->field_8 == v0 && v4->field_10 == v1)
                    return v4;
            } while ((v3 = (struct_0 *)v4->field_0, v4->field_0));
        }
        else if (*(__errno_location()) != 2)
        {
            sub_432d30(a0);
            return NULL;
        }
    }
    return NULL;
}



// Function: repair_delayed_set_stat @ 0x16e60
typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
    char padding_24[4];
    uint128_t field_28;
    uint128_t field_38;
    unsigned int field_48;
    unsigned int field_4c;
    char field_50;
    char padding_51[3];
    unsigned int field_54;
    char padding_58[72];
    long long field_a0;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern unsigned int g_481234;
extern struct_0 *g_484320;
extern unsigned long long g_484898;
extern unsigned long long g_4848a0;
extern unsigned int g_4848b0;
extern unsigned int g_4848b4;
extern unsigned int g_4848b8;
extern uint128_t g_484928;
extern uint128_t g_484938;
extern struct_2 *g_484d50;
extern unsigned int g_484d58;

void repair_delayed_set_stat(long long a0, struct_1 *a1)
{
    struct_0 *v4;  // rbx
    struct_0 *idx;  // rbx
    char v0;  // [bp-0xc8]
    unsigned long v1;  // [bp-0xc0]
    unsigned int v2;  // [bp-0xb0]

    v4 = g_484320;
    if (g_484320)
    {
        do
        {
            idx = v4;
            if (fstatat(g_481234, *((long long *)&idx->padding_58[64]), &v0, idx->field_4c))
            {
                sub_432d30(*((long long *)&idx->padding_58[64]));
                return;
            }
            if (v0 == a1->padding_0 && v1 == a1->field_8)
            {
                idx->field_8 = g_484898;
                idx->field_10 = g_4848a0;
                idx->field_18 = g_4848b0;
                idx->field_1c = g_4848b4;
                *((unsigned int *)((char *)&idx->field_38 + 12)) = 0xffffffff;
                idx->field_20 = g_4848b8;
                *((char *)&idx->field_48) = 0;
                *((unsigned int *)((char *)&idx->field_38 + 8)) = v2;
                *((uint128_t *)(idx->padding_24)[1]) = g_484928;
                *((uint128_t *)((char *)&idx->field_28 + 8)) = g_484938;
                return;
            }
            v4 = idx->field_0;
        } while (idx->field_0);
    }
    if (g_484d50)
        g_484d50();
    sub_44aa70(a0);
    error(0, 0, dcgettext(NULL, "%s: Unexpected inconsistency when making directory", 5));
    g_484d58 = 2;
    return;
}



// Function: check_time @ 0x16fd0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b81;
extern long long g_484a10;
extern long long g_484a18;
extern struct_0 *g_484d50;

void check_time(unsigned long a0, long long a1, long long a2)
{
    long long v4;  // r15
    long long v5;  // r14
    long long v0;  // [bp-0x68]
    long long v1;  // [bp-0x60]
    char v2;  // [bp-0x58]

    if (a1 >= 0)
    {
        if (sub_44be80(g_484a10, g_484a18, a1, a2) >= 0)
            return;
        sub_4411a0(&v0);
        if (sub_44be80(v0, v1, a1, a2) >= 0)
            return;
        v4 = a1 - v0;
        v5 = a2 - v1;
        if (a2 - v1 >> 63 & 1)
        {
            v5 += 1000000000;
            v4 -= 1;
        }
        if (!(g_483b81 & 128))
            return;
        if (g_484d50)
            g_484d50();
        sub_424270(v4, v5, &v2);
        sub_422080(a1, a2, 1);
        error(0, 0, dcgettext(NULL, "%s: time stamp %s is %s s in the future", 5));
        return;
    }
    else
    {
        if (!(g_483b81 & 128))
            return;
        if (g_484d50)
            g_484d50();
        sub_422080(a1, a2, 1);
        error(0, 0, dcgettext(NULL, "%s: implausibly old time stamp %s", 5));
        return;
    }
}



// Function: fd_chown @ 0x17180
extern unsigned int g_481234;

int fd_chown(int a0, long long a1, unsigned int a2, unsigned int a3, unsigned int a4)
{
    unsigned int v1;  // eax

    if (a0 >= 0)
    {
        v1 = fchown(a0, a2, a3);
        if (!v1)
        {
            return v1;
        }
        else if (sub_416c80(*(__errno_location())))
        {
            return v1;
        }
    }
    return fchownat(g_481234, a1, a2, a3, a4);
}



// Function: fd_stat @ 0x171f0
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

extern unsigned int g_481234;

int fd_stat(int a0, unsigned long a1, stat *a2)
{
    if (a0 < 0)
        return fstatat(g_481234);
    return fstat(a0, a2);
}



// Function: fd_i_chmod @ 0x17210
extern unsigned int g_481234;

int fd_i_chmod(int a0, long long a1, unsigned int a2, unsigned int a3)
{
    unsigned int v1;  // eax

    if (a0 >= 0)
    {
        v1 = fchmod(a0, a2);
        if (!v1)
        {
            return v1;
        }
        else if (sub_416c80(*(__errno_location())))
        {
            return v1;
        }
    }
    return fchmodat(g_481234, a1, a2, a3);
}



// Function: fd_chmod @ 0x17280
void fd_chmod(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4)
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
    sub_417298();
    return;
}



// Function: set_mode @ 0x17410
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

void set_mode(long long a0, unsigned int a1, unsigned int a2, unsigned int a3, unsigned int a4, unsigned int a5, char a6, unsigned int a7)
{
    unsigned int v2;  // r8d
    unsigned int v3;  // ebx
    unsigned int v4;  // ebp
    stat v0;  // [bp-0xd8]
    unsigned int v1;  // [bp-0xc0]

    if (!(a2 & (a4 ^ a1 | ~(a5))))
        return;
    if (~((unsigned short)a5 | (unsigned short)a2) & 0xfff)
    {
        a4 = v1;
        if (sub_4171f0(a3, a0, &v0))
        {
            sub_432d30(a0);
            return;
        }
    }
    v2 = a4 & 0xfff;
    v3 = a1 ^ v2;
    if (!(v3 & a2))
        return;
    v4 = v2 ^ v3 & a2;
    if (!(unsigned int)(unsigned long long)sub_417280(a3, a0, v4, a7, a6))
        return;
    sub_432550(a0, v4);
    return;
}



// Function: set_stat @ 0x17530
typedef struct struct_0 {
    char padding_0[112];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    uint128_t field_e8;
    uint128_t field_f8;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned int g_481234;
extern char g_483b82;
extern unsigned int g_484328;
extern char g_484a68;
extern void g_484aa0;
extern int g_484aa4;
extern int g_484aa8;
extern char g_484b50;
extern struct_1 *g_484d50;

long long set_stat(long long a0, struct_0 *idx, unsigned int a2, unsigned int a3, unsigned int a4, unsigned int v15, unsigned int a5, unsigned int a6)
{
    unsigned long long v7;  // r15
    unsigned long long v8;  // r14
    unsigned int v14;  // r10d
    unsigned int v16;  // r8d
    unsigned long long v9;  // r13
    unsigned long long v17;  // r12
    unsigned long long v18;  // rbx
    unsigned long v19;  // rax
    char *v20;  // rax
    unsigned int v10;  // r13d
    unsigned int v11;  // ebx
    unsigned long v12;  // fs
    unsigned int v13;  // edx
    unsigned int v0;  // [bp-0x6c]
    int v1;  // [bp-0x68]
    int v2;  // [bp-0x58]
    unsigned long v3;  // [bp-0x40]
    unsigned long long v4;  // [bp-0x18]
    unsigned long long v5;  // [bp-0x10]
    unsigned long long v6;  // [bp-0x8]

    v6 = v7;
    v5 = v8;
    v4 = v9;
    v10 = (char)v15;
    v11 = a4;
    v3 = *((long long *)(40 + v12));
    v14 = a5;
    v15 = a6;
    if (!g_484a68)
    {
        v13 = 0x1ff;
        if ((char)v14)
            goto LABEL_417588;
        v1 = (int)(!g_484b50 ? _INSERT(0, 8, 0x3ffffffe) : *((int128_t *)&(idx->padding_7c)[1]));
        v15 = v14;
        v14 = v15;
        v2 = (int)*((int128_t *)((char *)&idx->field_e8 + 8));
        v15 = v14;
        v14 = v15;
        if (!sub_43f360(a2, g_481234, a0, &v1))
        {
            if (g_484b50)
                sub_416fd0(a0, *((unsigned long long *)&v1), *((unsigned long long *)(&v1 + 8)));
            sub_416fd0(a0, *((unsigned long long *)&v2), *((unsigned long long *)(&v2 + 8)));
        }
        else
        {
            if ((char)v15 == 50)
            {
                v15 = v15;
                v14 = v14;
                v15 = v15;
                v15 = v15;
                v14 = v14;
                v15 = v15;
                if (!sub_416c80(*(__errno_location())))
                    goto LABEL_41769a;
            }
            sub_432dd0(a0);
        }
LABEL_41769a:
        if (g_484aa8 <= 0)
            goto LABEL_417738;
    }
    else if (g_484aa8 <= 0)
    {
LABEL_417738:
        if (g_484aa4 <= 0 || (char)v14)
        {
LABEL_41772b:
            v13 = 0x1ff;
            goto LABEL_417588;
        }
        else
        {
            v13 = 0xfff;
            goto LABEL_417588;
        }
    }
    else if (!(!(char)v14))
    {
        goto LABEL_41772b;
    }
    v0 = v15;
    v15 = idx->field_78;
    v14 = idx->field_74;
    v16 = v15;
    if (!sub_417180(a2, a0, idx->field_74, idx->field_78, v15))
    {
        v15 = v16;
        if (((char)~(v11) | (char)a3) & 73)
        {
            v11 &= ~(a3 & 0xc00);
            v15 = v16;
        }
    }
    else
    {
        v15 = v14;
        v14 = v15;
        if ((char)v0 == 50)
        {
            v15 = v16;
            v16 = v15;
            if (!sub_416c80(*(__errno_location())))
                goto LABEL_4176fb;
        }
        v15 = v16;
        sub_432600(a0, v15, v14);
    }
LABEL_4176fb:
    v13 = (g_484aa4 <= 0 ? 0x1ff : 0xfff);
LABEL_417588:
    sub_417410(a0, ~(g_484328) & idx->field_70, v13, a2, a3, v11, v10, v15);
    sub_431f20(idx, a0, v10, 1);
    sub_431a70(idx, a0, v10);
    if (v3 != *((long long *)(40 + v12)))
        __stack_chk_fail(); /* do not return */
    if (*((int *)&g_484aa0) <= 0 || !*((long long *)&idx->padding_0[48]))
        return *((int *)&g_484aa0);
    v6 = v4;
    v5 = v17;
    v4 = v18;
    v19 = ((char)v10 == 50 ? sub_44baa0(g_481234) : sub_44b880(g_481234));
    if ((unsigned int)v19 != 0xffffffff)
    {
        return v19;
    }
    else if (!(g_483b82 & 32))
    {
        return v19;
    }
    else
    {
        if (g_484d50)
            g_484d50();
        v20 = dcgettext(NULL, "%s: Cannot set SELinux context for file '%s'", 5);
        return (unsigned long long)error(0, *(__errno_location()), v20);
    }
    return *((int *)&g_484aa0);
}



// Function: apply_nonancestor_delayed_set_stat @ 0x17870
typedef struct struct_0 {
    char padding_0[112];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    uint128_t field_e8;
    uint128_t field_f8;
} struct_0;

extern unsigned int g_481234;
extern void* g_484320;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void apply_nonancestor_delayed_set_stat(void* a0, char a1)
{
    unsigned long len;  // rax
    void* idx;  // r15
    unsigned int v20;  // ebx
    unsigned int v21;  // eax
    unsigned int v22;  // r12d
    unsigned int v23;  // r13d
    unsigned long v24;  // rdx
    unsigned long v0;  // [bp-0x290]
    char v1;  // [bp-0x288]
    unsigned long v2;  // [bp-0x280]
    unsigned int v3;  // [bp-0x270]
    struct_0 v4;  // [bp-0x1f8]
    unsigned long v5;  // [bp-0x1c8]
    unsigned long v6;  // [bp-0x1c0]
    unsigned long v7;  // [bp-0x1b8]
    unsigned long v8;  // [bp-0x1b0]
    unsigned long v9;  // [bp-0x1a8]
    unsigned int v10;  // [bp-0x188]
    unsigned int v11;  // [bp-0x184]
    unsigned int v12;  // [bp-0x180]
    int v13;  // [bp-0x110]
    int v14;  // [bp-0x100]
    unsigned long v15;  // [bp-0xa0]
    unsigned long v16;  // [bp-0x98]

    len = strlen(a0);
    idx = g_484320;
    v0 = len;
    if (!g_484320)
        return;
    v20 = 0;
    while (1)
    {
        v21 = (char)idx[88];
        v22 = (int)idx[72];
        v23 = (int)idx[76];
        v20 |= v21;
        if ((char)v21 > a1)
            return;
        v24 = (long long)idx[152];
        if (v24 < v0 && *((char *)a0 + v24) && (*((char *)a0 + v24) == 47 || *((char *)a0 + v24 - 1) == 47) && !memcmp(a0, (long long)idx[160], v24))
            return;
        sub_424c80((int)idx[92]);
        if (!(char)v20)
        {
LABEL_4178c0:
            v10 = (int)idx[24];
            v11 = (int)idx[28];
            v13 = (int)(int128_t)idx[40];
            v12 = (int)idx[32];
            v14 = (int)(int128_t)idx[56];
            v5 = (long long)idx[96];
            v6 = (long long)idx[104];
            v7 = (long long)idx[112];
            v8 = (long long)idx[120];
            v9 = (long long)idx[128];
            v16 = (long long)idx[144];
            v15 = (long long)idx[0x88];
            sub_417530((long long)idx[160], &v4, 0xffffffff, v22, v23, 53, (char)idx[80], (int)idx[84]);
            goto LABEL_41798c;
        }
        else if (!fstatat(g_481234, (long long)idx[160], &v1, (int)idx[84]))
        {
            v22 = v3;
            if (v1 != (long long)idx[8] || v2 != (long long)idx[16])
            {
                if (g_484d50)
                    g_484d50();
                sub_44aa70((long long)idx[160]);
                error(0, 0, dcgettext(NULL, "%s: Directory renamed before its status could be extracted", 5));
                g_484d58 = 2;
                goto LABEL_41798c;
            }
            else
            {
                v23 = 0xffffffff;
                goto LABEL_4178c0;
            }
        }
        else
        {
            sub_432d30((long long)idx[160]);
LABEL_41798c:
            g_484320 = *((long long *)idx);
            sub_416c90(idx);
            idx = g_484320;
            if (!g_484320)
                return;
        }
    }
}



// Function: mark_after_links @ 0x17b10
void mark_after_links(unsigned long a0)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_417b15();
    return;
}



// Function: fixup_delayed_set_stat @ 0x17bb0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[84];
    unsigned int field_5c;
    char padding_60[56];
    char field_98;
    char padding_99[7];
    void* field_a0;
} struct_0;

extern struct_0 *g_484320;
extern char g_484580;

long long fixup_delayed_set_stat(char *a0, unsigned long a1)
{
    struct_0 *v1;  // rbx
    unsigned long v2;  // rax
    struct_0 *idx;  // rbx
    unsigned long len;  // rax

    v1 = g_484320;
    if (!g_484320)
        return v2;
    while (1)
    {
        idx = v1;
        if (idx->field_5c == *((int *)&g_484580))
        {
            v2 = strcmp(idx->field_a0, a0);
            if (!(unsigned int)v2)
            {
                free(idx->field_a0);
                idx->field_a0 = sub_44c7a0(a1);
                len = strlen(a1);
                *((unsigned long *)&idx->field_98) = len;
                return len;
            }
        }
        v1 = idx->field_0;
        if (!idx->field_0)
            return v2;
    }
}



// Function: delay_set_stat @ 0x17c30
typedef struct struct_0 {
    char padding_0[48];
    long long field_30;
    char padding_38[8];
    unsigned long long field_40;
    char padding_48[8];
    unsigned long long field_50;
    unsigned long long field_58;
    unsigned long long field_60;
    char padding_68[12];
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    uint128_t field_e8;
    uint128_t field_f8;
} struct_0;

extern unsigned int g_481234;
extern void* g_484320;
extern unsigned int g_484580;

void delay_set_stat(unsigned long a0, struct_0 *index, unsigned int a2, unsigned int a3, unsigned int a4, unsigned int a5)
{
    unsigned long len;  // rax
    void* v5;  // r15
    unsigned long long v14;  // rdi
    unsigned long v15;  // rax
    void* idx;  // r15
    void* v7;  // rax
    unsigned long long v8;  // rax
    long long v9;  // rsi
    uint128_t v10;  // xmm0
    uint128_t v11;  // xmm1
    unsigned int v12;  // eax
    unsigned long long v13;  // rax
    unsigned long v0;  // [bp-0xe0]
    char v1;  // [bp-0xd8]
    unsigned long v2;  // [bp-0xd0]

    len = strlen(a0);
    v5 = g_484320;
    v0 = len;
    if (g_484320)
    {
        while (1)
        {
            idx = v5;
            if (!strcmp((long long)idx[160], a0))
                break;
            v5 = *((long long *)idx);
            if (!*((long long *)idx))
                goto LABEL_417ef8;
        }
        if ((char)idx[80])
        {
            if (!fstatat(g_481234, (long long)idx[160], &v1, (int)idx[84]))
            {
                *((unsigned long long *)&idx[8]) = v1;
                *((unsigned long *)&idx[16]) = v2;
            }
            else
            {
                sub_432d30((long long)idx[160]);
            }
        }
        *((unsigned int *)&idx[24]) = a4;
        if (index)
            goto LABEL_417cc6;
    }
    else
    {
LABEL_417ef8:
        idx = sub_44c5b0(168);
        v7 = g_484320;
        g_484320 = idx;
        *((void* *)idx) = v7;
        *((unsigned long *)&idx[152]) = v0;
        v8 = sub_44c7a0(a0);
        *((char *)&idx[88]) = 0;
        *((unsigned long long *)&idx[160]) = v8;
        if (!index)
        {
            *((unsigned int *)&idx[24]) = a4;
        }
        else
        {
            *((unsigned long long *)&idx[8]) = index->field_58;
            *((unsigned long long *)&idx[16]) = index->field_60;
            *((unsigned int *)&idx[24]) = a4;
LABEL_417cc6:
            v9 = index->field_30;
            v10 = *((int128_t *)&(index->padding_7c)[1]);
            v11 = *((int128_t *)((char *)&index->field_e8 + 8));
            *((unsigned int *)&idx[28]) = index->field_74;
            v12 = index->field_78;
            *((unsigned int *)&idx[72]) = a2;
            *((unsigned int *)&idx[32]) = v12;
            *((char *)&idx[80]) = 0;
            *((unsigned int *)&idx[76]) = a3;
            *((unsigned int *)&idx[84]) = a5;
            *((unsigned int *)&idx[92]) = g_484580;
            *((unsigned long long *)&idx[96]) = 0;
            *((uint128_t *)&idx[40]) = v10;
            *((uint128_t *)&idx[56]) = v11;
            sub_423be0(idx + 96, v9);
            if (index->padding_38)
            {
                v13 = sub_44c770(index->padding_38, index->field_40 + 1);
                v14 = (unsigned long long)index->padding_48;
                *((unsigned long long *)&idx[104]) = v13;
                *((unsigned long long *)&idx[112]) = index->field_40;
                if (!v14)
                    goto LABEL_417dee;
LABEL_417d61:
                *((unsigned long long *)&idx[120]) = sub_44c770(v14, index->field_50 + 1);
                v15 = index->field_50;
            }
            else
            {
                v14 = (unsigned long long)index->padding_48;
                *((unsigned long long *)&idx[104]) = 0;
                *((unsigned long long *)&idx[112]) = 0;
                if (v14)
                    goto LABEL_417d61;
LABEL_417dee:
                *((unsigned long long *)&idx[120]) = 0;
                v15 = 0;
            }
            *((unsigned long *)&idx[128]) = v15;
            sub_41c840(index, idx + 144, idx + 0x88);
            if ((char)sub_4244e0(a0))
            {
                sub_417b10(idx);
                return;
            }
            return;
        }
    }
    *((unsigned int *)&idx[72]) = a2;
    *((char *)&idx[80]) = 1;
    *((unsigned int *)&idx[76]) = a3;
    *((unsigned int *)&idx[84]) = a5;
    *((unsigned int *)&idx[92]) = g_484580;
    memset(idx + 96, 0, 56);
    if ((char)sub_4244e0(a0))
    {
        sub_417b10(idx);
        return;
    }
    return;
}



// Function: make_directories @ 0x17f70
extern unsigned int g_481234;
extern unsigned int g_484328;
extern unsigned int g_48432c;
extern char g_484330;

unsigned int make_directories(void* a0, char *a1)
{
    void* iter;  // rbx
    char i;  // al
    unsigned int v4;  // r12d
    unsigned long v5;  // r15
    unsigned int *err;  // rax
    unsigned int v7;  // r15d
    char v0;  // [bp-0xd8]

    iter = a0;
    if (*((char *)a0))
    {
LABEL_417fb0:
        i = (char)iter[1];
        iter += 1;
        if (i)
        {
            do
            {
                if (a0 == iter || i != 47 || *((char *)iter - 1) == 47 || *((char *)iter - 1) == 46 && (iter == a0 + 1 || *((char *)iter - 2) == 47 || *((char *)iter - 2) == 46 && (iter == a0 + 2 || *((char *)iter - 3) == 47)))
                    goto LABEL_417fb0;
                *((char *)iter) = 0;
                v4 = ~(g_48432c) & 0x1ff;
                v5 = v4;
                if (!g_484330)
                    v5 |= 192;
                if (!mkdirat(g_481234, a0, v5 & 0xffffffff))
                {
                    sub_417c30(a0, NULL, ~(g_484328) & (unsigned int)v5, 0x1ff, v4, 0x100);
                    sub_422a00(a0, (iter & 0xffffffff) - a0, v4);
                    *(a1) = 1;
                    *((char *)iter) = 47;
                    goto LABEL_417fb0;
                }
                err = __errno_location();
                if (*(err) != 0x11)
                {
                    v7 = fstatat(g_481234, a0, &v0, 0);
                    if (v7)
                    {
                        *(err) = *(err);
                        sub_432760(a0);
                        *((char *)iter) = 47;
                        return v7;
                    }
                }
                *((char *)iter) = 47;
                i = (char)iter[1];
                iter += 1;
            } while (i);
        }
    }
    return 0;
}



// Function: open_output_file @ 0x18140
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
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern char g_483b80;
extern unsigned int g_484310;
extern unsigned int g_484328;
extern unsigned int g_484b34;
extern char g_484b69;
extern struct_0 *g_484d50;

unsigned int open_output_file(long long a0, unsigned int a1, unsigned int a2, unsigned int a3, unsigned int *a4, unsigned int *a5)
{
    unsigned int v2;  // r12d
    unsigned int v4;  // r12d
    unsigned int v6;  // eax
    unsigned int *err;  // rax
    stat v0;  // [bp-0xd8]

    v2 = 526785;
    if (g_484b34 == 2)
        v2 = (-(g_484b69 < 1) & 0x20000) + 527169;
    if (a3)
        v2 = _INSERT(v2, 0, (char)v2 & 127);
    if (a1 == 55 && !g_484310)
    {
        g_484310 = 1;
        if (g_483b80 & 8)
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "Extracting contiguous files as regular files", 5));
        }
    }
    v4 = openat(g_481234, a0, v2, a2);
    if (v4 < 0)
    {
        return v4;
    }
    else if (g_484b34 != 2)
    {
        *(a4) = ~(g_484328) & a2;
        *(a5) = 0x1ff;
        return v4;
    }
    else
    {
        if (!fstat(v4, &v0))
        {
            v6 = v0.st_nlink;
            if (((unsigned short)v6 & 0xf000) == 0x8000)
            {
                *(a4) = v6;
                *(a5) = 0xffffffff;
                return v4;
            }
            close(v4);
            *(__errno_location()) = 0x11;
        }
        else
        {
            err = __errno_location();
            close(v4);
            *(err) = *(err);
        }
        return 0xffffffff;
    }
}



// Function: find_direct_ancestor @ 0x182e0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[80];
    char field_58;
    char padding_59[63];
    char field_98;
    char padding_99[7];
    char *field_a0;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[80];
    char field_58;
    char padding_59[63];
    unsigned long long field_98;
    unsigned long long field_a0;
} struct_1;

extern struct_0 *g_484320;

struct_1 * find_direct_ancestor(unsigned long a0)
{
    struct_0 *v1;  // r12
    unsigned long v2;  // rbx

    v1 = g_484320;
    if (!g_484320)
        return g_484320;
    while (1)
    {
        if (!v1->field_58)
        {
            v2 = *((long long *)&v1->field_98);
            if (!strncmp(a0, v1->field_a0, v2) && *((char *)(a0 + v2)) == 47 && sub_43e390(a0) == a0 + v2 + 1)
                return v1;
        }
        v1 = v1->field_0;
        if (!v1)
            return v1;
    }
}



// Function: apply_delayed_links @ 0x18350
typedef struct struct_1 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    long long field_18;
    long long field_20;
    char field_28;
    char padding_29[3];
    unsigned int field_2c;
    unsigned int field_30;
    unsigned int field_34;
    uint128_t field_38;
    uint128_t field_48;
    unsigned int field_58;
    char padding_5c[4];
    struct struct_0 *field_60;
    unsigned long long field_68;
    unsigned long long field_70;
    unsigned long long field_78;
    unsigned long long field_80;
    unsigned long long field_88;
    unsigned long long field_90;
    unsigned long long field_98;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern unsigned int g_481234;
extern struct_1 *g_484318;

void apply_delayed_links(void)
{
    struct_1 *ptr;  // r12
    long long v17;  // r14
    struct_0 **v26;  // rbx
    struct_0 **ptr1;  // rbx
    struct_0 **v28;  // rbx
    struct_1 *v29;  // rbx
    unsigned long v18;  // r15
    struct_0 **v19;  // rbx
    struct_0 **v20;  // rbx
    long long v21;  // rbp
    long long v22;  // rax
    long long v23;  // rdx
    unsigned int v24;  // edi
    unsigned int v25;  // eax
    char v0;  // [bp-0x288]
    unsigned long v1;  // [bp-0x280]
    struct_0 v2;  // [bp-0x1f8]
    unsigned long v3;  // [bp-0x1c8]
    unsigned long v4;  // [bp-0x1c0]
    unsigned long v5;  // [bp-0x1b8]
    unsigned long v6;  // [bp-0x1b0]
    unsigned long v7;  // [bp-0x1a8]
    unsigned int v8;  // [bp-0x188]
    unsigned int v9;  // [bp-0x184]
    unsigned int v10;  // [bp-0x180]
    int v11;  // [bp-0x110]
    int v12;  // [bp-0x100]
    unsigned long v13;  // [bp-0xa0]
    unsigned long v14;  // [bp-0x98]

    ptr = g_484318;
    if (g_484318)
    {
        while (1)
        {
            v17 = 0;
            v18 = &ptr->field_98;
            sub_424c80(*((int *)((char *)&ptr->field_48 + 8)));
            v19 = *((long long *)&ptr->field_58);
            if (*((long long *)&ptr->field_58))
            {
                while (1)
                {
                    v20 = v19;
                    v21 = v20 + 1;
                    if (!fstatat(g_481234, v21, &v0, 0x100) && v0 == ptr->field_8 && v1 == ptr->field_10 && (v22 = (long long)(unsigned long long)sub_44bd30(&v0), !(unsigned int)(unsigned long long)sub_44be80((unsigned long long)sub_44bd30(&v0), v23, ptr->field_18, ptr->field_20)))
                    {
                        if (!unlinkat(g_481234, v21, 0))
                        {
                            v24 = g_481234;
                            if (!v17 || !(v25 = (unsigned int)(unsigned long long)linkat((unsigned long long)g_481234, v17, (unsigned long long)g_481234, v21, 0), v24 = g_481234, !(unsigned int)(unsigned long long)linkat((unsigned long long)g_481234, v17, (unsigned long long)g_481234, v21, 0)))
                            {
                                if (!ptr->field_28)
                                {
                                    if (linkat())
                                    {
                                        sub_4326e0(v18, v21);
                                        v19 = *(v20);
                                        if (!*(v20))
                                            break;
                                        else
                                            continue;
                                    }
                                }
                                else
                                {
                                    if (symlinkat(v18, v24, v21))
                                    {
                                        sub_432c90(v18, v21);
                                    }
                                    else
                                    {
                                        v8 = ptr->field_2c;
                                        v9 = ptr->field_30;
                                        v11 = (int)*((int128_t *)&(&ptr->field_34)[1]);
                                        v10 = ptr->field_34;
                                        v12 = (int)*((int128_t *)((char *)&ptr->field_38 + 8));
                                        v3 = ptr->field_60;
                                        v4 = ptr->field_68;
                                        v5 = ptr->field_70;
                                        v6 = ptr->field_78;
                                        v7 = ptr->field_80;
                                        v14 = ptr->field_90;
                                        v13 = ptr->field_88;
                                        sub_417530(v21, &v2, 0xffffffff, 0, 0, 50, 0, 0x100);
                                        v17 = v21;
                                    }
                                }
                            }
                        }
                        else
                        {
                            sub_432db0(v21);
                        }
                    }
                    v19 = *(v20);
                    if (!*(v20))
                        break;
                }
                v26 = *((long long *)&ptr->field_58);
                if (*((long long *)&ptr->field_58))
                {
                    do
                    {
                        ptr1 = v26;
                        v28 = *(ptr1);
                        free(ptr1);
                        v26 = v28;
                    } while (*(ptr1));
                }
            }
            sub_41c600(ptr->field_90, ptr->field_88);
            free(ptr->field_60);
            v29 = ptr->field_0;
            free(ptr);
            if (!ptr->field_0)
                break;
            ptr = v29;
        }
    }
    g_484318 = 0;
    return;
}



// Function: extract_finish @ 0x18640
extern void g_467708;

void extract_finish(void)
{
    sub_417870(&g_467708, 0);
    sub_418350();
    sub_417870(&g_467708, 1);
    return;
}



// Function: maybe_recoverable @ 0x18750
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b82;
extern unsigned int g_484b34;
extern char g_484b69;
extern struct_0 *g_484d50;

unsigned int maybe_recoverable(unsigned long a0, char a1, char *a2)
{
    int *err;  // rax
    int v3;  // ebp
    char v0;  // [bp-0xc8]

    if (*(a2))
        return 0;
    err = __errno_location();
    v3 = *(err);
    if (v3 == 0x11)
        goto LABEL_418812;
    if (v3 > 0x11)
    {
        if ((v3 == 40 || (v3 & 0xffffffbf) == 31) && a1 && g_484b34 == 2 && !g_484b69)
        {
            if (!strchr(a0, 47))
            {
LABEL_418850:
                if (sub_425490(a0, 0) > 0)
                {
LABEL_418809:
                    return 1;
                }
                goto LABEL_4187f5;
            }
            else if (!sub_4246f0(a0, &v0))
            {
LABEL_418812:
                if (g_484b34 == 5)
                {
                    if (!(g_483b82 & 16))
                        return 2;
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcgettext(NULL, "%s: skipping existing file", 5));
                    return 2;
                }
                if (g_484b34 <= 5)
                {
                    if (g_484b34 <= 2)
                        goto LABEL_418850;
                    if (g_484b34 == 4)
                        return 0;
                }
                else if (g_484b34 == 6 && !(char)sub_418670(a0))
                {
                    goto LABEL_418850;
                }
LABEL_4187f5:
                if (!sub_417f70(a0, a2) && *(a2))
                    goto LABEL_418809;
            }
        }
    }
    else
    {
        if (v3 == 2)
            goto LABEL_4187f5;
    }
    *(err) = v3;
    return 0;
}



// Function: extract_fifo @ 0x18900
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern unsigned int g_484328;
extern struct_0 g_484840;
extern unsigned int g_4848b0;
extern int g_484aa8;

unsigned int extract_fifo(long long a0, char a1)
{
    unsigned int v3;  // ebx
    unsigned int v4;  // ebx
    unsigned int v5;  // eax
    unsigned int v0;  // [bp-0x3c]
    char v1;  // [bp-0x31]

    v3 = g_4848b0;
    v1 = 0;
    if (g_484aa8 > 0)
        v3 = g_4848b0 & 0xffffffc0;
    v4 = v3 & 0x1ff;
    while (1)
    {
        v5 = mkfifoat(g_481234, a0, v4);
        if (!v5)
        {
            v0 = v5;
            sub_417530(a0, &g_484840.field_0, 0xffffffff, ~(g_484328) & v4, 0x1ff, a1, 0, 0x100);
            return v0;
        }
        switch ((unsigned int)sub_418750(a0, 0, &v1))
        {
        case 0:
            sub_432780(a0);
            return 0xffffffff;
        case 2:
            return 0;
        default:
            continue;
        }
    }
}



// Function: extract_node @ 0x189e0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern unsigned int g_484328;
extern struct_0 g_484840;
extern unsigned int g_4848b0;
extern long long g_4848c0;
extern int g_484aa8;

unsigned int extract_node(long long a0, char a1)
{
    unsigned int v3;  // ebx
    unsigned int v4;  // ebx
    unsigned int v5;  // eax
    unsigned int v0;  // [bp-0x3c]
    char v1;  // [bp-0x31]

    v3 = g_4848b0;
    v1 = 0;
    if (g_484aa8 > 0)
        v3 = g_4848b0 & 0xffffffc0;
    v4 = v3 & 0x61ff;
    while (1)
    {
        v5 = mknodat(g_481234, a0, v4, g_4848c0);
        if (!v5)
        {
            v0 = v5;
            sub_417530(a0, &g_484840.field_0, 0xffffffff, ~(g_484328) & v4, 0x1ff, a1, 0, 0x100);
            return v0;
        }
        switch ((unsigned int)sub_418750(a0, 0, &v1))
        {
        case 0:
            sub_4327a0(a0);
            return 0xffffffff;
        case 2:
            return 0;
        default:
            continue;
        }
    }
}



// Function: create_placeholder_file @ 0x18ad0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
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

typedef struct struct_1 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char field_28;
    char padding_29[3];
    unsigned int field_2c;
    unsigned long long field_30;
    uint128_t field_38;
    uint128_t field_48;
    unsigned int field_58;
    char padding_5c[4];
    unsigned long long field_60;
} struct_1;

extern unsigned int g_481234;
extern struct_1 *g_484318;
extern unsigned int g_484580;
extern long long g_484840;
extern char *g_484858;
extern long long g_484870;
extern unsigned int g_4848b0;
extern unsigned long long g_4848b4;
extern uint128_t g_484928;
extern uint128_t g_484938;

unsigned int create_placeholder_file(unsigned long a0, char a1, char *a2, struct_1 **a3)
{
    unsigned int v2;  // r14d
    unsigned int i;  // eax
    struct_1 *v12;  // rax
    unsigned int v5;  // r14d
    struct_0 *idx;  // rbx
    struct_1 *v7;  // rax
    struct_1 *v8;  // rax
    unsigned long long v9;  // rax
    unsigned long v10;  // rdx
    char *v11;  // rax
    stat v0;  // [bp-0xd8]

    do
    {
        v2 = openat(g_481234, a0, 193, 0);
        if (v2 < 0)
        {
            if (*(__errno_location()) == 0x11 && sub_416da0(a0))
                break;
        }
        else
        {
            if (!fstat(v2, &v0))
            {
                v5 = close(v2);
                if (v5)
                {
                    sub_432680(a0);
                    return 0xffffffff;
                }
                idx = sub_44c5b0(strlen(g_484858) + 161);
                if (a3)
                {
                    v7 = *(a3);
                    *(a3) = idx;
                    idx->field_0 = v7;
                }
                else
                {
                    v8 = g_484318;
                    g_484318 = idx;
                    idx->field_0 = v8;
                }
                idx->field_8 = *((unsigned long long *)&v0);
                idx->field_10 = v0.st_ino;
                v9 = sub_44bd30(&v0);
                *((char *)&idx[1].field_10) = a1;
                idx[1].field_0 = v9;
                idx[1].field_8 = v10;
                if (a1)
                {
                    *((unsigned int *)((char *)&idx[1].field_10 + 4)) = g_4848b0;
                    *((uint128_t *)&idx[2].field_8) = g_484928;
                    idx[2].field_0 = g_4848b4;
                    *((uint128_t *)&idx[3].field_0) = g_484938;
                }
                *((unsigned int *)&idx[3].field_10) = g_484580;
                v11 = sub_44c5b0(strlen(a0) + 9);
                idx[4].field_0 = v11;
                *((unsigned long long *)&v11) = 0;
                strcpy(v11 + 8, a0);
                idx[4].field_8 = 0;
                sub_423be0(&idx[4].field_8, g_484870);
                memset(&idx[4].field_10, 0, 32);
                sub_41c840(&g_484840, &idx[6].field_8, idx + 6);
                strcpy(&idx[6].field_10, g_484858);
                v12 = sub_4182e0(a0);
                if (v12)
                {
                    sub_417b10(v12);
                    return v5;
                }
            }
            else
            {
                sub_432d30(a0);
                close(v2);
                return 0xffffffff;
            }
        }
        if (!sub_418750(a0, 0, a2))
        {
            sub_4327c0(a0);
            return 0xffffffff;
        }
    } while (i != 2);
    return 0;
}



// Function: extract_link @ 0x18d40
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char field_28;
    char padding_29[3];
    unsigned int field_2c;
    unsigned long long field_30;
    uint128_t field_38;
    uint128_t field_48;
    unsigned int field_58;
    char padding_5c[4];
    unsigned long long field_60;
} struct_1;

extern unsigned int g_481234;
extern struct_1 *g_484318;
extern unsigned int g_484580;
extern void g_484858;
extern char g_484b50;
extern char g_484b92;

unsigned int extract_link(unsigned long a0)
{
    void* v6;  // rax
    unsigned int *err;  // rbx
    unsigned int v8;  // eax
    struct_1 *idx;  // rbx
    long long v10;  // rax
    long long v11;  // rdx
    struct_0 **v12;  // r12
    char v0;  // [bp-0x159]
    char v1;  // [bp-0x158]
    unsigned long v2;  // [bp-0x150]
    char v3;  // [bp-0xc8]
    unsigned long v4;  // [bp-0xc0]

    v0 = 0;
    if (!g_484b92 && (char)sub_427cc0(*((long long *)&g_484858)))
        return sub_418ad0(a0, 0, &v0, NULL);
    v6 = sub_416da0(*((long long *)&g_484858));
    if (v6)
        return sub_418ad0(a0, 0, &v0, v6);
    err = __errno_location();
    while (1)
    {
        if (linkat(g_481234, *((long long *)&g_484858), g_481234, a0, 0))
        {
            if (*(err) == 0x11 && !strcmp(*((long long *)&g_484858), a0) || !fstatat(g_481234, *((long long *)&g_484858), &v1, 0x100) && !fstatat(g_481234, a0, &v3, 0x100) && v1 == v3 && v2 == v4)
                goto LABEL_418e68;
            *(err) = *(err);
            v8 = sub_418750(a0, 0, &v0);
            if (v8 != 1)
            {
                if (v8 == 2)
                    return 0;
                if (g_484b50 && *(err) == 0x11)
                    return 0;
                sub_4326e0(*((long long *)&g_484858), a0);
                return 1;
            }
        }
        else
        {
            idx = g_484318;
            if (g_484318 && !fstatat(g_481234, *((long long *)&g_484858), &v1, 0x100))
            {
                do
                {
                    if (*((int *)((char *)&idx->field_48 + 8)) == g_484580 && idx->field_8 == v1 && idx->field_10 == v2 && (v10 = (long long)(unsigned long long)sub_44bd30(&v1), !(unsigned int)(unsigned long long)sub_44be80(idx->field_18, idx->field_20, (unsigned long long)sub_44bd30(&v1), v11)))
                    {
                        v12 = sub_44c5b0(strlen(a0) + 9);
                        strcpy(v12 + 1, a0);
                        *(v12) = *((long long *)&idx->field_58);
                        *((struct_0 ***)&idx->field_58) = v12;
                        break;
                    }
                } while ((idx = (struct_1 *)idx->field_0, idx));
            }
LABEL_418e68:
            break;
        }
    }
    return 0;
}



// Function: extract_symlink @ 0x18fc0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_484840;
extern struct_0 *g_484858;
extern char g_484b92;

unsigned int extract_symlink(long long a0)
{
    unsigned int v3;  // eax
    unsigned int v0;  // [bp-0x2c]
    char v1;  // [bp-0x21]

    v1 = 0;
    if (!g_484b92 && ((char)g_484858->field_0 == 47 || (char)sub_427cc0()))
        return sub_418ad0(a0, 1, &v1, NULL);
    while (1)
    {
        v3 = symlinkat();
        if (!v3)
        {
            v0 = v3;
            sub_417530(a0, &g_484840.field_0, 0xffffffff, 0, 0, 50, 0, 0x100);
            return v0;
        }
        switch ((unsigned int)sub_418750(a0, 0, &v1))
        {
        case 0:
            sub_432c90(g_484858, a0);
            return 0xffffffff;
        case 2:
            return 0;
        default:
            break;
        }
    }
}



// Function: extract_file @ 0x191d0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_484840;
extern unsigned int g_4848b0;
extern long long g_4848c8;
extern char g_484960;
extern char g_484a60;
extern char g_484a6a;
extern int g_484aa8;
extern unsigned int g_484b34;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int extract_file(long long a0, unsigned int a1)
{
    unsigned int v8;  // r14d
    unsigned int v9;  // eax
    unsigned int v10;  // r13d
    unsigned int v11;  // eax
    long long v12;  // rdi
    unsigned long v13;  // rbx
    long long v14;  // r13
    long long v15;  // r15
    unsigned int v17;  // r13d
    unsigned int v0;  // [bp-0x6c]
    char *v1;  // [bp-0x68]
    char *v2;  // [bp-0x60]
    char v3;  // [bp-0x51]
    unsigned int v4;  // [bp-0x50]
    unsigned int v5;  // [bp-0x4c]
    long long i;  // [bp-0x48], Other Possible Types: unsigned int

    v0 = a1;
    v3 = 0;
    v8 = g_4848b0 & 448;
    if (g_484aa8 <= 0)
        v8 = g_4848b0 & 0x1ff;
    v4 = 0;
    v5 = 0;
    if (!g_484a6a)
    {
        if (*((long long *)&g_484a60))
        {
            v9 = sub_42b640(a0, 0x66, &g_484840);
            if (v9 < 0)
            {
                sub_422bd0();
                return 0;
            }
        }
        else
        {
            i = 0;
            v10 = sub_4190d0(a0, 0, (char)v0, &i);
            if (v10)
                return 1;
            v1 = &v5;
            v2 = &v3;
            while (1)
            {
                if (sub_418140(a0, v0, v8, i, &v4, v1) >= 0)
                    break;
                v11 = sub_418750(a0, 1, v2);
                if (v11 != 1)
                {
                    v0 = v11;
                    sub_422bd0();
                    if (v0 != 2)
                    {
                        sub_4327c0(a0);
                        return 1;
                    }
                    return v10;
                }
            }
LABEL_419415:
        }
        sub_40ccb0(&g_484840);
        if (!g_484960)
        {
            v12 = g_4848c8;
            for (i = g_4848c8; v12 > 0; i = v12)
            {
                sub_40ccf0(v12);
                v13 = sub_40e130();
                if (v13)
                {
                    v14 = sub_40d2b0(v13);
                    if (v14 > i)
                        v14 = i;
                    *(__errno_location()) = 0;
                    v12 = i - v14;
                    v15 = sub_4249f0(v9, v13, v14);
                    sub_40d1f0(v13 + v14 - 1);
                    if (v14 != v15)
                    {
                        if (!*((long long *)&g_484a60))
                            sub_432e30(a0, v15, v14);
                        break;
                    }
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
                    g_484d58 = 2;
                    break;
                }
            }
        }
        else
        {
            sub_429ed0(v9, &g_484840, &i);
        }
        sub_422b10();
        sub_40ccd0();
        if (g_484a6a)
            return 0;
        if (*((long long *)&g_484a60))
        {
            v17 = close(v9);
            if (v17 < 0)
                goto LABEL_4194fa;
        }
        else
        {
            sub_417530(a0, &g_484840, v9, v4, v5, (char)v0, 0, (g_484b34 != 2) * 0x100);
            v17 = close(v9);
            if (v17 < 0)
            {
LABEL_4194fa:
                sub_432680(a0);
            }
        }
        if (!*((long long *)&g_484a60))
            return v17;
        sub_42b6f0();
        return v17;
    }
    v9 = 1;
    goto LABEL_419415;
}



// Function: prepare_to_extract @ 0x19540
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b81;
extern char g_483b82;
extern char g_4847e0;
extern long long g_484848;
extern char g_484850;
extern unsigned int g_4848b0;
extern char g_4849c8;
extern char g_484a60;
extern char g_484a6a;
extern char g_484aba;
extern unsigned int g_484b34;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int prepare_to_extract(long long a0, unsigned int a1, unsigned long long *a2)
{
    unsigned long long v1;  // rbx

    switch (a1)
    {
    case 0: case 48: case 55:
        if (!g_484850)
            goto LABEL_4195f9;
        v1 = sub_419850;
        break;
    case 49:
        v1 = sub_418d40;
        break;
    case 50:
        v1 = sub_418fc0;
        break;
    case 51:
        g_4848b0 = g_4848b0 | 0x2000;
        v1 = sub_4189e0;
        break;
    case 52:
        g_4848b0 = g_4848b0 | 0x6000;
        v1 = sub_4189e0;
        break;
    case 53: case 68:
        v1 = sub_419850;
        if (g_4849c8)
        {
            g_4847e0 = 1;
            break;
        }
        break;
    case 54:
        v1 = sub_418900;
        break;
    case 75: case 76:
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Unexpected long name header", 5));
        g_484d58 = 2;
        return 0;
    case 77:
        if (g_484d50)
            g_484d50();
        sub_44aa70(g_484848);
        error(0, 0, dcgettext(NULL, "%s: Cannot extract -- file is continued from another volume", 5));
        g_484d58 = 2;
        return 0;
    case 83:
LABEL_4195f9:
        if (g_484a6a)
        {
            *(a2) = sub_4191d0;
            return 1;
        }
        v1 = sub_4191d0;
        if (*((long long *)&g_484a60))
        {
            *(a2) = sub_4191d0;
            return 1;
        }
LABEL_419631:
        if (g_484b34 != 3)
        {
            if (g_484b34 != 6)
            {
                *(a2) = v1;
                return 1;
            }
            else if (!(char)sub_418670(a0, 0))
            {
                *(a2) = v1;
                return 1;
            }
            else if (g_483b81 & 8)
            {
                if (g_484d50)
                    g_484d50();
                sub_44ac50(a0);
                error(0, 0, dcgettext(NULL, "Current %s is newer or same age", 5));
                goto LABEL_41959a;
            }
        }
        else
        {
            if (sub_425490(a0, g_484aba))
            {
                *(a2) = v1;
                return 1;
            }
            else if (!(*(__errno_location()) & 0xfffffffd))
            {
                *(a2) = v1;
                return 1;
            }
            else
            {
                sub_432db0(a0);
                return 0;
            }
        }
    case 86:
LABEL_41959a:
        return 0;
    default:
        if (g_483b82 & 1)
        {
            if (g_484d50)
                g_484d50();
            sub_44aa70(a0);
            error(0, 0, dcgettext(NULL, "%s: Unknown file type '%c', extracted as normal file", 5));
        }
    }
    if (!g_484a6a)
    {
        if (!*((long long *)&g_484a60))
            goto LABEL_419631;
        if (v1 == sub_4191d0)
        {
            *(a2) = sub_4191d0;
            return 1;
        }
    }
    goto LABEL_41959a;
}



// Function: extract_dir @ 0x19850
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern unsigned int g_484328;
extern unsigned long long g_4847f0;
extern struct_0 g_484840;
extern unsigned int g_484898;
extern unsigned int g_4848b0;
extern char g_484ad1;
extern char g_484b30;
extern char g_484b34;
extern char g_484b50;

unsigned int extract_dir(long long a0, unsigned int a1)
{
    unsigned int v6;  // r15d
    unsigned int v7;  // r14d
    unsigned int *iter;  // rdi
    unsigned int v8;  // r13d
    unsigned int v9;  // r12d
    int *err;  // rbx
    unsigned int i;  // eax
    unsigned int v12;  // eax
    unsigned int v13;  // rsi
    unsigned int v14;  // r8
    unsigned long long v15;  // rcx
    char *j;  // [bp-0xf8]
    char v1[16];  // [bp-0xe9]
    char v2[16];  // [bp-0xd9]
    char v3;  // [bp-0xd8]
    unsigned int v4;  // [bp-0xc0]

    v2 = 0;
    if (g_484ad1 && !g_4847f0)
    {
        if (!fstatat(g_481234, ".", &v3, 0))
            g_4847f0 = v3;
        else
            sub_424ec0(".");
    }
    if (g_484b50)
    {
        sub_4206e0(a0);
    }
    else if (a1 == 0x44)
    {
        sub_422bd0();
    }
    v6 = 0;
    v7 = 0;
    v8 = sub_419820(g_4848b0);
    j = &v3;
    do
    {
        v9 = mkdirat(g_481234, a0, v8);
        if (!v9)
        {
            v7 = ~(g_484328) & v8;
            sub_417c30(a0, &g_484840.field_0, v7, 0x1ff, g_4848b0, 0x100);
            return v9;
        }
        err = __errno_location();
        if (*(err) == 0x11)
        {
            v1 = v2;
            if (v2)
            {
                if (!g_484b30)
                    goto LABEL_419915;
            }
            else
            {
                if (!g_484b30)
                {
                    if (*((int *)&g_484b34) > 2)
                        goto LABEL_419949;
LABEL_419915:
                    if (!sub_4246f0(a0, j))
                    {
                        v7 = v4;
                        if (((unsigned short)v7 & 0xf000) != 0x4000)
                        {
                            v6 = 0xffffffff;
                        }
                        else
                        {
                            if (v1)
                            {
                                sub_416e60(a0, j);
                                return 0;
                            }
                            v12 = *((int *)&g_484b34);
                            if (*((int *)&g_484b34) == 1)
                            {
                                v8 = sub_419820(v7, a0);
                                v9 = (unsigned long long)sub_417280(0xffffffff, v13, v8, 0x100, v14);
                                if (!v9)
                                {
                                    v15 = 36;
                                    for (iter = &g_484898; v15; j += 4)
                                    {
                                        v15 -= 1;
                                        *(iter) = *((int *)&j);
                                        iter += 1;
                                    }
                                    sub_417c30(a0, &g_484840.field_0, v7, 0x1ff, g_4848b0, 0x100);
                                    return v9;
                                }
                                sub_432550(a0, v8);
                                v12 = *((int *)&g_484b34);
                            }
                            v6 = 0xffffffff;
LABEL_41996f:
                            if (v12 & 0xfffffffd)
                                return v9;
                            sub_417c30(a0, &g_484840.field_0, v7, v6, g_4848b0, 0);
                            return v9;
                        }
                    }
                    *(err) = 0x11;
                    goto LABEL_419949;
                }
            }
            if ((char)sub_416ce0(a0))
                return 0;
            goto LABEL_419915;
        }
        else
        {
LABEL_419949:
            if (!sub_418750(a0, 0, v2) && *(err) != 0x11)
            {
                sub_432760(a0);
                return 1;
            }
        }
    } while (i == 1);
    v12 = *((int *)&g_484b34);
    goto LABEL_41996f;
}



// Function: extr_init @ 0x19b30
extern unsigned int g_484328;
extern unsigned int g_48432c;
extern char g_484330;
extern int g_484aa4;
extern unsigned int g_484aa8;

unsigned int extr_init(void)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    unsigned int v4;  // eax

    v1 = geteuid();
    g_484330 = !v1;
    v2 = !v1;
    g_484aa4 = g_484aa4 + v2;
    g_484aa8 = g_484aa8 + v2;
    v3 = umask(0);
    g_48432c = v3;
    if (g_484aa4 <= 0)
    {
        v4 = umask(v3);
        g_484328 = v3;
        return v4;
    }
    g_484328 = 0;
    return v3;
}



// Function: remove_delayed_set_stat @ 0x19b90
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[84];
    unsigned int field_5c;
    char padding_60[64];
    char *field_a0;
} struct_0;

extern struct_0 *g_484320;
extern char g_484580;

int remove_delayed_set_stat(char *a0)
{
    struct_0 *v1;  // rbp
    struct_0 *v2;  // r12
    struct_0 *v3;  // rbp
    struct_0 *v4;  // rbx
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    unsigned int v7;  // eax
    unsigned int v8;  // eax

    v1 = g_484320;
    if (!g_484320)
        return v8;
    v2 = NULL;
    while (1)
    {
        v3 = v1;
        v4 = v3->field_0;
        if (v3->field_5c == *((int *)&g_484580) && !strcmp(v3->field_a0, a0))
            break;
        if (!v4)
            return v7;
        v2 = v3;
        v1 = v4;
    }
    sub_416c90(v3);
    if (!v2)
    {
        g_484320 = v4;
        return v6;
    }
    v2->field_0 = v4;
    return v5;
}



// Function: extract_archive @ 0x19c10
typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

typedef struct struct_1 {
    long long field_0;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_4842f0;
extern struct_0 *g_484408;
extern unsigned int g_484580;
extern char g_4847e0;
extern long long g_484840;
extern struct_1 *g_484848;
extern unsigned int g_484a54;
extern char g_484b40;
extern char g_484b84;
extern char g_484b92;
extern struct_2 *g_484d50;
extern unsigned int g_484d58;

void extract_archive(void)
{
    int *err;  // rax
    char *v3;  // rax
    unsigned int v4;  // r12d
    unsigned long long *v0;  // [bp-0x28]

    g_4842f0 = sub_418640;
    sub_40d1f0(g_484408);
    if (!g_484b92 && (char)sub_427cc0(g_484840))
    {
        if (g_484d50)
            g_484d50();
        sub_44aa70(g_484840);
        error(0, 0, dcgettext(NULL, "%s: Member name contains '..'", 5));
        g_484d58 = 2;
        sub_422bd0();
        return;
    }
    if (!(char)g_484848->field_0)
    {
        sub_422bd0();
        return;
    }
    if (!g_484b40)
    {
        if (g_484a54)
            goto LABEL_419dd6;
LABEL_419c7b:
        if (!g_4847e0)
            goto LABEL_419dfd;
    }
    else
    {
        if (!sub_42ca20("extract"))
        {
            sub_422bd0();
            return;
        }
        if (!g_484a54)
            goto LABEL_419c7b;
LABEL_419dd6:
        sub_422990(&g_484840, g_484408, -0x1);
        if (!g_4847e0)
        {
LABEL_419dfd:
            sub_417870(g_484848, 0);
            sub_424c80(g_484580);
        }
    }
    if (g_484b84 && !(char)sub_424710(g_484848, 0))
    {
        err = __errno_location();
        if (g_484d50)
            g_484d50();
        sub_44aa70(g_484848);
        v3 = dcgettext(NULL, "%s: Was unable to backup this file", 5);
        error(0, *(err), v3);
        g_484d58 = 2;
        sub_422bd0();
        return;
    }
    v4 = 83;
    if (!(char)sub_429e10(&g_484840))
        v4 = *((char *)&g_484408[19].field_0 + 4);
    if (!(char)sub_419540(g_484848, v4, &v0))
    {
        sub_422bd0();
    }
    else if (!v0(g_484848, v4))
    {
        return;
    }
    if (!g_484b84)
        return;
    sub_4245d0();
    return;
}



// Function: rename_directory @ 0x19ec0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int rename_directory(unsigned long a0, unsigned long a1)
{
    unsigned int *err;  // rax
    unsigned int v3;  // r13d
    char v0;  // [bp-0x31]

    if (!renameat(g_481234, a0, g_481234, a1))
    {
        sub_417bb0(a0, a1);
        return 1;
    }
    err = __errno_location();
    v3 = *(err);
    if (v3 == 2 && !sub_417f70(a1, &v0))
    {
        if (!renameat(g_481234, a0, g_481234, a1))
            return 1;
        v3 = *(err);
    }
    if (g_484d50)
        g_484d50();
    sub_44ac40(1, a1);
    sub_44ac40(0, a0);
    error(0, v3, dcgettext(NULL, "Cannot rename %s to %s", 5));
    g_484d58 = 2;
    return 0;
}



// Function: xattr_decode_keyword @ 0x19ff0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

long long xattr_decode_keyword(struct_0 *node)
{
    struct_0 *iter;  // rdx
    unsigned long v2;  // rax

    iter = &node->field_1;
    while (1)
    {
        v2 = node->field_0;
        if ((char)v2 != 37)
        {
            *(&iter->field_0 - 1) = v2;
            if (!(char)v2)
                break;
            else
                goto LABEL_41a007;
        }
        if (node->field_1 != 0x33)
        {
            if (node->field_1 != 50 || node->field_2 != 53)
                goto LABEL_41a028;
            *(&iter->field_0 - 1) = 37;
            node += 1;
        }
        else if (node->field_2 != 0x44)
        {
LABEL_41a028:
            *(&iter->field_0 - 1) = 37;
LABEL_41a007:
            node = &node->field_1;
        }
        else
        {
            *(&iter->field_0 - 1) = 61;
            node += 1;
        }
        iter = &iter->field_1;
    }
    return v2;
}



// Function: dummy_coder @ 0x1a060
void dummy_coder(void)
{
    return;
}



// Function: dummy_decoder @ 0x1a070
void dummy_decoder(void)
{
    return;
}



// Function: xheader_protected_pattern_p @ 0x1a080
typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[3];
    char field_1c;
    char padding_1d[3];
    char *field_20;
} struct_0;

extern struct_0 g_47f200;

unsigned int xheader_protected_pattern_p(char *a0)
{
    char *v1;  // rsi
    struct_0 *v2;  // rbx
    struct_0 *v3;  // rbx

    v1 = "atime";
    v2 = &g_47f200.field_0;
    while (1)
    {
        v3 = v2;
        if (!v3->field_1c && v3->field_18 & 1 && !fnmatch(a0, v1, 0))
            return 1;
        v1 = v3->field_20;
        v2 = &v3->field_20;
        if (!v3->field_20)
            return 0;
    }
}



// Function: xheader_set_single_keyword @ 0x1a0f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;

void xheader_set_single_keyword(unsigned long a0)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Keyword %s is unknown or not yet implemented", 5));
    sub_42c970(2); /* do not return */
}



// Function: xheader_protected_keyword_p @ 0x1a140
typedef struct struct_0 {
    char padding_0[24];
    char field_18;
    char padding_19[3];
    char field_1c;
    char padding_1d[3];
    char *field_20;
} struct_0;

extern struct_0 g_47f200;

unsigned int xheader_protected_keyword_p(char *a0)
{
    char *v1;  // rdi
    struct_0 *v2;  // rbx
    struct_0 *v3;  // rbx

    v1 = "atime";
    v2 = &g_47f200.field_0;
    while (1)
    {
        v3 = v2;
        if (!v3->field_1c && v3->field_18 & 1 && !strcmp(v1, a0))
            return 1;
        v1 = v3->field_20;
        v2 = &v3->field_20;
        if (!v3->field_20)
            return 0;
    }
}



// Function: xheader_keyword_override_p @ 0x1a1a0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char *field_8;
} struct_0;

extern struct_0 *g_484380;

unsigned int xheader_keyword_override_p(char *a0)
{
    struct_0 *v1;  // rbx
    struct_0 *v2;  // rbx

    v1 = g_484380;
    if (!g_484380)
        return 0;
    while (1)
    {
        v2 = v1;
        if (!strcmp(v2->field_8, a0))
            return 1;
        v1 = v2->field_0;
        if (!v2->field_0)
            return 0;
    }
}



// Function: xheader_set_keyword_equal @ 0x1a320
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern long long g_484358;
extern long long g_484370;
extern struct_0 *g_484d50;

int xheader_set_keyword_equal(void* a0, void* a1)
{
    char v1;  // r12b
    void* iter;  // rbx
    unsigned long long *v3;  // rax
    unsigned long v4;  // rbx
    void* v5;  // rbp
    unsigned long long *v6;  // rax
    char *v8;  // rdx

    if (a1 != a0)
    {
        v1 = 1;
        iter = a1;
        if (*((char *)iter - 1) == 58)
        {
            iter = a1 - 1;
            v1 = 0;
        }
        if (a0 < iter)
        {
            v3 = __ctype_b_loc();
            do
            {
            } while (*((char *)(*(v3) + *((char *)iter) * 2 + 1)) & 32 && (iter -= 1, a0 != iter));
        }
        *((char *)iter) = 0;
        v4 = (char)a1[1];
        v5 = a1 + 1;
        if ((char)v4)
        {
            v6 = __ctype_b_loc();
            do
            {
            } while (*((char *)(*(v6) + v4 * 2 + 1)) & 32 && (v4 = (unsigned long)(unsigned long long)(char)(char)v5[1], v5 += 1, (char)v4));
        }
        if (strcmp(a0, "delete"))
        {
            if (!strcmp(a0, "exthdr.name"))
            {
                return sub_423be0(&g_484370, v5);
            }
            if (!strcmp(a0, "globexthdr.name"))
                return sub_423be0(&g_484358, v5);
            if (!(char)sub_41a140(a0))
            {
                if (!v1)
                    goto LABEL_41a47c;
            }
            else
            {
                if (g_484d50)
                    g_484d50();
                v8 = dcgettext(NULL, "Keyword %s cannot be overridden", 5);
            }
        }
        else if (!(char)sub_41a080(v5))
        {
LABEL_41a47c:
        }
        else
        {
            if (g_484d50)
                g_484d50();
            sub_44ac50(v5);
            v8 = dcgettext(NULL, "Pattern %s cannot be used", 5);
        }
    }
    else
    {
        if (g_484d50)
            g_484d50();
        sub_44ac50(a0);
        v8 = dcgettext(NULL, "Malformed pax option: %s", 5);
    }
    error(0, 0, v8);
    sub_42c970(2); /* do not return */
}



// Function: dumpdir_decoder @ 0x1a560
typedef struct struct_0 {
    char padding_0[400];
    void* field_190;
} struct_0;

void* dumpdir_decoder(struct_0 *a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    void* v2;  // rax
    char v0;  // [bp+0x0]

    v2 = sub_44c5b0(a3);
    a0->field_190 = v2;
    return memcpy(v2, &v0, a3);
}



// Function: xheader_init @ 0x1a5a0
void xheader_init(long long *a0)
{
    long long v1;  // rax

    if (!*(a0))
    {
        v1 = sub_44c5b0(88);
        *(a0) = v1;
        _obstack_begin(v1, 0, 0, sub_44c5b0, free);
    }
    return;
}



// Function: decode_string @ 0x1a5e0
void decode_string(void* *a0, long long a1)
{
    if (*(a0))
    {
        free(*(a0));
        *(a0) = NULL;
    }
    if (!(char)sub_431340(0, a1, a0))
        sub_423be0(a0, a1);
    return;
}



// Function: xattr_selinux_decoder @ 0x1a640
void xattr_selinux_decoder(void* *a0, unsigned long a1, long long a2)
{
    sub_41a5e0(a0 + 6, a2);
    return;
}



// Function: xattr_encode_keyword @ 0x1a6b0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_443325;
extern unsigned long long g_484338;
extern struct_0 *g_484340;

char * xattr_encode_keyword(char *a0)
{
    char *iter;  // r12
    char *v2;  // rdi
    char v3;  // bpl
    unsigned long long v4;  // rbx
    unsigned long long v5;  // r13
    void* v6;  // rax

    iter = a0;
    v2 = g_484340;
    if (v2)
    {
        *(v2) = 0;
    }
    else
    {
        g_484338 = 0x100;
        g_484340 = sub_44c5b0(0x100);
        v2 = g_484340;
    }
    v3 = *(iter);
    v4 = 0;
    if (!v3)
    {
        *(v2) = 0;
        return v2;
    }
    do
    {
        v5 = v4 + 2;
        if (v5 < g_484338)
        {
            v6 = &v2[v4];
            if (v3 == 37)
                goto LABEL_41a73f;
            goto LABEL_41a6fa;
        }
        else
        {
            v2 = sub_44c710();
            g_484340 = v2;
            v6 = &v2[v4];
            if (v3 != 37)
            {
LABEL_41a6fa:
                if (v3 != 61)
                {
                    *((char *)v6) = v3;
                }
                else
                {
                    *((unsigned int *)v6) = (unsigned int)&g_443325;
                    v4 = v5;
                }
            }
            else
            {
LABEL_41a73f:
                *((unsigned int *)v6) = 3486245;
                v4 = v5;
            }
        }
    } while ((v3 = iter[1], iter += 1, v4 += 1, v3));
    v2[v4] = 0;
    return v2;
}



// Function: x_obstack_grow @ 0x1a7c0
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
    unsigned long long field_20;
} struct_1;

void* x_obstack_grow(struct_0 *index, void* a1, unsigned long a2)
{
    unsigned long long *idx;  // r12
    unsigned long v2;  // rdi
    void* dst;  // rax

    idx = index->field_0;
    v2 = idx[3];
    if (a2 > idx[4] - idx[3])
    {
        _obstack_newchunk(idx, a2);
        v2 = idx[3];
    }
    dst = memcpy(v2, a1, a2);
    idx[3] = idx[3] + a2;
    index->field_8 = index->field_8 + a2;
    return dst;
}



// Function: x_obstack_1grow @ 0x1a830
typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

char * x_obstack_1grow(struct_2 *index, char a1)
{
    struct_0 *idx;  // rbp
    char *v2;  // rax

    idx = index->field_0;
    v2 = &idx->field_18->field_0;
    if (*((long long *)&idx[1].padding_0[0]) == idx->field_18)
    {
        _obstack_newchunk(idx, 1);
        v2 = &idx->field_18->field_0;
    }
    idx->field_18 = v2 + 1;
    *(v2) = a1;
    index->field_8 = index->field_8 + 1;
    return v2;
}



// Function: xheader_print @ 0x1a980
long long xheader_print(unsigned long a0, unsigned long a1, char *a2)
{
    strlen(a2);
}



// Function: code_string @ 0x1a9b0
void code_string(long long a0, unsigned int a1, unsigned int a2)
{
    void* ptr;  // [bp-0x28], Other Possible Types: char

    if (!(char)sub_431340(1, a0, &ptr))
    {
        ptr = sub_44c7a0(a0);
        sub_41a980(a2, a1, ptr);
        free(ptr);
        return;
    }
    sub_41a980(a2, a1, ptr);
    free(ptr);
    return;
}



// Function: xattr_selinux_coder @ 0x1aa40
typedef struct struct_0 {
    char padding_0[48];
    long long field_30;
} struct_0;

void xattr_selinux_coder(struct_0 *a0, unsigned int a1, unsigned int a2)
{
    sub_41a9b0(a0->field_30, a1, a2);
    return;
}



// Function: x_obstack_blank @ 0x1ab10
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
    unsigned long long field_20;
} struct_1;

long long x_obstack_blank(struct_0 *index, unsigned long a1)
{
    unsigned long long *idx;  // r12
    unsigned long v2;  // rax

    idx = index->field_0;
    v2 = idx[3];
    if (a1 > idx[4] - idx[3])
    {
        _obstack_newchunk(idx);
        v2 = idx[3];
    }
    idx[3] = v2 + a1;
    index->field_8 = index->field_8 + a1;
    return v2 + a1;
}



// Function: xheader_xattr__add @ 0x1ab50
long long xheader_xattr__add(long long *a0, unsigned long long *a1, long long a2, long long a3, long long a4)
{
    long long v1;  // rdi
    unsigned long v2;  // rax
    unsigned long long v3;  // rbx
    unsigned long long v4;  // rbx
    long long v5;  // rax
    unsigned long long v6;  // rax
    long long v7;  // rax

    v1 = *(a0);
    v2 = *(a1) + 1;
    *(a1) = v2;
    v3 = v2 * 24;
    v4 = v3 - 24;
    v5 = sub_44c610(v1, v3);
    *(a0) = v5;
    *((unsigned long long *)(v5 + v4)) = sub_44c7a0(a2);
    v6 = sub_44c770(a3, a4 + 1);
    *((unsigned long long *)(*(a0) + v4 + 8)) = v6;
    v7 = *(a0);
    *((long long *)(v7 + v4 + 16)) = a4;
    return v7;
}



// Function: xattr_acls_d_decoder @ 0x1abd0
void xattr_acls_d_decoder(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_41abe3();
    return;
}



// Function: xattr_acls_a_decoder @ 0x1ac00
void xattr_acls_a_decoder(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_41ac13();
    return;
}



// Function: locate_handler @ 0x1ac30
typedef struct struct_0 {
    char padding_0[28];
    char field_1c;
    char padding_1d[3];
    char *field_20;
} struct_0;

extern struct_0 g_47f200;

struct_0 * locate_handler(char *a0)
{
    struct_0 *v1;  // r12
    char *v2;  // rbp
    struct_0 *v3;  // r12

    v1 = &g_47f200.field_0;
    v2 = "atime";
    while (1)
    {
        v3 = v1;
        if (v3->field_1c)
        {
            if (!strncmp(v2, a0, strlen(v2)))
                return v3;
        }
        else
        {
            if (!strcmp(v2, a0))
                return v3;
        }
        v2 = v3->field_20;
        v1 = &v3->field_20;
        if (!v3->field_20)
            return NULL;
    }
}



// Function: run_override_list @ 0x1acb0
typedef struct struct_0 {
    char padding_0[28];
    char field_1c;
    char padding_1d[3];
    char *field_20;
} struct_0;

struct_0 * run_override_list(struct_0 *a0, unsigned long a1)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    struct_0 *v7;  // rbp
    struct_0 *v8;  // rax
    unsigned long len;  // rax
    struct_0 *v10;  // rax
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    if (!a0)
        return v10;
    v2 = v4;
    v1 = v5;
    v0 = v6;
    do
    {
        v7 = a0;
        v8 = sub_41ac30(*((long long *)&v7->padding_0[8]));
        if (v8)
        {
            len = strlen(*((long long *)&v7->padding_0[16]));
            v8 = (*((long long *)&v8->padding_0[16]))(a1, *((long long *)&v8->padding_0[0]), *((long long *)&v7->padding_0[16]), len);
        }
    } while ((a0 = (struct_0 *)*((long long *)&v7->padding_0[0]), *((long long *)&v7->padding_0[0])));
    return v8;
}



// Function: decg @ 0x1ad20
void decg(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v4;  // r14
    unsigned long long v5;  // r13
    unsigned long long v6;  // r12
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    sub_41ad3a();
    return;
}



// Function: out_of_range_header @ 0x1ad90
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void out_of_range_header(unsigned long a0, unsigned long a1, long long a2, long long a3)
{
    unsigned long long v4;  // rax
    unsigned long long v0;  // [bp-0x78]
    char v1;  // [bp-0x68]
    char v2;  // [bp-0x48]

    sub_442f50(a2, &v1);
    v4 = sub_443090(a3, &v2);
    if (g_484d50)
        g_484d50();
    v0 = v4;
    error(0, 0, dcgettext(NULL, "Extended header %s=%s is out of range %s..%s", 5));
    g_484d58 = 2;
    return;
}



// Function: decode_time @ 0x1ae40
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;
extern unsigned int g_484d58;

int decode_time(long long *a0, char *a1, unsigned int a2)
{
    long long v2;  // rax
    long long v3;  // rdx
    unsigned int v4;  // eax
    char v0;  // [bp-0x48]

    v2 = sub_4242f0(a1, &v0, 1);
    v4 = sub_423ba0(v2, v3);
    if ((char)v4)
    {
        *(a0) = v2;
        a0[1] = v3;
        return v4;
    }
    if (v0 > a1 && !*((char *)v0))
    {
        sub_41ad90(a2, a1, -0x8000000000000000, 0x7fffffffffffffff);
        return v4;
    }
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Malformed extended header: invalid %s=%s", 5));
    g_484d58 = 2;
    return v4;
}



// Function: mtime_decoder @ 0x1af40
typedef struct struct_0 {
    char padding_0[248];
    uint128_t field_f8;
} struct_0;

void mtime_decoder(struct_0 *a0, unsigned int a1, char *a2)
{
    long long v0[3];  // [bp-0x28]

    if ((char)sub_41ae40(v0, a2, a1))
        *((long long [3])&(a0->padding_0)[1]) = v0;
    return;
}



// Function: ctime_decoder @ 0x1afa0
typedef struct struct_0 {
    char padding_0[264];
    uint128_t field_108;
} struct_0;

void ctime_decoder(struct_0 *a0, unsigned int a1, char *a2)
{
    long long v0[3];  // [bp-0x28]

    if ((char)sub_41ae40(v0, a2, a1))
        *((long long [3])&(a0->padding_0)[1]) = v0;
    return;
}



// Function: atime_decoder @ 0x1b000
typedef struct struct_0 {
    char padding_0[232];
    uint128_t field_e8;
} struct_0;

void atime_decoder(struct_0 *a0, unsigned int a1, char *a2)
{
    long long v0[3];  // [bp-0x28]

    if ((char)sub_41ae40(v0, a2, a1))
        *((long long [3])&(a0->padding_0)[1]) = v0;
    return;
}



// Function: dumpdir_coder @ 0x1b060
void dumpdir_coder(unsigned long a0, unsigned long a1, unsigned long a2, long long a3)
{
    sub_41f4c0(a3);
}



// Function: sparse_map_decoder @ 0x1b090
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void sparse_map_decoder(unsigned long long *idx, unsigned int a1, char *a2)
{
    char *v3;  // r13
    unsigned int v4;  // r14d
    char *v14;  // rsi
    unsigned int *err;  // r15
    unsigned long long v6;  // r12
    unsigned long long v7;  // rax
    unsigned long long v8;  // rdx
    unsigned long long *v9;  // rdx
    char *v10;  // rax
    char v11;  // r8b
    char v0;  // [bp-0x49]
    char *v1;  // [bp-0x48]

    v3 = a2;
    idx[38] = 0;
    if (*(a2) - 48 <= 9)
    {
        v4 = 1;
        err = __errno_location();
        while (1)
        {
            *(err) = 0;
            v7 = strtoimax(v3, &v1, 10);
            if (v4)
            {
                if (*(err) != 0x22)
                {
                    v6 = v7;
                    goto LABEL_41b0fd;
                }
                else
                {
                    sub_41ad90(a1, v3, 0, 0x7fffffffffffffff);
                    return;
                }
            }
            if (*(err) == 0x22)
            {
                sub_41ad90(a1, v3, 0, 0x7fffffffffffffff);
                return;
            }
            v8 = idx[38];
            if (v8 >= idx[39])
                break;
            v9 = v8 * 16 + idx[40];
            idx[38] = v8 + 1;
            *(v9) = v6;
            v9[1] = v7;
LABEL_41b0fd:
            v10 = v1;
            v11 = *(v10);
            switch (v11)
            {
            case 0:
                if (!v4)
                    return;
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Malformed extended header: invalid %s: odd number of values", 5));
                g_484d58 = 2;
                return;
            case 44:
                v3 = v10 + 1;
                if (v10[1] - 48 > 9)
                    goto LABEL_41b190;
                v4 ^= 1;
                break;
            default:
                if (g_484d50)
                {
                    g_484d50();
                    v11 = *(v1);
                }
                v0 = v11;
                error(0, 0, dcgettext(NULL, "Malformed extended header: invalid %s: unexpected delimiter %c", 5));
                g_484d58 = 2;
                return;
            }
        }
        if (g_484d50)
            g_484d50();
        v14 = "Malformed extended header: excess %s=%s";
    }
    else
    {
LABEL_41b190:
        if (g_484d50)
            g_484d50();
        v14 = "Malformed extended header: invalid %s=%s";
    }
    error(0, 0, dcgettext(NULL, v14, 5));
    g_484d58 = 2;
    return;
}



// Function: sparse_path_decoder @ 0x1b310
typedef struct struct_0 {
    char padding_0[337];
    char field_151;
} struct_0;

void sparse_path_decoder(struct_0 *a0, unsigned long a1, unsigned long a2)
{
    a0->field_151 = 1;
}



// Function: mtime_coder @ 0x1b390
long long mtime_coder(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long long a3)
{
}



// Function: gid_decoder @ 0x1b530
typedef struct struct_0 {
    char padding_0[120];
    unsigned int field_78;
} struct_0;

void gid_decoder(struct_0 *a0, long long a1, long long a2)
{
    char v0;  // [bp-0x18]

    if ((char)sub_41b440(&v0, a2, 0xffffffff, a1))
        a0->field_78 = v0;
    return;
}



// Function: uid_decoder @ 0x1b590
typedef struct struct_0 {
    char padding_0[116];
    unsigned int field_74;
} struct_0;

void uid_decoder(struct_0 *a0, long long a1, long long a2)
{
    char v0;  // [bp-0x18]

    if ((char)sub_41b440(&v0, a2, 0xffffffff, a1))
        a0->field_74 = v0;
    return;
}



// Function: volume_offset_decoder @ 0x1b640
extern unsigned long long g_483f00;

void volume_offset_decoder(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
        g_483f00 = v0;
    return;
}



// Function: volume_size_decoder @ 0x1b6a0
extern unsigned long long g_483f08;

void volume_size_decoder(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
        g_483f08 = v0;
    return;
}



// Function: sparse_numbytes_decoder @ 0x1b700
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern struct_1 *g_484d50;
extern unsigned int g_484d58;

void sparse_numbytes_decoder(unsigned long long *idx, unsigned long a1, unsigned long a2)
{
    unsigned long long v2;  // rax
    struct_0 *v3;  // rax
    unsigned long long v0;  // [bp-0x28]

    if (!(char)sub_41b5f0(&v0))
        return;
    v2 = idx[38];
    if (v2 < idx[39])
    {
        v3 = v2 * 16 + idx[40];
        idx[38] = v2 + 1;
        v3->field_8 = v0;
        return;
    }
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Malformed extended header: excess %s=%s", 5));
    g_484d58 = 2;
    return;
}



// Function: sparse_offset_decoder @ 0x1b7e0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;
extern unsigned int g_484d58;

int sparse_offset_decoder(void)
{
    unsigned long long *idx;  // rdi
    unsigned long long v0;  // [bp-0x28]

    if (!(char)sub_41b5f0(&v0))
    {
        return;
    }
    else if (idx[38] < idx[39])
    {
        *((unsigned long long *)(idx[38] * 16 + idx[40])) = v0;
        return;
    }
    else
    {
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Malformed extended header: excess %s=%s", 5));
        g_484d58 = 2;
        return;
    }
}



// Function: sparse_size_decoder @ 0x1b8b0
typedef struct struct_0 {
    char padding_0[328];
    unsigned long long field_148;
    char field_150;
} struct_0;

void sparse_size_decoder(struct_0 *idx, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
    {
        idx->field_150 = 1;
        idx->field_148 = v0;
    }
    return;
}



// Function: sparse_minor_decoder @ 0x1b920
typedef struct struct_0 {
    char padding_0[296];
    unsigned int field_128;
} struct_0;

void sparse_minor_decoder(struct_0 *a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
        a0->field_128 = v0;
    return;
}



// Function: sparse_major_decoder @ 0x1b980
typedef struct struct_0 {
    char padding_0[292];
    unsigned int field_124;
} struct_0;

void sparse_major_decoder(struct_0 *a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
        a0->field_124 = v0;
    return;
}



// Function: size_decoder @ 0x1b9e0
typedef struct struct_0 {
    char padding_0[136];
    unsigned long long field_88;
} struct_0;

void size_decoder(struct_0 *a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
        a0->field_88 = v0;
    return;
}



// Function: sparse_numblocks_decoder @ 0x1ba40
typedef struct struct_0 {
    char padding_0[304];
    unsigned long long field_130;
    long long field_138;
    unsigned long long field_140;
} struct_0;

void sparse_numblocks_decoder(struct_0 *idx, unsigned long a1, unsigned long a2)
{
    unsigned long long v2;  // rax
    long long v0;  // [bp-0x18]

    if ((char)sub_41b5f0(&v0))
    {
        idx->field_138 = v0;
        v2 = sub_44c720(v0, 16);
        idx->field_130 = 0;
        idx->field_140 = v2;
        return;
    }
    return;
}



// Function: gid_coder @ 0x1bb20
void gid_coder(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: uid_coder @ 0x1bb40
void uid_coder(unsigned long a0, unsigned long a1, unsigned long a2)
{
}



// Function: xheader_keyword_deleted_p @ 0x1bef0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char *field_8;
} struct_0;

extern struct_0 *g_484390;

unsigned int xheader_keyword_deleted_p(char *a0)
{
    struct_0 *v1;  // rbx
    struct_0 *v2;  // rbx

    v1 = g_484390;
    if (!g_484390)
        return 0;
    while (1)
    {
        v2 = v1;
        if (!fnmatch(v2->field_8, a0, 0))
            return 1;
        v1 = v2->field_0;
        if (!v2->field_0)
            return 0;
    }
}



// Function: decx @ 0x1bf50
void decx(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long long v4;  // r14
    unsigned long long v5;  // r13
    unsigned long long v6;  // r12
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    sub_41bf6a();
    return;
}



// Function: xheader_set_option @ 0x1c010
void xheader_set_option(void)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_41c020();
    return;
}



// Function: xheader_format_name @ 0x1c080
extern char g_484b92;

void* xheader_format_name(long long *a0, char *a1, long long a2)
{
    char *node;  // r13
    char v9;  // al
    char *v17;  // rax
    char *iter;  // r10
    long long v19;  // rdi
    void* v20;  // r12
    char v21;  // al
    void* v22;  // rbx
    char *v10;  // r15
    void* iter1;  // rbx
    char v24;  // dl
    char v25;  // al
    char *v11;  // r14
    unsigned long ptr;  // rbp
    long long iter2;  // r12
    char *idx;  // rbx
    char v15;  // dl
    char *v16;  // rax
    char *v0;  // [bp-0xa8]
    char *v1;  // [bp-0xa0]
    char *v2;  // [bp-0x90]
    char v3;  // [bp-0x78]
    char v4;  // [bp-0x64]
    char v5;  // [bp-0x58]
    char v6;  // [bp-0x44]

    node = a1;
    v9 = *(a1);
    if (*(a1))
    {
        v10 = NULL;
        v0 = NULL;
        v11 = NULL;
        ptr = 0;
        iter2 = 0;
        v1 = NULL;
        v2 = &v5;
        while (1)
        {
            idx = a1;
            v15 = idx[1];
            iter = idx + 1;
            if (v9 != 37 || !v15)
            {
                iter2 += 1;
            }
            else if (v15 != 0x66)
            {
                if (v15 <= 0x66)
                {
                    if (v15 == 37)
                    {
                        v15 = idx[2];
                        iter2 += 1;
                        iter = idx + 2;
                    }
                    else if (v15 == 100 && a0)
                    {
                        if (!ptr)
                        {
                            ptr = sub_43e4f0(*(a0));
                            a0 = a0;
                        }
                        v11 = sub_433020(ptr, 0, g_484b92);
                        iter2 += strlen(v11);
                        v9 = idx[2];
                        a1 = iter + 1;
                        if (!idx[2])
                            break;
                        else
                            continue;
                    }
                }
                else
                {
                    if (v15 != 110)
                    {
                        if (v15 == 112)
                        {
                            a0 = a0;
                            v16 = sub_443090((int)getpid(), &v3);
                            v0 = v16;
                            v17 = &v4;
                            goto LABEL_41c295;
                        }
                        v15 = idx[2];
                        iter2 += 2;
                        iter += 1;
                    }
                    else
                    {
                        v16 = sub_443090(a2, v2);
                        v1 = v16;
                        v17 = &v6;
LABEL_41c295:
                        iter = idx + 2;
                        iter2 += v17 - v16;
                        v15 = idx[2];
                    }
                }
            }
            else
            {
                if (a0)
                {
                    v10 = sub_43e390(*(a0));
                    iter = idx + 2;
                    iter2 += strlen(v10);
                    v15 = idx[2];
                }
                else
                {
                    v15 = idx[2];
                    iter = idx + 2;
                }
            }
            a1 = iter;
            v9 = v15;
            if (!v9)
                break;
        }
        v19 = iter2 + 1;
    }
    else
    {
        v0 = NULL;
        v10 = NULL;
        v11 = NULL;
        ptr = 0;
        v1 = NULL;
        v19 = 1;
    }
    v20 = sub_44c5b0(v19);
    v21 = *(node);
    if (v21)
    {
        v22 = v20;
        iter1 = v22;
        while (1)
        {
            v24 = node[1];
            if (v21 != 37)
            {
                *((char *)iter1) = v21;
                node += 1;
                v25 = v24;
                iter1 += 1;
                v21 = v25;
                goto LABEL_41c1bc;
            }
            if (v25 == 0x66)
            {
                if (!v10)
                {
LABEL_41c1fd:
                    v21 = node[2];
                    node += 2;
                    goto LABEL_41c206;
                }
                else
                {
                    iter1 = stpcpy(iter1, v10);
                    goto LABEL_41c2cd;
                }
            }
            if (v25 <= 0x66)
            {
                if (v25 == 37)
                {
                    *((char *)iter1) = 37;
                    v21 = node[2];
                    iter1 += 1;
                    node += 2;
                    goto LABEL_41c1bc;
                }
                else if (v25 != 100)
                {
                    *((char *)iter1) = 37;
                    if (!v25)
                    {
                        iter1 += 1;
                        break;
                    }
                }
                else if (v11)
                {
                    iter1 = stpcpy(iter1, v11);
                    goto LABEL_41c1fd;
                }
            }
            else
            {
                if (v25 != 110)
                {
                    if (v25 == 112)
                    {
                        iter1 = stpcpy(iter1, v0);
                        goto LABEL_41c2cd;
                    }
                    else
                    {
                        *((char *)iter1) = 37;
                        *((char *)&iter1[1]) = v25;
                        v21 = node[2];
                        iter1 += 2;
                        node += 2;
                    }
LABEL_41c1bc:
                    if (!v21)
                        break;
                }
                else
                {
                    iter1 = stpcpy(iter1, v1);
LABEL_41c2cd:
                    v21 = node[2];
                    node += 2;
LABEL_41c206:
                    if (!v21)
                        break;
                }
            }
        }
        free(ptr);
        if (v22 < iter1)
        {
            do
            {
            } while (*((char *)iter1 - 1) == 47 && (iter1 -= 1, v22 != iter1));
        }
    }
    else
    {
        free(ptr);
        iter1 = v22;
    }
    *((char *)iter1) = 0;
    return v22;
}



// Function: xheader_xhdr_name @ 0x1c460
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_47f5e0;
extern char *g_484370;
extern char g_484a38;

void* xheader_xhdr_name(long long *a0)
{
    if (!g_484370)
        sub_423be0(&g_484370, (&g_47f5e0.field_0)[g_484a38], g_484a38);
    return sub_41c080(a0, g_484370, 0);
}



// Function: xheader_ghdr_name @ 0x1c4b0
extern char g_47f5e0;
extern void* g_484358;
extern unsigned long long g_484398;
extern char g_484a38;

void* xheader_ghdr_name(void)
{
    void* dst;  // r8
    unsigned long v2;  // r12
    char *v3;  // rax
    char *v4;  // rax
    unsigned long len;  // rax

    dst = g_484358;
    if (!dst)
    {
        v2 = *((long long *)&(&g_47f5e0)[16 + 8 * g_484a38]);
        v3 = getenv("TMPDIR");
        v4 = "/tmp";
        if (!v3)
            v3 = v4;
        len = strlen(v3);
        g_484358 = sub_44c5b0(len + strlen(v2) + 1);
        dst = strcat(strcpy(g_484358, v3), v2);
    }
    return sub_41c080(NULL, dst, g_484398 + 1);
}



// Function: xheader_forbid_global @ 0x1c560
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_484388;
extern struct_0 *g_484d50;

void xheader_forbid_global(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!g_484388)
        return;
    v0 = v2;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "can't update global extended header record", 5));
    sub_42c970(2); /* do not return */
}



// Function: xheader_xattr_init @ 0x1c5b0
void xheader_xattr_init(unsigned long long *idx)
{
    idx[44] = 0;
    idx[43] = 0;
    memset(idx + 6, 0, 40);
    return;
}



// Function: xheader_xattr_free @ 0x1c600
void xheader_xattr_free(void* *ptr, unsigned long long i)
{
    unsigned long long v1;  // rbp
    void* *v2;  // rbx
    void* *v3;  // rbx

    if (!i)
    {
        free(ptr);
        return;
    }
    v1 = 0;
    v2 = ptr;
    do
    {
        v1 += 1;
        v3 = v2 + 3;
        free(*(v2));
        free(*((long long *)((char *)v3 - 16)));
        v2 = v3;
    } while (i != v1);
    free(ptr);
    return;
}



// Function: xheader_xattr_add @ 0x1c650
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char field_c;
} struct_0;

void xheader_xattr_add(void* a0, char *ptr, long long a2, long long a3)
{
    unsigned long len;  // rax
    struct_0 *ptr1;  // rbp

    len = strlen(ptr);
    ptr1 = sub_44c5b0(len + strlen("SCHILY.xattr.") + 1);
    ptr1->field_0 = 8659957317661901651;
    ptr1->field_8 = 1920234593;
    ptr1->field_c = 46;
    strcpy(ptr1 + 1, ptr);
    sub_41ab50(a0 + 352, a0 + 344, ptr1, a2, a3);
    free(ptr1);
    return;
}



// Function: xattr_decoder @ 0x1c6e0
void xattr_decoder(void* a0, void* ptr, void* a2, unsigned long a3)
{
    unsigned long len;  // rax
    unsigned long v5;  // rax
    char *i;  // rcx
    unsigned long long v7;  // rdx
    unsigned long long v8;  // rbx
    unsigned long v9;  // rax
    unsigned long long j;  // rcx
    unsigned long long v11;  // rdx
    unsigned long long v12;  // r15
    char v0;  // [bp-0x2048]
    char v1;  // [bp-0x1048]
    char v2;  // [bp-0x48]

    len = strlen(ptr);
    v5 = len + 24;
    i = &(&v2)[-1 * (v5 & 0xfffffffffffff000)];
    if (&v2 != i)
    {
        do
        { } while (&v0 != i);
    }
    v7 = (unsigned int)v5 & 0xfffffff0 & 0xfff;
    /* unsupported instruction */ = (int)&(&v1)[-1 * v7];
    if (v7)
        *((long long *)(/* unsupported instruction */ + v7 - 8)) = *((long long *)(/* unsupported instruction */ + v7 - 8));
    v8 = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    memcpy(v8, ptr, len + 1);
    v9 = a3 + 24;
    j = /* unsupported instruction */ - (v9 & 0xfffffffffffff000);
    if (/* unsupported instruction */ != j)
    {
        do
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
            *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8));
        } while (/* unsupported instruction */ != j);
    }
    v11 = (unsigned int)v9 & 0xfffffff0 & 0xfff;
    /* unsupported instruction */ = /* unsupported instruction */ - v11;
    if (v11)
        *((long long *)(/* unsupported instruction */ + v11 - 8)) = *((long long *)(/* unsupported instruction */ + v11 - 8));
    v12 = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
    memcpy(v12, a2, a3 + 1);
    sub_419ff0(v8);
    sub_41c650(a0, v8 + strlen("SCHILY.xattr."), v12, a3);
    return;
}



// Function: xheader_xattr_copy @ 0x1c840
long long xheader_xattr_copy(unsigned long long *a0, unsigned long long *a1, unsigned long long *a2)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    unsigned long long i;  // rbx
    unsigned long long v8;  // rbx
    long long *v9;  // rax
    long long v10;  // rax
    long long v11;  // rax
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    *(a1) = 0;
    *(a2) = 0;
    if (!a0[43])
        return v11;
    v2 = v4;
    v1 = v5;
    v0 = v6;
    i = 0;
    do
    {
        v8 = i + 1;
        v9 = a0[44] + i * 24;
        v10 = sub_41ab50(a1, a2, *(v9), v9[1], v9[2]);
        i = v8;
    } while (a0[43] > i);
    return v10;
}



// Function: xheader_decode @ 0x1c8c0
typedef struct struct_0 {
    char padding_0[136];
    unsigned long long field_88;
    char padding_90[136];
    unsigned long long field_118;
    char padding_120[40];
    unsigned long long field_148;
    char field_150;
    char padding_151[31];
    unsigned long long field_170;
    unsigned long long field_178;
} struct_0;

extern struct_0 *g_484378;
extern struct_0 *g_484380;
extern struct_0 *g_484388;

void xheader_decode(struct_0 *idx)
{
    unsigned long v2;  // 4119
    unsigned long v0;  // [bp-0x28]

    sub_41acb0(g_484388, idx);
    sub_41acb0(g_484378, idx);
    if (idx->field_170)
    {
        v0 = idx->field_178 + 0x200;
        while ((char)sub_41bc70())
        ;
    }
    sub_41acb0(g_484380, idx);
    v2 = idx->field_150;
    idx->field_118 = idx->field_88;
    if ((char)v2)
        idx->field_88 = idx->field_148;
    return;
}



// Function: xheader_decode_global @ 0x1c9a0
extern long long g_484378;

void xheader_decode_global(long long *a0)
{
    unsigned long v0;  // [bp-0x38]

    if (!a0[1])
        return;
    v0 = a0[2] + 0x200;
    sub_41bc20();
    do
    { } while ((char)sub_41bc70(a0[1], a0[2], &v0, sub_41ad20, &g_484378));
    return;
}



// Function: xheader_store @ 0x1ca30
typedef struct struct_0 {
    char padding_0[28];
    char field_1c;
    char padding_1d[3];
    char *field_20;
} struct_0;

struct_0 * xheader_store(char *a0, struct_0 *a1, unsigned long a2)
{
    struct_0 *v5;  // rax
    unsigned long long v6;  // r14
    unsigned long long v7;  // r13
    unsigned long long v8;  // r12
    unsigned long long v9;  // rbx
    struct_0 *v10;  // rax
    struct_0 *v11;  // rax
    struct_0 *v12;  // rax
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]
    unsigned long long v3;  // [bp-0x8]

    if (*((long long *)&a1[9].padding_0[16]))
        return v5;
    v3 = v6;
    v2 = v7;
    v1 = v8;
    v0 = v9;
    v10 = sub_41ac30(a0);
    if (!v10)
    {
        return v10;
    }
    else if (*((long long *)&v10->padding_0[8]))
    {
        v11 = sub_41bef0(a0);
        if (*((char *)&v11))
            return v11;
        sub_41a5a0(a1 + 9);
        v12 = sub_41a1a0(a0);
        if (*((char *)&v12))
            return v12;
        goto *((void *)(*((long long *)&v10->padding_0[8])));
    }
    else
    {
        return v10;
    }
}



// Function: xheader_read @ 0x1cac0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;

void* xheader_read(void* idx, void* a1, unsigned long a2)
{
    unsigned long v4;  // rax
    unsigned long long v5;  // r13
    unsigned long long *iter;  // rdi
    void* v15;  // rcx
    void* i;  // rsi
    unsigned long long v17;  // rcx
    unsigned long v18;  // d
    unsigned long long v6;  // r14
    unsigned long long v7;  // r12
    void* v8;  // r12
    unsigned long v9;  // r13
    unsigned long long v10;  // rbx
    unsigned long long v11;  // rax
    unsigned long v12;  // rbp
    void* v13;  // rcx
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v4 = 0;
    if (a2 < NULL)
        a2 = v4;
    v2 = v5;
    v6 = 0;
    v1 = v7;
    v8 = a1;
    v9 = a2 + 0x200;
    v0 = v10;
    *((unsigned long *)&idx[8]) = v9;
    v11 = sub_44c5b0();
    *((unsigned long long *)&idx[16]) = v11;
    *((char *)(v11 + a2 + 0x200)) = 0;
    while (1)
    {
        v12 = 0x200;
        if (v9 <= 0x200)
            v12 = v9;
        if (!v8)
            break;
        v13 = (long long)idx[16] + v6;
        if ((unsigned int)v12 >= 8)
        {
            iter = v13 + 8 & 0xfffffffffffffff8;
            *((long long *)v13) = *((long long *)v8);
            *((long long *)((char *)v13 + (v12 & 0xffffffff) - 8)) = *((long long *)((char *)v8 + (v12 & 0xffffffff) - 8));
            v15 = v13 - iter;
            i = v8 - v15;
            for (v17 = (v15 + v12 & 0xffffffff) >> 3 & 0xffffffff; v17; i += v18 * 8)
            {
                v17 -= 1;
                *(iter) = *((long long *)i);
                iter = &iter[v18];
            }
        }
        else if ((char)v12 & 4)
        {
            *((int *)v13) = *((int *)v8);
            *((int *)((char *)v13 + (v12 & 0xffffffff) - 4)) = *((int *)((char *)v8 + (v12 & 0xffffffff) - 4));
        }
        else if ((unsigned int)v12)
        {
            *((char *)v13) = *((char *)v8);
            if ((char)v12 & 2)
                *((short *)((char *)v13 + (v12 & 0xffffffff) - 2)) = *((short *)((char *)v8 + (v12 & 0xffffffff) - 2));
        }
        v9 -= v12;
        v6 += v12;
        sub_40d1f0(v8);
        v8 = sub_40e130();
        if (v9 <= NULL)
            return v8;
    }
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
    sub_4163d0(); /* do not return */
}



// Function: xheader_finish @ 0x1cc10
typedef struct struct_3 {
    struct struct_3 *field_0;
    unsigned long long field_8;
    long long field_10;
} struct_3;

typedef struct struct_2 {
    struct struct_0 *field_0;
    char padding_8[8];
    unsigned long long field_10;
} struct_2;

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
} struct_0;

extern struct_3 *g_484380;

struct_0 * xheader_finish(struct_2 *a0)
{
    unsigned long long v2;  // rbx
    struct_3 *v3;  // rbx
    struct_3 *v4;  // rbx
    struct_0 *idx;  // rax
    unsigned long long v6;  // rsi
    unsigned long long v7;  // rdx
    unsigned long long v8;  // rdx
    unsigned long long v9;  // rcx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    v3 = g_484380;
    if (g_484380)
    {
        do
        {
            v4 = v3;
            sub_41a9b0(v4->field_10, v4->field_8, *((unsigned int *)&a0));
            v3 = v4->field_0;
        } while (v4->field_0);
    }
    idx = a0->field_0;
    v6 = idx->field_10;
    v7 = idx->field_18;
    if (v7 == v6)
        idx->field_50 = idx->field_50 | 2;
    v8 = v7 + idx->field_30 & ~(idx->field_30);
    v9 = idx->field_20;
    if (v8 - idx->field_8 <= idx->field_20 - idx->field_8)
        v9 = v8;
    idx->field_18 = v9;
    idx->field_10 = v9;
    a0->field_10 = v6;
    return idx;
}



// Function: xheader_destroy @ 0x1cca0
void xheader_destroy(void* idx)
{
    if (*((long long *)idx))
    {
        _obstack_free(*((long long *)idx), 0);
        free(*((long long *)idx));
        *((unsigned long *)idx) = 0;
    }
    else
    {
        free((long long)idx[16]);
    }
    *((void* *)&idx[16]) = NULL;
    *((unsigned long long *)&idx[8]) = 0;
    return;
}



// Function: xheader_write @ 0x1ccf0
typedef struct struct_0 {
    char padding_0[8];
    char field_8;
    char padding_9[7];
    unsigned long long field_10;
} struct_0;

extern void g_484348;
extern unsigned long long g_484350;
extern void g_484360;
extern unsigned long long g_484368;
extern unsigned long long g_484398;

void xheader_write(char a0, unsigned int a1, unsigned long a2, struct_0 *idx)
{
    unsigned long v1;  // r14
    struct_0 *v2;  // rax
    void* dst;  // r8
    unsigned int v12;  // eax
    char *v13;  // r14
    unsigned long long *iter;  // rdi
    unsigned long long v15;  // rcx
    unsigned long long v3;  // rbx
    unsigned long i;  // r14
    unsigned long long *v5;  // r15
    unsigned long long *ptr;  // r8
    unsigned long long *node;  // rdi
    unsigned long long *v8;  // rcx
    unsigned long long *j;  // rsi
    unsigned long long v10;  // rcx

    v1 = *((long long *)&idx->field_8);
    switch (a0)
    {
    case 103:
        if (g_484350)
        {
            a2 = *((long long *)&g_484348);
            break;
        }
        break;
    case 120:
        if (g_484368)
        {
            a2 = *((long long *)&g_484360);
            break;
        }
        break;
    default:
        goto LABEL_41cd30;
    }
LABEL_41cd30:
    v2 = sub_412cb0(a1, v1, a2);
    v2[6].padding_9[3] = a0;
    sub_412f00(v2);
    v3 = idx->field_10 + v1;
    i = v1;
    do
    {
        v5 = v3 - i;
        ptr = sub_40e130();
        if (i == ((unsigned int)v1 & 0x1ff))
        {
            dst = memcpy(ptr, v5, i);
            v12 = 0x200 - i;
            v13 = i + dst;
            if (v12 >= 8)
            {
                *((unsigned long long *)&v13) = 0;
                *((unsigned long *)(&v13[v12] - 8)) = 0;
                iter = v13 + 8 & 0xfffffffffffffff8;
                for (v15 = (v12 + (v13 - iter & 0xffffffff) & 0xffffffff) >> 3 & 0xffffffff; v15; iter += 1)
                {
                    v15 -= 1;
                    *(iter) = 0;
                }
            }
            else if ((char)v12 & 4)
            {
                *((unsigned int *)&v13) = 0;
                *((unsigned int *)(&v13[v12] - 4)) = 0;
            }
            else if (v12)
            {
                *(v13) = 0;
                if ((char)v12 & 2)
                    *((unsigned short *)(&v13[v12] - 2)) = 0;
            }
            sub_40d1f0(dst);
            break;
        }
        node = ptr + 1 & 0xfffffffffffffff8;
        i -= 0x200;
        v8 = ptr - node;
        *(ptr) = *(v5);
        j = v5 - v8;
        v10 = (unsigned int)v8 + 0x200 >> 3;
        for (ptr[63] = v5[63]; v10; j += 1)
        {
            v10 -= 1;
            *(node) = *(j);
            node += 1;
        }
        sub_40d1f0(ptr);
    } while (i);
    sub_41cca0(idx);
    if (a0 == 103)
        g_484398 = g_484398 + 1;
    return;
}



// Function: xheader_string_begin @ 0x1cf40
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

void xheader_string_begin(struct_0 *a0)
{
    a0->field_18 = 0;
    return;
}



// Function: xheader_string_add @ 0x1cf50
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

void xheader_string_add(struct_0 *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    if (!a0->field_10)
    {
        v0 = v2;
        sub_41cf69();
    }
    return;
}



// Function: xheader_string_end @ 0x1cfa0
typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

unsigned int xheader_string_end(struct_2 *a0, char *a1)
{
    char *v4;  // rbx
    unsigned long len;  // rax
    char *i;  // rbx
    char *v7;  // r8
    unsigned long long v8;  // rbx
    unsigned long v9;  // rax
    void* v10;  // rbp
    char *v11;  // rax
    char *v0;  // [bp-0x60]
    char v1;  // [bp-0x58]
    char v2;  // [bp-0x44]

    if (a0[1].field_0)
        return 0;
    v4 = NULL;
    sub_41a5a0(a0);
    len = strlen(a1);
    do
    {
        i = v4;
        v7 = sub_443090(&i[3 + len + a0[1].field_8], &v1);
        v4 = &v2 - v7;
    } while (i != v4);
    v0 = v7;
    v8 = &v4[strlen(a1) + 2];
    sub_41ab10(a0, v8);
    sub_41a830(a0, 10);
    v9 = ~(a0[1].field_8);
    v10 = &a0->field_0->field_18[v9 + -1 * v8];
    memmove(&a0->field_0->field_18[v9], v10, a0[1].field_8);
    v11 = stpcpy(v10, v0);
    *(v11) = 32;
    *(stpcpy(v11 + 1, a1)) = 61;
    return 1;
}



// Function: hash_directory_meta @ 0x1d0c0
void hash_directory_meta(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: compare_directory_meta @ 0x1d0e0
unsigned long long compare_directory_meta(unsigned long long *a0, unsigned long a1)
{
    if (a0[3] == *((long long *)(a1 + 24)))
        return _INSERT(*((long long *)(a1 + 32)), 0, a0[4] == *((long long *)(a1 + 32)));
    return 0;
}



// Function: dumpdir_next @ 0x1d110
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

char * dumpdir_next(struct_0 *idx)
{
    unsigned long index;  // rbp
    unsigned long long *v2;  // rax
    char *v3;  // r12
    unsigned long v4;  // r12

    index = idx->field_10;
    v2 = &idx->field_0->field_0;
    if (idx->field_8)
    {
        v3 = *(v2) + index;
        if (!*(v3))
            return NULL;
        idx->field_10 = index + strlen(v3) + 1;
        return v3;
    }
    else if (v2[2] <= index)
    {
        return NULL;
    }
    else
    {
        v4 = *((long long *)(v2[3] + index * 8));
        idx->field_10 = index + 1;
        return v4 - 1;
    }
}



// Function: free_directory @ 0x1d180
void free_directory(void* *a0)
{
    free(a0[10]);
    free(a0[11]);
    free(a0);
    return;
}



// Function: dumpdir_free @ 0x1d1a0
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
} struct_0;

void dumpdir_free(struct_0 *a0)
{
    free(a0->field_18);
    free(a0);
    return;
}



// Function: dirlist_replace_prefix @ 0x1d1c0
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_4843c0;

long long dirlist_replace_prefix(unsigned long a0, unsigned long a1)
{
    unsigned long len;  // rax
    unsigned long len1;  // rax
    struct_0 **v3;  // rbx
    struct_0 **v4;  // rbx
    unsigned long v5;  // rax

    len = strlen(a0);
    len1 = strlen(a1);
    v3 = g_4843c0;
    if (!g_4843c0)
        return len1;
    do
    {
        v4 = v3;
        v5 = sub_423fc0(v4 + 11, a0, len, a1, len1);
        v3 = *(v4);
    } while (*(v4));
    return v5;
}



// Function: dumpdir_first @ 0x1d220
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

char * dumpdir_first(unsigned long a0, unsigned int a1, struct_0 **a2)
{
    struct_0 *idx;  // rax

    idx = sub_44c5b0(24);
    idx->field_0 = a0;
    idx->field_8 = a1;
    idx->field_10 = 0;
    *(a2) = idx;
    return sub_41d110(idx);
}



// Function: make_directory @ 0x1d260
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[32];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[4];
    unsigned int field_3c;
    unsigned long long field_40;
    unsigned long long field_48;
    unsigned long long field_50;
    void* field_58;
} struct_0;

struct_0 * make_directory(void* ptr, unsigned long a1)
{
    unsigned long len;  // rbx
    struct_0 *v2;  // rax
    struct_0 *idx;  // r12
    unsigned long v4;  // rdi
    void* v5;  // rax
    void* dst;  // rax

    len = strlen(ptr);
    v2 = sub_44c5b0(96);
    v2->field_0 = 0;
    idx = v2;
    idx->field_30 = 0;
    idx->field_28 = 0;
    idx->field_40 = 0;
    idx->field_3c = 0;
    if (len <= 1 || *((char *)ptr + len - 1) != 47)
    {
        v4 = len + 1;
    }
    else
    {
        v4 = len;
        len -= 1;
    }
    v5 = sub_44c5b0(v4);
    idx->field_58 = v5;
    dst = memcpy(v5, ptr, len);
    idx->field_50 = a1;
    *((char *)dst + len) = 0;
    idx->field_48 = 0;
    return idx;
}



// Function: find_directory @ 0x1d310
extern long long g_4843b0;
extern unsigned int g_484580;

long long find_directory(unsigned long a0)
{
    unsigned long v1;  // rax
    long long v2;  // rax

    if (!g_4843b0)
        return g_4843b0;
    v1 = sub_41d260(a0, sub_425340(g_484580, a0));
    v2 = sub_4418d0(g_4843b0, v1);
    sub_41d180(v1);
    return v2;
}



// Function: find_directory_meta @ 0x1d370
extern void g_467708;
extern long long g_4843a8;

long long find_directory_meta(unsigned long a0, unsigned long a1)
{
    unsigned long long *idx;  // rax
    long long v2;  // rax

    if (!g_4843a8)
        return g_4843a8;
    idx = &sub_41d260(&g_467708, 0)->field_0;
    idx[3] = a0;
    idx[4] = a1;
    v2 = sub_4418d0(g_4843a8, idx);
    sub_41d180(idx);
    return v2;
}



// Function: attach_directory @ 0x1d3d0
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[32];
    unsigned long long field_28;
    unsigned long long field_30;
    char padding_38[4];
    unsigned int field_3c;
    unsigned long long field_40;
    unsigned long long field_48;
    unsigned long long field_50;
    void* field_58;
} struct_0;

extern struct_0 *g_4843b8;
extern unsigned long long g_4843c0;
extern unsigned int g_484580;

struct_0 * attach_directory(unsigned long a0)
{
    struct_0 *v1;  // rax

    v1 = sub_41d260(a0, sub_425340(g_484580, a0));
    if (g_4843b8)
    {
        g_4843b8->field_0 = v1;
        g_4843b8 = v1;
    }
    else
    {
        g_4843c0 = v1;
        g_4843b8 = v1;
    }
    return v1;
}



// Function: compare_dirnames @ 0x1d420
int compare_dirnames(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: compare_directory_canonical_names @ 0x1d430
typedef struct struct_0 {
    char padding_0[80];
    char *field_50;
} struct_0;

char compare_directory_canonical_names(struct_0 *a0, struct_0 *a1)
{
    return !strcmp(a0->field_50, a1->field_50);
}



// Function: dumpdir_locate @ 0x1d450
typedef struct struct_0 {
    char padding_0[16];
    char field_10;
    char padding_11[7];
    void* field_18;
} struct_0;

unsigned long long dumpdir_locate(struct_0 *a0, unsigned long a1)
{
    unsigned long long *v2;  // rax
    unsigned long v0;  // [bp-0x10]

    v0 = a1;
    if (!a0)
        return 0;
    v2 = bsearch(&v0, a0->field_18, *((long long *)&a0->field_10), 8, sub_41d420);
    if (!v2)
        return 0;
    return *(v2) - 1;
}



// Function: dumpdir_create0 @ 0x1d4a0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

struct_0 * dumpdir_create0(char *ptr, char *a1)
{
    char v3;  // r13b
    unsigned long n;  // r12
    unsigned long long v13;  // rax
    struct_0 *v14;  // rbx
    unsigned long long iter;  // r13
    unsigned int v16;  // esi
    unsigned long long index;  // r12
    unsigned long long v5;  // rbx
    char *node;  // r15
    unsigned long v7;  // r14
    unsigned long v8;  // rdi
    unsigned long long v9;  // r8
    struct_0 *v10;  // rax
    struct_0 *v11;  // rdi
    struct_0 *idx;  // r14
    unsigned long long v0;  // [bp-0x50]
    unsigned long long v1;  // [bp-0x40]

    v3 = *(ptr);
    if (*(ptr))
    {
        v0 = 0;
        n = 1;
        v5 = 0;
        node = ptr;
        do
        {
            n += strlen(node) + 1;
            if (!a1 || strchr(a1, v3))
                v0 += 1;
        } while ((node += v7, v5 += 1, v3 = *(node), *(node)));
        v8 = n + 32;
        v9 = v0 + 1;
    }
    else
    {
        v0 = 0;
        v9 = 1;
        v8 = 33;
        v5 = 0;
        n = 1;
    }
    v1 = v9;
    v10 = sub_44c5b0(v8);
    v11 = &v10->field_18;
    idx = v10;
    idx->field_0 = v11;
    memcpy(v11, ptr, n);
    *((unsigned long long *)&idx->field_1) = v5;
    idx->field_8 = v0;
    v13 = sub_44c720(v1, 8);
    v14 = idx->field_0;
    idx->field_10 = v13;
    iter = v13;
    v16 = (char)v14->field_0;
    if ((char)v16)
    {
        index = 0;
        do
        {
            if (!a1 || strchr(a1, v16))
            {
                *((struct_0 **)(iter + index * 8)) = (char *)&v14->field_0 + 1;
                index += 1;
            }
        } while ((v14 = v14 + strlen(v14) + 1, v16 = (unsigned int)(int)(char)v14->field_0, (char)v16));
        iter += index * 8;
    }
    *((unsigned long long *)iter) = 0;
    return idx;
}



// Function: makedumpdir @ 0x1d600
typedef struct struct_1 {
    char padding_0[40];
    void* field_28;
    void* field_30;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    void* field_28;
    void* field_30;
    unsigned int field_38;
    char padding_3c[4];
    struct struct_1 *field_40;
    unsigned long long field_48;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

void makedumpdir(struct_0 *idx, char *a1)
{
    void* v2;  // r14
    char *v3;  // rbx
    void* v12;  // rbx
    void* v13;  // rdx
    char *node;  // rax
    char j;  // dl
    void* v17;  // rax
    void* *index;  // rax
    unsigned long long v5;  // r15
    unsigned long i;  // rbp
    char *v7;  // r12
    unsigned long len;  // rax
    struct_2 **ptr;  // r12
    struct_2 **iter;  // r15
    unsigned long idx1;  // r15
    void* v0;  // [bp-0x40], Other Possible Types: unsigned long long

    v2 = NULL;
    v3 = a1;
    if (idx->field_38 != 2)
    {
        index = idx->field_40;
        if (index)
        {
            v2 = index[6];
            if (!index[6])
            {
                v2 = index[5];
                goto LABEL_41d637;
            }
        }
        else
        {
            v2 = idx->field_28;
            if (*(v3))
                goto LABEL_41d640;
LABEL_41d78c:
            i = 0;
            v0 = 1;
            ptr = sub_44c720(0, 8);
            if (*(v3))
                goto LABEL_41d68b;
            qsort(ptr, 0, 8, sub_41d420);
            v0 = sub_44c5b0(1);
            v12 = v0;
        }
    }
    else
    {
LABEL_41d637:
        if (!*(v3))
            goto LABEL_41d78c;
LABEL_41d640:
        v5 = 0;
        i = 0;
        v7 = v3;
        do
        {
            i += 1;
            len = strlen(v7);
            v7 = &v7[len + 1];
            v5 = len + v5 + 2;
        } while (*(v7));
        v0 = v5 + 1;
        ptr = sub_44c720(i, 8);
        if (*(v3))
        {
LABEL_41d68b:
            iter = ptr;
            do
            {
                *(iter) = v3;
                iter += 1;
                v3 = &v3[strlen(v3) + 1];
            } while (*(v3));
        }
        idx1 = 0;
        qsort(ptr, i, 8, sub_41d420);
        v0 = sub_44c5b0(v0);
        v12 = v0;
        if (i)
        {
            do
            {
                v13 = v12 + 1;
                *((char *)v12) = (!sub_41d450(v2, ptr[idx1]) ? ((char)-(idx->field_48 < 1) & 16) + 73 : ((char)-(idx->field_48 < 1) & 215) + 73);
                node = &ptr[idx1]->field_0;
                do
                {
                    j = *(node);
                    node += 1;
                    v12 = v13 + 1;
                    *((char *)v12 - 1) = j;
                    v13 = v12;
                } while (j);
                idx1 += 1;
            } while (i != idx1);
        }
    }
    v17 = idx->field_28;
    *((char *)v12) = 0;
    idx->field_30 = v17;
    idx->field_28 = sub_41d4a0(v0, NULL);
    free(v0);
    free(ptr);
    return;
}



// Function: dumpdir_create @ 0x1d7f0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

struct_0 * dumpdir_create(char *a0)
{
    return sub_41d4a0(a0, "YND");
}



// Function: maketagdumpdir @ 0x1d800
typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

void maketagdumpdir(struct_0 *idx)
{
    unsigned long len;  // r12
    char *v2;  // rax
    void* v3;  // rsi
    char *ptr;  // rbp
    struct_0 *v5;  // rax

    len = strlen(idx->field_48);
    v2 = sub_44c5b0(len + 3);
    v3 = idx->field_48;
    *(v2) = 89;
    ptr = v2;
    memcpy(ptr + 1, v3, len + 1);
    v5 = idx->field_28;
    ptr[2 + len] = 0;
    idx->field_30 = v5;
    idx->field_28 = sub_41d4a0(ptr, NULL);
    free(ptr);
    return;
}



// Function: dumpdir_ok @ 0x1d860
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b80;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int dumpdir_ok(char *a0)
{
    char v1;  // bl
    unsigned int v2;  // r12d
    unsigned int v3;  // r13d

    v1 = *(a0);
    if (!*(a0))
        return 1;
    v2 = 0;
    v3 = 0;
    do
    {
        if (v2)
        {
            if (v1 != v2)
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Malformed dumpdir: expected '%c' but found %#3o", 5));
                g_484d58 = 2;
                return 0;
            }
            if (v1 == 84)
            {
                if (v2 == 84)
                {
                    if (!a0[1] && !v3)
                    {
                        if (g_484d50)
                            g_484d50();
                        error(0, 0, dcgettext(NULL, "Malformed dumpdir: empty name in 'T'", 5));
                        g_484d58 = 2;
                        return 0;
                    }
                    v2 = 0;
                    continue;
                }
LABEL_41da57:
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Malformed dumpdir: 'T' not preceded by 'R'", 5));
                g_484d58 = 2;
                return 0;
            }
        }
        else if (v1 == 84)
        {
            goto LABEL_41da57;
        }
        switch (v1)
        {
        case 88:
            if (!v3)
            {
                v3 = 1;
                break;
            }
            else
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Malformed dumpdir: 'X' duplicated", 5));
                g_484d58 = 2;
                return 0;
            }
        case 82:
            if (a0[1])
            {
                v2 = 84;
                break;
            }
            else if (v3)
            {
                v2 = 84;
                v3 = 0;
                break;
            }
            else
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Malformed dumpdir: empty name in 'R'", 5));
                g_484d58 = 2;
                return 0;
            }
        default:
            continue;
        }
    } while ((a0 = a0 + strlen(a0) + 1, v1 = *(a0), *(a0)));
    if (v2)
    {
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Malformed dumpdir: expected '%c' but found end of data", 5));
        g_484d58 = 2;
        return 0;
    }
    else if (!v3)
    {
        return 1;
    }
    else if (g_483b80 & 2)
    {
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Malformed dumpdir: 'X' never used", 5));
    }
    else
    {
        return 1;
    }
    return 1;
}



// Function: note_directory @ 0x1dac0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    void* field_28;
    char padding_30[8];
    unsigned int field_38;
    unsigned int field_3c;
} struct_0;

extern unsigned long long g_4843a8;
extern unsigned long long g_4843b0;

struct_0 * note_directory(unsigned int a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, char a5, char a6, void* a7)
{
    void* v0;  // rbp
    struct_0 *v1;  // rax
    struct_0 *idx;  // r12
    unsigned long long v3;  // rdi
    unsigned long long v4;  // rdi

    v0 = a7;
    v1 = sub_41d3d0(a0);
    v1->field_10 = a2;
    idx = v1;
    idx->field_8 = a1;
    idx->field_18 = a3;
    idx->field_20 = a4;
    idx->field_38 = 1;
    if (a5)
        idx->field_3c = idx->field_3c | 2;
    if (a6)
        idx->field_3c = idx->field_3c | 4;
    if (v0)
        v0 = sub_41d7f0(v0);
    <None|const 0> = g_4843b0;
    idx->field_28 = v0;
    if (v3 || (g_4843b0 = (unsigned long long)sub_441b30(0, 0, sub_41e1a0, sub_41d430, 0), v3 = g_4843b0, g_4843b0))
    {
        if (sub_442140(v3, idx))
        {
            <None|const 0> = g_4843a8;
            if (v4 || (g_4843a8 = (unsigned long long)sub_441b30(0, 0, sub_41d0c0, sub_41d0e0, 0), v4 = g_4843a8, g_4843a8))
            {
                if (sub_442140(v4, idx))
                    return idx;
            }
        }
    }
    sub_416420(); /* do not return */
}



// Function: procdir @ 0x1dbe0
typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

typedef struct struct_0 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    int field_1a8;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern char g_483b81;
extern char g_483b82;
extern char g_484ad1;
extern unsigned long long g_484b00;
extern unsigned long long g_484b08;
extern char g_484b20;
extern unsigned long long g_484b28;
extern unsigned int g_484b8c;
extern struct_2 *g_484d50;

void* procdir(unsigned long a0, struct_1 *index, unsigned int a2, char *a3)
{
    unsigned long v5;  // r14
    void* v6;  // rax
    struct_0 *idx;  // r15
    unsigned long long v16;  // rax
    long long v17;  // rdx
    unsigned long v18;  // r14
    long long v21;  // rax
    unsigned int v22;  // eax
    struct_0 *v24;  // rdi
    void* idx1;  // r13
    unsigned int v25;  // eax
    unsigned int v27;  // eax
    unsigned long v8;  // rdi
    struct_0 *idx2;  // r15
    unsigned long v10;  // rdi
    char v12;  // dl
    unsigned long v13;  // rax
    unsigned int v14;  // eax
    unsigned long long v0;  // [bp-0x68], Other Possible Types: unsigned long
    unsigned long v1;  // [bp-0x60]
    struct_1 *v2;  // [bp-0x50], Other Possible Types: unsigned long long
    long long v3;  // [bp-0x48], Other Possible Types: unsigned long long

    v5 = *((long long *)&index->padding_0[88]);
    v6 = sub_41d310(a0);
    if (v6)
    {
        idx1 = v6;
        if ((char)v6[60] & 1)
        {
            if ((char)a2 & 32)
            {
                sub_423be0(v6 + 88, a0);
            }
            else
            {
                *(a3) = 78;
                return idx1;
            }
        }
        if (strcmp((long long)idx1[88], a0))
        {
            *(a3) = 78;
            return idx1;
        }
        if (!(!g_484b20 || (char)idx1[60] & 2 && v5 < NULL || !(v8 = (unsigned long)*((long long *)&index->padding_0[88]), (long long)idx1[24] != *((long long *)&index->padding_0[88]))))
        {
LABEL_41dc86:
            idx2 = sub_41d370(v8, *((long long *)&index->padding_0[96]));
            if (idx2)
            {
                v0 = idx2->field_58;
                if (strcmp(idx2->field_58, a0))
                {
                    v10 = v0;
                    if (g_483b81 & 32)
                    {
                        if (g_484d50)
                        {
                            g_484d50();
                            v10 = idx2->field_58;
                        }
                        v2 = sub_44ac40(1, v10);
                        v0 = sub_44aa70(a0);
                        error(0, 0, dcgettext(NULL, "%s: Directory has been renamed from %s", 5));
                        v10 = idx2->field_58;
                    }
                    *((unsigned int *)&idx1[60]) = (int)idx1[60] | 16;
                    *((struct_0 **)&idx1[64]) = idx2;
                    idx2->field_3c = idx2->field_3c & 0xffffffef;
                    sub_41d1c0(v10, a0);
                }
                *((unsigned int *)&idx1[56]) = 1;
                v12 = 0;
            }
            else
            {
                v13 = *((long long *)&index->padding_0[88]);
                *((unsigned int *)&idx1[56]) = 2;
                v12 = 1;
                *((unsigned long *)&idx1[24]) = v13;
                *((long long *)&idx1[32]) = *((long long *)&index->padding_0[96]);
            }
            v14 = (int)idx1[60];
            if (v5 < NULL)
                v14 = (int)idx1[60] | 2;
        }
        else if ((long long)idx1[32] != *((long long *)&index->padding_0[96]))
        {
            v8 = *((long long *)&index->padding_0[88]);
            goto LABEL_41dc86;
        }
        else
        {
            *((unsigned int *)&idx1[56]) = 1;
            v14 = (int)idx1[60];
            v12 = 0;
        }
        *((unsigned int *)&idx1[60]) = v14 | 4;
    }
    else
    {
        v2 = &index->padding_0[88];
        idx = sub_41d370(*((long long *)&index->padding_0[88]), *((long long *)&index->padding_0[96]));
        v0 = *((long long *)&index->padding_0[96]);
        v16 = sub_44bd20(&index->padding_0[88]);
        idx1 = sub_41dac0(a0, v16, v17, *((long long *)&index->padding_0[88]), v0, v5 >> 63, 1, NULL);
        if (idx)
        {
            v18 = idx->field_58;
            if (strcmp(v18, a0))
            {
                if (g_483b81 & 32)
                {
                    if (g_484d50)
                    {
                        g_484d50();
                        v18 = idx->field_58;
                    }
                    v0 = sub_44ac40(1, v18);
                    sub_44aa70(a0);
                    error(0, 0, dcgettext(NULL, "%s: Directory has been renamed from %s", 5));
                    v18 = idx->field_58;
                }
                *((unsigned int *)&idx1[60]) = (int)idx1[60] | 16;
                *((struct_0 **)&idx1[64]) = idx;
                idx->field_3c = idx->field_3c & 0xffffffef;
                sub_41d1c0(v18, a0);
            }
            *((unsigned int *)&idx1[56]) = 1;
            v12 = 0;
        }
        else
        {
            *((unsigned int *)&idx1[60]) = (int)idx1[60] | 8;
            if (g_483b81 & 16)
            {
                if (g_484d50)
                    g_484d50();
                sub_44aa70(a0);
                error(0, 0, dcgettext(NULL, "%s: Directory is new", 5));
            }
            if (g_484b28 || (v21 = (long long)(unsigned long long)sub_44bd20(v2), v0 = (unsigned long)g_484b00, v1 = (unsigned long)g_484b08, (unsigned int)(unsigned long long)sub_44be80((unsigned long long)sub_44bd20(v2), v17, g_484b00, g_484b08) < 0 || g_484b8c && (unsigned int)(unsigned long long)sub_44be80((unsigned long long)sub_44bd10(v2), v17, v0, v1) < 0))
                v22 = 2;
            else
                v22 = 1;
            *((unsigned int *)&idx1[56]) = v22;
            v12 = 0;
        }
    }
    if (g_484ad1 && index->field_198 && *((long long *)&index->padding_0[88]) != *((long long *)&index->field_198->padding_0[88]))
    {
        if (g_483b82 & 4)
        {
            if (g_484d50)
                g_484d50();
            sub_44aa70((long long)idx1[88]);
            error(0, 0, dcgettext(NULL, "%s: directory is on a different filesystem; not dumped", 5));
        }
        v24 = (long long)idx1[40];
        *((unsigned int *)&idx1[56]) = 0;
        if (!v24)
        {
            *((unsigned int *)&idx1[60]) = (int)idx1[60] | 1;
            return idx1;
        }
        sub_41d1a0(v24);
        *((struct_0 **)&idx1[40]) = NULL;
        v25 = (int)idx1[56];
    }
    else
    {
        if ((char)a2 & 16)
        {
            *((unsigned int *)&idx1[56]) = a2 & 3;
            if (!((char)a2 & 3))
                *(a3) = 78;
        }
        if (v12 && g_483b81 & 32)
        {
            if (g_484d50)
                g_484d50();
            sub_44aa70(a0);
            error(0, 0, dcgettext(NULL, "%s: Directory has been renamed", 5));
        }
        v25 = (int)idx1[56];
    }
    *((unsigned int *)&idx1[60]) = (int)idx1[60] | 1;
    if (!v25)
        return idx1;
    v27 = sub_414160(index, &v3);
    if (v27 == 2)
    {
        sub_412ad0(a0, v3, dcgettext(NULL, "contents not dumped", 5));
        *((long long *)&idx1[72]) = v3;
        return idx1;
    }
    else if (v27 == 3)
    {
        sub_412ad0(a0, v3, dcgettext(NULL, "directory not dumped", 5));
        *(a3) = 78;
        *((unsigned int *)&idx1[56]) = 0;
        return idx1;
    }
    else if (v27 != 1)
    {
        return idx1;
    }
    else
    {
        sub_412ad0(a0, v3, dcgettext(NULL, "contents not dumped", 5));
        *((unsigned int *)&idx1[56]) = 0;
        *((unsigned long long *)&idx1[72]) = v3;
        return idx1;
    }
}



// Function: hash_directory_canonical_name @ 0x1e1a0
typedef struct struct_0 {
    char padding_0[80];
    struct struct_1 *field_50;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

unsigned long long hash_directory_canonical_name(struct_0 *a0, unsigned long long a1)
{
    char *v1;  // rdi
    unsigned long v2;  // rcx
    unsigned long long v3;  // rdx

    v1 = &a0->field_50->field_0;
    v2 = a0->field_50->field_0;
    v3 = 0;
    if ((char)v2)
    {
        do
        {
            v1 += 1;
            v3 = (v3 * 31 + v2) % a1;
            v2 = *(v1);
        } while ((char)v2);
    }
    return v3;
}



// Function: obstack_code_rename @ 0x1e1b0
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_484b92;

void* obstack_code_rename(struct_0 *idx, char *a1, char *a2)
{
    char *ptr;  // r13
    char *ptr1;  // r12
    void* dst;  // rax
    char *v3;  // rax
    unsigned long len;  // rax
    char *v5;  // rdi
    unsigned long n;  // rbp
    char *v7;  // rax
    unsigned long len1;  // rax
    char *v9;  // rdi
    unsigned long v10;  // rbp

    ptr = a1;
    ptr1 = a2;
    if (!*(a1))
    {
        v3 = &idx->field_18->field_0;
        if (*((long long *)&idx[1].padding_0[0]) == idx->field_18)
            goto LABEL_41e29a;
    }
    else
    {
        ptr = sub_433020(ptr, 0, g_484b92);
        v3 = &idx->field_18->field_0;
        if (*((long long *)&idx[1].padding_0[0]) == idx->field_18)
        {
LABEL_41e29a:
            _obstack_newchunk(idx, 1);
            v3 = &idx->field_18->field_0;
        }
    }
    idx->field_18 = v3 + 1;
    *(v3) = 82;
    len = strlen(ptr);
    v5 = &idx->field_18->field_0;
    n = len + 1;
    if (n > *((long long *)&idx[1].padding_0[0]) - (char *)idx->field_18)
    {
        _obstack_newchunk(idx, n);
        v5 = &idx->field_18->field_0;
    }
    memcpy(v5, ptr, n);
    v7 = &idx->field_18[n].field_0;
    idx->field_18 = v7;
    if (*(ptr1))
    {
        ptr1 = sub_433020(ptr1, 0, g_484b92);
        v7 = &idx->field_18->field_0;
        if (*((long long *)&idx[1].padding_0[0]) == v7)
        {
LABEL_41e2ea:
            _obstack_newchunk(idx, 1);
            v7 = &idx->field_18->field_0;
        }
    }
    else if (*((long long *)&idx[1].padding_0[0]) == v7)
    {
        goto LABEL_41e2ea;
    }
    idx->field_18 = v7 + 1;
    *(v7) = 84;
    len1 = strlen(ptr1);
    v9 = &idx->field_18->field_0;
    v10 = len1 + 1;
    if (v10 > *((long long *)&idx[1].padding_0[0]) - (char *)idx->field_18)
    {
        _obstack_newchunk(idx, v10);
        v9 = &idx->field_18->field_0;
    }
    dst = memcpy(v9, ptr1, v10);
    idx->field_18 = &idx->field_18[v10];
    return dst;
}



// Function: store_rename @ 0x1e320
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_467708;

void* store_rename(void* i, struct_0 *idx)
{
    void* iter;  // rbx
    void* v2;  // rax
    unsigned long v3;  // rax
    void* ptr;  // r13
    char *v5;  // rax
    unsigned long len;  // rax
    char *v7;  // rdi
    unsigned long n;  // r14

    iter = i;
    while (1)
    {
        if (!i)
        {
            for (; iter; iter = (long long)iter[64])
            {
                v3 = (long long)iter[64];
                if (!v3)
                    return v3;
                v2 = sub_41e1b0(idx, *((long long *)(v3 + 88)), (long long)iter[88]);
            }
            return v2;
        }
        v2 = (long long)i[64];
        if (v2 == iter)
            break;
        i = v2;
    }
    ptr = sub_43e4f0((long long)iter[88]);
    v5 = &idx->field_18->field_0;
    if (*((long long *)&idx[1].padding_0[0]) == idx->field_18)
    {
        _obstack_newchunk(idx, 1);
        v5 = &idx->field_18->field_0;
    }
    idx->field_18 = v5 + 1;
    *(v5) = 88;
    len = strlen(ptr);
    v7 = &idx->field_18->field_0;
    n = len + 1;
    if (n > *((long long *)&idx[1].padding_0[0]) - (char *)idx->field_18)
    {
        _obstack_newchunk(idx, n);
        v7 = &idx->field_18->field_0;
    }
    memcpy(v7, ptr, n);
    idx->field_18 = &idx->field_18[n];
    sub_41e1b0(idx, (long long)iter[88], &g_467708);
    if (i != iter)
    {
        do
        {
            sub_41e1b0(idx, *((long long *)((long long)iter[64] + 88)), (long long)iter[88]);
            iter = (long long)iter[64];
        } while (i != iter);
    }
    sub_41e1b0(idx, &g_467708, (long long)i[88]);
    return (unsigned long long)free(ptr);
}



// Function: read_incr_db_01 @ 0x1e470
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_4843a0;
extern long long g_484b00;
extern unsigned long long g_484b08;
extern long long g_484b28;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void read_incr_db_01(unsigned int a0, char *a1)
{
    long long v8;  // rdx
    char *v16;  // r12
    long long v17;  // rbp
    unsigned long long v18;  // r13
    int *err;  // rbx
    unsigned long long v20;  // rax
    unsigned long long v21;  // rax
    char *v22;  // rbx
    char *v23;  // rsi
    int *err1;  // rax
    char *v9;  // rbx
    int *err2;  // rax
    int *v11;  // rbp
    unsigned long long v12;  // rax
    unsigned int v13;  // eax
    char *v14;  // rdx
    void* v15;  // rax
    char *v0;  // [bp-0x78], Other Possible Types: unsigned long long
    char *v1;  // [bp-0x70]
    unsigned long long v2;  // [bp-0x68]
    unsigned int v3;  // [bp-0x60]
    char *str;  // [bp-0x58]
    unsigned long v5;  // [bp-0x50]
    char v6;  // [bp-0x48]

    str = NULL;
    v5 = 0;
    if (a0 != 1)
    {
        str = strdup(a1);
        v5 = strlen(str) + 1;
        g_484b00 = sub_4242f0(str, &v6, 0);
        g_484b08 = v8;
        v2 = 1;
        v1 = &str;
        if ((char)sub_423ba0(g_484b00, v8))
            goto LABEL_41e5ad;
    }
    else
    {
        v1 = &str;
        if (getline(&str, &v5, g_4843a0) <= 0)
        {
            sub_432820(g_484b28);
            free(str);
            return;
        }
        g_484b00 = sub_4242f0(str, &v6, 0);
        g_484b08 = v8;
        if ((char)sub_423ba0(g_484b00, v8))
        {
            v2 = 2;
            if (*((char *)v6))
            {
                v9 = v6 + 1;
                err2 = __errno_location();
                *(err2) = 0;
                v11 = err2;
                v12 = strtoumax(v9, &v6, 10);
                if (!*(v11))
                {
                    if (v12 > 0x3b9ac9ff)
                    {
                        *(v11) = 0x22;
                    }
                    else if (!*(v11) && v6 != v9)
                    {
                        g_484b08 = v12;
                        v2 = 2;
                        goto LABEL_41e5ad;
                    }
                }
                if (g_484d50)
                    g_484d50();
                dcgettext(NULL, "Invalid time stamp", 5);
                sub_44aa70(g_484b28);
                error(0, *(v11), "%s:%ld: %s");
                g_484d58 = 2;
                g_484b00 = 0x8000000000000000;
                g_484b08 = 0xffffffffffffffff;
                v2 = 2;
            }
            while (1)
            {
LABEL_41e5ad:
                v13 = getline(v1, &v5, g_4843a0);
                if (v13 <= 0)
                {
                    free(str);
                    return;
                }
                v14 = str;
                v15 = &v14[v13] - 1;
                *((char *)&v3) = *(v14) == 43;
                v2 += 1;
                v16 = &v14[*(v14) == 43];
                if (*((char *)v15) == 10)
                    *((char *)v15) = 0;
                v17 = 0;
                v18 = 0;
                err = __errno_location();
                if (a0 == 1)
                {
                    v17 = sub_4242f0(v16, &v6, 0);
                    v0 = (char *)v6;
                    if (!(char)sub_423ba0(v17, v8) || *(v0) != 32)
                    {
                        if (g_484d50)
                            g_484d50();
                        v23 = "Invalid modification time";
                        break;
                    }
                    *(err) = 0;
                    v18 = strtoumax(v0, &v6, 10);
                    if (v18 > 0x3b9ac9ff)
                    {
                        if (!*(err))
                            *(err) = 0x22;
LABEL_41e67d:
                        if (g_484d50)
                            g_484d50();
                        v23 = "Invalid modification time (nanoseconds)";
                        break;
                    }
                    else if (*(err) || (v16 = (char *)v6, v16 == v0 || *(v16) != 32))
                    {
                        goto LABEL_41e67d;
                    }
                }
                v20 = sub_4240a0(v16, &v6, 0, -0x1);
                if (!*(err) && *((char *)v6) == 32)
                {
                    v21 = sub_4240a0();
                    if (!*(err) && *((char *)v6) == 32)
                    {
                        v22 = v6 + 1;
                        v0 = v21;
                        sub_423c70(v22);
                        sub_41dac0(*((unsigned int *)&v22), v17, v18, v20, v0, (char)v3 & 1, 0, NULL);
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        v23 = "Invalid inode number";
                        break;
                    }
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    v23 = "Invalid device number";
                    break;
                }
            }
            dcgettext(NULL, v23, 5);
            sub_44aa70(g_484b28);
            error(0, *(err), "%s:%ld: %s");
            sub_4163d0(); /* do not return */
        }
    }
    if (g_484d50)
        g_484d50();
    dcgettext(NULL, "Invalid time stamp", 5);
    sub_44aa70(g_484b28);
    err1 = __errno_location();
    error(0, *(err1), "%s:%ld: %s");
    sub_4163d0(); /* do not return */
}



// Function: read_obstack @ 0x1e960
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

int read_obstack(FILE *a0, struct_0 *idx, unsigned long long *a2)
{
    unsigned long long v1;  // r13
    unsigned int v2;  // r12d
    char *v3;  // rax
    char *v4;  // rax

    v1 = 0;
    v2 = getc_unlocked(a0);
    if (v2 + 1 > 1)
    {
        do
        {
            v3 = &idx->field_18->field_0;
            if (*((long long *)&idx[1].padding_0[0]) == idx->field_18)
            {
                _obstack_newchunk(idx, 1);
                v3 = &idx->field_18->field_0;
            }
        } while ((v1 += 1, idx->field_18 = (struct struct_1 *)(v3 + 1), *(v3) = (char)v2, v2 = (unsigned int)(int)(long long)getc_unlocked(a0), v2 + 1 > 1));
    }
    v4 = &idx->field_18->field_0;
    if (*((long long *)&idx[1].padding_0[0]) == idx->field_18)
    {
        _obstack_newchunk(idx, 1);
        v4 = &idx->field_18->field_0;
    }
    idx->field_18 = v4 + 1;
    *(v4) = 0;
    *(a2) = v1;
    return v2;
}



// Function: read_num @ 0x1ea10
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern long long g_484b28;
extern struct_0 *g_484d50;

unsigned int read_num(FILE *a0, unsigned long long a1, long long a2, long long a3, unsigned long long *a4)
{
    unsigned long long ch;  // rax
    unsigned int v11;  // ecx
    unsigned long long v21;  // rcx
    char v12;  // bpl
    unsigned long long v13;  // rbx
    char *v15;  // rdx
    unsigned int v16;  // ebp
    unsigned long long v18;  // rax
    unsigned long long v0;  // [bp-0xe8]
    unsigned long long v1;  // [bp-0xe0]
    char *v2;  // [bp-0xd8], Other Possible Types: unsigned long long
    char *v3;  // [bp-0xd0], Other Possible Types: unsigned long, unsigned long long
    char v4;  // [bp-0xc8], Other Possible Types: long long, unsigned int, unsigned long
    char v5;  // [bp-0xb8]
    char v6;  // [bp-0x98]
    char v7;  // [bp-0x78]
    char v8;  // [bp-0x58]

    v4 = a2;
    ch = getc_unlocked(a0);
    v11 = ch;
    v12 = (unsigned int)ch == 45;
    v13 = 0;
    while (1)
    {
        if (!(!(unsigned int)v13 & v12))
        {
            if (v11 - 48 <= 9)
            {
                (&v5)[v13] = v11;
                if (v13 != 20)
                    goto LABEL_41ea7b;
                if (g_484d50)
                    g_484d50();
                sub_442ff0(ftello(a0), &v6);
                sub_44aa70(g_484b28);
                v3 = &v5;
                v2 = 21;
                v15 = dcgettext(NULL, "%s: byte %s: %s %.*s... too long", 5);
            }
            else
            {
                (&v5)[(unsigned int)v13] = 0;
                if (v11 >= 0)
                {
                    if (!v11)
                    {
                        *(a4) = sub_4240a0(&v5, 0, v4, a3);
                        v16 = *(__errno_location());
                        switch (v16)
                        {
                        case 0:
                            return 1;
                        case 34:
                            if (g_484d50)
                                g_484d50();
                            v18 = sub_443090(a3, &v8);
                            sub_442f50(v4, &v7);
                            sub_442ff0(ftello(a0), &v6);
                            sub_44aa70(g_484b28);
                            v3 = v21;
                            v2 = &v5;
                            v1 = a1;
                            v0 = v18;
                            error(0, 0x22, dcgettext(NULL, "%s: byte %s: (valid range %s..%s)\n\t%s %s", 5));
                            sub_4163d0(); /* do not return */
                        default:
                            if (g_484d50)
                                g_484d50();
                            sub_442ff0(ftello(a0), &v6);
                            sub_44aa70(g_484b28);
                            v3 = dcgettext(NULL, "%s: byte %s: %s %s", 5);
                            v2 = &v5;
                            error(0, v16, v3);
                            sub_4163d0(); /* do not return */
                        }
                    }
                    if (g_484d50)
                    {
                        *((unsigned int *)&v4) = v11;
                        g_484d50();
                        v11 = v4;
                    }
                    v4 = v11;
                    sub_442ff0(ftello(a0), &v6);
                    sub_44aa70(g_484b28);
                    v15 = dcgettext(NULL, "%s: byte %s: %s %s followed by invalid byte 0x%02x", 5);
                    v3 = v4;
                    v2 = &v5;
                }
                else
                {
                    v4 = _INSERT(ch, 0, !(unsigned int)v13 & v12) & 0xffffffff;
                    if (ferror_unlocked(a0))
                    {
                        sub_4329b0(g_484b28); /* do not return */
                    }
                    else if (!(unsigned int)v13)
                    {
                        return v4;
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        dcgettext(NULL, "Unexpected EOF in snapshot file", 5);
                        sub_44aa70(g_484b28);
                        error(0, 0, "%s: %s");
                        sub_4163d0(); /* do not return */
                    }
                }
            }
            error(0, 0, v15);
            sub_4163d0(); /* do not return */
        }
        v5 = v11;
LABEL_41ea7b:
        v13 += 1;
        ch = getc_unlocked(a0);
        v11 = ch;
    }
}



// Function: read_timespec @ 0x1ed70
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern long long g_484b28;
extern struct_0 *g_484d50;

void read_timespec(FILE *a0, unsigned long long *a1)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]

    if ((char)sub_41ea10(a0, "sec", -0x8000000000000000, 0x7fffffffffffffff, &v0) && (char)sub_41ea10(a0, "nsec", 0, 0x3b9ac9ff, &v1))
    {
        *(a1) = v0;
        a1[1] = v1;
        return;
    }
    if (g_484d50)
        g_484d50();
    dcgettext(NULL, "Unexpected EOF in snapshot file", 5);
    sub_44aa70(g_484b28);
    error(0, 0, "%s: %s");
    sub_4163d0(); /* do not return */
}



// Function: read_incr_db_2 @ 0x1ee50
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct FILE {
} FILE;

extern FILE *g_4843a0;
extern unsigned long long g_484b00;
extern long long g_484b28;
extern struct_0 *g_484d50;

void read_incr_db_2(void)
{
    unsigned long long v18;  // rax
    unsigned long long v19;  // rax
    unsigned long long v20;  // r10
    unsigned int v0;  // [bp-0xec]
    unsigned long long v1;  // [bp-0xe8]
    char *v2;  // [bp-0xe0]
    unsigned long long v3;  // [bp-0xd8]
    unsigned long long v4;  // [bp-0xd0]
    unsigned int v5;  // [bp-0xc8]
    unsigned int v6;  // [bp-0xc0]
    struct_0 v7;  // [bp-0xb8]
    unsigned long long v8;  // [bp-0xb0]
    unsigned long long v9;  // [bp-0xa8]
    unsigned long long v10;  // [bp-0xa8]
    unsigned long long v11;  // [bp-0xa8]
    unsigned long long v12;  // [bp-0xa0]
    unsigned long long v13;  // [bp-0x98]
    unsigned long v14;  // [bp-0x88]
    char v15;  // [bp-0x68]
    char v16;  // [bp-0x58]

    _obstack_begin(&v7, 0, 0, sub_44c5b0, free);
    sub_41ed70(g_4843a0, &g_484b00);
    v2 = &v5;
    while ((char)sub_41ea10(g_4843a0, "nfs", 0, 1, &v3))
    {
        *((char *)&v0) = v3;
        sub_41ed70(g_4843a0, v2);
        if ((char)sub_41ea10(g_4843a0, "dev", 0, -0x1, &v3) && (char)sub_41ea10(g_4843a0, "ino", 0, -0x1, &v3) && !sub_41e960(g_4843a0, &v7, &v4))
        {
            if (v12 == v11)
                v15 |= 2;
            v18 = v12 + v14 & ~(v14);
            if (v18 - v8 > v13 - v8)
                v18 = v13;
            v12 = v18;
            v9 = v18;
            do
            { } while (!sub_41e960(g_4843a0, &v7, &v4) && v4 > 1);
            if (!getc_unlocked(g_4843a0))
            {
                if (v12 == v9)
                    v15 |= 2;
                v19 = v12 + v14 & ~(v14);
                if (v19 - v8 > v13 - v8)
                    v19 = v13;
                v12 = v19;
                v10 = v19;
                v1 = v9;
                sub_41dac0(v11, v5, v6, v3, v3, (char)v0 & 1, 0, v9);
                v20 = v1;
                if (v8 < v20 && v13 > v20)
                {
                    v11 = v20;
                    v12 = v20;
                }
                else
                {
                    _obstack_free(&v7, v20);
                    v11 = v10;
                }
            }
            else
            {
                if (g_484d50)
                    g_484d50();
                dcgettext(NULL, "Missing record terminator", 5);
                sub_442ff0(ftello(g_4843a0), &v16);
                sub_44aa70(g_484b28);
                error(0, 0, dcgettext(NULL, "%s: byte %s: %s", 5));
                sub_4163d0(); /* do not return */
            }
        }
        else
        {
            if (g_484d50)
                g_484d50();
            dcgettext(NULL, "Unexpected EOF in snapshot file", 5);
            sub_44aa70(g_484b28);
            error(0, 0, "%s: %s");
            sub_4163d0(); /* do not return */
        }
    }
    return;
}



// Function: write_directory_file_entry @ 0x1f1c0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern void g_461eec;

int write_directory_file_entry(void)
{
    void* v3;  // rdi
    char v4;  // al
    void* v13;  // rax
    unsigned long len1;  // rax
    void* i;  // rax
    void* v18;  // r13
    FILE *v19;  // rsi
    void* v5;  // rdi
    FILE *v6;  // rsi
    void* v7;  // rax
    void* v9;  // rax
    void* v11;  // rax
    void* v0;  // [bp-0x50], Other Possible Types: struct_0 *
    char v1;  // [bp-0x48]

    v4 = (int)v3[60];
    if (!(v4 & 4))
    {
        ferror_unlocked(v19);
        return;
    }
    v5 = "1";
    if (!(v4 & 2))
        v5 = "0";
    fwrite_unlocked(v5, 2, 1, v6);
    v7 = sub_424080((long long)v3[8], -0x8000000000000000, 0x7fffffffffffffff, &v1);
    fwrite_unlocked(v7, strlen(v7) + 1, 1, v6);
    v9 = sub_442f50((long long)v3[16], &v1);
    fwrite_unlocked(v9, strlen(v9) + 1, 1, v6);
    v11 = sub_424080((long long)v3[24], 0, -0x1, &v1);
    fwrite_unlocked(v11, strlen(v11) + 1, 1, v6);
    v13 = sub_424080((long long)v3[32], 0, -0x1, &v1);
    fwrite_unlocked(v13, strlen(v13) + 1, 1, v6);
    len1 = strlen((long long)v3[88]);
    fwrite_unlocked((long long)v3[88], len1 + 1, 1, v6);
    if ((long long)v3[40])
    {
        i = sub_41d220((long long)v3[40], 0, &v0);
        if (i)
        {
            do
            {
                fwrite_unlocked(i, strlen(i) + 1, 1, v6);
                i = sub_41d110(v0);
            } while (i);
        }
        else
        {
            v18 = v0;
        }
        free(v18);
    }
    fwrite_unlocked(&g_461eec, 2, 1, v6);
    ferror_unlocked(v6);
    return;
}



// Function: get_gnu_dumpdir @ 0x1f3c0
typedef struct struct_0 {
    char padding_0[136];
    char field_88;
    char padding_89[256];
    char field_189;
    char padding_18a[6];
    void* field_190;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned int g_484408;
extern struct_1 *g_484d50;

void* get_gnu_dumpdir(struct_0 *idx)
{
    unsigned long v2;  // rbp
    void* iter;  // rax
    unsigned long i;  // rbp
    unsigned long v5;  // r14
    unsigned long n;  // rax
    unsigned long v7;  // rbx
    void* v0;  // [bp-0x40]

    v2 = *((long long *)&idx->field_88);
    iter = sub_44c5b0(v2);
    v0 = iter;
    sub_40d1f0(*((long long *)&g_484408));
    sub_40ccb0(idx);
    if (v2)
    {
        do
        {
            i = v2;
            sub_40ccf0(i);
            v5 = sub_40e130();
            if (!v5)
            {
                if (g_484d50)
                    g_484d50();
                error(1, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
            }
            n = sub_40d2b0(v5);
            if (n > i)
                n = i;
            v7 = n;
            memcpy(iter, v5, n);
            iter += v7;
            sub_40d1f0(v5 + v7 - 1);
            v2 = i - v7;
        } while (i != v7);
    }
    sub_40ccd0();
    idx->field_189 = 1;
    idx->field_190 = v0;
    return v0;
}



// Function: dumpdir_size @ 0x1f4c0
unsigned long long dumpdir_size(char *a0)
{
    unsigned long long v2;  // rbp
    unsigned long long v3;  // rbx
    unsigned long long v4;  // rbp
    unsigned long v5;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (!*(a0))
        return 1;
    v2 = 0;
    v0 = v3;
    do
    {
        v4 = v2;
        v5 = strlen(a0) + 1;
        a0 = &a0[v5];
        v2 = v4 + v5;
    } while (*(a0));
    return v4 + v5 + 1;
}



// Function: clear_directory_table @ 0x1f510
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern long long g_4843a8;
extern long long g_4843b0;
extern unsigned long long g_4843b8;
extern struct_0 *g_4843c0;

void clear_directory_table(void)
{
    struct_0 **v1;  // rbx
    struct_0 **v2;  // rbx
    struct_0 **v3;  // rbx

    if (g_4843b0)
        sub_441c30(g_4843b0);
    if (g_4843a8)
        sub_441c30(g_4843a8);
    v1 = g_4843c0;
    if (g_4843c0)
    {
        do
        {
            v2 = v1;
            v3 = *(v2);
            sub_41d180(v2);
            v1 = v3;
        } while (*(v2));
    }
    g_4843b8 = 0;
    g_4843c0 = 0;
    return;
}



// Function: rebase_directory @ 0x1f570
void rebase_directory(long long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4)
{
    sub_423fc0(a0 + 88);
    return;
}



// Function: update_parent_directory @ 0x1f580
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[420];
    int field_1a8;
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

typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    char padding_18[64];
    long long field_58;
} struct_1;

void update_parent_directory(struct_0 *a0)
{
    struct_1 *v2;  // rax
    struct_1 *idx;  // rbx
    unsigned long v5;  // rdx
    stat v0;  // [bp-0xb8]

    v2 = sub_41d310(*((long long *)&a0->field_0));
    if (!v2)
        return;
    idx = v2;
    if (fstat(a0->field_1a8, &v0))
    {
        sub_424ec0(idx->field_58);
        return;
    }
    idx->field_8 = sub_44bd20(&v0);
    idx->field_10 = v5;
    return;
}



// Function: scan_directory @ 0x1f610
typedef struct struct_0 {
    char padding_0[408];
    struct struct_0 *field_198;
    char padding_1a0[8];
    int field_1a8;
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

extern unsigned int g_484800;
extern unsigned int g_484808;
extern char g_484ad1;
extern unsigned int g_484ad4;
extern unsigned long long g_484b00;
extern unsigned long long g_484b08;
extern unsigned int g_484b8c;

struct_0 * scan_directory(struct_0 *idx)
{
    unsigned long v12;  // r15
    unsigned long ptr;  // r14
    char *v22;  // r9
    int v23;  // eax
    unsigned int v24;  // eax
    long long v25;  // rax
    long long v26;  // rdx
    unsigned int v27;  // r11d
    void* v28;  // r15
    unsigned long v29;  // r13
    unsigned int v30;  // edx
    struct_0 *v31;  // r12
    unsigned long v14;  // r13
    unsigned int v15;  // edx
    struct_0 *v16;  // r12
    long long v17;  // r13
    char *v18;  // rbx
    char *v19;  // r15
    unsigned long v20;  // rax
    unsigned long v21;  // r10
    char *v0;  // [bp-0x238], Other Possible Types: unsigned int, unsigned long
    unsigned long v1;  // [bp-0x230]
    unsigned long v2;  // [bp-0x220]
    char *v3;  // [bp-0x218]
    char *v4;  // [bp-0x210], Other Possible Types: unsigned int, unsigned long
    char v5;  // [bp-0x201]
    void* v6;  // [bp-0x200]
    char v7;  // [bp-0x1f8]
    stat v8;  // [bp-0x1a0]
    struct_0 *v9;  // [bp-0x60]
    int v10;  // [bp-0x50]

    v12 = *((long long *)&idx->padding_0[0]);
    ptr = sub_413fe0(idx);
    v2 = *((long long *)&idx->padding_0[88]);
    if (ptr)
    {
        sub_416890(idx);
        v14 = sub_44c7a0(v12);
        sub_423e60(v14);
        v15 = 0;
        if (!idx->field_198)
            v15 = 32;
        v16 = sub_41dbe0(v14, idx, v15, &v5);
        free(v14);
        v17 = sub_425040(v12);
        if (!*((int *)&v16->padding_0[56]))
        {
            if (*((long long *)&v16->padding_0[72]))
                sub_41d800(v16);
            sub_4250b0(v17);
            free(ptr);
            return v16;
        }
        else
        {
            sub_41d600(v16, ptr);
            v18 = sub_41d220(*((long long *)&v16->padding_0[40]), 1, &v6);
            if (v18)
            {
                v3 = &v7;
                do
                {
                    v19 = v18 + 1;
                    v20 = sub_4250d0(v17, v19);
                    if (*(v18) == 73 || (v0 = v20, (char)sub_416b00(v20, idx)))
                    {
                        *(v18) = 78;
                        continue;
                    }
                    v4 = v0;
                    v0 = idx->field_1a8;
                    sub_42f630(v3);
                    if (v0 < 0)
                    {
                        v4 = v0;
                        v0 = v4;
                        v21 = v0;
                        *(__errno_location()) = -(v4);
                        goto LABEL_41f7d3;
                    }
                    v4 = v0;
                    v0 = &v8;
                    v22 = v0;
                    v21 = v4;
                    if (fstatat(v0, v19, &v8, g_484800))
                    {
LABEL_41f84a:
                        goto LABEL_41f7d3;
                    }
                    else
                    {
                        if (((unsigned short)v8.st_nlink & 0xf000) != 0x4000)
                        {
LABEL_41f8cb:
                            if (!g_484ad1 || *((unsigned long long *)&v8) == v2)
                            {
                                if (*(v18) == 89)
                                    goto LABEL_41f7e0;
                                v4 = v22;
                                v25 = sub_44bd20(v22);
                                v0 = g_484b00;
                                v1 = g_484b08;
                                if (sub_44be80(v25, v26, g_484b00, g_484b08) >= 0 || g_484b8c && sub_44be80(sub_44bd10(v4), v26, v0, v1) >= 0)
                                {
                                    *(v18) = 89;
                                    goto LABEL_41f7e0;
                                }
                            }
                            *(v18) = 78;
                            goto LABEL_41f7e0;
                        }
                        v0 = v21;
                        v4 = v22;
                        v23 = sub_4140d0(idx, v19, g_484808);
                        v21 = v0;
                        if (v23 < 0)
                        {
LABEL_41f7d3:
                            sub_424ef0(v21, 0);
                            *(v18) = 78;
                            goto LABEL_41f7e0;
                        }
                        v10 = v23;
                        v24 = fstat(v23, v4);
                        v21 = v0;
                        if (v24)
                            goto LABEL_41f84a;
                        v22 = v4;
                        if ((v8.st_nlink & 0xf000) != 0x4000)
                            goto LABEL_41f8cb;
                        v27 = (!g_484ad4 ? 16 : (*((int *)&v16->padding_0[56]) == 2 ? 18 : v24));
                        *(v18) = 0x44;
                        v9 = idx;
                        sub_41dbe0(v21, v3, v27, v18);
                        sub_414200(v3);
LABEL_41f7e0:
                        sub_42f6f0(v3);
                    }
                } while ((v28 = v6, v18 = sub_41d110(v28), v18));
            }
            else
            {
                v28 = v6;
            }
            free(v28);
            sub_4250b0(v17);
            free(ptr);
            return v16;
        }
    }
    else
    {
        sub_432ac0(v12);
        sub_416890(idx);
        v29 = sub_44c7a0(v12);
        sub_423e60(v29);
        v30 = 0;
        if (!idx->field_198)
            v30 = 32;
        v31 = sub_41dbe0(v29, idx, v30, &v5);
        free(v29);
        sub_4250b0(sub_425040(v12));
        free(ptr);
        return v31;
    }
}



// Function: directory_contents @ 0x1fa60
typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

struct_0 ** directory_contents(struct_1 *a0)
{
    struct_0 **v1;  // rax

    if (!a0)
        return NULL;
    v1 = a0->field_28;
    if (!v1)
        return v1;
    return *(v1);
}



// Function: safe_directory_contents @ 0x1fa90
typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 **g_461eef;

struct_0 *** safe_directory_contents(struct_1 *a0)
{
    struct_0 ***v1;  // rax

    v1 = sub_41fa60(a0);
    if (v1)
        return v1;
    return &g_461eef;
}



// Function: show_snapshot_field_ranges @ 0x1fc70
extern void g_47f600;

void show_snapshot_field_ranges(void)
{
    void* iter;  // rbx
    long long v4;  // rsi
    long long v5;  // rdi
    char v0;  // [bp-0x68]
    char v1;  // [bp-0x48]

    iter = &g_47f600;
    __printf_chk(1, "This tar's snapshot file field ranges are\n");
    __printf_chk(1, "   (%-15s => [ %s, %s ]):\n\n", "field name", "min");
    v4 = 0;
    v5 = 1;
    while (1)
    {
        iter += 24;
        sub_424080(v5, v4, v5, &v1);
        sub_424080(*((long long *)((char *)iter - 16)), *((long long *)((char *)iter - 16)), *((long long *)((char *)iter - 8)), &v0);
        __printf_chk(1, "    %-15s => [ %s, %s ],\n");
        if (!*((long long *)iter))
            break;
        v5 = (long long)iter[16];
        v4 = (long long)iter[8];
    }
    __printf_chk(1, "\n");
    return;
}



// Function: read_directory_file @ 0x1fd80
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern FILE *g_4843a0;
extern unsigned int g_484b24;
extern void g_484b28;
extern struct_0 *g_484d50;

void read_directory_file(void)
{
    unsigned int v3;  // eax
    long long v4;  // rdi
    char v13;  // al
    unsigned long long v15;  // rax
    unsigned int v17;  // edi
    long long v5;  // rsi
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9
    char *v10;  // rbp
    char *iter;  // rbx
    char *v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]

    v0 = NULL;
    v1 = 0;
    v3 = open(*((long long *)&g_484b28), (-(g_484b24 < 1) & 0x200) + 66, 438);
    if (v3 < 0)
    {
        sub_4327c0(*((long long *)&g_484b28));
        return;
    }
    g_4843a0 = fdopen(v3, "r+");
    if (!g_4843a0)
    {
        sub_4327c0(*((long long *)&g_484b28));
        close(v3);
        return;
    }
    sub_427b00();
    sub_427b80(v4, v5, v6, v7, v8, v9);
    if (getline(&v0, &v1, g_4843a0) > NULL)
    {
        v10 = v0;
        if (!strncmp(v10, "GNU tar", 7))
        {
            iter = v10 + 8;
            if (v10[7] != 45)
            {
                if (g_484d50)
                    g_484d50();
                error(1, 0, dcgettext(NULL, "Bad incremental file format", 5));
            }
            v13 = v10[8];
            if (v10[8] != 45)
            {
                do
                {
                    if (!v13)
                    {
                        if (g_484d50)
                            g_484d50();
                        error(1, 0, dcgettext(NULL, "Bad incremental file format", 5));
                    }
                } while ((v13 = iter[1], iter += 1, v13 != 45));
            }
            v15 = strtoumax(iter + 1, 0, 10);
            if (v15 > 1)
            {
                if (v15 == 2)
                {
                    sub_41ee50();
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    error(1, 0, dcgettext(NULL, "Unsupported incremental format version: %lu", 5));
                }
                goto LABEL_41ff10;
            }
            else
            {
                v10 = v0;
                v17 = v15;
            }
        }
        else
        {
            v17 = 0;
        }
        sub_41e470(v17, v10);
        if (ferror_unlocked(g_4843a0))
        {
            sub_432820(*((long long *)&g_484b28));
            free(v0);
            return;
        }
        free(v0);
        return;
    }
    else
    {
LABEL_41ff10:
        if (ferror_unlocked(g_4843a0))
        {
            sub_432820(*((long long *)&g_484b28));
            free(v0);
            return;
        }
        free(v0);
        return;
    }
}



// Function: is_dumpdir @ 0x201b0
typedef struct struct_0 {
    char padding_0[136];
    char field_88;
    char padding_89[256];
    char field_189;
    char padding_18a[6];
    void* field_190;
} struct_0;

long long is_dumpdir(struct_0 *a0)
{
    unsigned long v1;  // rax

    v1 = a0->padding_89[0xff];
    if (!(char)v1)
    {
        return v1;
    }
    else if (a0->field_190)
    {
        return v1;
    }
    else
    {
        sub_41f3c0(a0);
        return a0->padding_89[0xff];
    }
}



// Function: try_purge_directory @ 0x201f0
typedef struct struct_0 {
    char padding_0[136];
    char field_88;
    char padding_89[256];
    char field_189;
    char padding_18a[6];
    void* field_190;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned long long g_483f28;
extern char g_4847f0;
extern struct_0 g_484840;
extern struct_0 *g_4849d0;
extern unsigned int g_484a54;
extern char g_484ad1;
extern char g_484b40;
extern char g_484b92;
extern struct_1 *g_484d50;
extern unsigned int g_484d58;
extern unsigned long long g_484dd0;

unsigned int try_purge_directory(long long a0)
{
    long long v6;  // r12
    unsigned int v7;  // r13d
    unsigned long len;  // r14
    unsigned long long idx;  // rax
    char *v18;  // rax
    int *err;  // rax
    unsigned long ptr;  // rbp
    char *v21;  // rbx
    struct_0 *v22;  // r15
    char *v23;  // rsi
    char *v24;  // rax
    char *ptr1;  // rbx
    char *v26;  // rax
    int *err1;  // rax
    char *v28;  // rax
    char *ptr2;  // rbp
    char v10;  // r15b
    unsigned long len1;  // rax
    char *v12;  // r12
    char *v13;  // r14
    char *v14;  // r15
    char *v15;  // r15
    char *v0;  // [bp-0xf0]
    char v1;  // [bp-0xe8]
    unsigned long v2;  // [bp-0xe0], Other Possible Types: unsigned long long
    char v3;  // [bp-0xd8]
    unsigned int v4;  // [bp-0xc0]

    v6 = a0;
    if (!(char)sub_4201b0(&g_484840.field_0))
        return 0;
    v0 = sub_4253c0(v6, 0);
    if (v0)
    {
        v7 = sub_41d860(g_4849d0);
        if ((char)v7)
        {
            ptr1 = g_4849d0;
            ptr2 = NULL;
            v10 = g_4849d0->field_0;
            if (v10)
            {
                v6 = v6;
                do
                {
                    if (v10 != 88)
                    {
                        len1 = strlen(ptr1);
                        if (v10 != 82)
                            continue;
                        v12 = &ptr1[len1 + 1];
                        v13 = ptr1 + 1;
                        v14 = v12 + 1;
                        if (ptr1[1])
                            v13 = sub_433020(v13, 0, g_484b92);
                        if (v12[1])
                            v14 = sub_433020(v14, 0, g_484b92);
                        if (!*(v13))
                        {
                            v13 = ptr2;
                        }
                        else if (!*(v14))
                        {
                            v14 = ptr2;
                        }
                        if ((char)sub_419ec0(v13, v14))
                        {
                            ptr1 = v12;
                            continue;
                        }
                        goto LABEL_4203d9;
                    }
                    else
                    {
                        v15 = ptr1 + 1;
                        len = strlen(v15);
                        ptr2 = sub_44c610(ptr2, len + 12);
                        memcpy(ptr2, v15, len);
                        idx = &ptr2[len + 1];
                        ptr2[len] = 47;
                        *((unsigned long long *)idx) = 6365935209047810420;
                        *((unsigned short *)(idx + 8)) = 0x5858;
                        *((char *)(idx + 10)) = 0;
                        if (!mkdtemp(ptr2))
                        {
                            if (g_484d50)
                                g_484d50();
                            sub_44ac50(ptr2);
                            v18 = dcgettext(NULL, "Cannot create temporary directory using template %s", 5);
                            err = __errno_location();
                            error(0, *(err), v18);
                            g_484d58 = 2;
LABEL_4203d9:
                            free(ptr2);
                            free(v0);
                            return 0;
                        }
                    }
                } while ((ptr1 = ptr1 + strlen(ptr1) + 1, v10 = *(ptr1), *(ptr1)));
            }
            ptr = 0;
            free(ptr2);
            v21 = v0;
            v22 = sub_41d7f0(g_4849d0);
            if (*(v21))
            {
                v1 = v7;
                do
                {
                    free(ptr);
                    ptr = sub_427bc0(v6, v21);
                    if (sub_4246f0(ptr, &v3))
                    {
                        if (*(__errno_location()) == 2)
                            continue;
                        sub_424ec0(ptr);
                        if (g_484d50)
                            g_484d50();
                        sub_44aa70(ptr);
                        v23 = "%s: Not purging directory: unable to stat";
                        goto LABEL_4204b7;
                    }
                    else
                    {
                        v24 = sub_41d450(v22, v21);
                        if (v24)
                        {
                            if (*(v24) != 0x44)
                            {
                                if (*(v24) != 89 || ((unsigned short)v4 & 0xf000) != 0x4000)
                                    continue;
                            }
                            else
                            {
                                if ((v4 & 0xf000) == 0x4000)
                                    continue;
                            }
                        }
                        if (g_484ad1 && v3 != *((long long *)&g_4847f0))
                        {
                            if (g_484d50)
                                g_484d50();
                            sub_44aa70(ptr);
                            v23 = "%s: directory is on a different device: not purging";
LABEL_4204b7:
                            error(0, 0, dcgettext(NULL, v23, 5));
                        }
                        else if (!g_484b40 || sub_42ca20("delete", ptr))
                        {
                            if (g_484a54)
                            {
                                sub_44ac50(ptr);
                                v2 = g_484dd0;
                                v26 = dcgettext(NULL, "%s: Deleting %s\n", 5);
                                __fprintf_chk(g_483f28, 0x1, v26);
                            }
                            if (!sub_425490(ptr, 1))
                            {
                                err1 = __errno_location();
                                if (g_484d50)
                                    g_484d50();
                                v2 = sub_44aa70(ptr);
                                v28 = dcgettext(NULL, "%s: Cannot remove", 5);
                                error(0, *(err1), v28);
                                g_484d58 = 2;
                            }
                        }
                    }
                } while ((v21 = v21 + strlen(v21) + 1, *(v21)));
                v7 = v1;
            }
            free(ptr);
            sub_41d1a0(v22);
            free(v0);
            return v7;
        }
    }
    return 0;
}



// Function: purge_directory @ 0x206e0
unsigned int purge_directory(long long a0)
{
    unsigned int v1;  // eax

    v1 = sub_4201f0(a0);
    if (!(char)v1)
        return sub_422bd0();
    return v1;
}



// Function: list_dumpdir @ 0x20710
typedef struct FILE {
} FILE;

extern FILE *g_483f28;

int list_dumpdir(char *a0, unsigned long long a1)
{
    char *v2;  // rbp
    unsigned int v3;  // r12d
    unsigned int v4;  // ecx
    unsigned long v5;  // rax
    int v6;  // eax
    int v7;  // eax
    int v8;  // eax
    unsigned long v0;  // [bp-0x158]

    v2 = &a0[a1];
    if (!a1)
        return v8;
    v3 = 0;
    while (1)
    {
        v4 = *(a0);
        a0 += 1;
        if ((char)v4)
        {
            v5 = v4 - 0x44;
            if ((char)v5 > 21 || (v0 = 3228673, !(((char)v5 & 254 | *((char *)((char *)&v0 + ((long long)(v5 & 63) >> 3))) >> (unsigned long long)((char)v5 & 63 & 7) & 1) & 1)))
            {
                v6 = fputc_unlocked(v4, g_483f28);
            }
            else
            {
                v6 = __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, "%c");
                if (!v3)
                    v6 = __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, " ");
                v3 = 1;
            }
            if (a0 == v2)
                return v6;
        }
        else
        {
            v3 = 0;
            v7 = fputc_unlocked(10, g_483f28);
            if (a0 == v2)
                return v7;
        }
    }
}



// Function: decode_xform @ 0x207f0
extern unsigned long long g_484a90;

long long decode_xform(unsigned long a0, unsigned int *a1)
{
    unsigned long len;  // rax

    switch (*(a1))
    {
    case 2:
        a0 = sub_433020();
        if (!g_484a90)
            return a0;
    case 4:
        return a0;
    case 1:
        a0 = sub_433020();
        goto LABEL_42080a;
    default:
LABEL_42080a:
        if (!g_484a90)
            return a0;
    }
    len = sub_427c50(a0);
    if (len == 0xffffffffffffffff)
        len = strlen(a0);
    return a0 + len;
}



// Function: from_header @ 0x20890
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern long long g_484448;
extern char g_484450;
extern char g_484460;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned long long from_header(void* a0, unsigned long n, unsigned long long a2, unsigned long long a3, long long a4, unsigned int a5, char a6)
{
    void* i;  // rbx
    void* v10;  // rcx
    unsigned int v18;  // edx
    char v19;  // dil
    unsigned int v20;  // edx
    unsigned long long v21;  // r14
    long long v22;  // r10, Other Possible Types: unsigned long long
    unsigned int v23;  // edi
    unsigned int v24;  // eax
    unsigned long long v25;  // r14
    unsigned long long *iter;  // r11, Other Possible Types: void*, unsigned long, unsigned long long
    unsigned long v28;  // rax
    void* v31;  // rcx
    char v32;  // dil
    unsigned long v33;  // rdx
    unsigned long v11;  // rdx
    unsigned long v34;  // rax
    unsigned long v35;  // rax
    char *j;  // rcx
    unsigned long long v37;  // rax
    unsigned long long v38;  // r14
    unsigned int v12;  // eax
    void* v41;  // rcx
    unsigned int v42;  // edi
    void* v43;  // rcx
    void* v44;  // rbx
    unsigned long long v45;  // rax
    unsigned int v13;  // r14d
    char *v47;  // rsi
    void* iter1;  // r9
    unsigned long long v15;  // r14
    unsigned long long v16;  // r8
    void* node;  // r15
    char v0;  // [bp-0x2498]
    char v1;  // [bp-0x1498]
    unsigned long long v2;  // [bp-0x4a8]
    char v3;  // [bp-0x498]
    void* v4;  // [bp-0x490]
    char v5;  // [bp-0x468]
    char v6;  // [bp-0x447]
    char v7;  // [bp-0x428]
    char v8;  // [bp-0x427]

    i = a0 + n;
    v22 = a4;
    *((unsigned int *)&iter) = a5;
    v10 = a0 + (*((char *)a0) < 1);
    if (i != v10)
    {
        iter = v10;
        iter = __ctype_b_loc();
        while (1)
        {
            v11 = *((char *)iter);
            v12 = v11;
            if (!(*((char *)(*(iter) + v11 * 2 + 1)) & 32))
                break;
            iter += 1;
            if (i == iter)
                goto LABEL_420a30;
        }
        iter = -(a3);
        v13 = (char)v11 - 48;
        if (v13 <= 7)
        {
            iter1 = iter + 1;
            v15 = v13;
            if (i != iter1)
            {
                v16 = 0;
                node = iter1;
                do
                {
                    v18 = *((char *)node);
                    v19 = v18;
                    if (v18 - 48 > 7)
                    {
                        if (v22 < v15 || (char)v16)
                            goto LABEL_420cce;
                        if (i == node || !v19)
                            return sub_423b90(v15);
                        goto LABEL_4209ca;
                    }
                } while ((v21 = (long long)(int)v20 + v15 * 8, node += 1, v16 = (unsigned long long)((unsigned int)v16 | (unsigned int)_INSERT(v15 & 0x1fffffffffffffff, 0, (char)(char)((v15 & 0x1fffffffffffffff) != v15))), v15 = v21, i != node));
            }
            else
            {
                v16 = 0;
                node = i;
                v21 = v15;
            }
            v15 = v21;
            if (v22 >= v15 && !(char)v16)
                return sub_423b90(v15);
LABEL_420cce:
            if ((char)v12 > 49)
            {
                if (a2)
                {
                    v22 = 7 - ((char)(v12 - 48) | 4);
                    if (i != iter1)
                    {
                        v23 = 0;
                        v16 = 7;
                        do
                        {
                            if (*((char *)iter1) - 48 > 7)
                                goto LABEL_420d4e;
                        } while ((iter1 += 1, v23 |= (unsigned int)_INSERT(v22 & 0x1fffffffffffffff, 0, (char)(char)((v22 & 0x1fffffffffffffff) != v22)), v22 = (long long)(int)(unsigned int)(7 - (unsigned long long)v24) + v22 * 8, i != iter1));
                        iter1 = i;
                    }
                    else
                    {
                        v23 = 0;
                        iter1 = i;
                    }
LABEL_420d4e:
                    v25 = v22;
                    v15 = v25 + 1;
                    v16 = _INSERT(v16, 0, v25 == 0xffffffffffffffff) | v23;
                    if ((char)v16 != 1)
                    {
                        node = iter1;
                        if (iter >= v15)
                        {
                            if (!a6)
                            {
                                if (g_484d50)
                                    g_484d50();
                                error(0, 0, dcgettext(NULL, "Archive octal value %.*s is out of %s range; assuming two's complement", 5));
                            }
                            if (!(i != iter1 && (v28 = (unsigned long)(unsigned long long)(char)*((char *)iter1), (char)v28)))
                            {
                                return sub_423b90(~(v22));
                            }
                            else if (*((char *)(*(iter) + v28 * 2 + 1)) & 32)
                            {
                                return sub_423b90(-(v15));
                            }
LABEL_420a9a:
                            if (!g_484448)
                            {
                                g_484448 = sub_44a530(0);
                                sub_44a590(g_484448, 8);
                            }
                            for (; a0 != i && !*((char *)i - 1); i -= 1);
                            sub_44a650(&v7, 1000, a0, i - a0, g_484448);
                            if (a6)
                                return 0xffffffffffffffff;
                            if (g_484d50)
                                g_484d50();
                            error(0, 0, dcgettext(NULL, "Archive contains %.*s where numeric %s value expected", 5));
                            g_484d58 = 2;
                            return 0xffffffffffffffff;
                        }
                    }
                    else
                    {
LABEL_4210d1:
                        if (!a2)
                        {
                            return 0xffffffffffffffff;
                        }
                        else if (a6 == 1)
                        {
                            return 0xffffffffffffffff;
                        }
                        else
                        {
                            if (g_484d50)
                                g_484d50();
                            error(0, 0, dcgettext(NULL, "Archive octal value %.*s is out of %s range", 5));
                            g_484d58 = 2;
                            return 0xffffffffffffffff;
                        }
                    }
                }
                else
                {
                    if ((char)v16)
                        return 0xffffffffffffffff;
LABEL_420e44:
                    if (i == node || (v19 = *((char *)node), !*((char *)node)))
                        goto LABEL_4209e0;
LABEL_4209ca:
                    if (*((char *)(*(iter) + v19 * 2 + 1)) & 32)
                        goto LABEL_4209e0;
                    goto LABEL_420a91;
                }
            }
            if ((char)v16)
                goto LABEL_4210d1;
            goto LABEL_420e44;
        }
        if ((char)iter)
        {
LABEL_420a89:
            if (!(char)v12)
                return sub_423b90(0);
            goto LABEL_420a91;
        }
        if (!((char)(v11 - 43) & 253))
        {
            if (!a6 && !g_484450)
            {
                g_484450 = 1;
                if (g_484d50)
                {
                    v4 = iter;
                    g_484d50();
                    iter = v4;
                }
                v4 = iter;
                error(0, 0, dcgettext(NULL, "Archive contains obsolescent base-64 headers", 5));
                iter = v4;
                v12 = *((char *)iter);
            }
            v31 = iter + 1;
            v32 = (char)v12 == 45;
            v15 = 0;
            if (i == v31)
                return sub_423b90(0);
            while (1)
            {
                v33 = *((char *)v31);
                v34 = (&g_484460)[v33];
                if ((char)v34 > 63)
                    break;
                if ((v15 & 0x3ffffffffffffff) != v15)
                {
                    v35 = n + 24;
                    j = &(&v3)[-1 * (v35 & 0xfffffffffffff000)];
                    if (&v3 != j)
                    {
                        do
                        { } while (&v0 != j);
                    }
                    v37 = (unsigned int)v35 & 0xfffffff0 & 0xfff;
                    /* unsupported instruction */ = (int)&(&v1)[-1 * v37];
                    if (v37)
                        *((long long *)(/* unsupported instruction */ + v37 - 8)) = *((long long *)(/* unsupported instruction */ + v37 - 8));
                    v38 = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
                    memcpy(v38, a0, n);
                    *((char *)(v38 + n)) = 0;
                    if (a6 == 1)
                    {
                        return 0xffffffffffffffff;
                    }
                    else if (!a2)
                    {
                        return 0xffffffffffffffff;
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        sub_44ac50(v38);
                        error(0, 0, dcgettext(NULL, "Archive signed base-64 string %s is out of %s range", 5));
                        g_484d58 = 2;
                        return 0xffffffffffffffff;
                    }
                }
                v31 += 1;
                v15 = v15 * 64 | v34;
                if (i == v31)
                    goto LABEL_420bd5;
            }
            if (v31 == i || !(char)v33 || *((char *)(*(iter) + v33 * 2 + 1)) & 32)
                goto LABEL_420bd5;
LABEL_420a91:
            if (!a2)
                return 0xffffffffffffffff;
            goto LABEL_420a9a;
        }
        if ((char)v12 != 128 && (char)v12 != 0xff)
            goto LABEL_420a89;
        v41 = iter + 1;
        v42 = v12 & 64;
        v15 = (v12 & 63) - v42;
        do
        {
            v43 = v41 + 1;
            v15 = v15 * 0x100 + *((char *)v41);
            if (i == v43)
            {
                v32 = v42;
                if ((char)v12 & 64)
                    v15 = -(v15);
                goto LABEL_420bd5;
            }
        } while ((v41 = v43, (v15 & 0xffffffffffffff | (unsigned long long)-(v42) * 0x4000000000000) == v15));
        if (!a2)
            return 0xffffffffffffffff;
        if (a6 == 1)
            return 0xffffffffffffffff;
        if (g_484d50)
            g_484d50();
        v47 = "Archive base-256 value is out of %s range";
        goto LABEL_420a58;
LABEL_420bd5:
        if (v32)
        {
            if (iter >= v15)
                return sub_423b90(-(v15));
            if (a6 == 1)
                return 0xffffffffffffffff;
            if (!a2)
                return 0xffffffffffffffff;
            v44 = sub_443090(iter, &v6);
            *((char *)(sub_443090(v15, &v8) - 1)) = 45;
LABEL_420c2e:
            if (iter)
                *((char *)v44 - 1) = 45;
            if (g_484d50)
                g_484d50();
            v45 = sub_443090(v22, &v5);
            v2 = v45;
            error(0, 0, dcgettext(NULL, "Archive value %s is out of %s range %s..%s", 5));
            g_484d58 = 2;
            return 0xffffffffffffffff;
        }
LABEL_4209e0:
        if (v22 >= v15)
        {
            return sub_423b90(v15);
        }
        else if (a2)
        {
            if (a6 == 1)
                return 0xffffffffffffffff;
            v44 = sub_443090(iter, &v6);
            sub_443090(v15, &v8);
            goto LABEL_420c2e;
        }
        else
        {
            return 0xffffffffffffffff;
        }
        goto LABEL_420c2e;
    }
    else
    {
LABEL_420a30:
        if (a2 && a6 != 1)
        {
            if (g_484d50)
                g_484d50();
            v47 = "Blanks in header where numeric %s value expected";
LABEL_420a58:
            error(0, 0, dcgettext(NULL, v47, 5));
            g_484d58 = 2;
        }
    }
    return 0xffffffffffffffff;
}



// Function: base64_init @ 0x21160
extern void g_484460;
extern uint128_t g_484470;
extern uint128_t g_484480;
extern uint128_t g_484490;
extern uint128_t g_4844a0;
extern uint128_t g_4844b0;
extern uint128_t g_4844c0;
extern uint128_t g_4844d0;
extern uint128_t g_4844e0;
extern uint128_t g_4844f0;
extern uint128_t g_484500;
extern uint128_t g_484510;
extern uint128_t g_484520;
extern uint128_t g_484530;
extern uint128_t g_484540;
extern uint128_t g_484550;

unsigned long long base64_init(void)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rdx

    v1 = 0;
    v2 = 65;
    *((uint128_t *)&g_484460) = (uint128_t)0x40404040404040404040404040404040;
    g_484470 = (uint128_t)0x40404040404040404040404040404040;
    g_484480 = (uint128_t)0x40404040404040404040404040404040;
    g_484490 = (uint128_t)0x40404040404040404040404040404040;
    g_4844a0 = (uint128_t)0x40404040404040404040404040404040;
    g_4844b0 = (uint128_t)0x40404040404040404040404040404040;
    g_4844c0 = (uint128_t)0x40404040404040404040404040404040;
    g_4844d0 = (uint128_t)0x40404040404040404040404040404040;
    g_4844e0 = (uint128_t)0x40404040404040404040404040404040;
    g_4844f0 = (uint128_t)0x40404040404040404040404040404040;
    g_484500 = (uint128_t)0x40404040404040404040404040404040;
    g_484510 = (uint128_t)0x40404040404040404040404040404040;
    g_484520 = (uint128_t)0x40404040404040404040404040404040;
    g_484530 = (uint128_t)0x40404040404040404040404040404040;
    g_484540 = (uint128_t)0x40404040404040404040404040404040;
    g_484550 = (uint128_t)0x40404040404040404040404040404040;
    while (1)
    {
        (&g_484460)[v2] = v1;
        v1 += 1;
        if (v1 == 64)
            break;
        v2 = *((char *)(v1 + 4597024));
    }
    return 64;
}



// Function: transform_stat_info @ 0x21440
void transform_stat_info(unsigned int a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (a0 != 86)
    {
        v0 = v2;
        sub_421459();
    }
    return;
}



// Function: tar_checksum @ 0x214d0
unsigned int tar_checksum(char *a0, char a1)
{
    unsigned long long iter;  // rsi
    unsigned long long node;  // rcx
    char *i;  // rdx
    unsigned long v5;  // rdi
    char *j;  // rdx
    unsigned long long v7;  // rbx
    unsigned long v8;  // rdi
    unsigned int v9;  // eax
    unsigned long long v0;  // [bp-0x10]

    iter = 0;
    node = 0;
    i = a0;
    do
    {
        v5 = *(i);
        i += 1;
        node += v5;
        iter += (char)v5;
    } while (a0 + 0x200 != i);
    if (!(unsigned int)node)
        return 3;
    j = a0 + 155;
    v0 = v7;
    do
    {
        v8 = *(j);
        j -= 1;
        node -= v8;
        iter -= (char)v8;
    } while (a0 + 147 != j);
    v9 = sub_420890(a0 + 148, 8, 0, 0, 0x7fffffff, 1, a1);
    if (v9 < 0)
        return 5;
    if ((unsigned int)node + 0x100 == v9 || (unsigned int)iter + 0x100 == v9)
        return 1;
    return 5;
    return 1;
}



// Function: decode_header @ 0x215c0
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
    char padding_9d[108];
    char field_109;
    char padding_10a[31];
    char field_129;
    char padding_12a[177];
    char field_1db;
    char field_1dc;
    char padding_1dd[10];
    char field_1e7;
    char field_1e8;
    char padding_1e9[10];
    char field_1f3;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned int g_484400;
extern struct_1 *g_484408;
extern uint128_t g_484a20;
extern char g_484ad2;
extern char g_484b50;

void decode_header(struct_0 *a0, void* idx, unsigned int *a2, unsigned int a3)
{
    struct_0 *v2;  // r12
    unsigned int v3;  // eax
    unsigned int v4;  // r12d
    unsigned long long v5;  // rax
    unsigned long v6;  // 4128
    unsigned long long v7;  // rax
    unsigned long long v8;  // rax
    unsigned long long v9;  // rax
    unsigned int v10;  // eax
    char v0;  // [bp-0x41]

    v2 = &a0->padding_9d[100];
    v3 = sub_421210(&a0->padding_0[100], &v0);
    if (strcmp(v2, "ustar"))
    {
        v4 = 1;
        if (!strcmp(v2, "ustar  "))
            v4 = (-(v0 < 1) & 4) + 2;
    }
    else if (a0->field_1db || a0->field_1dc - 48 > 7 || a0->field_1e7 != 32 || a0->field_1e8 - 48 > 7 || (v4 = 5, a0->field_1f3 != 32))
    {
        v4 = 4 - ((long long)idx[368] < 1);
    }
    *(a2) = v4;
    *((unsigned int *)&idx[112]) = v3;
    v5 = sub_421320(&a0->padding_0[0x88]);
    v6 = a0->field_109;
    *((unsigned long long *)&idx[0x100]) = 0;
    *((unsigned long long *)&idx[248]) = v5;
    sub_423c20(idx + 32);
    sub_423c20(idx + 40);
    sub_41c5b0(idx);
    if (v4 != 2)
    {
        if (v4 != 5)
        {
            *((uint128_t *)&idx[264]) = g_484a20;
            *((uint128_t *)&idx[232]) = g_484a20;
            if (v4 != 1)
                goto LABEL_4216dd;
            *((unsigned int *)&idx[116]) = sub_421260(&a0->padding_0[108]);
            *((unsigned int *)&idx[120]) = sub_421290(&a0->padding_0[116]);
            v7 = 0;
        }
        else
        {
            *((unsigned long long *)&idx[232]) = sub_421320(&a0->field_1dc);
            v8 = sub_421320(&a0->field_1e8);
            *((unsigned long long *)&idx[272]) = 0;
            *((unsigned long long *)&idx[264]) = v8;
            *((unsigned long long *)&idx[240]) = 0;
LABEL_4216dd:
            if (a3)
            {
                if (g_484ad2 || !a0->field_109 || !sub_4269f0(&a0->field_109, idx + 116))
                    *((unsigned int *)&idx[116]) = sub_421260(&a0->padding_0[108]);
                if (g_484ad2 || !a0->field_129 || !sub_426aa0(&a0->field_129, idx + 120))
                    *((unsigned int *)&idx[120]) = sub_421290(&a0->padding_0[116]);
            }
            v7 = 0;
            if (a0->field_9c - 0x33 <= 1)
            {
                v10 = sub_4212c0(&a0->padding_12a[39]);
                v7 = gnu_dev_makedev(sub_4212f0(&a0->padding_12a[31]) & 0xffffffff, v10);
            }
        }
    }
    else
    {
        if (!g_484b50)
        {
            *((uint128_t *)&idx[264]) = g_484a20;
            *((uint128_t *)&idx[232]) = g_484a20;
            goto LABEL_4216dd;
        }
        else
        {
            *((unsigned long long *)&idx[232]) = sub_421320(&a0->padding_12a[47]);
            v9 = sub_421320(&a0->padding_12a[59]);
            *((unsigned long long *)&idx[272]) = 0;
            *((unsigned long long *)&idx[264]) = v9;
            *((unsigned long long *)&idx[240]) = 0;
            goto LABEL_4216dd;
        }
    }
    *((unsigned long long *)&idx[128]) = v7;
    sub_41c8c0(idx);
    if ((char)sub_429e10(idx))
    {
        sub_429e70(idx);
        *((char *)&idx[288]) = 1;
        return;
    }
    *((char *)&idx[288]) = 0;
    if (((g_484400 & 0xfffffffb) != 2 || *((char *)&g_484408[19].field_0 + 4) != 0x44) && !(long long)idx[400])
        return;
    *((char *)&idx[392]) = 1;
    return;
}



// Function: off_from_header @ 0x219b0
unsigned long long off_from_header(void* a0, unsigned int a1)
{
    return sub_420890(a0, a1, "off_t", 0, 0x7fffffffffffffff, 0, 0);
}



// Function: read_header @ 0x219e0
typedef struct struct_2 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_2;

typedef struct struct_1 {
    char field_0[8];
    char padding_8[496];
    unsigned long long field_1f8;
    char field_200;
} struct_1;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_4843e0;
extern unsigned long long g_4843e8;
extern void* g_4843f0;
extern void* g_4843f8;
extern struct_0 *g_484560;
extern struct_3 *g_484d50;
extern unsigned int g_484d58;

unsigned int read_header(void* *a0, struct_2 *idx, unsigned int a2)
{
    void* i;  // rax
    void* idx1;  // r15
    unsigned long long v31;  // rax
    void* v32;  // rsi
    unsigned long long v33;  // rcx
    unsigned int v35;  // edx
    unsigned long v36;  // rbx
    unsigned long long v37;  // r13
    struct_1 *v38;  // rbp
    unsigned long long *node;  // rdi
    struct_1 *iter;  // rbp
    unsigned int v23;  // eax
    struct_1 *v41;  // rcx
    void* v42;  // rsi
    unsigned long long v43;  // rcx
    unsigned long v44;  // rbx
    unsigned long v45;  // rbx
    unsigned long v46;  // r13
    unsigned long n;  // rax
    unsigned long v48;  // r15
    struct_1 *v50;  // rbp
    void* v24;  // rbp
    void* index;  // rax
    void* v52;  // rdx
    unsigned int v53;  // edx
    unsigned long v25;  // rax
    char v26;  // dl
    char v27;  // cl
    unsigned long long *v29;  // rax
    unsigned long long *iter1;  // rdi
    struct_1 *ptr;  // [bp-0x188], Other Possible Types: void*
    struct_1 *ptr1;  // [bp-0x180], Other Possible Types: void*
    unsigned long long v2;  // [bp-0x178]
    unsigned long long v3;  // [bp-0x170]
    long long v4[2];  // [bp-0x168]
    uint128_t v5;  // [bp-0x158]
    int v6;  // [bp-0x148], Other Possible Types: char[112]
    int v7;  // [bp-0x138]
    int v8;  // [bp-0x128]
    int v9;  // [bp-0x118]
    int v10;  // [bp-0x108]
    int v11;  // [bp-0xf8]
    unsigned int v12;  // [bp-0xe8]
    char v13;  // [bp-0xe4]
    int v14;  // [bp-0xd8]
    int v15;  // [bp-0xc8]
    unsigned long v16;  // [bp-0xb8]
    unsigned short v17;  // [bp-0xb0]
    char v18;  // [bp-0xae]
    char v19;  // [bp-0xad]

    v2 = 0;
    v3 = 0;
    ptr = NULL;
    ptr1 = NULL;
    i = sub_40e130();
    *(a0) = i;
    idx1 = i;
    if (i)
    {
        do
        {
            v23 = sub_4214d0(i, 0);
            if (v23 != 1)
            {
                free(ptr1);
                free(ptr);
                return v23;
            }
            if ((char)idx1[156] == 49)
            {
                *((unsigned long *)&idx[8].padding_0[0]) = 0;
                goto LABEL_421a83;
            }
            else
            {
                v24 = idx1 + 124;
                v25 = sub_4219b0(v24, 12);
                *((unsigned long *)&idx[8].padding_0[0]) = v25;
                if (v25 < NULL)
                {
                    free(ptr1);
                    free(ptr);
                    return 5;
                }
                v27 = (char)idx1[156] - 75;
                if (v27 <= 1)
                    goto LABEL_421bb7;
                switch (v26)
                {
                case 120:
LABEL_421bb7:
                    if (a2 == 1)
                    {
                        free(ptr1);
                        free(ptr);
                        return 2;
                    }
                    if (v27 <= 1)
                    {
                        v35 = v25;
                        v36 = v25 + 0x200;
                        if ((unsigned short)v35 & 0x1ff)
                        {
                            v36 = v25 + 0x400 - (v35 & 0x1ff);
                            if (v25 > v36)
                                sub_416420(); /* do not return */
                        }
                        v37 = v36 >> 9;
                        v38 = sub_44c5b0(v36 + 1);
                        if ((char)idx1[156] != 76)
                        {
                            free(ptr);
                            v2 = v37;
                            ptr = v38;
                        }
                        else
                        {
                            free(ptr1);
                            v3 = v37;
                            ptr1 = v38;
                        }
                        sub_40d1f0(idx1);
                        node = v38->padding_8 & 0xfffffffffffffff8;
                        iter = &v38->field_200;
                        v41 = v38 - node;
                        *((long long *)(&iter->field_0[0] - 0x200)) = *((long long *)idx1);
                        v42 = idx1 - v41;
                        *((long long *)(&iter->field_0[0] - 8)) = (long long)idx1[504];
                        for (v43 = (unsigned int)v41 + 0x200 >> 3; v43; v42 += 8)
                        {
                            v43 -= 1;
                            *(node) = *((long long *)v42);
                            node += 1;
                        }
                        v44 = v36 - 0x200;
                        if (v36 != 0x200)
                        {
                            do
                            {
                                v45 = v44;
                                v46 = sub_40e130();
                                if (!v46)
                                {
                                    if (g_484d50)
                                        g_484d50();
                                    error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
                                    g_484d58 = 2;
                                    break;
                                }
                                n = sub_40d2b0(v46);
                                if (v45 <= n)
                                    n = v45;
                                v48 = n;
                                memcpy(iter, v46, n);
                                iter = &iter->field_0[v48];
                                sub_40d1f0(v46 + v48 - 1);
                                v44 = v45 - v48;
                            } while (v45 != v48);
                        }
                        iter->field_0[0] = 0;
                        break;
                    }
                    break;
                case 88: case 103:
                    if (a2 == 1)
                    {
                        free(ptr1);
                        free(ptr);
                        return 2;
                    }
                    if ((char)idx1[156] == 88)
                        continue;
                    v29 = &g_484560->field_0;
                    if (!v29)
                    {
                        v29 = sub_44c5b0(0x200);
                        g_484560 = v29;
                    }
                    iter1 = v29 + 1 & 0xfffffffffffffff8;
                    *(v29) = *((long long *)idx1);
                    v29[63] = (long long)idx1[504];
                    v31 = v29 - iter1;
                    v32 = idx1 - v31;
                    for (v33 = (unsigned int)v31 + 0x200 >> 3; v33; v32 += 8)
                    {
                        v33 -= 1;
                        *(iter1) = *((long long *)v32);
                        iter1 += 1;
                    }
                    v4 = 0;
                    v5 = 0;
                    sub_41cac0(v4, idx1, sub_4219b0(v24, 12));
                    sub_41c9a0(v4);
                    sub_41cca0(v4);
                    if (a2 == 2)
                    {
                        free(ptr1);
                        free(ptr);
                        return 2;
                    }
                    break;
                default:
LABEL_421a83:
                    free(g_4843f8);
                    v50 = &ptr1->field_200;
                    if (!ptr1)
                    {
                        if ((char)idx1[345])
                        {
                            v50 = v6;
                            index = v6;
                            if (!strcmp(idx1 + 0x101, "ustar"))
                            {
                                v19 = 0;
                                v6 = (int)(int128_t)idx1[345];
                                v16 = (long long)idx1[489];
                                v7 = (int)(int128_t)idx1[361];
                                v18 = (char)idx1[499];
                                v17 = (short)idx1[497];
                                v8 = (int)(int128_t)idx1[377];
                                v9 = (int)(int128_t)idx1[393];
                                v10 = (int)(int128_t)idx1[409];
                                v11 = (int)(int128_t)idx1[425];
                                v6 = (char[112])_INSERT(v6 CONCAT 0, 96, (int128_t)idx1[441]);
                                v14 = (int)(int128_t)idx1[457];
                                v15 = (int)(int128_t)idx1[473];
                                v52 = &v6[strlen(v6)];
                                *((char *)v52) = 47;
                                index = v52 + 1;
                            }
                        }
                        else
                        {
                            v50 = v6;
                            index = v6;
                        }
                        v3 = 0;
                        *((int128_t *)index) = *((int128_t *)idx1);
                        *((int128_t *)&index[16]) = (int128_t)idx1[16];
                        *((int128_t *)&index[32]) = (int128_t)idx1[32];
                        *((int128_t *)&index[48]) = (int128_t)idx1[48];
                        *((int128_t *)&index[64]) = (int128_t)idx1[64];
                        *((int128_t *)&index[80]) = (int128_t)idx1[80];
                        v53 = (int)idx1[96];
                        *((char *)&index[100]) = 0;
                        *((unsigned int *)&index[96]) = v53;
                    }
                    g_4843f8 = ptr1;
                    g_4843e8 = v3;
                    sub_423be0(idx, v50);
                    sub_423be0(&idx->field_8, v50);
                    idx->field_10 = sub_43e630(idx->field_8);
                    free(g_4843f0);
                    if (!ptr)
                    {
                        v13 = 0;
                        v2 = 0;
                        v12 = (int)idx1[253];
                        v6 = (int)(int128_t)idx1[157];
                        v7 = (int)(int128_t)idx1[173];
                        v8 = (int)(int128_t)idx1[189];
                        v9 = (int)(int128_t)idx1[205];
                        v10 = (int)(int128_t)idx1[0xdd];
                        v11 = (int)(int128_t)idx1[237];
                    }
                    g_4843f0 = ptr;
                    g_4843e0 = v2;
                    sub_423be0(&idx[1].padding_0[7]);
                    ptr = NULL;
                    ptr1 = NULL;
                    free(ptr1);
                    free(ptr);
                    return v23;
                }
            }
            sub_41cac0(&idx[21].padding_0[3], idx1, sub_4219b0(v24, 12));
            i = sub_40e130();
            *(a0) = i;
            idx1 = i;
        } while (i);
    }
    free(ptr1);
    free(ptr);
    return 4;
}



// Function: uintmax_from_header @ 0x22050
unsigned long long uintmax_from_header(void* a0, unsigned int a1)
{
    return sub_420890(a0, a1, "uintmax_t", 0, -0x1, 0, 0);
}



// Function: tartime @ 0x22080
typedef struct tm {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
    long tm_gmtoff;
    char * tm_zone;
} tm;

extern char g_484420;
extern long long g_484426;
extern void g_484434;
extern long long g_48443a;
extern void g_484441;
extern char g_484b91;

char * tartime(int a0[2], unsigned int a1, char a2)
{
    tm *tm;  // rcx
    void* v5;  // rax
    void* v6;  // rax
    void* v7;  // rcx
    void* iter;  // rdx
    unsigned long long v9;  // rdx
    char *v0;  // [bp-0x30]
    int v1[2];  // [bp-0x28]

    v1 = a0;
    if (a1 && a0 < NULL)
    {
        v1 = a0 + 1;
        a1 = 1000000000 - a1;
    }
    if (g_484b91)
    {
        tm = gmtime(v1);
        if (!tm)
            goto LABEL_42212d;
    }
    else
    {
        tm = localtime(v1);
        if (!tm)
        {
LABEL_42212d:
            if (a0 >= NULL)
            {
                v5 = sub_443090(v1, &g_484426);
            }
            else
            {
                v6 = sub_443090(-(v1), &g_484426);
                v5 = v6 - 1;
                *((char *)v6 - 1) = 45;
            }
            v7 = &g_484441;
            if (!a2)
                v7 = &g_484434;
            for (iter = v5; iter > v7; iter -= 1)
            {
                *((char *)iter - 1) = 32;
            }
            v9 = v7 - v5;
            if (v7 > v5)
                v9 = 0;
            if (a2)
            {
                v0 = v5 + v9;
                sub_424170(a1, &g_48443a);
                return v0;
            }
            return v5 + v9;
        }
    }
    if (a2)
    {
        strftime(&g_484420, 37, "%Y-%m-%d %H:%M:%S", tm);
        sub_424170(a1, &(&g_484420)[strlen(&g_484420)]);
        return &g_484420;
    }
    strftime(&g_484420, 37, "%Y-%m-%d %H:%M", tm);
    return &g_484420;
}



// Function: simple_print_header @ 0x22220
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char field_10;
    char padding_11[7];
    long long field_18;
    char padding_20[80];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[4];
    long long field_80;
    long long field_88;
    char padding_90[104];
    int field_f8[2];
    unsigned long long field_100;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned int g_481228;
extern unsigned int g_48122c;
extern FILE *g_483f28;
extern unsigned long long g_4843e0;
extern unsigned long long g_4843e8;
extern unsigned int g_484400;
extern char g_484451;
extern char g_4847e1;
extern int g_484a54;
extern char g_484ad2;
extern char g_484b7c;
extern char g_484b90;
extern struct_1 *g_484d50;
extern unsigned int g_484d58;

int simple_print_header(void)
{
    struct_0 *idx;  // rdi
    unsigned long v20;  // r15
    unsigned int v27;  // eax
    char *v28;  // r12
    char *v29;  // r14
    unsigned int v33;  // eax
    unsigned long v34;  // rsi
    unsigned long v35;  // rcx
    unsigned int v36;  // eax
    char *v37;  // rax
    char *v38;  // rsi
    char *v39;  // rsi
    char *v41;  // rax
    char *v42;  // rax
    char *v43;  // rax
    char *v44;  // rax
    char *v45;  // rax
    long long v21;  // rdx
    long long v22;  // rax
    long long v23;  // rdx
    char *v24;  // rax
    void* v25;  // rsi
    unsigned long v0;  // [bp-0x138]
    char *v1;  // [bp-0x130]
    unsigned long v2;  // [bp-0x128]
    char *v3;  // [bp-0x120]
    char *v4;  // [bp-0x110]
    unsigned long len1;  // [bp-0x108]
    unsigned int v6;  // [bp-0xfc]
    unsigned long len;  // [bp-0xf8]
    char *v8;  // [bp-0xf0]
    char v9;  // [bp-0xe6]
    char v10;  // [bp-0xe5]
    char v11;  // [bp-0xe4]
    char v12;  // [bp-0xe3]
    char v13;  // [bp-0xda]
    char v14;  // [bp-0xd8]
    char v15;  // [bp-0xb8]
    char v16;  // [bp-0x98]
    char v17[56];  // [bp-0x78]

    if (g_4847e1)
    {
        v20 = idx->field_8;
        if (!idx->field_8)
            v20 = idx->field_0;
    }
    else
    {
        v20 = idx->field_0;
        if (!idx->field_0)
            v20 = idx->field_8;
    }
    if (g_484b7c)
    {
        if (v21 < NULL)
            v22 = sub_40d180();
        v23 = v22 - g_4843e0 - g_4843e8;
        sub_443090(v23, v17, v23);
        v24 = dcgettext(NULL, "block %s: ", 5);
        __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, v24);
    }
    if (g_484a54 > 1)
    {
        v11 = 63;
        switch ((char)v25[156])
        {
        case 0: case 48: case 83:
            v11 = ((char)-(idx->field_10 < 1) & 201) + 100;
            break;
        case 49:
            v11 = 104;
            break;
        case 50:
            v11 = 108;
            break;
        case 51:
            v11 = 99;
            break;
        case 52:
            v11 = 98;
            break;
        case 53: case 68:
            v11 = 100;
            break;
        case 54:
            v11 = 112;
            break;
        case 55:
            v11 = 67;
            break;
        case 75: case 76:
            v11 = 76;
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "Unexpected long name header", 5));
            g_484d58 = 2;
            break;
        case 77:
            v11 = 77;
            break;
        case 86:
            g_484451 = 1;
            v11 = 86;
            break;
        }
        sub_432310(idx->field_70, &v12);
        sub_432010(idx, &v13);
        v4 = sub_422080(idx->field_f8, idx->field_100, g_484b90);
        v27 = strlen(v4);
        if (g_481228 < v27)
            g_481228 = v27;
        v28 = *((long long *)&idx->padding_20[0]);
        if (!v28 || !*(v28) || g_484400 == 1 || g_484ad2)
            v28 = sub_443090(idx->field_74, &v14);
        v29 = *((long long *)&idx->padding_20[8]);
        if (!v29 || !*(v29) || g_484400 == 1 || g_484ad2)
            v29 = sub_443090(idx->field_78, &v15);
        if ((char)v25[156] - 0x33 > 1)
        {
            v8 = &v16;
            __strcpy_chk(v17, sub_443090(idx->field_88, &v16), 42);
        }
        else
        {
            v8 = &v16;
            __strcpy_chk(v17, sub_443090(gnu_dev_major(idx->field_80) & 0xffffffff, &v16), 42);
            __strcat_chk(v17, ",", 42);
            __strcat_chk(v17, sub_443090(gnu_dev_minor(idx->field_80) & 0xffffffff, v8), 42);
        }
        len = strlen(v17);
        v6 = len;
        len1 = strlen(v28);
        v33 = strlen(v29);
        v34 = v6;
        v35 = g_48122c;
        v36 = (unsigned int)len1 + v33 + 2 + (unsigned int)len;
        if ((unsigned int)v35 >= v36)
            v34 = v35 - g_48122c + len;
        else
            g_48122c = v36;
        v3 = v4;
        v2 = g_481228;
        v1 = &v17[0];
        v0 = v34;
        __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, "%s %s/%s %*s %-*s", &v11, v28, v29);
        v37 = sub_44a8a0(v20);
        __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, " %s", v37);
        if (g_4847e1 && idx->field_10)
            fputc_unlocked(47, g_483f28);
        switch ((char)v25[156])
        {
        case 0: case 48: case 51: case 52: case 53: case 54: case 55: case 68: case 83:
            putc_unlocked(10, g_483f28);
            fflush_unlocked(g_483f28);
            sub_432150(idx);
            return;
        case 49:
            sub_44a8a0(idx->field_18);
            v38 = " link to %s\n";
LABEL_422652:
            v44 = dcgettext(NULL, v38, 5);
            __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, v44);
            fflush_unlocked(g_483f28);
            sub_432150(idx);
            return;
        case 50:
            v43 = sub_44a8a0(idx->field_18);
            __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, " -> %s\n", v43);
            fflush_unlocked(g_483f28);
            sub_432150(idx);
            return;
        case 75:
            v39 = "--Long Link--\n";
            break;
        case 76:
            v39 = "--Long Name--\n";
            break;
        case 77:
            __strcpy_chk(v17, sub_443090(sub_422050(v25 + 369, 12), v8), 42);
            v41 = dcgettext(NULL, "--Continued at byte %s--\n", 5);
            __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, v41);
            fflush_unlocked(g_483f28);
            sub_432150(idx);
            return;
        case 86:
            v39 = "--Volume Header--\n";
            break;
        default:
            v9 = (char)v25[156];
            v10 = 0;
            sub_44ac50(&v9);
            v38 = " unknown file type %s\n";
            goto LABEL_422652;
        }
        v42 = dcgettext(NULL, v39, 5);
        __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, v42);
        fflush_unlocked(g_483f28);
        sub_432150(idx);
        return;
    }
    else
    {
        v45 = sub_44a8a0(v20);
        fputs_unlocked(v45, g_483f28);
        if (g_4847e1 && idx->field_10)
            fputc_unlocked(47, g_483f28);
        fputc_unlocked(10, g_483f28);
        fflush_unlocked(g_483f28);
        sub_432150(idx);
        return;
    }
}



// Function: print_volume_label @ 0x228b0
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
    char padding_9d[108];
    char field_109;
    char padding_10a[31];
    char field_129;
    char padding_12a[177];
    char field_1db;
    char field_1dc;
    char padding_1dd[10];
    char field_1e7;
    char field_1e8;
    char padding_1e9[10];
    char field_1f3;
} struct_0;

extern long long g_483f18;
extern struct_0 *g_484560;

void print_volume_label(void)
{
    unsigned long long v8;  // rcx
    unsigned long long *iter;  // rdi
    unsigned int v0;  // [bp-0x3ec]
    char v1;  // [bp-0x3e8]
    char v2;  // [bp-0x3e0]
    struct_0 v3;  // [bp-0x228]
    unsigned long v4;  // [bp-0x1a0]
    unsigned int v5;  // [bp-0x198]
    char v6;  // [bp-0x18c]

    v8 = 64;
    for (iter = &v3; v8; iter += 1)
    {
        v8 -= 1;
        *(iter) = 0;
    }
    v6 = 86;
    if (g_484560)
    {
        v4 = g_484560[0x11].field_0;
        v5 = g_484560[18].field_0;
    }
    sub_42f630(&v1);
    sub_423be0(&v2, ".");
    sub_4215c0(&v3, &v1, &v0, 0);
    sub_423be0(&v2, g_483f18);
    sub_422220(&v1, &v3, 0);
    sub_42f6f0(&v1);
    return;
}



// Function: print_header @ 0x22990
extern unsigned long long g_483f18;
extern unsigned int g_484400;
extern char g_484451;

void print_header(long long a0, long long a1)
{
    long long v1;  // rdx

    if (g_484400 != 4)
    {
        sub_422220();
        return;
    }
    else if (g_484451)
    {
        sub_422220();
        return;
    }
    else if (!g_483f18)
    {
        sub_422220();
        return;
    }
    else
    {
        sub_4228b0();
        g_484451 = 1;
        sub_422220(a0, a1, v1);
        return;
    }
}



// Function: print_for_mkdir @ 0x22a00
extern unsigned int g_481228;
extern unsigned int g_48122c;
extern unsigned long long g_483f28;
extern int g_484a54;
extern char g_484b7c;

void print_for_mkdir(long long a0, unsigned long a1, unsigned int a2)
{
    char *v6;  // rax
    unsigned long long v7;  // rax
    unsigned long long v0;  // [bp-0x58]
    char v1;  // [bp-0x43]
    char v2;  // [bp-0x42]
    char v3;  // [bp-0x38]

    if (g_484a54 <= 1)
        return;
    v1 = 100;
    sub_432310(a2, &v2);
    if (g_484b7c)
    {
        sub_443090(sub_40d180(), &v3);
        v6 = dcgettext(NULL, "block %s: ", 5);
        __fprintf_chk(g_483f28, 0x1, v6);
    }
    v7 = sub_44a8a0(a0);
    dcgettext(NULL, "Creating directory:", 5);
    v0 = v7;
    __fprintf_chk(g_483f28, 0x1, "%s %*s %s\n", &v1, g_48122c + g_481228 + 1);
    return;
}



// Function: skip_file @ 0x22b10
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

extern char g_4847f8;
extern struct_0 *g_484d50;

struct_0 * skip_file(unsigned long a0)
{
    unsigned long v1;  // rbx
    long long v2;  // rax
    struct_0 *v3;  // rax
    unsigned long iter;  // rbx
    unsigned long long v5;  // rax
    struct_0 *v6;  // rax

    v1 = a0;
    if (g_4847f8)
    {
        v2 = sub_40d540(a0);
        if (v2 >= 0)
            v1 -= v2 * 0x200;
        else
            g_4847f8 = 0;
    }
    v3 = sub_40ccf0(v1);
    if (v1 <= 0)
        return v3;
    iter = v1;
    while (1)
    {
        v5 = sub_40e130();
        if (!v5)
            break;
        sub_40d1f0(v5);
        iter -= 0x200;
        v6 = sub_40ccf0(iter);
        if (iter == v1 - 0x200 - (v1 - 1 & 0xfffffffffffffe00))
            return v6;
    }
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
    sub_4163d0(); /* do not return */
}



// Function: skip_member @ 0x22bd0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484408;
extern char g_484840;
extern unsigned int g_4848c8;
extern char g_484960;
extern char g_4849c9;

void skip_member(void)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (g_4849c9)
        return;
    v0 = v2;
    sub_40d1f0(g_484408);
    sub_40ccb0(&g_484840);
    if (g_484960)
    {
        sub_429fe0(&g_484840);
        sub_40ccd0();
        return;
    }
    if (*((char *)&g_484408[19].field_0 + 4) == 53)
    {
        sub_40ccd0();
    }
    else
    {
        sub_422b10(*((long long *)&g_4848c8));
        sub_40ccd0();
    }
    return;
}



// Function: read_and @ 0x22c60
typedef struct struct_1 {
    long long field_0;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_462478[4];
extern void* g_484408;
extern unsigned long long g_4846b8;
extern struct_0 *g_4846c0;
extern char g_484840;
extern char g_484aac;
extern char g_484b38;
extern struct_1 *g_484d50;
extern unsigned int g_484d58;

long long read_and(long long a0)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    long long v16;  // r9
    unsigned long v17;  // rax
    unsigned long long v18;  // rbp
    unsigned long long v19;  // rbx
    unsigned long long *idx;  // rbx
    unsigned long v21;  // rdi
    unsigned long v23;  // 4112
    long long v24;  // rbp
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long index;  // fs
    long long v11;  // rdi
    long long v12;  // rsi
    long long v13;  // rdx
    long long v14;  // rcx
    long long v15;  // r8
    unsigned long v0;  // [bp-0x40]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = *((long long *)(40 + index));
    sub_421160();
    sub_426d90(v11, v12, v13, v14, v15, v16, a0);
    sub_40fb70(0);
    sub_42f6f0(&g_484840);
    sub_4219e0(&g_484408, &g_484840, 0);
    if (sub_4219e0(&g_484408, &g_484840, 0) <= 5)
    {
        goto *((void *)((long long)(g_462478[sub_4219e0(&g_484408, &g_484840, 0) & 0xffffffff] + (char *)&g_462478[0])));
    }
    else
    {
        sub_40f900();
        v17 = v0 - *((long long *)(40 + index));
        if (v0 != *((long long *)(40 + index)))
            __stack_chk_fail(); /* do not return */
        v4 = v2;
        v3 = v1;
        v2 = v18;
        v1 = v19;
        idx = &g_4846c0->field_0;
        if (g_4846c0)
        {
            while (1)
            {
                v17 = *((long long *)&g_484b38);
                if (!*((long long *)&g_484b38))
                {
                    if (!idx[6] && (v21 = (unsigned long)idx[2], *((char *)v21)))
                        goto LABEL_427494;
                    goto LABEL_42746e;
                }
                else if (*((long long *)&g_484b38) <= idx[6] || !(v21 = (unsigned long)idx[2], *((char *)v21)))
                {
LABEL_42746e:
                    idx = *(idx);
                    if (!idx)
                        break;
                }
                else
                {
LABEL_427494:
                    sub_4263f0(v21);
                    if (g_484d50)
                        g_484d50();
                    sub_44aa70(idx[2]);
                    v17 = (unsigned long long)error(0, 0, (!idx[6] ? dcgettext(NULL, "%s: Not found in archive", 5) : dcgettext(NULL, "%s: Required occurrence not found in archive", 5)));
                    idx = *(idx);
                    g_484d58 = 2;
                    if (!idx)
                        break;
                }
            }
        }
        v23 = g_484aac;
        g_4846c0 = 0;
        g_4846b8 = 0;
        if (!(char)v23)
            return v17;
        while (1)
        {
            v24 = sub_426cd0(1);
            if (!v24)
                break;
            sub_4263f0(v24);
            if (g_484d50)
                g_484d50();
            sub_44aa70(v24);
            error(0, 0, dcgettext(NULL, "%s: Not found in archive", 5));
            g_484d58 = 2;
        }
        return 0;
    }
}



// Function: list_archive @ 0x23260
extern long long g_484408;
extern char g_484840;
extern void* g_4849d0;
extern int g_484a54;
extern char g_484b50;

void list_archive(void)
{
    unsigned long long v1;  // rax

    sub_40d180();
    if (!g_484a54)
    {
        sub_422bd0();
        return;
    }
    sub_422990(&g_484840, g_484408);
    if (!g_484b50)
    {
        sub_422bd0();
        return;
    }
    else if (g_484a54 <= 2)
    {
        sub_422bd0();
        return;
    }
    else if (!(char)sub_4201b0(&g_484840))
    {
        sub_422bd0();
        return;
    }
    else
    {
        v1 = sub_41f4c0(g_4849d0);
        sub_420710(g_4849d0, v1);
        sub_422bd0();
        return;
    }
}



// Function: test_archive_label @ 0x232e0
typedef struct struct_0 {
    char padding_0[16];
    long long field_10;
    char padding_18[24];
    unsigned long long field_30;
} struct_0;

extern void* g_483f18;
extern unsigned int g_484400;
extern struct_0 *g_484408;
extern unsigned long long g_4846b8;
extern struct_0 *g_4846c0;
extern char g_484840;
extern unsigned int g_484a54;
extern char g_484aac;
extern char g_484b10;
extern char g_484b38;

unsigned long long * test_archive_label(void)
{
    long long v2;  // rdi
    long long v3;  // rsi
    unsigned long long v12;  // rbx
    unsigned long long *v14;  // rax
    struct_0 *v15;  // rbx
    struct_0 *v16;  // rbx
    unsigned long v17;  // 4111
    long long v18;  // rax
    unsigned long long *v19;  // rax
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    long long v8;  // rbp
    unsigned long v9;  // rax
    struct_0 *v10;  // rax
    struct_0 *v11;  // rax
    unsigned long long v0;  // [bp-0x8]

    sub_421160();
    sub_426d90(v2, v3, v4, v5, v6, v7, v8);
    sub_40fb70(0);
    if (sub_4219e0(&g_484408, &g_484840, 0) == 1)
    {
        sub_4215c0(g_484408, &g_484840, &g_484400, 0);
        if (*((char *)&g_484408[19].field_0 + 4) == 86)
            sub_423c20(&g_483f18);
        if (g_483f18)
        {
            if (g_484a54)
                sub_4228b0();
            if (!(char)sub_427250() && g_484b10)
            {
                v9 = sub_40de80(g_483f18);
                sub_427250(v9);
                free(v9);
            }
        }
    }
    sub_40f900();
    v10 = g_4846c0;
    if (!g_4846c0)
        return g_4846c0;
    do
    {
        v11 = v10;
        if (!*((long long *)&g_484b38))
        {
            if (v11->field_30)
                return v11;
        }
        else
        {
            if (*((long long *)&g_484b38) <= v11->field_30)
                return v11;
        }
    } while ((v10 = (struct_0 *)*((long long *)&v11->padding_0[0]), *((long long *)&v11->padding_0[0])));
    v0 = v12;
    if (g_484a54)
        error(0, 0, dcgettext(NULL, "Archive label mismatch", 5));
    v14 = sub_42f7d0(1);
    v15 = g_4846c0;
    if (g_4846c0)
    {
        do
        {
            v16 = v15;
            v14 = sub_4263f0(v16->field_10);
        } while (!(unsigned int)v14 && (v15 = (struct_0 *)*((long long *)&v16->padding_0[0]), *((long long *)&v16->padding_0[0])));
    }
    v17 = g_484aac;
    g_4846c0 = 0;
    g_4846b8 = 0;
    if (!(char)v17)
        return v14;
    while (1)
    {
        <None|const 0> = sub_426cd0(1);
        if (!v18)
            return NULL;
        v19 = sub_4263f0(v18);
        if ((unsigned int)v19)
            return v19;
    }
}



// Function: map_hash @ 0x233c0
void map_hash(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: map_compare @ 0x233d0
long long map_compare(unsigned long a0, unsigned long long *a1)
{
    return _INSERT(*(a1), 0, *((long long *)a0) == *(a1));
}



// Function: name_to_uid @ 0x233e0
typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

unsigned long long name_to_uid(char *a0)
{
    passwd *v1;  // rax

    v1 = getpwnam(a0);
    if (!v1)
        return 0xffffffffffffffff;
    return v1->pw_uid;
}



// Function: name_to_gid @ 0x23410
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

unsigned long long name_to_gid(char *a0)
{
    group *v1;  // rax

    v1 = getgrnam(a0);
    if (!v1)
        return 0xffffffffffffffff;
    return v1->gr_gid;
}



// Function: owner_map_read @ 0x239a0
void owner_map_read(unsigned long a0)
{
}



// Function: owner_map_translate @ 0x239d0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern long long g_484570;
extern unsigned int g_484abc;
extern unsigned long long g_484ac0;

unsigned int owner_map_translate(unsigned int a0, unsigned int *a1, unsigned long long *a2)
{
    struct_0 *v2;  // rax
    unsigned long long v3;  // rax
    unsigned int v4;  // eax
    unsigned long v0;  // [bp-0x38]

    if (g_484570)
    {
        v0 = a0;
        v2 = sub_4418d0(g_484570, &v0);
        if (v2)
        {
            v3 = v2->field_10;
            *(a1) = v2->field_8;
            *(a2) = v3;
            return 0;
        }
    }
    v4 = 1;
    if (g_484abc != 0xffffffff)
    {
        *(a1) = g_484abc;
        v4 = 0;
    }
    if (g_484ac0)
        *(a2) = g_484ac0;
    return v4;
}



// Function: group_map_read @ 0x23a70
void group_map_read(unsigned long a0)
{
}



// Function: group_map_translate @ 0x23aa0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern long long g_484568;
extern unsigned int g_484b54;
extern unsigned long long g_484b58;

unsigned int group_map_translate(unsigned int a0, unsigned int *a1, unsigned long long *a2)
{
    struct_0 *v2;  // rax
    unsigned long long v3;  // rax
    unsigned long v0;  // [bp-0x38]

    if (g_484568)
    {
        v0 = a0;
        v2 = sub_4418d0(g_484568, &v0);
        if (v2)
        {
            v3 = v2->field_10;
            *(a1) = v2->field_8;
            *(a2) = v3;
            return 0;
        }
    }
    if (g_484b54 != 0xffffffff)
        *(a1) = g_484b54;
    if (g_484b58)
        *(a2) = g_484b58;
    return 0;
}



// Function: namebuf_finish @ 0x23b60
unsigned long long namebuf_finish(unsigned long long *ptr)
{
    unsigned long long v1;  // r12

    v1 = *(ptr);
    if (*((char *)(v1 + ptr[2] - 1)) == 47)
        *((char *)(v1 + ptr[2])) = 0;
    free(ptr);
    return v1;
}



// Function: quote_n_colon @ 0x23bb0
long long quote_n_colon(unsigned int a0, long long a1)
{
    long long v9;  // r8
    char v0;  // [bp-0x98]
    uint128_t v1;  // [bp-0x88]
    uint128_t v2;  // [bp-0x78]
    unsigned long long v3;  // [bp-0x68]
    char v4;  // [bp-0x58]
    uint128_t v5;  // [bp-0x48]
    uint128_t v6;  // [bp-0x38]
    unsigned long long v7;  // [bp-0x28]

    sub_44a570(0);
    sub_448800(&v0);
    memcpy(&v4, &v0, 16);
    v7 = v3;
    v5 = v1;
    v6 = v2;
    sub_44a5b0(&v4, 58, 1);
    return sub_44a380(a0, a1, -0x1, v9);
}



// Function: assign_string @ 0x23be0
void assign_string(void* a0, long long a1)
{
    long long v1;  // rbp

    v1 = a1;
    free(*((long long *)a0));
    if (v1)
        v1 = sub_44c7a0(v1);
    *((long long *)a0) = v1;
    return;
}



// Function: assign_string_n @ 0x23c20
long long assign_string_n(void* a0, void* a1, unsigned long a2)
{
    unsigned long v1;  // rax
    unsigned long len;  // r12
    unsigned long dst;  // rax

    v1 = (unsigned long long)free(*((long long *)a0));
    if (!a1)
        return v1;
    len = strnlen(a1, a2);
    dst = memcpy(sub_44c5b0(len + 1), a1, len);
    *((char *)(dst + len)) = 0;
    *((unsigned long *)a0) = dst;
    return dst;
}



// Function: unquote_string @ 0x23c70
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
} struct_0;


unsigned int unquote_string(struct_0 *iter)
{
    char v1;  // dl
    struct_0 *node;  // rax
    unsigned int v3;  // r8d
    struct_0 *v4;  // rcx
    struct_0 *v5;  // r9
    char v6;  // sil
    unsigned int v7;  // edx
    char v8;  // sil
    unsigned int v9;  // esi
    unsigned int v10;  // edx

    v1 = iter->field_0;
    node = iter;
    if (!v1)
        return 1;
    v3 = 1;
    while (1)
    {
        v4 = &iter->field_1;
        v5 = &node->field_1;
        if (v1 == 92)
        {
            v6 = iter->field_1;
            switch (v6)
            {
            case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55:
                v7 = iter->field_2;
                v8 = v6 - 48;
                if ((char)(v7 - 48) <= 7)
                {
                    v9 = v7 + v8 * 8 - 48;
                    v10 = iter->field_3;
                    if ((char)(v10 - 48) > 7)
                    {
                        node->field_0 = v9;
                        iter = &iter->field_3;
                        v1 = iter->field_0;
                        node = v5;
                        break;
                    }
                    else
                    {
                        iter += 1;
                        node->field_0 = v10 + v9 * 8 - 48 & 0xffffffff;
                        node = v5;
                        v1 = iter->field_0;
                        break;
                    }
                }
                else
                {
                    node->field_0 = v8;
                    iter = &iter->field_2;
                    v1 = iter->field_0;
                    node = v5;
                    break;
                }
            case 63:
                node->field_0 = 127;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 92:
                node->field_0 = 92;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 97:
                node->field_0 = 7;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 98:
                node->field_0 = 8;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 102:
                node->field_0 = 12;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 110:
                node->field_0 = 10;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 114:
                node->field_0 = 13;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 116:
                node->field_0 = 9;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            case 118:
                node->field_0 = 11;
                iter = &iter->field_2;
                v1 = iter->field_0;
                node = v5;
                break;
            default:
                node->field_0 = 92;
                if (iter->field_1)
                {
                    node->field_1 = iter->field_1;
                    iter = &iter->field_2;
                    v1 = iter->field_0;
                    node = &node->field_2;
                    v3 = 0;
                    break;
                }
                else
                {
                    node = v5;
                    iter = v4;
                    v3 = 0;
                    goto LABEL_423cf0;
                }
            }
        }
        else if (node != iter)
        {
            node->field_0 = v1;
            node = v5;
            v1 = iter->field_1;
            iter = v4;
        }
        else
        {
            v1 = node->field_1;
            iter = v4;
            node = v5;
            if (!v1)
                break;
            else
                continue;
        }
        if (!v1)
            break;
    }
LABEL_423cf0:
    if (iter == node)
        return v3;
    node->field_0 = 0;
    return v3;
}



// Function: zap_slashes @ 0x23e60
void* zap_slashes(void* a0)
{
    void* iter;  // rax

    if (!a0)
    {
        return a0;
    }
    else if (*((char *)a0))
    {
        iter = a0 + strlen(a0) - 1;
        if (a0 >= iter)
            return a0;
        while (*((char *)iter) == 47)
        {
            iter -= 1;
            *((char *)&iter[1]) = 0;
            if (a0 == iter)
                return a0;
        }
        return a0;
    }
    else
    {
        return a0;
    }
}



// Function: normalize_filename_x @ 0x23ea0
long long normalize_filename_x(void* a0)
{
    char v1;  // dl
    void* v2;  // rax
    void* iter;  // rsi
    char i;  // dl
    void* v5;  // rcx
    void* v6;  // rcx
    void* node;  // rax
    void* iter1;  // rcx
    char v9;  // dl
    unsigned long v10;  // rax

    v1 = *((char *)a0);
    v2 = a0;
    iter = a0;
    if (v1 == 46)
    {
        do
        {
            if ((char)v2[1] != 47)
                break;
            i = (char)v2[2];
            v5 = v2 + 2;
            v2 = v5;
            if ((char)v2[2] == 47)
            {
                i = (char)v5[1];
                v2 = v5 + 1;
                if ((char)v5[1] == 47)
                {
                    do
                    {
                        v6 = v2;
                        i = (char)v6[1];
                        v2 = v6 + 1;
                    } while ((char)v6[1] == 47);
                }
            }
            iter += i < 1;
            *((char *)iter) = i;
        } while (i == 46);
        v1 = *((char *)v2);
    }
    *((char *)iter) = v1;
    node = v2 + 1;
    iter1 = iter + 1;
    if (v1)
    {
        while (1)
        {
            if (v1 != 47)
            {
                v9 = *((char *)node);
                v1 = v9;
                goto LABEL_423f1a;
            }
            else
            {
                v1 = v9;
                if (*((char *)node + (v9 == 46)) != 47)
                {
LABEL_423f1a:
                    iter1 += 1;
                    node += 1;
                    *((char *)iter1 - 1) = v1;
                    if (!v1)
                        break;
                }
                else
                {
                    do
                    {
                        node = node + (v1 == 46) + 1;
                        v1 = *((char *)node);
                    } while (*((char *)node + (v1 == 46)) == 47);
                    iter1 += 1;
                    node += 1;
                    *((char *)iter1 - 1) = v1;
                    if (!v1)
                        break;
                }
            }
        }
    }
    if (iter1 - a0 <= 2)
        return iter1 - a0;
    v10 = *((char *)iter1 - 2);
    if ((char)v10 != 46)
    {
        if ((char)v10 == 47)
            goto LABEL_423f98;
    }
    else
    {
        if (*((char *)iter1 - 3) == 47 && !(iter1 -= 1, v10 = (unsigned long)(iter1 - a0), v10 <= 2))
        {
LABEL_423f98:
            iter1 -= 1;
        }
    }
    *((char *)iter1 - 1) = 0;
    return v10;
}



// Function: replace_prefix @ 0x23fc0
long long replace_prefix(void* a0, void* a1, unsigned long a2, void* a3, unsigned long a4)
{
    unsigned long v1;  // r13
    unsigned long len;  // rax
    unsigned long v3;  // rbp
    unsigned long v4;  // rax
    char *v5;  // rsi

    v1 = *((long long *)a0);
    len = strlen(v1);
    if (len <= a2)
        return len;
    v3 = len;
    v4 = memcmp(v1, a1, a2);
    if ((unsigned int)v4)
        return v4;
    v5 = v1 + a2;
    if (*(v5) != 47)
        return v4;
    if (a2 < a4)
    {
        v1 = sub_44c610(v1, v3 - a2 + a4 + 1);
        *((unsigned long *)a0) = v1;
        v5 = v1 + a2;
    }
    memmove(v1 + a4, v5, v3 - a2 + 1);
    return memcpy(v1, a3, a4);
}



// Function: sysinttostr @ 0x24080
void sysinttostr(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    if (a0 <= a2)
        sub_443090();
    else
        sub_442f50();
    return;
}



// Function: strtosysint @ 0x240a0
long long strtosysint(char *a0, long long a1, long long a2, long long a3)
{
    unsigned int *err;  // rax
    unsigned int *v2;  // rbx
    long long v3;  // rax
    long long v4;  // rax
    long long v5;  // rax

    err = __errno_location();
    *(err) = 0;
    v2 = err;
    if (a3 >= 0)
    {
        if (a0[*(a0) == 45] - 48 <= 9)
        {
            v3 = strtoimax(a0, a1, 10);
            if (v3 >= a2 && a3 >= v3)
                return v3;
            v4 = a3;
            *(v2) = 0x22;
            if (v3 < a2)
                v4 = a2;
            return v4;
        }
    }
    else
    {
        if (*(a0) - 48 <= 9)
        {
            v5 = strtoumax(a0, a1, 10);
            if (a3 < v5)
            {
                *(v2) = 0x22;
                return a3;
            }
            return sub_423b90(v5);
        }
    }
    *(v2) = 22;
    return 0;
}



// Function: code_ns_fraction @ 0x24170
long long code_ns_fraction(unsigned int a0, char *idx)
{
    unsigned int v1;  // edx
    unsigned long v2;  // rax
    int v11;  // r9d
    unsigned long v12;  // rdi
    unsigned long v3;  // rax
    unsigned int v4;  // ecx
    unsigned int v5;  // edi
    int v6;  // r8d
    int v7;  // r8d
    int v8;  // r8d
    unsigned int i;  // ecx
    unsigned int v10;  // edi

    v1 = a0;
    if (!a0)
    {
        *(idx) = 0;
        return v2;
    }
    v3 = a0;
    *(idx) = 46;
    v4 = v1;
    v5 = (unsigned int)(((long long)(v3 * 1717986919) >> 0x22) - ((int)(a0) >> 31)) * 10;
    if (v4 == v5)
    {
        v6 = 9;
        do
        {
            v8 = v6 - 1;
            v1 = ((long long)(v3 * 1717986919) >> 0x22) - ((int)(v1) >> 31);
            v3 = v1;
            i = v1;
            v10 = (unsigned int)(((long long)(v3 * 1717986919) >> 0x22) - ((int)(v1) >> 31)) * 10;
            v6 = v8;
        } while (i == v10);
        v11 = v7 - 2;
        v12 = v11;
        idx[1 + v8] = 0;
        idx[1 + v12] = (char)(i - v10) + 48;
        if (!v11)
            return v3;
    }
    else
    {
        idx[10] = 0;
        v12 = 8;
        idx[9] = (char)(v4 - v5) + 48;
    }
    do
    {
        v1 = ((long long)(v3 * 1717986919) >> 0x22) - ((int)(v1) >> 31);
        v3 = v1;
        idx[v12] = (char)(v1 - (unsigned int)(((long long)(v3 * 1717986919) >> 0x22) - ((int)(v1) >> 31)) * 10) + 48;
        v12 -= 1;
    } while ((unsigned int)v12);
    return v3;
}



// Function: code_timespec @ 0x24270
long long code_timespec(long long a0, unsigned int a1, unsigned long a2)
{
    long long v1;  // rax
    unsigned long v2;  // r10
    unsigned long v3;  // r10

    v1 = a0;
    if (a1 <= 0x3b9ac9ff)
    {
        if (!a1 || a0 >= 0)
            goto LABEL_4242d2;
        v1 += 1;
        a1 = 1000000000 - a1;
    }
    else
    {
        a1 = 0;
LABEL_4242d2:
        if (a0 >= 0)
        {
            sub_443090();
            sub_424170(a1, a2 + 21);
            return v3;
        }
    }
    *((char *)(sub_443090(-(v1), a2 + 1) - 1)) = 45;
    sub_424170(a1, a2 + 21);
    return v2;
}



// Function: decode_timespec @ 0x242f0
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

long long decode_timespec(char *a0, struct_0 **a1, char a2)
{
    char v1;  // r14b
    unsigned int *err;  // r12
    int iter;  // eax
    unsigned int v12;  // edx
    long long v3;  // r8
    struct_0 *node;  // rbp
    unsigned int v5;  // r10d
    struct_0 *v6;  // rax
    long long v7;  // rax
    unsigned int v8;  // esi
    unsigned long v9;  // rcx
    unsigned int v10;  // r9d

    v1 = *(a0);
    err = __errno_location();
    if (a0[v1 == 45] - 48 > 9)
    {
        *(err) = 22;
        *(a1) = a0;
        return -0x8000000000000000;
    }
    *(err) = 0;
    if (v1 != 45)
    {
        v3 = strtoumax(a0, a1, 10);
        if (v3 >= 0)
        {
            node = *(a1);
            v5 = *(err);
            if (a2)
                goto LABEL_42438c;
        }
        else
        {
            *(err) = 0x22;
            node = *(a1);
            if (!a2 || node->field_0 != 46)
            {
                *(a1) = node;
                return -0x8000000000000000;
            }
            v6 = node;
            v5 = 0x22;
            v3 = -0x8000000000000000;
LABEL_424395:
            v8 = v6->field_1;
            node = &node->field_1;
            v9 = v8;
            if (v8 - 48 <= 9)
            {
                v10 = 0;
                iter = 0;
                v12 = 0;
                do
                {
                    if (iter <= 8)
                    {
                        iter += 1;
                        v12 = v8 + v12 * 10 - 48;
                    }
                    else
                    {
                        v10 |= _INSERT(v9, 0, (char)v9 != 48);
                    }
                    v8 = node->field_1;
                    node = &node->field_1;
                    v9 = v8;
                } while (v8 - 48 <= 9);
                if (iter > 8)
                    goto LABEL_4243fd;
            }
            else
            {
                v12 = 0;
                v10 = 0;
                iter = 0;
            }
            do
            {
                iter += 1;
                v12 *= 10;
            } while (iter != 9);
LABEL_4243fd:
            if (v1 == 45 && v12 + (v10 & 0xff) && v3 != -0x8000000000000000)
                v3 -= 1;
        }
    }
    else
    {
        v7 = strtoimax(a0, a1, 10);
        node = *(a1);
        v5 = *(err);
        v3 = v7;
        if (a2)
        {
LABEL_42438c:
            v6 = node;
            if (node->field_0 == 46)
                goto LABEL_424395;
        }
    }
    *(a1) = node;
    if (v5 != 0x22)
        return v3;
    return v3;
}



// Function: must_be_dot_or_slash @ 0x244e0
unsigned int must_be_dot_or_slash(char *iter)
{
    unsigned long v1;  // rax

    v1 = *(iter);
    if ((char)v1 != 47)
    {
LABEL_4244eb:
        if ((char)v1 == 46)
        {
            do
            {
                v1 = iter[1];
                if ((char)v1 != 47)
                    return _INSERT(v1, 0, !(char)v1);
                iter += 2;
                if (iter[2] == 47)
                    goto LABEL_424510;
                else
                    goto LABEL_4244eb;
                do
                {
LABEL_424510:
                    v1 = iter[1];
                    iter += 1;
                } while ((char)v1 == 47);
            } while ((char)v1 == 46);
        }
    }
    else
    {
        while (1)
        {
            while (1)
            {
                if (iter[1] != 47)
                    break;
                iter += 1;
            }
            if ((char)v1 != 46)
                break;
            iter = &iter[(iter[2] == 46) + 2];
            if (*(iter) != 47)
                return 0;
        }
    }
    return _INSERT(v1, 0, !(char)v1);
}



// Function: safer_rmdir @ 0x24560
extern unsigned int g_481234;

unsigned int safer_rmdir(unsigned long a0)
{
    unsigned int v1;  // r12d

    if ((char)sub_4244e0(a0))
    {
        *(__errno_location()) = 0;
        return 0xffffffff;
    }
    v1 = unlinkat(g_481234, a0, 0x200);
    if (v1)
        return 0xffffffff;
    sub_419b90(a0);
    return v1;
}



// Function: undo_last_backup @ 0x245d0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern unsigned long long g_483f28;
extern long long g_4845f8;
extern long long g_484600;
extern unsigned int g_484a54;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void undo_last_backup(void)
{
    int *err;  // rax
    char *v2;  // rax
    char *v3;  // rax

    if (!g_4845f8)
        return;
    if (renameat(g_481234))
    {
        err = __errno_location();
        if (g_484d50)
            g_484d50();
        sub_44ac40(1, g_484600);
        sub_44aa70(g_4845f8);
        v2 = dcgettext(NULL, "%s: Cannot rename to %s", 5);
        error(0, *(err), v2);
        g_484d58 = 2;
    }
    if (g_484a54)
    {
        sub_44ac40(1, g_484600);
        sub_44ac40(0, g_4845f8);
        v3 = dcgettext(NULL, "Renaming %s back to %s\n", 5);
        __fprintf_chk(g_483f28, 0x1, v3);
        sub_423be0(&g_4845f8, 0);
        return;
    }
    sub_423be0(&g_4845f8, 0);
    return;
}



// Function: deref_stat @ 0x246f0
extern unsigned int g_481234;
extern unsigned int g_484800;

void deref_stat(long long a0, long long a1)
{
    fstatat(g_481234, a0, a1, g_484800);
    return;
}



// Function: maybe_backup_file @ 0x24710
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern unsigned long long g_483f28;
extern long long g_4845f8;
extern long long g_484600;
extern unsigned int g_484a54;
extern unsigned int g_484b80;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;
extern char g_484d70;
extern unsigned long long g_484d78;

unsigned int maybe_backup_file(unsigned long a0, char a1)
{
    int *err;  // rax
    char *v4;  // rax
    char *v5;  // rax
    char v0;  // [bp-0xc8]
    unsigned int v1;  // [bp-0xb0]

    sub_423be0(&g_484600, a0);
    sub_423be0(&g_4845f8, 0);
    if (a1 && !g_484d70 && !(g_484d78 = (unsigned long long)strchr(a0, 58), !g_484d78 || g_484d78 <= a0 || memchr(a0, 47, g_484d78 - a0)))
        return 1;
    if ((unsigned int)(unsigned long long)sub_4246f0(a0, &v0))
    {
        if (*(__errno_location()) != 2)
        {
            sub_432d30(a0);
            return 0;
        }
    }
    else
    {
        if (((unsigned short)v1 & 0xf000) != 0x4000 && (!a1 || ((unsigned short)v1 & 0xb000) != 0x2000))
        {
            g_4845f8 = sub_43e2f0(g_481234, a0, g_484b80);
            if (!g_4845f8)
            {
                sub_416420(); /* do not return */
            }
            else if (renameat(g_481234, g_484600, g_481234))
            {
                err = __errno_location();
                if (g_484d50)
                    g_484d50();
                sub_44ac40(1, g_4845f8);
                sub_44aa70(g_484600);
                v4 = dcgettext(NULL, "%s: Cannot rename to %s", 5);
                error(0, *(err), v4);
                g_484d58 = 2;
                sub_423be0(&g_4845f8, 0);
                return 0;
            }
            else if (g_484a54)
            {
                sub_44ac40(1, g_4845f8);
                sub_44ac40(0, g_484600);
                v5 = dcgettext(NULL, "Renaming %s to %s\n", 5);
                __fprintf_chk(g_483f28, 0x1, v5);
            }
        }
    }
    return 1;
}



// Function: blocking_read @ 0x24960
unsigned long long blocking_read(unsigned int a0, long long a1, long long a2)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    v1 = sub_44af10();
    if (v1 != 0xffffffffffffffff)
    {
        return v1;
    }
    else if (*(__errno_location()) != 11)
    {
        return v1;
    }
    else
    {
        v2 = sub_44d180(a0, 3);
        if ((unsigned int)v2 < 0)
        {
            return v1;
        }
        else if (!(*((char *)((void*)&v2 + 1)) & 8))
        {
            return v1;
        }
        else if (sub_44d180(a0, 4, _INSERT(v2, 1, *((char *)((void*)&v2 + 1)) & 247) & 0xffffffff) == 0xffffffff)
        {
            return v1;
        }
        else
        {
            return sub_44af10(a0, a1, a2);
        }
    }
}



// Function: blocking_write @ 0x249f0
unsigned long long blocking_write(unsigned int a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long v1;  // r12
    unsigned long long v2;  // rax

    v1 = sub_441120();
    if (a2 <= v1)
    {
        return v1;
    }
    else if (*(__errno_location()) != 11)
    {
        return v1;
    }
    else
    {
        v2 = sub_44d180(a0, 3);
        if ((unsigned int)v2 < 0)
        {
            return v1;
        }
        else if (!(*((char *)((void*)&v2 + 1)) & 8))
        {
            return v1;
        }
        else if (sub_44d180(a0, 4, _INSERT(v2, 1, *((char *)((void*)&v2 + 1)) & 247) & 0xffffffff) == 0xffffffff)
        {
            return v1;
        }
        else
        {
            return v1 + sub_441120(a0, a1 + v1, a2 - v1);
        }
    }
}



// Function: set_file_atime @ 0x24a80
int set_file_atime(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long long a3, unsigned long long a4)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x20]

    v1 = a4;
    v0 = a3;
    v2 = 0x3ffffffe;
    return sub_43f360();
}



// Function: chdir_count @ 0x24ae0
extern unsigned long long g_4845e8;

unsigned long long chdir_count(void)
{
    if (g_4845e8)
        return g_4845e8 - 1 & 0xffffffff;
    return 0;
}



// Function: chdir_arg @ 0x24b00
typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern unsigned long long g_4845e0;
extern unsigned long long g_4845e8;
extern struct_2 *g_4845f0;

unsigned long long chdir_arg(char *a0)
{
    unsigned long long v1;  // rax
    char *v2;  // rbx
    char *v3;  // rbx
    char *v4;  // rcx

    v1 = g_4845e8;
    v2 = a0;
    if (v1 == g_4845e0)
    {
        if (!v1)
            g_4845e0 = 2;
        g_4845f0 = sub_44c680(g_4845f0, &g_4845e0, 24);
        v1 = g_4845e8;
        if (!v1)
        {
            *((unsigned long long *)&g_4845f0->padding_0[8]) = 0;
            *((char **)&g_4845f0->padding_0[0]) = ".";
            v1 = 1;
            *((unsigned int *)&g_4845f0->field_10) = 0xffffff9c;
            g_4845e8 = 1;
        }
    }
    v3 = v2;
    v2 = v3;
    if (v2[1] != 47 && !v2[1])
        return (unsigned int)v1 - 1;
    v4 = v2 + 2;
    v3 = v4;
    if (v2[2] == 47)
    {
        while (1)
        {
            v3 = v4 + 1;
            if (v4[1] != 47)
                break;
            v4 = v3;
        }
        v3 = v3;
        if (v4[1] == 46)
            continue;
    }
}



// Function: tar_dirname @ 0x24c30
extern int g_484580;
extern unsigned long long g_4845f0;

long long tar_dirname(void)
{
    return *((long long *)(g_4845f0 + g_484580 * 24));
}



// Function: close_diag @ 0x24c50
extern char g_483b82;
extern char g_484b52;

void close_diag(long long a0)
{
    if (!g_484b52)
    {
        sub_432680();
        return;
    }
    if (!(g_483b82 & 128))
        return;
    sub_4324f0("close", a0);
    return;
}



// Function: chdir_do @ 0x24c80
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[8];
    int field_10;
} struct_0;

typedef struct struct_2 {
    long long field_0;
    char padding_8[8];
    int field_10;
} struct_2;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned int g_481234;
extern unsigned int g_484580;
extern void* g_484590;
extern unsigned int g_4845a0;
extern void g_4845a4;
extern int g_4845dc;
extern void* g_4845f0;

void* chdir_do(unsigned int i)
{
    unsigned long long v3;  // r12
    void* iter;  // rax
    long long v13;  // r8
    long long v14;  // r9
    unsigned int v15;  // edx
    unsigned int v16;  // edx
    void* v17;  // rax
    unsigned long long v5;  // rbx
    struct_0 *idx;  // r12
    int v7;  // ebp
    struct_2 *v8;  // r12
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    long long v12;  // rcx
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (g_484580 == i)
        return v17;
    v1 = v3;
    iter = g_4845f0;
    v0 = v5;
    idx = g_4845f0 + i * 24;
    v7 = idx->field_10;
    if (!v7)
    {
        if (idx->field_0->field_0 != 47)
            chdir_do(i - 1);
        v7 = __openat_2(g_481234);
        if (v7 < 0)
        {
            sub_4327e0(idx->field_0);
            return sub_424dd0(v9, v10, v11, v12, v13, v14);
        }
        iter = g_484590;
        idx->field_10 = v7;
        if (iter <= 0xf)
        {
            g_484590 = iter + 1;
            *((unsigned int *)((char *)&g_4845a0 + 0x4 * iter)) = i;
        }
        else
        {
            v8 = g_4845f0 + g_4845dc * 24;
            iter = close(v8->field_10);
            if ((unsigned int)iter)
                iter = (unsigned long long)sub_424c50(v8->field_0);
            v8->field_10 = 0;
            g_4845dc = i;
            if (v7 <= 0)
            {
                g_484580 = i;
                g_481234 = v7;
                return iter;
            }
LABEL_424d0a:
            v15 = g_4845a0;
            if (i != g_4845a0)
            {
                iter = &g_4845a4;
                do
                {
                    v16 = *((int *)iter);
                    iter += 4;
                    *((unsigned int *)((char *)iter - 4)) = v15;
                    v15 = v16;
                } while (i != v15);
            }
            g_4845a0 = i;
            g_484580 = i;
            g_481234 = v7;
            return iter;
        }
    }
    if (v7 <= 0)
    {
        g_484580 = i;
        g_481234 = v7;
        return iter;
    }
    goto LABEL_424d0a;
}



// Function: open_diag @ 0x24dd0
extern char g_483b82;
extern char g_484b52;

void open_diag(unsigned long a0)
{
    if (!g_484b52)
    {
        sub_4327c0();
        return;
    }
    if (!(g_483b82 & 128))
        return;
    sub_432800();
    return;
}



// Function: read_diag_details @ 0x24e00
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b82;
extern char g_484b52;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void read_diag_details(long long a0, long long a1, unsigned int a2)
{
    unsigned long long v7;  // r13
    unsigned long long v8;  // r12
    unsigned long v17;  // fs
    int *err;  // rax
    char *v19;  // rax
    unsigned long long v9;  // rbp
    unsigned long long v10;  // rbx
    unsigned long v11;  // fs
    int *err1;  // rax
    char *v13;  // rax
    unsigned long long v14;  // r13
    unsigned long long v15;  // r12
    unsigned long long v16;  // rbx
    char v0;  // [bp-0x48]
    unsigned long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    unsigned long long v4;  // [bp-0x10]
    unsigned long long v5;  // [bp-0x8]

    if (!g_484b52)
    {
        v5 = v14;
        v4 = v15;
        v2 = v16;
        v1 = *((long long *)(40 + v17));
        err = __errno_location();
        if (g_484d50)
            g_484d50();
        sub_443090(a1, &v0);
        sub_44aa70(a0);
        v19 = dcngettext(NULL, "%s: Read error at byte %s, while reading %lu byte", "%s: Read error at byte %s, while reading %lu bytes", a2, 5);
        error(0, *(err), v19);
        g_484d58 = 2;
        if (v1 != *((long long *)(40 + v17)))
            __stack_chk_fail(); /* do not return */
        return;
    }
    else if (!(g_483b82 & 128))
    {
        return;
    }
    else
    {
        v5 = v7;
        v4 = v8;
        v3 = v9;
        v2 = v10;
        v1 = *((long long *)(40 + v11));
        err1 = __errno_location();
        if (g_484d50)
            g_484d50();
        sub_443090(a1, &v0);
        sub_44aa70(a0);
        v13 = dcngettext(NULL, "%s: Warning: Read error at byte %s, while reading %lu byte", "%s: Warning: Read error at byte %s, while reading %lu bytes", a2, 5);
        error(0, *(err1), v13);
        if (v1 != *((long long *)(40 + v11)))
            __stack_chk_fail(); /* do not return */
        return;
    }
}



// Function: readlink_diag @ 0x24e30
extern char g_483b82;
extern char g_484b52;

void readlink_diag(unsigned long a0)
{
    if (!g_484b52)
    {
        sub_432a60();
        return;
    }
    if (!(g_483b82 & 128))
        return;
    sub_432a80();
    return;
}



// Function: savedir_diag @ 0x24e60
extern char g_483b82;
extern char g_484b52;

void savedir_diag(long long a0)
{
    if (!g_484b52)
    {
        sub_432ac0();
        return;
    }
    if (!(g_483b82 & 128))
        return;
    sub_4324f0("savedir", a0);
    return;
}



// Function: seek_diag_details @ 0x24e90
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b82;
extern char g_484b52;
extern struct_0 *g_484d50;

void seek_diag_details(long long a0, long long a1)
{
    unsigned long long v6;  // r13
    unsigned long long v7;  // r12
    unsigned long long v8;  // rbx
    unsigned long v9;  // fs
    int *err;  // rax
    char *v11;  // rax
    char v0;  // [bp-0x48]
    unsigned long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!g_484b52)
    {
        sub_432b20();
        return;
    }
    else if (!(g_483b82 & 128))
    {
        return;
    }
    else
    {
        v4 = v6;
        v3 = v7;
        v2 = v8;
        v1 = *((long long *)(40 + v9));
        err = __errno_location();
        if (g_484d50)
            g_484d50();
        sub_443090(a1, &v0);
        sub_44aa70(a0);
        v11 = dcgettext(NULL, "%s: Warning: Cannot seek to %s", 5);
        error(0, *(err), v11);
        if (v1 != *((long long *)(40 + v9)))
            __stack_chk_fail(); /* do not return */
        return;
    }
}



// Function: stat_diag @ 0x24ec0
extern char g_483b82;
extern char g_484b52;

void stat_diag(unsigned long a0)
{
    if (!g_484b52)
    {
        sub_432d30();
        return;
    }
    if (!(g_483b82 & 128))
        return;
    sub_432d50();
    return;
}



// Function: file_removed_diag @ 0x24ef0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b80;
extern struct_0 *g_484d50;

long long file_removed_diag(long long a0, char a1, unsigned long a2)
{
    if (!a1 && *(__errno_location()) == 2)
    {
        if (!(g_483b80 & 64))
            return sub_42f7d0(1);
        if (g_484d50)
            g_484d50();
        sub_44aa70(a0);
        error(0, 0, dcgettext(NULL, "%s: File removed before we read it", 5));
        return sub_42f7d0(1);
    }
    goto *((void *)(a2));
}



// Function: xfork @ 0x24f80
unsigned long long xfork(void)
{
    unsigned long long pid;  // rax

    pid = fork();
    if ((unsigned int)pid == 0xffffffff)
        sub_432490("fork", dcgettext(NULL, "child process", 5)); /* do not return */
    return pid;
}



// Function: xpipe @ 0x24fc0
unsigned long long xpipe(int a0[2])
{
    unsigned long long v1;  // rax

    v1 = pipe(a0);
    if ((unsigned int)v1 < 0)
        sub_432490("pipe", dcgettext(NULL, "interprocess channel", 5)); /* do not return */
    return v1;
}



// Function: page_aligned_alloc @ 0x25000
int page_aligned_alloc(unsigned long long *a0, unsigned long a1)
{
    unsigned long v1;  // rbp

    v1 = (int)getpagesize();
    if (__CFADD__(a1, v1) & 1)
        sub_416420(); /* do not return */
    *(a0) = sub_44c5b0(a1 + v1);
}



// Function: namebuf_create @ 0x25040
typedef struct struct_0 {
    char *field_0;
    char field_8;
    char padding_9[7];
    char field_10;
} struct_0;

struct_0 * namebuf_create(char *a0)
{
    struct_0 *idx;  // r12
    unsigned long v2;  // rdi
    char *ptr;  // rbx
    unsigned long len;  // rax
    char v5;  // 4098

    idx = sub_44c5b0(24);
    v2 = strlen(a0) + 2;
    *((unsigned long *)&idx->field_8) = v2;
    ptr = sub_44c5b0(v2);
    idx->field_0 = ptr;
    strcpy(ptr, a0);
    len = strlen(ptr);
    v5 = *(&ptr[len] - 1);
    *((unsigned long *)&idx->field_10) = len;
    if (v5 != 47)
    {
        ptr[len] = 47;
        *((unsigned long long *)&idx->field_10) = len + 1;
    }
    return idx;
}



// Function: namebuf_free @ 0x250b0
void namebuf_free(void* *a0)
{
    free(*(a0));
    free(a0);
    return;
}



// Function: namebuf_name @ 0x250d0
long long namebuf_name(unsigned long long *idx, char *ptr)
{
    unsigned long len;  // rbp
    unsigned long v2;  // rax
    unsigned long v3;  // rdi
    unsigned long v4;  // rax

    len = strlen(ptr);
    v2 = idx[2];
    if (idx[1] <= v2 + len + 1)
    {
        v3 = *(idx);
        do
        {
            v4 = sub_44c710(v3, idx + 1);
            *(idx) = v4;
            v3 = v4;
            v2 = idx[2];
        } while (idx[2] + len + 1 >= idx[1]);
    }
    else
    {
        v3 = *(idx);
    }
    strcpy(v3 + v2, ptr);
    return *(idx);
}



// Function: namebuf_add_dir @ 0x25150
long long namebuf_add_dir(unsigned long long *idx, char *a1)
{
    unsigned long len;  // rax

    if (*((char *)(*(idx) + idx[2] - 1)) != 47)
    {
        sub_4250d0(idx, "/");
        idx[2] = idx[2] + 1;
    }
    sub_4250d0(idx, a1);
    len = strlen(a1);
    idx[2] = idx[2] + len;
    return len;
}



// Function: tar_getcdpath @ 0x251a0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_2 {
    long long field_0;
    unsigned long long field_8;
} struct_2;

extern unsigned int g_484580;
extern unsigned long long g_484588;
extern struct_0 *g_4845f0;

unsigned long long tar_getcdpath(unsigned int a0)
{
    unsigned long long v2;  // r13
    struct_0 *v3;  // rax
    unsigned long long v12;  // rax
    unsigned int v4;  // edx
    unsigned int v5;  // ebx
    unsigned int v6;  // ebx
    unsigned long long v7;  // rax
    unsigned long long v8;  // r14
    unsigned long long v9;  // rbp
    struct_2 *v10;  // r15
    void* v11;  // r15
    unsigned int v0;  // [bp-0x3c]

    if (g_4845f0)
    {
        v2 = a0 * 24;
        if (*((long long *)((char *)g_4845f0 + v2 + 8)))
            return *((long long *)((char *)g_4845f0 + v2 + 8));
        v0 = g_484580;
        if (a0 >= 0)
        {
            v3 = (char *)g_4845f0 + v2 - 16;
            v4 = a0;
            do
            {
                v4 -= 1;
                if (!v4)
                    goto LABEL_4252cc;
            } while ((v3 -= 24, !v3[1].field_8));
        }
        else
        {
            v5 = a0 + 1;
        }
        if (v5 > a0)
        {
            sub_424c80(v0);
            return *((long long *)((char *)g_4845f0 + v2 + 8));
        }
        while (1)
        {
LABEL_4252cc:
            sub_424c80(v5);
            v6 = v5;
            if (!v6)
            {
                v7 = sub_44c7c0();
                g_4845f0[1].field_0 = v7;
                if (!v7)
                    sub_432490("getcwd", "."); /* do not return */
                if (a0 <= 0)
                {
                    sub_424c80(v0);
                    return *((long long *)((char *)g_4845f0 + v2 + 8));
                }
                v6 = 1;
                sub_424c80(1);
            }
            v5 = v6 + 1;
            v8 = v6 * 3;
            v9 = v8 * 8;
            v10 = (char *)g_4845f0 + v9;
            if (*((char *)v10->field_0) != 47)
            {
                v11 = sub_425040(*((long long *)((char *)g_4845f0 + v9 - 16)));
                sub_425150(v11, g_4845f0[v8].field_0);
                v12 = sub_423b60(v11);
                *((unsigned long long *)(v9 + (char *)g_4845f0 + 8)) = v12;
            }
            else
            {
                v10->field_8 = sub_44c7a0(v10->field_0);
            }
            if (a0 < v5)
            {
                sub_424c80(v0);
                return *((long long *)((char *)g_4845f0 + v2 + 8));
            }
        }
    }
    else if (g_484588)
    {
        return g_484588;
    }
    else
    {
        g_484588 = sub_44c7c0();
        if (!g_484588)
            sub_432490("getcwd", "."); /* do not return */
        return g_484588;
    }
}



// Function: normalize_filename @ 0x25340
void* normalize_filename(unsigned int a0, char *a1)
{
    unsigned long ptr;  // rax
    unsigned long len;  // rax
    unsigned long v3;  // rbx
    void* v4;  // rax
    void* v5;  // r12
    void* v6;  // r12

    if (*(a1) == 47)
    {
        v6 = sub_44c7a0(a1);
        sub_423ea0(v6);
        return v6;
    }
    ptr = sub_4251a0(a0);
    len = strlen(ptr);
    v3 = len + 1;
    v4 = sub_44c5b0(v3 + strlen(a1) + 1);
    v5 = v4;
    strcpy(v4, ptr);
    *((char *)v5 + len) = 47;
    strcpy(v5 + v3, a1);
    sub_423ea0(v5);
    return v5;
}



// Function: tar_savedir @ 0x253c0
extern unsigned int g_481234;
extern unsigned int g_4847e4;
extern unsigned int g_484808;

long long tar_savedir(long long a0, unsigned int a1)
{
    unsigned int v1;  // eax
    unsigned long fp;  // r13
    unsigned long v3;  // rax
    unsigned long v4;  // r13

    v1 = __openat_2(g_481234, a0, g_484808 | 0x10000);
    if (v1 >= 0)
    {
        fp = fdopendir(v1);
        if (fp)
        {
            v3 = sub_44b0d0(fp, g_4847e4);
            if (!v3)
                sub_432ac0(a0);
            v4 = v3;
            if (!closedir(fp))
                return v4;
            sub_432ac0(a0);
            return v4;
        }
        else
        {
            sub_432ac0(a0);
            if (close(v1))
            {
                sub_432ac0(a0);
                return fp;
            }
        }
    }
    else if (a1 || *(__errno_location()) != 2)
    {
        sub_4327c0(a0);
    }
    return 0;
}



// Function: remove_any_file @ 0x25490
extern unsigned int g_481234;

unsigned int remove_any_file(long long a0, unsigned int a1)
{
    unsigned int *err;  // rbx
    unsigned long v5;  // rdx
    unsigned long v6;  // r13
    unsigned long ptr;  // r14
    unsigned long len;  // rax
    unsigned long v9;  // r14
    unsigned long v10;  // rax
    unsigned long v0;  // [bp-0x168]
    unsigned int v1;  // [bp-0x40]
    unsigned int v2;  // [bp-0x3c]

    if (!unlinkat(g_481234, a0, 0))
        return 1;
    err = __errno_location();
    if (*(err) != 1 && *(err) != 21)
        return 0;
    if (sub_424560(a0))
    {
        v5 = *(err);
        if ((unsigned int)v5 > 39)
            return 0;
        v0 = 549755944961;
        if (!(((char)v5 & 254 | (char)(*((char *)&v0 + ((long long)(v5 & 63) >> 3)) >> ((char)v5 & 63 & 7)) & 1) & 1))
        {
            return 0;
        }
        else if (a1 != 1)
        {
            return -(a1 == 2);
        }
        else
        {
            v6 = sub_4253c0(a0, 0);
            if (v6)
            {
                ptr = v6;
                len = strlen(ptr);
                v9 = ptr;
                if (len)
                {
                    do
                    {
                        v10 = sub_427bc0(a0, v9);
                        v1 = remove_any_file(v10, 1);
                        v2 = *(err);
                        free(v10);
                        if (!v1)
                        {
                            free(ptr);
                            *(err) = v2;
                            return v1;
                        }
                    } while ((v9 = (unsigned long)(v9 + len + 1), len = (unsigned long)strlen(v9), len));
                }
                free(ptr);
                return !sub_424560(a0);
            }
        }
    }
    else
    {
        return 1;
    }
    return 0;
}



// Function: file_selection_option @ 0x255e0
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[20];
    unsigned long long field_20;
} struct_0;

extern struct_0 g_481280;

struct_0 * file_selection_option(unsigned int a0)
{
    struct_0 *v1;  // rax

    v1 = &g_481280.field_0;
    while (1)
    {
        if (!v1->field_0 && !v1->field_8 && !v1->field_20)
            return NULL;
        if (a0 == v1->field_8)
            return v1;
        v1 = &v1[1].field_8;
    }
}



// Function: file_selection_option_name @ 0x25610
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[20];
    unsigned long long field_20;
} struct_0;

struct_0 * file_selection_option_name(unsigned int a0)
{
    struct_0 *v1;  // rax

    v1 = sub_4255e0(a0);
    if (!v1)
        return v1;
    return v1->field_0;
}



// Function: is_file_selection_option @ 0x25620
char is_file_selection_option(unsigned int a0)
{
    return sub_4255e0(a0);
}



// Function: name_list_adjust @ 0x25630
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

extern struct_0 *g_4846b0;

struct_0 * name_list_adjust(void)
{
    struct_0 *v1;  // rax
    struct_0 *i;  // rax
    struct_0 *v3;  // rax
    struct_0 *v4;  // rax

    v1 = g_4846b0;
    if (!v1)
        return v1;
    i = v1->field_8;
    if (!i)
        return i;
    do
    {
        v3 = i;
        v4 = v3->field_8;
        i = v4;
    } while (i);
    g_4846b0 = v3;
    return v4;
}



// Function: merge_sort_sll @ 0x25690
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

struct_0 ** merge_sort_sll(struct_0 **a0, unsigned int a1, unsigned long long *a2)
{
    struct_0 **v2;  // rax
    int v3;  // edx
    struct_0 **v12;  // rbx
    struct_0 **v13;  // rbp
    struct_0 **v14;  // rax
    int v4;  // edx
    int i;  // edx
    int v6;  // edx
    struct_0 **v7;  // r14
    struct_0 **v8;  // rbp
    struct_0 **v9;  // rbx
    struct_0 **v10;  // r13
    struct_0 **v11;  // rax
    struct_0 **v0;  // [bp-0x38], Other Possible Types: char

    if (a1 != 1)
    {
        if (a1 != 2)
        {
            v2 = a0;
            v3 = (int)((a1 + 1 >> 31) + a1 + 1) >> 1;
            v4 = v3 - 1;
            if (v3 != 1)
            {
                do
                {
                    i = v4;
                    v2 = *(v2);
                    v6 = i - 1;
                    v4 = v6;
                } while (i != 1);
            }
            v7 = *(v2);
            *(v2) = NULL;
            v8 = merge_sort_sll(a0, v3, a2);
            v9 = merge_sort_sll(v7, (int)((a1 >> 31) + a1) >> 1, a2);
            if (v8 && v9)
            {
                v10 = &v0;
                while (1)
                {
                    if (a2(v8, v9) >= 0)
                    {
                        v11 = v9;
                        v12 = *(v9);
                        *(v10) = v9;
                        v9 = v12;
                        if (!v8)
                            break;
                    }
                    else
                    {
                        v11 = v8;
                        v13 = *(v8);
                        *(v10) = v8;
                        v8 = v13;
                        v12 = v9;
                        if (!v8)
                            break;
                    }
                    v13 = v8;
                    if (!v9)
                        break;
                    v10 = v11;
                }
            }
            else
            {
                v11 = &v0;
                v13 = v8;
                v12 = v9;
            }
            if (!v13)
                v13 = v12;
            *(v11) = v13;
            return v0;
        }
        else
        {
            a0 = a0;
            if (a2() > 0)
            {
                v14 = *(a0);
                *(v14) = a0;
                *(a0) = NULL;
                return v14;
            }
        }
    }
    return a0;
}



// Function: merge_sort @ 0x25800
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
} struct_0;

struct_0 * merge_sort(struct_0 ***a0, unsigned int a1, unsigned long long *a2)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // rsi
    struct_0 *v3;  // rdx
    struct_0 *v4;  // rsi
    struct_0 *v5;  // rcx

    v1 = sub_425690(a0, a1, a2);
    if (!v1)
        return v1;
    v2 = NULL;
    v3 = v1;
    while (1)
    {
        v4 = v2;
        v5 = v3->field_0;
        v3->field_8 = v4;
        if (!v5)
            break;
        v3 = v5;
        v2 = v3;
    }
    return v1;
}



// Function: file_list_name @ 0x25830
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
    char padding_20[16];
    unsigned long long field_30;
} struct_0;

extern struct_0 *g_4846b0;

char * file_list_name(void)
{
    struct_0 *v1;  // rax
    struct_0 *v2;  // rax

    v1 = g_4846b0;
    if (g_4846b0)
    {
        do
        {
            v2 = v1;
            if (v2->field_10 == 2 && v2->field_30)
                return v2->field_18;
        } while ((v1 = (struct_0 *)v2->field_0, v2->field_0));
    }
    return dcgettext(NULL, "command line", 5);
}



// Function: name_elt_alloc @ 0x25880
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    unsigned int field_10;
} struct_0;

extern struct_0 *g_4846b0;

void name_elt_alloc(void)
{
    struct_0 *idx;  // rax
    struct_0 *index;  // rdx
    struct_0 *v3;  // rcx

    idx = sub_44c5b0(56);
    index = g_4846b0;
    if (!index)
    {
        idx->field_0 = NULL;
        idx->field_8 = NULL;
        idx->field_10 = 3;
        g_4846b0 = idx;
        idx = sub_44c5b0(56);
        index = g_4846b0;
    }
    v3 = index->field_8;
    idx->field_8 = v3;
    if (v3)
        v3->field_0 = idx;
    idx->field_0 = index;
    index->field_8 = idx;
    return;
}



// Function: name_add_dir @ 0x258f0
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
} struct_0;

void name_add_dir(unsigned long a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_425880();
    idx->field_10 = 1;
    idx->field_18 = a0;
    return;
}



// Function: name_add_file @ 0x25910
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
} struct_0;

extern unsigned int g_484630;

void name_add_file(unsigned long a0)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_425880();
    g_484630 = 2;
    idx->field_10 = 2;
    idx->field_18 = a0;
    idx->field_20 = 0;
    idx->field_30 = 0;
    return;
}



// Function: name_add_option @ 0x25940
void name_add_option(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_425947();
    return;
}



// Function: compare_names_found @ 0x25970
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
} struct_0;

extern unsigned long long g_484b38;

int compare_names_found(struct_0 *a0, struct_0 *a1)
{
    unsigned int v1;  // eax
    unsigned int v2;  // edx
    unsigned int v3;  // eax
    unsigned int v4;  // edx

    if (!g_484b38)
    {
        v1 = *((long long *)&a1[2].padding_0[0]);
        v2 = *((long long *)&a0[2].padding_0[0]);
        if (v1 != v2)
            return v1 - v2;
    }
    else
    {
        v3 = g_484b38 <= *((long long *)&a1[2].padding_0[0]);
        v4 = g_484b38 <= *((long long *)&a0[2].padding_0[0]);
        if (v3 != v4)
            return v3 - v4;
    }
    return strcmp(a0->field_10, a1->field_10);
}



// Function: name_compare @ 0x259d0
typedef struct struct_0 {
    char padding_0[88];
    char *field_58;
} struct_0;

char name_compare(struct_0 *a0, struct_0 *a1)
{
    return !strcmp(a0->field_58, a1->field_58);
}



// Function: compare_names @ 0x259f0
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
} struct_0;

int compare_names(struct_0 *a0, struct_0 *a1)
{
    return strcmp(a0->field_10, a1->field_10);
}



// Function: unconsumed_option_free @ 0x25a10
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

extern struct_0 *g_484628;

void unconsumed_option_free(void)
{
    struct_0 *v1;  // rdi

    v1 = g_484628;
    if (!g_484628)
        return;
    do
    {
        g_484628 = v1->field_8;
        free(v1);
        v1 = g_484628;
    } while (g_484628);
    return;
}



// Function: name_list_advance @ 0x25a50
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned int field_10;
} struct_0;

extern struct_0 *g_4846b0;
extern unsigned int g_484ba8;

long long name_list_advance(void)
{
    struct_0 *v1;  // rbp

    v1 = g_4846b0;
    g_4846b0 = v1->field_0;
    if (g_4846b0)
        g_4846b0[1].field_0 = 0;
    switch (v1->field_10)
    {
    case 1: case 4:
        if (g_484ba8 != 3 && g_484ba8 != 8)
            return g_484ba8;
        break;
    case 3:
        return (unsigned long long)free(v1);
    default:
        sub_425a10();
        return (unsigned long long)free(v1);
    }
}



// Function: free_name @ 0x25ad0
void free_name(void* *a0)
{
    if (a0)
    {
        free(a0[2]);
        free(a0[11]);
        free(a0);
    }
    return;
}



// Function: add_file_id @ 0x25b00
typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char *field_18;
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
    struct struct_0 *field_0;
} struct_0;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

extern struct_0 *g_484698;
extern struct_3 *g_484d50;
extern unsigned int g_484d58;

unsigned int add_file_id(unsigned long a0)
{
    unsigned int v2;  // eax
    char *v3;  // rax
    struct_0 **v4;  // rbx
    struct_0 **v5;  // rbx
    unsigned int v6;  // eax
    struct_2 *idx;  // rax
    stat v0;  // [bp-0xc8]

    v2 = stat(a0, &v0);
    if (v2)
        sub_432d10(a0); /* do not return */
    v3 = sub_425830();
    v4 = g_484698;
    if (g_484698)
    {
        do
        {
            v5 = v4;
            if (v5[1] == v0.st_ino && v5[2] == *((unsigned long long *)&v0))
            {
                v6 = sub_44a5b0(0, 58, 1);
                if (g_484d50)
                    g_484d50();
                sub_44a870(0, a0);
                error(0, 0, dcgettext(NULL, "%s: file list requested from %s already read from %s", 5));
                g_484d58 = 2;
                sub_44a5b0(0, 58, v6);
                return 1;
            }
        } while ((v4 = (struct_0 **)*(v5), *(v5)));
    }
    idx = sub_44c5b0(32);
    idx->field_18 = v3;
    idx->field_0 = g_484698;
    g_484698 = idx;
    idx->field_8 = v0.st_ino;
    idx->field_10 = *((unsigned long long *)&v0);
    return v2;
}



// Function: chopslash @ 0x25c70
void* chopslash(void* a0)
{
    void* iter;  // rax

    iter = a0 + strlen(a0) - 1;
    if (a0 >= iter)
        return iter;
    while (*((char *)iter) == 47)
    {
        iter -= 1;
        *((char *)&iter[1]) = 0;
        if (a0 == iter)
            return iter;
    }
    return iter;
}



// Function: read_name_from_file @ 0x25ca0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned int field_28;
    char padding_2c[4];
    struct FILE *field_30;
} struct_0;

extern long long g_4846a0;
extern unsigned long long g_4846a8;

unsigned int read_name_from_file(struct_0 *idx)
{
    FILE *v1;  // r13
    unsigned int v2;  // r14d
    unsigned int v3;  // ebx
    char v4;  // al
    unsigned long long v5;  // rbp
    unsigned long long v6;  // r12
    char v7;  // al
    unsigned int v8;  // ebp
    unsigned long v9;  // rdi

    idx->field_20 = idx->field_20 + 1;
    v1 = idx->field_30;
    v2 = idx->field_28;
    v3 = getc_unlocked(v1);
    v4 = v3 != 0xffffffff;
    if (v2 != v3 && v4)
    {
        v5 = 0;
        while (1)
        {
            if (g_4846a0 != v5)
            {
                *((char *)(g_4846a8 + v5)) = v3;
                v6 = v5 + 1;
                if (!v3)
                    return 2;
            }
            else
            {
                v6 = v5 + 1;
                g_4846a8 = sub_44c710(g_4846a8, &g_4846a0);
                *((char *)(g_4846a8 + v5)) = v3;
                if (!v3)
                    return 2;
            }
            v3 = getc_unlocked(v1);
            if (v2 == v3 || v3 == 0xffffffff)
                break;
            v5 = v6;
        }
        v8 = _INSERT(v5, 0, !v6);
        v4 = v7 & (char)v8;
    }
    else
    {
        v8 = 1;
        v6 = 0;
    }
    if (v4)
        return 3;
    v9 = g_4846a8;
    if (g_4846a0 == v6)
    {
        g_4846a8 = sub_44c710(v9, &g_4846a0);
        v9 = g_4846a8;
    }
    *((char *)(v9 + v6)) = 0;
    sub_425c70(v9);
    return v3 == 0xffffffff & v8;
}



// Function: handle_option @ 0x25dc0
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern struct_1 *g_484d50;
extern unsigned long long g_484dd0;

unsigned int handle_option(struct_0 *a0, unsigned long long *idx)
{
    char v7;  // bl
    unsigned long long *v8;  // rax
    struct_0 *v9;  // r12
    unsigned int v10;  // r13d
    unsigned long long *iter;  // rax
    unsigned int v0;  // [bp-0x138]
    unsigned long v1;  // [bp-0x130]
    unsigned long v2;  // [bp-0x128]
    char v3;  // [bp-0x118], Other Possible Types: unsigned long
    unsigned long long *v4;  // [bp-0x110]
    unsigned long long v5;  // [bp-0x108]

    v7 = a0->field_0;
    if (!a0->field_0)
        return 1;
    v8 = __ctype_b_loc();
    do
    {
        v9 = a0;
        if (!(*((char *)(*(v8) + v7 * 2 + 1)) & 32))
        {
            if (v7 == 45)
            {
                v5 = 1;
                v10 = sub_4382c0(v9, &v3, 33558086);
                if (!v10)
                {
                    *(v4) = g_484dd0;
                    v0 = 2;
                    v1 = idx[3];
                    v2 = idx[4];
                    sub_42f5b0((unsigned int)v5 + (unsigned int)v3, v4, &v0);
                    if (v3 + v5)
                    {
                        iter = v4;
                        do
                        {
                            *(iter) = 0;
                            iter += 1;
                        } while (&v4[v5 + v3] != iter);
                    }
                    sub_435420(&v3);
                    return v10;
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    sub_4354b0(&v3);
                    error(0, 0, dcgettext(NULL, "cannot split string '%s': %s", 5));
                    sub_4163d0(); /* do not return */
                }
            }
            break;
        }
    } while ((v7 = v9->field_1, a0 = v9 + 1, v9->field_1));
    return 1;
}



// Function: handle_file_selection_option @ 0x25f80
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481238;
extern char g_48123c;
extern unsigned int g_4846f8;
extern unsigned int g_4846fc;
extern char g_484700;
extern char g_4847e8;
extern unsigned int g_484ad4;
extern long long g_484b60;
extern struct_0 *g_484d50;

long long handle_file_selection_option(unsigned int a0, unsigned long a1)
{
    unsigned long v3;  // rax
    unsigned long v5;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x1c]

    switch (a0)
    {
    case 88:
        v5 = sub_43f2a0(sub_43ee30, g_484b60, a1, g_4846f8 | g_484ad4 | (g_4846fc != 1) * 0x10000000, 10);
        if (!(unsigned int)v5)
            return v5;
        err = __errno_location();
        if (g_484d50)
            g_484d50();
        sub_44aa70(a1);
        error(0, *(err), "%s");
        sub_4163d0(); /* do not return */
    case 260:
        goto LABEL_426168;
    case 261:
        return sub_43ee30(g_484b60);
    case 262:
        return sub_416820(a1, 2);
    case 263:
        return sub_416820(a1, 1);
    case 264:
        break;
    case 265:
        break;
    case 266:
        break;
    case 269:
        g_4846f8 = g_4846f8 | 16;
        return v3;
    case 270:
        g_4846f8 = g_4846f8 & 0xffffffef;
        return v3;
    case 271:
        g_4846f8 = g_4846f8 | 0x40000000;
        return v3;
    case 272:
        g_4846f8 = g_4846f8 & 0xbfffffff;
        g_481238 = 0;
        return v3;
    case 273:
        g_484ad4 = 8;
        return v3;
    case 274:
        g_484ad4 = 0;
        return v3;
    case 275:
        g_4847e8 = 1;
        return v3;
    case 276:
        g_4847e8 = 0;
        return v3;
    case 277:
        g_484700 = 0;
        return v3;
    case 278:
        g_4846f8 = g_4846f8 | 1;
        return v3;
    case 279:
        g_4846fc = 1;
        return v3;
    case 280:
        g_48123c = 0;
        g_484700 = 1;
        return v3;
    case 281:
        g_48123c = 10;
        g_484700 = 0;
        return v3;
    case 282:
        g_484700 = 1;
        return v3;
    case 283:
        g_4846f8 = g_4846f8 & 0xfffffffe;
        return v3;
    case 284:
        g_4846fc = 2;
        return v3;
    case 258:
LABEL_426168:
    default:
        if (g_484d50)
        {
            v0 = a0;
            g_484d50();
        }
        error(0, 0, "unhandled positional option %d");
        sub_4163d0(); /* do not return */
    }
}



// Function: unconsumed_option_report @ 0x26210
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern void* g_484628;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void unconsumed_option_report(void)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    char *v16;  // rsi
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    void* v12;  // rax
    void* v13;  // rax
    void* v14;  // rbx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (!g_484628)
        return;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly.", 5));
    v12 = g_484628;
    g_484d58 = 2;
    do
    {
        v13 = v12;
        v14 = v13;
        v12 = (long long)v13[8];
    } while ((long long)v13[8]);
    while (1)
    {
        if ((int)v14[16] == 1)
        {
            if (g_484d50)
                g_484d50();
            sub_44ac50((long long)v14[24]);
            goto LABEL_42632e;
        }
        if ((int)v14[16] != 4)
        {
LABEL_4262fc:
            v14 = *((long long *)v14);
            if (!v14)
            {
                sub_425a10();
                return;
            }
        }
        else if ((long long)v14[32])
        {
            if (g_484d50)
                g_484d50();
            sub_44ac50();
            sub_425610((int)v14[24]);
            error(0, 0, dcgettext(NULL, v16, 5));
            g_484d58 = 2;
            goto LABEL_4262fc;
        }
        else
        {
            if (g_484d50)
                g_484d50();
            sub_425610((int)v14[24]);
LABEL_42632e:
            error(0, 0, dcgettext(NULL, v16, 5));
            v14 = *((long long *)v14);
            g_484d58 = 2;
            if (!v14)
            {
                sub_425a10();
                return;
            }
        }
    }
}



// Function: make_name @ 0x263a0
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
    char field_18;
} struct_0;

struct_0 * make_name(unsigned long long a0)
{
    struct_0 *idx;  // r12
    char *v2;  // rax

    idx = sub_44c760(96);
    v2 = sub_44c7a0();
    idx->field_10 = v2;
    *((unsigned long *)&idx->field_18) = strlen(v2);
    return idx;
}



// Function: regex_usage_warning @ 0x263f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_484634;
extern unsigned int g_4846fc;
extern struct_0 *g_484d50;

int regex_usage_warning(void)
{
    if (g_4846fc)
        return g_484634;
    if (!(char)sub_43ead0())
        return g_484634;
    g_484634 = 1;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Pattern matching characters used in file names", 5));
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning", 5));
    return g_484634;
}



// Function: name_hash @ 0x26490
void name_hash(unsigned long a0)
{
}



// Function: name_next_elt @ 0x267d0
extern unsigned int g_463688[4];
extern void g_484660;
extern unsigned int g_484670;
extern unsigned long long g_484678;
extern unsigned long long g_4846a8;
extern void* g_4846b0;
extern char g_4847e8;

void* name_next_elt(unsigned int a0)
{
LABEL_4267d0:
    if (!g_4846b0)
    {
        sub_426210();
        return g_4846b0;
    }
    else if (!((int)g_4846b0[16] <= 4))
    {
        goto LABEL_4267d0;
    }
    goto *((void *)((long long)(long long)(g_463688[(int)g_4846b0[16]] + &g_463688[0])));

    switch (/* incomplete */)
    {
    case 0x426817:
LABEL_426817:
        sub_425a50();
        goto LABEL_4267d0;
    case 0x426860:
        if (!(unsigned int)(unsigned long long)sub_426500(g_4846b0))
            return &g_484660;
        goto LABEL_4267d0;
    case 0x426820:
LABEL_426820:
        sub_426770((long long)g_4846b0[24]);
        if (g_4847e8)
            sub_423c70(g_4846a8);
        g_484670 = (unsigned int)(int)g_4846b0[16];
        g_484678 = g_4846a8;
        sub_425a50();
        return &g_484660;
    case 0x426880:
        if (!a0)
            goto LABEL_426820;
        sub_424c80((unsigned int)sub_424b00((unsigned long long)sub_44c7a0((long long)g_4846b0[24])));
        sub_425a50();
        goto LABEL_4267d0;
    case 0x426808:
        sub_425f80((int)g_4846b0[24], (long long)g_4846b0[32]);
        goto LABEL_426817;
    }
}



// Function: uid_to_uname @ 0x268d0
typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

extern long long g_467708;
extern unsigned int g_4846cc;
extern unsigned int g_4846e4;
extern unsigned long long g_4846f0;

void uid_to_uname(unsigned int a0, unsigned long long *a1)
{
    passwd *v1;  // rax
    unsigned long v2;  // rsi

    if (a0 && g_4846cc == a0)
    {
        *(a1) = sub_44c7a0(&g_467708);
        return;
    }
    if (!g_4846f0 || g_4846e4 != a0)
    {
        v1 = getpwuid(a0);
        if (!v1)
        {
            g_4846cc = a0;
            *(a1) = sub_44c7a0(&g_467708);
            return;
        }
        v2 = v1->pw_name;
        g_4846e4 = a0;
        sub_423be0(&g_4846f0, v2);
    }
    *(a1) = sub_44c7a0();
    return;
}



// Function: gid_to_gname @ 0x26960
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

extern long long g_467708;
extern unsigned int g_4846c8;
extern unsigned int g_4846e0;
extern unsigned long long g_4846e8;

void gid_to_gname(unsigned int a0, unsigned long long *a1)
{
    group *v1;  // rax
    unsigned long v2;  // rsi

    if (a0 && g_4846c8 == a0)
    {
        *(a1) = sub_44c7a0(&g_467708);
        return;
    }
    if (!g_4846e8 || g_4846e0 != a0)
    {
        v1 = getgrgid(a0);
        if (!v1)
        {
            g_4846c8 = a0;
            *(a1) = sub_44c7a0(&g_467708);
            return;
        }
        v2 = v1->gr_name;
        g_4846e0 = a0;
        sub_423be0(&g_4846e8, v2);
    }
    *(a1) = sub_44c7a0();
    return;
}



// Function: uname_to_uid @ 0x269f0
typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

extern char *g_4846d8;
extern unsigned int g_4846e4;
extern char *g_4846f0;

unsigned int uname_to_uid(char *a0, unsigned int *a1)
{
    unsigned int v1;  // eax
    passwd *v2;  // rax
    unsigned long v3;  // rsi

    if (g_4846d8)
    {
        v1 = strcmp(a0, g_4846d8);
        if (!v1)
            return v1;
    }
    if (!g_4846f0 || *(a0) != *(g_4846f0))
    {
        v2 = getpwnam(a0);
        if (!v2)
        {
            sub_423be0(&g_4846d8, a0);
            return 0;
        }
    }
    else if (!strcmp(a0, g_4846f0))
    {
        *(a1) = g_4846e4;
        return 1;
    }
    else
    {
        v2 = getpwnam(a0);
        if (!v2)
        {
            sub_423be0(&g_4846d8, a0);
            return 0;
        }
    }
    v3 = v2->pw_name;
    g_4846e4 = v2->pw_uid;
    sub_423be0(&g_4846f0, v3);
    *(a1) = g_4846e4;
    return 1;
}



// Function: gname_to_gid @ 0x26aa0
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

extern char *g_4846d0;
extern unsigned int g_4846e0;
extern char *g_4846e8;

unsigned int gname_to_gid(char *a0, unsigned int *a1)
{
    unsigned int v1;  // eax
    group *v2;  // rax

    if (g_4846d0)
    {
        v1 = strcmp(a0, g_4846d0);
        if (!v1)
            return v1;
    }
    if (!g_4846e8 || *(a0) != *(g_4846e8))
    {
        v2 = getgrnam(a0);
        if (!v2)
        {
            sub_423be0(&g_4846d0, a0);
            return 0;
        }
    }
    else if (!strcmp(a0, g_4846e8))
    {
        *(a1) = g_4846e0;
        return 1;
    }
    else
    {
        v2 = getgrnam(a0);
        if (!v2)
        {
            sub_423be0(&g_4846d0, a0);
            return 0;
        }
    }
    g_4846e0 = v2->gr_gid;
    sub_423be0(&g_4846e8, a0);
    *(a1) = g_4846e0;
    return 1;
}



// Function: name_add_name @ 0x26b50
typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
} struct_0;

extern unsigned int g_484630;

long long name_add_name(unsigned long a0)
{
    struct_0 *idx;  // rax
    unsigned long v2;  // rax

    idx = (unsigned long long)sub_425880();
    idx->field_10 = 0;
    idx->field_18 = a0;
    v2 = g_484630;
    if (!(unsigned int)v2)
    {
        g_484630 = 1;
        return v2;
    }
    if ((unsigned int)v2 != 1)
        return v2;
    g_484630 = 2;
    return v2;
}



// Function: names_parse_opt @ 0x26ba0
typedef struct struct_0 {
    unsigned int field_0;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

unsigned int names_parse_opt(unsigned int a0, unsigned int a1, struct_1 *a2)
{
    unsigned int v2;  // rdi
    unsigned int v3;  // rsi

    switch (a0)
    {
    case 256:
        sub_426b50(a1);
        return 0;
    case 16777221:
        if (a2->field_28->field_0->field_0 != 2)
            return 7;
        error(0, 0, dcgettext(NULL, "%s:%lu: unrecognized option", 5));
        sub_42f7d0(2);
        return 7;
    case 67:
        sub_4258f0(a1);
        return 0;
    case 84:
        sub_425910(a1);
        return 0;
    default:
        if (!sub_425620(a0))
            return 7;
        sub_425940(v2, v3);
        return 0;
    }
}



// Function: name_init @ 0x26c90
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

extern unsigned long long g_4846a0;
extern unsigned long long g_4846a8;

struct_0 * name_init(void)
{
    unsigned long v1;  // rax

    v1 = sub_44c5b0(0x66);
    g_4846a0 = 100;
    g_4846a8 = v1;
    return sub_425630();
}



// Function: name_term @ 0x26cc0
extern void* g_4846a8;

void name_term(void)
{
    free(g_4846a8);
    return;
}



// Function: name_next @ 0x26cd0
typedef struct struct_0 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_0;

struct_0 * name_next(unsigned int a0)
{
    struct_0 *v1;  // rax

    v1 = sub_4267d0(a0);
    if (!v1)
        return v1;
    return v1->field_18;
}



// Function: addname @ 0x26cf0
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
    char field_18;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern unsigned int g_481238;
extern struct_1 *g_4846b8;
extern unsigned long long g_4846c0;
extern unsigned int g_4846f8;
extern unsigned int g_4846fc;
extern unsigned int g_484ad4;

struct_0 * addname(unsigned long long a0, unsigned int a1, char a2, unsigned long a3)
{
    struct_0 *idx;  // rax
    unsigned int v2;  // edx
    unsigned long v3;  // 4215
    struct_0 **v4;  // rsi

    idx = sub_4263a0(a0);
    v2 = g_481238 | g_4846f8 | g_484ad4;
    v3 = g_4846fc;
    *((unsigned long *)&idx->padding_0[0]) = 0;
    v4 = g_4846b8;
    *((unsigned int *)&idx[1].padding_0[15]) = a1;
    *((unsigned long *)((char *)&idx[1].field_10 + 7)) = 0;
    *((struct_0 ***)&idx->padding_0[8]) = v4;
    *((unsigned int *)&idx[1].padding_0[7]) = v2 | ((unsigned int)v3 == 2) * 0x10000000;
    *((unsigned long *)&idx[2].padding_0[6]) = 0;
    *((unsigned long *)&idx[2].padding_0[14]) = a3;
    idx[1].padding_0[11] = a2;
    if (v4)
    {
        *(v4) = idx;
        g_4846b8 = idx;
    }
    else
    {
        g_4846c0 = idx;
        g_4846b8 = idx;
    }
    return idx;
}



// Function: name_gather @ 0x26d90
extern unsigned int g_481238;
extern void* *g_484638;
extern unsigned int g_484640;
extern unsigned long long g_4846b8;
extern unsigned long long g_4846c0;
extern unsigned int g_4846f8;
extern unsigned int g_4846fc;
extern char g_484aac;
extern unsigned int g_484ad4;

void* name_gather(void)
{
    unsigned int v2;  // ebx
    unsigned int v3;  // ebp
    void* v4;  // rax
    unsigned int v5;  // esi
    void* v6;  // rax
    void* idx;  // rax
    unsigned int v8;  // edx
    unsigned long v9;  // 4222
    void* v0;  // [bp-0x20]

    v2 = 0;
    if (!g_484aac)
    {
        while (1)
        {
            v3 = v2;
            while (1)
            {
                v4 = sub_4267d0(0);
                if (!v4)
                {
                    if (v2 == v3)
                        return v4;
                    return sub_426cf0(0, v3, 0, 0);
                }
                if ((int)v4[16] != 1)
                    break;
                v3 = sub_424b00(sub_44c7a0((long long)v4[24]));
            }
            sub_426cf0((long long)v4[24], v3, 1, 0);
            v2 = v3;
        }
    }
    else
    {
        while (1)
        {
            v6 = sub_4267d0(0);
            if (!v6)
                break;
            if ((int)v6[16] != 1)
            {
                v0 = v6;
                sub_425ad0(g_484638);
                idx = sub_4263a0((long long)v0[24]);
                *((unsigned long long *)idx) = 0;
                *((unsigned int *)&idx[40]) = g_484640;
                v8 = g_481238 | g_4846f8 | g_484ad4;
                *((unsigned long long *)&idx[48]) = 0;
                v9 = g_4846fc;
                *((unsigned long long *)&idx[56]) = 0;
                *((unsigned long long *)&idx[64]) = 0;
                *((char *)&idx[36]) = 1;
                g_484638 = idx;
                *((unsigned int *)&idx[32]) = v8 | ((unsigned int)v9 == 2) * 0x10000000;
                g_4846b8 = idx;
                g_4846c0 = idx;
                return idx;
            }
            g_484640 = sub_424b00(sub_44c7a0((long long)v6[24]));
        }
        v5 = g_484640;
        if (g_484640)
        {
            return sub_426cf0(0, v5, 0, 0);
        }
        return v6;
    }
}



// Function: add_hierarchy_to_namelist @ 0x26ee0
typedef struct struct_0 {
    char padding_0[424];
    int field_1a8;
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

typedef struct struct_2 {
    char padding_0[16];
    char *field_10;
    unsigned long long field_18;
    char padding_20[8];
    unsigned int field_28;
    char padding_2c[12];
    void* field_38;
    char padding_40[8];
    struct struct_1 *field_48;
} struct_2;

typedef struct struct_1 {
    char padding_0[80];
    struct struct_1 *field_50;
} struct_1;

extern unsigned int g_484808;

void add_hierarchy_to_namelist(struct_0 *a0, struct_2 *idx)
{
    void* v10;  // rax
    void* v11;  // rax
    struct_1 *v20;  // r15
    int v21;  // eax
    unsigned long v22;  // rdi
    unsigned long long v12;  // r15
    void* ptr;  // rbx
    unsigned long v14;  // rdi
    unsigned long ptr1;  // r13
    char i;  // r14b
    unsigned long len;  // rbp
    unsigned long v18;  // r14
    void* v19;  // r14
    unsigned long long v1;  // [bp-0x238]
    struct_1 *v2;  // [bp-0x230]
    struct_1 *v3;  // [bp-0x230]
    char *v4;  // [bp-0x220]
    struct_1 *v5;  // [bp-0x218], Other Possible Types: int
    unsigned int v6;  // [bp-0x20c]
    unsigned long j;  // [bp-0x200]
    stat v8;  // [bp-0x1f8], Other Possible Types: struct_1

    v10 = sub_41f610(a0);
    idx->field_38 = v10;
    v11 = sub_41fa60(v10);
    if (!v11)
        return;
    v12 = idx->field_18;
    ptr = v11;
    v14 = v12 + 0x66;
    v1 = v12;
    if (v12 <= 99)
        v14 = 0x66;
    j = v14;
    ptr1 = sub_44c5b0();
    v6 = idx->field_28;
    strcpy(ptr1, idx->field_10);
    if (*((char *)(ptr1 + v12 - 1)) != 47)
    {
        *((char *)(ptr1 + v12)) = 47;
        *((char *)(ptr1 + v12 + 1)) = 0;
        v1 = v12 + 1;
    }
    v2 = NULL;
    i = *((char *)ptr);
    if (i)
    {
        v5 = NULL;
        v4 = &v8;
        do
        {
            len = strlen(ptr);
            if (i != 0x44)
                continue;
            v18 = v1 + len;
            if (j < v18)
            {
                do
                {
                    ptr1 = sub_44c710(ptr1, &j);
                } while (j < v18);
            }
            v19 = ptr + 1;
            strcpy(ptr1 + v1, v19);
            v20 = sub_426cf0(ptr1, v6, 0, idx);
            if (v2)
            {
                v5->field_50 = v3;
            }
            else
            {
                v3 = v20;
                v2 = v3;
            }
            sub_42f630(v4);
            *((struct_0 **)&(&v8)[4].padding_0[56]) = a0;
            v5 = a0->field_1a8;
            if (a0->field_1a8 < 0)
            {
                *(__errno_location()) = -(v5);
                goto LABEL_4270a5;
            }
            else
            {
                v21 = sub_4140d0(a0, v19, g_484808 | 0x10000);
                if (v21 < 0)
                {
LABEL_4270a5:
                    sub_424dd0(ptr1);
                    goto LABEL_4270ad;
                }
                else
                {
                    *((int *)&(&v8)[4].padding_0[72]) = v21;
                    v22 = ptr1;
                    if (fstat(v21, (char *)&v8 - 88))
                    {
                        sub_424ec0(v22);
                    }
                    else
                    {
                        *((unsigned long long *)&v8.padding_0[0]) = sub_44c7a0(v22);
                        add_hierarchy_to_namelist(v4, v3);
                        sub_414200(v4);
                    }
LABEL_4270ad:
                    sub_42f6f0(v4);
                    v5 = v3;
                }
            }
            ptr = ptr + len + 1;
            i = *((char *)ptr);
        } while (i);
    }
    free(ptr1);
    idx->field_48 = v2;
    return;
}



// Function: remname @ 0x27150
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_0;

typedef struct struct_4 {
    struct struct_0 *field_0;
    struct struct_1 *field_8;
} struct_4;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned long long g_4846b8;
extern unsigned long long g_4846c0;

struct_0 ** remname(struct_4 *a0)
{
    struct_0 **v1;  // rax
    struct_0 *v2;  // rdx

    v1 = a0->field_8;
    v2 = a0->field_0;
    if (v1)
    {
        *(v1) = v2;
        if (!v2)
        {
            g_4846b8 = v1;
            return v1;
        }
    }
    else
    {
        g_4846c0 = v2;
        if (!v2)
        {
            g_4846b8 = v1;
            return v1;
        }
    }
    v2->field_8 = v1;
    return v1;
}



// Function: add_starting_file @ 0x27190
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_0;

typedef struct struct_4 {
    struct struct_0 *field_0;
    struct struct_1 *field_8;
} struct_4;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned int g_481238;
extern unsigned long long g_4846b8;
extern struct_4 *g_4846c0;
extern unsigned int g_4846f8;
extern unsigned int g_4846fc;
extern char g_484a78;
extern unsigned int g_484ad4;

long long add_starting_file(unsigned long long a0)
{
    struct_0 *idx;  // rbx
    void* *v2;  // rdi
    struct_4 *v3;  // rax
    unsigned long long v4;  // 4099
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    unsigned long v7;  // 4203

    idx = sub_4263a0(a0);
    if (g_484a78)
    {
        sub_427150(g_4846c0);
        sub_425ad0(v2);
    }
    v3 = g_4846c0;
    v4 = g_4846b8;
    *((unsigned long *)&idx->padding_0[8]) = 0;
    g_4846c0 = idx;
    *((struct_4 **)&idx->padding_0[0]) = v3;
    if (!v4)
        g_4846b8 = idx;
    v5 = g_481238 | g_4846f8;
    *((unsigned long *)((char *)&idx[1].field_10 + 7)) = 0;
    v6 = v5 | g_484ad4;
    v7 = g_4846fc;
    *((unsigned int *)&idx[1].padding_0[15]) = 0;
    *((unsigned long *)&idx[2].padding_0[6]) = 0;
    *((unsigned long *)&idx[2].padding_0[14]) = 0;
    idx[1].padding_0[11] = 1;
    *((unsigned int *)&idx[1].padding_0[7]) = v6 | ((unsigned int)v7 == 2) * 0x10000000;
    g_484a78 = 1;
    return v6 | ((unsigned int)v7 == 2) * 0x10000000;
}



// Function: name_match @ 0x27250
typedef struct struct_0 {
    char padding_0[16];
    struct struct_1 *field_10;
    unsigned long long field_18;
    char padding_20[8];
    unsigned int field_28;
    char padding_2c[4];
    unsigned long long field_30;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_4846b8;
extern struct_0 *g_4846c0;
extern char g_484a78;
extern char g_484aac;
extern unsigned int g_484ad4;
extern char g_484b38;

unsigned int name_match(long long a0)
{
    struct_0 *v2;  // rax
    struct_0 *idx;  // rax
    struct_0 *v0;  // [bp-0x10]

    v2 = g_4846c0;
    while (1)
    {
        if (!v2)
            return 1;
        if (!v2->field_10->field_0)
        {
            sub_424c80(v2->field_28);
            g_4846c0 = 0;
            g_4846b8 = 0;
            return 1;
        }
        idx = sub_4264a0(a0);
        if (g_484a78)
        {
            if (g_4846c0 == idx && !(g_484a78 = 0, !idx))
                goto LABEL_4272d7;
        }
        else
        {
            if (idx)
            {
LABEL_4272d7:
                if (*((char *)(a0 + idx->field_18)) != 47 || !g_484ad4 || !idx->field_30)
                {
                    idx->field_30 = idx->field_30 + 1;
                    break;
                }
            }
        }
        if (!g_484aac || !g_4846c0[6].field_0 || !(sub_426d90(), v2 = (struct_0 *)g_4846c0, !g_4846c0[6].field_0))
            return 0;
    }
    v0 = idx;
    sub_424c80(idx->field_28);
    return (!*((long long *)&g_484b38) ? _INSERT(v0, 0, v0->field_30) : _INSERT(v0, 0, v0->field_30 == *((long long *)&g_484b38)));
}



// Function: all_names_found @ 0x273a0
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
    char field_10;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[8];
    struct struct_2 *field_10;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern struct_1 *g_4846c0;
extern unsigned long long g_484b38;

unsigned int all_names_found(struct_0 *a0)
{
    char *v1;  // rbx
    unsigned int v2;  // r12d
    unsigned long len;  // rax
    struct_1 *v4;  // rdx
    struct_1 *v5;  // rdx

    v1 = a0->field_8;
    if (!v1)
    {
        return 0;
    }
    else if (g_484b38)
    {
        v2 = a0->field_10;
        if ((char)v2)
            return 0;
        len = strlen(v1);
        v4 = g_4846c0;
        if (!g_4846c0)
            return 1;
        do
        {
            v5 = v4;
            if (v5->field_10->field_0 && g_484b38 > v5[2].field_0)
                return v2;
            if (v5[1].field_0 <= len && *((char *)(v1 + v5[1].field_0)) == 47)
                return v2;
            v4 = v5->field_0;
        } while (v5->field_0);
        return 1;
    }
    else
    {
        return 0;
    }
}



// Function: collect_and_sort_names @ 0x276a0
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_0;

typedef struct struct_4 {
    struct struct_0 *field_0;
    struct struct_1 *field_8;
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
    struct struct_0 *field_0;
} struct_1;

typedef struct struct_2 {
    struct struct_2 *field_0;
    char padding_8[8];
    struct struct_1 *field_10;
    unsigned long long field_18;
    char padding_20[3];
    char field_23;
    char padding_24[4];
    unsigned int field_28;
    char padding_2c[4];
    unsigned long long field_30;
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
    char padding_50[8];
    unsigned long long field_58;
} struct_2;

extern unsigned int g_481234;
extern unsigned long long g_4846b8;
extern struct_0 *g_4846c0;
extern unsigned int g_484808;
extern unsigned long long g_484b28;
extern struct_4 *g_484d50;

void collect_and_sort_names(void)
{
    struct_0 *v4;  // rbp
    unsigned int v5;  // eax
    struct_0 *v14;  // rbx
    long long v15;  // r14
    unsigned int iter;  // r13d
    struct_2 *v17;  // r15
    struct_0 *index;  // rbp
    struct_4 *v19;  // rax
    void* *v20;  // rdi
    char *v6;  // rsi
    struct_0 *i;  // rax
    unsigned long long v22;  // 4098
    unsigned int node;  // r12d
    struct_0 *idx;  // rbp
    unsigned int v10;  // eax
    struct_0 v12;  // rax
    long long v13;  // rax
    struct_0 v0;  // [bp-0x1f8]
    int v1;  // [bp-0x1a0], Other Possible Types: stat
    unsigned int v2;  // [bp-0x50]

    sub_426d90();
    v4 = g_4846c0;
    if (!g_4846c0)
    {
        sub_426cf0(".", 0, 0, 0);
        if (!g_484b28)
            goto LABEL_427710;
    }
    else if (!g_484b28)
    {
        goto LABEL_427720;
    }
    v5 = sub_424ae0();
    if (v5)
    {
        if (v5 == 1)
        {
            if ((int)g_4846c0[5].field_0)
                goto LABEL_42770b;
            if (g_484d50)
                g_484d50();
            v6 = "Using -C option inside file list is not allowed with --listed-incremental";
        }
        else
        {
            if (g_484d50)
                g_484d50();
            v6 = "Only one -C option is allowed with --listed-incremental";
        }
        error(0, 0, dcgettext(NULL, v6, 5));
        sub_42c970(2); /* do not return */
    }
LABEL_42770b:
    sub_41fd80();
LABEL_427710:
    v4 = g_4846c0;
    if (!g_4846c0)
    {
        node = 0;
LABEL_4277b7:
        g_4846c0 = sub_425800(v4, node, sub_4259f0);
        v13 = sub_441b30(0, 0, sub_426490, sub_4259d0, 0);
        v14 = g_4846c0;
        v15 = v13;
        if (g_4846c0)
        {
            iter = 0;
            v17 = NULL;
            while (1)
            {
                index = v14;
                v14 = v14->field_0;
                index->field_58 = sub_425340(index->field_28, index->field_10);
                if (v17 && (v19 = (struct_4 *)(unsigned long long)sub_4418d0(v15, index), v19))
                {
                    if (index->field_40)
                    {
                        if (index->field_48)
                            sub_4266c0(index->field_48, v19[1].field_0, v19[1].field_8);
                        sub_427150(index);
                        sub_425ad0(v20);
                        if (!v14)
                            break;
                        else
                            continue;
                    }
                    else
                    {
                        if (v19[4].field_8)
                            sub_4266c0(v19[4].field_8, index->field_10, index->field_18);
                        iter -= 1;
                        sub_4421a0(v15, index);
                        sub_427150(v19);
                        sub_425ad0(v20);
                    }
                }
                index->field_30 = 0;
                if (!sub_442140(v15, index))
                    sub_416420(); /* do not return */
                iter += 1;
                v17 = index;
                if (!v14)
                    break;
            }
        }
        else
        {
            v17 = NULL;
            iter = 0;
        }
        g_4846b8 = v17;
        sub_441ce0(v15);
        i = sub_425800(g_4846c0, iter, sub_425970);
        v22 = g_484b28;
        if (!v22)
            return;
        for (g_4846c0 = i; i; i = i->field_0)
        {
        }
        return;
    }
LABEL_427720:
    node = 0;
    while (1)
    {
        idx = v4;
        if (!idx->field_30 && !idx->field_38 && !(idx->field_23 & 16) && (sub_424c80(idx->field_28), idx->field_10->field_0))
        {
            sub_42f630(&v0);
            if (!(unsigned int)(unsigned long long)sub_4246f0(idx->field_10, &v1))
            {
                if (((unsigned short)*((unsigned int *)(&v1 + 24)) & 0xf000) == 0x4000)
                {
                    v10 = __openat_2(g_481234, idx->field_10, g_484808 | 0x10000);
                    if (v10 >= 0)
                    {
                        v2 = v10;
                        if (fstat(v10, &v1))
                        {
                            sub_424ec0(idx->field_10);
                        }
                        else
                        {
                            v12 = (struct_0)sub_44c7a0(idx->field_10);
                            idx->field_30 = idx->field_30 + 1;
                            v0 = v12;
                            sub_426ee0(&v0, idx);
                        }
                    }
                    else
                    {
                        sub_424dd0(idx->field_10);
                    }
                }
                node += 1;
                sub_42f6f0(&v0);
                v4 = idx->field_0;
                if (!idx->field_0)
                    break;
                else
                    continue;
            }
            else
            {
                sub_424ec0(idx->field_10);
            }
        }
        v4 = idx->field_0;
        node += 1;
        if (!v4)
            break;
    }
    v4 = g_4846c0;
    goto LABEL_4277b7;
}



// Function: name_scan @ 0x27aa0
typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

extern struct_0 *g_4846c0;
extern char g_484aac;

unsigned long long name_scan(long long a0)
{
    unsigned long long v1;  // rax

    while (1)
    {
        v1 = sub_4264a0(a0);
        if (v1)
        {
            return v1;
        }
        else if (!g_484aac)
        {
            return v1;
        }
        else if (!g_4846c0)
        {
            return v1;
        }
        else if (g_4846c0[6].field_0)
        {
            sub_426d90();
            if (g_4846c0[6].field_0)
                return v1;
        }
        else
        {
            return v1;
        }
    }
}



// Function: name_from_list @ 0x27b00
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    struct struct_1 *field_10;
    char padding_18[16];
    unsigned int field_28;
    char padding_2c[4];
    unsigned long long field_30;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern struct_0 *g_484620;
extern struct_0 *g_4846c0;

struct_0 * name_from_list(void)
{
    struct_0 *v1;  // rax

    v1 = g_484620;
    if (!v1)
    {
        v1 = g_4846c0;
        g_484620 = g_4846c0;
        if (!g_4846c0)
            return NULL;
    }
    while (1)
    {
        if (!v1->field_30 && v1->field_10->field_0)
        {
            v1->field_30 = 1;
            sub_424c80(v1->field_28);
            return g_484620;
        }
        v1 = v1->field_0;
        g_484620 = v1;
        if (!v1)
            return NULL;
    }
}



// Function: blank_name_list @ 0x27b80
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[40];
    unsigned long long field_30;
} struct_0;

extern unsigned long long g_484620;
extern struct_0 *g_4846c0;

struct_0 * blank_name_list(void)
{
    struct_0 *i;  // rax

    i = g_4846c0;
    g_484620 = 0;
    if (g_4846c0)
    {
        do
        {
            i->field_30 = 0;
            i = i->field_0;
        } while (i);
    }
    return i;
}



// Function: make_file_name @ 0x27bc0
void* make_file_name(void* ptr, void* a1)
{
    unsigned long v1;  // rbp
    unsigned long len;  // rbx
    unsigned long n;  // r15
    void* v4;  // r12

    v1 = 0;
    len = strlen(ptr);
    n = strlen(a1) + 1;
    if (len)
    {
        v1 = len;
        if (*((char *)ptr + v1 - 1) != 47)
            v1 = len + 1;
    }
    v4 = sub_44c5b0(n + v1);
    memcpy(v4, ptr, len);
    *((char *)v4 + len) = 47;
    memcpy(v4 + v1, a1, n);
    return v4;
}



// Function: stripped_prefix_len @ 0x27c50
char * stripped_prefix_len(char *a0, unsigned long long a1)
{
    char i;  // dl
    char *iter;  // rax
    unsigned long long v3;  // rsi

    i = *(a0);
    iter = a0;
    if (*(a0) == 47)
    {
        do
        {
            i = iter[1];
            iter += 1;
        } while (i == 47);
        if (!i)
            return 0xffffffffffffffff;
    }
    while (1)
    {
        iter += 1;
        if (i != 47)
        {
            i = *(iter);
        }
        else
        {
            v3 = a1 - 1;
            if (a1 == 1)
                return iter - a0;
            while (1)
            {
                i = *(iter);
                a1 = v3;
                if (*(iter) != 47)
                    break;
                iter += 1;
            }
        }
        if (!i)
            return 0xffffffffffffffff;
    }
}



// Function: contains_dot_dot @ 0x27cc0
int contains_dot_dot(unsigned long a0)
{
    char *v1;  // rdi
    unsigned long long v2;  // rdx
    unsigned long long v4;  // cc_dep1, Other Possible Types: unsigned long
    unsigned long long v5;  // cc_dep2
    char v6;  // al
    unsigned long long v7;  // cc_op
    unsigned long long v8;  // cc_op
    unsigned long v9;  // cc_dep1
    unsigned long long v10;  // cc_dep2
    unsigned long long v11;  // cc_op
    unsigned long long v12;  // cc_dep1
    unsigned long long v13;  // cc_dep2
    unsigned long long v14;  // rax
    unsigned int v15;  // eax
    unsigned long long v3;  // rcx

    while (1)
    {
        tmp_0 = reg72<64>;
        tmp_6 = *((char *)tmp_0);
        v2 = tmp_6;
        tmp_3 = v2;
        v7 = 5;
        v4 = tmp_3;
        v5 = 46;
        if (tmp_3 == 46)
        {
            tmp_5 = reg72<64>;
            tmp_2 = *((char *)(tmp_5 + 1));
            v7 = 5;
            v4 = tmp_2;
            v5 = 46;
            if (tmp_2 == 46)
            {
                tmp_13 = reg72<64>;
                tmp_16 = *((char *)(tmp_13 + 2));
                v3 = tmp_16;
                tmp_3 = v3;
                *((char *)&v3) = !tmp_3;
                v7 = 0x11;
                v4 = tmp_3 == 47 | !tmp_3;
                v5 = 0;
                v6 = tmp_3 == 47 | !tmp_3;
                if (tmp_3 == 47 || !tmp_3)
                    break;
            }
        }
        while (1)
        {
            tmp_2 = v1;
            v1 = tmp_2 + 1;
            tmp_5 = v2;
            v8 = 5;
            v9 = tmp_5;
            v10 = 47;
            if (tmp_5 == 47)
                break;
            tmp_2 = v2;
            v7 = 0x11;
            v4 = tmp_2;
            v5 = 0;
            if (!tmp_2)
            {
                v11 = 19;
                v12 = 0;
                v13 = 0;
                v14 = 0;
                tmp_3 = reg48<64>;
                reg48<64> = tmp_3 + 8;
                return v14;
            }
            tmp_0 = v1;
            tmp_3 = *((char *)tmp_0);
            v2 = tmp_3;
        }
    }
    tmp_0 = reg48<64>;
    reg48<64> = tmp_0 + 8;
    return v15;
}



// Function: tar_sparse_member_p @ 0x27d10
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

unsigned long long tar_sparse_member_p(struct_1 *a0)
{
    if (!a0->field_20->field_10)
        return 0;
    goto *((void *)(a0->field_20->field_10));
}



// Function: tar_sparse_done @ 0x27d30
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

unsigned long long tar_sparse_done(struct_1 *a0)
{
    if (!a0->field_20->field_8)
        return 1;
    goto *((void *)(a0->field_20->field_8));
}



// Function: tar_sparse_scan @ 0x27d50
typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
} struct_0;

unsigned long long tar_sparse_scan(struct_1 *a0)
{
    if (!a0->field_20->field_30)
        return 1;
    goto *((void *)(a0->field_20->field_30));
}



// Function: tar_sparse_dump_region @ 0x27d70
typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[56];
    unsigned long long field_38;
} struct_0;

unsigned long long tar_sparse_dump_region(struct_1 *a0)
{
    if (!a0->field_20->field_38)
        return 0;
    goto *((void *)(a0->field_20->field_38));
}



// Function: tar_sparse_extract_region @ 0x27d90
typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[64];
    unsigned long long field_40;
} struct_0;

unsigned long long tar_sparse_extract_region(struct_1 *a0)
{
    if (!a0->field_20->field_40)
        return 0;
    goto *((void *)(a0->field_20->field_40));
}



// Function: tar_sparse_decode_header @ 0x27db0
typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
} struct_0;

unsigned long long tar_sparse_decode_header(struct_1 *a0)
{
    if (!a0->field_20->field_28)
        return 1;
    goto *((void *)(a0->field_20->field_28));
}



// Function: tar_sparse_fixup_header @ 0x27dd0
typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

unsigned long long tar_sparse_fixup_header(struct_1 *a0)
{
    if (!a0->field_20->field_20)
        return 1;
    goto *((void *)(a0->field_20->field_20));
}



// Function: zero_block_p @ 0x27df0
unsigned int zero_block_p(void* a0, unsigned long a1)
{
    void* i;  // rdi

    i = a0;
    while (i != a1 + a0)
    {
        i += 1;
        if (*((char *)i - 1))
            return 0;
    }
    return 1;
}



// Function: sparse_add_map @ 0x27e20
typedef struct struct_0 {
    char padding_0[304];
    unsigned long long field_130;
    char padding_138[8];
    long long field_140;
} struct_0;

unsigned long long sparse_add_map(struct_0 *idx, uint128_t *a1)
{
    long long v1;  // rdi
    unsigned long long index;  // rbp

    v1 = idx->field_140;
    index = idx->field_130;
    if (idx->padding_138 == index)
    {
        v1 = sub_44c680(v1, idx->padding_138, 16);
        idx->field_140 = v1;
    }
    *((uint128_t *)(v1 + index * 16)) = *(a1);
    idx->field_130 = index + 1;
    return index * 16;
}



// Function: sparse_select_optab @ 0x27e90
extern char g_47f780;
extern char g_47f7e0;
extern char g_47f840;
extern unsigned int g_484400;
extern unsigned int g_484ba4;

unsigned int sparse_select_optab(unsigned long idx)
{
    unsigned int v1;  // eax

    v1 = (!g_484400 ? g_484ba4 : g_484400);
    if (v1 == 5)
    {
        *((char **)(idx + 32)) = &g_47f7e0;
        return 1;
    }
    if (v1 <= 5)
    {
        if (v1 != 2)
        {
            if (v1 == 4)
            {
                *((char **)(idx + 32)) = &g_47f780;
                return 1;
            }
            return 0;
        }
    }
    else
    {
        if (v1 != 6)
            return 0;
    }
    *((char **)(idx + 32)) = &g_47f840;
    return 1;
}



// Function: tar_sparse_init @ 0x27f70
typedef struct struct_0 {
    char padding_0[32];
    struct struct_1 *field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned int tar_sparse_init(void* a0)
{
    unsigned int v1;  // eax
    struct_0 *v2;  // rdi

    memset(a0, 0, 48);
    v1 = sub_427e90(a0);
    if (!(char)v1)
    {
        return v1;
    }
    else if (v2->field_20->field_0)
    {
        goto *((void *)(v2->field_20->field_0));
    }
    else
    {
        return v1;
    }
}



// Function: dump_zeros @ 0x27fb0
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    long long field_8;
} struct_0;

extern void g_463760;

unsigned int dump_zeros(struct_0 *idx, long long a1)
{
    long long v1;  // rax
    unsigned long v2;  // rdx
    unsigned long count;  // rax

    v1 = idx->field_8;
    if (v1 > a1)
    {
        *(__errno_location()) = 22;
        return 0;
    }
    else if (v1 < a1)
    {
        while (1)
        {
            v2 = a1 - v1;
            if (0x200 < v2)
                v2 = 0x200;
            count = write(idx->field_0, &g_463760, v2);
            if (count <= 0)
                break;
            v1 = count + idx->field_8;
            idx->field_8 = v1;
            if (v1 >= a1)
                return 1;
        }
        if (count)
            return 0;
        *(__errno_location()) = 22;
        return 0;
    }
    else
    {
        return 1;
    }
}



// Function: lseek_or_error @ 0x28050
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    long long field_8;
} struct_0;

unsigned int lseek_or_error(struct_0 *a0, long long a1)
{
    if (a0->padding_4[0])
    {
        if (lseek(a0->field_0, a1, 0) >= 0)
            return 1;
        sub_424e90(*((long long *)a0[1].field_8), a1);
        return 0;
    }
    else
    {
        if ((char)sub_427fb0(a0, a1))
            return 1;
        sub_424e90(*((long long *)a0[1].field_8), a1);
        return 0;
    }
}



// Function: sparse_extract_region @ 0x280b0
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    long long field_8;
} struct_0;

extern struct_0 *g_484d50;
extern unsigned int g_484d58;

unsigned int sparse_extract_region(struct_0 *idx, unsigned long a1)
{
    unsigned long long v1;  // rsi
    unsigned int v2;  // eax
    unsigned long v3;  // r13
    unsigned long v4;  // r12
    unsigned long long v5;  // rax
    unsigned int v6;  // edi
    unsigned long v7;  // rbp

    v1 = a1 * 16;
    v2 = sub_428050(idx, *((long long *)(*((long long *)(idx[1].field_8 + 320)) + v1)));
    if (!(char)v2)
        return v2;
    v3 = *((long long *)(*((long long *)(idx[1].field_8 + 320)) + v1 + 8));
    if (v3)
    {
        while (1)
        {
            if (v3 <= 0)
                return v2;
            v4 = 0x200;
            if (v3 <= 0x200)
                v4 = v3;
            v5 = sub_40e130();
            if (!v5)
                break;
            sub_40d1f0(v5);
            v6 = idx->field_0;
            *((unsigned long long *)&idx[1].field_0) = *((long long *)&idx[1].field_0) + 0x200;
            v7 = sub_4249f0(v6, v5, v4);
            v3 -= v7;
            sub_40ccf0(*((long long *)(idx[1].field_8 + 280)) - *((long long *)&idx[1].field_0));
            idx->field_8 = idx->field_8 + v7;
            if (v4 != v7)
            {
                sub_432e30(*((long long *)idx[1].field_8), v7, v4);
                return 0;
            }
        }
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
        g_484d58 = 2;
        return 0;
    }
    else if (!idx->padding_4[0])
    {
        return v2;
    }
    else if (!sub_42aed0(idx->field_0))
    {
        return v2;
    }
    else
    {
        sub_432d90(*((long long *)idx[1].field_8));
        return idx->padding_4[0];
    }
}



// Function: star_fixup_header @ 0x28220
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
} struct_0;

extern unsigned long long g_484408;

unsigned long long star_fixup_header(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    unsigned long v3;  // rax
    void* idx;  // rdx
    unsigned long v5;  // rcx
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    v3 = sub_4219b0(g_484408 + 452, 12);
    idx = a0->field_18;
    *((long long *)&idx[280]) = (long long)idx[0x88];
    v5 = 0;
    if (v3 >= 0)
        v5 = v3;
    *((unsigned long *)&idx[0x88]) = v5;
    return ~(v3) >> 63;
}



// Function: oldgnu_fixup_header @ 0x28270
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
} struct_0;

extern unsigned long long g_484408;

unsigned long long oldgnu_fixup_header(struct_0 *a0)
{
    unsigned long long v2;  // rbx
    unsigned long v3;  // rax
    void* idx;  // rdx
    unsigned long v5;  // rcx
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    v3 = sub_4219b0(g_484408 + 483, 12);
    idx = a0->field_18;
    *((long long *)&idx[280]) = (long long)idx[0x88];
    v5 = 0;
    if (v3 >= 0)
        v5 = v3;
    *((unsigned long *)&idx[0x88]) = v5;
    return ~(v3) >> 63;
}



// Function: oldgnu_add_sparse @ 0x282c0
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

unsigned int oldgnu_add_sparse(struct_1 *a0, struct_0 *a1)
{
    struct_0 *v3;  // rdi
    uint128_t v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]

    if (!a1->field_c)
        return 1;
    v0 = sub_4219b0(a1, 12);
    v1 = sub_4219b0(&a1->field_c, 12);
    if ((v1 | v0) < 0)
    {
        return 2;
    }
    else if (__OFADD__(v0, v1) & 1)
    {
        return 2;
    }
    else
    {
        v3 = a0->field_18;
        if (*((long long *)&v3[10].padding_0[6]) < v0 + v1)
        {
            return 2;
        }
        else if (*((long long *)&v3[21].padding_0[7]) - 0 >> 63 & 1)
        {
            return 2;
        }
        else
        {
            sub_427e20(v3, &v0);
            return 0;
        }
    }
}



// Function: star_get_sparse_info @ 0x28370
typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[304];
    unsigned long long field_130;
    char padding_138[8];
    long long field_140;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern struct_0 *g_484408;
extern struct_2 *g_484d50;
extern unsigned int g_484d58;

int star_get_sparse_info(void)
{
    unsigned long v1;  // 4140
    struct_1 *idx;  // rdi
    unsigned long v3;  // rbx
    unsigned int v4;  // eax
    struct_1 *v5;  // rax
    struct_1 *v6;  // r13
    unsigned long long v7;  // rbx
    struct_1 *v8;  // rbp
    char v9;  // dl

    v1 = *((char *)&g_484408[43].field_0 + 1);
    idx->field_18->field_130 = 0;
    if (!(char)v1 && *((char *)&g_484408[45].field_0 + 6))
    {
        v3 = (char *)&g_484408[44].field_0 + 4;
        do
        {
            v4 = sub_4282c0(idx, v3);
            if (v4)
            {
                if (v4 != 2)
                    return;
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "%s: invalid sparse archive member", 5));
                g_484d58 = 2;
                return;
            }
        } while ((v3 = (unsigned long)(v3 + 24), g_484408 + 452 != v3));
        if (!*((char *)&g_484408[44].field_0 + 3))
            return;
        v5 = sub_40e130();
        v6 = v5;
        if (!v5)
            goto LABEL_428462;
        goto LABEL_4283c1;
    }
    do
    {
        v5 = sub_40e130();
        v6 = v5;
        if (!v5)
            goto LABEL_428462;
LABEL_4283c1:
        v7 = 0;
        sub_40d1f0(v5);
        v8 = v6;
        do
        {
            v7 += 1;
            v9 = !sub_4282c0(idx, v8);
            v8 = &v8->field_18;
        } while (v7 <= 20 && v9);
        *((unsigned long long *)&idx->padding_0[16]) = *((long long *)&idx->padding_0[16]) + 0x200;
    } while ((char)v6[15].field_18 && v9);
LABEL_428462:
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
    g_484d58 = 2;
    return;
}



// Function: oldgnu_get_sparse_info @ 0x28500
typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[304];
    unsigned long long field_130;
    char padding_138[8];
    long long field_140;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern struct_0 *g_484408;
extern struct_2 *g_484d50;
extern unsigned int g_484d58;

unsigned int oldgnu_get_sparse_info(struct_1 *a0)
{
    unsigned long v1;  // rbx
    unsigned long v2;  // rbp
    unsigned int v3;  // eax
    struct_1 *v4;  // r13
    unsigned long long v5;  // rbx
    struct_1 *v6;  // rbp
    char v7;  // dl

    v1 = (char *)&g_484408[48].field_0 + 2;
    v2 = (char *)&g_484408[60].field_0 + 2;
    a0->field_18->field_130 = 0;
    do
    {
        if (sub_4282c0(a0, v1))
            goto LABEL_4285b0;
    } while ((v1 = (unsigned long)(v1 + 24), v1 != v2));
    if (!*((char *)&g_484408[60].field_0 + 2))
        return 1;
    do
    {
        v4 = sub_40e130();
        if (!v4)
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
            g_484d58 = 2;
            return 0;
        }
        v5 = 0;
        sub_40d1f0(v4);
        v6 = v4;
        do
        {
            v5 += 1;
            v7 = !sub_4282c0(a0, v6);
            v6 = &v6->field_18;
        } while (v5 <= 20 && v7);
    } while ((char)v4[15].field_18 && v7);
LABEL_4285b0:
    if (v3 != 2)
        return 1;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "%s: invalid sparse archive member", 5));
    g_484d58 = 2;
    return 0;
}



// Function: sparse_dump_region @ 0x28660
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    long long field_8;
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

extern char g_483b80;
extern char g_484b52;
extern struct_0 *g_484d50;

int sparse_dump_region(struct_0 *idx, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5, unsigned long a6, unsigned long a7, unsigned long a8)
{
    long long *v4;  // rsi
    long long i;  // rbx
    long long v6;  // r12
    unsigned long v7;  // r15
    unsigned long long v8;  // r14
    unsigned long long *v9;  // r15
    unsigned long long *v11;  // r15
    long long v12;  // r12
    char v0;  // [bp-0xf1]
    unsigned long long v1;  // [bp-0xf0]
    stat v2;  // [bp-0xe8]
    char v3;  // [bp-0x58]

    v1 = a1 * 16;
    v4 = v1 + *((long long *)(idx[1].field_8 + 320));
    i = v4[1];
    v0 = sub_428050(idx, *(v4));
    if (v0 && i > 0)
    {
        do
        {
            v6 = 0x200;
            if (i <= 0x200)
                v6 = i;
            v7 = sub_40e130();
            v8 = sub_44af10(idx->field_0, v7, v6);
            if (v8 == 0xffffffffffffffff)
            {
                v9 = v1 + *((long long *)(idx[1].field_8 + 320));
                sub_424e00(*((long long *)idx[1].field_8), v9[1] + *(v9) - i, v6);
                v0 = 0;
                break;
            }
            else if (!v8)
            {
                if (fstat(idx->field_0, &v2))
                {
                    v11 = v1 + *((long long *)(idx[1].field_8 + 320));
                    v12 = i + *((long long *)(idx[1].field_8 + 0x88)) - (v11[1] + *(v11));
                }
                else
                {
                    v12 = *((long long *)(idx[1].field_8 + 0x88)) - v2.st_atime;
                }
                if (g_483b80 & 128)
                {
                    if (g_484d50)
                        g_484d50();
                    sub_443090(v12, &v3);
                    sub_44aa70(*((long long *)idx[1].field_8));
                    error(0, 0, dcngettext(NULL, "%s: File shrank by %s byte; padding with zeros", "%s: File shrank by %s bytes; padding with zeros", v12, 5));
                }
                if (!g_484b52)
                    sub_42f7d0(1);
                v0 = 0;
                break;
            }
            i -= v8;
            memset(v7 + v8, 0, 0x200 - v8);
            *((unsigned long long *)&idx[1].field_0) = *((long long *)&idx[1].field_0) + v8;
            sub_40d1f0(v7);
        } while (i > 0);
    }
    return v0;
}



// Function: decode_num @ 0x28870
unsigned int decode_num(unsigned long long *a0, char *a1, unsigned long long a2)
{
    unsigned int *err;  // rax
    unsigned long long v3;  // rax
    char v0;  // [bp-0x38]

    if (*(a1) - 48 <= 9)
    {
        err = __errno_location();
        *(err) = 0;
        v3 = strtoumax(a1, &v0, 10);
        if (v3 <= a2 && *(err) != 0x22 && !*((char *)v0))
        {
            *(a0) = v3;
            return 1;
        }
    }
    return 0;
}



// Function: pax_decode_header @ 0x28910
typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    struct struct_0 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[292];
    unsigned int field_124;
    char padding_128[8];
    unsigned long long field_130;
    unsigned long long field_138;
    unsigned long long field_140;
} struct_0;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern unsigned int g_484408;
extern struct_2 *g_484d50;
extern unsigned int g_484d58;

unsigned int pax_decode_header(struct_1 *idx, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5, unsigned long a6, unsigned long a7, unsigned long a8)
{
    char *v10;  // rax
    char *v11;  // r8
    unsigned long long v20;  // 4101
    char *v21;  // rcx
    void* iter;  // rdx
    char *v23;  // rbx
    char v24;  // al
    char *v25;  // r8
    char *v26;  // rcx
    void* node;  // rdx
    char v28;  // al
    char *v12;  // rbx
    long long v31;  // rax
    char *v13;  // rcx
    void* iter1;  // r13
    char *v15;  // rbx
    char v16;  // al
    unsigned int v17;  // r15d
    struct_0 *index;  // r14
    char *v19;  // r8
    long long v0;  // [bp-0xa0]
    void* v1;  // [bp-0x98], Other Possible Types: char *
    char *v2;  // [bp-0x90]
    unsigned long long v3;  // [bp-0x88]
    char *v4;  // [bp-0x80]
    unsigned long long v5;  // [bp-0x70]
    char v6;  // [bp-0x68], Other Possible Types: unsigned long long
    unsigned long long v7;  // [bp-0x60]
    char v8;  // [bp-0x58]
    char v9;  // [bp-0x44]

    if (!idx->field_18->field_124)
        return 1;
    v0 = sub_40d180();
    sub_40d1f0(*((long long *)&g_484408));
    v10 = sub_40e130();
    v11 = v10;
    if (v10)
    {
        v12 = v10;
        v13 = v10 + 0x200;
        iter1 = &v8;
        while (1)
        {
            if (v12 == v13)
            {
                sub_40d1f0(v11);
                v11 = sub_40e130();
                if (!v11)
                    goto LABEL_428ca8;
                v13 = v11 + 0x200;
                v12 = v11;
            }
            v15 = v12 + 1;
            v16 = *(v12);
            iter1 += 1;
            *((char *)iter1 - 1) = v16;
            if (v16 == 10)
                break;
            v12 = v15;
            if (iter1 == &v9)
                goto LABEL_428a00;
        }
        *((char *)iter1 - 1) = 0;
        v2 = v13;
        v1 = v11;
        v17 = sub_428870(&v5, &v8, 0xffffffffffffffff);
        if (!(char)v17)
        {
LABEL_428a87:
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "%s: malformed sparse archive member", 5));
            g_484d58 = 2;
            return 0;
        }
        index = idx->field_18;
        index->field_138 = v5;
        v19 = v1;
        index->field_140 = sub_44c720(v5, 16);
        v20 = idx->field_18->field_138;
        idx->field_18->field_130 = 0;
        if (!v20)
        {
            sub_40d1f0(v19);
            v31 = sub_40d180();
            idx->field_10 = idx->field_10 + (v31 - v0) * 0x200;
            return v17;
        }
        v21 = v2;
        v3 = 0;
        v4 = &v6;
        while (1)
        {
            iter = &v8;
            while (1)
            {
                if (v15 == v21)
                {
                    v1 = iter;
                    sub_40d1f0(v19);
                    v15 = sub_40e130();
                    iter = v1;
                    v19 = v15;
                    if (!v15)
                        goto LABEL_428ca8;
                    v21 = v15 + 0x200;
                }
                v23 = v15 + 1;
                v24 = *(v15);
                iter += 1;
                *((char *)iter - 1) = v24;
                if (v24 == 10)
                    break;
                v15 = v23;
                if (iter == &v9)
                    goto LABEL_428a00;
            }
            *((char *)iter - 1) = 0;
            v2 = v21;
            v1 = v19;
            v25 = v1;
            v26 = v2;
            if (!(char)sub_428870(&v5, &v8, 0x7fffffffffffffff))
                goto LABEL_428a87;
            node = &v8;
            v6 = v5;
            while (1)
            {
                if (v23 == v26)
                {
                    v1 = node;
                    sub_40d1f0(v25);
                    v23 = sub_40e130();
                    node = v1;
                    v25 = v23;
                    if (v23)
                    {
                        v26 = v23 + 0x200;
                        goto LABEL_428bb8;
                    }
                    goto LABEL_428ca8;
                }
LABEL_428bb8:
                v15 = v23 + 1;
                v28 = *(v23);
                node += 1;
                *((char *)node - 1) = v28;
                if (v28 == 10)
                    break;
                v23 = v15;
                if (node == &v9)
                {
LABEL_428a00:
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcgettext(NULL, "%s: numeric overflow in sparse archive member", 5));
                    g_484d58 = 2;
                    return 0;
                }
            }
            *((char *)node - 1) = 0;
            v2 = v26;
            v1 = v25;
            if (!(char)sub_428870(&v5, &v8, 0x7fffffffffffffff))
                goto LABEL_428a87;
            v7 = v5;
            sub_427e20(idx->field_18, v4);
            v3 += 1;
            v19 = v1;
            v21 = v2;
            if (idx->field_18->field_138 <= v3)
            {
                sub_40d1f0(v19);
                v31 = sub_40d180();
                idx->field_10 = idx->field_10 + (v31 - v0) * 0x200;
                return v17;
            }
        }
    }
LABEL_428ca8:
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
    sub_4163d0(); /* do not return */
}



// Function: pax_start_header @ 0x28ce0
void* pax_start_header(unsigned long long *idx)
{
    unsigned long long v1;  // rbp
    void* v2;  // rax

    v1 = idx[0x11];
    idx[0x11] = idx[35];
    v2 = sub_4132d0(idx);
    idx[0x11] = v1;
    return v2;
}



// Function: pax_dump_header @ 0x29390
typedef struct struct_1 {
    char padding_0[292];
    unsigned int field_124;
    unsigned int field_128;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

extern unsigned int g_484a80;
extern unsigned int g_484a84;

long long pax_dump_header(struct_0 *a0)
{
    unsigned int *idx;  // rdx

    idx = a0->field_18;
    idx[73] = g_484a84;
    idx[74] = g_484a80;
}



// Function: oldgnu_store_sparse_info @ 0x293d0
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[304];
    unsigned long long field_130;
    char padding_138[8];
    unsigned long long field_140;
} struct_1;

unsigned long long oldgnu_store_sparse_info(struct_0 *idx, unsigned long long *a1, char *a2, unsigned long long a3)
{
    char *v1;  // rbp
    unsigned long long *v2;  // rdx
    unsigned long long v3;  // rax
    unsigned long long v4;  // r13
    char *v5;  // rbp

    v1 = a2;
    v2 = idx->field_18;
    v3 = *(a1);
    if (v3 >= v2[38])
        return v3;
    v4 = a3;
    while (1)
    {
        v4 -= 1;
        sub_412be0(*((long long *)(v3 * 16 + v2[40])), v1, 12);
        v5 = v1 + 24;
        sub_412be0(*((long long *)(*(a1) * 16 + idx->field_18->field_140 + 8)), v1 + 12, 12);
        v2 = idx->field_18;
        v3 = *(a1) + 1;
        *(a1) = v3;
        if (v3 >= v2[38])
            return v3;
        v1 = v5;
        if (!v4)
            return v3;
    }
}



// Function: oldgnu_dump_header @ 0x29480
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[304];
    unsigned long long field_130;
    char padding_138[8];
    unsigned long long field_140;
} struct_1;

unsigned int oldgnu_dump_header(struct_0 *index)
{
    char *v2;  // rax
    char *idx;  // r12
    unsigned long long *v4;  // rax
    unsigned long long *v5;  // rdi
    unsigned long v6;  // r14
    struct_0 *v7;  // rax
    struct_0 *v8;  // r13
    unsigned long long *iter;  // rdi
    unsigned long long v10;  // rcx
    unsigned long i;  // r14
    unsigned long v0;  // [bp-0x38]

    sub_40d180();
    v2 = sub_4132d0(index->field_18);
    v2[156] = 83;
    idx = v2;
    v4 = index->field_18;
    if (v4[38] > 4)
        idx[482] = 1;
    sub_412be0(v4[0x11], idx + 483, 12);
    sub_412be0(*((long long *)&index->field_18->padding_0[280]), idx + 124, 12);
    v0 = 0;
    sub_4293d0(index, &v0, idx + 386, 4);
    v5 = index->field_18;
    v6 = v0;
    idx[482] = v5[38] > v6;
    sub_412f60(v5, idx);
    if (v6 >= index->field_18->field_130)
        return 1;
    do
    {
        v7 = sub_40e130();
        *((unsigned long long *)&v7->padding_0[0]) = 0;
        v8 = v7;
        iter = &v7->padding_0[8] & 0xfffffffffffffff8;
        v7[15].field_18 = 0;
        for (v10 = (unsigned int)(v7 - iter) + 0x200 >> 3; v10; iter += 1)
        {
            v10 -= 1;
            *(iter) = 0;
        }
        sub_4293d0(index, &v0, v8, 21);
        i = v0;
        if (index->field_18->field_130 > i)
            *((char *)&v8[15].field_18) = 1;
        sub_40d1f0(v8);
    } while (i < index->field_18->field_130);
    return 1;
}



// Function: check_sparse_region @ 0x29770
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    long long field_8;
} struct_0;

unsigned int check_sparse_region(struct_0 *a0, long long a1, long long a2)
{
    long long v3;  // r15
    unsigned int v4;  // eax
    long long v5;  // r14
    unsigned long long v6;  // rax
    unsigned long v7;  // rdx
    unsigned long v8;  // rax
    unsigned long v9;  // rax
    char v0;  // [bp-0x268]
    char v1;  // [bp-0x248]

    v3 = a1;
    v4 = sub_428050(a0, a1);
    if (!(char)v4)
    {
        return v4;
    }
    else if (v3 < a2)
    {
        while (1)
        {
            v5 = a2 - v3;
            if (0x200 < v5)
                v5 = 0x200;
            v6 = sub_44af10();
            if (v6 == 0xffffffffffffffff)
            {
                sub_424e00(*((long long *)a0[1].field_8), v3, v5);
                break;
            }
            else if (!v6)
            {
                v8 = dcgettext(NULL, "Size differs", 5);
                sub_410b50(a0[1].field_8, v8);
                break;
            }
            else if ((char)sub_427df0(&v1, v6))
            {
                v3 += v7;
                if (a2 <= v3)
                    return v4;
            }
            else
            {
                sub_442ff0(v3, &v0);
                v9 = dcgettext(NULL, "File fragment at %s is not a hole", 5);
                sub_410b50(a0[1].field_8, v9);
                break;
            }
        }
        return 0;
    }
    else
    {
        return v4;
    }
}



// Function: check_data_region @ 0x298b0
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    long long field_8;
} struct_0;

extern struct_0 g_484840;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

int check_data_region(struct_0 *idx, unsigned long a1)
{
    unsigned long long v4;  // r15
    long long i;  // rbp
    long long v6;  // r12
    unsigned long v7;  // rax
    unsigned long v8;  // rdi
    unsigned long v9;  // rax
    unsigned long long *v11;  // r15
    unsigned long v13;  // rax
    char v0;  // [bp-0x251]
    unsigned long long v1;  // [bp-0x250]
    char v2;  // [bp-0x248]

    v4 = a1 * 16;
    v1 = v4;
    v0 = sub_428050(idx, *((long long *)(*((long long *)(idx[1].field_8 + 320)) + v4)));
    if (v0)
    {
        i = *((long long *)(*((long long *)(idx[1].field_8 + 320)) + v4 + 8));
        sub_40ccf0(*((long long *)(idx[1].field_8 + 280)) - *((long long *)&idx[1].field_0));
        while (i > 0)
        {
            v6 = 0x200;
            if (i <= 0x200)
                v6 = i;
            v7 = sub_40e130();
            if (!v7)
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Unexpected EOF in archive", 5));
                g_484d58 = 2;
                goto LABEL_429a20;
            }
            sub_40d1f0(v7);
            v8 = idx->field_0;
            *((unsigned long long *)&idx[1].field_0) = *((long long *)&idx[1].field_0) + 0x200;
            v9 = sub_44af10(v8, &v2, v6);
            if (v9 == 0xffffffffffffffff)
            {
                v11 = v1 + *((long long *)(idx[1].field_8 + 320));
                sub_424e00(*((long long *)idx[1].field_8), v11[1] + *(v11) - i, v6);
                goto LABEL_429a20;
            }
            if (!v9)
            {
                sub_410b50(&g_484840.field_0, dcgettext(NULL, "Size differs", 5));
                goto LABEL_429a20;
            }
            i -= v9;
            sub_40ccf0(*((long long *)(idx[1].field_8 + 280)) - *((long long *)&idx[1].field_0));
            if (memcmp(v7, &v2, v9))
            {
                v13 = dcgettext(NULL, "Contents differ", 5);
                sub_410b50(idx[1].field_8, v13);
LABEL_429a20:
                v0 = 0;
                break;
            }
        }
    }
    return v0;
}



// Function: sparse_dump_file @ 0x29cd0
typedef struct struct_1 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[124];
    unsigned int field_88;
    char padding_8c[140];
    unsigned long long field_118;
    char padding_120[16];
    unsigned long long field_130;
} struct_1;

typedef struct struct_0 {
    char padding_0[56];
    unsigned long long field_38;
} struct_0;

unsigned int sparse_dump_file(unsigned int a0, struct_1 *a1)
{
    struct_1 *v6;  // rax
    unsigned long long v7;  // rbx
    char v0;  // [bp-0x58], Other Possible Types: unsigned int
    char v1;  // [bp-0x54]
    unsigned long v2;  // [bp-0x48]
    struct_1 *v3;  // [bp-0x40]
    struct_0 *v4;  // [bp-0x38]

    if (!(char)(sub_427f70(&v0) & 0xffffffff))
        return 3;
    v3 = a1;
    v0 = a0;
    v1 = 1;
    if ((char)sub_429c60(&v0))
    {
        if (v4->field_38)
        {
            sub_429cb0(&v0);
            v6 = v3;
            if (a0 < 0)
            {
                sub_413c80(v6->field_118 - v2);
                return (char)sub_427d30(&v0) ^ 1;
            }
            v7 = 0;
            sub_40cc00(*((long long *)&v6->field_8), *((long long *)&v6->field_88), v6->field_118 - v2);
            do
            {
                v6 = v3;
                if (v6->field_130 <= v7)
                {
                    sub_413c80(v6->field_118 - v2);
                    return (char)sub_427d30(&v0) ^ 1;
                }
            } while ((v7 += 1, (char)sub_427d70(&v0)));
        }
        else
        {
            v6 = v3;
            sub_413c80(v6->field_118 - v2);
            return (char)sub_427d30(&v0) ^ 1;
        }
    }
    sub_413c80(v3->field_118 - v2);
    sub_427d30(&v0);
    return 1;
}



// Function: sparse_member_p @ 0x29e10
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

unsigned long long sparse_member_p(unsigned long long a0)
{
    unsigned long long v3;  // rax
    struct_1 v0;  // [bp-0x58]
    unsigned long long v1;  // [bp-0x40]

    v3 = sub_427f70(&v0);
    if (!(char)v3)
        return v3;
    v1 = a0;
    return sub_427d10(&v0);
}



// Function: sparse_fixup_header @ 0x29e70
typedef struct struct_1 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

unsigned long long sparse_fixup_header(unsigned long long a0)
{
    unsigned long long v3;  // rax
    struct_1 v0;  // [bp-0x58]
    unsigned long long v1;  // [bp-0x40]

    v3 = sub_427f70(&v0);
    if (!(char)v3)
        return v3;
    v1 = a0;
    return sub_427dd0(&v0);
}



// Function: sparse_extract_file @ 0x29ed0
typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

unsigned int sparse_extract_file(unsigned int a0, struct_0 *idx, unsigned long long *a2)
{
    unsigned long long v5;  // rbx
    char v0;  // [bp-0x68], Other Possible Types: unsigned int
    char v1;  // [bp-0x64]
    unsigned long long v2;  // [bp-0x60]
    unsigned long v3;  // [bp-0x58]

    if (!(char)(sub_427f70(&v0) & 0xffffffff))
        return 3;
    v0 = a0;
    v2 = 0;
    v1 = !lseek(a0, 0, 0);
    v5 = 0;
    if ((char)sub_427db0(&v0))
    {
        do
        {
            if (*((long long *)&idx[1].padding_0[16]) <= v5)
            {
                *(a2) = idx->field_118 - v3;
                return (char)sub_427d30(&v0) ^ 1;
            }
        } while ((v5 += 1, (char)sub_427d90(&v0)));
    }
    *(a2) = idx->field_118 - v3;
    sub_427d30(&v0);
    return 1;
}



// Function: sparse_skip_file @ 0x29fe0
typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

unsigned int sparse_skip_file(struct_0 *a0)
{
    char v3;  // al
    unsigned int v4;  // eax
    char v0;  // [bp-0x58], Other Possible Types: unsigned int
    unsigned long v1;  // [bp-0x48]

    if (!(char)(sub_427f70(&v0) & 0xffffffff))
        return 3;
    v0 = 0xffffffff;
    v3 = sub_427db0(&v0);
    sub_422b10(a0->field_118 - v1);
    v4 = (char)sub_427d30(&v0) & v3 ^ 1;
    return v4;
}



// Function: sparse_diff_file @ 0x2a070
int sparse_diff_file(unsigned int a0, unsigned long long *a1)
{
    unsigned int v5;  // eax
    unsigned int v6;  // r13d
    long long v7;  // rsi
    unsigned long long v8;  // rbp
    unsigned long long *v9;  // rax
    unsigned long long v10;  // rbx
    char v12;  // al
    unsigned long long *v13;  // rbx
    char v0;  // [bp-0x68], Other Possible Types: unsigned int
    char v1;  // [bp-0x64]
    unsigned long v2;  // [bp-0x58]
    unsigned long long *v3;  // [bp-0x50]

    v5 = sub_427f70(&v0);
    if (!(char)v5)
        return v5;
    v3 = a1;
    v0 = a0;
    v1 = 1;
    v6 = sub_427db0(&v0);
    sub_40ccb0(a1);
    if ((char)v6)
    {
        v7 = 0;
        v8 = 0;
        v9 = v3;
        do
        {
            if (v9[38] <= v8)
            {
                sub_40ccd0();
                sub_427d30(&v0);
                return v6;
            }
            v10 = v8 * 16;
            if (!(char)sub_429770(&v0, v7, *((long long *)(v9[40] + v10))))
                goto LABEL_42a127;
        } while ((v12 = (char)(long long)sub_4298b0(&v0, v8), v9 = v3, v13 = (unsigned long long *)(v10 + v9[40]), v7 = (long long)(v13[1] + *(v13)), v8 += 1, (char)(long long)sub_4298b0(&v0, v8)));
    }
LABEL_42a127:
    v9 = v3;
    sub_422b10(v9[35] - v2);
    sub_40ccd0();
    sub_427d30(&v0);
    return 0;
}



// Function: find_compression_suffix @ 0x2a190
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[16];
    struct struct_0 *field_18;
} struct_0;

extern struct_0 g_481920;

struct_0 * find_compression_suffix(char *a0, unsigned long long *a1)
{
    struct_0 *ptr;  // rbp
    struct_0 *v2;  // r15
    unsigned long len;  // rax
    struct_0 *v4;  // rbp
    unsigned long v5;  // r12
    struct_0 *iter;  // rbx

    ptr = strrchr(a0, 46);
    if (!ptr)
        return ptr;
    v2 = (char *)&ptr->field_0 + 1;
    len = strlen(v2);
    v4 = g_481920.field_0;
    v5 = len;
    if (!g_481920.field_0)
        return g_481920.field_0;
    iter = &g_481920.field_0;
    while (*((long long *)&iter->padding_8[0]) != v5 || memcmp(v4, v2, v5))
    {
        v4 = iter->field_18;
        iter = &iter->field_18;
        if (!v4)
            return v4;
    }
    if (!a1)
        return iter;
    *(a1) = strlen(a0) - v5 - 1;
    return iter;
}



// Function: find_compression_program @ 0x2a240
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[16];
    struct struct_0 *field_18;
} struct_0;

long long find_compression_program(char *a0, unsigned long a1)
{
    struct_0 *v1;  // rdx

    v1 = sub_42a190(a0, NULL);
    if (!v1)
        return a1;
    return *((long long *)&v1->padding_8[8]);
}



// Function: set_compression_program_by_suffix @ 0x2a260
extern unsigned long long g_484b70;

long long set_compression_program_by_suffix(char *a0, unsigned int a1)
{
    long long v1;  // rax

    v1 = sub_42a240(a0, a1);
    if (v1)
        g_484b70 = v1;
    return v1;
}



// Function: strip_compression_suffix @ 0x2a280
typedef struct struct_0 {
    char field_0;
} struct_0;

struct_0 ** strip_compression_suffix(void* a0)
{
    struct_0 **v2;  // r12
    unsigned long n;  // rbx
    unsigned long v4;  // r13
    struct_0 **v5;  // r12
    unsigned long long v0;  // [bp-0x38]

    v2 = sub_42a190(a0, &v0);
    if (!v2)
        return v2;
    n = v0;
    if (n > 4)
    {
        v4 = n - 4;
        if (!strncmp(a0 + v4, ".tar", 4) && *(v2)->field_0 != 116)
            n = v4;
    }
    else if (!n)
    {
        return NULL;
    }
    v5 = sub_44c5b0(n + 1);
    memcpy(v5, a0, n);
    *((char *)v5 + n) = 0;
    return v5;
}



// Function: is_regular_file @ 0x2a340
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

char is_regular_file(char *a0)
{
    unsigned int v2;  // eax
    char v3;  // al
    stat v0;  // [bp-0xa8]

    v2 = stat(a0, &v0);
    v3 = (!v2 ? (v0.st_nlink & 0xf000) == 0x8000 : *(__errno_location()) == 2);
    return v3;
}



// Function: xexec @ 0x2a3b0
int xexec(long long a0)
{
    unsigned long v6;  // fs
    unsigned long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    long long v2;  // [bp-0x28]
    unsigned long long v3;  // [bp-0x20]
    unsigned long v4;  // [bp-0x10]

    v4 = *((long long *)(40 + v6));
    v0 = "/bin/sh";
    v1 = "-c";
    v2 = a0;
    v3 = 0;
    execv("/bin/sh", &v0);
    sub_4326c0(a0); /* do not return */
}



// Function: xdup2 @ 0x2a400
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;

unsigned long long xdup2(int a0, int a1)
{
    unsigned long long v2;  // rax
    unsigned long long v3;  // r12
    unsigned int v4;  // r12d
    char *v5;  // rax
    int v6;  // eax
    int *err;  // rax
    char *v8;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (a0 == a1)
        return v2;
    v0 = v3;
    if (close(a1))
    {
        v4 = *(__errno_location());
        if (*(__errno_location()) != 9)
        {
            if (g_484d50)
                g_484d50();
            v5 = dcgettext(NULL, "Cannot close", 5);
            error(0, *(__errno_location()), v5);
            sub_4163d0(); /* do not return */
        }
    }
    v6 = dup(a0);
    if (a1 == v6)
    {
        return sub_40d2c0(a0);
    }
    else if (v6 < 0)
    {
        err = __errno_location();
        if (g_484d50)
            g_484d50();
        v8 = dcgettext(NULL, "Cannot dup", 5);
        error(0, *(err), v8);
        sub_4163d0(); /* do not return */
    }
    else
    {
        abort(); /* do not return */
    }
}



// Function: wait_for_grandchild @ 0x2a4c0
extern long long g_484b70;

void wait_for_grandchild(int a0)
{
    unsigned long long v4;  // rbx
    unsigned long v5;  // fs
    unsigned int v6;  // edi
    unsigned int v0;  // [bp-0x24]
    unsigned long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x10]

    v2 = v4;
    v1 = *((long long *)(40 + v5));
    do
    {
        if (waitpid(a0, &v0, 0) != 0xffffffff)
            goto LABEL_42a4fb;
    } while (*(__errno_location()) == 4);
    sub_432df0(g_484b70);
LABEL_42a4fb:
    v6 = v0 & 0xff;
    if (((char)v0 & 127) + 1 > 1)
    {
        raise(v0 & 127);
        v6 = 0;
    }
    exit(v6); /* do not return */
}



// Function: run_decompress_program @ 0x2a530
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_483b82;
extern unsigned long long environ;
extern struct_0 *g_484d50;

long long run_decompress_program(void)
{
    unsigned long long v13;  // r15
    unsigned long long v14;  // r14
    unsigned long long v15;  // r13
    unsigned long long v16;  // r12
    unsigned long long v17;  // rbx
    unsigned long v18;  // fs
    unsigned long v19;  // rax
    unsigned long i;  // r12
    unsigned long v21;  // r14
    int *err;  // rax
    char *v0;  // [bp-0x140]
    char v1;  // [bp-0x12c]
    unsigned long idx;  // [bp-0x128]
    char **ptr;  // [bp-0x120]
    unsigned long long v4;  // [bp-0x118]
    unsigned long v5;  // [bp-0xb8]
    unsigned long v6;  // [bp-0x40]
    unsigned long long v7;  // [bp-0x30]
    unsigned long long v8;  // [bp-0x20]
    unsigned long long v9;  // [bp-0x18]
    unsigned long long v10;  // [bp-0x10]
    unsigned long long v11;  // [bp-0x8]

    v11 = v13;
    v10 = v14;
    v9 = v15;
    v8 = v16;
    v7 = v17;
    v6 = *((long long *)(40 + v18));
    v4 = 1;
    v5 = environ;
    v19 = sub_40cdf0(&v1);
    if (v19)
    {
        for (i = v19; !sub_4382c0(i, &idx); i = v21)
        {
            memmove(ptr, &ptr[v4], idx * 8);
            ptr[idx] = "-d";
            execvp(*(ptr), ptr);
            ptr[idx] = 0;
            v21 = sub_40ce40(&v1);
            if (!v21)
            {
                sub_4326c0(i); /* do not return */
            }
            else if (g_483b82 & 8)
            {
                if (g_484d50)
                    g_484d50();
                v0 = dcgettext(NULL, "cannot run %s", 5);
                err = __errno_location();
                error(0, *(err), v0);
                if (g_483b82 & 8)
                {
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcgettext(NULL, "trying %s", 5));
                }
            }
        }
        if (g_484d50)
            g_484d50();
        sub_4354b0(&idx);
        error(0, 0, dcgettext(NULL, "cannot split string '%s': %s", 5));
        sub_4163d0(); /* do not return */
    }
    else
    {
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "unable to run decompression program", 5));
        sub_4163d0(); /* do not return */
    }
}



// Function: str_to_env @ 0x2a720
int str_to_env(char *a0, char *a1)
{
    int v1;  // eax

    if (!a1)
        return unsetenv(a0);
    v1 = setenv(a0, a1, 1);
    if (v1)
        sub_416420(); /* do not return */
    return v1;
}



// Function: time_to_env @ 0x2a750
void time_to_env(char *a0, long long a1, unsigned int a2)
{
    char v0;  // [bp-0x38]

    if (setenv(a0, sub_424270(a1, a2, &v0), 1))
        sub_416420(); /* do not return */
    return;
}



// Function: dec_to_env @ 0x2a7b0
void dec_to_env(char *a0, long long a1)
{
    char v0;  // [bp-0x28]

    if (setenv(a0, sub_443090(a1, &v0), 1))
        sub_416420(); /* do not return */
    return;
}



// Function: stat_to_env @ 0x2a8f0
typedef struct struct_1 {
    char padding_0[8];
    char *field_8;
    char padding_10[8];
    char *field_18;
    char *field_20;
    char *field_28;
    char padding_30[64];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[4];
    long long field_80;
    long long field_88;
    char padding_90[88];
    long long field_e8;
    unsigned long long field_f0;
    long long field_f8;
    unsigned long long field_100;
    long long field_108;
    unsigned long long field_110;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484828;
extern unsigned long long g_484838;
extern int g_484ba0;

int stat_to_env(char *a0, char a1, struct_1 *a2)
{
    sub_42a720("TAR_VERSION", "1.34");
    sub_42a720("TAR_ARCHIVE", g_484828->field_0);
    sub_42a7b0("TAR_VOLUME", ((char *)g_484828 - g_484838 >> 3) + 1);
    sub_42a7b0("TAR_BLOCKING_FACTOR", g_484ba0);
    sub_42a720("TAR_FORMAT", sub_42c230());
    sub_42a810(a1);
    sub_42a870(a2->field_70);
    sub_42a720("TAR_FILENAME", a0);
    sub_42a720("TAR_REALNAME", a2->field_8);
    sub_42a720("TAR_UNAME", a2->field_20);
    sub_42a720("TAR_GNAME", a2->field_28);
    sub_42a750("TAR_ATIME", a2->field_e8, a2->field_f0);
    sub_42a750("TAR_MTIME", a2->field_f8, a2->field_100);
    sub_42a750("TAR_CTIME", a2->field_108, a2->field_110);
    sub_42a7b0("TAR_SIZE", a2->field_88);
    sub_42a7b0("TAR_UID", a2->field_74);
    sub_42a7b0("TAR_GID", a2->field_78);
    switch (a1)
    {
    case 104: case 108:
        unsetenv("TAR_MINOR");
        unsetenv("TAR_MAJOR");
        return sub_42a720("TAR_LINKNAME", a2->field_18);
    case 98: case 99:
        sub_42a7b0("TAR_MINOR", gnu_dev_minor(a2->field_80) & 0xffffffff);
        sub_42a7b0("TAR_MAJOR", gnu_dev_major(a2->field_80) & 0xffffffff);
        return unsetenv("TAR_LINKNAME");
    default:
        unsetenv("TAR_MINOR");
        unsetenv("TAR_MAJOR");
        return unsetenv("TAR_LINKNAME");
    }
}



// Function: sys_get_archive_stat @ 0x2ab10
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

extern stat g_484740;
extern int g_484a34;

char sys_get_archive_stat(void)
{
    return !fstat(g_484a34, &g_484740.st_mode);
}



// Function: sys_file_is_archive @ 0x2ab40
typedef struct struct_0 {
    char padding_0[96];
    unsigned long long field_60;
} struct_0;

extern char g_484810;
extern unsigned long long g_484818;

unsigned long long sys_file_is_archive(struct_0 *a0)
{
    if (!g_484818)
    {
        return 0;
    }
    else if (g_484818 != *((long long *)&a0->padding_0[88]))
    {
        return 0;
    }
    else
    {
        return a0->field_60 == *((long long *)&g_484810);
    }
}



// Function: sys_save_archive_dev_ino @ 0x2ab80
extern unsigned long long g_484740;
extern unsigned long long g_484748;
extern unsigned int g_484758;
extern unsigned long long g_484810;
extern unsigned long long g_484818;
extern int g_484a34;

unsigned long long sys_save_archive_dev_ino(void)
{
    unsigned long long v1;  // rax

    if (g_484a34 <= 0x3fffffff)
    {
        v1 = g_484758 & 0xf000;
        if ((unsigned int)v1 == 0x8000)
        {
            g_484818 = g_484740;
            g_484810 = g_484748;
            return g_484748;
        }
    }
    g_484818 = 0;
    return v1;
}



// Function: sys_detect_dev_null_output @ 0x2abd0
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
    unsigned long long field_0;
} struct_0;

extern char g_484740;
extern char g_484748;
extern unsigned int g_484758;
extern struct_0 *g_484838;
extern char g_484a30;
extern int g_484a34;

void sys_detect_dev_null_output(void)
{
    char v2;  // al
    unsigned int v3;  // eax
    stat v0;  // [bp-0xa8]

    v2 = 1;
    if (strcmp(g_484838->field_0, "/dev/null"))
    {
        if (g_484a34 > 0x3fffffff || ((unsigned short)g_484758 & 0xf000) != 0x2000 || (v3 = (unsigned int)(int)(long long)stat("/dev/null", &v0), (int)(long long)stat("/dev/null", &v0) || *((long long *)&g_484740) != *((unsigned long long *)(void*)&v0)))
            v2 = 0;
        else
            v2 = *((long long *)&g_484748) == v0.st_ino;
    }
    g_484a30 = v2;
    return;
}



// Function: sys_wait_for_child @ 0x2aca0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern long long g_484b70;
extern struct_0 *g_484d50;

void sys_wait_for_child(int a0, char a1)
{
    unsigned int v3;  // ecx
    unsigned int v4;  // ecx
    char *v5;  // rax
    unsigned int v0;  // [bp-0x2c]
    int v1;  // [bp-0x24]

    if (!a0)
        return;
    do
    {
        if (waitpid(a0, &v1, 0) != 0xffffffff)
        {
            if (((char)v1 & 127) + 1 <= 1)
                goto LABEL_42ad28;
            goto LABEL_42acff;
        }
    } while (*(__errno_location()) == 4);
    sub_432df0(g_484b70);
    v3 = v1;
    if (((char)v3 & 127) + 1 > 1)
    {
LABEL_42acff:
        v4 = v3 & 127;
        if (v4 == 13 && !a1)
            return;
        if (g_484d50)
            g_484d50();
        v0 = v4;
        v5 = dcgettext(NULL, "Child died with signal %d", 5);
    }
    else
    {
LABEL_42ad28:
        if (!(char)v3)
            return;
        if (g_484d50)
            g_484d50();
        v5 = dcgettext(NULL, "Child returned status %d", 5);
    }
    error(0, 0, v5);
    sub_4163d0(); /* do not return */
}



// Function: sys_spawn_shell @ 0x2add0
int sys_spawn_shell(void)
{
    unsigned long v3;  // rax
    unsigned int v4;  // eax
    unsigned long v5;  // fs
    int v0;  // [bp-0x24]
    unsigned long v1;  // [bp-0x20]

    v3 = getenv("SHELL");
    if (!v3)
        v3 = "/bin/sh";
    v4 = sub_424f80();
    if (!v4)
    {
        execlp(v3, "-sh");
        sub_4326c0(v3); /* do not return */
    }
    while (waitpid(v4, &v0, 0) == 0xffffffff)
    {
        if (*(__errno_location()) != 4)
        {
            sub_432df0(v3);
            break;
            return v1 - *((long long *)(40 + v5));
        }
    }
    return v1 - *((long long *)(40 + v5));
}



// Function: sys_compare_uid @ 0x2ae90
typedef struct struct_0 {
    char padding_0[28];
    unsigned int field_1c;
} struct_0;

long long sys_compare_uid(unsigned long a0, struct_0 *a1)
{
    unsigned long v1;  // rax

    v1 = a1->field_1c;
    return _INSERT(v1, 0, *((int *)(a0 + 28)) == (unsigned int)v1);
}



// Function: sys_compare_gid @ 0x2aea0
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

long long sys_compare_gid(unsigned long a0, struct_0 *a1)
{
    unsigned long v1;  // rax

    v1 = a1->field_20;
    return _INSERT(v1, 0, *((int *)(a0 + 32)) == (unsigned int)v1);
}



// Function: sys_compare_links @ 0x2aeb0
unsigned long long sys_compare_links(unsigned long a0, unsigned long long *a1)
{
    if (*(a1) == *((long long *)a0))
        return _INSERT(*((long long *)(a0 + 8)), 0, a1[1] == *((long long *)(a0 + 8)));
    return 0;
}



// Function: sys_truncate @ 0x2aed0
int sys_truncate(int a0)
{
    long long v1;  // rax

    v1 = lseek(a0, 0, 1);
    if (v1 < 0)
        return -0x1;
    return ftruncate(a0, v1);
}



// Function: sys_write_archive_buffer @ 0x2af00
extern char g_483c20;
extern long long g_483f58;
extern void g_484a34;
extern long long g_484b98;

long long sys_write_archive_buffer(void)
{
    unsigned long long v6;  // r13
    unsigned long long v7;  // r12
    unsigned int v8;  // ebp
    unsigned long long v9;  // rbx
    unsigned long v10;  // fs
    sighandler_t v11;  // rax
    long long v12;  // r12
    long long v13;  // rax
    char v0[72];  // [bp-0x78]
    unsigned long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (*((int *)&g_484a34) <= 0x3fffffff)
        return sub_441120(*((int *)&g_484a34), g_483f58, g_484b98);
    v4 = v6;
    v3 = v7;
    v8 = *((int *)&g_484a34) - 0x40000000;
    v2 = v9;
    v1 = *((long long *)(40 + v10));
    __sprintf_chk(v0, 0x1);
    if (sub_4335d0(v8, v0) != 0xffffffff)
    {
        v11 = signal(13, 0x1);
        v12 = sub_441120(*((int *)&(&g_483c20)[4 + 8 * v8]), g_483f58, g_484b98);
        signal(13, v11);
        if (g_484b98 == v12)
        {
            v13 = sub_4337a0(v8);
            if (v13 >= 0)
            {
                if (v12 == v13)
                {
                    if (v1 == *((long long *)(40 + v10)))
                        return v12;
                    __stack_chk_fail(); /* do not return */
                }
                v12 = v13;
            }
            else
            {
                if (v1 == *((long long *)(40 + v10)))
                    return 0;
                __stack_chk_fail(); /* do not return */
            }
        }
        sub_4331f0(v8, 5);
        if (v1 != *((long long *)(40 + v10)))
            __stack_chk_fail(); /* do not return */
        return v12;
    }
    else if (v1 == *((long long *)(40 + v10)))
    {
        return 0;
    }
    else
    {
        __stack_chk_fail(); /* do not return */
    }
}



// Function: sys_child_open_for_compress @ 0x2af40
extern unsigned long long g_483f58;
extern unsigned long long g_484838;
extern int g_484a34;
extern long long g_484ab0;
extern long long g_484b70;
extern char g_484b84;
extern void g_484b98;
extern char g_484d70;
extern unsigned long long g_484d78;

long long sys_child_open_for_compress(void)
{
    unsigned int v6;  // eax
    unsigned long i;  // rbx
    unsigned long long v17;  // rax
    unsigned long v18;  // rdi
    unsigned long long v8;  // rdi
    unsigned long long v9;  // rdi
    unsigned int *err;  // rax
    unsigned int v11;  // eax
    unsigned long long v12;  // r12
    unsigned int v13;  // eax
    unsigned long v14;  // rdx
    unsigned long v15;  // r12
    unsigned long long v0;  // [bp-0x40]
    int v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x2c]
    char v3;  // [bp-0x28]
    int v4;  // [bp-0x24]

    signal(13, 0x1);
    sub_424fc0(&v1);
    v6 = sub_424f80();
    if (v6 > 0)
    {
LABEL_42b076:
        g_484a34 = v2;
        sub_40d2c0(v1);
        return v6;
    }
    sub_448760(dcgettext(NULL, "tar (child)", 5));
    signal(13, NULL);
    sub_42a400(v1, 0);
    sub_40d2c0(v2);
    if (!g_484d70)
    {
        g_484d78 = strchr(*((long long *)g_484838), 58);
        v8 = *((long long *)g_484838);
        v9 = v8;
        if (g_484d78 && !(v9 = v8, g_484d78 <= v9 || (v0 = v8, v9 = v0, memchr(v8, 47, g_484d78 - v8))))
            goto LABEL_42b0cf;
    }
    else
    {
        v9 = *((long long *)g_484838);
    }
    if (sub_42a340(v9))
    {
        if (g_484b84)
            sub_424710(*((long long *)g_484838), 1);
        if (strcmp(*((long long *)g_484838), "-"))
        {
            g_484a34 = creat(*((long long *)g_484838), 438);
            if (g_484a34 < 0)
            {
                err = __errno_location();
                if (g_484b84)
                    sub_4245d0();
                *(err) = *(err);
                v6 = sub_4327e0(*((long long *)g_484838));
                goto LABEL_42b076;
            }
            else
            {
                sub_42a400(g_484a34, 1);
            }
        }
    }
    else
    {
LABEL_42b0cf:
        sub_424fc0(&v3);
        v11 = sub_424f80();
        if (v11)
        {
            sub_42a400(v3, 0);
            sub_40d2c0(v4);
            v12 = *((long long *)g_484838);
            if (!strcmp(v12, "-"))
            {
                g_484a34 = 1;
LABEL_42b127:
                v14 = *((long long *)&g_484b98);
            }
            else
            {
                if (!g_484d70 && !(g_484d78 = (unsigned long long)strchr(v12, 58), v12 = (unsigned long long)*((long long *)g_484838), !g_484d78 || g_484d78 <= v12 || memchr(v12, 47, g_484d78 - v12)))
                    v13 = sub_4338d0(v12, 65, 0x40000000, g_484ab0);
                else
                    v13 = creat(v12, 438);
                g_484a34 = v13;
                if (v13 >= 0)
                    goto LABEL_42b127;
                sub_4327e0(*((long long *)g_484838));
                goto LABEL_42b290;
            }
            while (1)
            {
                v15 = g_483f58;
                if (!v14)
                    sub_42a4c0(v11); /* do not return */
                i = 0;
                do
                {
                    v17 = sub_44af10(0, v15, v14 - i);
                    if (v17 == 0xffffffffffffffff)
                        sub_4329b0(g_484b70); /* do not return */
                    if (!v17)
                    {
                        if (i)
                        {
                            memset(g_483f58 + i, 0, *((long long *)&g_484b98) - i);
                            v18 = sub_42af00();
                            if (*((long long *)&g_484b98) == v18)
                                sub_42a4c0(v11); /* do not return */
                            sub_40d6d0(v18); /* do not return */
                        }
                        else
                        {
                            sub_42a4c0(v11); /* do not return */
                        }
                    }
                    v14 = *((long long *)&g_484b98);
                    i += v17;
                    v15 += v17;
                } while (*((long long *)&g_484b98) > i);
LABEL_42b290:
                v14 = sub_42af00();
                if (*((long long *)&g_484b98) != v14)
                    sub_40d6d0(v18); /* do not return */
            }
        }
        else
        {
            sub_448760(dcgettext(NULL, "tar (grandchild)", 5));
            sub_42a400(v4, 1);
            sub_40d2c0(v3);
        }
    }
    sub_42a3b0(g_484b70); /* do not return */
}



// Function: sys_child_open_for_uncompress @ 0x2b310
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern long long g_483f58;
extern struct_0 *g_484838;
extern unsigned int g_484a34;
extern long long g_484ab0;
extern long long g_484b70;
extern long long g_484b98;
extern char g_484d70;
extern unsigned long long g_484d78;

long long sys_child_open_for_uncompress(void)
{
    unsigned int v5;  // eax
    unsigned long long v7;  // rbp
    unsigned int v8;  // eax
    unsigned long long v9;  // rbp
    unsigned int v10;  // eax
    unsigned long long v11;  // rbx
    unsigned long long i;  // rbx
    unsigned long long v13;  // rbp
    char v0;  // [bp-0x40]
    int v1;  // [bp-0x3c]
    char v2;  // [bp-0x38]
    int v3;  // [bp-0x34]

    sub_424fc0(&v0);
    v5 = sub_424f80();
    if (v5 > 0)
    {
        g_484a34 = v0;
        sub_40d2c0(v1);
        return v5;
    }
    sub_448760(dcgettext(NULL, "tar (child)", 5));
    signal(13, NULL);
    sub_42a400(v1, 1);
    sub_40d2c0(v0);
    v7 = g_484838->field_0;
    if (!strcmp(v7, "-") || !g_484d70 && !(g_484d78 = (unsigned long long)strchr(v7, 58), v7 = g_484838->field_0, !g_484d78 || g_484d78 <= v7 || memchr(v7, 47, g_484d78 - v7)) || !sub_42a340(v7))
    {
        sub_424fc0(&v2);
        v8 = sub_424f80();
        if (!v8)
        {
            sub_448760(dcgettext(NULL, "tar (grandchild)", 5));
            sub_42a400(v2, 0);
            sub_40d2c0(v3);
            sub_42a530(); /* do not return */
        }
        sub_42a400(v3, 1);
        sub_40d2c0(v2);
        v9 = g_484838->field_0;
        if (!strcmp(v9, "-"))
        {
            g_484a34 = 0;
        }
        else
        {
            if (g_484d70 || (g_484d78 = (unsigned long long)strchr(v9, 58), v9 = g_484838->field_0, !g_484d78 || g_484d78 <= v9 || memchr(v9, 47, g_484d78 - v9)))
                v10 = open(v9, 0, 438);
            else
                v10 = sub_4338d0(v9, 0, 0x40000000, g_484ab0);
            g_484a34 = v10;
            if (v10 < 0)
                goto LABEL_42b5a3;
        }
        while (1)
        {
            sub_40cd10();
            while (1)
            {
                if (g_484a34 > 0x3fffffff)
                {
                    if (sub_433da0(g_484a34 - 0x40000000) != 0xffffffffffffffff)
                        break;
                }
                else
                {
                    if (sub_44af10(g_484a34, g_483f58, g_484b98) != 0xffffffffffffffff)
                        break;
                }
                sub_40d300();
            }
            if (!v11)
                break;
            do
            {
                i = v11;
                v13 = 0x200;
                if (i <= 0x200)
                    v13 = i;
                if (sub_441120() != v13)
                    sub_432e10(g_484b70);
                v11 = i - v13;
            } while (i != v13);
        }
        sub_40d2c0(1);
        sub_42a4c0(v8); /* do not return */
    }
    else
    {
        do
        {
LABEL_42b5a3:
            sub_4327e0(g_484838->field_0);
        } while ((g_484a34 = (unsigned int)(int)(long long)open(g_484838->field_0, 0, 438), g_484a34 < 0));
        sub_42a400(g_484a34, 0);
        sub_42a530(); /* do not return */
    }
}



// Function: sys_exec_command @ 0x2b640
typedef struct struct_1 {
    char padding_0[8];
    char *field_8;
    char padding_10[8];
    char *field_18;
    char *field_20;
    char *field_28;
    char padding_30[64];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[4];
    long long field_80;
    long long field_88;
    char padding_90[88];
    long long field_e8;
    unsigned long long field_f0;
    long long field_f8;
    unsigned long long field_100;
    long long field_108;
    unsigned long long field_110;
} struct_1;

extern unsigned long long g_484728;
extern unsigned int g_484730;
extern long long g_484a60;

long long sys_exec_command(char *a0, char a1, struct_1 *a2)
{
    char v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x24]

    sub_424fc0(&v0);
    g_484728 = signal(13, 0x1);
    g_484730 = sub_424f80();
    if (!g_484730)
    {
        sub_42a400(v0, 0);
        sub_40d2c0(v1);
        sub_42a8f0(a0, a1, a2);
        sub_42a3b0(g_484a60); /* do not return */
    }
    sub_40d2c0(v0);
    return v1;
}



// Function: sys_wait_command @ 0x2b6f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern sighandler_t g_484728;
extern int g_484730;
extern char g_484a59;
extern long long g_484a60;
extern struct_0 *g_484d50;
extern unsigned int g_484d58;

void sys_wait_command(void)
{
    int i;  // ebp
    unsigned int v0;  // [bp-0x24]

    i = g_484730;
    if (g_484730 < 0)
        return;
    signal(13, g_484728);
    for (; waitpid(i, &v0, 0) == 0xffffffff; i = g_484730)
    {
        if (*(__errno_location()) != 4)
        {
            g_484730 = 0xffffffff;
            sub_432df0(g_484a60);
            return;
        }
    }
    if ((char)v0 & 127)
    {
        if (((char)v0 & 127) + 1 > 1)
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "%lu: Child terminated on signal %d", 5));
        }
        else
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "%lu: Child terminated on unknown reason", 5));
            g_484d58 = 2;
        }
    }
    else
    {
        if (!g_484a59 && (char)v0)
        {
            if (g_484d50)
                g_484d50();
            error(0, 0, dcgettext(NULL, "%lu: Child returned status %d", 5));
            g_484d58 = 2;
        }
    }
    g_484730 = 0xffffffff;
    return;
}



// Function: sys_exec_info_script @ 0x2b8b0
typedef struct FILE {
} FILE;

extern unsigned int g_484400;
extern sighandler_t g_484720;
extern long long g_484b48;
extern int g_484ba0;
extern unsigned int g_484ba4;
extern unsigned int g_484ba8;

unsigned int sys_exec_info_script(char **a0, int a1)
{
    unsigned int v7;  // eax
    FILE *fp;  // rax
    unsigned int v9;  // eax
    unsigned int v10;  // ebp
    void* v11;  // rax
    unsigned int v12;  // r12d
    unsigned long v16;  // rdi
    unsigned int v0;  // [bp-0x64]
    char *ptr;  // [bp-0x60]
    unsigned long v2;  // [bp-0x58]
    char v3;  // [bp-0x50]
    int v4;  // [bp-0x4c]
    char v5;  // [bp-0x48]

    sub_424fc0(&v3);
    g_484720 = signal(13, 0x1);
    v7 = sub_424f80();
    if (v7)
    {
        ptr = NULL;
        v2 = 0;
        sub_40d2c0(v4);
        fp = fdopen(v3, "r");
        v9 = getline(&ptr, &v2, fp);
        v10 = v9;
        fclose(fp);
        if (v10 > 0)
        {
            v11 = &ptr[v10] - 1;
            if (*((char *)v11) == 10)
            {
                *((char *)v11) = 0;
                v9 = v10 - 1;
            }
        }
        while (1)
        {
            v12 = waitpid(v7, &v0, 0);
            if (v12 != 0xffffffff)
                break;
            if (*(__errno_location()) != 4)
            {
                signal(13, g_484720);
                sub_432df0(g_484b48);
                return v12;
            }
        }
        signal(13, g_484720);
        if ((char)v0 & 127)
        {
            free(ptr);
            return 0xffffffff;
        }
        if (!(char)v0 && v9 > 0)
        {
            *(a0) = ptr;
            return v0 & 127;
        }
        free(ptr);
        return *((char *)((void*)&v0 + 1));
    }
    else
    {
        setenv("TAR_VERSION", "1.34", 1);
        setenv("TAR_ARCHIVE", *(a0), 1);
        setenv("TAR_VOLUME", sub_443090(a1, &v5), 1);
        setenv("TAR_BLOCKING_FACTOR", sub_443090(g_484ba0, &v5), 1);
        setenv("TAR_SUBCOMMAND", sub_42c7e0(g_484ba8), 1);
        v16 = g_484400;
        if (!g_484400)
            v16 = g_484ba4;
        setenv("TAR_FORMAT", sub_42c230(v16), 1);
        setenv("TAR_FD", sub_443090(v4, &v5), 1);
        sub_40d2c0(v3);
        sub_42a3b0(g_484b48); /* do not return */
    }
}



// Function: sys_exec_checkpoint_script @ 0x2bb40
extern unsigned int g_484400;
extern int g_484ba0;
extern unsigned int g_484ba4;
extern unsigned int g_484ba8;

int sys_exec_checkpoint_script(long long a0, char *a1, int a2)
{
    unsigned int v4;  // eax
    unsigned long v5;  // fs
    unsigned long v9;  // rdi
    int v0;  // [bp-0x4c]
    char v1;  // [bp-0x48]
    unsigned long v2;  // [bp-0x30]

    v4 = sub_424f80();
    if (v4)
    {
        while (waitpid(v4, &v0, 0) == 0xffffffff)
        {
            if (*(__errno_location()) != 4)
            {
                sub_432df0(a0);
                break;
                return v2 - *((long long *)(40 + v5));
            }
        }
        return v2 - *((long long *)(40 + v5));
    }
    else
    {
        setenv("TAR_VERSION", "1.34", 1);
        setenv("TAR_ARCHIVE", a1, 1);
        setenv("TAR_CHECKPOINT", sub_443090(a2, &v1), 1);
        setenv("TAR_BLOCKING_FACTOR", sub_443090(g_484ba0, &v1), 1);
        setenv("TAR_SUBCOMMAND", sub_42c7e0(g_484ba8), 1);
        v9 = g_484400;
        if (!g_484400)
            v9 = g_484ba4;
        setenv("TAR_FORMAT", sub_42c230(v9), 1);
        sub_42a3b0(a0); /* do not return */
    }
}



// Function: optloc_lookup @ 0x2bcb0
extern unsigned long long g_484bc0[4];

long long optloc_lookup(int a0)
{
    return g_484bc0[a0];
}



// Function: option_set_in_cl @ 0x2bcc0
unsigned int option_set_in_cl(int a0)
{
    unsigned int *v1;  // rax

    v1 = sub_42bcb0(a0);
    if (!v1)
        return 0;
    return *(v1) == 1;
}



// Function: find_argp_option_key @ 0x2bce0
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
    unsigned int field_18;
    char padding_1c[4];
    unsigned long long field_20;
} struct_0;

struct_0 * find_argp_option_key(struct_0 *a0, unsigned int a1)
{
    while (1)
    {
        if (!a0->field_0 && !a0->field_8 && !a0->field_10 && !a0->field_18 && !a0->field_20)
            return NULL;
        if (a1 == a0->field_8)
            return a0;
        a0 = &a0[1].field_8;
    }
}



// Function: optloc_eq @ 0x2bd20
typedef struct struct_1 {
    char padding_0[8];
    char *field_8;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
} struct_0;

unsigned int optloc_eq(struct_0 *a0, struct_1 *a1)
{
    if (a0->field_0 != *((int *)&a1->padding_0[0]))
    {
        return 0;
    }
    else if (a0->field_0 == 1)
    {
        return 1;
    }
    else
    {
        return !strcmp(a0->field_8, a1->field_8);
    }
}



// Function: optloc_save @ 0x2bd60
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern unsigned long long g_484bc0[4];

unsigned long long optloc_save(unsigned int a0, struct_0 *index)
{
    unsigned long v1;  // rdi
    struct_1 *v2;  // rax
    char *dst;  // rsi
    struct_1 *idx;  // rbx
    unsigned int v5;  // eax
    unsigned long idx1;  // rax
    unsigned long v7;  // r8

    v1 = 32;
    if (index->field_8)
        v1 = strlen(index->field_8) + 33;
    v2 = sub_44c5b0(v1);
    dst = index->field_8;
    idx = v2;
    if (dst)
        dst = strcpy(idx + 1, dst);
    v5 = index->field_0;
    idx->field_8 = dst;
    idx->field_0 = v5;
    idx->field_10 = index->field_10;
    idx1 = a0;
    v7 = g_484bc0[idx1];
    g_484bc0[idx1] = idx;
    idx->field_18 = v7;
    return v7;
}



// Function: tar_set_quoting_style @ 0x2bde0
typedef struct struct_1 {
    long long field_0;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_480680;
extern struct_1 *g_484d50;

int tar_set_quoting_style(char *a0)
{
    char *v1;  // rsi
    unsigned long long idx;  // rbx

    v1 = g_480680.field_0;
    if (g_480680.field_0)
    {
        idx = 0;
        do
        {
            if (!strcmp(a0, v1))
                return sub_44a590(0, idx & 0xffffffff);
        } while ((idx += 1, v1 = (char *)(&g_480680.field_0)[idx], (&g_480680.field_0)[idx]));
    }
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.", 5));
    sub_4163d0(); /* do not return */
}



// Function: sigstat @ 0x2be90
sighandler_t sigstat(unsigned long a0)
{
    char v0;  // [bp+0x0]

    sub_40cd50();
    sub_40d150();
    return signal((unsigned int)&v0, sigstat);
}



// Function: archive_format_string @ 0x2c230
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned int field_18;
} struct_0;

extern struct_0 g_47f9a0;

unsigned long long archive_format_string(unsigned int a0)
{
    unsigned int v1;  // edx
    struct_0 *i;  // rax
    unsigned long long v3;  // r8
    struct_0 *v4;  // rax

    v1 = 1;
    i = &g_47f9a0.field_0;
    for (v3 = "v7"; a0 != v1; i = v4)
    {
        v3 = i->field_10;
        v4 = &i->field_10;
        if (!i->field_10)
            return "unknown?";
        v1 = *((int *)&v4->padding_0[8]);
    }
    return v3;
}



// Function: format_default_settings @ 0x2c280
int format_default_settings(void)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = "/usr/bin/rsh";
    return sub_44cc50("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s", sub_42c230(6), "-");
}



// Function: tar_help_filter @ 0x2c2d0
unsigned long long tar_help_filter(int a0, unsigned long long a1)
{
    void* ptr;  // rax
    unsigned long len;  // r12
    unsigned long long v19;  // rax
    unsigned long long v20;  // rax
    unsigned long long v21;  // rax
    unsigned long v22;  // fs
    void* ptr1;  // rdi
    unsigned short *v12;  // r12
    void* ptr2;  // rax
    unsigned long len1;  // r12
    unsigned short *v15;  // rdi
    void* v16;  // rax
    unsigned long len2;  // r12
    void* v18;  // rdi
    char v0;  // [bp-0x78]
    unsigned long v1;  // [bp-0x70]
    unsigned long long v2;  // [bp-0x68]
    unsigned short *iter;  // [bp-0x60], Other Possible Types: void*, unsigned long long
    unsigned long long v4;  // [bp-0x58]
    unsigned long v5;  // [bp-0x48]
    char v6;  // [bp-0x28]
    unsigned long v7;  // [bp-0x20]

    switch (a0)
    {
    case 148:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 207:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 106:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 33554436:
        _obstack_begin(&v0, 0, 0, sub_44c5b0, free);
        ptr = dcgettext(NULL, "Valid arguments for the --quoting-style option are:", 5);
        len = strlen(ptr);
        ptr1 = iter;
        if (len > v4 - ptr1)
        {
            _obstack_newchunk(&v0, len);
            ptr1 = iter;
        }
        memcpy(ptr1, ptr, len);
        v12 = len + iter;
        iter = v12;
        if (v4 - (char *)v12 <= 1)
        {
            _obstack_newchunk(&v0, 2);
            v12 = iter;
        }
        *(v12) = 0xa0a;
        iter += 1;
        sub_42c050(&v0);
        ptr2 = dcgettext(NULL, "\n*This* tar defaults to:\n", 5);
        len1 = strlen(ptr2);
        v15 = iter;
        if (len1 > v4 - (char *)v15)
        {
            _obstack_newchunk(&v0, len1);
            v15 = iter;
        }
        memcpy(v15, ptr2, len1);
        iter = (char *)iter + len1;
        v16 = sub_42c280();
        len2 = strlen(v16);
        v18 = iter;
        if (len2 > v4 - v18)
        {
            _obstack_newchunk(&v0, len2);
            v18 = iter;
        }
        memcpy(v18, v16, len2);
        v19 = iter + len2;
        iter = v19;
        if (v19 == v4)
        {
            _obstack_newchunk(&v0, 1);
            v19 = iter;
        }
        iter = v19 + 1;
        *((char *)v19) = 10;
        if (v4 == iter)
        {
            _obstack_newchunk(&v0, 1);
            iter = iter;
        }
        iter += 1;
        *((char *)iter) = 0;
        if (iter == v2)
            v6 |= 2;
        v20 = iter + v5 & ~(v5);
        if (v20 - v1 <= v4 - v1)
            v4 = v20;
        iter = v4;
        v2 = v4;
        v21 = sub_44c7a0();
        _obstack_free(&v0, 0);
        return v21;
    case 149:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 122:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 74:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 150:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    case 90:
        dcgettext(NULL, "filter the archive through %s", 5);
        if (v7 != *((long long *)(40 + v22)))
            __stack_chk_fail(); /* do not return */
        break;
    default:
        return a1;
    }
    return sub_44cc50();
}



// Function: subcommand_string @ 0x2c7e0
extern char g_463997;
extern char g_4639ca;
extern char g_465b22;
extern char g_465b87;
extern char g_465b8a;
extern char g_465b8d;
extern char g_465b90;
extern char g_465b93;
extern char g_465b96;
extern char g_465b99;

int subcommand_string(unsigned int a0)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    switch (a0)
    {
    case 0:
        return &g_465b22;
    case 1:
        return &g_465b87;
    case 2:
        return &g_465b8a;
    case 3:
        return &g_463997;
    case 4:
        return &g_465b8d;
    case 5:
        return &g_4639ca;
    case 6:
        return &g_465b90;
    case 7:
        return &g_465b93;
    case 8:
        return &g_465b96;
    case 9:
        return &g_465b99;
    default:
        v0 = v2;
        abort(); /* do not return */
    }
}



// Function: decode_signal @ 0x2c8a0
typedef struct struct_0 {
    char *field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern struct_0 g_47f8e0;
extern struct_0 g_47f930;
extern struct_1 *g_484d50;

int decode_signal(char *a0)
{
    char *v1;  // r12
    char *v2;  // rbp
    struct_0 *v3;  // rbx

    v1 = a0;
    v2 = v1 + 3;
    v3 = &g_47f8e0.field_0;
    if (!strncmp(a0, "SIG", 3))
        v1 = v2;
    if (strcmp("USR1", v1))
    {
        do
        {
            v3 = (char *)&v3[1].field_0 + 4;
            if (v3 == &g_47f930.field_0)
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Unknown signal name: %s", 5));
                sub_4163d0(); /* do not return */
            }
        } while (strcmp(v3->field_0, v1));
    }
    return v3->field_8;
}



// Function: set_stat_signal @ 0x2c950
int set_stat_signal(char *a0)
{
    sub_42c8a0(a0);
}



// Function: usage @ 0x2c970
extern long long g_481b20;
extern long long stderr;
extern long long g_484dd0;

void usage(int a0)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    long long v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9
    long long v7;  // rbp

    argp_help(&g_481b20, stderr, 4, g_484dd0);
    sub_43e440(v1, v2, v3, v4, v5, v6, v7);
    exit(a0); /* do not return */
}



// Function: request_stdin @ 0x2c9b0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_484c10;
extern struct_0 *g_484d50;

void request_stdin(unsigned long a0)
{
    if (!g_484c10)
    {
        g_484c10 = a0;
        return;
    }
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "Options '%s' and '%s' both want standard input", 5));
    sub_42c970(2); /* do not return */
}



// Function: confirm @ 0x2ca20
typedef struct FILE {
} FILE;

extern unsigned long long stdin;
extern FILE *g_483f28;
extern unsigned int g_484a34;
extern unsigned int g_484bac;
extern FILE *g_484bb0;
extern unsigned long long g_484c10;

int confirm(char *a0, unsigned long a1)
{
    unsigned long v3;  // r12
    char *v4;  // rax
    char *ptr;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]

    v3 = a1;
    if (!g_484bb0)
    {
        if (!g_484a34 || g_484c10)
        {
            g_484bb0 = fopen("/dev/tty", "r");
            if (!g_484bb0)
            {
                sub_4327e0("/dev/tty");
LABEL_42cb26:
                ptr = NULL;
                v1 = 0;
                if (getline(&ptr, &v1, g_484bb0) < NULL)
                {
                    g_484bac = 1;
                    free(ptr);
                    fputc_unlocked(10, g_483f28);
                    fflush_unlocked(g_483f28);
                    return v3;
                }
                v3 = rpmatch(ptr) > 0;
                free(ptr);
                if (!g_484bac)
                    return v3;
                fputc_unlocked(10, g_483f28);
                fflush_unlocked(g_483f28);
                return v3;
            }
        }
        else
        {
            sub_42c9b0("-w");
            g_484bb0 = stdin;
        }
    }
    v4 = sub_44ac50(v3);
    __fprintf_chk(*((unsigned int *)&g_483f28), 0x1, "%s %s?", a0, v4);
    fflush_unlocked(g_483f28);
    v3 = g_484bac;
    if (g_484bac)
    {
        fputc_unlocked(10, g_483f28);
        fflush_unlocked(g_483f28);
        return 0;
    }
    goto LABEL_42cb26;
}



// Function: get_date_or_file @ 0x2cbe0
typedef struct struct_2 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    uint128_t field_8;
    unsigned long long field_18;
    unsigned long long field_20;
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

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

extern struct_3 *g_484d50;

unsigned int get_date_or_file(struct_2 *idx, unsigned long a1, char *a2, void* idx1)
{
    unsigned int v2;  // eax
    unsigned long long v3;  // rdx
    struct_0 *v6;  // rax
    int v7;  // xmm0
    struct_0 *index;  // r14
    struct_0 *v9;  // rax
    stat v0;  // [bp-0xc8]

    if (*(a2) - 46 <= 1)
    {
        v2 = stat(a2, &v0);
        if (!v2)
        {
            *((unsigned long long *)idx1) = sub_44bd20(&v0);
            *((unsigned long long *)&idx1[8]) = v3;
            return v2;
        }
        sub_432d30(a2);
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "Date sample file not found", 5));
        sub_42c970(2); /* do not return */
    }
    else if (!(char)sub_4486d0(idx1, a2, 0))
    {
        if (g_484d50)
            g_484d50();
        sub_44ac50(a2);
        sub_422080(*((long long *)idx1), (long long)idx1[8], 0);
        error(0, 0, dcgettext(NULL, "Substituting %s for unknown date format %s", 5));
        *((unsigned long long *)&idx1[8]) = 0;
        return 1;
    }
    else
    {
        v6 = sub_44c5b0(40);
        v7 = (int)*((int128_t *)idx1);
        *((unsigned long *)((char *)&v6->field_8 + 8)) = a1;
        index = v6;
        *((uint128_t *)&(&index->field_0)[1]) = (uint128_t)v7;
        index->field_18 = sub_44c7a0(a2);
        v9 = idx->field_8;
        idx->field_8 = index;
        index->field_0 = v9;
        return 0;
    }
}



// Function: expand_pax_option @ 0x2cd70
typedef struct struct_2 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[2];
    char field_2;
} struct_0;

unsigned long long expand_pax_option(struct_2 *a0, char *a1)
{
    char *ptr;  // rbp
    unsigned long long ptr1;  // r15
    struct_0 *iter;  // r14
    unsigned long long *v26;  // rax
    void* ptr2;  // r12
    void* v28;  // rdi
    unsigned long v29;  // rax
    unsigned long len;  // r14
    void* v31;  // rdi
    char *v32;  // rbp
    char v33;  // al
    unsigned long long v34;  // rax
    unsigned long long v17;  // r12
    unsigned long long v35;  // rax
    unsigned long v18;  // r12
    unsigned long n;  // rbx
    struct_0 *v20;  // rax
    unsigned long n1;  // r14
    struct_0 *v22;  // rcx
    unsigned long n2;  // r15
    void* v24;  // r12
    struct_0 *v0;  // [bp-0x100], Other Possible Types: char
    char *v1;  // [bp-0xf8]
    char *v2;  // [bp-0xe8]
    char *v3;  // [bp-0xe0]
    long long v4;  // [bp-0xd8]
    unsigned long v5;  // [bp-0xd0]
    char v6;  // [bp-0xc8]
    unsigned long v7;  // [bp-0xc0]
    unsigned long long v8;  // [bp-0xb8]
    void* node;  // [bp-0xb0], Other Possible Types: unsigned long long
    unsigned long long v10;  // [bp-0xa8]
    unsigned long v11;  // [bp-0x98]
    char v12;  // [bp-0x78]
    char v13;  // [bp-0x68]

    ptr = a1;
    v1 = &v6;
    _obstack_begin(&v6, 0, 0, sub_44c5b0, free);
    v2 = &v4;
    v3 = &v13;
    while (1)
    {
        ptr1 = node;
        v17 = v10;
        if (!*(ptr))
            break;
        v18 = v17 - ptr1;
        n = strcspn(ptr, ",");
        v20 = memchr(ptr, 61, n);
        if (!v20)
        {
            if (n > v18)
            {
                _obstack_newchunk(v1, n);
                ptr1 = node;
            }
            memcpy(ptr1, ptr, n);
            ptr1 = node + n;
            node = ptr1;
            goto LABEL_42ce3f;
        }
        else
        {
            n1 = v20 - ptr + 1;
            if (n1 > v18)
            {
                v0 = v20;
                _obstack_newchunk(v1, n1);
                ptr1 = node;
                v20 = v0;
            }
            v0 = v20;
            memcpy(ptr1, ptr, n1);
            v22 = v0;
            n2 = n - n1;
            v24 = node + n1;
            iter = &v22->padding_0[1];
            node = v24;
            v0 = v22->padding_0[1];
            if (v22->padding_0[1])
            {
                v26 = __ctype_b_loc();
                while (*((char *)(*(v26) + v0 * 2 + 1)) & 32)
                {
                    iter = &iter->padding_0[1];
                    n2 -= 1;
                    if (!iter->padding_0[1])
                        goto LABEL_42cf7c;
                }
                if (v0 != 123 || *(&iter->padding_0[n2] - 1) != 125)
                    goto LABEL_42cf7c;
                ptr2 = sub_44c5b0(n2);
                memcpy(ptr2, &iter->padding_0[1], n2 - 2);
                *((char *)ptr2 + n2 - 2) = 0;
                if (sub_42cbe0(a0, "--pax-option", ptr2, v2))
                {
                    v28 = node;
                    if (v10 - v28 < n2)
                    {
                        _obstack_newchunk(v1, n2);
                        v28 = node;
                    }
                    memcpy(v28, iter, n2);
                    node += n2;
                }
                else
                {
                    v29 = sub_424270(v4, v5, v3);
                    len = strlen(v29);
                    v31 = node;
                    if (len > v10 - v31)
                    {
                        _obstack_newchunk(v1, len);
                        v31 = node;
                    }
                    memcpy(v31, v29, len);
                    node += len;
                }
                free(ptr2);
                ptr1 = node;
LABEL_42ce3f:
                v32 = &ptr[n];
                v33 = *(v32);
                if (v33)
                    goto LABEL_42cfb4;
                goto LABEL_42ce4e;
            }
            else
            {
LABEL_42cf7c:
                if (n2 > v10 - v24)
                {
                    _obstack_newchunk(v1, n2);
                    v24 = node;
                }
                v32 = &ptr[n];
                memcpy(v24, iter, n2);
                v33 = *(v32);
                ptr1 = n2 + node;
                node = ptr1;
                if (!v33)
                {
LABEL_42ce4e:
                    v17 = v10;
                    break;
                }
                else
                {
LABEL_42cfb4:
                    if (v10 == ptr1)
                    {
                        _obstack_newchunk(v1, 1);
                        ptr1 = node;
                        v33 = *(v32);
                    }
                    ptr = v32 + 1;
                    node = ptr1 + 1;
                    *((char *)ptr1) = v33;
                }
            }
        }
    }
    if (ptr1 == v17)
    {
        _obstack_newchunk(v1, 1);
        ptr1 = node;
    }
    node = ptr1 + 1;
    *((char *)ptr1) = 0;
    if (node == v8)
        v12 |= 2;
    v34 = node + v11 & ~(v11);
    if (v34 - v7 <= v10 - v7)
        v10 = v34;
    node = v10;
    v8 = v10;
    v35 = sub_44c7a0();
    _obstack_free(v1, 0);
    return v35;
}



// Function: set_subcommand_option @ 0x2d160
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_484ba8;
extern struct_0 *g_484d50;

long long set_subcommand_option(unsigned int a0)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x8]

    v2 = g_484ba8;
    if ((unsigned int)v2 && (unsigned int)v2 != a0)
    {
        v0 = v2;
        if (g_484d50)
            g_484d50();
        error(0, 0, dcgettext(NULL, "You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option", 5));
        sub_42c970(2); /* do not return */
    }
    g_484ba8 = a0;
    return v2;
}



// Function: option_conflict_error @ 0x2d1b0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484d50;

void option_conflict_error(unsigned long a0, unsigned long a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    if (g_484d50)
        g_484d50();
    error(0, 0, dcgettext(NULL, "'%s' cannot be used with '%s'", 5));
    sub_42c970(2); /* do not return */
}



// Function: set_old_files_option @ 0x2d200
void set_old_files_option(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_42d20d();
    return;
}



// Function: set_use_compress_program_option @ 0x2d260
void set_use_compress_program_option(unsigned long a0)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_42d267();
    return;
}



// Function: set_archive_format @ 0x2d2e0
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    char *field_10;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern struct_0 g_47f9a0;
extern unsigned int g_484ba4;
extern struct_1 *g_484d50;

int set_archive_format(unsigned long a0)
{
    char *v1;  // rdi
    struct_0 *v2;  // rbx
    struct_0 *idx;  // rbx

    v1 = "v7";
    v2 = &g_47f9a0.field_0;
    do
    {
        idx = v2;
        if (!strcmp(v1, a0))
        {
            g_484ba4 = idx->field_8;
            return g_484ba4;
        }
    } while ((v1 = idx->field_10, v2 = idx + 16, idx->field_10));
    if (g_484d50)
        g_484d50();
    sub_44aa70(a0);
    error(0, 0, dcgettext(NULL, "%s: Invalid archive format", 5));
    sub_42c970(2); /* do not return */
}



// Function: set_xattr_option @ 0x2d370
extern unsigned int g_484a98;

int set_xattr_option(unsigned int a0)
{
    int v1;  // eax
    int v2;  // eax

    if (a0 == 1)
    {
        v2 = sub_42d2e0("posix");
        g_484a98 = a0;
        return v2;
    }
    g_484a98 = a0;
    return v1;
}



// Function: parse_opt @ 0x2d3a0
typedef struct struct_0 {
    char padding_0[32];
    struct struct_1 *field_20;
} struct_0;

typedef struct struct_2 {
    struct struct_3 *field_0;
    char padding_8[8];
    char field_10;
    char field_11;
    char field_12;
    char padding_13[5];
    void* field_18;
    void* field_20;
} struct_2;

typedef struct struct_3 {
    unsigned int field_0;
} struct_3;

typedef struct struct_4 {
    struct struct_0 *field_0;
    char padding_8[32];
    struct struct_2 *field_28;
    unsigned long long field_30;
} struct_4;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned int g_466a88[4];
extern unsigned int g_466a90[4];
extern long long g_467708;
extern unsigned int g_481ae8[4];
extern unsigned int g_483b80;
extern unsigned long long g_483c60;
extern char g_4847e0;
extern char g_4847e1;
extern unsigned int g_4847e4;
extern unsigned int g_4847fc;
extern unsigned long long g_484820;
extern unsigned long long g_484830;
extern unsigned long long g_484838;
extern unsigned long long g_484a40;
extern unsigned long long g_484a48;
extern char g_484a50;
extern unsigned int g_484a54;
extern char g_484a58;
extern char g_484a59;
extern unsigned long long g_484a60;
extern char g_484a68;
extern char g_484a69;
extern char g_484a6a;
extern unsigned long long g_484a70;
extern unsigned int g_484a7c;
extern unsigned int g_484a80;
extern unsigned int g_484a84;
extern char g_484a88;
extern char g_484a89;
extern unsigned long long g_484a90;
extern unsigned int g_484a9c;
extern unsigned int g_484aa0;
extern unsigned int g_484aa4;
extern unsigned int g_484aa8;
extern char g_484aac;
extern unsigned long long g_484ab0;
extern char g_484ab8;
extern char g_484ab9;
extern char g_484aba;
extern unsigned int g_484abc;
extern char *g_484ac0;
extern unsigned long long g_484ac8;
extern char g_484ad0;
extern char g_484ad1;
extern char g_484ad2;
extern void g_484ae0;
extern unsigned int g_484af0;
extern void g_484b00;
extern unsigned long long g_484b08;
extern char g_484b10;
extern unsigned int g_484b14;
extern unsigned long long g_484b18;
extern char g_484b20;
extern unsigned int g_484b24;
extern unsigned long long g_484b28;
extern char g_484b30;
extern unsigned long long g_484b38;
extern char g_484b40;
extern unsigned long long g_484b48;
extern char g_484b50;
extern char g_484b51;
extern char g_484b52;
extern unsigned int g_484b54;
extern char *g_484b58;
extern char g_484b68;
extern char g_484b69;
extern unsigned int g_484b78;
extern char g_484b7c;
extern char g_484b84;
extern unsigned int g_484b88;
extern unsigned int g_484b8c;
extern char g_484b90;
extern char g_484b91;
extern char g_484b92;
extern unsigned long long g_484b98;
extern void g_484ba0;
extern long long g_484c18;
extern unsigned int g_484c20;
extern struct_1 *g_484d50;
extern char g_484d70;

int parse_opt(void)
{
    struct_4 *index;  // rdx
    struct_2 *idx;  // r13
    unsigned long long v14;  // rsi
    char *v15;  // rsi
    unsigned long v16;  // rax
    unsigned long long v17;  // rax
    unsigned int v19;  // eax
    unsigned long long v20;  // 4098
    unsigned long v21;  // rbx
    unsigned long v22;  // xmm0lq
    unsigned int v5;  // edi
    unsigned long idx1;  // rax
    unsigned long v24;  // rdi
    long long v26;  // rdi
    long long v27;  // rcx
    long long v28;  // r8
    long long v29;  // r9
    void* v30;  // rax
    long long v31;  // rdi
    long long v32;  // rcx
    void* v6;  // rsi
    long long v33;  // r8
    long long v34;  // r9
    unsigned long long v35;  // rdx
    unsigned long long v36;  // rax
    unsigned long long v37;  // rdx
    void* iter;  // r12
    unsigned long long v9;  // rax
    char *v10;  // rsi
    unsigned long long idx2;  // rax
    char *v12;  // rsi
    char v0;  // [bp-0x40]
    void* v1;  // [bp-0x38], Other Possible Types: char

    idx = index->field_28;
    if (v5 > 207)
    {
        switch (v5)
        {
        case 16777219:
            if (index->field_0->field_20 && index->field_0->field_20->field_0)
            {
                v35 = 32;
                v36 = 0;
                while (1)
                {
                    *((struct_2 **)(index->field_30 + v36)) = idx;
                    v36 += 8;
                    v37 = v35 + 32;
                    if (!*((long long *)((char *)index->field_0->field_20 + v35)))
                        break;
                    idx = index->field_28;
                    v35 = v37;
                }
            }
            break;
        case 16777221:
            if (idx->field_0->field_0 != 2)
            {
                if (!idx->field_0->field_0)
                {
                    error(0, 0, dcgettext(NULL, "error parsing %s", 5));
                    break;
                }
            }
            else
            {
                error(0, 0, dcgettext(NULL, "%s:%lu: location of the error", 5));
                break;
            }
            exit(64); /* do not return */
        default:
            return;
        }
    }
    else if (v5 >= 0)
    {
        switch (v5)
        {
        case 0:
            sub_426b50(v6);
            return;
        case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55:
            argp_error(index, dcgettext(NULL, "Options '-[0-7][lmh]' not supported by *this* tar", 5));
            exit(64); /* do not return */
        case 65:
            sub_42d160(2);
            return;
        case 66:
            g_484ab9 = 1;
            return;
        case 70:
            g_484b48 = v6;
            g_484b10 = 1;
            return;
        case 72:
            sub_42d2e0(v6);
            return;
        case 73:
            sub_42d260(iter);
            return;
        case 74:
            sub_42d260("xz");
            return;
        case 75:
            sub_42bd60(5, idx->field_0);
            sub_427190(iter);
            return;
        case 76:
            if (!sub_44c870(iter, &v1, 10, &v0, "bBcGgkKMmPTtw"))
            {
                v21 = v0;
                if (v1 <= iter || strchr("bBcGgkKMmPTtw", *((char *)(v1 - 1))))
                    v22 = (v21 < 0 ? (unsigned long long)(AddV(v21 >> 1 | (unsigned int)v21 & 1, v21 >> 1 | (unsigned int)v21 & 1)) : v21);
                else
                    v22 = MulV((v21 < 0 ? AddV(v21 >> 1 | (unsigned int)v21 & 1, v21 >> 1 | (unsigned int)v21 & 1) : v21), 0x4090000000000000);
                g_484b10 = 1;
                g_484a70 = v22;
                return;
            }
            if (g_484d50)
                g_484d50();
            v12 = "Invalid tape length";
            goto LABEL_42e2b3;
            goto LABEL_42e2b3;
        case 77:
            g_484b10 = 1;
            return;
        case 78:
            v20 = g_484b08;
            g_484b8c = 1;
            if (v20 - 0 >> 63 & 1)
            {
                v14 = "--after-date";
LABEL_42d8ca:
                sub_42cbe0(idx, v14, iter, &g_484b00);
                sub_42bd60(3, idx->field_0);
                return;
            }
LABEL_42e380:
            if (g_484d50)
                g_484d50();
            v10 = "More than one threshold date";
            break;
            break;
        case 79:
            g_484a6a = 1;
            return;
        case 80:
            sub_42bd60(8, idx->field_0);
            g_484b92 = 1;
            return;
        case 82:
            g_484b7c = 1;
            return;
        case 83:
            g_484a88 = 1;
            return;
        case 85:
            sub_42d200(3, idx->field_0);
            return;
        case 86:
            g_484a40 = v6;
            return;
        case 87:
            sub_42bd60(4, idx->field_0);
            g_484a50 = 1;
            return;
        case 90:
            sub_42d260("compress");
            return;
        case 97:
            idx->field_12 = 1;
            return;
        case 98:
            if (!sub_44c870(iter, 0, 10, &v1, &g_467708))
            {
                *((unsigned int *)&g_484ba0) = (unsigned int)v1;
                if (*((int *)&g_484ba0) > 0)
                {
                    g_484b98 = *((int *)&g_484ba0) * 0x200;
                    if (*((int *)&g_484ba0) == g_484b98 >> 9)
                        return;
                }
            }
            if (g_484d50)
                g_484d50();
            v12 = "Invalid blocking factor";
LABEL_42e2b3:
            dcgettext(NULL, v12, 5);
            sub_44aa70(iter);
            error(0, 0, "%s: %s");
            sub_42c970(2); /* do not return */
        case 99:
            sub_42d160(3);
            return;
        case 100:
            sub_42d160(5);
            return;
        case 102:
            idx1 = g_484830;
            v24 = g_484838;
            if (g_484830 == g_484c18)
            {
                v24 = sub_44c680(v24, &g_484c18, 8);
                g_484838 = v24;
                idx1 = g_484830;
            }
            g_484830 = idx1 + 1;
            *((void* *)(v24 + idx1 * 8)) = iter;
            return;
        case 104:
            g_484b69 = 1;
            return;
        case 105:
            g_484b51 = 1;
            return;
        case 106:
            sub_42d260("bzip2");
            return;
        case 107:
            sub_42d200(4, idx->field_0);
            return;
        case 108:
            g_484c20 = 1;
            return;
        case 109:
            g_484a68 = 1;
            return;
        case 110:
            g_4847fc = 1;
            return;
        case 111:
            idx->field_10 = 1;
            return;
        case 112:
            g_484aa4 = 1;
            return;
        case 114:
            sub_42d160(1);
            return;
        case 115:
            sub_42bd60(6, idx->field_0);
            g_484aac = 1;
            return;
        case 116:
            sub_42d160(7);
            g_484a54 = g_484a54 + 1;
            return;
        case 117:
            sub_42d160(8);
            return;
        case 118:
            g_484a54 = g_484a54 + 1;
            g_483b80 = g_483b80 | 0x583000;
            return;
        case 119:
            g_484b40 = 1;
            return;
        case 120:
            sub_42d160(6);
            return;
        case 122:
            sub_42d260("gzip");
            return;
        case 128:
            sub_42d2e0("posix");
            g_484a9c = 1;
            return;
        case 129:
            v19 = 1;
            if (v6)
                v19 = g_466a90[sub_439670("--atime-preserve")];
            g_484b88 = v19;
            return;
        case 130:
            g_484b84 = 1;
            if (v6)
            {
                idx->field_20 = v6;
                return;
            }
            break;
        case 131:
            g_484b20 = 1;
            return;
        case 132:
            if (!v6)
            {
                g_484b78 = 10;
                return;
            }
            if (*((char *)v6) == 46)
            {
                iter += 1;
                sub_410490(".");
            }
            g_484b78 = strtoul(iter, &v1, 0);
            if (!*((char *)v1))
                return;
            if (g_484d50)
                g_484d50();
            v15 = "--checkpoint value is not an integer";
LABEL_42e366:
            error(0, 0, dcgettext(NULL, v15, 5));
            sub_4163d0(); /* do not return */
        case 133:
            sub_410490(v6);
            return;
        case 134:
            g_484af0 = 2;
            return;
        case 135:
            g_4847e0 = 1;
            return;
        case 136:
            g_484b68 = 1;
            return;
        case 137:
            sub_42d160(4);
            return;
        case 138:
            g_484d70 = 1;
            return;
        case 139:
            g_484b90 = 1;
            return;
        case 140:
            v17 = sub_42bf00(iter, &g_484b58);
            if (v17 != 0xffffffffffffffff)
            {
                g_484b54 = v17;
                return;
            }
            g_484b54 = 0xffffffff;
            if (g_484b58)
            {
                sub_426aa0(g_484b58, &g_484b54);
                return;
            }
        case 141:
            sub_423a70(v6);
            return;
        case 142:
            g_484a59 = 1;
            return;
        case 143:
            g_484b52 = 1;
            return;
        case 144:
            g_484820 = v6;
            return;
        case 145:
            g_484b30 = 1;
            return;
        case 146:
            sub_42d200(6, idx->field_0);
            return;
        case 147:
            g_484b24 = strtoul(iter, &v1, 10);
            if (!*((char *)v1))
                return;
            if (g_484d50)
                g_484d50();
            v10 = "Invalid incremental level value";
            break;
            error(0, 0, dcgettext(NULL, v10, 5));
            sub_42c970(2); /* do not return */
        case 148:
            sub_42d260("lzip");
            return;
        case 149:
            sub_42d260("lzma");
            return;
        case 150:
            sub_42d260("lzop");
            return;
        case 151:
            g_484b18 = sub_4435a0(v6);
            if (g_484b18)
            {
                g_484b14 = umask(0);
                umask(g_484b14);
                return;
            }
            if (g_484d50)
                g_484d50();
            v15 = "Invalid mode given on option";
            goto LABEL_42e366;
            goto LABEL_42e366;
        case 152:
            sub_42cbe0(idx, "--mtime", v6, &g_484ae0);
            if (!g_484af0)
            {
                g_484af0 = 1;
                return;
            }
            break;
        case 153:
            if (!(g_484b08 - 0 >> 63 & 1))
                goto LABEL_42e380;
            v14 = "--newer-mtime";
            goto LABEL_42d8ca;
        case 154:
            g_484a9c = 0xffffffff;
            return;
        case 155:
            idx->field_12 = 0;
            return;
        case 156:
            g_484b20 = 0;
            return;
        case 157:
            g_4847e0 = 0;
            return;
        case 158:
            g_484a59 = 0;
            return;
        case 159:
            sub_42d200(1, idx->field_0);
            return;
        case 160:
            while (*((char *)iter))
            {
                iter += 1;
                sub_44a5b0(0);
            }
            return;
        case 161:
            g_484aa8 = 0xffffffff;
            return;
        case 162:
            g_484aa4 = 0xffffffff;
            return;
        case 163:
            g_4847fc = 0;
            return;
        case 164:
            g_484aa0 = 0xffffffff;
            return;
        case 165:
            sub_42d370(0xffffffff);
            return;
        case 166:
            g_484ad2 = 1;
            return;
        case 167:
            sub_42bd60(1, idx->field_0);
            if (!iter)
            {
                g_484b38 = 1;
                return;
            }
            else if (!sub_44c870(iter, 0, 10, &v1, &g_467708))
            {
                g_484b38 = v1;
                return;
            }
            else
            {
                if (g_484d50)
                    g_484d50();
                dcgettext(NULL, "Invalid number", 5);
                sub_44aa70(iter);
                error(0, 0, "%s: %s");
                sub_4163d0(); /* do not return */
            }
        case 168:
            sub_42d2e0("v7");
            return;
        case 169:
            g_484ad1 = 1;
            return;
        case 170:
            sub_42bd60(7, idx->field_0);
            g_484ad0 = 1;
            g_484ac8 = iter;
            return;
        case 171:
            sub_42d200(0, idx->field_0);
            return;
        case 172:
            sub_42d200(2, idx->field_0);
            return;
        case 173:
            v9 = sub_42bf00(iter, &g_484ac0);
            if (v9 != 0xffffffffffffffff)
            {
                g_484abc = v9;
                return;
            }
            g_484abc = 0xffffffff;
            if (g_484ac0)
            {
                sub_4269f0(g_484ac0, &g_484abc);
                return;
            }
        case 174:
            sub_4239a0(v6);
            return;
        case 175:
            v16 = sub_42cd70(idx, v6);
            idx->field_11 = 1;
            sub_41c010();
            free(v16);
            return;
        case 176:
            sub_42d2e0("posix");
            return;
        case 177:
            while (*((char *)iter))
            {
                iter += 1;
                sub_44a5b0(0);
            }
            return;
        case 178:
            sub_42bde0(v6);
            return;
        case 179:
            if (!sub_44c870(iter, 0, 10, &v1, "bBcGgkKMmPTtw"))
            {
                g_484b98 = v1;
                if (!((unsigned short)g_484b98 & 0x1ff))
                {
                    *((unsigned int *)&g_484ba0) = g_484b98 >> 9;
                    return;
                }
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "Record size must be a multiple of %d.", 5));
                sub_42c970(2); /* do not return */
            }
            if (g_484d50)
                g_484d50();
            v12 = "Invalid record size";
            goto LABEL_42e2b3;
            goto LABEL_42e2b3;
        case 180:
            g_484aba = 1;
            return;
        case 181:
            g_484ab8 = 1;
            return;
        case 182:
            g_484a58 = 1;
            return;
        case 183:
            g_483c60 = v6;
            return;
        case 184:
            g_484ab0 = v6;
            return;
        case 185:
            g_484aa8 = 1;
            return;
        case 186:
            sub_42d2e0("posix");
            g_484aa0 = 1;
            return;
        case 187:
            v30 = sub_42c280();
            __printf_chk(1, "%s\n", v30);
            sub_43e440(v31, v6, index, v32, v33, v34);
            free(v30);
            exit(0); /* do not return */
        case 188:
            g_484a89 = 1;
            return;
        case 189:
            sub_41fc70();
            sub_43e440(v26, v6, index, v27, v28, v29);
            exit(0); /* do not return */
        case 190:
            g_4847e1 = 1;
            return;
        case 191:
            sub_42d200(5, idx->field_0);
            return;
        case 192:
            g_4847e4 = g_481ae8[sub_439670("--sort")];
            return;
        case 193:
            idx2 = sub_439670("--hole-detection");
            g_484a88 = 1;
            g_484a7c = g_466a88[idx2];
            return;
        case 194:
            g_484a88 = 1;
            g_484a84 = strtoul(iter, &v1, 10);
            if (!*((char *)v1) || *((char *)v1) == 46 && !(g_484a80 = (unsigned int)(unsigned long long)strtoul(v1 + 1, &v1, 10), *((char *)v1)))
                return;
            if (g_484d50)
                g_484d50();
            v10 = "Invalid sparse version value";
            break;
            break;
        case 195:
            if (!sub_44c870(iter, 0, 10, &v1, &g_467708))
            {
                g_484a90 = v1;
                return;
            }
            if (g_484d50)
                g_484d50();
            v12 = "Invalid number of elements";
            goto LABEL_42e2b3;
            goto LABEL_42e2b3;
        case 196:
            g_484b84 = 1;
            idx->field_18 = v6;
            return;
        case 197:
            sub_42d160(9);
            return;
        case 198:
            if (v6)
            {
                sub_42c950(v6);
                return;
            }
            g_484a69 = 1;
            return;
        case 199:
            if (!g_484a60)
            {
                g_484a60 = v6;
                return;
            }
            if (g_484d50)
                g_484d50();
            v10 = "Only one --to-command option allowed";
            break;
            break;
        case 200:
            sub_430710(v6);
            return;
        case 201:
            g_484b91 = 1;
            return;
        case 202:
            g_484a48 = v6;
            return;
        case 203:
            sub_431470(v6);
            return;
        case 204:
            sub_42d370(1);
            return;
        case 205: case 206:
            sub_42d370(1);
            sub_431af0(iter, v5 == 206);
            return;
        case 207:
            sub_42d260("zstd");
            return;
        case 103:
            sub_42bd60(2, idx->field_0);
            g_484b28 = iter;
            g_484b8c = 1;
        case 71:
            g_484b50 = 1;
            return;
        default:
            return;
        }
    }
    else
    {
        return;
    }
    return;
}



// Function: parse_default_options @ 0x2e4b0
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern long long g_481b20;
extern struct_1 *g_484d50;
extern unsigned long long g_484dd0;

void parse_default_options(struct_0 **a0)
{
    unsigned long v9;  // rax
    char *v10;  // r12
    char v0;  // [bp-0x12c]
    unsigned int v1;  // [bp-0x128]
    char *v2;  // [bp-0x120]
    unsigned long long v3;  // [bp-0x118]
    unsigned long long v4;  // [bp-0x110]
    char v5;  // [bp-0x108], Other Possible Types: unsigned long
    unsigned long long *v6;  // [bp-0x100]
    unsigned long long v7;  // [bp-0xf8]

    v9 = getenv("TAR_OPTIONS");
    v1 = 0;
    v2 = "TAR_OPTIONS";
    v3 = 0;
    v4 = 0;
    if (!v9)
        return;
    v7 = 1;
    if (!sub_4382c0(v9, &v5, 33558086))
    {
        if (v5)
        {
            v10 = &*(a0)->field_0;
            *(v6) = g_484dd0;
            *(a0) = &v1;
            if (argp_parse(&g_481b20, (unsigned int)v5 + (unsigned int)v7, v6, 40, &v0, a0))
                abort(); /* do not return */
            *(a0) = v10;
            if (!(char)sub_42bca0())
            {
                v5 = 0;
            }
            else
            {
                if (g_484d50)
                    g_484d50();
                error(0, 0, dcgettext(NULL, "non-option arguments in %s", 5));
                sub_42c970(2); /* do not return */
            }
        }
        sub_435420(&v5);
        return;
    }
    else
    {
        if (g_484d50)
            g_484d50();
        sub_4354b0(&v5);
        error(0, 0, dcgettext(NULL, "cannot split TAR_OPTIONS: %s", 5));
        sub_4163d0(); /* do not return */
    }
}



// Function: decode_options @ 0x2e690
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
} struct_1;

typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    long long field_18;
    long long field_20;
} struct_2;

extern char g_466a60[4];
extern long long g_481b00;
extern long long g_481b20;
extern unsigned long long stdout;
extern unsigned long long stderr;
extern unsigned long long g_483f28;
extern unsigned int g_4847e4;
extern char g_4847e8;
extern unsigned int g_4847fc;
extern unsigned int g_484800;
extern unsigned int g_484804;
extern unsigned int g_484808;
extern void g_484820;
extern unsigned long long g_484828;
extern unsigned long long g_484830;
extern void* g_484838;
extern char g_484a38;
extern unsigned long long g_484a40;
extern char g_484a50;
extern unsigned int g_484a54;
extern unsigned long long g_484a60;
extern char g_484a6a;
extern unsigned long long g_484a70;
extern char g_484a78;
extern unsigned int g_484a7c;
extern unsigned int g_484a80;
extern unsigned int g_484a84;
extern char g_484a88;
extern int g_484a98;
extern int g_484a9c;
extern int g_484aa0;
extern unsigned int g_484aa8;
extern char g_484aac;
extern char g_484aba;
extern unsigned int g_484abc;
extern unsigned long long g_484ac0;
extern unsigned long long g_484ac8;
extern char g_484ad0;
extern unsigned int g_484ad4;
extern unsigned long long g_484ae0;
extern unsigned long long g_484ae8;
extern unsigned int g_484af0;
extern void g_484b00;
extern unsigned long long g_484b08;
extern char g_484b10;
extern char g_484b20;
extern unsigned int g_484b24;
extern unsigned long long g_484b28;
extern unsigned int g_484b34;
extern unsigned long long g_484b38;
extern char g_484b50;
extern unsigned int g_484b54;
extern unsigned long long g_484b58;
extern unsigned long long g_484b60;
extern char g_484b69;
extern unsigned long long g_484b70;
extern unsigned int g_484b80;
extern char g_484b84;
extern unsigned int g_484b88;
extern char g_484b91;
extern char g_484b92;
extern void g_484b98;
extern unsigned int g_484ba0;
extern unsigned int g_484ba4;
extern unsigned int g_484ba8;
extern struct_2 *g_484d50;
extern unsigned long long g_484db8;

void decode_options(int a0, struct_0 **a1)
{
    struct_0 **v17;  // r12
    unsigned long v18;  // rbp
    struct_0 **v27;  // r13
    int v28;  // eax
    struct_0 **v29;  // rax
    char *iter;  // rbx
    struct_0 **node;  // r14
    struct_0 **v32;  // r12
    char v33;  // al
    struct_0 **v34;  // r13
    struct_1 *v35;  // rax
    char *v19;  // rax
    struct_0 **j;  // rsi
    struct_0 **iter1;  // rdi
    unsigned int v39;  // ebx
    unsigned int i;  // eax
    unsigned long long v41;  // rax
    char *v42;  // rax
    char *v43;  // rsi
    unsigned int *v44;  // rbp
    unsigned long long v46;  // rbp
    long long v20;  // rdi
    unsigned long v47;  // r12
    unsigned long v49;  // rdx
    unsigned long v50;  // 4110
    unsigned int *v53;  // rbp
    void* v54;  // rax
    unsigned int v55;  // edx
    unsigned int v56;  // ecx
    long long v21;  // rsi
    unsigned long v57;  // xmm1lq
    unsigned long v58;  // rax
    unsigned long long v59;  // rdx
    unsigned int *v60;  // rbp
    void* v61;  // rbx
    unsigned long long v62;  // r12
    void* v63;  // rdx
    void* v64;  // rax
    void* v65;  // rbx
    void* v66;  // r12
    long long v22;  // rdx
    long long v23;  // rcx
    long long v24;  // r8
    long long v25;  // r9
    unsigned long v26;  // rax
    int v0;  // [bp-0xb4]
    struct_0 **v1;  // [bp-0xb0]
    char v2;  // [bp-0x9c], Other Possible Types: unsigned int
    char v3;  // [bp-0x98], Other Possible Types: unsigned int
    unsigned long long v4;  // [bp-0x90]
    unsigned long long v5;  // [bp-0x88]
    unsigned long long v6;  // [bp-0x80]
    char *v7;  // [bp-0x78]
    long long v8;  // [bp-0x70]
    unsigned short v9;  // [bp-0x68]
    char v10;  // [bp-0x66]
    unsigned long v11;  // [bp-0x60], Other Possible Types: unsigned long long
    long long v12;  // [bp-0x58]
    char v13;  // [bp-0x43]
    char v14;  // [bp-0x42]
    char v15;  // [bp-0x41]

    v17 = a1;
    v18 = a0;
    v9 = 0;
    v7 = &v3;
    v3 = 1;
    v4 = 0;
    v5 = 0;
    v6 = 0;
    v8 = 0;
    v10 = 0;
    v11 = 0;
    v12 = 0;
    sub_43da30("tar", &g_481b00);
    v11 = getenv("SIMPLE_BACKUP_SUFFIX");
    v19 = getenv("POSIXLY_CORRECT");
    g_484ba8 = 0;
    g_484ba4 = 0;
    g_484ba0 = 20;
    *((unsigned long *)&g_484b98) = 0x2800;
    g_484a38 = v19;
    v26 = sub_43eba0(v20, v21, v22, v23, v24, v25);
    g_4847e8 = 1;
    g_484b60 = v26;
    g_484a7c = 0;
    *((unsigned long *)&g_484b00) = 0x8000000000000000;
    g_484b08 = 0xffffffffffffffff;
    g_484ae0 = 0x8000000000000000;
    g_484ae8 = 0xffffffffffffffff;
    g_484ad4 = 8;
    g_484a84 = 1;
    g_484a80 = 0;
    g_4847e4 = 0;
    g_484abc = 0xffffffff;
    g_484ac0 = 0;
    g_484b54 = 0xffffffff;
    g_484b58 = 0;
    g_484b20 = 1;
    g_484b24 = 0xffffffff;
    g_4847fc = 0xffffffff;
    if ((unsigned int)v18 > 1 && v17[1]->field_0 != 45)
    {
        v13 = 45;
        v27 = v17 + 2;
        v15 = 0;
        v28 = (unsigned int)strlen(v17[1]) + (unsigned int)v18;
        v0 = v28 - 1;
        v29 = sub_44c5b0(v28 * 8);
        iter = &v17[1]->field_0;
        v1 = v29;
        node = v1 + 1;
        v32 = &v17[v18];
        *(v1) = *(v17);
        v33 = *(iter);
        v34 = v27;
        if (v33)
        {
            while (1)
            {
                v14 = v33;
                *(node) = sub_44c7a0(&v13);
                v35 = sub_42beb0(*(iter));
                if (v35 && v35->field_10)
                {
                    if (v27 >= v32)
                    {
                        if (g_484d50)
                            g_484d50();
                        error(0, 0, dcgettext(NULL, "Old option '%c' requires an argument.", 5));
                        sub_42c970(2); /* do not return */
                    }
                    iter += 1;
                    node += 2;
                    v34 = v27 + 1;
                    *((struct_0 **)((char *)node - 8)) = *(v27);
                    v33 = *(iter);
                    v27 = v34;
                    if (!v33)
                        break;
                }
                else
                {
                    v33 = iter[1];
                    iter += 1;
                    node += 1;
                    v34 = v27;
                    if (!v33)
                        break;
                }
            }
        }
        if (v34 < v32)
        {
            j = v34;
            iter1 = node;
            do
            {
                *(iter1) = *(j);
                iter1 += 1;
                j += 1;
            } while (j < v32);
            node = &node[((char *)v32 - 1 - v34 >> 3) + 1];
        }
        v17 = v1;
        v18 = v0;
        *(node) = NULL;
    }
    sub_42e4b0(&v7);
    v39 = argp_parse(&g_481b20, v18 & 0xffffffff, v17, 8, &v2, &v7);
    if (v39)
        exit(2); /* do not return */
    if ((char)v9)
    {
        if (g_484ba8 != 3)
            g_484aa8 = 0xffffffff;
        else
            sub_42d2e0("v7");
    }
    i = v2;
    if ((unsigned int)v18 > i)
    {
        do
        {
            sub_426b50(v17[i]);
            i = v2 + 1;
            v2 = i;
        } while (i < (unsigned int)v18);
    }
    if (!g_484ba4)
    {
        if (*((char *)((void*)&v9 + 1)))
            g_484ba4 = 4;
        else
            g_484ba4 = 6;
    }
    if (g_484a40 && g_484ba8 == 3 || g_484b50 || g_484b10 || g_484a88)
        sub_42e640();
    if (!g_484b38)
    {
LABEL_42e904:
        v41 = g_484830;
        if (!v41)
        {
            g_484830 = 1;
            v42 = getenv("TAPE");
            *((char **)g_484838) = v42;
            if (!*((long long *)g_484838))
                *((char **)g_484838) = "-";
            v41 = g_484830;
        }
        if (v41 <= 1 || g_484b10)
        {
            if (g_484b28)
            {
                if (!(g_484b08 - 0 >> 63 & 1))
                {
                    v44 = sub_42bcb0(2);
                    if (sub_42bd20(v44, sub_42bcb0(3)))
                        sub_42d1b0("--listed-incremental", "--newer"); /* do not return */
                    if (*(v44) == 1)
                    {
                        g_484b28 = 0;
                        goto LABEL_42e97a;
                    }
                    else
                    {
                        *((uint128_t *)&g_484b00) = (uint128_t)0;
                    }
                }
            }
            else
            {
LABEL_42e97a:
                if (g_484b24 != 0xffffffff)
                {
                    if (g_484d50)
                        g_484d50();
                    error(0, 0, dcgettext(NULL, "--level is meaningless without --listed-incremental", 5));
                }
            }
            v46 = g_484a40;
            if (g_484a40 && (g_484ba4 & 0xfffffffb) == 2)
            {
                v47 = ((unsigned int)-(g_484b10 < 1) & 18) + 81;
                if (strlen(g_484a40) > v47)
                {
                    if (g_484d50)
                    {
                        g_484d50();
                        v46 = g_484a40;
                    }
                    sub_44aa70(v46);
                    error(0, 0, dcngettext(NULL, "%s: Volume label is too long (limit is %lu byte)", "%s: Volume label is too long (limit is %lu bytes)", v47, 5));
                    sub_42c970(2); /* do not return */
                }
            }
            if (!g_484a50)
            {
                if (!g_484b70)
                    goto LABEL_42ea08;
                if (!g_484b10)
                {
                    if (!(g_466a60[4 * g_484ba8] & 4))
                    {
                        if (g_484ba8 != 2)
                            goto LABEL_42ea08;
                        if (g_484d50)
                            g_484d50();
                        v43 = "Cannot concatenate compressed archives";
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        v43 = "Cannot update compressed archives";
                    }
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    v43 = "Cannot use multi-volume compressed archives";
                }
            }
            else
            {
                if (!g_484b10)
                {
                    if (!g_484b70)
                    {
                        if (!(g_466a60[4 * g_484ba8] & 2))
                        {
                            if (!sub_42bcc0(4))
                                g_484a50 = 0;
                            else
                                sub_42d1b0("--verify", sub_42c7e0(v22)); /* do not return */
                        }
LABEL_42ea08:
                        if (g_484af0 != 2 || !(g_484ae8 - 0 >> 63 & 1))
                        {
                            if (*((char *)((void*)&v9 + 1)) && g_484ba4 != 4)
                            {
                                v49 = g_484ba8;
                                if (g_466a60[4 * g_484ba8] & 1)
                                {
                                    if (g_484a9c > 0)
                                        goto LABEL_42ea51;
                                    if (g_484aa0 <= 0)
                                        goto LABEL_42ee71;
LABEL_42ee3f:
                                    if (g_466a60[4 * v49] & 1)
                                        goto LABEL_42ea5e;
                                    if (g_484d50)
                                        g_484d50();
                                    v43 = "--selinux can be used only on POSIX archives";
                                }
                                else
                                {
                                    if (g_484d50)
                                        g_484d50();
                                    v43 = "--pax-option can be used only on POSIX archives";
                                }
                            }
                            else
                            {
                                if (g_484a9c > 0)
                                {
                                    if (g_484ba4 == 4)
                                    {
                                        if (g_484aa0 > 0)
                                            goto LABEL_42ec28;
                                        goto LABEL_42ee71;
                                    }
                                    else
                                    {
                                        if (g_466a60[4 * g_484ba8] & 1)
                                        {
LABEL_42ea51:
                                            if (g_484aa0 > 0)
                                                goto LABEL_42ea5e;
                                            goto LABEL_42ee71;
                                        }
                                        else
                                        {
                                            if (g_484d50)
                                                g_484d50();
                                            v43 = "--acls can be used only on POSIX archives";
                                        }
                                    }
                                }
                                else
                                {
                                    if (g_484aa0 > 0 && g_484ba4 != 4)
                                    {
                                        v49 = g_484ba8;
                                        goto LABEL_42ee3f;
                                    }
LABEL_42ee71:
                                    if (g_484a98 <= 0 || g_484ba4 == 4)
                                    {
LABEL_42ec28:
                                        if (!g_484a78)
                                        {
LABEL_42ea67:
                                            if (g_484aac && !(g_466a60[4 * g_484ba8] & 1))
                                                goto LABEL_42ea86;
                                        }
                                        else
                                        {
                                            if (!(g_466a60[4 * g_484ba8] & 1))
                                            {
                                                if (sub_42bcc0(5))
                                                    sub_42d1b0("--starting-file", sub_42c7e0(v22)); /* do not return */
                                                v50 = g_484aac;
                                                g_484a78 = 0;
                                                if ((char)v50)
                                                {
LABEL_42ea86:
                                                    if (sub_42bcc0(6))
                                                        sub_42d1b0("--same-order", sub_42c7e0(v24)); /* do not return */
                                                    g_484aac = 0;
                                                }
                                            }
                                        }
LABEL_42ea9f:
                                        if (g_484ad0)
                                        {
                                            if (g_484b92)
                                            {
                                                v53 = sub_42bcb0(7);
                                                if (sub_42bd20(v53, sub_42bcb0(8)))
                                                    sub_42d1b0("--one-top-level", "--absolute-names"); /* do not return */
                                                if (*(v53) != 1)
                                                {
                                                    g_484ad0 = 0;
                                                    goto LABEL_42eaf0;
                                                }
                                                else
                                                {
                                                    g_484b92 = 0;
                                                }
                                            }
                                            if (g_484ac8)
                                                goto LABEL_42eaf0;
                                            v54 = sub_43e510(*((long long *)g_484838));
                                            g_484ac8 = sub_42a280(v54);
                                            free(v54);
                                            if (g_484ac8)
                                                goto LABEL_42eaf0;
                                            if (g_484d50)
                                                g_484d50();
                                            v43 = "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR";
                                        }
                                        else
                                        {
LABEL_42eaf0:
                                            if (g_484aba)
                                                g_484b34 = 3;
                                            if (g_484b69)
                                            {
                                                v55 = 0x80900;
                                                if (g_484b88 != 2)
                                                {
                                                    v56 = 0x90900;
                                                }
                                                else
                                                {
                                                    v39 = 0x40000;
LABEL_42f08e:
                                                    v55 |= v39;
                                                    v56 = v55 | 0x10000;
                                                    v39 = (g_484b69 ^ 1) * 0x100;
                                                }
                                            }
                                            else
                                            {
                                                if (g_484b88 != 2)
                                                {
                                                    v55 = 0xa0900;
                                                    goto LABEL_42f08e;
                                                }
                                                else
                                                {
                                                    v55 = 0xe0900;
                                                    v56 = 0xf0900;
                                                    v39 = 0x100;
                                                }
                                            }
                                            g_484800 = v39;
                                            g_484808 = v55;
                                            g_484804 = v56;
                                            if (g_484ba8 != 9)
                                            {
                                                if (g_484b91)
                                                    g_484a54 = 2;
                                            }
                                            else
                                            {
                                                if (!(char)sub_42bca0())
                                                    g_484a54 = g_484a54 + 1;
                                            }
                                            if ((char)((CmpF(g_484a70, 0) & 69) >> 2) & 1 || !(CmpF(g_484a70, 0) & 64))
                                            {
                                                if (*((long long *)&g_484b98) >= 0)
                                                {
                                                    v57 = *((long long *)&g_484b98);
                                                }
                                                else
                                                {
                                                    v58 = (int)*((long long *)&g_484b98) & 1;
                                                    v59 = *((long long *)&g_484b98) >> 1;
                                                    v57 = AddV(v59 | v58, v59 | v58);
                                                }
                                                if (((CmpF(v57, g_484a70) & 69 | (CmpF(v57, g_484a70) & 69) >> 6) & 1) == 1)
                                                    goto LABEL_42eb96;
                                                if (g_484d50)
                                                    g_484d50();
                                                v43 = "Volume length cannot be less than record size";
                                            }
                                            else
                                            {
LABEL_42eb96:
                                                if (g_484aac)
                                                {
                                                    if (!g_484b28)
                                                        goto LABEL_42ebe4;
                                                    v60 = sub_42bcb0(6);
                                                    if (sub_42bd20(v60, sub_42bcb0(2)))
                                                        sub_42d1b0("--preserve-order", "--listed-incremental"); /* do not return */
                                                    if (*(v60) != 1)
                                                    {
                                                        g_484aac = 0;
                                                        goto LABEL_42ebe4;
                                                    }
                                                    else
                                                    {
                                                        g_484b28 = 0;
                                                        goto LABEL_42ebe4;
                                                    }
                                                }
                                                else
                                                {
LABEL_42ebe4:
                                                    switch (g_484ba8)
                                                    {
                                                    case 1: case 2: case 8:
                                                        v65 = g_484838;
                                                        v66 = g_484838 + g_484830 * 8;
                                                        g_484828 = g_484838;
                                                        if (g_484838 >= v66)
                                                            goto LABEL_42ef93;
                                                        do
                                                        {
                                                            if (!strcmp(*((long long *)v65), "-"))
                                                            {
                                                                if (g_484d50)
                                                                    g_484d50();
                                                                v43 = "Options '-Aru' are incompatible with '-f -'";
                                                                break;
                                                            }
                                                        } while ((v65 += 8, g_484828 = (unsigned long long)v65, v65 < v66));
                                                    case 3:
                                                        if ((char)sub_42bca0())
                                                        {
                                                            if (v10 && g_484830 && strcmp(*((long long *)g_484838), "-"))
                                                            {
                                                                sub_42a260(*((long long *)g_484838), g_484b70);
                                                                goto LABEL_42ef93;
                                                            }
                                                        }
                                                        else
                                                        {
                                                            if (g_484d50)
                                                                g_484d50();
                                                            v43 = "Cowardly refusing to create an empty archive";
                                                            break;
                                                        }
                                                    case 5: case 6: case 7: case 9:
                                                        v61 = g_484838;
                                                        v62 = g_484830;
                                                        v63 = g_484838 + g_484830 * 8;
                                                        g_484828 = g_484838;
                                                        v64 = g_484838;
                                                        if (g_484838 < v63)
                                                        {
                                                            do
                                                            {
                                                                if (!strcmp(*((long long *)v64), "-"))
                                                                {
                                                                    sub_42c9b0("-f");
                                                                    v62 = g_484830;
                                                                    v61 = g_484838;
                                                                }
                                                            } while ((v64 = (void*)(g_484828 + 8), g_484828 = (unsigned long long)v64, v64 < v61 + v62 * 8));
                                                        }
                                                        goto LABEL_42ef93;
                                                    default:
LABEL_42ef93:
                                                        if (*((long long *)&g_484820))
                                                        {
                                                            g_483f28 = fopen(*((long long *)&g_484820), "w");
                                                            if (!g_483f28)
                                                            {
                                                                sub_4327e0(*((long long *)&g_484820));
                                                                __stack_chk_fail(); /* do not return */
                                                            }
                                                        }
                                                        else
                                                        {
                                                            g_483f28 = (!g_484a6a ? stdout : stderr);
                                                        }
                                                        g_484828 = g_484838;
                                                        if (v11)
                                                            g_484db8 = sub_44c7a0(v11);
                                                        if (!g_484b84)
                                                        {
                                                            sub_4107a0();
                                                            sub_42c160(v8);
                                                            return;
                                                        }
                                                        g_484b80 = sub_43e360("--backup", v12);
                                                        if (g_484b80 && !g_484a6a && !g_484a60)
                                                        {
                                                            sub_4107a0();
                                                            sub_42c160(v8);
                                                            return;
                                                        }
                                                        g_484b84 = 0;
                                                        sub_4107a0();
                                                        sub_42c160(v8);
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    else
                                    {
                                        if (g_466a60[4 * g_484ba8] & 1)
                                        {
LABEL_42ea5e:
                                            if (!g_484a78)
                                                goto LABEL_42ea67;
                                            goto LABEL_42ea9f;
                                        }
                                        else
                                        {
                                            if (g_484d50)
                                                g_484d50();
                                            v43 = "--xattrs can be used only on POSIX archives";
                                        }
                                    }
                                }
                            }
                        }
                        else
                        {
                            if (g_484d50)
                                g_484d50();
                            v43 = "--clamp-mtime needs a date specified using --mtime";
                        }
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        v43 = "Cannot verify compressed archives";
                    }
                }
                else
                {
                    if (g_484d50)
                        g_484d50();
                    v43 = "Cannot verify multi-volume archives";
                }
            }
        }
        else
        {
            if (g_484d50)
                g_484d50();
            v43 = "Multiple archive files require '-M' option";
        }
    }
    else if (!(char)sub_42bca0())
    {
        if (g_484d50)
            g_484d50();
        v43 = "--occurrence is meaningless without a file list";
    }
    else if (!(g_466a60[4 * g_484ba8] & 16))
    {
        if (!sub_42bcc0(1))
        {
            g_484b38 = 0;
            goto LABEL_42e904;
        }
        else
        {
            sub_42d1b0("--occurrence", sub_42c7e0(v22)); /* do not return */
        }
    }
    error(0, 0, dcgettext(NULL, v43, 5));
    sub_42c970(2); /* do not return */
}



// Function: more_options @ 0x2f5b0
extern long long g_481240;

void more_options(unsigned int a0, long long a1, unsigned long a2)
{
    unsigned long v3;  // fs
    unsigned long v0;  // [bp-0x38]
    long long v1;  // [bp-0x28], Other Possible Types: unsigned long

    v0 = a2;
    *((unsigned short *)&v1) = 0;
    *((char *)&v1 + 2) = 0;
    argp_parse(&g_481240, a0, a1, 42, 0, &v0, v0, 0, v1, 0, 0, *((long long *)(40 + v3)));
    return;
}



// Function: tar_stat_init @ 0x2f630
unsigned int tar_stat_init(unsigned long long *a0)
{
    unsigned long long *v1;  // rdi
    unsigned long long *iter;  // rdi
    unsigned long long v3;  // rcx

    *(a0) = 0;
    v1 = a0 + 1;
    v1[53] = 0;
    iter = v1 & 0xfffffffffffffff8;
    for (v3 = (unsigned int)(a0 - iter) + 440 >> 3; v3; iter += 1)
    {
        v3 -= 1;
        *(iter) = 0;
    }
    return 0;
}



// Function: tar_stat_close @ 0x2f670
typedef struct DIR {
} DIR;

typedef struct struct_0 {
    long long field_0;
    char padding_8[408];
    struct DIR *field_1a0;
    int field_1a8;
} struct_0;

unsigned int tar_stat_close(struct_0 *idx)
{
    unsigned int v1;  // eax

    if (idx->field_1a0)
    {
        v1 = closedir(idx->field_1a0);
    }
    else
    {
        if (idx->field_1a8 <= 0)
        {
            idx->field_1a8 = 0;
            return 1;
        }
        v1 = close(idx->field_1a8);
    }
    idx->field_1a8 = 0;
    idx->field_1a0 = NULL;
    if (!v1)
        return 1;
    sub_424c50(idx->field_0);
    return 0;
}



// Function: tar_stat_destroy @ 0x2f6f0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[16];
    long long field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[432];
    struct struct_0 *field_1b0;
} struct_1;

unsigned int tar_stat_destroy(struct_1 *a0)
{
    unsigned long long *iter;  // rdi
    unsigned long long v2;  // rcx

    sub_42f670(a0);
    sub_41c600(*((long long *)&a0->padding_0[352]), *((long long *)&a0->padding_0[344]));
    free(*((long long *)&a0->padding_0[0]));
    free(*((long long *)&a0->padding_0[8]));
    free(*((long long *)&a0->padding_0[24]));
    free(*((long long *)&a0->padding_0[32]));
    free(*((long long *)&a0->padding_0[40]));
    free(*((long long *)&a0->padding_0[48]));
    free(*((long long *)&a0->padding_0[56]));
    free(*((long long *)&a0->padding_0[72]));
    free(*((long long *)&a0->padding_0[320]));
    free(*((long long *)&a0->padding_0[400]));
    sub_41cca0(&a0->padding_0[360]);
    sub_416ab0(a0);
    *((unsigned long *)&a0->padding_0[0]) = 0;
    a0->field_1b0 = 0;
    iter = &a0->padding_0[8] & 0xfffffffffffffff8;
    for (v2 = (unsigned int)(a0 - iter) + 440 >> 3; v2; iter += 1)
    {
        v2 -= 1;
        *(iter) = 0;
    }
    return 0;
}



// Function: tar_timespec_cmp @ 0x2f7b0
extern unsigned int g_484400;

int tar_timespec_cmp(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    if (g_484400 != 4)
    {
        if (g_484400 == 4)
            goto LABEL_42f7c3;
LABEL_42f7c2:
    }
    else
    {
        if (g_484400 != 4)
            goto LABEL_42f7c2;
LABEL_42f7c3:
    }
    return sub_44be80();
}



// Function: set_exit_status @ 0x2f7d0
extern int g_484d58;

void set_exit_status(int a0)
{
    if (g_484d58 < a0)
        g_484d58 = a0;
    return;
}



// Function: parse_xform_flags @ 0x2f7f0

unsigned int parse_xform_flags(unsigned int *a0, unsigned int a1)
{
    switch (a1)
    {
    case 72:
        *(a0) = *(a0) & 0xfffffffd;
        return 0;
    case 82:
        *(a0) = *(a0) & 0xfffffffe;
        return 0;
    case 83:
        *(a0) = *(a0) & 0xfffffffb;
        return 0;
    case 104:
        *(a0) = *(a0) | 2;
        return 0;
    case 114:
        *(a0) = *(a0) | 1;
        return 0;
    case 115:
        *(a0) = *(a0) | 4;
        return 0;
    default:
        return 1;
    }
}



// Function: add_segment @ 0x2f880
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

void add_segment(struct_2 *idx)
{
    struct_0 **v1;  // rax
    struct_0 **v2;  // rdx

    v1 = sub_44c5b0(32);
    v2 = idx->field_60;
    *(v1) = NULL;
    if (v2)
    {
        *(v2) = v1;
        idx->field_68 = idx->field_68 + 1;
        idx->field_60 = v1;
    }
    else
    {
        idx->field_68 = idx->field_68 + 1;
        idx->field_58 = v1;
        idx->field_60 = v1;
    }
    return;
}



// Function: add_case_ctl_segment @ 0x2f8c0
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

void add_case_ctl_segment(struct_2 *a0, unsigned int a1)
{
    unsigned int *idx;  // rax

    idx = (unsigned long long)sub_42f880(a0);
    idx[2] = 2;
    idx[4] = a1;
    return;
}



// Function: add_backref_segment @ 0x2f8e0
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

void add_backref_segment(struct_2 *a0, unsigned long a1)
{
    struct_0 *idx;  // rax

    idx = (unsigned long long)sub_42f880(a0);
    idx->field_8 = 1;
    idx->field_10 = a1;
    return;
}



// Function: add_char_segment @ 0x2f900
typedef struct struct_1 {
    char field_0;
    char field_1;
} struct_1;

typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    struct struct_1 *field_10;
    unsigned long long field_18;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

char * add_char_segment(struct_2 *a0, char a1)
{
    struct_0 *v1;  // rax
    struct_0 *idx;  // rbx
    char *v3;  // rax
    char *v4;  // rax

    v1 = (unsigned long long)sub_42f880(a0);
    v1->field_8 = 0;
    idx = v1;
    v3 = sub_44c5b0(2);
    idx->field_10 = v3;
    *(v3) = a1;
    v4 = &idx->field_10->field_0;
    v4[1] = 0;
    idx->field_18 = 1;
    return v4;
}



// Function: new_transform @ 0x2f940
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_484cc0;
extern unsigned long long g_484cc8;

void new_transform(void)
{
    unsigned long v1;  // rax

    v1 = sub_44c760(112);
    if (g_484cc0)
    {
        g_484cc0->field_0 = v1;
        g_484cc0 = v1;
    }
    else
    {
        g_484cc8 = v1;
        g_484cc0 = v1;
    }
    return;
}



// Function: add_literal_segment @ 0x2f990
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    void* field_10;
    char field_18;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

void* add_literal_segment(struct_2 *a0, void* a1, void* a2)
{
    unsigned long n;  // rbx
    void* v2;  // rax
    struct_0 *v3;  // rax
    struct_0 *idx;  // r12
    void* v5;  // rax
    void* v6;  // rax

    n = a2 - a1;
    if (a2 != a1)
    {
        v3 = (unsigned long long)sub_42f880(a0);
        v3->field_8 = 0;
        idx = v3;
        v5 = sub_44c5b0(n + 1);
        idx->field_10 = v5;
        memcpy(v5, a1, n);
        v6 = idx->field_10;
        *((char *)v6 + n) = 0;
        *((unsigned long *)&idx->field_18) = n;
        return v6;
    }
    return v2;
}



// Function: parse_transform_expr @ 0x2f9f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_483b60;
extern struct_0 *g_484d50;

void* parse_transform_expr(void* index)
{
    unsigned int *v12;  // rax
    char v13;  // sil
    char v22;  // al
    unsigned int v23;  // r14d
    unsigned long v24;  // rdx
    void* v25;  // rcx
    char i;  // bl
    unsigned int v27;  // r15d
    void* v28;  // rdi
    char v29;  // sil
    int v30;  // r8d
    void* v31;  // rax
    void* v14;  // r12
    void* v32;  // rax
    char *ptr;  // r13
    unsigned int v35;  // eax
    unsigned long n;  // r13
    char *ptr1;  // r12
    char *v38;  // rsi
    char *iter;  // rdx
    char v40;  // al
    char *v41;  // rax
    void* v15;  // r12
    char v42;  // dl
    unsigned long long num;  // rax
    unsigned int v44;  // esi
    char v45;  // sil
    char *v46;  // rsi
    unsigned int *v16;  // rcx
    char *v17;  // rsi
    unsigned int *idx;  // rbp
    unsigned int v19;  // eax
    unsigned int v20;  // r8d
    unsigned long long v21;  // r12
    unsigned int v0;  // [bp-0x278]
    unsigned int *v1;  // [bp-0x278]
    void* v2;  // [bp-0x270]
    void* ptr2;  // [bp-0x268]
    int v4;  // [bp-0x260]
    unsigned int v5;  // [bp-0x25c]
    char *v6;  // [bp-0x258]
    void* v7;  // [bp-0x250]
    char v8;  // [bp-0x248]
    char v9;  // [bp-0x247]
    char v10;  // [bp-0x246]

    v12 = (unsigned long long)sub_42f940();
    if (*((char *)index) == 115)
    {
        idx = v12;
        v19 = (char)index[1];
        v5 = v19;
        if (v5 && !(v20 = 2, v21 = 2, v2 = index + 2, v22 = (char)index[2], !(char)index[2]))
        {
            while (1)
            {
                v23 = v20 + 1;
                if ((char)v19 == v22)
                    break;
                if (v22 == 92)
                    v23 = v23 + 1 - (*((char *)index + v21 + 1) < 1);
                v21 = v23;
                v22 = *((char *)index + v21);
                v20 = v21;
                if (!*((char *)index + v21))
                    goto LABEL_42ffb5;
            }
            v25 = index + v23;
            i = *((char *)v25);
            ptr2 = v25;
            v27 = v23;
            if (*((char *)ptr2))
            {
                while (i != v22)
                {
                    if (i == 92)
                        v27 = v27 + 1 - (*((char *)index + v24 + 1) < 1);
                    v24 = v27 + 1;
                    i = *((char *)index + v24);
                    if (!*((char *)index + v24))
                        goto LABEL_42ffb5;
                }
                v28 = index + v24 + 1;
                idx[2] = 0;
                v7 = v28;
                idx[3] = g_483b60;
                v29 = *((char *)v28);
                v0 = 0;
                if (v29 && v29 != 59)
                {
                    while (1)
                    {
                        if (v29 == 105)
                        {
                            v31 = v7;
                            v0 |= 2;
                            v28 = v31 + 1;
                            goto LABEL_42fbdb;
                        }
                        if (v29 > 105)
                            break;
                        if (v29 <= 57 && v29 > 47)
                        {
                            v4 = v30;
                            v28 = v7;
                            idx[4] = strtoul();
                            v31 = v28 - 1;
LABEL_42fbdb:
                            v7 = v28;
                            v29 = (char)v31[1];
                            if (!v29)
                                goto LABEL_42fc30;
                            goto LABEL_42fbe9;
                        }
                        if (v29 != 103)
                            goto LABEL_42fbc2;
                        v32 = v7;
                        idx[2] = 1;
                        v28 = v32 + 1;
                        v7 = v28;
                        v29 = (char)v32[1];
                        if (!v29)
                            goto LABEL_42fc30;
LABEL_42fbe9:
                        if (v29 == 59)
                            goto LABEL_42ff30;
                    }
                    switch (v29)
                    {
                    case 120:
                        v31 = v7;
                        v0 |= 1;
                        v28 = v31 + 1;
                        goto LABEL_42fbdb;
                        break;
                    }
LABEL_42fbc2:
                    if (!sub_42f7f0(idx + 3, v29))
                    {
                        v31 = v7;
                        v28 = v31 + 1;
                        goto LABEL_42fbdb;
                    }
                    else
                    {
                        if (g_484d50)
                            g_484d50();
                        v17 = "Unknown flag in transform expression: %c";
                    }
                }
                else
                {
LABEL_42fc30:
                    if (v29 == 59)
                    {
LABEL_42ff30:
                        v7 = v28 + 1;
                    }
                    v4 = v30;
                    ptr = sub_44c5b0(v30 - 1);
                    memcpy(ptr, v2, v4 - 2);
                    *(&ptr[v21] - 2) = 0;
                    v1 = idx + 6;
                    v35 = sub_45ae70(v1, ptr, v0);
                    if (!v35)
                    {
                        if (*(ptr) == 94 || v4 != 2 && *(&ptr[v21] - 3) == 36)
                            idx[2] = 0;
                        free(ptr);
                        n = v27 - v23;
                        ptr1 = sub_44c5b0((unsigned int)n + 1);
                        memcpy(ptr1, ptr2, n);
                        ptr1[n] = 0;
                        v38 = ptr1;
                        iter = ptr1;
                        v6 = ptr1;
                        if (!*(ptr1))
                        {
                            sub_42f990(idx, v38, iter);
                            free(ptr1);
                            return v7;
                        }
                        while (1)
                        {
                            switch (v40)
                            {
                            case 92:
                                sub_42f990(idx, v38, iter);
                                v41 = v6;
                                v6 = v41 + 1;
                                v42 = v41[1];
                                switch (v42)
                                {
                                case 38:
                                    v45 = 38;
                                    break;
                                case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
                                    num = strtoul();
                                    if (*((long long *)&idx[18]) < num)
                                        goto LABEL_42ffee;
                                    sub_42f8e0(idx, num);
                                    v38 = v6;
                                    goto LABEL_42fe28;
                                case 69:
                                    v44 = 0;
                                    break;
                                case 76:
                                    v44 = 4;
                                    break;
                                case 85:
                                    v44 = 3;
                                    sub_42f8c0(idx, v44);
                                    v38 = v6 + 1;
                                    v6 = v38;
                                    goto LABEL_42fe28;
                                case 92:
                                    v45 = 92;
                                    break;
                                case 97:
                                    v45 = 7;
                                    break;
                                case 98:
                                    v45 = 8;
                                    break;
                                case 102:
                                    v45 = 12;
                                    break;
                                case 108:
                                    v44 = 2;
                                    break;
                                case 110:
                                    v45 = 10;
                                    break;
                                case 114:
                                    v45 = 13;
                                    break;
                                case 116:
                                    v45 = 9;
                                    break;
                                case 117:
                                    v44 = 1;
                                    break;
                                case 118:
                                    v45 = 11;
                                    break;
                                default:
                                    if (v42 != i)
                                    {
                                        v9 = v42;
                                        v8 = 92;
                                        sub_42f990(idx, &v8, &v10);
                                    }
                                    else
                                    {
                                        sub_42f900(idx, v5);
                                    }
                                    v38 = v6 + 1;
                                    v6 = v38;
                                    goto LABEL_42fe28;
                                }
                                sub_42f900(idx, v45);
                                v38 = v6 + 1;
                                v6 = v38;
LABEL_42fe28:
                                iter = v38;
                                if (!*(iter))
                                {
                                    sub_42f990(idx, v38, iter);
                                    free(ptr1);
                                    return v7;
                                }
                                continue;
                            case 38:
                                sub_42f990(idx, v38, iter);
                                sub_42f8e0(idx, 0);
                                v38 = v6 + 1;
                                v6 = v38;
                                iter = v38;
LABEL_42fd19:
                                if (!*(iter))
                                {
                                    sub_42f990(idx, v38, iter);
                                    free(ptr1);
                                    return v7;
                                }
                                continue;
                            default:
                                iter += 1;
                                v6 = iter;
                                goto LABEL_42fd19;
                            }
                        }
LABEL_42ffee:
                        if (g_484d50)
                            g_484d50();
                        v46 = "Invalid transform replacement: back reference out of range";
                        goto LABEL_42ffcf;
                    }
                    else
                    {
                        sub_45af90(v35, v1, &v8, 0x200);
                        if (g_484d50)
                            g_484d50();
                        error(0, 0, dcgettext(NULL, "Invalid transform expression: %s", 5));
                        sub_42c970(2); /* do not return */
                    }
                }
LABEL_42ff6e:
                error(0, 0, dcgettext(NULL, v17, 5));
                sub_42c970(2); /* do not return */
            }
        }
    }
    else if (!strncmp(index, "flags=", 6))
    {
        v13 = (char)index[6];
        v14 = index + 6;
        g_483b60 = 0;
        if (!v13)
            return index + 6;
        while (1)
        {
            v15 = v14;
            if (v13 == 59)
                return v15 + 1;
            if (sub_42f7f0(v16, v13))
                break;
            v13 = (char)v15[1];
            v14 = v15 + 1;
            if (!v13)
                return v15 + 1;
        }
        if (g_484d50)
            g_484d50();
        v17 = "Unknown transform flag: %c";
        goto LABEL_42ff6e;
    }
LABEL_42ffb5:
    if (g_484d50)
        g_484d50();
    v46 = "Invalid transform expression";
LABEL_42ffcf:
    error(0, 0, dcgettext(NULL, v46, 5));
    sub_42c970(2); /* do not return */
}



// Function: run_case_conv @ 0x30070
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern void g_484c40;
extern struct_0 *g_484cb8;

char * run_case_conv(unsigned int a0, void* a1, unsigned long n)
{
    char *ptr;  // r12
    unsigned long long *v2;  // rax
    char *v11;  // rdx
    unsigned long long *v3;  // rax
    char *v4;  // rbx
    unsigned long long *v5;  // rax
    char *i;  // rdx
    char *v7;  // rdx
    char *v8;  // rbx
    unsigned long long *v9;  // rax
    char *j;  // rdx

    ptr = g_484cb8;
    if (*((long long *)&g_484c40) < n)
    {
        *((unsigned long *)&g_484c40) = n;
        ptr = sub_44c610(ptr, n);
        g_484cb8 = ptr;
    }
    memcpy(ptr, a1, n);
    if (a0 == 3)
    {
        v8 = &ptr[n];
        if (v8 <= ptr)
            return ptr;
        v9 = __ctype_toupper_loc();
        j = ptr;
        do
        {
            v11 = j + 1;
            *(v11 - 1) = *((int *)(*(v9) + *(j) * 4));
            j = v11;
        } while (j != v8);
        return ptr;
    }
    else if (a0 <= 3)
    {
        if (a0 != 1)
        {
            if (a0 != 2)
                return ptr;
            v2 = __ctype_tolower_loc();
            *(ptr) = *((int *)(*(v2) + *(ptr) * 4));
            return ptr;
        }
        else
        {
            v3 = __ctype_toupper_loc();
            *(ptr) = *((int *)(*(v3) + *(ptr) * 4));
            return ptr;
        }
    }
    else
    {
        if (a0 != 4)
            return ptr;
        v4 = &ptr[n];
        if (v4 <= ptr)
            return ptr;
        v5 = __ctype_tolower_loc();
        i = ptr;
        do
        {
            v7 = i + 1;
            *(v7 - 1) = *((int *)(*(v5) + *(i) * 4));
            i = v7;
        } while (i != v4);
        return ptr;
    }
}



// Function: _single_transform_name_to_obstack @ 0x301b0
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    unsigned int field_10;
    char padding_14[52];
    unsigned long long field_48;
    char padding_50[8];
    void* field_58;
} struct_0;

typedef struct struct_1 {
    long long field_0;
} struct_1;

extern long long g_484c60;
extern struct_1 *g_484c78;
extern char g_484c80;

void _single_transform_name_to_obstack(struct_0 *a0, char *a1)
{
    char *ptr;  // rbp
    void* ptr1;  // r14
    unsigned int v17;  // eax
    unsigned long v18;  // r8
    char *v19;  // rdi
    unsigned long n;  // rdx
    unsigned int v21;  // eax
    void* v23;  // rdx
    unsigned long v24;  // rax
    unsigned long len;  // rax
    char *v26;  // rdi
    unsigned int v9;  // r12d
    unsigned long len1;  // rax
    char *v28;  // rdi
    unsigned long n1;  // rbx
    char *v30;  // rax
    char *v31;  // 4102
    char *ptr2;  // rbp
    unsigned long n2;  // r15
    unsigned long v12;  // rbx
    char *v13;  // rdi
    unsigned long v14;  // rax
    char *v15;  // rdi
    void* idx;  // r15
    unsigned long v0;  // [bp-0x68]
    unsigned long v1;  // [bp-0x60]
    struct_0 *v2;  // [bp-0x58]
    long long v3;  // [bp-0x50]
    unsigned int v4;  // [bp-0x44]
    unsigned long v5;  // [bp-0x40]

    ptr = a1;
    ptr1 = sub_44c5b0((a0->field_48 + 1) * 16);
    if (*(ptr))
    {
        v4 = 0;
        v9 = 0;
        v2 = &a0->padding_14[4];
        v0 = 0;
        v3 = &g_484c60;
        do
        {
            ptr2 = ptr;
            if (!sub_45b070(v2, ptr2, a0->field_48 + 1, ptr1, 0))
            {
                n2 = *((long long *)ptr1);
                v12 = (long long)ptr1[8];
                if (n2)
                {
                    v13 = g_484c78;
                    if (n2 > *((long long *)&g_484c80) - (char *)g_484c78)
                    {
                        _obstack_newchunk(v3, n2);
                        v13 = g_484c78;
                    }
                    memcpy(v13, ptr2, n2);
                    g_484c78 = (char *)g_484c78 + n2;
                }
                v14 = a0->field_10;
                v0 += 1;
                if ((unsigned int)v14 && v14 > v0)
                {
                    v15 = g_484c78;
                    if (v12 > *((long long *)&g_484c80) - (char *)g_484c78)
                    {
                        _obstack_newchunk(v3, v12);
                        v15 = g_484c78;
                    }
                    ptr = &ptr2[v12];
                    memcpy(v15, ptr2, v12);
                    g_484c78 = (char *)g_484c78 + v12;
                    continue;
                }
                else
                {
                    idx = a0->field_58;
                    if (idx)
                    {
                        do
                        {
                            v17 = (int)idx[8];
                            if (v17 != 1)
                            {
                                if (v17 == 2)
                                {
                                    v21 = (int)idx[16];
                                    if (v21 > 2)
                                    {
                                        if (v21 - 3 <= 1)
                                            v9 = v21;
                                        continue;
                                    }
                                    else if (!v21)
                                    {
                                        v9 = 0;
                                        continue;
                                    }
                                    else if (v4 && v4 - 3 > 1)
                                    {
                                        v9 = v21;
                                        continue;
                                    }
                                    else
                                    {
                                        v4 = v9;
                                        v9 = v21;
                                        continue;
                                    }
                                }
                                if (v17)
                                    continue;
                                if (!v9)
                                {
                                    v18 = (long long)idx[16];
                                }
                                else
                                {
                                    v18 = sub_430070(v9, (long long)idx[16], (long long)idx[24]);
                                    if (v9 - 1 <= 1)
                                    {
                                        v9 = v4;
                                        v4 = 0;
                                    }
                                }
                                v19 = g_484c78;
                                n = (long long)idx[24];
                                if (n <= *((long long *)&g_484c80) - (char *)g_484c78)
                                    goto LABEL_430405;
                            }
                            else
                            {
                                v23 = (long long)idx[16] * 16 + ptr1;
                                v24 = *((long long *)v23);
                                if (v24 == 0xffffffffffffffff || (long long)v23[8] == 0xffffffffffffffff)
                                    continue;
                                n = (long long)v23[8] - v24;
                                v18 = &ptr2[v24];
                                if (v9)
                                {
                                    v1 = n;
                                    v18 = sub_430070(v9, v18, n);
                                    n = v1;
                                    if (v9 - 1 <= 1)
                                    {
                                        v9 = v4;
                                        v4 = 0;
                                        n = v1;
                                    }
                                }
                                v19 = g_484c78;
                                if (n <= *((long long *)&g_484c80) - (char *)g_484c78)
                                    goto LABEL_430405;
                            }
                            v5 = v18;
                            v1 = n;
                            _obstack_newchunk(v3, n);
                            v18 = v5;
                            n = v1;
                            v19 = g_484c78;
LABEL_430405:
                            v1 = n;
                            memcpy(v19, v18, n);
                            g_484c78 = (char *)g_484c78 + v1;
                        } while ((idx = (void*)*((long long *)idx), idx));
                    }
                }
            }
            else
            {
                len = strlen(ptr2);
                v26 = g_484c78;
                v12 = len;
                if (v12 > *((long long *)&g_484c80) - (char *)g_484c78)
                {
                    _obstack_newchunk(v3, v12);
                    v26 = g_484c78;
                }
                memcpy(v26, ptr2, v12);
                g_484c78 = (char *)g_484c78 + v12;
            }
            ptr = &ptr2[v12];
            if (!a0->field_8)
            {
                len1 = strlen(ptr);
                v28 = g_484c78;
                n1 = len1;
                if (n1 > *((long long *)&g_484c80) - (char *)g_484c78)
                {
                    _obstack_newchunk(v3, n1);
                    v28 = g_484c78;
                }
                memcpy(v28, ptr, n1);
                v30 = (char *)g_484c78 + n1;
                v31 = *((long long *)&g_484c80);
                g_484c78 = v30;
                if (v31 != v30)
                {
                    g_484c78 = v30 + 1;
                    *(v30) = 0;
                    free(ptr1);
                    return;
                }
                _obstack_newchunk(v3, 1);
                v30 = g_484c78;
                g_484c78 = v30 + 1;
                *(v30) = 0;
                free(ptr1);
                return;
            }
        } while (*(ptr));
        v30 = g_484c78;
    }
    else
    {
        v30 = g_484c78;
        v3 = &g_484c60;
    }
    if (*((long long *)&g_484c80) != v30)
    {
        g_484c78 = v30 + 1;
        *(v30) = 0;
        free(ptr1);
        return;
    }
    _obstack_newchunk(v3, 1);
    v30 = g_484c78;
    g_484c78 = v30 + 1;
    *(v30) = 0;
    free(ptr1);
    return;
}



// Function: _transform_name_to_obstack @ 0x30600
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    unsigned int field_10;
    char padding_14[52];
    unsigned long long field_48;
    char padding_50[8];
    void* field_58;
} struct_0;

extern char g_484c48;
extern long long g_484c60;
extern unsigned long long g_484c68;
extern char *g_484c70;
extern unsigned long long g_484c78;
extern unsigned long long g_484c80;
extern unsigned long long g_484c90;
extern char g_484cb0;
extern struct_0 *g_484cc8;

unsigned int _transform_name_to_obstack(unsigned int a0, char *a1, char **a2)
{
    char *v1;  // r12
    struct_0 *v2;  // rbx
    unsigned int v3;  // r8d
    unsigned long long v4;  // rax
    unsigned long v5;  // rdx

    v1 = a1;
    if (!g_484c48)
    {
        _obstack_begin(&g_484c60, 0, 0, sub_44c5b0, free);
        g_484c48 = 1;
    }
    v2 = g_484cc8;
    v3 = 0;
    if (g_484cc8)
    {
        while (1)
        {
            if (!(a0 & v2->padding_c))
            {
                v2 = (struct_0 *)v2->padding_0;
                if (!v2)
                    break;
            }
            else
            {
                sub_4301b0(v2, v1);
                v1 = g_484c70;
                if (g_484c78 == v1)
                    g_484cb0 = g_484cb0 | 2;
                v3 = 1;
                v4 = g_484c78 + g_484c90 & ~(g_484c90);
                v5 = g_484c80;
                if (v4 - g_484c68 <= g_484c80 - g_484c68)
                    v5 = v4;
                g_484c78 = v5;
                v2 = (struct_0 *)v2->padding_0;
                g_484c70 = v5;
                if (!v2)
                    break;
            }
        }
    }
    *(a2) = v1;
    return v3;
}



// Function: set_transform_expr @ 0x30710
char * set_transform_expr(char *a0)
{
    char *v1;  // rax

    if (!*(a0))
        return v1;
    do
    {
        a0 = sub_42f9f0(a0);
    } while (*(a0));
    return a0;
}



// Function: transform_name_fp @ 0x30740
extern long long g_484c60;
extern char g_484c68;
extern unsigned long long g_484c70;
extern unsigned long long g_484c78;
extern char g_484c80;

unsigned int transform_name_fp(char **a0, unsigned int a1, unsigned long long *a2, unsigned long a3)
{
    unsigned int v2;  // eax
    unsigned long v3;  // r13
    unsigned long v4;  // rsi
    unsigned long v5;  // r12
    char *v0;  // [bp-0x38]

    v2 = sub_430600(a1, *(a0), &v0);
    if ((char)v2)
    {
        v3 = v0;
        v4 = v3;
        if (a2)
            v4 = a2(v3, a3);
        sub_423be0(a0, v4);
        if (*((long long *)&g_484c68) < v3 && *((long long *)&g_484c80) > v3)
        {
            g_484c70 = v3;
            g_484c78 = v3;
            return v2;
        }
        _obstack_free(&g_484c60, v3);
        return v2;
    }
    else if (!a2)
    {
        return v2;
    }
    else
    {
        v5 = v0;
        *(a0) = NULL;
        sub_423be0(a0, a2(v5, a3));
        free(v5);
        return 1;
    }
}



// Function: transform_name @ 0x30830
unsigned int transform_name(char **a0, unsigned int a1)
{
    return sub_430740(a0, a1, NULL, 0);
}



// Function: transform_program_p @ 0x30840
extern unsigned long long g_484cc8;

char transform_program_p(void)
{
    return g_484cc8;
}



// Function: dunlink_reclaim @ 0x308a0
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    void* field_10;
} struct_0;

extern unsigned long long g_484cd0;

unsigned long long dunlink_reclaim(struct_0 *a0)
{
    unsigned long long v1;  // rax

    free(a0->field_10);
    v1 = g_484cd0;
    g_484cd0 = a0;
    a0->field_0 = v1;
    return v1;
}



// Function: flush_deferred_unlinks @ 0x308c0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
    char padding_c[4];
    struct struct_1 *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned int g_481234;
extern char g_483f30;
extern unsigned int g_484580;
extern unsigned long long g_484cd8;
extern unsigned long long g_484ce0;
extern struct_0 *g_484ce8;

void* flush_deferred_unlinks(char a0)
{
    struct_0 *v2;  // rbx
    struct_0 *v3;  // r15
    struct_0 *v4;  // rbp
    char *v5;  // r13
    unsigned int v6;  // eax
    struct_0 *i;  // rbx
    struct_0 *v8;  // rbp
    char *v9;  // r12
    unsigned int v0;  // [bp-0x40]

    v2 = NULL;
    v3 = g_484ce8;
    v0 = g_484580;
    if (g_484ce8)
    {
        while (1)
        {
            v4 = v3;
            v3 = v3->field_0;
            if (!a0 && v4->field_20 >= *((long long *)&g_483f30))
                goto LABEL_430900;
            else
                goto LABEL_430924;
LABEL_430924:
            sub_424c80(v4->field_8);
            if (v4->field_18)
            {
                v5 = &v4->field_10->field_0;
                if (v4->field_8 && (!*(v5) || !strcmp(v5, ".")))
                    goto LABEL_430900;
                if (!unlinkat(g_481234, v5, 0x200))
                    goto LABEL_430990;
                v6 = *(__errno_location());
                switch (v6)
                {
                case 17: case 39:
LABEL_430900:
                    v2 = v4;
                    break;
                case 2:
LABEL_430990:
                    sub_4308a0(v4);
                    g_484cd8 = g_484cd8 - 1;
                    if (v2)
                    {
                        v2->field_0 = v3;
                        if (!v3)
                            goto LABEL_4309b5;
                        else
                            continue;
                    }
                    else
                    {
                        g_484ce8 = v3;
                        break;
                    }
                default:
                    sub_432aa0(v5);
                    goto LABEL_430990;
                }
                if (!v3)
                    break;
            }
            else if (unlinkat(g_481234, v4->field_10, 0) && *(__errno_location()) != 2)
            {
                sub_432db0(v4->field_10);
                goto LABEL_430990;
            }
        }
LABEL_4309b5:
        i = g_484ce8;
        if (!g_484ce8)
            goto LABEL_430ae8;
        if (a0)
        {
            do
            {
                v8 = i;
                i = i->field_0;
                sub_424c80(v8->field_8);
                v9 = &v8->field_10->field_0;
                if (v8->field_8 && v8->field_18 && (!*(v9) || !strcmp(v9, ".")))
                {
                    v9 = sub_424c30();
                    sub_424c80(v8->field_8 - 1);
                }
                if (unlinkat(g_481234, v9, 0x200) && *(__errno_location()) != 2)
                    sub_432aa0(v9);
                sub_4308a0(v8);
                g_484cd8 = g_484cd8 - 1;
            } while (i);
            g_484ce0 = 0;
            g_484ce8 = 0;
        }
    }
    else
    {
LABEL_430ae8:
        g_484ce0 = 0;
    }
    return sub_424c80(v0);
}



// Function: dunlink_alloc @ 0x30b00
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_484cd0;

struct_0 ** dunlink_alloc(void)
{
    struct_0 **v1;  // rax
    struct_0 **v2;  // rdx

    v1 = g_484cd0;
    if (!v1)
        return sub_44c5b0(40);
    v2 = *(v1);
    *(v1) = NULL;
    g_484cd0 = v2;
    return v1;
}



// Function: finish_deferred_unlinks @ 0x30b30
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_484cd0;

long long finish_deferred_unlinks(void)
{
    unsigned long v1;  // rax
    struct_0 **ptr;  // rbx
    struct_0 **v3;  // rbx
    unsigned long v4;  // rax

    v1 = sub_4308c0(1);
    ptr = g_484cd0;
    if (!g_484cd0)
        return v1;
    do
    {
        v3 = *(ptr);
        v4 = (unsigned long long)free(ptr);
        g_484cd0 = v3;
        ptr = v3;
    } while (ptr);
    return v4;
}



// Function: queue_deferred_unlink @ 0x30b70
typedef struct struct_0 {
    struct struct_0 *field_0;
    int field_8;
    char padding_c[4];
    struct struct_1 *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_483f30;
extern unsigned int g_484580;
extern unsigned long long g_484cd8;
extern struct_0 *g_484ce0;
extern struct_0 *g_484ce8;

struct_0 * queue_deferred_unlink(long long a0, char a1)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    struct_0 *idx;  // rbp
    char *v8;  // rax
    struct_0 *v9;  // rbx
    struct_0 *v10;  // r12
    struct_0 *v11;  // rsi
    struct_0 *v12;  // rdi
    struct_0 *v13;  // rax
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    if (g_484ce8 && g_484ce8[4].field_0 < *((long long *)&g_483f30))
        sub_4308c0(0);
    idx = sub_430b00();
    idx->field_0 = NULL;
    idx->field_8 = g_484580;
    v8 = sub_44c7a0(a0);
    idx->field_10 = v8;
    sub_423ea0(v8);
    idx->field_18 = a1;
    idx->field_20 = *((long long *)&g_483f30);
    if (a1)
    {
        if (!idx->field_10->field_0)
        {
            v9 = g_484ce8;
            if (g_484ce8)
                goto LABEL_430bf3;
        }
        else if (!strcmp(idx->field_10, ".") && !(v9 = (struct_0 *)g_484ce8, !g_484ce8))
        {
LABEL_430bf3:
            v10 = NULL;
            while (1)
            {
                if (v9->field_18 && (!v9->field_10->field_0 || !strcmp(v9->field_10, ".")) && v9->field_8 < idx->field_8)
                {
                    v11 = idx;
                    v12 = v10;
                    goto LABEL_430850;
                }
                v10 = v9;
                if (!v9->field_0)
                    break;
                v9 = v9->field_0;
            }
            goto LABEL_430850;
        }
    }
    v12 = g_484ce0;
    v11 = idx;
LABEL_430850:
    if (v12)
    {
        v11->field_0 = v12->field_0;
        v12->field_0 = v11;
        v13 = v11->field_0;
        if (!v13)
        {
            g_484cd8 = g_484cd8 + 1;
            g_484ce0 = v11;
            return v13;
        }
    }
    else
    {
        v13 = g_484ce8;
        g_484ce8 = v11;
        v11->field_0 = v13;
        if (!v13)
        {
            g_484cd8 = g_484cd8 + 1;
            g_484ce0 = v11;
            return v13;
        }
    }
    g_484cd8 = g_484cd8 + 1;
    return v13;
}



// Function: append_file @ 0x30ca0
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
    unsigned long long field_0;
} struct_0;

extern unsigned int g_481234;
extern struct_0 *g_484d50;

void append_file(long long a0)
{
    unsigned int v3;  // eax
    unsigned int v4;  // ebp
    unsigned long long v13;  // rcx
    unsigned long long v14;  // rax
    unsigned long i;  // r12
    unsigned long v7;  // r14
    unsigned long v8;  // r15
    unsigned int v9;  // edx
    unsigned int v10;  // eax
    char *v11;  // rdx
    unsigned long long *iter;  // rdi
    stat v0;  // [bp-0xe8]
    char v1;  // [bp-0x58]

    v3 = openat(g_481234, a0, 0);
    if (v3 < 0)
    {
        sub_4327c0(a0);
        return;
    }
    v4 = v3;
    if (!fstat(v3, &v0))
    {
        i = v0.st_atime;
        if (i > 0)
        {
            do
            {
                v7 = sub_40e130();
                v8 = sub_40d2b0(v7);
                if (i < v8)
                {
                    v9 = i;
                    v8 = i;
                    if ((unsigned short)v9 & 0x1ff)
                    {
                        v10 = 0x200 - (v9 & 0x1ff);
                        v11 = v7 + i;
                        if (v10 >= 8)
                        {
                            *((unsigned long long *)&v11) = 0;
                            *((unsigned long *)(&v11[v10] - 8)) = 0;
                            iter = v11 + 8 & 0xfffffffffffffff8;
                            for (v13 = (v10 + v11 - iter & 0xffffffff) >> 3 & 0xffffffff; v13; iter += 1)
                            {
                                v13 -= 1;
                                *(iter) = 0;
                            }
                        }
                        else if ((char)v10 & 4)
                        {
                            *((unsigned int *)&v11) = 0;
                            *((unsigned int *)(&v11[v10] - 4)) = 0;
                        }
                        else if (v10)
                        {
                            *(v11) = 0;
                            if ((char)v10 & 2)
                                *((unsigned short *)(&v11[v10] - 2)) = 0;
                        }
                        v8 = i;
                    }
                }
                v14 = sub_44af10(v4, v7, v8);
                if (v14 == 0xffffffffffffffff)
                    sub_4329d0(a0, v0.st_atime - i, v8); /* do not return */
                if (!v14)
                {
                    if (g_484d50)
                        g_484d50();
                    sub_443090(i, &v1);
                    sub_44aa70(a0);
                    error(0, 0, dcngettext(NULL, "%s: File shrank by %s byte", "%s: File shrank by %s bytes", i, 5));
                    sub_4163d0(); /* do not return */
                }
                i -= v14;
                sub_40d1f0(v7 + (v14 - 1 & 0xfffffffffffffe00));
            } while (i > 0);
        }
    }
    else
    {
        sub_432d30(a0);
    }
    if (close(v4))
    {
        sub_432680(a0);
        return;
    }
    return;
}



// Function: update_archive @ 0x30ef0
extern unsigned int g_46706c[4];
extern void* g_484408;
extern char g_484840;

long long update_archive(void)
{
    unsigned long long v9;  // r15
    unsigned long long v10;  // r14
    unsigned long long v11;  // r13
    unsigned long long v12;  // r12
    unsigned long long v13;  // rbx
    unsigned long v14;  // fs
    unsigned int v15;  // eax
    char *v0;  // [bp-0x100]
    char v1;  // [bp-0xd8]
    unsigned long v2;  // [bp-0x40]
    unsigned long long v3;  // [bp-0x30]
    unsigned long long v4;  // [bp-0x20]
    unsigned long long v5;  // [bp-0x18]
    unsigned long long v6;  // [bp-0x10]
    unsigned long long v7;  // [bp-0x8]

    v7 = v9;
    v6 = v10;
    v5 = v11;
    v4 = v12;
    v3 = v13;
    v2 = *((long long *)(40 + v14));
    sub_426d90();
    sub_40fb70(2);
    sub_41c560();
    v0 = &v1;
    while (1)
    {
        v15 = sub_4219e0(&g_484408, &g_484840, 0);
        if (sub_4219e0(&g_484408, &g_484840, 0) <= 5)
            break;
        sub_42f6f0(&g_484840);
    }
    goto *((void *)((long long)(g_46706c[sub_4219e0(&g_484408, &g_484840, 0) & 0xffffffff] + (char *)&g_46706c[0])));
}



// Function: utf8_init @ 0x312d0
extern unsigned long long g_483b70[4];
extern unsigned long long g_483b78;

long long utf8_init(char a0)
{
    if (g_483b70[a0] != 0xffffffffffffffff)
    {
        return g_483b70[a0];
    }
    else if (!a0)
    {
        g_483b70[0] = iconv_open(sub_4430f0(), "UTF-8");
        return g_483b70;
    }
    else
    {
        g_483b78 = iconv_open("UTF-8", sub_4430f0());
        return g_483b78;
    }
}



// Function: utf8_convert @ 0x31340
typedef struct struct_0 {
    char field_0;
} struct_0;

unsigned int utf8_convert(char a0, unsigned long a1, struct_0 **a2)
{
    long long v5;  // rax
    unsigned long v0;  // [bp-0x50]
    char *ptr;  // [bp-0x48]
    unsigned long v2;  // [bp-0x40]
    unsigned long v3;  // [bp-0x38]

    v5 = sub_4312d0(a0);
    if (!v5)
    {
        *(a2) = sub_44c7a0(a1);
        return 1;
    }
    else if (v5 != -0x1)
    {
        v2 = strlen(a1) + 1;
        v3 = v2 * 16 + 1;
        ptr = sub_44c5b0(v3);
        v0 = a1;
        if (iconv(v5, &v0, &v2, &ptr, &v3))
        {
            free(ptr);
            return 0;
        }
        *(ptr) = 0;
        *(a2) = ptr;
        return 1;
    }
    else
    {
        return 0;
    }
}



// Function: string_ascii_p @ 0x31440
unsigned int string_ascii_p(char *i)
{
    for (; *(i); i += 1)
    {
        if (*(i) < 0)
            return 0;
    }
    return 1;
}



// Function: set_warning_option @ 0x31470
extern long long g_47fa20;
extern unsigned int g_483b80;
extern unsigned int g_483ba0[4];
extern long long g_483dd8;

int set_warning_option(unsigned long a0)
{
    unsigned int v1;  // eax
    unsigned long long v2;  // rax
    unsigned long long idx;  // rax

    v1 = strcmp(a0, "none");
    if (v1)
    {
        if (strlen(a0) > 2 && !memcmp(a0, "no-", 3))
        {
            v2 = sub_439670("--warning", a0 + 3, &g_47fa20, &g_483ba0[0], 4, g_483dd8);
            v1 = ~(g_483ba0[v2]) & g_483b80;
        }
        else
        {
            idx = sub_439670("--warning", a0, &g_47fa20, &g_483ba0[0], 4, g_483dd8);
            g_483b80 = g_483b80 | g_483ba0[idx];
            return g_483b80;
        }
    }
    g_483b80 = v1;
    return v1;
}



// Function: mask_map_realloc @ 0x31550
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
} struct_0;

unsigned long long mask_map_realloc(struct_0 *idx)
{
    unsigned long long v2;  // rax
    unsigned long long v3;  // rbx
    unsigned long long v4;  // rax
    unsigned long long v0;  // [bp-0x8]

    v2 = idx->field_8;
    if (idx->field_10 != v2)
        return v2;
    v0 = v3;
    if (!v2)
        idx->field_8 = 4;
    v4 = sub_44c680(idx->field_0, &idx->field_8, 8);
    idx->field_0 = v4;
    return v4;
}



// Function: xattrs_kw_included @ 0x31600
extern unsigned long long g_484d28;

long long xattrs_kw_included(char *a0, unsigned int a1)
{
    unsigned long long v1;  // rax

    if (!g_484d28)
    {
        if ((char)a1)
            return a1;
        v1 = strncmp(a0, "user.", 5);
        return _INSERT(v1, 0, !(unsigned int)v1);
    }
}



// Function: xattrs_masked_out @ 0x31730
unsigned long long xattrs_masked_out(char *a0, char a1)
{
    if (!(char)sub_431600(a0, a1))
        return 1;
}



// Function: xattrs_acls_get @ 0x319f0
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern int g_484a9c;
extern unsigned int g_484d14;
extern struct_0 *g_484d50;

long long xattrs_acls_get(void)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax
    unsigned long v3;  // rax

    if (g_484a9c <= 0)
        return v3;
    v1 = g_484d14;
    if ((unsigned int)v1)
    {
        g_484d14 = 1;
        return v1;
    }
    if (g_484d50)
        g_484d50();
    v2 = (unsigned long long)error(0, 0, dcgettext(NULL, "POSIX ACL support is not available", 5));
    g_484d14 = 1;
    return v2;
}



// Function: xattrs_acls_set @ 0x31a70
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern int g_484a9c;
extern unsigned int g_484d10;
extern struct_0 *g_484d50;

long long xattrs_acls_set(unsigned long a0, unsigned long a1, char a2)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax
    unsigned long v3;  // rax
    unsigned long v4;  // rax

    if (g_484a9c <= 0)
    {
        return v4;
    }
    else if (a2 != 50)
    {
        v1 = g_484d10;
        if ((unsigned int)v1)
        {
            g_484d10 = 1;
            return v1;
        }
        if (g_484d50)
            g_484d50();
        v2 = (unsigned long long)error(0, 0, dcgettext(NULL, "POSIX ACL support is not available", 5));
        g_484d10 = 1;
        return v2;
    }
    else
    {
        return v3;
    }
}



// Function: xattrs_mask_add @ 0x31af0
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
} struct_0;

extern struct_0 g_484d20;
extern struct_0 g_484d38;

unsigned long long xattrs_mask_add(unsigned long a0, char a1)
{
    struct_0 *idx;  // rbx
    struct_0 *v2;  // rax
    unsigned long index;  // rax
    unsigned long v4;  // rdx

    idx = &g_484d20.field_0;
    v2 = &g_484d38.field_0;
    if (!a1)
        idx = v2;
    sub_431550(idx);
    index = idx->field_10;
    v4 = idx->field_0;
    idx->field_10 = index + 1;
    *((unsigned long *)(v4 + index * 8)) = a0;
    return index;
}



// Function: xattrs_clear_setup @ 0x31b40
extern long long g_484d20;
extern long long g_484d28;
extern long long g_484d38;
extern long long g_484d40;

void xattrs_clear_setup(void)
{
    sub_4319d0(g_484d20, g_484d28);
    sub_4319d0(g_484d38, g_484d40);
    return;
}



// Function: xattrs_xattrs_get @ 0x31b80
extern long long g_483c08;
extern long long g_483c10;
extern void g_484a98;
extern unsigned long long g_484d00;
extern unsigned long long g_484d08;

long long xattrs_xattrs_get(unsigned int a0, long long a1, void* a2, unsigned int a3)
{
    unsigned long long v4;  // r12
    unsigned long v5;  // rax
    unsigned long i;  // rbp
    long long v7;  // rax
    unsigned long len;  // rax
    unsigned long long v9;  // rdx
    long long v10;  // rcx
    unsigned int v11;  // eax
    unsigned long v12;  // rax
    unsigned long v0;  // [bp-0x58]
    long long v1;  // [bp-0x50]
    long long v2;  // [bp-0x40]

    if (*((int *)&g_484a98) <= 0)
        return *((int *)&g_484a98);
    v4 = g_484d08;
    if (!v4)
    {
        v4 = sub_44c680(0, &g_483c10, 1);
        g_484d08 = v4;
    }
    while (1)
    {
        if (!a3)
        {
            v5 = sub_438f90(a0, a1, v4, g_483c10);
            i = v5;
            if (v5 != -0x1)
            {
                v4 = g_484d08;
LABEL_431c38:
                if (!g_484d00)
                {
                    v5 = sub_44c680(0, &g_483c08, 1);
                    g_484d00 = v5;
                }
                if (i <= 0)
                    return v5;
                v7 = "fgetxattr";
                if (!a3)
                {
                    v7 = "lgetxattrat";
                    break;
                }
                else
                {
                    break;
                }
            }
        }
        else
        {
            v5 = flistxattr(a3, v4);
            i = v5;
            if (v5 != 0xffffffffffffffff)
                goto LABEL_431c38;
        }
        if (*(__errno_location()) != 0x22)
            return sub_4324f0();
        g_484d08 = sub_44c680(g_484d08, &g_483c10, 1);
        v4 = g_484d08;
    }
    v2 = v7;
    do
    {
        len = strlen(v4);
        v9 = g_484d00;
        v0 = len;
        while (1)
        {
            if (!a3)
            {
                v10 = sub_438a90(a0, a1, v4, v9);
                if (v10 != -0x1)
                    goto LABEL_431cf1;
            }
            else
            {
                v10 = fgetxattr(a3, v4);
                if (v10 != -0x1)
                {
LABEL_431cf1:
                    v1 = v10;
                    if (!(char)sub_431730(v4, 1))
                    {
                        sub_41c650(a2, v4, g_484d00, v1);
                        break;
                    }
                }
            }
            v11 = *(__errno_location());
            if (*(__errno_location()) == 0x22)
            {
                g_484d00 = sub_44c680(g_484d00, &g_483c08, 1);
                v9 = g_484d00;
            }
            else if (*(__errno_location()) != 61)
            {
                sub_4324f0(v2, a1);
                break;
            }
        }
        v12 = v0;
        v4 = v4 + v12 + 1;
        i = i - v12 - 1;
    } while (i > 0);
    return v12;
}



// Function: xattrs_selinux_get @ 0x31de0
extern void g_484aa0;

unsigned long long xattrs_selinux_get(unsigned long a0, unsigned long a1, long long a2, unsigned int a3)
{
    unsigned long long v2;  // rbx
    unsigned long long v3;  // rax
    unsigned long long v4;  // rax
    unsigned long long v5;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (*((int *)&g_484aa0) <= 0)
        return *((int *)&g_484aa0);
    v0 = v2;
    if (!a3)
    {
        v3 = sub_44b660();
        if ((unsigned int)v3 != 0xffffffff)
            return v3;
    }
    else
    {
        v4 = sub_44d390(a3, a2 + 48);
        if ((unsigned int)v4 != 0xffffffff)
            return v4;
    }
    v5 = *(__errno_location());
    if ((unsigned int)v5 == 95)
    {
        return v5;
    }
    else if ((unsigned int)v5 == 61)
    {
        return v5;
    }
    else
    {
        return sub_4324f0();
    }
}



// Function: xattrs_xattrs_set @ 0x31f20
extern void g_484a98;

unsigned long long xattrs_xattrs_set(unsigned long long *idx, long long a1, char a2, unsigned int a3)
{
    unsigned long long v1;  // rbx
    unsigned long long index;  // rax
    unsigned long len;  // rax
    unsigned long v4;  // r14
    unsigned long long v5;  // rax
    long long *v6;  // r12

    if (*((int *)&g_484a98) <= 0)
    {
        return *((int *)&g_484a98);
    }
    else if (idx[43])
    {
        v1 = 0;
        do
        {
            index = v1 * 3;
            len = strlen("SCHILY.xattr.");
            v4 = *((long long *)(idx[44] + index * 8)) + len;
            if ((a2 != 48 || (v5 = (unsigned long long)(char)((int)(long long)strcmp(v4, "security.capability")), (unsigned int)(char)((int)(long long)strcmp(v4, "security.capability")) != a3)) && !(char)sub_431730(v4, 0))
            {
                v6 = index * 8 + idx[44];
                v5 = sub_431650(a1, a2, v4, v6[1], v6[2]);
            }
        } while ((v1 += 1, idx[43] > v1));
        return v5;
    }
    else
    {
        return *((int *)&g_484a98);
    }
}



// Function: xattrs_print_char @ 0x32010
extern int g_484a54;
extern int g_484a98;
extern void g_484a9c;
extern int g_484aa0;

long long xattrs_print_char(unsigned long long *a0, void* a1)
{
    unsigned long long v1;  // r13
    unsigned long len;  // rax
    unsigned long v3;  // rax
    unsigned long v4;  // rax

    if (g_484a54 <= 1)
    {
        *((char *)a1) = 0;
        return v4;
    }
    if (g_484a98 > 0 || g_484aa0 > 0 || *((int *)&g_484a9c) > 0)
    {
        *((unsigned short *)a1) = 32;
        if (g_484a98 > 0 && a0[43])
        {
            v1 = 0;
            do
            {
                len = strlen("SCHILY.xattr.");
                if (!(char)sub_431730(len + *((long long *)(a0[44] + v1 * 24)), 0))
                {
                    *((char *)a1) = 42;
                    if (g_484aa0 <= 0)
                        goto LABEL_4320cb;
                    goto LABEL_4320c0;
                }
            } while ((v1 += 1, a0[43] > v1));
        }
        else if (g_484aa0 > 0)
        {
LABEL_4320c0:
            if (a0[6])
                *((char *)a1) = 46;
        }
        else
        {
            v3 = *((int *)&g_484a9c);
            if (*((int *)&g_484a9c) <= 0)
                return *((int *)&g_484a9c);
LABEL_4320d5:
            if (!a0[8] && !a0[10])
                return v3;
            *((char *)a1) = 43;
            return v3;
        }
    }
LABEL_4320cb:
    v3 = *((int *)&g_484a9c);
    if (*((int *)&g_484a9c) <= 0)
        return *((int *)&g_484a9c);
    goto LABEL_4320d5;
}



// Function: xattrs_print @ 0x32150
typedef struct struct_0 {
    char padding_0[48];
    char *field_30;
    long long field_38;
    long long field_40;
    long long field_48;
    unsigned long long field_50;
    char padding_58[256];
    unsigned long long field_158;
    unsigned long long field_160;
} struct_0;

extern long long g_467708;
extern unsigned long long g_483f28;
extern int g_484a54;
extern void g_484a98;
extern int g_484a9c;
extern int g_484aa0;

unsigned long long xattrs_print(struct_0 *a0)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long iter;  // rbp
    unsigned long len;  // rax
    unsigned long long v13;  // rax
    unsigned long long v14;  // rax
    unsigned long long v15;  // rax
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    if (g_484a54 <= 2)
        return v15;
    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    if (g_484aa0 > 0 && a0->field_30)
        __fprintf_chk(g_483f28, 0x1, "  s: %s\n", a0->field_30);
    if (g_484a9c > 0 && (a0->field_40 || a0->field_50))
    {
        __fprintf_chk(g_483f28, 0x1, "  a: ");
        sub_431760(&g_467708, a0->field_38, a0->field_40);
        if (a0->field_40 && a0->field_50)
            __fprintf_chk(g_483f28, 0x1, ",");
        sub_431760("default:", a0->field_48);
        __fprintf_chk(g_483f28, 0x1, "\n");
    }
    if (*((int *)&g_484a98) <= 0)
    {
        return *((int *)&g_484a98);
    }
    else if (a0->field_158)
    {
        iter = 0;
        while (1)
        {
            len = strlen("SCHILY.xattr.");
            v13 = sub_431730(len + *((long long *)(a0->field_160 + iter * 24)), 0);
            if ((char)v13)
            {
                iter += 1;
                if (a0->field_158 <= iter)
                    return v13;
            }
            else
            {
                iter += 1;
                v14 = __fprintf_chk(g_483f28, 0x1, "  x: %lu %s\n");
                if (a0->field_158 <= iter)
                    return v14;
            }
        }
    }
    else
    {
        return *((int *)&g_484a98);
    }
}



// Function: mv_begin_write @ 0xab09
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned long long g_49efa0;
extern unsigned long long g_49efb0;
extern unsigned long long g_49f060;
extern struct_1 *g_49f068;
extern char g_49f8fc;

unsigned long long * mv_begin_write(long long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long *idx;  // [bp-0x10]

    if (!g_49f8fc)
        return g_49f8fc;
    idx = sub_460b5b(48);
    if (g_49f068)
        g_49f068->field_0 = idx;
    else
        g_49f060 = idx;
    g_49f068 = idx;
    *(idx) = 0;
    idx[1] = (long long)(g_49efb0 - g_49efa0) >> 9;
    idx[2] = sub_460cc6(a0);
    idx[3] = a1;
    idx[4] = a2;
    idx[5] = 0;
    return idx;
}



// Function: bufmap_locate @ 0xabd9
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f060;

struct_0 * bufmap_locate(unsigned long long a0)
{
    struct_0 *i;  // [bp-0x10]

    for (i = g_49f060; i; i = i->field_0)
    {
        if (!i->field_0)
        {
            return i;
        }
        else if (a0 < i->field_0->field_8 * 0x200)
        {
            return i;
        }
    }
    return i;
}



// Function: bufmap_free @ 0xac2b
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    void* field_10;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[8];
    unsigned long long field_10;
} struct_1;

extern struct_0 *g_49f060;
extern unsigned long long g_49f068;

struct_1 * bufmap_free(struct_0 *a0)
{
    struct_0 *v3;  // rax
    struct_0 *ptr;  // [bp-0x18]
    struct_1 *v1;  // [bp-0x10]

    for (ptr = g_49f060; ptr && ptr != a0; ptr = v1)
    {
        v1 = ptr->field_0;
        free(ptr->field_10);
        free(ptr);
    }
    g_49f060 = ptr;
    v3 = g_49f060;
    if (!g_49f060)
    {
        v3 = g_49f060;
        g_49f068 = g_49f060;
    }
    return v3;
}



// Function: bufmap_reset @ 0xacb0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    char padding_10[24];
    unsigned long long field_28;
} struct_0;

struct_0 * bufmap_reset(struct_0 *a0, unsigned long a1)
{
    struct_0 *v2;  // rax
    struct_0 *idx;  // [bp-0x10]

    idx = a0;
    if (!idx)
        return v2;
    for (v2 = sub_40ac2b(idx); idx; idx = idx->field_0)
    {
        idx->field_8 = idx->field_8 + a1;
        idx->field_28 = 0;
        v2 = idx->field_0;
    }
    return v2;
}



// Function: buffer_write_global_xheader @ 0xad11
extern long long g_49f1e8;

int buffer_write_global_xheader(void)
{
    return sub_41d3e6(&g_49f1e8);
}



// Function: mv_begin_read @ 0xad2b
typedef struct struct_0 {
    long long field_0;
    char padding_8[128];
    unsigned int field_88;
} struct_0;

unsigned long long * mv_begin_read(struct_0 *a0)
{
    return sub_40ab09(a0->field_0, *((long long *)&a0->field_88), *((long long *)&a0->field_88));
}



// Function: mv_end @ 0xad66
extern char g_49f8fc;

long long mv_end(void)
{
    unsigned long v1;  // rax

    v1 = g_49f8fc;
    if (g_49f8fc)
        v1 = sub_40ac2b(NULL);
    return v1;
}



// Function: mv_size_left @ 0xad86
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_49f060;

struct_0 * mv_size_left(unsigned long a0)
{
    struct_0 *v1;  // rax

    v1 = g_49f060;
    if (g_49f060)
    {
        v1 = g_49f060;
        g_49f060[2].field_0 = a0;
    }
    return v1;
}



// Function: clear_read_error_count @ 0xadb0
extern unsigned int g_49f044;

void clear_read_error_count(void)
{
    g_49f044 = 0;
    return;
}



// Function: set_start_time @ 0xadc5
extern unsigned long long g_49f9e0;
extern unsigned long long g_49f9e8;
extern unsigned long long g_49f9f0;
extern unsigned long long g_49f9f8;
extern unsigned long long g_49fa00;
extern unsigned long long g_49fa08;

long long set_start_time(void)
{
    sub_4521fa(&g_49f9e0);
    g_49f9f0 = g_49f9e0;
    g_49f9f8 = g_49f9e8;
    g_49fa00 = g_49f9e0;
    g_49fa08 = g_49f9e8;
    return g_49f9e0;
}



// Function: set_volume_start_time @ 0xae17
extern unsigned long long g_49f9f0;
extern unsigned long long g_49f9f8;
extern unsigned long long g_49fa00;
extern unsigned long long g_49fa08;

long long set_volume_start_time(void)
{
    sub_4521fa(&g_49f9f0);
    g_49fa00 = g_49f9f0;
    g_49fa08 = g_49f9f8;
    return g_49f9f0;
}



// Function: compute_duration @ 0xae4d
extern unsigned long long g_49f238;
extern unsigned long long g_49fa00;
extern unsigned long long g_49fa08;

double compute_duration(void)
{
    char v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    sub_4521fa(&v0);
    g_49f238 = AddV(g_49f238, AddV(v0 - g_49fa00, DivV(v1 - g_49fa08, 0x41cdcd6500000000)));
    sub_4521fa(&g_49fa00);
    return g_49f238;
}



// Function: find_zip_program @ 0xaefd
extern unsigned int g_49c020[4];

unsigned int * find_zip_program(unsigned int *a0, unsigned int *a1)
{
    unsigned int i;  // [bp-0xc]

    for (i = *(a1); g_49c020[6 * i]; i += 1)
    {
        if (a0 == g_49c020[6 * i])
        {
            *(a1) = i + 1;
            return &g_49c020[6 * i];
        }
    }
    *(a1) = i;
    return NULL;
}



// Function: first_decompress_program @ 0xafa2
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f240;
extern unsigned long long g_49f898;

unsigned long long first_decompress_program(unsigned int *a0)
{
    struct_0 *v0;  // [bp-0x10]

    if (g_49f898)
    {
        return g_49f898;
    }
    else if (!g_49f240)
    {
        return 0;
    }
    else
    {
        *(a0) = 0;
        v0 = sub_40aefd(g_49f240, a0);
        return (!v0 ? 0 : v0->field_8);
    }
}



// Function: next_decompress_program @ 0xb012
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f240;
extern unsigned long long g_49f898;

unsigned long long next_decompress_program(unsigned int *a0)
{
    struct_0 *v0;  // [bp-0x10]

    if (!g_49f898)
    {
        v0 = sub_40aefd(g_49f240, a0);
        return (!v0 ? 0 : v0->field_8);
    }
    return 0;
}



// Function: compress_option @ 0xb065
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

unsigned long long compress_option(unsigned int a0)
{
    unsigned int v0;  // [bp-0x1c]
    struct_0 *v1;  // [bp-0x18]

    v0 = 0;
    v1 = sub_40aefd(a0, &v0);
    return (!v1 ? 0 : v1->field_10);
}



// Function: check_compressed_archive @ 0xb0cb
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char field_8;
    char padding_9[7];
    void* field_10;
} struct_0;

extern struct_0 g_49a170;
extern struct_0 g_49a230;
extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern char g_49f049;

unsigned int check_compressed_archive(char *a0)
{
    char *v0;  // [bp-0x30]
    char v1;  // [bp-0x1a]
    char v2;  // [bp-0x19]
    struct_0 *i;  // [bp-0x18]

    v0 = a0;
    if (!v0)
        v0 = &v1;
    g_49efa8 = g_49efa0;
    v2 = g_49f049;
    g_49f049 = 1;
    *(v0) = !sub_40bb4b();
    g_49f049 = v2;
    if (g_49efa0 != g_49efa8 && (!strcmp(g_49efa0 + 0x101, "ustar") || !strcmp(g_49efa0 + 0x101, "ustar  ")) && sub_425d84(g_49efa0, 1) == 1)
        return 1;
    for (i = &g_49a170.field_0; i < &g_49a230.field_0; i += 1)
    {
        if (!memcmp(g_49efa0, i->field_10, *((long long *)&i->field_8)))
            return i->field_0;
    }
    return 0;
}



// Function: guess_seekable_archive @ 0xb214
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

extern unsigned int g_49f860;
extern unsigned long long g_49f898;
extern char g_49f8fc;
extern int g_49f9cc;
extern unsigned int g_49fc14;
extern char g_49fc18;

void guess_seekable_archive(void)
{
    stat v0;  // [bp-0xa8]

    if (g_49f860 == 4)
        g_49fc18 = 0;
    if (g_49fc14 != 0xffffffff)
    {
        g_49fc18 = g_49fc14;
        return;
    }
    if (g_49f8fc != 1 && !g_49f898 && !fstat(g_49f9cc, &v0))
    {
        g_49fc18 = (v0.st_nlink & 0xf000) == 0x8000;
        return;
    }
    g_49fc18 = 0;
    return;
}



// Function: open_compressed_archive @ 0xb2cf
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern unsigned long long g_49efc0;
extern unsigned int g_49f040;
extern char g_49f048;
extern char g_49f049;
extern unsigned int g_49f240;
extern unsigned long long g_49f898;
extern char g_49f8fc;
extern long long g_49f950;
extern int g_49f9cc;
extern unsigned long long g_49fbd8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;
extern void g_49fdf0;
extern char g_49fdf8;

int open_compressed_archive(void)
{
    unsigned int v3;  // eax
    long long v6;  // rdi
    long long v7;  // rsi
    long long v8;  // rdx
    long long v9;  // rcx
    long long v10;  // r8
    long long v11;  // r9
    char v0;  // [bp-0x15]
    unsigned int v1;  // [bp-0x14]

    if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbd8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbd8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbd8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbd8))))
        v3 = sub_43f860(*((long long *)g_49fbd8), 0, 0x40000000, g_49f950);
    else
        v3 = open(*((long long *)g_49fbd8), 0, 438);
    g_49f9cc = v3;
    if (g_49f9cc == -0x1)
        return g_49f9cc;
    if (g_49f8fc != 1)
    {
        if (!g_49f898)
        {
            v1 = sub_40b0cb(&v0);
            if (v1)
            {
                if (v1 == 1)
                {
                    if (!v0)
                        return g_49f9cc;
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("This does not look like a tar archive"));
                    g_49fdd0 = 2;
                    return g_49f9cc;
                }
                g_49f240 = v1;
            }
            else
            {
                if (v0)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("This does not look like a tar archive"));
                    g_49fdd0 = 2;
                }
                sub_432970(*((long long *)g_49fbd8), 0);
                if (!g_49f898)
                    return g_49f9cc;
            }
        }
        if (g_49f9cc > 0x3fffffff)
            sub_43fe0c(g_49f9cc - 0x40000000);
        else
            close(g_49f9cc);
        g_49f048 = 0;
        g_49f040 = sub_43399b(v6, v7, v8, v9, v10, v11);
        g_49f049 = 1;
    }
    g_49efc0 = 0;
    g_49efa8 = g_49efa0;
    return g_49f9cc;
}



// Function: print_stats @ 0xb570
typedef struct FILE {
} FILE;

extern unsigned long long g_49f238;

long long print_stats(FILE *a0, char *a1)
{
    unsigned long v5;  // xmm0lq
    unsigned long v6;  // xmm1lq
    int v7;  // xmm0
    char *v8;  // rax
    unsigned int v0;  // [bp-0x550]
    unsigned int v1;  // [bp-0x54c]
    char v2;  // [bp-0x548]
    char v3;  // [bp-0x2b8]

    v0 = 0;
    v1 = 432;
    if (a1 && *(a1))
        v0 += fprintf(a0, "%s: ", gettext(a1));
    if (((CmpF(g_49f238, 0) & 69 | (char)((CmpF(g_49f238, 0) & 69) >> 6)) & 1) != 1 && !(v6 = (unsigned long)(unsigned long long)(DivV((uint128_t)v5, (uint128_t)g_49f238)), (((char)(CmpF(0x43f0000000000000, (unsigned long long)(DivV((uint128_t)v5, (uint128_t)g_49f238)))) & 69 | (char)((CmpF(0x43f0000000000000, (unsigned long long)(DivV((uint128_t)v5, (uint128_t)g_49f238))) & 69) >> 6)) & 1) == 1))
    {
        v7 = (int)(DivV(v5, g_49f238));
        v8 = sub_453f1c((CmpF(*((unsigned long long *)&v7), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v7) : *((unsigned long long *)&SubV(v7, 0x43e0000000000000)) ^ 0x8000000000000000), &v3, v1, 1, 1);
    }
    else
    {
        v8 = "?";
    }
    return fprintf(a0, "%.0f (%s, %s/s)", v5, sub_453f1c((CmpF(v5, 0x43e0000000000000) & 69 & 1 ? v5 : (unsigned long long)(SubV(v5, 0x43e0000000000000)) ^ 0x8000000000000000), &v2, v1, 1, 1), v8) + v0;
}



// Function: format_total_stats @ 0xb768
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

extern long long g_49efc0;
extern unsigned long long g_49f000;
extern unsigned long long g_49f008;
extern unsigned long long g_49f370;
extern unsigned int g_49f860;
extern unsigned long long g_49f870;

long long format_total_stats(FILE *a0, struct_0 *a1, int a2, int a3)
{
    unsigned long long v3;  // rax
    int v4;  // xmm0
    unsigned int iter;  // [bp-0x3c]
    char v1;  // [bp-0x38]

    if (g_49f860 != 8)
    {
        if (g_49f860 > 8)
            abort(); /* do not return */
        if (g_49f860 > 7)
            abort(); /* do not return */
        if (g_49f860 >= 5)
        {
            iter = sub_40b570(a0, gettext(a1->field_0));
        }
        else if (g_49f860 <= 3)
        {
            if (!g_49f860)
                abort(); /* do not return */
            goto LABEL_40b7d0;
        }
        else if (g_49f860 == 4)
        {
            iter = sub_40b570(a0, a1->field_0);
            fputc_unlocked(a2, a0);
            iter += 1;
            iter += (int)sub_40b570(a0, a1->field_8);
            fputc_unlocked(a2, a0);
            iter += 1;
            if (a1->field_10 && a1->field_10->field_0)
                iter += fprintf(a0, "%s: ", gettext(a1->field_10));
            v3 = (g_49efc0 - g_49f370) * g_49f870;
            v4 = (int)(SubV((v3 < 0 ? AddV(v3 >> 1 | (unsigned int)v3 & 1, v3 >> 1 | (unsigned int)v3 & 1) : v3), AddV(g_49f008, g_49f000)));
            iter += fprintf(a0, "%s", sub_454e5a((CmpF(*((unsigned long long *)&v4), 0x43e0000000000000) & 69 & 1 ? *((unsigned long long *)&v4) : *((unsigned long long *)&SubV(v4, 0x43e0000000000000)) ^ 0x8000000000000000), &v1, &v1));
        }
        else
        {
            abort(); /* do not return */
        }
    }
    else
    {
LABEL_40b7d0:
        iter = sub_40b570(a0, a1->field_8);
    }
    if (!a3)
        return iter;
    fputc_unlocked(a3, a0);
    iter += 1;
    return iter;
}



// Function: print_total_stats @ 0xbaa2
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 g_49a230;
extern FILE *stderr;

long long print_total_stats(void)
{
    return sub_40b768(stderr, &g_49a230.field_0, 10, 10);
}



// Function: current_block_ordinal @ 0xbacd
extern unsigned long long g_49efa0;
extern unsigned long long g_49efb0;
extern unsigned long long g_49f038;

long long current_block_ordinal(void)
{
    return g_49f038 + ((long long)(g_49efb0 - g_49efa0) >> 9);
}



// Function: reset_eof @ 0xbaf9
extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern unsigned long long g_49efb0;
extern unsigned int g_49efb8;
extern char g_49f048;
extern int g_49f868;

long long reset_eof(void)
{
    unsigned long v1;  // rax

    v1 = g_49f048;
    if ((char)v1)
    {
        g_49f048 = 0;
        g_49efb0 = g_49efa0;
        v1 = g_49f868 * 0x200 + g_49efa0;
        g_49efa8 = v1;
        g_49efb8 = 1;
    }
    return v1;
}



// Function: find_next_block @ 0xbb4b
extern char g_49efa8;
extern unsigned long long g_49efb0;
extern char g_49f048;

unsigned long long find_next_block(void)
{
    if (g_49efb0 == *((long long *)&g_49efa8))
    {
        if (g_49f048)
            return 0;
        sub_40cac8();
        if (g_49efb0 == *((long long *)&g_49efa8))
        {
            g_49f048 = 1;
            return 0;
        }
    }
    return g_49efb0;
}



// Function: set_next_block_after @ 0xbba7
extern char g_49efa8;
extern unsigned long long g_49efb0;

long long set_next_block_after(unsigned long i)
{
    for (; i >= g_49efb0; g_49efb0 = g_49efb0 + 0x200);
    if (g_49efb0 <= *((long long *)&g_49efa8))
        return *((long long *)&g_49efa8);
    abort(); /* do not return */
}



// Function: available_space_after @ 0xbbf5
extern unsigned long long g_49efa8;

long long available_space_after(unsigned long a0)
{
    return g_49efa8 - a0;
}



// Function: xclose @ 0xbc11
unsigned long long xclose(int a0)
{
    unsigned long long v1;  // rax

    v1 = close(a0);
    if (!(unsigned int)v1)
        return v1;
    return sub_43e1ff(gettext("(pipe)"));
}



// Function: init_buffer @ 0xbc48
extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern unsigned long long g_49efb0;
extern char g_49f010;
extern unsigned long long g_49f020[4];
extern int g_49f030;
extern int g_49f868;
extern long long g_49f870;

long long init_buffer(void)
{
    unsigned long long v1;  // rcx
    unsigned long v2;  // rax

    if (!g_49f020[g_49f030])
    {
        v1 = g_49f030 * 8;
        v2 = sub_42b601(&(&g_49f010)[v1], g_49f870, &(&g_49f010)[v1], v1);
        g_49f020[g_49f030] = v2;
    }
    g_49efa0 = g_49f020[g_49f030];
    g_49efb0 = g_49efa0;
    g_49efa8 = g_49f868 * 0x200 + g_49efa0;
    return g_49f868 * 0x200 + g_49efa0;
}



// Function: check_tty @ 0xbd12
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_0 *g_49fbd8;
extern struct_1 *g_49fdc8;

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

    v1 = strcmp(g_49fbd8->field_0, "-");
    if (v1)
        return v1;
    v2 = isatty(a0);
    if (!v2)
        return v2;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, (!a0 ? gettext("Refusing to read archive contents from terminal (missing -f option?)") : gettext("Refusing to write archive contents to terminal (missing -f option?)")));
    sub_4177f4(v4, v5, v6, v7, v8, v9); /* do not return */
}



// Function: _open_archive @ 0xbdb0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long stderr;
extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern unsigned int g_49efb8;
extern unsigned long long g_49efc0;
extern unsigned long long g_49efc8;
extern unsigned long long g_49efd0;
extern char g_49efd8;
extern unsigned int g_49f030;
extern unsigned int g_49f040;
extern char g_49f049;
extern unsigned long long g_49f870;
extern char g_49f884;
extern unsigned long long g_49f898;
extern char g_49f94d;
extern long long g_49f950;
extern char g_49f9b0;
extern int g_49f9cc;
extern long long g_49fa20;
extern unsigned long long g_49fbd8;
extern unsigned long long g_49fbe0;
extern char g_49fbf0;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;
extern void g_49fdf0;
extern char g_49fdf8;

long long _open_archive(unsigned int a0)
{
    long long v7;  // rdi
    long long v16;  // rdx
    long long v17;  // rcx
    long long v18;  // r8
    long long v19;  // r9
    long long v21;  // rdi
    long long v22;  // rsi
    long long v23;  // rdx
    long long v24;  // rcx
    long long v25;  // r8
    long long v8;  // rsi
    long long v26;  // r9
    long long v28;  // rdi
    long long v29;  // rsi
    long long v30;  // rdx
    long long v31;  // rcx
    long long v32;  // r8
    long long v33;  // r9
    unsigned int v35;  // eax
    long long v9;  // rdx
    unsigned int v36;  // eax
    unsigned int v37;  // eax
    long long v39;  // rdi
    long long v40;  // rsi
    long long v41;  // rdx
    long long v42;  // rcx
    long long v43;  // r8
    long long v44;  // r9
    long long v45;  // rdi
    long long v10;  // rcx
    long long v46;  // rsi
    long long v47;  // rdx
    long long v48;  // rcx
    long long v49;  // r8
    long long v50;  // r9
    unsigned long v51;  // fs
    long long v11;  // r8
    long long v12;  // r9
    long long v14;  // rdi
    long long v15;  // rsi
    char v0;  // [bp-0x2d]
    unsigned int v1;  // [bp-0x2c]
    unsigned int v2;  // [bp-0x28]
    unsigned int v3;  // [bp-0x24]
    unsigned long v4;  // [bp-0x20]

    v1 = 0;
    if (!g_49f870)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Invalid value for record_size"));
        sub_4177f4(v7, v8, v9, v10, v11, v12); /* do not return */
    }
    else if (!g_49fbe0)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("No archive name given"));
        sub_4177f4(v14, v15, v16, v17, v18, v19); /* do not return */
    }
    else
    {
        sub_439d1c(&g_49fa20);
        g_49f030 = 0;
        sub_40bc48();
        g_49efb8 = (a0 == 2 ? 0 : a0);
        sub_40bd12(g_49efb8);
        g_49f049 = g_49f94d;
        g_49efc0 = 0;
        if (g_49f898)
        {
            switch (a0)
            {
            case 2:
                abort(); /* do not return */
            default:
                if (a0 <= 2)
                {
                    if (a0)
                    {
                        if (a0 == 1)
                            g_49f040 = sub_43321b();
                    }
                    else
                    {
                        g_49f040 = sub_43399b();
                        g_49f049 = 1;
                        g_49efa8 = g_49efa0;
                    }
                }
                if (!*((long long *)&g_49fbf0) && a0 == 1 && !strcmp(*((long long *)g_49fbd8), "-"))
                    g_49efd0 = stderr;
            }
        }
        else if (!strcmp(*((long long *)g_49fbd8), "-"))
        {
            g_49f049 = 1;
            if (g_49f9b0)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Cannot verify stdin/stdout archive"));
                sub_4177f4(v21, v22, v23, v24, v25, v26); /* do not return */
            }
            else
            {
                if (a0 != 2)
                {
                    if (a0 <= 2)
                    {
                        if (a0)
                        {
                            if (a0 == 1)
                            {
                                g_49f9cc = 1;
                                if (!*((long long *)&g_49fbf0))
                                    g_49efd0 = stderr;
                            }
                        }
                        else
                        {
                            g_49f9cc = 0;
                            v2 = sub_40b0cb(&v0);
                            if (v2 != 1 && v2)
                            {
                                if (g_49fdc8)
                                    g_49fdc8();
                                sub_40b065(v2);
                                error(0, 0, gettext("Archive is compressed. Use %s option"));
                                sub_4177f4(v28, v29, v30, v31, v32, v33); /* do not return */
                            }
                            if (v0)
                            {
                                if (g_49fdc8)
                                    g_49fdc8();
                                error(0, 0, gettext("This does not look like a tar archive"));
                                g_49fdd0 = 2;
                            }
                        }
                    }
                }
                else
                {
                    g_49f9cc = 0;
                    g_49efd8 = 1;
                    g_49efa8 = g_49efa0;
                    if (!*((long long *)&g_49fbf0))
                        g_49efd0 = stderr;
                }
            }
        }
        else if (a0 == 2)
        {
            if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbd8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbd8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbd8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbd8))))
                v37 = sub_43f860(*((long long *)g_49fbd8), 66, 0x40000000, g_49f950);
            else
                v37 = open(*((long long *)g_49fbd8), 66, 438);
            g_49f9cc = v37;
            if (sub_40b0cb(NULL) > 1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Cannot update compressed archives"));
                sub_4177f4(v39, v40, v41, v42, v43, v44); /* do not return */
            }
        }
        else if (a0 <= 2)
        {
            if (!a0)
            {
                g_49f9cc = sub_40b2cf();
                if (g_49f9cc >= 0)
                    sub_40b214();
            }
            else if (a0 == 1)
            {
                if (g_49f884)
                {
                    sub_42a571(*((long long *)g_49fbd8), 1);
                    v1 = 1;
                }
                if (g_49f9b0)
                {
                    if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbd8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbd8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbd8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbd8))))
                        v35 = sub_43f860(*((long long *)g_49fbd8), 66, 0x40000000, g_49f950);
                    else
                        v35 = open(*((long long *)g_49fbd8), 66, 438);
                    g_49f9cc = v35;
                }
                else
                {
                    if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbd8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbd8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbd8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbd8))))
                        v36 = sub_43f860(*((long long *)g_49fbd8), 65, 0x40000000, g_49f950);
                    else
                        v36 = creat(*((long long *)g_49fbd8), 438);
                    g_49f9cc = v36;
                }
            }
        }
        if (g_49f9cc >= 0 && (g_49f9cc > 0x3fffffff || (char)sub_432b11() == 1))
        {
            sub_432bdc();
            sub_432b88(v45, v46, v47, v48, v49, v50);
            if (!a0)
            {
                sub_40bb4b();
            }
            else if (a0 - 1 <= 1)
            {
                g_49efc8 = 0;
            }
            return v4 - *((long long *)(40 + v51));
        }
        v3 = *(__errno_location());
        if (v1)
            sub_42a874();
        *(__errno_location()) = v3;
        sub_43e3b4(*((long long *)g_49fbd8)); /* do not return */
    }
}



// Function: _flush_write @ 0xc54f
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    char padding_10[16];
    unsigned long long field_20;
    unsigned long long field_28;
} struct_0;

extern unsigned long long g_49f008;
extern unsigned int g_49f070;
extern unsigned long long g_49f870;
extern char g_49f8fc;
extern unsigned long long g_49f990;
extern char g_49f9d0;

unsigned long long _flush_write(void)
{
    unsigned long long v0;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    sub_4106d8(1);
    if (((char)((CmpF(g_49f990, 0) & 69) >> 2) & 1 || ((char)((CmpF(g_49f990, 0) & 69) >> 6) & 1) != 1) && !(CmpF(g_49f008, g_49f990) & 1))
    {
        *(__errno_location()) = 28;
        v0 = 0;
    }
    else
    {
        v0 = (!g_49f9d0 ? sub_43301a() : g_49f870);
    }
    if (!v0)
    {
        return v0;
    }
    else if (!g_49f8fc)
    {
        return v0;
    }
    else if (!g_49f070)
    {
        idx = sub_40abd9(v0);
        if (!idx)
            return v0;
        v2 = v0 - idx->field_8 * 0x200;
        idx->field_28 = idx->field_28 + (v2 >> 9);
        if (v2 > idx->field_20)
            v2 = idx->field_20;
        idx->field_20 = idx->field_20 - v2;
        if (!idx->field_20)
        {
            v3 = idx->field_28 + idx->field_8;
            idx = idx->field_0;
        }
        else
        {
            v3 = idx->field_8;
        }
        sub_40acb0(idx, -(v3));
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: archive_write_error @ 0xc6d5
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern long long g_49f870;
extern char g_49f999;
extern struct_0 *g_49fbe8;

long long archive_write_error(long long a0)
{
    unsigned int v0;  // [bp-0xc]

    if (g_49f999)
    {
        v0 = *(__errno_location());
        sub_40baa2();
        *(__errno_location()) = v0;
    }
    sub_40d132(g_49fbe8->field_0, a0, g_49f870, a0); /* do not return */
}



// Function: archive_read_error @ 0xc729
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49f038;
extern unsigned int g_49f044;
extern struct_0 *g_49fbe8;
extern struct_0 *g_49fdc8;

long long archive_read_error(void)
{
    long long v2;  // rdi
    long long v11;  // rsi
    long long v12;  // rdx
    long long v13;  // rcx
    long long v14;  // r8
    long long v15;  // r9
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned long v8;  // rax
    long long v10;  // rdi

    sub_43e403(g_49fbe8->field_0);
    if (!g_49f038)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("At beginning of tape, quitting now"));
        sub_4177f4(v2, v3, v4, v5, v6, v7); /* do not return */
    }
    else
    {
        v8 = g_49f044;
        g_49f044 = (unsigned int)v8 + 1;
        if ((unsigned int)v8 <= 10)
            return v8;
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Too many errors, quitting"));
        sub_4177f4(v10, v11, v12, v13, v14, v15); /* do not return */
    }
}



// Function: archive_is_dev @ 0xc7e6
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
    unsigned long long field_8;
} struct_0;

extern int g_49f9cc;
extern struct_0 *g_49fbe8;

unsigned int archive_is_dev(void)
{
    unsigned int v3;  // eax
    stat v0;  // [bp-0xa8]

    if (fstat(g_49f9cc, &v0))
    {
        sub_42b444(g_49fbe8->field_0);
        return 0;
    }
    if (((unsigned short)v0.st_nlink & 0xf000) == 0x6000 || ((unsigned short)v0.st_nlink & 0xf000) == 0x2000)
        v3 = 1;
    else
        v3 = 0;
    return v3 & 1;
}



// Function: short_read @ 0xc881
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern unsigned long long g_49efc0;
extern unsigned long long g_49f038;
extern char g_49f049;
extern unsigned long long g_49f870;
extern void g_49f9cc;
extern struct_0 *g_49fdc8;

unsigned long long short_read(unsigned long long a0)
{
    unsigned long v7;  // rcx
    unsigned long long v16;  // rax
    unsigned long long v8;  // rax
    long long v10;  // rdi
    long long v11;  // rsi
    long long v12;  // rdx
    long long v13;  // rcx
    long long v14;  // r8
    long long v15;  // r9
    unsigned long long v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]
    long long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    unsigned long long v4;  // [bp-0x10]

    v0 = a0;
    v2 = v0 + g_49efa0;
    v1 = g_49f870 - v0;
    if (v1 && !((unsigned short)v1 & 0x1ff) && g_49ec84 & 0xffffff & 0x400000 && !g_49f038 && v0 && (char)sub_40c7e6())
    {
        v3 = v0 >> 9;
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, ngettext("Record size = %lu block", "Record size = %lu blocks", v3));
    }
    while (1)
    {
        if (!((unsigned short)v1 & 0x1ff) && (!v1 || !v0 || !g_49f049))
        {
            g_49efa8 = (g_49f870 - v1 & 0xfffffffffffffe00) + g_49efa0;
            v16 = g_49efc0;
            g_49efc0 = v16 + 1;
            return v16 + 1;
        }
        if (v0)
        {
            while (1)
            {
                if (*((int *)&g_49f9cc) > 0x3fffffff)
                {
                    v7 = *((int *)&g_49f9cc) - 0x40000000;
                    v8 = sub_43fe62(v7 & 0xffffffff, v2, v1, v7);
                }
                else
                {
                    v8 = sub_45ec70(*((int *)&g_49f9cc), v2, v1, v2);
                }
                v0 = v8;
                if (v0 != 0xffffffffffffffff)
                    break;
                sub_40c729();
            }
        }
        if (!v0)
        {
            g_49efa8 = (g_49f870 - v1 & 0xfffffffffffffe00) + g_49efa0;
            v16 = g_49efc0;
            g_49efc0 = v16 + 1;
            return v16 + 1;
        }
        if (g_49f049 != 1)
            break;
        v1 -= v0;
        v2 += v0;
    }
    v4 = g_49f870 - v1;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, ngettext("Unaligned block (%lu byte) in archive", "Unaligned block (%lu bytes) in archive", v4));
    sub_4177f4(v10, v11, v12, v13, v14, v15); /* do not return */
}



// Function: flush_archive @ 0xcac8
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49efa0;
extern void* g_49efa8;
extern unsigned long long g_49efb0;
extern unsigned int g_49efb8;
extern unsigned long long g_49f038;
extern struct_0 *g_49f050;
extern int g_49f868;
extern char g_49fd70;

void* flush_archive(void)
{
    unsigned long long v0;  // [bp-0x10]

    if (!g_49efb8 && g_49fd70)
    {
        g_49efb8 = 1;
        g_49fd70 = 0;
        sub_40cc3a();
        if (g_49efa8 - g_49efa0 >> 9 < g_49f868)
        {
            memset(g_49efa8, 0, (-(g_49efa8 - g_49efa0 >> 9) + g_49f868) * 0x200);
            g_49efa8 = g_49f868 * 0x200 + g_49efa0;
            return g_49efa8;
        }
    }
    v0 = g_49efb0 - g_49efa0;
    g_49f038 = g_49f038 + (g_49efa8 - g_49efa0 >> 9);
    g_49efb0 = g_49efa0;
    g_49efa8 = g_49f868 * 0x200 + g_49efa0;
    if (g_49efb8 == 2)
    {
        abort(); /* do not return */
    }
    else if (g_49efb8 > 2)
    {
        return g_49efb8;
    }
    else if (!g_49efb8)
    {
        return sub_40f662();
    }
    else if (g_49efb8 == 1)
    {
        return g_49f050(v0);
    }
    else
    {
        return g_49efb8;
    }
}



// Function: backspace_output @ 0xcc3a
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern void* g_49efa0;
extern unsigned long long g_49efa8;
extern int g_49f9cc;
extern char g_49fd78;
extern struct_0 *g_49fdc8;

void backspace_output(void)
{
    unsigned long v4;  // rcx
    char v5;  // al
    unsigned long v6;  // rcx
    char v7;  // al
    unsigned long v9;  // rcx
    unsigned long v10;  // rax
    unsigned long v0;  // [bp-0x20]
    unsigned short v1;  // [bp-0x18]
    unsigned int v2;  // [bp-0x14]

    v1 = 4;
    v2 = 1;
    if (g_49f9cc > 0x3fffffff)
    {
        v4 = g_49f9cc - 0x40000000;
        v5 = ~((unsigned int)sub_440254(v4 & 0xffffffff, 1074294017, &v1, v4)) >> 31;
    }
    else
    {
        v5 = ~((int)ioctl(g_49f9cc, 1074294017)) >> 31;
    }
    if (v5)
        return;
    if (*(__errno_location()) == 5)
    {
        if (g_49f9cc > 0x3fffffff)
        {
            v6 = g_49f9cc - 0x40000000;
            v7 = ~((unsigned int)sub_440254(v6 & 0xffffffff, 1074294017, &v1, v6)) >> 31;
        }
        else
        {
            v7 = ~((int)ioctl(g_49f9cc, 1074294017)) >> 31;
        }
        if (v7)
            return;
    }
    v0 = (g_49f9cc <= 0x3fffffff ? lseek(g_49f9cc, 0, 1) : sub_4400be(g_49f9cc - 0x40000000, 0, 1));
    v0 -= g_49efa8 - g_49efa0;
    if (v0 - 0 >> 63 & 1)
        v0 = 0;
    if (g_49f9cc > 0x3fffffff)
    {
        v9 = g_49f9cc - 0x40000000;
        v10 = sub_4400be(v9 & 0xffffffff, v0, 0, v9);
    }
    else
    {
        v10 = lseek(g_49f9cc, v0, 0);
    }
    if (v10 == v0)
        return;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Cannot backspace archive file; it may be unreadable without -i"));
    if (g_49efa0 == *((long long *)&g_49fd78))
        return;
    memset(g_49efa0, 0, *((long long *)&g_49fd78) - g_49efa0);
    return;
}



// Function: seek_archive @ 0xce79
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49efa0;
extern unsigned long long g_49efa8;
extern unsigned long long g_49efb0;
extern unsigned long long g_49efc0;
extern unsigned long long g_49f038;
extern int g_49f868;
extern unsigned long long g_49f870;
extern int g_49f9cc;
extern struct_0 *g_49fdc8;

long long seek_archive(unsigned long long a0)
{
    long long v8;  // rdi
    long long v9;  // rsi
    long long v10;  // rdx
    long long v11;  // rcx
    long long v12;  // r8
    long long v13;  // r9
    long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    long long v2;  // [bp-0x20]
    long long v3;  // [bp-0x18], Other Possible Types: unsigned long
    unsigned long v4;  // [bp-0x10]

    v0 = sub_40bacd();
    v1 = (-((long long)(g_49efb0 - g_49efa0) >> 9) + g_49f868) * 0x200;
    if (a0 <= v1)
        return 0;
    v2 = (a0 - v1) / g_49f870;
    if (!v2)
        return 0;
    v3 = (g_49f9cc <= 0x3fffffff ? lseek(g_49f9cc, v2 * g_49f870, 1) : sub_4400be(g_49f9cc - 0x40000000, v2 * g_49f870, 1, v2 * g_49f870));
    if (v3 - 0 >> 63 & 1)
    {
        return v3;
    }
    else if (v3 % g_49f870)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("rmtlseek not stopped at a record boundary"));
        sub_4177f4(v8, v9, v10, v11, v12, v13); /* do not return */
    }
    else
    {
        v3 = (long long)((v3 < 0 ? v3 + 0x1ff : v3)) >> 9;
        v4 = v3 - v0;
        g_49efc0 = g_49efc0 + v4 / g_49f868;
        g_49f038 = v3 - g_49f868;
        g_49efb0 = g_49efa8;
        return v4;
    }
}



// Function: close_archive @ 0xd046
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    void* field_10;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[8];
    unsigned long long field_10;
} struct_1;

extern char g_49efa0;
extern unsigned long long g_49efb0;
extern unsigned int g_49efb8;
extern void* g_49f010;
extern void* g_49f018;
extern unsigned int g_49f040;
extern char g_49f048;
extern char g_49f9b0;
extern int g_49f9cc;
extern long long g_49fa20;
extern struct_0 *g_49fbe8;
extern char g_49fd70;

struct_1 * close_archive(void)
{
    if (g_49fd70 || g_49efb8 == 1)
    {
        do
        {
            sub_40cac8();
        } while (g_49efb0 > *((long long *)&g_49efa0));
    }
    sub_40ae4d();
    if (g_49f9b0)
        sub_411a1c();
    if ((g_49f9cc <= 0x3fffffff ? close(g_49f9cc) : sub_43fe0c(g_49f9cc - 0x40000000)))
        sub_43e1ff(g_49fbe8->field_0);
    sub_432ca8(g_49f040, g_49f048, g_49f048);
    sub_439d1c(&g_49fa20);
    free(g_49f010);
    free(g_49f018);
    return sub_40ac2b(NULL);
}



// Function: write_fatal_details @ 0xd132
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f040;
extern int g_49f9cc;
extern struct_0 *g_49fbe8;

int write_fatal_details(long long a0, long long a1, long long a2)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9

    sub_43eb4b(a0, a1, a2, a1);
    if ((g_49f9cc <= 0x3fffffff ? close(g_49f9cc) : sub_43fe0c(g_49f9cc - 0x40000000)))
        sub_43e1ff(g_49fbe8->field_0);
    sub_432ca8(g_49f040, 0);
    sub_4177f4(v2, v3, v4, v5, v6, v7); /* do not return */
}



// Function: init_volume_number @ 0xd1c6
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern int g_49c014;
extern void g_49f9b8;
extern struct_0 *g_49fdc8;

unsigned long long init_volume_number(void)
{
    unsigned long long v2;  // rax
    long long v4;  // rdi
    long long v5;  // rsi
    long long v6;  // rdx
    long long v7;  // rcx
    long long v8;  // r8
    long long v9;  // r9
    unsigned long long v10;  // rax
    FILE *fp;  // [bp-0x20]

    fp = fopen(*((long long *)&g_49f9b8), "r");
    if (fp)
    {
        if (__isoc99_fscanf(fp, "%d", &g_49c014) == 1 && g_49c014 >= 0)
        {
            if (ferror_unlocked(fp))
                sub_43e403(*((long long *)&g_49f9b8));
            v2 = fclose(fp);
            if (!(unsigned int)v2)
                return v2;
            return sub_43e1ff(*((long long *)&g_49f9b8));
        }
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(*((long long *)&g_49f9b8));
        error(0, 0, gettext("%s: contains invalid volume number"));
        sub_4177f4(v4, v5, v6, v7, v8, v9); /* do not return */
    }
    else
    {
        v10 = *(__errno_location());
        if ((unsigned int)v10 == 2)
            return v10;
        return sub_43e38b(*((long long *)&g_49f9b8));
    }
}



// Function: closeout_volume_number @ 0xd2e3
typedef struct FILE {
} FILE;

extern int g_49c014;
extern void g_49f9b8;

unsigned long long closeout_volume_number(void)
{
    unsigned long long v2;  // rax
    FILE *fp;  // [bp-0x10]

    fp = fopen(*((long long *)&g_49f9b8), "w");
    if (!fp)
        return sub_43e38b(*((long long *)&g_49f9b8));
    fprintf(fp, "%d\n", g_49c014);
    if (ferror_unlocked(fp))
        sub_43eb22(*((long long *)&g_49f9b8));
    v2 = fclose(fp);
    if (!(unsigned int)v2)
        return v2;
    return sub_43e1ff(*((long long *)&g_49f9b8));
}



// Function: increase_volume_number @ 0xd386
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c010;
extern int g_49c014;
extern struct_0 *g_49fdc8;

unsigned int increase_volume_number(void)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned int v8;  // eax

    g_49c014 = g_49c014 + 1;
    if (g_49c014 >= 0)
    {
        v8 = g_49c010;
        g_49c010 = v8 + 1;
        return v8 + 1;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Volume number overflow"));
    sub_4177f4(v2, v3, v4, v5, v6, v7); /* do not return */
}



// Function: change_tape_menu @ 0xd3f9
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern FILE *stderr;
extern unsigned int g_49f860;
extern char g_49f9a9;
extern struct_1 *g_49fbe8;
extern struct_1 *g_49fdc8;

long long change_tape_menu(FILE *a0)
{
    char *v7;  // rax
    unsigned int v8;  // eax
    unsigned long v19;  // fs
    char *v9;  // rax
    char *v10;  // rax
    char *v11;  // rax
    unsigned long long v12;  // rax
    char *v13;  // rax
    char *v14;  // rax
    char v0;  // [bp-0x41]
    struct_0 *v1;  // [bp-0x40]
    unsigned long v2;  // [bp-0x38]
    struct_0 *v3;  // [bp-0x30]
    struct_0 *iter;  // [bp-0x28]
    unsigned long v5;  // [bp-0x20]

    v1 = NULL;
    v2 = 0;
    v0 = 0;
    while (1)
    {
        if (v0 == 1)
        {
            free(v1);
            return v5 - *((long long *)(40 + v19));
        }
        fputc_unlocked(7, stderr);
        sub_45e84e(g_49fbe8->field_0);
        v7 = gettext("Prepare volume #%d for %s and hit return: ");
        fprintf(stderr, v7);
        fflush_unlocked(stderr);
        if (getline(&v1, &v2, a0) <= 0)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("EOF where user reply was expected"));
            if (g_49f860 != 6 && g_49f860 != 7 && g_49f860 != 5)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("WARNING: Archive is incomplete"));
                break;
            }
        }
        else
        {
            if (v1->field_0 == 10)
            {
                free(v1);
                return v5 - *((long long *)(40 + v19));
            }
            if (v1->field_0 == 121)
            {
                free(v1);
                return v5 - *((long long *)(40 + v19));
            }
            if (v1->field_0 == 89)
            {
                free(v1);
                return v5 - *((long long *)(40 + v19));
            }
            switch (v8)
            {
            case 113:
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("No new volume; exiting.\n"));
                if (g_49f860 != 6 && g_49f860 != 7 && g_49f860 != 5)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("WARNING: Archive is incomplete"));
                }
                sub_4177f4(); /* do not return */
            case 110:
                for (v3 = &v1->field_1; v3->field_0 == 32 || v3->field_0 == 9; v3 = &v3->field_1);
                for (iter = v3; iter->field_0 && iter->field_0 != 10; iter = &iter->field_1);
                iter->field_0 = 0;
                if (v3->field_0)
                {
                    v12 = sub_460cc6(v3);
                    g_49fbe8->field_0 = v12;
                    v0 = 1;
                    break;
                }
                else
                {
                    v13 = gettext("File name not specified. Try again.\n");
                    fprintf(stderr, "%s", v13);
                    break;
                }
            case 33:
                if (g_49f9a9 != 1)
                {
                    sub_432df0();
                    continue;
                }
                break;
            case 63:
                v9 = gettext(" n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n");
                fprintf(stderr, v9);
                if (g_49f9a9 != 1)
                {
                    v10 = gettext(" !             Spawn a subshell\n");
                    fprintf(stderr, v10);
                }
                v11 = gettext(" ?             Print this list\n");
                fprintf(stderr, v11);
                continue;
            default:
                v14 = gettext("Invalid input. Type ? for help.\n");
                fprintf(stderr, v14);
            }
        }
    }
    sub_4177f4(); /* do not return */
}



// Function: new_volume @ 0xd818
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c014;
extern unsigned long long stdin;
extern unsigned long long g_49efa0;
extern unsigned long long g_49efb0;
extern long long g_49efe0;
extern long long g_49efe8;
extern unsigned long long g_49eff0;
extern unsigned long long g_49eff8;
extern char g_49f049;
extern FILE *g_49f248;
extern unsigned int g_49f250;
extern char g_49f320;
extern char g_49f884;
extern long long g_49f8c0;
extern long long g_49f950;
extern char g_49f9b0;
extern char g_49f9b8;
extern int g_49f9cc;
extern unsigned long long g_49fbd8;
extern unsigned long long g_49fbe0;
extern long long g_49fbe8;
extern struct_0 *g_49fdc8;
extern void g_49fdf0;
extern char g_49fdf8;

unsigned int new_volume(unsigned int a0)
{
    long long v12;  // rdx
    long long v13;  // rcx
    long long v14;  // r8
    long long v15;  // r9
    unsigned long v4;  // rdx
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    unsigned int v7;  // eax
    unsigned int v8;  // eax
    long long v10;  // rdi
    long long v11;  // rsi
    unsigned int v0;  // [bp-0x1c]

    if (!g_49f248 && !g_49f8c0)
        g_49f248 = (!g_49f9cc ? fopen("/dev/tty", "r") : stdin);
    if (g_49f320)
        return 0;
    if (g_49f9b0)
        sub_411a1c();
    sub_429547(&g_49efe0, 0);
    sub_429547(&g_49efe8, 0);
    g_49eff8 = 0;
    g_49eff0 = g_49eff8;
    g_49efb0 = g_49efa0;
    if ((g_49f9cc <= 0x3fffffff ? close(g_49f9cc) : sub_43fe0c(g_49f9cc - 0x40000000)))
        sub_43e1ff(*((long long *)g_49fbe8));
    g_49fbe8 = g_49fbe8 + 8;
    if (g_49fbd8 + g_49fbe0 * 8 == g_49fbe8)
    {
        g_49fbe8 = g_49fbd8;
        g_49f250 = 1;
    }
    v0 = g_49f250;
    while (1)
    {
        if (v0)
        {
            if (g_49f8c0)
            {
                if (*((long long *)&g_49f9b8))
                    sub_40d2e3();
                v4 = g_49c014 + 1;
                if (sub_4345e7(g_49fbe8, v4 & 0xffffffff, v4))
                    break;
            }
            else
            {
                sub_40d3f9(g_49f248);
            }
        }
        if (!strcmp(*((long long *)g_49fbe8), "-"))
        {
            g_49f049 = 1;
            g_49f9cc = 0;
        }
        else if (g_49f9b0)
        {
            if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbe8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbe8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbe8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbe8))))
                v5 = sub_43f860(*((long long *)g_49fbe8), 66, 0x40000000, g_49f950);
            else
                v5 = open(*((long long *)g_49fbe8), 66, 438);
            g_49f9cc = v5;
        }
        else if (a0 == 2)
        {
            if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbe8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbe8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbe8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbe8))))
                v8 = sub_43f860(*((long long *)g_49fbe8), 66, 0x40000000, g_49f950);
            else
                v8 = open(*((long long *)g_49fbe8), 66, 438);
            g_49f9cc = v8;
        }
        else if (a0 <= 2)
        {
            if (!a0)
            {
                if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbe8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbe8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbe8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbe8))))
                    v7 = sub_43f860(*((long long *)g_49fbe8), 0, 0x40000000, g_49f950);
                else
                    v7 = open(*((long long *)g_49fbe8), 0, 438);
                g_49f9cc = v7;
                sub_40b214();
            }
            else if (a0 == 1)
            {
                if (g_49f884)
                    sub_42a571(*((long long *)g_49fbe8), 1);
                if (g_49fdf8 != 1 && !(*((char **)&g_49fdf0) = strchr(*((long long *)g_49fbe8), 58), !*((long long *)&g_49fdf0) || *((long long *)g_49fbe8) >= *((long long *)&g_49fdf0) || memchr(*((long long *)g_49fbe8), 47, *((long long *)&g_49fdf0) - *((long long *)g_49fbe8))))
                    v6 = sub_43f860(*((long long *)g_49fbe8), 65, 0x40000000, g_49f950);
                else
                    v6 = creat(*((long long *)g_49fbe8), 438);
                g_49f9cc = v6;
            }
        }
        if (g_49f9cc >= 0)
            return 1;
        sub_43e3da(*((long long *)g_49fbe8));
        if (g_49f9b0 != 1 && a0 == 1 && g_49f884)
            sub_42a874();
        v0 = 1;
    }
    if (g_49fdc8)
        g_49fdc8();
    sub_45e84e(g_49f8c0);
    error(0, 0, gettext("%s command failed"));
    sub_4177f4(v10, v11, v12, v13, v14, v15); /* do not return */
}



// Function: read_header0 @ 0xde84
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f460;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int read_header0(long long a0)
{
    unsigned int v0;  // [bp-0xc]

    sub_439c55(a0);
    v0 = sub_425ecf(&g_49f460, a0, 0);
    if (v0 == 1)
    {
        sub_40bba7(*((long long *)&g_49f460));
        return 1;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("This does not look like a tar archive"));
    g_49fdd0 = 2;
    return 0;
}



// Function: try_new_volume @ 0xdf26
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern long long g_49efa0;
extern long long g_49efe0;
extern void g_49efe8;
extern long long g_49eff0;
extern long long g_49eff8;
extern void* g_49f060;
extern long long g_49f080;
extern long long g_49f460;
extern unsigned int g_49f860;
extern unsigned int g_49f864;
extern void g_49f870;
extern void g_49f9cc;
extern struct_1 *g_49fdc8;

unsigned int try_new_volume(void)
{
    unsigned long v9;  // rcx
    unsigned long v10;  // rax
    unsigned long long v20;  // rax
    unsigned int v12;  // eax
    unsigned long long v0;  // [bp-0xb8]
    unsigned int v1;  // [bp-0xa4]
    struct_0 *v2;  // [bp-0xa0]
    unsigned long v3;  // [bp-0x98]
    long long v4;  // [bp-0x90]
    char v5;  // [bp-0x88]
    char v6;  // [bp-0x68]
    char v7;  // [bp-0x48]

    if (g_49f860 <= 2)
    {
        if (!g_49f860)
            goto LABEL_40df6a;
LABEL_40df5e:
        v1 = 2;
    }
    else
    {
        if (g_49f860 == 8)
            goto LABEL_40df5e;
LABEL_40df6a:
        v1 = 0;
    }
    if ((char)sub_40d818(v1) != 1)
        return 1;
    while (1)
    {
        if (*((int *)&g_49f9cc) > 0x3fffffff)
        {
            v9 = *((int *)&g_49f9cc) - 0x40000000;
            v10 = sub_43fe62(v9 & 0xffffffff, g_49efa0, *((long long *)&g_49f870), v9);
        }
        else
        {
            v10 = sub_45ec70(*((int *)&g_49f9cc), g_49efa0, *((long long *)&g_49f870), g_49efa0);
        }
        v3 = v10;
        if (v3 != 0xffffffffffffffff)
            break;
        sub_40c729();
    }
    if (v3 != *((long long *)&g_49f870))
        sub_40c881(v3);
    v2 = sub_40bb4b();
    if (!v2)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("This does not look like a tar archive"));
        return 0;
    }
    else
    {
        v12 = v2->field_9c;
        switch (v12)
        {
        case 103:
            sub_439c55(&g_49f080);
            if (sub_425ecf(&v2, &g_49f080, 2, &g_49f080) != 2)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("This does not look like a tar archive"));
                return 0;
            }
            sub_41df5f(&g_49f080);
            sub_439d1c(&g_49f080);
            switch ((unsigned int)sub_425ecf(&v2, &g_49f080, 0, &g_49f080))
            {
            case 1:
                sub_40bba7(v2);
            case 5:
                break;
            default:
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("This does not look like a tar archive"));
                return 0;
            }
        case 77:
LABEL_40e24b:
            if ((char)sub_40de84(&g_49f080) != 1)
                return 0;
            sub_439d1c(&g_49f080);
            sub_429591(&g_49efe8, g_49f460, 100);
            g_49eff0 = sub_427a73(g_49f460 + 124, 12);
            g_49eff8 = sub_427a73(g_49f460 + 369, 12);
            break;
        case 86:
            if ((char)sub_40de84(&g_49f080) != 1)
                return 0;
            sub_439d1c(&g_49f080);
            sub_429591(&g_49efe0, g_49f460, 100);
            sub_40bba7(v2);
            v2 = sub_40bb4b();
            if (v2->field_9c == 77)
                goto LABEL_40e24b;
            else
                goto LABEL_40e2de;
        default:
            break;
        }
LABEL_40e2de:
        if (g_49f060)
        {
            if (!*((long long *)&g_49efe8))
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_45e84e((long long)g_49f060[16]);
                error(0, 0, gettext("%s is not continued on this volume"));
                return 0;
            }
            else
            {
                if (strcmp(*((long long *)&g_49efe8), (long long)g_49f060[16]))
                {
                    if ((g_49f864 == 6 || g_49f864 == 2) && strlen((long long)g_49f060[16]) > 99 && !strncmp(*((long long *)&g_49efe8), (long long)g_49f060[16], 100))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        sub_45e84e((long long)g_49f060[16]);
                        error(0, 0, gettext("%s is possibly continued on this volume: header contains truncated name"));
                    }
                    else
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        sub_45e84e((long long)g_49f060[16]);
                        error(0, 0, gettext("%s is not continued on this volume"));
                        return 0;
                    }
                }
                v4 = g_49eff8 + g_49eff0;
                if ((long long)g_49f060[24] != v4 || v4 < g_49eff8)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    v20 = sub_454e5a(g_49eff8, &v7, &v7);
                    sub_454e5a(g_49eff0, &v6, &v6);
                    sub_454e5a((long long)g_49f060[24], &v5, (long long)g_49f060[24]);
                    sub_45e84e(*((long long *)&g_49efe8));
                    v0 = v20;
                    error(0, 0, gettext("%s is the wrong size (%s != %s + %s)"));
                    return 0;
                }
                else if ((long long)g_49f060[24] - (long long)g_49f060[32] != g_49eff8)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_454e5a(g_49eff8, &v7, &v7);
                    sub_454e5a((long long)g_49f060[32], &v6, (long long)g_49f060[32]);
                    sub_454e5a((long long)g_49f060[24], &v5, (long long)g_49f060[24]);
                    error(0, 0, gettext("This volume is out of sequence (%s - %s != %s)"));
                    return 0;
                }
            }
        }
        sub_40d386();
        return 1;
    }
}



// Function: drop_volume_label_suffix @ 0xe680
void* drop_volume_label_suffix(char *ptr)
{
    unsigned long long *v4;  // rax
    char *iter;  // [bp-0x20]
    unsigned long len;  // [bp-0x18]
    void* v2;  // [bp-0x10]

    len = strlen(ptr);
    if (!len)
        return NULL;
    for (iter = ptr + len - 1; iter > ptr && (v4 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)*(iter) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x800); iter -= 1);
    if (iter <= ptr || ptr >= iter - 7)
        return NULL;
    iter -= 7;
    if (!memcmp(iter, " Volume ", 8))
    {
        len = iter - ptr;
        v2 = sub_460b5b(len + 1);
        memcpy(v2, ptr, len);
        *(len + (char *)v2) = 0;
        return v2;
    }
    return NULL;
}



// Function: check_label_pattern @ 0xe78a
extern char g_49f8fc;
extern char *g_49f9c0;

unsigned int check_label_pattern(void* a0)
{
    char v0;  // [bp-0x11]
    void* v1;  // [bp-0x10]

    v0 = 0;
    if (!fnmatch(g_49f9c0, a0, 0))
    {
        return 1;
    }
    else if (g_49f8fc != 1)
    {
        return 0;
    }
    else
    {
        v1 = sub_40e680(a0);
        if (v1)
        {
            v0 = !fnmatch(v1, g_49f9c0, 0);
            free(v1);
        }
        return v0;
    }
}



// Function: match_volume_label @ 0xe825
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern void g_49efe0;
extern long long g_49f9c0;
extern struct_1 *g_49fdc8;

void match_volume_label(void)
{
    long long v5;  // rdi
    long long v14;  // rsi
    long long v15;  // rdx
    long long v16;  // rcx
    long long v17;  // r8
    long long v18;  // r9
    long long v20;  // rdi
    long long v21;  // rsi
    long long v22;  // rdx
    long long v23;  // rcx
    long long v6;  // rsi
    long long v24;  // r8
    long long v25;  // r9
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    long long v13;  // rdi
    struct_0 *v0;  // [bp-0x1e0]
    char v1;  // [bp-0x1d8]
    char v2;  // [bp-0x70]

    if (!*((long long *)&g_49efe0))
    {
        v0 = sub_40bb4b();
        if (!v0)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e84e(g_49f9c0);
            error(0, 0, gettext("Archive not labeled to match %s"));
            sub_4177f4(v5, v6, v7, v8, v9, v10); /* do not return */
        }
        else if (v0->field_9c == 86)
        {
            sub_429591(&g_49efe0, v0, 100);
        }
        else if (v0->field_9c == 103)
        {
            sub_439c55(&v1);
            sub_41e28e(&v2, v0, sub_42798b(&v0->padding_0[124], 12), &v2);
            sub_41df5f(&v1);
            sub_439d1c(&v1);
        }
    }
    if (!*((long long *)&g_49efe0))
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e84e(g_49f9c0);
        error(0, 0, gettext("Archive not labeled to match %s"));
        sub_4177f4(v13, v14, v15, v16, v17, v18); /* do not return */
    }
    else if ((char)sub_40e78a(*((long long *)&g_49efe0)) != 1)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e821(1, g_49f9c0);
        sub_45e821(0, *((long long *)&g_49efe0));
        error(0, 0, gettext("Volume %s does not match %s"));
        sub_4177f4(v20, v21, v22, v23, v24, v25); /* do not return */
    }
    else
    {
        return;
    }
}



// Function: _write_volume_label @ 0xea79
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

extern long long g_49f080;
extern unsigned int g_49f864;
extern long long g_49f9e0;
extern long long g_49fa20;
extern long long g_49fa28;
extern char g_49fa30;

long long _write_volume_label(unsigned long a0)
{
    struct_0 *ptr;  // [bp-0x10]

    if (g_49f864 != 4)
    {
        ptr = sub_40bb4b();
        memset(ptr, 0, 0x200);
        strcpy(ptr, a0);
        sub_429547(&g_49fa28, ptr);
        g_49fa30 = sub_44e2e5(g_49fa28);
        ptr->field_9c = 86;
        sub_412af2(g_49f9e0, &ptr->padding_0[0x88], 12, &ptr->padding_0[0x88]);
        sub_41422e(&g_49fa20, ptr, -0x1);
        return sub_40bba7(ptr);
    }
    return sub_41e1d3("GNU.volume.label", &g_49f080, a0);
}



// Function: add_volume_label @ 0xeb6f
extern int g_49c010;
extern char *g_49f9c0;

void add_volume_label(void)
{
    unsigned long len;  // rax
    char *v0;  // [bp-0x48]
    void* ptr;  // [bp-0x40]
    char v2;  // [bp-0x38]

    v0 = sub_454e5a(g_49c010, &v2, &v2);
    len = strlen(g_49f9c0);
    ptr = sub_460b5b(strlen(v0) + len + 9);
    sprintf(ptr, "%s %s %s", g_49f9c0, "Volume", v0);
    sub_40ea79(ptr);
    free(ptr);
    return;
}



// Function: add_chunk_header @ 0xec37
extern int g_49c010;
extern unsigned int g_49f864;

void add_chunk_header(unsigned long long *idx)
{
    long long v0;  // [bp-0x1d0]
    char v1;  // [bp-0x1c8], Other Possible Types: unsigned long
    unsigned long v2;  // [bp-0x1c0]
    unsigned int v3;  // [bp-0x158]
    unsigned int v4;  // [bp-0x154]
    unsigned int v5;  // [bp-0x150]
    unsigned long v6;  // [bp-0x140]
    unsigned long v7;  // [bp-0xb0]

    if (g_49f864 != 4)
        return;
    memset(&v1, 0, 440);
    v2 = idx[2];
    v1 = v2;
    v3 = 33188;
    v4 = getuid();
    v5 = getgid();
    v1 = sub_41cd02(&v1, "%d/GNUFileParts/%f.%n", g_49c010, "%d/GNUFileParts/%f.%n");
    v2 = v1;
    v6 = idx[4];
    v7 = v6;
    v0 = sub_41381d(&v1);
    if (v0)
    {
        sub_41418c(sub_4135e3(0, &v1, v0));
        free(v1);
        return;
    }
    abort(); /* do not return */
}



// Function: write_volume_label @ 0xed89
extern char g_49f8fc;
extern unsigned int g_49f9c0;

long long write_volume_label(void)
{
    return (!g_49f8fc ? sub_40ea79(*((long long *)&g_49f9c0)) : (unsigned long long)sub_40eb6f());
}



// Function: gnu_add_multi_volume_header @ 0xedb5
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned int g_49f9ac;
extern long long g_49fa20;
extern struct_1 *g_49fdc8;

long long gnu_add_multi_volume_header(void* a0)
{
    unsigned int v0;  // [bp-0x2c]
    unsigned long len;  // [bp-0x28]
    struct_0 *ptr;  // [bp-0x20]

    ptr = sub_40bb4b();
    len = strlen((long long)a0[16]);
    if (len > 100)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548((long long)a0[16]);
        error(0, 0, gettext("%s: file name too long to be stored in a GNU multivolume header, truncated"));
        len = 100;
    }
    memset(ptr, 0, 0x200);
    memcpy(ptr, (long long)a0[16], len);
    ptr->field_9c = 77;
    sub_412a9a((long long)a0[32], &ptr->padding_0[124], 12, &ptr->padding_0[124]);
    sub_412a9a((long long)a0[24] - (long long)a0[32], &ptr[2].padding_0[55], 12, &ptr[2].padding_0[55]);
    v0 = g_49f9ac;
    g_49f9ac = 0;
    sub_41422e(&g_49fa20, ptr, -0x1);
    g_49f9ac = v0;
    return sub_40bba7(ptr);
}



// Function: add_multi_volume_header @ 0xef1c
typedef struct struct_0 {
    char padding_0[16];
    long long field_10;
    unsigned long long field_18;
    long long field_20;
} struct_0;

extern long long g_49f080;
extern unsigned int g_49f864;

void add_multi_volume_header(struct_0 *idx)
{
    unsigned long v0;  // [bp-0x18]

    if (g_49f864 != 4)
    {
        sub_40edb5(idx);
        return;
    }
    v0 = idx->field_18 - idx->field_20;
    sub_41e1d3("GNU.volume.filename", &g_49f080, idx->field_10, idx->field_20);
    sub_41e1d3("GNU.volume.size", &g_49f080, &idx->field_20);
    sub_41e1d3("GNU.volume.offset", &g_49f080, &v0);
    return;
}



// Function: simple_flush_read @ 0xeff1
extern long long g_49efa0;
extern unsigned long long g_49efc0;
extern char g_49efd8;
extern unsigned long long g_49f038;
extern unsigned int g_49f044;
extern char g_49f870;
extern void g_49f9cc;

unsigned long long simple_flush_read(void)
{
    unsigned long v2;  // rcx
    unsigned long v3;  // rax
    unsigned long v0;  // [bp-0x10]

    sub_4106d8(0);
    g_49f044 = 0;
    if (g_49efd8 && g_49f038)
    {
        *((unsigned int *)&g_49f9cc) = 1;
        v0 = sub_43301a();
        *((unsigned int *)&g_49f9cc) = 0;
        if (v0 != *((long long *)&g_49f870))
            sub_40c6d5(v0); /* do not return */
    }
    while (1)
    {
        if (*((int *)&g_49f9cc) > 0x3fffffff)
        {
            v2 = *((int *)&g_49f9cc) - 0x40000000;
            v3 = sub_43fe62(v2 & 0xffffffff, g_49efa0, *((long long *)&g_49f870), v2);
        }
        else
        {
            v3 = sub_45ec70(*((int *)&g_49f9cc), g_49efa0, *((long long *)&g_49f870), g_49efa0);
        }
        v0 = v3;
        if (v0 == *((long long *)&g_49f870))
        {
            g_49efc0 = g_49efc0 + 1;
            return g_49efc0;
        }
        if (v0 != 0xffffffffffffffff)
            return sub_40c881(v0);
        sub_40c729();
    }
}



// Function: simple_flush_write @ 0xf0fb
extern unsigned long long g_49efc8;
extern unsigned long long g_49f008;
extern char g_49f870;

unsigned long long simple_flush_write(unsigned long long a0)
{
    unsigned long long v3;  // rax
    unsigned long long v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x10]

    v0 = a0;
    v1 = sub_40c54f();
    if (v1 == *((long long *)&g_49f870))
    {
        v3 = g_49efc8;
        g_49efc8 = v3 + 1;
        g_49f008 = AddV(g_49f008, v1);
        return v3 + 1;
    }
    sub_40c6d5(v1); /* do not return */
}



// Function: _gnu_flush_read @ 0xf160
extern long long g_49efa0;
extern unsigned long long g_49efa8;
extern char g_49efb0;
extern unsigned long long g_49efc0;
extern char g_49efd8;
extern unsigned long long g_49f038;
extern unsigned int g_49f044;
extern char g_49f870;
extern char g_49f8fc;
extern void g_49f9cc;

unsigned long long _gnu_flush_read(void)
{
    unsigned long v2;  // rcx
    unsigned long v3;  // rax
    unsigned long long v4;  // rax
    unsigned long v0;  // [bp-0x10]

    sub_4106d8(0);
    g_49f044 = 0;
    if (g_49efd8 && g_49f038)
    {
        *((unsigned int *)&g_49f9cc) = 1;
        v0 = sub_43301a();
        *((unsigned int *)&g_49f9cc) = 0;
        if (v0 != *((long long *)&g_49f870))
            sub_40c6d5(v0); /* do not return */
    }
    while (1)
    {
        if (*((int *)&g_49f9cc) > 0x3fffffff)
        {
            v2 = *((int *)&g_49f9cc) - 0x40000000;
            v3 = sub_43fe62(v2 & 0xffffffff, g_49efa0, *((long long *)&g_49f870), v2);
        }
        else
        {
            v3 = sub_45ec70(*((int *)&g_49f9cc), g_49efa0, *((long long *)&g_49f870), g_49efa0);
        }
        v0 = v3;
        if (v0 == *((long long *)&g_49f870))
        {
            g_49efc0 = g_49efc0 + 1;
            return g_49efc0;
        }
        if ((!v0 || v0 == 0xffffffffffffffff && *(__errno_location()) == 28) && g_49f8fc)
            break;
        if (v0 != 0xffffffffffffffff)
            return sub_40c881(v0);
        sub_40c729();
    }
    do
    { } while ((char)sub_40df26() != 1);
    v4 = g_49efa8;
    if (*((long long *)&g_49efb0) == g_49efa8)
        v4 = sub_40cac8();
    return v4;
}



// Function: gnu_flush_read @ 0xf2b9
extern unsigned long long g_49f058;

int gnu_flush_read(void)
{
    g_49f058 = sub_40eff1;
    sub_40f160();
    g_49f058 = gnu_flush_read;
    return gnu_flush_read;
}



// Function: _gnu_flush_write @ 0xf2e5
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49efa0;
extern unsigned long long g_49efc8;
extern unsigned long long g_49f000;
extern unsigned long long g_49f008;
extern unsigned int g_49f030;
extern unsigned int g_49f070;
extern long long g_49f080;
extern char g_49f870;
extern char g_49f8fc;
extern unsigned long long g_49f9c0;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long _gnu_flush_write(unsigned long a0)
{
    unsigned long v8;  // rax
    unsigned long ptr;  // [bp-0x38]
    void* iter;  // [bp-0x30]
    unsigned long n;  // [bp-0x28]
    unsigned long n1;  // [bp-0x20]
    unsigned long v4;  // [bp-0x18]
    void* v5;  // [bp-0x10]

    v4 = sub_40c54f();
    if (v4 != *((long long *)&g_49f870) && g_49f8fc != 1)
        sub_40c6d5(v4); /* do not return */
    if (v4)
        g_49efc8 = g_49efc8 + 1;
    g_49f008 = AddV(g_49f008, v4);
    if (v4 == *((long long *)&g_49f870))
        return *((long long *)&g_49f870);
    v5 = sub_40abd9(v4);
    if ((unsigned short)v4 & 0x1ff)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("write did not end on a block boundary"));
        g_49fdd0 = 2;
        sub_40c6d5(v4); /* do not return */
    }
    else
    {
        if (v4 - 0 >> 63 & 1 && *(__errno_location()) != 28 && *(__errno_location()) != 5 && *(__errno_location()) != 6)
            sub_40c6d5(v4); /* do not return */
        v8 = sub_40d818(1) ^ 1;
        if ((char)v8)
            return v8;
        sub_439d1c(&g_49f080);
        sub_40d386();
        g_49f000 = AddV(g_49f008, g_49f000);
        g_49f008 = 0;
        iter = v4 + g_49efa0;
        n = a0 - v4;
        g_49f030 = !g_49f030;
        sub_40bc48();
        g_49f070 = 1;
        if (g_49f9c0)
            sub_40eb6f();
        if (v5)
            sub_40ef1c(v5);
        sub_4135e3(1, &g_49f080, sub_40bb4b());
        sub_439d1c(&g_49f080);
        if (v5)
            sub_40ec37(v5);
        ptr = sub_40bb4b();
        sub_40acb0(v5, (long long)(ptr - g_49efa0) >> 9);
        n1 = sub_40bbf5(ptr);
        for (g_49f070 = 0; n1 < n; n1 = sub_40bbf5(ptr))
        {
            memcpy(ptr, iter, n1);
            iter += n1;
            n -= n1;
            sub_40bba7(ptr + (n1 - 1 & 0xfffffffffffffe00));
            ptr = sub_40bb4b();
        }
        memcpy(ptr, iter, n);
        memset(ptr + n, 0, n1 - n);
        sub_40bba7(ptr + (n - 1 & 0xfffffffffffffe00));
        return sub_40bb4b();
    }
}



// Function: gnu_flush_write @ 0xf627
extern unsigned long long g_49f050;

int gnu_flush_write(unsigned int a0)
{
    g_49f050 = sub_40f0fb;
    sub_40f2e5(a0);
    g_49f050 = gnu_flush_write;
    return gnu_flush_write;
}



// Function: flush_read @ 0xf662
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f058;

long long flush_read(void)
{
    return g_49f058();
}



// Function: flush_write @ 0xf676
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f050;
extern unsigned long long g_49f870;

long long flush_write(void)
{
    return g_49f050(g_49f870);
}



// Function: open_archive @ 0xf694
extern unsigned long long g_49efc8;
extern unsigned long long g_49f050;
extern unsigned long long g_49f058;
extern unsigned long long g_49f9c0;

long long open_archive(unsigned int a0)
{
    g_49f058 = sub_40f2b9;
    g_49f050 = sub_40f627;
    sub_40bdb0(a0);
    if (a0 == 2)
    {
LABEL_40f6e3:
        if (g_49f9c0)
            sub_40e825();
    }
    else if (a0 <= 2)
    {
        if (!a0)
            goto LABEL_40f6e3;
        if (a0 == 1)
        {
            g_49efc8 = 0;
            if (g_49f9c0)
                sub_40ed89();
        }
    }
    return sub_40ae17();
}



// Function: alloc_action @ 0xf720
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned long long g_49f268;
extern struct_2 *g_49f270;

struct_0 * alloc_action(unsigned int a0)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_460c06(24);
    if (g_49f270)
        g_49f270->field_0 = v0;
    else
        g_49f268 = v0;
    g_49f270 = v0;
    v0->field_8 = a0;
    return v0;
}



// Function: copy_string_unquote @ 0xf77f
char * copy_string_unquote(long long a0)
{
    char *ptr;  // [bp-0x18]
    unsigned long len;  // [bp-0x10]

    ptr = sub_460cc6(a0);
    len = strlen(ptr);
    if (*(ptr) != 0x22 && *(ptr) != 39)
    {
        sub_42961d(ptr);
        return ptr;
    }
    if (*(&ptr[len] - 1) == *(ptr))
    {
        memmove(ptr, ptr + 1, len - 2);
        *(&ptr[len] - 2) = 0;
        sub_42961d(ptr);
    }
    else
    {
        sub_42961d(ptr);
    }
    return ptr;
}



// Function: checkpoint_compile_action @ 0xf825
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
} struct_0;

extern unsigned int g_49f278;
extern int g_49f280;
extern struct_0 *g_49fdc8;

void checkpoint_compile_action(unsigned long a0)
{
    long long v5;  // rdi
    long long v14;  // rdx
    long long v15;  // rcx
    long long v16;  // r8
    long long v17;  // r9
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    long long v12;  // rdi
    long long v13;  // rsi
    char *v0;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x20]
    unsigned long long num;  // [bp-0x18]

    if (!g_49f278)
    {
        sigemptyset(&g_49f280);
        g_49f278 = 1;
    }
    if (strcmp(a0, ".") && strcmp(a0, "dot"))
    {
        if (!strcmp(a0, "bell"))
        {
            sub_40f720(1);
            return;
        }
        else if (!strcmp(a0, "echo"))
        {
            sub_40f720(2);
            return;
        }
        else if (!strncmp(a0, "echo=", 5))
        {
            idx = sub_40f720(2);
            *((char **)&idx[1].padding_0[4]) = sub_40f77f(a0 + 5);
            return;
        }
        else if (!strncmp(a0, "exec=", 5))
        {
            idx = sub_40f720(5);
            *((char **)&idx[1].padding_0[4]) = sub_40f77f(a0 + 5);
            return;
        }
        else if (!strncmp(a0, "ttyout=", 7))
        {
            idx = sub_40f720(3);
            *((char **)&idx[1].padding_0[4]) = sub_40f77f(a0 + 7);
            return;
        }
        else if (!strncmp(a0, "sleep=", 6))
        {
            num = strtoul(a0 + 6, &v0, 10, a0 + 6);
            if (!*(v0))
            {
                idx = sub_40f720(4);
                *((unsigned long long *)&idx[1].padding_0[4]) = num;
                return;
            }
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%s: not a valid timeout"));
            sub_4177f4(v5, v6, v7, v8, v9, v10); /* do not return */
        }
        else if (!strcmp(a0, "totals"))
        {
            sub_40f720(6);
            return;
        }
        else if (!strncmp(a0, "wait=", 5))
        {
            idx = sub_40f720(7);
            *((unsigned int *)&idx[1].padding_0[4]) = sub_435562(a0 + 5);
            sigaddset(&g_49f280, *((int *)&idx[1].padding_0[4]));
            return;
        }
        else
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%s: unknown checkpoint action"));
            sub_4177f4(v12, v13, v14, v15, v16, v17); /* do not return */
        }
    }
    sub_40f720(0);
    return;
}



// Function: checkpoint_finish_compile @ 0xfb72
extern unsigned long long g_49f268;
extern unsigned int g_49f278;
extern int g_49f280;
extern unsigned int g_49f890;

long long checkpoint_finish_compile(void)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax

    if (!g_49f278 && g_49f890 && !g_49f268)
        sub_40f825("echo");
    v1 = g_49f278;
    if ((unsigned int)v1 != 1)
        return v1;
    sigprocmask(0, &g_49f280, NULL);
    v2 = g_49f890;
    if (!(unsigned int)v2)
        g_49f890 = 10;
    g_49f278 = 2;
    return v2;
}



// Function: getwidth @ 0xfbee
typedef struct FILE {
} FILE;

long long getwidth(FILE *a0)
{
    char *v0;  // [bp-0x28]
    long long num;  // [bp-0x20]
    unsigned short v2;  // [bp-0x16]

    if (!ioctl(fileno(a0), 21523) && v2)
        return v2;
    v0 = getenv("COLUMNS");
    if (v0)
    {
        num = strtol(v0, NULL, 10);
        if (num > NULL)
            return num;
    }
    return 80;
}



// Function: getarg @ 0xfca5
typedef struct struct_0 {
    char field_0;
} struct_0;

long long getarg(char *a0, struct_0 **a1, void* a2, void* a3)
{
    char *ptr;  // [bp-0x18]
    unsigned long n;  // [bp-0x10]

    if (*(a0) == 123)
    {
        ptr = strchr(a0 + 1, 125);
        if (ptr)
        {
            n = ptr - a0;
            if (n > *((long long *)a3))
            {
                *((unsigned long *)a3) = n;
                *((unsigned long long *)a2) = sub_460b8d(*((long long *)a2), *((long long *)a3), *((long long *)a3));
            }
            n -= 1;
            memcpy(*((long long *)a2), a0 + 1, n);
            *((char *)(n + *((long long *)a2))) = 0;
            *(a1) = ptr + 1;
            return *((long long *)a2);
        }
    }
    *(a1) = a0;
    return 0;
}



// Function: format_checkpoint_string @ 0xfd96
typedef struct struct_0 {
    char field_0;
} struct_0;

extern struct_0 g_49c140;
extern void* g_49c158;
extern unsigned int g_49f300;
extern void g_49f310;
extern void g_49f318;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long format_checkpoint_string(unsigned long a0, long long a1, void* a2, char a3, unsigned int a4)
{
    int v20;  // eax
    void* v0;  // [bp-0x1c0]
    long long iter;  // [bp-0x1b8]
    int node;  // [bp-0x19c]
    void* iter1;  // [bp-0x198]
    unsigned long v4;  // [bp-0x190]
    struct_0 *v5;  // [bp-0x188]
    char *v6;  // [bp-0x180]
    char *v7;  // [bp-0x178]
    unsigned long v8;  // [bp-0x170]
    unsigned long tm;  // [bp-0x168]
    long long v10;  // [bp-0x160]
    int v11[3];  // [bp-0x158]
    unsigned long long v12;  // [bp-0x150]
    char v13;  // [bp-0x148]
    char v14;  // [bp-0x128]
    unsigned long v15;  // [bp-0x120]
    unsigned long long v16;  // [bp-0xf0]
    char v17;  // [bp-0x38]

    iter = a1;
    v0 = a2;
    v6 = (!a3 ? gettext("read") : gettext("write"));
    v7 = sub_454e5a(a4, &v17, &v17);
    v4 = 0;
    if (!v0)
        v0 = (!a3 ? gettext("Read checkpoint %u") : gettext("Write checkpoint %u"));
    for (iter1 = v0; *((char *)iter1); iter1 += 1)
    {
        if (*((char *)iter1) == 37)
        {
            iter1 += 1;
            if (*((char *)iter1) == 123 && !(v4 = (unsigned long)sub_40fca5(iter1, &iter1, &g_49f310, &g_49f318), v4))
            {
                fputc_unlocked(37, a0);
                fputc_unlocked(*((char *)iter1), a0);
                iter += 2;
            }
            else
            {
                v20 = *((char *)iter1);
                if (v20 != 42)
                {
                    if (v20 < 42)
                        goto LABEL_4102eb;
                    if (v20 > 117)
                        goto LABEL_4102eb;
                    if (v20 < 84)
                        goto LABEL_4102eb;
                    switch (v20)
                    {
                    case 84:
                        v5 = &g_49c140.field_0;
                        sub_40ae4d();
                        if (v4)
                        {
                            v16 = ",";
                            if (sub_4461a6(v4, &v14, 17988, &v14))
                            {
                                if (g_49fdc8)
                                    g_49fdc8();
                                sub_44647f(&v14);
                                error(0, 0, gettext("cannot split string '%s': %s"));
                                g_49fdd0 = 2;
                            }
                            else
                            {
                                for (node = 0; node < v14; node += 1)
                                {
                                    *((long long *)&(&v13)[8 * node]) = *((long long *)(node * 8 + v15));
                                }
                                for (; node <= 2; node += 1)
                                {
                                    *((unsigned long *)&(&v13)[8 * node]) = 0;
                                }
                                v5 = &v13;
                            }
                        }
                        iter += (int)sub_40b768(a0, v5, 44, 0);
                        if (v4)
                            sub_446399(&v14);
                        break;
                    case 99:
                        iter += (int)format_checkpoint_string(a0, iter, g_49c158, a3, a4);
                        break;
                    case 100:
                        iter += (int)fprintf(a0, "%.0f", sub_40ae4d());
                        break;
                    case 115:
                        fputs_unlocked(v6, a0);
                        iter += strlen(v6);
                        break;
                    case 116:
                        v8 = (!v4 ? "%c" : v4);
                        gettimeofday(v11, NULL);
                        tm = localtime(v11);
                        iter += sub_44f679(a0, v8, tm, 0, (unsigned int)v12 * 1000);
                        break;
                    case 117:
                        fputs_unlocked(v7, a0);
                        iter += strlen(v7);
                        break;
                    default:
LABEL_4102eb:
                        fputc_unlocked(37, a0);
                        fputc_unlocked(*((char *)iter1), a0);
                        iter += 2;
                        break;
                    }
                }
                else
                {
                    for (v10 = (!v4 ? sub_40fbee(a0) : strtol(v4, NULL, 10)); iter < v10; iter += 1)
                    {
                        fputc_unlocked(32, a0);
                    }
                }
                v4 = 0;
            }
        }
        else
        {
            fputc_unlocked(*((char *)iter1), a0);
            if (*((char *)iter1) == 13)
            {
                iter = 0;
                g_49f300 = 1;
            }
            else
            {
                iter += 1;
            }
        }
    }
    fflush_unlocked(a0);
    return iter;
}



// Function: run_checkpoint_actions @ 0x103d4
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern void stderr;
extern FILE *g_49efd0;
extern unsigned int g_49f260;
extern void* g_49f268;
extern long long g_49f280;
extern unsigned long long g_49f308;
extern struct_0 *g_49fbe8;
extern char *g_49fe48;

void run_checkpoint_actions(char a0)
{
    char v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]
    void* i;  // [bp-0x18]

    for (i = g_49f268; i; i = *((long long *)i))
    {
        switch ((int)i[8])
        {
        case 0:
            fputc_unlocked(46, g_49efd0);
            fflush_unlocked(g_49efd0);
            break;
        case 1:
            if (!g_49f308)
                g_49f308 = fopen("/dev/tty", "w");
            if (g_49f308)
            {
                fputc_unlocked(7, g_49f308);
                fflush_unlocked(g_49f308);
            }
            break;
        case 2:
            v1 = fprintf(*((long long *)&stderr), "%s: ", g_49fe48);
            sub_40fd96(*((long long *)&stderr), v1, (long long)i[16], a0, g_49f260);
            fputc_unlocked(10, *((long long *)&stderr));
            break;
        case 3:
            if (!g_49f308)
                g_49f308 = fopen("/dev/tty", "w");
            if (g_49f308)
                sub_40fd96(g_49f308, 0, (long long)i[16], a0, g_49f260);
            break;
        case 4:
            sleep((long long)i[16]);
            break;
        case 5:
            sub_4348d7((long long)i[16], g_49fbe8->field_0, g_49f260, g_49fbe8->field_0);
            break;
        case 6:
            sub_40ae4d();
            sub_40baa2();
            break;
        case 7:
            sigwait(&g_49f280, &v0);
            break;
        }
    }
    return;
}



// Function: checkpoint_flush_actions @ 0x10622
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
} struct_0;

extern struct_0 *g_49f268;
extern unsigned int g_49f300;
extern FILE *g_49f308;

struct_0 * checkpoint_flush_actions(void)
{
    struct_0 *v4;  // rax
    struct_0 *i;  // [bp-0x18]
    long long v1;  // [bp-0x10]
    long long v2;  // [bp-0x10]

    v4 = g_49f268;
    for (i = g_49f268; i; i = i->field_0)
    {
        if (i->field_8 == 3 && g_49f308 && g_49f300)
        {
            v1 = sub_40fbee(g_49f308);
            while (1)
            {
                v2 = v1 - 1;
                if (!v1)
                    break;
                fputc_unlocked(32, g_49f308);
                v1 = v2;
            }
            fputc_unlocked(13, g_49f308);
            fflush_unlocked(g_49f308);
        }
        v4 = i->field_0;
    }
    return v4;
}



// Function: checkpoint_run @ 0x106d8
extern unsigned int g_49f260;
extern unsigned int g_49f890;

long long checkpoint_run(char a0)
{
    unsigned long v1;  // rax

    v1 = g_49f890;
    if (g_49f890)
    {
        g_49f260 = g_49f260 + 1;
        v1 = g_49f260 % g_49f890 & 0xffffffff;
        if (!(unsigned int)v1)
            v1 = (unsigned long long)sub_4103d4(a0);
    }
    return v1;
}



// Function: checkpoint_finish @ 0x10729
typedef struct FILE {
} FILE;

extern FILE *g_49f308;
extern unsigned int g_49f890;

FILE * checkpoint_finish(void)
{
    if (!g_49f890)
        return g_49f890;
    sub_410622();
    if (!g_49f308)
        return g_49f308;
    return fclose(g_49f308);
}



// Function: diff_init @ 0x1075e
extern unsigned long long g_49f328;
extern long long g_49f870;
extern unsigned long long g_49f8e0;

void diff_init(void)
{
    char v0;  // [bp-0x18]

    g_49f328 = sub_42b601(&v0, g_49f870, g_49f870);
    if (!g_49f8e0)
        return;
    sub_424090();
    return;
}



// Function: report_difference @ 0x107be
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_49efd0;

void report_difference(struct_0 *a0, char *a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v12;  // fs
    char *v13;  // rax
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

    v6 = a2;
    v7 = a3;
    v8 = a4;
    v9 = a5;
    v4 = *((long long *)(40 + v12));
    if (a1)
    {
        v13 = sub_429516(1, a0->field_8);
        fprintf(g_49efd0, "%s: ", v13);
        v0 = 16;
        v1 = 48;
        v2 = &v10;
        v3 = &v5;
        vfprintf(g_49efd0, a1, &v0);
        fprintf(g_49efd0, "\n");
    }
    sub_439e98(1);
    if (v4 == *((long long *)(40 + v12)))
        return;
    __stack_chk_fail(); /* do not return */
}



// Function: process_noop @ 0x10903
unsigned int process_noop(unsigned long a0, unsigned long a1)
{
    return 1;
}



// Function: process_rawdata @ 0x1091a
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f324;
extern void g_49f328;
extern struct_0 g_49fa20;
extern long long g_49fa28;

unsigned int process_rawdata(unsigned long a0, void* a1)
{
    unsigned long long v2;  // rdx
    unsigned long long v3;  // rcx
    unsigned long long v8;  // rdx
    unsigned long long v9;  // rcx
    unsigned long long v10;  // r8
    unsigned long long v11;  // r9
    unsigned long long v4;  // r8
    unsigned long long v5;  // r9
    unsigned long v0;  // [bp-0x10]

    v0 = sub_42a9e9(g_49f324, *((long long *)&g_49f328), a0, *((long long *)&g_49f328));
    if (v0 != a0)
    {
        if (v0 == 0xffffffffffffffff)
        {
            sub_43e403(g_49fa28);
            sub_4107be(&g_49fa20.field_0, NULL, v2, v3, v4, v5);
        }
        else
        {
            sub_4107be(&g_49fa20.field_0, ngettext("Could only read %lu of %lu byte", "Could only read %lu of %lu bytes", a0), v0, a0, v4, v5);
        }
        return 0;
    }
    else if (memcmp(a1, *((long long *)&g_49f328), a0))
    {
        sub_4107be(&g_49fa20.field_0, gettext("Contents differ"), v8, v9, v10, v11);
        return 0;
    }
    else
    {
        return 1;
    }
}



// Function: read_and_process @ 0x10a26
typedef struct struct_0 {
    long long field_0;
    char padding_8[128];
    unsigned int field_88;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long read_and_process(struct_0 *a0, unsigned long long *a1)
{
    unsigned long v5;  // rax
    unsigned long long *v0;  // [bp-0x38]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = a1;
    v2 = *((long long *)&a0->field_88);
    sub_40ad2b(a0);
    while (1)
    {
        if (!v2)
            return sub_40ad66();
        v3 = sub_40bb4b();
        if (!v3)
            break;
        v1 = sub_40bbf5(v3);
        if (v1 > v2)
            v1 = v2;
        if (!v0(v1, v3, v3, v0))
            v0 = sub_410903;
        sub_40bba7(v3 + v1 - 1);
        v2 -= v1;
        sub_40ad86(v2);
    }
    if (g_49fdc8)
        g_49fdc8();
    v5 = (unsigned long long)error(0, 0, gettext("Unexpected EOF in archive"));
    g_49fdd0 = 2;
    return v5;
}



// Function: get_stat_data @ 0x10b3a
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 g_49fa20;

unsigned int get_stat_data(long long a0, long long a1)
{
    unsigned long long v2;  // rdx
    unsigned long long v3;  // rcx
    unsigned long long v4;  // r8
    unsigned long long v5;  // r9
    unsigned int v0;  // [bp-0xc]

    v0 = sub_42a9b8(a0, a1, a1);
    if (!v0)
        return 1;
    if (*(__errno_location()) == 2)
        sub_43ea2c(a0);
    else
        sub_43ea03(a0);
    sub_4107be(&g_49fa20.field_0, NULL, v2, v3, v4, v5);
    return 0;
}



// Function: diff_dir @ 0x10bb7
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 g_49fa20;
extern long long g_49fa28;
extern unsigned int g_49fa90;

void diff_dir(void)
{
    unsigned long long v4;  // rdx
    unsigned long long v5;  // rcx
    unsigned long long v6;  // r8
    unsigned long long v7;  // r9
    unsigned long long v9;  // rdx
    unsigned long long v10;  // rcx
    unsigned long long v11;  // r8
    unsigned long long v12;  // r9
    char v0;  // [bp-0xa8]
    unsigned int v1;  // [bp-0x90]

    if (!sub_410b3a(g_49fa28, &v0))
    {
        return;
    }
    else if (((unsigned short)v1 & 0xf000) != 0x4000)
    {
        sub_4107be(&g_49fa20.field_0, gettext("File type differs"), v4, v5, v6, v7);
        return;
    }
    else if (((unsigned short)v1 ^ (unsigned short)g_49fa90) & 0xfff)
    {
        sub_4107be(&g_49fa20.field_0, gettext("Mode differs"), v9, v10, v11, v12);
        return;
    }
    else
    {
        return;
    }
}



// Function: diff_file @ 0x10c82
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern void g_49f324;
extern struct_0 *g_49f460;
extern unsigned int g_49f880;
extern char g_49fa20;
extern unsigned long long g_49fa28;
extern long long g_49fa78;
extern unsigned int g_49fa90;
extern char g_49faa8;
extern long long g_49fb18;
extern long long g_49fb20;
extern char g_49fb40;
extern unsigned int g_49fc08;

void diff_file(void)
{
    unsigned long v8;  // rdx
    unsigned long v17;  // r8
    unsigned long v18;  // r9
    long long v21;  // rax
    long long v24;  // rdi
    long long v25;  // rsi
    long long v26;  // rdi
    unsigned long v9;  // rcx
    long long v27;  // rsi
    unsigned long v10;  // r8
    unsigned long v11;  // r9
    long long v12;  // rdi
    long long v13;  // rsi
    unsigned long v15;  // rdx
    unsigned long v16;  // rcx
    unsigned int v0;  // [bp-0xd4]
    unsigned long v1;  // [bp-0xd0]
    long long v2;  // [bp-0xc8]
    char v3;  // [bp-0xb8]
    unsigned int v4;  // [bp-0xa0]
    unsigned long long v5;  // [bp-0x88]

    v1 = g_49fa28;
    if (!sub_410b3a(v1, &v3))
    {
        sub_428891();
        return;
    }
    else if (((unsigned short)v4 & 0xf000) != 0x8000)
    {
        sub_4107be(&g_49fa20, gettext("File type differs"), v8, v9, v10, v11);
        sub_428891(v12, v13, v8, v9, v10, v11);
        return;
    }
    else
    {
        if (((unsigned short)v4 ^ (unsigned short)g_49fa90) & 0xfff)
            sub_4107be(&g_49fa20, gettext("Mode differs"), v15, v16, v17, v18);
        if ((char)sub_432ebf(&v3, &g_49fa78, &g_49fa78) != 1)
            sub_4107be(&g_49fa20, gettext("Uid differs"), v15, v16, v17, v18);
        if ((char)sub_432ee4(&v3, &g_49fa78, &g_49fa78) != 1)
            sub_4107be(&g_49fa20, gettext("Gid differs"), v15, v16, v17, v18);
        v21 = sub_45fd9e(&v3);
        if (sub_439e35(v21, v15, g_49fb18, g_49fb20))
            sub_4107be(&g_49fa20, gettext("Mod time differs"), v15, v16, v17, v18);
        if (*((char *)&g_49f460[9].field_8 + 4) != 83 && v5 != *((long long *)&g_49faa8))
        {
            sub_4107be(&g_49fa20, gettext("Size differs"), v15, v16, v17, v18);
            sub_428891(v24, v25, v15, v16, v17, v18);
            return;
        }
        *((unsigned int *)&g_49f324) = openat(g_49c268, v1, g_49fc08, v1);
        if (*((int *)&g_49f324) < 0)
        {
            sub_43e38b(v1);
            sub_428891(v26, v27, v15, v16, v17, v18);
            sub_4107be(&g_49fa20, NULL, v15, v16, v17, v18);
            return;
        }
        if (g_49fb40)
            sub_430972(*((int *)&g_49f324), &g_49fa20, &g_49fa20);
        else
            sub_410a26(&g_49fa20, sub_41091a);
        if (g_49f880 == 1 && v5)
        {
            v2 = sub_45fd6a(&v3);
            if (sub_42ab5e(*((int *)&g_49f324), g_49c268, v1, v2, v15))
                sub_43ead0(v1);
        }
        v0 = close(*((int *)&g_49f324));
        if (!v0)
            return;
        sub_43e1ff(v1);
        return;
    }
}



// Function: diff_link @ 0x10fea
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 g_49fa20;
extern long long g_49fa28;
extern long long g_49fa38;

void diff_link(void)
{
    unsigned long long v3;  // rax
    unsigned long long v5;  // rcx
    unsigned long long v6;  // r8
    unsigned long long v7;  // r9
    char v0;  // [bp-0x148]
    char v1;  // [bp-0xb8]

    if (!sub_410b3a(g_49fa28, &v0))
    {
        return;
    }
    else if (!sub_410b3a(g_49fa38, &v1))
    {
        return;
    }
    else if ((char)sub_432f09(&v0, &v1, &v1) != 1)
    {
        v3 = sub_429516(0, g_49fa38);
        sub_4107be(&g_49fa20.field_0, gettext("Not linked to %s"), v3, v5, v6, v7);
        return;
    }
    else
    {
        return;
    }
}



// Function: diff_symlink @ 0x110be
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern struct_0 g_49fa20;
extern long long g_49fa28;
extern void* g_49fa38;

void diff_symlink(void)
{
    unsigned long long v6;  // rdx
    unsigned long long v8;  // rcx
    unsigned long long v9;  // r8
    unsigned long long v10;  // r9
    unsigned long len;  // [bp-0x430]
    char *v1;  // [bp-0x428]
    unsigned long count;  // [bp-0x420]
    char v3;  // [bp-0x418]

    len = strlen(g_49fa38);
    v1 = (len <= 0x3ff ? &v3 : sub_460b5b(len + 1));
    count = readlinkat(g_49c268, g_49fa28, v1, len + 1);
    if (count - 0 >> 63 & 1)
    {
        if (*(__errno_location()) == 2)
            sub_43e6dc(g_49fa28);
        else
            sub_43e6b3(g_49fa28);
        sub_4107be(&g_49fa20.field_0, NULL, v6, v8, v9, v10);
    }
    else if (count != len || memcmp(g_49fa38, v1, len))
    {
        sub_4107be(&g_49fa20.field_0, gettext("Symlink differs"), v6, v8, v9, v10);
    }
    if (v1 != &v3)
        free(v1);
    return;
}



// Function: diff_special @ 0x11230
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f460;
extern struct_0 g_49fa20;
extern long long g_49fa28;
extern unsigned int g_49fa90;
extern unsigned long long g_49faa0;

void diff_special(void)
{
    unsigned long long v5;  // rdx
    unsigned long long v15;  // rdx
    unsigned long long v16;  // rcx
    unsigned long long v17;  // r8
    unsigned long long v18;  // r9
    unsigned long long v6;  // rcx
    unsigned long long v7;  // r8
    unsigned long long v8;  // r9
    unsigned long long v10;  // rdx
    unsigned long long v11;  // rcx
    unsigned long long v12;  // r8
    unsigned long long v13;  // r9
    char v0;  // [bp-0xa8]
    unsigned int v1;  // [bp-0x90]
    unsigned long v2;  // [bp-0x80]

    if (!sub_410b3a(g_49fa28, &v0))
    {
        return;
    }
    else if ((*((char *)&g_49f460[9].field_8 + 4) == 0x33 ? (v1 & 0xf000) != 0x2000 : (*((char *)&g_49f460[9].field_8 + 4) == 52 ? (v1 & 0xf000) != 0x6000 : (v1 & 0xf000) != 0x1000)))
    {
        sub_4107be(&g_49fa20.field_0, gettext("File type differs"), v5, v6, v7, v8);
        return;
    }
    else
    {
        if ((*((char *)&g_49f460[9].field_8 + 4) == 0x33 || *((char *)&g_49f460[9].field_8 + 4) == 52) && g_49faa0 != v2)
        {
            sub_4107be(&g_49fa20.field_0, gettext("Device number differs"), v10, v11, v12, v13);
            return;
        }
        if (!(((unsigned short)v1 ^ (unsigned short)g_49fa90) & 0xfff))
            return;
        sub_4107be(&g_49fa20.field_0, gettext("Mode differs"), v15, v16, v17, v18);
        return;
    }
}



// Function: dumpdir_cmp @ 0x113b1
unsigned int dumpdir_cmp(char *a0, char *a1)
{
    unsigned int v4;  // eax
    unsigned int v5;  // eax
    char *node;  // [bp-0x28]
    char *iter;  // [bp-0x20]
    unsigned long v2;  // [bp-0x10]

    iter = a0;
    node = a1;
    while (1)
    {
        while (1)
        {
            do
            {
                if (!*(iter))
                    return *(node);
            } while ((v4 = (unsigned int)(int)(char)(*(iter) - 0x44), 21 < v4));
            v5 = 1 << ((char)v4 & 63);
            if (v5 & 0xffffff & 0x114000)
                return *(node);
            if (v5 & 0xffffff & 0x200400)
            {
                if (!strchr("YN", *(node)))
                    return 1;
                if (strcmp(iter + 1, node + 1))
                    return 1;
                v2 = strlen(iter) + 1;
                iter = &iter[v2];
                node = &node[v2];
            }
            else if ((char)v5 & 1)
            {
                break;
            }
        }
        if (strcmp(iter, node))
            break;
        v2 = strlen(iter) + 1;
        iter = &iter[v2];
        node = &node[v2];
    }
    return 1;
}



// Function: diff_dumpdir @ 0x11512
typedef struct struct_0 {
    long long field_0;
    char padding_8[392];
    char *field_190;
    long long field_198;
    char padding_1a0[8];
    unsigned int field_1a8;
} struct_0;

extern unsigned int g_49fc08;

long long diff_dumpdir(struct_0 *a0)
{
    unsigned long v4;  // rax
    unsigned long v5;  // rax
    unsigned long long v6;  // rcx
    unsigned long long v7;  // r8
    unsigned long long v8;  // r9
    unsigned int v0;  // [bp-0x1c]
    long long v1;  // [bp-0x18]
    char *v2;  // [bp-0x10]

    if (!a0->field_1a8)
    {
        v1 = 0;
        v0 = sub_4157f9(a0->field_198, a0->field_0, g_49fc08, a0->field_0);
        if (v0 < 0)
        {
            v1 = sub_42b2b1;
        }
        else if (fstat(v0, &a0->padding_8[80]))
        {
            v1 = sub_42b444;
            close(v0);
        }
        else
        {
            a0->field_1a8 = v0;
        }
        if (v1)
            return sub_42b48b(a0->field_0, 0, v1);
    }
    v2 = sub_42241b(sub_421eb9(a0));
    if (!v2)
        return sub_410a26(a0, sub_410903);
    v4 = sub_4113b1(a0->field_190, v2);
    if (!(unsigned int)v4)
        return v4;
    v5 = gettext("Contents differ");
    return (unsigned long long)sub_4107be(a0, v5, v5, v6, v7, v8);
}



// Function: diff_multivol @ 0x11650
extern unsigned int g_49c268;
extern unsigned long long g_49f460;
extern char g_49fa20;
extern long long g_49fa28;
extern char g_49fa30;
extern unsigned long long g_49faa8;
extern unsigned int g_49fc08;

void diff_multivol(void)
{
    unsigned long v8;  // rdx
    unsigned long v17;  // r8
    unsigned long v18;  // r9
    long long v19;  // rdi
    long long v20;  // rsi
    unsigned long long v21;  // rdx
    unsigned long long v22;  // rcx
    unsigned long long v23;  // r8
    unsigned long long v24;  // r9
    unsigned long v26;  // rdx
    unsigned long v9;  // rcx
    unsigned long v27;  // rcx
    unsigned long v28;  // r8
    unsigned long v29;  // r9
    long long v30;  // rdi
    long long v31;  // rsi
    unsigned long v10;  // r8
    unsigned long v11;  // r9
    long long v12;  // rdi
    long long v13;  // rsi
    char v14;  // cl
    unsigned long v15;  // rdx
    unsigned long v16;  // rcx
    unsigned int v0;  // [bp-0xb8]
    unsigned int v1;  // [bp-0xb4]
    unsigned long v2;  // [bp-0xb0]
    char v3;  // [bp-0xa8]
    unsigned int v4;  // [bp-0x90]
    unsigned long v5;  // [bp-0x78]

    if (g_49fa30)
    {
        sub_410bb7();
        return;
    }
    else if (!sub_410b3a(g_49fa28, &v3))
    {
        return;
    }
    else if (((unsigned short)v4 & 0xf000) != 0x8000)
    {
        sub_4107be(&g_49fa20, gettext("File type differs"), v8, v9, v10, v11);
        sub_428891(v12, v13, v8, v9, v10, v11);
        return;
    }
    else
    {
        v2 = sub_42798b(g_49f460 + 369, 12);
        if (!(v2 - 0 >> 63 & 1))
        {
            v14 = 0;
            if (__OFADD__(v2, g_49faa8) & 1)
                v14 = 1;
            if (!(v14 & 1) && v5 == v2 + g_49faa8)
            {
                v0 = openat(g_49c268, g_49fa28, g_49fc08, g_49fa28);
                if (v0 < 0)
                {
                    sub_43e38b(g_49fa28);
                    sub_4107be(&g_49fa20, NULL, v15, v16, v17, v18);
                    sub_428891(v19, v20, v15, v16, v17, v18);
                    return;
                }
                if (lseek(v0, v2, 0) < 0)
                {
                    sub_43e7a9(g_49fa28, v2, v2);
                    sub_4107be(&g_49fa20, NULL, v21, v22, v23, v24);
                }
                else
                {
                    sub_410a26(&g_49fa20, sub_41091a);
                }
                v1 = close(v0);
                if (!v1)
                    return;
                sub_43e1ff(g_49fa28);
                return;
            }
        }
        sub_4107be(&g_49fa20, gettext("Size differs"), v26, v27, v28, v29);
        sub_428891(v30, v31, v26, v27, v28, v29);
        return;
    }
}



// Function: diff_archive @ 0x11897
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

typedef struct FILE {
} FILE;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_1 g_480ab4;
extern FILE *g_49efd0;
extern char g_49f320;
extern struct_0 *g_49f460;
extern unsigned int g_49f9ac;
extern char g_49fa20;
extern long long g_49fa28;
extern char g_49fa30;
extern struct_2 *g_49fdc8;
extern unsigned int g_49fdd0;

long long diff_archive(void)
{
    char *v1;  // rax
    unsigned int v2;  // eax

    sub_40bba7(g_49f460);
    if (g_49f9ac)
    {
        if (g_49f320)
        {
            v1 = gettext("Verify ");
            fprintf(g_49efd0, v1);
        }
        sub_42864c(&g_49fa20, g_49f460, -0x1);
    }
    v2 = *((char *)&g_49f460[9].field_8 + 4);
    switch (v2)
    {
    case 0: case 48: case 55: case 83:
        return (!g_49fa30 ? (unsigned long long)sub_410c82() : (unsigned long long)sub_410bb7());
    case 49:
        return (unsigned long long)sub_410fea();
    case 50:
        return (unsigned long long)sub_4110be();
    case 51: case 52: case 54:
        return (unsigned long long)sub_411230();
    case 53: case 68:
        if (!(char)sub_42490b(&g_49fa20))
            return (unsigned long long)sub_410bb7();
        sub_411512(&g_49fa20);
        return (unsigned long long)sub_410bb7();
    case 77:
        return (unsigned long long)sub_411650();
    case 86:
        return *((int *)(4 * v2 + (char *)&g_480ab4.field_0)) + (char *)&g_480ab4.field_0;
    default:
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(g_49fa28);
        error(0, 0, gettext("%s: Unknown file type '%c', diffed as normal file"));
        g_49fdd0 = 2;
        break;
    }
}



// Function: verify_volume @ 0x11a1c
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern unsigned int g_49efb8;
extern char g_49f320;
extern unsigned long long g_49f328;
extern void g_49f460;
extern long long g_49f468;
extern char g_49f8bd;
extern int g_49f9cc;
extern long long g_49fa20;
extern struct_0 *g_49fbd8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void verify_volume(void)
{
    unsigned long v11;  // rcx
    unsigned int v12;  // eax
    unsigned long v13;  // rcx
    unsigned int v14;  // eax
    long long v17;  // rax
    unsigned int v0;  // [bp-0x50]
    unsigned int i;  // [bp-0x4c]
    int v2;  // [bp-0x48]
    unsigned int v3;  // [bp-0x44]
    unsigned short v4;  // [bp-0x40]
    unsigned int v5;  // [bp-0x3c]
    char v6;  // [bp-0x38]

    v0 = 0;
    if ((char)sub_43ed80())
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Archive contains file names with leading prefixes removed."));
        v0 = 1;
    }
    if ((char)sub_43ba20())
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Archive contains transformed file names."));
        v0 = 1;
    }
    if (v0)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Verification may fail to locate original files."));
    }
    sub_420f88();
    if (!g_49f328)
        sub_41075e();
    fsync(g_49f9cc);
    ioctl(g_49f9cc, 587);
    v4 = 2;
    v5 = 1;
    if (g_49f9cc > 0x3fffffff)
    {
        v11 = g_49f9cc - 0x40000000;
        v12 = sub_440254(v11 & 0xffffffff, 1074294017, &v4, v11);
    }
    else
    {
        v12 = ioctl(g_49f9cc, 1074294017);
    }
    v3 = v12;
    if ((char)(v3 >> 31) & 1)
    {
        if (*(__errno_location()) == 5)
        {
            if (g_49f9cc > 0x3fffffff)
            {
                v13 = g_49f9cc - 0x40000000;
                v14 = sub_440254(v13 & 0xffffffff, 1074294017, &v4, v13);
            }
            else
            {
                v14 = ioctl(g_49f9cc, 1074294017);
            }
            v3 = v14;
            if ((char)(v3 >> 31) & 1)
                goto LABEL_411c1f;
        }
        else
        {
LABEL_411c1f:
            if ((g_49f9cc <= 0x3fffffff ? lseek(g_49f9cc, 0, 0) : sub_4400be(g_49f9cc - 0x40000000, 0, 0)))
            {
                sub_43e867(g_49fbd8->field_0);
                return;
            }
        }
    }
    g_49efb8 = 0;
    g_49f320 = 1;
    sub_40f662();
    while (1)
    {
        i = sub_425ecf(&g_49f460, &g_49fa20, 0);
        if (i == 5)
        {
            v2 = 0;
            do
            {
                v2 += 1;
                sub_40bba7(*((long long *)&g_49f460));
                i = sub_425ecf(&g_49f460, &g_49fa20, 0);
            } while (i == 5);
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, ngettext("VERIFY FAILURE: %d invalid header detected", "VERIFY FAILURE: %d invalid headers detected", v2));
            g_49fdd0 = 2;
        }
        if (i == 4)
            break;
        if (i == 3)
        {
            sub_40bba7(*((long long *)&g_49f460));
            if (g_49f8bd != 1)
            {
                i = sub_425ecf(&g_49f460, &g_49fa20, 0);
                if (i == 3)
                    break;
                if ((char)g_49ec84 & 1)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    v17 = sub_40bacd();
                    sub_454e5a(v17, &v6, v17);
                    error(0, 0, gettext("A lone zero block at %s"));
                }
            }
        }
        else
        {
            sub_426747(*((long long *)&g_49f460), &g_49fa20, &g_49f468, 1);
            sub_411897();
            sub_439d1c(&g_49fa20);
        }
    }
    g_49efb8 = 1;
    g_49f320 = 0;
    return;
}



// Function: add_exclusion_tag @ 0x11ea1
typedef struct struct_0 {
    char *field_0;
    char field_8;
    char padding_9[7];
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned long long g_49f330;

struct_0 * add_exclusion_tag(char *a0, unsigned int a1, unsigned long a2)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_460b5b(40);
    idx->field_20 = g_49f330;
    idx->field_0 = a0;
    idx->field_10 = a1;
    idx->field_18 = a2;
    *((unsigned long *)&idx->field_8) = strlen(a0);
    g_49f330 = idx;
    return idx;
}



// Function: exclusion_tag_warning @ 0x11f18
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern unsigned int g_49f9ac;
extern struct_0 *g_49fdc8;

long long exclusion_tag_warning(long long a0, long long a1, unsigned long long a2)
{
    unsigned long v2;  // rax
    unsigned long long v0;  // [bp-0x30]

    v0 = a2;
    if (!g_49f9ac)
        return g_49f9ac;
    v2 = g_49ec84 & 4;
    if (!(unsigned int)v2)
        return v2;
    if (g_49fdc8)
        g_49fdc8();
    sub_45e285(1, a1);
    sub_45e548(a0);
    return (unsigned long long)error(0, 0, gettext("%s: contains a cache directory tag %s; %s"));
}



// Function: check_exclusion_tags @ 0x11fbf
typedef struct struct_0 {
    long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    struct struct_1 *field_18;
    struct struct_0 *field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern struct_0 *g_49f330;
extern unsigned int g_49fc08;

unsigned int check_exclusion_tags(long long a0, long long *a1)
{
    char v4;  // al
    char v0;  // [bp-0x15]
    unsigned int v1;  // [bp-0x14]
    struct_0 *iter;  // [bp-0x10]

    iter = g_49f330;
    while (1)
    {
        if (!iter)
            return 0;
        v1 = sub_4157f9(a0, iter->field_0, g_49fc08, iter->field_0);
        if (v1 >= 0)
        {
            if (!iter->field_18 || (char)iter->field_18(v1))
                v4 = 1;
            else
                v4 = 0;
            v0 = v4;
            v0 &= 1;
            close(v1);
            if (v0)
                break;
        }
        iter = iter->field_20;
    }
    if (a1)
        *(a1) = iter->field_0;
    return iter->field_10;
}



// Function: cachedir_file_p @ 0x12087
long long cachedir_file_p(int a0)
{
    unsigned int v2;  // eax
    char v0;  // [bp-0x48]

    if (read(a0, &v0, 43) == 43 && !memcmp(&v0, "Signature: 8a477f597d28d172789f06886806bc55", 43))
        v2 = 1;
    else
        v2 = 0;
    return v2 & 1;
}



// Function: to_octal @ 0x12105
long long to_octal(unsigned long long a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x18]
    unsigned long long i;  // [bp-0x10]

    v0 = a0;
    i = a2;
    do
    {
        i -= 1;
        *((char *)(i + a1)) = ((char)v0 & 7) + 48;
        v0 >>= 3;
    } while (i);
    return i + a1;
}



// Function: tar_copy_str @ 0x12157
unsigned long long tar_copy_str(unsigned long a0, unsigned long a1, unsigned long long a2)
{
    char *v2;  // rax
    unsigned long long v3;  // rax
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a2; i += 1)
    {
        v2 = i + a0;
        *(v2) = *((char *)(a1 + i));
        v3 = *(v2);
        if (!(char)v3)
            return v3;
    }
    return i;
}



// Function: tar_name_copy_str @ 0x121ac
extern unsigned int g_49f864;

long long tar_name_copy_str(unsigned int a0, unsigned int a1, unsigned long long a2)
{
    sub_412157(a0, a1, a2);
    if (g_49f864 != 2)
        return g_49f864;
    *((char *)(a0 + a2 - 1)) = 0;
    return a0 + a2 - 1;
}



// Function: to_base256 @ 0x121fb
unsigned long long to_base256(int a0, unsigned long long a1, unsigned long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long i;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = a1;
    v3 = -(a0) * 0x100000000000000;
    i = a3;
    do
    {
        v1 = v0;
        i -= 1;
        *((char *)(i + a2)) = v1;
        v0 = v1 >> 8 | v3;
    } while (i);
    return v1 >> 8 | v3;
}



// Function: to_chars_subst @ 0x12262
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f864;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int to_chars_subst(unsigned int a0, unsigned int a1, long long a2, long long a3, unsigned long long *a4, long long a5, unsigned long a6, long long a7)
{
    unsigned long long v0;  // [bp-0x128]
    char *v1;  // [bp-0x120]
    unsigned int v2;  // [bp-0xdc]
    void* v3;  // [bp-0xd8], Other Possible Types: unsigned long long
    void* v4;  // [bp-0xd0], Other Possible Types: unsigned long long
    char *node;  // [bp-0xc8]
    unsigned long v6;  // [bp-0xc0]
    unsigned long long v7;  // [bp-0xb8]
    unsigned long v8;  // [bp-0xb0]
    void* iter1;  // [bp-0xa8]
    void* iter;  // [bp-0xa0]
    unsigned long long v11;  // [bp-0x98]
    unsigned long long v12;  // [bp-0x90]
    char v13;  // [bp-0x88]
    char v14;  // [bp-0x68]
    char v15;  // [bp-0x67]
    char v16;  // [bp-0x47]
    char v17;  // [bp-0x27]

    v6 = (!a1 ? (a6 * 3 - 3 <= 63 ? (1 << ((char)((unsigned int)a6 * 3 - 3) & 63)) - 1 : 0xffffffffffffffff) : ((a6 - 1) * 8 <= 63 ? (1 << ((char)(a6 - 1) * 8 & 63)) - 1 : 0xffffffffffffffff));
    v7 = sub_454e5a(v6, &v13, &v13);
    if (a1)
    {
        v8 = (v6 == 0xffffffffffffffff ? (v6 >> 1) + 1 : v6 + 1);
        iter1 = sub_454e5a(v8, &v16, v8);
        iter1 -= 1;
        *((char *)iter1) = 45;
        v3 = iter1;
    }
    else
    {
        v3 = "0";
    }
    if (a0)
    {
        iter = sub_454e5a(-(a2), &v15, -(a2));
        iter -= 1;
        *((char *)iter) = 45;
        v4 = iter;
    }
    else
    {
        v4 = sub_454e5a(a2, &v14, &v14);
    }
    if (a4)
    {
        v11 = a4(&v2) & v6;
        v2 &= g_49f864 == 6;
        v12 = (!v2 ? v11 : -(v11));
        node = sub_454e5a(v12, &v17, v12);
        if (v2)
        {
            node -= 1;
            *(node) = 45;
        }
        if (g_49fdc8)
            g_49fdc8();
        v1 = node;
        v0 = v7;
        error(0, 0, gettext("value %s out of %s range %s..%s; substituting %s"));
        return sub_41262b(v2, v12, a3, 0, a5, a6, a7);
    }
    else
    {
        if (g_49fdc8)
            g_49fdc8();
        v0 = v7;
        error(0, 0, gettext("value %s out of %s range %s..%s"));
        g_49fdd0 = 2;
        return 0;
    }
}



// Function: to_chars @ 0x1262b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f348;
extern unsigned int g_49f864;
extern struct_0 *g_49fdc8;

unsigned int to_chars(unsigned int a0, unsigned long a1, unsigned long a2, unsigned long long *a3, char *a4, unsigned long long a5, long long a6)
{
    unsigned int v3;  // eax
    unsigned long v4;  // rax
    unsigned long long *v0;  // [bp-0x38]
    unsigned long v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0xc]

    v1 = a1;
    v0 = a3;
    if (g_49f864 == 6 || g_49f864 == 2)
        v3 = 1;
    else
        v3 = 0;
    v2 = v3;
    if (!a0 && (a5 * 3 - 3 <= 63 ? (1 << ((char)((unsigned int)a5 * 3 - 3) & 63)) - 1 : 0xffffffffffffffff) >= v1)
    {
        *(&a4[a5] - 1) = 0;
        sub_412105(v1, a4, a5 - 1);
        return 1;
    }
    if (v2)
    {
        if (a0)
            v4 = ~(v1);
        else
            v4 = v1;
        if (v4 <= ((a5 - 1) * 8 <= 63 ? (1 << ((char)(a5 - 1) * 8 & 63)) - 1 : 0xffffffffffffffff))
        {
            *(a4) = (!a0 ? 128 : 0xff);
            sub_4121fb(a0, v1, a4 + 1, a5 - 1);
            return 1;
        }
        else if (a0 && a2 * 8 <= a5 * 3 - 3)
        {
            if (!g_49f348)
            {
                g_49f348 = 1;
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Generating negative octal headers"));
            }
            *(&a4[a5] - 1) = 0;
            sub_412105((a2 * 8 <= 63 ? (1 << ((char)a2 * 8 & 63)) - 1 : 0xffffffffffffffff) & v1, a4, a5 - 1);
            return 1;
        }
    }
    else
    {
        v0 = NULL;
    }
    return sub_412262(a0, v2, v1, a2, v0, a4, a5, a6);
}



// Function: gid_substitute @ 0x128a8
extern unsigned int g_49f34c;

unsigned int gid_substitute(unsigned int *a0)
{
    unsigned int v0;  // [bp-0xc]

    if (!g_49f34c && !sub_42c261("nobody", &g_49f34c))
        g_49f34c = 0xfffffffe;
    v0 = g_49f34c;
    *(a0) = 0;
    return v0;
}



// Function: gid_to_chars @ 0x12901
unsigned int gid_to_chars(unsigned int a0, char *a1, unsigned long long a2)
{
    return sub_41262b(0, a0, 4, sub_4128a8, a1, a2, "gid_t");
}



// Function: major_to_chars @ 0x12957
unsigned int major_to_chars(unsigned int a0, char *a1, unsigned long long a2)
{
    return sub_41262b((char)(a0 >> 31), a0, 4, NULL, a1, a2, "major_t");
}



// Function: minor_to_chars @ 0x129ae
unsigned int minor_to_chars(unsigned int a0, char *a1, unsigned long long a2)
{
    return sub_41262b((char)(a0 >> 31), a0, 4, NULL, a1, a2, "minor_t");
}



// Function: mode_to_chars @ 0x12a05
extern unsigned int g_49f864;

unsigned int mode_to_chars(unsigned int a0, char *a1, unsigned long long a2)
{
    unsigned int v0;  // [bp-0x14]
    unsigned long v1;  // [bp-0x10]

    if (g_49f864 != 4 && g_49f864 != 3 && g_49f864 != 6)
    {
        v0 = 0;
        v1 = a0;
    }
    else
    {
        v0 = 0;
        v1 = a0 & 0xfff;
    }
    return sub_41262b(v0, v1, 4, NULL, a1, a2, "mode_t");
}



// Function: off_to_chars @ 0x12a9a
unsigned int off_to_chars(unsigned long long a0, char *a1, unsigned long long a2)
{
    return sub_41262b((char)(a0 >> 63), a0, 8, NULL, a1, a2, "off_t");
}



// Function: time_to_chars @ 0x12af2
unsigned int time_to_chars(unsigned long long a0, char *a1, unsigned long long a2)
{
    return sub_41262b((char)(a0 >> 63), a0, 8, NULL, a1, a2, "time_t");
}



// Function: uid_substitute @ 0x12b4a
extern unsigned int g_49f350;

unsigned int uid_substitute(unsigned int *a0)
{
    unsigned int v0;  // [bp-0xc]

    if (!g_49f350 && !sub_42c173("nobody", &g_49f350))
        g_49f350 = 0xfffffffe;
    v0 = g_49f350;
    *(a0) = 0;
    return v0;
}



// Function: uid_to_chars @ 0x12ba3
unsigned int uid_to_chars(unsigned int a0, char *a1, unsigned long long a2)
{
    return sub_41262b(0, a0, 4, sub_412b4a, a1, a2, "uid_t");
}



// Function: uintmax_to_chars @ 0x12bf9
unsigned int uintmax_to_chars(unsigned int a0, char *a1, unsigned long long a2)
{
    return sub_41262b(0, a0, 8, NULL, a1, a2, "uintmax_t");
}



// Function: string_to_chars @ 0x12c4c
long long string_to_chars(unsigned int a0, unsigned int a1, unsigned long long a2)
{
    sub_412157(a1, a0, a2);
    *((char *)(a1 + a2 - 1)) = 0;
    return a1 + a2 - 1;
}



// Function: file_dumpable_p @ 0x12c90
extern char g_49f979;
extern char g_49f999;
extern char g_49f9d0;

unsigned int file_dumpable_p(void* a0)
{
    unsigned long v1;  // rax
    int v2;  // eax
    unsigned int v3;  // eax
    unsigned int v4;  // eax

    if (((short)(int)a0[24] & 0xf000) == 0x4000)
    {
        return 1;
    }
    else if (((short)(int)a0[24] & 0xf000) != 0x8000)
    {
        return 0;
    }
    else if (g_49f9d0)
    {
        if (g_49f999 && g_49f979)
        {
            v1 = (long long)a0[48];
            if ((short)(long long)a0[48] & 0x1ff && (long long)a0[48] + 0x1ff > 0x3fe)
                v2 = 1;
            else
                v2 = 0;
            if ((long long)a0[64] >= v2 + ((long long)((v1 < 0 ? v1 + 0x1ff : v1)) >> 9))
                goto LABEL_412d65;
            v3 = 1;
        }
        else
        {
LABEL_412d65:
            v3 = 0;
        }
        return v3 & 1;
    }
    else
    {
        if ((long long)a0[48] || ((short)(int)a0[24] & 292) != 292)
            v4 = 1;
        else
            v4 = 0;
        return v4 & 1;
    }
}



// Function: write_eot @ 0x12da0
long long write_eot(void)
{
    unsigned long ptr;  // [bp-0x10]

    ptr = sub_40bb4b();
    memset(ptr, 0, 0x200);
    sub_40bba7(ptr);
    ptr = sub_40bb4b();
    memset(ptr, 0, sub_40bbf5(ptr));
    return sub_40bba7(ptr);
}



// Function: start_private_header @ 0x12e0f
long long start_private_header(unsigned int a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x10]

    v0 = a2;
    v1 = sub_40bb4b();
    memset(v1, 0, 0x200);
    sub_4121ac(v1, a0, 100);
    sub_412a9a(a1, v1 + 124, 12);
    if (!(v0 - 0 >> 63 & 1))
    {
        if (v0 > 0x1ffffffff)
            v0 = 0x1ffffffff;
    }
    else
    {
        v0 = 0;
    }
    sub_412af2(v0, v1 + 0x88, 12);
    sub_412a05(33188, v1 + 100, 8);
    sub_412ba3(0, v1 + 108, 8);
    sub_412901(0, v1 + 116, 8);
    memcpy(v1 + 0x101, "ustar", 6);
    memcpy(v1 + 263, "00", 2);
    return v1;
}



// Function: write_short_name @ 0x12f4e
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

long long write_short_name(struct_0 *a0)
{
    unsigned long v0;  // [bp-0x10]

    v0 = sub_40bb4b();
    memset(v0, 0, 0x200);
    sub_4121ac(v0, a0->field_8, 100);
    return v0;
}



// Function: write_gnu_long_link @ 0x12f9f
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

extern unsigned long long g_49f358;
extern unsigned long long g_49f360;
extern char g_49f934;

long long write_gnu_long_link(long long a0, void* a1, char a2)
{
    void* iter;  // [bp-0x38]
    unsigned long n;  // [bp-0x20]
    unsigned long n1;  // [bp-0x18]
    struct_0 *ptr;  // [bp-0x10], Other Possible Types: unsigned long

    iter = a1;
    n = strlen(iter) + 1;
    ptr = sub_412e0f(4722252, n, 0);
    if (g_49f934 != 1)
    {
        if (!g_49f358)
        {
            sub_42bfd1(0, &g_49f358);
            sub_42c0a2(0, &g_49f360);
        }
        sub_412c4c(g_49f358, *((unsigned int *)&(struct struct_0 *)&ptr[1].padding_0[108]), 32);
        sub_412c4c(g_49f360, *((unsigned int *)&(struct struct_0 *)&ptr[1].padding_0[140]), 32);
    }
    strcpy(&ptr[1].padding_0[100], "ustar  ");
    ptr->field_9c = a2;
    sub_41422e(a0, ptr, -0x1, ptr);
    ptr = sub_40bb4b();
    for (n1 = sub_40bbf5(ptr); n1 < n; n1 = sub_40bbf5(ptr))
    {
        memcpy(ptr, iter, n1);
        iter += n1;
        n -= n1;
        sub_40bba7(ptr + (n1 - 1 & 0xfffffffffffffe00));
        ptr = sub_40bb4b();
    }
    memcpy(ptr, iter, n);
    memset(ptr + n, 0, n1 - n);
    return sub_40bba7(ptr + (n - 1 & 0xfffffffffffffe00));
}



// Function: split_long_name @ 0x13197
unsigned long long split_long_name(char *a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long i;  // [bp-0x10]

    v0 = a1;
    if (v0 > 156)
    {
        v0 = 156;
    }
    else if (*(&a0[v0] - 1) == 47)
    {
        v0 -= 1;
    }
    for (i = v0 - 1; i; i -= 1)
    {
        if (a0[i] == 47)
            return i;
    }
    return i;
}



// Function: write_ustar_long_name @ 0x1320b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long write_ustar_long_name(unsigned long ptr)
{
    unsigned long len;  // [bp-0x38]
    unsigned long n;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    unsigned long ptr1;  // [bp-0x20]

    len = strlen(ptr);
    if (len > 0x100)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(ptr);
        error(0, 0, gettext("%s: file name is too long (max %d); not dumped"));
        g_49fdd0 = 2;
        return 0;
    }
    else
    {
        n = sub_413197(ptr, len);
        if (n)
        {
            v2 = len - n - 1;
            if (v2 <= 100 && v2)
            {
                ptr1 = sub_40bb4b();
                memset(ptr1, 0, 0x200);
                memcpy(ptr1 + 345, ptr, n);
                memcpy(ptr1, n + 1 + ptr, len - n - 1);
                return ptr1;
            }
        }
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(ptr);
        error(0, 0, gettext("%s: file name is too long (cannot be split); not dumped"));
        g_49fdd0 = 2;
        return 0;
    }
}



// Function: write_long_link @ 0x133a8
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern char g_49f864;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long write_long_link(void* a0)
{
    unsigned int v1;  // eax
    unsigned long v2;  // rax

    if (6 >= *((int *)&g_49f864))
    {
        v1 = 1 << ((char)*((int *)&g_49f864) & 63);
        if ((char)v1 & 42)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548((long long)a0[24]);
            v2 = (unsigned long long)error(0, 0, gettext("%s: link name is too long; not dumped"));
            g_49fdd0 = 2;
            return v2;
        }
        else if ((char)v1 & 0x44)
        {
            return sub_412f9f(a0, (long long)a0[24], 75);
        }
        else if ((char)v1 & 16)
        {
            return sub_41e1d3("linkpath", a0, 0);
        }
    }
    abort(); /* do not return */
}



// Function: write_long_name @ 0x134b6
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f864;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long write_long_name(void* a0)
{
    switch (g_49f864)
    {
    case 1:
        if (strlen((long long)a0[8]) > 99)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548((long long)a0[8]);
            error(0, 0, gettext("%s: file name is too long (max %d); not dumped"));
            g_49fdd0 = 2;
            return 0;
        }
        break;
    case 2: case 6:
        sub_412f9f(a0, (long long)a0[8], 76);
        break;
    case 3: case 5:
        return sub_41320b((long long)a0[8]);
    case 4:
        sub_41e1d3("path", a0, 0);
        break;
    default:
        abort(); /* do not return */
    }
    return sub_412f4e(a0);
}



// Function: write_extended @ 0x135e3
extern unsigned int g_49f910;
extern unsigned long long g_49f920;
extern unsigned long long g_49f9e0;

long long write_extended(char a0, unsigned long long *a1, unsigned long a2)
{
    unsigned int v0;  // [bp-0x234]
    unsigned long v1;  // [bp-0x230]
    unsigned long v2;  // [bp-0x228]
    unsigned long v3;  // [bp-0x220]
    char v4;  // [bp-0x218]

    if (a1[47] || !a1[45])
        return a2;
    sub_41e672(a1 + 45);
    memcpy(&v4, a2, 0x200);
    if (a0)
    {
        v0 = 103;
        v1 = sub_41d18d();
        v2 = g_49f9e0;
    }
    else
    {
        v0 = 120;
        v1 = sub_41d123(a1);
        v2 = (!g_49f910 ? a1[22] : g_49f920);
    }
    sub_41d280((char)v0, v1, v2, a1 + 45);
    free(v1);
    v3 = sub_40bb4b();
    memcpy(v3, &v4, 0x200);
    return v3;
    return a2;
}



// Function: write_header_name @ 0x13775
extern unsigned int g_49f864;

long long write_header_name(void* a0)
{
    if (g_49f864 == 4 && (char)sub_43c9b7((long long)a0[8]) != 1)
    {
        sub_41e1d3("path", a0, 0);
        return sub_412f4e(a0);
    }
    return ((g_49f864 == 2 ? 99 : 100) < strlen((long long)a0[8]) ? sub_4134b6(a0) : sub_412f4e(a0));
}



// Function: start_header @ 0x1381d
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

extern char g_49f864;
extern char g_49f8be;
extern long long g_49f8f0;
extern unsigned int g_49f8f8;
extern unsigned int g_49f910;
extern unsigned long long g_49f920;
extern unsigned long long g_49f928;
extern char g_49f934;
extern int g_49f964;
extern int g_49f968;
extern int g_49f96c;

struct_0 * start_header(void* idx)
{
    unsigned int v12;  // eax
    unsigned int v0;  // [bp-0x70]
    unsigned int v1;  // [bp-0x6c]
    unsigned int v2;  // [bp-0x68]
    unsigned int v3;  // [bp-0x64]
    unsigned long v4;  // [bp-0x60]
    unsigned long v5;  // [bp-0x58]
    unsigned long long v6;  // [bp-0x50]
    struct_0 *v7;  // [bp-0x48]
    unsigned long v8;  // [bp-0x40]
    unsigned long long i;  // [bp-0x38]
    unsigned long long v10;  // [bp-0x30]

    v4 = 0;
    v5 = 0;
    v7 = sub_413775(idx);
    if (!v7)
        return NULL;
    sub_429274((int)idx[116], idx + 116, &v4, idx + 116);
    sub_4293ce((int)idx[120], idx + 120, &v5, idx + 120);
    if (g_49f8f0)
    {
        v12 = sub_455b51((int)idx[112], ((int)idx[112] & 0xf000) == 0x4000, g_49f8f8, g_49f8f0, 0);
        *((unsigned int *)&idx[112]) = (int)idx[112] & 0xfffff000 | v12;
    }
    if (*((int *)&g_49f864) == 1 || *((int *)&g_49f864) == 3)
        sub_412a05((int)idx[112] & 0xfff, &v7->padding_0[100], 8);
    else
        sub_412a05((int)idx[112], &v7->padding_0[100], 8);
    v0 = (int)idx[116];
    if (*((int *)&g_49f864) == 4 && v0 > 0x1fffff)
    {
        sub_41e1d3("uid", idx, 0);
        v0 = 0;
    }
    if ((char)sub_412ba3(v0, &v7->padding_0[108], 8) != 1)
        return NULL;
    v1 = (int)idx[120];
    if (*((int *)&g_49f864) == 4 && v1 > 0x1fffff)
    {
        sub_41e1d3("gid", idx, 0);
        v1 = 0;
    }
    if ((char)sub_412901(v1, &v7->padding_0[116], 8) != 1)
        return NULL;
    v6 = (long long)idx[0x88];
    if (*((int *)&g_49f864) == 4 && v6 > 0x1ffffffff)
    {
        sub_41e1d3("size", idx, 0);
        v6 = 0;
    }
    if ((char)sub_412a9a(v6, &v7->padding_0[124], 12) != 1)
        return NULL;
    if (g_49f910 == 2)
    {
        if (sub_45ff90((long long)idx[248], (long long)idx[0x100], g_49f920, g_49f928) <= 0)
        {
            i = (long long)idx[248];
            v10 = (long long)idx[0x100];
        }
        else
        {
            i = g_49f920;
            v10 = g_49f928;
        }
    }
    else if (g_49f910 <= 2)
    {
        if (!g_49f910)
        {
            i = (long long)idx[248];
            v10 = (long long)idx[0x100];
        }
        else if (g_49f910 == 1)
        {
            i = g_49f920;
            v10 = g_49f928;
        }
    }
    if (*((int *)&g_49f864) == 4)
    {
        if (i > 0x1ffffffff || v10)
            sub_41e1d3("mtime", idx, &i);
        if (i > 0x1ffffffff)
            i = 0;
    }
    if ((char)sub_412af2(i, &v7->padding_0[0x88], 12) != 1)
        return NULL;
    if (((short)(int)idx[112] & 0xf000) == 0x2000 || ((short)(int)idx[112] & 0xf000) == 0x6000)
    {
        v2 = gnu_dev_major((long long)idx[128]);
        v3 = gnu_dev_minor((long long)idx[128]);
        if (*((int *)&g_49f864) == 4 && v2 > 0x1fffff)
        {
            sub_41e1d3("devmajor", idx, 0);
            v2 = 0;
        }
        if ((char)sub_412957(v2, &v7[2].padding_0[15], 8) != 1)
            return NULL;
        if (*((int *)&g_49f864) == 4 && v3 > 0x1fffff)
        {
            sub_41e1d3("devminor", idx, 0);
            v3 = 0;
        }
        if ((char)sub_4129ae(v3, &v7[2].padding_0[23], 8) != 1)
            return NULL;
    }
    else if (*((int *)&g_49f864) != 6 && *((int *)&g_49f864) != 2 && ((char)sub_412957(0, &v7[2].padding_0[15], 8) != 1 || (char)sub_4129ae(0, &v7[2].padding_0[23], 8) != 1))
    {
        return NULL;
    }
    if (*((int *)&g_49f864) == 4)
    {
        sub_41e1d3("atime", idx, 0);
        sub_41e1d3("ctime", idx, 0);
    }
    else if (g_49f8be && (*((int *)&g_49f864) == 2 || *((int *)&g_49f864) == 6))
    {
        sub_412af2((long long)idx[232], &v7[2].padding_0[31], 12);
        sub_412af2((long long)idx[264], &v7[2].padding_0[43], 12);
    }
    v7->field_9c = (*((int *)&g_49f864) == 1 ? 0 : 48);
    if (*((int *)&g_49f864) != 6)
    {
        if (*((int *)&g_49f864) > 6)
            abort(); /* do not return */
        if (*((int *)&g_49f864) > 4)
            abort(); /* do not return */
        if (*((int *)&g_49f864) >= 3)
        {
            memcpy(&v7[1].padding_0[100], "ustar", 6);
            memcpy(&v7[1].padding_0[106], "00", 2);
        }
        else if (*((int *)&g_49f864) != 1)
        {
            if (*((int *)&g_49f864) != 2)
                abort(); /* do not return */
            goto LABEL_413e77;
        }
    }
    else
    {
LABEL_413e77:
        strcpy(&v7[1].padding_0[100], "ustar  ");
    }
    if (*((int *)&g_49f864) != 1 && !g_49f934)
    {
        if (v4)
            *((unsigned long long *)&idx[32]) = sub_460cc6(v4);
        else
            sub_42bfd1((int)idx[116], idx + 32, idx + 32);
        if (v5)
            *((unsigned long long *)&idx[40]) = sub_460cc6(v5);
        else
            sub_42c0a2((int)idx[120], idx + 40, idx + 40);
        if (*((int *)&g_49f864) == 4 && (strlen((long long)idx[32]) > 32 || (char)sub_43c9b7((long long)idx[32]) != 1))
            sub_41e1d3("uname", idx, 0);
        sub_412c4c((long long)idx[32], *((unsigned int *)&(struct struct_0 *)&v7[1].padding_0[108]), 32);
        if (*((int *)&g_49f864) == 4 && (strlen((long long)idx[40]) > 32 || (char)sub_43c9b7((long long)idx[40]) != 1))
            sub_41e1d3("gname", idx, 0);
        sub_412c4c((long long)idx[40], *((unsigned int *)&(struct struct_0 *)&v7[1].padding_0[140]), 32);
    }
    if (*((int *)&g_49f864) != 4)
        return v7;
    if (g_49f968 > 0)
    {
        if ((long long)idx[56])
            sub_41e1d3("SCHILY.acl.access", idx, 0);
        if ((long long)idx[72])
            sub_41e1d3("SCHILY.acl.default", idx, 0);
    }
    if (g_49f964 > 0 && (long long)idx[48])
        sub_41e1d3("RHT.security.selinux", idx, 0);
    if (g_49f96c > 0)
    {
        i = 0;
        for (v8 = (long long)idx[352]; (long long)idx[344] > i; i += 1)
        {
            sub_41e1d3(*((long long *)(v8 + i * 24)), idx, &i, idx);
        }
    }
    return v7;
}



// Function: simple_finish_header @ 0x1418c
long long simple_finish_header(char *a0)
{
    char *v5;  // rax
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x18]
    char *v3;  // [bp-0x10]

    memcpy(a0 + 148, "        ", 8);
    v0 = 0;
    v3 = a0;
    v1 = 0x200;
    while (1)
    {
        v2 = v1 - 1;
        if (!v1)
            break;
        v5 = v3;
        v3 = v5 + 1;
        v0 += *(v5);
        v1 = v2;
    }
    sub_412bf9(v0, a0 + 148, 7);
    return sub_40bba7(a0);
}



// Function: finish_header @ 0x1422e
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

extern unsigned int g_49f468;
extern unsigned int g_49f864;
extern unsigned int g_49f9ac;

long long finish_header(unsigned long a0, struct_0 *a1, long long a2)
{
    struct_0 *v0;  // [bp-0x18], Other Possible Types: unsigned long

    v0 = a1;
    if (g_49f9ac && v0->field_9c != 75 && v0->field_9c != 76 && v0->field_9c != 120 && v0->field_9c != 103)
    {
        g_49f468 = g_49f864;
        sub_42864c(a0, v0, a2, v0);
    }
    v0 = sub_4135e3(0, a0, v0);
    return sub_41418c(v0);
}



// Function: pad_archive @ 0x142d7
long long pad_archive(long long a0)
{
    long long v3;  // rax
    long long i;  // [bp-0x20]
    unsigned long v1;  // [bp-0x10]

    for (i = a0; i > 0; i -= 0x200)
    {
        v1 = sub_40bb4b();
        memset(v1, 0, 0x200);
        v3 = sub_40bba7(v1);
    }
    return v3;
}



// Function: dump_regular_file @ 0x14327
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

extern unsigned int g_49ec84;
extern char g_49f8bc;
extern struct_0 *g_49fdc8;

unsigned int dump_regular_file(unsigned int a0, void* idx)
{
    unsigned long v7;  // rax
    long long v8;  // rcx
    unsigned long v0;  // [bp-0x60]
    unsigned long v1;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    struct_0 *v3;  // [bp-0x48], Other Possible Types: unsigned long
    unsigned long i;  // [bp-0x40], Other Possible Types: unsigned long long
    char v5;  // [bp-0x38]

    v0 = (long long)idx[0x88];
    v2 = sub_40bacd();
    v3 = sub_41381d(idx);
    if (!v3)
        return 2;
    sub_41422e(idx, v3, v2);
    sub_40ab09((long long)idx[8], (long long)idx[0x88], (long long)idx[0x88]);
    do
    {
        if (v0 <= NULL)
            return 0;
        v3 = sub_40bb4b();
        v1 = sub_40bbf5(v3);
        if (v1 > v0)
        {
            v1 = v0;
            i = (unsigned int)v1 & 0x1ff;
            if (i)
                memset(v3 + v0, 0, 0x200 - i);
        }
        if (a0 > 0)
            v7 = sub_42a9e9(a0, v3, v1, v3);
        else
            v7 = v1;
        i = v7;
        if (i == 0xffffffffffffffff)
        {
            v8 = (long long)idx[0x88] - v0;
            sub_42b2f8(*((long long *)idx), v8, v1, v8);
            sub_4142d7(v0);
            return 1;
        }
        v0 -= i;
        sub_40bba7(v3 + (v1 - 1 & 0xfffffffffffffe00));
    } while (i == v1);
    memset(v3 + i, 0, v1 - i);
    if ((char)g_49ec84 & 128)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_454e5a(v0, &v5, &v5);
        sub_45e548(*((long long *)idx));
        error(0, 0, ngettext("%s: File shrank by %s byte; padding with zeros", "%s: File shrank by %s bytes; padding with zeros", v0));
    }
    if (g_49f8bc != 1)
        sub_439e98(1);
    sub_4142d7(v0 + i - v1);
    return 1;
}



// Function: dump_dir0 @ 0x145e3
typedef struct struct_1 {
    char padding_0[156];
    char field_9c;
} struct_1;

typedef struct struct_0 {
    char padding_0[88];
    unsigned long long field_58;
} struct_0;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned int g_49ec84;
extern struct_2 *g_49f6b0;
extern unsigned int g_49f864;
extern char g_49f8be;
extern unsigned int g_49f930;
extern char g_49f935;
extern unsigned int g_49f9ac;
extern struct_3 *g_49fdc8;

void dump_dir0(void* a0, unsigned long a1)
{
    unsigned int v18;  // eax
    unsigned long v19;  // rax
    unsigned long len;  // rax
    unsigned long v21;  // rax
    char v0;  // [bp-0x91]
    unsigned long v1;  // [bp-0x90]
    unsigned long i;  // [bp-0x88]
    unsigned long n;  // [bp-0x80]
    unsigned long node;  // [bp-0x78]
    unsigned long ptr1;  // [bp-0x70]
    unsigned long v6;  // [bp-0x68]
    unsigned long iter;  // [bp-0x60]
    struct_1 *ptr;  // [bp-0x58], Other Possible Types: unsigned long, unsigned long long
    long long v9;  // [bp-0x50]
    unsigned long v10;  // [bp-0x48]
    unsigned long v11;  // [bp-0x40]
    unsigned long long v12;  // [bp-0x38]
    unsigned long len1;  // [bp-0x30]
    unsigned long len2;  // [bp-0x28]

    v0 = !(long long)a0[408];
    ptr = 0;
    v9 = sub_40bacd();
    *((unsigned long long *)&a0[0x88]) = 0;
    ptr = sub_41381d(a0);
    if (!ptr)
        return;
    sub_417909(a0);
    if (g_49f8be && g_49f864 != 4)
        ptr->field_9c = 0x44;
    else
        ptr->field_9c = 53;
    if (g_49f8be != 1)
    {
        sub_41422e(a0, ptr, v9);
    }
    else if (g_49f6b0[7].field_0)
    {
        if (g_49f864 == 4)
        {
            sub_41e1d3("GNU.dumpdir", a0, sub_422456(g_49f6b0[7].field_0));
            sub_41422e(a0, ptr, v9);
            return;
        }
        v9 = sub_40bacd();
        v10 = sub_422456(g_49f6b0[7].field_0);
        v11 = sub_420bfe(v10);
        sub_412a9a(v11, &ptr->padding_0[124], 12);
        sub_41422e(a0, ptr, v9);
        node = v10;
        i = v11;
        sub_40ab09((long long)a0[8], v11, v11);
        while (i > NULL)
        {
            ptr = sub_40bb4b();
            n = sub_40bbf5(ptr);
            if (n > i)
            {
                n = i;
                v12 = (unsigned int)n & 0x1ff;
                if (v12)
                    memset(i + ptr, 0, 0x200 - v12);
            }
            memcpy(ptr, node, n);
            i -= n;
            node += n;
            sub_40bba7(ptr + (n - 1 & 0xfffffffffffffe00));
        }
        return;
    }
    if (!g_49f930)
        return;
    if (g_49f935 && v0 != 1 && *((long long *)((long long)a0[408] + 88)) != (long long)a0[88])
    {
        if (!g_49f9ac || !(g_49ec84 & 0xffffff & 0x40000))
            return;
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(*((long long *)a0));
        error(0, 0, gettext("%s: file is on a different filesystem; not dumped"));
        return;
    }
    v18 = sub_411fbf(a0, &v1);
    if (v18 == 3)
    {
        return;
    }
    else if (v18 > 3)
    {
        return;
    }
    else if (v18 == 2)
    {
        v21 = gettext("contents not dumped");
        sub_411f18(*((long long *)a0), v1, v21);
        return;
    }
    else if (v18 > 2)
    {
        return;
    }
    else if (!v18)
    {
        ptr1 = sub_460cc6(*((long long *)a0));
        len1 = strlen(ptr1);
        v6 = len1;
        iter = a1;
        while (1)
        {
            len2 = strlen(iter);
            if (!len2)
                break;
            if (v6 < len2 + len1)
            {
                v6 = len2 + len1;
                ptr1 = sub_460b8d(ptr1, v6 + 1, v6 + 1);
            }
            strcpy(ptr1 + len1, iter);
            if ((char)sub_417c43(ptr1, a0, a0) != 1)
                sub_416a25(a0, iter, ptr1, iter);
            iter += len2 + 1;
        }
        free(ptr1);
        return;
    }
    else if (v18 == 1)
    {
        v19 = gettext("contents not dumped");
        sub_411f18(*((long long *)a0), v1, v19);
        len = strlen(*((long long *)a0));
        v6 = strlen(v1) + len + 1;
        ptr1 = sub_460b5b(v6);
        strcpy(ptr1, *((long long *)a0));
        strcat(ptr1, v1);
        sub_416a25(a0, v1, ptr1, v1);
        free(ptr1);
        return;
    }
    else
    {
        return;
    }
}



// Function: ensure_slash @ 0x14bcf
long long ensure_slash(void* a0)
{
    unsigned long v3;  // rdx
    unsigned long len;  // [bp-0x10]
    unsigned long v1;  // [bp-0x10]

    for (len = strlen(*((long long *)a0)); len && *((char *)(len - 1 + *((long long *)a0))) == 47; len -= 1);
    if (*((char *)(len + *((long long *)a0))) != 47)
        *((unsigned long long *)a0) = sub_460b8d(*((long long *)a0), len + 2, len + 2);
    v1 = len + 1;
    *((char *)(len + *((long long *)a0))) = 47;
    v3 = *((long long *)a0);
    *((char *)(v1 + v3)) = 0;
    return v1 + v3;
}



// Function: open_failure_recover @ 0x14c7c
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
    struct_0 *iter;  // [bp-0x10]

    if (*(__errno_location()) == 24 && a0 && a0->field_198)
    {
        for (iter = a0->field_198->field_198; iter; iter = iter->field_198)
        {
            if (iter->field_1a8 > 0 && (!iter->field_198 || iter->field_198->field_1a8 <= 0))
            {
                sub_439c7e(iter);
                return 1;
            }
        }
        *(__errno_location()) = 24;
    }
    return 0;
}



// Function: get_directory_entries @ 0x14d3d
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

extern unsigned int g_49fc2c;

unsigned long long get_directory_entries(struct_1 *a0)
{
    while (1)
    {
        *((struct DIR **)&a0[1].padding_0[0]) = fdopendir(*((int *)&a0[1].padding_0[8]));
        if (*((long long *)&a0[1].padding_0[0]))
        {
            return sub_45eee2(*((long long *)&a0[1].padding_0[0]), g_49fc2c, g_49fc2c);
        }
        else if ((char)sub_414c7c(a0) != 1)
        {
            return 0;
        }
    }
}



// Function: dump_dir @ 0x14db4
unsigned int dump_dir(long long *a0)
{
    unsigned long v0;  // [bp-0x10]

    v0 = sub_414d3d(a0);
    if (v0)
    {
        sub_4145e3(a0, v0);
        sub_41588e(a0);
        free(v0);
        return 1;
    }
    sub_42b3a4(*(a0));
    return 0;
}



// Function: create_archive @ 0x14e23
typedef struct struct_0 {
    char padding_0[16];
    long long field_10;
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

extern unsigned int g_49c268;
extern unsigned long long g_49f338;
extern unsigned int g_49f6a0;
extern char g_49f8a0;
extern char g_49f8be;
extern unsigned long long g_49f8e0;
extern unsigned int g_49fc0c;

void create_archive(void)
{
    int v13;  // eax
    long long v15;  // rdi
    long long v16;  // rsi
    long long v17;  // rdx
    long long v18;  // rcx
    long long v19;  // r8
    long long v20;  // r9
    unsigned int v0;  // [bp-0x204]
    unsigned long v1;  // [bp-0x200]
    unsigned long ptr;  // [bp-0x1f8]
    char *iter;  // [bp-0x1f0]
    unsigned long len;  // [bp-0x1e8]
    unsigned long v5;  // [bp-0x1e8]
    long long v6;  // [bp-0x1e0]
    struct_0 *v7;  // [bp-0x1d8], Other Possible Types: void*
    unsigned long len1;  // [bp-0x1d0]
    unsigned long v9;  // [bp-0x1c8]
    stat v10;  // [bp-0x170]
    unsigned int v11;  // [bp-0x20]

    if (g_49f6a0 != 2 && g_49f8a0 != 1)
        v13 = 1;
    else
        v13 = 0;
    g_49f338 = v13;
    sub_40f694(1);
    sub_40ad11();
    if (g_49f8be)
    {
        v1 = 0;
        ptr = 0;
        sub_42e719();
        while (1)
        {
            v7 = sub_42eda0();
            if (!v7)
                break;
            if ((char)sub_417c43(v7->field_10, 0) != 1)
                sub_416a25(0, v7->field_10, v7->field_10);
        }
        sub_42ee43();
        while (1)
        {
            v7 = sub_42eda0();
            if (!v7)
                break;
            if ((char)sub_417c43((long long)v7[16], 0) != 1)
            {
                for (len = strlen((long long)v7[16]); len >= v1; ptr = sub_460bd8(ptr, &v1, &v1));
                memcpy(ptr, (long long)v7[16], len);
                if (*((char *)(ptr + len - 1)) != 47)
                {
                    v5 = len + 1;
                    *((char *)(len + ptr)) = 47;
                    len = v5;
                }
                sub_439c55(&v9);
                if (iter)
                {
                    for (iter = sub_42241b((long long)v7[56]); *(iter); iter = &iter[1 + len1])
                    {
                        len1 = strlen(iter);
                        if (*(iter) == 89)
                        {
                            if (!v9)
                            {
                                v0 = openat(g_49c268, (long long)v7[16], g_49fc0c, (long long)v7[16]);
                                if (v0 < 0)
                                {
                                    sub_42b48b((long long)v7[16], !(long long)v7[64], sub_42b2b1, !(long long)v7[64]);
                                    break;
                                }
                                else
                                {
                                    v11 = v0;
                                    if (fstat(v0, &v10))
                                    {
                                        sub_42b48b((long long)v7[16], !(long long)v7[64], sub_42b444, !(long long)v7[64]);
                                        break;
                                    }
                                    else
                                    {
                                        v9 = sub_460cc6((long long)v7[16]);
                                    }
                                }
                            }
                            for (; len + len1 > v1; ptr = sub_460bd8(ptr, &v1, &v1));
                            strcpy(len + ptr, iter + 1);
                            sub_416a25(&v9, iter + 1, ptr, iter + 1);
                        }
                    }
                }
                sub_439d1c(&v9);
            }
        }
        free(ptr);
    }
    else
    {
        while (1)
        {
            v6 = sub_42d37c(1);
            if (!v6)
                break;
            if (!((char)sub_417c43(v6, 0) ^ 1))
                continue;
            sub_416a25(0, v6, v6);
        }
    }
    sub_412da0();
    sub_40d046();
    sub_43be6b(v15, v16, v17, v18, v19, v20);
    if (!g_49f8e0)
        return;
    sub_424600();
    return;
}



// Function: hash_link @ 0x1529f
long long hash_link(unsigned long long *a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0[1] ^ *(a0);
    return v0 % a1;
}



// Function: compare_links @ 0x152df
long long compare_links(unsigned long long *a0, unsigned long long *a1)
{
    return _INSERT(a1[1] ^ a0[1] | *(a0) ^ *(a1), 0, !(a1[1] ^ a0[1]) && !(*(a0) ^ *(a1)));
}



// Function: unknown_file_error @ 0x15331
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern char g_49f8bc;
extern struct_0 *g_49fdc8;

unsigned long long unknown_file_error(long long a0)
{
    unsigned long long v2;  // rax

    if ((char)g_49ec84 & 32)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(a0);
        error(0, 0, gettext("%s: Unknown file type; file ignored"));
    }
    v2 = g_49f8bc ^ 1;
    if ((char)v2)
        v2 = sub_439e98(2);
    return v2;
}



// Function: dump_hard_link @ 0x153bb
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

extern char g_49f338;
extern long long g_49f340;
extern unsigned int g_49f864;
extern char g_49f878;
extern char g_49f94e;

unsigned int dump_hard_link(struct_0 *idx)
{
    struct_1 *index;  // [bp-0x68]
    unsigned long v1;  // [bp-0x60]
    long long v2;  // [bp-0x58]
    struct_0 *v3;  // [bp-0x50]
    unsigned long v4;  // [bp-0x48]
    unsigned long v5;  // [bp-0x40]

    if (g_49f340 && (idx->field_68 > *((long long *)&g_49f338) || g_49f94e))
    {
        v5 = idx->field_60;
        v4 = idx->field_58;
        index = sub_452567(g_49f340, &v4, &v4);
        if (index)
        {
            v1 = sub_43edd9(index + 1, 1, g_49f878, index + 1);
            if (index->field_10)
                index->field_10 = index->field_10 - 1;
            v2 = sub_40bacd();
            sub_429547(&idx->padding_8[16], v1, &idx->padding_8[16]);
            if ((g_49f864 == 2 ? 99 : 100) < strlen(v1))
                sub_4133a8(idx);
            idx->field_88 = 0;
            v3 = sub_41381d(idx);
            if (!v3)
                return 0;
            sub_412157(&v3[1].padding_8[5], v1, 100);
            v3[1].padding_8[4] = 49;
            sub_41422e(idx, v3, v2);
            if (!g_49f94e)
                return 1;
            sub_43bebb(idx->field_0, 0);
            return 1;
        }
    }
    return 0;
}



// Function: file_count_links @ 0x15598
typedef struct struct_0 {
    long long field_0;
    char padding_8[80];
    unsigned long long field_58;
    unsigned long long field_60;
    unsigned long long field_68;
} struct_0;

extern char g_49f338;
extern long long g_49f340;
extern char g_49f878;
extern char g_49f8a1;

long long file_count_links(struct_0 *index)
{
    long long v5;  // rax
    unsigned long v6;  // fs
    unsigned long ptr;  // [bp-0x28]
    unsigned long long *idx;  // [bp-0x20]
    unsigned long long *v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    if (g_49f8a1 || index->field_68 <= *((long long *)&g_49f338))
        return v3 - *((long long *)(40 + v6));
    ptr = 0;
    v5 = sub_43edd9(index->field_0, 1, g_49f878);
    sub_429547(&ptr, v5, v5);
    sub_43b9f2(&ptr, 2);
    idx = sub_460b5b(strlen(ptr) + 25);
    idx[1] = index->field_60;
    *(idx) = index->field_58;
    idx[2] = index->field_68;
    strcpy(idx + 3, ptr);
    free(ptr);
    if (!g_49f340 && !(g_49f340 = (long long)(unsigned long long)sub_452c1c(0, 0, sub_41529f, sub_4152df, 0), g_49f340) || !(v2 = (unsigned long long *)(unsigned long long)sub_45395d(g_49f340, idx, idx), v2))
        sub_41783c(); /* do not return */
    if (v2 != idx)
        abort(); /* do not return */
    idx[2] = idx[2] - 1;
    return v3 - *((long long *)(40 + v6));
}



// Function: check_links @ 0x1573f
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern struct_0 *g_49f340;
extern struct_2 *g_49fdc8;

struct_0 * check_links(void)
{
    struct_0 *v2;  // rax
    struct_0 *i;  // [bp-0x20]

    if (!g_49f340)
        return g_49f340;
    for (i = sub_4525fe(g_49f340); i; i = sub_45265e(g_49f340, i, i))
    {
        if (i->field_10)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e84e(i + 1);
            error(0, 0, gettext("Missing links to %s."));
        }
    }
    return v2;
}



// Function: subfile_open @ 0x157f9
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

extern char g_481099;
extern unsigned int g_49c268;
extern char g_49f368;

int subfile_open(struct_1 *a0, long long a1, unsigned int a2)
{
    unsigned int v0;  // [bp-0xc]

    if (g_49f368 != 1)
    {
        g_49f368 = 1;
        strerror(2);
        gettext(&g_481099);
    }
    while (1)
    {
        v0 = openat((!a0 ? g_49c268 : *((int *)&a0[1].padding_0[8])), a1, a2, a1);
        if (v0 >= 0)
        {
            return v0;
        }
        else if (!(char)sub_414c7c(a0))
        {
            return v0;
        }
    }
}



// Function: restore_parent_fd @ 0x1588e
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

extern unsigned int g_49c268;
extern unsigned int g_49fc0c;

void restore_parent_fd(struct_1 *a0)
{
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    unsigned int v0;  // [bp-0xb8]
    unsigned int v1;  // [bp-0xb4]
    struct_0 *v2;  // [bp-0xb0]
    stat v3;  // [bp-0xa8]

    v2 = a0->field_198;
    if (!v2)
    {
        return;
    }
    else if (!v2->field_1a8)
    {
        v0 = openat(a0->field_1a8, "..", g_49fc0c, "..");
        if (v0 < 0)
        {
            v0 = -(*(__errno_location()));
        }
        else
        {
            v5 = fstat(v0, &v3);
            if (v5 || *((long long *)&v2->padding_60[0]) != v3.st_ino || v2->field_58 != *((unsigned long long *)&v3))
            {
                close(v0);
                v0 = 2;
            }
        }
        if (v0 < 0)
        {
            v1 = openat(g_49c268, v2->field_0, g_49fc0c, v2->field_0);
            if (v1 >= 0)
            {
                v6 = fstat(v0, &v3);
                if (!v6 && *((long long *)&v2->padding_60[0]) == v3.st_ino && v2->field_58 == *((unsigned long long *)&v3))
                    v0 = v1;
                else
                    close(v1);
            }
        }
        v2->field_1a8 = v0;
        return;
    }
    else
    {
        return;
    }
}



// Function: dump_file0 @ 0x15a63
typedef struct struct_2 {
    char padding_0[156];
    char field_9c;
} struct_2;

typedef struct struct_0 {
    char padding_0[424];
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

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned int g_49c268;
extern unsigned int g_49ec84;
extern unsigned int g_49f864;
extern char g_49f878;
extern unsigned int g_49f87c;
extern unsigned int g_49f880;
extern char g_49f8be;
extern char g_49f8c8;
extern long long g_49f900;
extern long long g_49f908;
extern char g_49f94e;
extern char g_49f979;
extern unsigned int g_49f9ac;
extern unsigned int g_49fc08;
extern unsigned int g_49fc10;
extern struct_1 *g_49fdc8;

long long dump_file0(void* idx, long long a1, long long a2)
{
    long long v17;  // rax
    void* idx1;  // rbx
    unsigned long v27;  // rax
    unsigned long v28;  // rax
    int v29;  // eax
    unsigned long v32;  // fs
    long long v19;  // rdx
    void* idx2;  // rbx
    void* v21;  // rcx
    char v0;  // [bp-0x108]
    char v1;  // [bp-0x107]
    char v2;  // [bp-0x106]
    char v3;  // [bp-0x105]
    unsigned int v4;  // [bp-0x104]
    unsigned int v5;  // [bp-0x100]
    unsigned int v6;  // [bp-0xfc]
    long long v7;  // [bp-0xf8]
    long long v8;  // [bp-0xf0]
    unsigned long long v9;  // [bp-0xe8], Other Possible Types: long long
    struct_0 *v10;  // [bp-0xe0]
    unsigned long v11;  // [bp-0xd8]
    struct_0 *index;  // [bp-0xd0], Other Possible Types: struct_2 *
    long long v13;  // [bp-0xc8]
    int v14;  // [bp-0xb8], Other Possible Types: stat
    unsigned long v15;  // [bp-0x20]

    v9 = 0xffffffffffffffff;
    v5 = 0;
    v10 = (long long)idx[408];
    v2 = !v10;
    v4 = (!v2 ? v10->field_1a8 : g_49c268);
    v8 = 0;
    if (!g_49f8c8 || sub_434af4("add", a2))
    {
        sub_429547(idx, a2, a2);
        v17 = sub_43edd9(a2, 0, g_49f878);
        sub_429547(idx + 8, v17, v17);
        sub_43b9f2(idx + 8, 1);
        if (v4 < 0 && v2 != 1)
        {
            *(__errno_location()) = -(v4);
            v8 = sub_42b2b1;
        }
        else if (fstatat(v4, a1, idx + 88, g_49fc10))
        {
            v8 = sub_42b444;
        }
        else if ((char)sub_412c90(idx + 88))
        {
            v5 = sub_4157f9(v10, a1, g_49fc08);
            if (v5 < 0)
            {
                v8 = sub_42b2b1;
            }
            else
            {
                *((unsigned int *)&idx[424]) = v5;
                if (fstat(v5, idx + 88))
                    v8 = sub_42b444;
            }
        }
        if (v8)
        {
            sub_42b48b(a2, v2, v8, v2);
        }
        else
        {
            v11 = (long long)idx[0x88];
            *((unsigned long *)&idx[280]) = v11;
            idx1 = idx;
            *((unsigned long long *)&idx1[232]) = sub_45fd6a(idx + 88);
            *((long long *)&idx1[240]) = v19;
            idx2 = idx;
            *((unsigned long long *)&idx2[248]) = sub_45fd9e(idx + 88);
            *((long long *)&idx2[0x100]) = v19;
            v13 = sub_45fd84(idx + 88);
            v21 = idx;
            *((long long *)&v21[264]) = v13;
            *((long long *)&v21[272]) = v19;
            if ((g_49f8be != 1 || v2) && ((short)(int)idx[112] & 0xf000) != 0x4000 && sub_45ff90((long long)idx[248], (long long)idx[0x100], g_49f900, g_49f908) < 0 && (!g_49f87c || sub_45ff90((long long)idx[264], (long long)idx[272], g_49f900, g_49f908) < 0))
            {
                if (g_49f8be != 1 && g_49f9ac && (unsigned short)g_49ec84 & 0x100)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_45e548(a2);
                    error(0, 0, gettext("%s: file is unchanged; not dumped"));
                }
            }
            else
            {
                if (!(char)sub_432b37(idx))
                {
                    v3 = ((int)idx[112] & 0xf000) == 0x4000;
                    if (v3 == 1 || !(char)sub_4153bb(idx))
                    {
                        if (v3 || ((short)(int)idx[112] & 0xf000) == 0x8000)
                        {
                            sub_43cf89(v4, a1, idx, 0, v3 ^ 1);
                            sub_43d566(v4, a1, idx, v5);
                            sub_43d1a5(v4, a1, idx, v5);
                            if (v3)
                            {
                                sub_414bcf(idx);
                                sub_414bcf(idx + 8);
                                if (sub_411fbf(idx, &v7) == 3)
                                {
                                    v27 = gettext("directory not dumped");
                                    sub_411f18(*((long long *)idx), v7, v27);
                                }
                                else
                                {
                                    v0 = sub_414db4(idx);
                                    v5 = (int)idx[424];
                                    v4 = (!v2 ? v10->field_1a8 : g_49c268);
LABEL_416229:
                                    if (v0)
                                    {
                                        if (v5 < 0)
                                        {
                                            *(__errno_location()) = -(v5);
                                            v0 = 0;
                                        }
                                        else
                                        {
                                            if (!v5)
                                            {
                                                if (v4 < 0 && v2 != 1)
                                                {
                                                    *(__errno_location()) = -(v4);
                                                    v0 = 0;
                                                }
                                                else
                                                {
                                                    v0 = !fstatat(v4, a1, &v14, g_49fc10);
                                                }
                                            }
                                            else
                                            {
                                                v0 = !fstat(v5, &v14);
                                            }
                                        }
                                        if (v0 != 1)
                                            sub_42b48b(a2, v2, sub_42b444, v2);
                                    }
                                    if (v0)
                                    {
                                        if (sub_45ff90(sub_45fd84(&v14), v19, v13, v19) && (g_49f94e != 1 || v3 != 1) || v11 < *((unsigned long long *)(&v14 + 48)))
                                        {
                                            if ((char)g_49ec84 & 16)
                                            {
                                                if (g_49fdc8)
                                                    g_49fdc8();
                                                sub_45e548(a2);
                                                error(0, 0, gettext("%s: file changed as we read it"));
                                            }
                                            sub_439e98(1);
                                        }
                                        else
                                        {
                                            if (g_49f880 == 1 && v5 && (v3 || v11) && sub_42ab5e(v5, v4, a1, (long long)idx[232], (long long)idx[240]))
                                                sub_43ead0(a2);
                                        }
                                    }
                                    v0 = (char)sub_439c7e(idx) & v0;
                                    if (v0 && g_49f94e)
                                        sub_43bebb(a2, v3, v3);
                                }
                            }
                            else
                            {
                                if (v5 && g_49f979)
                                {
                                    v28 = (long long)idx[0x88];
                                    if ((short)(long long)idx[0x88] & 0x1ff && (long long)idx[0x88] + 0x1ff > 0x3fe)
                                        v29 = 1;
                                    else
                                        v29 = 0;
                                    if ((long long)idx[152] >= v29 + ((long long)((v28 < 0 ? v28 + 0x1ff : v28)) >> 9))
                                        goto LABEL_4161c2;
                                    v6 = sub_430090(v5, idx, idx);
                                    if (v6 == 3)
                                        v6 = sub_414327(v5, idx);
                                }
                                else
                                {
LABEL_4161c2:
                                    v6 = sub_414327(v5, idx);
                                }
                                if (v6 == 3)
                                    abort(); /* do not return */
                                if (v6 <= 3 && v6 <= 1)
                                    sub_415598(idx);
                                v0 = !v6;
                                goto LABEL_416229;
                            }
                        }
                        else
                        {
                            if (((short)(int)idx[112] & 0xf000) == 0xa000)
                            {
                                *((unsigned long long *)&idx[24]) = sub_447649(v4, a1, (long long)idx[0x88], a1);
                                if (!(long long)idx[24])
                                {
                                    if (*(__errno_location()) == 12)
                                        sub_41783c(); /* do not return */
                                    sub_42b48b(a2, v2, sub_42b35d, v2);
                                }
                                else
                                {
                                    sub_43b9f2(idx + 24, 4);
                                    if ((g_49f864 == 2 ? 99 : 100) < strlen((long long)idx[24]))
                                        sub_4133a8(idx);
                                    sub_43d566(v4, a1, idx, 0);
                                    sub_43d1a5(v4, a1, idx, 0);
                                    v9 = sub_40bacd();
                                    *((unsigned long *)&idx[0x88]) = 0;
                                    index = sub_41381d(idx);
                                    if (index)
                                    {
                                        sub_412157(&index->padding_0[157], (long long)idx[24], 100);
                                        index->padding_0[156] = 50;
                                        sub_41422e(idx, index, v9);
                                        if (g_49f94e)
                                            sub_43bebb(a2, 0);
                                        sub_415598(idx);
                                    }
                                }
                            }
                            else
                            {
                                if (((short)(int)idx[112] & 0xf000) == 0x2000)
                                {
                                    v1 = 0x33;
                                    sub_43cf89(v4, a1, idx, 0, 1);
                                    sub_43d566(v4, a1, idx, 0);
                                    sub_43d1a5(v4, a1, idx, 0);
                                }
                                else
                                {
                                    if (((short)(int)idx[112] & 0xf000) == 0x6000)
                                    {
                                        v1 = 52;
                                        sub_43cf89(v4, a1, idx, 0, 1);
                                        sub_43d566(v4, a1, idx, 0);
                                        sub_43d1a5(v4, a1, idx, 0);
                                    }
                                    else
                                    {
                                        if (((short)(int)idx[112] & 0xf000) == 0x1000)
                                        {
                                            v1 = 54;
                                            sub_43cf89(v4, a1, idx, 0, 1);
                                            sub_43d566(v4, a1, idx, 0);
                                            sub_43d1a5(v4, a1, idx, 0);
                                        }
                                        else
                                        {
                                            if (((short)(int)idx[112] & 0xf000) == 0xc000)
                                            {
                                                if (!((char)g_49ec84 & 32))
                                                    return v15 - *((long long *)(40 + v32));
                                                if (g_49fdc8)
                                                    g_49fdc8();
                                                sub_45e548(a2);
                                                error(0, 0, gettext("%s: socket ignored"));
                                                return v15 - *((long long *)(40 + v32));
                                            }
                                            else
                                            {
                                                sub_415331(a2);
                                                return v15 - *((long long *)(40 + v32));
                                            }
                                        }
                                    }
                                }
                                if (g_49f864 == 1)
                                {
                                    sub_415331(a2);
                                }
                                else
                                {
                                    v9 = sub_40bacd();
                                    *((unsigned long *)&idx[0x88]) = 0;
                                    index = sub_41381d(idx);
                                    if (index)
                                    {
                                        index->field_9c = v1;
                                        if (v1 != 54)
                                        {
                                            sub_412957(gnu_dev_major((long long)idx[128]), &index[2].padding_0[15], 8);
                                            sub_4129ae(gnu_dev_minor((long long)idx[128]), &index[2].padding_0[23], 8);
                                        }
                                        sub_41422e(idx, index, v9);
                                        if (g_49f94e)
                                            sub_43bebb(a2, 0);
                                    }
                                }
                            }
                        }
                    }
                }
                else if ((unsigned short)g_49ec84 & 0x400)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_45e548(a2);
                    error(0, 0, gettext("%s: file is the archive; not dumped"));
                }
            }
        }
    }
    return v15 - *((long long *)(40 + v32));
}



// Function: dump_file @ 0x16a25
extern unsigned long long g_49f8e0;

void dump_file(long long a0, long long a1, long long a2)
{
    char v0;  // [bp-0x1c8]
    long long v1;  // [bp-0x30]

    sub_439c55(&v0);
    v1 = a0;
    sub_415a63(&v0, a1, a2);
    if (!a0)
    {
        sub_439d1c(&v0);
        return;
    }
    else if (g_49f8e0)
    {
        sub_42130d(a0);
        sub_439d1c(&v0);
        return;
    }
    else
    {
        sub_439d1c(&v0);
        return;
    }
}



// Function: move_archive @ 0x16add
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49f870;
extern int g_49f9cc;
extern struct_0 *g_49fbd8;

void move_archive(unsigned long a0)
{
    char v6;  // al
    unsigned long v7;  // rcx
    char v8;  // al
    unsigned long v9;  // rcx
    char v10;  // al
    unsigned long v12;  // rcx
    long long v13;  // rax
    long long v0;  // [bp-0x30]
    long long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]
    unsigned short v3;  // [bp-0x18]
    unsigned int v4;  // [bp-0x14]

    if (!a0)
        return;
    if (a0 - 0 >> 63 & 1)
    {
        v3 = 4;
        v4 = -((unsigned int)a0);
        v6 = v4 == -(a0);
    }
    else
    {
        v3 = 3;
        v4 = a0;
        v6 = a0 == v4;
    }
    if (v6)
    {
        if (g_49f9cc > 0x3fffffff)
        {
            v7 = g_49f9cc - 0x40000000;
            v8 = ~((unsigned int)sub_440254(v7 & 0xffffffff, 1074294017, &v3, v7)) >> 31;
        }
        else
        {
            v8 = ~((int)ioctl(g_49f9cc, 1074294017)) >> 31;
        }
        if (v8)
        {
            return;
        }
        else if (*(__errno_location()) == 5)
        {
            if (g_49f9cc > 0x3fffffff)
            {
                v9 = g_49f9cc - 0x40000000;
                v10 = ~((unsigned int)sub_440254(v9 & 0xffffffff, 1074294017, &v3, v9)) >> 31;
            }
            else
            {
                v10 = ~((int)ioctl(g_49f9cc, 1074294017)) >> 31;
            }
            if (v10)
                return;
        }
    }
    v1 = (g_49f9cc <= 0x3fffffff ? lseek(g_49f9cc, 0, 1) : sub_4400be(g_49f9cc - 0x40000000, 0, 1));
    v2 = g_49f870 * a0;
    v0 = v2 + v1;
    if (v2 / a0 == g_49f870 && !((char)(v2 >> 63) ^ v0 < v1))
    {
        if (v0 < 0)
            v0 = 0;
        if (g_49f9cc > 0x3fffffff)
        {
            v12 = g_49f9cc - 0x40000000;
            v13 = sub_4400be(v12 & 0xffffffff, v0, 0, v12);
        }
        else
        {
            v13 = lseek(g_49f9cc, v0, 0);
        }
        if (v13 == v0)
            return;
    }
    sub_43e7a9(g_49fbd8->field_0, v0, v0);
    return;
}



// Function: write_record @ 0x16d4a
extern unsigned long long g_49efa0;
extern unsigned long long g_49efc0;
extern unsigned long long g_49efc8;
extern unsigned long long g_49f370;
extern unsigned long long g_49f378;
extern unsigned int g_49f380;
extern char g_49f384;
extern unsigned int g_49f9cc;

long long write_record(unsigned int a0)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x10]

    v0 = g_49efa0;
    g_49efa0 = g_49f378;
    if (g_49f384)
    {
        g_49f9cc = 1;
        sub_40f676();
        g_49f9cc = 0;
    }
    else
    {
        sub_416add(g_49f370 + g_49efc8 - g_49efc0);
        sub_40f676();
    }
    v2 = v0;
    g_49efa0 = v2;
    if (a0)
    {
        v2 = g_49f384 ^ 1;
        if ((char)v2)
            v2 = (unsigned long long)sub_416add(g_49efc0 - (g_49f370 + g_49efc8));
    }
    g_49f380 = 0;
    return v2;
}



// Function: write_recent_blocks @ 0x16e0f
extern unsigned long long g_49f378;
extern unsigned int g_49f380;
extern char g_49f868;

unsigned long long write_recent_blocks(unsigned long a0, unsigned long long a1)
{
    unsigned int v2;  // eax
    unsigned long long *v3;  // rdx
    unsigned long long *v4;  // rax
    unsigned long long *iter;  // rdi
    unsigned long long *v6;  // rdx
    unsigned long long *j;  // rax
    unsigned long long v8;  // rdx
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a1; i += 1)
    {
        v2 = g_49f380;
        g_49f380 = v2 + 1;
        v3 = v2 * 0x200 + g_49f378;
        v4 = i * 0x200 + a0;
        *(v3) = *(v4);
        v3[63] = v4[63];
        iter = v3 + 1 & 0xfffffffffffffff8;
        v6 = v3 - iter;
        j = v4 - v6;
        for (v8 = (0x200 + (unsigned int)v6 & 0xfffffff8) >> 3; v8; j += 1)
        {
            v8 -= 1;
            *(iter) = *(j);
            iter += 1;
        }
        if (g_49f380 == *((int *)&g_49f868))
            sub_416d4a(1);
    }
    return i;
}



// Function: write_recent_bytes @ 0x16ee1
extern unsigned long long g_49f378;
extern void g_49f380;
extern unsigned int g_49f868;

long long write_recent_bytes(unsigned int a0, unsigned long long a1)
{
    unsigned long v3;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long n;  // [bp-0x10]

    v0 = a1 >> 9;
    n = a1 - v0 * 0x200;
    sub_416e0f(a0, v0);
    memcpy(*((int *)&g_49f380) * 0x200 + g_49f378, v0 * 0x200 + a0, n);
    if (n <= 0x1ff)
        memset(*((int *)&g_49f380) * 0x200 + g_49f378 + n, 0, 0x200 - n);
    *((unsigned int *)&g_49f380) = *((int *)&g_49f380) + 1;
    v3 = g_49f868;
    if (*((int *)&g_49f380) == g_49f868)
        v3 = sub_416d4a(1);
    return v3;
}



// Function: flush_file @ 0x16fd5
extern unsigned long long g_49efa8;
extern unsigned long long g_49efb0;
extern unsigned int g_49f460;
extern unsigned long long g_49faa8;

long long flush_file(void)
{
    unsigned long v2;  // rax
    unsigned long long v3;  // rdx
    long long i;  // [bp-0x10]

    sub_40bba7(*((long long *)&g_49f460));
    v2 = g_49faa8 + 0x1ff;
    i = (long long)((v2 < 0 ? v2 + 0x1ff : v2)) >> 9;
    while (i >= (long long)(g_49efa8 - g_49efb0) >> 9)
    {
        i += -((long long)(g_49efa8 - g_49efb0) >> 9);
        sub_40cac8();
    }
    v3 = g_49efb0;
    g_49efb0 = i * 0x200 + v3;
    return i * 0x200 + v3;
}



// Function: delete_archive_members @ 0x17069
typedef struct struct_2 {
    char padding_0[48];
    unsigned long long field_30;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49efa0;
extern char g_49efa8;
extern void* g_49efb0;
extern unsigned long long g_49efc0;
extern char g_49efd8;
extern unsigned long long g_49f370;
extern void* g_49f378;
extern int g_49f380;
extern char g_49f384;
extern struct_0 *g_49f460;
extern unsigned int g_49f470;
extern unsigned int g_49f478;
extern unsigned long long g_49f480;
extern unsigned long long g_49f488;
extern char g_49f868;
extern long long g_49f870;
extern char g_49f8bd;
extern char g_49f8d0;
extern void g_49f9cc;
extern long long g_49fa20;
extern long long g_49fa28;
extern unsigned long long g_49faa8;
extern unsigned long long g_49fb90;
extern unsigned long long g_49fb98;
extern void* g_49fbd8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long delete_archive_members(void)
{
    long long v20;  // rsi
    long long v21;  // rdx
    long long v22;  // rcx
    long long v23;  // r8
    long long v24;  // r9
    unsigned long long *v12;  // rdx
    unsigned long long *iter;  // rdi
    unsigned long long *v14;  // rdx
    unsigned long long *v15;  // rax
    unsigned long long v16;  // rdx
    unsigned long v17;  // rax
    long long v19;  // rdi
    unsigned int v0;  // [bp-0x38]
    unsigned int v1;  // [bp-0x34]
    int v2;  // [bp-0x30]
    int v3;  // [bp-0x2c]
    int v4;  // [bp-0x28]
    unsigned int v5;  // [bp-0x24]
    unsigned int v6;  // [bp-0x20]
    long long v7;  // [bp-0x18], Other Possible Types: unsigned long long
    struct_2 *idx;  // [bp-0x10]

    v0 = 0;
    v1 = 0;
    v7 = 0;
    sub_42d3b1();
    sub_40f694(2);
    g_49f384 = !strcmp(*((long long *)g_49fbd8), "-");
    while (1)
    {
        v5 = sub_425ecf(&g_49f460, &g_49fa20, 1);
        switch (v5)
        {
        case 0:
            abort(); /* do not return */
        case 5:
            sub_40bba7(g_49f460);
            if (v1 != 5)
            {
                if (v1 > 5)
                    break;
                if (v1 == 4)
                    abort(); /* do not return */
                if (v1 > 4)
                    break;
                if (!v1)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("This does not look like a tar archive"));
                }
                else if (!(v1 - 1 <= 2))
                {
                    break;
                }
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Skipping to next header"));
                g_49fdd0 = 2;
            }
            goto LABEL_417299;
        case 1:
            idx = sub_42ed0e(g_49fa28);
            if (!idx)
            {
                sub_428891();
                continue;
            }
            else
            {
                idx->field_30 = idx->field_30 + 1;
                if ((!*((long long *)&g_49f8d0) ? !idx->field_30 : idx->field_30 != *((long long *)&g_49f8d0)))
                {
                    sub_428891();
                    continue;
                }
            }
        case 2:
            v0 = v5;
            continue;
        case 3:
            if (g_49f8bd)
            {
                sub_40bba7(g_49f460);
                continue;
            }
        case 4:
            v0 = 4;
            continue;
        default:
LABEL_417299:
            v1 = v5;
            if (v0)
            {
                g_49f370 = g_49efc0 - 1;
                g_49f378 = sub_460b5b(g_49f870);
                if (v0 == 1 || v0 == 2)
                {
                    g_49efd8 = 0;
                    g_49f380 = g_49efb0 - g_49efa0 >> 9;
                    if (g_49f380)
                        memcpy(g_49f378, g_49efa0, g_49f380 * 0x200);
                    if (v0 == 1)
                    {
                        v0 = 0;
                        sub_416fd5();
                    }
                    while (v0 != 4)
                    {
                        if (g_49efb0 == *((long long *)&g_49efa8))
                            sub_40cac8();
                        sub_425ecf(&g_49f460, &g_49fa20, 0);
                        switch ((unsigned int)sub_425ecf(&g_49f460, &g_49fa20, 0))
                        {
                        case 0: case 2:
                            abort(); /* do not return */
                        case 1:
                            sub_41df5f(&g_49fa20);
                            idx = sub_42ed0e(g_49fa28);
                            if (idx && !(idx->field_30 = idx->field_30 + 1, !(!*((long long *)&g_49f8d0) ? (char)(char)(idx->field_30) : (char)(char)(idx->field_30 == *((long long *)&g_49f8d0)))))
                            {
                                sub_416fd5();
                                break;
                            }
                            else
                            {
                                if (g_49fb90)
                                {
                                    sub_416ee1(g_49fb98, g_49fb90);
                                }
                                else
                                {
                                    sub_416e0f(*((long long *)&g_49f470), g_49f480);
                                    sub_416e0f(*((long long *)&g_49f478), g_49f488);
                                }
                                v12 = g_49f380 * 0x200 + g_49f378;
                                *(v12) = g_49f460->field_0;
                                v12[63] = g_49f460[31].field_8;
                                iter = v12 + 1 & 0xfffffffffffffff8;
                                v14 = v12 - iter;
                                v15 = g_49f460 - v14;
                                for (v16 = (0x200 + (unsigned int)v14 & 0xfffffff8) >> 3; v16; v15 += 1)
                                {
                                    v16 -= 1;
                                    *(iter) = *(v15);
                                    iter += 1;
                                }
                                g_49f380 = g_49f380 + 1;
                                v17 = g_49faa8 + 0x1ff;
                                v7 = (long long)((v17 < 0 ? v17 + 0x1ff : v17)) >> 9;
                                sub_40bba7(g_49f460);
                                if (g_49f380 == *((int *)&g_49f868))
                                    sub_416d4a(1);
                                v2 = *((long long *)&g_49efa8) - g_49efb0 >> 9;
                                if (v7 < v2)
                                    v2 = v7;
                                while (v7)
                                {
                                    if (g_49efb0 == *((long long *)&g_49efa8))
                                    {
                                        sub_40f662();
                                        g_49efb0 = g_49efa0;
                                        v2 = *((int *)&g_49f868);
                                        if (v7 < v2)
                                            v2 = v7;
                                    }
                                    v3 = v2;
                                    if (v3 > *((int *)&g_49f868) - g_49f380)
                                        v3 = *((int *)&g_49f868) - g_49f380;
                                    if (!v3)
                                        abort(); /* do not return */
                                    memcpy(g_49f380 * 0x200 + g_49f378, g_49efb0, v3 * 0x200);
                                    g_49f380 = v3 + g_49f380;
                                    g_49efb0 = v3 * 0x200 + g_49efb0;
                                    v7 -= v3;
                                    v2 -= v3;
                                    if (g_49f380 == *((int *)&g_49f868))
                                        sub_416d4a(1);
                                }
                            }
                        case 3:
                            if (g_49f8bd)
                            {
                                sub_40bba7(g_49f460);
                                break;
                            }
                            else
                            {
                                v0 = 4;
                                break;
                            }
                        case 4:
                            v0 = 4;
                            break;
                        case 5:
                            if (g_49fdc8)
                                g_49fdc8();
                            error(0, 0, gettext("Deleting non-header from archive"));
                            g_49fdd0 = 2;
                            sub_40bba7(g_49f460);
                            break;
                        default:
                            abort(); /* do not return */
                        }
                        sub_439d1c(&g_49fa20);
                    }
                    if (v0 == 4)
                    {
                        v4 = 0;
                        do
                        {
                            v6 = *((int *)&g_49f868) - g_49f380;
                            memset(g_49f378 + g_49f380 * 0x200, 0, v6 * 0x200);
                            v4 += v6;
                            sub_416d4a(v4 <= 1);
                        } while (v4 <= 1);
                    }
                    if (g_49f384 != 1 && *((int *)&g_49f9cc) <= 0x3fffffff && sub_432f52(*((int *)&g_49f9cc)))
                        sub_43ea7e(*((long long *)g_49fbd8));
                }
                free(g_49f378);
                sub_40d046();
                return sub_42dbba(v19, v20, v21, v22, v23, v24);
            }
            continue;
        }
    }
}



// Function: fatal_exit @ 0x177f4
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f388;

void fatal_exit(void)
{
    if (g_49f388)
        g_49f388();
    error(2, 0, gettext("Error is not recoverable: exiting now"));
}



// Function: xalloc_die @ 0x1783c
void xalloc_die(void)
{
    gettext("memory exhausted");
    error(0, 0, "%s");
    sub_4177f4(); /* do not return */
}



// Function: excfile_add @ 0x17879
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned long long g_49f390;
extern struct_2 *g_49f398;

struct_0 * excfile_add(char *a0, unsigned int a1)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_460b5b(strlen(a0) + 16);
    v0->field_0 = 0;
    v0->field_8 = a1;
    strcpy(v0 + 1, a0);
    if (g_49f398)
        g_49f398->field_0 = v0;
    else
        g_49f390 = v0;
    g_49f398 = v0;
    return v0;
}



// Function: info_attach_exclist @ 0x17909
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
    char field_c;
} struct_0;

typedef struct struct_2 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    long long field_10;
    struct struct_1 *field_18;
    long long field_20;
} struct_2;

typedef struct struct_1 {
    char padding_0[408];
    struct struct_0 *field_198;
} struct_1;

extern unsigned int g_49c268;
extern struct_0 *g_49f390;
extern struct_1 *g_49fdc8;
extern unsigned int g_49fdd0;

struct_1 * info_attach_exclist(struct_1 *a0)
{
    char *v10;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x58]
    unsigned int v1;  // [bp-0x54]
    struct_0 *v2;  // [bp-0x50]
    struct_1 *v3;  // [bp-0x48]
    struct_1 *v4;  // [bp-0x40]
    long long v5;  // [bp-0x38]
    unsigned long fp;  // [bp-0x30]
    struct_2 *v7;  // [bp-0x28]
    struct_1 *idx;  // [bp-0x20]

    v3 = NULL;
    v4 = NULL;
    if (*((long long *)&a0[1].padding_0[16]))
        return *((long long *)&a0[1].padding_0[16]);
    v2 = g_49f390;
    while (1)
    {
        if (!v2)
        {
            *((struct_1 **)&a0[1].padding_0[16]) = v3;
            return a0;
        }
        if (!faccessat((!a0 ? g_49c268 : *((int *)&a0[1].padding_0[8]))))
        {
            v5 = 0;
            v0 = sub_4157f9(a0, &v2->field_c, 0);
            if (v0 == 0xffffffff)
            {
                sub_43e38b(&v2->field_c);
            }
            else
            {
                fp = fdopen(v0, "r");
                if (!fp)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    v10 = gettext("%s: fdopen failed");
                    err = __errno_location();
                    error(0, *(err), v10);
                    g_49fdd0 = 2;
                    close(v0);
                }
                else
                {
                    if (!v5)
                        v5 = sub_44e53e();
                    v7 = sub_418242(&v2->field_c);
                    if (v7->field_18)
                        v7->field_20 = v7->field_18(v7->field_20);
                    if (sub_44f189(v7->field_10, v5, fp, 1342177281, 10, v7->field_20))
                        break;
                    fclose(fp);
                    idx = sub_460b5b(32);
                    *((long long *)&idx->padding_0[24]) = v5;
                    *((unsigned int *)&idx->padding_0[16]) = (!v2->field_8 ? v2->field_8 : v7->field_8);
                    *((struct_1 **)&idx->padding_0[8]) = v4;
                    *((unsigned long long *)&idx->padding_0[0]) = 0;
                    if (v4)
                        *((struct_1 **)&v4->padding_0[0]) = idx;
                    else
                        v3 = idx;
                    v4 = idx;
                }
            }
        }
        v2 = v2->field_0;
    }
    v1 = *(__errno_location());
    if (g_49fdc8)
        g_49fdc8();
    sub_45e548(&v2->field_c);
    error(0, v1, "%s");
    sub_4177f4(); /* do not return */
}



// Function: info_free_exclist @ 0x17bda
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[16];
    long long field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[432];
    struct struct_0 *field_1b0;
} struct_1;

struct_1 * info_free_exclist(struct_1 *idx)
{
    struct_0 *ptr;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    for (ptr = idx->field_1b0; ptr; ptr = v1)
    {
        v1 = ptr->field_0;
        sub_44e904(ptr->field_18);
        free(ptr);
    }
    idx->field_1b0 = NULL;
    return idx;
}



// Function: excluded_name @ 0x17c43
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    long long field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[408];
    struct struct_1 *field_198;
    char padding_1a0[16];
    struct struct_0 *field_1b0;
} struct_1;

extern long long g_49f8a8;

unsigned int excluded_name(char *a0, struct_1 *a1)
{
    struct_1 *iter;  // [bp-0x38]
    char v1;  // [bp-0x25]
    unsigned int i;  // [bp-0x24]
    struct_0 *j;  // [bp-0x20]
    char *k;  // [bp-0x18]
    unsigned long v5;  // [bp-0x10]

    iter = a1;
    k = NULL;
    v5 = 0;
    i = 0;
    if ((char)sub_44ed6a(g_49f8a8, a0, a0))
    {
        return 1;
    }
    else if (!iter)
    {
        return 0;
    }
    else
    {
        for (v1 = 0; iter && v1 != 1; i = 2)
        {
            for (j = iter->field_1b0; j; j = j->field_0)
            {
                if (!(j->field_10 & i))
                {
                    v1 = sub_44ed6a(j->field_18, a0, a0);
                    if (v1)
                        break;
                    if (!k)
                    {
                        for (k = a0; *(k) == 46 && k[1] == 47; k += 2);
                    }
                    v1 = sub_44ed6a(j->field_18, k, k);
                    if (v1)
                        break;
                    if (!v5)
                        v5 = sub_44e13e(a0);
                    v1 = sub_44ed6a(j->field_18, v5, v5);
                    if (v1)
                        break;
                }
            }
            iter = iter->field_198;
        }
        free(v5);
        return v1;
    }
}



// Function: cvs_addfn @ 0x17ddd
void cvs_addfn(long long a0, long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x128]
    unsigned long i;  // [bp-0x100]
    char v2;  // [bp-0xf8]
    unsigned long v3;  // [bp-0xf0]

    v0 = a3;
    if (sub_4461a6(a1, &v2, 2116, &v2))
        return;
    for (i = 0; i < v2; i += 1)
    {
        sub_44ee5d(a0, *((long long *)(i * 8 + v3)), a2, *((long long *)(i * 8 + v3)));
    }
    sub_446399(&v2);
    return;
}



// Function: git_addfn @ 0x17eb5
long long git_addfn(long long a0, void* a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long *v3;  // rax
    unsigned long v4;  // rax
    unsigned long long v0;  // [bp-0x28]
    void* iter;  // [bp-0x18]

    iter = a1;
    v0 = a3;
    while (1)
    {
        v3 = __ctype_b_loc();
        if (!(*((short *)(*((char *)iter) * 2 + *(v3))) & 0x2000))
            break;
        iter += 1;
    }
    v4 = *((char *)iter);
    if ((char)v4)
    {
        v4 = *((char *)iter);
        if ((char)v4 != 35)
        {
            if (*((char *)iter) == 92 && (char)iter[1] == 35)
                iter += 1;
            return sub_44ee5d(a0, iter, a2, iter);
        }
    }
    return v4;
}



// Function: bzr_addfn @ 0x17f4f
long long bzr_addfn(long long a0, void* a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long *v4;  // rax
    unsigned long v5;  // rax
    unsigned long long v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x1c]
    void* iter;  // [bp-0x18]

    iter = a1;
    v1 = a2;
    v0 = a3;
    while (1)
    {
        v4 = __ctype_b_loc();
        if (!(*((short *)(*((char *)iter) * 2 + *(v4))) & 0x2000))
            break;
        iter += 1;
    }
    v5 = *((char *)iter);
    if ((char)v5)
    {
        v5 = *((char *)iter);
        if ((char)v5 != 35)
        {
            if (*((char *)iter) == 33)
            {
                iter += 1;
                if (*((char *)iter) == 33)
                    iter += 1;
                else
                    v1 |= 0x20000000;
            }
            if (!strncmp(iter, "RE:", 3))
            {
                iter += 3;
                v1 &= 0xefffffff;
                v1 |= 0x8000000;
            }
            return sub_44ee5d(a0, iter, v1, iter);
        }
    }
    return v5;
}



// Function: hg_initfn @ 0x18025
extern unsigned int g_49f3a0;

unsigned int * hg_initfn(unsigned int *a0)
{
    unsigned int *v0;  // [bp-0x20]
    unsigned int *v1;  // [bp-0x10]

    v0 = a0;
    if (!v0)
        v0 = &g_49f3a0;
    v1 = v0;
    *(v1) = 0x8000000;
    return v1;
}



// Function: hg_addfn @ 0x18059
unsigned int * hg_addfn(long long a0, void* a1, unsigned int a2, unsigned int *a3)
{
    unsigned long long *v5;  // rax
    unsigned int *v6;  // rax
    unsigned long long *v7;  // rax
    unsigned int *v8;  // rax
    unsigned int v0;  // [bp-0x3c]
    void* iter;  // [bp-0x38]
    unsigned long len;  // [bp-0x18]
    void* v3;  // [bp-0x10]

    iter = a1;
    v0 = a2;
    while (1)
    {
        v5 = __ctype_b_loc();
        if (!(*((short *)(*((char *)iter) * 2 + *(v5))) & 0x2000))
            break;
        iter += 1;
    }
    v6 = *((char *)iter);
    if (*((char *)&v6))
    {
        v6 = *((char *)iter);
        if (*((char *)&v6) != 35)
        {
            if (!strncmp(iter, "syntax:", 7))
            {
                iter += 7;
                while (1)
                {
                    v7 = __ctype_b_loc();
                    if (!(*((short *)(*((char *)iter) * 2 + *(v7))) & 0x2000))
                        break;
                    iter += 1;
                }
                if (!strcmp(iter, "regexp"))
                {
                    v8 = a3;
                    *(v8) = 0x8000000;
                }
                else
                {
                    v8 = strcmp(iter, "glob");
                    if (!(unsigned int)v8)
                    {
                        v8 = a3;
                        *(v8) = 0x10000000;
                    }
                }
                return v8;
            }
            else
            {
                len = strlen(iter);
                if (*((char *)iter + len - 1) == 47)
                {
                    len -= 1;
                    v3 = sub_460b5b(len + 1);
                    memcpy(v3, iter, len);
                    *(len + (char *)v3) = 0;
                    iter = v3;
                    sub_44e34b(a0, v3, v3);
                    v0 |= 67108872;
                }
                return sub_44ee5d(a0, iter, (*(a3) == 0x8000000 ? v0 & 0xefffffff : v0 & 0xf7ffffff) | *(a3), iter);
            }
        }
    }
    return v6;
}



// Function: get_vcs_ignore_file @ 0x18242
extern char *g_49c160;

char ** get_vcs_ignore_file(char *a0)
{
    char **i;  // [bp-0x10]

    for (i = &g_49c160; *(i); i += 5)
    {
        if (!strcmp(*(i), a0))
            return i;
    }
    return i;
}



// Function: exclude_vcs_ignores @ 0x18293
extern char *g_49c160;

unsigned long long exclude_vcs_ignores(void)
{
    char **i;  // [bp-0x10]

    for (i = &g_49c160; *(i); i += 5)
    {
        sub_417879(*(i), 0);
    }
    return 0;
}



// Function: implemented @ 0x182d5
unsigned int implemented(unsigned int a0)
{
    unsigned int v1;  // eax

    if (a0 != 38 && a0 != 95)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: extr_init @ 0x182fd
extern char g_49f3a8;
extern unsigned int g_49f3ac;
extern unsigned int g_49f3b0;
extern unsigned int g_49f95c;
extern void g_49f960;

long long extr_init(void)
{
    g_49f3a8 = !geteuid();
    *((unsigned int *)&g_49f960) = *((int *)&g_49f960) + g_49f3a8;
    g_49f95c = g_49f95c + g_49f3a8;
    g_49f3ac = umask(0);
    if (*((int *)&g_49f960) <= 0)
    {
        umask(g_49f3ac);
        g_49f3b0 = g_49f3ac;
        return g_49f3ac;
    }
    g_49f3b0 = 0;
    return *((int *)&g_49f960);
}



// Function: fd_i_chmod @ 0x18387
extern unsigned int g_49c268;

int fd_i_chmod(int a0, long long a1, unsigned int a2, unsigned int a3)
{
    unsigned int v0;  // [bp-0xc]

    if (a0 >= 0)
    {
        v0 = fchmod(a0, a2);
        if (!v0 || (char)sub_4182d5(*(__errno_location())))
            return v0;
    }
    return fchmodat(g_49c268, a1, a2, a3);
}



// Function: fd_chmod @ 0x183ee
unsigned int fd_chmod(int a0, long long a1, unsigned int a2, unsigned int a3, unsigned int a4)
{
    unsigned int v0;  // [bp-0xc]

    v0 = (!sub_418387(a0, a1, a2, a3) ? 0 : *(__errno_location()));
    if (v0 == 1 && (unsigned short)a2 & 0x800 && !sub_45c835())
    {
        v0 = (!sub_418387(a0, a1, a2, a3) ? 0 : *(__errno_location()));
        sub_45c826();
    }
    if (a3 && a4 != 50 && (char)sub_4182d5(v0) != 1)
        v0 = (!sub_418387(a0, a1, a2, 0) ? 0 : *(__errno_location()));
    if (v0 && (a4 != 50 || (char)sub_4182d5(v0)))
    {
        *(__errno_location()) = v0;
        return 0xffffffff;
    }
    return 0;
}



// Function: fd_chown @ 0x184f7
extern unsigned int g_49c268;

int fd_chown(int a0, long long a1, unsigned int a2, unsigned int a3, unsigned int a4)
{
    unsigned int v0;  // [bp-0xc]

    if (a0 >= 0)
    {
        v0 = fchown(a0, a2, a3);
        if (!v0 || (char)sub_4182d5(*(__errno_location())))
            return v0;
    }
    return fchownat(g_49c268, a1, a2, a3, a4);
}



// Function: fd_stat @ 0x1856b
extern unsigned int g_49c268;

unsigned int fd_stat(int a0, long long a1, unsigned long a2, unsigned int a3)
{
    return (a0 < 0 ? (unsigned int)fstatat(g_49c268, a1, a2, a3) : (int)fstat(a0, a2));
}



// Function: set_mode @ 0x185b8
void set_mode(long long a0, unsigned int a1, unsigned int a2, int a3, unsigned int a4, unsigned int a5, char a6, unsigned int a7)
{
    unsigned int v0;  // [bp-0xc0]
    unsigned int v1;  // [bp-0xb4]
    char v2;  // [bp-0xa8]
    unsigned int v3;  // [bp-0x90]

    v1 = a1;
    v0 = a4;
    if (!((~(a5) | v0 ^ v1) & a2))
        return;
    if (~((unsigned short)a2 | (unsigned short)a5) & 0xfff)
    {
        if (sub_41856b(a3, a0, &v2, a7))
        {
            sub_43ea03(a0);
            return;
        }
        v0 = v3;
    }
    v0 &= 0xfff;
    v1 = v1 & a2 | ~(a2) & v0;
    if (v0 == v1)
    {
        return;
    }
    else if (sub_4183ee(a3, a0, v1, a7, a6))
    {
        sub_43e0b2(a0, v1, v1);
        return;
    }
    else
    {
        return;
    }
}



// Function: check_time @ 0x18719
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern long long g_49f9f0;
extern long long g_49f9f8;
extern struct_0 *g_49fdc8;

void check_time(unsigned long long a0, long long a1, long long a2)
{
    unsigned long long v0;  // [bp-0x70]
    long long v1;  // [bp-0x68]
    long long v2;  // [bp-0x60]
    long long v3;  // [bp-0x58]
    long long v4;  // [bp-0x50]
    char v5;  // [bp-0x48]

    v0 = a0;
    if (a1 < 0)
    {
        if ((unsigned short)g_49ec84 & 0x8000)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_427abc(a1, a2, 1, a1);
            error(0, 0, gettext("%s: implausibly old time stamp %s"));
            return;
        }
        else
        {
            return;
        }
    }
    else
    {
        if (sub_45ff90(g_49f9f0, g_49f9f8, a1, a2) < 0)
        {
            sub_4521fa(&v1);
            if (sub_45ff90(v1, v2, a1, a2) >= 0)
                return;
            v3 = a1 - v1;
            v4 = a2 - v2;
            if (v4 < 0)
            {
                v4 += 1000000000;
                v3 -= 1;
            }
            if (!((unsigned short)g_49ec84 & 0x8000))
                return;
            if (g_49fdc8)
                g_49fdc8();
            sub_429f2e(v3, v4, &v5, v3);
            sub_427abc(a1, a2, 1, a1);
            error(0, 0, gettext("%s: time stamp %s is %s s in the future"));
            return;
        }
        else
        {
            return;
        }
    }
}



// Function: set_stat @ 0x18913
typedef struct struct_0 {
    char padding_0[112];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49f3b0;
extern char g_49f8be;
extern int g_49f95c;
extern int g_49f960;
extern char g_49f99a;

void set_stat(unsigned long a0, struct_0 *idx, unsigned int a2, unsigned int a3, unsigned int a4, char a5, char a6, unsigned int a7)
{
    unsigned int v7;  // r10d
    unsigned int v0;  // [bp-0x64]
    unsigned int v1;  // [bp-0x40]
    unsigned int v2;  // [bp-0x3c]
    unsigned long v3;  // [bp-0x38]
    unsigned long v4;  // [bp-0x30]
    unsigned long v5;  // [bp-0x28]
    unsigned long v6;  // [bp-0x20]

    v0 = a4;
    if (g_49f99a != 1 && a6 != 1)
    {
        if (g_49f8be)
        {
            v3 = idx->field_e8;
            v4 = idx->field_f0;
        }
        else
        {
            v4 = 0x3ffffffe;
        }
        v5 = idx->field_f8;
        v6 = idx->field_100;
        if (!sub_44f4ec(a2, g_49c268, a0, &v3, a7))
        {
            if (g_49f8be)
                sub_418719(a0, v3, v4);
            sub_418719(a0, v5, v6);
        }
        else if (a5 != 50 || (char)sub_4182d5(*(__errno_location())))
        {
            sub_43ead0(a0);
        }
    }
    if (g_49f95c > 0 && a6 != 1)
    {
        v1 = idx->field_74;
        v2 = idx->field_78;
        if (!sub_4184f7(a2, a0, v1, v2, a7))
        {
            if ((~((char)v0) | (char)a3) & 73)
                v0 &= ~(a3 & 0xc00);
        }
        else
        {
            if (a5 != 50 || (char)sub_4182d5(*(__errno_location())))
                sub_43e169(a0, v1, v2, v1);
        }
    }
    if (g_49f960 > 0 && a6 != 1)
        v7 = 0xfff;
    else
        v7 = 0x1ff;
    sub_4185b8(a0, idx->field_70 & ~(g_49f3b0), v7, a2, a3, v0, a5, a7);
    sub_43d8a1(idx, a0, a5, 1);
    sub_43d003(idx, a0, a5, a0);
    sub_43d609(idx, a0, a5, a0);
    return;
}



// Function: find_direct_ancestor @ 0x18b89
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

extern struct_0 *g_49f3b8;

struct_1 * find_direct_ancestor(unsigned long a0)
{
    unsigned long long v2;  // rax
    struct_0 *i;  // [bp-0x10]

    for (i = g_49f3b8; i; i = i->field_0)
    {
        if (i->field_58 != 1 && !strncmp(a0, i->field_a0, *((long long *)&i->field_98)) && *((char *)(a0 + *((long long *)&i->field_98))) == 47)
        {
            v2 = sub_44df20(a0);
            if (v2 == a0 + *((long long *)&i->field_98) + 1)
                return i;
        }
    }
    return i;
}



// Function: mark_after_links @ 0x18c40
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char padding_18[64];
    char field_58;
    char padding_59[71];
    long long field_a0;
} struct_0;

void mark_after_links(struct_0 *a0)
{
    struct_0 *idx;  // [bp-0xb0]
    char v1;  // [bp-0xa8]
    unsigned long v2;  // [bp-0xa0]

    idx = a0;
    while (1)
    {
        idx->field_58 = 1;
        if (sub_42a9b8(idx->field_a0, &v1, &v1))
        {
            sub_43ea03(idx->field_a0);
        }
        else
        {
            idx->field_8 = v1;
            idx->field_10 = v2;
        }
        idx = idx->field_0;
        if (!idx)
        {
            return;
        }
        else if (idx->field_58 == 1)
        {
            return;
        }
    }
}



// Function: delay_set_stat @ 0x18d26
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
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
} struct_0;

extern unsigned int g_49c268;
extern void* g_49f3b8;
extern unsigned int g_49f600;

void delay_set_stat(unsigned long a0, struct_0 *index, unsigned int a2, unsigned int a3, unsigned int a4, unsigned int a5)
{
    void* idx;  // rcx
    unsigned long v6;  // rdx
    void* idx1;  // rcx
    unsigned long v8;  // rdx
    unsigned long v9;  // rdx
    unsigned long v10;  // rdx
    void* idx2;  // [bp-0xb8]
    unsigned long len;  // [bp-0xb0]
    char v2;  // [bp-0xa8]
    unsigned long v3;  // [bp-0xa0]

    len = strlen(a0);
    for (idx2 = g_49f3b8; idx2 && strcmp((long long)idx2[160], a0); idx2 = *((long long *)idx2));
    if (!idx2)
    {
        idx2 = sub_460b5b(168);
        *((void* *)idx2) = g_49f3b8;
        g_49f3b8 = idx2;
        *((unsigned long *)&idx2[152]) = len;
        *((unsigned long long *)&idx2[160]) = sub_460cc6(a0);
        *((char *)&idx2[88]) = 0;
        if (index)
        {
            *((unsigned long long *)&idx2[8]) = index->field_58;
            *((unsigned long long *)&idx2[16]) = index->field_60;
        }
    }
    else if ((char)idx2[80])
    {
        if (fstatat(g_49c268, (long long)idx2[160], &v2, (int)idx2[84]))
        {
            sub_43ea03((long long)idx2[160]);
        }
        else
        {
            *((unsigned long long *)&idx2[8]) = v2;
            *((unsigned long *)&idx2[16]) = v3;
        }
    }
    *((unsigned int *)&idx2[24]) = a4;
    if (index)
    {
        *((unsigned int *)&idx2[28]) = index->field_74;
        *((unsigned int *)&idx2[32]) = index->field_78;
        idx = idx2;
        v6 = index->field_f0;
        *((unsigned long long *)&idx[40]) = index->field_e8;
        *((unsigned long *)&idx[48]) = v6;
        idx1 = idx2;
        v8 = index->field_100;
        *((unsigned long long *)&idx1[56]) = index->field_f8;
        *((unsigned long *)&idx1[64]) = v8;
    }
    *((unsigned int *)&idx2[72]) = a2;
    *((unsigned int *)&idx2[76]) = a3;
    *((char *)&idx2[80]) = !index;
    *((unsigned int *)&idx2[84]) = a5;
    *((unsigned int *)&idx2[92]) = g_49f600;
    *((unsigned long long *)&idx2[96]) = 0;
    if (index)
        sub_429547(idx2 + 96, index->field_30, idx2 + 96);
    if (index && index->padding_38)
    {
        v9 = index->field_40 + 1;
        *((unsigned long long *)&idx2[104]) = sub_460c8e(index->padding_38, v9, v9);
        *((unsigned long long *)&idx2[112]) = index->field_40;
    }
    else
    {
        *((unsigned long long *)&idx2[104]) = 0;
        *((unsigned long long *)&idx2[112]) = 0;
    }
    if (index && index->padding_48)
    {
        v10 = index->field_50 + 1;
        *((unsigned long long *)&idx2[120]) = sub_460c8e(index->padding_48, v10, v10);
        *((unsigned long long *)&idx2[128]) = index->field_50;
    }
    else
    {
        *((unsigned long long *)&idx2[120]) = 0;
        *((unsigned long long *)&idx2[128]) = 0;
    }
    if (index)
    {
        sub_41d854(index, idx2 + 144, idx2 + 0x88, idx2 + 144);
    }
    else
    {
        *((unsigned long long *)&idx2[144]) = 0;
        *((unsigned long long *)&idx2[0x88]) = 0;
    }
    if (!(char)sub_42a1fe(a0))
        return;
    sub_418c40(idx2);
    return;
}



// Function: repair_delayed_set_stat @ 0x191c5
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
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned int field_48;
    unsigned int field_4c;
    char field_50;
    char padding_51[3];
    unsigned int field_54;
    char padding_58[72];
    long long field_a0;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned int g_49c268;
extern struct_0 *g_49f3b8;
extern unsigned long long g_49fa78;
extern unsigned long long g_49fa80;
extern unsigned int g_49fa90;
extern unsigned int g_49fa94;
extern unsigned int g_49fa98;
extern unsigned long long g_49fb08;
extern unsigned long long g_49fb10;
extern unsigned long long g_49fb18;
extern unsigned long long g_49fb20;
extern struct_2 *g_49fdc8;
extern unsigned int g_49fdd0;

void repair_delayed_set_stat(long long a0, struct_1 *a1)
{
    struct_0 *index;  // rcx
    struct_0 *idx;  // rcx
    struct_0 *idx1;  // [bp-0xc0]
    char v1;  // [bp-0xb8]
    unsigned long v2;  // [bp-0xb0]
    unsigned int v3;  // [bp-0xa0]

    for (idx1 = g_49f3b8; idx1; idx1 = idx1->field_0)
    {
        if (fstatat(g_49c268, idx1->field_a0, &v1, idx1->field_54))
        {
            sub_43ea03(idx1->field_a0);
            return;
        }
        if (v1 == a1->padding_0 && v2 == a1->field_8)
        {
            idx1->field_8 = g_49fa78;
            idx1->field_10 = g_49fa80;
            idx1->field_18 = g_49fa90;
            idx1->field_1c = g_49fa94;
            idx1->field_20 = g_49fa98;
            index = idx1;
            index->field_28 = g_49fb08;
            index->field_30 = g_49fb10;
            idx = idx1;
            idx->field_38 = g_49fb18;
            idx->field_40 = g_49fb20;
            idx1->field_48 = v3;
            idx1->field_4c = 0xffffffff;
            idx1->field_50 = 0;
            return;
        }
    }
    if (g_49fdc8)
        g_49fdc8();
    sub_45e548(a0);
    error(0, 0, gettext("%s: Unexpected inconsistency when making directory"));
    g_49fdd0 = 2;
    return;
}



// Function: free_delayed_set_stat @ 0x193d1
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
    sub_41d55d(a0->field_90, a0->field_88, a0->field_88);
    free(a0->field_60);
    free(a0->field_68);
    free(a0->field_78);
    free(a0);
    return;
}



// Function: remove_delayed_set_stat @ 0x19454
typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[84];
    unsigned int field_5c;
    char padding_60[64];
    char *field_a0;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[84];
    unsigned int field_5c;
    char padding_60[64];
    unsigned long long field_a0;
} struct_0;

extern struct_1 *g_49f3b8;
extern char g_49f600;

struct_0 * remove_delayed_set_stat(char *a0)
{
    struct_1 *v4;  // rax
    struct_1 *v0;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x18]
    struct_0 *v2;  // [bp-0x10]

    v1 = NULL;
    v4 = g_49f3b8;
    v0 = g_49f3b8;
    while (1)
    {
        if (!v0)
            return v4;
        v2 = v0->field_0;
        if (v0->field_5c == *((int *)&g_49f600) && !strcmp(v0->field_a0, a0))
            break;
        v1 = v0;
        v4 = v2;
        v0 = v4;
    }
    sub_4193d1(v0);
    if (!v1)
    {
        g_49f3b8 = v2;
        return v2;
    }
    v1->field_0 = v2;
    return v1;
}



// Function: fixup_delayed_set_stat @ 0x194f9
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[84];
    unsigned int field_5c;
    char padding_60[56];
    struct struct_0 *field_98;
    void* field_a0;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[84];
    unsigned int field_5c;
    char padding_60[56];
    struct struct_1 *field_98;
    unsigned long long field_a0;
} struct_1;

extern struct_0 *g_49f3b8;
extern char g_49f600;

struct_1 * fixup_delayed_set_stat(char *a0, unsigned long a1)
{
    struct_0 *v2;  // rax
    struct_1 *len;  // rax
    struct_0 *idx;  // [bp-0x10]

    v2 = g_49f3b8;
    for (idx = g_49f3b8; idx; idx = idx->field_0)
    {
        if (idx->field_5c == *((int *)&g_49f600) && !strcmp(idx->field_a0, a0))
        {
            free(idx->field_a0);
            idx->field_a0 = sub_460cc6(a1);
            len = strlen(a1);
            idx->field_98 = len;
            return len;
        }
        v2 = idx->field_0;
    }
    return v2;
}



// Function: make_directories @ 0x195aa
extern unsigned int g_49c268;
extern char g_49f3a8;
extern unsigned int g_49f3ac;
extern unsigned int g_49f3b0;

unsigned int make_directories(void* a0, unsigned long a1)
{
    unsigned int v0;  // [bp-0xc8]
    unsigned int v1;  // [bp-0xc4]
    unsigned int v2;  // [bp-0xc0]
    unsigned int v3;  // [bp-0xbc]
    void* iter;  // [bp-0xb8]
    char v5;  // [bp-0xa8]

    for (iter = a0; *((char *)iter); iter += 1)
    {
        if (*((char *)iter) == 47 && iter != a0 && *((char *)iter - 1) != 47 && (*((char *)iter - 1) != 46 || iter != a0 + 1 && *((char *)iter - 2) != 47 && (*((char *)iter - 2) != 46 || iter != a0 + 2 && *((char *)iter - 3) != 47)))
        {
            *((char *)iter) = 0;
            v1 = ~(g_49f3ac) & 0x1ff;
            v2 = (!g_49f3a8 ? 192 : 0) | v1;
            v0 = mkdirat(g_49c268, a0, v2, a0);
            if (!v0)
            {
                sub_418d26(a0, NULL, ~(g_49f3b0) & v2, 0x1ff, v1, 0x100);
                sub_4286af(a0, iter - a0 & 0xffffffff, v1, iter - a0 & 0xffffffff);
                *((char *)a1) = 1;
            }
            else if (*(__errno_location()) == 0x11)
            {
                v0 = 0;
            }
            else
            {
                v3 = *(__errno_location());
                v0 = fstatat(g_49c268, a0, &v5, 0);
                if (v0)
                {
                    *(__errno_location()) = v3;
                    sub_43e310(a0);
                }
            }
            *((char *)iter) = 47;
            if (v0)
                return v0;
        }
    }
    return 0;
}



// Function: file_newer_p @ 0x19851
typedef struct struct_0 {
    char field_0;
    char padding_1[23];
    unsigned int field_18;
} struct_0;

unsigned int file_newer_p(long long a0, struct_0 *a1, long long *a2)
{
    long long v3;  // rax
    long long v4;  // rdx
    unsigned int v5;  // eax
    struct_0 *v0;  // [bp-0xb8]
    char v1;  // [bp-0xa8]

    v0 = a1;
    if (!v0)
    {
        if (sub_42a9b8(a0, &v1, &v1))
        {
            if (*(__errno_location()) != 2)
            {
                sub_43ea2c(a0);
                return 1;
            }
            return 0;
        }
        v0 = &v1;
    }
    if (((unsigned short)v0->field_18 & 0xf000) != 0x4000 && !(v3 = (long long)(unsigned long long)sub_45fd9e(v0), (unsigned int)(unsigned long long)sub_439e35(a2[31], a2[32], (unsigned long long)sub_45fd9e(v0), v4) > 0))
        v5 = 1;
    else
        v5 = 0;
    return v5 & 1;
}



// Function: maybe_recoverable @ 0x19950
typedef struct struct_0 {
    char field_0;
    char padding_1[23];
    unsigned int field_18;
} struct_0;


unsigned int maybe_recoverable(unsigned long a0, char a1, char *a2)
{
    unsigned int v0;  // [bp-0xb4]
    struct_0 *v1;  // [bp-0xb0]

    v0 = *(__errno_location());
    v1 = NULL;
    if (*(a2))
        return 0;
}



// Function: set_xattr @ 0x19bce
typedef struct struct_0 {
    char padding_0[344];
    unsigned long long field_158;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49f3b0;
extern int g_49f96c;
extern unsigned int g_49fa90;

unsigned int set_xattr(long long a0, struct_0 *a1, unsigned int a2, char a3, unsigned int *a4)
{
    unsigned int i;  // eax
    char v0;  // [bp-0x15]
    unsigned int v1;  // [bp-0x14]

    v0 = 0;
    if (g_49f96c > 0 && a1->field_158)
    {
        v1 = ~(g_49f3b0) & g_49fa90 & 0x1ff;
        while (1)
        {
            do
            {
                if (!mknodat(g_49c268, a0, v1 ^ a2, 0))
                {
                    sub_43d8a1(a1, a0, a3, 0);
                    *(a4) = 1;
                    return 0;
                }
                if (sub_419950(a0, 0, &v0) == 2)
                    return 0;
            } while (i > 2);
            if (!i)
                break;
        }
        sub_428891();
        sub_43e38b(a0);
        return 1;
    }
    return 0;
}



// Function: apply_nonancestor_delayed_set_stat @ 0x19ce9
typedef struct struct_0 {
    char padding_0[112];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
} struct_0;

extern unsigned int g_49c268;
extern void* g_49f3b8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void apply_nonancestor_delayed_set_stat(void* a0, char a1)
{
    char v0;  // [bp-0x282]
    char v1;  // [bp-0x281]
    unsigned int v2;  // [bp-0x280]
    unsigned int v3;  // [bp-0x27c]
    unsigned long len;  // [bp-0x278]
    void* idx;  // [bp-0x270]
    char v6;  // [bp-0x268]
    unsigned long v7;  // [bp-0x260]
    unsigned int v8;  // [bp-0x250]
    struct_0 v9;  // [bp-0x1d8]
    unsigned long v10;  // [bp-0x1a8]
    unsigned long v11;  // [bp-0x1a0]
    unsigned long v12;  // [bp-0x198]
    unsigned long v13;  // [bp-0x190]
    unsigned long v14;  // [bp-0x188]
    unsigned int v15;  // [bp-0x168]
    unsigned int v16;  // [bp-0x164]
    unsigned int v17;  // [bp-0x160]
    unsigned long v18;  // [bp-0xf0]
    unsigned long v19;  // [bp-0xe8]
    unsigned long v20;  // [bp-0xe0]
    unsigned long v21;  // [bp-0xd8]
    unsigned long v22;  // [bp-0x80]
    unsigned long v23;  // [bp-0x78]

    len = strlen(a0);
    v0 = 0;
    while (g_49f3b8)
    {
        idx = g_49f3b8;
        v1 = 0;
        v2 = (int)idx[72];
        v3 = (int)idx[76];
        v0 = (char)idx[88] || v0;
        if (a1 < (char)idx[88])
            return;
        if (len > (long long)idx[152] && *((char *)a0 + idx[152]) && (*((char *)a0 + idx[152]) == 47 || *((char *)a0 + idx[152] - 1) == 47) && !memcmp(a0, (long long)idx[160], (long long)idx[152]))
            return;
        sub_42adf1((int)idx[92]);
        if (v0)
        {
            if (fstatat(g_49c268, (long long)idx[160], &v6, (int)idx[84]))
            {
                sub_43ea03((long long)idx[160]);
                v1 = 1;
            }
            else
            {
                v2 = v8;
                v3 = 0xffffffff;
                if (v6 != (long long)idx[8] || v7 != (long long)idx[16])
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_45e548((long long)idx[160]);
                    error(0, 0, gettext("%s: Directory renamed before its status could be extracted"));
                    g_49fdd0 = 2;
                    v1 = 1;
                }
            }
        }
        if (v1 != 1)
        {
            v15 = (int)idx[24];
            v16 = (int)idx[28];
            v17 = (int)idx[32];
            v18 = (long long)idx[40];
            v19 = (long long)idx[48];
            v20 = (long long)idx[56];
            v21 = (long long)idx[64];
            v10 = (long long)idx[96];
            v11 = (long long)idx[104];
            v12 = (long long)idx[112];
            v13 = (long long)idx[120];
            v14 = (long long)idx[128];
            v23 = (long long)idx[144];
            v22 = (long long)idx[0x88];
            sub_418913((long long)idx[160], &v9, 0xffffffff, v2, v3, 53, (char)idx[80], (int)idx[84]);
        }
        g_49f3b8 = *((long long *)idx);
        sub_4193d1(idx);
    }
    return;
}



// Function: is_directory_link @ 0x1a10f
extern unsigned int g_49c268;

char is_directory_link(long long a0)
{
    unsigned int v5;  // eax
    unsigned int v0;  // [bp-0xb0]
    unsigned int v1;  // [bp-0xac]
    char v2;  // [bp-0xa8]
    unsigned int v3;  // [bp-0x90]

    v0 = *(__errno_location());
    if (!fstatat(g_49c268, a0, &v2, 0x100) && ((unsigned short)v3 & 0xf000) == 0xa000 && !fstatat(g_49c268, a0, &v2, 0) && ((unsigned short)v3 & 0xf000) == 0x4000)
        v5 = 1;
    else
        v5 = 0;
    v1 = v5;
    *(__errno_location()) = v0;
    return v1;
}



// Function: safe_dir_mode @ 0x1a1ec
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_49f3a8;
extern int g_49f95c;
extern int g_49f960;

unsigned int safe_dir_mode(struct_0 *a0)
{
    unsigned int v1;  // eax

    if (g_49f95c > 0 || g_49f960 > 0)
        v1 = 448;
    else
        v1 = 0x1ff;
    return (!g_49f3a8 ? 192 : 0) | a0->field_18 & v1;
}



// Function: extract_dir @ 0x1a238
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49f3b0;
extern char g_49f8be;
extern char g_49f8d8;
extern char g_49f8dc;
extern char g_49f935;
extern struct_0 g_49fa20;
extern struct_0 g_49fa78;
extern unsigned long long g_49fa80;
extern unsigned long long g_49fa88;
extern void g_49fa90;
extern unsigned long long g_49fa98;
extern unsigned long long g_49faa0;
extern unsigned long long g_49faa8;
extern unsigned long long g_49fab0;
extern unsigned long long g_49fab8;
extern unsigned long long g_49fac0;
extern unsigned long long g_49fac8;
extern unsigned long long g_49fad0;
extern unsigned long long g_49fad8;
extern unsigned long long g_49fae0;
extern unsigned long long g_49fae8;
extern unsigned long long g_49faf0;
extern unsigned long long g_49faf8;
extern unsigned long long g_49fb00;
extern unsigned long long g_49fc20;

unsigned int extract_dir(long long a0, unsigned int a1)
{
    int i;  // eax
    char v0;  // [bp-0xbd]
    unsigned int v1;  // [bp-0xbc]
    unsigned int v2;  // [bp-0xb8]
    unsigned int v3;  // [bp-0xb4]
    unsigned int v4;  // [bp-0xb0]
    unsigned int v5;  // [bp-0xac]
    char v6;  // [bp-0xa8]
    unsigned long v7;  // [bp-0xa0]
    unsigned long v8;  // [bp-0x98]
    unsigned long v9;  // [bp-0x90]
    unsigned long v10;  // [bp-0x88]
    unsigned long v11;  // [bp-0x80]
    unsigned long v12;  // [bp-0x78]
    unsigned long v13;  // [bp-0x70]
    unsigned long v14;  // [bp-0x68]
    unsigned long v15;  // [bp-0x60]
    unsigned long v16;  // [bp-0x58]
    unsigned long v17;  // [bp-0x50]
    unsigned long v18;  // [bp-0x48]
    unsigned long v19;  // [bp-0x40]
    unsigned long v20;  // [bp-0x38]
    unsigned long v21;  // [bp-0x30]
    unsigned long v22;  // [bp-0x28]
    unsigned long v23;  // [bp-0x20]

    v2 = 0;
    v3 = 0;
    v4 = 0;
    v0 = 0;
    if (g_49f935 && !g_49fc20)
    {
        if (fstatat(g_49c268, ".", &v6, 0))
            sub_42b444(".");
        else
            g_49fc20 = v6;
    }
    if (g_49f8be)
    {
        sub_425377(a0);
    }
    else if (a1 == 0x44)
    {
        sub_428891();
    }
    v5 = sub_41a1ec(&g_49fa78.field_0);
    do
    {
        v1 = mkdirat(g_49c268, a0, v5, a0);
        if (!v1)
        {
            v2 = ~(g_49f3b0) & v5;
            v3 = 0x1ff;
            v4 = 0x100;
            goto LABEL_41a647;
        }
        if (*(__errno_location()) == 0x11 && (v0 || g_49f8dc || *((int *)&g_49f8d8) == 1 || !*((int *)&g_49f8d8) || *((int *)&g_49f8d8) == 2))
        {
            if (g_49f8dc && sub_41a10f(a0))
                return 0;
            if (!sub_42a9b8(a0, &v6, &v6) && !(v2 = (unsigned int)v9, v3 = 0xffffffff, ((unsigned short)v2 & 0xf000) != 0x4000))
            {
                if (v0)
                {
                    sub_4191c5(a0, &v6);
                    return 0;
                }
                if (*((int *)&g_49f8d8) == 1)
                {
                    v5 = sub_41a1ec(&v6);
                    v1 = sub_4183ee(-0x1, a0, v5, 0x100, 53);
                    if (!v1)
                    {
                        g_49fa78.field_0 = v6;
                        g_49fa80 = v7;
                        g_49fa88 = v8;
                        *((unsigned long *)&g_49fa90) = v9;
                        g_49fa98 = v10;
                        g_49faa0 = v11;
                        g_49faa8 = v12;
                        g_49fab0 = v13;
                        g_49fab8 = v14;
                        g_49fac0 = v15;
                        g_49fac8 = v16;
                        g_49fad0 = v17;
                        g_49fad8 = v18;
                        g_49fae0 = v19;
                        g_49fae8 = v20;
                        g_49faf0 = v21;
                        g_49faf8 = v22;
                        g_49fb00 = v23;
                        v2 = ~(g_49f3b0) & v5;
                        v3 = 0x1ff;
                        v4 = 0x100;
                        goto LABEL_41a647;
                    }
                    else
                    {
                        sub_43e0b2(a0, v5, v5);
                    }
                }
                goto LABEL_41a647;
            }
            else
            {
                *(__errno_location()) = 0x11;
            }
        }
        i = sub_419950(a0, 0, &v0);
        if (i == 2 || i > 2)
            break;
        if (!i && *(__errno_location()) != 0x11)
        {
            sub_43e310(a0);
            return 1;
        }
    } while (i == 1);
LABEL_41a647:
    if (!v1 || !*((int *)&g_49f8d8) || *((int *)&g_49f8d8) == 2)
        sub_418d26(a0, &g_49fa20.field_0, v2, v3, *((int *)&g_49fa90), v4);
    return v1;
}



// Function: open_output_file @ 0x1a6b5
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
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49ec84;
extern unsigned int g_49f3b0;
extern unsigned int g_49f3c8;
extern char g_49f8a0;
extern unsigned int g_49f8d8;
extern struct_0 *g_49fdc8;

unsigned int open_output_file(long long a0, unsigned int a1, unsigned int a2, unsigned int a3, unsigned int *a4, unsigned int *a5)
{
    char v0;  // [bp-0xb5]
    unsigned int v1;  // [bp-0xb4]
    unsigned int v2;  // [bp-0xb0]
    unsigned int v3;  // [bp-0xac]
    stat v4;  // [bp-0xa8]

    v0 = g_49f8d8 == 2;
    v1 = (!v0 ? 526785 : (!g_49f8a0 ? 658241 : 527169));
    if (a3)
        v1 &= 0xffffff7f;
    if (a1 == 55 && !g_49f3c8)
    {
        g_49f3c8 = 1;
        if ((char)g_49ec84 & 8)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Extracting contiguous files as regular files"));
        }
    }
    v2 = openat(g_49c268, a0, v1, a2);
    if (v2 < 0)
        return v2;
    if (v0)
    {
        if (fstat(v2, &v4))
        {
            v3 = *(__errno_location());
            close(v2);
            *(__errno_location()) = v3;
            return 0xffffffff;
        }
        if (((unsigned short)v4.st_nlink & 0xf000) != 0x8000)
        {
            close(v2);
            *(__errno_location()) = 0x11;
            return 0xffffffff;
        }
        *(a4) = v4.st_nlink;
        *(a5) = 0xffffffff;
    }
    else
    {
        *(a4) = ~(g_49f3b0) & a2;
        *(a5) = 0x1ff;
    }
    return v2;
}



// Function: extract_file @ 0x1a8d0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f8d8;
extern int g_49f95c;
extern char g_49f998;
extern char g_49f9a0;
extern char g_49fa20;
extern unsigned int g_49fa90;
extern unsigned long long g_49faa8;
extern char g_49fb40;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int extract_file(long long a0, unsigned int a1)
{
    char v0;  // [bp-0x4d]
    unsigned int v1;  // [bp-0x4c]
    unsigned int v2;  // [bp-0x48]
    unsigned int v3;  // [bp-0x44]
    unsigned int v4;  // [bp-0x40]
    unsigned int v5;  // [bp-0x3c]
    unsigned int v6;  // [bp-0x38]
    unsigned int v7;  // [bp-0x34]
    unsigned int i[2];  // [bp-0x30]
    unsigned long v9;  // [bp-0x28]
    unsigned long v10;  // [bp-0x20]
    unsigned long v11;  // [bp-0x18]

    v0 = 0;
    v4 = (g_49f95c <= 0 ? 0xffffffff : 0xffffffc0) & g_49fa90 & 0x1ff;
    v5 = (g_49f95c <= 0 ? 0 : v4 & 63);
    v1 = 0;
    v2 = 0;
    if (g_49f998)
    {
        v3 = 1;
    }
    else if (*((long long *)&g_49f9a0))
    {
        v3 = sub_434323(a0, 0x66, &g_49fa20);
        if (v3 < 0)
        {
            sub_428891();
            return 0;
        }
    }
    else
    {
        *(i) = 0;
        if (sub_419bce(a0, &g_49fa20, v5, a1, i))
            return 1;
        while (1)
        {
            v3 = sub_41a6b5(a0, a1, v4, *(&i[0]), &v1, &v2);
            if (v3 >= 0)
                break;
            v6 = sub_419950(a0, 1, &v0);
            if (v6 != 1)
            {
                sub_428891();
                if (v6 != 2)
                {
                    sub_43e38b(a0);
                    return 1;
                }
                return 0;
            }
        }
    }
    sub_40ad2b(&g_49fa20);
    if (g_49fb40)
    {
        sub_4302a4(v3, &g_49fa20, i, &g_49fa20);
    }
    else
    {
        i = (unsigned int (32 bits)[2])g_49faa8;
        while (i > 0)
        {
            sub_40ad86(i);
            v10 = sub_40bb4b();
            if (!v10)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Unexpected EOF in archive"));
                g_49fdd0 = 2;
                break;
            }
            v9 = sub_40bbf5(v10);
            if (v9 > i)
                v9 = (unsigned long)i;
            *(__errno_location()) = 0;
            v11 = sub_42aa95(v3, v10, v9, v10);
            i -= v9;
            sub_40bba7(v10 + v9 - 1);
            if (v11 != v9)
            {
                if (!*((long long *)&g_49f9a0))
                    sub_43eb4b(a0, v11, v9, v11);
                break;
            }
        }
    }
    sub_4287c1(i);
    sub_40ad66();
    if (g_49f998)
        return 0;
    if (!*((long long *)&g_49f9a0))
        sub_418913(a0, &g_49fa20, v3, v1, v2, a1, 0, (g_49f8d8 == 2 ? 0 : 0x100));
    v7 = close(v3);
    if (v7 < 0)
        sub_43e1ff(a0);
    if (!*((long long *)&g_49f9a0))
        return v7;
    sub_4343ea();
    return v7;
}



// Function: find_delayed_link_source @ 0x1ac61
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern unsigned int g_49c268;
extern struct_0 *g_49f3c0;

struct_0 * find_delayed_link_source(long long a0)
{
    struct_0 *i;  // [bp-0xb0]
    char v1;  // [bp-0xa8]
    unsigned long v2;  // [bp-0xa0]

    if (!g_49f3c0)
    {
        return NULL;
    }
    else if (fstatat(g_49c268, a0, &v1, 0x100))
    {
        if (*(__errno_location()) == 2)
            return NULL;
        sub_43ea03(a0);
        return NULL;
    }
    else
    {
        for (i = g_49f3c0; i && (i->field_8 != v1 || i->field_10 != v2); i = i->field_0);
        return i;
    }
}



// Function: create_placeholder_file @ 0x1ad5b
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

extern unsigned int g_49c268;
extern struct_0 *g_49f3c0;
extern unsigned int g_49f600;
extern long long g_49fa20;
extern char *g_49fa38;
extern long long g_49fa50;
extern unsigned int g_49fa90;
extern unsigned int g_49fa94;
extern unsigned int g_49fa98;
extern unsigned long long g_49fb08;
extern unsigned long long g_49fb10;
extern unsigned long long g_49fb18;
extern unsigned long long g_49fb20;

unsigned int create_placeholder_file(unsigned long a0, char a1, char *a2, struct_0 **a3)
{
    unsigned int v5;  // eax
    struct_0 *idx;  // rbx
    unsigned long v8;  // rdx
    struct_0 *index;  // rcx
    struct_0 *idx1;  // rcx
    unsigned int v0;  // [bp-0xcc]
    struct_0 *idx2;  // [bp-0xc8]
    void* v2;  // [bp-0xc0]
    stat v3;  // [bp-0xb8]

    while (1)
    {
        v0 = openat(g_49c268, a0, 193, 0);
        if (v0 >= 0)
            break;
        if (*(__errno_location()) == 0x11 && sub_41ac61(a0))
            return 0;
        v5 = sub_419950(a0, 0, a2);
        if (v5 == 2)
        {
            return 0;
        }
        else if (v5 <= 2 && !v5)
        {
            sub_43e38b(a0);
            return 0xffffffff;
        }
    }
    if (fstat(v0, &v3))
    {
        sub_43ea03(a0);
        close(v0);
        return 0xffffffff;
    }
    else if (close(v0))
    {
        sub_43e1ff(a0);
        return 0xffffffff;
    }
    else
    {
        idx2 = sub_460b5b(strlen(g_49fa38) + 161);
        if (a3)
        {
            idx2->field_0 = *(a3);
            *(a3) = idx2;
        }
        else
        {
            idx2->field_0 = g_49f3c0;
            g_49f3c0 = idx2;
        }
        idx2->field_8 = *((unsigned long long *)&v3);
        idx2->field_10 = v3.st_ino;
        idx = idx2;
        idx[1].field_0 = sub_45fdb8(&v3);
        idx[1].field_8 = v8;
        *((char *)&idx2[1].field_10) = a1;
        if (a1)
        {
            *((unsigned int *)((char *)&idx2[1].field_10 + 4)) = g_49fa90;
            *((unsigned int *)&idx2[2].field_0) = g_49fa94;
            *((unsigned int *)((char *)&idx2[2].field_0 + 4)) = g_49fa98;
            index = idx2;
            index[2].field_8 = g_49fb08;
            index[2].field_10 = g_49fb10;
            idx1 = idx2;
            idx1[3].field_0 = g_49fb18;
            idx1[3].field_8 = g_49fb20;
        }
    }
    *((unsigned int *)&idx2[3].field_10) = g_49f600;
    idx2[4].field_0 = sub_460b5b(strlen(a0) + 9);
    idx2[4].field_0->field_0 = 0;
    strcpy(&idx2[4].field_0->field_8, a0);
    idx2[4].field_8 = 0;
    sub_429547(&idx2[4].field_8, g_49fa50, &idx2[4].field_8);
    memset(&idx2[4].field_10, 0, 32);
    sub_41d854(&g_49fa20, &idx2[6].field_8, idx2 + 6);
    strcpy(&idx2[6].field_10, g_49fa38);
    v2 = sub_418b89(a0);
    if (v2)
        sub_418c40(v2);
    return 0;
}



// Function: extract_link @ 0x1b161
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    char padding_8[16];
    long long field_18;
    long long field_20;
    char padding_28[56];
    struct struct_0 *field_60;
} struct_1;

extern unsigned int g_49c268;
extern struct_1 *g_49f3c0;
extern unsigned int g_49f600;
extern char g_49f878;
extern char g_49f8be;
extern unsigned long long g_49fa38;

unsigned int extract_link(unsigned long a0, unsigned int a1)
{
    long long v14;  // rax
    long long v15;  // rdx
    unsigned int v0;  // [bp-0x174]
    char v1;  // [bp-0x165]
    unsigned int v2;  // [bp-0x164]
    unsigned int v3;  // [bp-0x160]
    unsigned int i;  // [bp-0x15c]
    struct_1 *idx;  // [bp-0x158]
    unsigned long v6;  // [bp-0x150]
    void* v7;  // [bp-0x148]
    struct_0 **v8;  // [bp-0x140]
    char v9;  // [bp-0x138]
    unsigned long v10;  // [bp-0x130]
    char v11;  // [bp-0xa8]
    unsigned long v12;  // [bp-0xa0]

    v0 = a1;
    v1 = 0;
    v6 = g_49fa38;
    if (g_49f878 != 1 && (char)sub_42efdc(v6))
        return sub_41ad5b(a0, 0, &v1, NULL);
    v7 = sub_41ac61(v6);
    if (v7)
        return sub_41ad5b(a0, 0, &v1, v7);
    do
    {
        v2 = linkat(g_49c268, v6, g_49c268, a0, 0);
        v3 = *(__errno_location());
        if (!v2)
        {
            if (!idx || fstatat(g_49c268, v6, &v9, 0x100))
                return 0;
            for (idx = g_49f3c0; idx; idx = idx->field_0)
            {
                if (*((int *)&idx->padding_28[48]) == g_49f600 && *((long long *)&idx->padding_8[0]) == v9 && *((long long *)&idx->padding_8[8]) == v10 && !(v14 = (long long)(unsigned long long)sub_45fdb8(&v9), (unsigned int)(unsigned long long)sub_45ff90(idx->field_18, idx->field_20, (unsigned long long)sub_45fdb8(&v9), v15)))
                {
                    v8 = sub_460b5b(strlen(a0) + 9);
                    strcpy(v8 + 1, a0);
                    *(v8) = idx->field_60;
                    idx->field_60 = v8;
                    break;
                }
            }
            return 0;
        }
        if (v3 == 0x11 && !strcmp(v6, a0) || !fstatat(g_49c268, v6, &v9, 0x100) && !fstatat(g_49c268, a0, &v11, 0x100) && v9 == v11 && v10 == v12)
            return 0;
        *(__errno_location()) = v3;
        i = sub_419950(a0, 0, &v1);
    } while (i == 1);
    if (i == 2)
        return 0;
    if (g_49f8be == 1 && *(__errno_location()) == 0x11)
        return 0;
    sub_43e277(v6, a0, a0);
    return 1;
}



// Function: extract_symlink @ 0x1b508
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern unsigned int g_49c268;
extern char g_49f878;
extern struct_0 g_49fa20;
extern struct_0 *g_49fa38;

unsigned int extract_symlink(long long a0, unsigned int a1)
{
    unsigned int i;  // eax
    unsigned int v0;  // [bp-0x24]
    char v1;  // [bp-0x11]

    v0 = a1;
    v1 = 0;
    if (g_49f878 != 1 && ((char)g_49fa38->field_0 == 47 || (char)sub_42efdc(g_49fa38)))
        return sub_41ad5b(a0, 1, &v1, NULL);
    while (1)
    {
        do
        {
            if (!symlinkat(g_49fa38, g_49c268, a0, g_49c268))
            {
                sub_418913(a0, &g_49fa20.field_0, 0xffffffff, 0, 0, 50, 0, 0x100);
                return 0;
            }
            if (sub_419950(a0, 0, &v1) == 2)
                return 0;
        } while (i > 2);
        if (!i)
            break;
    }
    sub_43e944(g_49fa38, a0, a0);
    return 0xffffffff;
}



// Function: extract_node @ 0x1b63a
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49f3b0;
extern int g_49f95c;
extern struct_0 g_49fa20;
extern unsigned int g_49fa90;
extern long long g_49faa0;

unsigned int extract_node(long long a0, char a1)
{
    unsigned int i;  // eax
    char v0;  // [bp-0x15]
    unsigned int v1;  // [bp-0x14]

    v0 = 0;
    v1 = (g_49f95c <= 0 ? 0xffffffff : 0xffffffc0) & g_49fa90 & 0x61ff;
    while (1)
    {
        do
        {
            if (!mknodat(g_49c268, a0, v1, g_49faa0))
            {
                sub_418913(a0, &g_49fa20.field_0, 0xffffffff, ~(g_49f3b0) & v1, 0x1ff, a1, 0, 0x100);
                return 0;
            }
            if (sub_419950(a0, 0, &v0) == 2)
                return 0;
        } while (i > 2);
        if (!i)
            break;
    }
    sub_43e362(a0);
    return 0xffffffff;
}



// Function: extract_fifo @ 0x1b745
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49f3b0;
extern int g_49f95c;
extern struct_0 g_49fa20;
extern unsigned int g_49fa90;

unsigned int extract_fifo(long long a0, char a1)
{
    unsigned int i;  // eax
    char v0;  // [bp-0x15]
    unsigned int v1;  // [bp-0x14]

    v0 = 0;
    v1 = (g_49f95c <= 0 ? 0xffffffff : 0xffffffc0) & g_49fa90 & 0x1ff;
    while (1)
    {
        do
        {
            if (!mkfifoat(g_49c268, a0, v1, a0))
            {
                sub_418913(a0, &g_49fa20.field_0, 0xffffffff, ~(g_49f3b0) & v1, 0x1ff, a1, 0, 0x100);
                return 0;
            }
            if (sub_419950(a0, 0, &v0) == 2)
                return 0;
        } while (i > 2);
        if (!i)
            break;
    }
    sub_43e339(a0);
    return 0xffffffff;
}



// Function: prepare_to_extract @ 0x1b84c
typedef struct struct_0 {
    char field_0;
    char padding_1[23];
    unsigned int field_18;
} struct_0;

extern unsigned int g_49ec84;
extern unsigned int g_49f8d8;
extern char g_49f94c;
extern char g_49f998;
extern unsigned long long g_49f9a0;
extern long long g_49fa20;
extern long long g_49fa28;
extern char g_49fa30;
extern unsigned int g_49fa90;
extern char g_49fba8;
extern char g_49fc31;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int prepare_to_extract(long long a0, unsigned int a1, unsigned long long *a2)
{
    unsigned long long v0;  // [bp-0x20]

    v0 = 0;
    switch (a1)
    {
    case 0: case 48: case 55:
        v0 = (!g_49fa30 ? sub_41a8d0 : sub_41a238);
        break;
    case 49:
        v0 = sub_41b161;
        break;
    case 50:
        v0 = sub_41b508;
        break;
    case 51:
        g_49fa90 = g_49fa90 | 32;
        v0 = sub_41b63a;
        break;
    case 52:
        g_49fa90 = g_49fa90 | 96;
        v0 = sub_41b63a;
        break;
    case 53: case 68:
        v0 = sub_41a238;
        if (g_49fba8)
            g_49fc31 = 1;
        break;
    case 54:
        v0 = sub_41b745;
        break;
    case 75: case 76:
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Unexpected long name header"));
        g_49fdd0 = 2;
        return 0;
    case 77:
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(g_49fa28);
        error(0, 0, gettext("%s: Cannot extract -- file is continued from another volume"));
        g_49fdd0 = 2;
        return 0;
    case 83:
        v0 = sub_41a8d0;
        break;
    case 86:
        return 0;
    default:
        if (g_49ec84 & 0xffffff & 0x10000)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548(a0);
            error(0, 0, gettext("%s: Unknown file type '%c', extracted as normal file"));
        }
        v0 = sub_41a8d0;
        break;
    }
    if (g_49f998 || g_49f9a0)
    {
        if (v0 != sub_41a8d0)
            return 0;
    }
    else
    {
        if (g_49f8d8 != 3)
        {
            if (g_49f8d8 == 6 && (char)sub_419851(a0, NULL, &g_49fa20))
            {
                if (!((unsigned short)g_49ec84 & 0x800))
                    return 0;
                if (g_49fdc8)
                    g_49fdc8();
                sub_45e84e(a0);
                error(0, 0, gettext("Current %s is newer or same age"));
                return 0;
            }
        }
        else
        {
            if (!sub_42a36f(a0, g_49f94c, g_49f94c) && *(__errno_location()) && *(__errno_location()) != 2)
            {
                sub_43eaa7(a0);
                return 0;
            }
        }
    }
    *(a2) = v0;
    return 1;
}



// Function: extract_archive @ 0x1bbcb
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned long long g_49f388;
extern struct_0 *g_49f460;
extern unsigned int g_49f600;
extern char g_49f878;
extern char g_49f884;
extern char g_49f8c8;
extern unsigned int g_49f9ac;
extern long long g_49fa20;
extern struct_2 *g_49fa28;
extern char g_49fc31;
extern struct_1 *g_49fdc8;
extern unsigned int g_49fdd0;

void extract_archive(void)
{
    char v6;  // al
    long long v9;  // rdi
    long long v10;  // rsi
    long long v11;  // rdx
    long long v12;  // rcx
    long long v13;  // r8
    long long v14;  // r9
    char v0;  // [bp-0x32]
    char v1;  // [bp-0x31]
    unsigned int v2;  // [bp-0x30]
    unsigned int v3;  // [bp-0x2c]
    unsigned long long *v4;  // [bp-0x28]

    g_49f388 = sub_41c2ee;
    sub_40bba7(g_49f460);
    if (g_49f878 != 1 && (char)sub_42efdc(g_49fa20))
        v6 = 1;
    else
        v6 = 0;
    v0 = v6;
    v0 &= 1;
    if (v0)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(g_49fa20);
        error(0, 0, gettext("%s: Member name contains '..'"));
        g_49fdd0 = 2;
    }
    if ((char)g_49fa28->field_0 && !v0 && (!g_49f8c8 || sub_434af4("extract", g_49fa28)))
    {
        if (g_49f9ac)
            sub_42864c(&g_49fa20, g_49f460, -0x1);
        if (g_49fc31 != 1)
        {
            v2 = g_49f600;
            sub_419ce9(g_49fa28, 0);
            sub_42adf1(v2);
        }
        if (g_49f884 && (char)sub_42a571(g_49fa28, 0) != 1)
        {
            v3 = *(__errno_location());
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548(g_49fa28);
            error(0, v3, gettext("%s: Was unable to backup this file"));
            g_49fdd0 = 2;
            sub_428891(v9, v10, v11, v12, v13, v14);
            return;
        }
        v1 = (!(char)sub_4301de(&g_49fa20) ? *((char *)&g_49f460[9].field_8 + 4) : 83);
        if (!(char)sub_41b84c(g_49fa28, v1, &v4))
        {
            sub_428891();
        }
        else if (!v4(g_49fa28, v1, v1, v4))
        {
            return;
        }
        if (!g_49f884)
            return;
        sub_42a874();
        return;
    }
    sub_428891();
    return;
}



// Function: apply_delayed_links @ 0x1be65
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
    unsigned long long field_38;
    unsigned long long field_40;
    unsigned long long field_48;
    unsigned long long field_50;
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
    char field_a0;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern unsigned int g_49c268;
extern struct_1 *g_49f3c0;

void apply_delayed_links(void)
{
    long long v24;  // rax
    long long v25;  // rdx
    struct_1 *ptr;  // [bp-0x298]
    struct_0 **ptr1;  // [bp-0x290]
    long long v2;  // [bp-0x288]
    struct_1 *v3;  // [bp-0x280]
    struct_0 **v4;  // [bp-0x278]
    long long v5;  // [bp-0x270]
    char v6;  // [bp-0x268]
    unsigned long v7;  // [bp-0x260]
    struct_0 v8;  // [bp-0x1d8]
    unsigned long v9;  // [bp-0x1a8]
    unsigned long v10;  // [bp-0x1a0]
    unsigned long v11;  // [bp-0x198]
    unsigned long v12;  // [bp-0x190]
    unsigned long v13;  // [bp-0x188]
    unsigned int v14;  // [bp-0x168]
    unsigned int v15;  // [bp-0x164]
    unsigned int v16;  // [bp-0x160]
    unsigned long v17;  // [bp-0xf0]
    unsigned long v18;  // [bp-0xe8]
    unsigned long v19;  // [bp-0xe0]
    unsigned long v20;  // [bp-0xd8]
    unsigned long v21;  // [bp-0x80]
    unsigned long v22;  // [bp-0x78]

    for (ptr = g_49f3c0; ptr; ptr = v3)
    {
        ptr1 = ptr->field_60;
        v2 = 0;
        sub_42adf1(ptr->field_58);
        for (ptr1 = ptr->field_60; ptr1; ptr1 = *(ptr1))
        {
            v5 = ptr1 + 1;
            if (!fstatat(g_49c268, v5, &v6, 0x100) && v6 == ptr->field_8 && v7 == ptr->field_10)
            {
                v24 = sub_45fdb8(&v6);
                if (!sub_45ff90(v24, v25, ptr->field_18, ptr->field_20))
                {
                    if (unlinkat(g_49c268, v5, 0, v5))
                    {
                        sub_43eaa7(v5);
                    }
                    else if (!v2 || linkat(g_49c268, v2, g_49c268, v5, 0))
                    {
                        if (ptr->field_28 != 1)
                        {
                            if (linkat(g_49c268, &ptr->field_a0, g_49c268, v5, 0))
                                sub_43e277(&ptr->field_a0, v5, &ptr->field_a0);
                        }
                        else
                        {
                            if (symlinkat(&ptr->field_a0, g_49c268, v5, &ptr->field_a0))
                            {
                                sub_43e944(&ptr->field_a0, v5, &ptr->field_a0);
                            }
                            else
                            {
                                v14 = ptr->field_2c;
                                v15 = ptr->field_30;
                                v16 = ptr->field_34;
                                v17 = ptr->field_38;
                                v18 = ptr->field_40;
                                v19 = ptr->field_48;
                                v20 = ptr->field_50;
                                v9 = ptr->field_68;
                                v10 = ptr->field_70;
                                v11 = ptr->field_78;
                                v12 = ptr->field_80;
                                v13 = ptr->field_88;
                                v22 = ptr->field_98;
                                v21 = ptr->field_90;
                                sub_418913(v5, &v8, 0xffffffff, 0, 0, 50, 0, 0x100);
                                v2 = v5;
                            }
                        }
                    }
                }
            }
        }
        for (ptr1 = ptr->field_60; ptr1; ptr1 = v4)
        {
            v4 = *(ptr1);
            free(ptr1);
        }
        sub_41d55d(ptr->field_98, ptr->field_90, ptr->field_90);
        free(ptr->field_68);
        v3 = ptr->field_0;
        free(ptr);
    }
    g_49f3c0 = 0;
    return;
}



// Function: extract_finish @ 0x1c2ee
extern void g_481623;

void extract_finish(void)
{
    sub_419ce9(&g_481623, 0);
    sub_41be65();
    sub_419ce9(&g_481623, 1);
    return;
}



// Function: rename_directory @ 0x1c326
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int rename_directory(unsigned long a0, unsigned long a1)
{
    char v0;  // [bp-0x25]
    unsigned int v1;  // [bp-0x24]

    if (!renameat(g_49c268, a0, g_49c268, a1))
    {
        sub_4194f9(a0, a1);
        return 1;
    }
    v1 = *(__errno_location());
    if (v1 == 2)
    {
        if (!sub_4195aa(a1, &v0))
        {
            if (!renameat(g_49c268, a0, g_49c268, a1))
                return 1;
            v1 = *(__errno_location());
        }
    }
    if (g_49fdc8)
        g_49fdc8();
    sub_45e821(1, a1);
    sub_45e821(0, a0);
    error(0, v1, gettext("Cannot rename %s to %s"));
    g_49fdd0 = 2;
    return 0;
}



// Function: x_obstack_grow @ 0x1c47d
typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
    unsigned long long field_20;
} struct_0;

struct_1 * x_obstack_grow(struct_1 *index, void* a1, unsigned long n)
{
    struct_0 *idx;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x10]

    idx = index->field_0;
    v1 = idx;
    if (v1->field_20 - v1->field_18 < n)
        _obstack_newchunk(idx, n, n);
    memcpy(idx->field_18, a1, n);
    idx->field_18 = idx->field_18 + n;
    index->field_8 = index->field_8 + n;
    return index;
}



// Function: x_obstack_1grow @ 0x1c52b
typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

struct_2 * x_obstack_1grow(struct_2 *index, char a1)
{
    char *v3;  // rax
    struct_0 *idx;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    idx = index->field_0;
    v1 = idx;
    if (!(v1->field_20 - (char *)v1->field_18))
        _obstack_newchunk(idx, 1);
    v3 = &idx->field_18->field_0;
    idx->field_18 = v3 + 1;
    *(v3) = a1;
    index->field_8 = index->field_8 + 1;
    return index;
}



// Function: x_obstack_blank @ 0x1c5b0
typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
    unsigned long long field_20;
} struct_1;

struct_0 * x_obstack_blank(struct_0 *index, long long a1)
{
    unsigned long long *idx;  // [bp-0x20]
    unsigned long long *v1;  // [bp-0x10]

    idx = index->field_0;
    v1 = idx;
    if (v1[4] - v1[3] < a1)
        _obstack_newchunk(idx, a1, a1);
    idx[3] = idx[3] + a1;
    index->field_8 = index->field_8 + a1;
    return index;
}



// Function: xheader_keyword_deleted_p @ 0x1c63f
typedef struct struct_0 {
    struct struct_0 *field_0;
    char *field_8;
} struct_0;

extern struct_0 *g_49f3d8;

unsigned int xheader_keyword_deleted_p(char *a0)
{
    struct_0 *i;  // [bp-0x10]

    for (i = g_49f3d8; i; i = i->field_0)
    {
        if (!fnmatch(i->field_8, a0, 0))
            return 1;
    }
    return 0;
}



// Function: xheader_keyword_override_p @ 0x1c69c
typedef struct struct_0 {
    struct struct_0 *field_0;
    char *field_8;
} struct_0;

extern struct_0 *g_49f3e8;

unsigned int xheader_keyword_override_p(char *a0)
{
    struct_0 *i;  // [bp-0x10]

    for (i = g_49f3e8; i; i = i->field_0)
    {
        if (!strcmp(i->field_8, a0))
            return 1;
    }
    return 0;
}



// Function: xheader_list_append @ 0x1c6f4
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

struct_0 ** xheader_list_append(struct_0 **a0, long long a1, long long a2)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_460b5b(24);
    idx->field_8 = sub_460cc6(a1);
    idx->field_10 = (!a2 ? 0 : sub_460cc6(a2));
    idx->field_0 = *(a0);
    *(a0) = idx;
    return a0;
}



// Function: xheader_list_destroy @ 0x1c772
typedef struct struct_0 {
    struct struct_0 *field_0;
    void* field_8;
    void* field_10;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_1;

struct_0 ** xheader_list_destroy(struct_0 **a0)
{
    struct_0 **v3;  // rax
    struct_1 *ptr;  // [bp-0x18]
    struct_1 *v1;  // [bp-0x10]

    if (!a0)
        return v3;
    for (ptr = *(a0); ptr; ptr = v1)
    {
        v1 = ptr->field_0;
        free(ptr->field_8);
        free(ptr->field_10);
        free(ptr);
    }
    *(a0) = NULL;
    return a0;
}



// Function: xheader_set_single_keyword @ 0x1c7ea
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;

void xheader_set_single_keyword(unsigned long long a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Keyword %s is unknown or not yet implemented"));
    sub_43814c(2); /* do not return */
}



// Function: assign_time_option @ 0x1c846
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void assign_time_option(long long a0, long long *a1, long long a2)
{
    long long v3;  // rdx
    char v0;  // [bp-0x30]
    long long v1;  // [bp-0x28]

    v1 = sub_429ff7(a2, &v0, 0, &v0);
    if ((char)sub_4294f0(v1, v3, v1) == 1 && !*((char *)v0))
    {
        *(a1) = v1;
        sub_429547(a0, a2, a2);
        return;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Time stamp is out of allowed range"));
    g_49fdd0 = 2;
    return;
}



// Function: xheader_set_keyword_equal @ 0x1c92e
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f3d8;
extern struct_0 *g_49f3e0;
extern struct_0 *g_49f3e8;
extern long long g_49f3f8;
extern long long g_49f400;
extern long long g_49f408;
extern long long g_49f410;
extern long long g_49f418;
extern long long g_49f420;
extern struct_0 *g_49fdc8;

long long xheader_set_keyword_equal(void* a0, void* a1)
{
    unsigned long long *v4;  // rax
    unsigned long long *v5;  // rax
    char v0;  // [bp-0x21]
    void* iter;  // [bp-0x20]

    v0 = 1;
    iter = a1;
    if (a1 == a0)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e84e(a0);
        error(0, 0, gettext("Malformed pax option: %s"));
        sub_43814c(2); /* do not return */
    }
    else
    {
        if (*((char *)a1 - 1) == 58)
        {
            iter -= 1;
            v0 = 0;
        }
        for (; iter > a0 && (v4 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)(char)*((char *)iter) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000); iter -= 1);
        *((char *)iter) = 0;
        for (iter = a1 + 1; *((char *)iter) && (v5 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)(char)*((char *)iter) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000); iter += 1);
        if (!strcmp(a0, "delete"))
        {
            if ((char)sub_41d9d3(iter))
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_45e84e(iter);
                error(0, 0, gettext("Pattern %s cannot be used"));
                sub_43814c(2); /* do not return */
            }
            else
            {
                return sub_41c6f4(&g_49f3d8, iter, 0);
            }
        }
        else
        {
            if (!strcmp(a0, "exthdr.name"))
            {
                return sub_429547(&g_49f3f8, iter);
            }
            else if (!strcmp(a0, "globexthdr.name"))
            {
                return sub_429547(&g_49f410, iter);
            }
            else if (!strcmp(a0, "exthdr.mtime"))
            {
                return (unsigned long long)sub_41c846(&g_49f400, &g_49f408, iter);
            }
            else if (!strcmp(a0, "globexthdr.mtime"))
            {
                return (unsigned long long)sub_41c846(&g_49f418, &g_49f420, iter);
            }
            else if ((char)sub_41da4b(a0))
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Keyword %s cannot be overridden"));
                sub_43814c(2); /* do not return */
            }
            else
            {
                return (!v0 ? sub_41c6f4(&g_49f3e8, a0, iter) : sub_41c6f4(&g_49f3e0, a0, iter));
            }
        }
    }
}



// Function: xheader_set_option @ 0x1cc78
long long xheader_set_option(char *a0)
{
    unsigned long tok;  // rax
    unsigned long i;  // [bp-0x18]
    void* ptr;  // [bp-0x10]

    tok = strtok(a0, ",");
    for (i = tok; i; i = tok)
    {
        ptr = strchr(i, 61);
        if (!ptr)
            sub_41c7ea(i); /* do not return */
        sub_41c92e(i, ptr);
        tok = strtok(NULL, ",");
    }
    return tok;
}



// Function: xheader_format_name @ 0x1cd02
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

extern char g_49f878;

void* xheader_format_name(long long *a0, struct_0 *a1, long long a2)
{
    unsigned int v16;  // eax
    struct_0 *v17;  // rdx
    void* v18;  // rax
    struct_0 *v19;  // rdx
    void* v20;  // rax
    struct_0 *v21;  // rdx
    void* v22;  // rax
    long long node;  // [bp-0x90]
    void* iter;  // [bp-0x88]
    void* v2;  // [bp-0x88]
    struct_0 *iter1;  // [bp-0x80]
    unsigned long v4;  // [bp-0x78]
    char *v5;  // [bp-0x70]
    char *v6;  // [bp-0x68]
    char *v7;  // [bp-0x60]
    char *v8;  // [bp-0x58]
    void* v9;  // [bp-0x50]
    char v10;  // [bp-0x48]
    char v11;  // [bp-0x34]
    char v12;  // [bp-0x28]
    char v13;  // [bp-0x14]

    v4 = 0;
    v5 = NULL;
    v6 = NULL;
    v7 = NULL;
    v8 = NULL;
    node = 0;
    for (iter1 = a1; iter1->field_0; iter1 = &iter1->field_1)
    {
        if (iter1->field_0 == 37 && iter1->field_1)
        {
            iter1 = &iter1->field_1;
            switch (iter1->field_0)
            {
            case 112:
                v7 = sub_454e5a((int)getpid(), &v10, &v10);
                node += &v11 - v7;
                break;
            case 110:
                v8 = sub_454e5a(a2, &v12, &v12);
                node += &v13 - v8;
                break;
            case 102:
                if (a0)
                {
                    v6 = sub_44df20(*(a0));
                    node += strlen(v6);
                }
                break;
            case 37:
                node += 1;
                break;
            case 100:
                if (a0)
                {
                    if (!v4)
                        v4 = sub_44e10c(*(a0));
                    v5 = sub_43edd9(v4, 0, g_49f878);
                    node += strlen(v5);
                }
                goto LABEL_41cf0c;
            default:
                node += 2;
                goto LABEL_41cf0c;
            }
        }
        else
        {
            node += 1;
        }
LABEL_41cf0c:
    }
    v9 = sub_460b5b(node + 1);
    iter = v9;
    for (iter1 = a1; iter1->field_0; *((char *)v22) = v21->field_0)
    {
        if (iter1->field_0 == 37)
        {
            v16 = iter1->field_1;
            switch (v16)
            {
            case 112:
                iter = stpcpy(iter, v7);
                iter1 += 1;
                break;
            case 110:
                iter = stpcpy(iter, v8);
                iter1 += 1;
                break;
            case 102:
                if (v6)
                    iter = stpcpy(iter, v6);
                iter1 += 1;
                break;
            case 37:
                v17 = iter1;
                iter1 = &v17->field_1;
                v18 = iter;
                iter = v18 + 1;
                *((char *)v18) = v17->field_0;
                iter1 = &iter1->field_1;
                break;
            case 100:
                if (v5)
                    iter = stpcpy(iter, v5);
                iter1 += 1;
                break;
            default:
                v19 = iter1;
                iter1 = &v19->field_1;
                v20 = iter;
                iter = v20 + 1;
                *((char *)v20) = v19->field_0;
                if (iter1->field_0)
                {
                    iter1 = &iter1->field_1;
                    v2 = iter + 1;
                    *((char *)iter) = iter1->field_0;
                    iter = v2;
                    break;
                }
            }
        }
        else
        {
            v21 = iter1;
            iter1 = &v21->field_1;
            v22 = iter;
            iter = v22 + 1;
        }
    }
    free(v4);
    for (; iter > v9 && *((char *)iter - 1) == 47; iter -= 1);
    *((char *)iter) = 0;
    return v9;
}



// Function: xheader_xhdr_name @ 0x1d123
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

extern unsigned long long g_49c240[4];
extern struct_0 *g_49f3f8;
extern char g_49f9c8;

void* xheader_xhdr_name(long long *a0)
{
    unsigned long long v1;  // rdx

    if (!g_49f3f8)
    {
        v1 = g_49f9c8 * 8;
        sub_429547(&g_49f3f8, *((long long *)(v1 + (char *)&g_49c240[0])), v1);
    }
    return sub_41cd02(a0, g_49f3f8, 0);
}



// Function: xheader_ghdr_name @ 0x1d18d
extern unsigned long long g_49c240[4];
extern unsigned long long g_49f3d0;
extern void* g_49f410;
extern char g_49f9c8;

void* xheader_ghdr_name(void)
{
    unsigned long len;  // rax
    char *ptr;  // [bp-0x30]
    unsigned long long *v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]

    if (g_49f410)
        return sub_41cd02(NULL, g_49f410, g_49f3d0 + 1);
    v1 = g_49c240[g_49f9c8 + 2];
    ptr = getenv("TMPDIR");
    if (!ptr)
        ptr = "/tmp";
    len = strlen(ptr);
    v2 = strlen(v1) + len + 1;
    g_49f410 = sub_460b5b(v2);
    strcpy(g_49f410, ptr);
    strcat(g_49f410, v1);
    return sub_41cd02(NULL, g_49f410, g_49f3d0 + 1);
}



// Function: xheader_write @ 0x1d280
typedef struct struct_1 {
    char padding_0[156];
    char field_9c;
} struct_1;

typedef struct struct_0 {
    char padding_0[8];
    char field_8;
    char padding_9[7];
    unsigned long long field_10;
} struct_0;

extern unsigned long long g_49f3d0;
extern unsigned long long g_49f400;
extern unsigned long long g_49f408;
extern unsigned long long g_49f418;
extern unsigned long long g_49f420;

unsigned long long xheader_write(char a0, unsigned int a1, unsigned long long a2, struct_0 *idx)
{
    unsigned long long v7;  // rax
    unsigned long long v0;  // [bp-0x40]
    unsigned long i;  // [bp-0x28]
    unsigned long iter;  // [bp-0x20]
    unsigned long n;  // [bp-0x18]
    struct_1 *ptr;  // [bp-0x10], Other Possible Types: unsigned long

    v0 = a2;
    i = *((long long *)&idx->field_8);
    switch (a0)
    {
    case 103:
        if (g_49f418)
            v0 = g_49f420;
        break;
    case 120:
        if (g_49f400)
            v0 = g_49f408;
        break;
    default:
        goto LABEL_41d2ef;
    }
LABEL_41d2ef:
    ptr = sub_412e0f(a1, i, v0);
    ptr->field_9c = a0;
    sub_41418c(ptr);
    iter = idx->field_10;
    do
    {
        ptr = sub_40bb4b();
        n = 0x200;
        if (n > i)
            n = i;
        memcpy(ptr, iter, n);
        if (n <= 0x1ff)
            memset(ptr + n, 0, 0x200 - n);
        iter += n;
        i -= n;
        sub_40bba7(ptr);
    } while (i);
    v7 = sub_41e785(idx);
    if (a0 != 103)
        return v7;
    g_49f3d0 = g_49f3d0 + 1;
    return g_49f3d0;
}



// Function: xheader_write_global @ 0x1d3e6
typedef struct struct_0 {
    struct struct_0 *field_0;
    long long field_8;
    long long field_10;
} struct_0;

extern struct_0 *g_49f3e0;
extern unsigned long long g_49f9e0;

long long xheader_write_global(unsigned long long *a0)
{
    struct_0 *i;  // [bp-0x18]
    void* v1;  // [bp-0x10]

    if (g_49f3e0)
    {
        sub_41e173(a0);
        for (i = g_49f3e0; i; i = i->field_0)
        {
            sub_41ecd7(i->field_10, i->field_8, a0, i->field_8);
        }
    }
    if (!*(a0))
        return *(a0);
    sub_41e672(a0);
    v1 = sub_41d18d();
    sub_41d280(103, *((unsigned int *)&v1), g_49f9e0, a0);
    return (unsigned long long)free(v1);
}



// Function: xheader_forbid_global @ 0x1d498
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49f3e0;
extern struct_0 *g_49fdc8;

unsigned long long xheader_forbid_global(void)
{
    if (!g_49f3e0)
        return 0;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("can't update global extended header record"));
    sub_43814c(2); /* do not return */
}



// Function: xheader_xattr_init @ 0x1d4f4
unsigned long long * xheader_xattr_init(unsigned long long *idx)
{
    idx[44] = 0;
    idx[43] = 0;
    memset(idx + 7, 0, 32);
    idx[6] = 0;
    return idx;
}



// Function: xheader_xattr_free @ 0x1d55d
void xheader_xattr_free(void* a0, unsigned long long a1)
{
    unsigned long long i;  // [bp-0x10]

    for (i = 0; i < a1; i += 1)
    {
        free(*((long long *)((char *)a0 + 24 * i)));
        free(*((long long *)((char *)a0 + 24 * i + 8)));
    }
    free(a0);
    return;
}



// Function: xheader_xattr__add @ 0x1d5e6
long long xheader_xattr__add(long long *a0, long long *a1, long long a2, long long a3, long long a4)
{
    long long v2;  // rax
    unsigned long v3;  // rcx
    unsigned long long v4;  // rax
    unsigned long long v5;  // rax
    unsigned long long v6;  // rax
    long long idx;  // [bp-0x20]

    v2 = *(a1);
    v3 = v2 + 1;
    *(a1) = v3;
    idx = v2;
    v4 = *(a1) * 3;
    *(a0) = sub_460b8d(*(a0), v4 * 8, v4 * 8, v3);
    v5 = sub_460cc6(a2);
    *((unsigned long long *)(*(a0) + idx * 24)) = v5;
    v6 = sub_460c8e(a3, a4 + 1, a4 + 1, idx);
    *((unsigned long long *)(*(a0) + idx * 24 + 8)) = v6;
    *((long long *)(*(a0) + idx * 24 + 16)) = a4;
    return a4;
}



// Function: xattr_decode_keyword @ 0x1d6da
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

long long xattr_decode_keyword(struct_0 *a0)
{
    unsigned long v3;  // rax
    struct_0 *node;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    iter = a0;
    node = iter;
    while (1)
    {
        if (node->field_0 != 37)
        {
LABEL_41d763:
            iter->field_0 = node->field_0;
            if (!node->field_0)
                break;
            node = &node->field_1;
            iter = &iter->field_1;
        }
        else if (node->field_1 == 0x33 && node->field_2 == 0x44)
        {
            iter->field_0 = 61;
            node += 1;
            iter = &iter->field_1;
        }
        else
        {
            if (node->field_1 != 50 || node->field_2 != 53)
                goto LABEL_41d763;
            iter->field_0 = 37;
            node += 1;
            iter = &iter->field_1;
        }
    }
    return v3;
}



// Function: xheader_xattr_add @ 0x1d78e
void xheader_xattr_add(long long *a0, char *a1, long long a2, long long a3)
{
    unsigned long len;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]
    char *v2;  // [bp-0x10], Other Possible Types: unsigned long

    len = strlen(a1);
    v1 = sub_460b5b(strlen("SCHILY.xattr.") + len + 1);
    v2 = v1;
    v2 = stpcpy(v2, "SCHILY.xattr.");
    stpcpy(v2, a1);
    sub_41d5e6(a0 + 44, a0 + 43, v1, a2, a3);
    free(v1);
    return;
}



// Function: xheader_xattr_copy @ 0x1d854
typedef struct struct_0 {
    char padding_0[352];
    unsigned long long field_160;
} struct_0;

unsigned long long xheader_xattr_copy(struct_0 *idx, unsigned long long *a1, unsigned long long *a2)
{
    unsigned long long index;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    index = 0;
    *(a1) = 0;
    for (*(a2) = 0; index < *((long long *)&idx->padding_0[344]); index += 1)
    {
        v1 = *((long long *)(index * 24 + idx->field_160));
        v2 = *((long long *)(index * 24 + idx->field_160 + 8));
        v3 = *((long long *)(index * 24 + idx->field_160 + 16));
        sub_41d5e6(a1, a2, v1, v2, v3);
    }
    return *((long long *)&idx->padding_0[344]);
}



// Function: locate_handler @ 0x1d940
typedef struct struct_0 {
    char *field_0;
    char padding_8[20];
    char field_1c;
} struct_0;

extern struct_0 g_49a260;

struct_0 * locate_handler(char *a0)
{
    unsigned long len;  // rax
    struct_0 *i;  // [bp-0x10]

    for (i = &g_49a260.field_0; i->field_0; i = (char *)&i[1].field_0 + 3)
    {
        if (i->field_1c)
        {
            len = strlen(i->field_0);
            if (!strncmp(i->field_0, a0, len))
                return i;
        }
        else if (!strcmp(i->field_0, a0))
        {
            return i;
        }
    }
    return NULL;
}



// Function: xheader_protected_pattern_p @ 0x1d9d3
typedef struct struct_0 {
    char *field_0;
    char padding_8[16];
    unsigned int field_18;
    char field_1c;
} struct_0;

extern struct_0 g_49a260;

unsigned int xheader_protected_pattern_p(char *a0)
{
    struct_0 *i;  // [bp-0x10]

    for (i = &g_49a260.field_0; i->field_0; i = (char *)&i[1].field_0 + 3)
    {
        if (i->field_1c != 1 && (char)i->field_18 & 1 && !fnmatch(a0, i->field_0, 0))
            return 1;
    }
    return 0;
}



// Function: xheader_protected_keyword_p @ 0x1da4b
typedef struct struct_0 {
    char *field_0;
    char padding_8[16];
    unsigned int field_18;
    char field_1c;
} struct_0;

extern struct_0 g_49a260;

unsigned int xheader_protected_keyword_p(char *a0)
{
    struct_0 *i;  // [bp-0x10]

    for (i = &g_49a260.field_0; i->field_0; i = (char *)&i[1].field_0 + 3)
    {
        if (i->field_1c != 1 && (char)i->field_18 & 1 && !strcmp(i->field_0, a0))
            return 1;
    }
    return 0;
}



// Function: decode_record @ 0x1dabe
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_1 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int decode_record(unsigned long long *idx, struct_0 **a1, unsigned long long *a2, unsigned long a3)
{
    char *v13;  // rax
    unsigned int v0;  // [bp-0x4c]
    char v1;  // [bp-0x48]
    char *ptr;  // [bp-0x40]
    char *v3;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x30]
    unsigned long long v5;  // [bp-0x28]
    char *v6;  // [bp-0x20]
    char *v7;  // [bp-0x18]

    v3 = &*(a1)->field_0;
    ptr = v3;
    for (v4 = idx[1] + idx[2] - v3; *(ptr) == 32 || *(ptr) == 9; ptr += 1);
    if (*(ptr) - 48 > 9)
    {
        if (!*(ptr))
            return 0;
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Malformed extended header: missing length"));
        g_49fdd0 = 2;
        return 0;
    }
    else
    {
        v5 = strtoumax(ptr, &v1, 10, &v1);
        if (v4 < v5)
        {
            v0 = (unsigned int)(v1 - ptr);
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Extended header length %*s is out of range"));
            g_49fdd0 = 2;
            return 0;
        }
        else
        {
            v6 = &v3[v5];
            for (ptr = (char *)v1; *(ptr) == 32 || *(ptr) == 9; ptr += 1);
            if (ptr == v1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Malformed extended header: missing blank after length"));
                g_49fdd0 = 2;
                return 0;
            }
            else
            {
                v7 = ptr;
                ptr = strchr(ptr, 61);
                if (ptr && ptr < v6)
                {
                    if (*(v6 - 1) != 10)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("Malformed extended header: missing newline"));
                        g_49fdd0 = 2;
                        return 0;
                    }
                    else
                    {
                        v13 = v6 - 1;
                        *(v13) = 0;
                        *(ptr) = *(v13);
                        a2(a3, v7, ptr + 1, v6 - ptr - 2, a2);
                        *(ptr) = 61;
                        *(v6 - 1) = 10;
                        *(a1) = v6;
                        return 1;
                    }
                }
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Malformed extended header: missing equal sign"));
                g_49fdd0 = 2;
                return 0;
            }
        }
    }
}



// Function: run_override_list @ 0x1de07
typedef struct struct_0 {
    struct struct_0 *field_0;
    char *field_8;
    char *field_10;
} struct_0;

struct_0 * run_override_list(struct_0 *a0, unsigned long a1)
{
    struct_0 *v3;  // rax
    unsigned long len;  // rax
    struct_0 *i;  // [bp-0x30]
    struct_0 *v1;  // [bp-0x20]

    for (i = a0; i; i = i->field_0)
    {
        v1 = sub_41d940(i->field_8);
        if (v1)
        {
            len = strlen(i->field_10);
            v1->field_10(a1, v1->field_0, i->field_10, len);
        }
        v3 = i->field_0;
    }
    return v3;
}



// Function: decx @ 0x1de86
typedef struct struct_0 {
    char *field_0;
    char padding_8[20];
    char field_1c;
} struct_0;

extern unsigned int g_49ec84;
extern struct_0 *g_49fdc8;

long long decx(unsigned long a0, char *a1, unsigned long a2, unsigned long a3)
{
    unsigned long v2;  // rax
    unsigned long v3;  // rax
    struct_0 *v0;  // [bp-0x10]

    v2 = sub_41c63f(a1);
    if (!(char)v2)
    {
        v2 = sub_41c69c(a1);
        if (!(char)v2)
        {
            v0 = sub_41d940(a1);
            if (v0)
                return (*((long long *)&v0->padding_8[8]))(a0, a1, a2, a3, *((long long *)&v0->padding_8[8]));
            v3 = g_49ec84 & 0x20000;
            if (!(unsigned int)v3)
                return v3;
            if (!g_49fdc8)
                return (unsigned long long)error(0, 0, gettext("Ignoring unknown extended header keyword '%s'"));
            g_49fdc8();
            return (unsigned long long)error(0, 0, gettext("Ignoring unknown extended header keyword '%s'"));
        }
    }
    return v2;
}



// Function: xheader_decode @ 0x1df5f
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

extern struct_0 *g_49f3e0;
extern struct_0 *g_49f3e8;
extern struct_0 *g_49f3f0;

void xheader_decode(struct_0 *idx)
{
    unsigned long v0;  // [bp-0x18]

    sub_41de07(g_49f3e0, idx);
    sub_41de07(g_49f3f0, idx);
    if (idx->field_170)
    {
        v0 = idx->field_178 + 0x200;
        do
        { } while ((char)sub_41dabe(&idx->padding_151[23], &v0, sub_41de86, idx));
    }
    sub_41de07(g_49f3e8, idx);
    idx->field_118 = idx->field_88;
    if (!idx->field_150)
        return;
    idx->field_88 = idx->field_148;
    return;
}



// Function: decg @ 0x1e063
typedef struct struct_0 {
    char *field_0;
    char padding_8[20];
    char field_1c;
} struct_0;

struct_0 ** decg(struct_0 **a0, unsigned long a1, long long a2, unsigned long a3)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_41d940(a1);
    if (v0 && (char)*((int *)&v0->padding_8[16]) & 2)
        return (*((long long *)&v0->padding_8[8]))(a0, a1, a2, a3, *((long long *)&v0->padding_8[8]));
    return sub_41c6f4(a0, a1, a2);
}



// Function: xheader_decode_global @ 0x1e0e7
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f3f0;

void xheader_decode_global(unsigned long long *a0)
{
    unsigned long v0;  // [bp-0x18]

    if (!a0[1])
        return;
    v0 = a0[2] + 0x200;
    sub_41c772(&g_49f3f0);
    do
    { } while ((char)sub_41dabe(a0, &v0, sub_41e063, &g_49f3f0));
    return;
}



// Function: xheader_init @ 0x1e173
long long xheader_init(long long *a0)
{
    if (*(a0))
        return *(a0);
    *(a0) = sub_460b5b(88);
    return _obstack_begin(*(a0), 0, 0, sub_460b5b, free);
}



// Function: xheader_store @ 0x1e1d3
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[376];
    struct struct_0 *field_178;
} struct_1;

struct_0 * xheader_store(char *a0, struct_1 *a1, unsigned long a2)
{
    struct_0 *v2;  // rax
    struct_0 *v3;  // rax
    struct_0 *v4;  // rax
    struct_0 *v0;  // [bp-0x10]

    if (a1->field_178)
        return a1->field_178;
    v2 = sub_41d940(a0);
    v0 = v2;
    if (v0)
    {
        v2 = *((long long *)&v0->padding_8[0]);
        if (*((long long *)&v0->padding_8[0]))
        {
            v3 = sub_41c63f(a0);
            if (*((char *)&v3))
                return v3;
            sub_41e173(&a1->padding_0[360]);
            v4 = sub_41c69c(a0) ^ 1;
            if (!*((char *)&v4))
                return v4;
            return (*((long long *)&v0->padding_8[0]))(a1, a0, &a1->padding_0[360], a2, *((long long *)&v0->padding_8[0]));
        }
    }
    return v2;
}



// Function: xheader_read @ 0x1e28e
typedef struct struct_0 {
    char padding_0[8];
    char field_8;
    char padding_9[7];
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_1 *g_49fdc8;

long long xheader_read(struct_0 *idx, unsigned long a1, unsigned long a2)
{
    unsigned long iter;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x18]
    unsigned long n;  // [bp-0x10]

    v1 = a1;
    iter = a2;
    v2 = 0;
    if (iter - 0 >> 63 & 1)
        iter = 0;
    if (iter > 0xfffffffffffffdfe)
        sub_41783c(); /* do not return */
    iter += 0x200;
    *((unsigned long *)&idx->field_8) = iter;
    idx->field_10 = sub_460b5b(iter + 1);
    *((char *)(iter + idx->field_10)) = 0;
    while (1)
    {
        n = iter;
        if (n > 0x200)
            n = 0x200;
        if (!v1)
            break;
        memcpy(idx->field_10 + v2, v1, n);
        sub_40bba7(v1);
        v1 = sub_40bb4b();
        v2 += n;
        iter -= n;
        if (iter <= NULL)
            return iter - n;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Unexpected EOF in archive"));
    sub_4177f4(); /* do not return */
}



// Function: xattr_encode_keyword @ 0x1e3ca
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f428;
extern long long g_49f430;

char * xattr_encode_keyword(char *a0)
{
    char *node;  // [bp-0x20]
    char v1;  // [bp-0x11]
    long long iter;  // [bp-0x10]

    node = a0;
    if (!g_49f428)
    {
        g_49f430 = 0x100;
        g_49f428 = sub_460b5b(g_49f430);
    }
    else
    {
        *((char *)&g_49f428->field_0) = 0;
    }
    for (iter = 0; *(node); node += 1)
    {
        v1 = *(node);
        if (iter + 2 >= g_49f430)
            g_49f428 = sub_460bd8(g_49f428, &g_49f430, &g_49f430);
        switch (v1)
        {
        case 37:
            strcpy(iter + (char *)g_49f428, "%25");
            iter += 2;
            break;
        case 61:
            strcpy(iter + (char *)g_49f428, "%3D");
            iter += 2;
            break;
        default:
            *((char *)g_49f428 + iter) = v1;
            break;
        }
        iter += 1;
    }
    *(iter + (char *)g_49f428) = 0;
    return g_49f428;
}



// Function: xheader_print_n @ 0x1e4ff
void xheader_print_n(void* a0, char *a1, void* a2, unsigned int a3)
{
    char *v0;  // [bp-0x68], Other Possible Types: void*
    char *i;  // [bp-0x50]
    unsigned long len;  // [bp-0x48]
    unsigned long v3;  // [bp-0x40]
    char *v4;  // [bp-0x38]
    void* v5;  // [bp-0x30]
    char v6;  // [bp-0x28]
    char v7;  // [bp-0x14]

    v0 = a1;
    i = NULL;
    v0 = sub_41e3ca(v0);
    len = strlen(v0);
    v3 = a3 + len + 3;
    do
    {
        v4 = i;
        v5 = sub_454e5a(&v4[v3], &v6, &v4[v3]);
        i = &v7 - v5;
    } while (i != v4);
    sub_41c47d(a0, v5, i);
    sub_41c52b(a0, 32);
    sub_41c47d(a0, v0, len);
    sub_41c52b(a0, 61);
    sub_41c47d(a0, a2, a3);
    sub_41c52b(a0, 10);
    return;
}



// Function: xheader_print @ 0x1e634
void xheader_print(void* a0, char *a1, void* a2)
{
    sub_41e4ff(a0, a1, a2, strlen(a2));
    return;
}



// Function: xheader_finish @ 0x1e672
typedef struct struct_0 {
    struct struct_0 *field_0;
    long long field_8;
    long long field_10;
} struct_0;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[8];
    unsigned long long field_10;
} struct_2;

typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
} struct_1;

extern struct_0 *g_49f3e8;

struct_2 * xheader_finish(struct_2 *a0)
{
    struct_0 *i;  // [bp-0x20]
    struct_1 *idx;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10]

    for (i = g_49f3e8; i; i = i->field_0)
    {
        sub_41ecd7(i->field_10, i->field_8, a0, i->field_8);
    }
    idx = a0->field_0;
    v2 = idx->field_10;
    if (v2 == idx->field_18)
        idx->field_50 = idx->field_50 | 2;
    idx->field_18 = ~(idx->field_30) & idx->field_18 + idx->field_30;
    if (idx->field_18 - idx->field_8 > idx->field_20 - idx->field_8)
        idx->field_18 = idx->field_20;
    idx->field_10 = idx->field_18;
    a0->field_10 = v2;
    return a0;
}



// Function: xheader_destroy @ 0x1e785
typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    void* field_10;
} struct_1;

typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    long long field_20;
} struct_0;

struct_1 * xheader_destroy(struct_1 *index)
{
    struct_0 *idx;  // [bp-0x18]
    long long v1;  // [bp-0x10]

    if (index->field_0)
    {
        idx = index->field_0;
        v1 = 0;
        if (v1 > *((long long *)&idx->padding_0[8]) && v1 < idx->field_20)
        {
            idx->field_10 = v1;
            idx->field_18 = idx->field_10;
        }
        else
        {
            _obstack_free(idx, v1, v1);
        }
        free(index->field_0);
        index->field_0 = NULL;
    }
    else
    {
        free(index->field_10);
    }
    index->field_10 = NULL;
    index->field_8 = 0;
    return index;
}



// Function: xheader_string_begin @ 0x1e848
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

struct_0 * xheader_string_begin(struct_0 *a0)
{
    a0->field_18 = 0;
    return a0;
}



// Function: xheader_string_add @ 0x1e863
typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
    unsigned long long field_20;
} struct_0;

struct_1 * xheader_string_add(unsigned long long *idx, void* a1)
{
    unsigned long len;  // rax

    if (idx[2])
        return idx[2];
    sub_41e173(idx);
    len = strlen(a1);
    idx[3] = idx[3] + len;
    return sub_41c47d(idx, a1, strlen(a1));
}



// Function: xheader_string_end @ 0x1e8dc
typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[16];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
} struct_1;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int xheader_string_end(struct_2 *a0, char *a1)
{
    unsigned long len;  // rax
    char *i;  // [bp-0x78]
    unsigned long v1;  // [bp-0x70]
    char *v2;  // [bp-0x68], Other Possible Types: unsigned long
    char *v3;  // [bp-0x60]
    unsigned long v4;  // [bp-0x58]
    void* iter;  // [bp-0x50], Other Possible Types: char *
    struct_1 *index;  // [bp-0x48]
    struct_1 *idx;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    char v10;  // [bp-0x28]
    char v11;  // [bp-0x14]

    i = NULL;
    if (a0[1].field_0)
        return 0;
    sub_41e173(a0);
    len = strlen(a1);
    v1 = a0[1].field_8 + len + 3;
    do
    {
        v2 = i;
        v3 = sub_454e5a(&v2[v1], &v10, &v2[v1]);
        i = &v11 - v3;
    } while (i != v2);
    v2 = &i[strlen(a1) + 2];
    v4 = v2;
    if (v4 == v2)
    {
        sub_41c5b0(a0, v2);
        sub_41c52b(a0, 10);
        iter = -(a0[1].field_8) - v2 + (char *)a0->field_0->field_18 - 1;
        memmove(iter + v2, iter, a0[1].field_8);
        iter = stpcpy(iter, v3);
        iter += 1;
        *(iter) = 32;
        iter = stpcpy(iter, a1);
        iter += 1;
        *(iter) = 61;
        return 1;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Generated keyword/value pair is too long (keyword=%s, length=%s)"));
    g_49fdd0 = 2;
    index = a0->field_0;
    idx = a0->field_0;
    v8 = idx->field_10;
    if (v8 == idx->field_18)
        idx->field_50 = idx->field_50 | 2;
    idx->field_18 = ~(idx->field_30) & idx->field_18 + idx->field_30;
    if (idx->field_18 - *((long long *)&idx->padding_0[8]) > *((long long *)&idx->padding_20[0]) - *((long long *)&idx->padding_0[8]))
        idx->field_18 = *((long long *)&idx->padding_20[0]);
    idx->field_10 = idx->field_18;
    v9 = v8;
    if (v9 > *((long long *)&index->padding_0[8]) && v9 < *((long long *)&index->padding_20[0]))
    {
        index->field_10 = v9;
        index->field_18 = index->field_10;
    }
    else
    {
        _obstack_free(index, v9, v9);
    }
    return 0;
}



// Function: out_of_range_header @ 0x1ec02
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void out_of_range_header(unsigned long long a0, unsigned long long a1, long long a2, long long a3)
{
    unsigned long long v0;  // [bp-0x88]
    unsigned long long v1;  // [bp-0x68]
    unsigned long long v2;  // [bp-0x60]
    unsigned long long v3;  // [bp-0x58]
    unsigned long long v4;  // [bp-0x50]
    char v5;  // [bp-0x48]
    char v6;  // [bp-0x28]

    v2 = a0;
    v1 = a1;
    v3 = sub_454bde(a2, &v5, &v5);
    v4 = sub_454e5a(a3, &v6, &v6);
    if (g_49fdc8)
        g_49fdc8();
    v0 = v4;
    error(0, 0, gettext("Extended header %s=%s is out of range %s..%s"));
    g_49fdd0 = 2;
    return;
}



// Function: code_string @ 0x1ecd7
void code_string(long long a0, char *a1, void* a2)
{
    void* ptr;  // [bp-0x18], Other Possible Types: char

    if ((char)sub_43c8a0(1, a0, &ptr) == 1)
    {
        sub_41e634(a2, a1, ptr);
        free(ptr);
        return;
    }
    ptr = sub_460cc6(a0);
    sub_41e634(a2, a1, ptr);
    free(ptr);
    return;
}



// Function: decode_string @ 0x1ed64
long long decode_string(void* *a0, long long a1)
{
    unsigned long v1;  // rax

    if (*(a0))
    {
        free(*(a0));
        *(a0) = NULL;
    }
    v1 = sub_43c8a0(0, a1, a0) ^ 1;
    if (!(char)v1)
        return v1;
    return sub_429547(a0, a1, a1);
}



// Function: code_time @ 0x1edd0
void code_time(long long a0, long long a1, char *a2, void* a3)
{
    char v0;  // [bp-0x38]

    sub_41e634(a3, a2, sub_429f2e(a0, a1, &v0, a0, a0));
    return;
}



// Function: decode_time @ 0x1ee4e
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int decode_time(long long *a0, char *a1, unsigned long long a2)
{
    long long v3;  // rdx
    char v0;  // [bp-0x30]
    long long v1;  // [bp-0x28]

    v1 = sub_429ff7(a1, &v0, 1, &v0);
    if ((char)sub_4294f0(v1, v3, v1) == 1)
    {
        *(a0) = v1;
        a0[1] = v3;
        return 1;
    }
    if (a1 < v0 && !*((char *)v0))
    {
        sub_41ec02(a2, a1, -0x8000000000000000, 0x7fffffffffffffff);
    }
    else
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Malformed extended header: invalid %s=%s"));
        g_49fdd0 = 2;
    }
    return 0;
}



// Function: code_signed_num @ 0x1ef79
void code_signed_num(long long a0, char *a1, long long a2, long long a3, void* a4)
{
    char v0;  // [bp-0x28]

    sub_41e634(a4, a1, sub_429c96(a0, a2, a3, &v0));
    return;
}



// Function: code_num @ 0x1efed
void code_num(long long a0, char *a1, void* a2)
{
    sub_41ef79(a0, a1, 0, -0x1, a2);
    return;
}



// Function: decode_signed_num @ 0x1f02b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int decode_signed_num(unsigned long long *a0, unsigned long a1, long long a2, long long a3, unsigned long long a4)
{
    char v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]

    v1 = sub_429cfc(a1, &v0, a2, a3);
    if (*(__errno_location()) != 22 && !*((char *)v0))
    {
        if (*(__errno_location()) == 0x22)
        {
            sub_41ec02(a4, a1, a2, a3);
            return 0;
        }
        *(a0) = v1;
        return 1;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Malformed extended header: invalid %s=%s"));
    g_49fdd0 = 2;
    return 0;
}



// Function: dummy_coder @ 0x1f1bd
void dummy_coder(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    return;
}



// Function: dummy_decoder @ 0x1f1d8
void dummy_decoder(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    return;
}



// Function: atime_coder @ 0x1f1f3
void atime_coder(long long *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41edd0(a0[29], a0[30], a1, a2);
    return;
}



// Function: atime_decoder @ 0x1f237
void atime_decoder(unsigned long long *idx, unsigned long long a1, char *a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x48]
    long long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]

    v0 = a3;
    if (!(char)sub_41ee4e(&v1, a2, a1))
        return;
    idx[29] = v1;
    idx[30] = v2;
    return;
}



// Function: gid_coder @ 0x1f2ae
typedef struct struct_0 {
    char padding_0[120];
    unsigned int field_78;
} struct_0;

void gid_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ef79(a0->field_78, a1, 0, 0xffffffff, a2);
    return;
}



// Function: gid_decoder @ 0x1f2f6
typedef struct struct_0 {
    char padding_0[120];
    unsigned int field_78;
} struct_0;

void gid_decoder(struct_0 *a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f02b(&v1, a2, 0, 0xffffffff, a1))
        return;
    a0->field_78 = v1;
    return;
}



// Function: gname_coder @ 0x1f36d
typedef struct struct_0 {
    char padding_0[40];
    long long field_28;
} struct_0;

void gname_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ecd7(a0->field_28, a1, a2);
    return;
}



// Function: gname_decoder @ 0x1f3a7
long long gname_decoder(void* *a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a3;
    return sub_41ed64(a0 + 5, a2);
}



// Function: linkpath_coder @ 0x1f3dd
typedef struct struct_0 {
    char padding_0[24];
    long long field_18;
} struct_0;

void linkpath_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ecd7(a0->field_18, a1, a2);
    return;
}



// Function: linkpath_decoder @ 0x1f417
long long linkpath_decoder(void* *a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a3;
    return sub_41ed64(a0 + 3, a2);
}



// Function: ctime_coder @ 0x1f44d
void ctime_coder(long long *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41edd0(a0[33], a0[0x22], a1, a2);
    return;
}



// Function: ctime_decoder @ 0x1f491
void ctime_decoder(unsigned long long *idx, unsigned long long a1, char *a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x48]
    long long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]

    v0 = a3;
    if (!(char)sub_41ee4e(&v1, a2, a1))
        return;
    idx[33] = v1;
    idx[0x22] = v2;
    return;
}



// Function: mtime_coder @ 0x1f508
void mtime_coder(long long *a0, char *a1, void* a2, long long *a3)
{
    if (!a3)
    {
        sub_41edd0(a0[31], a0[32], a1, a2);
        return;
    }
    sub_41edd0(*(a3), a3[1], a1, a2);
    return;
}



// Function: mtime_decoder @ 0x1f574
void mtime_decoder(unsigned long long *idx, unsigned long long a1, char *a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x48]
    long long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]

    v0 = a3;
    if (!(char)sub_41ee4e(&v1, a2, a1))
        return;
    idx[31] = v1;
    idx[32] = v2;
    return;
}



// Function: path_coder @ 0x1f5eb
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
} struct_0;

void path_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ecd7(a0->field_8, a1, a2);
    return;
}



// Function: raw_path_decoder @ 0x1f625
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

unsigned long long raw_path_decoder(struct_0 *a0, long long a1)
{
    unsigned long long v1;  // rax

    sub_41ed64(a0, a1);
    sub_41ed64(&a0->field_8, a1);
    v1 = sub_44e2e5(a0->field_8);
    a0->field_10 = v1;
    return v1;
}



// Function: path_decoder @ 0x1f67d
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

unsigned long long path_decoder(struct_0 *a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v3;  // rax
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a3;
    v3 = *((char *)&a0[19].field_8 + 6) ^ 1;
    if ((char)v3)
        v3 = sub_41f625(a0, a2);
    return v3;
}



// Function: sparse_path_decoder @ 0x1f6c1
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

unsigned long long sparse_path_decoder(struct_0 *a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a3;
    *((char *)&a0[19].field_8 + 6) = 1;
    return sub_41f625(a0, a2);
}



// Function: size_coder @ 0x1f6fe
typedef struct struct_0 {
    char padding_0[136];
    long long field_88;
} struct_0;

void size_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41efed(a0->field_88, a1, a2);
    return;
}



// Function: size_decoder @ 0x1f73e
typedef struct struct_0 {
    char padding_0[136];
    unsigned long long field_88;
} struct_0;

void size_decoder(struct_0 *a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, 0x7fffffffffffffff, a1))
        return;
    a0->field_88 = v1;
    return;
}



// Function: uid_coder @ 0x1f7b7
typedef struct struct_0 {
    char padding_0[116];
    unsigned int field_74;
} struct_0;

void uid_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ef79(a0->field_74, a1, 0, 0xffffffff, a2);
    return;
}



// Function: uid_decoder @ 0x1f7ff
typedef struct struct_0 {
    char padding_0[116];
    unsigned int field_74;
} struct_0;

void uid_decoder(struct_0 *a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f02b(&v1, a2, 0, 0xffffffff, a1))
        return;
    a0->field_74 = v1;
    return;
}



// Function: uname_coder @ 0x1f876
typedef struct struct_0 {
    char padding_0[32];
    long long field_20;
} struct_0;

void uname_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ecd7(a0->field_20, a1, a2);
    return;
}



// Function: uname_decoder @ 0x1f8b0
long long uname_decoder(void* *a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a3;
    return sub_41ed64(a0 + 4, a2);
}



// Function: sparse_size_coder @ 0x1f8e6
typedef struct struct_0 {
    char padding_0[136];
    long long field_88;
} struct_0;

void sparse_size_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    sub_41f6fe(a0, a1, a2, a3);
    return;
}



// Function: sparse_size_decoder @ 0x1f91d
typedef struct struct_0 {
    char padding_0[328];
    unsigned long long field_148;
    char field_150;
} struct_0;

void sparse_size_decoder(struct_0 *idx, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, 0x7fffffffffffffff, a1))
        return;
    idx->field_150 = 1;
    idx->field_148 = v1;
    return;
}



// Function: sparse_numblocks_coder @ 0x1f9a1
typedef struct struct_0 {
    char padding_0[304];
    long long field_130;
} struct_0;

void sparse_numblocks_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41efed(a0->field_130, a1, a2);
    return;
}



// Function: sparse_numblocks_decoder @ 0x1f9de
typedef struct struct_0 {
    char padding_0[304];
    unsigned long long field_130;
    long long field_138;
    unsigned long long field_140;
} struct_0;

void sparse_numblocks_decoder(struct_0 *idx, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, -0x1, a1))
        return;
    idx->field_138 = v1;
    idx->field_140 = sub_460c29(v1, 16, v1);
    idx->field_130 = 0;
    return;
}



// Function: sparse_offset_coder @ 0x1fa7f
typedef struct struct_0 {
    char padding_0[320];
    unsigned long long field_140;
} struct_0;

void sparse_offset_coder(struct_0 *a0, char *a1, void* a2, unsigned long long *a3)
{
    sub_41efed(*((long long *)(*(a3) * 16 + a0->field_140)), a1, a2);
    return;
}



// Function: sparse_offset_decoder @ 0x1fad8
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void sparse_offset_decoder(unsigned long long *idx, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, 0x7fffffffffffffff, a1))
    {
        return;
    }
    else if (idx[38] < idx[39])
    {
        *((unsigned long long *)(idx[38] * 16 + idx[40])) = v1;
        return;
    }
    else
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Malformed extended header: excess %s=%s"));
        g_49fdd0 = 2;
        return;
    }
}



// Function: sparse_numbytes_coder @ 0x1fbda
typedef struct struct_0 {
    char padding_0[320];
    unsigned long long field_140;
} struct_0;

void sparse_numbytes_coder(struct_0 *a0, char *a1, void* a2, unsigned long long *a3)
{
    sub_41efed(*((long long *)(*(a3) * 16 + a0->field_140 + 8)), a1, a2);
    return;
}



// Function: sparse_numbytes_decoder @ 0x1fc34
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void sparse_numbytes_decoder(unsigned long long *idx, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v3;  // rdi
    unsigned long long v4;  // rax
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, 0x7fffffffffffffff, a1))
    {
        return;
    }
    else if (idx[38] < idx[39])
    {
        v3 = idx[40];
        v4 = idx[38];
        idx[38] = v4 + 1;
        *((unsigned long long *)(v4 * 16 + v3 + 8)) = v1;
        return;
    }
    else
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Malformed extended header: excess %s=%s"));
        g_49fdd0 = 2;
        return;
    }
}



// Function: sparse_map_decoder @ 0x1fd46
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void sparse_map_decoder(unsigned long long *idx, unsigned long long a1, char *a2, unsigned long long a3)
{
    unsigned long v8;  // rsi
    unsigned long long v9;  // rax
    unsigned long long *v10;  // rcx
    unsigned long long v0;  // [bp-0x78]
    char *v1;  // [bp-0x70]
    unsigned int v2;  // [bp-0x4c]
    char *v3;  // [bp-0x48]
    unsigned long long v4;  // [bp-0x40]
    unsigned long long v5;  // [bp-0x38]
    unsigned long long v6;  // [bp-0x30]

    v1 = a2;
    v0 = a3;
    v2 = 1;
    idx[38] = 0;
    while (1)
    {
        if (*(v1) - 48 > 9)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Malformed extended header: invalid %s=%s"));
            g_49fdd0 = 2;
            return;
        }
        *(__errno_location()) = 0;
        v4 = strtoimax(v1, &v3, 10, &v3);
        if (v2)
        {
            v5 = v4;
            if (*(__errno_location()) == 0x22)
            {
                sub_41ec02(a1, v1, 0, 0x7fffffffffffffff);
                return;
            }
        }
        else
        {
            v6 = v4;
            if (*(__errno_location()) == 0x22)
            {
                sub_41ec02(a1, v1, 0, 0x7fffffffffffffff);
                return;
            }
            if (idx[38] < idx[39])
            {
                v8 = idx[40];
                v9 = idx[38];
                idx[38] = v9 + 1;
                v10 = v8 + v9 * 16;
                *(v10) = v5;
                v10[1] = v6;
            }
            else
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Malformed extended header: excess %s=%s"));
                g_49fdd0 = 2;
                return;
            }
        }
        v2 = !v2;
        if (!*(v3))
            break;
        if (*(v3) != 44)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Malformed extended header: invalid %s: unexpected delimiter %c"));
            g_49fdd0 = 2;
            return;
        }
        v1 = v3 + 1;
    }
    if (v2)
        return;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Malformed extended header: invalid %s: odd number of values"));
    g_49fdd0 = 2;
    return;
}



// Function: dumpdir_coder @ 0x2004a
void dumpdir_coder(unsigned long long a0, char *a1, void* a2, unsigned long a3)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    sub_41e4ff(a2, a1, a3, sub_420bfe(a3));
    return;
}



// Function: dumpdir_decoder @ 0x2008c
typedef struct struct_0 {
    char padding_0[400];
    void* field_190;
} struct_0;

void* dumpdir_decoder(struct_0 *a0, unsigned long long a1, void* a2, unsigned long a3)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    a0->field_190 = sub_460b5b(a3);
    return memcpy(a0->field_190, a2, a3);
}



// Function: volume_label_coder @ 0x200e3
void volume_label_coder(unsigned long long a0, char *a1, void* a2, long long a3)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    sub_41ecd7(a3, a1, a2);
    return;
}



// Function: volume_label_decoder @ 0x20119
extern void* g_49efe0;

long long volume_label_decoder(unsigned long long a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a3;
    return sub_41ed64(&g_49efe0, a2);
}



// Function: volume_size_coder @ 0x2014e
void volume_size_coder(unsigned long long a0, char *a1, void* a2, long long *a3)
{
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    sub_41efed(*(a3), a1, a2);
    return;
}



// Function: volume_size_decoder @ 0x20192
extern unsigned long long g_49eff0;

void volume_size_decoder(unsigned long long a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]

    v1 = a0;
    v0 = a3;
    if (!(char)sub_41f138(&v2, a2, -0x1, a1))
        return;
    g_49eff0 = v2;
    return;
}



// Function: volume_offset_coder @ 0x20201
void volume_offset_coder(unsigned long long a0, char *a1, void* a2, long long *a3)
{
    unsigned long long v0;  // [bp-0x20]

    v0 = a0;
    sub_41efed(*(a3), a1, a2);
    return;
}



// Function: volume_offset_decoder @ 0x20245
extern unsigned long long g_49eff8;

void volume_offset_decoder(unsigned long long a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]

    v1 = a0;
    v0 = a3;
    if (!(char)sub_41f138(&v2, a2, -0x1, a1))
        return;
    g_49eff8 = v2;
    return;
}



// Function: volume_filename_decoder @ 0x202b4
extern void* g_49efe8;

long long volume_filename_decoder(unsigned long long a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = a0;
    v1 = a1;
    v0 = a3;
    return sub_41ed64(&g_49efe8, a2);
}



// Function: xattr_selinux_coder @ 0x202e9
typedef struct struct_0 {
    char padding_0[48];
    long long field_30;
} struct_0;

void xattr_selinux_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41ecd7(a0->field_30, a1, a2);
    return;
}



// Function: xattr_selinux_decoder @ 0x20323
long long xattr_selinux_decoder(void* *a0, unsigned long long a1, long long a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    v1 = a1;
    v0 = a3;
    return sub_41ed64(a0 + 6, a2);
}



// Function: xattr_acls_a_coder @ 0x20359
typedef struct struct_0 {
    char padding_0[56];
    void* field_38;
    unsigned long long field_40;
} struct_0;

void xattr_acls_a_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41e4ff(a2, a1, a0->field_38, a0->field_40);
    return;
}



// Function: xattr_acls_a_decoder @ 0x20398
typedef struct struct_0 {
    char padding_0[56];
    unsigned long long field_38;
    long long field_40;
} struct_0;

struct_0 * xattr_acls_a_decoder(struct_0 *idx, unsigned long long a1, long long a2, long long a3)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    idx->field_38 = sub_460c8e(a2, a3 + 1, a3 + 1);
    idx->field_40 = a3;
    return idx;
}



// Function: xattr_acls_d_coder @ 0x203e2
typedef struct struct_0 {
    char padding_0[72];
    void* field_48;
    unsigned long long field_50;
} struct_0;

void xattr_acls_d_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41e4ff(a2, a1, a0->field_48, a0->field_50);
    return;
}



// Function: xattr_acls_d_decoder @ 0x20421
typedef struct struct_0 {
    char padding_0[72];
    unsigned long long field_48;
    long long field_50;
} struct_0;

struct_0 * xattr_acls_d_decoder(struct_0 *idx, unsigned long long a1, long long a2, long long a3)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    idx->field_48 = sub_460c8e(a2, a3 + 1, a3 + 1);
    idx->field_50 = a3;
    return idx;
}



// Function: xattr_coder @ 0x2046b
typedef struct struct_0 {
    char padding_0[352];
    unsigned long long field_160;
} struct_0;

void xattr_coder(struct_0 *a0, char *a1, void* a2, unsigned long long *a3)
{
    unsigned long v0;  // [bp-0x18]

    v0 = a0->field_160;
    sub_41e4ff(a2, a1, *((long long *)(v0 + *(a3) * 24 + 8)), *((long long *)(v0 + *(a3) * 24 + 16)));
    return;
}



// Function: xattr_decoder @ 0x204f5
void xattr_decoder(long long *a0, void* ptr, void* a2, unsigned long a3)
{
    unsigned long long v6;  // rax
    unsigned long long v7;  // rax
    char v0;  // [bp-0x1048]
    unsigned long v1;  // [bp-0x48]
    unsigned long len1;  // [bp-0x28]
    unsigned long long v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]

    v1 = a3;
    len1 = strlen(ptr);
    v6 = (unsigned long long)((15 + len1 + 9) / 16) * 16;
    while (&v0 != (char *)&v1 - (v6 & 0xfffffffffffff000))
    ;
    /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v6 & 0xfff)];
    if ((unsigned short)v6 & 0xfff)
        *((long long *)(((unsigned int)v6 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v6 & 0xfff) - 8 + /* unsupported instruction */));
    v3 = (/* unsupported instruction */ + 15 >> 4) * 16;
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    memcpy(v3, ptr, len1 + 1);
    v7 = (unsigned long long)((15 + v1 + 9) / 16) * 16;
    for (/* unsupported instruction */ = /* unsupported instruction */; /* unsupported instruction */ != /* unsupported instruction */ - (v7 & 0xfffffffffffff000); *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8)))
    {
        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
    }
    /* unsupported instruction */ = /* unsupported instruction */ - ((unsigned int)v7 & 0xfff);
    if ((unsigned short)v7 & 0xfff)
        *((long long *)(((unsigned int)v7 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v7 & 0xfff) - 8 + /* unsupported instruction */));
    v4 = (/* unsupported instruction */ + 15 >> 4) * 16;
    memcpy(v4, a2, v1 + 1);
    sub_41d6da(v3);
    sub_41d78e(a0, strlen("SCHILY.xattr.") + v3, v4, v1);
    return;
}



// Function: sparse_major_coder @ 0x206ce
typedef struct struct_0 {
    char padding_0[292];
    unsigned int field_124;
} struct_0;

void sparse_major_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41efed(a0->field_124, a1, a2);
    return;
}



// Function: sparse_major_decoder @ 0x2070c
typedef struct struct_0 {
    char padding_0[292];
    unsigned int field_124;
} struct_0;

void sparse_major_decoder(struct_0 *a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, 0xffffffff, a1))
        return;
    a0->field_124 = v1;
    return;
}



// Function: sparse_minor_coder @ 0x2077e
typedef struct struct_0 {
    char padding_0[296];
    unsigned int field_128;
} struct_0;

void sparse_minor_coder(struct_0 *a0, char *a1, void* a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a3;
    sub_41efed(a0->field_128, a1, a2);
    return;
}



// Function: sparse_minor_decoder @ 0x207bc
typedef struct struct_0 {
    char padding_0[296];
    unsigned int field_128;
} struct_0;

void sparse_minor_decoder(struct_0 *a0, unsigned long long a1, unsigned int a2, unsigned long long a3)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x18]

    v0 = a3;
    if (!(char)sub_41f138(&v1, a2, 0xffffffff, a1))
        return;
    a0->field_128 = v1;
    return;
}



// Function: dumpdir_create0 @ 0x2082e
typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    long long field_10;
    unsigned long long field_18;
} struct_0;

struct_0 * dumpdir_create0(char *a0, char *a1)
{
    long long iter;  // [bp-0x40], Other Possible Types: unsigned long long
    unsigned long long v1;  // [bp-0x38]
    unsigned long n;  // [bp-0x30]
    struct_0 *j;  // [bp-0x28]
    char *i;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]
    unsigned long v6;  // [bp-0x10]

    iter = 0;
    v1 = 0;
    n = 1;
    for (i = a0; *(i); i = &i[v6])
    {
        v6 = strlen(i) + 1;
        n += v6;
        if (!a1 || strchr(a1, *(i)))
            iter += 1;
        v1 += 1;
    }
    idx = sub_460b5b(n + 32);
    idx->field_0 = &idx->field_18;
    memcpy(idx->field_0, a0, n);
    *((unsigned long long *)&idx->field_1) = v1;
    idx->field_8 = iter;
    idx->field_10 = sub_460c29(iter + 1, 8, iter);
    iter = 0;
    for (j = idx->field_0; (char)j->field_0; j = (char *)j + strlen(j) + 1)
    {
        if (!a1 || strchr(a1, (char)j->field_0))
        {
            iter += 1;
            *((struct_0 **)(iter * 8 + idx->field_10)) = (char *)&j->field_0 + 1;
        }
    }
    *((unsigned long *)(iter * 8 + idx->field_10)) = 0;
    return idx;
}



// Function: dumpdir_create @ 0x209d4
typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    long long field_10;
    unsigned long long field_18;
} struct_0;

extern char g_481be0;

struct_0 * dumpdir_create(char *a0)
{
    return sub_42082e(a0, &g_481be0);
}



// Function: dumpdir_free @ 0x209fc
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



// Function: compare_dirnames @ 0x20a2b
int compare_dirnames(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: dumpdir_locate @ 0x20a6a
typedef struct struct_0 {
    char padding_0[16];
    char field_10;
    char padding_11[7];
    void* field_18;
} struct_0;

unsigned long long dumpdir_locate(struct_0 *a0, unsigned long a1)
{
    unsigned long v0;  // [bp-0x28]
    unsigned long long *v1;  // [bp-0x10]

    v0 = a1;
    if (a0)
    {
        v1 = bsearch(&v0, a0->field_18, *((long long *)&a0->field_10), 8, sub_420a2b);
        return (!v1 ? 0 : *(v1) - 1);
    }
    return 0;
}



// Function: dumpdir_next @ 0x20ad3
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
    unsigned long len;  // rax
    char *v0;  // [bp-0x28]
    unsigned long index;  // [bp-0x20]

    index = idx->field_10;
    v0 = NULL;
    if (idx->field_8)
    {
        v0 = index + idx->field_0->field_0;
        if (!*(v0))
            return NULL;
        len = strlen(v0);
        idx->field_10 = len + idx->field_10 + 1;
    }
    else if (index < idx->field_0->field_10)
    {
        v0 = *((long long *)(index * 8 + idx->field_0->field_18)) - 1;
        idx->field_10 = idx->field_10 + 1;
    }
    return v0;
}



// Function: dumpdir_first @ 0x20b9f
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
    struct_0 *idx;  // [bp-0x10]

    idx = sub_460b5b(24);
    idx->field_0 = a0;
    idx->field_8 = a1;
    idx->field_10 = 0;
    *(a2) = idx;
    return sub_420ad3(idx);
}



// Function: dumpdir_size @ 0x20bfe
unsigned long long dumpdir_size(char *a0)
{
    char *i;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10]

    i = a0;
    for (v1 = 0; *(i); i = &i[v2])
    {
        v2 = strlen(i) + 1;
        v1 += v2;
    }
    return v1 + 1;
}



// Function: hash_directory_canonical_name @ 0x20c51
typedef struct struct_0 {
    char padding_0[80];
    long long field_50;
} struct_0;

long long hash_directory_canonical_name(struct_0 *a0, long long a1)
{
    return sub_452834(a0->field_50, a1, a1);
}



// Function: compare_directory_canonical_names @ 0x20c86
typedef struct struct_0 {
    char padding_0[80];
    char *field_50;
} struct_0;

char compare_directory_canonical_names(struct_0 *a0, struct_0 *a1)
{
    return !strcmp(a0->field_50, a1->field_50);
}



// Function: hash_directory_meta @ 0x20ccc
long long hash_directory_meta(unsigned long long *a0, unsigned long long a1)
{
    return (a0[4] + a0[3]) % a1;
}



// Function: compare_directory_meta @ 0x20d05
unsigned int compare_directory_meta(unsigned long long *a0, unsigned long a1)
{
    unsigned int v1;  // eax

    if (a0[3] == *((long long *)(a1 + 24)) && a0[4] == *((long long *)(a1 + 32)))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: make_directory @ 0x20d60
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
    unsigned long len;  // [bp-0x18]
    struct_0 *idx;  // [bp-0x10]

    len = strlen(ptr);
    idx = sub_460b5b(96);
    idx->field_0 = 0;
    idx->field_30 = 0;
    idx->field_28 = idx->field_30;
    idx->field_40 = 0;
    idx->field_3c = 0;
    if (len > 1 && *((char *)ptr + len - 1) == 47)
        len -= 1;
    idx->field_58 = sub_460b5b(len + 1);
    memcpy(idx->field_58, ptr, len);
    *(len + (char *)idx->field_58) = 0;
    idx->field_50 = a1;
    idx->field_48 = 0;
    return idx;
}



// Function: free_directory @ 0x20e58
void free_directory(void* *a0)
{
    free(a0[10]);
    free(a0[11]);
    free(a0);
    return;
}



// Function: attach_directory @ 0x20e97
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

extern unsigned long long g_49f438;
extern struct_0 *g_49f440;
extern unsigned int g_49f600;

struct_0 * attach_directory(unsigned long a0)
{
    unsigned long long v0;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v0 = sub_429aac(g_49f600, a0, a0);
    v1 = sub_420d60(a0, v0);
    if (g_49f440)
        g_49f440->field_0 = v1;
    else
        g_49f438 = v1;
    g_49f440 = v1;
    return v1;
}



// Function: dirlist_replace_prefix @ 0x20f0e
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_49f438;

struct_0 ** dirlist_replace_prefix(unsigned long a0, unsigned long a1)
{
    struct_0 **v4;  // rax
    struct_0 **i;  // [bp-0x20]
    unsigned long len;  // [bp-0x18]
    unsigned long len1;  // [bp-0x10]

    len = strlen(a0);
    len1 = strlen(a1);
    v4 = g_49f438;
    for (i = g_49f438; i; i = *(i))
    {
        sub_429b93(i + 11, a0, len, a1, len1);
        v4 = *(i);
    }
    return v4;
}



// Function: clear_directory_table @ 0x20f88
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_49f438;
extern unsigned long long g_49f440;
extern long long g_49f448;
extern long long g_49f450;

long long clear_directory_table(void)
{
    struct_0 **i;  // [bp-0x18]
    struct_0 **v1;  // [bp-0x10]

    if (g_49f448)
        sub_452d91(g_49f448);
    if (g_49f450)
        sub_452d91(g_49f450);
    for (i = g_49f438; i; i = v1)
    {
        v1 = *(i);
        sub_420e58(i);
    }
    g_49f440 = 0;
    g_49f438 = g_49f440;
    return g_49f440;
}



// Function: note_directory @ 0x21019
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    struct struct_0 *field_28;
    char padding_30[8];
    unsigned int field_38;
    unsigned int field_3c;
} struct_0;

extern long long g_49f448;
extern long long g_49f450;

struct_0 * note_directory(unsigned int a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, char a5, char a6, char *a7)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_420e97(a0);
    idx->field_8 = a1;
    idx->field_10 = a2;
    idx->field_18 = a3;
    idx->field_20 = a4;
    idx->field_38 = 1;
    if (a5)
        idx->field_3c = idx->field_3c | 2;
    if (a6)
        idx->field_3c = idx->field_3c | 4;
    if (a7)
        idx->field_28 = sub_4209d4(a7);
    else
        idx->field_28 = NULL;
    if ((g_49f448 || (g_49f448 = (long long)(unsigned long long)sub_452c1c(0, 0, sub_420c51, sub_420c86, 0), g_49f448)) && sub_45395d(g_49f448, idx, idx))
    {
        if ((g_49f450 || (g_49f450 = (long long)(unsigned long long)sub_452c1c(0, 0, sub_420ccc, sub_420d05, 0), g_49f450)) && sub_45395d(g_49f450, idx, idx))
            return idx;
        sub_41783c(); /* do not return */
    }
    sub_41783c(); /* do not return */
}



// Function: find_directory @ 0x211c6
extern long long g_49f448;
extern unsigned int g_49f600;

unsigned long long find_directory(unsigned long a0)
{
    unsigned long long v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    if (g_49f448)
    {
        v0 = sub_429aac(g_49f600, a0, a0);
        v1 = sub_420d60(a0, v0);
        v2 = sub_452567(g_49f448, v1, v1);
        sub_420e58(v1);
        return v2;
    }
    return 0;
}



// Function: rebase_directory @ 0x21244
long long rebase_directory(long long a0, long long a1, long long a2, long long a3, long long a4)
{
    return sub_429b93(a0 + 88, a1, a2, a3, a4);
}



// Function: find_directory_meta @ 0x2128a
extern void g_481be4;
extern long long g_49f450;

unsigned long long find_directory_meta(unsigned long a0, unsigned long a1)
{
    unsigned long long *idx;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    if (g_49f450)
    {
        idx = &sub_420d60(&g_481be4, 0)->field_0;
        idx[3] = a0;
        idx[4] = a1;
        v1 = sub_452567(g_49f450, idx, idx);
        sub_420e58(idx);
        return v1;
    }
    return 0;
}



// Function: update_parent_directory @ 0x2130d
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
    unsigned long v4;  // rdx
    struct_1 *idx;  // [bp-0xc0]
    stat v1;  // [bp-0xb8]

    idx = sub_4211c6(*((long long *)&a0->field_0));
    if (!idx)
    {
        return;
    }
    else if (fstat(a0->field_1a8, &v1))
    {
        sub_42b444(idx->field_58);
        return;
    }
    else
    {
        idx->field_8 = sub_45fd9e(&v1);
        idx->field_10 = v4;
        return;
    }
}



// Function: procdir @ 0x213c6
typedef struct struct_0 {
    char padding_0[96];
    unsigned int field_60;
    char padding_64[308];
    void* field_198;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned int g_49ec84;
extern unsigned int g_49f87c;
extern unsigned long long g_49f8e0;
extern char g_49f8ec;
extern long long g_49f900;
extern long long g_49f908;
extern char g_49f935;
extern struct_2 *g_49fdc8;

void* procdir(unsigned long a0, struct_0 *a1, unsigned int a2, char *a3)
{
    unsigned long long v9;  // rax
    unsigned int v18;  // eax
    long long v10;  // rdx
    long long v13;  // rax
    long long v14;  // rax
    unsigned int v15;  // edx
    char v0;  // [bp-0x5a]
    char v1;  // [bp-0x59]
    long long v2;  // [bp-0x58]
    void* idx;  // [bp-0x50]
    struct_0 *v4;  // [bp-0x48]
    void* index;  // [bp-0x40]
    void* idx1;  // [bp-0x38]

    v4 = &a1->padding_0[88];
    v1 = *((long long *)&v4->padding_0[0]) >> 63;
    v0 = 0;
    idx = sub_4211c6(a0);
    if (idx)
    {
        if ((char)(int)idx[60] & 1)
        {
            if ((char)a2 & 32)
            {
                sub_429547(idx + 88, a0, idx + 88);
            }
            else
            {
                *(a3) = 78;
                return idx;
            }
        }
        if (strcmp((long long)idx[88], a0))
        {
            *(a3) = 78;
            return idx;
        }
        if (g_49f8ec && (!((char)(int)idx[60] & 2) || v1 != 1) && (long long)idx[24] != *((long long *)&v4->padding_0[0]) || (long long)idx[32] != *((long long *)&v4->padding_0[8]))
        {
            idx1 = sub_42128a(*((long long *)&v4->padding_0[0]), *((long long *)&v4->padding_0[8]));
            if (idx1)
            {
                if (strcmp((long long)idx1[88], a0))
                {
                    if ((unsigned short)g_49ec84 & 0x2000)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        sub_45e821(1, (long long)idx1[88]);
                        sub_45e548(a0);
                        error(0, 0, gettext("%s: Directory has been renamed from %s"));
                    }
                    *((void* *)&idx[64]) = idx1;
                    *((unsigned int *)&idx[60]) = (int)idx[60] | 16;
                    *((unsigned int *)&idx1[60]) = (int)idx1[60] & 0xffffffef;
                    sub_420f0e((long long)idx1[88], a0);
                }
                *((unsigned int *)&idx[56]) = 1;
            }
            else
            {
                v0 = 1;
                *((unsigned int *)&idx[56]) = 2;
                *((long long *)&idx[24]) = *((long long *)&v4->padding_0[0]);
                *((long long *)&idx[32]) = *((long long *)&v4->padding_0[8]);
            }
            if (v1)
                *((unsigned int *)&idx[60]) = (int)idx[60] | 2;
        }
        else
        {
            *((unsigned int *)&idx[56]) = 1;
        }
        *((unsigned int *)&idx[60]) = (int)idx[60] | 4;
    }
    else
    {
        index = sub_42128a(*((long long *)&v4->padding_0[0]), *((long long *)&v4->padding_0[8]));
        v9 = sub_45fd9e(v4);
        idx = sub_421019(a0, v9, v10, *((long long *)&v4->padding_0[0]), *((long long *)&v4->padding_0[8]), v1, 1, NULL);
        if (index)
        {
            if (strcmp((long long)index[88], a0))
            {
                if ((unsigned short)g_49ec84 & 0x2000)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_45e821(1, (long long)index[88]);
                    sub_45e548(a0);
                    error(0, 0, gettext("%s: Directory has been renamed from %s"));
                }
                *((void* *)&idx[64]) = index;
                *((unsigned int *)&idx[60]) = (int)idx[60] | 16;
                *((unsigned int *)&index[60]) = (int)index[60] & 0xffffffef;
                sub_420f0e((long long)index[88], a0);
            }
            *((unsigned int *)&idx[56]) = 1;
        }
        else
        {
            *((unsigned int *)&idx[60]) = (int)idx[60] | 8;
            if ((unsigned short)g_49ec84 & 0x1000)
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_45e548(a0);
                error(0, 0, gettext("%s: Directory is new"));
            }
            if (g_49f8e0 || (v13 = (long long)(unsigned long long)sub_45fd9e(v4), (unsigned int)(unsigned long long)sub_45ff90((unsigned long long)sub_45fd9e(v4), v10, g_49f900, g_49f908) < 0 || g_49f87c && !(v14 = (long long)(unsigned long long)sub_45fd84(v4), (unsigned int)(unsigned long long)sub_45ff90((unsigned long long)sub_45fd84(v4), v10, g_49f900, g_49f908) >= 0)))
                v15 = 2;
            else
                v15 = 1;
            *((unsigned int *)&idx[56]) = v15;
        }
    }
    if (g_49f935 && a1->field_198 && *((long long *)&v4->padding_0[0]) != (long long)a1->field_198[88])
    {
        if (g_49ec84 & 0xffffff & 0x40000)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548((long long)idx[88]);
            error(0, 0, gettext("%s: directory is on a different filesystem; not dumped"));
        }
        *((unsigned int *)&idx[56]) = 0;
        if ((long long)idx[40])
        {
            sub_4209fc((long long)idx[40]);
            *((struct_0 **)&idx[40]) = NULL;
        }
        v0 = 0;
    }
    else if ((char)a2 & 16)
    {
        *((unsigned int *)&idx[56]) = a2 & 3;
        if (!(int)idx[56])
            *(a3) = 78;
    }
    if (v0 && (unsigned short)g_49ec84 & 0x2000)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(a0);
        error(0, 0, gettext("%s: Directory has been renamed"));
    }
    *((unsigned int *)&idx[60]) = (int)idx[60] | 1;
    if (!(int)idx[56])
        return idx;
    v18 = sub_411fbf(a1, &v2);
    if (v18 == 3)
    {
        sub_411f18(a0, v2, gettext("directory not dumped"));
        *(a3) = 78;
        *((unsigned int *)&idx[56]) = 0;
    }
    else if (v18 <= 3)
    {
        if (v18 == 2)
        {
            sub_411f18(a0, v2, gettext("contents not dumped"));
            *((long long *)&idx[72]) = v2;
        }
        else if (v18 <= 2 && v18 && v18 == 1)
        {
            sub_411f18(a0, v2, gettext("contents not dumped"));
            *((unsigned int *)&idx[56]) = 0;
            *((long long *)&idx[72]) = v2;
        }
    }
    return idx;
}



// Function: makedumpdir @ 0x21b74
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

void makedumpdir(struct_0 *idx, char *a1)
{
    char *v10;  // rax
    char *v11;  // rax
    unsigned long iter1;  // [bp-0x50], Other Possible Types: unsigned long long
    unsigned long i;  // [bp-0x48]
    unsigned long long v2;  // [bp-0x40]
    char *node;  // [bp-0x38]
    char *iter;  // [bp-0x30]
    void* v5;  // [bp-0x28]
    void* ptr;  // [bp-0x20]
    char *v7;  // [bp-0x18]
    unsigned long long v8;  // [bp-0x10]

    if (idx->field_38 == 2)
    {
        v5 = NULL;
    }
    else if (idx->field_40)
    {
        v5 = (!idx->field_40->field_30 ? idx->field_40->field_28 : idx->field_40->field_30);
    }
    else
    {
        v5 = idx->field_28;
    }
    i = 0;
    v2 = 0;
    for (node = a1; *(node); i += 1)
    {
        v2 = strlen(node) + v2 + 2;
        node = &node[1 + strlen(node)];
    }
    v2 += 1;
    ptr = sub_460c29(i, 8);
    iter1 = 0;
    for (node = a1; *(node); iter1 += 1)
    {
        *((char **)(8 * iter1 + (char *)ptr)) = node;
        node = &node[1 + strlen(node)];
    }
    qsort(ptr, i, 8, sub_420a2b);
    v7 = sub_460b5b(v2);
    iter = v7;
    for (iter1 = 0; iter1 < i; iter1 += 1)
    {
        v8 = sub_420a6a(v5, *((long long *)((char *)ptr + 8 * iter1)));
        if (v8)
        {
            if (idx->field_48)
                *(iter) = 73;
            else
                *(iter) = 32;
            iter += 1;
        }
        else if (idx->field_48)
        {
            v10 = iter;
            iter = v10 + 1;
            *(v10) = 73;
        }
        else
        {
            v11 = iter;
            iter = v11 + 1;
            *(v11) = 89;
        }
        node = *((long long *)((char *)ptr + 8 * iter1));
        do
        {
            node += 1;
            iter += 1;
            *(iter) = *(node);
        } while (*(iter));
    }
    *(iter) = 0;
    idx->field_30 = idx->field_28;
    idx->field_28 = sub_42082e(v7, NULL);
    free(v7);
    free(ptr);
    return;
}



// Function: maketagdumpdir @ 0x21e0d
typedef struct struct_0 {
    struct struct_0 *field_0;
    char field_1;
    char padding_2[6];
    unsigned long long field_8;
    long long field_10;
    unsigned long long field_18;
} struct_0;

void maketagdumpdir(struct_0 *idx)
{
    unsigned long n;  // [bp-0x18]
    char *ptr;  // [bp-0x10]

    n = strlen(idx->field_48) + 1;
    ptr = sub_460b5b(n + 2);
    *(ptr) = 89;
    memcpy(ptr + 1, idx->field_48, n);
    ptr[1 + n] = 0;
    idx->field_30 = idx->field_28;
    idx->field_28 = sub_42082e(ptr, NULL);
    free(ptr);
    return;
}



// Function: scan_directory @ 0x21eb9
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

extern unsigned int g_49f87c;
extern long long g_49f900;
extern long long g_49f908;
extern unsigned int g_49f930;
extern char g_49f935;
extern unsigned int g_49fc08;
extern unsigned int g_49fc10;

struct_0 * scan_directory(struct_0 *idx)
{
    long long v21;  // rax
    long long v22;  // rdx
    long long v23;  // rax
    char v0;  // [bp-0x236]
    char v1;  // [bp-0x235]
    unsigned int v2;  // [bp-0x234]
    unsigned int v3;  // [bp-0x230]
    int v4;  // [bp-0x22c]
    void* v5;  // [bp-0x228]
    unsigned long i;  // [bp-0x220]
    long long v7;  // [bp-0x218]
    unsigned long v8;  // [bp-0x210]
    unsigned long ptr;  // [bp-0x208]
    unsigned long v10;  // [bp-0x200]
    unsigned long v11;  // [bp-0x1f8]
    struct_0 *v12;  // [bp-0x1f0]
    long long v13;  // [bp-0x1e8]
    unsigned long v14;  // [bp-0x1e0]
    char v15;  // [bp-0x1d8]
    int v16;  // [bp-0x180], Other Possible Types: stat
    struct_0 *v17;  // [bp-0x40]
    int v18;  // [bp-0x30]

    v8 = *((long long *)&idx->padding_0[0]);
    ptr = sub_414d3d(idx);
    v10 = *((long long *)&idx->padding_0[88]);
    v1 = !idx->field_198;
    if (!ptr)
        sub_43e72e(v8);
    sub_417909(idx);
    v11 = sub_460cc6(v8);
    sub_4298d7(v11);
    v12 = sub_4213c6(v11, idx, (!v1 ? 0 : 32), &v0);
    free(v11);
    v13 = sub_42b66c(v8);
    if (!ptr)
    {
        sub_42b72d(v13);
        free(ptr);
        return v12;
    }
    else if (*((int *)&v12->padding_0[56]))
    {
        sub_421b74(v12, ptr);
        for (i = sub_420b9f(*((long long *)&v12->padding_0[40]), 1, &v5); i; i = sub_420ad3(v5))
        {
            v14 = sub_42b75b(v13, i + 1, i + 1);
            if (*((char *)i) == 73)
            {
                *((char *)i) = 78;
            }
            else if ((char)sub_417c43(v14, idx))
            {
                *((char *)i) = 78;
            }
            else
            {
                v3 = idx->field_1a8;
                v7 = 0;
                sub_439c55(&v15);
                if (v3 < 0)
                {
                    *(__errno_location()) = -(v3);
                    v7 = sub_42b2b1;
                }
                else if (fstatat(v3, i + 1, &v16, g_49fc10))
                {
                    v7 = sub_42b444;
                }
                else if (((unsigned short)*((unsigned int *)(&v16 + 24)) & 0xf000) == 0x4000)
                {
                    v4 = sub_4157f9(idx, i + 1, g_49fc08);
                    if (v4 < 0)
                    {
                        v7 = sub_42b2b1;
                    }
                    else
                    {
                        v18 = v4;
                        if (fstat(v4, &v16))
                            v7 = sub_42b444;
                    }
                }
                if (v7)
                {
                    sub_42b48b(v14, 0, v7);
                    *((char *)i) = 78;
                }
                else if (((unsigned short)*((unsigned int *)(&v16 + 24)) & 0xf000) == 0x4000)
                {
                    v2 = 0;
                    if (!g_49f930)
                    {
                        v2 |= 16;
                    }
                    else
                    {
                        if (*((int *)&v12->padding_0[56]) == 2)
                            v2 |= 18;
                    }
                    *((char *)i) = 0x44;
                    v17 = idx;
                    sub_4213c6(v14, &v15, v2, i);
                    sub_41588e(&v15);
                }
                else if (g_49f935 && v10 != *((unsigned long long *)&v16))
                {
                    *((char *)i) = 78;
                }
                else if (*((char *)i) != 89)
                {
                    v21 = sub_45fd9e(&v16);
                    if (sub_45ff90(v21, v22, g_49f900, g_49f908) < 0 && (!g_49f87c || (v23 = (long long)(unsigned long long)sub_45fd84(&v16), (unsigned int)(unsigned long long)sub_45ff90((unsigned long long)sub_45fd84(&v16), v22, g_49f900, g_49f908) < 0)))
                        *((char *)i) = 78;
                    else
                        *((char *)i) = 89;
                }
                sub_439d1c(&v15);
            }
        }
        free(v5);
        sub_42b72d(v13);
        free(ptr);
        return v12;
    }
    else
    {
        if (*((long long *)&v12->padding_0[72]))
            sub_421e0d(v12);
        sub_42b72d(v13);
        free(ptr);
        return v12;
    }
}



// Function: directory_contents @ 0x2241b
typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long directory_contents(struct_0 *a0)
{
    return (!a0 ? 0 : (!a0->field_28 ? 0 : a0->field_28->field_0));
}



// Function: safe_directory_contents @ 0x22456
typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned long long g_481ca8;

unsigned long long safe_directory_contents(struct_0 *a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_42241b(a0);
    if (!v0)
        return &g_481ca8;
    return v0;
}



// Function: obstack_code_rename @ 0x2248c
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_49f878;

struct_0 * obstack_code_rename(struct_0 *a0, char *a1, char *a2)
{
    char *v13;  // rax
    char *v15;  // rax
    char *v0;  // [bp-0x78]
    char *ptr;  // [bp-0x60]
    struct_0 *index;  // [bp-0x58]
    struct_0 *v3;  // [bp-0x50]
    struct_0 *idx1;  // [bp-0x48]
    unsigned long n1;  // [bp-0x40]
    struct_0 *v6;  // [bp-0x38]
    struct_0 *idx2;  // [bp-0x30]
    struct_0 *v8;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x20]
    unsigned long n;  // [bp-0x18]
    struct_0 *v11;  // [bp-0x10]

    v0 = a1;
    if (*(v0))
        v0 = sub_43edd9(v0, 0, g_49f878);
    ptr = v0;
    index = a0;
    v3 = index;
    if (!(v3->field_20 - (char *)v3->field_18))
        _obstack_newchunk(index, 1);
    v13 = &index->field_18->field_0;
    index->field_18 = v13 + 1;
    *(v13) = 82;
    idx1 = a0;
    n1 = strlen(ptr) + 1;
    v6 = idx1;
    if (v6->field_20 - (char *)v6->field_18 < n1)
        _obstack_newchunk(idx1, n1, n1);
    memcpy(idx1->field_18, ptr, n1);
    idx1->field_18 = &idx1->field_18[n1];
    ptr = (!*(a2) ? a2 : sub_43edd9(a2, 0, g_49f878));
    idx2 = a0;
    v8 = idx2;
    if (!(v8->field_20 - (char *)v8->field_18))
        _obstack_newchunk(idx2, 1);
    v15 = &idx2->field_18->field_0;
    idx2->field_18 = v15 + 1;
    *(v15) = 84;
    idx = a0;
    n = strlen(ptr) + 1;
    v11 = idx;
    if (v11->field_20 - (char *)v11->field_18 < n)
        _obstack_newchunk(idx, n, n);
    memcpy(idx->field_18, ptr, n);
    idx->field_18 = &idx->field_18[n];
    return idx;
}



// Function: store_rename @ 0x226b7
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_481be4;

void* store_rename(void* a0, struct_0 *a1)
{
    void* v9;  // rax
    char *v10;  // rax
    void* node;  // [bp-0x48]
    void* iter;  // [bp-0x40]
    void* v2;  // [bp-0x38]
    struct_0 *idx;  // [bp-0x30]
    struct_0 *v4;  // [bp-0x28]
    struct_0 *index;  // [bp-0x20]
    unsigned long n;  // [bp-0x18]
    struct_0 *v7;  // [bp-0x10]

    for (node = a0; node && a0 != (long long)node[64]; node = (long long)node[64]);
    if (!node)
    {
        v9 = a0;
        for (iter = v9; iter; iter = (long long)iter[64])
        {
            if (!(long long)iter[64])
                return (long long)iter[64];
            sub_42248c(a1, *((long long *)((long long)iter[64] + 88)), (long long)iter[88]);
            v9 = (long long)iter[64];
        }
        return v9;
    }
    else
    {
        v2 = sub_44e10c((long long)a0[88]);
        idx = a1;
        v4 = idx;
        if (!(v4->field_20 - (char *)v4->field_18))
            _obstack_newchunk(idx, 1);
        v10 = &idx->field_18->field_0;
        idx->field_18 = v10 + 1;
        *(v10) = 88;
        index = a1;
        n = strlen(v2) + 1;
        v7 = index;
        if (v7->field_20 - (char *)v7->field_18 < n)
            _obstack_newchunk(index, n, n);
        memcpy(index->field_18, v2, n);
        index->field_18 = &index->field_18[n];
        sub_42248c(a1, (long long)a0[88], &g_481be4);
        for (iter = a0; iter != node; iter = (long long)iter[64])
        {
            sub_42248c(a1, *((long long *)((long long)iter[64] + 88)), (long long)iter[88]);
        }
        sub_42248c(a1, &g_481be4, (long long)node[88]);
        return (unsigned long long)free(v2);
    }
}



// Function: append_incremental_renames @ 0x228cd
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[52];
    unsigned int field_3c;
} struct_0;

extern struct_0 *g_49f438;

void append_incremental_renames(struct_0 *a0)
{
    char *v21;  // rax
    unsigned long v0;  // [bp-0xd0]
    struct_0 *i;  // [bp-0xc8]
    unsigned long v2;  // [bp-0xc0]
    char *idx2;  // [bp-0xb8]
    unsigned long n;  // [bp-0xb0]
    char *v5;  // [bp-0xa8]
    char *v6;  // [bp-0xa0]
    char *idx;  // [bp-0x98]
    char *v8;  // [bp-0x90]
    char *idx1;  // [bp-0x88]
    char *v10;  // [bp-0x80]
    char *index;  // [bp-0x78]
    unsigned long long v12;  // [bp-0x70]
    struct_0 v13;  // [bp-0x68]
    unsigned long long v14;  // [bp-0x60]
    char *v15;  // [bp-0x58], Other Possible Types: unsigned long long
    void* v16;  // [bp-0x50], Other Possible Types: char *, unsigned long long
    unsigned long long v17;  // [bp-0x48]
    unsigned long v18;  // [bp-0x38]
    char v19;  // [bp-0x18]

    if (!g_49f438)
        return;
    _obstack_begin(&v13, 0, 0, sub_460b5b, free);
    v2 = sub_42241b(a0);
    if (v2)
    {
        v0 = sub_420bfe(v2) - 1;
        idx2 = &v13;
        n = v0;
        v5 = idx2;
        if (v17 - v16 < n)
            _obstack_newchunk(idx2, n, n);
        memcpy(v16, v2, n);
        v16 += n;
    }
    else
    {
        v0 = 0;
    }
    for (i = g_49f438; i; i = i->field_0)
    {
        if ((char)i->field_3c & 16)
            sub_4226b7(i, &v13);
    }
    if (a0)
    {
        v6 = &v13;
        if (v16 - v15 != v0)
        {
            idx = &v13;
            v8 = idx;
            if (!(v17 - v16))
                _obstack_newchunk(idx, 1);
            v21 = v16;
            v16 = v21 + 1;
            *(v21) = 0;
            sub_4209fc(*((long long *)&a0->padding_8[32]));
            idx1 = &v13;
            v10 = v15;
            if (v10 == v16)
                v19 |= 2;
            v16 = ~(v18) & &v16[v18];
            if (v16 - v14 > v17 - v14)
                v16 = v17;
            v15 = v16;
            *((struct_0 **)&a0->padding_8[32]) = sub_4209d4(v10);
        }
    }
    index = &v13;
    v12 = 0;
    if (v12 > v14 && v12 < v17)
    {
        v15 = v12;
        v16 = v15;
        return;
    }
    _obstack_free(index, v12, v12);
    return;
}



// Function: read_incr_db_01 @ 0x22c72
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_49f458;
extern long long g_49f8e0;
extern long long g_49f900;
extern long long g_49f908;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void read_incr_db_01(unsigned int a0, char *a1)
{
    long long v14;  // rdx
    int *err;  // rax
    int *err1;  // rax
    int *err2;  // rax
    int *v18;  // rax
    int *v19;  // rax
    int *v20;  // rax
    char v0;  // [bp-0x85]
    unsigned int v1;  // [bp-0x84]
    char *str;  // [bp-0x80]
    unsigned long v3;  // [bp-0x78]
    char *v4;  // [bp-0x70]
    unsigned long long iter;  // [bp-0x68]
    char *v6;  // [bp-0x60]
    char *v7;  // [bp-0x58]
    long long v8;  // [bp-0x50], Other Possible Types: unsigned long long
    unsigned long long v9;  // [bp-0x48]
    unsigned long long v10;  // [bp-0x40]
    long long v11;  // [bp-0x38]
    long long v12;  // [bp-0x30]

    str = NULL;
    v3 = 0;
    iter = 1;
    if (a0 == 1)
    {
        if (getline(&str, &v3, g_49f458) <= 0)
        {
            sub_43e403(g_49f8e0);
            free(str);
            return;
        }
        iter += 1;
    }
    else
    {
        str = strdup(a1);
        v3 = strlen(str) + 1;
    }
    g_49f900 = sub_429ff7(str, &v4, 0, &v4);
    g_49f908 = v14;
    if ((char)sub_4294f0(g_49f900, g_49f908, g_49f900) != 1)
    {
        if (g_49fdc8)
            g_49fdc8();
        gettext("Invalid time stamp");
        sub_45e548(g_49f8e0);
        err = __errno_location();
        error(0, *(err), "%s:%ld: %s");
        sub_4177f4(); /* do not return */
    }
    if (a0 == 1 && *(v4))
    {
        v7 = v4 + 1;
        *(__errno_location()) = 0;
        v8 = strtoumax(v7, &v4, 10, &v4);
        if (!*(__errno_location()) && v8 > 0x3b9ac9ff)
            *(__errno_location()) = 0x22;
        if (*(__errno_location()) || v7 == v4)
        {
            if (g_49fdc8)
                g_49fdc8();
            gettext("Invalid time stamp");
            sub_45e548(g_49f8e0);
            err1 = __errno_location();
            error(0, *(err1), "%s:%ld: %s");
            g_49fdd0 = 2;
            g_49f900 = 0x8000000000000000;
            g_49f908 = 0xffffffffffffffff;
        }
        else
        {
            g_49f908 = v8;
        }
    }
    while (1)
    {
        v1 = getline(&str, &v3, g_49f458);
        if (v1 <= 0)
        {
            free(str);
            return;
        }
        v0 = *(str) == 43;
        v6 = &str[v0];
        iter += 1;
        if (*(&str[v1] - 1) == 10)
            *(&str[v1] - 1) = 0;
        if (a0 == 1)
        {
            v11 = sub_429ff7(v6, &v4, 0, &v4);
            v12 = v14;
            v6 = v4;
            if ((char)sub_4294f0(v11, v12, v11) != 1 || *(v6) != 32)
            {
                if (g_49fdc8)
                    g_49fdc8();
                gettext("Invalid modification time");
                sub_45e548(g_49f8e0);
                err2 = __errno_location();
                error(0, *(err2), "%s:%ld: %s");
                sub_4177f4(); /* do not return */
            }
            *(__errno_location()) = 0;
            v8 = strtoumax(v6, &v4, 10, &v4);
            if (!*(__errno_location()) && v8 > 0x3b9ac9ff)
                *(__errno_location()) = 0x22;
            if (*(__errno_location()) || v6 == v4 || *(v4) != 32)
            {
                if (g_49fdc8)
                    g_49fdc8();
                gettext("Invalid modification time (nanoseconds)");
                sub_45e548(g_49f8e0);
                v18 = __errno_location();
                error(0, *(v18), "%s:%ld: %s");
                sub_4177f4(); /* do not return */
            }
            v12 = v8;
            v6 = v4;
        }
        else
        {
            v12 = 0;
            v11 = v12;
        }
        v9 = sub_429cfc(v6, &v4, 0, -0x1);
        v6 = v4;
        if (*(__errno_location()) || *(v6) != 32)
        {
            if (g_49fdc8)
                g_49fdc8();
            gettext("Invalid device number");
            sub_45e548(g_49f8e0);
            v19 = __errno_location();
            error(0, *(v19), "%s:%ld: %s");
            sub_4177f4(); /* do not return */
        }
        v10 = sub_429cfc(v6, &v4, 0, -0x1);
        v6 = v4;
        if (*(__errno_location()) || *(v6) != 32)
        {
            if (g_49fdc8)
                g_49fdc8();
            gettext("Invalid inode number");
            sub_45e548(g_49f8e0);
            v20 = __errno_location();
            error(0, *(v20), "%s:%ld: %s");
            sub_4177f4(); /* do not return */
        }
        v6 += 1;
        sub_42961d(v6);
        sub_421019((unsigned int)v6, v11, v12, v9, v10, v0, 0, NULL);
    }
}



// Function: read_obstack @ 0x232e4
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

int read_obstack(FILE *a0, struct_0 *a1, unsigned long long *a2)
{
    char *v7;  // rax
    char *v8;  // rax
    unsigned int v0;  // [bp-0x34]
    unsigned long long i;  // [bp-0x30]
    struct_0 *index;  // [bp-0x28]
    struct_0 *v3;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]
    struct_0 *v5;  // [bp-0x10]

    i = 0;
    for (v0 = getc_unlocked(a0); v0 != 0xffffffff && v0; i += 1)
    {
        index = a1;
        v3 = index;
        if (!(v3->field_20 - (char *)v3->field_18))
            _obstack_newchunk(index, 1);
        v7 = &index->field_18->field_0;
        index->field_18 = v7 + 1;
        *(v7) = v0;
        v0 = getc_unlocked(a0);
    }
    idx = a1;
    v5 = idx;
    if (!(v5->field_20 - (char *)v5->field_18))
        _obstack_newchunk(idx, 1);
    v8 = &idx->field_18->field_0;
    idx->field_18 = v8 + 1;
    *(v8) = 0;
    *(a2) = i;
    return v0;
}



// Function: read_num @ 0x233ed
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern long long g_49f8e0;
extern struct_0 *g_49fdc8;

unsigned long long read_num(FILE *a0, unsigned long long a1, long long a2, long long a3, unsigned long long *a4)
{
    long long v14;  // rax
    long long v16;  // rax
    unsigned long long v18;  // rax
    long long v19;  // rax
    long long v21;  // rax
    unsigned long long v0;  // [bp-0x118]
    unsigned long long v1;  // [bp-0x110]
    char *v2;  // [bp-0x108], Other Possible Types: unsigned long
    char *v3;  // [bp-0x100], Other Possible Types: unsigned long
    char v4;  // [bp-0xb9]
    unsigned int i;  // [bp-0xb8]
    unsigned int v6;  // [bp-0xb4]
    unsigned int v7;  // [bp-0xb0]
    unsigned int v8;  // [bp-0xac]
    unsigned short v9;  // [bp-0xa8]
    char v10;  // [bp-0x88]
    char v11;  // [bp-0x68]
    char v12;  // [bp-0x48]

    v6 = getc_unlocked(a0);
    v4 = v6 == 45;
    for (i = 0; !i && v4 || v6 - 48 <= 9; i += 1)
    {
        *((char *)&v9 + i) = v6;
        if (i == 20)
        {
            if (g_49fdc8)
                g_49fdc8();
            v14 = ftello(a0);
            sub_454d1c(v14, &v10, v14);
            sub_45e548(g_49f8e0);
            v3 = &v9;
            v2 = i + 1;
            error(0, 0, gettext("%s: byte %s: %s %.*s... too long"));
            sub_4177f4(); /* do not return */
        }
        v6 = getc_unlocked(a0);
    }
    *((char *)&v9 + i) = 0;
    if (v6 < 0)
    {
        if (ferror_unlocked(a0))
        {
            sub_43e5d6(g_49f8e0); /* do not return */
        }
        else if (i)
        {
            if (g_49fdc8)
                g_49fdc8();
            gettext("Unexpected EOF in snapshot file");
            sub_45e548(g_49f8e0);
            error(0, 0, "%s: %s");
            sub_4177f4(); /* do not return */
        }
        else
        {
            return 0;
        }
    }
    else
    {
        if (v6)
        {
            v8 = v6;
            if (g_49fdc8)
                g_49fdc8();
            v16 = ftello(a0);
            sub_454d1c(v16, &v10, v16);
            sub_45e548(g_49f8e0);
            v3 = v8;
            v2 = &v9;
            error(0, 0, gettext("%s: byte %s: %s %s followed by invalid byte 0x%02x"));
            sub_4177f4(); /* do not return */
        }
        else
        {
            *(a4) = sub_429cfc(&v9, 0, a2, a3);
            v7 = *(__errno_location());
            switch (v7)
            {
            case 0:
                return 1;
            case 34:
                if (g_49fdc8)
                    g_49fdc8();
                v18 = sub_454e5a(a3, &v12, &v12);
                sub_454bde(a2, &v11, &v11);
                v19 = ftello(a0);
                sub_454d1c(v19, &v10, v19);
                sub_45e548(g_49f8e0);
                v2 = &v9;
                v1 = a1;
                v0 = v18;
                error(0, v7, gettext("%s: byte %s: (valid range %s..%s)\n\t%s %s"));
                sub_4177f4(); /* do not return */
            default:
                if (g_49fdc8)
                    g_49fdc8();
                v21 = ftello(a0);
                sub_454d1c(v21, &v10, v21);
                sub_45e548(g_49f8e0);
                v2 = &v9;
                error(0, v7, gettext("%s: byte %s: %s %s"));
                sub_4177f4(); /* do not return */
            }
        }
    }
}



// Function: read_timespec @ 0x238b6
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern long long g_49f8e0;
extern struct_0 *g_49fdc8;

long long read_timespec(FILE *a0, unsigned long long *a1)
{
    unsigned long v4;  // fs
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]

    v2 = *((long long *)(40 + v4));
    if ((char)sub_4233ed(a0, "sec", -0x8000000000000000, 0x7fffffffffffffff, &v0) && (char)sub_4233ed(a0, "nsec", 0, 0x3b9ac9ff, &v1))
    {
        *(a1) = v0;
        a1[1] = v1;
        if (v2 == *((long long *)(40 + v4)))
            return v2 - *((long long *)(40 + v4));
        __stack_chk_fail(); /* do not return */
    }
    if (g_49fdc8)
        g_49fdc8();
    gettext("Unexpected EOF in snapshot file");
    sub_45e548(g_49f8e0);
    error(0, 0, "%s: %s");
    sub_4177f4(); /* do not return */
}



// Function: read_incr_db_2 @ 0x239ca
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct FILE {
} FILE;

extern FILE *g_49f458;
extern long long g_49f8e0;
extern unsigned long long g_49f900;
extern struct_0 *g_49fdc8;

void read_incr_db_2(void)
{
    long long v24;  // rax
    char v0;  // [bp-0x119]
    unsigned long long v1;  // [bp-0x118]
    unsigned long long v2;  // [bp-0x110]
    unsigned long long v3;  // [bp-0x108]
    unsigned long long v4;  // [bp-0x100]
    char *idx1;  // [bp-0xf8]
    unsigned long long v6;  // [bp-0xf0]
    unsigned long long v7;  // [bp-0xe8]
    char *index;  // [bp-0xe0]
    unsigned long long v9;  // [bp-0xd8]
    unsigned long long v10;  // [bp-0xd0]
    char *idx;  // [bp-0xc8]
    unsigned long long v12;  // [bp-0xc0]
    unsigned long long v13;  // [bp-0xb8]
    unsigned int v14;  // [bp-0xb0]
    struct_0 v15;  // [bp-0xa8]
    unsigned long long v16;  // [bp-0xa0]
    unsigned long long v17;  // [bp-0x98]
    unsigned long long v18;  // [bp-0x90]
    unsigned long long v19;  // [bp-0x88]
    unsigned long v20;  // [bp-0x78]
    char v21;  // [bp-0x58]
    char v22;  // [bp-0x48]

    _obstack_begin(&v15, 0, 0, sub_460b5b, free);
    sub_4238b6(g_49f458, &g_49f900);
    while ((char)sub_4233ed(g_49f458, "nfs", 0, 1, &v1) == 1)
    {
        v0 = v1;
        sub_4238b6(g_49f458, &v13);
        if ((char)sub_4233ed(g_49f458, "dev", 0, -0x1, &v1) == 1 && (v3 = v1, (char)sub_4233ed(g_49f458, "ino", 0, -0x1, &v1) == 1 && (v4 = v1, !(int)(long long)sub_4232e4(g_49f458, &v15, &v2))))
        {
            idx1 = &v15.padding_0[0];
            v6 = v17;
            if (v6 == v18)
                v21 |= 2;
            v18 = ~(v20) & v18 + v20;
            if (v18 - v16 > v19 - v16)
                v18 = v19;
            v17 = v18;
            v7 = v6;
            do
            { } while (!sub_4232e4(g_49f458, &v15, &v2) && v2 > 1);
            if (getc_unlocked(g_49f458))
            {
                if (g_49fdc8)
                    g_49fdc8();
                gettext("Missing record terminator");
                v24 = ftello(g_49f458);
                sub_454d1c(v24, &v22, v24);
                sub_45e548(g_49f8e0);
                error(0, 0, gettext("%s: byte %s: %s"));
                sub_4177f4(); /* do not return */
            }
            index = &v15.padding_0[0];
            v9 = v17;
            if (v9 == v18)
                v21 |= 2;
            v18 = ~(v20) & v18 + v20;
            if (v18 - v16 > v19 - v16)
                v18 = v19;
            v17 = v18;
            v10 = v9;
            sub_421019(v7, v13, v14, v3, v4, v0, 0, v10);
            idx = &v15.padding_0[0];
            v12 = v10;
            if (v12 > v16 && v12 < v19)
            {
                v17 = v12;
                v18 = v17;
            }
            else
            {
                _obstack_free(idx, v12, v12);
            }
        }
        else
        {
            if (g_49fdc8)
                g_49fdc8();
            gettext("Unexpected EOF in snapshot file");
            sub_45e548(g_49f8e0);
            error(0, 0, "%s: %s");
            sub_4177f4(); /* do not return */
        }
    }
    return;
}



// Function: show_snapshot_field_ranges @ 0x23f6b
typedef struct struct_0 {
    char *field_0;
    long long field_8;
    long long field_10;
} struct_0;

extern struct_0 g_49a640;

void show_snapshot_field_ranges(void)
{
    char *v4;  // rax
    struct_0 *i;  // [bp-0x60]
    char v1;  // [bp-0x58]
    char v2;  // [bp-0x38]

    printf("This tar's snapshot file field ranges are\n");
    printf("   (%-15s => [ %s, %s ]):\n\n", "field name", "min");
    for (i = &g_49a640.field_0; i->field_0; i += 1)
    {
        sub_429c96(i->field_10, i->field_8, i->field_10, &v2);
        v4 = sub_429c96(i->field_8, i->field_8, i->field_10, &v1);
        printf("    %-15s => [ %s, %s ],\n", i->field_0, v4);
    }
    printf("\n");
    return;
}



// Function: read_directory_file @ 0x24090
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern FILE *g_49f458;
extern void g_49f8e0;
extern unsigned int g_49f8e8;
extern struct_0 *g_49fdc8;

void read_directory_file(void)
{
    long long v8;  // rdi
    long long v9;  // rsi
    long long v10;  // rdx
    long long v11;  // rcx
    long long v12;  // r8
    long long v13;  // r9
    unsigned int v0;  // [bp-0x38]
    unsigned int v1;  // [bp-0x34]
    char *v2;  // [bp-0x30]
    unsigned long v3;  // [bp-0x28]
    char *v4;  // [bp-0x20]
    char *v5;  // [bp-0x20]
    unsigned long long v6;  // [bp-0x18]

    v2 = NULL;
    v3 = 0;
    v0 = 66;
    if (!g_49f8e8)
        v0 |= 0x200;
    v1 = open(*((long long *)&g_49f8e0), v0, 438);
    if (v1 < 0)
    {
        sub_43e38b(*((long long *)&g_49f8e0));
        return;
    }
    g_49f458 = fdopen(v1, "r+");
    if (!g_49f458)
    {
        sub_43e38b(*((long long *)&g_49f8e0));
        close(v1);
        return;
    }
    sub_42eda0();
    sub_42ee43(v8, v9, v10, v11, v12, v13);
    if (getline(&v2, &v3, g_49f458) > NULL)
    {
        if (!strncmp(v2, "GNU tar", 7))
        {
            v4 = v2 + 7;
            v5 = v4 + 1;
            if (*(v4) != 45)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(1, 0, gettext("Bad incremental file format"));
            }
            else
            {
                while (*(v5) != 45)
                {
                    if (!*(v5))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(1, 0, gettext("Bad incremental file format"));
                    }
                    else
                    {
                        v5 += 1;
                    }
                }
                v6 = strtoumax(v5 + 1, 0, 10);
            }
        }
        else
        {
            v6 = 0;
        }
        if (v6 <= 1)
        {
            sub_422c72(v6, v2);
        }
        else if (v6 == 2)
        {
            sub_4239ca();
        }
        else
        {
            if (g_49fdc8)
                g_49fdc8();
            error(1, 0, gettext("Unsupported incremental format version: %lu"));
        }
    }
    if (ferror_unlocked(g_49f458))
        sub_43e403(*((long long *)&g_49f8e0));
    free(v2);
    return;
}



// Function: write_directory_file_entry @ 0x24350
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

extern void g_481f30;

char write_directory_file_entry(void* a0, FILE *a1)
{
    unsigned long v9;  // rax
    struct_0 *v0;  // [bp-0x50]
    void* i;  // [bp-0x48]
    void* ptr;  // [bp-0x30]
    char v3;  // [bp-0x28]

    if (!((char)(int)a0[60] & 4))
        return !ferror_unlocked(a1);
    ptr = (!((int)a0[60] & 2) ? "0" : "1");
    fwrite_unlocked(ptr, 2, 1, a1);
    ptr = sub_429c96((long long)a0[8], -0x8000000000000000, 0x7fffffffffffffff, &v3);
    fwrite_unlocked(ptr, strlen(ptr) + 1, 1, a1);
    ptr = sub_454bde((long long)a0[16], &v3, &v3);
    fwrite_unlocked(ptr, strlen(ptr) + 1, 1, a1);
    ptr = sub_429c96((long long)a0[24], 0, -0x1, &v3);
    fwrite_unlocked(ptr, strlen(ptr) + 1, 1, a1);
    ptr = sub_429c96((long long)a0[32], 0, -0x1, &v3);
    fwrite_unlocked(ptr, strlen(ptr) + 1, 1, a1);
    v9 = strlen((long long)a0[88]);
    fwrite_unlocked((long long)a0[88], v9 + 1, 1, a1);
    if ((long long)a0[40])
    {
        for (i = sub_420b9f((long long)a0[40], 0, &v0); i; i = sub_420ad3(v0))
        {
            fwrite_unlocked(i, strlen(i) + 1, 1, a1);
        }
        free(v0);
    }
    fwrite_unlocked(&g_481f30, 2, 1, a1);
    return !ferror_unlocked(a1);
}



// Function: write_directory_file @ 0x24600
extern long long g_49f448;
extern unsigned long long g_49f458;
extern long long g_49f8e0;
extern long long g_49f9e0;
extern long long g_49f9e8;

void write_directory_file(void)
{
    unsigned long long v0;  // [bp-0x38]
    void* ptr;  // [bp-0x30]
    char v2;  // [bp-0x28]

    v0 = g_49f458;
    if (!v0)
        return;
    if (fseeko(v0, 0, 0))
        sub_43e780(g_49f8e0);
    if (sub_432f52(fileno(v0) & 0xffffffff))
        sub_43ea55(g_49f8e0);
    fprintf(v0, "%s-%s-%d\n", "GNU tar", "1.34", 2);
    ptr = sub_454bde(g_49f9e0, &v2, &v2);
    fwrite_unlocked(ptr, strlen(ptr) + 1, 1, v0);
    ptr = sub_454e5a(g_49f9e8, &v2, g_49f9e8);
    fwrite_unlocked(ptr, strlen(ptr) + 1, 1, v0);
    if (!ferror_unlocked(v0) && g_49f448)
        sub_45279c(g_49f448, sub_424350, v0, sub_424350);
    if (ferror_unlocked(v0))
        sub_43eb22(g_49f8e0);
    if (!fclose(v0))
        return;
    sub_43e1ff(g_49f8e0);
    return;
}



// Function: get_gnu_dumpdir @ 0x247c6
typedef struct struct_0 {
    long long field_0;
    char padding_8[128];
    unsigned int field_88;
} struct_0;

extern unsigned int g_49f460;
extern struct_0 *g_49fdc8;

struct_0 * get_gnu_dumpdir(struct_0 *idx)
{
    unsigned long v0;  // [bp-0x30]
    unsigned long n;  // [bp-0x28]
    void* iter;  // [bp-0x20]
    void* v3;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    v0 = *((long long *)&idx->field_88);
    v3 = sub_460b5b(v0);
    iter = v3;
    sub_40bba7(*((long long *)&g_49f460));
    sub_40ad2b(idx);
    while (1)
    {
        if (!v0)
        {
            sub_40ad66();
            *((void* *)&idx[2].padding_8[112]) = v3;
            idx[2].padding_8[105] = 1;
            return idx;
        }
        sub_40ad86(v0);
        v4 = sub_40bb4b();
        if (!v4)
            break;
        n = sub_40bbf5(v4);
        if (n > v0)
            n = v0;
        memcpy(iter, v4, n);
        iter += n;
        sub_40bba7(v4 + n - 1);
        v0 -= n;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(1, 0, gettext("Unexpected EOF in archive"));
}



// Function: is_dumpdir @ 0x2490b
typedef struct struct_0 {
    long long field_0;
    char padding_8[128];
    unsigned int field_88;
} struct_0;

int is_dumpdir(struct_0 *a0)
{
    if (a0[2].padding_8[104] && !*((long long *)&a0[2].padding_8[112]))
        sub_4247c6(a0);
    return a0[2].padding_8[104];
}



// Function: dumpdir_ok @ 0x24953
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int dumpdir_ok(char *a0)
{
    char v0;  // [bp-0x29]
    unsigned int v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x24]
    char *iter;  // [bp-0x20]

    v1 = 0;
    v2 = 0;
    for (iter = a0; *(iter); iter = &iter[1 + strlen(iter)])
    {
        if (v2 && v2 != *(iter))
        {
            v0 = *(iter);
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Malformed dumpdir: expected '%c' but found %#3o"));
            g_49fdd0 = 2;
            return 0;
        }
        switch (*(iter))
        {
        case 82:
            if (!iter[1])
            {
                if (!v1)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Malformed dumpdir: empty name in 'R'"));
                    g_49fdd0 = 2;
                    return 0;
                }
                v1 = 0;
            }
            v2 = 84;
            break;
        case 84:
            if (v2 != 84)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Malformed dumpdir: 'T' not preceded by 'R'"));
                g_49fdd0 = 2;
                return 0;
            }
            if (!iter[1] && !v1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Malformed dumpdir: empty name in 'T'"));
                g_49fdd0 = 2;
                return 0;
            }
            v2 = 0;
            break;
        case 88:
            if (v1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Malformed dumpdir: 'X' duplicated"));
                g_49fdd0 = 2;
                return 0;
            }
            v1 = 1;
            break;
        default:
            break;
        }
    }
    if (v2)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Malformed dumpdir: expected '%c' but found end of data"));
        g_49fdd0 = 2;
        return 0;
    }
    else if (v1 && (char)g_49ec84 & 2)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Malformed dumpdir: 'X' never used"));
    }
    return 1;
}



// Function: try_purge_directory @ 0x24ca0
typedef struct struct_0 {
    long long field_0;
    char padding_8[128];
    unsigned int field_88;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_49efd0;
extern char g_49f878;
extern char g_49f8c8;
extern char g_49f935;
extern unsigned int g_49f9ac;
extern struct_0 g_49fa20;
extern unsigned long long g_49fbb0;
extern char g_49fc20;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int try_purge_directory(long long a0)
{
    char *v14;  // rax
    int *err;  // rax
    char *v18;  // rax
    unsigned int v0;  // [bp-0x10c]
    char *i;  // [bp-0x108]
    unsigned long iter;  // [bp-0x100]
    unsigned long ptr2;  // [bp-0xf8]
    unsigned long long ptr;  // [bp-0xf0]
    unsigned long long v5;  // [bp-0xe8]
    unsigned long long v6;  // [bp-0xe0]
    char *ptr1;  // [bp-0xd8]
    struct_0 *v8;  // [bp-0xd0]
    char *v9;  // [bp-0xc8]
    unsigned long len;  // [bp-0xc0]
    char v11;  // [bp-0xb8]
    unsigned int v12;  // [bp-0xa0]

    ptr = 0;
    if ((char)sub_42490b(&g_49fa20.field_0) != 1)
        return 0;
    ptr1 = sub_42b8e1(a0, 0);
    if (!ptr1)
    {
        return 0;
    }
    else if ((char)sub_424953(g_49fbb0) != 1)
    {
        return 0;
    }
    else
    {
        iter = g_49fbb0;
        while (1)
        {
            if (!*((char *)iter))
            {
                free(ptr);
                v8 = sub_4209d4(g_49fbb0);
                ptr2 = 0;
                for (i = ptr1; *(i); i = &i[1 + strlen(i)])
                {
                    free(ptr2);
                    ptr2 = sub_42ee85(a0, i, i);
                    if (!sub_42a9b8(ptr2, &v11, &v11))
                    {
                        v9 = sub_420a6a(v8, i);
                        if (!v9 || *(v9) == 0x44 && ((unsigned short)v12 & 0xf000) != 0x4000 || *(v9) == 89 && ((unsigned short)v12 & 0xf000) == 0x4000)
                        {
                            if (g_49f935 && v11 != *((long long *)&g_49fc20))
                            {
                                if (g_49fdc8)
                                    g_49fdc8();
                                sub_45e548(ptr2);
                                error(0, 0, gettext("%s: directory is on a different device: not purging"));
                            }
                            else
                            {
                                if (g_49f8c8 != 1 || sub_434af4("delete", ptr2))
                                {
                                    if (g_49f9ac)
                                    {
                                        sub_45e84e(ptr2);
                                        v18 = gettext("%s: Deleting %s\n");
                                        fprintf(g_49efd0, v18);
                                    }
                                    if (!sub_42a36f(ptr2, 1))
                                    {
                                        v0 = *(__errno_location());
                                        if (g_49fdc8)
                                            g_49fdc8();
                                        sub_45e548(ptr2);
                                        error(0, v0, gettext("%s: Cannot remove"));
                                        g_49fdd0 = 2;
                                    }
                                }
                            }
                        }
                    }
                    else if (*(__errno_location()) != 2)
                    {
                        sub_42b444(ptr2);
                        if (g_49fdc8)
                            g_49fdc8();
                        sub_45e548(ptr2);
                        error(0, 0, gettext("%s: Not purging directory: unable to stat"));
                    }
                }
                free(ptr2);
                sub_4209fc(v8);
                free(ptr1);
                return 1;
            }
            if (*((char *)iter) == 88)
            {
                len = strlen(iter + 1);
                ptr = sub_460b8d(ptr, len + 12, len + 12);
                memcpy(ptr, iter + 1, len);
                *((char *)(len + ptr)) = 47;
                memcpy(ptr + len + 1, "tar.XXXXXX", 11);
                if (!mkdtemp(ptr))
                    break;
            }
            else if (*((char *)iter) == 82)
            {
                v5 = iter + 1;
                iter += strlen(iter) + 1;
                v6 = iter + 1;
                if (*((char *)v5))
                    v5 = sub_43edd9(v5, 0, g_49f878);
                if (*((char *)v6))
                    v6 = sub_43edd9(v6, 0, g_49f878);
                if (!*((char *)v5))
                {
                    v5 = ptr;
                }
                else if (!*((char *)v6))
                {
                    v6 = ptr;
                }
                if ((char)sub_41c326(v5, v6) != 1)
                {
                    free(ptr);
                    free(ptr1);
                    return 0;
                }
            }
            iter += strlen(iter) + 1;
        }
        if (g_49fdc8)
            g_49fdc8();
        sub_45e84e(ptr);
        v14 = gettext("Cannot create temporary directory using template %s");
        err = __errno_location();
        error(0, *(err), v14);
        g_49fdd0 = 2;
        free(ptr);
        free(ptr1);
        return 0;
    }
}



// Function: purge_directory @ 0x25377
long long purge_directory(long long a0)
{
    unsigned long v1;  // rax

    v1 = sub_424ca0(a0) ^ 1;
    if (!(char)v1)
        return v1;
    return sub_428891();
}



// Function: list_dumpdir @ 0x253a2
typedef struct FILE {
} FILE;

extern FILE *g_49efd0;

int list_dumpdir(char *a0, unsigned long long a1)
{
    int v4;  // eax
    unsigned int v5;  // eax
    unsigned long long node;  // [bp-0x28]
    char *iter;  // [bp-0x20]
    unsigned int v2;  // [bp-0xc]

    iter = a0;
    node = a1;
    v2 = 0;
    while (node)
    {
        v5 = *(iter);
        if (!v5)
        {
            v4 = fputc_unlocked(10, g_49efd0);
            iter += 1;
            node -= 1;
            v2 = 0;
        }
        else if (v5 >= 0 && v5 <= 89 && v5 >= 0x44 && (char)(3228673 >> ((char)(v5 - 0x44) & 63)) & 1)
        {
            v4 = fprintf(g_49efd0, "%c", *(iter));
            if (!v2)
            {
                v4 = fprintf(g_49efd0, " ");
                v2 = 1;
            }
            iter += 1;
            node -= 1;
        }
        else
        {
            v4 = fputc_unlocked(*(iter), g_49efd0);
            iter += 1;
            node -= 1;
        }
    }
    return v4;
}



// Function: base64_init @ 0x254c9
extern char g_4821e0;
extern void g_49f4a0;

void* base64_init(void)
{
    void* dst;  // rax
    unsigned int i;  // [bp-0xc]

    dst = memset(&g_49f4a0, 64, 0x100);
    for (i = 0; i <= 63; i += 1)
    {
        *(&(&g_49f4a0)[*(&(&g_4821e0)[i])]) = i;
    }
    return dst;
}



// Function: decode_xform @ 0x25529
extern char g_49f878;
extern long long g_49f970;

long long decode_xform(unsigned long a0, int *a1)
{
    unsigned long iter;  // [bp-0x20]
    int v1;  // [bp-0x14]
    unsigned long len;  // [bp-0x10]

    iter = a0;
    v1 = *(a1);
    switch (v1)
    {
    case 4:
        return iter;
    case 1:
        iter = sub_43edd9(iter, 0, g_49f878);
        goto LABEL_4255aa;
    case 2:
        iter = sub_43edd9(iter, 1, g_49f878);
        goto LABEL_4255aa;
    default:
LABEL_4255aa:
        if (g_49f970)
        {
            len = sub_42ef5c(iter, g_49f970, g_49f970);
            if (len == 0xffffffffffffffff)
            {
                len = strlen(iter);
                break;
            }
            iter += len;
        }
        return iter;
    }
}



// Function: transform_member_name @ 0x255f5
long long transform_member_name(long long a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x14]

    v0 = a1;
    return sub_43b8af(a0, v0, sub_425529, &v0);
}



// Function: enforce_one_top_level @ 0x25627
typedef struct struct_0 {
    char field_0;
} struct_0;

extern void g_49f938;

long long enforce_one_top_level(struct_0 **a0)
{
    unsigned long v4;  // rax
    unsigned int v0;  // [bp-0x1c]
    char *i;  // [bp-0x18]
    char *v2;  // [bp-0x10]

    v2 = &*(a0)->field_0;
    for (i = v2; *(i) && (*(i) == 47 || *(i) == 46); i += 1);
    if (*(i))
    {
        v0 = strlen(*((long long *)&g_49f938));
        if (!strncmp(i, *((long long *)&g_49f938), v0))
        {
            v4 = i[v0];
            if ((char)v4 == 47 || !(v4 = (unsigned long)(unsigned long long)i[v0], (char)v4))
            {
                return v4;
            }
        }
        *(a0) = sub_42ee85(*((long long *)&g_49f938), v2, v2);
        sub_429946(*(a0));
    }
    else
    {
        *(a0) = sub_460cc6(*((long long *)&g_49f938));
    }
    return (unsigned long long)free(v2);
}



// Function: transform_stat_info @ 0x25733
extern char g_49f936;
extern char *g_49fa28;

long long transform_stat_info(unsigned int a0, long long a1)
{
    long long v1;  // rax

    if (a0 == 86)
        return v1;
    sub_4255f5(a1 + 8, 1);
    if (a0 == 49)
    {
        sub_4255f5(a1 + 24, 2);
    }
    else if (a0 == 50)
    {
        sub_4255f5(a1 + 24, 4);
    }
    if (!g_49f936)
        return g_49f936;
    return sub_425627(&g_49fa28);
}



// Function: read_and @ 0x257b8
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

typedef struct FILE {
} FILE;

extern unsigned int g_49ec84;
extern FILE *g_49efd0;
extern struct_0 *g_49f460;
extern long long g_49f468;
extern unsigned long long g_49f480;
extern unsigned long long g_49f488;
extern char g_49f88c;
extern char g_49f8bd;
extern long long g_49f900;
extern char g_49f908;
extern char g_49f978;
extern long long g_49fa20;
extern void g_49fa28;
extern long long g_49fb18;
extern long long g_49fb20;
extern struct_1 *g_49fbb8;
extern struct_1 *g_49fdc8;
extern unsigned int g_49fdd0;

void read_and(unsigned long long *a0)
{
    long long v7;  // rdi
    long long v8;  // rsi
    char *v18;  // rax
    long long v20;  // rax
    long long v22;  // rax
    char *v23;  // rax
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    unsigned int v13;  // eax
    long long v15;  // rax
    char *v16;  // rax
    unsigned int v0;  // [bp-0x58]
    unsigned int v1;  // [bp-0x54]
    unsigned long long iter;  // [bp-0x50], Other Possible Types: long long
    unsigned long v3;  // [bp-0x48]
    unsigned long long v4;  // [bp-0x40]
    char v5;  // [bp-0x38]

    v0 = 0;
    sub_4254c9();
    sub_42d3b1(v7, v8, v9, v10, v11, v12);
    sub_40f694(0);
    while (1)
    {
        v1 = v0;
        sub_439d1c(&g_49fa20);
        v0 = sub_425ecf(&g_49f460, &g_49fa20, 0);
        switch (v0)
        {
        case 0: case 2:
            abort(); /* do not return */
            continue;
        case 1:
            sub_426747(g_49f460, &g_49fa20, &g_49f468, 1);
            if ((char)sub_42d88a(*((long long *)&g_49fa28)) != 1 || *((long long *)&g_49f908) >= 0 && !(v3 = (unsigned long)(unsigned long long)sub_4279d7(g_49f460 + 0x88, 12), v4 = 0, g_49fb18 = (long long)v3, g_49fb20 = (long long)v4, !((char)((unsigned int)(unsigned long long)sub_45ff90(g_49fb18, g_49fb20, g_49f900, *((long long *)&g_49f908)) >> 31) & 1)) || (char)sub_417c43(*((long long *)&g_49fa28), g_49fbb8))
            {
                v13 = *((char *)&g_49f460[9].field_8 + 4);
                switch (v13)
                {
                case 77: case 86:
                    break;
                case 53:
                    if (g_49f978)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        sub_45e548(*((long long *)&g_49fa28));
                        error(0, 0, gettext("%s: Omitting"));
                    }
                    break;
                default:
                    sub_428891();
                    goto LABEL_425cb1;
                }
            }
            sub_425733(*((char *)&g_49f460[9].field_8 + 4), &g_49fa20);
            a0(v7, v8, v9, v10, v11, v12);
            goto LABEL_425cb1;
        case 3:
            if (g_49f88c)
            {
                v15 = sub_40bacd();
                sub_454e5a(v15, &v5, v15);
                v16 = gettext("block %s: ** Block of NULs **\n");
                fprintf(g_49efd0, v16);
            }
            sub_40bba7(g_49f460);
            if (g_49f8bd != 1)
            {
                v0 = sub_425ecf(&g_49f460, &g_49fa20, 0);
                if (v0 != 3)
                {
                    if ((char)g_49ec84 & 1)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        v20 = sub_40bacd();
                        sub_454e5a(v20, &v5, v20);
                        error(0, 0, gettext("A lone zero block at %s"));
                    }
                    break;
                }
            }
            else
            {
                v0 = v1;
                goto LABEL_425cb1;
            }
        case 4:
            if (g_49f88c)
            {
                v22 = sub_40bacd();
                sub_454e5a(v22, &v5, v22);
                v23 = gettext("block %s: ** End of File **\n");
                fprintf(g_49efd0, v23);
            }
            break;
        case 5:
            sub_40bba7(g_49f460);
            switch (v1)
            {
            case 0:
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("This does not look like a tar archive"));
                g_49fdd0 = 2;
            case 1: case 3:
                if (g_49f88c)
                {
                    iter = sub_40bacd();
                    iter -= g_49f480;
                    iter -= g_49f488;
                    sub_454e5a(iter, &v5, &v5);
                    v18 = gettext("block %s: ");
                    fprintf(g_49efd0, v18);
                }
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Skipping to next header"));
                g_49fdd0 = 2;
                goto LABEL_425ca4;
            case 2:
                abort(); /* do not return */
            case 4: case 5:
LABEL_425ca4:
                goto LABEL_425cb0;
            default:
LABEL_425cb0:
LABEL_425cb1:
                if ((char)sub_42da15(&g_49fa20) == 1)
                {
                    sub_40d046();
                    sub_42dbba(v7, v8, v9, v10, v11, v12);
                    return;
                }
                break;
            }
            continue;
        }
    }
    sub_40d046();
    sub_42dbba(v7, v8, v9, v10, v11, v12);
    return;
}



// Function: list_archive @ 0x25cf3
extern long long g_49f460;
extern char g_49f8be;
extern int g_49f9ac;
extern char g_49fa20;
extern void* g_49fbb0;

long long list_archive(void)
{
    unsigned long long v2;  // rax
    long long v0;  // [bp-0x10]

    v0 = sub_40bacd();
    if (g_49f9ac)
        sub_42864c(&g_49fa20, g_49f460, v0);
    if (g_49f8be && g_49f9ac > 2 && (char)sub_42490b(&g_49fa20))
    {
        v2 = sub_420bfe(g_49fbb0);
        sub_4253a2(g_49fbb0, v2);
    }
    return sub_428891();
}



// Function: tar_checksum @ 0x25d84
unsigned int tar_checksum(void* a0, char a1)
{
    void* v7;  // rax
    int iter;  // [bp-0x28]
    int node;  // [bp-0x24]
    int v2;  // [bp-0x20]
    int v3;  // [bp-0x1c]
    unsigned long long iter1;  // [bp-0x18]
    void* v5;  // [bp-0x10]

    iter = 0;
    node = 0;
    v5 = a0;
    iter1 = 0x200;
    while (1)
    {
        iter1 -= 1;
        if (!iter1)
            break;
        iter += *((char *)v5);
        v7 = v5;
        v5 = v7 + 1;
        node += *((char *)v7);
    }
    if (!iter)
        return 3;
    iter1 = 8;
    while (1)
    {
        iter1 -= 1;
        if (!iter1)
            break;
        iter -= *(iter1 + (char *)a0 + 148);
        node -= *(iter1 + (char *)a0 + 148);
    }
    iter += 0x100;
    node += 0x100;
    v2 = sub_426c90(a0 + 148, 8, 0, 0, 0x7fffffff, 1, a1);
    if (v2 < 0)
        return 5;
    v3 = v2;
    if (iter == v3 || node == v3)
        return 1;
    return 5;
    return 1;
}



// Function: read_header @ 0x25ecf
typedef struct struct_3 {
    char padding_0[8];
    long long field_8;
    char field_10;
    char padding_11[119];
    unsigned long long field_88;
} struct_3;

typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[337];
    char field_159;
} struct_0;

typedef struct struct_1 {
    char field_0[8];
    char padding_8[504];
    char field_200;
} struct_1;

typedef struct struct_4 {
    unsigned long long field_0;
} struct_4;

extern void* g_49f470;
extern void* g_49f478;
extern unsigned long long g_49f480;
extern unsigned long long g_49f488;
extern unsigned long long g_49f490;
extern struct_4 *g_49fdc8;
extern unsigned int g_49fdd0;

long long read_header(struct_0 **a0, struct_3 *idx, unsigned int a2)
{
    struct_1 *v24;  // rdx
    unsigned long long *node;  // rdi
    struct_1 *v26;  // rdx
    struct_0 *i;  // rax
    unsigned long long v28;  // rdx
    unsigned int v0;  // [bp-0x1b4]
    struct_1 *iter1;  // [bp-0x1b0]
    unsigned long iter;  // [bp-0x1a8]
    unsigned long n;  // [bp-0x1a0]
    struct_1 *ptr;  // [bp-0x198]
    struct_1 *ptr1;  // [bp-0x190]
    unsigned long long v6;  // [bp-0x188]
    unsigned long long v7;  // [bp-0x180]
    struct_1 *v8;  // [bp-0x178]
    char *ptr2;  // [bp-0x170]
    char *v10;  // [bp-0x170]
    struct_0 *v11;  // [bp-0x168]
    struct_0 *v12;  // [bp-0x160]
    unsigned long v13;  // [bp-0x158]
    unsigned long long v14;  // [bp-0x150]
    struct_1 *v15;  // [bp-0x148]
    unsigned long v16;  // [bp-0x140]
    struct_1 v17;  // [bp-0x138]
    char v18;  // [bp-0x118]
    char v19;  // [bp-0xb4]
    char v20;  // [bp-0x7d]

    ptr = NULL;
    ptr1 = NULL;
    v6 = 0;
    v7 = 0;
    v0 = 1;
    while (1)
    {
        v11 = sub_40bb4b();
        *(a0) = v11;
        if (!v11)
        {
            v0 = 4;
            free(ptr);
            free(ptr1);
            return v0;
        }
        v0 = sub_425d84(v11, 0);
        if (v0 != 1)
        {
            free(ptr);
            free(ptr1);
            return v0;
        }
        if (v11->padding_8[148] == 49)
        {
            idx->field_88 = 0;
        }
        else
        {
            idx->field_88 = sub_42798b(&v11->padding_8[116], 12);
            if (idx->field_88 < NULL)
            {
                v0 = 5;
                free(ptr);
                free(ptr1);
                return v0;
            }
        }
        if (v11->padding_8[148] != 76 && v11->padding_8[148] != 75 && v11->padding_8[148] != 120 && v11->padding_8[148] != 103 && v11->padding_8[148] != 88)
        {
            v12 = v11;
            free(g_49f470);
            if (ptr)
            {
                v8 = &ptr->field_200;
                g_49f470 = ptr;
                g_49f480 = v6;
                ptr = NULL;
                break;
            }
            else
            {
                ptr2 = &v18;
                if (v12->field_159 && !strcmp(&v12->padding_8[249], "ustar"))
                {
                    memcpy(ptr2, &v12->field_159, 155);
                    v20 = 0;
                    v10 = &ptr2[strlen(ptr2)];
                    ptr2 = v10 + 1;
                    *(v10) = 47;
                }
                memcpy(ptr2, v12, 100);
                ptr2[100] = 0;
                v8 = &v18;
                g_49f470 = 0;
                g_49f480 = 0;
                break;
            }
        }
        if (a2 == 1)
        {
            v0 = 2;
            free(ptr);
            free(ptr1);
            return v0;
        }
        if (v11->padding_8[148] == 76 || v11->padding_8[148] == 75)
        {
            v13 = idx->field_88;
            v14 = (unsigned int)v13 & 0x1ff;
            iter = v13 + 0x200;
            if (v14)
                iter = iter - v14 + 0x200;
            if (idx->field_88 != v13 || iter < v13)
                sub_41783c(); /* do not return */
            v15 = sub_460b5b(iter + 1);
            if (v11->padding_8[148] == 76)
            {
                free(ptr);
                ptr = v15;
                v6 = iter >> 9;
            }
            else
            {
                free(ptr1);
                ptr1 = v15;
                v7 = iter >> 9;
            }
            sub_40bba7(v11);
            v24 = v15;
            *((unsigned long long *)&v24->field_0[0]) = v11->field_0;
            *((long long *)&v24->padding_8[496]) = *((long long *)&v11[1].padding_8[150]);
            node = v24->padding_8 & 0xfffffffffffffff8;
            v26 = v24 - node;
            i = v11 - v26;
            for (v28 = (0x200 + (unsigned int)v26 & 0xfffffff8) >> 3; v28; i = i->padding_8)
            {
                v28 -= 1;
                *(node) = i->field_0;
                node += 1;
            }
            iter1 = &v15->field_200;
            for (iter -= 0x200; iter; iter -= n)
            {
                v16 = sub_40bb4b();
                if (!v16)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Unexpected EOF in archive"));
                    g_49fdd0 = 2;
                    break;
                }
                else
                {
                    n = sub_40bbf5(v16);
                    if (n > iter)
                        n = iter;
                    memcpy(iter1, v16, n);
                    iter1 = &iter1->field_0[n];
                    sub_40bba7(v16 + n - 1);
                }
            }
            iter1->field_0[0] = 0;
        }
        else if (v11->padding_8[148] == 120 || v11->padding_8[148] == 88)
        {
            sub_41e28e(&idx[2].padding_11[55], v11, sub_42798b(&v11->padding_8[116], 12));
        }
        else
        {
            if (v11->padding_8[148] == 103)
            {
                if (!g_49f490)
                    g_49f490 = sub_460b5b(0x200);
                memcpy(g_49f490, v11, 0x200);
                memset(&v17, 0, 32);
                sub_41e28e(&v17, v11, sub_42798b(&v11->padding_8[116], 12));
                sub_41e0e7(&v17);
                sub_41e785(&v17);
                if (a2 == 2)
                {
                    v0 = 2;
                    free(ptr);
                    free(ptr1);
                    return v0;
                }
            }
            goto LABEL_426706;
        }
LABEL_426706:
    }
    sub_429547(idx, v8, v8);
    sub_429547(&idx->field_8, v8, &idx->field_8);
    idx->field_10 = sub_44e2e5(idx->field_8);
    free(g_49f478);
    if (ptr1)
    {
        v8 = &ptr1->field_200;
        g_49f478 = ptr1;
        g_49f488 = v7;
        ptr1 = NULL;
    }
    else
    {
        memcpy(&v18, &v12->padding_8[149], 100);
        v19 = 0;
        v8 = &v18;
        g_49f478 = 0;
        g_49f488 = 0;
    }
    sub_429547(&idx->padding_11[7], v8, &idx->padding_11[7]);
    free(ptr);
    free(ptr1);
    return v0;
}



// Function: decode_header @ 0x26747
typedef struct struct_1 {
    char padding_0[112];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
    unsigned long long field_108;
    unsigned long long field_110;
    char padding_118[88];
    unsigned long long field_170;
} struct_1;

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

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern struct_2 *g_49f460;
extern unsigned int g_49f468;
extern char g_49f8be;
extern char g_49f934;
extern unsigned long long g_49f9e0;
extern unsigned long long g_49f9e8;

void decode_header(struct_0 *a0, struct_1 *idx, unsigned int *a2, unsigned int a3)
{
    unsigned long v4;  // rdx
    struct_1 *v5;  // rdx
    struct_1 *v6;  // rdx
    struct_1 *index;  // rcx
    struct_1 *idx1;  // rcx
    unsigned long v9;  // rdx
    struct_1 *v10;  // rdx
    unsigned int v11;  // eax
    char v0;  // [bp-0x29]
    unsigned int v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x24]

    v2 = sub_42790d(&a0->padding_0[100], 8, &v0, &a0->padding_0[100]);
    if (!strcmp(&a0->padding_9d[100], "ustar"))
    {
        if (!a0->field_1db && a0->field_1dc > 47 && a0->field_1dc <= 55 && a0->field_1e7 == 32 && a0->field_1e8 > 47 && a0->field_1e8 <= 55 && a0->field_1f3 == 32)
            v1 = 5;
        else
            v1 = (!idx->field_170 ? 3 : 4);
    }
    else
    {
        if (!strcmp(&a0->padding_9d[100], "ustar  "))
            v1 = (!v0 ? 6 : 2);
        else
            v1 = 1;
    }
    *(a2) = v1;
    v4 = v2;
    idx->field_70 = v4;
    idx->field_f8 = sub_4279d7(&a0->padding_0[0x88], 12, v4);
    idx->field_100 = 0;
    sub_429591(&idx->padding_0[32], (!a0->field_109 ? 0 : (struct struct_0 *)&a0->field_109), 32, &idx->padding_0[32]);
    sub_429591(&idx->padding_0[40], (!a0->field_129 ? 0 : (struct struct_0 *)&a0->field_129), 32, &idx->padding_0[40]);
    sub_41d4f4(idx);
    if (v1 == 2 && g_49f8be)
    {
        v5 = idx;
        v5->field_e8 = sub_4279d7(&a0->padding_12a[47], 12);
        idx->field_108 = sub_4279d7(&a0->padding_12a[59], 12, v5);
        idx->field_110 = 0;
        idx->field_f0 = idx->field_110;
    }
    else if (v1 == 5)
    {
        v6 = idx;
        v6->field_e8 = sub_4279d7(&a0->field_1dc, 12);
        idx->field_108 = sub_4279d7(&a0->field_1e8, 12, v6);
        idx->field_110 = 0;
        idx->field_f0 = idx->field_110;
    }
    else
    {
        index = idx;
        index->field_108 = g_49f9e0;
        index->field_110 = g_49f9e8;
        idx1 = idx;
        v9 = idx->field_110;
        idx1->field_e8 = idx->field_108;
        idx1->field_f0 = v9;
    }
    if (v1 == 1)
    {
        v10 = idx;
        v10->field_74 = sub_427a2b(&a0->padding_0[108], 8);
        idx->field_78 = sub_427831(&a0->padding_0[116], 8, v10);
        *((unsigned long long *)&idx->padding_7c[4]) = 0;
    }
    else
    {
        if (a3)
        {
            if (g_49f934 || !a0->field_109 || !sub_42c173(&a0->field_109, &idx->field_74, &idx->field_74))
                idx->field_74 = sub_427a2b(&a0->padding_0[108], 8);
            if (g_49f934 || !a0->field_129 || !sub_42c261(&a0->field_129, &idx->field_78, &idx->field_78))
                idx->field_78 = sub_427831(&a0->padding_0[116], 8);
        }
        if (a0->field_9c - 0x33 <= 1)
        {
            v11 = sub_4278c3(&a0->padding_12a[39], 8);
            *((unsigned long long *)&idx->padding_7c[4]) = gnu_dev_makedev(sub_427879(&a0->padding_12a[31], 8) & 0xffffffff, v11);
        }
        else
        {
            *((unsigned long long *)&idx->padding_7c[4]) = 0;
        }
    }
    sub_41df5f(idx);
    if ((char)sub_4301de(idx))
    {
        sub_430241(idx);
        idx->padding_118[8] = 1;
        return;
    }
    idx->padding_118[8] = 0;
    if ((g_49f468 != 6 && g_49f468 != 2 || *((char *)&g_49f460[19].field_0 + 4) != 0x44) && !*((long long *)&idx[1].padding_0[24]))
        return;
    idx[1].padding_0[16] = 1;
    return;
}



// Function: from_header @ 0x26c90
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern char g_49f4a0;
extern char g_49f5a1;
extern long long g_49f5a8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned long long from_header(void* a0, unsigned long n, unsigned long long a2, long long a3, unsigned long long a4, char a5, char a6)
{
    unsigned long long *v21;  // rax
    unsigned long long v31;  // rax
    unsigned long long *v33;  // rax
    unsigned long long v35;  // rax
    void* v23;  // rax
    void* v26;  // rax
    void* v27;  // rax
    void* v30;  // rax
    char v0;  // [bp-0x14d8]
    unsigned long long v1;  // [bp-0x4e8]
    char v2;  // [bp-0x4d8]
    char v3;  // [bp-0x4a6]
    char v4;  // [bp-0x4a5]
    unsigned int v5;  // [bp-0x4a4]
    int v6;  // [bp-0x4a0]
    unsigned int v7;  // [bp-0x49c]
    unsigned long long node;  // [bp-0x498]
    void* iter2;  // [bp-0x490]
    void* j;  // [bp-0x488]
    void* iter;  // [bp-0x480]
    void* iter1;  // [bp-0x478]
    long long v13;  // [bp-0x468]
    unsigned long long v14;  // [bp-0x460]
    unsigned long long v15;  // [bp-0x458]
    void* v16;  // [bp-0x450]
    char v17;  // [bp-0x448]
    char v18;  // [bp-0x427]
    char v19;  // [bp-0x408]
    char v20;  // [bp-0x407]

    v2 = a6;
    v13 = -(a3);
    iter2 = a0;
    j = n + iter2;
    v3 = 0;
    iter2 += !*((char *)iter2);
    while (1)
    {
        if (iter2 == j)
        {
            if (!a2 || v2 == 1)
                return 0xffffffffffffffff;
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Blanks in header where numeric %s value expected"));
            g_49fdd0 = 2;
            return 0xffffffffffffffff;
        }
        v21 = __ctype_b_loc();
        if (!(*((short *)(*((char *)iter2) * 2 + *(v21))) & 0x2000))
            break;
        iter2 += 1;
    }
    node = 0;
    if (*((char *)iter2) - 48 <= 7)
    {
        v16 = iter2;
        v4 = 0;
        while (1)
        {
            v23 = iter2;
            iter2 = v23 + 1;
            node += *((char *)v23) - 48;
            if (iter2 == j || *((char *)iter2) - 48 > 7)
                break;
            v4 = node != (0x1fffffffffffffff & node) || v4;
            node *= 8;
        }
        if ((v4 || a4 < node) && *((char *)v16) > 49 && a2)
        {
            v5 = *((char *)v16) - 48 | 4;
            v4 = 0;
            node = 0;
            iter2 = v16;
            while (1)
            {
                node += 7 - v5;
                iter2 += 1;
                if (iter2 == j || *((char *)iter2) - 48 > 7)
                    break;
                v5 = *((char *)iter2) - 48;
                v4 = node != (0x1fffffffffffffff & node) || v4;
                node *= 8;
            }
            node += 1;
            v4 = !node || v4;
            if (v4 != 1 && node <= v13)
            {
                if (v2 != 1)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Archive octal value %.*s is out of %s range; assuming two's complement"));
                }
                v3 = 1;
            }
        }
        if (v4)
        {
            if (!a2 || v2 == 1)
                return 0xffffffffffffffff;
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Archive octal value %.*s is out of %s range"));
            g_49fdd0 = 2;
            return 0xffffffffffffffff;
        }
    }
    else if (!a5)
    {
        if (*((char *)iter2) == 45 || *((char *)iter2) == 43)
        {
            if (v2 != 1 && g_49f5a1 != 1)
            {
                g_49f5a1 = 1;
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Archive contains obsolescent base-64 headers"));
            }
            v30 = iter2;
            iter2 = v30 + 1;
            for (v3 = *((char *)v30) == 45; iter2 != j && !(v7 = (unsigned int)(int)*((char *)(long long)(int)(unsigned int)(*((char *)iter2) + &g_49f4a0)), v7 > 63); iter2 += 1)
            {
                if (node != (0x3ffffffffffffff & node))
                {
                    v31 = (unsigned long long)((15 + n + 9) / 16) * 16;
                    while (&v0 != &(&v2)[-1 * (v31 & 0xfffffffffffff000)])
                    ;
                    /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v31 & 0xfff)];
                    if ((unsigned short)v31 & 0xfff)
                        *((long long *)(((unsigned int)v31 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v31 & 0xfff) - 8 + /* unsupported instruction */));
                    v15 = (/* unsupported instruction */ + 15 >> 4) * 16;
                    memcpy(v15, a0, n);
                    *((char *)(n + v15)) = 0;
                    if (!a2 || v2 == 1)
                        return 0xffffffffffffffff;
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_45e84e(v15);
                    error(0, 0, gettext("Archive signed base-64 string %s is out of %s range"));
                    g_49fdd0 = 2;
                    return 0xffffffffffffffff;
                }
                node = v7 | node * 64;
            }
        }
        else if (*((char *)iter2) == 128 || *((char *)iter2) == 0xff)
        {
            v6 = *((char *)iter2) & 64;
            v14 = -(v6) * 0x4000000000000;
            v26 = iter2;
            iter2 = v26 + 1;
            node = (*((char *)v26) & 63) - v6;
            while (1)
            {
                v27 = iter2;
                iter2 = v27 + 1;
                node = *((char *)v27) + node * 0x100;
                if (iter2 == j)
                    break;
                if (node != (0xffffffffffffff & node | v14))
                {
                    if (!a2 || v2 == 1)
                        return 0xffffffffffffffff;
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Archive base-256 value is out of %s range"));
                    g_49fdd0 = 2;
                    return 0xffffffffffffffff;
                }
            }
            v3 = v6;
            if (v3)
                node = -(node);
        }
    }
    if (iter2 != j && *((char *)iter2))
    {
        v33 = __ctype_b_loc();
        if (!(*((short *)(*((char *)iter2) * 2 + *(v33))) & 0x2000))
        {
            if (!a2)
                return 0xffffffffffffffff;
            if (!g_49f5a8)
            {
                g_49f5a8 = sub_45c92f(0);
                sub_45c9a2(g_49f5a8, 8);
            }
            for (; a0 != j && !*((char *)j - 1); j -= 1);
            sub_45dd44(&v19, 1000, a0, j - a0, g_49f5a8);
            if (v2 != 1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Archive contains %.*s where numeric %s value expected"));
                g_49fdd0 = 2;
            }
            return 0xffffffffffffffff;
        }
    }
    if ((!v3 ? a4 : v13) >= node)
    {
        return sub_4294ad((!v3 ? node : -(node)));
    }
    else if (a2 && v2 != 1)
    {
        iter = sub_454e5a(v13, &v18, v13);
        iter1 = sub_454e5a(node, &v20, node);
        if (v3)
        {
            iter1 -= 1;
            *((char *)iter1) = 45;
        }
        if (v13)
        {
            iter -= 1;
            *((char *)iter) = 45;
        }
        if (g_49fdc8)
            g_49fdc8();
        v35 = sub_454e5a(a4, &v17, &v17);
        v1 = v35;
        error(0, 0, gettext("Archive value %s is out of %s range %s..%s"));
        g_49fdd0 = 2;
    }
    return 0xffffffffffffffff;
}



// Function: gid_from_header @ 0x27831
unsigned long long gid_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "gid_t", 0, 0xffffffff, 0, 0);
}



// Function: major_from_header @ 0x27879
unsigned long long major_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "major_t", -0x80000000, 0x7fffffff, 0, 0);
}



// Function: minor_from_header @ 0x278c3
unsigned long long minor_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "minor_t", -0x80000000, 0x7fffffff, 0, 0);
}



// Function: mode_from_header @ 0x2790d
long long mode_from_header(void* a0, unsigned int a1, char *a2)
{
    unsigned int v0;  // [bp-0x14]
    unsigned long long v1;  // [bp-0x10]

    v1 = sub_426c90(a0, a1, "mode_t", -0x8000000000000000, 0xffffffffffffffff, 0, 0);
    v0 = (unsigned int)v1 & 0xfff;
    *(a2) = v1 & 0xfffffffffffff000;
    return v0;
}



// Function: off_from_header @ 0x2798b
unsigned long long off_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "off_t", 0, 0x7fffffffffffffff, 0, 0);
}



// Function: time_from_header @ 0x279d7
unsigned long long time_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "time_t", -0x8000000000000000, 0x7fffffffffffffff, 0, 0);
}



// Function: uid_from_header @ 0x27a2b
unsigned long long uid_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "uid_t", 0, 0xffffffff, 0, 0);
}



// Function: uintmax_from_header @ 0x27a73
unsigned long long uintmax_from_header(void* a0, unsigned int a1)
{
    return sub_426c90(a0, a1, "uintmax_t", 0, 0xffffffffffffffff, 0, 0);
}



// Function: tartime @ 0x27abc
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

extern char g_49f5c0;
extern long long g_49f5c6;
extern char g_49f5d4;
extern long long g_49f5da;
extern char g_49f5e1;
extern char g_49f879;

char * tartime(unsigned long a0, unsigned int a1, char a2)
{
    unsigned long len;  // rax
    char v0;  // [bp-0x3d]
    unsigned int v1;  // [bp-0x3c]
    unsigned long v2;  // [bp-0x38]
    char *iter;  // [bp-0x30]
    tm *v4;  // [bp-0x28]

    v2 = a0;
    v1 = a1;
    v0 = v2 >> 63;
    if (v0 && v1)
    {
        v2 += 1;
        v1 = 1000000000 - v1;
    }
    v4 = (!g_49f879 ? localtime(&v2) : gmtime(&v2));
    if (v4)
    {
        if (a2)
        {
            strftime(&g_49f5c0, 37, "%Y-%m-%d %H:%M:%S", v4);
            len = strlen(&g_49f5c0);
            sub_429e23(v1, &(&g_49f5c0)[len], &(&g_49f5c0)[len]);
        }
        else
        {
            strftime(&g_49f5c0, 37, "%Y-%m-%d %H:%M", v4);
        }
        return &g_49f5c0;
    }
    else
    {
        if (v0)
            v2 = -(v2);
        iter = sub_454e5a(v2, &g_49f5c6);
        if (v0)
        {
            iter -= 1;
            *(iter) = 45;
        }
        for (; (!a2 ? &g_49f5d4 : &g_49f5e1) < iter; *(iter) = 32)
        {
            iter -= 1;
        }
        if (!a2)
            return iter;
        sub_429e23(v1, &g_49f5da, &g_49f5da);
        return iter;
    }
}



// Function: simple_print_header @ 0x27c80
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char field_10;
    char padding_11[7];
    long long field_18;
    struct struct_1 *field_20;
    struct struct_1 *field_28;
    char padding_30[64];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[4];
    long long field_80;
    long long field_88;
    char padding_90[104];
    unsigned int field_f8;
    char padding_fc[4];
    unsigned long long field_100;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

typedef struct FILE {
} FILE;

extern unsigned int g_49c260;
extern unsigned int g_49c264;
extern FILE *g_49efd0;
extern unsigned int g_49f468;
extern unsigned long long g_49f480;
extern unsigned long long g_49f488;
extern char g_49f5a0;
extern char g_49f87a;
extern char g_49f88c;
extern char g_49f934;
extern int g_49f9ac;
extern char g_49fc30;
extern struct_3 *g_49fdc8;
extern unsigned int g_49fdd0;

void simple_print_header(struct_0 *idx, void* a1, unsigned long a2)
{
    char *v21;  // rax
    char *v22;  // rax
    unsigned int v32;  // eax
    char *v33;  // rax
    char *v34;  // rax
    char *v35;  // rax
    char *v36;  // rax
    char *v37;  // rax
    char *v38;  // rax
    unsigned long v39;  // rax
    char *v41;  // rax
    char *v42;  // rax
    unsigned long v24;  // rdx
    unsigned long v25;  // rdx
    unsigned long v26;  // rdx
    unsigned int v27;  // eax
    unsigned int v29;  // eax
    char *v0;  // [bp-0x138]
    unsigned long v1;  // [bp-0x130]
    void* v2;  // [bp-0x128]
    long long iter;  // [bp-0x110], Other Possible Types: unsigned long, unsigned long long
    unsigned int v4;  // [bp-0xf4]
    unsigned int v5;  // [bp-0xf0]
    int v6;  // [bp-0xec]
    long long v7;  // [bp-0xe8]
    char *v8;  // [bp-0xe0]
    char *v9;  // [bp-0xd8]
    void* v10;  // [bp-0xd0]
    char v11;  // [bp-0xc6]
    char v12;  // [bp-0xc5]
    char v13;  // [bp-0xc4]
    char v14;  // [bp-0xc3]
    char v15;  // [bp-0xba]
    char v16;  // [bp-0xb8]
    char v17;  // [bp-0x98]
    char v18;  // [bp-0x78]
    char v19[56];  // [bp-0x58]

    iter = a2;
    if (g_49fc30)
        v7 = (!idx->field_8 ? idx->field_0 : idx->field_8);
    else
        v7 = (!idx->field_0 ? idx->field_8 : idx->field_0);
    if (g_49f88c)
    {
        if (iter - 0 >> 63 & 1)
            iter = sub_40bacd();
        iter -= g_49f480;
        iter -= g_49f488;
        sub_454e5a(iter, v19, v19);
        v21 = gettext("block %s: ");
        fprintf(g_49efd0, v21);
    }
    if (g_49f9ac <= 1)
    {
        v22 = sub_45e2eb(v7);
        fputs_unlocked(v22, g_49efd0);
        if (g_49fc30 && idx->field_10)
            fputc_unlocked(47, g_49efd0);
        fputc_unlocked(10, g_49efd0);
        fflush_unlocked(g_49efd0);
        sub_43db33(idx);
        return;
    }
    else
    {
        v13 = 63;
        switch ((char)a1[156])
        {
        case 0: case 48: case 83:
            v13 = (!idx->field_10 ? 45 : 100);
            break;
        case 49:
            v13 = 104;
            break;
        case 50:
            v13 = 108;
            break;
        case 51:
            v13 = 99;
            break;
        case 52:
            v13 = 98;
            break;
        case 53:
            v13 = 100;
            break;
        case 54:
            v13 = 112;
            break;
        case 55:
            v13 = 67;
            break;
        case 68:
            v13 = 100;
            break;
        case 75: case 76:
            v13 = 76;
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Unexpected long name header"));
            g_49fdd0 = 2;
            break;
        case 77:
            v13 = 77;
            break;
        case 86:
            g_49f5a0 = 1;
            v13 = 86;
            break;
        }
        v24 = idx->field_70;
        sub_43dd6f(v24 & 0xffffffff, &v14, v24);
        sub_43d9ec(idx, &v15, idx);
        v10 = sub_427abc(*((long long *)&idx->field_f8), idx->field_100, g_49f87a);
        v4 = strlen(v10);
        if (v4 > g_49c264)
            g_49c264 = v4;
        if (idx->field_20 && idx->field_20->field_0 && g_49f468 != 1 && g_49f934 != 1)
        {
            v8 = &idx->field_20->field_0;
        }
        else
        {
            v25 = idx->field_74;
            v8 = sub_454e5a(v25, &v16, v25);
        }
        if (idx->field_28 && idx->field_28->field_0 && g_49f468 != 1 && g_49f934 != 1)
        {
            v9 = &idx->field_28->field_0;
        }
        else
        {
            v26 = idx->field_78;
            v9 = sub_454e5a(v26, &v17, v26);
        }
        if ((char)a1[156] - 0x33 <= 1)
        {
            v27 = gnu_dev_major(idx->field_80);
            strcpy(v19, sub_454e5a(v27, &v18, v27));
            strcat(v19, ",");
            v29 = gnu_dev_minor(idx->field_80);
            strcat(v19, sub_454e5a(v29, &v18, v29));
        }
        else
        {
            strcpy(v19, sub_454e5a(idx->field_88, &v18, idx->field_88));
        }
        v5 = strlen(v19);
        v32 = strlen(v8);
        v6 = v5 + v32 + (unsigned int)strlen(v9) + 2;
        if (v6 > g_49c260)
            g_49c260 = v6;
        v2 = v10;
        v1 = g_49c264;
        v0 = &v19[0];
        fprintf(g_49efd0, "%s %s/%s %*s %-*s", &v13, v8, v9);
        v33 = sub_45e2eb(v7);
        fprintf(g_49efd0, " %s", v33);
        if (g_49fc30 && idx->field_10)
            fputc_unlocked(47, g_49efd0);
        switch ((char)a1[156])
        {
        case 0: case 48: case 51: case 52: case 53: case 54: case 55: case 68: case 83:
            putc_unlocked(10, g_49efd0);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        case 49:
            sub_45e2eb(idx->field_18);
            v35 = gettext(" link to %s\n");
            fprintf(g_49efd0, v35);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        case 50:
            v34 = sub_45e2eb(idx->field_18);
            fprintf(g_49efd0, " -> %s\n", v34);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        case 75:
            v36 = gettext("--Long Link--\n");
            fprintf(g_49efd0, v36);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        case 76:
            v37 = gettext("--Long Name--\n");
            fprintf(g_49efd0, v37);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        case 77:
            v39 = sub_427a73(a1 + 369, 12);
            strcpy(v19, sub_454e5a(v39, &v18, v39));
            v41 = gettext("--Continued at byte %s--\n");
            fprintf(g_49efd0, v41);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        case 86:
            v38 = gettext("--Volume Header--\n");
            fprintf(g_49efd0, v38);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        default:
            v11 = (char)a1[156];
            v12 = 0;
            sub_45e84e(&v11);
            v42 = gettext(" unknown file type %s\n");
            fprintf(g_49efd0, v42);
            fflush_unlocked(g_49efd0);
            sub_43db33(idx);
            return;
        }
    }
}



// Function: print_volume_label @ 0x28528
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

extern long long g_49efe0;
extern unsigned long long g_49f490;

void print_volume_label(void)
{
    unsigned int v0;  // [bp-0x3dc]
    char v1;  // [bp-0x3d8]
    char v2;  // [bp-0x3d0]
    struct_0 v3;  // [bp-0x218]
    char v4;  // [bp-0x190]
    char v5;  // [bp-0x17c]

    memset(&v3, 0, 0x200);
    v5 = 86;
    if (g_49f490)
        memcpy(&v4, g_49f490 + 0x88, 12);
    sub_439c55(&v1);
    sub_429547(&v2, ".", ".");
    sub_426747(&v3, &v1, &v0, 0);
    sub_429547(&v2, g_49efe0, &v2);
    sub_427c80(&v1, &v3, 0);
    sub_439d1c(&v1);
    return;
}



// Function: print_header @ 0x2864c
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char field_10;
    char padding_11[7];
    long long field_18;
    struct struct_1 *field_20;
    struct struct_1 *field_28;
    char padding_30[64];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[4];
    long long field_80;
    long long field_88;
    char padding_90[104];
    unsigned int field_f8;
    char padding_fc[4];
    unsigned long long field_100;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_49efe0;
extern unsigned int g_49f468;
extern char g_49f5a0;

void print_header(struct_0 *a0, void* a1, unsigned int a2)
{
    if (g_49f468 != 4)
    {
        sub_427c80(a0, a1, a2);
        return;
    }
    else if (g_49f5a0 != 1)
    {
        if (g_49efe0)
        {
            sub_428528();
            g_49f5a0 = 1;
            sub_427c80(a0, a1, a2);
        }
        else
        {
            sub_427c80(a0, a1, a2);
        }
        return;
    }
    else
    {
        sub_427c80(a0, a1, a2);
        return;
    }
}



// Function: print_for_mkdir @ 0x286af
typedef struct FILE {
} FILE;

extern unsigned int g_49c260;
extern unsigned int g_49c264;
extern FILE *g_49efd0;
extern char g_49f88c;
extern int g_49f9ac;

void print_for_mkdir(long long a0, unsigned int a1, unsigned int a2)
{
    long long v5;  // rax
    char *v6;  // rax
    unsigned int v0;  // [bp-0x54]
    char v1;  // [bp-0x43]
    char v2;  // [bp-0x42]
    char v3;  // [bp-0x38]

    v0 = a1;
    if (g_49f9ac <= 1)
        return;
    v1 = 100;
    sub_43dd6f(a2, &v2, a2);
    if (g_49f88c)
    {
        v5 = sub_40bacd();
        sub_454e5a(v5, &v3, v5);
        v6 = gettext("block %s: ");
        fprintf(g_49efd0, v6);
    }
    sub_45e2eb(a0);
    gettext("Creating directory:");
    fprintf(g_49efd0, "%s %*s %s\n", &v1, g_49c260 + 1 + g_49c264);
    return;
}



// Function: skip_file @ 0x287c1
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

extern char g_49fc18;
extern struct_0 *g_49fdc8;

struct_0 * skip_file(unsigned long a0)
{
    struct_0 *v4;  // rax
    unsigned long iter;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    iter = a0;
    if (g_49fc18)
    {
        v1 = sub_40ce79(iter);
        if (!(v1 - 0 >> 63 & 1))
            iter -= v1 * 0x200;
        else
            g_49fc18 = 0;
    }
    v4 = sub_40ad86(iter);
    while (1)
    {
        if (iter <= NULL)
            return v4;
        v2 = sub_40bb4b();
        if (!v2)
            break;
        sub_40bba7(v2);
        iter -= 0x200;
        v4 = sub_40ad86(iter);
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Unexpected EOF in archive"));
    sub_4177f4(); /* do not return */
}



// Function: skip_member @ 0x28891
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f460;
extern char g_49fa20;
extern unsigned int g_49faa8;
extern char g_49fb40;
extern char g_49fba9;

long long skip_member(void)
{
    long long v2;  // rax
    char v0;  // [bp-0x9]

    v2 = g_49fba9 ^ 1;
    if (!(char)v2)
        return v2;
    v0 = *((char *)&g_49f460[9].field_8 + 4);
    sub_40bba7(g_49f460);
    sub_40ad2b(&g_49fa20);
    if (g_49fb40)
    {
        sub_4303bd(&g_49fa20);
    }
    else if (v0 != 53)
    {
        sub_4287c1(*((long long *)&g_49faa8));
    }
    return sub_40ad66();
}



// Function: test_archive_label @ 0x28913
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern void g_49efe0;
extern struct_0 *g_49f460;
extern unsigned int g_49f468;
extern char g_49f8fc;
extern unsigned int g_49f9ac;
extern char g_49fa20;

long long test_archive_label(void)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned long v0;  // [bp-0x10]

    sub_4254c9();
    sub_42d3b1(v2, v3, v4, v5, v6, v7);
    sub_40f694(0);
    if ((int)sub_425ecf(&g_49f460, &g_49fa20, 0) == 1)
    {
        sub_426747(g_49f460, &g_49fa20, &g_49f468, 0);
        if (*((char *)&g_49f460[9].field_8 + 4) == 86)
            sub_429591(&g_49efe0, g_49f460, 100);
        if (*((long long *)&g_49efe0))
        {
            if (g_49f9ac)
                sub_428528();
            if ((char)sub_42d88a(*((long long *)&g_49efe0)) != 1 && g_49f8fc)
            {
                v0 = sub_40e680(*((long long *)&g_49efe0));
                sub_42d88a(v0);
                free(v0);
            }
        }
    }
    sub_40d046();
    return sub_42dd64(v2, v3, v4, v5, v6, v7);
}



// Function: map_hash @ 0x28a20
long long map_hash(unsigned long long *a0, unsigned long long a1)
{
    return *(a0) % a1;
}



// Function: map_compare @ 0x28a4d
long long map_compare(unsigned long a0, unsigned long long *a1)
{
    return _INSERT(*(a1), 0, *((long long *)a0) == *(a1));
}



// Function: parse_id @ 0x28a83
unsigned int parse_id(unsigned long long *a0, long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned int a5)
{
    long long v0;  // [bp-0x68]
    unsigned int v1;  // [bp-0x54]
    unsigned long long v2;  // [bp-0x50]
    unsigned long long v3;  // [bp-0x40]
    char *v4;  // [bp-0x20]
    unsigned long long v5;  // [bp-0x18]

    v3 = a2;
    v2 = a4;
    v1 = a5;
    *(__errno_location()) = 0;
    v5 = strtoumax(a1, &v4, 10, &v4);
    if (!*(v4) && !*(__errno_location()))
    {
        if (v5 > a3)
        {
            v0 = a1;
            error(0, 0, gettext("%s:%u: %s out of range: %s"));
            return 0xffffffff;
        }
        *(a0) = v5;
        return 0;
    }
    v0 = a1;
    error(0, 0, gettext("%s:%u: invalid %s: %s"));
    return 0xffffffff;
}



// Function: map_read @ 0x28bbb
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;

long long map_read(long long *a0, unsigned long a1, unsigned long long *a2, unsigned long long a3, unsigned long long a4)
{
    char *v19;  // rax
    char *v20;  // rax
    unsigned long long v21;  // rdx
    unsigned long v24;  // fs
    unsigned long v0;  // [bp-0x1a8]
    unsigned int v1;  // [bp-0x15c]
    unsigned int v2;  // [bp-0x158]
    unsigned int v3;  // [bp-0x154]
    char *v4;  // [bp-0x150]
    unsigned long v5;  // [bp-0x148]
    unsigned long long v6;  // [bp-0x140]
    unsigned long long v7;  // [bp-0x138]
    unsigned long v8;  // [bp-0x130]
    FILE *fp;  // [bp-0x128]
    unsigned long v10;  // [bp-0x120]
    unsigned long ptr;  // [bp-0x118]
    unsigned long long *index;  // [bp-0x110]
    char v13;  // [bp-0x108]
    unsigned long long *idx;  // [bp-0x100]
    unsigned long long v15;  // [bp-0xc8]
    unsigned long v16;  // [bp-0x20]

    v4 = NULL;
    v5 = 0;
    v3 = 0;
    fp = fopen(a1, "r");
    if (!fp)
        sub_43e3b4(a1); /* do not return */
    v15 = "#";
    v1 = 36420;
    v2 = 0;
    do
    {
        while (1)
        {
            v10 = getline(&v4, &v5, fp);
            if (v10 <= NULL)
            {
                if ((char)v1 & 8)
                    sub_446399(&v13);
                fclose(fp);
                if (!v3)
                    return v16 - *((long long *)(40 + v24));
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("errors reading map file"));
                sub_4177f4(); /* do not return */
            }
            v8 = 0;
            v2 += 1;
            if (sub_4461a6(v4, &v13, v1, &v13))
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_44647f(&v13);
                error(0, 0, gettext("%s:%u: cannot split line: %s"));
                sub_4177f4(); /* do not return */
            }
            v1 |= 8;
            switch (v13)
            {
            case 0:
                break;
            case 2:
                if (*((char *)*(idx)) == 43)
                {
                    if (sub_428a83(&v6, *(idx) + 1, a3, a4, a1, v2))
                    {
                        v3 = 1;
                        continue;
                    }
                }
                else
                {
                    if (a2 && (v6 = (unsigned long long)a2(*(idx)), v6 == 0xffffffffffffffff))
                    {
                        v19 = gettext("%s:%u: can't obtain %s of %s");
                        v0 = *(idx);
                        error(0, 0, v19);
                        v3 = 1;
                        continue;
                    }
                }
                ptr = strchr(idx[1], 58);
                if (ptr)
                {
                    if (ptr > idx[1])
                        v8 = idx[1];
                    ptr += 1;
                    *((char *)ptr) = 0;
                    if (sub_428a83(&v7, ptr, a3, a4, a1, v2))
                    {
                        v3 = 1;
                        continue;
                    }
                    goto LABEL_42905b;
                }
                if (*((char *)idx[1]) == 43)
                {
                    if (sub_428a83(&v7, idx[1], a3, a4, a1, v2))
                    {
                        v3 = 1;
                        continue;
                    }
                    goto LABEL_42905b;
                }
                else
                {
                    v8 = idx[1];
                    v7 = a2(idx[1]);
                    if (v7 == 0xffffffffffffffff)
                    {
                        v20 = gettext("%s:%u: can't obtain %s of %s");
                        v0 = idx[1];
                        error(0, 0, v20);
                        v3 = 1;
                        break;
                    }
                    else
                    {
LABEL_42905b:
                        index = sub_460b5b(24);
                        *(index) = v6;
                        index[1] = v7;
                        v21 = (!v8 ? 0 : sub_460cc6(v8));
                    }
                }
            default:
                error(0, 0, gettext("%s:%u: malformed line"));
                v3 = 1;
                continue;
            }
        }
    } while ((index[2] = v21, (*(a0) || (*(a0) = (long long)(unsigned long long)sub_452c1c(0, 0, sub_428a20, sub_428a4d, 0), *(a0))) && (unsigned long long)sub_45395d(*(a0), index, index)));
    sub_41783c(); /* do not return */
}



// Function: name_to_uid @ 0x291f9
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
    passwd *v0;  // [bp-0x10]

    v0 = getpwnam(a0);
    return (!v0 ? 0xffffffffffffffff : v0->pw_uid);
}



// Function: owner_map_read @ 0x29234
extern long long g_49f5e8;

long long owner_map_read(unsigned int a0)
{
    return sub_428bbb(&g_49f5e8, a0, sub_4291f9, "UID", 0xffffffff);
}



// Function: owner_map_translate @ 0x29274
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern long long g_49f5e8;
extern unsigned long long g_49f940;
extern unsigned int g_49f948;

unsigned int owner_map_translate(unsigned int a0, unsigned int *a1, unsigned long long *a2)
{
    unsigned int v0;  // [bp-0x34]
    struct_0 *v1;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28]

    v0 = 1;
    if (g_49f5e8)
    {
        v2 = a0;
        v1 = sub_452567(g_49f5e8, &v2, &v2);
        if (v1)
        {
            *(a1) = v1->field_8;
            *(a2) = v1->field_10;
            return 0;
        }
    }
    if (g_49f948 != 0xffffffff)
    {
        *(a1) = g_49f948;
        v0 = 0;
    }
    if (g_49f940)
    {
        *(a2) = g_49f940;
        v0 = 0;
    }
    return v0;
}



// Function: name_to_gid @ 0x29353
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

unsigned long long name_to_gid(char *a0)
{
    group *v0;  // [bp-0x10]

    v0 = getgrnam(a0);
    return (!v0 ? 0xffffffffffffffff : v0->gr_gid);
}



// Function: group_map_read @ 0x2938e
extern unsigned long long g_48295e;
extern long long g_49f5f0;

long long group_map_read(unsigned int a0)
{
    return sub_428bbb(&g_49f5f0, a0, sub_429353, &g_48295e, 0xffffffff);
}



// Function: group_map_translate @ 0x293ce
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

extern long long g_49f5f0;
extern unsigned long long g_49f8b0;
extern unsigned int g_49f8b8;

unsigned int group_map_translate(unsigned int a0, unsigned int *a1, unsigned long long *a2)
{
    unsigned int v0;  // [bp-0x34]
    struct_0 *v1;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28]

    v0 = 1;
    if (g_49f5f0)
    {
        v2 = a0;
        v1 = sub_452567(g_49f5f0, &v2, &v2);
        if (v1)
        {
            *(a1) = v1->field_8;
            *(a2) = v1->field_10;
            return 0;
        }
    }
    if (g_49f8b8 != 0xffffffff)
    {
        *(a1) = g_49f8b8;
        v0 = 0;
    }
    if (g_49f8b0)
    {
        *(a2) = g_49f8b0;
        v0 = 0;
    }
    return v0;
}



// Function: quote_n_colon @ 0x29516
int quote_n_colon(unsigned int a0, long long a1)
{
    unsigned int v1;  // eax

    v1 = sub_45c97e(0);
    return sub_45e599(a0, v1, a1, v1);
}



// Function: assign_string @ 0x29547
void* * assign_string(void* *a0, long long a1)
{
    free(*(a0));
    *(a0) = (!a1 ? 0 : sub_460cc6(a1));
    return a0;
}



// Function: assign_string_n @ 0x29591
void* * assign_string_n(void* *a0, void* a1, unsigned long a2)
{
    void* *v3;  // rax
    unsigned long len;  // [bp-0x18]
    void* v1;  // [bp-0x10]

    v3 = (unsigned long long)free(*(a0));
    if (!a1)
        return v3;
    len = strnlen(a1, a2);
    v1 = sub_460b5b(len + 1);
    memcpy(v1, a1, len);
    *(len + (char *)v1) = 0;
    *(a0) = v1;
    return a0;
}



// Function: unquote_string @ 0x2961d

unsigned int unquote_string(char *a0)
{
    char *v7;  // rax
    char *v8;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    char *v21;  // rax
    char *v22;  // rax
    char *v23;  // rdx
    char *v24;  // rax
    char *v9;  // rax
    char *v10;  // rax
    char *v11;  // rax
    char *v12;  // rax
    char *v13;  // rax
    char *v14;  // rax
    char *v15;  // rax
    char *v16;  // rax
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]
    char *iter;  // [bp-0x18]
    char *v3;  // [bp-0x18]
    char *node;  // [bp-0x10]
    char *v5;  // [bp-0x10]

    v0 = 1;
    iter = a0;
    node = a0;
    while (*(iter))
    {
        if (*(iter) == 92)
        {
            iter += 1;
            switch (*(iter))
            {
            case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55:
                v16 = iter;
                iter = v16 + 1;
                v1 = *(v16) - 48;
                if (*(iter) <= 47 || *(iter) > 55)
                {
                    v21 = node;
                    node = v21 + 1;
                    *(v21) = v1;
                    break;
                }
                else
                {
                    v17 = iter;
                    iter = v17 + 1;
                    v1 = *(v17) + v1 * 8 - 48;
                    if (*(iter) <= 47 || *(iter) > 55)
                    {
                        v20 = node;
                        node = v20 + 1;
                        *(v20) = v1;
                        break;
                    }
                    else
                    {
                        v18 = iter;
                        iter = v18 + 1;
                        v1 = *(v18) + v1 * 8 - 48;
                        v19 = node;
                        node = v19 + 1;
                        *(v19) = v1;
                        break;
                    }
                }
            case 63:
                v15 = node;
                node = v15 + 1;
                *(v15) = 127;
                iter += 1;
                break;
            case 92:
                v7 = node;
                node = v7 + 1;
                *(v7) = 92;
                iter += 1;
                break;
            case 97:
                v8 = node;
                node = v8 + 1;
                *(v8) = 7;
                iter += 1;
                break;
            case 98:
                v9 = node;
                node = v9 + 1;
                *(v9) = 8;
                iter += 1;
                break;
            case 102:
                v10 = node;
                node = v10 + 1;
                *(v10) = 12;
                iter += 1;
                break;
            case 110:
                v11 = node;
                node = v11 + 1;
                *(v11) = 10;
                iter += 1;
                break;
            case 114:
                v12 = node;
                node = v12 + 1;
                *(v12) = 13;
                iter += 1;
                break;
            case 116:
                v13 = node;
                node = v13 + 1;
                *(v13) = 9;
                iter += 1;
                break;
            case 118:
                v14 = node;
                node = v14 + 1;
                *(v14) = 11;
                iter += 1;
                break;
            default:
                v0 = 0;
                v22 = node;
                node = v22 + 1;
                *(v22) = 92;
                if (*(iter))
                {
                    v23 = iter;
                    iter = v23 + 1;
                    v5 = node + 1;
                    *(node) = *(v23);
                    node = v5;
                }
                break;
            }
        }
        else if (iter != node)
        {
            v3 = iter + 1;
            v24 = node;
            node = v24 + 1;
            *(v24) = *(iter);
            iter = v3;
        }
        else
        {
            iter += 1;
            node += 1;
        }
    }
    if (iter != node)
        *(node) = 0;
    return v0;
}



// Function: zap_slashes @ 0x298d7
char * zap_slashes(char *a0)
{
    char *i;  // [bp-0x10]
    char *v1;  // [bp-0x10]

    if (a0 && *(a0))
    {
        for (i = a0 + strlen(a0) - 1; i > a0 && *(i) == 47; i = v1)
        {
            v1 = i - 1;
            *(i) = 0;
        }
        return a0;
    }
    return a0;
}



// Function: normalize_filename_x @ 0x29946
void* normalize_filename_x(void* a0)
{
    void* v4;  // rax
    int v5;  // eax
    int v6;  // eax
    char v0;  // [bp-0x21]
    void* iter;  // [bp-0x20]
    void* node;  // [bp-0x18]

    iter = a0;
    node = iter;
    while (1)
    {
        *((char *)iter) = *((char *)node);
        if (*((char *)iter) != 46 || (char)node[1] != 47)
            break;
        for (node += 2; *((char *)node) == 47; node += 1);
        iter += !*((char *)node);
    }
    while (1)
    {
        v4 = node;
        node = v4 + 1;
        v0 = *((char *)v4);
        iter += 1;
        *((char *)iter) = v0;
        if (!*((char *)iter))
            break;
        iter = iter;
        if (v0 != 47)
            continue;
        while (1)
        {
            if (*((char *)node + (*((char *)node) == 46)) != 47)
                break;
            node += (*((char *)node) == 46 ? 2 : 1);
        }
    }
    if (iter - a0 <= 2)
        return iter - a0;
    if (*((char *)iter - 2) == 46 && *((char *)iter - 3) == 47)
        v5 = 1;
    else
        v5 = 0;
    iter += -(v5);
    if (iter - a0 > 2 && *((char *)iter - 2) == 47)
        v6 = 1;
    else
        v6 = 0;
    iter += -(v6);
    *((char *)iter - 1) = 0;
    return iter - 1;
}



// Function: normalize_filename @ 0x29aac
void* normalize_filename(unsigned int a0, char *a1)
{
    char v0;  // [bp-0x31]
    void* v1;  // [bp-0x30]
    char *ptr;  // [bp-0x28]
    unsigned long len;  // [bp-0x20]

    v1 = NULL;
    if (*(a1) != 47)
    {
        ptr = sub_42afed(a0);
        len = strlen(ptr);
        v0 = 1;
        v1 = sub_460b5b(strlen(a1) + v0 + len + 1);
        strcpy(v1, ptr);
        *(len + (char *)v1) = 47;
        strcpy(v0 + len + v1, a1);
    }
    if (v1)
    {
        sub_429946(v1);
        return v1;
    }
    v1 = sub_460cc6(a1);
    sub_429946(v1);
    return v1;
}



// Function: replace_prefix @ 0x29b93
long long replace_prefix(unsigned long long *a0, void* a1, unsigned long a2, void* a3, unsigned long a4)
{
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    unsigned long v0;  // [bp-0x18]
    unsigned long len;  // [bp-0x10]

    v0 = *(a0);
    len = strlen(v0);
    if (len <= a2)
        return len;
    v3 = memcmp(v0, a1, a2);
    if ((unsigned int)v3)
        return v3;
    v4 = *((char *)(a2 + v0));
    if ((char)v4 != 47)
        return v4;
    if (a4 > a2)
    {
        v0 = sub_460b8d(v0, a4 + len - a2 + 1, a4 + len - a2 + 1);
        *(a0) = v0;
    }
    memmove(a4 + v0, v0 + a2, len - a2 + 1);
    return memcpy(v0, a3, a4);
}



// Function: sysinttostr @ 0x29c96
long long sysinttostr(long long a0, unsigned long a1, long long a2, long long a3)
{
    unsigned long long v0;  // [bp-0x10]

    if (a0 > a2)
    {
        v0 = a0 - a1;
        return sub_454bde(v0 + a1, a3, v0 + a1);
    }
    return sub_454e5a(a0, a3, a3);
}



// Function: strtosysint @ 0x29cfc
long long strtosysint(char *a0, long long a1, long long a2, long long a3)
{
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    *(__errno_location()) = 0;
    if (a3 >= 0)
    {
        if (a0[*(a0) == 45] - 48 <= 9)
        {
            v1 = strtoimax(a0, a1, 10, a1);
            if (a2 <= v1 && v1 <= a3)
                return v1;
            *(__errno_location()) = 0x22;
            return (v1 < a2 ? a2 : a3);
        }
    }
    else
    {
        if (*(a0) - 48 <= 9)
        {
            v0 = strtoumax(a0, a1, 10, a1);
            if (v0 > a3)
            {
                *(__errno_location()) = 0x22;
                return a3;
            }
            return sub_4294ad(v0);
        }
    }
    *(__errno_location()) = 22;
    return 0;
}



// Function: code_ns_fraction @ 0x29e23
char * code_ns_fraction(unsigned int a0, char *a1)
{
    char *v0;  // [bp-0x28]
    char *v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x1c]
    int iter;  // [bp-0xc]

    v2 = a0;
    v0 = a1;
    if (!v2)
    {
        *(v0) = 0;
        return v0;
    }
    iter = 9;
    v1 = v0 + 1;
    for (*(v0) = 46; !(v2 % 10); iter -= 1)
    {
        v2 /= 10;
    }
    v1[iter] = 0;
    while (1)
    {
        iter -= 1;
        v1[iter] = (char)(v2 % 10) + 48;
        if (!iter)
            break;
        v2 /= 10;
    }
    return &v1[iter];
}



// Function: code_timespec @ 0x29f2e
void* code_timespec(unsigned long a0, unsigned int a1, unsigned long a2)
{
    char v0;  // [bp-0x2d]
    unsigned int v1;  // [bp-0x2c]
    unsigned long v2;  // [bp-0x28]
    void* iter;  // [bp-0x20]

    v2 = a0;
    v1 = a1;
    v0 = v2 >> 63;
    if (v1 > 0x3b9ac9ff || v1 < 0)
        v1 = 0;
    if (v0 && v1)
    {
        v2 += 1;
        v1 = 1000000000 - v1;
    }
    if (v0)
        v2 = -(v2);
    iter = sub_454e5a(v2, a2 + 1);
    if (!v0)
    {
        sub_429e23(v1, a2 + 21);
        return iter;
    }
    iter -= 1;
    *((char *)iter) = 45;
    sub_429e23(v1, a2 + 21);
    return iter;
}



// Function: decode_timespec @ 0x29ff7
typedef struct struct_0 {
    char field_0;
} struct_0;

long long decode_timespec(char *a0, struct_0 **a1, char a2)
{
    char v0;  // [bp-0x42]
    char v1;  // [bp-0x41]
    unsigned int i;  // [bp-0x40]
    int iter;  // [bp-0x3c]
    long long v4;  // [bp-0x38]
    char *v5;  // [bp-0x30]
    long long v6;  // [bp-0x28]
    long long v7;  // [bp-0x20]
    long long v8;  // [bp-0x18]

    v4 = -0x8000000000000000;
    i = 0xffffffff;
    v5 = a0;
    v1 = *(a0) == 45;
    if (a0[v1] - 48 > 9)
    {
        *(__errno_location()) = 22;
    }
    else
    {
        *(__errno_location()) = 0;
        if (v1)
        {
            v7 = strtoimax(a0, a1, 10, a1);
            v4 = v7;
        }
        else
        {
            v6 = strtoumax(a0, a1, 10, a1);
            if (v6 >= 0)
                v4 = v6;
            else
                *(__errno_location()) = 0x22;
        }
        v5 = &*(a1)->field_0;
        i = 0;
        if (a2 && *(v5) == 46)
        {
            iter = 0;
            v0 = 0;
            while (1)
            {
                v5 += 1;
                if (*(v5) - 48 > 9)
                    break;
                if (iter <= 8)
                {
                    iter += 1;
                    i = *(v5) - 48 + i * 10;
                }
                else
                {
                    v0 = *(v5) != 48 || v0;
                }
            }
            for (; iter <= 8; i *= 10)
            {
                iter += 1;
            }
            if (v1)
            {
                i += v0;
                if (i)
                {
                    if (v4 == -0x8000000000000000)
                    {
                        i = 0xffffffff;
                    }
                    else
                    {
                        v4 -= 1;
                        i = 1000000000 - i;
                    }
                }
            }
        }
        if (*(__errno_location()) == 0x22)
            i = 0xffffffff;
    }
    *(a1) = v5;
    v8 = v4;
    return v8;
}



// Function: must_be_dot_or_slash @ 0x2a1fe
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

long long must_be_dot_or_slash(struct_0 *a0)
{
    unsigned long v2;  // rax
    unsigned int v4;  // eax
    struct_0 *iter;  // [bp-0x10]

    if (iter->field_0 == 47)
    {
        while (1)
        {
            for (iter = a0; iter->field_1 == 47; iter = &iter->field_1);
            if (iter->field_1 != 46 || (&iter->field_0)[(iter->field_2 == 46 ? 3 : 2)] != 47)
                break;
            iter = &(&iter->field_0)[(iter->field_2 == 46 ? 3 : 2)];
        }
        v2 = iter->field_1;
        return _INSERT(v2, 0, !(char)v2);
    }
    else
    {
        while (iter->field_0 == 46 && iter->field_1 == 47)
        {
            for (iter = &iter->field_2; iter->field_0 == 47; iter = &iter->field_1);
        }
        if (!iter->field_0 || iter->field_0 == 46 && !iter->field_1)
            v4 = 1;
        else
            v4 = 0;
        return v4 & 1;
    }
}



// Function: safer_rmdir @ 0x2a306
extern unsigned int g_49c268;

unsigned int safer_rmdir(unsigned long a0)
{
    if ((char)sub_42a1fe(a0))
    {
        *(__errno_location()) = 0;
        return 0xffffffff;
    }
    else if (!unlinkat(g_49c268, a0, 0x200, a0))
    {
        sub_419454(a0);
        return 0;
    }
    else
    {
        return 0xffffffff;
    }
}



// Function: remove_any_file @ 0x2a36f
extern unsigned int g_49c268;

unsigned int remove_any_file(long long a0, unsigned int a1)
{
    unsigned int v8;  // eax
    char v0;  // [bp-0x31]
    unsigned int v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x2c]
    unsigned long iter;  // [bp-0x28]
    unsigned long ptr;  // [bp-0x20]
    unsigned long len;  // [bp-0x18]
    unsigned long v6;  // [bp-0x10]

    v0 = 1;
    if (v0)
    {
        if (!unlinkat(g_49c268, a0, 0, a0))
        {
            return 1;
        }
        else if (*(__errno_location()) != 1 && *(__errno_location()) != 21)
        {
            return 0;
        }
    }
    if (!sub_42a306(a0))
        return 1;
    v8 = *(__errno_location());
    switch (v8)
    {
    case 0: case 17: case 39:
        if (a1 != 2)
        {
            if (a1 > 2 || !a1 || a1 != 1)
                return 0;
            ptr = sub_42b8e1(a0, 0);
            if (!ptr)
                return 0;
            iter = ptr;
            while (1)
            {
                len = strlen(iter);
                if (!len)
                {
                    free(ptr);
                    return !sub_42a306(a0);
                }
                v6 = sub_42ee85(a0, iter, iter);
                v1 = remove_any_file(v6, 1);
                v2 = *(__errno_location());
                free(v6);
                if (!v1)
                {
                    free(ptr);
                    *(__errno_location()) = v2;
                    return 0;
                }
                iter += len + 1;
            }
            return 0;
        }
        else
        {
            return 0xffffffff;
        }
    case 20:
        if (v0 == 1 || unlinkat(g_49c268, a0, 0, a0))
            return 0;
        return 1;
        return 0;
    default:
        return 0;
    }
}



// Function: maybe_backup_file @ 0x2a571
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern unsigned int g_49c268;
extern FILE *g_49efd0;
extern long long g_49f608;
extern long long g_49f610;
extern unsigned int g_49f888;
extern unsigned int g_49f9ac;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;
extern unsigned long long g_49fdf0;
extern char g_49fdf8;

unsigned int maybe_backup_file(unsigned long a0, char a1)
{
    char *v4;  // rax
    unsigned int v0;  // [bp-0xbc]
    char v1;  // [bp-0xb8]
    unsigned int v2;  // [bp-0xa0]

    sub_429547(&g_49f608, a0);
    sub_429547(&g_49f610, 0);
    if (a1 && g_49fdf8 != 1)
    {
        g_49fdf0 = strchr(a0, 58);
        if (g_49fdf0 && a0 < g_49fdf0 && !memchr(a0, 47, g_49fdf0 - a0))
            return 1;
    }
    if (sub_42a9b8(a0, &v1, &v1))
    {
        if (*(__errno_location()) == 2)
            return 1;
        sub_43ea03(a0);
        return 0;
    }
    else
    {
        if (((unsigned short)v2 & 0xf000) == 0x4000)
            return 1;
        if (a1 && (((unsigned short)v2 & 0xf000) == 0x6000 || ((unsigned short)v2 & 0xf000) == 0x2000))
            return 1;
        g_49f610 = sub_44de0e(g_49c268, a0, g_49f888, a0);
        if (!g_49f610)
        {
            sub_41783c(); /* do not return */
        }
        else if (!renameat(g_49c268, g_49f608, g_49c268, g_49f610))
        {
            if (!g_49f9ac)
                return 1;
            sub_45e821(1, g_49f610);
            sub_45e821(0, g_49f608);
            v4 = gettext("Renaming %s to %s\n");
            fprintf(g_49efd0, v4);
            return 1;
        }
        else
        {
            v0 = *(__errno_location());
            if (g_49fdc8)
                g_49fdc8();
            sub_45e821(1, g_49f610);
            sub_45e548(g_49f608);
            error(0, v0, gettext("%s: Cannot rename to %s"));
            g_49fdd0 = 2;
            sub_429547(&g_49f610, 0);
            return 0;
        }
    }
}



// Function: undo_last_backup @ 0x2a874
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern unsigned int g_49c268;
extern FILE *g_49efd0;
extern long long g_49f608;
extern long long g_49f610;
extern unsigned int g_49f9ac;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

long long undo_last_backup(void)
{
    char *v3;  // rax
    unsigned int v0;  // [bp-0x1c]

    if (!g_49f610)
        return g_49f610;
    if (renameat(g_49c268, g_49f610, g_49c268, g_49f608))
    {
        v0 = *(__errno_location());
        if (g_49fdc8)
            g_49fdc8();
        sub_45e821(1, g_49f608);
        sub_45e548(g_49f610);
        error(0, v0, gettext("%s: Cannot rename to %s"));
        g_49fdd0 = 2;
    }
    if (!g_49f9ac)
        return sub_429547(&g_49f610, 0);
    sub_45e821(1, g_49f608);
    sub_45e821(0, g_49f610);
    v3 = gettext("Renaming %s back to %s\n");
    fprintf(g_49efd0, v3);
    return sub_429547(&g_49f610, 0);
}



// Function: deref_stat @ 0x2a9b8
extern unsigned int g_49c268;
extern unsigned int g_49fc10;

int deref_stat(long long a0, long long a1)
{
    return fstatat(g_49c268, a0, a1, g_49fc10);
}



// Function: blocking_read @ 0x2a9e9
unsigned long long blocking_read(unsigned int a0, long long a1, long long a2)
{
    unsigned int v0;  // [bp-0x14]
    unsigned long long v1;  // [bp-0x10]

    v1 = sub_45ec70(a0, a1, a2, a1);
    if (v1 != 0xffffffffffffffff)
    {
        return v1;
    }
    else if (*(__errno_location()) == 11)
    {
        v0 = sub_4617eb(a0, 3);
        if (v0 < 0)
        {
            return v1;
        }
        else if (!((unsigned short)v0 & 0x800))
        {
            return v1;
        }
        else if (sub_4617eb(a0, 4, _INSERT(v0, 1, (char)v0 & 247) & 0xffffffff) != 0xffffffff)
        {
            v1 = sub_45ec70(a0, a1, a2, a1);
            return v1;
        }
        else
        {
            return v1;
        }
    }
    else
    {
        return v1;
    }
}



// Function: blocking_write @ 0x2aa95
long long blocking_write(unsigned int a0, long long a1, long long a2)
{
    unsigned int v0;  // [bp-0x1c]
    long long v1;  // [bp-0x18]

    v1 = sub_452175(a0, a1, a2, a1);
    if (v1 >= a2)
    {
        return v1;
    }
    else if (*(__errno_location()) == 11)
    {
        v0 = sub_4617eb(a0, 3);
        if (v0 < 0)
        {
            return v1;
        }
        else if (!((unsigned short)v0 & 0x800))
        {
            return v1;
        }
        else if (sub_4617eb(a0, 4, _INSERT(v0, 1, (char)v0 & 247) & 0xffffffff) != 0xffffffff)
        {
            v1 += sub_452175(a0, a1 + v1, a2 - v1, a1 + v1);
            return v1;
        }
        else
        {
            return v1;
        }
    }
    else
    {
        return v1;
    }
}



// Function: set_file_atime @ 0x2ab5e
extern unsigned int g_49fc10;

int set_file_atime(unsigned int a0, unsigned int a1, long long a2, unsigned long a3, unsigned long long a4)
{
    unsigned long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x20]

    v0 = a3;
    v1 = a4;
    v2 = 0x3ffffffe;
    return sub_44f4ec(a0, a1, a2, &v0, g_49fc10);
}



// Function: chdir_count @ 0x2abe0
extern unsigned long long g_49f620;

unsigned long long chdir_count(void)
{
    return (!g_49f620 ? g_49f620 : (unsigned int)g_49f620 - 1);
}



// Function: chdir_arg @ 0x2ac09
extern long long g_49f618;
extern unsigned long long g_49f620;
extern unsigned long long g_49f628;

unsigned long long chdir_arg(char *a0)
{
    unsigned long long v2;  // rax
    char *iter;  // [bp-0x10]

    iter = a0;
    if (g_49f620 == g_49f628)
    {
        if (!g_49f628)
            g_49f628 = 2;
        g_49f618 = sub_460a5f(g_49f618, &g_49f628, 24, &g_49f628);
        if (!g_49f620)
        {
            *((char **)(g_49f620 * 24 + g_49f618)) = ".";
            *((unsigned long *)(g_49f620 * 24 + g_49f618 + 8)) = 0;
            *((unsigned int *)(g_49f620 * 24 + g_49f618 + 16)) = 0xffffff9c;
            g_49f620 = g_49f620 + 1;
        }
    }
    if (*(iter))
    {
        while (*(iter) == 46 && iter[1] == 47)
        {
            for (iter += 2; *(iter) == 47; iter += 1);
        }
        if (!iter[*(iter) == 46])
            return (unsigned int)g_49f620 - 1;
    }
    *((char **)(g_49f618 + g_49f620 * 24)) = iter;
    *((unsigned long *)(g_49f620 * 24 + g_49f618 + 8)) = 0;
    *((unsigned int *)(g_49f620 * 24 + g_49f618 + 16)) = 0;
    v2 = g_49f620;
    g_49f620 = v2 + 1;
    return v2;
}



// Function: chdir_do @ 0x2adf1
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

extern unsigned int g_49c268;
extern unsigned int g_49f600;
extern unsigned long long g_49f618;
extern unsigned int g_49f640[4];
extern int g_49f67c;
extern unsigned long long g_49f680;
extern unsigned int g_49fc0c;

long long chdir_do(unsigned int *a0)
{
    unsigned long v7;  // rax
    unsigned long long idx;  // rax
    int v0;  // [bp-0x2c]
    unsigned int *v1;  // [bp-0x28]
    unsigned int *v2;  // [bp-0x24]
    unsigned long long index;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    struct_2 *v5;  // [bp-0x10]

    v7 = g_49f600;
    if (a0 == (unsigned int)v7)
        return v7;
    idx1 = (long long)a0 * 24 + g_49f618;
    v0 = idx1->field_10;
    if (!v0)
    {
        if (idx1->field_0->field_0 != 47)
            chdir_do((char *)a0 - 1);
        v0 = openat(g_49c268, idx1->field_0, g_49fc0c & 0xfffdffff, idx1->field_0);
        if (v0 < 0)
            sub_43e3b4(idx1->field_0); /* do not return */
        idx1->field_10 = v0;
        if (g_49f680 <= 15)
        {
            idx = g_49f680;
            g_49f680 = idx + 1;
            g_49f640[idx] = (unsigned int)a0;
        }
        else
        {
            v5 = g_49f67c * 24 + g_49f618;
            if (close(v5->field_10))
                sub_42b26a(v5->field_0);
            v5->field_10 = 0;
            g_49f67c = (unsigned int)a0;
        }
    }
    if (v0 > 0)
    {
        v1 = (unsigned int *)(int)g_49f640;
        for (index = 1; v1 != a0 && (v2 = (unsigned int *)g_49f640[index], g_49f640[index] = (unsigned int)v1, v2 != a0); index += 1)
        {
            v1 = v2;
        }
        g_49f640[0] = (unsigned int)a0;
    }
    g_49f600 = (unsigned int)a0;
    g_49c268 = v0;
    return v0;
}



// Function: tar_dirname @ 0x2afc0
extern int g_49f600;
extern unsigned long long g_49f618;

long long tar_dirname(void)
{
    return *((long long *)(g_49f600 * 24 + g_49f618));
}



// Function: tar_getcdpath @ 0x2afed
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f600;
extern long long g_49f618;
extern unsigned long long g_49f688;

unsigned long long tar_getcdpath(unsigned int a0)
{
    struct_0 *v4;  // rbx
    unsigned long v5;  // rcx
    unsigned long long v6;  // rax
    unsigned long long v7;  // rax
    unsigned int iter;  // [bp-0x28]
    unsigned int v1;  // [bp-0x24]
    long long v2;  // [bp-0x20]

    if (!g_49f618)
    {
        if (g_49f688)
            return g_49f688;
        g_49f688 = sub_460cf7();
        if (!g_49f688)
            sub_43dfc0("getcwd", "."); /* do not return */
        return g_49f688;
    }
    else
    {
        if (*((long long *)(a0 * 24 + g_49f618 + 8)))
            return *((long long *)(a0 * 24 + g_49f618 + 8));
        v1 = g_49f600;
        for (iter = a0; iter >= 0 && !*((long long *)(iter * 24 + g_49f618 + 8)); iter -= 1);
        while (1)
        {
            iter += 1;
            if (iter > a0)
                break;
            sub_42adf1(iter);
            if (!iter)
            {
                v4 = g_49f618 + iter * 24;
                v4->field_8 = sub_460cf7();
                if (!v4->field_8)
                    sub_43dfc0("getcwd", "."); /* do not return */
            }
            else if (*((char *)*((long long *)(iter * 24 + g_49f618))) == 47)
            {
                v5 = iter;
                v6 = sub_460cc6(*((long long *)(iter * 24 + g_49f618)), g_49f618, *((long long *)(iter * 24 + g_49f618)), v5);
                *((unsigned long long *)(g_49f618 + v5 * 24 + 8)) = v6;
            }
            else
            {
                v2 = sub_42b66c(*((long long *)(iter * 24 - 24 + g_49f618 + 8)));
                sub_42b7ec(v2, *((long long *)(iter * 24 + g_49f618)), *((long long *)(iter * 24 + g_49f618)), g_49f618);
                v7 = sub_42b882(v2);
                *((unsigned long long *)(g_49f618 + iter * 24 + 8)) = v7;
            }
        }
        sub_42adf1(v1);
        return *((long long *)(a0 * 24 + g_49f618 + 8));
    }
}



// Function: close_diag @ 0x2b26a
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long close_diag(long long a0)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43e1ff(a0);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43e228(a0);
}



// Function: open_diag @ 0x2b2b1
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long open_diag(long long a0)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43e38b(a0);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43e3da(a0);
}



// Function: read_diag_details @ 0x2b2f8
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long read_diag_details(long long a0, long long a1, long long a2)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43e42c(a0, a1, a2, a1);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43e506(a0, a1, a2, a1);
}



// Function: readlink_diag @ 0x2b35d
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long readlink_diag(long long a0)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43e6b3(a0);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43e6dc(a0);
}



// Function: savedir_diag @ 0x2b3a4
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long savedir_diag(long long a0)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43e72e(a0);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43e757(a0);
}



// Function: seek_diag_details @ 0x2b3eb
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long seek_diag_details(long long a0, long long a1)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43e7a9(a0, a1, a1);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43e890(a0, a1, a1);
}



// Function: stat_diag @ 0x2b444
extern unsigned int g_49ec84;
extern char g_49f8bc;
extern char g_800000;

unsigned long long stat_diag(long long a0)
{
    unsigned long long v1;  // rax

    if (!g_49f8bc)
        return sub_43ea03(a0);
    v1 = g_49ec84 & &g_800000;
    if (!(unsigned int)v1)
        return v1;
    return sub_43ea2c(a0);
}



// Function: file_removed_diag @ 0x2b48b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern struct_0 *g_49fdc8;

long long file_removed_diag(long long a0, char a1, unsigned long long *a2)
{
    if (a1 != 1 && *(__errno_location()) == 2)
    {
        if (!((char)g_49ec84 & 64))
            return sub_439e98(1);
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(a0);
        error(0, 0, gettext("%s: File removed before we read it"));
        return sub_439e98(1);
    }
    return a2(a0);
}



// Function: xfork @ 0x2b537
int xfork(void)
{
    unsigned int v0;  // [bp-0xc]

    v0 = fork();
    if (v0 != 0xffffffff)
        return v0;
    sub_43dfc0("fork", gettext("child process")); /* do not return */
}



// Function: xpipe @ 0x2b577
int xpipe(int a0[2])
{
    unsigned int v1;  // eax

    v1 = pipe(a0);
    if (v1 >= 0)
        return v1;
    sub_43dfc0("pipe", gettext("interprocess channel")); /* do not return */
}



// Function: ptr_align @ 0x2b5bb
long long ptr_align(unsigned long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0 + a1 - 1;
    return v0 + -(v0 % a1);
}



// Function: page_aligned_alloc @ 0x2b601
long long page_aligned_alloc(unsigned int *a0, long long a1)
{
    unsigned long v0;  // [bp-0x18]
    long long v1;  // [bp-0x10]

    v0 = (int)getpagesize();
    v1 = v0 + a1;
    if (v1 >= a1)
    {
        *((unsigned long long *)&a0) = sub_460b5b(v1);
        return sub_42b5bb(*((long long *)&a0), v0);
    }
    sub_41783c(); /* do not return */
}



// Function: namebuf_create @ 0x2b66c
typedef struct struct_0 {
    char *field_0;
    char field_8;
    char padding_9[7];
    char field_10;
} struct_0;

struct_0 * namebuf_create(char *a0)
{
    char *v2;  // rsi
    unsigned long v3;  // rax
    struct_0 *idx;  // [bp-0x10]

    idx = sub_460b5b(24);
    *((unsigned long long *)&idx->field_8) = strlen(a0) + 2;
    idx->field_0 = sub_460b5b(*((long long *)&idx->field_8));
    strcpy(idx->field_0, a0);
    *((unsigned long *)&idx->field_10) = strlen(idx->field_0);
    if (*(&idx->field_0[*((long long *)&idx->field_10)] - 1) != 47)
    {
        v2 = idx->field_0;
        v3 = *((long long *)&idx->field_10);
        *((unsigned long long *)&idx->field_10) = v3 + 1;
        v2[v3] = 47;
    }
    return idx;
}



// Function: namebuf_free @ 0x2b72d
void namebuf_free(void* *a0)
{
    free(*(a0));
    free(a0);
    return;
}



// Function: namebuf_name @ 0x2b75b
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

long long namebuf_name(struct_0 *a0, char *ptr)
{
    unsigned long len;  // [bp-0x10]

    for (len = strlen(ptr); len + a0->field_10 + 1 >= a0->field_8; a0->field_0 = sub_460bd8(a0->field_0, &a0->field_8, &a0->field_8));
    strcpy(a0->field_0 + a0->field_10, ptr);
    return a0->field_0;
}



// Function: namebuf_add_dir @ 0x2b7ec
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

struct_0 * namebuf_add_dir(struct_0 *idx, char *a1)
{
    unsigned long len;  // rax

    if (*((char *)(idx->field_10 - 1 + idx->field_0)) != 47)
    {
        sub_42b75b(idx, "/");
        idx->field_10 = idx->field_10 + 1;
    }
    sub_42b75b(idx, a1);
    len = strlen(a1);
    idx->field_10 = idx->field_10 + len;
    return idx;
}



// Function: namebuf_finish @ 0x2b882
long long namebuf_finish(unsigned long long *ptr)
{
    unsigned long v0;  // [bp-0x10]

    v0 = *(ptr);
    if (*((char *)(ptr[2] - 1 + *(ptr))) == 47)
        *((char *)(ptr[2] + *(ptr))) = 0;
    free(ptr);
    return v0;
}



// Function: tar_savedir @ 0x2b8e1
extern unsigned int g_49c268;
extern unsigned int g_49fc08;
extern unsigned int g_49fc2c;

unsigned long long tar_savedir(long long a0, unsigned int a1)
{
    char v4;  // al
    char v5;  // al
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]
    unsigned long fp;  // [bp-0x10]

    v1 = 0;
    fp = 0;
    v0 = openat(g_49c268, a0, g_49fc08 | 0x10000, a0);
    if (v0 < 0)
    {
        if (!a1 && *(__errno_location()) == 2)
            return 0;
        sub_43e38b(a0);
    }
    else
    {
        fp = fdopendir(v0);
        if (!fp || (v1 = (unsigned long long)sub_45eee2(fp, (unsigned long long)g_49fc2c, (unsigned long long)g_49fc2c), !v1))
            sub_43e72e(a0);
    }
    if (fp)
    {
        v4 = closedir(fp);
    }
    else
    {
        if (v0 >= 0 && close(v0))
            v5 = 1;
        else
            v5 = 0;
        v4 = v5 & 1;
    }
    if (v4)
        sub_43e72e(a0);
    return v1;
}



// Function: file_selection_option @ 0x2b9f2
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[20];
    unsigned long long field_20;
} struct_0;

extern struct_0 g_49c280;

struct_0 * file_selection_option(unsigned int a0)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = &g_49c280.field_0;
    while (1)
    {
        if (!v0->field_0 && !v0->field_8 && !v0->field_20)
            return NULL;
        if (a0 == v0->field_8)
            return v0;
        v0 = &v0[1].field_8;
    }
}



// Function: file_selection_option_name @ 0x2ba4c
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[20];
    unsigned long long field_20;
} struct_0;

unsigned long long file_selection_option_name(unsigned int a0)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_42b9f2(a0);
    return (!v0 ? 0 : v0->field_0);
}



// Function: is_file_selection_option @ 0x2ba80
char is_file_selection_option(unsigned int a0)
{
    return sub_42b9f2(a0);
}



// Function: names_parse_opt @ 0x2baa1
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

unsigned int names_parse_opt(unsigned int a0, long long a1, struct_1 *a2)
{
    struct_0 **v0;  // [bp-0x20]

    switch (a0)
    {
    case 16777221:
        v0 = a2->field_28;
        if (*(v0)->field_0 != 2)
            return 7;
        error(0, 0, gettext("%s:%lu: unrecognized option"));
        sub_439e98(2);
        return 7;
    case 256:
        sub_42c829(a1);
        break;
    case 67:
        sub_42c8c4(a1);
        break;
    case 84:
        sub_42c8f7(a1);
        break;
    default:
        if (sub_42ba80(a0))
        {
            sub_42c884(a0, a1, a1);
            break;
        }
        else
        {
            return 7;
        }
    }
    return 0;
}



// Function: add_exclude_array @ 0x2bbce
extern long long g_49f8a8;

unsigned long long add_exclude_array(unsigned long a0, unsigned int a1)
{
    int i;  // [bp-0xc]

    for (i = 0; *((long long *)(a0 + i * 8)); i += 1)
    {
        sub_44ee5d(g_49f8a8, *((long long *)(a0 + i * 8)), a1, *((long long *)(a0 + i * 8)));
    }
    return 0;
}



// Function: handle_file_selection_option @ 0x2bc3a
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned int g_49a6e0;
extern unsigned int g_49a7a0;
extern char g_49c270;
extern unsigned int g_49c274;
extern char g_49f6b8;
extern unsigned int g_49f6bc;
extern unsigned int g_49f6c0;
extern long long g_49f8a8;
extern unsigned int g_49f930;
extern char g_49fc28;
extern struct_1 *g_49fdc8;

long long handle_file_selection_option(unsigned int a0, unsigned long a1)
{
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    unsigned int v0;  // [bp-0xc]

    switch (a0)
    {
    case 88:
        v4 = sub_44f407(sub_44ee5d, g_49f8a8, a1, (g_49f6bc == 1 ? 0 : 0x10000000) | g_49f6c0 | g_49f930, 10);
        if (!(unsigned int)v4)
            return v4;
        v0 = *(__errno_location());
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(a1);
        error(0, v0, "%s");
        sub_4177f4(); /* do not return */
    case 257:
        return sub_42bbce(&g_49a7a0, 0x10000000);
    case 258:
        return sub_411ea1("CACHEDIR.TAG", 1, sub_412087);
    case 259:
        return sub_411ea1("CACHEDIR.TAG", 2, sub_412087);
    case 260:
        return sub_411ea1("CACHEDIR.TAG", 3, sub_412087);
    case 261:
        return sub_44ee5d(g_49f8a8, a1, (g_49f6bc == 1 ? 0 : 0x10000000) | g_49f6c0 | g_49f930, a1);
    case 262:
        return sub_417879(a1, 2);
    case 263:
        return sub_417879(a1, 1);
    case 264:
        return sub_411ea1(a1, 1, 0);
    case 265:
        return sub_411ea1(a1, 2, 0);
    case 266:
        return sub_411ea1(a1, 3, 0);
    case 267:
        return sub_42bbce(&g_49a6e0, 0);
    case 268:
        return sub_418293();
    case 269:
        g_49f6c0 = g_49f6c0 | 16;
        return g_49f6c0;
    case 270:
        g_49f6c0 = g_49f6c0 & 0xffffffef;
        return g_49f6c0;
    case 271:
        g_49f6c0 = g_49f6c0 | 0x40000000;
        return g_49f6c0;
    case 272:
        g_49c274 = 0;
        g_49f6c0 = g_49f6c0 & 0xbfffffff;
        return g_49f6c0;
    case 273:
        g_49f930 = 8;
        return v3;
    case 274:
        g_49f930 = 0;
        return v3;
    case 275:
        g_49fc28 = 1;
        return v3;
    case 276:
        g_49fc28 = 0;
        return v3;
    case 277:
        g_49f6b8 = 0;
        return v3;
    case 278:
        g_49f6c0 = g_49f6c0 | 1;
        return g_49f6c0;
    case 279:
        g_49f6bc = 1;
        return v3;
    case 280:
        g_49c270 = 0;
        g_49f6b8 = 1;
        return v3;
    case 281:
        g_49c270 = 10;
        g_49f6b8 = 0;
        return v3;
    case 282:
        g_49f6b8 = 1;
        return v3;
    case 283:
        g_49f6c0 = g_49f6c0 & 0xfffffffe;
        return g_49f6c0;
    case 284:
        g_49f6bc = 2;
        return v3;
    default:
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, "unhandled positional option %d");
        sub_4177f4(); /* do not return */
    }
}



// Function: uid_to_uname @ 0x2bfd1
typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

extern long long g_483794;
extern long long g_49f6c8;
extern unsigned int g_49f6d8;
extern unsigned int g_49f6f0;

unsigned long long * uid_to_uname(unsigned int a0, unsigned long long *a1)
{
    passwd *v0;  // [bp-0x10]

    if (a0 && a0 == g_49f6f0)
    {
        *(a1) = sub_460cc6(&g_483794);
        return a1;
    }
    if (!g_49f6c8 || a0 != g_49f6d8)
    {
        v0 = getpwuid(a0);
        if (!v0)
        {
            g_49f6f0 = a0;
            *(a1) = sub_460cc6(&g_483794);
            return a1;
        }
        g_49f6d8 = a0;
        sub_429547(&g_49f6c8, v0->pw_name);
    }
    *(a1) = sub_460cc6(g_49f6c8);
    return a1;
}



// Function: gid_to_gname @ 0x2c0a2
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

extern long long g_483794;
extern long long g_49f6d0;
extern unsigned int g_49f6dc;
extern unsigned int g_49f6f4;

unsigned long long * gid_to_gname(unsigned int a0, unsigned long long *a1)
{
    group *v0;  // [bp-0x10]

    if (a0 && a0 == g_49f6f4)
    {
        *(a1) = sub_460cc6(&g_483794);
        return a1;
    }
    if (!g_49f6d0 || a0 != g_49f6dc)
    {
        v0 = getgrgid(a0);
        if (!v0)
        {
            g_49f6f4 = a0;
            *(a1) = sub_460cc6(&g_483794);
            return a1;
        }
        g_49f6dc = a0;
        sub_429547(&g_49f6d0, v0->gr_name);
    }
    *(a1) = sub_460cc6(g_49f6d0);
    return a1;
}



// Function: uname_to_uid @ 0x2c173
typedef struct passwd {
    char * pw_name;
    char * pw_passwd;
    unsigned int pw_uid;
    unsigned int pw_gid;
    char * pw_gecos;
    char * pw_dir;
    char * pw_shell;
} passwd;

extern char *g_49f6c8;
extern unsigned int g_49f6d8;
extern char *g_49f6e0;

unsigned int uname_to_uid(char *a0, unsigned int *a1)
{
    passwd *v0;  // [bp-0x10]

    if (g_49f6e0 && !strcmp(a0, g_49f6e0))
        return 0;
    if (!g_49f6c8 || *(a0) != *(g_49f6c8) || strcmp(a0, g_49f6c8))
    {
        v0 = getpwnam(a0);
        if (!v0)
        {
            sub_429547(&g_49f6e0, a0);
            return 0;
        }
        g_49f6d8 = v0->pw_uid;
        sub_429547(&g_49f6c8, v0->pw_name);
    }
    *(a1) = g_49f6d8;
    return 1;
}



// Function: gname_to_gid @ 0x2c261
typedef struct group {
    char * gr_name;
    char * gr_passwd;
    unsigned int gr_gid;
    char ** gr_mem;
} group;

extern char *g_49f6d0;
extern unsigned int g_49f6dc;
extern char *g_49f6e8;

unsigned int gname_to_gid(char *a0, unsigned int *a1)
{
    group *v0;  // [bp-0x10]

    if (g_49f6e8 && !strcmp(a0, g_49f6e8))
        return 0;
    if (!g_49f6d0 || *(a0) != *(g_49f6d0) || strcmp(a0, g_49f6d0))
    {
        v0 = getgrnam(a0);
        if (!v0)
        {
            sub_429547(&g_49f6e8, a0);
            return 0;
        }
        g_49f6dc = v0->gr_gid;
        sub_429547(&g_49f6d0, a0);
    }
    *(a1) = g_49f6dc;
    return 1;
}



// Function: make_name @ 0x2c34c
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
    char field_18;
} struct_0;

extern long long g_483794;

struct_0 * make_name(long long a0)
{
    long long v0;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x10]

    v0 = a0;
    idx = sub_460c06(96);
    if (!v0)
        v0 = &g_483794;
    idx->field_10 = sub_460cc6(v0);
    *((unsigned long *)&idx->field_18) = strlen(idx->field_10);
    return idx;
}



// Function: free_name @ 0x2c3b1
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



// Function: name_elt_alloc @ 0x2c3f7
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    unsigned int field_10;
} struct_0;

extern struct_0 *g_49f708;

struct_0 * name_elt_alloc(void)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_460b5b(56);
    if (!g_49f708)
    {
        g_49f708 = v0;
        g_49f708->field_0 = NULL;
        g_49f708->field_8 = g_49f708->field_0;
        *((unsigned int *)&g_49f708[1].field_0) = 3;
        v0 = sub_460b5b(56);
    }
    v0->field_8 = g_49f708->field_8;
    if (g_49f708->field_8)
        *((struct_0 **)g_49f708->field_8) = v0;
    v0->field_0 = g_49f708;
    g_49f708->field_8 = v0;
    return v0;
}



// Function: name_list_adjust @ 0x2c4b8
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

extern struct_0 *g_49f708;

struct_0 * name_list_adjust(void)
{
    if (!g_49f708)
        return g_49f708;
    for (; g_49f708->field_8; g_49f708 = g_49f708->field_8);
    return g_49f708->field_8;
}



// Function: unconsumed_option_push @ 0x2c4f3
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_1 *field_8;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_1 *g_49f6a8;

struct_0 * unconsumed_option_push(struct_0 *a0)
{
    a0->field_0 = 0;
    a0->field_8 = g_49f6a8;
    if (g_49f6a8)
        g_49f6a8->field_0 = a0;
    g_49f6a8 = a0;
    return a0;
}



// Function: unconsumed_option_free @ 0x2c541
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

extern struct_0 *g_49f6a8;

unsigned long long unconsumed_option_free(void)
{
    struct_0 *v0;  // [bp-0x10]

    while (g_49f6a8)
    {
        v0 = g_49f6a8;
        g_49f6a8 = g_49f6a8->field_8;
        free(v0);
    }
    return 0;
}



// Function: unconsumed_option_report @ 0x2c588
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern void* g_49f6a8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void* unconsumed_option_report(void)
{
    void* iter;  // [bp-0x20]

    if (!g_49f6a8)
        return g_49f6a8;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly."));
    g_49fdd0 = 2;
    for (iter = g_49f6a8; (long long)iter[8]; iter = (long long)iter[8]);
    for (; iter; iter = *((long long *)iter))
    {
        if ((int)iter[16] == 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e84e((long long)iter[24]);
            error(0, 0, gettext("-C %s has no effect"));
            g_49fdd0 = 2;
        }
        else if ((int)iter[16] == 4)
        {
            if ((long long)iter[32])
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_45e84e((long long)iter[32]);
                sub_42ba4c((int)iter[24]);
                error(0, 0, gettext("--%s %s has no effect"));
                g_49fdd0 = 2;
            }
            else
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_42ba4c((int)iter[24]);
                error(0, 0, gettext("--%s has no effect"));
                g_49fdd0 = 2;
            }
        }
    }
    return sub_42c541();
}



// Function: name_list_advance @ 0x2c78a
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned int field_10;
} struct_0;

extern struct_0 *g_49f708;
extern unsigned int g_49f860;

long long name_list_advance(void)
{
    unsigned long v2;  // rax
    struct_0 *v0;  // [bp-0x10]

    v0 = g_49f708;
    g_49f708 = v0->field_0;
    if (g_49f708)
        g_49f708->field_8 = 0;
    if (v0->field_10 != 4 && v0->field_10 != 1)
    {
        if (v0->field_10 == 3)
            return (unsigned long long)free(v0);
        sub_42c541();
        return (unsigned long long)free(v0);
    }
    if (g_49f860 == 3 || (v2 = (unsigned long)(unsigned long long)g_49f860, g_49f860 == 8))
        v2 = sub_42c4f3(v0);
    return v2;
}



// Function: name_add_name @ 0x2c829
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    unsigned int field_10;
} struct_0;

extern unsigned int g_49f6a0;

long long name_add_name(unsigned long a0)
{
    unsigned long v2;  // rax
    struct_0 *idx;  // [bp-0x10]

    idx = sub_42c3f7();
    idx->field_10 = 0;
    *((unsigned long *)((char *)&idx[1].field_0 + 4)) = a0;
    v2 = g_49f6a0;
    if (!(unsigned int)v2)
    {
        g_49f6a0 = 1;
        return v2;
    }
    if ((unsigned int)v2 != 1)
        return v2;
    g_49f6a0 = 2;
    return v2;
}



// Function: name_add_option @ 0x2c884
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    unsigned int field_10;
} struct_0;

struct_0 * name_add_option(unsigned int a0, unsigned long a1)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_42c3f7();
    idx->field_10 = 4;
    *((unsigned int *)((char *)&idx[1].field_0 + 4)) = a0;
    *((unsigned long *)((char *)&idx[1].field_8 + 4)) = a1;
    return idx;
}



// Function: name_add_dir @ 0x2c8c4
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    unsigned int field_10;
} struct_0;

struct_0 * name_add_dir(unsigned long a0)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_42c3f7();
    idx->field_10 = 1;
    *((unsigned long *)((char *)&idx[1].field_0 + 4)) = a0;
    return idx;
}



// Function: name_add_file @ 0x2c8f7
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    unsigned int field_10;
} struct_0;

extern unsigned int g_49f6a0;

struct_0 * name_add_file(unsigned long a0)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_42c3f7();
    idx->field_10 = 2;
    *((unsigned long *)((char *)&idx[1].field_0 + 4)) = a0;
    *((unsigned long *)((char *)&idx[1].field_8 + 4)) = 0;
    idx[2].field_8 = 0;
    g_49f6a0 = 2;
    return idx;
}



// Function: name_init @ 0x2c94c
typedef struct struct_0 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_0;

extern unsigned long long g_49f710;
extern unsigned long long g_49f718;

struct_0 * name_init(void)
{
    g_49f710 = sub_460b5b(0x66);
    g_49f718 = 100;
    return sub_42c4b8();
}



// Function: name_term @ 0x2c978
extern void* g_49f710;

void name_term(void)
{
    free(g_49f710);
    return;
}



// Function: file_list_name @ 0x2c992
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    unsigned long long field_18;
    char padding_20[16];
    unsigned long long field_30;
} struct_0;

extern struct_0 *g_49f708;

char * file_list_name(void)
{
    struct_0 *i;  // [bp-0x10]

    for (i = g_49f708; i; i = i->field_0)
    {
        if (i->field_10 == 2 && i->field_30)
            return i->field_18;
    }
    return gettext("command line");
}



// Function: add_file_id @ 0x2c9f1
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

extern struct_0 *g_49f720;
extern struct_3 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned long long add_file_id(unsigned long a0)
{
    unsigned int v0;  // [bp-0xcc]
    struct_0 **idx;  // [bp-0xc8], Other Possible Types: struct_2 *
    char *v2;  // [bp-0xc0]
    stat v3;  // [bp-0xb8]

    if (stat(a0, &v3))
        sub_43e9dd(a0); /* do not return */
    v2 = sub_42c992();
    idx = g_49f720;
    while (1)
    {
        if (!idx)
        {
            idx = sub_460b5b(32);
            idx->field_0 = g_49f720;
            idx->field_8 = v3.st_ino;
            idx->field_10 = *((unsigned long long *)&v3);
            idx->field_18 = v2;
            g_49f720 = idx;
            return 0;
        }
        if (idx[1] == v3.st_ino && idx[2] == *((unsigned long long *)&v3))
            break;
        idx = *(idx);
    }
    v0 = sub_45c9cd(0, 58, 1);
    if (g_49fdc8)
        g_49fdc8();
    sub_45e285(0, a0);
    error(0, 0, gettext("%s: file list requested from %s already read from %s"));
    g_49fdd0 = 2;
    sub_45c9cd(0, 58, v0);
    return 1;
}



// Function: chopslash @ 0x2cbf6
char * chopslash(char *a0)
{
    unsigned long v3;  // rax
    char *i;  // [bp-0x10]
    char *v1;  // [bp-0x10]

    for (i = a0 + strlen(a0) - 1; i > a0; i = v1)
    {
        v3 = *(i);
        if ((char)v3 != 47)
            return v3;
        v1 = i - 1;
        *(i) = 0;
    }
    return i;
}



// Function: read_name_from_file @ 0x2cc4a
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
    unsigned int field_28;
    char padding_2c[4];
    unsigned long long field_30;
} struct_0;

extern void g_49f710;
extern long long g_49f718;

unsigned int read_name_from_file(struct_0 *idx)
{
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]
    unsigned long long i;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    i = 0;
    v4 = idx->field_30;
    v1 = idx->field_28;
    idx->field_20 = idx->field_20 + 1;
    for (v0 = getc_unlocked(v4); v0 != 0xffffffff && v0 != v1; i = v3)
    {
        if (i == g_49f718)
            *((unsigned long long *)&g_49f710) = sub_460bd8(*((long long *)&g_49f710), &g_49f718, &g_49f718);
        v3 = i + 1;
        *((char *)(i + *((long long *)&g_49f710))) = v0;
        if (!v0)
            return 2;
        v0 = getc_unlocked(v4);
    }
    if (!i && v0 != 0xffffffff)
        return 3;
    if (i == g_49f718)
        *((unsigned long long *)&g_49f710) = sub_460bd8(*((long long *)&g_49f710), &g_49f718, &g_49f718);
    *((char *)(i + *((long long *)&g_49f710))) = 0;
    sub_42cbf6(*((long long *)&g_49f710));
    if (!i && v0 == 0xffffffff)
        return 1;
    return 0;
}



// Function: handle_option @ 0x2cd8e
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned long long g_49fe48;

unsigned int handle_option(char *a0, unsigned long long *idx)
{
    unsigned long long *v9;  // rax
    char *i;  // [bp-0x140]
    int j;  // [bp-0x12c]
    unsigned int v2;  // [bp-0x128]
    unsigned long v3;  // [bp-0x120]
    unsigned long v4;  // [bp-0x118]
    char v5;  // [bp-0x108], Other Possible Types: unsigned long
    unsigned long long *v6;  // [bp-0x100]
    unsigned long long v7;  // [bp-0xf8]

    for (i = a0; *(i) && (v9 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((long long)*(i) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000); i += 1);
    if (*(i) != 45)
        return 1;
    v7 = 1;
    if (sub_4461a6(i, &v5, 33558086, &v5))
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_44647f(&v5);
        error(0, 0, gettext("cannot split string '%s': %s"));
        sub_4177f4(); /* do not return */
    }
    else
    {
        *(v6) = g_49fe48;
        v2 = 2;
        v3 = idx[3];
        v4 = idx[4];
        sub_43828e(v7 + (v5 & 0xffffffff) & 0xffffffff, v6, &v2, v7 + (v5 & 0xffffffff) & 0xffffffff);
        for (j = 0; j < v7 + v5; j += 1)
        {
            v6[j] = 0;
        }
        sub_446399(&v5);
        return 0;
    }
}



// Function: read_next_name @ 0x2cf89
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    char *field_18;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern char g_49c270;
extern unsigned int g_49ec84;
extern unsigned long long stdin;
extern char g_49f6b8;
extern char *g_49f710;
extern char g_49fc28;
extern struct_0 *g_49fdc8;

unsigned long long read_next_name(void* idx, struct_1 *index)
{
    unsigned int i;  // eax

    if (!(long long)idx[48])
    {
        if (!strcmp((long long)idx[24], "-"))
        {
            sub_434a6f("-T");
            *((unsigned long long *)&idx[48]) = stdin;
        }
        else if ((unsigned int)sub_42c9f1((long long)idx[24]))
        {
            sub_42c78a();
            return 1;
        }
        else
        {
            *((struct FILE **)&idx[48]) = fopen((long long)idx[24], "r");
            if (!(long long)idx[48])
                sub_43e3b4((long long)idx[24]); /* do not return */
        }
        *((int *)&idx[40]) = g_49c270;
        *((char *)&idx[44]) = g_49f6b8;
    }
    do
    {
        while (1)
        {
            i = sub_42cc4a(idx);
            if (i == 3)
                continue;
            if (i <= 3)
                break;
        }
        if (i != 2)
        {
            if (i > 2)
                continue;
            if (!i)
                goto LABEL_42d103;
        }
        else
        {
            if ((unsigned short)g_49ec84 & 0x200)
            {
                if (g_49fdc8)
                    g_49fdc8();
                sub_45e548((long long)idx[24]);
                error(0, 0, "%s: file name read contains nul character");
            }
            *((unsigned int *)&idx[40]) = 0;
LABEL_42d103:
            if ((char)idx[44] != 1)
            {
                if (g_49fc28)
                    sub_42961d(g_49f710);
                if (!sub_42cd8e(g_49f710, idx))
                {
                    sub_42c4b8();
                    return 1;
                }
            }
            index->field_10 = 0;
            index->field_18 = g_49f710;
            return 0;
        }
    } while (i != 1);
    if (strcmp((long long)idx[24], "-"))
        fclose((long long)idx[48]);
    *((FILE **)&idx[48]) = NULL;
    sub_42c78a();
    return 1;
}



// Function: copy_name @ 0x2d1c1
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

extern void g_49f710;
extern long long g_49f718;

char * copy_name(struct_0 *a0)
{
    unsigned long ptr;  // [bp-0x18]
    unsigned long len;  // [bp-0x10]

    ptr = a0->field_18;
    for (len = strlen(ptr); len >= g_49f718; *((unsigned long long *)&g_49f710) = sub_460bd8(*((long long *)&g_49f710), &g_49f718, &g_49f718));
    strcpy(*((long long *)&g_49f710), ptr);
    return sub_42cbf6(*((long long *)&g_49f710));
}



// Function: name_next_elt @ 0x2d244
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    char *field_18;
} struct_1;

extern unsigned int g_483944[4];
extern void* g_49f708;

struct_1 * name_next_elt(unsigned int a0)
{
    void* v0;  // [bp-0x10]

    while (1)
    {
        v0 = g_49f708;
        if (!v0)
        {
            sub_42c588();
            return NULL;
        }
        else if (!((int)v0[16] <= 4))
        {
            continue;
        }
        goto *((void *)((long long)(g_483944[(int)v0[16]] + (char *)&g_483944[0])));
        sub_42bc3a((int)v0[24], (long long)v0[32]);
        sub_42c78a();
    }
    sub_42c78a();
    continue;
}



// Function: name_next @ 0x2d37c
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    char *field_18;
} struct_1;

unsigned long long name_next(unsigned int a0)
{
    struct_1 *v0;  // [bp-0x10]

    v0 = sub_42d244(a0);
    return (!v0 ? 0 : v0->field_18);
}



// Function: name_gather @ 0x2d3b1
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    char *field_18;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c274;
extern unsigned int g_49f6bc;
extern unsigned int g_49f6c0;
extern unsigned long long g_49f6f8;
extern unsigned long long g_49f700;
extern unsigned int g_49f778;
extern struct_0 *g_49f780;
extern unsigned int g_49f930;
extern char g_49f958;

long long name_gather(void)
{
    unsigned long v4;  // rax
    unsigned int v0;  // [bp-0x18]
    unsigned int v1;  // [bp-0x14]
    struct_1 *v2;  // [bp-0x10]

    if (g_49f958)
    {
        while (1)
        {
            v2 = sub_42d244(0);
            if (!v2 || v2->field_10 != 1)
                break;
            g_49f778 = sub_42ac09(sub_460cc6(v2->field_18));
        }
        if (v2)
        {
            sub_42c3b1(g_49f780);
            g_49f780 = sub_42c34c(v2->field_18);
            *((unsigned int *)&g_49f780[2].field_8) = g_49f778;
            g_49f780->field_0 = 0;
            g_49f780[3].field_0 = 0;
            *((unsigned int *)&g_49f780[2].field_0) = g_49f930 | (g_49f6bc == 2 ? 0x10000000 : 0) | g_49c274 | g_49f6c0;
            g_49f780[3].field_8 = 0;
            g_49f780[4].field_0 = 0;
            *((char *)&g_49f780[2].field_0 + 4) = 1;
            g_49f700 = g_49f780;
            g_49f6f8 = g_49f700;
            return g_49f700;
        }
        else if (g_49f778)
        {
            return sub_42d5b5(0, g_49f778, 0, 0);
        }
        else
        {
            return g_49f778;
        }
    }
    else
    {
        v0 = 0;
        while (1)
        {
            v1 = v0;
            while (1)
            {
                v2 = sub_42d244(0);
                if (!v2 || v2->field_10 != 1)
                    break;
                v0 = sub_42ac09(sub_460cc6(v2->field_18));
            }
            if (!v2)
                break;
            sub_42d5b5(v2->field_18, v0, 1, 0);
        }
        v4 = v0;
        if ((unsigned int)v4 != v1)
            v4 = sub_42d5b5(0, v0, 0, 0);
        return v4;
    }
}



// Function: addname @ 0x2d5b5
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
    char field_18;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned int g_49c274;
extern unsigned int g_49f6bc;
extern unsigned int g_49f6c0;
extern unsigned long long g_49f6f8;
extern struct_1 *g_49f700;
extern unsigned int g_49f930;

struct_0 * addname(long long a0, unsigned int a1, char a2, unsigned long a3)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_42c34c(a0);
    *((struct_1 **)&idx->padding_0[8]) = g_49f700;
    *((unsigned long *)&idx->padding_0[0]) = 0;
    *((unsigned long *)((char *)&idx[1].field_10 + 7)) = 0;
    *((unsigned int *)&idx[1].padding_0[7]) = (g_49f6bc == 2 ? 0x10000000 : 0) | g_49c274 | g_49f6c0 | g_49f930;
    *((unsigned int *)&idx[1].padding_0[15]) = a1;
    *((unsigned long *)&idx[2].padding_0[6]) = 0;
    *((unsigned long *)&idx[2].padding_0[14]) = a3;
    idx[1].padding_0[11] = a2;
    if (g_49f700)
        g_49f700->field_0 = idx;
    else
        g_49f6f8 = idx;
    g_49f700 = idx;
    return idx;
}



// Function: add_starting_file @ 0x2d6a2
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
    char field_18;
} struct_0;

extern unsigned int g_49c274;
extern unsigned int g_49f6bc;
extern unsigned int g_49f6c0;
extern unsigned long long g_49f6f8;
extern unsigned long long g_49f700;
extern unsigned int g_49f930;
extern char g_49f988;

struct_0 * add_starting_file(long long a0)
{
    struct_0 *idx;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    idx = sub_42c34c(a0);
    if (g_49f988)
    {
        v1 = g_49f6f8;
        sub_42d817(v1);
        sub_42c3b1(v1);
    }
    *((unsigned long *)&idx->padding_0[8]) = 0;
    *((unsigned long long *)&idx->padding_0[0]) = g_49f6f8;
    g_49f6f8 = idx;
    if (!g_49f700)
        g_49f700 = g_49f6f8;
    *((unsigned long *)((char *)&idx[1].field_10 + 7)) = 0;
    *((unsigned int *)&idx[1].padding_0[7]) = (g_49f6bc == 2 ? 0x10000000 : 0) | g_49c274 | g_49f6c0 | g_49f930;
    *((unsigned int *)&idx[1].padding_0[15]) = 0;
    *((unsigned long *)&idx[2].padding_0[6]) = 0;
    *((unsigned long *)&idx[2].padding_0[14]) = 0;
    idx[1].padding_0[11] = 1;
    g_49f988 = 1;
    return idx;
}



// Function: namelist_match @ 0x2d7a6
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    struct struct_1 *field_10;
    char padding_18[8];
    unsigned int field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern struct_0 *g_49f6f8;

struct_0 * namelist_match(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x28]
    struct_0 *i;  // [bp-0x10]

    v0 = a1;
    for (i = g_49f6f8; i; i = i->field_0)
    {
        if (i->field_10->field_0 && (char)sub_44ead5(i->field_10, a0, i->field_20, a0))
            return i;
    }
    return NULL;
}



// Function: remname @ 0x2d817
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

extern unsigned long long g_49f6f8;
extern struct_0 *g_49f700;

struct_0 * remname(struct_4 *idx)
{
    struct_0 *v0;  // [bp-0x10], Other Possible Types: struct_0 **

    v0 = idx->field_8;
    if (v0)
        *(v0) = idx->field_0;
    else
        g_49f6f8 = idx->field_0;
    v0 = idx->field_0;
    if (!v0)
    {
        g_49f700 = idx->field_8;
        return g_49f700;
    }
    v0->field_8 = idx->field_8;
    return v0;
}



// Function: name_match @ 0x2d88a
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

extern struct_0 *g_49f6f8;
extern unsigned long long g_49f700;
extern char g_49f8d0;
extern unsigned int g_49f930;
extern char g_49f958;
extern char g_49f988;

unsigned long long name_match(unsigned long a0)
{
    struct_0 *idx;  // [bp-0x18]
    unsigned long len;  // [bp-0x10]

    len = strlen(a0);
    while (1)
    {
        idx = g_49f6f8;
        if (!idx)
            return 1;
        if (!idx->field_10->field_0)
        {
            sub_42adf1(idx->field_28);
            g_49f6f8 = 0;
            g_49f700 = 0;
            return 1;
        }
        idx = sub_42d7a6(a0, len);
        if (g_49f988)
        {
            if (idx == g_49f6f8)
                g_49f988 = 0;
            else
                idx = NULL;
        }
        if (idx)
            break;
        if (!g_49f958 || !g_49f6f8[3].field_0)
            return 0;
        sub_42d3b1();
        if (g_49f6f8[3].field_0)
            return 0;
    }
    if (*((char *)(a0 + idx->field_18)) != 47 || !g_49f930 || !idx->field_30)
        idx->field_30 = idx->field_30 + 1;
    sub_42adf1(idx->field_28);
    return (!*((long long *)&g_49f8d0) ? _INSERT(idx->field_30, 0, idx->field_30) : _INSERT(*((long long *)&g_49f8d0), 0, idx->field_30 == *((long long *)&g_49f8d0)));
}



// Function: all_names_found @ 0x2da15
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

extern struct_1 *g_49f6f8;
extern unsigned long long g_49f8d0;

unsigned int all_names_found(struct_0 *a0)
{
    struct_1 *v0;  // [bp-0x18]
    unsigned long len;  // [bp-0x10]

    if (a0->field_8 && g_49f8d0 && !a0->field_10)
    {
        len = strlen(a0->field_8);
        v0 = g_49f6f8;
        while (1)
        {
            if (!v0)
                return 1;
            if (v0->field_10->field_0)
            {
                if (!g_49f8d0)
                {
                    if (!v0[2].field_0)
                        break;
                }
                else
                {
                    if (v0[2].field_0 < g_49f8d0)
                        break;
                }
            }
            if (len >= v0[1].field_0 && *((char *)(v0[1].field_0 + a0->field_8)) == 47)
                break;
            v0 = v0->field_0;
        }
        return 0;
    }
    return 0;
}



// Function: regex_usage_warning @ 0x2dafb
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f6bc;
extern unsigned int g_49f788;
extern struct_0 *g_49fdc8;

int regex_usage_warning(long long a0)
{
    if (!g_49f6bc && (char)sub_44e397(a0, 0))
    {
        g_49f788 = 1;
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Pattern matching characters used in file names"));
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning"));
    }
    return g_49f788;
}



// Function: names_notfound @ 0x2dbba
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    struct struct_1 *field_10;
    char padding_18[24];
    unsigned long long field_30;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern struct_0 *g_49f6f8;
extern unsigned long long g_49f700;
extern char g_49f8d0;
extern char g_49f958;
extern struct_2 *g_49fdc8;
extern unsigned int g_49fdd0;

long long names_notfound(void)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    for (i = g_49f6f8; i; i = i->field_0)
    {
        if ((!*((long long *)&g_49f8d0) ? !i->field_30 : i->field_30 < *((long long *)&g_49f8d0)) && i->field_10->field_0)
        {
            sub_42dafb(i->field_10);
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548(i->field_10);
            error(0, 0, (!i->field_30 ? gettext("%s: Not found in archive") : gettext("%s: Required occurrence not found in archive")));
            g_49fdd0 = 2;
        }
    }
    g_49f6f8 = 0;
    g_49f700 = 0;
    if (!g_49f958)
        return g_49f958;
    while (1)
    {
        v1 = sub_42d37c(1);
        if (!v1)
            break;
        sub_42dafb(v1);
        if (g_49fdc8)
            g_49fdc8();
        sub_45e548(v1);
        error(0, 0, gettext("%s: Not found in archive"));
        g_49fdd0 = 2;
    }
    return v1;
}



// Function: label_notfound @ 0x2dd64
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    long long field_10;
    char padding_18[24];
    unsigned long long field_30;
} struct_0;

extern struct_0 *g_49f6f8;
extern unsigned long long g_49f700;
extern char g_49f8d0;
extern char g_49f958;
extern unsigned int g_49f9ac;

struct_0 * label_notfound(void)
{
    struct_0 *v3;  // rax
    struct_0 *v5;  // rax
    struct_0 *i;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    if (!g_49f6f8)
        return g_49f6f8;
    for (i = g_49f6f8; i; i = i->field_0)
    {
        v3 = (struct_0 *)(!*((long long *)&g_49f8d0) ? _INSERT(i->field_30, 0, i->field_30) : _INSERT(*((long long *)&g_49f8d0), 0, *((long long *)&g_49f8d0) <= i->field_30));
        if (*((char *)&v3))
            return v3;
    }
    if (g_49f9ac)
        error(0, 0, gettext("Archive label mismatch"));
    sub_439e98(1);
    for (i = g_49f6f8; i && !sub_42dafb(i->field_10); i = i->field_0);
    g_49f6f8 = 0;
    g_49f700 = 0;
    if (!g_49f958)
        return g_49f958;
    while (1)
    {
        v1 = sub_42d37c(1);
        if (!v1)
            return v1;
        v5 = sub_42dafb(v1);
        if ((unsigned int)v5)
            return v5;
    }
}



// Function: merge_sort_sll @ 0x2de97
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

struct_0 ** merge_sort_sll(struct_0 **a0, unsigned int a1, unsigned long long *a2)
{
    int i;  // [bp-0x44]
    int v1;  // [bp-0x40]
    int v2;  // [bp-0x3c]
    struct_0 **v3;  // [bp-0x38]
    struct_0 **v4;  // [bp-0x30]
    struct_0 **v5;  // [bp-0x28]
    struct_0 **v6;  // [bp-0x20]
    struct_0 **v7;  // [bp-0x18]

    if (a1 == 1)
    {
        return a0;
    }
    else if (a1 != 2)
    {
        v4 = a0;
        v1 = (int)(a1 + 1 + (a1 + 1 >> 31)) >> 1;
        v2 = (int)(a1 + (a1 >> 31)) >> 1;
        v7 = a0;
        for (i = v1 - 1; i; i -= 1)
        {
            v7 = *(v7);
        }
        v5 = *(v7);
        *(v7) = NULL;
        v4 = merge_sort_sll(v4, v1, a2);
        v5 = merge_sort_sll(v5, v2, a2);
        v6 = &v3;
        while (v4 && v5)
        {
            if (a2(v4, v5, v5, a2) < 0)
            {
                v7 = *(v4);
                *(v6) = v4;
                v6 = v4;
                v4 = v7;
            }
            else
            {
                v7 = *(v5);
                *(v6) = v5;
                v6 = v5;
                v5 = v7;
            }
        }
        if (v4)
            *(v6) = v4;
        else
            *(v6) = v5;
        return v3;
    }
    else if (a2(a0, *(a0), *(a0), a2) > 0)
    {
        v3 = *(a0);
        *(v3) = a0;
        *(a0) = NULL;
        return v3;
    }
    else
    {
        return a0;
    }
}



// Function: merge_sort @ 0x2e071
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
} struct_0;

struct_0 * merge_sort(struct_0 ***a0, unsigned int a1, unsigned long long *a2)
{
    struct_0 *i;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x18]
    struct_0 *v2;  // [bp-0x10]

    v2 = sub_42de97(a0, a1, a2);
    v1 = NULL;
    for (i = v2; i; i = i->field_0)
    {
        i->field_8 = v1;
        v1 = i;
    }
    return v2;
}



// Function: compare_names_found @ 0x2e0df
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
} struct_0;

extern unsigned long long g_49f8d0;

unsigned int compare_names_found(struct_0 *a0, struct_0 *a1)
{
    unsigned int v0;  // [bp-0xc]

    v0 = (!g_49f8d0 ? *((long long *)&a1[2].padding_0[0]) : g_49f8d0 <= *((long long *)&a1[2].padding_0[0])) - (!g_49f8d0 ? *((long long *)&a0[2].padding_0[0]) : g_49f8d0 <= *((long long *)&a0[2].padding_0[0]));
    if (v0)
        return v0;
    return strcmp(a0->field_10, a1->field_10);
}



// Function: compare_names @ 0x2e18e
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
} struct_0;

int compare_names(struct_0 *a0, struct_0 *a1)
{
    return strcmp(a0->field_10, a1->field_10);
}



// Function: add_hierarchy_to_namelist @ 0x2e1bf
typedef struct struct_3 {
    char padding_0[424];
    int field_1a8;
} struct_3;

typedef struct struct_1 {
    char padding_0[16];
    char *field_10;
    unsigned long long field_18;
    char padding_20[8];
    unsigned int field_28;
    char padding_2c[12];
    void* field_38;
    char padding_40[8];
    struct struct_0 *field_48;
} struct_1;

typedef struct struct_0 {
    char padding_0[80];
    struct struct_0 *field_50;
} struct_0;

extern unsigned int g_49fc08;

void add_hierarchy_to_namelist(struct_3 *a0, struct_1 *idx)
{
    int *err;  // rax
    int v1;  // [bp-0x228]
    unsigned int v2;  // [bp-0x224]
    unsigned long v3;  // [bp-0x220]
    struct_0 *v4;  // [bp-0x218]
    struct_0 *v5;  // [bp-0x210]
    unsigned long long v6;  // [bp-0x208]
    unsigned long v7;  // [bp-0x208]
    unsigned long ptr;  // [bp-0x200]
    unsigned long iter;  // [bp-0x1f8]
    unsigned long v10;  // [bp-0x1f0]
    unsigned long len;  // [bp-0x1e8]
    struct_0 *v12;  // [bp-0x1e0]
    char v13;  // [bp-0x1d8]
    int v14;  // [bp-0x1d8]

    idx->field_38 = sub_421eb9(a0);
    v10 = sub_42241b(idx->field_38);
    if (!v10)
        return;
    v4 = NULL;
    v5 = NULL;
    v6 = idx->field_18;
    v3 = (v6 <= 99 ? 0x66 : v6 + 0x66);
    ptr = sub_460b5b(v3);
    v2 = idx->field_28;
    strcpy(ptr, idx->field_10);
    if (*((char *)(ptr + v6 - 1)) != 47)
    {
        v7 = v6 + 1;
        *((char *)(v6 + ptr)) = 47;
        *((char *)(v7 + ptr)) = 0;
        v6 = v7;
    }
    for (iter = v10; *((char *)iter); iter += len + 1)
    {
        if (*((char *)iter) == 0x44)
        {
            for (len = strlen(iter); v6 + len > v3; ptr = sub_460bd8(ptr, &v3, &v3));
            strcpy(v6 + ptr, iter + 1);
            v12 = sub_42d5b5(ptr, v2, 0, idx);
            if (!v4)
                v4 = v12;
            else
                v5->field_50 = v12;
            v5 = v12;
            sub_439c55(&v13);
            v14 = (int)_INSERT(v13 CONCAT 0, 408, a0);
            if (a0->field_1a8 < 0)
            {
                v1 = -0x1;
                err = __errno_location();
                *(err) = -(a0->field_1a8);
            }
            else
            {
                v1 = sub_4157f9(a0, iter + 1, g_49fc08 | 0x10000);
            }
            if (v1 < 0)
            {
                sub_42b2b1(ptr);
            }
            else
            {
                *((int *)&(&v14)[424]) = v1;
                if (fstat(v1, &v14 - 88))
                {
                    sub_42b444(ptr);
                }
                else
                {
                    *((unsigned long long *)&v14) = sub_460cc6(ptr);
                    add_hierarchy_to_namelist(&v14, v12);
                    sub_41588e(&v14);
                }
            }
            sub_439d1c(&v14);
        }
    }
    free(ptr);
    idx->field_48 = v4;
    return;
}



// Function: name_hash @ 0x2e590
typedef struct struct_0 {
    char padding_0[88];
    long long field_58;
} struct_0;

long long name_hash(struct_0 *a0, long long a1)
{
    return sub_452834(a0->field_58, a1, a1);
}



// Function: name_compare @ 0x2e5c5
typedef struct struct_0 {
    char padding_0[88];
    char *field_58;
} struct_0;

char name_compare(struct_0 *a0, struct_0 *a1)
{
    return !strcmp(a0->field_58, a1->field_58);
}



// Function: rebase_child_list @ 0x2e60b
typedef struct struct_2 {
    char padding_0[16];
    struct struct_0 *field_10;
    unsigned long long field_18;
} struct_2;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

typedef struct struct_4 {
    char padding_0[16];
    void* field_10;
    long long field_18;
    char padding_20[24];
    long long field_38;
    struct struct_1 *field_40;
    char padding_48[8];
    struct struct_4 *field_50;
} struct_4;

typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[24];
    unsigned long long field_38;
    struct struct_1 *field_40;
    char padding_48[8];
    struct struct_0 *field_50;
} struct_0;

struct_4 * rebase_child_list(struct_4 *a0, struct_2 *index)
{
    struct_0 *v7;  // rax
    struct_4 *iter;  // [bp-0x40]
    unsigned long v1;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28]
    struct_0 *v3;  // [bp-0x20]
    long long v4;  // [bp-0x18]
    char *ptr;  // [bp-0x10]

    iter = a0;
    v1 = iter->field_40->field_18;
    v2 = index->field_18;
    v7 = index->field_10;
    for (v3 = v7; iter; iter = iter->field_50)
    {
        v4 = v2 + iter->field_18 - v1;
        ptr = sub_460b5b(v4 + 1);
        strcpy(ptr, v3);
        strcat(ptr, iter->field_10 + v1);
        free(iter->field_10);
        iter->field_10 = ptr;
        iter->field_18 = v4;
        sub_421244(iter->field_38, iter->field_40->field_10, v1, v3, v2);
        v7 = iter->field_50;
    }
    return v7;
}



// Function: collect_and_sort_names @ 0x2e719
typedef struct struct_0 {
    char padding_0[16];
    char *field_10;
    char field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

typedef struct struct_3 {
    char padding_0[424];
    int field_1a8;
} struct_3;

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

typedef struct struct_4 {
    char padding_0[16];
    void* field_10;
    long long field_18;
    char padding_20[24];
    long long field_38;
    struct struct_1 *field_40;
    char padding_48[8];
    struct struct_4 *field_50;
} struct_4;

typedef struct struct_2 {
    struct struct_2 *field_0;
    char padding_8[8];
    struct struct_1 *field_10;
    char padding_18[8];
    unsigned int field_20;
    char padding_24[4];
    unsigned int field_28;
    char padding_2c[4];
    unsigned long long field_30;
    struct struct_0 *field_38;
    unsigned long long field_40;
    struct struct_4 *field_48;
    char padding_50[8];
    void* field_58;
} struct_2;

extern unsigned int g_49c268;
extern struct_2 *g_49f6f8;
extern unsigned long long g_49f700;
extern unsigned long long g_49f8e0;
extern unsigned int g_49fc08;
extern struct_4 *g_49fdc8;

void collect_and_sort_names(void)
{
    unsigned int v11;  // eax
    unsigned int iter;  // [bp-0x1f8]
    unsigned int v1;  // [bp-0x1f4]
    struct_2 *idx;  // [bp-0x1f0]
    struct_0 *v3;  // [bp-0x1e8]
    long long v4;  // [bp-0x1e0]
    struct_0 *v5;  // [bp-0x1d8]
    struct_2 *v6;  // [bp-0x1d0]
    struct_3 v7;  // [bp-0x1c8]
    int v8;  // [bp-0x170], Other Possible Types: stat
    unsigned int v9;  // [bp-0x20]

    v3 = NULL;
    sub_42d3b1();
    if (!g_49f6f8)
        sub_42d5b5(".", 0, 0, 0);
    if (g_49f8e0)
    {
        v11 = sub_42abe0();
        if (v11)
        {
            if (v11 != 1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Only one -C option is allowed with --listed-incremental"));
                sub_43814c(2); /* do not return */
            }
            else if (!(int)g_49f6f8[5].field_0)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Using -C option inside file list is not allowed with --listed-incremental"));
                sub_43814c(2); /* do not return */
            }
        }
        sub_424090();
    }
    iter = 0;
    for (idx = g_49f6f8; idx; iter += 1)
    {
        if (!idx->field_30 && !idx->field_38 && !(idx->field_20 & 0x10000000))
        {
            sub_42adf1(idx->field_28);
            if (idx->field_10->field_0)
            {
                sub_439c55(&v7);
                if (sub_42a9b8(idx->field_10, &v8))
                {
                    sub_42b444(idx->field_10);
                }
                else
                {
                    if (((unsigned short)*((unsigned int *)(&v8 + 24)) & 0xf000) == 0x4000)
                    {
                        v1 = openat(g_49c268, idx->field_10, g_49fc08 | 0x10000, idx->field_10);
                        if (v1 < 0)
                        {
                            sub_42b2b1(idx->field_10);
                        }
                        else
                        {
                            v9 = v1;
                            if (fstat(v1, &v8))
                            {
                                sub_42b444(idx->field_10);
                            }
                            else
                            {
                                v7 = (struct_3)sub_460cc6(idx->field_10);
                                idx->field_30 = idx->field_30 + 1;
                                sub_42e1bf(&v7, idx);
                            }
                        }
                    }
                    sub_439d1c(&v7);
                }
            }
        }
        idx = idx->field_0;
    }
    g_49f6f8 = sub_42e071(g_49f6f8, iter, sub_42e18e);
    iter = 0;
    v4 = sub_452c1c(0, 0, sub_42e590, sub_42e5c5, 0);
    for (idx = g_49f6f8; idx; idx = v5)
    {
        v5 = idx->field_0;
        idx->field_58 = sub_429aac(idx->field_28, idx->field_10);
        if (v3 && !(v6 = (struct_2 *)(unsigned long long)sub_452567(v4, idx, idx), !v6))
        {
            if (!idx->field_40)
            {
                if (v6->field_48)
                    sub_42e60b(v6->field_48, idx);
                sub_4539cd(v4, idx, idx);
                sub_42d817(v6);
                sub_42c3b1(v6);
                iter -= 1;
            }
            else
            {
                if (idx->field_48)
                    sub_42e60b(idx->field_48, v6);
                sub_42d817(idx);
                sub_42c3b1(idx);
                goto LABEL_42ec33;
            }
        }
        idx->field_30 = 0;
        if (!sub_45395d(v4, idx, idx))
            sub_41783c(); /* do not return */
        v3 = idx;
        iter += 1;
LABEL_42ec33:
    }
    g_49f700 = v3;
    sub_452e9c(v4);
    g_49f6f8 = sub_42e071(g_49f6f8, iter, sub_42e0df);
    if (!g_49f8e0)
        return;
    for (idx = g_49f6f8; idx && !idx->field_10->field_0; idx = idx->field_0);
    if (!idx)
        return;
    sub_4228cd(idx->field_38);
    return;
}



// Function: name_scan @ 0x2ed0e
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[8];
    struct struct_1 *field_10;
    char padding_18[8];
    unsigned int field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern struct_0 *g_49f6f8;
extern char g_49f958;

struct_0 * name_scan(unsigned long a0)
{
    unsigned long len;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    len = strlen(a0);
    while (1)
    {
        v1 = sub_42d7a6(a0, len);
        if (v1)
            return v1;
        if (!g_49f958 || !g_49f6f8 || !g_49f6f8[3].field_0)
            break;
        sub_42d3b1();
        if (g_49f6f8[3].field_0)
            return NULL;
    }
    return NULL;
}



// Function: name_from_list @ 0x2eda0
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

extern struct_0 *g_49f6b0;
extern unsigned long long g_49f6f8;

struct_0 * name_from_list(void)
{
    if (!g_49f6b0)
        g_49f6b0 = g_49f6f8;
    for (; g_49f6b0 && (g_49f6b0[3].field_0 || !(char)g_49f6b0[1].field_0->field_0); g_49f6b0 = g_49f6b0->field_0);
    if (!g_49f6b0)
        return NULL;
    g_49f6b0[3].field_0 = (char *)&g_49f6b0[3].field_0->field_0 + 1;
    sub_42adf1((unsigned int)g_49f6b0[2].field_8);
    return g_49f6b0;
}



// Function: blank_name_list @ 0x2ee43
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[40];
    unsigned long long field_30;
} struct_0;

extern unsigned long long g_49f6b0;
extern struct_0 *g_49f6f8;

struct_0 * blank_name_list(void)
{
    struct_0 *v2;  // rax
    struct_0 *i;  // [bp-0x10]

    g_49f6b0 = 0;
    v2 = g_49f6f8;
    for (i = g_49f6f8; i; i = i->field_0)
    {
        i->field_30 = 0;
    }
    return v2;
}



// Function: make_file_name @ 0x2ee85
void* make_file_name(void* ptr, void* a1)
{
    int v5;  // eax
    int v0;  // [bp-0x24]
    unsigned long len;  // [bp-0x20]
    unsigned long n;  // [bp-0x18]
    void* v3;  // [bp-0x10]

    len = strlen(ptr);
    n = strlen(a1) + 1;
    if (len && *((char *)ptr + len - 1) != 47)
        v5 = 1;
    else
        v5 = 0;
    v0 = v5;
    v3 = sub_460b5b(n + v0 + len);
    memcpy(v3, ptr, len);
    *(len + (char *)v3) = 47;
    memcpy(v0 + len + v3, a1, n);
    return v3;
}



// Function: stripped_prefix_len @ 0x2ef5c
char * stripped_prefix_len(char *a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x28]
    char v1;  // [bp-0x11]
    char *iter;  // [bp-0x10]

    v0 = a1;
    for (iter = a0; *(iter) == 47; iter += 1);
    while (1)
    {
        do
        {
            if (!*(iter))
                return 0xffffffffffffffff;
        } while ((v1 = (char)(char)(*(iter) == 47), iter += 1, !v1));
        v0 -= 1;
        if (!v0)
            break;
        for (; *(iter) == 47; iter += 1);
    }
    return iter - a0;
}



// Function: contains_dot_dot @ 0x2efdc
unsigned int contains_dot_dot(char *a0)
{
    char *i;  // [bp-0x10]
    char *v1;  // [bp-0x10]

    for (i = a0; *(i) != 46 || i[1] != 46 || i[2] != 47 && i[2]; i = v1 + 1)
    {
        while (1)
        {
            v1 = i;
            if (*(v1) == 47)
                break;
            i = v1 + 1;
            if (!*(v1))
                return 0;
        }
    }
    return 1;
}



// Function: dump_zeros @ 0x2f05d
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

extern void g_483d00;

unsigned int dump_zeros(struct_0 *idx, long long a1)
{
    unsigned long v3;  // rax
    unsigned long v0;  // [bp-0x18]
    unsigned long count;  // [bp-0x10]

    if (a1 < idx->field_8)
    {
        *(__errno_location()) = 22;
        return 0;
    }
    while (1)
    {
        if (a1 <= idx->field_8)
            return 1;
        v3 = a1 - idx->field_8;
        if (0x200 < v3)
            v3 = 0x200;
        v0 = v3;
        count = write(idx->field_0, &g_483d00, v0);
        if (count <= 0)
            break;
        idx->field_8 = idx->field_8 + count;
    }
    if (count)
        return 0;
    *(__errno_location()) = 22;
    return 0;
}



// Function: tar_sparse_member_p @ 0x2f126
typedef struct struct_0 {
    char padding_0[16];
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_member_p(struct_2 *a0)
{
    return (!a0->field_20->field_10 ? 0 : a0->field_20->field_10(a0));
}



// Function: tar_sparse_init @ 0x2f165
typedef struct struct_1 {
    char padding_0[32];
    struct struct_2 *field_20;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

unsigned long long tar_sparse_init(struct_1 *a0)
{
    memset(a0, 0, 48);
    return (!((char)sub_42fa5c(a0) ^ 1) ? (!a0->field_20->field_0 ? 1 : a0->field_20->field_0(a0)) : 0);
}



// Function: tar_sparse_done @ 0x2f1d2
typedef struct struct_0 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_done(struct_2 *a0)
{
    return (!a0->field_20->field_8 ? 1 : a0->field_20->field_8(a0));
}



// Function: tar_sparse_scan @ 0x2f211
typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_0 {
    char padding_0[48];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_scan(struct_2 *a0, unsigned int a1, unsigned long a2)
{
    return (!a0->field_20->field_30 ? 1 : a0->field_20->field_30(a0, a1, a2, a1, a0->field_20->field_30));
}



// Function: tar_sparse_dump_region @ 0x2f261
typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_0 {
    char padding_0[56];
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_dump_region(struct_2 *a0, unsigned long a1)
{
    return (!a0->field_20->field_38 ? 0 : a0->field_20->field_38(a0, a1, a1, a0->field_20->field_38));
}



// Function: tar_sparse_extract_region @ 0x2f2ab
typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_0 {
    char padding_0[64];
    struct struct_1 *field_40;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_extract_region(struct_2 *a0, unsigned long a1)
{
    return (!a0->field_20->field_40 ? 0 : a0->field_20->field_40(a0, a1, a1, a0->field_20->field_40));
}



// Function: tar_sparse_dump_header @ 0x2f2f5
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_dump_header(struct_2 *a0)
{
    return (!a0->field_20->field_18 ? 0 : a0->field_20->field_18(a0));
}



// Function: tar_sparse_decode_header @ 0x2f334
typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_decode_header(struct_2 *a0)
{
    return (!a0->field_20->field_28 ? 1 : a0->field_20->field_28(a0));
}



// Function: tar_sparse_fixup_header @ 0x2f373
typedef struct struct_0 {
    char padding_0[32];
    struct struct_1 *field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long tar_sparse_fixup_header(struct_0 *a0)
{
    return (!a0->field_20[4].field_0 ? 1 : a0->field_20[4].field_0(a0));
}



// Function: lseek_or_error @ 0x2f3b2
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

unsigned int lseek_or_error(struct_0 *a0, long long a1)
{
    if (!(!a0->padding_4[0] ? (char)sub_42f05d(a0, a1) ^ 1 : (char)(lseek(a0->field_0, a1, 0) >> 63)))
        return 1;
    sub_42b3eb(*((long long *)a0[1].field_8), a1);
    return 0;
}



// Function: zero_block_p @ 0x2f433
unsigned int zero_block_p(char *a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x18]
    char *v2;  // [bp-0x10]
    char *v3;  // [bp-0x10]

    v2 = a0;
    v0 = a1;
    while (1)
    {
        v3 = v2;
        v1 = v0 - 1;
        if (!v0)
            return 1;
        v2 = v3 + 1;
        v0 = v1;
        if (*(v3))
            return 0;
    }
}



// Function: sparse_add_map @ 0x2f477
unsigned long long * sparse_add_map(unsigned long long *idx, unsigned long long *a1)
{
    unsigned long long *v3;  // rcx
    unsigned long long v4;  // rdx
    unsigned long v0;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    v0 = idx[40];
    v1 = idx[38];
    if (v1 == idx[39])
    {
        v0 = sub_460a5f(v0, idx + 39, 16, idx + 39);
        idx[40] = v0;
    }
    v3 = v1 * 16 + v0;
    v4 = a1[1];
    *(v3) = *(a1);
    v3[1] = v4;
    idx[38] = v1 + 1;
    return idx;
}



// Function: sparse_scan_file_raw @ 0x2f527
typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long sparse_scan_file_raw(struct_2 *a0)
{
    unsigned int v0;  // [bp-0x244]
    unsigned long long v1;  // [bp-0x240]
    struct_0 *idx;  // [bp-0x238]
    unsigned long long v3;  // [bp-0x230]
    unsigned long long v4;  // [bp-0x228]
    unsigned long long v5;  // [bp-0x220]
    char v6[520];  // [bp-0x218]

    idx = *((long long *)&a0->padding_0[24]);
    v0 = *((int *)&a0->padding_0[0]);
    v3 = 0;
    v1 = 0;
    v4 = 0;
    v5 = 0;
    idx->field_118 = 0;
    if ((char)sub_42f211(a0, 0, 0) != 1)
        return 0;
    while (1)
    {
        v3 = sub_42a9e9(v0, v6, 0x200);
        if (!v3 || v3 == 0xffffffffffffffff)
            break;
        if (!(char)sub_42f433(v6, v3))
        {
            if (!v5)
                v4 = v1;
            v5 += v3;
            idx->field_118 = v3 + idx->field_118;
            if ((char)sub_42f211(a0, 1, v6) != 1)
                return 0;
        }
        else if (v5)
        {
            sub_42f477(idx, &v4);
            v5 = 0;
            if ((char)sub_42f211(a0, 1, 0) != 1)
                return 0;
        }
        v1 += v3;
    }
    if (!v5)
        v4 = v1;
    sub_42f477(idx, &v4);
    idx->field_118 = v3 + idx->field_118;
    return sub_42f211(a0, 2, 0);
}



// Function: sparse_scan_file_wholesparse @ 0x2f7c6
typedef struct struct_0 {
    char padding_0[136];
    unsigned long long field_88;
    char padding_90[8];
    unsigned long long field_98;
    char padding_a0[120];
    unsigned long long field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

unsigned int sparse_scan_file_wholesparse(struct_1 *a0)
{
    struct_0 *idx;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]

    idx = a0->field_18;
    v1 = 0;
    v2 = 0;
    if (idx->field_98)
        return 0;
    idx->field_118 = 0;
    v1 = idx->field_88;
    sub_42f477(idx, &v1);
    return 1;
}



// Function: sparse_scan_file_seek @ 0x2f864
typedef struct struct_0 {
    char padding_0[136];
    unsigned long long field_88;
    char padding_90[136];
    unsigned long long field_118;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[20];
    struct struct_0 *field_18;
} struct_1;

unsigned int sparse_scan_file_seek(struct_1 *a0)
{
    unsigned int v0;  // [bp-0x4c]
    unsigned long long v1;  // [bp-0x48]
    struct_0 *idx;  // [bp-0x40]
    unsigned long long v3;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x30]
    unsigned long long v5;  // [bp-0x28]
    unsigned long long v6;  // [bp-0x20]

    idx = a0->field_18;
    v0 = a0->field_0;
    v5 = 0;
    v6 = 0;
    v1 = 0;
    idx->field_118 = 0;
    while (1)
    {
        v3 = lseek(v0, v1, 3);
        if (v3 == 0xffffffffffffffff)
            break;
        v4 = lseek(v0, v3, 4);
        if (!v1 && !v3 && v4 == idx->field_88)
        {
            lseek(v0, 0, 0);
            return 0;
        }
        v5 = v3;
        v6 = v4 - v3;
        sub_42f477(idx, &v5);
        idx->field_118 = idx->field_118 + v6;
        v1 = v4;
    }
    if (*(__errno_location()) != 6)
        return 0;
    v6 = 0;
    v5 = idx->field_88;
    sub_42f477(idx, &v5);
    return 1;
}



// Function: sparse_scan_file @ 0x2f9e5
typedef struct struct_2 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_2;

typedef struct struct_0 {
    char padding_0[48];
    struct struct_1 *field_30;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned int g_49f984;

unsigned int sparse_scan_file(struct_2 *a0)
{
    if ((char)sub_42f7c6(a0))
        return 1;
    if (g_49f984 == 2)
    {
LABEL_42fa27:
        if ((char)sub_42f864(a0))
            return 1;
LABEL_42fa3e:
        if ((char)sub_42f527(a0))
            return 1;
    }
    else if (g_49f984 <= 2)
    {
        if (!g_49f984)
            goto LABEL_42fa27;
        if (g_49f984 == 1)
            goto LABEL_42fa3e;
    }
    return 0;
}



// Function: sparse_select_optab @ 0x2fa5c
extern char g_49a7c0;
extern char g_49a820;
extern char g_49a880;
extern unsigned int g_49f468;
extern unsigned int g_49f864;

unsigned int sparse_select_optab(unsigned long idx)
{
    switch ((!g_49f468 ? g_49f864 : g_49f468))
    {
    case 1: case 3:
        return 0;
    case 2: case 6:
        *((char **)(idx + 32)) = &g_49a7c0;
        break;
    case 4:
        *((char **)(idx + 32)) = &g_49a880;
        break;
    case 5:
        *((char **)(idx + 32)) = &g_49a820;
        break;
    default:
        return 0;
    }
    return 1;
}



// Function: sparse_dump_region @ 0x2faf0
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
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

extern unsigned int g_49ec84;
extern char g_49f8bc;
extern struct_0 *g_49fdc8;

unsigned int sparse_dump_region(struct_0 *idx, unsigned long a1)
{
    long long v8;  // rax
    unsigned int v9;  // eax
    long long v0;  // [bp-0xf0]
    long long v1;  // [bp-0xe8]
    long long v2;  // [bp-0xe0]
    unsigned long v3;  // [bp-0xd8]
    unsigned long long v4;  // [bp-0xd0]
    stat v5;  // [bp-0xc8]
    char v6;  // [bp-0x38]

    v0 = *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)) + 8));
    if ((char)sub_42f3b2(idx, *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320))))) != 1)
        return 0;
    while (1)
    {
        if (v0 <= NULL)
            return 1;
        if (0x200 < v0)
            v8 = 0x200;
        else
            v8 = v0;
        v2 = v8;
        v3 = sub_40bb4b();
        v4 = sub_45ec70(idx->field_0, v3, v2, v3);
        if (v4 == 0xffffffffffffffff)
        {
            sub_42b2f8(*((long long *)idx[1].field_8), *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)) + 8)) + *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)))) - v0, v2);
            return 0;
        }
        if (!v4)
            break;
        memset(v3 + v4, 0, 0x200 - v4);
        v0 -= v4;
        *((unsigned long long *)&idx[1].field_0) = v4 + *((long long *)&idx[1].field_0);
        sub_40bba7(v3);
    }
    v9 = fstat(idx->field_0, &v5);
    v1 = (!v9 ? *((long long *)(idx[1].field_8 + 0x88)) - v5.st_atime : *((long long *)(idx[1].field_8 + 0x88)) - (*((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)) + 8)) + *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)))) - v0));
    v1 = v1;
    if ((char)g_49ec84 & 128)
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_454e5a(v1, &v6, &v6);
        sub_45e548(*((long long *)idx[1].field_8));
        error(0, 0, ngettext("%s: File shrank by %s byte; padding with zeros", "%s: File shrank by %s bytes; padding with zeros", v1));
    }
    if (g_49f8bc == 1)
        return 0;
    sub_439e98(1);
    return 0;
}



// Function: sparse_extract_region @ 0x2fe94
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int sparse_extract_region(struct_0 *idx, unsigned long a1)
{
    long long v5;  // rax
    long long i;  // [bp-0x28]
    long long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    long long v3;  // [bp-0x10]

    if ((char)sub_42f3b2(idx, *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320))))) != 1)
        return 0;
    i = *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)) + 8));
    if (i)
    {
        while (i > 0)
        {
            if (0x200 < i)
                v5 = 0x200;
            else
                v5 = i;
            v1 = v5;
            v2 = sub_40bb4b();
            if (!v2)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Unexpected EOF in archive"));
                g_49fdd0 = 2;
                return 0;
            }
            else
            {
                sub_40bba7(v2);
                *((unsigned long long *)&idx[1].field_0) = *((long long *)&idx[1].field_0) + 0x200;
                v3 = sub_42aa95(idx->field_0, v2, v1);
                i -= v3;
                sub_40ad86(*((long long *)(idx[1].field_8 + 280)) - *((long long *)&idx[1].field_0));
                idx->field_8 = v3 + idx->field_8;
                if (v3 != v1)
                {
                    sub_43eb4b(*((long long *)idx[1].field_8), v3, v1, v3);
                    return 0;
                }
            }
        }
    }
    else if (idx->padding_4[0] && sub_432f52(idx->field_0))
    {
        sub_43ea7e(*((long long *)idx[1].field_8));
    }
    return 1;
}



// Function: sparse_dump_file @ 0x30090
typedef struct struct_1 {
    char padding_0[32];
    struct struct_2 *field_20;
} struct_1;

typedef struct struct_0 {
    char padding_0[56];
    unsigned long long field_38;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

unsigned int sparse_dump_file(unsigned int a0, void* a1)
{
    char v0;  // [bp-0x51]
    unsigned long i;  // [bp-0x50]
    struct_1 v2;  // [bp-0x48]
    char v3;  // [bp-0x44]
    unsigned long v4;  // [bp-0x38]
    struct_0 *v5;  // [bp-0x28]

    if ((char)sub_42f165(&v2) != 1)
        return 3;
    v2 = (struct_1)a0;
    v3 = 1;
    v0 = sub_42f9e5(&v2);
    if (v0 && v5->field_38)
    {
        sub_42f2f5(&v2);
        if (a0 >= 0)
        {
            sub_40ab09((long long)a1[8], (long long)a1[0x88], (long long)a1[280] - v4);
            for (i = 0; v0 && i < (long long)a1[304]; i += 1)
            {
                v0 = sub_42f261(&v2, i);
            }
        }
    }
    sub_4142d7((long long)a1[280] - v4);
    if (!(char)sub_42f1d2(&v2) || !v0)
        return 1;
    return 0;
    return 1;
}



// Function: sparse_member_p @ 0x301de
unsigned long long sparse_member_p(unsigned long long a0)
{
    char v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x30]

    if ((char)sub_42f165(&v0) == 1)
    {
        v1 = a0;
        return sub_42f126(&v0);
    }
    return 0;
}



// Function: sparse_fixup_header @ 0x30241
unsigned long long sparse_fixup_header(unsigned long long a0)
{
    char v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x30]

    if ((char)sub_42f165(&v0) == 1)
    {
        v1 = a0;
        return sub_42f373(&v0);
    }
    return 0;
}



// Function: sparse_extract_file @ 0x302a4
typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[32];
    struct struct_2 *field_20;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

unsigned int sparse_extract_file(unsigned int a0, struct_0 *a1, unsigned long long *a2)
{
    char v0;  // [bp-0x51]
    unsigned long long i;  // [bp-0x50]
    struct_1 v2;  // [bp-0x48]
    char v3;  // [bp-0x44]
    unsigned long long v4;  // [bp-0x40]
    unsigned long v5;  // [bp-0x38]

    v0 = 1;
    if ((char)sub_42f165(&v2) != 1)
        return 3;
    v2 = (struct_1)a0;
    v3 = !lseek(a0, 0, 0);
    v4 = 0;
    v0 = sub_42f334(&v2);
    for (i = 0; v0 && i < *((long long *)&a1[1].padding_0[16]); i += 1)
    {
        v0 = sub_42f2ab(&v2, i);
    }
    *(a2) = a1->field_118 - v5;
    if (!(char)sub_42f1d2(&v2) || !v0)
        return 1;
    return 0;
    return 1;
}



// Function: sparse_skip_file @ 0x303bd
typedef struct struct_0 {
    char padding_0[280];
    unsigned long long field_118;
} struct_0;

typedef struct struct_1 {
    char padding_0[32];
    struct struct_2 *field_20;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

unsigned int sparse_skip_file(struct_0 *a0)
{
    char v0;  // [bp-0x49]
    struct_1 v1;  // [bp-0x48]
    unsigned long v2;  // [bp-0x38]

    v0 = 1;
    if ((char)sub_42f165(&v1) != 1)
        return 3;
    v1 = (struct_1)0xffffffff;
    v0 = sub_42f334(&v1);
    sub_4287c1(a0->field_118 - v2);
    if (!(char)sub_42f1d2(&v1) || !v0)
        return 1;
    return 0;
    return 1;
}



// Function: check_sparse_region @ 0x3046a
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

unsigned int check_sparse_region(struct_0 *a0, long long a1, long long a2)
{
    long long v6;  // rax
    unsigned long v7;  // rax
    unsigned long long v8;  // rcx
    unsigned long long v9;  // r8
    unsigned long long v10;  // r9
    unsigned long long v11;  // rax
    unsigned long v12;  // rax
    unsigned long long v13;  // r8
    unsigned long long v14;  // r9
    long long i;  // [bp-0x268]
    long long v1;  // [bp-0x258]
    unsigned long long v2;  // [bp-0x250]
    char v3;  // [bp-0x248]
    char v4[520];  // [bp-0x228]

    if ((char)sub_42f3b2(a0, i) != 1)
        return 0;
    for (i = a1; i < a2; i += v2)
    {
        v6 = a2 - i;
        if (0x200 < v6)
            v6 = 0x200;
        v1 = v6;
        v2 = sub_45ec70();
        if (v2 == 0xffffffffffffffff)
        {
            sub_42b2f8(*((long long *)a0[1].field_8), i, v1);
            return 0;
        }
        if (!v2)
        {
            v7 = gettext("Size differs");
            sub_4107be(a0[1].field_8, v7, v7, v8, v9, v10);
            return 0;
        }
        if ((char)sub_42f433(v4, v2) != 1)
        {
            v11 = sub_454d1c(i, &v3, &v3);
            v12 = gettext("File fragment at %s is not a hole");
            sub_4107be(a0[1].field_8, v12, v11, v12, v13, v14);
            return 0;
        }
    }
    return 1;
}



// Function: check_data_region @ 0x3064f
typedef struct struct_0 {
    int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

extern struct_0 g_49fa20;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int check_data_region(struct_0 *idx, unsigned long a1)
{
    long long v6;  // rax
    unsigned long long v16;  // r9
    unsigned long long v9;  // rdx
    unsigned long long v10;  // rcx
    unsigned long long v11;  // r8
    unsigned long long v12;  // r9
    unsigned long v13;  // rax
    unsigned long long v14;  // rcx
    unsigned long long v15;  // r8
    long long v0;  // [bp-0x238]
    long long v1;  // [bp-0x230]
    unsigned long v2;  // [bp-0x228]
    unsigned long v3;  // [bp-0x220]
    char v4;  // [bp-0x218]

    if ((char)sub_42f3b2(idx, *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320))))) != 1)
        return 0;
    v0 = *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)) + 8));
    sub_40ad86(*((long long *)(idx[1].field_8 + 280)) - *((long long *)&idx[1].field_0));
    while (1)
    {
        if (v0 <= NULL)
            return 1;
        if (0x200 < v0)
            v6 = 0x200;
        else
            v6 = v0;
        v1 = v6;
        v2 = sub_40bb4b();
        if (!v2)
            break;
        sub_40bba7(v2);
        *((unsigned long long *)&idx[1].field_0) = *((long long *)&idx[1].field_0) + 0x200;
        v3 = sub_45ec70(idx->field_0, &v4, v1, &v4);
        if (v3 == 0xffffffffffffffff)
        {
            sub_42b2f8(*((long long *)idx[1].field_8), *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)) + 8)) + *((long long *)(a1 * 16 + *((long long *)(idx[1].field_8 + 320)))) - v0, v1);
            return 0;
        }
        else if (!v3)
        {
            sub_4107be(&g_49fa20.field_0, gettext("Size differs"), v9, v10, v11, v12);
            return 0;
        }
        else
        {
            v0 -= v3;
            sub_40ad86(*((long long *)(idx[1].field_8 + 280)) - *((long long *)&idx[1].field_0));
            if (memcmp(v2, &v4, v3))
            {
                v13 = gettext("Contents differ");
                sub_4107be(idx[1].field_8, v13, v13, v14, v15, v16);
                return 0;
            }
        }
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Unexpected EOF in archive"));
    g_49fdd0 = 2;
    return 0;
}



// Function: sparse_diff_file @ 0x30972
typedef struct struct_1 {
    char padding_0[32];
    struct struct_2 *field_20;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

unsigned int sparse_diff_file(unsigned int a0, struct_0 *idx)
{
    char v7;  // al
    char v0;  // [bp-0x59]
    unsigned long index;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    struct_1 v3;  // [bp-0x48]
    char v4;  // [bp-0x44]
    unsigned long v5;  // [bp-0x38]

    v0 = 1;
    v2 = 0;
    if ((char)sub_42f165(&v3) != 1)
        return 0;
    v3 = (struct_1)a0;
    v4 = 1;
    v0 = sub_42f334(&v3);
    sub_40ad2b(idx);
    for (index = 0; v0 && index < idx[38].field_0; index += 1)
    {
        if ((char)sub_43046a(&v3, v2, *((long long *)(index * 16 + idx[40].field_0))) && (char)sub_43064f(&v3, index))
            v7 = 1;
        else
            v7 = 0;
        v0 = v7;
        v0 &= 1;
        v2 = *((long long *)(index * 16 + idx[40].field_0 + 8)) + *((long long *)(index * 16 + idx[40].field_0));
    }
    if (v0 != 1)
        sub_4287c1(idx[35].field_0 - v5);
    sub_40ad66();
    sub_42f1d2(&v3);
    return v0;
}



// Function: oldgnu_sparse_member_p @ 0x30afb
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f460;

long long oldgnu_sparse_member_p(unsigned long a0)
{
    unsigned long v1;  // rax

    v1 = *((char *)&g_49f460[9].field_8 + 4);
    return _INSERT(v1, 0, (char)v1 == 83);
}



// Function: oldgnu_add_sparse @ 0x30b1c
typedef struct struct_0 {
    char padding_0[12];
    char field_c;
} struct_0;

typedef struct struct_2 {
    char padding_0[136];
    long long field_88;
    char padding_90[136];
    long long field_118;
} struct_2;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_2 *field_18;
} struct_1;

unsigned int oldgnu_add_sparse(struct_1 *a0, struct_0 *a1)
{
    char v3;  // cl
    unsigned long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]

    if (!a1->field_c)
        return 1;
    v0 = sub_42798b(a1, 12);
    v1 = sub_42798b(&a1->field_c, 12);
    if (v0 < 0 || v1 < 0)
        return 2;
    v3 = 0;
    if (__OFADD__(v1, v0) & 1)
        v3 = 1;
    if (!(v3 & 1) && a0->field_18->field_88 >= v1 + v0 && a0->field_18->field_118 >= 0)
    {
        sub_42f477(a0->field_18, &v0);
        return 0;
    }
    return 2;
}



// Function: oldgnu_fixup_header @ 0x30c22
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
} struct_0;

extern unsigned long long g_49f460;

unsigned long long oldgnu_fixup_header(struct_0 *idx)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x10]

    v0 = sub_42798b(g_49f460 + 483, 12);
    *((long long *)&idx->field_18[280]) = (long long)idx->field_18[0x88];
    if (v0 < 0)
        v2 = 0;
    else
        v2 = v0;
    *((unsigned long *)&idx->field_18[0x88]) = v2;
    return ~(v0) >> 63;
}



// Function: oldgnu_get_sparse_info @ 0x30c9a
typedef struct struct_2 {
    char padding_0[136];
    long long field_88;
    char padding_90[136];
    long long field_118;
} struct_2;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_2 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[482];
    char field_1e2;
} struct_0;

extern struct_0 *g_49f460;
extern struct_2 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int oldgnu_get_sparse_info(struct_1 *a0)
{
    unsigned int j;  // [bp-0x30]
    unsigned int v1;  // [bp-0x2c]
    unsigned long long iter;  // [bp-0x28]
    struct_0 *v3;  // [bp-0x20], Other Possible Types: struct_1 *

    v3 = g_49f460;
    *((unsigned long *)&a0->field_18[1].padding_0[16]) = 0;
    for (iter = 0; iter <= 3 && (v1 = (unsigned int)(unsigned long long)sub_430b1c(a0, v3 + iter * 24 + 384 + 2), !v1); iter += 1);
    for (j = v3->field_1e2; !v1 && j; j = (char)v3[15].field_18)
    {
        v3 = sub_40bb4b();
        if (!v3)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Unexpected EOF in archive"));
            g_49fdd0 = 2;
            return 0;
        }
        sub_40bba7(v3);
        for (iter = 0; iter <= 20 && !v1; iter += 1)
        {
            v1 = sub_430b1c(a0, &(v3->padding_0)[iter]);
        }
    }
    if (v1 != 2)
        return 1;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("%s: invalid sparse archive member"));
    g_49fdd0 = 2;
    return 0;
}



// Function: oldgnu_store_sparse_info @ 0x30e70
typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[320];
    unsigned long long field_140;
} struct_0;

long long oldgnu_store_sparse_info(struct_1 *a0, unsigned long long *a1, char *a2, unsigned long long a3)
{
    unsigned long long i;  // [bp-0x28]
    char *v1;  // [bp-0x20]

    v1 = a2;
    for (i = a3; *(a1) < *((long long *)&a0->field_18->padding_0[304]) && i; *(a1) = *(a1) + 1)
    {
        sub_412a9a(*((long long *)(*(a1) * 16 + a0->field_18->field_140)), v1, 12);
        sub_412a9a(*((long long *)(*(a1) * 16 + a0->field_18->field_140 + 8)), v1 + 12, 12);
        i -= 1;
        v1 += 24;
    }
    return *((long long *)&a0->field_18->padding_0[304]);
}



// Function: oldgnu_dump_header @ 0x30f43
typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[320];
    unsigned long long field_140;
} struct_0;

unsigned int oldgnu_dump_header(struct_1 *a0)
{
    unsigned long i;  // [bp-0x28]
    long long v1;  // [bp-0x20]
    char *ptr;  // [bp-0x18], Other Possible Types: unsigned long

    v1 = sub_40bacd();
    ptr = &sub_41381d(a0->field_18)->padding_0[0];
    ptr[156] = 83;
    if (*((long long *)&a0->field_18->padding_0[304]) > 4)
        ptr[482] = 1;
    sub_412a9a(*((long long *)&a0->field_18->padding_0[0x88]), ptr + 483, 12);
    sub_412a9a(*((long long *)&a0->field_18->padding_0[280]), ptr + 124, 12);
    i = 0;
    sub_430e70(a0, &i, ptr + 386, 4);
    ptr[482] = _INSERT(i, 0, *((long long *)&a0->field_18->padding_0[304]) > i) & 0xffffffff;
    sub_41422e(a0->field_18, ptr, v1);
    while (*((long long *)&a0->field_18->padding_0[304]) > i)
    {
        ptr = sub_40bb4b();
        memset(ptr, 0, 0x200);
        sub_430e70(a0, &i, ptr, 21);
        if (*((long long *)&a0->field_18->padding_0[304]) > i)
            *((char *)(ptr + 504)) = 1;
        sub_40bba7(ptr);
    }
    return 1;
}



// Function: star_sparse_member_p @ 0x310ff
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49f460;

long long star_sparse_member_p(unsigned long a0)
{
    unsigned long v1;  // rax

    v1 = *((char *)&g_49f460[9].field_8 + 4);
    return _INSERT(v1, 0, (char)v1 == 83);
}



// Function: star_fixup_header @ 0x31120
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
} struct_0;

extern unsigned long long g_49f460;

unsigned long long star_fixup_header(struct_0 *idx)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x10]

    v0 = sub_42798b(g_49f460 + 452, 12);
    *((long long *)&idx->field_18[280]) = (long long)idx->field_18[0x88];
    if (v0 < 0)
        v2 = 0;
    else
        v2 = v0;
    *((unsigned long *)&idx->field_18[0x88]) = v2;
    return ~(v0) >> 63;
}



// Function: star_get_sparse_info @ 0x31198
typedef struct struct_2 {
    char padding_0[136];
    long long field_88;
    char padding_90[136];
    long long field_118;
} struct_2;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_2 *field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[345];
    char field_159;
    char padding_15a[9];
    char field_163;
    char padding_164[10];
    char field_16e;
} struct_0;

extern struct_0 *g_49f460;
extern struct_2 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int star_get_sparse_info(struct_1 *idx)
{
    unsigned int j;  // [bp-0x30]
    unsigned int v1;  // [bp-0x2c]
    unsigned long long iter;  // [bp-0x28]
    struct_0 *v3;  // [bp-0x20], Other Possible Types: struct_1 *

    v3 = g_49f460;
    v1 = 0;
    *((unsigned long *)&idx->field_18[1].padding_0[16]) = 0;
    if (!v3->field_159 && v3->field_16e)
    {
        for (iter = 0; iter <= 3 && (v1 = (unsigned int)(unsigned long long)sub_430b1c(idx, v3 + iter * 24 + 352 + 4), !v1); iter += 1);
        j = v3->field_163;
    }
    else
    {
        j = 1;
    }
    for (; !v1 && j; j = (char)v3[15].field_18)
    {
        v3 = sub_40bb4b();
        if (!v3)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Unexpected EOF in archive"));
            g_49fdd0 = 2;
            return 0;
        }
        sub_40bba7(v3);
        for (iter = 0; iter <= 20 && !v1; iter += 1)
        {
            v1 = sub_430b1c(idx, &(v3->padding_0)[iter]);
        }
        *((unsigned long long *)&idx->padding_0[16]) = *((long long *)&idx->padding_0[16]) + 0x200;
    }
    if (v1 != 2)
        return 1;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("%s: invalid sparse archive member"));
    g_49fdd0 = 2;
    return 0;
}



// Function: pax_sparse_member_p @ 0x313b3
typedef struct struct_0 {
    char padding_0[292];
    unsigned int field_124;
    char padding_128[8];
    unsigned long long field_130;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

unsigned int pax_sparse_member_p(struct_1 *a0)
{
    unsigned int v1;  // eax

    if (a0->field_18->field_130 || a0->field_18->field_124)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: pax_start_header @ 0x313f6
typedef struct struct_0 {
    char padding_0[156];
    char field_9c;
} struct_0;

struct_0 * pax_start_header(unsigned long long *idx)
{
    unsigned long v0;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v0 = idx[0x11];
    idx[0x11] = idx[35];
    v1 = sub_41381d(idx);
    idx[0x11] = v0;
    return v1;
}



// Function: pax_dump_header_0 @ 0x31450
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    struct struct_0 *field_18;
} struct_1;

extern unsigned int g_49f980;

unsigned int pax_dump_header_0(struct_1 *idx)
{
    void* v7;  // rax
    void* v8;  // rax
    void* v9;  // rax
    unsigned long long iter;  // [bp-0x60]
    void* v1;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    unsigned long v3;  // [bp-0x48]
    struct_0 *v4;  // [bp-0x40]
    char v5;  // [bp-0x38]

    v2 = sub_40bacd();
    v3 = idx->field_18->field_140;
    v1 = NULL;
    sub_41e1d3("GNU.sparse.size", idx->field_18, 0);
    sub_41e1d3("GNU.sparse.numblocks", idx->field_18, 0);
    if ((char)sub_41c63f("GNU.sparse.map") || !g_49f980)
    {
        for (iter = 0; idx->field_18->field_130 > iter; iter += 1)
        {
            sub_41e1d3("GNU.sparse.offset", idx->field_18, &iter);
            sub_41e1d3("GNU.sparse.numbytes", idx->field_18, &iter);
        }
    }
    else
    {
        sub_41e1d3("GNU.sparse.name", idx->field_18, 0);
        v1 = idx->field_18->field_8;
        v7 = sub_41cd02(idx->field_18, "%d/GNUSparseFile.%p/%f", 0);
        idx->field_18->field_8 = v7;
        sub_41e848(&idx->field_18->field_168);
        for (iter = 0; idx->field_18->field_130 > iter; iter += 1)
        {
            if (iter)
                sub_41e863(&idx->field_18->field_168, ",");
            v8 = sub_454e5a(*((long long *)(v3 + iter * 16)), &v5, *((long long *)(v3 + iter * 16)));
            sub_41e863(&idx->field_18->field_168, v8);
            sub_41e863(&idx->field_18->field_168, ",");
            v9 = sub_454e5a(*((long long *)(v3 + iter * 16 + 8)), &v5, *((long long *)(v3 + iter * 16 + 8)));
            sub_41e863(&idx->field_18->field_168, v9);
        }
        if ((char)sub_41e8dc(&idx->field_18->field_168, "GNU.sparse.map") != 1)
        {
            free(idx->field_18->field_8);
            idx->field_18->field_8 = v1;
            return 0;
        }
    }
    v4 = sub_4313f6(idx->field_18);
    sub_41422e(idx->field_18, v4, v2);
    if (v1)
    {
        free(idx->field_18->field_8);
        idx->field_18->field_8 = v1;
    }
    return 1;
}



// Function: pax_dump_header_1 @ 0x3179e
typedef struct struct_2 {
    char padding_0[16];
    unsigned long long field_10;
    struct struct_1 *field_18;
} struct_2;

typedef struct struct_1 {
    char padding_0[8];
    struct struct_0 *field_8;
    char padding_10[264];
    unsigned long long field_118;
    char padding_120[16];
    long long field_130;
    char padding_138[8];
    unsigned long long field_140;
} struct_1;

typedef struct struct_0 {
    char padding_0[100];
    char field_64;
} struct_0;

unsigned int pax_dump_header_1(struct_2 *idx)
{
    unsigned long v29;  // rax
    struct_0 *v30;  // rax
    char *v0;  // [bp-0xd8], Other Possible Types: struct_0 *
    char *ptr;  // [bp-0xd0]
    char *v2;  // [bp-0xd0]
    long long iter;  // [bp-0xc8]
    long long v4;  // [bp-0xc0], Other Possible Types: unsigned long long, unsigned long
    char *v5;  // [bp-0xb8]
    char *j;  // [bp-0xb0]
    char *v7;  // [bp-0xb0]
    char *v8;  // [bp-0xa8]
    char *k;  // [bp-0xa0]
    char *v10;  // [bp-0xa0]
    char *v11;  // [bp-0x98]
    char *l;  // [bp-0x90]
    char *v13;  // [bp-0x90]
    char *v14;  // [bp-0x88]
    char *m;  // [bp-0x80]
    char *v16;  // [bp-0x80]
    char *v17;  // [bp-0x78]
    char *n;  // [bp-0x70]
    char *v19;  // [bp-0x70]
    char *v20;  // [bp-0x68]
    char *i0;  // [bp-0x60]
    char *v22;  // [bp-0x60]
    long long v23;  // [bp-0x58]
    unsigned long v24;  // [bp-0x50]
    struct_0 *v25;  // [bp-0x48]
    char *v26;  // [bp-0x40]
    char v27;  // [bp-0x38]

    v23 = sub_40bacd();
    v4 = 0;
    v24 = idx->field_18->field_140;
    v25 = idx->field_18->field_8;
    v26 = sub_454e5a(idx->field_18->field_130, &v27, &v27);
    v4 = strlen(v26) + v4 + 1;
    for (iter = 0; iter < idx->field_18->field_130; iter += 1)
    {
        v26 = sub_454e5a(*((long long *)(v24 + iter * 16)), &v27, *((long long *)(v24 + iter * 16)));
        v4 = strlen(v26) + v4 + 1;
        v26 = sub_454e5a(*((long long *)(v24 + iter * 16 + 8)), &v27, *((long long *)(v24 + iter * 16 + 8)));
        v4 = strlen(v26) + v4 + 1;
    }
    v29 = v4 + 0x1ff;
    v4 = (long long)((v29 < 0 ? v29 + 0x1ff : v29)) >> 9;
    idx->field_18->field_118 = idx->field_18->field_118 + v4 * 0x200;
    idx->field_10 = idx->field_10 + v4 * 0x200;
    sub_41e1d3("GNU.sparse.major", idx->field_18, 0);
    sub_41e1d3("GNU.sparse.minor", idx->field_18, 0);
    sub_41e1d3("GNU.sparse.name", idx->field_18, 0);
    sub_41e1d3("GNU.sparse.realsize", idx->field_18, 0);
    v30 = sub_41cd02(idx->field_18, "%d/GNUSparseFile.%p/%f", 0);
    idx->field_18->field_8 = v30;
    if (strlen(idx->field_18->field_8) > 100)
        idx->field_18->field_8->field_64 = 0;
    v0 = sub_4313f6(idx->field_18);
    sub_41422e(idx->field_18, v0, v23);
    free(idx->field_18->field_8);
    idx->field_18->field_8 = v25;
    v0 = sub_40bb4b();
    ptr = v0;
    v26 = sub_454e5a(idx->field_18->field_130, &v27, &v27);
    v5 = v0 + 0x200;
    for (j = v26; *(j); j = v7)
    {
        if (ptr == v5)
        {
            sub_40bba7(v0);
            v0 = sub_40bb4b();
            ptr = v0;
            v5 = v0 + 0x200;
        }
        v7 = j + 1;
        v2 = ptr + 1;
        *(ptr) = *(j);
        ptr = v2;
    }
    v8 = v0 + 0x200;
    for (k = "\n"; *(k); k = v10)
    {
        if (ptr == v8)
        {
            sub_40bba7(v0);
            v0 = sub_40bb4b();
            ptr = v0;
            v8 = v0 + 0x200;
        }
        v10 = k + 1;
        v2 = ptr + 1;
        *(ptr) = *(k);
        ptr = v2;
    }
    for (iter = 0; iter < idx->field_18->field_130; iter += 1)
    {
        v26 = sub_454e5a(*((long long *)(v24 + iter * 16)), &v27, *((long long *)(v24 + iter * 16)));
        v11 = v0 + 0x200;
        for (l = v26; *(l); l = v13)
        {
            if (ptr == v11)
            {
                sub_40bba7(v0);
                v0 = sub_40bb4b();
                ptr = v0;
                v11 = v0 + 0x200;
            }
            v13 = l + 1;
            v2 = ptr + 1;
            *(ptr) = *(l);
            ptr = v2;
        }
        v14 = v0 + 0x200;
        for (m = "\n"; *(m); m = v16)
        {
            if (ptr == v14)
            {
                sub_40bba7(v0);
                v0 = sub_40bb4b();
                ptr = v0;
                v14 = v0 + 0x200;
            }
            v16 = m + 1;
            v2 = ptr + 1;
            *(ptr) = *(m);
            ptr = v2;
        }
        v26 = sub_454e5a(*((long long *)(v24 + iter * 16 + 8)), &v27, *((long long *)(v24 + iter * 16 + 8)));
        v17 = v0 + 0x200;
        for (n = v26; *(n); n = v19)
        {
            if (ptr == v17)
            {
                sub_40bba7(v0);
                v0 = sub_40bb4b();
                ptr = v0;
                v17 = v0 + 0x200;
            }
            v19 = n + 1;
            v2 = ptr + 1;
            *(ptr) = *(n);
            ptr = v2;
        }
        v20 = v0 + 0x200;
        for (i0 = "\n"; *(i0); i0 = v22)
        {
            if (ptr == v20)
            {
                sub_40bba7(v0);
                v0 = sub_40bb4b();
                ptr = v0;
                v20 = v0 + 0x200;
            }
            v22 = i0 + 1;
            v2 = ptr + 1;
            *(ptr) = *(i0);
            ptr = v2;
        }
    }
    memset(ptr, 0, 0x200 - (ptr - v0));
    sub_40bba7(v0);
    return 1;
}



// Function: pax_dump_header @ 0x31fd5
typedef struct struct_1 {
    char padding_0[292];
    unsigned int field_124;
    unsigned int field_128;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
} struct_0;

extern unsigned int g_49f97c;
extern unsigned int g_49f980;

long long pax_dump_header(struct_0 *idx)
{
    char v1;  // al
    char v3;  // al

    idx->field_18->field_124 = g_49f97c;
    idx->field_18->field_128 = g_49f980;
    if (idx->field_18->field_124)
    {
        v3 = sub_43179e(idx);
        return _INSERT(v3, 0, v3);
    }
    v1 = sub_431450(idx);
    return _INSERT(v1, 0, v1);
}



// Function: decode_num @ 0x3204b
unsigned int decode_num(unsigned long long *a0, char *a1, unsigned long long a2)
{
    char v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]

    if (*(a1) - 48 > 9)
        return 0;
    *(__errno_location()) = 0;
    v1 = strtoumax(a1, &v0, 10, &v0);
    if (v1 > a2 || *(__errno_location()) == 0x22 || *((char *)v0))
        return 0;
    *(a0) = v1;
    return 1;
    return 0;
}



// Function: pax_decode_header @ 0x32100
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
    long long field_138;
    unsigned long long field_140;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

extern unsigned int g_49f460;
extern struct_2 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int pax_decode_header(struct_1 *idx)
{
    long long v29;  // rax
    unsigned long long v22;  // rax
    unsigned long long v0;  // [bp-0xa0]
    char *v1;  // [bp-0x98]
    char *v2;  // [bp-0x90]
    char *v3;  // [bp-0x90]
    long long i;  // [bp-0x88]
    char *v5;  // [bp-0x80]
    char *v6;  // [bp-0x78]
    char *v7;  // [bp-0x78]
    char *v8;  // [bp-0x70]
    void* iter;  // [bp-0x68]
    char *v10;  // [bp-0x60]
    void* node;  // [bp-0x58]
    long long v12;  // [bp-0x50]
    unsigned long long v13;  // [bp-0x48]
    unsigned long long v14;  // [bp-0x40]
    char v15[4];  // [bp-0x38]
    char v16;  // [bp-0x24]

    if (!idx->field_18->field_124)
        return 1;
    v12 = sub_40bacd();
    sub_40bba7(*((long long *)&g_49f460));
    v1 = sub_40bb4b();
    if (!v1)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Unexpected EOF in archive"));
        sub_4177f4(); /* do not return */
    }
    v3 = v1;
    v5 = v1 + 0x200;
    v6 = &v15[0];
    do
    {
        v7 = v6;
        if (v7 == &v16)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%s: numeric overflow in sparse archive member"));
            g_49fdd0 = 2;
            return 0;
        }
        v2 = v3;
        if (v2 == v5)
        {
            sub_40bba7(v1);
            v1 = sub_40bb4b();
            if (!v1)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Unexpected EOF in archive"));
                sub_4177f4(); /* do not return */
            }
            v2 = v1;
            v5 = v1 + 0x200;
        }
        v3 = v2 + 1;
        *(v7) = *(v2);
        v6 = v7 + 1;
    } while (*(v7) != 10);
    *(v6 - 1) = 0;
    if ((char)sub_43204b(&v0, v15, 0xffffffffffffffff) != 1)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("%s: malformed sparse archive member"));
        g_49fdd0 = 2;
        return 0;
    }
    idx->field_18->field_138 = v0;
    v22 = sub_460c29(idx->field_18->field_138, 16, idx);
    idx->field_18->field_140 = v22;
    idx->field_18->field_130 = 0;
    for (i = 0; i < idx->field_18->field_138; i += 1)
    {
        v8 = v1 + 0x200;
        iter = v15;
        do
        {
            if (iter == &v16)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("%s: numeric overflow in sparse archive member"));
                g_49fdd0 = 2;
                return 0;
            }
            v2 = v3;
            if (v2 == v8)
            {
                sub_40bba7(v1);
                v1 = sub_40bb4b();
                if (!v1)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Unexpected EOF in archive"));
                    sub_4177f4(); /* do not return */
                }
                v2 = v1;
                v8 = v1 + 0x200;
            }
            v3 = v2 + 1;
            *((char *)iter) = *(v2);
            iter += 1;
        } while (*((char *)iter) != 10);
        *((char *)iter - 1) = 0;
        if ((char)sub_43204b(&v0, v15, 0x7fffffffffffffff) != 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%s: malformed sparse archive member"));
            g_49fdd0 = 2;
            return 0;
        }
        v13 = v0;
        v10 = v1 + 0x200;
        node = v15;
        do
        {
            if (node == &v16)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("%s: numeric overflow in sparse archive member"));
                g_49fdd0 = 2;
                return 0;
            }
            v2 = v3;
            if (v2 == v10)
            {
                sub_40bba7(v1);
                v1 = sub_40bb4b();
                if (!v1)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Unexpected EOF in archive"));
                    sub_4177f4(); /* do not return */
                }
                v2 = v1;
                v10 = v1 + 0x200;
            }
            v3 = v2 + 1;
            *((char *)node) = *(v2);
            node += 1;
        } while (*((char *)node) != 10);
        *((char *)node - 1) = 0;
        if ((char)sub_43204b(&v0, v15, 0x7fffffffffffffff) != 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%s: malformed sparse archive member"));
            g_49fdd0 = 2;
            return 0;
        }
        v14 = v0;
        sub_42f477(idx->field_18, &v13);
    }
    sub_40bba7(v1);
    v29 = sub_40bacd();
    idx->field_10 = (v29 - v12) * 0x200 + idx->field_10;
    return 1;
}



// Function: find_compression_suffix @ 0x3286e
extern void* g_49c960;

void* * find_compression_suffix(char *a0, unsigned long long *a1)
{
    void* *i;  // [bp-0x20]
    char *ptr;  // [bp-0x18]
    unsigned long len;  // [bp-0x10]

    ptr = strrchr(a0, 46);
    if (ptr)
    {
        ptr += 1;
        len = strlen(ptr);
        for (i = &g_49c960; *(i); i += 3)
        {
            if (len == i[1] && !memcmp(*(i), ptr, len))
            {
                if (!a1)
                    return i;
                *(a1) = strlen(a0) - len - 1;
                return i;
            }
        }
    }
    return NULL;
}



// Function: find_compression_program @ 0x32930
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long find_compression_program(char *a0, unsigned long a1)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = sub_43286e(a0, NULL);
    if (!v0)
        return a1;
    return v0->field_10;
}



// Function: set_compression_program_by_suffix @ 0x32970
extern unsigned long long g_49f898;

long long set_compression_program_by_suffix(char *a0, unsigned int a1)
{
    long long v0;  // [bp-0x10]

    v0 = sub_432930(a0, a1);
    if (v0)
        g_49f898 = v0;
    return v0;
}



// Function: strip_compression_suffix @ 0x329b0
typedef struct struct_0 {
    char field_0;
} struct_0;

void* strip_compression_suffix(void* a0)
{
    unsigned long long n;  // [bp-0x28]
    void* v1;  // [bp-0x20]
    struct_0 **v2;  // [bp-0x18]

    v1 = NULL;
    v2 = sub_43286e(a0, &n);
    if (v2)
    {
        if (n > 4 && !strncmp(a0 + n - 4, ".tar", 4) && *(v2)->field_0 != 116)
            n -= 4;
        if (!n)
            return NULL;
        v1 = sub_460b5b(n + 1);
        memcpy(v1, a0, n);
        *((char *)v1 + n) = 0;
    }
    return v1;
}



// Function: xexec @ 0x32aaa
long long xexec(long long a0)
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
    sub_43e251(a0); /* do not return */
}



// Function: sys_get_archive_stat @ 0x32b11
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

extern stat g_49f7a0;
extern int g_49f9cc;

char sys_get_archive_stat(void)
{
    return !fstat(g_49f9cc, &g_49f7a0.st_mode);
}



// Function: sys_file_is_archive @ 0x32b37
typedef struct struct_0 {
    char padding_0[96];
    unsigned long long field_60;
} struct_0;

extern unsigned long long g_49fbf8;
extern char g_49fc00;

unsigned int sys_file_is_archive(struct_0 *a0)
{
    unsigned int v1;  // eax

    if (g_49fbf8 && *((long long *)&a0->padding_0[88]) == g_49fbf8 && a0->field_60 == *((long long *)&g_49fc00))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: sys_save_archive_dev_ino @ 0x32b88
extern unsigned long long g_49f7a0;
extern unsigned long long g_49f7a8;
extern unsigned int g_49f7b8;
extern void g_49f9cc;
extern unsigned long long g_49fbf8;
extern unsigned long long g_49fc00;

long long sys_save_archive_dev_ino(void)
{
    unsigned long v1;  // rax

    v1 = *((int *)&g_49f9cc);
    if (*((int *)&g_49f9cc) <= 0x3fffffff)
    {
        v1 = g_49f7b8 & 0xf000;
        if ((unsigned int)v1 == 0x8000)
        {
            g_49fbf8 = g_49f7a0;
            g_49fc00 = g_49f7a8;
            return g_49f7a8;
        }
    }
    g_49fbf8 = 0;
    return v1;
}



// Function: sys_detect_dev_null_output @ 0x32bdc
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
    unsigned long long field_8;
} struct_0;

extern char g_49f7a0;
extern char g_49f7a8;
extern unsigned int g_49f7b8;
extern int g_49f9cc;
extern char g_49f9d0;
extern struct_0 *g_49fbd8;

void sys_detect_dev_null_output(void)
{
    unsigned int v2;  // eax
    char v3;  // al
    stat v0;  // [bp-0xa8]

    if (!strcmp(g_49fbd8->field_0, "/dev/null") || g_49f9cc <= 0x3fffffff && ((unsigned short)g_49f7b8 & 0xf000) == 0x2000 && !(v2 = (unsigned int)(int)(long long)stat("/dev/null", &v0), (int)(long long)stat("/dev/null", &v0) || *((long long *)&g_49f7a0) != *((unsigned long long *)(void*)&v0) || *((long long *)&g_49f7a8) != v0.st_ino))
        v3 = 1;
    else
        v3 = 0;
    g_49f9d0 = v3 & 1;
    return;
}



// Function: sys_wait_for_child @ 0x32ca8
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern long long g_49f898;
extern struct_0 *g_49fdc8;

void sys_wait_for_child(int a0, char a1)
{
    int v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x24]

    if (!a0)
        return;
    do
    {
        if (waitpid(a0, &v0, 0) != 0xffffffff)
            goto LABEL_432d10;
    } while (*(__errno_location()) == 4);
    sub_43eaf9(g_49f898);
LABEL_432d10:
    if (((char)v0 & 127) + 1 >> 1 > 0)
    {
        v1 = v0 & 127;
        if (!a1 && v1 == 13)
            return;
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Child died with signal %d"));
        sub_4177f4(); /* do not return */
    }
    else if ((char)(v0 >> 8))
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Child returned status %d"));
        sub_4177f4(); /* do not return */
    }
    else
    {
        return;
    }
}



// Function: sys_spawn_shell @ 0x32df0
long long sys_spawn_shell(void)
{
    unsigned long v5;  // fs
    int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]
    unsigned long v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    v2 = getenv("SHELL");
    if (!v2)
        v2 = "/bin/sh";
    v1 = sub_42b537();
    if (!v1)
    {
        sub_45c835();
        execlp(v2, "-sh");
        sub_43e251(v2); /* do not return */
    }
    while (waitpid(v1, &v0, 0) == 0xffffffff)
    {
        if (*(__errno_location()) != 4)
        {
            sub_43eaf9(v2);
            break;
            return v3 - *((long long *)(40 + v5));
        }
    }
    return v3 - *((long long *)(40 + v5));
}



// Function: sys_compare_uid @ 0x32ebf
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



// Function: sys_compare_gid @ 0x32ee4
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



// Function: sys_compare_links @ 0x32f09
unsigned int sys_compare_links(unsigned long a0, unsigned long long *a1)
{
    unsigned int v1;  // eax

    if (*(a1) == *((long long *)a0) && a1[1] == *((long long *)(a0 + 8)))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: sys_truncate @ 0x32f52
unsigned int sys_truncate(int a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = lseek(a0, 0, 1);
    return (v0 - 0 >> 63 & 1 ? 0xffffffff : (int)ftruncate(a0, v0));
}



// Function: is_regular_file @ 0x32f9a
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



// Function: sys_write_archive_buffer @ 0x3301a
extern long long g_49efa0;
extern long long g_49f870;
extern void g_49f9cc;

long long sys_write_archive_buffer(void)
{
    unsigned long v1;  // rcx

    if (*((int *)&g_49f9cc) <= 0x3fffffff)
        return sub_452175(*((int *)&g_49f9cc), g_49efa0, g_49f870, g_49efa0);
    v1 = *((int *)&g_49f9cc) - 0x40000000;
    return sub_43ff8c(v1 & 0xffffffff, g_49efa0, g_49f870, v1);
}



// Function: xdup2 @ 0x3307b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;

unsigned long long xdup2(unsigned int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x14]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    if (a0 == a1)
        return a0;
    v0 = close(a1);
    if (v0 && *(__errno_location()) != 9)
    {
        v1 = *(__errno_location());
        if (g_49fdc8)
            g_49fdc8();
        error(0, v1, gettext("Cannot close"));
        sub_4177f4(); /* do not return */
    }
    v0 = dup(a0);
    if (v0 == a1)
    {
        return sub_40bc11(a0);
    }
    else if (v0 < 0)
    {
        v2 = *(__errno_location());
        if (g_49fdc8)
            g_49fdc8();
        error(0, v2, gettext("Cannot dup"));
        sub_4177f4(); /* do not return */
    }
    else
    {
        abort(); /* do not return */
    }
}



// Function: wait_for_grandchild @ 0x33179
extern long long g_49f898;

void wait_for_grandchild(int a0)
{
    unsigned long v4;  // fs
    int v0;  // [bp-0x18]
    unsigned int v1;  // [bp-0x14]
    unsigned long v2;  // [bp-0x10]

    v2 = *((long long *)(40 + v4));
    v1 = 0;
    do
    {
        if (waitpid(a0, &v0, 0) != 0xffffffff)
            goto LABEL_4331d8;
    } while (*(__errno_location()) == 4);
    sub_43eaf9(g_49f898);
LABEL_4331d8:
    if (((char)v0 & 127) + 1 >> 1 > 0)
    {
        raise(v0 & 127);
    }
    else if ((char)(v0 >> 8))
    {
        v1 = v0 >> 8 & 0xff;
    }
    exit(v1); /* do not return */
}



// Function: sys_child_open_for_compress @ 0x3321b
extern unsigned long long g_49efa0;
extern unsigned long long g_49f870;
extern char g_49f884;
extern long long g_49f898;
extern long long g_49f950;
extern void g_49f9cc;
extern unsigned long long g_49fbd8;
extern unsigned long long g_49fdf0;
extern char g_49fdf8;

long long sys_child_open_for_compress(void)
{
    unsigned long v13;  // fs
    long long v16;  // rdi
    long long v17;  // rsi
    long long v18;  // rdx
    long long v19;  // rcx
    long long v20;  // r8
    long long v21;  // r9
    unsigned int v22;  // eax
    unsigned int v0;  // [bp-0x4c]
    unsigned int v1;  // [bp-0x48]
    unsigned int v2;  // [bp-0x44]
    unsigned long long v3;  // [bp-0x40], Other Possible Types: long long
    unsigned long i;  // [bp-0x38]
    long long v5;  // [bp-0x30]
    long long v6;  // [bp-0x28]
    unsigned int v7;  // [bp-0x20]
    unsigned int v8;  // [bp-0x1c]
    unsigned int v9;  // [bp-0x18]
    unsigned int v10;  // [bp-0x14]
    unsigned long v11;  // [bp-0x10]

    v11 = *((long long *)(40 + v13));
    signal(13, 0x1);
    sub_42b577(&v7);
    v0 = sub_42b537();
    if (v0 > 0)
    {
        *((unsigned int *)&g_49f9cc) = v8;
        sub_40bc11(v7);
        if (v11 == *((long long *)(40 + v13)))
            return v0;
        __stack_chk_fail(); /* do not return */
    }
    else
    {
        sub_45c844(gettext("tar (child)"));
        signal(13, NULL);
        sub_43307b(v7, 0);
        sub_40bc11(v8);
        if ((g_49fdf8 || (g_49fdf0 = (unsigned long long)strchr(*((long long *)g_49fbd8), 58), !g_49fdf0 || *((long long *)g_49fbd8) >= g_49fdf0 || memchr(*((long long *)g_49fbd8), 47, g_49fdf0 - *((long long *)g_49fbd8)))) && sub_432f9a(*((long long *)g_49fbd8)))
        {
            if (g_49f884)
                sub_42a571(*((long long *)g_49fbd8), 1);
            if (strcmp(*((long long *)g_49fbd8), "-"))
            {
                *((unsigned int *)&g_49f9cc) = creat(*((long long *)g_49fbd8), 438);
                if (*((int *)&g_49f9cc) < 0)
                {
                    v1 = *(__errno_location());
                    if (g_49f884)
                        sub_42a874();
                    *(__errno_location()) = v1;
                    sub_43e3b4(*((long long *)g_49fbd8)); /* do not return */
                }
                sub_43307b(*((int *)&g_49f9cc), 1);
            }
            sub_45c835();
            sub_432aaa(g_49f898); /* do not return */
        }
        sub_42b577(&v9);
        v2 = sub_42b537();
        if (!v2)
        {
            sub_45c844(gettext("tar (grandchild)"));
            sub_43307b(v10, 1);
            sub_40bc11(v9);
            sub_45c835(v16, v17, v18, v19, v20, v21);
            sub_432aaa(g_49f898); /* do not return */
        }
        sub_43307b(v9, 0);
        sub_40bc11(v10);
        if (!strcmp(*((long long *)g_49fbd8), "-"))
        {
            *((unsigned int *)&g_49f9cc) = 1;
        }
        else
        {
            if (g_49fdf8 != 1 && !(g_49fdf0 = (unsigned long long)strchr(*((long long *)g_49fbd8), 58), !g_49fdf0 || *((long long *)g_49fbd8) >= g_49fdf0 || memchr(*((long long *)g_49fbd8), 47, g_49fdf0 - *((long long *)g_49fbd8))))
                v22 = sub_43f860(*((long long *)g_49fbd8), 65, 0x40000000, g_49f950);
            else
                v22 = creat(*((long long *)g_49fbd8), 438);
            *((unsigned int *)&g_49f9cc) = v22;
            if (*((int *)&g_49f9cc) < 0)
                sub_43e3b4(*((long long *)g_49fbd8)); /* do not return */
        }
        while (1)
        {
            v3 = 0;
            v5 = 0;
            for (i = g_49efa0; v5 < g_49f870; i += v3)
            {
                v6 = g_49f870 - v5;
                v3 = sub_45ec70(0, i, v6);
                if (v3 == 0xffffffffffffffff)
                    sub_43e5d6(g_49f898); /* do not return */
                if (!v3)
                    break;
                v5 += v3;
            }
            if (!v3)
                break;
            v3 = sub_43301a();
            if (v3 != g_49f870)
                sub_40c6d5(v3); /* do not return */
        }
        if (v5)
        {
            memset(g_49efa0 + v5, 0, g_49f870 - v5);
            v3 = sub_43301a();
            if (v3 != g_49f870)
                sub_40c6d5(v3); /* do not return */
        }
        sub_433179(v2); /* do not return */
    }
}



// Function: run_decompress_program @ 0x336d5
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec84;
extern unsigned long long environ;
extern struct_0 *g_49fdc8;

long long run_decompress_program(void)
{
    unsigned long long v11;  // rbx
    unsigned long v12;  // fs
    char *v13;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x120]
    unsigned int v1;  // [bp-0x11c]
    unsigned long i;  // [bp-0x118]
    unsigned long v3;  // [bp-0x110]
    unsigned long idx;  // [bp-0x108]
    char **v5;  // [bp-0x100]
    unsigned long long v6;  // [bp-0xf8]
    unsigned long v7;  // [bp-0x98]
    unsigned long v8;  // [bp-0x20]
    unsigned long long v9;  // [bp-0x10]

    v9 = v11;
    v8 = *((long long *)(40 + v12));
    v3 = 0;
    v1 = 34082310;
    v7 = environ;
    v6 = 1;
    for (i = sub_40afa2(&v0); i; i = sub_40b012(&v0))
    {
        if (v3)
        {
            if (g_49ec84 & 0xffffff & 0x80000)
            {
                if (g_49fdc8)
                    g_49fdc8();
                v13 = gettext("cannot run %s");
                err = __errno_location();
                error(0, *(err), v13);
            }
            if (g_49ec84 & 0xffffff & 0x80000)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("trying %s"));
            }
        }
        if (sub_4461a6(i, &idx, v1, &idx))
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_44647f(&idx);
            error(0, 0, gettext("cannot split string '%s': %s"));
            sub_4177f4(); /* do not return */
        }
        v1 |= 8;
        memmove(v5, &v5[v6], idx * 8);
        v5[idx] = "-d";
        v3 = i;
        execvp(*(v5), v5);
        v5[idx] = 0;
    }
    if (v3)
    {
        sub_43e251(v3); /* do not return */
    }
    else if (g_49fdc8)
    {
        g_49fdc8();
    }
    error(0, 0, gettext("unable to run decompression program"));
    sub_4177f4(); /* do not return */
}



// Function: sys_child_open_for_uncompress @ 0x3399b
extern unsigned long long g_49efa0;
extern long long g_49f870;
extern long long g_49f898;
extern long long g_49f950;
extern void g_49f9cc;
extern unsigned long long g_49fbd8;
extern unsigned long long g_49fdf0;
extern char g_49fdf8;

long long sys_child_open_for_uncompress(void)
{
    unsigned long v12;  // fs
    long long v22;  // rsi
    long long v23;  // rdx
    long long v24;  // rcx
    long long v25;  // r8
    long long v26;  // r9
    unsigned int v27;  // eax
    unsigned long v28;  // rcx
    unsigned long long v29;  // rax
    unsigned long long v30;  // rax
    long long v14;  // rdi
    long long v15;  // rsi
    long long v16;  // rdx
    long long v17;  // rcx
    long long v18;  // r8
    long long v19;  // r9
    long long v21;  // rdi
    unsigned int v0;  // [bp-0x48]
    unsigned int v1;  // [bp-0x44]
    unsigned long v2;  // [bp-0x40]
    unsigned long long i;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x30]
    unsigned long long v5;  // [bp-0x28]
    char v6;  // [bp-0x20]
    unsigned int v7;  // [bp-0x1c]
    unsigned int v8;  // [bp-0x18]
    unsigned int v9;  // [bp-0x14]
    unsigned long v10;  // [bp-0x10]

    v10 = *((long long *)(40 + v12));
    sub_42b577(&v6);
    v0 = sub_42b537();
    if (v0 > 0)
    {
        *((unsigned int *)&g_49f9cc) = v6;
        sub_40bc11(v7);
        if (v10 == *((long long *)(40 + v12)))
            return v0;
        __stack_chk_fail(); /* do not return */
    }
    else
    {
        sub_45c844(gettext("tar (child)"));
        signal(13, NULL);
        sub_43307b(v7, 1);
        sub_40bc11(v6);
        if (strcmp(*((long long *)g_49fbd8), "-") && (g_49fdf8 || (g_49fdf0 = (unsigned long long)strchr(*((long long *)g_49fbd8), 58), !g_49fdf0 || *((long long *)g_49fbd8) >= g_49fdf0 || memchr(*((long long *)g_49fbd8), 47, g_49fdf0 - *((long long *)g_49fbd8)))) && sub_432f9a(*((long long *)g_49fbd8)))
        {
            *((int *)&g_49f9cc) = open(*((long long *)g_49fbd8), 0, 438);
            if (*((int *)&g_49f9cc) >= 0)
            {
                sub_43307b(*((int *)&g_49f9cc), 0);
                sub_45c835(v14, v15, v16, v17, v18, v19);
                sub_4336d5(); /* do not return */
            }
            sub_43e3b4(*((long long *)g_49fbd8)); /* do not return */
        }
        sub_42b577(&v8);
        v1 = sub_42b537();
        if (!v1)
        {
            sub_45c844(gettext("tar (grandchild)"));
            sub_43307b(v8, 0);
            sub_40bc11(v9);
            sub_45c835(v21, v22, v23, v24, v25, v26);
            sub_4336d5(); /* do not return */
        }
        sub_43307b(v9, 1);
        sub_40bc11(v8);
        if (!strcmp(*((long long *)g_49fbd8), "-"))
        {
            *((unsigned int *)&g_49f9cc) = 0;
        }
        else
        {
            if (g_49fdf8 != 1 && !(g_49fdf0 = (unsigned long long)strchr(*((long long *)g_49fbd8), 58), !g_49fdf0 || *((long long *)g_49fbd8) >= g_49fdf0 || memchr(*((long long *)g_49fbd8), 47, g_49fdf0 - *((long long *)g_49fbd8))))
                v27 = sub_43f860(*((long long *)g_49fbd8), 0, 0x40000000, g_49f950);
            else
                v27 = open(*((long long *)g_49fbd8), 0, 438);
            *((unsigned int *)&g_49f9cc) = v27;
        }
        if (*((int *)&g_49f9cc) < 0)
            sub_43e3b4(*((long long *)g_49fbd8)); /* do not return */
        while (1)
        {
            sub_40adb0();
            while (1)
            {
                if (*((int *)&g_49f9cc) > 0x3fffffff)
                {
                    v28 = *((int *)&g_49f9cc) - 0x40000000;
                    v29 = sub_43fe62(v28 & 0xffffffff, g_49efa0, g_49f870, v28);
                }
                else
                {
                    v29 = sub_45ec70(*((int *)&g_49f9cc), g_49efa0, g_49f870, g_49efa0);
                }
                v4 = v29;
                if (v4 != 0xffffffffffffffff)
                    break;
                sub_40c729();
            }
            if (!v4)
                break;
            v2 = g_49efa0;
            for (i = v4; i; i -= v5)
            {
                if (i > 0x200)
                    v30 = 0x200;
                else
                    v30 = i;
                v5 = v30;
                if (v5 != sub_452175())
                    sub_43eb22(g_49f898);
                v2 += v5;
            }
        }
        sub_40bc11(1);
        sub_433179(v1); /* do not return */
    }
}



// Function: dec_to_env @ 0x33de6
void dec_to_env(char *a0, long long a1)
{
    char *v0;  // [bp-0x30]
    char v1;  // [bp-0x28]

    v0 = sub_454e5a(a1, &v1, &v1);
    if (!setenv(a0, v0, 1))
        return;
    sub_41783c(); /* do not return */
}



// Function: time_to_env @ 0x33e58
void time_to_env(char *a0, unsigned int a1, unsigned int a2)
{
    char v0;  // [bp-0x38]

    if (!setenv(a0, sub_429f2e(a1, a2, &v0), 1))
        return;
    sub_41783c(); /* do not return */
}



// Function: oct_to_env @ 0x33ed9
void oct_to_env(char *a0, unsigned int a1)
{
    char v0[24];  // [bp-0x28]

    snprintf(v0, 24, "0%lo", a1);
    if (!setenv(a0, v0, 1))
        return;
    sub_41783c(); /* do not return */
}



// Function: str_to_env @ 0x33f58
int str_to_env(char *a0, char *a1)
{
    unsigned int v1;  // eax

    if (!a1)
        return unsetenv(a0);
    v1 = setenv(a0, a1, 1);
    if (!v1)
        return v1;
    sub_41783c(); /* do not return */
}



// Function: chr_to_env @ 0x33fa3
void chr_to_env(char *a0, char a1)
{
    char v0;  // [bp-0x12]
    char v1;  // [bp-0x11]

    v0 = a1;
    v1 = 0;
    if (!setenv(a0, &v0, 1))
        return;
    sub_41783c(); /* do not return */
}



// Function: stat_to_env @ 0x3400a
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
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
    unsigned long long field_108;
    unsigned long long field_110;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f468;
extern unsigned int g_49f864;
extern int g_49f868;
extern unsigned long long g_49fbd8;
extern struct_0 *g_49fbe8;

int stat_to_env(char *a0, char a1, struct_1 *a2)
{
    int v2;  // eax

    sub_433f58("TAR_VERSION", "1.34");
    sub_433f58("TAR_ARCHIVE", g_49fbe8->field_0);
    sub_433de6("TAR_VOLUME", ((char *)g_49fbe8 - g_49fbd8 >> 3) + 1);
    sub_433de6("TAR_BLOCKING_FACTOR", g_49f868);
    sub_433f58("TAR_FORMAT", sub_434d66((!g_49f468 ? g_49f864 : g_49f468)));
    sub_433fa3("TAR_FILETYPE", a1);
    sub_433ed9("TAR_MODE", a2->field_70);
    sub_433f58("TAR_FILENAME", a0);
    sub_433f58("TAR_REALNAME", a2->field_8);
    sub_433f58("TAR_UNAME", a2->field_20);
    sub_433f58("TAR_GNAME", a2->field_28);
    sub_433e58("TAR_ATIME", a2->field_e8, a2->field_f0);
    sub_433e58("TAR_MTIME", a2->field_f8, a2->field_100);
    sub_433e58("TAR_CTIME", a2->field_108, a2->field_110);
    sub_433de6("TAR_SIZE", a2->field_88);
    sub_433de6("TAR_UID", a2->field_74);
    sub_433de6("TAR_GID", a2->field_78);
    v2 = a1;
    if (v2 == 108)
    {
LABEL_4342b8:
        unsetenv("TAR_MINOR");
        unsetenv("TAR_MAJOR");
        return sub_433f58("TAR_LINKNAME", a2->field_18);
    }
    if (v2 <= 108)
    {
        if (v2 <= 99)
        {
            if (v2 >= 98)
            {
                sub_433de6("TAR_MINOR", gnu_dev_minor(a2->field_80) & 0xffffffff);
                sub_433de6("TAR_MAJOR", gnu_dev_major(a2->field_80) & 0xffffffff);
                return unsetenv("TAR_LINKNAME");
            }
        }
        else
        {
            if (v2 == 104)
                goto LABEL_4342b8;
        }
    }
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    return unsetenv("TAR_LINKNAME");
}



// Function: sys_exec_command @ 0x34323
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
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
    unsigned long long field_108;
    unsigned long long field_110;
} struct_1;

extern unsigned int g_49f830;
extern unsigned long long g_49f838;
extern long long g_49f9a0;

long long sys_exec_command(char *a0, char a1, struct_1 *a2)
{
    unsigned long v4;  // fs
    long long v5;  // rdi
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    char v0;  // [bp-0x18]
    unsigned int v1;  // [bp-0x14]
    unsigned long v2;  // [bp-0x10]

    v2 = *((long long *)(40 + v4));
    sub_42b577(&v0);
    g_49f838 = signal(13, 0x1);
    g_49f830 = sub_42b537();
    if (!g_49f830)
    {
        sub_43307b(v0, 0);
        sub_40bc11(v1);
        sub_43400a(a0, a1, a2);
        sub_45c835(v5, v6, v7, v8, v9, v10);
        sub_432aaa(g_49f9a0); /* do not return */
    }
    sub_40bc11(v0);
    if (v2 == *((long long *)(40 + v4)))
        return v1;
    __stack_chk_fail(); /* do not return */
}



// Function: sys_wait_command @ 0x343ea
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern int g_49f830;
extern sighandler_t g_49f838;
extern long long g_49f9a0;
extern char g_49f9a8;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

void sys_wait_command(void)
{
    int v0;  // [bp-0x24]

    if (g_49f830 < 0)
        return;
    signal(13, g_49f838);
    while (waitpid(g_49f830, &v0, 0) == 0xffffffff)
    {
        if (*(__errno_location()) != 4)
        {
            g_49f830 = 0xffffffff;
            sub_43eaf9(g_49f9a0);
            return;
        }
    }
    if (!((char)v0 & 127))
    {
        if (g_49f9a8 != 1 && (char)(v0 >> 8))
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%lu: Child returned status %d"));
            g_49fdd0 = 2;
        }
    }
    else
    {
        if (((char)v0 & 127) + 1 >> 1 > 0)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%lu: Child terminated on signal %d"));
        }
        else
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("%lu: Child terminated on unknown reason"));
            g_49fdd0 = 2;
        }
    }
    g_49f830 = 0xffffffff;
    return;
}



// Function: sys_exec_info_script @ 0x345e7
typedef struct FILE {
} FILE;

extern unsigned int g_49f468;
extern sighandler_t g_49f840;
extern unsigned int g_49f860;
extern unsigned int g_49f864;
extern int g_49f868;
extern long long g_49f8c0;

unsigned long long sys_exec_info_script(char **a0, int a1)
{
    long long v20;  // r9
    long long v15;  // rdi
    long long v16;  // rsi
    long long v17;  // rdx
    long long v18;  // rcx
    long long v19;  // r8
    int v0;  // [bp-0x54]
    unsigned int v1;  // [bp-0x50]
    unsigned int v2;  // [bp-0x4c]
    char *ptr;  // [bp-0x48]
    unsigned long v4;  // [bp-0x40]
    FILE *fp;  // [bp-0x38]
    char v6;  // [bp-0x30]
    int v7;  // [bp-0x2c]
    char v8;  // [bp-0x28]

    sub_42b577(&v6);
    g_49f840 = signal(13, 0x1);
    v2 = sub_42b537();
    if (!v2)
    {
        setenv("TAR_VERSION", "1.34", 1);
        setenv("TAR_ARCHIVE", *(a0), 1);
        setenv("TAR_VOLUME", sub_454e5a(a1, &v8, &v8), 1);
        setenv("TAR_BLOCKING_FACTOR", sub_454e5a(g_49f868, &v8, &v8), 1);
        setenv("TAR_SUBCOMMAND", sub_434e1c(g_49f860), 1);
        setenv("TAR_FORMAT", sub_434d66((!g_49f468 ? g_49f864 : g_49f468)), 1);
        setenv("TAR_FD", sub_454e5a(v7, &v8, &v8), 1);
        sub_40bc11(v6);
        sub_45c835(v15, v16, v17, v18, v19, v20);
        sub_432aaa(g_49f8c0); /* do not return */
    }
    ptr = NULL;
    v4 = 0;
    sub_40bc11(v7);
    fp = fdopen(v6, "r");
    v1 = getline(&ptr, &v4, fp);
    fclose(fp);
    if (v1 > 0 && *(&ptr[v1] - 1) == 10)
    {
        v1 -= 1;
        ptr[v1] = 0;
    }
    while (waitpid(v2, &v0, 0) == 0xffffffff)
    {
        if (*(__errno_location()) != 4)
        {
            signal(13, g_49f840);
            sub_43eaf9(g_49f8c0);
            return 0xffffffff;
        }
    }
    signal(13, g_49f840);
    if ((char)v0 & 127)
    {
        free(ptr);
        return 0xffffffff;
    }
    if (!(char)(v0 >> 8) && v1 > 0)
        *(a0) = ptr;
    else
        free(ptr);
    return (char)(v0 >> 8);
}



// Function: sys_exec_checkpoint_script @ 0x348d7
extern unsigned int g_49f468;
extern unsigned int g_49f860;
extern unsigned int g_49f864;
extern int g_49f868;

long long sys_exec_checkpoint_script(long long a0, char *a1, int a2)
{
    unsigned long v5;  // fs
    long long v15;  // r9
    long long v10;  // rdi
    long long v11;  // rsi
    long long v12;  // rdx
    long long v13;  // rcx
    long long v14;  // r8
    int v0;  // [bp-0x30]
    unsigned int v1;  // [bp-0x2c]
    char v2;  // [bp-0x28]
    unsigned long v3;  // [bp-0x10]

    v1 = sub_42b537();
    if (!v1)
    {
        setenv("TAR_VERSION", "1.34", 1);
        setenv("TAR_ARCHIVE", a1, 1);
        setenv("TAR_CHECKPOINT", sub_454e5a(a2, &v2, &v2), 1);
        setenv("TAR_BLOCKING_FACTOR", sub_454e5a(g_49f868, &v2, &v2), 1);
        setenv("TAR_SUBCOMMAND", sub_434e1c(g_49f860), 1);
        setenv("TAR_FORMAT", sub_434d66((!g_49f468 ? g_49f864 : g_49f468)), 1);
        sub_45c835(v10, v11, v12, v13, v14, v15);
        sub_432aaa(a0); /* do not return */
    }
    while (waitpid(v1, &v0, 0) == 0xffffffff)
    {
        if (*(__errno_location()) != 4)
        {
            sub_43eaf9(a0);
            break;
            return v3 - *((long long *)(40 + v5));
        }
    }
    return v3 - *((long long *)(40 + v5));
}



// Function: request_stdin @ 0x34a6f
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49fc40;
extern struct_0 *g_49fdc8;

long long request_stdin(unsigned long a0)
{
    if (!g_49fc40)
    {
        g_49fc40 = a0;
        return a0;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Options '%s' and '%s' both want standard input"));
    sub_43814c(2); /* do not return */
}



// Function: confirm @ 0x34af4
typedef struct FILE {
} FILE;

extern unsigned long long stdin;
extern FILE *g_49efd0;
extern unsigned int g_49f9cc;
extern unsigned long long g_49fc40;
extern FILE *g_49fcb0;
extern unsigned int g_49fcb8;

long long confirm(char *a0, long long a1)
{
    char *v4;  // rax
    char v0;  // [bp-0x21]
    char *v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]

    v0 = 0;
    if (!g_49fcb0)
    {
        if (!g_49f9cc || g_49fc40)
        {
            g_49fcb0 = fopen("/dev/tty", "r");
            if (!g_49fcb0)
                sub_43e3b4("/dev/tty"); /* do not return */
        }
        else
        {
            sub_434a6f("-w");
            g_49fcb0 = stdin;
        }
    }
    v4 = sub_45e84e(a1);
    fprintf(g_49efd0, "%s %s?", a0, v4);
    fflush_unlocked(g_49efd0);
    if (!g_49fcb8)
    {
        v1 = NULL;
        v2 = 0;
        if (getline(&v1, &v2, g_49fcb0) < NULL)
            g_49fcb8 = 1;
        else
            v0 = rpmatch(v1) > 0;
        free(v1);
    }
    if (!g_49fcb8)
        return v0;
    fputc_unlocked(10, g_49efd0);
    fflush_unlocked(g_49efd0);
    return v0;
}



// Function: set_archive_format @ 0x34c7e
typedef struct struct_0 {
    char *field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_0 g_49a8e0;
extern unsigned int g_49f864;
extern struct_1 *g_49fdc8;

int set_archive_format(unsigned long a0)
{
    struct_0 *v0;  // [bp-0x20]

    v0 = &g_49a8e0.field_0;
    do
    {
        if (!strcmp(v0->field_0, a0))
        {
            g_49f864 = v0->field_8;
            return g_49f864;
        }
    } while ((v0 += 16, v0->field_0));
    if (g_49fdc8)
        g_49fdc8();
    sub_45e548(a0);
    error(0, 0, gettext("%s: Invalid archive format"));
    sub_43814c(2); /* do not return */
}



// Function: set_xattr_option @ 0x34d36
extern unsigned int g_49f96c;

unsigned int set_xattr_option(unsigned int a0)
{
    if (a0 == 1)
        sub_434c7e("posix");
    g_49f96c = a0;
    return a0;
}



// Function: archive_format_string @ 0x34d66
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
} struct_0;

extern struct_0 g_49a8e0;

long long archive_format_string(unsigned int a0)
{
    struct_0 *i;  // [bp-0x10]

    for (i = &g_49a8e0.field_0; i->field_0; i = (char *)&i[1].field_0 + 4)
    {
        if (a0 == i->field_8)
            return i->field_0;
    }
    return "unknown?";
}



// Function: assert_format @ 0x34dad
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f864;
extern struct_0 *g_49fdc8;

unsigned long long assert_format(unsigned int a0)
{
    unsigned long long v1;  // rax

    v1 = 1 << ((char)g_49f864 & 31) & a0;
    if ((unsigned int)v1)
        return v1;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("GNU features wanted on incompatible archive format"));
    sub_43814c(2); /* do not return */
}



// Function: subcommand_string @ 0x34e1c
extern char g_48427d;
extern char g_4842bb;
extern char g_4842be;
extern char g_4842c1;
extern char g_4842c4;
extern char g_4842c7;
extern char g_4842ca;
extern char g_4842cd;
extern char g_4842d0;
extern char g_4842d3;

int subcommand_string(unsigned int a0)
{
    switch (a0)
    {
    case 0:
        return &g_48427d;
    case 1:
        return &g_4842bb;
    case 2:
        return &g_4842be;
    case 3:
        return &g_4842c1;
    case 4:
        return &g_4842c4;
    case 5:
        return &g_4842c7;
    case 6:
        return &g_4842ca;
    case 7:
        return &g_4842cd;
    case 8:
        return &g_4842d0;
    case 9:
        return &g_4842d3;
    default:
        abort(); /* do not return */
    }
}



// Function: tar_list_quoting_styles @ 0x34eb6
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned long long g_49b6c0[4];

unsigned long long tar_list_quoting_styles(struct_0 *a0, void* ptr)
{
    unsigned long v11;  // rax
    int i;  // [bp-0x54]
    unsigned long len;  // [bp-0x50]
    struct_0 *idx;  // [bp-0x48]
    unsigned long n;  // [bp-0x40]
    struct_0 *v4;  // [bp-0x38]
    struct_0 *index;  // [bp-0x30]
    unsigned long len1;  // [bp-0x28]
    struct_0 *v7;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    struct_0 *v9;  // [bp-0x10]

    len = strlen(ptr);
    for (i = 0; g_49b6c0[i]; i += 1)
    {
        idx = a0;
        n = len;
        v4 = idx;
        if (v4->field_20 - v4->field_18 < n)
            _obstack_newchunk(idx, n, n);
        memcpy(idx->field_18, ptr, n);
        idx->field_18 = idx->field_18 + n;
        index = a0;
        len1 = strlen(g_49b6c0[i]);
        v7 = index;
        if (v7->field_20 - v7->field_18 < len1)
            _obstack_newchunk(index, len1, len1);
        memcpy(index->field_18, g_49b6c0[i], len1);
        index->field_18 = index->field_18 + len1;
        idx1 = a0;
        v9 = idx1;
        if (!(v9->field_20 - v9->field_18))
            _obstack_newchunk(idx1, 1);
        v11 = idx1->field_18;
        idx1->field_18 = v11 + 1;
        *((char *)v11) = 10;
    }
    return 0;
}



// Function: tar_set_quoting_style @ 0x35084
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49b6c0[4];
extern struct_0 *g_49fdc8;

int tar_set_quoting_style(char *a0)
{
    unsigned int i;  // [bp-0x1c]

    for (i = 0; g_49b6c0[i]; i += 1)
    {
        if (!strcmp(a0, g_49b6c0[i]))
            return sub_45c9a2(0, i);
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list."));
    sub_4177f4(); /* do not return */
}



// Function: format_default_settings @ 0x35152
extern long long g_49b6f8;

int format_default_settings(void)
{
    long long v1;  // rax

    v1 = sub_434d66(6);
    return sub_461277("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s", v1, "-", 20, g_49b6f8, "/usr/local/libexec/rmt", "/usr/bin/rsh");
}



// Function: option_conflict_error @ 0x351b6
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fdc8;

void option_conflict_error(unsigned long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v1 = a0;
    v0 = a1;
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("'%s' cannot be used with '%s'"));
    sub_43814c(2); /* do not return */
}



// Function: optloc_save @ 0x3521d
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_1 *field_8;
    unsigned long long field_10;
    struct struct_2 *field_18;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_2;

extern unsigned long long g_49fc60[4];

unsigned long long * optloc_save(unsigned int a0, struct_0 *a1)
{
    unsigned long v0;  // [bp-0x20]
    struct_1 *idx;  // [bp-0x18]
    struct_1 *v2;  // [bp-0x10]

    if (a0 > 9)
        abort(); /* do not return */
    v0 = 32;
    if (a1->field_8)
        v0 = strlen(a1->field_8) + v0 + 1;
    idx = sub_460b5b(v0);
    if (a1->field_8)
    {
        v2 = idx + 1;
        strcpy(v2, a1->field_8);
        idx->field_8 = v2;
    }
    else
    {
        idx->field_8 = NULL;
    }
    idx->field_0 = a1->field_0;
    idx->field_10 = a1->field_10;
    idx->field_18 = g_49fc60[a0];
    g_49fc60[a0] = idx;
    return idx->field_18;
}



// Function: optloc_lookup @ 0x35327
extern unsigned long long g_49fc60[4];

unsigned long long * optloc_lookup(int a0)
{
    return g_49fc60[a0];
}



// Function: option_set_in_cl @ 0x3534c
unsigned int option_set_in_cl(int a0)
{
    unsigned int *v0;  // [bp-0x10]

    v0 = sub_435327(a0);
    return (!v0 ? 0 : *(v0) == 1);
}



// Function: optloc_eq @ 0x35388
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
    return (a0->field_0 == *((int *)&a1->padding_0[0]) ? (a0->field_0 == 1 ? 1 : !strcmp(a0->field_8, a1->field_8)) : 0);
}



// Function: set_subcommand_option @ 0x353ea
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49f860;
extern struct_0 *g_49fdc8;

unsigned int set_subcommand_option(unsigned int a0)
{
    if (g_49f860 && a0 != g_49f860)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option"));
        sub_43814c(2); /* do not return */
    }
    g_49f860 = a0;
    return a0;
}



// Function: set_use_compress_program_option @ 0x3545f
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
    unsigned long long field_10;
} struct_0;

extern char *g_49f898;
extern struct_0 *g_49fdc8;

long long set_use_compress_program_option(unsigned long a0, struct_0 *a1)
{
    unsigned int *v0;  // [bp-0x10]

    v0 = sub_43521d(0, a1);
    if (g_49f898 && strcmp(g_49f898, a0) && *(v0) == 1)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Conflicting compression options"));
        sub_43814c(2); /* do not return */
    }
    g_49f898 = a0;
    return a0;
}



// Function: sigstat @ 0x3550c
sighandler_t sigstat(int a0)
{
    sub_40ae4d();
    sub_40baa2();
    return signal(a0, sigstat);
}



// Function: stat_on_signal @ 0x3553c
sighandler_t stat_on_signal(int a0)
{
    return signal(a0, sub_43550c);
}



// Function: decode_signal @ 0x35562
typedef struct struct_0 {
    char *field_0;
    unsigned int field_8;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern struct_0 g_49a9c0;
extern struct_0 g_49aa10;
extern struct_1 *g_49fdc8;

int decode_signal(char *a0)
{
    struct_0 *i;  // [bp-0x18]
    char *v1;  // [bp-0x10]

    v1 = a0;
    if (!strncmp(v1, "SIG", 3))
        v1 += 3;
    for (i = &g_49a9c0.field_0; i < &g_49aa10.field_0; i = (char *)&i[1].field_0 + 4)
    {
        if (!strcmp(i->field_0, v1))
            return i->field_8;
    }
    if (g_49fdc8)
        g_49fdc8();
    error(0, 0, gettext("Unknown signal name: %s"));
    sub_4177f4(); /* do not return */
}



// Function: set_stat_signal @ 0x35629
sighandler_t set_stat_signal(char *a0)
{
    return sub_43553c(sub_435562(a0));
}



// Function: get_date_or_file @ 0x3564f
typedef struct struct_3 {
    char padding_0[8];
    struct struct_2 *field_8;
} struct_3;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    struct struct_1 *field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_1;

typedef struct struct_2 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
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

typedef struct struct_4 {
    unsigned long long field_0;
} struct_4;

extern struct_4 *g_49fdc8;

unsigned int get_date_or_file(struct_3 *idx, unsigned long a1, char *a2, struct_0 *index)
{
    unsigned long long v4;  // rdx
    unsigned long long v7;  // rdx
    struct_1 *idx1;  // [bp-0xc0]
    stat v1;  // [bp-0xb8]

    if (*(a2) == 47 || *(a2) == 46)
    {
        if (stat(a2, &v1))
        {
            sub_43ea03(a2);
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Date sample file not found"));
            sub_43814c(2); /* do not return */
        }
        *((unsigned long long *)&index->field_0) = sub_45fd9e(&v1);
        index->field_8 = v7;
    }
    else
    {
        if ((char)sub_45a8ed(index, a2, 0, a2) != 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e84e(a2);
            sub_427abc(*((long long *)&index->field_0), index->field_8, 0);
            error(0, 0, gettext("Substituting %s for unknown date format %s"));
            index->field_8 = 0;
            return 1;
        }
        idx1 = sub_460b5b(40);
        v4 = index->field_8;
        *((long long *)&idx1->field_8) = *((long long *)&index->field_0);
        idx1->field_10 = v4;
        idx1->field_18 = a1;
        idx1->field_20 = sub_460cc6(a2);
        idx1->field_0 = idx->field_8;
        idx->field_8 = idx1;
    }
    return 0;
}



// Function: report_textual_dates @ 0x3589e
typedef struct struct_1 {
    char padding_0[8];
    struct struct_0 *field_8;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
    char padding_18[8];
    void* field_20;
} struct_0;

typedef struct struct_2 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char padding_18[8];
    unsigned long long field_20;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

extern unsigned int g_49f9ac;
extern struct_3 *g_49fdc8;

struct_2 * report_textual_dates(struct_1 *idx)
{
    struct_0 *v4;  // rax
    struct_0 *ptr;  // [bp-0x30]
    struct_2 *v1;  // [bp-0x28]
    void* v2;  // [bp-0x20]

    v4 = idx->field_8;
    for (ptr = idx->field_8; ptr; ptr = v1)
    {
        v1 = ptr->field_0;
        if (g_49f9ac)
        {
            v2 = sub_427abc(*((long long *)&ptr->field_8), ptr->field_10, 1);
            if (strcmp(ptr->field_20, v2))
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Option %s: Treating date '%s' as %s"));
            }
        }
        free(ptr->field_20);
        free(ptr);
    }
    return v4;
}



// Function: tar_help_filter @ 0x359a8
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
    unsigned long long field_20;
} struct_0;

unsigned long long tar_help_filter(int a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long v31;  // rax
    unsigned long v32;  // rax
    unsigned long v33;  // rax
    unsigned long v34;  // rax
    unsigned long v35;  // rax
    unsigned long v36;  // rax
    unsigned long v37;  // rax
    unsigned long v38;  // rax
    char *v39;  // rax
    char *v40;  // rax
    unsigned long long v0;  // [bp-0x130]
    void* ptr;  // [bp-0x118], Other Possible Types: unsigned long long
    void* ptr1;  // [bp-0x110]
    char *idx;  // [bp-0x108]
    unsigned long len;  // [bp-0x100]
    char *v5;  // [bp-0xf8]
    char *idx1;  // [bp-0xf0]
    unsigned long n;  // [bp-0xe8]
    char *v8;  // [bp-0xe0]
    char *idx2;  // [bp-0xd8]
    unsigned long len1;  // [bp-0xd0]
    char *v11;  // [bp-0xc8]
    char *v12;  // [bp-0xc0]
    unsigned long len2;  // [bp-0xb8]
    char *v14;  // [bp-0xb0]
    char *v15;  // [bp-0xa8]
    char *v16;  // [bp-0xa0]
    char *v17;  // [bp-0x98]
    char *v18;  // [bp-0x90]
    char *v19;  // [bp-0x88]
    char *v20;  // [bp-0x80]
    char *index;  // [bp-0x78]
    unsigned long long v22;  // [bp-0x70]
    struct_0 v23;  // [bp-0x68]
    unsigned long long v24;  // [bp-0x60]
    char *v25;  // [bp-0x58], Other Possible Types: unsigned long long
    char *ptr2;  // [bp-0x50], Other Possible Types: void*, unsigned long long
    unsigned long long v27;  // [bp-0x48]
    unsigned long v28;  // [bp-0x38]
    char v29;  // [bp-0x18]

    v0 = a2;
    switch (a0)
    {
    case 33554436:
        _obstack_begin(&v23, 0, 0, sub_460b5b, free);
        ptr1 = gettext("Valid arguments for the --quoting-style option are:");
        idx = &v23.padding_0[0];
        len = strlen(ptr1);
        v5 = idx;
        if (v27 - ptr2 < len)
            _obstack_newchunk(idx, len, len);
        memcpy(ptr2, ptr1, len);
        ptr2 += len;
        idx1 = &v23.padding_0[0];
        n = 2;
        v8 = idx1;
        if (v27 - ptr2 < n)
            _obstack_newchunk(idx1, n, n);
        memcpy(ptr2, "\n\n", n);
        ptr2 += n;
        sub_434eb6(&v23, "  ");
        ptr1 = gettext("\n*This* tar defaults to:\n");
        idx2 = &v23.padding_0[0];
        len1 = strlen(ptr1);
        v11 = idx2;
        if (v27 - ptr2 < len1)
            _obstack_newchunk(idx2, len1, len1);
        memcpy(ptr2, ptr1, len1);
        ptr2 += len1;
        ptr = sub_435152();
        v12 = &v23.padding_0[0];
        len2 = strlen(ptr);
        v14 = v12;
        if (v27 - ptr2 < len2)
            _obstack_newchunk(v12, len2, len2);
        memcpy(ptr2, ptr, len2);
        ptr2 += len2;
        v15 = &v23.padding_0[0];
        v16 = v15;
        if (!(v27 - ptr2))
            _obstack_newchunk(v15, 1);
        v39 = ptr2;
        ptr2 = v39 + 1;
        *(v39) = 10;
        v17 = &v23.padding_0[0];
        v18 = v17;
        if (!(v27 - ptr2))
            _obstack_newchunk(v17, 1);
        v40 = ptr2;
        ptr2 = v40 + 1;
        *(v40) = 0;
        v19 = &v23.padding_0[0];
        v20 = v25;
        if (v20 == ptr2)
            v29 |= 2;
        ptr2 = ~(v28) & &ptr2[v28];
        if (ptr2 - v24 > v27 - v24)
            ptr2 = v27;
        v25 = ptr2;
        ptr = sub_460cc6(v20);
        index = &v23.padding_0[0];
        v22 = 0;
        if (v22 > v24 && v22 < v27)
        {
            v25 = v22;
            ptr2 = v25;
            return ptr;
        }
        _obstack_free(index, v22, v22);
        return ptr;
    default:
        if (a0 <= 33554436)
        {
            if (a0 == 207)
            {
                v38 = gettext("filter the archive through %s");
                ptr = sub_461277(v38, "zstd", v38);
                return ptr;
            }
            else if (a0 <= 207)
            {
                if (a0 == 150)
                {
                    v37 = gettext("filter the archive through %s");
                    ptr = sub_461277(v37, "lzop", v37);
                    return ptr;
                }
                else if (a0 <= 150)
                {
                    if (a0 == 149)
                    {
                        v36 = gettext("filter the archive through %s");
                        ptr = sub_461277(v36, "lzma", v36);
                        return ptr;
                    }
                    else if (a0 <= 149)
                    {
                        if (a0 == 148)
                        {
                            v35 = gettext("filter the archive through %s");
                            ptr = sub_461277(v35, "lzip", v35);
                            return ptr;
                        }
                        else if (a0 <= 148)
                        {
                            if (a0 == 122)
                            {
                                v34 = gettext("filter the archive through %s");
                                ptr = sub_461277(v34, "gzip", v34);
                                return ptr;
                            }
                            else if (a0 <= 122)
                            {
                                if (a0 == 106)
                                {
                                    v33 = gettext("filter the archive through %s");
                                    ptr = sub_461277(v33, "bzip2", v33);
                                    return ptr;
                                }
                                else if (a0 <= 106)
                                {
                                    if (a0 == 74)
                                    {
                                        v32 = gettext("filter the archive through %s");
                                        ptr = sub_461277(v32, "xz", v32);
                                        return ptr;
                                    }
                                    else if (a0 == 90)
                                    {
                                        v31 = gettext("filter the archive through %s");
                                        ptr = sub_461277(v31, "compress", v31);
                                        return ptr;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ptr = a1;
        return ptr;
    }
}



// Function: expand_pax_option @ 0x361aa
typedef struct struct_3 {
    char padding_0[8];
    struct struct_2 *field_8;
} struct_3;

typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

typedef struct struct_2 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_2;

unsigned long long expand_pax_option(struct_3 *a0, char *a1)
{
    unsigned long long *v41;  // rax
    char *v42;  // rax
    char *v43;  // rax
    char *node;  // [bp-0x1a8]
    struct_0 *ptr1;  // [bp-0x190]
    unsigned long v2;  // [bp-0x188]
    char *v3;  // [bp-0x180]
    char *v4;  // [bp-0x178]
    char *v5;  // [bp-0x170]
    char *v6;  // [bp-0x168]
    unsigned long long v7;  // [bp-0x160]
    char *index;  // [bp-0x158]
    unsigned long long v9;  // [bp-0x150]
    unsigned long v10;  // [bp-0x148]
    char *v11;  // [bp-0x140]
    unsigned long n2;  // [bp-0x138]
    char *v13;  // [bp-0x130]
    char *v14;  // [bp-0x128]
    unsigned long v15;  // [bp-0x120]
    char *v16;  // [bp-0x118]
    void* ptr2;  // [bp-0x110]
    char *idx1;  // [bp-0x108]
    unsigned long n;  // [bp-0x100]
    char *v20;  // [bp-0xf8]
    void* v21;  // [bp-0xf0]
    char *idx;  // [bp-0xe8]
    unsigned long len;  // [bp-0xe0]
    char *v24;  // [bp-0xd8]
    char *idx2;  // [bp-0xd0]
    unsigned long n1;  // [bp-0xc8]
    char *v27;  // [bp-0xc0]
    char *v28;  // [bp-0xb8]
    char *v29;  // [bp-0xb0]
    unsigned int v30;  // [bp-0xa8]
    unsigned long v31;  // [bp-0xa0]
    char v32;  // [bp-0x98]
    unsigned long long v33;  // [bp-0x90]
    char *v34;  // [bp-0x88], Other Possible Types: unsigned long long
    void* ptr;  // [bp-0x80], Other Possible Types: char *, unsigned long, unsigned long long
    unsigned long long v36;  // [bp-0x78]
    unsigned long v37;  // [bp-0x68]
    char v38;  // [bp-0x48]
    char v39;  // [bp-0x38]

    node = a1;
    _obstack_begin(&v32, 0, 0, sub_460b5b, free);
    for (; *(node); node += 1)
    {
        v10 = strcspn(node, ",");
        ptr1 = memchr(node, 61, v10);
        if (ptr1)
        {
            v2 = ptr1 - node + 1;
            v14 = &v32;
            v15 = v2;
            v16 = v14;
            if (v36 - ptr < v15)
                _obstack_newchunk(v14, v15, v15);
            memcpy(ptr, node, v15);
            ptr = &ptr[v15];
            v2 = v10 - v2;
            for (ptr1 = &ptr1->field_1; ptr1->padding_0 && (v41 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)ptr1->padding_0 * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000); ptr1 = &ptr1->field_1)
            {
                v2 -= 1;
            }
            if (ptr1->padding_0 == 123 && *(&ptr1->padding_0[v2] - 1) == 125)
            {
                ptr2 = sub_460b5b(v2);
                memcpy(ptr2, &ptr1->field_1, v2 - 2);
                *((char *)ptr2 + v2 - 2) = 0;
                if (!sub_43564f(a0, "--pax-option", ptr2, &v30))
                {
                    v21 = sub_429f2e(v30, v31, &v39);
                    idx = &v32;
                    len = strlen(v21);
                    v24 = idx;
                    if (v36 - ptr < len)
                        _obstack_newchunk(idx, len, len);
                    memcpy(ptr, v21, len);
                    ptr += len;
                }
                else
                {
                    idx1 = &v32;
                    n = v2;
                    v20 = idx1;
                    if (v36 - ptr < n)
                        _obstack_newchunk(idx1, n, n);
                    memcpy(ptr, ptr1, n);
                    ptr += n;
                }
                free(ptr2);
            }
            else
            {
                idx2 = &v32;
                n1 = v2;
                v27 = idx2;
                if (v36 - ptr < n1)
                    _obstack_newchunk(idx2, n1, n1);
                memcpy(ptr, ptr1, n1);
                ptr += n1;
            }
        }
        else
        {
            v11 = &v32;
            n2 = v10;
            v13 = v11;
            if (v36 - ptr < n2)
                _obstack_newchunk(v11, n2, n2);
            memcpy(ptr, node, n2);
            ptr = &ptr[n2];
        }
        node = &node[v10];
        if (*(node))
        {
            v28 = &v32;
            v29 = v28;
            if (!(v36 - ptr))
                _obstack_newchunk(v28, 1);
            v42 = ptr;
            ptr = v42 + 1;
            *(v42) = *(node);
        }
    }
    v3 = &v32;
    v4 = v3;
    if (!(v36 - ptr))
        _obstack_newchunk(v3, 1);
    v43 = ptr;
    ptr = v43 + 1;
    *(v43) = 0;
    v5 = &v32;
    v6 = v34;
    if (v6 == ptr)
        v38 |= 2;
    ptr = ~(v37) & &ptr[v37];
    if (ptr - v33 > v36 - v33)
        ptr = v36;
    v34 = ptr;
    v7 = sub_460cc6(v6);
    index = &v32;
    v9 = 0;
    if (v9 > v33 && v9 < v36)
    {
        v34 = v9;
        ptr = v34;
        return v7;
    }
    _obstack_free(index, v9, v9);
    return v7;
}



// Function: parse_owner_group @ 0x369f0
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern long long g_4869f3;
extern struct_1 *g_49fdc8;

unsigned long long parse_owner_group(char *a0, unsigned long long a1, struct_0 **a2)
{
    unsigned int v8;  // eax
    unsigned long long v0;  // [bp-0x58]
    char v1;  // [bp-0x50]
    char v2;  // [bp-0x48]
    char *v3;  // [bp-0x40]
    char *v4;  // [bp-0x38]
    char *ptr;  // [bp-0x30]
    char *v6;  // [bp-0x28]

    v0 = 0xffffffffffffffff;
    v3 = NULL;
    v4 = NULL;
    ptr = strchr(a0, 58);
    if (ptr)
    {
        v6 = ptr + 1;
        *(ptr) = 0;
        if (*(a0))
            v3 = a0;
        if (v6 && (sub_460e19(v6, &v1, 10, &v0, &g_4869f3) || a1 < v0))
            v4 = v6;
    }
    else
    {
        if (*(a0) > 47 && *(a0) <= 57)
            v8 = sub_460e19(a0, &v1, 10, &v2, &g_4869f3);
        else
            v8 = 4;
        if (v8)
        {
            if (v8 == 1)
                goto LABEL_436b1f;
            v3 = a0;
        }
        else if (a1 >= v2)
        {
            v0 = v2;
        }
        else
        {
LABEL_436b1f:
            v4 = a0;
        }
    }
    if (v4)
    {
        if (g_49fdc8)
            g_49fdc8();
        gettext("Invalid owner or group ID");
        sub_45e548(v4);
        error(0, 0, "%s: %s");
        sub_4177f4(); /* do not return */
    }
    else
    {
        if (!v3)
            return v0;
        *(a2) = v3;
        return v0;
    }
}



// Function: set_old_files_option @ 0x36bbb
extern unsigned long long g_49aa20[4];
extern unsigned int g_49f8d8;

unsigned int set_old_files_option(unsigned int a0, void* a1)
{
    void* v0;  // [bp-0x10]

    v0 = sub_43521d(9, a1);
    if (v0 && sub_435388(a1, v0) && a0 != g_49f8d8)
        sub_4351b6(g_49aa20[a0], g_49aa20[g_49f8d8]); /* do not return */
    g_49f8d8 = a0;
    return a0;
}



// Function: parse_opt @ 0x36c56
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    struct struct_2 *field_0;
    char padding_8[8];
    char field_10;
    char field_11;
    char field_12;
    char padding_13[5];
    struct struct_3 *field_18;
    struct struct_3 *field_20;
} struct_1;

typedef struct struct_2 {
    unsigned int field_0;
} struct_2;

typedef struct struct_4 {
    struct struct_0 *field_0;
    char padding_8[32];
    struct struct_1 *field_28;
    unsigned long long field_30;
} struct_4;

typedef struct struct_3 {
    char field_0;
} struct_3;

typedef struct struct_5 {
    unsigned long long field_0;
} struct_5;

extern unsigned int g_4867c8[4];
extern long long g_4869f3;
extern unsigned int g_486a30[4];
extern long long g_49a950;
extern long long g_49a980;
extern long long g_49a9a0;
extern unsigned int g_49cb40[4];
extern unsigned int g_49ec84;
extern unsigned long long g_49ece0;
extern long long g_49ee48;
extern int g_49f868;
extern unsigned long long g_49f870;
extern char g_49f878;
extern char g_49f879;
extern char g_49f87a;
extern unsigned int g_49f87c;
extern unsigned int g_49f880;
extern char g_49f884;
extern char g_49f88c;
extern unsigned int g_49f890;
extern char g_49f8a0;
extern char g_49f8a1;
extern char *g_49f8b0;
extern unsigned int g_49f8b8;
extern char g_49f8bc;
extern char g_49f8bd;
extern char g_49f8be;
extern unsigned long long g_49f8c0;
extern char g_49f8c8;
extern unsigned long long g_49f8d0;
extern char g_49f8dc;
extern unsigned long long g_49f8e0;
extern unsigned int g_49f8e8;
extern char g_49f8ec;
extern unsigned long long g_49f8f0;
extern unsigned int g_49f8f8;
extern char g_49f8fc;
extern struct_0 g_49f900;
extern long long g_49f908;
extern unsigned int g_49f910;
extern struct_0 g_49f920;
extern char g_49f934;
extern char g_49f935;
extern char g_49f936;
extern unsigned long long g_49f938;
extern char *g_49f940;
extern unsigned int g_49f948;
extern char g_49f94c;
extern char g_49f94d;
extern char g_49f94e;
extern unsigned long long g_49f950;
extern char g_49f958;
extern unsigned int g_49f95c;
extern unsigned int g_49f960;
extern unsigned int g_49f964;
extern unsigned int g_49f968;
extern unsigned long long g_49f970;
extern char g_49f978;
extern char g_49f979;
extern unsigned int g_49f97c;
extern unsigned int g_49f980;
extern unsigned int g_49f984;
extern unsigned long long g_49f990;
extern char g_49f998;
extern char g_49f999;
extern char g_49f99a;
extern unsigned long long g_49f9a0;
extern char g_49f9a8;
extern char g_49f9a9;
extern unsigned int g_49f9ac;
extern char g_49f9b0;
extern unsigned long long g_49f9b8;
extern unsigned long long g_49f9c0;
extern long long g_49fbd8;
extern unsigned long long g_49fbe0;
extern unsigned long long g_49fbf0;
extern unsigned int g_49fc14;
extern unsigned int g_49fc2c;
extern char g_49fc30;
extern char g_49fc31;
extern unsigned int g_49fc34;
extern long long g_49fc38;
extern struct_5 *g_49fdc8;
extern char g_49fdf8;

unsigned int parse_opt(unsigned int a0, char *a1, struct_4 *index)
{
    unsigned long long idx;  // rax
    unsigned long v11;  // rax
    long long v22;  // rdi
    long long v23;  // rsi
    long long v24;  // rdx
    long long v25;  // rcx
    long long v26;  // r8
    long long v27;  // r9
    long long v28;  // rdi
    long long v29;  // rsi
    unsigned long v12;  // rax
    long long v30;  // rdx
    long long v31;  // rcx
    long long v32;  // r8
    long long v33;  // r9
    unsigned long v13;  // rax
    char *iter;  // [bp-0x78]
    int i;  // [bp-0x5c]
    unsigned long v2;  // [bp-0x58]
    unsigned long long v3;  // [bp-0x50]
    struct_1 *idx1;  // [bp-0x48]
    void* v5;  // [bp-0x40]
    unsigned long v6;  // [bp-0x38]
    unsigned long long v7;  // [bp-0x30]
    unsigned long long v8;  // [bp-0x28]

    iter = a1;
    idx1 = index->field_28;
    if (a0 == 16777221)
    {
        if (idx1->field_0->field_0 == 2)
        {
            error(0, 0, gettext("%s:%lu: location of the error"));
        }
        else
        {
            if (!idx1->field_0->field_0)
                error(0, 0, gettext("error parsing %s"));
        }
        exit(64); /* do not return */
    }
    else if (a0 <= 16777221)
    {
        if (a0 <= 207)
        {
            if (a0 >= 0)
            {
                switch (a0)
                {
                case 0:
                    sub_42c829(iter);
                    break;
                case 1: case 2: case 3: case 4: case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15: case 16: case 17: case 18: case 19: case 20: case 21: case 22: case 23: case 24: case 25: case 26: case 27: case 28: case 29: case 30: case 31: case 32: case 33: case 34: case 35: case 36: case 37: case 38: case 39: case 40: case 41: case 42: case 43: case 44: case 45: case 46: case 47: case 56: case 57: case 58: case 59: case 60: case 61: case 62: case 63: case 64: case 67: case 68: case 69: case 81: case 84: case 88: case 89: case 91: case 92: case 93: case 94: case 95: case 96: case 101: case 113: case 121: case 123: case 124: case 125: case 126: case 127:
                    return 7;
                case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55:
                    v11 = gettext("Options '-[0-7][lmh]' not supported by *this* tar");
                    argp_error(index, v11, v11);
                    exit(64); /* do not return */
                case 65:
                    sub_4353ea(2);
                    break;
                case 66:
                    g_49f94d = 1;
                    break;
                case 70:
                    g_49f8c0 = iter;
                    g_49f8fc = 1;
                    break;
                case 72:
                    sub_434c7e(iter);
                    break;
                case 73:
                    sub_43545f(iter, idx1->field_0);
                    break;
                case 74:
                    sub_43545f("xz", idx1->field_0);
                    break;
                case 75:
                    sub_43521d(5, idx1->field_0);
                    sub_42d6a2(iter);
                    break;
                case 76:
                    if (sub_460e19(iter, &v3, 10, &v2, "bBcGgkKMmPTtw"))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        gettext("Invalid tape length");
                        sub_45e548(iter);
                        error(0, 0, "%s: %s");
                        sub_43814c(2); /* do not return */
                    }
                    if (iter < v3 && !strchr("bBcGgkKMmPTtw", *((char *)(v3 - 1))))
                    {
                        v12 = v2;
                        g_49f990 = MulV((v12 < 0 ? AddV(v12 >> 1 | (unsigned int)v12 & 1, v12 >> 1 | (unsigned int)v12 & 1) : v12), 0x4090000000000000);
                    }
                    else
                    {
                        v13 = v2;
                        g_49f990 = (v13 < 0 ? (unsigned long long)(AddV(v13 >> 1 | (unsigned int)v13 & 1, v13 >> 1 | (unsigned int)v13 & 1)) : v13);
                    }
                    g_49f8fc = 1;
                    break;
                case 77:
                    g_49f8fc = 1;
                    break;
                case 79:
                    g_49f998 = 1;
                    break;
                case 80:
                    sub_43521d(8, idx1->field_0);
                    g_49f878 = 1;
                    break;
                case 82:
                    g_49f88c = 1;
                    break;
                case 83:
                    g_49f979 = 1;
                    break;
                case 85:
                    sub_436bbb(3, idx1->field_0);
                    break;
                case 86:
                    g_49f9c0 = iter;
                    break;
                case 87:
                    sub_43521d(4, idx1->field_0);
                    g_49f9b0 = 1;
                    break;
                case 90:
                    sub_43545f("compress", idx1->field_0);
                    break;
                case 97:
                    idx1->field_12 = 1;
                    break;
                case 98:
                    if (sub_460e19(iter, 0, 10, &v3, &g_4869f3) || !(g_49f868 = (int)(unsigned int)v3, (long long)g_49f868 == v3 && g_49f868 > 0 && (g_49f870 = v3 * 0x200, g_49f870 >> 9 == v3)))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        gettext("Invalid blocking factor");
                        sub_45e548(iter);
                        error(0, 0, "%s: %s");
                        sub_43814c(2); /* do not return */
                    }
                    break;
                case 99:
                    sub_4353ea(3);
                    break;
                case 100:
                    sub_4353ea(5);
                    break;
                case 102:
                    if (g_49fbe0 == g_49fc38)
                        g_49fbd8 = sub_460a5f(g_49fbd8, &g_49fc38, 8, &g_49fc38);
                    idx = g_49fbe0;
                    g_49fbe0 = idx + 1;
                    *((char **)(g_49fbd8 + idx * 8)) = iter;
                    break;
                case 104:
                    g_49f8a0 = 1;
                    break;
                case 105:
                    g_49f8bd = 1;
                    break;
                case 106:
                    sub_43545f("bzip2", idx1->field_0);
                    break;
                case 107:
                    sub_436bbb(4, idx1->field_0);
                    break;
                case 108:
                    g_49fc34 = 1;
                    break;
                case 109:
                    g_49f99a = 1;
                    break;
                case 110:
                    g_49fc14 = 1;
                    break;
                case 111:
                    idx1->field_10 = 1;
                    break;
                case 112:
                    g_49f960 = 1;
                    break;
                case 114:
                    sub_4353ea(1);
                    break;
                case 115:
                    sub_43521d(6, idx1->field_0);
                    g_49f958 = 1;
                    break;
                case 116:
                    sub_4353ea(7);
                    g_49f9ac = g_49f9ac + 1;
                    break;
                case 117:
                    sub_4353ea(8);
                    break;
                case 118:
                    g_49f9ac = g_49f9ac + 1;
                    g_49ec84 = g_49ec84 | 0x583000;
                    break;
                case 119:
                    g_49f8c8 = 1;
                    break;
                case 120:
                    sub_4353ea(6);
                    break;
                case 122:
                    sub_43545f("gzip", idx1->field_0);
                    break;
                case 128:
                    sub_434c7e("posix");
                    g_49f968 = 1;
                    break;
                case 129:
                    g_49f880 = (unsigned int)(!iter ? 0x1 : g_4867c8[sub_447c61("--atime-preserve", iter, &g_49a950, &g_4867c8[0], 4, g_49ee48)]);
                    break;
                case 130:
                    g_49f884 = 1;
                    if (iter)
                        idx1->field_20 = iter;
                    break;
                case 131:
                    g_49f8ec = 1;
                    break;
                case 132:
                    if (iter)
                    {
                        if (*(iter) == 46)
                        {
                            sub_40f825(".");
                            iter += 1;
                        }
                        g_49f890 = strtoul(iter, &v3, 0, &v3);
                        if (*((char *)v3))
                        {
                            if (g_49fdc8)
                                g_49fdc8();
                            error(0, 0, gettext("--checkpoint value is not an integer"));
                            sub_4177f4(); /* do not return */
                        }
                    }
                    else
                    {
                        g_49f890 = 10;
                    }
                    break;
                case 133:
                    sub_40f825(iter);
                    break;
                case 134:
                    g_49f910 = 2;
                    break;
                case 135:
                    g_49fc31 = 1;
                    break;
                case 136:
                    g_49f8a1 = 1;
                    break;
                case 137:
                    sub_4353ea(4);
                    break;
                case 138:
                    g_49fdf8 = 1;
                    break;
                case 139:
                    g_49f87a = 1;
                    break;
                case 140:
                    v8 = sub_4369f0(iter, 0xffffffff, &g_49f8b0);
                    if (v8 == 0xffffffffffffffff)
                    {
                        g_49f8b8 = 0xffffffff;
                        if (g_49f8b0)
                            sub_42c261(g_49f8b0, &g_49f8b8);
                    }
                    else
                    {
                        g_49f8b8 = v8;
                    }
                    break;
                case 141:
                    sub_42938e(*((unsigned int *)&iter));
                    break;
                case 142:
                    g_49f9a8 = 1;
                    break;
                case 143:
                    g_49f8bc = 1;
                    break;
                case 144:
                    g_49fbf0 = iter;
                    break;
                case 145:
                    g_49f8dc = 1;
                    break;
                case 146:
                    sub_436bbb(6, idx1->field_0);
                    break;
                case 147:
                    g_49f8e8 = strtoul(iter, &v3, 10, &v3);
                    if (*((char *)v3))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("Invalid incremental level value"));
                        sub_43814c(2); /* do not return */
                    }
                    break;
                case 148:
                    sub_43545f("lzip", idx1->field_0);
                    break;
                case 149:
                    sub_43545f("lzma", idx1->field_0);
                    break;
                case 150:
                    sub_43545f("lzop", idx1->field_0);
                    break;
                case 151:
                    g_49f8f0 = sub_455658(iter);
                    if (!g_49f8f0)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("Invalid mode given on option"));
                        sub_4177f4(); /* do not return */
                    }
                    g_49f8f8 = umask(0);
                    umask(g_49f8f8);
                    break;
                case 152:
                    sub_43564f(idx1, "--mtime", iter, &g_49f920.field_0);
                    if (!g_49f910)
                        g_49f910 = 1;
                    break;
                case 154:
                    g_49f968 = 0xffffffff;
                    break;
                case 155:
                    idx1->field_12 = 0;
                    break;
                case 156:
                    g_49f8ec = 0;
                    break;
                case 157:
                    g_49fc31 = 0;
                    break;
                case 158:
                    g_49f9a8 = 0;
                    break;
                case 159:
                    sub_436bbb(1, idx1->field_0);
                    break;
                case 160:
                    for (; *(iter); iter += 1)
                    {
                        sub_45c9cd(0, *(iter), 0);
                    }
                case 161:
                    g_49f95c = 0xffffffff;
                    break;
                case 162:
                    g_49f960 = 0xffffffff;
                    break;
                case 163:
                    g_49fc14 = 0;
                    break;
                case 164:
                    g_49f964 = 0xffffffff;
                    break;
                case 165:
                    sub_434d36(0xffffffff);
                    break;
                case 166:
                    g_49f934 = 1;
                    break;
                case 167:
                    sub_43521d(1, idx1->field_0);
                    if (!iter)
                    {
                        g_49f8d0 = 1;
                        break;
                    }
                    else if (!sub_460e19(iter, 0, 10, &v3, &g_4869f3))
                    {
                        g_49f8d0 = v3;
                        break;
                    }
                    else
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        gettext("Invalid number");
                        sub_45e548(iter);
                        error(0, 0, "%s: %s");
                        sub_4177f4(); /* do not return */
                    }
                case 168:
                    sub_434c7e("v7");
                    break;
                case 169:
                    g_49f935 = 1;
                    break;
                case 170:
                    sub_43521d(7, idx1->field_0);
                    g_49f936 = 1;
                    g_49f938 = iter;
                    break;
                case 171:
                    sub_436bbb(0, idx1->field_0);
                    break;
                case 172:
                    sub_436bbb(2, idx1->field_0);
                    break;
                case 173:
                    v7 = sub_4369f0(iter, 0xffffffff, &g_49f940);
                    if (v7 == 0xffffffffffffffff)
                    {
                        g_49f948 = 0xffffffff;
                        if (g_49f940)
                            sub_42c173(g_49f940, &g_49f948);
                    }
                    else
                    {
                        g_49f948 = v7;
                    }
                    break;
                case 174:
                    sub_429234(*((unsigned int *)&iter));
                    break;
                case 175:
                    v6 = sub_4361aa(idx1, iter);
                    idx1->field_11 = 1;
                    sub_41cc78(v6);
                    free(v6);
                    break;
                case 176:
                    sub_434c7e("posix");
                    break;
                case 177:
                    for (; *(iter); iter += 1)
                    {
                        sub_45c9cd(0, *(iter), 1);
                    }
                case 178:
                    sub_435084(iter);
                    break;
                case 179:
                    if (sub_460e19(iter, 0, 10, &v3, "bBcGgkKMmPTtw"))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        gettext("Invalid record size");
                        sub_45e548(iter);
                        error(0, 0, "%s: %s");
                        sub_43814c(2); /* do not return */
                    }
                    g_49f870 = v3;
                    if ((unsigned short)g_49f870 & 0x1ff)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("Record size must be a multiple of %d."));
                        sub_43814c(2); /* do not return */
                    }
                    g_49f868 = g_49f870 >> 9;
                    break;
                case 180:
                    g_49f94c = 1;
                    break;
                case 181:
                    g_49f94e = 1;
                    break;
                case 182:
                    g_49f9a9 = 1;
                    break;
                case 183:
                    g_49ece0 = iter;
                    break;
                case 184:
                    g_49f950 = iter;
                    break;
                case 185:
                    g_49f95c = 1;
                    break;
                case 186:
                    sub_434c7e("posix");
                    g_49f964 = 1;
                    break;
                case 187:
                    v5 = sub_435152();
                    printf("%s\n", v5);
                    sub_44e014(v22, v23, v24, v25, v26, v27);
                    free(v5);
                    exit(0); /* do not return */
                case 188:
                    g_49f978 = 1;
                    break;
                case 189:
                    sub_423f6b();
                    sub_44e014(v28, v29, v30, v31, v32, v33);
                    exit(0); /* do not return */
                case 190:
                    g_49fc30 = 1;
                    break;
                case 191:
                    sub_436bbb(5, idx1->field_0);
                    break;
                case 192:
                    g_49fc2c = g_49cb40[sub_447c61("--sort", iter, &g_49a980, &g_49cb40[0], 4, g_49ee48)];
                    break;
                case 193:
                    g_49f984 = g_486a30[sub_447c61("--hole-detection", iter, &g_49a9a0, &g_486a30[0], 4, g_49ee48)];
                    g_49f979 = 1;
                    break;
                case 194:
                    g_49f979 = 1;
                    g_49f97c = strtoul(iter, &v3, 10, &v3);
                    if (*((char *)v3))
                    {
                        if (*((char *)v3) != 46)
                        {
                            if (g_49fdc8)
                                g_49fdc8();
                            error(0, 0, gettext("Invalid sparse version value"));
                            sub_43814c(2); /* do not return */
                        }
                        else
                        {
                            g_49f980 = strtoul(v3 + 1, &v3, 10, v3 + 1);
                            if (*((char *)v3))
                            {
                                if (g_49fdc8)
                                    g_49fdc8();
                                error(0, 0, gettext("Invalid sparse version value"));
                                sub_43814c(2); /* do not return */
                            }
                        }
                    }
                    break;
                case 195:
                    if (sub_460e19(iter, 0, 10, &v3, &g_4869f3))
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        gettext("Invalid number of elements");
                        sub_45e548(iter);
                        error(0, 0, "%s: %s");
                        sub_43814c(2); /* do not return */
                    }
                    g_49f970 = v3;
                    break;
                case 196:
                    g_49f884 = 1;
                    idx1->field_18 = iter;
                    break;
                case 197:
                    sub_4353ea(9);
                    break;
                case 198:
                    if (iter)
                    {
                        sub_435629(iter);
                        break;
                    }
                    else
                    {
                        g_49f999 = 1;
                        break;
                    }
                case 199:
                    if (g_49f9a0)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("Only one --to-command option allowed"));
                        sub_43814c(2); /* do not return */
                    }
                    g_49f9a0 = iter;
                    break;
                case 200:
                    sub_43ae0b(iter);
                    break;
                case 201:
                    g_49f879 = 1;
                    break;
                case 202:
                    g_49f9b8 = iter;
                    break;
                case 203:
                    sub_43c9ee(iter);
                    break;
                case 204:
                    sub_434d36(1);
                    break;
                case 205: case 206:
                    sub_434d36(1);
                    sub_43d0e5(iter, a0 == 206, a0 == 206);
                    break;
                case 207:
                    sub_43545f("zstd", idx1->field_0);
                    break;
                case 78:
                    g_49f87c = 1;
                case 153:
                    if (g_49f908 >= 0)
                    {
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("More than one threshold date"));
                        sub_43814c(2); /* do not return */
                    }
                    sub_43564f(idx1, (a0 == 0x99 ? "--newer-mtime" : "--after-date"), iter, &g_49f900.field_0);
                    sub_43521d(3, idx1->field_0);
                    break;
                case 103:
                    sub_43521d(2, idx1->field_0);
                    g_49f8e0 = iter;
                    g_49f87c = 1;
                case 71:
                    g_49f8be = 1;
                    break;
                default:
                    return 7;
                }
            }
            else
            {
                return 7;
            }
        }
        else
        {
            if (a0 != 16777219)
            {
                return 7;
            }
            else if (index->field_0->field_20)
            {
                for (i = 0; *((long long *)(i * 32 + index->field_0->field_20)); i += 1)
                {
                    *((struct struct_1 **)(index->field_30 + i * 8)) = index->field_28;
                }
            }
        }
        return 0;
    }
    else
    {
        return 7;
    }
}



// Function: usage @ 0x3814c
extern long long g_49eb60;
extern long long stderr;
extern long long g_49fe48;

void usage(int a0)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    long long v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9

    argp_help(&g_49eb60, stderr, 4, g_49fe48);
    sub_44e014(v1, v2, v3, v4, v5, v6);
    exit(a0); /* do not return */
}



// Function: find_argp_option_key @ 0x38192
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
    struct_0 *v0;  // [bp-0x10]

    v0 = a0;
    while (1)
    {
        if (!v0->field_0 && !v0->field_8 && !v0->field_10 && !v0->field_18 && !v0->field_20)
            return NULL;
        if (a1 == v0->field_8)
            return v0;
        v0 = &v0[1].field_8;
    }
}



// Function: find_argp_option @ 0x381fd
typedef struct struct_1 {
    struct struct_0 *field_0;
    char padding_8[24];
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[4];
    unsigned long long field_10;
    unsigned int field_18;
    char padding_1c[4];
    unsigned long long field_20;
} struct_0;

struct_0 * find_argp_option(struct_1 *a0, unsigned int a1)
{
    struct_0 *v0;  // [bp-0x18], Other Possible Types: unsigned long long
    struct_0 **i;  // [bp-0x10]

    v0 = 0;
    v0 = sub_438192(a0->field_0, a1);
    if (v0)
    {
        return v0;
    }
    else if (a0->field_20)
    {
        for (i = a0->field_20; *(i); i += 4)
        {
            v0 = sub_438192(*(i)->field_0, a1);
            if (v0)
                return v0;
        }
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: more_options @ 0x3828e
extern long long g_49c920;

void more_options(unsigned int a0, long long a1, unsigned long a2)
{
    unsigned long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    char v2;  // [bp-0x28]
    char v3;  // [bp-0x27]
    char v4;  // [bp-0x26]
    unsigned long long v5;  // [bp-0x20]
    unsigned long long v6;  // [bp-0x18]

    v0 = a2;
    v1 = 0;
    v2 = 0;
    v3 = 0;
    v4 = 0;
    v5 = 0;
    v6 = 0;
    argp_parse(&g_49c920, a0, a1, 42, 0, &v0);
    return;
}



// Function: parse_default_options @ 0x38321
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern long long g_49eb60;
extern struct_1 *g_49fdc8;
extern unsigned long long g_49fe48;

void parse_default_options(struct_0 **a0)
{
    char v0;  // [bp-0x13c]
    unsigned long v1;  // [bp-0x138]
    char *v2;  // [bp-0x130]
    unsigned int v3;  // [bp-0x128]
    char *v4;  // [bp-0x120]
    unsigned long long v5;  // [bp-0x118]
    unsigned long long v6;  // [bp-0x110]
    unsigned long v7;  // [bp-0x108]
    unsigned long long *v8;  // [bp-0x100]
    unsigned long long v9;  // [bp-0xf8]

    v1 = getenv("TAR_OPTIONS");
    v3 = 0;
    v4 = "TAR_OPTIONS";
    v5 = 0;
    v6 = 0;
    if (!v1)
        return;
    v9 = 1;
    if (sub_4461a6(v1, &v7, 33558086, &v7))
    {
        if (g_49fdc8)
            g_49fdc8();
        sub_44647f(&v7);
        error(0, 0, gettext("cannot split TAR_OPTIONS: %s"));
        sub_4177f4(); /* do not return */
    }
    else
    {
        if (v7)
        {
            *(v8) = g_49fe48;
            v2 = &*(a0)->field_0;
            *(a0) = &v3;
            if (argp_parse(&g_49eb60, v7 + (v9 & 0xffffffff) & 0xffffffff, v8, 40, &v0, a0))
                abort(); /* do not return */
            *(a0) = v2;
            if ((char)sub_434a5e())
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("non-option arguments in %s"));
                sub_43814c(2); /* do not return */
            }
            v7 = 0;
        }
        sub_446399(&v7);
        return;
    }
}



// Function: decode_options @ 0x3854d
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_1 *field_8;
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_2 {
    struct struct_0 *field_0;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

extern unsigned int g_49cb60[4];
extern char g_49eb60;
extern long long g_49eba0;
extern unsigned long long stdout;
extern unsigned long long stderr;
extern unsigned long long g_49efd0;
extern unsigned int g_49f860;
extern unsigned int g_49f864;
extern unsigned int g_49f868;
extern void g_49f870;
extern char g_49f878;
extern char g_49f879;
extern unsigned int g_49f880;
extern char g_49f884;
extern unsigned int g_49f888;
extern unsigned long long g_49f898;
extern char g_49f8a0;
extern unsigned long long g_49f8a8;
extern unsigned long long g_49f8b0;
extern unsigned int g_49f8b8;
extern char g_49f8be;
extern unsigned long long g_49f8d0;
extern unsigned int g_49f8d8;
extern unsigned long long g_49f8e0;
extern unsigned int g_49f8e8;
extern char g_49f8ec;
extern char g_49f8fc;
extern unsigned long long g_49f900;
extern unsigned long long g_49f908;
extern unsigned int g_49f910;
extern unsigned long long g_49f920;
extern unsigned long long g_49f928;
extern unsigned int g_49f930;
extern char g_49f936;
extern unsigned long long g_49f938;
extern unsigned long long g_49f940;
extern unsigned int g_49f948;
extern char g_49f94c;
extern char g_49f958;
extern unsigned int g_49f95c;
extern int g_49f964;
extern int g_49f968;
extern int g_49f96c;
extern char g_49f979;
extern unsigned int g_49f97c;
extern unsigned int g_49f980;
extern unsigned int g_49f984;
extern char g_49f988;
extern unsigned long long g_49f990;
extern char g_49f998;
extern unsigned long long g_49f9a0;
extern unsigned int g_49f9ac;
extern char g_49f9b0;
extern void g_49f9c0;
extern char g_49f9c8;
extern void* g_49fbd8;
extern unsigned long long g_49fbe0;
extern struct_3 *g_49fbe8;
extern void g_49fbf0;
extern unsigned int g_49fc08;
extern unsigned int g_49fc0c;
extern unsigned int g_49fc10;
extern unsigned int g_49fc14;
extern char g_49fc28;
extern unsigned int g_49fc2c;
extern struct_2 *g_49fdc8;
extern unsigned long long g_49fe30;

long long decode_options(unsigned int a0, struct_0 *a1)
{
    long long v34;  // rdi
    long long v35;  // rsi
    char *v44;  // rax
    long long v36;  // rdx
    unsigned long v62;  // xmm0lq
    unsigned long long v63;  // rdx
    long long v37;  // rcx
    unsigned long v64;  // rax
    unsigned long v68;  // fs
    long long v38;  // r8
    long long v39;  // r9
    struct_0 *v40;  // rax
    struct_0 *v0;  // [bp-0x108]
    unsigned int v1;  // [bp-0xfc]
    char j;  // [bp-0xec], Other Possible Types: unsigned int
    unsigned int v3;  // [bp-0xe8]
    unsigned int v4;  // [bp-0xe4]
    struct_0 *node;  // [bp-0xe0]
    struct_0 *iter;  // [bp-0xd8]
    char *iter1;  // [bp-0xd0]
    struct_0 *v8;  // [bp-0xc8]
    struct_0 *v9;  // [bp-0xc0]
    unsigned int *v10;  // [bp-0xb8]
    void* v11;  // [bp-0xb0]
    unsigned long v12;  // [bp-0xa8]
    unsigned int *v13;  // [bp-0xa0]
    void* v14;  // [bp-0x98]
    void* v15;  // [bp-0x90]
    unsigned int *v16;  // [bp-0x88]
    void* v17;  // [bp-0x80]
    unsigned int v18;  // [bp-0x78]
    unsigned long long v19;  // [bp-0x70]
    unsigned long long v20;  // [bp-0x68]
    unsigned long long v21;  // [bp-0x60]
    char *v22;  // [bp-0x58]
    unsigned long long v23;  // [bp-0x50]
    char v24;  // [bp-0x48]
    char v25;  // [bp-0x47]
    char v26;  // [bp-0x46]
    unsigned long v27;  // [bp-0x40], Other Possible Types: unsigned long long
    long long v28;  // [bp-0x38]
    char v29;  // [bp-0x23]
    char v30;  // [bp-0x22]
    char v31;  // [bp-0x21]
    unsigned long v32;  // [bp-0x20]

    v1 = a0;
    v0 = a1;
    v18 = 1;
    v19 = 0;
    v20 = 0;
    v21 = 0;
    v22 = &v18;
    v23 = 0;
    v24 = 0;
    v25 = 0;
    v26 = 0;
    v27 = 0;
    v28 = 0;
    sub_44d302("tar", &g_49eba0);
    v27 = getenv("SIMPLE_BACKUP_SUFFIX");
    g_49f9c8 = getenv("POSIXLY_CORRECT");
    g_49f860 = 0;
    g_49f864 = 0;
    g_49f868 = 20;
    *((unsigned long *)&g_49f870) = 0x2800;
    g_49f8a8 = sub_44e53e(v34, v35, v36, v37, v38, v39);
    g_49f984 = 0;
    g_49f900 = 0x8000000000000000;
    g_49f908 = 0xffffffffffffffff;
    g_49f920 = 0x8000000000000000;
    g_49f928 = 0xffffffffffffffff;
    g_49f930 = 8;
    g_49fc28 = 1;
    g_49f97c = 1;
    g_49f980 = 0;
    g_49fc2c = 0;
    g_49f948 = 0xffffffff;
    g_49f940 = 0;
    g_49f8b8 = 0xffffffff;
    g_49f8b0 = 0;
    g_49f8ec = 1;
    g_49f8e8 = 0xffffffff;
    g_49fc14 = 0xffffffff;
    if (v1 > 1 && v0->field_8->field_0 != 45)
    {
        v29 = 45;
        v31 = 0;
        v3 = v1 + (unsigned int)strlen(v0->field_8) - 1;
        v8 = sub_460b5b((v3 + 1) * 8);
        node = v0;
        iter = v8;
        node = &node->field_8;
        iter = &iter->field_8;
        *((long long *)&iter->field_0) = *((long long *)&node->field_0);
        v40 = node;
        node = &v40->field_8;
        iter1 = *((long long *)&v40->field_0);
        while (1)
        {
            if (!*(iter1))
                break;
            v30 = *(iter1);
            iter = &iter->field_8;
            *((unsigned long long *)&iter->field_0) = sub_460cc6(&v29);
            v9 = sub_4381fd(&g_49eb60, *(iter1));
            if (v9 && v9->field_10)
            {
                if (node < &(&v0->field_0)[2 * v1])
                {
                    node = &node->field_8;
                    iter = &iter->field_8;
                    *((long long *)&iter->field_0) = *((long long *)&node->field_0);
                }
                else
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Old option '%c' requires an argument."));
                    sub_43814c(2); /* do not return */
                }
            }
            iter1 += 1;
        }
        for (; node < &(&v0->field_0)[2 * v1]; iter = iter)
        {
            node = &node->field_8;
            iter = &iter->field_8;
            *((long long *)&iter->field_0) = *((long long *)&node->field_0);
        }
        *((char **)&iter->field_0) = NULL;
        v1 = v3;
        v0 = v8;
    }
    sub_438321(&v22);
    if (argp_parse(&g_49eb60, v1, v0, 8, &j, &v22))
        exit(2); /* do not return */
    if (v24)
    {
        if (g_49f860 == 3)
            sub_434c7e("v7");
        else
            g_49f95c = 0xffffffff;
    }
    for (; v1 > j; j += 1)
    {
        sub_42c829(*((long long *)&(&v0->field_0)[2 * j]));
    }
    if (!g_49f864)
    {
        if (v25)
            g_49f864 = 4;
        else
            g_49f864 = 6;
    }
    if (*((long long *)&g_49f9c0) && g_49f860 == 3 || g_49f8be || g_49f8fc || g_49f979)
        sub_434dad(84);
    if (g_49f8d0)
    {
        if ((char)sub_434a5e() != 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("--occurrence is meaningless without a file list"));
            sub_43814c(2); /* do not return */
        }
        else if (!((char)g_49cb60[g_49f860] & 16))
        {
            if (sub_43534c(1))
                sub_4351b6("--occurrence", sub_434e1c(g_49f860)); /* do not return */
            g_49f8d0 = 0;
        }
    }
    if (!g_49fbe0)
    {
        g_49fbe0 = 1;
        v44 = getenv("TAPE");
        *((char **)g_49fbd8) = v44;
        if (!*((long long *)g_49fbd8))
            *((char **)g_49fbd8) = "-";
    }
    if (g_49fbe0 > 1 && g_49f8fc != 1)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Multiple archive files require '-M' option"));
        sub_43814c(2); /* do not return */
    }
    if (g_49f8e0 && g_49f908 >= 0)
    {
        v10 = sub_435327(2);
        v11 = sub_435327(3);
        if (sub_435388(v10, v11))
            sub_4351b6("--listed-incremental", "--newer"); /* do not return */
        if (*(v10) == 1)
            g_49f8e0 = 0;
        else
            memset(&g_49f900, 0, 16);
    }
    if (g_49f8e8 != 0xffffffff && !g_49f8e0)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("--level is meaningless without --listed-incremental"));
    }
    if (*((long long *)&g_49f9c0) && (g_49f864 == 6 || g_49f864 == 2))
    {
        v12 = (!g_49f8fc ? 99 : 81);
        if (v12 < strlen(*((long long *)&g_49f9c0)))
        {
            if (g_49fdc8)
                g_49fdc8();
            sub_45e548(*((long long *)&g_49f9c0));
            error(0, 0, ngettext("%s: Volume label is too long (limit is %lu byte)", "%s: Volume label is too long (limit is %lu bytes)", v12));
            sub_43814c(2); /* do not return */
        }
    }
    if (g_49f9b0)
    {
        if (g_49f8fc)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Cannot verify multi-volume archives"));
            sub_43814c(2); /* do not return */
        }
        else if (g_49f898)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Cannot verify compressed archives"));
            sub_43814c(2); /* do not return */
        }
        else if (!((char)g_49cb60[g_49f860] & 2))
        {
            if (sub_43534c(4))
                sub_4351b6("--verify", sub_434e1c(g_49f860)); /* do not return */
            g_49f9b0 = 0;
        }
    }
    if (g_49f898)
    {
        if (g_49f8fc)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Cannot use multi-volume compressed archives"));
            sub_43814c(2); /* do not return */
        }
        else if ((char)g_49cb60[g_49f860] & 4)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Cannot update compressed archives"));
            sub_43814c(2); /* do not return */
        }
        else if (g_49f860 == 2)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Cannot concatenate compressed archives"));
            sub_43814c(2); /* do not return */
        }
    }
    if (g_49f910 == 2 && g_49f928 < 0)
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("--clamp-mtime needs a date specified using --mtime"));
        sub_43814c(2); /* do not return */
    }
    if (v25 && g_49f864 != 4 && !((char)g_49cb60[g_49f860] & 1))
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("--pax-option can be used only on POSIX archives"));
        sub_43814c(2); /* do not return */
    }
    if (g_49f968 > 0 && g_49f864 != 4 && !((char)g_49cb60[g_49f860] & 1))
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("--acls can be used only on POSIX archives"));
        sub_43814c(2); /* do not return */
    }
    if (g_49f964 > 0 && g_49f864 != 4 && !((char)g_49cb60[g_49f860] & 1))
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("--selinux can be used only on POSIX archives"));
        sub_43814c(2); /* do not return */
    }
    if (g_49f96c > 0 && g_49f864 != 4 && !((char)g_49cb60[g_49f860] & 1))
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("--xattrs can be used only on POSIX archives"));
        sub_43814c(2); /* do not return */
    }
    if (g_49f988 && !((char)g_49cb60[g_49f860] & 1))
    {
        if (sub_43534c(5))
            sub_4351b6("--starting-file", sub_434e1c(g_49f860)); /* do not return */
        g_49f988 = 0;
    }
    if (g_49f958 && !((char)g_49cb60[g_49f860] & 1))
    {
        if (sub_43534c(6))
            sub_4351b6("--same-order", sub_434e1c(g_49f860)); /* do not return */
        g_49f958 = 0;
    }
    if (g_49f936)
    {
        if (g_49f878)
        {
            v13 = sub_435327(7);
            v14 = sub_435327(8);
            if (sub_435388(v13, v14))
                sub_4351b6("--one-top-level", "--absolute-names"); /* do not return */
            if (*(v13) == 1)
                g_49f878 = 0;
            else
                g_49f936 = 0;
        }
        if (g_49f936 && !g_49f938)
        {
            v15 = sub_44e13e(*((long long *)g_49fbd8));
            g_49f938 = sub_4329b0(v15);
            free(v15);
            if (!g_49f938)
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR"));
                sub_43814c(2); /* do not return */
            }
        }
    }
    if (g_49f94c)
        g_49f8d8 = 3;
    v4 = (g_49f880 == 2 ? 0x40000 : 0) | (!g_49f8a0 ? 0xa0900 : 0x80900);
    g_49fc08 = v4;
    g_49fc0c = v4 | 0x10000;
    g_49fc10 = (!g_49f8a0 ? 0x100 : 0);
    if (g_49f860 == 9)
    {
        if ((char)sub_434a5e() != 1)
            g_49f9ac = g_49f9ac + 1;
    }
    else
    {
        if (g_49f879)
            g_49f9ac = 2;
    }
    if ((char)((CmpF(g_49f990, 0) & 69) >> 2) & 1 || ((char)((CmpF(g_49f990, 0) & 69) >> 6) & 1) != 1)
    {
        if (*((long long *)&g_49f870) >= 0)
        {
            v62 = *((long long *)&g_49f870);
        }
        else
        {
            v63 = *((long long *)&g_49f870) >> 1;
            v64 = (int)*((long long *)&g_49f870) & 1;
            v62 = AddV(v63 | v64, v63 | v64);
        }
        if (((CmpF(v62, g_49f990) & 69 | (char)((CmpF(v62, g_49f990) & 69) >> 6)) & 1) != 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Volume length cannot be less than record size"));
            sub_43814c(2); /* do not return */
        }
    }
    if (g_49f958 && g_49f8e0)
    {
        v16 = sub_435327(6);
        v17 = sub_435327(2);
        if (sub_435388(v16, v17))
            sub_4351b6("--preserve-order", "--listed-incremental"); /* do not return */
        if (*(v16) == 1)
            g_49f8e0 = 0;
        else
            g_49f958 = 0;
    }
    switch (g_49f860)
    {
    case 1: case 2: case 8:
        for (g_49fbe8 = g_49fbd8; g_49fbd8 + g_49fbe0 * 8 > g_49fbe8; g_49fbe8 = g_49fbe8 + 1)
        {
            if (!strcmp(g_49fbe8->field_0, "-"))
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Options '-Aru' are incompatible with '-f -'"));
                sub_43814c(2); /* do not return */
            }
        }
    case 3:
        if ((char)sub_434a5e() != 1)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Cowardly refusing to create an empty archive"));
            sub_43814c(2); /* do not return */
        }
        if (v26 && g_49fbe0 && strcmp(*((long long *)g_49fbd8), "-"))
            sub_432970(*((long long *)g_49fbd8), g_49f898);
        break;
    case 5: case 6: case 7: case 9:
        for (g_49fbe8 = g_49fbd8; g_49fbd8 + g_49fbe0 * 8 > g_49fbe8; g_49fbe8 = g_49fbe8 + 1)
        {
            if (!strcmp(g_49fbe8->field_0, "-"))
                sub_434a6f("-f");
        }
    default:
        break;
    }
    if (*((long long *)&g_49fbf0))
    {
        g_49efd0 = fopen(*((long long *)&g_49fbf0), "w");
        if (!g_49efd0)
            sub_43e3b4(*((long long *)&g_49fbf0)); /* do not return */
    }
    else
    {
        g_49efd0 = (!g_49f998 ? stdout : stderr);
    }
    g_49fbe8 = g_49fbd8;
    if (v27)
        g_49fe30 = sub_460cc6(v27);
    if (g_49f884)
    {
        g_49f888 = sub_44dec2("--backup", v28);
        if (!g_49f888 || g_49f998 || g_49f9a0)
            g_49f884 = 0;
    }
    sub_40fb72();
    sub_43589e(&v22);
    return v32 - *((long long *)(40 + v68));
}



// Function: main @ 0x39948
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    struct struct_1 *field_8;
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct FILE {
} FILE;

extern char g_4869f3;
extern unsigned int g_49ee88;
extern char stdout;
extern FILE *stderr;
extern unsigned long long g_49efd0;
extern unsigned int g_49f860;
extern char g_49f999;
extern char g_49f9b8;
extern void* g_49fbd8;
extern unsigned long long g_49fbe0;
extern unsigned int g_49fc34;
extern unsigned long long g_49fc38;
extern struct_0 *g_49fdc8;
extern unsigned int g_49fdd0;

unsigned int main(unsigned int a0, struct_0 *a1)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9

    sub_40adc5();
    sub_45c844(*((long long *)&a1->field_0));
    setlocale(6, &g_4869f3);
    bindtextdomain("tar", "/usr/local/share/locale");
    textdomain("tar");
    g_49ee88 = 2;
    g_49fdd0 = 0;
    g_49fdc8 = sub_410622;
    sub_45c9a2(0, 7);
    sub_44dfe0(gettext("stdout"));
    if (sub_45fe52(v2, v3, v4, v5, v6, v7))
    {
        if (g_49fdc8)
            g_49fdc8();
        gettext("failed to assert availability of the standard file descriptors");
        error(0, 0, "%s");
        sub_4177f4(); /* do not return */
    }
    g_49fc38 = 10;
    g_49fbd8 = sub_460b5b(g_49fc38 * 8);
    g_49fbe0 = 0;
    signal(0x11, NULL);
    sub_45c826(v2, v3, v4, v5, v6, v7);
    sub_43854d(a0, a1);
    sub_42c94c();
    if (*((long long *)&g_49f9b8))
        sub_40d1c6();
    switch (g_49f860)
    {
    case 0:
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("You must specify one of the '-Acdtrux', '--delete' or '--test-label' options"));
        sub_43814c(2); /* do not return */
    case 1: case 2: case 8:
        sub_43c325();
        goto LABEL_439b84;
    case 3:
        sub_414e23();
        goto LABEL_439b84;
    case 4:
        sub_417069();
        goto LABEL_439b84;
    case 5:
        sub_41075e();
        sub_4257b8(sub_411897);
        goto LABEL_439b84;
    case 6:
        sub_4182fd();
        sub_4257b8(sub_41bbcb);
        sub_41c2ee();
        goto LABEL_439b84;
    case 7:
        sub_4257b8(sub_425cf3);
        goto LABEL_439b84;
    case 9:
        sub_428913();
    default:
LABEL_439b84:
        sub_410729();
        if (g_49f999)
            sub_40baa2();
        if (g_49fc34)
            sub_41573f();
        if (*((long long *)&g_49f9b8))
            sub_40d2e3();
        free(g_49fbd8);
        sub_43d17c(v2, v3, v4, v5, v6, v7);
        sub_42c978();
        if (g_49fdd0 == 2)
            error(0, 0, gettext("Exiting with failure status due to previous errors"));
        if (g_49efd0 == *((long long *)&stdout))
        {
            sub_44e014();
            break;
        }
        else if (ferror_unlocked(stderr) || fclose(stderr))
        {
            sub_439e98(2);
        }
        return g_49fdd0;
    }
}



// Function: tar_stat_init @ 0x39c55
void* tar_stat_init(void* a0)
{
    return memset(a0, 0, 440);
}



// Function: tar_stat_close @ 0x39c7e
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
    unsigned int v0;  // [bp-0xc]

    v0 = (!idx->field_1a0 ? (idx->field_1a8 <= 0 ? 0 : (int)close(idx->field_1a8)) : (int)closedir(idx->field_1a0));
    idx->field_1a0 = NULL;
    idx->field_1a8 = 0;
    if (v0)
    {
        sub_42b26a(idx->field_0);
        return 0;
    }
    return 1;
}



// Function: tar_stat_destroy @ 0x39d1c
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    char padding_10[8];
    void* field_18;
    void* field_20;
    void* field_28;
    void* field_30;
    void* field_38;
    char padding_40[8];
    void* field_48;
    char padding_50[240];
    void* field_140;
    char padding_148[16];
    unsigned long long field_158;
    void* field_160;
    char padding_168[40];
    void* field_190;
} struct_0;

void* tar_stat_destroy(struct_0 *a0)
{
    sub_439c7e(a0);
    sub_41d55d(a0->field_160, a0->field_158);
    free(a0->field_0);
    free(a0->field_8);
    free(a0->field_18);
    free(a0->field_20);
    free(a0->field_28);
    free(a0->field_30);
    free(a0->field_38);
    free(a0->field_48);
    free(a0->field_140);
    free(a0->field_190);
    sub_41e785(a0->padding_168);
    sub_417bda(a0);
    return memset(a0, 0, 440);
}



// Function: tar_timespec_cmp @ 0x39e35
extern unsigned int g_49f468;

long long tar_timespec_cmp(long long a0, long long a1, long long a2, long long a3)
{
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x10]

    v1 = a1;
    v0 = a3;
    if (g_49f468 != 4)
    {
        v0 = 0;
        v1 = v0;
    }
    return sub_45ff90(a0, v1, a2, v0);
}



// Function: set_exit_status @ 0x39e98
extern unsigned int g_49fdd0;

long long set_exit_status(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_49fdd0;
    if (a0 > (unsigned int)v1)
    {
        v1 = a0;
        g_49fdd0 = v1;
    }
    return v1;
}



// Function: new_transform @ 0x39eba
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned long long g_49fcc0;
extern struct_0 *g_49fcc8;

long long new_transform(void)
{
    unsigned long v0;  // [bp-0x10]

    v0 = sub_460c06(112);
    if (g_49fcc8)
        g_49fcc8->field_0 = v0;
    else
        g_49fcc0 = v0;
    g_49fcc8 = v0;
    return v0;
}



// Function: add_segment @ 0x39f0c
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

struct_0 ** add_segment(struct_2 *idx)
{
    struct_0 **v0;  // [bp-0x10]

    v0 = sub_460b5b(32);
    *(v0) = NULL;
    if (idx->field_60)
        idx->field_60->field_0 = v0;
    else
        idx->field_58 = v0;
    idx->field_60 = v0;
    idx->field_68 = idx->field_68 + 1;
    return v0;
}



// Function: add_literal_segment @ 0x39f85
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

struct_0 * add_literal_segment(struct_2 *a0, void* a1, unsigned long a2)
{
    unsigned long n;  // [bp-0x18]
    struct_0 *idx;  // [bp-0x10]

    n = a2 - a1;
    if (!n)
        return a2 - a1;
    idx = sub_439f0c(a0);
    idx->field_8 = 0;
    idx->field_10 = sub_460b5b(n + 1);
    memcpy(idx->field_10, a1, n);
    *(n + (char *)idx->field_10) = 0;
    *((unsigned long *)&idx->field_18) = n;
    return idx;
}



// Function: add_char_segment @ 0x3a022
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

struct_0 * add_char_segment(struct_2 *a0, char a1)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_439f0c(a0);
    idx->field_8 = 0;
    idx->field_10 = sub_460b5b(2);
    idx->field_10->field_0 = a1;
    idx->field_10->field_1 = 0;
    idx->field_18 = 1;
    return idx;
}



// Function: add_backref_segment @ 0x3a090
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

struct_0 * add_backref_segment(struct_2 *a0, unsigned long a1)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_439f0c(a0);
    idx->field_8 = 1;
    idx->field_10 = a1;
    return idx;
}



// Function: parse_xform_flags @ 0x3a0ce

unsigned int parse_xform_flags(unsigned int *a0, unsigned int a1)
{
    switch (a1)
    {
    case 72:
        *(a0) = *(a0) & 0xfffffffd;
        break;
    case 82:
        *(a0) = *(a0) & 0xfffffffe;
        break;
    case 83:
        *(a0) = *(a0) & 0xfffffffb;
        break;
    case 104:
        *(a0) = *(a0) | 2;
        break;
    case 114:
        *(a0) = *(a0) | 1;
        break;
    case 115:
        *(a0) = *(a0) | 4;
        break;
    default:
        return 1;
    }
    return 0;
}



// Function: add_case_ctl_segment @ 0x3a18f
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

typedef struct struct_2 {
    char padding_0[88];
    struct struct_0 *field_58;
    struct struct_0 *field_60;
    unsigned long long field_68;
} struct_2;

unsigned int * add_case_ctl_segment(struct_2 *a0, unsigned int a1)
{
    unsigned int *idx;  // [bp-0x10]

    idx = sub_439f0c(a0);
    idx[2] = 2;
    idx[4] = a1;
    return idx;
}



// Function: parse_transform_expr @ 0x3a1ca
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49ec00;
extern struct_0 *g_49fdc8;

char * parse_transform_expr(char *a0)
{
    unsigned int v21;  // eax
    char *node;  // [bp-0x280]
    int iter;  // [bp-0x26c]
    int iter2;  // [bp-0x268]
    unsigned int v3;  // [bp-0x264]
    unsigned int v4;  // [bp-0x260]
    unsigned int v5;  // [bp-0x25c]
    char *iter1;  // [bp-0x258]
    char *l;  // [bp-0x250]
    char *v8;  // [bp-0x248]
    void* idx;  // [bp-0x240]
    char *ptr;  // [bp-0x238]
    unsigned long num;  // [bp-0x230]
    char v12;  // [bp-0x228]
    char v13;  // [bp-0x227]
    char v14;  // [bp-0x226]

    node = a0;
    v3 = 0;
    idx = sub_439eba();
    if (*(node) == 115)
    {
        v4 = node[1];
        if (!v4)
        {
            if (g_49fdc8)
                g_49fdc8();
            error(0, 0, gettext("Invalid transform expression"));
            sub_43814c(2); /* do not return */
        }
        else
        {
            for (iter = 2; node[iter] && v4 != node[iter]; iter += 1)
            {
                if (node[iter] == 92 && node[1 + iter])
                    iter += 1;
            }
            if (v4 != node[iter])
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Invalid transform expression"));
                sub_43814c(2); /* do not return */
            }
            else
            {
                for (iter2 = iter + 1; node[iter2] && v4 != node[iter2]; iter2 += 1)
                {
                    if (node[iter2] == 92 && node[1 + iter2])
                        iter2 += 1;
                }
                if (v4 != node[iter2])
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Invalid transform expression"));
                    sub_43814c(2); /* do not return */
                }
                else
                {
                    *((unsigned int *)&idx[8]) = 0;
                    *((unsigned int *)&idx[12]) = g_49ec00;
                    for (l = &node[1 + iter2]; *(l) && *(l) != 59; l += 1)
                    {
                        v21 = *(l);
                        switch (v21)
                        {
                        case 120:
                            v3 |= 1;
                            break;
                        case 105:
                            v3 |= 2;
                            break;
                        case 103:
                            *((unsigned int *)&idx[8]) = 1;
                            break;
                        default:
                            if (sub_43a0ce(idx + 12, *(l)))
                            {
                                if (g_49fdc8)
                                    g_49fdc8();
                                error(0, 0, gettext("Unknown flag in transform expression: %c"));
                                sub_43814c(2); /* do not return */
                            }
                        }
                    }
                    if (*(l) == 59)
                        l += 1;
                    ptr = sub_460b5b(iter - 1);
                    memcpy(ptr, node + 2, iter - 2);
                    *(&ptr[iter] - 2) = 0;
                    v5 = sub_467ccd(idx + 24, ptr, v3, idx + 24);
                    if (v5)
                    {
                        sub_467e43(v5, idx + 24, &v12, 0x200);
                        if (g_49fdc8)
                            g_49fdc8();
                        error(0, 0, gettext("Invalid transform expression: %s"));
                        sub_43814c(2); /* do not return */
                    }
                    else
                    {
                        if (*(ptr) == 94 || iter > 2 && *(&ptr[iter] - 3) == 36)
                            *((unsigned int *)&idx[8]) = 0;
                    }
                    free(ptr);
                    iter += 1;
                    ptr = sub_460b5b(iter2 - iter + 1);
                    memcpy(ptr, &node[iter], iter2 - iter);
                    ptr[iter2 + -1 * iter] = 0;
                    v8 = ptr;
                    iter1 = v8;
                    while (1)
                    {
                        while (1)
                        {
                            if (!*(iter1))
                            {
                                sub_439f85(idx, v8, iter1);
                                free(ptr);
                                return l;
                            }
                            if (*(iter1) == 92)
                                break;
                            if (*(iter1) == 38)
                            {
                                sub_439f85(idx, v8, iter1);
                                sub_43a090(idx, 0);
                                iter1 += 1;
                                v8 = iter1;
                            }
                            else
                            {
                                iter1 += 1;
                            }
                        }
                        sub_439f85(idx, v8, iter1);
                        iter1 += 1;
                        switch (*(iter1))
                        {
                        case 38:
                            sub_43a022(idx, 38);
                            iter1 += 1;
                            break;
                        case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
                            num = strtoul(iter1, &iter1, 10, &iter1);
                            if (num > (long long)idx[72])
                                goto LABEL_43a9ed;
                            sub_43a090(idx, num);
                            break;
                        case 69:
                            sub_43a18f(idx, 0);
                            iter1 += 1;
                            break;
                        case 76:
                            sub_43a18f(idx, 4);
                            iter1 += 1;
                            break;
                        case 85:
                            sub_43a18f(idx, 3);
                            iter1 += 1;
                            break;
                        case 92:
                            sub_43a022(idx, 92);
                            iter1 += 1;
                            break;
                        case 97:
                            sub_43a022(idx, 7);
                            iter1 += 1;
                            break;
                        case 98:
                            sub_43a022(idx, 8);
                            iter1 += 1;
                            break;
                        case 102:
                            sub_43a022(idx, 12);
                            iter1 += 1;
                            break;
                        case 108:
                            sub_43a18f(idx, 2);
                            iter1 += 1;
                            break;
                        case 110:
                            sub_43a022(idx, 10);
                            iter1 += 1;
                            break;
                        case 114:
                            sub_43a022(idx, 13);
                            iter1 += 1;
                            break;
                        case 116:
                            sub_43a022(idx, 9);
                            iter1 += 1;
                            break;
                        case 117:
                            sub_43a18f(idx, 1);
                            iter1 += 1;
                            break;
                        case 118:
                            sub_43a022(idx, 11);
                            iter1 += 1;
                            break;
                        default:
                            if (v4 == *(iter1))
                            {
                                sub_43a022(idx, v4);
                            }
                            else
                            {
                                v12 = 92;
                                v13 = *(iter1);
                                sub_439f85(idx, &v12, &v14);
                            }
                            iter1 += 1;
                            break;
                        }
                        v8 = iter1;
                    }
LABEL_43a9ed:
                    if (g_49fdc8)
                        g_49fdc8();
                    error(0, 0, gettext("Invalid transform replacement: back reference out of range"));
                    sub_43814c(2); /* do not return */
                }
            }
        }
    }
    else if (!strncmp(node, "flags=", 6))
    {
        g_49ec00 = 0;
        for (node += 6; *(node); node += 1)
        {
            if (*(node) == 59)
            {
                node += 1;
                break;
            }
            else if (sub_43a0ce(&g_49ec00, *(node)))
            {
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Unknown transform flag: %c"));
                sub_43814c(2); /* do not return */
            }
        }
        return node;
    }
    else
    {
        if (g_49fdc8)
            g_49fdc8();
        error(0, 0, gettext("Invalid transform expression"));
        sub_43814c(2); /* do not return */
    }
}



// Function: set_transform_expr @ 0x3ae0b
long long set_transform_expr(char *a0)
{
    unsigned long v2;  // rax
    char *v0;  // [bp-0x10]

    v0 = a0;
    while (1)
    {
        if (!*(v0))
            break;
        v0 = sub_43a1ca(v0);
    }
    return v2;
}



// Function: run_case_conv @ 0x3ae3c
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 *g_49fcd0;
extern unsigned long long g_49fd40;

char * run_case_conv(unsigned int a0, void* a1, unsigned long n)
{
    char v2;  // al
    char v3;  // al
    char *iter;  // [bp-0x10]

    if (n > g_49fd40)
    {
        g_49fd40 = n;
        g_49fcd0 = sub_460b8d(g_49fcd0, g_49fd40, g_49fd40);
    }
    memcpy(g_49fcd0, a1, n);
    switch (a0)
    {
    case 0:
        break;
    case 1:
        v2 = toupper((char)g_49fcd0->field_0);
        *((char *)&g_49fcd0->field_0) = v2;
        break;
    case 2:
        v3 = tolower((char)g_49fcd0->field_0);
        *((char *)&g_49fcd0->field_0) = v3;
        break;
    case 3:
        for (iter = g_49fcd0; iter < n + (char *)g_49fcd0; iter += 1)
        {
            *(iter) = toupper(*(iter)) & 0xffffffff;
        }
        break;
    case 4:
        for (iter = g_49fcd0; iter < n + (char *)g_49fcd0; iter += 1)
        {
            *(iter) = tolower(*(iter)) & 0xffffffff;
        }
        break;
    }
    return g_49fcd0;
}



// Function: _single_transform_name_to_obstack @ 0x3afa7
typedef struct struct_1 {
    char padding_0[24];
    void* field_18;
    unsigned long long field_20;
} struct_1;

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

extern struct_1 g_49fce0;

void _single_transform_name_to_obstack(struct_0 *a0, char *a1)
{
    unsigned int v31;  // eax
    unsigned int v32;  // eax
    char *v33;  // rax
    char *iter;  // [bp-0xf8]
    unsigned int v1;  // [bp-0xe4]
    unsigned int v2;  // [bp-0xe0]
    unsigned int v3;  // [bp-0xdc]
    unsigned long long v4;  // [bp-0xd8]
    unsigned long len1;  // [bp-0xd0]
    char *ptr;  // [bp-0xc8], Other Possible Types: unsigned long
    void* idx1;  // [bp-0xc0]
    void* ptr1;  // [bp-0xb8]
    struct_1 *v9;  // [bp-0xb0]
    unsigned long v10;  // [bp-0xa8]
    struct_1 *v11;  // [bp-0xa0]
    struct_1 *v12;  // [bp-0x98]
    unsigned long v13;  // [bp-0x90]
    struct_1 *v14;  // [bp-0x88]
    struct_1 *idx;  // [bp-0x80]
    unsigned long n;  // [bp-0x78]
    struct_1 *v17;  // [bp-0x70]
    unsigned long v18;  // [bp-0x68]
    struct_1 *idx2;  // [bp-0x60]
    unsigned long n1;  // [bp-0x58]
    struct_1 *v21;  // [bp-0x50]
    struct_1 *v22;  // [bp-0x48]
    unsigned long n2;  // [bp-0x40]
    struct_1 *v24;  // [bp-0x38]
    struct_1 *v25;  // [bp-0x30]
    unsigned long len;  // [bp-0x28]
    struct_1 *v27;  // [bp-0x20]
    struct_1 *index;  // [bp-0x18]
    struct_1 *v29;  // [bp-0x10]

    iter = a1;
    v4 = 0;
    v1 = 0;
    v2 = 0;
    ptr1 = sub_460b5b((a0->field_48 + 1) * 16);
    do
    {
        if (!*(iter))
            goto LABEL_43b6cc;
        v3 = sub_46f294(&a0->padding_14[4], iter, a0->field_48 + 1, ptr1, 0);
        if (!v3)
        {
            len1 = (long long)ptr1[8];
            if (*((long long *)ptr1))
            {
                v12 = &g_49fce0.field_0;
                v13 = *((long long *)ptr1);
                v14 = v12;
                if (v14->field_20 - v14->field_18 < v13)
                    _obstack_newchunk(v12, v13, v13);
                memcpy(v12->field_18, iter, v13);
                v12->field_18 = v12->field_18 + v13;
            }
            v4 += 1;
            if (a0->field_10 && v4 < a0->field_10)
            {
                idx = &g_49fce0.field_0;
                n = len1;
                v17 = idx;
                if (v17->field_20 - v17->field_18 < n)
                    _obstack_newchunk(idx, n, n);
                memcpy(idx->field_18, iter, n);
                idx->field_18 = idx->field_18 + n;
                iter = &iter[len1];
                continue;
            }
            else
            {
                for (idx1 = a0->field_58; idx1; idx1 = *((long long *)idx1))
                {
                    v31 = (int)idx1[8];
                    if (v31 != 2 && v31 <= 2)
                    {
                        if (!v31)
                        {
                            if (!v1)
                            {
                                ptr = (long long)idx1[16];
                            }
                            else
                            {
                                ptr = sub_43ae3c(v1, (long long)idx1[16], (long long)idx1[24]);
                                if (v1 == 1 || v1 == 2)
                                {
                                    v1 = v2;
                                    v2 = 0;
                                }
                            }
                            v22 = &g_49fce0.field_0;
                            n2 = (long long)idx1[24];
                            v24 = v22;
                            if (v24->field_20 - v24->field_18 < n2)
                                _obstack_newchunk(v22, n2, n2);
                            memcpy(v22->field_18, ptr, n2);
                            v22->field_18 = v22->field_18 + n2;
                            goto LABEL_43b528;
                        }
                        else if (v31 == 1)
                        {
                            if (*((long long *)((char *)ptr1 + 16 * idx1[16])) != 0xffffffffffffffff && *((long long *)((char *)ptr1 + 16 * idx1[16] + 8)) != 0xffffffffffffffff)
                            {
                                v18 = *((long long *)((char *)ptr1 + 16 * idx1[16] + 8)) - *((long long *)((char *)ptr1 + 16 * idx1[16]));
                                ptr = &iter[*((long long *)((char *)ptr1 + 16 * idx1[16]))];
                                if (v1)
                                {
                                    ptr = sub_43ae3c(v1, ptr, v18);
                                    if (v1 == 1 || v1 == 2)
                                    {
                                        v1 = v2;
                                        v2 = 0;
                                    }
                                }
                                idx2 = &g_49fce0.field_0;
                                n1 = v18;
                                v21 = idx2;
                                if (v21->field_20 - v21->field_18 < n1)
                                    _obstack_newchunk(idx2, n1, n1);
                                memcpy(idx2->field_18, ptr, n1);
                                idx2->field_18 = idx2->field_18 + n1;
                            }
                            goto LABEL_43b528;
                        }
                    }
                    v32 = (int)idx1[16];
                    if (v32 <= 4)
                    {
                        if (v32 < 3 && v32)
                        {
                            if (v32 - 1 > 1)
                                goto LABEL_43b528;
                            if (!v2 || v2 - 3 <= 1)
                                v2 = v1;
                        }
                        v1 = (int)idx1[16];
                    }
LABEL_43b528:
                }
            }
        }
        else
        {
            len1 = strlen(iter);
            v9 = &g_49fce0.field_0;
            v10 = len1;
            v11 = v9;
            if (v11->field_20 - v11->field_18 < v10)
                _obstack_newchunk(v9, v10, v10);
            memcpy(v9->field_18, iter, v10);
            v9->field_18 = v9->field_18 + v10;
        }
        iter = &iter[len1];
    } while (a0->field_8);
    v25 = &g_49fce0.field_0;
    len = strlen(iter);
    v27 = v25;
    if (v27->field_20 - v27->field_18 < len)
        _obstack_newchunk(v25, len, len);
    memcpy(v25->field_18, iter, len);
    v25->field_18 = v25->field_18 + len;
LABEL_43b6cc:
    index = &g_49fce0.field_0;
    v29 = index;
    if (!(v29->field_20 - v29->field_18))
        _obstack_newchunk(index, 1);
    v33 = index->field_18;
    index->field_18 = v33 + 1;
    *(v33) = 0;
    free(ptr1);
    return;
}



// Function: _transform_name_to_obstack @ 0x3b734
typedef struct struct_1 {
    struct struct_1 *field_0;
    char padding_8[4];
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    char *field_10;
    char padding_18[8];
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
} struct_0;

extern struct_1 *g_49fcc0;
extern struct_0 g_49fce0;
extern char g_49fd38;

int _transform_name_to_obstack(unsigned int a0, char *a1, char **a2)
{
    char *v0;  // [bp-0x38]
    char v1;  // [bp-0x21]
    struct_1 *i;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    v0 = a1;
    v1 = 0;
    if (g_49fd38 != 1)
    {
        _obstack_begin(&g_49fce0.field_0, 0, 0, sub_460b5b, free);
        g_49fd38 = 1;
    }
    for (i = g_49fcc0; i; i = i->field_0)
    {
        if (i->field_c & a0)
        {
            sub_43afa7(i, v0);
            idx = &g_49fce0.field_0;
            v4 = idx->field_10;
            if (v4 == idx->padding_18)
                idx->field_50 = idx->field_50 | 2;
            idx->padding_18 = ~(idx->field_30) & idx->padding_18 + idx->field_30;
            if (idx->padding_18 - idx->field_8 > idx->field_20 - idx->field_8)
                *((unsigned long long *)&idx->padding_18[0]) = idx->field_20;
            *((char [8])&idx->field_10) = idx->padding_18;
            v0 = v4;
            v1 = 1;
        }
    }
    *(a2) = v0;
    return v1;
}



// Function: transform_name_fp @ 0x3b8af
extern unsigned long long g_49fce0[5];

long long transform_name_fp(char **a0, unsigned int a1, unsigned long long *a2, unsigned long a3)
{
    unsigned long v5;  // rax
    char v0;  // [bp-0x29]
    char *v1;  // [bp-0x28]
    unsigned long long *idx;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]

    v0 = sub_43b734(a1, *(a0), &v1);
    if (v0)
    {
        if (a2)
            v5 = a2(v1, a3, a3, a2);
        else
            v5 = v1;
        sub_429547(a0, v5);
        idx = &g_49fce0[0];
        v3 = v1;
        if (v3 > idx[1] && v3 < idx[4])
        {
            idx[2] = v3;
            idx[3] = idx[2];
            return v0;
        }
        _obstack_free(idx, v3, v3);
        return v0;
    }
    else if (a2)
    {
        *(a0) = NULL;
        sub_429547(a0, a2(v1, a3, a3, a2));
        free(v1);
        v0 = 1;
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: transform_name @ 0x3b9f2
long long transform_name(char **a0, unsigned int a1)
{
    return sub_43b8af(a0, a1, NULL, 0);
}



// Function: transform_program_p @ 0x3ba20
extern unsigned long long g_49fcc0;

long long transform_program_p(void)
{
    return _INSERT(g_49fcc0, 0, g_49fcc0);
}



// Function: dunlink_alloc @ 0x3ba37
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_49fd60;

struct_0 ** dunlink_alloc(void)
{
    struct_0 **v0;  // [bp-0x10]

    if (!g_49fd60)
    {
        v0 = sub_460b5b(40);
        return v0;
    }
    v0 = g_49fd60;
    g_49fd60 = *(v0);
    *(v0) = NULL;
    return v0;
}



// Function: dunlink_insert @ 0x3ba89
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_49fd48;
extern unsigned long long g_49fd50;
extern unsigned long long g_49fd58;

unsigned long long dunlink_insert(struct_0 **a0, struct_0 **a1)
{
    unsigned long long v1;  // rax

    if (a0)
    {
        *(a1) = *(a0);
        *(a0) = a1;
    }
    else
    {
        *(a1) = g_49fd48;
        g_49fd48 = a1;
    }
    if (!*(a1))
        g_49fd50 = a1;
    v1 = g_49fd58;
    g_49fd58 = v1 + 1;
    return v1 + 1;
}



// Function: dunlink_reclaim @ 0x3bb00
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    void* field_10;
} struct_0;

extern unsigned long long g_49fd60;

struct_0 * dunlink_reclaim(struct_0 *a0)
{
    free(a0->field_10);
    a0->field_0 = g_49fd60;
    g_49fd60 = a0;
    return a0;
}



// Function: flush_deferred_unlinks @ 0x3bb3c
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

extern unsigned int g_49c268;
extern char g_49efc8;
extern unsigned int g_49f600;
extern struct_0 *g_49fd48;
extern unsigned long long g_49fd50;
extern unsigned long long g_49fd58;
extern unsigned long long g_49fd68;

long long flush_deferred_unlinks(char a0)
{
    int v8;  // eax
    unsigned int v0;  // [bp-0x3c]
    struct_0 *idx;  // [bp-0x38]
    struct_0 *v2;  // [bp-0x30]
    char *v3;  // [bp-0x28]
    struct_0 *v4;  // [bp-0x20]
    struct_0 *v5;  // [bp-0x18]
    char *v6;  // [bp-0x10]

    v2 = NULL;
    v0 = g_49f600;
    idx = g_49fd48;
    while (idx)
    {
        v5 = idx->field_0;
        if (a0 || idx->field_20 + g_49fd68 < *((long long *)&g_49efc8))
        {
            sub_42adf1(idx->field_8);
            if (idx->field_18)
            {
                if (idx->field_8 && idx->field_18 && (!idx->field_10->field_0 || !strcmp(idx->field_10, ".")))
                {
                    v2 = idx;
                    idx = v5;
                }
                else
                {
                    v6 = &idx->field_10->field_0;
                    if (!unlinkat(g_49c268, v6, 0x200, v6))
                        goto LABEL_43bcc2;
                    v8 = *(__errno_location());
                    switch (v8)
                    {
                    case 17: case 39:
                        v2 = idx;
                        idx = v5;
                        break;
                    case 2:
LABEL_43bcc2:
                        sub_43bb00(idx);
                        g_49fd58 = g_49fd58 - 1;
                        idx = v5;
                        if (v2)
                        {
                            v2->field_0 = idx;
                            break;
                        }
                        else
                        {
                            g_49fd48 = idx;
                            break;
                        }
                    default:
                        sub_43e705(v6);
                        goto LABEL_43bcc2;
                    }
                }
            }
            else
            {
                if (unlinkat(g_49c268, idx->field_10, 0, idx->field_10) && *(__errno_location()) != 2)
                {
                    sub_43eaa7(idx->field_10);
                    goto LABEL_43bcc2;
                }
            }
        }
        else
        {
            v2 = idx;
            idx = v5;
        }
    }
    if (!g_49fd48)
    {
        g_49fd50 = 0;
    }
    else if (a0)
    {
        for (idx = g_49fd48; idx; idx = v4)
        {
            v4 = idx->field_0;
            sub_42adf1(idx->field_8);
            if (idx->field_8 && idx->field_18 && (!idx->field_10->field_0 || !strcmp(idx->field_10, ".")))
            {
                v3 = sub_42afc0();
                sub_42adf1(idx->field_8 - 1);
            }
            else
            {
                v3 = &idx->field_10->field_0;
            }
            if (unlinkat(g_49c268, v3, 0x200, v3) && *(__errno_location()) != 2)
                sub_43e705(v3);
            sub_43bb00(idx);
            g_49fd58 = g_49fd58 - 1;
        }
        g_49fd50 = 0;
        g_49fd48 = g_49fd50;
    }
    return sub_42adf1(v0);
}



// Function: finish_deferred_unlinks @ 0x3be6b
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_49fd60;

unsigned long long finish_deferred_unlinks(void)
{
    struct_0 **v0;  // [bp-0x10]

    sub_43bb3c(1);
    for (; g_49fd60; g_49fd60 = v0)
    {
        v0 = g_49fd60->field_0;
        free(g_49fd60);
    }
    return 0;
}



// Function: queue_deferred_unlink @ 0x3bebb
typedef struct struct_0 {
    struct struct_0 *field_0;
    int field_8;
    char padding_c[4];
    struct struct_1 *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_0;

typedef struct struct_4 {
    unsigned long long field_0;
    unsigned int field_8;
    char padding_c[4];
    struct struct_1 *field_10;
    char field_18;
    char padding_19[7];
    unsigned long long field_20;
} struct_4;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_49efc8;
extern unsigned int g_49f600;
extern struct_0 *g_49fd48;
extern struct_0 **g_49fd50;
extern unsigned long long g_49fd68;

unsigned long long queue_deferred_unlink(long long a0, char a1)
{
    struct_0 *i;  // [bp-0x20]
    struct_0 *v1;  // [bp-0x18]
    struct_4 *idx;  // [bp-0x10]

    if (g_49fd48 && (char *)g_49fd48[2].field_0 + g_49fd68 < *((long long *)&g_49efc8))
        sub_43bb3c(0);
    idx = sub_43ba37();
    idx->field_0 = 0;
    idx->field_8 = g_49f600;
    idx->field_10 = sub_460cc6(a0);
    sub_429946(idx->field_10);
    idx->field_18 = a1;
    idx->field_20 = *((long long *)&g_49efc8);
    if (idx->field_18 && (!idx->field_10->field_0 || !strcmp(idx->field_10, ".")))
    {
        i = g_49fd48;
        for (v1 = NULL; i && (!i->field_18 || i->field_10->field_0 && strcmp(i->field_10, ".") || i->field_8 >= idx->field_8); i = i->field_0)
        {
            v1 = i;
        }
        return (!i ? sub_43ba89(g_49fd50, idx) : sub_43ba89(v1, idx));
    }
    return sub_43ba89(g_49fd50, idx);
}



// Function: append_file @ 0x3c078
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
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern struct_0 *g_49fdc8;

long long append_file(long long a0)
{
    unsigned long v11;  // fs
    unsigned int v0;  // [bp-0xec]
    unsigned long i;  // [bp-0xe8]
    unsigned long v2;  // [bp-0xe0]
    unsigned long v3;  // [bp-0xd8]
    unsigned long long v4;  // [bp-0xd0]
    stat v5;  // [bp-0xc8]
    char v6;  // [bp-0x38]
    unsigned long v7;  // [bp-0x20]

    v0 = openat(g_49c268, a0, 0, a0);
    if (v0 < 0)
    {
        sub_43e38b(a0);
    }
    else
    {
        if (fstat(v0, &v5))
        {
            sub_43ea03(a0);
        }
        else
        {
            i = v5.st_atime;
            while (i > 0)
            {
                v3 = sub_40bb4b();
                v2 = sub_40bbf5(v3);
                if (v2 > i)
                {
                    v2 = i;
                    v4 = (unsigned int)v2 & 0x1ff;
                    if (v4)
                        memset(v3 + i, 0, 0x200 - v4);
                }
                v4 = sub_45ec70(v0, v3, v2, v3);
                if (v4 == 0xffffffffffffffff)
                    sub_43e5fc(a0, v5.st_atime - i, v2, v5.st_atime - i); /* do not return */
                if (!v4)
                {
                    if (g_49fdc8)
                        g_49fdc8();
                    sub_454e5a(i, &v6, &v6);
                    sub_45e548(a0);
                    error(0, 0, ngettext("%s: File shrank by %s byte", "%s: File shrank by %s bytes", i));
                    sub_4177f4(); /* do not return */
                }
                i -= v4;
                sub_40bba7(v3 + (v4 - 1 & 0xfffffffffffffe00));
            }
        }
        if (close(v0))
            sub_43e1ff(a0);
    }
    return v7 - *((long long *)(40 + v11));
}



// Function: update_archive @ 0x3c325
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[337];
    char field_159;
} struct_0;

typedef struct struct_1 {
    char padding_0[112];
    unsigned int field_70;
    unsigned int field_74;
    unsigned int field_78;
    char padding_7c[108];
    unsigned long long field_e8;
    unsigned long long field_f0;
    unsigned long long field_f8;
    unsigned long long field_100;
    unsigned long long field_108;
    unsigned long long field_110;
    char padding_118[88];
    unsigned long long field_170;
} struct_1;

extern unsigned long long g_49efb0;
extern struct_0 *g_49f460;
extern unsigned int g_49f468;
extern char g_49f860;
extern unsigned int g_49f864;
extern char g_49f8c8;
extern char g_49fa20;
extern void g_49fa28;
extern long long g_49fb18;
extern long long g_49fb20;
extern char g_49fd70;
extern unsigned long long g_49fd78;
extern struct_1 *g_49fdc8;
extern unsigned int g_49fdd0;

void update_archive(void)
{
    long long v12;  // rax
    long long v13;  // rax
    long long v14;  // rdx
    char i;  // [bp-0xf1]
    unsigned int v1;  // [bp-0xf0]
    unsigned int v2;  // [bp-0xec]
    char *v3;  // [bp-0xe8]
    struct_0 *v4;  // [bp-0xe0]
    unsigned long v5;  // [bp-0xd8]
    void* v6;  // [bp-0xd0]
    char *v7;  // [bp-0xc8]
    struct_0 *v8;  // [bp-0xc0]
    char v9;  // [bp-0xb8]
    unsigned int v10;  // [bp-0xa0]

    v1 = 0;
    i = 0;
    sub_42d3b1();
    sub_40f694(2);
    sub_41d498();
    while (i != 1)
    {
        v2 = sub_425ecf(&g_49f460, &g_49fa20, 0);
        switch (v2)
        {
        case 0: case 2:
            abort(); /* do not return */
        case 1:
            sub_426747(g_49f460, &g_49fa20, &g_49f468, 0);
            sub_425733(*((char *)&g_49f460[9].field_8 + 4), &g_49fa20);
            g_49f864 = g_49f468;
            if (*((int *)&g_49f860) == 8)
            {
                v6 = sub_42ed0e(*((long long *)&g_49fa28));
                if (v6)
                {
                    sub_42adf1((int)v6[40]);
                    if (!sub_42a9b8(*((long long *)&g_49fa28), &v9))
                    {
                        if (((unsigned short)v10 & 0xf000) == 0x4000)
                        {
                            v7 = sub_42b8e1((long long)v6[16], 1);
                            if (v7)
                            {
                                v8 = sub_42b66c((long long)v6[16]);
                                for (v3 = v7; *(v3); v3 = &v3[1 + strlen(v3)])
                                {
                                    v12 = sub_42b75b(v8, v3);
                                    sub_42d5b5(v12, (int)v6[40], 0, 0);
                                }
                                sub_42b72d(v8);
                                free(v7);
                                sub_42d817(v6);
                                break;
                            }
                        }
                        else
                        {
                            v13 = sub_45fd9e(&v9);
                            if ((int)sub_439e35(v13, v14, g_49fb18, g_49fb20) <= 0)
                                sub_42d817(v6);
                        }
                    }
                }
            }
            sub_428891();
            goto LABEL_43c6c1;
        case 3:
            g_49efb0 = g_49f460;
            i = 1;
            goto LABEL_43c6c1;
        case 4:
            i = 1;
            goto LABEL_43c6c1;
        case 5:
            sub_40bba7(g_49f460);
            switch (v1)
            {
            case 0:
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("This does not look like a tar archive"));
            case 2: case 4:
                abort(); /* do not return */
            case 1: case 3:
                if (g_49fdc8)
                    g_49fdc8();
                error(0, 0, gettext("Skipping to next header"));
                g_49fdd0 = 2;
            case 5:
                goto LABEL_43c6c0;
            default:
LABEL_43c6c0:
                break;
            }
        default:
LABEL_43c6c1:
            sub_439d1c(&g_49fa20);
            v1 = v2;
            continue;
        }
    }
    sub_40baf9();
    g_49fd70 = 1;
    g_49fd78 = g_49efb0;
    while (1)
    {
        do
        {
            do
            {
                v4 = sub_42eda0();
                if (!v4)
                {
                    sub_412da0();
                    sub_40d046();
                    sub_43be6b();
                    sub_42dbba();
                    return;
                }
            } while ((v5 = (unsigned long)*((long long *)&v4->padding_8[8]), (char)sub_417c43(v5, NULL)));
        } while (g_49f8c8 && !(int)sub_434af4("add", v5));
        if (*((int *)&g_49f860) == 2)
            sub_43c078(v5);
        else
            sub_416a25(0, v5, v5);
    }
}



// Function: utf8_init @ 0x3c7e3
extern unsigned long long g_49ec10[4];

unsigned long long * utf8_init(char a0)
{
    long long v1;  // rax

    if (g_49ec10[a0] == 0xffffffffffffffff)
    {
        if (a0)
        {
            g_49ec10[a0] = iconv_open("UTF-8", sub_454ee3());
        }
        else
        {
            v1 = sub_454ee3();
            g_49ec10[a0] = iconv_open(v1, "UTF-8", v1);
        }
    }
    return g_49ec10[a0];
}



// Function: utf8_convert @ 0x3c8a0
typedef struct struct_0 {
    char field_0;
} struct_0;

unsigned int utf8_convert(char a0, unsigned long a1, struct_0 **a2)
{
    unsigned long v0;  // [bp-0x40]
    char *v1;  // [bp-0x38]
    unsigned long v2;  // [bp-0x30]
    unsigned long v3;  // [bp-0x28]
    unsigned long v4;  // [bp-0x20]
    char *v5;  // [bp-0x18]

    v4 = sub_43c7e3(a0);
    if (!v4)
    {
        *(a2) = sub_460cc6(a1);
        return 1;
    }
    else if (v4 == 0xffffffffffffffff)
    {
        return 0;
    }
    else
    {
        v2 = strlen(a1) + 1;
        v3 = v2 * 16 + 1;
        v5 = sub_460b5b(v3);
        v1 = v5;
        v0 = a1;
        if (!iconv(v4, &v0, &v2, &v1, &v3))
        {
            *(v1) = 0;
            *(a2) = v5;
            return 1;
        }
        free(v5);
        return 0;
    }
}



// Function: string_ascii_p @ 0x3c9b7
unsigned int string_ascii_p(char *a0)
{
    char *i;  // [bp-0x10]

    for (i = a0; *(i); i += 1)
    {
        if (*(i) < 0)
            return 0;
    }
    return 1;
}



// Function: set_warning_option @ 0x3c9ee
extern long long g_49aa60;
extern unsigned int g_49ec20[4];
extern unsigned int g_49ec84;
extern long long g_49ee48;

int set_warning_option(unsigned long a0)
{
    unsigned int v4;  // eax
    unsigned long v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x10]
    unsigned int *v2;  // [bp-0xc]

    v0 = a0;
    v1 = 0;
    v4 = strcmp(v0, "none");
    if (!v4)
    {
        g_49ec84 = 0;
        return v4;
    }
    if (strlen(v0) > 2 && !memcmp(v0, "no-", 3))
    {
        v1 = 1;
        v0 += 3;
    }
    v2 = (unsigned int *)g_49ec20[sub_447c61("--warning", v0, &g_49aa60, &g_49ec20[0], 4, g_49ee48)];
    if (!v1)
    {
        g_49ec84 = g_49ec84 | v2;
        return g_49ec84;
    }
    g_49ec84 = g_49ec84 & ~(v2);
    return g_49ec84;
}



// Function: acls_one_line @ 0x3cae5
typedef struct FILE {
} FILE;

extern FILE *g_49efd0;

void acls_one_line(void* a0, char a1, void* a2, unsigned long a3)
{
    char *v27;  // rax
    char *v28;  // rax
    void* iter;  // [bp-0x110]
    int v1;  // [bp-0xec]
    unsigned int v2;  // [bp-0xe8]
    unsigned int v3;  // [bp-0xe4]
    void* v4;  // [bp-0xe0]
    char *v5;  // [bp-0xd8]
    char *v6;  // [bp-0xd0]
    char *idx;  // [bp-0xc8]
    unsigned long n;  // [bp-0xc0]
    char *v9;  // [bp-0xb8]
    char *idx1;  // [bp-0xb0]
    unsigned long n1;  // [bp-0xa8]
    char *v12;  // [bp-0xa0]
    char *idx2;  // [bp-0x98]
    char *v14;  // [bp-0x90]
    char *v15;  // [bp-0x88]
    char *v16;  // [bp-0x80]
    char *index;  // [bp-0x78]
    unsigned long long v18;  // [bp-0x70]
    char v19;  // [bp-0x68]
    unsigned long long v20;  // [bp-0x60]
    char *v21;  // [bp-0x58], Other Possible Types: unsigned long long
    void* ptr;  // [bp-0x50], Other Possible Types: char *, unsigned long long
    unsigned long long v23;  // [bp-0x48]
    unsigned long v24;  // [bp-0x38]
    char v25;  // [bp-0x18]

    iter = a2;
    v2 = strlen(a0);
    v4 = iter;
    v1 = 0;
    if (!iter || !a3)
        return;
    _obstack_begin(&v19, 0, 0, sub_460b5b, free);
    for (; a3 >= v1 && (v3 = (unsigned int)strcspn(iter, ",\n"), v3); iter += v3 + 1)
    {
        if (v4 != iter)
        {
            v5 = &v19;
            v6 = v5;
            if (!(v23 - ptr))
                _obstack_newchunk(v5, 1);
            v27 = ptr;
            ptr = v27 + 1;
            *(v27) = a1;
        }
        idx = &v19;
        n = v2;
        v9 = idx;
        if (v23 - ptr < n)
            _obstack_newchunk(idx, n, n);
        memcpy(ptr, a0, n);
        ptr = &ptr[n];
        idx1 = &v19;
        n1 = v3;
        v12 = idx1;
        if (v23 - ptr < n1)
            _obstack_newchunk(idx1, n1, n1);
        memcpy(ptr, iter, n1);
        ptr += n1;
        v1 += v3 + 1;
    }
    idx2 = &v19;
    v14 = idx2;
    if (!(v23 - ptr))
        _obstack_newchunk(idx2, 1);
    v28 = ptr;
    ptr = v28 + 1;
    *(v28) = 0;
    v15 = &v19;
    v16 = v21;
    if (v16 == ptr)
        v25 |= 2;
    ptr = ~(v24) & &ptr[v24];
    if (ptr - v20 > v23 - v20)
        ptr = v23;
    v21 = ptr;
    fprintf(g_49efd0, "%s", v16);
    index = &v19;
    v18 = 0;
    if (v18 > v20 && v18 < v23)
    {
        v21 = v18;
        ptr = v21;
        return;
    }
    _obstack_free(index, v18, v18);
    return;
    return;
}



// Function: xattrs_acls_get @ 0x3cf89
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern void g_49f968;
extern unsigned int g_49fdb0;
extern struct_0 *g_49fdc8;

long long xattrs_acls_get(unsigned int a0, unsigned long long a1, unsigned long long a2, unsigned int a3, unsigned int a4)
{
    unsigned long v6;  // rax
    unsigned int v0;  // [bp-0x24]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned int v3;  // [bp-0x10]
    unsigned int v4;  // [bp-0xc]

    v4 = a0;
    v2 = a1;
    v1 = a2;
    v3 = a3;
    v0 = a4;
    if (*((int *)&g_49f968) <= 0)
        return *((int *)&g_49f968);
    v6 = g_49fdb0;
    if (!g_49fdb0)
    {
        if (g_49fdc8)
            g_49fdc8();
        v6 = (unsigned long long)error(0, 0, gettext("POSIX ACL support is not available"));
    }
    g_49fdb0 = 1;
    return v6;
}



// Function: xattrs_acls_set @ 0x3d003
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern void g_49f968;
extern unsigned int g_49fdb4;
extern struct_0 *g_49fdc8;

long long xattrs_acls_set(unsigned long long a0, unsigned long long a1, char a2)
{
    unsigned long v3;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v1 = a0;
    v0 = a1;
    if (*((int *)&g_49f968) <= 0)
    {
        return *((int *)&g_49f968);
    }
    else if (a2 != 50)
    {
        v3 = g_49fdb4;
        if (!g_49fdb4)
        {
            if (g_49fdc8)
                g_49fdc8();
            v3 = (unsigned long long)error(0, 0, gettext("POSIX ACL support is not available"));
        }
        g_49fdb4 = 1;
        return v3;
    }
    else
    {
        return *((int *)&g_49f968);
    }
}



// Function: mask_map_realloc @ 0x3d07e
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
} struct_0;

unsigned long long mask_map_realloc(struct_0 *a0)
{
    unsigned long long v1;  // rax

    if (a0->field_10 != a0->field_8)
        return a0->field_8;
    if (!a0->field_8)
        a0->field_8 = 4;
    v1 = sub_460a5f(a0->field_0, &a0->field_8, 8, &a0->field_8);
    a0->field_0 = v1;
    return v1;
}



// Function: xattrs_mask_add @ 0x3d0e5
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
} struct_0;

extern struct_0 g_49fd80;
extern struct_0 g_49fd98;

long long xattrs_mask_add(unsigned long a0, char a1)
{
    unsigned long v2;  // rsi
    unsigned long index;  // rax
    struct_0 *idx;  // [bp-0x10]

    idx = (!a1 ? &g_49fd98.field_0 : &g_49fd80.field_0);
    sub_43d07e(idx);
    v2 = idx->field_0;
    index = idx->field_10;
    idx->field_10 = index + 1;
    *((unsigned long *)(v2 + index * 8)) = a0;
    return a0;
}



// Function: clear_mask_map @ 0x3d14d
typedef struct struct_0 {
    void* field_0;
    unsigned long long field_8;
} struct_0;

long long clear_mask_map(struct_0 *a0)
{
    unsigned long long v1;  // rax

    v1 = a0->field_8;
    if (a0->field_8)
        v1 = (unsigned long long)free(a0->field_0);
    return v1;
}



// Function: xattrs_clear_setup @ 0x3d17c
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern struct_0 g_49fd80;
extern struct_0 g_49fd98;

long long xattrs_clear_setup(void)
{
    sub_43d14d(&g_49fd80.field_0);
    return sub_43d14d(&g_49fd98.field_0);
}



// Function: xattrs_xattrs_get @ 0x3d1a5
extern long long g_49ec88;
extern long long g_49ec90;
extern void g_49f96c;
extern unsigned long long g_49fdb8;
extern long long g_49fdc0;

long long xattrs_xattrs_get(unsigned int a0, long long a1, long long *a2, unsigned int a3)
{
    long long v5;  // rax
    long long i;  // [bp-0x28], Other Possible Types: unsigned long long
    unsigned long long iter;  // [bp-0x20]
    long long v2;  // [bp-0x18], Other Possible Types: unsigned long long
    unsigned long len;  // [bp-0x10]

    if (*((int *)&g_49f96c) <= 0)
        return *((int *)&g_49f96c);
    i = 0xffffffffffffffff;
    if (!g_49fdb8)
        g_49fdb8 = sub_460a5f(g_49fdb8, &g_49ec88, 1, &g_49ec88);
    while (1)
    {
        if (!a3)
        {
            i = sub_447394(a0, a1, g_49fdb8, g_49ec88);
            if (i != -0x1)
                break;
        }
        else
        {
            i = flistxattr(a3, g_49fdb8, g_49ec88, g_49fdb8);
            if (i != -0x1)
                break;
        }
        if (*(__errno_location()) != 0x22)
            break;
        g_49fdb8 = sub_460a5f(g_49fdb8, &g_49ec88, 1, &g_49ec88);
    }
    if (i == -0x1)
        return sub_43e038((!a3 ? "llistxattrat" : "flistxattr"), a1, a1);
    iter = g_49fdb8;
    if (!g_49fdc0)
        g_49fdc0 = sub_460a5f(g_49fdc0, &g_49ec90, 1, &g_49ec90);
    for (; i > 0; i = i - len - 1)
    {
        len = strlen(iter);
        v2 = 0;
        while (1)
        {
            if (!a3)
            {
                v2 = sub_446e17(a0, a1, iter, g_49fdc0, g_49ec90);
                if (v2 != -0x1)
                    break;
            }
            else
            {
                v2 = fgetxattr(a3, iter, g_49fdc0, g_49ec90);
                if (v2 != -0x1)
                    break;
            }
            if (*(__errno_location()) != 0x22)
                break;
            g_49fdc0 = sub_460a5f(g_49fdc0, &g_49ec90, 1, &g_49ec90);
        }
        if (v2 != -0x1)
        {
            if ((char)sub_43d84f(iter, 1) != 1)
                sub_41d78e(a2, iter, g_49fdc0, v2);
        }
        else
        {
            if (*(__errno_location()) != 61)
                sub_43e038((!a3 ? "lgetxattrat" : "fgetxattr"), a1, a1);
        }
        iter += len + 1;
    }
    return v5;
}



// Function: xattrs__fd_set @ 0x3d44b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern unsigned int g_49c268;
extern unsigned int g_49ec84;
extern struct_0 *g_49fdc8;

long long xattrs__fd_set(unsigned long long a0, long long a1, unsigned int a2, long long a3, long long a4, long long a5)
{
    unsigned long v5;  // rax
    unsigned long v6;  // rax
    char *v7;  // rax
    char v0;  // [bp-0x3c]
    unsigned long long v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x24]
    unsigned long long v3;  // [bp-0x20]

    v1 = a0;
    v0 = a2;
    if (!a4)
        return a2;
    v3 = "setxattrat";
    v2 = 0xffffffff;
    if (v0 != 50)
    {
        v5 = sub_446591(g_49c268, a1, a3, a4, a5, 0);
        v2 = v5;
    }
    else
    {
        v3 = "lsetxattr";
        v5 = sub_446870(g_49c268, a1, a3, a4, a5, 0);
        v2 = v5;
    }
    if (v2 != 0xffffffff)
        return v5;
    v6 = g_49ec84 & 0x200000;
    if (!(unsigned int)v6)
        return v6;
    if (g_49fdc8)
        g_49fdc8();
    v7 = gettext("%s: Cannot set '%s' extended attribute for file '%s'");
    return (unsigned long long)error(0, *(__errno_location()), v7);
}



// Function: xattrs_selinux_get @ 0x3d566
extern void g_49f964;

long long xattrs_selinux_get(unsigned int a0, long long a1, long long a2, unsigned int a3)
{
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    unsigned int v0;  // [bp-0xc]

    if (*((int *)&g_49f964) <= 0)
        return *((int *)&g_49f964);
    v0 = (!a3 ? sub_45f56b(a0, a1, a2 + 48, a1) : sub_461d8a(a3, a2 + 48, a2 + 48));
    if (v0 != 0xffffffff)
        return v0;
    v3 = *(__errno_location());
    if ((unsigned int)v3 == 61)
        return v3;
    v4 = *(__errno_location());
    if ((unsigned int)v4 == 95)
        return v4;
    return sub_43e038((!a3 ? "lgetfileconat" : "fgetfilecon"), a1, a1);
}



// Function: xattrs_selinux_set @ 0x3d609
typedef struct struct_0 {
    char padding_0[48];
    long long field_30;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

extern unsigned int g_49c268;
extern unsigned int g_49ec84;
extern void g_49f964;
extern struct_1 *g_49fdc8;

long long xattrs_selinux_set(struct_0 *a0, long long a1, char a2)
{
    unsigned long v3;  // rax
    unsigned long v4;  // rax
    char *v5;  // rax
    unsigned int v0;  // [bp-0x24]
    unsigned long long v1;  // [bp-0x20], Other Possible Types: unsigned long

    if (*((int *)&g_49f964) <= 0)
        return *((int *)&g_49f964);
    v1 = "setfilecon";
    if (!a0->field_30)
        return a0->field_30;
    if (a2 != 50)
    {
        v0 = sub_45f7f9(g_49c268, a1, a0->field_30, a1);
        v3 = "setfileconat";
        v1 = "setfileconat";
    }
    else
    {
        v0 = sub_45fa87(g_49c268, a1, a0->field_30, a1);
        v3 = "lsetfileconat";
        v1 = "lsetfileconat";
    }
    if (v0 != 0xffffffff)
        return v3;
    v4 = g_49ec84 & 0x200000;
    if (!(unsigned int)v4)
        return v4;
    if (g_49fdc8)
        g_49fdc8();
    v5 = gettext("%s: Cannot set SELinux context for file '%s'");
    return (unsigned long long)error(0, *(__errno_location()), v5);
}



// Function: xattrs_matches_mask @ 0x3d717
unsigned int xattrs_matches_mask(char *a0, unsigned long long *a1)
{
    int i;  // [bp-0xc]

    if (!a1[1])
        return 0;
    for (i = 0; i < a1[2]; i += 1)
    {
        if (!fnmatch(*((long long *)(i * 8 + *(a1))), a0, 0))
            return 1;
    }
    return 0;
}



// Function: xattrs_kw_included @ 0x3d79b
extern unsigned long long g_49fd80;
extern unsigned long long g_49fd88;

unsigned int xattrs_kw_included(char *a0, char a1)
{
    unsigned long long v1;  // rax

    if (g_49fd88)
    {
        return sub_43d717(a0, &g_49fd80);
    }
    else if (a1)
    {
        return 1;
    }
    else
    {
        v1 = strncmp(a0, "user.", 5);
        return _INSERT(v1, 0, !(unsigned int)v1);
    }
}



// Function: xattrs_kw_excluded @ 0x3d803
extern unsigned long long g_49fd98;
extern unsigned long long g_49fda0;

unsigned int xattrs_kw_excluded(char *a0, char a1)
{
    char v0;  // [bp-0x14]

    v0 = a1;
    if (g_49fda0 && (char)sub_43d717(a0, &g_49fd98))
        return 1;
    return 0;
}



// Function: xattrs_masked_out @ 0x3d84f
unsigned int xattrs_masked_out(char *a0, char a1)
{
    if ((char)sub_43d79b(a0, a1) && !(char)sub_43d803(a0, a1))
        return 0;
    return 1;
}



// Function: xattrs_xattrs_set @ 0x3d8a1
typedef struct struct_0 {
    char padding_0[352];
    unsigned long long field_160;
} struct_0;

extern void g_49f96c;

unsigned long long xattrs_xattrs_set(struct_0 *a0, long long a1, char a2, unsigned int a3)
{
    unsigned long long idx;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    if (*((int *)&g_49f96c) <= 0)
    {
        return *((int *)&g_49f96c);
    }
    else if (*((long long *)&a0->padding_0[344]))
    {
        for (idx = 0; idx < *((long long *)&a0->padding_0[344]); idx += 1)
        {
            v1 = *((long long *)(idx * 24 + a0->field_160));
            v1 += strlen("SCHILY.xattr.");
            if ((a2 != 48 || a3 != strcmp(v1, "security.capability")) && !(char)sub_43d84f(v1, 0))
                sub_43d44b(a0, a1, a2, v1, *((long long *)(idx * 24 + a0->field_160 + 8)), *((long long *)(idx * 24 + a0->field_160 + 16)));
        }
        return *((long long *)&a0->padding_0[344]);
    }
    else
    {
        return *((long long *)&a0->padding_0[344]);
    }
}



// Function: xattrs_print_char @ 0x3d9ec
typedef struct struct_1 {
    char field_0;
    char field_1;
} struct_1;

typedef struct struct_0 {
    char padding_0[48];
    unsigned long long field_30;
    char padding_38[8];
    unsigned long long field_40;
    char padding_48[8];
    struct struct_1 *field_50;
    char padding_58[256];
    unsigned long long field_158;
    unsigned long long field_160;
} struct_0;

extern int g_49f964;
extern void g_49f968;
extern int g_49f96c;
extern int g_49f9ac;

char * xattrs_print_char(struct_0 *a0, char *a1)
{
    unsigned long len;  // rax
    int i;  // [bp-0x24]
    char *v1;  // [bp-0x20]

    if (g_49f9ac <= 1)
    {
        *(a1) = 0;
        return a1;
    }
    if (g_49f96c > 0 || g_49f964 > 0 || *((int *)&g_49f968) > 0)
    {
        *(a1) = 32;
        a1[1] = 0;
    }
    if (g_49f96c > 0 && a0->field_158)
    {
        for (i = 0; i < a0->field_158; i += 1)
        {
            len = strlen("SCHILY.xattr.");
            v1 = len + *((long long *)(i * 24 + a0->field_160));
            if ((char)sub_43d84f(v1, 0) != 1)
            {
                *(a1) = 42;
                break;
            }
        }
    }
    if (g_49f964 > 0 && a0->field_30)
        *(a1) = 46;
    if (*((int *)&g_49f968) <= 0)
        return *((int *)&g_49f968);
    if (!a0->field_40 && !a0->field_50)
        return a0->field_50;
    *(a1) = 43;
    return a1;
}



// Function: xattrs_print @ 0x3db33
typedef struct FILE {
} FILE;

extern void g_487cb4;
extern FILE *g_49efd0;
extern int g_49f964;
extern int g_49f968;
extern void g_49f96c;
extern void g_49f9ac;

unsigned long long xattrs_print(void* a0)
{
    unsigned long len;  // rax
    int i;  // [bp-0x24]
    char *v1;  // [bp-0x20]

    if (*((int *)&g_49f9ac) <= 2)
        return *((int *)&g_49f9ac);
    if (g_49f964 > 0 && (long long)a0[48])
        fprintf(g_49efd0, "  s: %s\n", (long long)a0[48]);
    if (g_49f968 > 0 && ((long long)a0[64] || (long long)a0[80]))
    {
        fprintf(g_49efd0, "  a: ");
        sub_43cae5(&g_487cb4, 44, (long long)a0[56], (long long)a0[64]);
        if ((long long)a0[64] && (long long)a0[80])
            fprintf(g_49efd0, ",");
        sub_43cae5("default:", 44, (long long)a0[72], (long long)a0[80]);
        fprintf(g_49efd0, "\n");
    }
    if (*((int *)&g_49f96c) <= 0)
    {
        return *((int *)&g_49f96c);
    }
    else if ((long long)a0[344])
    {
        for (i = 0; i < (long long)a0[344]; i += 1)
        {
            len = strlen("SCHILY.xattr.");
            v1 = len + *((long long *)(i * 24 + (long long)a0[352]));
            if ((char)sub_43d84f(v1, 0) != 1)
                fprintf(g_49efd0, "  x: %lu %s\n", *((long long *)(i * 24 + (long long)a0[352] + 16)), v1);
        }
        return (long long)a0[344];
    }
    else
    {
        return (long long)a0[344];
    }
}



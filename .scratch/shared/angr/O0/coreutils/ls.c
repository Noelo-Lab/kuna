// Function: file_or_link_mode @ 0x53e7
typedef struct struct_0 {
    char padding_0[48];
    unsigned int field_30;
    char padding_34[120];
    unsigned int field_ac;
    char padding_b0[9];
    char field_b9;
} struct_0;

extern char g_42b358;

int file_or_link_mode(struct_0 *a0)
{
    if (g_42b358 && a0->field_b9)
        return a0->field_ac;
    return a0->field_30;
}



// Function: dired_outbyte @ 0x5422
extern unsigned long long g_42b500;

int dired_outbyte(char a0)
{
    g_42b500 = g_42b500 + 1;
    return putchar_unlocked(a0);
}



// Function: dired_outbuf @ 0x5453
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned long long g_42b500;

long long dired_outbuf(void* a0, unsigned long a1)
{
    g_42b500 = a1 + g_42b500;
    return fwrite_unlocked(a0, 1, a1, stdout);
}



// Function: dired_outstring @ 0x549e
long long dired_outstring(void* a0)
{
    return sub_405453(a0, strlen(a0));
}



// Function: dired_indent @ 0x54cf
extern char g_42b3cc;

long long dired_indent(void)
{
    unsigned long v1;  // rax

    v1 = g_42b3cc;
    if (g_42b3cc)
        v1 = sub_40549e("  ");
    return v1;
}



// Function: push_current_dired_pos @ 0x54f4
typedef struct struct_0 {
    char padding_0[24];
    void* field_18;
    unsigned long long field_20;
} struct_0;

extern char g_42b3cc;
extern void g_42b500;

struct_0 * push_current_dired_pos(struct_0 *idx)
{
    unsigned long n;  // [bp-0x18]

    if (!g_42b3cc)
        return g_42b3cc;
    n = 8;
    if (idx->field_20 - idx->field_18 < n)
        _obstack_newchunk(idx, n, n);
    memcpy(idx->field_18, &g_42b500, n);
    idx->field_18 = idx->field_18 + n;
    return idx;
}



// Function: dev_ino_push @ 0x558e
extern unsigned long long g_42b5e0[5];
extern void* g_42b5f8;

long long dev_ino_push(unsigned long a0, unsigned long a1)
{
    int v0;  // [bp-0x34]
    unsigned long long *idx;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28]
    unsigned long long *v3;  // [bp-0x20]
    void* v4;  // [bp-0x18]
    void* iter;  // [bp-0x10], Other Possible Types: unsigned long

    v0 = 16;
    idx = &g_42b5e0[0];
    v2 = v0;
    v3 = idx;
    if (v3[4] - v3[3] < v2)
        _obstack_newchunk(idx, v2, v2);
    idx[3] = idx[3] + v2;
    v4 = g_42b5f8;
    iter = v4;
    iter -= 16;
    *((unsigned long *)(iter + 8)) = a0;
    *((unsigned long *)iter) = a1;
    return iter;
}



// Function: dev_ino_pop @ 0x563d
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_42b5e0[4];
extern struct_0 *g_42b5f8;

long long dev_ino_pop(void)
{
    int v0;  // [bp-0x24]
    unsigned long long *v1;  // [bp-0x20]
    unsigned long long *v2;  // [bp-0x18]
    unsigned long long *v3;  // [bp-0x10]

    v0 = 16;
    v1 = &g_42b5e0[0];
    if (v1[3] - v1[2] >= v0)
    {
        g_42b5f8 = -(v0) + (char *)g_42b5f8;
        v2 = &g_42b5f8->field_0;
        v3 = v2;
        return *(v3);
    }
    __assert_fail(); /* do not return */
}



// Function: assert_matching_dev_ino @ 0x56dc
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

void assert_matching_dev_ino(char *a0, unsigned long a1, unsigned long a2)
{
    stat v0;  // [bp-0xa8]

    if (!a0)
    {
        __assert_fail(); /* do not return */
    }
    else if (stat(a0, &v0) < 0)
    {
        __assert_fail(); /* do not return */
    }
    else if (*((unsigned long long *)&v0) != a2)
    {
        __assert_fail(); /* do not return */
    }
    else if (v0.st_ino != a1)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        return;
    }
}



// Function: dired_dump_obstack @ 0x581f
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

typedef struct FILE {
} FILE;

extern FILE *stdout;

unsigned long long dired_dump_obstack(char *a0, struct_0 *a1)
{
    unsigned long long idx1;  // [bp-0x40]
    struct_0 *index;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    struct_0 *idx;  // [bp-0x28]
    unsigned long v4;  // [bp-0x20]
    unsigned long v5;  // [bp-0x18]
    unsigned long v6;  // [bp-0x10]

    index = a1;
    v2 = index->field_18 - index->field_10 >> 3;
    if (!v2)
        return v2;
    idx = a1;
    v4 = idx->field_10;
    if (v4 == idx->field_18)
        idx->field_50 = idx->field_50 | 2;
    idx->field_18 = ~(idx->field_30) & idx->field_18 + idx->field_30;
    if (idx->field_18 - idx->field_8 > idx->field_20 - idx->field_8)
        idx->field_18 = idx->field_20;
    idx->field_10 = idx->field_18;
    v5 = v4;
    fputs_unlocked(a0, stdout);
    for (idx1 = 0; idx1 < v2; idx1 += 1)
    {
        v6 = *((long long *)(v5 + idx1 * 8));
        printf(" %ld", v6);
    }
    return putchar_unlocked(10);
}



// Function: get_stat_btime @ 0x5990
unsigned long long get_stat_btime(long long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = sub_41b491(a0);
    return v0;
}



// Function: time_type_to_statx @ 0x59be
extern unsigned int g_42b3b0;

unsigned int time_type_to_statx(void)
{
    if (g_42b3b0 == 3)
        return 0x800;
    if (g_42b3b0 <= 3)
    {
        if (g_42b3b0 == 2)
        {
            return 32;
        }
        else if (g_42b3b0 <= 2)
        {
            if (!g_42b3b0)
            {
                return 64;
            }
            else if (g_42b3b0 == 1)
            {
                return 128;
            }
        }
    }
    abort(); /* do not return */
}



// Function: calc_req_mask @ 0x5a0c
extern char g_42b022;
extern char g_42b023;
extern unsigned int g_42b3ac;
extern unsigned int g_42b3b4;
extern char g_42b3b9;
extern char g_42b3bb;
extern char g_42b3e9;

unsigned int calc_req_mask(void)
{
    unsigned int v0;  // [bp-0xc]

    v0 = 2;
    if (g_42b3e9)
        v0 |= 0x100;
    if (g_42b3bb)
        v0 |= 0x400;
    if (!g_42b3ac)
    {
        v0 = sub_4059be() | v0 | 516;
        if (g_42b022 || g_42b3b9)
            v0 |= 8;
        if (g_42b023)
            v0 |= 16;
    }
    switch (g_42b3b4)
    {
    case 0: case 1: case 2: case 4: case 6:
        return v0;
    case 3:
        v0 |= 0x200;
        return v0;
    case 5:
        v0 |= sub_4059be();
        return v0;
    default:
        abort(); /* do not return */
    }
}



// Function: do_statx @ 0x5ad2
long long do_statx(unsigned int a0, long long a1, unsigned long long *idx, unsigned int a3, unsigned int a4)
{
    unsigned long v6;  // rdx
    char v0;  // [bp-0x12d]
    unsigned int v1;  // [bp-0x12c]
    char v2;  // [bp-0x128]
    long long v3;  // [bp-0xd8]
    long long v4;  // [bp-0xd0]

    v0 = a4 & 0x800;
    v1 = statx(a0, a1, a3 | 8, a4, &v2);
    if (v1 < 0)
        return v1;
    sub_405297(&v2, idx, idx);
    if (!v0)
    {
        return v1;
    }
    else if (v2 & 0x800)
    {
        idx[11] = sub_405267(v3, v4, v3);
        idx[12] = v6;
        return v1;
    }
    else
    {
        idx[12] = 0xffffffffffffffff;
        idx[11] = idx[12];
        return v1;
    }
}



// Function: do_stat @ 0x5c0e
long long do_stat(long long a0, unsigned long long *a1)
{
    return sub_405ad2(0xffffff9c, a0, a1, 0, sub_405a0c());
}



// Function: do_lstat @ 0x5c48
long long do_lstat(long long a0, unsigned long long *a1)
{
    return sub_405ad2(0xffffff9c, a0, a1, 0x100, sub_405a0c());
}



// Function: stat_for_mode @ 0x5c82
long long stat_for_mode(long long a0, unsigned long long *a1)
{
    return sub_405ad2(0xffffff9c, a0, a1, 0, 2);
}



// Function: stat_for_ino @ 0x5cb8
long long stat_for_ino(long long a0, unsigned long long *a1)
{
    return sub_405ad2(0xffffff9c, a0, a1, 0, 0x100);
}



// Function: fstat_for_ino @ 0x5cee
extern long long g_41f27b;

long long fstat_for_ino(unsigned int a0, unsigned long long *a1)
{
    return sub_405ad2(a0, &g_41f27b, a1, 0x1000, 0x100);
}



// Function: first_percent_b @ 0x5d23
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

struct_0 * first_percent_b(struct_0 *a0)
{
    struct_0 *iter;  // [bp-0x10]

    iter = a0;
    while (iter->field_0)
    {
        if (iter->field_0 != 37)
            goto LABEL_405d60;
        switch (iter->field_1)
        {
        case 37:
            iter = &iter->field_1;
            goto LABEL_405d60;
        case 98:
            return iter;
        default:
LABEL_405d60:
            iter = &iter->field_1;
            break;
        }
    }
    return NULL;
}



// Function: file_escape_init @ 0x5d77
extern char g_42b640;

int file_escape_init(void)
{
    char v2;  // cl
    unsigned int v3;  // eax
    unsigned int i;  // [bp-0xc]

    for (i = 0; i <= 0xff; i += 1)
    {
        if ((char)sub_410016(i) || i == 126 || i == 45 || i == 46 || i == 95)
            v2 = 1;
        else
            v2 = 0;
        *(&(&g_42b640)[i]) = *(&(&g_42b640)[i]) | v2;
    }
    return v3;
}



// Function: abmon_init @ 0x5df4
unsigned int abmon_init(unsigned long a0)
{
    unsigned long long *v8;  // rax
    char v9;  // al
    unsigned int j;  // [bp-0x50]
    unsigned int v1;  // [bp-0x4c]
    unsigned long v2;  // [bp-0x48]
    unsigned long v3;  // [bp-0x40]
    unsigned long i;  // [bp-0x38]
    char *v5;  // [bp-0x30]
    unsigned long long v6;  // [bp-0x28]

    v3 = 12;
    do
    {
        i = v3;
        v3 = 0;
        for (j = 0; j <= 11; j += 1)
        {
            v2 = i;
            v5 = nl_langinfo(j + 131086);
            if (strchr(v5, 37))
                return 0;
            v8 = __ctype_b_loc();
            v9 = sub_404eb2(*(v5));
            v1 = *((short *)(v9 * 2 + *(v8))) & 0x800;
            v6 = sub_415291(v5, j * 128 + a0, 128, &v2, v1, 0);
            if (v6 > 127)
                return 0;
            if (v2 > v3)
                v3 = v2;
        }
    } while (i > v3);
    return 1;
}



// Function: abformat_init @ 0x5f43
extern unsigned long long g_42b200[4];
extern char g_42b740;
extern char g_42c340;

void abformat_init(void)
{
    unsigned long v0;  // [bp-0x668]
    int i;  // [bp-0x64c]
    int j;  // [bp-0x648]
    int k;  // [bp-0x644]
    int v4;  // [bp-0x640]
    unsigned int v5;  // [bp-0x63c]
    unsigned long long *v6;  // [bp-0x638]
    char *v7;  // [bp-0x630]
    int <0x405f43[is_2]|Stack bp-0x628, 1 B>;  // [bp-0x628]
    unsigned long long v9;  // [bp-0x620]
    char v10;  // [bp-0x618]

    for (i = 0; i <= 1; i += 1)
    {
        (&<0x405f43[is_2]|Stack bp-0x628, 1 B>)[i] = sub_405d23(g_42b200[i]);
    }
    if (!<0x405f43[is_2]|Stack bp-0x628, 1 B> && !v9)
        return;
    if ((char)sub_405df4(&v10) != 1)
        return;
    for (j = 0; j <= 1; j += 1)
    {
        v6 = g_42b200[j];
        for (k = 0; k <= 11; k += 1)
        {
            v7 = &(&g_42b740)[0x600 * j + 128 * k];
            if (!(&<0x405f43[is_2]|Stack bp-0x628, 1 B>)[j])
            {
                v4 = snprintf(v7, 128, "%s", v6);
            }
            else if ((&<0x405f43[is_2]|Stack bp-0x628, 1 B>)[j] - (char *)v6 <= 128)
            {
                v5 = (unsigned int)(unsigned long long *)((&<0x405f43[is_2]|Stack bp-0x628, 1 B>)[j] - (char *)v6);
                v0 = (&<0x405f43[is_2]|Stack bp-0x628, 1 B>)[j] + 2;
                v4 = snprintf(v7, 128, "%.*s%s%s", v5, v6, &(&v10)[128 * k]);
            }
            else
            {
                return;
            }
            if (v4 < 0 || v4 > 127)
                return;
        }
    }
    g_42c340 = 1;
    return;
}



// Function: dev_ino_hash @ 0x61bd
long long dev_ino_hash(unsigned long long *a0, unsigned long long a1)
{
    return *(a0) % a1;
}



// Function: dev_ino_compare @ 0x61ea
unsigned int dev_ino_compare(unsigned long long *a0, unsigned long a1)
{
    unsigned int v1;  // eax

    if (*(a0) == *((long long *)a1) && a0[1] == *((long long *)(a1 + 8)))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: dev_ino_free @ 0x6243
void dev_ino_free(void* a0)
{
    free(a0);
    return;
}



// Function: visit_dir @ 0x6262
extern long long g_42b320;

long long visit_dir(unsigned long a0, unsigned long a1)
{
    char v0;  // [bp-0x19]
    unsigned long long *ptr;  // [bp-0x18]
    unsigned long long *v2;  // [bp-0x10]

    ptr = sub_41c68b(16);
    *(ptr) = a1;
    ptr[1] = a0;
    v2 = sub_4136d0(g_42b320, ptr, ptr);
    if (!v2)
        sub_41ccc3(); /* do not return */
    v0 = v2 != ptr;
    if (!v0)
        return v0;
    free(ptr);
    return v0;
}



// Function: free_pending_ent @ 0x62e7
void free_pending_ent(void* *a0)
{
    free(*(a0));
    free(a0[1]);
    free(a0);
    return;
}



// Function: is_colored @ 0x6325
extern unsigned long long g_42b080[4];
extern unsigned long long g_42b088[4];

unsigned int is_colored(unsigned int a0)
{
    unsigned int v3;  // eax
    unsigned long long *v0;  // [bp-0x18]
    unsigned long long *v1;  // [bp-0x10]

    v0 = g_42b080[2 * a0];
    v1 = g_42b088[2 * a0];
    if (v0 && (v0 != 0x1 || strncmp(v1, "0", 1)) && (v0 != 0x2 || strncmp(v1, "00", 2)))
        v3 = 1;
    else
        v3 = 0;
    return v3 & 1;
}



// Function: restore_default_color @ 0x63ca
extern long long g_42b080;
extern long long g_42b090;

int restore_default_color(void)
{
    sub_40e4ca(&g_42b080);
    return sub_40e4ca(&g_42b090);
}



// Function: set_normal_color @ 0x63f3
extern long long g_42b080;
extern long long g_42b090;
extern long long g_42b0c0;
extern char g_42b3d4;

unsigned long long set_normal_color(void)
{
    unsigned long long v1;  // rax

    if (!g_42b3d4)
        return g_42b3d4;
    v1 = sub_406325(4);
    if (!(char)v1)
        return v1;
    sub_40e4ca(&g_42b080);
    sub_40e4ca(&g_42b0c0);
    return sub_40e4ca(&g_42b090);
}



// Function: sighandler @ 0x6444
extern unsigned int g_42b4e0;

long long sighandler(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_42b4e0;
    if (!(unsigned int)v1)
    {
        v1 = a0;
        g_42b4e0 = v1;
    }
    return v1;
}



// Function: stophandler @ 0x6465
extern unsigned int g_42b4e0;
extern unsigned int g_42b4e4;

long long stophandler(unsigned int a0)
{
    unsigned long v2;  // rax
    unsigned int v0;  // [bp-0xc]

    v0 = a0;
    v2 = g_42b4e0;
    if (!g_42b4e0)
    {
        v2 = g_42b4e4 + 1;
        g_42b4e4 = v2;
    }
    return v2;
}



// Function: process_signals @ 0x648c
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_42b3d6;
extern int g_42b460;
extern unsigned int g_42b4e0;
extern unsigned int g_42b4e4;

void process_signals(void)
{
    unsigned int v0;  // [bp-0xa0]
    unsigned int v1;  // [bp-0x9c]
    int v2[34];  // [bp-0x98]

    while (g_42b4e0 || g_42b4e4)
    {
        if (g_42b3d6)
            sub_4063ca();
        fflush_unlocked(stdout);
        sigprocmask(0, &g_42b460, v2);
        v0 = g_42b4e0;
        v1 = g_42b4e4;
        if (v1)
        {
            g_42b4e4 = v1 - 1;
            v0 = 19;
        }
        else
        {
            signal(v0, NULL);
        }
        raise(v0);
        sigprocmask(2, v2, NULL);
    }
    return;
}



// Function: signal_setup @ 0x6593
typedef struct sigaction {
} sigaction;

extern unsigned int g_423380[4];
extern unsigned long long g_42b460;
extern unsigned long long g_42b468;
extern unsigned long long g_42b470;
extern unsigned long long g_42b478;
extern unsigned long long g_42b480;
extern unsigned long long g_42b488;
extern unsigned long long g_42b490;
extern unsigned long long g_42b498;
extern unsigned long long g_42b4a0;
extern unsigned long long g_42b4a8;
extern unsigned long long g_42b4b0;
extern unsigned long long g_42b4b8;
extern unsigned long long g_42b4c0;
extern unsigned long long g_42b4c8;
extern unsigned long long g_42b4d0;
extern unsigned long long g_42b4d8;

void signal_setup(char a0)
{
    int iter;  // [bp-0xac]
    sigaction v1;  // [bp-0xa8]
    unsigned long v2;  // [bp-0xa0]
    unsigned long v3;  // [bp-0x98]
    unsigned long v4;  // [bp-0x90]
    unsigned long v5;  // [bp-0x88]
    unsigned long v6;  // [bp-0x80]
    unsigned long v7;  // [bp-0x78]
    unsigned long v8;  // [bp-0x70]
    unsigned long v9;  // [bp-0x68]
    unsigned long v10;  // [bp-0x60]
    unsigned long v11;  // [bp-0x58]
    unsigned long v12;  // [bp-0x50]
    unsigned long v13;  // [bp-0x48]
    unsigned long v14;  // [bp-0x40]
    unsigned long v15;  // [bp-0x38]
    unsigned long v16;  // [bp-0x30]
    unsigned long v17;  // [bp-0x28]
    unsigned int v18;  // [bp-0x20]

    if (a0)
    {
        sigemptyset(&g_42b460);
        for (iter = 0; iter <= 11; iter += 1)
        {
            sigaction(g_423380[iter], NULL, &v1);
            if (v1 != 1)
                sigaddset(&g_42b460, g_423380[iter]);
        }
        v2 = g_42b460;
        v3 = g_42b468;
        v4 = g_42b470;
        v5 = g_42b478;
        v6 = g_42b480;
        v7 = g_42b488;
        v8 = g_42b490;
        v9 = g_42b498;
        v10 = g_42b4a0;
        v11 = g_42b4a8;
        v12 = g_42b4b0;
        v13 = g_42b4b8;
        v14 = g_42b4c0;
        v15 = g_42b4c8;
        v16 = g_42b4d0;
        v17 = g_42b4d8;
        v18 = 0x10000000;
        for (iter = 0; iter <= 11; iter += 1)
        {
            if (sigismember(&g_42b460, g_423380[iter]))
            {
                v1 = (sigaction)(g_423380[iter] == 0x14 ? sub_406465 : sub_406444);
                sigaction(g_423380[iter], &v1, NULL);
            }
        }
        return;
    }
    else
    {
        for (iter = 0; iter <= 11; iter += 1)
        {
            if (sigismember(&g_42b460, g_423380[iter]))
                signal(g_423380[iter], NULL);
        }
        return;
    }
}



// Function: signal_init @ 0x6858
void signal_init(void)
{
    sub_406593(1);
    return;
}



// Function: signal_restore @ 0x686d
void signal_restore(void)
{
    sub_406593(0);
    return;
}



// Function: main @ 0x6882
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    char field_10;
    char padding_11[7];
    struct struct_0 *field_18;
} struct_0;

typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_41f27b;
extern void g_420638;
extern unsigned long long g_42a9e0[4];
extern unsigned long long g_42b080;
extern void* g_42b088;
extern unsigned long long g_42b090;
extern struct_1 *g_42b098;
extern FILE *stdout;
extern unsigned long long g_42b320;
extern unsigned long long g_42b328;
extern long long g_42b330;
extern unsigned long long g_42b338;
extern char g_42b358;
extern unsigned long long g_42b360;
extern struct_0 *g_42b368;
extern unsigned long long g_42b370;
extern unsigned long long g_42b378;
extern char g_42b380;
extern char g_42b3ac;
extern unsigned int g_42b3b4;
extern char g_42b3bb;
extern char g_42b3cc;
extern char g_42b3d0;
extern char g_42b3d4;
extern char g_42b3d5;
extern char g_42b3d6;
extern char g_42b3e8;
extern unsigned int g_42b3ec;
extern char g_42b3f0;
extern char g_42b3f1;
extern char g_42b3f2;
extern long long g_42b410;
extern unsigned long long g_42b420;
extern char g_42b428;
extern unsigned long long g_42b438;
extern char g_42b440;
extern char g_42b441;
extern unsigned int g_42b4e0;
extern unsigned int g_42b4e4;
extern unsigned int g_42b4e8;
extern char g_42b520;
extern char g_42b580;
extern long long g_42b5e0;

int main(unsigned int a0, long long *a1)
{
    unsigned int v8;  // eax
    char v9;  // al
    char v10;  // al
    long long v11;  // rdi
    long long v12;  // rsi
    long long v13;  // rdx
    long long v14;  // rcx
    long long v15;  // r8
    long long v16;  // r9
    unsigned int v17;  // eax
    unsigned int i;  // [bp-0x44]
    unsigned int v1;  // [bp-0x44]
    unsigned int j;  // [bp-0x40]
    int v3;  // [bp-0x3c]
    struct_0 *v4;  // [bp-0x38]
    void* v5;  // [bp-0x30]
    unsigned long v6;  // [bp-0x28]

    sub_418e89(*(a1));
    setlocale(6, &g_41f27b);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_404e99(2);
    sub_41ec10(sub_4110ae);
    g_42b4e8 = 0;
    g_42b428 = 1;
    g_42b368 = 0;
    g_42b370 = 0x8000000000000000;
    g_42b378 = 0xffffffffffffffff;
    i = sub_4070af(a0, a1, a1);
    if (g_42b3d4)
        sub_4088ab();
    if (g_42b3d4)
        g_42b420 = 0;
    if (g_42b3f2)
    {
        g_42b3e8 = 1;
    }
    else if (g_42b3d4 && ((char)sub_406325(13) || (char)sub_406325(14) && g_42b358 || (char)sub_406325(12) && !*((int *)&g_42b3ac)))
    {
        g_42b3e8 = 1;
    }
    if (!g_42b3ec)
    {
        if (g_42b3f1 || *((int *)&g_42b3d0) == 3 || !*((int *)&g_42b3ac))
            v8 = 1;
        else
            v8 = 3;
        g_42b3ec = v8;
    }
    if (g_42b3f0)
    {
        g_42b320 = sub_41298f(30, 0, sub_4061bd, sub_4061ea, sub_406243);
        if (!g_42b320)
            sub_41ccc3(); /* do not return */
        _obstack_begin(&g_42b5e0, 0, 0, malloc, free);
    }
    g_42b438 = sub_41b525(getenv("TZ"));
    if (g_42b3b4 == 5 || g_42b3b4 == 3 || !*((int *)&g_42b3ac) || g_42b380 || g_42b3bb)
        v9 = 1;
    else
        v9 = 0;
    g_42b440 = v9 & 1;
    if (g_42b440 != 1 && (g_42b3f0 || g_42b3d4 || *((int *)&g_42b3d0) || g_42b3f2))
        v10 = 1;
    else
        v10 = 0;
    g_42b441 = v10 & 1;
    if (g_42b3cc)
    {
        _obstack_begin(&g_42b520, 0, 0, malloc, free);
        _obstack_begin(&g_42b580, 0, 0, malloc, free);
    }
    if (g_42b3d5)
    {
        sub_405d77();
        g_42b360 = sub_41ce96(v11, v12, v13, v14, v15, v16);
        if (!g_42b360)
            g_42b360 = &g_41f27b;
    }
    g_42b330 = 100;
    g_42b328 = sub_41c804(g_42b330, 208);
    g_42b338 = 0;
    sub_4096f3(v11, v12, v13, v14, v15, v16);
    v3 = a0 - i;
    if (v3 > 0)
    {
        do
        {
            v1 = i + 1;
            sub_409a40(a1[i], 0, 0, 1, &g_41f27b);
            i = v1;
        } while (i < a0);
    }
    else if (g_42b3f1)
    {
        sub_409a40(".", 3, 0, 1, &g_41f27b);
    }
    else
    {
        sub_408e46(".", 0, 1);
    }
    if (g_42b338)
    {
        sub_40bfc5();
        if (g_42b3f1 != 1)
            sub_40aa65(0, 1);
    }
    if (g_42b338)
    {
        sub_40c12d();
        if (g_42b368)
            sub_405422(10);
    }
    else if (v3 <= 1 && g_42b368 && !g_42b368[3].field_0)
    {
        g_42b428 = 0;
    }
    while (g_42b368)
    {
        v4 = g_42b368;
        g_42b368 = g_42b368[3].field_0;
        if (g_42b320 && !v4->field_0)
        {
            v6 = sub_40563d();
            v5 = sub_413740(g_42b320, &v6, &v6);
            if (!v5)
                __assert_fail(); /* do not return */
            sub_406243(v5);
            sub_4062e7(v4);
        }
        else
        {
            sub_408ede(v4->field_0, v4->field_8, v4->field_10, v4->field_8);
            sub_4062e7(v4);
            g_42b428 = 1;
        }
    }
    if (g_42b3d4 && g_42b3d6)
    {
        if (g_42b080 != 2 || memcmp(g_42b088, &g_420638, 2) || g_42b090 != 1 || g_42b098->field_0 != 109)
            sub_4063ca();
        fflush_unlocked(stdout);
        sub_40686d();
        for (j = g_42b4e4; j; j -= 1)
        {
            raise(19);
        }
        j = g_42b4e0;
        if (j)
            raise(j);
    }
    if (g_42b3cc)
    {
        sub_40581f("//DIRED//", &g_42b520);
        sub_40581f("//SUBDIRED//", &g_42b580);
        v17 = sub_418fc3(g_42b410);
        printf("//DIRED-OPTIONS// --quoting-style=%s\n", g_42a9e0[v17]);
    }
    if (g_42b320)
    {
        if (sub_412002(g_42b320))
            __assert_fail(); /* do not return */
        sub_412c0f(g_42b320);
    }
    return g_42b4e8;
}



// Function: decode_line_length @ 0x6fe6
extern long long g_41f27b;

long long decode_line_length(long long a0)
{
    unsigned int v2;  // eax
    char v0;  // [bp-0x18], Other Possible Types: long long

    v2 = sub_41d251(a0, 0, 0, &v0, &g_41f27b);
    if (v2)
        return (v2 == 1 ? 0 : -0x1);
    if (v0 < 0)
        v0 = 0;
    return v0;
}



// Function: stdout_isatty @ 0x706a
extern void g_42b031;

long long stdout_isatty(void)
{
    if (g_42b031 < 0)
        g_42b031 = isatty(1);
    return _INSERT(g_42b031, 0, g_42b031);
}



// Function: decode_switches @ 0x70af
typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern long long g_41f27b;
extern long long g_4205b0;
extern unsigned int g_4205e0[4];
extern unsigned int g_4208a0[4];
extern unsigned int g_4208d0[4];
extern unsigned int g_420920[4];
extern unsigned int g_420980[4];
extern unsigned int g_4238c0[4];
extern char *g_429f60;
extern long long g_429fa0;
extern char g_42a0c0;
extern long long g_42a660;
extern long long g_42a6a0;
extern long long g_42a6e0;
extern long long g_42a720;
extern long long g_42a9e0;
extern char g_42b022;
extern char g_42b023;
extern unsigned long long g_42b028;
extern char g_42b030;
extern unsigned long long g_42b200[4];
extern unsigned long long g_42b208;
extern char g_42b210;
extern long long g_42b218;
extern long long g_42b220;
extern long long stdout;
extern unsigned int optind;
extern struct_0 *optarg;
extern FILE *stderr;
extern char g_42b341;
extern char g_42b380;
extern unsigned int g_42b3ac;
extern unsigned int g_42b3b0;
extern unsigned int g_42b3b4;
extern char g_42b3b8;
extern char g_42b3b9;
extern char g_42b3ba;
extern char g_42b3bb;
extern unsigned int g_42b3bc;
extern unsigned long long g_42b3c0;
extern unsigned int g_42b3c8;
extern char g_42b3cc;
extern unsigned int g_42b3d0;
extern char g_42b3d4;
extern char g_42b3d5;
extern char g_42b3e9;
extern unsigned int g_42b3ec;
extern char g_42b3f0;
extern char g_42b3f1;
extern char g_42b3f2;
extern unsigned int g_42b3f4;
extern unsigned long long g_42b400;
extern char g_42b408;
extern long long g_42b410;
extern long long g_42b418;
extern unsigned long long g_42b420;
extern unsigned long long g_42b430;
extern unsigned long long g_42b4f8;

long long decode_switches(int a0, char **a1)
{
    unsigned long v30;  // rax
    char v31;  // al
    unsigned int *v41;  // eax
    char *v43;  // rax
    char *v44;  // rax
    char v32;  // al
    char v35;  // al
    char v36;  // al
    char v38;  // al
    unsigned long v39;  // rcx
    char v0;  // [bp-0xc5]
    unsigned int *v1;  // [bp-0xc4]
    int v2;  // [bp-0xc0]
    unsigned int *v3;  // [bp-0xbc]
    unsigned int *v4;  // [bp-0xb8]
    unsigned int *v5;  // [bp-0xb4]
    unsigned int *v6;  // [bp-0xb0]
    unsigned int *v7;  // [bp-0xac]
    unsigned int *v8;  // [bp-0xa8], Other Possible Types: unsigned int
    int k;  // [bp-0xa4]
    unsigned int v10;  // [bp-0xa0]
    unsigned int v11;  // [bp-0x9c]
    char *v12;  // [bp-0x98]
    unsigned long long v13;  // [bp-0x90]
    unsigned long v14;  // [bp-0x88]
    unsigned long v15;  // [bp-0x80]
    char *i;  // [bp-0x78]
    char *j;  // [bp-0x70]
    char *ptr;  // [bp-0x68]
    char *v19;  // [bp-0x68]
    char **v20;  // [bp-0x60]
    char **v21;  // [bp-0x60]
    struct_1 *v22;  // [bp-0x58]
    unsigned long v23;  // [bp-0x50]
    char *v24;  // [bp-0x48]
    unsigned long v25;  // [bp-0x40]
    unsigned long v26;  // [bp-0x38]
    char *v27;  // [bp-0x30]
    int v28[2];  // [bp-0x28]

    v12 = NULL;
    v0 = 0;
    v1 = 0xffffffff;
    v2 = -0x1;
    v3 = 0xffffffff;
    v4 = 0xffffffff;
    v13 = 0xffffffffffffffff;
    v14 = 0xffffffffffffffff;
    while (1)
    {
        *(v28) = 0xffffffff;
        v10 = getopt_long(a0, a1, "abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1", &g_42a0c0, v28);
        if (v10 == 0xffffffff)
            break;
        switch (v10)
        {
        case -131:
            sub_41c4ab(stdout, (*((int *)&g_42b210) == 1 ? "ls" : (*((int *)&g_42b210) == 2 ? "dir" : "vdir")), "GNU coreutils", g_42b218, "Richard M. Stallman", "David MacKenzie", 0);
            exit(0); /* do not return */
        case -130:
            sub_40f135(0); /* do not return */
        case 49:
            if (v1)
                v1 = 0x1;
            break;
        case 65:
            g_42b3f4 = 1;
            break;
        case 66:
            sub_4094e9("*~");
            sub_4094e9(".*~");
            break;
        case 67:
            v1 = 0x2;
            break;
        case 68:
            g_42b3cc = 1;
            break;
        case 70:
            v5 = (!optarg ? 0x1 : g_420980[sub_40fe17("--classify", optarg, &g_42a720, &g_420980[0], 4, g_42b220, 1)]);
            v5 = v5;
            if (v5 == 1 || v5 == 2 && (char)sub_40706a())
                g_42b3d0 = 3;
            continue;
        case 71:
            g_42b023 = 0;
            break;
        case 72:
            g_42b3ec = 2;
            break;
        case 73:
            sub_4094e9(optarg);
            break;
        case 76:
            g_42b3ec = 4;
            break;
        case 78:
            v3 = NULL;
            break;
        case 81:
            v3 = 0x5;
            break;
        case 82:
            g_42b3f0 = 1;
            break;
        case 83:
            v4 = 0x3;
            break;
        case 84:
            v30 = gettext("invalid tab size");
            v13 = sub_41cd03(optarg, 0, 0, 0x7fffffffffffffff, &g_41f27b, v30, 2);
            break;
        case 85:
            v4 = 0x6;
            break;
        case 88:
            v4 = 0x1;
            break;
        case 90:
            g_42b380 = 1;
            break;
        case 97:
            g_42b3f4 = 2;
            break;
        case 98:
            v3 = 0x7;
            break;
        case 99:
            g_42b3b0 = 1;
            break;
        case 100:
            g_42b3f1 = 1;
            break;
        case 102:
            g_42b3f4 = 2;
            v4 = 0x6;
            if (!v1)
                v1 = 0xffffffff;
            g_42b3d4 = 0;
            g_42b3d5 = 0;
            g_42b3bb = 0;
            break;
        case 103:
            v1 = NULL;
            g_42b022 = 0;
            break;
        case 104:
            g_42b3bc = 176;
            g_42b3c8 = g_42b3bc;
            g_42b3c0 = 1;
            g_42b028 = g_42b3c0;
            break;
        case 105:
            g_42b3e9 = 1;
            break;
        case 107:
            v0 = 1;
            break;
        case 108:
            v1 = NULL;
            break;
        case 109:
            v1 = 0x4;
            break;
        case 110:
            g_42b3ba = 1;
            v1 = NULL;
            break;
        case 111:
            v1 = NULL;
            g_42b023 = 0;
            break;
        case 112:
            g_42b3d0 = 1;
            break;
        case 113:
            v2 = 1;
            break;
        case 114:
            g_42b3b8 = 1;
            break;
        case 115:
            g_42b3bb = 1;
            break;
        case 116:
            v4 = 0x5;
            break;
        case 117:
            g_42b3b0 = 2;
            break;
        case 118:
            v4 = 0x4;
            break;
        case 119:
            v14 = sub_406fe6(optarg);
            if (v14 - 0 >> 63 & 1)
            {
                sub_41aeca(optarg);
                gettext("invalid line width");
                error(2, 0, "%s: %s");
            }
            continue;
        case 120:
            v1 = 0x3;
            break;
        case 128:
            g_42b3b9 = 1;
            break;
        case 129:
            v11 = sub_4149fd(optarg, &g_42b3bc, &g_42b3c0, &g_42b3bc);
            if (v11)
                sub_41d130(v11, *((unsigned int *)v28), 0, &g_42a0c0, optarg); /* do not return */
            g_42b3c8 = g_42b3bc;
            g_42b028 = g_42b3c0;
            break;
        case 130:
            v6 = (!optarg ? 0x1 : g_420980[sub_40fe17("--color", optarg, &g_42a720, &g_420980[0], 4, g_42b220, 1)]);
            v6 = v6;
            if (v6 == 1 || v6 == 2 && (char)sub_40706a())
                v31 = 1;
            else
                v31 = 0;
            g_42b3d4 = v31 & 1;
            break;
        case 131:
            g_42b3ec = 3;
            break;
        case 132:
            g_42b3d0 = 2;
            break;
        case 133:
            v1 = (unsigned int *)g_4208a0[sub_40fe17("--format", optarg, &g_42a660, &g_4208a0[0], 4, g_42b220, 1)];
            break;
        case 134:
            v1 = NULL;
            v12 = sub_40523e("full-iso");
            break;
        case 135:
            g_42b3f2 = 1;
            break;
        case 136:
            v22 = sub_41c68b(16);
            v22->field_0 = optarg;
            v22->field_8 = g_42b400;
            g_42b400 = v22;
            break;
        case 137:
            v7 = (!optarg ? 0x1 : g_420980[sub_40fe17("--hyperlink", optarg, &g_42a720, &g_420980[0], 4, g_42b220, 1)]);
            v7 = v7;
            if (v7 == 1 || v7 == 2 && (char)sub_40706a())
                v32 = 1;
            else
                v32 = 0;
            g_42b3d5 = v32 & 1;
            break;
        case 138:
            g_42b3d0 = g_4205e0[sub_40fe17("--indicator-style", optarg, &g_429fa0, &g_4205e0[0], 4, g_42b220, 1)];
            break;
        case 139:
            v3 = (unsigned int *)g_4238c0[sub_40fe17("--quoting-style", optarg, &g_42a9e0, &g_4238c0[0], 4, g_42b220, 1)];
            break;
        case 140:
            v2 = 0;
            break;
        case 141:
            g_42b3bc = 144;
            g_42b3c8 = g_42b3bc;
            g_42b3c0 = 1;
            g_42b028 = g_42b3c0;
            break;
        case 142:
            v4 = (unsigned int *)g_4208d0[sub_40fe17("--sort", optarg, &g_42a6a0, &g_4208d0[0], 4, g_42b220, 1)];
            break;
        case 143:
            g_42b3b0 = g_420920[sub_40fe17("--time", optarg, &g_42a6e0, &g_420920[0], 4, g_42b220, 1)];
            break;
        case 144:
            v12 = optarg;
            break;
        case 145:
            g_42b030 = 0;
            v2 = 0;
            if (v1)
                v1 = 0x1;
            g_42b3d4 = 0;
            v3 = NULL;
            break;
        default:
            sub_40f135(2); /* do not return */
        }
    }
    if (!g_42b3c0)
    {
        v23 = getenv("LS_BLOCK_SIZE");
        sub_4149fd(v23, &g_42b3bc, &g_42b3c0, &g_42b3bc);
        if (v23 || getenv("BLOCK_SIZE"))
        {
            g_42b3c8 = g_42b3bc;
            g_42b028 = g_42b3c0;
        }
        if (v0)
        {
            g_42b3bc = 0;
            g_42b3c0 = 0x400;
        }
    }
    g_42b3ac = (v1 < NULL ? (*((int *)&g_42b210) == 1 ? (!(char)sub_40706a() ? 1 : 2) : (*((int *)&g_42b210) == 2 ? 2 : 0)) : v1);
    v15 = v14;
    if (g_42b3ac == 2 || g_42b3ac == 3 || g_42b3ac == 4 || g_42b3d4)
    {
        if (v15 - 0 >> 63 & 1 && (char)sub_40706a() && ioctl(1, 21523) >= 0 && *((unsigned short *)((void*)v28 + 2)))
            v15 = *((unsigned short *)((void*)v28 + 2));
        if (v15 - 0 >> 63 & 1)
        {
            v24 = getenv("COLUMNS");
            if (v24 && *(v24))
            {
                v15 = sub_406fe6(v24);
                if (v15 - 0 >> 63 & 1)
                {
                    sub_41aeca(v24);
                    error(0, 0, gettext("ignoring invalid width in environment variable COLUMNS: %s"));
                }
            }
        }
    }
    g_42b430 = (v15 - 0 >> 63 & 1 ? 80 : v15);
    g_42b4f8 = g_42b430 / 3;
    g_42b4f8 = g_42b4f8 + (g_42b430 % 3);
    if (g_42b3ac == 2 || g_42b3ac == 3 || g_42b3ac == 4)
    {
        if (!(v13 - 0 >> 63 & 1))
        {
            g_42b420 = v13;
        }
        else
        {
            g_42b420 = 8;
            v25 = getenv("TABSIZE");
            if (v25)
            {
                if (!sub_41d251(v25, 0, 0, v28, &g_41f27b))
                {
                    *((int [2])&g_42b420) = v28;
                }
                else
                {
                    sub_41aeca(v25);
                    error(0, 0, gettext("ignoring invalid tab size in environment variable TABSIZE: %s"));
                }
            }
        }
    }
    if (v2 < 0)
    {
        if (*((int *)&g_42b210) == 1 && (char)sub_40706a())
            v35 = 1;
        else
            v35 = 0;
        v36 = v35 & 1;
    }
    else
    {
        v36 = v2;
    }
    g_42b408 = v36;
    v8 = v3;
    if (v8 < 0)
        v8 = (unsigned int *)(unsigned int)sub_408cff();
    if (v8 < 0)
        v8 = (unsigned int *)(*((int *)&g_42b210) == 1 ? (!(char)sub_40706a() ? 0xffffffff : 3) : 0x7);
    if (v8 >= 0)
        sub_418fe7(0, v8);
    v8 = sub_418fc3(0);
    if ((!g_42b3ac || (g_42b3ac == 2 || g_42b3ac == 3) && g_42b430) && (v8 == 1 || v8 == 3 || v8 == 6))
        v38 = 1;
    else
        v38 = 0;
    g_42b341 = v38 & 1;
    g_42b410 = sub_418f74(0);
    if (v8 == 7)
        sub_419012(g_42b410, 32, 1);
    if (g_42b3d0 > 1)
    {
        for (i = (char *)(g_42b3d0 + 4328580); *(i); i += 1)
        {
            v39 = *(i);
            sub_419012(g_42b410, v39 & 0xffffffff, 1, v39);
        }
    }
    g_42b418 = sub_418f74(0);
    sub_419012(g_42b418, 58, 1);
    g_42b3cc = (g_42b3d5 ^ 1) & !g_42b3ac & g_42b3cc;
    if (g_42b030 < g_42b3cc)
        error(2, 0, gettext("--dired and --zero are incompatible"));
    if (v4 >= 0)
    {
        v41 = v4;
    }
    else if (g_42b3ac && (g_42b3b0 == 1 || g_42b3b0 == 2 || g_42b3b0 == 3))
    {
        v41 = 0x5;
    }
    else
    {
        v41 = NULL;
    }
    g_42b3b4 = (unsigned int)v41;
    if (g_42b3ac)
        return optind;
    j = v12;
    if (!j)
    {
        j = getenv("TIME_STYLE");
        if (!j)
            j = sub_40523e("locale");
    }
    for (; !strncmp(j, "posix-", 6); j += 6)
    {
        if ((char)sub_411f2c(2) != 1)
            return optind;
    }
    if (*(j) == 43)
    {
        v27 = j + 1;
        ptr = strchr(v27, 10);
        if (!ptr)
        {
            v19 = v27;
        }
        else if (strchr(ptr + 1, 10))
        {
            sub_41aeca(v27);
            error(2, 0, gettext("invalid time style format %s"));
        }
        else
        {
            v19 = ptr + 1;
            *(ptr) = 0;
        }
        g_42b200[0] = v27;
        g_42b208 = v19;
    }
    else
    {
        v26 = sub_40fa7e(j, &g_429f60, &g_4205b0, 4);
        if (v26 - 0 >> 63 & 1)
        {
            sub_40fc27("time style", j, v26);
            v43 = gettext("Valid arguments are:\n");
            fputs_unlocked(v43, stderr);
            v20 = &g_429f60;
            while (1)
            {
                v21 = v20;
                if (!*(v21))
                    break;
                v20 = v21 + 1;
                fprintf(stderr, "  - [posix-]%s\n", *(v21));
            }
            v44 = gettext("  - +FORMAT (e.g., +%H:%M) for a 'date'-style format\n");
            fputs_unlocked(v44, stderr);
            sub_40f135(2); /* do not return */
        }
        else if (v26 != 3)
        {
            if (v26 <= 3)
            {
                if (v26 == 2)
                {
                    g_42b200[0] = "%Y-%m-%d ";
                    g_42b208 = "%m-%d %H:%M";
                }
                else if (v26 <= 2)
                {
                    if (!v26)
                    {
                        g_42b208 = "%Y-%m-%d %H:%M:%S.%N %z";
                        g_42b200[0] = g_42b208;
                    }
                    else if (v26 == 1)
                    {
                        g_42b208 = "%Y-%m-%d %H:%M";
                        g_42b200[0] = g_42b208;
                    }
                }
            }
        }
        else
        {
            if ((char)sub_411f2c(2))
            {
                for (k = 0; k <= 1; k += 1)
                {
                    g_42b200[k] = dcgettext(NULL, g_42b200[k], 2);
                }
            }
        }
    }
    sub_405f43();
    return optind;
}



// Function: get_funky_string @ 0x842f

long long get_funky_string(void* *a0, void* *a1, char a2, unsigned long long *a3)
{
    unsigned int v6;  // eax
    void* v7;  // rdx
    void* v16;  // rax
    void* v17;  // rax
    void* v18;  // rax
    void* v19;  // rax
    void* v20;  // rax
    void* v8;  // rax
    unsigned int v9;  // eax
    void* v10;  // rax
    void* v11;  // rax
    void* v12;  // rax
    unsigned int v13;  // eax
    void* v14;  // rax
    void* v15;  // rax
    char v0;  // [bp-0x25]
    unsigned int i;  // [bp-0x24]
    unsigned long long node;  // [bp-0x20]
    void* iter;  // [bp-0x18]
    void* v4;  // [bp-0x10]

    iter = *(a1);
    v4 = *(a0);
    node = 0;
    v0 = 0;
    i = 0;
    while (i <= 4)
    {
        switch (i)
        {
        case 0:
            v6 = *((char *)iter);
            switch (v6)
            {
            case 94:
                i = 4;
                iter += 1;
                break;
            case 92:
                i = 1;
                iter += 1;
                break;
            case 61:
                if (a2)
                {
                    i = 5;
                    break;
                }
                break;
            case 0: case 58:
                i = 5;
                break;
            default:
                v7 = iter;
                iter = v7 + 1;
                v8 = v4;
                v4 = v8 + 1;
                *((char *)v8) = *((char *)v7);
                node += 1;
                continue;
            }
        case 1:
            v9 = *((char *)iter);
            if (v9)
            {
                if (v9 < 0)
                    goto LABEL_4085f8;
                if (v9 > 120)
                    goto LABEL_4085f8;
                if (v9 < 48)
                    goto LABEL_4085f8;
                switch (v9)
                {
                case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55:
                    i = 2;
                    v0 = *((char *)iter) - 48;
                    break;
                case 63:
                    v0 = 127;
                    break;
                case 88: case 120:
                    i = 3;
                    v0 = 0;
                    break;
                case 95:
                    v0 = 32;
                    break;
                case 97:
                    v0 = 7;
                    break;
                case 98:
                    v0 = 8;
                    break;
                case 101:
                    v0 = 27;
                    break;
                case 102:
                    v0 = 12;
                    break;
                case 110:
                    v0 = 10;
                    break;
                case 114:
                    v0 = 13;
                    break;
                case 116:
                    v0 = 9;
                    break;
                case 118:
                    v0 = 11;
                    break;
                default:
LABEL_4085f8:
                    v0 = *((char *)iter);
                    break;
                }
            }
            else
            {
                i = 6;
            }
            if (i == 1)
            {
                v10 = v4;
                v4 = v10 + 1;
                *((char *)v10) = v0;
                node += 1;
                i = 0;
            }
            iter += 1;
            continue;
        case 2:
            if (*((char *)iter) <= 47 || *((char *)iter) > 55)
            {
                v12 = v4;
                v4 = v12 + 1;
                *((char *)v12) = v0;
                node += 1;
                i = 0;
            }
            else
            {
                v11 = iter;
                iter = v11 + 1;
                v0 = *((char *)v11) + v0 * 8 - 48;
            }
            continue;
        case 3:
            v13 = *((char *)iter);
            if (v13 <= 0x66)
            {
                if (v13 >= 97)
                {
                    v16 = iter;
                    iter = v16 + 1;
                    v0 = *((char *)v16) + v0 * 16 - 87;
                    break;
                }
                if (v13 <= 57)
                {
                    if (v13 >= 48)
                    {
                        v14 = iter;
                        iter = v14 + 1;
                        v0 = *((char *)v14) + v0 * 16 - 48;
                        break;
                    }
                }
                else
                {
                    if (v13 - 65 <= 5)
                    {
                        v15 = iter;
                        iter = v15 + 1;
                        v0 = *((char *)v15) + v0 * 16 - 55;
                        break;
                    }
                }
            }
            v17 = v4;
            v4 = v17 + 1;
            *((char *)v17) = v0;
            node += 1;
            i = 0;
            continue;
        case 4:
            i = 0;
            if (*((char *)iter) > 63 && *((char *)iter) != 127)
            {
                v18 = iter;
                iter = v18 + 1;
                v19 = v4;
                v4 = v19 + 1;
                *((char *)v19) = *((char *)v18) & 31;
                node += 1;
            }
            else if (*((char *)iter) == 63)
            {
                v20 = v4;
                v4 = v20 + 1;
                *((char *)v20) = 127;
                node += 1;
                continue;
            }
            else
            {
                i = 6;
            }
            continue;
        default:
            abort(); /* do not return */
        }
    }
    *(a0) = v4;
    *(a1) = iter;
    *(a3) = node;
    return _INSERT(a3, 0, i != 6);
}



// Function: known_term_type @ 0x87ec
unsigned int known_term_type(void)
{
    char *i;  // [bp-0x18]
    char *v1;  // [bp-0x10]

    v1 = getenv("TERM");
    if (v1 && *(v1))
    {
        for (i = "# Configuration file for dircolors, a utility to help you set the"; i - "# Configuration file for dircolors, a utility to help you set the" <= 0x12c7; i = &i[1 + strlen(i)])
        {
            if (!strncmp(i, "TERM ", 5) && !fnmatch(i + 5, v1, 0))
                return 1;
        }
        return 0;
    }
    return 0;
}



// Function: parse_ls_color @ 0x88ab
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

extern unsigned long long g_429fe0[4];
extern char g_42b080;
extern unsigned long long g_42b088;
extern unsigned long long g_42b0f0;
extern char *g_42b0f8;
extern char g_42b358;
extern char g_42b3d4;
extern struct_0 *g_42b3d8;
extern void* g_42b3e0;

void parse_ls_color(void)
{
    unsigned int v11;  // eax
    char *v13;  // rax
    char *v14;  // rax
    char *v15;  // rax
    char *v18;  // rax
    int v0;  // [bp-0x60]
    unsigned int v1;  // [bp-0x5c]
    char *node;  // [bp-0x58]
    void* v3;  // [bp-0x50]
    unsigned long long *idx;  // [bp-0x48]
    struct_0 *iter;  // [bp-0x40]
    struct_0 *ptr;  // [bp-0x38]
    char *v7;  // [bp-0x30]
    char v8;  // [bp-0x23]
    char v9;  // [bp-0x22]

    node = getenv("LS_COLORS");
    if (node && *(node))
    {
        idx = NULL;
        strcpy(&v8, "??");
        g_42b3e0 = sub_41cc92(node);
        v3 = g_42b3e0;
        v1 = 1;
        while (1)
        {
            switch (v1)
            {
            case 1:
                v11 = *(node);
                if (v11 == 58)
                {
                    node += 1;
                }
                else if (v11 <= 58)
                {
                    if (!v11)
                    {
                        v1 = 5;
                        goto LABEL_408c36;
                    }
                    if (v11 != 42)
                        goto LABEL_408a5c;
                    idx = sub_41c68b(40);
                    idx[4] = g_42b3d8;
                    g_42b3d8 = idx;
                    node += 1;
                    idx[1] = v3;
                    v1 = (!(char)sub_40842f(&v3, &node, 1, idx) ? 6 : 4);
                }
                else
                {
LABEL_408a5c:
                    v13 = node;
                    node = v13 + 1;
                    v8 = *(v13);
                    v1 = 2;
                }
                break;
            case 2:
                if (*(node))
                {
                    v14 = node;
                    node = v14 + 1;
                    v9 = *(v14);
                    v1 = 3;
                    break;
                }
                else
                {
                    v1 = 6;
                    break;
                }
            case 3:
                v1 = 6;
                v15 = node;
                node = v15 + 1;
                if (*(v15) == 61)
                {
                    for (v0 = 0; g_429fe0[v0]; v0 += 1)
                    {
                        if (!strcmp(&v8, g_429fe0[v0]))
                        {
                            (&g_42b088)[2 * v0] = v3;
                            v1 = (!(char)sub_40842f(&v3, &node, 0, &(&g_42b080)[16 * v0]) ? 6 : 1);
                            break;
                        }
                    }
                    if (v1 == 6)
                    {
                        sub_41aeca(&v8);
                        error(0, 0, gettext("unrecognized prefix: %s"));
                    }
                }
                break;
            case 4:
                v18 = node;
                node = v18 + 1;
                if (*(v18) == 61)
                {
                    idx[3] = v3;
                    v1 = (!(char)sub_40842f(&v3, &node, 0, idx + 2) ? 6 : 1);
                    break;
                }
                else
                {
                    v1 = 6;
                    break;
                }
            case 6:
                goto LABEL_408c36;
            default:
                abort(); /* do not return */
            }
        }
LABEL_408c36:
        if (v1 == 6)
        {
            error(0, 0, gettext("unparsable value for LS_COLORS environment variable"));
            free(g_42b3e0);
            iter = g_42b3d8;
            while (iter)
            {
                ptr = iter;
                iter = iter->field_20;
                free(ptr);
            }
            g_42b3d4 = 0;
        }
        if (g_42b0f0 != 6)
        {
            return;
        }
        else if (!strncmp(g_42b0f8, "target", 6))
        {
            g_42b358 = 1;
            return;
        }
        else
        {
            return;
        }
    }
    v7 = getenv("COLORTERM");
    if ((!v7 || !*(v7)) && (char)sub_4087ec() != 1)
    {
        g_42b3d4 = 0;
        return;
    }
    return;
}



// Function: getenv_quoting_style @ 0x8cff
extern unsigned int g_4238c0[4];
extern long long g_42a9e0;

unsigned int * getenv_quoting_style(void)
{
    unsigned int v0;  // [bp-0x24]
    unsigned long v1;  // [bp-0x20]

    v1 = getenv("QUOTING_STYLE");
    if (!v1)
        return 0xffffffff;
    v0 = sub_40fa7e(v1, &g_42a9e0, &g_4238c0[0], 4);
    if (v0 >= 0)
        return g_4238c0[v0];
    sub_41aeca(v1);
    error(0, 0, gettext("ignoring invalid value of environment variable QUOTING_STYLE: %s"));
    return 0xffffffff;
}



// Function: set_exit_status @ 0x8db1
extern unsigned int g_42b4e8;

long long set_exit_status(unsigned int a0)
{
    unsigned long v2;  // rax
    char v0;  // [bp-0xc]

    v0 = a0;
    if (v0)
    {
        g_42b4e8 = 2;
        return a0;
    }
    v2 = g_42b4e8;
    if ((unsigned int)v2)
        return v2;
    g_42b4e8 = 1;
    return v2;
}



// Function: file_failure @ 0x8de7
long long file_failure(char a0, char *a1, long long a2)
{
    int *err;  // rax

    sub_41aa83(4, a2);
    err = __errno_location();
    error(0, *(err), a1);
    return sub_408db1(a0);
}



// Function: queue_directory @ 0x8e46
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char field_10;
    char padding_11[7];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_42b368;

struct_0 * queue_directory(long long a0, long long a1, char a2)
{
    struct_0 *idx;  // [bp-0x10]

    idx = sub_41c68b(32);
    idx->field_8 = (!a1 ? 0 : sub_41cc92(a1));
    idx->field_0 = (!a0 ? 0 : sub_41cc92(a0));
    idx->field_10 = a2;
    idx->field_18 = g_42b368;
    g_42b368 = idx;
    return idx;
}



// Function: print_dir @ 0x8ede
typedef struct DIR {
} DIR;

extern char g_42b030;
extern char g_42b032;
extern unsigned long long g_42b320;
extern unsigned long long g_42b338;
extern unsigned int g_42b3ac;
extern unsigned int g_42b3b4;
extern char g_42b3bb;
extern unsigned int g_42b3bc;
extern long long g_42b3c0;
extern char g_42b3d5;
extern char g_42b3f0;
extern char g_42b428;

void print_dir(unsigned long a0, unsigned long long a1, char a2)
{
    long long v24;  // r9
    long long v19;  // rdi
    long long v20;  // rsi
    long long v21;  // rdx
    long long v22;  // rcx
    long long v23;  // r8
    void* v0;  // [bp-0x3b8]
    unsigned int v1;  // [bp-0x380]
    unsigned int v2;  // [bp-0x37c]
    long long v3;  // [bp-0x378]
    void* v4;  // [bp-0x370]
    DIR *ptr;  // [bp-0x368]
    unsigned long count;  // [bp-0x360]
    void* iter;  // [bp-0x358]
    char *v8;  // [bp-0x350]
    char *v9;  // [bp-0x350]
    unsigned long long v10;  // [bp-0x348]
    unsigned int v11;  // [bp-0x340]
    char v12;  // [bp-0x2b7]

    v3 = 0;
    *(__errno_location()) = 0;
    ptr = opendir(a0);
    if (!ptr)
    {
        sub_408de7(a2, gettext("cannot open directory %s"), a0);
        return;
    }
    if (g_42b320)
    {
        v2 = dirfd(ptr);
        if ((v2 < 0 ? (char)((sub_405cb8(a0, &v10) & 0xffffffff) >> 31) : (char)((sub_405cee(v2, &v10) & 0xffffffff) >> 31)))
        {
            sub_408de7(a2, gettext("cannot determine device and inode of %s"), a0);
            closedir(ptr);
            return;
        }
        if ((char)sub_406262(v10, v11))
        {
            sub_41ac15(0, 3, a0);
            error(0, 0, gettext("%s: not listing already-listed directory"));
            closedir(ptr);
            sub_408db1(1);
            return;
        }
        sub_40558e(v10, v11);
    }
    sub_4096f3();
    if (g_42b3f0 || g_42b428)
    {
        if (g_42b032 != 1)
            sub_405422(10);
        g_42b032 = 0;
        sub_4054cf();
        v4 = NULL;
        if (g_42b3d5)
        {
            v4 = sub_41101b(a0, 2);
            if (!v4)
                sub_408de7(a2, gettext("error canonicalizing %s"), a0);
        }
        v0 = v4;
        sub_40d87f((!a1 ? a0 : a1));
        free(v4);
        sub_40549e(":\n");
    }
    while (1)
    {
        *(__errno_location()) = 0;
        count = readdir(ptr);
        if (!count)
        {
            if (!*(__errno_location()))
                break;
            sub_408de7(a2, gettext("reading directory %s"), a0);
            if (*(__errno_location()) != 75)
                break;
        }
        else if ((char)sub_40958d(count + 19) != 1)
        {
            v1 = 0;
            switch (*((char *)(count + 18)))
            {
            case 1:
                v1 = 1;
                break;
            case 2:
                v1 = 2;
                break;
            case 4:
                v1 = 3;
                break;
            case 6:
                v1 = 4;
                break;
            case 8:
                v1 = 5;
                break;
            case 10:
                v1 = 6;
                break;
            case 12:
                v1 = 7;
                break;
            case 14:
                v1 = 8;
                break;
            }
            v3 += sub_409a40(count + 19, v1, 0, 0, a0);
            if (g_42b3ac == 1 && g_42b3b4 == 6 && g_42b3bb != 1 && g_42b3f0 != 1)
            {
                sub_40bfc5();
                sub_40c12d(v19, v20, v21, v22, v23, v24);
                sub_4096f3(v19, v20, v21, v22, v23, v24);
            }
        }
        sub_40648c();
    }
    if (closedir(ptr))
        sub_408de7(a2, gettext("closing directory %s"), a0);
    sub_40bfc5();
    if (g_42b3f0)
        sub_40aa65(a0, 0);
    if (!g_42b3ac || g_42b3bb)
    {
        iter = sub_413d91(v3, &v12, g_42b3bc, 0x200, g_42b3c0);
        v8 = strlen(iter) + iter;
        iter -= 1;
        *((char *)iter) = 32;
        v9 = v8 + 1;
        *(v8) = g_42b030;
        sub_4054cf();
        sub_40549e(gettext("total"));
        sub_405453(iter, v9 - iter);
    }
    if (!g_42b338)
        return;
    sub_40c12d();
    return;
}



// Function: add_ignore_pattern @ 0x94e9
extern unsigned long long g_42b3f8;

unsigned long long * add_ignore_pattern(unsigned long a0)
{
    unsigned long long *v0;  // [bp-0x10]

    v0 = sub_41c68b(16);
    *(v0) = a0;
    v0[1] = g_42b3f8;
    g_42b3f8 = v0;
    return v0;
}



// Function: patterns_match @ 0x952f
typedef struct struct_0 {
    char *field_0;
    struct struct_0 *field_8;
} struct_0;

unsigned int patterns_match(struct_0 *a0, char *a1)
{
    struct_0 *iter;  // [bp-0x10]

    for (iter = a0; iter; iter = iter->field_8)
    {
        if (!fnmatch(iter->field_0, a1, 4))
            return 1;
    }
    return 0;
}



// Function: file_ignored @ 0x958d
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_42b3f4;
extern struct_0 *g_42b3f8;
extern struct_0 *g_42b400;

unsigned int file_ignored(char *a0)
{
    unsigned int v1;  // eax

    if (g_42b3f4 != 2 && *(a0) == 46 && (!g_42b3f4 || !a0[(a0[1] == 46 ? 2 : 1)]) || !g_42b3f4 && (char)sub_40952f(g_42b400, a0) || (char)sub_40952f(g_42b3f8, a0))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: unsigned_file_size @ 0x9635
long long unsigned_file_size(unsigned long a0)
{
    return a0;
}



// Function: has_capability @ 0x9647
unsigned int has_capability(unsigned long long a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    *(__errno_location()) = 95;
    return 0;
}



// Function: free_ent @ 0x9669

long long free_ent(void* a0)
{
    free(*((long long *)a0));
    free((long long)a0[8]);
    free((long long)a0[16]);
    if ((long long)a0[176] == "?")
        return (long long)a0[176];
    return (!(char)sub_40525c() ? freecon((long long)a0[176]) : (unsigned long long)free((long long)a0[176]));
}



// Function: clear_files @ 0x96f3
extern unsigned long long g_42b338;
extern char g_42b340;
extern unsigned long long g_42b348;
extern char g_42b381;
extern unsigned int g_42b384;
extern unsigned int g_42b388;
extern unsigned int g_42b38c;
extern unsigned int g_42b390;
extern unsigned int g_42b394;
extern unsigned int g_42b398;
extern unsigned int g_42b39c;
extern unsigned int g_42b3a0;
extern unsigned int g_42b3a4;
extern unsigned int g_42b3a8;

unsigned long long clear_files(void)
{
    unsigned long long v3;  // rax
    unsigned long long idx;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    idx = 0;
    while (1)
    {
        if (idx >= g_42b338)
            break;
        v1 = *((long long *)(idx * 8 + g_42b348));
        sub_409669(v1);
        idx += 1;
    }
    g_42b338 = 0;
    g_42b340 = 0;
    g_42b381 = 0;
    g_42b384 = 0;
    g_42b388 = 0;
    g_42b38c = 0;
    g_42b394 = 0;
    g_42b398 = 0;
    g_42b39c = 0;
    g_42b390 = 0;
    g_42b3a0 = 0;
    g_42b3a4 = 0;
    g_42b3a8 = 0;
    return v3;
}



// Function: errno_unsupported @ 0x97c0
unsigned int errno_unsupported(unsigned int a0)
{
    unsigned int v1;  // eax

    if (a0 == 22 || a0 == 38 || (char)sub_40524c(a0))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: getfilecon_cache @ 0x97fa
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_42c428;

int getfilecon_cache(long long a0, struct_0 *a1, char a2)
{
    int v0;  // [bp-0xc], Other Possible Types: unsigned int

    if (a1->field_18 == g_42c428)
    {
        *(__errno_location()) = 95;
        return -0x1;
    }
    v0 = 0;
    v0 = (!a2 ? (unsigned int)sub_41b268(a0, &a1[5].padding_0[16], &a1[5].padding_0[16]) : (unsigned int)sub_41b22b(a0, &a1[5].padding_0[16], &a1[5].padding_0[16]));
    if (v0 >= 0 || !(char)sub_4097c0(*(__errno_location())))
        return v0;
    g_42c428 = a1->field_18;
    return v0;
}



// Function: file_has_acl_cache @ 0x98ab
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_42c430;

unsigned int file_has_acl_cache(long long a0, struct_0 *a1)
{
    unsigned int v0;  // [bp-0xc]

    if (a1->field_18 == g_42c430)
    {
        *(__errno_location()) = 95;
        return 0;
    }
    *(__errno_location()) = 0;
    v0 = sub_4112c6(a0, &a1->field_18, &a1->field_18);
    if (v0 > 0 || !(char)sub_4097c0(*(__errno_location())))
        return v0;
    g_42c430 = a1->field_18;
    return v0;
}



// Function: has_capability_cache @ 0x9936
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_42c438;

unsigned int has_capability_cache(unsigned long long a0, struct_0 *a1)
{
    char v0;  // [bp-0x9]

    if (a1->field_18 == g_42c438)
    {
        *(__errno_location()) = 95;
        return 0;
    }
    v0 = sub_409647(a0);
    if (v0 == 1 || !(char)sub_4097c0(*(__errno_location())))
        return v0;
    g_42c438 = a1->field_18;
    return v0;
}



// Function: needs_quoting @ 0x99b1
extern long long g_42b410;

long long needs_quoting(char *a0)
{
    unsigned int v3;  // eax
    unsigned long v0;  // [bp-0x20]
    char v1;  // [bp-0x12]

    v0 = sub_41a389(&v1, 2, a0, -0x1, g_42b410);
    if (*(a0) != v1 || v0 != strlen(a0))
        v3 = 1;
    else
        v3 = 0;
    return v3 & 1;
}



// Function: gobble_file @ 0x9a40
extern char g_42b022;
extern char g_42b023;
extern long long g_42b028;
extern long long g_42b328;
extern unsigned long long g_42b330;
extern long long g_42b338;
extern char g_42b340;
extern char g_42b341;
extern char g_42b358;
extern char g_42b380;
extern char g_42b381;
extern unsigned int g_42b384;
extern unsigned int g_42b388;
extern unsigned int g_42b38c;
extern unsigned int g_42b390;
extern unsigned int g_42b394;
extern unsigned int g_42b398;
extern unsigned int g_42b39c;
extern unsigned int g_42b3a0;
extern unsigned int g_42b3a4;
extern unsigned int g_42b3a8;
extern unsigned int g_42b3ac;
extern char g_42b3b9;
extern char g_42b3bb;
extern unsigned int g_42b3bc;
extern long long g_42b3c0;
extern unsigned int g_42b3c8;
extern unsigned int g_42b3d0;
extern char g_42b3d4;
extern char g_42b3d5;
extern char g_42b3e8;
extern char g_42b3e9;
extern char g_42b3ec;
extern char g_42b3f1;
extern char g_42b440;
extern char g_42b441;

long long gobble_file(unsigned long a0, unsigned int a1, unsigned long long a2, char a3, char *a4)
{
    unsigned long len;  // rax
    unsigned long long v29;  // rax
    char v31;  // al
    unsigned int v33;  // eax
    int *err;  // rax
    unsigned int v35;  // eax
    unsigned int v36;  // eax
    char v0;  // [bp-0x13e8]
    char *v1;  // [bp-0x3e8]
    char v2;  // [bp-0x3c4]
    char v3;  // [bp-0x3c3]
    char v4;  // [bp-0x3c2]
    char v5;  // [bp-0x3c1]
    unsigned int v6;  // [bp-0x3c0]
    unsigned int v7;  // [bp-0x3bc]
    unsigned int v8;  // [bp-0x3b8]
    unsigned int v9;  // [bp-0x3b4]
    unsigned int v10;  // [bp-0x3b0]
    unsigned int v11;  // [bp-0x3ac]
    unsigned int v12;  // [bp-0x3a8]
    unsigned int v13;  // [bp-0x3a4]
    unsigned int v14;  // [bp-0x3a0]
    unsigned int v15;  // [bp-0x39c]
    unsigned int v16;  // [bp-0x398]
    unsigned int v17;  // [bp-0x394]
    long long v18;  // [bp-0x390]
    unsigned long long v19;  // [bp-0x388]
    void* ptr;  // [bp-0x380]
    unsigned long v21;  // [bp-0x378]
    long long v22;  // [bp-0x370]
    unsigned long long v23[3];  // [bp-0x368]
    unsigned int v24;  // [bp-0x350]
    char v25;  // [bp-0x2d8]
    char v26;  // [bp-0x2b8]

    v1 = a4;
    v18 = 0;
    if (a3 == 1 && a2)
        __assert_fail(); /* do not return */
    if (g_42b338 == g_42b330)
    {
        g_42b328 = sub_41c638(g_42b328, g_42b330, 416, g_42b330);
        g_42b330 = g_42b330 * 2;
    }
    ptr = g_42b338 * 208 + g_42b328;
    memset(ptr, 0, 208);
    *((unsigned long long *)&ptr[32]) = a2;
    *((unsigned int *)&ptr[168]) = a1;
    *((unsigned int *)&ptr[196]) = 0xffffffff;
    if (g_42b340 != 1 && g_42b341)
    {
        *((unsigned int *)&ptr[196]) = (char)sub_4099b1(a0);
        if ((int)ptr[196])
            g_42b340 = 1;
    }
    if (a3 || g_42b3d5 || g_42b440 || a1 == 3 && g_42b3d4 && ((char)sub_406325(19) || (char)sub_406325(18) || (char)sub_406325(20)) || (g_42b3e9 || g_42b441) && (a1 == 6 || !a1) && (*((int *)&g_42b3ec) == 4 || g_42b358 || g_42b3e8) || g_42b3e9 && !a2 || g_42b441 && (!a1 || a3 || a1 == 5 && (g_42b3d0 == 3 || g_42b3d4 && ((char)sub_406325(14) || (char)sub_406325(16) || (char)sub_406325(0x11) || (char)sub_406325(21)))))
    {
        if (*((char *)a0) == 47 || !*(v1))
        {
            v19 = a0;
        }
        else
        {
            len = strlen(a0);
            v29 = (unsigned long long)((15 + strlen(v1) + len + 10) / 16) * 16;
            while (&v0 != (char *)&v1 - (v29 & 0xfffffffffffff000))
            ;
            /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v29 & 0xfff)];
            if ((unsigned short)v29 & 0xfff)
                *((long long *)(((unsigned int)v29 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v29 & 0xfff) - 8 + /* unsupported instruction */));
            v19 = (/* unsupported instruction */ + 15 >> 4) * 16;
            sub_40eb94(v19, v1, a0, v1);
        }
        if (g_42b3d5)
        {
            *((unsigned long long *)&ptr[16]) = sub_41101b(v19, 2);
            if (!(long long)ptr[16])
                sub_408de7(a3, gettext("error canonicalizing %s"), v19);
        }
        if (*((int *)&g_42b3ec) <= 3)
        {
            if (*((int *)&g_42b3ec) < 2 || !a3 || (v6 = (unsigned int)(int)sub_405c0e(v19, ptr + 24), v2 = 1, *((int *)&g_42b3ec) != 2 && !(v31 = (v6 < 0 ? (char)(char)(*(__errno_location()) == 2) : (char)(char)(((int)ptr[48] & 0xf000) != 0x4000)), v5 = v31, v5 != 1)))
                goto LABEL_409f7c;
        }
        else
        {
            if (*((int *)&g_42b3ec) == 4)
            {
                v6 = sub_405c0e(v19, ptr + 24);
                v2 = 1;
            }
            else
            {
LABEL_409f7c:
                v6 = sub_405c48(v19, ptr + 24);
                v2 = 0;
            }
        }
        if (v6)
        {
            sub_408de7(a3, gettext("cannot access %s"), v19);
            *((char **)&ptr[176]) = "?";
            if (!a3)
            {
                *((unsigned long long *)ptr) = sub_41cc92(a0);
                g_42b338 = g_42b338 + 1;
                return 0;
            }
            return 0;
        }
        else
        {
            *((char *)&ptr[184]) = 1;
            if ((a1 == 5 || ((short)(int)ptr[48] & 0xf000) == 0x8000) && g_42b3d4 && (char)sub_406325(21))
                *((char *)&ptr[192]) = sub_409936(v19, ptr);
            if (!g_42b3ac || g_42b380)
            {
                v3 = 0;
                v4 = 0;
                v7 = sub_4097fa(v19, ptr, v2);
                v6 = v7 >> 31 & 0xff;
                if (!v6)
                {
                    v3 = (!(char)sub_40525c() ? strcmp("unlabeled", (long long)ptr[176]) : strcmp("_", (long long)ptr[176]));
                }
                else
                {
                    *((char **)&ptr[176]) = "?";
                    if ((char)sub_40524c(*(__errno_location())) || *(__errno_location()) == 61)
                        v6 = 0;
                }
                if (!v6 && !g_42b3ac)
                {
                    v8 = sub_4098ab(v19, ptr);
                    v6 = v8 >> 31 & 0xff;
                    v4 = 0 < v8;
                }
                if (!(v3 || v4))
                {
                    v33 = 0;
                }
                else if (v3 && v4 != 1)
                {
                    v33 = 1;
                }
                else
                {
                    v33 = 2;
                }
                *((unsigned int *)&ptr[188]) = v33;
                g_42b381 = (int)ptr[188] || g_42b381;
                if (v6)
                {
                    sub_41ac15(0, 3, v19);
                    err = __errno_location();
                    error(0, *(err), "%s");
                }
            }
            if (((short)(int)ptr[48] & 0xf000) == 0xa000 && (!g_42b3ac || g_42b3e8))
            {
                sub_40a8f5(v19, ptr, a3, ptr);
                v21 = sub_40a964(v19, (long long)ptr[8], (long long)ptr[8]);
                if (v21 && !(int)ptr[196] && (char)sub_4099b1((long long)ptr[8]))
                    *((unsigned int *)&ptr[196]) = 0xffffffff;
                if (v21 && (g_42b3d0 > 1 || g_42b3e8) && !(int)sub_405c82(v21, v23))
                {
                    *((char *)&ptr[185]) = 1;
                    *((unsigned int *)&ptr[172]) = v24;
                }
                free(v21);
            }
            if (((short)(int)ptr[48] & 0xf000) == 0xa000)
            {
                *((unsigned int *)&ptr[168]) = 6;
            }
            else if (((short)(int)ptr[48] & 0xf000) != 0x4000)
            {
                *((unsigned int *)&ptr[168]) = 5;
            }
            else if (a3 && g_42b3f1 != 1)
            {
                *((unsigned int *)&ptr[168]) = 9;
            }
            else
            {
                *((unsigned int *)&ptr[168]) = 3;
            }
            v18 = (long long)ptr[88];
            if (!g_42b3ac || g_42b3bb)
            {
                v9 = sub_41574e(sub_413d91(v18, &v26, g_42b3bc, 0x200, g_42b3c0), 0);
                if (v9 > g_42b388)
                    g_42b388 = v9;
            }
            if (!g_42b3ac)
            {
                if (g_42b022)
                {
                    v10 = sub_40c5c3((int)ptr[52]);
                    if (v10 > g_42b394)
                        g_42b394 = v10;
                }
                if (g_42b023)
                {
                    v11 = sub_40c603((int)ptr[56]);
                    if (v11 > g_42b398)
                        g_42b398 = v11;
                }
                if (g_42b3b9)
                {
                    v12 = sub_40c5c3((int)ptr[52]);
                    if (v12 > g_42b39c)
                        g_42b39c = v12;
                }
            }
            if (g_42b380)
            {
                v13 = strlen((long long)ptr[176]);
                if (v13 > g_42b390)
                    g_42b390 = v13;
            }
            if (!g_42b3ac)
            {
                v14 = strlen(sub_415035((long long)ptr[40], &v25, &v25));
                if (v14 > g_42b38c)
                    g_42b38c = v14;
                if (((short)(int)ptr[48] & 0xf000) == 0x2000 || ((short)(int)ptr[48] & 0xf000) == 0x6000)
                {
                    v35 = gnu_dev_major((long long)ptr[64]);
                    v16 = strlen(sub_415035(v35, &v26, v35));
                    if (v16 > g_42b3a0)
                        g_42b3a0 = v16;
                    v36 = gnu_dev_minor((long long)ptr[64]);
                    v16 = strlen(sub_415035(v36, &v26, v36));
                    if (v16 > g_42b3a4)
                        g_42b3a4 = v16;
                    v16 = g_42b3a4 + g_42b3a0 + 2;
                    if (v16 > g_42b3a8)
                        g_42b3a8 = v16;
                }
                else
                {
                    v22 = sub_409635((long long)ptr[72]);
                    v15 = sub_41574e(sub_413d91(v22, &v26, g_42b3c8, 1, g_42b028), 0);
                    if (v15 > g_42b3a8)
                        g_42b3a8 = v15;
                }
            }
        }
    }
    if (g_42b3e9)
    {
        v17 = strlen(sub_415035((long long)ptr[32], &v26, &v26));
        if (v17 > g_42b384)
            g_42b384 = v17;
    }
    *((unsigned long long *)ptr) = sub_41cc92(a0);
    g_42b338 = g_42b338 + 1;
    return v18;
}



// Function: is_directory @ 0xa869
typedef struct struct_0 {
    char padding_0[168];
    unsigned int field_a8;
} struct_0;

unsigned int is_directory(struct_0 *a0)
{
    unsigned int v1;  // eax

    if (a0->field_a8 == 3 || a0->field_a8 == 9)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: is_linked_directory @ 0xa8a4
unsigned int is_linked_directory(unsigned int *a0)
{
    unsigned int v1;  // eax

    if (a0[42] == 3 || a0[42] == 9 || ((unsigned short)a0[43] & 0xf000) == 0x4000)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: get_link_name @ 0xa8f5
long long get_link_name(long long a0, long long *a1, char a2)
{
    a1[1] = sub_40f7a0(a0, a1[9], a1[9]);
    if (a1[1])
        return a1[1];
    return sub_408de7(a2, gettext("cannot read symbolic link %s"), a0);
}



// Function: make_link_name @ 0xa964
char * make_link_name(unsigned long a0, char *a1)
{
    unsigned long v0;  // [bp-0x18]
    char *v1;  // [bp-0x10]

    if (!a1)
    {
        return NULL;
    }
    else if (*(a1) == 47)
    {
        return sub_41cc92(a1);
    }
    else
    {
        v0 = sub_4111a6(a0);
        if (!v0)
            return sub_41cc92(a1);
        v1 = sub_41c68b(strlen(a1) + v0 + 2);
        if (*((char *)(a0 + v0 - 1)) != 47)
            v0 += 1;
        stpcpy(stpncpy(v1, a0, v0), a1);
        return v1;
    }
}



// Function: basename_is_dot_or_dotdot @ 0xaa37
long long basename_is_dot_or_dotdot(long long a0)
{
    long long v0;  // [bp-0x10]

    v0 = sub_40ff56(a0);
    return sub_404ec1(v0);
}



// Function: extract_dirs_from_files @ 0xaa65
typedef struct struct_0 {
    char padding_0[168];
    unsigned int field_a8;
} struct_0;

typedef struct struct_1 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    char padding_10[152];
    unsigned int field_a8;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern unsigned long long g_42b320;
extern unsigned long long g_42b338;
extern unsigned long long g_42b348;

unsigned long long extract_dirs_from_files(long long a0, char a1)
{
    char v0;  // [bp-0x31]
    unsigned long iter;  // [bp-0x30], Other Possible Types: unsigned long long
    unsigned long long index;  // [bp-0x28]
    struct_0 *v3;  // [bp-0x20]
    struct_1 *v4;  // [bp-0x18]
    unsigned long v5;  // [bp-0x10]

    v0 = a0;
    if (a0 && g_42b320)
        sub_408e46(0, a0, 0);
    iter = g_42b338;
    while (1)
    {
        do
        {
            do
            {
                do
                {
                    iter -= 1;
                    if (!iter)
                    {
                        iter = 0;
                        for (index = 0; iter < g_42b338; iter += 1)
                        {
                            v3 = *((long long *)(iter * 8 + g_42b348));
                            *((struct_0 **)(g_42b348 + index * 8)) = v3;
                            index += v3->field_a8 != 9;
                        }
                        g_42b338 = index;
                        return index;
                    }
                } while ((v4 = (struct_1 *)*((long long *)(iter * 8 + g_42b348)), iter = (unsigned long long)iter, !(char)sub_40a869(v4)));
            } while (v0 == 1 && (iter = (unsigned long long)iter, (char)sub_40aa37(v4->field_0) == 1));
            if (!a0 || v4->field_0->field_0 == 47)
            {
                sub_408e46(v4->field_0, v4->field_8, a1);
            }
            else
            {
                v5 = sub_4116a7(a0, v4->field_0, 0, v4->field_0);
                sub_408e46(v5, v4->field_8, a1);
                free(v5);
            }
            iter = iter;
        } while (v4->field_a8 != 9);
        sub_409669(v4);
    }
}



// Function: xstrcoll @ 0xac36
extern long long g_42c360;

int xstrcoll(unsigned long a0, unsigned long a1)
{
    char *v2;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x2c]

    *(__errno_location()) = 0;
    v0 = strcoll(a0, a1);
    if (!*(__errno_location()))
        return v0;
    sub_41ae9d(1, a1);
    sub_41ae9d(0, a0);
    v2 = gettext("cannot compare file names %s and %s");
    err = __errno_location();
    error(0, *(err), v2);
    sub_408db1(0);
    longjmp(&g_42c360, 1); /* do not return */
}



// Function: dirfirst_check @ 0xad02
unsigned long long dirfirst_check(unsigned int *a0, unsigned int *a1, unsigned long long *a2)
{
    char v2;  // al
    unsigned int v0;  // [bp-0x1c]

    v2 = sub_40a8a4(a1);
    v0 = v2 - (char)sub_40a8a4(a0);
    return (!v0 ? a2(a0, a1, a1, a2) : v0);
}



// Function: cmp_ctime @ 0xad65
unsigned long long cmp_ctime(unsigned long long *a0, unsigned long long *a1, unsigned long long *a2)
{
    long long v2;  // rax
    long long v3;  // rdx
    unsigned int v0;  // [bp-0x2c]

    v2 = sub_41b477(a0 + 3);
    v0 = sub_41bc1f(sub_41b477(a1 + 3), v3, v2, v3);
    return (!v0 ? a2(*(a0), *(a1), *(a1), a2) : v0);
}



// Function: cmp_mtime @ 0xadf8
unsigned long long cmp_mtime(unsigned long long *a0, unsigned long long *a1, unsigned long long *a2)
{
    long long v2;  // rax
    long long v3;  // rdx
    unsigned int v0;  // [bp-0x2c]

    v2 = sub_41b491(a0 + 3);
    v0 = sub_41bc1f(sub_41b491(a1 + 3), v3, v2, v3);
    return (!v0 ? a2(*(a0), *(a1), *(a1), a2) : v0);
}



// Function: cmp_atime @ 0xae8b
unsigned long long cmp_atime(unsigned long long *a0, unsigned long long *a1, unsigned long long *a2)
{
    long long v2;  // rax
    long long v3;  // rdx
    unsigned int v0;  // [bp-0x2c]

    v2 = sub_41b45d(a0 + 3);
    v0 = sub_41bc1f(sub_41b45d(a1 + 3), v3, v2, v3);
    return (!v0 ? a2(*(a0), *(a1), *(a1), a2) : v0);
}



// Function: cmp_btime @ 0xaf1e
unsigned long long cmp_btime(unsigned long long *a0, unsigned long long *a1, unsigned long long *a2)
{
    unsigned long v2;  // rax
    unsigned int v3;  // rsi
    long long v4;  // rdx
    unsigned int v0;  // [bp-0x2c]

    v2 = sub_405990(a0 + 3, a1, a2);
    v0 = sub_41bc1f(sub_405990(a1 + 3, v3, v4), v4, v2, v4);
    return (!v0 ? a2(*(a0), *(a1), *(a1), a2) : v0);
}



// Function: off_cmp @ 0xafb1
unsigned int off_cmp(long long a0, long long a1)
{
    return (a0 < a1 ? 0xffffffff : a1 < a0);
}



// Function: cmp_size @ 0xafe2
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[64];
    long long field_48;
} struct_0;

unsigned long long cmp_size(struct_0 *a0, struct_0 *a1, unsigned long long *a2)
{
    unsigned int v0;  // [bp-0xc]

    v0 = sub_40afb1(a1->field_48, a0->field_48);
    return (!v0 ? a2(a0->field_0, a1->field_0, a1->field_0, a2) : v0);
}



// Function: cmp_name @ 0xb03f
long long cmp_name(unsigned long long *a0, unsigned long long *a1, unsigned long long *a2)
{
    return a2(*(a0), *(a1), *(a1), a2);
}



// Function: cmp_extension @ 0xb073
extern char g_41f27b;

unsigned long long cmp_extension(char **a0, char **a1, unsigned long long *a2)
{
    unsigned int v0;  // [bp-0x1c]
    char *ptr;  // [bp-0x18]
    char *ptr1;  // [bp-0x10]

    ptr = strrchr(*(a0), 46);
    ptr1 = strrchr(*(a1), 46);
    if (!ptr1)
        ptr1 = &g_41f27b;
    if (!ptr)
        ptr = &g_41f27b;
    v0 = a2(ptr, ptr1);
    return (!v0 ? a2(*(a0), *(a1), *(a1), a2) : v0);
}



// Function: fileinfo_name_width @ 0xb119
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

extern long long g_42b410;

unsigned long long fileinfo_name_width(struct_0 *a0)
{
    return (!a0->field_c8 ? sub_40d671(a0->field_0, g_42b410, a0->field_c4, g_42b410) : a0->field_c8);
}



// Function: cmp_width @ 0xb16b
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

unsigned long long cmp_width(struct_0 *a0, struct_0 *a1, unsigned long long *a2)
{
    unsigned int v2;  // eax
    unsigned int v0;  // [bp-0x1c]

    v2 = sub_40b119(a0);
    v0 = v2 - sub_40b119(a1);
    return (!v0 ? a2(a0->field_0, a1->field_0, a1->field_0, a2) : v0);
}



// Function: xstrcoll_ctime @ 0xb1d2
unsigned long long xstrcoll_ctime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ad65(a0, a1, sub_40ac36);
}



// Function: strcmp_ctime @ 0xb202
unsigned long long strcmp_ctime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ad65(a0, a1, strcmp);
}



// Function: rev_xstrcoll_ctime @ 0xb232
unsigned long long rev_xstrcoll_ctime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ad65(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_ctime @ 0xb262
unsigned long long rev_strcmp_ctime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ad65(a1, a0, strcmp);
}



// Function: xstrcoll_df_ctime @ 0xb292
unsigned long long xstrcoll_df_ctime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b1d2);
}



// Function: strcmp_df_ctime @ 0xb2c2
unsigned long long strcmp_df_ctime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b202);
}



// Function: rev_xstrcoll_df_ctime @ 0xb2f2
unsigned long long rev_xstrcoll_df_ctime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b232);
}



// Function: rev_strcmp_df_ctime @ 0xb322
unsigned long long rev_strcmp_df_ctime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b262);
}



// Function: xstrcoll_mtime @ 0xb352
unsigned long long xstrcoll_mtime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40adf8(a0, a1, sub_40ac36);
}



// Function: strcmp_mtime @ 0xb382
unsigned long long strcmp_mtime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40adf8(a0, a1, strcmp);
}



// Function: rev_xstrcoll_mtime @ 0xb3b2
unsigned long long rev_xstrcoll_mtime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40adf8(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_mtime @ 0xb3e2
unsigned long long rev_strcmp_mtime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40adf8(a1, a0, strcmp);
}



// Function: xstrcoll_df_mtime @ 0xb412
unsigned long long xstrcoll_df_mtime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b352);
}



// Function: strcmp_df_mtime @ 0xb442
unsigned long long strcmp_df_mtime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b382);
}



// Function: rev_xstrcoll_df_mtime @ 0xb472
unsigned long long rev_xstrcoll_df_mtime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b3b2);
}



// Function: rev_strcmp_df_mtime @ 0xb4a2
unsigned long long rev_strcmp_df_mtime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b3e2);
}



// Function: xstrcoll_atime @ 0xb4d2
unsigned long long xstrcoll_atime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ae8b(a0, a1, sub_40ac36);
}



// Function: strcmp_atime @ 0xb502
unsigned long long strcmp_atime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ae8b(a0, a1, strcmp);
}



// Function: rev_xstrcoll_atime @ 0xb532
unsigned long long rev_xstrcoll_atime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ae8b(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_atime @ 0xb562
unsigned long long rev_strcmp_atime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40ae8b(a1, a0, strcmp);
}



// Function: xstrcoll_df_atime @ 0xb592
unsigned long long xstrcoll_df_atime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b4d2);
}



// Function: strcmp_df_atime @ 0xb5c2
unsigned long long strcmp_df_atime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b502);
}



// Function: rev_xstrcoll_df_atime @ 0xb5f2
unsigned long long rev_xstrcoll_df_atime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b532);
}



// Function: rev_strcmp_df_atime @ 0xb622
unsigned long long rev_strcmp_df_atime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b562);
}



// Function: xstrcoll_btime @ 0xb652
unsigned long long xstrcoll_btime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40af1e(a0, a1, sub_40ac36);
}



// Function: strcmp_btime @ 0xb682
unsigned long long strcmp_btime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40af1e(a0, a1, strcmp);
}



// Function: rev_xstrcoll_btime @ 0xb6b2
unsigned long long rev_xstrcoll_btime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40af1e(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_btime @ 0xb6e2
unsigned long long rev_strcmp_btime(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40af1e(a1, a0, strcmp);
}



// Function: xstrcoll_df_btime @ 0xb712
unsigned long long xstrcoll_df_btime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b652);
}



// Function: strcmp_df_btime @ 0xb742
unsigned long long strcmp_df_btime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b682);
}



// Function: rev_xstrcoll_df_btime @ 0xb772
unsigned long long rev_xstrcoll_df_btime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b6b2);
}



// Function: rev_strcmp_df_btime @ 0xb7a2
unsigned long long rev_strcmp_df_btime(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b6e2);
}



// Function: xstrcoll_size @ 0xb7d2
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[64];
    long long field_48;
} struct_0;

unsigned long long xstrcoll_size(struct_0 *a0, struct_0 *a1)
{
    return sub_40afe2(a0, a1, sub_40ac36);
}



// Function: strcmp_size @ 0xb802
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[64];
    long long field_48;
} struct_0;

unsigned long long strcmp_size(struct_0 *a0, struct_0 *a1)
{
    return sub_40afe2(a0, a1, strcmp);
}



// Function: rev_xstrcoll_size @ 0xb832
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[64];
    long long field_48;
} struct_0;

unsigned long long rev_xstrcoll_size(struct_0 *a0, struct_0 *a1)
{
    return sub_40afe2(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_size @ 0xb862
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[64];
    long long field_48;
} struct_0;

unsigned long long rev_strcmp_size(struct_0 *a0, struct_0 *a1)
{
    return sub_40afe2(a1, a0, strcmp);
}



// Function: xstrcoll_df_size @ 0xb892
unsigned long long xstrcoll_df_size(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b7d2);
}



// Function: strcmp_df_size @ 0xb8c2
unsigned long long strcmp_df_size(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b802);
}



// Function: rev_xstrcoll_df_size @ 0xb8f2
unsigned long long rev_xstrcoll_df_size(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b832);
}



// Function: rev_strcmp_df_size @ 0xb922
unsigned long long rev_strcmp_df_size(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b862);
}



// Function: xstrcoll_name @ 0xb952
long long xstrcoll_name(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40b03f(a0, a1, sub_40ac36);
}



// Function: strcmp_name @ 0xb982
long long strcmp_name(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40b03f(a0, a1, strcmp);
}



// Function: rev_xstrcoll_name @ 0xb9b2
long long rev_xstrcoll_name(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40b03f(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_name @ 0xb9e2
long long rev_strcmp_name(unsigned long long *a0, unsigned long long *a1)
{
    return sub_40b03f(a1, a0, strcmp);
}



// Function: xstrcoll_df_name @ 0xba12
unsigned long long xstrcoll_df_name(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b952);
}



// Function: strcmp_df_name @ 0xba42
unsigned long long strcmp_df_name(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b982);
}



// Function: rev_xstrcoll_df_name @ 0xba72
unsigned long long rev_xstrcoll_df_name(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b9b2);
}



// Function: rev_strcmp_df_name @ 0xbaa2
unsigned long long rev_strcmp_df_name(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40b9e2);
}



// Function: xstrcoll_extension @ 0xbad2
unsigned long long xstrcoll_extension(char **a0, char **a1)
{
    return sub_40b073(a0, a1, sub_40ac36);
}



// Function: strcmp_extension @ 0xbb02
unsigned long long strcmp_extension(char **a0, char **a1)
{
    return sub_40b073(a0, a1, strcmp);
}



// Function: rev_xstrcoll_extension @ 0xbb32
unsigned long long rev_xstrcoll_extension(char **a0, char **a1)
{
    return sub_40b073(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_extension @ 0xbb62
unsigned long long rev_strcmp_extension(char **a0, char **a1)
{
    return sub_40b073(a1, a0, strcmp);
}



// Function: xstrcoll_df_extension @ 0xbb92
unsigned long long xstrcoll_df_extension(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bad2);
}



// Function: strcmp_df_extension @ 0xbbc2
unsigned long long strcmp_df_extension(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bb02);
}



// Function: rev_xstrcoll_df_extension @ 0xbbf2
unsigned long long rev_xstrcoll_df_extension(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bb32);
}



// Function: rev_strcmp_df_extension @ 0xbc22
unsigned long long rev_strcmp_df_extension(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bb62);
}



// Function: xstrcoll_width @ 0xbc52
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

unsigned long long xstrcoll_width(struct_0 *a0, struct_0 *a1)
{
    return sub_40b16b(a0, a1, sub_40ac36);
}



// Function: strcmp_width @ 0xbc82
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

unsigned long long strcmp_width(struct_0 *a0, struct_0 *a1)
{
    return sub_40b16b(a0, a1, strcmp);
}



// Function: rev_xstrcoll_width @ 0xbcb2
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

unsigned long long rev_xstrcoll_width(struct_0 *a0, struct_0 *a1)
{
    return sub_40b16b(a1, a0, sub_40ac36);
}



// Function: rev_strcmp_width @ 0xbce2
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

unsigned long long rev_strcmp_width(struct_0 *a0, struct_0 *a1)
{
    return sub_40b16b(a1, a0, strcmp);
}



// Function: xstrcoll_df_width @ 0xbd12
unsigned long long xstrcoll_df_width(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bc52);
}



// Function: strcmp_df_width @ 0xbd42
unsigned long long strcmp_df_width(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bc82);
}



// Function: rev_xstrcoll_df_width @ 0xbd72
unsigned long long rev_xstrcoll_df_width(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bcb2);
}



// Function: rev_strcmp_df_width @ 0xbda2
unsigned long long rev_strcmp_df_width(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40bce2);
}



// Function: cmp_version @ 0xbdd2
int cmp_version(void* a0, void* a1)
{
    unsigned int v0;  // [bp-0xc]

    v0 = sub_411c04(*((long long *)a0), *((long long *)a1), *((long long *)a1));
    if (v0)
        return v0;
    return strcmp(*((long long *)a0), *((long long *)a1));
}



// Function: xstrcoll_version @ 0xbe28
int xstrcoll_version(void* a0, void* a1)
{
    return sub_40bdd2(a0, a1);
}



// Function: rev_xstrcoll_version @ 0xbe51
int rev_xstrcoll_version(void* a0, void* a1)
{
    return sub_40bdd2(a1, a0);
}



// Function: xstrcoll_df_version @ 0xbe7a
unsigned long long xstrcoll_df_version(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40be28);
}



// Function: rev_xstrcoll_df_version @ 0xbeaa
unsigned long long rev_xstrcoll_df_version(unsigned int *a0, unsigned int *a1)
{
    return sub_40ad02(a0, a1, sub_40be51);
}



// Function: initialize_ordering_vector @ 0xbeda
extern unsigned long long g_42b328;
extern char g_42b338;
extern unsigned long long g_42b348;

unsigned long long initialize_ordering_vector(void)
{
    unsigned long long idx;  // [bp-0x10]

    for (idx = 0; idx < *((long long *)&g_42b338); idx += 1)
    {
        *((unsigned long long *)(idx * 8 + g_42b348)) = g_42b328 + idx * 208;
    }
    return *((long long *)&g_42b338);
}



// Function: update_current_files_info @ 0xbf3d
typedef struct struct_0 {
    char padding_0[200];
    unsigned long long field_c8;
} struct_0;

extern char g_42b338;
extern unsigned long long g_42b348;
extern unsigned int g_42b3ac;
extern unsigned int g_42b3b4;
extern unsigned long long g_42b430;

unsigned long long update_current_files_info(void)
{
    unsigned long long idx;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    if (g_42b3b4 != 2)
    {
        if (!g_42b430)
        {
            return g_42b430;
        }
        else if (g_42b3ac != 2 && g_42b3ac != 3)
        {
            return g_42b3ac;
        }
    }
    for (idx = 0; idx < *((long long *)&g_42b338); idx += 1)
    {
        v1 = *((long long *)(idx * 8 + g_42b348));
        v1->field_c8 = sub_40b119(v1);
    }
    return *((long long *)&g_42b338);
}



// Function: sort_files @ 0xbfc5
extern unsigned long long g_42a780[4];
extern void g_42b338;
extern void g_42b348;
extern unsigned long long g_42b350;
extern unsigned int g_42b3b0;
extern unsigned int g_42b3b4;
extern char g_42b3b8;
extern char g_42b3f2;
extern long long g_42c360;

long long sort_files(void)
{
    char v0;  // [bp-0x9]

    if ((*((long long *)&g_42b338) >> 1) + *((long long *)&g_42b338) > g_42b350)
    {
        free(*((long long *)&g_42b348));
        *((unsigned long long *)&g_42b348) = sub_41c804(*((long long *)&g_42b338), 24);
        g_42b350 = *((long long *)&g_42b338) * 3;
    }
    sub_40beda();
    sub_40bf3d();
    if (g_42b3b4 == 6)
        return g_42b3b4;
    if (!_setjmp(&g_42c360))
    {
        v0 = 0;
    }
    else
    {
        v0 = 1;
        if (g_42b3b4 == 4)
            __assert_fail(); /* do not return */
        sub_40beda();
    }
    return sub_415dfc(*((long long *)&g_42b348), *((long long *)&g_42b338), g_42a780[8 * ((g_42b3b4 == 5 ? g_42b3b0 : 0) + g_42b3b4 & 0xffffffff) + 4 * v0 + 2 * g_42b3b8 + g_42b3f2], *((long long *)&g_42b338));
}



// Function: print_current_files @ 0xc12d
extern char g_42b030;
extern char g_42b338;
extern long long g_42b348;
extern unsigned int g_42b3ac;
extern unsigned long long g_42b430;

unsigned long long print_current_files(void)
{
    unsigned long long iter;  // [bp-0x10]

    switch (g_42b3ac)
    {
    case 0:
        for (iter = 0; iter < *((long long *)&g_42b338); iter += 1)
        {
            sub_4063f3();
            sub_40c6c8(*((long long *)(iter * 8 + g_42b348)));
            sub_405422(g_42b030);
        }
        return *((long long *)&g_42b338);
    case 1:
        for (iter = 0; iter < *((long long *)&g_42b338); iter += 1)
        {
            sub_40dd38(*((long long *)(iter * 8 + g_42b348)), 0, g_42b348);
            putchar_unlocked(g_42b030);
        }
        return *((long long *)&g_42b338);
    case 2:
        return (!g_42b430 ? sub_40e9d6(32) : sub_40e708());
    case 3:
        return (!g_42b430 ? sub_40e9d6(32) : sub_40e86a());
    case 4:
        return sub_40e9d6(44);
    default:
        return g_42b3ac;
    }
}



// Function: align_nstrftime @ 0xc25f
typedef struct struct_0 {
    char padding_0[16];
    int field_10;
} struct_0;

extern unsigned long long g_42b200[4];
extern char g_42b740;
extern char g_42c340;

long long align_nstrftime(long long a0, long long a1, char a2, struct_0 *a3, long long a4, unsigned int a5)
{
    unsigned long long *v0;  // [bp-0x10]

    v0 = (!g_42c340 ? g_42b200[a2] : &(&g_42b740)[0x600 * a2 + 128 * a3->field_10]);
    return sub_415f45(a0, a1, v0, a3, a4, a5);
}



// Function: long_time_expected_width @ 0xc308
typedef struct struct_0 {
    char padding_0[16];
    int field_10;
} struct_0;

extern void g_42b034;
extern long long g_42b438;

long long long_time_expected_width(void)
{
    unsigned long v0;  // [bp-0x458]
    long long v1;  // [bp-0x450]
    struct_0 v2;  // [bp-0x448]
    char v3;  // [bp-0x408]

    if (*((int *)&g_42b034) < 0)
    {
        v0 = 0;
        if (sub_41b9a5(g_42b438, &v0, &v2, &v0))
        {
            v1 = sub_40c25f(&v3, 1001, 0, &v2, g_42b438, 0);
            if (v1)
                *((unsigned int *)&g_42b034) = sub_415784(&v3, v1, 0, v1);
        }
        if (*((int *)&g_42b034) < 0)
            *((unsigned int *)&g_42b034) = 0;
    }
    return *((int *)&g_42b034);
}



// Function: format_user_or_group @ 0xc3fc
extern unsigned long long g_42b500;

long long format_user_or_group(unsigned long a0, unsigned long long a1, unsigned int a2)
{
    unsigned int v4;  // eax
    unsigned long v5;  // rax
    unsigned int v6;  // eax
    unsigned long long v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    v0 = a1;
    if (!a0)
    {
        v6 = printf("%*lu ");
        g_42b500 = g_42b500 + v6;
        return g_42b500;
    }
    v2 = a2 - sub_41574e(a0, 0);
    v4 = v2;
    if (v4 < 0)
        v4 = 0;
    v1 = v4;
    sub_40549e(a0);
    do
    {
        sub_405422(32);
        v1 -= 1;
    } while ((unsigned int)v5);
    return v5;
}



// Function: format_user @ 0xc49e
extern char g_42b3ba;

long long format_user(unsigned int a0, unsigned int a1, char a2)
{
    unsigned long long v1;  // rax

    v1 = (!(a2 ^ 1) ? (!g_42b3ba ? sub_414a53(a0) : 0) : "?");
    return sub_40c3fc(v1, a0, a1);
}



// Function: format_group @ 0xc4fe
extern char g_42b3ba;

long long format_group(unsigned int a0, unsigned int a1, char a2)
{
    unsigned long long v1;  // rax

    v1 = (!(a2 ^ 1) ? (!g_42b3ba ? sub_414ca5(a0) : 0) : "?");
    return sub_40c3fc(v1, a0, a1);
}



// Function: format_user_or_group_width @ 0xc55e
unsigned long long format_user_or_group_width(long long a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0xc]

    if (!a0)
        return snprintf(NULL, 0, "%lu", a1);
    v0 = sub_41574e(a0, 0);
    return (v0 < 0 ? 0 : v0);
}



// Function: format_user_width @ 0xc5c3
extern char g_42b3ba;

unsigned long long format_user_width(unsigned int a0)
{
    long long v1;  // rax

    v1 = (!g_42b3ba ? sub_414a53(a0) : 0);
    return sub_40c55e(v1, a0);
}



// Function: format_group_width @ 0xc603
extern char g_42b3ba;

unsigned long long format_group_width(unsigned int a0)
{
    long long v1;  // rax

    v1 = (!g_42b3ba ? sub_414ca5(a0) : 0);
    return sub_40c55e(v1, a0);
}



// Function: format_inode @ 0xc643
typedef struct struct_0 {
    char padding_0[32];
    long long field_20;
    char padding_28[144];
    char field_b8;
} struct_0;

long long format_inode(long long a0, unsigned long long a1, struct_0 *a2)
{
    if (a1 <= 20)
        __assert_fail(); /* do not return */
    if (!a2->field_b8 || !a2->field_20)
        return "?";
    return sub_415035(a2->field_20, a0, a0);
    return "?";
}



// Function: print_long_format @ 0xc6c8
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[24];
    long long field_28;
    unsigned int field_30;
    unsigned int field_34;
    unsigned int field_38;
    char padding_3c[4];
    long long field_40;
    unsigned int field_48;
    char padding_4c[12];
    long long field_58;
    char padding_60[72];
    unsigned int field_a8;
    unsigned int field_ac;
    unsigned int field_b0;
    char padding_b4[4];
    char field_b8;
    char padding_b9[3];
    unsigned int field_bc;
} struct_0;

extern char g_420588;
extern char g_42b022;
extern char g_42b023;
extern long long g_42b028;
extern long long g_42b370;
extern unsigned long long g_42b378;
extern char g_42b380;
extern char g_42b381;
extern unsigned int g_42b388;
extern unsigned int g_42b390;
extern unsigned int g_42b394;
extern unsigned int g_42b398;
extern unsigned int g_42b39c;
extern unsigned int g_42b3a0;
extern unsigned int g_42b3a4;
extern unsigned int g_42b3a8;
extern unsigned int g_42b3b0;
extern char g_42b3b9;
extern char g_42b3bb;
extern unsigned int g_42b3bc;
extern long long g_42b3c0;
extern unsigned int g_42b3c8;
extern unsigned int g_42b3d0;
extern char g_42b3e9;
extern long long g_42b438;
extern long long g_42b520;

void print_long_format(struct_0 *a0)
{
    unsigned long v26;  // rdx
    char *v32;  // rax
    long long v33;  // rax
    unsigned int v34;  // eax
    char v35;  // al
    unsigned long v36;  // rcx
    unsigned int v27;  // rsi
    unsigned int v29;  // eax
    unsigned int v30;  // eax
    unsigned int v31;  // eax
    char v0;  // [bp-0x11be]
    char v1;  // [bp-0x11bd]
    int v2;  // [bp-0x11bc]
    int v3;  // [bp-0x11b8]
    unsigned int v4;  // [bp-0x11b4]
    long long v5;  // [bp-0x11b0]
    char *iter;  // [bp-0x11a8], Other Possible Types: void*
    char *v7;  // [bp-0x11a0]
    char *v8;  // [bp-0x11a0]
    char *v9;  // [bp-0x1198]
    char *v10;  // [bp-0x1198]
    unsigned long long v11;  // [bp-0x1190]
    unsigned long v12;  // [bp-0x1188]
    unsigned long v13;  // [bp-0x1180]
    unsigned long v14;  // [bp-0x1178]
    unsigned long v15;  // [bp-0x1170]
    struct_0 v16;  // [bp-0x1168]
    char v17;  // [bp-0x1124]
    char v18;  // [bp-0x1123]
    char v19;  // [bp-0x111a]
    char v20;  // [bp-0x1119]
    char v21;  // [bp-0x1118]
    char v22;  // [bp-0x10f8]
    char v23;  // [bp-0xe6c]
    char v24;  // [bp-0xe68]

    v0 = 1;
    if (a0->field_b8)
    {
        sub_41167b(&a0->padding_10[8], &v17, &a0->padding_10[8]);
    }
    else
    {
        v17 = *(&(&g_420588)[a0->field_a8]);
        memset(&v18, 63, 10);
        v20 = 0;
    }
    if (g_42b381 != 1)
    {
        v19 = 0;
    }
    else if (a0->field_bc == 1)
    {
        v19 = 46;
    }
    else if (a0->field_bc == 2)
    {
        v19 = 43;
    }
    if (g_42b3b0 == 3)
    {
        v12 = sub_405990(&a0->padding_10[8], v27, v26);
        v13 = v26;
        if (v12 == 0xffffffffffffffff && v13 == 0xffffffffffffffff)
            v0 = 0;
    }
    else if (g_42b3b0 > 3)
    {
        abort(); /* do not return */
    }
    else if (g_42b3b0 == 2)
    {
        v12 = sub_41b45d(&a0->padding_10[8]);
        v13 = v26;
    }
    else if (g_42b3b0 > 2)
    {
        abort(); /* do not return */
    }
    else if (!g_42b3b0)
    {
        v12 = sub_41b491(&a0->padding_10[8]);
        v13 = v26;
    }
    else if (g_42b3b0 == 1)
    {
        v12 = sub_41b477(&a0->padding_10[8]);
        v13 = v26;
    }
    else
    {
        abort(); /* do not return */
    }
    iter = &v24;
    if (g_42b3e9)
    {
        sub_40c643(&v22, 21, a0);
        iter = &iter[(int)sprintf(iter, "%*s ")];
    }
    if (g_42b3bb)
    {
        v7 = (!(a0->field_b8 ^ 1) ? sub_413d91(a0->field_58, &v22, g_42b3bc, 0x200, g_42b3c0) : "?");
        v29 = sub_41574e(v7, 0);
        for (v2 = g_42b388 - v29; v2 > 0; iter = iter)
        {
            iter += 1;
            *(iter) = 32;
            v2 -= 1;
        }
        do
        {
            v8 = v7 + 1;
            iter += 1;
            *(iter) = *(v7);
            v7 = v8;
        } while (*(iter));
        *(iter - 1) = 32;
    }
    if (a0->field_b8 == 1)
        sub_415035(a0->field_28, &v22, &v22);
    iter = &iter[(int)sprintf(iter, "%s %*s ", &v17)];
    sub_4054cf();
    if (g_42b022 || g_42b023 || g_42b3b9 || g_42b380)
    {
        sub_405453(&v24, iter - &v24);
        if (g_42b022)
            sub_40c49e(a0->field_34, g_42b394, a0->field_b8);
        if (g_42b023)
            sub_40c4fe(a0->field_38, g_42b398, a0->field_b8);
        if (g_42b3b9)
            sub_40c49e(a0->field_34, g_42b39c, a0->field_b8);
        if (g_42b380)
            sub_40c3fc(*((long long *)&a0->field_b0), 0, g_42b390);
        iter = &v24;
    }
    if (a0->field_b8 && (((unsigned short)a0->field_30 & 0xf000) == 0x2000 || ((unsigned short)a0->field_30 & 0xf000) == 0x6000))
    {
        v4 = g_42b3a8 - (g_42b3a4 + g_42b3a0 + 2);
        v30 = gnu_dev_minor(a0->field_40);
        sub_415035(v30, &v22, v30);
        v31 = gnu_dev_major(a0->field_40);
        sub_415035(v31, &v21, v31);
        iter += (int)sprintf(iter, "%*s, %*s ");
    }
    else
    {
        if (a0->field_b8 != 1)
        {
            v32 = "?";
        }
        else
        {
            v33 = sub_409635(*((long long *)&a0->field_48));
            v32 = sub_413d91(v33, &v22, g_42b3c8, 1, g_42b028);
        }
        v9 = v32;
        v34 = sub_41574e(v9, 0);
        for (v3 = g_42b3a8 - v34; v3 > 0; iter = iter)
        {
            iter += 1;
            *((char *)iter) = 32;
            v3 -= 1;
        }
        do
        {
            v10 = v9 + 1;
            iter += 1;
            *((char *)iter) = *(v9);
            v9 = v10;
        } while (*((char *)iter));
        *((char *)iter - 1) = 32;
    }
    v5 = 0;
    *((char *)iter) = 1;
    if (a0->field_b8 && v0 && sub_41b9a5(g_42b438, &v12, &v16, &v12))
    {
        if (sub_41bc1f(g_42b370, g_42b378, v12, v13) < 0)
            sub_411ec3(&g_42b370);
        v14 = g_42b370 - 15778476;
        v15 = g_42b378;
        if (sub_41bc1f(v14, v15, v12, v13) < 0 && sub_41bc1f(v12, v13, g_42b370, g_42b378) < 0)
            v35 = 1;
        else
            v35 = 0;
        v1 = v35;
        v1 &= 1;
        v5 = sub_40c25f(iter, 1001, v1, &v16, g_42b438, v13);
    }
    if (v5 || !*((char *)iter))
    {
        iter += v5;
        iter += 1;
        *((char *)iter) = 32;
    }
    else
    {
        if (a0->field_b8 && v0)
            sub_405219(v12, &v22, &v22);
        sub_40c308();
        iter += (int)sprintf(iter, "%*s ");
    }
    sub_405453(&v24, iter - &v24);
    v11 = sub_40db96(a0, 0, &g_42b520, iter - &v24);
    if (a0->field_a8 != 6)
    {
        if (!g_42b3d0)
            return;
        v36 = a0->field_30;
        sub_40e05c(a0->field_b8, v36 & 0xffffffff, a0->field_a8, v36);
        return;
    }
    else if (a0->field_8)
    {
        sub_40549e(" -> ");
        sub_40db96(a0, 1, 0, &(-(&v23))[v11] + iter);
        if (!g_42b3d0)
            return;
        sub_40e05c(1, a0->field_ac, 0);
        return;
    }
    else
    {
        return;
    }
}



// Function: quote_name_buf @ 0xd10c
typedef struct mbstate_t {
} mbstate_t;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_42b340;
extern char g_42b341;
extern char g_42b408;

long long quote_name_buf(struct_0 **a0, unsigned long a1, char *ptr, long long a3, unsigned int a4, void* a5, char *a6)
{
    char v19;  // al
    char v20;  // al
    char v29;  // al
    unsigned int v21;  // eax
    char *v22;  // rax
    char *v23;  // rax
    char *v24;  // rax
    unsigned long long *v25;  // rax
    char v26;  // al
    unsigned long long *v27;  // rax
    char v28;  // al
    char v0;  // [bp-0x8e]
    char v1;  // [bp-0x8d]
    unsigned int v2;  // [bp-0x8c]
    unsigned int v3;  // [bp-0x88]
    unsigned int v4;  // [bp-0x84]
    char *v5;  // [bp-0x80]
    unsigned long iter;  // [bp-0x78]
    unsigned long len;  // [bp-0x70], Other Possible Types: unsigned long long
    char *v8;  // [bp-0x68]
    char *iter1;  // [bp-0x68]
    char *l;  // [bp-0x60]
    char *v11;  // [bp-0x60]
    unsigned long long v12;  // [bp-0x58]
    char *node;  // [bp-0x50]
    char *k;  // [bp-0x48]
    char *v15;  // [bp-0x40]
    char *v16;  // [bp-0x38]
    char *v17;  // [bp-0x30]
    mbstate_t v18;  // [bp-0x28]

    v5 = &*(a0)->field_0;
    len = 0;
    v3 = sub_418fc3(a3);
    if (g_42b408 && (v3 == 1 || v3 == 2 || !v3))
        v19 = 1;
    else
        v19 = 0;
    v1 = v19;
    v1 &= 1;
    if (a4)
    {
        len = sub_41a389(v5, a1, ptr, -0x1, a3);
        if (a1 <= len)
        {
            v5 = sub_41c68b(len + 1);
            sub_41a389(v5, len + 1, ptr, -0x1, a3);
        }
        if (*(ptr) != *(v5) || len != strlen(ptr))
            v20 = 1;
        else
            v20 = 0;
        v0 = v20;
        v0 &= 1;
    }
    else if (v1)
    {
        len = strlen(ptr);
        if (a1 <= len)
            v5 = sub_41c68b(len + 1);
        memcpy(v5, ptr, len + 1);
        v0 = 0;
    }
    else
    {
        len = strlen(ptr);
        v5 = ptr;
        v0 = 0;
    }
    if (v1)
    {
        if (__ctype_get_mb_cur_max() > 1)
        {
            iter1 = v5;
            v17 = &v5[len];
            l = v5;
            iter = 0;
            while (iter1 < v17)
            {
                v21 = *(iter1);
                if (v21 <= 126)
                {
                    if (v21 < 97)
                    {
                        if (v21 > 95)
                            goto LABEL_40d3b9;
                        if (v21 < 65)
                        {
                            if (v21 <= 35)
                            {
                                if (v21 < 32)
                                    goto LABEL_40d3b9;
                            }
                            else
                            {
                                if (v21 - 37 > 26)
                                    goto LABEL_40d3b9;
                            }
                        }
                    }
                    v8 = iter1 + 1;
                    v11 = l + 1;
                    *(l) = *(iter1);
                    iter += 1;
                    iter1 = v8;
                    l = v11;
                }
                else
                {
LABEL_40d3b9:
                    v18 = (mbstate_t)0;
                    do
                    {
                        v12 = sub_4150be(&v2, iter1, v17 - iter1, &v18);
                        if (v12 == 0xffffffffffffffff)
                        {
                            iter1 += 1;
                            v23 = l;
                            l = v23 + 1;
                            *(v23) = 63;
                            iter += 1;
                            break;
                        }
                        else
                        {
                            switch (v12)
                            {
                            case 18446744073709551614:
                                iter1 = v17;
                                v24 = l;
                                l = v24 + 1;
                                *(v24) = 63;
                                iter += 1;
                                goto LABEL_40d4c5;
                            case 0:
                                v12 = 1;
                                break;
                            default:
                                goto LABEL_40d449;
                            }
                        }
LABEL_40d449:
                        if (v4 >= 0)
                        {
                            for (v4 = wcwidth(v2); v12; l = v11)
                            {
                                v8 = iter1 + 1;
                                v11 = l + 1;
                                *(l) = *(iter1);
                                v12 -= 1;
                                iter1 = v8;
                            }
                            iter += v4;
                        }
                        else
                        {
                            iter1 = &iter1[v12];
                            v22 = l;
                            l = v22 + 1;
                            *(v22) = 63;
                            iter += 1;
                        }
                    } while (!mbsinit(&v18));
LABEL_40d4c5:
                }
            }
            len = l - v5;
        }
        else
        {
            node = v5;
            for (v16 = &v5[len]; node < v16; node += 1)
            {
                v25 = __ctype_b_loc();
                v26 = sub_404eb2(*(node));
                if (!(*((short *)(v26 * 2 + *(v25))) & 0x4000))
                    *(node) = 63;
            }
            iter = len;
        }
    }
    else
    {
        if (a5)
        {
            if (__ctype_get_mb_cur_max() > 1)
            {
                iter = sub_415784(v5, len, 0, len);
            }
            else
            {
                k = v5;
                v15 = &v5[len];
                for (iter = 0; k < v15; k += 1)
                {
                    v27 = __ctype_b_loc();
                    v28 = sub_404eb2(*(k));
                    if (*((short *)(v28 * 2 + *(v27))) & 0x4000)
                        iter += 1;
                }
            }
        }
    }
    if (g_42b341 && g_42b340 && v0 != 1)
        v29 = 1;
    else
        v29 = 0;
    *(a6) = v29 & 1;
    if (a5)
        *((unsigned long *)a5) = iter;
    *(a0) = v5;
    return len;
}



// Function: quote_name_width @ 0xd671
unsigned long long quote_name_width(char *a0, long long a1, unsigned int a2)
{
    char v0;  // [bp-0x2029]
    char *v1;  // [bp-0x2028]
    char v2;  // [bp-0x2020], Other Possible Types: unsigned long long
    char v3;  // [bp-0x2018]

    v1 = &v3;
    sub_40d10c(&v1, 0x2000, a0, a1, a2, &v2, &v0);
    if (v1 != &v3 && a0 != v1)
        free(v1);
    v2 += v0;
    return v2;
}



// Function: file_escape @ 0xd775
extern char g_42b640;

char * file_escape(char *a0, char a1)
{
    char *v4;  // rax
    char *v5;  // rdx
    char *v6;  // rax
    char *v7;  // rax
    char *v0;  // [bp-0x20]
    char *iter;  // [bp-0x18]
    char *v2;  // [bp-0x10]

    v0 = a0;
    v2 = sub_41c804(3, strlen(v0) + 1);
    iter = v2;
    while (*(v0))
    {
        if (a1 && *(v0) == 47)
        {
            v4 = iter;
            iter = v4 + 1;
            *(v4) = 47;
            v0 += 1;
        }
        else if (*((char *)(unsigned int)&(&g_42b640)[sub_404eb2(*(v0))]))
        {
            v5 = v0;
            v0 = v5 + 1;
            v6 = iter;
            iter = v6 + 1;
            *(v6) = *(v5);
        }
        else
        {
            v7 = v0;
            v0 = v7 + 1;
            iter = &iter[(int)sprintf(iter, "%%%02x", (char)sub_404eb2(*(v7)))];
        }
    }
    *(iter) = 0;
    return v2;
}



// Function: quote_name @ 0xd87f
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern char g_41f27b;
extern char g_42160d;
extern char g_421621;
extern FILE *stdout;
extern char g_42b340;
extern char g_42b341;
extern char *g_42b360;
extern unsigned long long g_42b500;

long long quote_name(char *a0, long long a1, unsigned int a2, long long a3, char a4, struct_0 *a5, char *a6)
{
    char v0;  // [bp-0x203a]
    char v1;  // [bp-0x2039]
    char *v2;  // [bp-0x2038]
    long long v3;  // [bp-0x2030]
    void* v4;  // [bp-0x2028]
    char *v5;  // [bp-0x2020]
    char v6;  // [bp-0x2018]

    v2 = &v6;
    v3 = sub_40d10c(&v2, 0x2000, a0, a1, a2, NULL, &v0);
    if (v0 && a4)
        sub_405422(32);
    if (a3)
        sub_40e0a3(a3);
    v1 = 0;
    if (a6)
    {
        if (g_42b341 && g_42b340 && v0 != 1)
        {
            v1 = 1;
            putchar_unlocked(v6);
        }
        v4 = sub_40d775(g_42b360, 0);
        v5 = sub_40d775(a6, 1);
        printf(&g_42160d, v4, (*(v5) == 47 ? &g_41f27b : "/"), v5);
        free(v4);
        free(v5);
    }
    if (a5)
        sub_4054f4(a5);
    fwrite_unlocked(&v2[v1], 1, v3 - v1 * 2, stdout);
    g_42b500 = v3 + g_42b500;
    if (a5)
        sub_4054f4(a5);
    if (a6)
    {
        fputs_unlocked(&g_421621, stdout);
        if (v1)
            putchar_unlocked(*(&v2[v3] - 1));
    }
    if (v2 != &v6 && a0 != v2)
        free(v2);
    return v3 + v0;
}



// Function: print_name_with_quoting @ 0xdb96
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char *field_10;
    char padding_18[172];
    unsigned int field_c4;
} struct_0;

extern long long g_42b1f0;
extern char g_42b3d4;
extern long long g_42b410;
extern unsigned long long g_42b430;

long long print_name_with_quoting(struct_0 *a0, char a1, char *a2, unsigned long long a3)
{
    char v6;  // al
    char v0;  // [bp-0x21]
    char *v1;  // [bp-0x20]
    long long v2;  // [bp-0x18]
    long long v3;  // [bp-0x10]

    v1 = (!a1 ? a0->field_0 : a0->field_8);
    v2 = (!g_42b3d4 ? 0 : sub_40e101(a0, a1, a1));
    if (g_42b3d4 && (v2 || (char)sub_406325(4)))
        v6 = 1;
    else
        v6 = 0;
    v0 = v6;
    v0 &= 1;
    v3 = sub_40d87f(v1, g_42b410, a0->field_c4, v2, !a1, a2, a0->field_10);
    sub_40648c();
    if (!v0)
        return v3;
    sub_40dce3();
    if (!g_42b430)
        return v3;
    if (a3 / g_42b430 == (v3 + a3 - 1) / g_42b430)
        return v3;
    sub_40e4ca(&g_42b1f0);
    return v3;
}



// Function: prep_non_filename_text @ 0xdce3
extern long long g_42b080;
extern long long g_42b090;
extern long long g_42b0a0;
extern unsigned long long g_42b0a8;
extern long long g_42b0b0;

int prep_non_filename_text(void)
{
    if (!g_42b0a8)
    {
        sub_40e4ca(&g_42b080);
        sub_40e4ca(&g_42b0b0);
        return sub_40e4ca(&g_42b090);
    }
    return sub_40e4ca(&g_42b0a0);
}



// Function: print_file_name_and_frills @ 0xdd38
typedef struct struct_0 {
    char padding_0[48];
    unsigned int field_30;
    char padding_34[36];
    long long field_58;
    char padding_60[72];
    unsigned int field_a8;
    char padding_ac[12];
    char field_b8;
} struct_0;

extern char g_42b380;
extern char g_42b3bb;
extern unsigned int g_42b3bc;
extern long long g_42b3c0;
extern unsigned int g_42b3d0;
extern char g_42b3e9;

long long print_file_name_and_frills(struct_0 *a0, unsigned long long a1)
{
    unsigned long v3;  // rcx
    long long v0;  // [bp-0x2b0]
    char v1;  // [bp-0x2a8]

    sub_4063f3();
    if (g_42b3e9)
    {
        sub_40c643(&v1, 652, a0);
        printf("%*s ");
    }
    if (g_42b3bb)
    {
        if (a0->field_b8 == 1)
            sub_413d91(a0->field_58, &v1, g_42b3bc, 0x200, g_42b3c0);
        printf("%*s ");
    }
    if (g_42b380)
        printf("%*s ");
    v0 = sub_40db96(a0, 0, NULL, a1);
    if (!g_42b3d0)
        return v0;
    v3 = a0->field_30;
    v0 += sub_40e05c(a0->field_b8, v3 & 0xffffffff, a0->field_a8, v3) & 0xff;
    return v0;
}



// Function: get_type_indicator @ 0xdf23
extern unsigned int g_42b3d0;

int get_type_indicator(char a0, unsigned int a1, unsigned int a2)
{
    char v2;  // al
    char v3;  // al
    char v0;  // [bp-0x9]

    if (!(!a0 ? a2 == 5 : (a1 & 0xf000) == 0x8000))
    {
        if (a0)
        {
            v2 = (a1 & 0xf000) == 0x4000;
        }
        else
        {
            if (a2 == 3 || a2 == 9)
                v3 = 1;
            else
                v3 = 0;
            v2 = v3 & 1;
        }
        if (v2)
        {
            v0 = 47;
        }
        else
        {
            if (g_42b3d0 == 1)
            {
                v0 = 0;
            }
            else
            {
                if ((!a0 ? a2 == 6 : (a1 & 0xf000) == 0xa000))
                {
                    v0 = 64;
                }
                else
                {
                    if ((!a0 ? a2 == 1 : (a1 & 0xf000) == 0x1000))
                        v0 = 124;
                    else
                        v0 = (!(!a0 ? a2 == 7 : (a1 & 0xf000) == 0xc000) ? 0 : 61);
                }
            }
        }
    }
    else if (a0 && g_42b3d0 == 3 && (char)a1 & 73)
    {
        v0 = 42;
    }
    else
    {
        v0 = 0;
    }
    return v0;
}



// Function: print_type_indicator @ 0xe05c
char print_type_indicator(char a0, unsigned int a1, unsigned int a2)
{
    char v0;  // [bp-0x9]

    v0 = sub_40df23(a0, a1, a2);
    if (v0)
        sub_405422(v0);
    return v0;
}



// Function: print_color_indicator @ 0xe0a3
extern long long g_42b080;
extern long long g_42b090;

char print_color_indicator(long long a0)
{
    if (a0)
    {
        if ((char)sub_406325(4))
            sub_4063ca();
        sub_40e4ca(&g_42b080);
        sub_40e4ca(a0);
        sub_40e4ca(&g_42b090);
    }
    return a0;
}



// Function: get_color_indicator @ 0xe101
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[16];
    struct struct_0 *field_20;
} struct_0;

extern char g_42b080;
extern char g_42b358;
extern struct_0 *g_42b3d8;

struct_0 * get_color_indicator(struct_0 *idx, char a1)
{
    long long v8;  // rcx
    unsigned int *v0;  // [bp-0x34]
    unsigned int v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x2c]
    struct_0 *iter;  // [bp-0x28]
    unsigned long node;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned long len;  // [bp-0x18]
    struct_0 *v6;  // [bp-0x10]

    if (a1)
    {
        node = idx->field_8;
        v1 = *((int *)((char *)&idx[4].field_8 + 4));
        v2 = (!idx[4].padding_10[9] ? 0xffffffff : 0);
    }
    else
    {
        node = idx->field_0;
        v1 = sub_4053e7(idx);
        v2 = idx[4].padding_10[9];
    }
    if (v2 == 0xffffffff && (char)sub_406325(12))
    {
        v0 = 0xc;
    }
    else if (idx[4].padding_10[8] != 1)
    {
        v0 = (unsigned int *)*((int *)(4 * (int)idx[4].field_8 + 4370496));
    }
    else if (((unsigned short)v1 & 0xf000) == 0x8000)
    {
        v0 = 0x5;
        if ((unsigned short)v1 & 0x800 && (char)sub_406325(16))
        {
            v0 = 0x10;
        }
        else
        {
            if ((unsigned short)v1 & 0x400 && (char)sub_406325(0x11))
            {
                v0 = 0x11;
            }
            else
            {
                if ((char)sub_406325(21) && (char)idx[4].field_20)
                {
                    v0 = 0x15;
                }
                else
                {
                    if ((char)v1 & 73 && (char)sub_406325(14))
                    {
                        v0 = 0xe;
                    }
                    else
                    {
                        if (idx[1].field_0 > 1 && (char)sub_406325(22))
                            v0 = 0x16;
                    }
                }
            }
        }
    }
    else if (((unsigned short)v1 & 0xf000) == 0x4000)
    {
        v0 = 0x6;
        if ((unsigned short)v1 & 0x200 && (char)v1 & 2 && (char)sub_406325(20))
        {
            v0 = 0x14;
        }
        else if ((char)v1 & 2 && (char)sub_406325(19))
        {
            v0 = 0x13;
        }
        else if ((unsigned short)v1 & 0x200 && (char)sub_406325(18))
        {
            v0 = 0x12;
        }
    }
    else
    {
        v0 = ((v1 & 0xf000) == 0xa000 ? 0x7 : ((v1 & 0xf000) == 0x1000 ? 8 : ((v1 & 0xf000) == 0xc000 ? 9 : ((v1 & 0xf000) == 0x6000 ? 10 : ((v1 & 0xf000) == 0x2000 ? 11 : 13)))));
    }
    iter = NULL;
    if (v0 == 0x5)
    {
        len = strlen(node);
        node += len;
        for (iter = g_42b3d8; iter && (len < iter->field_0 || !(v8 = (long long)(node + -(iter->field_0)), !(unsigned int)(unsigned long long)sub_4102b5(v8, iter->field_8, iter->field_0, v8))); iter = iter->field_20);
    }
    if (v0 == 0x7 && !v2 && (g_42b358 || (char)sub_406325(13)))
        v0 = 0xd;
    v6 = (!iter ? &g_42b080 + 0x10 * v0 : (struct struct_0 *)iter->padding_10);
    if (!v6->field_8)
        return NULL;
    return v6;
}



// Function: put_indicator @ 0xe4ca
typedef struct struct_0 {
    char field_0;
    char padding_1[7];
    void* field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_42b3d6;

long long put_indicator(struct_0 *a0)
{
    if (g_42b3d6 != 1)
    {
        g_42b3d6 = 1;
        if (tcgetpgrp(1) >= 0)
            sub_406858();
        sub_40dce3();
    }
    return fwrite_unlocked(a0->field_8, *((long long *)&a0->field_0), 1, stdout);
}



// Function: length_of_file_name_and_frills @ 0xe530
typedef struct struct_0 {
    long long field_0;
    char padding_8[188];
    unsigned int field_c4;
    unsigned long long field_c8;
} struct_0;

extern char g_42b380;
extern unsigned int g_42b384;
extern unsigned int g_42b388;
extern unsigned int g_42b390;
extern unsigned int g_42b3ac;
extern char g_42b3bb;
extern unsigned int g_42b3bc;
extern long long g_42b3c0;
extern unsigned int g_42b3d0;
extern char g_42b3e9;

unsigned long long length_of_file_name_and_frills(struct_0 *a0)
{
    char *v5;  // rax
    unsigned long v6;  // rax
    char v0;  // [bp-0x2b1]
    unsigned long long iter;  // [bp-0x2b0]
    char v2;  // [bp-0x2a8]

    iter = 0;
    if (g_42b3e9)
        iter += (g_42b3ac == 4 ? strlen(sub_415035(*((long long *)&a0->padding_8[24]), &v2, &v2)) + 1 : g_42b384 + 1);
    if (g_42b3bb)
    {
        if (g_42b3ac == 4)
        {
            v5 = (!(a0->padding_8[176] ^ 1) ? sub_413d91(*((long long *)&a0->padding_8[80]), &v2, g_42b3bc, 0x200, g_42b3c0) : "?");
            v6 = strlen(v5) + 1;
        }
        else
        {
            v6 = g_42b388 + 1;
        }
        iter += v6;
    }
    if (g_42b380)
        iter += (g_42b3ac == 4 ? strlen(*((long long *)&a0->padding_8[168])) + 1 : g_42b390 + 1);
    iter += sub_40b119(a0);
    if (!g_42b3d0)
        return iter;
    v0 = sub_40df23(a0->padding_8[176], *((int *)&a0->padding_8[40]), *((int *)&a0->padding_8[160]));
    iter += v0;
    return iter;
}



// Function: print_many_per_line @ 0xe708
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

extern char g_42b030;
extern unsigned long long g_42b338;
extern unsigned long long g_42b348;
extern unsigned long long g_42b4f0;

unsigned long long print_many_per_line(void)
{
    unsigned long long idx;  // rax
    unsigned long long i;  // [bp-0x58]
    unsigned long long v1;  // [bp-0x50]
    unsigned long long iter;  // [bp-0x48]
    unsigned long v3;  // [bp-0x40]
    unsigned long long v4;  // [bp-0x38]
    struct_0 *v5;  // [bp-0x30]
    unsigned long long v6;  // [bp-0x28]
    unsigned long v7;  // [bp-0x20]
    unsigned long long v8;  // [bp-0x18]
    unsigned long v9;  // [bp-0x10]

    v4 = sub_40ee73(1);
    v5 = v4 * 24 - 24 + g_42b4f0;
    v6 = (g_42b338 % v4) + g_42b338 / v4;
    for (i = 0; i < v6; i += 1)
    {
        v1 = 0;
        iter = i;
        v3 = 0;
        while (1)
        {
            v7 = *((long long *)(iter * 8 + g_42b348));
            v8 = sub_40e530(v7);
            idx = v1;
            v1 = idx + 1;
            v9 = *((long long *)(idx * 8 + v5->field_10));
            sub_40dd38(v7, v3);
            iter += v6;
            if (iter >= g_42b338)
                break;
            sub_40eaef(v8 + v3, v3 + v9, v3 + v9, v3);
            v3 += v9;
        }
        putchar_unlocked(g_42b030);
    }
    return i;
}



// Function: print_horizontal @ 0xe86a
typedef struct struct_0 {
    char padding_0[16];
    struct struct_1 *field_10;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern char g_42b030;
extern char g_42b338;
extern struct_1 *g_42b348;
extern unsigned long long g_42b4f0;

int print_horizontal(void)
{
    unsigned long long index;  // [bp-0x48]
    unsigned long v1;  // [bp-0x40]
    unsigned long v2;  // [bp-0x38]
    unsigned long v3;  // [bp-0x30]
    unsigned long long v4;  // [bp-0x28]
    struct_0 *idx;  // [bp-0x20]
    unsigned long v6;  // [bp-0x18]
    unsigned long long idx1;  // [bp-0x10]

    v1 = 0;
    v4 = sub_40ee73(0);
    idx = v4 * 24 - 24 + g_42b4f0;
    v6 = *((long long *)&g_42b348);
    v2 = sub_40e530(v6);
    v3 = idx->field_10->field_0;
    sub_40dd38(v6, 0);
    for (index = 1; index < *((long long *)&g_42b338); index += 1)
    {
        idx1 = index % v4;
        if (!idx1)
        {
            putchar_unlocked(g_42b030);
            v1 = 0;
        }
        else
        {
            sub_40eaef(v2 + v1, v1 + v3, v1 + v3, v1);
            v1 += v3;
        }
        v6 = *((long long *)&g_42b348[8 * index]);
        sub_40dd38(v6, v1);
        v2 = sub_40e530(v6);
        v3 = idx->field_10[idx1].field_0;
    }
    return putchar_unlocked(g_42b030);
}



// Function: print_with_separator @ 0xe9d6
extern char g_42b030;
extern char g_42b338;
extern unsigned long long g_42b348;
extern unsigned long long g_42b430;

int print_with_separator(char a0)
{
    char v0;  // [bp-0x29]
    unsigned long long idx;  // [bp-0x28]
    unsigned long long iter;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]
    unsigned long long v4;  // [bp-0x10]

    iter = 0;
    for (idx = 0; idx < *((long long *)&g_42b338); idx += 1)
    {
        v3 = *((long long *)(idx * 8 + g_42b348));
        v4 = (!g_42b430 ? 0 : sub_40e530(v3));
        if (idx)
        {
            if (!g_42b430 || v4 + iter + 2 < g_42b430 && iter <= 0xfffffffffffffffd - v4)
            {
                iter += 2;
                v0 = 32;
            }
            else
            {
                iter = 0;
                v0 = g_42b030;
            }
            putchar_unlocked(a0);
            putchar_unlocked(v0);
        }
        sub_40dd38(v3, iter);
        iter += v4;
    }
    return putchar_unlocked(g_42b030);
}



// Function: indent @ 0xeaef
extern unsigned long long g_42b420;

unsigned long long indent(unsigned long long a0, unsigned long long a1)
{
    unsigned long long iter;  // [bp-0x10]

    iter = a0;
    while (iter < a1)
    {
        if (g_42b420 && a1 / g_42b420 > (iter + 1) / g_42b420)
        {
            putchar_unlocked(9);
            iter += g_42b420 - iter % g_42b420;
        }
        else
        {
            putchar_unlocked(32);
            iter += 1;
        }
    }
    return iter;
}



// Function: attach @ 0xeb94
char * attach(char *a0, void* a1, char *a2)
{
    char *v7;  // rax
    char *v8;  // rax
    char *j;  // [bp-0x30]
    char *v1;  // [bp-0x30]
    char *v2;  // [bp-0x20]
    char *v3;  // [bp-0x20]
    void* i;  // [bp-0x10]
    void* v5;  // [bp-0x10]

    v2 = a0;
    j = a2;
    if (*((char *)a1) != 46 || (char)a1[1])
    {
        for (i = a1; *((char *)i); i = v5)
        {
            v5 = i + 1;
            v7 = v2;
            v2 = v7 + 1;
            *(v7) = *((char *)i);
        }
        if (i > a1 && *((char *)i - 1) != 47)
        {
            v8 = v2;
            v2 = v8 + 1;
            *(v8) = 47;
        }
    }
    for (; *(j); j = v1)
    {
        v1 = j + 1;
        v3 = v2 + 1;
        *(v2) = *(j);
        v2 = v3;
    }
    *(v2) = 0;
    return v2;
}



// Function: init_column_info @ 0xec50
extern long long g_42b4f0;
extern char g_42b4f8;
extern unsigned long long g_42c440;

unsigned long long init_column_info(unsigned long long a0)
{
    unsigned long long iter;  // [bp-0x40]
    unsigned long long v1;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    unsigned long long index;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    unsigned long long v5;  // [bp-0x18]
    unsigned long long v6;  // [bp-0x10]

    if (a0 > g_42c440)
    {
        if (!*((long long *)&g_42b4f8) || a0 < *((long long *)&g_42b4f8) >> 1)
        {
            g_42b4f0 = sub_41c638(g_42b4f0, a0, 48, a0);
            v1 = a0 * 2;
        }
        else
        {
            g_42b4f0 = sub_41c638(g_42b4f0, *((long long *)&g_42b4f8), 24, *((long long *)&g_42b4f8));
            v1 = *((long long *)&g_42b4f8);
        }
        v4 = v1 - g_42c440;
        v5 = v1 + g_42c440 + 1;
        v6 = v4 * v5;
        if (v5 < v1 || v5 != v6 / v4)
            sub_41ccc3(); /* do not return */
        v2 = sub_41c804(v6 >> 1, 8);
        for (iter = g_42c440; iter < v1; iter += 1)
        {
            *((unsigned long long *)(g_42b4f0 + iter * 24 + 16)) = v2;
            v2 += (iter + 1) * 8;
        }
        g_42c440 = v1;
    }
    for (iter = 0; iter < a0; iter += 1)
    {
        *((char *)(iter * 24 + g_42b4f0)) = 1;
        *((unsigned long long *)(g_42b4f0 + iter * 24 + 8)) = (iter + 1) * 3;
        for (index = 0; index <= iter; index += 1)
        {
            *((unsigned long *)(index * 8 + *((long long *)(iter * 24 + g_42b4f0 + 16)))) = 3;
        }
    }
    return iter;
}



// Function: calculate_columns @ 0xee73
extern char g_42b338;
extern unsigned long long g_42b348;
extern char g_42b430;
extern unsigned long long g_42b4f0;
extern unsigned long long g_42b4f8;

unsigned long long calculate_columns(char a0)
{
    unsigned long long v9;  // rax
    unsigned long long idx;  // [bp-0x48]
    unsigned long long j;  // [bp-0x40]
    unsigned long long index;  // [bp-0x38]
    unsigned long long v3;  // [bp-0x30]
    unsigned long v4;  // [bp-0x28]
    unsigned long long v5;  // [bp-0x20]
    unsigned long long idx1;  // [bp-0x18]
    unsigned long v7;  // [bp-0x10]

    if (g_42b4f8 && g_42b4f8 < *((long long *)&g_42b338))
        v9 = g_42b4f8;
    else
        v9 = *((long long *)&g_42b338);
    v3 = v9;
    sub_40ec50(v3);
    for (idx = 0; idx < *((long long *)&g_42b338); idx += 1)
    {
        v4 = *((long long *)(idx * 8 + g_42b348));
        v5 = sub_40e530(v4);
        for (index = 0; index < v3; index += 1)
        {
            if (*((char *)(index * 24 + g_42b4f0)))
            {
                idx1 = (!a0 ? idx % (index + 1) : idx / ((index + *((long long *)&g_42b338)) / (index + 1)));
                v7 = v5 + (idx1 == index ? 0 : 2);
                if (v7 > *((long long *)(idx1 * 8 + *((long long *)(index * 24 + g_42b4f0 + 16)))))
                {
                    *((unsigned long *)(index * 24 + g_42b4f0 + 8)) = *((long long *)(index * 24 + g_42b4f0 + 8)) + v7 - *((long long *)(idx1 * 8 + *((long long *)(index * 24 + g_42b4f0 + 16))));
                    *((unsigned long *)(*((long long *)(index * 24 + g_42b4f0 + 16)) + idx1 * 8)) = v7;
                    *((char *)(index * 24 + g_42b4f0)) = *((long long *)(index * 24 + g_42b4f0 + 8)) < *((long long *)&g_42b430);
                }
            }
        }
    }
    for (j = v3; j > 1; j -= 1)
    {
        if (*((char *)(j * 24 - 24 + g_42b4f0)))
            return j;
    }
    return j;
}



// Function: usage @ 0xf135
typedef struct FILE {
} FILE;

extern unsigned int g_42b210;
extern FILE *stdout;
extern FILE *stderr;

void usage(int a0)
{
    unsigned long long v2;  // rbx
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
    char *v24;  // rax
    char *v25;  // rax
    char *v26;  // rax
    char *v27;  // rax
    char *v28;  // rax
    char *v29;  // rax
    char *v30;  // rax
    char *v31;  // rax
    char *v5;  // rax
    char *v32;  // rax
    char *v33;  // rax
    char *v34;  // rax
    char *v35;  // rax
    char *v36;  // rax
    char *v37;  // rax
    char *v38;  // rax
    char *v39;  // rax
    char *v40;  // rax
    char *v41;  // rax
    long long v6;  // rdi
    char *v42;  // rax
    char *v43;  // rax
    char *v44;  // rax
    char *v45;  // rax
    char *v46;  // rax
    char *v47;  // rax
    char *v48;  // rax
    char *v49;  // rax
    char *v50;  // rax
    char *v51;  // rax
    long long v7;  // rsi
    char *v52;  // rax
    char *v53;  // rax
    char *v54;  // rax
    long long v8;  // rdx
    long long v9;  // rcx
    long long v10;  // r8
    long long v11;  // r9
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    if (a0)
    {
        v3 = gettext("Try '%s --help' for more information.\n");
        fprintf(stderr, v3);
    }
    else
    {
        printf(gettext("Usage: %s [OPTION]... [FILE]...\n"));
        v5 = gettext("List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n");
        fputs_unlocked(v5, stdout);
        sub_404f20(v6, v7, v8, v9, v10, v11);
        v12 = gettext("  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n");
        fputs_unlocked(v12, stdout);
        v13 = gettext("      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., '--block-size=M'; see SIZE format below\n\n");
        fputs_unlocked(v13, stdout);
        v14 = gettext("  -B, --ignore-backups       do not list implied entries ending with ~\n");
        fputs_unlocked(v14, stdout);
        v15 = gettext("  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n");
        fputs_unlocked(v15, stdout);
        v16 = gettext("  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs' dired mode\n");
        fputs_unlocked(v16, stdout);
        v17 = gettext("  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append '*'\n");
        fputs_unlocked(v17, stdout);
        v18 = gettext("      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n");
        fputs_unlocked(v18, stdout);
        v19 = gettext("      --full-time            like -l --time-style=full-iso\n");
        fputs_unlocked(v19, stdout);
        v20 = gettext("  -g                         like -l, but do not list owner\n");
        fputs_unlocked(v20, stdout);
        v21 = gettext("      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n");
        fputs_unlocked(v21, stdout);
        v22 = gettext("  -G, --no-group             in a long listing, don't print group names\n");
        fputs_unlocked(v22, stdout);
        v23 = gettext("  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n");
        fputs_unlocked(v23, stdout);
        v24 = gettext("  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n");
        fputs_unlocked(v24, stdout);
        v25 = gettext("      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n");
        fputs_unlocked(v25, stdout);
        v26 = gettext("      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n");
        fputs_unlocked(v26, stdout);
        v27 = gettext("      --hyperlink[=WHEN]     hyperlink file names WHEN\n");
        fputs_unlocked(v27, stdout);
        v28 = gettext("      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n");
        fputs_unlocked(v28, stdout);
        v29 = gettext("  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n");
        fputs_unlocked(v29, stdout);
        v30 = gettext("  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n");
        fputs_unlocked(v30, stdout);
        v31 = gettext("  -l                         use a long listing format\n");
        fputs_unlocked(v31, stdout);
        v32 = gettext("  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n");
        fputs_unlocked(v32, stdout);
        v33 = gettext("  -m                         fill width with a comma separated list of entries\n");
        fputs_unlocked(v33, stdout);
        v34 = gettext("  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n");
        fputs_unlocked(v34, stdout);
        v35 = gettext("  -q, --hide-control-chars   print ? instead of nongraphic characters\n");
        fputs_unlocked(v35, stdout);
        v36 = gettext("      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is 'ls' and output is a terminal)\n\n");
        fputs_unlocked(v36, stdout);
        v37 = gettext("  -Q, --quote-name           enclose entry names in double quotes\n");
        fputs_unlocked(v37, stdout);
        v38 = gettext("      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n");
        fputs_unlocked(v38, stdout);
        v39 = gettext("  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n");
        fputs_unlocked(v39, stdout);
        v40 = gettext("  -S                         sort by file size, largest first\n");
        fputs_unlocked(v40, stdout);
        v41 = gettext("      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n");
        fputs_unlocked(v41, stdout);
        v42 = gettext("      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n");
        fputs_unlocked(v42, stdout);
        v43 = gettext("      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n");
        fputs_unlocked(v43, stdout);
        v44 = gettext("  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n");
        fputs_unlocked(v44, stdout);
        v45 = gettext("  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n");
        fputs_unlocked(v45, stdout);
        v46 = gettext("  -U                         do not sort; list entries in directory order\n");
        fputs_unlocked(v46, stdout);
        v47 = gettext("  -v                         natural sort of (version) numbers within text\n");
        fputs_unlocked(v47, stdout);
        v48 = gettext("  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n");
        fputs_unlocked(v48, stdout);
        v49 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v49, stdout);
        v50 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v50, stdout);
        sub_404f51(v6, v7, v8, v9, v10, v11);
        v51 = gettext("\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with 'posix-' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n");
        fputs_unlocked(v51, stdout);
        v52 = gettext("\nThe WHEN argument defaults to 'always' and can also be 'auto' or 'never'.\n");
        fputs_unlocked(v52, stdout);
        v53 = gettext("\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n");
        fputs_unlocked(v53, stdout);
        v54 = gettext("\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n");
        fputs_unlocked(v54, stdout);
        sub_404f82((g_42b210 == 1 ? "ls" : (g_42b210 == 2 ? "dir" : "vdir")));
    }
    exit(a0); /* do not return */
}



// Function: hash_get_n_buckets @ 0x11fd6
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0x11fec
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0x12002
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0x12018
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_get_max_bucket_length(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *iter;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v1 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v3 = 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                v3 += 1;
            }
            if (v3 > v1)
                v1 = v3;
        }
    }
    return v1;
}



// Function: hash_table_ok @ 0x1209a
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned int hash_table_ok(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long node;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v1 = 0;
    node = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v1 += 1;
            node += 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                node += 1;
            }
        }
    }
    if (v1 == a0[3] && node == a0[4])
        return 1;
    return 0;
}



// Function: hash_print_statistics @ 0x12138
typedef struct FILE {
} FILE;

int hash_print_statistics(void* a0, FILE *ptr)
{
    uint128_t v5;  // xmm0
    long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = sub_412002(a0);
    v1 = sub_411fd6(a0);
    v2 = sub_411fec(a0);
    v3 = sub_412018(a0);
    fprintf(ptr, "# entries:         %lu\n", v0);
    fprintf(ptr, "# buckets:         %lu\n", v1);
    v5 = (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1);
    fprintf(ptr, "# buckets used:    %lu (%.2f%%)\n", v2, (unsigned long long)(DivV(MulV(0x4059000000000000, (v2 < 0 ? AddV(v2 >> 1 | (unsigned int)v2 & 1, v2 >> 1 | (unsigned int)v2 & 1) : v2)), (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1))));
    return fprintf(ptr, "max bucket length: %lu\n", v3);
}



// Function: safe_hasher @ 0x1227f
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
    unsigned long long v0;  // [bp-0x10]

    v0 = a0->field_30(a1, a0->field_10, a0->field_10, a0->field_30);
    if (v0 < a0->field_10)
        return v0 * 16 + a0->field_0;
    abort(); /* do not return */
}



// Function: hash_lookup @ 0x122da
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
    struct_0 *iter;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v1 = sub_41227f(a0, a1);
    if (!v1->field_0)
        return 0;
    iter = v1;
    while (1)
    {
        if (!iter)
            return 0;
        if (a1 == iter->field_0 || (char)a0[1].field_0(a1, iter->field_0, iter->field_0, a0[1].field_0))
            break;
        iter = (struct_0 *)iter->padding_8;
    }
    return iter->field_0;
}



// Function: hash_get_first @ 0x12371
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[24];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_get_first(struct_0 *a0)
{
    unsigned long long *v0;  // [bp-0x10]

    if (!a0->field_20)
        return 0;
    v0 = &a0->field_0->field_0;
    while (1)
    {
        if (v0 >= *((long long *)&a0->padding_8[0]))
            abort(); /* do not return */
        if (*(v0))
            return *(v0);
        v0 += 2;
    }
}



// Function: hash_get_next @ 0x123d1
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

unsigned long long hash_get_next(struct_0 *a0, unsigned int a1)
{
    struct_0 *v0;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = sub_41227f(a0, a1);
    iter = v0;
    while (1)
    {
        if (a1 == iter->field_0 && iter->padding_8)
            return *((long long *)iter->padding_8);
        iter = (struct_0 *)iter->padding_8;
        if (!iter)
        {
            while (1)
            {
                v0 = &v0->field_10;
                if (v0 >= a0->padding_8)
                {
                    return 0;
                }
                else if (v0->field_0)
                {
                    return v0->field_0;
                }
            }
        }
    }
}



// Function: hash_get_entries @ 0x1246f
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_get_entries(struct_0 **a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long idx;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    idx = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; idx = v1)
            {
                if (idx >= a2)
                    return idx;
                v1 = idx + 1;
                *((unsigned long long *)(idx * 8 + a1)) = iter->field_0;
                iter = iter->field_8;
            }
        }
    }
    return idx;
}



// Function: hash_do_for_each @ 0x1250f
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

unsigned long long hash_do_for_each(struct_0 **a0, unsigned long long *a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; iter = iter->field_8)
            {
                if ((char)a1(iter->field_0, a2, a2, a1) != 1)
                    return v0;
                v0 += 1;
            }
        }
    }
    return v0;
}



// Function: hash_string @ 0x125a7
unsigned long long hash_string(char *a0, unsigned long long a1)
{
    char *iter;  // [bp-0x20]
    char v1;  // [bp-0x11]
    unsigned long long v2;  // [bp-0x10]

    iter = a0;
    v2 = 0;
    while (1)
    {
        v1 = *(iter);
        if (!v1)
            break;
        v2 = (v1 + v2 * 31) % a1;
        iter += 1;
    }
    return v2;
}



// Function: is_prime @ 0x12601
long long is_prime(unsigned long long a0)
{
    unsigned long long iter;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    iter = 3;
    for (v1 = iter * iter; v1 < a0 && a0 % iter; iter += 1)
    {
        iter += 1;
        v1 += iter * 4;
    }
    return _INSERT(a0 % iter, 0, a0 % iter);
}



// Function: next_prime @ 0x12670
unsigned long long next_prime(unsigned long long a0)
{
    unsigned long long i;  // [bp-0x10]

    i = a0;
    if (i <= 9)
        i = 10;
    for (i |= 1; i != 0xffffffffffffffff; i += 2)
    {
        if ((char)sub_412601(i) == 1)
            return i;
    }
    return i;
}



// Function: hash_reset_tuning @ 0x126bb
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
    char field_10;
} struct_0;

struct_0 * hash_reset_tuning(struct_0 *idx)
{
    idx->field_0 = 0;
    idx->field_4 = 0x3f800000;
    idx->field_8 = 1061997773;
    idx->field_c = 1068826100;
    idx->field_10 = 0;
    return idx;
}



// Function: raw_hasher @ 0x12711
long long raw_hasher(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_41e664(a0, 3);
    return v0 % a1;
}



// Function: raw_comparator @ 0x1274c
long long raw_comparator(unsigned long a0, unsigned long a1)
{
    return _INSERT(a0, 0, a0 == a1);
}



// Function: check_tuning @ 0x12769
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

extern unsigned int g_423470[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int v3;  // ymm1
    unsigned int v4;  // ymm1
    unsigned int v0;  // [bp-0x14]
    unsigned int *v1;  // [bp-0x10]

    v1 = &idx->field_28->field_0;
    if (v1 == &g_423470[0])
        return 1;
    v0 = 1036831949;
    if (((CmpF(v1[2], v0) & 69 | (char)((CmpF(v1[2], v0) & 69) >> 6)) & 1) != 1 && ((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69 | (char)((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69) >> 6)) & 1) != 1 && ((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69 | (char)((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69) >> 6)) & 1) != 1 && !(CmpF(*(v1), 0) & 1))
    {
        v3 = AddV(*(v1), v0);
        if (((CmpF(v1[1], v3) & 69 | (char)((CmpF(v1[1], v3) & 69) >> 6)) & 1) != 1 && !(CmpF(0x3f800000, v1[1]) & 1))
        {
            v4 = AddV(*(v1), v0);
            if (((CmpF(v1[2], v4) & 69 | (char)((CmpF(v1[2], v4) & 69) >> 6)) & 1) != 1)
                return 1;
        }
    }
    idx->field_28 = &g_423470[0];
    return 0;
}



// Function: compute_bucket_size @ 0x12876
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long compute_bucket_size(unsigned long a0, struct_0 *a1)
{
    char v3;  // al
    unsigned long v0;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xc]

    v0 = a0;
    if (a1->field_10 != 1)
    {
        v1 = DivV((v0 < 0 ? AddV((unsigned int)(v0 >> 1 | (unsigned int)v0 & 1), (unsigned int)(v0 >> 1 | (unsigned int)v0 & 1)) : (unsigned int)v0), a1->field_8);
        if (!(CmpF(v1, 0x5f800000) & 1))
            return 0;
        if (CmpF(v1, 0x5f000000) & 1)
        {
            v0 = v1;
        }
        else
        {
            v0 = (unsigned int)(SubV(v1, 0x5f000000));
            v0 ^= 0x8000000000000000;
        }
    }
    v0 = sub_412670(v0);
    v3 = 0;
    if (v0 & 0xe000000000000000)
        v3 = 1;
    if (v0 * 8 < 0)
        v3 = 1;
    if (!(v3 & 1))
        return v0;
    return 0;
}



// Function: hash_initialize @ 0x1298f
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

extern struct_0 g_423470;

struct_0 * hash_initialize(unsigned int a0, struct_0 *a1, unsigned long long a2, unsigned long long a3, unsigned long a4)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    struct_0 *v2;  // [bp-0x28]
    struct_0 *ptr;  // [bp-0x10]

    v2 = a1;
    v1 = a2;
    v0 = a3;
    if (!v1)
        v1 = sub_412711;
    if (!v0)
        v0 = sub_41274c;
    ptr = malloc(80);
    if (!ptr)
        return NULL;
    if (!v2)
        v2 = &g_423470.field_0;
    ptr->field_28 = v2;
    if ((char)sub_412769(ptr) == 1)
    {
        *((unsigned long long *)&ptr->field_10) = sub_412876(a0, v2);
        if (*((long long *)&ptr->field_10))
        {
            ptr->field_0 = calloc(*((long long *)&ptr->field_10), 16);
            if (ptr->field_0)
            {
                ptr->field_8 = ptr->field_0 + *((long long *)&ptr->field_10) * 16;
                ptr->field_18 = 0;
                ptr->field_20 = 0;
                ptr->field_30 = v1;
                ptr->field_38 = v0;
                ptr->field_40 = a4;
                ptr->field_48 = 0;
                return ptr;
            }
        }
    }
    free(ptr);
    return NULL;
}



// Function: hash_clear @ 0x12b04
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
    unsigned long long field_20;
    char padding_28[24];
    struct struct_3 *field_40;
    struct struct_0 *field_48;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

struct_2 * hash_clear(struct_2 *idx)
{
    struct_1 *iter;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    struct_0 *v2;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx1 = iter->field_8; idx1; idx1 = v2)
            {
                if (idx->field_40)
                    idx->field_40(idx1->field_0);
                idx1->field_0 = 0;
                v2 = idx1->field_8;
                idx1->field_8 = idx->field_48;
                idx->field_48 = idx1;
            }
            if (idx->field_40)
                idx->field_40(iter->field_0);
            iter->field_0 = 0;
            iter->field_8 = NULL;
        }
    }
    idx->field_18 = 0;
    idx->field_20 = 0;
    return idx;
}



// Function: hash_free @ 0x12c0f
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
    struct_0 *node;  // [bp-0x20]
    struct_1 *iter;  // [bp-0x18], Other Possible Types: struct_0 *
    struct_0 *v2;  // [bp-0x10], Other Possible Types: struct_1 *

    if (ptr->field_40 && ptr->field_20)
    {
        for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
        {
            if (node->field_0)
            {
                for (iter = node; iter; iter = iter->field_8)
                {
                    ptr->field_40(iter->field_0);
                }
            }
        }
    }
    for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
    {
        for (iter = node->field_8; iter; iter = v2)
        {
            v2 = iter->field_8;
            free(iter);
        }
    }
    for (iter = ptr->field_48; iter; iter = v2)
    {
        v2 = iter->field_8;
        free(iter);
    }
    free(ptr->field_0);
    free(ptr);
    return;
}



// Function: allocate_entry @ 0x12d3e
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
    struct_0 *ptr;  // [bp-0x10]

    if (!idx->field_48)
    {
        ptr = malloc(16);
        return ptr;
    }
    ptr = idx->field_48;
    idx->field_48 = ptr->field_8;
    return ptr;
}



// Function: free_entry @ 0x12d8d
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_2;

struct_2 * free_entry(struct_2 *idx, struct_0 *a1)
{
    a1->field_0 = 0;
    a1->field_8 = idx->field_48;
    idx->field_48 = a1;
    return idx;
}



// Function: hash_find_entry @ 0x12dc7
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[56];
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

unsigned long long hash_find_entry(struct_0 *a0, unsigned long long a1, struct_2 **a2, char a3)
{
    struct_2 *v7;  // rdx
    struct_2 *iter;  // [bp-0x38]
    struct_2 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    struct_2 *v5;  // [bp-0x10]

    index = sub_41227f(a0, a1);
    *(a2) = index;
    if (!index->field_0)
        return 0;
    if (a1 != index->field_0 && !(char)a0->field_38(a1, index->field_0, index->field_0, a0->field_38))
    {
        iter = index;
        while (1)
        {
            if (!iter->field_8)
                return 0;
            if (a1 == iter->field_8->field_0 || (char)a0->field_38(a1, iter->field_8->field_0, iter->field_8->field_0, a0->field_38))
                break;
            iter = iter->field_8;
        }
        v2 = iter->field_8->field_0;
        if (!a3)
            return v2;
        v3 = iter->field_8;
        iter->field_8 = v3->field_8;
        sub_412d8d(a0, v3);
        return v2;
    }
    v4 = index->field_0;
    if (!a3)
        return v4;
    if (index->field_8)
    {
        v5 = index->field_8;
        v7 = v5->field_8;
        index->field_0 = v5->field_0;
        index->field_8 = v7;
        sub_412d8d(a0, v5);
    }
    else
    {
        index->field_0 = 0;
    }
    return v4;
}



// Function: transfer_entries @ 0x12f60
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

unsigned int transfer_entries(struct_2 *index, struct_2 *idx, char a2)
{
    struct_1 *iter;  // [bp-0x38]
    struct_0 *idx2;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28], Other Possible Types: unsigned long long
    struct_0 *idx1;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]
    struct_0 *v5;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx2 = iter->field_8; idx2; idx2 = v5)
            {
                v2 = idx2->field_0;
                idx1 = sub_41227f(index, v2);
                v5 = idx2->field_8;
                if (idx1->field_0)
                {
                    idx2->field_8 = idx1->field_8;
                    idx1->field_8 = idx2;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                    sub_412d8d(index, idx2);
                }
            }
            v2 = iter->field_0;
            iter->field_8 = NULL;
            if (!a2)
            {
                idx1 = sub_41227f(index, v2);
                if (idx1->field_0)
                {
                    v4 = sub_412d3e(index);
                    if (!v4)
                        return 0;
                    v4->field_0 = v2;
                    v4->field_8 = idx1->field_8;
                    idx1->field_8 = v4;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                }
                iter->field_0 = 0;
                idx->field_18 = idx->field_18 - 1;
            }
        }
    }
    return 1;
}



// Function: hash_rehash @ 0x13134
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

unsigned int hash_rehash(struct_2 *index, unsigned int a1)
{
    unsigned long v0;  // [bp-0x78]
    struct_2 *idx;  // [bp-0x70]
    char v2;  // [bp-0x68], Other Possible Types: unsigned long
    unsigned long v3;  // [bp-0x60]
    unsigned long v4;  // [bp-0x58]
    unsigned long long v5;  // [bp-0x50]
    unsigned long long v6;  // [bp-0x48]
    unsigned long v7;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    unsigned long v10;  // [bp-0x28]
    unsigned long v11;  // [bp-0x20]

    v0 = sub_412876(a1, *((long long *)&index[1].padding_8[0]));
    if (!v0)
    {
        return 0;
    }
    else if (v0 == *((long long *)&index->padding_8[8]))
    {
        return 1;
    }
    else
    {
        idx = &v2;
        v2 = calloc(v0, 16);
        if (!v2)
            return 0;
        v4 = v0;
        v3 = v2 + v0 * 16;
        v5 = 0;
        v6 = 0;
        v7 = *((long long *)&index[1].padding_8[0]);
        v8 = *((long long *)&index[1].padding_8[8]);
        v9 = index[1].field_18;
        v10 = index[2].field_0;
        v11 = *((long long *)&index[2].padding_8[0]);
        if ((char)sub_412f60(idx, index, 0))
        {
            free(index->field_0);
            index->field_0 = v2;
            *((unsigned long *)&index->padding_8[0]) = v3;
            *((unsigned long *)&index->padding_8[8]) = v4;
            index->field_18 = v5;
            *((unsigned long *)&index[2].padding_8[0]) = v11;
            return 1;
        }
        *((unsigned long *)&index[2].padding_8[0]) = v11;
        if ((char)sub_412f60(index, idx, 1) == 1 && (char)sub_412f60(index, idx, 0) == 1)
        {
            free(v2);
            return 0;
        }
        abort(); /* do not return */
    }
}



// Function: hash_insert_if_absent @ 0x1335f
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned int hash_insert_if_absent(void* idx, unsigned long long a1, unsigned long long *a2)
{
    unsigned long v6;  // rax
    unsigned int v7;  // ymm0
    unsigned long v16;  // rax
    unsigned int v17;  // ymm0
    unsigned long v18;  // rax
    int v8;  // xmm2
    unsigned long v9;  // rax
    unsigned long v11;  // rax
    unsigned int v12;  // ymm0
    int v13;  // xmm2
    unsigned long v14;  // rax
    unsigned int v0;  // [bp-0x34]
    struct_0 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]

    if (!a1)
        abort(); /* do not return */
    v2 = sub_412dc7(idx, a1, &index, 0);
    if (v2)
    {
        if (!a2)
            return 0;
        *(a2) = v2;
        return 0;
    }
    else
    {
        v6 = (long long)idx[24];
        v7 = (v6 < 0 ? (unsigned int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1))) : (int)v6);
        v8 = (int)*((int *)((long long)idx[40] + 8));
        v9 = (long long)idx[16];
        if (...)
        {
            sub_412769(idx);
            v11 = (long long)idx[24];
            v12 = (v11 < 0 ? (unsigned int)(AddV((unsigned int)(v11 >> 1 | (unsigned int)v11 & 1), (unsigned int)(v11 >> 1 | (unsigned int)v11 & 1))) : (int)v11);
            v13 = (int)*((int *)((long long)idx[40] + 8));
            v14 = (long long)idx[16];
            if (...)
            {
                v3 = (long long)idx[40];
                if ((char)v3[1].field_0)
                {
                    v16 = (long long)idx[16];
                    v17 = MulV((v16 < 0 ? AddV((unsigned int)(v16 >> 1 | (unsigned int)v16 & 1), (unsigned int)(v16 >> 1 | (unsigned int)v16 & 1)) : (unsigned int)v16), *((int *)((char *)&v3->field_8 + 4)));
                }
                else
                {
                    v18 = (long long)idx[16];
                    v17 = MulV((int)v3->field_8, MulV(*((int *)((char *)&v3->field_8 + 4)), (v18 < 0 ? AddV((unsigned int)(v18 >> 1 | (unsigned int)v18 & 1), (unsigned int)(v18 >> 1 | (unsigned int)v18 & 1)) : (unsigned int)v18)));
                }
                v0 = v17;
                if (!(CmpF(v0, 0x5f800000) & 1))
                {
                    return 0xffffffff;
                }
                else if ((char)sub_413134(idx, (CmpF(v0, 0x5f000000) & 69 & 1 ? (unsigned int)v0 : (unsigned int)(SubV(v0, 0x5f000000)) ^ 0)) != 1)
                {
                    return 0xffffffff;
                }
                else if (sub_412dc7(idx, a1, &index, 0))
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
        v4 = sub_412d3e(idx);
        if (v4)
        {
            v4->field_0 = a1;
            v4->field_8 = index->field_8;
            index->field_8 = v4;
            *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
            return 1;
        }
        return 0xffffffff;
    }
}



// Function: hash_insert @ 0x136d0
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]

    v0 = sub_41335f(a0, a1, &v1);
    if (v0 == 0xffffffff)
    {
        return 0;
    }
    else if (!v0)
    {
        return v1;
    }
    else
    {
        return a1;
    }
}



// Function: hash_remove @ 0x13740
typedef struct struct_1 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

unsigned long long hash_remove(struct_2 *idx, unsigned long long a1)
{
    unsigned long v7;  // rax
    unsigned int v8;  // ymm0
    unsigned long v17;  // rax
    int v18;  // xmm0
    unsigned long long v19;  // rax
    unsigned long v20;  // rax
    int v21;  // xmm0
    int v9;  // xmm2
    unsigned long v10;  // rax
    unsigned long v12;  // rax
    unsigned int v13;  // ymm0
    int v14;  // xmm2
    unsigned long v15;  // rax
    struct_2 *v0;  // [bp-0x40]
    struct_1 *ptr;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    struct_0 *v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    struct_1 *v5;  // [bp-0x18]

    v2 = sub_412dc7(idx, a1, &v0, 1);
    if (!v2)
        return 0;
    idx[2].field_0 = idx[2].field_0 - 1;
    if (v0->field_0)
        return v2;
    idx[1].field_8 = (char *)idx[1].field_8 - 1;
    v7 = idx[1].field_8;
    v8 = (v7 < 0 ? (unsigned int)(AddV((unsigned int)(v7 >> 1 | (unsigned int)v7 & 1), (unsigned int)(v7 >> 1 | (unsigned int)v7 & 1))) : (int)v7);
    v9 = (int)(unsigned int)idx[2].field_8->field_0;
    v10 = idx[1].field_0;
    if (...)
    {
        sub_412769(idx);
        v12 = idx[1].field_8;
        v13 = (v12 < 0 ? (unsigned int)(AddV((unsigned int)(v12 >> 1 | (unsigned int)v12 & 1), (unsigned int)(v12 >> 1 | (unsigned int)v12 & 1))) : (int)v12);
        v14 = (int)(unsigned int)idx[2].field_8->field_0;
        v15 = idx[1].field_0;
        if (...)
        {
            v3 = idx[2].field_8;
            if (v3->field_10)
            {
                v17 = idx[1].field_0;
                v18 = (int)(MulV((v17 < 0 ? AddV((unsigned int)(v17 >> 1 | (unsigned int)v17 & 1), (unsigned int)(v17 >> 1 | (unsigned int)v17 & 1)) : (unsigned int)v17), v3->field_4));
                v19 = (CmpF(*((unsigned int *)&v18), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v18) : *((unsigned int *)&SubV(v18, 0x5f000000)) ^ 0x8000000000000000);
            }
            else
            {
                v20 = idx[1].field_0;
                v21 = (int)(MulV(v3->field_8, MulV(v3->field_4, (v20 < 0 ? AddV((unsigned int)(v20 >> 1 | (unsigned int)v20 & 1), (unsigned int)(v20 >> 1 | (unsigned int)v20 & 1)) : (unsigned int)v20))));
                v19 = (CmpF(*((unsigned int *)&v21), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v21) : *((unsigned int *)&SubV(v21, 0x5f000000)) ^ 0x8000000000000000);
            }
            v4 = v19;
            if ((char)sub_413134(idx, v4) != 1)
            {
                for (ptr = idx[4].field_8; ptr; ptr = v5)
                {
                    v5 = ptr->field_8;
                    free(ptr);
                }
                idx[4].field_8 = NULL;
            }
        }
    }
    return v2;
}



// Function: hash_delete @ 0x13a46
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned long long hash_delete(struct_2 *a0, unsigned long long a1)
{
    return sub_413740(a0, a1);
}



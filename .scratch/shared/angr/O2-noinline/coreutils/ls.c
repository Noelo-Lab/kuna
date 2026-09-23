// Function: main @ 0x4dd0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char field_0;
} struct_1;

typedef struct struct_2 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_2;

extern char g_41abb9;
extern void g_41add7;
extern struct_2 g_4259e0;
extern unsigned long long g_426060;
extern void* g_426068;
extern unsigned long long g_426070;
extern struct_1 *g_426078;
extern FILE *stdout;
extern long long g_4270e0;
extern long long g_427140;
extern long long g_4271a0;
extern unsigned int g_427210;
extern unsigned int g_427214;
extern int g_427218;
extern char g_4272a0;
extern char g_4272a1;
extern unsigned long long g_4272a8;
extern char g_4272b8;
extern unsigned long long g_4272c0;
extern long long g_4272d0;
extern char g_4272f4;
extern char g_4272f5;
extern char g_4272f6;
extern unsigned int g_4272f8;
extern char g_4272fd;
extern char g_427310;
extern char g_427311;
extern char g_427312;
extern char g_427314;
extern char g_427318;
extern char g_42732c;
extern unsigned int g_427330;
extern char g_427338;
extern char g_427365;
extern unsigned long long g_427370;
extern unsigned long long g_427378;
extern struct_0 *g_427380;
extern unsigned long long g_427388;
extern char g_427390;
extern unsigned long long g_4273b0;
extern unsigned long long g_4273b8;
extern unsigned long long g_4273c0;
extern unsigned long long g_4273c8;

int main(unsigned int i, long long *a1)
{
    long long v3;  // rdi
    long long v4;  // rsi
    unsigned long v13;  // 4111
    unsigned long v14;  // rax
    unsigned long v15;  // rax
    unsigned int v16;  // r13d
    unsigned long v17;  // rbx
    unsigned long long v18;  // rbp
    unsigned long long v19;  // rax
    unsigned long long v20;  // rdi
    long long v21;  // rax
    unsigned int v22;  // ebx
    long long v5;  // rdx
    unsigned int j;  // ebx
    unsigned int v24;  // eax
    long long v6;  // rcx
    long long v7;  // r8
    long long v8;  // r9
    unsigned long v9;  // rbx
    unsigned long v10;  // 4110
    unsigned int v11;  // eax
    char v12;  // al
    unsigned long v0;  // [bp-0x58]
    long long v1;  // [bp-0x50]

    sub_413390(*(a1));
    setlocale(6, &g_41abb9);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_408dd0(v3, v4, v5, v6, v7, v8);
    sub_418230(sub_40df00);
    g_427210 = 0;
    g_4272b8 = 1;
    g_427380 = 0;
    g_427370 = 0x8000000000000000;
    g_427378 = 0xffffffffffffffff;
    v9 = sub_40bda0(i, a1);
    if (!(!g_427312 || !(sub_406af0(), g_427312)))
    {
        v10 = g_4272f4;
        g_4272c0 = 0;
        if ((char)v10 || (char)sub_406420(13) || (char)sub_406420(14) && g_427390 || (char)sub_406420(12) && !*((int *)&g_427338))
        {
LABEL_4051c8:
            g_4272fd = 1;
        }
    }
    else if (!(!g_4272f4))
    {
        goto LABEL_4051c8;
    }
    if (!g_4272f8)
    {
        v11 = 1;
        if (!g_4272f5 && *((int *)&g_427314) != 3)
            v11 = (-(*((int *)&g_427338) < 1) & 0xfffffffe) + 3;
        g_4272f8 = v11;
    }
    if (g_4272f6)
    {
        g_4273c8 = sub_40f380(30, 0, sub_4056d0, sub_4056e0, sub_4061a0);
        if (!g_4273c8)
            sub_416da0(); /* do not return */
        _obstack_begin(&g_4270e0, 0, 0, malloc, free);
    }
    g_4272a8 = sub_415d80(getenv("TZ"));
    if (!(g_427330 - 3 & 0xfffffffd) || !*((int *)&g_427338) || g_427365 || g_42732c)
    {
        g_4272a1 = 1;
        v12 = 0;
    }
    else
    {
        v13 = g_4272f6;
        g_4272a1 = 0;
        v12 = 1;
        if (!(char)v13 && !g_427312 && !*((int *)&g_427314))
            v12 = g_4272f4;
    }
    g_4272a0 = v12;
    g_4272a0 = g_4272a0 & 1;
    if (g_427318)
    {
        _obstack_begin(&g_4271a0, 0, 0, malloc, free);
        _obstack_begin(&g_427140, 0, 0, malloc, free);
    }
    if (g_427311)
    {
        sub_405670();
        v14 = sub_416f20(v3, v4, v5, v6, v7, v8);
        if (!v14)
            v14 = &g_41abb9;
        g_427388 = v14;
    }
    g_4273b8 = 100;
    v15 = sub_416b00(100, 208);
    v16 = i - v9;
    g_4273b0 = 0;
    g_4273c0 = v15;
    sub_406e20(v3, v4, v5, v6, v7, v8);
    if (v16 > 0)
    {
        do
        {
            v17 = v9 + 1;
            sub_40a550(a1[v9], 0, 1, &g_41abb9);
            v9 = v17;
        } while (i > (unsigned int)v9);
        if (!g_4273b0)
            goto LABEL_404fe9;
    }
    else
    {
        if (g_4272f5)
            sub_40a550(".", 3, 1, &g_41abb9);
        else
            sub_406840(".", 0, 1);
        v18 = g_427380;
        if (!g_4273b0)
            goto LABEL_405292;
    }
    sub_4089f0();
    if (!g_4272f5)
        sub_40a1e0(0, 1);
    if (!g_4273b0)
    {
LABEL_404fe9:
        v19 = g_427380;
        v18 = g_427380;
        if (v16 > 1)
            goto LABEL_404ffd;
LABEL_405292:
        if (v18)
        {
            if (!*((long long *)(v18 + 24)))
                g_4272b8 = 0;
LABEL_405009:
            while (1)
            {
                v20 = *((long long *)v18);
                g_427380 = *((long long *)(v18 + 24));
                if (!g_4273c8 || v20)
                {
                    sub_40b180(v20, *((long long *)(v18 + 8)), *((char *)(v18 + 16)));
                    sub_4061b0(v18);
                    v18 = g_427380;
                    g_4272b8 = 1;
                    if (!g_427380)
                        break;
                }
                else
                {
                    v0 = sub_406330();
                    v1 = v5;
                    v21 = sub_40fa00(g_4273c8, &v0);
                    if (!v21)
                        __assert_fail(); /* do not return */
                    sub_4061a0(v21);
                    sub_4061b0(v18);
                    v18 = g_427380;
                    if (!g_427380)
                        break;
                }
            }
        }
    }
    else
    {
        sub_409f20();
        if (g_427380)
        {
            sub_407460(10);
            v19 = g_427380;
LABEL_404ffd:
            v18 = v19;
            if (v19)
                goto LABEL_405009;
        }
    }
    if (g_427312 && g_427310)
    {
        if (g_426060 != 2 || memcmp(g_426068, &g_41add7, 2) || g_426070 != 1 || g_426078->field_0 != 109)
            sub_407cc0();
        fflush_unlocked(stdout);
        sub_407dd0(v3, v4, v5, v6, v7, v8);
        v22 = g_427214;
        if (g_427214)
        {
            do
            {
                j = v22;
                raise(19);
                v22 = j - 1;
            } while (j != 1);
        }
        if (g_427218)
            raise(g_427218);
    }
    if (g_427318)
    {
        sub_408060("//DIRED//", &g_4271a0);
        sub_408060("//SUBDIRED//", &g_427140);
        v24 = sub_4151e0(g_4272d0);
        __printf_chk(1, "//DIRED-OPTIONS// --quoting-style=%s\n", (&g_4259e0.field_0)[v24]);
    }
    if (g_4273c8)
    {
        if (!sub_40ef20(g_4273c8))
            sub_40f540(g_4273c8);
        else
            __assert_fail(); /* do not return */
    }
    return g_427210;
}



// Function: file_or_link_mode @ 0x55e0
typedef struct struct_0 {
    char padding_0[48];
    unsigned int field_30;
    char padding_34[120];
    unsigned int field_ac;
    char padding_b0[9];
    char field_b9;
} struct_0;

extern char g_427390;

int file_or_link_mode(struct_0 *a0)
{
    if (g_427390 && a0->field_b9)
        return a0->field_ac;
    return a0->field_30;
}



// Function: first_percent_b @ 0x5620
unsigned int first_percent_b(char *iter)
{
    char v1;  // dl
    char v2;  // cl

    v1 = *(iter);
    if (!*(iter))
        return 0;
    while (1)
    {
        v2 = iter[1];
        if (v1 != 37)
        {
LABEL_405639:
            v1 = v2;
            goto LABEL_40563b;
        }
        else if (v2 != 37)
        {
            if (v2 == 98)
                return *((unsigned int *)&iter);
            goto LABEL_405639;
        }
        else
        {
            v1 = iter[2];
            iter += 1;
LABEL_40563b:
            iter += 1;
            if (!v1)
                return 0;
        }
    }
}



// Function: file_escape_init @ 0x5670
extern char g_426fe0;

unsigned int file_escape_init(void)
{
    unsigned long long iter;  // rbx
    unsigned int v2;  // eax
    unsigned long long v3;  // rax

    iter = 0;
    do
    {
        v2 = 1;
        if (!(char)(sub_40d3f0(iter & 0xffffffff) & 0xffffffff))
        {
            v3 = iter - 45 & 0xffffffff;
            if ((unsigned int)v3 > 1 && (unsigned int)iter != 126)
                v2 = _INSERT(v3, 0, (unsigned int)iter == 95);
            else
                v2 = 1;
        }
    } while (((&g_426fe0)[iter] = (&g_426fe0)[iter] | (char)v2, iter += 1, iter != 0x100));
    return v2;
}



// Function: dev_ino_hash @ 0x56d0
void dev_ino_hash(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: dev_ino_compare @ 0x56e0
unsigned long long dev_ino_compare(unsigned long long *a0, unsigned long a1)
{
    if (*(a0) == *((long long *)a1))
        return _INSERT(*((long long *)(a1 + 8)), 0, a0[1] == *((long long *)(a1 + 8)));
    return 0;
}



// Function: sighandler @ 0x5700
extern unsigned int g_427218;

long long sighandler(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_427218;
    if (!(unsigned int)v1)
        g_427218 = a0;
    return v1;
}



// Function: stophandler @ 0x5720
extern unsigned int g_427214;
extern unsigned int g_427218;

long long stophandler(void)
{
    unsigned long v1;  // rax

    v1 = g_427218;
    if (!g_427218)
    {
        v1 = g_427214 + 1;
        g_427214 = v1;
    }
    return v1;
}



// Function: get_funky_string @ 0x5740
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_0;


unsigned int get_funky_string(void* *a0, struct_0 **a1, unsigned int a2, unsigned long long *a3)
{
    unsigned int v1;  // eax
    struct_0 *node;  // rcx
    unsigned long long iter;  // rdi
    unsigned int v4;  // edx
    unsigned long long v5;  // r8
    void* v7;  // r9

    v1 = a2;
    node = *(a1);
    iter = 1;
    v4 = node->field_0;
    v5 = 0;
    v7 = *(a0) - 0;
    if ((char)v4 != 92)
        if ((char)v4 <= 92)
            goto LABEL_0x40577c;
        else
            goto LABEL_0x4057f0;
}



// Function: unsigned_file_size @ 0x59e0
long long unsigned_file_size(unsigned long a0)
{
    return a0;
}



// Function: errno_unsupported @ 0x59f0
unsigned long long errno_unsupported(unsigned int a0)
{
    if (!(a0 - 22 & 0xffffffef))
        return 1;
    return sub_4055c0();
}



// Function: is_linked_directory @ 0x5a10
unsigned long long is_linked_directory(unsigned int *a0)
{
    unsigned int v1;  // edx
    unsigned long long v2;  // rax
    unsigned long long v3;  // rax

    v1 = a0[42];
    v2 = _INSERT(0, 0, v1 == 3 | v1 == 9);
    if (v1 != 3 && v1 != 9)
    {
        v3 = a0[43] & 0xf000;
        v2 = _INSERT(v3, 0, (unsigned int)v3 == 0x4000);
    }
    return v2;
}



// Function: dirfirst_check @ 0x5a40
long long dirfirst_check(unsigned long a0, unsigned int *a1, unsigned long a2)
{
    unsigned int *v1;  // r9
    char v2;  // al
    char v3;  // cl
    unsigned long v4;  // r10

    sub_405a10(a1);
    v2 = sub_405a10(v1);
    if (v3 != v2)
        return v3 - v2;
    goto *((void *)(v4));
}



// Function: off_cmp @ 0x5c40
unsigned int off_cmp(long long a0, long long a1)
{
    if (a0 >= a1)
        return a1 < a0;
    return 0xffffffff;
}



// Function: xstrcoll_ctime @ 0x5c60
void xstrcoll_ctime(void)
{
}



// Function: strcmp_ctime @ 0x5c70
void strcmp_ctime(void)
{
}



// Function: rev_xstrcoll_ctime @ 0x5c80
void rev_xstrcoll_ctime(unsigned long a0, unsigned long a1)
{
}



// Function: rev_strcmp_ctime @ 0x5ca0
void rev_strcmp_ctime(unsigned long a0, unsigned long a1)
{
}



// Function: xstrcoll_df_ctime @ 0x5cc0
long long xstrcoll_df_ctime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405c60);
}



// Function: strcmp_df_ctime @ 0x5cd0
long long strcmp_df_ctime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405c70);
}



// Function: rev_xstrcoll_df_ctime @ 0x5ce0
long long rev_xstrcoll_df_ctime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405c80);
}



// Function: rev_strcmp_df_ctime @ 0x5cf0
long long rev_strcmp_df_ctime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405ca0);
}



// Function: xstrcoll_atime @ 0x5da0
void xstrcoll_atime(void)
{
}



// Function: strcmp_atime @ 0x5db0
void strcmp_atime(void)
{
}



// Function: rev_xstrcoll_atime @ 0x5dc0
void rev_xstrcoll_atime(unsigned long a0, unsigned long a1)
{
}



// Function: rev_strcmp_atime @ 0x5de0
void rev_strcmp_atime(unsigned long a0, unsigned long a1)
{
}



// Function: xstrcoll_df_atime @ 0x5e00
long long xstrcoll_df_atime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405da0);
}



// Function: strcmp_df_atime @ 0x5e10
long long strcmp_df_atime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405db0);
}



// Function: rev_xstrcoll_df_atime @ 0x5e20
long long rev_xstrcoll_df_atime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405dc0);
}



// Function: rev_strcmp_df_atime @ 0x5e30
long long rev_strcmp_df_atime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405de0);
}



// Function: xstrcoll_btime @ 0x5e40
void xstrcoll_btime(void)
{
}



// Function: strcmp_btime @ 0x5e50
void strcmp_btime(void)
{
}



// Function: rev_xstrcoll_btime @ 0x5e60
void rev_xstrcoll_btime(unsigned long a0, unsigned long a1)
{
}



// Function: rev_strcmp_btime @ 0x5e80
void rev_strcmp_btime(unsigned long a0, unsigned long a1)
{
}



// Function: xstrcoll_df_btime @ 0x5ea0
long long xstrcoll_df_btime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405e40);
}



// Function: strcmp_df_btime @ 0x5eb0
long long strcmp_df_btime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405e50);
}



// Function: rev_xstrcoll_df_btime @ 0x5ec0
long long rev_xstrcoll_df_btime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405e60);
}



// Function: rev_strcmp_df_btime @ 0x5ed0
long long rev_strcmp_df_btime(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_405e80);
}



// Function: xstrcoll_df_size @ 0x5ee0
extern unsigned int g_40a1a0;

long long xstrcoll_df_size(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, &g_40a1a0);
}



// Function: strcmp_df_size @ 0x5ef0
extern unsigned int g_40a170;

long long strcmp_df_size(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, &g_40a170);
}



// Function: rev_xstrcoll_df_size @ 0x5f00
extern unsigned int g_40a150;

long long rev_xstrcoll_df_size(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, &g_40a150);
}



// Function: rev_strcmp_df_size @ 0x5f10
extern unsigned int g_40a130;

long long rev_strcmp_df_size(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, &g_40a130);
}



// Function: rev_strcmp_df_extension @ 0x5f90
long long rev_strcmp_df_extension(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_407350);
}



// Function: xstrcoll_df_width @ 0x5fa0
long long xstrcoll_df_width(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_408970);
}



// Function: strcmp_df_width @ 0x5fb0
long long strcmp_df_width(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_408940);
}



// Function: rev_xstrcoll_df_width @ 0x5fc0
long long rev_xstrcoll_df_width(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_408950);
}



// Function: rev_strcmp_df_width @ 0x5fd0
long long rev_strcmp_df_width(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_408920);
}



// Function: xstrcoll_df_version @ 0x5fe0
long long xstrcoll_df_version(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_40a050);
}



// Function: rev_xstrcoll_df_version @ 0x5ff0
long long rev_xstrcoll_df_version(unsigned int a0, unsigned int *a1)
{
    return sub_405a40(a0, a1, sub_40a060);
}



// Function: initialize_ordering_vector @ 0x6000
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 *g_4273a0;
extern unsigned long long g_4273b0;
extern unsigned long long g_4273c0;

unsigned long long * initialize_ordering_vector(void)
{
    unsigned long long *iter;  // rax
    unsigned long long v2;  // rdx
    unsigned long long *v3;  // rcx
    unsigned long long *v4;  // rax

    if (!g_4273b0)
        return v4;
    iter = g_4273a0;
    v2 = g_4273c0;
    v3 = &(g_4273a0->padding_0)[g_4273b0];
    do
    {
        *(iter) = v2;
        iter += 1;
        v2 += 208;
    } while (iter != v3);
    return iter;
}



// Function: get_type_indicator @ 0x6040
extern unsigned int g_427314;

unsigned int get_type_indicator(char a0, unsigned int a1, unsigned int a2)
{
    unsigned int v1;  // ecx
    char v2;  // cl
    unsigned int v3;  // esi
    unsigned int v4;  // eax

    if (a0)
    {
        v1 = a1 & 0xf000;
        switch (v1)
        {
        case 32768:
            if (g_427314 != 3)
                return 0;
            return -(0 < (a1 & 73)) & 42;
        case 16384:
            return 47;
        default:
            v2 = v1 == 0xa000;
            if (g_427314 == 1)
                return 0;
        }
    }
    else
    {
        switch (a2)
        {
        case 5:
            return 0;
        case 3:
            return 47;
        case 9:
            return 47;
        default:
            v2 = a2 == 6;
            if (g_427314 == 1)
                return 0;
        }
    }
    if (v2)
        return 64;
    if (a0)
    {
        v3 = a1 & 0xf000;
        if (v3 == 0x1000)
            return 124;
        v4 = v3 == 0xc000;
    }
    else if (a2 != 1)
    {
        v4 = a2 == 7;
    }
    else
    {
        return 124;
    }
    return -(v4) & 61;
}



// Function: attach @ 0x6140
long long attach(char *iter, void* a1, char *a2)
{
    unsigned long v5;  // rax

    do
    {
        *(iter) = v5;
        iter += 1;
        a2 += 1;
    } while ((v5 = (unsigned long)(unsigned long long)*(a2), (char)v5));
    *(iter) = 0;
    return v5;
}



// Function: free_pending_ent @ 0x61b0
void free_pending_ent(void* *a0)
{
    free(*(a0));
    free(a0[1]);
    free(a0);
    return;
}



// Function: init_column_info @ 0x61d0
extern unsigned long long g_4262e0;
extern char g_427200;
extern long long g_427208;

unsigned long long init_column_info(unsigned long long a0)
{
    unsigned long v1;  // rax
    unsigned long long v2;  // rbp
    unsigned long long j;  // rcx
    unsigned long long v12;  // rax
    unsigned long long iter;  // rax
    uint128_t v3;  // rax
    unsigned long v4;  // rdx
    unsigned long long v5;  // rdx
    unsigned long long v6;  // 4155
    unsigned long long v7;  // 4160
    unsigned long long v8;  // rax
    unsigned long long i;  // rdx
    unsigned long long v10;  // rsi

    if (g_4262e0 < a0)
    {
        if (!*((long long *)&g_427200) || *((long long *)&g_427200) >> 1 > a0)
        {
            v2 = a0 * 2;
            g_427208 = sub_416ad0(g_427208, a0, 48);
        }
        else
        {
            v1 = sub_416ad0();
            v2 = *((long long *)&g_427200);
            g_427208 = v1;
        }
        v3 = v2 - g_4262e0;
        v4 = g_4262e0 + 1;
        v5 = v4 + v2;
        v6 = __CFADD__(v4, v2);
        v7 = __OFUMUL__(v3, v5);
        if ((char)v6 & 1 || (char)v7 & 1)
            sub_416da0(); /* do not return */
        v8 = sub_416b00((unsigned long long)(v3 * v5) >> 1, 8);
        if (v2 > g_4262e0)
        {
            i = g_4262e0 * 8 + 8;
            do
            {
                *((unsigned long long *)(g_427208 + i * 3 - 8)) = v8;
                v8 += i;
                i += 8;
            } while (i != v2 * 8 + 8);
        }
        g_4262e0 = v2;
    }
    if (!a0)
        return v8;
    v10 = 0;
    j = 3;
    do
    {
        v12 = *((long long *)(g_427208 + j * 8 - 8));
        *((char *)(g_427208 + j * 8 - 24)) = 1;
        v10 += 8;
        *((unsigned long long *)(g_427208 + j * 8 - 16)) = j;
        iter = v12;
        do
        {
            *((unsigned long long *)iter) = 3;
            iter += 8;
        } while (v10 + v12 != iter);
        j += 3;
    } while (a0 * 3 + 3 != j);
    return iter;
}



// Function: dev_ino_pop @ 0x6330
extern unsigned long long g_4270f0;
extern void* g_4270f8;

long long dev_ino_pop(void)
{
    void* v0;  // [bp-0x8]

    if (g_4270f8 - g_4270f0 <= 15)
    {
        v0 = g_4270f8;
        __assert_fail(); /* do not return */
    }
    g_4270f8 = v0 - 16;
    return *((long long *)((char *)v0 - 16));
}



// Function: add_ignore_pattern @ 0x6380
extern unsigned long long g_4272e8;

void add_ignore_pattern(unsigned long a0)
{
    unsigned long long *v1;  // rax

    v1 = sub_4169e0(16);
    *(v1) = a0;
    v1[1] = g_4272e8;
    g_4272e8 = v1;
    return;
}



// Function: decode_line_length @ 0x63b0
long long decode_line_length(void)
{
    unsigned int v2;  // eax
    long long v0;  // [bp-0x18]

    v2 = sub_417190();
    if (v2)
        return -(v2 != 1);
    if (v0 < 0)
        v0 = 0;
    return v0;
}



// Function: is_colored @ 0x6420
extern unsigned long long g_426060[4];

unsigned long long is_colored(unsigned int a0)
{
    unsigned long v1;  // rdx

    v1 = g_426060[2 * a0];
    if (!v1)
    {
        return 0;
    }
    else if (v1 == 1)
    {
        return _INSERT(0, 0, strncmp(g_426060[1 + 2 * a0], "0", 1)) & 0xffffffff;
    }
    else if (v1 != 2)
    {
        return 1;
    }
    else
    {
        return _INSERT(0, 0, strncmp(g_426060[1 + 2 * a0], "00", 2)) & 0xffffffff;
    }
}



// Function: getenv_quoting_style @ 0x64b0
extern unsigned int g_41d9c0[4];
extern long long g_4259e0;

unsigned int getenv_quoting_style(void)
{
    unsigned long v1;  // rax
    unsigned int v2;  // eax

    v1 = getenv("QUOTING_STYLE");
    if (!v1)
        return 0xffffffff;
    v2 = sub_40cf70(v1, &g_4259e0, &g_41d9c0[0], 4);
    if (v2 < 0)
    {
        sub_4158c0(v1);
        error(0, 0, dcgettext(NULL, "ignoring invalid value of environment variable QUOTING_STYLE: %s", 5));
        return 0xffffffff;
    }
    return g_41d9c0[v2];
}



// Function: stdout_isatty @ 0x6590
extern char g_426018;

unsigned int stdout_isatty(void)
{
    unsigned int v1;  // eax

    v1 = g_426018;
    if ((char)v1 < 0)
    {
        g_426018 = isatty(1);
        return g_426018 & 1;
    }
    return v1 & 1;
}



// Function: abmon_init @ 0x65c0
unsigned int abmon_init(long long a0)
{
    unsigned long v3;  // r13
    unsigned int i;  // r15d
    unsigned long v5;  // rbx
    long long v6;  // r12
    char *v7;  // rbp
    long long v8;  // rax
    long long v0;  // [bp-0x50]
    unsigned long v1;  // [bp-0x48]

    v3 = 12;
    v0 = a0;
    while (1)
    {
        i = 131086;
        v5 = 0;
        v6 = v0;
        do
        {
            v1 = v3;
            v7 = nl_langinfo(i);
            if (strchr(v7, 37))
                return 0;
            v8 = __ctype_b_loc();
            sub_405570(*(v7), v6, v8, &v1);
            if (sub_410df0(v7) > 127)
                return 0;
            if (v5 < v1)
                v5 = v1;
            i += 1;
            v6 += 128;
        } while (i != 131098);
        if (v3 <= v5)
            break;
        v3 = v5;
    }
    return 1;
}



// Function: abformat_init @ 0x66c0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 g_426040;
extern char *g_426048;
extern char g_4263c8;
extern char g_4263e0;

void abformat_init(void)
{
    unsigned long long v10;  // rsi
    unsigned long long idx;  // r12
    unsigned long v12;  // r13
    unsigned long v13;  // r15
    unsigned long v14;  // r14
    char *v15;  // rbx
    unsigned long v0;  // [bp-0x688]
    char *v1;  // [bp-0x680]
    unsigned long v2;  // [bp-0x678]
    char *v3;  // [bp-0x668]
    char *v4;  // [bp-0x660]
    unsigned long v5;  // [bp-0x658]
    unsigned long long v6;  // [bp-0x650]
    char v7;  // [bp-0x648]
    char v8;  // [bp-0x48]

    v5 = sub_405620(g_426040.padding_0);
    v6 = sub_405620(g_426048);
    if (!(v10 || v6))
    {
        return;
    }
    else if ((char)sub_4065c0(&v7))
    {
        v4 = &v7;
        idx = 0;
        v3 = &v5;
        while (1)
        {
            v12 = *((long long *)&g_426040.padding_0[8 * idx]);
            v13 = *((long long *)&v3[8 * idx]);
            v14 = &(&g_4263e0)[0x600 * idx];
            v15 = v4;
            do
            {
                if (v13)
                {
                    if (v13 - v12 <= 128)
                    {
                        v2 = v13 + 2;
                        v1 = v15;
                        v0 = v12;
                        if (__snprintf_chk(v14, 128, 1, -0x1, "%.*s%s%s") > 127)
                            return;
                    }
                    else
                    {
                        return;
                    }
                }
                else
                {
                    if (snprintf(v14, 128, "%s", v12) > 127)
                        return;
                }
            } while ((v15 += 128, v14 = (unsigned long)(v14 + 128), v15 != &v8));
            if (idx == 1)
                break;
            idx = 1;
        }
        g_4263c8 = 1;
        return;
    }
    else
    {
        return;
    }
}



// Function: queue_directory @ 0x6840
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    char field_10;
    char padding_11[7];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_427380;

unsigned long long queue_directory(long long a0, long long a1, char a2)
{
    long long v1;  // r12
    long long v2;  // rbp
    struct_0 *idx;  // rbx
    unsigned long long v4;  // rax

    v1 = a1;
    v2 = a0;
    idx = sub_4169e0(32);
    if (v1)
        v1 = sub_416d80(v1);
    idx->field_8 = v1;
    if (v2)
        v2 = sub_416d80(v2);
    v4 = g_427380;
    idx->field_0 = v2;
    idx->field_10 = a2;
    idx->field_18 = v4;
    g_427380 = idx;
    return v4;
}



// Function: time_type_to_statx @ 0x68b0
extern unsigned int g_427334;

int time_type_to_statx(void)
{
    unsigned long v0;  // [bp-0x8]

    if (g_427334 > 3)
    {
        v0 = g_427334;
        abort(); /* do not return */
    }
    return *((int *)(4 * g_427334 + 4298272));
}



// Function: calc_req_mask @ 0x68d0
extern unsigned int g_419144[4];
extern unsigned int g_427330;
extern unsigned int g_427338;

long long calc_req_mask(void)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (!g_427338)
        sub_4068b0();
    if (g_427330 > 6)
        abort(); /* do not return */
    goto *((void *)((long long)(g_419144[g_427330] + (char *)&g_419144[0])));
}



// Function: patterns_match @ 0x6990
typedef struct struct_0 {
    char *field_0;
    struct struct_0 *field_8;
} struct_0;

unsigned int patterns_match(struct_0 *a0, char *a1)
{
    unsigned long long v2;  // rbx
    struct_0 *v3;  // rbx
    unsigned long long v0;  // [bp-0x10]

    if (!a0)
        return 0;
    v0 = v2;
    while (1)
    {
        v3 = a0;
        if (!fnmatch(v3->field_0, a1, 4))
            return 1;
        a0 = v3->field_8;
        if (!v3->field_8)
            return 0;
    }
}



// Function: file_ignored @ 0x69f0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 *g_4272e0;
extern struct_0 *g_4272e8;
extern unsigned int g_4272f0;

unsigned int file_ignored(char *a0)
{
    if (g_4272f0 != 2)
    {
        if (*(a0) != 46)
        {
            if (!g_4272f0 && (char)sub_406990(g_4272e0, a0))
                return 1;
        }
        else
        {
            if (!g_4272f0)
            {
                return 1;
            }
            else if (!a0[1 + (a0[1] == 46)])
            {
                return 1;
            }
        }
    }
    return sub_406990(g_4272e8, a0);
}



// Function: known_term_type @ 0x6a50
unsigned int known_term_type(void)
{
    char *v1;  // rax
    char *v2;  // rbx

    v1 = getenv("TERM");
    if (v1 && *(v1))
    {
        v2 = "# Configuration file for dircolors, a utility to help you set the";
        do
        {
            if (!strncmp(v2, "TERM ", 5) && !fnmatch(v2 + 5, v1, 0))
                return 1;
        } while ((v2 = v2 + strlen(v2) + 1, v2 - "# Configuration file for dircolors, a utility to help you set the" <= 0x12c7));
    }
    return 0;
}



// Function: parse_ls_color @ 0x6af0
typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[8];
    unsigned long long field_18;
    void* field_20;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 g_425860;
extern char g_426060;
extern unsigned long long g_4260d0;
extern char *g_4260d8;
extern void* g_427300;
extern unsigned long long g_427308;
extern char g_427312;
extern char g_427390;

void parse_ls_color(void)
{
    char *v8;  // r13
    char *v9;  // rbx
    void* ptr;  // rbx
    void* v19;  // rbx
    char *v20;  // rax
    char v10;  // al
    unsigned long long index;  // rbx
    char *v12;  // rsi
    unsigned long long *v13;  // rcx
    struct_1 *idx;  // rbp
    void* v17;  // rbx
    char *v0;  // [bp-0x68]
    struct_2 **v1;  // [bp-0x60]
    char *v2;  // [bp-0x58]
    void* v3;  // [bp-0x50]
    char v4;  // [bp-0x43], Other Possible Types: unsigned short
    char v5;  // [bp-0x42]
    char v6;  // [bp-0x41]

    v2 = getenv("LS_COLORS");
    if (v2 && *(v2))
    {
        v4 = 0x3f3f;
        v8 = "lc";
        v6 = 0;
        g_427300 = sub_416d80(v2);
        v3 = g_427300;
        while (1)
        {
            v9 = v2;
            v10 = *(v9);
            switch (v10)
            {
            case 42:
                idx = sub_4169e0(40);
                v1 = &v2;
                v0 = &v3;
                idx->field_20 = g_427308;
                g_427308 = idx;
                v2 = v9 + 1;
                idx->field_8 = v3;
                if (!(char)sub_405740(&v3, &v2, 1, idx) || !(v2 += 1, *(v2) == 61 && (idx->field_18 = (unsigned long long)v3, (char)sub_405740(v0, v1, 0, idx + 16))))
                    break;
                continue;
            case 58:
                v2 = v9 + 1;
                continue;
            case 0:
LABEL_406cb8:
                if (g_4260d0 != 6)
                {
                    return;
                }
                else if (strncmp(g_4260d8, "target", 6))
                {
                    return;
                }
                else
                {
                    g_427390 = 1;
                    return;
                }
            default:
                v4 = v10;
                v2 = v9 + 1;
                if (v9[1] && (v5 = v9[1], v2 = v9 + 3, v9[2] == 61))
                {
                    index = 0;
                    v12 = v8;
                    while (strcmp(&v4, v12))
                    {
                        index += 1;
                        v12 = *((long long *)&g_425860.padding_0[8 * index]);
                        if (!v12)
                            goto LABEL_406c36;
                    }
                    v13 = &(&g_426060)[16 * (unsigned int)index];
                    v13[1] = v3;
                    if ((char)sub_405740(&v3, &v2, 0, v13))
                        continue;
LABEL_406c36:
                    sub_4158c0(&v4);
                    error(0, 0, dcgettext(NULL, "unrecognized prefix: %s", 5));
                    break;
                }
                error(0, 0, dcgettext(NULL, "unparsable value for LS_COLORS environment variable", 5));
                free(g_427300);
                v17 = g_427308;
                if (g_427308)
                {
                    do
                    {
                        ptr = v17;
                        v19 = (long long)ptr[32];
                        free(ptr);
                        v17 = v19;
                    } while ((long long)ptr[32]);
                }
                g_427312 = 0;
                goto LABEL_406cb8;
            }
        }
    }
    v20 = getenv("COLORTERM");
    if (v20 && *(v20))
        return;
    if ((char)sub_406a50())
        return;
    g_427312 = 0;
    return;
}



// Function: free_ent @ 0x6dc0
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    void* field_10;
    char padding_18[152];
    long long field_b0;
} struct_0;

extern long long g_42602a;

long long free_ent(struct_0 *a0)
{
    void* v1;  // rdi

    free(a0->field_0);
    free(a0->field_8);
    free(a0->field_10);
    if (a0->field_b0 == "?")
    {
        return &g_42602a;
    }
    else if ((char)sub_4055d0(a0->field_b0))
    {
        return (unsigned int)(unsigned long long)free(v1);
    }
    else
    {
        return (unsigned int)freecon();
    }
}



// Function: clear_files @ 0x6e20
extern unsigned int g_42733c;
extern unsigned int g_427340;
extern unsigned int g_427344;
extern unsigned int g_427348;
extern unsigned int g_42734c;
extern unsigned int g_427350;
extern unsigned int g_427354;
extern unsigned int g_427358;
extern unsigned int g_42735c;
extern unsigned int g_427360;
extern char g_427364;
extern unsigned long long g_4273a0;
extern char g_4273a9;
extern unsigned long long g_4273b0;

long long clear_files(void)
{
    unsigned long long i;  // rbx
    unsigned long long v2;  // rbx
    long long v3;  // rax

    if (g_4273b0)
    {
        i = 0;
        do
        {
            v2 = i + 1;
            v3 = sub_406dc0(*((long long *)(g_4273a0 + i * 8)));
            i = v2;
        } while (g_4273b0 > i);
    }
    g_4273b0 = 0;
    g_4273a9 = 0;
    g_427364 = 0;
    g_427360 = 0;
    g_42735c = 0;
    g_427358 = 0;
    g_427350 = 0;
    g_42734c = 0;
    g_427348 = 0;
    g_427354 = 0;
    g_427344 = 0;
    g_427340 = 0;
    g_42733c = 0;
    return v3;
}



// Function: format_inode @ 0x6f10
typedef struct struct_0 {
    char padding_0[32];
    long long field_20;
    char padding_28[144];
    char field_b8;
} struct_0;

extern char g_41aaab;

int format_inode(long long a0, unsigned long a1, struct_0 *a2)
{
    if (!a2->field_b8)
    {
        return &g_41aaab;
    }
    else if (!a2->field_20)
    {
        return &g_41aaab;
    }
    else
    {
        return sub_410c20(a2->field_20, a0);
    }
}



// Function: file_failure @ 0x6f50
void file_failure(unsigned long a0, char *a1, long long a2)
{
    int *err;  // rax

    sub_415600(4, a2);
    err = __errno_location();
    error(0, *(err), a1);
}



// Function: getfilecon_cache @ 0x6f90
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_4262f0;

unsigned int getfilecon_cache(unsigned long a0, struct_0 *a1, char a2)
{
    unsigned int v1;  // r12d

    if (a1->field_18 == g_4262f0)
    {
        *(__errno_location()) = 95;
        return 0xffffffff;
    }
    if (!a2)
    {
        v1 = sub_415b30();
        if (v1 >= 0)
            return v1;
    }
    else
    {
        v1 = sub_415b10();
        if (v1 >= 0)
            return v1;
    }
    if (!(char)sub_4059f0(*(__errno_location())))
        return v1;
    g_4262f0 = a1->field_18;
    return v1;
}



// Function: file_has_acl_cache @ 0x7020
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_4262e8;

unsigned int file_has_acl_cache(long long a0, struct_0 *a1)
{
    unsigned int *err;  // rbp
    unsigned int v2;  // r8d
    unsigned int v3;  // r8d
    unsigned int v4;  // r8d

    err = __errno_location();
    if (a1->field_18 == g_4262e8)
    {
        *(err) = 95;
        return 0;
    }
    *(err) = 0;
    if (sub_40e060(a0, &a1->field_18) > 0)
    {
        return v2;
    }
    else if (!(char)sub_4059f0(*(err)))
    {
        return v3;
    }
    else
    {
        g_4262e8 = a1->field_18;
        return v4;
    }
}



// Function: get_link_name @ 0x70b0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

void get_link_name(long long a0, struct_0 *a1, char a2)
{
    unsigned long long v1;  // rax

    v1 = sub_40cdd0();
    a1->field_8 = v1;
    if (!v1)
        sub_406f50(a2, dcgettext(NULL, "cannot read symbolic link %s", 5), a0);
    return;
}



// Function: make_link_name @ 0x7100
char * make_link_name(char *a0, char *a1)
{
    char *v1;  // rdi
    unsigned long v2;  // r12
    char *v3;  // r14

    if (!a1)
        return NULL;
    if (*(a1) != 47)
    {
        v1 = a1;
        v2 = sub_40dfb0();
        if (v2)
        {
            v3 = sub_4169e0(v2 + strlen(v1) + 2);
            if (*(&a0[v2] - 1) != 47)
                v2 += 1;
            strcpy(stpncpy(v3, a0, v2), a1);
            return v3;
        }
    }
    else
    {
        v1 = a1;
    }
    return sub_416d80(v1);
}



// Function: needs_quoting @ 0x71a0
extern long long g_4272d0;

unsigned int needs_quoting(char *a0)
{
    unsigned long v2;  // rax
    unsigned long len;  // rax
    char v0;  // [bp-0x22]

    v2 = sub_4152c0(&v0, 2, a0, -0x1, g_4272d0);
    if (*(a0) == v0)
    {
        len = strlen(a0);
        return _INSERT(len, 0, len != v2);
    }
    return 1;
}



// Function: format_group_width @ 0x7220
extern char g_42732d;

void format_group_width(unsigned long a0)
{
    if (!g_42732d)
        sub_4109e0();
}



// Function: format_user_width @ 0x7260
extern char g_42732d;

void format_user_width(unsigned long a0)
{
    if (!g_42732d)
        sub_410840();
}



// Function: xstrcoll @ 0x72a0
void xstrcoll(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_4072ad();
    return;
}



// Function: rev_strcmp_extension @ 0x7350
void rev_strcmp_extension(unsigned long a0, unsigned long a1)
{
}



// Function: basename_is_dot_or_dotdot @ 0x73b0
void basename_is_dot_or_dotdot(void)
{
    sub_40d360();
}



// Function: indent @ 0x73d0
extern unsigned long long g_4272c0;

unsigned long long indent(unsigned long long a0, unsigned long long a1)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned long long v5;  // rbx
    unsigned long long v6;  // rbx
    unsigned long long v7;  // rax
    unsigned long long v8;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    if (a0 >= a1)
        return v8;
    v1 = v3;
    v0 = v4;
    while (1)
    {
        v5 = a0;
        if (!g_4272c0 || a1 / g_4272c0 <= v6 / g_4272c0)
        {
            v6 = v5 + 1;
            v7 = putchar_unlocked(32);
            a0 = v6;
            if (a0 >= a1)
                return v7;
        }
        else
        {
            putchar_unlocked(9);
            a0 = g_4272c0 + v5 - v5 % g_4272c0;
            if (a0 >= a1)
                return v5 / g_4272c0;
        }
    }
}



// Function: dired_outbyte @ 0x7460
extern unsigned long long g_4271f8;

int dired_outbyte(char a0)
{
    g_4271f8 = g_4271f8 + 1;
    return putchar_unlocked(a0);
}



// Function: get_color_indicator @ 0x7480
typedef struct struct_0 {
    char padding_0[48];
    unsigned int field_30;
    char padding_34[120];
    unsigned int field_ac;
    char padding_b0[9];
    char field_b9;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    long long field_8;
    char field_10;
    char padding_11[7];
    unsigned long long field_18;
    struct struct_2 *field_20;
} struct_2;

extern char g_426060;
extern char g_426068;
extern struct_2 *g_427308;
extern char g_427390;

unsigned long long get_color_indicator(struct_0 *idx, char a1)
{
    unsigned long v1;  // rbp
    unsigned int v2;  // r12d
    unsigned long len;  // rax
    struct_2 *v13;  // rbx
    unsigned long long v14;  // r12
    struct_2 *v15;  // rbx
    char v3;  // dl
    unsigned int v4;  // esi
    unsigned long long v5;  // rax
    struct_0 *v6;  // rdi
    unsigned long v7;  // rax
    unsigned long long v9;  // rdx
    unsigned long v10;  // rax

    if (a1)
    {
        v1 = *((long long *)&idx->padding_0[8]);
        v2 = idx->field_ac;
        if (idx->field_b9)
        {
            v3 = idx->padding_b0[8];
            v4 = 0;
            if (idx->padding_b0[8])
                goto LABEL_407551;
            goto LABEL_4074b9;
        }
        else
        {
            v4 = 0xffffffff;
            if (!(char)sub_406420(12))
                goto LABEL_407542;
            v5 = 192;
        }
    }
    else
    {
        v2 = sub_4055e0(idx);
        v1 = *((long long *)&v6->padding_0[0]);
        v4 = v6->field_b9;
LABEL_407542:
        v3 = idx->padding_b0[8];
        if (idx->padding_b0[8])
        {
LABEL_407551:
            switch (v2 & 0xf000)
            {
            case 32768:
                if (!(!((unsigned short)v2 & 0x800) || !(char)sub_406420(16)))
                {
                    v5 = 0x100;
                    break;
                }
                else if ((unsigned short)v2 & 0x400 && (char)sub_406420(0x11))
                {
                    v5 = 272;
                    break;
                }
                else if ((char)sub_406420(21) && idx[1].padding_0[6])
                {
                    v5 = 336;
                    break;
                }
                else if (!(!((char)v2 & 73) || !(char)sub_406420(14)))
                {
                    v5 = 224;
                    break;
                }
                else if (*((long long *)&idx->padding_0[40]) <= 1 || !(char)sub_406420(22))
                {
LABEL_407660:
                    len = strlen(v1);
                    v13 = g_427308;
                    v14 = len;
                    if (g_427308)
                    {
                        do
                        {
                            v15 = v13;
                            if (v15->field_0 <= v14 && !sub_40d5c0(v14 - v15->field_0 + v1, v15->field_8))
                            {
                                if (!v15->field_18)
                                    return 0;
                                return &v15->field_10;
                            }
                        } while ((v13 = (struct_2 *)v15->field_20, v15->field_20));
                    }
                    v5 = 80;
                    break;
                }
                else
                {
                    v5 = 352;
                    break;
                }
            case 16384:
                if ((((unsigned short)v2 & 0x202) != 0x202 || (v5 = 320, !(char)(sub_406420(20) & 0xffffffff))) && (!((char)v2 & 2) || (v5 = 304, !(char)(sub_406420(19) & 0xffffffff))))
                {
                    v5 = 96;
                    if ((unsigned short)v2 & 0x200)
                    {
                        v10 = -((char)sub_406420(18) < 1);
                        v5 = _INSERT(v10, 0, (char)v10 & 64) + 288;
                        break;
                    }
                }
                break;
            case 40960:
                v7 = 7;
LABEL_4074d8:
                if (!v4 && v3)
                {
                    v5 = 208;
                    if (!g_427390)
                    {
                        v5 = (-((char)sub_406420(13) < 1) & 0xffffffffffffffa0) + 208;
                        break;
                    }
                }
                else
                {
                    v5 = v7 * 16;
                    break;
                }
            case 4096:
                v5 = 128;
                break;
            case 49152:
                v5 = 144;
                break;
            case 24576:
                v5 = 160;
                break;
            case 8192:
                v5 = 208;
                v9 = 176;
                v5 = v9;
                goto LABEL_4075c1;
            default:
                v5 = 208;
                v9 = 176;
LABEL_4075c1:
                break;
            }
        }
LABEL_4074b9:
        v7 = *((int *)(4 * *((int *)&idx->padding_34[116]) + 4298368));
        v3 = (unsigned int)v7 == 7;
        if ((unsigned int)v7 != 5)
            goto LABEL_4074d8;
        goto LABEL_407660;
    }
    if (*((long long *)&(&g_426068)[v5]))
        return &(&g_426060)[v5];
    return 0;
}



// Function: dired_outbuf @ 0x77d0
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned long long g_4271f8;

long long dired_outbuf(void* a0, unsigned long a1)
{
    g_4271f8 = g_4271f8 + a1;
    return fwrite_unlocked(a0, 1, a1, stdout);
}



// Function: dired_outstring @ 0x77f0
long long dired_outstring(char *a0)
{
    char v0;  // [bp+0x0]

    return sub_4077d0(&v0, strlen(a0));
}



// Function: dired_indent @ 0x7810
extern char g_427318;

long long dired_indent(void)
{
    long long v1;  // rax

    if (g_427318)
        return sub_4077f0("  ");
    return v1;
}



// Function: file_escape @ 0x7830
extern char g_426fe0;

char * file_escape(char *a0, char a1)
{
    char *iter;  // r14
    char *v2;  // rax
    char v3;  // dl
    char *v4;  // r15
    char *v5;  // r15
    char *node;  // r15
    char v7;  // dl

    iter = a0;
    v2 = sub_416b00(3, strlen(a0) + 1);
    v3 = *(iter);
    v4 = v2;
    v5 = v4;
    if (v3)
    {
        node = v4;
        while (1)
        {
            iter += 1;
            if (!(v3 != 47 || !a1))
            {
                *(node) = 47;
                node += 1;
LABEL_40787f:
                v3 = *(iter);
                v5 = node;
                if (!*(iter))
                    break;
            }
            else if ((&g_426fe0)[sub_405570(v3) & 0xff])
            {
                *(node) = v7;
                node += 1;
                goto LABEL_40787f;
            }
            else
            {
                v5 = node + 3;
                __sprintf_chk(node, 0x1);
                v3 = *(iter);
                node = v5;
                if (!v3)
                    break;
            }
        }
    }
    *(v5) = 0;
    return v4;
}



// Function: push_current_dired_pos @ 0x7900
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned long long g_4271f8;
extern char g_427318;

long long push_current_dired_pos(struct_0 *idx)
{
    unsigned long v1;  // rax

    if (!g_427318)
        return v1;
    if (idx->field_20 - (char *)idx->field_18 > 7)
    {
        idx->field_18->field_0 = g_4271f8;
        idx->field_18 = idx->field_18 + 1;
    }
    else
    {
        _obstack_newchunk();
        idx->field_18->field_0 = g_4271f8;
        idx->field_18 = idx->field_18 + 1;
    }
    return g_4271f8;
}



// Function: dev_ino_push @ 0x7970
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    char field_10;
} struct_0;

extern long long g_4270e0;
extern struct_0 *g_4270f8;
extern unsigned long long g_427100;

struct_0 * dev_ino_push(unsigned long a0, unsigned long a1)
{
    struct_0 *v1;  // rax

    v1 = g_4270f8;
    if (g_427100 - (char *)g_4270f8 <= 15)
    {
        _obstack_newchunk(&g_4270e0, 16);
        v1 = g_4270f8;
    }
    g_4270f8 = &v1->field_10;
    v1->field_8 = a0;
    v1->field_0 = a1;
    return v1;
}



// Function: signal_init @ 0x7ba0
void signal_init(void)
{
}



// Function: put_indicator @ 0x7bb0
typedef struct struct_0 {
    char field_0;
    char padding_1[7];
    void* field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_427310;

long long put_indicator(struct_0 *a0)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    long long v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9

    if (!g_427310)
    {
        g_427310 = 1;
        if (tcgetpgrp(1) < 0)
        {
            sub_407c10();
        }
        else
        {
            sub_407ba0();
            sub_407c10(v1, v2, v3, v4, v5, v6);
        }
    }
    return fwrite_unlocked(a0->field_8, *((long long *)&a0->field_0), 1, stdout);
}



// Function: prep_non_filename_text @ 0x7c10
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 g_426060;
extern struct_0 g_426070;
extern struct_0 g_426080;
extern unsigned long long g_426088;
extern struct_0 g_426090;

long long prep_non_filename_text(void)
{
    if (!g_426088)
    {
        sub_407bb0(&g_426060.padding_0[0]);
        sub_407bb0(&g_426090.padding_0[0]);
        return sub_407bb0(&g_426070.padding_0[0]);
    }
    return sub_407bb0(&g_426080.padding_0[0]);
}



// Function: set_normal_color @ 0x7c60
extern char g_427312;

void set_normal_color(void)
{
    if (g_427312)
        sub_407c70();
    return;
}



// Function: restore_default_color @ 0x7cc0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 g_426060;
extern struct_0 g_426070;

long long restore_default_color(void)
{
    sub_407bb0(&g_426060.padding_0[0]);
    return sub_407bb0(&g_426070.padding_0[0]);
}



// Function: process_signals @ 0x7ce0
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_427214;
extern unsigned int g_427218;
extern int g_427220;
extern char g_427310;

void process_signals(void)
{
    unsigned int v2;  // r12d
    int v0[34];  // [bp-0xa8]

    while (g_427218 || g_427214)
    {
        if (g_427310)
            sub_407cc0();
        fflush_unlocked(stdout);
        sigprocmask(0, &g_427220, v0);
        v2 = g_427218;
        if (g_427214)
        {
            v2 = 19;
            g_427214 = g_427214 - 1;
        }
        else
        {
            signal(g_427218, NULL);
        }
        raise(v2);
        sigprocmask(2, v0, NULL);
    }
    return;
}



// Function: signal_restore @ 0x7dd0
void signal_restore(void)
{
}



// Function: visit_dir @ 0x7df0
extern long long g_4273c8;

int visit_dir(unsigned long a0, unsigned long a1)
{
    unsigned long long *v1;  // rax
    unsigned long long *ptr;  // rbp
    unsigned long long *v3;  // rax

    v1 = sub_4169e0(16);
    *(v1) = a1;
    ptr = v1;
    ptr[1] = a0;
    v3 = sub_40f9a0(g_4273c8, ptr);
    if (!v3)
        sub_416da0(); /* do not return */
    if (ptr == v3)
        return _INSERT(a1, 0, ptr != v3);
    free(ptr);
    return _INSERT(a1, 0, ptr != v3);
}



// Function: dired_dump_obstack @ 0x8060
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

unsigned long long dired_dump_obstack(char *a0, struct_0 *idx)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rbx
    unsigned long long v3;  // rbp
    unsigned long long v4;  // rax
    unsigned long long v5;  // rdx
    unsigned long long i;  // rbx

    v1 = idx->field_18;
    v2 = idx->field_10;
    v3 = v1 - v2;
    if (v3 <= 7)
        return v1;
    if (v1 == v2)
        idx->field_50 = idx->field_50 | 2;
    v4 = v1 + idx->field_30 & ~(idx->field_30);
    v5 = idx->field_20;
    if (v4 - idx->field_8 <= idx->field_20 - idx->field_8)
        v5 = v4;
    idx->field_18 = v5;
    idx->field_10 = v5;
    fputs_unlocked(a0, stdout);
    i = v2;
    do
    {
        i += 8;
        __printf_chk(1, " %ld");
    } while ((v3 & 0xfffffffffffffff8) + v2 != i);
    return putchar_unlocked(10);
}



// Function: format_user_or_group @ 0x8100
void format_user_or_group(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_408104();
    return;
}



// Function: format_group @ 0x8180
extern char g_42732d;

void format_group(unsigned int a0, unsigned int a1, char a2)
{
    unsigned long v2;  // rsi
    unsigned long v0;  // [bp-0x10]

    v2 = a0;
    if (!a2)
    {
        sub_408100("?", v2, a1);
        return;
    }
    else if (g_42732d)
    {
        sub_408100(0, v2, a1);
        return;
    }
    else
    {
        v0 = v2;
        sub_408100(sub_4109e0(), v0, a1);
        return;
    }
}



// Function: format_user @ 0x81d0
extern char g_42732d;

void format_user(unsigned int a0, unsigned int a1, char a2)
{
    unsigned long v2;  // rsi
    unsigned long v0;  // [bp-0x10]

    v2 = a0;
    if (!a2)
    {
        sub_408100("?", v2, a1);
        return;
    }
    else if (g_42732d)
    {
        sub_408100(0, v2, a1);
        return;
    }
    else
    {
        v0 = v2;
        sub_408100(sub_410840(), v0, a1);
        return;
    }
}



// Function: long_time_expected_width @ 0x82b0
extern unsigned int g_426014;
extern long long g_4272a8;

unsigned long long long_time_expected_width(void)
{
    unsigned long long v4;  // rax
    long long v5;  // rax
    unsigned long long v6;  // rax
    unsigned long v0;  // [bp-0x460]
    char v1;  // [bp-0x458]
    char v2;  // [bp-0x418]

    v4 = g_426014;
    if ((unsigned int)v4 >= 0)
        return v4;
    v0 = 0;
    if (!sub_4160a0(g_4272a8, &v0, &v1) || (v5 = (long long)(unsigned long long)sub_408240(&v2, 0, &v1, g_4272a8, 0), !v5))
    {
        v6 = g_426014;
    }
    else
    {
        v6 = sub_411180(&v2, v5, 0);
        g_426014 = v6;
    }
    if ((unsigned int)v6 >= 0)
        return v6;
    g_426014 = 0;
    return v6;
}



// Function: fileinfo_name_width @ 0x8890
typedef struct struct_0 {
    char padding_0[200];
    unsigned long long field_c8;
} struct_0;

unsigned long long fileinfo_name_width(struct_0 *a0)
{
    if (a0->field_c8)
        return a0->field_c8;
}



// Function: rev_strcmp_width @ 0x8920
void rev_strcmp_width(unsigned long a0, unsigned long a1)
{
}



// Function: strcmp_width @ 0x8940
void strcmp_width(void)
{
}



// Function: rev_xstrcoll_width @ 0x8950
void rev_xstrcoll_width(unsigned long a0, unsigned long a1)
{
}



// Function: xstrcoll_width @ 0x8970
void xstrcoll_width(void)
{
}



// Function: update_current_files_info @ 0x8980
extern unsigned long long g_4272b0;
extern unsigned int g_427330;
extern unsigned int g_427338;
extern unsigned long long g_4273a0;
extern char g_4273b0;

unsigned long long update_current_files_info(void)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax
    unsigned long long idx;  // rbx
    unsigned long long v4;  // rbx
    unsigned long long v5;  // rax
    unsigned long long i;  // 4098

    if (g_427330 != 2)
    {
        if (g_4272b0)
        {
            v1 = g_427338 - 2;
            if ((unsigned int)v1 > 1)
                return v1;
        }
        else
        {
            return v2;
        }
    }
    idx = 0;
    if (!*((long long *)&g_4273b0))
        return v1;
    do
    {
        v4 = idx + 1;
        v5 = sub_408890(*((long long *)(g_4273a0 + idx * 8)));
        i = *((long long *)&g_4273b0);
        *((unsigned long long *)(*((long long *)(g_4273a0 + idx * 8)) + 200)) = v5;
        idx = v4;
    } while (i > idx);
    return v5;
}



// Function: sort_files @ 0x89f0
typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 g_424f60;
extern long long g_426300;
extern char g_4272f4;
extern char g_42732f;
extern unsigned int g_427330;
extern unsigned int g_427334;
extern unsigned long long g_427398;
extern void g_4273a0;
extern void g_4273b0;

unsigned long long sort_files(void)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // eax
    unsigned long long v3;  // rsi
    unsigned long v4;  // rsi

    if ((*((long long *)&g_4273b0) >> 1) + *((long long *)&g_4273b0) > g_427398)
    {
        free(*((long long *)&g_4273a0));
        *((unsigned long long *)&g_4273a0) = sub_416b00(*((long long *)&g_4273b0), 24);
        g_427398 = *((long long *)&g_4273b0) * 3;
    }
    sub_406000();
    v1 = sub_408980();
    if (g_427330 == 6)
        return v1;
    v2 = _setjmp(&g_426300);
    if (v2)
    {
        if (g_427330 != 4)
        {
            sub_406000();
            v2 = 1;
        }
        else
        {
            __assert_fail(); /* do not return */
        }
    }
    if ((unsigned int)v3 == 5)
        v4 = g_427334 + 5;
    return sub_411610(*((long long *)&g_4273a0), *((long long *)&g_4273b0), *((long long *)&g_424f60.padding_0[64 * v4 + 32 * v2 + 16 * g_42732f + 8 * g_4272f4]), v2 + v4 * 2);
}



// Function: length_of_file_name_and_frills @ 0x8af0
typedef struct struct_0 {
    char padding_0[200];
    unsigned long long field_c8;
} struct_0;

extern char g_4272fc;
extern unsigned int g_427314;
extern long long g_427320;
extern unsigned int g_427328;
extern char g_42732c;
extern unsigned int g_427338;
extern unsigned int g_427354;
extern unsigned int g_42735c;
extern unsigned int g_427360;
extern char g_427365;

unsigned long long length_of_file_name_and_frills(struct_0 *a0)
{
    unsigned long iter;  // rbx
    unsigned long v3;  // rax
    char *v4;  // rdi
    unsigned long v5;  // rax
    unsigned long long v6;  // r8
    unsigned long v7;  // r8
    char v0;  // [bp-0x2b8]

    if (g_4272fc)
    {
        if (g_427338 != 4)
        {
            iter = g_427360 + 1;
            if (!g_42732c)
            {
                if (!g_427365)
                    goto LABEL_408b5c;
LABEL_408bf9:
                v5 = g_427354 + 1;
                goto LABEL_408c04;
            }
LABEL_408b41:
            v3 = g_42735c + 1;
        }
        else
        {
            iter = strlen(sub_410c20(*((long long *)&a0->padding_0[32]), &v0)) + 1;
LABEL_408bba:
            if (!g_42732c)
                goto LABEL_408b4f;
            if (g_427338 != 4)
                goto LABEL_408b41;
            v4 = "?";
            if (a0->padding_0[184])
                v4 = sub_40ffa0(*((long long *)&a0->padding_0[88]), &v0, g_427328, 0x200, g_427320);
            v3 = strlen(v4) + 1;
        }
        iter += v3;
LABEL_408b4f:
        if (g_427365)
        {
            if (g_427338 != 4)
                goto LABEL_408bf9;
            v5 = strlen(*((long long *)&a0->padding_0[176])) + 1;
LABEL_408c04:
            iter += v5;
        }
    }
    else
    {
        iter = 0;
        goto LABEL_408bba;
    }
LABEL_408b5c:
    v6 = iter + sub_408890(a0);
    if (!g_427314)
        return v6;
    return v7 + 1 - ((char)sub_406040(a0->padding_0[184], *((int *)&a0->padding_0[48]), *((int *)&a0->padding_0[168])) < 1);
}



// Function: calculate_columns @ 0x8c90
typedef struct struct_0 {
    char field_0;
    char padding_1[7];
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
} struct_0;

extern unsigned long long g_427200;
extern struct_0 *g_427208;
extern char g_4272b0;
extern unsigned long long g_4273a0;
extern unsigned long long g_4273b0;

unsigned long long calculate_columns(char a0)
{
    unsigned long long v1;  // r12
    unsigned long v2;  // rdx
    void* v11;  // rax
    unsigned long long v12;  // r12
    unsigned long long v3;  // rbx
    unsigned long long v4;  // rax
    struct_0 *iter;  // rsi
    unsigned long long v6;  // rcx
    unsigned long long v7;  // rcx
    unsigned long long v8;  // r8
    unsigned long long *v9;  // rdx
    unsigned long long v10;  // rax

    v1 = g_4273b0;
    v2 = g_427200;
    if (g_4273b0 <= g_427200)
        v2 = v1;
    if (!g_427200)
        v2 = v1;
    v3 = 0;
    sub_4061d0(v2);
    if (g_4273b0)
    {
        do
        {
            v4 = sub_408af0(*((long long *)(g_4273a0 + v3 * 8)));
            if (v2)
            {
                iter = g_427208;
                v6 = 0;
                do
                {
                    v7 = v6 + 1;
                    if (iter->field_0)
                    {
                        v8 = (!a0 ? v3 % v7 : v3 / ((g_4273b0 + v7 - 1) / v7));
                        v9 = iter->field_10 + v8 * 8;
                        v10 = v4 + (v8 != v6) * 2;
                        if (*(v9) < v10)
                        {
                            iter->field_8 = iter->field_8 + v10 - *(v9);
                            *(v9) = v10;
                            iter->field_0 = iter->field_8 < *((long long *)&g_4272b0);
                        }
                    }
                } while ((iter += 24, v6 = v7, v2 != v6));
            }
        } while ((v3 += 1, v3 < g_4273b0));
    }
    if (v2 <= 1)
        return v2;
    v11 = (char *)g_427208 + 24 * v2 - 24;
    do
    {
    } while (!*((char *)v11) && (v12 = v2 - 1, v11 -= 24, v2 = (unsigned long)v12, v2 != 1));
    return v12;
}



// Function: quote_name @ 0x8eb0
typedef struct struct_0 {
    char padding_0[24];
    struct struct_1 *field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

typedef struct FILE {
} FILE;

extern char g_41ab9f;
extern char g_41abb3;
extern char g_41abb9;
extern FILE *stdout;
extern unsigned long long g_4271f8;
extern char *g_427388;
extern char g_4273a8;
extern char g_4273a9;

long long quote_name(char *a0, long long a1, unsigned int a2, long long a3, unsigned int a4, struct_0 *a5, char *a6)
{
    unsigned long v7;  // rbx
    char v8;  // r14b
    unsigned long v9;  // r10
    char *v10;  // rcx
    char *v11;  // r8
    long long v0;  // [bp-0x2078], Other Possible Types: unsigned long long
    void* count;  // [bp-0x2070], Other Possible Types: unsigned long
    char *v2;  // [bp-0x2068]
    unsigned long count1;  // [bp-0x2060]
    char v4;  // [bp-0x2051]
    char *v5;  // [bp-0x2050]
    char v6;  // [bp-0x2048]

    *((unsigned int *)&v0) = a4;
    v5 = &v6;
    v7 = sub_408390(&v5, a0, a1, a2, 0, &v4, v0);
    if (v4 && (char)v0)
        sub_407460(32);
    if (a3)
        sub_408e50(a3);
    if (a6)
    {
        v8 = g_4273a8;
        if (!g_4273a8 || (v8 = g_4273a9, !g_4273a9))
        {
            v0 = 0;
            v9 = v7;
        }
        else if (v4)
        {
            v0 = 0;
            v8 = 0;
            v9 = v7;
        }
        else
        {
            putchar_unlocked(v6);
            v0 = 1;
            v9 = v7 - 2;
        }
        count1 = v9;
        count = sub_407830(g_427388, 0);
        v10 = &g_41abb9;
        v11 = sub_407830(a6, 1);
        if (*(v11) != 47)
            v10 = "/";
        v2 = v11;
        __printf_chk(1, &g_41ab9f, count, v10, v11);
        free(count);
        free(v2);
    }
    else
    {
        v0 = 0;
        v8 = 0;
        count1 = v7;
    }
    if (a5)
    {
        count = count1;
        sub_407900(a5);
        fwrite_unlocked(&v5[v0], 1, count, stdout);
        g_4271f8 = g_4271f8 + v7;
        sub_407900(a5);
    }
    else
    {
        fwrite_unlocked(&v5[v0], 1, count1, stdout);
        g_4271f8 = g_4271f8 + v7;
    }
    if (a6)
    {
        fputs_unlocked(&g_41abb3, stdout);
        if (v8)
            putchar_unlocked(*(&v5[v7] - 1));
    }
    if (v5 != &v6 && v5 != a0)
        free(v5);
    return v4 + v7;
}



// Function: print_name_with_quoting @ 0x9160
typedef struct struct_0 {
    char padding_0[48];
    unsigned int field_30;
    char padding_34[120];
    unsigned int field_ac;
    char padding_b0[9];
    char field_b9;
} struct_0;

extern struct_0 g_4261d0;
extern unsigned long long g_4272b0;
extern long long g_4272d0;
extern char g_427312;

long long print_name_with_quoting(struct_0 *a0, char a1, struct_0 *a2, unsigned long long a3)
{
    unsigned long v1;  // rdx
    unsigned long v2;  // r14
    unsigned long v3;  // rax
    long long v4;  // rax
    long long v5;  // r12

    v1 = *((long long *)&a0->padding_0[8]);
    if (!a1)
        v1 = *((long long *)&a0->padding_0[0]);
    v2 = v1;
    if (g_427312)
    {
        v3 = sub_407480(a0, a1);
        if (v3 || (char)sub_406420(4))
        {
            v5 = sub_408eb0(v2, g_4272d0, *((int *)&a0[1].padding_0[10]), v3, a1 ^ 1, a2, *((long long *)&a0->padding_0[16]));
            sub_407ce0();
            sub_407c10();
            if (!g_4272b0)
            {
                return v5;
            }
            else if (a3 / g_4272b0 == (v5 + a3 - 1) / g_4272b0)
            {
                return v5;
            }
            else
            {
                sub_407bb0(&g_4261d0.padding_0[0]);
                return v5;
            }
        }
    }
    v4 = sub_408eb0(v2, g_4272d0, *((int *)&a0[1].padding_0[10]), 0, a1 ^ 1, a2, *((long long *)&a0->padding_0[16]));
    sub_407ce0();
    return v4;
}



// Function: print_long_format @ 0x92b0
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

extern char g_426028;
extern char g_426029;
extern struct_0 g_4271a0;
extern long long g_4272a8;
extern char g_4272fc;
extern unsigned int g_427314;
extern long long g_427320;
extern unsigned int g_427328;
extern char g_42732c;
extern char g_42732e;
extern unsigned int g_427334;
extern unsigned int g_42733c;
extern unsigned int g_427340;
extern unsigned int g_427348;
extern unsigned int g_42734c;
extern unsigned int g_427350;
extern unsigned int g_427354;
extern unsigned int g_42735c;
extern char g_427364;
extern char g_427365;
extern long long g_427370;
extern long long g_427378;

void print_long_format(struct_0 *a0)
{
    unsigned int v13;  // eax
    char v14;  // r14b
    unsigned int v19;  // eax
    int v20;  // eax
    void* i;  // rax
    char v22;  // al
    void* v23;  // rax
    void* iter;  // rbx
    void* iter1;  // r15
    unsigned int v26;  // eax
    int v27;  // ebp
    void* iter2;  // rbp
    long long v15;  // rdx
    char k;  // al
    unsigned long long v30;  // rax
    unsigned long v32;  // r8
    long long v33;  // r9
    unsigned long long v34;  // rax
    void* v35;  // rbx
    unsigned long long v36;  // rbx
    long long v37;  // rax
    void* v16;  // r13
    void* node;  // rbx
    void* v18;  // r15
    void* v0;  // [bp-0x11b8], Other Possible Types: unsigned long
    unsigned long long v1;  // [bp-0x11b0]
    unsigned int v2;  // [bp-0x11a8]
    unsigned long v3;  // [bp-0x1198]
    long long v4;  // [bp-0x1190]
    char v5;  // [bp-0x1188]
    char v6;  // [bp-0x1144]
    char v7;  // [bp-0x113a]
    char v8;  // [bp-0x1139]
    char v9;  // [bp-0x1138]
    char v10;  // [bp-0x1118]
    char v11[3656];  // [bp-0xe88]

    if (a0->field_b8)
    {
        sub_40e2f0(&a0->padding_10[8], &v6);
    }
    else
    {
        strncpy(&v8, "??????????", 10);
        v6 = *((char *)(a0->field_a8 + 4298608));
    }
    if (!g_427364)
    {
        v7 = 0;
    }
    else if (a0->field_bc == 1)
    {
        v7 = 46;
    }
    else if (a0->field_bc == 2)
    {
        v13 = g_427334;
        v7 = 43;
        if (g_427334 != 2)
        {
            if (g_427334 <= 2)
                goto LABEL_409325;
LABEL_4095a0:
            if (v13 != 3)
                abort(); /* do not return */
            v3 = (unsigned long long)sub_405610(&a0->padding_10[8]);
            v4 = v15;
            v14 = (v3 & v15) != 0xffffffffffffffff;
        }
        else
        {
LABEL_409600:
            v14 = 1;
            v3 = sub_415c90(&a0->padding_10[8]);
            v4 = v15;
        }
LABEL_409347:
        v16 = v11;
        node = v11;
        if (g_4272fc)
        {
            sub_406f10(&v10, 21, a0);
            node = &v11[(int)__sprintf_chk(v11, 0x1)];
        }
        if (g_42732c)
        {
            v18 = "?";
            if (a0->field_b8)
                v18 = sub_40ffa0(a0->field_58, &v10, g_427328, 0x200, g_427320);
            v19 = sub_4113a0(v18, 0);
            v20 = g_42735c - v19;
            if (v20 > 0)
            {
                i = v20 + node;
                do
                {
                    node += 1;
                    *((char *)node - 1) = 32;
                } while (i != node);
            }
            while (1)
            {
                v18 += 1;
                node = i + 1;
                v22 = *((char *)v18 - 1);
                *((char *)node - 1) = v22;
                if (!v22)
                    break;
                i = node;
            }
            *((char *)node - 1) = 32;
        }
        v23 = "?";
        if (a0->field_b8)
            v23 = sub_410c20(a0->field_28, &v10);
        v0 = v23;
        iter = node + (int)__sprintf_chk(node, 0x1);
        sub_407810();
        if (g_426029 || g_426028 || g_42732e || g_427365)
        {
            sub_4077d0(v11, iter - v11);
            if (!g_426029)
            {
                if (g_426028)
                    goto LABEL_4097ce;
LABEL_409460:
                if (g_42732e)
                    goto LABEL_4097f4;
            }
            else
            {
                sub_4081d0(a0->field_34, g_427350, a0->field_b8);
                if (!g_426028)
                    goto LABEL_409460;
LABEL_4097ce:
                sub_408180(a0->field_38, g_42734c, a0->field_b8);
                if (g_42732e)
                {
LABEL_4097f4:
                    sub_4081d0(a0->field_34, g_427348, a0->field_b8);
                }
            }
            iter = v16;
            if (g_427365)
            {
                sub_408100(*((long long *)&a0->field_b0), 0, g_427354);
                iter = v16;
            }
        }
        if (!a0->field_b8)
        {
            iter1 = "?";
            goto LABEL_409493;
        }
        else if (((unsigned short)a0->field_30 & 0xb000) != 0x2000)
        {
            iter1 = sub_40ffa0(sub_4059e0(*((long long *)&a0->field_48)));
LABEL_409493:
            v26 = sub_4113a0(iter1, 0);
            v27 = g_42733c - v26;
            if (v27 > 0)
            {
                iter2 = v27 + iter;
                do
                {
                    iter += 1;
                    *((char *)iter - 1) = 32;
                } while (iter2 != iter);
            }
            else
            {
                iter2 = iter;
            }
            do
            {
                k = *((char *)iter1);
                iter1 += 1;
                iter2 += 1;
                *((char *)iter2 - 1) = k;
            } while (k);
            *((char *)iter2 - 1) = 32;
        }
        else
        {
            v30 = sub_410c20(gnu_dev_minor(a0->field_40) & 0xffffffff, &v10);
            v2 = g_427340;
            sub_410c20(gnu_dev_major(a0->field_40) & 0xffffffff, &v9);
            v1 = v30;
            v0 = g_427340;
            iter2 = (int)__sprintf_chk(iter, 0x1) + iter;
        }
        *((char *)iter2) = 1;
        if (!a0->field_b8 || !v14)
        {
LABEL_4094f4:
            goto LABEL_4094fb;
        }
        else if (sub_4160a0(g_4272a8, &v3, &v5))
        {
            v32 = v3;
            v33 = v4;
            if (sub_4162a0(g_427370, g_427378, v32, v33, v32, v33, v32, v33) < 0)
            {
                sub_40e960(&g_427370);
                v32 = v3;
                v33 = v4;
            }
            if (sub_4162a0(g_427370 - 15778476, g_427378, v32, v33) < 0)
                sub_4162a0(v3, v4, g_427370, g_427378);
            v34 = sub_408240(iter2);
            if (!v34)
                goto LABEL_409a20;
            iter2 += v34;
LABEL_409a10:
            *((char *)iter2) = 32;
            v35 = iter2 + 1;
        }
        else
        {
LABEL_409a20:
            if (!*((char *)iter2))
                goto LABEL_409a10;
            if (!a0->field_b8)
                goto LABEL_4094f4;
            sub_407de0(v3, &v10);
LABEL_4094fb:
            sub_4082b0();
            v35 = (int)__sprintf_chk(iter2, 0x1) + iter2;
        }
        v36 = v35 - v11;
        sub_4077d0(v11, v36);
        v37 = sub_409160(a0, 0, &g_4271a0.padding_0[0], v36);
        if (a0->field_a8 != 6)
        {
            if (!g_427314)
                return;
            sub_409280(a0->field_b8, a0->field_30);
            return;
        }
        else
        {
            if (!a0->field_8)
                return;
            sub_4077f0(" -> ");
            sub_409160(a0, 1, NULL, v36 + v37 + 4);
            if (g_427314)
            {
                sub_409280(1, a0->field_ac, 0);
                return;
            }
            return;
        }
    }
    v13 = g_427334;
    if (g_427334 == 2)
        goto LABEL_409600;
    if (g_427334 > 2)
        goto LABEL_4095a0;
LABEL_409325:
    if (v13)
    {
        v14 = 1;
        v3 = sub_415ca0(&a0->padding_10[8]);
        v4 = v15;
        goto LABEL_409347;
    }
    else
    {
        v14 = 1;
        v3 = sub_415cb0(&a0->padding_10[8]);
        v4 = v15;
        goto LABEL_409347;
    }
}



// Function: print_current_files @ 0x9f20
extern unsigned int g_419160[4];
extern unsigned int g_427338;

long long print_current_files(void)
{
    unsigned long long v2;  // rbx
    unsigned long v3;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (g_427338 > 4)
        return v3;
    v0 = v2;
    goto *((void *)((long long)(g_419160[g_427338] + (char *)&g_419160[0])));
}



// Function: xstrcoll_version @ 0xa050
void xstrcoll_version(unsigned long a0, unsigned long a1)
{
}



// Function: rev_xstrcoll_version @ 0xa060
void rev_xstrcoll_version(unsigned long a0, unsigned long a1)
{
}



// Function: extract_dirs_from_files @ 0xa1e0
typedef struct struct_1 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    char padding_10[152];
    unsigned int field_a8;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 *g_4273a0;
extern unsigned long long g_4273b0;
extern unsigned long long g_4273c8;

long long extract_dirs_from_files(long long a0, char a1)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // rbx
    unsigned long idx;  // rbx
    struct_1 *v4;  // rbp
    char *v5;  // r14
    unsigned long v6;  // rax
    unsigned long long *v7;  // rsi
    unsigned long i;  // rdi
    unsigned long long index;  // rdx
    unsigned int v10;  // 4099

    if (a0 && g_4273c8)
        sub_406840(0, a0, 0);
    v1 = g_4273b0;
    v2 = g_4273b0 - 1;
    if (g_4273b0)
    {
        while (1)
        {
            idx = v2;
            v4 = *((long long *)&(g_4273a0->padding_0)[idx]);
            v1 = sub_40a1d0(v4->field_a8);
            if (!(char)v1)
            {
LABEL_40a268:
                v2 = idx - 1;
                if (idx < 1)
                    break;
            }
            else
            {
                v5 = &v4->field_0->field_0;
                if (a0)
                {
                    v1 = (unsigned long long)sub_4073b0();
                    if (!(char)v1)
                    {
                        if (*(v5) == 47)
                            goto LABEL_40a291;
                        v6 = sub_40e300(a0, v5, 0);
                        sub_406840(v6, v4->field_8, a1);
                        v1 = (unsigned long long)free(v6);
                        if (v4->field_a8 == 9)
                            goto LABEL_40a2a9;
                        goto LABEL_40a268;
                    }
                }
LABEL_40a291:
                v1 = sub_406840(v5, v4->field_8, a1);
                if (v4->field_a8 != 9)
                    goto LABEL_40a268;
LABEL_40a2a9:
                v1 = sub_406dc0(v4);
                v2 = idx - 1;
                if (idx < 1)
                    break;
            }
        }
        if (g_4273b0)
        {
            v7 = g_4273a0;
            i = &(g_4273a0->padding_0)[g_4273b0];
            index = 0;
            do
            {
                v10 = *((int *)(*(v7) + 168));
                *((unsigned long long *)&(g_4273a0->padding_0)[index]) = *(v7);
                v7 += 1;
                index += v10 != 9;
            } while (i != v7);
            g_4273b0 = index;
            return v7;
        }
    }
    g_4273b0 = 0;
    return v1;
}



// Function: do_stat @ 0xa4c0
long long do_stat(unsigned long a0, unsigned long a1)
{
    sub_4068d0();
}



// Function: do_lstat @ 0xa4f0
long long do_lstat(unsigned long a0, unsigned long a1)
{
    sub_4068d0();
}



// Function: stat_for_mode @ 0xa530
void stat_for_mode(unsigned long a0, unsigned long a1)
{
}



// Function: fstat_for_ino @ 0xb140
void fstat_for_ino(unsigned long a0, unsigned long a1)
{
}



// Function: stat_for_ino @ 0xb160
void stat_for_ino(unsigned long a0, unsigned long a1)
{
}



// Function: print_dir @ 0xb180
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern char g_426010;
extern char g_426019;
extern struct_0 g_427140;
extern char g_4272b8;
extern long long g_4272c8;
extern char g_4272f6;
extern char g_427311;
extern long long g_427320;
extern unsigned int g_427328;
extern char g_42732c;
extern unsigned int g_427330;
extern unsigned int g_427338;
extern unsigned long long g_4273b0;
extern unsigned long long g_4273c8;

long long print_dir(unsigned long a0, unsigned long a1, char a2)
{
    unsigned long v6;  // r14
    unsigned int *err;  // rax
    dirent *count;  // rax
    dirent *v17;  // r14
    void* ptr;  // rbp
    unsigned long len;  // rax
    char *v22;  // rbx
    unsigned long v24;  // fs
    unsigned int *v8;  // rbx
    DIR *ptr1;  // rax
    DIR *v10;  // rbp
    unsigned int v11;  // eax
    void* v14;  // r15
    long long v0;  // [bp-0x378], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0x368]
    unsigned long long v2;  // [bp-0x360]
    char v3;  // [bp-0x2d7]
    unsigned long v4;  // [bp-0x40]

    v6 = a1;
    err = __errno_location();
    *(err) = 0;
    v8 = err;
    ptr1 = opendir(a0);
    if (!ptr1)
        return (unsigned long long)sub_406f50(a2, dcgettext(NULL, "cannot open directory %s", 5), a0);
    v10 = ptr1;
    if (g_4273c8)
    {
        v11 = dirfd(ptr1);
        if (v11 >= 0)
        {
            if ((unsigned int)(unsigned long long)sub_40b140(v11, &v1) < 0)
                goto LABEL_40b480;
        }
        else
        {
            if ((unsigned int)(unsigned long long)sub_40b160(a0, &v1) < 0)
            {
LABEL_40b480:
                sub_406f50(a2, dcgettext(NULL, "cannot determine device and inode of %s", 5), a0);
                return closedir(v10);
            }
        }
        v0 = v2;
        if (!(char)sub_407df0(v1, v0))
        {
            sub_407970(v1, v0);
        }
        else
        {
            sub_415700(0, 3, a0);
            error(0, 0, dcgettext(NULL, "%s: not listing already-listed directory", 5));
            closedir(v10);
        }
    }
    sub_406e20();
    if (!g_4272f6)
    {
        if (!g_4272b8)
            goto LABEL_40b2ad;
        if (!g_426010)
        {
LABEL_40b45a:
            sub_407460(10);
        }
    }
    else if (!g_426010)
    {
        goto LABEL_40b45a;
    }
    g_426010 = 0;
    v14 = NULL;
    sub_407810();
    if (g_427311)
    {
        v14 = sub_40de90(a0, 2);
        if (!v14)
            sub_406f50(a2, dcgettext(NULL, "error canonicalizing %s", 5), a0);
    }
    if (!v6)
        v6 = a0;
    sub_408eb0(v6, g_4272c8, 0xffffffff, 0, 1, &g_427140.padding_0[0], v14);
    free(v14);
    sub_4077f0(":\n");
LABEL_40b2ad:
    v0 = 0;
    while (1)
    {
        *(v8) = 0;
        count = readdir(v10);
        if (count)
        {
            v17 = (char *)&count[1].d_ino + 6;
            if (!(char)sub_4069f0(v17))
            {
                v0 += sub_40a550(v17);
                if (g_427338 == 1 && g_427330 == 6 && !g_42732c && !g_4272f6)
                {
                    sub_4089f0();
                    sub_409f20();
                    sub_406e20();
                }
            }
        }
        else
        {
            if (!*(v8))
                break;
            sub_406f50(a2, dcgettext(NULL, "reading directory %s", 5), a0);
            if (*(v8) != 75)
                break;
        }
        sub_407ce0();
    }
    if (closedir(v10))
        sub_406f50(a2, dcgettext(NULL, "closing directory %s", 5), a0);
    sub_4089f0();
    if (g_4272f6)
        sub_40a1e0(a0, 0);
    if (!g_427338 || g_42732c)
    {
        ptr = sub_40ffa0(v0, &v3, g_427328, 0x200, g_427320);
        len = strlen(ptr);
        *((char *)ptr - 1) = 32;
        v22 = ptr + len;
        *(v22) = g_426019;
        sub_407810();
        sub_4077f0(dcgettext(NULL, "total", 5));
        sub_4077d0(ptr - 1, v22 + 1 - (ptr - 1));
    }
    if (g_4273b0)
        return sub_409f20();
    return v4 - *((long long *)(40 + v24));
}



// Function: usage @ 0xb690
typedef struct FILE {
} FILE;

extern unsigned int g_4261e0;
extern FILE *stdout;
extern unsigned long long stderr;

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
    long long v55;  // rdi
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [FILE]...\n", 5));
        v5 = dcgettext(NULL, "List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n", 5);
        fputs_unlocked(v5, stdout);
        sub_406530(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., '--block-size=M'; see SIZE format below\n\n", 5);
        fputs_unlocked(v13, stdout);
        v14 = dcgettext(NULL, "  -B, --ignore-backups       do not list implied entries ending with ~\n", 5);
        fputs_unlocked(v14, stdout);
        v15 = dcgettext(NULL, "  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n", 5);
        fputs_unlocked(v15, stdout);
        v16 = dcgettext(NULL, "  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs' dired mode\n", 5);
        fputs_unlocked(v16, stdout);
        v17 = dcgettext(NULL, "  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append '*'\n", 5);
        fputs_unlocked(v17, stdout);
        v18 = dcgettext(NULL, "      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n", 5);
        fputs_unlocked(v18, stdout);
        v19 = dcgettext(NULL, "      --full-time            like -l --time-style=full-iso\n", 5);
        fputs_unlocked(v19, stdout);
        v20 = dcgettext(NULL, "  -g                         like -l, but do not list owner\n", 5);
        fputs_unlocked(v20, stdout);
        v21 = dcgettext(NULL, "      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n", 5);
        fputs_unlocked(v21, stdout);
        v22 = dcgettext(NULL, "  -G, --no-group             in a long listing, don't print group names\n", 5);
        fputs_unlocked(v22, stdout);
        v23 = dcgettext(NULL, "  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n", 5);
        fputs_unlocked(v23, stdout);
        v24 = dcgettext(NULL, "  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n", 5);
        fputs_unlocked(v24, stdout);
        v25 = dcgettext(NULL, "      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "      --hyperlink[=WHEN]     hyperlink file names WHEN\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n", 5);
        fputs_unlocked(v29, stdout);
        v30 = dcgettext(NULL, "  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n", 5);
        fputs_unlocked(v30, stdout);
        v31 = dcgettext(NULL, "  -l                         use a long listing format\n", 5);
        fputs_unlocked(v31, stdout);
        v32 = dcgettext(NULL, "  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n", 5);
        fputs_unlocked(v32, stdout);
        v33 = dcgettext(NULL, "  -m                         fill width with a comma separated list of entries\n", 5);
        fputs_unlocked(v33, stdout);
        v34 = dcgettext(NULL, "  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n", 5);
        fputs_unlocked(v34, stdout);
        v35 = dcgettext(NULL, "  -q, --hide-control-chars   print ? instead of nongraphic characters\n", 5);
        fputs_unlocked(v35, stdout);
        v36 = dcgettext(NULL, "      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is 'ls' and output is a terminal)\n\n", 5);
        fputs_unlocked(v36, stdout);
        v37 = dcgettext(NULL, "  -Q, --quote-name           enclose entry names in double quotes\n", 5);
        fputs_unlocked(v37, stdout);
        v38 = dcgettext(NULL, "      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n", 5);
        fputs_unlocked(v38, stdout);
        v39 = dcgettext(NULL, "  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n", 5);
        fputs_unlocked(v39, stdout);
        v40 = dcgettext(NULL, "  -S                         sort by file size, largest first\n", 5);
        fputs_unlocked(v40, stdout);
        v41 = dcgettext(NULL, "      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n", 5);
        fputs_unlocked(v41, stdout);
        v42 = dcgettext(NULL, "      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n", 5);
        fputs_unlocked(v42, stdout);
        v43 = dcgettext(NULL, "      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n", 5);
        fputs_unlocked(v43, stdout);
        v44 = dcgettext(NULL, "  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n", 5);
        fputs_unlocked(v44, stdout);
        v45 = dcgettext(NULL, "  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n", 5);
        fputs_unlocked(v45, stdout);
        v46 = dcgettext(NULL, "  -U                         do not sort; list entries in directory order\n", 5);
        fputs_unlocked(v46, stdout);
        v47 = dcgettext(NULL, "  -v                         natural sort of (version) numbers within text\n", 5);
        fputs_unlocked(v47, stdout);
        v48 = dcgettext(NULL, "  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n", 5);
        fputs_unlocked(v48, stdout);
        v49 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v49, stdout);
        v50 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v50, stdout);
        sub_406560(v6, v7, v8, v9, v10, v11);
        v51 = dcgettext(NULL, "\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with 'posix-' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n", 5);
        fputs_unlocked(v51, stdout);
        v52 = dcgettext(NULL, "\nThe WHEN argument defaults to 'always' and can also be 'auto' or 'never'.\n", 5);
        fputs_unlocked(v52, stdout);
        v53 = dcgettext(NULL, "\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n", 5);
        fputs_unlocked(v53, stdout);
        v54 = dcgettext(NULL, "\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n", 5);
        fputs_unlocked(v54, stdout);
        v55 = "ls";
        if (g_4261e0 != 1)
        {
            v55 = "dir";
            if (g_4261e0 != 2)
                v55 = "vdir";
        }
        sub_407e40(v55);
    }
    exit(a0); /* do not return */
}



// Function: decode_switches @ 0xbda0
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    struct struct_2 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

typedef struct struct_0 {
    char padding_0[8];
    long long field_8;
    char field_10;
} struct_0;

extern struct_0 g_419174;
extern unsigned int g_4196c0[4];
extern unsigned int g_4196f0[4];
extern struct_0 g_419710;
extern struct_0 g_419730;
extern unsigned int g_419750[4];
extern long long g_419760;
extern long long g_41abb9;
extern void g_41d9c0;
extern long long g_425200;
extern long long g_425240;
extern long long g_425280;
extern char g_4252c0;
extern long long g_425940;
extern unsigned long long g_425980;
extern long long g_4259e0;
extern char g_426019;
extern unsigned long long g_426020;
extern char g_426028;
extern char g_426029;
extern char *g_426040;
extern char *g_426048;
extern char g_4261e0;
extern long long g_4261f0;
extern long long stdout;
extern unsigned int optind;
extern char optarg;
extern FILE *stderr;
extern char g_427200;
extern unsigned long long g_4272b0;
extern unsigned long long g_4272c0;
extern long long g_4272c8;
extern long long g_4272d0;
extern char g_4272d8;
extern unsigned long long g_4272e0;
extern unsigned int g_4272f0;
extern char g_4272f4;
extern char g_4272f5;
extern char g_4272f6;
extern unsigned int g_4272f8;
extern char g_4272fc;
extern char g_427311;
extern char g_427312;
extern unsigned int g_427314;
extern char g_427318;
extern unsigned int g_42731c;
extern unsigned long long g_427320;
extern unsigned int g_427328;
extern char g_42732c;
extern char g_42732d;
extern char g_42732e;
extern char g_42732f;
extern unsigned int g_427330;
extern unsigned int g_427334;
extern unsigned int g_427338;
extern char g_427365;
extern char g_4273a8;

long long decode_switches(int a0, char **a1)
{
    unsigned int v12;  // eax
    unsigned int v22;  // eax
    unsigned long v23;  // rax
    unsigned long long v24;  // r10
    unsigned int v25;  // eax
    unsigned long v26;  // rax
    unsigned int v27;  // eax
    unsigned long v28;  // rax
    char *v29;  // rax
    long long v31;  // rax
    unsigned long v14;  // rax
    unsigned long v32;  // rax
    char v34;  // al
    char v35;  // al
    unsigned int v36;  // ebx
    unsigned long v37;  // rax
    unsigned long iter;  // rbx
    char i;  // al
    char v40;  // al
    char v41;  // 4110
    unsigned int v15;  // edx
    int v42;  // eax
    unsigned long long *v43;  // rbx
    long long v44;  // rax
    char *v45;  // rax
    char *v46;  // rax
    char *v47;  // rbp
    char *ptr;  // rax
    char *v49;  // r12
    char v16;  // al
    struct_1 *v17;  // rax
    unsigned long v18;  // rdx
    unsigned long long v19;  // rdi
    unsigned int v20;  // edx
    char v21;  // al
    unsigned long long v0;  // [bp-0x88]
    unsigned long v1;  // [bp-0x80], Other Possible Types: unsigned long long
    int v2;  // [bp-0x74]
    unsigned int v3;  // [bp-0x70]
    int v4;  // [bp-0x6c]
    long long v5;  // [bp-0x68]
    char *node;  // [bp-0x60]
    long long v7;  // [bp-0x58]
    unsigned int v8;  // [bp-0x50]
    char v9;  // [bp-0x49]
    long long v10;  // [bp-0x48]

    v4 = -0x1;
    v5 = -0x1;
    v7 = -0x1;
    v3 = 0xffffffff;
    v8 = 0xffffffff;
    v2 = -0x1;
    v9 = 0;
    node = NULL;
    while (1)
    {
        *((unsigned int *)&v10) = 0xffffffff;
        v12 = getopt_long(a0, a1, "abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1", &g_4252c0, &v10);
        if (v12 == 0xffffffff)
            break;
        switch (v12)
        {
        case -131:
            v1 = v19;
            v0 = 0;
            sub_416870(stdout);
            exit(0); /* do not return */
        case -130:
            sub_40b690(0); /* do not return */
        case 49:
            v2 = v2;
            break;
        case 65:
            g_4272f0 = 1;
            break;
        case 66:
            sub_406380("*~");
            sub_406380(".*~");
            break;
        case 67:
            v2 = 2;
            break;
        case 68:
            g_427318 = 1;
            break;
        case 70:
            if (!*((long long *)&optarg) || (v1 = v24, v0 = 1, v25 = g_4196c0[sub_40d270("--classify")], g_4196c0[sub_40d270("--classify")] == 1 || g_4196c0[sub_40d270("--classify")] == 2 && (char)sub_406590()))
            {
                g_427314 = 3;
                break;
            }
            break;
        case 71:
            g_426028 = 0;
            break;
        case 72:
            g_4272f8 = 2;
            break;
        case 73:
            sub_406380(*((long long *)&optarg));
            break;
        case 76:
            g_4272f8 = 4;
            break;
        case 78:
            v3 = 0;
            break;
        case 81:
            v3 = 5;
            break;
        case 82:
            g_4272f6 = 1;
            break;
        case 83:
            v4 = 3;
            break;
        case 84:
            v23 = dcgettext(NULL, "invalid tab size", 5);
            v7 = sub_416de0(*((long long *)&optarg), 0, 0, 0x7fffffffffffffff, &g_41abb9, v23, 2);
            break;
        case 85:
            v4 = 6;
            break;
        case 88:
            v4 = 1;
            break;
        case 90:
            g_427365 = 1;
            break;
        case 97:
            g_4272f0 = 2;
            break;
        case 98:
            v3 = 7;
            break;
        case 99:
            g_427334 = 1;
            break;
        case 100:
            g_4272f5 = 1;
            break;
        case 102:
            g_4272f0 = 2;
            g_427312 = 0;
            g_427311 = 0;
            g_42732c = 0;
            v4 = 6;
            v2 = (v2 ? v2 : -0x1);
            break;
        case 103:
            g_426029 = 0;
            v2 = 0;
            break;
        case 104:
            g_427328 = 176;
            g_42731c = 176;
            g_427320 = 1;
            g_426020 = 1;
            break;
        case 105:
            g_4272fc = 1;
            break;
        case 107:
            v9 = 1;
            break;
        case 108:
            v2 = 0;
            break;
        case 109:
            v2 = 4;
            break;
        case 110:
            g_42732d = 1;
            v2 = 0;
            break;
        case 111:
            g_426028 = 0;
            v2 = 0;
            break;
        case 112:
            g_427314 = 1;
            break;
        case 113:
            v8 = 1;
            break;
        case 114:
            g_42732f = 1;
            break;
        case 115:
            g_42732c = 1;
            break;
        case 116:
            v4 = 5;
            break;
        case 117:
            g_427334 = 2;
            break;
        case 118:
            v4 = 4;
            break;
        case 119:
            v5 = sub_4063b0();
            if (v5 < NULL)
            {
                sub_4158c0(*((long long *)&optarg));
                dcgettext(NULL, "invalid line width", 5);
                error(2, 0, "%s: %s");
            }
            break;
        case 120:
            v2 = 3;
            break;
        case 128:
            g_42732e = 1;
            break;
        case 129:
            v22 = sub_410810(*((long long *)&optarg), &g_427328, &g_427320);
            if (v22)
                sub_417110(v22, (unsigned int)v10, 0, &g_4252c0, *((long long *)&optarg)); /* do not return */
            g_42731c = g_427328;
            g_426020 = g_427320;
            break;
        case 130:
            if (*((long long *)&optarg) && !(v1 = v19, v0 = 1, v20 = g_4196c0[sub_40d270("--color")], g_4196c0[sub_40d270("--color")] == 1))
            {
                v21 = 0;
                if (g_4196c0[sub_40d270("--color")] == 2)
                    v21 = sub_406590();
            }
            else
            {
                v21 = 1;
            }
            g_427312 = v21;
            g_427312 = g_427312 & 1;
            break;
        case 131:
            g_4272f8 = 3;
            break;
        case 132:
            g_427314 = 2;
            break;
        case 133:
            v2 = *((int *)&g_419730.padding_0[4 * sub_40d270("--format", *((long long *)&optarg), &g_425280, &g_419730.padding_0[0], 4, g_4261f0, 1)]);
            break;
        case 134:
            v2 = 0;
            node = sub_4055b0("full-iso");
            break;
        case 135:
            g_4272f4 = 1;
            break;
        case 136:
            v17 = sub_4169e0(16);
            v17->field_0 = *((long long *)&optarg);
            v18 = g_4272e0;
            g_4272e0 = v17;
            v17->field_8 = v18;
            break;
        case 137:
            if (*((long long *)&optarg) && !(v1 = (unsigned long)(v14 + &g_419174.padding_0[0]), v0 = 1, v15 = g_4196c0[sub_40d270("--hyperlink")], g_4196c0[sub_40d270("--hyperlink")] == 1))
            {
                v16 = 0;
                if (g_4196c0[sub_40d270("--hyperlink")] == 2)
                    v16 = sub_406590();
            }
            else
            {
                v16 = 1;
            }
            g_427311 = v16;
            g_427311 = g_427311 & 1;
            break;
        case 138:
            g_427314 = g_419750[sub_40d270("--indicator-style", *((long long *)&optarg), &g_425940, &g_419750[0], 4, g_4261f0, 1)];
            break;
        case 139:
            v3 = *((int *)&(&g_41d9c0)[4 * sub_40d270("--quoting-style", *((long long *)&optarg), &g_4259e0, &g_41d9c0, 4, g_4261f0, 1)]);
            break;
        case 141:
            g_427328 = 144;
            g_42731c = 144;
            g_427320 = 1;
            g_426020 = 1;
            break;
        case 142:
            v4 = *((int *)&g_419710.padding_0[4 * sub_40d270("--sort", *((long long *)&optarg), &g_425240, &g_419710.padding_0[0], 4, g_4261f0, 1)]);
            break;
        case 143:
            g_427334 = g_4196f0[sub_40d270("--time", *((long long *)&optarg), &g_425200, &g_4196f0[0], 4, g_4261f0, 1)];
            break;
        case 144:
            node = *((long long *)&optarg);
            break;
        case 145:
            g_426019 = 0;
            g_427312 = 0;
            v3 = 0;
            v2 = v2;
        case 140:
            v8 = 0;
            break;
        default:
            sub_40b690(2); /* do not return */
        }
    }
    if (!g_427320)
    {
        v26 = getenv("LS_BLOCK_SIZE");
        sub_410810(v26, &g_427328, &g_427320);
        if (v26 || getenv("BLOCK_SIZE"))
        {
            g_42731c = g_427328;
            g_426020 = g_427320;
        }
        if (v9)
        {
            g_427328 = 0;
            g_427320 = 0x400;
        }
    }
    if (v2 >= 0)
    {
        g_427338 = v2;
        if (g_427338 - 2 > 2)
            goto LABEL_40c89f;
        goto LABEL_40c580;
    }
    else
    {
        if (*((int *)&g_4261e0) != 1)
        {
            if (*((int *)&g_4261e0) == 2)
                goto LABEL_40cae0;
            v27 = 0;
            goto LABEL_40c899;
        }
        if ((char)sub_406590())
        {
LABEL_40cae0:
            g_427338 = 2;
LABEL_40c580:
            if (v5 == -0x1)
            {
                if (!(char)sub_406590() || ioctl(1, 21523) < 0 || !(v28 = (unsigned long)(unsigned long long)*((unsigned short *)((void*)&v10 + 2)), (unsigned short)v28))
                {
                    v29 = getenv("COLUMNS");
                    if (!v29 || !*(v29))
                    {
LABEL_40c8b8:
                        v5 = 80;
                        goto LABEL_40c58c;
                    }
                    else
                    {
                        v5 = sub_4063b0();
                        if (v5 < NULL)
                        {
                            sub_4158c0(v29);
                            error(0, 0, dcgettext(NULL, "ignoring invalid width in environment variable COLUMNS: %s", 5));
                            v5 = 80;
                            goto LABEL_40c58c;
                        }
                    }
                }
                else
                {
                    v5 = v28;
                    goto LABEL_40c58c;
                }
            }
        }
        else
        {
            v27 = 1;
LABEL_40c899:
            g_427338 = v27;
LABEL_40c89f:
            if (g_427312)
                goto LABEL_40c580;
            if (v5 == -0x1)
                goto LABEL_40c8b8;
LABEL_40c58c:
            g_4272b0 = v5;
            *((unsigned long long *)&g_427200) = (v5 != (v5 * 12297829382473034411 >> 64 & 0xfffffffffffffffe) + v5 / 3) + v5 / 3;
            if (g_427338 - 2 <= 2)
            {
                v31 = v7;
                if (v31 >= NULL)
                {
LABEL_40c7a4:
                    g_4272c0 = v31;
                }
                else
                {
                    g_4272c0 = 8;
                    v32 = getenv("TABSIZE");
                    if (v32)
                    {
                        if (!sub_417190(v32, 0, 0, &v10, &g_41abb9))
                        {
                            v31 = v10;
                            goto LABEL_40c7a4;
                        }
                        else
                        {
                            sub_4158c0(v32);
                            error(0, 0, dcgettext(NULL, "ignoring invalid tab size in environment variable TABSIZE: %s", 5));
                        }
                    }
                }
            }
            v34 = (char)v8 & 1;
            if (v8 == 0xffffffff)
            {
                v35 = 0;
                if (*((int *)&g_4261e0) == 1)
                    v35 = sub_406590();
                v34 = v35 & 1;
            }
            g_4272d8 = v34;
            if (v3 >= 0 || (v3 = (unsigned int)(unsigned long long)sub_4064b0(), v3 >= 0 || (v3 = 7, *((int *)&g_4261e0) != 1)))
            {
LABEL_40c5fd:
                sub_415200(0, v3);
                goto LABEL_40c608;
            }
            else if (!(char)sub_406590())
            {
LABEL_40c608:
                v36 = sub_4151e0(0);
                if (g_427338 && (g_427338 - 2 > 1 || !g_4272b0) || v36 != 3 && v36 != 6 && v36 != 1)
                {
                    g_4273a8 = 0;
                    g_4272d0 = sub_4151a0(0);
                    if (v36 == 7)
                        sub_415220(g_4272d0, 32, 1);
                }
                else
                {
                    g_4273a8 = 1;
                    g_4272d0 = sub_4151a0(0);
                }
                if (g_427314 > 1)
                {
                    v37 = g_427314 - 2;
                    iter = v37 + 4304164;
                    i = *((char *)(v37 + 4304164));
                    if (*((char *)(v37 + 4304164)))
                    {
                        do
                        {
                            iter += 1;
                            sub_415220(g_4272d0, i, 1);
                            i = *((char *)iter);
                        } while (i);
                    }
                }
                g_4272c8 = sub_4151a0(0);
                sub_415220(g_4272c8, 58, 1);
                v40 = (g_427311 ^ 1) & !g_427338;
                v41 = g_427318;
                g_427318 = v40 & g_427318;
                if ((v40 & v41) > g_426019)
                    error(2, 0, dcgettext(NULL, "--dired and --zero are incompatible", 5));
                v42 = v4;
                if (v42 >= 0)
                {
LABEL_40c700:
                    g_427330 = v42;
                    if (!g_427338)
                        goto LABEL_40c7c2;
                }
                else if (!g_427338)
                {
                    g_427330 = 0;
LABEL_40c7c2:
                    if (!node)
                    {
                        node = getenv("TIME_STYLE");
                        if (!node)
                            node = sub_4055b0("locale");
                    }
                    for (; !strncmp(node, "posix-", 6); node += 6)
                    {
                        if (!(char)sub_40e9c0(2))
                            return optind;
                    }
                    node = node;
                    if (*(node) != 43)
                    {
                        v43 = &g_425980;
                        v44 = sub_40cf70(node, &g_425980, &g_419760, 4);
                        if (v44 >= NULL)
                        {
                            if (v44 != 2)
                            {
                                if (v44 <= 2)
                                {
                                    if (v44)
                                    {
                                        if (v44 == 1)
                                        {
                                            g_426048 = "%Y-%m-%d %H:%M";
                                            g_426040 = "%Y-%m-%d %H:%M";
                                        }
                                    }
                                    else
                                    {
                                        g_426048 = "%Y-%m-%d %H:%M:%S.%N %z";
                                        g_426040 = "%Y-%m-%d %H:%M:%S.%N %z";
                                    }
                                }
                                else
                                {
                                    if (v44 == 3 && (char)sub_40e9c0(2))
                                    {
                                        g_426040 = dcgettext(NULL, g_426040, 2);
                                        g_426048 = dcgettext(NULL, g_426048, 2);
                                    }
                                }
                            }
                            else
                            {
                                g_426040 = "%Y-%m-%d ";
                                g_426048 = "%m-%d %H:%M";
                            }
                        }
                        else
                        {
                            sub_40d0e0("time style", node, v44);
                            v45 = dcgettext(NULL, "Valid arguments are:\n", 5);
                            fputs_unlocked(v45, stderr);
                            do
                            {
                                v43 += 1;
                                __fprintf_chk(*((unsigned int *)&stderr), 0x1, "  - [posix-]%s\n");
                            } while (*(v43));
                            v46 = dcgettext(NULL, "  - +FORMAT (e.g., +%H:%M) for a 'date'-style format\n", 5);
                            fputs_unlocked(v46, stderr);
                        }
                    }
                    else
                    {
                        v47 = node + 1;
                        ptr = strchr(v47, 10);
                        if (ptr)
                        {
                            v49 = ptr + 1;
                            if (strchr(v49, 10))
                            {
                                sub_4158c0(v47);
                                error(2, 0, dcgettext(NULL, "invalid time style format %s", 5));
                            }
                            *(ptr) = 0;
                        }
                        else
                        {
                            v49 = v47;
                        }
                        g_426040 = v47;
                        g_426048 = v49;
                    }
                    sub_4066c0();
                }
                else if (g_427334 - 1 > 2)
                {
                    v42 = 0;
                    goto LABEL_40c700;
                }
                else
                {
                    g_427330 = 5;
                }
            }
            else
            {
                v3 = 3;
                goto LABEL_40c5fd;
            }
        }
    }
    return optind;
}



// Function: is_prime @ 0xea50
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



// Function: next_prime @ 0xeac0
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
    while (!sub_40ea50(v2, v3))
    {
        v2 = v4 + 2;
        if (v2 == 0xffffffffffffffff)
            return v4;
    }
    return v4;
}



// Function: raw_hasher @ 0xeb00
long long raw_hasher(unsigned long a0, unsigned long long a1)
{
    return sub_417eb0() % a1;
}



// Function: raw_comparator @ 0xeb20
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0xeb30
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

extern unsigned int g_41d2f0[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_41d2f0[0])
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
    idx->field_28 = &g_41d2f0[0];
    return 0;
}



// Function: free_entry @ 0xebc0
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



// Function: safe_hasher @ 0xebe0
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



// Function: hash_find_entry @ 0xec10
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

    v2 = sub_40ebe0(a0, a1);
    *(a2) = v2;
    v3 = *((long long *)v2);
    if (!v3)
    {
LABEL_40ecb1:
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
                    sub_40ebc0(a0, v6);
                    return v0;
                }
                return *((long long *)v6);
            }
        }
        goto LABEL_40ecb1;
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
    sub_40ebc0(a0, v7);
    return v0;
}



// Function: allocate_entry @ 0xed10
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



// Function: transfer_entries @ 0xed40
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
LABEL_40ed68:
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
                    idx2 = sub_40ebe0(idx, *((long long *)&v3->field_0));
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
                        sub_40ebc0(idx, v5);
                        v3 = v6;
                        if (!v3)
                            break;
                    }
                }
                v2 = iter->field_0;
            }
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_40ed68;
            index = sub_40ebe0(idx, v2);
            if (index->field_0)
            {
                v8 = sub_40ed10(idx);
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



// Function: hash_get_n_buckets @ 0xef00
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0xef10
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0xef20
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0xef30
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



// Function: hash_print_statistics @ 0xf000
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
    sub_40f00d();
    return;
}



// Function: hash_lookup @ 0xf120
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
    struct_0 *iter;  // rax
    unsigned long long i;  // rsi

    iter = sub_40ebe0(a0, a1);
    if (!iter->field_0)
        return 0;
    for (i = iter->field_0; i != a1; i = iter->field_0)
    {
        if ((char)a0[1].field_0(a1))
            return iter->field_0;
        iter = (struct_0 *)iter->padding_8;
        if (!iter)
            return 0;
    }
    return a1;
}



// Function: hash_get_next @ 0xf1d0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40f1dc();
    return;
}



// Function: hash_do_for_each @ 0xf290
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



// Function: hash_initialize @ 0xf380
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

extern struct_0 g_41d2f0;

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

    v1 = sub_40eb00;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_40eb20;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_41d2f0.padding_0[0];
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_40eb30(ptr))
    {
        v6 = sub_40ee70(a0, v2->field_10);
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



// Function: hash_clear @ 0xf490
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



// Function: hash_free @ 0xf540
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
            goto LABEL_40f5ec;
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
LABEL_40f5ec:
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



// Function: hash_rehash @ 0xf630
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

    v11 = sub_40ee70(a1, idx->field_28->field_10);
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
        v13 = sub_40ed40(&ptr, idx, 0);
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
        if ((char)sub_40ed40(idx, &ptr, 1) && (char)sub_40ed40(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0xf7b0
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
    v2 = sub_40ec10(idx, a1, &index, 0);
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
            sub_40eb30(idx);
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
                else if (!(char)sub_40f630(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else if (sub_40ec10(idx, a1, &index, 0))
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
        v11 = sub_40ed10(idx);
        if (!v11)
            return 0xffffffff;
        v11->field_0 = a1;
        v11->field_8 = index->field_8;
        index->field_8 = v11;
        *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
        return 1;
    }
}



// Function: hash_insert @ 0xf9a0
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x18]

    v2 = sub_40f7b0(a0, a1, &v0);
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



// Function: hash_remove @ 0xfa00
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

    v2 = sub_40ec10(idx, a1, &v0, 1);
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
            goto LABEL_40fb46;
LABEL_40fa90:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = idx[2].field_0;
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_40fa90;
LABEL_40fb46:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_40eb30(idx);
    v9 = idx[5].field_0;
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_40f630(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



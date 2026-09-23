// Function: main @ 0x29a0
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

typedef struct FILE {
} FILE;

extern char g_40faa9;
extern char g_415020;
extern char *g_415028;
extern FILE *stdout;
extern char optind;
extern char g_41510c;
extern char g_41510e;
extern unsigned int g_415110;
extern char g_415114;
extern char g_415115;
extern char g_415116;
extern char g_415117;
extern char g_415118;
extern char g_415119;

unsigned int main(unsigned int a0, long long *a1)
{
    char v7;  // al
    unsigned int v8;  // ebx
    void* iter;  // rax
    void* v18;  // r15
    void* j;  // rbp
    char **v20;  // rbx
    unsigned int v21;  // ebx
    void* v22;  // rbp
    unsigned int v24;  // ebp
    int *err;  // r12
    char *v26;  // rax
    long long *v9;  // rbp
    char *v27;  // rax
    int *err1;  // rax
    char *v29;  // rax
    int *err2;  // rax
    char **i;  // rdx
    char v11;  // r14b
    char **v12;  // r12
    char **v13;  // rbp
    unsigned int v14;  // eax
    long long v0;  // [bp-0x100]
    unsigned int v1;  // [bp-0xf4]
    long long v2;  // [bp-0xf0]
    unsigned long v3;  // [bp-0xe8]
    char v4;  // [bp-0xe0]
    stat v5;  // [bp-0xd8]

    v1 = 0;
    v2 = 10;
    v3 = 0x3ff0000000000000;
    sub_4091a0(*(a1));
    setlocale(6, &g_40faa9);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_40d460(sub_407b00);
    g_41510e = 0;
    g_415119 = 1;
    g_415115 = 0;
    g_415116 = 0;
    g_415118 = 0;
    g_415114 = 10;
    v7 = sub_403bc0(a0, a1, &v2);
    v8 = a0 - v7;
    v9 = &a1[v7];
    sub_407080(v8, v9, &v2, &v1, &v3);
    if (g_415116 && v2)
        v2 -= 1;
    i = &g_415028;
    v0 = 1;
    if (*((int *)&optind) < v8)
    {
        i = &v9[*((int *)&optind)];
        v0 = v8 - *((int *)&optind);
    }
    v11 = 0;
    v12 = &i[v0];
    v13 = i;
    do
    {
        if (!strcmp(*(v13), "-"))
            v11 = 1;
    } while ((v13 += 8, v12 != v13));
    if (v11)
    {
        if (*((int *)&g_415020) == 1)
        {
            sub_40b410(4, "-");
            error(1, 0, dcgettext(NULL, "cannot follow %s by name", 5));
        }
        if (!g_415118)
        {
            if (!v2)
            {
LABEL_402c53:
                if (!g_415116)
                    return 0;
            }
LABEL_402b6a:
            iter = sub_40c0c0(v0, 96);
            v18 = iter;
            j = iter;
            do
            {
                v20 = i + 1;
                iter += 96;
                *((char **)((char *)iter - 96)) = *(i);
                i = v20;
            } while (i != v12);
            if (v1 == 1 || v0 != 1 && !v1)
                g_415115 = 1;
            v21 = 1;
            do
            {
                v22 = j + 96;
                v21 &= sub_406ab0(j, v2);
                j = v22;
            } while (v0 * 96 + v18 != j);
            if (g_415118 && sub_403a50(v18, v0))
            {
                if (fstat(1, &v5) >= 0)
                {
                    g_415117 = (v5.st_nlink & 0xf000) == 0x1000;
                    if (!g_41510c && !(char)sub_403ae0(v18, v0) && !(char)sub_4038d0(v18, v0) && (char)sub_403900(v18) && !(char)sub_404b40(v18) && !(char)sub_403930(v18, v0) && ((char)v21 || *((int *)&g_415020) != 2))
                    {
                        v24 = inotify_init();
                        err = __errno_location();
                        if (v24 >= 0)
                        {
                            if (!fflush_unlocked(stdout))
                            {
                                sub_405880(v24, v18, v0, &v4);
                                sub_408a10(v4);
                                close(v24);
                                *(err) = 0;
                            }
                            else
                            {
                                v26 = dcgettext(NULL, "write error", 5);
                                error(1, *(err), v26);
                            }
                        }
                        v27 = dcgettext(NULL, "inotify cannot be used, reverting to polling", 5);
                        error(0, *(err), v27);
                    }
                    g_41510c = 1;
                    sub_406220(v18, v0);
                }
                else
                {
                    v29 = dcgettext(NULL, "standard output", 5);
                    err2 = __errno_location();
                    error(1, *(err2), v29);
                }
            }
            if (!g_41510e || close(0) >= 0)
                return (char)v21 ^ 1;
            err1 = __errno_location();
            error(1, *(err1), "-");
            return (char)v21 ^ 1;
        }
        else if ((g_415110 || *((int *)&g_415020) != 2 || v0 != 1 || (v14 = (unsigned int)(int)(long long)fstat(0, &v5), (int)(long long)fstat(0, &v5) || ((unsigned short)v5.st_nlink & 0xf000) == 0x8000)) && isatty(0))
        {
            error(0, 0, dcgettext(NULL, "warning: following standard input indefinitely is ineffective", 5));
        }
    }
    if (!v2 && !g_415118)
        goto LABEL_402c53;
    goto LABEL_402b6a;
}



// Function: record_open_fd @ 0x3830
typedef struct struct_1 {
    unsigned long long field_0;
    unsigned long long field_8;
    char padding_10[8];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned int field_30;
    char field_34;
    char padding_35[3];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
    char padding_44[20];
    unsigned long long field_58;
} struct_0;

unsigned int record_open_fd(struct_0 *idx, unsigned int a1, unsigned long a2, struct_1 *index, unsigned int a4)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // rdx
    unsigned int v3;  // eax

    idx->field_38 = a1;
    idx->field_8 = a2;
    idx->field_10 = sub_40b7b0(index);
    v1 = index->field_0;
    idx->field_18 = v2;
    idx->field_20 = v1;
    idx->field_28 = index->field_8;
    v3 = index->field_18;
    idx->field_40 = a4;
    idx->field_30 = v3;
    idx->field_58 = 0;
    idx->field_34 = 0;
    return v3;
}



// Function: any_live_files @ 0x3880
typedef struct struct_0 {
    char padding_0[52];
    char field_34;
    char padding_35[3];
    int field_38;
} struct_0;

extern unsigned int g_415020;
extern char g_41511a;

unsigned long long any_live_files(struct_0 *a0, unsigned long a1)
{
    struct_0 *v1;  // rdi
    struct_0 *v2;  // rdi
    unsigned long v3;  // rax
    unsigned long long v4;  // rax

    if (g_41511a && g_415020 == 1)
        return g_41511a;
    v1 = &a0->field_34;
    v2 = v1;
    while (*((int *)&v2->padding_0[4]) < 0)
    {
        v3 = v2->padding_0[0] ^ 1;
        v4 = _INSERT(v3, 0, (char)v3 & g_41511a);
        if ((char)v3 & g_41511a)
            return v4;
        v2 = &v2[1].padding_0[36];
        if (v2 == 96 * a1 + (char *)v1)
            return v4;
    }
    return 1;
}



// Function: any_remote_file @ 0x38d0
typedef struct struct_0 {
    char padding_0[53];
    char field_35;
    char padding_36[2];
    int field_38;
} struct_0;

unsigned int any_remote_file(struct_0 *a0, unsigned long a1)
{
    struct_0 *v1;  // rdi
    struct_0 *v2;  // rdi
    unsigned int v3;  // r8d

    v1 = &a0->field_35;
    v2 = v1;
    while (1)
    {
        if (*((int *)&v2->padding_0[3]) >= 0)
        {
            v3 = v2->padding_0[0];
            if ((char)v3)
                return v3;
        }
        v2 = &v2[1].padding_0[36];
        if (v2 == 96 * a1 + (char *)v1)
            return 0;
    }
}



// Function: any_non_remote_file @ 0x3900
typedef struct struct_0 {
    char padding_0[53];
    char field_35;
    char padding_36[2];
    int field_38;
} struct_0;

unsigned int any_non_remote_file(struct_0 *a0, unsigned long a1)
{
    struct_0 *v1;  // rdi
    struct_0 *v2;  // rdi

    v1 = &a0->field_35;
    v2 = v1;
    while (1)
    {
        if (*((int *)&v2->padding_0[3]) >= 0 && !v2->padding_0[0])
            return 1;
        v2 = &v2[1].padding_0[36];
        if (v2 == 96 * a1 + (char *)v1)
            return 0;
    }
}



// Function: any_non_regular_fifo @ 0x3930
typedef struct struct_0 {
    char padding_0[48];
    unsigned int field_30;
    char padding_34[4];
    unsigned int field_38;
} struct_0;

unsigned int any_non_regular_fifo(struct_0 *a0, unsigned long a1)
{
    struct_0 *v1;  // rdi
    struct_0 *iter;  // rdi
    unsigned long v3;  // rax
    unsigned int v4;  // edx

    v1 = &a0->field_30;
    iter = v1;
    while (1)
    {
        v3 = *((int *)&iter->padding_0[8]);
        if ((unsigned int)v3 >= 0)
        {
            v4 = *((int *)&iter->padding_0[0]) & 0xf000;
            if (v4 != 0x1000 & v4 != 0x8000)
                return _INSERT(v3, 0, v4 != 0x1000 & v4 != 0x8000);
        }
        iter = &iter[1].padding_0[36];
        if (96 * a1 + (char *)v1 == iter)
            return 0;
    }
}



// Function: wd_hasher @ 0x3980
void wd_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: wd_comparator @ 0x39a0
void wd_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: write_header @ 0x3a10
extern char g_40faa9;
extern char g_415010;

int write_header(char *a0)
{
    char *v1;  // rax
    char *v2;  // rdx
    int v3;  // eax

    v1 = "\n";
    v2 = &g_40faa9;
    if (g_415010)
        v1 = v2;
    v3 = __printf_chk(1, "%s==> %s <==\n", v1, a0);
    g_415010 = 0;
    return v3;
}



// Function: ignore_fifo_and_pipe @ 0x3a50
typedef struct struct_0 {
    char *field_0;
    char padding_8[40];
    unsigned int field_30;
    char field_34;
    char padding_35[3];
    int field_38;
} struct_0;

unsigned long long ignore_fifo_and_pipe(struct_0 *a0, unsigned long a1)
{
    unsigned long long v1;  // r12
    struct_0 *iter;  // rbx

    v1 = 0;
    iter = a0;
    do
    {
        if (strcmp(iter->field_0, "-") || iter->field_34 || iter->field_38 < 0 || ((unsigned short)iter->field_30 & 0xf000) != 0x1000)
        {
            v1 += 1;
        }
        else
        {
            iter->field_38 = -0x1;
            iter->field_34 = 1;
        }
    } while ((iter += 96, a1 * 96 + a0 != iter));
    return v1;
}



// Function: tailable_stdin @ 0x3ae0
typedef struct struct_0 {
    char *field_0;
    char padding_8[44];
    char field_34;
} struct_0;

unsigned int tailable_stdin(struct_0 *a0, unsigned long a1)
{
    struct_0 *v1;  // rbx

    v1 = a0;
    while (1)
    {
        if (!v1->field_34 && !strcmp(v1->field_0, "-"))
            return 1;
        v1 = &v1[1].padding_8[35];
        if (v1 == 96 * a1 + (char *)a0)
            return 0;
    }
}



// Function: close_fd @ 0x3b30
long long close_fd(unsigned int a0, long long a1)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    unsigned long v7;  // rax
    char *v8;  // rax
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    if (a0 + 1 <= 1)
        return a0 + 1;
    v2 = v4;
    v1 = v5;
    v0 = v6;
    v7 = close(a0);
    if ((unsigned int)v7)
    {
        sub_40b410(4, a1);
        v8 = dcgettext(NULL, "closing %s (fd=%d)", 5);
        return (unsigned long long)error(0, *(__errno_location()), v8);
    }
    return v7;
}



// Function: parse_obsolete_option @ 0x3bc0
typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_1 {
    char padding_0[8];
    struct struct_0 *field_8;
    struct struct_2 *field_10;
} struct_1;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern char g_415116;
extern char g_415118;
extern char g_415119;

unsigned int parse_obsolete_option(unsigned int a0, struct_1 *idx, unsigned long long *a2)
{
    char *v1;  // rdi
    unsigned int v2;  // eax
    unsigned int v11;  // ecx
    unsigned long v12;  // rax
    char v13;  // r14b
    int *err;  // rax
    struct_0 *v3;  // rdx
    unsigned long v4;  // rdi
    char v5;  // bpl
    unsigned long v7;  // rdx
    char v8;  // cl
    char v10;  // r13b

    if (a0 != 2)
    {
        if (a0 != 3)
        {
            if (a0 != 4)
                return 0;
            v1 = &idx->field_10->field_0;
LABEL_403be5:
            if (strcmp(v1, "--"))
                return 0;
        }
        else
        {
            v1 = &idx->field_10->field_0;
            if (*(v1) == 45 && v1[1])
                goto LABEL_403be5;
        }
    }
    v2 = sub_409110();
    v3 = idx->field_8;
    v4 = &v3->field_1;
    if (v3->field_0 == 43)
    {
        v5 = 1;
        if (v2 - 200112 > 696)
            goto LABEL_403c37;
    }
    else if (v3->field_0 == 45 && (v2 <= 200111 || (&v3->field_1)[v3->field_1 == 99]))
    {
        v5 = 0;
LABEL_403c37:
        v7 = v4;
        if (v3->field_1 - 48 <= 9)
        {
            do
            {
                v7 += 1;
            } while (*((char *)(v7 + 1)) - 48 <= 9);
        }
        switch (v8)
        {
        case 99:
            v10 = 0;
            v11 = 10;
            break;
        case 108:
            v10 = 1;
            v11 = 10;
            break;
        case 98:
            v10 = 0;
            v11 = 0x1400;
            break;
        default:
            v10 = 1;
            v11 = 10;
            v12 = v7;
            goto LABEL_403c8b;
        }
        v12 = v7 + 1;
LABEL_403c8b:
        v13 = 0;
        if (*((char *)v12) == 0x66)
        {
            v12 += 1;
            v13 = 1;
        }
        if (!*((char *)v12))
        {
            if (v4 == v7)
            {
                *(a2) = v11;
            }
            else if (sub_40c6c0() & 0xfffffffd)
            {
                sub_40b6d0(idx->field_8);
                dcgettext(NULL, "invalid number", 5);
                err = __errno_location();
                error(1, *(err), "%s: %s");
            }
            g_415116 = v5;
            g_415119 = v10;
            g_415118 = v13;
            return 1;
        }
    }
    return 0;
}



// Function: xwrite_stdout @ 0x3da0
typedef struct FILE {
} FILE;

extern FILE *stdout;

long long xwrite_stdout(void* a0, unsigned long a1)
{
    unsigned long v4;  // rax
    unsigned long long v5;  // r13
    unsigned long long v6;  // r12
    unsigned long long v7;  // rbx
    unsigned long count;  // rax
    char *v9;  // rax
    int *err;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    if (!a1)
        return v4;
    v2 = v5;
    v1 = v6;
    v0 = v7;
    count = fwrite_unlocked(a0, 1, a1, stdout);
    if (count >= a1)
        return count;
    clearerr_unlocked(stdout);
    sub_40b410(4, "standard output");
    v9 = dcgettext(NULL, "error writing %s", 5);
    err = __errno_location();
    error(1, *(err), v9);
}



// Function: start_lines @ 0x3e30
extern char g_415114;

unsigned int start_lines(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    unsigned long long v3;  // rbx
    char *v4;  // r14
    unsigned long long v5;  // rax
    char *v6;  // rbp
    unsigned long long v7;  // rbx
    char *v8;  // rdi
    char *ptr;  // rax
    char *v10;  // rax
    int *err;  // rax
    long long v0;  // [bp-0x2050], Other Possible Types: unsigned long
    char v1;  // [bp-0x2048]

    v0 = a0;
    if (!a2)
        return 0;
    v3 = a2;
    v4 = &v1;
    while (1)
    {
        v5 = sub_40b6e0(a1, &v1, 0x2000);
        if (!v5)
        {
            return 0xffffffff;
        }
        else if (v5 != 0xffffffffffffffff)
        {
            *(a3) = *(a3) + v5;
            v6 = &(&v1)[v5];
            v7 = v3;
            v8 = v4;
            while (1)
            {
                v3 = v7;
                ptr = memchr(v8, g_415114, v6 - v8);
                if (!ptr)
                    break;
                v8 = ptr + 1;
                v7 = v3 - 1;
                if (v3 == 1)
                {
                    if (v6 <= v8)
                        return 0;
                    *((unsigned int *)&v0) = 0;
                    sub_403da0(v8, v6 - v8);
                    return v0;
                }
            }
        }
        else
        {
            sub_40b410(4, v0);
            v10 = dcgettext(NULL, "error reading %s", 5);
            err = __errno_location();
            error(0, *(err), v10);
            return 1;
        }
    }
}



// Function: dump_remainder @ 0x3f80
unsigned long long dump_remainder(char a0, unsigned long a1, unsigned long a2, unsigned long long a3)
{
    unsigned long long v2;  // r12
    unsigned long long v3;  // rbx
    unsigned long long v4;  // r14
    unsigned long long v5;  // r12
    int *err;  // rax
    char *v8;  // rax
    char v0;  // [bp-0x2048]

    v2 = 0;
    v3 = a3;
    while (1)
    {
        v4 = sub_40b6e0();
        if (v4 == 0xffffffffffffffff)
            break;
        if (!v4)
            return v2;
        if (a0)
            sub_403a10(a1);
        v5 = v2 + v4;
        sub_403da0(&v0, v4);
        if (a3 != 0xffffffffffffffff && (v3 == v4 || !(v3 -= v4, a3 != 0xfffffffffffffffe)))
            return v2 + v4;
        a0 = 0;
        v2 = v5;
    }
    err = __errno_location();
    if (*(err) != 11)
    {
        sub_40b410(4, a1);
        v8 = dcgettext(NULL, "error reading %s", 5);
        error(1, *(err), v8);
    }
    return v2;
}



// Function: start_bytes @ 0x40c0
unsigned int start_bytes(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    unsigned long long v2;  // r14
    unsigned long long v3;  // r14
    unsigned long long v4;  // rax
    char *v5;  // rax
    int *err;  // rax
    char v0;  // [bp-0x2038]

    if (!a2)
        return 0;
    v2 = a2;
    do
    {
        v3 = v2;
        v4 = sub_40b6e0(a1, &v0, 0x2000);
        if (!v4)
            return 0xffffffff;
        if (v4 == 0xffffffffffffffff)
        {
            sub_40b410(4, a0);
            v5 = dcgettext(NULL, "error reading %s", 5);
            err = __errno_location();
            error(0, *(err), v5);
            return 1;
        }
        *(a3) = *(a3) + v4;
        if (v4 > v3)
        {
            sub_403da0(&(&v0)[v3], v4 - v3);
            break;
        }
    } while ((v2 = v3 - v4, v3 != v4));
    return 0;
}



// Function: xlseek @ 0x41c0
long long xlseek(int a0, long long a1, int a2, long long a3)
{
    long long v2;  // rax
    int *err;  // rax
    char *v4;  // rsi
    char *v5;  // rax
    char v0;  // [bp-0x48]

    v2 = lseek(a0, a1, a2);
    if (v2 >= 0)
        return v2;
    sub_409070(a1, &v0);
    err = __errno_location();
    if (a2 != 1)
    {
        sub_40b510(0, 3, a3);
        v4 = "%s: cannot seek to offset %s";
    }
    else
    {
        sub_40b510(0, 3, a3);
        v4 = "%s: cannot seek to relative offset %s";
    }
    v5 = dcgettext(NULL, v4, 5);
    error(0, *(err), v5);
    exit(1); /* do not return */
}



// Function: file_lines @ 0x4290
extern char g_415114;

unsigned int file_lines(long long a0, unsigned int a1, unsigned long long a2, long long a3, unsigned long a4, unsigned long long *a5)
{
    unsigned long long iter;  // r14
    unsigned long v4;  // rbx
    unsigned long long v13;  // rax
    char *v14;  // rax
    int *err;  // rax
    unsigned long long v5;  // rax
    unsigned long long v6;  // rbx
    long long v7;  // rbx
    long long v8;  // r12
    unsigned long v9;  // rbp
    unsigned int v10;  // ebx
    unsigned long v11;  // rdx
    void* v12;  // rax
    unsigned long v0;  // [bp-0x2050]
    char v1;  // [bp-0x2048]

    v0 = a4;
    if (!a2)
        return 1;
    iter = a2;
    v4 = a4 - a3;
    v5 = (long long)(v4) >> 63 >> 0x33;
    v6 = (unsigned int)v4 + (unsigned int)v5 & 0x1fff;
    v7 = v6 - v5;
    if (v6 == v5)
        v7 = 0x2000;
    v8 = a4 - v7;
    sub_4041c0(a1, v8, 0, a0);
    v9 = sub_40b6e0(a1, &v1, v7);
    if (v9 != 0xffffffffffffffff)
    {
        v10 = g_415114;
        *(a5) = v8 + v9;
        if (v9 && *((char *)&v0 + v9 - 7) != g_415114)
            iter -= 1;
        while (1)
        {
            for (v11 = v9; v11 && (v12 = memrchr(&v1, v10, v11), v12); iter -= 1)
            {
                v11 = v12 - &v1;
                if (!iter)
                {
                    sub_403da0(v12 + 1, v9 - 1 - v11);
                    v13 = sub_403f80(0, a0, a1, v0 - v9 - v8);
                    *(a5) = *(a5) + v13;
                    return 1;
                }
            }
            if (v8 == a3)
            {
                sub_4041c0(a1, v8, 0, a0);
                *(a5) = v8 + sub_403f80(0, a0, a1, v0);
                return 1;
            }
            v8 -= 0x2000;
            sub_4041c0(a1, v8, 0, a0);
            v9 = sub_40b6e0(a1, &v1, 0x2000);
            if (v9 == 0xffffffffffffffff)
                break;
            *(a5) = v8 + v9;
            if (!v9)
                return 1;
            v10 = g_415114;
        }
    }
    sub_40b410(4, a0);
    v14 = dcgettext(NULL, "error reading %s", 5);
    err = __errno_location();
    error(0, *(err), v14);
    return 0;
}



// Function: pipe_bytes @ 0x4510
unsigned int pipe_bytes(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    unsigned long long v1;  // r15
    void* v2;  // rax
    void* node;  // rbp
    unsigned long long i;  // rax
    void* v13;  // rdi
    void* iter;  // rbp
    unsigned int v15;  // r12d
    char *v16;  // rax
    int *err;  // rax
    void* ptr;  // rbx
    void* v19;  // rbx
    void* idx;  // r14
    void* iter1;  // rbx
    void* ptr1;  // r13
    void* ptr2;  // rbp
    unsigned long n;  // r12
    unsigned long v8;  // rdi
    unsigned long long v9;  // rax
    unsigned long v10;  // rsi

    v1 = 0;
    v2 = sub_40bfa0(8208);
    *((unsigned long *)&v2[0x2000]) = 0;
    idx = v2;
    *((void* *)&idx[8200]) = NULL;
    iter1 = idx;
    ptr1 = sub_40bfa0(8208);
    ptr2 = ptr1;
    while (1)
    {
        n = sub_40b6e0(a1, ptr2, 0x2000);
        if (n - 1 > 0xfffffffffffffffd)
            break;
        v1 += n;
        *(a3) = *(a3) + n;
        *((unsigned long *)&ptr1[0x2000]) = n;
        v8 = (long long)idx[0x2000];
        *((void* *)&ptr1[8200]) = NULL;
        if (v8 + n > 0x1fff)
        {
            *((void* *)&idx[8200]) = ptr1;
            v9 = v1 - (long long)iter1[0x2000];
            if (v9 > a2)
            {
                ptr2 = iter1;
                idx = ptr1;
                v1 = v9;
                ptr1 = iter1;
                iter1 = (long long)iter1[8200];
            }
            else
            {
                idx = ptr1;
                ptr2 = sub_40bfa0(8208);
                ptr1 = ptr2;
            }
        }
        else
        {
            memcpy(v8 + idx, ptr2, n);
            *((long long *)&idx[0x2000]) = (long long)idx[0x2000] + (long long)ptr1[0x2000];
        }
    }
    free(ptr1);
    if (n != 0xffffffffffffffff)
    {
        v10 = (long long)iter1[0x2000];
        node = iter1;
        i = v1 - (long long)iter1[0x2000];
        if (a2 < i)
        {
            do
            {
                node = (long long)node[8200];
                v1 = i;
                i -= (long long)node[0x2000];
            } while (i > a2);
        }
        v13 = node;
        if (a2 < v1)
        {
            v10 += a2 - v1;
            v13 = node + v1 - a2;
        }
        sub_403da0(v13, v10);
        iter = (long long)node[8200];
        if ((long long)node[8200])
        {
            do
            {
                sub_403da0(iter, (long long)iter[0x2000]);
                iter = (long long)iter[8200];
            } while (iter);
        }
        v15 = 1;
    }
    else
    {
        sub_40b410(4, a0);
        v16 = dcgettext(NULL, "error reading %s", 5);
        err = __errno_location();
        v15 = 0;
        error(0, *(err), v16);
        if (!iter1)
            return 0;
    }
    do
    {
        ptr = iter1;
        v19 = (long long)ptr[8200];
        free(ptr);
        iter1 = v19;
    } while ((long long)ptr[8200]);
    return v15;
}



// Function: pipe_lines @ 0x4760
typedef struct struct_0 {
    char padding_0[8192];
    char field_2000;
    char padding_2001[7];
    unsigned long long field_2008;
    struct struct_0 *field_2010;
} struct_0;

extern char g_415114;

unsigned int pipe_lines(long long a0, unsigned int a1, unsigned long long a2, unsigned long long *a3)
{
    unsigned long long iter;  // r14
    struct_0 *v2;  // rax
    unsigned int v11;  // r15d
    struct_0 *iter1;  // rbx
    unsigned long long i;  // rax
    struct_0 *v14;  // rdi
    unsigned long long v15;  // r14
    unsigned long long j;  // r14
    struct_0 *iter2;  // rbx
    char *v18;  // rax
    int *err;  // rax
    struct_0 *ptr;  // r12
    struct_0 *idx;  // r13
    struct_0 *v21;  // r12
    struct_0 *node;  // r12
    struct_0 *ptr1;  // r15
    unsigned long long v6;  // rbx
    struct_0 *v7;  // rdi
    struct_0 *ptr2;  // rax
    unsigned long long v9;  // rax
    unsigned long long v10;  // rcx

    iter = 0;
    v2 = sub_40bfa0(8216);
    v2->field_2008 = 0;
    idx = v2;
    *((unsigned long long *)&idx->field_2000) = 0;
    node = idx;
    idx->field_2010 = NULL;
    ptr1 = sub_40bfa0(8216);
    while (1)
    {
        v6 = sub_40b6e0(a1, ptr1, 0x2000);
        if (v6 - 1 > 0xfffffffffffffffd)
            break;
        *((unsigned long long *)&ptr1->field_2000) = v6;
        v7 = ptr1;
        *(a3) = *(a3) + v6;
        ptr1->field_2008 = 0;
        ptr1->field_2010 = 0;
        while (1)
        {
            ptr2 = memchr(v7, g_415114, &ptr1->padding_0[v6] - v7);
            if (!ptr2)
                break;
            ptr1->field_2008 = ptr1->field_2008 + 1;
            v7 = &ptr2->padding_0[1];
        }
        iter += ptr1->field_2008;
        if (*((long long *)&ptr1->field_2000) + *((long long *)&idx->field_2000) > 0x1fff)
        {
            idx->field_2010 = ptr1;
            v9 = iter - node->field_2008;
            if (v9 > a2)
            {
                idx = ptr1;
                iter = v9;
                ptr1 = node;
                node = node->field_2010;
            }
            else
            {
                idx = ptr1;
                ptr1 = sub_40bfa0(8216);
            }
        }
        else
        {
            memcpy(&idx->padding_0[*((long long *)&idx->field_2000)], ptr1, *((long long *)&ptr1->field_2000));
            *((long long *)&idx->field_2000) = *((long long *)&idx->field_2000) + *((long long *)&ptr1->field_2000);
            idx->field_2008 = idx->field_2008 + ptr1->field_2008;
        }
    }
    free(ptr1);
    if (v6 != 0xffffffffffffffff)
    {
        v10 = *((long long *)&idx->field_2000);
        v11 = _INSERT(0, 0, !v10 | !a2);
        if (!v10 || !a2)
            goto LABEL_404906;
        if (*(&idx->padding_0[v10] - 1) != g_415114)
        {
            idx->field_2008 = idx->field_2008 + 1;
            iter += 1;
        }
        iter1 = node;
        i = iter - node->field_2008;
        if (a2 < i)
        {
            do
            {
                iter1 = iter1->field_2010;
                iter = i;
                i -= iter1->field_2008;
            } while (i > a2);
        }
        v14 = iter1;
        if (a2 < iter)
        {
            v15 = iter - a2;
            do
            {
                j = v15;
                v14 = rawmemchr(v14, g_415114) + 1;
                v15 = j - 1;
            } while (j != 1);
        }
        sub_403da0(v14, &iter1->padding_0[*((long long *)&iter1->field_2000)] - v14);
        iter2 = iter1->field_2010;
        if (iter1->field_2010)
        {
            do
            {
                sub_403da0(iter2, *((long long *)&iter2->field_2000));
                iter2 = iter2->field_2010;
            } while (iter2);
        }
        v11 = 1;
    }
    else
    {
        v11 = 0;
        sub_40b410(4, a0);
        v18 = dcgettext(NULL, "error reading %s", 5);
        err = __errno_location();
        error(0, *(err), v18);
LABEL_404906:
        if (!node)
            return v11;
    }
    do
    {
        ptr = node;
        v21 = ptr->field_2010;
        free(ptr);
        node = v21;
    } while (ptr->field_2010);
    return v11;
}



// Function: fremote @ 0x4a40
unsigned int fremote(unsigned long a0, long long a1)
{
    int *err;  // rax
    char *v3;  // rax
    unsigned int v4;  // eax
    long long v0;  // [bp-0xa8]

    if (fstatfs())
    {
        err = __errno_location();
        if (*(err) != 38)
        {
            sub_40b410(4, a1);
            v3 = dcgettext(NULL, "cannot determine location of %s. reverting to polling", 5);
            error(0, *(err), v3);
            return 1;
        }
        return 1;
    }
    else
    {
        v4 = sub_402fb0(v0);
        if (v4 <= 0)
        {
            if (v4 >= 0xffffffff)
                return 1;
        }
        else
        {
            if (v4 == 1)
                return 0;
        }
        __assert_fail(); /* do not return */
    }
}



// Function: any_symlinks @ 0x4b40
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

unsigned int any_symlinks(char **a0, unsigned long a1)
{
    char **v2;  // rbx
    stat v0;  // [bp-0xb8]

    v2 = a0;
    while (1)
    {
        if (!lstat(*(v2), &v0) && ((unsigned short)v0.st_nlink & 0xf000) == 0xa000)
            return 1;
        v2 += 12;
        if (v2 == &a0[12 * a1])
            return 0;
    }
}



// Function: die_pipe @ 0x4bd0
void die_pipe(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    raise(13);
    exit(1); /* do not return */
}



// Function: check_output_alive @ 0x4bf0
extern char g_415117;

void check_output_alive(void)
{
    unsigned long v0;  // [bp-0x18]

    if (!g_415117)
        return;
    v0 = 1;
    if (poll(&v0, 1, 0) < 0)
    {
        return;
    }
    else if (!(*((char *)((void*)&v0 + 6)) & 24))
    {
        return;
    }
    else
    {
        sub_404bd0(); /* do not return */
    }
}



// Function: check_fspec @ 0x5090
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[16];
    unsigned int field_30;
    char padding_34[4];
    unsigned int field_38;
    unsigned int field_3c;
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

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_415115;

void check_fspec(void* idx, struct_0 **a1)
{
    long long v2;  // r13
    unsigned int v12;  // edi
    unsigned int v4;  // ebp
    long long v5;  // rax
    long long v6;  // rdx
    unsigned long long v8;  // rax
    char *v9;  // rax
    int *err;  // rax
    unsigned int *err1;  // rax
    stat v0;  // [bp-0xc8]

    if ((int)idx[56] == 0xffffffff)
        return;
    v2 = sub_405050(*((long long *)idx));
    v4 = fstat((int)idx[56], &v0);
    if (v4)
    {
        err1 = __errno_location();
        v12 = (int)idx[56];
        *((unsigned int *)&idx[60]) = *(err1);
        sub_403b30(v12, v2);
        *((unsigned int *)&idx[56]) = 0xffffffff;
        return;
    }
    if (((short)(int)idx[48] & 0xf000) == 0x8000)
    {
        if (v0.st_atime < (long long)idx[8])
        {
            sub_40b510(0, 3, v2);
            error(0, 0, dcgettext(NULL, "%s: file truncated", 5));
            sub_4041c0((int)idx[56], 0, 0, v2);
            *((long long *)&idx[8]) = 0;
        }
        else if (v0.st_atime == (long long)idx[8])
        {
            v5 = sub_40b7b0(&v0);
            if (!sub_40b800((long long)idx[16], (long long)idx[24], v5, v6))
                return;
        }
    }
    if (g_415115)
        v4 = *(a1) != idx;
    v8 = sub_403f80(v4, v2, (int)idx[56], 0xffffffffffffffff);
    *((unsigned long long *)&idx[8]) = (long long)idx[8] + v8;
    if (!v8)
        return;
    *(a1) = idx;
    if (!fflush_unlocked(stdout))
        return;
    v9 = dcgettext(NULL, "write error", 5);
    err = __errno_location();
    error(1, *(err), v9);
}



// Function: recheck @ 0x5270
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned int field_30;
    char field_34;
    char padding_35[3];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
    char padding_44[20];
    unsigned long long field_58;
} struct_0;

extern unsigned int g_415020;
extern char g_41510c;
extern char g_41511a;

void recheck(struct_0 *idx, char a1)
{
    unsigned int v3;  // eax
    unsigned int v4;  // r14d
    char v14;  // al
    char *v18;  // rsi
    long long v22;  // rax
    unsigned int v5;  // r12d
    unsigned long v26;  // r15
    int *err;  // rax
    char v29;  // r13b
    unsigned int v30;  // eax
    char *v33;  // rax
    unsigned long v34;  // rdi
    long long v38;  // rax
    unsigned int v7;  // ebp
    unsigned int v8;  // eax
    unsigned int v10;  // eax
    char v0;  // [bp-0xdd]
    int v1;  // [bp-0xd8]

    v3 = strcmp(idx->padding_0, "-");
    v4 = (unsigned int)idx->padding_3c;
    v5 = v3;
    v0 = idx->padding_35[1];
    v7 = (!v5 ? 0 : (unsigned int)sub_407ea0(idx->padding_0, (a1 ^ 1) * 0x800));
    if (!(char)sub_405260(idx->field_38))
        __assert_fail(); /* do not return */
    if (!g_41511a)
    {
        idx->padding_35[1] = 1;
        if (!g_41510c && (v8 = (unsigned int)(int)(long long)lstat(idx->padding_0, &v1), !v8 && ((unsigned short)*((unsigned int *)((void*)&v1 + 24)) & 0xf000) == 0xa000))
            goto LABEL_405750;
        if (v7 == 0xffffffff || (v10 = (unsigned int)(int)(long long)fstat(v7, &v1), (int)(long long)fstat(v7, &v1) < 0))
            goto LABEL_405321;
        if (!((*((unsigned int *)(&v1 + 24)) & 0xf000) - 0x1000 & 0xffffe000) || ((unsigned short)*((unsigned int *)(&v1 + 24)) & 0xb000) == 0x8000)
        {
            v14 = sub_404a40(v7, sub_405050(idx->padding_0));
            idx->padding_35[0] = v14;
            if (!v14 || g_41510c)
            {
                *((unsigned int *)&idx->padding_3c[0]) = 0;
                if (v4 & 0xfffffffd)
                {
                    if (idx->field_38 == 0xffffffff)
                    {
                        sub_40b410(4, sub_405050(idx->padding_0));
                        v18 = "%s has become accessible";
                        goto LABEL_40546c;
                    }
                    else
                    {
                        __assert_fail(); /* do not return */
                    }
                }
                else
                {
                    if (idx->field_38 != 0xffffffff)
                    {
                        if (idx->field_28 == *((unsigned long long *)(&v1 + 8)) && idx->field_20 == *((unsigned long long *)&v1))
                        {
                            sub_403b30(v7, sub_405050());
                            return;
                        }
                        sub_40b410(4, sub_405050());
                        error(0, 0, dcgettext(NULL, "%s has been replaced;  following new file", 5));
                        v22 = sub_405050(idx->padding_0);
                        sub_403b30(idx->field_38, v22);
                    }
                    else
                    {
                        sub_40b410(4, sub_405050(idx->padding_0));
                        v18 = "%s has appeared;  following new file";
LABEL_40546c:
                        error(0, 0, dcgettext(NULL, v18, 5));
                    }
                    sub_403830(idx, v7, 0, &v1, (!v5 ? 0xffffffff : a1));
                    sub_4041c0(v7, 0, 0, sub_405050(idx->padding_0));
                    return;
                }
            }
            else
            {
                *((unsigned int *)&idx->padding_3c[0]) = 0xffffffff;
                sub_40b410(4, sub_405050(idx->padding_0));
                error(0, 0, dcgettext(NULL, "%s has been replaced with an untailable remote file", 5));
                *((unsigned short *)&idx->field_34) = 0x101;
            }
        }
        else
        {
            *((unsigned int *)&idx->padding_3c[0]) = 0xffffffff;
            idx->padding_35[1] = 0;
            if (g_41511a && g_415020 == 1)
            {
                idx->field_34 = 0;
                if (!v0 && v4 == 0xffffffff)
                    goto LABEL_405348;
            }
            else
            {
                idx->field_34 = 1;
                if (!v0 && v4 == 0xffffffff)
                    goto LABEL_405348;
                dcgettext(NULL, "; giving up on this name", 5);
            }
            sub_40b410(4, sub_405050(idx->padding_0));
            error(0, 0, dcgettext(NULL, "%s has been replaced with an untailable file%s", 5));
        }
    }
    else if (v7 != 0xffffffff)
    {
        idx->padding_35[1] = 1;
LABEL_405750:
        *((unsigned int *)&idx->padding_3c[0]) = 0xffffffff;
        v34 = (unsigned long)idx->padding_0;
        idx->field_34 = 1;
        sub_40b410(4, sub_405050(v34));
        error(0, 0, dcgettext(NULL, "%s has been replaced with an untailable symbolic link", 5));
    }
    else
    {
        idx->padding_35[1] = 0;
        v26 = (unsigned long)idx->padding_0;
        if (g_41510c)
        {
            *((int *)&idx->padding_3c[0]) = *(__errno_location());
LABEL_4054d9:
            if (v0)
            {
                sub_40b410(4, sub_405050(v26));
                v33 = dcgettext(NULL, "%s has become inaccessible", 5);
                error(0, idx->padding_3c, v33);
            }
        }
        else
        {
            if (!lstat(v26, &v1) && ((unsigned short)*((unsigned int *)(&v1 + 24)) & 0xf000) == 0xa000)
                goto LABEL_405750;
LABEL_405321:
            err = __errno_location();
            v29 = idx->padding_35[1];
            v26 = (unsigned long)idx->padding_0;
            v30 = *(err);
            *((unsigned int *)&idx->padding_3c[0]) = v30;
            if (!v29)
                goto LABEL_4054d9;
            if (v30 != v4)
            {
                sub_40b510(0, 3, sub_405050(v26));
                error(0, *(err), "%s");
            }
        }
    }
LABEL_405348:
    sub_403b30(v7, sub_405050(idx->padding_0));
    v38 = sub_405050(idx->padding_0);
    sub_403b30(idx->field_38, v38);
    idx->field_38 = 0xffffffff;
    return;
}



// Function: tail_forever_inotify @ 0x5880
extern char g_415020;

long long tail_forever_inotify(unsigned int a0, void* a1, long long a2, long long *a3)
{
    unsigned int v20;  // ebx
    unsigned long long v21;  // r13
    void* v23;  // r12
    unsigned int v0;  // [bp-0x1a8]
    long long v1;  // [bp-0x1a0]
    char v2;  // [bp-0x198]
    int *v5;  // [bp-0x178]
    unsigned long long v6;  // [bp-0x170]
    long long v7;  // [bp-0x168]
    unsigned int v8;  // [bp-0x160]

    v20 = a0;
    v1 = sub_408850(a2, 0, sub_403980, sub_4039a0, 0);
    if (!v1)
        sub_40c360(); /* do not return */
    v2 = 0;
    *((char *)&v5) = 0;
    *(a3) = v1;
    v7 = a2;
    v0 = *((int *)&g_415020);
    v21 = 0;
    v8 = (*((int *)&g_415020) == 1 ? 3078 : 2);
    v6 = a2 * 96;
    v23 = a1 + v6;
}



// Function: tail_forever @ 0x6220
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

typedef struct FILE {
} FILE;

extern unsigned long long g_415018;
extern unsigned int g_415020;
extern FILE *stdout;
extern void g_415110;
extern char g_415115;

void tail_forever(void* index, unsigned long long a1)
{
    unsigned int v10;  // r13d
    unsigned long long v11;  // xmm0lq
    unsigned int v20;  // eax
    unsigned int v21;  // eax
    int *err;  // rax
    unsigned long long v23;  // rcx
    char *v25;  // r9
    unsigned int v26;  // r11d
    long long v27;  // rax
    long long v28;  // rdx
    unsigned int v29;  // r14d
    int v12;  // 4138
    unsigned long long v30;  // rax
    unsigned long long v32;  // rax
    unsigned long v33;  // rax
    unsigned long v34;  // 4098
    int *err1;  // rax
    void* v36;  // rbx
    char v37;  // al
    long long v38;  // rdi
    char *v39;  // rax
    unsigned int v13;  // eax
    int *err2;  // rax
    char *v42;  // rax
    char *v43;  // rax
    int *v44;  // rax
    void* iter;  // rbx
    unsigned long long node;  // rbp
    int v16;  // r15d
    unsigned long v17;  // rax
    unsigned int v18;  // r14d
    unsigned long v19;  // r12
    int *v0;  // [bp-0x110], Other Possible Types: unsigned long long
    char v1;  // [bp-0x104], Other Possible Types: unsigned int
    unsigned long long v2;  // [bp-0x100]
    char *v3;  // [bp-0xf8], Other Possible Types: unsigned int, unsigned long long
    unsigned int v4;  // [bp-0xe8]
    char v5;  // [bp-0xe2]
    char v6;  // [bp-0xe1]
    unsigned long long v7;  // [bp-0xe0]
    int v8;  // [bp-0xd8], Other Possible Types: stat

    v10 = *((int *)&g_415110);
    v0 = a1;
    v7 = v11;
    if (!*((int *)&g_415110))
    {
        v12 = (int)(g_415020 == 2);
        v5 = (char)v12 & a1 == 1;
        if ((char)v12 & a1 == 1)
        {
            if ((int)index[56] != 0xffffffff)
            {
                v1 = (int)index[48];
                v13 = (int)index[48] & 0xf000;
                v10 = v13 != 0x8000;
                v5 = (v13 == 0x8000 ? 0 : (char)v12 & a1 == 1);
            }
            else
            {
                v5 = 0;
            }
        }
    }
    else
    {
        v5 = 0;
        v10 = 0;
    }
    v6 = 0;
    v2 = v0 - 1;
    while (1)
    {
        v1 = 0;
        iter = index;
        node = 0;
        do
        {
            if ((char)iter[52])
            {
LABEL_40633a:
                node += 1;
                iter += 96;
                if (v0 != node)
                    continue;
                break;
            }
            v16 = (int)iter[56];
            if (v16 >= 0)
            {
                v17 = sub_405050(*((long long *)iter));
                v18 = (int)iter[48];
                v19 = v17;
                v20 = v10;
                if ((int)iter[64] != v10)
                {
                    v21 = sub_407d50(v16, 3);
                    if (!v10)
                    {
                        if (v21 < 0 || v21 != (v21 | 8) && sub_407d50(v16, 4) == 0xffffffff)
                            goto LABEL_4064f8;
                        goto LABEL_406309;
                    }
                    else if (v21 >= 0)
                    {
LABEL_406309:
                        *((unsigned int *)&iter[64]) = v10;
                        v20 = v10;
                    }
                    else
                    {
LABEL_4064f8:
                        err = __errno_location();
                        if (((short)(int)iter[48] & 0xf000) != 0x8000 || *(err) != 1)
                        {
                            v0 = err;
                            sub_40b510(0, 3, v19);
                            v42 = dcgettext(NULL, "%s: cannot change nonblocking mode", 5);
                            error(1, *(v0), v42);
                        }
                        v20 = (int)iter[64];
                    }
                }
                v23 = 0xfffffffffffffffe;
                if (v20)
                {
LABEL_40631f:
                    v32 = sub_403f80(0, v19, v16, v23);
                    *((unsigned long long *)&iter[8]) = (long long)iter[8] + v32;
                    v1 |= v32;
                    goto LABEL_40633a;
                }
                else
                {
                    v3 = &v8;
                    v25 = v3;
                    if (!fstat(v16, &v8))
                    {
                        v26 = v8.st_nlink;
                        if ((int)iter[48] != v26 || ((unsigned short)v26 & 0xf000) == 0x8000 && (long long)iter[8] != v8.st_atime)
                        {
LABEL_406461:
                            v3 = v26;
                            v29 = v18 & 0xf000;
                            v30 = sub_40b7b0(v25);
                            *((unsigned long long *)&iter[88]) = 0;
                            *((unsigned long long *)&iter[16]) = v30;
                            *((long long *)&iter[24]) = v28;
                            *((unsigned int *)&iter[48]) = v3;
                            if (v29 == 0x8000 && v8.st_atime < (long long)iter[8])
                            {
                                v3 = sub_40b510(0, 3, v19);
                                error(0, 0, dcgettext(NULL, "%s: file truncated", 5));
                                sub_4041c0(v16, 0, 0, v19);
                                *((unsigned long long *)&iter[8]) = 0;
                            }
                            if (v2 != node && g_415115)
                                sub_403a10(v19);
                            v2 = node;
                            if (!(int)iter[64])
                            {
                                v23 = 0xffffffffffffffff;
                                if (v29 == 0x8000 && (char)iter[53])
                                {
                                    v23 = v8.st_atime - (long long)iter[8];
                                    goto LABEL_40631f;
                                }
                            }
                            else
                            {
                                v23 = 0xfffffffffffffffe;
                                goto LABEL_40631f;
                            }
                        }
                        else
                        {
                            v4 = v26;
                            v3 = v25;
                            v27 = sub_40b7b0(v25);
                            v25 = v3;
                            v26 = v4;
                            if (sub_40b800((long long)iter[16], (long long)iter[24], v27, v28))
                                goto LABEL_406461;
                            v33 = (long long)iter[88];
                            v34 = g_415018;
                            *((unsigned long long *)&iter[88]) = v33 + 1;
                            if (v33 >= v34 && g_415020 == 1)
                            {
                                sub_405270(iter, (int)iter[64]);
                                *((unsigned long long *)&iter[88]) = 0;
                                goto LABEL_40633a;
                            }
                        }
                    }
                    else
                    {
                        *((unsigned int *)&iter[56]) = 0xffffffff;
                        err1 = __errno_location();
                        *((int *)&iter[60]) = *(err1);
                        sub_40b510(0, 3, v19);
                        error(0, *(err1), "%s");
                        close(v16);
                        goto LABEL_40633a;
                    }
                }
            }
        } while ((node += 1, v36 = iter + 96, sub_405270(iter, (char)v10), iter += 96, v0 != node));
        if (!(char)sub_403880(index, v0))
        {
            error(0, 0, dcgettext(NULL, "no files remaining", 5));
            return;
        }
        v37 = v1 ^ 1;
        if (!(v37 || v5))
        {
            sub_404bf0();
        }
        else if (!fflush_unlocked(stdout))
        {
            sub_404bf0();
            if (!v1)
            {
                if (v6)
                {
                    return;
                }
                else if (!(!*((int *)&g_415110) || !kill(*((int *)&g_415110), 0) || *(__errno_location()) == 1))
                {
                    v6 = v37 | v5;
                }
                else if (sub_40c4e0(v38))
                {
                    v39 = dcgettext(NULL, "cannot read realtime clock", 5);
                    err2 = __errno_location();
                    error(1, *(err2), v39);
                }
            }
        }
        else
        {
            v43 = dcgettext(NULL, "write error", 5);
            v44 = __errno_location();
            error(1, *(v44), v43);
        }
    }
}



// Function: tail @ 0x6a90
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

extern char g_41510d;
extern char g_415116;
extern char g_415119;

long long tail(long long a0, int a1, long long a2, unsigned long long *a3)
{
    unsigned long v7;  // 4110
    unsigned long long v8;  // r14
    unsigned long long v9;  // r13
    unsigned long long v10;  // r12
    unsigned long long v11;  // rbx
    unsigned long v12;  // fs
    unsigned long v14;  // rbx
    unsigned long v15;  // rax
    unsigned long v16;  // rdx
    stat v0;  // [bp-0xc8]
    unsigned long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    unsigned long long v3;  // [bp-0x18]
    unsigned long long v4;  // [bp-0x10]
    unsigned long long v5;  // [bp-0x8]

    v7 = g_415119;
    *(a3) = 0;
    if (!(char)v7)
    {
        v5 = v8;
        v4 = v9;
        v3 = v10;
        v2 = v11;
        v1 = *((long long *)(40 + v12));
        if (fstat(a1, &v0))
            goto LABEL_0x4069c0;
        if (g_415116)
        {
            if (g_41510d)
                goto LABEL_0x406990;
            if (a2 < 0)
                goto LABEL_0x406990;
            if ((v0.st_nlink & 0xf000) == 0x8000)
                goto LABEL_0x406a20;
            else
                goto LABEL_0x40689a;
        }
        else
        {
            if (g_41510d)
                goto LABEL_0x406a08;
            if (a2 < 0)
                goto LABEL_0x406a08;
            if ((char)sub_406810(v0.st_nlink))
            {
                v14 = v0.st_atime;
                v15 = 0xffffffffffffffff;
            }
            else
            {
                v15 = lseek(a1, -(a2), 2);
                if (v15 == 0xffffffffffffffff)
                    goto LABEL_0x406a08;
                v14 = v15 + a2;
            }
            v16 = v0.st_mtime;
            if (v0.st_mtime - 1 > 0x1fffffffffffffff)
                v16 = 0x200;
            if (v14 <= v16)
                goto LABEL_0x406a08;
            if (v15 == 0xffffffffffffffff)
                v15 = sub_4041c0(a1, 0, 1, a0);
            if (v14 > v15)
                if (a2 < v14 - v15)
                    goto LABEL_0x406a48;
                else
                    goto LABEL_406982;
LABEL_406982:
        }
    }
}



// Function: tail_file @ 0x6ab0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    unsigned long long field_28;
    unsigned int field_30;
    char field_34;
    char padding_35[3];
    unsigned int field_38;
    char padding_3c[4];
    unsigned int field_40;
    char padding_44[20];
    unsigned long long field_58;
} struct_0;

extern char g_41510e;
extern char g_415115;
extern char g_415118;
extern char g_41511a;

unsigned int tail_file(struct_0 *idx, long long a1)
{
    unsigned long v3;  // r12
    unsigned int v4;  // ebp
    char v13;  // al
    int *err;  // rax
    unsigned long v17;  // rdi
    long long v18;  // rax
    char *v19;  // rax
    unsigned long v20;  // rdi
    long long v21;  // rax
    int *err1;  // rax
    unsigned int v5;  // r14d
    int v23;  // eax
    char *v25;  // rax
    unsigned int v7;  // r12d
    char *v9;  // rax
    int *err2;  // rax
    unsigned long long v0;  // [bp-0xe0]
    int v1;  // [bp-0xd8]

    v3 = (unsigned long)idx->padding_0;
    v4 = strcmp(v3, "-");
    if (!v4)
    {
        g_41510e = 1;
        v5 = 0;
        goto LABEL_406b00;
    }
    else
    {
        v5 = sub_407ea0(v3, 0);
        if (!g_41511a)
        {
            idx->padding_35[1] = 1;
            v3 = (unsigned long)idx->padding_0;
            if (v5 == 0xffffffff)
                goto LABEL_406c50;
        }
        else
        {
            v3 = (unsigned long)idx->padding_0;
            if (v5 == 0xffffffff)
            {
                idx->padding_35[1] = 0;
LABEL_406c50:
                err1 = __errno_location();
                if (g_415118)
                {
                    idx->field_38 = 0xffffffff;
                    v23 = *(err1);
                    idx->field_34 = g_41511a ^ 1;
                    *((int *)&idx->padding_3c[0]) = v23;
                    idx->field_28 = 0;
                    idx->field_20 = 0;
                }
                sub_40b410(4, sub_405050(idx->padding_0));
                v25 = dcgettext(NULL, "cannot open %s for reading", 5);
                error(0, *(err1), v25);
                return 0;
            }
LABEL_406b00:
            idx->padding_35[1] = 1;
        }
        if (g_415115)
        {
            sub_403a10(sub_405050(v3));
            v3 = (unsigned long)idx->padding_0;
        }
        v7 = sub_406a90(sub_405050(v3), v5, a1, &v0);
        if (g_415118)
        {
            *((unsigned int *)&idx->padding_3c[0]) = (char)v7 - 1;
            if (fstat(v5, &v1) >= 0)
            {
                if (!((*((unsigned int *)(&v1 + 24)) & 0xf000) - 0x1000 & 0xffffe000) || ((unsigned short)*((unsigned int *)(&v1 + 24)) & 0xb000) == 0x8000)
                {
                    if ((char)v7)
                    {
                        sub_403830(idx, v5, v0, &v1, -(v4 < 1) | 1);
                        idx->padding_35[0] = sub_404a40(v5, sub_405050(idx->padding_0));
                        return v7;
                    }
                }
                else
                {
                    *((int *)&idx->padding_3c[0]) = -0x1;
                    idx->padding_35[1] = 0;
                    v13 = g_41511a ^ 1;
                    idx->field_34 = v13;
                    if (v13)
                        dcgettext(NULL, "; giving up on this name", 5);
                    sub_40b510(0, 3, sub_405050(idx->padding_0));
                    error(0, 0, dcgettext(NULL, "%s: cannot follow end of this type of file%s", 5));
                }
            }
            else
            {
                err = __errno_location();
                v17 = (unsigned long)idx->padding_0;
                *((int *)&idx->padding_3c[0]) = *(err);
                v18 = sub_405050(v17);
                sub_40b410(4, v18);
                v19 = dcgettext(NULL, "error reading %s", 5);
                error(0, *(err), v19);
            }
            v20 = (unsigned long)idx->padding_0;
            idx->field_34 = g_41511a ^ 1;
            v21 = sub_405050(v20);
            sub_403b30(v5, v21);
            idx->field_38 = 0xffffffff;
            return 0;
        }
        else if (!v4)
        {
            return v7;
        }
        else if (!close(v5))
        {
            return v7;
        }
        else
        {
            sub_40b410(4, sub_405050(idx->padding_0));
            v9 = dcgettext(NULL, "error reading %s", 5);
            err2 = __errno_location();
            error(0, *(err2), v9);
            return 0;
        }
    }
}



// Function: usage @ 0x6e60
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned long long stderr;

void usage(int a0)
{
    unsigned long long v2;  // r12
    char *v3;  // rax
    char *v12;  // rax
    char *v13;  // rax
    char *v15;  // rax
    char *v16;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    char *v21;  // rax
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [FILE]...\n", 5));
        __printf_chk(1, dcgettext(NULL, "Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n", 5));
        sub_4039b0(v6, v7, v8, v9, v10, v11);
        sub_4039e0(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means 'descriptor'\n  -F                       same as --follow=name --retry\n", 5);
        fputs_unlocked(v13, stdout);
        __printf_chk(1, dcgettext(NULL, "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n", 5));
        v15 = dcgettext(NULL, "      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n", 5);
        fputs_unlocked(v15, stdout);
        v16 = dcgettext(NULL, "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n", 5);
        fputs_unlocked(v16, stdout);
        v17 = dcgettext(NULL, "  -z, --zero-terminated    line delimiter is NUL, not newline\n", 5);
        fputs_unlocked(v17, stdout);
        v18 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v18, stdout);
        v19 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v19, stdout);
        v20 = dcgettext(NULL, "\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n", 5);
        fputs_unlocked(v20, stdout);
        v21 = dcgettext(NULL, "With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n", 5);
        fputs_unlocked(v21, stdout);
        sub_404e20(v6, v7, v8, v9, v10, v11);
    }
    exit(a0); /* do not return */
}



// Function: is_prime @ 0x7f20
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



// Function: next_prime @ 0x7f90
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
    while (!sub_407f20(v2, v3))
    {
        v2 = v4 + 2;
        if (v2 == 0xffffffffffffffff)
            return v4;
    }
    return v4;
}



// Function: raw_hasher @ 0x7fd0
long long raw_hasher(unsigned long a0, unsigned long long a1)
{
    return sub_40caf0() % a1;
}



// Function: raw_comparator @ 0x7ff0
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0x8000
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

extern unsigned int g_40f5d0[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_40f5d0[0])
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
    idx->field_28 = &g_40f5d0[0];
    return 0;
}



// Function: free_entry @ 0x8090
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



// Function: safe_hasher @ 0x80b0
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



// Function: hash_find_entry @ 0x80e0
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

    v2 = sub_4080b0(a0, a1);
    *(a2) = v2;
    v3 = *((long long *)v2);
    if (!v3)
    {
LABEL_408181:
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
                    sub_408090(a0, v6);
                    return v0;
                }
                return *((long long *)v6);
            }
        }
        goto LABEL_408181;
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
    sub_408090(a0, v7);
    return v0;
}



// Function: allocate_entry @ 0x81e0
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



// Function: transfer_entries @ 0x8210
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
LABEL_408238:
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
                    idx2 = sub_4080b0(idx, *((long long *)&v3->field_0));
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
                        sub_408090(idx, v5);
                        v3 = v6;
                        if (!v3)
                            break;
                    }
                }
                v2 = iter->field_0;
            }
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_408238;
            index = sub_4080b0(idx, v2);
            if (index->field_0)
            {
                v8 = sub_4081e0(idx);
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



// Function: hash_get_n_buckets @ 0x83d0
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0x83e0
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0x83f0
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0x8400
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



// Function: hash_print_statistics @ 0x84d0
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
    sub_4084dd();
    return;
}



// Function: hash_lookup @ 0x85f0
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

    iter = sub_4080b0(a0, a1);
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



// Function: hash_get_next @ 0x86a0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_4086ac();
    return;
}



// Function: hash_do_for_each @ 0x8760
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



// Function: hash_initialize @ 0x8850
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

extern struct_0 g_40f5d0;

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

    v1 = sub_407fd0;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_407ff0;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_40f5d0.padding_0[0];
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_408000(ptr))
    {
        v6 = sub_408340(a0, v2->field_10);
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



// Function: hash_clear @ 0x8960
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



// Function: hash_free @ 0x8a10
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
            goto LABEL_408abc;
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
LABEL_408abc:
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



// Function: hash_rehash @ 0x8b00
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

    v11 = sub_408340(a1, idx->field_28->field_10);
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
        v13 = sub_408210(&ptr, idx, 0);
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
        if ((char)sub_408210(idx, &ptr, 1) && (char)sub_408210(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0x8c80
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
    v2 = sub_4080e0(idx, a1, &index, 0);
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
            sub_408000(idx);
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
                else if (!(char)sub_408b00(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else if (sub_4080e0(idx, a1, &index, 0))
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
        v11 = sub_4081e0(idx);
        if (!v11)
            return 0xffffffff;
        v11->field_0 = a1;
        v11->field_8 = index->field_8;
        index->field_8 = v11;
        *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
        return 1;
    }
}



// Function: hash_insert @ 0x8e70
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x18]

    v2 = sub_408c80(a0, a1, &v0);
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



// Function: hash_remove @ 0x8ed0
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

    v2 = sub_4080e0(idx, a1, &v0, 1);
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
            goto LABEL_409016;
LABEL_408f60:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = idx[2].field_0;
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_408f60;
LABEL_409016:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_408000(idx);
    v9 = idx[5].field_0;
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_408b00(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



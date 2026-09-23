// Function: main @ 0x2960
typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_1 {
    long long field_0;
    struct struct_0 *field_8;
    struct struct_2 *field_10;
} struct_1;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern char g_40ea41;
extern char g_41310e;
extern char g_413114;
extern char g_413115;
extern char g_413116;
extern char g_413118;
extern char g_413119;

int main(unsigned int a0, struct_1 *idx)
{
    unsigned long long v10;  // r15
    unsigned long long v11;  // r14
    char v20;  // r12b
    unsigned long v22;  // rdx
    char v23;  // cl
    unsigned long v25;  // rax
    char v26;  // r14b
    unsigned int v27;  // ecx
    char v28;  // r13b
    int *err;  // rax
    unsigned long long v12;  // r13
    unsigned long long v13;  // r12
    unsigned long long v14;  // rbx
    unsigned long v15;  // fs
    char *v16;  // rdi
    unsigned int v17;  // eax
    struct_0 *v18;  // rdx
    unsigned long v19;  // rdi
    unsigned int v0;  // [bp-0x260]
    unsigned long long v1;  // [bp-0x208]
    unsigned long v2;  // [bp-0x1d8], Other Possible Types: unsigned long long
    unsigned long v3;  // [bp-0x40]
    unsigned long long v4;  // [bp-0x30]
    unsigned long long v5;  // [bp-0x20]
    unsigned long long v6;  // [bp-0x18]
    unsigned long long v7;  // [bp-0x10]
    unsigned long long v8;  // [bp-0x8]

    v8 = v10;
    v7 = v11;
    v6 = v12;
    v5 = v13;
    v4 = v14;
    v3 = *((long long *)(40 + v15));
    v2 = 10;
    sub_409150(idx->field_0);
    setlocale(6, &g_40ea41);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_40cf70(sub_4079b0);
    g_41310e = 0;
    g_413119 = 1;
    g_413115 = 0;
    g_413116 = 0;
    g_413118 = 0;
    g_413114 = 10;
    if (a0 != 2)
    {
        if (a0 != 3)
        {
            if (a0 != 4)
            {
                v0 = 0;
                v1 = 0x3ff0000000000000;
            }
            v16 = &idx->field_10->field_0;
LABEL_402a20:
            if (strcmp(v16, "--"))
            {
                v0 = 0;
                v1 = 0x3ff0000000000000;
            }
        }
        else
        {
            v16 = &idx->field_10->field_0;
            if (*(v16) == 45 && v16[1])
                goto LABEL_402a20;
        }
    }
    v17 = sub_4090c0();
    v18 = idx->field_8;
    v19 = &v18->field_1;
    if (v18->field_0 == 43)
    {
        v20 = 1;
        if (v17 - 200112 > 696)
            goto LABEL_4038a5;
    }
    else if (v18->field_0 == 45 && (v17 <= 200111 || (&v18->field_1)[v18->field_1 == 99]))
    {
        v20 = 0;
LABEL_4038a5:
        v22 = v19;
        if (v18->field_1 - 48 <= 9)
        {
            do
            {
                v22 += 1;
            } while (*((char *)(v22 + 1)) - 48 <= 9);
        }
        switch (v23)
        {
        case 99:
            v26 = 0;
            v27 = 10;
            break;
        case 108:
            v26 = 1;
            v27 = 10;
            break;
        case 98:
            v26 = 0;
            v27 = 0x1400;
            break;
        default:
            v25 = v22;
            v26 = 1;
            v27 = 10;
            goto LABEL_4038fb;
        }
        v25 = v22 + 1;
LABEL_4038fb:
        v28 = 0;
        if (*((char *)v25) == 0x66)
        {
            v25 += 1;
            v28 = 1;
        }
        if (!*((char *)v25))
        {
            if (v19 == v22)
            {
                v2 = v27;
            }
            else if (sub_40c520() & 0xfffffffd)
            {
                sub_40b520(idx->field_8);
                dcgettext(NULL, "invalid number", 5);
                err = __errno_location();
                error(1, *(err), "%s: %s");
            }
            g_413116 = v20;
            g_413119 = v26;
            g_413118 = v28;
            v0 = 0;
            v1 = 0x3ff0000000000000;
        }
    }
    v0 = 0;
    v1 = 0x3ff0000000000000;
}



// Function: wd_hasher @ 0x5430
void wd_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: wd_comparator @ 0x5450
void wd_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: fremote @ 0x5460
unsigned int fremote(unsigned long a0, long long a1)
{
    int *err;  // rax
    char *v3;  // rax
    unsigned long v12;  // r12
    unsigned long v13;  // r12
    unsigned long v14;  // r12
    unsigned long v15;  // r12
    unsigned long v16;  // r12
    unsigned long v17;  // r12
    unsigned long v18;  // r12
    unsigned long v19;  // r12
    unsigned long v20;  // r12
    unsigned long v21;  // r12
    unsigned long v4;  // r12
    unsigned long v22;  // r12
    unsigned long v23;  // r12
    unsigned long v24;  // r12
    unsigned long v25;  // r12
    unsigned long v26;  // r12
    unsigned long v27;  // r12
    unsigned long v28;  // r12
    unsigned long v29;  // r12
    unsigned long v30;  // r12
    unsigned long v31;  // r12
    unsigned long v5;  // r12
    unsigned long v32;  // r12
    unsigned long v33;  // r12
    unsigned long v34;  // r12
    unsigned long v6;  // r12
    unsigned long v7;  // r12
    unsigned long v8;  // r12
    unsigned long v9;  // r12
    unsigned long v10;  // r12
    unsigned long v11;  // r12
    long long v0;  // [bp-0xa8]

    if (fstatfs())
    {
        err = __errno_location();
        if (*(err) != 38)
        {
            sub_40ae30(4, a1);
            v3 = dcgettext(NULL, "cannot determine location of %s. reverting to polling", 5);
            error(0, *(err), v3);
            return 1;
        }
        return 1;
    }
    else
    {
        if (v0 == 732765674)
            return 0;
        if (v0 <= 732765674)
        {
            if (v0 <= 40866)
            {
                if (v0 <= 0x9f9f)
                {
                    if (v0 <= 16390)
                    {
                        if (v0 > 0x3fff)
                        {
                            return (unsigned int)(81 >> ((char)v0 & 63)) & 1 ^ 1;
                        }
                        else if (v0 <= 5007)
                        {
                            if (v0 > 4978)
                            {
                                return (unsigned int)(268440577 >> ((char)(v0 - 4979) & 63)) & 1 ^ 1;
                            }
                            else if (v0 != 391)
                            {
                                return (v0 <= 391 ? _INSERT(v4, 0, v0 - 47 & 0xfffffffffffffffb) : _INSERT(v4, 0, v0 != 1984));
                            }
                        }
                        else
                        {
                            if (v0 != 9336)
                            {
                                if (v0 > 9336)
                                {
                                    return _INSERT(v5, 0, v0 != 0x3434);
                                }
                                else if (v0 != 7377)
                                {
                                    return _INSERT(v6, 0, v0 != 9320);
                                }
                            }
                        }
                    }
                    else
                    {
                        if (v0 != 19802)
                        {
                            if (v0 > 19802)
                            {
                                if (v0 != 29366)
                                {
                                    if (v0 > 29366)
                                    {
                                        return _INSERT(v7, 0, v0 != 38496);
                                    }
                                    else if (v0 != 24053)
                                    {
                                        return _INSERT(v8, 0, v0 != 29301);
                                    }
                                }
                            }
                            else
                            {
                                if (v0 != 18520)
                                {
                                    if (v0 > 18520)
                                    {
                                        return _INSERT(v9, 0, v0 != 19780);
                                    }
                                    else if (v0 != 16964)
                                    {
                                        return _INSERT(v10, 0, v0 != 18475);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            else
            {
                if (v0 <= 0x12ff7b7)
                {
                    if (v0 <= 0x12ff7b3 && v0 != 2613483)
                    {
                        if (v0 > 2613483)
                        {
                            if (v0 != 16914836)
                            {
                                if (v0 > 16914836)
                                {
                                    if (v0 != 16914839)
                                        return _INSERT(v11, 0, v0 != 19911021);
                                }
                                else
                                {
                                    if (v0 != 4278867)
                                        return _INSERT(v12, 0, v0 != 0xc0ffee);
                                }
                            }
                        }
                        else
                        {
                            if (v0 != 61267)
                            {
                                if (v0 > 61267)
                                {
                                    if (v0 != 61791)
                                        return _INSERT(v13, 0, v0 != 72020);
                                }
                                else
                                {
                                    if (v0 != 0xadff && v0 != 61265)
                                        return _INSERT(v14, 0, v0 != 44533);
                                }
                            }
                        }
                    }
                }
                else
                {
                    if (v0 != 352400198)
                    {
                        if (v0 > 352400198)
                        {
                            if (v0 != 538032816)
                            {
                                if (v0 > 538032816)
                                {
                                    if (v0 != 604313861)
                                        return _INSERT(v15, 0, v0 != 684539205);
                                }
                                else
                                {
                                    if (v0 != 427819522)
                                        return _INSERT(v16, 0, v0 != 464386766);
                                }
                            }
                        }
                        else
                        {
                            if (v0 != 195894762)
                            {
                                if (v0 > 195894762)
                                {
                                    if (v0 != 288389204)
                                        return _INSERT(v17, 0, v0 != 325456742);
                                }
                                else
                                {
                                    if (v0 != 124082209)
                                        return _INSERT(v18, 0, v0 != 151263540);
                                }
                            }
                        }
                    }
                }
            }
        }
        else
        {
            if (v0 != 1650812272)
            {
                if (v0 > 1650812272)
                {
                    if (v0 != 2240043254)
                    {
                        if (v0 > 2240043254)
                        {
                            if (v0 != 3405662737)
                            {
                                if (v0 > 3405662737)
                                {
                                    if (v0 != 4076150800)
                                    {
                                        if (v0 > 4076150800)
                                        {
                                            if (v0 != 0xf97cff8c)
                                                return _INSERT(v19, 0, v0 != 4187351113);
                                        }
                                        else
                                        {
                                            if (v0 != 3730735588)
                                                return _INSERT(v20, 0, v0 != 3774210530);
                                        }
                                    }
                                }
                                else
                                {
                                    if (v0 != 2881100148)
                                    {
                                        if (v0 > 2881100148)
                                        {
                                            if (v0 != 3344373136)
                                                return _INSERT(v21, 0, v0 != 3380511080);
                                        }
                                        else
                                        {
                                            if (v0 != 2435016766)
                                                return _INSERT(v22, 0, v0 != 2508478710);
                                        }
                                    }
                                }
                            }
                        }
                        else
                        {
                            if (v0 != 1819242352)
                            {
                                if (v0 > 1819242352)
                                {
                                    if (v0 != 1936814952)
                                    {
                                        if (v0 > 1936814952)
                                        {
                                            if (v0 != 1936880249)
                                                return _INSERT(v23, 0, v0 != 1953653091);
                                        }
                                        else
                                        {
                                            if (v0 != 1853056627)
                                                return _INSERT(v24, 0, v0 != 1935894131);
                                        }
                                    }
                                }
                                else
                                {
                                    if (v0 != 1684300152)
                                    {
                                        if (v0 > 1684300152)
                                        {
                                            if (v0 != 1733912937)
                                                return _INSERT(v25, 0, v0 != 1746473250);
                                        }
                                        else
                                        {
                                            if (v0 != 1667723888 && v0 != 1684170528)
                                                return _INSERT(v26, 0, v0 != 1650812274);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else
                {
                    if (v0 != 1397114950)
                    {
                        if (v0 > 1397114950)
                        {
                            if (v0 != 1481003842)
                            {
                                if (v0 > 1481003842)
                                {
                                    if (v0 != 1573531125)
                                    {
                                        if (v0 > 1573531125)
                                        {
                                            if (v0 != 1634035564)
                                                return _INSERT(v27, 0, v0 != 1650746742);
                                        }
                                        else
                                        {
                                            if (v0 != 1513908720)
                                                return _INSERT(v28, 0, v0 != 1515144787);
                                        }
                                    }
                                }
                                else
                                {
                                    if (v0 != 0x54190100)
                                    {
                                        if (v0 > 0x54190100)
                                        {
                                            if (v0 != 1448756819)
                                                return _INSERT(v29, 0, v0 != 1479104553);
                                        }
                                        else
                                        {
                                            if (v0 != 1397118030)
                                                return _INSERT(v30, 0, v0 != 1397703499);
                                        }
                                    }
                                }
                            }
                        }
                        else
                        {
                            if (v0 != 1161678120)
                            {
                                if (v0 > 1161678120)
                                {
                                    if (v0 != 1382369651)
                                    {
                                        if (v0 > 1382369651)
                                        {
                                            if (v0 != 1397048141)
                                                return _INSERT(v31, 0, v0 != 1397109069);
                                        }
                                        else
                                        {
                                            if (v0 != 1162691661)
                                                return _INSERT(v32, 0, v0 != 1163413075);
                                        }
                                    }
                                }
                                else
                                {
                                    if (v0 != 1112100429)
                                    {
                                        if (v0 > 1112100429)
                                        {
                                            if (v0 != 1128357203)
                                                return _INSERT(v33, 0, v0 != 1145913666);
                                        }
                                        else
                                        {
                                            if (v0 != 827541066 && v0 != 1111905073)
                                                return _INSERT(v34, 0, v0 != 801189825);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return 0;
    }
}



// Function: start_lines @ 0x5ce0
extern char g_413114;

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
        v5 = sub_40b540(a1, &v1, 0x2000);
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
                ptr = memchr(v8, g_413114, v6 - v8);
                if (!ptr)
                    break;
                v8 = ptr + 1;
                v7 = v3 - 1;
                if (v3 == 1)
                {
                    if (v6 <= v8)
                    {
                        return 0;
                    }
                    else if (v6 == v8)
                    {
                        return 0;
                    }
                    else
                    {
                        *((unsigned int *)&v0) = 0;
                        sub_405c60();
                        return v0;
                    }
                }
            }
        }
        else
        {
            sub_40ae30(4, v0);
            v10 = dcgettext(NULL, "error reading %s", 5);
            err = __errno_location();
            error(0, *(err), v10);
            return 1;
        }
    }
}



// Function: start_bytes @ 0x5e30
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
        v4 = sub_40b540(a1, &v0, 0x2000);
        if (!v4)
            return 0xffffffff;
        if (v4 == 0xffffffffffffffff)
        {
            sub_40ae30(4, a0);
            v5 = dcgettext(NULL, "error reading %s", 5);
            err = __errno_location();
            error(0, *(err), v5);
            return 1;
        }
        *(a3) = *(a3) + v4;
        if (v4 > v3 && v4 != v3)
        {
            sub_405c60(&(&v0)[v3]);
            break;
        }
    } while ((v2 = v3 - v4, v3 != v4));
    return 0;
}



// Function: dump_remainder @ 0x6020
extern char g_40ea41;
extern char g_413010;

unsigned long long dump_remainder(char a0, unsigned long a1, unsigned long a2, long long a3)
{
    unsigned long long v2;  // r12
    char v3;  // bpl
    long long v4;  // rbx
    long long v5;  // r14
    char *v6;  // rdx
    unsigned long long v7;  // r12
    int *err;  // rax
    char *v10;  // rax
    char v0;  // [bp-0x2048]

    v2 = 0;
    v3 = a0;
    v4 = a3;
    while (1)
    {
        v5 = sub_40b540();
        if (v5 == -0x1)
            break;
        if (!v5)
            return v2;
        if (v3)
        {
            v6 = "\n";
            if (g_413010)
                v6 = &g_40ea41;
            __printf_chk(1, "%s==> %s <==\n", v6, a1);
            g_413010 = 0;
        }
        v7 = v2 + v5;
        sub_405c60(&v0, v5);
        if (a3 != -0x1 && (v4 == v5 || !(v4 -= v5, a3 != -0x2)))
            return v2 + v5;
        v3 = 0;
        v2 = v7;
    }
    err = __errno_location();
    if (*(err) != 11)
    {
        sub_40ae30(4, a1);
        v10 = dcgettext(NULL, "error reading %s", 5);
        error(1, *(err), v10);
    }
    return v2;
}



// Function: file_lines @ 0x6190
extern char g_413114;

unsigned long long file_lines(long long a0, unsigned int a1, unsigned long long a2, long long a3, unsigned long a4, unsigned long long *a5)
{
    unsigned long long iter;  // r14
    long long v4;  // rbx
    unsigned long long v13;  // rax
    char *v14;  // rax
    int *err;  // rax
    unsigned long v5;  // rax
    unsigned long long v6;  // rdx
    unsigned long long v7;  // rax
    long long v8;  // r15
    unsigned long v9;  // rbp
    unsigned int v10;  // ebx
    unsigned long v11;  // rdx
    unsigned long v12;  // rax
    unsigned long v0;  // [bp-0x2050]
    char v1;  // [bp-0x2048]

    v0 = a4;
    if (!a2)
        return 1;
    iter = a2;
    v4 = 0x2000;
    v5 = a4 - a3;
    v6 = (long long)(v5) >> 63 >> 0x33;
    v7 = (unsigned int)v5 + (unsigned int)v6 & 0x1fff;
    if (v7 != v6)
        v4 = v7 - v6;
    v8 = a4 - v4;
    if (lseek(a1, v8, 0) < 0)
        sub_405f40(v8, 0, a0); /* do not return */
    v9 = sub_40b540(a1, &v1, v4);
    if (v9 != 0xffffffffffffffff)
    {
        v10 = g_413114;
        *(a5) = v8 + v9;
        if (v9 && *((char *)&v0 + v9 - 7) != g_413114)
            iter -= 1;
        while (1)
        {
            for (v11 = v9; v11 && (v12 = (unsigned long)memrchr(&v1, v10, v11), v12); iter -= 1)
            {
                v11 = v12 - &v1;
                if (!iter)
                {
                    if (v9 - 1 != v11)
                        sub_405c60(v12 + 1);
                    v13 = sub_406020(0, a0, a1, v0 - v9 - v8);
                    *(a5) = *(a5) + v13;
                    return 1;
                }
            }
            if (v8 == a3)
            {
                if (lseek(a1, v8, 0) < 0)
                    sub_405f40(v8, 0, a0); /* do not return */
                *(a5) = sub_406020(0, a0, a1, v0) + v8;
                return 1;
            }
            v8 -= 0x2000;
            if (lseek(a1, v8, 0) < 0)
                sub_405f40(v8, 0, a0); /* do not return */
            v9 = sub_40b540(a1, &v1, 0x2000);
            if (v9 == 0xffffffffffffffff)
                break;
            *(a5) = v8 + v9;
            if (!v9)
                return 1;
            v10 = g_413114;
        }
    }
    sub_40ae30(4, a0);
    v14 = dcgettext(NULL, "error reading %s", 5);
    err = __errno_location();
    error(0, *(err), v14);
    return 0;
}



// Function: check_fspec @ 0x6430
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
extern char g_413115;

long long check_fspec(void* idx, struct_0 **a1)
{
    unsigned int v5;  // r12d
    unsigned long v6;  // r13
    unsigned long v15;  // fs
    unsigned int v8;  // r12d
    unsigned long long v10;  // rax
    char *v11;  // rax
    int *err;  // rax
    unsigned int *err1;  // rax
    unsigned long v14;  // rdi
    stat v0;  // [bp-0xc8]
    long long v1;  // [bp-0x70]
    long long v2;  // [bp-0x68]
    unsigned long v3;  // [bp-0x30]

    v5 = (int)idx[56];
    if ((int)idx[56] == 0xffffffff)
        return v3 - *((long long *)(40 + v15));
    v6 = *((long long *)idx);
    if (!strcmp(*((long long *)idx), "-"))
    {
        v6 = dcgettext(NULL, "standard input", 5);
        v5 = (int)idx[56];
    }
    v8 = fstat(v5, &v0);
    if (v8)
    {
        err1 = __errno_location();
        v14 = (int)idx[56];
        *((unsigned int *)&idx[60]) = *(err1);
        sub_405bd0(v14, v6);
        *((unsigned int *)&idx[56]) = 0xffffffff;
    }
    else if (((short)(int)idx[48] & 0xf000) != 0x8000)
    {
LABEL_4064a1:
        if (g_413115)
            v8 = *(a1) != idx;
        v10 = sub_406020(v8, v6, (int)idx[56], -0x1);
        *((unsigned long long *)&idx[8]) = (long long)idx[8] + v10;
        if (v10)
        {
            *(a1) = idx;
            if (fflush_unlocked(stdout))
            {
                v11 = dcgettext(NULL, "write error", 5);
                err = __errno_location();
                error(1, *(err), v11);
            }
        }
    }
    else if (v0.st_atime < (long long)idx[8])
    {
        sub_40b1b0(0, 3, v6);
        error(0, 0, dcgettext(NULL, "%s: file truncated", 5));
        if (lseek((int)idx[56], 0, 0) < 0)
            sub_405f40(0, 0, v6); /* do not return */
        *((long long *)&idx[8]) = 0;
        goto LABEL_4064a1;
    }
    else if (!(v0.st_atime == (long long)idx[8]) || (unsigned int)((v2 < (long long)idx[24]) - ((long long)idx[24] < v2)) + (unsigned int)((v1 < (long long)idx[16]) - ((long long)idx[16] < v1)) * 2)
    {
        goto LABEL_4064a1;
    }
    return v3 - *((long long *)(40 + v15));
}



// Function: recheck @ 0x6660
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

extern unsigned int g_413020;
extern char g_41310c;
extern char g_41311a;

long long recheck(void* idx, char a1)
{
    unsigned int v6;  // eax
    unsigned int v7;  // r15d
    unsigned long v15;  // r14
    char v17;  // al
    unsigned long v18;  // r13
    unsigned long v20;  // r15
    char *v21;  // rsi
    unsigned long v22;  // r15
    unsigned int v23;  // eax
    unsigned int v8;  // r14d
    unsigned long v25;  // r15
    unsigned long v27;  // r13
    unsigned long v28;  // r14
    int *v30;  // dl, Other Possible Types: char
    int *err;  // rax
    unsigned int v32;  // edx
    unsigned long v33;  // r13
    unsigned int v34;  // eax
    unsigned int v9;  // r12d
    int *v35;  // r8
    unsigned int v36;  // eax
    char *v37;  // rax
    unsigned long v38;  // r13
    unsigned long v40;  // r12
    unsigned long v41;  // fs
    unsigned int v10;  // eax
    unsigned int v11;  // eax
    stat *v12;  // rsi
    unsigned int v13;  // eax
    int *v0;  // [bp-0xf0], Other Possible Types: char, unsigned long, unsigned int
    stat v1;  // [bp-0xd8]
    unsigned long v2;  // [bp-0x80]
    unsigned long v3;  // [bp-0x78]
    unsigned long v4;  // [bp-0x40]

    v6 = strcmp(*((long long *)idx), "-");
    v7 = (int)idx[60];
    v8 = v6;
    v0 = (char)idx[54];
    if (v8)
    {
        v9 = sub_407dd0(*((long long *)idx), (a1 ^ 1) * 0x800);
        v10 = (int)idx[60];
    }
    else
    {
        v9 = 0;
        v10 = v7;
    }
    if ((int)idx[56] == 0xffffffff == !v10)
        __assert_fail(); /* do not return */
    v30 = g_41311a;
    if (!g_41311a)
    {
        *((char *)&idx[54]) = 1;
        if (!g_41310c && (v30 = g_41311a, v11 = (unsigned int)(int)(long long)lstat(*((long long *)idx), &v1), !(int)(long long)lstat(*((long long *)idx), &v1) && ((unsigned short)v1.st_nlink & 0xf000) == 0xa000))
            goto LABEL_406cd0;
        if (v9 != 0xffffffff)
        {
            v12 = &v1;
LABEL_4067fb:
            if (fstat(v9, v12) >= 0)
            {
                if (!((v1.st_nlink & 0xf000) - 0x1000 & 0xffffe000) || ((unsigned short)v1.st_nlink & 0xb000) == 0x8000)
                {
                    v0 = *((long long *)idx);
                    if (!strcmp(*((long long *)idx), "-"))
                        v0 = dcgettext(NULL, "standard input", 5);
                    v17 = sub_405460(v9, v0);
                    *((char *)&idx[53]) = v17;
                    if (!v17 || g_41310c)
                    {
                        *((unsigned int *)&idx[60]) = 0;
                        if (v7 & 0xfffffffd)
                        {
                            if ((int)idx[56] == 0xffffffff)
                            {
                                v20 = *((long long *)idx);
                                if (!strcmp(*((long long *)idx), "-"))
                                    v20 = dcgettext(NULL, "standard input", 5);
                                sub_40ae30(4, v20);
                                v21 = "%s has become accessible";
                                goto LABEL_4068cb;
                            }
                            else
                            {
                                __assert_fail(); /* do not return */
                            }
                        }
                        else
                        {
                            v22 = *((long long *)idx);
                            v0 = (int)idx[56];
                            v23 = strcmp(v22, "-");
                            if (v0 != 0xffffffff)
                            {
                                if ((long long)idx[40] == v1.st_ino && (long long)idx[32] == *((unsigned long long *)&v1))
                                {
                                    if (!v23)
                                        v22 = dcgettext(NULL, "standard input", 5);
                                    sub_405bd0(v9, v22);
                                    return v4 - *((long long *)(40 + v41));
                                }
                                if (!v23)
                                    v22 = dcgettext(NULL, "standard input", 5);
                                sub_40ae30(4, v22);
                                error(0, 0, dcgettext(NULL, "%s has been replaced;  following new file", 5));
                                v25 = *((long long *)idx);
                                if (!strcmp(*((long long *)idx), "-"))
                                    v25 = dcgettext(NULL, "standard input", 5);
                                sub_405bd0((int)idx[56], v25);
                            }
                            else
                            {
                                if (!v23)
                                    v22 = dcgettext(NULL, "standard input", 5);
                                sub_40ae30(4, v22);
                                v21 = "%s has appeared;  following new file";
LABEL_4068cb:
                                error(0, 0, dcgettext(NULL, v21, 5));
                            }
                            *((unsigned int *)&idx[56]) = v9;
                            *((char *)&idx[52]) = 0;
                            *((unsigned long long *)&idx[8]) = 0;
                            *((unsigned long *)&idx[16]) = v2;
                            *((unsigned int *)&idx[64]) = (!v8 ? 0xffffffff : a1);
                            v27 = *((long long *)idx);
                            *((unsigned long *)&idx[24]) = v3;
                            *((unsigned long long *)&idx[88]) = 0;
                            *((unsigned long long *)&idx[32]) = *((unsigned long long *)&v1);
                            *((unsigned long *)&idx[40]) = v1.st_ino;
                            *((unsigned int *)&idx[48]) = v1.st_nlink;
                            if (!strcmp(v27, "-"))
                                v27 = dcgettext(NULL, "standard input", 5);
                            if (lseek(v9, 0, 0) < 0)
                                sub_405f40(0, 0, v27); /* do not return */
                            return v4 - *((long long *)(40 + v41));
                        }
                    }
                    else
                    {
                        v18 = *((long long *)idx);
                        *((unsigned int *)&idx[60]) = 0xffffffff;
                        if (!strcmp(v18, "-"))
                            v18 = dcgettext(NULL, "standard input", 5);
                        sub_40ae30(4, v18);
                        error(0, 0, dcgettext(NULL, "%s has been replaced with an untailable remote file", 5));
                        *((unsigned short *)&idx[52]) = 0x101;
                        goto LABEL_4069a7;
                    }
                }
                else
                {
                    *((unsigned int *)&idx[60]) = 0xffffffff;
                    *((char *)&idx[54]) = 0;
                    if (v30 && g_413020 == 1)
                    {
                        *((char *)&idx[52]) = 0;
                        if (!v0 && v7 == 0xffffffff)
                            goto LABEL_4069a7;
                    }
                    else
                    {
                        *((char *)&idx[52]) = 1;
                        if (!v0 && v7 == 0xffffffff)
                            goto LABEL_4069a7;
                        dcgettext(NULL, "; giving up on this name", 5);
                    }
                    v15 = *((long long *)idx);
                    if (!strcmp(*((long long *)idx), "-"))
                        v15 = dcgettext(NULL, "standard input", 5);
                    sub_40ae30(4, v15);
                    error(0, 0, dcgettext(NULL, "%s has been replaced with an untailable file%s", 5));
                    goto LABEL_4069a7;
                }
            }
        }
LABEL_406714:
        v30 = (char)idx[54];
        err = __errno_location();
        v28 = *((long long *)idx);
        v32 = *(err);
        v33 = v28;
        *((unsigned int *)&idx[60]) = v32;
        if (!v30)
            goto LABEL_406991;
        v0 = v32;
        v30 = err;
        v34 = strcmp(v28, "-");
        if (v0 != v7)
        {
            v35 = v30;
            if (!v34)
            {
                v0 = v35;
                v35 = v0;
                v28 = dcgettext(NULL, "standard input", 5);
            }
            v0 = v35;
            sub_40b1b0(0, 3, v28);
            error(0, *(v0), "%s");
            v33 = *((long long *)idx);
        }
    }
    else if (v9 != 0xffffffff)
    {
        *((char *)&idx[54]) = 1;
        v12 = &v1;
        if (g_41310c || !(v30 = g_41311a, v12 = (stat *)&v1, v13 = (unsigned int)(int)(long long)lstat(*((long long *)idx), &v1), !(int)(long long)lstat(*((long long *)idx), &v1) && ((unsigned short)v1.st_nlink & 0xf000) == 0xa000))
            goto LABEL_4067fb;
        goto LABEL_406cd0;
    }
    else
    {
        *((char *)&idx[54]) = 0;
        v28 = *((long long *)idx);
        if (g_41310c)
        {
            *((int *)&idx[60]) = *(__errno_location());
LABEL_406991:
            v36 = strcmp(v28, "-");
            if (!v0)
            {
LABEL_4069a7:
                v33 = *((long long *)idx);
                if (!strcmp(v33, "-"))
                {
LABEL_4069bd:
                    v33 = dcgettext(NULL, "standard input", 5);
                }
LABEL_40676d:
                sub_405bd0(v9, v33);
                v40 = *((long long *)idx);
                if (!strcmp(*((long long *)idx), "-"))
                    v40 = dcgettext(NULL, "standard input", 5);
                sub_405bd0((int)idx[56], v40);
                *((unsigned int *)&idx[56]) = 0xffffffff;
                return v4 - *((long long *)(40 + v41));
            }
            if (!v36)
                v28 = dcgettext(NULL, "standard input", 5);
            sub_40ae30(4, v28);
            v37 = dcgettext(NULL, "%s has become inaccessible", 5);
            error(0, (int)idx[60], v37);
            v33 = *((long long *)idx);
        }
        else
        {
            if (lstat(v28, &v1) || ((unsigned short)v1.st_nlink & 0xf000) != 0xa000)
                goto LABEL_406714;
LABEL_406cd0:
            v38 = *((long long *)idx);
            *((unsigned int *)&idx[60]) = 0xffffffff;
            *((char *)&idx[52]) = 1;
            if (!strcmp(v38, "-"))
                v38 = dcgettext(NULL, "standard input", 5);
            sub_40ae30(4, v38);
            error(0, 0, dcgettext(NULL, "%s has been replaced with an untailable symbolic link", 5));
            v33 = *((long long *)idx);
        }
    }
    if (!strcmp(v33, "-"))
        goto LABEL_4069bd;
    goto LABEL_40676d;
}



// Function: usage @ 0x6f70
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char field_0;
    char padding_1[15];
    char *field_10;
} struct_0;

extern FILE *stdout;
extern unsigned long long stderr;

void usage(int a0)
{
    unsigned long long v16;  // r15
    unsigned long long v17;  // r14
    char *v26;  // rax
    char *v27;  // rax
    char *v28;  // rax
    char *v29;  // rax
    char *v31;  // rax
    char *v32;  // rax
    char *v33;  // rax
    char *v34;  // rax
    char *v35;  // rax
    unsigned long long v18;  // r13
    char *v36;  // rax
    char *v37;  // rax
    char *v38;  // rsi
    struct_0 *v39;  // rbx
    char *v40;  // r14
    char *v42;  // rax
    char *v43;  // rax
    unsigned long long v19;  // r12
    unsigned long long v20;  // rbx
    unsigned long v21;  // fs
    char *v22;  // rax
    struct_0 *v23;  // rbx
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [FILE]...\n", 5));
        __printf_chk(1, dcgettext(NULL, "Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n", 5));
        v26 = dcgettext(NULL, "\nWith no FILE, or when FILE is -, read standard input.\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means 'descriptor'\n  -F                       same as --follow=name --retry\n", 5);
        fputs_unlocked(v29, stdout);
        __printf_chk(1, dcgettext(NULL, "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n", 5));
        v31 = dcgettext(NULL, "      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n", 5);
        fputs_unlocked(v31, stdout);
        v32 = dcgettext(NULL, "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n", 5);
        fputs_unlocked(v32, stdout);
        v33 = dcgettext(NULL, "  -z, --zero-terminated    line delimiter is NUL, not newline\n", 5);
        fputs_unlocked(v33, stdout);
        v34 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v34, stdout);
        v35 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v35, stdout);
        v36 = dcgettext(NULL, "\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n", 5);
        fputs_unlocked(v36, stdout);
        v37 = dcgettext(NULL, "With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n", 5);
        fputs_unlocked(v37, stdout);
        v0 = (int)_INSERT(v0 CONCAT 0, 0, "[");
        v38 = "[";
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
        } while (strcmp("tail", v38) && (v38 = v39->field_10, v23 = v39 + 16, v39->field_10));
        v40 = *((long long *)&v23->padding_1[7]);
        if (!*((long long *)&v23->padding_1[7]))
            v40 = "tail";
        __printf_chk(1, dcgettext(NULL, "\n%s online help: <%s>\n", 5));
        v42 = setlocale(5, NULL);
        if (v42 && strncmp(v42, "en_", 3))
        {
            v43 = dcgettext(NULL, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
            fputs_unlocked(v43, stdout);
        }
        __printf_chk(1, dcgettext(NULL, "Full documentation <%s%s>\n", 5));
        __printf_chk(1, dcgettext(NULL, "or available locally via: info '(coreutils) %s%s'\n", 5));
    }
    exit(a0); /* do not return */
}



// Function: raw_hasher @ 0x7e50
void raw_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: raw_comparator @ 0x7e70
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0x7e80
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

extern unsigned int g_40e590[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_40e590[0])
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
    idx->field_28 = &g_40e590[0];
    return 0;
}



// Function: hash_find_entry @ 0x7f10
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
LABEL_407fbf:
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
        goto LABEL_407fbf;
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



// Function: transfer_entries @ 0x8160
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
LABEL_408188:
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
                            goto LABEL_40820e;
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
LABEL_40820e:
                            v2 = iter->field_0;
                            goto LABEL_408211;
                        }
                    }
                    index = v6;
                }
            }
LABEL_408211:
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_408188;
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



// Function: hash_print_statistics @ 0x83d0
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
            goto LABEL_408509;
LABEL_40848a:
        v12 = (int)v5;
    }
    else
    {
        v11 = MulV(AddV(v6 >> 1 | (unsigned int)v6 & 1, v6 >> 1 | (unsigned int)v6 & 1), 0x4059000000000000);
        if (v5 >= 0)
            goto LABEL_40848a;
LABEL_408509:
        v12 = (int)(AddV(v5 >> 1 | (unsigned int)v5 & 1, v5 >> 1 | (unsigned int)v5 & 1));
    }
    __fprintf_chk(a1, 0x1, "# buckets used:    %lu (%.2f%%)\n", v6, (unsigned long long)(DivV(v11, v12)));
    return __fprintf_chk(*((unsigned int *)&&v0), 0x1, "max bucket length: %lu\n", v2);
}



// Function: hash_lookup @ 0x8530
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

unsigned long long hash_lookup(struct_0 *a0, unsigned long long a1)
{
    unsigned long long v1;  // rax
    struct_3 *iter;  // rbx
    unsigned long long i;  // rsi

    v1 = a0->field_30(a1, a0->field_10);
    if (v1 >= a0->field_10)
        abort(); /* do not return */
    iter = v1 * 16 + a0->field_0;
    if (!iter->padding_0)
        return 0;
    for (i = (unsigned long long)iter->padding_0; i != a1; i = (unsigned long long)iter->padding_0)
    {
        if ((char)a0->field_38(a1))
            return iter->padding_0;
        iter = iter->field_8;
        if (!iter)
            return 0;
    }
    return a1;
}



// Function: hash_get_next @ 0x85f0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_4085fc();
    return;
}



// Function: hash_do_for_each @ 0x86d0
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



// Function: hash_initialize @ 0x87c0
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

extern struct_0 g_40e590;

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

    v1 = sub_407e50;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_407e70;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_40e590.padding_0[0];
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_407e80(ptr))
    {
        v6 = sub_408020(a0, v2->field_10);
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



// Function: hash_clear @ 0x88d0
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



// Function: hash_free @ 0x8980
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
            goto LABEL_408a2c;
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
LABEL_408a2c:
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



// Function: hash_rehash @ 0x8a70
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

    v11 = sub_408020(a1, idx->field_28->field_10);
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
        v13 = sub_408160(&ptr, idx, 0);
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
        if ((char)sub_408160(idx, &ptr, 1) && (char)sub_408160(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0x8bf0
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
    sub_407f10(idx, a1, &v0, 0);
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
                goto LABEL_408d06;
LABEL_408c7f:
            v6 = (int)(unsigned int)v5;
        }
        else
        {
            v5 = *((long long *)&idx[1].field_0);
            v4 = AddV((unsigned int)(v3 >> 1) | (unsigned int)v3 & 1, (unsigned int)(v3 >> 1) | (unsigned int)v3 & 1);
            if (v5 >= 0)
                goto LABEL_408c7f;
LABEL_408d06:
            v6 = (int)(AddV((unsigned int)(v5 >> 1 | (unsigned int)v5 & 1), (unsigned int)(v5 >> 1 | (unsigned int)v5 & 1)));
        }
        if (((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69 | (char)((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69) >> 6)) & 1) != 1)
        {
            sub_407e80(idx);
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
                else if (!(char)sub_408a70(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else
                {
                    sub_407f10(idx, a1, &v0, 0);
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



// Function: hash_insert @ 0x8e20
long long hash_insert(unsigned int *a0, long long a1)
{
    unsigned int v2;  // eax
    long long v0;  // [bp-0x18], Other Possible Types: unsigned long long

    v2 = sub_408bf0(a0, a1, &v0);
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



// Function: hash_remove @ 0x8e80
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

    sub_407f10(idx, a1, &v0, 1);
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
            goto LABEL_408fc6;
LABEL_408f10:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = *((long long *)&idx[1].field_0);
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_408f10;
LABEL_408fc6:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_407e80(idx);
    v9 = *((long long *)&idx[2].field_8);
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_408a70(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



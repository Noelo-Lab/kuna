// Function: dfaerror @ 0x513f
void dfaerror(unsigned long long a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    error(2, 0, "%s");
}



// Function: dfawarn @ 0x5174
void dfawarn(unsigned long long a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    error(0, 0, gettext("warning: %s"));
    return;
}



// Function: kwsmusts @ 0x51b4
typedef struct struct_1 {
    char field_0;
    char field_1;
    char field_2;
} struct_1;

typedef struct struct_0 {
    long long field_0;
    long long field_8;
    char padding_10[40];
    unsigned long long field_38;
    char field_40;
} struct_0;

extern char g_433183;

struct_1 * kwsmusts(struct_0 *idx)
{
    unsigned long len;  // rax
    struct_1 *v0;  // [bp-0x30]
    unsigned long len1;  // [bp-0x28]
    long long v2;  // [bp-0x20]
    char *v3;  // [bp-0x18]
    char *iter;  // [bp-0x10]

    v0 = sub_41ae2f(idx->field_8);
    if (!v0)
        return v0;
    idx->field_0 = sub_40f7c8(0);
    if (v0->field_0)
    {
        idx->field_38 = idx->field_38 + 1;
        len1 = strlen(v0 + 1);
        v2 = v0->field_2 + v0->field_1 + len1;
        v3 = sub_426b10(v2);
        iter = v3;
        *(iter) = g_433183;
        iter = &iter[v0->field_1];
        idx->field_40 = v0->field_1 || idx->field_40;
        memcpy(iter, v0 + 1, len1);
        if (v0->field_2)
            iter[len1] = g_433183;
        sub_40cacb(idx->field_0, v3, v2, v3);
        free(v3);
    }
    else
    {
        len = strlen(v0 + 1);
        sub_40cacb(idx->field_0, v0 + 1, len, v0 + 1);
    }
    sub_40d9b5(idx->field_0);
    return sub_41bb28(v0);
}



// Function: possible_backrefs_in_pattern @ 0x5354
typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

unsigned int possible_backrefs_in_pattern(struct_0 *a0, unsigned long long a1, char a2)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned int v1;  // [bp-0x1c]
    struct_0 *ptr;  // [bp-0x18]
    struct_0 *v3;  // [bp-0x10]

    v0 = a1;
    v1 = (!a2 ? 128 : 92);
    v0 -= 1;
    if (!(v0 - 0 >> 63 & 1))
    {
        v3 = &(a0->padding_0)[v0];
        ptr = a0;
        while (1)
        {
            ptr = memchr(ptr, 92, v3 - ptr);
            if (!ptr)
                break;
            if (ptr->field_1 > 48 && ptr->field_1 <= 57)
                return 1;
            if (v1 == ptr->field_1 && (ptr += 1, ptr == v3))
                break;
            ptr = &ptr->field_1;
        }
    }
    return 0;
}



// Function: regex_compile @ 0x5428
typedef struct regex_t {
} regex_t;

typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
} struct_1;

typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    void* field_20;
    char padding_28[8];
    unsigned long long field_30;
    unsigned long long field_38;
} struct_0;

extern char g_42901f;
extern char g_433180;

unsigned int regex_compile(struct_1 *a0, long long a1, long long a2, unsigned long a3, unsigned long a4, long long a5, char a6)
{
    struct_0 *idx;  // rax
    char v0;  // [bp-0x90]
    long long v1;  // [bp-0x88]
    unsigned long long v2;  // [bp-0x80]
    char *v3;  // [bp-0x78]
    unsigned long long v4;  // [bp-0x70]
    regex_t v5;  // [bp-0x68]
    unsigned long long v6;  // [bp-0x60]
    unsigned long v7;  // [bp-0x58]
    unsigned long v8;  // [bp-0x50]
    void* v9;  // [bp-0x48]
    unsigned long long v10;  // [bp-0x40]
    unsigned long v11;  // [bp-0x38]
    unsigned long v12;  // [bp-0x30]

    v5 = (regex_t)0;
    v6 = 0;
    v1 = 0xff;
    v9 = (!g_433180 && !a6 ? sub_426b10(v1 + 1) : NULL);
    v10 = 0;
    if (a6)
        re_set_syntax(a5 | 0x2000000);
    else
        re_set_syntax(a5);
    v2 = re_compile_pattern(a1, a2, &v5, a2);
    if (!v2)
    {
        if (a6)
        {
            regfree(&v5);
        }
        else
        {
            idx = a3 * 64 + a0->field_10;
            *((struct regex_t *)&idx->padding_0[0]) = v5;
            *((unsigned long long *)&idx->padding_0[8]) = v6;
            idx->field_10 = v7;
            idx->field_18 = v8;
            idx->field_20 = v9;
            *((unsigned long long *)&idx->padding_28[0]) = v10;
            idx->field_30 = v11;
            idx->field_38 = v12;
        }
        return 1;
    }
    else
    {
        free(v9);
        v3 = (a4 - 0 >> 63 & 1 ? &g_42901f : sub_406a2e(a4, &v0, &v0));
        if (!*(v3))
        {
            error(0, 0, "%s");
        }
        else
        {
            v4 = v0;
            error(0, 0, "%s:%td: %s");
        }
        return 0;
    }
}



// Function: GEAcompile @ 0x5661
extern char g_400000;
extern char g_433180;
extern char g_433181;
extern char g_433182;
extern char g_433183;
extern char g_4331a0;
extern char g_4331a2;

void* GEAcompile(unsigned long a0, unsigned long a1, unsigned long long a2, char a3)
{
    long long v28;  // rdi
    long long v29;  // rsi
    long long v30;  // rdx
    long long v31;  // rcx
    long long v32;  // r8
    long long v33;  // r9
    unsigned long long v0;  // [bp-0xd0]
    unsigned long n1;  // [bp-0xc8]
    unsigned long v2;  // [bp-0xc0]
    char v3;  // [bp-0xb3]
    char v4;  // [bp-0xb2]
    char v5;  // [bp-0xb1]
    unsigned int v6;  // [bp-0xb0]
    unsigned int v7;  // [bp-0xac]
    unsigned long v8;  // [bp-0xa8]
    unsigned long v9;  // [bp-0xa0]
    unsigned long ptr2;  // [bp-0x98]
    unsigned long i;  // [bp-0x90]
    unsigned long ptr;  // [bp-0x88]
    unsigned long v13;  // [bp-0x80]
    unsigned long iter;  // [bp-0x78]
    unsigned long long v15;  // [bp-0x70]
    void* idx;  // [bp-0x68]
    unsigned long v17;  // [bp-0x60]
    unsigned long ptr1;  // [bp-0x58]
    unsigned long v19;  // [bp-0x50]
    unsigned long n;  // [bp-0x48]
    long long v21;  // [bp-0x40]
    long long v22;  // [bp-0x38]
    unsigned long n2;  // [bp-0x30]
    unsigned long long v24;  // [bp-0x28]
    unsigned long v25;  // [bp-0x20]
    unsigned long long len;  // [bp-0x18], Other Possible Types: unsigned long

    v2 = a0;
    n1 = a1;
    v0 = a2;
    idx = sub_426fc8(1, 72);
    *((unsigned long long *)&idx[8]) = sub_41bb47(v28, v29, v30, v31, v32, v33);
    if (g_433180)
        v0 |= &g_400000;
    v6 = (!g_433183 ? 2 : 0) | (unsigned int)v0 & 16 | 44;
    sub_41bb5b((long long)idx[8], &g_4331a0, v0, v6);
    v4 = g_4331a2 || g_4331a0 ^ 1;
    i = v2;
    v17 = v2 + n1;
    v3 = 0;
    *((unsigned long long *)&idx[16]) = sub_426aea(64);
    *((unsigned long long *)&idx[16]) = (long long)idx[16] + 64;
    *((unsigned long *)&idx[24]) = 0;
    v8 = 1;
    ptr = v2;
    v13 = 0;
    iter = 0;
    v9 = 0;
    v15 = 0;
    do
    {
        ptr1 = rawmemchr(i, 10);
        v19 = ptr1 - i;
        v5 = sub_405354(i, v19, v4);
        if (v5 && ptr < i)
        {
            n = i - ptr;
            v21 = n + iter - v9;
            if (v21 > 0)
                v13 = sub_426dc6(v13, &v9, v21, -0x1, 1);
            memcpy(iter + v13, ptr, n);
            iter += n;
        }
        v22 = (long long)idx[24] - v8 + 2;
        if (v22 > 0)
        {
            *((unsigned long long *)&idx[16]) = sub_426dc6((long long)idx[16] - 64, &v8, v22, -0x1, 64);
            *((unsigned long long *)&idx[16]) = (long long)idx[16] + 64;
        }
        re_set_syntax(v0);
        if ((char)sub_405428(idx, i, v19, (long long)idx[24], v15, v0, !v5) != 1)
            v3 = 1;
        i = ptr1 + 1;
        v15 += 1;
        if (v5)
        {
            *((unsigned long long *)&idx[24]) = (long long)idx[24] + 1;
            ptr = i;
        }
    } while (i <= v17);
    if (v3)
        exit(2); /* do not return */
    if (ptr <= v17)
    {
        if (v2 < ptr)
        {
            n2 = v17 - ptr;
            v13 = sub_426b9f(v13, iter + n2, iter + n2);
            memcpy(iter + v13, ptr, n2);
            iter += n2;
        }
        else
        {
            v13 = v2;
            iter = n1;
        }
    }
    if (g_433181 || g_433182)
    {
        v7 = !((unsigned int)v0 & 0x2000);
        v24 = 45;
        v25 = sub_426b10(v24 + n1);
        strcpy(v25, (!g_433182 ? (!v7 ? "(^|[^[:alnum:]_])(" : "\\(^\\|[^[:alnum:]_]\\)\\(") : (!v7 ? "^(" : "^\\(")));
        len = strlen(v25);
        memcpy(len + v25, v2, n1);
        len += n1;
        strcpy(v25 + len, (!g_433182 ? (!v7 ? ")([^[:alnum:]_]|$)" : "\\)\\([^[:alnum:]_]\\|$\\)") : (!v7 ? ")$" : "\\)$")));
        len += strlen(v25 + len);
        ptr2 = v25;
        v2 = ptr2;
        n1 = len;
    }
    else
    {
        ptr2 = 0;
    }
    sub_414b84(v2, n1, (long long)idx[8], n1);
    sub_4051b4(idx);
    sub_41a490(0, 0, (long long)idx[8], 1);
    if (!v13)
    {
        free(ptr2);
        return idx;
    }
    if (a3 || (char)sub_419e86((long long)idx[8]) != 1)
    {
        *((unsigned long long *)&idx[16]) = (long long)idx[16] - 64;
        *((unsigned long long *)&idx[24]) = (long long)idx[24] + 1;
        if ((char)sub_405428(idx, v13, iter, 0, 0xffffffffffffffff, v0, 0) != 1)
            abort(); /* do not return */
    }
    if (v13 != v2)
        free(v13);
    free(ptr2);
    return idx;
}



// Function: EGexecute @ 0x5d23
typedef struct struct_1 {
    unsigned int field_0;
} struct_1;

typedef struct struct_2 {
    char padding_0[56];
    char field_38;
} struct_2;

typedef struct struct_0 {
    long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
    char padding_20[16];
    struct struct_1 *field_30;
    long long field_38;
    char field_40;
} struct_0;

extern char g_433181;
extern char g_433182;
extern char g_433183;
extern char g_4331a0;
extern char g_4331a2;

unsigned long long EGexecute(struct_0 *idx, unsigned long a1, unsigned long a2, long long *a3, unsigned long a4)
{
    long long v24;  // rax
    long long v25;  // rdx
    long long v26;  // rax
    long long v27;  // rdx
    long long v28;  // rax
    struct_2 *idx1;  // rax
    struct_2 *idx2;  // rax
    struct_2 *index;  // rax
    struct_2 *v32;  // rax
    char v0;  // [bp-0xbc]
    char v1;  // [bp-0xbb]
    char v2;  // [bp-0xba]
    char v3;  // [bp-0xb9]
    int v4;  // [bp-0xb8]
    unsigned int v5;  // [bp-0xb4]
    unsigned long v6;  // [bp-0xb0]
    unsigned long v7;  // [bp-0xa8]
    unsigned long node;  // [bp-0xa0]
    unsigned long ptr;  // [bp-0x98]
    unsigned long v10;  // [bp-0x90]
    unsigned long i;  // [bp-0x88]
    unsigned long v12;  // [bp-0x80]
    unsigned long v13;  // [bp-0x78], Other Possible Types: long long
    long long v14;  // [bp-0x70]
    long long v15;  // [bp-0x68]
    unsigned long v16;  // [bp-0x60]
    long long v17;  // [bp-0x50]
    unsigned long v18;  // [bp-0x48]
    unsigned long long v19;  // [bp-0x40]
    unsigned long v20;  // [bp-0x38]
    unsigned long v21;  // [bp-0x30]
    long long v22;  // [bp-0x28]

    v2 = g_433183;
    v17 = sub_419da3(idx->field_8);
    v3 = sub_419dbc(idx->field_8);
    v6 = a1;
    v18 = a1 + a2;
    ptr = a1;
    node = ptr;
    while (1)
    {
        if (ptr >= v18)
            return 0xffffffffffffffff;
        ptr = v18;
        if (a4)
        {
            v10 = a4;
LABEL_406255:
            if (ptr - node > 0x80000000)
                sub_427122(); /* do not return */
            v12 = ptr;
            v14 = 0;
            v15 = 0;
            while (1)
            {
                if (v15 >= idx->field_18)
                {
                    if (v12 < ptr)
                    {
                        node = v12;
                        v13 = v14;
                        *(a3) = v13;
                        return node - a1;
                    }
                    goto LABEL_4066c4;
                }
                idx1 = v15 * 64 + idx->field_10;
                idx1->field_38 = idx1->field_38 & 191;
                idx2 = v15 * 64 + idx->field_10;
                idx2->field_38 = idx2->field_38 & 127 | (g_433183 == 10) * 128;
                v5 = re_search(idx->field_10 + v15 * 64, node, (unsigned int)(ptr - node) - 1, v10 - node & 0xffffffff, (unsigned int)(ptr - v10) - 1, idx->padding_20);
                if (v5 < 0xffffffff)
                    sub_427122(); /* do not return */
                if (v5 >= 0 && (v13 = (long long)(int)(idx->field_30->field_0 - v5), i = node + (long long)(int)v5, i <= v12))
                    break;
LABEL_406687:
                v15 += 1;
            }
            if (!a4 || g_433181 == 1)
            {
                if (g_433182 != 1 && g_433181 != 1 || g_433182 && v13 == ptr - v10 - 1)
                {
                    i = v10;
                    v13 = ptr - v10;
                }
                else
                {
                    if (g_433182 != 1 && g_433181)
                    {
                        while (i <= v12)
                        {
                            v4 = 0;
                            if (!sub_40fbe0(i + v13, ptr - 1, ptr - 1, v13) && !sub_40fc0e(node, i, ptr - 1, i))
                                goto LABEL_406645;
                            if (v13 > 0)
                            {
                                v13 -= 1;
                                index = v15 * 64 + idx->field_10;
                                index->field_38 = index->field_38 | 64;
                                v4 = re_match(idx->field_10 + v15 * 64, node, i + v13 - v10 & 0xffffffff, i - node & 0xffffffff, idx->padding_20);
                                if (v4 < 0xffffffff)
                                    sub_427122(); /* do not return */
                            }
                            if (v4 > 0)
                            {
                                v13 = v4;
                            }
                            else
                            {
                                if (i == ptr - 1)
                                    break;
                                i += 1;
                                v32 = v15 * 64 + idx->field_10;
                                v32->field_38 = v32->field_38 & 191;
                                v5 = re_search(idx->field_10 + v15 * 64, node, (unsigned int)(ptr - node) - 1, i - node & 0xffffffff, (unsigned int)(ptr - i) - 1, idx->padding_20);
                                if (v5 < 0)
                                {
                                    if (v5 < 0xffffffff)
                                        sub_427122(); /* do not return */
                                    break;
                                }
                                else
                                {
                                    v13 = idx->field_30->field_0 - v5;
                                    i = node + v5;
                                }
                            }
                        }
                        goto LABEL_406645;
                    }
                    goto LABEL_406687;
                }
            }
LABEL_406645:
            if (a4 && (i < v12 || i == v12 && v13 > v14))
            {
                v12 = i;
                v14 = v13;
                goto LABEL_406687;
            }
            break;
        }
        v16 = node;
        v7 = 0;
        v1 = 0;
        v0 = 0;
        if (idx->field_0)
        {
            v19 = sub_40f585(idx->field_0, -(idx->field_40) + node, v18 - node + idx->field_40, &v22, 1);
            if (v19 - 0 >> 63 & 1)
                return v19;
            i = node + v19;
            v20 = node;
            node = memrchr(a1, v2, i - a1);
            node = (!node ? a1 : node + 1);
            v16 = node;
            v1 = v22 < idx->field_38;
            if (!v1 && v3 == 1)
            {
                v24 = i - node;
                v25 = 16;
                if (16 <= v24)
                    v25 = v24;
                if (v25 >= (long long)(i - v20) >> 2)
                {
                    v26 = i - node;
                    v27 = 16;
                    if (16 <= v26)
                        v27 = v26;
                    if (v27 < (long long)(v18 - v20) >> 2)
                    {
                        v28 = i - node;
                        if (v28 < 16)
                            v28 = 16;
                        ptr = rawmemchr(v20 + v28 * 4, v2);
                        ptr += 1;
                    }
                    else
                    {
                        ptr = v18;
                    }
LABEL_406014:
                    if (v1)
                    {
                        if (g_4331a2 || g_4331a0 ^ 1)
                            break;
                        if (node > v6)
                            v6 = node;
                        if (!sub_40f84a(&v6, 0, i, v18))
                            break;
                        v16 = v6;
                        goto LABEL_406088;
                    }
                }
            }
            ptr = rawmemchr(i, v2);
            ptr += 1;
            goto LABEL_406014;
        }
        else
        {
LABEL_406088:
            if (v17 && v1 != 1)
            {
                v21 = sub_419d4d(v17, v16, ptr, 0, &v7, 0);
                if (v21 && v21 != ptr)
                {
                    if (v7)
                    {
                        node = memrchr(a1, v2, v21 - a1);
                        node += 1;
                        v16 = node;
                    }
                    ptr = rawmemchr(v21, v2);
                    ptr += 1;
                    v7 = 0;
                    goto LABEL_40616c;
                }
                else
                {
LABEL_4066c4:
                    node = ptr;
                }
            }
            else
            {
LABEL_40616c:
                v21 = sub_419d4d(idx->field_8, v16, ptr, 0, &v7, &v0);
                if (v21 && v21 != ptr)
                {
                    if (v7)
                    {
                        node = memrchr(a1, v2, v21 - a1);
                        node += 1;
                    }
                    ptr = rawmemchr(v21, v2);
                    ptr += 1;
                    if (v0 ^ 1)
                        break;
                    v10 = node;
                    goto LABEL_406255;
                }
                else
                {
                    goto LABEL_4066c4;
                }
            }
        }
    }
    v13 = ptr - node;
    *(a3) = v13;
    return node - a1;
}



// Function: hash_pattern @ 0x6760
extern unsigned long long g_433780;

long long hash_pattern(unsigned long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x38]
    char *i;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    unsigned long long v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    unsigned long v5;  // [bp-0x10]

    v2 = 5381;
    v3 = 3657500101;
    v4 = 4123221751654370051;
    v0 = v4;
    v5 = a0 - 1;
    for (i = v5 + g_433780; *(i) != 10; i += 1)
    {
        v0 = *(i) ^ v0 * 33;
    }
    return v0 % a1;
}



// Function: compare_patterns @ 0x67f8
extern unsigned long long g_433780;

unsigned int compare_patterns(unsigned long a0, unsigned long a1)
{
    char *v0;  // [bp-0x28]
    unsigned long long i;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    v2 = a0 - 1;
    v3 = a1 - 1;
    v0 = v2 + g_433780;
    for (i = v3 + g_433780; *(v0) == *((char *)i); i += 1)
    {
        if (*(v0) == 10)
            return 1;
        v0 += 1;
    }
    return 0;
}



// Function: update_patterns @ 0x687b
extern long long g_433768;
extern long long g_433770;
extern unsigned long long g_433778;
extern unsigned long long g_433788;
extern long long g_433790;

long long update_patterns(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3)
{
    unsigned long v10;  // rax
    unsigned long long *idx;  // rax
    unsigned int v0;  // [bp-0x68]
    unsigned int v1;  // [bp-0x64]
    void* iter;  // [bp-0x60]
    unsigned long long v3;  // [bp-0x58]
    void* node;  // [bp-0x50]
    void* v5;  // [bp-0x48]
    void* ptr;  // [bp-0x40]
    unsigned long n;  // [bp-0x38]
    unsigned long v8;  // [bp-0x30]

    iter = a0 + a1;
    v3 = 1;
    v0 = 0;
    v5 = a0 + a2;
    for (node = a0 + a1; node < v5; node += n)
    {
        ptr = rawmemchr(node, 10);
        n = ptr + 1 - node;
        memmove(iter, node, n);
        v8 = iter - a0 + 1;
        v1 = sub_4212a6(g_433790, v8, 0, v8);
        if (v1)
        {
            if (v1 < 0)
                sub_427122(); /* do not return */
            iter += n;
            if (!v0)
            {
                if (g_433778 == g_433770)
                    g_433768 = sub_426dc6(g_433768, &g_433770, 1, -0x1, 24);
                v10 = g_433778;
                g_433778 = v10 + 1;
                idx = v10 * 24 + g_433768;
                *(idx) = g_433788;
                idx[1] = a3;
                idx[2] = v3;
            }
            g_433788 = g_433788 + 1;
        }
        v0 = v1;
        v3 += 1;
    }
    return iter - a0;
}



// Function: pattern_file_name @ 0x6a2e
extern unsigned long long g_433768;
extern char g_433778;

long long pattern_file_name(long long a0, unsigned long long *a1)
{
    long long i;  // [bp-0x10]

    for (i = 1; i < *((long long *)&g_433778) && a0 >= *((long long *)(i * 24 + g_433768)); i += 1);
    *(a1) = a0 - *((long long *)(i * 24 - 24 + g_433768)) + *((long long *)(i * 24 - 24 + g_433768 + 16));
    return *((long long *)(i * 24 - 24 + g_433768 + 8));
}



// Function: clear_asan_poison @ 0x6b00
void clear_asan_poison(void)
{
    return;
}



// Function: asan_poison @ 0x6b0b
void asan_poison(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: pr_sgr_start @ 0x6b1e
extern long long g_433068;

long long pr_sgr_start(char *a0)
{
    unsigned long v1;  // rax

    v1 = *(a0);
    if ((char)v1)
        v1 = sub_4276e4(g_433068, a0, a0);
    return v1;
}



// Function: pr_sgr_end @ 0x6b52
extern long long g_433070;

long long pr_sgr_end(char *a0)
{
    unsigned long v1;  // rax

    v1 = *(a0);
    if ((char)v1)
        v1 = sub_427713(g_433070);
    return v1;
}



// Function: pr_sgr_start_if @ 0x6b7f
extern unsigned int g_433758;

long long pr_sgr_start_if(char *a0)
{
    unsigned long v1;  // rax

    v1 = g_433758;
    if (g_433758)
        v1 = sub_406b1e(a0);
    return v1;
}



// Function: pr_sgr_end_if @ 0x6ba8
extern unsigned int g_433758;

long long pr_sgr_end_if(char *a0)
{
    unsigned long v1;  // rax

    v1 = g_433758;
    if (g_433758)
        v1 = sub_406b52(a0);
    return v1;
}



// Function: color_cap_mt_fct @ 0x6bd1
extern unsigned long long g_433028;
extern unsigned long long g_433030;

long long color_cap_mt_fct(void)
{
    g_433030 = g_433028;
    return g_433028;
}



// Function: color_cap_rv_fct @ 0x6bea
extern unsigned int g_433758;

void color_cap_rv_fct(void)
{
    g_433758 = 0xffffffff;
    return;
}



// Function: color_cap_ne_fct @ 0x6bff
extern char g_4290e3;
extern char g_4290e9;
extern unsigned long long g_433068;
extern unsigned long long g_433070;

int color_cap_ne_fct(void)
{
    g_433068 = &g_4290e3;
    g_433070 = &g_4290e9;
    return &g_4290e9;
}



// Function: putchar_errno @ 0x6c26
extern unsigned int g_433798;

int putchar_errno(int a0)
{
    unsigned int v1;  // eax

    v1 = putchar_unlocked(a0);
    if (v1 >= 0)
        return v1;
    g_433798 = (unsigned int)__errno_location();
    return g_433798;
}



// Function: fputs_errno @ 0x6c53
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_433798;

int fputs_errno(char *a0)
{
    unsigned int v1;  // eax

    v1 = fputs_unlocked(a0, stdout);
    if (v1 >= 0)
        return v1;
    g_433798 = (unsigned int)__errno_location();
    return g_433798;
}



// Function: printf_errno @ 0x6c8d
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_433798;

void printf_errno(char *a0, unsigned long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v13;  // fs
    unsigned int v0;  // [bp-0xd8]
    unsigned int v1;  // [bp-0xd4]
    char *v2;  // [bp-0xd0]
    char *v3;  // [bp-0xc8]
    unsigned long v4;  // [bp-0xc0]
    char v5;  // [bp-0xb8]
    unsigned long long v6;  // [bp-0xb0]
    unsigned long long v7;  // [bp-0xa8]
    unsigned long long v8;  // [bp-0xa0]
    unsigned long long v9;  // [bp-0x98]
    unsigned long long v10;  // [bp-0x90]
    unsigned long v11;  // [bp+0x8]

    v6 = a1;
    v7 = a2;
    v8 = a3;
    v9 = a4;
    v10 = a5;
    v4 = *((long long *)(40 + v13));
    v0 = 8;
    v1 = 48;
    v2 = &v11;
    v3 = &v5;
    if (vfprintf(stdout, a0, &v0) < 0)
        g_433798 = *(__errno_location());
    if (v4 == *((long long *)(40 + v13)))
        return;
    __stack_chk_fail(); /* do not return */
}



// Function: fwrite_errno @ 0x6d74
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_433798;

long long fwrite_errno(void* a0, unsigned long a1, unsigned long a2)
{
    unsigned long count;  // rax
    unsigned int v2;  // eax

    count = fwrite_unlocked(a0, a1, a2, stdout);
    if (count == a2)
        return count;
    v2 = *(__errno_location());
    g_433798 = *(__errno_location());
    return g_433798;
}



// Function: fflush_errno @ 0x6dc0
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_433798;

int fflush_errno(void)
{
    unsigned int v1;  // eax

    v1 = fflush_unlocked(stdout);
    if (!v1)
        return v1;
    g_433798 = (unsigned int)__errno_location();
    return g_433798;
}



// Function: is_device_mode @ 0x6deb
unsigned int is_device_mode(unsigned int a0)
{
    unsigned int v1;  // eax

    if (((unsigned short)a0 & 0xf000) == 0x2000 || ((unsigned short)a0 & 0xf000) == 0x6000 || ((unsigned short)a0 & 0xf000) == 0xc000 || ((unsigned short)a0 & 0xf000) == 0x1000)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: skip_devices @ 0x6e43
extern unsigned int g_4337cc;

unsigned int skip_devices(char a0)
{
    unsigned int v1;  // eax

    if (g_4337cc == 2 || (a0 ^ 1) & !g_4337cc)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: usable_st_size @ 0x6e8a
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

long long usable_st_size(struct_0 *a0)
{
    unsigned long long v1;  // rax

    v1 = a0->field_18 & 0xf000;
    return _INSERT(v1, 0, (unsigned int)v1 == 0x8000);
}



// Function: input_filename @ 0x6eac
extern unsigned long long g_4337c0;

unsigned long long input_filename(void)
{
    if (!g_4337c0)
        g_4337c0 = gettext("(standard input)");
    return g_4337c0;
}



// Function: suppressible_error @ 0x6edf
extern char g_433755;
extern char g_4337c9;

long long suppressible_error(int a0)
{
    unsigned long v1;  // rax

    v1 = g_433755 ^ 1;
    if ((char)v1)
    {
        sub_406eac();
        v1 = (unsigned long long)error(0, a0, "%s");
    }
    g_4337c9 = 1;
    return v1;
}



// Function: clean_up_stdout @ 0x6f2c
extern unsigned int g_433798;

long long clean_up_stdout(void)
{
    unsigned long v1;  // rax

    v1 = g_433798;
    if (!g_433798)
        v1 = sub_41143e();
    return v1;
}



// Function: initialize_unibyte_mask @ 0x6f46
extern char g_4331a3;
extern char g_4337e8;

long long initialize_unibyte_mask(void)
{
    char v0;  // [bp-0x11]
    unsigned int j;  // [bp-0x10]
    unsigned int i;  // [bp-0xc]

    v0 = 0;
    for (j = 1; i <= 0xff; i += 1)
    {
        if (!(v0 & i) & *(&(&g_4331a3)[i]) != 1)
        {
            for (i = 1; i >= j * 2; j *= 2);
            v0 |= (char)j;
        }
    }
    *((unsigned long long *)&g_4337e8) = (0xffffffffffffffff * 9259542123273814145 >> 71) * v0;
    return (0xffffffffffffffff * 9259542123273814145 >> 71) * v0;
}



// Function: skip_easy_bytes @ 0x6fdd
extern unsigned long long g_4337e8;

void* skip_easy_bytes(void* a0)
{
    char v4;  // al
    char v5;  // al
    void* iter;  // [bp-0x20]
    void* node;  // [bp-0x18]
    void* v2;  // [bp-0x10]

    for (iter = a0; (char)iter & 7; iter += 1)
    {
        v4 = sub_40f64d(*((char *)iter));
        if (g_4337e8 & v4)
            return iter;
    }
    v2 = iter;
    for (node = v2; !(g_4337e8 & *((long long *)node)); node += 8);
    iter = node;
    while (1)
    {
        v5 = sub_40f64d(*((char *)iter));
        if (g_4337e8 & v5)
            break;
        iter += 1;
    }
    return iter;
}



// Function: buf_has_encoding_errors @ 0x7098
extern unsigned long long g_4337e8;

unsigned int buf_has_encoding_errors(void* a0, unsigned long a1)
{
    void* v0;  // [bp-0x28], Other Possible Types: unsigned long
    unsigned long long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]

    if (!g_4337e8)
        return 0;
    v2 = 0;
    *((char *)a0 + a1) = 0xff;
    v0 = a0;
    while (1)
    {
        v0 = sub_406fdd(v0);
        if (v0 >= a0 + a1)
            return 0;
        v1 = sub_40f686(v0, a0 + a1 - v0, &v2, a0 + a1 - v0);
        if (v1 - 0 >> 63 & 1)
            return 1;
        v0 += v1;
    }
}



// Function: buf_has_nulls @ 0x716c
char buf_has_nulls(char *a0, unsigned long a1)
{
    a0[a1] = 0;
    return strlen(a0) != a1;
}



// Function: file_must_have_nulls @ 0x71a6
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4337d0;

unsigned int file_must_have_nulls(unsigned long a0, int a1, struct_0 *a2)
{
    int *err;  // rax
    unsigned long v0;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    if (g_4337d0 != 1 && (char)sub_406e8a(a2) && a0 < *((long long *)&a2[1].padding_0[20]))
    {
        v0 = a0;
        if (!a1)
        {
            v0 = lseek(a1, 0, 1);
            if (v0 - 0 >> 63 & 1)
                return 0;
        }
        v1 = lseek(a1, v0, 4);
        if (!(v1 - 0 >> 63 & 1))
        {
            if (lseek(a1, v0, 0) < 0)
            {
                err = __errno_location();
                sub_406edf(*(err));
            }
            if (v1 < *((long long *)&a2[1].padding_0[20]))
                return 1;
        }
    }
    return 0;
}



// Function: context_length_arg @ 0x728f
extern long long g_4290d2;

long long context_length_arg(long long a0, long long *a1)
{
    if (sub_427281(a0, 0, 10, a1, &g_4290d2) <= 1 && *(a1) >= 0)
        return *(a1);
    gettext("invalid context length argument");
    error(2, 0, "%s: %s");
}



// Function: exclude_options @ 0x7316
unsigned int exclude_options(char a0)
{
    return (!a0 ? 0x50000000 : 0x10000000);
}



// Function: skipped_file @ 0x7337
extern unsigned int g_433010;
extern unsigned long long g_4337a0;
extern unsigned long long g_4337b0;
extern char g_4337c8;

unsigned int skipped_file(long long a0, char a1, char a2)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x10]

    if (a2 != 1)
    {
        v0 = &g_4337a0;
    }
    else
    {
        if (g_433010 == 4)
            return 1;
        if (a1 && g_4337c8)
            return 0;
        v0 = &g_4337b0;
    }
    if (*((long long *)(v0 + a1 * 8)) && (char)sub_41cbb6(*((long long *)(v0 + a1 * 8)), a0, a0))
        v2 = 1;
    else
        v2 = 0;
    return v2 & 1;
}



// Function: add_count @ 0x73f5
unsigned long long add_count(unsigned long a0, unsigned long a1)
{
    char v2;  // cl
    unsigned long long v0;  // [bp-0x18]

    v2 = 0;
    if (__OFADD__(a1, a0) & 1)
        v2 = 1;
    v0 = a1 + a0;
    if (!(v2 & 1))
        return v0;
    error(2, 0, gettext("input is too large to count"));
}



// Function: all_zeros @ 0x747d
unsigned int all_zeros(char *a0, unsigned long a1)
{
    char *i;  // [bp-0x10]

    for (i = a0; i < &a0[a1]; i += 1)
    {
        if (*(i))
            return 0;
    }
    return 1;
}



// Function: reset @ 0x74c6
extern char g_433183;
extern char g_4337d0;
extern char g_4337d1;
extern unsigned long long g_4337f0;
extern unsigned int g_433800;
extern void* g_433808;
extern unsigned long long g_433810;
extern unsigned long long g_433818;
extern unsigned long long g_433820;

unsigned int reset(unsigned int a0, unsigned long long a1)
{
    int *err;  // rax
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    g_433810 = (!((g_4337f0 + 1) % g_433818) ? g_4337f0 + 1 : g_433818 - (g_4337f0 + 1) % g_433818 + 1 + g_4337f0);
    g_433808 = g_433810;
    *((char *)g_433808 - 1) = g_433183;
    g_433800 = a0;
    g_433820 = (!a0 ? lseek(a0, 0, 1) : 0);
    g_4337d0 = g_433820 >> 63;
    g_4337d1 = g_4337d0;
    if (g_4337d0)
    {
        if (*(__errno_location()) != 29)
        {
            err = __errno_location();
            sub_406edf(*(err));
            return 0;
        }
        g_433820 = 0;
    }
    return 1;
}



// Function: fillbuf @ 0x7608
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4337d1;
extern unsigned long long g_4337f0;
extern unsigned long long g_4337f8;
extern void g_433800;
extern unsigned long long g_433808;
extern unsigned long long g_433810;
extern unsigned long long g_433818;
extern unsigned long long g_433820;
extern char g_433830;
extern unsigned int g_433838;

long long fillbuf(unsigned long a0, struct_0 *a1)
{
    unsigned long long v14;  // rdx
    char v15;  // cl
    long long v16;  // rdx
    char v0;  // [bp-0x71]
    long long v1;  // [bp-0x70]
    unsigned long v2;  // [bp-0x68]
    unsigned long v3;  // [bp-0x60]
    long long v4;  // [bp-0x58]
    unsigned long long v5;  // [bp-0x50]
    unsigned long v6;  // [bp-0x48]
    unsigned long v7;  // [bp-0x40]
    unsigned long long v8;  // [bp-0x38]
    long long v9;  // [bp-0x30]
    unsigned long long v10;  // [bp-0x28]
    unsigned long n;  // [bp-0x20]
    long long v12;  // [bp-0x18], Other Possible Types: unsigned long long

    v7 = g_433818 + 8;
    if (v7 <= g_4337f8 + g_4337f0 - g_433810)
    {
        v2 = g_433810;
    }
    else
    {
        v8 = a0 + g_433818;
        v9 = v7 + v8 - g_4337f8;
        if (v9 <= 0)
        {
            v3 = g_4337f0;
        }
        else
        {
            v4 = -0x1;
            if ((char)sub_406e8a(a1))
            {
                v10 = *((long long *)&a1[1].padding_0[20]) - g_433820;
                if (!(v10 - 0 >> 63 & 1))
                {
                    v14 = a0 + v7;
                    v15 = 0;
                    if (__OFADD__(v10, v14) & 1)
                        v15 = 1;
                    v1 = v10 + v14;
                    if ((v15 & 1) != 1)
                    {
                        v16 = g_4337f8 + v9;
                        if (v1 > v16)
                            v16 = v1;
                        v4 = v16;
                    }
                }
            }
            v3 = sub_426dc6(0, &g_4337f8, v9, v4, 1);
        }
        v2 = (!((v3 + a0 + 1) % g_433818) ? v3 + a0 + 1 : v3 + a0 + g_433818 - (v3 + a0 + 1) % g_433818 + 1);
        n = a0 + 1;
        memmove(-(n) + v2, g_433810 + -(n), n);
        if (v9 > 0)
        {
            free(g_4337f0);
            g_4337f0 = v3;
        }
    }
    g_433808 = v2 + -(a0);
    sub_406b00();
    v12 = g_4337f8 - 8 + g_4337f0 - v2;
    v12 -= v12 % g_433818;
    v0 = 1;
    while (1)
    {
        do
        {
            v5 = sub_424c3e(*((int *)&g_433800), v2, v12, v2);
            if (v5 == 0xffffffffffffffff)
            {
                v5 = 0;
                v0 = 0;
            }
            g_433820 = v5 + g_433820;
            if (g_433830 != 1 || !v5 || (char)sub_40747d(v2, v5) != 1)
            {
                g_433810 = v2 + v5;
                memset(g_433810, 0, 8);
                sub_406b0b(g_433810 + 8, g_4337f8 - (g_433810 - g_4337f0) - 8);
                return v0;
            }
            *((unsigned long long *)&g_433838) = sub_4073f5(*((long long *)&g_433838), v5);
        } while (g_4337d1 == 1);
        v6 = lseek(*((int *)&g_433800), g_433820, 3);
        if (v6 - 0 >> 63 & 1 && *(__errno_location()) == 6 && (char)sub_406e8a(a1) && *((long long *)&a1[1].padding_0[20]) > g_433820)
            v6 = lseek(*((int *)&g_433800), 0, 2);
        if (v6 - 0 >> 63 & 1)
        {
            g_4337d1 = 1;
        }
        else
        {
            *((unsigned long long *)&g_433838) = sub_4073f5(*((long long *)&g_433838), v6 - g_433820);
            g_433820 = v6;
        }
    }
}



// Function: nlscan @ 0x7a8d
extern char g_433183;
extern unsigned int g_433838;
extern unsigned long long g_433890;

void* nlscan(unsigned long a0)
{
    unsigned long long v0;  // [bp-0x18]
    void* ptr;  // [bp-0x10], Other Possible Types: unsigned long

    v0 = 0;
    for (ptr = g_433890; ptr < a0 && (ptr = memchr(ptr, (int)g_433183, a0 - ptr), ptr); ptr += 1)
    {
        v0 += 1;
    }
    *((unsigned long long *)&g_433838) = sub_4073f5(*((long long *)&g_433838), v0);
    g_433890 = a0;
    return a0;
}



// Function: print_filename @ 0x7b22
extern char *g_433038;

long long print_filename(void)
{
    sub_406b7f(g_433038);
    sub_406c53(sub_406eac());
    return sub_406ba8(g_433038);
}



// Function: print_sep @ 0x7b58
extern char *g_433050;

long long print_sep(char a0)
{
    sub_406b7f(g_433050);
    sub_406c26(a0);
    return sub_406ba8(g_433050);
}



// Function: print_offset @ 0x7b95
extern unsigned int g_433760;

long long print_offset(unsigned long long a0, char *a1)
{
    unsigned long long v1;  // rcx
    unsigned long long v2;  // r8
    unsigned long long v3;  // r9

    sub_406b7f(a1);
    sub_406c8d("%*ld", g_433760, a0, v1, v2, v3);
    return sub_406ba8(a1);
}



// Function: print_line_head @ 0x7be4
extern char *g_433040;
extern char *g_433048;
extern char g_43375d;
extern char g_4337ca;
extern unsigned long long g_433808;
extern void g_433838;
extern unsigned int g_433840;
extern unsigned int g_433848;
extern unsigned int g_43384c;
extern char g_433852;
extern char g_433853;
extern unsigned int g_433888;
extern unsigned long long g_433890;

unsigned int print_line_head(void* a0, unsigned long long a1, unsigned long a2, char a3)
{
    char v0;  // [bp-0x12]
    char v1;  // [bp-0x11]
    unsigned long long v2;  // [bp-0x10]

    if (g_433840 != 1)
    {
        v0 = *((char *)a0 + a1);
        v1 = sub_407098(a0, a1);
        *(a1 + (char *)a0) = v0;
        if (v1)
        {
            g_4337ca = 1;
            return 0;
        }
    }
    if (g_433848)
    {
        sub_407b22();
        if (g_43384c)
            sub_407b58(a3);
        else
            sub_406c26(0);
    }
    if (g_433852)
    {
        if (a2 > g_433890)
        {
            sub_407a8d(a0);
            *((unsigned long long *)&g_433838) = sub_4073f5(*((long long *)&g_433838), 1);
            g_433890 = a2;
        }
        sub_407b95(*((long long *)&g_433838), g_433040);
        sub_407b58(a3);
    }
    if (g_433853)
    {
        v2 = sub_4073f5(*((long long *)&g_433888), a0 - g_433808);
        sub_407b95(v2, g_433048);
        sub_407b58(a3);
    }
    if (g_43375d && (g_433853 || g_433852 || g_433848) && a1)
        sub_406c26(9);
    return 1;
}



// Function: print_line_middle @ 0x7d91
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_433183;
extern char g_43375c;
extern struct_0 *g_4337d8;
extern unsigned long long g_4337e0;
extern char g_433851;

void* print_line_middle(void* a0, void* a1, char *a2, char *a3)
{
    char v0;  // [bp-0x39]
    unsigned long v1;  // [bp-0x38]
    void* i;  // [bp-0x30]
    void* v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    void* v5;  // [bp-0x18]

    v3 = NULL;
    for (i = a0; i < a1 && (v4 = (unsigned long long)g_4337d8(g_4337e0, a0, a1 - a0, &v1, i, g_4337d8), !(v4 - 0 >> 63 & 1) && (v5 = a0 + v4, v5 != a1)); i = v5 + v1)
    {
        if (!v1)
        {
            v1 = 1;
            if (!v3)
                v3 = i;
        }
        else
        {
            if (g_43375c)
            {
                v0 = (!g_433851 ? 58 : 45);
                if ((char)sub_407be4(v5, v1, a1, v0) != 1)
                    return NULL;
            }
            else
            {
                sub_406b1e(a2);
                if (v3)
                {
                    i = v3;
                    v3 = NULL;
                }
                sub_406d74(i, 1, v5 - i);
            }
            sub_406b7f(a3);
            sub_406d74(v5, 1, v1);
            sub_406ba8(a3);
            if (g_43375c)
                sub_406c26(g_433183);
        }
    }
    if (g_43375c)
    {
        i = a1;
    }
    else if (v3)
    {
        i = v3;
    }
    return i;
}



// Function: print_line_tail @ 0x7f87
extern char g_433183;

void* print_line_tail(void* a0, void* a1, char *a2)
{
    int v4;  // eax
    int v5;  // eax
    void* v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18], Other Possible Types: unsigned long long
    long long v2;  // [bp-0x10]

    v0 = a0;
    if (a1 > v0 && *((char *)a1 - 1) == g_433183)
        v4 = 1;
    else
        v4 = 0;
    v1 = v4;
    if (v0 < a1 + !v1 && *((char *)a1 + ~(v1)) == 13)
        v5 = 1;
    else
        v5 = 0;
    v1 += v5;
    v2 = a1 + -(v1) - v0;
    if (v2 <= 0)
        return v0;
    sub_406b1e(a2);
    sub_406d74(v0, 1, v2);
    v0 += v2;
    sub_406b52(a2);
    return v0;
}



// Function: prline @ 0x806d
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 *g_433028;
extern struct_0 *g_433030;
extern struct_0 *g_433058;
extern struct_0 *g_433060;
extern int g_433758;
extern char g_43375c;
extern int g_433798;
extern char g_433851;
extern char g_433878;
extern unsigned long long g_433898;

void* prline(void* a0, void* a1, char a2)
{
    void* v5;  // rax
    char v6;  // al
    char *v7;  // rax
    void* v0;  // [bp-0x30]
    char v1;  // [bp-0x19]
    char *v2;  // [bp-0x18]
    char *v3;  // [bp-0x10]

    v0 = a0;
    if (g_43375c != 1)
    {
        v5 = sub_407be4(v0, a1 - v0 - 1, a1, a2) ^ 1;
        if (*((char *)&v5))
            return v5;
    }
    v1 = g_433851 ^ a2 == 58;
    if (g_433758)
    {
        if (g_433851 && g_433758 < 0)
            v6 = 1;
        else
            v6 = 0;
        v2 = (!(v6 & 1 ^ a2 == 58) ? g_433060 : g_433058);
        v3 = (a2 == 58 ? g_433028 : g_433030);
    }
    else
    {
        v3 = NULL;
        v2 = v3;
    }
    if (g_43375c && v1 || g_433758 && (*(v2) || *(v3)))
    {
        if (v1 && (g_43375c || *(v3)))
        {
            v0 = sub_407d91(v0, a1, v2, v3);
            if (!v0)
                return v0;
        }
        if (g_43375c != 1 && *(v2))
            v0 = sub_407f87(v0, a1, v2);
    }
    if (g_43375c != 1 && a1 > v0)
        sub_406d74(v0, 1, a1 - v0);
    if (g_433878)
        sub_406dc0();
    if (!g_433798)
    {
        g_433898 = a1;
        return a1;
    }
    v7 = gettext("write error");
    error(2, g_433798, v7);
}



// Function: prpending @ 0x828e
extern char g_433183;
extern unsigned long long g_433808;
extern void* g_433898;
extern long long g_4338a8;

long long prpending(void* a0)
{
    void* ptr;  // [bp-0x10]

    if (!g_433898)
        g_433898 = g_433808;
    for (; g_4338a8 > 0; g_4338a8 = g_4338a8 - 1)
    {
        if (a0 > g_433898)
        {
            ptr = rawmemchr(g_433898, g_433183);
            sub_40806d(g_433898, ptr + 1, 45);
        }
        else
        {
            return g_433898;
        }
    }
    return g_4338a8;
}



// Function: prtext @ 0x8326
extern char *g_433020;
extern char *g_433050;
extern char g_433183;
extern unsigned long long g_433808;
extern unsigned long long g_433810;
extern unsigned long long g_433820;
extern unsigned long long g_433828;
extern char g_433850;
extern char g_433851;
extern char g_433858;
extern char g_433860;
extern char g_433898;
extern unsigned long long g_4338a0;
extern long long g_4338a8;
extern char g_4339c0;

long long prtext(void* a0, void* a1)
{
    long long v8;  // rax
    char l;  // [bp-0x39]
    void* j;  // [bp-0x38]
    long long i;  // [bp-0x30]
    long long k;  // [bp-0x28]
    void* v4;  // [bp-0x20]
    void* ptr;  // [bp-0x18]
    void* ptr1;  // [bp-0x10]

    l = g_433183;
    if (g_433850 != 1 && g_4338a8 > 0)
        sub_40828e(a0);
    j = a0;
    if (g_433850 != 1)
    {
        v4 = (!*((long long *)&g_433898) ? g_433808 : *((long long *)&g_433898));
        for (i = 0; i < *((long long *)&g_433858); i += 1)
        {
            if (j > v4)
            {
                do
                {
                    j -= 1;
                } while (l != *((char *)j - 1));
            }
        }
        if ((*((long long *)&g_433858) >= 0 || *((long long *)&g_433860) >= 0) && g_4339c0 && j != *((long long *)&g_433898) && g_433020)
        {
            sub_406b7f(g_433050);
            sub_406c53(g_433020);
            sub_406ba8(g_433050);
            sub_406c26(10);
        }
        for (; j < a0; j = ptr)
        {
            ptr = rawmemchr(j, l);
            ptr += 1;
            sub_40806d(j, ptr, 45);
        }
    }
    if (g_433851)
    {
        for (k = 0; j < a1 && k < g_4338a0; k += 1)
        {
            ptr1 = rawmemchr(j, l);
            ptr1 += 1;
            if (g_433850 != 1)
                sub_40806d(j, ptr1, 58);
            j = ptr1;
        }
    }
    else
    {
        if (g_433850 != 1)
            sub_40806d(a0, a1, 58);
        k = 1;
        j = a1;
    }
    g_433828 = g_433820 - (g_433810 - j);
    if (g_433850)
    {
        v8 = 0;
    }
    else
    {
        v8 = *((long long *)&g_433860);
        if (*((long long *)&g_433860) < 0)
            v8 = 0;
    }
    g_4338a8 = v8;
    g_4339c0 = 1;
    g_4338a0 = g_4338a0 - k;
    return g_4338a0;
}



// Function: zap_nuls @ 0x85c1
char * zap_nuls(char *a0, char *a1, unsigned int a2)
{
    char v0;  // [bp-0x1c]
    char *iter;  // [bp-0x10]

    iter = a0;
    v0 = a2;
    if (!v0)
        return a2;
    while (1)
    {
        *(a1) = 0;
        iter = &iter[strlen(iter)];
        *(a1) = v0;
        if (iter == a1)
            break;
        do
        {
            iter += 1;
            *(iter) = v0;
        } while (!*(iter));
    }
    return iter;
}



// Function: grepbuf @ 0x862e
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned int g_433080;
extern char g_4337c9;
extern struct_0 *g_4337d8;
extern unsigned long long g_4337e0;
extern char g_433851;
extern unsigned long long g_4338a0;
extern char g_4338b0;
extern char g_4338b1;

long long grepbuf(void* a0, void* a1)
{
    unsigned long v0;  // [bp-0x50]
    void* v1;  // [bp-0x48]
    unsigned long long v2;  // [bp-0x40]
    unsigned long long v3;  // [bp-0x38]
    void* v4;  // [bp-0x30]
    void* v5;  // [bp-0x28]
    void* v6;  // [bp-0x20]
    void* v7;  // [bp-0x18]

    v3 = g_4338a0;
    v1 = a0;
    while (1)
    {
        if (v1 >= a1)
            return v3 - g_4338a0;
        v2 = g_4337d8(g_4337e0, v1, a1 - v1, &v0, 0, g_4337d8);
        if (v2 - 0 >> 63 & 1)
        {
            if (g_433851 == 1)
            {
                v2 = a1 - v1;
                v0 = 0;
            }
            else
            {
                return v3 - g_4338a0;
            }
        }
        v4 = v1 + v2;
        v5 = v4 + v0;
        if (g_433851 != 1 && v4 == a1)
            return v3 - g_4338a0;
        if (g_433851 != 1 || v1 < v4)
        {
            if (!g_433851)
                v1 = v4;
            v6 = v1;
            if (!g_433851)
                v4 = v5;
            v7 = v4;
            sub_408326(v6, v7);
            if (!g_4338a0 || g_4338b0)
                break;
        }
        v1 = v5;
    }
    if (!g_4338b1)
        return v3 - g_4338a0;
    exit((!g_4337c9 ? 0 : g_433080)); /* do not return */
}



// Function: grep @ 0x87e9
typedef struct struct_1 {
    char padding_0[48];
    unsigned long long field_30;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_433183;
extern char g_43375d;
extern unsigned int g_433760;
extern char g_4337ca;
extern void* g_433808;
extern struct_0 *g_433810;
extern unsigned long long g_433828;
extern char g_433830;
extern char g_433831;
extern unsigned long long g_433838;
extern unsigned int g_433840;
extern char g_433850;
extern char g_433852;
extern char g_433853;
extern char g_433858;
extern char g_433868;
extern unsigned long long g_433870;
extern unsigned int g_433888;
extern unsigned long long g_433890;
extern unsigned long long g_433898;
extern unsigned long long g_4338a0;
extern unsigned long long g_4338a8;
extern char g_4338b0;

long long grep(unsigned int a0, struct_1 *a1, char *a2)
{
    char v15;  // al
    int *err;  // rax
    int v18;  // eax
    unsigned long v19;  // rax
    char *v20;  // rax
    int *err1;  // rax
    char v0;  // [bp-0x5e]
    char v1;  // [bp-0x5d]
    char j;  // [bp-0x5c]
    char v3;  // [bp-0x5b]
    char v4;  // [bp-0x5a]
    char v5;  // [bp-0x59]
    unsigned long node;  // [bp-0x58]
    long long v7;  // [bp-0x50]
    unsigned long long v8;  // [bp-0x48]
    unsigned long long v9;  // [bp-0x40]
    void* iter;  // [bp-0x38]
    void* v11;  // [bp-0x30]
    unsigned long v12;  // [bp-0x28]
    uint128_t i;  // [bp-0x20], Other Possible Types: unsigned long long

    j = g_433183;
    v0 = 0;
    v3 = g_4338b0;
    v4 = g_433850;
    v12 = 0xffffffffffffffff;
    if ((char)sub_4074c6(a0, a1) != 1)
        return 0;
    *((unsigned long *)&g_433888) = 0;
    g_433898 = 0;
    g_433838 = 0;
    g_4338a0 = g_433870;
    g_433828 = 0;
    g_4338a8 = 0;
    if (g_433831 && !j)
        v15 = 1;
    else
        v15 = 0;
    g_433830 = v15 & 1;
    g_4337ca = 0;
    node = 0;
    v8 = 0;
    v9 = 0;
    if ((char)sub_407608(v9, a1) != 1)
    {
        err = __errno_location();
        sub_406edf(*(err));
        return 0;
    }
    g_433760 = 0;
    if (g_43375d)
    {
        i = (!(char)sub_406e8a(a1) ? 0x7fffffffffffffff : a1->field_30);
        if (g_433852 && i != 0x7fffffffffffffff)
            v18 = 1;
        else
            v18 = 0;
        i += v18;
        do
        {
            g_433760 = g_433760 + 1;
            i = (unsigned long long)((int128_t)(i * 7378697629483820647) >> 66) - ((int128_t)(i) >> 63);
        } while (i);
    }
    v1 = 1;
    while (1)
    {
        if (v12 - 0 >> 63 & 1 && j && g_433840 != 1 && (sub_40716c(g_433808, g_433810 - g_433808) || v1 && (char)sub_4071a6(g_433810 - g_433808, a0, a1)))
        {
            if (g_433840 == 2)
                return 0;
            if (g_433868 != 1)
            {
                g_433850 = 1;
                g_4338b0 = g_433850;
            }
            v12 = node;
            v0 = j;
            g_433830 = g_433831;
        }
        g_433890 = g_433808;
        if (g_433898)
            g_433898 = g_433808;
        iter = v9 + g_433808;
        if (iter == g_433810)
            break;
        sub_4085c1(iter, g_433810, v0);
        v5 = *((char *)iter - 1);
        *((char *)iter - 1) = j;
        v11 = memrchr(iter - 1, j, g_433810 - iter + 1);
        v11 += 1;
        *((char *)iter - 1) = v5;
        if (v11 == iter)
            v11 = iter + -(v8);
        iter += -(v8);
        v8 = g_433810 - v11;
        if (iter < v11)
        {
            if (g_4338a0)
                node += sub_40862e(iter, v11);
            if (g_4338a8)
                sub_40828e(v11);
            if (g_4338a0 || g_4338a8)
            {
                if (!g_4338b0)
                    goto LABEL_408c1d;
                if (v12 < 0)
                    v19 = 0;
                else
                    v19 = v12;
                if (node <= v19)
                    goto LABEL_408c1d;
            }
            goto LABEL_408d91;
        }
        else
        {
LABEL_408c1d:
            v7 = 0;
            iter = v11;
            while (v7 < *((long long *)&g_433858) && iter > g_433808 && iter != g_433898)
            {
                v7 += 1;
                do
                {
                    iter -= 1;
                } while (j != *((char *)iter - 1));
            }
            if (iter != g_433898)
                g_433898 = 0;
            v9 = v11 + v8 - iter;
            if (g_433853)
                *((unsigned long long *)&g_433888) = sub_4073f5(*((long long *)&g_433888), g_433810 - g_433808 - v9);
            if (g_433852)
                sub_407a8d(iter);
            if ((char)sub_407608(v9, a1) != 1)
            {
                err1 = __errno_location();
                sub_406edf(*(err1));
                goto LABEL_408d91;
            }
            else
            {
                v1 = 0;
            }
        }
    }
    *(a2) = 1;
    if (v8)
    {
        v20 = g_433810;
        g_433810 = v20 + 1;
        *(v20) = j;
        if (g_4338a0)
            node += sub_40862e(v9 - v8 + g_433808, g_433810);
        if (!g_4338a8)
            goto LABEL_408d8d;
        sub_40828e(g_433810);
    }
    else
    {
LABEL_408d8d:
    }
LABEL_408d91:
    g_4338b0 = v3;
    g_433850 = v4;
    if (!g_433840 && g_433850 != 1 && (g_4337ca || !(v12 - 0 >> 63 & 1) && v12 < node))
    {
        sub_406eac();
        error(0, 0, gettext("%s: binary file matches"));
    }
    return node;
}



// Function: grepdirent @ 0x8e14
typedef struct struct_0 {
    char field_0;
    char padding_1[47];
    long long field_30;
    unsigned long long field_38;
    int field_40;
    char padding_44[20];
    unsigned long long field_58;
    char padding_60[8];
    unsigned short field_68;
    char padding_6a[30];
    unsigned int field_88;
} struct_0;

typedef struct struct_2 {
    char padding_0[44];
    unsigned int field_2c;
    char padding_30[24];
    unsigned int field_48;
} struct_2;

typedef struct struct_1 {
    char padding_0[1];
    char field_1;
} struct_1;

extern unsigned int g_433010;
extern char g_433755;
extern struct_1 *g_4337c0;
extern char g_4337c8;

unsigned int grepdirent(struct_2 *a0, struct_0 *a1, char a2)
{
    char v6;  // al
    char v7;  // al
    int *err;  // rax
    char v0;  // [bp-0xdc]
    char v1;  // [bp-0xc5]
    unsigned int v2;  // [bp-0xc4]
    struct_0 *v3;  // [bp-0xc0]
    char v4;  // [bp-0xb8]

    v0 = a2;
    v0 = !a1->field_58 & v0;
    if (a1->field_68 == 6)
        return 1;
    if (v0 != 1)
    {
        if (a1->field_68 == 1 || a1->field_68 == 2 || a1->field_68 == 4)
            v6 = 1;
        else
            v6 = 0;
        if ((char)sub_407337(&a1[1].padding_6a[10], 0, v6 & 1))
        {
            sub_41e92f(a0, a1, 4, a1);
            return 1;
        }
    }
    g_4337c0 = a1->field_38;
    if (g_4337c8 && g_4337c0->field_1)
        g_4337c0 = g_4337c0 + 1;
    if ((char)a0->field_48 & 2 || (char)a0->field_48 & 1 && v0)
        v7 = 1;
    else
        v7 = 0;
    v1 = v7;
    v1 &= 1;
    switch (a1->field_68)
    {
    case 1:
        if (g_433010 == 3)
            return 1;
        sub_41e92f(a0, a1, 4, a1);
        break;
    case 2:
        if (g_433755 == 1)
            return 1;
        error(0, 0, gettext("%s: warning: recursive directory loop"));
        return 1;
    case 3: case 11:
        if ((char)sub_406e43(v0))
        {
            v3 = &a1->padding_6a[6];
            if (!*((int *)&v3->padding_1[23]))
            {
                v2 = (!v1 ? 0x100 : 0);
                if (fstatat(a0->field_2c, a1->field_30, &v4, v2))
                {
                    err = __errno_location();
                    sub_406edf(*(err));
                    return 1;
                }
                v3 = &v4;
            }
            if ((char)sub_406deb(*((int *)&v3->padding_1[23])))
                return 1;
        }
        break;
    case 4: case 7: case 10:
        sub_406edf(a1->field_40);
        return 1;
    case 8: case 13:
        break;
    case 12: case 14:
        return 1;
    default:
        abort(); /* do not return */
    }
    return sub_4091bc(a0->field_2c, a1->field_30, v1, v0);
}



// Function: open_symlink_nofollow_error @ 0x9197
unsigned int open_symlink_nofollow_error(unsigned int a0)
{
    if (a0 != 40 && a0 != 31)
        return 0;
    return 1;
}



// Function: grepfile @ 0x91bc
unsigned int grepfile(unsigned int a0, long long a1, char a2, char a3)
{
    int *err;  // rax
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]

    v0 = (!(char)sub_406e43(a3) ? 0 : 0x800) | (!a2 ? 0x20100 : 0x100);
    v1 = sub_422af5(a0, a1, v0, a1);
    if (v1 >= 0)
        return sub_409456(v1, a3, a3);
    if (a2 || (char)sub_409197(*(__errno_location())) != 1)
    {
        err = __errno_location();
        sub_406edf(*(err));
    }
    return 1;
}



// Function: drain_input @ 0x9272
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern long long g_4337f0;
extern long long g_4337f8;
extern char g_4338b2;

unsigned int drain_input(unsigned int a0, struct_0 *a1)
{
    unsigned long v2;  // rax
    unsigned long i;  // [bp-0x10], Other Possible Types: unsigned long long

    if (((unsigned short)a1->field_18 & 0xf000) == 0x1000 && g_4338b2)
    {
        if (!(i - 0 >> 63 & 1) || !(v2 = (unsigned long)(unsigned long long)(unsigned int)*(__errno_location()), *(__errno_location()) == 22))
        {
            for (i = splice(a0, 0, 1, 0, 0x18000, 1); i > 0; i = splice(a0, 0, 1, 0, 0x18000, 1));
            return _INSERT(v2, 0, !i);
        }
    }
    while (1)
    {
        i = sub_424c3e(a0, g_4337f0, g_4337f8, g_4337f0);
        if (!i)
        {
            return 1;
        }
        else if (i == 0xffffffffffffffff)
        {
            return 0;
        }
    }
}



// Function: finalize_input @ 0x9361
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

extern char g_4337d0;
extern int g_433820;
extern char g_433828;
extern unsigned long long g_4338a0;

long long finalize_input(unsigned int a0, struct_0 *a1, unsigned int a2)
{
    unsigned int v2;  // eax
    long long v3;  // rax
    unsigned int v4;  // eax
    char v0;  // [bp-0x10]

    v0 = a2;
    if (a0)
        return a2;
    if (g_4338a0)
    {
        if (v0 != 1 && (g_4337d0 || lseek(a0, 0, 2) < 0 && *(__errno_location()) != 22) && (char)sub_409272(a0, a1) != 1)
            v2 = 1;
        else
            v2 = 0;
        v3 = v2 & 1;
    }
    else
    {
        if (*((long long *)&g_433820) != *((long long *)&g_433828) && g_4337d0 != 1 && lseek(a0, *((long long *)&g_433828), 0) < 0)
            v4 = 1;
        else
            v4 = 0;
        v3 = v4 & 1;
    }
    if (!(char)v3)
        return v3;
    return sub_406edf(*(__errno_location()));
}



// Function: grepdesc @ 0x9456
typedef struct struct_0 {
    char field_0;
    char padding_1[47];
    long long field_30;
    unsigned long long field_38;
    int field_40;
    char padding_44[20];
    unsigned long long field_58;
    char padding_60[8];
    unsigned short field_68;
    char padding_6a[30];
    unsigned int field_88;
} struct_0;

extern unsigned int g_433010;
extern unsigned int g_433014;
extern char g_4336c0;
extern char g_4336c8;
extern char g_433755;
extern unsigned long long g_4337c0;
extern char g_4337c9;
extern char g_4337cc;
extern unsigned int g_433844;
extern char g_433848;
extern unsigned int g_43384c;
extern char g_433850;
extern char g_433868;
extern long long g_433870;
extern char g_433878;

long long grepdesc(unsigned int a0, char a1)
{
    int *err;  // rax
    int *err1;  // rax
    int *err2;  // rax
    int *v13;  // rax
    int *v14;  // rax
    unsigned long long v16;  // rdx
    unsigned long long v17;  // rcx
    unsigned long long v18;  // r8
    unsigned long long v19;  // r9
    char v0;  // [bp-0xe6]
    char v1;  // [bp-0xe5]
    unsigned int v2;  // [bp-0xe4]
    unsigned long v3;  // [bp-0xe0]
    struct_0 *v4;  // [bp-0xd8]
    unsigned long v5;  // [bp-0xd0]
    unsigned long v6;  // [bp-0xc8]
    unsigned long long v7;  // [bp-0xc0]
    int v8;  // [bp-0xb8]

    v1 = 1;
    v0 = 0;
    if (fstat(a0, &v8))
    {
        err = __errno_location();
        sub_406edf(*(err));
    }
    else if ((!a0 || !(char)sub_406e43(a1) || !(char)sub_406deb(*((unsigned int *)(&v8 + 24)))) && (!a0 || !a1 || !(char)sub_407337(g_4337c0, 1, (*((unsigned int *)(&v8 + 24)) & 0xf000) == 0x4000)))
    {
        if (*((int *)&g_433848) < 0)
            *((unsigned int *)&g_433848) = (*((unsigned int *)(&v8 + 24)) & 0xf000) == 0x4000;
        if (a0 && g_433010 == 3 && ((unsigned short)*((unsigned int *)(&v8 + 24)) & 0xf000) == 0x4000)
        {
            v2 = g_433014 & ~(a1 ^ 1);
            if (close(a0))
            {
                err2 = __errno_location();
                sub_406edf(*(err2));
            }
            v6 = g_4337c0;
            v7 = 0;
            v3 = sub_41d975(&v6, v2, 0, v2);
            if (!v3)
                sub_427122(); /* do not return */
            while (1)
            {
                v4 = sub_41e2b9(v3);
                if (!v4)
                    break;
                v1 = sub_408e14(v3, v4, a1) & 0xff & v1;
            }
            if (*(__errno_location()))
            {
                v13 = __errno_location();
                sub_406edf(*(v13));
            }
            if (!sub_41ddff(v3))
                return v1;
            v14 = __errno_location();
            sub_406edf(*(v14));
            return v1;
        }
        if (!a0 || (g_433010 != 4 || ((unsigned short)*((unsigned int *)(&v8 + 24)) & 0xf000) != 0x4000) && (*((int *)&g_4337cc) != 2 && (*((int *)&g_4337cc) || a1 == 1) || !(char)sub_406deb(*((unsigned int *)(&v8 + 24)))))
        {
            if (g_433850 != 1 && !g_433844 && g_433870 > 1 && ((unsigned short)*((unsigned int *)(&v8 + 24)) & 0xf000) == 0x8000 && *((unsigned long long *)(&v8 + 8)) == *((long long *)&g_4336c8) && *((unsigned long long *)&v8) == *((long long *)&g_4336c0))
            {
                if (g_433755 != 1)
                {
                    sub_406eac();
                    error(0, 0, gettext("%s: input file is also the output"));
                }
                g_4337c9 = 1;
            }
            else
            {
                v5 = sub_4087e9(a0, &v8, &v0);
                if (g_433868)
                {
                    if (*((int *)&g_433848))
                    {
                        sub_407b22();
                        if (g_43384c)
                            sub_407b58(58);
                        else
                            sub_406c26(0);
                    }
                    sub_406c8d("%ld\n", v5, v16, v17, v18, v19);
                    if (g_433878)
                        sub_406dc0();
                }
                v1 = !v5;
                if (!g_433844)
                {
                    sub_409361(a0, &v8, v0);
                }
                else if ((!v1 ? 1 : 2) == g_433844)
                {
                    sub_407b22();
                    sub_406c26(g_43384c & 10);
                    if (!g_433878)
                        goto LABEL_4098e1;
                    sub_406dc0();
                }
                else
                {
LABEL_4098e1:
                }
            }
        }
    }
    if (a0 && close(a0))
    {
        err1 = __errno_location();
        sub_406edf(*(err1));
    }
    return v1;
}



// Function: grep_command_line_arg @ 0x992b
extern unsigned long long g_4337c0;
extern unsigned long long g_433880;
extern char g_4338b3;

long long grep_command_line_arg(unsigned long a0)
{
    if (strcmp(a0, "-"))
    {
        g_4337c0 = a0;
        return sub_4091bc(0xffffff9c, a0, 1, 1);
    }
    g_4337c0 = g_433880;
    if (!g_4338b3)
        return sub_409456(0, 1);
    sub_42716d(0, 0);
    return sub_409456(0, 1);
}



// Function: usage @ 0x99b6
typedef struct FILE {
} FILE;

extern FILE *stderr;

void usage(int a0)
{
    unsigned long long v2;  // rbx
    char *v3;  // rax
    long long v7;  // rdi
    long long v8;  // rsi
    long long v9;  // rdx
    long long v10;  // rcx
    long long v11;  // r8
    long long v12;  // r9
    char *v4;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    if (a0)
    {
        sub_41ff0c();
        v3 = gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n");
        fprintf(stderr, v3);
        sub_41ff0c(v7, v8, v9, v10, v11, v12);
        v4 = gettext("Try '%s --help' for more information.\n");
        fprintf(stderr, v4);
    }
    else
    {
        sub_41ff0c();
        printf(gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n"));
        printf(gettext("Search for PATTERNS in each FILE.\n"));
        sub_41ff0c(v7, v8, v9, v10, v11, v12);
        printf(gettext("Example: %s -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n"));
        printf(gettext("  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n"));
        printf(gettext("  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n"));
        printf(gettext("\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n"));
        printf(gettext("\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n"));
        printf(gettext("  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is 'binary', 'text', or 'without-match'\n  -a, --text                equivalent to --binary-files=text\n"));
        printf(gettext("  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is 'read', 'recurse', or 'skip'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is 'read' or 'skip'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n"));
        printf(gettext("      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n"));
        printf(gettext("  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n"));
        printf(gettext("\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n"));
        printf(gettext("  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is 'always', 'never', or 'auto'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n"));
        printf(gettext("When FILE is '-', read standard input.  With no FILE, read '.' if\nrecursive, '-' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n"));
        sub_4269ee(v7, v8, v9, v10, v11, v12);
    }
    exit(a0); /* do not return */
}



// Function: setmatcher @ 0x9be6
int setmatcher(char *a0, int a1)
{
    int v0;  // [bp-0xc]

    v0 = 0;
    while (1)
    {
        if (v0 > 6)
            error(2, 0, gettext("invalid matcher %s"));
        if (!strcmp(a0, (char *)(32 * v0 + 4401056)))
            break;
        v0 += 1;
    }
    if (a1 < 0)
    {
        return v0;
    }
    else if (a1 != v0)
    {
        error(2, 0, gettext("conflicting matchers specified"));
    }
    else
    {
        return v0;
    }
}



// Function: get_nondigit_option @ 0x9c9f
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern option g_4320a0;
extern unsigned int g_433018;
extern unsigned int optind;

long long get_nondigit_option(int a0, char **a1, long long *a2)
{
    char v0;  // [bp-0x49]
    unsigned int v1;  // [bp-0x48]
    unsigned int v2;  // [bp-0x44]
    char *v3;  // [bp-0x40]
    char *ptr;  // [bp-0x40]
    char v5;  // [bp-0x38]
    char v6;  // [bp-0x23]

    v3 = &v5;
    v0 = 0;
    v1 = optind;
    while (1)
    {
        v2 = getopt_long(a0, a1, "0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz", &g_4320a0.name, NULL);
        if ((char)sub_410eab(v2) != 1)
            break;
        if (v1 != g_433018 || v0 != 1)
            ptr = &v5;
        else
            ptr = &v3[-(v5 == 48)];
        if (ptr == &v6)
        {
            strcpy(ptr, "...");
            v3 = ptr + 3;
            break;
        }
        else
        {
            v3 = ptr + 1;
            *(ptr) = v2;
            v0 = 1;
            g_433018 = v1;
            v1 = optind;
        }
    }
    if (v3 != &v5)
    {
        *(v3) = 0;
        sub_40728f(&v5, a2);
    }
    return v2;
}



// Function: parse_grep_colors @ 0x9ddf
typedef struct struct_0 {
    char *field_0;
    struct struct_1 *field_8;
    void* field_10;
} struct_0;

typedef struct struct_1 {
    void* field_0;
} struct_1;

extern struct_0 g_431f80;

void* parse_grep_colors(void)
{
    void* v7;  // rax
    void* v8;  // rax
    void* v9;  // rax
    void* v10;  // rax
    unsigned long v11;  // rax
    char v0;  // [bp-0x31]
    void* iter;  // [bp-0x30]
    void* v2;  // [bp-0x28]
    void* v3;  // [bp-0x20]
    struct_0 *node;  // [bp-0x18]
    void* v5;  // [bp-0x10]

    v7 = getenv("GREP_COLORS");
    v5 = v7;
    if (v5)
    {
        v7 = *((char *)v5);
        if (*((char *)&v7))
        {
            iter = sub_4270f1(v5);
            v2 = iter;
            v3 = NULL;
            while (1)
            {
                if (*((char *)iter) == 58 || !*((char *)iter))
                {
                    v0 = *((char *)iter);
                    v10 = iter;
                    iter = v10 + 1;
                    *((char *)v10) = 0;
                    for (node = &g_431f80.field_0; node->field_0 && strcmp(node->field_0, v2); node += 1);
                    if (node->field_8 && v3)
                        node->field_8->field_0 = v3;
                    v11 = node->field_10;
                    if (node->field_10)
                        v11 = node->field_10();
                    if (!v0)
                        return v11;
                    v2 = iter;
                    v3 = NULL;
                }
                else if (*((char *)iter) == 61)
                {
                    if (iter == v2 || v3)
                        break;
                    v8 = iter;
                    iter = v8 + 1;
                    *((char *)v8) = 0;
                    v3 = iter;
                    continue;
                }
                else if (!v3)
                {
                    iter += 1;
                    continue;
                }
                else
                {
                    if (*((char *)iter) != 59)
                    {
                        v9 = sub_410eab(*((char *)iter));
                        if (!*((char *)&v9))
                            return v9;
                    }
                    iter += 1;
                }
            }
            return iter;
        }
    }
    return v7;
}



// Function: contains_encoding_error @ 0x9f7b
unsigned int contains_encoding_error(unsigned long a0, long long a1)
{
    long long i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]

    v2 = 0;
    for (i = 0; i < a1; i += v1)
    {
        v1 = sub_40f6da(i + a0, a1 - i, &v2, i + a0);
        if (v1 - 0 >> 63 & 1)
            return 1;
    }
    return 0;
}



// Function: setup_ok_fold @ 0xa011
typedef struct mbstate_t {
} mbstate_t;

extern unsigned int g_4331a4[4];
extern char g_4338c0;

void setup_ok_fold(void)
{
    int i;  // [bp-0xc0]
    unsigned int v1;  // [bp-0xbc]
    unsigned int v2;  // [bp-0xb8]
    unsigned int *v3;  // [bp-0xb4]
    mbstate_t v4;  // [bp-0xb0]
    char v5;  // [bp-0xa8]
    char v6[24];  // [bp-0x28]

    for (i = 0; i <= 0xff; i += 1)
    {
        v3 = (unsigned int *)g_4331a4[64 + i];
        if (v3 != 0xffffffff)
        {
            v1 = 1;
            v2 = sub_41c01a(v3, &v5, &v5);
            do
            {
                v2 -= 1;
                if (v2 < 0)
                    goto LABEL_40a0e9;
            } while ((v4 = (mbstate_t)0, wcrtomb(v6, (short)*((int *)&(&v5)[4 * v2]), &v4) == 1));
            v1 = 0xffffffff;
LABEL_40a0e9:
            *(&(&g_4338c0)[i]) = v1;
        }
    }
    return;
}



// Function: fgrep_icase_charlen @ 0xa134
extern unsigned int g_4331a4[4];
extern char g_4338c0;

long long fgrep_icase_charlen(char *a0, long long a1, long long a2)
{
    char v0;  // [bp-0xae]
    char v1;  // [bp-0xad]
    char v2;  // [bp-0xac], Other Possible Types: unsigned int
    long long v3;  // [bp-0xa8]
    long long v4;  // [bp-0xa0]
    char v5;  // [bp-0x98]

    v0 = *(a0);
    if (g_4331a4[v0 + 64] != 0xffffffff)
        return *((char *)(unsigned int)&(&g_4338c0)[v0]);
    v4 = sub_421b80(&v2, a0, a1, a2);
    if (v4 > 16)
    {
        return -0x1;
    }
    else if (sub_41c01a(v2, &v5, v2))
    {
        return -0x1;
    }
    else
    {
        v3 = v4;
        while (1)
        {
            v3 -= 1;
            if (v3 <= 0)
                return v4;
            v1 = a0[v3];
            if ((int)toupper(v1) != v1)
                return -0x1;
        }
    }
}



// Function: fgrep_icase_available @ 0xa29a
unsigned int fgrep_icase_available(unsigned long a0, long long a1)
{
    unsigned int v0;  // [bp-0x24]
    long long i;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]

    v2 = 0;
    for (i = 0; i < a1; i += v0)
    {
        v0 = sub_40a134(i + a0, a1 - i, &v2);
        if (v0 < 0)
            return 0;
    }
    return 1;
}



// Function: fgrep_to_grep_pattern @ 0xa32f
typedef struct struct_0 {
    char field_0;
} struct_0;

void fgrep_to_grep_pattern(struct_0 **a0, void* a1)
{
    unsigned long v0;  // [bp-0x40]
    char *v1;  // [bp-0x38]
    char *v2;  // [bp-0x30]
    unsigned long v4;  // [bp-0x28]
    char *v5;  // [bp-0x20]
    unsigned long v6;  // [bp-0x18]

    v0 = *((long long *)a1);
    v1 = &*(a0)->field_0;
    v6 = 0;
    v5 = sub_426c63(v0 + 1, 2);
    v2 = v5;
    if (!v0)
    {
        *(v2) = 10;
        free(*(a0));
        *(a0) = v5;
        *((unsigned long *)a1) = v2 - v5;
        return;
    }
    v4 = sub_40f6da(v1, v0, &v6, v0);
}



// Function: try_fgrep_pattern @ 0xa4d3
extern char g_433180;

long long try_fgrep_pattern(unsigned int a0, char *a1, long long *a2)
{
    int v9;  // eax
    unsigned long v10;  // rdx
    int v11;  // eax
    unsigned long v12;  // rdx
    unsigned long v13;  // rdx
    unsigned int v0;  // [bp-0x4c]
    long long iter;  // [bp-0x48]
    void* v2;  // [bp-0x40]
    char *node;  // [bp-0x38]
    void* ptr;  // [bp-0x30]
    char *v5;  // [bp-0x28]
    unsigned long v6;  // [bp-0x20]
    unsigned long v7;  // [bp-0x18]

    v0 = a0;
    iter = *(a2);
    ptr = sub_426b10(iter + 1);
    v2 = ptr;
    node = a1;
    v7 = 0;
    while (1)
    {
        if (!iter)
        {
            if (*(a2) != v2 - ptr)
            {
                *(a2) = v2 - ptr;
                v5 = mempcpy(a1, ptr, v2 - ptr);
                *(v5) = 10;
            }
            v0 = 2;
            free(ptr);
            break;
        }
        v9 = *(node);
        if (v9 <= 124)
        {
            if (v9 >= 123)
            {
LABEL_40a5e5:
                if (a0)
                {
                    free(ptr);
                    return v0;
                }
            }
            else if (v9 == 94)
            {
LABEL_40a7a2:
                free(ptr);
                return v0;
            }
            else if (v9 <= 94)
            {
                if (v9 != 92)
                {
                    if (v9 <= 92)
                    {
                        if (v9 == 91)
                            goto LABEL_40a7a2;
                        if (v9 <= 91)
                        {
                            if (v9 <= 46)
                            {
                                if (v9 >= 36)
                                {
                                    v10 = 1 << ((char)v9 & 63);
                                    if (0x441000000000 & v10)
                                        goto LABEL_40a7a2;
                                    if (0x90000000000 & v10)
                                        goto LABEL_40a5e5;
                                }
                            }
                            else
                            {
                                if (v9 == 63)
                                    goto LABEL_40a5e5;
                            }
                        }
                    }
                }
                else
                {
                    if (iter > 1)
                    {
                        v11 = node[1];
                        if (v11 <= 60)
                        {
                            if (v11 >= 10)
                            {
                                v12 = 1 << ((char)v11 & 63);
                                if (0x13fe008000000400 & v12)
                                {
LABEL_40a7a5:
                                    free(ptr);
                                    return v0;
                                }
                                if (0xb0000000000 & v12)
                                {
LABEL_40a6af:
                                    if (!a0)
                                    {
                                        free(ptr);
                                        return v0;
                                    }
                                }
                            }
                        }
                        else
                        {
                            if (v11 <= 124 && v11 >= 62)
                            {
                                v13 = 1 << ((char)(v11 - 62) & 63);
                                if (153122473265594385 & v13)
                                    goto LABEL_40a7a5;
                                if (6917529027641081858 & v13)
                                    goto LABEL_40a6af;
                            }
                        }
                        node += 1;
                        iter -= 1;
                        goto LABEL_40a6c9;
                    }
                    else
                    {
LABEL_40a6c9:
                    }
                }
            }
        }
        v6 = (!g_433180 ? sub_40f6da(node, iter, &v7, iter) : sub_40a134(node, iter, &v7));
        if (v6 - 0 >> 63 & 1)
        {
            free(ptr);
            return v0;
        }
        v2 = mempcpy(v2, node, v6);
        node = &node[v6];
        iter -= v6;
    }
    return v0;
}



// Function: main @ 0xa7d1
typedef struct struct_0 {
    char field_0;
    char field_1;
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
    char padding_0[1];
    char field_1;
} struct_1;

extern char g_4290d2;
extern unsigned int g_4293a8[4];
extern long long g_432700;
extern void g_432720;
extern void g_432730;
extern unsigned int g_4327ac[4];
extern unsigned long long g_4327b0[4];
extern unsigned long long g_4327b8[4];
extern unsigned int g_433010;
extern unsigned int g_433014;
extern unsigned long long g_433020;
extern unsigned long long g_433028;
extern unsigned long long g_433030;
extern long long g_433078;
extern unsigned int g_433080;
extern long long stdout;
extern unsigned long long stdin;
extern int optind;
extern char optarg;
extern char g_433180;
extern char g_433181;
extern char g_433182;
extern char g_433183;
extern char g_4331a0;
extern unsigned long long g_4336c0;
extern unsigned long long g_4336c8;
extern unsigned long long g_4336d0;
extern unsigned long long g_4336d8;
extern unsigned long long g_4336e0;
extern unsigned long long g_4336e8;
extern unsigned long long g_4336f0;
extern unsigned long long g_4336f8;
extern unsigned long long g_433700;
extern unsigned long long g_433708;
extern unsigned long long g_433710;
extern unsigned long long g_433718;
extern unsigned long long g_433720;
extern unsigned long long g_433728;
extern unsigned long long g_433730;
extern unsigned long long g_433738;
extern unsigned long long g_433740;
extern unsigned long long g_433748;
extern unsigned int g_433750;
extern char g_433754;
extern char g_433755;
extern unsigned int g_433758;
extern char g_43375c;
extern char g_43375d;
extern unsigned long long g_433780;
extern long long g_433788;
extern long long g_433790;
extern unsigned long long g_4337a0[4];
extern unsigned long long g_4337b0[4];
extern char g_4337c8;
extern char g_4337c9;
extern unsigned int g_4337cc;
extern struct_1 *g_4337d8;
extern unsigned long long g_4337e0;
extern unsigned long long g_4337f0;
extern long long g_4337f8;
extern unsigned long long g_433818;
extern char g_433831;
extern unsigned int g_433840;
extern unsigned int g_433844;
extern unsigned int g_433848;
extern unsigned int g_43384c;
extern char g_433850;
extern char g_433851;
extern char g_433852;
extern char g_433853;
extern long long g_433858;
extern long long g_433860;
extern char g_433868;
extern unsigned long long g_433870;
extern char g_433878;
extern unsigned long long g_433880;
extern char g_4338b0;
extern char g_4338b1;
extern char g_4338b2;
extern char g_4338b3;

unsigned int main(int a0, char **a1)
{
    unsigned int v48;  // eax
    int v58;  // eax
    char v59;  // al
    unsigned int v62;  // eax
    char v64;  // al
    char v65;  // al
    char v66;  // al
    unsigned int v49;  // eax
    unsigned long long v67;  // rax
    unsigned int v68;  // eax
    void* v69;  // rax
    unsigned int v50;  // eax
    int *err;  // rax
    int *err1;  // rax
    long long v56;  // rax
    char v0;  // [bp-0x203]
    char v1;  // [bp-0x202]
    char v2;  // [bp-0x201]
    unsigned int v3;  // [bp-0x200]
    int v4;  // [bp-0x1fc]
    unsigned int v5;  // [bp-0x1f8]
    unsigned int v6;  // [bp-0x1f4]
    int v7;  // [bp-0x1f0]
    int v8;  // [bp-0x1ec]
    int v9;  // [bp-0x1e8]
    int v10;  // [bp-0x1e4]
    unsigned int v11;  // [bp-0x1e0]
    int v12;  // [bp-0x1dc]
    long long iter;  // [bp-0x1d8]
    unsigned long v14;  // [bp-0x1d0]
    long long v15;  // [bp-0x1c8]
    char v16;  // [bp-0x1c0]
    unsigned long long v17;  // [bp-0x1b8]
    unsigned long long fp;  // [bp-0x1b0]
    long long v19;  // [bp-0x1a8]
    long long v20;  // [bp-0x1a8]
    struct_0 *i;  // [bp-0x1a0]
    void* v22;  // [bp-0x198]
    char *v23;  // [bp-0x190]
    unsigned long len1;  // [bp-0x188]
    struct_0 *v25;  // [bp-0x180]
    long long v26;  // [bp-0x178]
    long long v27;  // [bp-0x170]
    unsigned long count;  // [bp-0x168]
    unsigned long len;  // [bp-0x160]
    long long v30;  // [bp-0x158]
    char *v31;  // [bp-0x150]
    stat v32;  // [bp-0x148]
    unsigned long v33;  // [bp-0x100]
    unsigned long v34;  // [bp-0xf8]
    unsigned long v35;  // [bp-0xf0]
    unsigned long v36;  // [bp-0xe8]
    unsigned long v37;  // [bp-0xe0]
    unsigned long v38;  // [bp-0xd8]
    unsigned long v39;  // [bp-0xd0]
    unsigned long v40;  // [bp-0xc8]
    unsigned long v41;  // [bp-0xc0]
    stat v42;  // [bp-0xb8]
    char v43;  // [bp-0x23]
    char v44;  // [bp-0x22]
    char v45;  // [bp-0x21]

    v17 = 0;
    iter = 0;
    v14 = 0;
    v3 = 0xffffffff;
    g_433080 = 2;
    v5 = 0;
    g_433183 = 10;
    g_43384c = 0xffffffff;
    g_433870 = 0x7fffffffffffffff;
    g_433858 = 0xffffffffffffffff;
    g_433860 = g_433858;
    v15 = -0x1;
    g_43375c = 0;
    setlocale(6, &g_4290d2);
    bindtextdomain("grep", "/usr/local/share/locale");
    textdomain("grep");
    sub_41bf18(&g_4331a0);
    sub_4285f0(sub_406f2c);
    sub_4112dd(0);
    v4 = 0;
    g_433790 = sub_4208d6(0, 0, sub_406760, sub_4067f8, 0);
    if (!g_433790)
        sub_427122(); /* do not return */
    while (1)
    {
        v10 = optind;
        v11 = sub_409c9f(a0, a1, &v15);
        if (v11 != 0xffffffff)
        {
            switch (v11)
            {
            case 0: case 85:
                break;
            case 65:
                sub_40728f(*((long long *)&optarg), &g_433860);
                continue;
            case 66:
                sub_40728f(*((long long *)&optarg), &g_433858);
                continue;
            case 67:
                sub_40728f(*((long long *)&optarg), &v15);
                continue;
            case 68:
                if (!strcmp(*((long long *)&optarg), "read"))
                {
                    g_4337cc = 1;
                    break;
                }
                else
                {
                    if (!strcmp(*((long long *)&optarg), "skip"))
                    {
                        g_4337cc = 2;
                        break;
                    }
                    else
                    {
                        error(2, 0, gettext("unknown devices method"));
                    }
                }
            case 69:
                v3 = sub_409be6("egrep", v3);
                continue;
            case 70:
                v3 = sub_409be6("fgrep", v3);
                continue;
            case 71:
                v3 = sub_409be6("grep", v3);
                continue;
            case 72:
                v5 = 1;
                continue;
            case 73:
                g_433840 = 2;
                continue;
            case 76:
                g_433844 = 2;
                continue;
            case 80:
                v3 = sub_409be6("perl", v3);
                continue;
            case 84:
                g_43375d = 1;
                continue;
            case 86:
                g_433754 = 1;
                continue;
            case 88:
                v3 = sub_409be6(*((long long *)&optarg), v3);
                continue;
            case 90:
                g_43384c = 0;
                continue;
            case 97:
                g_433840 = 1;
                continue;
            case 98:
                g_433853 = 1;
                continue;
            case 99:
                g_433868 = 1;
                continue;
            case 100:
                g_433010 = g_4293a8[sub_410c86("--directories", *((long long *)&optarg), &g_432700, &g_4293a8[0], 4, g_433078, 1)];
                if (g_433010 == 3)
                {
                    v4 = v10;
                    break;
                }
                break;
            case 101:
                len = strlen(*((long long *)&optarg));
                v30 = len + iter - v14 + 1;
                if (v30 > 0)
                {
                    v17 = sub_426dc6(v17, &v14, v30, -0x1, 1);
                    g_433780 = v17;
                }
                v31 = mempcpy(v17 + iter, *((long long *)&optarg), len);
                *(v31) = 10;
                iter = sub_40687b(v17, iter, len + iter + 1, &g_4290d2);
                continue;
            case 102:
                if (!strcmp(*((long long *)&optarg), "-"))
                {
                    if (g_4338b3)
                        sub_42716d(0, 0);
                    fp = stdin;
                }
                else
                {
                    fp = fopen(*((long long *)&optarg), (!g_4338b3 ? "r" : "rb"));
                    if (!fp)
                    {
                        err = __errno_location();
                        error(2, *(err), "%s");
                    }
                }
                v19 = iter;
                while (1)
                {
                    v27 = v19 - v14 + 2;
                    if (v27 > 0)
                    {
                        v17 = sub_426dc6(v17, &v14, v27, -0x1, 1);
                        g_433780 = v17;
                    }
                    count = fread_unlocked(v19 + v17, 1, v14 - (v19 + 1), fp);
                    if (!count)
                        break;
                    v19 += count;
                }
                v6 = *(__errno_location());
                if (!ferror_unlocked(fp))
                {
                    v6 = 0;
                    if (fp == stdin)
                    {
                        clearerr_unlocked(fp);
                    }
                    else
                    {
                        if (fclose(fp))
                            v6 = *(__errno_location());
                    }
                }
                if (v6)
                    error(2, v6, "%s");
                if (v19 != iter && *((char *)(v17 + v19 - 1)) != 10)
                {
                    v20 = v19 + 1;
                    *((char *)(v17 + v19)) = 10;
                    v19 = v20;
                }
                iter = sub_40687b(v17, iter, v19, *((long long *)&optarg));
                break;
            case 104:
                v5 = 0xffffffff;
                continue;
            case 105: case 121:
                g_433180 = 1;
                continue;
            case 108:
                g_433844 = 1;
                continue;
            case 109:
                if (sub_427281(*((long long *)&optarg), 0, 10, &g_433870, &g_4290d2) > 1)
                    error(2, 0, gettext("invalid max count"));
                continue;
            case 110:
                g_433852 = 1;
                continue;
            case 111:
                g_43375c = 1;
                continue;
            case 113:
                g_4338b1 = 1;
                g_433080 = 0;
                continue;
            case 115:
                g_433755 = 1;
                continue;
            case 117:
                error(0, 0, gettext("warning: --unix-byte-offsets (-u) is obsolete"));
                continue;
            case 118:
                g_433851 = 1;
                continue;
            case 119:
                sub_40f770();
                g_433181 = 1;
                continue;
            case 120:
                g_433182 = 1;
                continue;
            case 122:
                g_433183 = 0;
                continue;
            case 128:
                if (!strcmp(*((long long *)&optarg), "binary"))
                {
                    g_433840 = 0;
                    break;
                }
                else
                {
                    if (!strcmp(*((long long *)&optarg), "text"))
                    {
                        g_433840 = 1;
                        break;
                    }
                    else
                    {
                        if (!strcmp(*((long long *)&optarg), "without-match"))
                        {
                            g_433840 = 2;
                            break;
                        }
                        else
                        {
                            error(2, 0, gettext("unknown binary-files type"));
                        }
                    }
                }
            case 129:
                if (*((long long *)&optarg))
                {
                    if (!sub_411390(*((long long *)&optarg), "always", "always") || !sub_411390(*((long long *)&optarg), "yes", "yes") || !sub_411390(*((long long *)&optarg), "force", "force"))
                    {
                        g_433758 = 1;
                    }
                    else
                    {
                        if (!sub_411390(*((long long *)&optarg), "never", "never") || !sub_411390(*((long long *)&optarg), "no", "no") || !sub_411390(*((long long *)&optarg), "none", "none"))
                        {
                            g_433758 = 0;
                        }
                        else
                        {
                            if (!sub_411390(*((long long *)&optarg), "auto", "auto") || !sub_411390(*((long long *)&optarg), "tty", "tty") || !sub_411390(*((long long *)&optarg), "if-tty", "if-tty"))
                            {
                                g_433758 = 2;
                            }
                            else
                            {
                                g_433750 = 1;
                                continue;
                            }
                        }
                    }
                }
                else
                {
                    g_433758 = 2;
                }
                continue;
            case 130:
                sub_41c12b(*((long long *)&optarg));
                for (v9 = 0; v9 <= 1; v9 += 1)
                {
                    if (!g_4337b0[v9])
                        g_4337b0[v9] = sub_41c384();
                    v50 = sub_407316(v9);
                    sub_41cca9(g_4337b0[v9], *((long long *)&optarg), v50, *((long long *)&optarg));
                }
            case 131: case 134:
                for (v7 = 0; v7 <= 1; v7 += 1)
                {
                    if (!g_4337a0[v7])
                        g_4337a0[v7] = sub_41c384();
                    v48 = sub_407316(v7);
                    sub_41cca9(g_4337a0[v7], *((long long *)&optarg), (v11 == 134 ? 0x20000000 : 0) | v48, *((long long *)&optarg));
                }
            case 132:
                v8 = 0;
                while (v8 <= 1)
                {
                    if (!g_4337a0[v8])
                        g_4337a0[v8] = sub_41c384();
                    v49 = sub_407316(v8);
                    if (sub_41d272(sub_41cca9, g_4337a0[v8], *((long long *)&optarg), v49, 10))
                    {
                        err1 = __errno_location();
                        error(2, *(err1), "%s");
                    }
                    v8 += 1;
                }
            case 133:
                g_433020 = *((long long *)&optarg);
                continue;
            case 135:
                g_433878 = 1;
                continue;
            case 136:
                g_433880 = *((long long *)&optarg);
                break;
            case 137:
                g_433180 = 0;
                continue;
            case 82:
                g_433014 = 778;
            case 114:
                g_433010 = 3;
                v4 = v10;
                continue;
            default:
                sub_4099b6(2); /* do not return */
            }
        }
        else if (g_433754)
        {
            v56 = sub_41ff0c();
            sub_42690a(stdout, v56, "GNU grep", "3.8", 0);
            puts(gettext("Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>."));
            return 0;
        }
        else if (g_433750)
        {
            sub_4099b6(0); /* do not return */
        }
        else
        {
            if (v17)
            {
                if (!iter)
                {
                    g_433851 = g_433851 ^ 1;
                    g_433181 = 0;
                    g_433182 = g_433181;
                    iter += 1;
                    *((char *)(v17 + iter)) = 10;
                }
            }
            else
            {
                if (a0 > optind)
                {
                    v58 = optind;
                    optind = v58 + 1;
                    v23 = a1[v58];
                    if (v3 != 2 && *(v23) == 92 && v23[1] == 45)
                        v59 = 1;
                    else
                        v59 = 0;
                    v2 = v59;
                    v2 &= 1;
                    v17 = sub_4270f1(&v23[v2]);
                    g_433780 = v17;
                    len1 = strlen(v17);
                    *((char *)(v17 + len1)) = 10;
                    iter = sub_40687b(v17, 0, len1 + 1, &g_4290d2);
                }
                else
                {
                    sub_4099b6(2); /* do not return */
                }
            }
            iter -= 1;
            sub_420b56(g_433790);
            v0 = 0;
            if (g_4338b1 != 1 && !fstat(1, &v32))
            {
                if (((unsigned short)v32.st_nlink & 0xf000) == 0x8000)
                {
                    g_4336c0 = *((unsigned long long *)&v32);
                    g_4336c8 = v32.st_ino;
                    g_4336d0 = v32.st_dev;
                    g_4336d8 = *((unsigned long long *)((void*)&v32 + 24));
                    g_4336e0 = *((unsigned long long *)((void*)&v32 + 32));
                    g_4336e8 = v32.st_size;
                    g_4336f0 = v32.st_atime;
                    g_4336f8 = v32.st_mtime;
                    g_433700 = v32.st_ctime;
                    g_433708 = v33;
                    g_433710 = v34;
                    g_433718 = v35;
                    g_433720 = v36;
                    g_433728 = v37;
                    g_433730 = v38;
                    g_433738 = v39;
                    g_433740 = v40;
                    g_433748 = v41;
                    break;
                }
                else if (((unsigned short)v32.st_nlink & 0xf000) == 0x2000)
                {
                    if (!stat("/dev/null", &v42) && v32.st_ino == v42.st_ino && *((unsigned long long *)&v32) == *((unsigned long long *)&v42))
                    {
                        g_4338b2 = 1;
                        break;
                    }
                    else
                    {
                        v0 = 1;
                        break;
                    }
                }
            }
        }
    }
    if (g_4338b2 || g_4338b1)
        g_433844 = 0;
    if (g_4338b2 || g_4338b1 || g_433844)
    {
        g_433868 = 0;
        g_4338b0 = 1;
    }
    g_433850 = g_4338b0 || g_433868;
    if (g_433860 < 0)
        g_433860 = v15;
    if (g_433858 < 0)
        g_433858 = v15;
    if ((!g_433870 || !iter && g_433851 && g_433182 != 1 && g_433181 != 1) && g_433844 != 2)
        return 1;
    if (g_433758 == 2)
    {
        if (v0 && sub_42768b() && isatty(1))
            v62 = 1;
        else
            v62 = 0;
        g_433758 = v62;
    }
    sub_4276d9();
    if (g_433758)
    {
        v25 = getenv("GREP_COLOR");
        if (v25 && v25->field_0)
        {
            for (i = v25; i->field_0 == 59 || (char)sub_410eab(i->field_0); i = &i->field_1)
            {
                if (!i->field_1)
                {
                    g_433030 = v25;
                    g_433028 = g_433030;
                    break;
                }
            }
        }
        sub_409ddf();
        if (v25 == g_433028 || v25 == g_433030)
            error(0, 0, gettext("warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'"));
    }
    sub_406f46();
    if (v3 < 0)
        v3 = 0;
    if (v3 == 2 || v3 == 1 || !v3)
    {
        if (g_433180)
            sub_40a011();
        if (v3 == 2)
        {
            if (g_4331a0 != 1)
            {
                if (g_433788 == 1 && g_433181)
                    v64 = 1;
                else
                    v64 = 0;
                v65 = v64 & 1;
            }
            else
            {
                if ((char)sub_409f7b(v17, iter) || g_433180 && (char)sub_40a29a(v17, iter) != 1)
                    v66 = 1;
                else
                    v66 = 0;
                v65 = v66 & 1;
            }
            if (v65)
            {
                sub_40a32f(&g_433780, &iter);
                v17 = g_433780;
                v3 = 0;
            }
        }
        else if (g_433788 > 1)
        {
            v3 = sub_40a4d3(v3, v17, &iter);
        }
    }
    g_4337d8 = g_4327b8[4 * v3];
    g_4337e0 = g_4327b0[4 * v3](v17, iter, g_4327ac[8 * v3], g_433758 || g_43375c, g_4327b0[4 * v3]);
    v43 = 0;
    v44 = g_433183;
    v45 = 0;
    v67 = g_4337d8(g_4337e0, &v44, 1, &v16, 0, g_4337d8);
    g_433831 = !v67 == g_433851;
    v12 = a0 - optind;
    if (!v5 && v12 <= 1)
        v68 = -(g_433010 == 3);
    else
        v68 = ~(v5) >> 31 & 0xff;
    g_433848 = v68;
    if (g_4338b3)
        sub_42716d(1, 0);
    v26 = sysconf(30);
    if (v26 > 0 && v26 <= 0x3ffffffffffffffb)
    {
        g_433818 = v26;
        g_4337f8 = g_433818 + (!(unsigned long long)(0x18000 % g_433818) ? 0x18000 : g_433818 - (unsigned long long)(0x18000 % g_433818) + 0x18000) + 8;
        g_4337f0 = sub_426b10(g_4337f8);
        if ((char)g_433014 & 2 && !g_4337cc)
            g_4337cc = 1;
        if (v12 > 0)
        {
            v22 = &a1[optind];
        }
        else if (g_433010 == 3 && v4 > 0)
        {
            v22 = &g_432720;
            g_4337c8 = 1;
        }
        else
        {
            v22 = &g_432730;
        }
        v1 = 1;
        do
        {
            v69 = v22;
            v22 = v69 + 8;
            v1 = (char)sub_40992b(*((long long *)v69)) & v1;
        } while (*((long long *)v22));
        return (!g_4337c9 ? v1 : 2);
    }
    abort(); /* do not return */
}



// Function: Fcompile @ 0xc03f
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_433182;
extern char g_433183;

struct_0 * Fcompile(char *a0, unsigned long a1, unsigned long long a2, char a3)
{
    char v0;  // [bp-0x74]
    unsigned long long v1;  // [bp-0x70]
    unsigned long v2;  // [bp-0x50]
    char *ptr;  // [bp-0x48]
    char *iter;  // [bp-0x40]
    unsigned long n;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    char *ptr1;  // [bp-0x28]
    unsigned long long v8;  // [bp-0x20]
    struct_0 *idx;  // [bp-0x18]

    v1 = a2;
    v0 = a3;
    ptr = NULL;
    v2 = 0;
    v6 = sub_40f7c8(1);
    iter = a0;
    do
    {
        ptr1 = rawmemchr(iter, 10);
        n = ptr1 - iter;
        if (g_433182)
        {
            if (g_433183 == 10 && a0 < iter)
            {
                iter -= 1;
            }
            else
            {
                if (n + 1 >= v2)
                {
                    free(ptr);
                    v2 = n;
                    ptr = sub_426dc6(0, &v2, 2, -0x1, 1);
                    *(ptr) = g_433183;
                }
                memcpy(ptr + 1, iter, n);
                ptr[1 + n] = g_433183;
                iter = ptr;
            }
            n += 2;
        }
        sub_40cacb(v6, iter, n, iter);
        iter = ptr1 + 1;
    } while (iter <= &a0[a1]);
    free(ptr);
    v8 = sub_40d633(v6);
    sub_40d9b5(v6);
    idx = sub_426aea(40);
    idx->field_0 = v6;
    idx->field_8 = v8;
    idx->field_10 = a0;
    idx->field_18 = a1;
    idx->field_20 = 0;
    return idx;
}



// Function: Fexecute @ 0xc23c
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[4];
    unsigned int field_18;
    char padding_1c[4];
    void* field_20;
} struct_0;

extern char g_433181;
extern char g_433182;
extern char g_433183;
extern char g_4331a0;
extern char g_4331a2;

unsigned long long Fexecute(struct_0 *a0, unsigned long a1, unsigned long a2, long long *a3, unsigned long a4)
{
    unsigned long v18;  // rax
    unsigned long v0;  // [bp-0xc0]
    unsigned long v1;  // [bp-0xa8]
    char v2;  // [bp-0x93]
    char v3;  // [bp-0x92]
    char v4;  // [bp-0x91]
    unsigned long v5;  // [bp-0x90]
    unsigned long v6;  // [bp-0x88]
    unsigned long iter;  // [bp-0x80]
    unsigned long ptr;  // [bp-0x78]
    long long node;  // [bp-0x70]
    unsigned long v10;  // [bp-0x60]
    unsigned long long v11;  // [bp-0x58]
    unsigned long v12;  // [bp-0x50]
    char v13;  // [bp-0x48]
    unsigned long v14;  // [bp-0x38]
    char v15;  // [bp-0x28]
    long long v16;  // [bp-0x18]

    v1 = a1;
    v0 = a4;
    v2 = g_433183;
    v10 = a0->field_0;
    v3 = (g_433182 ^ 1) & g_4331a0 & (g_4331a2 ^ 1);
    v4 = (g_433182 ^ 1) & (v3 | v0 | g_433181);
    if (v0)
        v18 = v0;
    else
        v18 = v1;
    iter = v18;
    v5 = iter;
    while (1)
    {
        if (iter > v1 + a2 || !(v11 = (unsigned long long)sub_40f585(v10, -((unsigned long long)g_433182) + iter, v1 + a2 - iter + (unsigned long long)g_433182, &v13, (unsigned long long)v4), !(v11 - 0 >> 63 & 1)))
            return 0xffffffffffffffff;
        node = v14 - g_433182 * 2;
        v6 = 0;
        if (v3 && sub_40f84a(&v5, &v6, v11 + iter, a2 + v1))
        {
            iter = v5 - 1;
        }
        else
        {
            iter += v11;
            if ((g_433181 ^ 1) & v0)
            {
                *(a3) = node;
                return iter - v1;
            }
            if (g_433182)
            {
                node += !v0;
                *(a3) = node;
                return iter - v1;
            }
            if (g_433181 != 1)
                break;
            if (!v6)
            {
                v12 = memrchr(v5, v2, iter - v5);
                if (v12)
                    v5 = v12 + 1;
            }
            if ((v6 <= 0 ? !sub_40fc0e(v5, iter, a2 + v1, iter) : !sub_40fbe0(iter + -(v6), a2 + v1, a2 + v1, -(v6))))
            {
                while (1)
                {
                    if (!sub_40fbe0(iter + node, a2 + v1, a2 + v1, node))
                    {
                        if (v0)
                        {
                            *(a3) = node;
                            return iter - v1;
                        }
                        goto LABEL_40c73e;
                    }
                    if (!v0 && g_4331a0 != 1)
                    {
                        if (!a0->field_20)
                        {
                            sub_40a32f(&a0->field_10, &a0->field_18);
                            a0->field_20 = sub_405661(*((long long *)&a0->field_10), *((long long *)&a0->field_18), 68166, v0);
                        }
                        if (node + iter < v1 + a2)
                        {
                            ptr = rawmemchr(iter + node, v2);
                            ptr += 1;
                        }
                        else
                        {
                            ptr = v1 + a2;
                        }
                        if (sub_405d23(a0->field_20, iter, ptr - iter, a3, 0) >= 0)
                        {
                            iter = memrchr(v1, v2, iter - v1);
                            iter = (!iter ? v1 : iter + 1);
                            node = ptr - iter;
                            *(a3) = node;
                            return iter - v1;
                        }
                        iter = ptr - 1;
                        break;
                    }
                    if (!node || !(node = (long long)(node - 1), !(unsigned long long)sub_40f585(v10, iter, node, &v15, 1)))
                        break;
                    node = v16;
                }
            }
            iter += sub_40fbb2(iter, a2 + v1, a2 + v1);
            v5 = iter;
        }
        iter += 1;
    }
LABEL_40c73e:
    if (node + iter >= v1 + a2)
    {
        ptr = v1 + a2;
        iter = memrchr(v1, v2, iter - v1);
        iter = (!iter ? v1 : iter + 1);
        node = ptr - iter;
        *(a3) = node;
        return iter - v1;
    }
    ptr = rawmemchr(iter + node, v2);
    ptr += 1;
    iter = memrchr(v1, v2, iter - v1);
    iter = (!iter ? v1 : iter + 1);
    node = ptr - iter;
    *(a3) = node;
    return iter - v1;
}



// Function: U @ 0xc845
int U(char a0)
{
    return sub_40f64d(a0);
}



// Function: tr @ 0xc863
unsigned int tr(char *a0, char a1)
{
    return (!a0 ? a1 : a0[(char)sub_40c845(a1)]);
}



// Function: kwsalloc @ 0xc89b
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
    char padding_51[7];
    unsigned long long field_58;
    void* field_60;
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
} struct_0;

struct_0 * kwsalloc(unsigned long a0)
{
    struct_0 *index;  // [bp-0x40]
    struct_0 *v1;  // [bp-0x38]
    struct_0 *idx;  // [bp-0x30]
    long long v3;  // [bp-0x28]
    struct_0 *v4;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    unsigned long v6;  // [bp-0x10]

    index = sub_426aea(2464);
    _obstack_begin(index, 0, 0, sub_426aea, free);
    index->field_58 = 0;
    v1 = index;
    idx = v1;
    v3 = 64;
    v4 = idx;
    if (v4->field_20 - v4->field_18 < v3)
        _obstack_newchunk(idx, v3, v3);
    idx->field_18 = idx->field_18 + v3;
    idx1 = v1;
    v6 = idx1->field_10;
    if (v6 == idx1->field_18)
        idx1->field_50 = idx1->field_50 | 2;
    idx1->field_18 = ~(idx1->field_30) & idx1->field_18 + idx1->field_30;
    if (idx1->field_18 - idx1->field_8 > idx1->field_20 - idx1->field_8)
        idx1->field_18 = idx1->field_20;
    idx1->field_10 = idx1->field_18;
    index->field_60 = v6;
    memset(index->field_60, 0, 56);
    index->field_68 = 0x7fffffffffffffff;
    index->field_970 = 0;
    index->field_980 = a0;
    index[1].field_10 = sub_40f50c;
    return index;
}



// Function: kwsincr @ 0xcacb
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    void* field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char padding_28[8];
    unsigned long long field_30;
    char padding_38[24];
    char field_50;
    char padding_51[7];
    unsigned long long field_58;
    void* field_60;
    long long field_68;
    char padding_70[2320];
    struct struct_1 *field_980;
    char padding_988[16];
    unsigned long long field_998;
} struct_0;

typedef struct struct_2 {
    char padding_0[25];
    char field_19;
} struct_2;

typedef struct struct_6 {
    char padding_0[8];
    struct struct_2 *field_8;
    char padding_10[9];
    char field_19;
} struct_6;

typedef struct struct_1 {
    char field_0;
} struct_1;

void kwsincr(struct_0 *idx, char *a1, unsigned long long a2)
{
    char v30;  // al
    char *v31;  // rax
    unsigned long long idx1;  // rax
    unsigned long long idx2;  // rax
    struct_2 *v34;  // rax
    struct_2 *v35;  // rax
    unsigned int v36;  // eax
    unsigned long v37;  // rax
    unsigned long v38;  // rax
    unsigned long long v0;  // [bp-0x170]
    unsigned long long v1;  // [bp-0x170]
    char *node;  // [bp-0x168]
    char v3;  // [bp-0x153]
    char v4;  // [bp-0x152]
    char v5;  // [bp-0x151]
    void* v6;  // [bp-0x150]
    void* iter1;  // [bp-0x148]
    unsigned long long iter;  // [bp-0x140]
    struct_2 *v9;  // [bp-0x138]
    char *v10;  // [bp-0x130]
    struct_0 *v11;  // [bp-0x128]
    struct_0 *v12;  // [bp-0x120]
    long long v13;  // [bp-0x118]
    struct_0 *v14;  // [bp-0x110]
    struct_0 *v15;  // [bp-0x108]
    void* v16;  // [bp-0x100]
    struct_0 *v17;  // [bp-0xf8]
    struct_0 *v18;  // [bp-0xf0]
    long long v19;  // [bp-0xe8]
    struct_0 *v20;  // [bp-0xe0]
    struct_0 *v21;  // [bp-0xd8]
    void* v22;  // [bp-0xd0]
    struct_6 *v23;  // [bp-0xc8], Other Possible Types: unsigned long
    unsigned long v24;  // [bp-0xc0]
    struct_2 *v25;  // [bp-0xb8], Other Possible Types: unsigned long
    unsigned long v26;  // [bp-0xb0]
    unsigned int v27;  // [bp-0xa8]
    unsigned long v28;  // [bp-0x78]

    node = a1;
    v0 = a2;
    v6 = idx->field_60;
    v10 = &idx->field_980->field_0;
    v3 = idx->field_998 == sub_40f061;
    if (v3)
        node = &node[v0];
    while (1)
    {
        v1 = v0 - 1;
        if (!v0)
            break;
        if (v3)
        {
            node -= 1;
            v30 = *(node);
        }
        else
        {
            v31 = node;
            node = v31 + 1;
            v30 = *(v31);
        }
        v4 = v30;
        v5 = (!v10 ? v4 : v10[v4]);
        iter1 = (long long)v6[8];
        v28 = v6 + 8;
        v27 = 0;
        iter = 1;
        while (iter1 && v5 != (char)iter1[24])
        {
            (&v28)[iter] = iter1;
            if (v5 < (char)iter1[24])
            {
                idx1 = iter;
                iter = idx1 + 1;
                (&v27)[idx1] = 0;
                iter1 = *((long long *)iter1);
            }
            else
            {
                idx2 = iter;
                iter = idx2 + 1;
                (&v27)[idx2] = 1;
                iter1 = (long long)iter1[8];
            }
        }
        if (!iter1)
        {
            v11 = idx;
            v12 = v11;
            v13 = 32;
            v14 = v12;
            if (v14->field_20 - v14->field_18 < v13)
                _obstack_newchunk(v12, v13, v13);
            v12->field_18 = v12->field_18 + v13;
            v15 = v11;
            v16 = v15->field_10;
            if (v16 == v15->field_18)
                v15->field_50 = v15->field_50 | 2;
            v15->field_18 = ~(v15->field_30) & v15->field_18 + v15->field_30;
            if (v15->field_18 - v15->field_8 > v15->field_20 - v15->field_8)
                v15->field_18 = v15->field_20;
            v15->field_10 = v15->field_18;
            iter1 = v16;
            *((void* *)iter1) = NULL;
            *((void* *)&iter1[8]) = NULL;
            v17 = idx;
            v18 = v17;
            v19 = 64;
            v20 = v18;
            if (v20->field_20 - v20->field_18 < v19)
                _obstack_newchunk(v18, v19, v19);
            v18->field_18 = v18->field_18 + v19;
            v21 = v17;
            v22 = v21->field_10;
            if (v22 == v21->field_18)
                v21->field_50 = v21->field_50 | 2;
            v21->field_18 = ~(v21->field_30) & v21->field_18 + v21->field_30;
            if (v21->field_18 - v21->field_8 > v21->field_20 - v21->field_8)
                v21->field_18 = v21->field_20;
            v21->field_10 = v21->field_18;
            *((void* *)&iter1[16]) = v22;
            *((void* *)(long long)iter1[16]) = NULL;
            *((void* *)((long long)iter1[16] + 8)) = NULL;
            *((void* *)((long long)iter1[16] + 16)) = v6;
            *((unsigned long *)((long long)iter1[16] + 24)) = 0;
            *((unsigned long *)((long long)iter1[16] + 32)) = 0;
            *((unsigned long long *)((long long)iter1[16] + 40)) = (long long)v6[40] + 1;
            *((unsigned long *)((long long)iter1[16] + 48)) = 0;
            *((char *)&iter1[24]) = v5;
            *((char *)&iter1[25]) = 0;
            iter -= 1;
            if (!(&v27)[iter])
                *((void* *)(&v28)[iter]) = iter1;
            else
                *((void* *)((&v28)[iter] + 8)) = iter1;
            for (; iter && !*((char *)((&v28)[iter] + 25)); iter -= 1)
            {
                if (!(&v27)[iter])
                    *((char *)((&v28)[iter] + 25)) = *((char *)((&v28)[iter] + 25)) - 1;
                else
                    *((char *)((&v28)[iter] + 25)) = *((char *)((&v28)[iter] + 25)) + 1;
            }
            if (iter && (!(&v27)[iter] && !(v34 = (struct_2 *)(&v28)[iter], v34->field_19 = v34->field_19 - 1, !v34->field_19) || (&v27)[iter] == 1 && !(v35 = (struct_2 *)(&v28)[iter], v35->field_19 = v35->field_19 + 1, !v35->field_19)))
            {
                v36 = *((char *)((&v28)[iter] + 25));
                switch (v36)
                {
                case 4294967294:
                    if (!(&v27)[1 + iter])
                    {
                        v25 = (&v28)[iter];
                        v9 = *((long long *)&v25->padding_0[0]);
                        v26 = *((long long *)&v9->padding_0[8]);
                        *((struct_2 **)&v9->padding_0[8]) = v25;
                        *((unsigned long *)&v25->padding_0[0]) = v26;
                        v25->field_19 = 0;
                        v9->field_19 = v25->field_19;
                    }
                    else if ((&v27)[1 + iter] == 1)
                    {
                        v25 = (&v28)[iter];
                        v23 = *((long long *)v25);
                        v9 = *((long long *)(v23 + 8));
                        v26 = *((long long *)&v9->padding_0[8]);
                        v24 = *((long long *)&v9->padding_0[0]);
                        *((unsigned long *)&v9->padding_0[0]) = v23;
                        *((unsigned long *)(v23 + 8)) = v24;
                        *((unsigned long *)&v9->padding_0[8]) = v25;
                        *((unsigned long *)v25) = v26;
                        *((char *)(v23 + 25)) = (v9->field_19 == 1 ? 0xff : 0);
                        v38 = v9->field_19;
                        *((char *)(v25 + 25)) = _INSERT(v38, 0, (char)v38 == 0xff) & 0xffffffff;
                        v9->field_19 = 0;
                    }
                    else
                    {
                        abort(); /* do not return */
                    }
                    break;
                case 2:
                    if (!(&v27)[1 + iter])
                    {
                        v23 = (&v28)[iter];
                        v25 = *((long long *)(v23 + 8));
                        v9 = *((long long *)v25);
                        v24 = *((long long *)&v9->padding_0[0]);
                        v26 = *((long long *)&v9->padding_0[8]);
                        *((unsigned long *)&v9->padding_0[0]) = v23;
                        *((unsigned long *)(v23 + 8)) = v24;
                        *((unsigned long *)&v9->padding_0[8]) = v25;
                        *((unsigned long *)v25) = v26;
                        *((char *)(v23 + 25)) = (v9->field_19 == 1 ? 0xff : 0);
                        v37 = v9->field_19;
                        *((char *)(v25 + 25)) = _INSERT(v37, 0, (char)v37 == 0xff) & 0xffffffff;
                        v9->field_19 = 0;
                    }
                    else if ((&v27)[1 + iter] == 1)
                    {
                        v23 = (&v28)[iter];
                        v9 = v23->field_8;
                        v24 = *((long long *)&v9->padding_0[0]);
                        *((struct_6 **)&v9->padding_0[0]) = v23;
                        v23->field_8 = v24;
                        v23->field_19 = 0;
                        v9->field_19 = v23->field_19;
                    }
                    else
                    {
                        abort(); /* do not return */
                    }
                    break;
                default:
                    abort(); /* do not return */
                }
                if (!*((int *)((char *)&(&v27)[iter] - 4)))
                    *((struct_2 **)*((long long *)((char *)&(&v28)[iter] - 8))) = v9;
                else
                    *((struct_2 **)(*((long long *)((char *)&(&v28)[iter] - 8)) + 8)) = v9;
            }
        }
        v6 = (long long)iter1[16];
        v0 = v1;
    }
    if (!*((long long *)v6))
        *((unsigned long long *)v6) = idx->field_58 * 2 + 1;
    idx->field_58 = idx->field_58 + 1;
    if ((long long)v6[40] < idx->field_68)
        idx->field_68 = (long long)v6[40];
    return;
}



// Function: kwswords @ 0xd633
typedef struct struct_0 {
    char padding_0[88];
    unsigned long long field_58;
} struct_0;

long long kwswords(struct_0 *a0)
{
    return a0->field_58;
}



// Function: enqueue @ 0xd649
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[24];
    unsigned long long field_18;
} struct_1;

struct_1 ** enqueue(struct_0 *a0, struct_1 **a1)
{
    struct_1 *idx;  // rax
    struct_1 **v2;  // rax

    if (!a0)
        return v2;
    enqueue(a0->field_0, a1);
    enqueue(a0->field_8, a1);
    idx = *(a1);
    idx->field_18 = a0->field_10;
    *(a1) = idx->field_18;
    return a1;
}



// Function: treefails @ 0xd6b4
typedef struct struct_6 {
    void* field_0;
    void* field_8;
    struct struct_0 *field_10;
} struct_6;

typedef struct struct_2 {
    struct struct_2 *field_0;
    struct struct_2 *field_8;
    struct struct_1 *field_10;
    char field_18;
} struct_2;

typedef struct struct_3 {
    char padding_0[8];
    struct struct_2 *field_8;
    char padding_10[16];
    struct struct_3 *field_20;
} struct_3;

typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[24];
    struct struct_1 *field_20;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
} struct_1;

struct_0 * treefails(struct_6 *idx, struct_3 *a1, struct_0 **a2, unsigned int a3)
{
    struct_0 *v4;  // rax
    struct_0 *v5;  // rax
    char v0;  // [bp-0x34]
    struct_3 *iter;  // [bp-0x28]
    struct_2 *idx1;  // [bp-0x10]

    iter = a1;
    v0 = a3;
    if (!idx)
        return a3;
    treefails(idx->field_0, iter, a2, v0);
    treefails(idx->field_8, iter, a2, v0);
    while (1)
    {
        if (!iter)
        {
            v5 = idx->field_10;
            v5->field_20 = a2;
            return v5;
        }
        for (idx1 = iter->field_8; idx1 && (char)idx[1].field_0 != idx1->field_18; idx1 = ((char)idx[1].field_0 < idx1->field_18 ? idx1->field_0 : idx1->field_8));
        if (idx1)
            break;
        iter = iter->field_20;
    }
    idx->field_10->field_20 = idx1->field_10;
    v4 = v0 ^ 1;
    if (*((char *)&v4))
    {
        v4 = idx1->field_10->field_0;
        if (v4)
        {
            v4 = idx->field_10->field_0;
            if (!v4)
            {
                v4 = idx->field_10;
                v4->field_0 = 0xffffffffffffffff;
            }
        }
    }
    return v4;
}



// Function: treedelta @ 0xd7f4
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    char padding_10[8];
    char field_18;
} struct_0;

char * treedelta(struct_0 *a0, char *a1, char *a2)
{
    char *v1;  // rax
    char *v2;  // rax
    char *v3;  // rax

    if (!a0)
        return v3;
    treedelta(a0->field_0, a1, a2);
    treedelta(a0->field_8, a1, a2);
    v1 = a2[a0->field_18];
    if (a1 >= v1)
        return v1;
    v2 = &a2[a0->field_18];
    *(v2) = *((char *)&a1);
    return v2;
}



// Function: hasevery @ 0xd883
typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    char padding_10[8];
    char field_18;
} struct_0;

unsigned int hasevery(struct_0 *a0, struct_0 *a1)
{
    struct_0 *v2;  // rax
    struct_0 *iter;  // [bp-0x10]

    iter = a0;
    if (!a1)
    {
        return 1;
    }
    else if ((char)hasevery(iter, a1->field_0) != 1)
    {
        return 0;
    }
    else if ((char)(hasevery(iter, a1->field_8) ^ 1))
    {
        return 0;
    }
    else
    {
        while (iter && (v2 = (struct_0 *)(unsigned long long)iter->field_18, a1->field_18 != *((char *)(void*)&v2)))
        {
            if (a1->field_18 < iter->field_18)
            {
                v2 = iter->field_0;
                iter = iter->field_0;
            }
            else
            {
                v2 = iter->field_8;
                iter = iter->field_8;
            }
        }
        return _INSERT(v2, 0, iter);
    }
}



// Function: treenext @ 0xd943
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    unsigned long long field_10;
    char field_18;
} struct_0;

int treenext(struct_0 *a0, long long a1)
{
    unsigned long long v1;  // rax

    if (a0)
    {
        treenext(a0->field_0, a1);
        treenext(a0->field_8, a1);
        v1 = a0->field_10;
        *((unsigned long long *)(a0->field_18 * 8 + a1)) = v1;
    }
    return v1;
}



// Function: kwsprep @ 0xd9b5
typedef struct struct_0 {
    void* field_0;
    void* field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_1 {
    char padding_0[56];
    long long field_38;
} struct_1;

typedef struct struct_6 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[16];
    struct struct_6 *field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
} struct_6;

typedef struct struct_7 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    struct struct_1 *field_10;
    struct struct_7 *field_18;
    struct struct_6 *field_20;
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned long long field_38;
} struct_7;

void kwsprep(void* a0)
{
    unsigned long long v40;  // rcx
    void* iter;  // rdi
    struct_0 *iter1;  // rsi
    unsigned int v43;  // eax
    void* idx;  // [bp-0xa30]
    char v1;  // [bp-0xa1d]
    unsigned int v2;  // [bp-0xa1c]
    unsigned int v3;  // [bp-0xa18]
    unsigned int l;  // [bp-0xa14]
    unsigned int v5;  // [bp-0xa10]
    int i0;  // [bp-0xa0c]
    struct_7 *v7;  // [bp-0xa08]
    struct_7 *iter2;  // [bp-0xa00]
    unsigned long j;  // [bp-0x9f8]
    struct_6 *node;  // [bp-0x9f0]
    unsigned long m;  // [bp-0x9e8]
    unsigned long n;  // [bp-0x9e0]
    char *v13;  // [bp-0x9d8]
    void* v14;  // [bp-0x9d0]
    void* v15;  // [bp-0x9c8]
    void* v16;  // [bp-0x9c0]
    unsigned long v17;  // [bp-0x9b8]
    void* v18;  // [bp-0x9b0]
    void* idx1;  // [bp-0x9a8]
    unsigned long v20;  // [bp-0x9a0]
    struct_0 *ptr;  // [bp-0x998]
    void* idx2;  // [bp-0x990]
    unsigned long v23;  // [bp-0x988]
    void* v24;  // [bp-0x980]
    void* v25;  // [bp-0x978]
    void* v26;  // [bp-0x970]
    unsigned long v27;  // [bp-0x968]
    void* v28;  // [bp-0x960]
    void* v29;  // [bp-0x958]
    unsigned long v30;  // [bp-0x950]
    void* v31;  // [bp-0x948]
    void* v32;  // [bp-0x940]
    unsigned long long v33;  // [bp-0x938]
    void* v34;  // [bp-0x930]
    void* v35;  // [bp-0x928]
    unsigned long v36;  // [bp-0x920]
    char v37;  // [bp-0x918]
    char v38;  // [bp-0x118]

    idx = a0;
    v13 = (long long)idx[2432];
    v14 = (!v13 ? idx + 112 : &v38);
    v1 = (long long)idx[88] == 1;
    if (v1)
    {
        v7 = (long long)idx[96];
        for (iter2 = v7; iter2; iter2 = iter2->field_18)
        {
            sub_40d649(iter2->field_8, &v7);
        }
        v15 = idx;
        v16 = v15;
        v17 = (long long)idx[104];
        v18 = v16;
        if ((long long)v18[32] - (long long)v18[24] < v17)
            _obstack_newchunk(v16, v17, v17);
        *((unsigned long *)&v16[24]) = (long long)v16[24] + v17;
        idx1 = v15;
        v20 = (long long)idx1[16];
        if (v20 == (long long)idx1[24])
            *((char *)&idx1[80]) = (char)idx1[80] | 2;
        *((long long *)&idx1[24]) = ~((long long)idx1[48]) & (long long)idx1[24] + (long long)idx1[48];
        if ((long long)idx1[24] - (long long)idx1[8] > (long long)idx1[32] - (long long)idx1[8])
            *((long long *)&idx1[24]) = (long long)idx1[32];
        *((long long *)&idx1[16]) = (long long)idx1[24];
        *((unsigned long *)&idx[2416]) = v20;
        iter2 = (long long)idx[96];
        for (j = 0; j < (long long)idx[104]; j += 1)
        {
            *((char *)(j + (long long)idx[2416])) = iter2->field_8->field_18;
            iter2 = iter2->field_18;
        }
        ptr = sub_40c89b((long long)idx[2432]);
        ptr[0x66].field_8 = sub_40f061;
        sub_40cacb(ptr, (long long)idx[2416], (long long)idx[104]);
        idx2 = idx;
        v23 = 0;
        if (v23 > (long long)idx2[8] && v23 < (long long)idx2[32])
        {
            *((unsigned long *)&idx2[16]) = v23;
            *((long long *)&idx2[24]) = (long long)idx2[16];
        }
        else
        {
            _obstack_free(idx2, v23, v23);
        }
        v40 = 308;
        iter = idx;
        for (iter1 = ptr; v40; iter1 = &iter1->field_8)
        {
            v40 -= 1;
            *((void* *)iter) = iter1->field_0;
            iter += 8;
        }
        free(ptr);
    }
    v43 = (long long)idx[104];
    if (0xff < (long long)idx[104])
        v43 = 0xff;
    memset(v14, v43, 0x100);
    v7 = (long long)idx[96];
    for (iter2 = v7; iter2; iter2 = iter2->field_18)
    {
        sub_40d649(iter2->field_8, &v7);
        sub_40d7f4(iter2->field_8, iter2->field_28, v14);
        sub_40d6b4(iter2->field_8, iter2->field_20, (long long)idx[96], v1);
        if (v1)
        {
            iter2->field_30 = (long long)idx[104];
            iter2->field_38 = (long long)idx[104];
            for (node = iter2->field_20; node; node = node->field_20)
            {
                if ((char)sub_40d883(node->field_8, iter2->field_8) != 1 && iter2->field_28 - node->field_28 < node->field_30)
                    node->field_30 = iter2->field_28 - node->field_28;
                if (iter2->field_0 && node->field_38 > iter2->field_28 - node->field_28)
                    node->field_38 = iter2->field_28 - node->field_28;
            }
        }
    }
    if (v1)
    {
        for (iter2 = *((long long *)((long long)idx[96] + 24)); iter2; iter2 = iter2->field_18)
        {
            if (iter2->field_38 > iter2->field_10->field_38)
                iter2->field_38 = iter2->field_10->field_38;
            if (iter2->field_30 > iter2->field_38)
                iter2->field_30 = iter2->field_38;
        }
    }
    v24 = (!v13 ? idx + 368 : &v37);
    memset(v24, 0, 0x800);
    sub_40d943(*((long long *)((long long)idx[96] + 8)), v24);
    v2 = 0xfffffffe;
    v3 = 0xffffffff;
    for (l = 0; l <= 0xff; l += 1)
    {
        v5 = l;
        if (v13)
        {
            v5 = sub_40c845(v13[l]) & 0xff;
            *((long long *)((char *)idx + 8 * l + 368)) = *((long long *)((char *)v24 + 8 * v5));
        }
        if (*((long long *)((char *)idx + 8 * l + 368)))
        {
            if (v2 < 0xffffffff)
            {
                v2 = v5;
                v3 = l;
            }
            else if (v2 == v5)
            {
                v3 = (v3 == v5 ? l : 0xffffffff);
            }
            else if (l == v5 && v2 == v3)
            {
                v3 = l;
            }
            else
            {
                v2 = 0xffffffff;
            }
        }
    }
    *((unsigned int *)&idx[2440]) = v2;
    *((unsigned int *)&idx[2444]) = v3;
    if (v1)
    {
        v25 = idx;
        v26 = v25;
        v27 = (long long)idx[104];
        v28 = v26;
        if ((long long)v28[32] - (long long)v28[24] < v27)
            _obstack_newchunk(v26, v27, v27);
        *((unsigned long *)&v26[24]) = (long long)v26[24] + v27;
        v29 = v25;
        v30 = (long long)v29[16];
        if (v30 == (long long)v29[24])
            *((char *)&v29[80]) = (char)v29[80] | 2;
        *((long long *)&v29[24]) = ~((long long)v29[48]) & (long long)v29[24] + (long long)v29[48];
        if ((long long)v29[24] - (long long)v29[8] > (long long)v29[32] - (long long)v29[8])
            *((long long *)&v29[24]) = (long long)v29[32];
        *((long long *)&v29[16]) = (long long)v29[24];
        *((unsigned long *)&idx[2416]) = v30;
        iter2 = (long long)idx[96];
        for (m = (long long)idx[104]; m > NULL; m -= 1)
        {
            *((char *)(m - 1 + (long long)idx[2416])) = iter2->field_8->field_18;
            iter2 = iter2->field_18;
        }
        if ((long long)idx[104] > 1)
        {
            v31 = idx;
            v32 = v31;
            v33 = ((long long)idx[104] - 1) * 8;
            v34 = v32;
            if ((long long)v34[32] - (long long)v34[24] < v33)
                _obstack_newchunk(v32, v33, v33);
            *((unsigned long long *)&v32[24]) = (long long)v32[24] + v33;
            v35 = v31;
            v36 = (long long)v35[16];
            if (v36 == (long long)v35[24])
                *((char *)&v35[80]) = (char)v35[80] | 2;
            *((long long *)&v35[24]) = ~((long long)v35[48]) & (long long)v35[24] + (long long)v35[48];
            if ((long long)v35[24] - (long long)v35[8] > (long long)v35[32] - (long long)v35[8])
                *((long long *)&v35[24]) = (long long)v35[32];
            *((long long *)&v35[16]) = (long long)v35[24];
            *((unsigned long *)&idx[2424]) = v36;
            iter2 = *((long long *)((long long)idx[96] + 24));
            for (n = 0; n < (long long)idx[104] - 1; n += 1)
            {
                *((unsigned long long *)((long long)idx[2424] + n * 8)) = iter2->field_30;
                iter2 = iter2->field_18;
            }
            *((char *)&idx[2448]) = sub_40c863(v13, *((char *)((long long)idx[104] - 2 + (long long)idx[2416])));
        }
    }
    if (!v13)
        return;
    for (i0 = 0; i0 <= 0xff; i0 += 1)
    {
        *((char *)idx + i0 + 112) = *((char *)v14 + sub_40c845(v13[i0]));
    }
    return;
}



// Function: bm_delta2_search @ 0xe78b
typedef struct struct_0 {
    char padding_0[2424];
    unsigned long long field_978;
} struct_0;

unsigned int bm_delta2_search(void* *a0, void* a1, unsigned long a2, long long a3, char *a4, char a5, char a6, char *a7, struct_0 *a8)
{
    char v4;  // al
    char v5;  // al
    void* iter;  // [bp-0x38]
    long long v1;  // [bp-0x30]
    long long v2;  // [bp-0x28]
    long long node;  // [bp-0x20]

    iter = *(a0);
    v1 = a3;
    v2 = 0;
    while (1)
    {
        node = 2;
        if (a6 == (char)sub_40c863(a4, *((char *)iter - 2)))
        {
            do
            {
                node += 1;
            } while (node <= v1 && (v4 = (char)(unsigned long long)sub_40c863(a4, *((char *)((char *)iter + -(node)))), v4 == (char)sub_40c863(a4, *((char *)(a2 + -(node))))));
            if (node > v1)
            {
                for (node = v2 + v1 + 1; node <= a3 && (v5 = (char)(unsigned long long)sub_40c863(a4, *((char *)((char *)iter + -(node)))), (char)(unsigned long long)sub_40c863(a4, *((char *)((char *)iter + -(node)))) == (char)sub_40c863(a4, *((char *)(a2 + -(node))))); node += 1);
                if (node > a3)
                {
                    *(a0) = -(a3) + iter;
                    return 1;
                }
            }
        }
        v1 = *((long long *)(node * 8 - 16 + a8->field_978));
        iter += v1;
        if (iter > a1)
            break;
        if (a5 != (char)sub_40c863(a4, *((char *)iter - 1)))
        {
            if (a7)
                iter += a7[(char)sub_40c845(*((char *)iter - 1))];
            break;
        }
        else
        {
            v2 = node - 1;
        }
    }
    *(a0) = iter;
    return 0;
}



// Function: memchr_kwset @ 0xe9c2
char * memchr_kwset(char *a0, unsigned long a1, void* a2)
{
    char *iter;  // [bp-0x30]
    int v1;  // [bp-0x1c]
    char *v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    iter = a0;
    if ((int)a2[2444] < 0)
    {
        for (v2 = &iter[a1]; iter < v2; iter += 1)
        {
            if (*((long long *)((char *)a2 + 8 * ((char)sub_40c845(*(iter)) + 46))))
                return iter;
        }
    }
    else
    {
        v1 = 2;
        v3 = v1 * 8;
        while (iter < v2)
        {
            if (*((long long *)((char *)a2 + 8 * ((char)sub_40c845(*(iter)) + 46))))
                return iter;
            iter += 1;
            if (!(iter % v3))
                return sub_4222e7(iter, (int)a2[2440], (int)a2[2444], v2 - iter);
        }
    }
    return NULL;
}



// Function: bmexec_trans @ 0xeae1
typedef struct struct_0 {
    char field_0;
    char padding_1[103];
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
    unsigned int field_988;
    char padding_98c[4];
    char field_990;
} struct_0;

void* bmexec_trans(struct_0 *idx, char *a1, unsigned long long a2)
{
    char v13;  // dl
    char v0;  // [bp-0x5a]
    char v1;  // [bp-0x59]
    unsigned int v2;  // [bp-0x58]
    int v3;  // [bp-0x54]
    char *iter;  // [bp-0x50], Other Possible Types: void*
    unsigned long long v5;  // [bp-0x48]
    unsigned long v6;  // [bp-0x40]
    unsigned long v7;  // [bp-0x38]
    struct_0 *v8;  // [bp-0x30]
    unsigned long long v9;  // [bp-0x28]
    void* i;  // [bp-0x20]
    void* v11;  // [bp-0x18]

    v6 = idx->field_68;
    v7 = idx->field_980;
    if (!v6)
    {
        return NULL;
    }
    else if (v6 > a2)
    {
        return 0xffffffffffffffff;
    }
    else if (v6 == 1)
    {
        iter = sub_40e9c2(a1, a2, idx);
        return (!iter ? 0xffffffffffffffff : iter - a1);
    }
    else
    {
        v8 = idx->padding_70;
        v9 = v6 + idx->field_970;
        iter = &a1[v6];
        v0 = idx->field_988;
        v1 = idx->field_990;
        v13 = 0;
        if (__OFSMUL__(v6, 12) & 1)
            v13 = 1;
        v5 = v6 * 12;
        if ((v13 & 1) != 1 && a2 > v5)
        {
            i = &a1[a2 + -11 * v6];
            while (i >= iter)
            {
                v11 = iter;
                v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                iter += v2;
                v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                iter += v2;
                if (v2)
                {
                    v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                    iter += v2;
                    v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                    iter += v2;
                    v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                    iter += v2;
                    if (v2)
                    {
                        v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                        iter += v2;
                        v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                        iter += v2;
                        v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                        iter += v2;
                        if (v2)
                        {
                            v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                            iter += v2;
                            v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
                            iter += v2;
                            v3 = 128;
                            if (v3 <= iter - v11)
                                continue;
                            iter -= 1;
                            iter = sub_40e9c2(iter, &a1[a2] - iter, idx);
                            if (!iter)
                                return 0xffffffffffffffff;
                            iter += 1;
                            if (i <= iter)
                                break;
                        }
                    }
                }
                if ((char)sub_40e78b(&iter, i, v9, v6, v7, v0, v1, v8, idx))
                    return iter - a1;
            }
        }
        i = &a1[a2];
        v2 = (&v8->field_0)[(char)sub_40c845(*((char *)iter - 1))];
        do
        {
            do
            {
                if (v2 > i - iter)
                    return 0xffffffffffffffff;
            } while ((iter += (long long)(int)v2, v2 = (unsigned int)(&v8->field_0)[(char)(char)sub_40c845(*((char *)((char *)iter - 1)))], v2));
        } while (!(char)sub_40e78b(&iter, i, v9, v6, v7, v0, v1, NULL, idx));
        return iter - a1;
    }
}



// Function: bmexec @ 0xf061
typedef struct struct_0 {
    char field_0;
    char padding_1[103];
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
    unsigned int field_988;
    char padding_98c[4];
    char field_990;
} struct_0;

void* bmexec(struct_0 *a0, char *a1, unsigned long long a2, struct_0 *idx, char a4)
{
    char v0;  // [bp-0x3c]
    void* v1;  // [bp-0x10]

    v0 = a4;
    v1 = (!a0->field_980 ? sub_40eae1(a0, a1, a2) : sub_40eae1(a0, a1, a2));
    *((unsigned long long *)&idx->field_0) = 0;
    *((void* *)&idx->padding_1[7]) = v1;
    *((unsigned long long *)&idx->padding_1[15]) = a0->field_68;
    return v1;
}



// Function: acexec_trans @ 0xf0f4
typedef struct struct_3 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
} struct_3;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[88];
    struct struct_1 *field_60;
    char padding_68[2328];
    unsigned long long field_980;
    int field_988;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    struct struct_1 *field_10;
    char field_18;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char padding_10[16];
    struct struct_1 *field_20;
    unsigned long long field_28;
} struct_1;

unsigned long long acexec_trans(struct_2 *index, char *a1, long long a2, struct_3 *idx, char a4)
{
    char *v13;  // rax
    char *v14;  // rax
    char *v15;  // rax
    char *v16;  // rax
    char k;  // [bp-0x56]
    char v1;  // [bp-0x55]
    int v2;  // [bp-0x54]
    struct_1 *iter1;  // [bp-0x50]
    struct_1 *node;  // [bp-0x48]
    char *v5;  // [bp-0x40]
    unsigned long long v6;  // [bp-0x38]
    struct_0 *idx1;  // [bp-0x30]
    struct_1 *iter;  // [bp-0x28]
    unsigned long v9;  // [bp-0x20]
    char *v10;  // [bp-0x18]
    unsigned long long v11;  // [bp-0x10]

    if (a2 < *((long long *)&index->padding_68[0]))
        return 0xffffffffffffffff;
    v9 = index->field_980;
    iter1 = index->field_60;
    v10 = &a1[a2];
    v5 = a1;
    if (!iter1->field_0)
    {
        v2 = index->field_988;
LABEL_40f17b:
        if (v2 < 0)
        {
            while (1)
            {
                v13 = v5;
                v5 = v13 + 1;
                k = sub_40c863(v9, *(v13));
                iter1 = (&index->field_0)[k + 46];
                if (iter1)
                    break;
                if (v5 >= v10)
                    return 0xffffffffffffffff;
            }
        }
        else
        {
            v5 = sub_40e9c2(v5, v10 - v5, index);
            if (!v5)
                return 0xffffffffffffffff;
            v14 = v5;
            v5 = v14 + 1;
            k = sub_40c863(v9, *(v14));
            iter1 = (&index->field_0)[k + 46];
        }
        for (; !iter1->field_0; iter1 = idx1->field_10)
        {
            if (v5 >= v10)
                return 0xffffffffffffffff;
            v15 = v5;
            v5 = v15 + 1;
            k = sub_40c863(v9, *(v15));
            idx1 = iter1->field_8;
            while (k != idx1->field_18)
            {
                idx1 = (k < idx1->field_18 ? idx1->field_0 : idx1->field_8);
                if (!idx1)
                {
                    iter1 = iter1->field_20;
                    if (!iter1)
                    {
                        iter1 = (&index->field_0)[k + 46];
                        if (v5 >= v10)
                            return 0xffffffffffffffff;
                        goto LABEL_40f17b;
                    }
                    else if (iter1->field_0)
                    {
                        v5 -= 1;
                        goto LABEL_40f366;
                    }
                    else
                    {
                        idx1 = iter1->field_8;
                    }
                }
            }
        }
    }
LABEL_40f366:
    for (node = iter1; node->field_0 < NULL; node = node->field_20);
    if (a4)
    {
        for (v6 = &v5[-(node->field_28)]; v5 < v10; node = iter)
        {
            v16 = v5;
            v5 = v16 + 1;
            v1 = sub_40c863(v9, *(v16));
            do
            {
                for (idx1 = iter1->field_8; idx1 && v1 != idx1->field_18; idx1 = (v1 < idx1->field_18 ? idx1->field_0 : idx1->field_8));
            } while (!idx1 && !(iter1 = (struct_1 *)iter1->field_20, !iter1) && node->field_28 <= iter1->field_28);
            if (!idx1)
                break;
            iter1 = idx1->field_10;
            if (!iter1->field_0)
                continue;
            for (iter = iter1; iter->field_0 < NULL; iter = iter->field_20);
            v11 = &v5[-(iter->field_28)];
            if (v11 > v6)
                continue;
            v6 = v11;
        }
    }
    idx->field_0 = (long long)(node->field_0) >> 1;
    idx->field_8 = v6 - a1;
    idx->field_10 = node->field_28;
    return v6 - a1;
}



// Function: acexec @ 0xf50c
typedef struct struct_3 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
} struct_3;

typedef struct struct_2 {
    struct struct_1 *field_0;
    char padding_8[88];
    struct struct_1 *field_60;
    char padding_68[2328];
    unsigned long long field_980;
    int field_988;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    struct struct_0 *field_8;
    struct struct_1 *field_10;
    char field_18;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char padding_10[16];
    struct struct_1 *field_20;
    unsigned long long field_28;
} struct_1;

unsigned long long acexec(struct_2 *a0, char *a1, long long a2, struct_3 *a3, char a4)
{
    return (!a0->field_980 ? sub_40f0f4(a0, a1, a2, a3, a4) : sub_40f0f4(a0, a1, a2, a3, a4));
}



// Function: kwsexec @ 0xf585
typedef struct struct_0 {
    char padding_0[2456];
    struct struct_1 *field_998;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

int kwsexec(struct_0 *a0, unsigned long a1, unsigned long a2, unsigned long a3, char a4)
{
    return a0->field_998(a0, a1, a2, a3, a4, a0->field_998);
}



// Function: kwsfree @ 0xf5d1
void kwsfree(unsigned long long *ptr)
{
    long long v0;  // [bp-0x10]

    v0 = 0;
    if (v0 > ptr[1] && v0 < ptr[4])
    {
        ptr[2] = v0;
        ptr[3] = ptr[2];
        free(ptr);
        return;
    }
    _obstack_free(ptr, v0, v0);
    free(ptr);
    return;
}



// Function: wordchar @ 0xf73c
unsigned int wordchar(int a0)
{
    unsigned int v1;  // eax

    if (a0 == 95 || iswalnum(a0))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: wordinit @ 0xf770
extern unsigned int g_4331a4[4];
extern char g_4339e0;

unsigned int wordinit(void)
{
    unsigned int v2;  // eax
    int i;  // [bp-0xc]

    for (i = 0; i <= 0xff; i += 1)
    {
        *(&(&g_4339e0)[i]) = sub_40f73c(g_4331a4[64 + i]);
    }
    return v2;
}



// Function: kwsinit @ 0xf7c8
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
    char padding_51[7];
    unsigned long long field_58;
    void* field_60;
    unsigned long long field_68;
    char padding_70[2304];
    unsigned long long field_970;
    char padding_978[8];
    unsigned long long field_980;
} struct_0;

extern char g_433180;

struct_0 * kwsinit(char a0)
{
    int i;  // [bp-0x14]
    char *v1;  // [bp-0x10]

    v1 = NULL;
    if (g_433180 && (__ctype_get_mb_cur_max() == 1 || a0))
    {
        v1 = sub_426b10(0x100);
        for (i = 0; i <= 0xff; i += 1)
        {
            v1[i] = toupper(i) & 0xffffffff;
        }
    }
    return sub_40c89b(v1);
}



// Function: mb_goback @ 0xf84a
typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_4331a2;

char * mb_goback(struct_0 **a0, unsigned long long *a1, char *a2, unsigned long a3)
{
    char v0;  // [bp-0x3d]
    int i;  // [bp-0x3c]
    char *v2;  // [bp-0x38]
    char *v3;  // [bp-0x30]
    unsigned long long v4;  // [bp-0x28]
    unsigned long long v5;  // [bp-0x20]
    char v6;  // [bp-0x18], Other Possible Types: unsigned long

    v2 = &*(a0)->field_0;
    v3 = v2;
    if (a2 <= v2)
        return a2 - v2;
    if (g_4331a2)
    {
        v2 = a2;
        if ((*(a2) & 192) == 128)
        {
            for (i = 1; i <= 3; i += 1)
            {
                if ((a2[-(i)] & 192) != 128)
                {
                    v0 = !((int)(~(a2[-(i)]) & 0xff) >> ((char)(7 - i) & 31));
                    if (v0)
                    {
                        v6 = 0;
                        v5 = sub_40f686(&a2[-(i)], a3 - &a2[-(i)], &v6, &a2[-(i)]);
                        if (!(v5 - 0 >> 63 & 1))
                        {
                            v3 = &a2[-(i)];
                            v2 = &v3[v5];
                        }
                    }
                    break;
                }
            }
        }
    }
    else
    {
        v6 = 0;
        do
        {
            v4 = sub_40f6da(v2, a3 - v2, &v6, a3 - v2);
            if (v4 - 0 >> 63 & 1)
            {
                v4 = 1;
                memset(&v6, 0, 8);
            }
        } while ((v3 = v2, v2 += v4, v2 < a2));
        if (a1)
            *(a1) = v4;
    }
    *(a0) = v2;
    return (v2 == a2 ? NULL : a2 - v3);
}



// Function: wordchars_count @ 0xfa82
extern char g_4331a3;
extern char g_4339e0;

long long wordchars_count(unsigned long a0, unsigned long a1, char a2)
{
    char v0;  // [bp-0x2d]
    unsigned int v1;  // [bp-0x2c]
    long long iter;  // [bp-0x28]
    unsigned long long v3;  // [bp-0x20]
    unsigned long v4;  // [bp-0x18]

    iter = 0;
    v4 = 0;
    while (iter < a1 - a0)
    {
        v0 = *((char *)(a0 + iter));
        if (*((char *)(unsigned int)&(&g_4339e0)[v0]))
        {
            iter += 1;
        }
        else if (*((char *)(unsigned int)&(&g_4331a3)[v0]) == 254)
        {
            v1 = 0;
            v3 = sub_421b80(&v1, iter + a0, a1 - a0 - iter, &v4);
            if ((char)sub_40f73c(v1) != 1)
                return iter;
            iter += (!v3) + v3;
        }
        else
        {
            return iter;
        }
        if (a2 != 1)
            return iter;
    }
    return iter;
}



// Function: wordchars_size @ 0xfbb2
long long wordchars_size(unsigned int a0, unsigned int a1)
{
    return sub_40fa82(a0, a1, 1);
}



// Function: wordchar_next @ 0xfbe0
long long wordchar_next(unsigned int a0, unsigned int a1)
{
    return sub_40fa82(a0, a1, 0);
}



// Function: wordchar_prev @ 0xfc0e
extern char g_4331a0;
extern char g_4331a2;
extern char g_4339e0;

long long wordchar_prev(void* a0, void* a1, unsigned int a2)
{
    void* iter;  // [bp-0x38], Other Possible Types: unsigned long long
    char v1;  // [bp-0x19]

    iter = a1;
    if (a0 == iter)
        return 0;
    iter -= 1;
    v1 = *((char *)iter);
    if (g_4331a0 != 1 || ~(v1 >> 7) & g_4331a2)
        return *((char *)(unsigned int)&(&g_4339e0)[v1]);
    iter += -(sub_40f84a(&a0, NULL, iter, a2));
    return sub_40fbe0(iter, a2);
    return *((char *)(unsigned int)&(&g_4339e0)[v1]);
}



// Function: private_malloc @ 0xfd0f
long long private_malloc(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    if (a0 >= 0)
        return sub_426b10(a0);
    sub_427122(); /* do not return */
}



// Function: private_free @ 0xfd3f
void private_free(void* a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x18]

    v0 = a1;
    free(a0);
    return;
}



// Function: jit_exec @ 0xfd62
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    long long field_18;
    long long field_20;
    unsigned long long field_28;
} struct_0;

long long jit_exec(struct_0 *idx, long long a1, long long a2, long long a3, unsigned int a4)
{
    char v7;  // cl
    unsigned int v0;  // [bp-0x34]
    int v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x2c]
    unsigned long long v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    unsigned long long v5;  // [bp-0x18]

    while (1)
    {
        v1 = 0x2000;
        v3 = (v1 <= 0 ? -(v1) : 0x7fffffffffffffff);
        v2 = pcre2_match_8(idx->field_8, a1, a2, a3, a4, idx->field_18, idx->field_10);
        if (v2 == 0xffffffd2 && idx->field_28 <= (long long)(v3 + (v3 >> 63)) >> 1)
        {
            v4 = idx->field_28;
            idx->field_28 = v4 * 2;
            v5 = idx->field_28;
            pcre2_jit_stack_free_8(idx->field_20);
            idx->field_20 = pcre2_jit_stack_create_8(v4, v5, idx->field_0, v5);
            if (!idx->field_20)
                sub_427122(); /* do not return */
            if (!idx->field_10)
                idx->field_10 = pcre2_match_context_create_8(idx->field_0);
            pcre2_jit_stack_assign_8(idx->field_10, 0, idx->field_20);
        }
        else if (v2 == 0xffffffcb)
        {
            pcre2_config_8(7, &v0);
            v7 = 0;
            if (v0 & 0x80000000)
                v7 = 1;
            v0 *= 2;
            if (v7 & 1)
                return v2;
            if (!idx->field_10)
                idx->field_10 = pcre2_match_context_create_8(idx->field_0);
            pcre2_set_depth_limit_8(idx->field_10, v0, v0);
        }
        else
        {
            return v2;
        }
    }
}



// Function: bad_utf8_from_pcre2 @ 0xff6e
unsigned int bad_utf8_from_pcre2(unsigned long a0)
{
    return 0;
}



// Function: Pcompile @ 0xff80
typedef struct struct_0 {
    unsigned long long field_0;
    long long field_8;
    char padding_10[8];
    unsigned long long field_18;
    char padding_20[16];
    unsigned int field_30;
    unsigned int field_34;
} struct_0;

extern long long g_42a907;
extern char g_433180;
extern char g_433181;
extern char g_433182;
extern char g_4331a0;
extern char g_4331a2;

struct_0 * Pcompile(unsigned long a0, unsigned long a1, unsigned long long a2, char a3)
{
    long long v19;  // rax
    char v0;  // [bp-0x194]
    unsigned long long v1;  // [bp-0x190]
    unsigned long v2;  // [bp-0x188]
    unsigned long v3;  // [bp-0x180]
    char v4;  // [bp-0x178], Other Possible Types: unsigned int
    unsigned int v5;  // [bp-0x174]
    char v6;  // [bp-0x170]
    unsigned long v7;  // [bp-0x168]
    void* v8;  // [bp-0x160]
    struct_0 *idx;  // [bp-0x158]
    unsigned long v10;  // [bp-0x150]
    long long v11;  // [bp-0x148]
    unsigned long v12;  // [bp-0x140]
    unsigned long v13;  // [bp-0x138]
    void* v14;  // [bp-0x130]
    char v15;  // [bp-0x128]

    v3 = a0;
    v2 = a1;
    v1 = a2;
    v0 = a3;
    v5 = (!g_433180 ? 16 : 24);
    v8 = v3 + v2;
    idx = sub_426b10(56);
    idx->field_0 = pcre2_general_context_create_8(sub_40fd0f, sub_40fd3f, 0);
    v10 = idx->field_0;
    v11 = pcre2_compile_context_create_8(v10);
    if (g_4331a0)
    {
        if (g_4331a2 != 1)
            error(2, 0, gettext("-P supports only unibyte and UTF-8 locales"));
        v5 |= 0x80000;
        v5 |= 0x4000000;
    }
    if (v8 != rawmemchr(v3, 10))
        error(2, 0, gettext("the -P option only supports a single pattern"));
    v7 = 0;
    if (g_433182)
    {
        pcre2_set_compile_extra_options_8(v11, 8);
    }
    else if (g_433181)
    {
        v12 = v2 + 0x11;
        v7 = sub_426b10(v12);
        v13 = v7;
        v14 = mempcpy(v13, "(?<!\\w)(?:)(?!\\w)invalid argument %s for %s", 10);
        v14 = mempcpy(v14, v3, v2);
        memcpy(v14, ")(?!\\w)invalid argument %s for %s", 7);
        v3 = v13;
        v2 = v12;
    }
    v19 = pcre2_maketables_8(v10);
    pcre2_set_character_tables_8(v11, v19, v19);
    idx->field_8 = pcre2_compile_8(v3, v2, v5, &v4, &v6, v11);
    if (!idx->field_8)
    {
        pcre2_get_error_message_8(v4, &v15, 0x100, &v15);
        error(2, 0, "%s");
    }
    free(v7);
    pcre2_compile_context_free_8(v11);
    *((unsigned long long *)&idx->padding_10[0]) = 0;
    idx->field_18 = pcre2_match_data_create_from_pattern_8(idx->field_8, v10, v10);
    v4 = pcre2_jit_compile_8(idx->field_8, 1, idx);
    if (v4 && v4 != 0xffffffd3 && v4 != 0xffffffd0)
        error(2, 0, gettext("JIT internal error: %d"));
    *((unsigned long long *)&idx->padding_20[0]) = 0;
    *((unsigned long long *)&idx->padding_20[8]) = 0x8000;
    idx->field_30 = sub_40fd62(idx, &g_42a907, 0, 0, 1);
    idx->field_34 = sub_40fd62(idx, &g_42a907, 0, 0, 0);
    return idx;
}



// Function: Pexecute @ 0x10409
typedef struct struct_0 {
    long long field_0;
    long long field_8;
    long long field_10;
    long long field_18;
    long long field_20;
    unsigned long long field_28;
} struct_0;

extern char g_433183;
extern char g_4331a3;

unsigned long long Pexecute(struct_0 *a0, void* a1, unsigned long a2, unsigned long long *a3, void* a4)
{
    void* v17;  // rax
    void* *v18;  // rax
    void* v0;  // [bp-0xb0]
    void* v1;  // [bp-0x98]
    char i;  // [bp-0x81]
    int v3;  // [bp-0x80], Other Possible Types: unsigned int
    unsigned int v4;  // [bp-0x7c]
    void* iter;  // [bp-0x78]
    void* v6;  // [bp-0x70]
    void* node;  // [bp-0x68]
    void* v8;  // [bp-0x60]
    void* v9;  // [bp-0x58]
    void* *idx;  // [bp-0x48]
    void* ptr;  // [bp-0x40]
    void* v12;  // [bp-0x38]
    void* v13;  // [bp-0x30]
    void* v14;  // [bp-0x28]
    void* v15;  // [bp-0x20]

    v1 = a1;
    v0 = a4;
    if (v0)
        v17 = v0;
    else
        v17 = v1;
    iter = v17;
    i = *((char *)iter - 1) == g_433183;
    v6 = v1;
    v3 = 0xffffffff;
    idx = pcre2_get_ovector_pointer_8(a0->field_18);
    node = v1;
    do
    {
        ptr = rawmemchr(iter, g_433183);
        while (1)
        {
            for (; *((char *)(unsigned int)&(&g_4331a3)[sub_40f64d(*((char *)iter))]) == 0xff; i = 0)
            {
                iter += 1;
                node = iter;
            }
            v12 = iter - node;
            if (iter == ptr)
            {
                v18 = idx + 1;
                *(v18) = v12;
                *(idx) = *(v18);
                v3 = *((int *)((char *)a0 + 4 * i + 48));
                break;
            }
            else
            {
                v4 = 0;
                if (i != 1)
                    v4 |= 1;
                v3 = sub_40fd62(a0, node, ptr - node, v12, v4);
                if ((char)sub_40ff6e(v3) != 1)
                    break;
                v13 = pcre2_get_startchar_8(a0->field_18);
                if (v12 <= v13)
                {
                    if (!v13)
                    {
                        *(idx) = v13;
                        idx[1] = NULL;
                        v3 = *((int *)((char *)a0 + 4 * i + 48));
                    }
                    else
                    {
                        v3 = sub_40fd62(a0, node, v13, v12, v4 | 1073741826);
                    }
                    if (v3 != -0x1)
                        break;
                    iter = node + v13 + 1;
                    i = 0;
                }
                node += v13 + 1;
            }
        }
    } while (v3 == -0x1 && (i = 1, v6 = ptr + 1, node = v6, iter = node, iter < v1 + a2));
    if (v3 <= 0)
    {
        if (v3 <= -0x2e)
        {
            if (v3 >= -0x3f)
            {
                switch (v3)
                {
                case -63:
                    sub_406eac();
                    error(2, 0, gettext("%s: exceeded PCRE's heap limit"));
                case -53:
                    sub_406eac();
                    error(2, 0, gettext("%s: exceeded PCRE's nested backtracking limit"));
                case -52:
                    sub_406eac();
                    error(2, 0, gettext("%s: PCRE detected recurse loop"));
                case -48:
                    sub_406eac();
                    error(2, 0, gettext("%s: memory exhausted"));
                case -47:
                    sub_406eac();
                    error(2, 0, gettext("%s: exceeded PCRE's backtracking limit"));
                case -46:
                    sub_406eac();
                    error(2, 0, gettext("%s: exhausted PCRE JIT stack"));
                default:
                    sub_406eac();
                }
            }
            else
            {
                sub_406eac();
            }
        }
        else
        {
            if (v3 != -0x1)
                sub_406eac();
            else
                return 0xffffffffffffffff;
        }
        error(2, 0, gettext("%s: internal PCRE error: %d"));
    }
    else
    {
        v14 = node + *(idx);
        v15 = node + idx[1];
        if (v0)
        {
            v8 = v14;
            v9 = v15;
        }
        else
        {
            v8 = v6;
            v9 = ptr + 1;
        }
        *(a3) = v9 - v8;
        return v8 - v1;
    }
}



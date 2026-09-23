// Function: discard_confusing_lines @ 0x6122
typedef struct struct_0 {
    char padding_0[200];
    long long field_c8;
    char padding_d0[40];
    unsigned long long field_f8;
    char padding_100[40];
    unsigned long long field_128;
    char padding_130[200];
    long long field_1f8;
    char padding_200[40];
    unsigned long long field_228;
} struct_0;

extern char g_42d348;

void discard_confusing_lines(struct_0 *index)
{
    unsigned long long *v24;  // rax
    unsigned long long *v25;  // rax
    unsigned long v26;  // r13
    unsigned long iter;  // rbx
    unsigned long long iter1;  // rbx
    unsigned long long v29;  // r12
    unsigned long long i0;  // rbx
    unsigned long long v31;  // r12
    unsigned long long i1;  // rbx
    unsigned long long v33;  // r12
    int node;  // [bp-0xe4]
    long long j;  // [bp-0xe0], Other Possible Types: unsigned long
    unsigned long iter2;  // [bp-0xd8], Other Possible Types: unsigned long long
    unsigned long long v3;  // [bp-0xd0]
    unsigned long long v4;  // [bp-0xc8]
    unsigned long long l;  // [bp-0xc0]
    unsigned long long v6;  // [bp-0xb8]
    long long v7;  // [bp-0xb0]
    unsigned long long idx;  // [bp-0xa8]
    unsigned long long v9;  // [bp-0xa8]
    unsigned long v10;  // [bp-0xa0]
    unsigned long v11;  // [bp-0x98]
    unsigned long v12;  // [bp-0x90]
    unsigned long long m;  // [bp-0x88]
    unsigned long v14;  // [bp-0x80]
    unsigned long v15;  // [bp-0x78]
    unsigned long v16;  // [bp-0x70]
    unsigned long v17;  // [bp-0x68]
    unsigned long long v18;  // [bp-0x60]
    unsigned long v19;  // [bp-0x58]
    unsigned long long v20;  // [bp-0x50]
    unsigned long ptr;  // [bp-0x48]
    unsigned long long v22;  // [bp-0x40]

    iter2 = sub_41c945((index->field_1f8 + index->field_c8) * 16);
    for (node = 0; node <= 1; node += 1)
    {
        *((unsigned long long *)&index->padding_0[304 * node + 0x100]) = iter2;
        iter2 += (&index->field_c8)[38 * node] * 8;
        *((unsigned long long *)&index->padding_0[304 * node + 264]) = iter2;
        iter2 += (&index->field_c8)[38 * node] * 8;
    }
    iter2 = sub_413030(index->field_128 * 16);
    ptr = iter2;
    v22 = iter2 + index->field_128 * 8;
    for (j = 0; j < index->field_c8; j += 1)
    {
        v24 = *((long long *)(j * 8 + index->field_f8)) * 8 + ptr;
        *(v24) = *(v24) + 1;
    }
    for (j = 0; j < index->field_1f8; j += 1)
    {
        v25 = *((long long *)(j * 8 + index->field_228)) * 8 + v22;
        *(v25) = *(v25) + 1;
    }
    v19 = sub_413030(index->field_1f8 + index->field_c8);
    v20 = index->field_c8 + v19;
    for (node = 0; node <= 1; node += 1)
    {
        v14 = (&index->field_c8)[38 * node];
        v15 = (&v19)[node];
        v16 = (&ptr)[1 + -1 * node];
        v17 = (&index->field_f8)[38 * node];
        v3 = 5;
        v4 = v14 >> 6;
        while (1)
        {
            v4 >>= 2;
            if (!v4)
                break;
            v3 *= 2;
        }
        for (j = 0; v14 > j; j += 1)
        {
            if (*((long long *)(v17 + j * 8)))
            {
                v18 = *((long long *)(v16 + *((long long *)(v17 + j * 8)) * 8));
                if (!v18)
                {
                    *((char *)(v15 + j)) = 1;
                }
                else if (v3 < v18)
                {
                    *((char *)(v15 + j)) = 2;
                }
            }
        }
    }
    for (node = 0; node <= 1; node += 1)
    {
        v12 = (&index->field_c8)[38 * node];
        v26 = (&v19)[node];
        for (j = 0; j < v12; j += 1)
        {
            if (*((char *)(j + v26)) == 2)
            {
                *((char *)(j + v26)) = 0;
            }
            else if (*((char *)(j + v26)))
            {
                l = 0;
                for (iter = j; iter < v12 && *((char *)(iter + v26)); iter += 1)
                {
                    if (*((char *)(iter + v26)) == 2)
                        l += 1;
                }
                for (; iter > j && *((char *)(iter - 1 + v26)) == 2; l -= 1)
                {
                    *((char *)(iter - 1 + v26)) = 0;
                }
                if (m < l * 4)
                {
                    for (m = iter - j; iter > j; *((char *)(iter + v26)) = 0)
                    {
                        iter -= 1;
                        if (*((char *)(iter + v26)) != 2)
                            continue;
                    }
                }
                else
                {
                    v6 = 1;
                    v7 = (long long)(m) >> 2;
                    while (1)
                    {
                        v7 >>= 2;
                        if (v7 <= 0)
                            break;
                        v6 *= 2;
                    }
                    v6 += 1;
                    iter1 = 0;
                    for (v29 = 0; iter1 < m; iter1 += 1)
                    {
                        if (*((char *)(j + iter1 + v26)) != 2)
                        {
                            v29 = 0;
                        }
                        else
                        {
                            v29 += 1;
                            if (v29 == v6)
                            {
                                iter1 -= v29;
                            }
                            else if (v6 < v29)
                            {
                                *((char *)(j + iter1 + v26)) = 0;
                            }
                        }
                    }
                    i0 = 0;
                    for (v31 = 0; i0 < m && (i0 <= 7 || *((char *)(j + i0 + v26)) != 1); i0 += 1)
                    {
                        if (*((char *)(j + i0 + v26)) == 2)
                        {
                            v31 = 0;
                            *((char *)(j + i0 + v26)) = 0;
                        }
                        else
                        {
                            v31 = (!*((char *)(j + i0 + v26)) ? 0 : v31 + 1);
                        }
                        if (v31 == 3)
                            break;
                    }
                    j += m - 1;
                    i1 = 0;
                    for (v33 = 0; i1 < m && (i1 <= 7 || *((char *)(j - i1 + v26)) != 1); i1 += 1)
                    {
                        if (*((char *)(j - i1 + v26)) == 2)
                        {
                            v33 = 0;
                            *((char *)(j - i1 + v26)) = 0;
                        }
                        else
                        {
                            v33 = (!*((char *)(j - i1 + v26)) ? 0 : v33 + 1);
                        }
                        if (v33 == 3)
                            break;
                    }
                }
            }
        }
    }
    for (node = 0; node <= 1; node += 1)
    {
        v10 = (&v19)[node];
        v11 = (&index->field_c8)[38 * node];
        idx = 0;
        for (j = 0; j < v11; j += 1)
        {
            if (g_42d348 || !*((char *)(v10 + j)))
            {
                *((long long *)(*((long long *)&index->padding_0[304 * node + 0x100]) + idx * 8)) = *((long long *)((&index->field_f8)[38 * node] + j * 8));
                v9 = idx + 1;
                *((unsigned long *)(*((long long *)&index->padding_0[304 * node + 264]) + idx * 8)) = j;
                idx = v9;
            }
            else
            {
                *((char *)(j + *((long long *)&index->padding_0[304 * node + 280]))) = 1;
            }
        }
        *((unsigned long long *)&index->padding_0[304 * node + 272]) = idx;
    }
    free(v19);
    free(ptr);
    return;
}



// Function: shift_boundaries @ 0x6b31
long long shift_boundaries(unsigned long a0)
{
    long long v13;  // rax
    int i;  // [bp-0x54]
    long long iter;  // [bp-0x50]
    unsigned long long iter1;  // [bp-0x48]
    unsigned long long v3;  // [bp-0x48]
    unsigned long long v4;  // [bp-0x48]
    long long node;  // [bp-0x40]
    long long l;  // [bp-0x38]
    unsigned long v7;  // [bp-0x30]
    unsigned long v8;  // [bp-0x28]
    unsigned long v9;  // [bp-0x20]
    long long v10;  // [bp-0x18]
    unsigned long long k;  // [bp-0x10]

    for (i = 0; i <= 1; i += 1)
    {
        v7 = *((long long *)(a0 + i * 304 + 280));
        v8 = *((long long *)(a0 + (1 - i) * 304 + 280));
        v9 = *((long long *)(a0 + i * 304 + 248));
        iter = 0;
        iter1 = 0;
        v10 = *((long long *)(a0 + i * 304 + 200));
        while (1)
        {
            for (; iter < v10 && !*((char *)(v7 + iter)); iter1 = v4)
            {
                do
                {
                    v3 = iter1;
                    v4 = v3 + 1;
                    iter1 = v4;
                } while (*((char *)(v8 + v3)));
                iter += 1;
            }
            if (iter == v10)
                break;
            node = iter;
            do
            {
                iter += 1;
            } while (*((char *)(v7 + iter)));
            for (; *((char *)(v8 + iter1)); iter1 += 1);
            do
            {
                k = iter - node;
                while (node && *((long long *)(v9 + node * 8 - 8)) == *((long long *)(v9 + iter * 8 - 8)))
                {
                    node -= 1;
                    *((char *)(v7 + node)) = 1;
                    iter -= 1;
                    for (*((char *)(v7 + iter)) = 0; *((char *)(v7 + node - 1)); node -= 1);
                    do
                    {
                        iter1 -= 1;
                    } while (*((char *)(v8 + iter1)));
                }
                l = (!*((char *)(v8 + iter1 - 1)) ? v10 : iter);
                while (iter != v10 && *((long long *)(v9 + node * 8)) == *((long long *)(v9 + iter * 8)))
                {
                    node += 1;
                    *((char *)(v7 + node)) = 0;
                    iter += 1;
                    for (*((char *)(v7 + iter)) = 1; *((char *)(v7 + iter)); iter += 1);
                    while (1)
                    {
                        iter1 += 1;
                        if (!*((char *)(v8 + iter1)))
                            break;
                        l = iter;
                    }
                }
            } while (k != iter - node);
            while (l < iter)
            {
                node -= 1;
                *((char *)(v7 + node)) = 1;
                iter -= 1;
                *((char *)(v7 + iter)) = 0;
                do
                {
                    iter1 -= 1;
                } while (*((char *)(v8 + iter1)));
            }
        }
    }
    return v13;
}



// Function: add_change @ 0x6e92
unsigned long long * add_change(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4)
{
    unsigned long long *idx;  // [bp-0x10]

    idx = sub_41c945(48);
    idx[3] = a0;
    idx[4] = a1;
    idx[1] = a3;
    idx[2] = a2;
    *(idx) = a4;
    return idx;
}



// Function: build_reverse_script @ 0x6f01
typedef struct struct_0 {
    char padding_0[200];
    long long field_c8;
    char padding_d0[72];
    unsigned long long field_118;
    char padding_120[216];
    long long field_1f8;
    char padding_200[72];
    unsigned long long field_248;
} struct_0;

unsigned long long * build_reverse_script(struct_0 *idx)
{
    unsigned long long *v0;  // [bp-0x50]
    long long iter;  // [bp-0x48]
    long long node;  // [bp-0x40]
    unsigned long v3;  // [bp-0x38]
    unsigned long v4;  // [bp-0x30]
    long long v5;  // [bp-0x28]
    long long v6;  // [bp-0x20]
    long long v7;  // [bp-0x18]
    long long v8;  // [bp-0x10]

    v0 = NULL;
    v3 = idx->field_118;
    v4 = idx->field_248;
    v5 = idx->field_c8;
    v6 = idx->field_1f8;
    iter = 0;
    for (node = 0; iter < v5 || node < v6; node += 1)
    {
        if (*((char *)(v4 + node)) || *((char *)(v3 + iter)))
        {
            v7 = iter;
            for (v8 = node; *((char *)(v3 + iter)); iter += 1);
            for (; *((char *)(v4 + node)); node += 1);
            v0 = sub_406e92(v7, v8, iter - v7, node - v8, v0);
        }
        iter += 1;
    }
    return v0;
}



// Function: build_script @ 0x7037
unsigned long long * build_script(unsigned long long *idx)
{
    unsigned long long *v0;  // [bp-0x40]
    unsigned long iter;  // [bp-0x38]
    unsigned long node;  // [bp-0x30]
    unsigned long v3;  // [bp-0x28]
    unsigned long v4;  // [bp-0x20]
    unsigned long v5;  // [bp-0x18]
    unsigned long v6;  // [bp-0x10]

    v0 = NULL;
    v3 = idx[35];
    v4 = idx[73];
    iter = idx[25];
    for (node = idx[63]; !(iter - 0 >> 63 & 1) || !(node - 0 >> 63 & 1); node -= 1)
    {
        if (*((char *)(v4 + node - 1)) || *((char *)(v3 + iter - 1)))
        {
            v5 = iter;
            for (v6 = node; *((char *)(v3 + iter - 1)); iter -= 1);
            for (; *((char *)(v4 + node - 1)); node -= 1);
            v0 = sub_406e92(iter, node, v5 - iter, v6 - node, v0);
        }
        iter -= 1;
    }
    return v0;
}



// Function: briefly_report @ 0x7167
extern unsigned long long g_42d200;
extern unsigned long long g_42d208;
extern char g_42d2a0;

long long briefly_report(unsigned int a0, unsigned long long *a1)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax

    if (!a0)
        return v2;
    v1 = (!g_42d2a0 ? gettext("Binary files %s and %s differ\n") : gettext("Files %s and %s differ\n"));
    return sub_4109a8(v1, (!g_42d200 ? a1[1] : g_42d200), (!g_42d208 ? a1[39] : g_42d208));
}



// Function: diff_2_files @ 0x7206
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern unsigned int g_42d1c0;
extern char g_42d1c8;
extern char g_42d1ec;
extern char g_42d1ed;
extern unsigned long long g_42d200[4];
extern unsigned long long g_42d208;
extern unsigned long long g_42d280;
extern char g_42d2a0;
extern char g_42d338;
extern char g_42d348;
extern unsigned long long g_42d360;
extern unsigned long long g_42d490;

long long diff_2_files(void* a0)
{
    long long v26;  // rax
    unsigned long long *node;  // rdi
    unsigned long long v28;  // rcx
    void* iter;  // rsi
    void* l;  // rdx
    unsigned long long v31;  // rcx
    unsigned long long *iter1;  // rdi
    void* idx;  // [bp-0xe0]
    int iter2;  // [bp-0xd0]
    unsigned int m;  // [bp-0xcc]
    char v3;  // [bp-0xc8]
    char v4;  // [bp-0xc0]
    char v5;  // [bp-0xb8]
    char v6;  // [bp-0xb0]
    struct_0 **ptr;  // [bp-0xa8]
    struct_0 **v8;  // [bp-0xa0]
    long long j;  // [bp-0x98]
    unsigned long long v10;  // [bp-0x90]
    struct_0 **v11;  // [bp-0x88]
    unsigned long v12;  // [bp-0x80]
    void* v13;  // [bp-0x78]
    struct_0 **v14;  // [bp-0x70]
    struct_0 **v15;  // [bp-0x68]
    struct_0 **v16;  // [bp-0x60]
    long long v17;  // [bp-0x58]
    unsigned long v18;  // [bp-0x50]
    unsigned long v19;  // [bp-0x48]
    unsigned long v20;  // [bp-0x40]
    unsigned long long v21;  // [bp-0x38]
    unsigned long long v22;  // [bp-0x30]
    char v23;  // [bp-0x28]
    unsigned long long v24;  // [bp-0x20]

    idx = a0;
    if ((char)sub_40fa13(idx, g_42d1ed, g_42d1ed))
    {
        if ((long long)idx[64] != (long long)idx[368] && (long long)idx[64] > NULL && (long long)idx[368] > NULL && (*((int *)idx) < 0 || ((short)(int)idx[40] & 0xf000) == 0x8000) && ((int)idx[304] < 0 || ((short)(int)idx[344] & 0xf000) == 0x8000))
        {
            m = 1;
        }
        else if (*((int *)idx) == (int)idx[304])
        {
            m = 0;
        }
        else
        {
            v17 = 0x7ffffffffffffffe;
            v26 = sub_41d2a2((long long)idx[72], (long long)idx[376], v17, (long long)idx[72]);
            v18 = sub_41d2a2(8, v26, v17, v26);
            for (iter2 = 0; iter2 <= 1; iter2 += 1)
            {
                *((unsigned long long *)(304 * iter2 + (char *)idx + 160)) = sub_41c991(*((long long *)(304 * iter2 + (char *)idx + 160)), v18, v18, idx);
            }
            while (1)
            {
                for (iter2 = 0; iter2 <= 1; iter2 += 1)
                {
                    if (*((int *)(304 * iter2 + (char *)idx)) >= 0)
                        sub_40de51(iter2 * 304 + idx, v18 - *((long long *)(304 * iter2 + (char *)idx + 176)), idx, v18 - *((long long *)(304 * iter2 + (char *)idx + 176)));
                }
                if ((long long)idx[176] != (long long)idx[480] || memcmp((long long)idx[160], (long long)idx[464], (long long)idx[176]))
                {
                    m = 1;
                    break;
                }
                else if (v18 != (long long)idx[176])
                {
                    m = 0;
                    break;
                }
                else
                {
                    *((unsigned long *)&idx[480]) = 0;
                    *((long long *)&idx[176]) = (long long)idx[480];
                }
            }
        }
        sub_407167(m, idx);
    }
    else
    {
        v12 = (long long)idx[504] + (long long)idx[200] + 4;
        v13 = sub_413030(v12);
        *((void* *)&idx[280]) = v13 + 1;
        *((void* *)&idx[584]) = (long long)idx[200] + 3 + v13;
        sub_406122(idx);
        v19 = (long long)idx[0x100];
        v20 = (long long)idx[560];
        j = (long long)idx[576] + (long long)idx[272] + 3;
        v21 = sub_41c945(j * 16);
        v22 = j * 8 + v21;
        v21 += ((long long)idx[576] + 1) * 8;
        v22 += ((long long)idx[576] + 1) * 8;
        v23 = g_42d338;
        for (v10 = 1; j; j >>= 2)
        {
            v10 *= 2;
        }
        if (v10 < 0x1000)
            v10 = 0x1000;
        v24 = v10;
        node = &g_42d360;
        v28 = 38;
        for (iter = idx; v28; iter += 8)
        {
            v28 -= 1;
            *(node) = *((long long *)iter);
            node += 1;
        }
        l = idx + 304;
        v31 = 38;
        for (iter1 = &g_42d490; v31; l += 8)
        {
            v31 -= 1;
            *(iter1) = *((long long *)l);
            iter1 += 1;
        }
        sub_405d78(0, (long long)idx[272], 0, (long long)idx[576], g_42d348, &v19);
        free(0xfffffffffffffff8 - (long long)idx[576] * 8 + v21);
        sub_406b31(idx);
        v8 = (g_42d1c0 == 4 ? sub_406f01(idx) : sub_407037(idx));
        v8 = v8;
        if (g_42d1ec || g_42d280)
        {
            v11 = v8;
            for (m = 0; v11 && !m; *(v15) = v11)
            {
                v14 = v11;
                v15 = sub_412437(v11);
                v11 = *(v15);
                *(v15) = NULL;
                if (sub_412af4(v14, &v3, &v4, &v5, &v6))
                    m = 1;
            }
        }
        else
        {
            m = v8;
        }
        if (g_42d2a0)
        {
            sub_407167(m, idx);
        }
        else if (m || g_42d1c8 != 1)
        {
            sub_4118ac((!g_42d200 ? (long long)idx[8] : g_42d200), (!g_42d208 ? (long long)idx[312] : g_42d208));
            switch (g_42d1c0)
            {
            case 1:
                sub_40fcfd(v8);
                break;
            case 2:
                sub_407f2c(v8, 0);
                break;
            case 3:
                sub_407f2c(v8, 1);
                break;
            case 4:
                sub_40cb85(v8);
                break;
            case 5:
                sub_40cd6a(v8);
                break;
            case 6:
                sub_40ceba(v8);
                break;
            case 7:
                sub_40d076(v8);
                break;
            case 8:
                sub_40ff70(v8);
                break;
            default:
                abort(); /* do not return */
            }
            sub_411e1b();
        }
        free((long long)idx[0x100]);
        free(v13);
        for (iter2 = 0; iter2 <= 1; iter2 += 1)
        {
            free(*((long long *)(304 * iter2 + (char *)idx + 248)));
            free(*((long long *)(304 * iter2 + (char *)idx + 192)) * 8 + *((long long *)(304 * iter2 + (char *)idx + 184)));
        }
        for (ptr = v8; ptr; ptr = v16)
        {
            v16 = *(ptr);
            free(ptr);
        }
        if (g_42d1c0 == 4 || g_42d1c0 == 5)
        {
            for (iter2 = 0; iter2 <= 1; iter2 += 1)
            {
                if (*(304 * iter2 + (char *)idx + 288))
                {
                    gettext("No newline at end of file");
                    error(0, 0, "%s: %s\n");
                    m = 2;
                }
            }
        }
    }
    if ((long long)idx[160] != (long long)idx[464])
        free((long long)idx[160]);
    free((long long)idx[464]);
    return m;
}



// Function: print_context_label @ 0x7d13
typedef struct struct_0 {
    char padding_0[104];
    unsigned long long field_68;
} struct_0;

typedef struct FILE {
} FILE;

extern long long g_42d350;
extern FILE *g_42d5c0;

void print_context_label(char *a0, struct_0 *a1, char *a2, char *a3)
{
    unsigned int v0;  // [bp-0x5c]
    unsigned long tm;  // [bp-0x58]
    unsigned long v2;  // [bp-0x50]
    char v3[56];  // [bp-0x48]

    sub_4128c2(0);
    if (a3)
    {
        fprintf(g_42d5c0, "%s %s", a0, a3);
        sub_4128c2(3);
        putc_unlocked(10, g_42d5c0);
        return;
    }
    tm = localtime(&a1->field_68);
    v0 = sub_41bb4f(&a1->padding_0[16]);
    if (!tm || !sub_41710b(v3, 43, g_42d350, tm, 0, v0))
    {
        v2 = a1->field_68;
        sprintf(v3, "%ld.%.9d", v2, v0);
    }
    fprintf(g_42d5c0, "%s %s\t%s", a0, a2, v3);
    sub_4128c2(3);
    putc_unlocked(10, g_42d5c0);
    return;
}



// Function: print_context_header @ 0x7e5c
typedef struct struct_0 {
    char padding_0[104];
    unsigned long long field_68;
} struct_0;

extern char *g_42d200;
extern char *g_42d208;

void print_context_header(struct_0 *a0, char **a1, char a2)
{
    if (!a2)
    {
        sub_407d13("***", a0, *(a1), g_42d200);
        sub_407d13("---", &a0[2].padding_0[80], a1[1], g_42d208);
        return;
    }
    sub_407d13("---", a0, *(a1), g_42d200);
    sub_407d13("+++", &a0[2].padding_0[80], a1[1], g_42d208);
    return;
}



// Function: print_context_script @ 0x7f2c
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[32];
    char field_28;
} struct_0;

extern unsigned long long g_42d1b0;
extern unsigned long long g_42d1b8;
extern char g_42d1ec;
extern unsigned long long g_42d280;
extern unsigned long long g_42d448;

unsigned long long print_context_script(struct_0 *a0, char a1)
{
    struct_0 *i;  // [bp-0x10]

    if (g_42d1ec || g_42d280)
    {
        sub_408c9d(a0);
    }
    else
    {
        for (i = a0; i; i = i->field_0)
        {
            i->field_28 = 0;
        }
    }
    g_42d1b0 = -(g_42d448);
    g_42d1b8 = 0x7fffffffffffffff;
    return (!a1 ? sub_41245b(a0, sub_408bbf, sub_40818f, sub_408bbf) : sub_41245b(a0, sub_408bbf, sub_408710, sub_408bbf));
}



// Function: print_context_number_range @ 0x7ff1
typedef struct FILE {
} FILE;

extern FILE *g_42d5c0;

void print_context_number_range(long long a0, long long a1, long long a2)
{
    char v0;  // [bp-0x20]
    char v1;  // [bp-0x18]

    sub_4129d0(a0, a1, a2, &v0, &v1);
    if (v1 > v0)
    {
        fprintf(g_42d5c0, "%ld,%ld", v0, v1);
        return;
    }
    fprintf(g_42d5c0, "%ld", v1);
    return;
}



// Function: print_context_function @ 0x80a2
typedef struct FILE {
} FILE;

long long print_context_function(FILE *a0, char *a1)
{
    int i;  // [bp-0x10]
    int iter;  // [bp-0xc]

    putc_unlocked(32, a0);
    for (i = 0; (char)sub_4133c9(a1[i]) && a1[i] != 10; i += 1);
    for (iter = i; iter <= i + 39 && a1[iter] != 10; iter += 1);
    for (; i < iter && (char)sub_4133c9(*(&a1[iter] - 1)); iter -= 1);
    return fwrite_unlocked(&a1[i], 1, iter - i, a0);
}



// Function: pr_context_hunk @ 0x818f
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    long long field_10;
    char padding_18[8];
    long long field_20;
} struct_0;

extern unsigned long long g_42d1d0;
extern unsigned long long g_42d240;
extern long long g_42d360;
extern long long g_42d418;
extern unsigned long long g_42d430;
extern long long g_42d448;
extern long long g_42d490;
extern unsigned long long g_42d548;
extern unsigned long long g_42d560;
extern unsigned long long g_42d5c0;

void pr_context_hunk(struct_0 *a0)
{
    long long v12;  // rdx
    long long v13;  // rdx
    long long v14;  // rcx
    long long v15;  // rcx
    unsigned int v0;  // [bp-0x64]
    long long v1;  // [bp-0x60], Other Possible Types: unsigned long
    long long v2;  // [bp-0x58]
    long long v3;  // [bp-0x50], Other Possible Types: unsigned long
    long long v4;  // [bp-0x48]
    long long iter;  // [bp-0x40]
    long long v6;  // [bp-0x38]
    char *v7;  // [bp-0x30]
    struct_0 *j;  // [bp-0x28]
    struct_0 *k;  // [bp-0x20]
    unsigned long v10;  // [bp-0x18]

    v0 = sub_412af4(a0, &v1, &v2, &v3, &v4);
    if (!v0)
        return;
    iter = -(g_42d448);
    v12 = v1 - g_42d1d0;
    if (iter > v12)
        v12 = iter;
    v1 = v12;
    v13 = v3 - g_42d1d0;
    if (iter > v13)
        v13 = iter;
    v3 = v13;
    v2 = (g_42d430 - g_42d1d0 <= v2 ? g_42d430 - 1 : g_42d1d0 + v2);
    v4 = (g_42d560 - g_42d1d0 <= v4 ? g_42d560 - 1 : g_42d1d0 + v4);
    v7 = NULL;
    if (g_42d240)
        v7 = sub_408d30(g_42d418, v1, v1);
    sub_411b50();
    v10 = g_42d5c0;
    fputs_unlocked("***************", v10);
    if (v7)
        sub_4080a2(v10, v7);
    putc_unlocked(10, v10);
    sub_4128c2(4);
    fputs_unlocked("*** ", v10);
    sub_407ff1(&g_42d360, v1, v2);
    fputs_unlocked(" ****", v10);
    sub_4128c2(3);
    putc_unlocked(10, v10);
    if ((char)v0 & 1)
    {
        j = a0;
        for (iter = v1; iter <= v2; iter += 1)
        {
            sub_4128c2(2);
            for (; j && iter >= j->field_10 + j->padding_18; j = j->field_0);
            v6 = " ";
            if (j && iter >= j->padding_18)
                v6 = (j->field_8 <= 0 ? "-" : "!");
            v14 = g_42d418 + iter * 8;
            sub_4124fe(v6, v14, 1, v14);
            sub_4128c2(3);
            if (*((char *)(*((long long *)((iter + 1) * 8 + g_42d418)) - 1)) == 10)
                putc_unlocked(10, v10);
        }
    }
    sub_4128c2(4);
    fputs_unlocked("--- ", v10);
    sub_407ff1(&g_42d490, v3, v4);
    fputs_unlocked(" ----", v10);
    sub_4128c2(3);
    putc_unlocked(10, v10);
    if (!((char)v0 & 2))
        return;
    k = a0;
    for (iter = v3; iter <= v4; iter += 1)
    {
        sub_4128c2(1);
        for (; k && iter >= k->field_8 + k->field_20; k = k->field_0);
        v6 = " ";
        if (k && iter >= k->field_20)
            v6 = (k->field_10 <= 0 ? "+" : "!");
        v15 = g_42d548 + iter * 8;
        sub_4124fe(v6, v15, 1, v15);
        sub_4128c2(3);
        if (*((char *)(*((long long *)((iter + 1) * 8 + g_42d548)) - 1)) == 10)
            putc_unlocked(10, v10);
    }
    return;
}



// Function: print_unidiff_number_range @ 0x863e
typedef struct FILE {
} FILE;

extern FILE *g_42d5c0;

void print_unidiff_number_range(long long a0, long long a1, long long a2)
{
    char v0;  // [bp-0x20], Other Possible Types: long long
    char v1;  // [bp-0x18], Other Possible Types: long long

    sub_4129d0(a0, a1, a2, &v0, &v1);
    if (v1 > v0)
    {
        fprintf(g_42d5c0, "%ld,%ld", v0, v1 - v0 + 1);
        return;
    }
    fprintf(g_42d5c0, (v1 < v0 ? "%ld,0" : "%ld"));
    return;
}



// Function: pr_unidiff_hunk @ 0x8710
typedef struct struct_1 {
    struct struct_2 *field_0;
    void* field_8;
} struct_1;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern char g_4230f4;
extern unsigned long long g_42d1d0;
extern unsigned long long g_42d240;
extern char g_42d2b0;
extern char g_42d2b1;
extern long long g_42d360;
extern long long g_42d418;
extern unsigned long long g_42d430;
extern long long g_42d448;
extern long long g_42d490;
extern unsigned long long g_42d548;
extern unsigned long long g_42d560;
extern unsigned long long g_42d5c0;

void pr_unidiff_hunk(struct_0 *a0)
{
    long long v14;  // rdx
    long long v15;  // rdx
    long long v16;  // rax
    long long v17;  // rax
    long long v18;  // rax
    long long v0;  // [bp-0x78], Other Possible Types: unsigned long
    long long v1;  // [bp-0x70]
    long long v2;  // [bp-0x68], Other Possible Types: unsigned long
    long long v3;  // [bp-0x60]
    long long v4;  // [bp-0x58]
    long long v5;  // [bp-0x50]
    unsigned long iter;  // [bp-0x48], Other Possible Types: unsigned long long
    struct_0 *idx;  // [bp-0x40]
    char *v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    struct_1 *v10;  // [bp-0x28]
    struct_1 *v11;  // [bp-0x20]
    struct_2 **v12;  // [bp-0x18]

    if (!sub_412af4(a0, &v0, &v1, &v2, &v3))
        return;
    v4 = -(g_42d448);
    v14 = v0 - g_42d1d0;
    if (v4 > v14)
        v14 = v4;
    v0 = v14;
    v15 = v2 - g_42d1d0;
    if (v4 > v15)
        v15 = v4;
    v2 = v15;
    v1 = (g_42d430 - g_42d1d0 <= v1 ? g_42d430 - 1 : g_42d1d0 + v1);
    v3 = (g_42d560 - g_42d1d0 <= v3 ? g_42d560 - 1 : g_42d1d0 + v3);
    v8 = NULL;
    if (g_42d240)
        v8 = sub_408d30(g_42d418, v0, v0);
    sub_411b50();
    v9 = g_42d5c0;
    sub_4128c2(4);
    fputs_unlocked("@@ -", v9);
    sub_40863e(&g_42d360, v0, v1);
    fputs_unlocked(" +", v9);
    sub_40863e(&g_42d490, v2, v3);
    fputs_unlocked(&g_4230f4, v9);
    sub_4128c2(3);
    if (v8)
        sub_4080a2(v9, v8);
    putc_unlocked(10, v9);
    idx = a0;
    v4 = v0;
    v5 = v2;
    while (v4 <= v1 || v5 <= v3)
    {
        if (!idx || v4 < idx[1].field_0)
        {
            v18 = v4;
            v4 = v18 + 1;
            v12 = v18 * 8 + g_42d418;
            if (g_42d2b1 != 1 || *(v12)->field_0 != 10)
                putc_unlocked((!g_42d2b0 ? 32 : 9), v9);
            sub_4124cf(0, v12);
            v5 += 1;
        }
        else
        {
            iter = idx->field_10;
            while (1)
            {
                iter -= 1;
                if (!iter)
                    break;
                v16 = v4;
                v4 = v16 + 1;
                v11 = v16 * 8 + g_42d418;
                sub_4128c2(2);
                putc_unlocked(45, v9);
                if (g_42d2b0 && (g_42d2b1 != 1 || v11->field_0->field_0 != 10))
                    putc_unlocked(9, v9);
                sub_4124fe(0, v11, 1);
                sub_4128c2(3);
                iter = iter;
                if (*((char *)v11->field_8 - 1) == 10)
                    putc_unlocked(10, v9);
            }
            iter = idx->field_8;
            while (1)
            {
                iter -= 1;
                if (!iter)
                    break;
                v17 = v5;
                v5 = v17 + 1;
                v10 = v17 * 8 + g_42d548;
                sub_4128c2(1);
                putc_unlocked(43, v9);
                if (g_42d2b0 && (g_42d2b1 != 1 || v10->field_0->field_0 != 10))
                    putc_unlocked(9, v9);
                sub_4124fe(0, v10, 1);
                sub_4128c2(3);
                iter = iter;
                if (*((char *)v10->field_8 - 1) == 10)
                    putc_unlocked(10, v9);
            }
            idx = idx->field_0;
        }
    }
    return;
}



// Function: find_hunk @ 0x8bbf
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char field_28;
} struct_0;

extern long long g_42d1d0;

struct_0 * find_hunk(struct_0 *a0)
{
    long long v8;  // rax
    struct_0 *idx;  // [bp-0x40]
    long long v1;  // [bp-0x38]
    long long v2;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    struct_0 *v5;  // [bp-0x18]
    long long v6;  // [bp-0x10]

    idx = a0;
    v1 = g_42d1d0;
    v2 = g_42d1d0 * 2 + 1;
    do
    {
        v3 = idx->field_10 + idx->field_18;
        v4 = idx->field_8 + idx->field_20;
        v5 = idx;
        idx = idx->field_0;
        if (idx && idx->field_28)
            v8 = v1;
        else
            v8 = v2;
        v6 = v8;
        if (idx && idx->field_18 - v3 != idx->field_20 - v4)
            abort(); /* do not return */
    } while (idx && v6 > idx->field_18 - v3);
    return v5;
}



// Function: mark_ignorable @ 0x8c9d
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[32];
    char field_28;
} struct_0;

void mark_ignorable(struct_0 *a0)
{
    struct_0 *i;  // [bp-0x40]
    char v1;  // [bp-0x38]
    char v2;  // [bp-0x30]
    char v3;  // [bp-0x28]
    char v4;  // [bp-0x20]
    struct_0 *v5;  // [bp-0x18]

    for (i = a0; i; i = v5)
    {
        v5 = i->field_0;
        i->field_0 = NULL;
        i->field_28 = !sub_412af4(i, &v1, &v2, &v3, &v4);
        i->field_0 = v5;
    }
    return;
}



// Function: find_function @ 0x8d30
extern unsigned long long g_42d1b0;
extern unsigned long long g_42d1b8;

unsigned long long find_function(unsigned long a0, unsigned long a1)
{
    unsigned int v6;  // eax
    unsigned int v0;  // [bp-0x2c]
    unsigned long idx;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]
    unsigned long long v4;  // [bp-0x10]

    idx = a1;
    v2 = g_42d1b0;
    g_42d1b0 = idx;
    while (1)
    {
        idx -= 1;
        if (idx < v2)
            return (g_42d1b8 == 0x7fffffffffffffff ? 0 : *((long long *)(a0 + g_42d1b8 * 8)));
        v3 = *((long long *)(a0 + idx * 8));
        v4 = *((long long *)(a0 + (idx + 1) * 8)) - v3 - 1;
        v6 = v4;
        if (v4 > 0x7fffffff)
            v6 = 0x7fffffff;
        v0 = v6;
        if (re_search() >= 0)
        {
            g_42d1b8 = idx;
            return v3;
        }
    }
}



// Function: option_list @ 0x8e44
char * option_list(unsigned long a0, int a1)
{
    int iter;  // [bp-0x24]
    long long v1;  // [bp-0x20]
    char *v2;  // [bp-0x18]
    char *v3;  // [bp-0x18]
    char *v4;  // [bp-0x10]

    v1 = 1;
    for (iter = 0; iter < a1; iter += 1)
    {
        v1 += sub_41a4f6(*((long long *)(a0 + iter * 8))) + 1;
    }
    v4 = sub_41c945(v1);
    v2 = v4;
    for (iter = 0; iter < a1; iter += 1)
    {
        v3 = v2 + 1;
        *(v2) = 32;
        v2 = sub_41a550(v3, *((long long *)(a0 + iter * 8)), *((long long *)(a0 + iter * 8)));
    }
    *(v2) = 0;
    return v4;
}



// Function: exclude_options @ 0x8f0e
extern char g_42d1ef;

unsigned int exclude_options(void)
{
    return (!g_42d1ef ? 0x10000000 : 268435472);
}



// Function: main @ 0x8f2f
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_42353e;
extern option g_42bf40;
extern long long g_42d0c0;
extern unsigned int g_42d0c8;
extern long long stdout;
extern unsigned int optind;
extern unsigned long long optarg;
extern unsigned int g_42d1c0;
extern unsigned int g_42d1c4;
extern char g_42d1c8;
extern unsigned long long g_42d1d0;
extern char g_42d1d8;
extern unsigned long long g_42d1e0;
extern unsigned int g_42d1e8;
extern char g_42d1ec;
extern char g_42d1ed;
extern char g_42d1ee;
extern char g_42d1ef;
extern char g_42d1f0;
extern unsigned long long g_42d200;
extern unsigned long long g_42d208;
extern char g_42d220;
extern char g_42d260;
extern char g_42d2a0;
extern char g_42d2a1;
extern unsigned long long g_42d2a8;
extern char g_42d2b0;
extern char g_42d2b1;
extern char g_42d2b2;
extern long long g_42d2b8;
extern char g_42d2c0;
extern struct_0 *g_42d2e0;
extern long long g_42d2e8;
extern long long g_42d2f0;
extern unsigned long long g_42d2f8;
extern struct struct_0 *g_42d300[4];
extern char g_42d318;
extern char g_42d319;
extern char g_42d31a;
extern unsigned long long g_42d320;
extern unsigned long long g_42d328;
extern unsigned long long g_42d330;
extern char g_42d338;
extern long long g_42d340;
extern char g_42d348;
extern unsigned long long g_42d350;
extern char g_42d5c8;
extern char g_42d5c9;
extern long long g_42d5e0;
extern unsigned long long g_42d600;
extern unsigned long long g_42d620;
extern unsigned long long g_42d640;
extern char g_42d648;
extern char g_42d649;
extern char g_42d64a;

unsigned int main(int j, long long *a1)
{
    long long v22;  // rdi
    long long v23;  // rsi
    long long v32;  // rbx
    long long v33;  // rax
    long long v34;  // rax
    unsigned long v35;  // rdx
    unsigned long v36;  // rax
    char v37;  // al
    char v38;  // al
    unsigned int v39;  // eax
    unsigned long long v40;  // rcx
    unsigned long long v41;  // rdx
    long long v24;  // rdx
    unsigned long long v42;  // rcx
    long long v25;  // rcx
    long long v26;  // r8
    long long v27;  // r9
    unsigned int v28;  // eax
    unsigned int v29;  // eax
    long long v30;  // r13
    long long v31;  // r12
    char v0;  // [bp-0xb2]
    char v1;  // [bp-0xb1]
    int v2;  // [bp-0xb0]
    int iter1;  // [bp-0xac]
    int iter;  // [bp-0xa8]
    int v5;  // [bp-0xa4]
    int v6;  // [bp-0xa0]
    int v7;  // [bp-0x9c]
    unsigned long v8;  // [bp-0x98]
    unsigned long v9;  // [bp-0x90]
    char *v10;  // [bp-0x88]
    long long v11;  // [bp-0x80]
    unsigned long v12;  // [bp-0x78]
    long long v13;  // [bp-0x70], Other Possible Types: unsigned long
    unsigned long node;  // [bp-0x68]
    char *v15;  // [bp-0x60]
    unsigned long v16;  // [bp-0x58]
    unsigned long v17;  // [bp-0x50]
    unsigned long v18;  // [bp-0x48]
    unsigned long long v19;  // [bp-0x40]
    unsigned long v20;  // [bp-0x38]

    v2 = 0;
    v5 = -0x1;
    v11 = -0x1;
    v0 = 0;
    v12 = 0;
    v1 = 0;
    v8 = 0;
    v9 = 0;
    g_42d0c8 = 2;
    sub_419a53(*(a1));
    setlocale(6, &g_42353e);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_413721(0);
    g_42d600 = &g_42d220;
    g_42d640 = &g_42d260;
    re_set_syntax(330310);
    g_42d340 = sub_4139c7(v22, v23, v24, v25, v26, v27);
    g_42d5c8 = 0;
    sub_41cfb9(v22, v23, v24, v25, v26, v27);
    while (1)
    {
        iter1 = getopt_long(j, a1, "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ", &g_42bf40.name, NULL);
        if (iter1 == -0x1)
            break;
        switch (iter1)
        {
        case 0: case 104: case 128: case 133:
            break;
        case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
            v11 = (v5 - 48 <= 9 ? (v11 - (iter1 <= 0x33) <= 461168601842738789 ? iter1 - 48 + v11 * 10 : 0x3fffffffffffffff) : iter1 - 48);
            break;
        case 66:
            g_42d1ec = 1;
            break;
        case 67: case 85:
            if (optarg)
            {
                v13 = strtoimax(optarg, &v10, 10, &v10);
                if (*(v10) || v13 - 0 >> 63 & 1)
                {
                    sub_40a338("invalid context length '%s'", optarg); /* do not return */
                }
                else if (v13 > 0x3fffffffffffffff)
                {
                    v13 = 0x3fffffffffffffff;
                }
            }
            else
            {
                v13 = 3;
            }
            sub_40a63e((iter1 == 0x55 ? 3 : 2));
            if (v13 > g_42d1d0)
                g_42d1d0 = v13;
            v0 = 1;
            break;
        case 68:
            sub_40a63e(7);
            node = sub_41c945(strlen(optarg) * 7 + 97);
            sprintf(node, "%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n", 0, optarg, optarg, 0, optarg, optarg, 0, optarg, optarg, optarg);
            for (iter = 0; iter <= 3; iter += 1)
            {
                sub_40a5af(&(&g_42d2e0)[iter], node, "-D", &(&g_42d2e0)[iter]);
                node += strlen(node) + 1;
            }
        case 69:
            if (g_42d1e8 <= 3)
                g_42d1e8 = g_42d1e8 | 1;
            break;
        case 70:
            sub_40a107(&g_42d5e0, optarg);
            break;
        case 72:
            g_42d338 = 1;
            break;
        case 73:
            sub_40a107(&g_42d620, optarg);
            break;
        case 76:
            if (!g_42d200)
            {
                g_42d200 = optarg;
                break;
            }
            else if (!g_42d208)
            {
                g_42d208 = optarg;
                break;
            }
            else
            {
                sub_410966("too many file label options"); /* do not return */
            }
        case 78:
            g_42d648 = 1;
            break;
        case 80:
            g_42d649 = 1;
            break;
        case 83:
            sub_40a5af(&g_42d2b8, optarg, "-S");
            break;
        case 84:
            g_42d2b0 = 1;
            break;
        case 87:
            v13 = strtoimax(optarg, &v10, 10, &v10);
            if (v13 <= 0 || *(v10))
                sub_40a338("invalid width '%s'", optarg); /* do not return */
            if (v12 != v13)
            {
                if (v12)
                    sub_410966("conflicting width options"); /* do not return */
                v12 = v13;
            }
            break;
        case 88:
            v29 = sub_408f0e();
            if (sub_4148b2(sub_4142e9, g_42d340, optarg, v29, 10))
                sub_410925(optarg); /* do not return */
            break;
        case 90:
            if (g_42d1e8 <= 3)
                g_42d1e8 = g_42d1e8 | 2;
            break;
        case 97:
            g_42d1d8 = 1;
            break;
        case 98:
            if (g_42d1e8 <= 3)
                g_42d1e8 = 4;
            break;
        case 99:
            sub_40a63e(2);
            if (g_42d1d0 <= 2)
                g_42d1d0 = 3;
            break;
        case 100:
            g_42d348 = 1;
            break;
        case 101:
            sub_40a63e(4);
            break;
        case 102:
            sub_40a63e(5);
            break;
        case 105:
            g_42d1ee = 1;
            break;
        case 108:
            if (0)
                sub_40a338("pagination not supported on this host", 0); /* do not return */
            g_42d2c0 = 1;
            signal(0x11, NULL);
            break;
        case 110:
            sub_40a63e(6);
            break;
        case 112:
            v1 = 1;
            sub_40a107(&g_42d5e0, "^[[:alpha:]$_]");
            break;
        case 113:
            g_42d2a0 = 1;
            break;
        case 114:
            g_42d5c9 = 1;
            break;
        case 115:
            g_42d64a = 1;
            break;
        case 116:
            g_42d2a1 = 1;
            break;
        case 117:
            sub_40a63e(3);
            if (g_42d1d0 <= 2)
                g_42d1d0 = 3;
            break;
        case 118:
            v30 = sub_41a014("Len Tower");
            v31 = sub_41a014("Richard Stallman");
            v32 = sub_41a014("David Hayes");
            v33 = sub_41a014("Mike Haertel");
            v34 = sub_41a014("Paul Eggert");
            sub_41c71c(stdout, "diff", "GNU diffutils", g_42d0c0, v34, v33, v32, v31, v30, 0);
            sub_40a3ae(v22, v23, v24, v25, v26, v27);
            return 0;
        case 119:
            g_42d1e8 = 5;
            break;
        case 120:
            v28 = sub_408f0e();
            sub_4142e9(g_42d340, optarg, v28, optarg);
            break;
        case 121:
            sub_40a63e(8);
            break;
        case 129:
            sub_40a5af(&v8, optarg, "--from-file", optarg);
            break;
        case 130:
            sub_40a405();
            sub_40a3ae(v22, v23, v24, v25, v26, v27);
            return 0;
        case 131:
            v13 = strtoimax(optarg, &v10, 10, &v10);
            if (*(v10) || v13 - 0 >> 63 & 1)
                sub_40a338("invalid horizon length '%s'", optarg); /* do not return */
            if (v13 <= g_42d1e0)
                v13 = g_42d1e0;
            g_42d1e0 = v13;
            break;
        case 132:
            g_42d1ef = 1;
            break;
        case 134:
            g_42d319 = 1;
            break;
        case 135:
            sub_40a63e(7);
            for (iter = 0; iter <= 2; iter += 1)
            {
                sub_40a5af(&g_42d300[iter], optarg, "--line-format", &g_42d300[iter]);
            }
        case 136:
            g_42d1f0 = 1;
            break;
        case 137:
            g_42d1ef = 0;
            break;
        case 138:
            sub_40a63e(1);
            break;
        case 139:
            sub_40a63e(8);
            g_42d318 = 1;
            break;
        case 140:
            g_42d2b2 = 1;
            break;
        case 141:
            g_42d2b1 = 1;
            break;
        case 142:
            g_42d31a = 1;
            break;
        case 143:
            v13 = strtoimax(optarg, &v10, 10, &v10);
            if (v13 <= 0 || v13 > 0xfffffffffffffffc || *(v10))
                sub_40a338("invalid tabsize '%s'", optarg); /* do not return */
            if (g_42d2a8 != v13)
            {
                if (g_42d2a8)
                    sub_410966("conflicting tabsize options"); /* do not return */
                g_42d2a8 = v13;
            }
            break;
        case 144:
            sub_40a5af(&v9, optarg, "--to-file", optarg);
            break;
        case 145: case 146: case 147:
            sub_40a63e(7);
            iter1 -= 145;
            sub_40a5af(&g_42d300[iter1], optarg, (char *)(24 * iter1 + 4338112), &g_42d300[iter1]);
            break;
        case 148: case 149: case 150: case 151:
            sub_40a63e(7);
            iter1 -= 148;
            sub_40a5af(&(&g_42d2e0)[iter1], optarg, (char *)(25 * iter1 + 4337984), &(&g_42d2e0)[iter1]);
            break;
        case 152:
            sub_40a682(optarg);
            break;
        case 153:
            sub_41140f(optarg);
            break;
        case 154:
            g_42d5c8 = 1;
            break;
        default:
            sub_40a338(0, 0); /* do not return */
        }
        v5 = iter1;
    }
    if (g_42d1c4 == 1)
    {
        v15 = getenv("TERM");
        if (v15 && !strcmp(v15, "dumb"))
            g_42d1c4 = 0;
    }
    if (!g_42d1c0)
    {
        if (v1)
        {
            sub_40a63e(2);
            if (v11 - 0 >> 63 & 1)
                g_42d1d0 = 3;
        }
        else
        {
            sub_40a63e(1);
        }
    }
    if (g_42d1c0 != 2 || (char)sub_414cdf(2))
        g_42d350 = "%Y-%m-%d %H:%M:%S.%N %z";
    else
        g_42d350 = "%a %b %e %T %Y";
    if (!(v11 - 0 >> 63 & 1) && (g_42d1c0 == 2 || g_42d1c0 == 3) && (v11 > g_42d1d0 || v11 < g_42d1d0 && v0 != 1))
        g_42d1d0 = v11;
    if (!g_42d2a8)
        g_42d2a8 = 8;
    if (!v12)
        v12 = 130;
    v16 = (!g_42d2a1 ? g_42d2a8 : 1);
    v17 = v12;
    v18 = v16 + 3;
    v19 = ((unsigned int)v17 & (unsigned int)v18 & 1) + (v17 >> 1) + (v18 >> 1);
    v20 = v19 - v19 % v16;
    if (v20 > 3 && v17 > v20)
    {
        v35 = v20 - 3;
        v36 = v17 - v20;
        if (v35 <= v36)
            v36 = v35;
    }
    else
    {
        v36 = 0;
    }
    g_42d320 = v36;
    g_42d328 = (!g_42d320 ? v17 : v20);
    if (g_42d1e0 < g_42d1d0)
        g_42d1e0 = g_42d1d0;
    sub_40a296(&g_42d5e0);
    sub_40a296(&g_42d620);
    if (g_42d1c0 == 7)
    {
        for (iter = 0; iter <= 2; iter += 1)
        {
            if (!g_42d300[iter])
                g_42d300[iter] = "%l\n";
        }
        if (!g_42d2e8)
            g_42d2e8 = (!g_42d2f8 ? "%<" : g_42d2f8);
        if (!g_42d2f0)
            g_42d2f0 = (!g_42d2f8 ? "%>" : g_42d2f8);
        if (!g_42d2e0)
            g_42d2e0 = "%=";
        if (!g_42d2f8)
            g_42d2f8 = sub_412fa9(g_42d2e8, g_42d2f0, &g_42353e, g_42d2f0);
    }
    if (g_42d1c0 == 7)
    {
        if (!g_42d2e0->field_0 || !strcmp(g_42d2e0, "%=") && !(char)g_42d300[0])
            v37 = 1;
        else
            v37 = 0;
        v38 = v37 & 1;
    }
    else
    {
        v38 = g_42d31a || g_42d1c0 != 8;
    }
    g_42d1c8 = v38;
    if (g_42d620 || g_42d1e8)
        v39 = 1;
    else
        v39 = 0;
    g_42d1ed = ~(v39 | g_42d2b2 | g_42d1ec | g_42d1ee) & g_42d2a0 & 1;
    if (v8)
    {
        if (v9)
            sub_410966("--from-file and --to-file both specified"); /* do not return */
        for (; j > optind; optind = optind + 1)
        {
            v7 = sub_40a746(0, v8, a1[optind]);
            if (v2 < v7)
                v2 = v7;
        }
    }
    else if (v9)
    {
        for (g_42d330 = sub_408e44(a1 + 1, optind - 1); j > optind; optind = optind + 1)
        {
            v40 = optind * 8;
            v6 = sub_40a746(0, *((long long *)((char *)a1 + v40)), v9, v40);
            if (v2 < v6)
                v2 = v6;
        }
    }
    else
    {
        if (j - optind != 2)
        {
            if (j - optind <= 1)
                sub_40a338("missing operand after '%s'", *((long long *)((char *)&a1[j] - 8)), j * 8 - 8); /* do not return */
            v41 = (optind + 2) * 8;
            sub_40a338("extra operand '%s'", *((long long *)((char *)a1 + v41)), v41); /* do not return */
        }
        v42 = optind * 8;
        v2 = sub_40a746(0, *((long long *)((char *)a1 + v42)), a1[1 + optind], v42);
    }
    sub_410bca();
    sub_40a3ae(v22, v23, v24, v25, v26, v27);
    exit(v2); /* do not return */
}



// Function: add_regexp @ 0xa107
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    long long field_20;
} struct_0;

void* add_regexp(struct_0 *idx, unsigned long a1)
{
    char v0;  // [bp-0x39]
    unsigned long long v1;  // [bp-0x38]
    unsigned long v2;  // [bp-0x30]
    unsigned long v3;  // [bp-0x30]
    unsigned long long i;  // [bp-0x28]
    unsigned long len;  // [bp-0x20]
    unsigned long long v6;  // [bp-0x18]
    unsigned long v7;  // [bp-0x10]

    len = strlen(a1);
    v6 = re_compile_pattern(a1, len, idx->field_20, len);
    if (v6)
        error(2, 0, "%s: %s");
    v1 = idx->field_0;
    v2 = idx->field_8;
    idx->field_18 = v1;
    v0 = idx->field_18;
    idx->field_8 = v0 * 2 + v2 + len;
    v7 = idx->field_8;
    i = idx->field_10;
    if (i <= v7)
    {
        if (!i)
            i = 1;
        do
        {
            i *= 2;
        } while (i <= v7);
        idx->field_10 = i;
        v1 = sub_41c991(v1, i, i);
        idx->field_0 = v1;
    }
    if (v0)
    {
        v3 = v2 + 1;
        *((char *)(v2 + v1)) = 92;
        v2 = v3 + 1;
        *((char *)(v3 + v1)) = 124;
    }
    return memcpy(v1 + v2, a1, len + 1);
}



// Function: summarize_regexp_list @ 0xa296
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    long long field_0;
    long long field_8;
    char padding_10[8];
    char field_18;
    char padding_19[7];
    struct struct_0 *field_20;
} struct_1;

long long summarize_regexp_list(struct_1 *a0)
{
    unsigned long long v2;  // rax
    long long v3;  // rax
    long long v0;  // [bp-0x20]

    if (!a0->field_0)
        return a0->field_0;
    v2 = sub_41c945(0x100);
    a0->field_20->field_20 = v2;
    v3 = a0->field_18;
    if (!(char)v3)
        return v3;
    v0 = re_compile_pattern(a0->field_0, a0->field_8, a0->field_20, a0->field_8);
    if (!v0)
        return v0;
    error(2, 0, "%s: %s");
}



// Function: try_help @ 0xa338
void try_help(char *a0, unsigned long long a1)
{
    unsigned long long v3;  // rbx
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x10]

    v1 = v3;
    v0 = a1;
    if (a0)
        error(0, 0, gettext(a0));
    error(2, 0, gettext("Try '%s --help' for more information."));
}



// Function: check_stdout @ 0xa3ae
typedef struct FILE {
} FILE;

extern FILE *stdout;

unsigned long long check_stdout(void)
{
    unsigned long long v1;  // rax

    if (ferror_unlocked(stdout))
        sub_410966("write failed"); /* do not return */
    v1 = fclose(stdout);
    if (!(unsigned int)v1)
        return v1;
    sub_410925(gettext("standard output")); /* do not return */
}



// Function: usage @ 0xa405
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern struct_0 *g_42c6e0;
extern FILE *stdout;

long long usage(void)
{
    char *v7;  // rax
    unsigned int v8;  // eax
    unsigned int v0;  // [bp-0x34]
    struct_0 **i;  // [bp-0x30]
    char *v2;  // [bp-0x28]
    char *ptr;  // [bp-0x20]

    printf(gettext("Usage: %s [OPTION]... FILES\n"));
    printf("%s\n\n", gettext("Compare FILES line by line."));
    v7 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
    fputs_unlocked(v7, stdout);
    for (i = &g_42c6e0; *(i); i += 1)
    {
        if (!*(i)->field_0)
        {
            putchar_unlocked(10);
        }
        else
        {
            v2 = gettext(*(i));
            while (1)
            {
                ptr = strchr(v2, 10);
                if (!ptr)
                    break;
                v0 = ptr + 1 - v2;
                if (v0 > 0xfff)
                    __assert_fail(); /* do not return */
                printf("  %.*s", v0);
                v2 = ptr + 1;
            }
            if (*(v2) != 32 && *(v2) != 45)
                v8 = 1;
            else
                v8 = 0;
            printf((char *)(2 * v8 + 4345550));
        }
    }
    return sub_41c800();
}



// Function: specify_value @ 0xa5af
char ** specify_value(char **a0, char *a1, unsigned long long a2)
{
    unsigned long long v0;  // [bp-0x20]

    v0 = a2;
    if (*(a0) && strcmp(*(a0), a1))
    {
        error(0, 0, gettext("conflicting %s option value '%s'"));
        sub_40a338(NULL, 0); /* do not return */
    }
    *(a0) = a1;
    return a0;
}



// Function: specify_style @ 0xa63e
extern unsigned int g_42d1c0;

long long specify_style(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_42d1c0;
    if (a0 == (unsigned int)v1)
    {
        return v1;
    }
    else if (g_42d1c0)
    {
        sub_40a338("conflicting output style options", 0); /* do not return */
    }
    else
    {
        g_42d1c0 = a0;
        return a0;
    }
}



// Function: specify_colors_style @ 0xa682
extern unsigned int g_42d1c4;

int specify_colors_style(unsigned long a0)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    unsigned int v3;  // eax

    if (a0)
    {
        v1 = strcmp(a0, "auto");
        if (v1)
        {
            v2 = strcmp(a0, "always");
            if (!v2)
            {
                g_42d1c4 = 2;
                return v2;
            }
            v3 = strcmp(a0, "never");
            if (v3)
                sub_40a338("invalid color '%s'", a0); /* do not return */
            g_42d1c4 = 0;
            return v3;
        }
    }
    g_42d1c4 = 1;
    return v1;
}



// Function: set_mtime_to_now @ 0xa723
long long set_mtime_to_now(long long a0)
{
    return sub_414c76(a0 + 88);
}



// Function: compare_files @ 0xa746
typedef struct struct_0 {
    char field_0[8];
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
    unsigned long long field_68;
    unsigned long long field_70;
    unsigned long long field_78;
    unsigned long long field_80;
    unsigned long long field_88;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_42d1c0;
extern char g_42d1c8;
extern char g_42d1ed;
extern char g_42d1f0;
extern char g_42d200;
extern char g_42d5c9;
extern char g_42d648;
extern char g_42d649;
extern char g_42d64a;

unsigned long long compare_files(long long *a0, unsigned long a1, unsigned long a2)
{
    long long *v53;  // rcx
    int i;  // ebx
    unsigned int k;  // ebx
    int l;  // ebx
    long long v65;  // rax
    int v67;  // eax
    char v68;  // al
    int m;  // ebx
    int n;  // ebx
    long long *v71;  // rcx
    struct_0 *idx;  // rax
    long long v56;  // rax
    long long v57;  // rcx
    int j;  // ebx
    char v60;  // al
    char v61;  // al
    char v62;  // al
    unsigned long v0;  // [bp-0x310]
    unsigned long v1;  // [bp-0x308]
    char v2;  // [bp-0x2f5]
    unsigned int v3;  // [bp-0x2f4]
    int v4;  // [bp-0x2f0]
    int v5;  // [bp-0x2ec]
    int v6;  // [bp-0x2e8]
    int v7;  // [bp-0x2e4]
    unsigned long v8;  // [bp-0x2e0]
    unsigned long v9;  // [bp-0x2d8]
    unsigned long v10;  // [bp-0x2d0]
    unsigned long v11;  // [bp-0x2c8]
    unsigned long v12;  // [bp-0x2c0]
    unsigned long v13;  // [bp-0x2b8]
    unsigned long v14;  // [bp-0x2b0]
    unsigned long v15;  // [bp-0x2a8]
    unsigned long long v16;  // [bp-0x2a0]
    unsigned long v17;  // [bp-0x298]
    char *v18;  // [bp-0x290]
    char v19;  // [bp-0x288], Other Possible Types: unsigned int
    unsigned long v20;  // [bp-0x280]
    unsigned long v21;  // [bp-0x278]
    unsigned long v22;  // [bp-0x270]
    unsigned long v23;  // [bp-0x268]
    unsigned long v24;  // [bp-0x260]
    unsigned long v25;  // [bp-0x258]
    unsigned long v26;  // [bp-0x250]
    unsigned long v27;  // [bp-0x248]
    unsigned long v28;  // [bp-0x240]
    unsigned long v29;  // [bp-0x238]
    unsigned long v30;  // [bp-0x230]
    unsigned long v31;  // [bp-0x228]
    unsigned long v32;  // [bp-0x220]
    unsigned long v33;  // [bp-0x218]
    unsigned long v34;  // [bp-0x210]
    unsigned long v35;  // [bp-0x208]
    unsigned long v36;  // [bp-0x200]
    unsigned long v37;  // [bp-0x1f8]
    unsigned long v38;  // [bp-0x1f0]
    unsigned int v39;  // [bp-0x158]
    unsigned long v40;  // [bp-0x150]
    unsigned long v41;  // [bp-0x148]
    unsigned long v42;  // [bp-0x140]
    unsigned long v43;  // [bp-0x138]
    unsigned int v44;  // [bp-0x130]
    unsigned int v45;  // [bp-0x12c]
    unsigned int v46;  // [bp-0x128]
    unsigned long v47;  // [bp-0x120]
    unsigned long v48;  // [bp-0x118]
    unsigned long v49;  // [bp-0xf0]
    unsigned long v50;  // [bp-0xe0]
    long long *v51;  // [bp-0x28]

    v1 = a1;
    v0 = a2;
    v3 = 0;
    if ((!v1 || !v0) && (g_42d649 != 1 || !v0) && g_42d648 != 1)
    {
        if (v1)
            v0 = v1;
        v10 = v0;
        v53 = a0;
        v11 = v53[1 + 38 * (!v1)];
        sub_4109a8("Only in %s: %s\n", v11, v10, v53);
        return 1;
    }
    memset(&v19, 0, 608);
    v51 = a0;
    v19 = (!v1 ? 0xffffffff : 0xfffffffe);
    v39 = (!v0 ? 0xffffffff : 0xfffffffe);
    if (!v1)
        v1 = v0;
    if (!v0)
        v0 = v1;
    if (!a0)
    {
        v8 = 0;
        v9 = 0;
        v20 = v1;
        v40 = v0;
    }
    else
    {
        v8 = sub_414ade(a0[1], v1, 0, v1);
        v20 = v8;
        v9 = sub_414ade(a0[39], v0, 0, v0);
        v40 = v9;
    }
    for (i = 0; i <= 1; i += 1)
    {
        if ((&v19)[76 * i] != 0xffffffff)
        {
            if (i && !strcmp((&v20)[38 * i], v20))
            {
                (&v19)[76 * i] = v19;
                idx = &(&v21)[38 * i];
                *((unsigned long *)&idx->field_0[0]) = v21;
                idx->field_8 = v22;
                idx->field_10 = v23;
                idx->field_18 = v24;
                idx->field_20 = v25;
                idx->field_28 = v26;
                idx->field_30 = v27;
                idx->field_38 = v28;
                idx->field_40 = v29;
                idx->field_48 = v30;
                idx->field_50 = v31;
                idx->field_58 = v32;
                idx->field_60 = v33;
                idx->field_68 = v34;
                idx->field_70 = v35;
                idx->field_78 = v36;
                idx->field_80 = v37;
                idx->field_88 = v38;
            }
            else if (!strcmp((&v20)[38 * i], "-"))
            {
                (&v19)[76 * i] = 0;
                if (!isatty(0))
                    sub_4131e6(0, 0);
                if (fstat(0, &(&v21)[38 * i]))
                {
                    (&v19)[76 * i] = 0xfffffffd - *(__errno_location());
                }
                else
                {
                    if (((short)(&v24)[38 * i] & 0xf000) == 0x8000)
                    {
                        v16 = lseek(0, 0, 1);
                        if (v16 - 0 >> 63 & 1)
                        {
                            (&v19)[76 * i] = 0xfffffffd - *(__errno_location());
                        }
                        else
                        {
                            v56 = (&v27)[38 * i] - v16;
                            v57 = 0;
                            if (v56 >= 0)
                                v57 = v56;
                            (&v27)[38 * i] = v57;
                        }
                    }
                    sub_40a723(&(&v21)[38 * i]);
                }
            }
            else if ((!g_42d1f0 ? stat((&v20)[38 * i], &(&v21)[38 * i]) : lstat((&v20)[38 * i], &(&v21)[38 * i])))
            {
                (&v19)[76 * i] = 0xfffffffd - *(__errno_location());
            }
        }
    }
    for (j = 0; j <= 1; j += 1)
    {
        if (g_42d648 || !j && g_42d649)
        {
            if ((&v19)[76 * j] == 0xfffffffe)
            {
                if (((short)(&v24)[38 * j] & 0xf000) == 0x8000 && !((short)(&v24)[38 * j] & 0x1ff) && !(&v27)[38 * j])
                    v60 = 1;
                else
                    v60 = 0;
                v61 = v60 & 1;
            }
            else
            {
                if (((&v19)[76 * j] == 0xfffffffb || (&v19)[76 * j] == 0xfffffff4) && !a0 && ((&v19)[76 + -76 * j] == 0xfffffffe || !(&v19)[76 + -76 * j]))
                    v62 = 1;
                else
                    v62 = 0;
                v61 = v62 & 1;
            }
            if (v61)
                (&v19)[76 * j] = 0xffffffff;
        }
    }
    for (k = 0; k <= 1; k += 1)
    {
        if ((&v19)[76 * k] == 0xffffffff)
        {
            memset(&(&v21)[38 * k], 0, 144);
            *((int *)&(&v24)[38 * k]) = (&v24)[38 * (unsigned int)(1 - k)];
        }
    }
    for (l = 0; l <= 1; l += 1)
    {
        v7 = 0xfffffffd - (&v19)[76 * l];
        if (v7 >= 0)
        {
            *(__errno_location()) = v7;
            sub_4108ec((&v20)[38 * l]);
            v3 = 2;
        }
    }
    if (!v3 && !a0 && (v44 & 0xf000) == 0x4000 ^ ((unsigned int)v24 & 0xf000) == 0x4000)
    {
        v4 = ((unsigned int)v24 & 0xf000) == 0x4000;
        v5 = 1 - v4;
        v12 = (&v20)[38 * v4];
        v13 = (&v20)[38 * v5];
        v65 = sub_413111(v12);
        v8 = sub_40c9bd(v13, v65, v65);
        (&v20)[38 * v5] = v8;
        v14 = (&v20)[38 * v5];
        if (!strcmp(v12, "-"))
        {
            sub_410966("cannot compare '-' to a directory"); /* do not return */
        }
        else if ((!g_42d1f0 ? stat(v14, &(&v21)[38 * v5]) : lstat(v14, &(&v21)[38 * v5])))
        {
            sub_4108ec(v14);
            v3 = 2;
        }
    }
    if (!v3 && (v19 != 0xffffffff || v39 != 0xffffffff))
    {
        if (v19 != 0xffffffff && v39 != 0xffffffff)
        {
            if (v22 == v42 && v21 == v41 || (((unsigned short)v24 & 0xf000) == 0x6000 && ((unsigned short)v44 & 0xf000) == 0x6000 || ((unsigned short)v24 & 0xf000) == 0x2000 && ((unsigned short)v44 & 0xf000) == 0x2000) && v26 == v47)
                v67 = 1;
            else
                v67 = 0;
            if (v67 <= 0 || (unsigned int)v24 != v44 || v23 != v43 || *((unsigned int *)((void*)&v24 + 4)) != v45 || (unsigned int)v25 != v46 || v27 != v48 || v32 != v49 || v34 != v50)
                goto LABEL_40b57a;
            v68 = 1;
        }
        else
        {
LABEL_40b57a:
            v68 = 0;
        }
        v2 = v68;
        v2 &= 1;
        if (!v2 || !g_42d1c8)
        {
            if ((v44 & 0xf000) == 0x4000 & ((unsigned int)v24 & 0xf000) == 0x4000)
            {
                if (g_42d1c0 == 7)
                    sub_410966("-D option not supported with directories"); /* do not return */
                if (a0 && g_42d5c9 != 1)
                    sub_4109a8("Common subdirectories: %s and %s\n", v20, v40);
                else
                    v3 = sub_40c21b(&v19, compare_files, compare_files);
            }
            else if (((unsigned short)v44 & 0xf000) == 0x4000 || ((unsigned short)v24 & 0xf000) == 0x4000 || a0 && (((unsigned short)v24 & 0xf000) != 0x8000 && ((unsigned short)v24 & 0xf000) != 0xa000 || ((unsigned short)v44 & 0xf000) != 0x8000 && ((unsigned short)v44 & 0xf000) != 0xa000))
            {
                if (v19 == 0xffffffff || v39 == 0xffffffff)
                {
                    if ((((unsigned short)v44 & 0xf000) == 0x4000 || ((unsigned short)v24 & 0xf000) == 0x4000) && g_42d5c9 && (g_42d648 || g_42d649 && v19 == 0xffffffff))
                    {
                        v3 = sub_40c21b(&v19, compare_files, compare_files);
                    }
                    else
                    {
                        if (!a0)
                            __assert_fail(); /* do not return */
                        v71 = a0;
                        v15 = v71[1 + 38 * (v19 == 0xffffffff)];
                        sub_4109a8("Only in %s: %s\n", v15, v1, v71);
                        v3 = 1;
                    }
                }
                else
                {
                    sub_414997(&v41);
                    sub_414997(&v21);
                    sub_4109e2("File %s is a %s while file %s is a %s\n", (!*((long long *)&g_42d200) ? v20 : *((long long *)&g_42d200)));
                    v3 = 1;
                }
            }
            else
            {
                if (((unsigned short)v24 & 0xf000) == 0xa000 || ((unsigned short)v44 & 0xf000) == 0xa000)
                {
                    if (!g_42d1f0)
                        __assert_fail(); /* do not return */
                    if (((unsigned short)v24 & 0xf000) == 0xa000 && ((unsigned short)v44 & 0xf000) == 0xa000)
                    {
                        v17 = 0;
                        v18 = NULL;
                        for (m = 0; m <= 1; m += 1)
                        {
                            (&v17)[m] = sub_41cf7b((&v20)[38 * m]);
                            if (!(&v17)[m])
                            {
                                sub_4108ec((&v20)[38 * m]);
                                v3 = 2;
                                break;
                            }
                        }
                        if (!v3 && strcmp(v17, v18))
                        {
                            sub_4109a8("Symbolic links %s and %s differ\n", v20, v40);
                            v3 = 1;
                        }
                        for (n = 0; n <= 1; n += 1)
                        {
                            free((&v17)[n]);
                        }
                    }
                    else
                    {
                        sub_414997(&v41);
                        sub_414997(&v21);
                        sub_4109e2("File %s is a %s while file %s is a %s\n", (!*((long long *)&g_42d200) ? v20 : *((long long *)&g_42d200)));
                        v3 = 1;
                    }
                }
                else if (g_42d1ed && ((unsigned short)v24 & 0xf000) == 0x8000 && ((unsigned short)v44 & 0xf000) == 0x8000 && v27 != v48 && v27 > 0 && v48 > 0)
                {
                    sub_4109a8("Files %s and %s differ\n", (!*((long long *)&g_42d200) ? v20 : *((long long *)&g_42d200)));
                    v3 = 1;
                }
                else
                {
                    v6 = 0;
                    if (v19 == 0xfffffffe)
                    {
                        v19 = open(v20, v6, 0);
                        if (v19 < 0)
                        {
                            sub_4108ec(v20);
                            v3 = 2;
                        }
                    }
                    if (v39 == 0xfffffffe)
                    {
                        if (v2)
                        {
                            v39 = v19;
                        }
                        else
                        {
                            v39 = open(v40, v6, 0);
                            if (v39 < 0)
                            {
                                sub_4108ec(v40);
                                v3 = 2;
                            }
                        }
                    }
                    if (!v3)
                        v3 = sub_407206(&v19);
                    if (v19 >= 0 && close(v19))
                    {
                        sub_4108ec(v20);
                        v3 = 2;
                    }
                    if (v39 >= 0 && v19 != v39 && close(v39))
                    {
                        sub_4108ec(v40);
                        v3 = 2;
                    }
                }
            }
        }
    }
    if (!v3)
    {
        if (g_42d64a && ((unsigned short)v24 & 0xf000) != 0x4000)
            sub_4109a8("Files %s and %s are identical\n", (!*((long long *)&g_42d200) ? v20 : *((long long *)&g_42d200)));
    }
    else
    {
        if (fflush_unlocked(stdout))
            sub_410925(gettext("standard output")); /* do not return */
    }
    free(v8);
    free(v9);
    return v3;
}



// Function: dir_read @ 0xbde2
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
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
} struct_0;

extern long long g_42d340;

unsigned long long dir_read(struct_0 *a0, void* idx)
{
    DIR *ptr;  // rbx
    dirent *count;  // r12
    unsigned long long index;  // rbx
    unsigned int v0;  // [bp-0x54]
    unsigned long long idx1;  // [bp-0x50]
    unsigned long v2;  // [bp-0x48]
    unsigned long long v3;  // [bp-0x40]
    unsigned long long v4;  // [bp-0x38]
    dirent *v5;  // [bp-0x30]
    unsigned long n;  // [bp-0x28]
    unsigned long long v7;  // [bp-0x20]

    *((unsigned long long *)&idx[8]) = 0;
    *((unsigned long *)&idx[16]) = 0;
    idx1 = 0;
    v2 = 0;
    if (a0->field_0 != 0xffffffff)
    {
        ptr = opendir(a0->field_8);
        if (!ptr)
            return 0;
        v3 = 0x200;
        v4 = 0;
        v2 = sub_41c945(v3);
        *((unsigned long *)&idx[16]) = v2;
        while (1)
        {
            *(__errno_location()) = 0;
            count = readdir(ptr);
            if (!count)
                break;
            v5 = (char *)&count[1].d_ino + 6;
            if (((char)v5->d_ino != 46 || *((char *)&v5->d_ino + 1) && (*((char *)&v5->d_ino + 1) != 46 || *((char *)&v5->d_ino + 2))) && !(char)sub_4141f6(g_42d340, v5, v5))
            {
                for (n = strlen((char *)&count[1].d_ino + 6) + 1; v3 < n + v4; *((unsigned long *)&idx[16]) = v2)
                {
                    if (v3 > 0x3ffffffffffffffe)
                        sub_41cf09(); /* do not return */
                    v3 *= 2;
                    v2 = sub_41c991(v2, v3, v3);
                }
                memcpy(v2 + v4, v5, n);
                v4 += n;
                idx1 += 1;
            }
        }
        if (*(__errno_location()))
        {
            v0 = *(__errno_location());
            closedir(ptr);
            *(__errno_location()) = v0;
            return 0;
        }
        else if (closedir(ptr))
        {
            return 0;
        }
    }
    if (idx1 > 0xffffffffffffffd)
        sub_41cf09(); /* do not return */
    v7 = sub_41c945((idx1 + 1) * 8);
    *((unsigned long long *)&idx[8]) = v7;
    *((unsigned long long *)idx) = idx1;
    for (index = 0; index < idx1; index += 1)
    {
        *((unsigned long *)(index * 8 + v7)) = v2;
        v2 += strlen(v2) + 1;
    }
    *((unsigned long *)(v7 + idx1 * 8)) = 0;
    return 1;
}



// Function: compare_collated @ 0xc084
extern char g_42d1ef;
extern long long g_42d680;

unsigned int compare_collated(char *a0, char *a1)
{
    char *v2;  // rax
    int *err;  // rax
    unsigned int v0;  // [bp-0x1c]

    *(__errno_location()) = 0;
    v0 = (!g_42d1ef ? (int)strcoll(a0, a1) : (int)strcasecmp(a0, a1));
    if (!*(__errno_location()))
        return v0;
    v2 = gettext("cannot compare file names '%s' and '%s'");
    err = __errno_location();
    error(0, *(err), v2);
    longjmp(&g_42d680, 1); /* do not return */
}



// Function: compare_names @ 0xc140
extern char g_42d1ef;
extern char g_42d660;

int compare_names(char *a0, char *a1)
{
    unsigned int v0;  // [bp-0xc]

    if (g_42d660)
    {
        v0 = sub_40c084(a0, a1);
        if (v0 || g_42d1ef)
            return v0;
    }
    return strcmp(a0, a1);
}



// Function: compare_names_for_qsort @ 0xc1a0
extern char g_42d660;

int compare_names_for_qsort(unsigned long long *a0, unsigned long long *a1)
{
    unsigned int v0;  // [bp-0x2c]
    unsigned long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10]

    v1 = *(a0);
    v2 = *(a1);
    if (g_42d660)
    {
        v0 = sub_40c084(v1, v2);
        if (v0)
            return v0;
    }
    return strcmp(v1, v2);
}



// Function: diff_dirs @ 0xc21b
typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[300];
    unsigned int field_130;
    char padding_134[300];
    unsigned long long field_260;
} struct_0;

extern char g_42d1ef;
extern char *g_42d2b8;
extern char g_42d660;
extern long long g_42d680;

int diff_dirs(struct_0 *a0, unsigned long long *a1)
{
    unsigned long long *v19;  // rax
    unsigned long v20;  // rax
    int l;  // [bp-0xa0]
    int iter;  // [bp-0x9c]
    int v2;  // [bp-0x98]
    unsigned int v3;  // [bp-0x94]
    unsigned int v4;  // [bp-0x90]
    int v5;  // [bp-0x8c]
    unsigned int v6;  // [bp-0x88]
    int v7;  // [bp-0x84]
    unsigned long m;  // [bp-0x80]
    struct_1 *ptr;  // [bp-0x78]
    unsigned long v10;  // [bp-0x70]
    unsigned long long *j;  // [bp-0x68]
    unsigned long long node;  // [bp-0x60]
    char v13;  // [bp-0x58]
    unsigned long long *v14;  // [bp-0x50]
    char v15;  // [bp-0x48]
    unsigned long long v16;  // [bp-0x38]

    l = 0;
    if ((a0->field_0 == 0xffffffff || (char)sub_40c796(a0, 0)) && (a0->field_130 == 0xffffffff || (char)sub_40c796(a0, 1)))
    {
        error(0, 0, gettext("%s: recursive directory loop"));
        return 2;
    }
    for (iter = 0; iter <= 1; iter += 1)
    {
        if ((char)sub_40bde2(&(&a0->field_0)[76 * iter], &(&v13)[24 * iter]) != 1)
        {
            sub_4108ec(*((long long *)&(&a0->field_0)[76 * iter + 2]));
            l = 2;
        }
    }
    if (!l)
    {
        j = v14;
        node = v16;
        g_42d660 = 1;
        if (_setjmp(&g_42d680))
            g_42d660 = 0;
        for (iter = 0; iter <= 1; iter += 1)
        {
            qsort((&j)[iter], *((long long *)&(&v13)[24 * iter]), 8, sub_40c1a0);
        }
        if (g_42d2b8 && !a0->field_260)
        {
            for (; *(j) && sub_40c140(*(j), g_42d2b8) < 0; j += 1);
            for (; *((long long *)node) && sub_40c140(*((long long *)node), g_42d2b8) < 0; node += 8);
        }
        for (; *(j) || *((long long *)node); l = v7)
        {
            v2 = (!*(j) ? 1 : (!*((long long *)node) ? 0xffffffff : (int)sub_40c140(*(j), *((long long *)node))));
            if (!v2 && g_42d1ef)
            {
                v3 = strcmp(*(j), *((long long *)node));
                if (v3)
                {
                    v4 = v3 >> 31 & 0xff;
                    v5 = 1 - v4;
                    ptr = (&j)[v5];
                    v10 = *((&j)[v4]);
                    for (m = &ptr->field_8; *((long long *)m) && !sub_40c140(*((long long *)m), v10); m += 8)
                    {
                        v6 = strcmp(*((long long *)m), v10);
                        if (v6 >= 0)
                        {
                            if (!v6)
                            {
                                memmove(&ptr->field_8, ptr, m - (char *)ptr);
                                *((unsigned long *)&ptr->padding_0[0]) = v10;
                            }
                            break;
                        }
                    }
                }
            }
            if (v2 >= 0)
                node += 8;
            if (v2 <= 0)
            {
                v19 = j;
                j = v19 + 1;
                v20 = *(v19);
            }
            else
            {
                v20 = 0;
            }
            v7 = a1(a0, v20);
            if (v7 <= l)
                continue;
        }
    }
    for (iter = 0; iter <= 1; iter += 1)
    {
        free((&v14)[3 * iter]);
        free(*((long long *)&(&v15)[24 * iter]));
    }
    return l;
}



// Function: dir_loop @ 0xc796
typedef struct struct_0 {
    char padding_0[608];
    struct struct_0 *field_260;
} struct_0;

unsigned int dir_loop(struct_0 *a0, int a1)
{
    int v2;  // eax
    struct_0 *iter;  // [bp-0x10]

    iter = a0;
    while (1)
    {
        iter = iter->field_260;
        if (!iter)
            return 0;
        if (*((long long *)&iter->padding_0[24 + 304 * a1]) == *((long long *)&a0->padding_0[24 + 304 * a1]) && *((long long *)&iter->padding_0[16 + 304 * a1]) == *((long long *)&a0->padding_0[16 + 304 * a1]) || (((short)*((int *)&iter->padding_0[40 + 304 * a1]) & 0xf000) == 0x6000 && ((short)*((int *)&a0->padding_0[40 + 304 * a1]) & 0xf000) == 0x6000 || ((short)*((int *)&iter->padding_0[40 + 304 * a1]) & 0xf000) == 0x2000 && ((short)*((int *)&a0->padding_0[40 + 304 * a1]) & 0xf000) == 0x2000) && *((long long *)&iter->padding_0[56 + 304 * a1]) == *((long long *)&a0->padding_0[56 + 304 * a1]))
            v2 = 1;
        else
            v2 = 0;
        if (v2 > 0)
            return 1;
    }
}



// Function: find_dir_file_pathname @ 0xc9bd
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
} struct_0;

extern char g_42d1ef;
extern char g_42d660;
extern long long g_42d680;

unsigned long long find_dir_file_pathname(long long a0, unsigned long a1)
{
    unsigned long v0;  // [bp-0x180]
    void* iter;  // [bp-0x178]
    unsigned long long v2;  // [bp-0x170]
    char v3;  // [bp-0x168]
    void* v4;  // [bp-0x160]
    void* v5;  // [bp-0x158]
    struct_0 v6;  // [bp-0x148]
    long long v7;  // [bp-0x140]

    v0 = a1;
    v4 = NULL;
    v5 = NULL;
    if (g_42d1ef)
    {
        v7 = a0;
        v6 = (struct_0)0;
        if ((char)sub_40bde2(&v6, &v3))
        {
            g_42d660 = 1;
            if (_setjmp(&g_42d680))
            {
                v0 = a1;
            }
            else
            {
                for (iter = v4; *((long long *)iter); iter += 8)
                {
                    if (!sub_40c140(*((long long *)iter), a1))
                    {
                        if (!strcmp(*((long long *)iter), a1))
                        {
                            v0 = *((long long *)iter);
                            break;
                        }
                        else if (v0 == a1)
                        {
                            v0 = *((long long *)iter);
                        }
                    }
                }
            }
        }
    }
    v2 = sub_414ade(a0, v0, 0, v0);
    free(v4);
    free(v5);
    return v2;
}



// Function: print_ed_script @ 0xcb85
long long print_ed_script(long long a0)
{
    return sub_41245b(a0, sub_412449, sub_40cbb5, sub_412449);
}



// Function: print_ed_hunk @ 0xcbb5
typedef struct FILE {
} FILE;

extern long long g_425164;
extern char g_425780;
extern long long g_42d360;
extern long long g_42d548;
extern FILE *g_42d5c0;

void print_ed_hunk(long long a0)
{
    char v0;  // [bp-0x3d]
    unsigned int v1;  // [bp-0x3c]
    char v2;  // [bp-0x38]
    char v3;  // [bp-0x30]
    char v4;  // [bp-0x28]
    char v5;  // [bp-0x20]
    long long i;  // [bp-0x18]

    v1 = sub_412af4(a0, &v2, &v3, &v4, &v5);
    if (!v1)
        return;
    sub_411b50();
    sub_412a37(44, &g_42d360, v2, v3);
    fputc_unlocked(*(&(&g_425780)[v1]), g_42d5c0);
    fputc_unlocked(10, g_42d5c0);
    if (v1 == 1)
        return;
    v0 = 1;
    for (i = v4; i <= v5; i += 1)
    {
        if (v0 != 1)
        {
            fputs_unlocked("a\n", g_42d5c0);
            v0 = 1;
        }
        if (*((char *)*((long long *)(i * 8 + g_42d548))) == 46 && *((char *)(*((long long *)(i * 8 + g_42d548)) + 1)) == 10)
        {
            fputs_unlocked("..\n.\ns/.//\n", g_42d5c0);
            v0 = 0;
        }
        else
        {
            sub_4124cf(&g_425164, i * 8 + g_42d548, g_42d548);
        }
    }
    if (!v0)
        return;
    fputs_unlocked(".\n", g_42d5c0);
    return;
}



// Function: pr_forward_ed_script @ 0xcd6a
long long pr_forward_ed_script(long long a0)
{
    return sub_41245b(a0, sub_412437, sub_40cd9a, sub_412437);
}



// Function: pr_forward_ed_hunk @ 0xcd9a
typedef struct FILE {
} FILE;

extern long long g_425164;
extern char g_425780;
extern long long g_42d360;
extern long long g_42d548;
extern FILE *g_42d5c0;

void pr_forward_ed_hunk(long long a0)
{
    unsigned int v0;  // [bp-0x3c]
    char v1;  // [bp-0x38]
    char v2;  // [bp-0x30]
    char v3;  // [bp-0x28]
    char v4;  // [bp-0x20]
    long long i;  // [bp-0x18]

    v0 = sub_412af4(a0, &v1, &v2, &v3, &v4);
    if (!v0)
        return;
    sub_411b50();
    fputc_unlocked(*(&(&g_425780)[v0]), g_42d5c0);
    sub_412a37(32, &g_42d360, v1, v2);
    fputc_unlocked(10, g_42d5c0);
    if (v0 == 1)
        return;
    for (i = v3; i <= v4; i += 1)
    {
        sub_4124cf(&g_425164, i * 8 + g_42d548, g_42d548);
    }
    fputs_unlocked(".\n", g_42d5c0);
    return;
}



// Function: print_rcs_script @ 0xceba
long long print_rcs_script(long long a0)
{
    return sub_41245b(a0, sub_412437, sub_40ceea, sub_412437);
}



// Function: print_rcs_hunk @ 0xceea
typedef struct FILE {
} FILE;

extern long long g_425164;
extern long long g_42d360;
extern long long g_42d490;
extern long long g_42d548;
extern FILE *g_42d5c0;

void print_rcs_hunk(long long a0)
{
    unsigned int v0;  // [bp-0x5c]
    char v1;  // [bp-0x58]
    char v2;  // [bp-0x50]
    char v3;  // [bp-0x48]
    char v4;  // [bp-0x40]
    long long v5;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    char v7;  // [bp-0x28], Other Possible Types: long long
    char v8;  // [bp-0x20], Other Possible Types: long long
    long long i;  // [bp-0x18]

    v0 = sub_412af4(a0, &v1, &v2, &v3, &v4);
    if (!v0)
        return;
    sub_411b50();
    sub_4129d0(&g_42d360, v1, v2, &v5, &v6);
    if ((char)v0 & 1)
        fprintf(g_42d5c0, "d%ld %ld\n", v5, (v5 <= v6 ? v6 - v5 + 1 : 1));
    if (!((char)v0 & 2))
        return;
    sub_4129d0(&g_42d490, v3, v4, &v7, &v8);
    fprintf(g_42d5c0, "a%ld %ld\n", v6, (v7 <= v8 ? v8 - v7 + 1 : 1));
    for (i = v3; i <= v4; i += 1)
    {
        sub_4124cf(&g_425164, i * 8 + g_42d548, g_42d548);
    }
    return;
}



// Function: print_ifdef_script @ 0xd076
extern long long g_42d2e0;
extern char g_42d430;
extern unsigned long long g_42d448;
extern char g_42d560;
extern long long g_42d748;
extern long long g_42d750;

long long print_ifdef_script(long long a0)
{
    g_42d750 = -(g_42d448);
    g_42d748 = g_42d750;
    sub_41245b(a0, sub_412437, sub_40d11e, sub_412437);
    if (*((long long *)&g_42d430) <= g_42d748 && *((long long *)&g_42d560) <= g_42d750)
        return g_42d750;
    sub_411b50();
    return sub_40d221(g_42d2e0, g_42d748, *((long long *)&g_42d430), g_42d750, *((long long *)&g_42d560));
}



// Function: print_ifdef_hunk @ 0xd11e
typedef struct struct_0 {
    char field_0;
} struct_0;

extern struct_0 g_42d2e0;
extern long long g_42d748;
extern long long g_42d750;

void print_ifdef_hunk(long long a0)
{
    unsigned int v0;  // [bp-0x34]
    char v1;  // [bp-0x30]
    char v2;  // [bp-0x28]
    char v3;  // [bp-0x20]
    char v4;  // [bp-0x18]

    v0 = sub_412af4(a0, &v1, &v2, &v3, &v4);
    if (!v0)
        return;
    sub_411b50();
    if (g_42d748 < v1 || g_42d750 < v3)
        sub_40d221(*((long long *)&g_42d2e0), g_42d748, v1, g_42d750, v3);
    g_42d748 = v2 + 1;
    g_42d750 = v4 + 1;
    sub_40d221(*((long long *)&(&g_42d2e0.field_0)[8 * v0]), v1, g_42d748, v3, g_42d750);
    return;
}



// Function: format_ifdef @ 0xd221
extern char g_42d360;
extern unsigned long long g_42d490;
extern long long g_42d5c0;

void format_ifdef(long long a0, unsigned long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4)
{
    unsigned long v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x40]
    unsigned long long v2;  // [bp-0x38]
    unsigned long long v3;  // [bp-0x30]
    unsigned long long v4;  // [bp-0x28]
    unsigned long long v5;  // [bp-0x20]

    v0 = &g_42d360;
    v1 = a1;
    v2 = a2;
    v3 = &g_42d490;
    v4 = a3;
    v5 = a4;
    sub_40d2bc(g_42d5c0, a0, 0, &v0);
    return;
}



// Function: format_group @ 0xd2bc
extern char g_42517c;
extern long long g_42d300;
extern long long g_42d308;
extern long long g_42d310;

char * format_group(unsigned long a0, char *a1, char a2, long long a3)
{
    char *iter;  // rbx
    char v9;  // r13b
    char *v10;  // rax
    char *v11;  // rbx
    unsigned long v12;  // rdx
    char *v13;  // rax
    int v0;  // [bp-0x6c]
    char v1;  // [bp-0x68]
    unsigned long v2;  // [bp-0x60]
    unsigned long v3;  // [bp-0x58]
    char *v4;  // [bp-0x50]
    int <0x40d2bc[is_3]|Stack bp-0x48, 1 B>;  // [bp-0x48]
    unsigned long v6;  // [bp-0x40]

    iter = a1;
    while (1)
    {
        while (1)
        {
            v9 = *(iter);
            if (v9 == a2)
                return iter;
            switch (v9)
            {
            case 0:
                return iter;
            case 37:
                iter += 1;
                v4 = iter;
                break;
            default:
                iter += 1;
                v4 = iter;
                break;
            }
            v10 = iter;
            iter = v10 + 1;
            v9 = *(v10);
            switch (v9)
            {
            case 37:
LABEL_40d52f:
                if (a0)
                    putc_unlocked(v9, a0);
                break;
            case 40:
                for (v0 = 0; v0 <= 1; v0 += 1)
                {
                    if (*(iter) - 48 <= 9)
                    {
                        *(__errno_location()) = 0;
                        (&<0x40d2bc[is_3]|Stack bp-0x48, 1 B>)[v0] = strtoimax(iter, &v1, 10);
                        if (*(__errno_location()))
                            break;
                        v11 = (char *)v1;
                    }
                    else
                    {
                        v12 = *(iter);
                        (&<0x40d2bc[is_3]|Stack bp-0x48, 1 B>)[v0] = sub_40d577(a3, v12 & 0xffffffff, v12);
                        if ((&<0x40d2bc[is_3]|Stack bp-0x48, 1 B>)[v0] < 0)
                            break;
                        v11 = iter + 1;
                    }
                    v13 = v11;
                    iter = v13 + 1;
                    if (*(v13) != *(&(&g_42517c)[v0]))
                        break;
                }
                if (<0x40d2bc[is_3]|Stack bp-0x48, 1 B> == v6)
                {
                    v2 = a0;
                    v3 = 0;
                }
                else
                {
                    v2 = 0;
                    v3 = a0;
                }
                iter = format_group(v2, iter, 58, a3);
                if (*(iter))
                {
                    iter = format_group(v3, iter + 1, 41, a3);
                    if (*(iter))
                    {
                        iter += 1;
                        continue;
                    }
                }
                break;
            case 60:
                sub_40d6bd(a0, g_42d308, a3);
                continue;
            case 61:
                sub_40d6bd(a0, g_42d300, a3);
                continue;
            case 62:
                sub_40d6bd(a0, g_42d310, a3 + 24);
                continue;
            default:
                iter = sub_40da03(a0, iter - 2, 0, 0, a3);
                if (!iter)
                    break;
                else
                    continue;
            }
        }
        v9 = 37;
        iter = v4;
        goto LABEL_40d52f;
    }
}



// Function: groups_letter_value @ 0xd577

unsigned long long groups_letter_value(long long *a0, char a1)
{
    char v0;  // [bp-0x14]
    long long *iter;  // [bp-0x10]

    iter = a0;
    v0 = a1;
    switch (v0)
    {
    case 69:
        v0 = 101;
        iter += 3;
        break;
    case 70:
        v0 = 0x66;
        iter += 3;
        break;
    case 76:
        v0 = 108;
        iter += 3;
        break;
    case 77:
        v0 = 109;
        iter += 3;
        break;
    case 78:
        v0 = 110;
        iter += 3;
        break;
    }
    switch (v0)
    {
    case 101:
        return sub_4129a8(*(iter), iter[1], iter[1]) - 1;
    case 102:
        return sub_4129a8(*(iter), iter[1], iter[1]);
    case 108:
        return sub_4129a8(*(iter), iter[2], iter[2]) - 1;
    case 109:
        return sub_4129a8(*(iter), iter[2], iter[2]);
    case 110:
        return iter[2] - iter[1];
    default:
        return 0xffffffffffffffff;
    }
}



// Function: print_ifdef_lines @ 0xd6bd
typedef struct struct_0 {
    char padding_0[184];
    unsigned long long field_b8;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_1;

extern char g_42d2a1;

long long print_ifdef_lines(unsigned long a0, char *a1, struct_1 *idx)
{
    char *v7;  // rbx
    char *v8;  // rax
    char v9;  // r12b
    char *v10;  // rax
    char *v0;  // [bp-0x60]
    unsigned long v1;  // [bp-0x50]
    struct_0 *v2;  // [bp-0x48]
    unsigned long v3;  // [bp-0x40]
    unsigned long v4;  // [bp-0x38]
    char *v5;  // [bp-0x30]

    v0 = a1;
    v2 = idx->field_0;
    v3 = v2->field_b8;
    v1 = idx->field_8;
    v4 = idx->field_10;
    if (!a0)
        return idx->field_10;
    if (g_42d2a1 != 1 && *(v0) == 37)
    {
        if (v0[1] == 108 && v0[2] == 10 && !v0[3] && v1 < v4)
            return fwrite_unlocked(*((long long *)(v3 + v1 * 8)), 1, *((long long *)(v3 + v4 * 8)) + (*((char *)(*((long long *)(v3 + v4 * 8)) - 1)) != 10) - *((long long *)(v3 + v1 * 8)), a0);
        if (v0[1] == 76 && !v0[2])
            return fwrite_unlocked(*((long long *)(v3 + v1 * 8)), 1, *((long long *)(v3 + v4 * 8)) - *((long long *)(v3 + v1 * 8)), a0);
    }
LABEL_40d9e7:
    if (v1 >= v4)
        return v1;
    v7 = v0;
    while (1)
    {
        v8 = v7;
        v7 = v8 + 1;
        v9 = *(v8);
        switch (v9)
        {
        case 0:
            v1 += 1;
            goto LABEL_40d9e7;
        case 37:
            v5 = v7;
            v10 = v7;
            v7 = v10 + 1;
            v9 = *(v10);
            switch (*(v10))
            {
            case 108:
                sub_41267d(*((long long *)(v3 + v1 * 8)), *((long long *)(v3 + (v1 + 1) * 8)) + -(*((char *)(*((long long *)(v3 + (v1 + 1) * 8)) - 1)) == 10), 0, 0);
                continue;
            case 37:
LABEL_40d9c0:
                putc_unlocked(v9, a0);
            case 76:
                sub_41267d(*((long long *)(v3 + v1 * 8)), *((long long *)(v3 + (v1 + 1) * 8)), 0, 0);
                continue;
            default:
                v7 = sub_40da03(a0, v7 - 2, v2, v1, 0);
                if (v7)
                    continue;
                v9 = 37;
                v7 = v5;
                break;
            }
            break;
        default:
            v5 = v7;
            goto LABEL_40d9c0;
        }
    }
    return v1;
}



// Function: do_printf_spec @ 0xda03
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

struct_0 * do_printf_spec(FILE *a0, struct_0 *a1, long long a2, long long a3, long long *a4)
{
    struct_0 *v14;  // rax
    struct_0 *v15;  // rax
    struct_0 *v16;  // rax
    struct_0 *v17;  // rax
    unsigned int v18;  // eax
    unsigned long long v19;  // rax
    unsigned long long v20;  // rax
    unsigned long long v21;  // rax
    char v0;  // [bp-0x1088]
    char v1;  // [bp-0x88]
    char v2;  // [bp-0x4b]
    char i;  // [bp-0x4a]
    char v4;  // [bp-0x49]
    struct_0 *v5;  // [bp-0x48]
    unsigned long long v6;  // [bp-0x40]
    unsigned long long v7;  // [bp-0x38]
    unsigned long n;  // [bp-0x30]
    unsigned long n1;  // [bp-0x28]
    unsigned long long v10;  // [bp-0x20]
    char *v11;  // [bp-0x18]
    char *v12;  // [bp-0x18]

    v5 = a1;
    v5 = &v5->field_1;
    while (1)
    {
        v14 = v5;
        v5 = &v14->field_1;
        i = (char)v14->padding_0;
        if (i != 45 && i != 39 && i != 48)
            break;
    }
    for (; i - 48 <= 9; i = (char)v15->padding_0)
    {
        v15 = v5;
        v5 = &v15->field_1;
    }
    if (i == 46)
    {
        do
        {
            v16 = v5;
            v5 = &v16->field_1;
            i = (char)v16->padding_0;
        } while (i - 48 <= 9);
    }
    v17 = v5;
    v5 = &v17->field_1;
    v4 = (char)v17->padding_0;
    v18 = i - 88;
    if (32 < v18)
        return NULL;
    v19 = 1 << ((char)v18 & 63);
    if (4303360001 & v19)
    {
        if (a2)
        {
            if (v4 != 110)
                return NULL;
            v6 = sub_4129a8(a2, a3, a3);
        }
        else
        {
            v6 = sub_40d577(a4, v4);
            if (v6 - 0 >> 63 & 1)
                return NULL;
        }
        if (a0)
        {
            v7 = v6;
            n = v5 - a1 - 2;
            n1 = 1;
            if (n1 + n + 2 <= 4000)
            {
                v20 = (unsigned long long)((15 + n1 + n + 41) / 16) * 16;
                while (&v0 != &(&v1)[-1 * (v20 & 0xfffffffffffff000)])
                ;
                /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v20 & 0xfff)];
                if ((unsigned short)v20 & 0xfff)
                    *((long long *)(((unsigned int)v20 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v20 & 0xfff) - 8 + /* unsupported instruction */));
                v21 = (/* unsupported instruction */ + 15 >> 4) * 16 + 31 & 0xffffffffffffffe0;
            }
            else
            {
                v21 = sub_41cf49(n1 + n + 2);
            }
            v10 = v21;
            v11 = v10 + n + n1;
            memcpy(v10, a1, n);
            memcpy(v10 + n, "l", n1);
            v12 = v11 + 1;
            *(v11) = i;
            *(v12) = 0;
            fprintf(a0, v10);
            sub_4168f3(v10);
        }
    }
    else if (!((unsigned short)v19 & 0x800))
    {
        return NULL;
    }
    else if (v4 != 39)
    {
        return NULL;
    }
    else
    {
        v5 = sub_40dd74(v5, &v2, &v2);
        if (!v5)
        {
            return NULL;
        }
        else if (a0)
        {
            putc_unlocked(v2, a0);
        }
    }
    return v5;
}



// Function: scan_char_literal @ 0xdd74
char * scan_char_literal(char *a0, char *a1)
{
    char *v5;  // rbx
    char *v7;  // rax
    char *v8;  // rax
    char v0;  // [bp-0x1e]
    char v1;  // [bp-0x1d]
    unsigned int v2;  // [bp-0x1c]
    long long v3;  // [bp-0x18]

    v5 = a0 + 1;
    v1 = *(a0);
    switch (v1)
    {
    case 92:
        v0 = 0;
        while (1)
        {
            v8 = v5;
            v5 = v8 + 1;
            v1 = *(v8);
            if (v1 == 39)
                break;
            v2 = v1 - 48;
            if (v2 > 7)
                return NULL;
            v0 = v2 + v0 * 8;
        }
        v3 = v5 - a0 - 2;
        if (v3 <= 0 || v3 > 3)
            return NULL;
    case 0: case 39:
        return NULL;
    default:
        v0 = v1;
        v7 = v5;
        v5 = v7 + 1;
        if (*(v7) != 39)
            return NULL;
    }
    *(a1) = v0;
    return v5;
}



// Function: file_block_read @ 0xde51
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

struct_0 * file_block_read(struct_0 *idx, long long a1)
{
    struct_0 *v2;  // rax
    long long v3;  // rcx
    struct_0 *v4;  // rax
    long long v0;  // [bp-0x10]

    if (!a1)
        return v4;
    v2 = idx->field_121 ^ 1;
    if (!*((char *)&v2))
        return v2;
    v3 = idx->field_a0 + idx->field_b0;
    v0 = sub_41d1d3(idx->field_0, v3, a1, v3);
    if (v0 != -0x1)
    {
        idx->field_b0 = idx->field_b0 + v0;
        idx->field_121 = v0 < a1;
        return idx;
    }
    sub_410925(idx->field_8); /* do not return */
}



// Function: sip @ 0xdf00
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

unsigned long long sip(struct_0 *idx, char a1)
{
    void* ptr;  // rax
    unsigned int v0;  // [bp-0x14]
    unsigned long v1;  // [bp-0x10]

    if (idx->field_0 < 0)
    {
        *((unsigned long *)&idx->padding_a8[0]) = 8;
        idx->field_a0 = sub_41c945(idx->padding_a8);
    }
    else
    {
        *((unsigned long long *)&idx->padding_a8[0]) = sub_41d2a2(8, *((long long *)&idx->padding_10[56]), 0x7fffffffffffffef, *((long long *)&idx->padding_10[56]));
        idx->field_a0 = sub_41c945(idx->padding_a8);
        if (a1 != 1)
        {
            v0 = sub_4131e6(idx->field_0, 0);
            sub_40de51(idx, idx->padding_a8);
            v1 = idx->field_b0;
            if (v0)
            {
                if (lseek(idx->field_0, -(v1), 1) < 0)
                    sub_410925(idx->field_8); /* do not return */
                sub_4131e6(idx->field_0, v0, v0);
                idx->field_b0 = 0;
                idx->field_121 = 0;
            }
            ptr = memchr(idx->field_a0, 0, v1);
            return _INSERT(ptr, 0, ptr);
        }
    }
    idx->field_b0 = 0;
    idx->field_121 = 0;
    return 0;
}



// Function: slurp @ 0xe09d
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

long long slurp(struct_0 *idx)
{
    long long v3;  // rax
    long long v4;  // rax
    long long v0;  // [bp-0x18]
    long long v1;  // [bp-0x10], Other Possible Types: unsigned long

    v3 = idx->field_0;
    if ((int)v3 < 0)
        return v3;
    if (((short)*((int *)&idx->padding_10[24]) & 0xf000) == 0x8000)
    {
        v0 = *((long long *)&idx->padding_10[48]);
        v1 = (v0 & 0xfffffffffffffff8) + 16;
        if (*((long long *)&idx->padding_10[48]) != v0 || v1 < v0 || v1 > 0x7ffffffffffffffe)
            sub_41cf09(); /* do not return */
        if (v1 > idx->padding_a8)
        {
            *((long long *)&idx->padding_a8[0]) = v1;
            idx->field_a0 = sub_41c991(idx->field_a0, v1, v1);
        }
        if (v0 >= idx->field_b0)
        {
            sub_40de51(idx, v0 - idx->field_b0 + 1);
            if (v0 >= idx->field_b0)
                return idx->field_b0;
        }
    }
    sub_40de51(idx, idx->padding_a8 - idx->field_b0);
    if (!idx->field_b0)
        return idx->field_b0;
    while (idx->field_b0 == idx->padding_a8)
    {
        if (idx->padding_a8 > 0x3ffffffffffffff7)
            sub_41cf09(); /* do not return */
        idx->padding_a8 = idx->padding_a8 * 2;
        idx->field_a0 = sub_41c991(idx->field_a0, idx->padding_a8, idx->padding_a8);
        sub_40de51(idx, idx->padding_a8 - idx->field_b0);
    }
    v1 = idx->field_b0 + 16;
    *((unsigned long long *)&idx->padding_a8[0]) = v1 & 0xfffffffffffffff8;
    v4 = sub_41c991(idx->field_a0, idx->padding_a8, idx->padding_a8);
    idx->field_a0 = v4;
    return v4;
}



// Function: find_and_hash_each_line @ 0xe2f8
typedef struct struct_0 {
    char padding_0[160];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    unsigned long long field_b8;
    unsigned long long field_c0;
    long long field_c8;
    long long field_d0;
    long long field_d8;
    struct struct_1 *field_e0;
    char padding_e8[16];
    long long field_f8;
    char padding_100[32];
    char field_120;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned int g_42d1c0;
extern char g_42d1c8;
extern char g_42d1d0;
extern char g_42d1e8;
extern char g_42d1ee;
extern unsigned long long g_42d2a8;
extern unsigned long long g_42d758;
extern unsigned long long g_42d760;
extern unsigned long long g_42d768;
extern unsigned long long g_42d770;
extern unsigned long long g_42d778;

unsigned long long find_and_hash_each_line(struct_0 *idx)
{
    char *v27;  // rax
    unsigned long long *v28;  // rax
    char *v37;  // rax
    char *v38;  // rax
    char *v39;  // rax
    unsigned long long *v40;  // rax
    unsigned long long v42;  // rax
    char *v29;  // rax
    unsigned long long *v30;  // rax
    char *v32;  // rax
    unsigned long long *v33;  // rax
    char *v34;  // rax
    unsigned long long *v35;  // rax
    char v0;  // [bp-0xb1]
    char v1;  // [bp-0xb0]
    char v2;  // [bp-0xaf]
    char v3;  // [bp-0xae]
    char v4;  // [bp-0xad]
    unsigned int v5;  // [bp-0xac]
    char *iter;  // [bp-0xa8]
    long long idx1;  // [bp-0xa0], Other Possible Types: unsigned long long
    unsigned long long *v8;  // [bp-0x98]
    unsigned long iter1;  // [bp-0x90], Other Possible Types: long long, unsigned long long
    unsigned long v10;  // [bp-0x88]
    unsigned long iter2;  // [bp-0x80]
    long long v12;  // [bp-0x78]
    unsigned long v13;  // [bp-0x70]
    unsigned long long v14;  // [bp-0x68]
    unsigned long long v15;  // [bp-0x60]
    unsigned long v16;  // [bp-0x58]
    unsigned long long node;  // [bp-0x50]
    char *v18;  // [bp-0x48]
    unsigned long long i;  // [bp-0x40]
    unsigned long v20;  // [bp-0x38]
    char *v21;  // [bp-0x30]
    char *v22;  // [bp-0x28]
    char *v23;  // [bp-0x20]
    unsigned long v24;  // [bp-0x18]
    unsigned long v25;  // [bp-0x10]

    iter = &idx->field_e0->field_0;
    iter1 = idx->field_b8;
    v10 = idx->field_d8;
    iter2 = 0;
    v20 = idx->field_c0;
    v12 = sub_41c945(v10 * 8);
    v13 = g_42d768;
    v14 = g_42d770;
    v15 = g_42d778;
    v21 = *((long long *)&idx->padding_e8[8]);
    v22 = idx->field_b0 + idx->field_a0;
    v1 = g_42d1ee;
    v5 = *((int *)&g_42d1e8);
    v2 = v5;
    v3 = v2 || v1;
    while (1)
    {
        if (iter >= v21)
        {
            idx->field_c8 = iter2;
            idx1 = 0;
            while (1)
            {
                if (iter2 == v10)
                {
                    if (v10 > 3074457345618258601 || v10 * 2 - v20 > 0xffffffffffffffe || v10 - v20 > 0xffffffffffffffe)
                        sub_41cf09(); /* do not return */
                    v10 = v10 * 2 - v20;
                    iter1 += v20 * 8;
                    iter1 = sub_41c991(iter1, (v10 - v20) * 8, (v10 - v20) * 8);
                    iter1 += -(v20 * 8);
                }
                *((char **)(iter2 * 8 + iter1)) = iter;
                if (iter == v22)
                {
                    if (idx->field_120 && g_42d1c0 != 4 && g_42d1c0 != 5)
                        *((unsigned long long *)(iter1 + iter2 * 8)) = *((long long *)(iter1 + iter2 * 8)) - 1;
                    break;
                }
                else
                {
                    if (idx1 >= *((long long *)&g_42d1d0) && g_42d1c8)
                        break;
                    iter2 += 1;
                    do
                    {
                        iter += 1;
                    } while (*(iter) != 10);
                    idx1 += 1;
                }
            }
            idx->field_b8 = iter1;
            idx->field_d0 = iter2;
            idx->field_d8 = v10;
            idx->field_f8 = v12;
            g_42d768 = v13;
            g_42d778 = v15;
            g_42d770 = v14;
            return v14;
        }
        v23 = iter;
        v16 = 0;
        if (v5 == 5)
        {
            while (1)
            {
                v39 = iter;
                iter = v39 + 1;
                v0 = *(v39);
                if (v0 == 10)
                    break;
                v40 = __ctype_b_loc();
                if (*((short *)(v0 * 2 + *(v40))) & 0x2000)
                    continue;
                v16 = (!v1 ? v0 : (int)tolower(v0)) + __ROL__(v16, 7);
            }
        }
        else if (v5 > 5)
        {
LABEL_40e783:
            if (v1)
            {
                while (1)
                {
                    v37 = iter;
                    iter = v37 + 1;
                    v0 = *(v37);
                    if (v0 == 10)
                        break;
                    v16 = __ROL__(v16, 7) + (int)tolower(v0);
                }
            }
            else
            {
                while (1)
                {
                    v38 = iter;
                    iter = v38 + 1;
                    v0 = *(v38);
                    if (v0 == 10)
                        break;
                    v16 = __ROL__(v16, 7) + v0;
                }
            }
        }
        else if (v5 <= 3)
        {
            if (!v5)
                goto LABEL_40e783;
            node = 0;
            while (1)
            {
                v27 = iter;
                iter = v27 + 1;
                v0 = *(v27);
                if (v0 == 10)
                    break;
                if ((char)v5 & 2 && (v28 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)v0 * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000))
                {
                    v18 = iter;
                    do
                    {
                        v29 = v18;
                        v18 = v29 + 1;
                        v4 = *(v29);
                        if (v4 == 10)
                        {
                            iter = v18;
                            goto LABEL_40e811;
                        }
                    } while ((v30 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)v4 * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000));
                }
                i = 1;
                if ((char)v5 & 1)
                {
                    switch (v0)
                    {
                    case 13:
                        node = 0;
                        break;
                    case 8:
                        node -= node;
                        break;
                    case 9:
                        v0 = 32;
                        i = g_42d2a8 - node % g_42d2a8;
                        node = (node <= i + node ? i + node : 0);
                        break;
                    default:
                        node += 1;
                        break;
                    }
                }
                if (v1)
                    v0 = tolower(v0);
                do
                {
                    v16 = __ROL__(v16, 7) + v0;
                    i -= 1;
                } while (i);
            }
        }
        else
        {
            if (v5 != 4)
                goto LABEL_40e783;
            while (1)
            {
                v32 = iter;
                iter = v32 + 1;
                v0 = *(v32);
                if (v0 == 10)
                    break;
                v33 = __ctype_b_loc();
                if (*((short *)(v0 * 2 + *(v33))) & 0x2000)
                {
                    while (1)
                    {
                        v34 = iter;
                        iter = v34 + 1;
                        v0 = *(v34);
                        if (v0 == 10)
                            break;
                        v35 = __ctype_b_loc();
                        if (!(*((short *)(v0 * 2 + *(v35))) & 0x2000))
                        {
                            v16 = __ROL__(v16, 7) + 32;
                            goto LABEL_40e56f;
                        }
                    }
                }
                else
                {
LABEL_40e56f:
                    v16 = (!v1 ? v0 : (int)tolower(v0)) + __ROL__(v16, 7);
                }
            }
        }
LABEL_40e811:
        v8 = v16 % g_42d760 * 8 + g_42d758;
        v24 = iter - v23 - 1;
        if (iter == v22 && idx->field_120 && g_42d1c0 != 4 && g_42d1c0 != 5 && v5 <= 1)
            v8 = g_42d758 - 8;
        idx1 = *(v8);
        while (1)
        {
            if (!idx1)
            {
                v42 = v14;
                v14 = v42 + 1;
                idx1 = v42;
                if (idx1 == v15)
                {
                    if (v15 > 0x1fffffffffffffe)
                        sub_41cf09(); /* do not return */
                    v15 *= 2;
                    v13 = sub_41c991(v13, v15 * 32, v15 * 32);
                }
                *((unsigned long long *)(idx1 * 32 + v13)) = *(v8);
                *((unsigned long *)(idx1 * 32 + v13 + 8)) = v16;
                *((char **)(idx1 * 32 + v13 + 16)) = v23;
                *((unsigned long *)(idx1 * 32 + v13 + 24)) = v24;
                *(v8) = idx1;
                break;
            }
            if (v16 == *((long long *)(v13 + idx1 * 32 + 8)))
            {
                v25 = *((long long *)(v13 + idx1 * 32 + 16));
                if (v24 == *((long long *)(v13 + idx1 * 32 + 24)))
                {
                    if (!memcmp(v25, v23, v24))
                        break;
                    if (!(v3 ^ 1))
                        goto LABEL_40ea3f;
                }
                else if (v2 == 1)
                {
LABEL_40ea3f:
                    if ((char)sub_411f90(v25, v23, v23) != 1)
                        break;
                }
            }
            idx1 = *((long long *)(v13 + idx1 * 32));
        }
        if (iter2 == v10)
        {
            if (v10 > 3074457345618258601 || v10 * 2 - v20 > 0xffffffffffffffe || v10 - v20 > 0xffffffffffffffe)
                break;
            v10 = v10 * 2 - v20;
            v12 = sub_41c991(v12, v10 * 8, v10 * 8);
            iter1 += v20 * 8;
            iter1 = sub_41c991(iter1, (v10 - v20) * 8, (v10 - v20) * 8);
            iter1 += -(v20 * 8);
        }
        *((char **)(iter2 * 8 + iter1)) = v23;
        *((unsigned long long *)(iter2 * 8 + v12)) = idx1;
        iter2 += 1;
    }
    sub_41cf09(); /* do not return */
}



// Function: prepare_text @ 0xedad
typedef struct struct_0 {
    char padding_0[160];
    struct struct_0 *field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[104];
    char field_120;
} struct_0;

extern char g_42d2b2;

struct_0 * prepare_text(struct_0 *idx)
{
    int v8;  // eax
    unsigned long long v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x30]
    char *ptr;  // [bp-0x28]
    char *v3;  // [bp-0x28]
    char *iter;  // [bp-0x20]
    struct_0 *v5;  // [bp-0x18]
    char *v6;  // [bp-0x10]

    v0 = idx->field_b0;
    v5 = idx->field_a0;
    if (!v5)
        return idx->field_a0;
    if (g_42d2b2)
    {
        v6 = &v5->padding_0[v0];
        *(v6) = 13;
        ptr = rawmemchr(v5, 13);
        for (iter = ptr; iter != v6; ptr = v3)
        {
            if (*(iter) == 13 && iter[1] == 10)
                v8 = 1;
            else
                v8 = 0;
            iter = &iter[v8];
            v3 = ptr + 1;
            *(ptr) = *(iter);
            iter += 1;
        }
        v0 -= v6 - ptr;
    }
    if (v0 && *(&v5->padding_0[v0] - 1) != 10)
    {
        v1 = v0 + 1;
        v5->padding_0[v0] = 10;
        idx->field_120 = 1;
        v0 = v1;
    }
    memset(&v5->padding_0[v0], 0, 8);
    idx->field_b0 = v0;
    return idx;
}



// Function: guess_lines @ 0xeef5
unsigned long long guess_lines(unsigned long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v3;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v0 = (a0 <= 9 ? 32 : a1 / (a0 - 1));
    v3 = a2 / v0;
    if (!v3)
        v3 = 1;
    v1 = v3;
    if (v1 > 542551296285575042)
        v1 = 542551296285575042;
    return v1 + 5;
}



// Function: find_identical_ends @ 0xef6a
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[156];
    void* field_a0;
    unsigned long long field_a8;
    unsigned long long field_b0;
    unsigned long long field_b8;
    unsigned long long field_c0;
    char padding_c8[16];
    unsigned long long field_d8;
    void* field_e0;
    unsigned long long field_e8;
    void* field_f0;
    char padding_f8[40];
    char field_120;
    char padding_121[175];
    void* field_1d0;
    unsigned long long field_1d8;
    unsigned long long field_1e0;
    unsigned long long field_1e8;
    unsigned long long field_1f0;
    char padding_1f8[16];
    unsigned long long field_208;
    void* field_210;
    unsigned long long field_218;
    void* field_220;
    char padding_228[40];
    char field_250;
} struct_0;

extern char g_42d1c0;
extern char g_42d1c8;
extern char g_42d1d0;
extern unsigned long long g_42d1e0;
extern unsigned long long g_42d240;

struct_0 * find_identical_ends(struct_0 *idx)
{
    int v24;  // eax
    char v25;  // al
    unsigned long long v26;  // rax
    unsigned long long v27;  // rax
    struct_0 *idx1;  // rax
    struct_0 *idx2;  // rax
    char v0;  // [bp-0xb9]
    void* v1;  // [bp-0xb8]
    void* i;  // [bp-0xb0]
    void* iter;  // [bp-0xa8]
    void* node;  // [bp-0xa0]
    void* v5;  // [bp-0x98]
    long long v6;  // [bp-0x90]
    unsigned long iter1;  // [bp-0x88], Other Possible Types: unsigned long long
    unsigned long long v8;  // [bp-0x80]
    unsigned long long v9;  // [bp-0x78]
    unsigned long long k;  // [bp-0x70]
    void* v11;  // [bp-0x68]
    void* v12;  // [bp-0x60]
    unsigned long v13;  // [bp-0x58]
    unsigned long long v14;  // [bp-0x50]
    void* v15;  // [bp-0x48]
    unsigned long long v16;  // [bp-0x40]
    unsigned long long v17;  // [bp-0x38]
    unsigned long long v18;  // [bp-0x30]
    unsigned long long v19;  // [bp-0x28]
    unsigned long long v20;  // [bp-0x20]
    unsigned long long v21;  // [bp-0x18]
    unsigned long long v22;  // [bp-0x10]

    sub_40e09d(idx);
    sub_40edad(idx);
    if (idx->field_0 != *((int *)&idx->padding_121[15]))
    {
        sub_40e09d(&idx->padding_121[15]);
        sub_40edad(&idx->padding_121[15]);
    }
    else
    {
        idx->field_1d0 = idx->field_a0;
        idx->field_1d8 = idx->field_a8;
        idx->field_1e0 = idx->field_b0;
        idx->field_250 = idx->field_120;
    }
    v1 = idx->field_a0;
    i = idx->field_1d0;
    v11 = v1;
    iter = v11;
    v12 = i;
    node = v12;
    v13 = idx->field_b0;
    v14 = idx->field_1e0;
    if (iter == node)
    {
        node += v14;
        iter = node;
    }
    else
    {
        if (v13 < v14)
            *(v13 + (char *)iter) = ~(*(v13 + (char *)node));
        else
            *(v14 + (char *)node) = ~(*(v14 + (char *)iter));
        for (; *((long long *)v1) == *((long long *)i); i += 8)
        {
            v1 += 8;
        }
        iter = v1;
        for (node = i; *((char *)iter) == *((char *)node); node += 1)
        {
            iter += 1;
        }
        if (*((int *)&g_42d1c0) != 4 && *((int *)&g_42d1c0) != 5 && node > v12 + v14 - idx->field_250 ^ iter > v11 + v13 - idx->field_120)
        {
            iter -= 1;
            node -= 1;
        }
    }
    iter1 = g_42d1e0;
    while (1)
    {
        if (iter == v11 || !(iter1 = iter1, *((char *)((char *)iter - 1)) != 10 || !(iter1 = (unsigned long)(iter1 - 1), !iter1)))
            break;
        iter -= 1;
        node -= 1;
    }
    idx->field_e0 = iter;
    idx->field_210 = node;
    iter = v13 + v11;
    node = v14 + v12;
    if (*((int *)&g_42d1c0) == 4 || *((int *)&g_42d1c0) == 5 || (iter = iter, idx->field_120 == idx->field_250))
    {
        v15 = iter;
        v5 = (v13 < v14 ? 0 : v13 - v14) + idx->field_e0;
        do
        {
            if (iter == v5)
                goto LABEL_40f3ed;
        } while ((iter -= 1, node -= 1, *((char *)iter) == *((char *)node)));
        iter += 1;
        node += 1;
        v5 = iter;
LABEL_40f3ed:
        if (v11 != iter && *((char *)iter - 1) != 10 || v12 != node && *((char *)node - 1) != 10)
            v24 = 1;
        else
            v24 = 0;
        iter1 = g_42d1e0 + v24;
        while (1)
        {
            iter1 -= 1;
            if (!iter1 || iter == v15)
                break;
            do
            {
                iter += 1;
            } while (*((char *)iter) != 10);
        }
        node += iter - v5;
    }
    idx->field_f0 = iter;
    idx->field_220 = node;
    if (g_42d1c8 && !g_42d240 && *((long long *)&g_42d1d0) <= 0x1ffffffffffffffe && v13 > *((long long *)&g_42d1d0))
    {
        v16 = sub_40eef5(0, 0, iter - idx->field_e0);
        v17 = sub_40eef5(0, 0, v13 + v11 - iter);
        for (k = 1; k <= *((long long *)&g_42d1d0); k *= 2);
        if (*((long long *)&g_42d1d0) <= v17)
            v17 = *((long long *)&g_42d1d0);
        v9 = v17 + k + v16;
    }
    else
    {
        k = 0;
        v9 = sub_40eef5(0, 0, v13);
    }
    v18 = k - 1;
    v8 = 0;
    v6 = sub_41c945(v9 * 8);
    if (g_42d1c8 != 1 || iter != idx->field_e0 || node != idx->field_210)
        v25 = 1;
    else
        v25 = 0;
    v0 = v25;
    v0 &= 1;
    iter = v11;
    if (v0)
    {
        v15 = idx->field_e0;
        while (1)
        {
            if (iter == v15)
                break;
            v26 = v8;
            v8 = v26 + 1;
            v19 = v26 & v18;
            if (v19 == v9)
            {
                if (v9 > 0x7fffffffffffffe)
                    sub_41cf09(); /* do not return */
                v9 *= 2;
                v6 = sub_41c991(v6, v9 * 8, v9 * 8);
            }
            *((void* *)(v19 * 8 + v6)) = iter;
            do
            {
                iter += 1;
            } while (*((char *)iter) != 10);
        }
    }
    if (k && v8 > *((long long *)&g_42d1d0))
        v27 = *((long long *)&g_42d1d0);
    else
        v27 = v8;
    v20 = v27;
    v16 = sub_40eef5(v8, iter - v11, node - idx->field_210);
    v17 = sub_40eef5(v8, iter - v11, v14 + v12 - node);
    if (*((long long *)&g_42d1d0) <= v17)
        v17 = *((long long *)&g_42d1d0);
    v21 = v17 + v20 + v16;
    if (v21 >= v20 && v21 <= 0xffffffffffffffe)
    {
        v22 = sub_41c945(v21 * 8);
        if (v20 != v8)
        {
            for (iter1 = 0; iter1 < v20; iter1 += 1)
            {
                *((long long *)(v22 + iter1 * 8)) = *((long long *)(v6 + (iter1 + v8 - *((long long *)&g_42d1d0) & v18) * 8));
            }
            for (iter1 = 0; iter1 < v20; iter1 += 1)
            {
                *((long long *)(v6 + iter1 * 8)) = *((long long *)(v22 + iter1 * 8));
            }
        }
        for (iter1 = 0; iter1 < v20; iter1 += 1)
        {
            *((unsigned long *)(v22 + iter1 * 8)) = v12 + *((long long *)(v6 + iter1 * 8)) - v11;
        }
        idx->field_b8 = v20 * 8 + v6;
        idx->field_1e8 = v22 + v20 * 8;
        idx1 = &idx->padding_121[15];
        idx1->field_c0 = -(v20);
        idx->field_c0 = idx1->field_c0;
        idx->field_d8 = v9 - v20;
        idx->field_208 = v21 - v20;
        idx2 = &idx->padding_121[15];
        idx2->field_e8 = v8;
        idx->field_e8 = idx2->field_e8;
        return idx;
    }
    sub_41cf09(); /* do not return */
}



// Function: read_files @ 0xfa13
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[156];
    unsigned long long field_a0;
    unsigned long long field_a8;
    unsigned long long field_b0;
    char padding_b8[32];
    unsigned long long field_d8;
    char padding_e0[72];
    unsigned long long field_128;
    char padding_130[160];
    unsigned long long field_1d0;
    unsigned long long field_1d8;
    unsigned long long field_1e0;
    char padding_1e8[32];
    unsigned long long field_208;
    char padding_210[72];
    unsigned long long field_258;
} struct_0;

extern char g_425240;
extern char g_42d1d8;
extern unsigned long long g_42d758;
extern unsigned long long g_42d760;
extern void* g_42d768;
extern unsigned long long g_42d770;
extern void g_42d778;

unsigned long long read_files(struct_0 *idx, char a1)
{
    struct_0 *index;  // rax
    char v0;  // [bp-0xe]
    char v1;  // [bp-0xd]
    int iter;  // [bp-0xc], Other Possible Types: unsigned int

    v1 = g_42d1d8 || a1;
    v0 = (char)sub_40df00(idx, v1) || a1;
    if (idx->field_0 != *((int *)&idx->padding_130[0]))
    {
        v0 = (char)sub_40df00(idx->padding_130, v1 | v0) || v0;
    }
    else
    {
        idx->field_1d0 = idx->field_a0;
        idx->field_1d8 = idx->field_a8;
        idx->field_1e0 = idx->field_b0;
    }
    if (v0)
    {
        sub_4131e6(idx->field_0, 0);
        sub_4131e6(*((int *)&idx->padding_130[0]), 0);
        return 1;
    }
    sub_40ef6a(idx);
    *((unsigned long long *)&g_42d778) = idx->field_208 + idx->field_d8 + 1;
    if (*((long long *)&g_42d778) > 0x3fffffffffffffe)
        sub_41cf09(); /* do not return */
    g_42d768 = sub_41c945(*((long long *)&g_42d778) * 32);
    g_42d770 = 1;
    for (iter = 9; 1 << ((char)iter & 63) < (*((long long *)&g_42d778) * 6148914691236517206 >> 64) - (*((long long *)&g_42d778) >> 63); iter += 1);
    g_42d760 = (1 << ((char)iter & 63)) - *(&(&g_425240)[iter]);
    if (g_42d760 > 0xffffffffffffffe)
        sub_41cf09(); /* do not return */
    g_42d758 = sub_413030((g_42d760 + 1) * 8);
    g_42d758 = g_42d758 + 8;
    for (iter = 0; iter <= 1; iter += 1)
    {
        sub_40e2f8(&(&idx->field_0)[76 * iter]);
    }
    index = idx->padding_130;
    index->field_128 = g_42d770;
    idx->field_128 = index->field_128;
    free(g_42d768);
    free(g_42d758 - 8);
    return 0;
}



// Function: print_normal_script @ 0xfcfd
long long print_normal_script(long long a0)
{
    return sub_41245b(a0, sub_412437, sub_40fd2d, sub_412437);
}



// Function: print_normal_hunk @ 0xfd2d
typedef struct FILE {
} FILE;

extern char g_425780;
extern long long g_42d360;
extern unsigned long long g_42d418;
extern long long g_42d490;
extern unsigned long long g_42d548;
extern FILE *g_42d5c0;

void print_normal_hunk(long long a0)
{
    unsigned int v0;  // [bp-0x44]
    char i;  // [bp-0x40], Other Possible Types: long long
    char v2;  // [bp-0x38]
    char j;  // [bp-0x30], Other Possible Types: long long
    char v4;  // [bp-0x28]

    v0 = sub_412af4(a0, &i, &v2, &j, &v4);
    if (!v0)
        return;
    sub_411b50();
    sub_4128c2(4);
    sub_412a37(44, &g_42d360, i, v2);
    fputc_unlocked(*(&(&g_425780)[v0]), g_42d5c0);
    sub_412a37(44, &g_42d490, j, v4);
    sub_4128c2(3);
    fputc_unlocked(10, g_42d5c0);
    if ((char)v0 & 1)
    {
        for (; i <= v2; i += 1)
        {
            sub_4128c2(2);
            sub_4124fe("<", g_42d418 + i * 8, 1);
            sub_4128c2(3);
            if (*((char *)(*((long long *)((i + 1) * 8 + g_42d418)) - 1)) == 10)
                putc_unlocked(10, g_42d5c0);
        }
    }
    if (v0 == 3)
        fputs_unlocked("---\n", g_42d5c0);
    if (!((char)v0 & 2))
        return;
    for (; j <= v4; j += 1)
    {
        sub_4128c2(1);
        sub_4124fe(">", g_42d548 + j * 8, 1);
        sub_4128c2(3);
        if (*((char *)(*((long long *)((j + 1) * 8 + g_42d548)) - 1)) == 10)
            putc_unlocked(10, g_42d5c0);
    }
    return;
}



// Function: print_sdiff_script @ 0xff70
extern long long g_42d430;
extern unsigned long long g_42d448;
extern long long g_42d560;
extern unsigned long long g_42d780;
extern unsigned long long g_42d788;

long long print_sdiff_script(long long a0)
{
    sub_411b50();
    g_42d788 = -(g_42d448);
    g_42d780 = g_42d788;
    sub_41245b(a0, sub_412437, sub_4106e7, sub_412437);
    return sub_41055a(g_42d430, g_42d560, g_42d560);
}



// Function: tab_from_to @ 0xffdd
extern char g_42d2a1;
extern unsigned long long g_42d2a8;
extern unsigned long long g_42d5c0;

unsigned long long tab_from_to(unsigned long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x30]
    unsigned long long i;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    v0 = a0;
    v3 = g_42d5c0;
    v4 = g_42d2a8;
    if (g_42d2a1 != 1)
    {
        for (i = v0 + v4 - v0 % v4; i <= a1; i += v4)
        {
            putc_unlocked(9, v3);
            v0 = i;
        }
    }
    while (1)
    {
        v1 = v0 + 1;
        if (a1 <= v0)
            break;
        putc_unlocked(32, v3);
        v0 = v1;
    }
    return a1;
}



// Function: print_half_line @ 0x10090
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_42d2a1;
extern unsigned long long g_42d2a8;
extern unsigned long long g_42d5c0;

unsigned long long print_half_line(struct_0 **a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long node;  // rbx
    unsigned long long iter;  // r12
    char *v11;  // r14
    char *v12;  // rax
    char v13;  // r13b
    unsigned int v14;  // eax
    unsigned long long v15;  // rax
    unsigned int v0;  // [bp-0x78]
    int v1;  // [bp-0x74]
    unsigned long long v2;  // [bp-0x70]
    unsigned long v3;  // [bp-0x68]
    char *v4;  // [bp-0x60]
    unsigned long long v5;  // [bp-0x58]
    unsigned long count;  // [bp-0x50]
    unsigned long v7;  // [bp-0x48]

    v3 = g_42d5c0;
    node = 0;
    iter = 0;
    v11 = &*(a0)->field_0;
    v7 = 0;
    while (1)
    {
        do
        {
            while (1)
            {
                if (v11 >= a0[1])
                    return iter;
                v4 = v11;
                v12 = v11;
                v11 = v12 + 1;
                v13 = *(v12);
                v14 = v13;
                if (v14 <= 95)
                {
                    if (v14 < 8)
                        goto LABEL_410288;
                    switch (v14)
                    {
                    case 8:
                        if (!node)
                            break;
                    case 9:
                        v5 = g_42d2a8 - node % g_42d2a8;
                        if (node == iter)
                        {
                            v2 = v5 + iter;
                            if (g_42d2a1)
                            {
                                if (a2 < v2)
                                    v2 = a2;
                                for (; iter < v2; iter += 1)
                                {
                                    putc_unlocked(32, v3);
                                }
                            }
                            else if (v2 < a2)
                            {
                                putc_unlocked(v13, v3);
                                iter = v2;
                            }
                        }
                        node += v5;
                        continue;
                    case 10:
                        return iter;
                    case 11: case 12:
                        if (node < a2)
                        {
                            putc_unlocked(v13, v3);
                            break;
                        }
                        break;
                    case 13:
                        putc_unlocked(v13, v3);
                        sub_40ffdd(0, a1);
                        iter = 0;
                        node = 0;
                        continue;
                    case 32: case 33: case 34: case 35: case 37: case 38: case 39: case 40: case 41: case 42: case 43: case 44: case 45: case 46: case 47: case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57: case 58: case 59: case 60: case 61: case 62: case 63: case 65: case 66: case 67: case 68: case 69: case 70: case 71: case 72: case 73: case 74: case 75: case 76: case 77: case 78: case 79: case 80: case 81: case 82: case 83: case 84: case 85: case 86: case 87: case 88: case 89: case 90: case 91: case 92: case 93: case 94: case 95:
LABEL_41032d:
                        v15 = node;
                        node = v15 + 1;
                        if (a2 > v15)
                        {
                            putc_unlocked(v13, v3);
                            iter = node;
                            break;
                        }
                        break;
                    default:
LABEL_410288:
                        count = sub_41d369(&v0, v4, a0[1] - v4, &v7);
                        if (count && count <= 0xfffffffffffffffd)
                        {
                            v1 = wcwidth(v0);
                            if (v1 > 0)
                                node += v1;
                            if (node <= a2)
                            {
                                fwrite_unlocked(v4, 1, count, stdout);
                                iter = node;
                            }
                            v11 = &v4[count];
                            continue;
                        }
                    }
                }
                else
                {
                    if (v14 - 97 <= 29)
                        goto LABEL_41032d;
                    else
                        goto LABEL_410288;
                }
            }
        } while ((node -= 1, node >= a2));
        if (iter <= node)
        {
            for (; iter < node; iter += 1)
            {
                putc_unlocked(32, v3);
            }
        }
        else
        {
            putc_unlocked(v13, v3);
            iter = node;
        }
    }
}



// Function: print_1sdiff_line @ 0x10388
typedef struct struct_0 {
    char padding_0[8];
    void* field_8;
} struct_0;

typedef struct struct_1 {
    struct struct_2 *field_0;
    void* field_8;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern unsigned long long g_42d320;
extern unsigned long long g_42d328;
extern unsigned long long g_42d5c0;

unsigned long long print_1sdiff_line(struct_0 *a0, char a1, struct_1 *a2)
{
    unsigned long long v8;  // rax
    char v0;  // [bp-0x44]
    char v1;  // [bp-0x2a]
    char v2;  // [bp-0x29]
    unsigned long long v3;  // [bp-0x28]
    unsigned long v4;  // [bp-0x20]
    unsigned long v5;  // [bp-0x18]
    unsigned long long v6;  // [bp-0x10]

    v0 = a1;
    v4 = g_42d5c0;
    v5 = g_42d320;
    v8 = g_42d328;
    v6 = g_42d328;
    v3 = 0;
    v1 = 0;
    v2 = 0;
    switch (v0)
    {
    case 60:
        v8 = sub_4128c2(2);
        v2 = 1;
        break;
    case 62:
        v8 = sub_4128c2(1);
        v2 = 1;
        break;
    default:
        goto LABEL_4103fc;
    }
LABEL_4103fc:
    if (a0)
    {
        v1 = *((char *)a0->field_8 - 1) == 10 || v1;
        v8 = sub_410090(a0, 0, v5);
        v3 = v8;
    }
    if (v0 != 32)
    {
        v3 = sub_40ffdd(v3, v6 + v5 - 1 >> 1) + 1;
        if (v0 == 124 && v1 != *((char *)a2->field_8 - 1) == 10)
            v0 = (!v1 ? 92 : 47);
        v8 = putc_unlocked(v0, v4);
    }
    if (a2)
    {
        v1 = *((char *)a2->field_8 - 1) == 10 || v1;
        v8 = a2->field_0->field_0;
        if ((char)v8 != 10)
        {
            v3 = sub_40ffdd(v3, v6);
            v8 = sub_410090(a2, v3, v5);
        }
    }
    if (v1)
        v8 = putc_unlocked(10, v4);
    if (v2)
        v8 = sub_4128c2(3);
    return v8;
}



// Function: print_sdiff_common_lines @ 0x1055a
typedef struct FILE {
} FILE;

extern char g_42d318;
extern char g_42d319;
extern char g_42d31a;
extern unsigned long long g_42d418;
extern unsigned long long g_42d548;
extern FILE *g_42d5c0;
extern unsigned long long g_42d780;
extern unsigned long long g_42d788;

long long print_sdiff_common_lines(unsigned long a0, unsigned long a1)
{
    unsigned long iter;  // [bp-0x28]
    unsigned long node;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    iter = g_42d780;
    node = g_42d788;
    if (g_42d31a != 1 && (iter != a0 || node != a1))
    {
        if (g_42d318)
        {
            v2 = a0 - iter;
            v3 = a1 - node;
            fprintf(g_42d5c0, "i%ld,%ld\n", v2, v3);
        }
        iter = iter;
        if (g_42d319 != 1)
        {
            while (1)
            {
                if (iter == a0 || node == a1)
                    break;
                node += 1;
                iter += 1;
                sub_410388(iter * 8 + g_42d418, 32, g_42d548 + node * 8);
            }
            while (1)
            {
                if (node == a1)
                    break;
                node += 1;
                sub_410388(NULL, 41, node * 8 + g_42d548);
            }
        }
        while (1)
        {
            if (iter == a0)
                break;
            iter += 1;
            sub_410388(iter * 8 + g_42d418, 40, NULL);
        }
    }
    g_42d780 = a0;
    g_42d788 = a1;
    return a1;
}



// Function: print_sdiff_hunk @ 0x106e7
typedef struct FILE {
} FILE;

extern char g_42d318;
extern unsigned long long g_42d418;
extern unsigned long long g_42d548;
extern FILE *g_42d5c0;
extern unsigned long long g_42d780;
extern unsigned long long g_42d788;

void print_sdiff_hunk(long long a0)
{
    unsigned int v0;  // [bp-0x54]
    char iter;  // [bp-0x50], Other Possible Types: long long
    long long v2;  // [bp-0x48]
    char node;  // [bp-0x40], Other Possible Types: long long
    char v4;  // [bp-0x38], Other Possible Types: long long
    unsigned long v5;  // [bp-0x30]
    unsigned long v6;  // [bp-0x28]

    v0 = sub_412af4(a0, &iter, &v2, &node, &v4);
    if (!v0)
        return;
    sub_41055a(iter, node);
    if (g_42d318)
    {
        v5 = v2 - iter + 1;
        v6 = v4 - node + 1;
        fprintf(g_42d5c0, "c%ld,%ld\n", v5, v6);
    }
    if (v0 == 3)
    {
        for (; iter <= v2 && node <= v4; node += 1)
        {
            sub_410388(g_42d418 + iter * 8, 124, node * 8 + g_42d548);
            iter += 1;
        }
        v0 = (node <= v4 ? 2 : 0) + (iter <= v2);
        g_42d780 = iter;
        g_42d788 = node;
    }
    if ((char)v0 & 2)
    {
        for (; node <= v4; node += 1)
        {
            sub_410388(NULL, 62, g_42d548 + node * 8);
        }
        g_42d788 = node;
    }
    if (!((char)v0 & 1))
        return;
    for (; iter <= v2; iter += 1)
    {
        sub_410388(g_42d418 + iter * 8, 60, NULL);
    }
    g_42d780 = iter;
    return;
}



// Function: perror_with_name @ 0x108ec
void perror_with_name(unsigned long long a0)
{
    int *err;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    err = __errno_location();
    error(0, *(err), "%s");
    return;
}



// Function: pfatal_with_name @ 0x10925
void pfatal_with_name(unsigned long long a0)
{
    long long v3;  // rdi
    long long v4;  // rsi
    long long v5;  // rdx
    long long v6;  // rcx
    long long v7;  // r8
    long long v8;  // r9
    unsigned long long v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0xc]

    v0 = a0;
    v1 = *(__errno_location());
    sub_410bca(v3, v4, v5, v6, v7, v8);
    error(2, v1, "%s");
}



// Function: fatal @ 0x10966
void fatal(char *a0)
{
    sub_410bca();
    gettext(a0);
    error(2, 0, "%s");
}



// Function: message @ 0x109a8
long long message(long long a0, long long a1, long long a2)
{
    return sub_4109e2(a0, a1, a2, 0, 0);
}



// Function: message5 @ 0x109e2
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_42540f;
extern struct_0 *g_42d020;
extern char g_42d2c0;
extern char g_42d318;

void message5(unsigned long a0, unsigned long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4)
{
    int v13;  // eax
    unsigned long long v0;  // [bp-0xb0]
    unsigned long long v1;  // [bp-0xa8]
    int i;  // [bp-0x84]
    unsigned long iter;  // [bp-0x80]
    long long v4;  // [bp-0x78]
    unsigned long v5;  // [bp-0x70]
    unsigned long v6;  // [bp-0x68]
    unsigned long long v7;  // [bp-0x60]
    unsigned long long v8;  // [bp-0x58]
    unsigned long long v9;  // [bp-0x50]
    unsigned long long v10;  // [bp-0x48]
    char v11;  // [bp-0x38]

    v1 = a3;
    v0 = a4;
    if (g_42d2c0)
    {
        v4 = 8;
        v6 = a0;
        v7 = a1;
        v8 = a2;
        if (!v1)
            v1 = &g_42540f;
        v9 = v1;
        if (!v0)
            v0 = &g_42540f;
        v10 = v0;
        for (i = 0; i <= 4; i += 1)
        {
            *((unsigned long long *)&(&v11)[8 * i]) = strlen((&v6)[i]) + 1;
            v4 += *((long long *)&(&v11)[8 * i]);
        }
        v5 = sub_41c945(v4);
        i = 0;
        for (iter = v5 + 8; i <= 4; iter += *((long long *)&(&v11)[8 * v13]))
        {
            memcpy(iter, (&v6)[i], *((long long *)&(&v11)[8 * i]));
            v13 = i;
            i = v13 + 1;
        }
        *((unsigned long *)&g_42d020) = v5;
        *((unsigned long long *)v5) = 0;
        g_42d020 = v5;
        return;
    }
    else
    {
        if (g_42d318)
            putchar_unlocked(32);
        printf(gettext(a0));
        return;
    }
}



// Function: print_message_queue @ 0x10bca
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_42d7a0;

void print_message_queue(void)
{
    unsigned long len;  // rax
    int j;  // [bp-0x6c]
    struct_0 **ptr;  // [bp-0x68]
    struct_0 **v2;  // [bp-0x60]
    unsigned long v3;  // [bp-0x58]

    for (ptr = g_42d7a0; ptr; ptr = v2)
    {
        v2 = *(ptr);
        v3 = ptr + 1;
        for (j = 0; j <= 3; j += 1)
        {
            len = strlen((&v3)[j]);
            (&v3)[1 + j] = len + 1 + (&v3)[j];
        }
        printf(gettext(v3));
        free(ptr);
    }
    return;
}



// Function: sighandler @ 0x10ccc
extern unsigned int g_42d840;

long long sighandler(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_42d840;
    if (!(unsigned int)v1)
    {
        v1 = a0;
        g_42d840 = v1;
    }
    return v1;
}



// Function: stophandler @ 0x10ced
extern unsigned int g_42d840;
extern unsigned int g_42d844;

long long stophandler(unsigned int a0)
{
    unsigned long v2;  // rax
    unsigned int v0;  // [bp-0xc]

    v0 = a0;
    v2 = g_42d840;
    if (!g_42d840)
    {
        v2 = g_42d844 + 1;
        g_42d844 = v2;
    }
    return v2;
}



// Function: process_signals @ 0x10d14
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern int g_42d7c0;
extern unsigned int g_42d840;
extern unsigned int g_42d844;

void process_signals(void)
{
    unsigned int v0;  // [bp-0xa0]
    unsigned int v1;  // [bp-0x9c]
    int v2[34];  // [bp-0x98]

    while (g_42d840 || g_42d844)
    {
        sub_4128c2(3);
        fflush_unlocked(stdout);
        sigprocmask(0, &g_42d7c0, v2);
        v0 = g_42d840;
        v1 = g_42d844;
        if (v1)
        {
            g_42d844 = v1 - 1;
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



// Function: install_signal_handlers @ 0x10e15
typedef struct sigaction {
} sigaction;

extern unsigned int g_4257c0[4];
extern unsigned long long g_42d7c0;
extern unsigned long long g_42d7c8;
extern unsigned long long g_42d7d0;
extern unsigned long long g_42d7d8;
extern unsigned long long g_42d7e0;
extern unsigned long long g_42d7e8;
extern unsigned long long g_42d7f0;
extern unsigned long long g_42d7f8;
extern unsigned long long g_42d800;
extern unsigned long long g_42d808;
extern unsigned long long g_42d810;
extern unsigned long long g_42d818;
extern unsigned long long g_42d820;
extern unsigned long long g_42d828;
extern unsigned long long g_42d830;
extern unsigned long long g_42d838;

void install_signal_handlers(void)
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

    sigemptyset(&g_42d7c0);
    for (iter = 0; iter <= 11; iter += 1)
    {
        sigaction(g_4257c0[iter], NULL, &v1);
        if (v1 != 1)
            sigaddset(&g_42d7c0, g_4257c0[iter]);
    }
    v2 = g_42d7c0;
    v3 = g_42d7c8;
    v4 = g_42d7d0;
    v5 = g_42d7d8;
    v6 = g_42d7e0;
    v7 = g_42d7e8;
    v8 = g_42d7f0;
    v9 = g_42d7f8;
    v10 = g_42d800;
    v11 = g_42d808;
    v12 = g_42d810;
    v13 = g_42d818;
    v14 = g_42d820;
    v15 = g_42d828;
    v16 = g_42d830;
    v17 = g_42d838;
    v18 = 0x10000000;
    for (iter = 0; iter <= 11; iter += 1)
    {
        if (sigismember(&g_42d7c0, g_4257c0[iter]))
        {
            v1 = (sigaction)(g_4257c0[iter] == 0x14 ? sub_410ced : sub_410ccc);
            sigaction(g_4257c0[iter], &v1, NULL);
        }
    }
    return;
}



// Function: get_funky_string @ 0x11052

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
                    goto LABEL_41121b;
                if (v9 > 120)
                    goto LABEL_41121b;
                if (v9 < 48)
                    goto LABEL_41121b;
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
LABEL_41121b:
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



// Function: set_color_palette @ 0x1140f
extern unsigned long long g_42d868;

long long set_color_palette(unsigned long a0)
{
    g_42d868 = a0;
    return a0;
}



// Function: parse_diff_color @ 0x11429
typedef struct struct_1 {
    char padding_0[32];
    unsigned long long field_20;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_42c920[4];
extern char g_42d040;
extern unsigned long long g_42d048;
extern char g_42d859;
extern struct_1 *g_42d860;
extern struct_0 *g_42d868;

void parse_diff_color(void)
{
    unsigned int v11;  // eax
    char *v13;  // rax
    char *v14;  // rax
    char *v15;  // rax
    char *v18;  // rax
    int v0;  // [bp-0x50]
    unsigned int v1;  // [bp-0x4c]
    char *node;  // [bp-0x48]
    void* v3;  // [bp-0x40]
    unsigned long long *idx;  // [bp-0x38]
    struct_1 *iter;  // [bp-0x30]
    void* v6;  // [bp-0x28]
    struct_1 *ptr;  // [bp-0x20]
    char v8;  // [bp-0x13]
    char v9;  // [bp-0x12]

    node = &g_42d868->field_0;
    if (!node || !*(node))
        return;
    idx = NULL;
    strcpy(&v8, "??");
    v6 = sub_41ced8(node);
    v3 = v6;
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
                    goto LABEL_41175d;
                }
                if (v11 != 42)
                    goto LABEL_41158e;
                idx = sub_41c945(40);
                idx[4] = g_42d860;
                g_42d860 = idx;
                node += 1;
                idx[1] = v3;
                v1 = (!(char)sub_411052(&v3, &node, 1, idx) ? 6 : 4);
            }
            else
            {
LABEL_41158e:
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
                for (v0 = 0; g_42c920[v0]; v0 += 1)
                {
                    if (!strcmp(&v8, g_42c920[v0]))
                    {
                        (&g_42d048)[2 * v0] = v3;
                        v1 = (!(char)sub_411052(&v3, &node, 0, &(&g_42d040)[16 * v0]) ? 6 : 1);
                        break;
                    }
                }
                if (v1 == 6)
                    error(0, 0, gettext("unrecognized prefix: %s"));
            }
            break;
        case 4:
            v18 = node;
            node = v18 + 1;
            if (*(v18) == 61)
            {
                idx[3] = v3;
                v1 = (!(char)sub_411052(&v3, &node, 0, idx + 2) ? 6 : 1);
                break;
            }
            else
            {
                v1 = 6;
                break;
            }
        case 6:
            goto LABEL_41175d;
        default:
            abort(); /* do not return */
        }
    }
LABEL_41175d:
    if (v1 != 6)
        return;
    error(0, 0, gettext("unparsable value for --palette"));
    free(v6);
    iter = g_42d860;
    while (iter)
    {
        ptr = iter;
        iter = iter->field_20;
        free(ptr);
    }
    g_42d859 = 0;
    return;
    return;
}



// Function: check_color_output @ 0x117e9
typedef struct FILE {
} FILE;

extern unsigned int g_42d1c4;
extern FILE *g_42d5c0;
extern char g_42d5c8;
extern char g_42d859;

long long check_color_output(char a0)
{
    FILE *v2;  // rax
    char v3;  // al
    char v4;  // al
    unsigned long v5;  // rax
    char v0;  // [bp-0x9]

    v2 = g_42d5c0;
    if (g_42d5c0)
    {
        v2 = g_42d1c4;
        if (g_42d1c4)
        {
            if (g_42d5c8 || a0 != 1 && isatty(fileno(g_42d5c0)))
                v3 = 1;
            else
                v3 = 0;
            v0 = v3;
            v0 &= 1;
            if (g_42d1c4 == 2 || g_42d1c4 == 1 && v0)
                v4 = 1;
            else
                v4 = 0;
            g_42d859 = v4 & 1;
            v5 = g_42d859;
            if (g_42d859)
                v5 = (unsigned long long)sub_411429();
            if (!v0)
                return v5;
            return (unsigned long long)sub_410e15();
        }
    }
    return v2;
}



// Function: setup_output @ 0x118ac
extern unsigned long long g_42d5c0;
extern unsigned long long g_42d848;
extern unsigned long long g_42d850;
extern char g_42d858;

int setup_output(unsigned long a0, unsigned long a1, char a2)
{
    g_42d848 = a0;
    g_42d850 = a1;
    g_42d858 = a2;
    g_42d5c0 = 0;
    return a2;
}



// Function: c_escape_char @ 0x118ef

unsigned int c_escape_char(char a0)
{
    unsigned long v1;  // rax

    v1 = a0;
    if ((unsigned int)v1 <= 0x22)
    {
        if ((unsigned int)v1 >= 7)
        {
            v1 = (unsigned int)v1 - 7;
            switch ((unsigned int)v1)
            {
            case 0:
                return 97;
            case 1:
                return 98;
            case 2:
                return 116;
            case 3:
                return 110;
            case 4:
                return 118;
            case 5:
                return 0x66;
            case 6:
                return 114;
            case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15: case 16: case 17: case 18: case 19: case 20: case 21: case 22: case 23: case 24: case 25: case 26:
                return _INSERT(v1, 0, a0 <= 31);
            case 27:
                return 0x22;
            default:
                return _INSERT(v1, 0, a0 <= 31);
            }
        }
        else
        {
            return _INSERT(v1, 0, a0 <= 31);
        }
    }
    else
    {
        if ((unsigned int)v1 == 92)
            return 92;
    }
}



// Function: c_escape @ 0x11984
char * c_escape(char *a0)
{
    unsigned int v10;  // eax
    unsigned int v11;  // eax
    char *v12;  // rax
    char *v13;  // rax
    char *v14;  // rax
    char *v15;  // rax
    char v0;  // [bp-0x34]
    char v1;  // [bp-0x33]
    char v2;  // [bp-0x32]
    char v3;  // [bp-0x31]
    char *node;  // [bp-0x30]
    unsigned long long iter;  // [bp-0x28]
    char *iter1;  // [bp-0x20]
    char *v7;  // [bp-0x18]
    char *v8;  // [bp-0x10]

    iter = 0;
    v0 = 0;
    for (node = a0; *(node); node += 1)
    {
        v3 = *(node);
        if (v3 == 32)
        {
            v0 = 1;
            goto LABEL_4119ec;
        }
        v10 = (char)sub_4118ef(*(node));
        if (v10)
        {
            if (v10 == 1)
            {
                iter += 3;
            }
            else
            {
                iter += 1;
                goto LABEL_4119ec;
            }
        }
LABEL_4119ec:
    }
    if (!v0 && !iter)
        return a0;
    v7 = node - a0;
    v8 = sub_41c945(&v7[iter + 3]);
    iter1 = v8;
    iter1 += 1;
    *(iter1) = 0x22;
    for (node = a0; *(node); node += 1)
    {
        v1 = *(node);
        v2 = sub_4118ef(v1);
        v11 = v2;
        if (!v11)
        {
            v14 = iter1;
            iter1 = v14 + 1;
            *(v14) = v1;
        }
        else if (v11 == 1)
        {
            v12 = iter1;
            iter1 = v12 + 1;
            *(v12) = 92;
            iter1 += 1;
            *(iter1) = (v1 >> 6) + 48;
            iter1 += 1;
            *(iter1) = ((char)(v1) >> 3 & 7) + 48;
            iter1 += 1;
            *(iter1) = (v1 & 7) + 48;
        }
        else
        {
            v13 = iter1;
            iter1 = v13 + 1;
            *(v13) = 92;
            iter1 += 1;
            *(iter1) = v2;
        }
    }
    v15 = iter1;
    iter1 = v15 + 1;
    *(v15) = 0x22;
    *(iter1) = 0;
    return v8;
}



// Function: begin_output @ 0x11b50
typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_425400;
extern unsigned long long stdout;
extern unsigned int g_42d1c0;
extern char g_42d2c0;
extern long long g_42d330;
extern struct_0 g_42d360;
extern unsigned long long g_42d5c0;
extern void* g_42d848;
extern void g_42d850;
extern char g_42d858;
extern int g_42d870;

void begin_output(void)
{
    void* v0;  // [bp-0x58]
    int v1;  // [bp-0x50]
    int v2;  // [bp-0x4c]
    void* v3;  // [bp-0x48]
    unsigned long v4;  // [bp-0x40]
    unsigned long v5;  // [bp-0x38]
    unsigned long long v6;  // [bp-0x30]
    void* v7;  // [bp-0x28]
    unsigned long long v8;  // [bp-0x20]

    if (g_42d5c0)
        return;
    v3 = sub_411984(g_42d848);
    v4 = sub_411984(*((long long *)&g_42d850));
    v0 = sub_41d0f7("diff%s %s %s", g_42d330, v3, v4);
    if (g_42d2c0)
    {
        if (fflush_unlocked(stdout))
            sub_410925(gettext("write failed")); /* do not return */
        v5 = &g_425400;
        v6 = "-h";
        v7 = v0;
        v8 = 0;
        if (pipe(&v1))
            sub_410925("pipe"); /* do not return */
        g_42d870 = fork();
        if (g_42d870 < 0)
            sub_410925("fork"); /* do not return */
        if (!g_42d870)
        {
            close(v2);
            if (v1)
            {
                if (dup2(v1, 0) < 0)
                    sub_410925("dup2"); /* do not return */
                close(v1);
            }
            execv(&g_425400, &v5);
            _exit((*(__errno_location()) == 2 ? 127 : 126)); /* do not return */
        }
        close(v1);
        g_42d5c0 = fdopen(v2, "w");
        if (!g_42d5c0)
            sub_410925("fdopen"); /* do not return */
        sub_4117e9(1);
    }
    else
    {
        g_42d5c0 = stdout;
        sub_4117e9(0);
        if (g_42d858)
            printf("%s\n", v0);
    }
    free(v0);
    if (g_42d1c0 == 2)
    {
        sub_407e5c(&g_42d360.field_0, &v3, 0);
    }
    else if (g_42d1c0 == 3)
    {
        sub_407e5c(&g_42d360.field_0, &v3, 1);
    }
    if (v3 != g_42d848)
        free(v3);
    if (v4 == *((long long *)&g_42d850))
        return;
    free(v4);
    return;
}



// Function: finish_output @ 0x11e1b
typedef struct FILE {
} FILE;

extern char stdout;
extern FILE *g_42d5c0;
extern int g_42d870;

void finish_output(void)
{
    unsigned int v5;  // eax
    int v0;  // [bp-0x1c]
    int v1;  // [bp-0x18]
    unsigned int v2;  // [bp-0x14]

    if (g_42d5c0 && g_42d5c0 != *((long long *)&stdout))
    {
        v1 = 0;
        if (ferror_unlocked(g_42d5c0))
        {
            sub_410966("write failed"); /* do not return */
        }
        else if (fclose(g_42d5c0))
        {
            sub_410925(gettext("write failed")); /* do not return */
        }
        else if (waitpid(g_42d870, &v0, 0) < 0)
        {
            sub_410925("waitpid"); /* do not return */
        }
        else
        {
            if (!v1 && !((char)v0 & 127))
                v5 = v0 >> 8 & 0xff;
            else
                v5 = 0x7fffffff;
            v2 = v5;
            if (v2)
                error(2, v1, gettext((v2 == 126 ? "subsidiary program '%s' could not be invoked" : (v2 == 127 ? "subsidiary program '%s' not found" : (v2 == 0x7fffffff ? "subsidiary program '%s' failed" : "subsidiary program '%s' failed (exit status %d)")))));
        }
    }
    g_42d5c0 = 0;
    return;
}



// Function: lines_differ @ 0x11f90
extern unsigned int g_42d1e8;
extern char g_42d1ee;
extern unsigned long long g_42d2a8;

unsigned int lines_differ(void* a0, void* a1)
{
    void* iter;  // r14
    void* iter1;  // r13
    unsigned long long *v18;  // rax
    void* v19;  // rax
    unsigned long long *v20;  // rax
    unsigned long long *v21;  // rax
    void* v22;  // rax
    unsigned long long *v23;  // rax
    unsigned long long *v24;  // rax
    unsigned long long *v25;  // rax
    unsigned long long *v26;  // rax
    unsigned long long *v27;  // rax
    void* v10;  // rax
    unsigned long long *v28;  // rax
    unsigned long long *v29;  // rax
    void* v30;  // rax
    void* v31;  // rax
    char v11;  // bl
    void* v12;  // rax
    char v13;  // r12b
    unsigned long long *v14;  // rax
    void* v15;  // rax
    unsigned long long *v16;  // rax
    void* v17;  // rax
    void* v0;  // [bp-0x68]
    void* v1;  // [bp-0x60]
    char v2;  // [bp-0x49]
    unsigned long long iter2;  // [bp-0x48]
    void* node;  // [bp-0x40]
    void* v5;  // [bp-0x38]
    unsigned long long v6;  // [bp-0x30]

    v1 = a0;
    v0 = a1;
    iter2 = 0;
    iter = v0;
    iter1 = v1;
    while (1)
    {
        v10 = iter1;
        iter1 = v10 + 1;
        v11 = *((char *)v10);
        v12 = iter;
        iter = v12 + 1;
        v13 = *((char *)v12);
        if (v11 != v13)
        {
            switch (g_42d1e8)
            {
            case 0:
                goto LABEL_4123b9;
            case 1:
LABEL_4122e2:
                if (v11 == 32 && v13 == 9 || v11 == 9 && v13 == 32)
                {
                    v6 = iter2;
                    while (1)
                    {
                        switch (v11)
                        {
                        case 32:
                            iter2 += 1;
                            break;
                        case 9:
                            iter2 += g_42d2a8 - iter2 % g_42d2a8;
                            break;
                        default:
                            while (1)
                            {
                                switch (v13)
                                {
                                case 32:
                                    v6 += 1;
                                    break;
                                case 9:
                                    v6 += g_42d2a8 - v6 % g_42d2a8;
                                    break;
                                default:
                                    if (iter2 != v6)
                                        return 1;
                                    goto LABEL_4123b8;
                                }
                                v31 = iter;
                                iter = v31 + 1;
                                v13 = *((char *)v31);
                            }
                        }
                        v30 = iter1;
                        iter1 = v30 + 1;
                        v11 = *((char *)v30);
                    }
                }
LABEL_4123b8:
                goto LABEL_4123b9;
            case 2: case 3:
                v26 = __ctype_b_loc();
                if (!(*((short *)(v11 * 2 + *(v26))) & 0x2000 && (v27 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)v13 * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000)))
                {
                    if (g_42d1e8 != 2)
                        goto LABEL_4122e2;
                    goto LABEL_4123b9;
                }
                else if (v11 != 10)
                {
                    node = iter1;
                    while (1)
                    {
                        v2 = *((char *)node);
                        if (v2 == 10 || !(v28 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)v2 * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000))
                            break;
                        node += 1;
                    }
                    if (v2 == 10)
                        goto LABEL_41227d;
                    goto LABEL_4123b9;
                }
                else
                {
LABEL_41227d:
                    if (v13 == 10)
                        break;
                    v5 = iter;
                    while (1)
                    {
                        v2 = *((char *)v5);
                        if (v2 == 10 || !(v29 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)v2 * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000))
                            break;
                        v5 += 1;
                    }
                    if (v2 == 10)
                        break;
                    else
                        goto LABEL_4123b9;
                }
            case 4:
                v18 = __ctype_b_loc();
                if (*((short *)(v11 * 2 + *(v18))) & 0x2000)
                {
                    while (v11 != 10)
                    {
                        v19 = iter1;
                        iter1 = v19 + 1;
                        v11 = *((char *)v19);
                        v20 = __ctype_b_loc();
                        if (!(*((short *)(*((char *)v19) * 2 + *(v20))) & 0x2000))
                        {
                            iter1 -= 1;
                            v11 = 32;
                            break;
                        }
                    }
                }
                v21 = __ctype_b_loc();
                if (*((short *)(v13 * 2 + *(v21))) & 0x2000)
                {
                    while (v13 != 10)
                    {
                        v22 = iter;
                        iter = v22 + 1;
                        v13 = *((char *)v22);
                        v23 = __ctype_b_loc();
                        if (!(*((short *)(*((char *)v22) * 2 + *(v23))) & 0x2000))
                        {
                            iter -= 1;
                            v13 = 32;
                            break;
                        }
                    }
                }
                if (v11 != v13)
                {
                    if (v13 == 32 && v11 != 10 && iter1 > v1 + 1 && (v24 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((short *)((unsigned long long)(char)*((char *)((char *)iter1 - 2)) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000))
                    {
                        iter1 -= 1;
                        goto LABEL_41241f;
                    }
                    if (v11 == 32 && v13 != 10 && iter > v0 + 1 && !(v25 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), !(*((short *)((unsigned long long)(char)*((char *)((char *)iter - 2)) * 2 + *((long long *)(unsigned long long)__ctype_b_loc()))) & 0x2000)))
                    {
                        iter -= 1;
                        goto LABEL_41241f;
                    }
                }
                goto LABEL_4123b9;
            case 5:
                while (1)
                {
                    v14 = __ctype_b_loc();
                    if (!(*((short *)(v11 * 2 + *(v14))) & 0x2000) || v11 == 10)
                        break;
                    v15 = iter1;
                    iter1 = v15 + 1;
                    v11 = *((char *)v15);
                }
                while (1)
                {
                    v16 = __ctype_b_loc();
                    if (!(*((short *)(v13 * 2 + *(v16))) & 0x2000) || v13 == 10)
                        break;
                    v17 = iter;
                    iter = v17 + 1;
                    v13 = *((char *)v17);
                }
                goto LABEL_4123b9;
            default:
LABEL_4123b9:
                if (g_42d1ee)
                {
                    v11 = tolower(v11);
                    v13 = tolower(v13);
                }
                if (v11 != v13)
                    return 1;
                goto LABEL_4123e3;
            }
        }
        else
        {
LABEL_4123e3:
            if (v11 == 10)
                return 0;
            iter2 += (v11 == 9 ? g_42d2a8 - iter2 % g_42d2a8 : 1);
LABEL_41241f:
        }
    }
    return 0;
}



// Function: find_change @ 0x12437
long long find_change(unsigned long a0)
{
    return a0;
}



// Function: find_reverse_change @ 0x12449
long long find_reverse_change(unsigned long a0)
{
    return a0;
}



// Function: print_script @ 0x1245b
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

struct_0 ** print_script(struct_0 **a0, unsigned long long *a1, unsigned long long *a2)
{
    struct_0 **v4;  // rax
    struct_0 **i;  // [bp-0x20]
    struct_0 **v1;  // [bp-0x18]
    struct_0 **v2;  // [bp-0x10]

    for (i = a0; i; *(v2) = i)
    {
        v1 = i;
        v2 = a1(i);
        i = *(v2);
        *(v2) = NULL;
        a2(v1);
    }
    return v4;
}



// Function: print_1_line @ 0x124cf
long long print_1_line(long long a0, long long a1)
{
    return sub_4124fe(a0, a1, 0, a1);
}



// Function: print_1_line_nl @ 0x124fe
typedef struct struct_0 {
    struct struct_1 *field_0;
    void* field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern char g_42d2b0;
extern char g_42d2b1;
extern unsigned long long g_42d5c0;

int print_1_line_nl(char *a0, struct_0 *a1, char a2)
{
    int v7;  // eax
    int v8;  // rax
    int v9;  // rax
    unsigned long v0;  // [bp-0x38]
    unsigned long v1;  // [bp-0x30]
    char *v2;  // [bp-0x28], Other Possible Types: unsigned long
    char *v3;  // [bp-0x20]
    void* v4;  // [bp-0x18]
    unsigned long v5;  // [bp-0x10]

    v3 = &a1->field_0->field_0;
    v4 = a1->field_8;
    v5 = g_42d5c0;
    v0 = 0;
    if (a0 && *(a0))
    {
        v0 = (!g_42d2b0 ? "%s " : "%s\t");
        v1 = v0;
        v2 = a0;
        if (g_42d2b1 && a1->field_0->field_0 == 10)
        {
            v1 = "%s";
            v2 = &v2[*(v2) == 32];
        }
        fprintf(v5, v1);
    }
    if (a2 && *((char *)v4 - 1) == 10)
        v7 = 1;
    else
        v7 = 0;
    sub_41267d(v3, -(v7) + v4, v0, a0);
    if (a0)
    {
        v8 = *(a0);
        if (!(char)v8)
            return v8;
    }
    v9 = *((char *)v4 - 1);
    if ((char)v9 == 10)
        return v9;
    sub_4128c2(3);
    return fprintf(v5, "\n\\ %s\n", gettext("No newline at end of file"));
}



// Function: output_1_line @ 0x1267d
typedef struct FILE {
} FILE;

extern char g_42d2a1;
extern unsigned long long g_42d2a8;
extern FILE *g_42d5c0;

long long output_1_line(char *a0, char *a1, char *a2, unsigned long long a3)
{
    unsigned long i;  // rax
    unsigned long v11;  // rax
    unsigned long long iter;  // rbx
    unsigned long v13;  // rax
    char *v14;  // rax
    char v15;  // r14b
    unsigned long long *v16;  // rax
    unsigned long long v0;  // [bp-0x88]
    char *v1;  // [bp-0x70]
    unsigned long v2;  // [bp-0x60]
    unsigned long v3;  // [bp-0x58]
    unsigned long long j;  // [bp-0x50]
    unsigned long v5;  // [bp-0x48]
    unsigned long v6;  // [bp-0x40]
    unsigned long v7;  // [bp-0x38]
    unsigned long count;  // [bp-0x30]

    v1 = a0;
    v0 = a3;
    v5 = 0x400;
    if (g_42d2a1 != 1)
    {
        i = a1 - v1;
        for (v2 = i; v2; i = (unsigned long long)sub_410d14())
        {
            if (v2 <= v5)
                v11 = v2;
            else
                v11 = v5;
            v7 = v11;
            count = fwrite_unlocked(v1, 1, v7, g_42d5c0);
            if (count < v7)
                return count;
            v1 = &v1[count];
            v2 -= count;
        }
        return i;
    }
    iter = 0;
    v13 = g_42d2a8;
    v6 = g_42d2a8;
    v3 = 0;
    while (1)
    {
        while (1)
        {
            if (v1 >= a1)
                return v13;
            v3 += 1;
            if (v3 == v5)
            {
                sub_410d14();
                v3 = 0;
            }
            v14 = v1;
            v1 = v14 + 1;
            v15 = *(v14);
            v13 = v15;
            if ((unsigned int)v13 != 13)
                break;
            v13 = putc_unlocked(v15, g_42d5c0);
            if (a2 && v1 < a1)
            {
                v13 = *(v1);
                if ((char)v13 != 10)
                    v13 = fprintf(g_42d5c0, a2);
            }
            iter = 0;
        }
        if ((unsigned int)v13 <= 13)
        {
            if ((unsigned int)v13 != 8)
            {
                if ((unsigned int)v13 == 9)
                {
                    j = v6 - iter % v6;
                    iter += j;
                    do
                    {
                        v13 = putc_unlocked(32, g_42d5c0);
                        j -= 1;
                    } while (j);
                }
            }
            else
            {
                if (iter)
                {
                    iter -= 1;
                    v13 = putc_unlocked(v15, g_42d5c0);
                    continue;
                }
            }
        }
        v16 = __ctype_b_loc();
        iter += *((short *)(v15 * 2 + *(v16))) & 0x4000;
        v13 = putc_unlocked(v15, g_42d5c0);
    }
}



// Function: put_indicator @ 0x12889
typedef struct struct_0 {
    char field_0;
    char padding_1[7];
    void* field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_42d5c0;

long long put_indicator(struct_0 *a0)
{
    return fwrite_unlocked(a0->field_8, *((long long *)&a0->field_0), 1, g_42d5c0);
}



// Function: set_color_context @ 0x128c2
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned int g_42d010;
extern struct_0 g_42d040;
extern struct_0 g_42d050;
extern struct_0 g_42d070;
extern struct_0 g_42d080;
extern struct_0 g_42d090;
extern struct_0 g_42d0a0;
extern struct_0 g_42d0b0;
extern char g_42d859;

long long set_color_context(unsigned int a0)
{
    unsigned long v1;  // rax

    if (a0 != 3)
        sub_410d14();
    if (!g_42d859)
        return g_42d859;
    v1 = g_42d010;
    if (a0 == (unsigned int)v1)
        return v1;
    sub_412889(&g_42d040.field_0);
    switch (a0)
    {
    case 0:
        sub_412889(&g_42d080.field_0);
        break;
    case 1:
        sub_412889(&g_42d090.field_0);
        break;
    case 2:
        sub_412889(&g_42d0a0.field_0);
        break;
    case 3:
        sub_412889(&g_42d070.field_0);
        sub_412889(&g_42d050.field_0);
        g_42d010 = a0;
        return a0;
    case 4:
        sub_412889(&g_42d0b0.field_0);
        break;
    default:
        abort(); /* do not return */
    }
}



// Function: translate_line_number @ 0x129a8
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

long long translate_line_number(struct_0 *a0, unsigned long a1)
{
    return a1 + a0->field_e8 + 1;
}



// Function: translate_range @ 0x129d0
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

long long * translate_range(struct_0 *a0, unsigned int a1, unsigned int a2, long long *a3, long long *a4)
{
    *(a3) = sub_4129a8(a0, a1 - 1) + 1;
    *(a4) = sub_4129a8(a0, a2 + 1) - 1;
    return a4;
}



// Function: print_number_range @ 0x12a37
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_42d5c0;

void print_number_range(char a0, struct_0 *a1, unsigned int a2, unsigned int a3)
{
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    sub_4129d0(a1, a2, a3, &v0, &v1);
    if (v1 <= v0)
    {
        fprintf(g_42d5c0, "%ld", v1);
        return;
    }
    fprintf(g_42d5c0, "%ld%c%ld", v0, a0, v1);
    return;
}



// Function: analyze_hunk @ 0x12af4
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned int g_42d1e8;
extern char g_42d1ec;
extern long long g_42d260;
extern unsigned long long g_42d280;
extern unsigned long long g_42d418;
extern unsigned long long g_42d548;

unsigned int analyze_hunk(struct_0 *idx, unsigned long long *a1, unsigned long long *a2, unsigned long long *a3, unsigned long long *a4)
{
    char v23;  // al
    char v24;  // al
    char v25;  // al
    unsigned long long *v26;  // rax
    unsigned long long *v27;  // rax
    char v0;  // [bp-0xa3]
    char v1;  // [bp-0xa2]
    char v2;  // [bp-0xa1]
    struct_0 *idx1;  // [bp-0xa0]
    unsigned long long v4;  // [bp-0x98]
    unsigned long long v5;  // [bp-0x90]
    unsigned long iter;  // [bp-0x88]
    unsigned long k;  // [bp-0x80]
    char *l;  // [bp-0x78]
    unsigned long v9;  // [bp-0x70]
    unsigned long v10;  // [bp-0x68]
    unsigned long v11;  // [bp-0x60]
    unsigned long v12;  // [bp-0x58]
    unsigned long v13;  // [bp-0x50]
    unsigned long v14;  // [bp-0x48]
    unsigned long v15;  // [bp-0x40]
    unsigned long v16;  // [bp-0x38]
    unsigned long long v17;  // [bp-0x30]
    char *v18;  // [bp-0x28]
    unsigned long v19;  // [bp-0x20]
    unsigned long v20;  // [bp-0x18]
    unsigned long long v21;  // [bp-0x10]

    if (g_42d1ec || g_42d280)
        v23 = 1;
    else
        v23 = 0;
    v0 = v23;
    v0 &= 1;
    v9 = g_42d1ec - 1;
    if (g_42d1ec && g_42d1e8 > 1)
        v24 = 1;
    else
        v24 = 0;
    v1 = v24;
    v1 &= 1;
    if (v1 && g_42d1e8 > 3)
        v25 = 1;
    else
        v25 = 0;
    v2 = v25;
    v2 &= 1;
    v10 = g_42d418;
    v11 = g_42d548;
    v5 = 0;
    v4 = v5;
    *(a1) = idx->field_18;
    *(a3) = idx->field_20;
    idx1 = idx;
    do
    {
        v12 = idx1->field_10 + idx1->field_18 - 1;
        v13 = idx1->field_8 + idx1->field_20 - 1;
        v4 += idx1->field_10;
        v5 += idx1->field_8;
        for (iter = idx1->field_18; iter <= v12 && v0; iter += 1)
        {
            v14 = *((long long *)(v10 + iter * 8));
            v15 = *((long long *)(v10 + (iter + 1) * 8)) - 1;
            v16 = v15 + (*((char *)v15) != 10);
            v17 = v16 - v14;
            if (v1)
            {
                for (k = v14; *((char *)k) != 10; k += 1)
                {
                    v26 = __ctype_b_loc();
                    if (!(*((short *)(*((char *)k) * 2 + *(v26))) & 0x2000))
                    {
                        if (v2 != 1)
                            k = v14;
                        break;
                    }
                }
            }
            if (v16 - k != v9 && (!g_42d280 || re_search(&g_42d260, v14, v17 & 0xffffffff, 0, v17 & 0xffffffff, 0) < 0))
                v0 = 0;
        }
        for (iter = idx1->field_20; iter <= v13 && v0; iter += 1)
        {
            v18 = *((long long *)(v11 + iter * 8));
            v19 = *((long long *)(v11 + (iter + 1) * 8)) - 1;
            v20 = v19 + (*((char *)v19) != 10);
            v21 = v20 - v18;
            if (v1)
            {
                for (l = v18; *(l) != 10; l += 1)
                {
                    v27 = __ctype_b_loc();
                    if (!(*((short *)(*(l) * 2 + *(v27))) & 0x2000))
                    {
                        if (v2 != 1)
                            l = v18;
                        break;
                    }
                }
            }
            if (v20 - l != v9 && (!g_42d280 || re_search(&g_42d260, v18, v21 & 0xffffffff, 0, v21 & 0xffffffff, 0) < 0))
                v0 = 0;
        }
        idx1 = idx1->field_0;
    } while (idx1);
    *(a2) = v12;
    *(a4) = v13;
    if (!v0)
        return v4 | (!v5 ? 0 : 2);
    return 0;
}



// Function: concat @ 0x12fa9
char * concat(char *a0, char *a1, char *a2)
{
    unsigned long len;  // rax
    unsigned long len1;  // rax
    char *v0;  // [bp-0x20]

    len = strlen(a0);
    len1 = strlen(a1);
    v0 = sub_41c945(strlen(a2) + len + len1 + 1);
    sprintf(v0, "%s%s%s", a0, a1, a2);
    return v0;
}



// Function: zalloc @ 0x13030
void* zalloc(unsigned long n)
{
    void* v0;  // [bp-0x10]

    v0 = sub_41c945(n);
    memset(v0, 0, n);
    return v0;
}



// Function: debug_script @ 0x1306b
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern FILE *stderr;

int debug_script(struct_0 *a0)
{
    struct_0 *idx;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]
    unsigned long v4;  // [bp-0x10]

    idx = a0;
    fflush_unlocked(stdout);
    for (; idx; idx = idx->field_0)
    {
        v1 = idx->field_18;
        v2 = idx->field_20;
        v3 = idx->field_10;
        v4 = idx->field_8;
        fprintf(stderr, "%3ld %3ld delete %ld insert %ld\n", v1, v2, v3, v4);
    }
    return fflush_unlocked(stderr);
}



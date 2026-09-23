// Function: main @ 0x39c0
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern unsigned int g_412f80[4];
extern option g_4187c0;
extern int g_419028;
extern int g_41902c;
extern long long stdout;
extern int optind;
extern long long stderr;
extern unsigned int g_41b420;
extern unsigned short g_41b424;
extern long long g_41b850;
extern char g_41b85c;
extern char g_4dd510;
extern unsigned int g_4dd920;
extern unsigned long long g_4dd9c0;
extern long long g_4dd9c8;
extern unsigned long long g_4dd9d0;
extern unsigned int g_4dd9e8;
extern unsigned int g_4dd9f8;
extern unsigned int g_4dd9fc;

int main(int a0, long long *a1)
{
    unsigned long long v11;  // r15
    unsigned long long v12;  // r14
    void* v21;  // rax
    unsigned int v22;  // ecx
    unsigned int v23;  // eax
    unsigned long v24;  // rax
    long long v25;  // rdi
    long long v26;  // rsi
    long long v27;  // rdx
    long long v28;  // rcx
    long long v29;  // r8
    long long v30;  // r9
    unsigned long long v13;  // r13
    unsigned long v31;  // rax
    unsigned int v32;  // eax
    int v33;  // r12d
    void* err;  // rax
    unsigned long v35;  // rdi
    char *v36;  // rdi
    char v37;  // cl
    unsigned long long v14;  // r12
    unsigned long long v15;  // rbx
    unsigned long v16;  // fs
    unsigned long v17;  // rax
    unsigned long len;  // rax
    void* v19;  // r12
    long long *v20;  // r15
    int v0;  // [bp-0x64], Other Possible Types: unsigned int
    char v1;  // [bp-0x50]
    int v2;  // [bp-0x4c]
    long long *v3;  // [bp-0x48]
    unsigned long v4;  // [bp-0x40]
    unsigned long long v5;  // [bp-0x30]
    unsigned long long v6;  // [bp-0x20]
    unsigned long long v7;  // [bp-0x18]
    unsigned long long v8;  // [bp-0x10]
    unsigned long long v9;  // [bp-0x8]

    v9 = v11;
    v8 = v12;
    v7 = v13;
    v6 = v14;
    v5 = v15;
    v0 = a0;
    v4 = *((long long *)(40 + v16));
    v17 = sub_40d3b0(*(a1));
    g_41b850 = v17;
    len = strlen(v17);
    if (len > 4)
    {
        v19 = v17 + len - 4;
        if (!strcmp(v19, ".exe"))
            *((char *)v19) = 0;
    }
    v3 = a1;
    v20 = sub_40d3d0(&v1, &v3, "GZIP");
    g_4dd9d0 = v20;
    if (v20)
        v20 = v3;
    g_4dd9c8 = ".gz";
    g_4dd9c0 = strlen(".gz");
    v2 = -0x1;
    if (v20)
    {
        v21 = optind;
        if (*((long long *)((char *)v20 + 0x8 * v21)) && !strcmp(*((long long *)((char *)v20 + 0x8 * v21)), "--"))
        {
            if (!v20)
                goto LABEL_403b70;
            v22 = 176;
            goto LABEL_403ac1;
        }
        else
        {
            v23 = getopt_long(v1, v20, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &g_4187c0.name, &v2);
            v22 = v23 + 131;
            if (v23 >= 0)
            {
                v24 = v22 - 49;
                if ((unsigned int)v24 > 211)
                    goto LABEL_403ba1;
LABEL_403b5e:
                goto *((void *)((long long)(g_412f80[v24] + (char *)&g_412f80[0])));
            }
            else if (optind == v1)
            {
                if (optind != 1 && !*((int *)&g_41b85c))
                    sub_40f630(stderr, "%s: warning: GZIP environment variable is deprecated; use an alias or script\n", g_41b850);
                free(v20);
                v2 = -0x1;
                optind = 1;
            }
            else
            {
                sub_40f630(stderr, "%s: %s: non-option in GZIP environment variable\n", g_41b850, v20[optind]);
                sub_407040(v25, v26, v27, v28, v29, v30); /* do not return */
            }
        }
    }
    else
    {
LABEL_403b70:
    }
    v22 = getopt_long(v0, a1, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &g_4187c0.name, &v2);
    if (v22 >= 0)
    {
        v24 = v22 - 49;
        if ((unsigned int)v24 <= 211)
            goto LABEL_403b5e;
LABEL_403ba1:
        if (v22 <= 130 || v22 == 194)
        {
LABEL_403bb5:
            sub_407040(); /* do not return */
        }
LABEL_403ac1:
        v0 = v22;
        sub_40f630(stderr, "%s: ", g_41b850);
        v31 = v2;
        if ((unsigned int)v31 >= 0)
            sub_40f630(stderr, "--%s: ", (&g_4187c0.name)[4 * v31]);
        else
            sub_40f630(stderr, "-%c: ", v0 - 131, v0);
        sub_40f630(stderr, "option not valid in GZIP environment variable\n");
        goto LABEL_403bb5;
    }
    if (g_419028 < 0)
        g_419028 = g_4dd9f8;
    if (g_41902c < 0)
        g_41902c = g_4dd9f8;
    if (g_4dd9fc)
    {
        if (*((int *)&g_41b85c))
        {
            if (g_4dd9c0 - 1 > 29)
                goto LABEL_4040cd;
            goto LABEL_403f9e;
        }
        else
        {
            sub_40f630(stderr, "%s: option --ascii ignored on this system\n", g_41b850);
        }
    }
    if (g_4dd9c0 - 1 > 29)
    {
LABEL_4040cd:
        sub_40f630(stderr, "%s: invalid suffix '%s'\n", g_41b850, g_4dd9c8);
        sub_406fd0(1); /* do not return */
    }
    v32 = *((int *)&g_41b85c);
    if (!*((int *)&g_41b85c))
        goto LABEL_403fa3;
LABEL_403f9e:
    v32 = 13;
LABEL_403fa3:
    g_4dd920 = v32;
    v33 = v0 - optind;
    sub_405570();
    if (v33)
    {
        while (1)
        {
            err = optind;
            if ((unsigned int)err >= v0)
                break;
            v35 = *((long long *)((char *)a1 + 0x8 * err));
            optind = (unsigned int)err + 1;
            sub_407b60(v35);
        }
    }
    else
    {
        err = sub_407070();
    }
    if (g_4dd510)
    {
        err = close(0);
        if ((unsigned int)err)
        {
            g_41b420 = 1768191091;
            g_41b424 = 110;
            sub_40d600(); /* do not return */
        }
    }
    if (g_4dd9e8)
    {
        if (!*((int *)&g_41b85c) && v33 > 1)
            sub_406c40(0xffffffff);
        err = sub_40e460(stdout);
    }
    __unsupported_jumpkind_Ijk_NoDecode()
    *((unsigned int *)err) = *((int *)err) + (unsigned int)err;
    *(v36) = *(v36) + v37;
    *((char *)err) = *((char *)err) + *((char *)&err);
}



// Function: bi_init @ 0x4260
extern unsigned long long g_41a050;
extern unsigned int g_41a058;
extern unsigned short g_41a05c;

int bi_init(unsigned int a0)
{
    unsigned int v1;  // eax

    g_41a058 = 0;
    v1 = 0;
    g_41a05c = 0;
    if (a0 != 0xffffffff)
    {
        v1 = (unsigned int)sub_40e010;
        g_41a050 = sub_40e010;
    }
    return v1;
}



// Function: send_bits @ 0x4290
extern void g_41a058;
extern unsigned short g_41a05c;
extern unsigned int g_41b004;
extern char g_45c000;

unsigned long long send_bits(unsigned int a0, unsigned int a1)
{
    unsigned int v1;  // edx
    unsigned long v2;  // rax
    unsigned long long v3;  // rax
    unsigned int v4;  // esi
    unsigned long v5;  // rcx
    unsigned long idx;  // rsi

    v1 = *((int *)&g_41a058);
    v2 = a0 << ((char)v1 & 31);
    v3 = _INSERT(v2, 0, (unsigned short)v2 | g_41a05c);
    v4 = a1 + v1;
    if (16 - a1 < v1)
    {
        v5 = g_41b004;
        g_41a05c = v3;
        idx = (unsigned int)v5 + 1;
        if ((unsigned int)v5 <= 0x3fffd)
        {
            (&g_45c000)[v5 & 0xffffffff] = v3;
            g_41b004 = (unsigned int)v5 + 2;
            (&g_45c000)[idx] = *((char *)((void*)&v3 + 1));
            v4 = v1 + a1 - 16;
            v3 = (int)(a0 & 0xffff) >> ((char)(16 - v1) & 31);
        }
        else
        {
            g_41b004 = idx;
            (&g_45c000)[v5] = v3;
            if ((unsigned int)idx == 0x40000)
            {
                sub_40d7a0();
                v3 = g_41a05c;
                idx = g_41b004;
            }
            g_41b004 = (unsigned int)idx + 1;
            (&g_45c000)[idx] = *((char *)((void*)&v3 + 1));
            if (g_41b004 == 0x40000)
                goto LABEL_404358;
            while (1)
            {
LABEL_404358:
                sub_40d7a0();
            }
        }
    }
    g_41a05c = v3;
    *((unsigned int *)&g_41a058) = v4;
    return v3;
}



// Function: bi_reverse @ 0x4380
unsigned int bi_reverse(unsigned int a0, int i)
{
    unsigned int v1;  // edx
    unsigned int v2;  // edi
    unsigned int v3;  // edx
    unsigned int v5;  // edx

    v1 = 0;
    do
    {
        v2 = a0;
        i -= 1;
        v5 = (v2 & 1 | v1) * 2;
        v1 = v5;
        a0 = v2 >> 1;
    } while (i > 0);
    return (v2 & 1 | v3) & 0x7fffffff;
}



// Function: bi_windup @ 0x43b0
extern int g_41a058;
extern unsigned short g_41a05c;
extern unsigned int g_41b004;
extern char g_45c000;

unsigned int bi_windup(void)
{
    unsigned long v1;  // rax
    char v2;  // cl
    unsigned long idx;  // rdx
    unsigned long v4;  // rax

    if (g_41a058 > 8)
    {
        v1 = g_41b004;
        v2 = g_41a05c;
        idx = (unsigned int)v1 + 1;
        if ((unsigned int)v1 <= 0x3fffd)
        {
            (&g_45c000)[v1 & 0xffffffff] = g_41a05c;
            g_41b004 = (unsigned int)v1 + 2;
            (&g_45c000)[idx] = g_41a05c;
        }
        else
        {
            g_41b004 = idx;
            (&g_45c000)[v1] = g_41a05c;
            if ((unsigned int)idx == 0x40000)
            {
                sub_40d7a0();
                v2 = g_41a05c;
                idx = g_41b004;
            }
            (&g_45c000)[idx] = v2;
            g_41b004 = (unsigned int)idx + 1;
            if (g_41b004 == 0x40000)
                goto LABEL_40443b;
        }
    }
    else if (g_41a058 > 0 && !(v4 = (unsigned long)(unsigned long long)g_41b004, g_41b004 = (unsigned int)v4 + 1, (&g_45c000)[v4] = (char)g_41a05c, g_41b004 != 0x40000))
    {
LABEL_40443b:
        sub_40d7a0();
    }
    g_41a058 = 0;
    g_41a05c = 0;
    return 0;
}



// Function: copy_block @ 0x44a0
extern unsigned int g_41b004;
extern char g_45c000;

unsigned long long copy_block(char *a0, unsigned int a1, unsigned int a2)
{
    char *iter;  // rbx
    unsigned long long v3;  // rax
    unsigned long long v12;  // rax
    char v13;  // cl
    unsigned long long v14;  // rax
    unsigned long long v4;  // rax
    char v5;  // r12b
    unsigned long long idx;  // rdx
    unsigned long v7;  // rax
    unsigned long index;  // rdx
    char v9;  // sil
    unsigned int v10;  // ecx
    char *v11;  // r12
    unsigned int v0;  // [bp-0x2c]

    iter = a0;
    v3 = sub_4043b0();
    if (a2)
    {
        v4 = g_41b004;
        v5 = (unsigned short)a1 >> 8;
        idx = (unsigned int)v4 + 1;
        if ((unsigned int)v4 <= 0x3fffd)
        {
            v7 = (unsigned int)v4 + 2;
            (&g_45c000)[v4 & 0xffffffff] = a1;
            (&g_45c000)[idx] = v5;
        }
        else
        {
            g_41b004 = idx;
            (&g_45c000)[v4] = a1;
            if ((unsigned int)idx == 0x40000)
            {
                sub_40d7a0();
                idx = g_41b004;
            }
            v7 = (unsigned int)idx + 1;
            g_41b004 = v7;
            (&g_45c000)[idx] = v5;
            if ((unsigned int)v7 == 0x40000)
            {
                sub_40d7a0();
                v7 = g_41b004;
            }
        }
        index = (unsigned int)v7 + 1;
        v9 = ~(a1);
        v10 = _INSERT(~(a1), 0, (unsigned short)~(a1) >> 8);
        if ((unsigned int)v7 <= 0x3fffd)
        {
            v3 = (unsigned int)v7 + 2;
            (&g_45c000)[v7 & 0xffffffff] = v9;
            g_41b004 = v3;
            (&g_45c000)[index] = v10;
        }
        else
        {
            g_41b004 = index;
            (&g_45c000)[v7] = v9;
            if ((unsigned int)index == 0x40000)
            {
                v0 = v10;
                sub_40d7a0();
                index = g_41b004;
                v10 = v0;
            }
            v3 = (unsigned int)index + 1;
            g_41b004 = v3;
            (&g_45c000)[index] = v10;
            if ((unsigned int)v3 == 0x40000)
                v3 = sub_40d7a0();
        }
    }
    v11 = &iter[a1];
    if (!a1)
        return v3;
    while (1)
    {
        v12 = g_41b004;
        v13 = *(iter);
        iter += 1;
        g_41b004 = (unsigned int)v12 + 1;
        (&g_45c000)[v12] = v13;
        if (g_41b004 == 0x40000)
        {
            v14 = sub_40d7a0();
            if (iter == v11)
                return v14;
        }
        else if (iter == v11)
        {
            return v12;
        }
    }
}



// Function: longest_match @ 0x4620
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_41a060;
extern unsigned int g_41a064;
extern unsigned int g_41a06c;
extern int g_41a070;
extern struct_0 g_41b880;
extern char g_43c000;

int longest_match(unsigned int a0)
{
    unsigned int v4;  // edx
    int v5;  // r8d
    char v14;  // r10b
    char v15;  // r13b
    char *v16;  // rax
    char *iter;  // rsi
    unsigned long node;  // rax
    unsigned long v6;  // rcx
    unsigned long long v7;  // r12
    unsigned long v8;  // rsi
    char *v9;  // r12
    void* v10;  // rdi
    unsigned long long v11;  // rbx
    char *v12;  // rbx
    char v13;  // bpl
    unsigned long long v1;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x20]

    v4 = a0;
    v5 = g_41a070;
    v6 = g_41a064;
    v2 = v7;
    v8 = g_41a070;
    v9 = &(&g_43c000)[g_41a06c + 258];
    v10 = (g_41a06c < 32506 ? 32506 : g_41a06c) - 32506;
    v1 = v11;
    v12 = &(&g_43c000)[g_41a06c];
    v13 = *(&v12[g_41a070] - 1);
    v14 = v12[g_41a070];
    if (g_41a070 >= *((int *)&g_41a060))
        v6 = g_41a064 >> 2;
    v15 = 0;
    v16 = &(&g_43c000)[v4];
    if (v16[v8] != v14 || *(&v16[v8] - 1) != v13 || *(v16) != *(v12) || v16[1] != v12[1])
    {
        v4 = (&g_41b880.field_0)[v4 & 0x7fff];
    }
    else
    {
        iter = v12 + 2;
        node = v16 + 2;
    }
    if (iter[1] != *((char *)(node + 1)))
        iter += 1;
    v6 = _INSERT(v6, 0, (char)v6 + *((char *)v10));
    *((char *)node) = *((char *)node) + (char)node;
}



// Function: rsync_roll @ 0x4840
extern unsigned long long g_41a088;
extern unsigned long long g_41a090;
extern char g_43c000;
extern char g_43c001;

unsigned long long rsync_roll(unsigned int a0, unsigned int a1)
{
    unsigned long long v2;  // rax
    unsigned int v3;  // r9d
    unsigned long long v12;  // rbx
    char v13;  // r11b
    unsigned int v14;  // cc_dep1
    unsigned long long v4;  // r8
    unsigned long v5;  // rdi
    char *v6;  // rdx
    unsigned long long v7;  // rax
    char *v8;  // rdx
    char *v9;  // rdx
    unsigned long v10;  // rsi
    unsigned long long v11;  // r8
    unsigned long long v0;  // [bp-0x8]

    v2 = a0;
    v3 = (unsigned int)v2 + a1;
    if ((unsigned int)v2 <= 0xfff)
    {
        v4 = v2 & 0xffffffff;
        v5 = g_41a090;
        v6 = &(&g_43c000)[v4];
        v7 = 0;
        v8 = v6;
        do
        {
            if (&v6[a1] == v8)
            {
                if (!(char)v7)
                    return v7;
                g_41a090 = v5;
                return v7;
            }
        } while ((v9 = v8 + 1, v5 = (unsigned long)(v5 + (unsigned long long)*(v8)), v7 = 1, v8 += 1, &g_43c001 + v4 + 0xfff - v2 != v8));
        g_41a090 = v5;
        v2 = 0x1000;
    }
    if ((unsigned int)v2 >= v3)
        return v2;
    v10 = g_41a090;
    v11 = g_41a088;
    v0 = v12;
    v13 = 0;
    do
    {
        v10 += (&g_43c000)[v2] - (&g_43c000)[v2 - 0x1000 & 0xffffffff];
        if (v11 != 0xffffffff)
            continue;
        v14 = (unsigned int)v10 & 0xfff;
        if (!((unsigned short)v10 & 0xfff))
        {
            v11 = v2;
            if (v14)
                goto LABEL_404905;
            goto LABEL_404904;
        }
        else
        {
            v11 = v11;
            if (!v14)
            {
LABEL_404904:
                v13 = 1;
            }
            else
            {
LABEL_404905:
            }
        }
    } while ((v2 += 1, v3 > (unsigned int)v2));
    g_41a090 = v10;
    if (!v13)
        return v2;
    g_41a088 = v11;
    return v2;
}



// Function: fill_window @ 0x4940
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern struct_0 *g_41a050;
extern unsigned int g_41a068;
extern unsigned int g_41a06c;
extern unsigned long long g_41a078;
extern unsigned long long g_41a088;
extern unsigned int g_41a09c;
extern unsigned int g_41a0a0;
extern short g_41b880;
extern short g_42b880;
extern short g_43b880;
extern void g_43c000;
extern void g_444000;

short * fill_window(void)
{
    short *v1;  // rax
    short *v2;  // rdx
    short *v3;  // rdx
    short *v4;  // rax
    short *v5;  // rax

    if ((unsigned int)(0x10000 - (g_41a09c + g_41a06c)) != 0xffffffff && g_41a06c > 65273)
    {
        memcpy(&g_43c000, &g_444000, 0x8000);
        g_41a068 = g_41a068 - 0x8000;
        g_41a06c = g_41a06c - 0x8000;
        if (g_41a088 != 0xffffffff)
            g_41a088 = g_41a088 - 0x8000;
        g_41a078 = g_41a078 - 0x8000;
        v1 = &g_41b880;
        v2 = &g_42b880;
        do
        {
        } while ((v3 = v2 + 2, *((unsigned short *)((char *)v3 - 2)) = (*(v2) >= 0 ? 0 : *(v2) + 0x8000), v2 = v3, v2 != &g_43b880));
        do
        {
        } while ((v4 = v1 + 2, *((unsigned short *)((char *)v4 - 2)) = (*(v1) >= 0 ? 0 : *(v1) + 0x8000), v1 = v4, v1 != &g_42b880));
    }
    if (g_41a0a0)
        return v4;
    v5 = g_41a050(&(&g_43c000)[g_41a09c + g_41a06c]);
    if ((unsigned int)(short *)((char *)v5 - 1) > 0xfffffffd)
    {
        g_41a0a0 = 1;
        *((unsigned short *)&(&g_43c000)[g_41a09c + g_41a06c]) = 0;
        return g_41a06c + g_41a09c;
    }
    g_41a09c = g_41a09c + (unsigned int)v5;
    return v5;
}



// Function: lm_init @ 0x4db0
typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern struct_0 g_412020;
extern struct_1 *g_41a050;
extern unsigned int g_41a060;
extern unsigned int g_41a064;
extern unsigned int g_41a06c;
extern unsigned long long g_41a078;
extern unsigned int g_41a080;
extern unsigned long long g_41a088;
extern unsigned long long g_41a090;
extern unsigned int g_41a098;
extern unsigned int g_41a09c;
extern unsigned int g_41a0a0;
extern char g_41a0a4;
extern void g_42b880;
extern char g_43c000;
extern char g_43c001;

long long lm_init(int a0)
{
    unsigned long v1;  // rdi
    unsigned int v2;  // edx
    unsigned int v3;  // eax
    unsigned int v4;  // eax

    if (a0 - 1 > 8)
        sub_40d510("bad pack level"); /* do not return */
    memset(&g_42b880, 0, 0x10000);
    v1 = a0;
    g_41a090 = 0;
    g_41a088 = 0xffffffff;
    g_41a06c = 0;
    v2 = (&g_412020.field_0)[1 + 4 * v1];
    g_41a078 = 0;
    g_41a098 = v2;
    g_41a060 = (&g_412020.field_0)[4 * v1];
    v3 = (&g_412020.field_0)[3 + 4 * v1];
    g_41a080 = (&g_412020.field_0)[2 + 4 * v1];
    g_41a064 = v3;
    g_41a09c = g_41a050(&g_43c000, 0x10000, g_41a080);
    if (g_41a09c - 1 > 0xfffffffd)
    {
        g_41a0a0 = 1;
        g_41a09c = 0;
        return g_41a09c;
    }
    g_41a0a0 = 0;
    if (g_41a09c <= 261)
    {
        do
        {
            sub_404940();
        } while (g_41a09c <= 261 && !g_41a0a0);
    }
    v4 = g_43c000 * 32;
    *((unsigned int *)&g_41a0a4) = v4 ^ g_43c001;
    return v4 ^ g_43c001;
}



// Function: deflate @ 0x4ed0
extern unsigned int g_41a068;
extern unsigned int g_41a06c;
extern unsigned int g_41a070;
extern long long g_41a078;
extern unsigned long long g_41a088;
extern char g_41a098;
extern unsigned int g_41a09c;
extern unsigned int g_41a0a0;
extern unsigned int g_41a0a4;
extern unsigned int g_41b000;
extern void g_41b880;
extern char g_43c000;

long long deflate(int a0)
{
    unsigned int v4;  // edx
    unsigned int v5;  // r13d
    unsigned long v14;  // rcx
    unsigned int v15;  // eax
    unsigned int v16;  // edx
    unsigned int v17;  // edx
    unsigned int v18;  // edx
    unsigned int v19;  // esi
    unsigned int v20;  // r9d
    unsigned int v21;  // eax
    unsigned int v22;  // ebp
    unsigned int v23;  // eax
    unsigned int v6;  // esi
    unsigned int v24;  // edx
    unsigned int v25;  // r14d
    unsigned int v26;  // r9d
    unsigned int v27;  // ecx
    unsigned int i;  // eax
    unsigned int v29;  // eax
    unsigned long idx;  // r10
    void* v31;  // rsi
    unsigned short v32;  // r11w
    unsigned long long *v33;  // r8
    unsigned int v7;  // r14d
    unsigned int v34;  // r15d
    long long v35;  // rdi
    unsigned int v8;  // r13d
    unsigned int v9;  // r15d
    unsigned int v10;  // ebp
    void* v11;  // rax
    unsigned int v12;  // r8d
    int v13;  // eax
    unsigned int v0;  // [bp-0x44]
    unsigned int v1;  // [bp-0x40]
    unsigned int v2;  // [bp-0x3c]

    sub_404db0(a0);
    if (a0 > 3)
    {
        v4 = g_41a09c;
        v5 = 0xffffffff;
        if (g_41a09c)
        {
            v6 = 2;
            v7 = 0;
            v8 = 0;
            while (1)
            {
                v9 = g_41a06c;
                v10 = 2;
                g_41a0a4 = (g_41a0a4 * 32 ^ (&g_43c000)[2 + g_41a06c]) & 0x7fff;
                v11 = &(&g_41b880)[0x10000 + 2 * g_41a0a4];
                v12 = *((short *)v11);
                *((unsigned short *)&(&g_41b880)[2 * (g_41a06c & 0x7fff)]) = v12;
                *((unsigned short *)v11) = g_41a06c;
                g_41a070 = v6;
                if (v12 && *((int *)&g_41a098) > v6)
                {
                    if (g_41a06c - v12 > 32506 || g_41a06c > 65274)
                    {
                        v10 = 2;
                    }
                    else
                    {
                        v1 = v4;
                        v0 = g_41a068;
                        v13 = sub_404620(v12 & 0xffff);
                        __unsupported_jumpkind_Ijk_NoDecode()
                        (&g_41b880)[76 + 4 * v14] = (&g_41b880)[76 + 4 * v14] | (char)v13;
                        v15 = _INSERT(v13 CONCAT 0, 0, (char)v13 & 4);
                        if (v15 <= v16)
                        {
                            v17 = v15;
                            v18 = v17;
                        }
                        else
                        {
                            v18 = v16;
                        }
                        v10 = v18;
                        if (v18 != 3)
                        {
                            v9 = g_41a06c;
                        }
                        else
                        {
                            v9 = g_41a06c;
                            *((char *)(v14 * 5 - 8)) = *((char *)(v14 * 5 - 8)) + (char)v17;
                            if (4097 <= v17 - g_41a068)
                                v10 = 2;
                        }
                    }
                }
                if (v19 <= 2 || v10 > v19)
                    break;
                v21 = sub_40b290(v9 - v20 - 1 & 0xffffffff, v19 - 3);
                v22 = g_41a070;
                v8 = v21;
                v23 = v22 - 2;
                v24 = g_41a09c + 1 - v22;
                g_41a070 = v23;
                g_41a09c = v24;
                if (!g_41b000)
                {
                    v25 = g_41a06c;
                    v26 = v22 - 1;
                }
                else
                {
                    v25 = g_41a06c;
                    v2 = g_41a09c;
                    v1 = v23;
                    v0 = v22 - 1;
                    sub_404840(g_41a06c, v0);
                    v23 = v1;
                    v26 = v0;
                }
                v27 = g_41a0a4;
                i = v25;
                do
                {
                    v29 = i + 1;
                    idx = v29 & 0x7fff;
                    v27 = (v27 * 32 ^ (&g_43c000)[3 + i]) & 0x7fff;
                    v31 = &(&g_41b880)[0x10000 + 2 * v27];
                    v32 = *((short *)v31);
                    *((unsigned short *)&(&g_41b880)[2 * idx]) = v32;
                    *((unsigned short *)v31) = v29;
                    i = v29;
                } while (i != v23 + v25);
                g_41a0a4 = v27;
                *((unsigned short *)&(&g_41b880)[2 * idx]) = v32;
                *((unsigned int *)v31) = v25 + v22 - 2;
                g_41a070 = 0;
                g_41a06c = v26 + v25;
                if (g_41b000 && g_41a06c > g_41a088)
                {
                    v8 = 2;
                    g_41a088 = 0xffffffff;
                    goto LABEL_4052ed;
                }
                else if (v8)
                {
LABEL_4052ed:
                    v10 = 2;
                    v7 = 0;
                    sub_40b070();
                    *(v33) = *(v33) + (char *)v33;
                    g_41a078 = g_41a06c;
                }
                else
                {
                    v7 = 0;
                    v10 = 2;
                }
LABEL_40504f:
                v6 = v10;
                v4 = v16;
                if (v4 <= 261)
                {
                    while (!g_41a0a0)
                    {
                        sub_404940();
                        v4 = g_41a09c;
                        if (g_41a09c > 261)
                            goto LABEL_405087;
                    }
                    v4 = g_41a09c;
                    if (!g_41a09c)
                    {
                        v5 = v8 - 1;
                        goto LABEL_40509d;
                        if (v7)
                        {
                            sub_40b290(0, *(&(&g_43c000)[g_41a06c] - 1));
                            goto LABEL_40509d;
                        }
                    }
                }
LABEL_405087:
            }
            if (!v7)
            {
                if (g_41b000)
                {
                    if (v9 > g_41a088)
                    {
                        v8 = 2;
                        g_41a088 = 0xffffffff;
                        sub_40b070();
                        g_41a078 = g_41a06c;
                        v9 = g_41a06c;
                        if (g_41b000)
                            goto LABEL_405190;
                    }
                    else
                    {
LABEL_405190:
                        sub_404840(v9, 1);
                    }
                }
                v7 = 1;
                g_41a06c = v9 + 1;
                g_41a09c = g_41a09c - 1;
                goto LABEL_40504f;
            }
            v8 = sub_40b290(0, *(&(&g_43c000)[v9] - 1));
            if (g_41b000)
            {
                v34 = g_41a06c;
                if (g_41a06c > g_41a088)
                {
                    v8 = 2;
                    g_41a088 = 0xffffffff;
                }
                else if (!v8)
                {
LABEL_4051ca:
                    sub_404840(v34, 1);
LABEL_405168:
                    g_41a06c = v34 + 1;
                    g_41a09c = g_41a09c - 1;
                    goto LABEL_40504f;
                }
            }
            else
            {
                v34 = g_41a06c;
                if (!v8)
                    goto LABEL_405168;
            }
            sub_40b070();
            g_41a078 = g_41a06c;
            v34 = g_41a06c;
            if (g_41b000)
                goto LABEL_4051ca;
            goto LABEL_405168;
        }
LABEL_40509d:
        v35 = 0;
        if (g_41a078 >= 0)
            v35 = &(&g_43c000)[g_41a078 & 0xffffffff];
        return sub_40b070(v35, g_41a06c - g_41a078, v5, 1);
    }
}



// Function: get_input_size_and_time @ 0x53f0
extern unsigned int g_419028;
extern unsigned long long g_41b830;
extern unsigned long long g_41b838;
extern unsigned long long g_41b840;
extern long long g_4dd060;
extern unsigned int g_4dd078;
extern unsigned long long g_4dd090;
extern unsigned int g_4dd9e8;

unsigned long long get_input_size_and_time(void)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // rdx

    g_41b840 = 0xffffffffffffffff;
    g_41b838 = 0xffffffffffffffff;
    v1 = g_4dd078 & 0xf000;
    if ((unsigned int)v1 != 0x8000)
        return v1;
    g_41b840 = g_4dd090;
    if (g_419028 && !g_4dd9e8)
        return g_4dd9e8;
    g_41b830 = sub_40e940(&g_4dd060);
    g_41b838 = v2;
    return g_41b830;
}



// Function: volatile_strcpy @ 0x5470
unsigned long long volatile_strcpy(unsigned long a0, unsigned long a1)
{
    char *v1;  // rsi
    unsigned long long v2;  // rax
    char *v3;  // rdi

    v2 = *(v1);
    *(v3) = v2;
    if (!(char)v2)
        return v2;
}



// Function: help @ 0x5490
extern unsigned long long g_4186e0;
extern long long g_41b850;

int help(void)
{
    unsigned long long *v1;  // rbx
    unsigned int v2;  // eax

    v1 = &g_4186e0;
    sub_40f7b0("Usage: %s [OPTION]... [FILE]...\n", g_41b850);
    do
    {
        v1 += 1;
        v2 = sub_40f7b0("%s\n");
    } while (*(v1));
    return v2;
}



// Function: license @ 0x54e0
extern unsigned long long g_418b20;
extern long long g_4193f8;
extern long long g_41b850;

int license(void)
{
    unsigned long long *v1;  // rbx
    unsigned int v2;  // eax

    v1 = &g_418b20;
    sub_40f7b0("%s %s\n", g_41b850, g_4193f8);
    do
    {
        v1 += 1;
        v2 = sub_40f7b0("%s\n");
    } while (*(v1));
    return v2;
}



// Function: version @ 0x5540
int version(void)
{
    sub_4054e0();
    sub_40f7b0("\n");
    return sub_40f7b0();
}



// Function: install_signal_handlers @ 0x5570
typedef struct sigaction {
} sigaction;

typedef struct struct_0 {
    char padding_0[76];
    char field_4c;
} struct_0;

typedef struct struct_2 {
    char padding_0[102];
    unsigned long long field_66;
} struct_2;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern struct_1 g_413350;
extern unsigned int g_413368;
extern uint128_t g_4dd940;
extern uint128_t g_4dd950;
extern uint128_t g_4dd960;
extern uint128_t g_4dd970;
extern uint128_t g_4dd980;
extern uint128_t g_4dd990;
extern uint128_t g_4dd9a0;
extern uint128_t g_4dd9b0;

long long install_signal_handlers(unsigned long a0, unsigned int a1, char a2, unsigned long a3, unsigned long a4, unsigned long a5, unsigned long a6)
{
    unsigned long long v15;  // r15
    unsigned long long v16;  // r14
    unsigned long long index;  // rbx
    int v25;  // ebp
    struct_2 *v26;  // rsi
    unsigned long long v17;  // r13
    unsigned long long v18;  // r12
    char *v19;  // rax
    struct_0 *idx;  // rbx
    unsigned int *iter;  // rbx
    unsigned long v22;  // fs
    int v23;  // ebp
    sigaction v0;  // [bp-0xd0]
    int v1;  // [bp-0xc8]
    int v2;  // [bp-0xb8]
    int v3;  // [bp-0xa8]
    int v4;  // [bp-0x98]
    int v5;  // [bp-0x88]
    int v6;  // [bp-0x78]
    int v7;  // [bp-0x68]
    int v8;  // [bp-0x58]
    unsigned int v9;  // [bp-0x48]
    unsigned long v10;  // [bp-0x38]
    unsigned long long v11;  // [bp-0x20]
    unsigned long long v12;  // [bp-0x18]
    unsigned long long v13;  // [bp-0x10]
    unsigned long long v14;  // [bp-0x8]

    v14 = v15;
    v13 = v16;
    v12 = v17;
    v11 = v18;
    *(v19) = *(v19) + *((char *)&v19);
    idx->field_4c = idx->field_4c + a2;
    iter = a1;
    v10 = *((long long *)(40 + v22));
    sigemptyset(&g_4dd940);
    while (1)
    {
        sigaction(v23, NULL, &v0);
        if (v0 != 1)
            sigaddset(&g_4dd940, v23);
        iter += 1;
        if (iter == &g_413368)
            break;
        v23 = *(iter);
    }
    v9 = 0;
    v0 = (sigaction)sub_4057e0;
    index = 0;
    v1 = (int)g_4dd940;
    v25 = 2;
    v2 = (int)g_4dd950;
    v3 = (int)g_4dd960;
    v4 = (int)g_4dd970;
    v5 = (int)g_4dd980;
    v6 = (int)g_4dd990;
    v7 = (int)g_4dd9a0;
    v8 = (int)g_4dd9b0;
    while (1)
    {
        if (sigismember(&g_4dd940, v25))
        {
            if (!index)
            {
                __unsupported_jumpkind_Ijk_NoDecode()
                __unsupported_jumpkind_Ijk_NoDecode()
            }
            sigaction(v25, &v0, NULL);
        }
        index += 1;
        if (index == 6)
            break;
        v25 = *((int *)((char *)&g_413350.field_0 + 4 * index));
    }
    if (v10 == *((long long *)(40 + v22)))
        return v10 - *((long long *)(40 + v22));
    __unsupported_jumpkind_Ijk_NoDecode()
    goto *((void *)(*((long long *)&(v26->padding_0)[1])));
}



// Function: remove_output_file @ 0x56e0
extern int g_419018;
extern unsigned int g_4dd520;
extern int g_4dd940;

void remove_output_file(char a0, unsigned long a1, unsigned long a2, char a3)
{
    unsigned long idx;  // fs
    int v5;  // edi
    long long v6;  // r8
    int v7;  // edi
    void* v8;  // rax
    char v9;  // cl
    int v0[32];  // [bp-0x498]
    char v1;  // [bp-0x418]
    unsigned long v2;  // [bp-0x10]

    v2 = *((long long *)(40 + idx));
    if (a0)
    {
        v5 = g_419018;
        if (v5 >= 0)
        {
            g_419018 = 0xffffffff;
            close(v5);
            sub_405470(&v1, &g_4dd520);
            sub_40d3c0(v6);
        }
    }
    else
    {
        sigprocmask(0, &g_4dd940, v0);
        v7 = g_419018;
        if (v7 < 0)
        {
            sigprocmask(2, v0, NULL);
        }
        else
        {
            g_419018 = 0xffffffff;
            close(v7);
            sub_405470(&v1, &g_4dd520);
            sub_40d3c0(v6);
            sigprocmask(2, v0, NULL);
        }
    }
    v8 = v2 - *((long long *)(40 + idx));
    *((char *)v8 - 127) = *((char *)v8 - 127) + v9;
}



// Function: abort_gzip_signal @ 0x57e0
extern int g_4dd920;

int abort_gzip_signal(int a0)
{
    char v0;  // [bp+0x0]

    __unsupported_jumpkind_Ijk_NoDecode()
    if (g_4dd920 == a0)
        _exit(2); /* do not return */
    signal(a0, NULL);
    return raise((unsigned int)&v0);
}



// Function: progerror @ 0x5820
extern long long stderr;
extern unsigned int g_41b84c;
extern long long g_41b850;

void progerror(unsigned long a0)
{
    unsigned int *err;  // rax
    unsigned int v2;  // eax
    unsigned short v3;  // dx
    char *v4;  // rdi

    err = __errno_location();
    v2 = sub_40f630(stderr, "%s: ", g_41b850);
    __unsupported_jumpkind_Ijk_NoDecode()
    __outdword(v3, v2);
    *(err) = *(err);
    perror(v4);
    g_41b84c = 1;
    return;
}



// Function: check_ofname @ 0x5870
extern long long stderr;
extern long long g_41b020;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_4dd9e0;
extern unsigned int g_4dd9f4;

unsigned int check_ofname(void)
{
    char *v1;  // rax
    char v2;  // 4098
    unsigned int v11;  // eax
    unsigned long v3;  // cc_ndep
    unsigned long long v4;  // 4107
    long long v5;  // rdi
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9

    if (!g_4dd9f4)
    {
        v1 = sub_40f630(stderr, "%s: %s already exists;", g_41b850, &g_41b020);
        if (g_4dd9e0 && (__unsupported_jumpkind_Ijk_NoDecode()
, v2 = *(v1), *(v1) = *(v1) + *((char *)(void*)&v1), v4 = (unsigned long long)_ccall(4, 1, (unsigned long long)v2, (unsigned long long)*((char *)(void*)&v1), v3), !(v4 & 1) || (int)(long long)isatty(0)))
        {
            sub_40f630(stderr, " do you wish to overwrite (y or n)? ");
            sub_40e460(stderr);
            if ((char)sub_40f5f0(v5, v6, v7, v8, v9, v10))
                goto LABEL_4058de;
        }
        sub_40f630(stderr, "\tnot overwritten\n");
        if (g_41b84c)
            return 1;
        g_41b84c = 2;
        return 1;
    }
    else
    {
LABEL_4058de:
        v11 = sub_40d3c0(&g_41b020);
        if (v11)
        {
            sub_405820(&g_41b020);
            return 1;
        }
        return v11;
    }
}



// Function: input_eof @ 0x5940
extern unsigned int g_41b008;
extern char g_41b00c;
extern unsigned int g_4dd9dc;
extern unsigned int g_4dd9f8;

unsigned int input_eof(void)
{
    if (g_4dd9f8 && !g_4dd9dc)
    {
        if (g_41b008 != *((int *)&g_41b00c))
        {
            return g_4dd9dc;
        }
        else if (*((int *)&g_41b00c) == 0x40000 && sub_40d830(1) != 0xffffffff)
        {
            g_41b008 = 0;
            return g_4dd9dc;
        }
    }
    return 1;
}



// Function: atdir_eq @ 0x59a0
extern void g_4dd100;

unsigned int atdir_eq(void* a0, unsigned long a1)
{
    unsigned long v1;  // rdx
    unsigned long v2;  // rbx

    if (a1)
    {
        v1 = a1;
    }
    else
    {
        v1 = 1;
        a1 = 1;
        a0 = ".";
    }
    v2 = a1;
    if (memcmp(&g_4dd100, a0, v1))
        return 0;
    return !(&g_4dd100)[v2];
}



// Function: atdir_set @ 0x5a00
extern unsigned int g_419014;
extern long long g_4dd100;

long long atdir_set(unsigned long a0, unsigned long long a1)
{
    unsigned long long v1;  // rbx
    char v2;  // al
    unsigned long v3;  // rax
    long long dst;  // rax

    v1 = a1;
    v2 = sub_4059a0(a0, a1);
    v3 = g_419014;
    if (v2)
        return v3;
    if ((unsigned int)v3 >= 0)
    {
        close(v3);
        if (!v1)
        {
LABEL_405a60:
            v1 = 1;
            a0 = ".";
        }
    }
    else if (!v1)
    {
        goto LABEL_405a60;
    }
    dst = __memcpy_chk(&g_4dd100, a0);
    *((char *)(dst + v1)) = 0;
    g_419014 = sub_40e3c0(dst, 0x10000);
    return g_419014;
}



// Function: discard_input_bytes @ 0x5a90
extern unsigned int g_41b008;
extern char g_41b00c;
extern char g_49d000;

void discard_input_bytes(unsigned long long a0, unsigned int a1)
{
    unsigned int v2;  // ebp
    unsigned long long v3;  // rbx
    unsigned long long v4;  // rbx
    unsigned long v5;  // rax
    char v6;  // al
    char v0;  // [bp-0x21]

    v2 = a1 & 2;
    v3 = a0;
    if (!a0)
        return;
    while (1)
    {
        v4 = v3;
        v5 = g_41b008;
        if (g_41b008 < *((int *)&g_41b00c))
        {
LABEL_405ace:
            v6 = (&g_49d000)[v5];
            g_41b008 = (unsigned int)v5 + 1;
            v0 = v6;
            if (v2)
            {
LABEL_405b34:
                sub_40d1f0(&v0, 1);
            }
LABEL_405ae4:
            if (v4 != 0xffffffffffffffff)
            {
                v3 = v4 - 1;
                if (v4 == 1)
                    return;
            }
            else
            {
                if (!v0)
                    return;
                v5 = g_41b008;
                if (g_41b008 < *((int *)&g_41b00c))
                    goto LABEL_405ace;
                goto LABEL_405b25;
            }
        }
        else
        {
LABEL_405b25:
            v0 = sub_40d830(0);
            if (!v2)
                goto LABEL_405ae4;
            goto LABEL_405b34;
        }
    }
}



// Function: get_method @ 0x5b50
extern void g_412cb4;
extern void g_412cb7;
extern void g_412cd7;
extern void g_412cdc;
extern void g_412cdf;
extern void g_412ce2;
extern unsigned int g_419020;
extern unsigned int g_419028;
extern unsigned int g_41902c;
extern unsigned long long g_4190d0;
extern long long stderr;
extern unsigned int g_41b008;
extern unsigned int g_41b00c;
extern long long g_41b020;
extern char g_41b420;
extern unsigned long long g_41b830;
extern unsigned long long g_41b838;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_41b85c;
extern unsigned int g_41b860;
extern char g_49d000;
extern unsigned int g_4dd9d8;
extern unsigned int g_4dd9dc;
extern unsigned int g_4dd9e4;
extern unsigned int g_4dd9e8;
extern unsigned int g_4dd9f4;
extern unsigned long long g_4e0ea8;

unsigned long long get_method(unsigned int a0)
{
    unsigned int v11;  // ecx
    unsigned long v12;  // rax
    unsigned int v21;  // ecx
    unsigned long v22;  // rax
    unsigned int v23;  // eax
    unsigned long v24;  // rax
    unsigned int v25;  // ebx
    unsigned long v26;  // rax
    unsigned int v27;  // edx
    unsigned long v28;  // 4110
    unsigned long v29;  // rcx
    unsigned long v30;  // r14
    unsigned long v13;  // rdx
    unsigned int v31;  // eax
    unsigned long v32;  // rsi
    unsigned long v33;  // rax
    unsigned long long v34;  // r14
    unsigned int v35;  // eax
    unsigned long v36;  // rcx
    unsigned long v37;  // r13
    unsigned long long v38;  // r14
    unsigned int v39;  // eax
    unsigned long long v40;  // rax
    unsigned int v14;  // r14d
    unsigned long long v41;  // r14
    unsigned long v42;  // rax
    unsigned int v43;  // ecx
    unsigned long v44;  // rdx
    char v45;  // al
    char v46;  // al
    unsigned int v47;  // r13d
    char v48;  // r13b
    unsigned long v49;  // rax
    unsigned int v50;  // ecx
    unsigned int v15;  // eax
    unsigned long v51;  // rdx
    char v52;  // al
    char v53;  // al
    unsigned int v54;  // r14d
    char *iter;  // r15
    unsigned long v56;  // rax
    char v57;  // al
    void* ptr;  // rax
    unsigned short v60;  // ax
    unsigned int i;  // r12d
    unsigned int v61;  // ecx
    unsigned int v62;  // ebp
    unsigned long v63;  // rax
    unsigned long v64;  // rdx
    unsigned int v65;  // ebx
    unsigned int v66;  // eax
    unsigned int v67;  // eax
    unsigned int v68;  // eax
    unsigned long v69;  // 4110
    unsigned int v17;  // eax
    unsigned long v18;  // rdx
    char v19;  // al
    unsigned int *v20;  // rdi
    char *v0;  // [bp-0x60]
    char v1;  // [bp-0x4c]
    char v2;  // [bp-0x4b]
    char v3;  // [bp-0x4a]
    char v4;  // [bp-0x49]
    char v5;  // [bp-0x48]
    char v6;  // [bp-0x47]
    unsigned int v7;  // [bp-0x46]
    char v8;  // [bp-0x42]
    char v9;  // [bp-0x41]

    v11 = g_41b00c;
    v12 = g_41b008;
    if (!g_4dd9f4 || !g_41b860)
    {
        if ((unsigned int)v12 < g_41b00c)
        {
            v18 = (unsigned int)v12 + 1;
            g_41b008 = v18;
            v19 = (&g_49d000)[v12];
        }
        else
        {
            v19 = sub_40d830(0);
            v18 = g_41b008;
            v11 = g_41b00c;
        }
        v3 = v19;
        if (!v19)
        {
            if (v11 > (unsigned int)v18)
            {
                g_41b008 = (unsigned int)v18 + 1;
                i = (&g_49d000)[v18];
                v17 = i & 0xff;
            }
            else
            {
                v17 = sub_40d830(1);
                i = v17;
            }
            v14 = 0;
        }
        else
        {
            if (v11 > (unsigned int)v18)
            {
                g_41b008 = (unsigned int)v18 + 1;
                v17 = (&g_49d000)[v18];
            }
            else
            {
                v17 = sub_40d830(0);
            }
            i = 0;
            v14 = 0;
        }
    }
    else
    {
        if ((unsigned int)v12 < g_41b00c)
        {
            v13 = (unsigned int)v12 + 1;
            v14 = (&g_49d000)[v12];
            g_41b008 = v13;
            v15 = v14 & 0xff;
        }
        else
        {
            v15 = sub_40d830(1);
            v13 = g_41b008;
            v11 = g_41b00c;
            v14 = v15;
        }
        v3 = v15;
        if ((unsigned int)v13 < v11)
        {
            g_41b008 = (unsigned int)v13 + 1;
            i = (&g_49d000)[v13];
            v17 = i & 0xff;
        }
        else
        {
            v17 = sub_40d830(1);
            i = v17;
        }
    }
    v4 = v17;
    g_419020 = 0xffffffff;
    g_4e0ea8 = 0;
    g_4dd9d8 = g_4dd9d8 + 1;
    g_4dd9dc = 0;
    if (!memcmp(&v3, &g_412cb4, 2) || !memcmp(&v3, &g_412cb7, 2))
    {
        v22 = g_41b008;
        if ((unsigned int)v22 < g_41b00c)
        {
            g_41b008 = (unsigned int)v22 + 1;
            v23 = (&g_49d000)[v22];
        }
        else
        {
            v23 = sub_40d830(0);
        }
        g_419020 = v23;
        if (v23 != 8)
        {
            sub_40f630(stderr, "%s: %s: unknown method %d -- not supported\n", g_41b850, &g_41b420, v23);
            g_41b84c = 1;
            return 0xffffffff;
        }
        g_4190d0 = sub_40cd20;
        v24 = g_41b008;
        if ((unsigned int)v24 < g_41b00c)
        {
            g_41b008 = (unsigned int)v24 + 1;
            v25 = (&g_49d000)[v24];
        }
        else
        {
            v25 = sub_40d830(0);
        }
        if ((char)v25 & 32)
        {
LABEL_4066ac:
            sub_40f630(stderr);
            g_41b84c = 1;
        }
        else if (!((char)v25 & 192))
        {
            v26 = g_41b008;
            v27 = g_41b00c;
            if (g_41b008 >= g_41b00c)
                goto LABEL_406162;
LABEL_405e0f:
            v29 = (unsigned int)v26 + 1;
            g_41b008 = v29;
            v30 = (&g_49d000)[v26];
            if (v27 <= (unsigned int)v29)
                goto LABEL_406180;
            goto LABEL_405e2c;
        }
        else
        {
            sub_40f630(stderr, "%s: %s has flags 0x%x -- not supported\n", g_41b850, &g_41b420, (char)v25);
            v28 = g_4dd9f4;
            g_41b84c = 1;
            if ((unsigned int)v28 > 1)
            {
                v26 = g_41b008;
                v27 = g_41b00c;
                if (g_41b008 < g_41b00c)
                    goto LABEL_405e0f;
LABEL_406162:
                v31 = sub_40d830(0);
                v29 = g_41b008;
                v27 = g_41b00c;
                v30 = v31;
                if (g_41b00c > g_41b008)
                {
LABEL_405e2c:
                    v32 = (unsigned int)v29 + 1;
                    v33 = (&g_49d000)[v29];
                    g_41b008 = v32;
                    v34 = v30 | v33 * 0x100;
                    if ((unsigned int)v32 >= v27)
                        goto LABEL_4061a4;
                    goto LABEL_405e4f;
                }
                else
                {
LABEL_406180:
                    v35 = sub_40d830(0);
                    v32 = g_41b008;
                    v27 = g_41b00c;
                    v34 = v30 | v35 * 0x100;
                    if (g_41b008 < g_41b00c)
                    {
LABEL_405e4f:
                        v36 = (unsigned int)v32 + 1;
                        v37 = (&g_49d000)[v32];
                        g_41b008 = v36;
                        v38 = v34 | v37 * 0x10000;
                        if (v27 <= (unsigned int)v36)
                            goto LABEL_4061c9;
                        goto LABEL_405e73;
                    }
                    else
                    {
LABEL_4061a4:
                        v39 = sub_40d830(0);
                        v36 = g_41b008;
                        v38 = v34 | v39 * 0x10000;
                        if (g_41b00c > g_41b008)
                        {
LABEL_405e73:
                            g_41b008 = (unsigned int)v36 + 1;
                            v40 = (&g_49d000)[v36] * 0x1000000;
                            goto LABEL_405e8b;
                        }
                        else
                        {
LABEL_4061c9:
                            v40 = sub_40d830(0) * 0x1000000;
LABEL_405e8b:
                            v41 = v38 | v40;
                            if ((v38 || v40) && !g_419028)
                            {
                                if (v41 >= 0)
                                {
                                    g_41b830 = v41;
                                    g_41b838 = 0;
                                }
                                else
                                {
                                    if (!g_41b85c)
                                        sub_40f630(stderr, "%s: %s: MTIME %lu out of range for this platform\n", g_41b850, &g_41b420, v41);
                                    if (!g_41b84c)
                                        g_41b84c = 2;
                                    g_41b838 = 0x3b9ac9ff;
                                    g_41b830 = 0x7fffffffffffffff;
                                }
                            }
                            v42 = g_41b008;
                            v43 = g_41b00c;
                            if ((unsigned int)v42 < g_41b00c)
                            {
                                v44 = (unsigned int)v42 + 1;
                                g_41b008 = v44;
                                v45 = (&g_49d000)[v42];
                            }
                            else
                            {
                                v45 = sub_40d830(0);
                                v44 = g_41b008;
                                v43 = g_41b00c;
                            }
                            v8 = v45;
                            if (v43 > (unsigned int)v44)
                            {
                                g_41b008 = (unsigned int)v44 + 1;
                                v46 = (&g_49d000)[v44];
                            }
                            else
                            {
                                v46 = sub_40d830(0);
                            }
                            v47 = v25;
                            v9 = v46;
                            v48 = (char)v47 & 2;
                            if ((char)v47 & 2)
                            {
                                v5 = 8;
                                v6 = v25;
                                v7 = v41;
                                sub_40d1f0(0, 0);
                                sub_40d1f0(&v3, 10);
                            }
                            if ((char)v25 & 4)
                            {
                                v49 = g_41b008;
                                v50 = g_41b00c;
                                if ((unsigned int)v49 < g_41b00c)
                                {
                                    v51 = (unsigned int)v49 + 1;
                                    g_41b008 = v51;
                                    v52 = (&g_49d000)[v49];
                                }
                                else
                                {
                                    v52 = sub_40d830(0);
                                    v51 = g_41b008;
                                    v50 = g_41b00c;
                                }
                                v1 = v52;
                                if (v50 > (unsigned int)v51)
                                {
                                    g_41b008 = (unsigned int)v51 + 1;
                                    v53 = (&g_49d000)[v51];
                                }
                                else
                                {
                                    v53 = sub_40d830(0);
                                }
                                v2 = v53;
                                v54 = v53 * 0x100 | v52;
                                if (g_4dd9e4)
                                    sub_40f630(stderr, "%s: %s: extra field of %u bytes ignored\n", g_41b850, &g_41b420, v54);
                                if (v48)
                                    sub_40d1f0(&v1, 2);
                                sub_405a90(v54, (char)v25);
                            }
                            if ((char)v25 & 8)
                            {
                                if (!g_41902c && (!g_41b860 || g_4dd9e8) && g_4dd9d8 <= 1)
                                {
                                    v0 = sub_40d3b0(&g_41b020);
                                    iter = v0;
                                    while (1)
                                    {
                                        v56 = g_41b008;
                                        if ((unsigned int)v56 < g_41b00c)
                                        {
                                            v57 = (&g_49d000)[v56];
                                            g_41b008 = (unsigned int)v56 + 1;
                                        }
                                        else
                                        {
                                            v57 = sub_40d830(0);
                                        }
                                        *(iter) = v57;
                                        iter += 1;
                                        if (!v57)
                                            break;
                                        if (iter >= &g_41b420)
                                            sub_40d510("corrupted input -- file name too large"); /* do not return */
                                    }
                                    if (v48)
                                        sub_40d1f0(v0, (iter & 0xffffffff) - v0);
                                    ptr = sub_40d3b0(v0);
                                    memmove(v0, ptr, strlen(ptr) + 1);
                                }
                                else
                                {
                                    sub_405a90(0xffffffffffffffff, (char)v25);
                                }
                            }
                            if ((char)v25 & 16)
                                sub_405a90(0xffffffffffffffff, (char)v25);
                            if (!v48)
                            {
LABEL_406049:
                                if (g_4dd9d8 == 1)
                                {
                                    g_4e0ea8 = g_41b008 + 8;
                                    if (g_419020 >= 0)
                                        return g_419020;
                                    goto LABEL_4066ac;
                                }
                                else if (g_419020 >= 0)
                                {
                                    return g_419020;
                                }
                                goto LABEL_406060;
                            }
                            else
                            {
                                v60 = sub_40d1f0(&v3, 0);
                                v61 = g_41b00c;
                                v62 = v60;
                                v63 = g_41b008;
                                if ((unsigned int)v63 < g_41b00c)
                                {
                                    v64 = (unsigned int)v63 + 1;
                                    g_41b008 = v64;
                                    v65 = (&g_49d000)[v63];
                                }
                                else
                                {
                                    v66 = sub_40d830(0);
                                    v64 = g_41b008;
                                    v61 = g_41b00c;
                                    v65 = v66;
                                }
                                if ((unsigned int)v64 < v61)
                                {
                                    g_41b008 = (unsigned int)v64 + 1;
                                    v67 = (&g_49d000)[v64] * 0x100;
                                }
                                else
                                {
                                    v67 = sub_40d830(0) * 0x100;
                                }
                                v68 = v67 | v65;
                                if (v62 == v68 || !(sub_40f630(stderr, "%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n", g_41b850, &g_41b420, (unsigned long long)v68, (unsigned long long)(v62 & 0xffff)), v69 = (unsigned long)(unsigned long long)g_4dd9f4, g_41b84c = 1, (unsigned int)v69 <= 1))
                                    goto LABEL_406049;
                            }
                        }
                    }
                }
            }
        }
        return 0xffffffff;
    }
    if (memcmp(&v3, &g_412cd7, 2) || g_41b008 != 2 || memcmp(&g_49d000, &g_412cd7, 4))
    {
        if (!memcmp(&v3, &g_412cdc, 2))
        {
            g_419020 = 2;
            g_4190d0 = sub_40c9f0;
            return 2;
        }
        if (!memcmp(&v3, &g_412cdf, 2))
        {
            g_419020 = 1;
            g_4dd9dc = 1;
            g_4190d0 = sub_40c040;
            return 1;
        }
        if (!memcmp(&v3, &g_412ce2, 2))
        {
            g_419020 = 3;
            g_4dd9dc = 1;
            g_4190d0 = sub_40bfd0;
            return 3;
        }
        if (!g_4dd9f4 || !g_41b860 || g_4dd9e8)
            goto LABEL_40668e;
        g_419020 = 0;
        g_4190d0 = sub_40d720;
        if (i != 0xffffffff)
            g_41b008 = g_41b008 - 1;
        g_4dd9dc = 1;
        if (v14 == 0xffffffff)
            return g_4dd9e8;
        sub_40d6c0(1, &v3, 1);
    }
    else
    {
        g_41b008 = 0;
        g_4190d0 = sub_40cd20;
        if (sub_40cc20(a0))
            return 0xffffffff;
        g_4dd9dc = 1;
    }
    if (g_419020 >= 0)
        return g_419020;
LABEL_40668e:
    __unsupported_jumpkind_Ijk_NoDecode()
    *(v20) = *(v20) + v21;
LABEL_406060:
    if (!v3)
    {
        if (!i)
        {
            do
            {
                if (g_41b008 < g_41b00c)
                {
                    i = (&g_49d000)[g_41b008];
                    g_41b008 = g_41b008 + 1;
                }
                else
                {
                    i = sub_40d830(1);
                }
            } while (!i);
        }
        if (i == 0xffffffff)
        {
            if (!g_4dd9e4)
                return 0xfffffffd;
            if (!g_41b85c)
                sub_40f630(stderr, "\n%s: %s: decompression OK, trailing zero bytes ignored\n", g_41b850, &g_41b420);
            if (!g_41b84c)
                g_41b84c = 2;
            return 0xfffffffd;
        }
    }
    if (!g_41b85c)
        sub_40f630(stderr, "\n%s: %s: decompression OK, trailing garbage ignored\n", g_41b850, &g_41b420);
    if (!g_41b84c)
        g_41b84c = 2;
    return 0xfffffffe;
}



// Function: get_suffix @ 0x67e0
extern void g_419040;
extern void g_419048;
extern char g_4dd9c0;
extern void g_4dd9c8;

long long get_suffix(unsigned long a0)
{
    unsigned long v4;  // r13
    void* v5;  // rbx
    unsigned int v14;  // eax
    void* v6;  // rbx
    unsigned long len;  // rax
    void* iter;  // rbx
    unsigned long ptr;  // rax
    unsigned long long idx;  // rdx
    unsigned long v11;  // r15
    unsigned int v12;  // eax
    unsigned long v13;  // r15
    unsigned long v0;  // [bp-0x80]
    unsigned long len1;  // [bp-0x70]
    char v2[40];  // [bp-0x68]

    v4 = *((long long *)&g_419048);
    if (*((long long *)&g_419048))
    {
        v5 = &g_419048;
        do
        {
            v6 = v5;
            len = strlen(v4);
            if (*((long long *)&g_4dd9c0) < len && !strcmp(*((long long *)&g_4dd9c8), v4 + len - *((long long *)&g_4dd9c0)))
            {
                iter = &g_419048;
                sub_40d350(sub_40f5d0(*((long long *)&g_4dd9c8)));
                idx = 8;
                goto LABEL_406879;
            }
        } while ((v4 = (unsigned long)(long long)v6[8], v5 = v6 + 8, (long long)v6[8]));
    }
    ptr = sub_40f5d0(*((long long *)&g_4dd9c8));
    sub_40d350(ptr);
    idx = 0;
    iter = &g_419040;
LABEL_406879:
    *((unsigned long *)&(&g_419040)[8 * idx]) = ptr;
    len1 = strlen(v11);
    if ((unsigned int)len1 <= 32)
        __strcpy_chk(v2, v11, 33);
    else
        __strcpy_chk(v2, a0 + (int)len1 - 32, 33, a0);
    sub_40d350(v2);
    v12 = strlen(v2);
    v13 = *((long long *)iter);
    v0 = v12;
    while (1)
    {
        v14 = strlen(v13);
        if ((unsigned int)v0 > v14 && *(&v2[v0 + -1 * v14] - 1) != 47 && !strcmp(&v2[v0 + -1 * v14], v13))
        {
            free(ptr);
            return a0 + (int)len1 - v14;
        }
        v13 = (long long)iter[8];
        iter += 8;
        if (!v13)
        {
            free(ptr);
            return v13;
        }
    }
}



// Function: make_ofname @ 0x69d0
extern long long stderr;
extern char g_41b020;
extern long long g_41b420;
extern unsigned int g_41b848;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_41b858;
extern unsigned int g_41b85c;
extern unsigned int g_4dd9e4;
extern unsigned int g_4dd9ec;
extern unsigned int g_4dd9f4;
extern unsigned int g_4dd9f8;

unsigned int make_ofname(void)
{
    void* v1;  // rbx

    __strcpy_chk(&g_41b020, &g_41b420, 0x400);
    v1 = sub_4067e0(&g_41b020);
    if (g_4dd9f8)
    {
        if (v1)
        {
            sub_40d350(v1);
            if (strcmp(v1, ".tgz") && strcmp(v1, ".taz"))
            {
                *((char *)v1) = 0;
                return 0;
            }
            *((unsigned int *)v1) = 1918989358;
            *((char *)&v1[4]) = 0;
            return 0;
        }
        if (!g_4dd9ec)
        {
            if (g_41b858)
                return g_4dd9ec;
            if (g_4dd9e4)
                goto LABEL_406a8a;
            if (!g_41b85c)
            {
LABEL_406c10:
                sub_40f630(stderr, "%s: %s: unknown suffix -- ignored\n", g_41b850, &g_41b420);
LABEL_406aa0:
                if (!g_41b84c)
                {
                    g_41b84c = 2;
                    return 2;
                }
            }
        }
        else if (g_4dd9e4)
        {
LABEL_406a8a:
            if (!g_41b85c)
                goto LABEL_406c10;
            goto LABEL_406aa0;
        }
        goto LABEL_406ab0;
    }
    if (v1 && !g_4dd9f4)
    {
        if (g_4dd9e4 || !g_4dd9ec && !g_41b85c)
            sub_40f630(stderr, "%s: %s already has %s suffix -- unchanged\n", g_41b850, &g_41b420, v1);
LABEL_406ab0:
        return 2;
    }
    g_41b848 = 0;
    strlen(&g_41b020);
    __unsupported_jumpkind_Ijk_NoDecode()
    __debugbreak();
}



// Function: do_list @ 0x6c40
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

extern char g_0;
extern unsigned int g_419010;
extern long long stdout;
extern long long g_41b820;
extern long long g_41b828;
extern int g_41b830;
extern char g_41b85c;
extern unsigned long long g_4dd500;
extern unsigned long long g_4dd508;
extern char g_4dd9e4;
extern unsigned long long g_4e0ea8;

int do_list(void)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    long long v10;  // rsi
    int v11;  // edi
    long long v12;  // rsi
    tm *tm;  // rbp
    char v6;  // cl
    unsigned long v7;  // cc_ndep
    unsigned long long v8;  // 4108
    long long v9;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    if (g_419010)
    {
        if (v11 >= 0)
        {
            g_419010 = 0;
            if (*((int *)&g_4dd9e4))
                sub_40f7b0("method  crc     date  time  ");
            if (!*((int *)&g_41b85c))
                sub_40f7b0("%*.*s %*.*s  ratio uncompressed_name\n", 19, 19, "compressed", 19, 19, "uncompressed");
        }
        else
        {
LABEL_406d88:
            v12 = g_4dd508;
            if (g_4dd508 <= 0)
            {
                return;
            }
            else if (g_4dd500 > 0)
            {
                if (!*((int *)&g_4dd9e4) || (sub_40f7b0("                            "), v12 = (long long)g_4dd508, !*((int *)&g_4dd9e4)))
                {
                    if (*((int *)&g_41b85c))
                    {
                        sub_40d8e0(g_4dd500 - (v12 - g_4e0ea8), g_4dd500, stdout);
                        sub_40f7b0();
                        return;
                    }
                }
                sub_40d930(stdout);
                sub_40f7b0(" ");
                sub_40d930(stdout, g_4dd500, 19);
                sub_40f7b0(" ");
                sub_40d8e0(g_4dd500 - (g_4dd508 - g_4e0ea8), g_4dd500, stdout);
                sub_40f7b0();
                return;
            }
            else
            {
                return;
            }
        }
    }
    else
    {
        if (v11 < 0)
            goto LABEL_406d88;
    }
    if (*((int *)&g_4dd9e4))
    {
        tm = localtime(&g_41b830);
        g_0 = 0;
        if (tm)
        {
            v6 = tm->tm_hour;
            g_0 = 0;
            v8 = _ccall(12, 1, (unsigned long long)v6, (unsigned long long)v6, v7);
            if (v8 & 1)
                goto LABEL_0x406f26;
            else
                goto LABEL_0x406f23;
        }
        else
        {
            sub_40f7b0("??? ?? ??:?? ");
        }
    }
    sub_40d930(stdout, g_41b828, 19);
    sub_40f7b0(" ");
    sub_40d930(stdout, g_41b820, 19);
    sub_40f7b0(" ");
    v9 = g_41b828;
    if (v9 != -0x1)
    {
        if (g_4dd508 >= 0)
            g_4dd508 = g_4dd508 + v9;
        v10 = g_41b820;
        if (v10 == -0x1)
        {
            g_4dd500 = 0xffffffffffffffff;
            g_4e0ea8 = 0;
            g_41b820 = 0;
            g_41b828 = 0;
            sub_40d8e0(0, 0, stdout);
            sub_40f7b0();
            return;
        }
    }
    else
    {
        g_4dd508 = 0xffffffffffffffff;
        v10 = 0;
        v9 = 0;
        g_4e0ea8 = 0;
        g_41b820 = 0;
        g_41b828 = 0;
    }
    if (g_4dd500 >= 0)
    {
        __unsupported_jumpkind_Ijk_NoDecode()
        return;
    }
    sub_40d8e0(v10 - (v9 - g_4e0ea8), v10, stdout);
    sub_40f7b0();
    return;
}



// Function: do_exit @ 0x6fd0
extern char g_0;
extern unsigned int g_4dd040;
extern void* g_4dd9d0;

void do_exit(int a0)
{
    if (!g_4dd040)
    {
        g_4dd040 = 1;
        free(g_4dd9d0);
        g_4dd9d0 = 0;
    }
    exit(a0); /* do not return */
}



// Function: finish_out @ 0x7010
extern char g_0;
extern long long stdout;

long long finish_out(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (!sub_40e0f0(stdout))
        sub_406fd0(0); /* do not return */
    sub_40d670(); /* do not return */
}



// Function: try_help @ 0x7040
extern long long stderr;
extern long long g_41b850;

void try_help(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f630(stderr, "Try `%s --help' for more information.\n", g_41b850);
    sub_406fd0(1); /* do not return */
}



// Function: treat_stdin @ 0x7070
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

extern char g_0;
extern unsigned int g_419020;
extern long long stderr;
extern unsigned int g_41b014;
extern unsigned int g_41b020;
extern unsigned short g_41b024;
extern char g_41b026;
extern unsigned int g_41b420;
extern unsigned short g_41b424;
extern unsigned long long g_41b820;
extern long long g_41b828;
extern int g_41b84c;
extern unsigned int g_41b858;
extern unsigned int g_41b85c;
extern unsigned int g_41b860;
extern stat g_4dd060;
extern char g_4dd510;
extern unsigned int g_4dd9d8;
extern unsigned int g_4dd9e4;
extern unsigned int g_4dd9e8;
extern unsigned int g_4dd9f4;
extern unsigned int g_4dd9f8;
extern char g_4dda01;
extern unsigned long long g_4e0ea8;

long long treat_stdin(void)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    long long v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9
    unsigned long v7;  // rax
    unsigned long v8;  // rax
    char *v9;  // rax

    if (g_4dd9f4 || g_4dd9e8 || !g_4dda01 && !isatty(!g_4dd9f8))
    {
        g_41b420 = 1768191091;
        g_41b424 = 110;
        g_41b020 = 1868854387;
        g_41b024 = 29813;
        g_41b026 = 0;
        if (!fstat(0, &g_4dd060.st_mode))
        {
            sub_4053f0();
            sub_40d270(v1, v2, v3, v4, v5, v6);
            g_41b860 = 1;
            g_4dd9d8 = 0;
            g_41b014 = 0;
            g_4dd510 = 1;
            if (g_4dd9f8)
            {
                g_419020 = sub_405b50(0);
                if (g_419020 < 0)
                    sub_406fd0(g_41b84c); /* do not return */
            }
            while (1)
            {
                v7 = sub_40da60(0, 1);
                if ((unsigned int)v7)
                    return v7;
                if (sub_405940())
                    break;
                g_419020 = sub_405b50(g_41b014);
                if (g_419020 < 0)
                    return g_419020;
                __unsupported_jumpkind_Ijk_NoDecode()
                v9 = g_419020 + 83639;
                *(v9) = *(v9) + *((char *)&v9);
                *(v9) = *(v9) + *((char *)&v9);
            }
            if (g_4dd9e8)
                return sub_406c40(g_419020);
            if (!g_4dd9e4)
                return v8;
            if (!g_41b858)
            {
                if (g_4dd9f8)
                    return g_4dd9f8;
                sub_40d8e0(g_41b828 - g_41b820 + g_4e0ea8, g_41b828, stderr);
                return sub_40f630();
            }
            else
            {
                return sub_40f630();
            }
        }
        sub_405820("standard input");
    }
    else if (!g_41b85c)
    {
        sub_40f630();
    }
    sub_406fd0(1); /* do not return */
}



// Function: create_outfile @ 0x76e0
extern char g_0;
extern long long stderr;
extern int g_41b010;
extern int g_41b014;
extern char g_41b020;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_41b85c;
extern unsigned int g_4dd520;
extern int g_4dd940;
extern unsigned int g_4dd9f0;
extern unsigned int g_4dd9f8;

unsigned long long create_outfile(void)
{
    unsigned long v5;  // rax
    unsigned int v6;  // eax
    char *v7;  // rax
    char *idx;  // rcx
    int v9;  // edi
    unsigned long long v10;  // rax
    int *err;  // [bp-0xd8]
    unsigned int v1;  // [bp-0xd0]
    unsigned int v2;  // [bp-0xcc]
    int v3[34];  // [bp-0xc8]

    if (!g_4dd9f0)
    {
        v5 = sub_40e060(&g_41b020);
        v6 = sub_405a00(&g_41b020, v5 - &g_41b020);
        if (v6 >= 0)
            goto LABEL_407726;
    }
    v6 = 0xffffff9c;
    v5 = &g_41b020;
LABEL_407726:
    v2 = 0;
    err = __errno_location();
    while (1)
    {
        sub_405470(&g_4dd520, &g_41b020);
        sigprocmask(0, &g_4dd940, v3);
        v7 = sub_40e520(v6, v5, 193, 384);
        *(v7) = *(v7) + *((char *)&v7);
        idx[20481797] = idx[20481797] + *((char *)&idx);
        idx[18385157] = idx[18385157] + *((char *)&idx);
        v1 = *((int *)*((unsigned int *)&err));
        sigprocmask(v9, v3, NULL);
        if (g_41b010 >= 0)
            break;
        switch (v1)
        {
        case 17:
            if (sub_405870())
                break;
            continue;
        case 36:
            sub_407550();
            v2 = 1;
            continue;
        default:
            sub_405820(&g_41b020);
            break;
            close(g_41b014);
            return 1;
        }
    }
    if (v2 && g_4dd9f8)
    {
        if (!g_41b85c)
            sub_40f630(stderr, "%s: %s: warning, name truncated\n", g_41b850, &g_41b020);
        v10 = g_41b84c;
        if (!(unsigned int)v10)
        {
            g_41b84c = 2;
            return v10;
        }
    }
    return 0;
}



// Function: treat_file @ 0x7b60
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

extern char g_0;
extern void g_419014;
extern unsigned int g_419018;
extern void g_419020;
extern unsigned int g_41902c;
extern long long stderr;
extern void g_41b010;
extern void g_41b014;
extern unsigned int g_41b020;
extern unsigned short g_41b024;
extern char g_41b026;
extern char g_41b420;
extern long long g_41b820;
extern unsigned long long g_41b828;
extern unsigned int g_41b848;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern char g_41b858;
extern char g_41b85c;
extern unsigned int g_41b860;
extern unsigned long long g_4dd070;
extern unsigned int g_4dd078;
extern int g_4dd940;
extern unsigned int g_4dd9d8;
extern unsigned int g_4dd9e4;
extern unsigned int g_4dd9e8;
extern unsigned int g_4dd9ec;
extern unsigned int g_4dd9f0;
extern unsigned int g_4dd9f4;
extern char g_4dd9f8;
extern char g_4dda00;
extern unsigned long long g_4e0ea8;

long long treat_file(unsigned long a0)
{
    unsigned long long v16;  // r14
    unsigned long long v17;  // r13
    unsigned int v26;  // rsi
    unsigned int v27;  // rdx
    char v28;  // cl
    unsigned long v29;  // fs
    int v30;  // edi
    unsigned long long v31;  // r15
    unsigned long long v32;  // rbx
    unsigned long fp;  // rax
    unsigned long v34;  // r12
    unsigned long long v18;  // r12
    struct_0 *v35;  // r14
    unsigned long long v36;  // rbx
    unsigned long v37;  // rcx
    unsigned int v38;  // eax
    unsigned short v39;  // dx
    char *v40;  // rdi
    struct_0 *v41;  // r14
    unsigned long v42;  // 4110
    unsigned long v43;  // fs
    unsigned int v19;  // eax
    unsigned int v20;  // eax
    long long v21;  // rax
    unsigned int v22;  // eax
    unsigned int *err;  // rax
    unsigned int v24;  // r14d
    unsigned int v25;  // eax
    unsigned long len;  // [bp-0x458]
    struct_0 *v1;  // [bp-0x450]
    char v2;  // [bp-0x448]
    int v3[30];  // [bp-0xb8]
    unsigned long v4;  // [bp-0x40]
    unsigned long v5;  // [bp-0x30], Other Possible Types: unsigned long long
    unsigned long long v6;  // [bp-0x28]
    unsigned long long v7;  // [bp-0x20]
    unsigned long long v8;  // [bp-0x20]
    unsigned long long v9;  // [bp-0x18]
    unsigned long long v10;  // [bp-0x18]
    unsigned long long v11;  // [bp-0x10]
    unsigned long long v12;  // [bp-0x10]
    unsigned long long v13;  // [bp-0x8]
    unsigned long long v14;  // [bp-0x8]

    v13 = v16;
    v11 = v17;
    v9 = v18;
    if (strcmp(a0, "-"))
    {
        *((unsigned int *)&g_41b014) = sub_407380(a0);
        if (*((int *)&g_41b014) >= 0)
        {
            v19 = g_4dd078 & 0xf000;
            if (v19 != 0x4000)
            {
                if (!g_41b860)
                {
                    if (v19 != 0x8000)
                    {
                        if (!*((int *)&g_41b85c))
                            sub_40f630(stderr, "%s: %s is not a directory or a regular file - ignored\n", g_41b850, &g_41b420);
                    }
                    else
                    {
                        if (!((char)g_4dd078 & 8))
                        {
                            if (!((char)g_4dd078 & 4))
                            {
                                if (!g_4dd9f4)
                                {
                                    if (!((char)g_4dd078 & 2))
                                    {
                                        if (g_4dd070 <= 1)
                                            goto LABEL_407ca8;
                                        if (!*((int *)&g_41b85c))
                                            sub_40f630();
                                    }
                                    else
                                    {
                                        if (!*((int *)&g_41b85c))
                                        {
                                            sub_40f630(stderr, "%s: %s has the sticky bit set - file ignored\n", g_41b850, &g_41b420);
                                            __unsupported_jumpkind_Ijk_NoDecode()
                                        }
                                    }
                                }
                                else
                                {
LABEL_407ca8:
                                    sub_4053f0();
                                    goto LABEL_407d03;
                                }
                            }
                            else
                            {
                                if (!*((int *)&g_41b85c))
                                    sub_40f630(stderr, "%s: %s is set-group-ID on execution - ignored\n", g_41b850, &g_41b420);
                            }
                        }
                        else
                        {
                            if (!*((int *)&g_41b85c))
                                sub_40f630(stderr, "%s: %s is set-user-ID on execution - ignored\n", g_41b850, &g_41b420);
                        }
                    }
                    if (g_41b84c)
                        return close(v30);
                    g_41b84c = 2;
                    return close(v30);
                }
                sub_4053f0();
                if (*((int *)&g_41b858))
                {
LABEL_407d03:
                    if (sub_4069d0())
                        goto LABEL_407f99;
                    goto LABEL_407d10;
                }
                g_41b026 = 0;
                g_41b020 = 1868854387;
                g_41b024 = 29813;
LABEL_407d10:
                sub_40d270();
                g_4dd9d8 = 0;
                if (*((int *)&g_4dd9f8) && (*((unsigned int *)&g_419020) = (unsigned int)sub_405b50(*((int *)&g_41b014)), *((int *)&g_419020) < 0))
                {
LABEL_407f99:
                    if (v5 != *((long long *)(40 + v29)))
                        __stack_chk_fail(); /* do not return */
                    return close(v30);
                }
                if (g_41b860)
                {
                    *((unsigned int *)&g_41b010) = 1;
                    v20 = g_4dd9e4;
                    goto LABEL_407d4d;
                }
                else
                {
                    if (!(unsigned int)sub_4076e0())
                    {
                        v20 = g_4dd9e4;
                        if (*((int *)&g_4dd9f8))
                            goto LABEL_407d4d;
                        if (!g_41b848)
                        {
LABEL_407f20:
                            g_41b848 = !g_41902c;
LABEL_407d5b:
                            if (v20 && !g_4dd9e8)
                                sub_40f630(stderr, "%s:\t", &g_41b420);
                            while (1)
                            {
                                if (!sub_40da60(*((int *)&g_41b014), *((int *)&g_41b010)))
                                {
                                    if (sub_405940() || !(*((unsigned int *)&g_419020) = (unsigned int)sub_405b50(*((int *)&g_41b014)), *((int *)&g_419020) >= 0))
                                        break;
                                    g_41b820 = 0;
                                }
                                else
                                {
                                    *((unsigned int *)&g_419020) = 0xffffffff;
                                    break;
                                }
                            }
                            if (close(*((int *)&g_41b014)))
                                sub_40d600(); /* do not return */
                            if (g_4dd9e8)
                                return sub_406c40(*((int *)&g_419020));
                            if (g_41b860)
                            {
                                if (*((int *)&g_419020) != 0xffffffff)
                                    goto LABEL_407e15;
                            }
                            else
                            {
                                sub_4078e0();
                                if (g_4dda00 && (*((int *)&g_419014) >= 0 && fdatasync(*((int *)&g_419014)) && *(__errno_location()) != 22 || fsync(*((int *)&g_41b010)) && *(__errno_location()) != 22) || close(*((int *)&g_41b010)))
                                    sub_40d670(); /* do not return */
                                if (!g_4dd9f0)
                                {
                                    v21 = sub_40e060(&g_41b420);
                                    if (!(char)sub_4059a0(&g_41b420, v21 - &g_41b420))
                                    {
                                        sigprocmask(0, &g_4dd940, v3);
                                        g_419018 = 0xffffffff;
LABEL_4081b1:
                                        v22 = sub_40d3c0(&g_41b420);
                                    }
                                    else
                                    {
                                        sigprocmask(0, &g_4dd940, v3);
                                        g_419018 = 0xffffffff;
                                        if (*((int *)&g_419014) < 0)
                                            goto LABEL_4081b1;
                                        v22 = unlinkat(*((int *)&g_419014), v21, 0);
                                    }
                                    if (!v22)
                                    {
                                        sigprocmask(2, v3, NULL);
                                    }
                                    else
                                    {
                                        err = __errno_location();
                                        v24 = *(err);
                                        sigprocmask(2, v3, NULL);
                                        if (v24)
                                        {
                                            v25 = *((int *)&g_41b85c);
                                            if (*((int *)&g_41b85c))
                                            {
                                                if (g_41b84c)
                                                    goto LABEL_4081d0;
                                            }
                                            else
                                            {
                                                sub_40f630(stderr, "%s: ", g_41b850);
                                                v25 = *((int *)&g_41b85c);
                                                if (g_41b84c)
                                                    goto LABEL_408357;
                                            }
                                            g_41b84c = 2;
LABEL_408357:
                                            if (!v25)
                                            {
                                                *(err) = v24;
                                                perror(&g_41b420);
                                            }
                                        }
                                    }
                                }
LABEL_4081d0:
                                if (*((int *)&g_419020) != 0xffffffff)
                                {
LABEL_407e15:
                                    if (g_4dd9e4)
                                    {
                                        if (!*((int *)&g_41b858))
                                        {
                                            if (*((int *)&g_4dd9f8))
                                                sub_40d8e0(g_41b820 - (g_41b828 - g_4e0ea8), g_41b820);
                                            else
                                                sub_40d8e0(g_41b828 - (g_41b820 - g_4e0ea8));
                                        }
                                        else
                                        {
                                            sub_40f630(stderr, " OK");
                                        }
                                        if (!*((int *)&g_41b858))
                                            sub_40f630();
                                        return sub_40f630();
                                    }
                                }
                                else
                                {
                                    if (!g_41b860)
                                        return (unsigned long long)sub_4056e0(0, v26, v27, v28);
                                }
                            }
                        }
                        else
                        {
                            if (g_4dd9e4 || *((int *)&g_41b85c))
                                goto LABEL_407d5b;
                            sub_40f630(stderr, "%s: %s compressed to %s\n", g_41b850, &g_41b420, &g_41b020);
                            v20 = g_4dd9e4;
LABEL_407d4d:
                            if (!g_41b848)
                                goto LABEL_407f20;
                            goto LABEL_407d5b;
                        }
                    }
                }
            }
            else if (!g_4dd9ec)
            {
                close(*((int *)&g_41b014));
                if (!*((int *)&g_41b85c))
                    sub_40f630(stderr, "%s: %s is a directory -- ignored\n", g_41b850, &g_41b420);
                if (!g_41b84c)
                    g_41b84c = 2;
            }
            else if (v5 == *((long long *)(40 + v29)))
            {
                v14 = v31;
                v12 = v13;
                v10 = v11;
                v8 = v9;
                v6 = v7;
                v5 = v32;
                v4 = *((long long *)(40 + v29));
                fp = fdopendir(*((int *)&g_41b014));
                if (!fp)
                {
                    sub_405820(a0);
                    return close(*((int *)&g_41b014));
                }
                v34 = fp;
                v1 = sub_40e5b0(fp, 0);
                if (v1)
                {
                    if (closedir(v34))
                        sub_405820(a0);
                    v35 = v1;
                    if (v1->field_0)
                    {
                        while (1)
                        {
                            v36 = strlen(v35) & 0xffffffff;
                            if (!strcmp(v35, ".") || !strcmp(v35, ".."))
                            {
LABEL_4084d6:
                                v35 = &(&v35->field_0)[v36 + 1];
                                if (!v35->field_0)
                                    break;
                            }
                            else
                            {
                                len = strlen(a0);
                                if (len + v36 <= 0x3fd)
                                {
                                    __strcpy_chk(&v2, a0, 0x400);
                                    v37 = len;
                                    if (*((char *)sub_40e060(&v2)) && *((char *)&v1 + v37 - 7) != 47)
                                    {
                                        (&v2)[v37] = 47;
                                        v37 += 1;
                                    }
                                    v38 = (unsigned int)strcpy(&(&v2)[v37], v35);
                                    __unsupported_jumpkind_Ijk_NoDecode()
                                    __outdword(v39, v38);
                                    treat_file(v40);
                                    goto LABEL_4084d6;
                                }
                                else
                                {
                                    v41 = &(&v35->field_0)[v36 + 1];
                                    sub_40f630(stderr, "%s: %s/%s: pathname too long\n", g_41b850, a0, v35);
                                    v42 = v41->field_0;
                                    g_41b84c = 1;
                                    v35 = v41;
                                    if (!(char)v42)
                                        break;
                                }
                            }
                        }
                    }
                    free(v1);
                }
                else
                {
                    sub_405820(a0);
                    if (closedir(v34))
                        return (unsigned long long)sub_405820(a0);
                }
                return v4 - *((long long *)(40 + v29));
            }
            else
            {
                __stack_chk_fail(); /* do not return */
            }
        }
    }
    else
    {
        sub_407070();
        g_41b860 = g_41b860;
    }
    return v5 - *((long long *)(40 + v43));
}



// Function: abort_gzip @ 0x8620
extern char g_0;

void abort_gzip(unsigned long a0, unsigned int a1, unsigned int a2, char a3)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_4056e0(0, a1, a2, a3);
    sub_406fd0(1); /* do not return */
}



// Function: inflate_codes @ 0x8840
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_413380;
extern unsigned int g_41b004;
extern unsigned int g_41b008;
extern char g_41b00c;
extern char g_43c000;
extern char g_49d000;
extern unsigned int g_4dda0c;
extern unsigned long long g_4dda10;

unsigned int inflate_codes(unsigned long a0, unsigned long a1, unsigned int a2, int a3)
{
    unsigned long v7;  // r14
    unsigned long iter;  // r13
    unsigned long v17;  // rax
    void* v18;  // rax
    unsigned long v19;  // rax
    unsigned int v20;  // r15d
    unsigned long v21;  // rax
    unsigned long v22;  // rax
    unsigned long v23;  // r13
    unsigned long long v24;  // r15
    unsigned long long v25;  // rbx
    unsigned long long v26;  // r15
    unsigned long v9;  // r12
    unsigned long v27;  // rax
    unsigned long v28;  // rax
    void* v29;  // r13
    unsigned int i;  // r8d
    unsigned long v31;  // rax
    int v32;  // r14d
    unsigned long long v33;  // r15
    unsigned long v34;  // rax
    unsigned long v35;  // rax
    void* v36;  // rax
    unsigned long v10;  // r12
    unsigned int v37;  // eax
    unsigned int v38;  // edx
    unsigned long long v39;  // rsi
    unsigned long long v40;  // r15
    unsigned int v41;  // r13d
    unsigned long long v42;  // rbx
    unsigned long v43;  // rax
    unsigned long v44;  // rax
    unsigned long long v45;  // r13
    unsigned int j;  // r14d
    void* idx;  // rbx
    unsigned long v47;  // r12
    unsigned long long v48;  // rbx
    unsigned int v49;  // r15d
    unsigned long long v50;  // r15
    unsigned long long v51;  // rsi
    unsigned long long v52;  // rax
    unsigned long v54;  // r13
    unsigned long v55;  // rax
    unsigned long v56;  // rax
    unsigned long v12;  // r8
    unsigned long v13;  // rax
    unsigned long long v14;  // r15
    unsigned long v15;  // r13
    unsigned long v16;  // rax
    unsigned int v0;  // [bp-0x60]
    unsigned short v1;  // [bp-0x5c]
    unsigned int v2;  // [bp-0x5c]
    unsigned long v3;  // [bp-0x58], Other Possible Types: unsigned short
    unsigned int v4;  // [bp-0x48]
    void* v5;  // [bp-0x40], Other Possible Types: unsigned long

    v7 = g_4dda10;
    iter = g_4dda0c;
    v9 = g_41b004;
    v0 = (&g_413380.field_0)[a2];
    v4 = (&g_413380.field_0)[a3];
    while (1)
    {
LABEL_4088a0:
        if ((unsigned int)iter >= a2)
        {
            do
            {
                idx = a0 + (v0 & (unsigned int)v7) * 16;
                v12 = *((char *)idx);
                if ((unsigned int)v12 > 16)
                {
                    do
                    {
                        if ((unsigned int)v12 == 99)
                            return 1;
                        v13 = (char)idx[1];
                        v14 = v12 - 16 & 0xffffffff;
                        iter -= v13;
                        v7 >>= (char)v13 & 63;
                        if ((unsigned int)iter < (unsigned int)v14)
                        {
                            do
                            {
                                v15 = iter;
                                v16 = g_41b008;
                                if ((unsigned int)v16 < *((int *)&g_41b00c))
                                {
                                    v17 = (&g_49d000)[v16];
                                    g_41b008 = (unsigned int)v16 + 1;
                                }
                                else
                                {
                                    g_41b004 = v10;
                                    v17 = sub_40d830(0) & 0xff;
                                }
                            } while ((iter = (unsigned long)(unsigned long long)((unsigned int)v15 + 8), v7 |= v17 << ((char)(v15 & 0xffffffff) & 63), (unsigned int)v14 > (unsigned int)iter));
                        }
                        v18 = ((&g_413380.field_0)[v14] & (unsigned short)v7) * 16 + (long long)idx[8];
                        v12 = *((char *)v18);
                        idx = v18;
                    } while ((unsigned int)v12 > 16);
                }
                v19 = (char)idx[1];
                iter -= v19;
                v7 >>= (char)v19 & 63;
                if ((unsigned int)v12 == 16)
                {
                    v9 = (unsigned int)v10 + 1;
                    (&g_43c000)[v10 & 0xffffffff] = (short)idx[8];
                    if ((unsigned int)v9 != 0x8000)
                        goto LABEL_4088a0;
                }
                else
                {
                    continue;
                    if ((unsigned int)v12 == 15)
                    {
                        g_41b004 = v10;
                        g_4dda10 = v7;
                        g_4dda0c = iter;
                        return 0;
                    }
                    if ((unsigned int)v12 > (unsigned int)iter)
                    {
                        v20 = v12;
                        do
                        {
                            v21 = g_41b008;
                            if ((unsigned int)v21 < *((int *)&g_41b00c))
                            {
                                v22 = (&g_49d000)[v21];
                                g_41b008 = (unsigned int)v21 + 1;
                            }
                            else
                            {
                                g_41b004 = v10;
                                v22 = sub_40d830(0) & 0xff;
                            }
                        } while ((v23 = (unsigned long)(unsigned long long)((unsigned int)iter + 8), v7 |= v22 << ((char)(iter & 0xffffffff) & 63), iter = v23, v20 > (unsigned int)iter));
                        v12 = v20;
                        iter = v23;
                    }
                    v24 = (iter & 0xffffffff) - v12;
                    v1 = (short)idx[8];
                    v25 = v7 >> ((char)(v12 & 0xffffffff) & 63);
                    v3 = (&g_413380.field_0)[v12 & 0xffffffff];
                    v26 = v24;
                    if (a3 > (unsigned int)v26)
                    {
                        do
                        {
                            v27 = g_41b008;
                            if ((unsigned int)v27 < *((int *)&g_41b00c))
                            {
                                v28 = (&g_49d000)[v27];
                                g_41b008 = (unsigned int)v27 + 1;
                            }
                            else
                            {
                                g_41b004 = v10;
                                v28 = sub_40d830(0) & 0xff;
                            }
                        } while ((v26 = (unsigned long long)((unsigned int)v24 + 8), v25 = (unsigned long long)(v25 | v28 << ((char)(v24 & 0xffffffff) & 63)), v24 = v26, a3 > (unsigned int)v24));
                    }
                    v29 = (v4 & (unsigned int)v25) * 16 + a1;
                    i = *((char *)v29);
                    if (i > 16)
                    {
                        v5 = v7;
                        do
                        {
                            if (i == 99)
                                return 1;
                            v31 = (char)v29[1];
                            v32 = i - 16;
                            v33 = v26 - v31;
                            v25 >>= (char)v31 & 63;
                            v26 = v33;
                            if ((unsigned int)v26 < v32)
                            {
                                do
                                {
                                    v34 = g_41b008;
                                    if ((unsigned int)v34 < *((int *)&g_41b00c))
                                    {
                                        v35 = (&g_49d000)[v34];
                                        g_41b008 = (unsigned int)v34 + 1;
                                    }
                                    else
                                    {
                                        g_41b004 = v10;
                                        v35 = sub_40d830(0) & 0xff;
                                    }
                                } while ((v26 = (unsigned long long)((unsigned int)v33 + 8), v25 = (unsigned long long)(v25 | v35 << ((char)(v33 & 0xffffffff) & 63)), v33 = v26, v32 > (unsigned int)v33));
                            }
                            v36 = ((&g_413380.field_0)[v32] & (unsigned short)v25) * 16 + (long long)v29[8];
                            i = *((char *)v36);
                            v29 = v36;
                        } while (i > 16);
                        v7 = v5;
                    }
                    v37 = (char)v29[1];
                    v38 = (v26 & 0xffffffff) - v37;
                    v39 = v25 >> ((char)v37 & 63);
                    if (i > v38)
                    {
                        v5 = v29;
                        v40 = v39;
                        v41 = i;
                        v25 = v38;
                        while (1)
                        {
                            v42 = v25;
                            v43 = g_41b008;
                            if ((unsigned int)v43 < *((int *)&g_41b00c))
                            {
                                v44 = (&g_49d000)[v43];
                                v25 = (unsigned int)v42 + 8;
                                g_41b008 = (unsigned int)v43 + 1;
                                v40 |= v44 << ((char)(v42 & 0xffffffff) & 63);
                                if (v41 <= (unsigned int)v25)
                                    break;
                            }
                            else
                            {
                                g_41b004 = v10;
                                v25 = (unsigned int)v42 + 8;
                                v40 |= (sub_40d830(0) & 0xff) << ((char)(v42 & 0xffffffff) & 63);
                                if (v41 <= (unsigned int)v25)
                                    break;
                            }
                        }
                        i = v41;
                        v29 = v5;
                        v39 = v40;
                        v38 = v25;
                    }
                    v2 = v38 - i;
                    v45 = (v3 & (unsigned int)v7) + v1 + 22725 - (v38 < i) & 0xffffffff;
                    v3 = v39 >> ((char)i & 63);
                    j = ((v10 & 0xffffffff) - (short)v29[8] & 0xffffffff) - (*((short *)(v25 + i * 2)) & (unsigned int)v39);
                    while (1)
                    {
                        v47 = v10;
                        v48 = j & 0x7fff;
                        v49 = ((unsigned int)v47 < (unsigned int)v48 ? 0x8000 - (unsigned int)v48 : 0x8000 - (unsigned int)v47);
                        v50 = ((unsigned int)v45 <= v49 ? v45 & 0xffffffff : v49);
                        v51 = (v47 & 0xffffffff) - v48;
                        v45 -= v50;
                        if (((unsigned int)v48 < (unsigned int)v47 ? (unsigned int)v51 : (unsigned int)((v48 & 0xffffffff) - v47)) >= (unsigned int)v50)
                        {
                            j = (unsigned int)v48 + (unsigned int)v50;
                            v10 = v47 + v50;
                            memcpy(&(&g_43c000)[v47 & 0xffffffff], &(&g_43c000)[v48 & 0xffffffff], v50 & 0xffffffff);
                            if ((unsigned int)v10 == 0x8000)
                                goto LABEL_408d17;
                            goto LABEL_408ca2;
                        }
                        else
                        {
                            j = (unsigned int)v48 + (unsigned int)v50;
                            v52 = v48 & 0xffffffff;
                            do
                            {
                                (&g_43c000)[v51 + v52] = (&g_43c000)[v52 & 0xffffffff];
                                v52 = (unsigned int)v52 + 1;
                            } while ((unsigned int)v52 != j);
                            v10 = v47 + v50;
                            if ((unsigned int)v10 != 0x8000)
                            {
LABEL_408ca2:
                                if (!(unsigned int)v45)
                                    break;
                            }
                            else
                            {
LABEL_408d17:
                                g_41b004 = 0x8000;
                                v10 = 0;
                                sub_40d7e0();
                                if (!(unsigned int)v45)
                                    break;
                            }
                        }
                    }
                    iter = v2;
                    v7 = v3;
                    goto LABEL_4088a0;
                }
            } while ((g_41b004 = 0x8000, v9 = (unsigned long)0, sub_40d7e0(), (unsigned int)iter >= a2));
        }
        v54 = iter;
        v55 = g_41b008;
        if ((unsigned int)v55 < *((int *)&g_41b00c))
        {
            v56 = (&g_49d000)[v55];
            g_41b008 = (unsigned int)v55 + 1;
        }
        else
        {
            g_41b004 = v9;
            v56 = sub_40d830(0) & 0xff;
        }
        iter = (unsigned int)v54 + 8;
        v7 |= v56 << ((char)(v54 & 0xffffffff) & 63);
    }
}



// Function: huft_build @ 0x8dc0
typedef struct struct_4 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_4;

typedef struct struct_2 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned short field_10;
    char padding_12[14];
    unsigned short field_20;
} struct_2;

typedef struct struct_1 {
    char field_0[4];
    unsigned int field_4;
} struct_1;

typedef struct struct_0 {
    char field_0;
    char padding_1[3];
    unsigned int field_4;
} struct_0;

extern char g_0;
extern unsigned int g_4dda08;

unsigned int huft_build(unsigned int *a0, unsigned int a1, unsigned int a2, unsigned long a3, unsigned long a4, struct_2 *a5, unsigned int *a6)
{
    unsigned long v30;  // r10
    unsigned int *i;  // rdx
    unsigned int v40;  // r8d
    unsigned int v41;  // esi
    unsigned long idx;  // r9
    int v43;  // eax
    struct_0 *node;  // rdx
    unsigned int v45;  // esi
    unsigned long index;  // rdx
    unsigned int v47;  // esi
    void* iter;  // rcx
    unsigned int v49;  // edx
    unsigned long idx1;  // rcx
    void* iter1;  // rax
    unsigned long v51;  // rax
    unsigned long idx2;  // rdx
    unsigned long v53;  // rcx
    unsigned long v54;  // rax
    unsigned long long v55;  // r13
    unsigned long long v56;  // r11
    unsigned int v57;  // ebx
    unsigned int v58;  // r13d
    unsigned int v59;  // r12d
    unsigned int v33;  // edx
    struct_2 *v60;  // rsi
    struct_2 *v61;  // rax
    unsigned int v62;  // r8d
    unsigned int v63;  // edx
    unsigned int v64;  // r10d
    unsigned long long v65;  // r13
    unsigned long v66;  // r14
    unsigned long long v67;  // rdx
    unsigned long long v68;  // r12
    unsigned int v69;  // ebp
    unsigned long long v34;  // rax
    unsigned int v70;  // ecx
    unsigned int v71;  // ebx
    struct_1 *v72;  // rax
    unsigned int v73;  // ebx
    struct_2 *ptr;  // rcx
    unsigned int v76;  // ecx
    unsigned long v77;  // rdx
    struct_4 *v78;  // rdi
    char *v79;  // rdx
    unsigned int v35;  // ecx
    char v80;  // dil
    unsigned int v81;  // edx
    unsigned long v83;  // rdx
    struct_2 *v84;  // rcx
    unsigned int v85;  // edx
    unsigned long v86;  // rcx
    unsigned int m;  // r10d
    struct_2 *v88;  // r9
    unsigned int v36;  // eax
    struct_4 *v91;  // rdx
    unsigned int v92;  // edx
    unsigned long v93;  // rcx
    unsigned long v94;  // rcx
    unsigned long v95;  // rdx
    unsigned long v96;  // rdx
    struct_2 *ptr1;  // rax
    struct_2 *v99;  // rax
    unsigned long long v37;  // r15
    char *v38;  // r9
    unsigned int v39;  // r8d
    unsigned int v0;  // [bp-0x65c]
    struct_2 *v1;  // [bp-0x658]
    unsigned long long v2;  // [bp-0x650]
    unsigned int v3;  // [bp-0x648]
    unsigned int v4;  // [bp-0x644]
    unsigned int l;  // [bp-0x644]
    unsigned int j;  // [bp-0x640]
    int v7;  // [bp-0x63c]
    unsigned int v8;  // [bp-0x638]
    unsigned int v9;  // [bp-0x634]
    struct_2 *v10;  // [bp-0x630]
    struct_2 *v11;  // [bp-0x630]
    struct_1 *iter2;  // [bp-0x628]
    char *v13;  // [bp-0x620]
    unsigned int v14;  // [bp-0x618]
    unsigned int v15;  // [bp-0x614]
    char *v16;  // [bp-0x610]
    unsigned int v17;  // [bp-0x608]
    unsigned int v18;  // [bp-0x5f0]
    unsigned int v19;  // [bp-0x5ec]
    int v20;  // [bp-0x5e8]
    uint128_t v21;  // [bp-0x5d8]
    uint128_t v22;  // [bp-0x5c8]
    int v23;  // [bp-0x5b8]
    int <0x408dc0[is_12]|Stack bp-0x598, 1 B>;  // [bp-0x598]
    unsigned int v24;  // [bp-0x598]
    unsigned int v25;  // [bp-0x594]
    char v26;  // [bp-0x590]
    char v27;  // [bp-0x550]
    unsigned long v28;  // [bp-0x548]
    char v29;  // [bp-0x4c8]

    v30 = a1;
    v20 = (int)0;
    *((unsigned int *)&(&v23)[16]) = 0;
    v21 = 0;
    v22 = 0;
    *((uint128_t *)&v23) = 0;
    i = a0;
    do
    {
        idx1 = *(i);
        i += 1;
        *((unsigned int *)((char *)&v20 + 4 * idx1)) = *((int *)((char *)&v20 + 4 * idx1)) + 1;
    } while (&a0[v30] != i);
    v33 = *((unsigned int *)&v20);
    if (v33 != (unsigned int)v30)
    {
        v34 = 1;
        do
        {
            v35 = v34;
            if (*((int *)((char *)&v20 + 4 * v34)))
            {
                j = v34;
                v36 = 1 << ((char)v35 & 31);
                goto LABEL_408e9d;
            }
        } while ((v34 += 1, v34 != 0x11));
        j = 0x11;
        v36 = 0x20000;
        v35 = 0x11;
LABEL_408e9d:
        v37 = (*(a6) < v35 ? j : *(a6));
        v38 = &v23 - 16;
        v39 = 16;
        do
        {
            v40 = v39;
            if (*((int *)&v38))
            {
                v15 = v40;
                v37 = (v40 < (unsigned int)v37 ? v40 : v37 & 0xffffffff);
                *(a6) = v37;
                if (v35 < v40)
                {
                    idx = v35;
                    v43 = v36 - *((int *)((char *)&v20 + 4 * idx));
                    if (v36 - *((int *)((char *)&v20 + 4 * idx)) < 0)
                        return 2;
                    node = &v20 + idx * 4;
                    do
                    {
                        v36 = v43 * 2;
                        if (&v20 + ((v40 - v35 - 1 & 0xffffffff) + idx) * 4 == node)
                        {
                            v33 = *((int *)((char *)&v20 + 4 * v40));
                            goto LABEL_408faa;
                        }
                    } while ((v43 = (int)(v36 - node->field_4), node += 4, v43 >= 0));
LABEL_408faa:
                    v45 = v36;
                    v18 = v45 - v33;
                    if (v45 - v33 >= 0)
                    {
                        v25 = 0;
                        *((unsigned int *)((char *)&v20 + 4 * v40)) = v36;
                        goto LABEL_408fcd;
                    }
                }
                else
                {
                    index = v40;
                    v47 = v36;
                    v18 = v47 - *((int *)((char *)&v20 + 4 * index));
                    if (v47 - *((int *)((char *)&v20 + 4 * index)) < 0)
                        return 2;
                    *((unsigned int *)((char *)&v20 + 4 * index)) = v36;
                    v25 = 0;
                    if (v40 == 1)
                        goto LABEL_408ffb;
LABEL_408fcd:
                    iter = &v26;
                    v49 = 0;
                    iter1 = &v20 - 4;
                    do
                    {
                        iter1 += 4;
                        iter += 4;
                        v49 += *((int *)((char *)iter1 - 4));
                        *((unsigned int *)((char *)iter - 4)) = v49;
                    } while (&v20 - 8 + (v40 - 2) * 4 != iter1);
                }
            }
        } while ((v38 -= 4, v39 = v40 - 1, v40 != 1));
        if (!(unsigned int)v37)
        {
            v41 = v36;
            *(a6) = 0;
            v18 = v41 - *((unsigned int *)&v20);
            if (v41 - *((unsigned int *)&v20) >= 0)
            {
                *((unsigned int *)&v20) = v36;
                v25 = 0;
                v15 = 0;
                v40 = v39;
                goto LABEL_408fcd;
            }
        }
        else
        {
            *(a6) = 0;
            v37 = 0;
            v15 = 0;
            v40 = v39;
            goto LABEL_408faa;
        }
LABEL_408ffb:
        v51 = 0;
        do
        {
            idx2 = a0[v51];
            if ((unsigned int)idx2)
            {
                v53 = *((int *)((char *)&<0x408dc0[is_12]|Stack bp-0x598, 1 B> + 4 * idx2));
                *((unsigned int *)&(&v29)[4 * v53]) = v51;
                *((unsigned int *)((char *)&<0x408dc0[is_12]|Stack bp-0x598, 1 B> + 4 * idx2)) = (unsigned int)v53 + 1;
            }
        } while ((v51 = (unsigned long)(v51 + 1), v30 != v51));
        v54 = v15;
        v28 = 0;
        v24 = 0;
        if ((unsigned int)v54 < j)
            return v15 != 1 & _INSERT(v18, 0, v18);
        v56 = v55;
        v57 = 0;
        v58 = -((unsigned int)v37);
        v13 = &v29;
        v59 = 0;
        v60 = a5;
        v16 = &(&v29)[4 * (&v24)[v54]];
        v7 = -0x1;
        iter2 = &v20 + j * 4;
        v19 = v15 + 1;
        v61 = 0;
        do
        {
            v4 = (unsigned int)iter2->field_0;
            if (iter2->field_0)
            {
                v62 = v59;
                v17 = 1 << ((char)(j - 1) & 31);
                v11 = v10;
                do
                {
                    l = v4;
                    v63 = (unsigned int)v37 + v58;
                    if (v63 < j)
                    {
                        v64 = v63;
                        v14 = v62;
                        v65 = j - v63 & 0xffffffff;
                        v66 = v7 + 1;
                        v67 = v56;
                        v68 = v15 - v63 & 0xffffffff;
                        do
                        {
                            v9 = v64;
                            v7 = v66;
                            v57 = 1 << ((char)(v65 & 0xffffffff) & 31);
                            v69 = v65;
                            v3 = v64 - v37;
                            v8 = v65;
                            if (v57 > l)
                            {
                                v70 = ((unsigned int)v68 <= (unsigned int)v37 ? (unsigned int)v68 : (unsigned int)v37);
                                if (v70 > (unsigned int)v65)
                                {
                                    v69 = (unsigned int)v65 + 1;
                                    v71 = v57 - l;
                                    v72 = iter2;
                                    if (v70 > v69)
                                    {
                                        do
                                        {
                                            v73 = v71 * 2;
                                        } while (v72->field_4 < v73 && (v69 += 1, v71 = v73 - v72->field_4, v72 += 4, v69 < v70));
                                    }
                                    v57 = 1 << ((char)v69 & 31);
                                }
                            }
                            v2 = v67;
                            v1 = v60;
                            v0 = v64;
                            v67 = v2;
                            ptr = malloc((v57 + 1) * 16);
                            if (!ptr)
                            {
                                if (!v7)
                                    return 3;
                                sub_408d90(v28);
                                return 3;
                            }
                            g_4dda08 = v57 + g_4dda08 + 1;
                            v61 = &ptr->field_10;
                            v1->field_0 = v61;
                            v60 = &ptr->field_8;
                            ptr->field_8 = 0;
                            (&v28)[v66] = v61;
                            if ((unsigned int)v66)
                            {
                                v76 = v14;
                                v11 = v61;
                                v77 = _INSERT(v67, 0, (char)v69 + 16);
                                (&v24)[v66] = v76;
                                v78 = *((long long *)&(&v27)[8 * v66]) + (v76 >> ((char)v3 & 31)) * 16;
                                v67 = _INSERT(v77, 1, v37);
                                v78->field_8 = v61;
                                v78->field_0 = v67;
                            }
                            v66 += 1;
                            v64 = v0 + (unsigned int)v37;
                            v65 -= v37;
                            v68 -= v37;
                        } while (j > v64);
                        v58 = v9;
                        v62 = v14;
                        v56 = v67;
                    }
                    else
                    {
                        v8 = j - v58;
                    }
                    v79 = v13;
                    v80 = 99;
                    if (v16 > v79)
                    {
                        v81 = *((int *)&v79);
                        v13 = v79 + 4;
                        if (v81 < a2)
                        {
                            v80 = (char)((v81 != 0x100) + 15);
                            *((unsigned short *)&v11) = v81;
                        }
                        else
                        {
                            v83 = v81 - a2;
                            v84 = (struct_2 *)_INSERT(v11, 0, *((short *)(a3 + v83 * 2)));
                            v80 = *((char *)(a4 + v83 * 2));
                            v11 = v84;
                        }
                    }
                    v85 = v62 >> ((char)v58 & 31);
                    v86 = v85;
                    if (v85 < v57)
                    {
                        m = v57;
                        v88 = v61;
                        do
                        {
                            v56 = _INSERT(_INSERT(v56, 0, v80), 1, (char)j - v58 & 0xffffffff);
                            v91 = &(&v88->field_0)[2 * (v86 & 0xffffffff)];
                            v91->field_0 = v56;
                            v91->field_8 = v11;
                            v86 += 1 << ((char)v8 & 31);
                        } while ((unsigned int)v86 < m);
                        v57 = m;
                        v61 = v88;
                    }
                    v92 = v17;
                    if (v62 & v17)
                    {
                        do
                        {
                            v62 ^= v92;
                            v92 >>= 1;
                        } while (v92 & v62);
                    }
                    v62 ^= v92;
                    v93 = v7;
                    if (((1 << ((char)v58 & 31)) - 1 & v62) != (&v24)[v93])
                    {
                        v94 = v58;
                        v95 = (unsigned int)v93 - 1;
                        do
                        {
                            v96 = v95;
                            v94 -= v37;
                        } while ((v95 = (unsigned long)(v96 - 1), ((1 << (unsigned int)((char)v94 & 31)) - 1 & v62) != (&v25)[v95]));
                        v7 = v96;
                        v58 = v94;
                    }
                    v4 = l - 1;
                } while (l != 1);
                v59 = v62;
            }
            j += 1;
            iter2 = &iter2->field_4;
        } while (j != v19);
        return v15 != 1 & _INSERT(v18, 0, v18);
    }
    else
    {
        ptr1 = malloc(48);
        if (ptr1)
        {
            v99 = &ptr1->field_10;
            *((unsigned long long *)((char *)v99 - 8)) = 0;
            *((unsigned short *)&v99->field_0) = 355;
            v99->field_10 = 355;
            g_4dda08 = g_4dda08 + 3;
            a5->field_0 = v99;
            *(a6) = 1;
            return 0;
        }
        return 3;
    }
}



// Function: inflate_block @ 0x9d20
typedef struct struct_1 {
    char padding_0[267224320];
    char field_fed8500;
} struct_1;

extern char g_0;
extern unsigned int g_41b008;
extern char g_41b00c;
extern char g_49d000;
extern unsigned int g_4dda0c;
extern unsigned long long g_4dda10;

unsigned long long inflate_block(unsigned int *a0, unsigned long a1, unsigned int a2, struct_1 *index)
{
    unsigned int v21;  // ebp
    unsigned int v23;  // edx
    unsigned long long v24;  // r15
    unsigned long long v25;  // r14
    unsigned int v26;  // r13d
    unsigned int v27;  // ebx
    unsigned long v28;  // fs
    unsigned long v29;  // rax
    unsigned int v30;  // eax
    unsigned long long v17;  // rbx
    unsigned int v18;  // eax
    unsigned int v0;  // [bp-0x570]
    unsigned long v1;  // [bp-0x40]
    unsigned long long v2;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    unsigned long long v5;  // [bp-0x20]
    unsigned long long v6;  // [bp-0x18]
    unsigned long long v7;  // [bp-0x18]
    unsigned long long v8;  // [bp-0x10]
    unsigned long long v9;  // [bp-0x10]
    unsigned long long v10;  // [bp-0x8]
    unsigned long long v11;  // [bp-0x8]

    v18 = v21 - 3;
    g_4dda0c = v18;
    v23 = (unsigned int)v17 & 3;
    g_4dda10 = v17 >> 2;
    if (v23 == 2)
    {
        v10 = v24;
        v8 = v25;
        v6 = v11;
        v4 = v9;
        v3 = v7;
        v2 = v5;
        v26 = g_4dda0c;
        v27 = g_4dda10;
        v1 = *((long long *)(40 + v28));
        if (g_4dda0c <= 4)
        {
            v29 = g_41b008;
            if ((unsigned int)v29 < *((int *)&g_41b00c))
            {
                g_41b008 = (unsigned int)v29 + 1;
                v30 = (&g_49d000)[v29];
            }
            else
            {
                v30 = sub_40d830(0) & 0xff;
            }
            v26 = g_4dda0c + 8;
            v27 = (unsigned int)g_4dda10 | (unsigned int)(v30 << ((char)g_4dda0c & 63));
        }
        v0 = (v27 & 31) + 0x101;
        if (v26 - 5 > 4)
            goto LABEL_0x409570;
        if (g_41b008 >= *((int *)&g_41b00c))
            goto LABEL_0x409680;
        g_41b008 = g_41b008 + 1;
    }
    else if (v23 && v23 != 1)
    {
        return 2;
    }
}



// Function: inflate @ 0x9e80
typedef struct struct_1 {
    char padding_0[267224320];
    char field_fed8500;
} struct_1;

extern char g_0;
extern unsigned int g_41b004;
extern unsigned int g_41b008;
extern unsigned int g_4dda08;
extern unsigned int g_4dda0c;
extern unsigned long long g_4dda10;

unsigned long long inflate(unsigned long a0, unsigned long a1, unsigned int a2, unsigned long a3)
{
    char *v3;  // rax
    unsigned int v4;  // rsi
    unsigned int v5;  // edx
    struct_1 *v6;  // rcx
    unsigned long long v7;  // rax
    unsigned int v8;  // ecx
    unsigned long v9;  // fs
    unsigned int v0;  // [bp-0x1c]
    unsigned int i[3];  // [bp-0x14]

    *(v3) = *(v3) + *((char *)&v3);
    g_41b004 = 0;
    g_4dda0c = 0;
    g_4dda10 = 0;
    do
    {
        g_4dda08 = 0;
        v7 = sub_409d20(&(unsigned int)i, v4, v5, v6);
        if ((unsigned int)v7)
        {
            if (v3 == *((long long *)(40 + v9)))
                return v7;
            __stack_chk_fail(); /* do not return */
        }
    } while (!(unsigned int)i);
    if (g_4dda0c > 7)
    {
        v8 = g_4dda0c - 8;
        g_4dda0c = g_4dda0c & 7;
        g_41b008 = g_41b008 + ~(v8 >> 3);
    }
    v0 = v7;
    sub_40d7e0();
    v7 = v0;
    if (v3 != *((long long *)(40 + v9)))
        __stack_chk_fail(); /* do not return */
    return v7;
}



// Function: init_block @ 0x9f30
extern char g_0;
extern unsigned long long g_4dda38;
extern unsigned long long g_4dda40;
extern char g_4dda48;
extern char g_4dda49;
extern unsigned int g_4dda4c;
extern unsigned int g_4dda50;
extern unsigned int g_4dda54;
extern unsigned long long g_4df9e0;
extern unsigned long long g_4dfa2c;
extern unsigned long long g_4dff80;
extern unsigned long long g_4dfff8;
extern unsigned long long g_4e0080;
extern unsigned short g_4e0480;
extern unsigned long long g_4e04f8;

unsigned int init_block(void)
{
    unsigned long long iter;  // rax
    unsigned long long node;  // rax
    unsigned long long iter1;  // rax

    iter = &g_4e0080;
    do
    {
        iter += 4;
        *((unsigned short *)(iter - 4)) = 0;
    } while (iter != &g_4e04f8);
    node = &g_4dff80;
    do
    {
        node += 4;
        *((unsigned short *)(node - 4)) = 0;
    } while (node != &g_4dfff8);
    __unsupported_jumpkind_Ijk_NoDecode()
    iter1 = &g_4df9e0;
    do
    {
        iter1 += 4;
        *((unsigned short *)(iter1 - 4)) = 0;
    } while (iter1 != &g_4dfa2c);
    g_4dda38 = 0;
    g_4e0480 = 1;
    g_4dda40 = 0;
    g_4dda4c = 0;
    g_4dda50 = 0;
    g_4dda54 = 0;
    g_4dda49 = 0;
    g_4dda48 = 1;
    return 1;
}



// Function: pqdownheap @ 0x9fe0
extern char g_4dee60;
extern char g_4df0a4;
extern int g_4df0c0[4];

int pqdownheap(unsigned long a0, int a1)
{
    unsigned long idx;  // r8
    unsigned int v3;  // edx
    unsigned short v12;  // si
    unsigned long idx1;  // r15
    unsigned short v5;  // r12w
    unsigned long index;  // r10
    unsigned int v7;  // eax
    unsigned short v8;  // si
    unsigned int v9;  // ecx
    unsigned long idx2;  // r13
    unsigned int v0;  // [bp-0x34]

    idx = a1;
    v3 = a1 * 2;
    v0 = g_4df0c0[idx];
    if (v3 <= *((int *)&g_4df0a4))
    {
        idx1 = g_4df0c0[idx];
        v5 = *((short *)(a0 + idx1 * 4));
        do
        {
            index = g_4df0c0[v3];
            v7 = v3;
            v8 = *((short *)(a0 + index * 4));
            v9 = index;
            if (*((int *)&g_4df0a4) <= v3)
                continue;
            v7 += 1;
            idx2 = g_4df0c0[v7];
            if (*((short *)(a0 + idx2 * 4)) >= v8)
            {
                if (v12 != v8)
                {
                    v7 = v3;
                    continue;
                }
                if ((&g_4dee60)[idx2] <= (&g_4dee60)[index])
                {
                    v9 = idx2;
                    if ((&g_4dee60)[idx2] <= (&g_4dee60)[index])
                        goto LABEL_40a0af;
                }
                else
                {
                    v9 = v9;
                    if ((&g_4dee60)[idx2] <= (&g_4dee60)[index])
                    {
LABEL_40a0af:
LABEL_40a0b0:
                        continue;
                    }
                }
                v7 = v3;
                goto LABEL_40a0b0;
            }
            else
            {
                v12 = *((short *)(a0 + idx2 * 4));
                v9 = idx2;
                v8 = v12;
            }
        } while (v5 >= v8 && (v5 != v8 || (&g_4dee60)[idx1] > (&g_4dee60)[v9]) && (v3 = v7 * 2, g_4df0c0[idx] = (int)v9, idx = (unsigned long)(long long)(int)v7, v3 <= *((int *)&g_4df0a4)));
    }
    g_4df0c0[idx] = v0;
    return v0;
}



// Function: gen_bitlen @ 0xa0c0
typedef struct struct_0 {
    int field_0;
    int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern char g_0;
extern struct_1 *g_4dda38;
extern unsigned long long g_4dda40;
extern int g_4df0a0;
extern struct_0 g_4df0c0;
extern char g_4df0c4;
extern void g_4df9c0;
extern void g_4df9e0;

char * gen_bitlen(void* idx)
{
    unsigned long v6;  // r9
    int v7;  // r15d
    unsigned long v16;  // rcx
    int v17;  // edx
    unsigned long long v18;  // rcx
    unsigned short *v19;  // rbx
    char *v20;  // rax
    unsigned long index;  // rsi
    unsigned long long v22;  // rbp
    unsigned int v23;  // ebp
    unsigned int v24;  // ebx
    unsigned long idx1;  // r13
    unsigned int v8;  // r8d
    unsigned short k;  // cx
    unsigned long idx2;  // r12
    unsigned long v28;  // r11
    void* v29;  // r9
    unsigned int v30;  // ebp
    unsigned int v31;  // ebp
    unsigned long v32;  // rsi
    unsigned int iter;  // eax
    unsigned long iter1;  // rax
    void* node;  // rdx
    unsigned long long v9;  // r14
    unsigned long v36;  // rsi
    unsigned long long v37;  // rbp
    int v38;  // edx
    unsigned int v39;  // r12d
    unsigned long v40;  // rcx
    unsigned short *v41;  // r13
    unsigned long v42;  // r14
    unsigned long v43;  // rcx
    void* iter2;  // rax
    unsigned long v11;  // rax
    char v12;  // sil
    unsigned int v13;  // r12d
    unsigned long v14;  // r13
    struct_0 *j;  // r11
    unsigned long v0;  // [bp-0x58]
    struct_0 *v1;  // [bp-0x50]
    char v2;  // [bp-0x41]
    char *v3;  // [bp-0x40]
    unsigned long v4;  // [bp-0x38]

    v6 = *((long long *)idx);
    v7 = (int)idx[36];
    v8 = (int)idx[32];
    v9 = (long long)idx[8];
    v4 = (long long)idx[16];
    *((int *)&v0) = (int)idx[24];
    iter2 = &g_4df9c0;
    do
    {
        iter2 += 2;
        *((unsigned short *)((char *)iter2 - 2)) = 0;
    } while (iter2 != &g_4df9e0);
    v11 = *((int *)&(&g_4df0c0.field_0)[2 * g_4df0a0]);
    *((unsigned short *)(v6 + v11 * 4 + 2)) = 0;
    if (g_4df0a0 > 571)
        return v11;
    v12 = 0;
    v13 = 0;
    v2 = 0;
    v14 = g_4dda40;
    j = &(&g_4df0c0.field_0)[2 * g_4df0a0];
    v3 = g_4dda38;
    v1 = &(&g_4df0c4)[2284 + 4 * g_4df0a0 + -4 * g_4df0a0];
    do
    {
        v16 = j->field_4;
        v17 = v16;
        v18 = v16 * 4;
        v19 = v6 + v18;
        v20 = *((short *)(v6 + v19[1] * 4 + 2)) + 1;
        if (v8 < (unsigned int)v20)
        {
            v13 += 1;
            v20 = v8;
        }
        v19[1] = *((unsigned short *)&v20);
        if (v7 >= v17)
        {
            index = (unsigned int)v20;
            v22 = 0;
            *((unsigned short *)&(&g_4df9c0)[2 * index]) = *((short *)&(&g_4df9c0)[2 * index]) + 1;
            if ((unsigned int)v0 <= v17)
            {
                v22 = *((int *)(v4 + (v17 - (unsigned int)v0) * 4));
                index = &v20[v22];
            }
            v20 = *(v19);
            v14 += v20 * index;
            v12 = 1;
            if (v9)
            {
                v2 = 1;
                v3 = &v3[v20 * (*((short *)(v9 + v18 + 2)) + v22)];
            }
        }
        j = &j->field_4;
    } while (j != v1);
    if (v2)
    {
        v20 = v3;
        g_4dda38 = v20;
    }
    if (v12)
        g_4dda40 = v14;
    if (!v13)
        return v20;
    v23 = v13 - 2;
    v24 = v8 - 1;
    v0 = v6;
    idx1 = v24;
    k = *((short *)&(&g_4df9c0)[2 * idx1]);
    idx2 = v8;
    v28 = idx2;
    v29 = &(&g_4df9c0)[2 * idx2];
    v30 = v23;
    v31 = v23;
    if (k)
        goto LABEL_40a2a8;
    while (1)
    {
        iter = v24;
        node = v29;
        do
        {
            k = *((short *)((char *)node - 4));
            v32 = iter;
            node -= 2;
            iter -= 1;
        } while (!k);
        while (1)
        {
            *((unsigned short *)&(&g_4df9c0)[2 * iter]) = k - 1;
            *((unsigned short *)&(&g_4df9c0)[2 * v32]) = *((short *)&(&g_4df9c0)[2 * v32]) + 2;
            iter1 = *((short *)&(&g_4df9c0)[2 * idx2]) - 1;
            *((unsigned short *)&(&g_4df9c0)[2 * idx2]) = iter1;
            if (v30 == v23 - (v13 - 1 & 0xfffffffe))
            {
                if (!v8)
                    return iter1;
                v36 = g_4dda40;
                v37 = 0;
                v38 = 573;
                v39 = (unsigned short)iter1;
                while (1)
                {
                    if (v39)
                    {
                        iter1 = v38 - 1;
                        while (1)
                        {
                            v40 = *((int *)&(&g_4df0c0.field_0)[2 * iter1]);
                            v38 = iter1;
                            if (v7 >= (unsigned int)v40)
                            {
                                v41 = v0 + v40 * 4;
                                v42 = v41[1];
                                if ((unsigned int)v42 != v8)
                                {
                                    v43 = *(v41);
                                    v41[1] = v8;
                                    v37 = v28 - v42;
                                    *((char *)iter1) = *((char *)iter1) + (char)iter1;
                                    v36 += v37 * v43;
                                }
                                iter1 -= 1;
                                v39 -= 1;
                                if (v39 == 1)
                                    break;
                            }
                            else
                            {
                                iter1 -= 1;
                            }
                        }
                    }
                    v8 = v24;
                    v28 -= 1;
                    if (!v24)
                        break;
                    v39 = *((short *)&(&g_4df9c0)[2 * v28]);
                    v24 -= 1;
                }
                if (!(char)v37)
                    return iter1;
                g_4dda40 = v36;
                return iter1;
            }
            k = *((short *)&(&g_4df9c0)[2 * idx1]);
            v31 = v30 - 2;
            if (!k)
                break;
LABEL_40a2a8:
            v30 = v31;
            v32 = v8;
            iter = v24;
        }
    }
}



// Function: scan_tree @ 0xa370
extern char g_0;
extern unsigned short g_4df9e0[4];
extern unsigned short g_4dfa20;
extern unsigned short g_4dfa24;
extern unsigned short g_4dfa28;

unsigned long long scan_tree(unsigned short *a0, int a1)
{
    unsigned int v1;  // edx
    unsigned long long v2;  // rax
    int v3;  // r9d
    int v4;  // r8d
    unsigned short *v5;  // r11
    unsigned short *iter;  // rcx
    unsigned long long v7;  // rax
    unsigned int v8;  // edi
    unsigned int v9;  // esi

    v1 = a0[1];
    v2 = a1;
    a0[3 + 2 * v2] = 0xffff;
    v3 = -(v1 < 1) + 4;
    v4 = (-(v1 < 1) & 131) + 7;
    if (a1 < 0)
        return v2;
    v5 = &a0[2 * v2 + 5];
    iter = a0 + 3;
    v7 = 0;
    v8 = 0xffffffff;
    while (1)
    {
        v7 = (unsigned int)v7 + 1;
        v9 = v1;
        v1 = *(iter);
        if ((unsigned int)v7 < v4)
        {
            if (v1 == v9)
                goto LABEL_40a3f5;
            if ((unsigned int)v7 < v3)
            {
                g_4df9e0[2 * v9] = g_4df9e0[2 * v9] + (unsigned short)v7;
                if (!v1)
                    goto LABEL_40a440;
                iter += 2;
                v8 = v9;
                v3 = 4;
                v7 = 0;
                v4 = 7;
                if (v5 == iter)
                    return 0;
                continue;
            }
        }
        if (v9)
        {
            if (v9 != v8)
                g_4df9e0[2 * v9] = g_4df9e0[2 * v9] + 1;
            g_4dfa20 = g_4dfa20 + 1;
        }
        else if ((unsigned int)v7 <= 10)
        {
            g_4dfa24 = g_4dfa24 + 1;
        }
        else
        {
            g_4dfa28 = g_4dfa28 + 1;
        }
        if (!v1)
        {
LABEL_40a440:
            v8 = v9;
            v3 = 3;
            v4 = 138;
            v7 = 0;
LABEL_40a3f5:
            iter += 2;
            if (v5 == iter)
                return v7;
        }
        else if (v1 == v9)
        {
            v8 = v1 & 0xffff;
            v3 = 3;
            v4 = 6;
            v7 = 0;
            goto LABEL_40a3f5;
        }
    }
}



// Function: set_file_type @ 0xa480
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 *g_4dda20;
extern unsigned short g_4e0080;
extern unsigned short g_4e009c;
extern unsigned short g_4e0280;
extern unsigned short g_4e0480;

unsigned short * set_file_type(void)
{
    unsigned long long iter;  // rdx
    unsigned short *i;  // rax
    unsigned short *v3;  // rax
    unsigned short *j;  // rax
    unsigned long long v5;  // rcx
    unsigned short *v6;  // rax
    unsigned short *k;  // rax
    unsigned short *v8;  // rax

    iter = 0;
    i = &g_4e0080;
    do
    {
        v3 = i + 2;
        iter += *(i);
        i = v3;
    } while (i != &g_4e009c);
    j = &g_4e009c;
    v5 = 0;
    do
    {
        v6 = j + 2;
        v5 += *(j);
        j = v6;
    } while (j != &g_4e0280);
    k = &g_4e0280;
    do
    {
        v8 = k + 2;
        iter += *(k);
        k = v8;
    } while (k != &g_4e0480);
    g_4dda20->field_0 = (unsigned int)iter <= (unsigned int)v5 >> 2;
    return g_4dda20;
}



// Function: gen_codes @ 0xa500
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
} struct_0;

extern char g_0;
extern char g_4df9c0;

void gen_codes(struct_0 *a0, int a1)
{
    unsigned long long i;  // rdx
    unsigned long long v3;  // rax
    struct_0 *iter;  // rbx
    unsigned int v5;  // esi
    unsigned long idx;  // rax
    unsigned short v7;  // di
    char v0;  // [bp-0x48]
    unsigned int v8;  // [bp-0x46]

    i = 0;
    v3 = 0;
    do
    {
        v3 = _INSERT(v3, 0, (unsigned short)v3 + *((short *)&(&g_4df9c0)[i])) * 2;
        *((unsigned short *)((char *)&v8 + i)) = v3;
        i += 2;
    } while (i != 30);
    if (a1 < 0)
        return;
    iter = a0;
    do
    {
        v5 = iter->field_2;
        if (v5)
        {
            idx = v5;
            v7 = *((short *)&(&v0)[2 * idx]);
            *((unsigned short *)&(&v0)[2 * idx]) = v7 + 1;
            iter->field_0 = sub_404380(v7, v5);
        }
    } while ((iter += 4, iter != a0 + (long long)a1 * 4 + 4));
    return;
}



// Function: build_tree @ 0xa5a0
typedef struct struct_1 {
    unsigned short field_0;
} struct_1;

typedef struct struct_0 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    char padding_10[12];
    int field_1c;
    char padding_20[4];
    int field_24;
} struct_0;

extern char g_0;
extern unsigned long long g_4dda38;
extern unsigned long long g_4dda40;
extern char g_4dee60;
extern int g_4df0a0;
extern unsigned int g_4df0a4;
extern unsigned int g_4df0c0[4];
extern unsigned int g_4df0c4;

void build_tree(struct_0 *idx)
{
    int v6;  // eax
    unsigned short *v7;  // rbp
    char v16;  // r10b
    unsigned long v17;  // r9
    unsigned short *v18;  // r15
    unsigned long long v19;  // rcx
    unsigned long long v20;  // rax
    unsigned long v21;  // rax
    unsigned long v22;  // rax
    int v23;  // ecx
    int v24;  // r14d
    int i;  // r14d
    unsigned long long v8;  // r8
    unsigned long idx1;  // r14
    unsigned long long idx2;  // r12
    unsigned short *v28;  // r15
    unsigned long node;  // rbp
    char *v30;  // rax
    int v31;  // edx
    int v32;  // esi
    unsigned long v33;  // rdx
    unsigned short *v34;  // r9
    unsigned short *v35;  // r10
    unsigned long v9;  // r12
    unsigned short v36;  // si
    unsigned short v37;  // ax
    char v10;  // dl
    unsigned long iter;  // rax
    int v12;  // esi
    int v13;  // ecx
    int v14;  // edi
    unsigned long v15;  // rdx
    unsigned int *v0;  // [bp-0x58]
    int v1;  // [bp-0x50]
    unsigned short v2;  // [bp-0x4a]
    int v3;  // [bp-0x48]
    int v4;  // [bp-0x44]

    v6 = idx->field_1c;
    v7 = &idx->field_0->field_0;
    g_4df0a4 = 0;
    v8 = idx->field_8;
    v1 = v6;
    g_4df0a0 = 573;
    if (v1 > 0)
    {
        v9 = v1;
        v10 = 0;
        iter = 0;
        v12 = 0;
        v13 = -0x1;
        while (1)
        {
            if (v7[2 * iter])
            {
                v12 += 1;
                (&g_4dee60)[iter] = 0;
                v13 = iter;
                g_4df0c0[v12] = iter;
                iter += 1;
                v10 = 1;
                if (iter == v9)
                    break;
            }
            else
            {
                v7[1 + 2 * iter] = 0;
                iter += 1;
                if (iter == v9)
                    break;
            }
        }
        v3 = v13;
        if (v10)
        {
            g_4df0a4 = v12;
            if (v12 <= 1)
                goto LABEL_40a652;
            v4 = v12;
            v0 = &g_4df0c4;
        }
        else
        {
            v12 = 0;
            goto LABEL_40a652;
        }
    }
    else
    {
        v3 = -0x1;
        v9 = v1;
        v12 = 0;
LABEL_40a652:
        v14 = v3;
        v15 = v12;
        v16 = 0;
        v0 = &g_4df0c4;
        v17 = g_4dda38;
        do
        {
            if (v14 > 1)
            {
                v18 = v7;
                v19 = 0;
                v20 = 0;
            }
            else
            {
                v14 += 1;
                v20 = v14;
                v19 = v14 * 4;
                v18 = (char *)v7 + v19;
            }
            (&g_4df0c4)[v15] = v20;
            *(v18) = 1;
            (&g_4dee60)[v20] = 0;
            if (v8)
            {
                v16 = 1;
                v17 -= *((short *)(v8 + v19 + 2));
            }
            v15 += 1;
        } while ((unsigned int)v15 <= 1);
        v3 = v14;
        v21 = 1 - v12;
        v22 = -(v21);
        v23 = v12 + (v12 <= 1 ? (unsigned int)v21 : 0) + 1;
        if (1 < v12)
            v22 = 0;
        v4 = v23;
        g_4df0a4 = v23;
        g_4dda40 = g_4dda40 + v22 - 1;
        if (v16)
            g_4dda38 = v17;
    }
    idx->field_24 = v3;
    v24 = v4 >> 1;
    do
    {
        i = v24;
        sub_409fe0(v7, i);
        v24 = i - 1;
    } while (i != 1);
    idx1 = v4;
    idx2 = 571;
    v28 = v7;
    node = v9;
    v2 = (unsigned short)v1 + idx1;
    do
    {
        v30 = (unsigned int)(idx1 - 1) | 0x1be00;
        *(v30) = *(v30) + *((char *)&v30);
        g_4df0a4 = (unsigned int)v30;
        v1 = v31;
        g_4df0c4 = g_4df0c0[idx1];
        sub_409fe0(v28, v32);
        v33 = v1;
        g_4df0a0 = idx2;
        v34 = &v28[2 * v33];
        v35 = &v28[2 * g_4df0c4];
        v36 = *(v35) + *(v34);
        v0[idx2] = v33;
        g_4df0c0[idx2] = g_4df0c4;
        v28[2 * node] = v36;
        idx2 -= 2;
        (&g_4dee60)[node] = ((&g_4dee60)[g_4df0c4] <= (&g_4dee60)[v33] ? (&g_4dee60)[v33] + 1 : (&g_4dee60)[g_4df0c4] + 1);
        v37 = v2 - idx1;
        idx1 -= 1;
        v35[1] = v37;
        v34[1] = v37;
        g_4df0c4 = node;
        node += 1;
    } while ((unsigned int)idx1 > 1);
    g_4df0a0 = (v4 <= 1 ? 573 : -(v4) * 2 + 577) - 3;
    g_4df0c0[g_4df0a0] = g_4df0c4;
    sub_40a0c0(idx);
    sub_40a500(v28, v3);
    return;
}



// Function: build_bl_tree @ 0xa8c0
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern char g_413410;
extern struct_0 g_419340;
extern int g_4193a4;
extern int g_4193e4;
extern unsigned long long g_4dda40;
extern char g_4df9e0;
extern unsigned short g_4dff80;
extern unsigned short g_4e0080;

unsigned int build_bl_tree(void)
{
    unsigned long long v1;  // rax
    unsigned long long i;  // rdx

    sub_40a370(&g_4e0080, g_4193e4);
    sub_40a370(&g_4dff80, g_4193a4);
    sub_40a5a0(&g_419340.field_0);
    v1 = 18;
    for (i = 15; !*((short *)&(&g_4df9e0)[2 + 4 * i]); i = (&g_413410)[v1])
    {
        v1 -= 1;
        if (v1 == 2)
        {
            g_4dda40 = g_4dda40 + 23;
            return 2;
        }
    }
    g_4dda40 = g_4dda40 + (unsigned int)v1 * 3 + 0x11;
    return v1;
}



// Function: send_tree @ 0xa960
typedef struct struct_0 {
    char padding_0[2];
    unsigned short field_2;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern char g_0;
extern struct_1 g_4df9e0;
extern unsigned short g_4dfa20;
extern unsigned short g_4dfa22;
extern unsigned short g_4dfa24;
extern unsigned short g_4dfa26;
extern unsigned short g_4dfa28;
extern unsigned short g_4dfa2a;

struct_0 * send_tree(struct_0 *a0, unsigned int a1)
{
    unsigned int v2;  // ebp
    unsigned int v3;  // ecx
    unsigned long long v4;  // r12
    unsigned int v5;  // edx
    unsigned int v6;  // edi
    struct_0 *v7;  // rax
    unsigned int v8;  // r14d
    unsigned int v9;  // r13d
    unsigned int v0;  // [bp-0x3c]

    v2 = a0->field_2;
    v3 = (-(v2 < 1) & 131) + 7;
    if (a1 < 0)
        return a1;
    v4 = 0;
    v5 = 0;
    v6 = 0xffffffff;
    do
    {
        v7 = a0;
        v8 = v5 + 1;
        v9 = v2;
        v2 = v7[1 + v4].field_2;
        if (v8 < v3)
        {
            if (v2 != v9)
            {
                __unsupported_jumpkind_Ijk_NoDecode()
            }
            else
            {
                v5 = v8;
                continue;
            }
        }
        v0 = v5;
        if (v9)
        {
            if (v6 != v9)
            {
                sub_404290(*((short *)((char *)&g_4df9e0.field_0 + 4 * v9)), *((short *)((char *)&g_4df9e0.field_0 + 4 * v9 + 2)));
                v8 = v0;
            }
            sub_404290(g_4dfa20, g_4dfa22);
            v7 = sub_404290(v8 - 3, 2);
        }
        else if (v8 <= 10)
        {
            sub_404290(g_4dfa24, g_4dfa26);
            v7 = sub_404290(v0 - 2, 3);
        }
        else
        {
            sub_404290(g_4dfa28, g_4dfa2a);
            v7 = sub_404290(v0 - 10, 7);
        }
        if (!v2)
        {
            v3 = 138;
            v5 = 0;
            v6 = v9;
        }
        else if (v2 != v9)
        {
            v3 = 7;
            v5 = 0;
            v6 = v9;
        }
        else
        {
            v6 = v2 & 0xffff;
            v3 = 6;
            v5 = 0;
        }
    } while ((v4 += 1, a1 >= (unsigned int)v4));
    return v7;
}



// Function: send_all_trees @ 0xab20
typedef struct struct_0 {
    char padding_0[2];
    unsigned short field_2;
} struct_0;

extern char g_0;
extern char g_413410;
extern char g_413411;
extern char g_4df9e0;
extern struct_0 g_4dff80;
extern struct_0 g_4e0080;

struct_0 * send_all_trees(unsigned int a0, unsigned int a1, unsigned int a2)
{
    char *v1;  // r14
    unsigned long long v2;  // rax

    sub_404290(a0 - 0x101, 5);
    sub_404290(a1 - 1, 5);
    sub_404290(a2 - 4, 4);
    if (a2 > 0)
    {
        v1 = &g_413411;
        v2 = 16;
        while (1)
        {
            sub_404290(*((short *)&(&g_4df9e0)[2 + 4 * v2]), 3);
            if (v1 == &(&g_413410)[a2])
                break;
            v2 = *(v1);
            v1 += 1;
        }
    }
    sub_40a960(&g_4e0080.field_0, a0 - 1);
    return sub_40a960(&g_4dff80.field_0, a1 - 1);
}



// Function: compress_block @ 0xabd0
typedef struct struct_2 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_2;

typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_419240;
extern struct_0 g_4192c0;
extern struct_2 g_44c000;
extern char g_49d000;
extern unsigned int g_4dda54;
extern char g_4dda60;
extern unsigned int g_4dea60[4];
extern unsigned int g_4deae0[4];
extern char g_4deb60;
extern char g_4ded60;

unsigned long long compress_block(unsigned short *a0, unsigned long a1)
{
    unsigned long long v4;  // rbx
    unsigned int v5;  // r14d
    unsigned long long v6;  // r12
    unsigned long long v7;  // rax
    unsigned int v8;  // r14d
    unsigned int v9;  // ebp
    unsigned long v10;  // rax
    unsigned short *v11;  // rdx
    unsigned long v12;  // rax
    unsigned int v0;  // [bp-0x50], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0x48]
    unsigned int v2;  // [bp-0x44]

    if (!g_4dda54)
        return sub_404290(a0[0x200], a0[513]);
    v4 = 0;
    v5 = 0;
    v6 = 0;
    v7 = 0;
    while (1)
    {
        if (!((char)v7 & 7))
        {
            v8 = v5 + 1;
            v4 = (&g_4dda60)[v5];
            v5 = v8;
        }
        v9 = (unsigned int)v7 + 1;
        v10 = (&g_49d000)[v7];
        if (!((char)v4 & 1))
        {
            sub_404290(a0[2 * v10], a0[1 + 2 * v10]);
            goto LABEL_40ac21;
        }
        else
        {
            v0 = (char)v10;
            v1 = (&g_4ded60)[v10];
            v11 = &a0[0x202 + 2 * v1];
            sub_404290(*(v11), v11[1]);
            v12 = v1;
            if (*((int *)&(&g_4192c0.field_0)[2 * v12]))
                sub_404290(v0 - g_4deae0[v12], *((int *)&(&g_4192c0.field_0)[2 * v12]));
            v1 = (unsigned int)v6 + 1;
            v2 = (&g_44c000.field_0)[v6];
            v0 = (v2 <= 0xff ? (&g_4deb60)[v2 & 0xffff] : (&g_4deb60)[0x100 + ((v2 & 0xffff) >> 7)]);
            sub_404290(*((short *)(a1 + v0 * 4)), *((short *)(a1 + v0 * 4 + 2)));
            v6 = v1;
            if (!*((int *)&(&g_419240.field_0)[2 * v0]))
            {
LABEL_40ac21:
                v4 = _INSERT(v4, 0, (char)v4 >> 1);
                if (g_4dda54 <= v9)
                    break;
                else
                    goto LABEL_40ac2f;
            }
            else
            {
                v4 = _INSERT(v4, 0, (char)v4 >> 1);
                sub_404290(v2 - g_4dea60[v0], *((int *)&(&g_419240.field_0)[2 * v0]));
                if (g_4dda54 <= v9)
                    break;
LABEL_40ac2f:
                v7 = v9;
            }
        }
    }
    return sub_404290(a0[0x200], a0[513]);
}



// Function: ct_init @ 0xad80
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_419240;
extern struct_0 g_4192c0;
extern unsigned long long g_4dda20;
extern unsigned long long g_4dda28;
extern unsigned long long g_4dda30;
extern unsigned int g_4dea60;
extern unsigned int g_4deae0;
extern char g_4deb60;
extern char g_4deb61;
extern char g_4ded60;
extern char g_4ded61;
extern unsigned long long g_4df9c0;
extern unsigned short g_4df9ce;
extern unsigned short g_4df9d0;
extern unsigned short g_4df9d2;
extern unsigned long long g_4df9e0;
extern unsigned short g_4dfa80;
extern unsigned short g_4dfa82;
extern unsigned short g_4dfb00[2];
extern unsigned short g_4dfb02;
extern unsigned short g_4dfd42;
extern unsigned short g_4dff02;
extern unsigned short g_4dff62;
extern unsigned short g_4dff66;
extern unsigned short g_4dff6a;
extern unsigned short g_4dff6e;
extern unsigned short g_4dff72;
extern unsigned short g_4dff76;
extern unsigned short g_4dff7a;
extern unsigned short g_4dff7e;

unsigned int ct_init(unsigned long a0)
{
    unsigned long v1;  // 4113
    unsigned long long idx;  // rsi
    unsigned long long v11;  // r10
    unsigned long v12;  // rdx
    unsigned long iter;  // rax
    unsigned long long v14;  // rdi
    unsigned long long v15;  // r10
    unsigned long v16;  // rdx
    unsigned long iter1;  // rax
    unsigned long long iter2;  // rax
    unsigned short *j;  // rax
    unsigned short *k;  // rax
    unsigned long long v3;  // rdi
    unsigned short *l;  // rax
    unsigned long long index;  // rbx
    unsigned int v23;  // eax
    char v4;  // cl
    unsigned long long v5;  // r8
    unsigned long v6;  // rdx
    char *node;  // rax
    unsigned long long v8;  // rsi
    unsigned long long v9;  // rdi
    char v10;  // cl

    v1 = g_4dfa82;
    g_4dda20 = a0;
    g_4dda28 = 0;
    g_4dda30 = 0;
    if ((unsigned short)v1)
        return v23;
    idx = 0;
    v3 = 0;
    do
    {
        v4 = *((int *)&(&g_4192c0.field_0)[2 * idx]);
        (&g_4deae0)[idx] = v3;
        v5 = 1 << (v4 & 31);
        if ((unsigned int)v5 > 0)
        {
            v6 = (int)v3;
            node = &(&g_4ded60)[v6];
            do
            {
                *(node) = idx & 0xffffffff;
                node += 1;
            } while (node != &(&g_4ded61)[v6 + (v5 - 1 & 0xffffffff)]);
            v3 += v5;
        }
    } while ((idx += 1, idx != 28));
    v8 = 0;
    v9 = 0;
    (&g_4ded60)[(unsigned int)(v3 - 1)] = 28;
    do
    {
        v10 = *((int *)&(&g_419240.field_0)[2 * v8]);
        (&g_4dea60)[v8] = v9;
        v11 = 1 << (v10 & 31);
        if ((unsigned int)v11 > 0)
        {
            v12 = (int)v9;
            iter = &(&g_4deb60)[v12];
            do
            {
                *((char *)iter) = v8 & 0xffffffff;
                iter += 1;
            } while (iter != &(&g_4deb61)[v12 + (v11 - 1 & 0xffffffff)]);
            v9 += v11;
        }
    } while ((v8 += 1, v8 != 16));
    v14 = (int)((unsigned int)v9) >> 7;
    do
    {
        (&g_4dea60)[v8] = (unsigned int)v14 * 128;
        v15 = 1 << ((char)(*((int *)&(&g_419240.field_0)[2 * v8]) - 7) & 31);
        if ((unsigned int)v15 > 0)
        {
            v16 = (int)v14;
            iter1 = &(&g_4deb60)[v16];
            do
            {
                *((char *)(iter1 + 0x100)) = v8 & 0xffffffff;
                iter1 += 1;
            } while (iter1 != &(&g_4deb61)[v16 + (v15 - 1 & 0xffffffff)]);
            v14 += v15;
        }
    } while ((v8 += 1, v8 != 30));
    iter2 = &g_4df9c0;
    do
    {
        iter2 += 2;
        *((unsigned short *)(iter2 - 2)) = 0;
    } while (iter2 != &g_4df9e0);
    j = &g_4dfb02;
    do
    {
        j += 2;
        *((unsigned short *)((char *)j - 4)) = 8;
    } while (j != &g_4dfd42);
    k = &g_4dfd42;
    do
    {
        k += 2;
        *((unsigned short *)((char *)k - 4)) = 9;
    } while (k != &g_4dff02);
    g_4df9d2 = g_4df9d2 + 112;
    l = &g_4dff02;
    do
    {
        l += 2;
        *((unsigned short *)((char *)l - 4)) = 7;
    } while (l != &g_4dff62);
    g_4df9ce = g_4df9ce + 24;
    g_4dff66 = 8;
    g_4dff6a = 8;
    g_4dff7e = 8;
    index = 0;
    g_4dff62 = 8;
    g_4dff6e = 8;
    g_4dff72 = 8;
    g_4dff76 = 8;
    g_4dff7a = 8;
    g_4df9d0 = g_4df9d0 + 152;
    sub_40a500(&g_4dfb00[0], 287);
    do
    {
        (&g_4dfa82)[2 * index] = 5;
        (&g_4dfa80)[2 * index] = sub_404380(index, 5);
        index += 1;
    } while (index != 30);
    return sub_409f30();
}



// Function: flush_block @ 0xb070
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_419380;
extern unsigned int g_4193a4;
extern struct_0 g_4193c0;
extern unsigned int g_4193e4;
extern struct_0 *g_4dda20;
extern unsigned long long g_4dda28;
extern unsigned long long g_4dda30;
extern unsigned long long g_4dda38;
extern unsigned long long g_4dda40;
extern char g_4dda49;
extern unsigned int g_4dda4c;
extern char g_4dda60;
extern unsigned int g_4dfa80;
extern unsigned short g_4dfb00;
extern unsigned int g_4dff80;
extern unsigned short g_4e0080;

long long flush_block(char *a0, unsigned long long a1, unsigned int a2, unsigned int a3)
{
    unsigned long long v1;  // r15
    unsigned int v2;  // eax
    unsigned long long v3;  // rdi
    unsigned long long v4;  // rcx
    unsigned long long v5;  // rax
    unsigned long long v6;  // rax
    unsigned long v7;  // rax

    (&g_4dda60)[g_4dda4c] = g_4dda49;
    if (g_4dda20->field_0 == 0xffff)
        sub_40a480();
    v1 = a1 + 4;
    sub_40a5a0(&g_4193c0.field_0);
    sub_40a5a0(&g_419380.field_0);
    v2 = sub_40a8c0();
    g_4dda28 = g_4dda28 + a1;
    v3 = g_4dda40 + 10 >> 3;
    v4 = g_4dda38 + 10 >> 3;
    if (v3 <= v4)
    {
        v5 = v3;
        v6 = v5;
    }
    else
    {
        v6 = v4;
    }
    if (v1 > v6 || !a0)
    {
        if (v5 < v4)
        {
            sub_404290(a3 + 4, 3);
            sub_40ab20(g_4193e4 + 1, g_4193a4 + 1, v2 + 1);
            sub_40abd0(&g_4e0080, &g_4dff80);
            g_4dda30 = g_4dda30 + g_4dda40 + 3;
        }
        else
        {
            sub_404290(a3 + 2, 3);
            sub_40abd0(&g_4dfb00, &g_4dfa80);
            g_4dda30 = g_4dda30 + g_4dda38 + 3;
        }
        sub_409f30();
        if (a3)
            goto LABEL_40b1df;
LABEL_40b175:
        v7 = g_4dda30;
        if (a2 && (char)v7 & 7)
        {
            sub_404290(0, 3);
            g_4dda30 = g_4dda30 + 10 & 0xfffffffffffffff8;
            sub_4044a0(a0, 0, 1);
            v7 = g_4dda30;
        }
    }
    else
    {
        sub_404290(a3, 3);
        g_4dda30 = (g_4dda30 + 10 & 0xfffffffffffffff8) + v1 * 8;
        sub_4044a0(a0, a1, 1);
        sub_409f30();
        if (!a3)
            goto LABEL_40b175;
LABEL_40b1df:
        sub_4043b0();
        v7 = g_4dda30 + 7;
        g_4dda30 = v7;
    }
    return (long long)(v7) >> 3;
}



// Function: ct_tally @ 0xb290
extern char g_0;
extern int g_41901c;
extern char g_419240;
extern unsigned int g_41a06c;
extern unsigned long long g_41a078;
extern unsigned short g_44c000;
extern char g_49d000;
extern char g_4dda48;
extern char g_4dda49;
extern unsigned int g_4dda4c;
extern unsigned int g_4dda50;
extern unsigned int g_4dda54;
extern char g_4dda60;
extern char g_4deb60;
extern char g_4ded60;
extern unsigned short g_4dff80[4];
extern unsigned short g_4e0080[4];

unsigned int ct_tally(unsigned int a0, unsigned int a1)
{
    unsigned long v1;  // rax
    char v2;  // al
    unsigned long long v11;  // rdx
    unsigned int v12;  // eax
    unsigned int v3;  // edx
    unsigned long v4;  // rsi
    unsigned long long idx;  // rcx
    unsigned long index;  // rcx
    unsigned long v7;  // rax
    char v8;  // cl
    unsigned long long i;  // rdx
    unsigned long long v10;  // rcx

    v1 = g_4dda54;
    g_4dda54 = (unsigned int)v1 + 1;
    (&g_49d000)[v1] = a1;
    v2 = g_4dda48;
    if (!a0)
    {
        g_4e0080[2 * a1] = g_4e0080[2 * a1] + 1;
    }
    else
    {
        v3 = a0 - 1;
        v4 = (&g_4ded60)[a1];
        g_4e0080[0x202 + 2 * v4] = g_4e0080[0x202 + 2 * v4] + 1;
        idx = (v3 <= 0xff ? (&g_4deb60)[v3] : (&g_4deb60)[0x100 + ((int)(v3) >> 7)]);
        g_4dff80[2 * idx] = g_4dff80[2 * idx] + 1;
        index = g_4dda50;
        g_4dda50 = (unsigned int)index + 1;
        (&g_44c000)[index] = v3;
        g_4dda49 = g_4dda49 | v2;
    }
    g_4dda48 = v2 * 2;
    if (!((char)g_4dda54 & 7))
    {
        v7 = g_4dda4c;
        v8 = g_4dda49;
        g_4dda48 = 1;
        g_4dda49 = 0;
        g_4dda4c = (unsigned int)v7 + 1;
        (&g_4dda60)[v7] = v8;
    }
    if (g_41901c > 2 && !((unsigned short)g_4dda54 & 0xfff))
    {
        i = 0;
        v10 = g_4dda54 * 8;
        do
        {
            v11 = i + 4;
            v10 += *((short *)((char *)&g_4dff80[0] + i)) * (*((int *)&(&g_419240)[i]) + 5);
            i = v11;
        } while (i != 120);
        if (g_4dda54 >> 1 > g_4dda50 && g_41a06c - g_41a078 >> 1 > v10 >> 3)
            return 1;
    }
    v12 = 1;
    if (g_4dda54 != 0x7fff)
        v12 = g_4dda50 == 0x8000;
    return v12;
}



// Function: fillbuf @ 0xb450
void fillbuf(unsigned long a0)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_40b456();
    return;
}



// Function: init_getbits @ 0xb520
extern char g_0;
extern unsigned int g_4e098c;
extern unsigned int g_4e0990;
extern unsigned short g_4e0994;

void init_getbits(void)
{
    g_4e0990 = 0;
    g_4e0994 = 0;
    g_4e098c = 0;
    sub_40b450(16);
    return;
}



// Function: huf_decode_start @ 0xb550
extern char g_0;
extern unsigned int g_4e0ba0;

void huf_decode_start(void)
{
    sub_40b520();
    g_4e0ba0 = 0;
    return;
}



// Function: decode_start @ 0xb570
extern char g_0;
extern unsigned int g_4e0984;
extern unsigned int g_4e0988;

void decode_start(void)
{
    sub_40b550();
    g_4e0988 = 0;
    g_4e0984 = 0;
    return;
}



// Function: getbits @ 0xb5a0
extern unsigned short g_4e0994;

int getbits(unsigned int a0)
{
    sub_40b450(a0);
    return (short)(g_4e0994) >> ((char)(16 - a0) & 31);
}



// Function: decode_p @ 0xb5c0
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_41b880;
extern unsigned short g_4e0994;
extern struct_0 g_4e09a0;
extern char g_4e0bc0;

int decode_p(void)
{
    unsigned long v1;  // r12
    unsigned int v2;  // eax

    v1 = (&g_4e09a0.field_0)[(char)(g_4e0994 >> 8)];
    if ((unsigned int)v1 > 13)
    {
        v2 = 128;
        while (1)
        {
            if (g_4e0994 & v2)
            {
                v2 >>= 1;
                v1 = (&g_41b880.field_0)[0x8000 + v1];
                if ((unsigned int)v1 <= 13)
                    break;
            }
            else
            {
                v1 = (&g_41b880.field_0)[v1];
                v2 >>= 1;
                if ((unsigned int)v1 <= 13)
                    break;
            }
        }
    }
    sub_40b450((&g_4e0bc0)[v1 & 0xffffffff]);
    if ((unsigned int)v1)
        return (unsigned int)(1 << ((char)((unsigned int)v1 - 1) & 31)) + sub_40b5a0((unsigned int)v1 - 1);
    return v1;
}



// Function: read_c_len @ 0xb980
void read_c_len(void)
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
    sub_40b98f(9);
    return;
}



// Function: read_pt_len @ 0xbb40
void read_pt_len(unsigned long a0, unsigned long a1, unsigned long a2)
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
    sub_40bb4f();
    return;
}



// Function: decode_c @ 0xbd50
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_41b880;
extern struct_0 g_44c000;
extern char g_45c000;
extern unsigned short g_4e0994;
extern unsigned int g_4e0ba0;

unsigned int decode_c(void)
{
    unsigned int v1;  // eax
    unsigned long v2;  // r12
    unsigned int v3;  // eax

    v1 = g_4e0ba0;
    if (!v1)
    {
        g_4e0ba0 = sub_40b5a0(16);
        if (!g_4e0ba0)
            return 0x1fe;
        sub_40bb40(19, 5, 3);
        sub_40b980();
        sub_40bb40(14, 4, 0xffffffff);
        v1 = g_4e0ba0;
    }
    g_4e0ba0 = v1 - 1;
    v2 = (&g_44c000.field_0)[_INSERT(g_4e0994, 0, g_4e0994 >> 4) & 0xfff];
    if ((unsigned int)v2 > 509)
    {
        v3 = 8;
        while (1)
        {
            if (g_4e0994 & v3)
            {
                v3 >>= 1;
                v2 = (&g_41b880.field_0)[0x8000 + v2];
                if ((unsigned int)v2 <= 509)
                    break;
            }
            else
            {
                v2 = (&g_41b880.field_0)[v2];
                v3 >>= 1;
                if ((unsigned int)v2 <= 509)
                    break;
            }
        }
    }
    sub_40b450((&g_45c000)[v2 & 0xffffffff]);
    return v2;
}



// Function: unlzh @ 0xbfd0
extern char g_0;
extern unsigned int g_41b010;
extern unsigned int g_41b014;
extern long long g_43c000;
extern unsigned int g_4e0984;

unsigned int unlzh(unsigned int a0, unsigned int a1)
{
    unsigned int v1;  // eax

    g_41b014 = a0;
    g_41b010 = a1;
    sub_40b570();
    while (!g_4e0984)
    {
        v1 = sub_40be40(0x2000, &g_43c000);
        if (v1)
            sub_40d6c0(a1, &g_43c000, v1);
    }
    return 0;
}



// Function: unlzw @ 0xc040
extern char g_0;
extern void g_419024;
extern unsigned int g_4193e8;
extern long long stderr;
extern unsigned int g_41b008;
extern unsigned int g_41b00c;
extern long long g_41b420;
extern unsigned long long g_41b828;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_41b85c;
extern void g_41b880;
extern unsigned long long g_41b888;
extern unsigned long long g_41b978;
extern char g_43c000;
extern void g_45bffd;
extern void g_45bffe;
extern char g_45c000;
extern char g_49d000;
extern char g_49d001;

unsigned long long unlzw(unsigned int a0, unsigned int a1)
{
    unsigned long v16;  // rax
    unsigned int v17;  // eax
    unsigned int v26;  // r8d
    unsigned long long v27;  // r15
    long long v28;  // r13
    char *l;  // rax
    char *v30;  // rax
    unsigned long v31;  // rax
    unsigned long long v32;  // rcx
    unsigned long long v33;  // r13
    unsigned long long v34;  // r15
    unsigned long long idx;  // rax
    unsigned long v18;  // 4193
    void* iter;  // rdx
    char v37;  // sil
    void* node;  // r14
    unsigned int m;  // edx
    unsigned int v40;  // ecx
    unsigned long v41;  // r15
    unsigned int v42;  // r12d
    unsigned long long v43;  // r12
    unsigned long n;  // r14
    unsigned long v45;  // r15
    unsigned long v19;  // rsi
    unsigned long v46;  // r15
    unsigned int v47;  // r8d
    unsigned long long v48;  // r13
    unsigned int v49;  // edi
    unsigned long long *iter1;  // rdi
    unsigned long long v51;  // rcx
    unsigned long long v20;  // r13
    unsigned long long *iter2;  // rdi
    unsigned long long v22;  // rcx
    unsigned long long v23;  // rax
    unsigned long long j;  // rax
    unsigned long long v25;  // rax
    unsigned long long v0;  // [bp-0xa0]
    unsigned int v1;  // [bp-0x98], Other Possible Types: unsigned long long
    unsigned int v2;  // [bp-0x8c]
    unsigned long long v3;  // [bp-0x88]
    unsigned long v4;  // [bp-0x80]
    unsigned long v5;  // [bp-0x78]
    unsigned long long v6;  // [bp-0x70]
    char v7;  // [bp-0x65]
    unsigned int v8;  // [bp-0x60], Other Possible Types: unsigned long long
    unsigned long long v9;  // [bp-0x58]
    unsigned long long v10;  // [bp-0x50]
    unsigned int v11;  // [bp-0x48]
    unsigned int v12;  // [bp-0x48]
    unsigned int v13;  // [bp-0x44]
    unsigned int v14;  // [bp-0x40]

    v16 = g_41b008;
    if ((unsigned int)v16 < g_41b00c)
    {
        g_41b008 = (unsigned int)v16 + 1;
        v17 = (&g_49d000)[v16];
    }
    else
    {
        v17 = sub_40d830(0);
    }
    *((unsigned int *)&g_419024) = v17;
    g_4193e8 = v17 & 128;
    if ((char)v17 & 96)
    {
        if (!g_41b85c)
        {
            sub_40f630(stderr, "\n%s: %s: warning, unknown flags 0x%x\n", g_41b850, &g_41b420);
            if (!g_41b84c)
            {
LABEL_40c0b9:
                g_41b84c = 2;
            }
        }
        else if (!g_41b84c)
        {
            goto LABEL_40c0b9;
        }
        v17 = *((int *)&g_419024);
    }
    *((unsigned int *)&g_419024) = v17 & 31;
    v9 = 1 << ((char)*((int *)&g_419024) & 63);
    if (*((int *)&g_419024) > 16)
    {
        sub_40f630(stderr, "\n%s: %s: compressed with %d bits, can only handle %d bits\n", g_41b850, &g_41b420);
        g_41b84c = 1;
        return 1;
    }
    v18 = g_4193e8;
    *((unsigned long long *)&g_41b880) = 0;
    v19 = g_41b00c;
    g_41b978 = 0;
    v20 = g_41b008 * 8;
    iter2 = &g_41b888;
    v13 = g_41b00c;
    v0 = 0x101 - ((unsigned int)v18 < 1);
    for (v22 = 31; v22; iter2 += 1)
    {
        v22 -= 1;
        *(iter2) = 0;
    }
    v23 = 0xff;
    do
    {
        j = v23;
        (&g_43c000)[j] = j;
        v23 = j - 1;
    } while (j >= 1);
    v12 = 9;
    v3 = 0x1ff;
    v26 = 0;
    v27 = v25;
    v14 = 0x1ff;
    v2 = 0;
    while (1)
    {
LABEL_40c19c:
        v28 = (long long)(v20) >> 3;
        if ((int)v28 > (unsigned int)v19)
        {
LABEL_40c1a9:
            g_41b00c = 0;
LABEL_40c1ba:
            v1 = v26;
            v26 = v1;
            v13 = sub_40d2b0(a0);
            if (v13 == 0xffffffff)
                sub_40d600(); /* do not return */
            v31 = v13;
            g_41b828 = g_41b828 + v31;
            v19 = g_41b00c + v31;
            g_41b00c = v19;
            goto LABEL_40c1f9;
        }
        while (1)
        {
            v19 -= v28;
            if ((unsigned int)v19 > 0)
            {
                l = &g_49d000;
                do
                {
                    v30 = l + 1;
                    *(v30 - 1) = l[(int)v28];
                    l = v30;
                } while (&(&g_49d001)[v19 - 1 & 0xffffffff] != l);
            }
            g_41b00c = v19;
            if ((unsigned int)v19 <= 63)
                goto LABEL_40c1ba;
LABEL_40c1f9:
            v32 = v19 & 0xffffffff;
            if (v13)
            {
                v20 = 0;
                v6 = (v32 - ((v19 & 0xffffffff) % v12 & 0xffffffff)) * 8;
                if (v6 > 0)
                    goto LABEL_40c22b;
                goto LABEL_40c19c;
            }
            v6 = v32 * 8 - (v12 - 1);
            if (v6 <= 0)
                goto LABEL_40c5a5;
LABEL_40c22b:
            if (v0 <= v3)
                break;
            v48 = 0xffffffffffffffff;
LABEL_40c39f:
            v49 = v12;
            v12 = v49 + 1;
            v3 = v9;
            if (*((int *)&g_419024) != v12)
                v3 = (1 << ((char)v12 & 63)) - 1;
            v28 = (long long)(v48 + v49 * 8 - (v49 * 8 + v48) % (v49 * 8)) >> 3;
            v14 = (1 << ((char)v12 & 31)) - 1;
            if ((int)v28 > (unsigned int)v19)
                goto LABEL_40c1a9;
        }
        v33 = 0;
        v4 = v14;
        v5 = v12;
        while (1)
        {
            v1 = v27;
            v20 = v33 + v5;
            v34 = (long long)((&g_49d000)[1 + ((long long)(v33) >> 3)] * 0x100 | (&g_49d000)[2 + ((long long)(v33) >> 3)] * 0x10000 | (&g_49d000)[(long long)(v33) >> 3]) >> ((char)v33 & 7 & 63) & v4;
            if (v27 != 0xffffffffffffffff)
                break;
            if (v34 > 0xff)
                sub_40d510("corrupt input."); /* do not return */
            v2 = v34;
            v47 = v26 + 1;
            (&g_45c000)[v26] = v34;
            v26 = v47;
            if (v20 >= v6)
            {
LABEL_40c48f:
                v26 = v47;
                v27 = v34;
                if (v13)
                {
                    v19 = g_41b00c;
                    goto LABEL_40c19c;
                }
LABEL_40c5a5:
                if (v26 > 0)
                {
                    sub_40d6c0(a1, &g_45c000, v26);
                    return 0;
                }
                return 0;
            }
LABEL_40c385:
            v33 = v20;
            v27 = v34;
            if (v3 < v0)
            {
                v19 = g_41b00c;
                v48 = v33 - 1;
                goto LABEL_40c39f;
            }
        }
        if (v34 != 0x100)
        {
            if (v34 >= v0)
            {
                if (v34 > v0)
                {
                    if (v26 > 0)
                        sub_40d6c0(a1, &g_45c000, v26);
                    sub_40d510(); /* do not return */
                }
            }
            else
            {
                idx = v34;
                iter = &g_45bffe;
                while (1)
                {
LABEL_40c2f7:
                    if (idx <= 0xff)
                    {
                        v37 = (&g_43c000)[idx];
                        node = iter - 1;
                        *((char *)iter - 1) = v37;
                        m = &g_45bffe - node;
                        v7 = v37;
                        v40 = v26 + m;
                        v2 = v7;
                        if (v40 <= 0x3ffff)
                        {
                            v8 = v40;
                            memcpy(&(&g_45c000)[v26], node, m);
                            v26 = v8;
                        }
                        else
                        {
                            v8 = v20;
                            v10 = v34;
                            v41 = v26;
                            do
                            {
                                v42 = 0x40000 - (unsigned int)v41;
                                v43 = (m < v42 ? m : v42);
                                n = (int)v43;
                                v45 = v41;
                                if ((unsigned int)v43 > 0)
                                {
                                    v45 = v41 + v43;
                                    memcpy(&(&g_45c000)[v41], node, n);
                                }
                                v41 = v45;
                                if ((unsigned int)v41 > 0x3ffff)
                                {
                                    v46 = 0;
                                    sub_40d6c0(a1, &g_45c000, v41 & 0xffffffff);
                                    v41 = v46;
                                }
                                node += n;
                                m = &g_45bffe - node;
                            } while (m > 0);
                            v26 = v41;
                            v20 = v8;
                            v34 = v10;
                        }
                        if (v9 > v0)
                        {
                            *((unsigned short *)&(&g_41b880)[2 * v0]) = v1;
                            (&g_43c000)[v0] = v7;
                            v0 += 1;
                        }
                        v47 = v26;
                        if (v20 >= v6)
                            goto LABEL_40c48f;
                        goto LABEL_40c385;
                    }
LABEL_40c2e8:
                    iter -= 1;
                    *((char *)iter) = (&g_43c000)[idx];
                    idx = *((short *)&(&g_41b880)[2 * idx]);
                }
            }
            goto LABEL_40c2c8;
        }
        else if (!g_4193e8)
        {
            idx = 0x100;
            iter = &g_45bffe;
            if (v0 != 0x100)
                goto LABEL_40c2e8;
LABEL_40c2c8:
            iter = &g_45bffd;
            g_45bffd = v2;
            idx = v1;
            goto LABEL_40c2f7;
        }
        else
        {
            *((unsigned long long *)&g_41b880) = 0;
            iter1 = &g_41b888;
            g_41b978 = 0;
            for (v51 = 31; v51; iter1 += 1)
            {
                v51 -= 1;
                *(iter1) = 0;
            }
            v14 = 0x1ff;
            v0 = 0x100;
            v19 = g_41b00c;
            v11 = 9;
            v3 = 0x1ff;
            v27 = v1;
            v20 = v20 - 1 + v12 * 8 - (v20 - 1 + v12 * 8) % (v12 * 8);
            v12 = v11;
        }
    }
}



// Function: read_byte @ 0xc820
extern unsigned int g_41b008;
extern char g_41b00c;
extern char g_49d000;

long long read_byte(void)
{
    unsigned long v1;  // rax

    v1 = g_41b008;
    if ((unsigned int)v1 >= *((int *)&g_41b00c))
        return sub_40c848();
    g_41b008 = (unsigned int)v1 + 1;
    return (&g_49d000)[v1];
}



// Function: read_tree @ 0xc870
extern char g_0;
extern int g_4e0c80[4];
extern unsigned int g_4e0d00;
extern char g_4e0d80;
extern char g_4e0d81;
extern int g_4e0e80;
extern unsigned long long g_4e0e88;

unsigned long long read_tree(void)
{
    unsigned int v1;  // ebp
    unsigned long long v2;  // rbx
    unsigned long long idx;  // rax
    unsigned long long index;  // r12
    unsigned int v13;  // r13d
    int v14;  // ebp
    unsigned long v15;  // rbx
    unsigned long iter;  // r15
    unsigned long v17;  // rax
    unsigned int i;  // ebp
    unsigned long long idx1;  // r12
    unsigned int v6;  // ebx
    unsigned long long v7;  // rbp
    char v8;  // al
    int v9;  // edx
    unsigned long v10;  // rax

    v1 = 4;
    v2 = 0;
    g_4e0e88 = 0;
    do
    {
        i = v1;
        v2 = v2 * 0x100 | sub_40c820() & 0xff;
        g_4e0e88 = v2;
        v1 = i - 1;
    } while (i != 1);
    g_4e0e80 = (int)sub_40c820() & 0xff;
    if (g_4e0e80 - 1 > 24)
        sub_40d510("invalid compressed data -- Huffman code bit length out of range"); /* do not return */
    if (g_4e0e80)
    {
        idx1 = 1;
        v6 = 1;
        v7 = 0;
        do
        {
            v8 = sub_40c820();
            v9 = g_4e0e80;
            v10 = v8;
            g_4e0c80[idx1] = v10;
            if ((unsigned int)v10 > (unsigned int)(v6 - (g_4e0e80 == (unsigned int)idx1)))
                sub_40d510("too many leaves in Huffman tree"); /* do not return */
        } while ((idx1 += 1, v7 = (unsigned long long)(v7 + v10), v6 = (unsigned int)((unsigned long long)v6 - v10) * 2 + 1, g_4e0e80 >= (unsigned int)idx1));
        if ((unsigned int)v7 > 0xff)
            sub_40d510("too many leaves in Huffman tree"); /* do not return */
        idx = g_4e0e80;
        g_4e0c80[g_4e0e80] = g_4e0c80[g_4e0e80] + 1;
        if (g_4e0e80 > 0)
        {
            index = 1;
            v13 = 0;
            do
            {
                v14 = g_4e0c80[index];
                (&g_4e0d00)[index] = v13;
                if (v14 > 0)
                {
                    v15 = v13;
                    iter = &(&g_4e0d80)[v15];
                    v17 = v14 - 1;
                    do
                    {
                        iter += 1;
                        *((char *)(iter - 1)) = sub_40c820();
                    } while (iter != &(&g_4e0d81)[v15 + v17]);
                    v9 = g_4e0e80;
                    v13 = v13 + (unsigned int)v17 + 1;
                }
            } while ((index += 1, v9 >= (unsigned int)index));
            idx = v9;
        }
    }
    else
    {
        g_4e0c80[0] = (int)g_4e0c80 + 1;
        idx = 0;
    }
    g_4e0c80[idx] = g_4e0c80[idx] + 1;
    return idx;
}



// Function: unpack @ 0xc9f0
extern char g_0;
extern unsigned int g_41b004;
extern unsigned int g_41b010;
extern unsigned int g_41b014;
extern unsigned int g_41b820;
extern char g_43c000;
extern char g_45c000;
extern unsigned int g_4e0be0;
extern unsigned long long g_4e0be8;
extern void g_4e0bf0;
extern unsigned int g_4e0c00[4];
extern unsigned int g_4e0c80[4];
extern unsigned int g_4e0d00[4];
extern char g_4e0d80;
extern char g_4e0e80;
extern char g_4e0e88;

unsigned long long unpack(unsigned int a0, unsigned int a1)
{
    unsigned long long v3;  // r15
    unsigned int v4;  // esi
    unsigned long v13;  // rdx
    char v14;  // al
    unsigned int i;  // edx
    unsigned int v6;  // eax
    unsigned int v7;  // r12d
    unsigned long v8;  // r13
    unsigned long v9;  // rbx
    unsigned long v11;  // rbx
    unsigned long v12;  // rcx
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]

    v3 = 0;
    g_41b010 = a1;
    g_41b014 = a0;
    sub_40c870();
    sub_40c740();
    v4 = *((int *)&g_4e0bf0);
    g_4e0be0 = 0;
    g_4e0be8 = 0;
    v0 = (unsigned int)(1 << ((char)*((int *)&g_4e0bf0) & 31)) - 1;
    i = 0;
    v1 = g_4e0c80[*((int *)&g_4e0e80)] - 1;
    while (1)
    {
        if (i >= v4)
        {
            v6 = (unsigned int)(v3 >> ((char)(i - v4) & 63)) & v0;
            v7 = (&g_45c000)[v6];
            if (v7)
            {
                v6 >>= (char)(v4 - v7 & 0xffffffff) & 31;
            }
            else
            {
                v8 = v0;
                if (v6 < g_4e0c00[v4])
                {
                    v9 = v4 + 1;
                    while (1)
                    {
                        v7 = v9;
                        v8 = v8 * 2 + 1;
                        if ((unsigned int)v9 > i)
                        {
                            do
                            {
                                g_4e0be8 = v3 * 0x100 | sub_40c820() & 0xff;
                                v3 = g_4e0be8;
                                i = g_4e0be0 + 8;
                                g_4e0be0 = i;
                            } while (i < (unsigned int)v9);
                        }
                        v11 = v9 + 1;
                        v6 = (unsigned int)(v3 >> ((char)(i - v7) & 63)) & (unsigned int)v8;
                        if (g_4e0c00[v9] <= v6)
                            break;
                        i = g_4e0be0;
                        v9 = v11;
                    }
                }
                else
                {
                    v7 = v4;
                }
            }
            if (v6 == v1 && *((int *)&g_4e0e80) == v7)
                break;
            v12 = v7;
            v13 = g_41b004;
            v14 = (&g_4e0d80)[g_4e0d00[v12] + v6];
            g_41b004 = (unsigned int)v13 + 1;
            (&g_43c000)[v13] = v14;
            if (g_41b004 == 0x8000)
                sub_40d7e0(&g_43c000, g_41b004, v13, v12);
            v3 = g_4e0be8;
            i = g_4e0be0 - v7;
            g_4e0be0 = i;
        }
        else
        {
            g_4e0be8 = v3 * 0x100 | sub_40c820() & 0xff;
            v3 = g_4e0be8;
            i = g_4e0be0 + 8;
            g_4e0be0 = i;
        }
        v4 = *((int *)&g_4e0bf0);
    }
    sub_40d7e0();
    if (g_41b820 != *((long long *)&g_4e0e88))
        sub_40d510("invalid compressed data--length error"); /* do not return */
    return 0;
}



// Function: check_zipfile @ 0xcc20
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[2];
    char field_6;
    char padding_7[1];
    char field_8;
    char padding_9[17];
    unsigned short field_1a;
    unsigned short field_1c;
} struct_0;

extern char g_0;
extern unsigned int g_419020;
extern long long stderr;
extern unsigned int g_41b008;
extern unsigned int g_41b00c;
extern unsigned int g_41b014;
extern unsigned int g_41b84c;
extern char g_49d000;
extern unsigned int g_4e0e98;
extern unsigned int g_4e0e9c;
extern unsigned int g_4e0ea0;

unsigned int check_zipfile(unsigned int a0)
{
    struct_0 *v1;  // rax
    unsigned long v2;  // 4168
    unsigned long v3;  // rax

    g_41b014 = a0;
    v1 = &(&g_49d000)[g_41b008];
    v2 = g_41b00c;
    g_41b008 = v1->field_1a + v1->field_1c + 30 + g_41b008;
    if (g_41b008 <= (unsigned int)v2 && v1->field_0 == 67324752)
    {
        g_419020 = v1->field_8;
        if (!((char)g_419020 & 247))
        {
            v3 = v1->field_6;
            g_4e0ea0 = (unsigned int)v3 & 0xff & 1;
            if (!((char)v3 & 1))
            {
                g_4e0e9c = 1;
                g_4e0e98 = _INSERT(v3, 0, (char)v3 >> 3) & 1;
                return 0;
            }
        }
    }
    sub_40f630(stderr);
    g_41b84c = 1;
    return 1;
}



// Function: unzip @ 0xcd20
extern char g_0;
extern char g_419020;
extern long long stderr;
extern unsigned int g_41b004;
extern unsigned int g_41b008;
extern char g_41b00c;
extern unsigned int g_41b010;
extern unsigned int g_41b014;
extern long long g_41b420;
extern unsigned int g_41b820;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_41b858;
extern unsigned int g_41b85c;
extern unsigned int g_41b860;
extern char g_43c000;
extern long long g_45c000;
extern char g_49d000;
extern char g_49d001;
extern char g_49d002;
extern char g_49d003;
extern unsigned int g_49d00e;
extern unsigned int g_49d012;
extern unsigned int g_49d016;
extern unsigned long long g_4e0e90;
extern unsigned int g_4e0e98;
extern unsigned int g_4e0e9c;
extern unsigned int g_4e0ea0;

unsigned long long unzip(unsigned int a0, unsigned int a1, unsigned int a2, unsigned long a3)
{
    unsigned long long v6;  // rbp
    long long v7;  // r12
    unsigned int v16;  // edx
    unsigned int v17;  // rcx
    void* iter;  // rbx
    unsigned long v20;  // rax
    void* node;  // rbx
    unsigned long v22;  // rax
    unsigned int v23;  // r13d
    long long v8;  // rcx
    long long v9;  // r13
    long long v10;  // r13
    unsigned long v11;  // rax
    char v12;  // al
    unsigned int v13;  // ecx
    unsigned int v14;  // rdi
    unsigned int v15;  // rsi
    unsigned int v0;  // [bp-0x48]
    unsigned int v1;  // [bp-0x44]
    char v2;  // [bp-0x40]
    unsigned int v3;  // [bp-0x3c]
    char v4;  // [bp-0x38]

    g_41b014 = a0;
    g_41b010 = a1;
    sub_40d1f0(0, 0);
    if (g_4e0e9c)
    {
        if (!g_4e0e98)
        {
            v6 = g_49d00e;
            v7 = g_49d016;
        }
        else
        {
            v7 = 0;
            v6 = 0;
        }
        if (*((int *)&g_419020) == 8)
            goto LABEL_40ce42;
        if (*((int *)&g_419020))
            sub_40d510("internal error, invalid method"); /* do not return */
        v8 = g_49d012;
        v9 = g_49d016;
        if (g_4e0ea0)
            v8 = g_49d012 - 12;
        if (g_49d016 != v8)
        {
            sub_40f630(stderr, "len %lu, siz %lu\n", g_49d016);
            sub_40d510("invalid compressed data--length mismatch"); /* do not return */
        }
        if (g_49d016)
        {
            while (1)
            {
                v10 = v9;
                v11 = g_41b008;
                if ((unsigned int)v11 < *((int *)&g_41b00c))
                {
                    g_41b008 = (unsigned int)v11 + 1;
                    v12 = (&g_49d000)[v11];
                }
                else
                {
                    v12 = sub_40d830(0);
                }
                v13 = g_41b004 + 1;
                (&g_43c000)[g_41b004] = v12;
                g_41b004 = v13;
                if (g_41b004 != 0x8000)
                {
                    v9 = v10 - 1;
                    if (v10 == 1)
                        break;
                }
                else
                {
                    sub_40d7e0();
                    v9 = v10 - 1;
                    if (v10 == 1)
                        break;
                }
            }
        }
        sub_40d7e0();
        if (g_4e0e9c)
            goto LABEL_40d045;
        iter = &v0;
        while (1)
        {
            v20 = g_41b008;
            if ((unsigned int)v20 < *((int *)&g_41b00c))
            {
                iter += 1;
                g_41b008 = (unsigned int)v20 + 1;
                *((char *)iter - 1) = (&g_49d000)[v20];
                if (iter == &v2)
                    break;
            }
            else
            {
                iter += 1;
                *((char *)iter - 1) = sub_40d830(0);
                if (iter == &v2)
                    break;
            }
        }
        v6 = v0;
        v7 = v1;
    }
    else
    {
        if (*((int *)&g_419020) != 8)
            sub_40d510("internal error, invalid method"); /* do not return */
        v7 = 0;
        v6 = 0;
LABEL_40ce42:
        switch ((unsigned int)sub_409e80(v14, v15, v16, v17))
        {
        case 3:
            sub_40d550(); /* do not return */
        case 0:
            if (!g_4e0e9c)
                break;
LABEL_40d045:
            if (g_4e0e98)
            {
                node = &v0;
                while (1)
                {
                    v22 = g_41b008;
                    if ((unsigned int)v22 < *((int *)&g_41b00c))
                    {
                        node += 1;
                        g_41b008 = (unsigned int)v22 + 1;
                        *((char *)node - 1) = (&g_49d000)[v22];
                        if (node == &v4)
                            break;
                    }
                    else
                    {
                        node += 1;
                        *((char *)node - 1) = sub_40d830(0);
                        if (node == &v4)
                            break;
                    }
                }
                v6 = v1;
                v7 = v3;
                goto LABEL_40cec3;
            }
            break;
        default:
            sub_40d510("invalid compressed data--format violated"); /* do not return */
        }
    }
LABEL_40cec3:
    v23 = 0;
    if (sub_40d1f0(&g_45c000, 0) != v6)
    {
        v23 = 1;
        sub_40f630(stderr, "\n%s: %s: invalid compressed data--crc error\n", g_41b850, &g_41b420);
    }
    if (g_41b820 != v7)
    {
        sub_40f630(stderr, "\n%s: %s: invalid compressed data--length error\n", g_41b850, &g_41b420);
        if (!g_4e0e9c)
            goto LABEL_40d100;
        v23 = 1;
LABEL_40cf4a:
        if (g_41b008 + 4 < *((int *)&g_41b00c) && ((*(&(&g_49d003)[g_41b008]) * 0x100 | (&g_49d002)[g_41b008]) * 0x10000 | (&g_49d000)[g_41b008] | *(&(&g_49d001)[g_41b008]) * 0x100) == 67324752)
        {
            if (!g_41b860)
            {
                sub_40f630(stderr, "%s: %s has more than one entry -- unchanged\n", g_41b850, &g_41b420);
                g_4e0e9c = 0;
LABEL_40d100:
                g_4e0e98 = 0;
                g_4e0e90 = v6;
LABEL_40cfcb:
                g_41b84c = 1;
                if (!g_41b858)
                    sub_408620(v14, v15, v16, v17); /* do not return */
                return 1;
            }
            else
            {
                if (!g_41b85c)
                    sub_40f630(stderr, "%s: %s has more than one entry--rest ignored\n", g_41b850, &g_41b420);
                if (!g_41b84c)
                    g_41b84c = 2;
            }
        }
    }
    else if (!(!g_4e0e9c))
    {
        goto LABEL_40cf4a;
    }
    g_4e0e9c = 0;
    g_4e0e98 = 0;
    g_4e0e90 = v6;
    if (!v23)
        return v23;
    goto LABEL_40cfcb;
}



// Function: write_buffer @ 0xd1d0
long long write_buffer(int a0, void* a1, unsigned int a2)
{
    if (0x7fffffff < a2)
        a2 = 0x7fffffff;
    return write(a0, a1, a2);
}



// Function: updcrc @ 0xd1f0
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern char g_0;
extern struct_0 g_413800;
extern unsigned long long g_4193f0;

unsigned long long updcrc(char *a0, unsigned int a1)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rdx
    char *i;  // rdi
    char *v4;  // rdi

    v1 = 0;
    v2 = 0xffffffff;
    if (a0)
    {
        v2 = g_4193f0;
        if (a1)
        {
            i = a0;
            do
            {
                v4 = i + 1;
                v2 = v2 >> 8 ^ *((long long *)&(&g_413800.field_0)[4 * (*(i) ^ (char)v2)]);
                i = v4;
            } while (i != &a0[a1]);
        }
        v1 = 0xffffffff ^ v2;
    }
    g_4193f0 = v2;
    return v1;
}



// Function: getcrc @ 0xd250
extern unsigned long long g_4193f0;

long long getcrc(void)
{
    return 0xffffffff ^ g_4193f0;
}



// Function: clear_bufs @ 0xd270
extern char g_0;
extern unsigned int g_41b004;
extern unsigned int g_41b008;
extern unsigned int g_41b00c;
extern unsigned long long g_41b820;
extern unsigned long long g_41b828;

void clear_bufs(void)
{
    g_41b004 = 0;
    g_41b008 = 0;
    g_41b00c = 0;
    g_41b820 = 0;
    g_41b828 = 0;
    return;
}



// Function: read_buffer @ 0xd2b0
extern char g_0;

int read_buffer(unsigned int a0, void* a1, unsigned int a2)
{
    unsigned long long v3;  // r12
    unsigned long long v4;  // rbx
    unsigned int v5;  // eax
    unsigned int *err;  // rax
    unsigned long long v7;  // rax
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]

    if (0x7fffffff < a2)
        a2 = 0x7fffffff;
    v1 = v3;
    v0 = v4;
    v5 = read(a0, a1, a2);
    if (v5 >= 0)
        return v5;
    err = __errno_location();
    if (*(err) != 11)
        return v5;
    v7 = sub_40e270(a0, 3);
    if ((unsigned int)v7 < 0)
    {
        return v5;
    }
    else if (!(*((char *)((void*)&v7 + 1)) & 8))
    {
        *(err) = 11;
        return v5;
    }
    else if (sub_40e270(a0, 4, _INSERT(v7, 1, *((char *)((void*)&v7 + 1)) & 247) & 0xffffffff) == 0xffffffff)
    {
        return v5;
    }
    else
    {
        return read(a0, a1, a2);
    }
}



// Function: strlwr @ 0xd350
extern char g_0;

char * strlwr(char *a0)
{
    char v1;  // bl
    unsigned long long *v2;  // rax
    char *v3;  // rbp
    char *v4;  // rbp
    unsigned long idx;  // r12
    unsigned long long *v6;  // rax

    v1 = *(a0);
    if (!v1)
        return a0;
    v2 = __ctype_b_loc();
    v3 = a0;
    do
    {
        v4 = v3;
        idx = v1;
        if (*((char *)(*(v2) + idx * 2 + 1)) & 1)
        {
            v6 = __ctype_tolower_loc();
            v1 = *((char *)(*(v6) + idx * 4));
        }
    } while ((*(v4) = v1, v1 = v4[1], v3 = v4 + 1, v4[1]));
    return a0;
}



// Function: add_envopt @ 0xd3d0
typedef struct struct_0 {
    void* field_0;
} struct_0;

extern char g_0;

void* add_envopt(unsigned int *a0, struct_0 **a1, char *a2)
{
    void* v1;  // r13
    unsigned int v2;  // r12d
    void* *iter;  // r12
    void* *i;  // r14
    void* node;  // rbx
    void* v3;  // rbx
    void* v4;  // rbx
    void* v5;  // rbx
    void* v6;  // rax
    unsigned long v7;  // 4115
    unsigned long v8;  // rbx
    void* *v9;  // rax
    void* *v10;  // rdx

    v1 = getenv(a2);
    if (!v1)
        return v1;
    v2 = 0;
    v3 = sub_40f5d0(v1);
    v4 = v3;
    if (*((char *)v4))
    {
        while (1)
        {
            v5 = v4 + strspn(v4, " \t");
            if (!*((char *)v5))
                break;
            v2 += 1;
            v6 = strcspn(v5, " \t") + v5;
            if (!*((char *)v6) || !(v7 = (unsigned long)(unsigned long long)(char)(char)v6[1], *((char *)v6) = 0, v4 = v6 + 1, (char)v7))
                goto LABEL_40d452;
        }
        if (v2)
        {
LABEL_40d452:
            v8 = v2 + 1;
            *(a0) = v8;
            v9 = sub_40f4d0(v2 + 2, 8);
            v10 = &*(a1)->field_0;
            iter = v9 + 1;
            *(a1) = v9;
            i = &v9[v8];
            *(v9) = *(v10);
            node = v3;
            do
            {
                iter += 1;
                node += strspn(node, " \t");
                *((void* *)((char *)iter - 8)) = node;
                do
                {
                    node += 1;
                } while (*((char *)node - 1));
            } while (i != iter);
            *(i) = 0;
            return v3;
        }
    }
    free(v3);
    return 0;
}



// Function: gzip_error @ 0xd510
extern long long stderr;
extern long long g_41b420;
extern long long g_41b850;

void gzip_error(long long a0)
{
    unsigned long long v2;  // rax
    unsigned int v3;  // rdi
    unsigned int v4;  // esi
    unsigned int v5;  // edx
    char v6;  // cl
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f630(stderr, "\n%s: %s: %s\n", g_41b850, &g_41b420, a0);
    sub_408620(v3, v4, v5, v6); /* do not return */
}



// Function: xalloc_die @ 0xd550
extern long long stderr;
extern long long g_41b850;

void xalloc_die(unsigned long a0, unsigned long a1, unsigned long a2, char a3)
{
    unsigned long long v2;  // rax
    unsigned int v3;  // rdi
    unsigned int v4;  // esi
    unsigned int v5;  // edx
    char v6;  // cl
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f630(stderr, "\n%s: memory_exhausted\n", g_41b850);
    sub_408620(v3, v4, v5, v6); /* do not return */
}



// Function: warning @ 0xd580
extern char g_0;
extern long long stderr;
extern long long g_41b420;
extern unsigned int g_41b84c;
extern long long g_41b850;
extern unsigned int g_41b85c;

unsigned long long warning(long long a0)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    if (g_41b85c)
    {
        v1 = g_41b84c;
        if ((unsigned int)v1)
            return v1;
        g_41b84c = 2;
        return v1;
    }
    else
    {
        v2 = sub_40f630(stderr, "%s: %s: warning: %s\n", g_41b850, &g_41b420, a0);
        if (g_41b84c)
            return v2;
        g_41b84c = 2;
        return v2;
    }
}



// Function: read_error @ 0xd600
extern char g_0;
extern long long stderr;
extern char g_41b420;
extern long long g_41b850;

void read_error(void)
{
    unsigned long long v2;  // rbx
    unsigned int *err;  // rax
    unsigned int v4;  // rdi
    unsigned int v5;  // esi
    unsigned int v6;  // edx
    char v7;  // cl
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    err = __errno_location();
    sub_40f630(stderr, "\n%s: ", g_41b850);
    if (*(err))
    {
        *(err) = *(err);
        perror(&g_41b420);
    }
    else
    {
        sub_40f630(stderr, "%s: unexpected end of file\n", &g_41b420);
    }
    sub_408620(v4, v5, v6, v7); /* do not return */
}



// Function: write_error @ 0xd670
extern long long stderr;
extern char g_41b020;
extern long long g_41b850;

void write_error(void)
{
    unsigned long long v2;  // rbx
    unsigned int *err;  // rax
    unsigned int v4;  // rdi
    unsigned int v5;  // esi
    unsigned int v6;  // edx
    char v7;  // cl
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    err = __errno_location();
    sub_40f630(stderr, "\n%s: ", g_41b850);
    *(err) = *(err);
    perror(&g_41b020);
    sub_408620(v4, v5, v6, v7); /* do not return */
}



// Function: write_buf @ 0xd6c0
extern char g_0;
extern unsigned long long g_41b820;
extern unsigned int g_41b858;

long long write_buf(int a0, void* a1, unsigned int a2)
{
    unsigned long v3;  // rax
    unsigned long long v4;  // r12
    unsigned long long v5;  // rbx
    unsigned long v6;  // rax
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v3 = a2;
    g_41b820 = g_41b820 + v3;
    if (g_41b858)
        return v3;
    v1 = v4;
    v0 = v5;
    while (1)
    {
        v6 = sub_40d1d0(a0, a1, v3);
        if ((unsigned int)v3 == (unsigned int)v6)
        {
            return v6;
        }
        else if ((unsigned int)v6 != 0xffffffff)
        {
            v3 -= v6;
            a1 += v6 & 0xffffffff;
        }
        else
        {
            sub_40d670(); /* do not return */
        }
    }
}



// Function: copy @ 0xd720
extern char g_0;
extern unsigned int g_41b008;
extern unsigned int g_41b00c;
extern unsigned long long g_41b828;
extern void g_49d000;

unsigned int copy(unsigned int a0, int a1)
{
    unsigned int v1;  // edx
    unsigned long v2;  // rsi

    *(__errno_location()) = 0;
    v1 = g_41b00c;
    v2 = g_41b008;
    if (g_41b008 >= g_41b00c)
        return 0;
    while (1)
    {
        sub_40d6c0(a1, &(&g_49d000)[v2], v1 - v2);
        v1 = sub_40d2b0(a0, &g_49d000, 0x40000);
        if (v1 == 0xffffffff)
            sub_40d600(); /* do not return */
        g_41b00c = v1;
        v2 = 0;
        g_41b828 = g_41b828 + v1;
        g_41b008 = 0;
        if (!v1)
            return 0;
    }
}



// Function: flush_outbuf @ 0xd7a0
extern char g_0;
extern unsigned int g_41b004;
extern int g_41b010;
extern void g_45c000;

long long flush_outbuf(void)
{
    long long v1;  // rax
    long long v2;  // rax

    if (g_41b004)
    {
        v2 = sub_40d6c0(g_41b010, &g_45c000, g_41b004);
        g_41b004 = 0;
        return v2;
    }
    return v1;
}



// Function: flush_window @ 0xd7e0
extern char g_0;
extern unsigned int g_41b004;
extern int g_41b010;
extern char g_43c000;

long long flush_window(void)
{
    long long v1;  // rax
    void* v2;  // r9
    unsigned int v3;  // r8d
    long long v4;  // rax

    if (g_41b004)
    {
        sub_40d1f0(&g_43c000, g_41b004);
        v4 = sub_40d6c0(g_41b010, v2, v3);
        g_41b004 = 0;
        return v4;
    }
    return v1;
}



// Function: fill_inbuf @ 0xd830
extern char g_0;
extern unsigned int g_41b008;
extern unsigned int g_41b00c;
extern unsigned int g_41b014;
extern unsigned long long g_41b828;
extern char g_49d000;

unsigned int fill_inbuf(unsigned int a0)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // eax

    v1 = 0;
    g_41b00c = 0;
    while (1)
    {
        v2 = sub_40d2b0(g_41b014, &(&g_49d000)[v1], 0x40000 - v1);
        if (v2)
        {
            if (v2 == 0xffffffff)
                break;
            v1 = v2 + g_41b00c;
            g_41b00c = v1;
            if ((unsigned int)v1 > 0x3ffff)
            {
                g_41b008 = 1;
                g_41b828 = g_41b828 + v1;
                return g_49d000;
            }
        }
        else
        {
            v1 = g_41b00c;
            if (g_41b00c)
            {
                g_41b008 = 1;
                g_41b828 = g_41b828 + v1;
                return g_49d000;
            }
            if (a0)
                return 0xffffffff;
            sub_40d7e0();
            *(__errno_location()) = 0;
            break;
        }
    }
    sub_40d600(); /* do not return */
}



// Function: display_ratio @ 0xd8e0

int display_ratio(unsigned long a0, unsigned long long a1, long long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    return sub_40f630(a2, "%5.1f%%");
}



// Function: fprint_off @ 0xd930
typedef struct FILE {
} FILE;

extern char g_0;

void fprint_off(FILE *a0, uint128_t a1, unsigned int a2)
{
    uint128_t i;  // rcx
    void* v4;  // r12
    void* v13;  // rbx
    void* iter;  // rbx
    uint128_t v6;  // rdx
    void* v7;  // rdi
    void* v8;  // rdi
    uint128_t v9;  // rdx
    unsigned int v10;  // esi
    unsigned int v11;  // r13d
    unsigned int j;  // r13d
    char v1;  // [bp-0x3a]

    i = a1;
    v4 = &v1 - 2;
    iter = &v1 - 2;
    if (i >= 0)
    {
        do
        {
            iter -= 1;
            v6 = i / 10;
            *((char *)iter) = (char)(i - v6 * 10) + 48;
            i = v6;
        } while (i);
    }
    else
    {
        do
        {
            v8 = v4 - 1;
            v9 = (unsigned long long)((int128_t)(i * 7378697629483820647) >> 66) - ((int128_t)(i) >> 63);
            *((char *)v8) = 48 + (unsigned int)v9 * 10 - i;
            i = v9;
            v4 = v8;
        } while (i);
        *((char *)v8 - 1) = 45;
        iter = v7 - 2;
    }
    v10 = a2 - (&v1 - 2 - iter);
    v11 = v10 - 1;
    if (v10 > 0)
    {
        do
        {
            j = v11;
            putc(32, a0);
            v11 = j - 1;
        } while (j >= 1);
    }
    if (iter >= &v1 - 2)
        return;
    do
    {
        v13 = iter + 1;
        putc(*((char *)iter), a0);
        iter = v13;
    } while (iter != &v1 - 2);
    return;
}



// Function: zip @ 0xda60
extern char g_0;
extern int g_41901c;
extern unsigned int g_419020;
extern long long stderr;
extern unsigned int g_41b004;
extern unsigned int g_41b010;
extern unsigned int g_41b014;
extern long long g_41b420;
extern unsigned long long g_41b828;
extern unsigned long long g_41b830;
extern unsigned long long g_41b838;
extern unsigned long long g_41b840;
extern char g_41b848;
extern long long g_41b850;
extern void g_45c000;
extern char g_45c002;
extern char g_45c003;
extern char g_45c004;
extern char g_45c005;
extern unsigned long long g_4e0ea8;

unsigned int zip(unsigned int a0, unsigned int a1)
{
    unsigned long long v2;  // 4132
    char v3;  // dil
    unsigned long v12;  // rax
    unsigned long v13;  // rbx
    void* iter;  // rax
    char v15;  // dl
    char v16;  // al
    unsigned long v17;  // rbx
    unsigned long idx;  // r12
    unsigned long long v19;  // rbx
    char v20;  // al
    char v21;  // al
    char v4;  // dl
    unsigned long v22;  // rax
    unsigned long index;  // r12
    char v24;  // al
    unsigned long v25;  // rbx
    unsigned long v26;  // rax
    unsigned long long v27;  // rax
    unsigned long idx1;  // rdx
    unsigned long v29;  // rbx
    char v30;  // al
    unsigned long idx2;  // rdx
    char v5;  // sil
    char v6;  // cl
    unsigned long v7;  // rcx
    unsigned int v8;  // eax
    unsigned long long v9;  // rdx
    unsigned long long v10;  // rax
    char v11;  // cl
    unsigned short v0;  // [bp-0x22]

    g_41b010 = a1;
    v0 = 0;
    g_41b014 = a0;
    *((unsigned short *)&g_45c000) = 35615;
    v2 = g_41b838;
    g_419020 = 8;
    g_45c002 = 8;
    g_41b004 = 4;
    g_45c003 = (*((int *)&g_41b848)) * 8;
    if (v2 - 0 >> 63 & 1)
    {
        v6 = 0;
        v5 = 0;
        v4 = 0;
        v3 = 0;
LABEL_40db09:
        g_45c004 = v3;
        v10 = 7;
        g_45c005 = v4;
        v9 = 6;
        goto LABEL_40db20;
    }
    else if (g_41b830 - 1 <= 0xfffffffe)
    {
        v3 = g_41b830;
        v4 = *((char *)((void*)&g_41b830 + 1));
        v5 = g_41b830 >> 16;
        v6 = v5;
        goto LABEL_40db09;
    }
    else
    {
        sub_40d580("file timestamp out of range for gzip format");
        v7 = g_41b004;
        if ((unsigned int)v7 > 0x3fffd)
        {
            v8 = (unsigned int)v7 + 1;
            (&g_45c000)[v7] = 0;
            g_41b004 = v8;
            if (v8 == 0x40000)
            {
                sub_40d7a0();
                v8 = g_41b004;
            }
            v9 = v8 + 1;
            g_41b004 = v9;
            (&g_45c000)[v8] = 0;
            if ((unsigned int)v9 != 0x40000)
            {
                v10 = v8 + 2;
            }
            else
            {
                sub_40d7a0();
                v9 = g_41b004;
                v10 = g_41b004 + 1;
                if (g_41b004 > 0x3fffd)
                    goto LABEL_40ddb2;
LABEL_40dfd2:
                v6 = 0;
                v5 = 0;
LABEL_40db20:
                (&g_45c000)[v9 & 0xffffffff] = v5;
                g_41b004 = (unsigned int)v9 + 2;
                (&g_45c000)[v10] = v6;
                goto LABEL_40db34;
            }
        }
        else
        {
            v9 = (unsigned int)v7 + 2;
            v10 = (unsigned int)v7 + 3;
            (&g_45c000)[v7 & 0xffffffff] = 0;
            (&g_45c000)[1 + v7] = 0;
        }
        if ((unsigned int)v9 <= 0x3fffd)
            goto LABEL_40dfd2;
LABEL_40ddb2:
        g_41b004 = v10;
        (&g_45c000)[v9] = 0;
        if ((unsigned int)v10 == 0x40000)
        {
            sub_40d7a0();
            v10 = g_41b004;
        }
        (&g_45c000)[v10] = 0;
        g_41b004 = (unsigned int)v10 + 1;
        if (g_41b004 == 0x40000)
            sub_40d7a0();
    }
LABEL_40db34:
    sub_40d1f0(NULL, 0);
    sub_404260(a1);
    sub_40ad80(&v0);
    v11 = 4;
    if (g_41901c != 1)
        v11 = (g_41901c == 9) * 2;
    v12 = g_41b004 + 1;
    (&g_45c000)[g_41b004] = v11;
    g_41b004 = v12;
    if ((unsigned int)v12 == 0x40000)
    {
        sub_40d7a0();
        v12 = g_41b004;
    }
    v13 = (unsigned int)v12 + 1;
    (&g_45c000)[v12] = 3;
    g_41b004 = v13;
    if ((unsigned int)v13 == 0x40000)
    {
        sub_40d7a0();
        v13 = g_41b004;
    }
    if (*((int *)&g_41b848))
    {
        iter = (unsigned long long)sub_40d3b0(&g_41b420);
        while (1)
        {
            v15 = *((char *)iter);
            g_41b004 = (unsigned int)v13 + 1;
            (&g_45c000)[v13] = v15;
            v13 = g_41b004;
            if (g_41b004 != 0x40000)
            {
                iter += 1;
                if (!*((char *)iter - 1))
                    break;
            }
            else
            {
                sub_40d7a0();
                iter += 1;
                v13 = g_41b004;
                if (!*((char *)iter - 1))
                    break;
            }
        }
    }
    g_4e0ea8 = v13 & 0xffffffff;
    sub_404ed0(g_41901c);
    if (g_41b840 != 0xffffffffffffffff && g_41b840 != g_41b828)
        sub_40f630(stderr, "%s: %s: file size changed while zipping\n", g_41b850, &g_41b420);
    v16 = sub_40d250();
    v17 = g_41b004;
    idx = (unsigned int)v17 + 1;
    g_41b004 = idx;
    if ((unsigned int)v17 <= 0x3fffd)
    {
        v19 = (unsigned int)v17 + 2;
        (&g_45c000)[v17 & 0xffffffff] = v16;
        v20 = sub_40d250();
        g_41b004 = v19;
        (&g_45c000)[idx] = v20;
    }
    else
    {
        (&g_45c000)[v17] = v16;
        if ((unsigned int)idx == 0x40000)
        {
            sub_40d7a0();
            idx = g_41b004;
        }
        v21 = sub_40d250();
        v19 = (unsigned int)idx + 1;
        g_41b004 = v19;
        (&g_45c000)[idx] = v21;
        if ((unsigned int)v19 == 0x40000)
        {
            sub_40d7a0();
            v19 = g_41b004;
        }
    }
    v22 = sub_40d250();
    index = (unsigned int)v19 + 1;
    g_41b004 = index;
    v24 = v22 >> 16;
    if ((unsigned int)v19 <= 0x3fffd)
    {
        v25 = (unsigned int)v19 + 2;
        (&g_45c000)[v19 & 0xffffffff] = v24;
        (&g_45c000)[index] = *((char *)((void*)&sub_40d250() >> 16 + 1));
    }
    else
    {
        (&g_45c000)[v19] = v24;
        if ((unsigned int)index == 0x40000)
        {
            sub_40d7a0();
            index = g_41b004;
        }
        v26 = sub_40d250();
        v25 = (unsigned int)index + 1;
        g_41b004 = v25;
        (&g_45c000)[index] = *((char *)((void*)&v26 >> 16 + 1));
        if ((unsigned int)v25 == 0x40000)
        {
            sub_40d7a0();
            v25 = g_41b004;
        }
    }
    v27 = g_41b828;
    idx1 = (unsigned int)v25 + 1;
    if ((unsigned int)v25 <= 0x3fffd)
    {
        v29 = (unsigned int)v25 + 2;
        (&g_45c000)[v25 & 0xffffffff] = g_41b828;
        (&g_45c000)[idx1] = *((char *)((void*)&g_41b828 + 1));
    }
    else
    {
        g_41b004 = idx1;
        (&g_45c000)[v25] = g_41b828;
        if ((unsigned int)idx1 == 0x40000)
        {
            sub_40d7a0();
            v27 = g_41b828;
            idx1 = g_41b004;
        }
        v29 = (unsigned int)idx1 + 1;
        (&g_45c000)[idx1] = *((char *)((void*)&v27 + 1));
        g_41b004 = v29;
        if ((unsigned int)v29 == 0x40000)
        {
            sub_40d7a0();
            v29 = g_41b004;
            v27 = g_41b828;
        }
    }
    v30 = v27 >> 16;
    idx2 = (unsigned int)v29 + 1;
    if ((unsigned int)v29 <= 0x3fffd)
    {
        (&g_45c000)[v29 & 0xffffffff] = v30;
        g_41b004 = (unsigned int)v29 + 2;
        (&g_45c000)[idx2] = v30;
    }
    else
    {
        g_41b004 = idx2;
        (&g_45c000)[v29] = v30;
        if ((unsigned int)idx2 == 0x40000)
        {
            sub_40d7a0();
            idx2 = g_41b004;
            v30 = g_41b828 >> 16;
        }
        (&g_45c000)[idx2] = v30;
        g_41b004 = (unsigned int)idx2 + 1;
        if (g_41b004 == 0x40000)
            sub_40d7a0();
    }
    g_4e0ea8 = g_4e0ea8 + 8;
    sub_40d7a0();
    return 0;
}



// Function: file_read @ 0xe010
void file_read(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40e01f();
    return;
}



// Function: last_component @ 0xe060
extern char g_0;

char * last_component(char *a0)
{
    char v1;  // dl
    char *v2;  // rax
    char *v3;  // rax
    char *v4;  // rax
    char v5;  // sil
    char *v6;  // rax
    char *idx;  // rcx

    v1 = *(a0);
    v2 = a0;
    v3 = v2;
    if (*(a0) == 47)
    {
        do
        {
            v4 = v2;
            v1 = v4[1];
            v2 = v4 + 1;
        } while (v4[1] == 47);
    }
    if (!v1)
        return v3;
    v5 = 0;
    v6 = v3;
    while (1)
    {
        idx = v3;
        if (v1 != 47)
        {
            if (v5)
            {
                v5 = 0;
                v6 = idx;
            }
            v1 = idx[1];
            v3 = idx + 1;
            if (!idx[1])
                return v6;
        }
        else
        {
            v1 = idx[1];
            v3 = idx + 1;
            v5 = 1;
            if (!idx[1])
                return v6;
        }
    }
}



// Function: rpl_fcntl_DUPFD @ 0xe180

int rpl_fcntl_DUPFD(int a0, unsigned long a1)
{
    return fcntl(a0, 0);
}



// Function: rpl_fcntl_DUPFD_CLOEXEC @ 0xe190
extern char g_0;
extern int g_4e0eb0;

unsigned int rpl_fcntl_DUPFD_CLOEXEC(int a0, unsigned int a1)
{
    unsigned int v1;  // r12d
    unsigned int v2;  // r12d
    unsigned int *err;  // rax

    if (g_4e0eb0 >= 0)
    {
        v1 = fcntl(a0, 1030);
        if (v1 >= 0 || *(__errno_location()) != 22)
        {
            g_4e0eb0 = 1;
            return v1;
        }
        v2 = sub_40e180(a0, a1 & 0xffffffff);
        if (v2 < 0)
            return v2;
        g_4e0eb0 = 0xffffffff;
    }
    else
    {
        v2 = sub_40e180(a0, a1);
        if (v2 < 0)
        {
            return v2;
        }
        else if (g_4e0eb0 != -0x1)
        {
            return v2;
        }
    }
    if (fcntl(v2, 1) >= 0 && fcntl(v2, 2) != -0x1)
        return v2;
    err = __errno_location();
    close(v2);
    *(err) = *(err);
    return 0xffffffff;
}



// Function: rpl_fcntl @ 0xe270
extern char g_0;

int rpl_fcntl(int a0, unsigned int a1)
{
    unsigned long v8;  // rdx
    unsigned long long v9;  // rcx
    unsigned long long v10;  // r8
    unsigned long long v11;  // r9
    unsigned int v12;  // eax
    unsigned int v13;  // ecx
    unsigned int v14;  // eax
    unsigned int v0;  // [bp-0x58]
    char *v1;  // [bp-0x50]
    char *v2;  // [bp-0x48]
    int v3;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x20]
    unsigned long long v5;  // [bp-0x18]
    unsigned long long v6;  // [bp-0x10]
    unsigned long v7;  // [bp+0x8]

    *((unsigned long *)&(&v3)[16]) = v8;
    v4 = v9;
    v5 = v10;
    v6 = v11;
    v0 = 16;
    v1 = &v7;
    v2 = &v3;
    if (!a1)
    {
        return sub_40e180(a0, (int)(&v3)[16]);
    }
    else if (a1 != 1030)
    {
        if (a1 > 11)
        {
            v13 = a1 - 0x400;
            if (v13 <= 10)
            {
                v14 = 1 << ((char)v13 & 63);
                if (v14 & 709)
                    goto LABEL_40e380;
                if ((unsigned short)v14 & 1282)
                    return fcntl(a0, a1);
            }
        }
        else if (a1 > 0)
        {
            v12 = 1 << ((char)a1 & 63);
            if ((unsigned short)v12 & 1300)
            {
LABEL_40e380:
                return fcntl(a0, a1);
            }
            else if ((unsigned short)v12 & 0xa0a)
            {
                return fcntl(a0, a1);
            }
        }
        return fcntl(a0, a1);
    }
    else
    {
        return sub_40e190(a0, (int)(&v3)[16]);
    }
}



// Function: open_safer @ 0xe3c0
extern char g_0;

int open_safer(char *a0, int a1, unsigned int a2)
{
    unsigned int v5;  // edx
    unsigned int v0;  // [bp-0x58]
    char *v1;  // [bp-0x50]
    char *v2;  // [bp-0x48]
    char v3;  // [bp-0x38]
    unsigned long v4;  // [bp+0x8]

    v5 = 0;
    if ((char)a1 & 64)
    {
        v0 = 16;
        v1 = &v4;
        v2 = &v3;
        v5 = a2;
    }
    return sub_40e980(open(a0, a1, v5) & 0xffffffff);
}



// Function: clear_ungetc_buffer_preserving_position @ 0xe440
typedef struct FILE {
} FILE;

extern char g_0;

int clear_ungetc_buffer_preserving_position(FILE *a0)
{
    int v3;  // eax
    unsigned long long v4;  // rax
    unsigned long long v0;  // [bp-0x18]
    int v1;  // [bp-0xc]

    if (!((short)*((int *)&a0) & 0x100))
        return v3;
    if (*((long long *)&a0[16]) != *((long long *)&a0[8]) || *((long long *)&a0[40]) != *((long long *)&a0[32]) || *((long long *)&a0[72]))
        return fseeko(a0, 0, 1);
    v1 = 1;
    v0 = 0;
    v4 = lseek(fileno(a0), v0, v1);
    if (v4 == 0xffffffffffffffff)
        return -0x1;
    *((unsigned int *)&a0) = *((int *)&a0) & 0xffffffef;
    *((unsigned long long *)&a0[144]) = v4;
    return 0;
    return fseeko(a0, 0, 1);
}



// Function: openat_safer @ 0xe520
extern char g_0;

int openat_safer(unsigned long a0, unsigned long a1, char a2, unsigned long long a3)
{
    unsigned int v0;  // [bp-0x58]
    char *v1;  // [bp-0x50]
    char *v2;  // [bp-0x48]
    char v3;  // [bp-0x38]
    unsigned long long v4;  // [bp-0x20]
    unsigned long v5;  // [bp+0x8]

    v4 = a3;
    if (a2 & 64)
    {
        v0 = 24;
        v1 = &v5;
        v2 = &v3;
    }
    return sub_40e980(openat() & 0xffffffff);
}



// Function: direntry_cmp_name @ 0xe5a0
int direntry_cmp_name(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: streamsavedir @ 0xe5b0
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

extern char g_0;
extern unsigned long long g_418b50[4];

char * streamsavedir(DIR *a0, unsigned int a1)
{
    unsigned long v6;  // rbx
    unsigned long i;  // r15
    unsigned long v16;  // r15
    unsigned long v17;  // r10
    unsigned long v18;  // rax
    unsigned long v19;  // r10
    unsigned long v20;  // rbp
    char *v21;  // rbx
    unsigned long long v22;  // rbx
    unsigned long v23;  // rbp
    unsigned long v24;  // rbp
    unsigned long ptr;  // r13
    char *v9;  // r14
    unsigned int *err;  // r12
    dirent *count;  // rax
    char v12;  // dl
    dirent *v13;  // r9
    dirent *ptr1;  // r9
    unsigned long n;  // r10
    unsigned long v0;  // [bp-0x70]
    dirent *v1;  // [bp-0x68], Other Possible Types: unsigned long long *, unsigned long
    dirent *v2;  // [bp-0x60], Other Possible Types: unsigned long
    unsigned long v3;  // [bp-0x50]
    unsigned long v4;  // [bp-0x48]

    v3 = 0;
    v4 = 0;
    v0 = g_418b50[a1];
    if (!a0)
        return 0;
    v6 = 0;
    i = 0;
    ptr = 0;
    v9 = 0;
    err = __errno_location();
    while (1)
    {
        *(err) = 0;
        count = readdir(a0);
        if (count)
        {
            v12 = *((char *)&count[1].d_ino + 6);
            v13 = (char *)&count[1].d_ino + 6;
            if (*((char *)&count[1].d_ino + 6) != 46 || !(v12 = *((char *)((char *)&count[1].d_ino + 7)), v12 == 46))
            {
                if (!v12)
                    continue;
            }
            else
            {
                if (!(char)count[1].d_reclen)
                    continue;
            }
            v1 = v13;
            ptr1 = v1;
            n = strlen(v13) + 1;
            if (v0)
            {
                if (v4 == i)
                {
                    v2 = ptr1;
                    v1 = n;
                    ptr1 = v2;
                    n = v1;
                    ptr = sub_40f3f0(ptr, &v4, 1, -0x1, 8);
                }
                v2 = n;
                v16 = i + 1;
                v1 = ptr + i * 8;
                v17 = v2;
                *(v1) = sub_40f5d0(ptr1);
                i = v16;
            }
            else
            {
                v18 = v3 - v6;
                if (v18 <= n)
                {
                    v2 = ptr1;
                    v1 = n;
                    ptr1 = v2;
                    v19 = v1;
                    v9 = sub_40f3f0(v9, &v3, n - v18, 0x7ffffffffffffffe, 1);
                    n = v19;
                }
                v1 = n;
                memcpy(&v9[v6], ptr1, n);
                v17 = v1;
            }
            v6 += v17;
        }
        else if (*(err))
        {
            free(ptr);
            free(v9);
            return 0;
        }
        else if (v0)
        {
            v20 = v6 + 1;
            if (!i)
            {
                v9 = sub_40f250(v20);
                v21 = v9;
            }
            else
            {
                v22 = 0;
                qsort(ptr, i, 8, v0);
                v23 = 0;
                v9 = sub_40f250(v20);
                do
                {
                    v24 = v23 + 1;
                    v22 = v22 + stpcpy(&v9[v22], *((long long *)(ptr + v23 * 8))) - &v9[v22] + 1;
                    free(*((long long *)(ptr + v23 * 8)));
                    v23 = v24;
                } while (i != v23);
                v21 = &v9[v22];
            }
            free(ptr);
            break;
        }
        else
        {
            if (v6 == v3)
                v9 = sub_40f2c0(v9, v6 + 1);
            v21 = &v9[v6];
            break;
        }
    }
    *(v21) = 0;
    return v9;
}



// Function: savedir @ 0xe870
void savedir(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40e87a();
    return;
}



// Function: fd_safer @ 0xe980
int fd_safer(int a0)
{
    int v1;  // eax
    unsigned int *err;  // rax

    if (a0 <= 2)
    {
        v1 = sub_410b10();
        err = __errno_location();
        close(a0);
        *(err) = *(err);
        return v1;
    }
    return a0;
}



// Function: validate_timespec @ 0xe9e0
extern char g_0;

unsigned int validate_timespec(unsigned long long *idx)
{
    unsigned long long v1;  // r8
    unsigned long v2;  // rcx
    unsigned long long v3;  // rax
    unsigned long v4;  // rsi
    unsigned long long v5;  // rdx
    unsigned int v6;  // r9d

    v1 = idx[1];
    v2 = v1 - 0x3ffffffe;
    if (v2 <= 1 || v1 <= 0x3b9ac9ff)
    {
        v3 = idx[3];
        v4 = v3 - 0x3ffffffe;
        if (v4 <= 1 || v3 <= 0x3b9ac9ff)
        {
            v5 = 0;
            v6 = 0;
            if (v2 <= 1)
            {
                *(idx) = 0;
                v6 = 1;
                v5 = v1 == 0x3ffffffe;
            }
            if (v4 <= 1)
            {
                idx[2] = 0;
                v6 = 1;
                v5 += v3 == 0x3ffffffe;
            }
            return ((unsigned int)v5 == 1) + v6;
        }
    }
    *(__errno_location()) = 22;
    return 0xffffffff;
}



// Function: update_timespec @ 0xea90
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

extern char g_0;

unsigned int update_timespec(long long a0, struct_0 **a1, unsigned long a2)
{
    unsigned long long *idx;  // rbx
    unsigned long long v3;  // r12
    unsigned long long v4;  // rax
    unsigned long long v5;  // rdx
    unsigned long long v6;  // rdx
    char v0;  // [bp-0x19]

    idx = &*(a1)->field_0;
    v3 = idx[3];
    if (idx[1] == 0x3ffffffe)
    {
        if (v3 == 0x3ffffffe)
            return 1;
        *(idx) = sub_40e920();
        idx[1] = v6;
    }
    else if (idx[1] != 0x3fffffff)
    {
        if (v3 == 0x3ffffffe)
        {
            v4 = sub_40e940(a0);
            idx[3] = v5;
            idx[2] = v4;
            return 0;
        }
    }
    else
    {
        if (v3 != 0x3fffffff)
        {
            sub_410a40(idx);
            v3 = idx[3];
            if (v3 == 0x3ffffffe)
            {
                v4 = sub_40e940(a0);
                idx[3] = v5;
                idx[2] = v4;
                return 0;
            }
        }
        else
        {
            *(a1) = 0;
            return 0;
        }
    }
    if (v3 == 0x3fffffff)
    {
        v0 = 0;
        sub_410a40(idx + 2);
        return v0;
    }
    return 0;
}



// Function: fdutimens @ 0xeb50
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

extern char g_0;
extern unsigned int g_4e0eb4;
extern int g_4e0eb8;

unsigned int fdutimens(unsigned int a0, unsigned long a1, void* idx)
{
    int v11;  // eax
    void* v20;  // r14
    unsigned int v22;  // eax
    char v23;  // r13b
    char v25;  // r13b
    char v13;  // al
    unsigned int v15;  // rdx
    unsigned int v16;  // r12d
    char v18;  // al
    void* v0;  // [bp-0x140]
    unsigned long long v1[4];  // [bp-0x138]
    int v2;  // [bp-0x118], Other Possible Types: unsigned long long (64 bits)[4]
    int v3;  // [bp-0xf8]
    unsigned long long v4;  // [bp-0xf0]
    int v5;  // [bp-0xe8]
    unsigned long long v6;  // [bp-0xe0]
    int v7;  // [bp-0xd8], Other Possible Types: stat
    unsigned long v8;  // [bp-0x90]
    unsigned long v9;  // [bp-0x80]

    if (!idx)
    {
        v0 = 0;
        v11 = 0;
        goto LABEL_40ebbc;
    }
    v0 = v1;
    v1 = (unsigned long long (64 bits)[4])_INSERT(v1 CONCAT 0, 0, *((int128_t *)idx));
    *((int128_t *)&v1[2]) = (int128_t)idx[16];
    v11 = sub_40e9e0(v1);
    if (v11 < 0)
        return 0xffffffff;
    idx = v1;
LABEL_40ebbc:
    if (!(a0 >= 0 || a1))
    {
        *(__errno_location()) = 9;
    }
    else if (g_4e0eb8 < 0)
    {
LABEL_40ec05:
        g_4e0eb8 = 0xffffffff;
        g_4e0eb4 = 0xffffffff;
        if (!v11)
        {
LABEL_40ec64:
            v20 = v0;
            if (v20)
            {
                *((long long *)&v2) = *((long long *)v20);
                v2[2] = (long long)v20[16];
                *((long long *)&(&v2)[8]) = ((long long)v20[8] * 2361183241434822607 >> 71) - ((long long)v20[8] >> 63);
                v2[3] = ((long long)v20[24] * 2361183241434822607 >> 71) - ((long long)v20[24] >> 63);
                v20 = v2;
            }
            if (a0 < 0)
                return futimesat(0xffffff9c, a1);
            v22 = futimesat(a0, 0);
            if (v22)
            {
                if (a1)
                    return utimes(a1, v20);
                return 0xffffffff;
            }
            else
            {
                if (!v20)
                    return v22;
                v23 = 499999 < (long long)v20[24];
                if (499999 >= (long long)v20[8] && !v23)
                    return v22;
                if (fstat(a0, &v7))
                    return v22;
                v3 = (int)*((int128_t *)v20);
                v5 = (int)(int128_t)v20[16];
                v25 = v23 & v9 - (long long)v20[16] == 1;
                if (v8 - *((long long *)v20) == 1 && 499999 < (long long)v20[8] && !sub_40e8e0(&v7))
                {
                    v4 = 0;
                    if (!v25)
                    {
                        futimesat(a0, 0, &v3);
                        return 0;
                    }
                    if (sub_40e900(&v7))
                    {
                        futimesat(a0, 0, &v3);
                        return 0;
                    }
                }
                else
                {
                    if (!v25)
                        return 0;
                    if (sub_40e900(&v7))
                        return 0;
                }
                v6 = 0;
                futimesat(a0, 0, &v3);
                return 0;
            }
        }
        if (v11 != 3)
        {
            if (a0 >= 0)
                v18 = fstat(a0, &v7);
            else
                v18 = stat(a1, &v7);
            if (!v18)
                goto LABEL_40ec45;
        }
        else
        {
LABEL_40ec45:
            if (idx && (char)sub_40ea90(&v7, &v0, v15))
                return 0;
            goto LABEL_40ec64;
        }
    }
    else if (v11 != 2)
    {
LABEL_40ebdd:
        if (a0 >= 0)
        {
            v16 = futimens(a0, idx);
            if (v16 <= 0)
            {
                if (!v16)
                {
                    g_4e0eb8 = 1;
                    return v16;
                }
                goto LABEL_40ef28;
            }
        }
        else
        {
            v16 = utimensat(0xffffff9c, a1, idx, 0);
            if (v16 > 0)
            {
                *(__errno_location()) = 38;
                goto LABEL_40ec05;
            }
            else
            {
                if (!v16)
                {
                    g_4e0eb8 = 1;
                    return v16;
                }
LABEL_40ef28:
                if (*(__errno_location()) != 38)
                {
                    g_4e0eb8 = 1;
                    return v16;
                }
            }
        }
    }
    else
    {
        if (a0 >= 0)
            v13 = fstat(a0, &v7);
        else
            v13 = stat(a1, &v7);
        if (!v13)
        {
            if ((long long)idx[8] != 0x3ffffffe)
            {
                v11 = 3;
                if ((long long)idx[24] == 0x3ffffffe)
                {
                    *((unsigned long long *)&idx[16]) = sub_40e940(&v7);
                    *((unsigned long long *)&idx[24]) = v15;
                    goto LABEL_40ebdd;
                }
            }
            else
            {
                v11 = 3;
                *((unsigned long long *)idx) = sub_40e920(&v7);
                *((unsigned long long *)&idx[8]) = v15;
                goto LABEL_40ebdd;
            }
        }
    }
    return 0xffffffff;
}



// Function: lutimens @ 0xef90
typedef struct struct_0 {
    unsigned long long field_0[2];
    uint128_t field_10;
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

extern char g_0;
extern int g_4e0eb4;
extern unsigned int g_4e0eb8;

unsigned int lutimens(unsigned long a0, struct_0 *a1)
{
    struct_0 *v7;  // r12
    unsigned int v8;  // r13d
    unsigned long long v10;  // rdx
    unsigned int v11;  // r12d
    unsigned long long *v0;  // [bp-0xf0]
    unsigned long long v1[2];  // [bp-0xe8]
    unsigned long long v2;  // [bp-0xe0]
    int v3;  // [bp-0xd8], Other Possible Types: unsigned long long
    unsigned long long v4;  // [bp-0xd0]
    stat v5;  // [bp-0xc8]

    if (!a1)
    {
        v0 = 0;
        if (g_4e0eb4 < 0)
        {
            g_4e0eb4 = 0xffffffff;
            goto LABEL_40f134;
        }
LABEL_40f050:
        v11 = utimensat(0xffffff9c, a0, a1, 0x100);
        if (v11 > 0)
        {
            *(__errno_location()) = 38;
            goto LABEL_40f083;
        }
        else if (!v11 || *(__errno_location()) != 38)
        {
            g_4e0eb8 = 1;
            g_4e0eb4 = 1;
            return v11;
        }
    }
    v7 = v1;
    v0 = &v1[0];
    v1 = a1->field_0;
    v3 = (int)a1->field_10;
    v8 = sub_40e9e0(v1);
    if (v8 < 0)
        return 0xffffffff;
    if (g_4e0eb4 >= 0)
    {
        if (v8 != 2)
        {
            a1 = v1;
            goto LABEL_40f050;
        }
        else if (lstat(a0, &v5))
        {
            return 0xffffffff;
        }
        else if (*(&v1[1]) != 0x3ffffffe)
        {
            a1 = v7;
            if (*((unsigned long long *)(&v3 + 8)) == 0x3ffffffe)
            {
                v3 = sub_40e940(&v5);
                v4 = v10;
                a1 = v7;
                goto LABEL_40f050;
            }
        }
        else
        {
            a1 = v1;
            v1 = (unsigned long long (64 bits)[2])sub_40e920(&v5);
            v2 = v10;
            goto LABEL_40f050;
        }
    }
    a1 = v1;
LABEL_40f083:
    g_4e0eb4 = 0xffffffff;
    switch (v8)
    {
    case 0:
LABEL_40f134:
        if (!lstat(a0, &v5))
            goto LABEL_40f0bb;
        break;
    case 3:
        if (a1)
            goto LABEL_40f0a5;
        goto LABEL_40f0bb;
    default:
        if (!lstat(a0, &v5))
        {
            if (a1)
            {
LABEL_40f0a5:
                if ((char)sub_40ea90(&v5, &v0, v10))
                    return 0;
            }
LABEL_40f0bb:
            if (((unsigned short)v5.st_nlink & 0xf000) != 0xa000)
                return sub_40eb50(0xffffffff, a0, v0);
            *(__errno_location()) = 38;
            break;
        }
    }
    return 0xffffffff;
}



// Function: xmalloc @ 0xf230
long long xmalloc(unsigned long size)
{
    malloc(size);
}



// Function: ximalloc @ 0xf250
void ximalloc(void)
{
    sub_410aa0();
}



// Function: xrealloc @ 0xf280
extern char g_0;

void* xrealloc(void* a0, unsigned long a1)
{
    void* ptr;  // rax
    unsigned int v2;  // rdi
    unsigned int v3;  // rsi
    unsigned int v4;  // rdx
    char v5;  // cl

    ptr = realloc(a0, a1);
    if (ptr)
        return ptr;
    if (a0 && !a1)
        return ptr;
    sub_40d550(v2, v3, v4, v5); /* do not return */
}



// Function: xirealloc @ 0xf2c0
void xirealloc(void)
{
    sub_410ab0();
}



// Function: xreallocarray @ 0xf2e0
extern char g_0;

void* xreallocarray(void* a0, unsigned long a1, unsigned long a2)
{
    void* ptr;  // rax
    unsigned int v2;  // rdi
    unsigned int v3;  // rsi
    unsigned int v4;  // rdx
    char v5;  // cl

    ptr = reallocarray(a0, a1, a2);
    if (ptr)
        return ptr;
    if (a0)
    {
        if (!a1)
        {
            return ptr;
        }
        else if (!a2)
        {
            return ptr;
        }
    }
    sub_40d550(v2, v3, v4, v5); /* do not return */
}



// Function: xireallocarray @ 0xf330
void xireallocarray(void)
{
    sub_410ae0();
}



// Function: xpalloc @ 0xf3f0
extern char g_0;

void* xpalloc(void* a0, void* a1, long long a2, unsigned long a3, unsigned long a4)
{
    unsigned long v1;  // r9
    unsigned long v2;  // rcx
    unsigned long v11;  // rax
    unsigned long v12;  // rcx
    unsigned long v13;  // rcx
    void* v14;  // rax
    long long v3;  // rbx
    unsigned long v4;  // rbx
    unsigned long v5;  // rax
    unsigned long v6;  // rsi
    unsigned long v7;  // rsi
    long long v8;  // rdx
    long long v9;  // rdx
    unsigned long v10;  // rsi

    v1 = a3;
    v2 = *((long long *)a1);
    v3 = (long long)(v2) >> 1;
    v4 = v3 + v2;
    if (__OFADD__(v3, v2) & 1)
        v4 = 0x7fffffffffffffff;
    if (v4 <= v1)
        v5 = v4;
    else
        v5 = v1;
    if (v1 >= 0)
        v4 = v5;
    v6 = v4;
    v7 = a4 * v6;
    if (__OFSMUL__(a4, v6) & 1)
    {
        v10 = 0x7fffffffffffffff;
        goto LABEL_40f485;
    }
    else if (v7 > 127)
    {
        v8 = a2;
        v9 = a2;
        if (!a0)
            goto LABEL_40f498;
    }
    else
    {
        v10 = 128;
LABEL_40f485:
        v11 = v10;
        v8 = v11 % a4;
        v4 = v11 / a4;
        v7 = v10 - v8;
        v9 = v8;
        if (!a0)
        {
LABEL_40f498:
            v8 = v9;
            *((unsigned long long *)a1) = 0;
        }
    }
    if (v4 - v2 < a2)
    {
        v12 = v2 + a2;
        v4 = v12;
        v13 = v12;
        if (__OFADD__(v2, a2) & 1 || v12 > v1 && !(v13 = v12, v1 < 0) || !(v13 = a4 * v12, v7 = v13, !((unsigned long long)__OFSMUL__(a4, v12) & 1)))
            sub_40d550(a0, v7, v8, v13); /* do not return */
    }
    v14 = sub_40f280(a0, v7);
    *((unsigned long *)a1) = v4;
    return v14;
}



// Function: xcalloc @ 0xf4d0
long long xcalloc(unsigned long count, unsigned long size)
{
    calloc(count, size);
}



// Function: xicalloc @ 0xf500
void xicalloc(void)
{
    sub_410ad0();
}



// Function: ximemdup @ 0xf560
void ximemdup(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f570();
    return;
}



// Function: ximemdup0 @ 0xf590
void ximemdup0(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f5a1();
    return;
}



// Function: xstrdup @ 0xf5d0
long long xstrdup(char *a0)
{
    strlen(a0);
}



// Function: yesno @ 0xf5f0
extern char g_0;

int yesno(void)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    unsigned long v3;  // r12

    v1 = getchar();
    v2 = v1;
    if (v2 != 10)
    {
        do
        {
        } while (v2 != 0xffffffff && (v2 = (unsigned int)(int)(long long)getchar(), v2 != 10));
    }
    return _INSERT(v3, 0, (v1 & 0xffffffdf) == 89);
}



// Function: rpl_fprintf @ 0xf630
extern char g_0;

int rpl_fprintf(unsigned long a0, long long a1)
{
    unsigned long long v14;  // rdx
    unsigned long long v15;  // rcx
    unsigned long long v16;  // r8
    unsigned long long v17;  // r9
    unsigned long v19;  // fs
    char *ptr;  // rbp
    unsigned int v21;  // eax
    unsigned int v0;  // [bp-0x8dc]
    unsigned long v1;  // [bp-0x8d8]
    unsigned int v2;  // [bp-0x8d0]
    unsigned int v3;  // [bp-0x8cc]
    char *v4;  // [bp-0x8c8]
    char *v5;  // [bp-0x8c0]
    char v6;  // [bp-0x8b8]
    unsigned long v7;  // [bp-0xe0]
    char v8;  // [bp-0xd8]
    unsigned long long v9;  // [bp-0xc8]
    unsigned long long v10;  // [bp-0xc0]
    unsigned long long v11;  // [bp-0xb8]
    unsigned long long v12;  // [bp-0xb0]
    unsigned long v13;  // [bp+0x8]

    v9 = v14;
    v10 = v15;
    v11 = v16;
    v12 = v17;
    v7 = *((long long *)(40 + v19));
    v1 = 2000;
    v4 = &v13;
    v2 = 16;
    v3 = 48;
    v5 = &v8;
    ptr = sub_40f880(&v6, &v1, a1, &v2);
    if (!ptr)
    {
LABEL_40f797:
        sub_410a30(a0);
        v21 = 0xffffffff;
    }
    else if (fwrite(ptr, 1, v1, a0) >= v1)
    {
        if (ptr != &v6)
            free(ptr);
        if (v1 <= 0x7fffffff)
        {
            v21 = v1;
        }
        else
        {
            *(__errno_location()) = 75;
            goto LABEL_40f797;
        }
    }
    else
    {
        v21 = 0xffffffff;
        if (ptr != &v6)
        {
            v0 = 0xffffffff;
            free(ptr);
            v21 = v0;
        }
    }
    if (v7 != *((long long *)(40 + v19)))
        __stack_chk_fail(); /* do not return */
    return v21;
}



// Function: __printf__ @ 0xf7b0
extern long long stdout;

unsigned long long __printf__(long long a0)
{
    unsigned long long v12;  // rsi
    unsigned long long v13;  // rdx
    unsigned long long v14;  // rcx
    unsigned long long v15;  // r8
    unsigned long long v16;  // r9
    unsigned long v18;  // fs
    unsigned long long v19;  // rax
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

    v6 = v12;
    v7 = v13;
    v8 = v14;
    v9 = v15;
    v10 = v16;
    v4 = *((long long *)(40 + v18));
    v0 = 8;
    v2 = &v11;
    v1 = 48;
    v3 = &v5;
    v19 = sub_4108a0(stdout, a0, &v0);
    if (v4 != *((long long *)(40 + v18)))
        __stack_chk_fail(); /* do not return */
    return v19;
}



// Function: vasnprintf @ 0xf880
typedef struct struct_1 {
    char padding_0[16];
    unsigned int field_10;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[12];
    void* field_10;
} struct_0;

typedef struct struct_3 {
    char padding_0[2];
    char field_2;
} struct_3;

extern char g_0;
extern unsigned int g_414090[4];

void* vasnprintf(void* a0, unsigned long long *a1, unsigned long a2, long long a3)
{
    unsigned long v25;  // rdi
    unsigned long v26;  // rdi
    unsigned long long v35;  // r13
    unsigned long long v36;  // rax
    char *v37;  // rcx
    struct_0 *v38;  // rax
    char v39;  // al
    struct_3 *iter;  // r12
    unsigned long v41;  // rsi
    unsigned long v42;  // rsi
    char v43;  // al
    struct_1 *v44;  // rax
    char *i;  // rdx
    struct_1 *v45;  // rcx
    unsigned long index;  // rax
    unsigned long long v47;  // rax
    void* ptr;  // rax
    void* ptr1;  // rax
    unsigned int v50;  // ftop
    unsigned int v51;  // ftop
    unsigned long long v52;  // rsi
    unsigned long long v53;  // r12
    unsigned long long v54;  // rsi
    unsigned long long v28;  // rax
    unsigned long v55;  // r9
    unsigned int v56;  // eax
    unsigned long v57;  // r9
    unsigned int v58;  // eax
    unsigned int v59;  // ftop
    unsigned int v60;  // ftop
    unsigned int v61;  // eax
    unsigned int v62;  // eax
    unsigned long long v63;  // rsi
    unsigned int v64;  // eax
    unsigned long long v29;  // rbx
    unsigned int v65;  // eax
    unsigned int v66;  // edx
    unsigned long long v67;  // rcx
    unsigned long long v68;  // r12
    unsigned long long v69;  // rax
    char v70;  // r12b
    void* ptr2;  // rax
    void* v72;  // rax
    void* v73;  // rax
    void* v74;  // rax
    void* node;  // r14
    void* v75;  // rax
    unsigned long n;  // r9
    unsigned long v77;  // r8
    void* v78;  // rax
    void* v79;  // rax
    unsigned int v80;  // eax
    unsigned long long v81;  // rax
    unsigned long long v82;  // rdx
    char v83;  // r12b
    void* v84;  // rax
    unsigned long long v31;  // r12
    void* v85;  // rax
    unsigned long long v86;  // rsi
    void* v87;  // rax
    unsigned long v32;  // r8
    void* dst;  // r15
    unsigned long v34;  // rax
    char v0;  // [bp-0x2428]
    char v1;  // [bp-0x1428]
    char v2;  // [bp-0x4a8]
    char v3;  // [bp-0x428]
    unsigned long long v4;  // [bp-0x420]
    unsigned int v5;  // [bp-0x414]
    struct_3 *v6;  // [bp-0x410]
    struct_3 *v7;  // [bp-0x400]
    unsigned long long v8;  // [bp-0x3f8]
    struct_3 *v9;  // [bp-0x3f0]
    char *v10;  // [bp-0x3e8], Other Possible Types: unsigned int, unsigned long
    int *err;  // [bp-0x3e0], Other Possible Types: char, unsigned long
    char v12;  // [bp-0x3d8], Other Possible Types: unsigned int, unsigned long
    unsigned int v13;  // [bp-0x3c4]
    unsigned int v14;  // [bp-0x3c0]
    unsigned int v15;  // [bp-0x3bc]
    char v16;  // [bp-0x3b8]
    char *v17;  // [bp-0x3b0]
    char v18;  // [bp-0x3a8]
    unsigned long long v19;  // [bp-0x2c8]
    void* v20;  // [bp-0x2c0]
    long long v21;  // [bp-0x2b8]
    long long v22;  // [bp-0x2b0]
    char v23;  // [bp-0x2a8]

    if (sub_410dd0(a2, &v19, &v16) < 0)
    {
        return 0;
    }
    else if (sub_410ba0(a3, &v16) >= 0)
    {
        v25 = sub_410b70(7, v21, v22, 6);
        if (v25 <= 3999)
        {
            v26 = v25 + 23;
            i = &(&v3)[-1 * (v26 & 0xfffffffffffff000)];
            if (&v3 != i)
            {
                do
                { } while (&v0 != i);
            }
            v28 = (unsigned int)v26 & 0xfffffff0 & 0xfff;
            /* unsupported instruction */ = (int)&(&v1)[-1 * v28];
            if (v28)
                *((long long *)(/* unsupported instruction */ + v28 - 8)) = *((long long *)(/* unsupported instruction */ + v28 - 8));
            v6 = 0;
            v9 = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
            goto LABEL_40f941;
        }
        else if (v25 != 0xffffffffffffffff && !(/* unsupported instruction */ = (int)(&v2 - 120), v9 = (struct_3 *)malloc(v25), !v9))
        {
            v6 = v9;
LABEL_40f941:
            v29 = 0;
            if (a0)
                v29 = *(a1);
            node = v20;
            v8 = 0;
            v31 = 0;
            v32 = a2;
            dst = a0;
            v34 = *((long long *)node);
            if (v34 == v32)
            {
LABEL_410352:
                v35 = v31;
                goto LABEL_40fa3a;
            }
LABEL_40f988:
            err = v32;
            v12 = v34 - v32;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            n = v12;
            v77 = err;
            v35 = sub_410b30(v31, v12);
            if (v29 < v35)
            {
                if (v29)
                {
                    if (v29 < 0)
                        goto LABEL_40fb98;
                    v29 *= 2;
                }
                else
                {
                    v29 = 12;
                }
                if (v29 < v35)
                    v29 = v35;
                switch (v29)
                {
                case 18446744073709551615:
                    goto LABEL_40fb98;
                    break;
                }
                if (dst && !err)
                {
                    err = v77;
                    v12 = n;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v78 = realloc(dst, v29);
                    if (!v78)
                        goto LABEL_41044e;
                    n = v12;
                    v77 = err;
                    dst = v78;
                }
                else
                {
                    v10 = v77;
                    err = dst == a0;
                    v12 = n;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v79 = malloc(v29);
                    n = v12;
                    v77 = v10;
                    if (!v79)
                        goto LABEL_40fb98;
                    if (v31 && err)
                    {
                        err = v77;
                        v12 = n;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        n = v12;
                        v77 = err;
                        dst = memcpy(v79, dst, v31);
                    }
                    else
                    {
                        dst = v79;
                    }
                }
            }
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            memcpy(dst + v31, v77, n);
LABEL_40fa3a:
            if (v19 != v8)
            {
                v36 = (long long)node[80];
                if ((char)node[72] == 37)
                {
                    if (v36 != 0xffffffffffffffff)
                        goto LABEL_410885;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v31 = sub_410b30(v35, 1);
                    if (v29 < v31)
                    {
                        if (!v29)
                        {
                            v29 = 12;
                        }
                        else
                        {
                            if (v29 < 0)
                                goto LABEL_40fb98;
                            v29 *= 2;
                        }
                        if (v29 < v31)
                            v29 = v31;
                        if (v29 == 0xffffffffffffffff)
                            goto LABEL_40fb98;
                        if (dst && !v12)
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            v74 = realloc(dst, v29);
                            if (!v74)
                                goto LABEL_41044e;
                            dst = v74;
                        }
                        else
                        {
                            v12 = dst == a0;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            v75 = malloc(v29);
                            if (!v75)
                                goto LABEL_40fb98;
                            if (v35 && v12)
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                dst = memcpy(v75, dst, v35);
                            }
                            else
                            {
                                dst = v75;
                            }
                        }
                    }
                    *((char *)dst + v35) = 37;
                    goto LABEL_410335;
                }
                if (v36 == 0xffffffffffffffff)
                    goto LABEL_410885;
                v37 = v17;
                v38 = &v37[32 * v36];
                v12 = v38->field_0;
                if ((char)node[72] != 110)
                {
                    v39 = (int)node[16];
                    iter = &v9->padding_0[1];
                    v9->padding_0[0] = 37;
                    if (v39 & 1)
                    {
                        v9->padding_0[1] = 39;
                        iter = &v9->field_2;
                    }
                    if (v39 & 2)
                    {
                        iter->padding_0[0] = 45;
                        iter = &iter->padding_0[1];
                    }
                    if (v39 & 4)
                    {
                        iter->padding_0[0] = 43;
                        iter = &iter->padding_0[1];
                    }
                    if (v39 & 8)
                    {
                        iter->padding_0[0] = 32;
                        iter = &iter->padding_0[1];
                    }
                    if (v39 & 16)
                    {
                        iter->padding_0[0] = 35;
                        iter = &iter->padding_0[1];
                    }
                    if (v39 & 64)
                    {
                        iter->padding_0[0] = 73;
                        iter = &iter->padding_0[1];
                    }
                    if (v39 & 32)
                    {
                        iter->padding_0[0] = 48;
                        iter = &iter->padding_0[1];
                    }
                    v41 = (long long)node[24];
                    if (v41 != (long long)node[32])
                    {
                        v10 = v37;
                        err = (long long)node[32] - v41;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        memcpy(iter, v41, err);
                        v37 = v10;
                        iter = &iter->padding_0[err];
                    }
                    v42 = (long long)node[48];
                    if (v42 != (long long)node[56])
                    {
                        v10 = v37;
                        err = (long long)node[56] - v42;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        memcpy(iter, v42, err);
                        v37 = v10;
                        iter = &iter->padding_0[err];
                    }
                    switch (v12)
                    {
                    case 12:
                        iter->padding_0[0] = 76;
                        iter = &iter->padding_0[1];
                        break;
                    case 9: case 10:
                        iter->padding_0[0] = 108;
                        iter = &iter->padding_0[1];
                    case 7: case 8: case 14: case 16:
                        iter->padding_0[0] = 108;
                        iter = &iter->padding_0[1];
                        break;
                    }
                    v43 = (char)node[72];
                    iter->padding_0[1] = 0;
                    iter->padding_0[0] = v43;
                    if ((long long)node[40] != 0xffffffffffffffff)
                    {
                        v44 = &v37[32 * node[40]];
                        if (*((int *)&v44->padding_0[0]) != 5)
                            goto LABEL_410885;
                        v10 = 1;
                        v14 = v44->field_10;
                    }
                    else
                    {
                        v10 = 0;
                    }
                    if ((long long)node[64] != 0xffffffffffffffff)
                    {
                        v45 = &v37[32 * node[64]];
                        if (*((int *)&v45->padding_0[0]) != 5)
                            goto LABEL_410885;
                        index = v10;
                        (&v14)[index] = v45->field_10;
                        v10 = (unsigned int)index + 1;
                    }
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v47 = sub_410b30(v35, 2);
                    if (v47 > v29)
                    {
                        if (!v29)
                        {
                            v29 = 12;
                        }
                        else
                        {
                            if (v29 < 0)
                                goto LABEL_40fb98;
                            v29 *= 2;
                        }
                        if (v29 <= v47)
                            v29 = v47;
                        if (v29 == 0xffffffffffffffff)
                            goto LABEL_40fb98;
                        if (dst && !err)
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            ptr = realloc(dst, v29);
                            if (!ptr)
                                goto LABEL_40fb98;
                            dst = ptr;
                        }
                        else
                        {
                            err = dst == a0;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            ptr1 = malloc(v29);
                            if (!ptr1)
                                goto LABEL_40fb98;
                            if (v35 && err)
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                dst = memcpy(ptr1, dst, v35);
                            }
                            else
                            {
                                dst = ptr1;
                            }
                        }
                    }
                    *((char *)dst + v35) = 0;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v7 = iter;
                    err = __errno_location();
                    v5 = *(err);
                    v51 = v50;
                }
                else
                {
                    switch (v12)
                    {
                    case 18:
                        *((char *)v73) = v35;
                        v31 = v35;
                        break;
                    case 19:
                        *((unsigned short *)v73) = v35;
                        v31 = v35;
                        break;
                    case 20:
                        *((unsigned int *)v73) = v35;
                        v31 = v35;
                        break;
                    case 21: case 22:
                        *((unsigned long long *)v73) = v35;
                        v31 = v35;
                        break;
                    default:
LABEL_410885:
                        abort(); /* do not return */
                    }
                    goto LABEL_410335;
                }
                while (1)
                {
                    v52 = 0x7fffffff;
                    v13 = 0xffffffff;
                    v53 = v29 - v35;
                    *(err) = 0;
                    if (v53 <= 0x7fffffff)
                    {
                        v54 = v53;
                        v52 = v54;
                    }
                    if (v12 > 0x11)
                        goto LABEL_410885;
                    goto *((void *)((long long)(long long)(g_414090[v12] + &g_414090[0])));

                    switch (/* incomplete */)
                    {
                    case 0x40ff30:
                        v57 = (unsigned long)(unsigned long long)(unsigned int)*((int *)&v17[16 + 32 * node[80]]);
                        v58 = v10;
                        if (v58 == 1)
                            goto LABEL_410140;
                        goto LABEL_40ff56;
                    case 0x410118:
                        v57 = (unsigned long)(unsigned long long)v17[16 + 32 * node[80]];
                        v58 = v10;
                        if (v58 == 1)
                            goto LABEL_410140;
                        goto LABEL_40ff56;
                    case 0x4101e0:
                        v57 = (unsigned long)(unsigned long long)(unsigned int)(int)*((short *)&v17[16 + 32 * node[80]]);
                        v58 = v10;
                        if (v58 == 1)
                            goto LABEL_410140;
                        goto LABEL_40ff56;
                    case 0x410160:
                        v57 = (unsigned long)(unsigned long long)(unsigned short)*((short *)&v17[16 + 32 * node[80]]);
                        v65 = v10;
                        if (v65 == 1)
                        {
LABEL_410140:
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned long *)(/* unsupported instruction */ - 8)) = v57;
                            goto LABEL_41014a;
                        }
                        if (v65 == 2)
                        {
LABEL_410190:
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned long *)(/* unsupported instruction */ - 8)) = v57;
                            goto LABEL_41019e;
                        }
LABEL_40ff5f:
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        goto LABEL_40ff6b;
                    case 0x410210:
                        v57 = (unsigned long)(unsigned long long)(unsigned int)(int)(char)v17[16 + 32 * node[80]];
                        v58 = v10;
                        if (v58 == 1)
                            goto LABEL_410140;
LABEL_40ff56:
                        if (v58 == 2)
                            goto LABEL_410190;
                        goto LABEL_40ff5f;
                    case 0x410098:
                        v64 = v10;
                        if (v64 == 1)
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
LABEL_4100d7:
                            v4 = v52;
                            v62 = (unsigned int)(unsigned long long)__snprintf_chk();
                            v63 = v4;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ + 8);
                            goto LABEL_40fe55;
                        }
                        else if (v64 == 2)
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned long long *)(/* unsupported instruction */ - 8)) = (unsigned long long)v15;
                            goto LABEL_4100d7;
                        }
                        else
                        {
                            v4 = v52;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            v62 = (unsigned int)(unsigned long long)__snprintf_chk();
                            v63 = v4;
                            goto LABEL_40fe55;
                        }
                    case 0x410050:
                        v59 = v51 - 1;
                        if (/* unsupported instruction */)
                        {
                            v60 = v59 - 1;
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
                        else
                        {
                            v60 = v59 - 1;
                            /* unsupported instruction */
                            /* unsupported instruction */
                        }
                        v61 = v10;
                        if (v61 == 1)
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 24);
                            if (/* unsupported instruction */)
                            {
                                amd64g_dirtyhelper_storeF80le(vvar_1205{r48|8b}, Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                                /* unsupported instruction */
                                v51 = v60 + 1;
                            }
                            else
                            {
                                amd64g_dirtyhelper_storeF80le(vvar_1205{r48|8b}, Reinterpret(F64->I64, nan<64>))
                                /* unsupported instruction */
                                v51 = v60 + 1;
                            }
LABEL_4101a5:
                            v4 = v52;
                            v62 = (unsigned int)(unsigned long long)__snprintf_chk();
                            v63 = v4;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ + 24);
                            goto LABEL_40fe55;
                        }
                        if (v61 != 2)
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 16);
                            if (/* unsupported instruction */)
                            {
                                amd64g_dirtyhelper_storeF80le(vvar_1226{r48|8b}, Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                                /* unsupported instruction */
                                v51 = v60 + 1;
                            }
                            else
                            {
                                amd64g_dirtyhelper_storeF80le(vvar_1226{r48|8b}, Reinterpret(F64->I64, nan<64>))
                                /* unsupported instruction */
                                v51 = v60 + 1;
                            }
                        }
                        else
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            v4 = v52;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 32);
                            if (/* unsupported instruction */)
                            {
                                amd64g_dirtyhelper_storeF80le((vvar_2076{r48|8b} Add 8<64>), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                                /* unsupported instruction */
                                v51 = v60 + 1;
                            }
                            else
                            {
                                amd64g_dirtyhelper_storeF80le((vvar_2076{r48|8b} Add 8<64>), Reinterpret(F64->I64, nan<64>))
                                /* unsupported instruction */
                                v51 = v60 + 1;
                            }
                            *((unsigned long long *)(/* unsupported instruction */ - 8)) = (unsigned long long)v15;
                            v62 = (unsigned int)(unsigned long long)__snprintf_chk();
                            v63 = v4;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ + 32);
LABEL_40fe55:
                            v66 = v13;
                            if (v66 < 0)
                                goto LABEL_40ffa3;
                            goto LABEL_40fe63;
                        }
LABEL_40fe2b:
                        v4 = v52;
                        v62 = (unsigned int)(unsigned long long)__snprintf_chk();
                        v63 = v4;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ + 8);
                        goto LABEL_40fe55;
                    case 0x40fdf0:
                        v55 = (unsigned long)*((long long *)&v17[16 + 32 * node[80]]);
                        v56 = v10;
                        if (v56 != 1)
                        {
                            if (v56 != 2)
                            {
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                                goto LABEL_40fe2b;
                            }
                            else
                            {
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                *((unsigned long *)(/* unsupported instruction */ - 8)) = v55;
LABEL_41019e:
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                *((unsigned long long *)(/* unsupported instruction */ - 8)) = (unsigned long long)v15;
                                goto LABEL_4101a5;
                            }
                        }
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned long *)(/* unsupported instruction */ - 8)) = v55;
LABEL_41014a:
LABEL_40ff6b:
                        v4 = v52;
                        v62 = (unsigned int)(unsigned long long)__snprintf_chk();
                        v66 = v13;
                        v63 = v4;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ + 8);
                        if (v66 >= 0)
                        {
LABEL_40fe63:
                            v67 = (unsigned long long)(long long)(int)v66;
                            if (v67 < v63 && *((char *)(v67 + (char *)dst + v35)))
                                goto LABEL_410885;
                            if (v66 >= v62)
                                goto LABEL_40fe85;
                            goto LABEL_40fe7d;
                        }
LABEL_40ffa3:
                        if (!v7->padding_0[1])
                        {
                            if (v62 < 0)
                            {
                                if (!*(err))
                                {
                                    if (((char)node[72] & 0xffffffef) == 99)
                                        v80 = 84;
                                    else
                                        v80 = 22;
                                    *(err) = (int)v80;
                                }
                                if (dst != a0 && dst)
                                    free(dst);
                                if (v6)
                                    free(v6);
                                if (v20 != &v23)
                                    free(v20);
                                if (v17 != &v18)
                                {
                                    free(v17);
                                    return 0;
                                }
                                return 0;
                            }
LABEL_40fe7d:
                            v13 = v62;
                            v66 = v62;
LABEL_40fe85:
                            if ((unsigned long long)(v66 + 1) < v63)
                            {
                                v31 = (long long)(int)v66 + v35;
                                *(err) = (int)v5;
LABEL_410335:
                                v32 = (unsigned long)(long long)node[8];
                                node += 88;
                                v8 += 1;
                                if ((long long)node[88] != v32)
                                    goto LABEL_40f988;
                                goto LABEL_410352;
                            }
                            if (v54 > 0x7ffffffe)
                            {
                                if (dst != a0 && dst)
                                    free(dst);
                                if (v6)
                                    free(v6);
                                if (v20 != &v23)
                                    free(v20);
                                if (v17 != &v18)
                                    free(v17);
                                *(err) = (int)75;
                                return 0;
                            }
                            v68 = v29 * 2;
                            if (v29 < 0)
                                v68 = 0xffffffffffffffff;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 16);
                            v69 = (unsigned long long)sub_410b90((unsigned long long)sub_410b30(), v68);
                            if (v29 >= v69)
                                continue;
                            if (!v29)
                                v68 = 12;
                            if (v69 <= v68)
                                v69 = v68;
                            v29 = v69;
                            if (v69 == 0xffffffffffffffff)
                                goto LABEL_40fba4;
                            v70 = (char)(char)(dst == a0);
                            if (dst && !v70)
                            {
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                ptr2 = realloc(dst, v69);
                                if (!ptr2)
                                    goto LABEL_40fba4;
                                dst = ptr2;
                            }
                            else
                            {
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                v72 = malloc(v29);
                                if (!v72)
                                    goto LABEL_40fba4;
                                if (v35 && v70)
                                {
                                    /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                    dst = memcpy(v72, dst, v35);
                                }
                                else
                                {
                                    dst = v72;
                                }
                            }
                        }
                        else
                        {
                            v7->padding_0[1] = 0;
                        }
                    }
                }
            }
            else
            {
                v81 = sub_410b30(v35, 1);
                if (v81 <= v29)
                {
LABEL_4107c5:
                    *((char *)dst + v35) = 0;
                    if (dst != a0)
                    {
                        v86 = v35 + 1;
                        if (v86 < v29)
                        {
                            v87 = realloc(dst, v86);
                            if (v87)
                                dst = v87;
                        }
                    }
                    if (v6)
                        free(v6);
                    if (v20 != &v23)
                        free(v20);
                    if (v17 != &v18)
                        free(v17);
                    *(a1) = v35;
                    return dst;
                }
                if (v29)
                {
                    v82 = v29 * 2;
                    if (v29 < 0)
                        goto LABEL_40fb98;
                }
                else
                {
                    v82 = 12;
                }
                if (v82 <= v81)
                    v82 = v81;
                if (v82 == 0xffffffffffffffff)
                {
LABEL_40fb98:
                    err = __errno_location();
LABEL_40fba4:
                    if (dst == a0 || !dst)
                        goto LABEL_40fbb6;
                    goto LABEL_410460;
                }
                else
                {
                    v83 = dst == a0;
                    if (!dst || v83)
                    {
                        v85 = malloc(v82);
                        if (!v85)
                            goto LABEL_40fb98;
                        if (v35 && v83)
                        {
                            dst = memcpy(v85, dst, v35);
                            v29 = v82;
                            goto LABEL_4107c5;
                        }
                        else
                        {
                            dst = v85;
                            v29 = v82;
                            goto LABEL_4107c5;
                        }
                    }
                    v84 = realloc(dst, v82);
                    if (v84)
                    {
                        dst = v84;
                        v29 = v82;
                        goto LABEL_4107c5;
                    }
LABEL_41044e:
                    err = __errno_location();
LABEL_410460:
                    free(dst);
                }
LABEL_40fbb6:
                if (v6)
                    free(v6);
            }
        }
        else
        {
            err = __errno_location();
        }
        if (v20 != &v23)
            free(v20);
        if (v17 != &v18)
            free(v17);
        *(err) = 12;
        return 0;
    }
    else
    {
        if (v20 != &v23)
            free(v20);
        if (v17 != &v18)
            free(v17);
        *(__errno_location()) = 22;
        return 0;
    }
}



// Function: rpl_vfprintf @ 0x108a0
extern char g_0;

unsigned int rpl_vfprintf(unsigned long a0, unsigned int a1, long long a2)
{
    char *v4;  // rax
    char *ptr;  // rbp
    unsigned int v0;  // [bp-0x81c]
    unsigned long v1;  // [bp-0x810]
    char v2;  // [bp-0x808]

    v1 = 2000;
    v4 = sub_40f880(&v2, &v1, a1, a2);
    if (v4)
    {
        ptr = v4;
        if (fwrite(v4, 1, v1, a0) >= v1)
        {
            if (ptr != &v2)
                free(ptr);
            if (v1 <= 0x7fffffff)
                return v1;
            *(__errno_location()) = 75;
        }
        else if (ptr == &v2)
        {
            return 0xffffffff;
        }
        else
        {
            v0 = 0xffffffff;
            free(ptr);
            return v0;
        }
    }
    sub_410a30(a0);
    return 0xffffffff;
}



// Function: opendir_safer @ 0x10990
typedef struct DIR {
} DIR;

extern char g_0;

DIR * opendir_safer(char *a0)
{
    DIR *ptr;  // rax
    unsigned int v2;  // eax
    unsigned int v3;  // ebp
    unsigned int *err;  // rbx
    unsigned int v5;  // r14d
    DIR *fp;  // r13

    ptr = opendir(a0);
    if (!ptr)
        return ptr;
    v2 = dirfd(ptr);
    if (v2 > 2)
        return ptr;
    v3 = sub_40e270(v2, 1030);
    err = __errno_location();
    if (v3 < 0)
    {
        v5 = *(err);
        fp = 0;
    }
    else
    {
        fp = fdopendir(v3);
        v5 = *(err);
        if (!fp)
            close(v3);
    }
    closedir(ptr);
    *(err) = v5;
    return fp;
}



// Function: fseterr @ 0x10a30
void fseterr(unsigned int *a0)
{
    *(a0) = *(a0) | 32;
    return;
}



// Function: gettime @ 0x10a40
extern char g_0;

void gettime(long long a0)
{
    clock_gettime(0, a0);
    return;
}



// Function: current_timespec @ 0x10a50
long long current_timespec(void)
{
    char v0;  // [bp-0x28]

    sub_410a40(&v0);
    return v0;
}



// Function: dup_safer @ 0x10b10

int dup_safer(int a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    return sub_40e270(a0, 0);
}



// Function: printf_fetchargs @ 0x10ba0
typedef struct struct_0 {
    unsigned long long field_0;
    void* field_8;
} struct_0;

typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    void* field_8;
    unsigned long long field_10;
} struct_1;

extern char g_0;

unsigned int printf_fetchargs(struct_1 *idx, struct_0 *a1)
{
    unsigned long long v1;  // r8
    void* iter;  // rax
    void* v11;  // r11
    unsigned int v12;  // edx
    void* v13;  // r11
    unsigned int v14;  // edx
    void* v15;  // r11
    unsigned long v16;  // rdx
    unsigned int v17;  // edx
    void* v18;  // r11
    unsigned long v19;  // rdx
    unsigned long long v3;  // rsi
    unsigned int v4;  // edx
    void* v5;  // r11
    unsigned int v6;  // edx
    void* v7;  // r11
    unsigned int v8;  // edx
    void* v9;  // r11
    unsigned int v10;  // edx

    v1 = a1->field_0;
    iter = a1->field_8;
    if (!v1)
        return 0;
    v3 = 0;
    do
    {
        switch (*((int *)iter))
        {
        case 1: case 2:
            v10 = idx->field_0;
            if (v10 <= 47)
            {
                v11 = v10 + idx->field_10;
                idx->field_0 = v10 + 8;
            }
            else
            {
                v11 = idx->field_8;
                idx->field_8 = v11 + 8;
            }
            *((char *)&iter[16]) = *((int *)v11);
            break;
        case 3: case 4:
            v8 = idx->field_0;
            if (v8 <= 47)
            {
                v9 = v8 + idx->field_10;
                idx->field_0 = v8 + 8;
            }
            else
            {
                v9 = idx->field_8;
                idx->field_8 = v9 + 8;
            }
            *((short *)&iter[16]) = *((int *)v9);
            break;
        case 5: case 6: case 13: case 14:
            v6 = idx->field_0;
            if (v6 <= 47)
            {
                v7 = v6 + idx->field_10;
                idx->field_0 = v6 + 8;
            }
            else
            {
                v7 = idx->field_8;
                idx->field_8 = v7 + 8;
            }
            *((int *)&iter[16]) = *((int *)v7);
            break;
        case 7: case 8: case 9: case 10: case 17: case 18: case 19: case 20: case 21: case 22:
            v4 = idx->field_0;
            if (v4 <= 47)
            {
                v5 = v4 + idx->field_10;
                idx->field_0 = v4 + 8;
            }
            else
            {
                v5 = idx->field_8;
                idx->field_8 = v5 + 8;
            }
            *((long long *)&iter[16]) = *((long long *)v5);
            break;
        case 11:
            v12 = idx->field_4;
            if (v12 <= 175)
            {
                v13 = v12 + idx->field_10;
                idx->field_4 = v12 + 16;
            }
            else
            {
                v13 = idx->field_8;
                idx->field_8 = v13 + 8;
            }
            *((long long *)&iter[16]) = *((long long *)v13);
            break;
        case 12:
            idx->field_8 = (idx->field_8 + 15 & 0xfffffffffffffff0) + 16;
            /* unsupported instruction */
            /* unsupported instruction */
            amd64g_dirtyhelper_storeF80le((vvar_75{r16|8b} Add 16<64>), Reinterpret(F64->I64, (((unsupported_<class 'pyvex.expr.GetI'>() CmpNE 0<8>)) ? (unsupported_<class 'pyvex.expr.GetI'>()) : (nan<64>))))
            /* unsupported instruction */
            break;
        case 15:
            v17 = idx->field_0;
            if (v17 <= 47)
            {
                v18 = v17 + idx->field_10;
                idx->field_0 = v17 + 8;
            }
            else
            {
                v18 = idx->field_8;
                idx->field_8 = v18 + 8;
            }
            v19 = *((long long *)v18);
            if (!*((long long *)v18))
                v19 = "(NULL)";
            *((unsigned long *)&iter[16]) = v19;
            break;
        case 16:
            v14 = idx->field_0;
            if (v14 <= 47)
            {
                v15 = v14 + idx->field_10;
                idx->field_0 = v14 + 8;
            }
            else
            {
                v15 = idx->field_8;
                idx->field_8 = v15 + 8;
            }
            v16 = *((long long *)v15);
            if (!*((long long *)v15))
                v16 = "(";
            *((unsigned long *)&iter[16]) = v16;
            break;
        default:
            return 0xffffffff;
        }
    } while ((v3 += 1, iter += 32, v3 != v1));
    return 0;
}



// Function: printf_parse @ 0x10dd0
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_1 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    struct struct_1 *field_20;
    char padding_28[8];
    unsigned int field_30;
    char padding_34[4];
    struct struct_1 *field_38;
    struct struct_1 *field_40;
    char padding_48[8];
    struct struct_1 *field_50;
    struct struct_1 *field_58;
    unsigned long long field_60;
    char padding_68[8];
    unsigned long long field_70;
} struct_0;

extern char g_0;

unsigned long long printf_parse(char *a0, struct_0 *idx, struct_2 *index)
{
    struct_0 *v9;  // r10
    char *v10;  // rax
    char *v19;  // rax
    unsigned int v20;  // edx
    unsigned long long v21;  // rax
    unsigned long long v22;  // rdi
    char *iter;  // rdx
    unsigned int v24;  // eax
    char *v25;  // rdx
    char *v26;  // rdx
    char *v27;  // rdx
    unsigned long long v28;  // rax
    struct_0 *v11;  // rcx
    unsigned long long v29;  // rdi
    unsigned long long v30;  // rax
    char v31;  // sil
    char *v32;  // rax
    char *v33;  // rax
    char *v34;  // rax
    unsigned long long v35;  // rdi
    unsigned long long v36;  // rbp
    struct_2 *ptr;  // rdi
    unsigned long long v38;  // rax
    unsigned long long v12;  // r9
    unsigned long long v39;  // rdi
    unsigned long long v40;  // r9
    struct_2 *v41;  // rax
    unsigned long long v42;  // rsi
    unsigned long long v43;  // rdx
    unsigned int *v44;  // rax
    unsigned int *v45;  // rax
    unsigned int *v47;  // rbp
    unsigned long long v48;  // rdi
    unsigned long long v13;  // r11
    unsigned long long v49;  // rax
    char *node;  // rcx
    char v51;  // dil
    char *v52;  // rax
    char *v53;  // rax
    char *v54;  // rax
    unsigned long long v55;  // rbx
    unsigned long long v56;  // rsi
    unsigned long long v57;  // rax
    unsigned long long v58;  // rdi
    char *iter1;  // rbx
    unsigned long long v59;  // r9
    unsigned long long v60;  // rsi
    unsigned long long v61;  // r11
    unsigned long long v62;  // r9
    unsigned long long v63;  // rdx
    unsigned int *v64;  // rax
    unsigned int *v65;  // rax
    unsigned int *v67;  // rax
    char *v68;  // rdx
    struct_0 *idx1;  // r12
    char *v69;  // rdx
    char *v70;  // rdx
    unsigned long long v71;  // rax
    unsigned long long v72;  // rcx
    char *iter2;  // rbx
    char v16;  // bpl
    char *v17;  // rax
    char *v18;  // rax
    struct_0 *v0;  // [bp-0x78], Other Possible Types: unsigned long long
    unsigned long long v1;  // [bp-0x70]
    struct_2 *ptr1;  // [bp-0x68]
    unsigned long long v3;  // [bp-0x60]
    char *v4;  // [bp-0x58], Other Possible Types: struct_0 *, unsigned long long
    struct_0 *v5;  // [bp-0x50], Other Possible Types: unsigned long long
    unsigned long long v6;  // [bp-0x48]
    unsigned long long v7;  // [bp-0x40]

    v9 = &idx->field_20;
    v10 = a0;
    v11 = v9;
    v12 = 7;
    v13 = 7;
    idx->field_0 = 0;
    idx->field_8 = v9;
    index->field_0 = 0;
    ptr1 = index + 1;
    index->field_8 = ptr1;
    v1 = 0;
    v0 = 0;
    v3 = 0;
    while (1)
    {
        if (!*(v10))
        {
            v11->field_0 = v10;
            idx->field_10 = v0;
            idx->field_18 = v1;
            return 0;
        }
        iter1 = v10 + 1;
        if (*(v10) == 37)
            break;
        v10 = iter1;
    }
    idx1 = v11;
    idx1->field_0 = v10;
    *((unsigned int *)&idx1->field_10) = 0;
    idx1->field_18 = 0;
    idx1->field_20 = 0;
    *((unsigned long long *)&idx1->padding_28[0]) = 0xffffffffffffffff;
    *((unsigned long long *)&idx1->field_30) = 0;
    idx1->field_38 = 0;
    idx1->field_40 = 0xffffffffffffffff;
    idx1->field_50 = 0xffffffffffffffff;
    v16 = v10[1];
    if (v16 - 48 > 9)
    {
LABEL_410efe:
        iter = iter1 + 1;
        if (v16 == 39)
            goto LABEL_410fe1;
        while (1)
        {
            switch (v16)
            {
            case 32:
                v24 = (int)idx1->field_10 | 8;
                goto LABEL_410fc8;
            case 35:
                v24 = (int)idx1->field_10 | 16;
                goto LABEL_410fc8;
            case 43:
                v24 = (int)idx1->field_10 | 4;
                goto LABEL_410fc8;
            case 45:
                v24 = (int)idx1->field_10 | 2;
                goto LABEL_410fc8;
            case 48:
                v24 = (int)idx1->field_10 | 32;
                goto LABEL_410fc8;
            case 73:
                v24 = (int)idx1->field_10 | 64;
                goto LABEL_410fc8;
            default:
                if (v16 != 42)
                {
                    if (v16 - 48 > 9)
                        goto LABEL_410f3d;
                    idx1->field_18 = iter1;
                    if (*(iter1) - 48 <= 9)
                    {
                        v25 = iter1;
                        do
                        {
                            v27 = v25 + 1;
                            v25 = v27;
                        } while (v26[1] - 48 <= 9);
                        v28 = v27 - iter1;
                        iter1 = v27;
                        if (v28 <= v0)
                            v28 = v0;
                        v0 = v28;
                    }
                    idx1->field_20 = iter1;
                    goto LABEL_41145c;
                }
                v29 = v0;
                v30 = 1;
                idx1->field_18 = iter1;
                idx1->field_20 = iter;
                v31 = iter1[1];
                if (v29)
                    v30 = v29;
                v0 = v30;
                if (iter1[1] - 48 > 9)
                {
LABEL_41116d:
                    v35 = v3;
                    *((unsigned long long *)&idx1->padding_28[0]) = v35;
                    if (v35 != 0xffffffffffffffff)
                    {
                        v36 = v3;
                        v3 = v35 + 1;
                        iter1 = iter;
                        goto LABEL_411192;
                    }
                    else
                    {
                        ptr = index->field_8;
                        break;
                    }
                }
                else
                {
                    v32 = iter;
                    do
                    {
                        v34 = v32 + 1;
                        v32 = v34;
                    } while (v33[1] - 48 <= 9);
                    if (v33[1] != 36)
                        goto LABEL_41116d;
                    v4 = v9;
                    v38 = 0;
                    v5 = v13;
                    while (1)
                    {
                        v39 = 0xffffffffffffffff;
                        if (v38 <= 1844674407370955161)
                            v39 = v38 * 10;
                        v38 = sub_410b30(v39, v31 - 48);
                        v31 = iter[1];
                        if (v31 - 48 > 9)
                            break;
                        iter += 1;
                    }
                    v9 = v4;
                    v13 = v5;
                    v36 = v38 - 1;
                    if (v36 > 0xfffffffffffffffd)
                    {
                        ptr = index->field_8;
                        break;
                    }
                    else
                    {
                        *((unsigned long long *)&idx1->padding_28[0]) = v36;
                        iter1 = iter + 2;
                    }
LABEL_411192:
                    if (7 <= v36)
                    {
                        v40 = 14;
                        if (14 <= v36)
                        {
                            v5 = v13;
                            v4 = v9;
                            v13 = v5;
                            v9 = v4;
                            v40 = sub_410b30(v36, 1);
                        }
                        v41 = index->field_8;
                        if (v40 > 0x7ffffffffffffff)
                            goto LABEL_4116ed;
                        v6 = v40;
                        v42 = v40 * 32;
                        v5 = v13;
                        v4 = v9;
                        if (ptr1 != v41)
                        {
                            v9 = v4;
                            v13 = v5;
                            v12 = v6;
                            ptr = realloc(v41, v42);
                        }
                        else
                        {
                            v12 = v6;
                            v13 = v5;
                            v9 = v4;
                            ptr = malloc(v42);
                        }
                        v41 = index->field_8;
                        if (!ptr)
                            goto LABEL_4116ed;
                        if (ptr1 == v41)
                        {
                            v6 = v12;
                            v5 = v13;
                            v4 = v9;
                            v12 = v6;
                            v13 = v5;
                            v9 = v4;
                            ptr = memcpy(ptr, ptr1, index->field_0 * 32);
                        }
                        index->field_8 = ptr;
                    }
                    else
                    {
                        ptr = index->field_8;
                    }
                    v43 = index->field_0;
                    v44 = &ptr[2 * v43];
                    if (v43 <= v36)
                    {
                        do
                        {
                            v45 = v44;
                            v43 += 1;
                            *(v45) = 0;
                            v44 = v45 + 8;
                        } while (v43 <= v36);
                        index->field_0 = v43;
                        *(v45) = 0;
                    }
                    v47 = &ptr[2 * v36];
                    if (!*(v47))
                    {
                        *(v47) = 5;
                        v16 = *(iter1);
                        if (v16 != 46)
                            goto LABEL_410f47;
LABEL_411278:
                        if (iter1[1] != 42)
                        {
                            *((char **)&idx1->field_30) = iter1;
                            v68 = iter1 + 1;
                            if (iter1[1] - 48 <= 9)
                            {
                                do
                                {
                                    v70 = v68 + 1;
                                    v68 = v70;
                                } while (v69[1] - 48 <= 9);
                                v71 = v70 - iter1;
                                iter1 = v70;
                                v68 = v70;
                            }
                            else
                            {
                                iter1 = v68;
                                v71 = 1;
                            }
                            v72 = v1;
                            idx1->field_38 = v68;
                            v16 = *(v68);
                            if (v71 <= v72)
                                v71 = v72;
                            v1 = v71;
                            goto LABEL_410f47;
                        }
                        v48 = v1;
                        v49 = 2;
                        node = iter1 + 2;
                        *((char **)&idx1->field_30) = iter1;
                        idx1->field_38 = node;
                        if (2 <= v48)
                            v49 = v48;
                        v51 = iter1[2];
                        v1 = v49;
                        if (iter1[2] - 48 > 9)
                        {
LABEL_4112b5:
                            v55 = idx1->field_40;
                            if (v55 != 0xffffffffffffffff)
                                goto LABEL_4112c4;
                            v56 = v3;
                            idx1->field_40 = v56;
                            if (v56 == 0xffffffffffffffff)
                            {
                                ptr = index->field_8;
                                break;
                            }
                            else
                            {
                                v55 = v3;
                                v3 = v56 + 1;
                                goto LABEL_4112c4;
                            }
                        }
                        else
                        {
                            v52 = node;
                            do
                            {
                                v54 = v52 + 1;
                                v52 = v54;
                            } while (v53[1] - 48 <= 9);
                            if (v53[1] != 36)
                                goto LABEL_4112b5;
                            v4 = v9;
                            v57 = 0;
                            v5 = v12;
                            while (1)
                            {
                                v58 = 0xffffffffffffffff;
                                if (v57 <= 1844674407370955161)
                                    v58 = v57 * 10;
                                v57 = sub_410b30(v58, v51 - 48);
                                v51 = node[1];
                                if (v51 - 48 > 9)
                                    break;
                                node += 1;
                            }
                            v9 = v4;
                            v12 = v5;
                            v55 = v57 - 1;
                            if (v55 > 0xfffffffffffffffd)
                            {
                                ptr = index->field_8;
                                break;
                            }
                            else
                            {
                                idx1->field_40 = v55;
                                node += 2;
LABEL_4112c4:
                                if (v12 > v55)
                                {
                                    ptr = index->field_8;
                                    goto LABEL_4112d1;
                                }
                                else
                                {
                                    v59 = v12 * 2;
                                    if (v59 <= v55)
                                    {
                                        v6 = v13;
                                        v5 = v9;
                                        v4 = node;
                                        v13 = v6;
                                        v9 = v5;
                                        node = v4;
                                        v59 = sub_410b30(v55, 1);
                                    }
                                    v41 = index->field_8;
                                    if (v59 <= 0x7ffffffffffffff)
                                    {
                                        v7 = v59;
                                        v60 = v59 * 32;
                                        v6 = v13;
                                        v5 = v9;
                                        v4 = node;
                                        if (ptr1 != v41)
                                        {
                                            node = v4;
                                            v9 = v5;
                                            v61 = v6;
                                            v62 = v7;
                                            ptr = realloc(v41, v60);
                                        }
                                        else
                                        {
                                            v62 = v7;
                                            v61 = v6;
                                            v9 = v5;
                                            node = v4;
                                            ptr = malloc(v60);
                                        }
                                        v41 = index->field_8;
                                        if (ptr)
                                        {
                                            if (ptr1 == v41)
                                            {
                                                v7 = v62;
                                                v6 = v61;
                                                v5 = v9;
                                                v4 = node;
                                                v9 = v5;
                                                node = v4;
                                                ptr = memcpy(ptr, ptr1, index->field_0 * 32);
                                            }
                                            index->field_8 = ptr;
LABEL_4112d1:
                                            v63 = index->field_0;
                                            v64 = &ptr[2 * v63];
                                            if (v63 <= v55)
                                            {
                                                do
                                                {
                                                    v65 = v64;
                                                    v63 += 1;
                                                    *(v65) = 0;
                                                    v64 = v65 + 8;
                                                } while (v63 <= v55);
                                                index->field_0 = v63;
                                                *(v65) = 0;
                                            }
                                            v67 = &ptr[2 * v55];
                                            if (!*(v67))
                                            {
                                                *(v67) = 5;
                                                iter1 = node;
                                                v16 = *(node);
                                                goto LABEL_410f47;
                                            }
                                            if (*(v67) != 5)
                                                break;
                                            v16 = *(node);
                                            iter1 = node;
LABEL_410f47:
                                            iter2 = iter1 + 1;
                                            if (v16 == 104)
                                                goto LABEL_410fb0;
                                            while (1)
                                            {
                                                switch (v16)
                                                {
                                                case 76:
                                                    break;
                                                case 90: case 106: case 108: case 116: case 122:
                                                    break;
                                                default:
                                                    switch (v16)
                                                    {
                                                    case 38:
                                                        break;
                                                    default:
                                                        ptr = index->field_8;
                                                        break;
                                                    }
                                                }
                                                while (1)
                                                {
                                                    iter2 += 1;
                                                    if (*(iter2) != 104)
                                                        break;
LABEL_410fb0:
                                                }
                                            }
                                            break;
                                        }
                                    }
                                }
LABEL_4116ed:
                                if (ptr1 != v41)
                                {
                                    v0 = v9;
                                    free(v41);
                                    v9 = v0;
                                }
                                if (v9 != idx->field_8)
                                    free(idx->field_8);
                                *(__errno_location()) = 12;
                                return 0xffffffff;
                            }
                        }
                    }
                    else
                    {
                        if (*(v47) != 5)
                            break;
LABEL_41145c:
                        v16 = *(iter1);
LABEL_410f3d:
                        if (v16 != 46)
                            goto LABEL_410f47;
                        goto LABEL_411278;
                    }
                }
            }
            while (1)
            {
LABEL_410fc8:
                *((unsigned int *)&idx1->field_10) = v24;
                iter += 1;
                if (*(iter) != 39)
                    break;
LABEL_410fe1:
                v24 = (int)idx1->field_10 | 1;
            }
        }
    }
    else
    {
        v17 = iter1;
        do
        {
            v19 = v17 + 1;
            v17 = v19;
        } while (v18[1] - 48 <= 9);
        if (v18[1] != 36)
            goto LABEL_410efe;
        v20 = v16;
        v4 = 7;
        v21 = 0;
        while (1)
        {
            v22 = 0xffffffffffffffff;
            if (v21 <= 1844674407370955161)
                v22 = v21 * 10;
            v21 = sub_410b30(v22, (char)(v20 - 48), v20 - 48, 1844674407370955161);
            v20 = iter1[1];
            if ((char)(v20 - 48) > 9)
                break;
            iter1 += 1;
        }
        v13 = v4;
        if (v21 - 1 <= 0xfffffffffffffffd)
        {
            v16 = iter1[2];
            iter1 += 2;
            goto LABEL_410efe;
        }
        else
        {
            ptr = index->field_8;
        }
    }
    if (ptr1 != ptr)
    {
        v0 = v9;
        free(ptr);
        v9 = v0;
    }
    if (v9 != idx->field_8)
        free(idx->field_8);
    *(__errno_location()) = 22;
    return 0xffffffff;
}



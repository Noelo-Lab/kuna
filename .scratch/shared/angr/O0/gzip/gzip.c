// Function: bi_init @ 0x3a29
extern unsigned long long g_41a050;
extern unsigned int g_41a058;
extern unsigned short g_41a05c;
extern unsigned int g_41a060;

long long bi_init(unsigned int a0)
{
    unsigned long v1;  // rax

    g_41a058 = a0;
    g_41a05c = 0;
    g_41a060 = 0;
    v1 = g_41a058;
    if (g_41a058 != 0xffffffff)
    {
        v1 = sub_4101d7;
        g_41a050 = sub_4101d7;
    }
    return v1;
}



// Function: send_bits @ 0x3a6c
extern unsigned short g_41a05c;
extern unsigned int g_41a060;
extern char g_45c000;
extern unsigned int g_4dd870;

int send_bits(unsigned short a0, unsigned int a1)
{
    unsigned int v1;  // eax
    char v2;  // al
    unsigned int v3;  // eax
    unsigned int v4;  // eax
    char v5;  // al
    unsigned int v6;  // eax

    if (16 - a1 >= g_41a060)
    {
        g_41a05c = g_41a05c | (unsigned short)(a0 << ((char)g_41a060 & 31));
        g_41a060 = a1 + g_41a060;
        return g_41a060;
    }
    g_41a05c = g_41a05c | (unsigned short)(a0 << ((char)g_41a060 & 31));
    if (g_4dd870 <= 0x3fffd)
    {
        v1 = g_4dd870;
        g_4dd870 = v1 + 1;
        *(&(&g_45c000)[v1]) = g_41a05c;
        v2 = g_41a05c >> 8;
        v3 = g_4dd870;
        g_4dd870 = v3 + 1;
        *(&(&g_45c000)[v3]) = v2;
    }
    else
    {
        v4 = g_4dd870;
        g_4dd870 = v4 + 1;
        *(&(&g_45c000)[v4]) = g_41a05c;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v5 = g_41a05c >> 8;
        v6 = g_4dd870;
        g_4dd870 = v6 + 1;
        *(&(&g_45c000)[v6]) = v5;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    g_41a05c = (short)(a0) >> ((char)(16 - g_41a060 & 0xffffffff) & 31);
    g_41a060 = a1 + g_41a060 - 16;
    return g_41a060;
}



// Function: bi_reverse @ 0x3bed
unsigned int bi_reverse(unsigned int a0, int a1)
{
    unsigned int v4;  // ebx
    unsigned int v5;  // ebx
    unsigned int v6;  // ebx
    int i;  // [bp-0x18]
    unsigned int v1;  // [bp-0x14]
    unsigned int v2;  // [bp-0x14]

    v1 = a0;
    i = a1;
    v4 = 0;
    do
    {
        v2 = v1;
        v1 = v2 >> 1;
        v6 = (v4 | v2 & 1) * 2;
        i -= 1;
        v4 = v6;
    } while (i > 0);
    return (v5 | v2 & 1) & 0x7fffffff;
}



// Function: bi_windup @ 0x3c22
extern unsigned short g_41a05c;
extern void g_41a060;
extern unsigned long long g_45c000;
extern unsigned int g_4dd870;

unsigned long long bi_windup(void)
{
    unsigned int v1;  // eax
    char v2;  // al
    unsigned int v3;  // eax
    unsigned long long v4;  // rax
    unsigned int v5;  // eax
    char v6;  // al
    unsigned int v7;  // eax
    unsigned int v8;  // eax

    if (*((int *)&g_41a060) <= 8)
    {
        v4 = *((int *)&g_41a060);
        if (*((int *)&g_41a060) > 0)
        {
            v8 = g_4dd870;
            g_4dd870 = v8 + 1;
            *(v8 + (char *)&g_45c000) = g_41a05c;
            v4 = g_4dd870;
            if (g_4dd870 == 0x40000)
                v4 = sub_40f28e();
        }
    }
    else if (g_4dd870 <= 0x3fffd)
    {
        v1 = g_4dd870;
        g_4dd870 = v1 + 1;
        *(v1 + (char *)&g_45c000) = g_41a05c;
        v2 = g_41a05c >> 8;
        v3 = g_4dd870;
        g_4dd870 = v3 + 1;
        v4 = &g_45c000;
        *(v3 + (char *)&g_45c000) = v2;
    }
    else
    {
        v5 = g_4dd870;
        g_4dd870 = v5 + 1;
        *(v5 + (char *)&g_45c000) = g_41a05c;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v6 = g_41a05c >> 8;
        v7 = g_4dd870;
        g_4dd870 = v7 + 1;
        *(v7 + (char *)&g_45c000) = v6;
        v4 = g_4dd870;
        if (g_4dd870 == 0x40000)
            v4 = sub_40f28e();
    }
    g_41a05c = 0;
    *((unsigned int *)&g_41a060) = 0;
    return v4;
}



// Function: copy_block @ 0x3d59
extern char g_45c000;
extern unsigned int g_4dd870;

long long copy_block(char *a0, unsigned int a1, unsigned int a2)
{
    unsigned int v3;  // eax
    unsigned int v4;  // eax
    char v13;  // cl
    unsigned int v14;  // eax
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    unsigned int v7;  // eax
    unsigned int v8;  // eax
    unsigned int v9;  // eax
    unsigned int v10;  // eax
    unsigned long v11;  // rax
    char *v12;  // rax
    unsigned int v0;  // [bp-0x14]
    char *v1;  // [bp-0x10]

    v1 = a0;
    v0 = a1;
    sub_403c22();
    if (a2)
    {
        if (g_4dd870 <= 0x3fffd)
        {
            v3 = g_4dd870;
            g_4dd870 = v3 + 1;
            *(&(&g_45c000)[v3]) = v0;
            v4 = g_4dd870;
            g_4dd870 = v4 + 1;
            *(&(&g_45c000)[v4]) = _INSERT(v0, 0, (unsigned short)v0 >> 8) & 0xffffffff;
        }
        else
        {
            v5 = g_4dd870;
            g_4dd870 = v5 + 1;
            *(&(&g_45c000)[v5]) = v0;
            if (g_4dd870 == 0x40000)
                sub_40f28e();
            v6 = g_4dd870;
            g_4dd870 = v6 + 1;
            *(&(&g_45c000)[v6]) = _INSERT(v0, 0, (unsigned short)v0 >> 8) & 0xffffffff;
            if (g_4dd870 == 0x40000)
                sub_40f28e();
        }
        if (g_4dd870 <= 0x3fffd)
        {
            v7 = g_4dd870;
            g_4dd870 = v7 + 1;
            *(&(&g_45c000)[v7]) = ~(v0);
            v8 = g_4dd870;
            g_4dd870 = v8 + 1;
            *(&(&g_45c000)[v8]) = _INSERT(~(v0), 0, (unsigned short)~(v0) >> 8) & 0xffffffff;
        }
        else
        {
            v9 = g_4dd870;
            g_4dd870 = v9 + 1;
            *(&(&g_45c000)[v9]) = ~(v0);
            if (g_4dd870 == 0x40000)
                sub_40f28e();
            v10 = g_4dd870;
            g_4dd870 = v10 + 1;
            *(&(&g_45c000)[v10]) = _INSERT(~(v0), 0, (unsigned short)~(v0) >> 8) & 0xffffffff;
            if (g_4dd870 == 0x40000)
                sub_40f28e();
        }
    }
    while (1)
    {
        v11 = v0;
        v0 = v11 - 1;
        if (!(unsigned int)v11)
            return v11;
        v12 = v1;
        v1 = v12 + 1;
        v13 = *(v12);
        v14 = g_4dd870;
        g_4dd870 = v14 + 1;
        *(&(&g_45c000)[v14]) = v13;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
}



// Function: lm_init @ 0x3f52
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned short g_419040[4];
extern unsigned short g_419042[4];
extern unsigned short g_419044[4];
extern unsigned short g_419046[4];
extern struct_0 *g_41a050;
extern unsigned long long g_41a068;
extern unsigned int g_41a074;
extern unsigned int g_41a07c;
extern unsigned int g_41a080;
extern unsigned int g_41a084;
extern unsigned int g_41a088;
extern unsigned int g_41a08c;
extern unsigned int g_41a090;
extern unsigned long long g_41a098;
extern unsigned long long g_41a0a0;
extern unsigned int g_41a0a8;
extern char g_4ad000;
extern void g_4cd000;

long long lm_init(int a0)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rax
    unsigned int i;  // ebx

    if (a0 > 0 && a0 <= 9)
    {
        memset(&g_4cd000, 0, 0x10000);
        g_41a0a0 = 0xffffffff;
        g_41a098 = 0;
        g_41a090 = g_419042[4 * a0];
        g_41a080 = g_419040[4 * a0];
        g_41a0a8 = g_419044[4 * a0];
        g_41a07c = g_419046[4 * a0];
        g_41a074 = 0;
        g_41a068 = 0;
        g_41a08c = g_41a050(&g_4ad000, 0x10000, &g_4ad000);
        v1 = g_41a08c;
        if (g_41a08c)
        {
            v1 = g_41a08c;
            if (g_41a08c != 0xffffffff)
            {
                g_41a088 = 0;
                while (1)
                {
                    v2 = g_41a08c;
                    if (g_41a08c > 261 || !(v2 = (unsigned long)(unsigned long long)g_41a088, !g_41a088))
                        break;
                    sub_404350();
                }
                g_41a084 = 0;
                for (i = 0; i <= 1; i += 1)
                {
                    g_41a084 = (*(&(&g_4ad000)[i]) ^ g_41a084 * 32) & 0x7fff;
                }
                return v2;
            }
        }
        g_41a088 = 1;
        g_41a08c = 0;
        return v1;
    }
    sub_40f5ef("bad pack level"); /* do not return */
}



// Function: longest_match @ 0x40fe
extern int g_41a070;
extern unsigned int g_41a074;
extern unsigned int g_41a078;
extern unsigned int g_41a07c;
extern char g_41a080;
extern char g_41a0a8;
extern char g_4ad000;
extern char g_4ad102;
extern unsigned short g_4bd000[4];

int longest_match(unsigned int a0)
{
    unsigned long v5;  // rbx
    unsigned long v6;  // r13
    unsigned long v15;  // r12
    unsigned long v16;  // r12
    unsigned long v17;  // r12
    unsigned long v18;  // r12
    unsigned long v19;  // r12
    int v20;  // r12d
    char v7;  // r14b
    char v8;  // r15b
    char *v9;  // r12
    unsigned long v10;  // r12
    unsigned long iter;  // rbx
    unsigned long v12;  // r12
    unsigned long v13;  // r12
    unsigned long v14;  // r12
    unsigned int v0;  // [bp-0x44]
    unsigned int v1;  // [bp-0x3c]
    int v2;  // [bp-0x38]
    unsigned int v3;  // [bp-0x34]

    v0 = a0;
    v1 = g_41a07c;
    v5 = &(&g_4ad000)[g_41a074];
    v2 = g_41a070;
    v3 = (g_41a074 < 32506 ? 32506 : g_41a074) - 32506;
    v6 = &(&g_4ad102)[g_41a074];
    v7 = *((char *)(v2 - 1 + v5));
    v8 = *((char *)(v2 + v5));
    if (g_41a070 >= *((int *)&g_41a080))
        v1 >>= 2;
    while (1)
    {
        v9 = &(&g_4ad000)[v0];
        if (v8 == v9[v2] && v7 == *(&v9[v2] - 1) && *(v9) == *((char *)v5) && !(v10 = (unsigned long)(v9 + 1), *((char *)v10) != *((char *)(v5 + 1))))
        {
            iter = v5 + 2;
            v12 = v10 + 1;
            do
            {
                iter += 1;
                v13 = v12 + 1;
            } while (...);
            v20 = 258 - (unsigned int)(v6 - iter);
            v5 = v6 - 258;
            if (v20 > v2)
            {
                g_41a078 = v0;
                v2 = v20;
                if (v2 >= *((int *)&g_41a0a8))
                    return v2;
                v7 = *((char *)(v2 - 1 + v5));
                v8 = *((char *)(v2 + v5));
            }
        }
        v0 = g_4bd000[v0 & 0x7fff];
        if (v0 <= v3)
            return v2;
        v1 -= 1;
        if (!v1)
            return v2;
    }
}



// Function: fill_window @ 0x4350
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_419020;
extern struct_0 *g_41a050;
extern unsigned long long g_41a068;
extern unsigned int g_41a074;
extern unsigned int g_41a078;
extern unsigned int g_41a088;
extern unsigned int g_41a08c;
extern unsigned long long g_41a0a0;
extern void g_4ad000;
extern void g_4b5000;
extern unsigned short g_4bd000[4];

void* fill_window(void)
{
    unsigned int i;  // ebx
    unsigned int v3;  // r12d
    unsigned int j;  // ebx
    unsigned int v5;  // r12d
    void* v6;  // rax
    unsigned long long v7;  // rcx
    unsigned int v8;  // ebx
    unsigned int iter;  // [bp-0x1c]

    iter = g_419020 - g_41a08c - g_41a074;
    if (iter == 0xffffffff)
    {
        iter -= 1;
    }
    else if (g_41a074 > 65273)
    {
        memcpy(&g_4ad000, &g_4b5000, 0x8000);
        g_41a078 = g_41a078 - 0x8000;
        g_41a074 = g_41a074 - 0x8000;
        if (g_41a0a0 != 0xffffffff)
            g_41a0a0 = g_41a0a0 - 0x8000;
        g_41a068 = g_41a068 - 0x8000;
        for (i = 0; i <= 0x7fff; i += 1)
        {
            v3 = g_4bd000[0x8000 + i];
            g_4bd000[0x8000 + i] = (v3 <= 0x7fff ? 0 : (unsigned short)(v3 - 0x8000));
        }
        for (j = 0; j <= 0x7fff; j += 1)
        {
            v5 = g_4bd000[j];
            g_4bd000[j] = (v5 <= 0x7fff ? 0 : (unsigned short)(v5 - 0x8000));
        }
        iter += 0x8000;
    }
    v6 = g_41a088;
    if ((unsigned int)v6)
        return v6;
    v7 = g_41a074 + g_41a08c;
    v8 = g_41a050(&(&g_4ad000)[v7], iter, g_41a050, &(&g_4ad000)[v7]);
    if (v8 && v8 != 0xffffffff)
    {
        g_41a08c = g_41a08c + v8;
        return g_41a08c;
    }
    g_41a088 = 1;
    return memset(&(&g_4ad000)[g_41a08c + g_41a074], 0, 2);
}



// Function: rsync_roll @ 0x4567
extern unsigned long long g_41a098;
extern unsigned long long g_41a0a0;
extern char g_4ac000;
extern char g_4ad000;

long long rsync_roll(unsigned int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]
    unsigned int iter;  // [bp-0xc]

    v1 = a0;
    v0 = a1;
    if (v1 <= 0xfff)
    {
        for (iter = v1; iter <= 0xfff; iter += 1)
        {
            if (iter != v0 + v1)
                g_41a098 = g_41a098 + *(&(&g_4ad000)[iter]);
            else
                return v0 + v1;
        }
        v0 = v0 + v1 - 0x1000;
        v1 = 0x1000;
    }
    for (iter = v1; iter < v0 + v1; iter += 1)
    {
        g_41a098 = g_41a098 + *(&(&g_4ad000)[iter]);
        g_41a098 = g_41a098 - *(&(&g_4ac000)[iter]);
        if (g_41a0a0 == 0xffffffff && !((unsigned short)g_41a098 & 0xfff))
            g_41a0a0 = iter;
    }
    return v0 + v1;
}



// Function: deflate_fast @ 0x4677
extern unsigned long long g_419020;
extern long long g_41a068;
extern unsigned int g_41a070;
extern unsigned int g_41a074;
extern unsigned int g_41a078;
extern unsigned int g_41a084;
extern unsigned int g_41a088;
extern unsigned int g_41a08c;
extern char g_41a090;
extern unsigned long long g_41a0a0;
extern char g_4ad000;
extern char g_4ad001;
extern char g_4ad002;
extern unsigned short g_4bd000[4];
extern char g_4dd874;

long long deflate_fast(void)
{
    unsigned int v0;  // [bp-0x14]
    unsigned int iter;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    v0 = 0;
    iter = 0;
    g_41a070 = 2;
    while (g_41a08c)
    {
        g_41a084 = (*(&(&g_4ad002)[g_41a074]) ^ g_41a084 * 32) & 0x7fff;
        v2 = g_4bd000[0x8000 + g_41a084];
        g_4bd000[g_41a074 & 0x7fff] = v2;
        g_4bd000[0x8000 + g_41a084] = g_41a074;
        if (v2 && g_41a074 - v2 <= 32506 && g_41a074 <= g_419020 - 262)
        {
            iter = sub_4040fe(v2);
            if (iter > g_41a08c)
                iter = g_41a08c;
        }
        if (iter > 2)
        {
            v0 = sub_40c45d(g_41a074 - g_41a078 & 0xffffffff, iter - 3, iter - 3, g_41a078);
            g_41a08c = g_41a08c - iter;
            if (*((int *)&g_4dd874))
                sub_404567(g_41a074, iter);
            if (iter <= *((int *)&g_41a090))
            {
                iter -= 1;
                do
                {
                    g_41a074 = g_41a074 + 1;
                    g_41a084 = (*(&(&g_4ad002)[g_41a074]) ^ g_41a084 * 32) & 0x7fff;
                    v2 = g_4bd000[0x8000 + g_41a084];
                    g_4bd000[g_41a074 & 0x7fff] = v2;
                    g_4bd000[0x8000 + g_41a084] = g_41a074;
                    iter -= 1;
                } while (iter);
                g_41a074 = g_41a074 + 1;
            }
            else
            {
                g_41a074 = iter + g_41a074;
                iter = 0;
                g_41a084 = *(&(&g_4ad000)[g_41a074]);
                g_41a084 = (*(&(&g_4ad001)[g_41a074]) ^ g_41a084 * 32) & 0x7fff;
            }
        }
        else
        {
            v0 = sub_40c45d(0, *(&(&g_4ad000)[g_41a074]), g_41a074);
            if (*((int *)&g_4dd874))
                sub_404567(g_41a074, 1);
            g_41a08c = g_41a08c - 1;
            g_41a074 = g_41a074 + 1;
        }
        if (*((int *)&g_4dd874) && g_41a074 > g_41a0a0)
        {
            g_41a0a0 = 0xffffffff;
            v0 = 2;
        }
        if (v0)
        {
            sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
            g_41a068 = g_41a074;
        }
        while (g_41a08c <= 261 && !g_41a088)
        {
            sub_404350();
        }
    }
    return sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
}



// Function: deflate @ 0x4a9e
extern unsigned long long g_419020;
extern long long g_41a068;
extern unsigned int g_41a070;
extern unsigned int g_41a074;
extern unsigned int g_41a078;
extern unsigned int g_41a084;
extern unsigned int g_41a088;
extern unsigned int g_41a08c;
extern char g_41a090;
extern unsigned long long g_41a0a0;
extern char g_4ad000;
extern char g_4ad002;
extern unsigned short g_4bd000[4];
extern char g_4dd874;

long long deflate(int a0)
{
    unsigned int v5;  // ebx
    unsigned long v6;  // rdx
    unsigned long v7;  // rdx
    unsigned int v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x24]
    unsigned int v2;  // [bp-0x20]
    unsigned int v3;  // [bp-0x1c]

    v0 = 0;
    v1 = 0;
    v5 = 2;
    sub_403f52(a0);
    if (a0 <= 3)
        return sub_404677();
    while (g_41a08c)
    {
        g_41a084 = (*(&(&g_4ad002)[g_41a074]) ^ g_41a084 * 32) & 0x7fff;
        v2 = g_4bd000[0x8000 + g_41a084];
        g_4bd000[g_41a074 & 0x7fff] = v2;
        g_4bd000[0x8000 + g_41a084] = g_41a074;
        g_41a070 = v5;
        v3 = g_41a078;
        v5 = 2;
        if (v2 && g_41a070 < *((int *)&g_41a090) && g_41a074 - v2 <= 32506 && g_41a074 <= g_419020 - 262)
        {
            v5 = sub_4040fe(v2);
            if (v5 > g_41a08c)
                v5 = g_41a08c;
            if (v5 == 3 && g_41a074 - g_41a078 > 0x1000)
                v5 -= 1;
        }
        if (g_41a070 > 2 && v5 <= g_41a070)
        {
            v0 = sub_40c45d(g_41a074 - v3 - 1, g_41a070 - 3, g_41a070 - 3);
            g_41a08c = g_41a08c - g_41a070 + 1;
            g_41a070 = g_41a070 - 2;
            if (*((int *)&g_4dd874))
                sub_404567(g_41a074, g_41a070 + 1);
            do
            {
                g_41a074 = g_41a074 + 1;
                g_41a084 = (*(&(&g_4ad002)[g_41a074]) ^ g_41a084 * 32) & 0x7fff;
                v2 = g_4bd000[0x8000 + g_41a084];
                g_4bd000[g_41a074 & 0x7fff] = v2;
                g_4bd000[0x8000 + g_41a084] = g_41a074;
                g_41a070 = g_41a070 - 1;
            } while (g_41a070);
            v1 = 0;
            v5 = 2;
            g_41a074 = g_41a074 + 1;
            if (*((int *)&g_4dd874) && g_41a074 > g_41a0a0)
            {
                g_41a0a0 = 0xffffffff;
                v0 = 2;
            }
            if (v0)
            {
                sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
                g_41a068 = g_41a074;
            }
        }
        else if (v1)
        {
            v6 = g_41a074 - 1;
            v0 = sub_40c45d(0, (&g_4ad000)[v6], v6);
            if (*((int *)&g_4dd874) && g_41a074 > g_41a0a0)
            {
                g_41a0a0 = 0xffffffff;
                v0 = 2;
            }
            if (v0)
            {
                sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
                g_41a068 = g_41a074;
            }
            if (*((int *)&g_4dd874))
                sub_404567(g_41a074, 1);
            g_41a074 = g_41a074 + 1;
            g_41a08c = g_41a08c - 1;
        }
        else
        {
            if (*((int *)&g_4dd874) && g_41a074 > g_41a0a0)
            {
                g_41a0a0 = 0xffffffff;
                v0 = 2;
                sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
                g_41a068 = g_41a074;
            }
            v1 = 1;
            if (*((int *)&g_4dd874))
                sub_404567(g_41a074, 1);
            g_41a074 = g_41a074 + 1;
            g_41a08c = g_41a08c - 1;
        }
        while (g_41a08c <= 261 && !g_41a088)
        {
            sub_404350();
        }
    }
    if (!v1)
        return sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
    v7 = g_41a074 - 1;
    sub_40c45d(0, (&g_4ad000)[v7], v7);
    return sub_40c1ea((g_41a068 < 0 ? 0 : &(&g_4ad000)[g_41a068 & 0xffffffff]));
}



// Function: try_help @ 0x5097
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;

int try_help(void)
{
    fprintf(stderr, "Try `%s --help' for more information.\n", g_4dd010);
    sub_408e40(1); /* do not return */
}



// Function: help @ 0x50ce
extern char *g_418a40;
extern char *g_4dd010;

unsigned long long help(void)
{
    char **v0;  // [bp-0x10]
    char **v1;  // [bp-0x10]

    v0 = &g_418a40;
    printf("Usage: %s [OPTION]... [FILE]...\n", g_4dd010);
    while (1)
    {
        v1 = v0;
        if (!*(v1))
            break;
        v0 = v1 + 1;
        printf("%s\n", *(v1));
    }
    return 0;
}



// Function: license @ 0x513b
extern char *g_4186a0;
extern char *g_419558;
extern char *g_4dd010;

unsigned long long license(void)
{
    char **v0;  // [bp-0x10]
    char **v1;  // [bp-0x10]

    v0 = &g_4186a0;
    printf("%s %s\n", g_4dd010, g_419558);
    while (1)
    {
        v1 = v0;
        if (!*(v1))
            break;
        v0 = v1 + 1;
        printf("%s\n", *(v1));
    }
    return 0;
}



// Function: version @ 0x51af
int version(void)
{
    sub_40513b();
    printf("\n");
    return printf("Written by Jean-loup Gailly.\n");
}



// Function: progerror @ 0x51e7
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;
extern unsigned int g_4dd018;

void progerror(char *a0)
{
    unsigned int v0;  // [bp-0xc]

    v0 = *(__errno_location());
    fprintf(stderr, "%s: ", g_4dd010);
    *(__errno_location()) = v0;
    perror(a0);
    g_4dd018 = 1;
    return;
}



// Function: main @ 0x5249
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned long long g_4186e0[4];
extern int g_419090;
extern int g_419094;
extern unsigned int g_419098;
extern unsigned int g_4190a0;
extern long long stdout;
extern unsigned int optind;
extern struct_0 *optarg;
extern FILE *stderr;
extern unsigned int g_4dd000;
extern unsigned int g_4dd004;
extern unsigned int g_4dd008;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd060;
extern unsigned int g_4dd874;
extern char g_4dd878;
extern char g_4dd879;
extern unsigned int g_4dd87c;
extern unsigned int g_4dd880;
extern unsigned int g_4dd884;
extern unsigned int g_4dd888;
extern unsigned int g_4dd88c;
extern unsigned int g_4dd890;
extern unsigned int g_4dd894;
extern unsigned long long g_4dd8a8;
extern char *g_4dd8b0;
extern unsigned long long g_4dd8b8;
extern unsigned int g_4dd940;
extern char g_4ddd60;

int main(int i, long long *a1)
{
    unsigned long v9;  // fs
    int v10;  // eax
    long long v19;  // rdx
    long long v20;  // rcx
    long long v21;  // r8
    long long v22;  // r9
    long long v23;  // rdi
    long long v24;  // rsi
    long long v25;  // rdx
    long long v26;  // rcx
    long long v27;  // r8
    long long v28;  // r9
    long long v11;  // rdi
    long long v29;  // rdi
    long long v30;  // rsi
    long long v31;  // rdx
    long long v32;  // rcx
    long long v33;  // r8
    long long v34;  // r9
    long long v12;  // rsi
    long long v13;  // rdx
    long long v14;  // rcx
    long long v15;  // r8
    long long v16;  // r9
    long long v17;  // rdi
    long long v18;  // rsi
    char v0;  // [bp-0x38]
    int v1;  // [bp-0x34]
    unsigned int iter;  // [bp-0x30]
    int v3;  // [bp-0x2c]
    long long *v4;  // [bp-0x28]
    long long *v5;  // [bp-0x20]
    unsigned long len;  // [bp-0x18]
    unsigned long v7;  // [bp-0x10]

    v7 = *((long long *)(40 + v9));
    g_4dd010 = sub_40f40a(*(a1));
    len = strlen(g_4dd010);
    if (len > 4 && !strcmp(len + g_4dd010 - 4, ".exe"))
        *(&g_4dd010[len] - 4) = 0;
    v4 = a1;
    g_4dd8a8 = sub_40f454(&v0, &v4, "GZIP", &v4);
    v5 = (!g_4dd8a8 ? NULL : v4);
    g_4dd8b0 = ".gz";
    g_4dd8b8 = strlen(g_4dd8b0);
    while (1)
    {
        v1 = -0x1;
        if (v5)
        {
            if (v5[optind] && !strcmp(v5[optind], "--"))
            {
                iter = 176;
            }
            else
            {
                iter = getopt_long(v0, v5, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &g_4186e0[0], &v1);
                if (iter >= 0)
                {
                    iter += 131;
                }
                else
                {
                    if (optind != v0)
                    {
                        fprintf(stderr, "%s: %s: non-option in GZIP environment variable\n", g_4dd010, v5[optind]);
                        sub_405097(); /* do not return */
                    }
                    if (v0 != 1 && !g_4dd004)
                        fprintf(stderr, "%s: warning: GZIP environment variable is deprecated; use an alias or script\n", g_4dd010);
                    free(v5);
                    v5 = NULL;
                    optind = 1;
                    v1 = -0x1;
                }
            }
        }
        if (!v5)
            iter = getopt_long(i, a1, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &g_4186e0[0], &v1);
        if (iter < 0)
            break;
        switch (iter)
        {
        case 72: case 104:
            sub_4050ce();
            sub_408e91(v17, v18, v19, v20, v21, v22); /* do not return */
        case 76:
            sub_40513b();
            sub_408e91(v23, v24, v25, v26, v27, v28); /* do not return */
        case 77:
            g_419094 = 0;
            break;
        case 78: case 209:
            g_419094 = 0;
            g_419090 = g_419094;
            break;
        case 83:
            g_4dd8b8 = strlen(optarg);
            g_4dd8b0 = optarg;
            break;
        case 86:
            sub_4051af();
            sub_408e91(v29, v30, v31, v32, v33, v34); /* do not return */
        case 90:
            fprintf(stderr, "%s: -Z not supported in this version\n", g_4dd010);
            sub_405097(); /* do not return */
        case 97:
            g_4dd87c = 1;
            break;
        case 98:
            for (g_419098 = atoi(optarg); (char)optarg->field_0; optarg = (char *)&optarg->field_0 + 1)
            {
                if ((char)optarg->field_0 <= 47 || (char)optarg->field_0 > 57)
                {
                    fprintf(stderr, "%s: -b operand is not an integer\n", g_4dd010);
                    sub_405097(); /* do not return */
                }
            }
        case 99:
            g_4dd000 = 1;
            break;
        case 100:
            g_4dd880 = 1;
            break;
        case 102:
            g_4dd884 = g_4dd884 + 1;
            break;
        case 107:
            g_4dd888 = 1;
            break;
        case 108:
            g_4dd000 = 1;
            g_4dd008 = g_4dd000;
            g_4dd880 = g_4dd008;
            g_4dd890 = g_4dd880;
            break;
        case 109:
            g_419094 = 1;
            break;
        case 110: case 241:
            g_419094 = 1;
            g_419090 = g_419094;
            break;
        case 113: case 244:
            g_4dd004 = 1;
            g_4dd894 = 0;
            break;
        case 114:
            g_4dd88c = 1;
            break;
        case 116:
            g_4dd000 = 1;
            g_4dd880 = g_4dd000;
            g_4dd008 = g_4dd880;
            break;
        case 118: case 249:
            g_4dd894 = g_4dd894 + 1;
            g_4dd004 = 0;
            break;
        case 128:
            g_4dd878 = 1;
            break;
        case 129: case 260:
            g_4dd874 = 1;
            break;
        case 130:
            g_4dd879 = 1;
            break;
        case 180: case 181: case 182: case 183: case 184: case 185: case 186: case 187: case 188:
            iter -= 131;
        case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
            g_4190a0 = iter - 48;
            break;
        default:
            if (iter > 130 && iter != 194)
            {
                fprintf(stderr, "%s: ", g_4dd010);
                if (v1 < 0)
                    fprintf(stderr, "-%c: ", (char)(iter - 131));
                else
                    fprintf(stderr, "--%s: ", g_4186e0[4 * v1]);
                fprintf(stderr, "option not valid in GZIP environment variable\n");
            }
            sub_405097(); /* do not return */
        }
    }
    if (g_419094 < 0)
        g_419094 = g_4dd880;
    if (g_419090 < 0)
        g_419090 = g_4dd880;
    v3 = i - optind;
    if (g_4dd87c && !g_4dd004)
        fprintf(stderr, "%s: option --ascii ignored on this system\n", g_4dd010);
    if (g_4dd8b8 && g_4dd8b8 <= 30)
    {
        g_4dd940 = (!g_4dd004 ? 0 : 13);
        sub_408c07();
        if (v3)
        {
            while (i > optind)
            {
                v10 = optind;
                optind = v10 + 1;
                sub_405f11(a1[v10]);
            }
        }
        else
        {
            sub_405b3e();
        }
        if (g_4ddd60 && close(0))
        {
            strcpy(&g_4dd060, "stdin");
            sub_40f6c7(v11, v12, v13, v14, v15, v16); /* do not return */
        }
        if (g_4dd890)
        {
            if (!g_4dd004 && v3 > 1)
                sub_407fdc(0xffffffff);
            if (sub_4109d7(stdout))
                sub_40f74d(); /* do not return */
        }
        if (g_4dd000 && (g_4dd879 && fdatasync(1) && *(__errno_location()) != 22 || close(1)) && *(__errno_location()) != 9)
            sub_40f74d(); /* do not return */
        sub_408e40(g_4dd018); /* do not return */
    }
    fprintf(stderr, "%s: invalid suffix '%s'\n", g_4dd010, g_4dd8b0);
    sub_408e40(1); /* do not return */
}



// Function: input_eof @ 0x5a65
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd880;
extern unsigned int g_4dd89c;

unsigned int input_eof(void)
{
    if (g_4dd880 && !g_4dd89c)
    {
        if (g_4dd86c != *((int *)&g_4dd868))
            return 0;
        if (*((int *)&g_4dd868) != 0x40000 || sub_40f0bc(1) == 0xffffffff)
            return 1;
        g_4dd86c = 0;
        return 0;
    }
    return 1;
}



// Function: get_input_size_and_time @ 0x5acc
extern unsigned int g_419094;
extern unsigned long long g_4dd020;
extern unsigned long long g_4dd030;
extern unsigned long long g_4dd038;
extern unsigned int g_4dd890;
extern long long g_4de180;
extern unsigned int g_4de198;
extern unsigned long long g_4de1b0;

unsigned long long get_input_size_and_time(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // rdx

    g_4dd020 = 0xffffffffffffffff;
    g_4dd038 = 0xffffffffffffffff;
    v1 = g_4de198 & 0xf000;
    if ((unsigned int)v1 != 0x8000)
        return v1;
    g_4dd020 = g_4de1b0;
    if (g_419094 && !g_4dd890)
        return g_4dd890;
    g_4dd030 = sub_41108f(&g_4de180);
    g_4dd038 = v2;
    return g_4dd030;
}



// Function: treat_stdin @ 0x5b3e
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

extern char g_41378f;
extern unsigned int g_41909c;
extern void stderr;
extern unsigned int g_4dd000;
extern unsigned int g_4dd004;
extern char g_4dd008;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern long long g_4dd040;
extern unsigned long long g_4dd048;
extern char g_4dd060;
extern char g_4dd460;
extern unsigned int g_4dd860;
extern char g_4dd878;
extern unsigned int g_4dd880;
extern unsigned int g_4dd884;
extern unsigned int g_4dd890;
extern unsigned int g_4dd894;
extern unsigned int g_4dd8a0;
extern char g_4ddd60;
extern stat g_4de180;
extern unsigned long long g_4e1698;

unsigned long long treat_stdin(void)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    long long v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9
    unsigned long long v7;  // rax
    unsigned long v8;  // rcx

    if (!g_4dd884 && !g_4dd890 && (g_4dd878 || isatty(!g_4dd880)))
    {
        if (!g_4dd004)
            fprintf(*((long long *)&stderr), "%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n", g_4dd010, (!g_4dd880 ? "written to" : "read from"), (!g_4dd880 ? &g_41378f : "de"), g_4dd010);
        sub_408e40(1); /* do not return */
    }
    strcpy(&g_4dd060, "stdin");
    strcpy(&g_4dd460, "stdout");
    if (fstat(0, &g_4de180.st_mode))
    {
        sub_4051e7("standard input");
        sub_408e40(1); /* do not return */
    }
    sub_405acc(v1, v2, v3);
    sub_40f078(v1, v2, v3, v4, v5, v6);
    g_4dd000 = 1;
    g_4dd8a0 = 0;
    g_4dd860 = 0;
    g_4ddd60 = 1;
    if (g_4dd880)
    {
        g_41909c = sub_4072f1(g_4dd860);
        if (g_41909c < 0)
            sub_408e40(g_4dd018); /* do not return */
    }
    while (1)
    {
        v7 = sub_40f9dd(0, 1);
        if ((unsigned int)v7)
            return v7;
        if (sub_405a65())
            break;
        g_41909c = sub_4072f1(g_4dd860);
        if (g_41909c < 0)
            return g_41909c;
        g_4dd048 = 0;
    }
    if (g_4dd890)
    {
        return sub_407fdc(g_41909c);
    }
    else if (!g_4dd894)
    {
        return g_4dd894;
    }
    else if (*((int *)&g_4dd008))
    {
        return fprintf(*((long long *)&stderr), " OK\n");
    }
    else if (!g_4dd880)
    {
        v8 = g_4dd040 - (g_4dd048 - g_4e1698);
        sub_40f7a6(v8, g_4dd040, *((long long *)&stderr), v8);
        return fprintf(*((long long *)&stderr), "\n");
    }
    else
    {
        return g_4dd880;
    }
}



// Function: atdir_eq @ 0x5dee
extern void g_4ddd80;

unsigned int atdir_eq(void* a0, unsigned long a1)
{
    unsigned int v3;  // eax
    unsigned long v0;  // [bp-0x18]
    void* v1;  // [bp-0x10]

    v1 = a0;
    v0 = a1;
    if (!v0)
    {
        v1 = ".-";
        v0 = 1;
    }
    if (!memcmp(&g_4ddd80, v1, v0) && !(&g_4ddd80)[v0])
        v3 = 1;
    else
        v3 = 0;
    return v3 & 1;
}



// Function: atdir_set @ 0x5e60
extern int g_4190a8;
extern char g_4ddd80;

int atdir_set(void* a0, unsigned long a1)
{
    unsigned long n;  // [bp-0x18]
    void* v1;  // [bp-0x10]

    v1 = a0;
    n = a1;
    if ((char)sub_405dee(v1, n) != 1)
    {
        if (g_4190a8 >= 0)
            close(g_4190a8);
        if (!n)
        {
            v1 = ".-";
            n = 1;
        }
        memcpy(&g_4ddd80, v1, n);
        (&g_4ddd80)[n] = 0;
        g_4190a8 = sub_410863(&g_4ddd80, 0x10000, &g_4ddd80);
    }
    return g_4190a8;
}



// Function: treat_file @ 0x5f11
extern char g_41378f;
extern unsigned int g_419090;
extern void g_41909c;
extern unsigned int g_4190a4;
extern int g_4190a8;
extern long long g_4190d0;
extern void stderr;
extern unsigned int g_4dd000;
extern char g_4dd004;
extern unsigned int g_4dd008;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern unsigned int g_4dd01c;
extern long long g_4dd040;
extern long long g_4dd048;
extern char g_4dd060;
extern char g_4dd460;
extern void g_4dd860;
extern void g_4dd864;
extern char g_4dd879;
extern unsigned int g_4dd880;
extern unsigned int g_4dd884;
extern unsigned int g_4dd888;
extern unsigned int g_4dd88c;
extern unsigned int g_4dd890;
extern unsigned int g_4dd894;
extern unsigned int g_4dd8a0;
extern int g_4dd8c0;
extern long long g_4de180;
extern unsigned long long g_4de190;
extern unsigned int g_4de198;
extern unsigned long long g_4e1698;

long long treat_file(unsigned long a0)
{
    unsigned int v8;  // rdi
    unsigned int v9;  // rsi
    unsigned int v10;  // rdx
    unsigned long v14;  // rcx
    unsigned long v15;  // rcx
    unsigned long v16;  // fs
    unsigned int v0;  // [bp-0xb0]
    unsigned int v1;  // [bp-0xac]
    unsigned int v2;  // [bp-0xa8]
    unsigned int v3;  // [bp-0xa4]
    long long v4;  // [bp-0xa0]
    int v5[34];  // [bp-0x98]
    unsigned long v6;  // [bp-0x10]

    if (!strcmp(a0, "-"))
    {
        v3 = g_4dd000;
        sub_405b3e();
        g_4dd000 = v3;
    }
    else
    {
        *((unsigned int *)&g_4dd860) = sub_406d5a(a0, &g_4de180, &g_4de180);
        if (*((int *)&g_4dd860) >= 0)
        {
            if (((unsigned short)g_4de198 & 0xf000) == 0x4000)
            {
                if (g_4dd88c)
                {
                    sub_40897b(*((int *)&g_4dd860), a0, a0);
                }
                else
                {
                    close(*((int *)&g_4dd860));
                    if (!*((int *)&g_4dd004))
                        fprintf(*((long long *)&stderr), "%s: %s is a directory -- ignored\n", g_4dd010, &g_4dd060);
                    if (!g_4dd018)
                        g_4dd018 = 2;
                }
            }
            else
            {
                if (g_4dd000)
                {
LABEL_406295:
                    sub_405acc(v8, v9, v10);
                    if (g_4dd000 && !g_4dd008)
                    {
                        strcpy(&g_4dd460, "stdout");
                        goto LABEL_4062e4;
                    }
                    else
                    {
                        if (sub_406ff8())
                        {
                            close(*((int *)&g_4dd860));
                        }
                        else
                        {
LABEL_4062e4:
                            sub_40f078();
                            g_4dd8a0 = 0;
                            if (g_4dd880 && (*((unsigned int *)&g_41909c) = (unsigned int)(unsigned long long)sub_4072f1((unsigned long long)(unsigned int)*((int *)&g_4dd860)), *((int *)&g_41909c) < 0))
                            {
                                close(*((int *)&g_4dd860));
                            }
                            else
                            {
                                if (g_4dd000)
                                {
                                    *((unsigned int *)&g_4dd864) = 1;
                                    goto LABEL_4063a7;
                                }
                                else
                                {
                                    if (!sub_406814())
                                    {
                                        if (!g_4dd880 && g_4dd01c && !g_4dd894 && !*((int *)&g_4dd004))
                                            fprintf(*((long long *)&stderr), "%s: %s compressed to %s\n", g_4dd010, &g_4dd060, &g_4dd460);
LABEL_4063a7:
                                        if (!g_4dd01c)
                                            g_4dd01c = !g_419090;
                                        if (g_4dd894 && !g_4dd890)
                                            fprintf(*((long long *)&stderr), "%s:\t", &g_4dd060);
                                        while (1)
                                        {
                                            if (sub_40f9dd(*((int *)&g_4dd860), *((int *)&g_4dd864), *((int *)&g_4dd864), g_4190d0))
                                            {
                                                *((unsigned int *)&g_41909c) = 0xffffffff;
                                                break;
                                            }
                                            else
                                            {
                                                if (sub_405a65() || !(*((unsigned int *)&g_41909c) = (unsigned int)(unsigned long long)sub_4072f1((unsigned long long)(unsigned int)*((int *)&g_4dd860)), *((int *)&g_41909c) >= 0))
                                                    break;
                                                g_4dd048 = 0;
                                            }
                                        }
                                        if (close(*((int *)&g_4dd860)))
                                            sub_40f6c7(); /* do not return */
                                        if (g_4dd890)
                                        {
                                            sub_407fdc(*((int *)&g_41909c));
                                        }
                                        else
                                        {
                                            if (!g_4dd000)
                                            {
                                                sub_40870d(&g_4de180);
                                                if (g_4dd879 && (g_4190a8 >= 0 && fdatasync(g_4190a8) && *(__errno_location()) != 22 || fsync(*((int *)&g_4dd864)) && *(__errno_location()) != 22) || close(*((int *)&g_4dd864)))
                                                    sub_40f74d(); /* do not return */
                                                if (!g_4dd888)
                                                {
                                                    v4 = sub_410244(&g_4dd060);
                                                    v0 = (!(char)sub_405dee(&g_4dd060, v4 - &g_4dd060) ? 0xffffffff : g_4190a8);
                                                    sigprocmask(0, &g_4dd8c0, v5);
                                                    g_4190a4 = 0xffffffff;
                                                    v1 = (v0 < 0 ? (unsigned int)sub_40f430(&g_4dd060) : (unsigned int)unlinkat(v0, v4, 0, v4));
                                                    v2 = (!v1 ? 0 : *(__errno_location()));
                                                    sigprocmask(2, v5, NULL);
                                                    if (v2)
                                                    {
                                                        if (!*((int *)&g_4dd004))
                                                            fprintf(*((long long *)&stderr), "%s: ", g_4dd010);
                                                        if (!g_4dd018)
                                                            g_4dd018 = 2;
                                                        if (!*((int *)&g_4dd004))
                                                        {
                                                            *(__errno_location()) = v2;
                                                            perror(&g_4dd060);
                                                        }
                                                    }
                                                }
                                            }
                                            if (*((int *)&g_41909c) == 0xffffffff)
                                            {
                                                if (!g_4dd000)
                                                    sub_408ebb(0);
                                            }
                                            else
                                            {
                                                if (g_4dd894)
                                                {
                                                    if (g_4dd008)
                                                    {
                                                        fprintf(*((long long *)&stderr), " OK");
                                                    }
                                                    else
                                                    {
                                                        if (g_4dd880)
                                                        {
                                                            v14 = g_4dd048 - (g_4dd040 - g_4e1698);
                                                            sub_40f7a6(v14, g_4dd048, *((long long *)&stderr), v14);
                                                        }
                                                        else
                                                        {
                                                            v15 = g_4dd040 - (g_4dd048 - g_4e1698);
                                                            sub_40f7a6(v15, g_4dd040, *((long long *)&stderr), v15);
                                                        }
                                                    }
                                                    if (!g_4dd008)
                                                        fprintf(*((long long *)&stderr), " -- %s %s", (!g_4dd888 ? "replaced with" : "created"), &g_4dd460);
                                                    fprintf(*((long long *)&stderr), "\n");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else if (((unsigned short)g_4de198 & 0xf000) != 0x8000)
                {
                    if (!*((int *)&g_4dd004))
                        fprintf(*((long long *)&stderr), "%s: %s is not a directory or a regular file - ignored\n", g_4dd010, &g_4dd060);
                    if (!g_4dd018)
                        g_4dd018 = 2;
                    close(*((int *)&g_4dd860));
                }
                else if ((unsigned short)g_4de198 & 0x800)
                {
                    if (!*((int *)&g_4dd004))
                        fprintf(*((long long *)&stderr), "%s: %s is set-user-ID on execution - ignored\n", g_4dd010, &g_4dd060);
                    if (!g_4dd018)
                        g_4dd018 = 2;
                    close(*((int *)&g_4dd860));
                }
                else if ((unsigned short)g_4de198 & 0x400)
                {
                    if (!*((int *)&g_4dd004))
                        fprintf(*((long long *)&stderr), "%s: %s is set-group-ID on execution - ignored\n", g_4dd010, &g_4dd060);
                    if (!g_4dd018)
                        g_4dd018 = 2;
                    close(*((int *)&g_4dd860));
                }
                else
                {
                    if (g_4dd884)
                        goto LABEL_406295;
                    if ((unsigned short)g_4de198 & 0x200)
                    {
                        if (!*((int *)&g_4dd004))
                            fprintf(*((long long *)&stderr), "%s: %s has the sticky bit set - file ignored\n", g_4dd010, &g_4dd060);
                        if (!g_4dd018)
                            g_4dd018 = 2;
                        close(*((int *)&g_4dd860));
                    }
                    else
                    {
                        if (g_4de190 <= 1)
                            goto LABEL_406295;
                        if (!*((int *)&g_4dd004))
                            fprintf(*((long long *)&stderr), "%s: %s has %lu other link%s -- file ignored\n", g_4dd010, &g_4dd060, g_4de190 - 1, (g_4de190 == 2 ? &g_41378f : "s"));
                        if (!g_4dd018)
                            g_4dd018 = 2;
                        close(*((int *)&g_4dd860));
                    }
                }
            }
        }
    }
    return v6 - *((long long *)(40 + v16));
}



// Function: volatile_strcpy @ 0x67dc
long long volatile_strcpy(char *a0, char *a1)
{
    char *v4;  // rax
    unsigned long v5;  // rax
    char *v0;  // [bp-0x18]
    char *v1;  // [bp-0x10]
    char *v2;  // [bp-0x10]

    v1 = a0;
    v0 = a1;
    do
    {
        v4 = v0;
        v0 = v4 + 1;
        v2 = v1 + 1;
        v5 = *(v4);
        *(v1) = v5;
        v1 = v2;
    } while ((char)v5);
    return v5;
}



// Function: create_outfile @ 0x6814
typedef struct FILE {
} FILE;

extern unsigned int g_4190a4;
extern FILE *stderr;
extern unsigned int g_4dd004;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd460;
extern int g_4dd860;
extern int g_4dd864;
extern unsigned int g_4dd880;
extern unsigned int g_4dd888;
extern int g_4dd8c0;
extern char g_4dd960;

unsigned int create_outfile(void)
{
    unsigned int v0;  // [bp-0xbc]
    unsigned int v1;  // [bp-0xb8]
    unsigned int v2;  // [bp-0xb4]
    unsigned int v3;  // [bp-0xb0]
    unsigned int v4;  // [bp-0xac]
    unsigned long v5;  // [bp-0xa8]
    unsigned long v6;  // [bp-0xa0]
    int v7[34];  // [bp-0x98]

    v0 = 0;
    v2 = 193;
    v5 = &g_4dd460;
    v1 = 0xffffff9c;
    if (!g_4dd888)
    {
        v6 = sub_410244(&g_4dd460);
        v3 = sub_405e60(&g_4dd460, v6 - &g_4dd460);
        if (v3 >= 0)
        {
            v5 = v6;
            v1 = v3;
        }
    }
    while (1)
    {
        sub_4067dc(&g_4dd960, &g_4dd460);
        sigprocmask(0, &g_4dd8c0, v7);
        g_4dd864 = sub_410ae4(v1, v5, v2, 384);
        g_4190a4 = g_4dd864;
        v4 = *(__errno_location());
        sigprocmask(2, v7, NULL);
        if (g_4dd864 >= 0)
            break;
        switch (v4)
        {
        case 17:
            if (sub_4085c8())
            {
                close(g_4dd860);
                return 1;
            }
            break;
        case 36:
            sub_4083ea(&g_4dd460);
            v0 = 1;
            break;
        default:
            sub_4051e7(&g_4dd460);
            close(g_4dd860);
            return 1;
        }
    }
    if (v0 && g_4dd880)
    {
        if (!g_4dd004)
            fprintf(stderr, "%s: %s: warning, name truncated\n", g_4dd010, &g_4dd460);
        if (!g_4dd018)
            g_4dd018 = 2;
    }
    return 0;
}



// Function: get_suffix @ 0x6a54
extern unsigned long long g_4190e0;
extern char *g_4190e8;
extern void g_4dd8b0;
extern char g_4dd8b8;

char * get_suffix(char *a0)
{
    char v0;  // [bp-0x65]
    unsigned int v1;  // [bp-0x64]
    unsigned int v2;  // [bp-0x60]
    unsigned int v3;  // [bp-0x5c]
    char **iter;  // [bp-0x58], Other Possible Types: unsigned long long *
    char *v5;  // [bp-0x50]
    unsigned long len;  // [bp-0x48]
    unsigned long ptr;  // [bp-0x40]
    char v8[40];  // [bp-0x38]

    v0 = 0;
    for (iter = &g_4190e8; *(iter); iter += 1)
    {
        len = strlen(*(iter));
        if (len > *((long long *)&g_4dd8b8) && !strcmp(*((long long *)&g_4dd8b0), &(*(iter))[len + -1 * *((long long *)&g_4dd8b8)]))
        {
            v0 = 1;
            break;
        }
    }
    ptr = sub_412377(*((long long *)&g_4dd8b0));
    sub_40f393(ptr);
    (&g_4190e0)[(!v0 ? 0 : 8)] = ptr;
    iter = &(&g_4190e0)[v0];
    v1 = strlen(a0);
    if (v1 <= 32)
        strcpy(v8, a0);
    else
        strcpy(v8, v1 + a0 - 32);
    sub_40f393(v8);
    v2 = strlen(v8);
    v5 = NULL;
    while (1)
    {
        v3 = strlen(*(iter));
        if (v2 > v3 && *(&v8[v2 + -1 * v3] - 1) != 47 && !strcmp(&v8[v2 + -1 * v3], *(iter)))
        {
            v5 = &a0[v1 + -1 * v3];
            free(ptr);
            return v5;
        }
        iter += 1;
        if (!*(iter))
        {
            free(ptr);
            return v5;
        }
    }
}



// Function: open_and_stat @ 0x6c6d
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

extern unsigned int g_4dd000;
extern unsigned int g_4dd884;
extern unsigned int g_4dd888;

unsigned int open_and_stat(unsigned long a0, unsigned int a1, stat *a2)
{
    unsigned int v0;  // [bp-0x34]
    unsigned int v1;  // [bp-0x28]
    unsigned int v2;  // [bp-0x24]
    unsigned int v3;  // [bp-0x20]
    unsigned int v4;  // [bp-0x1c]
    unsigned long v5;  // [bp-0x18]
    unsigned long v6;  // [bp-0x10]

    v0 = a1;
    v1 = 0xffffff9c;
    v5 = a0;
    if (!g_4dd000 && !g_4dd884)
        v0 |= 0x20000;
    if (!g_4dd888)
    {
        v6 = sub_410244(a0);
        v2 = sub_405e60(a0, v6 - a0);
        if (v2 >= 0)
        {
            v5 = v6;
            v1 = v2;
        }
    }
    v3 = sub_410ae4(v1, v5, v0, v5);
    if (v3 >= 0 && fstat(v3, a2))
    {
        v4 = *(__errno_location());
        close(v3);
        *(__errno_location()) = v4;
        return 0xffffffff;
    }
    return v3;
}



// Function: open_input_file @ 0x6d5a
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

extern unsigned long long g_419140;
extern FILE *stderr;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd060;
extern unsigned int g_4dd880;
extern unsigned long long g_4dd8b0;

unsigned int open_input_file(char *ptr, stat *a1)
{
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]
    unsigned int v2;  // [bp-0x38]
    unsigned int v3;  // [bp-0x34]
    unsigned long long *iter;  // [bp-0x30]
    char *v5;  // [bp-0x28], Other Possible Types: unsigned long
    unsigned long v6;  // [bp-0x20]

    v0 = 0;
    iter = &g_419140;
    v1 = 0x900;
    *(iter) = g_4dd8b0;
    if (strlen(ptr) <= 0x3fe)
    {
        strcpy(&g_4dd060, ptr);
        v2 = sub_406c6d(&g_4dd060, v1, a1);
        if (v2 >= 0)
        {
            return v2;
        }
        else if (!g_4dd880 || *(__errno_location()) != 2)
        {
            sub_4051e7(&g_4dd060);
            return 0xffffffff;
        }
        else
        {
            v5 = sub_406a54(&g_4dd060);
            if (v5)
            {
                sub_4051e7(&g_4dd060);
                return 0xffffffff;
            }
            v3 = strlen(&g_4dd060);
            if (!strcmp(g_4dd8b0, ".gz"))
                iter += 1;
            while (1)
            {
                v5 = *(iter);
                v6 = v5;
                strcpy(&g_4dd060, ptr);
                if (strlen(v5) + v3 > 0x3ff)
                    break;
                strcat(&g_4dd060, v5);
                v2 = sub_406c6d(&g_4dd060, v1, a1);
                if (v2 >= 0)
                {
                    return v2;
                }
                else if (*(__errno_location()) != 2)
                {
                    sub_4051e7(&g_4dd060);
                    return 0xffffffff;
                }
                else
                {
                    if (!strcmp(v6, g_4dd8b0))
                        v0 = *(__errno_location());
                    iter += 1;
                    if (!*(iter))
                    {
                        strcpy(&g_4dd060, ptr);
                        strcat(&g_4dd060, g_4dd8b0);
                        *(__errno_location()) = v0;
                        sub_4051e7(&g_4dd060);
                        return 0xffffffff;
                    }
                }
            }
        }
    }
    fprintf(stderr, "%s: %s: file name too long\n", g_4dd010, ptr);
    g_4dd018 = 1;
    return 0xffffffff;
}



// Function: make_ofname @ 0x6ff8
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned int g_4dd004;
extern unsigned int g_4dd008;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern unsigned int g_4dd01c;
extern char g_4dd060;
extern char g_4dd460;
extern unsigned int g_4dd880;
extern unsigned int g_4dd884;
extern unsigned int g_4dd88c;
extern unsigned int g_4dd894;
extern char *g_4dd8b0;
extern unsigned long long g_4dd8b8;

unsigned int make_ofname(void)
{
    unsigned long len;  // rax
    char *ptr;  // [bp-0x10]

    strcpy(&g_4dd460, &g_4dd060);
    ptr = sub_406a54(&g_4dd460);
    if (g_4dd880)
    {
        if (!ptr)
        {
            if (!g_4dd88c && g_4dd008)
                return 0;
            if (!g_4dd894 && (g_4dd88c || g_4dd004))
                return 2;
            if (!g_4dd004)
                fprintf(stderr, "%s: %s: unknown suffix -- ignored\n", g_4dd010, &g_4dd060);
            if (!g_4dd018)
                g_4dd018 = 2;
            return 2;
        }
        sub_40f393(ptr);
        if (!strcmp(ptr, ".tgz") || !strcmp(ptr, ".taz"))
            strcpy(ptr, ".tar");
        else
            *(ptr) = 0;
    }
    else
    {
        if (ptr && !g_4dd884)
        {
            if (g_4dd894 || !g_4dd88c && !g_4dd004)
                fprintf(stderr, "%s: %s already has %s suffix -- unchanged\n", g_4dd010, &g_4dd060, ptr);
            return 2;
        }
        g_4dd01c = 0;
        len = strlen(&g_4dd460);
        if (len + g_4dd8b8 <= 0x3ff)
        {
            strcat(&g_4dd460, g_4dd8b0);
        }
        else
        {
            if (!g_4dd004)
                fprintf(stderr, "%s: %s: file name too long\n", g_4dd010, &g_4dd060);
            if (g_4dd018)
                return 2;
            g_4dd018 = 2;
            return 2;
        }
    }
    return 0;
}



// Function: discard_input_bytes @ 0x7240
extern char g_41b000;
extern char g_4dd868;
extern unsigned int g_4dd86c;

void discard_input_bytes(unsigned long long a0, unsigned int a1)
{
    unsigned int v3;  // eax
    char v4;  // al
    unsigned long long i;  // [bp-0x20]
    char v1;  // [bp-0x11]

    for (i = a0; i; i -= 1)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v3 = g_4dd86c;
            g_4dd86c = v3 + 1;
            v4 = *(&(&g_41b000)[v3]);
        }
        else
        {
            v4 = sub_40f0bc(0);
        }
        v1 = v4;
        if ((char)a1 & 2)
            sub_40efda(&v1, 1);
        if (i == 0xffffffffffffffff)
        {
            if (!v1)
                return;
        }
    }
    return;
}



// Function: get_method @ 0x72f1
typedef struct FILE {
} FILE;

extern void g_413a23;
extern void g_413a26;
extern void g_413b7c;
extern void g_413b81;
extern void g_413b84;
extern void g_413b87;
extern unsigned int g_419090;
extern unsigned int g_419094;
extern unsigned int g_41909c;
extern unsigned long long g_4190d0;
extern FILE *stderr;
extern void g_41b000;
extern unsigned int g_4dd000;
extern unsigned int g_4dd004;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern unsigned long long g_4dd030;
extern unsigned long long g_4dd038;
extern char g_4dd060;
extern long long g_4dd460;
extern char g_4dd860;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern int g_4dd884;
extern unsigned int g_4dd890;
extern unsigned int g_4dd894;
extern unsigned int g_4dd89c;
extern unsigned int g_4dd8a0;
extern unsigned long long g_4e1698;

int get_method(unsigned int a0)
{
    unsigned int v23;  // eax
    unsigned int v24;  // eax
    unsigned int v33;  // eax
    unsigned int v34;  // eax
    unsigned int v35;  // eax
    char v36;  // al
    unsigned int v37;  // eax
    unsigned long v38;  // rax
    unsigned int v39;  // eax
    unsigned long long v40;  // rax
    unsigned int v41;  // eax
    unsigned long long v42;  // rax
    unsigned int v25;  // eax
    unsigned int v43;  // eax
    unsigned long long v44;  // rax
    unsigned int v45;  // eax
    char v46;  // al
    unsigned int v47;  // eax
    char v48;  // al
    unsigned int v49;  // eax
    char v50;  // al
    unsigned int v51;  // eax
    char v52;  // al
    unsigned int v26;  // eax
    unsigned int v53;  // eax
    char v54;  // dl
    unsigned int v56;  // eax
    unsigned int v57;  // eax
    unsigned int v58;  // eax
    unsigned int v59;  // eax
    unsigned int v60;  // eax
    unsigned int v61;  // eax
    unsigned int v27;  // eax
    char v28;  // al
    unsigned int v29;  // eax
    char v30;  // al
    unsigned int v31;  // eax
    unsigned int v32;  // eax
    char v0;  // [bp-0x51]
    unsigned int v1;  // [bp-0x50]
    unsigned int v2;  // [bp-0x4c]
    unsigned int i;  // [bp-0x48]
    unsigned int v4;  // [bp-0x44]
    unsigned int v5;  // [bp-0x40]
    unsigned int v6;  // [bp-0x3c]
    void* iter;  // [bp-0x38], Other Possible Types: char *
    unsigned long v8;  // [bp-0x30], Other Possible Types: unsigned long long
    char *ptr;  // [bp-0x28]
    char v10;  // [bp-0x1c]
    char v11;  // [bp-0x1b]
    char v12;  // [bp-0x1a]
    char v13;  // [bp-0x19]
    char v14;  // [bp-0x18]
    char v15;  // [bp-0x17]
    char v16;  // [bp-0x16]
    char v17;  // [bp-0x15]
    char v18;  // [bp-0x14]
    char v19;  // [bp-0x13]
    char v20;  // [bp-0x12]
    char v21;  // [bp-0x11]

    if (g_4dd884 && g_4dd000)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v23 = g_4dd86c;
            g_4dd86c = v23 + 1;
            v24 = *(&(&g_41b000)[v23]);
        }
        else
        {
            v24 = sub_40f0bc(1);
        }
        v1 = v24;
        v12 = v1;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v25 = g_4dd86c;
            g_4dd86c = v25 + 1;
            v26 = *(&(&g_41b000)[v25]);
        }
        else
        {
            v26 = sub_40f0bc(1);
        }
        v2 = v26;
        v13 = v2;
    }
    else
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v27 = g_4dd86c;
            g_4dd86c = v27 + 1;
            v28 = *(&(&g_41b000)[v27]);
        }
        else
        {
            v28 = sub_40f0bc(0);
        }
        v12 = v28;
        v1 = 0;
        if (v12)
        {
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v29 = g_4dd86c;
                g_4dd86c = v29 + 1;
                v30 = *(&(&g_41b000)[v29]);
            }
            else
            {
                v30 = sub_40f0bc(0);
            }
            v13 = v30;
            v2 = 0;
        }
        else
        {
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v31 = g_4dd86c;
                g_4dd86c = v31 + 1;
                v32 = *(&(&g_41b000)[v31]);
            }
            else
            {
                v32 = sub_40f0bc(1);
            }
            v2 = v32;
            v13 = v2;
        }
    }
    g_41909c = 0xffffffff;
    g_4dd8a0 = g_4dd8a0 + 1;
    g_4e1698 = 0;
    g_4dd89c = 0;
    if (!memcmp(&v12, &g_413a23, 2) || !memcmp(&v12, &g_413a26, 2))
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v33 = g_4dd86c;
            g_4dd86c = v33 + 1;
            v34 = *(&(&g_41b000)[v33]);
        }
        else
        {
            v34 = sub_40f0bc(0);
        }
        g_41909c = v34;
        if (g_41909c != 8)
        {
            fprintf(stderr, "%s: %s: unknown method %d -- not supported\n", g_4dd010, &g_4dd060, g_41909c);
            g_4dd018 = 1;
            return -0x1;
        }
        g_4190d0 = sub_40e86a;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v35 = g_4dd86c;
            g_4dd86c = v35 + 1;
            v36 = *(&(&g_41b000)[v35]);
        }
        else
        {
            v36 = sub_40f0bc(0);
        }
        v0 = v36;
        if (v0 & 32)
        {
            fprintf(stderr, "%s: %s is encrypted -- not supported\n", g_4dd010, &g_4dd060);
            g_4dd018 = 1;
            return -0x1;
        }
        if (v0 & 192)
        {
            fprintf(stderr, "%s: %s has flags 0x%x -- not supported\n", g_4dd010, &g_4dd060, v0);
            g_4dd018 = 1;
            if (g_4dd884 <= 1)
                return -0x1;
        }
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v37 = g_4dd86c;
            g_4dd86c = v37 + 1;
            v38 = *(&(&g_41b000)[v37]);
        }
        else
        {
            v38 = sub_40f0bc(0);
        }
        v8 = v38;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v39 = g_4dd86c;
            g_4dd86c = v39 + 1;
            v40 = *(&(&g_41b000)[v39]) * 0x100;
        }
        else
        {
            v40 = sub_40f0bc(0) * 0x100;
        }
        v8 |= v40;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v41 = g_4dd86c;
            g_4dd86c = v41 + 1;
            v42 = *(&(&g_41b000)[v41]) * 0x10000;
        }
        else
        {
            v42 = sub_40f0bc(0) * 0x10000;
        }
        v8 |= v42;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v43 = g_4dd86c;
            g_4dd86c = v43 + 1;
            v44 = *(&(&g_41b000)[v43]) * 0x1000000;
        }
        else
        {
            v44 = sub_40f0bc(0) * 0x1000000;
        }
        v8 |= v44;
        if (v8 && !g_419094)
        {
            if (v8 >= NULL)
            {
                g_4dd030 = v8;
                g_4dd038 = 0;
            }
            else
            {
                if (!g_4dd004)
                    fprintf(stderr, "%s: %s: MTIME %lu out of range for this platform\n", g_4dd010, &g_4dd060, v8);
                if (!g_4dd018)
                    g_4dd018 = 2;
                g_4dd030 = 0x7fffffffffffffff;
                g_4dd038 = 0x3b9ac9ff;
            }
        }
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v45 = g_4dd86c;
            g_4dd86c = v45 + 1;
            v46 = *(&(&g_41b000)[v45]);
        }
        else
        {
            v46 = sub_40f0bc(0);
        }
        v20 = v46;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v47 = g_4dd86c;
            g_4dd86c = v47 + 1;
            v48 = *(&(&g_41b000)[v47]);
        }
        else
        {
            v48 = sub_40f0bc(0);
        }
        v21 = v48;
        if (v0 & 2)
        {
            v14 = 8;
            v15 = v0;
            v16 = v8;
            v17 = v8 >> 8;
            v18 = v8 >> 16;
            v19 = v8 >> 24;
            sub_40efda(0, 0);
            sub_40efda(&v12, 10);
        }
        if (v0 & 4)
        {
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v49 = g_4dd86c;
                g_4dd86c = v49 + 1;
                v50 = *(&(&g_41b000)[v49]);
            }
            else
            {
                v50 = sub_40f0bc(0);
            }
            v10 = v50;
            v4 = v10;
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v51 = g_4dd86c;
                g_4dd86c = v51 + 1;
                v52 = *(&(&g_41b000)[v51]);
            }
            else
            {
                v52 = sub_40f0bc(0);
            }
            v11 = v52;
            v4 |= v11 * 0x100;
            if (g_4dd894)
                fprintf(stderr, "%s: %s: extra field of %u bytes ignored\n", g_4dd010, &g_4dd060, v4);
            if (v0 & 2)
                sub_40efda(&v10, 2);
            sub_407240(v4, v0);
        }
        if (v0 & 8)
        {
            if (g_419090 || g_4dd000 && !g_4dd890 || g_4dd8a0 > 1)
            {
                sub_407240(0xffffffffffffffff, v0);
            }
            else
            {
                iter = sub_40f40a(&g_4dd460);
                ptr = iter;
                while (1)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v53 = g_4dd86c;
                        g_4dd86c = v53 + 1;
                        v54 = *(&(&g_41b000)[v53]);
                    }
                    else
                    {
                        v54 = sub_40f0bc(0);
                    }
                    *(iter) = v54;
                    iter += 1;
                    if (!*(iter))
                        break;
                    iter = iter;
                    if (iter >= &g_4dd860)
                        sub_40f5ef("corrupted input -- file name too large"); /* do not return */
                }
                if (v0 & 2)
                    sub_40efda(ptr, iter - ptr & 0xffffffff, iter - ptr & 0xffffffff);
                iter = sub_40f40a(ptr);
                memmove(ptr, iter, strlen(iter) + 1);
                if (!g_4dd890 && ptr)
                    g_4dd890 = 0;
            }
        }
        if (v0 & 16)
            sub_407240(0xffffffffffffffff, v0);
        if (v0 & 2)
        {
            v5 = sub_40efda(&v12, 0) & 0xffff;
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v56 = g_4dd86c;
                g_4dd86c = v56 + 1;
                v57 = *(&(&g_41b000)[v56]);
            }
            else
            {
                v57 = sub_40f0bc(0);
            }
            v6 = v57;
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v58 = g_4dd86c;
                g_4dd86c = v58 + 1;
                v59 = *(&(&g_41b000)[v58]) * 0x100;
            }
            else
            {
                v59 = sub_40f0bc(0) * 0x100;
            }
            v6 |= v59;
            if (v6 != v5)
            {
                fprintf(stderr, "%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n", g_4dd010, &g_4dd060, v6, v5);
                g_4dd018 = 1;
                if (g_4dd884 <= 1)
                    return -0x1;
            }
        }
        if (g_4dd8a0 == 1)
            g_4e1698 = g_4dd86c + 8;
    }
    else if (!memcmp(&v12, &g_413b7c, 2) && g_4dd86c == 2 && !memcmp(&g_41b000, &g_413b7c, 4))
    {
        g_4dd86c = 0;
        g_4190d0 = sub_40e86a;
        if (sub_40e655(a0))
            return -0x1;
        g_4dd89c = 1;
    }
    else if (!memcmp(&v12, &g_413b81, 2))
    {
        g_4190d0 = sub_40e3fd;
        g_41909c = 2;
    }
    else if (!memcmp(&v12, &g_413b84, 2))
    {
        g_4190d0 = sub_40d846;
        g_41909c = 1;
        g_4dd89c = 1;
    }
    else if (!memcmp(&v12, &g_413b87, 2))
    {
        g_4190d0 = sub_40d7d6;
        g_41909c = 3;
        g_4dd89c = 1;
    }
    else if (g_4dd884 && g_4dd000 && !g_4dd890)
    {
        g_41909c = 0;
        g_4190d0 = sub_40ef1d;
        if (v2 != 0xffffffff)
            g_4dd86c = g_4dd86c - 1;
        g_4dd89c = 1;
        if (v1 != 0xffffffff)
            sub_40f321(1, &v12, 1);
    }
    if (g_41909c >= 0)
    {
        return g_41909c;
    }
    else if (g_4dd8a0 == 1)
    {
        fprintf(stderr, "\n%s: %s: not in gzip format\n", g_4dd010, &g_4dd060);
        g_4dd018 = 1;
        return -0x1;
    }
    else
    {
        if (!v12)
        {
            for (i = v2; !i; i = v61)
            {
                if (g_4dd86c < *((int *)&g_4dd868))
                {
                    v60 = g_4dd86c;
                    g_4dd86c = v60 + 1;
                    v61 = *(&(&g_41b000)[v60]);
                }
                else
                {
                    v61 = sub_40f0bc(1);
                }
            }
            if (i == 0xffffffff)
            {
                if (!g_4dd894)
                    return -0x3;
                if (!g_4dd004)
                    fprintf(stderr, "\n%s: %s: decompression OK, trailing zero bytes ignored\n", g_4dd010, &g_4dd060);
                if (!g_4dd018)
                    g_4dd018 = 2;
                return -0x3;
            }
        }
        if (!g_4dd004)
            fprintf(stderr, "\n%s: %s: decompression OK, trailing garbage ignored\n", g_4dd010, &g_4dd060);
        if (g_4dd018)
            return -0x2;
        g_4dd018 = 2;
        return -0x2;
    }
}



// Function: do_list @ 0x7fdc
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

extern unsigned long long g_418b20[4];
extern unsigned int g_4190c8;
extern long long stdout;
extern unsigned int g_4dd004;
extern int g_4dd030;
extern long long g_4dd040;
extern unsigned long long g_4dd048;
extern char g_4dd460;
extern unsigned int g_4dd894;
extern unsigned int g_4dd89c;
extern unsigned long long g_4ddd68;
extern unsigned long long g_4ddd70;
extern unsigned long long g_4e1680;
extern unsigned long long g_4e1698;

long long do_list(int a0)
{
    long long v5;  // rax
    unsigned long v6;  // rcx
    unsigned long v7;  // rcx
    unsigned long long v0;  // [bp-0x48]
    unsigned int v1;  // [bp-0x1c]
    unsigned long long v2;  // [bp-0x18]
    tm *tm;  // [bp-0x10]

    v1 = 19;
    if (g_4190c8 && a0 >= 0)
    {
        g_4190c8 = 0;
        if (g_4dd894)
            printf("method  crc     date  time  ");
        if (!g_4dd004)
        {
            v0 = "uncompressed";
            printf("%*.*s %*.*s  ratio uncompressed_name\n");
        }
    }
    else if (a0 < 0)
    {
        v5 = g_4ddd68;
        if (g_4ddd68 > 0)
        {
            v5 = g_4ddd70;
            if (g_4ddd70 > 0)
            {
                if (g_4dd894)
                    printf("                            ");
                if (g_4dd894 || !g_4dd004)
                {
                    sub_40f81a(stdout, g_4ddd68, v1, g_4ddd68);
                    printf(" ");
                    sub_40f81a(stdout, g_4ddd70, v1, g_4ddd70);
                    printf(" ");
                }
                v6 = g_4ddd70 - (g_4ddd68 - g_4e1698);
                sub_40f7a6(v6, g_4ddd70, stdout, v6);
                return printf(" (totals)\n");
            }
        }
        return v5;
    }
    v2 = 0xffffffffffffffff;
    if (a0 == 8 && !g_4dd89c)
        v2 = g_4e1680;
    if (g_4dd894)
    {
        tm = localtime(&g_4dd030);
        printf("%5s %08lx ", g_418b20[a0], v2);
        if (tm)
            printf("%s%3d %02d:%02d ", (char *)(4 * tm->tm_mon + 4276960), tm->tm_mday, tm->tm_hour, tm->tm_min);
        else
            printf("??? ?? ??:?? ");
    }
    sub_40f81a(stdout, g_4dd040, v1, g_4dd040);
    printf(" ");
    sub_40f81a(stdout, g_4dd048, v1, g_4dd048);
    printf(" ");
    if (g_4dd040 == 0xffffffffffffffff)
    {
        g_4ddd68 = 0xffffffffffffffff;
        g_4e1698 = 0;
        g_4dd048 = g_4e1698;
        g_4dd040 = g_4dd048;
    }
    else if (g_4ddd68 >= 0)
    {
        g_4ddd68 = g_4dd040 + g_4ddd68;
    }
    if (g_4dd048 == 0xffffffffffffffff)
    {
        g_4ddd70 = 0xffffffffffffffff;
        g_4e1698 = 0;
        g_4dd048 = g_4e1698;
        g_4dd040 = g_4dd048;
    }
    else if (g_4ddd70 >= 0)
    {
        g_4ddd70 = g_4dd048 + g_4ddd70;
    }
    v7 = g_4dd048 - (g_4dd040 - g_4e1698);
    sub_40f7a6(v7, g_4dd048, stdout, v7);
    return printf(" %s\n", &g_4dd460);
}



// Function: shorten_name @ 0x83ea
extern unsigned int g_4dd01c;
extern unsigned int g_4dd880;
extern char *g_4dd8b0;

char * shorten_name(unsigned long a0)
{
    int v0;  // [bp-0x24]
    unsigned int v1;  // [bp-0x20]
    int v2;  // [bp-0x1c]
    char *ptr;  // [bp-0x18]
    char *node;  // [bp-0x10]

    ptr = NULL;
    v0 = 3;
    v1 = strlen(a0);
    if (!g_4dd880)
    {
        node = sub_406a54(a0);
        if (!node)
            sub_40f5ef("can't recover suffix\n"); /* do not return */
        *(node) = 0;
        g_4dd01c = 1;
        if (v1 > 4 && !strcmp(node - 4, ".tar"))
            return strcpy(node - 4, ".tgz");
        do
        {
            for (node = sub_410244(a0); *(node); node += 1)
            {
                v2 = strcspn(node, ".");
                node = &node[v2];
                if (v2 > v0)
                    ptr = node - 1;
                if (!*(node))
                    continue;
            }
        } while (!ptr && (v0 = (int)(v0 - 1), v0));
        if (ptr)
        {
            do
            {
                *(ptr) = ptr[1];
                ptr += 1;
            } while (*(ptr));
            ptr -= 1;
        }
        else
        {
            ptr = strrchr(a0, 46);
            if (!ptr)
                sub_40f5ef("internal error in shorten_name"); /* do not return */
            if (!ptr[1])
                ptr -= 1;
        }
        return strcpy(ptr, g_4dd8b0);
    }
    else if (v1 <= 1)
    {
        sub_40f5ef("name too short"); /* do not return */
    }
    else
    {
        *((char *)(a0 + v1 - 1)) = 0;
        return a0 + v1 - 1;
    }
}



// Function: check_ofname @ 0x85c8
extern void stderr;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd460;
extern char g_4dd878;
extern unsigned int g_4dd884;
extern unsigned int g_4dd898;

unsigned int check_ofname(void)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned int v0;  // [bp-0xc]

    if (!g_4dd884)
    {
        v0 = 0;
        fprintf(*((long long *)&stderr), "%s: %s already exists;", g_4dd010, &g_4dd460);
        if (g_4dd898 && (g_4dd878 || isatty(0)))
        {
            fprintf(*((long long *)&stderr), " do you wish to overwrite (y or n)? ");
            sub_4109d7(*((long long *)&stderr));
            v0 = (char)sub_4123a8(v2, v3, v4, v5, v6, v7);
        }
        if (!v0)
        {
            fprintf(*((long long *)&stderr), "\tnot overwritten\n");
            if (g_4dd018)
                return 1;
            g_4dd018 = 2;
            return 1;
        }
    }
    if (!sub_40f430(&g_4dd460))
        return 0;
    sub_4051e7(&g_4dd460);
    return 1;
}



// Function: do_chown @ 0x86dc
int do_chown(int a0, unsigned long long a1, unsigned int a2, unsigned int a3)
{
    unsigned long long v0;  // [bp-0x28]

    v0 = a1;
    return fchown(a0, a2, a3);
}



// Function: copy_stat @ 0x870d
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned int g_4dd004;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd030;
extern char g_4dd038;
extern char g_4dd460;
extern void g_4dd864;
extern unsigned int g_4dd880;
extern int g_4dd894;

void copy_stat(struct_0 *a0, unsigned long a1, unsigned long a2)
{
    long long v10;  // rdx
    char v11;  // al
    char v0;  // [bp-0x49]
    unsigned int v1;  // [bp-0x48]
    unsigned int v2;  // [bp-0x44]
    unsigned int v3;  // [bp-0x40]
    unsigned int v4;  // [bp-0x3c]
    unsigned long v5;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    unsigned long long v7;  // [bp-0x28]
    long long v8;  // [bp-0x20]

    v1 = a0->field_18 & 0x1ff;
    v5 = sub_41105b(a0);
    v6 = v10;
    v7 = sub_41108f(a0);
    v8 = v10;
    if (g_4dd880 && *((long long *)&g_4dd038) >= 0 && (v7 != *((long long *)&g_4dd030) || v8 != *((long long *)&g_4dd038)))
        v11 = 1;
    else
        v11 = 0;
    v0 = v11;
    v0 &= 1;
    if (v0)
    {
        v7 = *((long long *)&g_4dd030);
        v8 = *((long long *)&g_4dd038);
    }
    if (sub_4113ca(*((int *)&g_4dd864), &g_4dd460, &v5, &g_4dd460))
    {
        v2 = *(__errno_location());
        if (!g_4dd004)
            fprintf(stderr, "%s: ", g_4dd010);
        if (!g_4dd018)
            g_4dd018 = 2;
        if (!g_4dd004)
        {
            *(__errno_location()) = v2;
            perror(&g_4dd460);
        }
    }
    else if (v0 && g_4dd894 > 1)
    {
        fprintf(stderr, "%s: timestamp restored\n", &g_4dd460);
    }
    sub_4086dc(*((int *)&g_4dd864), &g_4dd460, 0xffffffff, a0->field_20);
    v3 = fchmod(*((int *)&g_4dd864), v1);
    if (!v3)
    {
        sub_4086dc(*((int *)&g_4dd864), &g_4dd460, a0->field_1c, 0xffffffff);
        return;
    }
    v4 = *(__errno_location());
    if (!g_4dd004)
        fprintf(stderr, "%s: ", g_4dd010);
    if (!g_4dd018)
        g_4dd018 = 2;
    if (g_4dd004)
    {
        sub_4086dc(*((int *)&g_4dd864), &g_4dd460, a0->field_1c, 0xffffffff);
        return;
    }
    *(__errno_location()) = v4;
    perror(&g_4dd460);
    sub_4086dc(*((int *)&g_4dd864), &g_4dd460, a0->field_1c, 0xffffffff);
    return;
}



// Function: treat_dir @ 0x897b
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;
extern unsigned int g_4dd018;

void treat_dir(int a0, char *ptr)
{
    char *iter;  // [bp-0x440]
    unsigned long len;  // [bp-0x438]
    unsigned long v2;  // [bp-0x438]
    unsigned long fp;  // [bp-0x430]
    char *v4;  // [bp-0x428]
    unsigned long len1;  // [bp-0x420]
    char v6[1032];  // [bp-0x418]

    fp = fdopendir(a0);
    if (!fp)
    {
        sub_4051e7(ptr);
        close(a0);
        return;
    }
    v4 = sub_410c61(fp, 0);
    if (!v4)
        sub_4051e7(ptr);
    if (closedir(fp))
        sub_4051e7(ptr);
    if (!v4)
        return;
    for (iter = v4; *(iter); iter = &iter[1 + len1])
    {
        len = strlen(ptr);
        len1 = strlen(iter);
        if (strcmp(iter, ".") && strcmp(iter, ".."))
        {
            if (len1 + len <= 0x3fd)
            {
                strcpy(v6, ptr);
                if (*((char *)sub_410244(v6)) && *((char *)&len1 + len - 7) != 47)
                {
                    v2 = len + 1;
                    v6[len] = 47;
                    len = v2;
                }
                strcpy(&v6[len], iter);
                sub_405f11(v6);
            }
            else
            {
                fprintf(stderr, "%s: %s/%s: pathname too long\n", g_4dd010, ptr, iter);
                g_4dd018 = 1;
            }
        }
    }
    free(v4);
    return;
}



// Function: install_signal_handlers @ 0x8c07
typedef struct sigaction {
} sigaction;

extern unsigned int g_4190b0[4];
extern unsigned int g_4dd898;
extern unsigned long long g_4dd8c0;
extern unsigned long long g_4dd8c8;
extern unsigned long long g_4dd8d0;
extern unsigned long long g_4dd8d8;
extern unsigned long long g_4dd8e0;
extern unsigned long long g_4dd8e8;
extern unsigned long long g_4dd8f0;
extern unsigned long long g_4dd8f8;
extern unsigned long long g_4dd900;
extern unsigned long long g_4dd908;
extern unsigned long long g_4dd910;
extern unsigned long long g_4dd918;
extern unsigned long long g_4dd920;
extern unsigned long long g_4dd928;
extern unsigned long long g_4dd930;
extern unsigned long long g_4dd938;

void install_signal_handlers(void)
{
    int iter;  // [bp-0xb0]
    int v1;  // [bp-0xac]
    sigaction v2;  // [bp-0xa8]
    unsigned long v3;  // [bp-0xa0]
    unsigned long v4;  // [bp-0x98]
    unsigned long v5;  // [bp-0x90]
    unsigned long v6;  // [bp-0x88]
    unsigned long v7;  // [bp-0x80]
    unsigned long v8;  // [bp-0x78]
    unsigned long v9;  // [bp-0x70]
    unsigned long v10;  // [bp-0x68]
    unsigned long v11;  // [bp-0x60]
    unsigned long v12;  // [bp-0x58]
    unsigned long v13;  // [bp-0x50]
    unsigned long v14;  // [bp-0x48]
    unsigned long v15;  // [bp-0x40]
    unsigned long v16;  // [bp-0x38]
    unsigned long v17;  // [bp-0x30]
    unsigned long v18;  // [bp-0x28]
    unsigned int v19;  // [bp-0x20]

    v1 = 6;
    sigemptyset(&g_4dd8c0);
    for (iter = 0; iter < v1; iter += 1)
    {
        sigaction(g_4190b0[iter], NULL, &v2);
        if (v2 != 1)
            sigaddset(&g_4dd8c0, g_4190b0[iter]);
    }
    v2 = (sigaction)sub_408fbb;
    v3 = g_4dd8c0;
    v4 = g_4dd8c8;
    v5 = g_4dd8d0;
    v6 = g_4dd8d8;
    v7 = g_4dd8e0;
    v8 = g_4dd8e8;
    v9 = g_4dd8f0;
    v10 = g_4dd8f8;
    v11 = g_4dd900;
    v12 = g_4dd908;
    v13 = g_4dd910;
    v14 = g_4dd918;
    v15 = g_4dd920;
    v16 = g_4dd928;
    v17 = g_4dd930;
    v18 = g_4dd938;
    v19 = 0;
    for (iter = 0; iter < v1; iter += 1)
    {
        if (sigismember(&g_4dd8c0, g_4190b0[iter]))
        {
            if (!iter)
                g_4dd898 = 1;
            sigaction(g_4190b0[iter], &v2, NULL);
        }
    }
    return;
}



// Function: do_exit @ 0x8e40
extern void* g_4dd8a8;
extern unsigned int g_4de210;

void do_exit(int a0)
{
    if (!g_4de210)
    {
        g_4de210 = 1;
        free(g_4dd8a8);
        g_4dd8a8 = 0;
        exit(a0); /* do not return */
    }
    exit(a0); /* do not return */
}



// Function: finish_out @ 0x8e91
extern long long stdout;

long long finish_out(void)
{
    if (!sub_410304(stdout))
        sub_408e40(0); /* do not return */
    sub_40f74d(); /* do not return */
}



// Function: remove_output_file @ 0x8ebb
extern int g_4190a4;
extern int g_4dd8c0;
extern char g_4dd960;

void remove_output_file(char a0)
{
    int v0;  // [bp-0x49c]
    int v1[32];  // [bp-0x498]
    char v2[1032];  // [bp-0x418]

    if (a0 != 1)
        sigprocmask(0, &g_4dd8c0, v1);
    v0 = g_4190a4;
    if (v0 >= 0)
    {
        g_4190a4 = 0xffffffff;
        close(v0);
        sub_4067dc(v2, &g_4dd960);
        sub_40f430(v2);
    }
    if (a0 != 1)
        sigprocmask(2, v1, NULL);
    return;
}



// Function: abort_gzip @ 0x8f9f
void abort_gzip(void)
{
    sub_408ebb(0);
    sub_408e40(1); /* do not return */
}



// Function: abort_gzip_signal @ 0x8fbb
extern char g_4dd940;

int abort_gzip_signal(int a0)
{
    sub_408ebb(1);
    if (a0 != *((int *)&g_4dd940))
    {
        signal(a0, NULL);
        return raise(a0);
    }
    _exit(2); /* do not return */
}



// Function: huft_build @ 0x9005
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    char field_10;
    char field_11;
    char padding_12[14];
    char field_20;
    char field_21;
} struct_0;

extern unsigned int g_4de224;

unsigned int huft_build(unsigned int *a0, unsigned int a1, unsigned int a2, unsigned long a3, unsigned long a4, struct_0 *a5, void* a6)
{
    unsigned int *v21;  // r13
    unsigned int i;  // r12d
    unsigned int v31;  // r12d
    unsigned int *v32;  // rax
    unsigned int v33;  // ebx
    unsigned int v34;  // eax
    unsigned int v35;  // r12d
    unsigned int *v36;  // r13
    struct_0 *v37;  // r14
    unsigned int v38;  // esi
    unsigned int v39;  // ebx
    struct_0 *ptr;  // r14
    struct_0 *ptr1;  // r14
    struct_2 *v41;  // rcx
    unsigned int *v42;  // rax
    unsigned int v43;  // edi
    unsigned int n;  // ebx
    struct_2 *v45;  // rcx
    unsigned int i0;  // ebx
    unsigned int node;  // ebx
    unsigned int iter;  // r15d
    unsigned int iter1;  // r12d
    unsigned long long v27;  // rbx
    unsigned int *v28;  // r13
    unsigned int *v29;  // rax
    unsigned int *v30;  // r13
    unsigned int v0;  // [bp-0x65c]
    struct_0 *v1;  // [bp-0x650]
    unsigned int v2;  // [bp-0x634]
    unsigned int *v3;  // [bp-0x630]
    unsigned int v4;  // [bp-0x61c]
    unsigned int v5;  // [bp-0x618]
    int v6;  // [bp-0x614]
    unsigned int v7;  // [bp-0x610]
    unsigned int l;  // [bp-0x60c], Other Possible Types: int
    unsigned int v9;  // [bp-0x608]
    unsigned int v10;  // [bp-0x604]
    char *iter2;  // [bp-0x600]
    unsigned long long v12;  // [bp-0x5f8], Other Possible Types: unsigned long
    struct_0 *v13;  // [bp-0x5f0]
    unsigned int v14;  // [bp-0x5e8]
    unsigned int v15;  // [bp-0x5e4]
    int <0x409005[is_15]|Stack bp-0x598, 1 B>;  // [bp-0x598]
    unsigned int v16;  // [bp-0x598]
    unsigned int v17;  // [bp-0x594]
    char v18;  // [bp-0x590]
    unsigned long v19;  // [bp-0x548]
    unsigned int v20;  // [bp-0x4c8]

    v3 = a0;
    v2 = a1;
    v1 = a5;
    memset(&v14, 0, 0x44);
    v21 = v3;
    i = v2;
    do
    {
        (&v14)[*(v21)] = (&v14)[*(v21)] + 1;
        v21 += 1;
        i -= 1;
    } while (i);
    if (v2 == v14)
    {
        ptr1 = malloc(48);
        if (ptr1)
        {
            g_4de224 = g_4de224 + 3;
            ptr1->field_8 = 0;
            ptr1->field_10 = 99;
            ptr1->field_11 = 1;
            ptr1->field_20 = 99;
            ptr1->field_21 = 1;
            v1->field_0 = &ptr1->field_10;
            *((unsigned int *)a6) = 1;
            return 0;
        }
        return 3;
    }
    else
    {
        v7 = *((int *)a6);
        for (node = 1; node <= 16 && !(&v14)[node]; node += 1);
        iter = node;
        if (node > v7)
            v7 = node;
        for (iter1 = 16; iter1 && !(&v14)[iter1]; iter1 -= 1);
        v10 = iter1;
        if (iter1 < v7)
            v7 = iter1;
        *((unsigned int *)a6) = v7;
        for (l = 1 << ((char)node & 31); node < iter1; l *= 2)
        {
            l -= (&v14)[node];
            if (l < 0)
                return 2;
            node += 1;
        }
        l -= (&v14)[iter1];
        if (l < 0)
            return 2;
        (&v14)[iter1] = l + (&v14)[iter1];
        v27 = 0;
        v17 = 0;
        v28 = &v15;
        iter2 = &v18;
        while (1)
        {
            iter1 -= 1;
            if (!iter1)
                break;
            v29 = v28;
            v28 = v29 + 1;
            v27 += *(v29);
            iter2 += 4;
            *((unsigned int *)&iter2) = v27;
        }
        v30 = v3;
        v31 = 0;
        do
        {
            v32 = v30;
            v30 = v32 + 1;
            v33 = *(v32);
            if (v33)
            {
                v34 = *((int *)((char *)&<0x409005[is_15]|Stack bp-0x598, 1 B> + 4 * v33));
                *((unsigned int *)((char *)&<0x409005[is_15]|Stack bp-0x598, 1 B> + 4 * v33)) = v34 + 1;
                (&v20)[v34] = v31;
            }
        } while ((v31 += 1, v31 < v2));
        v2 = *((int *)((char *)&<0x409005[is_15]|Stack bp-0x598, 1 B> + 4 * v10));
        v35 = 0;
        v16 = 0;
        v36 = &v20;
        v6 = -0x1;
        v0 = -(v7);
        v19 = 0;
        v37 = NULL;
        for (v9 = 0; iter <= v10; iter += 1)
        {
            v4 = (&v14)[iter];
            while (1)
            {
                v4 -= 1;
                if (!v4)
                    break;
                while (1)
                {
                    if (iter <= v7 + v0)
                        break;
                    v6 += 1;
                    v0 += v7;
                    v9 = v10 - v0;
                    v9 = (v9 <= v7 ? v9 : v7);
                    v39 = iter - v0;
                    v5 = 1 << ((char)v39 & 31);
                    if (v5 > v4 + 1)
                    {
                        v5 = v5 - v4 - 1;
                        iter2 = &(&v14)[iter];
                        if (v39 < v9)
                        {
                            while (1)
                            {
                                v39 += 1;
                                if (v39 >= v9 || !(v5 *= 2, iter2 += 4, v5 > *((int *)&iter2)))
                                    break;
                                v5 -= *((int *)&iter2);
                            }
                        }
                    }
                    v9 = 1 << ((char)v39 & 31);
                    ptr = malloc((v9 + 1) * 16);
                    if (!ptr)
                    {
                        if (!v6)
                            return 3;
                        sub_40975b(v19);
                        return 3;
                    }
                    else
                    {
                        g_4de224 = v9 + g_4de224 + 1;
                        v1->field_0 = &ptr->field_10;
                        v1 = &ptr->field_8;
                        v1->field_0 = NULL;
                        v37 = &ptr->field_10;
                        (&v19)[v6] = v37;
                        if (v6)
                        {
                            (&v16)[v6] = v35;
                            *((char *)&v12 + 1) = v7;
                            *((char *)&v12) = (char)v39 + 16;
                            v13 = v37;
                            v41 = *((long long *)((char *)&(&v19)[v6] - 8)) + (v35 >> ((char)(v0 - v7) & 31)) * 16;
                            v41->field_0 = v12;
                            v41->field_8 = v13;
                        }
                    }
                }
                *((char *)&v12 + 1) = iter - v38;
                if (v36 >= &(&v20)[v2])
                {
                    *((char *)&v12) = 99;
                }
                else if (a2 > *(v36))
                {
                    *((char *)&v12) = (*(v36) <= 0xff ? 16 : 15);
                    *((unsigned short *)&v13) = *(v36);
                    v36 += 1;
                }
                else
                {
                    *((char *)&v12) = *((short *)(a4 + (*(v36) - a2) * 2));
                    v42 = v36;
                    v36 = v42 + 1;
                    *((short *)&v13) = *((short *)(a3 + (*(v42) - a2) * 2));
                }
                v43 = v0;
                v5 = 1 << ((char)(iter - v43 & 0xffffffff) & 31);
                for (n = v35 >> ((char)v43 & 31); n < v9; n += v5)
                {
                    v45 = &(&v37->field_0)[2 * n];
                    v45->field_0 = v12;
                    v45->field_8 = v13;
                }
                for (i0 = 1 << ((char)(iter - 1) & 31); v35 & i0; i0 >>= 1)
                {
                    v35 ^= i0;
                }
                v35 ^= i0;
                while (1)
                {
                    if (((1 << ((char)v0 & 31)) - 1 & v35) == (&v16)[v6])
                        break;
                    v6 -= 1;
                    v0 -= v7;
                }
            }
        }
        if (!l || v10 == 1)
            return 0;
        return 1;
        return 0;
    }
}



// Function: huft_free @ 0x975b
unsigned int huft_free(void* a0)
{
    void* ptr;  // rbx
    void* i;  // [bp-0x20]

    i = a0;
    while (i)
    {
        ptr = i - 16;
        i = (long long)ptr[8];
        free(ptr);
    }
    return 0;
}



// Function: inflate_codes @ 0x979a
extern unsigned short g_4192e0[4];
extern char g_41b000;
extern char g_4ad000;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;
extern unsigned long long g_4de218;
extern unsigned int g_4de220;

unsigned int inflate_codes(unsigned long a0, unsigned long a1, unsigned int a2, int a3)
{
    unsigned long v7;  // r13
    unsigned long iter;  // rbx
    unsigned long long v17;  // r13
    unsigned long long j;  // rbx
    unsigned int v19;  // eax
    unsigned long v20;  // rax
    unsigned long v21;  // r12
    int v22;  // r12d
    unsigned int v23;  // eax
    unsigned long v24;  // rax
    unsigned long long v25;  // r13
    unsigned long long k;  // rbx
    unsigned int v9;  // eax
    unsigned int v27;  // eax
    unsigned long v28;  // rax
    unsigned int v29;  // r12d
    unsigned int m;  // r12d
    unsigned long v10;  // rax
    unsigned long v11;  // r12
    unsigned int v12;  // r12d
    unsigned int v13;  // eax
    unsigned long v14;  // rax
    unsigned int v15;  // eax
    unsigned long v16;  // rax
    unsigned int l;  // [bp-0x44]
    unsigned int node;  // [bp-0x40]
    unsigned int iter1;  // [bp-0x3c]
    unsigned int v3;  // [bp-0x38]
    unsigned int v4;  // [bp-0x34]
    void* idx;  // [bp-0x30]

    v7 = g_4de218;
    iter = g_4de220;
    iter1 = g_4dd870;
    v3 = g_4192e0[a2];
    v4 = g_4192e0[a3];
    while (1)
    {
        if ((unsigned int)iter < a2)
        {
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v9 = g_4dd86c;
                g_4dd86c = v9 + 1;
                v10 = *(&(&g_41b000)[v9]);
            }
            else
            {
                g_4dd870 = iter1;
                v10 = sub_40f0bc(0) & 0xff;
            }
            v7 |= v10 << ((char)(iter & 0xffffffff) & 63);
            iter = (unsigned int)iter + 8;
        }
        else
        {
            idx = a0 + ((unsigned int)v7 & v3) * 16;
            v11 = *((char *)idx);
            if ((unsigned int)v11 > 16)
            {
                do
                {
                    if ((unsigned int)v11 == 99)
                        return 1;
                    v7 >>= (char)idx[1] & 63;
                    iter -= (char)idx[1];
                    for (v12 = (unsigned int)v11 - 16; (unsigned int)iter < v12; iter = (unsigned int)iter + 8)
                    {
                        if (g_4dd86c < *((int *)&g_4dd868))
                        {
                            v13 = g_4dd86c;
                            g_4dd86c = v13 + 1;
                            v14 = *(&(&g_41b000)[v13]);
                        }
                        else
                        {
                            g_4dd870 = iter1;
                            v14 = sub_40f0bc(0) & 0xff;
                        }
                        v7 |= v14 << ((char)(iter & 0xffffffff) & 63);
                    }
                    idx = (g_4192e0[v12] & (unsigned int)v7) * 16 + (long long)idx[8];
                    v11 = *((char *)idx);
                } while ((unsigned int)v11 > 16);
            }
            v7 >>= (char)idx[1] & 63;
            if ((unsigned int)v11 == 16)
            {
                iter1 += 1;
                *(&(&g_4ad000)[iter1]) = (short)idx[8];
                if (iter1 == 0x8000)
                {
                    g_4dd870 = iter1;
                    sub_40f2cc();
                    iter1 = 0;
                }
            }
            else if ((unsigned int)v11 != 15)
            {
                for (iter -= (char)idx[1]; (unsigned int)iter < (unsigned int)v11; iter = (unsigned int)iter + 8)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v15 = g_4dd86c;
                        g_4dd86c = v15 + 1;
                        v16 = *(&(&g_41b000)[v15]);
                    }
                    else
                    {
                        g_4dd870 = iter1;
                        v16 = sub_40f0bc(0) & 0xff;
                    }
                    v7 |= v16 << ((char)(iter & 0xffffffff) & 63);
                }
                l = (g_4192e0[v11 & 0xffffffff] & (unsigned int)v7) + (short)idx[8];
                v17 = v7 >> ((char)(v11 & 0xffffffff) & 63);
                for (j = iter - v11; (unsigned int)j < a3; j = (unsigned int)j + 8)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v19 = g_4dd86c;
                        g_4dd86c = v19 + 1;
                        v20 = *(&(&g_41b000)[v19]);
                    }
                    else
                    {
                        g_4dd870 = iter1;
                        v20 = sub_40f0bc(0) & 0xff;
                    }
                    v17 |= v20 << ((char)(j & 0xffffffff) & 63);
                }
                idx = a1 + ((unsigned int)v17 & v4) * 16;
                v21 = *((char *)idx);
                if ((unsigned int)v21 > 16)
                {
                    do
                    {
                        if ((unsigned int)v21 == 99)
                            return 1;
                        v17 >>= (char)idx[1] & 63;
                        j -= (char)idx[1];
                        for (v22 = (unsigned int)v21 - 16; (unsigned int)j < v22; j = (unsigned int)j + 8)
                        {
                            if (g_4dd86c < *((int *)&g_4dd868))
                            {
                                v23 = g_4dd86c;
                                g_4dd86c = v23 + 1;
                                v24 = *(&(&g_41b000)[v23]);
                            }
                            else
                            {
                                g_4dd870 = iter1;
                                v24 = sub_40f0bc(0) & 0xff;
                            }
                            v17 |= v24 << ((char)(j & 0xffffffff) & 63);
                        }
                        idx = (g_4192e0[v22] & (unsigned int)v17) * 16 + (long long)idx[8];
                        v21 = *((char *)idx);
                    } while ((unsigned int)v21 > 16);
                }
                v25 = v17 >> ((char)idx[1] & 63);
                for (k = j - (char)idx[1]; (unsigned int)k < (unsigned int)v21; k = (unsigned int)k + 8)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v27 = g_4dd86c;
                        g_4dd86c = v27 + 1;
                        v28 = *(&(&g_41b000)[v27]);
                    }
                    else
                    {
                        g_4dd870 = iter1;
                        v28 = sub_40f0bc(0) & 0xff;
                    }
                    v25 |= v28 << ((char)(k & 0xffffffff) & 63);
                }
                node = (iter1 - (short)idx[8] & 0xffffffff) - ((unsigned int)v25 & g_4192e0[v21 & 0xffffffff]);
                v7 = v25 >> ((char)(v21 & 0xffffffff) & 63);
                iter = k - v21;
                do
                {
                    node &= 0x7fff;
                    v29 = (node <= iter1 ? 0x8000 - iter1 : 0x8000 - node);
                    m = (v29 <= l ? v29 : l);
                    l -= m;
                    if ((node < iter1 ? iter1 - node : node - iter1) >= m)
                    {
                        memcpy(&(&g_4ad000)[iter1], &(&g_4ad000)[node], m);
                        iter1 += m;
                        node += m;
                    }
                    else
                    {
                        do
                        {
                            node += 1;
                            iter1 += 1;
                            *(&(&g_4ad000)[iter1]) = *(&(&g_4ad000)[node]);
                            m -= 1;
                        } while (m);
                    }
                    if (iter1 == 0x8000)
                    {
                        g_4dd870 = iter1;
                        sub_40f2cc();
                        iter1 = 0;
                    }
                } while (l);
            }
            else
            {
                g_4dd870 = iter1;
                g_4de218 = v7;
                g_4de220 = iter;
                return 0;
            }
        }
    }
}



// Function: inflate_stored @ 0x9d74
extern char g_41b000;
extern char g_4ad000;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;
extern unsigned long long g_4de218;
extern unsigned int g_4de220;

unsigned int inflate_stored(void)
{
    unsigned long long v4;  // r12
    unsigned int i;  // ebx
    unsigned int v14;  // eax
    unsigned long v15;  // rax
    unsigned int v6;  // eax
    unsigned long v7;  // rax
    unsigned long long v8;  // r12
    unsigned int j;  // ebx
    unsigned int v10;  // eax
    unsigned long v11;  // rax
    unsigned long long v12;  // r12
    unsigned int iter;  // ebx
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]
    unsigned int v2;  // [bp-0x1c]

    v1 = g_4dd870;
    v0 = g_4de220 & 7;
    v4 = g_4de218 >> ((char)v0 & 63);
    for (i = g_4de220 - v0; i <= 15; i += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v6 = g_4dd86c;
            g_4dd86c = v6 + 1;
            v7 = *(&(&g_41b000)[v6]);
        }
        else
        {
            g_4dd870 = v1;
            v7 = sub_40f0bc(0) & 0xff;
        }
        v4 |= v7 << ((char)i & 63);
    }
    v0 = (unsigned int)v4 & 0xffff;
    v8 = v4 >> 16;
    for (j = i - 16; j <= 15; j += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v10 = g_4dd86c;
            g_4dd86c = v10 + 1;
            v11 = *(&(&g_41b000)[v10]);
        }
        else
        {
            g_4dd870 = v1;
            v11 = sub_40f0bc(0) & 0xff;
        }
        v8 |= v11 << ((char)j & 63);
    }
    if (v0 != (~((unsigned int)v8) & 0xffff))
        return 1;
    v12 = v8 >> 16;
    iter = j - 16;
    while (1)
    {
        v2 = v1;
        v0 -= 1;
        if (!v0)
            break;
        for (; iter <= 7; iter += 8)
        {
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v14 = g_4dd86c;
                g_4dd86c = v14 + 1;
                v15 = *(&(&g_41b000)[v14]);
            }
            else
            {
                g_4dd870 = v2;
                v15 = sub_40f0bc(0) & 0xff;
            }
            v12 |= v15 << ((char)iter & 63);
        }
        v1 = v2 + 1;
        *(&(&g_4ad000)[v2]) = v12 & 0xffffffff;
        if (v1 == 0x8000)
        {
            g_4dd870 = v1;
            sub_40f2cc();
            v1 = 0;
        }
        v12 >>= 8;
        iter -= 8;
    }
    g_4dd870 = v2;
    g_4de218 = v12;
    g_4de220 = iter;
    return 0;
}



// Function: inflate_fixed @ 0x9f58
extern unsigned int g_4191e0;
extern unsigned int g_419220;
extern unsigned int g_419260;
extern unsigned int g_4192a0;

unsigned int inflate_fixed(void)
{
    unsigned int v0;  // [bp-0x4b4]
    unsigned int v1;  // [bp-0x4b0]
    int iter;  // [bp-0x4ac], Other Possible Types: unsigned int
    void* v3;  // [bp-0x4a8]
    void* v4;  // [bp-0x4a0]
    unsigned int v5[290];  // [bp-0x498]

    for (iter = 0; iter <= 143; iter += 1)
    {
        v5[iter] = 8;
    }
    for (; iter <= 0xff; iter += 1)
    {
        v5[iter] = 9;
    }
    for (; iter <= 279; iter += 1)
    {
        v5[iter] = 7;
    }
    for (; iter <= 287; iter += 1)
    {
        v5[iter] = 8;
    }
    v0 = 7;
    iter = sub_409005(v5, 288, 0x101, &g_4191e0, &g_419220, &v3, &v0);
    if (iter)
        return iter;
    for (iter = 0; iter <= 29; iter += 1)
    {
        v5[iter] = 5;
    }
    v1 = 5;
    iter = sub_409005(v5, 30, 0, &g_419260, &g_4192a0, &v4, &v1);
    if (iter > 1)
    {
        sub_40975b(v3);
        return iter;
    }
    else if (sub_40979a(v3, v4, v0, v1))
    {
        return 1;
    }
    else
    {
        sub_40975b(v3);
        sub_40975b(v4);
        return 0;
    }
}



// Function: inflate_dynamic @ 0xa191
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    char field_10;
    char field_11;
    char padding_12[14];
    char field_20;
    char field_21;
} struct_0;

extern unsigned int g_419180[4];
extern unsigned int g_4191e0;
extern unsigned int g_419220;
extern unsigned int g_419260;
extern unsigned int g_4192a0;
extern unsigned short g_4192e0[4];
extern unsigned int g_419304;
extern unsigned int g_419308;
extern char g_41b000;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;
extern unsigned long long g_4de218;
extern unsigned int g_4de220;

unsigned int inflate_dynamic(void)
{
    unsigned long long v16;  // r12
    unsigned int i;  // ebx
    unsigned int v26;  // eax
    unsigned long v27;  // rax
    unsigned long long v28;  // r12
    unsigned int iter;  // ebx
    unsigned int v30;  // eax
    unsigned long v31;  // rax
    unsigned int v32;  // eax
    unsigned long v33;  // rax
    unsigned int v34;  // eax
    unsigned long v35;  // rax
    unsigned int v18;  // eax
    unsigned int v36;  // eax
    unsigned long v37;  // rax
    unsigned int v38;  // eax
    unsigned long v39;  // rax
    unsigned long v19;  // rax
    unsigned long long v20;  // r12
    unsigned int j;  // ebx
    unsigned int v22;  // eax
    unsigned long v23;  // rax
    unsigned long long v24;  // r12
    unsigned int k;  // ebx
    unsigned int v0;  // [bp-0x558]
    unsigned int v1;  // [bp-0x554]
    unsigned int iter1;  // [bp-0x550], Other Possible Types: int
    unsigned int node;  // [bp-0x54c]
    unsigned int v4;  // [bp-0x548]
    unsigned int v5;  // [bp-0x544]
    unsigned int v6;  // [bp-0x540]
    unsigned int v7;  // [bp-0x53c]
    unsigned int v8;  // [bp-0x538]
    int m;  // [bp-0x534]
    unsigned int v10;  // [bp-0x530]
    unsigned int v11;  // [bp-0x52c]
    struct_0 v12;  // [bp-0x528]
    struct_0 *v13;  // [bp-0x520]
    unsigned int v14[318];  // [bp-0x518]

    v16 = g_4de218;
    i = g_4de220;
    for (v5 = g_4dd870; i <= 4; i += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v18 = g_4dd86c;
            g_4dd86c = v18 + 1;
            v19 = *(&(&g_41b000)[v18]);
        }
        else
        {
            g_4dd870 = v5;
            v19 = sub_40f0bc(0) & 0xff;
        }
        v16 |= v19 << ((char)i & 63);
    }
    v6 = ((unsigned int)v16 & 31) + 0x101;
    v20 = v16 >> 5;
    for (j = i - 5; j <= 4; j += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v22 = g_4dd86c;
            g_4dd86c = v22 + 1;
            v23 = *(&(&g_41b000)[v22]);
        }
        else
        {
            g_4dd870 = v5;
            v23 = sub_40f0bc(0) & 0xff;
        }
        v20 |= v23 << ((char)j & 63);
    }
    v7 = ((unsigned int)v20 & 31) + 1;
    v24 = v20 >> 5;
    for (k = j - 5; k <= 3; k += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v26 = g_4dd86c;
            g_4dd86c = v26 + 1;
            v27 = *(&(&g_41b000)[v26]);
        }
        else
        {
            g_4dd870 = v5;
            v27 = sub_40f0bc(0) & 0xff;
        }
        v24 |= v27 << ((char)k & 63);
    }
    v8 = ((unsigned int)v24 & 15) + 4;
    v28 = v24 >> 4;
    iter = k - 4;
    if (v6 > 286 || v7 > 30)
        return 1;
    for (; node < v8; node += 1)
    {
        for (node = 0; iter <= 2; iter += 8)
        {
            if (g_4dd86c < *((int *)&g_4dd868))
            {
                v30 = g_4dd86c;
                g_4dd86c = v30 + 1;
                v31 = *(&(&g_41b000)[v30]);
            }
            else
            {
                g_4dd870 = v5;
                v31 = sub_40f0bc(0) & 0xff;
            }
            v28 |= v31 << ((char)iter & 63);
        }
        v14[g_419180[node]] = (unsigned int)v28 & 7;
        v28 >>= 3;
        iter -= 3;
    }
    for (; node <= 18; node += 1)
    {
        v14[g_419180[node]] = 0;
    }
    v0 = 7;
    iter1 = sub_409005(v14, 19, 19, 0, 0, &v12, &v0);
    if (iter1)
    {
        if (iter1 != 1)
            return iter1;
        sub_40975b(v12);
        return iter1;
    }
    else if (!v12)
    {
        return 2;
    }
    else
    {
        m = v7 + v6;
        v10 = g_4192e0[v0];
        v4 = 0;
        while (m > iter1)
        {
            for (iter1 = 0; iter < v0; iter += 8)
            {
                if (g_4dd86c < *((int *)&g_4dd868))
                {
                    v32 = g_4dd86c;
                    g_4dd86c = v32 + 1;
                    v33 = *(&(&g_41b000)[v32]);
                }
                else
                {
                    g_4dd870 = v5;
                    v33 = sub_40f0bc(0) & 0xff;
                }
                v28 |= v33 << ((char)iter & 63);
            }
            v13 = (struct_0 *)(v12 + ((unsigned int)v28 & v10) * 16);
            node = *((char *)&v13->field_0 + 1);
            v28 >>= (char)node & 63;
            iter -= node;
            if ((char)v13->field_0 == 99)
            {
                sub_40975b(v12);
                return 2;
            }
            if (node <= 15)
            {
                v4 = node;
                iter1 += 1;
                v14[iter1] = v4;
            }
            else if (node == 16)
            {
                for (node = (unsigned short)v13->field_8; iter <= 1; iter += 8)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v34 = g_4dd86c;
                        g_4dd86c = v34 + 1;
                        v35 = *(&(&g_41b000)[v34]);
                    }
                    else
                    {
                        g_4dd870 = v5;
                        v35 = sub_40f0bc(0) & 0xff;
                    }
                    v28 |= v35 << ((char)iter & 63);
                }
                node = ((unsigned int)v28 & 3) + 3;
                v28 >>= 2;
                iter -= 2;
                if (m < node + iter1)
                    return 1;
                while (1)
                {
                    node -= 1;
                    if (!node)
                        break;
                    iter1 += 1;
                    v14[iter1] = v4;
                }
            }
            else if (node == 0x11)
            {
                for (; iter <= 2; iter += 8)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v36 = g_4dd86c;
                        g_4dd86c = v36 + 1;
                        v37 = *(&(&g_41b000)[v36]);
                    }
                    else
                    {
                        g_4dd870 = v5;
                        v37 = sub_40f0bc(0) & 0xff;
                    }
                    v28 |= v37 << ((char)iter & 63);
                }
                node = ((unsigned int)v28 & 7) + 3;
                v28 >>= 3;
                iter -= 3;
                if (m < node + iter1)
                    return 1;
                while (1)
                {
                    node -= 1;
                    if (!node)
                        break;
                    iter1 += 1;
                    v14[iter1] = 0;
                }
                v4 = 0;
            }
            else
            {
                for (; iter <= 6; iter += 8)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v38 = g_4dd86c;
                        g_4dd86c = v38 + 1;
                        v39 = *(&(&g_41b000)[v38]);
                    }
                    else
                    {
                        g_4dd870 = v5;
                        v39 = sub_40f0bc(0) & 0xff;
                    }
                    v28 |= v39 << ((char)iter & 63);
                }
                node = ((unsigned int)v28 & 127) + 11;
                v28 >>= 7;
                iter -= 7;
                if (m < node + iter1)
                    return 1;
                while (1)
                {
                    node -= 1;
                    if (!node)
                        break;
                    iter1 += 1;
                    v14[iter1] = 0;
                }
                v4 = 0;
            }
        }
        sub_40975b(v12);
        g_4de218 = v28;
        g_4de220 = iter;
        v0 = g_419304;
        iter1 = sub_409005(v14, v6, 0x101, &g_4191e0, &g_419220, &v12, &v0);
        if (iter1)
        {
            if (iter1 != 1)
                return iter1;
            sub_40975b(v12);
            return iter1;
        }
        else
        {
            v1 = g_419308;
            iter1 = sub_409005(&v14[v6], v7, 0, &g_419260, &g_4192a0, &v13, &v1);
            if (!iter1)
            {
                v11 = sub_40979a(v12, v13, v0, v1);
                sub_40975b(v12);
                sub_40975b(v13);
                return v11;
            }
            if (iter1 == 1)
                sub_40975b(v13);
            sub_40975b(v12);
            return iter1;
        }
    }
    return 1;
}



// Function: inflate_block @ 0xaa58
extern char g_41b000;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;
extern unsigned long long g_4de218;
extern unsigned int g_4de220;

unsigned int inflate_block(unsigned int *a0)
{
    unsigned long long v3;  // r12
    unsigned int i;  // ebx
    unsigned int v5;  // eax
    unsigned long v6;  // rax
    unsigned long long v7;  // r12
    unsigned int j;  // ebx
    unsigned int v9;  // eax
    unsigned long v10;  // rax
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]

    v3 = g_4de218;
    i = g_4de220;
    for (v0 = g_4dd870; !i; i += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v5 = g_4dd86c;
            g_4dd86c = v5 + 1;
            v6 = *(&(&g_41b000)[v5]);
        }
        else
        {
            g_4dd870 = v0;
            v6 = sub_40f0bc(0) & 0xff;
        }
        v3 |= v6 << ((char)i & 63);
    }
    *(a0) = (unsigned int)v3 & 1;
    v7 = v3 >> 1;
    for (j = i - 1; j <= 1; j += 8)
    {
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v9 = g_4dd86c;
            g_4dd86c = v9 + 1;
            v10 = *(&(&g_41b000)[v9]);
        }
        else
        {
            g_4dd870 = v0;
            v10 = sub_40f0bc(0) & 0xff;
        }
        v7 |= v10 << ((char)j & 63);
    }
    v1 = (unsigned int)v7 & 3;
    g_4de218 = v7 >> 2;
    g_4de220 = j - 2;
    return (v1 == 2 ? (unsigned int)sub_40a191() : (!v1 ? (unsigned int)sub_409d74() : (v1 == 1 ? (unsigned int)sub_409f58() : 2)));
}



// Function: inflate @ 0xab98
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;
extern unsigned long long g_4de218;
extern unsigned int g_4de220;
extern unsigned int g_4de224;

unsigned int inflate(void)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned int i;  // [bp-0x18]
    unsigned int v2;  // [bp-0x14]

    g_4dd870 = 0;
    g_4de220 = 0;
    g_4de218 = 0;
    i = 0;
    while (1)
    {
        g_4de224 = 0;
        v2 = sub_40aa58(&v0);
        if (v2)
            return v2;
        if (i < g_4de224)
            i = g_4de224;
        if (v0)
        {
            for (; g_4de220 > 7; g_4dd86c = g_4dd86c - 1)
            {
                g_4de220 = g_4de220 - 8;
            }
            g_4dd870 = g_4dd870;
            sub_40f2cc();
            return 0;
        }
    }
}



// Function: ct_init @ 0xac6f
extern unsigned int g_419320[4];
extern unsigned int g_4193a0[4];
extern long long g_4dec40;
extern unsigned short g_4dec42;
extern unsigned short g_4df0c0;
extern unsigned short g_4df0c2;
extern unsigned short g_4df1e0;
extern unsigned short g_4df1ee;
extern unsigned short g_4df1f0;
extern unsigned short g_4df1f2;
extern char g_4dfd3f;
extern char g_4dfd40;
extern char g_4dfe40;
extern char g_4dff40;
extern unsigned int g_4e0040;
extern unsigned int g_4e00c0;
extern unsigned long long g_4e1160;
extern unsigned long long g_4e1168;
extern unsigned long long g_4e1170;
extern unsigned long long g_4e1178;

long long ct_init(unsigned long a0, unsigned long a1)
{
    unsigned int iter;  // [bp-0x1c], Other Possible Types: int
    int j;  // [bp-0x18]
    int l;  // [bp-0x14]
    int v3;  // [bp-0x14]
    unsigned int iter1;  // [bp-0x10]
    int node;  // [bp-0xc]

    g_4e1170 = a0;
    g_4e1178 = a1;
    g_4e1168 = 0;
    g_4e1160 = g_4e1168;
    if (g_4df0c2)
        return g_4df0c2;
    l = 0;
    for (iter1 = 0; iter1 <= 27; iter1 += 1)
    {
        (&g_4e0040)[iter1] = l;
        for (iter = 0; iter < 1 << ((char)g_419320[iter1] & 31); l = v3)
        {
            v3 = l + 1;
            *(&(&g_4dfd40)[l]) = iter1;
            iter += 1;
        }
    }
    *(&(&g_4dfd3f)[l]) = iter1;
    node = 0;
    for (iter1 = 0; iter1 <= 15; iter1 += 1)
    {
        (&g_4e00c0)[iter1] = node;
        for (iter = 0; iter < 1 << ((char)g_4193a0[iter1] & 31); node = node)
        {
            node += 1;
            *(&(&g_4dfe40)[node]) = iter1;
            iter += 1;
        }
    }
    for (node >>= 7; iter1 <= 29; iter1 += 1)
    {
        (&g_4e00c0)[iter1] = node * 128;
        for (iter = 0; iter < 1 << ((char)g_4193a0[iter1] - 7 & 31); node = node)
        {
            node += 1;
            *(&(&g_4dff40)[node]) = iter1;
            iter += 1;
        }
    }
    for (j = 0; j <= 15; j += 1)
    {
        (&g_4df1e0)[j] = 0;
    }
    for (iter = 0; iter <= 143; iter = iter)
    {
        iter += 1;
        (&g_4dec42)[2 * iter] = 8;
        g_4df1f0 = g_4df1f0 + 1;
    }
    for (; iter <= 0xff; iter = iter)
    {
        iter += 1;
        (&g_4dec42)[2 * iter] = 9;
        g_4df1f2 = g_4df1f2 + 1;
    }
    for (; iter <= 279; iter = iter)
    {
        iter += 1;
        (&g_4dec42)[2 * iter] = 7;
        g_4df1ee = g_4df1ee + 1;
    }
    for (; iter <= 287; iter = iter)
    {
        iter += 1;
        (&g_4dec42)[2 * iter] = 8;
        g_4df1f0 = g_4df1f0 + 1;
    }
    sub_40b76a(&g_4dec40, 287);
    for (iter = 0; iter <= 29; iter += 1)
    {
        (&g_4df0c2)[2 * iter] = 5;
        (&g_4df0c0)[2 * iter] = sub_403bed(iter, 5) & 0xffffffff;
    }
    return sub_40aff6();
}



// Function: init_block @ 0xaff6
extern unsigned short g_4de240;
extern unsigned short g_4de640;
extern unsigned short g_4deb40;
extern unsigned short g_4df140;
extern unsigned int g_4e1140;
extern unsigned int g_4e1144;
extern unsigned int g_4e1148;
extern char g_4e114c;
extern char g_4e114d;
extern unsigned long long g_4e1150;
extern unsigned long long g_4e1158;

int init_block(void)
{
    int iter;  // [bp-0xc]

    for (iter = 0; iter <= 285; iter += 1)
    {
        (&g_4de240)[2 * iter] = 0;
    }
    for (iter = 0; iter <= 29; iter += 1)
    {
        (&g_4deb40)[2 * iter] = 0;
    }
    for (iter = 0; iter <= 18; iter += 1)
    {
        (&g_4df140)[2 * iter] = 0;
    }
    g_4de640 = 1;
    g_4e1158 = 0;
    g_4e1150 = g_4e1158;
    g_4e1148 = 0;
    g_4e1144 = g_4e1148;
    g_4e1140 = g_4e1144;
    g_4e114c = 0;
    g_4e114d = 1;
    return g_4e1144;
}



// Function: pqdownheap @ 0xb0dd
extern unsigned int g_4df200[1277633];
extern char g_4dfaf4;
extern char g_4dfb00;

unsigned int * pqdownheap(unsigned long a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x24]
    unsigned int i;  // [bp-0x10]
    unsigned int *v2;  // [bp-0xc]

    v0 = a1;
    v2 = (unsigned int *)g_4df200[v0];
    for (i = v0 * 2; i <= *((int *)&g_4dfaf4); i *= 2)
    {
        if (i < *((int *)&g_4dfaf4) && (*((short *)(a0 + g_4df200[1 + i] * 4)) < *((short *)(a0 + g_4df200[i] * 4)) || *((short *)(a0 + g_4df200[1 + i] * 4)) == *((short *)(a0 + g_4df200[i] * 4)) && *(&(&g_4dfb00)[g_4df200[1 + i]]) <= *(&(&g_4dfb00)[g_4df200[i]])))
            i += 1;
        if (*((short *)(a0 + (long long)v2 * 4)) < *((short *)(a0 + g_4df200[i] * 4)) || *((short *)(a0 + (long long)v2 * 4)) == *((short *)(a0 + g_4df200[i] * 4)) && *((char *)(v2 + &g_4dfb00)) <= *(&(&g_4dfb00)[g_4df200[i]]))
            break;
        g_4df200[v0] = g_4df200[i];
        v0 = i;
    }
    g_4df200[v0] = (unsigned int)v2;
    return v2;
}



// Function: gen_bitlen @ 0xb363
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    struct struct_1 *field_18;
    unsigned int field_20;
    int field_24;
} struct_0;

typedef struct struct_2 {
    char padding_0[2];
    unsigned short field_2;
} struct_2;

extern unsigned short g_4df1e0[4];
extern unsigned int g_4df200[4];
extern int g_4dfaf8;
extern unsigned long long g_4e1150;
extern unsigned long long g_4e1158;

int gen_bitlen(struct_0 *idx)
{
    struct_2 *v14;  // rax
    unsigned short v0;  // [bp-0x46]
    int iter1;  // [bp-0x44]
    unsigned int *l;  // [bp-0x40], Other Possible Types: int
    int iter;  // [bp-0x3c], Other Possible Types: unsigned int
    unsigned int v4;  // [bp-0x38]
    int node;  // [bp-0x34]
    unsigned int *v6;  // [bp-0x30]
    int v7;  // [bp-0x2c]
    unsigned int v8;  // [bp-0x28]
    unsigned int *v9;  // [bp-0x24]
    unsigned long v10;  // [bp-0x20]
    unsigned long v11;  // [bp-0x18]
    unsigned long long v12;  // [bp-0x10]

    v10 = idx->field_0;
    v11 = idx->field_10;
    v6 = (unsigned int *)(int)idx->field_18;
    v7 = idx->field_24;
    v8 = idx->field_20;
    v12 = idx->field_8;
    node = 0;
    for (iter = 0; iter <= 15; iter += 1)
    {
        g_4df1e0[iter] = 0;
    }
    *((unsigned short *)(v10 + g_4df200[g_4dfaf8] * 4 + 2)) = 0;
    v14 = g_4dfaf8 + 1;
    for (iter1 = (unsigned int)v14; iter1 <= 572; iter1 += 1)
    {
        l = (unsigned int *)g_4df200[iter1];
        iter = *((short *)(v10 + *((short *)(v10 + (long long)l * 4 + 2)) * 4 + 2)) + 1;
        if (iter > v8)
        {
            iter = v8;
            node += 1;
        }
        *((unsigned short *)(v10 + (long long)l * 4 + 2)) = iter;
        v14 = l;
        if ((unsigned int)v14 <= v7)
        {
            g_4df1e0[iter] = g_4df1e0[iter] + 1;
            v4 = 0;
            if (l >= v6)
                v4 = *((int *)(v11 + (l - v6) * 4));
            v0 = *((short *)(v10 + (long long)l * 4));
            v14 = g_4e1150 + (v4 + iter) * v0;
            g_4e1150 = v14;
            if (v12)
            {
                v14 = g_4e1158 + (v4 + *((short *)(v12 + (long long)l * 4 + 2))) * v0;
                g_4e1158 = v14;
            }
        }
    }
    if (node)
    {
        do
        {
            for (iter = v8 - 1; !g_4df1e0[iter]; iter -= 1);
            g_4df1e0[iter] = g_4df1e0[iter] - 1;
            g_4df1e0[1 + iter] = g_4df1e0[1 + iter] + 2;
            g_4df1e0[v8] = g_4df1e0[v8] - 1;
            node -= 2;
        } while (node > 0);
        for (iter = v8; iter; iter -= 1)
        {
            for (l = g_4df1e0[iter]; l; l -= 1)
            {
                iter1 -= 1;
                v9 = (unsigned int *)g_4df200[iter1];
                if (v9 <= v7)
                {
                    v14 = iter;
                    if (*((short *)(v10 + (long long)v9 * 4 + 2)) != (unsigned int)v14)
                    {
                        g_4e1150 = g_4e1150 + (iter - *((short *)(v10 + (long long)v9 * 4 + 2))) * *((short *)(v10 + (long long)v9 * 4));
                        v14 = v10 + (long long)v9 * 4;
                        v14->field_2 = iter;
                    }
                }
            }
        }
    }
    return *((unsigned int *)&v14);
}



// Function: gen_codes @ 0xb76a
extern unsigned short g_4df1e0[4];

void gen_codes(unsigned long a0, int a1)
{
    unsigned short v6;  // ax
    unsigned short v0;  // [bp-0x46]
    int i;  // [bp-0x44]
    int j;  // [bp-0x40]
    unsigned int v3;  // [bp-0x3c]
    char v4;  // [bp-0x38]

    v0 = 0;
    for (i = 1; i <= 15; i += 1)
    {
        v0 = (v0 + *((short *)((char *)&g_4df1e0[i] - 2))) * 2;
        *((unsigned short *)&(&v4)[2 * i]) = v0;
    }
    for (j = 0; j <= a1; j += 1)
    {
        v3 = *((short *)(a0 + j * 4 + 2));
        if (v3)
        {
            v6 = *((short *)&(&v4)[2 * v3]);
            *((unsigned short *)&(&v4)[2 * v3]) = v6 + 1;
            *((unsigned short *)(a0 + j * 4)) = sub_403bed(v6, v3);
        }
    }
    return;
}



// Function: build_tree @ 0xb868
typedef struct struct_1 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_1;

typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    struct struct_1 *field_18;
    unsigned int field_20;
    int field_24;
} struct_0;

extern unsigned int g_4df200[4];
extern unsigned int g_4df204;
extern int g_4dfaf4;
extern int g_4dfaf8;
extern char g_4dfb00;
extern unsigned long long g_4e1150;
extern unsigned long long g_4e1158;

void build_tree(struct_0 *idx)
{
    int v10;  // edx
    int v11;  // eax
    struct_0 *index;  // rax
    unsigned int iter;  // [bp-0x30], Other Possible Types: int
    int node;  // [bp-0x2c]
    int v2;  // [bp-0x28]
    int v3;  // [bp-0x28]
    int v4;  // [bp-0x24]
    unsigned int v5;  // [bp-0x20]
    unsigned int *v6;  // [bp-0x1c]
    unsigned long v7;  // [bp-0x18]
    unsigned long v8;  // [bp-0x10]

    v7 = idx->field_0;
    v8 = idx->field_8;
    v4 = *((int *)((char *)&idx->field_18 + 4));
    node = -0x1;
    v3 = v4;
    g_4dfaf4 = 0;
    g_4dfaf8 = 573;
    for (iter = 0; iter < v4; iter += 1)
    {
        if (*((short *)(v7 + iter * 4)))
        {
            node = iter;
            g_4dfaf4 = g_4dfaf4 + 1;
            g_4df200[g_4dfaf4] = node;
            *(&(&g_4dfb00)[iter]) = 0;
        }
        else
        {
            *((unsigned short *)(v7 + iter * 4 + 2)) = 0;
        }
    }
    for (; g_4dfaf4 <= 1; g_4e1158 = g_4e1158 - *((short *)(v8 + (long long)v6 * 4 + 2)))
    {
        if (node <= 1)
        {
            node += 1;
            v10 = node;
        }
        else
        {
            v10 = 0;
        }
        g_4dfaf4 = g_4dfaf4 + 1;
        g_4df200[g_4dfaf4] = v10;
        v6 = (unsigned int *)g_4df200[g_4dfaf4];
        *((unsigned short *)(v7 + (long long)v6 * 4)) = 1;
        *((char *)(v6 + &g_4dfb00)) = 0;
        g_4e1150 = g_4e1150 - 1;
    }
    idx->field_24 = node;
    for (iter = g_4dfaf4 + (g_4dfaf4 >> 31) >> 1; iter > 0; iter -= 1)
    {
        sub_40b0dd(v7, iter);
    }
    do
    {
        iter = g_4df204;
        v11 = g_4dfaf4;
        g_4dfaf4 = v11 - 1;
        g_4df204 = g_4df200[v11];
        sub_40b0dd(v7, 1);
        v5 = g_4df204;
        g_4dfaf8 = g_4dfaf8 - 1;
        g_4df200[g_4dfaf8] = iter;
        g_4dfaf8 = g_4dfaf8 - 1;
        g_4df200[g_4dfaf8] = v5;
        *((unsigned short *)(v7 + v3 * 4)) = *((short *)(v7 + v5 * 4)) + *((short *)(v7 + iter * 4));
        *(&(&g_4dfb00)[v3]) = (*(&(&g_4dfb00)[iter]) < *(&(&g_4dfb00)[v5]) ? *(&(&g_4dfb00)[v5]) + 1 : *(&(&g_4dfb00)[iter]) + 1);
        index = v7 + v5 * 4;
        *((short *)((char *)&index->field_0 + 2)) = v3;
        *((short *)(v7 + iter * 4 + 2)) = *((short *)((char *)&index->field_0 + 2));
        v2 = v3 + 1;
        g_4df204 = v3;
        sub_40b0dd(v7, 1);
        v3 = v2;
    } while (g_4dfaf4 > 1);
    g_4dfaf8 = g_4dfaf8 - 1;
    g_4df200[g_4dfaf8] = g_4df204;
    sub_40b363(idx);
    sub_40b76a(v7, node);
    return;
}



// Function: scan_tree @ 0xbc5c
typedef struct struct_0 {
    char padding_0[2];
    unsigned short field_2;
} struct_0;

extern unsigned short g_4df140[4];
extern unsigned short g_4df180;
extern unsigned short g_4df184;
extern unsigned short g_4df188;

long long scan_tree(struct_0 *a0, unsigned int a1)
{
    unsigned int i;  // [bp-0x24]
    unsigned int v1;  // [bp-0x20]
    unsigned int v2;  // [bp-0x1c]
    unsigned int v3;  // [bp-0x18]
    unsigned int v4;  // [bp-0x14]
    unsigned int v5;  // [bp-0x10]
    unsigned int v6;  // [bp-0xc]

    v1 = 0xffffffff;
    v2 = a0->field_2;
    v3 = 0;
    v4 = 7;
    v5 = 4;
    if (!v2)
    {
        v4 = 138;
        v5 = 3;
    }
    a0[1 + a1].field_2 = 0xffff;
    for (i = 0; i <= a1; i += 1)
    {
        v6 = v2;
        v2 = a0[1 + i].field_2;
        v3 += 1;
        if (v3 >= v4 || v6 != v2)
        {
            if (v3 < v5)
            {
                g_4df140[2 * v6] = g_4df140[2 * v6] + v3;
            }
            else if (v6)
            {
                if (v6 != v1)
                    g_4df140[2 * v6] = g_4df140[2 * v6] + 1;
                g_4df180 = g_4df180 + 1;
            }
            else if (v3 <= 10)
            {
                g_4df184 = g_4df184 + 1;
            }
            else
            {
                g_4df188 = g_4df188 + 1;
            }
            v3 = 0;
            v1 = v6;
            if (!v2)
            {
                v4 = 138;
                v5 = 3;
            }
            else if (v6 == v2)
            {
                v4 = 6;
                v5 = 3;
            }
            else
            {
                v4 = 7;
                v5 = 4;
            }
        }
    }
    return i;
}



// Function: send_tree @ 0xbe31
typedef struct struct_0 {
    char padding_0[2];
    unsigned short field_2;
} struct_0;

extern unsigned short g_4df140[4];
extern unsigned short g_4df142[4];
extern unsigned short g_4df180;
extern unsigned short g_4df182;
extern unsigned short g_4df184;
extern unsigned short g_4df186;
extern unsigned short g_4df188;
extern unsigned short g_4df18a;

long long send_tree(struct_0 *a0, unsigned int a1)
{
    unsigned int i;  // [bp-0x24]
    unsigned int v1;  // [bp-0x20]
    unsigned int v2;  // [bp-0x1c]
    unsigned int iter;  // [bp-0x18]
    unsigned int v4;  // [bp-0x14]
    unsigned int v5;  // [bp-0x10]
    unsigned int v6;  // [bp-0xc]

    v1 = 0xffffffff;
    v2 = a0->field_2;
    iter = 0;
    v4 = 7;
    v5 = 4;
    if (!v2)
    {
        v4 = 138;
        v5 = 3;
    }
    for (i = 0; i <= a1; i += 1)
    {
        v6 = v2;
        v2 = a0[1 + i].field_2;
        iter += 1;
        if (iter >= v4 || v6 != v2)
        {
            if (iter < v5)
            {
                do
                {
                    sub_403a6c(g_4df140[2 * v6], g_4df142[2 * v6]);
                    iter -= 1;
                } while (iter);
            }
            else if (v6)
            {
                if (v6 != v1)
                {
                    sub_403a6c(g_4df140[2 * v6], g_4df142[2 * v6]);
                    iter -= 1;
                }
                sub_403a6c(g_4df180, g_4df182);
                sub_403a6c(iter - 3, 2);
            }
            else if (iter <= 10)
            {
                sub_403a6c(g_4df184, g_4df186);
                sub_403a6c(iter - 3, 3);
            }
            else
            {
                sub_403a6c(g_4df188, g_4df18a);
                sub_403a6c(iter - 11, 7);
            }
            iter = 0;
            v1 = v6;
            if (!v2)
            {
                v4 = 138;
                v5 = 3;
            }
            else if (v6 == v2)
            {
                v4 = 6;
                v5 = 3;
            }
            else
            {
                v4 = 7;
                v5 = 4;
            }
        }
    }
    return i;
}



// Function: build_bl_tree @ 0xc06c
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_419470;
extern unsigned int g_4194c4;
extern unsigned int g_419504;
extern struct_0 g_419520;
extern struct_0 g_4de240;
extern struct_0 g_4deb40;
extern unsigned short g_4df142[4];
extern unsigned long long g_4e1150;

int build_bl_tree(void)
{
    int i;  // [bp-0xc]

    sub_40bc5c(&g_4de240.field_0, g_4194c4);
    sub_40bc5c(&g_4deb40.field_0, g_419504);
    sub_40b868(&g_419520.field_0);
    for (i = 18; i > 2 && !g_4df142[2 * *(&(&g_419470)[i])]; i -= 1);
    g_4e1150 = g_4e1150 + (i + 1) * 3 + 14;
    return i;
}



// Function: send_all_trees @ 0xc120
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_419470;
extern struct_0 g_4de240;
extern struct_0 g_4deb40;
extern unsigned short g_4df142[4];

long long send_all_trees(unsigned int a0, unsigned int a1, unsigned int a2)
{
    unsigned int i;  // [bp-0xc]

    sub_403a6c(a0 - 0x101, 5);
    sub_403a6c(a1 - 1, 5);
    sub_403a6c(a2 - 4, 4);
    for (i = 0; i < a2; i += 1)
    {
        sub_403a6c(g_4df142[2 * *(&(&g_419470)[i])], 3);
    }
    sub_40be31(&g_4de240.field_0, a0 - 1);
    return sub_40be31(&g_4deb40.field_0, a1 - 1);
}



// Function: flush_block @ 0xc1ea
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_4194a0;
extern unsigned int g_4194c4;
extern struct_0 g_4194e0;
extern unsigned int g_419504;
extern long long g_4de240;
extern long long g_4deb40;
extern long long g_4dec40;
extern long long g_4df0c0;
extern char g_4e0140;
extern unsigned int g_4e1148;
extern char g_4e114c;
extern unsigned long long g_4e1150;
extern unsigned long long g_4e1158;
extern void g_4e1160;
extern unsigned long long g_4e1168;
extern struct_0 *g_4e1170;

long long flush_block(char *a0, unsigned long long a1, unsigned int a2, unsigned int a3)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    *(&(&g_4e0140)[g_4e1148]) = g_4e114c;
    if ((short)g_4e1170->field_0 == 0xffff)
        sub_40c97c();
    sub_40b868(&g_4194a0.field_0);
    sub_40b868(&g_4194e0.field_0);
    v0 = sub_40c06c();
    v1 = g_4e1150 + 10 >> 3;
    v2 = g_4e1158 + 10 >> 3;
    g_4e1168 = a1 + g_4e1168;
    if (v2 <= v1)
        v1 = v2;
    if (v1 >= a1 + 4 && a0)
    {
        sub_403a6c(a3, 3);
        *((unsigned long long *)&g_4e1160) = *((long long *)&g_4e1160) + 10 & 0xfffffffffffffff8;
        *((unsigned long long *)&g_4e1160) = *((long long *)&g_4e1160) + (a1 + 4) * 8;
        sub_403d59(a0, a1, 1);
    }
    else if (v2 == v1)
    {
        sub_403a6c((unsigned short)a3 + 2, 3);
        sub_40c6f9(&g_4dec40, &g_4df0c0);
        *((unsigned long long *)&g_4e1160) = *((long long *)&g_4e1160) + g_4e1158 + 3;
    }
    else
    {
        sub_403a6c((unsigned short)a3 + 4, 3);
        sub_40c120(g_4194c4 + 1, g_419504 + 1, v0 + 1);
        sub_40c6f9(&g_4de240, &g_4deb40);
        *((unsigned long long *)&g_4e1160) = *((long long *)&g_4e1160) + g_4e1150 + 3;
    }
    sub_40aff6();
    if (a3)
    {
        sub_403c22();
        *((unsigned long long *)&g_4e1160) = *((long long *)&g_4e1160) + 7;
    }
    else if (a2 && (char)*((long long *)&g_4e1160) & 7)
    {
        sub_403a6c(a3, 3);
        *((unsigned long long *)&g_4e1160) = *((long long *)&g_4e1160) + 10 & 0xfffffffffffffff8;
        sub_403d59(a0, 0, 1);
    }
    return *((long long *)&g_4e1160) >> 3;
}



// Function: ct_tally @ 0xc45d
extern int g_4190a0;
extern unsigned int g_4193a0[4];
extern unsigned long long g_41a068;
extern unsigned int g_41a074;
extern char g_41b000;
extern unsigned short g_49c800;
extern unsigned short g_4de240[4];
extern unsigned short g_4deb40[4];
extern char g_4dfd40;
extern char g_4dfe40;
extern char g_4dff40;
extern char g_4e0140;
extern unsigned int g_4e1140;
extern unsigned int g_4e1144;
extern unsigned int g_4e1148;
extern char g_4e114c;
extern char g_4e114d;

unsigned int ct_tally(int a0, unsigned int a1)
{
    unsigned int v5;  // eax
    unsigned int v6;  // eax
    int v7;  // eax
    unsigned int v8;  // eax
    unsigned int v9;  // edx
    int v0;  // [bp-0x2c]
    int i;  // [bp-0x1c]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = a0;
    v5 = g_4e1140;
    g_4e1140 = v5 + 1;
    *(&(&g_41b000)[v5]) = a1;
    if (!v0)
    {
        g_4de240[2 * a1] = g_4de240[2 * a1] + 1;
    }
    else
    {
        v0 -= 1;
        v6 = *(&(&g_4dfd40)[a1]) + 0x101;
        g_4de240[2 * v6] = g_4de240[2 * v6] + 1;
        v7 = (v0 <= 0xff ? *(&(&g_4dfe40)[v0]) : *(&(&g_4dff40)[v0 >> 7]));
        g_4deb40[2 * v7] = g_4deb40[2 * v7] + 1;
        v8 = g_4e1144;
        g_4e1144 = v8 + 1;
        (&g_49c800)[v8] = v0;
        g_4e114c = g_4e114d | g_4e114c;
    }
    g_4e114d = g_4e114d * 2;
    if (!((char)g_4e1140 & 7))
    {
        v9 = g_4e1148;
        g_4e1148 = v9 + 1;
        *(&(&g_4e0140)[v9]) = g_4e114c;
        g_4e114c = 0;
        g_4e114d = 1;
    }
    if (g_4190a0 > 2 && !((unsigned short)g_4e1140 & 0xfff))
    {
        v2 = g_4e1140 * 8;
        v3 = g_41a074 - g_41a068;
        for (i = 0; i <= 29; i += 1)
        {
            v2 += (g_4193a0[i] + 5) * g_4deb40[2 * i];
        }
        v2 >>= 3;
        if (g_4e1140 >> 1 > g_4e1144 && v2 < v3 >> 1)
            return 1;
    }
    if (g_4e1140 != 0x7fff && g_4e1144 != 0x8000)
        return 0;
    return 1;
}



// Function: compress_block @ 0xc6f9
extern unsigned int g_419320[4];
extern unsigned int g_4193a0[4];
extern char g_41b000;
extern unsigned short g_49c800[4];
extern char g_4dfd40;
extern char g_4dfe40;
extern char g_4dff40;
extern unsigned int g_4e0040[4];
extern unsigned int g_4e00c0[4];
extern char g_4e0140;
extern char g_4e1140;

int compress_block(unsigned short *a0, unsigned long a1)
{
    unsigned int v9;  // eax
    unsigned int v10;  // eax
    unsigned int v11;  // eax
    char v0;  // [bp-0x25]
    unsigned int i;  // [bp-0x24]
    unsigned int v2;  // [bp-0x20]
    unsigned int v3;  // [bp-0x1c]
    unsigned int v4;  // [bp-0x18]
    unsigned int v5;  // [bp-0x14]
    unsigned int *v6;  // [bp-0x10]
    unsigned int v7;  // [bp-0xc]

    i = 0;
    v2 = 0;
    v3 = 0;
    v0 = 0;
    if (!*((int *)&g_4e1140))
        return sub_403a6c(a0[0x200], a0[513]);
    do
    {
        if (!((char)i & 7))
        {
            v9 = v3;
            v3 = v9 + 1;
            v0 = *(&(&g_4e0140)[v9]);
        }
        v10 = i;
        i = v10 + 1;
        v4 = *(&(&g_41b000)[v10]);
        if (!(v0 & 1))
        {
            sub_403a6c(a0[2 * v4], a0[1 + 2 * v4]);
        }
        else
        {
            v5 = *(&(&g_4dfd40)[v4]);
            sub_403a6c(a0[0x202 + 2 * v5], a0[515 + 2 * v5]);
            v6 = (unsigned int *)g_419320[v5];
            if (v6)
            {
                v4 -= g_4e0040[v5];
                sub_403a6c(v4, v6);
            }
            v11 = v2;
            v2 = v11 + 1;
            v7 = g_49c800[v11];
            v5 = (v7 <= 0xff ? *(&(&g_4dfe40)[v7]) : *(&(&g_4dff40)[v7 >> 7]));
            sub_403a6c(*((short *)(a1 + v5 * 4)), *((short *)(a1 + v5 * 4 + 2)));
            v6 = (unsigned int *)g_4193a0[v5];
            if (v6)
            {
                v7 -= g_4e00c0[v5];
                sub_403a6c(v7, v6);
            }
        }
        v0 >>= 1;
    } while (i < *((int *)&g_4e1140));
    return sub_403a6c(a0[0x200], a0[513]);
}



// Function: set_file_type @ 0xc97c
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern unsigned short g_4de240[4];
extern struct_0 *g_4e1170;

unsigned short * set_file_type(void)
{
    int v4;  // eax
    int v5;  // eax
    int v6;  // eax
    int v0;  // [bp-0x14]
    unsigned int j;  // [bp-0x10]
    unsigned int iter;  // [bp-0xc]

    v0 = 0;
    j = 0;
    for (iter = 0; v0 <= 6; iter += g_4de240[2 * v4])
    {
        v4 = v0;
        v0 = v4 + 1;
    }
    for (; v0 <= 127; j += g_4de240[2 * v5])
    {
        v5 = v0;
        v0 = v5 + 1;
    }
    for (; v0 <= 0xff; iter += g_4de240[2 * v6])
    {
        v6 = v0;
        v0 = v6 + 1;
    }
    *((unsigned short *)&g_4e1170->field_0) = iter <= j >> 2;
    return g_4e1170;
}



// Function: fillbuf @ 0xca3c
extern char g_41b000;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned short g_4e13c0;
extern unsigned int g_4e13c4;
extern unsigned int g_4e13c8;

unsigned int fillbuf(unsigned int a0)
{
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    unsigned int v4;  // edx
    unsigned int v5;  // eax
    unsigned int i;  // [bp-0xc]

    i = a0;
    for (g_4e13c0 = g_4e13c0 << ((char)i & 31); i > g_4e13c8; g_4e13c8 = 8)
    {
        i -= g_4e13c8;
        g_4e13c0 = g_4e13c0 | (unsigned short)(g_4e13c4 << ((char)i & 31));
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v2 = g_4dd86c;
            g_4dd86c = v2 + 1;
            v3 = *(&(&g_41b000)[v2]);
        }
        else
        {
            v3 = sub_40f0bc(1);
        }
        g_4e13c4 = v3;
        if (g_4e13c4 == 0xffffffff)
            g_4e13c4 = 0;
    }
    g_4e13c8 = g_4e13c8 - i;
    v4 = g_4e13c4 >> ((char)g_4e13c8 & 31);
    v5 = g_4e13c0;
    g_4e13c0 = (unsigned short)v5 | (unsigned short)v4;
    return v5 | v4;
}



// Function: getbits @ 0xcb39
extern unsigned short g_4e13c0;

int getbits(unsigned int a0)
{
    unsigned int v0;  // [bp-0xc]

    v0 = (short)(g_4e13c0) >> ((char)(16 - a0 & 0xffffffff) & 31);
    sub_40ca3c(a0);
    return v0;
}



// Function: init_getbits @ 0xcb74
extern unsigned short g_4e13c0;
extern unsigned int g_4e13c4;
extern unsigned int g_4e13c8;

unsigned int init_getbits(void)
{
    g_4e13c0 = 0;
    g_4e13c4 = 0;
    g_4e13c8 = 0;
    return sub_40ca3c(16);
}



// Function: make_table @ 0xcba6
extern unsigned short g_4bd000[4];

int make_table(unsigned int a0, char *a1, unsigned int a2, unsigned long a3)
{
    unsigned int v17;  // eax
    unsigned long v18;  // fs
    unsigned int iter;  // [bp-0xc0]
    unsigned int v1;  // [bp-0xbc]
    unsigned int j;  // [bp-0xb8]
    unsigned int v3;  // [bp-0xb4]
    unsigned int v4;  // [bp-0xb4]
    unsigned int v5;  // [bp-0xb0]
    unsigned int v6;  // [bp-0xac]
    unsigned int v7;  // [bp-0xa8]
    unsigned int v8;  // [bp-0xa4]
    unsigned short *v9;  // [bp-0xa0]
    char v10;  // [bp-0x98]
    char v11;  // [bp-0x68]
    char v12;  // [bp-0x38]
    unsigned short v13;  // [bp-0x36]
    unsigned short v14;  // [bp-0x16]
    unsigned long v15;  // [bp-0x10]

    for (iter = 1; iter <= 16; iter += 1)
    {
        *((unsigned short *)&(&v10)[2 * iter]) = 0;
    }
    for (iter = 0; iter < a0; iter += 1)
    {
        v17 = a1[iter];
        *((unsigned short *)&(&v10)[2 * v17]) = *((short *)&(&v10)[2 * v17]) + 1;
    }
    v13 = 0;
    for (iter = 1; iter <= 16; iter += 1)
    {
        *((unsigned short *)&(&v12)[2 + 2 * iter]) = *((short *)&(&v12)[2 * iter]) + (*((short *)&(&v10)[2 * iter]) << ((char)(16 - iter) & 31));
    }
    if (v14)
        sub_40f5ef("Bad table\n"); /* do not return */
    v5 = 16 - a2;
    for (iter = 1; iter <= a2; iter += 1)
    {
        *((short *)&(&v12)[2 * iter]) = *((short *)&(&v12)[2 * iter]) >> ((char)v5 & 31);
        *((unsigned short *)&(&v11)[2 * iter]) = 1 << ((char)(a2 - iter) & 31) & 0xffffffff;
    }
    for (; iter <= 16; iter += 1)
    {
        *((unsigned short *)&(&v11)[2 * iter]) = 1 << ((char)(16 - iter) & 31) & 0xffffffff;
    }
    iter = *((short *)&(&v12)[2 + 2 * a2]) >> ((char)v5 & 31);
    if (iter)
    {
        for (v1 = 1 << ((char)a2 & 31); iter != v1; iter = iter)
        {
            iter += 1;
            *((unsigned short *)(a3 + iter * 2)) = 0;
        }
    }
    v3 = a0;
    v6 = 1 << ((char)(15 - a2) & 31);
    for (j = 0; j < a0; j += 1)
    {
        v7 = a1[j];
        if (v7)
        {
            v8 = *((short *)&(&v11)[2 * v7]) + *((short *)&(&v12)[2 * v7]);
            if (v7 <= a2)
            {
                if (v8 > 1 << ((char)a2 & 31))
                    sub_40f5ef("Bad table\n"); /* do not return */
                for (iter = *((short *)&(&v12)[2 * v7]); iter < v8; iter += 1)
                {
                    *((unsigned short *)(a3 + iter * 2)) = j;
                }
            }
            else
            {
                v1 = *((short *)&(&v12)[2 * v7]);
                v9 = a3 + (v1 >> ((char)v5 & 31)) * 2;
                for (iter = v7 - a2; iter; iter -= 1)
                {
                    if (!*(v9))
                    {
                        g_4bd000[v3] = 0;
                        g_4bd000[0x8000 + v3] = g_4bd000[v3];
                        v4 = v3 + 1;
                        *(v9) = v3;
                        v3 = v4;
                    }
                    v9 = (!(v1 & v6) ? &g_4bd000[*(v9)] : &g_4bd000[0x8000 + *(v9)]);
                    v1 *= 2;
                }
                *(v9) = j;
            }
            *((unsigned short *)&(&v12)[2 * v7]) = v8;
        }
    }
    return v15 - *((long long *)(40 + v18));
}



// Function: read_pt_len @ 0xd0a0
extern char g_4e1180;
extern unsigned short g_4e11c0;
extern unsigned short g_4e13c0;

unsigned short * read_pt_len(unsigned short *a0, unsigned int a1, unsigned short *a2)
{
    unsigned short *v5;  // eax
    unsigned int v6;  // eax
    unsigned short *iter;  // [bp-0x18], Other Possible Types: int
    unsigned int node;  // [bp-0x14]
    unsigned int v2;  // [bp-0x10]
    unsigned short *v3;  // [bp-0xc]

    v3 = (unsigned short *)(int)sub_40cb39(a1);
    if (!v3)
    {
        node = sub_40cb39(a1);
        iter = NULL;
        while (1)
        {
            if (iter >= a0)
                break;
            *((char *)(iter + &g_4e1180)) = 0;
            iter = (char *)iter + 1;
        }
        for (iter = 0; iter <= 0xff; iter += 1)
        {
            v5 = &g_4e11c0;
            (&g_4e11c0)[iter] = node;
        }
        return v5;
    }
    else
    {
        iter = NULL;
        while (1)
        {
            if (iter < v3)
            {
                node = g_4e13c0 >> 13;
                if (node == 7)
                {
                    for (v2 = 0x1000; g_4e13c0 & v2; node += 1)
                    {
                        v2 >>= 1;
                    }
                    if (node > 16)
                        sub_40f5ef("Bad table\n"); /* do not return */
                }
                v6 = node;
                if (v6 < 6)
                    v6 = 6;
                sub_40ca3c(v6 - 3);
                iter = (char *)iter + 1;
                *((char *)(iter + &g_4e1180)) = node;
                if (iter == a2)
                {
                    node = sub_40cb39(2);
                    while (1)
                    {
                        node -= 1;
                        if (node < NULL)
                            break;
                        iter = (char *)iter + 1;
                        *((char *)(iter + &g_4e1180)) = 0;
                    }
                }
            }
            else
            {
                for (iter = iter; iter < a0; iter = iter)
                {
                    iter = (char *)iter + 1;
                    *((char *)(iter + &g_4e1180)) = 0;
                }
                return (int)sub_40cba6(a0, &g_4e1180, 8, &g_4e11c0);
            }
        }
    }
}



// Function: read_c_len @ 0xd23e
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_45c000;
extern unsigned short g_49c800;
extern struct_0 g_4bd000;
extern char g_4e1180;
extern unsigned short g_4e11c0[4];
extern unsigned short g_4e13c0;

int read_c_len(void)
{
    unsigned short *v5;  // rax
    int iter;  // [bp-0x18], Other Possible Types: unsigned int
    unsigned int v1;  // [bp-0x14], Other Possible Types: int
    unsigned int v2;  // [bp-0x10]
    unsigned int v3;  // [bp-0xc]

    v3 = sub_40cb39(9);
    if (!v3)
    {
        v1 = sub_40cb39(9);
        for (iter = 0; iter <= 509; iter += 1)
        {
            *(&(&g_45c000)[iter]) = 0;
        }
        for (iter = 0; iter <= 0xfff; iter += 1)
        {
            v5 = &g_49c800;
            (&g_49c800)[iter] = v1;
        }
    }
    else
    {
        for (iter = 0; iter < v3; iter = iter)
        {
            v1 = g_4e11c0[g_4e13c0 >> 8];
            if (v1 > 18)
            {
                v2 = 128;
                do
                {
                    v1 = (!(g_4e13c0 & v2) ? *((short *)(2 * v1 + (char *)&g_4bd000.field_0)) : *((short *)((char *)&g_4bd000.field_0 + 2 * v1 + 0x10000)));
                } while ((v1 = v1, v2 >>= 1, v1 > 18));
            }
            sub_40ca3c(*(&(&g_4e1180)[v1]));
            if (v1 <= 2)
            {
                v1 = (!v1 ? 1 : (v1 == 1 ? sub_40cb39(4) + 3 : sub_40cb39(9) + 20));
                while (1)
                {
                    v1 -= 1;
                    if (v1 < 0)
                        break;
                    iter += 1;
                    *(&(&g_45c000)[iter]) = 0;
                }
            }
            else
            {
                iter += 1;
                *(&(&g_45c000)[iter]) = (v1 & 0xff) - 2;
            }
        }
        for (; iter <= 509; iter = iter)
        {
            iter += 1;
            *(&(&g_45c000)[iter]) = 0;
        }
        v5 = sub_40cba6(0x1fe, &g_45c000, 12, &g_49c800);
    }
    return *((unsigned int *)&v5);
}



// Function: decode_c @ 0xd445
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern char g_45c000;
extern unsigned short g_49c800[4];
extern struct_0 g_4bd000;
extern unsigned int g_4e11a0;
extern unsigned short g_4e13c0;

unsigned int decode_c(void)
{
    unsigned int v0;  // [bp-0x10]
    unsigned int v1;  // [bp-0xc]

    if (!g_4e11a0)
    {
        g_4e11a0 = sub_40cb39(16);
        if (!g_4e11a0)
            return 0x1fe;
        sub_40d0a0(0x13, 5, 0x3);
        sub_40d23e();
        sub_40d0a0(0xe, 4, 0xffffffff);
    }
    g_4e11a0 = g_4e11a0 - 1;
    v0 = g_49c800[g_4e13c0 >> 4];
    if (v0 > 509)
    {
        v1 = 8;
        do
        {
            v0 = (!(g_4e13c0 & v1) ? *((short *)(2 * v0 + (char *)&g_4bd000.field_0)) : *((short *)((char *)&g_4bd000.field_0 + 2 * v0 + 0x10000)));
        } while ((v0 = v0, v1 >>= 1, v0 > 509));
    }
    sub_40ca3c(*(&(&g_45c000)[v0]));
    return v0;
}



// Function: decode_p @ 0xd564
typedef struct struct_0 {
    unsigned long long field_0;
} struct_0;

extern struct_0 g_4bd000;
extern char g_4e1180;
extern unsigned short g_4e11c0[4];
extern unsigned short g_4e13c0;

int decode_p(void)
{
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x1c]

    v0 = g_4e11c0[g_4e13c0 >> 8];
    if (v0 > 13)
    {
        v1 = 128;
        do
        {
            v0 = (!(g_4e13c0 & v1) ? *((short *)(2 * v0 + (char *)&g_4bd000.field_0)) : *((short *)((char *)&g_4bd000.field_0 + 2 * v0 + 0x10000)));
        } while ((v0 = v0, v1 >>= 1, v0 > 13));
    }
    sub_40ca3c(*(&(&g_4e1180)[v0]));
    if (!v0)
        return v0;
    v0 = sub_40cb39(v0 - 1) + (unsigned int)(1 << ((char)(v0 - 1) & 31));
    return v0;
}



// Function: huf_decode_start @ 0xd641
extern unsigned int g_4e11a0;

unsigned int huf_decode_start(void)
{
    unsigned int v1;  // eax

    v1 = sub_40cb74();
    g_4e11a0 = 0;
    return v1;
}



// Function: decode_start @ 0xd65b
extern unsigned int g_4e13cc;
extern unsigned int g_4e13d0;

unsigned int decode_start(void)
{
    unsigned int v1;  // eax

    v1 = sub_40d641();
    g_4e13cc = 0;
    g_4e13d0 = 0;
    return v1;
}



// Function: decode @ 0xd67f
extern int g_4e13cc;
extern unsigned int g_4e13d0;
extern unsigned int g_4e13d4;

unsigned int decode(unsigned int a0, char *a1)
{
    unsigned int iter;  // [bp-0x10]
    unsigned int v1;  // [bp-0xc]

    iter = 0;
    while (1)
    {
        g_4e13cc = g_4e13cc - 1;
        if (g_4e13cc >= 0)
        {
            a1[iter] = a1[g_4e13d4];
            g_4e13d4 = g_4e13d4 + 1 & 0x1fff;
            iter += 1;
            if (iter == a0)
                return iter;
        }
        else
        {
            while (1)
            {
                v1 = sub_40d445();
                if (v1 == 0x1fe)
                {
                    g_4e13d0 = 1;
                    return iter;
                }
                else if (v1 <= 0xff)
                {
                    a1[iter] = v1;
                    iter += 1;
                    if (iter == a0)
                        return iter;
                }
                else
                {
                    g_4e13cc = v1 - 253;
                    g_4e13d4 = (unsigned int)(iter - sub_40d564()) - 1 & 0x1fff;
                    while (1)
                    {
                        g_4e13cc = g_4e13cc - 1;
                        if (g_4e13cc < 0)
                            break;
                        a1[iter] = a1[g_4e13d4];
                        g_4e13d4 = g_4e13d4 + 1 & 0x1fff;
                        iter += 1;
                        if (iter == a0)
                            return iter;
                    }
                }
            }
        }
    }
}



// Function: unlzh @ 0xd7d6
extern char g_4ad000;
extern unsigned int g_4dd860;
extern unsigned int g_4dd864;
extern unsigned int g_4e13d0;

unsigned int unlzh(unsigned int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0xc]

    g_4dd860 = a0;
    g_4dd864 = a1;
    sub_40d65b();
    while (!g_4e13d0)
    {
        v0 = sub_40d67f(0x2000, &g_4ad000);
        if (v0)
            sub_40f321(a1, &g_4ad000, v0, &g_4ad000);
    }
    return 0;
}



// Function: unlzw @ 0xd846
typedef struct FILE {
} FILE;

extern void g_419098;
extern unsigned int g_419548;
extern FILE *stderr;
extern char g_41b000;
extern long long g_45c000;
extern void g_4ac7fe;
extern char g_4ad000;
extern unsigned short g_4bd000[4];
extern unsigned int g_4dd000;
extern unsigned int g_4dd004;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern unsigned long long g_4dd040;
extern char g_4dd060;
extern unsigned int g_4dd868;
extern unsigned int g_4dd86c;

unsigned int unlzw(unsigned int a0, unsigned int a1)
{
    unsigned int v21;  // eax
    unsigned int v22;  // eax
    unsigned int v0;  // [bp-0x7c]
    unsigned int v1;  // [bp-0x78]
    unsigned int iter1;  // [bp-0x78]
    unsigned int v3;  // [bp-0x74]
    unsigned int v4;  // [bp-0x70]
    unsigned int v5;  // [bp-0x6c]
    int j;  // [bp-0x68]
    unsigned int l;  // [bp-0x64]
    unsigned int v8;  // [bp-0x60]
    int v9;  // [bp-0x5c]
    void* iter;  // [bp-0x58]
    unsigned long long node;  // [bp-0x50]
    unsigned long long v12;  // [bp-0x48]
    unsigned long v13;  // [bp-0x40]
    unsigned long long v14;  // [bp-0x38]
    unsigned long long v15;  // [bp-0x30]
    unsigned long long v16;  // [bp-0x28]
    unsigned long k;  // [bp-0x20]
    char *v18;  // [bp-0x18]
    unsigned long long v19;  // [bp-0x10]

    if (g_4dd86c < g_4dd868)
    {
        v21 = g_4dd86c;
        g_4dd86c = v21 + 1;
        v22 = *(&(&g_41b000)[v21]);
    }
    else
    {
        v22 = sub_40f0bc(0);
    }
    *((unsigned int *)&g_419098) = v22;
    g_419548 = *((int *)&g_419098) & 128;
    if ((char)*((int *)&g_419098) & 96)
    {
        if (!g_4dd004)
            fprintf(stderr, "\n%s: %s: warning, unknown flags 0x%x\n", g_4dd010, &g_4dd060, *((int *)&g_419098) & 96);
        if (!g_4dd018)
            g_4dd018 = 2;
    }
    *((unsigned int *)&g_419098) = *((int *)&g_419098) & 31;
    v16 = 1 << ((char)*((int *)&g_419098) & 63);
    if (*((int *)&g_419098) > 16)
    {
        fprintf(stderr, "\n%s: %s: compressed with %d bits, can only handle %d bits\n", g_4dd010, &g_4dd060, *((int *)&g_419098), 16);
        g_4dd018 = 1;
        return 1;
    }
    v5 = g_4dd868;
    v4 = 9;
    v15 = (1 << ((char)v4 & 63)) - 1;
    v3 = (1 << ((char)v4 & 31)) - 1;
    v12 = 0xffffffffffffffff;
    v0 = 0;
    iter1 = 0;
    v13 = g_4dd86c * 8;
    v14 = (!g_419548 ? 0x100 : 0x101);
    memset(&g_4bd000[0], 0, 0x100);
    for (node = 0xff; !(node - 0 >> 63 & 1); node -= 1)
    {
        (&g_4ad000)[node] = node & 0xffffffff;
    }
    while (1)
    {
        while (1)
        {
LABEL_40da53:
            v8 = (long long)(v13) >> 3;
            v9 = (v8 <= g_4dd868 ? g_4dd868 - v8 : 0);
            for (j = 0; j < v9; j += 1)
            {
                *(&(&g_41b000)[j]) = *(&(&g_41b000)[j + v8]);
            }
            g_4dd868 = v9;
            v13 = 0;
            if (g_4dd868 <= 63)
            {
                v5 = sub_40f194(a0, &(&g_41b000)[g_4dd868], 0x40000, &(&g_41b000)[g_4dd868]);
                if (v5 == 0xffffffff)
                    sub_40f6c7(); /* do not return */
                g_4dd868 = v5 + g_4dd868;
                g_4dd040 = g_4dd040 + v5;
            }
            k = (!v5 ? g_4dd868 * 8 - (v4 - 1) : (g_4dd868 - (g_4dd868 % v4 & 0xffffffff)) * 8);
            while (k > v13)
            {
                if (v14 > v15)
                {
                    v13 = v4 * 8 - (v4 * 8 + v13 - 1) % (v4 * 8) + v13 - 1;
                    v4 += 1;
                    v15 = (v4 == *((int *)&g_419098) ? v16 : (1 << ((char)v4 & 63)) - 1);
                    v3 = (unsigned int)(1 << ((char)v4 & 31)) - 1;
                    goto LABEL_40da53;
                }
                v18 = &(&g_41b000)[(long long)(v13) >> 3];
                node = v3 & (long long)(*(v18) | v18[1] * 0x100 | v18[2] * 0x10000) >> ((char)v13 & 7 & 63);
                v13 += v4;
                if (v12 == 0xffffffffffffffff)
                {
                    if (node > 0xff)
                        sub_40f5ef("corrupt input."); /* do not return */
                    v12 = node;
                    v0 = v12;
                    v1 = iter1 + 1;
                    *(iter1 + (char *)&g_45c000) = v0;
                    iter1 = v1;
                }
                else if (node == 0x100 && g_419548)
                {
                    memset(&g_4bd000[0], 0, 0x100);
                    v14 = 0x100;
                    v13 = v4 * 8 - (v4 * 8 + v13 - 1) % (v4 * 8) + v13 - 1;
                    v4 = 9;
                    v15 = (1 << ((char)v4 & 63)) - 1;
                    v3 = (unsigned int)(1 << ((char)v4 & 31)) - 1;
                    goto LABEL_40da53;
                }
                else
                {
                    v19 = node;
                    iter = &g_4ac7fe;
                    if (node >= v14)
                    {
                        if (node > v14)
                        {
                            if (iter1 > 0)
                                sub_40f321(a1, &g_45c000, iter1, &g_45c000);
                            sub_40f5ef((!g_4dd000 ? "corrupt input. Use zcat to recover some data." : "corrupt input.")); /* do not return */
                        }
                        iter -= 1;
                        *((char *)iter) = v0;
                        node = v12;
                    }
                    for (; node > 0xff; node = g_4bd000[node])
                    {
                        iter -= 1;
                        *((char *)iter) = (&g_4ad000)[node];
                    }
                    v0 = (&g_4ad000)[node];
                    iter -= 1;
                    *((char *)iter) = v0;
                    l = &g_4ac7fe - iter;
                    if (iter1 + l > 0x3ffff)
                    {
                        do
                        {
                            if (l > 0x40000 - iter1)
                                l = 0x40000 - iter1;
                            if (l > 0)
                            {
                                memcpy((char *)&g_45c000 + iter1, iter, l);
                                iter1 += l;
                            }
                            if (iter1 > 0x3ffff)
                            {
                                sub_40f321(a1, &g_45c000, iter1, &g_45c000);
                                iter1 = 0;
                            }
                            iter += l;
                            l = &g_4ac7fe - iter;
                        } while (l > 0);
                    }
                    else
                    {
                        memcpy((char *)&g_45c000 + iter1, iter, l);
                        iter1 += l;
                    }
                    node = v14;
                    if (node < v16)
                    {
                        g_4bd000[node] = v12 & 0xffffffff;
                        (&g_4ad000)[node] = v0;
                        v14 = node + 1;
                    }
                    v12 = v19;
                }
            }
            if (!v5)
            {
                if (iter1 <= 0)
                    return 0;
                sub_40f321(a1, &g_45c000, iter1, &g_45c000);
                return 0;
            }
            break;
        }
    }
}



// Function: read_byte @ 0xdff1
extern char g_41b000;
extern char g_4dd868;
extern unsigned int g_4dd86c;

int read_byte(void)
{
    unsigned int v2;  // eax
    unsigned int v3;  // eax
    unsigned int v0;  // [bp-0xc]

    if (g_4dd86c < *((int *)&g_4dd868))
    {
        v2 = g_4dd86c;
        g_4dd86c = v2 + 1;
        v3 = *(&(&g_41b000)[v2]);
    }
    else
    {
        v3 = sub_40f0bc(0);
    }
    v0 = v3;
    if (v0 >= 0)
        return v0;
    sub_40f5ef("invalid compressed data -- unexpected end of file"); /* do not return */
}



// Function: read_tree @ 0xe055
extern unsigned long long g_4e13e0;
extern unsigned int g_4e13e8;
extern char g_4e1400;
extern unsigned int g_4e1500;
extern unsigned int g_4e1580[4];

unsigned int * read_tree(void)
{
    char v6;  // al
    int node;  // [bp-0x28]
    int k;  // [bp-0x24]
    int v2;  // [bp-0x24]
    unsigned int *iter;  // [bp-0x20], Other Possible Types: int
    unsigned int v4;  // [bp-0x1c]

    v4 = 1;
    g_4e13e0 = 0;
    for (iter = 1; iter <= 4; iter += 1)
    {
        v6 = sub_40dff1();
        g_4e13e0 = v6 | g_4e13e0 * 0x100;
    }
    g_4e13e8 = sub_40dff1() & 0xff;
    if (g_4e13e8 > 0 && g_4e13e8 <= 25)
    {
        iter = NULL;
        for (node = 1; node <= g_4e13e8; node += 1)
        {
            g_4e1580[node] = sub_40dff1() & 0xff;
            if ((unsigned int)(v4 - (node == g_4e13e8)) < g_4e1580[node])
                sub_40f5ef("too many leaves in Huffman tree"); /* do not return */
            v4 = (v4 - g_4e1580[node] + 1) * 2 - 1;
            iter = (char *)iter + g_4e1580[node];
        }
        if (iter > 0xff)
            sub_40f5ef("too many leaves in Huffman tree"); /* do not return */
        g_4e1580[g_4e13e8] = g_4e1580[g_4e13e8] + 1;
        k = 0;
        for (node = 1; node <= g_4e13e8; node += 1)
        {
            (&g_4e1500)[node] = k;
            for (iter = (unsigned int *)g_4e1580[node]; iter > 0; k = v2)
            {
                v2 = k + 1;
                *(&(&g_4e1400)[k]) = sub_40dff1();
                iter = (char *)iter - 1;
            }
        }
        g_4e1580[g_4e13e8] = g_4e1580[g_4e13e8] + 1;
        return &g_4e1580[0];
    }
    sub_40f5ef("invalid compressed data -- Huffman code bit length out of range"); /* do not return */
}



// Function: unpack @ 0xe3fd
extern char g_45c000;
extern char g_4ad000;
extern unsigned long long g_4dd048;
extern unsigned int g_4dd860;
extern unsigned int g_4dd864;
extern unsigned int g_4dd870;
extern char g_4e13e0;
extern char g_4e13e8;
extern char g_4e1400;
extern unsigned int g_4e1500[4];
extern unsigned int g_4e1580[4];
extern unsigned int g_4e1600;
extern char g_4e1668;
extern unsigned long long g_4e1670;
extern void g_4e1678;

unsigned int unpack(unsigned int a0, unsigned int a1)
{
    char v5;  // al
    unsigned int j;  // ebx
    char v7;  // al
    unsigned int *v8;  // eax
    unsigned int v9;  // edx
    unsigned int v0;  // [bp-0x2c]
    unsigned int v1;  // [bp-0x28]
    unsigned int i;  // [bp-0x24]
    unsigned long k;  // [bp-0x20]

    g_4dd860 = a0;
    g_4dd864 = a1;
    sub_40e055();
    sub_40e29d();
    *((unsigned int *)&g_4e1678) = 0;
    g_4e1670 = 0;
    v1 = (unsigned int)(1 << ((char)*((int *)&g_4e1668) & 31)) - 1;
    while (1)
    {
        for (i = g_4e1580[*((int *)&g_4e13e8)] - 1; *((int *)&g_4e1678) < *((int *)&g_4e1668); *((unsigned int *)&g_4e1678) = *((int *)&g_4e1678) + 8)
        {
            v5 = sub_40dff1();
            g_4e1670 = v5 | g_4e1670 * 0x100;
        }
        j = (unsigned int)(g_4e1670 >> ((char)(*((int *)&g_4e1678) - *((int *)&g_4e1668) & 0xffffffff) & 63)) & v1;
        v0 = *(&(&g_45c000)[j]);
        if (v0 > 0)
        {
            j >>= (char)(*((int *)&g_4e1668) - v0) & 31;
        }
        else
        {
            k = v1;
            for (v0 = *((int *)&g_4e1668); j < (&g_4e1600)[v0]; j = (unsigned int)(g_4e1670 >> ((char)(*((int *)&g_4e1678) - v0) & 63)) & (unsigned int)k)
            {
                v0 += 1;
                for (k = k * 2 + 1; v0 > *((int *)&g_4e1678); *((unsigned int *)&g_4e1678) = *((int *)&g_4e1678) + 8)
                {
                    v7 = sub_40dff1();
                    g_4e1670 = v7 | g_4e1670 * 0x100;
                }
            }
        }
        if (j == i && v0 == *((int *)&g_4e13e8))
            break;
        v8 = (unsigned int *)g_4e1500[v0];
        v9 = g_4dd870;
        g_4dd870 = v9 + 1;
        *(&(&g_4ad000)[v9]) = (&g_4e1400)[j + (unsigned long long)v8 & 0xffffffff];
        if (g_4dd870 == 0x8000)
            sub_40f2cc();
        *((unsigned int *)&g_4e1678) = *((int *)&g_4e1678) - v0;
    }
    sub_40f2cc();
    if ((g_4dd048 & 0xffffffff) == *((long long *)&g_4e13e0))
        return 0;
    sub_40f5ef("invalid compressed data--length error"); /* do not return */
}



// Function: check_zipfile @ 0xe655
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char padding_4[2];
    char field_6;
    char padding_7[1];
    char field_8;
    char padding_9[17];
    char field_1a;
    char field_1b;
    char field_1c;
    char field_1d;
} struct_0;

typedef struct FILE {
} FILE;

extern unsigned int g_41909c;
extern FILE *stderr;
extern char g_41b000;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd060;
extern unsigned int g_4dd860;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4e1688;
extern unsigned int g_4e168c;
extern char g_4e1690;

unsigned long long check_zipfile(unsigned int a0)
{
    struct_0 *v0;  // [bp-0x10]

    v0 = &(&g_41b000)[g_4dd86c];
    g_4dd860 = a0;
    g_4dd86c = g_4dd86c + (v0->field_1d * 0x100 | v0->field_1c) + (v0->field_1b * 0x100 | v0->field_1a) + 30;
    if (g_4dd86c <= *((int *)&g_4dd868) && ((v0->field_3 * 0x100 | v0->field_2) * 0x10000 | v0->field_1 * 0x100 | v0->field_0) == 67324752)
    {
        g_41909c = v0->field_8;
        if (g_41909c && g_41909c != 8)
        {
            fprintf(stderr, "\n%s: %s: first entry not deflated or stored -- use unzip\n", g_4dd010, &g_4dd060);
            g_4dd018 = 1;
            return 1;
        }
        g_4e1688 = v0->field_6 & 1;
        if (!g_4e1688)
        {
            *((unsigned int *)&g_4e1690) = v0->field_6 & 8;
            g_4e168c = 1;
            return 0;
        }
        fprintf(stderr, "\n%s: %s: encrypted file -- use unzip\n", g_4dd010, &g_4dd060);
        g_4dd018 = 1;
        return 1;
    }
    fprintf(stderr, "\n%s: %s: not a valid zip file\n", g_4dd010, &g_4dd060);
    g_4dd018 = 1;
    return 1;
}



// Function: unzip @ 0xe86a
typedef struct FILE {
} FILE;

extern unsigned int g_41909c;
extern FILE *stderr;
extern char g_41b000;
extern char g_41b00e;
extern char g_41b00f;
extern char g_41b010;
extern char g_41b011;
extern char g_41b012;
extern char g_41b013;
extern char g_41b014;
extern char g_41b015;
extern char g_41b016;
extern char g_41b017;
extern char g_41b018;
extern char g_41b019;
extern long long g_45c000;
extern char g_4ad000;
extern unsigned int g_4dd000;
extern unsigned int g_4dd004;
extern unsigned int g_4dd008;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern unsigned long long g_4dd048;
extern char g_4dd060;
extern unsigned int g_4dd860;
extern unsigned int g_4dd864;
extern char g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;
extern unsigned long long g_4e1680;
extern unsigned int g_4e1688;
extern unsigned int g_4e168c;
extern unsigned int g_4e1690;

unsigned int unzip(unsigned int a0, unsigned int a1)
{
    unsigned long long v18;  // rbx
    unsigned long long v19;  // rax
    unsigned int v20;  // eax
    char v21;  // al
    unsigned int v22;  // eax
    unsigned int v23;  // eax
    char v24;  // al
    unsigned int v25;  // eax
    char v26;  // al
    char v27;  // al
    char v0;  // [bp-0x55]
    int iter;  // [bp-0x54]
    unsigned int v2;  // [bp-0x50]
    unsigned int v3;  // [bp-0x4c]
    unsigned long long v4;  // [bp-0x48]
    unsigned long long v5;  // [bp-0x40]
    char v6;  // [bp-0x38]
    char v7;  // [bp-0x36]
    char v8;  // [bp-0x35]
    char v9;  // [bp-0x34]
    char v10;  // [bp-0x33]
    char v11;  // [bp-0x32]
    char v12;  // [bp-0x31]
    char v13;  // [bp-0x2c]
    char v14;  // [bp-0x2b]
    char v15;  // [bp-0x2a]
    char v16;  // [bp-0x29]

    v4 = 0;
    v5 = 0;
    v2 = 0;
    g_4dd860 = a0;
    g_4dd864 = a1;
    sub_40efda(0, 0);
    if (g_4e168c && !g_4e1690)
    {
        v4 = (g_41b011 * 0x100 | g_41b010) * 0x10000 | g_41b00f * 0x100 | g_41b00e;
        v5 = (g_41b019 * 0x100 | g_41b018) * 0x10000 | g_41b017 * 0x100 | g_41b016;
    }
    if (g_41909c == 8)
    {
        v3 = sub_40ab98();
        switch (v3)
        {
        case 3:
            sub_40f634(); /* do not return */
        case 0:
LABEL_40eb57:
            if (!g_4e168c)
            {
                for (iter = 0; iter <= 7; iter += 1)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v23 = g_4dd86c;
                        g_4dd86c = v23 + 1;
                        v24 = *(&(&g_41b000)[v23]);
                    }
                    else
                    {
                        v24 = sub_40f0bc(0);
                    }
                    (&v6)[iter] = v24;
                }
                v4 = (v8 * 0x100 | v7) * 0x10000 | *(&(&v6)[1]) * 0x100 | *(&v6);
                v5 = (v12 * 0x100 | v11) * 0x10000 | v10 * 0x100 | v9;
                break;
            }
            else if (g_4e1690)
            {
                for (iter = 0; iter <= 15; iter += 1)
                {
                    if (g_4dd86c < *((int *)&g_4dd868))
                    {
                        v25 = g_4dd86c;
                        g_4dd86c = v25 + 1;
                        v26 = *(&(&g_41b000)[v25]);
                    }
                    else
                    {
                        v26 = sub_40f0bc(0);
                    }
                    (&v6)[iter] = v26;
                }
                v4 = (v12 * 0x100 | v11) * 0x10000 | v10 * 0x100 | v9;
                v5 = (v16 * 0x100 | v15) * 0x10000 | v14 * 0x100 | v13;
                break;
            }
            if (v4 != sub_40efda(&g_45c000, 0))
            {
                fprintf(stderr, "\n%s: %s: invalid compressed data--crc error\n", g_4dd010, &g_4dd060);
                v2 = 1;
            }
            if (v5 != (g_4dd048 & 0xffffffff))
            {
                fprintf(stderr, "\n%s: %s: invalid compressed data--length error\n", g_4dd010, &g_4dd060);
                v2 = 1;
            }
            if (g_4e168c && g_4dd86c + 4 < *((int *)&g_4dd868))
            {
                v27 = (&g_41b000)[3 + g_4dd86c];
                if (((v27 * 0x100 | (&g_41b000)[2 + g_4dd86c]) * 0x10000 | (&g_41b000)[1 + g_4dd86c] * 0x100 | (&g_41b000)[g_4dd86c]) == 67324752)
                {
                    if (g_4dd000)
                    {
                        if (!g_4dd004)
                            fprintf(stderr, "%s: %s has more than one entry--rest ignored\n", g_4dd010, &g_4dd060);
                        if (!g_4dd018)
                            g_4dd018 = 2;
                    }
                    else
                    {
                        fprintf(stderr, "%s: %s has more than one entry -- unchanged\n", g_4dd010, &g_4dd060);
                        v2 = 1;
                    }
                }
            }
            g_4e168c = 0;
            g_4e1690 = g_4e168c;
            g_4e1680 = v4;
            if (!v2)
                return 0;
            g_4dd018 = 1;
            if (g_4dd008)
                return v2;
            sub_408f9f(); /* do not return */
        default:
            sub_40f5ef("invalid compressed data--format violated"); /* do not return */
        }
    }
    if (!g_4e168c || g_41909c)
        sub_40f5ef("internal error, invalid method"); /* do not return */
    v18 = g_41b017 * 0x100 | g_41b016 | (g_41b019 * 0x100 | g_41b018) * 0x10000;
    if (v18 != (g_41b013 * 0x100 | g_41b012 | (g_41b015 * 0x100 | g_41b014) * 0x10000) - (!g_4e1688 ? 0 : 12))
    {
        fprintf(stderr, "len %lu, siz %lu\n", v18, g_41b013 * 0x100 | g_41b012 | (g_41b015 * 0x100 | g_41b014) * 0x10000);
        sub_40f5ef("invalid compressed data--length mismatch"); /* do not return */
    }
    while (1)
    {
        v19 = v18;
        v18 = v19 - 1;
        if (!v19)
            break;
        if (g_4dd86c < *((int *)&g_4dd868))
        {
            v20 = g_4dd86c;
            g_4dd86c = v20 + 1;
            v21 = *(&(&g_41b000)[v20]);
        }
        else
        {
            v21 = sub_40f0bc(0);
        }
        v0 = v21;
        v22 = g_4dd870;
        g_4dd870 = v22 + 1;
        *(&(&g_4ad000)[v22]) = v0;
        if (g_4dd870 == 0x8000)
            sub_40f2cc();
    }
    sub_40f2cc();
    goto LABEL_40eb57;
}



// Function: copy @ 0xef1d
extern long long g_41b000;
extern unsigned long long g_4dd040;
extern unsigned int g_4dd868;
extern unsigned int g_4dd86c;

unsigned int copy(unsigned int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0xc]

    for (*(__errno_location()) = 0; g_4dd868 > g_4dd86c; g_4dd86c = 0)
    {
        sub_40f321(a1, g_4dd86c + (char *)&g_41b000, g_4dd868 - g_4dd86c & 0xffffffff, g_4dd86c + (char *)&g_41b000);
        v0 = sub_40f194(a0, &g_41b000, 0x40000, &g_41b000);
        if (v0 == 0xffffffff)
            sub_40f6c7(); /* do not return */
        g_4dd040 = g_4dd040 + v0;
        g_4dd868 = v0;
    }
    return 0;
}



// Function: updcrc @ 0xefda
extern unsigned long long g_414680[4];
extern unsigned long long g_419550;

long long updcrc(char *a0, unsigned int a1)
{
    unsigned long long v3;  // rbx
    char *v4;  // rax
    unsigned int i;  // [bp-0x1c]
    char *v1;  // [bp-0x18]

    v1 = a0;
    i = a1;
    if (!v1)
    {
        v3 = 0xffffffff;
    }
    else
    {
        v3 = g_419550;
        if (i)
        {
            do
            {
                v4 = v1;
                v1 = v4 + 1;
                v3 = g_414680[*(v4) ^ (char)v3] ^ v3 >> 8;
                i -= 1;
            } while (i);
        }
    }
    g_419550 = v3;
    return 0xffffffff ^ v3;
}



// Function: getcrc @ 0xf05f
extern unsigned long long g_419550;

long long getcrc(void)
{
    return g_419550 ^ 0xffffffff;
}



// Function: clear_bufs @ 0xf078
extern unsigned long long g_4dd040;
extern unsigned long long g_4dd048;
extern unsigned int g_4dd868;
extern unsigned int g_4dd86c;
extern unsigned int g_4dd870;

long long clear_bufs(void)
{
    g_4dd870 = 0;
    g_4dd86c = 0;
    g_4dd868 = g_4dd86c;
    g_4dd048 = 0;
    g_4dd040 = g_4dd048;
    return g_4dd048;
}



// Function: fill_inbuf @ 0xf0bc
extern char g_41b000;
extern unsigned long long g_4dd040;
extern unsigned int g_4dd860;
extern unsigned int g_4dd868;
extern unsigned int g_4dd86c;

unsigned long long fill_inbuf(unsigned int a0)
{
    long long v2;  // rdi
    long long v3;  // rsi
    long long v4;  // rdx
    long long v5;  // rcx
    long long v6;  // r8
    long long v7;  // r9
    unsigned int v0;  // [bp-0xc]

    g_4dd868 = 0;
    do
    {
        v0 = sub_40f194(g_4dd860, &(&g_41b000)[g_4dd868], 0x40000 - g_4dd868, &(&g_41b000)[g_4dd868]);
        if (!v0)
            break;
        if (v0 == 0xffffffff)
            sub_40f6c7(); /* do not return */
        g_4dd868 = v0 + g_4dd868;
    } while (g_4dd868 <= 0x3ffff);
    if (g_4dd868)
    {
        g_4dd040 = g_4dd040 + g_4dd868;
        g_4dd86c = 1;
        return g_41b000;
    }
    else if (a0)
    {
        return 0xffffffff;
    }
    else
    {
        sub_40f2cc();
        *(__errno_location()) = 0;
        sub_40f6c7(v2, v3, v4, v5, v6, v7); /* do not return */
    }
}



// Function: read_buffer @ 0xf194
int read_buffer(unsigned int a0, void* a1, unsigned int a2)
{
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    v0 = a2;
    if (v0 < 0)
        v0 = 0x7fffffff;
    v1 = read(a0, a1, v0);
    if (v1 >= 0)
    {
        return v1;
    }
    else if (*(__errno_location()) == 11)
    {
        v2 = sub_4103bd(a0, 3);
        if (v2 < 0)
        {
            return v1;
        }
        else if (!((unsigned short)v2 & 0x800))
        {
            *(__errno_location()) = 11;
            return v1;
        }
        else if (sub_4103bd(a0, 4, _INSERT(v2, 1, (char)v2 & 247) & 0xffffffff) != 0xffffffff)
        {
            v1 = read(a0, a1, v0);
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



// Function: write_buffer @ 0xf254
long long write_buffer(int a0, void* a1, unsigned int a2)
{
    unsigned int v0;  // [bp-0x10]

    v0 = a2;
    if (v0 < 0)
        v0 = 0x7fffffff;
    return write(a0, a1, v0);
}



// Function: flush_outbuf @ 0xf28e
extern long long g_45c000;
extern unsigned int g_4dd864;
extern unsigned int g_4dd870;

unsigned long long flush_outbuf(void)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    v1 = g_4dd870;
    if (!(unsigned int)v1)
        return v1;
    v2 = sub_40f321(g_4dd864, &g_45c000, g_4dd870, &g_45c000);
    g_4dd870 = 0;
    return v2;
}



// Function: flush_window @ 0xf2cc
extern char g_4ad000;
extern unsigned int g_4dd864;
extern unsigned int g_4dd870;

unsigned long long flush_window(void)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    v1 = g_4dd870;
    if (!(unsigned int)v1)
        return v1;
    sub_40efda(&g_4ad000, g_4dd870);
    v2 = sub_40f321(g_4dd864, &g_4ad000, g_4dd870, &g_4ad000);
    g_4dd870 = 0;
    return v2;
}



// Function: write_buf @ 0xf321
extern unsigned int g_4dd008;
extern unsigned long long g_4dd048;

long long write_buf(int a0, void* a1, unsigned int a2)
{
    void* v0;  // [bp-0x28]
    unsigned int v1;  // [bp-0x20]
    unsigned int v2;  // [bp-0xc]

    v0 = a1;
    v1 = a2;
    g_4dd048 = g_4dd048 + v1;
    if (g_4dd008)
        return g_4dd008;
    while (1)
    {
        v2 = sub_40f254(a0, v0, v1);
        if (v2 == v1)
            return v2;
        if (v2 == 0xffffffff)
            sub_40f74d(); /* do not return */
        v1 -= v2;
        v0 += v2;
    }
}



// Function: strlwr @ 0xf393
char * strlwr(char *a0)
{
    unsigned long long *v2;  // rax
    char v3;  // al
    char *iter;  // [bp-0x10]

    for (iter = a0; *(iter); iter += 1)
    {
        v2 = __ctype_b_loc();
        v3 = (!(*((short *)(*(iter) * 2 + *(v2))) & 0x100) ? *(iter) : (char)tolower(*(iter)));
        *(iter) = v3;
    }
    return a0;
}



// Function: gzip_base_name @ 0xf40a
unsigned long long gzip_base_name(long long a0)
{
    long long v0;  // [bp-0x10], Other Possible Types: unsigned long long

    v0 = a0;
    v0 = sub_410244(v0);
    return v0;
}



// Function: xunlink @ 0xf430
int xunlink(char *a0)
{
    unsigned int v0;  // [bp-0xc]

    v0 = unlink(a0);
    return v0;
}



// Function: add_envopt @ 0xf454
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_1 *field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

char * add_envopt(unsigned int *a0, struct_0 **a1, char *a2)
{
    unsigned long v7;  // rdx
    unsigned int node;  // [bp-0x2c]
    char *iter;  // [bp-0x28]
    struct_0 *j;  // [bp-0x20]
    struct_0 *v3;  // [bp-0x20]
    char *v4;  // [bp-0x18], Other Possible Types: unsigned long
    struct_0 *v5;  // [bp-0x10]

    node = 0;
    v4 = getenv(a2);
    if (!v4)
        return NULL;
    v4 = sub_412377(v4);
    for (iter = v4; *(iter) && (iter += strspn(iter, " \t"), *(iter)); node += 1)
    {
        iter = &iter[strcspn(iter, " \t")];
        if (*(iter))
        {
            iter += 1;
            *(iter) = 0;
        }
    }
    if (!node)
    {
        free(v4);
        return NULL;
    }
    v7 = node + 1;
    *(a0) = v7;
    v3 = sub_41224e(*(a0) + 1, 8, v7);
    v5 = *(a1);
    *(a1) = v3;
    j = &v3->field_8;
    v3->field_0 = v5->field_0;
    for (iter = v4; node > 0; j = v3)
    {
        iter = &iter[strspn(iter, " \t")];
        v3 = &j->field_8;
        j->field_0 = iter;
        do
        {
            iter += 1;
        } while (*(iter));
        node -= 1;
    }
    j->field_0 = NULL;
    return v4;
}



// Function: gzip_error @ 0xf5ef
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;
extern char g_4dd060;

void gzip_error(char *a0)
{
    fprintf(stderr, "\n%s: %s: %s\n", g_4dd010, &g_4dd060, a0);
    sub_408f9f(); /* do not return */
}



// Function: xalloc_die @ 0xf634
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;

void xalloc_die(void)
{
    fprintf(stderr, "\n%s: memory_exhausted\n", g_4dd010);
    sub_408f9f(); /* do not return */
}



// Function: warning @ 0xf666
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern unsigned int g_4dd004;
extern char *g_4dd010;
extern unsigned int g_4dd018;
extern char g_4dd060;

long long warning(char *a0)
{
    unsigned long v1;  // rax

    if (!g_4dd004)
        fprintf(stderr, "%s: %s: warning: %s\n", g_4dd010, &g_4dd060, a0);
    v1 = g_4dd018;
    if (!(unsigned int)v1)
        g_4dd018 = 2;
    return v1;
}



// Function: read_error @ 0xf6c7
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;
extern char g_4dd060;

void read_error(void)
{
    unsigned int v0;  // [bp-0xc]

    v0 = *(__errno_location());
    fprintf(stderr, "\n%s: ", g_4dd010);
    if (v0)
    {
        *(__errno_location()) = v0;
        perror(&g_4dd060);
    }
    else
    {
        fprintf(stderr, "%s: unexpected end of file\n", &g_4dd060);
    }
    sub_408f9f(); /* do not return */
}



// Function: write_error @ 0xf74d
typedef struct FILE {
} FILE;

extern FILE *stderr;
extern char *g_4dd010;
extern char g_4dd460;

void write_error(void)
{
    unsigned int v0;  // [bp-0xc]

    v0 = *(__errno_location());
    fprintf(stderr, "\n%s: ", g_4dd010);
    *(__errno_location()) = v0;
    perror(&g_4dd460);
    sub_408f9f(); /* do not return */
}



// Function: display_ratio @ 0xf7a6
typedef struct FILE {
} FILE;

int display_ratio(unsigned long long a0, unsigned long long a1, FILE *a2)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    return fprintf(a2, "%5.1f%%");
}



// Function: fprint_off @ 0xf81a
typedef struct FILE {
} FILE;

void fprint_off(FILE *a0, uint128_t a1, unsigned int a2)
{
    unsigned int v0;  // [bp-0x7c]
    uint128_t i;  // [bp-0x78]
    void* iter;  // [bp-0x60]
    char v4;  // [bp-0x1b]

    i = a1;
    v0 = a2;
    iter = &v4 - 3;
    if (i - 0 >> 63 & 1)
    {
        do
        {
            iter -= 1;
            *((char *)iter) = 48 - (unsigned int)(i - ((unsigned long long)((int128_t)(i * 7378697629483820647) >> 66) - ((int128_t)(i) >> 63)) * 10);
            i = (unsigned long long)((int128_t)(i * 7378697629483820647) >> 66) - ((int128_t)(i) >> 63);
        } while (i);
        iter -= 1;
        *((char *)iter) = 45;
    }
    else
    {
        do
        {
            iter -= 1;
            *((char *)iter) = (char)(i - ((unsigned long long)((int128_t)(i * 7378697629483820647) >> 66) - ((int128_t)(i) >> 63)) * 10) + 48;
            i = (unsigned long long)((int128_t)(i * 7378697629483820647) >> 66) - ((int128_t)(i) >> 63);
        } while (i);
    }
    v0 -= &v4 - 3 - iter;
    while (1)
    {
        v0 -= 1;
        if (v0 <= 0)
            break;
        putc(32, a0);
    }
    for (; iter < &v4 - 3; iter += 1)
    {
        putc(*((char *)iter), a0);
    }
    return;
}



// Function: zip @ 0xf9dd
typedef struct FILE {
} FILE;

extern unsigned int g_41909c;
extern int g_4190a0;
extern FILE *stderr;
extern char g_45c000;
extern char *g_4dd010;
extern char g_4dd01c;
extern unsigned long long g_4dd020;
extern void g_4dd030;
extern long long g_4dd038;
extern unsigned long long g_4dd040;
extern char g_4dd060;
extern unsigned int g_4dd860;
extern unsigned int g_4dd864;
extern unsigned int g_4dd870;
extern unsigned long long g_4e1698;

unsigned int zip(unsigned int a0, unsigned int a1)
{
    unsigned int v7;  // eax
    unsigned int v8;  // eax
    unsigned int v17;  // eax
    unsigned int v18;  // eax
    unsigned int v19;  // eax
    unsigned int v20;  // eax
    char v21;  // cl
    unsigned int v22;  // eax
    char v23;  // al
    unsigned int v24;  // eax
    long long v25;  // rax
    unsigned int v26;  // eax
    unsigned int v9;  // eax
    char v27;  // al
    unsigned int v28;  // eax
    long long v29;  // rax
    unsigned int v30;  // eax
    unsigned long v31;  // rax
    unsigned int v32;  // eax
    unsigned long v33;  // rax
    unsigned int v34;  // eax
    unsigned long v35;  // rax
    unsigned int v36;  // eax
    unsigned int v10;  // eax
    unsigned long v37;  // rax
    unsigned int v38;  // eax
    unsigned int v39;  // eax
    char v40;  // al
    unsigned int v41;  // eax
    unsigned int v42;  // eax
    char v43;  // al
    unsigned int v44;  // eax
    char v45;  // al
    unsigned int v46;  // eax
    unsigned int v11;  // eax
    unsigned long long v47;  // rax
    unsigned int v48;  // eax
    char v49;  // al
    unsigned int v50;  // eax
    char v51;  // al
    unsigned int v52;  // eax
    unsigned int v12;  // eax
    unsigned int v13;  // eax
    unsigned int v14;  // eax
    unsigned int v15;  // eax
    unsigned int v16;  // eax
    char v0;  // [bp-0x25]
    unsigned short v1;  // [bp-0x24]
    unsigned short v2;  // [bp-0x22]
    unsigned long long v3;  // [bp-0x20]
    char *v4;  // [bp-0x18]
    char *v5;  // [bp-0x18]

    v0 = 0;
    v1 = 0;
    v2 = 0;
    g_4dd860 = a0;
    g_4dd864 = a1;
    g_4dd870 = 0;
    g_41909c = 8;
    v7 = g_4dd870;
    g_4dd870 = v7 + 1;
    *(&(&g_45c000)[v7]) = 31;
    if (g_4dd870 == 0x40000)
        sub_40f28e();
    v8 = g_4dd870;
    g_4dd870 = v8 + 1;
    *(&(&g_45c000)[v8]) = 139;
    if (g_4dd870 == 0x40000)
        sub_40f28e();
    v9 = g_4dd870;
    g_4dd870 = v9 + 1;
    *(&(&g_45c000)[v9]) = 8;
    if (g_4dd870 == 0x40000)
        sub_40f28e();
    if (*((int *)&g_4dd01c))
        v0 |= 8;
    v10 = g_4dd870;
    g_4dd870 = v10 + 1;
    *(&(&g_45c000)[v10]) = v0;
    if (g_4dd870 == 0x40000)
        sub_40f28e();
    if (g_4dd038 < 0)
    {
        v3 = 0;
    }
    else if (*((long long *)&g_4dd030) > 0 && *((long long *)&g_4dd030) <= 0xffffffff)
    {
        v3 = *((long long *)&g_4dd030);
    }
    else
    {
        sub_40f666("file timestamp out of range for gzip format");
        v3 = 0;
    }
    if (g_4dd870 <= 0x3fffd)
    {
        v11 = g_4dd870;
        g_4dd870 = v11 + 1;
        *(&(&g_45c000)[v11]) = v3 & 0xffffffff;
        v12 = g_4dd870;
        g_4dd870 = v12 + 1;
        *(&(&g_45c000)[v12]) = _INSERT(v3, 0, (unsigned short)v3 >> 8) & 0xffffffff;
    }
    else
    {
        v13 = g_4dd870;
        g_4dd870 = v13 + 1;
        *(&(&g_45c000)[v13]) = v3 & 0xffffffff;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v14 = g_4dd870;
        g_4dd870 = v14 + 1;
        *(&(&g_45c000)[v14]) = _INSERT(v3, 0, (unsigned short)v3 >> 8) & 0xffffffff;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    if (g_4dd870 <= 0x3fffd)
    {
        v15 = g_4dd870;
        g_4dd870 = v15 + 1;
        *(&(&g_45c000)[v15]) = v3 >> 16;
        v16 = g_4dd870;
        g_4dd870 = v16 + 1;
        *(&(&g_45c000)[v16]) = _INSERT(v3 >> 16, 0, (unsigned short)(v3 >> 16) >> 8) & 0xffffffff;
    }
    else
    {
        v17 = g_4dd870;
        g_4dd870 = v17 + 1;
        *(&(&g_45c000)[v17]) = v3 >> 16;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v18 = g_4dd870;
        g_4dd870 = v18 + 1;
        *(&(&g_45c000)[v18]) = _INSERT(v3 >> 16, 0, (unsigned short)(v3 >> 16) >> 8) & 0xffffffff;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    sub_40efda(NULL, 0);
    sub_403a29(a1);
    sub_40ac6f(&v1, &g_41909c);
    if (g_4190a0 == 1)
    {
        v2 |= 4;
    }
    else if (g_4190a0 == 9)
    {
        v2 |= 2;
    }
    v19 = g_4dd870;
    g_4dd870 = v19 + 1;
    *(&(&g_45c000)[v19]) = v2;
    if (g_4dd870 == 0x40000)
        sub_40f28e();
    v20 = g_4dd870;
    g_4dd870 = v20 + 1;
    *(&(&g_45c000)[v20]) = 3;
    if (g_4dd870 == 0x40000)
        sub_40f28e();
    if (*((int *)&g_4dd01c))
    {
        v4 = sub_40f40a(&g_4dd060);
        do
        {
            v5 = v4;
            v21 = *(v5);
            v22 = g_4dd870;
            g_4dd870 = v22 + 1;
            *(&(&g_45c000)[v22]) = v21;
            if (g_4dd870 == 0x40000)
                sub_40f28e();
        } while ((v4 = v5 + 1, *(v5)));
    }
    g_4e1698 = g_4dd870;
    sub_404a9e(g_4190a0);
    if (g_4dd020 != 0xffffffffffffffff && g_4dd040 != g_4dd020)
        fprintf(stderr, "%s: %s: file size changed while zipping\n", g_4dd010, &g_4dd060);
    if (g_4dd870 <= 0x3fffd)
    {
        v23 = sub_40f05f();
        v24 = g_4dd870;
        g_4dd870 = v24 + 1;
        *(&(&g_45c000)[v24]) = v23;
        v25 = sub_40f05f();
        v26 = g_4dd870;
        g_4dd870 = v26 + 1;
        *(&(&g_45c000)[v26]) = _INSERT(v25, 0, (unsigned short)v25 >> 8) & 0xffffffff;
    }
    else
    {
        v27 = sub_40f05f();
        v28 = g_4dd870;
        g_4dd870 = v28 + 1;
        *(&(&g_45c000)[v28]) = v27;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v29 = sub_40f05f();
        v30 = g_4dd870;
        g_4dd870 = v30 + 1;
        *(&(&g_45c000)[v30]) = _INSERT(v29, 0, (unsigned short)v29 >> 8) & 0xffffffff;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    if (g_4dd870 <= 0x3fffd)
    {
        v31 = sub_40f05f();
        v32 = g_4dd870;
        g_4dd870 = v32 + 1;
        *(&(&g_45c000)[v32]) = v31 >> 16;
        v33 = sub_40f05f();
        v34 = g_4dd870;
        g_4dd870 = v34 + 1;
        *(&(&g_45c000)[v34]) = _INSERT(v33 >> 16, 0, (unsigned short)(v33 >> 16) >> 8) & 0xffffffff;
    }
    else
    {
        v35 = sub_40f05f();
        v36 = g_4dd870;
        g_4dd870 = v36 + 1;
        *(&(&g_45c000)[v36]) = v35 >> 16;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v37 = sub_40f05f();
        v38 = g_4dd870;
        g_4dd870 = v38 + 1;
        *(&(&g_45c000)[v38]) = _INSERT(v37 >> 16, 0, (unsigned short)(v37 >> 16) >> 8) & 0xffffffff;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    if (g_4dd870 <= 0x3fffd)
    {
        v39 = g_4dd870;
        g_4dd870 = v39 + 1;
        *(&(&g_45c000)[v39]) = g_4dd040;
        v40 = (unsigned short)g_4dd040 >> 8;
        v41 = g_4dd870;
        g_4dd870 = v41 + 1;
        *(&(&g_45c000)[v41]) = v40;
    }
    else
    {
        v42 = g_4dd870;
        g_4dd870 = v42 + 1;
        *(&(&g_45c000)[v42]) = g_4dd040;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v43 = (unsigned short)g_4dd040 >> 8;
        v44 = g_4dd870;
        g_4dd870 = v44 + 1;
        *(&(&g_45c000)[v44]) = v43;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    if (g_4dd870 <= 0x3fffd)
    {
        v45 = g_4dd040 >> 16;
        v46 = g_4dd870;
        g_4dd870 = v46 + 1;
        *(&(&g_45c000)[v46]) = v45;
        v47 = g_4dd040 >> 16;
        v48 = g_4dd870;
        g_4dd870 = v48 + 1;
        *(&(&g_45c000)[v48]) = _INSERT(v47, 0, (unsigned short)v47 >> 8) & 0xffffffff;
    }
    else
    {
        v49 = g_4dd040 >> 16;
        v50 = g_4dd870;
        g_4dd870 = v50 + 1;
        *(&(&g_45c000)[v50]) = v49;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
        v51 = (unsigned short)(g_4dd040 >> 16) >> 8;
        v52 = g_4dd870;
        g_4dd870 = v52 + 1;
        *(&(&g_45c000)[v52]) = v51;
        if (g_4dd870 == 0x40000)
            sub_40f28e();
    }
    g_4e1698 = g_4e1698 + 8;
    sub_40f28e();
    return 0;
}



// Function: file_read @ 0x101d7
extern unsigned long long g_4dd040;
extern unsigned int g_4dd860;

int file_read(void* a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0xc]

    v0 = sub_40f194(g_4dd860, a0, a1);
    if (!v0)
    {
        return v0;
    }
    else if (v0 == 0xffffffff)
    {
        sub_40f6c7(); /* do not return */
    }
    else
    {
        sub_40efda(a0, v0);
        g_4dd040 = g_4dd040 + v0;
        return v0;
    }
}



// Function: last_component @ 0x10244
char * last_component(char *a0)
{
    char v0;  // [bp-0x19]
    char *iter;  // [bp-0x18]
    char *node;  // [bp-0x10]

    iter = a0;
    for (v0 = 0; *(iter) == 47; iter += 1);
    for (node = iter; *(node); node += 1)
    {
        if (*(node) == 47)
        {
            v0 = 1;
        }
        else if (v0)
        {
            iter = node;
            v0 = 0;
        }
    }
    return iter;
}



// Function: base_len @ 0x102b1
long long base_len(char *a0)
{
    unsigned long len;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v1 = 0;
    for (len = strlen(a0); len > 1; len -= 1)
    {
        if (*(&a0[len] - 1) != 47)
            return len;
    }
    return len;
}



// Function: rpl_fcntl @ 0x103bd
long long rpl_fcntl(unsigned int a0, unsigned int a1)
{
    unsigned long v13;  // rdx
    unsigned long v14;  // rcx
    void* v23;  // rax
    void* v24;  // rax
    unsigned long v15;  // r8
    unsigned long v16;  // r9
    unsigned long v18;  // fs
    void* v19;  // rax
    unsigned int v20;  // eax
    unsigned int v21;  // eax
    void* v22;  // rax
    unsigned int v0;  // [bp-0xf0]
    unsigned int v1;  // [bp-0xec]
    unsigned int v2;  // [bp-0xe8]
    unsigned int v3;  // [bp-0xe4]
    unsigned long v4;  // [bp-0xe0]
    unsigned int iter;  // [bp-0xd8]
    unsigned int v6;  // [bp-0xd4]
    void* v7;  // [bp-0xd0]
    char *v8;  // [bp-0xc8]
    unsigned long v9;  // [bp-0xc0]
    int v10;  // [bp-0xb8]
    unsigned long v11;  // [bp+0x8]

    *((unsigned long *)&(&v10)[16]) = v13;
    *((unsigned long *)&(&v10)[24]) = v14;
    *((unsigned long *)&(&v10)[32]) = v15;
    *((unsigned long *)&(&v10)[40]) = v16;
    v9 = *((long long *)(40 + v18));
    v0 = 0xffffffff;
    iter = 16;
    v6 = 48;
    v7 = &v11;
    v8 = &v10;
    if (!a1)
    {
        if (iter <= 47)
        {
            v24 = &v8[iter];
            iter += 8;
        }
        else
        {
            v24 = v7;
            v7 = v24 + 8;
        }
        v2 = *((int *)v24);
        v0 = sub_41072f(a0, v2, v2);
    }
    else if (a1 == 1030)
    {
        if (iter <= 47)
        {
            v19 = &v8[iter];
            iter += 8;
        }
        else
        {
            v19 = v7;
            v7 = v19 + 8;
        }
        v1 = *((int *)v19);
        v0 = sub_410760(a0, v1, v1);
    }
    else
    {
        if (a1 <= 11)
        {
            if (a1 < 0)
                goto LABEL_41069d;
            v20 = 1 << ((char)a1 & 63);
            if (!((unsigned short)v20 & 1301))
            {
                if (!(v20 & 0xa0a))
                    goto LABEL_41069d;
LABEL_410608:
                v0 = fcntl(a0, a1);
            }
            else
            {
LABEL_41062d:
                if (iter <= 47)
                {
                    v22 = &v8[iter];
                    iter += 8;
                }
                else
                {
                    v22 = v7;
                    v7 = v22 + 8;
                }
                v3 = *((int *)v22);
                v0 = fcntl(a0, a1);
            }
        }
        else
        {
            if (a1 <= 1034 && a1 >= 0x400)
            {
                v21 = 1 << ((char)(a1 - 0x400) & 63);
                if (v21 & 709)
                    goto LABEL_41062d;
                if (v21 & 1282)
                    goto LABEL_410608;
            }
LABEL_41069d:
            if (iter <= 47)
            {
                v23 = &v8[iter];
                iter += 8;
            }
            else
            {
                v23 = v7;
                v7 = v23 + 8;
            }
            v4 = *((long long *)v23);
            v0 = fcntl(a0, a1);
        }
    }
    if (v9 == *((long long *)(40 + v18)))
        return v0;
    __stack_chk_fail(); /* do not return */
}



// Function: rpl_fcntl_DUPFD @ 0x1072f
int rpl_fcntl_DUPFD(int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x20]
    unsigned int v1;  // [bp-0xc]

    v0 = a1;
    v1 = fcntl(a0, 0);
    return v1;
}



// Function: rpl_fcntl_DUPFD_CLOEXEC @ 0x10760
extern int g_4e16a0;

unsigned int rpl_fcntl_DUPFD_CLOEXEC(int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x14]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    if (g_4e16a0 >= 0)
    {
        v0 = fcntl(a0, 1030);
        if (v0 >= 0 || *(__errno_location()) != 22)
        {
            g_4e16a0 = 1;
        }
        else
        {
            v0 = sub_41072f(a0, a1);
            if (v0 >= 0)
                g_4e16a0 = 0xffffffff;
        }
    }
    else
    {
        v0 = sub_41072f(a0, a1);
    }
    if (v0 < 0)
    {
        return v0;
    }
    else if (g_4e16a0 == -0x1)
    {
        v1 = fcntl(v0, 1);
        if (v1 >= 0 && fcntl(v0, 2) != -0x1)
            return v0;
        v2 = *(__errno_location());
        close(v0);
        *(__errno_location()) = v2;
        v0 = 0xffffffff;
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: open_safer @ 0x10863
int open_safer(char *a0, int a1, unsigned long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v13;  // fs
    unsigned int *v14;  // rax
    unsigned int v15;  // eax
    unsigned int v0;  // [bp-0xdc]
    unsigned int v1;  // [bp-0xd8]
    unsigned int v2;  // [bp-0xd4]
    unsigned int *v3;  // [bp-0xd0]
    char *v4;  // [bp-0xc8]
    unsigned long v5;  // [bp-0xc0]
    int v6;  // [bp-0xb8]
    unsigned long long v7;  // [bp-0xa0]
    unsigned long long v8;  // [bp-0x98]
    unsigned long long v9;  // [bp-0x90]
    unsigned int v10;  // [bp+0x8]

    *((unsigned long *)&(&v6)[16]) = a2;
    v7 = a3;
    v8 = a4;
    v9 = a5;
    v5 = *((long long *)(40 + v13));
    v0 = 0;
    if ((char)a1 & 64)
    {
        v1 = 16;
        v2 = 48;
        v3 = &v10;
        v4 = &v6;
        if (v1 <= 47)
        {
            v14 = &v4[v1];
            v1 += 8;
        }
        else
        {
            v14 = v3;
            v3 = v14 + 2;
        }
        v0 = *(v14);
    }
    v15 = sub_4110e3(open(a0, a1, v0) & 0xffffffff);
    if (v5 == *((long long *)(40 + v13)))
        return v15;
    __stack_chk_fail(); /* do not return */
}



// Function: clear_ungetc_buffer_preserving_position @ 0x1099f
unsigned long long clear_ungetc_buffer_preserving_position(unsigned int *a0)
{
    unsigned long long v1;  // rax

    v1 = *(a0) & 0x100;
    if ((unsigned int)v1)
        v1 = sub_410a26(a0, 0, 1);
    return v1;
}



// Function: rpl_fseeko @ 0x10a26
typedef struct FILE {
} FILE;

unsigned int rpl_fseeko(FILE *a0, int a1, int a2)
{
    unsigned long long v0;  // [bp-0x10]

    if (*((long long *)&a0[16]) == *((long long *)&a0[8]) && *((long long *)&a0[40]) == *((long long *)&a0[32]) && !*((long long *)&a0[72]))
    {
        v0 = lseek(fileno(a0), a1, a2);
        if (v0 != 0xffffffffffffffff)
        {
            *((unsigned int *)&a0) = *((int *)&a0) & 0xffffffef;
            *((unsigned long long *)&a0[144]) = v0;
            return 0;
        }
        return 0xffffffff;
    }
    return fseeko(a0, a1, a2);
}



// Function: openat_safer @ 0x10ae4
int openat_safer(unsigned int a0, long long a1, unsigned int a2, unsigned long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v12;  // fs
    unsigned int *v13;  // rax
    unsigned int v14;  // eax
    unsigned int v0;  // [bp-0xdc]
    unsigned int v1;  // [bp-0xd8]
    unsigned int v2;  // [bp-0xd4]
    unsigned int *v3;  // [bp-0xd0]
    char *v4;  // [bp-0xc8]
    unsigned long v5;  // [bp-0xc0]
    int v6;  // [bp-0xb8]
    unsigned long long v7;  // [bp-0x98]
    unsigned long long v8;  // [bp-0x90]
    unsigned int v9;  // [bp+0x8]

    *((unsigned long *)&(&v6)[24]) = a3;
    v7 = a4;
    v8 = a5;
    v5 = *((long long *)(40 + v12));
    v0 = 0;
    if ((char)a2 & 64)
    {
        v1 = 24;
        v2 = 48;
        v3 = &v9;
        v4 = &v6;
        if (v1 <= 47)
        {
            v13 = &v4[v1];
            v1 += 8;
        }
        else
        {
            v13 = v3;
            v3 = v13 + 2;
        }
        v0 = *(v13);
    }
    v14 = sub_4110e3(openat(a0, a1, a2, v0) & 0xffffffff);
    if (v5 == *((long long *)(40 + v12)))
        return v14;
    __stack_chk_fail(); /* do not return */
}



// Function: direntry_cmp_name @ 0x10c22
int direntry_cmp_name(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: streamsavedir @ 0x10c61
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

extern unsigned long long g_418b70[4];

long long streamsavedir(DIR *a0, unsigned int a1)
{
    unsigned long v0;  // [bp-0x80]
    unsigned long v1;  // [bp-0x78]
    unsigned long v2;  // [bp-0x70]
    unsigned long ptr;  // [bp-0x68]
    unsigned long idx;  // [bp-0x60]
    unsigned long iter;  // [bp-0x58]
    unsigned long i;  // [bp-0x50]
    unsigned long long *v7;  // [bp-0x48]
    dirent *count;  // [bp-0x40]
    dirent *v9;  // [bp-0x38]
    unsigned long n;  // [bp-0x30]
    char *v11;  // [bp-0x28]

    v2 = 0;
    v0 = 0;
    ptr = 0;
    v1 = 0;
    idx = 0;
    iter = 0;
    v7 = g_418b70[a1];
    if (!a0)
        return 0;
    while (1)
    {
        *(__errno_location()) = 0;
        count = readdir(a0);
        if (!count)
            break;
        v9 = (char *)&count[1].d_ino + 6;
        if (*((char *)v9 + ((char)v9->d_ino == 46 ? (*((char *)&v9->d_ino + 1) == 46 ? 2 : 1) : 0)))
        {
            n = strlen((char *)&count[1].d_ino + 6) + 1;
            if (v7)
            {
                if (idx == v1)
                    ptr = sub_41204c(ptr, &v1, 1, -0x1, 8);
                *((unsigned long long *)(idx * 8 + ptr)) = sub_412377(v9);
                idx += 1;
            }
            else
            {
                if (n >= v0 - iter)
                    v2 = sub_41204c(v2, &v0, n - (v0 - iter), 0x7ffffffffffffffe, 1);
                memcpy(iter + v2, v9, n);
            }
            iter += n;
        }
    }
    if (*(__errno_location()))
    {
        free(ptr);
        free(v2);
        return 0;
    }
    else if (v7)
    {
        if (idx)
            qsort(ptr, idx, 8, v7);
        v2 = sub_411d96(iter + 1);
        iter = 0;
        for (i = 0; i < idx; i += 1)
        {
            v11 = v2 + iter;
            iter += stpcpy(v11, *((long long *)(ptr + i * 8))) - v11 + 1;
            free(*((long long *)(ptr + i * 8)));
        }
        free(ptr);
    }
    else if (iter == v0)
    {
        v2 = sub_411e25(v2, iter + 1, iter + 1);
    }
    *((char *)(v2 + iter)) = 0;
    return v2;
}



// Function: savedir @ 0x10f97
typedef struct DIR {
} DIR;

long long savedir(long long a0, unsigned int a1)
{
    DIR *v0;  // [bp-0x18]
    unsigned long v1;  // [bp-0x10]

    v0 = sub_4123f7(a0);
    if (!v0)
        return 0;
    v1 = sub_410c61(v0, a1);
    if (!closedir(v0))
        return v1;
    free(v1);
    return 0;
}



// Function: fd_safer @ 0x110e3
int fd_safer(unsigned int a0)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned int v1;  // [bp-0x10]
    unsigned int v2;  // [bp-0xc]

    v0 = a0;
    if (v0 < 0)
    {
        return v0;
    }
    else if (v0 <= 2)
    {
        v1 = sub_412614(v0);
        v2 = *(__errno_location());
        close(v0);
        *(__errno_location()) = v2;
        v0 = v1;
        return v0;
    }
    else
    {
        return v0;
    }
}



// Function: validate_timespec @ 0x11134
typedef struct struct_0 {
    unsigned long long field_0;
    long long field_8;
    unsigned long long field_10;
    long long field_18;
} struct_0;

unsigned int validate_timespec(struct_0 *a0)
{
    unsigned int v0;  // [bp-0x10]
    unsigned int iter;  // [bp-0xc]

    v0 = 0;
    iter = 0;
    if ((a0->field_8 == 0x3fffffff || a0->field_8 == 0x3ffffffe || a0->field_8 >= 0 && a0->field_8 <= 0x3b9ac9ff) && (a0->field_18 == 0x3fffffff || a0->field_18 == 0x3ffffffe || a0->field_18 >= 0 && a0->field_18 <= 0x3b9ac9ff))
    {
        if (a0->field_8 == 0x3fffffff || a0->field_8 == 0x3ffffffe)
        {
            a0->field_0 = 0;
            v0 = 1;
            if (a0->field_8 == 0x3ffffffe)
                iter += 1;
        }
        if (a0->field_18 == 0x3fffffff || a0->field_18 == 0x3ffffffe)
        {
            a0->field_10 = 0;
            v0 = 1;
            if (a0->field_18 == 0x3ffffffe)
                iter += 1;
        }
        return v0 + (iter == 1);
    }
    *(__errno_location()) = 22;
    return 0xffffffff;
}



// Function: update_timespec @ 0x1129e
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

unsigned int update_timespec(long long a0, struct_0 **a1, unsigned long a2)
{
    unsigned long long v2;  // rdx
    unsigned long v3;  // rbx
    unsigned long long *v0;  // [bp-0x20]

    v0 = &*(a1)->field_0;
    if (v0[1] == 0x3ffffffe && v0[3] == 0x3ffffffe)
        return 1;
    if (v0[1] == 0x3fffffff && v0[3] == 0x3fffffff)
    {
        *(a1) = NULL;
        return 0;
    }
    if (v0[1] == 0x3ffffffe)
    {
        *(v0) = sub_41105b(a0);
        v0[1] = v2;
    }
    else if (v0[1] == 0x3fffffff)
    {
        sub_4124c4(v0);
    }
    if (v0[3] == 0x3ffffffe)
    {
        v3 = v0 + 2;
        *((unsigned long long *)v3) = sub_41108f(a0);
        *((unsigned long long *)(v3 + 8)) = v2;
    }
    else if (v0[3] == 0x3fffffff)
    {
        sub_4124c4(v0 + 2);
    }
    return 0;
}



// Function: fdutimens @ 0x113ca
typedef struct struct_0 {
    char field_0[8];
    long long field_8;
    unsigned long long field_10;
    long long field_18;
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

extern int g_4e16a4;
extern unsigned int g_4e16a8;

unsigned long long fdutimens(unsigned int a0, unsigned long a1, unsigned long long *a2)
{
    unsigned long long *v24;  // rax
    char v26;  // al
    unsigned long v28;  // rdx
    unsigned long long *v29;  // rbx
    char v31;  // al
    char v0;  // [bp-0x14a]
    char v1;  // [bp-0x149]
    int v2;  // [bp-0x148]
    unsigned int v3;  // [bp-0x144]
    unsigned long long *idx;  // [bp-0x140]
    struct_0 *idx1;  // [bp-0x138]
    char *index;  // [bp-0x130]
    unsigned long long v7;  // [bp-0x128]
    unsigned long long v8;  // [bp-0x120]
    int <0x4113ca[is_1]|Stack bp-0x118, 1 B>;  // [bp-0x118]
    unsigned long v9;  // [bp-0x118]
    unsigned long v10;  // [bp-0x110]
    unsigned long v11;  // [bp-0x108]
    unsigned long v12;  // [bp-0x100]
    unsigned long v13;  // [bp-0xf8]
    unsigned long long v14;  // [bp-0xf0]
    unsigned long v15;  // [bp-0xe8]
    unsigned long long v16;  // [bp-0xe0]
    int v17;  // [bp-0xd8]
    unsigned long long v18;  // [bp-0xd0]
    long long v19;  // [bp-0xc0], Other Possible Types: unsigned long long
    int v20;  // [bp-0xb8], Other Possible Types: stat
    unsigned long v21;  // [bp-0x70]
    unsigned long v22;  // [bp-0x60]

    idx = (!a2 ? NULL : &<0x4113ca[is_1]|Stack bp-0x118, 1 B>);
    v2 = 0;
    if (idx)
    {
        v9 = *(a2);
        v10 = a2[1];
        v24 = a2 + 2;
        v11 = *(v24);
        v12 = v24[1];
        v2 = sub_411134(idx);
    }
    if (v2 < 0)
        return 0xffffffff;
    if (a0 < 0 && !a1)
    {
        *(__errno_location()) = 9;
        return 0xffffffff;
    }
    if (g_4e16a4 >= 0)
    {
        if (v2 == 2)
        {
            if (a0 < 0)
                v26 = stat(a1, &v20);
            else
                v26 = fstat(a0, &v20);
            if (v26)
                return 0xffffffff;
            if (idx[1] == 0x3ffffffe)
            {
                *(idx) = sub_41105b(&v20);
                idx[1] = v28;
            }
            else if (idx[3] == 0x3ffffffe)
            {
                v29 = idx + 2;
                *(v29) = sub_41108f(&v20);
                v29[1] = v28;
            }
            v2 += 1;
        }
        if (a0 < 0)
        {
            v3 = utimensat(0xffffff9c, a1, idx, 0);
            if (v3 > 0)
                *(__errno_location()) = 38;
            if (!v3 || *(__errno_location()) != 38)
            {
                g_4e16a4 = 1;
                return v3;
            }
        }
        if (a0 >= 0)
        {
            v3 = futimens(a0, idx, idx);
            if (v3 > 0)
                *(__errno_location()) = 38;
            if (!v3 || *(__errno_location()) != 38)
            {
                g_4e16a4 = 1;
                return v3;
            }
        }
    }
    g_4e16a4 = 0xffffffff;
    g_4e16a8 = 0xffffffff;
    if (v2)
    {
        if (v2 != 3)
        {
            if (a0 < 0)
                v31 = stat(a1, &v20);
            else
                v31 = fstat(a0, &v20);
            if (v31)
                return 0xffffffff;
        }
        if (idx && (char)sub_41129e(&v20, &idx, &idx))
            return 0;
    }
    if (idx)
    {
        v13 = *(idx);
        v14 = ((long long)(idx[1] * 2361183241434822607) >> 71) - ((long long)(idx[1]) >> 63);
        v15 = idx[2];
        v16 = ((long long)(idx[3] * 2361183241434822607) >> 71) - ((long long)(idx[3]) >> 63);
        idx1 = &v13;
    }
    else
    {
        idx1 = NULL;
    }
    if (a0 < 0)
    {
        return futimesat(0xffffff9c, a1, idx1);
    }
    else if (!futimesat(a0, 0, idx1))
    {
        if (!idx1)
            return 0;
        v0 = 499999 < idx1->field_8;
        v1 = 499999 < idx1->field_18;
        if ((v0 || v1) && !fstat(a0, &v20))
        {
            v7 = (unsigned long long)(v21 - idx1->field_0);
            v8 = v22 - idx1->field_10;
            index = NULL;
            *((char [8])&v17) = idx1->field_0;
            *((long long *)&(&v17)[8]) = idx1->field_8;
            *((unsigned long long *)&(&v17)[16]) = idx1->field_10;
            v19 = idx1->field_18;
            if (v0 && v7 == 1 && !sub_411006(&v20))
            {
                index = &v17;
                v18 = 0;
            }
            if (v1 && v8 == 1 && !sub_411032(&v20))
            {
                index = &v17;
                v19 = 0;
            }
            if (index)
                futimesat(a0, 0, index);
        }
        return 0;
    }
    else
    {
        return (!a1 ? 0xffffffff : utimes(a1, idx1));
    }
}



// Function: utimens @ 0x11a12
unsigned long long utimens(unsigned int a0, unsigned long long *a1)
{
    return sub_4113ca(0xffffffff, a0, a1);
}



// Function: lutimens @ 0x11a3d
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

extern unsigned int g_4e16a4;
extern int g_4e16a8;

unsigned long long lutimens(unsigned long a0, unsigned long long *a1)
{
    unsigned long long *v9;  // rax
    unsigned long v11;  // rdx
    unsigned long long *v12;  // rbx
    int v0;  // [bp-0xe8]
    unsigned int v1;  // [bp-0xe4]
    unsigned long long *v2;  // [bp-0xe0]
    int <0x411a3d[is_1]|Stack bp-0xd8, 1 B>;  // [bp-0xd8]
    unsigned long v3;  // [bp-0xd8]
    unsigned long v4;  // [bp-0xd0]
    unsigned long v5;  // [bp-0xc8]
    unsigned long v6;  // [bp-0xc0]
    int v7;  // [bp-0xb8], Other Possible Types: stat

    v2 = (!a1 ? NULL : &<0x411a3d[is_1]|Stack bp-0xd8, 1 B>);
    v0 = 0;
    if (v2)
    {
        v3 = *(a1);
        v4 = a1[1];
        v9 = a1 + 2;
        v5 = *(v9);
        v6 = v9[1];
        v0 = sub_411134(v2);
    }
    if (v0 < 0)
        return 0xffffffff;
    if (g_4e16a8 >= 0)
    {
        if (v0 == 2)
        {
            if (lstat(a0, &v7))
                return 0xffffffff;
            if (v2[1] == 0x3ffffffe)
            {
                *(v2) = sub_41105b(&v7);
                v2[1] = v11;
            }
            else if (v2[3] == 0x3ffffffe)
            {
                v12 = v2 + 2;
                *(v12) = sub_41108f(&v7);
                v12[1] = v11;
            }
            v0 += 1;
        }
        v1 = utimensat(0xffffff9c, a0, v2, 0x100);
        if (v1 > 0)
            *(__errno_location()) = 38;
        if (!v1 || *(__errno_location()) != 38)
        {
            g_4e16a4 = 1;
            g_4e16a8 = 1;
            return v1;
        }
    }
    g_4e16a8 = 0xffffffff;
    switch (v0)
    {
    case 0:
        if (!v0 && lstat(a0, &v7))
            return 0xffffffff;
        if (((unsigned short)*((unsigned int *)(&v7 + 24)) & 0xf000) == 0xa000)
        {
            *(__errno_location()) = 38;
            return 0xffffffff;
        }
        return sub_4113ca(0xffffffff, a0, v2);
    case 3:
        if (v2 && (char)sub_41129e(&v7, &v2, &v2))
            return 0;
    default:
        if (lstat(a0, &v7))
            return 0xffffffff;
    }
}



// Function: nonnull @ 0x11d4e
unsigned long long nonnull(unsigned long long a0)
{
    if (a0)
        return a0;
    sub_40f634(); /* do not return */
}



// Function: xmalloc @ 0x11d70
unsigned long long xmalloc(unsigned long a0)
{
    return sub_411d4e(malloc(a0));
}



// Function: ximalloc @ 0x11d96
unsigned long long ximalloc(long long a0)
{
    return sub_411d4e(sub_412547(a0));
}



// Function: xcharalloc @ 0x11dbc
unsigned long long xcharalloc(unsigned int a0)
{
    return sub_411d70(a0);
}



// Function: xrealloc @ 0x11dda
void* xrealloc(void* a0, unsigned long a1)
{
    void* ptr;  // [bp-0x10]

    ptr = realloc(a0, a1);
    if (ptr)
        return ptr;
    if (a0 && !a1)
        return ptr;
    sub_40f634(); /* do not return */
}



// Function: xirealloc @ 0x11e25
unsigned long long xirealloc(long long a0, long long a1)
{
    return sub_411d4e(sub_412565(a0, a1, a1));
}



// Function: xreallocarray @ 0x11e56
void* xreallocarray(void* a0, unsigned long a1, unsigned long a2)
{
    void* ptr;  // [bp-0x10]

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
    sub_40f634(); /* do not return */
}



// Function: xireallocarray @ 0x11eb0
unsigned long long xireallocarray(long long a0, long long a1, long long a2)
{
    return sub_411d4e(sub_4125c5(a0, a1, a2, a1));
}



// Function: xnmalloc @ 0x11ee9
void* xnmalloc(unsigned int a0, unsigned int a1)
{
    return sub_411e56(NULL, a0, a1);
}



// Function: xinmalloc @ 0x11f14
unsigned long long xinmalloc(long long a0, long long a1)
{
    return sub_411eb0(0, a0, a1);
}



// Function: x2realloc @ 0x11f3f
long long x2realloc(long long a0, long long a1)
{
    return sub_411f6d(a0, a1, 1, a1);
}



// Function: x2nrealloc @ 0x11f6d
void* x2nrealloc(void* a0, unsigned long long *a1, unsigned long long a2)
{
    unsigned long v3;  // rax
    char v4;  // cl
    void* v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]

    v0 = a0;
    v1 = *(a1);
    if (v0)
    {
        v3 = (v1 >> 1) + 1;
        v4 = 0;
        if (__CFADD__(v3, v1) & 1)
            v4 = 1;
        v1 += v3;
        if (v4 & 1)
            sub_40f634(); /* do not return */
    }
    else if (!v1)
    {
        v1 = 128 / a2;
        v1 += !v1;
    }
    v0 = sub_411e56(v0, v1, a2);
    *(a1) = v1;
    return v0;
}



// Function: xpalloc @ 0x1204c
void* xpalloc(void* a0, void* a1, long long a2, unsigned long a3, unsigned long a4)
{
    long long v6;  // rdx
    char v7;  // cl
    char v8;  // dl
    char v9;  // cl
    char v10;  // dl
    void* v0;  // [bp-0x40]
    unsigned long v1;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28]
    unsigned long v3;  // [bp-0x20]
    unsigned long v4;  // [bp-0x18]

    v0 = a0;
    v3 = *((long long *)a1);
    v6 = (long long)(v3) >> 1;
    v7 = 0;
    if (__OFADD__(v3, v6) & 1)
        v7 = 1;
    v1 = v3 + v6;
    if (v7 & 1)
        v1 = 0x7fffffffffffffff;
    if (!(a3 - 0 >> 63 & 1) && a3 < v1)
        v1 = a3;
    v8 = 0;
    if (__OFSMUL__(a4, v1) & 1)
        v8 = 1;
    v2 = a4 * v1;
    v4 = (!(v8 & 1) ? (v2 <= 127 ? 128 : 0) : 0x7fffffffffffffff);
    if (v4)
    {
        v1 = v4 / a4;
        v2 = v4 - v4 % a4;
    }
    if (!v0)
        *((unsigned long *)a1) = 0;
    if (a2 <= v1 - v3)
    {
        v0 = sub_411dda(v0, v2);
        *((unsigned long *)a1) = v1;
        return v0;
    }
    v9 = 0;
    if (__OFADD__(a2, v3) & 1)
        v9 = 1;
    v1 = a2 + v3;
    if (!(v9 & 1) && (a3 - 0 >> 63 & 1 || a3 >= v1))
    {
        v10 = 0;
        if (__OFSMUL__(a4, v1) & 1)
            v10 = 1;
        v2 = a4 * v1;
        if (!(v10 & 1))
        {
            v0 = sub_411dda(v0, v2);
            *((unsigned long *)a1) = v1;
            return v0;
        }
    }
    sub_40f634(); /* do not return */
}



// Function: xzalloc @ 0x12208
long long xzalloc(long long a0)
{
    return sub_41224e(a0, 1);
}



// Function: xizalloc @ 0x1222b
long long xizalloc(long long a0)
{
    return sub_41227f(a0, 1);
}



// Function: xcalloc @ 0x1224e
unsigned long long xcalloc(unsigned long a0, unsigned long a1)
{
    return sub_411d4e(calloc(a0, a1));
}



// Function: xicalloc @ 0x1227f
unsigned long long xicalloc(long long a0, long long a1)
{
    return sub_411d4e(sub_41259c(a0, a1, a1));
}



// Function: xmemdup @ 0x122b0
void* xmemdup(void* a0, unsigned long a1)
{
    return memcpy(sub_411d70(a1), a0, a1);
}



// Function: ximemdup @ 0x122e8
void* ximemdup(void* a0, unsigned long a1)
{
    return memcpy(sub_411d96(a1), a0, a1);
}



// Function: ximemdup0 @ 0x12328
void* ximemdup0(void* a0, unsigned long a1)
{
    unsigned long v0;  // [bp-0x10]

    v0 = sub_411d96(a1 + 1);
    *((char *)(v0 + a1)) = 0;
    return memcpy(v0, a0, a1);
}



// Function: xstrdup @ 0x12377
void* xstrdup(void* a0)
{
    return sub_4122b0(a0, strlen(a0) + 1);
}



// Function: yesno @ 0x123a8
int yesno(void)
{
    char v3;  // al
    char v0;  // [bp-0xd]
    unsigned int i;  // [bp-0xc]

    i = getchar();
    if (i == 121 || i == 89)
        v3 = 1;
    else
        v3 = 0;
    v0 = v3;
    for (v0 &= 1; i != 10 && i != 0xffffffff; i = getchar());
    return v0;
}



// Function: opendir_safer @ 0x123f7
typedef struct DIR {
} DIR;

DIR * opendir_safer(char *a0)
{
    unsigned int v0;  // [bp-0x24]
    unsigned int v1;  // [bp-0x20]
    unsigned int v2;  // [bp-0x1c]
    DIR *ptr;  // [bp-0x18]
    DIR *fp;  // [bp-0x10]

    ptr = opendir(a0);
    if (!ptr)
        return ptr;
    v1 = dirfd(ptr);
    if (v1 < 0)
    {
        return ptr;
    }
    else if (v1 <= 2)
    {
        v2 = sub_4103bd(v1, 1030);
        if (v2 < 0)
        {
            v0 = *(__errno_location());
            fp = NULL;
        }
        else
        {
            fp = fdopendir(v2);
            v0 = *(__errno_location());
            if (!fp)
                close(v2);
        }
        closedir(ptr);
        *(__errno_location()) = v0;
        ptr = fp;
        return ptr;
    }
    else
    {
        return ptr;
    }
}



// Function: gettime @ 0x124c4
long long gettime(long long a0)
{
    return clock_gettime(0, a0);
}



// Function: current_timespec @ 0x124e8
long long current_timespec(void)
{
    char v0;  // [bp-0x28]

    sub_4124c4(&v0);
    return v0;
}



// Function: dup_safer @ 0x12614
long long dup_safer(unsigned int a0)
{
    return sub_4103bd(a0, 0);
}



// Function: main @ 0x39a0
typedef struct sigaction {
} sigaction;

typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern sigaction g_405600;
extern unsigned int g_412f80[4];
extern struct_0 g_413350;
extern struct_0 g_413368;
extern option g_4177c0;
extern int g_418028;
extern int g_41802c;
extern long long stdout;
extern unsigned int optind;
extern long long stderr;
extern unsigned int g_41a420;
extern unsigned int g_41a84c;
extern long long g_41a850;
extern char g_41a85c;
extern unsigned int g_41a860;
extern char g_4dc510;
extern unsigned int g_4dc920;
extern unsigned int g_4dc940;
extern unsigned long long g_4dc9c0;
extern long long g_4dc9c8;
extern unsigned long long g_4dc9d0;
extern unsigned int g_4dc9e0;
extern unsigned int g_4dc9e8;
extern unsigned int g_4dc9f8;
extern unsigned int g_4dc9fc;
extern char g_4dca00;

int main(int a0, void* a1)
{
    unsigned long long v15;  // r15
    unsigned long long v16;  // r14
    char *v25;  // rax
    unsigned int v26;  // ecx
    unsigned int v27;  // eax
    unsigned long v28;  // rax
    char *v29;  // rax
    unsigned long v30;  // rax
    unsigned int v31;  // eax
    unsigned int *i;  // r12
    struct_0 *v33;  // r15
    char *iter;  // rdi
    unsigned long long v17;  // r13
    unsigned long long v35;  // rcx
    unsigned int v36;  // r14d
    unsigned long v37;  // r14
    unsigned long long v38;  // rax
    char v39;  // cl
    unsigned long v40;  // cc_ndep
    unsigned long long v41;  // 4106
    long long v42;  // rdi
    long long v43;  // rsi
    long long v44;  // rdx
    unsigned long long v18;  // r12
    long long v45;  // rcx
    long long v46;  // r8
    long long v47;  // r9
    unsigned long long v19;  // rbx
    unsigned long v20;  // fs
    unsigned long v21;  // rax
    unsigned long len;  // rax
    void* v23;  // r12
    void* node;  // r15
    int j;  // [bp-0xf4], Other Possible Types: unsigned int
    unsigned int v1;  // [bp-0xf0]
    char v2;  // [bp-0xe8]
    int v3;  // [bp-0xe4]
    void* v4;  // [bp-0xe0]
    sigaction v5;  // [bp-0xd8]
    char v6;  // [bp-0xd0]
    unsigned int v7;  // [bp-0x50]
    unsigned long v8;  // [bp-0x40]
    unsigned long long v9;  // [bp-0x30]
    unsigned long long v10;  // [bp-0x20]
    unsigned long long v11;  // [bp-0x18]
    unsigned long long v12;  // [bp-0x10]
    unsigned long long v13;  // [bp-0x8]

    v13 = v15;
    v12 = v16;
    v11 = v17;
    v10 = v18;
    v9 = v19;
    j = a0;
    v8 = *((long long *)(40 + v20));
    v21 = sub_40d290(*((long long *)a1));
    g_41a850 = v21;
    len = strlen(v21);
    if (len > 4)
    {
        v23 = v21 + len - 4;
        if (!strcmp(v23, ".exe"))
            *((char *)v23) = 0;
    }
    v4 = a1;
    node = sub_40d2b0(&v2, &v4, "GZIP");
    g_4dc9d0 = node;
    if (node)
        node = v4;
    g_4dc9c8 = ".gz";
    g_4dc9c0 = strlen(".gz");
    v3 = -0x1;
    if (node)
    {
        v25 = optind;
        if (*((long long *)((char *)node + 0x8 * v25)) && !strcmp(*((long long *)((char *)node + 0x8 * v25)), "--"))
        {
            if (!node)
                goto LABEL_403b60;
            v26 = 176;
        }
        else
        {
            v27 = getopt_long(v2, node, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &g_4177c0.name, &v3);
            v26 = v27 + 131;
            if (v27 >= 0)
            {
                v28 = v26 - 49;
                if ((unsigned int)v28 <= 211)
                    goto *((void *)((long long)(g_412f80[v28] + (char *)&g_412f80[0])));
            }
            else
            {
                if (optind != v2)
                {
                    sub_40f6b0(stderr, "%s: %s: non-option in GZIP environment variable\n", g_41a850, *((long long *)((char *)node + 8 * optind)));
                    sub_4069d0(v42, v43, v44, v45, v46, v47); /* do not return */
                }
                if (optind != 1 && !*((int *)&g_41a85c))
                    sub_40f6b0(stderr, "%s: warning: GZIP environment variable is deprecated; use an alias or script\n", g_41a850);
                free(node);
                v3 = -0x1;
                optind = 1;
LABEL_403b65:
                v26 = getopt_long(j, a1, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &g_4177c0.name, &v3);
                if (v26 < 0)
                {
                    if (g_418028 < 0)
                        g_418028 = g_4dc9f8;
                    if (g_41802c < 0)
                        g_41802c = g_4dc9f8;
                    v1 = optind;
                    if (g_4dc9fc)
                    {
                        if (*((int *)&g_41a85c))
                        {
                            if (g_4dc9c0 - 1 > 29)
                            {
                                sub_40f6b0(stderr, "%s: invalid suffix '%s'\n", g_41a850, g_4dc9c8);
                                sub_406960(1); /* do not return */
                            }
                            goto LABEL_403ff1;
                        }
                        else
                        {
                            sub_40f6b0(stderr, "%s: option --ascii ignored on this system\n", g_41a850);
                        }
                    }
                    if (g_4dc9c0 - 1 > 29)
                    {
                        sub_40f6b0(stderr, "%s: invalid suffix '%s'\n", g_41a850, g_4dc9c8);
                        sub_406960(1); /* do not return */
                    }
                    v31 = *((int *)&g_41a85c);
                    if (!*((int *)&g_41a85c))
                        goto LABEL_403ff6;
LABEL_403ff1:
                    v31 = 13;
LABEL_403ff6:
                    i = &g_4dc940;
                    g_4dc920 = v31;
                    v33 = &g_413350.field_0;
                    sigemptyset(&g_4dc940);
                    do
                    {
                        sigaction(v33->field_0, NULL, &v5);
                        if (v5 != 1)
                            sigaddset(&g_4dc940, v33->field_0);
                    } while ((v33 += 4, v33 != &g_413368.field_0));
                    iter = &v6;
                    v35 = 32;
                    for (node = NULL; v35; i += 1)
                    {
                        v35 -= 1;
                        *((unsigned int *)&iter) = *(i);
                        iter += 4;
                    }
                    v7 = 0;
                    v5 = (sigaction)&g_405600;
                    do
                    {
                        v36 = *((int *)((char *)&g_413350.field_0 + 0x4 * node));
                        if (sigismember(&g_4dc940, v36))
                        {
                            if (!node)
                                g_4dc9e0 = 1;
                            sigaction(v36, &v5, NULL);
                        }
                    } while ((node += 1, node != 0x6));
                    if (j != v1)
                    {
                        while (optind < j)
                        {
                            v37 = *((long long *)((char *)a1 + 8 * optind));
                            optind = optind + 1;
                            if (strcmp(v37, "-"))
                            {
                                sub_406e90(v37);
                            }
                            else
                            {
                                sub_406a80();
                                g_41a860 = g_41a860;
                            }
                        }
                    }
                    else
                    {
                        sub_406a80();
                    }
                    if (!g_4dc510 || (v38 = (unsigned long long)(long long)close(0), !(unsigned int)v38))
                    {
                        if (g_4dc9e8)
                        {
                            if (!*((int *)&g_41a85c) && j - v1 > 1)
                                sub_4065d0(0xffffffff);
                            if (sub_40e3e0(stdout))
                                sub_40d550(); /* do not return */
                        }
                        if (!g_41a860)
                            sub_406960(g_41a84c); /* do not return */
                    }
                    else
                    {
                        g_41a420 = 1768191091;
                        v29 = (char *)_INSERT(v38, 0, (char)v38 + v39);
                        v41 = _ccall(10, 1, (unsigned long long)(char)v38, (unsigned long long)v39, v40);
                        if (!(v41 & 1))
                        {
                            *(v29) = *(v29) + *((char *)&v29);
                            sub_40f6b0(stderr, "%s: %s: non-option in GZIP environment variable\n", g_41a850, *((long long *)((char *)node + 0x8 * v29)));
                            sub_4069d0(v42, v43, v44, v45, v46, v47); /* do not return */
                        }
                    }
                    if ((!g_4dca00 || !fdatasync(1) || *(__errno_location()) == 22) && !close(1))
                        sub_406960(g_41a84c); /* do not return */
                    if (*(__errno_location()) == 9)
                        sub_406960(g_41a84c); /* do not return */
                    sub_40d550(); /* do not return */
                }
                else if (v26 - 49 <= 211)
                {
                    goto *((void *)((long long)(*((int *)((char *)&g_412f80[v26] - 196)) + (char *)&g_412f80[0])));
                }
            }
            if (v26 <= 130)
            {
                sub_4069d0(); /* do not return */
            }
            else if (v26 == 194)
            {
                sub_4069d0(); /* do not return */
            }
        }
        j = v26;
        sub_40f6b0(stderr, "%s: ", g_41a850);
        v30 = v3;
        if ((unsigned int)v30 >= 0)
            sub_40f6b0(stderr, "--%s: ", (&g_4177c0.name)[4 * v30]);
        else
            sub_40f6b0(stderr, "-%c: ", j - 131, j);
        sub_40f6b0(stderr, "option not valid in GZIP environment variable\n");
        sub_4069d0(); /* do not return */
    }
LABEL_403b60:
    goto LABEL_403b65;
}



// Function: bi_init @ 0x4380
extern unsigned long long g_419050;
extern unsigned int g_419058;
extern unsigned short g_41905c;

int bi_init(unsigned int a0)
{
    unsigned int v1;  // eax

    g_419058 = 0;
    v1 = 0;
    g_41905c = 0;
    if (a0 != 0xffffffff)
    {
        v1 = (unsigned int)sub_40df80;
        g_419050 = sub_40df80;
    }
    return v1;
}



// Function: send_bits @ 0x43b0
extern void g_419058;
extern unsigned short g_41905c;
extern unsigned int g_41a004;
extern char g_45b000;

unsigned long long send_bits(unsigned int a0, unsigned int a1)
{
    unsigned int v1;  // edx
    unsigned long v2;  // rax
    unsigned long long v3;  // rax
    unsigned int v4;  // esi
    unsigned long v5;  // rcx
    unsigned long idx;  // rsi

    v1 = *((int *)&g_419058);
    v2 = a0 << ((char)v1 & 31);
    v3 = _INSERT(v2, 0, (unsigned short)v2 | g_41905c);
    v4 = a1 + v1;
    if (16 - a1 < v1)
    {
        v5 = g_41a004;
        g_41905c = v3;
        idx = (unsigned int)v5 + 1;
        if ((unsigned int)v5 <= 0x3fffd)
        {
            (&g_45b000)[v5 & 0xffffffff] = v3;
            g_41a004 = (unsigned int)v5 + 2;
            (&g_45b000)[idx] = *((char *)((void*)&v3 + 1));
            v4 = v1 + a1 - 16;
            v3 = (int)(a0 & 0xffff) >> ((char)(16 - v1) & 31);
        }
        else
        {
            g_41a004 = idx;
            (&g_45b000)[v5] = v3;
            if ((unsigned int)idx == 0x40000)
            {
                sub_40d600();
                v3 = g_41905c;
                idx = g_41a004;
            }
            g_41a004 = (unsigned int)idx + 1;
            (&g_45b000)[idx] = *((char *)((void*)&v3 + 1));
            if (g_41a004 == 0x40000)
                goto LABEL_404478;
            while (1)
            {
LABEL_404478:
                sub_40d600();
            }
        }
    }
    g_41905c = v3;
    *((unsigned int *)&g_419058) = v4;
    return v3;
}



// Function: bi_reverse @ 0x44a0
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



// Function: bi_windup @ 0x44d0
extern int g_419058;
extern unsigned short g_41905c;
extern unsigned int g_41a004;
extern char g_45b000;

unsigned int bi_windup(void)
{
    unsigned long v1;  // rax
    char v2;  // cl
    unsigned long idx;  // rdx
    unsigned long v4;  // rax

    if (g_419058 > 8)
    {
        v1 = g_41a004;
        v2 = g_41905c;
        idx = (unsigned int)v1 + 1;
        if ((unsigned int)v1 <= 0x3fffd)
        {
            (&g_45b000)[v1 & 0xffffffff] = g_41905c;
            g_41a004 = (unsigned int)v1 + 2;
            (&g_45b000)[idx] = g_41905c;
        }
        else
        {
            g_41a004 = idx;
            (&g_45b000)[v1] = g_41905c;
            if ((unsigned int)idx == 0x40000)
            {
                sub_40d600();
                v2 = g_41905c;
                idx = g_41a004;
            }
            (&g_45b000)[idx] = v2;
            g_41a004 = (unsigned int)idx + 1;
            if (g_41a004 == 0x40000)
                goto LABEL_40455b;
        }
    }
    else if (g_419058 > 0 && !(v4 = (unsigned long)(unsigned long long)g_41a004, g_41a004 = (unsigned int)v4 + 1, (&g_45b000)[v4] = (char)g_41905c, g_41a004 != 0x40000))
    {
LABEL_40455b:
        sub_40d600();
    }
    g_419058 = 0;
    g_41905c = 0;
    return 0;
}



// Function: copy_block @ 0x45c0
extern unsigned int g_41a004;
extern char g_45b000;

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
    v3 = sub_4044d0();
    if (a2)
    {
        v4 = g_41a004;
        v5 = (unsigned short)a1 >> 8;
        idx = (unsigned int)v4 + 1;
        if ((unsigned int)v4 <= 0x3fffd)
        {
            v7 = (unsigned int)v4 + 2;
            (&g_45b000)[v4 & 0xffffffff] = a1;
            (&g_45b000)[idx] = v5;
        }
        else
        {
            g_41a004 = idx;
            (&g_45b000)[v4] = a1;
            if ((unsigned int)idx == 0x40000)
            {
                sub_40d600();
                idx = g_41a004;
            }
            v7 = (unsigned int)idx + 1;
            g_41a004 = v7;
            (&g_45b000)[idx] = v5;
            if ((unsigned int)v7 == 0x40000)
            {
                sub_40d600();
                v7 = g_41a004;
            }
        }
        index = (unsigned int)v7 + 1;
        v9 = ~(a1);
        v10 = _INSERT(~(a1), 0, (unsigned short)~(a1) >> 8);
        if ((unsigned int)v7 <= 0x3fffd)
        {
            v3 = (unsigned int)v7 + 2;
            (&g_45b000)[v7 & 0xffffffff] = v9;
            g_41a004 = v3;
            (&g_45b000)[index] = v10;
        }
        else
        {
            g_41a004 = index;
            (&g_45b000)[v7] = v9;
            if ((unsigned int)index == 0x40000)
            {
                v0 = v10;
                sub_40d600();
                index = g_41a004;
                v10 = v0;
            }
            v3 = (unsigned int)index + 1;
            g_41a004 = v3;
            (&g_45b000)[index] = v10;
            if ((unsigned int)v3 == 0x40000)
                v3 = sub_40d600();
        }
    }
    v11 = &iter[a1];
    if (!a1)
        return v3;
    while (1)
    {
        v12 = g_41a004;
        v13 = *(iter);
        iter += 1;
        g_41a004 = (unsigned int)v12 + 1;
        (&g_45b000)[v12] = v13;
        if (g_41a004 == 0x40000)
        {
            v14 = sub_40d600();
            if (iter == v11)
                return v14;
        }
        else if (iter == v11)
        {
            return v12;
        }
    }
}



// Function: longest_match @ 0x4740
typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern char g_419060;
extern unsigned int g_419064;
extern unsigned int g_41906c;
extern int g_419070;
extern struct_0 g_41a880;
extern char g_43b000;

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
    v5 = g_419070;
    v6 = g_419064;
    v2 = v7;
    v8 = g_419070;
    v9 = &(&g_43b000)[g_41906c + 258];
    v10 = (g_41906c < 32506 ? 32506 : g_41906c) - 32506;
    v1 = v11;
    v12 = &(&g_43b000)[g_41906c];
    v13 = *(&v12[g_419070] - 1);
    v14 = v12[g_419070];
    if (g_419070 >= *((int *)&g_419060))
        v6 = g_419064 >> 2;
    v15 = 0;
    v16 = &(&g_43b000)[v4];
    if (v16[v8] != v14 || *(&v16[v8] - 1) != v13 || *(v16) != *(v12) || v16[1] != v12[1])
    {
        v4 = *((short *)((char *)&g_41a880.field_0 + 2 * (v4 & 0x7fff)));
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



// Function: rsync_roll @ 0x4960
extern unsigned long long g_419088;
extern unsigned long long g_419090;
extern char g_43b000;
extern char g_43b001;

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
        v5 = g_419090;
        v6 = &(&g_43b000)[v4];
        v7 = 0;
        v8 = v6;
        do
        {
            if (&v6[a1] == v8)
            {
                if (!(char)v7)
                    return v7;
                g_419090 = v5;
                return v7;
            }
        } while ((v9 = v8 + 1, v5 = (unsigned long)(v5 + (unsigned long long)*(v8)), v7 = 1, v8 += 1, &g_43b001 + v4 + 0xfff - v2 != v8));
        g_419090 = v5;
        v2 = 0x1000;
    }
    if ((unsigned int)v2 >= v3)
        return v2;
    v10 = g_419090;
    v11 = g_419088;
    v0 = v12;
    v13 = 0;
    do
    {
        v10 += (&g_43b000)[v2] - (&g_43b000)[v2 - 0x1000 & 0xffffffff];
        if (v11 != 0xffffffff)
            continue;
        v14 = (unsigned int)v10 & 0xfff;
        if (!((unsigned short)v10 & 0xfff))
        {
            v11 = v2;
            if (v14)
                goto LABEL_404a25;
            goto LABEL_404a24;
        }
        else
        {
            v11 = v11;
            if (!v14)
            {
LABEL_404a24:
                v13 = 1;
            }
            else
            {
LABEL_404a25:
            }
        }
    } while ((v2 += 1, v3 > (unsigned int)v2));
    g_419090 = v10;
    if (!v13)
        return v2;
    g_419088 = v11;
    return v2;
}



// Function: fill_window @ 0x4a60
typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern struct_0 *g_419050;
extern unsigned int g_419068;
extern unsigned int g_41906c;
extern unsigned long long g_419078;
extern unsigned long long g_419088;
extern unsigned int g_41909c;
extern unsigned int g_4190a0;
extern short g_41a880;
extern short g_42a880;
extern short g_43a880;
extern void g_43b000;
extern void g_443000;

short * fill_window(void)
{
    short *v1;  // rax
    short *v2;  // rdx
    short *v3;  // rdx
    short *v4;  // rax
    short *v5;  // rax

    if ((unsigned int)(0x10000 - (g_41909c + g_41906c)) != 0xffffffff && g_41906c > 65273)
    {
        memcpy(&g_43b000, &g_443000, 0x8000);
        g_419068 = g_419068 - 0x8000;
        g_41906c = g_41906c - 0x8000;
        if (g_419088 != 0xffffffff)
            g_419088 = g_419088 - 0x8000;
        g_419078 = g_419078 - 0x8000;
        v1 = &g_41a880;
        v2 = &g_42a880;
        do
        {
        } while ((v3 = v2 + 2, *((unsigned short *)((char *)v3 - 2)) = (*(v2) >= 0 ? 0 : *(v2) + 0x8000), v2 = v3, v2 != &g_43a880));
        do
        {
        } while ((v4 = v1 + 2, *((unsigned short *)((char *)v4 - 2)) = (*(v1) >= 0 ? 0 : *(v1) + 0x8000), v1 = v4, v1 != &g_42a880));
    }
    if (g_4190a0)
        return v4;
    v5 = g_419050(&(&g_43b000)[g_41909c + g_41906c]);
    if ((unsigned int)(short *)((char *)v5 - 1) > 0xfffffffd)
    {
        g_4190a0 = 1;
        *((unsigned short *)&(&g_43b000)[g_41909c + g_41906c]) = 0;
        return g_41906c + g_41909c;
    }
    g_41909c = g_41909c + (unsigned int)v5;
    return v5;
}



// Function: deflate @ 0x4bc0
typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern struct_0 g_412020;
extern struct_1 *g_419050;
extern unsigned int g_419060;
extern unsigned int g_419064;
extern unsigned int g_419068;
extern unsigned int g_41906c;
extern unsigned int g_419070;
extern unsigned long long g_419078;
extern unsigned int g_419080;
extern unsigned long long g_419088;
extern unsigned long long g_419090;
extern unsigned int g_419098;
extern unsigned int g_41909c;
extern unsigned int g_4190a0;
extern unsigned int g_4190a4;
extern unsigned int g_41a000;
extern unsigned short g_41a880[4];
extern void g_42a880;
extern char g_43b000;
extern char g_43b001;

long long deflate(int a0)
{
    unsigned long v3;  // rdx
    unsigned int v4;  // ecx
    unsigned int v13;  // esi
    unsigned short *v14;  // rax
    unsigned int v15;  // r8d
    unsigned int v16;  // eax
    unsigned int v17;  // edx
    unsigned int v18;  // eax
    unsigned int v19;  // r12d
    unsigned int v20;  // eax
    unsigned int v21;  // r14d
    unsigned int v22;  // r9d
    unsigned int v5;  // ecx
    unsigned int v23;  // ecx
    unsigned int j;  // eax
    unsigned int v25;  // eax
    unsigned long index;  // r10
    unsigned short *v27;  // rsi
    unsigned short v28;  // r11w
    unsigned int v29;  // eax
    unsigned int v30;  // r15d
    unsigned int v6;  // eax
    unsigned int v31;  // r13d
    unsigned int v7;  // esi
    unsigned long v32;  // rdx
    unsigned short *v33;  // rax
    unsigned int v34;  // edi
    unsigned int v35;  // eax
    unsigned int v36;  // r13d
    unsigned int v37;  // eax
    unsigned int v38;  // r15d
    unsigned int v39;  // edi
    unsigned int v40;  // edx
    unsigned long v41;  // rax
    unsigned int v8;  // edx
    unsigned long v42;  // rax
    unsigned long idx;  // r8
    unsigned short *v44;  // rcx
    unsigned short v45;  // r9w
    unsigned int v46;  // eax
    unsigned int v47;  // r13d
    unsigned int i;  // eax
    unsigned int v9;  // edi
    unsigned int v10;  // r14d
    unsigned int v11;  // r15d
    unsigned int v12;  // r12d
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]

    if (a0 - 1 > 8)
        sub_40d3f0("bad pack level"); /* do not return */
    memset(&g_42a880, 0, 0x10000);
    v3 = a0;
    g_419088 = 0xffffffff;
    g_419090 = 0;
    v4 = *((short *)((char *)&(&g_412020.field_0)[2 * v3] + 2));
    g_41906c = 0;
    g_419098 = v4;
    v5 = (unsigned short)(&g_412020.field_0)[2 * v3];
    g_419078 = 0;
    g_419060 = v5;
    v6 = *((short *)((char *)&(&g_412020.field_0)[1 + 2 * v3] + 2));
    g_419080 = (unsigned short)(&g_412020.field_0)[1 + 2 * v3];
    g_419064 = v6;
    g_41909c = g_419050(&g_43b000, 0x10000, v3, g_419080);
    if (g_41909c - 1 <= 0xfffffffd)
    {
        g_4190a0 = 0;
        if (g_41909c <= 261)
        {
            do
            {
                sub_404a60();
            } while (g_41909c <= 261 && !g_4190a0);
        }
        v7 = g_43b000 * 32 ^ g_43b001;
        g_4190a4 = v7;
        if (a0 <= 3)
            goto LABEL_405136;
        v8 = g_41909c;
        if (g_41909c)
        {
            v9 = 2;
            v10 = 0;
            while (1)
            {
                v11 = g_41906c;
                v12 = 2;
                v13 = g_419068;
                g_4190a4 = ((&g_43b000)[2 + g_41906c] ^ v7 * 32) & 0x7fff;
                v14 = &g_41a880[0x8000 + g_4190a4];
                v15 = *(v14);
                g_41a880[g_41906c & 0x7fff] = v15;
                *(v14) = g_41906c;
                g_419070 = v9;
                if (v15 && g_419098 > v9)
                {
                    if (g_41906c - v15 > 32506 || g_41906c > 65274)
                    {
                        v12 = 2;
                    }
                    else
                    {
                        v0 = v8;
                        v13 = g_419068;
                        v16 = sub_404740(v15 & 0xffff);
                        v17 = v0;
                        if (v16 <= v17)
                            v17 = v16;
                        v12 = v17;
                        if (v17 != 3)
                        {
                            v9 = g_419070;
                            v11 = g_41906c;
                        }
                        else
                        {
                            v11 = g_41906c;
                            v9 = g_419070;
                            if (4097 <= g_41906c - g_419068)
                                v12 = 2;
                        }
                    }
                }
                if (v9 <= 2 || v12 > v9)
                    break;
                v18 = sub_40b160(v11 - v13 - 1 & 0xffffffff, v9 - 3);
                v19 = g_419070;
                v20 = v19 - 2;
                v8 = g_41909c + 1 - v19;
                g_419070 = v20;
                g_41909c = v8;
                if (!g_41a000)
                {
                    v21 = g_41906c;
                    v22 = v19 - 1;
                }
                else
                {
                    v21 = g_41906c;
                    v1 = v8;
                    v0 = v20;
                    v13 = v19 - 1;
                    sub_404960(g_41906c, v13);
                    v8 = v1;
                    v20 = v0;
                    v22 = v13;
                }
                v23 = g_4190a4;
                j = v21;
                do
                {
                    v25 = j + 1;
                    index = v25 & 0x7fff;
                    v23 = (v23 * 32 ^ (&g_43b000)[3 + j]) & 0x7fff;
                    v27 = &g_41a880[0x8000 + v23];
                    v28 = *(v27);
                    g_41a880[index] = v28;
                    *(v27) = v25;
                    j = v25;
                } while (j != v20 + v21);
                g_4190a4 = v23;
                g_41a880[index] = v28;
                *(v27) = v21 + v19 - 2 & 0xffffffff;
                g_419070 = 0;
                g_41906c = v22 + v21;
                if (g_41a000 && g_41906c > g_419088)
                {
                    g_419088 = 0xffffffff;
                    goto LABEL_4050cf;
                }
                else if (v18)
                {
LABEL_4050cf:
                    v12 = 2;
                    v10 = 0;
                    sub_40add0();
                    v8 = g_41909c;
                    g_419078 = g_41906c;
                }
                else
                {
                    v10 = 0;
                    v12 = 2;
                }
LABEL_404e1f:
                if (v8 <= 261)
                {
                    while (!g_4190a0)
                    {
                        sub_404a60();
                        v8 = g_41909c;
                        if (g_41909c > 261)
                            goto LABEL_404e5b;
                    }
                    v8 = g_41909c;
                    if (!g_41909c && v10)
                    {
                        sub_40b160(0, *(&(&g_43b000)[g_41906c] - 1));
                        goto LABEL_404f79;
                    }
                }
LABEL_404e5b:
                v7 = g_4190a4;
                v9 = v12;
            }
            if (!v10)
            {
                if (g_41a000)
                {
                    if (v11 > g_419088)
                    {
                        g_419088 = 0xffffffff;
                        sub_40add0();
                        g_419078 = g_41906c;
                        v11 = g_41906c;
                        if (g_41a000)
                            goto LABEL_404f20;
                    }
                    else
                    {
LABEL_404f20:
                        sub_404960(v11, 1);
                    }
                }
                v10 = 1;
                g_41906c = v11 + 1;
                v8 = g_41909c - 1;
                g_41909c = v8;
                goto LABEL_404e1f;
            }
            v29 = sub_40b160(0, *(&(&g_43b000)[v11] - 1));
            if (g_41a000)
            {
                v30 = g_41906c;
                if (g_41906c > g_419088)
                {
                    g_419088 = 0xffffffff;
                }
                else if (!v29)
                {
LABEL_404f59:
                    sub_404960(v30, 1);
LABEL_404ef8:
                    g_41906c = v30 + 1;
                    v8 = g_41909c - 1;
                    g_41909c = v8;
                    goto LABEL_404e1f;
                }
            }
            else
            {
                v30 = g_41906c;
                if (!v29)
                    goto LABEL_404ef8;
            }
            sub_40add0();
            g_419078 = g_41906c;
            v30 = g_41906c;
            if (g_41a000)
                goto LABEL_404f59;
            goto LABEL_404ef8;
        }
        else
        {
LABEL_4054a1:
        }
LABEL_404f79:
    }
    else
    {
        g_4190a0 = 1;
        g_41909c = 0;
        if (a0 > 3)
            goto LABEL_4054a1;
LABEL_405136:
        v31 = g_41909c;
        g_419070 = 2;
        if (g_41909c)
        {
            do
            {
                do
                {
LABEL_405160:
                    v32 = g_41906c;
                    g_4190a4 = (g_4190a4 * 32 ^ (&g_43b000)[2 + v32]) & 0x7fff;
                    v33 = &g_41a880[0x8000 + g_4190a4];
                    v34 = *(v33);
                    g_41a880[(unsigned int)v32 & 0x7fff] = v34;
                    *(v33) = v32;
                    if (!v34 || (unsigned int)((v32 & 0xffffffff) - v34) > 32506 || (unsigned int)v32 > 65274)
                    {
LABEL_4051ba:
                        v46 = sub_40b160(0, (&g_43b000)[v32]);
                        v38 = g_41a000;
                        v47 = g_41906c;
                        v37 = v46;
                        if (g_41a000)
                            sub_404960(v47, 1);
                        g_41909c = g_41909c - 1;
                        g_41906c = v47 + 1;
                        goto LABEL_4051f2;
                    }
                    v35 = sub_404740(v34);
                    v36 = (v35 <= v31 ? v35 : v31);
                    if (v36 <= 2)
                    {
                        v32 = g_41906c;
                        goto LABEL_4051ba;
                    }
                    v37 = sub_40b160(g_41906c - g_419068, v36 - 3);
                    v38 = g_41a000;
                    g_41909c = g_41909c - v36;
                    v39 = g_41906c;
                    if (!g_41a000)
                    {
                        if (v36 <= g_419098)
                            goto LABEL_40538f;
                        goto LABEL_405338;
                    }
                    else
                    {
                        v13 = g_41906c;
                        sub_404960(g_41906c, v36);
                        v39 = v13;
                        if (v36 > g_419098)
                        {
LABEL_405338:
                            g_41906c = v36 + v39;
                            g_4190a4 = (&g_43b000)[g_41906c] * 32 ^ (&g_43b000)[1 + g_41906c];
                            goto LABEL_4051f2;
                        }
                        else
                        {
LABEL_40538f:
                            v40 = g_4190a4;
                            v41 = v39;
                            do
                            {
                                v42 = (unsigned int)v41 + 1;
                                idx = (unsigned int)v42 & 0x7fff;
                                v40 = (v40 * 32 ^ (&g_43b000)[3 + v41]) & 0x7fff;
                                v44 = &g_41a880[0x8000 + v40];
                                v45 = *(v44);
                                g_41a880[idx] = v45;
                                *(v44) = v42;
                                v41 = v42;
                            } while ((unsigned int)v41 != (unsigned int)(v36 + v39 - 1));
                            __unsupported_jumpkind_Ijk_NoDecode()
                            *((unsigned int *)((char *)v44 + v42 - 0x11)) = *((int *)((char *)v44 + v42 - 0x11)) + 1;
                            g_4190a4 = v40;
                            g_41a880[idx] = v45;
                            *(v44) = v42;
                            g_41906c = v39;
LABEL_4051f2:
                            if (v38 && g_41906c > g_419088)
                            {
                                g_419088 = 0xffffffff;
                            }
                            else if (!v37)
                            {
                                continue;
                            }
                            sub_40add0();
                            g_419078 = g_41906c;
                        }
                    }
                } while ((i = g_41909c, v31 = g_41909c, g_41909c > 261));
                while (!g_4190a0)
                {
                    sub_404a60();
                    i = g_41909c;
                    v31 = g_41909c;
                    if (g_41909c > 261)
                        goto LABEL_405160;
                }
                v31 = i;
            } while (i);
        }
    }
LABEL_404fa4:
    goto LABEL_404fa4;
}



// Function: license @ 0x54d0
extern unsigned int g_0;
extern unsigned long long g_417b20;

int license(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5, unsigned long a6)
{
    unsigned long long v2;  // rbx
    unsigned long long *v3;  // rbx
    unsigned int v4;  // eax
    unsigned long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    v1 = v2;
    v3 = &g_417b20;
    *((unsigned long long **)(v0 - 8)) = &g_417b20;
    g_0 = g_0;
    sub_40f830("%s %s\n");
    do
    {
        v3 += 1;
        v4 = sub_40f830(0x4137c7);
    } while (*(v3));
    return v4;
}



// Function: remove_output_file @ 0x5530
extern unsigned int g_0;
extern int g_418018;
extern char g_4dc520;
extern int g_4dc940;

void remove_output_file(char a0)
{
    char *v3;  // rax
    int v4;  // edi
    char *iter;  // rax
    char *node;  // rdx
    char v7;  // al
    unsigned long v8;  // fs
    int v0[32];  // [bp-0x4a8]
    char v1;  // [bp-0x428]

    *(v3) = *(v3) - *((char *)&v3);
    *(v3) = *(v3) + *((char *)&v3);
    if (a0)
    {
        v4 = g_418018;
        if (g_418018 < 0)
        {
            if (v3 == *((long long *)(40 + v8)))
                return;
            __stack_chk_fail(); /* do not return */
        }
    }
    else
    {
        sigprocmask(0, &g_4dc940, v0);
        v4 = g_418018;
        if (g_418018 < 0)
        {
            sigprocmask(2, v0, NULL);
            if (v3 != *((long long *)(40 + v8)))
                __stack_chk_fail(); /* do not return */
            return;
        }
    }
    g_418018 = 0xffffffff;
    close(v4);
    iter = &g_4dc520;
    node = &v1;
    while (1)
    {
        v7 = *(iter);
        *(node) = v7;
        if (!v7)
            break;
        iter += 1;
        node += 1;
    }
    sub_40d2a0();
    if (!a0)
    {
        sigprocmask(2, v0, NULL);
        if (v3 != *((long long *)(40 + v8)))
            __stack_chk_fail(); /* do not return */
        return;
    }
    else if (v3 == *((long long *)(40 + v8)))
    {
        return;
    }
    else
    {
        __stack_chk_fail(); /* do not return */
    }
}



// Function: discard_input_bytes @ 0x5690
extern unsigned int g_0;
extern unsigned int g_41a008;
extern char g_41a00c;
extern char g_49c000;

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
        v5 = g_41a008;
        if (g_41a008 < *((int *)&g_41a00c))
        {
LABEL_4056ce:
            v6 = (&g_49c000)[v5];
            g_41a008 = (unsigned int)v5 + 1;
            v0 = v6;
            if (v2)
            {
LABEL_405734:
                sub_40d0d0(&v0, 1);
            }
LABEL_4056e4:
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
                v5 = g_41a008;
                if (g_41a008 < *((int *)&g_41a00c))
                    goto LABEL_4056ce;
                goto LABEL_405725;
            }
        }
        else
        {
LABEL_405725:
            v0 = sub_40d6c0(0);
            if (!v2)
                goto LABEL_4056e4;
            goto LABEL_405734;
        }
    }
}



// Function: get_method @ 0x5750
extern unsigned int g_0;
extern void g_412c6d;
extern void g_412c70;
extern void g_412c90;
extern void g_412c95;
extern void g_412c98;
extern void g_412c9b;
extern unsigned int g_418020;
extern unsigned int g_418028;
extern unsigned int g_41802c;
extern unsigned long long g_4180d0;
extern long long stderr;
extern unsigned int g_41a008;
extern unsigned int g_41a00c;
extern long long g_41a020;
extern char g_41a420;
extern unsigned long long g_41a830;
extern unsigned long long g_41a838;
extern unsigned int g_41a84c;
extern long long g_41a850;
extern unsigned int g_41a85c;
extern unsigned int g_41a860;
extern char g_49c000;
extern unsigned int g_4dc9d8;
extern unsigned int g_4dc9dc;
extern unsigned int g_4dc9e4;
extern unsigned int g_4dc9e8;
extern unsigned int g_4dc9f4;
extern unsigned long long g_4dfea8;

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

    v11 = g_41a00c;
    v12 = g_41a008;
    if (!g_4dc9f4 || !g_41a860)
    {
        if ((unsigned int)v12 < g_41a00c)
        {
            v18 = (unsigned int)v12 + 1;
            g_41a008 = v18;
            v19 = (&g_49c000)[v12];
        }
        else
        {
            v19 = sub_40d6c0(0);
            v18 = g_41a008;
            v11 = g_41a00c;
        }
        v3 = v19;
        if (!v19)
        {
            if (v11 > (unsigned int)v18)
            {
                g_41a008 = (unsigned int)v18 + 1;
                i = (&g_49c000)[v18];
                v17 = i & 0xff;
            }
            else
            {
                v17 = sub_40d6c0(1);
                i = v17;
            }
            v14 = 0;
        }
        else
        {
            if (v11 > (unsigned int)v18)
            {
                g_41a008 = (unsigned int)v18 + 1;
                v17 = (&g_49c000)[v18];
            }
            else
            {
                v17 = sub_40d6c0(0);
            }
            i = 0;
            v14 = 0;
        }
    }
    else
    {
        if ((unsigned int)v12 < g_41a00c)
        {
            v13 = (unsigned int)v12 + 1;
            v14 = (&g_49c000)[v12];
            g_41a008 = v13;
            v15 = v14 & 0xff;
        }
        else
        {
            v15 = sub_40d6c0(1);
            v13 = g_41a008;
            v11 = g_41a00c;
            v14 = v15;
        }
        v3 = v15;
        if ((unsigned int)v13 < v11)
        {
            g_41a008 = (unsigned int)v13 + 1;
            i = (&g_49c000)[v13];
            v17 = i & 0xff;
        }
        else
        {
            v17 = sub_40d6c0(1);
            i = v17;
        }
    }
    v4 = v17;
    g_418020 = 0xffffffff;
    g_4dfea8 = 0;
    g_4dc9d8 = g_4dc9d8 + 1;
    g_4dc9dc = 0;
    if (!memcmp(&v3, &g_412c6d, 2) || !memcmp(&v3, &g_412c70, 2))
    {
        v22 = g_41a008;
        if ((unsigned int)v22 < g_41a00c)
        {
            g_41a008 = (unsigned int)v22 + 1;
            v23 = (&g_49c000)[v22];
        }
        else
        {
            v23 = sub_40d6c0(0);
        }
        g_418020 = v23;
        if (v23 != 8)
        {
            sub_40f6b0(stderr, "%s: %s: unknown method %d -- not supported\n", g_41a850, &g_41a420, v23);
            g_41a84c = 1;
            return 0xffffffff;
        }
        g_4180d0 = sub_40cc20;
        v24 = g_41a008;
        if ((unsigned int)v24 < g_41a00c)
        {
            g_41a008 = (unsigned int)v24 + 1;
            v25 = (&g_49c000)[v24];
        }
        else
        {
            v25 = sub_40d6c0(0);
        }
        if ((char)v25 & 32)
        {
LABEL_4062ac:
            sub_40f6b0(stderr);
            g_41a84c = 1;
        }
        else if (!((char)v25 & 192))
        {
            v26 = g_41a008;
            v27 = g_41a00c;
            if (g_41a008 >= g_41a00c)
                goto LABEL_405d62;
LABEL_405a0f:
            v29 = (unsigned int)v26 + 1;
            g_41a008 = v29;
            v30 = (&g_49c000)[v26];
            if (v27 <= (unsigned int)v29)
                goto LABEL_405d80;
            goto LABEL_405a2c;
        }
        else
        {
            sub_40f6b0(stderr, "%s: %s has flags 0x%x -- not supported\n", g_41a850, &g_41a420, (char)v25);
            v28 = g_4dc9f4;
            g_41a84c = 1;
            if ((unsigned int)v28 > 1)
            {
                v26 = g_41a008;
                v27 = g_41a00c;
                if (g_41a008 < g_41a00c)
                    goto LABEL_405a0f;
LABEL_405d62:
                v31 = sub_40d6c0(0);
                v29 = g_41a008;
                v27 = g_41a00c;
                v30 = v31;
                if (g_41a00c > g_41a008)
                {
LABEL_405a2c:
                    v32 = (unsigned int)v29 + 1;
                    v33 = (&g_49c000)[v29];
                    g_41a008 = v32;
                    v34 = v30 | v33 * 0x100;
                    if ((unsigned int)v32 >= v27)
                        goto LABEL_405da4;
                    goto LABEL_405a4f;
                }
                else
                {
LABEL_405d80:
                    v35 = sub_40d6c0(0);
                    v32 = g_41a008;
                    v27 = g_41a00c;
                    v34 = v30 | v35 * 0x100;
                    if (g_41a008 < g_41a00c)
                    {
LABEL_405a4f:
                        v36 = (unsigned int)v32 + 1;
                        v37 = (&g_49c000)[v32];
                        g_41a008 = v36;
                        v38 = v34 | v37 * 0x10000;
                        if (v27 <= (unsigned int)v36)
                            goto LABEL_405dc9;
                        goto LABEL_405a73;
                    }
                    else
                    {
LABEL_405da4:
                        v39 = sub_40d6c0(0);
                        v36 = g_41a008;
                        v38 = v34 | v39 * 0x10000;
                        if (g_41a00c > g_41a008)
                        {
LABEL_405a73:
                            g_41a008 = (unsigned int)v36 + 1;
                            v40 = (&g_49c000)[v36] * 0x1000000;
                            goto LABEL_405a8b;
                        }
                        else
                        {
LABEL_405dc9:
                            v40 = sub_40d6c0(0) * 0x1000000;
LABEL_405a8b:
                            v41 = v38 | v40;
                            if ((v38 || v40) && !g_418028)
                            {
                                if (v41 >= 0)
                                {
                                    g_41a830 = v41;
                                    g_41a838 = 0;
                                }
                                else
                                {
                                    if (!g_41a85c)
                                        sub_40f6b0(stderr, "%s: %s: MTIME %lu out of range for this platform\n", g_41a850, &g_41a420, v41);
                                    if (!g_41a84c)
                                        g_41a84c = 2;
                                    g_41a838 = 0x3b9ac9ff;
                                    g_41a830 = 0x7fffffffffffffff;
                                }
                            }
                            v42 = g_41a008;
                            v43 = g_41a00c;
                            if ((unsigned int)v42 < g_41a00c)
                            {
                                v44 = (unsigned int)v42 + 1;
                                g_41a008 = v44;
                                v45 = (&g_49c000)[v42];
                            }
                            else
                            {
                                v45 = sub_40d6c0(0);
                                v44 = g_41a008;
                                v43 = g_41a00c;
                            }
                            v8 = v45;
                            if (v43 > (unsigned int)v44)
                            {
                                g_41a008 = (unsigned int)v44 + 1;
                                v46 = (&g_49c000)[v44];
                            }
                            else
                            {
                                v46 = sub_40d6c0(0);
                            }
                            v47 = v25;
                            v9 = v46;
                            v48 = (char)v47 & 2;
                            if ((char)v47 & 2)
                            {
                                v5 = 8;
                                v6 = v25;
                                v7 = v41;
                                sub_40d0d0(0, 0);
                                sub_40d0d0(&v3, 10);
                            }
                            if ((char)v25 & 4)
                            {
                                v49 = g_41a008;
                                v50 = g_41a00c;
                                if ((unsigned int)v49 < g_41a00c)
                                {
                                    v51 = (unsigned int)v49 + 1;
                                    g_41a008 = v51;
                                    v52 = (&g_49c000)[v49];
                                }
                                else
                                {
                                    v52 = sub_40d6c0(0);
                                    v51 = g_41a008;
                                    v50 = g_41a00c;
                                }
                                v1 = v52;
                                if (v50 > (unsigned int)v51)
                                {
                                    g_41a008 = (unsigned int)v51 + 1;
                                    v53 = (&g_49c000)[v51];
                                }
                                else
                                {
                                    v53 = sub_40d6c0(0);
                                }
                                v2 = v53;
                                v54 = v53 * 0x100 | v52;
                                if (g_4dc9e4)
                                    sub_40f6b0(stderr, "%s: %s: extra field of %u bytes ignored\n", g_41a850, &g_41a420, v54);
                                if (v48)
                                    sub_40d0d0(&v1, 2);
                                sub_405690(v54, (char)v25);
                            }
                            if ((char)v25 & 8)
                            {
                                if (!g_41802c && (!g_41a860 || g_4dc9e8) && g_4dc9d8 <= 1)
                                {
                                    v0 = sub_40d290(&g_41a020);
                                    iter = v0;
                                    while (1)
                                    {
                                        v56 = g_41a008;
                                        if ((unsigned int)v56 < g_41a00c)
                                        {
                                            v57 = (&g_49c000)[v56];
                                            g_41a008 = (unsigned int)v56 + 1;
                                        }
                                        else
                                        {
                                            v57 = sub_40d6c0(0);
                                        }
                                        *(iter) = v57;
                                        iter += 1;
                                        if (!v57)
                                            break;
                                        if (iter >= &g_41a420)
                                            sub_40d3f0("corrupted input -- file name too large"); /* do not return */
                                    }
                                    if (v48)
                                        sub_40d0d0(v0, (iter & 0xffffffff) - v0);
                                    ptr = sub_40d290(v0);
                                    memmove(v0, ptr, strlen(ptr) + 1);
                                }
                                else
                                {
                                    sub_405690(0xffffffffffffffff, (char)v25);
                                }
                            }
                            if ((char)v25 & 16)
                                sub_405690(0xffffffffffffffff, (char)v25);
                            if (!v48)
                            {
LABEL_405c49:
                                if (g_4dc9d8 == 1)
                                {
                                    g_4dfea8 = g_41a008 + 8;
                                    if (g_418020 >= 0)
                                        return g_418020;
                                    goto LABEL_4062ac;
                                }
                                else if (g_418020 >= 0)
                                {
                                    return g_418020;
                                }
                                goto LABEL_405c60;
                            }
                            else
                            {
                                v60 = sub_40d0d0(&v3, 0);
                                v61 = g_41a00c;
                                v62 = v60;
                                v63 = g_41a008;
                                if ((unsigned int)v63 < g_41a00c)
                                {
                                    v64 = (unsigned int)v63 + 1;
                                    g_41a008 = v64;
                                    v65 = (&g_49c000)[v63];
                                }
                                else
                                {
                                    v66 = sub_40d6c0(0);
                                    v64 = g_41a008;
                                    v61 = g_41a00c;
                                    v65 = v66;
                                }
                                if ((unsigned int)v64 < v61)
                                {
                                    g_41a008 = (unsigned int)v64 + 1;
                                    v67 = (&g_49c000)[v64] * 0x100;
                                }
                                else
                                {
                                    v67 = sub_40d6c0(0) * 0x100;
                                }
                                v68 = v67 | v65;
                                if (v62 == v68 || !(sub_40f6b0(stderr, "%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n", g_41a850, &g_41a420, (unsigned long long)v68, (unsigned long long)(v62 & 0xffff)), v69 = (unsigned long)(unsigned long long)g_4dc9f4, g_41a84c = 1, (unsigned int)v69 <= 1))
                                    goto LABEL_405c49;
                            }
                        }
                    }
                }
            }
        }
        return 0xffffffff;
    }
    if (memcmp(&v3, &g_412c90, 2) || g_41a008 != 2 || memcmp(&g_49c000, &g_412c90, 4))
    {
        if (!memcmp(&v3, &g_412c95, 2))
        {
            g_418020 = 2;
            g_4180d0 = sub_40c590;
            return 2;
        }
        if (!memcmp(&v3, &g_412c98, 2))
        {
            g_418020 = 1;
            g_4dc9dc = 1;
            g_4180d0 = sub_40be90;
            return 1;
        }
        if (!memcmp(&v3, &g_412c9b, 2))
        {
            g_418020 = 3;
            g_4dc9dc = 1;
            g_4180d0 = sub_40b920;
            return 3;
        }
        if (!g_4dc9f4 || !g_41a860 || g_4dc9e8)
            goto LABEL_40628e;
        g_418020 = 0;
        g_4180d0 = sub_40d770;
        if (i != 0xffffffff)
            g_41a008 = g_41a008 - 1;
        g_4dc9dc = 1;
        if (v14 == 0xffffffff)
            return g_4dc9e8;
        sub_40d5a0(1, &v3, 1);
    }
    else
    {
        g_41a008 = 0;
        g_4180d0 = sub_40cc20;
        if (sub_40cb20(a0))
            return 0xffffffff;
        g_4dc9dc = 1;
    }
    if (g_418020 >= 0)
        return g_418020;
LABEL_40628e:
    __unsupported_jumpkind_Ijk_NoDecode()
    *(v20) = *(v20) + v21;
LABEL_405c60:
    if (!v3)
    {
        if (!i)
        {
            do
            {
                if (g_41a008 < g_41a00c)
                {
                    i = (&g_49c000)[g_41a008];
                    g_41a008 = g_41a008 + 1;
                }
                else
                {
                    i = sub_40d6c0(1);
                }
            } while (!i);
        }
        if (i == 0xffffffff)
        {
            if (!g_4dc9e4)
                return 0xfffffffd;
            if (!g_41a85c)
                sub_40f6b0(stderr, "\n%s: %s: decompression OK, trailing zero bytes ignored\n", g_41a850, &g_41a420);
            if (!g_41a84c)
                g_41a84c = 2;
            return 0xfffffffd;
        }
    }
    if (!g_41a85c)
        sub_40f6b0(stderr, "\n%s: %s: decompression OK, trailing garbage ignored\n", g_41a850, &g_41a420);
    if (!g_41a84c)
        g_41a84c = 2;
    return 0xfffffffe;
}



// Function: get_suffix @ 0x63e0
extern unsigned int g_0;
extern char *g_418040[2];
extern char *g_418048[2];
extern char g_4dc9c0;
extern void g_4dc9c8;

long long get_suffix(unsigned long a0)
{
    char *v4;  // r13
    char **v5;  // rbx
    unsigned int v14;  // eax
    char **v6;  // rbx
    unsigned long len;  // rax
    char **iter;  // rbx
    unsigned long ptr;  // rax
    unsigned long long idx;  // rdx
    unsigned long v11;  // r15
    unsigned int v12;  // eax
    unsigned long v13;  // r15
    unsigned long v0;  // [bp-0x80]
    unsigned long len1;  // [bp-0x70]
    char v2[40];  // [bp-0x68]

    v4 = g_418048;
    if (g_418048)
    {
        v5 = &g_418048[0];
        do
        {
            v6 = v5;
            len = strlen(v4);
            if (*((long long *)&g_4dc9c0) < len && !strcmp(*((long long *)&g_4dc9c8), &v4[len + -1 * *((long long *)&g_4dc9c0)]))
            {
                iter = &g_418048[0];
                sub_40d230(sub_40f640(*((long long *)&g_4dc9c8)));
                idx = 8;
                goto LABEL_406479;
            }
        } while ((v4 = v6[1], v5 = v6 + 8, v6[1]));
    }
    ptr = sub_40f640(*((long long *)&g_4dc9c8));
    sub_40d230(ptr);
    idx = 0;
    iter = &g_418040[0];
LABEL_406479:
    g_418040[idx] = ptr;
    len1 = strlen(v11);
    if ((unsigned int)len1 <= 32)
        __strcpy_chk(v2, v11, 33);
    else
        __strcpy_chk(v2, a0 + (int)len1 - 32, 33, a0);
    sub_40d230(v2);
    v12 = strlen(v2);
    v13 = *(iter);
    v0 = v12;
    while (1)
    {
        v14 = strlen(v13);
        if ((unsigned int)v0 > v14 && *(&v2[v0 + -1 * v14] - 1) != 47 && !strcmp(&v2[v0 + -1 * v14], v13))
        {
            free(ptr);
            return a0 + (int)len1 - v14;
        }
        v13 = iter[1];
        iter += 1;
        if (!v13)
        {
            free(ptr);
            return v13;
        }
    }
}



// Function: do_list @ 0x65d0
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

extern unsigned long long g_0;
extern unsigned int g_418010;
extern long long stdout;
extern long long g_41a820;
extern long long g_41a828;
extern int g_41a830;
extern char g_41a85c;
extern unsigned long long g_4dc500;
extern unsigned long long g_4dc508;
extern unsigned int g_4dc9e4;
extern unsigned long long g_4dfea8;

int do_list(void)
{
    unsigned long long v2;  // r12
    int v3;  // edi
    unsigned long v10;  // cc_ndep
    unsigned long long v11;  // 4110
    long long v12;  // rsi
    long long v4;  // rax
    long long v5;  // rsi
    tm *tm;  // rbp
    unsigned long v7;  // rbx
    char v8;  // cl
    unsigned long v9;  // rbp
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (g_418010)
    {
        if (v3 >= 0)
        {
            g_418010 = 0;
            if (g_4dc9e4)
                sub_40f830("method  crc     date  time  ");
            if (!*((int *)&g_41a85c))
                sub_40f830("%*.*s %*.*s  ratio uncompressed_name\n", 19, 19, "compressed", 19, 19, "uncompressed");
        }
        else
        {
LABEL_406718:
            v12 = g_4dc508;
            if (g_4dc508 <= 0)
            {
                return;
            }
            else if (g_4dc500 > 0)
            {
                if (!g_4dc9e4 || (sub_40f830("                            "), v12 = (long long)g_4dc508, !g_4dc9e4))
                {
                    if (*((int *)&g_41a85c))
                    {
                        sub_40d850(g_4dc500 - (v12 - g_4dfea8), g_4dc500, stdout);
                        sub_40f830();
                        return;
                    }
                }
                sub_40d8a0(stdout);
                sub_40f830(" ");
                sub_40d8a0(stdout, g_4dc500, 19);
                sub_40f830(" ");
                sub_40d850(g_4dc500 - (g_4dc508 - g_4dfea8), g_4dc500, stdout);
                sub_40f830();
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
        if (v3 < 0)
            goto LABEL_406718;
    }
    if (!g_4dc9e4)
    {
        sub_40d8a0(stdout, g_41a828, 19);
        sub_40f830(" ");
        sub_40d8a0(stdout, g_41a820, 19);
        sub_40f830(" ");
        v4 = g_41a828;
        if (v4 != -0x1)
        {
            if (g_4dc508 >= 0)
                g_4dc508 = g_4dc508 + v4;
            v5 = g_41a820;
            if (v5 == -0x1)
            {
                g_4dc500 = 0xffffffffffffffff;
                g_4dfea8 = 0;
                g_41a820 = 0;
                g_41a828 = 0;
                sub_40d850(0, 0, stdout);
                sub_40f830();
                return;
            }
        }
        else
        {
            g_4dc508 = 0xffffffffffffffff;
            v5 = 0;
            v4 = 0;
            g_4dfea8 = 0;
            g_41a820 = 0;
            g_41a828 = 0;
        }
        if (g_4dc500 >= 0)
        {
            __unsupported_jumpkind_Ijk_NoDecode()
            return;
        }
        sub_40d850(v5 - (v4 - g_4dfea8), v5, stdout);
        sub_40f830();
        return;
    }
    tm = localtime(&g_41a830);
    g_0 = v7;
    if (tm)
    {
        v8 = tm->tm_hour;
        g_0 = v9;
        v11 = _ccall(12, 1, (unsigned long long)v8, (unsigned long long)v8, v10);
        if (v11 & 1)
            goto LABEL_0x4068b6;
        else
            goto LABEL_0x4068b3;
    }
    else
    {
        sub_40f830("??? ?? ??:?? ");
        __unsupported_jumpkind_Ijk_NoDecode()
    }
}



// Function: do_exit @ 0x6960
extern unsigned long long g_0;
extern unsigned int g_4dc040;
extern void* g_4dc9d0;

void do_exit(int a0)
{
    if (!g_4dc040)
    {
        g_4dc040 = 1;
        free(g_4dc9d0);
        g_4dc9d0 = 0;
    }
    exit(a0); /* do not return */
}



// Function: finish_out @ 0x69a0
extern unsigned long long g_0;
extern long long stdout;

long long finish_out(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (!sub_40e060(stdout))
        sub_406960(0); /* do not return */
    sub_40d550(); /* do not return */
}



// Function: try_help @ 0x69d0
extern long long stderr;
extern long long g_41a850;

void try_help(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f6b0(stderr, "Try `%s --help' for more information.\n", g_41a850);
    sub_406960(1); /* do not return */
}



// Function: treat_stdin @ 0x6a80
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

extern unsigned long long g_0;
extern unsigned int g_418020;
extern long long stderr;
extern unsigned int g_41a008;
extern char g_41a00c;
extern unsigned int g_41a014;
extern unsigned int g_41a020;
extern unsigned short g_41a024;
extern char g_41a026;
extern unsigned int g_41a420;
extern unsigned short g_41a424;
extern unsigned long long g_41a820;
extern long long g_41a828;
extern int g_41a84c;
extern unsigned int g_41a858;
extern unsigned int g_41a85c;
extern unsigned int g_41a860;
extern stat g_4dc060;
extern char g_4dc510;
extern unsigned int g_4dc9d8;
extern char g_4dc9dc;
extern unsigned int g_4dc9e4;
extern unsigned int g_4dc9e8;
extern unsigned int g_4dc9f4;
extern unsigned int g_4dc9f8;
extern char g_4dca01;
extern unsigned long long g_4dfea8;

char * treat_stdin(void)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rdx
    char *v4;  // rcx
    long long v5;  // r8
    long long v6;  // r9
    char *v7;  // rax
    void* v8;  // rax

    if (g_4dc9f4 || g_4dc9e8 || !g_4dca01 && !isatty(!g_4dc9f8))
    {
        g_41a420 = 1768191091;
        g_41a424 = 110;
        g_41a020 = 1868854387;
        g_41a024 = 29813;
        g_41a026 = 0;
        if (!fstat(0, &g_4dc060.st_mode))
        {
            sub_406a00();
            sub_40d150(v1, v2, v3, v4, v5, v6);
            g_41a860 = 1;
            g_4dc9d8 = 0;
            g_41a014 = 0;
            g_4dc510 = 1;
            if (g_4dc9f8)
            {
                g_418020 = sub_405750(0);
                if (g_418020 < 0)
                    sub_406960(g_41a84c); /* do not return */
            }
            while (1)
            {
                v7 = sub_40d9d0(0, 1);
                if ((unsigned int)v7)
                    return v7;
                if (!g_4dc9f8 || *((int *)&g_4dc9dc))
                    break;
                v7 = *((int *)&g_41a00c);
                if (g_41a008 == *((int *)&g_41a00c))
                {
                    if (*((int *)&g_41a00c) != 0x40000 || !(v7 = (char *)(unsigned long long)sub_40d6c0(1), (unsigned int)(unsigned long long)sub_40d6c0(1) != 0xffffffff))
                        break;
                    g_41a008 = 0;
                }
                g_418020 = sub_405750(g_41a014);
                if (g_418020 < 0)
                    return g_418020;
                g_41a820 = 0;
            }
            *(v7) = *(v7) + *((char *)&v7);
            if (!g_4dc9e4)
                return v7;
            if (g_41a858)
                return sub_40f6b0();
            if (g_4dc9f8)
                return g_4dc9f8;
            v8 = sub_40d850(g_41a828 - g_41a820 + g_4dfea8, g_41a828, stderr);
            *(v4) = *(v4) + (char)v3;
            *((char *)v8 - 125) = *((char *)v8 - 125) >> 4 | *((char *)v8 - 125) * 16;
            *((char *)(v8 ^ 138)) = *((char *)(v8 ^ 138)) + (*((char *)&v8) ^ 138);
            return sub_40f6b0();
        }
        sub_405640("standard input");
    }
    else if (!g_41a85c)
    {
        sub_40f6b0();
    }
    sub_406960(1); /* do not return */
}



// Function: atdir_set @ 0x6d00
extern unsigned long long g_0;
extern unsigned int g_418014;
extern char g_4dc100;

long long atdir_set(unsigned long a0, unsigned long a1)
{
    unsigned long v2;  // rbx
    unsigned long n;  // r13
    unsigned int v4;  // eax
    unsigned long v5;  // rax
    char v0;  // [bp-0x28]

    if (a1)
    {
        v2 = a1;
    }
    else
    {
        a1 = 1;
        v2 = 1;
        a0 = ".";
    }
    n = a1;
    v4 = memcmp(&g_4dc100, a0, n);
    v5 = g_418014;
    if (!v4)
    {
        __unsupported_jumpkind_Ijk_NoDecode()
        if (!(&v0)[v2])
            return v5;
    }
    if ((unsigned int)v5 >= 0)
        close(v5);
    __memcpy_chk(&g_4dc100, a0, n, 0x400);
    (&g_4dc100)[v2] = 0;
    g_418014 = sub_40e360(&g_4dc100, 0x10000);
    return g_418014;
}



// Function: abort_gzip @ 0x8440
extern unsigned long long g_0;

void abort_gzip(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_405530(0);
    sub_406960(1); /* do not return */
}



// Function: inflate_codes @ 0x8460
typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned long long g_0;
extern struct_0 g_413380;
extern unsigned int g_41a004;
extern unsigned int g_41a008;
extern char g_41a00c;
extern char g_43b000;
extern char g_49c000;
extern unsigned int g_4dca0c;
extern unsigned long long g_4dca10;

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

    v7 = g_4dca10;
    iter = g_4dca0c;
    v9 = g_41a004;
    v0 = *((short *)((char *)&g_413380.field_0 + 2 * a2));
    v4 = *((short *)((char *)&g_413380.field_0 + 2 * a3));
    while (1)
    {
LABEL_4084c0:
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
                                v16 = g_41a008;
                                if ((unsigned int)v16 < *((int *)&g_41a00c))
                                {
                                    v17 = (&g_49c000)[v16];
                                    g_41a008 = (unsigned int)v16 + 1;
                                }
                                else
                                {
                                    g_41a004 = v10;
                                    v17 = sub_40d6c0(0) & 0xff;
                                }
                            } while ((iter = (unsigned long)(unsigned long long)((unsigned int)v15 + 8), v7 |= v17 << ((char)(v15 & 0xffffffff) & 63), (unsigned int)v14 > (unsigned int)iter));
                        }
                        v18 = (*((short *)((char *)&g_413380.field_0 + 2 * v14)) & (unsigned short)v7) * 16 + (long long)idx[8];
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
                    (&g_43b000)[v10 & 0xffffffff] = (short)idx[8];
                    if ((unsigned int)v9 != 0x8000)
                        goto LABEL_4084c0;
                }
                else
                {
                    continue;
                    if ((unsigned int)v12 == 15)
                    {
                        g_41a004 = v10;
                        g_4dca10 = v7;
                        g_4dca0c = iter;
                        return 0;
                    }
                    if ((unsigned int)v12 > (unsigned int)iter)
                    {
                        v20 = v12;
                        do
                        {
                            v21 = g_41a008;
                            if ((unsigned int)v21 < *((int *)&g_41a00c))
                            {
                                v22 = (&g_49c000)[v21];
                                g_41a008 = (unsigned int)v21 + 1;
                            }
                            else
                            {
                                g_41a004 = v10;
                                v22 = sub_40d6c0(0) & 0xff;
                            }
                        } while ((v23 = (unsigned long)(unsigned long long)((unsigned int)iter + 8), v7 |= v22 << ((char)(iter & 0xffffffff) & 63), iter = v23, v20 > (unsigned int)iter));
                        v12 = v20;
                        iter = v23;
                    }
                    v24 = (iter & 0xffffffff) - v12;
                    v1 = (short)idx[8];
                    v25 = v7 >> ((char)(v12 & 0xffffffff) & 63);
                    v3 = *((short *)((char *)&g_413380.field_0 + 2 * (v12 & 0xffffffff)));
                    v26 = v24;
                    if (a3 > (unsigned int)v26)
                    {
                        do
                        {
                            v27 = g_41a008;
                            if ((unsigned int)v27 < *((int *)&g_41a00c))
                            {
                                v28 = (&g_49c000)[v27];
                                g_41a008 = (unsigned int)v27 + 1;
                            }
                            else
                            {
                                g_41a004 = v10;
                                v28 = sub_40d6c0(0) & 0xff;
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
                                    v34 = g_41a008;
                                    if ((unsigned int)v34 < *((int *)&g_41a00c))
                                    {
                                        v35 = (&g_49c000)[v34];
                                        g_41a008 = (unsigned int)v34 + 1;
                                    }
                                    else
                                    {
                                        g_41a004 = v10;
                                        v35 = sub_40d6c0(0) & 0xff;
                                    }
                                } while ((v26 = (unsigned long long)((unsigned int)v33 + 8), v25 = (unsigned long long)(v25 | v35 << ((char)(v33 & 0xffffffff) & 63)), v33 = v26, v32 > (unsigned int)v33));
                            }
                            v36 = (*((short *)((char *)&g_413380.field_0 + 2 * v32)) & (unsigned short)v25) * 16 + (long long)v29[8];
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
                            v43 = g_41a008;
                            if ((unsigned int)v43 < *((int *)&g_41a00c))
                            {
                                v44 = (&g_49c000)[v43];
                                v25 = (unsigned int)v42 + 8;
                                g_41a008 = (unsigned int)v43 + 1;
                                v40 |= v44 << ((char)(v42 & 0xffffffff) & 63);
                                if (v41 <= (unsigned int)v25)
                                    break;
                            }
                            else
                            {
                                g_41a004 = v10;
                                v25 = (unsigned int)v42 + 8;
                                v40 |= (sub_40d6c0(0) & 0xff) << ((char)(v42 & 0xffffffff) & 63);
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
                    v45 = (v3 & (unsigned int)v7) + v1 + 21733 - (v38 < i) & 0xffffffff;
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
                            memcpy(&(&g_43b000)[v47 & 0xffffffff], &(&g_43b000)[v48 & 0xffffffff], v50 & 0xffffffff);
                            if ((unsigned int)v10 == 0x8000)
                                goto LABEL_408937;
                            goto LABEL_4088c2;
                        }
                        else
                        {
                            j = (unsigned int)v48 + (unsigned int)v50;
                            v52 = v48 & 0xffffffff;
                            do
                            {
                                (&g_43b000)[v51 + v52] = (&g_43b000)[v52 & 0xffffffff];
                                v52 = (unsigned int)v52 + 1;
                            } while ((unsigned int)v52 != j);
                            v10 = v47 + v50;
                            if ((unsigned int)v10 != 0x8000)
                            {
LABEL_4088c2:
                                if (!(unsigned int)v45)
                                    break;
                            }
                            else
                            {
LABEL_408937:
                                g_41a004 = 0x8000;
                                v10 = 0;
                                sub_40d640();
                                if (!(unsigned int)v45)
                                    break;
                            }
                        }
                    }
                    iter = v2;
                    v7 = v3;
                    goto LABEL_4084c0;
                }
            } while ((g_41a004 = 0x8000, v9 = (unsigned long)0, sub_40d640(), (unsigned int)iter >= a2));
        }
        v54 = iter;
        v55 = g_41a008;
        if ((unsigned int)v55 < *((int *)&g_41a00c))
        {
            v56 = (&g_49c000)[v55];
            g_41a008 = (unsigned int)v55 + 1;
        }
        else
        {
            g_41a004 = v9;
            v56 = sub_40d6c0(0) & 0xff;
        }
        iter = (unsigned int)v54 + 8;
        v7 |= v56 << ((char)(v54 & 0xffffffff) & 63);
    }
}



// Function: huft_build @ 0x89b0
typedef struct struct_2 {
    struct struct_2 *field_0;
    unsigned long long field_8;
} struct_2;

typedef struct struct_4 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_4;

typedef struct struct_7 {
    char padding_0[8];
    unsigned long long field_8;
    unsigned short field_10;
    char padding_12[14];
    unsigned short field_20;
} struct_7;

typedef struct struct_0 {
    char field_0;
    char padding_1[3];
    unsigned int field_4;
} struct_0;

extern unsigned long long g_0;
extern unsigned int g_4dca08;

int huft_build(void)
{
    unsigned int v30;  // esi
    unsigned long v31;  // r10
    char *v40;  // r9
    unsigned int v41;  // r8d
    unsigned int v42;  // r8d
    unsigned int v43;  // esi
    unsigned long idx;  // r9
    int v45;  // eax
    struct_0 *node;  // rdx
    unsigned int v47;  // esi
    unsigned long index;  // rdx
    unsigned int v49;  // esi
    unsigned int *v32;  // rdi
    void* iter;  // rcx
    unsigned int v51;  // edx
    void* iter1;  // rax
    unsigned long v53;  // rax
    unsigned long idx1;  // rdx
    unsigned long idx2;  // rcx
    unsigned long v56;  // rax
    unsigned long long v57;  // r13
    unsigned long long v58;  // r11
    unsigned int l;  // ebx
    unsigned int *i;  // rdx
    unsigned int v60;  // r13d
    struct_2 *v61;  // r9
    struct_2 *v62;  // rsi
    struct_2 *v63;  // rax
    unsigned int v64;  // r10d
    unsigned int v65;  // edx
    unsigned int k;  // r10d
    unsigned long long v67;  // r13
    unsigned long v68;  // r14
    unsigned long long v69;  // rdx
    unsigned long v34;  // rcx
    unsigned long long v70;  // r12
    unsigned int v71;  // ebp
    unsigned int v72;  // ecx
    unsigned int v73;  // ebx
    struct_0 *v74;  // rax
    unsigned int v75;  // ebx
    struct_2 *ptr;  // rcx
    unsigned int v78;  // ecx
    unsigned long v79;  // rdx
    unsigned int v35;  // edx
    struct_4 *v80;  // rdi
    void* v81;  // rbx
    void* v82;  // rbx
    char *v84;  // rdx
    char v85;  // dil
    unsigned int v86;  // edx
    unsigned int v87;  // edx
    unsigned long v89;  // rdx
    unsigned long long v36;  // rax
    unsigned long v90;  // rcx
    struct_2 *v91;  // rcx
    unsigned long v92;  // r8
    unsigned int v93;  // edx
    unsigned long v94;  // rcx
    struct_4 *v97;  // rdx
    unsigned int v98;  // edx
    unsigned int v99;  // r13d
    unsigned int v37;  // ecx
    unsigned long v100;  // rdx
    unsigned long v101;  // rcx
    unsigned long v102;  // rdx
    unsigned long v103;  // rdx
    struct_7 *ptr1;  // rax
    struct_7 *v106;  // rax
    unsigned int v38;  // eax
    unsigned long long v39;  // r15
    unsigned int v0;  // [bp-0x65c]
    struct_2 *v1;  // [bp-0x658]
    unsigned long long v2;  // [bp-0x650]
    unsigned int v3;  // [bp-0x648]
    unsigned int v4;  // [bp-0x644]
    unsigned int v5;  // [bp-0x640]
    int v6;  // [bp-0x63c], Other Possible Types: unsigned int
    unsigned int v7;  // [bp-0x638]
    unsigned int v8;  // [bp-0x634]
    struct_2 *v9;  // [bp-0x630]
    struct_0 *iter2;  // [bp-0x628]
    char *v11;  // [bp-0x620]
    unsigned int v12;  // [bp-0x618]
    unsigned int v13;  // [bp-0x614]
    char *v14;  // [bp-0x610]
    unsigned int v15;  // [bp-0x608]
    unsigned int v16;  // [bp-0x5f0]
    unsigned int v17;  // [bp-0x5ec]
    int v18;  // [bp-0x5e8]
    uint128_t v19;  // [bp-0x5d8]
    uint128_t v20;  // [bp-0x5c8]
    int v21;  // [bp-0x5b8]
    int <0x4089b0[is_18]|Stack bp-0x598, 1 B>;  // [bp-0x598]
    unsigned int v22;  // [bp-0x598]
    unsigned int v23;  // [bp-0x594]
    char v24;  // [bp-0x590]
    char v25;  // [bp-0x550]
    void* v26;  // [bp-0x548]
    char v27;  // [bp-0x4c8]
    unsigned int *v29;  // [bp+0x8]

    v31 = v30;
    v18 = (int)0;
    *((unsigned int *)&(&v21)[16]) = 0;
    v19 = 0;
    v20 = 0;
    *((uint128_t *)&v21) = 0;
    i = v32;
    do
    {
        v34 = *(i);
        i += 1;
        *((int *)((char *)&v18 + 4 * v34)) = *((int *)((char *)&v18 + 4 * v34)) + 1;
    } while (&v32[v31] != i);
    v35 = *((unsigned int *)&v18);
    if (v35 != (unsigned int)v31)
    {
        v36 = 1;
        do
        {
            v37 = v36;
            if (*((int *)((char *)&v18 + 4 * v36)))
            {
                v5 = v36;
                v38 = 1 << ((char)v37 & 31);
                goto LABEL_408a8d;
            }
        } while ((v36 += 1, v36 != 0x11));
        v5 = 0x11;
        v38 = 0x20000;
        v37 = 0x11;
LABEL_408a8d:
        v39 = (*(v29) < v37 ? v5 : *(v29));
        v40 = &v21 - 16;
        v41 = 16;
        do
        {
            v42 = v41;
            if (*((int *)&v40))
            {
                v13 = v42;
                v39 = (v42 < (unsigned int)v39 ? v42 : v39 & 0xffffffff);
                *(v29) = v39;
                if (v37 < v42)
                {
                    idx = v37;
                    v45 = v38 - *((int *)((char *)&v18 + 4 * idx));
                    if (v38 - *((int *)((char *)&v18 + 4 * idx)) < 0)
                        return;
                    node = &v18 + idx * 4;
                    do
                    {
                        v38 = v45 * 2;
                        if (&v18 + ((v42 - v37 - 1 & 0xffffffff) + idx) * 4 == node)
                        {
                            v35 = *((int *)((char *)&v18 + 4 * v42));
                            goto LABEL_408b9a;
                        }
                    } while ((v45 = (int)(v38 - node->field_4), node += 4, v45 >= 0));
LABEL_408b9a:
                    v47 = v38;
                    v16 = v47 - v35;
                    if (v47 - v35 >= 0)
                    {
                        v23 = 0;
                        *((unsigned int *)((char *)&v18 + 4 * v42)) = v38;
                        goto LABEL_408bbd;
                    }
                }
                else
                {
                    index = v42;
                    v49 = v38;
                    v16 = v49 - *((int *)((char *)&v18 + 4 * index));
                    if (v49 - *((int *)((char *)&v18 + 4 * index)) < 0)
                        return;
                    *((unsigned int *)((char *)&v18 + 4 * index)) = v38;
                    v23 = 0;
                    if (v42 == 1)
                        goto LABEL_408beb;
LABEL_408bbd:
                    iter = &v24;
                    v51 = 0;
                    iter1 = &v18 - 4;
                    do
                    {
                        iter1 += 4;
                        iter += 4;
                        v51 += *((int *)((char *)iter1 - 4));
                        *((unsigned int *)((char *)iter - 4)) = v51;
                    } while (&v18 - 8 + (v42 - 2) * 4 != iter1);
                }
            }
        } while ((v40 -= 4, v41 = v42 - 1, v42 != 1));
        if (!(unsigned int)v39)
        {
            v43 = v38;
            *(v29) = 0;
            v16 = v43 - *((unsigned int *)&v18);
            if (v43 - *((unsigned int *)&v18) >= 0)
            {
                *((unsigned int *)&v18) = v38;
                v23 = 0;
                v13 = 0;
                v42 = v41;
                goto LABEL_408bbd;
            }
        }
        else
        {
            *(v29) = 0;
            v39 = 0;
            v13 = 0;
            v42 = v41;
            goto LABEL_408b9a;
        }
LABEL_408beb:
        v53 = 0;
        do
        {
            idx1 = v32[v53];
            if ((unsigned int)idx1)
            {
                idx2 = *((int *)((char *)&<0x4089b0[is_18]|Stack bp-0x598, 1 B> + 4 * idx1));
                *((unsigned int *)&(&v27)[4 * idx2]) = v53;
                *((unsigned int *)((char *)&<0x4089b0[is_18]|Stack bp-0x598, 1 B> + 4 * idx1)) = (unsigned int)idx2 + 1;
            }
        } while ((v53 += 1, v31 != v53));
        v56 = v13;
        v26 = 0;
        v22 = 0;
        if ((unsigned int)v56 < v5)
            return;
        v58 = v57;
        l = 0;
        v11 = &v27;
        v60 = 0;
        v14 = &(&v27)[4 * (&v22)[v56]];
        v62 = v61;
        v6 = -0x1;
        iter2 = &v18 + v5 * 4;
        v17 = v13 + 1;
        v63 = 0;
        do
        {
            v4 = *((int *)&iter2->field_0);
            if (*((int *)&iter2->field_0))
            {
                v64 = -((unsigned int)v39);
                v15 = 1 << ((char)(v5 - 1) & 31);
                v65 = (unsigned int)v39 + v64;
                if (v65 < v5)
                {
                    v12 = 0;
                    k = v65;
                    v67 = v5 - v65 & 0xffffffff;
                    v68 = v6 + 1;
                    v69 = v58;
                    v70 = v13 - v65 & 0xffffffff;
                    do
                    {
                        v8 = k;
                        v6 = v68;
                        l = 1 << ((char)(v67 & 0xffffffff) & 31);
                        v71 = v67;
                        v3 = k - v39;
                        v7 = v67;
                        if (l > v4)
                        {
                            v72 = ((unsigned int)v70 <= (unsigned int)v39 ? (unsigned int)v70 : (unsigned int)v39);
                            if (v72 > (unsigned int)v67)
                            {
                                v71 = (unsigned int)v67 + 1;
                                v73 = l - v4;
                                v74 = iter2;
                                if (v72 > v71)
                                {
                                    do
                                    {
                                        v75 = v73 * 2;
                                    } while (v74->field_4 < v75 && (v71 += 1, v73 = v75 - v74->field_4, v74 += 4, v71 < v72));
                                }
                                l = 1 << ((char)v71 & 31);
                            }
                        }
                        v2 = v69;
                        v1 = v62;
                        v0 = k;
                        v69 = v2;
                        ptr = malloc((l + 1) * 16);
                        if (!ptr && v6)
                        {
                            v81 = v26;
                            if (v81)
                            {
                                do
                                {
                                    v82 = v81;
                                    free(v82 - 16);
                                    v81 = *((long long *)((char *)v82 - 8));
                                } while (*((long long *)((char *)v82 - 8)));
                            }
                        }
                        g_4dca08 = l + g_4dca08 + 1;
                        v63 = ptr + 1;
                        v1->field_0 = v63;
                        v62 = &ptr->field_8;
                        ptr->field_8 = 0;
                        (&v26)[v68] = v63;
                        if ((unsigned int)v68)
                        {
                            v78 = v12;
                            v9 = v63;
                            v79 = _INSERT(v69, 0, (char)v71 + 16);
                            (&v22)[v68] = v78;
                            v80 = *((long long *)&(&v25)[8 * v68]) + (v78 >> ((char)v3 & 31)) * 16;
                            v69 = _INSERT(v79, 1, v39);
                            v80->field_8 = v63;
                            v80->field_0 = v69;
                        }
                        v68 += 1;
                        k = v0 + (unsigned int)v39;
                        v67 -= v39;
                        v70 -= v39;
                    } while (k < v5);
                    v60 = v12;
                    v64 = v8;
                    v58 = v69;
                    goto LABEL_408e4d;
                }
                else
                {
                    v7 = v5 - v64;
LABEL_408e4d:
                    v84 = v11;
                    v85 = 99;
                    if (v14 > v84)
                    {
                        v86 = *((int *)&v27);
                        v11 = v84 + 4;
                        if (v86 < v87)
                        {
                            v85 = (char)((v86 != 0x100) + 15);
                            *((unsigned short *)&v9) = v86;
                        }
                        else
                        {
                            v89 = v86 - v87;
                            v91 = (struct_2 *)_INSERT(v9, 0, *((short *)(v90 + v89 * 2)));
                            v85 = *((char *)(v92 + v89 * 2));
                            v9 = v91;
                        }
                    }
                    v93 = v60 >> ((char)v64 & 31);
                    v94 = v93;
                    if (v93 < l)
                    {
                        do
                        {
                            v58 = _INSERT(_INSERT(v58, 0, v85), 1, (char)v5 - v64 & 0xffffffff);
                            v97 = &v63[v94 & 0xffffffff];
                            v97->field_0 = v58;
                            v97->field_8 = v9;
                            v94 += 1 << ((char)v7 & 31);
                        } while ((unsigned int)v94 < l);
                    }
                    v98 = v15;
                    if (v60 & v15)
                    {
                        do
                        {
                            v60 ^= v98;
                            v98 >>= 1;
                        } while (v98 & v60);
                    }
                    v99 = v60 ^ v98;
                    v100 = v6;
                    if ((&v22)[v100] == ((1 << ((char)v64 & 31)) - 1 & v99))
                        goto LABEL_0x408f71;
                    v101 = v64;
                    v102 = (unsigned int)v100 - 1;
                    do
                    {
                        v101 -= v39;
                        v102 -= 1;
                    } while (((1 << ((char)v101 & 31)) - 1 & v99) != (&v23)[v102]);
                    v6 = v103;
                    return;
                }
            }
        } while ((v5 += 1, iter2 += 4, v5 != v17));
        return;
    }
    else
    {
        ptr1 = malloc(48);
        if (!ptr1)
            return;
        v106 = &ptr1->field_10;
        *((unsigned long long *)(&v106->padding_0[0] - 8)) = 0;
        *((unsigned short *)&v106->padding_0[0]) = 355;
        v106->field_10 = 355;
        g_4dca08 = g_4dca08 + 3;
        v61->field_0 = v106;
        *(v29) = 1;
        return;
        return;
    }
}



// Function: inflate_fixed @ 0x90c0
typedef struct struct_0 {
    char padding_0[288];
    char field_120;
} struct_0;

extern unsigned long long g_0;
extern long long g_4180e0;
extern long long g_418120;
extern char g_1000000010;

unsigned int inflate_fixed(unsigned long a0, struct_0 *idx)
{
    unsigned int *node;  // rbx
    unsigned int *iter;  // rax
    void* v24;  // rbx
    void* v25;  // rbx
    void* v26;  // rbx
    void* v27;  // rbx
    unsigned int *iter1;  // rax
    unsigned int v17;  // r12d
    unsigned int v18;  // r12d
    unsigned int v19;  // eax
    void* v20;  // rbx
    void* v21;  // rbx
    void* v22;  // rbx
    void* v23;  // rbx
    char *v0;  // [bp-0x4d8]
    unsigned int v1;  // [bp-0x4c0]
    unsigned int v2;  // [bp-0x4bc]
    void* v3;  // [bp-0x4b8]
    char v4;  // [bp-0x4b0]
    char v5;  // [bp-0x4a8]
    char v6;  // [bp-0x430]
    char v7;  // [bp-0x268]
    char v8;  // [bp-0xa8]
    char v9;  // [bp-0x48]
    unsigned long long v10;  // [bp-0x40]
    unsigned long long v11;  // [bp-0x38]
    unsigned long long v12;  // [bp-0x30]

    node = &v5;
    iter = &v5;
    do
    {
        *(iter) = 8;
        iter += 1;
    } while (iter != &v7);
    do
    {
        *(iter) = 9;
        iter += 1;
    } while (iter != &v8);
    iter1 = &v8;
    do
    {
        *(iter1) = 7;
        iter1 += 1;
    } while (&v9 != iter1);
    v1 = 7;
    g_1000000010 = 8;
    idx->field_120 = idx->field_120 + *((char *)((void*)&&v5 + 1));
    v10 = 34359738376;
    v11 = 34359738376;
    v12 = 34359738376;
    v0 = &v1;
    v17 = sub_4089b0(&v5);
    if (v17)
        return v17;
    do
    {
        *(node) = 5;
        node += 1;
    } while (node != &v6);
    v2 = 5;
    v18 = sub_4089b0(&v5, 30, 0, &g_418120, &g_4180e0, &v4, &v2);
    if (v18 <= 1)
    {
        v19 = sub_408460(v3, v4, v1, v2);
        if (v19)
            return 1;
        if (v3)
        {
            do
            {
                v20 = v3;
                v21 = *((long long *)((char *)v20 - 8));
                free(v20 - 16);
                v3 = v21;
            } while (*((long long *)((char *)v20 - 8)));
        }
        v22 = (void*)v4;
        if (!v22)
            return v19;
        do
        {
            v23 = v22;
            v24 = *((long long *)((char *)v23 - 8));
            free(v23 - 16);
            v22 = v24;
        } while (*((long long *)((char *)v23 - 8)));
        return v19;
    }
    else
    {
        v25 = v3;
        if (!v25)
            return v18;
        do
        {
            v26 = v25;
            v27 = *((long long *)((char *)v26 - 8));
            free(v26 - 16);
            v25 = v27;
        } while (*((long long *)((char *)v26 - 8)));
        return v18;
    }
}



// Function: inflate_dynamic @ 0x92b0
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned long long g_0;
extern struct_1 g_413380;
extern struct_0 g_4133c0;
extern char g_4133d0;
extern long long g_4180e0;
extern long long g_418120;
extern long long g_418160;
extern long long g_4181a0;
extern unsigned int g_41a004;
extern unsigned int g_41a008;
extern char g_41a00c;
extern char g_49c000;
extern unsigned int g_4dca0c;
extern unsigned long long g_4dca10;

unsigned int inflate_dynamic(void)
{
    unsigned int v18;  // r13d
    unsigned long long v19;  // rbx
    unsigned int v28;  // r14d
    unsigned long long v29;  // r13
    unsigned long v30;  // rax
    unsigned long v31;  // rax
    unsigned int v33;  // r14d
    unsigned long long v34;  // r13
    struct_0 *i;  // r12
    unsigned long long v36;  // rax
    unsigned long v37;  // rax
    unsigned int v20;  // ebp
    unsigned long v38;  // rax
    unsigned long long v41;  // r13
    unsigned int j;  // ebx
    unsigned int k;  // r12d
    unsigned long v45;  // rdx
    unsigned long long v46;  // r14
    unsigned long v47;  // r13
    unsigned long v21;  // rax
    unsigned long v48;  // rax
    unsigned long v49;  // rax
    void* v51;  // rsi
    void* v52;  // rax
    unsigned long v53;  // rcx
    unsigned int v54;  // eax
    unsigned long long v56;  // r14
    unsigned long v57;  // rax
    unsigned long v22;  // rax
    unsigned long v58;  // rax
    unsigned int v60;  // esi
    unsigned long v61;  // rdi
    unsigned int *iter;  // rax
    unsigned long long v63;  // r14
    unsigned long v64;  // rax
    unsigned long v65;  // rax
    unsigned int v67;  // esi
    unsigned int v23;  // r12d
    unsigned long v68;  // rdi
    unsigned int *node;  // rax
    unsigned long long v70;  // r14
    unsigned long v71;  // rax
    unsigned long v72;  // rax
    unsigned int v74;  // esi
    unsigned long v75;  // rdi
    unsigned long long iter1;  // rax
    void* v77;  // rbx
    unsigned long long v24;  // rbx
    void* v78;  // rbx
    unsigned int v80;  // r12d
    void* v81;  // rbx
    void* v82;  // rbx
    unsigned int v84;  // r12d
    void* v85;  // rbx
    void* v87;  // rbx
    unsigned long v25;  // rax
    void* v88;  // rbx
    unsigned int v90;  // r12d
    void* v91;  // rbx
    void* v93;  // rbx
    void* v94;  // rbx
    void* v96;  // rbx
    void* v97;  // rbx
    unsigned long v26;  // rax
    void* v99;  // rbx
    void* v100;  // rbx
    unsigned int v0;  // [bp-0x588]
    unsigned int v1;  // [bp-0x584]
    unsigned int v2;  // [bp-0x580]
    unsigned int v3;  // [bp-0x57c]
    unsigned int v4;  // [bp-0x57c]
    char *v5;  // [bp-0x578]
    unsigned int v6;  // [bp-0x570]
    unsigned int v7;  // [bp-0x56c]
    char *v8;  // [bp-0x568]
    char *v9;  // [bp-0x560]
    unsigned int v10;  // [bp-0x550]
    unsigned int v11;  // [bp-0x54c]
    void* v12;  // [bp-0x548], Other Possible Types: char
    void* v13;  // [bp-0x540]
    char v14;  // [bp-0x538]
    char v15;  // [bp-0x52c]
    char v16;  // [bp-0x50c]

    v18 = g_4dca0c;
    v19 = g_4dca10;
    v20 = g_41a004;
    if (g_4dca0c <= 4)
    {
        v21 = g_41a008;
        if ((unsigned int)v21 < *((int *)&g_41a00c))
        {
            g_41a008 = (unsigned int)v21 + 1;
            v22 = (&g_49c000)[v21];
        }
        else
        {
            v22 = sub_40d6c0(0) & 0xff;
        }
        v18 = g_4dca0c + 8;
        v19 = g_4dca10 | v22 << ((char)g_4dca0c & 63);
    }
    v23 = v18 - 5;
    v24 = v19 >> 5;
    v6 = ((unsigned int)v19 & 31) + 0x101;
    if (v23 <= 4)
    {
        v25 = g_41a008;
        if ((unsigned int)v25 < *((int *)&g_41a00c))
        {
            g_41a008 = (unsigned int)v25 + 1;
            v26 = (&g_49c000)[v25];
        }
        else
        {
            g_41a004 = v20;
            v26 = sub_40d6c0(0) & 0xff;
        }
        v24 |= v26 << ((char)v23 & 63);
        v23 = v18 + 3;
    }
    v28 = v23 - 5;
    v29 = v24 >> 5;
    v7 = ((unsigned int)v24 & 31) + 1;
    if (v28 <= 3)
    {
        v30 = g_41a008;
        if ((unsigned int)v30 < *((int *)&g_41a00c))
        {
            g_41a008 = (unsigned int)v30 + 1;
            v31 = (&g_49c000)[v30];
        }
        else
        {
            g_41a004 = v20;
            v31 = sub_40d6c0(0) & 0xff;
        }
        v29 |= v31 << ((char)v28 & 63);
        v28 = v23 + 3;
    }
    if (v6 > 286 || v7 > 30)
        return 1;
    v33 = v28 - 4;
    v34 = v29 >> 4;
    i = &g_4133c0.field_0;
    v36 = (unsigned int)v29 & 15;
    v0 = (unsigned int)v36 + 4;
    do
    {
        if (v33 <= 2)
        {
            v37 = g_41a008;
            if ((unsigned int)v37 < *((int *)&g_41a00c))
            {
                v38 = (&g_49c000)[v37];
                g_41a008 = (unsigned int)v37 + 1;
            }
            else
            {
                g_41a004 = v20;
                v38 = sub_40d6c0(0) & 0xff;
            }
            v34 |= v38 << ((char)v33 & 63);
            v33 += 8;
        }
        v41 = v34 >> 3;
        v33 -= 3;
        *((unsigned int *)&(&v14)[4 * i->field_0]) = (unsigned int)v34 & 7;
        v34 = v41;
        i = &i->field_4;
    } while (&(&g_4133d0)[4 * v36] != i);
    j = v0;
    if (j != 19)
    {
        do
        {
            *((unsigned int *)&(&v14)[4 * (&g_4133c0.field_0)[j]]) = 0;
            j += 1;
        } while (j != 19);
    }
    v10 = 7;
    v9 = &v12;
    v5 = &v14;
    v8 = &v10;
    k = sub_4089b0(&v14, 19, 19, 0, 0, &v12, &v10);
    if (!k)
    {
        if (v12)
        {
            v45 = v10;
            v3 = 0;
            v0 = 0;
            v1 = v6 + v7;
            v2 = *((short *)((char *)&g_413380.field_0 + 2 * v45));
            v46 = v41;
            v47 = v33;
            do
            {
                if ((unsigned int)v45 > (unsigned int)v47)
                {
                    v48 = g_41a008;
                    if ((unsigned int)v48 < *((int *)&g_41a00c))
                    {
                        v49 = (&g_49c000)[v48];
                        g_41a008 = (unsigned int)v48 + 1;
                    }
                    else
                    {
                        g_41a004 = v20;
                        v49 = sub_40d6c0(0) & 0xff;
                    }
                    v46 |= v49 << ((char)(v47 & 0xffffffff) & 63);
                    v47 = (unsigned int)v47 + 8;
                    continue;
                }
                v51 = (void*)v12;
                v52 = (v2 & (unsigned int)v46) * 16 + v51;
                v13 = v52;
                v53 = (char)v52[1];
                v46 >>= (char)v53 & 63;
                v47 -= v53;
                if (*((char *)v52) == 99)
                {
                    if (v51)
                    {
                        __unsupported_jumpkind_Ijk_NoDecode()
                        v96 = v51 & 0xffffffff;
                        do
                        {
                            v97 = v96;
                            free(v97 - 16);
                            v96 = *((long long *)((char *)v97 - 8));
                        } while (*((long long *)((char *)v97 - 8)));
                        return 2;
                    }
                    else
                    {
                        return 2;
                    }
                }
                v54 = (short)v52[8];
                if (v54 <= 15)
                {
                    v4 = v54;
                    *((unsigned int *)&(&v14)[4 * k]) = v4;
                    v3 = v4;
                    k += 1;
                }
                else if (v4 == 16)
                {
                    v70 = v46;
                    if ((unsigned int)v47 <= 1)
                    {
                        v71 = g_41a008;
                        if ((unsigned int)v71 < *((int *)&g_41a00c))
                        {
                            v72 = (&g_49c000)[v71];
                            g_41a008 = (unsigned int)v71 + 1;
                        }
                        else
                        {
                            g_41a004 = v20;
                            v72 = sub_40d6c0(0) & 0xff;
                        }
                        v70 = v46 | v72 << ((char)(v47 & 0xffffffff) & 63);
                        v47 = (unsigned int)v47 + 8;
                    }
                    v46 = v70 >> 2;
                    v47 = (unsigned int)v47 - 2;
                    v74 = (unsigned int)v70 & 3;
                    if (v74 + v0 + 3 > v1)
                        return 1;
                    v75 = k;
                    iter1 = &v5[4 * v75];
                    do
                    {
                        *((unsigned int *)iter1) = v3;
                        iter1 += 4;
                    } while (&(&v15)[4 * v75 + 4 * v74] != iter1);
                    k = v74 + k + 3;
                }
                else if (v4 != 0x11)
                {
                    v56 = v46;
                    if ((unsigned int)v47 <= 6)
                    {
                        v57 = g_41a008;
                        if ((unsigned int)v57 < *((int *)&g_41a00c))
                        {
                            v58 = (&g_49c000)[v57];
                            g_41a008 = (unsigned int)v57 + 1;
                        }
                        else
                        {
                            g_41a004 = v20;
                            v58 = sub_40d6c0(0) & 0xff;
                        }
                        v56 = v46 | v58 << ((char)(v47 & 0xffffffff) & 63);
                        v47 = (unsigned int)v47 + 8;
                    }
                    v46 = v56 >> 7;
                    v47 = (unsigned int)v47 - 7;
                    v60 = (unsigned int)v56 & 127;
                    if (v60 + v0 + 11 > v1)
                        return 1;
                    v61 = k;
                    iter = &v5[4 * v61];
                    do
                    {
                        *(iter) = 0;
                        iter += 1;
                    } while (&(&v16)[4 * v61 + 4 * v60] != iter);
                    v3 = 0;
                    k = v60 + k + 11;
                }
                else
                {
                    v63 = v46;
                    if ((unsigned int)v47 <= 2)
                    {
                        v64 = g_41a008;
                        if ((unsigned int)v64 < *((int *)&g_41a00c))
                        {
                            v65 = (&g_49c000)[v64];
                            g_41a008 = (unsigned int)v64 + 1;
                        }
                        else
                        {
                            g_41a004 = v20;
                            v65 = sub_40d6c0(0) & 0xff;
                        }
                        v63 = v46 | v65 << ((char)(v47 & 0xffffffff) & 63);
                        v47 = (unsigned int)v47 + 8;
                    }
                    v46 = v63 >> 3;
                    v47 = (unsigned int)v47 - 3;
                    v67 = (unsigned int)v63 & 7;
                    if (v67 + v0 + 3 > v1)
                        return 1;
                    v68 = k;
                    node = &v5[4 * v68];
                    do
                    {
                        *(node) = 0;
                        node += 1;
                    } while (&(&v15)[4 * v68 + 4 * v67] != node);
                    v3 = 0;
                    k = v67 + k + 3;
                }
                v0 = k;
            } while (k < v1);
            v77 = (void*)v12;
            if (v77)
            {
                do
                {
                    v78 = v77;
                    free(v78 - 16);
                    v77 = *((long long *)((char *)v78 - 8));
                } while (*((long long *)((char *)v78 - 8)));
            }
            v10 = 9;
            g_4dca10 = v46;
            g_4dca0c = v47;
            v80 = sub_4089b0(v5, v6, 0x101, &g_4181a0, &g_418160, v9, v8);
            if (!v80)
            {
                v11 = 6;
                v84 = sub_4089b0(&v5[4 * v6], v7, 0, &g_418120, &g_4180e0, &v13, &v11);
                if (v84)
                {
                    if (v84 == 1 && v13)
                    {
                        do
                        {
                            v85 = v13;
                            free(v85 - 16);
                            v13 = *((long long *)((char *)v85 - 8));
                        } while (*((long long *)((char *)v85 - 8)));
                    }
                    v87 = (void*)v12;
                    if (!v87)
                        return v84;
                    do
                    {
                        v88 = v87;
                        free(v88 - 16);
                        v87 = *((long long *)((char *)v88 - 8));
                    } while (*((long long *)((char *)v88 - 8)));
                    return v84;
                }
                else
                {
                    v90 = sub_408460(v12, v13, v10, v11);
                    if (v12)
                    {
                        do
                        {
                            v91 = v12;
                            free(v91 - 16);
                            v12 = *((long long *)((char *)v91 - 8));
                        } while (*((long long *)((char *)v91 - 8)));
                    }
                    v93 = v13;
                    if (!v93)
                        return v90;
                    do
                    {
                        v94 = v93;
                        free(v94 - 16);
                        v93 = *((long long *)((char *)v94 - 8));
                    } while (*((long long *)((char *)v94 - 8)));
                    return v90;
                }
            }
            else if (v80 != 1)
            {
                return v80;
            }
            else
            {
                v81 = (void*)v12;
                if (v81)
                {
                    do
                    {
                        v82 = v81;
                        free(v82 - 16);
                        v81 = *((long long *)((char *)v82 - 8));
                    } while (*((long long *)((char *)v82 - 8)));
                }
            }
        }
        else
        {
            return 2;
        }
    }
    else
    {
        if (k != 1)
            return k;
        v99 = (void*)v12;
        if (v99)
        {
            do
            {
                v100 = v99;
                free(v100 - 16);
                v99 = *((long long *)((char *)v100 - 8));
            } while (*((long long *)((char *)v100 - 8)));
        }
    }
    return 1;
}



// Function: inflate @ 0x9a50
typedef struct struct_0 {
    char padding_0[288];
    char field_120;
} struct_0;

extern unsigned long long g_0;
extern unsigned int g_41a004;
extern unsigned int g_41a008;
extern char g_41a00c;
extern char g_43b000;
extern char g_49c000;
extern unsigned int g_4dca08;
extern unsigned int g_4dca0c;
extern unsigned long long g_4dca10;

unsigned int inflate(void)
{
    unsigned int v3;  // r14d
    unsigned long long v4;  // rbx
    unsigned long v13;  // rax
    unsigned int v14;  // eax
    unsigned int v15;  // rdi
    struct_0 *v16;  // rsi
    unsigned int v17;  // eax
    unsigned int v18;  // r13d
    unsigned int v19;  // eax
    unsigned long long v20;  // r14
    unsigned int v21;  // ebx
    unsigned int v22;  // ecx
    unsigned long v5;  // rax
    unsigned long v23;  // rax
    unsigned long v24;  // rax
    unsigned int v26;  // ecx
    unsigned long long v27;  // r15
    unsigned long v28;  // rax
    unsigned long v29;  // rax
    unsigned int v30;  // r14d
    unsigned long long v31;  // r15
    unsigned int v32;  // esi
    unsigned long v6;  // rax
    unsigned int v33;  // ebx
    unsigned int v34;  // r13d
    unsigned int v35;  // ebx
    unsigned long v36;  // rax
    unsigned long v37;  // rax
    unsigned long long v7;  // rax
    unsigned int v8;  // r15d
    unsigned int v9;  // r12d
    unsigned long long v10;  // rbx
    unsigned int v11;  // r15d
    unsigned long v12;  // rax
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]

    v3 = 0;
    v4 = 0;
    g_41a004 = 0;
    g_4dca0c = 0;
    g_4dca10 = 0;
    while (1)
    {
        g_4dca08 = 0;
        if (!v3)
        {
            v5 = g_41a008;
            if ((unsigned int)v5 < *((int *)&g_41a00c))
            {
                v6 = (&g_49c000)[v5];
                g_41a008 = (unsigned int)v5 + 1;
            }
            else
            {
                v6 = sub_40d6c0(0) & 0xff;
            }
            v7 = v6 | v4;
            v8 = 5;
            v9 = (unsigned int)v7 & 1;
            v10 = v7 >> 1;
        }
        else
        {
            v11 = v3 - 1;
            v10 = v4 >> 1;
            v9 = (unsigned int)v4 & 1;
            if (v11 <= 1)
            {
                v12 = g_41a008;
                if ((unsigned int)v12 < *((int *)&g_41a00c))
                {
                    v13 = (&g_49c000)[v12];
                    g_41a008 = (unsigned int)v12 + 1;
                }
                else
                {
                    v13 = sub_40d6c0(0) & 0xff;
                }
                v8 = v3 + 5;
                v10 |= v13 << ((char)v11 & 63);
            }
            else
            {
                v8 = v3 - 3;
            }
        }
        g_4dca0c = v8;
        v14 = (unsigned int)v10 & 3;
        g_4dca10 = v10 >> 2;
        if (v14 == 2)
        {
            v17 = sub_4092b0();
        }
        else if (v14)
        {
            if (v14 != 1)
                return 2;
            v17 = sub_4090c0(v15, v16);
        }
        else
        {
            v18 = g_41a004;
            v19 = v8 & 7;
            v1 = v19;
            v20 = g_4dca10 >> ((char)v19 & 63);
            v21 = v8 - v19;
            if (v21 <= 15)
            {
                v22 = v21;
                while (1)
                {
                    v23 = g_41a008;
                    if ((unsigned int)v23 < *((int *)&g_41a00c))
                    {
                        v24 = (&g_49c000)[v23];
                        g_41a008 = (unsigned int)v23 + 1;
                        v20 |= v24 << ((char)v22 & 63);
                        v22 += 8;
                        if (v22 > 15)
                            break;
                    }
                    else
                    {
                        v0 = v22;
                        g_41a004 = v18;
                        v26 = v0;
                        v22 = v26 + 8;
                        v20 |= (sub_40d6c0(0) & 0xff) << ((char)v26 & 63);
                        if (v22 > 15)
                            break;
                    }
                }
                v21 = v21 + (v1 - v8 + 15 & 0xfffffff8) + 8;
            }
            v27 = v20 >> 16;
            v3 = v21 - 16;
            v0 = (unsigned short)v20;
            if (v3 <= 15)
            {
                do
                {
                    v28 = g_41a008;
                    if ((unsigned int)v28 < *((int *)&g_41a00c))
                    {
                        v29 = (&g_49c000)[v28];
                        g_41a008 = (unsigned int)v28 + 1;
                    }
                    else
                    {
                        g_41a004 = v18;
                        v29 = sub_40d6c0(0) & 0xff;
                    }
                } while ((v30 = v3 + 8, v27 = (unsigned long long)(v27 | v29 << ((char)v3 & 63)), v3 += 8, v3 <= 15));
                v3 = v21 + ((unsigned int)(31 - v21) & 0xfffffff8) - 8;
            }
            if (v0 != (~((unsigned int)v27) & 0xffff))
                return 1;
            v31 = v27 >> 16;
            v32 = v0 - 16;
            v33 = v32 - 1;
            v34 = v18;
            if (v32)
            {
                while (1)
                {
                    v35 = v33;
                    if (v3 <= 7)
                    {
                        v36 = g_41a008;
                        if ((unsigned int)v36 < *((int *)&g_41a00c))
                        {
                            v37 = (&g_49c000)[v36];
                            g_41a008 = (unsigned int)v36 + 1;
                        }
                        else
                        {
                            g_41a004 = v18;
                            v37 = sub_40d6c0(0) & 0xff;
                        }
                        v31 |= v37 << ((char)v3 & 63);
                    }
                    else
                    {
                        v3 -= 8;
                    }
                    v34 = v18 + 1;
                    (&g_43b000)[v18] = v31;
                    if (v34 != 0x8000)
                    {
                        v31 >>= 8;
                        v33 = v35 - 1;
                        v18 = v34;
                        if (v35 < 1)
                            break;
                    }
                    else
                    {
                        g_41a004 = 0x8000;
                        v18 = 0;
                        v31 >>= 8;
                        sub_40d640();
                        v33 = v35 - 1;
                        v34 = v18;
                        if (v35 < 1)
                            break;
                    }
                }
            }
            g_41a004 = v34;
            g_4dca10 = v31;
            g_4dca0c = v3;
            if (v9)
                break;
            else
                goto LABEL_409da9;
        }
        if (v17)
            return v17;
        v3 = g_4dca0c;
        if (v9)
            break;
LABEL_409da9:
        v4 = g_4dca10;
    }
    if (v3 > 7)
    {
        g_41a008 = g_41a008 + ~(v3 - 8 >> 3);
        g_4dca0c = v3 & 7;
    }
    sub_40d640();
    return 0;
}



// Function: init_block @ 0x9df0
extern unsigned long long g_0;
extern unsigned long long g_4dca38;
extern unsigned long long g_4dca40;
extern char g_4dca48;
extern char g_4dca49;
extern unsigned int g_4dca4c;
extern unsigned int g_4dca50;
extern unsigned int g_4dca54;
extern unsigned long long g_4de9e0;
extern unsigned long long g_4dea2c;
extern unsigned long long g_4def80;
extern unsigned long long g_4deff8;
extern unsigned long long g_4df080;
extern unsigned short g_4df480;
extern unsigned long long g_4df4f8;

unsigned int init_block(void)
{
    unsigned long long iter;  // rax
    unsigned long long node;  // rax
    unsigned long long iter1;  // rax

    iter = &g_4df080;
    do
    {
        iter += 4;
        *((unsigned short *)(iter - 4)) = 0;
    } while (iter != &g_4df4f8);
    node = &g_4def80;
    do
    {
        node += 4;
        *((unsigned short *)(node - 4)) = 0;
    } while (node != &g_4deff8);
    __unsupported_jumpkind_Ijk_NoDecode()
    iter1 = &g_4de9e0;
    do
    {
        iter1 += 4;
        *((unsigned short *)(iter1 - 4)) = 0;
    } while (iter1 != &g_4dea2c);
    g_4dca38 = 0;
    g_4df480 = 1;
    g_4dca40 = 0;
    g_4dca4c = 0;
    g_4dca50 = 0;
    g_4dca54 = 0;
    g_4dca49 = 0;
    g_4dca48 = 1;
    return 1;
}



// Function: pqdownheap @ 0x9ea0
extern unsigned long long g_0;
extern char g_4dde60;
extern char g_4de0a4;
extern int g_4de0c0[4];

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
    v0 = g_4de0c0[idx];
    if (v3 <= *((int *)&g_4de0a4))
    {
        idx1 = g_4de0c0[idx];
        v5 = *((short *)(a0 + idx1 * 4));
        do
        {
            index = g_4de0c0[v3];
            v7 = v3;
            v8 = *((short *)(a0 + index * 4));
            v9 = index;
            if (*((int *)&g_4de0a4) <= v3)
                continue;
            v7 += 1;
            idx2 = g_4de0c0[v7];
            if (*((short *)(a0 + idx2 * 4)) >= v8)
            {
                if (v12 != v8)
                {
                    v7 = v3;
                    continue;
                }
                if ((&g_4dde60)[idx2] <= (&g_4dde60)[index])
                {
                    v9 = idx2;
                    if ((&g_4dde60)[idx2] <= (&g_4dde60)[index])
                        goto LABEL_409f6f;
                }
                else
                {
                    v9 = v9;
                    if ((&g_4dde60)[idx2] <= (&g_4dde60)[index])
                    {
LABEL_409f6f:
LABEL_409f70:
                        continue;
                    }
                }
                v7 = v3;
                goto LABEL_409f70;
            }
            else
            {
                v12 = *((short *)(a0 + idx2 * 4));
                v9 = idx2;
                v8 = v12;
            }
        } while (v5 >= v8 && (v5 != v8 || (&g_4dde60)[idx1] > (&g_4dde60)[v9]) && (v3 = v7 * 2, g_4de0c0[idx] = (int)v9, idx = (unsigned long)(long long)(int)v7, v3 <= *((int *)&g_4de0a4)));
    }
    g_4de0c0[idx] = v0;
    return v0;
}



// Function: scan_tree @ 0x9f80
extern unsigned long long g_0;
extern unsigned short g_4de9e0[4];
extern unsigned short g_4dea20;
extern unsigned short g_4dea24;
extern unsigned short g_4dea28;

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
                goto LABEL_40a005;
            if ((unsigned int)v7 < v3)
            {
                g_4de9e0[2 * v9] = g_4de9e0[2 * v9] + (unsigned short)v7;
                if (!v1)
                    goto LABEL_40a050;
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
                g_4de9e0[2 * v9] = g_4de9e0[2 * v9] + 1;
            g_4dea20 = g_4dea20 + 1;
        }
        else if ((unsigned int)v7 <= 10)
        {
            g_4dea24 = g_4dea24 + 1;
        }
        else
        {
            g_4dea28 = g_4dea28 + 1;
        }
        if (!v1)
        {
LABEL_40a050:
            v8 = v9;
            v3 = 3;
            v4 = 138;
            v7 = 0;
LABEL_40a005:
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
            goto LABEL_40a005;
        }
    }
}



// Function: build_tree @ 0xa090
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    char field_4;
} struct_0;

extern unsigned long long g_0;
extern unsigned long long g_4dca38;
extern unsigned long long g_4dca40;
extern char g_4dde60;
extern int g_4de0a0;
extern unsigned int g_4de0a4;
extern int g_4de0c0[4];
extern unsigned int g_4de0c4;
extern void g_4de9c0;
extern void g_4de9e0;

void build_tree(void* idx)
{
    struct_0 *v9;  // rbx
    unsigned long v10;  // rdi
    char v19;  // r9b
    unsigned long v20;  // r8
    struct_0 *v21;  // r14
    unsigned long long v22;  // rcx
    unsigned long long v23;  // rax
    unsigned long v24;  // rax
    unsigned long v25;  // rax
    int v26;  // edi
    int v27;  // r13d
    int i;  // r13d
    unsigned long v11;  // r15
    unsigned long idx1;  // r13
    unsigned long long idx2;  // r15
    struct_0 *v31;  // r14
    unsigned long iter;  // rbx
    unsigned long v33;  // rdx
    unsigned short *v34;  // r10
    unsigned short *v35;  // rdi
    unsigned short v36;  // si
    unsigned short v37;  // ax
    struct_0 *v38;  // rbx
    char v12;  // dl
    unsigned int v39;  // edx
    unsigned long long v40;  // rax
    void* v41;  // rdi
    int v42;  // r9d
    unsigned int v43;  // r8d
    void* iter1;  // rdx
    unsigned long v45;  // rcx
    unsigned int v46;  // r14d
    char v47;  // sil
    unsigned long v48;  // r13
    unsigned long iter2;  // rax
    int *v49;  // r10
    unsigned long v50;  // r12
    unsigned long v51;  // r15
    unsigned long v52;  // rcx
    int v53;  // edx
    unsigned long long v54;  // rcx
    unsigned short *v55;  // r11
    unsigned long v56;  // rax
    unsigned long index;  // rsi
    unsigned long long v58;  // rbp
    int v14;  // esi
    unsigned long v59;  // rax
    unsigned int v60;  // r12d
    unsigned int v61;  // r11d
    unsigned long v62;  // r13
    unsigned long v63;  // r14
    unsigned long v64;  // r10
    void* v65;  // rbx
    unsigned short l;  // cx
    unsigned int v67;  // r12d
    unsigned int v68;  // r12d
    int v15;  // ecx
    unsigned long v69;  // rsi
    unsigned int v70;  // eax
    unsigned short v71;  // si
    void* v72;  // rdx
    unsigned long v73;  // r13
    char v74;  // r14b
    int v75;  // edx
    unsigned int v76;  // esi
    unsigned long v77;  // rax
    unsigned long v78;  // rcx
    int v16;  // edi
    unsigned short *v79;  // r12
    unsigned long v80;  // r15
    unsigned long v81;  // rcx
    unsigned long long v82;  // rdx
    unsigned long long v83;  // rax
    unsigned int v84;  // eax
    unsigned long v85;  // rax
    struct_0 *node;  // rbx
    unsigned int v87;  // esi
    unsigned long v88;  // rax
    unsigned long v17;  // r11
    unsigned short v89;  // di
    unsigned long v18;  // rdx
    struct_0 *v0;  // [bp-0x98], Other Possible Types: unsigned long, unsigned int
    int *k;  // [bp-0x90], Other Possible Types: unsigned int *
    char v2;  // [bp-0x86], Other Possible Types: unsigned short
    int v3;  // [bp-0x84]
    int v4;  // [bp-0x80], Other Possible Types: unsigned long
    int *v5;  // [bp-0x78], Other Possible Types: void*
    unsigned long v6;  // [bp-0x70]
    char v7;  // [bp-0x68]

    v9 = *((long long *)idx);
    v5 = idx;
    v10 = (long long)idx[8];
    g_4de0a4 = 0;
    g_4de0a0 = 573;
    v6 = v10;
    v0 = (int)idx[28];
    if (v0 > 0)
    {
        v11 = v0;
        v12 = 0;
        iter2 = 0;
        v14 = 0;
        v15 = -0x1;
        while (1)
        {
            if ((&v9->field_0)[2 * iter2])
            {
                v14 += 1;
                (&g_4dde60)[iter2] = 0;
                v15 = iter2;
                g_4de0c0[v14] = iter2;
                iter2 += 1;
                v12 = 1;
                if (iter2 == v11)
                    break;
            }
            else
            {
                (&v9->field_2)[2 * iter2] = 0;
                iter2 += 1;
                if (iter2 == v11)
                    break;
            }
        }
        v3 = v15;
        if (v12)
        {
            g_4de0a4 = v14;
            if (v14 <= 1)
                goto LABEL_40a151;
            v4 = v14;
            k = &g_4de0c4;
        }
        else
        {
            v14 = 0;
            goto LABEL_40a151;
        }
    }
    else
    {
        v3 = -0x1;
        v11 = v0;
        v14 = 0;
LABEL_40a151:
        v16 = v3;
        v17 = v6;
        v18 = v14;
        v19 = 0;
        v20 = g_4dca38;
        k = &g_4de0c4;
        do
        {
            if (v16 > 1)
            {
                v21 = v9;
                v22 = 0;
                v23 = 0;
            }
            else
            {
                v16 += 1;
                v23 = v16;
                v22 = v16 * 4;
                v21 = (char *)v9 + v22;
            }
            (&g_4de0c4)[v18] = v23;
            v21->field_0 = 1;
            (&g_4dde60)[v23] = 0;
            if (v17)
            {
                v19 = 1;
                v20 -= *((short *)(v17 + v22 + 2));
            }
            v18 += 1;
        } while ((unsigned int)v18 <= 1);
        v3 = v16;
        v24 = 1 - v14;
        v25 = -(v24);
        v26 = v14 + (v14 <= 1 ? (unsigned int)v24 : 0) + 1;
        if (1 < v14)
            v25 = 0;
        v4 = v26;
        g_4de0a4 = v26;
        g_4dca40 = g_4dca40 + v25 - 1;
        if (v19)
            g_4dca38 = v20;
    }
    *((int *)&v5[36]) = v3;
    v27 = v4 >> 1;
    do
    {
        i = v27;
        sub_409ea0(v9, i);
        v27 = i - 1;
    } while (i != 1);
    idx1 = v4;
    idx2 = 571;
    v31 = v9;
    iter = v11;
    v2 = (unsigned short)v0 + idx1;
    do
    {
        g_4de0a4 = idx1 - 1;
        v0 = g_4de0c4;
        g_4de0c4 = g_4de0c0[idx1];
        sub_409ea0(v31, 1);
        v33 = v0;
        g_4de0a0 = idx2;
        v34 = &(&v31->field_0)[2 * g_4de0c4];
        k[idx2] = v33;
        v35 = &(&v31->field_0)[2 * v33];
        v36 = *(v34) + *(v35);
        g_4de0c0[idx2] = g_4de0c4;
        (&v31->field_0)[2 * iter] = v36;
        idx2 -= 2;
        (&g_4dde60)[iter] = ((&g_4dde60)[g_4de0c4] <= (&g_4dde60)[v33] ? (&g_4dde60)[v33] + 1 : (&g_4dde60)[g_4de0c4] + 1);
        v37 = v2 - idx1;
        idx1 -= 1;
        v34[1] = v37;
        v35[1] = v37;
        g_4de0c4 = iter;
        iter += 1;
        sub_409ea0(v31, 1);
    } while ((unsigned int)idx1 > 1);
    v38 = v31;
    v39 = -(v4) * 2;
    v40 = (v4 <= 1 ? 571 : v39 + 575);
    v41 = v5;
    g_4de0a0 = (v4 <= 1 ? 573 : v39 + 577) - 3;
    g_4de0c0[g_4de0a0] = g_4de0c4;
    v42 = (int)v41[36];
    v43 = (int)v41[32];
    v4 = (long long)v41[16];
    v0 = _INSERT(v0 CONCAT 0, 0, (int)v41[24]);
    iter1 = &g_4de9c0;
    do
    {
        iter1 += 2;
        *((unsigned short *)((char *)iter1 - 2)) = 0;
    } while (iter1 != &g_4de9e0);
    (&v38->field_2)[2 * g_4de0c4] = 0;
    if ((unsigned int)v40 <= 572)
    {
        v45 = (int)v40;
        v2 = 0;
        v46 = 0;
        v5 = &g_4de0c0[0];
        v47 = 0;
        v48 = v6;
        v49 = &g_4de0c0[v45];
        v50 = g_4dca40;
        v51 = g_4dca38;
        k = &k[572 + v45 + -1 * v40];
        do
        {
            v52 = *(v49);
            v53 = v52;
            v54 = v52 * 4;
            v55 = (char *)v38 + v54;
            v56 = (&v38->field_2)[2 * v55[1]] + 1;
            if (v43 < (unsigned int)v56)
            {
                v46 += 1;
                v56 = v43;
            }
            v55[1] = v56;
            if (v42 >= v53)
            {
                index = (int)v56;
                v58 = 0;
                *((short *)&(&g_4de9c0)[2 * index]) = *((short *)&(&g_4de9c0)[2 * index]) + 1;
                if ((unsigned int)v0 <= v53)
                {
                    v58 = *((int *)(v4 + (v53 - (unsigned int)v0) * 4));
                    index = v56 + v58;
                }
                v59 = *(v55);
                v50 += v59 * index;
                v47 = 1;
                if (v48)
                {
                    v2 = 1;
                    v51 += v59 * (*((short *)(v48 + v54 + 2)) + v58);
                }
            }
            v49 += 1;
        } while (k != v49);
        if (v2)
            g_4dca38 = v51;
        if (v47)
            g_4dca40 = v50;
        if (v46)
        {
            v60 = v46 - 2;
            v61 = v43 - 1;
            v62 = v43;
            v0 = v38;
            v63 = v61;
            v64 = v62;
            v65 = &(&g_4de9c0)[2 * v62];
            l = *((short *)&(&g_4de9c0)[2 * v63]);
            v67 = v60;
            v68 = v60;
            if (!l)
            {
LABEL_40a4e8:
                v70 = v61;
                v72 = v65;
                do
                {
                    l = *((short *)((char *)v72 - 4));
                    v69 = v70;
                    v72 -= 2;
                    v70 -= 1;
                } while (!l);
LABEL_40a503:
                *((unsigned short *)&(&g_4de9c0)[2 * v70]) = l - 1;
                *((short *)&(&g_4de9c0)[2 * v69]) = *((short *)&(&g_4de9c0)[2 * v69]) + 2;
                v71 = *((short *)&(&g_4de9c0)[2 * v62]) - 1;
                *((unsigned short *)&(&g_4de9c0)[2 * v62]) = v71;
                if (v67 != v60 - (v46 - 1 & 0xfffffffe))
                {
                    l = *((short *)&(&g_4de9c0)[2 * v63]);
                    v67 -= 2;
                    v68 = v67;
                    if (!l)
                        goto LABEL_40a4e8;
                    goto LABEL_40a535;
                }
                else
                {
                    v38 = v0;
                    if (v43)
                    {
                        v73 = g_4dca40;
                        v74 = 0;
                        v75 = 573;
                        while (1)
                        {
                            v76 = v71;
                            if (v76)
                            {
                                v77 = v75 - 1;
                                while (1)
                                {
                                    v78 = v5[v77];
                                    v75 = v77;
                                    if (v42 >= (unsigned int)v78)
                                    {
                                        v79 = &(&v38->field_0)[2 * v78];
                                        v80 = v79[1];
                                        if ((unsigned int)v80 != v43)
                                        {
                                            v81 = *(v79);
                                            v79[1] = v43;
                                            v74 = 1;
                                            v73 += (v64 - v80) * v81;
                                        }
                                        v77 -= 1;
                                        v76 -= 1;
                                        if (v76 == 1)
                                            break;
                                    }
                                    else
                                    {
                                        v77 -= 1;
                                    }
                                }
                            }
                            v43 = v61;
                            v64 -= 1;
                            if (!v61)
                                break;
                            v71 = *((short *)&(&g_4de9c0)[2 * v64]);
                            v61 -= 1;
                        }
                        if (v74)
                            g_4dca40 = v73;
                    }
                }
            }
            else
            {
LABEL_40a535:
                v67 = v68;
                v69 = v43;
                v70 = v61;
                goto LABEL_40a503;
            }
        }
    }
    v82 = 0;
    v83 = 0;
    do
    {
        v84 = _INSERT(v83, 0, (unsigned short)v83 + *((short *)&(&g_4de9c0)[v82]));
    } while ((v83 = (unsigned long long)_INSERT((unsigned long long)(v84 * 2), 0, (char)v84 * 2 + 0x11), v82 += 2, v82 != 30));
    v85 = v3;
    if ((unsigned int)v85 == 0xffffffff)
        return;
    node = v38;
    do
    {
        v87 = node->field_2;
        if (v87)
        {
            v88 = v87;
            v89 = *((short *)&(&v7)[2 * v88]);
            *((unsigned short *)&(&v7)[2 * v88]) = v89 + 1;
            node->field_0 = sub_4044a0(v89, v87);
        }
    } while ((node += 4, v38 + v85 * 4 + 4 != node));
    return;
}



// Function: send_tree @ 0xa6b0
typedef struct struct_0 {
    char padding_0[2];
    unsigned short field_2;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned long long g_0;
extern struct_1 g_4de9e0;
extern unsigned short g_4dea20;
extern unsigned short g_4dea22;
extern unsigned short g_4dea24;
extern unsigned short g_4dea26;
extern unsigned short g_4dea28;
extern unsigned short g_4dea2a;

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
                sub_4043b0(*((short *)((char *)&g_4de9e0.field_0 + 4 * v9)), *((short *)((char *)&g_4de9e0.field_0 + 4 * v9 + 2)));
                v8 = v0;
            }
            sub_4043b0(g_4dea20, g_4dea22);
            v7 = sub_4043b0(v8 - 3, 2);
        }
        else if (v8 <= 10)
        {
            sub_4043b0(g_4dea24, g_4dea26);
            v7 = sub_4043b0(v0 - 2, 3);
        }
        else
        {
            sub_4043b0(g_4dea28, g_4dea2a);
            v7 = sub_4043b0(v0 - 10, 7);
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



// Function: compress_block @ 0xa870
typedef struct struct_2 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_2;

typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned long long g_0;
extern struct_0 g_418240;
extern struct_0 g_4182c0;
extern struct_2 g_44b000;
extern char g_49c000;
extern unsigned int g_4dca54;
extern char g_4dca60;
extern unsigned int g_4dda60[4];
extern unsigned int g_4ddae0[4];
extern char g_4ddb60;
extern char g_4ddd60;

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

    if (!g_4dca54)
        return sub_4043b0(a0[0x200], a0[513]);
    v4 = 0;
    v5 = 0;
    v6 = 0;
    v7 = 0;
    while (1)
    {
        if (!((char)v7 & 7))
        {
            v8 = v5 + 1;
            v4 = (&g_4dca60)[v5];
            v5 = v8;
        }
        v9 = (unsigned int)v7 + 1;
        v10 = (&g_49c000)[v7];
        if (!((char)v4 & 1))
        {
            v7 = sub_4043b0(a0[2 * v10], a0[1 + 2 * v10]);
            goto LABEL_40a8c1;
        }
        else
        {
            v0 = (char)v10;
            v1 = (&g_4ddd60)[v10];
            v11 = &a0[0x202 + 2 * v1];
            sub_4043b0(*(v11), v11[1]);
            v12 = v1;
            if ((&g_4182c0.field_0)[v12])
                sub_4043b0(v0 - g_4ddae0[v12], (&g_4182c0.field_0)[v12]);
            v1 = (unsigned int)v6 + 1;
            v2 = (&g_44b000.field_0)[v6];
            v0 = (v2 <= 0xff ? (&g_4ddb60)[v2 & 0xffff] : (&g_4ddb60)[0x100 + ((v2 & 0xffff) >> 7)]);
            sub_4043b0(*((short *)(a1 + v0 * 4)), *((short *)(a1 + v0 * 4 + 2)));
            v7 = v0;
            v6 = v1;
            if (!(&g_418240.field_0)[v7])
            {
LABEL_40a8c1:
                v4 = _INSERT(v4, 0, (char)v4 >> 1);
                if (g_4dca54 <= v9)
                    break;
                else
                    goto LABEL_40a8cf;
            }
            else
            {
                v4 = _INSERT(v4, 0, (char)v4 >> 1);
                v7 = sub_4043b0(v2 - g_4dda60[v7], (&g_418240.field_0)[v7]);
                if (g_4dca54 <= v9)
                    break;
LABEL_40a8cf:
                __unsupported_jumpkind_Ijk_NoDecode()
                __unsupported_jumpkind_Ijk_NoDecode()
            }
        }
    }
    return sub_4043b0(a0[0x200], a0[513]);
}



// Function: ct_init @ 0xaa20
typedef struct struct_2 {
    unsigned short field_0;
    unsigned short field_2;
} struct_2;

typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned long long g_0;
extern struct_0 g_418240;
extern struct_0 g_4182c0;
extern unsigned long long g_4dca20;
extern unsigned long long g_4dca28;
extern unsigned long long g_4dca30;
extern unsigned int g_4dda60;
extern unsigned int g_4ddae0;
extern char g_4ddb60;
extern char g_4ddb61;
extern char g_4ddd60;
extern char g_4ddd61;
extern unsigned long long g_4de9c0;
extern unsigned short g_4de9ce;
extern unsigned short g_4de9d0;
extern unsigned short g_4de9d2;
extern unsigned long long g_4de9e0;
extern unsigned short g_4dea80;
extern unsigned short g_4dea82;
extern struct_2 g_4deb00;
extern unsigned short g_4deb02;
extern unsigned short g_4ded42;
extern unsigned short g_4def02;
extern unsigned short g_4def62;
extern unsigned short g_4def66;
extern unsigned short g_4def6a;
extern unsigned short g_4def6e;
extern unsigned short g_4def72;
extern unsigned short g_4def76;
extern unsigned short g_4def7a;
extern unsigned short g_4def7e;
extern struct_2 g_4def80;

unsigned int ct_init(unsigned long a0)
{
    unsigned long v3;  // rsi
    unsigned long long idx;  // rdi
    unsigned long long v13;  // r11
    unsigned long v14;  // rdx
    unsigned long iter;  // rax
    unsigned long long v16;  // r10
    unsigned long long v17;  // r12
    unsigned long v18;  // rdx
    unsigned long iter1;  // rax
    unsigned long long iter2;  // rax
    unsigned short *j;  // rax
    struct_2 *node;  // rbx
    unsigned long long v5;  // r8
    unsigned short *k;  // rax
    unsigned short *l;  // rax
    unsigned long long m;  // rax
    unsigned int v26;  // esi
    unsigned long idx1;  // rax
    unsigned short v28;  // di
    unsigned long long index;  // rbx
    unsigned long v30;  // fs
    char v6;  // cl
    unsigned long long v7;  // r9
    unsigned long v8;  // rdx
    char *i0;  // rax
    unsigned long long v10;  // rdi
    unsigned long long v11;  // r10
    char v12;  // cl
    char v0;  // [bp-0x58]
    unsigned int v31;  // [bp-0x56]
    unsigned long v1;  // [bp-0x30]

    v3 = g_4dea82;
    g_4dca20 = a0;
    g_4dca28 = 0;
    g_4dca30 = 0;
    if (g_4dea82)
        return v1 - *((long long *)(40 + v30));
    idx = 0;
    v5 = 0;
    do
    {
        v6 = (&g_4182c0.field_0)[idx];
        (&g_4ddae0)[idx] = v5;
        v7 = 1 << (v6 & 31);
        if ((unsigned int)v7 > 0)
        {
            v8 = (int)v5;
            i0 = &(&g_4ddd60)[v8];
            do
            {
                *(i0) = idx & 0xffffffff;
                i0 += 1;
            } while (i0 != &(&g_4ddd61)[v8 + (v7 - 1 & 0xffffffff)]);
            v5 += v7;
        }
    } while ((idx += 1, idx != 28));
    v10 = 0;
    v11 = 0;
    (&g_4ddd60)[(unsigned int)(v5 - 1)] = 28;
    do
    {
        v12 = (&g_418240.field_0)[v10];
        (&g_4dda60)[v10] = v11;
        v13 = 1 << (v12 & 31);
        if ((unsigned int)v13 > 0)
        {
            v14 = (int)v11;
            iter = &(&g_4ddb60)[v14];
            do
            {
                *((char *)iter) = v10 & 0xffffffff;
                iter += 1;
            } while (iter != &(&g_4ddb61)[v14 + (v13 - 1 & 0xffffffff)]);
            v11 += v13;
        }
    } while ((v10 += 1, v10 != 16));
    v16 = (int)((unsigned int)v11) >> 7;
    do
    {
        (&g_4dda60)[v10] = (unsigned int)v16 * 128;
        v17 = 1 << ((char)((&g_418240.field_0)[v10] - 7) & 31);
        if ((unsigned int)v17 > 0)
        {
            v18 = (int)v16;
            iter1 = &(&g_4ddb60)[v18];
            do
            {
                *((char *)(iter1 + 0x100)) = v10 & 0xffffffff;
                iter1 += 1;
            } while (iter1 != &(&g_4ddb61)[v18 + (v17 - 1 & 0xffffffff)]);
            v16 += v17;
        }
    } while ((v10 += 1, v10 != 30));
    iter2 = &g_4de9c0;
    do
    {
        iter2 += 2;
        *((unsigned short *)(iter2 - 2)) = 0;
    } while (iter2 != &g_4de9e0);
    j = &g_4deb02;
    node = &g_4deb00.field_0;
    do
    {
        j += 2;
        *((unsigned short *)((char *)j - 4)) = 8;
    } while (j != &g_4ded42);
    k = &g_4ded42;
    do
    {
        k += 2;
        *((unsigned short *)((char *)k - 4)) = 9;
    } while (k != &g_4def02);
    g_4de9d2 = g_4de9d2 + 112;
    l = &g_4def02;
    do
    {
        l += 2;
        *((unsigned short *)((char *)l - 4)) = 7;
    } while (l != &g_4def62);
    g_4def66 = 8;
    g_4def6a = 8;
    g_4def6e = 8;
    g_4def72 = 8;
    g_4def76 = 8;
    g_4def7a = 8;
    g_4def7e = 8;
    g_4de9ce = g_4de9ce + 24;
    g_4def62 = 8;
    g_4de9d0 = g_4de9d0 + 152;
    m = 0;
    do
    {
        v3 = _INSERT(v3, 0, (unsigned short)v3 + *((short *)((char *)&g_4de9c0 + m))) * 2;
        *((unsigned short *)((char *)&v31 + m)) = v3;
        m += 2;
    } while (m != 30);
    do
    {
        v26 = node->field_2;
        if (v26)
        {
            idx1 = v26;
            v28 = *((short *)&(&v0)[2 * idx1]);
            *((unsigned short *)&(&v0)[2 * idx1]) = v28 + 1;
            node->field_0 = sub_4044a0(v28, v26);
        }
    } while ((node += 4, node != &g_4def80.field_0));
    index = 0;
    do
    {
        (&g_4dea82)[2 * index] = 5;
        (&g_4dea80)[2 * index] = sub_4044a0(index, 5);
        index += 1;
    } while (index != 30);
    return sub_409df0();
}



// Function: flush_block @ 0xadd0
typedef struct struct_1 {
    char padding_0[1];
    char field_1;
} struct_1;

typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned long long g_0;
extern struct_1 g_413410;
extern void g_418340;
extern void g_418380;
extern unsigned int g_4183a4;
extern void g_4183c0;
extern unsigned int g_4183e4;
extern struct_0 *g_4dca20;
extern unsigned long long g_4dca28;
extern unsigned long long g_4dca30;
extern unsigned long long g_4dca38;
extern unsigned long long g_4dca40;
extern char g_4dca49;
extern unsigned int g_4dca4c;
extern char g_4dca60;
extern char g_4de9e0;
extern unsigned int g_4dea80;
extern unsigned short g_4deb00;
extern char g_4def80;
extern unsigned short g_4df080;
extern unsigned short g_4df09c;
extern unsigned short g_4df280;
extern unsigned short g_4df480;

long long flush_block(char *a0, unsigned long long a1, unsigned int a2, unsigned int a3)
{
    unsigned short *i;  // r13
    unsigned long v4;  // 4172
    unsigned long long v13;  // rdx
    unsigned long long v14;  // rbx
    unsigned long long v15;  // rax
    unsigned long long v16;  // rax
    unsigned long long v17;  // rdx
    unsigned long long v18;  // rsi
    unsigned long long v19;  // rsi
    unsigned long long v20;  // rax
    unsigned long v22;  // r8
    unsigned long long iter;  // rdx
    struct_1 *v23;  // r15
    unsigned long long v24;  // rax
    unsigned long v25;  // rax
    unsigned short *v6;  // rax
    unsigned short *j;  // rax
    unsigned long long v8;  // rcx
    unsigned short *v9;  // rax
    unsigned short *k;  // rax
    unsigned short *v11;  // rax
    unsigned long long v12;  // rax
    unsigned int v0;  // [bp-0x48], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0x3c]

    i = &g_4df080;
    v4 = (unsigned short)g_4dca20->field_0;
    (&g_4dca60)[g_4dca4c] = g_4dca49;
    if ((unsigned short)v4 == 0xffff)
    {
        iter = 0;
        do
        {
            v6 = i + 2;
            iter += *(i);
            i = v6;
        } while (i != &g_4df09c);
        j = &g_4df09c;
        v8 = 0;
        do
        {
            v9 = j + 2;
            v8 += *(j);
            j = v9;
        } while (j != &g_4df280);
        k = &g_4df280;
        do
        {
            v11 = k + 2;
            iter += *(k);
            k = v11;
        } while (k != &g_4df480);
        *((unsigned short *)&g_4dca20->field_0) = (unsigned int)iter <= (unsigned int)v8 >> 2;
    }
    sub_40a090(&g_4183c0);
    sub_40a090(&g_418380);
    sub_409f80(&g_4df080, g_4183e4);
    sub_409f80(&g_4def80, g_4183a4);
    sub_40a090(&g_418340);
    v12 = 18;
    v13 = 15;
    while (1)
    {
        v14 = (int)v12;
        if (!*((short *)&(&g_4de9e0)[2 + 4 * v13]))
        {
            v12 -= 1;
            if (v12 != 2)
            {
                v13 = *((char *)&g_413410.field_0 + v12);
            }
            else
            {
                v15 = 23;
                v14 = 2;
                break;
            }
        }
        else
        {
            v15 = (unsigned int)v14 * 3 + 0x11;
            break;
        }
    }
    g_4dca40 = v15 + g_4dca40;
    v16 = g_4dca40 + 10 >> 3;
    v0 = a1 + 4;
    v17 = g_4dca38 + 10 >> 3;
    g_4dca28 = g_4dca28 + a1;
    if (v16 <= v17)
    {
        v18 = v16;
        v19 = v18;
    }
    else
    {
        v19 = v17;
    }
    if (v0 > v19 || !a0)
    {
        if (v18 < v17)
        {
            sub_4043b0(a3 + 4, 3);
            v1 = g_4183e4;
            v0 = g_4183a4;
            sub_4043b0(g_4183e4 - 0x100, 5);
            v23 = &g_413410.field_0;
            sub_4043b0(g_4183a4, 5);
            sub_4043b0(v14 - 3, 4);
            v24 = 16;
            while (1)
            {
                sub_4043b0(*((short *)&(&g_4de9e0)[2 + 4 * v24]), 3);
                if (v14 + (char *)&g_413410.field_0 == v23)
                    break;
                v24 = v23->field_1;
                v23 = &v23->field_1;
            }
            sub_40a6b0(&g_4df080, v1);
            sub_40a6b0(&g_4def80, v0);
            sub_40a870(&g_4df080, &g_4def80);
            g_4dca30 = g_4dca30 + g_4dca40 + 3;
        }
        else
        {
            sub_4043b0(a3 + 2, 3);
            sub_40a870(&g_4deb00, &g_4dea80);
            g_4dca30 = g_4dca30 + g_4dca38 + 3;
        }
        sub_409df0();
        if (a3)
            goto LABEL_40b046;
LABEL_40afcc:
        v25 = g_4dca30;
        if (a2 && (char)v25 & 7)
        {
            sub_4043b0(0, 3);
            g_4dca30 = g_4dca30 + 10 & 0xfffffffffffffff8;
            sub_4045c0(a0, 0, 1);
            v25 = g_4dca30;
        }
    }
    else
    {
        v20 = sub_4043b0(a3, 3);
        __unsupported_jumpkind_Ijk_NoDecode()
        g_4dca30 = (_INSERT(v20, 0, (char)v20 & 16) + 10 & 0xfffffffffffffff8) + v22 * 8;
        sub_4045c0(a0, a1, 1);
        sub_409df0();
        if (!a3)
            goto LABEL_40afcc;
LABEL_40b046:
        sub_4044d0();
        v25 = g_4dca30 + 7;
        g_4dca30 = v25;
    }
    return (long long)(v25) >> 3;
}



// Function: ct_tally @ 0xb160
extern unsigned long long g_0;
extern char g_41801c;
extern char g_418240;
extern unsigned int g_41906c;
extern unsigned long long g_419078;
extern unsigned short g_44b000;
extern char g_49c000;
extern char g_4dca48;
extern char g_4dca49;
extern unsigned int g_4dca4c;
extern unsigned int g_4dca50;
extern unsigned int g_4dca54;
extern char g_4dca60;
extern char g_4ddb60;
extern char g_4ddd60;
extern unsigned short g_4def80[4];
extern unsigned short g_4df080[4];

unsigned int ct_tally(unsigned int a0, unsigned int a1)
{
    unsigned long v1;  // rax
    char v2;  // al
    unsigned long long v11;  // rdx
    unsigned int v12;  // eax
    unsigned int v3;  // edx
    unsigned long v4;  // rsi
    unsigned long idx;  // rcx
    unsigned long index;  // rcx
    unsigned long v7;  // rax
    char v8;  // cl
    unsigned long long i;  // rdx
    unsigned long long v10;  // rcx

    v1 = g_4dca54;
    g_4dca54 = (unsigned int)v1 + 1;
    (&g_49c000)[v1] = a1;
    v2 = g_4dca48;
    if (!a0)
    {
        g_4df080[2 * a1] = g_4df080[2 * a1] + 1;
    }
    else
    {
        v3 = a0 - 1;
        v4 = (&g_4ddd60)[a1];
        g_4df080[0x202 + 2 * v4] = g_4df080[0x202 + 2 * v4] + 1;
        if (v3 > 0xff)
        {
            idx = ((int)(v3) >> 7) + 0x100;
            v2 = g_4dca48 | 14;
        }
        else
        {
            idx = (&g_4ddb60)[v3];
        }
        g_4def80[2 * idx] = g_4def80[2 * idx] + 1;
        index = g_4dca50;
        g_4dca50 = (unsigned int)index + 1;
        (&g_44b000)[index] = v3;
        g_4dca49 = g_4dca49 | v2;
    }
    g_4dca48 = v2 * 2;
    if (!((char)g_4dca54 & 7))
    {
        v7 = g_4dca4c;
        v8 = g_4dca49;
        g_4dca48 = 1;
        g_4dca49 = 0;
        g_4dca4c = (unsigned int)v7 + 1;
        (&g_4dca60)[v7] = v8;
    }
    if (*((int *)&g_41801c) > 2 && !((unsigned short)g_4dca54 & 0xfff))
    {
        i = 0;
        v10 = g_4dca54 * 8;
        do
        {
            v11 = i + 4;
            v10 += *((short *)((char *)&g_4def80[0] + i)) * (*((int *)&(&g_418240)[i]) + 5);
            i = v11;
        } while (i != 120);
        if (g_4dca54 >> 1 > g_4dca50 && g_41906c - g_419078 >> 1 > v10 >> 3)
            return 1;
    }
    v12 = 1;
    if (g_4dca54 != 0x7fff)
        v12 = g_4dca50 == 0x8000;
    return v12;
}



// Function: fillbuf @ 0xb320
extern unsigned long long g_0;
extern unsigned int g_41a008;
extern unsigned int g_41a00c;
extern char g_49c000;
extern unsigned int g_4df98c;
extern unsigned int g_4df990;
extern unsigned short g_4df994;

int fillbuf(unsigned int a0)
{
    unsigned long v1;  // rbx
    unsigned int v2;  // eax
    unsigned int v3;  // edx
    unsigned long v4;  // rcx
    unsigned long v5;  // 4176

    v1 = a0;
    v2 = g_4df990;
    v3 = g_4df994 << ((char)a0 & 31);
    v4 = g_4df98c;
    if (g_4df98c < a0)
    {
        do
        {
            v1 -= v4;
            v3 |= v2 << ((char)(v1 & 0xffffffff) & 31);
            v5 = g_41a00c;
            g_4df994 = v3;
            if (g_41a008 < (unsigned int)v5)
            {
                v2 = (&g_49c000)[g_41a008];
                g_41a008 = g_41a008 + 1;
                g_4df990 = v2;
            }
            else
            {
                v2 = sub_40d6c0(1);
                g_4df990 = v2;
                if (v2 == 0xffffffff)
                {
                    g_4df990 = 0;
                    v3 = g_4df994;
                    v2 = 0;
                }
                else
                {
                    v3 = g_4df994;
                }
            }
        } while ((g_4df98c = 8, v4 = 8, (unsigned int)v1 > 8));
    }
    g_4df98c = v4 - v1;
    g_4df994 = (unsigned short)v3 | (unsigned short)(v2 >> ((char)g_4df98c & 31));
    return v2 >> ((char)g_4df98c & 31);
}



// Function: make_table @ 0xb3f0
extern unsigned long long g_0;

long long make_table(unsigned int a0, char *a1, unsigned int a2, unsigned long long a3)
{
    char *v13;  // r13
    void* iter;  // rax
    unsigned int v23;  // eax
    unsigned int v24;  // eax
    unsigned int m;  // r8d
    unsigned int v26;  // eax
    unsigned long long v27;  // rdi
    unsigned long v28;  // fs
    char *j;  // rax
    unsigned long index;  // rcx
    unsigned long long v17;  // r9
    unsigned long long idx;  // rax
    short v19;  // cx
    unsigned long long idx1;  // rax
    unsigned int v21;  // r11d
    unsigned int l;  // eax
    unsigned long long v0;  // [bp-0xe0]
    unsigned int v1;  // [bp-0xd8]
    unsigned int v2;  // [bp-0xd4]
    unsigned long long v3;  // [bp-0xd0]
    char v4;  // [bp-0xc8]
    char v5;  // [bp-0xc6]
    char v6;  // [bp-0xa6]
    char v7;  // [bp-0x98]
    char v8;  // [bp-0x68]
    unsigned short v9;  // [bp-0x66]
    short v10;  // [bp-0x46]
    unsigned long v11;  // [bp-0x40]

    v13 = a1;
    iter = &v5;
    do
    {
        iter += 2;
        *((unsigned short *)((char *)iter - 2)) = 0;
    } while (iter != &v6);
    j = v13;
    do
    {
        index = *(j);
        j += 1;
        *((short *)&(&v4)[2 * index]) = *((short *)&(&v4)[2 * index]) + 1;
    } while (j != &v13[a0]);
    v17 = 0;
    v9 = 0;
    idx = 1;
    do
    {
        v17 += *((short *)&(&v4)[2 * idx]) << ((char)(16 - idx) & 31);
        (&v9)[idx] = v17;
        idx += 1;
    } while (idx != 0x11);
    v19 = v10;
    if (v19)
        sub_40d3f0("Bad table\n"); /* do not return */
    idx1 = 1;
    v1 = 16 - a2;
    while (1)
    {
        *((short *)&(&v8)[2 * idx1]) = v19 >> ((char)v1 & 31);
        *((unsigned short *)&(&v7)[2 * idx1]) = 1 << ((char)(a2 - idx1) & 31);
        idx1 += 1;
        if (a2 < (unsigned int)idx1)
            break;
        v19 = *((short *)&(&v8)[2 * idx1]);
    }
    v21 = a2 + 1;
    l = v21;
    do
    {
        v23 = l + 1;
        *((unsigned short *)&(&v7)[2 * l]) = 1 << ((char)(16 - l) & 31);
        l = v23;
    } while (l != 0x11);
    v24 = *((short *)&(&v8)[2 * v21]) >> ((char)v1 & 31);
    if (v24)
    {
        m = 1 << ((char)a2 & 31);
        if (v24 != m)
        {
            do
            {
                v26 = v24 + 1;
                *((unsigned short *)(a3 + v24 * 2)) = 0;
                v24 = v26;
            } while (m != v24);
        }
    }
    v0 = a3;
    v27 = 0;
    v2 = 1 << ((char)a2 & 31);
    v3 = a3 + 2;
    while (1)
    {
        if (!v13[v27])
        {
            v27 += 1;
            if (a0 == v27)
                return v11 - *((long long *)(40 + v28));
        }
    }
}



// Function: read_pt_len @ 0xb700
extern unsigned long long g_0;
extern void g_4df994;
extern unsigned short g_4df9a0;
extern unsigned short g_4dfba0;
extern char g_4dfbc0;
extern char g_4dfbc1;

unsigned short * read_pt_len(unsigned int a0, unsigned int a1, unsigned int a2)
{
    char v2;  // r14b
    unsigned int v3;  // ebx
    unsigned int v12;  // eax
    unsigned int v13;  // eax
    unsigned int v14;  // eax
    unsigned long long v15;  // rbp
    unsigned long long v16;  // rax
    unsigned long long iter;  // rax
    unsigned long long v18;  // rcx
    char *iter1;  // rax
    unsigned long node;  // rax
    unsigned short *iter2;  // rax
    unsigned long long v4;  // r15
    unsigned long v5;  // rax
    void* k;  // r14
    unsigned long long v7;  // r15
    unsigned int v8;  // edi
    unsigned short v9;  // dx
    char v10;  // r12b
    unsigned int v11;  // edx
    unsigned int v0;  // [bp-0x3c]

    v2 = 16 - a1;
    v3 = *((short *)&g_4df994) >> (v2 & 31);
    sub_40b320(a1);
    if (v3)
    {
        v4 = 0;
        v0 = a2 - 1;
        while (1)
        {
            v5 = (int)v4;
            k = &(&g_4dfbc0)[v5];
            v7 = v4;
            do
            {
                if ((unsigned int)v7 >= v3)
                {
                    if ((unsigned int)v7 >= a0)
                        return sub_40b3f0(a0, &g_4dfbc0, 8, &g_4df9a0);
                    v18 = (int)v7;
                    iter1 = &(&g_4dfbc0)[v18];
                    do
                    {
                        *(iter1) = 0;
                        iter1 += 1;
                    } while (iter1 != a0 - v7 + &g_4dfbc1 + v18 - 1);
                    return sub_40b3f0(a0, &g_4dfbc0, 8, &g_4df9a0);
                }
                v8 = 3;
                v9 = *((short *)&g_4df994) >> 13;
                v10 = v9;
                if (v9 == 7)
                {
                    if (*((short *)&g_4df994) & 0x1000)
                    {
                        v11 = 0x1000;
                        v12 = 7;
                        do
                        {
                            v11 >>= 1;
                            v14 = v12 + 1;
                            v12 = v14;
                        } while (v11 & *((short *)&g_4df994));
                        if (v14 > 16)
                            sub_40d3f0("Bad table\n"); /* do not return */
                        v8 = v13 - 2;
                        v10 = v14;
                    }
                    else
                    {
                        v10 = 7;
                        v8 = 4;
                    }
                }
                sub_40b320(v8);
                k += 1;
                *((char *)k - 1) = v10;
                v7 = (unsigned int)v7 + 1;
            } while (k != &(&g_4dfbc1)[v5 + v0 + -1 * v4]);
            v15 = *((short *)&g_4df994) >> 14;
            sub_40b320(2);
            v4 = v7;
            if ((unsigned int)v15)
            {
                v16 = (int)v7;
                iter = v16;
                do
                {
                    (&g_4dfbc0)[iter] = 0;
                    iter += 1;
                } while (iter != (int)v15 + v16);
                v4 = v7 + v15;
            }
        }
    }
    else
    {
        sub_40b320(a1);
        node = &g_4dfbc0;
        do
        {
            *((char *)node) = 0;
            node += 1;
        } while (node != (unsigned long long)&(&g_4dfbc0)[a0]);
        iter2 = &g_4df9a0;
        do
        {
            *(iter2) = *((short *)&g_4df994) >> (v2 & 31);
            iter2 += 1;
        } while (iter2 != &g_4dfba0);
        return iter2;
    }
}



// Function: unlzh @ 0xb920
typedef struct struct_0 {
    unsigned short field_0;
    unsigned short field_2;
    unsigned short field_4;
    unsigned short field_6;
} struct_0;

extern unsigned long long g_0;
extern unsigned int g_41a010;
extern unsigned int g_41a014;
extern struct_0 g_41a880;
extern char g_43b000;
extern struct_0 g_44b000;
extern struct_0 g_44d000;
extern char g_45b000;
extern char g_45b001;
extern char g_45b1fe;
extern unsigned int g_4df980;
extern unsigned int g_4df984;
extern int g_4df988;
extern unsigned int g_4df98c;
extern unsigned int g_4df990;
extern void g_4df994;
extern struct_0 g_4df9a0;
extern unsigned int g_4dfba0;
extern char g_4dfbc0;

unsigned int unlzh(unsigned int a0, unsigned int a1)
{
    int v5;  // esi
    unsigned int v6;  // edx
    unsigned long v15;  // rdx
    unsigned int v16;  // ecx
    unsigned long v17;  // r15
    unsigned int v18;  // edx
    unsigned long long v19;  // rbp
    unsigned long v20;  // rdi
    char *iter;  // rcx
    unsigned long v22;  // rcx
    char *node;  // rdx
    char *iter1;  // rdx
    unsigned long long iter2;  // rcx
    short v25;  // _bp
    struct_0 *m;  // rdx
    unsigned long v27;  // rbp
    unsigned int v28;  // edx
    unsigned int v29;  // r13d
    unsigned long v30;  // rbp
    unsigned int v31;  // edx
    char *v32;  // rcx
    char v8;  // dil
    unsigned int j;  // r13d
    unsigned int v10;  // esi
    unsigned int v11;  // edx
    unsigned int v12;  // edx
    unsigned int v13;  // ecx
    unsigned long long v14;  // rbp
    unsigned int v0;  // [bp-0x54]
    struct_0 *v1;  // [bp-0x50]
    char *v2;  // [bp-0x48]
    int v3;  // [bp-0x40]

    g_41a014 = a0;
    g_41a010 = a1;
    *((unsigned short *)&g_4df994) = 0;
    g_4df990 = 0;
    g_4df98c = 0;
    sub_40b320(16);
    g_4dfba0 = 0;
    g_4df988 = 0;
    g_4df984 = 0;
    do
    {
        v5 = g_4df988;
        v6 = g_4df980;
        iter2 = 0;
        v8 = 0;
        do
        {
            j = iter2;
            v10 = v5 - 1;
            if (v5 - 1 < 0)
            {
                g_4df988 = v10;
                if (v8)
                    g_4df980 = v6;
                do
                {
                    v12 = g_4dfba0;
                    v13 = *((short *)&g_4df994);
                    if (v12)
                        continue;
                    sub_40b320(16);
                    g_4dfba0 = *((short *)&g_4df994);
                    if (*((short *)&g_4df994))
                    {
                        sub_40b700(19, 5, 3);
                        v14 = 0;
                        v3 = *((short *)&g_4df994) >> 7;
                        sub_40b320(9);
                        v1 = &g_4df9a0.field_0;
                        v2 = &g_4dfbc0;
                        if (v3)
                        {
                            while (1)
                            {
                                v15 = (&v1->field_0)[(char)(*((short *)&g_4df994) >> 8)];
                                if ((unsigned int)v15 > 18)
                                {
                                    v16 = 128;
                                    do
                                    {
                                    } while ((v15 = (unsigned long)(!((unsigned int)(unsigned short)*((short *)&g_4df994) & v16) ? (unsigned long long)(unsigned short)*((short *)((char *)&g_41a880.field_0 + 2 * v15)) : (unsigned long long)(unsigned short)*((short *)((char *)&g_41a880.field_0 + 2 * v15 + 0x10000))), v16 >>= 1, (unsigned int)v15 > 18));
                                }
                                v17 = (int)v15;
                                v0 = v15;
                                sub_40b320(v2[v17]);
                                v18 = v0;
                                if (v18 > 2)
                                {
                                    v19 = (unsigned int)v14 + 1;
                                    (&g_45b000)[(unsigned int)v14] = v18 - 2;
                                    v14 = v19;
                                    if (v3 <= (unsigned int)v14)
                                        break;
                                }
                                else
                                {
                                    if (v18)
                                    {
                                        if (v18 != 1)
                                        {
                                            sub_40b320(9);
                                            v18 = (*((short *)&g_4df994) >> 7) + 19;
                                            v17 = v18;
                                        }
                                        else
                                        {
                                            sub_40b320(4);
                                            v18 = (*((short *)&g_4df994) >> 12) + 2;
                                            v17 = v18;
                                        }
                                    }
                                    v20 = (int)v14;
                                    iter = &(&g_45b000)[v20];
                                    do
                                    {
                                        *(iter) = 0;
                                        iter += 1;
                                    } while (&(&g_45b001)[v20 + v17] != iter);
                                    v14 = v18 + (unsigned int)v14 + 1;
                                    if (v3 <= (unsigned int)v14)
                                        break;
                                }
                            }
                            if ((unsigned int)v19 <= 509)
                            {
                                v22 = (int)v19;
                                node = &(&g_45b000)[v22];
                                do
                                {
                                    *(node) = 0;
                                    node += 1;
                                } while (node != &(&g_45b001)[v22 + 509 + -1 * v19]);
                            }
                            sub_40b3f0(0x1fe, &g_45b000, 12, &g_44b000.field_0);
                        }
                        else
                        {
                            sub_40b320(9);
                            iter1 = &g_45b000;
                            v25 = *((short *)&g_4df994) >> 7;
                            do
                            {
                                *(iter1) = 0;
                                iter1 += 1;
                            } while (iter1 != &g_45b1fe);
                            m = &g_44b000.field_0;
                            do
                            {
                                m->field_0 = v25;
                                m = &m->field_2;
                            } while (m != &g_44d000.field_0);
                        }
                        sub_40b700(14, 4, 0xffffffff);
                        v12 = g_4dfba0;
                        v13 = *((short *)&g_4df994);
                    }
                    else
                    {
                        g_4df984 = 1;
                        if (!j)
                            return 0;
                        break;
                    }
                    g_4dfba0 = v12 - 1;
                    v27 = *((short *)((char *)&g_44b000.field_0 + 2 * (_INSERT(v13, 0, (unsigned short)v13 >> 4) & 0xfff)));
                    if ((unsigned int)v27 > 509)
                    {
                        v28 = 8;
                        while (1)
                        {
                            if (v13 & v28)
                            {
                                v28 >>= 1;
                                v27 = *((short *)((char *)&g_41a880.field_0 + 2 * v27 + 0x10000));
                                if ((unsigned int)v27 <= 509)
                                    break;
                            }
                            else
                            {
                                v27 = *((short *)((char *)&g_41a880.field_0 + 2 * v27));
                                v28 >>= 1;
                                if ((unsigned int)v27 <= 509)
                                    break;
                            }
                        }
                    }
                    sub_40b320((&g_45b000)[v27 & 0xffffffff]);
                    if ((unsigned int)v27 > 0xff)
                    {
                        g_4df988 = (unsigned int)v27 - 253;
                        v30 = *((short *)((char *)&g_4df9a0.field_0 + 2 * (*((short *)&g_4df994) >> 8)));
                        if ((unsigned int)v30 > 13)
                        {
                            v31 = 128;
                            while (1)
                            {
                                if (*((short *)&g_4df994) & v31)
                                {
                                    v31 >>= 1;
                                    v30 = *((short *)((char *)&g_41a880.field_0 + 2 * v30 + 0x10000));
                                    if ((unsigned int)v30 <= 13)
                                        break;
                                }
                                else
                                {
                                    v30 = *((short *)((char *)&g_41a880.field_0 + 2 * v30));
                                    v31 >>= 1;
                                    if ((unsigned int)v30 <= 13)
                                        break;
                                }
                            }
                        }
                        sub_40b320((&g_4dfbc0)[v30 & 0xffffffff]);
                        if ((unsigned int)v30)
                            sub_40b320(v30 - 1);
                        *(v32) = *(v32) + (char)(j - 1);
                    }
                    v29 = j + 1;
                    (&g_43b000)[j] = v27;
                    j = v29;
                } while (j != 0x2000);
            }
            v11 = v6 + 1 & 0x1fff;
            (&g_43b000)[iter2] = (&g_43b000)[v6];
            iter2 += 1;
            v8 = 1;
            v6 = v11;
            v5 = v10;
        } while (iter2 != 0x2000);
        g_4df988 = v10;
        v29 = 0x2000;
        g_4df980 = v11;
        sub_40d5a0(a1, &g_43b000, v29);
    } while (!g_4df984);
    return 0;
}



// Function: unlzw @ 0xbe90
extern unsigned long long g_0;
extern void g_418024;
extern unsigned int g_4183e8;
extern long long stderr;
extern unsigned int g_41a008;
extern unsigned int g_41a00c;
extern long long g_41a420;
extern unsigned long long g_41a828;
extern unsigned int g_41a84c;
extern long long g_41a850;
extern unsigned int g_41a85c;
extern void g_41a880;
extern unsigned long long g_41a888;
extern unsigned long long g_41a978;
extern char g_43b000;
extern void g_45affd;
extern void g_45affe;
extern char g_45b000;
extern char g_49c000;
extern char g_49c001;

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

    v16 = g_41a008;
    if ((unsigned int)v16 < g_41a00c)
    {
        g_41a008 = (unsigned int)v16 + 1;
        v17 = (&g_49c000)[v16];
    }
    else
    {
        v17 = sub_40d6c0(0);
    }
    *((unsigned int *)&g_418024) = v17;
    g_4183e8 = v17 & 128;
    if ((char)v17 & 96)
    {
        if (!g_41a85c)
        {
            sub_40f6b0(stderr, "\n%s: %s: warning, unknown flags 0x%x\n", g_41a850, &g_41a420);
            if (!g_41a84c)
            {
LABEL_40bf09:
                g_41a84c = 2;
            }
        }
        else if (!g_41a84c)
        {
            goto LABEL_40bf09;
        }
        v17 = *((int *)&g_418024);
    }
    *((unsigned int *)&g_418024) = v17 & 31;
    v9 = 1 << ((char)*((int *)&g_418024) & 63);
    if (*((int *)&g_418024) > 16)
    {
        sub_40f6b0(stderr, "\n%s: %s: compressed with %d bits, can only handle %d bits\n", g_41a850, &g_41a420);
        g_41a84c = 1;
        return 1;
    }
    v18 = g_4183e8;
    *((unsigned long long *)&g_41a880) = 0;
    v19 = g_41a00c;
    g_41a978 = 0;
    v20 = g_41a008 * 8;
    iter2 = &g_41a888;
    v13 = g_41a00c;
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
        (&g_43b000)[j] = j;
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
LABEL_40bfec:
        v28 = (long long)(v20) >> 3;
        if ((int)v28 > (unsigned int)v19)
        {
LABEL_40bff9:
            g_41a00c = 0;
LABEL_40c00a:
            v1 = v26;
            v26 = v1;
            v13 = sub_40d190(a0);
            if (v13 == 0xffffffff)
                sub_40d4e0(); /* do not return */
            v31 = v13;
            g_41a828 = g_41a828 + v31;
            v19 = g_41a00c + v31;
            g_41a00c = v19;
            goto LABEL_40c049;
        }
        while (1)
        {
            v19 -= v28;
            if ((unsigned int)v19 > 0)
            {
                l = &g_49c000;
                do
                {
                    v30 = l + 1;
                    *(v30 - 1) = l[(int)v28];
                    l = v30;
                } while (&(&g_49c001)[v19 - 1 & 0xffffffff] != l);
            }
            g_41a00c = v19;
            if ((unsigned int)v19 <= 63)
                goto LABEL_40c00a;
LABEL_40c049:
            v32 = v19 & 0xffffffff;
            if (v13)
            {
                v20 = 0;
                v6 = (v32 - ((v19 & 0xffffffff) % v12 & 0xffffffff)) * 8;
                if (v6 > 0)
                    goto LABEL_40c07b;
                goto LABEL_40bfec;
            }
            v6 = v32 * 8 - (v12 - 1);
            if (v6 <= 0)
                goto LABEL_40c3f5;
LABEL_40c07b:
            if (v0 <= v3)
                break;
            v48 = 0xffffffffffffffff;
LABEL_40c1ef:
            v49 = v12;
            v12 = v49 + 1;
            v3 = v9;
            if (*((int *)&g_418024) != v12)
                v3 = (1 << ((char)v12 & 63)) - 1;
            v28 = (long long)(v48 + v49 * 8 - (v49 * 8 + v48) % (v49 * 8)) >> 3;
            v14 = (1 << ((char)v12 & 31)) - 1;
            if ((int)v28 > (unsigned int)v19)
                goto LABEL_40bff9;
        }
        v33 = 0;
        v4 = v14;
        v5 = v12;
        while (1)
        {
            v1 = v27;
            v20 = v33 + v5;
            v34 = (long long)((&g_49c000)[1 + ((long long)(v33) >> 3)] * 0x100 | (&g_49c000)[2 + ((long long)(v33) >> 3)] * 0x10000 | (&g_49c000)[(long long)(v33) >> 3]) >> ((char)v33 & 7 & 63) & v4;
            if (v27 != 0xffffffffffffffff)
                break;
            if (v34 > 0xff)
                sub_40d3f0("corrupt input."); /* do not return */
            v2 = v34;
            v47 = v26 + 1;
            (&g_45b000)[v26] = v34;
            v26 = v47;
            if (v20 >= v6)
            {
LABEL_40c2df:
                v26 = v47;
                v27 = v34;
                if (v13)
                {
                    v19 = g_41a00c;
                    goto LABEL_40bfec;
                }
LABEL_40c3f5:
                if (v26 > 0)
                {
                    sub_40d5a0(a1, &g_45b000, v26);
                    return 0;
                }
                return 0;
            }
LABEL_40c1d5:
            v33 = v20;
            v27 = v34;
            if (v3 < v0)
            {
                v19 = g_41a00c;
                v48 = v33 - 1;
                goto LABEL_40c1ef;
            }
        }
        if (v34 != 0x100)
        {
            if (v34 >= v0)
            {
                if (v34 > v0)
                {
                    if (v26 > 0)
                        sub_40d5a0(a1, &g_45b000, v26);
                    sub_40d3f0(); /* do not return */
                }
            }
            else
            {
                idx = v34;
                iter = &g_45affe;
                while (1)
                {
LABEL_40c147:
                    if (idx <= 0xff)
                    {
                        v37 = (&g_43b000)[idx];
                        node = iter - 1;
                        *((char *)iter - 1) = v37;
                        m = &g_45affe - node;
                        v7 = v37;
                        v40 = v26 + m;
                        v2 = v7;
                        if (v40 <= 0x3ffff)
                        {
                            v8 = v40;
                            memcpy(&(&g_45b000)[v26], node, m);
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
                                    memcpy(&(&g_45b000)[v41], node, n);
                                }
                                v41 = v45;
                                if ((unsigned int)v41 > 0x3ffff)
                                {
                                    v46 = 0;
                                    sub_40d5a0(a1, &g_45b000, v41 & 0xffffffff);
                                    v41 = v46;
                                }
                                node += n;
                                m = &g_45affe - node;
                            } while (m > 0);
                            v26 = v41;
                            v20 = v8;
                            v34 = v10;
                        }
                        if (v9 > v0)
                        {
                            *((unsigned short *)&(&g_41a880)[2 * v0]) = v1;
                            (&g_43b000)[v0] = v7;
                            v0 += 1;
                        }
                        v47 = v26;
                        if (v20 >= v6)
                            goto LABEL_40c2df;
                        goto LABEL_40c1d5;
                    }
LABEL_40c138:
                    iter -= 1;
                    *((char *)iter) = (&g_43b000)[idx];
                    idx = *((short *)&(&g_41a880)[2 * idx]);
                }
            }
            goto LABEL_40c118;
        }
        else if (!g_4183e8)
        {
            idx = 0x100;
            iter = &g_45affe;
            if (v0 != 0x100)
                goto LABEL_40c138;
LABEL_40c118:
            iter = &g_45affd;
            g_45affd = v2;
            idx = v1;
            goto LABEL_40c147;
        }
        else
        {
            *((unsigned long long *)&g_41a880) = 0;
            iter1 = &g_41a888;
            g_41a978 = 0;
            for (v51 = 31; v51; iter1 += 1)
            {
                v51 -= 1;
                *(iter1) = 0;
            }
            v14 = 0x1ff;
            v0 = 0x100;
            v19 = g_41a00c;
            v11 = 9;
            v3 = 0x1ff;
            v27 = v1;
            v20 = v20 - 1 + v12 * 8 - (v20 - 1 + v12 * 8) % (v12 * 8);
            v12 = v11;
        }
    }
}



// Function: unpack @ 0xc590
extern unsigned long long g_0;
extern unsigned int g_41a004;
extern unsigned int g_41a008;
extern char g_41a00c;
extern unsigned int g_41a010;
extern unsigned int g_41a014;
extern unsigned int g_41a820;
extern char g_43b000;
extern char g_45b000;
extern char g_49c000;
extern void g_4dfbe0;
extern unsigned long long g_4dfbe8;
extern unsigned int g_4dfbf0;
extern unsigned int g_4dfc00[4];
extern int g_4dfc80[4];
extern unsigned int g_4dfd00[4];
extern char g_4dfd7e;
extern char g_4dfd7f;
extern char g_4dfd80;
extern unsigned int g_4dfe80;
extern unsigned long long g_4dfe88;

unsigned long long unpack(unsigned int a0, unsigned int a1)
{
    unsigned long long v3;  // rax
    unsigned int v4;  // ebp
    unsigned long v13;  // rax
    unsigned int v14;  // eax
    unsigned int v15;  // edi
    int v16;  // edx
    unsigned long long idx;  // r15
    unsigned long long v18;  // r13
    unsigned long iter;  // r12
    unsigned long v20;  // r14
    unsigned long v21;  // rax
    unsigned int v22;  // eax
    unsigned long v5;  // rdx
    unsigned long idx1;  // rax
    unsigned int v24;  // ecx
    int v25;  // edx
    int v26;  // edx
    int v27;  // edx
    unsigned long long idx2;  // r8
    char *v29;  // rax
    char *iter1;  // rax
    unsigned int v31;  // edx
    char *node;  // rax
    unsigned int v6;  // esi
    unsigned int l;  // r8d
    unsigned long long v34;  // rax
    unsigned int v35;  // esi
    unsigned int v36;  // r12d
    unsigned long v37;  // r13
    unsigned long v38;  // rbx
    unsigned long v39;  // rax
    unsigned int v40;  // eax
    unsigned long v42;  // rdx
    unsigned int i;  // ebp
    char v43;  // al
    unsigned long v44;  // rax
    unsigned int v45;  // eax
    unsigned int v8;  // eax
    unsigned long long v9;  // r12
    unsigned long long v10;  // r13
    unsigned int v11;  // ebp
    unsigned long v12;  // rax
    unsigned int v0;  // [bp-0x40]
    unsigned int v1;  // [bp-0x3c]

    v3 = 0;
    v4 = 4;
    g_41a014 = a0;
    v5 = g_41a008;
    g_4dfe88 = 0;
    g_41a010 = a1;
    v6 = *((int *)&g_41a00c);
    do
    {
        i = v4;
        if ((unsigned int)v5 < v6)
        {
            g_41a008 = (unsigned int)v5 + 1;
            v8 = (&g_49c000)[v5];
            v5 = g_41a008;
        }
        else
        {
            v8 = sub_40d6c0(0);
            if (v8 < 0)
                sub_40d3f0("invalid compressed data -- unexpected end of file"); /* do not return */
            v5 = g_41a008;
            v6 = *((int *)&g_41a00c);
        }
        v3 = (char)v8 | v3 * 0x100;
        g_4dfe88 = v3;
        v4 = i - 1;
    } while (i != 1);
    if ((unsigned int)v5 >= v6)
        goto LABEL_0x40cacf;
    g_41a008 = (unsigned int)v5 + 1;
    g_4dfe80 = (&g_49c000)[v5];
    if (g_4dfe80 - 1 > 24)
        sub_40d3f0("invalid compressed data -- Huffman code bit length out of range"); /* do not return */
    v9 = 1;
    v10 = 0;
    v11 = 1;
    do
    {
        v12 = g_41a008;
        if ((unsigned int)v12 < *((int *)&g_41a00c))
        {
            g_41a008 = (unsigned int)v12 + 1;
            v13 = (&g_49c000)[v12];
        }
        else
        {
            v14 = sub_40d6c0(0);
            if (v14 < 0)
                sub_40d3f0("invalid compressed data -- unexpected end of file"); /* do not return */
            v13 = (char)v14;
        }
        v15 = g_4dfe80;
        g_4dfc80[v9] = v13;
        if ((unsigned int)(v11 - (g_4dfe80 == (unsigned int)v9)) < (unsigned int)v13)
            sub_40d3f0("too many leaves in Huffman tree"); /* do not return */
        v9 += 1;
        v10 += v13;
        v11 = (unsigned int)(v11 - v13) * 2 + 1;
    } while (g_4dfe80 >= (unsigned int)v9);
    if ((unsigned int)v10 > 0xff)
        sub_40d3f0("too many leaves in Huffman tree"); /* do not return */
    v16 = g_4dfc80[g_4dfe80];
    g_4dfc80[g_4dfe80] = v16 + 1;
    if (g_4dfe80 <= 0)
    {
        g_4dfc80[g_4dfe80] = v16 + 2;
        sub_40d3f0("too few leaves in Huffman tree"); /* do not return */
    }
    idx = 1;
    v18 = 0;
    do
    {
        g_4dfd00[idx] = v18;
        if (g_4dfc80[idx] > 0)
        {
            iter = (unsigned int)v18 + 1;
            v20 = v18 + g_4dfc80[idx] - 1 + 2;
            while (1)
            {
                v21 = g_41a008;
                v18 = (int)iter;
                if ((unsigned int)v21 < *((int *)&g_41a00c))
                {
                    iter += 1;
                    g_41a008 = (unsigned int)v21 + 1;
                    (&g_4dfd7e)[iter] = (&g_49c000)[v21];
                    if (v20 == iter)
                        break;
                }
                else
                {
                    v22 = sub_40d6c0(0);
                    if (v22 < 0)
                        sub_40d3f0("invalid compressed data -- unexpected end of file"); /* do not return */
                    (&g_4dfd7f)[iter] = v22;
                    iter += 1;
                    if (v20 == iter)
                        break;
                }
            }
            v15 = g_4dfe80;
        }
    } while ((idx += 1, v15 >= (unsigned int)idx));
    idx1 = v15;
    v1 = g_4dfc80[idx1];
    v24 = g_4dfc80[idx1] + 1;
    g_4dfc80[idx1] = v24;
    if (v15 <= 0)
        sub_40d3f0("too few leaves in Huffman tree"); /* do not return */
    v25 = 0;
    while (1)
    {
        v26 = v25 >> 1;
        g_4dfc00[idx1] = v26;
        g_4dfd00[idx1] = g_4dfd00[idx1] - v26;
        idx1 -= 1;
        v25 = v26 + v24;
        if ((unsigned int)idx1 <= 0)
            break;
        v24 = g_4dfc80[idx1];
    }
    v27 = v25 >> 1;
    if (v27 != 1)
        sub_40d3f0("too few leaves in Huffman tree"); /* do not return */
    idx2 = 1;
    if (12 < v15)
        v15 = 12;
    g_4dfbf0 = v15;
    v29 = (int)(v27 << ((char)v15 & 31));
    iter1 = v29 + &g_45b000;
    do
    {
        v31 = g_4dfc80[idx2] << ((char)(v15 - idx2) & 31);
        node = iter1;
        if (v31)
        {
            node = iter1;
            do
            {
                node -= 1;
                *(node) = idx2 & 0xffffffff;
            } while (node != &iter1[~(v31 - 1)]);
        }
    } while ((iter1 = node, idx2 += 1, v15 >= (unsigned int)idx2));
    if (iter1 > &g_45b000)
    {
        do
        {
            iter1 -= 1;
            *(iter1) = 0;
        } while (iter1 != &g_45b000);
    }
    *((unsigned int *)&g_4dfbe0) = 0;
    l = 0;
    g_4dfbe8 = 0;
    v0 = (unsigned int)(v29 - 1);
    v34 = 0;
    while (1)
    {
        if (l >= v15)
        {
            do
            {
                v35 = v0 & (unsigned int)(v34 >> ((char)(l - v15) & 63));
                v36 = (&g_45b000)[v35];
                if (v36)
                {
                    v35 >>= (char)(v15 - v36) & 31;
                }
                else
                {
                    v37 = v0;
                    if (v35 < g_4dfc00[v15])
                    {
                        v38 = v15 + 1;
                        while (1)
                        {
                            v36 = v38;
                            v37 = v37 * 2 + 1;
                            if ((unsigned int)v38 > l)
                            {
                                do
                                {
                                    v39 = g_41a008;
                                    if ((unsigned int)v39 < *((int *)&g_41a00c))
                                    {
                                        g_41a008 = (unsigned int)v39 + 1;
                                        v40 = (&g_49c000)[v39];
                                    }
                                    else if (sub_40d6c0(0) < 0)
                                    {
                                        sub_40d3f0("invalid compressed data -- unexpected end of file"); /* do not return */
                                    }
                                } while ((v34 = (unsigned long long)(char)v40 | v34 * 0x100, l = *((int *)&g_4dfbe0) + 8, g_4dfbe8 = v34, *((unsigned int *)&g_4dfbe0) = l, l < (unsigned int)v38));
                            }
                            v35 = (unsigned int)(v34 >> ((char)(l - v36) & 63)) & (unsigned int)v37;
                            if (g_4dfc00[v38] <= v35)
                                break;
                            l = *((int *)&g_4dfbe0);
                            v38 += 1;
                        }
                    }
                    else
                    {
                        v36 = v15;
                    }
                }
                if (v35 == v1 && g_4dfe80 == v36)
                {
                    sub_40d640();
                    if (g_41a820 != g_4dfe88)
                        sub_40d3f0("invalid compressed data--length error"); /* do not return */
                    return 0;
                }
                v42 = g_41a004;
                v43 = (&g_4dfd80)[g_4dfd00[v36] + v35];
                g_41a004 = (unsigned int)v42 + 1;
                (&g_43b000)[v42] = v43;
                if (g_41a004 == 0x8000)
                    sub_40d640(&g_4dfd00[0]);
                v15 = g_4dfbf0;
                v34 = g_4dfbe8;
                l = *((int *)&g_4dfbe0) - v36;
                *((unsigned int *)&g_4dfbe0) = l;
            } while (l >= g_4dfbf0);
        }
        v44 = g_41a008;
        if ((unsigned int)v44 < *((int *)&g_41a00c))
        {
            g_41a008 = (unsigned int)v44 + 1;
            v45 = (&g_49c000)[v44];
        }
        else
        {
            v45 = sub_40d6c0(0);
            if (v45 < 0)
                break;
            l = *((int *)&g_4dfbe0);
            v15 = g_4dfbf0;
        }
        l += 8;
        v34 = (char)v45 | v34 * 0x100;
        *((unsigned int *)&g_4dfbe0) = l;
        g_4dfbe8 = v34;
    }
    sub_40d3f0("invalid compressed data -- unexpected end of file"); /* do not return */
}



// Function: check_zipfile @ 0xcb20
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

extern unsigned long long g_0;
extern unsigned int g_418020;
extern long long stderr;
extern unsigned int g_41a008;
extern unsigned int g_41a00c;
extern unsigned int g_41a014;
extern unsigned int g_41a84c;
extern char g_49c000;
extern unsigned int g_4dfe98;
extern unsigned int g_4dfe9c;
extern unsigned int g_4dfea0;

unsigned int check_zipfile(unsigned int a0)
{
    struct_0 *v1;  // rax
    unsigned long v2;  // 4168
    unsigned long v3;  // rax

    g_41a014 = a0;
    v1 = &(&g_49c000)[g_41a008];
    v2 = g_41a00c;
    g_41a008 = v1->field_1a + v1->field_1c + 30 + g_41a008;
    if (g_41a008 <= (unsigned int)v2 && v1->field_0 == 67324752)
    {
        g_418020 = v1->field_8;
        if (!((char)g_418020 & 247))
        {
            v3 = v1->field_6;
            g_4dfea0 = (unsigned int)v3 & 0xff & 1;
            if (!((char)v3 & 1))
            {
                g_4dfe9c = 1;
                g_4dfe98 = _INSERT(v3, 0, (char)v3 >> 3) & 1;
                return 0;
            }
        }
    }
    sub_40f6b0(stderr);
    g_41a84c = 1;
    return 1;
}



// Function: unzip @ 0xcc20
extern unsigned long long g_0;
extern char g_418020;
extern long long stderr;
extern unsigned int g_41a004;
extern unsigned int g_41a008;
extern char g_41a00c;
extern unsigned int g_41a010;
extern unsigned int g_41a014;
extern long long g_41a420;
extern unsigned int g_41a820;
extern unsigned int g_41a84c;
extern long long g_41a850;
extern unsigned int g_41a858;
extern unsigned int g_41a85c;
extern unsigned int g_41a860;
extern char g_43b000;
extern long long g_45b000;
extern char g_49c000;
extern char g_49c001;
extern char g_49c002;
extern char g_49c003;
extern unsigned int g_49c00e;
extern unsigned int g_49c012;
extern unsigned int g_49c016;
extern unsigned long long g_4dfe90;
extern unsigned int g_4dfe98;
extern unsigned int g_4dfe9c;
extern unsigned int g_4dfea0;
extern char g_4034add;

unsigned long long unzip(unsigned int a0, unsigned int a1)
{
    unsigned long long v6;  // rbp
    long long v7;  // r12
    unsigned long v16;  // rax
    void* iter;  // rbx
    unsigned int v18;  // eax
    unsigned int v19;  // r13d
    unsigned int v20;  // ecx
    long long v8;  // rcx
    long long v9;  // r13
    long long v10;  // r13
    unsigned long v11;  // rax
    char v12;  // al
    unsigned int v13;  // ecx
    void* node;  // rbx
    unsigned int v0;  // [bp-0x48]
    unsigned int v1;  // [bp-0x44]
    char v2;  // [bp-0x40]
    unsigned int v3;  // [bp-0x3c]
    char v4;  // [bp-0x38]

    g_41a014 = a0;
    g_41a010 = a1;
    sub_40d0d0(0, 0);
    if (g_4dfe9c)
    {
        if (!g_4dfe98)
        {
            v6 = g_49c00e;
            v7 = g_49c016;
        }
        else
        {
            v7 = 0;
            v6 = 0;
        }
        if (*((int *)&g_418020) == 8)
            goto LABEL_40cd42;
        if (*((int *)&g_418020))
            sub_40d3f0("internal error, invalid method"); /* do not return */
        v8 = g_49c012;
        v9 = g_49c016;
        if (g_4dfea0)
            v8 = g_49c012 - 12;
        if (g_49c016 != v8)
        {
            sub_40f6b0(stderr, "len %lu, siz %lu\n", g_49c016);
            sub_40d3f0("invalid compressed data--length mismatch"); /* do not return */
        }
        if (g_49c016)
        {
            while (1)
            {
                v10 = v9;
                v11 = g_41a008;
                if ((unsigned int)v11 < *((int *)&g_41a00c))
                {
                    g_41a008 = (unsigned int)v11 + 1;
                    v12 = (&g_49c000)[v11];
                }
                else
                {
                    v12 = sub_40d6c0(0);
                }
                v13 = g_41a004 + 1;
                (&g_43b000)[g_41a004] = v12;
                g_41a004 = v13;
                if (g_41a004 != 0x8000)
                {
                    v9 = v10 - 1;
                    if (v10 == 1)
                        break;
                }
                else
                {
                    sub_40d640();
                    v9 = v10 - 1;
                    if (v10 == 1)
                        break;
                }
            }
        }
        sub_40d640();
        if (g_4dfe9c)
            goto LABEL_40cf45;
        node = &v0;
        while (1)
        {
            v16 = g_41a008;
            if ((unsigned int)v16 < *((int *)&g_41a00c))
            {
                node += 1;
                g_41a008 = (unsigned int)v16 + 1;
                *((char *)node - 1) = (&g_49c000)[v16];
                if (node == &v2)
                    break;
            }
            else
            {
                node += 1;
                *((char *)node - 1) = sub_40d6c0(0);
                if (node == &v2)
                    break;
            }
        }
        v6 = v0;
        v7 = v1;
    }
    else
    {
        if (*((int *)&g_418020) != 8)
            sub_40d3f0("internal error, invalid method"); /* do not return */
        v7 = 0;
        v6 = 0;
LABEL_40cd42:
        switch ((unsigned int)sub_409a50())
        {
        case 3:
            sub_40d430(); /* do not return */
        case 0:
            if (!g_4dfe9c)
                break;
LABEL_40cf45:
            if (g_4dfe98)
            {
                iter = &v0;
                while (1)
                {
                    v18 = g_41a008;
                    if (v18 < *((int *)&g_41a00c))
                    {
                        iter += 1;
                        g_41a008 = v18 + 1;
                        *((char *)iter - 1) = (char)v18 + 4;
                        if (iter == &v4)
                            break;
                    }
                    else
                    {
                        iter += 1;
                        *((char *)iter - 1) = sub_40d6c0(0);
                        if (iter == &v4)
                            break;
                    }
                }
                v6 = v1;
                v7 = v3;
                goto LABEL_40cdc3;
            }
            break;
        default:
            sub_40d3f0("invalid compressed data--format violated"); /* do not return */
        }
    }
LABEL_40cdc3:
    v19 = 0;
    if (sub_40d0d0(&g_45b000, 0) != v6)
    {
        v19 = 1;
        sub_40f6b0(stderr, "\n%s: %s: invalid compressed data--crc error\n", g_41a850, &g_41a420);
    }
    if (g_41a820 != v7)
    {
        sub_40f6b0(stderr, "\n%s: %s: invalid compressed data--length error\n", g_41a850, &g_41a420);
        if (!g_4dfe9c)
            goto LABEL_40d000;
        v19 = 1;
    }
    else if (!g_4dfe9c)
    {
        goto LABEL_40ceab;
    }
    if (g_41a008 + 4 >= *((int *)&g_41a00c) || (v20 = (unsigned int)(char)*((char *)(unsigned long long)(g_41a008 + &g_49c001)) * 0x100, ((long long)(int)((unsigned int)(char)*((char *)(unsigned long long)(g_41a008 + &g_49c003)) * 0x100 | (unsigned int)(&g_49c002)[g_41a008]) * 0x10000 | (long long)(int)((unsigned int)(&g_49c000)[g_41a008] | v20)) != 67324752))
    {
LABEL_40ceab:
        g_4dfe9c = 0;
        g_4dfe98 = 0;
        g_4dfe90 = v6;
        if (!v19)
            return v19;
    }
    else
    {
        if (!g_41a860)
        {
            g_4034add = g_4034add + (char)v20;
            sub_40f6b0();
            g_4dfe9c = 0;
        }
        else if (!g_41a85c)
        {
            sub_40f6b0(stderr, "%s: %s has more than one entry--rest ignored\n", g_41a850, &g_41a420);
        }
LABEL_40d000:
        g_4dfe98 = 0;
        g_4dfe90 = v6;
    }
    g_41a84c = 1;
    if (!g_41a858)
        sub_408440(); /* do not return */
    return 1;
}



// Function: getcrc @ 0xd130
extern unsigned long long g_4183f0;

long long getcrc(void)
{
    return 0xffffffff ^ g_4183f0;
}



// Function: clear_bufs @ 0xd150
extern unsigned long long g_0;
extern unsigned int g_41a004;
extern unsigned long long g_41a820;
extern unsigned long long g_41a828;

char clear_bufs(void)
{
    char *v1;  // rax

    g_41a004 = 0;
    v1 = _INSERT(0, 0, *(0xc7000000000000ce)) + 52890;
    *(v1) = *(v1) + *((char *)&v1);
    *(v1) = *(v1) + *((char *)&v1);
    g_41a820 = 0;
    g_41a828 = 0;
    return *((char *)&v1);
}



// Function: read_buffer @ 0xd190
extern unsigned long long g_0;

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
    v7 = sub_40e0f0(a0, 3);
    if ((unsigned int)v7 < 0)
    {
        return v5;
    }
    else if (!(*((char *)((void*)&v7 + 1)) & 8))
    {
        *(err) = 11;
        return v5;
    }
    else if (sub_40e0f0(a0, 4, _INSERT(v7, 1, *((char *)((void*)&v7 + 1)) & 247) & 0xffffffff) == 0xffffffff)
    {
        return v5;
    }
    else
    {
        return read(a0, a1, a2);
    }
}



// Function: strlwr @ 0xd230
extern unsigned long long g_0;

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



// Function: add_envopt @ 0xd2b0
typedef struct struct_0 {
    void* field_0;
} struct_0;

extern unsigned long long g_0;

void* add_envopt(unsigned int *a0, struct_0 **a1, char *a2)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    void* v11;  // r13
    void* v12;  // rbx
    void* v13;  // rbx
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
    v11 = getenv(a2);
    if (!v11)
        return v11;
    v12 = sub_40f640(v11);
    v13 = v12;
    if (!*((char *)v13))
    {
        free(v12);
        return 0;
    }
}



// Function: gzip_error @ 0xd3f0
extern long long stderr;
extern char g_41a3a9;
extern char g_41a420;

void gzip_error(unsigned long a0, unsigned long a1, char a2)
{
    char v1;  // dh
    void* v2;  // rax

    g_41a420 = g_41a420 + v1;
    *((char *)v2 - 125) = *((char *)v2 - 125) >> 4 | *((char *)v2 - 125) * 16;
    g_41a3a9 = g_41a3a9 | 32;
    sub_40f6b0(stderr);
    sub_408440(); /* do not return */
}



// Function: xalloc_die @ 0xd430
extern long long stderr;
extern long long g_41a850;

void xalloc_die(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f6b0(stderr, "\n%s: memory_exhausted\n", g_41a850);
    sub_408440(); /* do not return */
}



// Function: warning @ 0xd460
extern unsigned long long g_0;
extern long long stderr;
extern long long g_41a420;
extern unsigned int g_41a84c;
extern long long g_41a850;
extern unsigned int g_41a85c;

unsigned long long warning(long long a0)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    if (g_41a85c)
    {
        v1 = g_41a84c;
        if ((unsigned int)v1)
            return v1;
        g_41a84c = 2;
        return v1;
    }
    else
    {
        v2 = sub_40f6b0(stderr, "%s: %s: warning: %s\n", g_41a850, &g_41a420, a0);
        if (g_41a84c)
            return v2;
        g_41a84c = 2;
        return v2;
    }
}



// Function: read_error @ 0xd4e0
extern unsigned long long g_0;
extern long long stderr;
extern char g_41a420;
extern long long g_41a850;

void read_error(void)
{
    unsigned long long v2;  // rbx
    unsigned int *err;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    err = __errno_location();
    sub_40f6b0(stderr, "\n%s: ", g_41a850);
    if (*(err))
    {
        *(err) = *(err);
        perror(&g_41a420);
    }
    else
    {
        sub_40f6b0(stderr, "%s: unexpected end of file\n", &g_41a420);
    }
    sub_408440(); /* do not return */
}



// Function: write_error @ 0xd550
extern long long stderr;
extern char g_41a020;
extern long long g_41a850;

void write_error(void)
{
    unsigned long long v2;  // rbx
    unsigned int *err;  // rax
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    err = __errno_location();
    sub_40f6b0(stderr, "\n%s: ", g_41a850);
    *(err) = *(err);
    perror(&g_41a020);
    sub_408440(); /* do not return */
}



// Function: flush_outbuf @ 0xd600
extern unsigned long long g_0;
extern unsigned int g_41a004;
extern unsigned int g_41a010;
extern long long g_45b000;

void flush_outbuf(void)
{
    if (g_41a004)
    {
        sub_40d5a0(g_41a010, &g_45b000);
        g_41a004 = 0;
    }
    return;
}



// Function: flush_window @ 0xd640
typedef struct struct_0 {
    int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned long long g_0;
extern struct_0 g_413800;
extern unsigned long long g_4183f0;
extern unsigned int g_41a004;
extern unsigned int g_41a010;
extern char g_43b000;

unsigned long long flush_window(void)
{
    unsigned long long v1;  // rax
    unsigned long v2;  // rax
    char *i;  // rdx
    char *v4;  // rdx
    unsigned long long v5;  // rax

    if (!g_41a004)
        return v1;
    v2 = g_4183f0;
    i = &g_43b000;
    do
    {
        v4 = i + 1;
        v2 = v2 >> 8 ^ *((long long *)&(&g_413800.field_0)[2 * (*(i) ^ (char)v2)]);
        i = v4;
    } while (i != &(&g_43b000)[g_41a004]);
    g_4183f0 = v2;
    v5 = sub_40d5a0(g_41a010, &g_43b000, g_41a004);
    g_41a004 = 0;
    return v5;
}



// Function: fill_inbuf @ 0xd6c0
extern unsigned long long g_0;
extern unsigned int g_41a008;
extern unsigned int g_41a00c;
extern unsigned int g_41a014;
extern unsigned long long g_41a828;
extern char g_49c000;

unsigned int fill_inbuf(unsigned int a0)
{
    unsigned long long v1;  // rax
    unsigned int v2;  // eax

    v1 = 0;
    g_41a00c = 0;
    while (1)
    {
        v2 = sub_40d190(g_41a014, &(&g_49c000)[v1], 0x40000 - v1);
        if (v2)
        {
            if (v2 == 0xffffffff)
                break;
            v1 = v2 + g_41a00c;
            g_41a00c = v1;
            if ((unsigned int)v1 > 0x3ffff)
            {
                g_41a008 = 1;
                g_41a828 = g_41a828 + v1;
                return g_49c000;
            }
        }
        else
        {
            v1 = g_41a00c;
            if (g_41a00c)
            {
                g_41a008 = 1;
                g_41a828 = g_41a828 + v1;
                return g_49c000;
            }
            if (a0)
                return 0xffffffff;
            sub_40d640();
            *(__errno_location()) = 0;
            break;
        }
    }
    sub_40d4e0(); /* do not return */
}



// Function: copy @ 0xd770
extern unsigned long long g_0;
extern unsigned int g_41a008;
extern unsigned int g_41a00c;
extern unsigned long long g_41a820;
extern unsigned long long g_41a828;
extern unsigned int g_41a858;
extern void g_49c000;

unsigned int copy(unsigned int a0, int a1)
{
    unsigned long v1;  // r13
    unsigned long v2;  // r14
    unsigned long v3;  // r13
    unsigned long v4;  // r14
    unsigned long v5;  // rdx
    unsigned long count;  // rax
    unsigned int v7;  // eax

    *(__errno_location()) = 0;
    v1 = g_41a00c;
    v2 = g_41a008;
    if (g_41a008 >= g_41a00c)
        return 0;
    while (1)
    {
        v3 = v1 - v2;
        v4 = &(&g_49c000)[v2];
        g_41a820 = g_41a820 + (v3 & 0xffffffff);
        if (!g_41a858)
        {
            while (1)
            {
                v5 = 0x7fffffff;
                if ((unsigned int)v3 <= 0x7fffffff)
                    v5 = v3;
                count = write(a1, v4, v5);
                if ((unsigned int)count == (unsigned int)v3)
                    break;
                if ((unsigned int)count == 0xffffffff)
                    sub_40d550(); /* do not return */
                v3 -= count;
                v4 += count & 0xffffffff;
            }
        }
        v7 = sub_40d190(a0, &g_49c000, 0x40000);
        v1 = v7;
        if (v7 == 0xffffffff)
            sub_40d4e0(); /* do not return */
        g_41a00c = v1;
        v2 = 0;
        g_41a828 = g_41a828 + v7;
        g_41a008 = 0;
        if (!(unsigned int)v1)
            return 0;
    }
}



// Function: display_ratio @ 0xd850

int display_ratio(unsigned long a0, unsigned long long a1, long long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    return sub_40f6b0(a2, "%5.1f%%");
}



// Function: fprint_off @ 0xd8a0
typedef struct FILE {
} FILE;

extern unsigned long long g_0;

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



// Function: zip @ 0xd9d0
extern unsigned long long g_0;
extern int g_41801c;
extern unsigned int g_418020;
extern long long stderr;
extern unsigned int g_41a004;
extern unsigned int g_41a010;
extern unsigned int g_41a014;
extern long long g_41a420;
extern unsigned long long g_41a828;
extern unsigned long long g_41a830;
extern unsigned long long g_41a838;
extern unsigned long long g_41a840;
extern char g_41a848;
extern long long g_41a850;
extern void g_45b000;
extern char g_45b002;
extern char g_45b003;
extern char g_45b004;
extern char g_45b005;
extern unsigned long long g_4dfea8;

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

    g_41a010 = a1;
    v0 = 0;
    g_41a014 = a0;
    *((unsigned short *)&g_45b000) = 35615;
    v2 = g_41a838;
    g_418020 = 8;
    g_45b002 = 8;
    g_41a004 = 4;
    g_45b003 = (*((int *)&g_41a848)) * 8;
    if (v2 - 0 >> 63 & 1)
    {
        v6 = 0;
        v5 = 0;
        v4 = 0;
        v3 = 0;
LABEL_40da79:
        g_45b004 = v3;
        v10 = 7;
        g_45b005 = v4;
        v9 = 6;
        goto LABEL_40da90;
    }
    else if (g_41a830 - 1 <= 0xfffffffe)
    {
        v3 = g_41a830;
        v4 = *((char *)((void*)&g_41a830 + 1));
        v5 = g_41a830 >> 16;
        v6 = v5;
        goto LABEL_40da79;
    }
    else
    {
        sub_40d460("file timestamp out of range for gzip format");
        v7 = g_41a004;
        if ((unsigned int)v7 > 0x3fffd)
        {
            v8 = (unsigned int)v7 + 1;
            (&g_45b000)[v7] = 0;
            g_41a004 = v8;
            if (v8 == 0x40000)
            {
                sub_40d600();
                v8 = g_41a004;
            }
            v9 = v8 + 1;
            g_41a004 = v9;
            (&g_45b000)[v8] = 0;
            if ((unsigned int)v9 != 0x40000)
            {
                v10 = v8 + 2;
            }
            else
            {
                sub_40d600();
                v9 = g_41a004;
                v10 = g_41a004 + 1;
                if (g_41a004 > 0x3fffd)
                    goto LABEL_40dd22;
LABEL_40df42:
                v6 = 0;
                v5 = 0;
LABEL_40da90:
                (&g_45b000)[v9 & 0xffffffff] = v5;
                g_41a004 = (unsigned int)v9 + 2;
                (&g_45b000)[v10] = v6;
                goto LABEL_40daa4;
            }
        }
        else
        {
            v9 = (unsigned int)v7 + 2;
            v10 = (unsigned int)v7 + 3;
            (&g_45b000)[v7 & 0xffffffff] = 0;
            (&g_45b000)[1 + v7] = 0;
        }
        if ((unsigned int)v9 <= 0x3fffd)
            goto LABEL_40df42;
LABEL_40dd22:
        g_41a004 = v10;
        (&g_45b000)[v9] = 0;
        if ((unsigned int)v10 == 0x40000)
        {
            sub_40d600();
            v10 = g_41a004;
        }
        (&g_45b000)[v10] = 0;
        g_41a004 = (unsigned int)v10 + 1;
        if (g_41a004 == 0x40000)
            sub_40d600();
    }
LABEL_40daa4:
    sub_40d0d0(0, 0);
    sub_404380(a1);
    sub_40aa20(&v0);
    v11 = 4;
    if (g_41801c != 1)
        v11 = (g_41801c == 9) * 2;
    v12 = g_41a004 + 1;
    (&g_45b000)[g_41a004] = v11;
    g_41a004 = v12;
    if ((unsigned int)v12 == 0x40000)
    {
        sub_40d600();
        v12 = g_41a004;
    }
    v13 = (unsigned int)v12 + 1;
    (&g_45b000)[v12] = 3;
    g_41a004 = v13;
    if ((unsigned int)v13 == 0x40000)
    {
        sub_40d600();
        v13 = g_41a004;
    }
    if (*((int *)&g_41a848))
    {
        iter = (unsigned long long)sub_40d290(&g_41a420);
        while (1)
        {
            v15 = *((char *)iter);
            g_41a004 = (unsigned int)v13 + 1;
            (&g_45b000)[v13] = v15;
            v13 = g_41a004;
            if (g_41a004 != 0x40000)
            {
                iter += 1;
                if (!*((char *)iter - 1))
                    break;
            }
            else
            {
                sub_40d600();
                iter += 1;
                v13 = g_41a004;
                if (!*((char *)iter - 1))
                    break;
            }
        }
    }
    g_4dfea8 = v13 & 0xffffffff;
    sub_404bc0(g_41801c);
    if (g_41a840 != 0xffffffffffffffff && g_41a840 != g_41a828)
        sub_40f6b0(stderr, "%s: %s: file size changed while zipping\n", g_41a850, &g_41a420);
    v16 = sub_40d130();
    v17 = g_41a004;
    idx = (unsigned int)v17 + 1;
    g_41a004 = idx;
    if ((unsigned int)v17 <= 0x3fffd)
    {
        v19 = (unsigned int)v17 + 2;
        (&g_45b000)[v17 & 0xffffffff] = v16;
        v20 = sub_40d130();
        g_41a004 = v19;
        (&g_45b000)[idx] = v20;
    }
    else
    {
        (&g_45b000)[v17] = v16;
        if ((unsigned int)idx == 0x40000)
        {
            sub_40d600();
            idx = g_41a004;
        }
        v21 = sub_40d130();
        v19 = (unsigned int)idx + 1;
        g_41a004 = v19;
        (&g_45b000)[idx] = v21;
        if ((unsigned int)v19 == 0x40000)
        {
            sub_40d600();
            v19 = g_41a004;
        }
    }
    v22 = sub_40d130();
    index = (unsigned int)v19 + 1;
    g_41a004 = index;
    v24 = v22 >> 16;
    if ((unsigned int)v19 <= 0x3fffd)
    {
        v25 = (unsigned int)v19 + 2;
        (&g_45b000)[v19 & 0xffffffff] = v24;
        (&g_45b000)[index] = *((char *)((void*)&sub_40d130() >> 16 + 1));
    }
    else
    {
        (&g_45b000)[v19] = v24;
        if ((unsigned int)index == 0x40000)
        {
            sub_40d600();
            index = g_41a004;
        }
        v26 = sub_40d130();
        v25 = (unsigned int)index + 1;
        g_41a004 = v25;
        (&g_45b000)[index] = *((char *)((void*)&v26 >> 16 + 1));
        if ((unsigned int)v25 == 0x40000)
        {
            sub_40d600();
            v25 = g_41a004;
        }
    }
    v27 = g_41a828;
    idx1 = (unsigned int)v25 + 1;
    if ((unsigned int)v25 <= 0x3fffd)
    {
        v29 = (unsigned int)v25 + 2;
        (&g_45b000)[v25 & 0xffffffff] = g_41a828;
        (&g_45b000)[idx1] = *((char *)((void*)&g_41a828 + 1));
    }
    else
    {
        g_41a004 = idx1;
        (&g_45b000)[v25] = g_41a828;
        if ((unsigned int)idx1 == 0x40000)
        {
            sub_40d600();
            v27 = g_41a828;
            idx1 = g_41a004;
        }
        v29 = (unsigned int)idx1 + 1;
        (&g_45b000)[idx1] = *((char *)((void*)&v27 + 1));
        g_41a004 = v29;
        if ((unsigned int)v29 == 0x40000)
        {
            sub_40d600();
            v29 = g_41a004;
            v27 = g_41a828;
        }
    }
    v30 = v27 >> 16;
    idx2 = (unsigned int)v29 + 1;
    if ((unsigned int)v29 <= 0x3fffd)
    {
        (&g_45b000)[v29 & 0xffffffff] = v30;
        g_41a004 = (unsigned int)v29 + 2;
        (&g_45b000)[idx2] = v30;
    }
    else
    {
        g_41a004 = idx2;
        (&g_45b000)[v29] = v30;
        if ((unsigned int)idx2 == 0x40000)
        {
            sub_40d600();
            idx2 = g_41a004;
            v30 = g_41a828 >> 16;
        }
        (&g_45b000)[idx2] = v30;
        g_41a004 = (unsigned int)idx2 + 1;
        if (g_41a004 == 0x40000)
            sub_40d600();
    }
    g_4dfea8 = g_4dfea8 + 8;
    sub_40d600();
    return 0;
}



// Function: file_read @ 0xdf80
void file_read(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40df8f();
    return;
}



// Function: last_component @ 0xdfd0
extern unsigned long long g_0;

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



// Function: rpl_fcntl @ 0xe0f0
extern unsigned long long g_0;
extern int g_4dfeb0;

unsigned int rpl_fcntl(int a0, unsigned int a1)
{
    unsigned long v8;  // rdx
    unsigned long long v9;  // rcx
    unsigned long long v10;  // r8
    unsigned long long v11;  // r9
    unsigned int v12;  // eax
    unsigned int v13;  // ecx
    unsigned int v14;  // eax
    unsigned int v15;  // r12d
    unsigned int v16;  // r12d
    unsigned int *err;  // rax
    unsigned int v0;  // [bp-0x68]
    char *v1;  // [bp-0x60]
    char *v2;  // [bp-0x58]
    int v3;  // [bp-0x48]
    unsigned long long v4;  // [bp-0x30]
    unsigned long long v5;  // [bp-0x28]
    unsigned long long v6;  // [bp-0x20]
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
        return fcntl(a0, 0);
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
                    goto LABEL_40e260;
                if ((unsigned short)v14 & 1282)
                    return fcntl(a0, a1);
            }
        }
        else if (a1 > 0)
        {
            v12 = 1 << ((char)a1 & 63);
            if ((unsigned short)v12 & 1300)
            {
LABEL_40e260:
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
        v0 = 24;
        if (g_4dfeb0 >= 0)
        {
            v15 = fcntl(a0, 1030);
            if (v15 >= 0 || *(__errno_location()) != 22)
            {
                g_4dfeb0 = 1;
                return v15;
            }
            v16 = fcntl(a0, 0);
            if (v16 < 0)
                return v16;
            g_4dfeb0 = 0xffffffff;
        }
        else
        {
            v16 = fcntl(a0, 0);
            if (v16 < 0)
            {
                return v16;
            }
            else if (g_4dfeb0 != -0x1)
            {
                return v16;
            }
        }
        if (fcntl(v16, 1) >= 0 && fcntl(v16, 2) != -0x1)
            return v16;
        err = __errno_location();
        close(v16);
        *(err) = *(err);
        return 0xffffffff;
    }
}



// Function: open_safer @ 0xe360
extern unsigned long long g_0;

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
    return sub_40e870(open(a0, a1, v5) & 0xffffffff);
}



// Function: rpl_fseeko @ 0xe430
typedef struct FILE {
} FILE;

extern unsigned long long g_0;

int rpl_fseeko(FILE *a0, int a1, int a2)
{
    unsigned long long v1;  // rax

    if (*((long long *)&a0[16]) == *((long long *)&a0[8]) && *((long long *)&a0[40]) == *((long long *)&a0[32]) && !*((long long *)&a0[72]))
    {
        v1 = lseek(fileno(a0), a1, a2);
        if (v1 == 0xffffffffffffffff)
            return -0x1;
        *((unsigned int *)&a0) = *((int *)&a0) & 0xffffffef;
        *((unsigned long long *)&a0[144]) = v1;
        return 0;
    }
    return fseeko(a0, a1, a2);
}



// Function: openat_safer @ 0xe4b0
extern unsigned long long g_0;

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
    return sub_40e870(openat() & 0xffffffff);
}



// Function: direntry_cmp_name @ 0xe530
int direntry_cmp_name(char **a0, char **a1)
{
    return strcmp(*(a0), *(a1));
}



// Function: streamsavedir @ 0xe540
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

extern unsigned long long g_0;
extern unsigned long long g_417b50[4];

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
    v0 = g_417b50[a1];
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
                    ptr = sub_40f440(ptr, &v4, 1, -0x1, 8);
                }
                v2 = n;
                v16 = i + 1;
                v1 = ptr + i * 8;
                v17 = v2;
                *(v1) = sub_40f640(ptr1);
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
                    v9 = sub_40f440(v9, &v3, n - v18, 0x7ffffffffffffffe, 1);
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
                v9 = sub_40f180(v20);
                v21 = v9;
            }
            else
            {
                v22 = 0;
                qsort(ptr, i, 8, v0);
                v23 = 0;
                v9 = sub_40f180(v20);
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
                v9 = sub_40f1f0(v9, v6 + 1);
            v21 = &v9[v6];
            break;
        }
    }
    *(v21) = 0;
    return v9;
}



// Function: savedir @ 0xe800
void savedir(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40e80a();
    return;
}



// Function: fd_safer @ 0xe870
extern unsigned long long g_0;

int fd_safer(int a0)
{
    int v1;  // eax
    unsigned int *err;  // rax

    if (a0 <= 2)
    {
        v1 = sub_410c00();
        err = __errno_location();
        close(a0);
        *(err) = *(err);
        return v1;
    }
    return a0;
}



// Function: validate_timespec @ 0xe8d0
extern unsigned long long g_0;

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



// Function: update_timespec @ 0xe980
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_0;

extern unsigned long long g_0;

unsigned int update_timespec(unsigned long long *index, struct_0 **a1)
{
    unsigned long long *idx;  // rbx
    unsigned long long v2;  // rax
    unsigned long long v3;  // rax
    unsigned long long v4;  // rdx

    idx = &*(a1)->field_0;
    v2 = idx[3];
    if (idx[1] == 0x3ffffffe)
    {
        if (v2 == 0x3ffffffe)
            return 1;
        v4 = index[10];
        *(idx) = index[9];
        idx[1] = v4;
    }
    else if (idx[1] != 0x3fffffff)
    {
        if (v2 == 0x3ffffffe)
        {
            v3 = index[12];
            idx[2] = index[11];
            idx[3] = v3;
            return 0;
        }
    }
    else
    {
        if (v2 != 0x3fffffff)
        {
            sub_410ba0(idx);
            v2 = idx[3];
            if (v2 == 0x3ffffffe)
            {
                v3 = index[12];
                idx[2] = index[11];
                idx[3] = v3;
                return 0;
            }
        }
        else
        {
            *(a1) = 0;
            return 0;
        }
    }
    if (v2 == 0x3fffffff)
        sub_410ba0(idx + 2);
    return 0;
}



// Function: fdutimens @ 0xea40
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

extern unsigned long long g_0;
extern unsigned int g_4dfeb4;
extern int g_4dfeb8;

unsigned int fdutimens(unsigned int a0, unsigned long a1, void* idx)
{
    int v13;  // eax
    unsigned int v23;  // eax
    char v24;  // r13b
    char v26;  // dl
    char v15;  // al
    unsigned int v17;  // r12d
    char v19;  // al
    void* v21;  // r14
    void* v0;  // [bp-0x130]
    unsigned long long v1[4];  // [bp-0x128]
    int v2;  // [bp-0x108], Other Possible Types: unsigned long long (64 bits)[4]
    int v3;  // [bp-0xe8]
    unsigned long long v4;  // [bp-0xe0]
    int v5;  // [bp-0xd8]
    unsigned long long v6;  // [bp-0xd0]
    int v7;  // [bp-0xc8], Other Possible Types: stat
    unsigned long v8;  // [bp-0x80]
    unsigned long long v9;  // [bp-0x78]
    unsigned long v10;  // [bp-0x70]
    unsigned long long v11;  // [bp-0x68]

    if (!idx)
    {
        v0 = 0;
        v13 = 0;
        goto LABEL_40eaaa;
    }
    v0 = v1;
    v1 = (unsigned long long (64 bits)[4])_INSERT(v1 CONCAT 0, 0, *((int128_t *)idx));
    *((int128_t *)&v1[2]) = (int128_t)idx[16];
    v13 = sub_40e8d0(v1);
    if (v13 < 0)
        return 0xffffffff;
    idx = v1;
LABEL_40eaaa:
    if (!(a0 >= 0 || a1))
    {
        *(__errno_location()) = 9;
    }
    else if (g_4dfeb8 < 0)
    {
LABEL_40eaf3:
        g_4dfeb8 = 0xffffffff;
        g_4dfeb4 = 0xffffffff;
        if (!v13)
        {
LABEL_40eb52:
            v21 = v0;
            if (v21)
            {
                *((long long *)&v2) = *((long long *)v21);
                v2[2] = (long long)v21[16];
                *((long long *)&(&v2)[8]) = ((long long)v21[8] * 2361183241434822607 >> 71) - ((long long)v21[8] >> 63);
                v2[3] = ((long long)v21[24] * 2361183241434822607 >> 71) - ((long long)v21[24] >> 63);
                v21 = v2;
            }
            if (a0 < 0)
                return futimesat(0xffffff9c, a1);
            v23 = futimesat(a0, 0);
            if (v23)
            {
                if (a1)
                    return utimes(a1, v21);
                return 0xffffffff;
            }
            else
            {
                if (!v21)
                    return v23;
                v24 = 499999 < (long long)v21[24];
                if (499999 >= (long long)v21[8] && !v24)
                    return v23;
                if (fstat(a0, &v7))
                    return v23;
                v3 = (int)*((int128_t *)v21);
                v5 = (int)(int128_t)v21[16];
                v26 = v10 - (long long)v21[16] == 1 & v24;
                if (v8 - *((long long *)v21) == 1 && 499999 < (long long)v21[8] && !v9)
                {
                    v4 = 0;
                    if (!v26)
                    {
                        futimesat(a0, 0, &v3);
                        return 0;
                    }
                    if (v11)
                    {
                        futimesat(a0, 0, &v3);
                        return 0;
                    }
                }
                else
                {
                    if (!v26)
                        return 0;
                    if (v11)
                        return 0;
                }
                v6 = 0;
                futimesat(a0, 0, &v3);
                return 0;
            }
        }
        if (v13 != 3)
        {
            if (a0 >= 0)
                v19 = fstat(a0, &v7);
            else
                v19 = stat(a1, &v7);
            if (!v19)
                goto LABEL_40eb33;
        }
        else
        {
LABEL_40eb33:
            if (idx && (char)sub_40e980(&v7, &v0))
                return 0;
            goto LABEL_40eb52;
        }
    }
    else if (v13 != 2)
    {
LABEL_40eacb:
        if (a0 >= 0)
        {
            v17 = futimens(a0, idx);
            if (v17 <= 0)
            {
                if (!v17)
                {
                    g_4dfeb8 = 1;
                    return v17;
                }
                goto LABEL_40ee20;
            }
        }
        else
        {
            v17 = utimensat(0xffffff9c, a1, idx, 0);
            if (v17 > 0)
            {
                *(__errno_location()) = 38;
                goto LABEL_40eaf3;
            }
            else
            {
                if (!v17)
                {
                    g_4dfeb8 = 1;
                    return v17;
                }
LABEL_40ee20:
                if (*(__errno_location()) != 38)
                {
                    g_4dfeb8 = 1;
                    return v17;
                }
            }
        }
    }
    else
    {
        if (a0 >= 0)
            v15 = fstat(a0, &v7);
        else
            v15 = stat(a1, &v7);
        if (!v15)
        {
            if ((long long)idx[8] != 0x3ffffffe)
            {
                v13 = 3;
                if ((long long)idx[24] == 0x3ffffffe)
                {
                    *((unsigned long *)&idx[16]) = v10;
                    *((unsigned long long *)&idx[24]) = v11;
                    goto LABEL_40eacb;
                }
            }
            else
            {
                v13 = 3;
                *((unsigned long *)idx) = v8;
                *((unsigned long long *)&idx[8]) = v9;
                goto LABEL_40eacb;
            }
        }
    }
    return 0xffffffff;
}



// Function: lutimens @ 0xee90
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

extern unsigned long long g_0;
extern int g_4dfeb4;
extern unsigned int g_4dfeb8;

unsigned int lutimens(unsigned long a0, struct_0 *a1)
{
    struct_0 *v11;  // r12
    unsigned int v12;  // r13d
    unsigned int v14;  // r12d
    unsigned long long *v0;  // [bp-0xf0]
    unsigned long long v1[2];  // [bp-0xe8]
    unsigned long long v2;  // [bp-0xe0]
    int v3;  // [bp-0xd8], Other Possible Types: unsigned long long
    unsigned long long v4;  // [bp-0xd0]
    stat v5;  // [bp-0xc8]
    unsigned long long v6;  // [bp-0x80]
    unsigned long long v7;  // [bp-0x78]
    unsigned long long v8;  // [bp-0x70]
    unsigned long long v9;  // [bp-0x68]

    if (!a1)
    {
        v0 = 0;
        if (g_4dfeb4 < 0)
        {
            g_4dfeb4 = 0xffffffff;
            goto LABEL_40f02c;
        }
LABEL_40ef48:
        v14 = utimensat(0xffffff9c, a0, a1, 0x100);
        if (v14 > 0)
        {
            *(__errno_location()) = 38;
            goto LABEL_40ef7b;
        }
        else if (!v14 || *(__errno_location()) != 38)
        {
            g_4dfeb8 = 1;
            g_4dfeb4 = 1;
            return v14;
        }
    }
    v11 = v1;
    v0 = &v1[0];
    v1 = a1->field_0;
    v3 = (int)a1->field_10;
    v12 = sub_40e8d0(v1);
    if (v12 < 0)
        return 0xffffffff;
    if (g_4dfeb4 >= 0)
    {
        if (v12 != 2)
        {
            a1 = v1;
            goto LABEL_40ef48;
        }
        else if (lstat(a0, &v5))
        {
            return 0xffffffff;
        }
        else if (*(&v1[1]) != 0x3ffffffe)
        {
            a1 = v11;
            if (*((unsigned long long *)(&v3 + 8)) == 0x3ffffffe)
            {
                v3 = v8;
                v4 = v9;
                a1 = v11;
                goto LABEL_40ef48;
            }
        }
        else
        {
            a1 = v1;
            v1 = (unsigned long long (64 bits)[2])v6;
            v2 = v7;
            goto LABEL_40ef48;
        }
    }
    a1 = v1;
LABEL_40ef7b:
    g_4dfeb4 = 0xffffffff;
    switch (v12)
    {
    case 0:
LABEL_40f02c:
        if (!lstat(a0, &v5))
            goto LABEL_40efb3;
        break;
    case 3:
        if (a1)
            goto LABEL_40ef9d;
        goto LABEL_40efb3;
    default:
        if (!lstat(a0, &v5))
        {
            if (a1)
            {
LABEL_40ef9d:
                if ((char)sub_40e980(&v5, &v0))
                    return 0;
            }
LABEL_40efb3:
            if (((unsigned short)v5.st_nlink & 0xf000) != 0xa000)
                return sub_40ea40(0xffffffff, a0, v0);
            *(__errno_location()) = 38;
            break;
        }
    }
    return 0xffffffff;
}



// Function: xmalloc @ 0xf160
extern unsigned long long g_0;

void* xmalloc(unsigned long a0)
{
    void* ptr;  // rax

    ptr = malloc(a0);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: ximalloc @ 0xf180
extern unsigned long long g_0;

void* ximalloc(unsigned long a0)
{
    void* ptr;  // rax

    ptr = malloc(a0);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: xirealloc @ 0xf1f0
extern unsigned long long g_0;

void* xirealloc(void* a0, unsigned long long a1)
{
    void* ptr;  // rax

    ptr = realloc(a0, a1 | !a1);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: xnmalloc @ 0xf2a0
extern unsigned long long g_0;

void* xnmalloc(unsigned long a0, unsigned long a1)
{
    void* ptr;  // rax

    ptr = reallocarray(NULL, a0, a1);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: xinmalloc @ 0xf2d0
extern unsigned long long g_0;

void* xinmalloc(unsigned long a0, unsigned long a1)
{
    void* ptr;  // rax

    if (!a0 || !a1)
    {
        a0 = 1;
        a1 = 1;
    }
    ptr = reallocarray(NULL, a0, a1);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: x2realloc @ 0xf320
extern unsigned long long g_0;

void* x2realloc(void* a0, void* a1)
{
    unsigned long v1;  // rbx
    unsigned long v2;  // rax
    unsigned long v3;  // rbx
    void* ptr;  // rax
    void* ptr1;  // rax

    v1 = *((long long *)a1);
    if (a0)
    {
        v2 = (v1 >> 1) + 1;
        v3 = v1 + v2;
        if (!(__CFADD__(v1, v2) & 1))
        {
            ptr = reallocarray(a0, v3, 1);
            if (ptr || !v3)
            {
                *((unsigned long *)a1) = v3;
                return ptr;
            }
        }
    }
    else
    {
        if (!v1)
            v1 = 128;
        ptr1 = reallocarray(NULL, v1, 1);
        if (ptr1)
        {
            *((unsigned long *)a1) = v1;
            return ptr1;
        }
    }
    sub_40d430(); /* do not return */
}



// Function: xpalloc @ 0xf440
extern unsigned long long g_0;

void* xpalloc(void* a0, long long *a1, long long a2, long long a3, unsigned long a4)
{
    long long v1;  // rsi
    long long v2;  // rcx
    long long v3;  // rbx
    long long v4;  // rbx
    long long v5;  // rax
    long long v6;  // rbp
    unsigned long v7;  // rbp
    unsigned long v8;  // rbp
    unsigned long v9;  // rax
    void* ptr;  // rax

    v1 = a3;
    v2 = *(a1);
    v3 = v2 >> 1;
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
        v8 = 0x7fffffffffffffff;
        goto LABEL_40f4ed;
    }
    else if (v7 <= 127)
    {
        v8 = 128;
LABEL_40f4ed:
        v9 = v8;
        v4 = v9 / a4;
        v7 = v8 - v9 % a4;
        if (!a0)
        {
LABEL_40f500:
            *(a1) = 0;
        }
    }
    else if (!a0)
    {
        goto LABEL_40f500;
    }
    if (v4 - v2 >= a2 || (v4 = v2 + a2, !((unsigned long long)__OFADD__(v2, a2) & 1) && (v4 <= v1 || v1 < 0) && !(v7 = a4 * v4, (unsigned long long)__OFSMUL__(a4, v4) & 1)))
    {
        ptr = realloc(a0, v7);
        if (ptr || a0 && !v7)
        {
            *(a1) = v4;
            return ptr;
        }
    }
    sub_40d430(); /* do not return */
}



// Function: xcalloc @ 0xf540
extern unsigned long long g_0;

void* xcalloc(unsigned long a0, unsigned long a1)
{
    void* ptr;  // rax

    ptr = calloc(a0, a1);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: xicalloc @ 0xf570
extern unsigned long long g_0;

void* xicalloc(unsigned long a0, unsigned long a1)
{
    void* ptr;  // rax

    ptr = calloc(a0, a1);
    if (!ptr)
        sub_40d430(); /* do not return */
    return ptr;
}



// Function: ximemdup @ 0xf5d0
void ximemdup(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f5e0();
    return;
}



// Function: ximemdup0 @ 0xf600
void ximemdup0(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40f611();
    return;
}



// Function: xstrdup @ 0xf640
long long xstrdup(char *a0)
{
    strlen(a0);
}



// Function: yesno @ 0xf660
typedef struct FILE {
} FILE;

extern unsigned long long g_0;
extern FILE *stdin;

int yesno(void)
{
    unsigned int v1;  // eax
    unsigned int v2;  // eax
    unsigned long v3;  // r12

    v1 = getc(stdin);
    v2 = v1;
    if (v2 != 10)
    {
        do
        {
        } while (v2 != 0xffffffff && (v2 = (unsigned int)(int)(long long)getc(stdin), v2 != 10));
    }
    return _INSERT(v3, 0, (v1 & 0xffffffdf) == 89);
}



// Function: rpl_fprintf @ 0xf6b0
extern unsigned long long g_0;

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
    ptr = sub_40f900(&v6, &v1, a1, &v2);
    if (!ptr)
    {
LABEL_40f817:
        sub_410b90(a0);
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
            goto LABEL_40f817;
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



// Function: __printf__ @ 0xf830
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
    v19 = sub_410a00(stdout, a0, &v0);
    if (v4 != *((long long *)(40 + v18)))
        __stack_chk_fail(); /* do not return */
    return v19;
}



// Function: vasnprintf @ 0xf900
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

extern unsigned long long g_0;
extern unsigned int g_414090[4];

void* vasnprintf(void* a0, void* a1, unsigned long a2, long long a3)
{
    unsigned long ptr;  // r8
    unsigned long long v26;  // rax
    unsigned long v35;  // r14
    unsigned long long n;  // rbx
    unsigned long long v37;  // rax
    char *v38;  // r14
    struct_0 *v39;  // rax
    char v40;  // al
    struct_3 *iter;  // r12
    unsigned long v42;  // rsi
    unsigned long v43;  // rsi
    char v44;  // al
    unsigned long v27;  // rdi
    struct_1 *v45;  // rax
    struct_1 *v46;  // rcx
    unsigned long index;  // rax
    unsigned long long v48;  // rax
    unsigned long long v49;  // rax
    void* ptr1;  // r14
    void* dst;  // r12
    unsigned int v53;  // ftop
    unsigned int v54;  // ftop
    unsigned long v28;  // rdi
    unsigned long long v55;  // rsi
    unsigned long long v56;  // r10
    unsigned long long v57;  // r14
    unsigned long v58;  // r9
    unsigned int v59;  // eax
    unsigned long v60;  // r9
    unsigned int v61;  // ftop
    unsigned int v62;  // ftop
    unsigned int v63;  // eax
    unsigned int v64;  // eax
    unsigned long long v29;  // r12
    unsigned long long v65;  // r10
    unsigned int v66;  // eax
    unsigned int v67;  // eax
    unsigned int v68;  // edx
    unsigned long long v69;  // rcx
    unsigned long v70;  // rax
    unsigned long long v71;  // rax
    unsigned long long v72;  // rcx
    unsigned long long v73;  // rdx
    char v74;  // r14b
    unsigned long v30;  // rax
    void* ptr2;  // rax
    void* v76;  // rax
    void* v77;  // rax
    char v78;  // r14b
    void* v79;  // rax
    void* v80;  // r9
    void* v81;  // rax
    unsigned long n1;  // r14
    unsigned long long v83;  // rbx
    void* v84;  // rax
    unsigned long long v31;  // rdx
    void* v85;  // rax
    void* v86;  // r14
    unsigned int v87;  // eax
    void* v88;  // r14
    unsigned long long v89;  // r12
    unsigned long long v90;  // r12
    char v91;  // r14b
    void* v92;  // rax
    void* v93;  // rax
    void* v94;  // rax
    unsigned long long v32;  // r13
    void* v95;  // rdi
    char *v96;  // rdi
    void* node;  // r15
    void* dst1;  // r9
    char v0;  // [bp-0x1428]
    char v1;  // [bp-0x4a8]
    char v2;  // [bp-0x428]
    unsigned long long v3;  // [bp-0x420]
    unsigned int v4;  // [bp-0x414]
    struct_3 *v5;  // [bp-0x410]
    struct_3 *v6;  // [bp-0x400], Other Possible Types: void*
    unsigned long long v7;  // [bp-0x3f8]
    struct_3 *v8;  // [bp-0x3f0]
    void* v9;  // [bp-0x3e8], Other Possible Types: unsigned int, unsigned long
    void* v10;  // [bp-0x3e0], Other Possible Types: unsigned int, unsigned long
    int *err;  // [bp-0x3d8], Other Possible Types: void*, char, unsigned long
    void* v12;  // [bp-0x3d0]
    unsigned int v13;  // [bp-0x3c4]
    unsigned int v14;  // [bp-0x3c0]
    unsigned int v15;  // [bp-0x3bc]
    char v16;  // [bp-0x3b8]
    char *v17;  // [bp-0x3b0]
    char v18;  // [bp-0x3a8]
    unsigned long long v19;  // [bp-0x2c8]
    void* v20;  // [bp-0x2c0]
    unsigned long long v21;  // [bp-0x2b8]
    unsigned long v22;  // [bp-0x2b0]
    char v23;  // [bp-0x2a8]

    v12 = a0;
    err = a2;
    if (sub_410e50(a2, &v19, &v16) < 0)
        return 0;
    ptr = err;
    if (sub_410c20(a3, &v16) >= 0)
    {
        v26 = v21 + 7;
        if (0xfffffffffffffff9 <= v21)
            v26 = 0xffffffffffffffff;
        if (__CFADD__(v26, v22) & 1 || (v27 = v26 + v22, v28 = (unsigned long)(v27 + 6), v29 = (unsigned long long)__CFADD__(v27, 6) & 1, (unsigned long long)__CFADD__(v27, 6) & 1))
        {
LABEL_40ffee:
            err = __errno_location();
            goto LABEL_40fab0;
        }
        if (v28 > 3999)
        {
            err = ptr;
            if (v28 == 0xffffffffffffffff || (/* unsupported instruction */ = (int)(&v1 - 120), v8 = (struct_3 *)malloc(v28), !v8))
                goto LABEL_40ffee;
            v5 = v8;
            ptr = err;
        }
        else
        {
            v30 = v27 + 29;
            while (&v0 != &(&v2)[-1 * (v30 & 0xfffffffffffff000)])
            ;
            v31 = (unsigned int)v30 & 0xfffffff0 & 0xfff;
            /* unsupported instruction */ = (int)&(&v0)[-1 * v31];
            if (v31)
                *((long long *)(/* unsupported instruction */ + v31 - 8)) = *((long long *)(/* unsupported instruction */ + v31 - 8));
            v5 = 0;
            v8 = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
        }
        v32 = 0;
        if (v12)
            v32 = *((long long *)a1);
        node = v20;
        dst1 = v12;
        v7 = 0;
        v35 = *((long long *)node);
        if (v35 == ptr)
        {
LABEL_40ff46:
            n = v29;
            goto LABEL_40fbd2;
        }
LABEL_40fa35:
        n1 = v35 - ptr;
        v83 = v29;
        n = v83 + n1;
        if (__CFADD__(v83, n1) & 1)
            n = 0xffffffffffffffff;
        if (v32 < n)
        {
            if (v32)
            {
                if (v32 < 0)
                    goto LABEL_40fa70;
                v32 *= 2;
            }
            else
            {
                v32 = 12;
            }
            if (v32 < n)
                v32 = n;
            switch (v32)
            {
            case 18446744073709551615:
                goto LABEL_40fa70;
                break;
            }
            if (dst1 && !err)
            {
                err = dst1;
                v10 = ptr;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v84 = realloc(dst1, v32);
                v80 = err;
                if (!v84)
                    goto LABEL_40ffff;
                ptr = v10;
                dst1 = v84;
            }
            else
            {
                v9 = ptr;
                v10 = dst1;
                err = dst1 == v12;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v85 = malloc(v32);
                ptr = v9;
                if (!v85)
                {
LABEL_40fa77:
                    v88 = v10;
                    err = __errno_location();
                    goto LABEL_40fa8a;
                }
                else if (v29 && err)
                {
                    err = ptr;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    ptr = err;
                    dst1 = memcpy(v85, v10, v29);
                }
                else
                {
                    dst1 = v85;
                }
            }
        }
        err = dst1;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        memcpy(dst1 + v29, ptr, n1);
        dst1 = err;
LABEL_40fbd2:
        if (v19 != v7)
        {
            v37 = (long long)node[80];
            if ((char)node[72] == 37)
            {
                if (v37 != 0xffffffffffffffff)
                    goto LABEL_4109d0;
                v29 = n + 1;
                if (0xffffffffffffffff <= n)
                    v29 = 0xffffffffffffffff;
                if (v32 < v29)
                {
                    if (!v32)
                    {
                        v32 = 12;
                    }
                    else
                    {
                        if (v32 < 0)
                            goto LABEL_40fa70;
                        v32 *= 2;
                    }
                    if (v32 < v29)
                        v32 = v29;
                    if (v32 == 0xffffffffffffffff)
                        goto LABEL_40fa70;
                    v78 = dst1 == v12;
                    if (dst1 && !v78)
                    {
                        err = dst1;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v79 = realloc(dst1, v32);
                        v80 = err;
                        if (!v79)
                            goto LABEL_40ffff;
                        dst1 = v79;
                    }
                    else
                    {
                        err = dst1;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v81 = malloc(v32);
                        dst1 = err;
                        if (!v81)
                            goto LABEL_40fa70;
                        if (n && v78)
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            dst1 = memcpy(v81, dst1, n);
                        }
                        else
                        {
                            dst1 = v81;
                        }
                    }
                }
                *((char *)dst1 + n) = 37;
                goto LABEL_40ff29;
            }
            if (v37 == 0xffffffffffffffff)
                goto LABEL_4109d0;
            v38 = v17;
            v39 = &v38[32 * v37];
            v10 = v39->field_0;
            if ((char)node[72] == 110)
            {
                switch (v10)
                {
                case 18:
                    *((char *)v77) = n;
                    v29 = n;
                    break;
                case 19:
                    *((unsigned short *)v77) = n;
                    v29 = n;
                    break;
                case 20:
                    *((unsigned int *)v77) = n;
                    v29 = n;
                    break;
                case 21: case 22:
                    *((unsigned long long *)v77) = n;
                    v29 = n;
                    break;
                default:
LABEL_4109d0:
                    abort(); /* do not return */
                }
                goto LABEL_40ff29;
            }
            v40 = (int)node[16];
            iter = &v8->padding_0[1];
            v8->padding_0[0] = 37;
            if (v40 & 1)
            {
                v8->padding_0[1] = 39;
                iter = &v8->field_2;
            }
            if (v40 & 2)
            {
                iter->padding_0[0] = 45;
                iter = &iter->padding_0[1];
            }
            if (v40 & 4)
            {
                iter->padding_0[0] = 43;
                iter = &iter->padding_0[1];
            }
            if (v40 & 8)
            {
                iter->padding_0[0] = 32;
                iter = &iter->padding_0[1];
            }
            if (v40 & 16)
            {
                iter->padding_0[0] = 35;
                iter = &iter->padding_0[1];
            }
            if (v40 & 64)
            {
                iter->padding_0[0] = 73;
                iter = &iter->padding_0[1];
            }
            if (v40 & 32)
            {
                iter->padding_0[0] = 48;
                iter = &iter->padding_0[1];
            }
            v42 = (long long)node[24];
            if (v42 != (long long)node[32])
            {
                v9 = dst1;
                err = (long long)node[32] - v42;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                memcpy(iter, v42, err);
                dst1 = v9;
                iter = &iter->padding_0[err];
            }
            v43 = (long long)node[48];
            if (v43 != (long long)node[56])
            {
                v9 = dst1;
                err = (long long)node[56] - v43;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                memcpy(iter, v43, err);
                dst1 = v9;
                iter = &iter->padding_0[err];
            }
            switch (v10)
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
            v44 = (char)node[72];
            iter->padding_0[1] = 0;
            iter->padding_0[0] = v44;
            if ((long long)node[40] != 0xffffffffffffffff)
            {
                v45 = &v38[32 * node[40]];
                if (*((int *)&v45->padding_0[0]) != 5)
                    goto LABEL_4109d0;
                v9 = 1;
                v14 = v45->field_10;
            }
            else
            {
                v9 = 0;
            }
            if ((long long)node[64] != 0xffffffffffffffff)
            {
                v46 = &v38[32 * node[64]];
                if (*((int *)&v46->padding_0[0]) != 5)
                    goto LABEL_4109d0;
                index = v9;
                (&v14)[index] = v46->field_10;
                v9 = (unsigned int)index + 1;
            }
            v48 = n;
            v49 = v48 + 2;
            if (__CFADD__(v48, 2) & 1)
            {
                if (v32 != 0xffffffffffffffff)
                    goto LABEL_40fa70;
                goto LABEL_410628;
            }
            if (v32 < v49)
            {
                if (v32)
                {
                    if (v32 < 0)
                        goto LABEL_40fa70;
                    v32 *= 2;
                    if (v32 >= v49)
                        goto LABEL_40fde5;
                }
                else if (v49 <= 12)
                {
                    v32 = 12;
                    goto LABEL_40fde5;
                }
                if (v49 == 0xffffffffffffffff)
                    goto LABEL_40fa70;
                v32 = v49;
LABEL_40fde5:
                if (dst1 && !err)
                {
                    err = dst1;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    ptr1 = realloc(dst1, v32);
                    dst1 = err;
                    if (!ptr1)
                        goto LABEL_40fa70;
                    goto LABEL_40fe25;
                }
                else
                {
                    v6 = dst1;
                    err = dst1 == v12;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    ptr1 = malloc(v32);
                    dst1 = v6;
                    if (!ptr1)
                        goto LABEL_40fa70;
                    if (n && err)
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        memcpy(ptr1, dst1, n);
                        goto LABEL_40fe25;
                    }
                }
            }
            else
            {
LABEL_410628:
                ptr1 = dst1;
LABEL_40fe25:
                *((char *)ptr1 + n) = 0;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v6 = iter;
                dst = ptr1;
                err = __errno_location();
                v4 = *(err);
                v54 = v53;
            }
            while (1)
            {
                v55 = 0x7fffffff;
                v13 = 0xffffffff;
                v56 = v32 - n;
                *(err) = 0;
                if (v56 <= 0x7fffffff)
                    v55 = v56;
                v57 = v55;
                if (v10 > 0x11)
                    goto LABEL_4109d0;
                goto *((void *)((long long)(long long)(g_414090[v10] + &g_414090[0])));

                switch (/* incomplete */)
                {
                case 0x410350:
                    v60 = (unsigned long)(unsigned long long)v17[16 + 32 * node[80]];
                    goto LABEL_410183;
                case 0x410370:
                    v60 = (unsigned long)(unsigned long long)(unsigned int)(int)(char)v17[16 + 32 * node[80]];
                    goto LABEL_410183;
                case 0x410390:
                    v60 = (unsigned long)(unsigned long long)(unsigned short)*((short *)&v17[16 + 32 * node[80]]);
                    goto LABEL_410183;
                case 0x410170:
                    v60 = (unsigned long)(unsigned long long)(unsigned int)*((int *)&v17[16 + 32 * node[80]]);
                    goto LABEL_410183;
                case 0x4102e0:
                    v3 = v56;
                    v66 = v9;
                    if (v66 == 1)
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                    }
                    else if (v66 != 2)
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        v64 = (unsigned int)(unsigned long long)__snprintf_chk();
                        v65 = v3;
                        goto LABEL_410098;
                    }
                    else
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned long long *)(/* unsupported instruction */ - 8)) = (unsigned long long)v15;
                    }
                    v64 = (unsigned int)(unsigned long long)__snprintf_chk();
                    v65 = v3;
                    /* unsupported instruction */ = (int)(/* unsupported instruction */ + 8);
                    goto LABEL_410098;
                case 0x4103b0:
                    v60 = (unsigned long)(unsigned long long)(unsigned int)(int)*((short *)&v17[16 + 32 * node[80]]);
LABEL_410183:
                    v67 = v9;
                    v3 = v56;
                    if (v67 == 1)
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned long *)(/* unsupported instruction */ - 8)) = v60;
                        goto LABEL_41056a;
                    }
                    else if (v67 != 2)
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        goto LABEL_4101b2;
                    }
                    else
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned long *)(/* unsupported instruction */ - 8)) = v60;
                        goto LABEL_41051e;
                    }
                case 0x410298:
                    v3 = v56;
                    v61 = v54 - 1;
                    if (/* unsupported instruction */)
                    {
                        v62 = v61 - 1;
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    else
                    {
                        v62 = v61 - 1;
                        /* unsupported instruction */
                        /* unsupported instruction */
                    }
                    v63 = v9;
                    if (v63 == 1)
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 24);
                        if (/* unsupported instruction */)
                        {
                            amd64g_dirtyhelper_storeF80le(vvar_1363{r48|8b}, Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                            /* unsupported instruction */
                            v54 = v62 + 1;
                        }
                        else
                        {
                            amd64g_dirtyhelper_storeF80le(vvar_1363{r48|8b}, Reinterpret(F64->I64, nan<64>))
                            /* unsupported instruction */
                            v54 = v62 + 1;
                        }
LABEL_410525:
                        v64 = (unsigned int)(unsigned long long)__snprintf_chk();
                        v65 = v3;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ + 24);
                        goto LABEL_410098;
                    }
                    else if (v63 != 2)
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 16);
                        if (/* unsupported instruction */)
                        {
                            amd64g_dirtyhelper_storeF80le(vvar_1379{r48|8b}, Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                            /* unsupported instruction */
                            v54 = v62 + 1;
                        }
                        else
                        {
                            amd64g_dirtyhelper_storeF80le(vvar_1379{r48|8b}, Reinterpret(F64->I64, nan<64>))
                            /* unsupported instruction */
                            v54 = v62 + 1;
                        }
                    }
                    else
                    {
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                        *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 32);
                        if (/* unsupported instruction */)
                        {
                            amd64g_dirtyhelper_storeF80le((vvar_2256{r48|8b} Add 8<64>), Reinterpret(F64->I64, unsupported_<class 'pyvex.expr.GetI'>()))
                            /* unsupported instruction */
                            v54 = v62 + 1;
                        }
                        else
                        {
                            amd64g_dirtyhelper_storeF80le((vvar_2256{r48|8b} Add 8<64>), Reinterpret(F64->I64, nan<64>))
                            /* unsupported instruction */
                            v54 = v62 + 1;
                        }
                        *((unsigned long long *)(/* unsupported instruction */ - 8)) = (unsigned long long)v15;
                        v64 = (unsigned int)(unsigned long long)__snprintf_chk();
                        v65 = v3;
                        /* unsupported instruction */ = (int)(/* unsupported instruction */ + 32);
LABEL_410098:
                        v68 = v13;
                        if (v68 < 0)
                            goto LABEL_4101e6;
                        goto LABEL_4100a6;
                    }
LABEL_410072:
                    v64 = (unsigned int)(unsigned long long)__snprintf_chk();
                    v65 = v3;
                    /* unsupported instruction */ = (int)(/* unsupported instruction */ + 8);
                    goto LABEL_410098;
                case 0x410030:
                    v58 = (unsigned long)*((long long *)&v17[16 + 32 * node[80]]);
                    v59 = v9;
                    v3 = v56;
                    if (v59 != 1)
                    {
                        if (v59 != 2)
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            goto LABEL_410072;
                        }
                        else
                        {
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned long *)(/* unsupported instruction */ - 8)) = v58;
LABEL_41051e:
                            /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                            *((unsigned long long *)(/* unsupported instruction */ - 8)) = (unsigned long long)v15;
                            goto LABEL_410525;
                        }
                    }
                    /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                    *((unsigned int **)(/* unsupported instruction */ - 8)) = &v13;
                    /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                    *((unsigned long *)(/* unsupported instruction */ - 8)) = v58;
LABEL_41056a:
LABEL_4101b2:
                    v64 = (unsigned int)(unsigned long long)__snprintf_chk();
                    v68 = v13;
                    v65 = v3;
                    /* unsupported instruction */ = (int)(/* unsupported instruction */ + 8);
                    if (v68 >= 0)
                    {
LABEL_4100a6:
                        v69 = (unsigned long long)(long long)(int)v68;
                        if (v69 < v57 && *((char *)(v69 + (char *)dst + n)))
                            goto LABEL_4109d0;
                        if (v68 >= v64)
                            goto LABEL_4100c7;
                        goto LABEL_4100bf;
                    }
LABEL_4101e6:
                    if (!v6->padding_0[1])
                    {
                        if (v64 < 0)
                        {
                            v86 = dst;
                            if (!*(err))
                            {
                                if (((char)node[72] & 0xffffffef) == 99)
                                    v87 = 84;
                                else
                                    v87 = 22;
                                *(err) = (int)v87;
                            }
                            if (v86 != v12 && v86)
                                free(v86);
                            if (v5)
                                free(v5);
                            if (v20 != &v23)
                                free(v20);
                            if (v17 != &v18)
                            {
                                free(v17);
                                return 0;
                            }
                            return 0;
                        }
LABEL_4100bf:
                        v13 = v64;
                        v68 = v64;
LABEL_4100c7:
                        if ((unsigned long long)(v68 + 1) < v57)
                        {
                            v29 = (long long)(int)v68 + n;
                            dst1 = dst;
                            *(err) = (int)v4;
LABEL_40ff29:
                            ptr = (unsigned long)(long long)node[8];
                            node += 88;
                            v7 += 1;
                            if ((long long)node[88] != ptr)
                                goto LABEL_40fa35;
                            goto LABEL_40ff46;
                        }
                        if (v65 > 0x7ffffffe)
                        {
                            if (dst != v12 && dst)
                                free(dst);
                            if (v5)
                                free(v5);
                            if (v20 != &v23)
                                free(v20);
                            if (v17 != &v18)
                                free(v17);
                            *(err) = (int)75;
                            return 0;
                        }
                        v70 = (unsigned long)(unsigned long long)(v68 + 2);
                        v71 = v70 + n;
                        v72 = (unsigned long long)__CFADD__(v70, n) & 1;
                        if (v32 >= 0)
                        {
                            v73 = v32 * 2;
                            if (v72)
                                goto LABEL_4104ca;
                            if (v71 <= v73)
                            {
                                v71 = v73;
                                goto LABEL_410109;
                            }
                            else
                            {
                                goto LABEL_410109;
                            }
                        }
                        if (!v72)
                        {
                            v71 = 0xffffffffffffffff;
                            v73 = 0xffffffffffffffff;
LABEL_410109:
                            if (v32 >= v71)
                                continue;
                            if (!v32)
                                v73 = 12;
                            if (v71 <= v73)
                                v71 = v73;
                            v32 = v71;
                            if (v71 == 0xffffffffffffffff)
                                goto LABEL_4104ca;
                            v74 = (char)(char)(dst == v12);
                            if (dst && !v74)
                            {
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                ptr2 = realloc(dst, v71);
                                if (!ptr2)
                                    goto LABEL_4104ca;
                                dst = ptr2;
                            }
                            else
                            {
                                /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                v76 = malloc(v32);
                                if (!v76)
                                {
LABEL_4104ca:
                                    v88 = dst;
LABEL_40fa8a:
                                    if (v88 == v12 || !v88)
                                        goto LABEL_40fa9c;
                                    goto LABEL_410020;
                                }
                                else
                                {
                                    if (n && v74)
                                    {
                                        /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8);
                                        dst = memcpy(v76, dst, n);
                                    }
                                    else
                                    {
                                        dst = v76;
                                    }
                                }
                            }
                        }
                        else if (!(v32 == 0xffffffffffffffff))
                        {
                            goto LABEL_4104ca;
                        }
                    }
                    else
                    {
                        v6->padding_0[1] = 0;
                    }
                }
            }
        }
        else
        {
            v89 = n;
            v90 = v89 + 1;
            if (!(__CFADD__(v89, 1) & 1))
            {
                if (v32 >= v90)
                {
LABEL_4108bd:
                    *((char *)dst1 + n) = 0;
                    if (v32 > v90 && dst1 != v12)
                    {
                        v12 = dst1;
                        v94 = realloc(dst1, v90);
                        dst1 = v12;
                        if (v94)
                            dst1 = v94;
                    }
                    if (v5)
                    {
                        v12 = dst1;
                        free(v5);
                        dst1 = v12;
                    }
                    v95 = v20;
                    if (v95 != &v23)
                    {
                        v12 = dst1;
                        free(v95);
                        dst1 = v12;
                    }
                    v96 = v17;
                    if (v96 != &v18)
                    {
                        v12 = dst1;
                        free(v96);
                        dst1 = v12;
                    }
                    *((unsigned long long *)a1) = n;
                    return dst1;
                }
                if (v32)
                {
                    if (v32 < 0)
                        goto LABEL_40fa70;
                    v32 *= 2;
                    if (v32 >= v90)
                        goto LABEL_41087b;
                }
                else if (v90 <= 12)
                {
                    v32 = 12;
                    goto LABEL_41087b;
                }
                if (v90 == 0xffffffffffffffff)
                    goto LABEL_40fa70;
                v32 = v90;
LABEL_41087b:
                v91 = dst1 == v12;
                if (!dst1 || v91)
                {
                    err = dst1;
                    v93 = malloc(v32);
                    dst1 = err;
                    if (!v93)
                        goto LABEL_40fa70;
                    if (n && v91)
                    {
                        dst1 = memcpy(v93, dst1, n);
                        goto LABEL_4108bd;
                    }
                    else
                    {
                        dst1 = v93;
                        goto LABEL_4108bd;
                    }
                }
                err = dst1;
                v92 = realloc(dst1, v32);
                v80 = err;
                if (v92)
                {
                    dst1 = v92;
                    goto LABEL_4108bd;
                }
            }
            else
            {
                if (v32 == 0xffffffffffffffff)
                    goto LABEL_4108bd;
LABEL_40fa70:
                v10 = dst1;
                goto LABEL_40fa77;
            }
LABEL_40ffff:
            v12 = v80;
            v88 = v12;
            err = __errno_location();
LABEL_410020:
            free(v88);
LABEL_40fa9c:
            if (v5)
                free(v5);
        }
LABEL_40fab0:
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



// Function: rpl_vfprintf @ 0x10a00
extern unsigned long long g_0;

unsigned int rpl_vfprintf(unsigned long a0, unsigned int a1, long long a2)
{
    char *v4;  // rax
    char *ptr;  // rbp
    unsigned int v0;  // [bp-0x81c]
    unsigned long v1;  // [bp-0x810]
    char v2;  // [bp-0x808]

    v1 = 2000;
    v4 = sub_40f900(&v2, &v1, a1, a2);
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
    sub_410b90(a0);
    return 0xffffffff;
}



// Function: opendir_safer @ 0x10af0
typedef struct DIR {
} DIR;

extern unsigned long long g_0;

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
    v3 = sub_40e0f0(v2, 1030);
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



// Function: fseterr @ 0x10b90
void fseterr(unsigned int *a0)
{
    *(a0) = *(a0) | 32;
    return;
}



// Function: current_timespec @ 0x10bb0
extern unsigned long long g_0;

long long current_timespec(void)
{
    char v0;  // [bp-0x28]

    clock_gettime(0, &v0);
    return v0;
}



// Function: dup_safer @ 0x10c00

unsigned int dup_safer(int a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5)
{
    return sub_40e0f0(a0, 0);
}



// Function: printf_fetchargs @ 0x10c20
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

extern unsigned long long g_0;

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



// Function: printf_parse @ 0x10e50
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

extern unsigned long long g_0;

unsigned long long printf_parse(char *a0, struct_0 *idx, struct_2 *index)
{
    struct_0 *v9;  // r9
    struct_2 *ptr;  // r10
    char *v19;  // rdx
    char *v20;  // rdx
    char *iter;  // rax
    unsigned long long v22;  // rdi
    char *v23;  // rcx
    unsigned long v24;  // rdx
    unsigned long long v25;  // rsi
    char v26;  // dl
    struct_2 *ptr1;  // rdi
    char *v28;  // rdx
    char *v11;  // rax
    unsigned int v29;  // eax
    char *v30;  // rdx
    char *v31;  // rdx
    char *v32;  // rdx
    unsigned long long v33;  // rax
    unsigned long long v34;  // rdi
    unsigned long long v35;  // rax
    char v36;  // cl
    char *v37;  // rax
    char *v38;  // rax
    struct_0 *v12;  // rcx
    char *v39;  // rax
    unsigned long long v40;  // rdi
    unsigned long long v41;  // rbp
    char *node;  // rbx
    unsigned long long v43;  // rsi
    char *v44;  // rdx
    unsigned long v45;  // rax
    unsigned long long v46;  // rcx
    char v47;  // al
    struct_2 *v48;  // rax
    unsigned long long v13;  // r8
    unsigned long long v49;  // r8
    unsigned long long v50;  // rsi
    struct_2 *v51;  // r10
    unsigned long long v52;  // rdx
    unsigned int *v53;  // rax
    unsigned int *v54;  // rax
    unsigned int *v56;  // rbp
    unsigned long long v57;  // rdi
    unsigned long long v58;  // rax
    unsigned long long v14;  // r11
    char *v59;  // rcx
    char v60;  // sil
    char *v61;  // rax
    char *v62;  // rax
    char *v63;  // rax
    unsigned long long v64;  // rbx
    unsigned long long v65;  // rdi
    char *iter1;  // rbx
    unsigned long long v67;  // rdi
    char *v68;  // rcx
    char *idx1;  // rbx
    unsigned long v69;  // rax
    unsigned long long v70;  // rdx
    char v71;  // al
    unsigned long long v72;  // r8
    unsigned long long v73;  // rsi
    unsigned long long v74;  // r11
    unsigned long long v75;  // r8
    unsigned long long v76;  // rdx
    unsigned int *v77;  // rax
    unsigned int *v78;  // rax
    struct_0 *idx2;  // r12
    unsigned int *v80;  // rax
    char *v81;  // rdx
    char *v82;  // rdx
    char *v83;  // rdx
    unsigned long long v84;  // rax
    unsigned long long v85;  // rdi
    char *iter2;  // rbx
    char v17;  // bpl
    char *v18;  // rdx
    struct_0 *v0;  // [bp-0x78], Other Possible Types: unsigned long long
    unsigned long long v1;  // [bp-0x70]
    unsigned long long v2;  // [bp-0x68]
    char *v3;  // [bp-0x60], Other Possible Types: struct_0 *, struct_2 *
    char *v4;  // [bp-0x58], Other Possible Types: struct_2 *, struct_0 *
    struct_0 *v5;  // [bp-0x50], Other Possible Types: struct_2 *, unsigned long long
    unsigned long long v6;  // [bp-0x48]
    unsigned long long v7;  // [bp-0x40]

    v9 = &idx->field_20;
    ptr = index + 1;
    v11 = a0;
    v12 = v9;
    v13 = 7;
    v14 = 7;
    idx->field_0 = 0;
    idx->field_8 = v9;
    index->field_0 = 0;
    index->field_8 = ptr;
    v1 = 0;
    v0 = 0;
    v2 = 0;
    while (1)
    {
        if (!*(v11))
        {
            v12->field_0 = v11;
            idx->field_10 = v0;
            idx->field_18 = v1;
            return 0;
        }
        idx1 = v11 + 1;
        if (*(v11) == 37)
            break;
        v11 = idx1;
    }
    idx2 = v12;
    idx2->field_0 = v11;
    *((unsigned int *)&idx2->field_10) = 0;
    idx2->field_18 = 0;
    idx2->field_20 = 0;
    *((unsigned long long *)&idx2->padding_28[0]) = 0xffffffffffffffff;
    *((unsigned long long *)&idx2->field_30) = 0;
    idx2->field_38 = 0;
    idx2->field_40 = 0xffffffffffffffff;
    idx2->field_50 = 0xffffffffffffffff;
    v17 = v11[1];
    if (v17 - 48 <= 9)
    {
        v18 = idx1;
        do
        {
            v20 = v18 + 1;
            v18 = v20;
        } while (v19[1] - 48 <= 9);
        if (v19[1] == 36)
        {
            iter = v11 + 2;
            v22 = 0;
            while (1)
            {
                v23 = iter - 1;
                v24 = v17 - 48;
                v25 = (v22 <= 1844674407370955161 ? v22 * 10 : 0xffffffffffffffff);
                while (1)
                {
                    v22 = v25 + v24;
                    v26 = *(iter) - 48;
                    if (!(__CFADD__(v25, v24) & 1))
                        break;
                    if (v26 <= 9)
                    {
                        v23 = iter;
                        v24 = v26;
                        iter += 1;
                        v25 = 0xffffffffffffffff;
                    }
                    else
                    {
                        ptr1 = index->field_8;
                        goto LABEL_411464;
                    }
                }
                if (v26 > 9)
                    break;
                iter += 1;
            }
            if (v22 - 1 > 0xfffffffffffffffd)
            {
                ptr1 = index->field_8;
                goto LABEL_411464;
            }
            else
            {
                v17 = v23[2];
                idx1 = v23 + 2;
            }
        }
    }
    v28 = idx1 + 1;
    if (v17 == 39)
        goto LABEL_411021;
    while (1)
    {
        switch (v17)
        {
        case 32:
            v29 = (int)idx2->field_10 | 8;
            goto LABEL_411008;
        case 35:
            v29 = (int)idx2->field_10 | 16;
            goto LABEL_411008;
        case 43:
            v29 = (int)idx2->field_10 | 4;
            goto LABEL_411008;
        case 45:
            v29 = (int)idx2->field_10 | 2;
            goto LABEL_411008;
        case 48:
            v29 = (int)idx2->field_10 | 32;
            goto LABEL_411008;
        case 73:
            v29 = (int)idx2->field_10 | 64;
            goto LABEL_411008;
        default:
            if (v17 != 42)
            {
                if (v17 - 48 > 9)
                    goto LABEL_410fbd;
                idx2->field_18 = idx1;
                if (*(idx1) - 48 <= 9)
                {
                    v30 = idx1;
                    do
                    {
                        v32 = v30 + 1;
                        v30 = v32;
                    } while (v31[1] - 48 <= 9);
                    v33 = v32 - idx1;
                    idx1 = v32;
                    if (v33 <= v0)
                        v33 = v0;
                    v0 = v33;
                }
                idx2->field_20 = idx1;
                goto LABEL_4117ec;
            }
            v34 = v0;
            v35 = 1;
            idx2->field_18 = idx1;
            idx2->field_20 = v28;
            v36 = idx1[1];
            if (v34)
                v35 = v34;
            v0 = v35;
            if (idx1[1] - 48 > 9)
            {
LABEL_4110d5:
                v40 = v2;
                *((unsigned long long *)&idx2->padding_28[0]) = v40;
                if (v40 != 0xffffffffffffffff)
                {
                    v41 = v2;
                    v2 = v40 + 1;
                    idx1 = v28;
                    goto LABEL_4110fa;
                }
                else
                {
                    ptr1 = index->field_8;
                    break;
                }
            }
            else
            {
                v37 = v28;
                do
                {
                    v39 = v37 + 1;
                    v37 = v39;
                } while (v38[1] - 48 <= 9);
                if (v38[1] != 36)
                    goto LABEL_4110d5;
                node = idx1 + 2;
                v43 = 0;
                while (1)
                {
                    v44 = node - 1;
                    v45 = v36 - 48;
                    v46 = (v43 <= 1844674407370955161 ? v43 * 10 : 0xffffffffffffffff);
                    while (1)
                    {
                        v43 = v46 + v45;
                        v47 = *(node) - 48;
                        if (!(__CFADD__(v46, v45) & 1))
                            break;
                        if (v47 > 9)
                        {
                            ptr1 = index->field_8;
                            break;
                        }
                        else
                        {
                            v44 = node;
                            v45 = v47;
                            node += 1;
                            v46 = 0xffffffffffffffff;
                        }
                    }
                    if (v47 > 9)
                        break;
                    node += 1;
                }
                v41 = v43 - 1;
                if (v41 > 0xfffffffffffffffd)
                {
                    ptr1 = index->field_8;
                    break;
                }
                else
                {
                    *((unsigned long long *)&idx2->padding_28[0]) = v41;
                    idx1 = v44 + 2;
                }
LABEL_4110fa:
                ptr1 = index->field_8;
                v48 = ptr1;
                if (7 <= v41)
                {
                    v49 = 14;
                    if (14 <= v41)
                        v49 = v41 + 1;
                    if (v49 & 0xf800000000000000)
                        goto LABEL_4116ab;
                    v6 = v49;
                    v5 = 7;
                    v50 = v49 * 32;
                    v4 = ptr;
                    v3 = v9;
                    if (ptr != ptr1)
                    {
                        v9 = v3;
                        ptr = v4;
                        v14 = v5;
                        v13 = v6;
                        ptr1 = realloc(ptr1, v50);
                    }
                    else
                    {
                        v13 = v6;
                        v14 = v5;
                        ptr = v4;
                        v9 = v3;
                        ptr1 = malloc(v50);
                    }
                    v48 = index->field_8;
                    if (!ptr1)
                        goto LABEL_4116ab;
                    if (ptr == v48)
                    {
                        v6 = v13;
                        v5 = v14;
                        v4 = v9;
                        v3 = ptr;
                        v13 = v6;
                        v14 = v5;
                        v9 = v4;
                        v51 = v3;
                        ptr1 = memcpy(ptr1, ptr, index->field_0 * 32);
                        ptr = v51;
                    }
                    index->field_8 = ptr1;
                }
                v52 = index->field_0;
                v53 = &ptr1[2 * v52];
                if (v52 <= v41)
                {
                    do
                    {
                        v54 = v53;
                        v52 += 1;
                        *(v54) = 0;
                        v53 = v54 + 8;
                    } while (v52 <= v41);
                    index->field_0 = v52;
                    *(v54) = 0;
                }
                v56 = &ptr1[2 * v41];
                if (!*(v56))
                {
                    *(v56) = 5;
                    v17 = *(idx1);
                    if (v17 != 46)
                        goto LABEL_410fc7;
LABEL_4111d8:
                    if (idx1[1] != 42)
                    {
                        *((char **)&idx2->field_30) = idx1;
                        v81 = idx1 + 1;
                        if (idx1[1] - 48 <= 9)
                        {
                            do
                            {
                                v83 = v81 + 1;
                                v81 = v83;
                            } while (v82[1] - 48 <= 9);
                            v84 = v83 - idx1;
                            idx1 = v83;
                            v81 = v83;
                        }
                        else
                        {
                            idx1 = v81;
                            v84 = 1;
                        }
                        v85 = v1;
                        idx2->field_38 = v81;
                        v17 = *(v81);
                        if (v84 <= v85)
                            v84 = v85;
                        v1 = v84;
                        goto LABEL_410fc7;
                    }
                    v57 = v1;
                    v58 = 2;
                    v59 = idx1 + 2;
                    *((char **)&idx2->field_30) = idx1;
                    idx2->field_38 = v59;
                    v60 = idx1[2];
                    if (2 <= v57)
                        v58 = v57;
                    v1 = v58;
                    if (idx1[2] - 48 <= 9)
                    {
                        v61 = v59;
                        do
                        {
                            v63 = v61 + 1;
                            v61 = v63;
                        } while (v62[1] - 48 <= 9);
                        if (v62[1] == 36)
                        {
                            iter1 = idx1 + 3;
                            v67 = 0;
                            while (1)
                            {
                                v68 = iter1 - 1;
                                v69 = v60 - 48;
                                v70 = (v67 <= 1844674407370955161 ? v67 * 10 : 0xffffffffffffffff);
                                while (1)
                                {
                                    v67 = v70 + v69;
                                    v71 = *(iter1) - 48;
                                    if (!(__CFADD__(v70, v69) & 1))
                                        break;
                                    if (v71 > 9)
                                    {
                                        ptr1 = index->field_8;
                                        break;
                                    }
                                    else
                                    {
                                        v68 = iter1;
                                        v69 = v71;
                                        iter1 += 1;
                                        v70 = 0xffffffffffffffff;
                                    }
                                }
                                if (v71 > 9)
                                    break;
                                iter1 += 1;
                            }
                            v64 = v67 - 1;
                            if (v64 > 0xfffffffffffffffd)
                            {
                                ptr1 = index->field_8;
                                break;
                            }
                            else
                            {
                                idx2->field_40 = v64;
                                v59 = v68 + 2;
                            }
LABEL_411224:
                            ptr1 = index->field_8;
                            v48 = ptr1;
                            if (v13 > v64)
                            {
LABEL_411234:
                                v76 = index->field_0;
                                v77 = &ptr1[2 * v76];
                                if (v76 <= v64)
                                {
                                    do
                                    {
                                        v78 = v77;
                                        v76 += 1;
                                        *(v78) = 0;
                                        v77 = v78 + 8;
                                    } while (v76 <= v64);
                                    index->field_0 = v76;
                                    *(v78) = 0;
                                }
                                v80 = &ptr1[2 * v64];
                                if (!*(v80))
                                {
                                    *(v80) = 5;
                                    idx1 = v59;
                                    v17 = *(v59);
                                    goto LABEL_410fc7;
                                }
                                if (*(v80) != 5)
                                    break;
                                v17 = *(v59);
                                idx1 = v59;
LABEL_410fc7:
                                iter2 = idx1 + 1;
                                if (v17 == 104)
                                    goto LABEL_411094;
                                while (1)
                                {
                                    switch (v17)
                                    {
                                    case 76:
                                        break;
                                    case 90: case 106: case 108: case 116: case 122:
                                        break;
                                    default:
                                        switch (v17)
                                        {
                                        case 38:
                                            break;
                                        default:
                                            ptr1 = index->field_8;
                                            break;
                                        }
                                    }
                                    while (1)
                                    {
                                        iter2 += 1;
                                        if (*(iter2) != 104)
                                            break;
LABEL_411094:
                                    }
                                }
                                break;
                            }
                            else
                            {
                                v72 = v13 * 2;
                                if (v72 <= v64)
                                    v72 = v64 + 1;
                                if (!(v72 & 0xf800000000000000))
                                {
                                    v7 = v72;
                                    v6 = v14;
                                    v73 = v72 * 32;
                                    v5 = ptr;
                                    v4 = v9;
                                    v3 = v59;
                                    if (ptr != ptr1)
                                    {
                                        v59 = v3;
                                        v9 = v4;
                                        ptr = v5;
                                        v74 = v6;
                                        ptr1 = realloc(ptr1, v73);
                                        v75 = v7;
                                    }
                                    else
                                    {
                                        v75 = v7;
                                        v74 = v6;
                                        ptr = v5;
                                        v9 = v4;
                                        ptr1 = malloc(v73);
                                        v59 = v3;
                                    }
                                    v48 = index->field_8;
                                    if (ptr1)
                                    {
                                        if (ptr == v48)
                                        {
                                            v7 = v75;
                                            v6 = v74;
                                            v5 = v9;
                                            v4 = v59;
                                            v3 = ptr;
                                            v9 = v5;
                                            v59 = v4;
                                            ptr1 = memcpy(ptr1, ptr, index->field_0 * 32);
                                            ptr = v3;
                                        }
                                        index->field_8 = ptr1;
                                        goto LABEL_411234;
                                    }
                                }
                            }
LABEL_4116ab:
                            if (ptr != v48)
                            {
                                v0 = v9;
                                free(v48);
                                v9 = v0;
                            }
                            if (v9 != idx->field_8)
                                free(idx->field_8);
                            *(__errno_location()) = 12;
                            return 0xffffffff;
                        }
                    }
                    v64 = idx2->field_40;
                    if (v64 != 0xffffffffffffffff)
                        goto LABEL_411224;
                    v65 = v2;
                    idx2->field_40 = v65;
                    if (v65 == 0xffffffffffffffff)
                    {
                        ptr1 = index->field_8;
                        break;
                    }
                    else
                    {
                        v64 = v2;
                        v2 = v65 + 1;
                        goto LABEL_411224;
                    }
                }
                else
                {
                    if (*(v56) != 5)
                        break;
LABEL_4117ec:
                    v17 = *(idx1);
LABEL_410fbd:
                    if (v17 != 46)
                        goto LABEL_410fc7;
                    goto LABEL_4111d8;
                }
            }
        }
        while (1)
        {
LABEL_411008:
            *((unsigned int *)&idx2->field_10) = v29;
            v28 += 1;
            if (*(v28) != 39)
                break;
LABEL_411021:
            v29 = (int)idx2->field_10 | 1;
        }
    }
LABEL_411464:
    if (ptr != ptr1)
    {
        v0 = v9;
        free(ptr1);
        v9 = v0;
    }
    if (v9 != idx->field_8)
        free(idx->field_8);
    *(__errno_location()) = 22;
    return 0xffffffff;
}



// Function: main @ 0x4050
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern unsigned int g_41a460[4];
extern unsigned int g_41a470[4];
extern char g_41bdf7;
extern option g_4224a0;
extern long long g_422880;
extern long long g_423020;
extern long long g_423028;
extern long long stdout;
extern int optind;
extern long long optarg;
extern char g_423158;
extern char g_423159;
extern char g_42315a;

int main(unsigned int a0, long long *a1)
{
    long long v29;  // rdi
    long long v30;  // rsi
    unsigned int v39;  // eax
    char *v40;  // rax
    char v42;  // 4098
    unsigned long v43;  // cc_ndep
    unsigned long long v44;  // 4107
    char *v45;  // rax
    int *err;  // rax
    char *v48;  // rsi
    long long v31;  // rdx
    long long v32;  // rcx
    long long v33;  // r8
    long long v34;  // r9
    int v35;  // eax
    long long v36;  // rax
    int *err1;  // rax
    unsigned long long v0;  // [bp-0xd8]
    long long v1;  // [bp-0xc8]
    long long v2;  // [bp-0xc0]
    long long v3;  // [bp-0xb8]
    long long v4;  // [bp-0xb0]
    char v5;  // [bp-0xa3]
    char v6;  // [bp-0xa2]
    char v7;  // [bp-0xa1]
    char *v8;  // [bp-0xa0]
    char v9;  // [bp-0x98], Other Possible Types: unsigned int
    unsigned int v10;  // [bp-0x94]
    unsigned long long v11;  // [bp-0x90]
    unsigned long long v12;  // [bp-0x88]
    char v13;  // [bp-0x84]
    unsigned short v14;  // [bp-0x80]
    char v15;  // [bp-0x7c], Other Possible Types: unsigned int
    char v16;  // [bp-0x7b]
    unsigned short v17;  // [bp-0x7a]
    char v18;  // [bp-0x78]
    unsigned long long v19;  // [bp-0x70]
    int v20;  // [bp-0x68]
    char v21;  // [bp-0x65]
    char v22;  // [bp-0x5d]
    char v23;  // [bp-0x5c], Other Possible Types: unsigned short
    char v24;  // [bp-0x5a]
    unsigned int v25;  // [bp-0x54]
    unsigned long long v26;  // [bp-0x50]
    unsigned long long v27;  // [bp-0x48]

    sub_412680(*(a1));
    setlocale(6, &g_41bdf7);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_419d70(sub_40e2b0);
    g_42315a = is_selinux_enabled(v29, v30, v31, v32, v33, v34) > 0;
    sub_407670(&v9);
    v18 = 0;
    v25 = 1;
    v11 = 8589934596;
    v15 = 0;
    v10 = 1;
    v12 = 0x100000000;
    v14 = 0;
    v19 = 0;
    *((unsigned long long *)&v20) = 0x100;
    *((unsigned int *)&(&v20)[8]) = 0;
    v23 = 0;
    v6 = 0;
    v26 = 0;
    v27 = 0;
    v4 = 0;
    v1 = 0;
    v7 = 0;
    v2 = 0;
    v3 = 0;
    v5 = 0;
    v24 = getenv("POSIXLY_CORRECT");
    while (1)
    {
        v35 = getopt_long(a0, a1, "abdfHilLnprst:uvxPRS:TZ", &g_4224a0.name, NULL);
        if (v35 != -0x1)
        {
            if (v35 > 0x88)
                goto LABEL_404799;
            if (v35 > 71)
            {
                switch (v35)
                {
                case 72:
                    v10 = 3;
                    continue;
                case 76:
                    v10 = 4;
                    continue;
                case 80:
                    v10 = 2;
                    continue;
                case 82: case 114:
                    *((char *)&(&v20)[8]) = 1;
                    continue;
                case 83:
                    v5 = 1;
                    v3 = optarg;
                    continue;
                case 84:
                    v6 = 1;
                    break;
                case 90:
                    if (g_42315a)
                    {
                        if (optarg)
                        {
                            v4 = optarg;
                            break;
                        }
                        else
                        {
                            v19 = selabel_open(0, 0, 0);
                            if (!v19)
                            {
                                v8 = dcgettext(NULL, "warning: ignoring --context", 5);
                                err1 = __errno_location();
                                error(0, *(err1), v8);
                                break;
                            }
                        }
                    }
                    else
                    {
                        if (optarg)
                        {
                            error(0, 0, dcgettext(NULL, "warning: ignoring --context; it requires an SELinux-enabled kernel", 5));
                            break;
                        }
                    }
                case 97:
                    *((char *)&v20) = 1;
                    v10 = 2;
                    v16 = 1;
                    v17 = 0x101;
                    *((char *)&(&v20)[2]) = 1;
                    if (g_42315a)
                        *((char *)&(&v20)[3]) = 1;
                    *((char *)&(&v20)[5]) = 1;
                    *((unsigned short *)&(&v20)[7]) = 0x101;
                    continue;
                case 98:
                    v36 = optarg;
                    v5 = 1;
                    if (!optarg)
                        v36 = v2;
                    v2 = v36;
                    continue;
                case 100:
                    *((char *)&v20) = 1;
                    v10 = 2;
                    continue;
                case 102:
                    *((char *)&v12 + 6) = 1;
                    continue;
                case 105:
                    *((unsigned int *)&v11) = 3;
                    continue;
                case 108:
                    *((char *)&v12 + 7) = 1;
                    continue;
                case 110:
                    *((unsigned int *)&v11) = 2;
                    continue;
                case 112:
                    v16 = 1;
                    v17 = 0x101;
                    *((char *)&(&v20)[2]) = 1;
                    continue;
                case 115:
                    *((char *)&(&v20)[10]) = 1;
                    continue;
                case 116:
                    if (!v1)
                    {
                        v1 = optarg;
                        break;
                    }
                    else
                    {
                        dcgettext(NULL, "multiple target directories specified", 5);
                    }
                case 117:
                    *((char *)&(&v20)[11]) = 1;
                    continue;
                case 118:
                    v23 = 1;
                    continue;
                case 120:
                    v15 = 1;
                    continue;
                case 128:
                    *((char *)&(&v20)[1]) = 0;
                    continue;
                case 129:
                    v7 = 1;
                    continue;
                case 130:
                    sub_4049a0(optarg, &v9, 0);
                    continue;
                case 131:
                    g_423159 = 1;
                    continue;
                case 132:
                    if (optarg)
                    {
                        sub_4049a0(optarg, &v9, 1);
                        *((char *)&(&v20)[2]) = 1;
                        continue;
                    }
                    break;
                case 133:
                    if (optarg)
                    {
                        v0 = 1;
                        v25 = g_41a460[sub_40cfc0("--reflink")];
                        break;
                    }
                    else
                    {
                        v25 = 2;
                        break;
                    }
                case 134:
                    *((unsigned int *)((char *)&v11 + 4)) = g_41a470[sub_40cfc0("--sparse", optarg, &g_422880, &g_41a470[0], 4, g_423028, 1)];
                    continue;
                case 135:
                    g_423158 = 1;
                    continue;
                case 136:
                    *((char *)&v12 + 5) = 1;
                    continue;
                default:
LABEL_404799:
                    sub_4054f0(1); /* do not return */
                }
            }
            else
            {
                if (v35 == -0x83)
                {
                    sub_416a20(stdout, "cp", "GNU coreutils", g_423020, "Torbjorn Granlund", "David MacKenzie", "Jim Meyering", 0);
                    exit(0); /* do not return */
                }
                else if (v35 == -0x82)
                {
                    sub_4054f0(0); /* do not return */
                }
                goto LABEL_404799;
            }
        }
        else
        {
            if (*((char *)((void*)&v12 + 7)) && *((char *)(&v20 + 10)))
            {
                v48 = "cannot make both hard and symbolic links";
                goto LABEL_404784;
            }
            if ((unsigned int)v11 == 2)
            {
                v22 = 0;
                if (v5)
                {
                    v48 = "options --backup and --no-clobber are mutually exclusive";
LABEL_404784:
                    error(0, 0, dcgettext(NULL, v48, 5));
                    goto LABEL_404799;
                }
            }
            else if (v5)
            {
                if (v25 != 2 || *((unsigned int *)((void*)&v11 + 4)) == 2)
                {
                    v39 = sub_40d8d0(dcgettext(NULL, "backup type", 5), v2);
                }
                else
                {
LABEL_404778:
                    v48 = "--reflink can be used only with --sparse=auto";
                    goto LABEL_404784;
                }
                goto LABEL_404586;
            }
            v39 = 0;
            if (v25 == 2 && *((unsigned int *)((void*)&v11 + 4)) != 2)
                goto LABEL_404778;
LABEL_404586:
            v9 = v39;
            sub_40d0f0(v3);
            if (v10 != 1)
            {
LABEL_40459f:
                if (!*((char *)(&v20 + 8)))
                    break;
                goto LABEL_404715;
            }
            else if (*((char *)(&v20 + 8)) && !*((char *)((void*)&v12 + 7)))
            {
                v10 = 2;
LABEL_404715:
                v13 = v7;
                break;
            }
            else
            {
                v10 = 4;
                goto LABEL_40459f;
            }
        }
    }
    v40 = *((char *)(&v20 + 4));
    if (v4 || v19)
    {
        if (!*((char *)&v40))
        {
            v21 = 0;
        }
        else
        {
            if (*((char *)(&v20 + 3)))
                error(1, 0, dcgettext(NULL, "cannot set target context and preserve it", 5));
            __unsupported_jumpkind_Ijk_NoDecode()
            v42 = *(v40);
            *(v40) = *(v40) + *((char *)&v40);
            v44 = _ccall(4, 1, (unsigned long long)v42, (unsigned long long)*((char *)(void*)&v40), v43);
            if (v44 & 1)
                error(1, 0, dcgettext(NULL, "cannot preserve security context without an SELinux-enabled kernel", 5));
        }
        if (v4 && setfscreatecon(v4) < 0)
        {
            sub_414b30(v4);
            v45 = dcgettext(NULL, "failed to set default file creation context to %s", 5);
            err = __errno_location();
            error(1, *(err), v45);
        }
    }
    else if (*((char *)&v40) && !g_42315a)
    {
        error(1, 0, dcgettext(NULL, "cannot preserve security context without an SELinux-enabled kernel", 5));
    }
    sub_40bdb0();
    return (char)sub_405a50(a0 - optind, &a1[optind], v1, v6, &v9) ^ 1;
}



// Function: decode_preserve_arg @ 0x49a0
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned int g_41a320[4];
extern struct_0 g_41a440;
extern long long g_422460;
extern long long g_423028;

long long decode_preserve_arg(unsigned long a0, unsigned long a1, char a2)
{
    unsigned long long v8;  // r15
    unsigned long long v9;  // r14
    unsigned long long v10;  // r13
    unsigned long long v11;  // r12
    unsigned long long v12;  // rbx
    long long v13;  // rax
    char *ptr;  // rax
    long long v0;  // [bp-0x48]
    unsigned long v1;  // [bp-0x40]
    unsigned long long v2;  // [bp-0x30]
    unsigned long long v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    unsigned long long v5;  // [bp-0x10]
    unsigned long long v6;  // [bp-0x8]

    v6 = v8;
    v5 = v9;
    v4 = v10;
    v3 = v11;
    v2 = v12;
    v1 = sub_417130();
    v13 = "--no-preserve";
    if (a2)
        v13 = "--preserve";
    v0 = v13;
    ptr = strchr(v1, 44);
    if (ptr)
        *(ptr) = 0;
    sub_40cfc0(v0, v1, &g_422460, &g_41a440.field_0, 4, g_423028, 1);
    if ((&g_41a440.field_0)[sub_40cfc0(v0, v1, &g_422460, &g_41a440.field_0, 4, g_423028, 1)] > 6)
        abort(); /* do not return */
    goto *((void *)((long long)(g_41a320[(&g_41a440.field_0)[sub_40cfc0(v0, v1, &g_422460, &g_41a440.field_0, 4, g_423028, 1)]] + (char *)&g_41a320[0])));
}



// Function: make_dir_parents_private @ 0x4b10
typedef struct struct_0 {
    uint128_t field_0;
    unsigned int field_10[4];
    char field_18;
    char padding_19[7];
    uint128_t field_20;
    uint128_t field_30;
    uint128_t field_40;
    uint128_t field_50;
    uint128_t field_60;
    uint128_t field_70;
    uint128_t field_80;
    char field_90;
    char padding_91[7];
    unsigned long long field_98;
    struct struct_0 *field_a0;
} struct_0;

typedef struct struct_1 {
    uint128_t field_0;
    unsigned int field_10[4];
    unsigned int field_18;
    char padding_1c[4];
    uint128_t field_20;
    uint128_t field_30;
    uint128_t field_40;
    uint128_t field_50;
    uint128_t field_60;
    uint128_t field_70;
    uint128_t field_80;
    char field_90;
    char padding_91[7];
    unsigned long long field_98;
    struct struct_1 *field_a0;
} struct_1;

typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_3 {
    char padding_0[28];
    char field_1c[4];
    char field_1d;
    char field_1e;
    char padding_1f[1];
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char padding_30[3];
    char field_33;
    char field_34;
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

unsigned int make_dir_parents_private(void* ptr, unsigned long a1, unsigned int a2, char *a3, struct_0 **a4, char *a5, struct_3 *a6)
{
    unsigned long n;  // r14
    unsigned long len;  // rax
    struct_2 *v24;  // rbx
    struct_2 *v25;  // rdi
    struct_2 *ptr1;  // rbx
    unsigned int v27;  // eax
    unsigned int v28;  // r8d
    struct_1 *idx;  // r9
    struct_0 *v30;  // rax
    struct_1 *v31;  // r9
    unsigned int v32;  // r13d
    struct_1 *v33;  // r9
    unsigned long v16;  // rax
    struct_1 *index;  // r9
    unsigned int v35;  // eax
    unsigned long v36;  // 4115
    unsigned int *v38;  // rdi
    char *v39;  // rsi
    char *v40;  // rax
    int *err;  // rax
    char *i;  // rcx
    unsigned long long v18;  // rdx
    unsigned long v19;  // rax
    unsigned long long j;  // rdx
    unsigned long long v21;  // rcx
    unsigned long long v22;  // r13
    unsigned long long v23;  // rsi
    char v0;  // [bp-0x21a8]
    char v1;  // [bp-0x11a8]
    int <0x404b10[is_1]|Stack bp-0x1a8, 1 B>;  // [bp-0x1a8]
    stat *v2;  // [bp-0x1a8]
    struct_1 *v3;  // [bp-0x198]
    char *v4;  // [bp-0x188]
    struct_2 *v5;  // [bp-0x178]
    unsigned int dst;  // [bp-0x170], Other Possible Types: unsigned long
    char v7;  // [bp-0x168]
    unsigned int v8;  // [bp-0x150]
    int v9;  // [bp-0xd8]
    int v10;  // [bp-0x88]
    int v11;  // [bp-0x78]
    int v12;  // [bp-0x68]
    int v13;  // [bp-0x58]

    n = sub_40e4a0();
    *(a4) = NULL;
    if (n <= a1)
        return 1;
    len = strlen(ptr);
    v16 = len + 24;
    i = (char *)&<0x404b10[is_1]|Stack bp-0x1a8, 1 B> - (v16 & 0xfffffffffffff000);
    if (&<0x404b10[is_1]|Stack bp-0x1a8, 1 B> != i)
    {
        do
        { } while (&v0 != i);
    }
    v18 = (unsigned int)v16 & 0xfffffff0 & 0xfff;
    /* unsupported instruction */ = (int)&(&v1)[-1 * v18];
    if (v18)
        *((long long *)(/* unsupported instruction */ + v18 - 8)) = *((long long *)(/* unsupported instruction */ + v18 - 8));
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    dst = memcpy(/* unsupported instruction */ + 15 & 0xfffffffffffffff0, ptr, len + 1);
    v19 = n + 24;
    j = /* unsupported instruction */ - (v19 & 0xfffffffffffff000);
    if (/* unsupported instruction */ != j)
    {
        do
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
            *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8));
        } while (/* unsupported instruction */ != j);
    }
    v21 = (unsigned int)v19 & 0xfffffff0 & 0xfff;
    /* unsupported instruction */ = /* unsupported instruction */ - v21;
    if (v21)
        *((long long *)(/* unsupported instruction */ + v21 - 8)) = *((long long *)(/* unsupported instruction */ + v21 - 8));
    v22 = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
    memcpy(v22, dst, n);
    v23 = v22 + a1;
    *((char *)(v22 + n)) = 0;
    if (*((char *)v23) == 47)
    {
        do
        {
            v23 += 1;
        } while (*((char *)v23) == 47);
    }
    v4 = &v7;
    if (fstatat(a2))
    {
        v24 = a1 + dst;
        v5 = v24;
        if (v24->field_0 == 47)
        {
            do
            {
                v24 = &v24->field_1;
            } while (v24->field_0 == 47);
        }
        else
        {
            v24 = v5;
        }
        v25 = v24;
        v2 = &v9;
        while (1)
        {
            ptr1 = strchr(v25, 47);
            if (!ptr1)
                break;
            ptr1->field_0 = 0;
            v27 = fstatat(a2, v24, v4, 0);
            if (!v27 && !(a6->field_1c & 0xffffff00))
            {
LABEL_404d37:
                if (!(char)sub_4072a0(v5, dst, 0, 0, a6))
                    return 0;
                if (((unsigned short)v8 & 0xf000) == 0x4000)
                {
                    *(a5) = 0;
                    goto LABEL_404d79;
                }
                else
                {
                    goto LABEL_40512c;
                }
            }
            else
            {
                if (stat(v5, v2))
                {
                    v28 = *(__errno_location());
                    if (v28)
                        goto LABEL_40503c;
                }
                else
                {
                    if (((unsigned short)*((unsigned int *)(&v9 + 24)) & 0xf000) != 0x4000)
                    {
                        v28 = 20;
LABEL_40503c:
                        dst = v28;
                        sub_414440(4, v5);
                        error(0, dst, dcgettext(NULL, "failed to get attributes of %s", 5));
                        return 0;
                    }
                }
                idx = sub_416c50(168);
                idx->field_0 = *((uint128_t *)&v9);
                *((uint128_t *)&idx->field_10[0]) = *((uint128_t *)(&v9 + 16));
                *((uint128_t *)&idx->field_18) = *((uint128_t *)(&v9 + 32));
                idx->field_20 = *((uint128_t *)(&v9 + 48));
                idx->field_30 = *((uint128_t *)(&v9 + 64));
                idx->field_40 = (uint128_t)v10;
                idx->field_50 = (uint128_t)v11;
                idx->field_60 = (uint128_t)v12;
                idx->field_70 = (uint128_t)v13;
                *((struct_2 **)((char *)&idx->field_80 + 8)) = &(&ptr1->field_0)[-1 * dst];
                v30 = *(a4);
                *((char *)&idx->field_80) = 0;
                *((struct_0 **)&idx->field_90) = v30;
                *(a4) = idx;
                if (!v27)
                    goto LABEL_404d37;
                v3 = idx;
                if (!(char)sub_4072a0(v5, dst, v3->field_10[2], 1, a6))
                    return 0;
                v31 = v3;
                *(a5) = 1;
                v32 = (!a6->field_1c[1] ? (!a6->field_1c[2] ? 0 : v31->field_10[2] & 18) : v31->field_10[2] & 63);
                v3 = v31;
                v33 = v3;
                if (mkdirat())
                {
                    sub_414440(4, dst);
                    v39 = "cannot make directory %s";
                    goto LABEL_405190;
                }
                if (a3)
                {
                    v3 = v33;
                    __printf_chk(1, a3);
                    v33 = v3;
                }
                v3 = v33;
                index = v3;
                if (fstatat(a2, v24, v4, 0x100))
                {
                    sub_414440(4, dst);
                    v39 = "failed to get attributes of %s";
                    goto LABEL_405190;
                }
                v35 = v8;
                if (!a6->field_1c[2] && (v32 & ~(v35) && !(v3 = index, index = v3, v32 &= ~((unsigned int)(unsigned long long)sub_40bbd0()), v35 = v8, !(v32 & ~(v35))) || ((unsigned short)v35 & 448) != 448))
                {
                    *((char *)&index->field_80) = 1;
                    index->field_10[2] = v32 | v35;
                }
                if ((v35 | 448) != v35 && fchmodat(a2, v24))
                {
                    sub_414440(4, dst);
                    v39 = "setting permissions for %s";
LABEL_405190:
                    v40 = dcgettext(NULL, v39, 5);
                    err = __errno_location();
                    error(0, *(err), v40);
                    return 0;
                }
                if (*(a5))
                    goto LABEL_404db0;
LABEL_404d79:
                if (*((long long *)&a6->padding_21[4]))
                {
                    if (!(char)sub_407510(dst, 0, a6) && *((char *)&a6->field_28 + 4))
                        return 0;
                }
                else
                {
                    if (*((char *)&a6->field_28 + 3))
                    {
                        __unsupported_jumpkind_Ijk_NoDecode()
                        *(v38) = *(v38) - 1;
                    }
                }
LABEL_404db0:
                v36 = ptr1->field_1;
                ptr1->field_0 = 47;
                v25 = &ptr1->field_1;
                if ((char)v36 == 47)
                {
                    do
                    {
                        v25 = &v25->field_1;
                    } while (v25->field_0 == 47);
                }
            }
        }
    }
    else if (((unsigned short)v8 & 0xf000) == 0x4000)
    {
        *(a5) = 0;
    }
    else
    {
LABEL_40512c:
        sub_414440(4);
        error(0, 0, dcgettext(NULL, "%s exists but is not a directory", 5));
        return 0;
    }
    return 1;
}



// Function: re_protect @ 0x5210
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
    char padding_24[36];
    unsigned long long field_48;
    unsigned long long field_50;
    unsigned long long field_58;
    unsigned long long field_60;
    char padding_68[40];
    char field_90;
    char padding_91[7];
    unsigned long long field_98;
    struct struct_0 *field_a0;
} struct_0;

typedef struct struct_1 {
    char padding_0[30];
    char field_1e;
    char field_1f;
} struct_1;

unsigned int re_protect(void* ptr, unsigned int a1, unsigned long long a2, struct_0 *a3, struct_1 *index)
{
    struct_0 *iter;  // rbx
    unsigned long len;  // rax
    char *v19;  // rax
    int *err;  // rax
    char *v21;  // rsi
    char *v22;  // rax
    int *err1;  // rax
    unsigned long v11;  // rax
    char *i;  // rcx
    unsigned long long v13;  // rdx
    unsigned long dst;  // r12
    long long v15;  // r13
    unsigned long v16;  // 4120
    char v17;  // r8b
    char v18;  // al
    char v0;  // [bp-0x2078]
    char v1;  // [bp-0x1078]
    int <0x405210[is_2]|Stack bp-0x78, 1 B>;  // [bp-0x78]
    char *v2;  // [bp-0x78]
    char v3;  // [bp-0x70], Other Possible Types: unsigned long long
    char v4;  // [bp-0x68], Other Possible Types: unsigned long
    unsigned long v5;  // [bp-0x60]
    unsigned long v6;  // [bp-0x58]
    unsigned long v7;  // [bp-0x50]

    iter = a3;
    v3 = a2;
    len = strlen(ptr);
    v11 = len + 24;
    i = (char *)&<0x405210[is_2]|Stack bp-0x78, 1 B> - (v11 & 0xfffffffffffff000);
    if (&<0x405210[is_2]|Stack bp-0x78, 1 B> != i)
    {
        do
        { } while (&v0 != i);
    }
    v13 = (unsigned int)v11 & 0xfffffff0 & 0xfff;
    /* unsupported instruction */ = (int)&(&v1)[-1 * v13];
    if (v13)
        *((long long *)(/* unsupported instruction */ + v13 - 8)) = *((long long *)(/* unsupported instruction */ + v13 - 8));
    dst = memcpy(/* unsupported instruction */ + 15 & 0xfffffffffffffff0, ptr, len + 1);
    v15 = dst + v3 - ptr;
    if (!iter)
        return 1;
    v2 = &v4;
    do
    {
        v16 = index->field_1f;
        *((char *)(dst + iter->field_98)) = 0;
        if ((char)v16)
        {
            v4 = iter->field_48;
            v5 = iter->field_50;
            v6 = iter->field_58;
            v7 = iter->field_60;
            if (utimensat(a1, v15, v2, 0))
            {
                sub_414440(4, dst);
                v19 = dcgettext(NULL, "failed to preserve times for %s", 5);
                err = __errno_location();
                error(0, *(err), v19);
                return 0;
            }
        }
        if (!index->padding_0[29] || !fchownat(a1, v15, iter->field_1c, iter->field_20, 0x100))
        {
            v17 = index->field_1e;
            if (v17)
                goto LABEL_405400;
            goto LABEL_4052e1;
        }
        else
        {
            v18 = sub_4076c0(index);
            if (!v18)
            {
                v3 = v18;
                sub_414440(4, dst);
                v21 = "failed to preserve ownership for %s";
LABEL_40546b:
                v22 = dcgettext(NULL, v21, 5);
                err1 = __errno_location();
                error(0, *(err1), v22);
                return v3;
            }
            fchownat(a1, v15, 0xffffffff, iter->field_20, 0x100);
            v17 = index->field_1e;
            if (!v17)
            {
LABEL_4052e1:
                if (!iter->field_90 || !(v3 = v17, (unsigned int)(unsigned long long)fchmodat((unsigned long long)a1, v15, (unsigned long long)iter->field_18, 0x100)))
                    continue;
                sub_414440(4, dst);
                v21 = "failed to preserve permissions for %s";
                goto LABEL_40546b;
            }
            else
            {
LABEL_405400:
                if (sub_40c830(v15, 0xffffffff, dst, 0xffffffff, iter->field_18))
                    return 0;
            }
        }
    } while ((*((char *)(dst + iter->field_98)) = 47, iter = (struct_0 *)iter->field_a0, iter));
    return 1;
}



// Function: usage @ 0x54f0
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
    char *v30;  // rax
    char *v31;  // rax
    char *v32;  // rax
    char *v33;  // rax
    char *v34;  // rax
    char *v35;  // rax
    unsigned long long v18;  // r13
    char *v36;  // rax
    char *v37;  // rax
    char *v38;  // rax
    char *v39;  // rax
    char *v40;  // rax
    char *v41;  // rax
    char *v42;  // rax
    char *v43;  // rax
    char *v44;  // rax
    char *v45;  // rsi
    unsigned long long v19;  // r12
    struct_0 *v46;  // rbx
    char *v47;  // r14
    char *v49;  // rax
    char *v50;  // rax
    unsigned long long v20;  // rbx
    unsigned long v21;  // fs
    char *v22;  // rax
    struct_0 *v23;  // rbx
    char *v25;  // rax
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n", 5));
        v25 = dcgettext(NULL, "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
        fputs_unlocked(v26, stdout);
        v27 = dcgettext(NULL, "  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don't copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n", 5);
        fputs_unlocked(v27, stdout);
        v28 = dcgettext(NULL, "  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n", 5);
        fputs_unlocked(v28, stdout);
        v29 = dcgettext(NULL, "  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n", 5);
        fputs_unlocked(v29, stdout);
        v30 = dcgettext(NULL, "  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n", 5);
        fputs_unlocked(v30, stdout);
        v31 = dcgettext(NULL, "  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n", 5);
        fputs_unlocked(v31, stdout);
        v32 = dcgettext(NULL, "      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n", 5);
        fputs_unlocked(v32, stdout);
        v33 = dcgettext(NULL, "  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n", 5);
        fputs_unlocked(v33, stdout);
        v34 = dcgettext(NULL, "      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n", 5);
        fputs_unlocked(v34, stdout);
        v35 = dcgettext(NULL, "  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n", 5);
        fputs_unlocked(v35, stdout);
        v36 = dcgettext(NULL, "  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n", 5);
        fputs_unlocked(v36, stdout);
        v37 = dcgettext(NULL, "  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n", 5);
        fputs_unlocked(v37, stdout);
        v38 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v38, stdout);
        v39 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v39, stdout);
        v40 = dcgettext(NULL, "\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n", 5);
        fputs_unlocked(v40, stdout);
        v41 = dcgettext(NULL, "\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n", 5);
        fputs_unlocked(v41, stdout);
        v42 = dcgettext(NULL, "\nThe backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n", 5);
        fputs_unlocked(v42, stdout);
        v43 = dcgettext(NULL, "  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n", 5);
        fputs_unlocked(v43, stdout);
        v44 = dcgettext(NULL, "\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n", 5);
        fputs_unlocked(v44, stdout);
        v0 = (int)_INSERT(v0 CONCAT 0, 0, "[");
        v45 = "[";
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
        } while (strcmp("cp", v45) && (v45 = v46->field_10, v23 = v46 + 16, v46->field_10));
        v47 = *((long long *)&v23->padding_1[7]);
        if (!*((long long *)&v23->padding_1[7]))
            v47 = "cp";
        __printf_chk(1, dcgettext(NULL, "\n%s online help: <%s>\n", 5));
        v49 = setlocale(5, NULL);
        if (v49 && strncmp(v49, "en_", 3))
        {
            v50 = dcgettext(NULL, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
            fputs_unlocked(v50, stdout);
        }
        __printf_chk(1, dcgettext(NULL, "Full documentation <%s%s>\n", 5));
        __printf_chk(1, dcgettext(NULL, "or available locally via: info '(coreutils) %s%s'\n", 5));
    }
    exit(a0); /* do not return */
}



// Function: do_copy @ 0x5a50
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[18];
    char field_16;
    char padding_17[37];
    char field_3c;
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

extern struct_0 g_423100;
extern char g_423158;
extern char g_423159;

unsigned int do_copy(int a0, void* a1, long long a2, unsigned long long a3, struct_0 *a4)
{
    char *v15;  // rax
    int *err;  // rax
    unsigned long long l;  // rcx
    unsigned long long v26;  // rdx
    unsigned long dst;  // r12
    unsigned long v28;  // r12
    unsigned long len;  // rax
    unsigned long v30;  // rax
    unsigned long long m;  // rcx
    unsigned long long v32;  // rdx
    unsigned long dst1;  // rax
    char *v34;  // rcx
    unsigned long v17;  // rax
    struct_0 *v35;  // rax
    unsigned long v36;  // 4148
    unsigned long v37;  // rdi
    char v38;  // al
    unsigned long v39;  // rdx
    char v40;  // al
    struct_0 *ptr;  // r15
    void* v43;  // rax
    void* v44;  // rcx
    unsigned long long idx;  // rbx
    int v45;  // edi
    int v46;  // esi
    unsigned long v47;  // r14
    unsigned long v48;  // r12
    unsigned int v49;  // eax
    struct_0 *j;  // r15
    unsigned long v51;  // rax
    struct_0 *iter;  // rdi
    unsigned long long v53;  // rcx
    char *v54;  // rsi
    unsigned int v19;  // r14d
    unsigned int *v55;  // rax
    char *v58;  // rsi
    unsigned long v20;  // r15
    unsigned long v21;  // rdi
    void* ptr1;  // rax
    unsigned long len1;  // rax
    unsigned long v24;  // rax
    int v0;  // [bp-0x1148]
    int <0x405a50[is_7]|Stack bp-0x1148, 1 B>;  // [bp-0x1148]
    char *v1;  // [bp-0x128]
    char *v2;  // [bp-0x120]
    char *v3;  // [bp-0x118]
    long long v4;  // [bp-0x110]
    struct_0 *v5;  // [bp-0x108]
    unsigned int v6;  // [bp-0x100]
    int i;  // [bp-0xfc]
    char k;  // [bp-0xea]
    char v9;  // [bp-0xe9]
    struct_0 *v10;  // [bp-0xe8]
    char v11;  // [bp-0xe0], Other Possible Types: unsigned long
    stat v12;  // [bp-0xd8]
    unsigned long v13;  // [bp+0x0]
    long long v14;  // [bp+0x8]

    i = a0;
    v4 = a2;
    v5 = a4;
    k = 0;
    if ((!a2) >= a0)
    {
        if (a0 == 1)
        {
            if (a3)
                goto LABEL_4060d2;
LABEL_4060d2:
            __unsupported_jumpkind_Ijk_NoDecode()
LABEL_406026:
            error(0, 0, dcgettext(NULL, v58, 5));
            sub_4054f0(1); /* do not return */
        }
        v54 = "missing file operand";
LABEL_4060a1:
        error(0, 0, dcgettext(NULL, v54, 5));
        sub_4054f0(1); /* do not return */
    }
    v12.st_nlink = 0;
    if ((char)a3)
    {
        if (v4)
            error(1, 0, dcgettext(NULL, "cannot combine --target-directory (-t) and --no-target-directory (-T)", 5));
        if (i > 2)
        {
            v55 = sub_414440(4, (long long)a1[16]);
            *(v55) = *(v55) + (unsigned int)v55;
            goto LABEL_406026;
        }
    }
    else if (v4)
    {
        /* unsupported instruction */ = (int)(long long *)((char *)&<0x405a50[is_7]|Stack bp-0x1148, 1 B> - 4120);
        v6 = sub_4156a0(v4, &v12);
        if (v6 == 0xffffffff)
        {
            sub_414440(4, v4);
            v15 = dcgettext(NULL, "target directory %s", 5);
            err = __errno_location();
            error(1, *(err), v15);
        }
        if (i != 1)
            goto LABEL_405eef;
        goto LABEL_405aeb;
    }
    else
    {
        v17 = i;
        v4 = *((long long *)((char *)a1 + 8 * v17 - 8));
        /* unsupported instruction */ = (int)(long long *)((char *)&<0x405a50[is_7]|Stack bp-0x1148, 1 B> - 4120);
        v6 = sub_4156a0(v4, &v12);
        if (v6 != 0xffffffff)
        {
            i = (unsigned int)v17 - 1;
            if (v4)
            {
                if (i > 1)
                {
LABEL_405eef:
                    sub_4075f0(v5);
                    /* unsupported instruction */ = (int)(long long *)((char *)&<0x405a50[is_7]|Stack bp-0x1148, 1 B> - 4120);
                    sub_407630(v5);
                }
                else if (i != 1)
                {
                    return 1;
                }
LABEL_405aeb:
                idx = 0;
                v19 = 1;
                v2 = &v11;
                v3 = &v9;
                v1 = &k;
                do
                {
                    v20 = *((long long *)((char *)a1 + 8 * idx));
                    v11 = 0;
                    if (!g_423158)
                    {
                        v21 = v20;
                        if (g_423159)
                            goto LABEL_405d80;
                        goto LABEL_405b4d;
                    }
                    else
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        sub_40e5a0(v20);
                        v21 = v20;
                        if (!g_423159)
                        {
LABEL_405b4d:
                            ptr1 = sub_40d970(v21);
                            /* unsupported instruction */ = /* unsupported instruction */ - 16;
                            len1 = strlen(ptr1);
                            v24 = len1 + 24;
                            l = /* unsupported instruction */ - (v24 & 0xfffffffffffff000);
                            if (/* unsupported instruction */ != l)
                            {
                                do
                                {
                                    /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                                    *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8));
                                } while (/* unsupported instruction */ != l);
                            }
                            v26 = (unsigned int)v24 & 0xfffffff0 & 0xfff;
                            /* unsupported instruction */ = /* unsupported instruction */ - v26;
                            if (v26)
                                *((long long *)(/* unsupported instruction */ + v26 - 8)) = *((long long *)(/* unsupported instruction */ + v26 - 8));
                            dst = memcpy(/* unsupported instruction */ + 15 & 0xfffffffffffffff0, ptr1, len1 + 1);
                            sub_40e5a0(dst);
                            /* unsupported instruction */ = /* unsupported instruction */ - 32;
                            v28 = sub_40ea10(v4, dst + (strcmp(dst, "..") < 1), v2);
                            goto LABEL_405bf4;
                        }
                        else
                        {
LABEL_405d80:
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            len = strlen(v21);
                            v30 = len + 24;
                            m = /* unsupported instruction */ - (v30 & 0xfffffffffffff000);
                            if (/* unsupported instruction */ != m)
                            {
                                do
                                {
                                    /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                                    *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8));
                                } while (/* unsupported instruction */ != m);
                            }
                            v32 = (unsigned int)v30 & 0xfffffff0 & 0xfff;
                            /* unsupported instruction */ = /* unsupported instruction */ - v32;
                            if (v32)
                                *((long long *)(/* unsupported instruction */ + v32 - 8)) = *((long long *)(/* unsupported instruction */ + v32 - 8));
                            dst1 = memcpy(/* unsupported instruction */ + 15 & 0xfffffffffffffff0, v20, len + 1);
                            sub_40e5a0(dst1);
                            /* unsupported instruction */ = /* unsupported instruction */ - 24;
                            v34 = NULL;
                            v28 = sub_40ea10(v4, dst1, v2);
                            v35 = v5;
                            v36 = v35->field_3c;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((unsigned long *)(/* unsupported instruction */ - 8)) = v37;
                            if ((char)v36)
                                v34 = "%s -> %s\n";
                            *((struct_0 **)(/* unsupported instruction */ - 8)) = v35;
                            v38 = sub_404b10(v28, v11 - v28, v6, v34, &v10, v1, v13);
                            /* unsupported instruction */ = /* unsupported instruction */;
                            if (*((char *)v11) == 47)
                            {
                                v39 = v11 + 1;
                                do
                                {
                                    v11 = v39;
                                    v39 += 1;
                                } while (*((char *)(v39 - 1)) == 47);
                            }
                            if (v38)
                            {
LABEL_405bf4:
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
                                *((char **)(/* unsupported instruction */ - 8)) = v3;
                                /* unsupported instruction */ = /* unsupported instruction */;
                                v19 &= sub_40ba80(v20, v28, v6, v11, k, v5, v13, v14);
                                if (!g_423159)
                                    continue;
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                v19 &= sub_405210(v28, v6, v11, v10, v5);
                                v40 = g_423159;
                                goto LABEL_405e83;
                            }
                            else
                            {
                                v40 = g_423159;
                                v19 = 0;
LABEL_405e83:
                                if (v40)
                                {
                                    ptr = v10;
                                    if (ptr)
                                    {
                                        do
                                        {
                                            v10 = *((long long *)&ptr[2].padding_17[15]);
                                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                            free(ptr);
                                            ptr = *((long long *)&ptr[2].padding_17[15]);
                                        } while (ptr);
                                    }
                                }
                            }
                        }
                    }
                    idx += 1;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    free(v28);
                } while (i > (unsigned int)idx);
                return v19;
            }
        }
        else
        {
            if (*(__errno_location()) == 2)
                goto LABEL_405fc0;
            for (; i > 2; k = 1)
            {
                v43 = sub_414440(4, v4);
                *((char *)v44) = *((char *)v44) + 5;
                *((unsigned int *)((char *)v44 - 0x77)) = *((int *)((char *)v44 - 0x77)) - 1;
                __outbyte(191, *((char *)&v43));
                *((unsigned int *)v43) = *((int *)v43) + (unsigned int)v43;
                *((char *)v43) = *((char *)v43) + *((char *)&v43);
                error(v45, v46, v43);
LABEL_405fc0:
            }
        }
    }
    v47 = *((long long *)a1);
    v48 = (long long)a1[8];
    if (!g_423159)
    {
        if (v5->field_16 && v5->field_0 && !strcmp(v47, v48) && !k && (v12.st_nlink || (v49 = (unsigned int)(int)(long long)stat(v48, &v12), !(int)(long long)stat(v48, &v12))))
        {
            if (((unsigned short)v12.st_nlink & 0xf000) == 0x8000)
            {
                j = v5;
                v51 = sub_40d850(0xffffff9c, v48, j->field_0);
                iter = &g_423100.field_0;
                for (v53 = 22; v53; j = j->padding_4)
                {
                    v53 -= 1;
                    iter->field_0 = j->field_0;
                    iter = iter->padding_4;
                }
                v48 = v51;
                g_423100.field_0 = 0;
                v5 = &g_423100.field_0;
            }
        }
        v0 = (int)_INSERT(<0x405a50[is_7]|Stack bp-0x1148, 1 B>, 4120, 0);
        *((char **)&(&v0)[0x1010]) = &v11;
        return sub_40ba80(v47, v48, 0xffffff9c, v48, -(k), v5, *((unsigned long long *)(&v0 + 0x1010)), *((unsigned long long *)(&v0 + 4120)));
    }
    v54 = "with --parents, the destination must be a directory";
    goto LABEL_4060a1;
    goto LABEL_4060a1;
}



// Function: copy_attr_free @ 0x60f0
void copy_attr_free(void)
{
    return;
}



// Function: copy_attr_quote @ 0x6100
void copy_attr_quote(void)
{
    sub_414440(4);
    return;
}



// Function: subst_suffix @ 0x6110
void* subst_suffix(void* a0, unsigned long a1, void* a2)
{
    unsigned long v1;  // r12
    unsigned long n;  // r15
    void* v3;  // rax

    v1 = a1 - a0;
    n = strlen(a2) + 1;
    v3 = sub_416c70(n + v1);
    memcpy(v3 + v1, a2, n);
    return memcpy(v3, a0, v1);
}



// Function: write_zeros @ 0x6170
extern void g_423018;
extern char g_423160;
extern unsigned long long g_423560;

unsigned int write_zeros(unsigned long a0, unsigned long a1)
{
    unsigned long v1;  // rbp
    unsigned long v2;  // rbp
    unsigned long v3;  // rbx
    unsigned long v4;  // rax

    v1 = a1;
    if (!g_423560)
    {
        g_423560 = calloc(*((long long *)&g_423018), 1);
        if (!g_423560)
        {
            *((unsigned long *)&g_423018) = 0x400;
            g_423560 = &g_423160;
        }
    }
    if (!v1)
        return 1;
    while (1)
    {
        v2 = v1;
        v3 = *((long long *)&g_423018);
        if (v2 <= *((long long *)&g_423018))
            v3 = v2;
        v4 = sub_4111c0();
        if (v4 != v3)
            return 0;
        v1 = v2 - v4;
        if (v2 == v4)
            return 1;
    }
}



// Function: copy_attr @ 0x6220
typedef struct struct_0 {
    char padding_0[40];
    unsigned long long field_28;
    char padding_30[1];
    char field_31;
    char padding_32[1];
    char field_33;
    char padding_34[2];
    char field_36;
    char field_37;
} struct_0;

char copy_attr(unsigned long a0, unsigned int a1, unsigned long a2, unsigned int a3, struct_0 *a4)
{
    char v4;  // dl
    unsigned long long v5;  // r9
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]

    v4 = a4->field_33;
    if (!a4->field_31 || a4->field_36)
    {
        v5 = sub_406440;
        if (!v4)
        {
            v5 = sub_406440;
            goto LABEL_406263;
        }
    }
    else if (!a4->field_37)
    {
        v5 = sub_406360;
LABEL_406263:
        v0 = v5;
        v1 = sub_406100;
        v2 = sub_4060f0;
    }
    return ((a1 | a3) < 0 ? !attr_copy_file() : !attr_copy_fd());
}



// Function: copy_attr_error @ 0x6360
void copy_attr_error(unsigned long a0, unsigned long a1)
{
    unsigned long long v11;  // rdx
    unsigned long long v12;  // rcx
    unsigned long long v13;  // r8
    unsigned long long v14;  // r9
    unsigned int v16;  // esi
    unsigned long v17;  // fs
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

    v6 = v11;
    v7 = v12;
    v8 = v13;
    v9 = v14;
    __unsupported_jumpkind_Ijk_NoDecode()
    v4 = *((long long *)0x28);
    v16 = *(__errno_location());
    if (*(__errno_location()) != 95 && *(__errno_location()) != 61)
    {
        v2 = &v10;
        v0 = 16;
        v1 = 48;
        v3 = &v5;
        sub_4163b0(0);
    }
    if (v4 != *((long long *)(40 + v17)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: copy_attr_allerror @ 0x6440
void copy_attr_allerror(unsigned long a0, long long a1)
{
    unsigned long long v12;  // rdx
    unsigned long long v13;  // rcx
    unsigned long long v14;  // r8
    unsigned long long v15;  // r9
    char v16;  // al
    unsigned long v17;  // fs
    unsigned int *err;  // rax
    int <0x406440[is_6]|Stack bp-0xd8, 1 B>;  // [bp-0xd8]
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
    unsigned int v10;  // [bp-0x48]
    unsigned long v11;  // [bp+0x8]

    v6 = v12;
    v7 = v13;
    v8 = v14;
    v9 = v15;
    if (v16)
        v10 -= (unsigned int)&<0x406440[is_6]|Stack bp-0xd8, 1 B>;
    v4 = *((long long *)(40 + v17));
    err = __errno_location();
    v0 = 16;
    v2 = &v11;
    v3 = &v5;
    v1 = 48;
    sub_4163b0(0, *(err), a1, &v0);
    if (v4 != *((long long *)(40 + v17)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: emit_verbose @ 0x6510
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern struct_0 *stdout;

char * emit_verbose(long long a0, long long a1, long long a2)
{
    char *v1;  // rax
    char *v4;  // rax

    v1 = sub_414310(1, 4, a1);
    __printf_chk(1, "%s -> %s", sub_414310(0, 4, a0), v1);
    if (a2)
    {
        sub_414440(4, a2);
        __printf_chk(1, dcgettext(NULL, " (backup: %s)", 5));
    }
    v4 = *((long long *)&stdout[2].field_8);
    if (v4 >= *((long long *)&stdout[3].field_0))
        return __overflow(stdout, 10);
    *((char **)&stdout[2].field_8) = v4 + 1;
    *(v4) = 10;
    return v4;
}



// Function: create_hard_link @ 0x65d0
unsigned int create_hard_link(unsigned long long a0, unsigned int a1, unsigned long a2, unsigned long a3, unsigned int a4, long long a5, char a6, unsigned int a7, char a8)
{
    unsigned long v1;  // rax
    unsigned long v2;  // r9
    unsigned long v3;  // rax
    char *v4;  // rax
    unsigned long v0;  // [bp-0x40]

    v1 = sub_40be90(a1, a2, a4, a5, a8 * 0x400, a6, -0x1);
    if ((unsigned int)v1 <= 0)
    {
        __unsupported_jumpkind_Ijk_NoDecode()
        return 1;
    }
    v2 = 0;
    v3 = v1;
    if (!a0)
    {
        v3 = sub_406110(a3, a5, a2);
        v2 = v3;
    }
    v4 = (char *)_INSERT(v3, 0, v3);
    *(v4) = *(v4) + (char)v3;
    v0 = v2;
    sub_414310(1);
    sub_414310(0, 4, a3);
    error(0, v1, dcgettext(NULL, "cannot create hard link %s to %s", 5));
    free(v0);
    return 0;
}



// Function: overwrite_ok @ 0x67a0
typedef struct FILE {
} FILE;

typedef struct struct_1 {
    char padding_0[20];
    unsigned int field_14;
    char field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_2 {
    unsigned int field_0;
} struct_2;

extern FILE *stdin;
extern unsigned long long stderr;
extern struct_2 *g_4235a0;

unsigned int overwrite_ok(struct_1 *a0, long long a1, unsigned int a2, long long a3, struct_0 *a4)
{
    unsigned long long v11;  // r14
    unsigned long long v12;  // r13
    unsigned long v21;  // fs
    long long v22;  // rax
    char *v23;  // rdi
    void* v24;  // rax
    unsigned int v25;  // r12d
    unsigned long long v13;  // r12
    unsigned long long v14;  // rbx
    char *v15;  // rax
    unsigned long v16;  // rax
    char *v18;  // rax
    int v19;  // edi
    FILE *v20;  // rsi
    char *v0;  // [bp-0x58]
    char v1;  // [bp-0x48]
    char v2;  // [bp-0x3c]
    char v3;  // [bp-0x32]
    unsigned long v4;  // [bp-0x30]
    unsigned long long v5;  // [bp-0x28]
    char *ptr;  // [bp-0x20]
    unsigned long v7;  // [bp-0x18], Other Possible Types: unsigned long long
    unsigned long v8;  // [bp-0x10], Other Possible Types: unsigned long long
    unsigned long long v9;  // [bp-0x8]

    v9 = v11;
    v8 = v12;
    v7 = v13;
    v5 = v14;
    if (((unsigned short)a4->field_18 & 0xf000) == 0xa000 || (char)sub_416bb0() || !faccessat(a2, a3, 2, 0x200))
    {
        sub_414440(4);
        v15 = dcgettext(NULL, "%s: overwrite %s? ", 5);
        __fprintf_chk(stderr, 0x1, v15);
    }
    else
    {
        sub_40e880(a4->field_18, &v2);
        v3 = 0;
        sub_414440(4, a1);
        if (a0->field_18 || a0->field_14 & 0xffffff & 0xffff00)
            v16 = dcgettext(NULL, "%s: replace %s, overriding mode %04lo (%s)? ", 5);
        else
            v16 = dcgettext(NULL, "%s: unwritable %s (mode %04lo, %s); try anyway? ", 5);
        v18 = (unsigned int)v16 - 130184376 + (&v1 < 0x8);
        g_4235a0->field_0 = 0x1be00;
        *(v18) = *(v18) + *((char *)&v18);
        v0 = v18;
        __fprintf_chk(v19, v20, v16);
    }
    if (v4 != *((long long *)(40 + v21)))
        __stack_chk_fail(); /* do not return */
    v9 = v7;
    v8 = *((long long *)(40 + v21));
    ptr = NULL;
    v7 = 0;
    v22 = __getdelim(&ptr, &v7, 10, stdin);
    if (v22 <= 0)
    {
        free(ptr);
        return 0;
    }
    v23 = ptr;
    v24 = &v23[v22] - 1;
    if (*((char *)v24) == 10)
    {
        *((char *)v24) = 0;
        v23 = ptr;
    }
    v25 = rpmatch(v23) > 0;
    free(ptr);
    return v25;
}



// Function: create_hole @ 0x6920
unsigned int create_hole(unsigned int a0, long long a1, char a2, long long a3)
{
    long long v1;  // rax
    int *err;  // rax
    int v3;  // eax
    char *v4;  // rax
    char *v5;  // rax
    int *err1;  // rax

    v1 = lseek(a0, a3, 1);
    if (v1 < 0)
    {
        sub_414440(4, a1);
        v5 = dcgettext(NULL, "cannot lseek %s", 5);
        err1 = __errno_location();
        error(0, *(err1), v5);
        return 0;
    }
    if (!a2 || fallocate(a0, 3, v1 - a3, a3) >= 0)
        return 1;
    err = __errno_location();
    v3 = *(err);
    if (v3 != 95 && v3 != 38)
    {
        sub_414440(4, a1);
        v4 = dcgettext(NULL, "error deallocating %s", 5);
        error(0, *(err), v4);
        return _INSERT(a3, 0, v3 == 95 | v3 == 38);
    }
    return 1;
}



// Function: sparse_copy @ 0x6a30
typedef struct struct_0 {
    char field_0;
} struct_0;


unsigned int sparse_copy(unsigned int a0, unsigned int a1, struct_0 **a2, unsigned long a3, unsigned long a4, char a5, char a6, long long a7, long long a8, unsigned long a9, unsigned long long *a10, char *a11)
{
    char *v10;  // rcx
    unsigned long long *v11;  // rsi
    unsigned long iter;  // r15
    unsigned long long v12;  // r13
    unsigned long iter2;  // r12
    unsigned long v14;  // rax
    int *err2;  // rax
    int v16;  // ecx
    char *v17;  // rax
    unsigned long v18;  // rdi
    unsigned int v19;  // r14d
    unsigned long v3;  // [bp-0x98]
    unsigned long v4;  // [bp-0x90]
    unsigned long long *v6;  // [bp-0x78]
    unsigned long v8;  // [bp-0x50]
    char *v9;  // [bp-0x48]

    v4 = a3;
    v10 = a11;
    v11 = a10;
    *(a11) = 0;
    v3 = a9;
    v6 = a10;
    v9 = a11;
    *(a10) = 0;
    if (!a4 && a6)
    {
        v12 = a0;
        iter2 = a9;
    }
    else if (!(!v3))
    {
        goto LABEL_406b28;
    }
    while (1)
    {
        while (1)
        {
            v14 = copy_file_range();
            if (!v14)
            {
                v3 = iter2;
                if (!*(v6))
                    goto LABEL_406b28;
            }
            if (v14 < 0)
                break;
            *(v6) = *(v6) + v14;
            iter2 -= v14;
        }
        err2 = __errno_location();
        v16 = *(err2);
        if (v16 == 38)
            break;
        if (v16 <= 26)
        {
            if (v16 > 0)
            {
                if ((unsigned int)(1 << ((char)v16 & 63)) & 0x4440200)
                    break;
                if (v16 == 1)
                {
                    v3 = iter2;
                    if (!*(v6))
                        goto LABEL_406b28;
LABEL_406ece:
                    sub_414310(1, 4, a8);
                    sub_414310(0, 4, a7);
                    v17 = dcgettext(NULL, "error copying %s to %s", 5);
                    error(0, *(err2), v17);
                    return 0;
                }
            }
            if (v16 != 4)
                goto LABEL_406ece;
        }
        else
        {
            v3 = iter2;
            if (v16 == 95)
                goto LABEL_406b28;
            goto LABEL_406ece;
        }
    }
    v3 = iter2;
LABEL_406b28:
    if (a4)
        v18 = a4;
    else
        v18 = v4;
    v19 = 0;
    iter = 0;
    v8 = v18;
}



// Function: set_process_security_ctx @ 0x72a0
typedef struct struct_0 {
    char padding_0[40];
    long long field_28;
    char padding_30[1];
    char field_31;
    char padding_32[1];
    char field_33;
    char field_34;
    char padding_35[2];
    char field_37;
} struct_0;

unsigned int set_process_security_ctx(long long a0, long long a1, unsigned long a2, char a3, struct_0 *a4)
{
    unsigned long v2;  // r12
    int *err;  // rax
    int *err1;  // rax
    char *v5;  // rax
    char *v6;  // rax
    long long v7;  // rdi
    int *err2;  // rax
    char *v9;  // rax
    long long v0;  // [bp-0x38]

    v2 = a4->field_33;
    if ((char)v2)
    {
        if (!a4->field_31 || a4->field_34)
        {
            if (sub_415590(a0, &v0) < 0)
            {
                err = __errno_location();
LABEL_4072f8:
                sub_414440(4, a0);
                v5 = dcgettext(NULL, "failed to get security context of %s", 5);
                error(0, *(err), v5);
                goto LABEL_40732e;
            }
            else
            {
                if (setfscreatecon(v0) >= 0)
                {
                    freecon(v0);
                    return v2;
                }
                err1 = __errno_location();
LABEL_4074ba:
                sub_414b30(v0);
                v6 = dcgettext(NULL, "failed to set default file creation context to %s", 5);
                error(0, *(err1), v6);
            }
        }
        else
        {
            if (a4->field_37)
            {
                if (sub_415590() < 0)
                    goto LABEL_40732e;
                if (setfscreatecon(v0) >= 0)
                {
                    freecon(v0);
                    return v2;
                }
            }
            else if (sub_415590() < 0)
            {
                err = __errno_location();
                if (*(err) != 95 && *(err) != 61)
                    goto LABEL_4072f8;
LABEL_40732e:
                if (!a4->field_34)
                    return v2;
                return 0;
            }
            else
            {
                if (setfscreatecon(v0) >= 0)
                {
                    freecon(v0);
                    return v2;
                }
                err1 = __errno_location();
                if (*(err1) != 95 && *(err1) != 61)
                    goto LABEL_4074ba;
            }
        }
        v0 = v0;
        if (!a4->field_34)
        {
            freecon(v0);
            return v2;
        }
        freecon(v0);
        return 0;
    }
    else
    {
        v7 = a4->field_28;
        if (!(v7 & a3) || sub_40c450(v7) >= 0)
            return 1;
        err2 = __errno_location();
        if (*(err2) != 95 && *(err2) != 61)
        {
            sub_414440(4, a1);
            v9 = dcgettext(NULL, "failed to set default file creation context for %s", 5);
            error(0, *(err2), v9);
            return _INSERT(v2, 0, v7 & a3);
        }
        return 1;
    }
}



// Function: set_file_security_ctx @ 0x7510
typedef struct struct_0 {
    char padding_0[40];
    long long field_28;
    char padding_30[1];
    char field_31;
    char padding_32[2];
    char field_34;
    char padding_35[2];
    char field_37;
} struct_0;

unsigned int set_file_security_ctx(long long a0, char a1, struct_0 *a2)
{
    unsigned int v1;  // eax
    int *err;  // rax
    char *v3;  // rax

    if (!a2->field_31 || a2->field_34)
    {
        if ((char)sub_40c6f0(a2->field_28, a0, a1))
            return 1;
        err = __errno_location();
    }
    else if (a2->field_37)
    {
        return sub_40c6f0();
    }
    else
    {
        v1 = sub_40c6f0();
        if ((char)v1)
            return 1;
        err = __errno_location();
        if (*(err) == 95)
        {
            return v1;
        }
        else if (*(err) == 61)
        {
            return v1;
        }
    }
    sub_414310(0, 4, a0);
    v3 = dcgettext(NULL, "failed to set the security context of %s", 5);
    error(0, *(err), v3);
    return 0;
}



// Function: dest_info_init @ 0x75f0
typedef struct struct_0 {
    char padding_0[72];
    unsigned long long field_48;
} struct_0;

extern long long g_4124d0;

void dest_info_init(struct_0 *a0)
{
    unsigned long long v1;  // rax

    v1 = sub_411bb0(61, 0, sub_412460, sub_412430, &g_4124d0);
    a0->field_48 = v1;
    if (!v1)
        sub_417150(); /* do not return */
    return;
}



// Function: src_info_init @ 0x7630
typedef struct struct_0 {
    char padding_0[80];
    unsigned long long field_50;
} struct_0;

extern long long g_4124d0;

void src_info_init(struct_0 *a0)
{
    unsigned long long v1;  // rax

    v1 = sub_411bb0(61, 0, sub_412410, sub_412430, &g_4124d0);
    a0->field_50 = v1;
    if (!v1)
        sub_417150(); /* do not return */
    return;
}



// Function: cp_options_default @ 0x7670
typedef struct struct_0 {
    unsigned long long field_0;
    char padding_8[18];
    char field_1a;
    char field_1b;
    char padding_1c[36];
    unsigned int field_40;
    char padding_44[12];
    unsigned long long field_50;
} struct_0;

char cp_options_default(struct_0 *idx)
{
    struct_0 *v1;  // rdi
    unsigned long long *iter;  // rdi
    unsigned long long v3;  // rcx
    unsigned int v4;  // eax

    v1 = idx->padding_8;
    *((unsigned long long *)((char *)v1 - 8)) = 0;
    *((unsigned long long *)&v1->padding_44[4]) = 0;
    iter = v1 & 0xfffffffffffffff8;
    for (v3 = (unsigned int)(idx - iter) + 88 >> 3; v3; iter += 1)
    {
        v3 -= 1;
        *(iter) = 0;
    }
    v4 = geteuid();
    idx->field_40 = 0xffffffff;
    idx->field_1b = !v4;
    idx->field_1a = !v4;
    return !v4;
}



// Function: copy_internal @ 0x76f0
typedef struct struct_4 {
    struct struct_2 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_4;

typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct struct_1 {
    char field_0;
    char padding_1[23];
    unsigned int field_18;
} struct_1;

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

typedef struct struct_2 {
    struct struct_2 *field_0;
} struct_2;

extern unsigned int g_423010;
extern long long g_423568;
extern long long g_423570;
extern void* g_423580;

unsigned int copy_internal(char *a0, char *a1, unsigned int a2, char *a3, unsigned int a4, unsigned int *a5, struct_2 **a6, void* a7, long long a8, char *a9, char *a10, unsigned long long a11)
{
    char *v50;  // r15
    char *v51;  // rbx
    unsigned long long v60;  // rax
    unsigned int v150;  // r8d
    char v151;  // al
    unsigned int v152;  // r10d
    char v154;  // al
    char *v155;  // rbx
    int *err;  // rax
    unsigned int v157;  // eax
    unsigned int v158;  // eax
    unsigned int v159;  // eax
    unsigned long v61;  // rbx
    unsigned int v160;  // esi
    long long v162;  // rax
    unsigned int v163;  // r8d
    int *err1;  // rax
    int v165;  // eax
    long long v167;  // rcx
    long long v168;  // rax
    char v169;  // r9b
    char v62;  // al
    char v170;  // r8b
    long long v171;  // r12
    unsigned long v172;  // r14
    long long v173;  // r15
    long long v174;  // rax
    unsigned long v175;  // rcx
    long long v176;  // rcx
    long long v177;  // r8
    long long v178;  // rax
    long long v179;  // r8
    int *err2;  // rax
    char *v180;  // rsi
    char v181;  // dl
    int *v182;  // rax
    char v183;  // r8b
    int *v184;  // rax
    int v185;  // eax
    char *v186;  // rax
    char *v187;  // rsi
    int *v188;  // rax
    unsigned long v189;  // rax
    char *v64;  // rax
    unsigned long v190;  // rdx
    unsigned int v191;  // eax
    char v192;  // al
    unsigned int v193;  // eax
    int *v194;  // rax
    int *v195;  // rax
    char *v196;  // rax
    int *v197;  // rax
    char *v198;  // rax
    int *v199;  // rax
    char v65;  // dl
    unsigned long long v200;  // rax
    char v201;  // bl
    char v202;  // al
    char v203;  // al
    unsigned int v204;  // eax
    unsigned int v205;  // r12d
    long long v206;  // rdx
    long long v207;  // rcx
    struct_2 **v208;  // rax
    unsigned int v66;  // r8d
    unsigned int v210;  // ebx
    char *v212;  // rbx
    void* v213;  // r12
    void* v214;  // r15
    char v215;  // al
    char v216;  // al
    char *v217;  // rax
    int *v218;  // rax
    char *v219;  // rax
    void* v67;  // r12
    int *v220;  // rax
    unsigned long v221;  // rax
    unsigned long v222;  // rdx
    unsigned int v223;  // eax
    unsigned int v224;  // ebx
    unsigned int v225;  // ebx
    char *v226;  // rsi
    char *v227;  // rax
    int *v228;  // rax
    char v68;  // r12b
    char *v229;  // rax
    int *v230;  // rax
    char *v232;  // rax
    int *v233;  // rax
    char *v234;  // rax
    char v69;  // cl
    unsigned int v52;  // eax
    unsigned int v70;  // eax
    unsigned long v71;  // r13
    unsigned int v72;  // eax
    char *v73;  // rsi
    unsigned long long v74;  // rax
    unsigned int v75;  // eax
    unsigned int v76;  // eax
    unsigned int v77;  // r13d
    unsigned long v78;  // r12
    char v79;  // al
    void* idx;  // r14
    char *v80;  // r13
    int *v81;  // rax
    char *v82;  // rax
    char *v84;  // r13
    unsigned long len;  // rax
    unsigned long v86;  // rax
    unsigned int v87;  // eax
    char *v88;  // r12
    void* ptr;  // r12
    unsigned long v54;  // r13
    unsigned long v90;  // r13
    unsigned long len1;  // rax
    unsigned long long v92;  // rax
    char *i;  // rsi
    unsigned long long v94;  // rcx
    int *v96;  // rax
    unsigned int v97;  // r12d
    struct_1 *v98;  // rdx
    char *v55;  // rsi
    unsigned long v100;  // r12
    char v101;  // al
    char *v102;  // r15
    unsigned long v103;  // rax
    unsigned int v105;  // ebx
    int *v106;  // rax
    char *v107;  // rax
    int v108;  // esi
    char *v109;  // rdx
    long long v56;  // rdi
    unsigned int v110;  // r13d
    unsigned int v116;  // ebx
    unsigned int v117;  // eax
    unsigned int v118;  // eax
    char v119;  // r13b
    unsigned int v57;  // r12d
    unsigned long ptr1;  // rbx
    char *v121;  // rsi
    unsigned int v122;  // eax
    char v123;  // r9b
    unsigned int v125;  // eax
    char *v126;  // rsi
    unsigned long ptr2;  // rbx
    unsigned int v128;  // eax
    char *v129;  // rax
    char *v58;  // rdx
    char *v131;  // rax
    int *v132;  // rax
    unsigned int v133;  // ebx
    long long v134;  // rdx
    unsigned int v135;  // r13d
    unsigned int v137;  // ebx
    int *v138;  // rax
    unsigned int v139;  // r8d
    char *v59;  // rsi
    char v140;  // r9b
    char v141;  // al
    char *v143;  // rdx
    int v144;  // esi
    char v145;  // r9b
    char *v146;  // rbx
    unsigned long long v147;  // r10
    unsigned int *v148;  // rax
    unsigned int v149;  // r10d
    uint8_t v237[*((long long *)((char *)stack_base - 824))];  // alloca
    char v0;  // [bp-0x23e8]
    char v2;  // [bp-0x630]
    unsigned long long v3;  // [bp-0x3e0]
    unsigned int v4;  // [bp-0x3d8]
    unsigned int v5;  // [bp-0x3d4]
    long long v6;  // [bp-0x3d0]
    unsigned int v7;  // [bp-0x3c8]
    unsigned int v8;  // [bp-0x3c4]
    void* v9;  // [bp-0x3c0], Other Possible Types: unsigned int
    char v10;  // [bp-0x3b8], Other Possible Types: long long
    char *v11;  // [bp-0x3b0], Other Possible Types: unsigned int
    struct_4 *index;  // [bp-0x3a8], Other Possible Types: char
    void* v13;  // [bp-0x3a0], Other Possible Types: unsigned int
    char *v14;  // [bp-0x398], Other Possible Types: long long, unsigned long
    char *v15;  // [bp-0x390], Other Possible Types: long long, unsigned long
    void* v16;  // [bp-0x388], Other Possible Types: unsigned int
    int *v17;  // [bp-0x380], Other Possible Types: char *, unsigned int, unsigned long, unsigned long long
    unsigned int *v18;  // [bp-0x378], Other Possible Types: char *, int *, long long, unsigned long, unsigned long long
    char *v19;  // [bp-0x370], Other Possible Types: unsigned long, unsigned long long
    char v20;  // [bp-0x368], Other Possible Types: unsigned long long
    struct_2 **v21;  // [bp-0x360], Other Possible Types: unsigned int, unsigned long, char, unsigned long long
    char *v22;  // [bp-0x358], Other Possible Types: int *, long long, unsigned long
    char *len2;  // [bp-0x350], Other Possible Types: struct_0 *, unsigned long, unsigned long long, unsigned int
    unsigned long v24;  // [bp-0x348], Other Possible Types: char, unsigned long long
    char *v25;  // [bp-0x328], Other Possible Types: char, unsigned long, unsigned long long
    unsigned long v26;  // [bp-0x320]
    char v27;  // [bp-0x312]
    char v28;  // [bp-0x311]
    char *v29;  // [bp-0x310]
    int v30;  // [bp-0x308], Other Possible Types: unsigned long
    unsigned int v31;  // [bp-0x304]
    int v32;  // [bp-0x2f8]
    int v33;  // [bp-0x2e8]
    int v34;  // [bp-0x2d8]
    int v35;  // [bp-0x2c8]
    unsigned long long v36;  // [bp-0x2b8]
    long long v37;  // [bp-0x2a8]
    long long v38;  // [bp-0x2a0]
    long long v39;  // [bp-0x298]
    long long v40;  // [bp-0x290]
    stat v41;  // [bp-0x288], Other Possible Types: char
    stat v42;  // [bp-0x1f8], Other Possible Types: struct_0
    stat v43;  // [bp-0x168], Other Possible Types: int
    stat v44;  // [bp-0xd8], Other Possible Types: int, char, long long
    int v45;  // [bp-0xd0], Other Possible Types: long long
    long long v46;  // [bp-0xc8]
    long long v47;  // [bp-0xc0]
    char v48;  // [bp-0x41]
    long long v49;  // [bp+0x0]

    v50 = a0;
    /* unsupported instruction */ = (int)(&v2 - 584);
    v51 = a10;
    v21 = a6;
    v52 = a8;
    v18 = a5;
    idx = a7;
    *((unsigned int *)&v25) = v52;
    v22 = a9;
    v54 = (int)idx[64];
    v19 = v51;
    v24 = a11;
    v20 = v52;
    *(v51) = 0;
    *((char *)&v26) = (char)idx[24];
    if (!(char)idx[24])
    {
        *((char *)&v26) = a4 > 0;
        if ((unsigned int)v54)
            goto LABEL_40779d;
    }
    else if ((unsigned int)v54 >= 0)
    {
LABEL_407989:
        *((char *)&v26) = !(unsigned int)v54;
        a4 = (char)v26;
        *((char *)v24) = a4;
        if ((unsigned int)v54)
        {
LABEL_40779d:
            if ((unsigned int)v54 != 0x11 || (int)idx[8] != 2)
            {
                v55 = v50;
                v56 = 0xffffff9c;
                goto LABEL_4077b2;
            }
        }
    }
    else
    {
        /* unsupported instruction */ = (int)(&v2 - 576);
        if (!sub_414b50(0xffffff9c, a0, a2, a3, 1))
        {
            a4 = 1;
            *((char *)v24) = 1;
        }
        else
        {
            /* unsupported instruction */ = (int)(&v2 - 576);
            v54 = *(__errno_location());
            goto LABEL_407989;
        }
    }
    if (!(char)idx[63])
    {
        v56 = a2;
        v54 = 0;
        v55 = a3;
LABEL_4077b2:
        /* unsupported instruction */ = (int)(&v2 - 576);
        if (fstatat(v56, v55, &v41, ((int)idx[4] == 2) * 0x100))
            goto LABEL_408528;
        a4 = v41.st_nlink;
        if (((unsigned short)a4 & 0xf000) == 0x4000 && (v57 = (unsigned int)(char)(char)idx[56], !(char)v57))
        {
            sub_414440(4, v50);
            if (!(char)idx[25])
            {
                v58 = dcgettext(NULL, "-r not specified; omitting directory %s", 5);
                goto LABEL_4084c7;
            }
            else
            {
                v59 = "omitting directory %s";
                goto LABEL_409ad4;
            }
        }
    }
    else
    {
        v54 = 0;
    }
    if ((char)v25 && (long long)idx[80])
    {
        if (((unsigned short)a4 & 0xf000) != 0x4000 && !*((int *)idx))
        {
            v17 = a4;
            len2 = &v41;
            v57 = sub_40e820();
            if ((char)v57)
            {
                sub_414440(4, v50);
                v59 = "warning: source file %s specified more than once";
LABEL_409ad4:
                v58 = dcgettext(NULL, v59, 5);
LABEL_4084c7:
                error(0, 0, v58);
                return v57;
            }
            a4 = v17;
        }
        len2 = a4;
        /* unsupported instruction */ = (int)(&v2 - 576);
        sub_40e790();
        a4 = len2;
    }
    if ((int)idx[4] == 4)
    {
        v17 = (char *)_INSERT(v17 CONCAT 0, 0, 1);
LABEL_40787c:
        len2 = 0;
        v51 = NULL;
        if (a4 <= 0)
            goto LABEL_407b88;
        goto LABEL_407892;
    }
    if ((int)idx[4] != 3 || !(char)v25)
    {
        v17 = (char *)_INSERT(v17 CONCAT 0, 0, 0);
        goto LABEL_40787c;
    }
    v17 = (char *)_INSERT(v17 CONCAT 0, 0, 1);
    if (a4 <= 0)
    {
LABEL_407b88:
        if ((unsigned int)v54 != 0x11 || (int)idx[8] != 2)
        {
            v60 = a4 & 0xf000;
            if ((unsigned int)v60 != 0x8000 && !(v61 = (unsigned long)(unsigned long long)((unsigned int)_INSERT(v51, 0, (char)(char)((unsigned int)v60 == 0xa000)) | (unsigned int)_INSERT(v60, 0, (char)(char)((unsigned int)v60 == 0x4000))), v62 = (char)idx[20] ^ 1, v51 = (char *)_INSERT(v61, 0, (char)v61 | v62), !(char)v61 && !v62) || (v51 = (char *)(unsigned long long)(char)(char)idx[24], *((char *)(void*)&v51) || (v51 = (char *)(unsigned long long)(char)(char)idx[58], *((char *)(void*)&v51) || (v51 = (char *)(unsigned long long)(char)(char)idx[23], *((char *)(void*)&v51)))))
            {
LABEL_407bca:
                a4 = 0x100;
                goto LABEL_407bd0;
            }
            if (*((int *)idx))
            {
                v51 = 0x1;
                goto LABEL_407bca;
            }
            v51 = (char)idx[21];
            if (*((char *)&v51))
                goto LABEL_407bca;
            if (a4)
                goto LABEL_408460;
LABEL_407bd0:
            /* unsupported instruction */ = (int)(&v2 - 576);
            if (!fstatat(a2, a3, &v42, a4))
                goto LABEL_407bf4;
            /* unsupported instruction */ = (int)(&v2 - 576);
            err2 = __errno_location();
            if (*(err2) == 40)
            {
                if ((char)idx[22])
                    goto LABEL_408467;
LABEL_4083d4:
                sub_414440(4, a1);
                v64 = dcgettext(NULL, "cannot stat %s", 5);
                error(0, *(err2), v64);
                return 0;
            }
            else if (!(*(err2) == 2))
            {
                goto LABEL_4083d4;
            }
LABEL_408460:
            *((char *)&v26) = 1;
LABEL_408467:
            len2 = 0;
            v51 = NULL;
            if ((unsigned int)v54 != 0x11)
                goto LABEL_407892;
LABEL_407bf4:
            if ((int)idx[8] != 2)
            {
                if (*((unsigned long long *)((void*)&v41 + 8)) != *((unsigned long long *)((void*)&v42 + 8)) || *((unsigned long long *)&v41) != *((unsigned long long *)&v42))
                {
                    v44 = v44;
                    if ((int)idx[4] == 2)
                    {
                        v65 = 0;
                        v66 = 0;
                        goto LABEL_407c23;
                    }
                }
                v54 = (char)idx[23];
                if (!(char)v54)
                {
                    if ((int)idx[4] != 2)
                    {
                        v67 = &v43;
                        /* unsupported instruction */ = (int)(&v2 - 576);
                        if (!fstatat(a2, a3, &v43, 0x100) && (len2 = (struct_0 *)&v44, /* unsupported instruction */ = (int)(&v2 - 576), !(int)(long long)lstat(v50, &v44)))
                        {
                            v65 = 0;
                            if (*((unsigned long long *)&v45) == v43.st_ino)
                                v65 = *((long long *)&v44.st_mode) == *((unsigned long long *)&v43);
                            if (((unsigned short)*((unsigned int *)(&v45 + 16)) & 0xf000) != 0xa000 || !(v44 = v44, ((unsigned short)v43.st_nlink & 0xf000) == 0xa000 && (v44 = v44, (char)idx[21])))
                            {
LABEL_407c47:
                                if (*((int *)idx))
                                {
                                    if (v65)
                                    {
                                        v54 = 0;
                                        /* unsupported instruction */ = (int)(&v2 - 576);
                                        v68 = (char)sub_414e20(0xffffff9c, v50, a2, a3) ^ 1;
                                        goto LABEL_40a173;
                                    }
                                    v44 = v44;
                                    if (!(char)idx[24] && !(v44 = v44, (int)idx[4] == 2 || (v44 = v44, ((unsigned short)len2->field_18 & 0xf000) != 0xa000 || (v44 = v44, ((short)(int)v67[24] & 0xf000) == 0xa000))))
                                        goto LABEL_407c99;
                                    goto LABEL_408af0;
                                }
                                v69 = (char)idx[24];
                                if (!(char)idx[24] && !(char)idx[21])
                                {
                                    if ((len2->field_18 & 0xf000) == 0xa000)
                                        goto LABEL_409be8;
                                    goto LABEL_409bb4;
                                }
                                v44 = v44;
                                if (((int)v67[24] & 0xf000) == 0xa000)
                                    goto LABEL_408af0;
                                if (!v65 || (long long)v67[16] <= 1)
                                {
                                    if ((len2->field_18 & 0xf000) == 0xa000)
                                        goto LABEL_409bce;
LABEL_409e99:
                                    if (*((long long *)&len2->padding_0[8]) != (long long)v67[8] || (v44 = v44, *((long long *)&len2->padding_0[0]) != *((long long *)v67)))
                                        goto LABEL_408af0;
                                    v54 = (char)idx[23];
                                    if (!(char)v54)
                                        goto LABEL_409bc9;
                                }
                                else
                                {
                                    /* unsupported instruction */ = (int)(&v2 - 576);
                                    v70 = sub_414e20(0xffffff9c, v50, a2, a3);
                                    v54 = v70;
                                    if ((char)v70)
                                    {
                                        if (((unsigned short)len2->field_18 & 0xf000) != 0xa000)
                                        {
LABEL_409bb4:
                                            if (((int)v67[24] & 0xf000) != 0xa000)
                                                goto LABEL_409e99;
                                        }
LABEL_409bc9:
                                        v69 = (char)idx[24];
LABEL_409bce:
                                        if (v69 && ((unsigned short)*((unsigned int *)((void*)&v41 + 24)) & 0xf000) == 0xa000 && (long long)v67[16] > 1 && (len2 = len2, /* unsupported instruction */ = (int)(&v2 - 576), len2 = len2, v54 = (unsigned long)canonicalize_file_name(v50), v54))
                                        {
                                            v71 = 0;
                                            v68 = (char)sub_414e20(0xffffff9c, v54, a2, a3) ^ 1;
                                            /* unsupported instruction */ = (int)(&v2 - 576);
                                            free(v54);
                                            v54 = v71;
                                            goto LABEL_40a173;
                                        }
LABEL_409be8:
                                        if ((char)idx[58] && (v44 = v44, ((short)(int)v67[24] & 0xf000) == 0xa000))
                                            goto LABEL_408af0;
                                        if ((int)idx[4] != 2)
                                            goto LABEL_407c99;
                                        if (((unsigned short)len2->field_18 & 0xf000) == 0xa000)
                                        {
                                            /* unsupported instruction */ = (int)(&v2 - 576);
                                            v44 = v44;
                                            if (!stat(v50, &v44))
                                                goto LABEL_409c8e;
                                            goto LABEL_408af0;
                                        }
                                        *((int128_t *)&v44) = *((int128_t *)&len2->padding_0[0]);
                                        *((int128_t *)&(&v44)[16]) = *((int128_t *)&len2->padding_0[16]);
                                        *((int128_t *)&(&v44)[32]) = *((int128_t *)&len2[1].padding_0[4]);
                                        *((int128_t *)&(&v44)[48]) = *((int128_t *)&len2[1].padding_0[20]);
                                        *((int128_t *)&(&v44)[64]) = *((int128_t *)&len2[2].padding_0[8]);
                                        *((int128_t *)&(&v44)[80]) = *((int128_t *)&len2[2].field_18);
                                        *((int128_t *)&(&v44)[96]) = *((int128_t *)&len2[3].padding_0[12]);
                                        *((int128_t *)&(&v44)[112]) = *((int128_t *)&len2[4].padding_0[0]);
                                        *((int128_t *)&(&v44)[1].st_mtime) = *((int128_t *)&len2[4].padding_0[16]);
LABEL_409c8e:
                                        if (((short)(int)v67[24] & 0xf000) != 0xa000)
                                        {
                                            *((int128_t *)&v43) = *((int128_t *)v67);
                                            *((int128_t *)&(&v43)[16]) = (int128_t)v67[16];
                                            *((int128_t *)&(&v43)[32]) = (int128_t)v67[32];
                                            *((int128_t *)&(&v43)[48]) = (int128_t)v67[48];
                                            *((int128_t *)&(&v43)[64]) = (int128_t)v67[64];
                                            *((int128_t *)&(&v43)[80]) = (int128_t)v67[80];
                                            *((int128_t *)&(&v43)[96]) = (int128_t)v67[96];
                                            *((int128_t *)&(&v43)[112]) = (int128_t)v67[112];
                                            *((int128_t *)&(&v43)[1].st_mtime) = (int128_t)v67[128];
                                        }
                                        /* unsupported instruction */ = (int)(&v2 - 576);
                                        if (fstatat(a2, a3, &v43, 0) || v44.st_ino != v43.st_ino || *((unsigned long long *)&v44) != *((unsigned long long *)&v43))
                                            goto LABEL_408af0;
                                        if (!(char)idx[23])
                                            goto LABEL_407c99;
                                        v54 = _INSERT(v54, 0, ((int)v67[24] & 0xf000) != 0xa000);
                                    }
                                    else
                                    {
                                        v68 = (char)idx[24] ^ 1;
LABEL_40a173:
                                        if (!v68)
                                        {
LABEL_407c99:
                                            sub_414310(1, 4, a1);
                                            sub_414310(0, 4, v50);
                                            v73 = "%s and %s are the same file";
                                            goto LABEL_407cd0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    else
                    {
                        v65 = 1;
                        v66 = 1;
LABEL_407c23:
                        v67 = &v42;
                        len2 = &v41;
                        if (((unsigned short)*((unsigned int *)((void*)&v41 + 24)) & 0xf000) != 0xa000 || !(v44 = v44, ((unsigned short)v42.field_18 & 0xf000) == 0xa000))
                            goto LABEL_407c47;
                        len2 = v66;
                        /* unsupported instruction */ = (int)(&v2 - 576);
                        v72 = sub_414e20(0xffffff9c, v50, a2, a3);
                        v54 = v72;
                        if ((char)v72)
                            goto LABEL_407c99;
                        if (!*((int *)idx) && len2)
                        {
                            v54 = 1;
                            v68 = (char)idx[24] ^ 1;
                            goto LABEL_40a173;
                        }
                    }
                }
            }
            else
            {
LABEL_408af0:
                v54 = 0;
            }
        }
        else
        {
            v51 = NULL;
            v54 = 0;
        }
        if (!(char)idx[59])
        {
            if ((char)idx[24])
                goto LABEL_40910b;
            if ((a4 & 0xf000) != 0x4000)
                goto LABEL_408c1d;
            goto LABEL_408c33;
        }
        if (((unsigned short)a4 & 0xf000) == 0x4000)
        {
            if ((char)idx[24])
                goto LABEL_40910b;
            goto LABEL_408c33;
        }
        /* unsupported instruction */ = (int)(&v2 - 576);
        if (sub_415b50(a2, a3, &v42, &v41) >= 0)
        {
            if (v24)
                *((char *)v24) = 1;
            v74 = sub_40bd30(a3, *((unsigned long long *)((void*)&v41 + 8)), *((unsigned long long *)&v41));
            if (!v74 || (char)sub_4065d0(0, a2, v74, a1, a2, a3, 1, (char)idx[60], *((unsigned int *)&v17)))
                goto LABEL_407d70;
            if ((char)idx[0x33] && setfscreatecon(0))
                goto LABEL_408bf7;
            goto LABEL_407cf0;
        }
        else
        {
            if (!(char)idx[24])
            {
LABEL_408c1d:
                if ((int)idx[8] != 2 && ((int)idx[8] != 3 || (/* unsupported instruction */ = (int)(&v2 - 576), (char)sub_4067a0(idx, a1, a2, a3, &v42))))
                    goto LABEL_408c33;
                goto LABEL_407d70;
            }
LABEL_40910b:
            v75 = (int)idx[8];
            if (v75 == 2)
                goto LABEL_4091b0;
            if (v75 != 3)
            {
                if (v75 != 4 || !(char)idx[61])
                    goto LABEL_408c33;
                v76 = v42.field_18;
                if (((unsigned short)v76 & 0xf000) != 0xa000)
                {
                    /* unsupported instruction */ = (int)(&v2 - 576);
                    if (!(char)sub_416bb0() && (/* unsupported instruction */ = (int)(&v2 - 576), (unsigned int)(unsigned long long)faccessat((unsigned long long)a2, a3, 2, 0x200)))
                        goto LABEL_40917c;
                    goto LABEL_408c33;
                }
                if ((char)v54)
                    goto LABEL_407d70;
LABEL_408c56:
                v77 = a4 & 0xf000;
                if (v77 != 0x4000)
                {
                    v78 = *((int *)idx);
                    if (!(char)v25)
                        goto LABEL_408ccc;
                    goto LABEL_408c7d;
                }
                if ((char)idx[24] && !(v78 = (unsigned long)(unsigned long long)(unsigned int)*((int *)idx), !(unsigned int)v78))
                {
                    if (!(char)v25)
                    {
LABEL_40982f:
                        if ((*((unsigned int *)((void*)&v41 + 24)) & 0xf000) == 0x4000)
                            goto LABEL_40a0b6;
                        goto LABEL_409847;
                    }
LABEL_408c7d:
                    if ((unsigned int)v78 != 3)
                    {
                        /* unsupported instruction */ = (int)(&v2 - 576);
                        if (!(char)sub_40e820((long long)idx[72], a3, &v42))
                        {
                            if (v77 != 0x4000 && !(v76 = v42.field_18, ((unsigned short)v76 & 0xf000) != 0x4000))
                                goto LABEL_409818;
                            goto LABEL_408cc9;
                        }
                        else
                        {
                            sub_414310(1, 4, v50);
                            sub_414310(0, 4, a1);
                            v73 = "will not overwrite just-created %s with %s";
                        }
                    }
                    else
                    {
                        if ((char)idx[24])
                            goto LABEL_40982f;
LABEL_409e1b:
                        /* unsupported instruction */ = (int)(&v2 - 576);
                        v80 = sub_40d970(v50);
                        if (*(v80) != 46)
                        {
LABEL_409883:
                            if ((v42.field_18 & 0xf000) != 0x4000)
                                goto LABEL_409899;
                            goto LABEL_408d80;
                        }
                        else
                        {
                            v79 = 0;
LABEL_40985e:
                            if (!v80[1 + (v80[1] == 46)] || v80[1 + (v80[1] == 46)] == 47)
                            {
LABEL_408ce2:
                                v54 = 0x11;
                                len2 = 0;
                                if (((unsigned short)v42.st_nlink & 0xf000) != 0x4000 && !v79 && ((char)idx[21] || (char)idx[49] && ((char)idx[48] && *((unsigned long long *)((void*)&v42 + 16)) > 1 || !(v54 = 0x11, len2 = 0, (int)idx[4] != 2 || ((unsigned short)*((unsigned int *)((void*)&v41 + 24)) & 0xf000) == 0x8000))))
                                {
                                    /* unsupported instruction */ = (int)(&v2 - 576);
                                    if (unlinkat(a2, a3, 0))
                                    {
                                        /* unsupported instruction */ = (int)(&v2 - 576);
                                        v81 = __errno_location();
                                        if (*(v81) != 2)
                                        {
                                            sub_414440(4, a1);
                                            v82 = dcgettext(NULL, "cannot remove %s", 5);
                                            error(0, *(v81), v82);
                                            return _INSERT(v78, 0, (v42.st_nlink & 0xf000) == 0x4000 | v79);
                                        }
                                    }
                                    *((char *)&v26) = (char)idx[60];
                                    if (!(char)idx[60])
                                    {
                                        *((char *)&v26) = 1;
                                        goto LABEL_408d80;
                                    }
                                    else
                                    {
                                        v54 = 0x11;
                                        sub_414440(4, a1);
                                        /* unsupported instruction */ = (int)(&v2 - 576);
                                        __printf_chk(1, dcgettext(NULL, "removed %s\n", 5));
                                        len2 = 0;
                                        goto LABEL_407892;
                                    }
                                }
                            }
                            else
                            {
                                if (!v79)
                                    goto LABEL_409883;
LABEL_409899:
                                if ((unsigned int)v78 != 3)
                                {
                                    v26 = strlen(v80);
                                    v16 = sub_40d970(a3);
                                    len2 = strlen(v16);
                                    if (v26 == strlen(g_423580) + len2)
                                    {
                                        v26 = len2;
                                        if (!memcmp(v80, v16, v26) && !strcmp(&v80[v26], g_423580))
                                        {
                                            v84 = a3;
                                            len = strlen(v84);
                                            v86 = sub_406110(v84, &v84[len], g_423580);
                                            v87 = fstatat(a2, v86, &v44, 0);
                                            free(v86);
                                            if (!v87 && *((unsigned long long *)((void*)&v41 + 8)) == *((unsigned long long *)((void*)&v44 + 8)) && *((unsigned long long *)&v41) == *((unsigned long long *)&v44))
                                            {
                                                v88 = (!(char)idx[24] ? dcgettext(NULL, "backing up %s might destroy source;  %s not copied", 5) : dcgettext(NULL, "backing up %s might destroy source;  %s not moved", 5));
                                                sub_414310(1, 4, v50);
                                                sub_414310(0, 4, a1);
                                                error(0, 0, v88);
                                                return 0;
                                            }
                                        }
                                    }
                                }
                                ptr = sub_40d840(a2, a3, *((int *)idx));
                                if (ptr)
                                {
                                    v90 = a3 - a1;
                                    len1 = strlen(ptr);
                                    v92 = v90 + len1 + 24;
                                    i = &v2 - (v92 & 0xfffffffffffff000) - 584;
                                    if (&v2 - 584 != i)
                                    {
                                        do
                                        { } while (&v0 != i);
                                    }
                                    v94 = (unsigned int)v92 & 0xfffffff0 & 0xfff;
                                    v26 = len1 + 1;
                                    v54 = 0x11;
                                    len2 = (unsigned long long)(v237 + 15 & 0xfffffffffffffff0);
                                    memcpy(mempcpy(len2, a1, v90), ptr, v26);
                                    free(ptr);
                                    *((char *)&v26) = 1;
                                    goto LABEL_407892;
                                }
                                else
                                {
                                    /* unsupported instruction */ = (int)(&v2 - 576);
                                    v96 = __errno_location();
                                    if (*(v96) != 2)
                                    {
                                        sub_414440(4, a1);
                                        v234 = dcgettext(NULL, "cannot backup %s", 5);
                                        error(0, *(v96), v234);
                                        return 0;
                                    }
                                    *((char *)&v26) = 1;
                                    v54 = 0x11;
                                    len2 = 0;
                                    goto LABEL_407892;
                                }
                            }
                        }
                    }
                }
                else
                {
                    sub_414310(1, 4, v50);
                    sub_414310(0, 4, a1);
                    v73 = "cannot overwrite non-directory %s with directory %s";
                }
LABEL_407cd0:
                error(0, 0, dcgettext(NULL, v73, 5));
                goto LABEL_407cf0;
            }
            else
            {
LABEL_40917c:
                /* unsupported instruction */ = (int)(&v2 - 576);
                if (!(char)sub_4067a0(idx, a1, a2, a3, &v42))
                    goto LABEL_4091b0;
LABEL_408c33:
                if (!(char)v54)
                {
                    v76 = v42.field_18;
                    if ((v76 & 0xf000) != 0x4000)
                        goto LABEL_408c56;
                    if (((unsigned short)a4 & 0xf000) != 0x4000)
                    {
LABEL_409818:
                        if (!(char)idx[24] || !(v78 = (unsigned long)(unsigned long long)(unsigned int)*((int *)idx), (unsigned int)v78))
                        {
                            sub_414440(4, a1);
                            error(0, 0, dcgettext(NULL, "cannot overwrite directory %s with non-directory", 5));
                            return 0;
                        }
                        goto LABEL_40982f;
                    }
LABEL_408cc9:
                    v78 = *((int *)idx);
LABEL_408ccc:
                    v79 = (char)idx[24];
                    if (!v79)
                    {
                        if (!(unsigned int)v78)
                            goto LABEL_408ce2;
                        goto LABEL_409e1b;
                    }
                    if ((*((unsigned int *)((void*)&v41 + 24)) & 0xf000) != 0x4000)
                        goto LABEL_409e4c;
                    v76 = v42.field_18;
LABEL_40a0b6:
                    if (((unsigned short)v76 & 0xf000) == 0x4000)
                    {
LABEL_409e4c:
                        if ((unsigned int)v78)
                            goto LABEL_409847;
LABEL_408d80:
                        len2 = 0;
                        v54 = 0x11;
LABEL_407892:
                        if ((char)v25)
                            goto LABEL_407ad0;
                        goto LABEL_40789f;
                    }
                    else if ((unsigned int)v78)
                    {
LABEL_409847:
                        /* unsupported instruction */ = (int)(&v2 - 576);
                        v80 = sub_40d970(v50);
                        v79 = 1;
                        if (*(v80) == 46)
                            goto LABEL_40985e;
                        goto LABEL_409899;
                    }
                    else
                    {
                        sub_4147c0(0, 3, a1);
                        sub_4147c0(0, 3, v50);
                        v73 = "cannot move directory onto non-directory: %s -> %s";
                        goto LABEL_407cd0;
                    }
                }
            }
        }
    }
    len2 = 0;
    v51 = NULL;
LABEL_407ad0:
    if (!(long long)idx[72] || !(v97 = (unsigned int)(char)(char)idx[24], !(char)v97))
    {
LABEL_40789f:
        if (!(char)idx[60] || (char)idx[24])
            goto LABEL_4078b1;
        goto LABEL_408258;
    }
    if (!*((int *)idx))
    {
        v98 = &v42;
        if (*((char *)&v51))
        {
LABEL_407b03:
            if (((unsigned short)v98->field_18 & 0xf000) == 0xa000)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if ((char)sub_40e820((long long)idx[72], a3))
                {
                    sub_414310(1, 4, a1);
                    sub_414310(0, 4, v50);
                    error(0, 0, dcgettext(NULL, "will not copy %s through just-created symlink %s", 5));
                    return v97;
                }
            }
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (!fstatat(a2, a3, &v44, 0x100))
            {
                v98 = &v44;
                goto LABEL_407b03;
            }
        }
        goto LABEL_40789f;
    }
    if ((char)idx[60])
    {
LABEL_408258:
        if (((unsigned short)a4 & 0xf000) != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            sub_406510(v50, a1, len2);
        }
LABEL_4078b1:
        if ((unsigned int)v54)
            goto LABEL_4078ba;
        if ((char)idx[24])
        {
LABEL_407d27:
            if ((char)idx[60])
            {
                __printf_chk(1, dcgettext(NULL, "renamed ", 5));
                sub_406510(v50, a1, len2);
            }
            if ((long long)idx[40])
                sub_407510(a1, 1, idx);
            if (v24)
                *((char *)v24) = 1;
            if ((char)v25 && !(char)idx[63])
            {
                sub_40e790((long long)idx[72], a3, &v41);
                return (char)v25;
            }
        }
        else
        {
LABEL_407d98:
            v110 = (char)v26;
            v100 = 0;
            goto LABEL_407da3;
        }
        goto LABEL_407d70;
    }
    if (!(unsigned int)v54)
        goto LABEL_407d98;
LABEL_4078ba:
    if ((char)idx[56] && ((unsigned short)a4 & 0xf000) == 0x4000)
    {
        if ((char)v25)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v100 = sub_40bd30(a3, *((unsigned long long *)((void*)&v41 + 8)));
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v100 = sub_40bcd0(*((unsigned long long *)((void*)&v41 + 8)), *((unsigned long long *)&v41), *((unsigned long long *)&v41));
        }
        if (v100)
            goto LABEL_40790d;
        goto LABEL_407970;
    }
    v101 = (char)idx[24];
    if (v101)
    {
        if (*((unsigned long long *)((void*)&v41 + 16)) != 1 && (!(char)idx[48] || (char)idx[23]))
        {
            v100 = 0;
            goto LABEL_407a20;
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v100 = sub_40bcd0(*((unsigned long long *)((void*)&v41 + 8)), *((unsigned long long *)&v41));
LABEL_408f3d:
        if (!v100)
        {
LABEL_407970:
            v101 = (char)idx[24];
            goto LABEL_40823e;
        }
        if (((unsigned short)a4 & 0xf000) != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = *((unsigned int *)&v17);
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = (char)idx[60];
            *((unsigned long *)(/* unsupported instruction */ - 16)) = 1;
            if (!(char)sub_4065d0(0, a2, v100, a1, a2, a3, v49, *((unsigned int *)&a6), *((char *)&a7)))
                goto LABEL_408f9c;
            goto LABEL_407d70;
        }
LABEL_40790d:
        if ((char)sub_414e20(0xffffff9c, v50, a2, v100))
        {
            sub_414310(1, 4, g_423568);
            sub_414310(0, 4, g_423570);
            error(0, 0, dcgettext(NULL, "cannot copy a directory, %s, into itself, %s", 5));
            *(v19) = 1;
            goto LABEL_408f9c;
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 16;
        if (!(char)sub_414e20(a2, a3, a2, v100))
        {
            if ((int)idx[4] == 4 || (int)idx[4] == 3 && (char)v25)
                goto LABEL_407970;
            v102 = a1;
            v103 = sub_406110(v102, a3, v100);
            sub_414310(1, 4, v103);
            sub_414310(0, 4, v102);
            error(0, 0, dcgettext(NULL, "will not create hard link %s to directory %s", 5));
            free(v103);
LABEL_408f9c:
            if ((char)idx[0x33] && setfscreatecon(0))
                goto LABEL_408bf7;
LABEL_40866c:
            if (len2)
            {
                if (renameat(a2, a3 - a1 + len2, a2, a3))
                {
                    sub_414440(4, a1);
                    v226 = "cannot un-backup %s";
LABEL_408541:
                    v232 = dcgettext(NULL, v226, 5);
                    v233 = __errno_location();
                    error(0, *(v233), v232);
                    goto LABEL_407cf0;
                }
                else if ((char)idx[60])
                {
                    sub_414310(1, 4, a1);
                    sub_414310(0, 4, len2);
                    __printf_chk(1, dcgettext(NULL, "%s -> %s (unbackup)\n", 5));
                    goto LABEL_407cf0;
                }
            }
        }
        else
        {
            sub_414440(4, g_423570);
            error(0, 0, dcgettext(NULL, "warning: source directory %s specified more than once", 5));
            if ((char)idx[24])
            {
LABEL_4091b0:
                if (v24)
                    *((char *)v24) = 1;
            }
LABEL_407d70:
            return 1;
        }
    }
    if (!(char)idx[48])
        goto LABEL_407d98;
    if (!(char)idx[23])
    {
        if (*((unsigned long long *)((void*)&v41 + 16)) > 1 || (char)v25 && (int)idx[4] == 3 || (v100 = 0, (int)idx[4] == 4))
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v100 = sub_40bd30(a3, *((unsigned long long *)((void*)&v41 + 8)), *((unsigned long long *)&v41));
            goto LABEL_408f3d;
        }
LABEL_40823e:
        if (!v101)
            goto LABEL_408246;
LABEL_407a20:
        if ((unsigned int)v54 == 0x11 && (!renameat(0xffffff9c, v50, a2, a3) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 16), v54 = (unsigned long)(unsigned long long)(unsigned int)*(__errno_location()), !(unsigned int)v54)))
            goto LABEL_407d27;
        if ((unsigned int)v54 == 22)
        {
            sub_414310(1, 4, g_423568);
            sub_414310(0, 4, g_423570);
            error(0, 0, dcgettext(NULL, "cannot move %s to a subdirectory of itself, %s", 5));
            *(v19) = 1;
            return 1;
        }
        if ((unsigned int)v54 != 18)
        {
            sub_414310(1, 4, a1);
            sub_414310(0, 4, v50);
            v108 = v54;
            v109 = dcgettext(NULL, "cannot move %s to %s", 5);
            goto LABEL_408e3a;
        }
        v105 = a4 & 0xf000;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        if (!unlinkat(a2, a3, (v105 == 0x4000) * 0x200) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v106 = __errno_location(), *(v106) == 2))
        {
            *((char *)&v26) = v105 != 0x4000 & (char)idx[60];
            if (!(v105 != 0x4000 & (char)idx[60]))
            {
                *((char *)&v26) = 1;
                v110 = 1;
                goto LABEL_407da3;
            }
            else
            {
                v110 = 1;
                __printf_chk(1, dcgettext(NULL, "copied ", 5));
                /* unsupported instruction */ = /* unsupported instruction */ - 24;
                sub_406510(v50, a1, len2);
                goto LABEL_407da3;
            }
        }
        else
        {
            sub_414310(1, 4, a1);
            sub_414310(0, 4, v50);
            v107 = dcgettext(NULL, "inter-device move failed: %s to %s; unable to remove target", 5);
            v108 = *(v106);
            v109 = v107;
LABEL_408e3a:
            error(0, v108, v109);
            sub_40bc60(*((unsigned long long *)((void*)&v41 + 8)), *((unsigned long long *)&v41));
            goto LABEL_407cf0;
        }
    }
    v100 = 0;
LABEL_408246:
    v110 = (char)v26;
LABEL_407da3:
    v116 = a4 & 0xfff;
    if ((char)idx[57])
        v116 = (int)idx[16] & 0xfff;
    if (!(char)idx[29])
    {
        v117 = a4 & 0xf000;
        *((unsigned int *)&v24) = v117;
        if (v117 != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v16 = 0;
            if ((char)sub_4072a0(v50, a1, a4, v110, idx))
                goto LABEL_407e7a;
            goto LABEL_407cf0;
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        if (!(char)sub_4072a0(v50, a1, a4, v110, idx))
            goto LABEL_407cf0;
        v16 = v116 & 18;
LABEL_40833e:
        v206 = *((unsigned long long *)((void*)&v41 + 8));
        v207 = *((unsigned long long *)&v41);
        v208 = v21;
        if (v208)
        {
            do
            {
                if (v208[1] == v206 && v208[2] == v207)
                {
                    sub_414440(4, v50);
                    v121 = "cannot copy cyclic symbolic link %s";
                    goto LABEL_408394;
                }
            } while ((v208 = (struct_2 **)*(v208), v208));
        }
        if (!1)
            goto LABEL_0x408718;
        /* unsupported instruction */ = /* unsupported instruction */ - 32;
        index = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
        index->field_0 = v21;
        index->field_8 = v206;
        index->field_10 = v207;
        if ((char)v26 || ((unsigned short)v42.field_18 & 0xf000) != 0x4000)
        {
            if (!mkdirat(a2, a3, ~(v16) & v116))
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 16;
                if (!fstatat(a2, a3, &v42, 0x100))
                {
                    v210 = v42.field_18;
                    v10 = 0;
                    if (((unsigned short)v210 & 448) == 448 || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v8 = v210, v10 = 1, !(unsigned int)(unsigned long long)fchmodat((unsigned long long)a2, a3, (unsigned long long)(v210 | 448), 0x100)))
                    {
                        if (!*(v22))
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            sub_40bd30(a3, *((unsigned long long *)((void*)&v42 + 8)), *((unsigned long long *)&v42));
                            *(v22) = 1;
                        }
                        if ((char)idx[60])
                        {
                            if ((char)idx[24])
                            {
                                v100 = sub_414440(4, a1);
                                /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                __printf_chk(1, dcgettext(NULL, "created directory %s\n", 5));
                                goto LABEL_408870;
                            }
                            else
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                sub_406510(v50, a1, 0);
                                goto LABEL_408870;
                            }
                        }
                    }
                    else
                    {
                        sub_414440(4, a1);
                        v126 = "setting permissions for %s";
                        goto LABEL_408638;
                    }
                }
                else
                {
                    sub_414440(4, a1);
                    v126 = "cannot stat %s";
                    goto LABEL_408638;
                }
            }
            else
            {
                sub_414440(4, a1);
                v126 = "cannot create directory %s";
                goto LABEL_408638;
            }
        }
        if (!(long long)idx[40] && !(char)idx[0x33] || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (char)sub_407510(a1, 0, idx) || !(char)idx[52]))
        {
            v10 = 0;
            v16 = 0;
LABEL_408870:
            v205 = _INSERT(v100, 0, v18 & (char)idx[28]);
            if (!(v18 & (char)idx[28]) || *((long long *)&v18) == *((unsigned long long *)&v41))
            {
                v30 = (int)*((int128_t *)idx);
                v32 = (int)(int128_t)idx[16];
                v36 = (long long)idx[80];
                v33 = (int)(int128_t)idx[32];
                v34 = (int)(int128_t)idx[48];
                v35 = (int)(int128_t)idx[64];
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v11 = sub_415300(v50, 2);
                if (v11)
                {
                    if ((int)idx[4] == 3)
                        v31 = 2;
                    if (*(v11))
                    {
                        v20 = a3 - a1;
                        v212 = v11;
                        v15 = &v29;
                        v18 = &v27;
                        v17 = &v28;
                        v9 = idx;
                        v21 = 0;
                        v14 = &v30;
                        v13 = v110;
                        v24 = 1;
                        len2 = v50;
                        v19 = &v41;
                        do
                        {
                            v213 = sub_40ea10(len2, v212, 0);
                            /* unsupported instruction */ = /* unsupported instruction */ - 16;
                            v214 = sub_40ea10(a1, v212, 0);
                            v215 = *(v22);
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((char **)(/* unsupported instruction */ - 8)) = v15;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((char **)(/* unsupported instruction */ - 8)) = v18;
                            v28 = v215;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((char **)(/* unsupported instruction */ - 8)) = v17;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((char **)(/* unsupported instruction */ - 8)) = v14;
                            *((struct_4 **)(/* unsupported instruction */ - 8)) = index;
                            v216 = copy_internal(v213, v214, a2, v214 + v20, v13, v19, v49, a6, *((unsigned int *)&a7), a8, a9, a10);
                            *(v19) = *(v19) | v27;
                            v24 &= v216;
                            free(v214);
                            /* unsupported instruction */ = /* unsupported instruction */ + 16;
                            free(v213);
                        } while (!v27 && (v21 |= v28, /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v212 = v212 + strlen(v212) + 1, *(v212)));
                        v205 = v24;
                        v50 = len2;
                        idx = v9;
                    }
                    else
                    {
                        v21 = 0;
                        v205 = 1;
                    }
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    free(v11);
                    *(v22) = v21;
                }
                else
                {
                    sub_414440(4, v50);
                    v217 = dcgettext(NULL, "cannot access %s", 5);
                    v218 = __errno_location();
                    v205 = 0;
                    /* unsupported instruction */ = /* unsupported instruction */ - 32;
                    error(0, *(v218), v217);
                }
            }
            if ((char)v25)
            {
                v123 = 0;
                v119 = 0;
                v201 = 0;
                v20 = v10;
LABEL_407f59:
                if ((long long)idx[72])
                {
                    v24 = v123;
                    v25 = &v44;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v123 = v24;
                    if (!fstatat(a2, a3, &v44, 0x100))
                    {
                        v25 = v123;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        sub_40e790((long long)idx[72], a3);
                        v123 = v25;
                    }
                }
LABEL_407fc0:
                if ((char)idx[23] && v201)
                    return v205;
                if (v123)
                    return v205;
                goto LABEL_407fd8;
            }
            v119 = 0;
            v20 = v10;
LABEL_407fd8:
            if ((char)idx[31])
            {
                v44 = *((unsigned long long *)((void*)&v41 + 72));
                v45 = *((unsigned long long *)((void*)&v41 + 80));
                v46 = *((unsigned long long *)((void*)&v41 + 88));
                v47 = *((unsigned long long *)((void*)&v41 + 96));
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (!utimensat(a2, a3, &v44, v119 * 0x100))
                    goto LABEL_408098;
                v25 = sub_414440(4, a1);
                v219 = dcgettext(NULL, "preserving times for %s", 5);
                v220 = __errno_location();
                /* unsupported instruction */ = /* unsupported instruction */ - 32;
                error(0, *(v220), v219);
                if (!(char)idx[50])
                    goto LABEL_408098;
            }
            else
            {
LABEL_408098:
                if (!v119)
                {
                    if ((char)idx[29] && (v221 = (unsigned long)(unsigned long long)*((unsigned int *)((void*)&v41 + 28)), v222 = (unsigned long)(unsigned long long)*((unsigned int *)((void*)&v41 + 32)), (char)v26 || *((unsigned int *)((void*)&v42 + 28)) != (unsigned int)v221 || *((unsigned int *)((void*)&v42 + 32)) != (unsigned int)v222))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((struct struct_0 **)(/* unsupported instruction */ - 8)) = &v42;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((unsigned long long *)(/* unsupported instruction */ - 8)) = (char)v26;
                        *((unsigned long *)(/* unsupported instruction */ - 8)) = v222;
                        *((unsigned long *)(/* unsupported instruction */ - 16)) = v221;
                        v223 = sub_407080(idx, a1, a2, a3, 0xffffffff, v41.st_nlink, v49, a6, a7, a8);
                        if (v223 != 0xffffffff && !v223)
                        {
                            a4 &= 0xfffff1ff;
                            goto LABEL_408120;
                        }
                    }
                    else
                    {
LABEL_408120:
                        if (!(char)idx[53] || sub_406220(v50, 0xffffffff, a1, 0xffffffff, idx) || !(char)idx[54])
                        {
                            if (!(0xff0000000000ff & (long long)idx[24]))
                            {
                                if ((char)idx[57])
                                    goto LABEL_40a366;
                                if (!((char)v26 & (char)idx[32]))
                                {
                                    v16 = v16;
                                    if (v16)
                                    {
                                        v224 = g_423010;
                                        if (v224 == 0xffffffff)
                                        {
                                            v224 = umask(0);
                                            g_423010 = v224;
                                            umask(v224);
                                        }
                                        v225 = ~(v224);
                                        v16 &= v225;
                                        if (v16 & v225)
                                        {
                                            if (v20 != 1)
                                            {
                                                if ((char)v26 && fstatat(a2, a3, &v42, 0x100))
                                                {
LABEL_408528:
                                                    sub_414440(4);
                                                    v226 = "cannot stat %s";
                                                    goto LABEL_408541;
                                                }
                                                else
                                                {
                                                    v8 = v42.st_nlink;
                                                    if (!(~(v8) & v16))
                                                        return v205;
                                                }
                                            }
LABEL_408176:
                                            if (!fchmodat(a2, a3, v8 | v16, 0x100))
                                                return v205;
                                            sub_414440(4, a1);
                                            v227 = dcgettext(NULL, "preserving permissions for %s", 5);
                                            v228 = __errno_location();
                                            error(0, *(v228), v227);
                                            goto LABEL_4081e1;
                                        }
                                    }
                                    if (!v20)
                                        return v205;
                                    goto LABEL_408176;
                                }
                                else
                                {
                                    if (g_423010 == 0xffffffff)
                                    {
                                        g_423010 = umask(0);
                                        umask(g_423010);
                                    }
LABEL_40a366:
                                    if (!sub_40c8e0(a1, 0xffffffff))
                                        return v205;
                                }
                            }
                            else
                            {
                                if (!sub_40c830(v50, 0xffffffff, a1, 0xffffffff, a4))
                                    return v205;
LABEL_4081e1:
                                if (!(char)idx[50])
                                    return v205;
                            }
                        }
                    }
                }
                else
                {
                    if (!(char)idx[53])
                    {
                        return v205;
                    }
                    else if (sub_406220(v50, 0xffffffff, a1, 0xffffffff, idx))
                    {
                        return v205;
                    }
                    else if (!(char)idx[54])
                    {
                        return v205;
                    }
                }
            }
LABEL_407cf0:
            return 0;
        }
        goto LABEL_408658;
    }
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    if (!(char)sub_4072a0(v50, a1, a4, v110, idx))
        goto LABEL_407cf0;
    v16 = v116 & 63;
    v118 = a4 & 0xf000;
    *((unsigned int *)&v24) = v118;
    if (v118 == 0x4000)
        goto LABEL_40833e;
LABEL_407e7a:
    v119 = (char)idx[58];
    if (v119)
    {
        if (*(v50) != 47)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            ptr1 = sub_40e480(a3);
            if (a2 == 0xffffff9c && !(v22 = ".", /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (int)(long long)strcmp(".", ptr1)) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (int)(long long)stat(".", &v43) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (unsigned int)(unsigned long long)fstatat((unsigned long long)a2, ptr1, &v44, 0) || v43.st_ino == *((unsigned long long *)((void*)&v44 + 8)) && *((unsigned long long *)(void*)&v43) == *((unsigned long long *)(void*)&v44))))
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                free(ptr1);
            }
            else
            {
                free(ptr1);
                sub_4147c0(0, 3, a1);
                v121 = "%s: can make relative symbolic links only in current directory";
                goto LABEL_408394;
            }
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v122 = sub_40c000(v50, a2, a3, (char)idx[22], 0xffffffff);
        v123 = 0;
        if (v122 <= 0)
            goto LABEL_407f17;
        sub_414310(1, 4, v50);
        sub_414310(0, 4, a1);
        error(0, v122, dcgettext(NULL, "cannot create symbolic link %s to %s", 5));
        goto LABEL_408658;
    }
    if ((char)idx[23])
    {
        v200 = 1;
        if (!(char)idx[22])
            v200 = (int)idx[8] == 3;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        *((unsigned long long *)(/* unsupported instruction */ - 8)) = *((unsigned int *)&v17);
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
        *((unsigned long long *)(/* unsupported instruction */ - 8)) = v200;
        /* unsupported instruction */ = /* unsupported instruction */ + 16;
        v123 = 0;
        if ((char)sub_4065d0(v50, 0xffffff9c, v50, a1, a2, a3, v49, *((unsigned int *)&a6), *((char *)&a7)))
            goto LABEL_407f17;
        goto LABEL_408658;
    }
    v125 = v24;
    if (v125 != 0x8000 && !(v125 != 0xa000 & (char)idx[20]))
    {
        *((char *)&v22) = v125 != 0xa000 & (char)idx[20];
        if ((unsigned int)v24 == 0x1000)
        {
            v133 = ~(v16) & a4;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v123 = *((char *)&v22);
            if (mknodat(a2, a3, v133, 0) && (v134 = (long long)_INSERT((unsigned long long)v133, 1, (char)v133 & 239), /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v123 = *((char *)(void*)&v22), (unsigned int)(unsigned long long)mkfifoat((unsigned long long)a2, a3, _INSERT((unsigned long long)v133, 1, (char)v133 & 239))))
            {
                sub_414440(4, a1);
                v126 = "cannot create fifo %s";
                goto LABEL_408638;
            }
        }
        if (_INSERT((unsigned int)v24, 1, (char)v24 & 191) == 0x2000 || (unsigned int)v24 == 0xc000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v123 = *((char *)&v22);
            if (mknodat(a2, a3, ~(v16) & a4, *((unsigned long long *)((void*)&v41 + 40))))
            {
                sub_414440(4, a1);
                v126 = "cannot create special file %s";
                goto LABEL_408638;
            }
        }
        if ((unsigned int)v24 == 0xa000)
        {
            ptr2 = sub_40c960(v50, *((unsigned long long *)((void*)&v41 + 48)));
            if (ptr2)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 16;
                v128 = sub_40c000(ptr2, a2, a3, (char)idx[22], 0xffffffff);
                if (v128 <= 0)
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    free(ptr2);
                    goto LABEL_40a90a;
                }
                if ((char)idx[59] != 1 || (char)v26 || ((unsigned short)v42.field_18 & 0xf000) != 0xa000 || (v22 = (long long)*((unsigned long long *)((void*)&v42 + 48)), v22 != strlen(ptr2) || (v129 = (char *)(unsigned long long)sub_40caf0((unsigned long long)a2, a3), !v129)))
                {
LABEL_409f8f:
                    free(ptr2);
                    sub_414440(4, a1);
                    error(0, v128, dcgettext(NULL, "cannot create symbolic link %s", 5));
                    goto LABEL_408658;
                }
                else
                {
                    v22 = v129;
                    if (strcmp(v129, ptr2))
                    {
                        free(v22);
                        goto LABEL_409f8f;
                    }
                    free(v22);
                    /* unsupported instruction */ = /* unsupported instruction */ - 40;
                    free(ptr2);
LABEL_40a90a:
                    if ((char)idx[0x33] && (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (unsigned int)(unsigned long long)setfscreatecon(0)))
                    {
LABEL_408bf7:
                        sub_406770(); /* do not return */
                    }
                    v123 = (char)idx[29];
                    v119 = 1;
                    if (!v123)
                    {
LABEL_407f17:
                        v201 = (unsigned int)v24 != 0x4000;
                        if ((char)v26 || !(v202 = (char)idx[20] ^ 1, v203 = v202 & v201, v202 & v201))
                        {
LABEL_407f3f:
                            v204 = v25;
                            v205 = 1;
                            if (!(char)v204)
                                goto LABEL_407fc0;
                            v20 = 0;
                            v205 = v204;
                            goto LABEL_407f59;
                        }
                        else if ((long long)idx[40] || (char)idx[0x33])
                        {
                            *((char *)&v24) = v123;
                            *((char *)&v22) = v203;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            v123 = v24;
                            v201 = sub_407510(a1, 0, idx);
                            if (v201)
                                goto LABEL_407f3f;
                            *((char *)&v26) = (char)idx[52];
                            if (!(char)idx[52])
                            {
                                v201 = v22;
                                goto LABEL_407f3f;
                            }
                        }
                        else
                        {
                            v201 = v203;
                            goto LABEL_407f3f;
                        }
                    }
                    else
                    {
                        *((char *)&v22) = v123;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        if (fchownat(a2, a3, v41.st_uid, v41.st_gid, 0x100))
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            v123 = 0;
                            v119 = sub_4076c0(idx);
                            if (v119)
                                goto LABEL_407f17;
                            v131 = dcgettext(NULL, "failed to preserve ownership for %s", 5);
                            v132 = __errno_location();
                            /* unsupported instruction */ = /* unsupported instruction */ - 24;
                            error(0, *(v132), v131);
                            if (!(char)idx[50])
                            {
                                v119 = *((char *)&v22);
                                v123 = 0;
                                goto LABEL_407f17;
                            }
                        }
                        else
                        {
                            v119 = *((char *)&v22);
                            v123 = 0;
                            goto LABEL_407f17;
                        }
                    }
                }
            }
            else
            {
                sub_414440(4, v50);
                v126 = "cannot read symbolic link %s";
LABEL_408638:
                v229 = dcgettext(NULL, v126, 5);
                v230 = __errno_location();
                error(0, *(v230), v229);
            }
        }
        else
        {
            sub_414440(4, v50);
            v121 = "%s has unknown file type";
LABEL_408394:
            error(0, 0, dcgettext(NULL, v121, 5));
            goto LABEL_408658;
        }
    }
    v29 = NULL;
    *((unsigned int *)&v19) = v41.st_nlink;
    *((char *)&v22) = (char)idx[49];
    *((char *)&v21) = (char)idx[53];
    v135 = sub_40e620(v50, ((int)idx[4] == 2) * 0x20000);
    if (v135 < 0)
    {
        sub_414440(4, v50);
        v126 = "cannot open %s for reading";
        goto LABEL_408638;
    }
    /* unsupported instruction */ = /* unsupported instruction */ - 16;
    if (fstat(v135, &v44))
    {
        v22 = sub_414440(4, v50);
        /* unsupported instruction */ = /* unsupported instruction */ - 24;
        v143 = dcgettext(NULL, "cannot fstat %s", 5);
        v144 = *(__errno_location());
        goto LABEL_40a000;
    }
    if (*((unsigned long long *)((void*)&v41 + 8)) != v44.st_ino || *((unsigned long long *)&v41) != *((unsigned long long *)&v44))
    {
        sub_414440(4, v50);
        /* unsupported instruction */ = /* unsupported instruction */ - 16;
        v144 = 0;
        v143 = dcgettext(NULL, "skipping file %s, as it was replaced while being copied", 5);
        goto LABEL_40a000;
    }
    v11 = v116 & 0x1ff;
    if (!(char)v26)
    {
        v137 = sub_4125a0(a2, a3, (-(*((char *)&v22) < 1) & 0xfffffe00) + 513);
        /* unsupported instruction */ = /* unsupported instruction */ - 16;
        v138 = __errno_location();
        v139 = *(v138);
        if (v137 >= 0)
        {
            if (!(long long)idx[40] && !(char)idx[0x33] || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v140 = (char)(unsigned long long)sub_407510(a1, 0, idx), v140 || !(char)idx[52]))
            {
                v4 = 0;
                v9 = 0;
                goto LABEL_40954f;
            }
            else
            {
                *((char *)&v26) = 0;
                goto LABEL_4096e5;
            }
        }
        if (v139 != 2)
        {
            v141 = (char)idx[22];
            if (!v141)
            {
LABEL_40a58d:
                *((char *)&v26) = v141;
                goto LABEL_40a593;
            }
            v18 = v138;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (!unlinkat(a2, a3, 0) && (char)idx[60])
            {
                sub_414440(4, a1);
                /* unsupported instruction */ = /* unsupported instruction */ - 24;
                __printf_chk(1, dcgettext(NULL, "removed %s\n", 5));
                goto LABEL_40ada1;
            }
            if (*(v18) == 2)
                goto LABEL_40ada1;
            v22 = v18;
            sub_414440(4, a1);
            /* unsupported instruction */ = /* unsupported instruction */ - 16;
            v143 = dcgettext(NULL, "cannot remove %s", 5);
            v144 = *(v22);
LABEL_40a000:
            v146 = NULL;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            error(0, v144, v143);
            v145 = 0;
            goto LABEL_409710;
        }
LABEL_40ada1:
        if (!(long long)idx[40] || !(/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v145 = (char)(unsigned long long)sub_4072a0(v50, a1, (unsigned long long)v11, 1, idx), !v145))
            goto LABEL_40a500;
        *((char *)&v26) = 0;
        v146 = NULL;
LABEL_409710:
        *((char *)&v22) = v145;
        if (close(v135) >= 0)
        {
            *((char *)&v22) = v22;
            v119 = 0;
            /* unsupported instruction */ = /* unsupported instruction */ - 16;
            free(v146);
            v123 = *((char *)&v22);
            if (v123)
                goto LABEL_407f17;
        }
        else
        {
            sub_414440(4, v50);
            v198 = dcgettext(NULL, "failed to close %s", 5);
            v199 = __errno_location();
            error(0, *(v199), v198);
            free(v146);
        }
LABEL_408658:
        if ((!(char)idx[0x33] || !setfscreatecon(0)) && !v100)
        {
            sub_40bc60(*((unsigned long long *)((void*)&v41 + 8)), *((unsigned long long *)&v41));
            goto LABEL_40866c;
        }
    }
LABEL_40a500:
    v147 = ~(v16) & v11;
    if ((char)v21 && !(char)idx[27])
        v147 |= 128;
    *((unsigned int *)&v17) = v147;
    v137 = sub_4125a0(a2, a3, 193, v147 & 0xffffffff);
    /* unsupported instruction */ = /* unsupported instruction */ - 16;
    v148 = __errno_location();
    v149 = *((unsigned int *)&v17);
    v150 = *(v148);
    v18 = v148;
    v151 = v137 >> 31;
    *((char *)&v26) = v150 == 0x11 & v151;
    if (v150 == 0x11 & v151)
    {
        v141 = (char)idx[24];
        v139 = 0x11;
        if (v141)
            goto LABEL_40a58d;
        *((unsigned int *)&v15) = 0x11;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v149 = *((unsigned int *)&v17);
        v150 = v15;
        if (readlinkat(a2, a3, &v48, 1) < 0)
            goto LABEL_40aa90;
        if ((char)idx[62])
        {
            *((unsigned int *)&v26) = v149;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v152 = v26;
            v137 = sub_4125a0(a2, a3, 65, v149);
            v150 = *(v18);
            v149 = v152;
        }
        else
        {
            *((char *)&v22) = (char)idx[62];
            sub_414440(4, a1);
            v146 = NULL;
            /* unsupported instruction */ = /* unsupported instruction */ - 24;
            error(0, 0, dcgettext(NULL, "not writing through dangling symlink %s", 5));
            v145 = *((char *)&v22);
            goto LABEL_409710;
        }
    }
LABEL_40aa90:
    v154 = v137 >> 31;
    *((char *)&v26) = v150 == 21 & v154;
    if (v150 == 21 & v154)
    {
        v155 = a1;
        v139 = 21;
        if (*(v155))
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v139 = (unsigned int)((*(&v155[strlen(v155)] - 1) != 47) + 20);
        }
LABEL_40a593:
        *((unsigned int *)&v22) = v139;
        sub_414440(4, a1);
        /* unsupported instruction */ = /* unsupported instruction */ - 16;
        v144 = *((unsigned int *)&v22);
        v143 = dcgettext(NULL, "cannot create regular file %s", 5);
        goto LABEL_40a000;
    }
    v9 = ~(v11) & v149;
    if (v137 < 0)
        goto LABEL_0x40b931;
    *((char *)&v26) = 1;
    v4 = v16;
LABEL_40954f:
    if (*((char *)&v22))
    {
        if (!(int)idx[0x44])
            goto LABEL_409568;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        if (!ioctl(v137, 1074041865))
            goto LABEL_40a64a;
        if ((int)idx[0x44] != 2)
            goto LABEL_409568;
        v19 = sub_414310(1, 4, v50);
        v21 = sub_414310(0, 4, a1);
        v22 = dcgettext(NULL, "failed to clone %s from %s", 5);
        err = __errno_location();
        /* unsupported instruction */ = /* unsupported instruction */ - 40;
        error(0, *(err), v22);
        v140 = 0;
        goto LABEL_4096e5;
    }
LABEL_40a64a:
    v157 = (char)idx[29];
    *((unsigned int *)&v22) = v157 | v9;
    if (!v157 && !v9)
    {
        v43.st_nlink = 0;
        if ((char)idx[31])
            goto LABEL_40a676;
        goto LABEL_40a74a;
    }
    else
    {
        *((char *)&v22) = 0;
    }
LABEL_409568:
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    v158 = fstat(v137, &v43);
    if (v158)
    {
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v187 = "cannot fstat %s";
        v21 = sub_414440(4, a1);
LABEL_40b133:
        v22 = dcgettext(NULL, v187, 5);
        v195 = __errno_location();
        /* unsupported instruction */ = /* unsupported instruction */ - 24;
        error(0, *(v195), v22);
LABEL_4096e2:
        v140 = 0;
        goto LABEL_4096e5;
    }
    *((unsigned int *)&v18) = v158;
    v159 = v43.st_nlink;
    v160 = v9 | v159;
    if (v159 != v160)
    {
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v9 = (!fchmod(v137, v160) ? v9 : *((unsigned int *)&v18));
    }
    if (!*((char *)&v22))
    {
        *((unsigned int *)&v22) = v9;
        if ((char)idx[31])
        {
LABEL_40a676:
            v37 = *((unsigned long long *)((void*)&v41 + 72));
            v38 = *((unsigned long long *)((void*)&v41 + 80));
            v39 = *((unsigned long long *)((void*)&v41 + 88));
            v40 = *((unsigned long long *)((void*)&v41 + 96));
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (!sub_40e6a0(v137, a2, a3, &v37, 0))
                goto LABEL_40a6d4;
            v17 = sub_414440(4, a1);
            v18 = dcgettext(NULL, "preserving times for %s", 5);
            v188 = __errno_location();
            /* unsupported instruction */ = /* unsupported instruction */ - 32;
            error(0, *(v188), v18);
            if (!(char)idx[50])
                goto LABEL_40a6d4;
            goto LABEL_4096e2;
        }
        else
        {
LABEL_40a6d4:
            if ((char)idx[29] && !(v189 = (unsigned long)(unsigned long long)*((unsigned int *)((void*)&v41 + 28)), v190 = (unsigned long)(unsigned long long)*((unsigned int *)((void*)&v41 + 32)), (unsigned int)v189 == v43.st_uid && (unsigned int)v190 == v43.st_gid))
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((struct stat **)(/* unsupported instruction */ - 8)) = &v43;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((unsigned long long *)(/* unsupported instruction */ - 8)) = (char)v26;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((unsigned long *)(/* unsupported instruction */ - 8)) = v190;
                *((unsigned long *)(/* unsupported instruction */ - 8)) = v189;
                v191 = sub_407080(idx, a1, a2, a3, v137, v41.st_nlink, v49, a6, a7, a8);
                /* unsupported instruction */ = /* unsupported instruction */ + 16;
                if (v191 != 0xffffffff && !v191)
                {
                    *((unsigned int *)&v19) = (unsigned int)v19 & 0xfffff1ff;
                    goto LABEL_40a74a;
                }
            }
            else
            {
LABEL_40a74a:
                if (!(char)v21 || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), sub_406220(v50, v135, a1, v137, idx)))
                    v140 = 1;
                else
                    v140 = (char)idx[54] ^ 1;
                if (0xff0000000000ff & (long long)idx[24])
                {
                    *((char *)&v22) = v140;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v140 = *((char *)&v22);
                    if (sub_40c830(v50, v135, a1, v137, (unsigned int)v19))
                    {
                        if ((char)idx[50])
                            v140 = 0;
                    }
                }
                else if (!(char)idx[57])
                {
                    v192 = v26;
                    v21 = v192 & (char)idx[32];
                    if (v192 & (char)idx[32])
                    {
                        if (g_423010 == 0xffffffff)
                        {
                            *((char *)&v26) = v140;
                            g_423010 = umask(0);
                            /* unsupported instruction */ = /* unsupported instruction */ - 16;
                            umask(g_423010);
                            v140 = v26;
                        }
                        *((char *)&v22) = v140;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((char *)&v26) = v21;
                        v140 = (sub_40c8e0(a1, v137, ~(g_423010) & 438) ? 0 : *((char *)&v22));
                    }
                    else if (*((unsigned int *)&v22) || v4)
                    {
                        v193 = g_423010;
                        if (v193 == 0xffffffff)
                        {
                            *((char *)&v19) = v140;
                            g_423010 = umask(0);
                            v21 = g_423010;
                            /* unsupported instruction */ = /* unsupported instruction */ - 16;
                            umask(g_423010);
                            v140 = v19;
                            if (~(v21) & v4 || *((unsigned int *)&v22))
                            {
                                v193 = g_423010;
                                if (v21 == 0xffffffff)
                                {
                                    *((char *)&v22) = v140;
                                    g_423010 = umask(0);
                                    /* unsupported instruction */ = /* unsupported instruction */ - 16;
                                    umask(g_423010);
                                    v193 = g_423010;
                                    v140 = *((char *)&v22);
                                }
LABEL_40a7ca:
                                *((char *)&v22) = v140;
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                v140 = *((char *)&v22);
                                if (fchmod(v137, v11 & ~(v193)))
                                {
                                    *((char *)&v19) = v140;
                                    v21 = sub_414440(4, a1);
                                    v22 = dcgettext(NULL, "preserving permissions for %s", 5);
                                    v194 = __errno_location();
                                    /* unsupported instruction */ = /* unsupported instruction */ - 32;
                                    error(0, *(v194), v22);
                                    v140 = ((char)idx[50] ? 0 : (char)v19);
                                }
                            }
                        }
                        else if (!(!(~(v193) & v4) && !*((unsigned int *)&v22)))
                        {
                            goto LABEL_40a7ca;
                        }
                    }
                }
                else
                {
                    *((char *)&v22) = v140;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v140 = (sub_40c8e0(a1, v137, (int)idx[16]) ? 0 : *((char *)&v22));
                }
LABEL_4096e5:
                *((char *)&v22) = v140;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (close(v137) >= 0)
                {
                    v146 = v29;
                    v145 = *((char *)&v22);
                    goto LABEL_409710;
                }
                else
                {
                    v22 = sub_414440(4, a1);
                    v196 = dcgettext(NULL, "failed to close %s", 5);
                    v197 = __errno_location();
                    /* unsupported instruction */ = /* unsupported instruction */ - 32;
                    error(0, *(v197), v196);
                    v146 = v29;
                    v145 = 0;
                    goto LABEL_409710;
                }
            }
        }
    }
    v6 = v43.st_mtime;
    v15 = v6;
    if (v6 - 0x20000 > 0x1ffffffffffe0000)
    {
        v15 = 0x20000;
        v162 = 0x200;
        if (v6 - 1 <= 0x1fffffffffffffff)
            v162 = v6;
        v6 = v162;
    }
    if (((unsigned short)v44.st_nlink & 0xf000) != 0x8000)
    {
        v163 = 1;
        goto LABEL_40ab10;
    }
    v163 = 1;
    if (v44.st_ctime >= (long long)(v44.st_atime / 0x200))
        goto LABEL_40ab10;
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    v14 = lseek(v135, 0, 3);
    if (v14 >= 0)
    {
LABEL_40b4d9:
        v163 = 3;
LABEL_40ab10:
        if (((unsigned short)v43.st_nlink & 0xf000) != 0x8000 || (int)idx[12] != 3 && (v163 == 1 || (int)idx[12] != 2))
        {
            *((unsigned int *)&v18) = v163;
            sub_40e5e0(v135, 0, 0, 2);
            /* unsupported instruction */ = /* unsupported instruction */ - 16;
            v167 = sub_40da00();
            if (((unsigned short)v44.st_nlink & 0xf000) == 0x8000 && v44.st_atime < v15)
                v15 = v44.st_atime + 1;
            v27 = 0;
            v168 = v167 + v15 - 1 - (v167 + v15 - 1) % v167;
            if (v168 > 0)
                v167 = v168;
            v15 = v167;
            v5 = (int)idx[0x44];
            if ((unsigned int)v18 != 3)
            {
                v6 = 0;
                v169 = (int)idx[12] == 3;
                goto LABEL_40abf8;
            }
            else
            {
                v7 = 1;
                goto LABEL_40b1f7;
            }
        }
        *((unsigned int *)&v18) = v163;
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        sub_40e5e0(v135, 0, 0, 2);
        v27 = 0;
        if ((unsigned int)v18 != 3)
        {
            v5 = (int)idx[0x44];
            v169 = (int)idx[12] == 3;
LABEL_40abf8:
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((char **)(/* unsupported instruction */ - 8)) = &v27;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long **)(/* unsupported instruction */ - 8)) = &v30;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long *)(/* unsupported instruction */ - 8)) = 0xffffffffffffffff;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((char **)(/* unsupported instruction */ - 8)) = a1;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((char **)(/* unsupported instruction */ - 8)) = v50;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v5;
            /* unsupported instruction */ = /* unsupported instruction */ + 32;
            v170 = (char)sub_406a30(v135, v137, &v29, v15, v6, v169, v49, a6, a7, a8, a9, a10) ^ 1;
            goto LABEL_40ac42;
        }
        v5 = (int)idx[0x44];
        v7 = (int)idx[12];
LABEL_40b1f7:
        v18 = v44.st_atime;
        if (v14 >= 0)
        {
            v17 = v50;
            v10 = 0;
            v3 = 0;
            index = *((char *)&v22);
            v14 = v100;
            v171 = v14;
            v13 = idx;
            v172 = 0;
            while (1)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v173 = lseek(v135, v171, 4);
                if (v173 >= 0)
                {
                    if (v18 < v173)
                        v18 = v173;
                    goto LABEL_40b293;
                }
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (*(__errno_location()) != 6)
                    goto LABEL_40b50c;
                v173 = v18;
                if (v18 > v171)
                    goto LABEL_40b293;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v174 = lseek(v135, 0, 2);
                v173 = v174;
                if (v174 < 0)
                {
LABEL_40b50c:
                    v100 = v14;
                    v50 = v17;
                    idx = v13;
LABEL_40b521:
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v180 = "cannot lseek %s";
                    v18 = sub_414440(4, v50);
                    goto LABEL_40b541;
                }
                if (v174 <= v171)
                    break;
                v18 = v174;
LABEL_40b293:
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (lseek(v135, v171, 0) < 0)
                    goto LABEL_40b50c;
                v175 = v171 - v3;
                v176 = v175 - v172;
                if (v175 == v172)
                {
                    if (v7 != 1)
                        v176 = v6;
                    index = 0;
                    v172 = v173 - v171;
                    v177 = v176;
                }
                else if (v7 != 1)
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    index = sub_406920(v137, a1, v7 == 3, v176);
                    if (!index)
                        goto LABEL_40b575;
                    v177 = v6;
                    v172 = v173 - v171;
                }
                else
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    if ((char)sub_406170(v137, v176))
                    {
                        index = 0;
                        v177 = 0;
                        v172 = v173 - v171;
                    }
                    else
                    {
                        v100 = v14;
                        v50 = v17;
                        idx = v13;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v180 = "%s: write failed";
                        v18 = sub_4147c0(0, 3, a1);
LABEL_40b541:
                        v22 = dcgettext(NULL, v180, 5);
                        v182 = __errno_location();
                        /* unsupported instruction */ = /* unsupported instruction */ - 24;
                        error(0, *(v182), v22);
                        goto LABEL_40b56d;
                    }
                }
                v50 = v17;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((char **)(/* unsupported instruction */ - 8)) = &v28;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((unsigned long **)(/* unsupported instruction */ - 8)) = &v30;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((unsigned long *)(/* unsupported instruction */ - 8)) = v172;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((char **)(/* unsupported instruction */ - 8)) = a1;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((char **)(/* unsupported instruction */ - 8)) = v50;
                *((unsigned long long *)(/* unsupported instruction */ - 8)) = v5;
                /* unsupported instruction */ = /* unsupported instruction */ + 32;
                if (!(char)sub_406a30(v135, v137, &v29, v15, v177, 1, v49, a6, a7, a8, a9, a10))
                {
LABEL_40b575:
                    v100 = v14;
                    v50 = v17;
                    idx = v13;
LABEL_40b56d:
                    v183 = 0;
                    goto LABEL_40b42b;
                }
                v10 = v30 + v171;
                index = (v30 ? v28 : index);
                if (v30 >= v172)
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v178 = lseek(v135, v10, 3);
                    if (v178 >= 0)
                    {
                        v3 = v171;
                        v171 = v178;
                    }
                    else
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v100 = v14;
                        idx = v13;
                        if (*(__errno_location()) != 6)
                            goto LABEL_40b521;
                        goto LABEL_40b3ff;
                    }
                }
                else
                {
                    v100 = v14;
                    v50 = v17;
                    idx = v13;
                    v179 = v10;
                    goto LABEL_40b500;
                }
            }
            v100 = v14;
            v50 = v17;
            v179 = v174;
            idx = v13;
LABEL_40b500:
            v18 = v179;
LABEL_40b3ff:
            v181 = v10 < v18;
            if (v18 > v10 || index)
                goto LABEL_40b5a2;
            goto LABEL_40b423;
        }
        v10 = 0;
        v181 = 0 < v18;
LABEL_40b5a2:
        if (v7 != 1)
        {
            *((char *)&v17) = v181;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (ftruncate(v137, v18))
                goto LABEL_40b908;
            if (v7 != 3 || !(char)v17 || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (unsigned int)(unsigned long long)fallocate((unsigned long long)v137, 3, v10, v18 - v10) >= 0 || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v184 = __errno_location(), v185 = *(v184), v185 == 95 || v185 == 38)))
                goto LABEL_40b423;
            *((char *)&v18) = v185 == 95 | v185 == 38;
            v17 = v184;
            v22 = sub_414440(4, a1);
            v186 = dcgettext(NULL, "error deallocating %s", 5);
            /* unsupported instruction */ = /* unsupported instruction */ - 24;
            error(0, *(v17), v186);
            v183 = v18;
LABEL_40b42b:
            v170 = v183 ^ 1;
LABEL_40ac42:
            if (!v170 && v27 && (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (int)(long long)ftruncate(v137, v30) < 0))
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v187 = "failed to extend %s";
                v21 = sub_414440(4, a1);
                goto LABEL_40b133;
            }
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (!(char)sub_406170(v137, v18 - v10))
            {
LABEL_40b908:
                v18 = sub_414440(4, a1);
            }
LABEL_40b423:
            v183 = *((char *)&v22);
            goto LABEL_40b42b;
        }
    }
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    err1 = __errno_location();
    v165 = *(err1);
    v18 = err1;
    if (v165 == 6)
    {
        goto LABEL_40b4d9;
    }
    else if (v165 == 22)
    {
        goto LABEL_0x40b4ce;
    }
    else if (v165 == 95)
    {
        goto LABEL_0x40b4ce;
    }
    else
    {
        goto LABEL_0x40969e;
    }
    goto LABEL_40b4d9;
}



// Function: copy @ 0xba80
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[8];
    unsigned int field_c;
    char padding_10[7];
    char field_17;
    char padding_18[34];
    char field_3a;
    char padding_3b[9];
    unsigned int field_44;
} struct_0;

extern unsigned long long g_423568;
extern unsigned long long g_423570;

unsigned int copy(unsigned long a0, unsigned long a1, unsigned int a2, char *a3, unsigned int a4, struct_0 *a5, char *a6, unsigned long long a7)
{
    char v0;  // [bp-0x11]

    if (a5->field_0 > 3)
    {
        __assert_fail(); /* do not return */
    }
    else if (a5->field_c - 1 > 2)
    {
        __assert_fail(); /* do not return */
    }
    else if (a5->field_44 <= 2)
    {
        if (a5->field_17 && a5->field_3a)
            __assert_fail(); /* do not return */
        if (a5->field_c != 2 && a5->field_44 == 2)
            __assert_fail(); /* do not return */
        v0 = 0;
        g_423570 = a0;
        g_423568 = a1;
        return sub_4076f0(a0, a1, a2, a3, a4, NULL, NULL, a5, 1, &v0, a6, a7);
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: src_to_dest_hash @ 0xbc10
void src_to_dest_hash(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: src_to_dest_compare @ 0xbc20
unsigned long long src_to_dest_compare(unsigned long long *a0, unsigned long a1)
{
    if (*(a0) == *((long long *)a1))
        return _INSERT(*((long long *)(a1 + 8)), 0, a0[1] == *((long long *)(a1 + 8)));
    return 0;
}



// Function: forget_created @ 0xbc60
typedef struct struct_0 {
    char padding_0[6];
    char field_6;
    char padding_7[9];
    unsigned int field_10;
} struct_0;

extern long long g_423578;

long long forget_created(unsigned long a0, unsigned long long a1, char a2)
{
    unsigned long v5;  // fs
    struct_0 *v6;  // rax
    struct_0 *ptr;  // rbp
    char v8;  // dh
    unsigned long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long v3;  // [bp-0x10]

    v3 = *((long long *)(40 + v5));
    v0 = a0;
    v1 = a1;
    v2 = 0;
    v6 = sub_412270(g_423578, &v0);
    if (v6)
    {
        __unsupported_jumpkind_Ijk_NoDecode()
        ptr = v6;
        free(v6->field_10);
        free(ptr);
    }
    ptr->field_6 = ptr->field_6 + v8;
}



// Function: src_to_dest_lookup @ 0xbcd0
typedef struct struct_0 {
    char padding_0[102];
    unsigned long long field_66;
} struct_0;


int src_to_dest_lookup(void)
{
    unsigned long v3;  // fs
    struct_0 *v4;  // rsi
    struct_0 *v0;  // [bp-0x20]
    unsigned long v1;  // [bp-0x10]

    v1 = *((long long *)(40 + v3));
    v0 = v4;
    if (v1 == *((long long *)(40 + v3)))
        return;
    __unsupported_jumpkind_Ijk_NoDecode()
    if (v1 >= *((long long *)(40 + v3)))
        goto LABEL_0x40bd21;
    goto *((void *)(*((long long *)&(v4->padding_0)[1])));
}



// Function: remember_copied @ 0xbd30
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    void* field_10;
} struct_0;

extern long long g_423578;

void* remember_copied(long long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    struct_0 *ptr;  // rbp
    void* v4;  // rax
    struct_0 *v5;  // rax
    unsigned long long v0;  // [bp-0x20]

    v0 = v2;
    ptr = sub_416c50(24);
    v4 = sub_417130(a0);
    ptr->field_0 = a1;
    ptr->field_10 = v4;
    ptr->field_8 = a2;
    v5 = sub_412210(g_423578, ptr);
    if (!v5)
    {
        sub_417150(); /* do not return */
    }
    else if (ptr != v5)
    {
        free(ptr->field_10);
        free(ptr);
        return v5->field_10;
    }
    else
    {
        return NULL;
    }
}



// Function: hash_init @ 0xbdb0
extern unsigned long long g_423578;

void hash_init(void)
{
    g_423578 = sub_411bb0(103, 0, sub_40bc10, sub_40bc20, sub_40bc40);
    if (!g_423578)
        sub_417150(); /* do not return */
    return;
}



// Function: try_link @ 0xbdf0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    unsigned int field_10;
    unsigned int field_14;
} struct_0;

long long try_link(long long a0, struct_0 *a1)
{
    return linkat(a1->field_0, a1->field_8, a1->field_10, a0, a1->field_14);
}



// Function: samedir_template @ 0xbe10
typedef struct struct_0 {
    unsigned long long field_0;
    char field_8;
} struct_0;

void* samedir_template(void* a0, void* ptr)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rdi
    struct_0 *v3;  // rax

    v1 = sub_40d970() - a0;
    v2 = v1 + 9;
    if (v2 > 0x100)
    {
        ptr = malloc(v2);
        if (!ptr)
            return NULL;
    }
    v3 = mempcpy(ptr, a0, v1);
    v3->field_0 = 6365935209750754627;
    v3->field_8 = 0;
    return ptr;
}



// Function: try_symlink @ 0xbe70
typedef struct struct_0 {
    long long field_0;
    unsigned int field_8;
} struct_0;

long long try_symlink(long long a0, struct_0 *a1)
{
    return symlinkat(a1->field_0, a1->field_8, a0);
}



// Function: force_linkat @ 0xbe90
int force_linkat(void)
{
    unsigned long long v11;  // r15
    unsigned long long v12;  // r14
    int v21;  // eax
    unsigned int v22;  // r8d
    unsigned int v23;  // r8d
    char v24;  // r9b
    unsigned long v25;  // r15
    unsigned int v26;  // edi
    unsigned long long v27;  // rsi
    unsigned long v28;  // fs
    unsigned long long v13;  // r12
    unsigned long long v14;  // rdx
    unsigned int v15;  // ebp
    unsigned long long v16;  // rbx
    unsigned int *v17;  // rax
    unsigned long v18;  // rcx
    char v19;  // 4102
    char *v20;  // rax
    unsigned long long v0;  // [bp-0x30]
    unsigned int v1;  // [bp-0x2c]
    unsigned long v2;  // [bp-0x28]
    unsigned int v3;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned long long v4;  // [bp-0x18]
    unsigned int v5;  // [bp-0x10], Other Possible Types: unsigned long long
    unsigned int v6;  // [bp-0xc]
    unsigned long long v7;  // [bp-0x8]
    char v8;  // [bp+0x0]
    int v10;  // [bp+0x150]

    v7 = v11;
    v5 = v12;
    v3 = v13;
    v15 = v14;
    v0 = v16;
    *(v17) = *(v17) + (unsigned int)v17;
    v19 = *((char *)v17 + 2 * v18 - 117);
    *((char *)v17 + 2 * v18 - 117) = *((char *)v17 + 2 * v18 - 117) + *((char *)((void*)&v17 + 1));
    v20 = (char *)_INSERT(v17, 0, *((char *)&v17) + 37 + ((char)(v19 + *((char *)((void*)&v17 + 1)) <= v19 ? 1 : 0) & 1));
    *(v20) = *(v20) - (*((char *)&v17) + 37 + ((char)(v19 + *((char *)((void*)&v17 + 1)) <= v19 ? 1 : 0) & 1));
    *(v20) = *(v20) + *((char *)&v17) + 37 + ((char)(v19 + *((char *)((void*)&v17 + 1)) <= v19 ? 1 : 0) & 1);
    v21 = v10;
    if (v21 < 0)
    {
        v1 = v22;
        if (!linkat())
        {
            if (v14 == *((long long *)(40 + v28)))
                return;
            __stack_chk_fail(); /* do not return */
        }
        v23 = v1;
        v21 = *(__errno_location());
    }
    v1 = v23;
    if (v24 == 1 && v21 == 0x11)
    {
        v25 = sub_40be10(v18, &v8);
        if (v25)
        {
            v6 = v1;
            v3 = v26;
            v4 = v27;
            v5 = v15;
            if (sub_4157b0(v25, 0, &v3, sub_40bdf0, 6))
            {
                __errno_location();
            }
            else
            {
                if (renameat(v15, v25, v15, v18))
                    __errno_location();
                unlinkat(v15, v25, 0);
            }
            if (v25 != v2)
                free(v25);
        }
        else
        {
            __errno_location();
        }
    }
    if (v14 != *((long long *)(40 + v28)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: force_symlinkat @ 0xc000
int force_symlinkat(unsigned long a0, unsigned int a1, unsigned long a2, char a3, int a4)
{
    int v4;  // eax
    char *v5;  // r15
    int v6;  // r12d
    unsigned int v7;  // r12d
    unsigned int v8;  // r12d
    unsigned long v0;  // [bp-0x158]
    unsigned int v1;  // [bp-0x150]
    char v2;  // [bp-0x148]

    if (a4 < 0)
    {
        v4 = symlinkat();
        if (!v4)
            return v4;
        a4 = *(__errno_location());
    }
    if (a3 != 1 || a4 != 0x11)
        return a4;
    v5 = sub_40be10(a2, &v2);
    if (!v5)
    {
        v7 = *(__errno_location());
        v8 = *(__errno_location());
        return *(__errno_location());
    }
    v0 = a0;
    v1 = a1;
    if (sub_4157b0(v5, 0, &v0, sub_40be70, 6))
    {
        v6 = *(__errno_location());
    }
    else
    {
        v6 = -0x1;
        if (renameat(a1, v5, a1, a2))
        {
            v6 = *(__errno_location());
            unlinkat(a1, v5, 0);
        }
    }
    if (v5 == &v2)
        return v6;
    free(v5);
    return v6;
    return a4;
}



// Function: restorecon_private @ 0xc140
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

unsigned int restorecon_private(long long a0, unsigned long a1, unsigned int a2)
{
    unsigned int v4;  // r12d
    unsigned int *err;  // rbx
    long long ptr;  // r15
    long long ptr1;  // r13
    unsigned int v9;  // ebp
    unsigned int v10;  // r14d
    unsigned int *err1;  // rbx
    unsigned int v13;  // ebp
    unsigned long v0;  // [bp-0xe8]
    unsigned long v1;  // [bp-0xe0]
    stat v2;  // [bp-0xd8]

    v0 = 0;
    v1 = 0;
    if (a0)
    {
        v4 = open(a1, 0x20000, a2);
        err = __errno_location();
        if (v4 != 0xffffffff)
        {
            if (fstat(v4, &v2) >= 0)
                goto LABEL_40c1bd;
            ptr = 0;
            ptr1 = 0;
            v9 = 0xffffffff;
            goto LABEL_40c3ab;
        }
        v10 = *(err);
        if (*(err) != 40)
        {
            ptr = 0;
            ptr1 = 0;
            goto LABEL_40c28b;
        }
        if (lstat(a1, &v2) < 0)
        {
            v10 = *(err);
            ptr = 0;
            ptr1 = 0;
            goto LABEL_40c28b;
        }
LABEL_40c1bd:
        if (selabel_lookup(a0, &v0, a1, v2.st_nlink) < 0)
        {
            v10 = *(err);
            ptr = 0;
            ptr1 = 0;
            if (v10 == 2)
            {
                *(err) = 61;
                v10 = 61;
                goto LABEL_40c244;
            }
        }
        ptr1 = context_new(v0);
        if (ptr1)
        {
            if (v4 != 0xffffffff)
            {
                if (sub_415610(v4) >= 0)
                {
                    ptr = context_new(v1);
                    if (ptr)
                        goto LABEL_40c221;
                    v10 = *(err);
                    v9 = 0xffffffff;
                    goto LABEL_40c24f;
                }
                else
                {
                    ptr = 0;
                    v9 = 0xffffffff;
                    goto LABEL_40c3ab;
                }
            }
            if (sub_415590(a1) >= 0)
            {
                ptr = context_new(v1);
                if (ptr)
                {
LABEL_40c221:
                    if (!context_type_get(ptr1) || context_type_set(ptr) || !context_str(ptr))
                    {
                        v10 = *(err);
                        goto LABEL_40c244;
                    }
                    if (v4 != 0xffffffff)
                    {
                        v9 = fsetfilecon(v4);
LABEL_40c3ab:
                        v10 = *(err);
LABEL_40c24f:
                        close(v4);
                    }
                    else
                    {
                        v4 = lsetfilecon(a1);
                        v10 = *(err);
LABEL_40c28b:
                        v9 = v4;
                    }
                }
                else
                {
                    v10 = *(err);
                    v9 = 0xffffffff;
                }
            }
            else
            {
                v10 = *(err);
                ptr = 0;
                goto LABEL_40c28b;
            }
        }
        else
        {
            v10 = *(err);
            ptr = 0;
LABEL_40c244:
            v9 = 0xffffffff;
            if (v4 != 0xffffffff)
                goto LABEL_40c24f;
        }
        context_free(ptr1);
        context_free(ptr);
        freecon(v0);
        freecon(v1);
        *(err) = v10;
        return v9;
    }
    else if (getfscreatecon(&v1) >= 0)
    {
        err1 = __errno_location();
        if (!v1)
        {
            *(err1) = 61;
            return 0xffffffff;
        }
        v13 = lsetfilecon(a1, v1);
        freecon(v1);
        *(err1) = *(err1);
        return v13;
    }
    else
    {
        return 0xffffffff;
    }
}



// Function: defaultcon @ 0xc450
unsigned int defaultcon(long long a0, char *a1, unsigned int a2)
{
    char *ptr;  // rbp
    unsigned int *err;  // rbx
    char *v15;  // rsi
    unsigned long long v16;  // rax
    long long v17;  // rax
    unsigned int v18;  // eax
    char *v7;  // rax
    unsigned int v8;  // r14d
    long long ptr1;  // r12
    unsigned int v10;  // r15d
    char *v11;  // r13
    unsigned long ptr2;  // rbp
    unsigned short v13;  // ax
    unsigned int v14;  // eax
    unsigned long v0;  // [bp-0x60]
    unsigned long v1;  // [bp-0x58]
    unsigned long v2;  // [bp-0x50]
    unsigned long v3;  // [bp-0x48]

    ptr = a1;
    v0 = 0;
    v1 = 0;
    err = __errno_location();
    if (*(ptr) != 47)
    {
        v7 = sub_40da80(ptr, 2);
        ptr = v7;
        if (v7)
        {
LABEL_40c4bb:
            if (selabel_lookup(a0, &v0, ptr, a2) >= 0)
            {
                v2 = 0;
                v3 = 0;
                ptr2 = sub_40e480(ptr);
                if (getcon(&v2) >= 0 && sub_415510(ptr2, &v3) >= 0 && !(v13 = (unsigned short)(unsigned long long)mode_to_security_class((unsigned long long)a2), !v13))
                {
                    v14 = security_compute_create(v2, v3, v13, &v1);
                    v8 = *(err);
                    free(ptr2);
                    freecon(v2);
                    freecon(v3);
                    *(err) = v8;
                    if (v14 < 0)
                        goto LABEL_40c699;
                    ptr = context_new(v0);
                    if (ptr)
                    {
                        ptr1 = context_new(v1);
                        if (ptr1)
                        {
                            v16 = _INSERT(ptr1, 0, *(v15));
                            if (0)
                                goto LABEL_0x40c5a2;
                            __unsupported_jumpkind_Ijk_NoDecode()
                            if (!v16 || context_type_set(ptr1) || (v17 = (long long)(unsigned long long)context_str(ptr1), !v17))
                                goto LABEL_40c678;
                            v18 = setfscreatecon(v17);
                            v8 = *(err);
                            v10 = v18;
                        }
                        else
                        {
LABEL_40c678:
                            v8 = *(err);
                            v10 = 0xffffffff;
                        }
                    }
                    else
                    {
                        v8 = *(err);
                        ptr1 = 0;
                        v10 = 0xffffffff;
                    }
                }
                else
                {
                    v8 = *(err);
                    ptr1 = 0;
                    ptr = NULL;
                    free(ptr2);
                    v10 = 0xffffffff;
                    freecon(v2);
                    freecon(v3);
                    *(err) = v8;
                }
            }
            else
            {
                v8 = *(err);
                if (v8 != 2)
                {
LABEL_40c699:
                    ptr1 = 0;
                    ptr = NULL;
                    v10 = 0xffffffff;
                }
                else
                {
                    *(err) = 61;
                    v8 = 61;
                    ptr1 = 0;
                    ptr = NULL;
                    v10 = 0xffffffff;
                }
            }
        }
        else
        {
            v8 = *(err);
            ptr1 = 0;
            v10 = 0xffffffff;
            v7 = v11;
        }
    }
    else
    {
        v7 = NULL;
        goto LABEL_40c4bb;
    }
    context_free(ptr);
    context_free(ptr1);
    freecon(v0);
    freecon(v1);
    free(v7);
    *(err) = v8;
    return v10;
}



// Function: restorecon @ 0xc6f0
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

unsigned int restorecon(long long a0, char *a1, char a2)
{
    char *v3;  // r13
    char *v4;  // rbp
    unsigned int *err;  // rbx
    unsigned int v6;  // r14d
    struct_0 *v7;  // rbp
    unsigned int v8;  // edx
    unsigned int v9;  // eax
    unsigned int v10;  // edx
    unsigned int v11;  // eax
    void* v12;  // rdi
    char v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x40]

    v3 = NULL;
    v4 = a1;
    if (*(a1) == 47)
    {
        err = __errno_location();
        if (!a2)
            goto LABEL_40c801;
    }
    else
    {
        v3 = sub_40da80(v4, 2);
        if (!v3)
            return 0;
        err = __errno_location();
        v4 = v3;
        if (!a2)
        {
LABEL_40c801:
            v11 = sub_40c140(a0, v4, v10);
            __outdword((unsigned short)v10, v11);
            free(v12);
            *(err) = *(err);
            return _INSERT(a0, 0, v11 != 0xffffffff);
        }
    }
    v1 = 0;
    v6 = 0;
    v7 = sub_417190(&v0, 16, 0);
LABEL_40c760:
    if (sub_410840(v7))
    {
        do
        {
            if (sub_40c140(a0, *((long long *)&v7->field_20), v8) >= 0)
                goto LABEL_40c760;
        } while ((v6 = *(err), (unsigned long long)sub_410840(v7)));
    }
    v9 = *(err);
    if (*(err))
        v6 = v9;
    if (sub_410650())
        v6 = *(err);
    free(v3);
    return _INSERT(a0, 0, !v6);
}



// Function: dir_name @ 0xe480
void dir_name(void)
{
    if (!sub_40e4e0())
        sub_417150(); /* do not return */
    return;
}



// Function: raw_hasher @ 0x11240
void raw_hasher(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: raw_comparator @ 0x11260
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0x11270
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

extern unsigned int g_41c620[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_41c620[0])
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
    idx->field_28 = &g_41c620[0];
    return 0;
}



// Function: hash_find_entry @ 0x11300
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
LABEL_4113af:
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
        goto LABEL_4113af;
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



// Function: transfer_entries @ 0x11550
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
LABEL_411578:
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
                            goto LABEL_4115fe;
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
LABEL_4115fe:
                            v2 = iter->field_0;
                            goto LABEL_411601;
                        }
                    }
                    index = v6;
                }
            }
LABEL_411601:
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_411578;
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



// Function: hash_print_statistics @ 0x117c0
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
            goto LABEL_4118f9;
LABEL_41187a:
        v12 = (int)v5;
    }
    else
    {
        v11 = MulV(AddV(v6 >> 1 | (unsigned int)v6 & 1, v6 >> 1 | (unsigned int)v6 & 1), 0x4059000000000000);
        if (v5 >= 0)
            goto LABEL_41187a;
LABEL_4118f9:
        v12 = (int)(AddV(v5 >> 1 | (unsigned int)v5 & 1, v5 >> 1 | (unsigned int)v5 & 1));
    }
    __fprintf_chk(a1, 0x1, "# buckets used:    %lu (%.2f%%)\n", v6, (unsigned long long)(DivV(v11, v12)));
    return __fprintf_chk(*((unsigned int *)&&v0), 0x1, "max bucket length: %lu\n", v2);
}



// Function: hash_lookup @ 0x11920
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



// Function: hash_get_next @ 0x119e0
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_4119ec();
    return;
}



// Function: hash_do_for_each @ 0x11ac0
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



// Function: hash_initialize @ 0x11bb0
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

extern struct_0 g_41c620;

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

    v1 = sub_411240;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_411260;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_41c620.field_0;
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_411270(ptr))
    {
        v6 = sub_411410(a0, v2->field_10);
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



// Function: hash_clear @ 0x11cc0
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



// Function: hash_free @ 0x11d70
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
            goto LABEL_411e1c;
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
LABEL_411e1c:
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



// Function: hash_rehash @ 0x11e60
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

    v11 = sub_411410(a1, idx->field_28->field_10);
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
        v13 = sub_411550(&ptr, idx, 0);
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
        if ((char)sub_411550(idx, &ptr, 1) && (char)sub_411550(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0x11fe0
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
    sub_411300(idx, a1, &v0, 0);
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
                goto LABEL_4120f6;
LABEL_41206f:
            v6 = (int)(unsigned int)v5;
        }
        else
        {
            v5 = *((long long *)&idx[1].field_0);
            v4 = AddV((unsigned int)(v3 >> 1) | (unsigned int)v3 & 1, (unsigned int)(v3 >> 1) | (unsigned int)v3 & 1);
            if (v5 >= 0)
                goto LABEL_41206f;
LABEL_4120f6:
            v6 = (int)(AddV((unsigned int)(v5 >> 1 | (unsigned int)v5 & 1), (unsigned int)(v5 >> 1 | (unsigned int)v5 & 1)));
        }
        if (((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69 | (char)((CmpF(v4, (unsigned int)(MulV(*((int *)(*((long long *)&idx[2].field_8) + 8)), v6))) & 69) >> 6)) & 1) != 1)
        {
            sub_411270(idx);
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
                else if (!(char)sub_411e60(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else
                {
                    sub_411300(idx, a1, &v0, 0);
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



// Function: hash_insert @ 0x12210
long long hash_insert(unsigned int *a0, long long a1)
{
    unsigned int v2;  // eax
    long long v0;  // [bp-0x18], Other Possible Types: unsigned long long

    v2 = sub_411fe0(a0, a1, &v0);
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



// Function: hash_remove @ 0x12270
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

    sub_411300(idx, a1, &v0, 1);
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
            goto LABEL_4123b6;
LABEL_412300:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = *((long long *)&idx[1].field_0);
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_412300;
LABEL_4123b6:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)*((long long *)&idx[2].field_8)), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_411270(idx);
    v9 = *((long long *)&idx[2].field_8);
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_411e60(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



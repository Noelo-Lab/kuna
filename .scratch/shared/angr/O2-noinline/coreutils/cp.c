// Function: main @ 0x40b0
typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern unsigned int g_41cad0[4];
extern unsigned int g_41cae0[4];
extern char g_41cdf8;
extern option g_4244a0;
extern long long g_424880;
extern long long g_425020;
extern long long g_425028;
extern long long stdout;
extern int optind;
extern char optarg;
extern char g_425158;
extern char g_425159;
extern char g_42515a;

int main(unsigned int a0, long long *a1)
{
    long long v36;  // rdi
    long long v37;  // rsi
    unsigned int v46;  // eax
    char *v47;  // rax
    char v49;  // 4098
    unsigned long v50;  // cc_ndep
    unsigned long long v51;  // 4107
    char *v52;  // rax
    int *err;  // rax
    char *v55;  // rsi
    long long v38;  // rdx
    unsigned long v57;  // cc_ndep
    unsigned long v58;  // 4110
    unsigned long v59;  // id
    unsigned long v60;  // ac
    long long v39;  // rcx
    long long v40;  // r8
    long long v41;  // r9
    int v42;  // eax
    long long v43;  // rax
    int *err1;  // rax
    unsigned long long v0;  // [bp-0xd8]
    unsigned long long v1;  // [bp-0xd0]
    long long v2;  // [bp-0xc8]
    long long v3;  // [bp-0xc0]
    long long v4;  // [bp-0xb8]
    long long v5;  // [bp-0xb0]
    char v6;  // [bp-0xa3]
    char v7;  // [bp-0xa2]
    char v8;  // [bp-0xa1]
    char *v9;  // [bp-0xa0]
    char v10;  // [bp-0x98], Other Possible Types: unsigned int
    unsigned int v11;  // [bp-0x94]
    unsigned int v12;  // [bp-0x90]
    unsigned int v13;  // [bp-0x8c]
    char v14;  // [bp-0x84]
    char v15;  // [bp-0x83]
    char v16;  // [bp-0x82]
    char v17;  // [bp-0x81]
    char v18;  // [bp-0x7c]
    char v19;  // [bp-0x7b]
    unsigned short v20;  // [bp-0x7a]
    unsigned long long v21;  // [bp-0x70]
    char v22;  // [bp-0x68]
    char v23;  // [bp-0x67]
    char v24;  // [bp-0x66]
    char v25;  // [bp-0x65]
    char v26;  // [bp-0x64]
    char v27;  // [bp-0x63]
    unsigned short v28;  // [bp-0x61]
    char v29;  // [bp-0x60]
    char v30;  // [bp-0x60]
    char v31;  // [bp-0x5e]
    char v32;  // [bp-0x5d]
    char v33;  // [bp-0x5c]
    unsigned int v34;  // [bp-0x54]

    sub_4127f0(*(a1));
    setlocale(6, &g_41cdf8);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_41a150(sub_40e560);
    g_42515a = is_selinux_enabled(v36, v37, v38, v39, v40, v41) > 0;
    sub_4049f0(&v10);
    v7 = 0;
    v5 = 0;
    v2 = 0;
    v8 = 0;
    v3 = 0;
    v4 = 0;
    v6 = 0;
    while (1)
    {
        v42 = getopt_long(a0, a1, "abdfHilLnprst:uvxPRS:TZ", &g_4244a0.name, NULL);
        if (v42 != -0x1)
        {
            if (v42 > 0x88)
                goto LABEL_404769;
            if (v42 > 71)
            {
                switch (v42)
                {
                case 72:
                    v11 = 3;
                    continue;
                case 76:
                    v11 = 4;
                    continue;
                case 80:
                    v11 = 2;
                    continue;
                case 82: case 114:
                    v29 = 1;
                    continue;
                case 83:
                    v6 = 1;
                    v4 = *((long long *)&optarg);
                    continue;
                case 84:
                    v7 = 1;
                    break;
                case 90:
                    if (g_42515a)
                    {
                        if (*((long long *)&optarg))
                        {
                            v5 = *((long long *)&optarg);
                            break;
                        }
                        else
                        {
                            v21 = selabel_open(0, 0, 0);
                            if (!v21)
                            {
                                v9 = dcgettext(NULL, "warning: ignoring --context", 5);
                                err1 = __errno_location();
                                error(0, *(err1), v9);
                                break;
                            }
                        }
                    }
                    else
                    {
                        if (*((long long *)&optarg))
                        {
                            error(0, 0, dcgettext(NULL, "warning: ignoring --context; it requires an SELinux-enabled kernel", 5));
                            break;
                        }
                    }
                case 97:
                    v22 = 1;
                    v11 = 2;
                    v19 = 1;
                    v20 = 0x101;
                    v24 = 1;
                    if (g_42515a)
                        v25 = 1;
                    v27 = 1;
                    v28 = 0x101;
                    continue;
                case 98:
                    v43 = *((long long *)&optarg);
                    v6 = 1;
                    if (!*((long long *)&optarg))
                        v43 = v3;
                    v3 = v43;
                    continue;
                case 100:
                    v22 = 1;
                    v11 = 2;
                    continue;
                case 102:
                    v16 = 1;
                    continue;
                case 105:
                    v12 = 3;
                    continue;
                case 108:
                    v17 = 1;
                    continue;
                case 110:
                    v12 = 2;
                    continue;
                case 112:
                    v19 = 1;
                    v20 = 0x101;
                    v24 = 1;
                    continue;
                case 115:
                    v31 = 1;
                    continue;
                case 116:
                    if (!v2)
                    {
                        v2 = *((long long *)&optarg);
                        break;
                    }
                    else
                    {
                        dcgettext(NULL, "multiple target directories specified", 5);
                        v58 = _ccall(19, 0, 0, v57);
                        v1 = v58 | v59 * 0x200000 & 0x200000 | v60 * 0x40000 & 0x40000;
                    }
                case 117:
                    v32 = 1;
                    continue;
                case 118:
                    v33 = 1;
                    continue;
                case 120:
                    v18 = 1;
                    continue;
                case 128:
                    v23 = 0;
                    continue;
                case 129:
                    v8 = 1;
                    continue;
                case 130:
                    sub_404a80(*((long long *)&optarg), &v10, 0);
                    continue;
                case 131:
                    g_425159 = 1;
                    continue;
                case 132:
                    if (*((long long *)&optarg))
                    {
                        sub_404a80(*((long long *)&optarg), &v10, 1);
                        v24 = 1;
                        continue;
                    }
                    break;
                case 133:
                    if (*((long long *)&optarg))
                    {
                        v0 = 1;
                        v34 = g_41cad0[sub_40d220("--reflink")];
                        break;
                    }
                    else
                    {
                        v34 = 2;
                        break;
                    }
                case 134:
                    v13 = g_41cae0[sub_40d220("--sparse", *((long long *)&optarg), &g_424880, &g_41cae0[0], 4, g_425028, 1)];
                    continue;
                case 135:
                    g_425158 = 1;
                    continue;
                case 136:
                    v15 = 1;
                    continue;
                default:
LABEL_404769:
                    sub_4057e0(1); /* do not return */
                }
            }
            else
            {
                if (v42 == -0x83)
                {
                    sub_416cc0(stdout, "cp", "GNU coreutils", g_425020, "Torbjorn Granlund", "David MacKenzie", "Jim Meyering", 0);
                    exit(0); /* do not return */
                }
                else if (v42 == -0x82)
                {
                    sub_4057e0(0); /* do not return */
                }
                goto LABEL_404769;
            }
        }
        else
        {
            if (v17 && v31)
            {
                v55 = "cannot make both hard and symbolic links";
                goto LABEL_404754;
            }
            if (v12 == 2)
            {
                v32 = 0;
                if (v6)
                {
                    v55 = "options --backup and --no-clobber are mutually exclusive";
LABEL_404754:
                    error(0, 0, dcgettext(NULL, v55, 5));
                    goto LABEL_404769;
                }
            }
            else if (v6)
            {
                if (v34 != 2 || v13 == 2)
                {
                    v46 = sub_40db20(dcgettext(NULL, "backup type", 5), v3);
                }
                else
                {
LABEL_404748:
                    v55 = "--reflink can be used only with --sparse=auto";
                    goto LABEL_404754;
                }
                goto LABEL_404556;
            }
            v46 = 0;
            if (v34 == 2 && v13 != 2)
                goto LABEL_404748;
LABEL_404556:
            v10 = v46;
            sub_40d750(v4);
            if (v11 != 1)
            {
LABEL_40456f:
                if (!v30)
                    break;
                goto LABEL_4046e5;
            }
            else if (v30 && !v17)
            {
                v11 = 2;
LABEL_4046e5:
                v14 = v8;
                break;
            }
            else
            {
                v11 = 4;
                goto LABEL_40456f;
            }
        }
    }
    v47 = v26;
    if (v5 || v21)
    {
        if (!*((char *)&v47))
        {
            v25 = 0;
        }
        else
        {
            if (v25)
                error(1, 0, dcgettext(NULL, "cannot set target context and preserve it", 5));
            __unsupported_jumpkind_Ijk_NoDecode()
            v49 = *(v47);
            *(v47) = *(v47) + *((char *)&v47);
            v51 = _ccall(4, 1, (unsigned long long)v49, (unsigned long long)*((char *)(void*)&v47), v50);
            if (v51 & 1)
                error(1, 0, dcgettext(NULL, "cannot preserve security context without an SELinux-enabled kernel", 5));
        }
        if (v5 && setfscreatecon(v5) < 0)
        {
            sub_414e00(v5);
            v52 = dcgettext(NULL, "failed to set default file creation context to %s", 5);
            err = __errno_location();
            error(1, *(err), v52);
        }
    }
    else if (*((char *)&v47) && !g_42515a)
    {
        error(1, 0, dcgettext(NULL, "cannot preserve security context without an SELinux-enabled kernel", 5));
    }
    sub_40bfe0();
    return (char)sub_405ae0(a0 - optind, &a1[optind], v2, v7, &v10) ^ 1;
}



// Function: cp_option_init @ 0x49f0
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned short field_18;
    char padding_1a[2];
    unsigned int field_1c;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    unsigned long long field_30;
    unsigned int field_38;
    unsigned short field_3c;
    char field_3e;
    char padding_3f[5];
    unsigned int field_44;
    unsigned long long field_48;
    unsigned long long field_50;
} struct_0;

char * cp_option_init(struct_0 *idx)
{
    char *v1;  // rax

    sub_4083b0();
    idx->field_44 = 1;
    idx->field_8 = 8589934596;
    idx->field_10 = 0x100000000;
    idx->field_1c = 0;
    idx->field_20 = 0;
    idx->field_4 = 1;
    idx->field_18 = 0;
    idx->field_28 = 0;
    idx->field_30 = 0x100;
    idx->field_38 = 0;
    idx->field_3c = 0;
    v1 = getenv("POSIXLY_CORRECT");
    idx->field_48 = 0;
    idx->field_50 = 0;
    idx->field_3e = v1;
    return v1;
}



// Function: decode_preserve_arg @ 0x4a80
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
} struct_0;

extern unsigned int g_41c990[4];
extern struct_0 g_41cab0;
extern long long g_424460;
extern long long g_425028;

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
    v1 = sub_417230();
    v13 = "--no-preserve";
    if (a2)
        v13 = "--preserve";
    v0 = v13;
    ptr = strchr(v1, 44);
    if (ptr)
        *(ptr) = 0;
    sub_40d220(v0, v1, &g_424460, &g_41cab0.field_0, 4, g_425028, 1);
    if ((&g_41cab0.field_0)[sub_40d220(v0, v1, &g_424460, &g_41cab0.field_0, 4, g_425028, 1)] > 6)
        abort(); /* do not return */
    goto *((void *)((long long)(g_41c990[(&g_41cab0.field_0)[sub_40d220(v0, v1, &g_424460, &g_41cab0.field_0, 4, g_425028, 1)]] + (char *)&g_41c990[0])));
}



// Function: make_dir_parents_private @ 0x4bf0
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
    int <0x404bf0[is_1]|Stack bp-0x1a8, 1 B>;  // [bp-0x1a8]
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

    n = sub_40e750();
    *(a4) = NULL;
    if (n <= a1)
        return 1;
    len = strlen(ptr);
    v16 = len + 24;
    i = (char *)&<0x404bf0[is_1]|Stack bp-0x1a8, 1 B> - (v16 & 0xfffffffffffff000);
    if (&<0x404bf0[is_1]|Stack bp-0x1a8, 1 B> != i)
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
LABEL_404e17:
                if (!(char)sub_407ff0(v5, dst, 0, 0, a6))
                    return 0;
                if (((unsigned short)v8 & 0xf000) == 0x4000)
                {
                    *(a5) = 0;
                    goto LABEL_404e59;
                }
                else
                {
                    goto LABEL_4051fc;
                }
            }
            else
            {
                if (stat(v5, v2))
                {
                    v28 = *(__errno_location());
                    if (v28)
                        goto LABEL_405114;
                }
                else
                {
                    if (((unsigned short)*((unsigned int *)(&v9 + 24)) & 0xf000) != 0x4000)
                    {
                        v28 = 20;
LABEL_405114:
                        dst = v28;
                        sub_414b40(4, v5);
                        error(0, dst, dcgettext(NULL, "failed to get attributes of %s", 5));
                        return 0;
                    }
                }
                idx = sub_416e90(168);
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
                    goto LABEL_404e17;
                v3 = idx;
                if (!(char)sub_407ff0(v5, dst, v3->field_10[2], 1, a6))
                    return 0;
                v31 = v3;
                *(a5) = 1;
                v32 = (!a6->field_1c[1] ? (!a6->field_1c[2] ? 0 : v31->field_10[2] & 18) : v31->field_10[2] & 63);
                v3 = v31;
                v33 = v3;
                if (mkdirat())
                {
                    sub_414b40(4, dst);
                    v39 = "cannot make directory %s";
                    goto LABEL_405260;
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
                    sub_414b40(4, dst);
                    v39 = "failed to get attributes of %s";
                    goto LABEL_405260;
                }
                v35 = v8;
                if (!a6->field_1c[2] && (v32 & ~(v35) && !(v3 = index, index = v3, v32 &= ~((unsigned int)(unsigned long long)sub_408640()), v35 = v8, !(v32 & ~(v35))) || ((unsigned short)v35 & 448) != 448))
                {
                    *((char *)&index->field_80) = 1;
                    index->field_10[2] = v32 | v35;
                }
                if ((v35 | 448) != v35 && sub_40e520(a2, v24))
                {
                    sub_414b40(4, dst);
                    v39 = "setting permissions for %s";
LABEL_405260:
                    v40 = dcgettext(NULL, v39, 5);
                    err = __errno_location();
                    error(0, *(err), v40);
                    return 0;
                }
                if (*(a5))
                    goto LABEL_404e90;
LABEL_404e59:
                if (*((long long *)&a6->padding_21[4]))
                {
                    if (!(char)sub_408250(dst, 0, a6) && *((char *)&a6->field_28 + 4))
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
LABEL_404e90:
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
LABEL_4051fc:
        sub_414b40(4);
        error(0, 0, dcgettext(NULL, "%s exists but is not a directory", 5));
        return 0;
    }
    return 1;
}



// Function: re_protect @ 0x52e0
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
    char padding_24[108];
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
    char v19;  // al
    char *v20;  // rax
    int *err;  // rax
    char *v22;  // rsi
    char *v23;  // rax
    int *err1;  // rax
    unsigned long v11;  // rax
    char *i;  // rcx
    unsigned long long v13;  // rdx
    unsigned long dst;  // r12
    long long v15;  // r13
    unsigned long v16;  // 4120
    unsigned long long v17;  // rdx
    char v18;  // r8b
    char v0;  // [bp-0x2078]
    char v1;  // [bp-0x1078]
    int <0x4052e0[is_2]|Stack bp-0x78, 1 B>;  // [bp-0x78]
    char *v2;  // [bp-0x78]
    char v3;  // [bp-0x70], Other Possible Types: unsigned long long
    char v4;  // [bp-0x68], Other Possible Types: unsigned long long
    unsigned long long v5;  // [bp-0x60]
    unsigned long long v6;  // [bp-0x58]
    unsigned long long v7;  // [bp-0x50]

    iter = a3;
    v3 = a2;
    len = strlen(ptr);
    v11 = len + 24;
    i = (char *)&<0x4052e0[is_2]|Stack bp-0x78, 1 B> - (v11 & 0xfffffffffffff000);
    if (&<0x4052e0[is_2]|Stack bp-0x78, 1 B> != i)
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
            v4 = sub_415960(iter);
            v5 = v17;
            v7 = v17;
            v6 = sub_415980(iter);
            if (utimensat(a1, v15, v2, 0))
            {
                sub_414b40(4, dst);
                v20 = dcgettext(NULL, "failed to preserve times for %s", 5);
                err = __errno_location();
                error(0, *(err), v20);
                return 0;
            }
        }
        if (!index->padding_0[29] || !sub_40e540(a1, v15, iter->field_1c, iter->field_20))
        {
            v18 = index->field_1e;
            if (v18)
                goto LABEL_4054c0;
            goto LABEL_4053b1;
        }
        else
        {
            v19 = sub_408400(index);
            if (!v19)
            {
                v3 = v19;
                sub_414b40(4, dst);
                v22 = "failed to preserve ownership for %s";
LABEL_405526:
                v23 = dcgettext(NULL, v22, 5);
                err1 = __errno_location();
                error(0, *(err1), v23);
                return v3;
            }
            sub_40e540(a1, v15, 0xffffffff, iter->field_20);
            v18 = index->field_1e;
            if (!v18)
            {
LABEL_4053b1:
                if (!iter->field_90 || !(v3 = v18, (unsigned int)(unsigned long long)sub_40e520((unsigned long long)a1, v15, (unsigned long long)iter->field_18)))
                    continue;
                sub_414b40(4, dst);
                v22 = "failed to preserve permissions for %s";
                goto LABEL_405526;
            }
            else
            {
LABEL_4054c0:
                if (sub_40caa0(v15, 0xffffffff, dst, 0xffffffff, iter->field_18))
                    return 0;
            }
        }
    } while ((*((char *)(dst + iter->field_98)) = 47, iter = (struct_0 *)iter->field_a0, iter));
    return 1;
}



// Function: usage @ 0x57e0
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
    char *v5;  // rax
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
        __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n", 5));
        v5 = dcgettext(NULL, "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n", 5);
        fputs_unlocked(v5, stdout);
        sub_404970(v6, v7, v8, v9, v10, v11);
        v12 = dcgettext(NULL, "  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don't copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n", 5);
        fputs_unlocked(v12, stdout);
        v13 = dcgettext(NULL, "  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n", 5);
        fputs_unlocked(v13, stdout);
        v14 = dcgettext(NULL, "  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n", 5);
        fputs_unlocked(v14, stdout);
        v15 = dcgettext(NULL, "  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n", 5);
        fputs_unlocked(v15, stdout);
        v16 = dcgettext(NULL, "  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n", 5);
        fputs_unlocked(v16, stdout);
        v17 = dcgettext(NULL, "      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n", 5);
        fputs_unlocked(v17, stdout);
        v18 = dcgettext(NULL, "  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n", 5);
        fputs_unlocked(v18, stdout);
        v19 = dcgettext(NULL, "      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n", 5);
        fputs_unlocked(v19, stdout);
        v20 = dcgettext(NULL, "  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n", 5);
        fputs_unlocked(v20, stdout);
        v21 = dcgettext(NULL, "  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n", 5);
        fputs_unlocked(v21, stdout);
        v22 = dcgettext(NULL, "  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n", 5);
        fputs_unlocked(v22, stdout);
        v23 = dcgettext(NULL, "      --help        display this help and exit\n", 5);
        fputs_unlocked(v23, stdout);
        v24 = dcgettext(NULL, "      --version     output version information and exit\n", 5);
        fputs_unlocked(v24, stdout);
        v25 = dcgettext(NULL, "\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n", 5);
        fputs_unlocked(v25, stdout);
        v26 = dcgettext(NULL, "\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n", 5);
        fputs_unlocked(v26, stdout);
        sub_4049a0(v6, v7, v8, v9, v10, v11);
        v27 = dcgettext(NULL, "\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n", 5);
        fputs_unlocked(v27, stdout);
        sub_4055b0(v6, v7, v8, v9, v10, v11);
    }
    exit(a0); /* do not return */
}



// Function: do_copy @ 0x5ae0
typedef struct struct_1 {
    char padding_0[16];
    long long field_10;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[18];
    char field_16;
    char padding_17[37];
    char field_3c;
} struct_0;

extern struct_0 g_425100;
extern char g_425158;
extern char g_425159;

unsigned int do_copy(int a0, void* a1, long long a2, char a3, struct_0 *a4)
{
    unsigned long v17;  // rax
    unsigned long v18;  // rax
    unsigned long v27;  // rax
    unsigned long long k;  // rcx
    unsigned long long v29;  // rdx
    unsigned long dst;  // r12
    unsigned long v31;  // r12
    unsigned long len;  // rax
    unsigned long v33;  // rax
    unsigned long long l;  // rcx
    unsigned long long v35;  // rdx
    unsigned long dst1;  // rax
    long long v19;  // rbx
    char *v37;  // rcx
    struct_0 *v38;  // rax
    unsigned long v39;  // 4148
    unsigned long v40;  // rdi
    char v41;  // al
    unsigned long v42;  // rdx
    char v43;  // al
    struct_0 *ptr;  // r15
    struct_0 *v45;  // r15
    unsigned int v46;  // r12d
    unsigned int v20;  // r14d
    char *v47;  // rax
    unsigned long v48;  // r14
    unsigned long v49;  // r12
    struct_0 *j;  // r15
    unsigned long v51;  // rax
    struct_0 *iter;  // rdi
    unsigned long long v53;  // rcx
    char *v54;  // rsi
    struct_1 *v55;  // rsi
    char *v56;  // rsi
    unsigned long long idx;  // rbx
    unsigned long v57;  // rax
    void* v59;  // rax
    unsigned int v22;  // r14d
    unsigned long v23;  // r15
    unsigned long v24;  // rdi
    void* ptr1;  // rax
    unsigned long len1;  // rax
    unsigned int v0;  // [bp-0x77bf048]
    int v1;  // [bp-0x1148]
    int <0x405ae0[is_8]|Stack bp-0x1148, 1 B>;  // [bp-0x1148]
    char *v2;  // [bp-0x128]
    char *v3;  // [bp-0x120]
    char *v4;  // [bp-0x118]
    long long v5;  // [bp-0x110]
    struct_0 *v6;  // [bp-0x108]
    unsigned int v7;  // [bp-0x100]
    int i;  // [bp-0xfc]
    char v9;  // [bp-0xea]
    char v10;  // [bp-0xe9]
    struct_0 *v11;  // [bp-0xe8]
    char v12;  // [bp-0xe0], Other Possible Types: unsigned long
    char v13;  // [bp-0xd8]
    unsigned int v14;  // [bp-0xc0]
    unsigned long v15;  // [bp+0x0]
    long long v16;  // [bp+0x8]

    i = a0;
    v5 = a2;
    v6 = a4;
    v9 = 0;
    v17 = !a2;
    if ((unsigned int)v17 >= a0)
    {
        if (a0 != 1)
        {
            v54 = "missing file operand";
        }
        else
        {
            sub_414b40(4, *((long long *)a1));
            v56 = "missing destination file operand after %s";
LABEL_4060c1:
            v57 = (unsigned long long)error(0, 0, dcgettext(NULL, v56, 5) & 0xffffffff);
            goto LABEL_4060d9;
        }
LABEL_40613c:
        v57 = (unsigned long long)error(0, 0, dcgettext(NULL, v54, 5));
LABEL_4060d9:
        v59 = (void*)_INSERT(_INSERT(v57, 0, (short)v57 / *((char *)((void*)&v19 + 1))), 1, (short)v57 % *((char *)((void*)&v19 + 1)));
        *((unsigned int *)((char *)v59 - 117)) = *((int *)((char *)v59 - 117)) - 1;
    }
    v14 = 0;
    if (!a3)
    {
        if (v5)
        {
            v7 = sub_415a20(v5, &v13);
            /* unsupported instruction */ = (int)(long long *)((char *)&<0x405ae0[is_8]|Stack bp-0x1148, 1 B> - 4120);
            if (!(char)sub_415a10(v7))
                goto LABEL_0x4060e3;
            if (i != 1)
                goto LABEL_405f87;
            goto LABEL_405b81;
        }
        else
        {
            v18 = i;
            v19 = *((long long *)((char *)a1 + 8 * v18 - 8));
            v5 = v19;
            v7 = sub_415a20(v19, &v13);
            /* unsupported instruction */ = (int)(long long *)((char *)&<0x405ae0[is_8]|Stack bp-0x1148, 1 B> - 4120);
            v20 = sub_415a10(v7);
            if ((char)v20)
            {
                i = (unsigned int)v18 - 1;
                if (v19)
                {
                    if (i > 1)
                    {
LABEL_405f87:
                        sub_408330(v6);
                        /* unsupported instruction */ = (int)(long long *)((char *)&<0x405ae0[is_8]|Stack bp-0x1148, 1 B> - 4120);
                        sub_408370(v6);
                    }
                    else if (i != 1)
                    {
                        return v20;
                    }
LABEL_405b81:
                    idx = 0;
                    v22 = 1;
                    v3 = &v12;
                    v4 = &v10;
                    v2 = &v9;
                    do
                    {
                        v23 = *((long long *)((char *)a1 + 8 * idx));
                        v12 = 0;
                        if (!g_425158)
                        {
                            v24 = v23;
                            if (g_425159)
                                goto LABEL_405e18;
                            goto LABEL_405be5;
                        }
                        else
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            sub_40e800(v23);
                            v24 = v23;
                            if (!g_425159)
                            {
LABEL_405be5:
                                ptr1 = sub_40db60(v24);
                                /* unsupported instruction */ = /* unsupported instruction */ - 16;
                                len1 = strlen(ptr1);
                                v27 = len1 + 24;
                                k = /* unsupported instruction */ - (v27 & 0xfffffffffffff000);
                                if (/* unsupported instruction */ != k)
                                {
                                    do
                                    {
                                        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                                        *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8));
                                    } while (/* unsupported instruction */ != k);
                                }
                                v29 = (unsigned int)v27 & 0xfffffff0 & 0xfff;
                                /* unsupported instruction */ = /* unsupported instruction */ - v29;
                                if (v29)
                                    *((long long *)(/* unsupported instruction */ + v29 - 8)) = *((long long *)(/* unsupported instruction */ + v29 - 8));
                                dst = memcpy(/* unsupported instruction */ + 15 & 0xfffffffffffffff0, ptr1, len1 + 1);
                                sub_40e800(dst);
                                /* unsupported instruction */ = /* unsupported instruction */ - 32;
                                v31 = sub_40ec90(v5, dst + (strcmp(dst, "..") < 1), v3);
                                goto LABEL_405c8c;
                            }
                            else
                            {
LABEL_405e18:
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                len = strlen(v24);
                                v33 = len + 24;
                                l = /* unsupported instruction */ - (v33 & 0xfffffffffffff000);
                                if (/* unsupported instruction */ != l)
                                {
                                    do
                                    {
                                        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                                        *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8));
                                    } while (/* unsupported instruction */ != l);
                                }
                                v35 = (unsigned int)v33 & 0xfffffff0 & 0xfff;
                                /* unsupported instruction */ = /* unsupported instruction */ - v35;
                                if (v35)
                                    *((long long *)(/* unsupported instruction */ + v35 - 8)) = *((long long *)(/* unsupported instruction */ + v35 - 8));
                                dst1 = memcpy(/* unsupported instruction */ + 15 & 0xfffffffffffffff0, v23, len + 1);
                                sub_40e800(dst1);
                                /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                v37 = NULL;
                                v31 = sub_40ec90(v5, dst1, v3);
                                v38 = v6;
                                v39 = v38->field_3c;
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                *((unsigned long *)(/* unsupported instruction */ - 8)) = v40;
                                if ((char)v39)
                                    v37 = "%s -> %s\n";
                                *((struct_0 **)(/* unsupported instruction */ - 8)) = v38;
                                v41 = sub_404bf0(v31, v12 - v31, v7, v37, &v11, v2, v15);
                                /* unsupported instruction */ = /* unsupported instruction */;
                                if (*((char *)v12) == 47)
                                {
                                    v42 = v12 + 1;
                                    do
                                    {
                                        v12 = v42;
                                        v42 += 1;
                                    } while (*((char *)(v42 - 1)) == 47);
                                }
                                if (v41)
                                {
LABEL_405c8c:
                                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                    *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
                                    *((char **)(/* unsupported instruction */ - 8)) = v4;
                                    /* unsupported instruction */ = /* unsupported instruction */;
                                    v22 &= sub_40bae0(v23, v31, v7, v12, v9, v6, v15, v16);
                                    if (!g_425159)
                                        continue;
                                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                    v22 &= sub_4052e0(v31, v7, v12, v11, v6);
                                    v43 = g_425159;
                                    goto LABEL_405f1b;
                                }
                                else
                                {
                                    v43 = g_425159;
                                    v22 = 0;
LABEL_405f1b:
                                    if (v43)
                                    {
                                        ptr = v11;
                                        if (ptr)
                                        {
                                            do
                                            {
                                                v45 = *((long long *)&ptr[2].padding_17[15]);
                                                v11 = v45;
                                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                                free(ptr);
                                                ptr = v45;
                                            } while (ptr);
                                        }
                                    }
                                }
                            }
                        }
                        idx += 1;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        free(v31);
                    } while (i > (unsigned int)idx);
                    return v22;
                }
            }
            else
            {
                v46 = *(__errno_location());
                if (*(__errno_location()) == 2)
                    v9 = 1;
                if (i > 2)
                {
                    sub_414b40(4, v5);
                    v47 = dcgettext(NULL, "target %s", 5);
                    error(1, *(__errno_location()), v47);
                }
            }
        }
    }
    else
    {
        if (v5)
        {
            /* unsupported instruction */
            /* unsupported instruction */
            error(1, 0, v17);
LABEL_4060a4:
            sub_414b40(4, v55->field_10);
            v56 = "extra operand %s";
            goto LABEL_4060c1;
        }
        else if (!(i <= 2))
        {
            goto LABEL_4060a4;
        }
    }
    v48 = *((long long *)a1);
    v49 = (long long)a1[8];
    if (!g_425159)
    {
        if (v6->field_16 && v6->field_0 && !strcmp(v48, v49) && !v9)
        {
            if (!v14)
            {
                /* unsupported instruction */
                /* unsupported instruction */
                v0 += 1;
            }
            else if (((unsigned short)v14 & 0xf000) == 0x8000)
            {
                j = v6;
                v51 = sub_40daa0(0xffffff9c, v49, j->field_0);
                iter = &g_425100.field_0;
                for (v53 = 22; v53; j = j->padding_4)
                {
                    v53 -= 1;
                    iter->field_0 = j->field_0;
                    iter = iter->padding_4;
                }
                v49 = v51;
                g_425100.field_0 = 0;
                v6 = &g_425100.field_0;
            }
        }
        v1 = (int)_INSERT(<0x405ae0[is_8]|Stack bp-0x1148, 1 B>, 4120, 0);
        *((char **)&(&v1)[0x1010]) = &v12;
        return sub_40bae0(v48, v49, 0xffffff9c, v49, -(v9), v6, *((unsigned long long *)(&v1 + 0x1010)), *((unsigned long long *)(&v1 + 4120)));
    }
    v54 = "with --parents, the destination must be a directory";
    goto LABEL_40613c;
    goto LABEL_40613c;
}



// Function: is_ancestor @ 0x61e0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

unsigned int is_ancestor(unsigned long a0, struct_0 *a1)
{
    struct_0 *v1;  // rsi

    if (!a1)
        return 0;
    while (1)
    {
        v1 = a1;
        if (v1->field_8 == *((long long *)(a0 + 8)) && v1->field_10 == *((long long *)a0))
            return 1;
        a1 = v1->field_0;
        if (!v1->field_0)
            return 0;
    }
}



// Function: owner_failure_ok @ 0x6230
typedef struct struct_0 {
    char padding_0[27];
    char field_1b;
} struct_0;

int owner_failure_ok(struct_0 *a0)
{
    unsigned int *err;  // rax
    unsigned int v2;  // edx

    err = __errno_location();
    v2 = *(err);
    if (v2 != 1 && v2 != 22)
        return _INSERT(err, 0, v2 == 1 | v2 == 22);
    return a0->field_1b ^ 1;
}



// Function: copy_attr_quote @ 0x6260
void copy_attr_quote(unsigned long a0, unsigned long a1)
{
    sub_414b40(4);
    return;
}



// Function: valid_options @ 0x6270
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

unsigned int valid_options(struct_0 *a0)
{
    unsigned int v1;  // edx
    char *v2;  // rax

    if (a0->field_0 > 3)
        __assert_fail(); /* do not return */
    v1 = a0->field_c;
    if (v1 - 1 > 2)
        __assert_fail(); /* do not return */
    v2 = a0->field_44;
    if ((unsigned int)v2 > 2)
        goto LABEL_406292;
LABEL_406292:
    *(v2) = *(v2) + *((char *)&v2);
    if (a0->field_17 && a0->field_3a)
        __assert_fail(); /* do not return */
    if ((unsigned int)v2 != 2)
    {
        return 1;
    }
    else if (v1 == 2)
    {
        return 1;
    }
    else
    {
        __assert_fail(); /* do not return */
    }
}



// Function: writable_destination @ 0x6370
unsigned int writable_destination(unsigned int a0, long long a1, unsigned int a2)
{
    unsigned long long v2;  // r12
    unsigned long long v3;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (((unsigned short)a2 & 0xf000) == 0xa000)
        return 1;
    v0 = v2;
    if (!(char)sub_416e20())
    {
        v3 = faccessat(a0, a1, 2, 0x200);
        return _INSERT(v3, 0, !(unsigned int)v3);
    }
    return 1;
}



// Function: overwrite_ok @ 0x63e0
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

extern FILE *stdin;
extern unsigned long long stderr;

unsigned int overwrite_ok(struct_1 *a0, long long a1, unsigned int a2, long long a3, struct_0 *a4)
{
    unsigned long long v11;  // r14
    unsigned long long v12;  // r13
    void* v21;  // rax
    unsigned int v22;  // r12d
    unsigned long long v13;  // r12
    unsigned long long v14;  // rbx
    char *v15;  // rax
    int v16;  // edi
    char *v17;  // rax
    unsigned long v18;  // fs
    unsigned long v19;  // rax
    char *v20;  // rdi
    char *v0;  // [bp-0x58]
    char v1;  // [bp-0x3c]
    char v2;  // [bp-0x3b]
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
    if (!(char)sub_406370(a2, a3, a4->field_18))
    {
        sub_40eb60(a4->field_18, &v1);
        v3 = 0;
        sub_414b40(4, a1);
        if (a0->field_18 || a0->field_14 & 0xffffff & 0xffff00)
            v15 = dcgettext(NULL, "%s: replace %s, overriding mode %04lo (%s)? ", 5);
        else
            v15 = dcgettext(NULL, "%s: unwritable %s (mode %04lo, %s); try anyway? ", 5);
        v0 = &v2;
        __fprintf_chk(v16, 0x1, v15);
    }
    else
    {
        sub_414b40(4, a1);
        v17 = dcgettext(NULL, "%s: overwrite %s? ", 5);
        __fprintf_chk(stderr, 0x1, v17);
    }
    if (v4 != *((long long *)(40 + v18)))
        __stack_chk_fail(); /* do not return */
    v9 = v7;
    v8 = *((long long *)(40 + v18));
    ptr = NULL;
    v7 = 0;
    v19 = getline(&ptr, &v7, stdin);
    if (v19 <= 0)
    {
        free(ptr);
        return 0;
    }
    v20 = ptr;
    v21 = &v20[v19] - 1;
    if (*((char *)v21) == 10)
    {
        *((char *)v21) = 0;
        v20 = ptr;
    }
    v22 = rpmatch(v20) > 0;
    free(ptr);
    return v22;
}



// Function: abandon_move @ 0x6530
typedef struct struct_1 {
    char padding_0[20];
    unsigned int field_14;
    char field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

unsigned long long abandon_move(struct_1 *a0, long long a1, unsigned int a2, long long a3, struct_0 *a4)
{
    unsigned long long v3;  // rax
    unsigned int v4;  // edx
    long long v0;  // [bp-0x28]
    struct_0 *v1;  // [bp-0x20]

    v3 = a0->field_18;
    if (!(char)v3)
        __assert_fail(); /* do not return */
    v4 = *((int *)&a0->padding_0[8]);
    if (v4 == 2)
        return v3;
    if (v4 != 3)
    {
        if (v4 != 4 || !a0[2].padding_0[11] || !(v1 = a4, v0 = a3, !(char)sub_406370(a2, a3, a4->field_18)))
            return 0;
        a3 = v0;
        a4 = v1;
    }
    return sub_4063e0(a0, a1, a2, a3, a4) ^ 1;
}



// Function: subst_suffix @ 0x65e0
void* subst_suffix(unsigned long a0, unsigned long a1, void* a2)
{
    unsigned long v1;  // r12
    unsigned long n;  // r15
    void* v3;  // rax

    v1 = a1 - a0;
    n = strlen(a2) + 1;
    v3 = sub_416eb0(n + v1);
    memcpy(v3 + v1, a2, n);
    return memcpy(v3, v3 & 0xffffffff, v1);
}



// Function: source_is_dst_backup @ 0x6640
extern void* g_425580;

unsigned int source_is_dst_backup(void* a0, unsigned long long *a1, unsigned int a2, unsigned long a3)
{
    unsigned long len;  // rax
    unsigned long len1;  // r13
    unsigned long len2;  // rax
    unsigned long v7;  // rax
    unsigned int v8;  // eax
    void* v0;  // [bp-0xf0]
    char v1;  // [bp-0xd8], Other Possible Types: unsigned long
    unsigned long v2;  // [bp-0xd0]

    len = strlen(a0);
    v0 = sub_40db60(a3);
    len1 = strlen(v0);
    if (strlen(g_425580) + len1 != len)
    {
        return 0;
    }
    else if (memcmp(a0, v0, len1))
    {
        return 0;
    }
    else if (strcmp(a0 + len1, g_425580))
    {
        return 0;
    }
    else
    {
        len2 = strlen(a3);
        v7 = sub_4065e0(a3, a3 + len2, g_425580);
        v8 = fstatat(a2, v7, &v1, 0);
        free(v7);
        if (v8)
        {
            return 0;
        }
        else if (a1[1] != v2)
        {
            return 0;
        }
        else
        {
            return *(a1) == v1;
        }
    }
}



// Function: clone_file @ 0x6770
int clone_file(int a0, unsigned long a1)
{
    return ioctl(a0, 1074041865);
}



// Function: infer_scantype @ 0x6780
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    char padding_1c[20];
    long long field_30;
    char padding_38[8];
    long long field_40;
} struct_0;

int infer_scantype(int a0, struct_0 *a1, long long *a2)
{
    long long v2;  // rax
    unsigned int v3;  // edi
    char v0;  // [bp-0x10]

    if (((unsigned short)a1->field_18 & 0xf000) == 0x8000 && a1->field_40 < a1->field_30 / 0x200)
    {
        v2 = lseek(a0, 0, 3);
        *(a2) = v2;
        if (v2 < 0)
        {
            v3 = *(__errno_location());
            if (*(__errno_location()) != 6 && *(__errno_location()) != 22)
                return (char)sub_4061c0() * 2;
        }
    }
    return (unsigned int)&v0;
}



// Function: write_zeros @ 0x6820
extern void g_425018;
extern char g_425160;
extern unsigned long long g_425560;

unsigned int write_zeros(unsigned long a0, unsigned long a1)
{
    unsigned long v1;  // rbp
    void* v2;  // rax
    unsigned long v3;  // rbp
    unsigned long v4;  // rbx
    unsigned long v5;  // rax

    v1 = a1;
    if (!g_425560)
    {
        *((unsigned int *)v2) = *((int *)v2) + (unsigned int)v2;
        *((char *)v2) = *((char *)v2) + *((char *)&v2);
        g_425560 = calloc(*((long long *)&g_425018), a1);
        if (!g_425560)
        {
            *((unsigned long *)&g_425018) = 0x400;
            g_425560 = &g_425160;
        }
    }
    if (!v1)
        return 1;
    while (1)
    {
        v3 = v1;
        v4 = *((long long *)&g_425018);
        if (v3 <= *((long long *)&g_425018))
            v4 = v3;
        v5 = sub_411320();
        if (v5 != v4)
            return 0;
        v1 = v3 - v5;
        if (v3 == v5)
            return 1;
    }
}



// Function: punch_hole @ 0x68d0
unsigned int punch_hole(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned int v1;  // eax
    char v2;  // al
    unsigned int v3;  // edx

    v1 = fallocate();
    if (v1 >= 0)
        return v1;
    v2 = sub_4061c0(*(__errno_location()));
    if (v3 == 38)
    {
        return 0;
    }
    else if (!v2)
    {
        return v1;
    }
    else
    {
        return 0;
    }
}



// Function: create_hole @ 0x6910
unsigned int create_hole(unsigned int a0, long long a1, char a2, unsigned long a3)
{
    long long v1;  // rax
    char *v2;  // rsi
    char *v3;  // rax
    int *err;  // rax

    v1 = lseek(a0, a3, 1);
    if (v1 >= 0)
    {
        if (!a2)
            return 1;
        if (sub_4068d0(a0, v1 - a3, a3) >= 0)
            return 1;
        sub_414b40(4, a1);
        v2 = "error deallocating %s";
    }
    else
    {
        sub_414b40(4, a1);
        v2 = "cannot lseek %s";
    }
    v3 = dcgettext(NULL, v2, 5);
    err = __errno_location();
    error(0, *(err), v3);
    return 0;
}



// Function: sparse_copy @ 0x6a40
unsigned int sparse_copy(int a0, unsigned int a1, unsigned long long *a2, unsigned long a3, unsigned long a4, char a5, char a6, long long a7, long long a8, unsigned long a9, unsigned long long *a10, char *a11)
{
    unsigned long v8;  // r12
    unsigned long v9;  // rax
    char *v16;  // rax
    unsigned long v17;  // rcx
    unsigned long iter;  // r14
    unsigned int v19;  // r13d
    unsigned long long v20;  // r11
    int size;  // eax
    unsigned long v22;  // rdx
    unsigned long long v23;  // r11
    int *err;  // rax
    unsigned long iter1;  // r12
    unsigned int v10;  // ecx
    unsigned long long iter2;  // r10
    unsigned long v27;  // rbx
    char v28;  // r12b
    unsigned int v29;  // ebp
    unsigned long long v30;  // r10
    char v31;  // r9b
    unsigned int v32;  // r12d
    char v33;  // r9b
    unsigned int v34;  // eax
    unsigned long v35;  // rdi
    unsigned int v11;  // eax
    char *v36;  // rax
    char *v38;  // rax
    int *err1;  // rax
    unsigned long v40;  // r13
    unsigned int v12;  // ecx
    int *v13;  // rdx
    int *v14;  // rbp
    unsigned int v15;  // r15d
    unsigned long v0;  // [bp-0xb0], Other Possible Types: unsigned long long
    char v1;  // [bp-0xa8], Other Possible Types: unsigned long long
    unsigned long long v2;  // [bp-0xa0]
    unsigned long node;  // [bp-0x98]
    unsigned long v4;  // [bp-0x90]
    char v5;  // [bp-0x7a]
    unsigned long count;  // [bp-0x70]
    unsigned long v7;  // [bp-0x50]

    v4 = a3;
    *(a11) = 0;
    node = a9;
    *(a10) = 0;
    if (!a4 && a6)
    {
        if (a9)
        {
            v8 = a9;
            while (1)
            {
                v9 = copy_file_range();
                if (!v9)
                {
                    node = v8;
                    if (*(a10))
                        return 1;
                    break;
                }
                if (v9 >= 0)
                {
                    *(a10) = *(a10) + v9;
                    v8 -= v9;
                    if (v8 == v9)
                        return 1;
                    continue;
                }
                v10 = *(__errno_location());
                if (*(__errno_location()) != 38)
                {
                    v11 = sub_4061c0(*(__errno_location()));
                    if ((char)v11)
                        goto LABEL_406f18;
                    if (v12 > 26)
                        continue;
                    if ((unsigned int)(1 << ((char)v12 & 63)) & 0x4440200)
                        goto LABEL_406f18;
                    if (v12 == 1)
                    {
                        v15 = v11;
                        node = v8;
                        v14 = v13;
                        if (!*(a10))
                            break;
                        goto LABEL_406e6a;
                    }
                }
                else
                {
LABEL_406f18:
                    node = v8;
                    break;
                }
                if (v12 != 4)
                {
                    v14 = v13;
                    v15 = v11;
LABEL_406e6a:
                    sub_414a70(1, 4, a8);
                    sub_414a70(0, 4, a7);
                    v16 = dcgettext(NULL, "error copying %s to %s", 5);
                    error(0, *(v14), v16);
                    return v15;
                }
            }
        }
        else
        {
            return 1;
        }
    }
    else
    {
        if (!node)
            return 1;
    }
    if (a4)
        v17 = a4;
    else
        v17 = v4;
    iter = 0;
    v19 = 0;
    v7 = v17;
    while (1)
    {
        while (1)
        {
            v20 = *(a2);
            if (!v20)
            {
                size = getpagesize();
                __unsupported_jumpkind_Ijk_NoDecode()
                v20 = sub_416e60(_INSERT(size CONCAT 0, 0, (char)size & 40));
                *(a2) = v20;
            }
            v22 = node;
            v0 = v20;
            if (v4 <= v22)
                v22 = v4;
            v23 = v0;
            count = read(a0, v20, v22);
            if (count >= 0)
                break;
            err = __errno_location();
            if (*(err) != 4)
            {
                sub_414b40(4, a7);
                v36 = dcgettext(NULL, "error reading %s", 5);
                error(0, *(err), v36);
                return 0;
            }
        }
        if (count)
        {
            iter1 = count;
            iter2 = v23;
            v27 = v7;
            *(a10) = *(a10) + iter1;
            v5 = a4;
            while (1)
            {
                v0 = iter1;
                if (v27 > iter1)
                    v27 = iter1;
                v28 = v27 == iter1;
                v29 = _INSERT(v27 CONCAT 0, 0, v27 & v5);
                if (!(v27 & v5))
                {
                    if (((char)v19 == 1 || !v28) && (v29 = v19, v27))
                        goto LABEL_406d71;
                    goto LABEL_406d13;
                }
                v2 = v23;
                v1 = iter2;
                v30 = v1;
                v23 = v2;
                v29 = sub_4069e0(iter2, v27);
                v31 = ((char)v29 ^ (char)v19) & iter;
                if (!((char)v29 != 1 && v28))
                {
                    iter2 = v30;
                    if (!v31)
                    {
LABEL_406d71:
                        iter += v27;
                        if (0x8000000000000000 + iter < v27)
                        {
                            sub_414b40(4, a7);
                            error(0, 0, dcgettext(NULL, "overflow reading %s", 5));
                            return 0;
                        }
                        iter1 -= v27;
                        iter2 += v27;
                        v19 = v29;
LABEL_406d93:
                        if (!iter1)
                            break;
                    }
                    else
                    {
                        v32 = 0;
LABEL_406c99:
                        v2 = v30;
                        v1 = v31;
                        if ((char)v19)
                            goto LABEL_406d32;
                        goto LABEL_406cac;
                    }
                }
                else if (v31)
                {
                    v32 = 1;
                    v29 = 0;
                    goto LABEL_406c99;
                }
                else
                {
                    v29 = 0;
                    iter2 = v30;
LABEL_406d13:
                    v2 = iter2;
                    iter += v27;
                    v32 = 1;
                    v1 = 0;
                    if ((char)v19)
                    {
LABEL_406d32:
                        v34 = sub_406910(a1, a8, a5, iter);
                        v33 = v1;
                        iter2 = v2;
                        if (!(char)v34)
                            return v34;
LABEL_406ccf:
                        if (v32)
                        {
                            if (v27)
                            {
                                if (v33)
                                {
                                    iter = v27;
                                    v19 = v29;
                                    v23 = iter2;
                                    v27 = 0;
                                    continue;
                                }
                            }
                            else
                            {
                                if (!v33)
                                {
                                    v0 = 0;
                                }
                                else
                                {
                                    iter = 0;
                                    break;
                                }
                            }
                            v23 = iter2;
                            v19 = v29;
                            iter2 += v27;
                            iter = 0;
                            iter1 = v0 - v27;
                            goto LABEL_406d93;
                        }
                        else
                        {
                            v23 = iter2;
                            iter1 -= v27;
                            v19 = v29;
                            iter2 += v27;
                            iter = v27;
                            goto LABEL_406d93;
                        }
                    }
LABEL_406cac:
                    v33 = v1;
                    iter2 = v2;
                    if (iter != sub_411320(a1, v23, iter))
                    {
                        sub_414b40(4, a8);
                        v38 = dcgettext(NULL, "error writing %s", 5);
                        err1 = __errno_location();
                        error(0, *(err1), v38);
                        return v19;
                    }
                    goto LABEL_406ccf;
                }
            }
            v35 = count;
            node -= v35;
            *(a11) = v29;
            if (node != v35)
            {
                v19 = v29;
            }
            else
            {
                v40 = iter;
                break;
            }
        }
        else
        {
            v40 = iter;
            v29 = v19;
            break;
        }
    }
    if ((char)v29)
        return sub_406910(a1, a8, a5, v40);
    return 1;
}



// Function: fchmod_or_lchmod @ 0x7020
int fchmod_or_lchmod(int a0, unsigned int a1, unsigned long a2, char a3)
{
    if (a0 >= 0)
        return fchmod(a0, a3);
    if (a0)
        goto LABEL_407033;
LABEL_407033:
    *((char *)a1) = *((char *)a1) + a3;
}



// Function: copy_attr @ 0x7040
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

extern unsigned long long g_406220;

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
        v5 = sub_407260;
        if (!v4)
        {
            v5 = sub_407260;
            goto LABEL_407083;
        }
    }
    else if (!a4->field_37)
    {
        v5 = sub_407180;
LABEL_407083:
        v0 = v5;
        v1 = sub_406260;
        v2 = &g_406220;
    }
    return ((a1 | a3) < 0 ? !attr_copy_file() : !attr_copy_fd());
}



// Function: copy_attr_error @ 0x7180
void copy_attr_error(unsigned long a0, unsigned long a1)
{
    unsigned long long v11;  // rdx
    unsigned long long v12;  // rcx
    unsigned long long v13;  // r8
    unsigned long long v14;  // r9
    unsigned long v16;  // rsi
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
    if (!(char)sub_4061d0(v16 & 0xffffffff, v16))
    {
        v2 = &v10;
        v0 = 16;
        v1 = 48;
        v3 = &v5;
        sub_416760(0);
    }
    if (v4 != *((long long *)(40 + v17)))
        __stack_chk_fail(); /* do not return */
    return;
}



// Function: copy_attr_allerror @ 0x7260
long long copy_attr_allerror(unsigned long a0, long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v12;  // fs
    unsigned int *err;  // rax
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

    v6 = a2;
    v7 = a3;
    v8 = a4;
    v9 = a5;
    v4 = *((long long *)(40 + v12));
    err = __errno_location();
    v0 = 16;
    v2 = &v10;
    v3 = &v5;
    v1 = 48;
    sub_416760(0, *(err), a1, &v0);
    if ((unsigned int)v4 != *((int *)40))
        __stack_chk_fail(); /* do not return */
    return (unsigned int)v4 - *((int *)0x28);
}



// Function: check_selinux_attr @ 0x7330
void check_selinux_attr(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_407349();
    return;
}



// Function: restore_default_fscreatecon_or_die @ 0x7380
void restore_default_fscreatecon_or_die(void)
{
    char *v1;  // rax
    int *err;  // rax

    if (!setfscreatecon(0))
        return;
    v1 = dcgettext(NULL, "failed to restore the default file creation context", 5);
    err = __errno_location();
    error(1, *(err), v1);
}



// Function: emit_verbose @ 0x73c0
int emit_verbose(long long a0, long long a1, long long a2)
{
    char *v1;  // rax

    v1 = sub_414a70(1, 4, a1);
    __printf_chk(1, "%s -> %s", sub_414a70(0, 4, a0), v1);
    if (a2)
    {
        sub_414b40(4, a2);
        __printf_chk(1, dcgettext(NULL, " (backup: %s)", 5));
    }
    return putchar_unlocked(10);
}



// Function: create_hard_link @ 0x7460
unsigned int create_hard_link(unsigned long a0, unsigned int a1, unsigned long a2, long long a3, unsigned int a4, long long a5, char a6, unsigned int a7, char a8)
{
    unsigned long v1;  // r14
    unsigned long v2;  // r12
    unsigned int v3;  // ebp
    unsigned long v5;  // r9
    unsigned long v0;  // [bp-0x40]

    v1 = a0;
    v2 = a7;
    v3 = sub_40c0c0(a1, a2, a4, a5, a8 * 0x400, a6, -0x1);
    if (v3 > 0)
    {
        v5 = 0;
        if (!v1)
        {
            v1 = sub_4065e0(a3, a5, a2);
            v5 = v1;
        }
        v0 = v5;
        sub_414a70(1, 4, v1);
        sub_414a70(0, 4, a3);
        error(0, v3, dcgettext(NULL, "cannot create hard link %s to %s", 5));
        free(v0);
        return 0;
    }
    else if (!((char)v2 & (char)(v3 >> 31)))
    {
        return 1;
    }
    else
    {
        sub_414b40(4, a3);
        __printf_chk(1, dcgettext(NULL, "removed %s\n", 5));
        return _INSERT(v2, 0, (char)v2 & (char)(v3 >> 31));
    }
}



// Function: set_process_security_ctx @ 0x7ff0
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

unsigned int set_process_security_ctx(long long a0, unsigned long long a1, unsigned long a2, char a3, struct_0 *a4)
{
    unsigned long v3;  // r12
    int *err;  // rax
    char *v13;  // rax
    long long v14;  // rdi
    void* err1;  // rax
    char *v16;  // rax
    void* err2;  // rax
    unsigned long v6;  // cc_op
    unsigned long v7;  // cc_dep1
    unsigned long v8;  // cc_dep2
    unsigned long v9;  // cc_ndep
    unsigned long long v10;  // 4106
    unsigned long long v11;  // rcx
    char *v12;  // rax
    unsigned long long v0;  // [bp-0x40]
    long long v1;  // [bp-0x38]

    v3 = a4->field_33;
    if ((char)v3)
    {
        if (!a4->field_31 || a4->field_34)
        {
            if (sub_4158e0(a0, &v1) < 0)
            {
                err = __errno_location();
LABEL_408048:
                sub_414b40(4, a0);
                v12 = dcgettext(NULL, "failed to get security context of %s", 5);
                error(0, *(err), v12);
                goto LABEL_40807e;
            }
            else
            {
                if (setfscreatecon(v1) >= 0)
                {
                    freecon(v1);
                    return v3;
                }
                err2 = __errno_location();
LABEL_4081fd:
                sub_414e00(v1);
                v13 = dcgettext(NULL, "failed to set default file creation context to %s", 5);
                error(0, *((int *)err2), v13);
            }
        }
        else
        {
            if (a4->field_37)
            {
                if (sub_4158e0() < 0)
                    goto LABEL_40807e;
                if (setfscreatecon(v1) >= 0)
                {
                    freecon(v1);
                    return v3;
                }
            }
            else if (sub_4158e0() < 0)
            {
                err = __errno_location();
                v10 = _ccall(5, v6, v7, v8, v9);
                if (v11 != 1 & v10 & 1)
                    goto LABEL_0x408101;
                __unsupported_jumpkind_Ijk_NoDecode()
                if (!*((char *)&err))
                    goto LABEL_408048;
LABEL_40807e:
                if (!a4->field_34)
                    return v3;
                return 0;
            }
            else
            {
                if (setfscreatecon(v1) >= 0)
                {
                    freecon(v1);
                    return v3;
                }
                err2 = __errno_location();
                if (!(char)sub_4061d0(*((int *)err2)))
                    goto LABEL_4081fd;
            }
        }
        v1 = v1;
        if (!a4->field_34)
        {
            freecon(v1);
            return v3;
        }
        freecon(v1);
        return 0;
    }
    else
    {
        v14 = a4->field_28;
        if (!(v14 & a3))
            return 1;
        v0 = a1;
        if (sub_40c760(v14) < 0)
        {
            err1 = __errno_location();
            if (!(char)sub_4061d0(*((int *)err1)))
            {
                sub_414b40(4);
                v16 = dcgettext(NULL, "failed to set default file creation context for %s", 5);
                error(0, *((int *)err1), v16);
                return _INSERT(v3, 0, v14 & a3);
            }
        }
        return 1;
    }
}



// Function: set_file_security_ctx @ 0x8250
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
    unsigned int v1;  // r13d
    void* err;  // rax
    char *v3;  // rax

    if (!a2->field_31 || a2->field_34)
    {
        if ((char)sub_40c960(a2->field_28, a0, a1))
            return 1;
        err = __errno_location();
    }
    else if (a2->field_37)
    {
        return sub_40c960();
    }
    else
    {
        v1 = sub_40c960();
        if ((char)v1)
            return 1;
        err = __errno_location();
        if ((char)sub_4061d0(*((int *)err)))
            return v1;
    }
    sub_414a70(0, 4, a0);
    v3 = dcgettext(NULL, "failed to set the security context of %s", 5);
    error(0, *((int *)err), v3);
    return v1;
}



// Function: dest_info_init @ 0x8330
typedef struct struct_0 {
    char padding_0[72];
    unsigned long long field_48;
} struct_0;

extern long long g_4125b0;

void dest_info_init(struct_0 *a0)
{
    unsigned long long v1;  // rax

    v1 = sub_411cd0(61, 0, sub_412540, sub_412510, &g_4125b0);
    a0->field_48 = v1;
    if (!v1)
        sub_417250(); /* do not return */
    return;
}



// Function: src_info_init @ 0x8370
typedef struct struct_0 {
    char padding_0[80];
    unsigned long long field_50;
} struct_0;

extern long long g_4125b0;

void src_info_init(struct_0 *a0)
{
    unsigned long long v1;  // rax

    v1 = sub_411cd0(61, 0, sub_4124f0, sub_412510, &g_4125b0);
    a0->field_50 = v1;
    if (!v1)
        sub_417250(); /* do not return */
    return;
}



// Function: cp_options_default @ 0x83b0
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



// Function: chown_failure_ok @ 0x8400
typedef struct struct_0 {
    char padding_0[26];
    char field_1a;
} struct_0;

int chown_failure_ok(struct_0 *a0)
{
    unsigned int *err;  // rax
    unsigned int v2;  // edx

    err = __errno_location();
    v2 = *(err);
    if (v2 != 1 && v2 != 22)
        return _INSERT(err, 0, v2 == 1 | v2 == 22);
    return a0->field_1a ^ 1;
}



// Function: cached_umask @ 0x8640
extern unsigned int g_425010;

unsigned int cached_umask(void)
{
    if (g_425010 == 0xffffffff)
    {
        g_425010 = umask(0);
        umask(g_425010);
    }
    return g_425010;
}



// Function: copy_reg @ 0x8680
typedef struct struct_0 {
    long long field_0;
    char padding_8[16];
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
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

unsigned int copy_reg(long long a0, unsigned long a1, unsigned int a2, long long a3, void* idx, unsigned int a5, unsigned int a6, char *a7, struct_0 *a8)
{
    unsigned int v18;  // eax
    unsigned int v19;  // ebp
    unsigned long long v28;  // r9
    unsigned int v29;  // r9d
    char v30;  // al
    char v31;  // al
    int v33;  // r10d
    char *v34;  // rax
    unsigned int v35;  // eax
    unsigned int v36;  // eax
    unsigned int v37;  // eax
    unsigned int v38;  // ecx
    void* v39;  // rax
    void* v40;  // rcx
    int v41;  // r10d
    struct_0 *v42;  // rsi
    unsigned int v43;  // r15d
    long long v44;  // r11
    int v45;  // r10d
    unsigned long v46;  // rcx
    unsigned long v47;  // r11
    unsigned int v21;  // r14d
    long long v48;  // rax
    char v49;  // r9b
    long long v50;  // rdx
    int v51;  // r10d
    char v52;  // al
    char *v53;  // rsi
    char *v54;  // rax
    unsigned long long v55;  // rdx
    char *v56;  // rax
    unsigned long v57;  // rax
    int v22;  // r10d
    unsigned long v58;  // rdx
    unsigned int v59;  // eax
    int v60;  // r10d
    char *v61;  // rax
    char *v62;  // rax
    char *v63;  // rax
    char *v64;  // rax
    char *v65;  // rax
    int *err;  // rax
    char *v67;  // rax
    unsigned int v23;  // r8d
    int *err1;  // rax
    unsigned int v24;  // r14d
    char *v26;  // rax
    unsigned long long v27;  // r13
    char *v0;  // [bp-0x1e8], Other Possible Types: unsigned int, unsigned long long, int
    void* v1;  // [bp-0x1d8], Other Possible Types: char, unsigned int, int
    char v2;  // [bp-0x1c8], Other Possible Types: int
    unsigned int v3;  // [bp-0x1c4]
    int *err2;  // [bp-0x1c0]
    unsigned long v5;  // [bp-0x1b8], Other Possible Types: unsigned int, int
    unsigned long v6;  // [bp-0x1b0], Other Possible Types: int, unsigned int
    char v7;  // [bp-0x1a1]
    unsigned long long v8;  // [bp-0x1a0]
    long long v9;  // [bp-0x198]
    int v10;  // [bp-0x190], Other Possible Types: unsigned long long
    unsigned long v11;  // [bp-0x188]
    unsigned long long v12;  // [bp-0x180]
    unsigned long long v13;  // [bp-0x178]
    unsigned long long v14;  // [bp-0x170]
    int v15;  // [bp-0x168], Other Possible Types: stat
    stat v16;  // [bp-0xd8]
    char v17;  // [bp-0x41]

    v0 = a7;
    v8 = 0;
    v3 = a8->field_18;
    v1 = (char)idx[49];
    v2 = (char)idx[53];
    v18 = sub_40e880();
    if (v18 < 0)
    {
        sub_414b40(4, a0);
        v67 = dcgettext(NULL, "cannot open %s for reading", 5);
        err1 = __errno_location();
        error(0, *(err1), v67);
        return 0;
    }
    v19 = v18;
    if (fstat(v18, &v16))
    {
        sub_414b40(4, a0);
        v64 = dcgettext(NULL, "cannot fstat %s", 5);
        v0 = *(__errno_location());
        goto LABEL_408779;
    }
    if (*((long long *)&a8->padding_8[0]) != v16.st_ino || a8->field_0 != *((unsigned long long *)&v16))
    {
        sub_414b40(4, a0);
        v0 = 0;
        v64 = dcgettext(NULL, "skipping file %s, as it was replaced while being copied", 5);
        goto LABEL_408779;
    }
    v21 = *(v0);
    err2 = __errno_location();
    if (!(char)v21)
    {
        v22 = sub_412710(a2, a3, (-(v1 < 1) & 0xfffffe00) + 513);
        v23 = *(err2);
        if (v22 >= 0)
        {
            if (((long long)idx[40] || (char)idx[0x33]) && (v5 = v22, v22 = v5, v21 = (unsigned int)(unsigned long long)sub_408250(a1, 0, idx), !(char)v21 && (char)idx[52]))
                goto LABEL_408ba8;
            if (!*(v0))
            {
                a6 = 0;
                v24 = 0;
                goto LABEL_4088bc;
            }
        }
        else if (v23 == 2)
        {
LABEL_408ca8:
            if ((long long)idx[40] && !(v21 = (unsigned int)(unsigned long long)sub_407ff0(a0, a1, (unsigned long long)a5, 1, idx), (char)v21))
                goto LABEL_4092ad;
            *(v0) = 1;
        }
        else if ((char)idx[22])
        {
            if (!unlinkat(a2, a3, 0) && (char)idx[60])
            {
                sub_414b40(4, a1);
                __printf_chk(1, dcgettext(NULL, "removed %s\n", 5));
                goto LABEL_408ca8;
            }
            if (*(err2) == 2)
                goto LABEL_408ca8;
            sub_414b40(4, a1);
            v26 = dcgettext(NULL, "cannot remove %s", 5);
            v27 = 0;
            error(0, *(err2), v26);
            goto LABEL_408788;
        }
        else if (!*(v0))
        {
            goto LABEL_408d49;
        }
    }
    v28 = ~(a6) & a5;
    if (v2 && !(char)idx[27])
        v28 |= 128;
    v5 = v28;
    v29 = v5;
    v22 = sub_412710(a2, a3, 193, v28 & 0xffffffff);
    v23 = *(err2);
    v30 = v22 >> 31;
    if (v23 != 0x11 || !v30)
    {
        v31 = v30 & v23 == 21;
        goto LABEL_408da9;
    }
    v23 = 0x11;
    if ((char)idx[24] || (v5 = 0x11, v6 = v29, v23 = v5, (unsigned long long)readlinkat((unsigned long long)a2, a3, &v17, 1) < 0))
        goto LABEL_408d49;
    v21 = (char)idx[62];
    if ((char)v21)
    {
        v5 = v6;
        v29 = v5;
        v22 = sub_412710(a2, a3, 65, v5);
        v23 = *(err2);
        v31 = v23 == 21 & (char)(v22 >> 31);
LABEL_408da9:
        if (v31)
        {
            v23 = 21;
            if (*((char *)a1))
                v23 = (unsigned int)((*((char *)(a1 + strlen(a1) - 1)) != 47) + 20);
        }
        else
        {
            v24 = ~(a5) & v29;
            if (v22 >= 0)
            {
LABEL_4088bc:
                v33 = v22;
                if (v1)
                {
                    if (!(int)idx[0x44])
                        goto LABEL_4088d2;
                    v5 = v22;
                    v33 = v5;
                    if (!sub_406770(v22, v19))
                        goto LABEL_408de0;
                    v22 = v33;
                    if ((int)idx[0x44] != 2)
                        goto LABEL_4088d2;
                    v21 = 0;
                    v0 = v33;
                    sub_414a70(1, 4, a0);
                    sub_414a70(0, 4, a1);
                    v34 = dcgettext(NULL, "failed to clone %s from %s", 5);
                    error(0, *(err2), v34);
                    v22 = v0;
                }
                else
                {
LABEL_408de0:
                    v22 = v33;
                    v35 = (char)idx[29];
                    v1 = v35 | v24;
                    if (!v35 && !v24)
                    {
                        *((unsigned int *)&(&v15)[24]) = 0;
                        if (!(char)idx[31])
                        {
LABEL_408b49:
                            v60 = v22;
                            if (!v2 || (v2 = v22, v60 = v2, sub_407040(a0, v19, a1, v22, idx)))
                                v21 = 1;
                            else
                                v21 = (char)idx[54] ^ 1;
                            if (0xff0000000000ff & (long long)idx[24])
                            {
                                v22 = v60;
                                if (sub_40caa0(a0, v19, a1, v60, v3))
                                {
                                    if ((char)idx[50])
                                        v21 = 0;
                                }
                            }
                            else if ((char)idx[57])
                            {
                                v0 = v60;
                                v22 = v0;
                                if (sub_40cb50(a1, v60, (int)idx[16]))
                                    v21 = 0;
                            }
                            else if (!(char)idx[32] || !*(v0))
                            {
                                v22 = v60;
                                if (v1 || a6)
                                {
                                    v0 = v60;
                                    v22 = v0;
                                    if (~((unsigned int)sub_408640()) & a6 || v1)
                                    {
                                        v22 = v0;
                                        if (sub_407020(v0, a2, a3, (char)a5 & (char)~((unsigned int)sub_408640())))
                                        {
                                            v61 = sub_414b40(4, a1);
                                            *((char *)v40 - 0x77) = *((char *)v40 - 0x77) + *((char *)&v40);
                                            error(0, *(err2), v61);
                                            v22 = v0;
                                            if ((char)idx[50])
                                                v21 = 0;
                                        }
                                    }
                                }
                            }
                            else
                            {
                                v0 = v60;
                                v22 = v0;
                                if (sub_40cb50(a1, v0, ~((unsigned int)sub_408640()) & 438))
                                    v21 = (char)idx[57];
                            }
                        }
                        else
                        {
LABEL_408e06:
                            v5 = v22;
                            v11 = sub_415960(a8);
                            v12 = v55;
                            v14 = v55;
                            v13 = sub_415980(a8);
                            v22 = v5;
                            if (!sub_40e900(v5, a2, a3, &v11, 0))
                            {
LABEL_408ad0:
                                if (!(char)idx[29] || (v57 = (unsigned long)(unsigned long long)a8->field_1c, v58 = (unsigned long)(unsigned long long)a8->field_20, (unsigned int)v57 == *((unsigned int *)((void*)&v15 + 28)) && *((unsigned int *)((void*)&v15 + 32)) == (unsigned int)v58))
                                    goto LABEL_408b49;
                                v21 = 0;
                                v5 = v22;
                                v59 = sub_408430(idx, a1, a2, a3, v22, a8->field_18, v57, v58, *(v0), &v15);
                                v22 = v5;
                                if (v59 == 0xffffffff)
                                    goto LABEL_408ba8;
                                v3 = (v59 ? v3 : _INSERT((unsigned int)v55, 1, (char)v55 & 241));
                                goto LABEL_408b49;
                            }
                            else
                            {
                                sub_414b40(4, a1);
                                v56 = dcgettext(NULL, "preserving times for %s", 5);
                                error(0, *(err2), v56);
                                v22 = v5;
                                if (!(char)idx[50])
                                    goto LABEL_408ad0;
LABEL_408eb0:
                                v21 = 0;
                            }
                        }
                    }
                    else
                    {
                        v1 = 0;
LABEL_4088d2:
                        v5 = v22;
                        v36 = fstat(v22, &v15);
                        v22 = v5;
                        if (!v36)
                        {
                            v6 = v36;
                            v37 = v15.st_nlink;
                            v38 = v37 | v24;
                            if (v37 != v38)
                            {
                                v5 = v22;
                                if (sub_407020(v22, a2, a3, v38))
                                {
                                    v24 = v6;
                                    v22 = v5;
                                }
                                else
                                {
                                    v22 = v5;
                                }
                            }
                            if (v1)
                            {
                                v6 = v22;
                                v5 = sub_407fd0(*((unsigned long long *)(&v15 + 56)), &v16);
                                v39 = 0x200;
                                if (v40 - 1 <= 0x1fffffffffffffff)
                                    v39 = v40;
                                v1 = v39;
                                v41 = v6;
                                v43 = sub_406780(v19, v42, &v9);
                                if (v43)
                                {
                                    if (((unsigned short)v15.st_nlink & 0xf000) != 0x8000 || (int)idx[12] != 3 && ((int)idx[12] != 2 || v43 == 1))
                                    {
                                        v6 = v41;
                                        sub_40e840(v19, 0, 0, 2);
                                        v44 = v5;
                                        v45 = v6;
                                        v46 = sub_40dbf0(sub_407fd0(v16.st_mtime), v5, 0x7fffffffffffffff);
                                        if (((unsigned short)v16.st_nlink & 0xf000) == 0x8000 && v16.st_atime < v44)
                                            v44 = v16.st_atime + 1;
                                        v47 = v44 + v46 - 1 - (v44 + v46 - 1) % v46;
                                        v48 = (int)idx[0x44];
                                        if (v47 <= 0)
                                        {
                                            v7 = 0;
                                            if (v43 != 3)
                                            {
                                                v1 = NULL;
                                                v47 = v46;
                                                v49 = (int)idx[12] == 3;
                                                goto LABEL_408a67;
                                            }
                                            else
                                            {
                                                v47 = v46;
                                                v50 = 1;
LABEL_4091e4:
                                                v1 = v45;
                                                v51 = v1;
                                                v52 = (char)sub_407b50(v19, v45, &v8, v47, v1, v9, v16.st_atime, v50, v48, a0, a1) ^ 1;
                                            }
                                        }
                                        else
                                        {
                                            v7 = 0;
                                            v50 = 1;
                                            if (v43 == 3)
                                                goto LABEL_4091e4;
                                            v1 = NULL;
                                            v49 = (int)idx[12] == 3;
LABEL_408a67:
                                            v1 = v45;
                                            v51 = v1;
                                            v52 = (char)sub_406a40(v19, v45, &v8, v47, v1, v49, v48, a0, a1, 0xffffffffffffffff, &v10, &v7) ^ 1;
                                        }
                                    }
                                    else
                                    {
                                        v6 = v5;
                                        v5 = v41;
                                        sub_40e840(v19, 0, 0, 2);
                                        v7 = 0;
                                        v45 = v5;
                                        v47 = v6;
                                        if (v43 != 3)
                                        {
                                            v48 = (int)idx[0x44];
                                            v49 = (int)idx[12] == 3;
                                            goto LABEL_408a67;
                                        }
                                        else
                                        {
                                            v50 = (int)idx[12];
                                            v48 = (int)idx[0x44];
                                            goto LABEL_4091e4;
                                        }
                                    }
                                    v22 = v51;
                                    if (!v52)
                                    {
                                        v22 = v51;
                                        if (!v7 || !(v1 = v51, v22 = v1, (int)(long long)ftruncate(v51, v10) < 0))
                                            goto LABEL_408ab7;
                                        *((int *)&v0) = v22;
                                        sub_414b40(4, a1);
                                        v53 = "failed to extend %s";
                                        goto LABEL_4091a1;
                                    }
                                }
                                else
                                {
                                    *((int *)&v0) = v41;
                                    sub_414b40(4, a0);
                                    v53 = "cannot lseek %s";
LABEL_4091a1:
                                    v54 = dcgettext(NULL, v53, 5);
                                    error(0, *(err2), v54);
                                    v22 = v0;
                                    goto LABEL_408eb0;
                                }
                            }
                            else
                            {
LABEL_408ab7:
                                v1 = v24;
                                if ((char)idx[31])
                                    goto LABEL_408e06;
                                goto LABEL_408ad0;
                            }
                        }
                        else
                        {
                            v0 = v22;
                            v21 = 0;
                            sub_414b40(4, a1);
                            v62 = dcgettext(NULL, "cannot fstat %s", 5);
                            error(0, *(err2), v62);
                            v22 = v0;
                        }
                    }
                }
LABEL_408ba8:
                if (close(v22) >= 0)
                {
                    v27 = v8;
                    if (close(v19) >= 0)
                    {
                        sub_40cbd0(v27);
                        return v21;
                    }
LABEL_408bd0:
                    sub_414b40(4, a0);
                    v65 = dcgettext(NULL, "failed to close %s", 5);
                    err = __errno_location();
                    error(0, *(err), v65);
                    sub_40cbd0(v27);
                    return 0;
                }
                v21 = 0;
                sub_414b40(4, a1);
                v63 = dcgettext(NULL, "failed to close %s", 5);
                error(0, *(err2), v63);
                v27 = v8;
                goto LABEL_408788;
                goto LABEL_408788;
            }
        }
LABEL_408d49:
        v0 = v23;
        sub_414b40(4, a1);
        v64 = dcgettext(NULL, "cannot create regular file %s", 5);
LABEL_408779:
        v27 = 0;
        v21 = 0;
        error(0, v0, v64);
    }
    else
    {
        sub_414b40(4, a1);
        error(0, 0, dcgettext(NULL, "not writing through dangling symlink %s", 5));
LABEL_4092ad:
        v27 = 0;
    }
LABEL_408788:
    if (close(v19) >= 0)
    {
        sub_40cbd0(v27);
        return v21;
    }
    goto LABEL_408bd0;
}



// Function: copy_internal @ 0x9470
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

typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    unsigned int field_10;
    char field_14;
    char field_15;
    char field_16;
    char field_17;
    char field_18;
    char field_19;
    char padding_1a[2];
    char field_1c;
    char field_1d;
    char padding_1e[1];
    char field_1f;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char field_30;
    char field_31;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char padding_37[1];
    char field_38;
    char field_39;
    char field_3a;
    char field_3b;
    char field_3c;
    char padding_3d[2];
    char field_3f;
    int field_40;
    char padding_44[4];
    long long field_48;
    unsigned long long field_50;
} struct_0;

extern long long g_425568;
extern long long g_425570;

unsigned int copy_internal(char *a0, char *a1, unsigned int a2, long long a3, int a4, unsigned long long *a5, unsigned long a6, struct_0 *idx, long long a8, char *a9, char *a10, unsigned long long a11)
{
    unsigned int v34;  // eax
    int v35;  // r14d
    char v44;  // al
    unsigned long v129;  // r9
    char *v130;  // rax
    int *err;  // rax
    char *v132;  // rax
    char v45;  // r11b
    unsigned int v46;  // eax
    int *err1;  // rax
    char *v48;  // rax
    unsigned int v49;  // eax
    char *v50;  // rsi
    long long v51;  // r14
    unsigned long long v52;  // rax
    unsigned int v53;  // r15d
    int v36;  // r12d
    unsigned int v54;  // r12d
    unsigned int v55;  // ecx
    unsigned int v56;  // edx
    unsigned int v57;  // edx
    int *err2;  // r12
    char *v59;  // rsi
    void* v61;  // rdi
    char *v62;  // r12
    long long v63;  // r14
    char *n;  // r15
    void* ptr;  // r12
    unsigned long v65;  // r14
    char *len;  // rax
    unsigned long long v67;  // rax
    char *i;  // rsi
    unsigned long long v69;  // rcx
    struct_1 *v71;  // rdx
    char v72;  // al
    char *v73;  // r14
    char *v38;  // rdx
    unsigned long v74;  // rax
    unsigned int v76;  // r12d
    int *v77;  // rax
    char *v78;  // rax
    int v79;  // esi
    char *v80;  // rdx
    unsigned int v82;  // r12d
    char v83;  // r14b
    char *v39;  // rsi
    unsigned long long v86;  // rax
    unsigned long long v89;  // r15
    unsigned long ptr1;  // r12
    unsigned int v91;  // eax
    char *v92;  // rsi
    unsigned int v93;  // eax
    unsigned int v40;  // eax
    char *v95;  // rsi
    unsigned long ptr2;  // r12
    unsigned int v97;  // eax
    void* v98;  // rax
    char v100;  // al
    char *v101;  // rax
    int *v102;  // rax
    unsigned long v103;  // rax
    char v41;  // r11b
    unsigned int v104;  // r12d
    long long v105;  // rdx
    unsigned long long v106;  // rax
    char v107;  // r8b
    char v108;  // r8b
    char v109;  // al
    unsigned int v110;  // eax
    unsigned int v111;  // r15d
    char v112;  // r10b
    unsigned int v42;  // eax
    unsigned long v113;  // r9
    unsigned long long v115;  // rdx
    char *v116;  // rax
    int *v117;  // rax
    unsigned long v118;  // rax
    unsigned long v119;  // rdx
    unsigned int v120;  // eax
    char v43;  // dl
    unsigned int v121;  // eax
    char *v122;  // rsi
    char *v123;  // rax
    int *v124;  // rax
    char *v125;  // rax
    int *v126;  // rax
    uint8_t v134[*((long long *)((char *)stack_base - 688))];  // alloca
    char v0;  // [bp-0x2308]
    int v2;  // [bp-0x450]
    int v3;  // [bp-0x420]
    unsigned int v4;  // [bp-0x304]
    unsigned int v5;  // [bp-0x300], Other Possible Types: unsigned long, char
    unsigned int v6;  // [bp-0x2f8]
    char v7;  // [bp-0x2f1]
    void* index;  // [bp-0x2d8], Other Possible Types: unsigned long long
    void* v9;  // [bp-0x2d0], Other Possible Types: unsigned long
    char *v10;  // [bp-0x2c8], Other Possible Types: unsigned long long, unsigned long, char
    char *v11;  // [bp-0x2c0], Other Possible Types: unsigned long long, unsigned long
    unsigned long long v12;  // [bp-0x2b8], Other Possible Types: unsigned long
    unsigned int v13;  // [bp-0x29c]
    char v14;  // [bp-0x28a]
    char v15;  // [bp-0x289]
    unsigned long long v16;  // [bp-0x288]
    long long v17;  // [bp-0x280]
    unsigned long long v18;  // [bp-0x278]
    unsigned int v19;  // [bp-0x270]
    unsigned int v20;  // [bp-0x26c]
    unsigned int v21;  // [bp-0x268]
    long long v22;  // [bp-0x260]
    long long v23;  // [bp-0x258]
    int v24;  // [bp-0x1f8], Other Possible Types: struct_0
    unsigned int v25;  // [bp-0x1dc]
    unsigned int v26;  // [bp-0x1d8]
    unsigned long v27;  // [bp-0x1c8]
    stat v28;  // [bp-0x168]
    char v29;  // [bp-0xd8], Other Possible Types: unsigned long
    unsigned long long v30;  // [bp-0xd0]
    unsigned long long v31;  // [bp-0xc8]
    unsigned long long v32;  // [bp-0xc0]
    long long v33;  // [bp+0x0]

    v9 = a6;
    v34 = a8;
    *((unsigned int *)&v12) = v34;
    v35 = idx->field_40;
    v11 = a11;
    *((char *)&index) = v34;
    *(a10) = 0;
    v14 = a4 > 0;
    if (idx->field_18)
    {
        if (v35 >= 0)
        {
LABEL_409510:
            v14 = !v35;
            *((char *)v11) = !v35;
            v36 = !v35;
            if (v35)
                goto LABEL_409560;
            goto LABEL_409531;
        }
        else if (!sub_414e40(0xffffff9c, a0, a2, a3, 1))
        {
            v14 = 1;
            v36 = 1;
            *((char *)v11) = 1;
            goto LABEL_409531;
        }
        else
        {
            v35 = *(__errno_location());
            goto LABEL_409510;
        }
    }
    v36 = a4;
    if (!v35)
    {
LABEL_409531:
        if (!idx->field_3f)
        {
            n = a1;
            v35 = 0;
            goto LABEL_409575;
        }
        else
        {
            v35 = 0;
LABEL_4095b7:
            if (!(char)v12)
                goto LABEL_409767;
            goto LABEL_4095c4;
        }
    }
LABEL_409560:
    if (v35 != 0x11 || idx->field_8 != 2)
    {
        n = a0;
LABEL_409575:
        if ((unsigned int)(unsigned long long)sub_406360())
            goto LABEL_40a440;
        v13 = v19;
        if (((unsigned short)v19 & 0xf000) != 0x4000 || !(n = (char *)(unsigned long long)idx->field_38, !*((char *)(void*)&n)))
            goto LABEL_4095b7;
        sub_414b40(4, a0);
        if (!idx->field_19)
        {
            v38 = dcgettext(NULL, "-r not specified; omitting directory %s", 5);
            goto LABEL_40a3de;
        }
        else
        {
            v39 = "omitting directory %s";
            goto LABEL_40b09c;
        }
    }
    if ((char)v12)
    {
LABEL_4095c4:
        if (!idx->field_50)
        {
LABEL_4095fd:
            /* unsupported instruction */ = (int)(&v2 - 320);
            v7 = sub_407fb0(idx->field_4, 1);
            if (v36 > 0)
            {
                v10 = 0;
                v41 = 0;
            }
        }
        if (((unsigned short)v13 & 0xf000) == 0x4000 || idx->field_0)
        {
LABEL_4095f2:
            sub_40ea00();
            goto LABEL_4095fd;
        }
        else
        {
            v10 = &v16;
            v40 = sub_40ea90();
            n = v40;
            if (!(char)v40)
            {
                goto LABEL_4095f2;
            }
            else
            {
                sub_414b40(4, a0);
                v39 = "warning: source file %s specified more than once";
                goto LABEL_40b09c;
            }
        }
    }
LABEL_409767:
    /* unsupported instruction */ = (int)(&v2 - 320);
    v7 = sub_407fb0(idx->field_4, 0);
    if (v36 > 0)
    {
        v10 = 0;
        goto LABEL_409668;
    }
    if (v35 != 0x11 || idx->field_8 != 2)
    {
        v42 = v13 & 0xf000;
        if (v42 != 0x8000 && !(v43 = (char)(char)(v42 == 0xa000) | (char)(char)(v42 == 0x4000), v44 = idx->field_14 ^ 1, v45 = v43 | v44, !v43 && !v44) || (v45 = idx->field_18, v45 || (v45 = idx->field_3a, v45 || (v45 = idx->field_17, v45))))
        {
LABEL_40989c:
            goto LABEL_4098a2;
        }
        if (!idx->field_0)
        {
            v45 = idx->field_15;
            if (v45)
                goto LABEL_40989c;
            if (!v36)
                goto LABEL_4098a2;
LABEL_40a36b:
            v14 = 1;
LABEL_40a372:
            v10 = 0;
            v41 = 0;
            if (v35 != 0x11)
                goto LABEL_40a38a;
            v46 = idx->field_8;
LABEL_4098db:
            v15 = 0;
            if (v46 == 2)
                goto LABEL_409935;
            v10 = v41;
            v3 = (int)_INSERT(*((uint2304_t *)(&v2 + 264)), 264, &v15);
            v49 = sub_4075b0(a0, &v16, a2, a3, &v24, idx, *((unsigned long long *)(&v3 + 264)));
            v41 = v10;
            n = v49;
            /* unsupported instruction */ = (int)(&v3 - 280);
            if ((char)v49)
                goto LABEL_409935;
            sub_414a70(1, 4, a1);
            sub_414a70(0, 4, a0);
            v50 = "%s and %s are the same file";
            goto LABEL_40abe0;
        }
        else
        {
            v45 = 1;
LABEL_4098a2:
            n = &v24;
            *((char *)&v10) = v45;
            /* unsupported instruction */ = (int)(&v2 - 320);
            if (!(unsigned int)(unsigned long long)sub_406360())
            {
                v46 = idx->field_8;
                v41 = v10;
                goto LABEL_4098db;
            }
            /* unsupported instruction */ = (int)(&v2 - 320);
            err1 = __errno_location();
            if (*(err1) != 40)
            {
                if (*(err1) != 2)
                    goto LABEL_40a2dc;
                goto LABEL_40a36b;
            }
            else
            {
                if (idx->field_16)
                    goto LABEL_40a372;
LABEL_40a2dc:
                sub_414b40(4, a1);
                v48 = dcgettext(NULL, "cannot stat %s", 5);
                error(0, *(err1), v48);
                return 0;
            }
        }
    }
    v15 = 0;
    v41 = 0;
LABEL_409935:
    if (!idx->field_3b)
    {
        if (idx->field_18)
            goto LABEL_40aa6a;
        if ((v13 & 0xf000) != 0x4000)
            goto LABEL_40a62c;
    }
    if (((unsigned short)v13 & 0xf000) == 0x4000)
    {
LABEL_40aa6a:
LABEL_40aa71:
        v10 = v41;
        /* unsupported instruction */ = (int)(&v3 - 272);
        v41 = v10;
        v53 = sub_406530(idx, a1, a2, a3, &v24);
        v86 = v11;
        if (v86)
            goto LABEL_40aaba;
        goto LABEL_409ab0;
    }
    v10 = v41;
    /* unsupported instruction */ = (int)(&v3 - 272);
    v41 = v10;
    if (sub_415e70(a2, a3, &v24, &v16) >= 0)
    {
        if (v11)
            *((char *)v11) = 1;
        v51 = a3;
        v52 = sub_40bf60(v51, v17, v16);
        if (!v52 || !(*((unsigned long long *)&(&v3)[264]) = (unsigned long long)v7, !(char)sub_407460(0, a2, v52, a1, a2, v51, 1, (unsigned int)(unsigned long long)idx->field_3c, *((char *)((void*)&v3 + 264)))))
            goto LABEL_409ab0;
        if (idx->field_33)
        {
            sub_407380();
            goto LABEL_409a38;
        }
    }
    if (idx->field_18)
        goto LABEL_40aa71;
LABEL_40a62c:
    if (idx->field_8 == 2 || idx->field_8 == 3 && !(v10 = v41, /* unsupported instruction */ = (int)(&v3 - 272), v41 = v10, (char)sub_4063e0(idx, a1, a2, a3, &v24)) || !(n = (char *)(unsigned long long)v15, !*((char *)(void*)&n)))
    {
LABEL_409ab0:
        return 1;
    }
    v54 = *((unsigned int *)((void*)&v24 + 24));
    v55 = v13 & 0xf000;
    if (((unsigned short)v54 & 0xf000) == 0x4000)
    {
        if (v55 == 0x4000)
        {
LABEL_40a6f4:
            v56 = idx->field_0;
LABEL_40a6f6:
            if (!idx->field_18)
            {
                v54 = *((unsigned int *)((void*)&v24 + 24));
                if (!v56)
                    goto LABEL_40a70f;
                goto LABEL_40b25f;
            }
        }
        else
        {
LABEL_40ae9c:
            if (!idx->field_18 || (v56 = idx->field_0, !v56))
            {
                sub_414b40(4, a1);
                v39 = "cannot overwrite directory %s with non-directory";
LABEL_40b09c:
                v38 = dcgettext(NULL, v39, 5);
LABEL_40a3de:
                error(0, 0, v38);
                return (unsigned int)n;
            }
            if ((v19 & 0xf000) == 0x4000)
                goto LABEL_40b487;
            goto LABEL_40aec6;
        }
    }
    if (v55 != 0x4000)
    {
        v56 = idx->field_0;
        if (!(char)v12)
            goto LABEL_40a6f6;
        goto LABEL_40a68f;
    }
    if (!idx->field_18 || (v56 = idx->field_0, !v56))
    {
        sub_414a70(1, 4, a0);
        sub_414a70(0, 4, a1);
        v50 = "cannot overwrite non-directory %s with directory %s";
LABEL_40abe0:
        error(0, 0, dcgettext(NULL, v50, 5));
        return (unsigned int)n;
    }
    if (!(char)v12)
    {
LABEL_40b0d1:
        if ((v19 & 0xf000) != 0x4000)
            goto LABEL_40b0e7;
        v54 = *((unsigned int *)((void*)&v24 + 24));
LABEL_40b487:
        if (((unsigned short)v54 & 0xf000) == 0x4000)
        {
LABEL_40b0e7:
            if (v56)
                goto LABEL_40aec6;
            goto LABEL_40a798;
        }
        else if (v56)
        {
LABEL_40aec6:
            v10 = v41;
            v5 = v56;
            /* unsupported instruction */ = (int)(&v3 - 272);
            v41 = v10;
            if (!(char)sub_406190(sub_40db60(a0)))
            {
                v57 = v5;
                goto LABEL_40aef9;
            }
        }
        else
        {
            sub_414c40(0, 3, a1);
            sub_414c40(0, 3, a0);
            v50 = "cannot move directory onto non-directory: %s -> %s";
            goto LABEL_40abe0;
        }
    }
LABEL_40a68f:
    v10 = _INSERT(v10 CONCAT 0, 0, v55);
    if (v56 != 3)
    {
        v5 = v41;
        /* unsupported instruction */ = (int)(&v3 - 272);
        v41 = v5;
        if (!(char)sub_40ea90(idx->field_48, a3, &v24))
        {
            if (v10 != 0x4000 && !(v54 = *((unsigned int *)((void*)&v24 + 24)), ((unsigned short)v54 & 0xf000) != 0x4000))
                goto LABEL_40ae9c;
            goto LABEL_40a6f4;
        }
        else
        {
            sub_414a70(1, 4, a0);
            sub_414a70(0, 4, a1);
            v50 = "will not overwrite just-created %s with %s";
            goto LABEL_40abe0;
        }
    }
    if (idx->field_18)
        goto LABEL_40b0d1;
LABEL_40b25f:
    *((unsigned int *)&v10) = v56;
    v5 = v41;
    /* unsupported instruction */ = (int)(&v3 - 272);
    v57 = v10;
    v41 = v5;
    if (!(char)sub_406190(sub_40db60(a0)))
    {
        if (((unsigned short)v54 & 0xf000) != 0x4000)
        {
LABEL_40aef9:
            if (v57 != 3)
            {
                v10 = v41;
                v41 = v10;
                if ((char)sub_406640(v61, &v16, a2, a3))
                {
                    v62 = (!idx->field_18 ? dcgettext(NULL, "backing up %s might destroy source;  %s not copied", 5) : dcgettext(NULL, "backing up %s might destroy source;  %s not moved", 5));
                    sub_414a70(1, 4, a0);
                    sub_414a70(0, 4, a1);
                    error(0, 0, v62);
                    return (unsigned int)n;
                }
            }
            v63 = a3;
            v10 = v41;
            ptr = sub_40da90(a2, v63, idx->field_0);
            if (ptr)
            {
                v65 = v63 - a1;
                len = strlen(ptr);
                n = len + 1;
                v67 = &len[v65 + 24];
                i = &v3 - 280 - (v67 & 0xfffffffffffff000);
                if (&v3 - 280 != i)
                {
                    do
                    { } while (&v0 != i);
                }
                v69 = (unsigned int)v67 & 0xfffffff0 & 0xfff;
                v5 = v10;
                v10 = (unsigned long long)(v134 + 15 & 0xfffffffffffffff0);
                memcpy(mempcpy(v10, a1, v65), ptr, n);
                free(ptr);
                v41 = v5;
            }
            else
            {
                v5 = v10;
                /* unsupported instruction */ = (int)(&v3 - 272);
                v41 = v5;
                v10 = 0;
                err2 = __errno_location();
                if (*(err2) != 2)
                {
                    sub_414b40(4, a1);
                    v59 = "cannot backup %s";
LABEL_40b95a:
                    v132 = dcgettext(NULL, v59, 5);
                    error(0, *(err2), v132);
                    return (unsigned int)n;
                }
            }
            v14 = 1;
            goto LABEL_40a7a3;
        }
LABEL_40a798:
        v10 = 0;
        goto LABEL_40a7a3;
    }
LABEL_40a70f:
    if (((unsigned short)v54 & 0xf000) == 0x4000 || !idx->field_15 && (!idx->field_31 || (!idx->field_30 || *((unsigned long long *)((void*)&v24 + 16)) <= 1) && (idx->field_4 != 2 || ((unsigned short)v19 & 0xf000) == 0x8000)))
        goto LABEL_40a798;
    *((char *)&v10) = v41;
    /* unsupported instruction */ = (int)(&v3 - 272);
    v41 = v10;
    if (unlinkat(a2, a3, 0) && (/* unsupported instruction */ = (int)(&v3 - 272), v41 = (char)v10, err2 = __errno_location(), *(err2) != 2))
    {
        sub_414b40(4, a1);
        v59 = "cannot remove %s";
        goto LABEL_40b95a;
    }
    v14 = 1;
    if (!idx->field_3c)
        goto LABEL_40a798;
    v5 = v41;
    sub_414b40(4, a1);
    /* unsupported instruction */ = (int)(&v3 - 272);
    __printf_chk(1, dcgettext(NULL, "removed %s\n", 5));
    v41 = v5;
    v10 = 0;
LABEL_40a7a3:
    v35 = 0x11;
LABEL_40a38a:
    if (!(char)v12 || !idx->field_48 || !(n = (char *)(unsigned long long)idx->field_18, !*((char *)(void*)&n)))
    {
LABEL_409668:
        if (!idx->field_3c || idx->field_18)
            goto LABEL_409678;
        goto LABEL_409f88;
    }
    if (!idx->field_0)
    {
        v71 = &v24;
        if (v41)
        {
LABEL_409652:
            if (((unsigned short)v71->field_18 & 0xf000) != 0xa000 || !(/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (char)(unsigned long long)sub_40ea90(idx->field_48, a3)))
                goto LABEL_409668;
            sub_414a70(1, 4, a1);
            sub_414a70(0, 4, a0);
            v50 = "will not copy %s through just-created symlink %s";
            goto LABEL_40abe0;
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (!fstatat(a2, a3, &v29, 0x100))
            {
                v71 = &v29;
                goto LABEL_409652;
            }
        }
    }
    if (idx->field_3c)
    {
LABEL_409f88:
        if (((unsigned short)v13 & 0xf000) != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            sub_4073c0(a0, a1, v10);
        }
LABEL_409678:
        if (v35)
            goto LABEL_409681;
        if (idx->field_18)
        {
LABEL_409a66:
            if (idx->field_3c)
            {
                __printf_chk(1, dcgettext(NULL, "renamed ", 5));
                sub_4073c0(a0, a1, v10);
            }
            if (idx->field_28)
                sub_408250(a1, 1, idx);
            if (v11)
                *((char *)v11) = 1;
            if ((char)v12 && !idx->field_3f)
            {
                sub_40ea00(idx->field_48, a3, &v16);
                return (char)v12;
            }
        }
        else
        {
LABEL_409ad8:
            v82 = v14;
            v5 = 0;
            v83 = v82;
            goto LABEL_409aee;
        }
        goto LABEL_409ab0;
    }
    if (!v35)
        goto LABEL_409ad8;
LABEL_409681:
    if (idx->field_38 && ((unsigned short)v13 & 0xf000) == 0x4000)
    {
        if (!(char)v12)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v5 = sub_40bf00(v17, v16, v16);
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v5 = sub_40bf60(a3, v17);
        }
        if (v5)
            goto LABEL_4096d5;
        goto LABEL_409740;
    }
    v72 = idx->field_18;
    if (v72)
    {
        if (v18 != 1 && (!idx->field_30 || idx->field_17))
        {
            v5 = 0;
            goto LABEL_4097c5;
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v5 = sub_40bf00(v17, v16);
LABEL_40a961:
        if (!v5)
        {
LABEL_409740:
            v72 = idx->field_18;
            goto LABEL_409f6b;
        }
        if (((unsigned short)v13 & 0xf000) != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v7;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = idx->field_3c;
            *((unsigned long *)(/* unsupported instruction */ - 16)) = 1;
            if (!(char)sub_407460(0, a2, v5, a1, a2, a3, v33, a6, *((char *)&idx)))
                goto LABEL_40a9c9;
            goto LABEL_409ab0;
        }
LABEL_4096d5:
        if ((char)sub_415130(0xffffff9c, a0, a2, v5))
        {
            sub_414a70(1, 4, g_425568);
            sub_414a70(0, 4, g_425570);
            error(0, 0, dcgettext(NULL, "cannot copy a directory, %s, into itself, %s", 5));
            *(a10) = 1;
            goto LABEL_40a9c9;
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 16;
        if (!(char)sub_415130(a2, a3, a2, v5))
        {
            if (idx->field_4 == 4 || idx->field_4 == 3 && (char)v12)
                goto LABEL_409740;
            v73 = a1;
            v74 = sub_4065e0(v73, a3, v5);
            sub_414a70(1, 4, v74);
            sub_414a70(0, 4, v73);
            error(0, 0, dcgettext(NULL, "will not create hard link %s to directory %s", 5));
            free(v74);
LABEL_40a9c9:
            if (!idx->field_33)
                goto LABEL_40a570;
LABEL_40a9d3:
            sub_407380();
LABEL_40a562:
            if (!v5)
                sub_40bea0(v17, v16);
LABEL_40a570:
            if (v10)
            {
                if (renameat(a2, a3 - a1 + v10, a2, a3))
                {
                    sub_414b40(4, a1);
                    v122 = "cannot un-backup %s";
LABEL_40a459:
                    v130 = dcgettext(NULL, v122, 5);
                    err = __errno_location();
                    error(0, *(err), v130);
                    return 0;
                }
                else if (idx->field_3c)
                {
                    sub_414a70(1, 4, a1);
                    sub_414a70(0, 4, v10);
                    __printf_chk(1, dcgettext(NULL, "%s -> %s (unbackup)\n", 5));
                    goto LABEL_409a38;
                }
            }
        }
        else
        {
            sub_414b40(4, g_425570);
            error(0, 0, dcgettext(NULL, "warning: source directory %s specified more than once", 5));
            v53 = _INSERT(n, 0, v11 & idx->field_18);
            if (v11 & idx->field_18)
            {
                v86 = v11;
LABEL_40aaba:
                *((char *)v86) = 1;
                return v53;
            }
            goto LABEL_409ab0;
        }
    }
    if (!idx->field_30)
        goto LABEL_409ad8;
    if (!idx->field_17)
    {
        if (v18 > 1 || (char)v12 && idx->field_4 == 3 || (v5 = 0, idx->field_4 == 4))
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v5 = sub_40bf60(a3, v17, v16);
            goto LABEL_40a961;
        }
LABEL_409f6b:
        if (!v72)
            goto LABEL_409f73;
LABEL_4097c5:
        if (v35 == 0x11 && (!renameat(0xffffff9c, a0, a2, a3) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 16), v35 = *(__errno_location()), !v35)))
            goto LABEL_409a66;
        switch (v35)
        {
        case 22:
            sub_414a70(1, 4, g_425568);
            sub_414a70(0, 4, g_425570);
            error(0, 0, dcgettext(NULL, "cannot move %s to a subdirectory of itself, %s", 5));
            *(a10) = 1;
            goto LABEL_409ab0;
        case 18:
            v76 = v13 & 0xf000;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if (!unlinkat(a2, a3, (v76 == 0x4000) * 0x200) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v77 = __errno_location(), *(v77) == 2))
            {
                if (idx->field_3c && v76 != 0x4000)
                {
                    __printf_chk(1, dcgettext(NULL, "copied ", 5));
                    /* unsupported instruction */ = /* unsupported instruction */ - 24;
                    sub_4073c0(a0, a1, v10);
                }
                v14 = 1;
                v82 = 1;
                v83 = 1;
                goto LABEL_409aee;
            }
            else
            {
                sub_414a70(1, 4, a1);
                sub_414a70(0, 4, a0);
                v78 = dcgettext(NULL, "inter-device move failed: %s to %s; unable to remove target", 5);
                v79 = *(v77);
                v80 = v78;
LABEL_40a84a:
                error(0, v79, v80);
                sub_40bea0(v17, v16);
                break;
            }
        default:
            sub_414a70(1, 4, a1);
            sub_414a70(0, 4, a0);
            v79 = v35;
            v80 = dcgettext(NULL, "cannot move %s to %s", 5);
            goto LABEL_40a84a;
        }
        goto LABEL_409a38;
    }
    v5 = 0;
LABEL_409f73:
    v82 = v14;
    v83 = v82;
LABEL_409aee:
    *((unsigned int *)&v11) = v13 & 0xfff;
    if (idx->field_39)
        *((unsigned int *)&v11) = idx->field_10 & 0xfff;
    if (!idx->field_1d)
    {
        v89 = v13 & 0xf000;
        if ((unsigned int)v89 != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v6 = 0;
            if ((char)sub_407ff0(a0, a1, v13, v82, idx))
                goto LABEL_409bba;
            goto LABEL_409a38;
        }
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        if (!(char)sub_407ff0(a0, a1, v13, v82, idx))
            goto LABEL_409a38;
        v6 = (unsigned int)v11 & 18;
LABEL_40a06a:
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        v7 = sub_4061e0(&v16, v9);
        if (v7)
        {
            sub_414b40(4, a0);
            v92 = "cannot copy cyclic symbolic link %s";
            goto LABEL_40ae6c;
        }
        if (!1)
            goto LABEL_0x40a098;
        /* unsupported instruction */ = /* unsupported instruction */ - 32;
        index = /* unsupported instruction */ + 15 & 0xfffffffffffffff0;
        *((unsigned long *)index) = v9;
        *((long long *)&index[8]) = v17;
        *((unsigned long long *)&index[16]) = v16;
        if (v83 || ((unsigned short)*((unsigned int *)((void*)&v24 + 24)) & 0xf000) != 0x4000)
        {
            v9 = v129;
            if (!mkdirat(a2, a3, ~(v6) & (unsigned int)v11))
            {
                v11 = v9;
                /* unsupported instruction */ = /* unsupported instruction */ - 16;
                v113 = v11;
                if (!fstatat(a2, a3, &v24, 0x100))
                {
                    v89 = *((unsigned int *)((void*)&v24 + 24));
                    v112 = 0;
                    if (((unsigned short)v89 & 448) != 448)
                    {
                        v11 = v113;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((unsigned int *)&(&v3)[284]) = v89;
                        v113 = v11;
                        v112 = 1;
                        if (!sub_40e520(a2, a3, (unsigned int)v89 | 448))
                            goto LABEL_40a1c6;
                        sub_414b40(4, a1);
                        v95 = "setting permissions for %s";
                        goto LABEL_40a538;
                    }
                    else
                    {
LABEL_40a1c6:
                        if (!*(a9))
                        {
                            v10 = v113;
                            *((char *)&v11) = v112;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            sub_40bf60(a3, *((unsigned long long *)(&v24 + 8)), *((unsigned long long *)&v24));
                            v113 = v10;
                            v112 = v11;
                            *(a9) = 1;
                        }
                        if (idx->field_3c)
                        {
                            v10 = v113;
                            *((char *)&v11) = v112;
                            if (idx->field_18)
                            {
                                v89 = sub_414b40(4, a1);
                                /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                __printf_chk(1, dcgettext(NULL, "created directory %s\n", 5));
                                v112 = v11;
                                goto LABEL_40a240;
                            }
                            else
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                sub_4073c0(a0, a1, 0);
                                v112 = v11;
                                goto LABEL_40a240;
                            }
                        }
                    }
                }
                else
                {
                    sub_414b40(4, a1);
                    v95 = "cannot stat %s";
                    goto LABEL_40a538;
                }
            }
            else
            {
                sub_414b40(4, a1);
                v95 = "cannot create directory %s";
                goto LABEL_40a538;
            }
        }
        if (!idx->field_28 && !idx->field_33 || (v11 = v129, /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (char)sub_408250(a1, 0, idx) || !idx->field_34))
        {
            v6 = 0;
            v112 = 0;
LABEL_40a240:
            v111 = _INSERT(v89, 0, a5 & idx->field_1c);
            if (!(a5 & idx->field_1c) || *(a5) == v16)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((char **)(/* unsupported instruction */ - 8)) = a10;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((char **)(/* unsupported instruction */ - 8)) = a9;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                *((struct_0 **)(/* unsupported instruction */ - 8)) = idx;
                *((void* *)(/* unsupported instruction */ - 8)) = index;
                *((char *)&v11) = v112;
                v112 = v11;
                /* unsupported instruction */ = /* unsupported instruction */ + 16;
                v111 = sub_40bbc0(a0, a1, a2, a3, v82);
            }
            if ((char)v12)
            {
                *((char *)&v9) = 0;
                v108 = 0;
                *((char *)&index) = v112;
LABEL_409c9d:
                if (idx->field_48)
                {
                    *((char *)&v12) = v108;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v108 = v12;
                    if (!fstatat(a2, a3, &v29, 0x100))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        sub_40ea00(idx->field_48, a3, &v29);
                        v108 = v12;
                    }
                }
LABEL_409cf8:
                if (idx->field_17 && v108)
                    return v111;
                if (v7)
                    return v111;
                goto LABEL_409d14;
            }
            *((char *)&v9) = 0;
            *((char *)&index) = v112;
LABEL_409d14:
            if (idx->field_1f)
            {
                v29 = sub_415960(&v16);
                v30 = v115;
                v32 = v115;
                v31 = sub_415980(&v16);
                /* unsupported instruction */ = /* unsupported instruction */ - 24;
                if (!utimensat(a2, a3, &v29, *((char *)&v9) * 0x100))
                    goto LABEL_409dd0;
                v12 = sub_414b40(4, a1);
                v116 = dcgettext(NULL, "preserving times for %s", 5);
                v117 = __errno_location();
                /* unsupported instruction */ = /* unsupported instruction */ - 32;
                error(0, *(v117), v116);
                if (!idx->field_32)
                    goto LABEL_409dd0;
            }
            else
            {
LABEL_409dd0:
                if (!*((char *)&v9))
                {
                    if (idx->field_1d && (v118 = (unsigned long)(unsigned long long)v20, v119 = (unsigned long)(unsigned long long)v21, v83 || v25 != (unsigned int)v118 || v26 != (unsigned int)v119))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((struct struct_0 **)(/* unsupported instruction */ - 8)) = &v24;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((unsigned long long *)(/* unsupported instruction */ - 8)) = v83;
                        *((unsigned long *)(/* unsupported instruction */ - 8)) = v119;
                        *((unsigned long *)(/* unsupported instruction */ - 16)) = v118;
                        v120 = sub_408430(idx, a1, a2, a3, 0xffffffff, v19, v33, a6, idx, a8);
                        if (v120 != 0xffffffff && !v120)
                        {
                            v13 &= 0xfffff1ff;
                            goto LABEL_409e58;
                        }
                    }
                    else
                    {
LABEL_409e58:
                        if (!idx->field_35 || sub_407040(a0, 0xffffffff, a1, 0xffffffff, idx) || !idx->field_36)
                        {
                            if (!(0xff0000000000ff & *((long long *)&idx->field_18)))
                            {
                                if (idx->field_39)
                                    goto LABEL_40b2bf;
                                if (!(v83 & idx->field_20))
                                {
                                    v6 = v6;
                                    if (!v6 || (v121 = ~((unsigned int)(unsigned long long)sub_408640()), v6 &= v121, !(v6 & v121)))
                                    {
                                        if (!(char)index)
                                            return v111;
                                    }
                                    else if ((char)index != 1)
                                    {
                                        if (v83 && fstatat(a2, a3, &v24, 0x100))
                                        {
LABEL_40a440:
                                            sub_414b40(4);
                                            v122 = "cannot stat %s";
                                            goto LABEL_40a459;
                                        }
                                        else
                                        {
                                            v4 = *((unsigned int *)(&v24 + 24));
                                            if (!(~(v4) & v6))
                                                return v111;
                                        }
                                    }
                                    if (!sub_40e520(a2, a3, *((unsigned int *)(&v3 + 284)) | v6))
                                        return v111;
                                    sub_414b40(4, a1);
                                    v123 = dcgettext(NULL, "preserving permissions for %s", 5);
                                    v124 = __errno_location();
                                    error(0, *(v124), v123);
                                    goto LABEL_409f0d;
                                }
                                else
                                {
                                    sub_408640();
LABEL_40b2bf:
                                    if (!sub_40cb50(a1, 0xffffffff))
                                        return v111;
                                }
                            }
                            else
                            {
                                if (!sub_40caa0(a0, 0xffffffff, a1, 0xffffffff, v13))
                                    return v111;
LABEL_409f0d:
                                if (!idx->field_32)
                                    return v111;
                            }
                        }
                    }
                }
                else
                {
                    if (!idx->field_35)
                    {
                        return v111;
                    }
                    else if (sub_407040(a0, 0xffffffff, a1, 0xffffffff, idx))
                    {
                        return v111;
                    }
                    else if (!idx->field_36)
                    {
                        return v111;
                    }
                }
            }
LABEL_409a38:
            return 0;
        }
    }
    else
    {
        /* unsupported instruction */ = /* unsupported instruction */ - 8;
        if (!(char)sub_407ff0(a0, a1, v13, v82, idx))
            goto LABEL_409a38;
        v89 = v13 & 0xf000;
        v6 = (unsigned int)v11 & 63;
        if ((unsigned int)v89 == 0x4000)
            goto LABEL_40a06a;
LABEL_409bba:
        *((char *)&v9) = idx->field_3a;
        if (idx->field_3a)
        {
            if (*(a0) != 47)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                ptr1 = sub_40e730(a3);
                if (a2 == 0xffffff9c && !(v11 = ".", /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (int)(long long)strcmp(".", ptr1)) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v91 = (unsigned int)(int)(long long)stat(".", &v28), (int)(long long)stat(".", &v28) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (unsigned int)(unsigned long long)fstatat((unsigned long long)a2, ptr1, &v29, 0) || v28.st_ino == *((unsigned long long *)((void*)&v29 + 8)) && *((unsigned long long *)(void*)&v28) == *((unsigned long long *)(void*)&v29))))
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    free(ptr1);
                }
                else
                {
                    free(ptr1);
                    sub_414c40(0, 3, a1);
                    v92 = "%s: can make relative symbolic links only in current directory";
                    goto LABEL_40ae6c;
                }
            }
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            v93 = sub_40c230(a0, a2, a3, idx->field_16, 0xffffffff);
            v7 = 0;
            if (v93 <= 0)
                goto LABEL_409c60;
            sub_414a70(1, 4, a0);
            sub_414a70(0, 4, a1);
            error(0, v93, dcgettext(NULL, "cannot create symbolic link %s to %s", 5));
        }
        else if (idx->field_17)
        {
            v106 = 1;
            if (!idx->field_16)
                v106 = idx->field_8 == 3;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v7;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v106;
            v7 = 0;
            /* unsupported instruction */ = /* unsupported instruction */ + 16;
            if ((char)sub_407460(a0, 0xffffff9c, a0, a1, a2, a3, v33, a6, *((char *)&idx)))
                goto LABEL_409c60;
        }
        else if ((unsigned int)v89 != 0x8000 && (v7 = (char)(char)((unsigned int)v89 != 0xa000) & idx->field_14, !((char)(char)((unsigned int)v89 != 0xa000) & idx->field_14)))
        {
            if ((unsigned int)v89 == 0x1000)
            {
                v104 = ~(v6) & v13;
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (mknodat(a2, a3, v104, 0) && (v105 = (long long)_INSERT((unsigned long long)v104, 1, (char)v104 & 239), /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (unsigned int)(unsigned long long)mkfifoat((unsigned long long)a2, a3, _INSERT((unsigned long long)v104, 1, (char)v104 & 239))))
                {
                    sub_414b40(4, a1);
                    v95 = "cannot create fifo %s";
                    goto LABEL_40a538;
                }
            }
            if (_INSERT(v89 & 0xffffffff, 1, *((char *)((void*)&v89 & 0xffffffff + 1)) & 191) == 0x2000 || (unsigned int)v89 == 0xc000)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (mknodat(a2, a3, ~(v6) & v13, v22))
                {
                    sub_414b40(4, a1);
                    v95 = "cannot create special file %s";
                    goto LABEL_40a538;
                }
            }
            if ((unsigned int)v89 == 0xa000)
            {
                ptr2 = sub_40cbf0(a0, v23);
                if (ptr2)
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 16;
                    v97 = sub_40c230(ptr2, a2, a3, idx->field_16, 0xffffffff);
                    *((unsigned int *)&v11) = v97;
                    if (v97 <= 0)
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        free(ptr2);
                        goto LABEL_40b8b9;
                    }
                    if (idx->field_3b != 1 || v83 || ((unsigned short)*((unsigned int *)((void*)&v24 + 24)) & 0xf000) != 0xa000 || (v9 = v27, v9 != strlen(ptr2) || (v98 = (void*)(unsigned long long)sub_40cd80((unsigned long long)a2, a3), !v98)))
                    {
LABEL_40b387:
                        free(ptr2);
                        sub_414b40(4, a1);
                        error(0, v11, dcgettext(NULL, "cannot create symbolic link %s", 5));
                    }
                    else
                    {
                        v9 = v98;
                        if (strcmp(v98, ptr2))
                        {
                            free(v9);
                            goto LABEL_40b387;
                        }
                        free(v9);
                        /* unsupported instruction */ = /* unsupported instruction */ - 40;
                        free(ptr2);
LABEL_40b8b9:
                        if (idx->field_33)
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            sub_407380();
                        }
                        *((char *)&v9) = 1;
                        v7 = idx->field_1d;
                        if (!idx->field_1d)
                        {
LABEL_409c60:
                            v107 = (unsigned int)v89 != 0x4000;
                            v108 = v107;
                            if (v83 || !(v109 = idx->field_14 ^ 1, v108 = v107, v109 & v108))
                            {
LABEL_409c83:
                                v110 = v12;
                                v111 = 1;
                                if (!(char)v110)
                                    goto LABEL_409cf8;
                                *((char *)&index) = 0;
                                v111 = v110;
                                goto LABEL_409c9d;
                            }
                            else if (idx->field_28 || idx->field_33)
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                v108 = sub_408250(a1, 0, idx);
                                if (v108)
                                    goto LABEL_409c83;
                                v83 = idx->field_34;
                                if (!v83)
                                    goto LABEL_40aa15;
                            }
                            else
                            {
LABEL_40aa15:
                                v108 = v109 & v107;
                                goto LABEL_409c83;
                            }
                        }
                        else
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            if (sub_40e540(a2, a3, v20, v21))
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                v100 = sub_408400(idx);
                                *((char *)&v9) = v100;
                                if (v100)
                                {
                                    v7 = 0;
                                    goto LABEL_409c60;
                                }
                                else
                                {
                                    v101 = dcgettext(NULL, "failed to preserve ownership for %s", 5);
                                    v102 = __errno_location();
                                    /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                    error(0, *(v102), v101);
                                    if (!idx->field_32)
                                        goto LABEL_40ba64;
                                }
                            }
                            else
                            {
LABEL_40ba64:
                                v103 = v7;
                                *((unsigned int *)v103) = *((int *)v103) + 1;
                                *((char *)&v9) = v103;
                                goto LABEL_409c60;
                            }
                        }
                    }
                }
                else
                {
                    sub_414b40(4, a0);
                    v95 = "cannot read symbolic link %s";
LABEL_40a538:
                    v125 = dcgettext(NULL, v95, 5);
                    v126 = __errno_location();
                    error(0, *(v126), v125);
                }
            }
            else
            {
                sub_414b40(4, a0);
                v92 = "%s has unknown file type";
LABEL_40ae6c:
                error(0, 0, dcgettext(NULL, v92, 5));
            }
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long long **)(/* unsupported instruction */ - 8)) = &v16;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((char **)(/* unsupported instruction */ - 8)) = &v14;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v6;
            /* unsupported instruction */ = /* unsupported instruction */ + 16;
            v7 = sub_408680(a0, a1, a2, a3, idx, (unsigned int)v11 & 0x1ff, v33, a6, idx);
            if (v7)
            {
                *((char *)&v9) = 0;
                v83 = v14;
                goto LABEL_409c60;
            }
        }
    }
    if (!idx->field_33)
        goto LABEL_40a562;
    goto LABEL_40a9d3;
}



// Function: copy @ 0xbae0
extern unsigned long long g_425568;
extern unsigned long long g_425570;

unsigned int copy(unsigned long a0, unsigned long a1, unsigned int a2, long long a3, int a4, void* a5, char *a6, unsigned long long a7)
{
    char v0;  // [bp-0x41]

    if (!(char)sub_406270(a5))
        __assert_fail(); /* do not return */
    v0 = 0;
    g_425570 = a0;
    g_425568 = a1;
    return sub_409470(a0, a1, a2, a3, a4, NULL, 0, a5, 1, &v0, a6, a7);
}



// Function: copy_dir @ 0xbbc0
typedef struct struct_0 {
    struct_0 field_0;
    unsigned int field_4;
    char padding_8[8];
    uint128_t field_10;
    uint128_t field_20;
    uint128_t field_30;
    uint128_t field_40;
    unsigned long long field_50;
} struct_0;

int copy_dir(long long a0, unsigned long a1, unsigned int a2, unsigned long a3, char a4, unsigned long long *a5, unsigned int a6, struct_0 *idx, char *a8, char *a9)
{
    char *v19;  // r14
    void* v20;  // r12
    void* v21;  // rbp
    unsigned int v22;  // eax
    char *v24;  // rax
    int v25;  // edx
    char *v26;  // rax
    int *err;  // rax
    char v0;  // [bp-0xfa]
    char v1;  // [bp-0xf9]
    unsigned int v2;  // [bp-0xe4]
    unsigned long long v3;  // [bp-0xe0]
    char *v4;  // [bp-0xd8]
    struct_0 *v5;  // [bp-0xd0]
    char *v6;  // [bp-0xc8]
    char *v7;  // [bp-0xc0]
    char *v8;  // [bp-0xb0]
    char v9;  // [bp-0x9b]
    char v10;  // [bp-0x9a]
    char v11;  // [bp-0x99]
    struct_0 v12;  // [bp-0x98]
    unsigned int v13;  // [bp-0x94]
    int v14;  // [bp-0x88]
    int v15;  // [bp-0x78]
    int v16;  // [bp-0x68]
    int v17;  // [bp-0x58]
    unsigned long v18;  // [bp-0x48]

    v12 = (struct_0)*((int128_t *)&idx->field_0.field_0);
    v14 = (int)*((int128_t *)&idx->field_0.field_10);
    v18 = idx->field_0.field_50;
    v15 = (int)*((int128_t *)&idx->field_0.field_20);
    v16 = (int)*((int128_t *)&idx->field_0.field_30);
    v17 = (int)*((int128_t *)&idx->field_0.field_40);
    v8 = sub_415610();
    if (v8)
    {
        if (idx->field_0.field_4 == 3)
            v13 = 2;
        if (*(v8))
        {
            v1 = 0;
            v2 = a4;
            v4 = &v11;
            v6 = &v9;
            v7 = &v10;
            v3 = a3 - a1;
            v19 = v8;
            v0 = 1;
            v5 = &v12;
        }
        else
        {
            v1 = 0;
            v0 = 1;
            goto LABEL_40bdb2;
        }
    }
    else
    {
        v24 = sub_414b40(4, a0);
        __unsupported_jumpkind_Ijk_NoDecode()
        *(v24) = *(v24) + *((char *)&v24);
        v26 = dcgettext(NULL, "cannot access %s", v25);
        err = __errno_location();
        error(0, *(err), v26);
        v0 = 0;
    }
    __unsupported_jumpkind_Ijk_NoDecode()
    v20 = sub_40ec90(a0, v19, 0);
    v21 = sub_40ec90((unsigned int)a1, v19, 0);
    v0 &= (char)sub_409470(v20, v21, a2, v21 + v3, v2, a5, a6, v5, 0, v7, v6, v4);
    *(a9) = *(a9) | v9;
    free(v21);
    v22 = (unsigned long long)free(v20);
    if (v9)
    {
LABEL_40bdb2:
        free(v8);
        *(a8) = v1;
    }
    v1 |= v10;
    __unsupported_jumpkind_Ijk_NoDecode()
    __unsupported_jumpkind_Ijk_NoDecode()
}



// Function: src_to_dest_hash @ 0xbe50
void src_to_dest_hash(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: src_to_dest_compare @ 0xbe60
unsigned long long src_to_dest_compare(unsigned long long *a0, unsigned long a1)
{
    if (*(a0) == *((long long *)a1))
        return _INSERT(*((long long *)(a1 + 8)), 0, a0[1] == *((long long *)(a1 + 8)));
    return 0;
}



// Function: src_to_dest_free @ 0xbe80
typedef struct struct_0 {
    char padding_0[16];
    void* field_10;
} struct_0;

void src_to_dest_free(struct_0 *a0)
{
    free(a0->field_10);
    free(a0);
    return;
}



// Function: forget_created @ 0xbea0
typedef struct struct_0 {
    char padding_0[16];
    void* field_10;
} struct_0;

extern long long g_425578;

void forget_created(unsigned long a0, unsigned long long a1)
{
    struct_0 *v4;  // rax
    unsigned long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]

    v0 = a0;
    v1 = a1;
    v2 = 0;
    v4 = sub_412350(g_425578, &v0);
    if (v4)
        sub_40be80(v4);
    return;
}



// Function: src_to_dest_lookup @ 0xbf00
typedef struct struct_0 {
    char padding_0[16];
    struct struct_0 *field_10;
} struct_0;

extern long long g_425578;

struct_0 * src_to_dest_lookup(unsigned long a0, unsigned long long a1)
{
    struct_0 *v3;  // rax
    unsigned long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]

    v0 = a0;
    v1 = a1;
    v3 = sub_411a70(g_425578, &v0);
    if (!v3)
        return v3;
    return v3->field_10;
}



// Function: remember_copied @ 0xbf60
extern long long g_425578;

unsigned long long remember_copied(long long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long *idx;  // rbp
    unsigned long long v2;  // rax
    unsigned long long *v3;  // rax

    idx = sub_416e90(24);
    v2 = sub_417230(a0);
    *(idx) = a1;
    idx[2] = v2;
    idx[1] = a2;
    v3 = sub_4122f0(g_425578, idx);
    if (!v3)
    {
        sub_417250(); /* do not return */
    }
    else if (idx != v3)
    {
        sub_40be80(idx);
        return v3[2];
    }
    else
    {
        return 0;
    }
}



// Function: hash_init @ 0xbfe0
extern unsigned long long g_425578;

void hash_init(void)
{
    g_425578 = sub_411cd0(103, 0, sub_40be50, sub_40be60, sub_40be80);
    if (!g_425578)
        sub_417250(); /* do not return */
    return;
}



// Function: try_link @ 0xc020
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



// Function: samedir_template @ 0xc040
typedef struct struct_0 {
    unsigned long long field_0;
    char field_8;
} struct_0;

void* samedir_template(void* a0, void* ptr)
{
    unsigned long v1;  // rax
    unsigned long v2;  // rdi
    struct_0 *v3;  // rax

    v1 = sub_40db60() - a0;
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



// Function: try_symlink @ 0xc0a0
typedef struct struct_0 {
    long long field_0;
    unsigned int field_8;
} struct_0;

long long try_symlink(long long a0, struct_0 *a1)
{
    return symlinkat(a1->field_0, a1->field_8, a0);
}



// Function: force_linkat @ 0xc0c0
int force_linkat(unsigned int a0, unsigned long long a1, unsigned int a2, unsigned long a3, unsigned int a4, char a5, int a6)
{
    int v7;  // eax
    int v8;  // eax
    char *v9;  // r15
    int v10;  // r12d
    unsigned int v11;  // r12d
    unsigned int v12;  // r12d
    unsigned int v0;  // [bp-0x174]
    char *v1;  // [bp-0x170]
    unsigned int v2;  // [bp-0x168]
    unsigned long long v3;  // [bp-0x160]
    unsigned int v4;  // [bp-0x158]
    unsigned int v5;  // [bp-0x154]
    char v6;  // [bp-0x148]

    v7 = a6;
    if (v7 < 0)
    {
        v8 = linkat();
        if (!v8)
            return v8;
        v7 = *(__errno_location());
    }
    v0 = a4;
    if (a5 != 1 || v7 != 0x11)
        return v7;
    v1 = &v6;
    v9 = sub_40c040(a3, &v6);
    if (!v9)
    {
        v11 = *(__errno_location());
        v12 = *(__errno_location());
        return *(__errno_location());
    }
    v5 = v0;
    v2 = a0;
    v3 = a1;
    v4 = a2;
    if (sub_415ba0(v9, 0, &v2, sub_40c020, 6))
    {
        v10 = *(__errno_location());
    }
    else
    {
        v10 = -0x1;
        if (renameat(a2, v9, a2, a3))
            v10 = *(__errno_location());
        unlinkat(a2, v9, 0);
    }
    if (v9 == v1)
        return v10;
    free(v9);
    return v10;
    return v7;
}



// Function: force_symlinkat @ 0xc230
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
    v5 = sub_40c040(a2, &v2);
    if (!v5)
    {
        v7 = *(__errno_location());
        v8 = *(__errno_location());
        return *(__errno_location());
    }
    v0 = a0;
    v1 = a1;
    if (sub_415ba0(v5, 0, &v0, sub_40c0a0, 6))
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



// Function: computecon @ 0xc370
int computecon(unsigned long a0, unsigned int a1, long long a2)
{
    unsigned long v3;  // rbp
    unsigned short v4;  // ax
    unsigned int v5;  // r12d
    unsigned int *err;  // rax
    unsigned long v0;  // [bp-0x40]
    unsigned long v1;  // [bp-0x38]

    v0 = 0;
    v1 = 0;
    v3 = sub_40e730();
    if (getcon(&v0) >= 0 && sub_4158c0(v3, &v1) >= 0 && !(v4 = (unsigned short)(unsigned long long)mode_to_security_class((unsigned long long)a1), !v4))
        v5 = security_compute_create(v0, v1, v4, a2);
    else
        v5 = 0xffffffff;
    err = __errno_location();
    free(v3);
    freecon(v0);
    freecon(v1);
    *(err) = *(err);
    return v5;
}



// Function: restorecon_private @ 0xc450
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
                goto LABEL_40c4cd;
            ptr = 0;
            ptr1 = 0;
            v9 = 0xffffffff;
            goto LABEL_40c6bb;
        }
        v10 = *(err);
        if (*(err) != 40)
        {
            ptr = 0;
            ptr1 = 0;
            goto LABEL_40c59b;
        }
        if (lstat(a1, &v2) < 0)
        {
            v10 = *(err);
            ptr = 0;
            ptr1 = 0;
            goto LABEL_40c59b;
        }
LABEL_40c4cd:
        if (selabel_lookup(a0, &v0, a1, v2.st_nlink) < 0)
        {
            v10 = *(err);
            ptr = 0;
            ptr1 = 0;
            if (v10 == 2)
            {
                *(err) = 61;
                v10 = 61;
                goto LABEL_40c554;
            }
        }
        ptr1 = context_new(v0);
        if (ptr1)
        {
            if (v4 != 0xffffffff)
            {
                if (sub_415900(v4) >= 0)
                {
                    ptr = context_new(v1);
                    if (ptr)
                        goto LABEL_40c531;
                    v10 = *(err);
                    v9 = 0xffffffff;
                    goto LABEL_40c55f;
                }
                else
                {
                    ptr = 0;
                    v9 = 0xffffffff;
                    goto LABEL_40c6bb;
                }
            }
            if (sub_4158e0(a1) >= 0)
            {
                ptr = context_new(v1);
                if (ptr)
                {
LABEL_40c531:
                    if (!context_type_get(ptr1) || context_type_set(ptr) || !context_str(ptr))
                    {
                        v10 = *(err);
                        goto LABEL_40c554;
                    }
                    if (v4 != 0xffffffff)
                    {
                        v9 = fsetfilecon(v4);
LABEL_40c6bb:
                        v10 = *(err);
LABEL_40c55f:
                        close(v4);
                    }
                    else
                    {
                        v4 = lsetfilecon(a1);
                        v10 = *(err);
LABEL_40c59b:
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
                goto LABEL_40c59b;
            }
        }
        else
        {
            v10 = *(err);
            ptr = 0;
LABEL_40c554:
            v9 = 0xffffffff;
            if (v4 != 0xffffffff)
                goto LABEL_40c55f;
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



// Function: defaultcon @ 0xc760
unsigned int defaultcon(long long a0, char *a1, unsigned int a2)
{
    char *ptr;  // rbp
    unsigned int *err;  // rbx
    char *v5;  // rax
    unsigned int v6;  // r15d
    long long ptr1;  // r12
    unsigned int v8;  // r14d
    long long v9;  // rax
    unsigned int v10;  // eax
    unsigned long v0;  // [bp-0x50]
    unsigned long v1;  // [bp-0x48]

    ptr = a1;
    v0 = 0;
    v1 = 0;
    err = __errno_location();
    if (*(ptr) != 47)
    {
        v5 = sub_40e4c0(ptr, 2);
        ptr = v5;
        if (v5)
        {
LABEL_40c7cb:
            if (selabel_lookup(a0, &v0, ptr, a2) < 0)
            {
                v6 = *(err);
                if (v6 != 2)
                {
                    ptr1 = 0;
                    ptr = NULL;
                    v8 = 0xffffffff;
                }
                else
                {
                    *(err) = 61;
                    v6 = 61;
                    ptr1 = 0;
                    ptr = NULL;
                    v8 = 0xffffffff;
                }
            }
            else if (sub_40c370(ptr, a2, &v1) >= 0)
            {
                ptr = context_new(v0);
                if (ptr)
                {
                    ptr1 = context_new(v1);
                    if (ptr1 && context_type_get(ptr) && !context_type_set(ptr1) && !(v9 = (long long)(unsigned long long)context_str(ptr1), !v9))
                    {
                        v10 = setfscreatecon(v9);
                        v6 = *(err);
                        v8 = v10;
                    }
                    else
                    {
                        v6 = *(err);
                        v8 = 0xffffffff;
                    }
                }
                else
                {
                    v6 = *(err);
                    ptr1 = 0;
                    v8 = 0xffffffff;
                }
            }
            else
            {
                v6 = *(err);
                ptr1 = 0;
                ptr = NULL;
                v8 = 0xffffffff;
            }
        }
        else
        {
            v6 = *(err);
            v5 = NULL;
            ptr1 = 0;
            v8 = 0xffffffff;
        }
    }
    else
    {
        v5 = NULL;
        goto LABEL_40c7cb;
    }
    context_free(ptr);
    context_free(ptr1);
    freecon(v0);
    freecon(v1);
    free(v5);
    *(err) = v6;
    return v8;
}



// Function: restorecon @ 0xc960
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

unsigned int restorecon(long long a0, char *a1, char a2)
{
    char *ptr;  // r13
    char *v4;  // rbp
    unsigned int *err;  // rbx
    unsigned int v6;  // r14d
    struct_0 *v7;  // rbp
    unsigned int v8;  // edx
    unsigned int v10;  // edx
    unsigned int v11;  // r12d
    char v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x40]

    ptr = NULL;
    v4 = a1;
    if (*(a1) == 47)
    {
        err = __errno_location();
        if (!a2)
        {
            v11 = _INSERT(a0, 0, sub_40c450(a0, v4, v10) != 0xffffffff);
            free(NULL);
            *(err) = *(err);
            return v11;
        }
    }
    else
    {
        v4 = sub_40e4c0(v4, 2);
        if (!v4)
            return 0;
        ptr = v4;
        err = __errno_location();
        if (!a2)
        {
            v11 = _INSERT(a0, 0, sub_40c450(a0, v4, v10) != 0xffffffff);
            free(ptr);
            *(err) = *(err);
            return v11;
        }
    }
    v1 = 0;
    v6 = 0;
    v7 = sub_417290(&v0, 16, 0);
LABEL_40c9d0:
    if (sub_410b30(v7))
    {
        do
        {
            if (sub_40c450(a0, *((long long *)&v7->field_20), v8) >= 0)
                goto LABEL_40c9d0;
        } while ((v6 = *(err), (unsigned long long)sub_410b30(v7)));
    }
    if (*(err))
        v6 = *(err);
    if (sub_410940())
        v6 = *(err);
    free(ptr);
    return _INSERT(a0, 0, !v6);
}



// Function: dir_name @ 0xe730
void dir_name(void)
{
    if (!sub_40e790())
        sub_417250(); /* do not return */
    return;
}



// Function: is_prime @ 0x113a0
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



// Function: next_prime @ 0x11410
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
    while (!sub_4113a0(v2, v3))
    {
        v2 = v4 + 2;
        if (v2 == 0xffffffffffffffff)
            return v4;
    }
    return v4;
}



// Function: raw_hasher @ 0x11450
long long raw_hasher(unsigned long a0, unsigned long long a1)
{
    return sub_417720() % a1;
}



// Function: raw_comparator @ 0x11470
void raw_comparator(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_tuning @ 0x11480
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

extern unsigned int g_41d660[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int *index;  // rax
    unsigned int v2;  // ymm0
    int v3;  // xmm1
    unsigned int v4;  // ymm1
    unsigned int v5;  // ymm2

    index = &idx->field_28->field_0;
    if (index == &g_41d660[0])
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
    idx->field_28 = &g_41d660[0];
    return 0;
}



// Function: free_entry @ 0x11510
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



// Function: safe_hasher @ 0x11530
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



// Function: hash_find_entry @ 0x11560
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

    v2 = sub_411530(a0, a1);
    *(a2) = v2;
    v3 = *((long long *)v2);
    if (!v3)
    {
LABEL_411601:
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
                    sub_411510(a0, v6);
                    return v0;
                }
                return *((long long *)v6);
            }
        }
        goto LABEL_411601;
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
    sub_411510(a0, v7);
    return v0;
}



// Function: allocate_entry @ 0x11660
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



// Function: transfer_entries @ 0x11690
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
LABEL_4116b8:
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
                    idx2 = sub_411530(idx, *((long long *)&v3->field_0));
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
                        sub_411510(idx, v5);
                        v3 = v6;
                        if (!v3)
                            break;
                    }
                }
                v2 = iter->field_0;
            }
            iter->field_8 = NULL;
            if (a2)
                goto LABEL_4116b8;
            index = sub_411530(idx, v2);
            if (index->field_0)
            {
                v8 = sub_411660(idx);
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



// Function: hash_get_n_buckets @ 0x11850
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0x11860
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0x11870
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0x11880
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



// Function: hash_print_statistics @ 0x11950
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
    sub_41195d();
    return;
}



// Function: hash_lookup @ 0x11a70
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

    iter = sub_411530(a0, a1);
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



// Function: hash_get_next @ 0x11b20
void hash_get_next(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_411b2c();
    return;
}



// Function: hash_do_for_each @ 0x11be0
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



// Function: hash_initialize @ 0x11cd0
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

extern struct_0 g_41d660;

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

    v1 = sub_411450;
    v2 = a1;
    if (!a2)
        a2 = v1;
    v3 = sub_411470;
    if (!a3)
        a3 = v3;
    ptr = malloc(80);
    if (!ptr)
        return ptr;
    v5 = &g_41d660.field_0;
    if (!v2)
        v2 = v5;
    ptr->field_28 = v2;
    if ((char)sub_411480(ptr))
    {
        v6 = sub_4117c0(a0, v2->field_10);
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



// Function: hash_clear @ 0x11de0
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



// Function: hash_free @ 0x11e90
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
            goto LABEL_411f3c;
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
LABEL_411f3c:
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



// Function: hash_rehash @ 0x11f80
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

    v11 = sub_4117c0(a1, idx->field_28->field_10);
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
        v13 = sub_411690(&ptr, idx, 0);
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
        if ((char)sub_411690(idx, &ptr, 1) && (char)sub_411690(idx, &ptr, 0))
        {
            free(ptr);
            return v13;
        }
        abort(); /* do not return */
    }
    return 0;
}



// Function: hash_insert_if_absent @ 0x12100
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
    v2 = sub_411560(idx, a1, &index, 0);
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
            sub_411480(idx);
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
                else if (!(char)sub_411f80(idx, (CmpF((unsigned int)v10, 0x5f000000) & 69 & 1 ? (unsigned int)v10 : 0x8000000000000000 ^ (unsigned int)(SubV(v10, 0x5f000000)))))
                {
                    return 0xffffffff;
                }
                else if (sub_411560(idx, a1, &index, 0))
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
        v11 = sub_411660(idx);
        if (!v11)
            return 0xffffffff;
        v11->field_0 = a1;
        v11->field_8 = index->field_8;
        index->field_8 = v11;
        *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
        return 1;
    }
}



// Function: hash_insert @ 0x122f0
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v2;  // eax
    unsigned long long v0;  // [bp-0x18]

    v2 = sub_412100(a0, a1, &v0);
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



// Function: hash_remove @ 0x12350
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

    v2 = sub_411560(idx, a1, &v0, 1);
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
            goto LABEL_412496;
LABEL_4123e0:
        v7 = (int)(unsigned int)v6;
    }
    else
    {
        v6 = idx[2].field_0;
        v5 = AddV((unsigned int)(v4 >> 1) | (unsigned int)v4 & 1, (unsigned int)(v4 >> 1) | (unsigned int)v4 & 1);
        if (v6 >= 0)
            goto LABEL_4123e0;
LABEL_412496:
        v7 = (int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1)));
    }
    if (((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(*((int *)idx[5].field_0), v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    sub_411480(idx);
    v9 = idx[5].field_0;
    if (((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69 | (char)((CmpF((unsigned int)(MulV(v9->field_0, v7)), v5) & 69) >> 6)) & 1) == 1)
        return v2;
    v11 = (uint128_t)(MulV(v7, v9->field_4));
    if (!v9->field_10)
        v11 = MulV(v11, v9->field_8);
    if ((char)sub_411f80(idx, (CmpF((unsigned int)v11, 0x5f000000) & 69 & 1 ? (unsigned int)v11 : 0x8000000000000000 ^ (unsigned int)(SubV(v11, 0x5f000000)))))
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



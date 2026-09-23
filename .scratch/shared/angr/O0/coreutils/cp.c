// Function: usage @ 0x44b3
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern FILE *stderr;

void usage(int a0)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    long long v14;  // r8
    long long v15;  // r9
    char *v16;  // rax
    char *v17;  // rax
    char *v18;  // rax
    char *v19;  // rax
    char *v20;  // rax
    char *v21;  // rax
    char *v22;  // rax
    char *v23;  // rax
    unsigned long long v6;  // rbx
    char *v24;  // rax
    char *v25;  // rax
    char *v26;  // rax
    char *v27;  // rax
    char *v28;  // rax
    char *v29;  // rax
    char *v30;  // rax
    char *v31;  // rax
    char *v7;  // rax
    char *v9;  // rax
    long long v10;  // rdi
    long long v11;  // rsi
    long long v12;  // rdx
    long long v13;  // rcx
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    if (a0)
    {
        v7 = gettext("Try '%s --help' for more information.\n");
        fprintf(stderr, v7);
    }
    else
    {
        printf(gettext("Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n"));
        v9 = gettext("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n");
        fputs_unlocked(v9, stdout);
        sub_404199(v10, v11, v12, v13, v14, v15);
        v16 = gettext("  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don't copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n");
        fputs_unlocked(v16, stdout);
        v17 = gettext("  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n");
        fputs_unlocked(v17, stdout);
        v18 = gettext("  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n");
        fputs_unlocked(v18, stdout);
        v19 = gettext("  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n");
        fputs_unlocked(v19, stdout);
        v20 = gettext("  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n");
        fputs_unlocked(v20, stdout);
        v21 = gettext("      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n");
        fputs_unlocked(v21, stdout);
        v22 = gettext("  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n");
        fputs_unlocked(v22, stdout);
        v23 = gettext("      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n");
        fputs_unlocked(v23, stdout);
        v24 = gettext("  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n");
        fputs_unlocked(v24, stdout);
        v25 = gettext("  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n");
        fputs_unlocked(v25, stdout);
        v26 = gettext("  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n");
        fputs_unlocked(v26, stdout);
        v27 = gettext("      --help        display this help and exit\n");
        fputs_unlocked(v27, stdout);
        v28 = gettext("      --version     output version information and exit\n");
        fputs_unlocked(v28, stdout);
        v29 = gettext("\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n");
        fputs_unlocked(v29, stdout);
        v30 = gettext("\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n");
        fputs_unlocked(v30, stdout);
        sub_4041ca(v10, v11, v12, v13, v14, v15);
        v31 = gettext("\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n");
        fputs_unlocked(v31, stdout);
        sub_40421c("cp");
    }
    exit(a0); /* do not return */
}



// Function: re_protect @ 0x4793
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    unsigned int field_1c;
    unsigned int field_20;
    char padding_24[116];
    unsigned long long field_98;
    struct struct_0 *field_a0;
} struct_0;

unsigned int re_protect(void* a0, unsigned int a1, unsigned long a2, struct_0 *a3, long long a4)
{
    unsigned long long v13;  // rax
    unsigned long long v14;  // rdx
    char *v15;  // rax
    int *err;  // rax
    char *v17;  // rax
    int *err1;  // rax
    char *v19;  // rax
    int *err2;  // rax
    char v0;  // [bp-0x10b8]
    char v1;  // [bp-0xb8]
    unsigned int v2;  // [bp-0x7c]
    struct_0 *iter;  // [bp-0x78]
    unsigned long v4;  // [bp-0x68]
    unsigned long long v5;  // [bp-0x60]
    unsigned long dst;  // [bp-0x58]
    long long v7;  // [bp-0x50]
    unsigned long v8;  // [bp-0x48]
    unsigned long long v9;  // [bp-0x40]
    unsigned long long v10;  // [bp-0x38]
    unsigned long long v11;  // [bp-0x30]

    v4 = strlen(a0) + 1;
    v13 = (unsigned long long)((15 + v4 + 8) / 16) * 16;
    while (&v0 != &(&v1)[-1 * (v13 & 0xfffffffffffff000)])
    ;
    /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v13 & 0xfff)];
    if ((unsigned short)v13 & 0xfff)
        *((long long *)(((unsigned int)v13 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v13 & 0xfff) - 8 + /* unsupported instruction */));
    v5 = (/* unsupported instruction */ + 15 >> 4) * 16;
    dst = memcpy(v5, a0, v4);
    v7 = dst + a2 - a0;
    for (iter = a3; iter; iter = iter->field_a0)
    {
        *((char *)(dst + iter->field_98)) = 0;
        if (*((char *)(a4 + 31)))
        {
            v8 = sub_419e57(iter);
            v9 = v14;
            v10 = sub_419e8b(iter);
            v11 = v14;
            if (utimensat(a1, v7, &v8, 0))
            {
                sub_418a0f(4, dst);
                v15 = gettext("failed to preserve times for %s");
                err = __errno_location();
                error(0, *(err), v15);
                return 0;
            }
        }
        if (*((char *)(a4 + 29)) && sub_4112c3(a1, v7, iter->field_1c, iter->field_20))
        {
            if ((char)sub_40dc1f(a4) != 1)
            {
                sub_418a0f(4, dst);
                v17 = gettext("failed to preserve ownership for %s");
                err1 = __errno_location();
                error(0, *(err1), v17);
                return 0;
            }
            v2 = sub_4112c3(a1, v7, 0xffffffff, iter->field_20);
        }
        if (*((char *)(a4 + 30)))
        {
            if (sub_40ec57(v7, 0xffffffff, dst, 0xffffffff, iter->field_18))
                return 0;
        }
        else
        {
            if (iter->padding_24[108] && sub_411260(a1, v7, iter->field_18, v7))
            {
                sub_418a0f(4, dst);
                v19 = gettext("failed to preserve permissions for %s");
                err2 = __errno_location();
                error(0, *(err2), v19);
                return 0;
            }
        }
        *((char *)(dst + iter->field_98)) = 47;
    }
    return 1;
}



// Function: make_dir_parents_private @ 0x4b50
typedef struct struct_0 {
    unsigned long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned int field_18[2];
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
    char field_90;
    char padding_91[7];
    unsigned long long field_98;
    struct struct_0 *field_a0;
} struct_0;

typedef struct struct_1 {
    char padding_0[29];
    char field_1d;
    char field_1e;
    char field_1f;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char padding_30[3];
    char field_33;
    char field_34;
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

unsigned int make_dir_parents_private(unsigned long a0, unsigned long a1, unsigned int a2, char *a3, void* *a4, char *a5, struct_1 *a6)
{
    unsigned long long v30;  // rax
    unsigned long long v31;  // rax
    int *err;  // rax
    unsigned int v32;  // eax
    int v33;  // eax
    char *v35;  // rax
    int *err1;  // rax
    char *v37;  // rax
    int *err2;  // rax
    char *v39;  // rax
    char v0;  // [bp-0x11f8]
    char v1;  // [bp-0x1f8]
    char v2;  // [bp-0x1ad]
    unsigned int v3;  // [bp-0x1ac]
    int v4;  // [bp-0x1a8]
    unsigned int v5;  // [bp-0x1a4]
    unsigned int v6;  // [bp-0x1a0]
    unsigned int v7;  // [bp-0x19c]
    char *node;  // [bp-0x198]
    char *iter;  // [bp-0x190]
    char *ptr;  // [bp-0x190]
    struct_0 *idx;  // [bp-0x188]
    unsigned long n;  // [bp-0x180]
    unsigned long v13;  // [bp-0x170]
    unsigned long long v14;  // [bp-0x168]
    unsigned long dst;  // [bp-0x160]
    char *v16;  // [bp-0x158]
    unsigned long long v17;  // [bp-0x150]
    char v18;  // [bp-0x148]
    unsigned int v19;  // [bp-0x130]
    stat v20;  // [bp-0xb8]
    unsigned long v21;  // [bp-0x70]
    unsigned long v22;  // [bp-0x68]
    unsigned long v23;  // [bp-0x60]
    unsigned long v24;  // [bp-0x58]
    unsigned long v25;  // [bp-0x50]
    unsigned long v26;  // [bp-0x48]
    unsigned long v27;  // [bp-0x40]
    unsigned long v28;  // [bp-0x38]
    unsigned long v29;  // [bp-0x30]

    n = sub_41158b(a0);
    *(a4) = NULL;
    if (a1 >= n)
        return 1;
    v13 = strlen(a0) + 1;
    v30 = (unsigned long long)((15 + v13 + 8) / 16) * 16;
    while (&v0 != &(&v1)[-1 * (v30 & 0xfffffffffffff000)])
    ;
    /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v30 & 0xfff)];
    if ((unsigned short)v30 & 0xfff)
        *((long long *)(((unsigned int)v30 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v30 & 0xfff) - 8 + /* unsupported instruction */));
    v14 = (/* unsupported instruction */ + 15 >> 4) * 16;
    /* unsupported instruction */ = /* unsupported instruction */ - 8;
    dst = memcpy(v14, a0, v13);
    v16 = a1 + dst;
    v31 = (unsigned long long)((15 + n + 9) / 16) * 16;
    for (/* unsupported instruction */ = /* unsupported instruction */; /* unsupported instruction */ != /* unsupported instruction */ - (v31 & 0xfffffffffffff000); *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8)))
    {
        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
    }
    /* unsupported instruction */ = /* unsupported instruction */ - ((unsigned int)v31 & 0xfff);
    if ((unsigned short)v31 & 0xfff)
        *((long long *)(((unsigned int)v31 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v31 & 0xfff) - 8 + /* unsupported instruction */));
    v17 = (/* unsupported instruction */ + 15 >> 4) * 16;
    memcpy(v17, dst, n);
    *((char *)(v17 + n)) = 0;
    for (node = a1 + v17; *(node) == 47; node += 1);
    if (fstatat(a2, node, &v18, 0))
    {
        for (iter = v16; *(iter) == 47; iter += 1);
        node = iter;
        while (1)
        {
            ptr = strchr(iter, 47);
            if (!ptr)
                break;
            *(ptr) = 0;
            v2 = fstatat(a2, node, &v18, 0);
            if (v2 || a6->field_1d || a6->field_1e || a6->field_1f)
            {
                v32 = stat(v16, &v20);
                v33 = (!v32 ? ((v20.st_nlink & 0xf000) == 0x4000 ? 0 : 20) : *(__errno_location()));
                v4 = v33;
                if (v4)
                {
                    sub_418a0f(4, v16);
                    error(0, v4, gettext("failed to get attributes of %s"));
                    return 0;
                }
                idx = sub_41b941(168);
                idx->field_0 = *((unsigned long long *)&v20);
                idx->field_8 = v20.st_ino;
                idx->field_10 = v20.st_dev;
                *((unsigned long long *)&idx->field_18[0]) = *((unsigned long long *)((void*)&v20 + 24));
                idx->field_20 = *((unsigned long long *)((void*)&v20 + 32));
                idx->field_28 = v20.st_size;
                idx->field_30 = v20.st_atime;
                idx->field_38 = v20.st_mtime;
                idx->field_40 = v20.st_ctime;
                idx->field_48 = v21;
                idx->field_50 = v22;
                idx->field_58 = v23;
                idx->field_60 = v24;
                idx->field_68 = v25;
                idx->field_70 = v26;
                idx->field_78 = v27;
                idx->field_80 = v28;
                idx->field_88 = v29;
                idx->field_98 = &ptr[-1 * dst];
                idx->field_90 = 0;
                idx->field_a0 = *(a4);
                *(a4) = idx;
            }
            if ((char)sub_4084c6(v16, dst, (!v2 ? 0 : idx->field_18[0]), v2, a6) != 1)
                return 0;
            if (v2)
            {
                *(a5) = 1;
                v5 = idx->field_18[0];
                v3 = (!(a6->field_1d ^ 1) ? 63 : (!a6->field_1e ? 0 : 18)) & v5;
                v6 = (!a6->field_20 ? v5 : 0x1ff);
                v6 = ~(v3) & v6 & 0xfff;
                if (mkdirat(a2, node, v6, node))
                {
                    sub_418a0f(4, dst);
                    v35 = gettext("cannot make directory %s");
                    err1 = __errno_location();
                    error(0, *(err1), v35);
                    return 0;
                }
                if (a3)
                    printf(a3);
                if (fstatat(a2, node, &v18, 0x100))
                {
                    sub_418a0f(4, dst);
                    v37 = gettext("failed to get attributes of %s");
                    err2 = __errno_location();
                    error(0, *(err2), v37);
                    return 0;
                }
                if (a6->field_1e != 1)
                {
                    if (~(v19) & v3)
                        v3 &= ~((unsigned int)sub_40dcaf());
                    if (~(v19) & v3 || ((unsigned short)v19 & 448) != 448)
                    {
                        idx->field_18[0] = v19 | v3;
                        idx->field_90 = 1;
                    }
                }
                v7 = v19 | 448;
                if (v7 != v19 && sub_411260(a2, node, v7, node))
                {
                    sub_418a0f(4, dst);
                    v39 = gettext("setting permissions for %s");
                    err = __errno_location();
                    error(0, *(err), v39);
                    return 0;
                }
            }
            else
            {
                if (((unsigned short)v19 & 0xf000) != 0x4000)
                {
                    sub_418a0f(4, dst);
                    error(0, 0, gettext("%s exists but is not a directory"));
                    return 0;
                }
                *(a5) = 0;
            }
            if (*(a5) != 1 && (a6->field_28 || a6->field_33) && (char)sub_408750(dst, 0, a6) != 1 && a6->field_34)
                return 0;
            iter = ptr + 1;
            for (*(ptr) = 47; *(iter) == 47; iter += 1);
        }
    }
    else
    {
        if (((unsigned short)v19 & 0xf000) != 0x4000)
        {
            sub_418a0f(4, v17);
            error(0, 0, gettext("%s exists but is not a directory"));
            return 0;
        }
        *(a5) = 0;
    }
    return 1;
}



// Function: do_copy @ 0x5522
typedef struct struct_1 {
    char padding_0[160];
    struct struct_1 *field_a0;
} struct_1;

typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    long long field_10;
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

extern char g_427121;
extern char g_427122;
extern void g_427140;
extern unsigned long long g_427148;
extern unsigned long long g_427150;
extern unsigned long long g_427158;
extern unsigned long long g_427160;
extern unsigned long long g_427168;
extern unsigned long long g_427170;
extern unsigned long long g_427178;
extern unsigned long long g_427180;
extern unsigned long long g_427188;
extern unsigned long long g_427190;

long long do_copy(int a0, struct_0 *a1, long long a2, char a3, unsigned long a4)
{
    unsigned int v43;  // eax
    void* idx;  // rax
    unsigned long v45;  // rbx
    unsigned long v46;  // rbx
    unsigned long v47;  // rbx
    unsigned long v48;  // rbx
    unsigned long v49;  // rbx
    char *v36;  // rax
    int *err;  // rax
    unsigned long long v39;  // rax
    char *v40;  // rcx
    unsigned long long v41;  // rax
    int v0;  // [bp-0x11a0]
    int v1;  // [bp-0x1a0]
    long long v2;  // [bp-0x170]
    int v3;  // [bp-0x15c]
    char v4;  // [bp-0x14c]
    char v5;  // [bp-0x14b]
    char v6;  // [bp-0x14a]
    char v7;  // [bp-0x149]
    unsigned int v8;  // [bp-0x148]
    int i;  // [bp-0x144]
    unsigned int v10;  // [bp-0x140]
    unsigned int v11;  // [bp-0x13c]
    struct_1 *iter;  // [bp-0x138]
    char *j;  // [bp-0x130]
    unsigned long v14;  // [bp-0x128]
    unsigned long v15;  // [bp-0x120]
    unsigned long v16;  // [bp-0x118]
    unsigned long v17;  // [bp-0x110]
    long long v18;  // [bp-0x108]
    void* v19;  // [bp-0x100]
    unsigned long v20;  // [bp-0xf8]
    unsigned long long v21;  // [bp-0xf0]
    long long dst1;  // [bp-0xe8], Other Possible Types: unsigned long
    long long v23;  // [bp-0xe0]
    unsigned long v24;  // [bp-0xd8]
    unsigned long long v25;  // [bp-0xd0]
    unsigned long dst;  // [bp-0xc8]
    struct_1 *ptr;  // [bp-0xc0]
    stat v28;  // [bp-0xb8]
    unsigned int v29;  // [bp-0xa0]
    unsigned long v30;  // [bp+0x0]
    long long v31;  // [bp+0x8]

    /* unsupported instruction */ = (int)(&v0 - 4136);
    v3 = a0;
    v2 = a2;
    *((unsigned long *)&(&v0)[4136]) = a4;
    v4 = 0;
    v6 = 1;
    if (v3 <= (!v2))
    {
        if (v3 <= 0)
        {
            error(0, 0, gettext("missing file operand"));
        }
        else
        {
            sub_418a0f(4, a1->field_0);
            error(0, 0, gettext("missing destination file operand after %s"));
        }
        sub_4044b3(1); /* do not return */
    }
    else
    {
        v29 = 0;
        v8 = 0xffffff9c;
        if (a3)
        {
            if (v2)
            {
                error(1, 0, gettext("cannot combine --target-directory (-t) and --no-target-directory (-T)"));
            }
            else if (v3 > 2)
            {
                sub_418a0f(4, a1->field_10);
                error(0, 0, gettext("extra operand %s"));
                sub_4044b3(1); /* do not return */
            }
        }
        else
        {
            if (v2)
            {
                v8 = sub_419f57(v2, &v28, &v28);
                /* unsupported instruction */ = (int)(&v0 - 4128);
                if ((char)sub_419edf(v8) != 1)
                {
                    sub_418a0f(4, v2);
                    v36 = gettext("target directory %s");
                    err = __errno_location();
                    error(1, *(err), v36);
                }
            }
            else
            {
                v16 = *((long long *)((char *)&(&a1->field_0)[v3] - 8));
                v10 = sub_419f57(v16, &v28, &v28);
                /* unsupported instruction */ = (int)(&v0 - 4128);
                if ((char)sub_419edf(v10))
                {
                    v8 = v10;
                    v2 = v16;
                    v3 -= 1;
                }
                else
                {
                    /* unsupported instruction */ = (int)(&v0 - 4128);
                    v11 = *(__errno_location());
                    if (v11 == 2)
                        v4 = 1;
                    if (v3 > 2)
                    {
                        sub_418a0f(4, v16);
                        error(1, v11, gettext("target %s"));
                    }
                }
            }
        }
        if (v2)
        {
            if (v3 > 1)
            {
                sub_40a549(*((unsigned long long *)(&v0 + 4136)));
                /* unsupported instruction */ = (int)(&v0 - 4128);
                sub_40a5a6(*((unsigned long long *)(&v0 + 4136)));
            }
            for (i = 0; i < v3; i += 1)
            {
                v7 = 1;
                j = NULL;
                v18 = (&a1->field_0)[i];
                if (g_427122)
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    sub_4116ab(v18);
                }
                if (g_427121)
                {
                    v23 = v18;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    v24 = strlen(v23) + 1;
                    v39 = (unsigned long long)((15 + v24 + 8) / 16) * 16;
                    for (/* unsupported instruction */ = /* unsupported instruction */; /* unsupported instruction */ != /* unsupported instruction */ - (v39 & 0xfffffffffffff000); *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8)))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                    }
                    /* unsupported instruction */ = /* unsupported instruction */ - ((unsigned int)v39 & 0xfff);
                    if ((unsigned short)v39 & 0xfff)
                        *((long long *)(((unsigned int)v39 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v39 & 0xfff) - 8 + /* unsupported instruction */));
                    v25 = (/* unsupported instruction */ + 15 >> 4) * 16;
                    dst = memcpy(v25, v23, v24);
                    sub_4116ab(dst);
                    v14 = sub_411da5(v2, dst, &j, dst);
                    v40 = (!*((char *)(*((unsigned long long *)(&v0 + 4136)) + 60)) ? NULL : "%s -> %s\n");
                    /* unsupported instruction */ = /* unsupported instruction */ - 32;
                    *((unsigned long long *)(/* unsupported instruction */ - 8)) = *((unsigned long long *)(&v0 + 4136));
                    for (v7 = sub_404b50(v14, &j[-1 * v14], v8, v40, &iter, &v4, v30); *(j) == 47; j += 1);
                }
                else
                {
                    v19 = sub_410369(v18);
                    /* unsupported instruction */ = /* unsupported instruction */ - 16;
                    v20 = strlen(v19) + 1;
                    v41 = (unsigned long long)((15 + v20 + 8) / 16) * 16;
                    for (/* unsupported instruction */ = /* unsupported instruction */; /* unsupported instruction */ != /* unsupported instruction */ - (v41 & 0xfffffffffffff000); *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8)))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                    }
                    /* unsupported instruction */ = /* unsupported instruction */ - ((unsigned int)v41 & 0xfff);
                    if ((unsigned short)v41 & 0xfff)
                        *((long long *)(((unsigned int)v41 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v41 & 0xfff) - 8 + /* unsupported instruction */));
                    v21 = (/* unsupported instruction */ + 15 >> 4) * 16;
                    dst1 = memcpy(v21, v19, v20);
                    sub_4116ab(dst1);
                    dst1 += !strcmp(dst1, "..");
                    /* unsupported instruction */ = /* unsupported instruction */ - 32;
                    v14 = sub_411da5(v2, dst1, &j, dst1);
                }
                if (v7 != 1)
                {
                    v6 = 0;
                }
                else
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
                    *((char **)(/* unsupported instruction */ - 8)) = &v5;
                    /* unsupported instruction */ = /* unsupported instruction */;
                    v6 = (char)sub_40daea(v18, v14, v8, j, v4, *((unsigned long long *)(&v0 + 4136)), v30, v31) & v6;
                    if (g_427121)
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v6 = sub_404793(v14, v8, j, iter, *((unsigned long long *)(&v0 + 4136))) & 0xff & v6;
                    }
                }
                if (g_427121)
                {
                    while (iter)
                    {
                        ptr = iter;
                        iter = iter->field_a0;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        free(ptr);
                    }
                }
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                free(v14);
            }
            return v6;
        }
        else
        {
            v17 = a1->field_0;
            v15 = a1->field_8;
            if (g_427121)
            {
                error(0, 0, gettext("with --parents, the destination must be a directory"));
                sub_4044b3(1); /* do not return */
            }
            if (*((char *)(*((unsigned long long *)(&v0 + 4136)) + 22)) && *((int *)*((unsigned long long *)(&v0 + 4136))) && !strcmp(v17, v15) && v4 != 1 && (v29 || (v43 = (unsigned int)(int)(long long)stat(v15, &v28), !(int)(long long)stat(v15, &v28))) && ((unsigned short)v28.st_nlink & 0xf000) == 0x8000)
            {
                v15 = sub_41024d(0xffffff9c, v15, *((int *)*((unsigned long long *)(&v0 + 4136))));
                idx = *((unsigned long long *)(&v0 + 4136));
                v45 = (long long)idx[8];
                *((long long *)&g_427140) = *((long long *)idx);
                g_427148 = v45;
                v46 = (long long)idx[24];
                g_427150 = (long long)idx[16];
                g_427158 = v46;
                v47 = (long long)idx[40];
                g_427160 = (long long)idx[32];
                g_427168 = v47;
                v48 = (long long)idx[56];
                g_427170 = (long long)idx[48];
                g_427178 = v48;
                v49 = (long long)idx[72];
                g_427180 = (long long)idx[64];
                g_427188 = v49;
                g_427190 = (long long)idx[80];
                *((unsigned int *)&g_427140) = 0;
                v1 = (int)_INSERT(*((uint384_t *)(&v0 + 40)), 40, &g_427140);
            }
            v6 = sub_40daea(v17, v15, 0xffffff9c, v15, -(v4), *((unsigned long long *)(&v1 + 40)), &j, 0);
            return v6;
        }
    }
}



// Function: cp_option_init @ 0x5f11
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
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
    char field_1e;
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
    char field_37;
    char field_38;
    char field_39;
    char field_3a;
    char padding_3b[3];
    char field_3e;
    char padding_3f[5];
    unsigned int field_44;
    unsigned long long field_48;
    unsigned long long field_50;
} struct_0;

struct_0 * cp_option_init(struct_0 *idx)
{
    sub_40dbcb(idx);
    idx->field_14 = 1;
    idx->field_4 = 1;
    idx->field_15 = 0;
    idx->field_16 = 0;
    idx->field_17 = 0;
    idx->field_8 = 4;
    idx->field_18 = 0;
    idx->field_19 = 0;
    idx->field_1c = 0;
    idx->field_44 = 1;
    idx->field_1d = 0;
    idx->field_30 = 0;
    idx->field_1e = 0;
    idx->field_1f = 0;
    idx->field_20 = 0;
    idx->field_33 = 0;
    idx->field_34 = 0;
    idx->field_28 = 0;
    idx->field_35 = 0;
    idx->field_37 = 0;
    idx->field_36 = 0;
    idx->field_31 = 1;
    idx->field_32 = 0;
    idx->field_38 = 0;
    idx->field_c = 2;
    idx->field_3a = 0;
    idx->field_39 = 0;
    idx->field_10 = 0;
    memset(idx->padding_3b, 0, 3);
    idx->field_3e = getenv("POSIXLY_CORRECT");
    idx->field_48 = 0;
    idx->field_50 = 0;
    return idx;
}



// Function: decode_preserve_arg @ 0x606f
extern unsigned int g_41fb60[4];
extern long long g_426840;
extern long long g_427028;
extern char g_427120;

void decode_preserve_arg(long long a0, char *idx, char a2)
{
    long long v5;  // rax
    char *i;  // [bp-0x20]
    char *ptr;  // [bp-0x18]
    char *v2;  // [bp-0x18]
    char *v3;  // [bp-0x10]

    v3 = sub_41bf48(a0);
    i = v3;
    do
    {
        ptr = strchr(i, 44);
        if (ptr)
        {
            v2 = ptr + 1;
            *(ptr) = 0;
            ptr = v2;
        }
        v5 = (!a2 ? "--no-preserve" : "--preserve");
        sub_40f72a(v5, i, &g_426840, &g_41fb60[0], 4, g_427028, 1);
        switch (g_41fb60[sub_40f72a(v5, i, &g_426840, &g_41fb60[0], 4, g_427028, 1)])
        {
        case 0:
            idx[30] = a2;
            idx[32] = !a2;
            break;
        case 1:
            idx[31] = a2;
            break;
        case 2:
            idx[29] = a2;
            break;
        case 3:
            idx[48] = a2;
            break;
        case 4:
            idx[52] = a2;
            idx[0x33] = a2;
            break;
        case 5:
            idx[53] = a2;
            idx[54] = a2;
            break;
        case 6:
            idx[30] = a2;
            idx[31] = a2;
            idx[29] = a2;
            idx[48] = a2;
            idx[32] = !a2;
            if (g_427120)
                idx[0x33] = a2;
            idx[53] = a2;
            break;
        default:
            abort(); /* do not return */
        }
        i = ptr;
    } while (i);
    free(v3);
    return;
}



// Function: main @ 0x6276
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
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
    char field_1e;
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
    char field_37;
    char field_38;
    char field_39;
    char field_3a;
    char padding_3b[3];
    char field_3e;
    char padding_3f[5];
    unsigned int field_44;
    unsigned long long field_48;
    unsigned long long field_50;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_41e353;
extern unsigned int g_41e3a0[4];
extern unsigned int g_41e3b0[4];
extern long long g_426440;
extern long long g_426460;
extern option g_426480;
extern long long g_427020;
extern long long g_427028;
extern long long stdout;
extern int optind;
extern char optarg;
extern char g_427120;
extern char g_427121;
extern char g_427122;

int main(int a0, long long *a1)
{
    long long v35;  // rdi
    long long v36;  // rsi
    unsigned long v48;  // rax
    unsigned int v49;  // eax
    char *v52;  // rax
    int *err;  // rax
    long long v37;  // rdx
    long long v38;  // rcx
    long long v39;  // r8
    long long v40;  // r9
    char *v41;  // rax
    int *err1;  // rax
    char v0;  // [bp-0xa0]
    char v1;  // [bp-0x9f]
    char v2;  // [bp-0x9e]
    char v3;  // [bp-0x9d]
    int v4;  // [bp-0x9c]
    long long v5;  // [bp-0x98]
    long long v6;  // [bp-0x90]
    long long v7;  // [bp-0x88]
    long long v8;  // [bp-0x80]
    struct_0 v9;  // [bp-0x78]
    unsigned int v10;  // [bp-0x74]
    unsigned int v11;  // [bp-0x70]
    unsigned int *v12;  // [bp-0x6c]
    char v13;  // [bp-0x64]
    char v14;  // [bp-0x63]
    char v15;  // [bp-0x62]
    char v16;  // [bp-0x61]
    char v17;  // [bp-0x5c]
    char v18;  // [bp-0x5b]
    char v19;  // [bp-0x5a]
    char v20;  // [bp-0x59]
    unsigned long long v21;  // [bp-0x50]
    char v22;  // [bp-0x48]
    char v23;  // [bp-0x47]
    char v24;  // [bp-0x46]
    char v25;  // [bp-0x45]
    char v26;  // [bp-0x44]
    char v27;  // [bp-0x43]
    char v28;  // [bp-0x41]
    char v29;  // [bp-0x40]
    char v30;  // [bp-0x3e]
    char v31;  // [bp-0x3d]
    char v32;  // [bp-0x3c]
    unsigned int *v33;  // [bp-0x34]

    v0 = 0;
    v5 = 0;
    v6 = 0;
    v1 = 0;
    v7 = 0;
    v2 = 0;
    v8 = 0;
    sub_416d05(*(a1));
    setlocale(6, &g_41e353);
    bindtextdomain("coreutils", "/usr/local/share/locale");
    textdomain("coreutils");
    sub_41d610(sub_411312);
    g_427120 = is_selinux_enabled(v35, v36, v37, v38, v39, v40) > 0;
    sub_405f11(&v9);
    while (1)
    {
        v4 = getopt_long(a0, a1, "abdfHilLnprst:uvxPRS:TZ", &g_426480.name, NULL);
        if (v4 != -0x1)
        {
            if (v4 > 0x88)
                goto LABEL_4067a1;
            if (v4 >= 72)
            {
                switch (v4)
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
                    v29 = 1;
                    continue;
                case 83:
                    v0 = 1;
                    v5 = *((long long *)&optarg);
                    break;
                case 84:
                    v2 = 1;
                    continue;
                case 90:
                    if (g_427120)
                    {
                        if (*((long long *)&optarg))
                        {
                            v8 = *((long long *)&optarg);
                            break;
                        }
                        else
                        {
                            v21 = selabel_open(0, 0, 0);
                            if (!v21)
                            {
                                v41 = gettext("warning: ignoring --context");
                                err1 = __errno_location();
                                error(0, *(err1), v41);
                                break;
                            }
                        }
                    }
                    else
                    {
                        if (*((long long *)&optarg))
                        {
                            error(0, 0, gettext("warning: ignoring --context; it requires an SELinux-enabled kernel"));
                            break;
                        }
                    }
                case 97:
                    v10 = 2;
                    v22 = 1;
                    v18 = 1;
                    v19 = 1;
                    v20 = 1;
                    v24 = 1;
                    if (g_427120)
                        v25 = 1;
                    v27 = 1;
                    v28 = 1;
                    v29 = 1;
                    continue;
                case 98:
                    v0 = 1;
                    if (*((long long *)&optarg))
                    {
                        v6 = *((long long *)&optarg);
                        break;
                    }
                    break;
                case 100:
                    v22 = 1;
                    v10 = 2;
                    continue;
                case 102:
                    v15 = 1;
                    continue;
                case 105:
                    v11 = 3;
                    continue;
                case 108:
                    v16 = 1;
                    continue;
                case 110:
                    v11 = 2;
                    continue;
                case 112:
                    v18 = 1;
                    v19 = 1;
                    v20 = 1;
                    v24 = 1;
                    continue;
                case 115:
                    v30 = 1;
                    continue;
                case 116:
                    if (v7)
                        error(1, 0, gettext("multiple target directories specified"));
                    v7 = *((long long *)&optarg);
                    break;
                case 117:
                    v31 = 1;
                    continue;
                case 118:
                    v32 = 1;
                    continue;
                case 120:
                    v17 = 1;
                    continue;
                case 128:
                    v23 = 0;
                    continue;
                case 129:
                    v1 = 1;
                    continue;
                case 130:
                    sub_40606f(*((long long *)&optarg), &v9, 0);
                    continue;
                case 131:
                    g_427121 = 1;
                    continue;
                case 132:
                    if (*((long long *)&optarg))
                    {
                        sub_40606f(*((long long *)&optarg), &v9, 1);
                        v24 = 1;
                        continue;
                    }
                    break;
                case 133:
                    if (!*((long long *)&optarg))
                    {
                        v33 = 0x2;
                        break;
                    }
                    else
                    {
                        v33 = (unsigned int *)g_41e3b0[sub_40f72a("--reflink", *((long long *)&optarg), &g_426460, &g_41e3b0[0], 4, g_427028, 1)];
                        break;
                    }
                case 134:
                    v12 = (unsigned int *)g_41e3a0[sub_40f72a("--sparse", *((long long *)&optarg), &g_426440, &g_41e3a0[0], 4, g_427028, 1)];
                    continue;
                case 135:
                    g_427122 = 1;
                    continue;
                case 136:
                    v14 = 1;
                    continue;
                default:
LABEL_4067a1:
                    sub_4044b3(1); /* do not return */
                }
            }
            else if (v4 == -0x83)
            {
                sub_41b6df(stdout, "cp", "GNU coreutils", g_427020, "Torbjorn Granlund", "David MacKenzie", "Jim Meyering", 0);
                exit(0); /* do not return */
            }
            else if (v4 == -0x82)
            {
                sub_4044b3(0); /* do not return */
            }
        }
        else
        {
            if (v16 && v30)
            {
                error(0, 0, gettext("cannot make both hard and symbolic links"));
                sub_4044b3(1); /* do not return */
            }
            if (v11 == 2)
            {
                v31 = 0;
                break;
            }
        }
    }
    if (v0 && v11 == 2)
    {
        error(0, 0, gettext("options --backup and --no-clobber are mutually exclusive"));
        sub_4044b3(1); /* do not return */
    }
    if (v33 == 2 && v12 != 2)
    {
        error(0, 0, gettext("--reflink can be used only with --sparse=auto"));
        sub_4044b3(1); /* do not return */
    }
    if (v0)
    {
        v48 = gettext("backup type");
        v49 = sub_41030b(v48, v6, v48);
    }
    else
    {
        v49 = 0;
    }
    v9 = (struct_0)v49;
    sub_40f869(v5);
    if (v10 == 1)
    {
        if (v29 && v16 != 1)
            v10 = 2;
        else
            v10 = 4;
    }
    if (v29)
        v13 = v1;
    if ((v21 || v8) && v26 != 1)
        v25 = 0;
    if (v25 && (v21 || v8))
        error(1, 0, gettext("cannot set target context and preserve it"));
    if (v26 && g_427120 != 1)
        error(1, 0, gettext("cannot preserve security context without an SELinux-enabled kernel"));
    if (v8 && setfscreatecon(v8) < 0)
    {
        sub_418e56(v8);
        v52 = gettext("failed to set default file creation context to %s");
        err = __errno_location();
        error(1, *(err), v52);
    }
    sub_40df3d();
    v3 = sub_405522(a0 - optind, &a1[optind], v7, v2, &v9);
    return v3 ^ 1;
}



// Function: follow_fstatat @ 0x6cbb
long long follow_fstatat(unsigned int a0, long long a1, long long a2, unsigned int a3)
{
    unsigned int v0;  // [bp-0xac]

    v0 = fstatat(a0, a1, a2, a3);
    return v0;
}



// Function: punch_hole @ 0x6d36
unsigned int punch_hole(unsigned int a0, long long a1, long long a2)
{
    unsigned int v0;  // [bp-0xc]

    v0 = 0;
    v0 = fallocate(a0, 3, a1, a2);
    if (v0 >= 0)
        return v0;
    if (!(char)sub_406bb9(*(__errno_location())) && *(__errno_location()) != 38)
        return v0;
    v0 = 0;
    return v0;
}



// Function: create_hole @ 0x6d9e
unsigned int create_hole(unsigned int a0, long long a1, char a2, long long a3)
{
    char *v2;  // rax
    int *err;  // rax
    char *v4;  // rax
    int *err1;  // rax
    unsigned long long v0;  // [bp-0x20]

    v0 = lseek(a0, a3, 1);
    if (v0 - 0 >> 63 & 1)
    {
        sub_418a0f(4, a1);
        v2 = gettext("cannot lseek %s");
        err = __errno_location();
        error(0, *(err), v2);
        return 0;
    }
    if (!a2 || sub_406d36(a0, v0 - a3, a3) >= 0)
        return 1;
    sub_418a0f(4, a1);
    v4 = gettext("error deallocating %s");
    err1 = __errno_location();
    error(0, *(err1), v4);
    return 0;
    return 1;
}



// Function: sparse_copy @ 0x6ea8
unsigned int sparse_copy(int a0, unsigned int a1, unsigned long long *a2, unsigned long a3, unsigned long a4, char a5, char a6, long long a7, long long a8, unsigned long iter, unsigned long long *a10, char *a11)
{
    char *v13;  // rax
    int *err;  // rax
    char *v23;  // rax
    int *err1;  // rax
    unsigned long v15;  // rdx
    unsigned long v16;  // rax
    char v17;  // al
    char v18;  // al
    char v19;  // cl
    unsigned long v20;  // rax
    char *v21;  // rax
    int *err2;  // rax
    unsigned long v0;  // [bp-0x90]
    char v1;  // [bp-0x74]
    char v2;  // [bp-0x73]
    char v3;  // [bp-0x72]
    char v4;  // [bp-0x71]
    unsigned long j;  // [bp-0x70]
    unsigned long long v6;  // [bp-0x68]
    unsigned long count;  // [bp-0x60]
    unsigned long v8;  // [bp-0x58]
    unsigned long long k;  // [bp-0x50]
    unsigned long long v10;  // [bp-0x48]
    unsigned long v11;  // [bp-0x40]
    unsigned long long v12;  // [bp-0x38]

    v0 = a3;
    *(a11) = 0;
    if (!a4 && a6)
    {
        for (*(a10) = 0; iter; *(a10) = *(a10) + v6)
        {
            v11 = 0x7fffffffc0000000;
            v6 = copy_file_range();
            if (!v6)
            {
                if (*(a10))
                    return 1;
                break;
            }
            else
            {
                if (v6 - 0 >> 63 & 1)
                {
                    if (*(__errno_location()) == 38 || (char)sub_406bb9(*(__errno_location())) || *(__errno_location()) == 22 || *(__errno_location()) == 9 || *(__errno_location()) == 18 || *(__errno_location()) == 26 || *(__errno_location()) == 1 && !*(a10))
                        break;
                    if (*(__errno_location()) != 4)
                    {
                        sub_418941(1, 4, a8);
                        sub_418941(0, 4, a7);
                        v13 = gettext("error copying %s to %s");
                        err = __errno_location();
                        error(0, *(err), v13);
                        return 0;
                    }
                    v6 = 0;
                }
                iter -= v6;
            }
        }
    }
    v1 = 0;
    for (j = 0; iter; *(a11) = v1)
    {
        if (!*(a2))
            *(a2) = sub_41b8b1((int)getpagesize(), v0);
        v12 = *(a2);
        if (v0 > iter)
            v15 = iter;
        else
            v15 = v0;
        count = read(a0, v12, v15);
        if (!(count - 0 >> 63 & 1))
        {
            if (!count)
                break;
            iter -= count;
            *(a10) = *(a10) + count;
            v8 = (!a4 ? v0 : a4);
            k = v12;
            for (v10 = v12; count; k += v8)
            {
                v2 = v1;
                if (v8 <= count)
                    v16 = v8;
                else
                    v16 = count;
                v8 = v16;
                if (a4 && v8)
                    v1 = sub_406b03(k, v8, v8);
                if (v1 != v2 && j)
                    v17 = 1;
                else
                    v17 = 0;
                v3 = v17;
                v3 &= 1;
                if (count == v8 && v1 != 1 || !v8)
                    v18 = 1;
                else
                    v18 = 0;
                v4 = v18;
                v4 &= 1;
                if (v3 || v4)
                {
                    if (v3 != 1)
                        j += v8;
                    if (v2 != 1)
                    {
                        if (sub_414bff(a1, v10, j, v10) != j)
                        {
                            sub_418a0f(4, a8);
                            v23 = gettext("error writing %s");
                            err1 = __errno_location();
                            error(0, *(err1), v23);
                            return 0;
                        }
                    }
                    else
                    {
                        if ((char)sub_406d9e(a1, a8, a5, j) != 1)
                            return 0;
                    }
                    v10 = k;
                    j = v8;
                    if (v4)
                    {
                        if (!v8)
                            count = 0;
                        if (v3)
                            v8 = 0;
                        else
                            j = 0;
                    }
                }
                else
                {
                    v19 = 0;
                    v20 = v8 + j;
                    if (0x8000000000000000 + v20 < v8)
                        v19 = 1;
                    j = v20;
                    if (v19 & 1)
                    {
                        sub_418a0f(4, a7);
                        error(0, 0, gettext("overflow reading %s"));
                        return 0;
                    }
                }
                count -= v8;
            }
        }
        else if (*(__errno_location()) != 4)
        {
            sub_418a0f(4, a7);
            v21 = gettext("error reading %s");
            err2 = __errno_location();
            error(0, *(err2), v21);
            return 0;
        }
    }
    if (v1 && (char)sub_406d9e(a1, a8, a5, j) != 1)
        return 0;
    return 1;
}



// Function: clone_file @ 0x750c
int clone_file(int a0, unsigned int a1)
{
    unsigned int v0;  // [bp-0x10]

    v0 = a1;
    return ioctl(a0, 1074041865);
}



// Function: write_zeros @ 0x7533
extern unsigned long long g_427010;
extern unsigned long long g_4271c0;
extern char g_4271e0;

unsigned int write_zeros(unsigned int a0, unsigned long a1)
{
    unsigned long v4;  // rax
    unsigned long i;  // [bp-0x28]
    unsigned int v1;  // [bp-0x1c]
    unsigned long v2;  // [bp-0x10]

    v1 = a0;
    i = a1;
    if (!g_4271c0)
    {
        g_4271c0 = calloc(g_427010, 1);
        if (!g_4271c0)
        {
            g_4271c0 = &g_4271e0;
            g_427010 = 0x400;
        }
    }
    for (; i; i -= v2)
    {
        v4 = g_427010;
        if (i <= g_427010)
            v4 = i;
        v2 = v4;
        if (v2 != sub_414bff())
            return 0;
    }
    return 1;
}



// Function: lseek_copy @ 0x75ed
unsigned int lseek_copy(int a0, unsigned int a1, unsigned long long *a2, unsigned int a3, unsigned long long a4, unsigned long a5, unsigned long a6, unsigned int a7, char a8, long long a9, long long a10)
{
    unsigned long long v11;  // r10
    char *v12;  // rax
    int *err;  // rax
    char *v15;  // rax
    int *err1;  // rax
    char *v17;  // rax
    int *err2;  // rax
    char *v19;  // rax
    int *v20;  // rax
    unsigned long v0;  // [bp-0x90]
    unsigned long long v1;  // [bp-0x88]
    char v2;  // [bp-0x5a]
    char v3;  // [bp-0x59]
    unsigned long long v4;  // [bp-0x58]
    unsigned long v5;  // [bp-0x50]
    unsigned long long v6;  // [bp-0x48]
    unsigned long v7;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    long long v9;  // [bp-0x30]
    unsigned long long v10;  // [bp-0x28]

    v1 = a4;
    v0 = a5;
    v5 = 0;
    v6 = 0;
    v7 = 0;
    v3 = 1;
    while (!(v0 - 0 >> 63 & 1))
    {
        v8 = lseek(a0, v0, 4);
        if (v8 - 0 >> 63 & 1)
        {
            if (*(__errno_location()) != 6)
                goto LABEL_407a25;
            v8 = a6;
            if (v8 <= v0)
            {
                a6 = lseek(a0, 0, 2);
                if (a6 - 0 >> 63 & 1)
                    goto LABEL_407a25;
                if (a6 <= v0)
                    break;
                v8 = a6;
            }
        }
        if (a6 < v8)
            a6 = v8;
        if (lseek(a0, v0, 0) < 0)
        {
LABEL_407a25:
            sub_418a0f(4, a9);
            v19 = gettext("cannot lseek %s");
            v20 = __errno_location();
            error(0, *(v20), v19);
            return 0;
        }
        v3 = 0;
        v9 = v0 - v5 - v6;
        if (v9)
        {
            if (a7 != 1)
            {
                if ((char)sub_406d9e(a1, a10, a7 == 3, v9) != 1)
                    return 0;
                v3 = 1;
            }
            else if ((char)sub_407533(a1, v9) != 1)
            {
                sub_418ba1(0, 3, a10);
                v12 = gettext("%s: write failed");
                err = __errno_location();
                error(0, *(err), v12);
                return 0;
            }
        }
        v10 = v8 - v0;
        v5 = v0;
        v6 = v10;
        if (a7 != 1)
            v11 = v1;
        else
            v11 = 0;
        if ((char)sub_406ea8(a0, a1, a2, a3, v11, 1, a8, a9, a10, v10, &v4, &v2) != 1)
            return 0;
        v7 = v0 + v4;
        if (v4)
            v3 = v2;
        if (v10 > v4)
        {
            a6 = v7;
            break;
        }
        v0 = lseek(a0, v7, 3);
        if (v0 - 0 >> 63 & 1 && *(__errno_location()) != 6)
            goto LABEL_407a25;
    }
    if ((v7 < a6 || v3) && (a7 == 1 ? (char)sub_407533(a1, a6 - v7) ^ 1 : ftruncate(a1, a6)))
    {
        sub_418a0f(4, a10);
        v15 = gettext("failed to extend %s");
        err1 = __errno_location();
        error(0, *(err1), v15);
        return 0;
    }
    if (a7 == 3 && v7 < a6 && sub_406d36(a1, v7, a6 - v7) < 0)
    {
        sub_418a0f(4, a10);
        v17 = gettext("error deallocating %s");
        err2 = __errno_location();
        error(0, *(err2), v17);
        return 0;
    }
    return 1;
}



// Function: is_ancestor @ 0x7a8e
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_0;

unsigned int is_ancestor(unsigned long a0, struct_0 *a1)
{
    struct_0 *i;  // [bp-0x18]

    for (i = a1; i; i = i->field_0)
    {
        if (i->field_8 == *((long long *)(a0 + 8)) && i->field_10 == *((long long *)a0))
            return 1;
    }
    return 0;
}



// Function: errno_unsupported @ 0x7ae9
unsigned int errno_unsupported(unsigned int a0)
{
    unsigned int v1;  // eax

    if (a0 == 95 || a0 == 61)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: copy_attr_error @ 0x7b11
void copy_attr_error(unsigned long long a0, long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v14;  // fs
    unsigned long long v0;  // [bp-0xf0]
    unsigned int v1;  // [bp-0xdc]
    unsigned int v2;  // [bp-0xd8]
    unsigned int v3;  // [bp-0xd4]
    char *v4;  // [bp-0xd0]
    char *v5;  // [bp-0xc8]
    unsigned long v6;  // [bp-0xc0]
    char v7;  // [bp-0xb8]
    unsigned long long v8;  // [bp-0xa8]
    unsigned long long v9;  // [bp-0xa0]
    unsigned long long v10;  // [bp-0x98]
    unsigned long long v11;  // [bp-0x90]
    unsigned long v12;  // [bp+0x8]

    v0 = a0;
    v8 = a2;
    v9 = a3;
    v10 = a4;
    v11 = a5;
    v6 = *((long long *)(40 + v14));
    if ((char)sub_407ae9(*(__errno_location())) != 1)
    {
        v1 = *(__errno_location());
        v2 = 16;
        v3 = 48;
        v4 = &v12;
        v5 = &v7;
        sub_41ae51(0, v1, a1, &v2);
    }
    if (v6 == *((long long *)(40 + v14)))
        return;
    __stack_chk_fail(); /* do not return */
}



// Function: copy_attr_allerror @ 0x7c09
void copy_attr_allerror(unsigned long long a0, long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4, unsigned long long a5)
{
    unsigned long v14;  // fs
    unsigned long long v0;  // [bp-0xf0]
    unsigned int v1;  // [bp-0xdc]
    unsigned int v2;  // [bp-0xd8]
    unsigned int v3;  // [bp-0xd4]
    char *v4;  // [bp-0xd0]
    char *v5;  // [bp-0xc8]
    unsigned long v6;  // [bp-0xc0]
    char v7;  // [bp-0xb8]
    unsigned long long v8;  // [bp-0xa8]
    unsigned long long v9;  // [bp-0xa0]
    unsigned long long v10;  // [bp-0x98]
    unsigned long long v11;  // [bp-0x90]
    unsigned long v12;  // [bp+0x8]

    v0 = a0;
    v8 = a2;
    v9 = a3;
    v10 = a4;
    v11 = a5;
    v6 = *((long long *)(40 + v14));
    v1 = *(__errno_location());
    v2 = 16;
    v3 = 48;
    v4 = &v12;
    v5 = &v7;
    sub_41ae51(0, v1, a1, &v2);
    if (v6 == *((long long *)(40 + v14)))
        return;
    __stack_chk_fail(); /* do not return */
}



// Function: copy_attr_quote @ 0x7cec
long long copy_attr_quote(unsigned long long a0, long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0;
    return sub_418a0f(4, a1);
}



// Function: copy_attr_free @ 0x7d13
void copy_attr_free(unsigned long a0, unsigned long a1)
{
    return;
}



// Function: check_selinux_attr @ 0x7d26
unsigned int check_selinux_attr(unsigned long a0, long long a1)
{
    if (strncmp(a0, "security.selinux", 16) && attr_copy_check_permissions(a0, a1, a1))
        return 1;
    return 0;
}



// Function: copy_attr @ 0x7d7e
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

char copy_attr(long long a0, unsigned int a1, long long a2, unsigned int a3, struct_0 *a4)
{
    char v8;  // al
    char v9;  // al
    long long v10;  // rax
    char *v11;  // rax
    char v0;  // [bp-0x3a]
    char v1;  // [bp-0x39]
    long long v2;  // [bp-0x38]
    char *v3;  // [bp-0x30]
    unsigned long v4;  // [bp-0x28]
    unsigned long long v5;  // [bp-0x20]
    unsigned long long v6;  // [bp-0x18]

    if (a4->field_31 != 1 || a4->field_36)
        v8 = 1;
    else
        v8 = 0;
    v0 = v8;
    v0 &= 1;
    if (v0 != 1 && a4->field_37 != 1)
        v9 = 1;
    else
        v9 = 0;
    v1 = v9;
    v1 &= 1;
    if (a4->field_33 || a4->field_28)
        v10 = sub_407d26;
    else
        v10 = 0;
    v2 = v10;
    if (v0 || v1)
    {
        v4 = (!v0 ? sub_407b11 : sub_407c09);
        v5 = sub_407cec;
        v6 = sub_407d13;
        v11 = &v4;
    }
    else
    {
        v11 = NULL;
    }
    v3 = v11;
    if (a1 >= 0 && a3 >= 0)
        return !attr_copy_fd(a0, a1, a2, a3, v2, v3);
    return !attr_copy_file(a0, a2, v2, v3);
}



// Function: copy_dir @ 0x7ee6
typedef struct struct_0 {
    unsigned int field_0[2];
    unsigned int field_4;
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
} struct_0;

unsigned int copy_dir(long long a0, long long a1, unsigned int a2, unsigned long a3, char a4, long long a5, long long a6, struct_0 *idx, char *a8, char *a9)
{
    char *v21;  // rax
    int *err;  // rax
    char v0;  // [bp-0x9d]
    char v1;  // [bp-0x9c]
    char v2;  // [bp-0x9b]
    char v3;  // [bp-0x9a]
    char v4;  // [bp-0x99]
    unsigned long i;  // [bp-0x98]
    unsigned long v6;  // [bp-0x90]
    unsigned long v7;  // [bp-0x88]
    unsigned long v8;  // [bp-0x80]
    unsigned long v9;  // [bp-0x78]
    unsigned int v10;  // [bp-0x74]
    unsigned long v11;  // [bp-0x70]
    unsigned long v12;  // [bp-0x68]
    unsigned long v13;  // [bp-0x60]
    unsigned long v14;  // [bp-0x58]
    unsigned long v15;  // [bp-0x50]
    unsigned long v16;  // [bp-0x48]
    unsigned long v17;  // [bp-0x40]
    unsigned long v18;  // [bp-0x38]
    unsigned long v19;  // [bp-0x30]
    unsigned long v20;  // [bp-0x28]

    v9 = (unsigned long)idx->field_0;
    v11 = *((long long *)&idx->field_4);
    v12 = idx->field_8;
    v13 = idx->field_10;
    v14 = idx->field_18;
    v15 = idx->field_20;
    v16 = idx->field_28;
    v17 = idx->field_30;
    v18 = idx->field_38;
    v19 = idx->field_40;
    v20 = idx->field_48;
    v3 = 1;
    v6 = sub_4199a1(a0, 2);
    if (!v6)
    {
        sub_418a0f(4, a0);
        v21 = gettext("cannot access %s");
        err = __errno_location();
        error(0, *(err), v21);
        return 0;
    }
    if (idx->field_0[1] == 3)
        v10 = 2;
    v4 = 0;
    for (i = v6; *((char *)i); i += strlen(i) + 1)
    {
        v7 = sub_411da5(a0, i, 0, i);
        v8 = sub_411da5(a1, i, 0, i);
        v1 = *(a8);
        v3 = (char)sub_40abda(v7, v8, a2, a3 - a1 + v8, a4, a5, a6, &v9, 0, &v1, &v0, &v2) & v3;
        *(a9) = v0 || *(a9);
        free(v8);
        free(v7);
        if (v0)
            break;
        v4 = v1 || v4;
    }
    free(v6);
    *(a8) = v4;
    return v3;
}



// Function: set_owner @ 0x820f
typedef struct struct_1 {
    char padding_0[24];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[16];
    unsigned int field_10;
    char padding_14[4];
    char field_18;
    char padding_19[5];
    char field_1e;
    char padding_1f[19];
    char field_32;
    char padding_33[6];
    char field_39;
} struct_0;

unsigned int set_owner(struct_0 *a0, long long a1, unsigned int a2, long long a3, unsigned int a4, unsigned int *idx, char a6, struct_1 *a7)
{
    unsigned int v9;  // eax
    char *v10;  // rax
    int *err;  // rax
    char *v12;  // rax
    int *err1;  // rax
    unsigned int v0;  // [bp-0x3c]
    unsigned int v1;  // [bp-0x38]
    unsigned int v2;  // [bp-0x34]
    unsigned int v3;  // [bp-0x30]
    unsigned int v4;  // [bp-0x2c]
    unsigned int v5;  // [bp-0x28]
    unsigned int v6;  // [bp-0x24]
    unsigned int v7;  // [bp-0x20]
    unsigned int v8;  // [bp-0x1c]

    v0 = idx[7];
    v1 = idx[8];
    if (a6 != 1 && (a0->field_1e || a0->field_18 || a0->field_39))
    {
        v2 = a7->field_18;
        if (a0->field_1e || a0->field_18)
            v9 = idx[6];
        else
            v9 = a0->field_10;
        v3 = v9;
        v4 = v2 & v3 & 448;
        if ((~((unsigned short)v3) | 14) & (unsigned short)v2 & 0xfff && sub_416e81(a1, a4, v4, a4))
        {
            if ((char)sub_40dc67(a0) == 1)
                return -(a0->field_32);
            sub_418a0f(4, a1);
            v10 = gettext("clearing permissions for %s");
            err = __errno_location();
            error(0, *(err), v10);
            return -(a0->field_32);
        }
    }
    if (a4 != 0xffffffff)
    {
        if (!fchown(a4, v0, v1))
        {
            return 1;
        }
        else if (*(__errno_location()) == 1 || *(__errno_location()) == 22)
        {
            v7 = *(__errno_location());
            v8 = fchown(a4, 0xffffffff, v1);
            *(__errno_location()) = v7;
        }
    }
    else
    {
        if (!sub_4112c3(a2, a3, v0, v1))
        {
            return 1;
        }
        else if (*(__errno_location()) == 1 || *(__errno_location()) == 22)
        {
            v5 = *(__errno_location());
            v6 = sub_4112c3(a2, a3, 0xffffffff, v1);
            *(__errno_location()) = v5;
        }
    }
    if ((char)sub_40dc1f(a0) != 1)
    {
        sub_418a0f(4, a1);
        v12 = gettext("failed to preserve ownership for %s");
        err1 = __errno_location();
        error(0, *(err1), v12);
        if (a0->field_32)
            return 0xffffffff;
    }
    return 0;
}



// Function: set_author @ 0x84b0
void set_author(unsigned long a0, unsigned long a1, unsigned long a2)
{
    return;
}



// Function: set_process_security_ctx @ 0x84c6
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

unsigned int set_process_security_ctx(long long a0, long long a1, unsigned int a2, char a3, struct_0 *a4)
{
    char v4;  // al
    char v5;  // al
    char *v6;  // rax
    int *err;  // rax
    char *v8;  // rax
    int *err1;  // rax
    char *v10;  // rax
    int *err2;  // rax
    char v0;  // [bp-0x2a]
    char v1;  // [bp-0x29]
    long long v2;  // [bp-0x28]

    if (a4->field_33)
    {
        if (a4->field_31 != 1 || a4->field_34)
            v4 = 1;
        else
            v4 = 0;
        v0 = v4;
        v0 &= 1;
        if (v0 != 1 && a4->field_37 != 1)
            v5 = 1;
        else
            v5 = 0;
        v1 = v5;
        v1 &= 1;
        if (sub_419d8b(a0, &v2, &v2) >= 0)
        {
            if (setfscreatecon(v2) < 0)
            {
                if (v0 || v1 && (char)sub_407ae9(*(__errno_location())) != 1)
                {
                    sub_418e56(v2);
                    v6 = gettext("failed to set default file creation context to %s");
                    err = __errno_location();
                    error(0, *(err), v6);
                }
                if (a4->field_34)
                {
                    freecon(v2);
                    return 0;
                }
            }
            freecon(v2);
        }
        else
        {
            if (v0 || v1 && (char)sub_407ae9(*(__errno_location())) != 1)
            {
                sub_418a0f(4, a0);
                v8 = gettext("failed to get security context of %s");
                err1 = __errno_location();
                error(0, *(err1), v8);
            }
            if (a4->field_34)
                return 0;
        }
    }
    else if (a4->field_28 && a3 && sub_40e549(a4->field_28, a1, a2, a1) < 0 && (char)sub_406c97(*(__errno_location())) != 1)
    {
        sub_418a0f(4, a1);
        v10 = gettext("failed to set default file creation context for %s");
        err2 = __errno_location();
        error(0, *(err2), v10);
    }
    return 1;
}



// Function: set_file_security_ctx @ 0x8750
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
    char v3;  // al
    char v4;  // al
    char *v5;  // rax
    int *err;  // rax
    char v0;  // [bp-0x1a]
    char v1;  // [bp-0x19]

    if (a2->field_31 != 1 || a2->field_34)
        v3 = 1;
    else
        v3 = 0;
    v0 = v3;
    v0 &= 1;
    if (v0 != 1 && a2->field_37 != 1)
        v4 = 1;
    else
        v4 = 0;
    v1 = v4;
    v1 &= 1;
    if ((char)sub_40ead7(a2->field_28, a0, a1, a0) == 1)
        return 1;
    if (v0 || v1 && (char)sub_407ae9(*(__errno_location())) != 1)
    {
        sub_418941(0, 4, a0);
        v5 = gettext("failed to set the security context of %s");
        err = __errno_location();
        error(0, *(err), v5);
    }
    return 0;
}



// Function: fchmod_or_lchmod @ 0x8868
unsigned int fchmod_or_lchmod(int a0, unsigned int a1, long long a2, unsigned int a3)
{
    return (a0 < 0 ? (unsigned int)sub_411260(a1, a2, a3, a2) : (int)fchmod(a0, a3));
}



// Function: infer_scantype @ 0x88ae
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
    char padding_1c[20];
    long long field_30;
} struct_0;

unsigned int infer_scantype(int a0, struct_0 *a1, void* a2)
{
    long long v1;  // rax

    if (((unsigned short)a1->field_18 & 0xf000) == 0x8000)
    {
        v1 = a1->field_30;
        if (*((long long *)&a1[1].padding_0[8]) < (long long)((v1 < 0 ? v1 + 0x1ff : v1)) >> 9)
        {
            *((long *)a2) = lseek(a0, 0, 3);
            if (*((long long *)a2) >= 0 || *(__errno_location()) == 6)
                return 3;
            if (*(__errno_location()) == 22 || (char)sub_406bb9(*(__errno_location())) == 1)
                return 2;
            return 0;
            return 2;
            return 3;
        }
    }
    return 1;
}



// Function: copy_reg @ 0x896f
typedef struct struct_1 {
    char padding_0[24];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    char padding_8[4];
    unsigned int field_c;
    unsigned int field_10;
    char padding_14[2];
    char field_16;
    char padding_17[1];
    char field_18;
    char padding_19[2];
    char field_1b;
    char padding_1c[1];
    char field_1d;
    char field_1e;
    char field_1f;
    char field_20;
    char padding_21[7];
    unsigned long long field_28;
    char padding_30[1];
    char field_31;
    char field_32;
    char field_33;
    char field_34;
    char field_35;
    char field_36;
    char padding_37[2];
    char field_39;
    char padding_3a[2];
    char field_3c;
    char padding_3d[1];
    char field_3e;
    char padding_3f[5];
    unsigned int field_44;
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

unsigned int copy_reg(long long a0, char *a1, unsigned int a2, long long a3, struct_0 *a4, unsigned int a5, unsigned int a6, unsigned long a7, void* a8)
{
    char *v65;  // rax
    int *err;  // rax
    char *v76;  // rax
    int *err1;  // rax
    char *v79;  // rax
    int *err2;  // rax
    unsigned long v81;  // rax
    char *v82;  // rax
    int *v83;  // rax
    char v84;  // al
    long long v85;  // rax
    char v86;  // al
    char *v87;  // rax
    int *v88;  // rax
    unsigned long long v89;  // rdx
    char *v90;  // rax
    int *v91;  // rax
    unsigned int v92;  // eax
    char *v93;  // rax
    int *v94;  // rax
    char *v68;  // rax
    char *v95;  // rax
    int *v96;  // rax
    char *v98;  // rax
    int *v99;  // rax
    int *v69;  // rax
    char *v71;  // rax
    int *v72;  // rax
    unsigned int v73;  // eax
    unsigned long v0;  // [bp-0x2b8]
    unsigned long v1;  // [bp-0x2b0]
    unsigned long v2;  // [bp-0x2a8]
    unsigned long v3;  // [bp-0x2a0]
    unsigned long v4;  // [bp-0x298]
    unsigned long v5;  // [bp-0x290]
    unsigned long v6;  // [bp-0x288]
    unsigned long v7;  // [bp-0x280]
    unsigned long v8;  // [bp-0x278]
    unsigned long long v9;  // [bp-0x270]
    unsigned long long v10;  // [bp-0x268]
    unsigned long long v11;  // [bp-0x260]
    unsigned long long v12;  // [bp-0x258]
    unsigned long long v13;  // [bp-0x250]
    unsigned long long v14;  // [bp-0x248]
    unsigned long long v15;  // [bp-0x240]
    unsigned long long v16;  // [bp-0x238]
    unsigned long long v17;  // [bp-0x230]
    char v18;  // [bp-0x1dd]
    char v19;  // [bp-0x1dc]
    char v20;  // [bp-0x1db]
    char v21;  // [bp-0x1da]
    char v22;  // [bp-0x1d9]
    int v23;  // [bp-0x1d8]
    unsigned int v24;  // [bp-0x1d4]
    unsigned int v25;  // [bp-0x1d0]
    unsigned int v26;  // [bp-0x1cc]
    unsigned int v27;  // [bp-0x1c8]
    unsigned int v28;  // [bp-0x1c4]
    unsigned int v29;  // [bp-0x1c0]
    unsigned int v30;  // [bp-0x1bc]
    unsigned int v31;  // [bp-0x1b8]
    unsigned int v32;  // [bp-0x1b4]
    unsigned long long v33;  // [bp-0x1b0]
    char v34;  // [bp-0x1a8], Other Possible Types: unsigned int
    int v35;  // [bp-0x1a0], Other Possible Types: unsigned long long
    unsigned long v36;  // [bp-0x198]
    unsigned long v37;  // [bp-0x190]
    unsigned long v38;  // [bp-0x188]
    unsigned long v39;  // [bp-0x180]
    unsigned long v40;  // [bp-0x178]
    unsigned long long v41;  // [bp-0x170]
    unsigned long long v42;  // [bp-0x168]
    unsigned long long v43;  // [bp-0x160]
    stat v44;  // [bp-0x158], Other Possible Types: struct_1
    unsigned long long v45;  // [bp-0x110]
    unsigned long long v46;  // [bp-0x108]
    unsigned long long v47;  // [bp-0x100]
    unsigned long long v48;  // [bp-0xf8]
    unsigned long long v49;  // [bp-0xf0]
    unsigned long long v50;  // [bp-0xe8]
    unsigned long long v51;  // [bp-0xe0]
    unsigned long long v52;  // [bp-0xd8]
    unsigned long long v53;  // [bp-0xd0]
    int v54;  // [bp-0xc8]
    unsigned long long v55;  // [bp-0x80]
    unsigned long long v56;  // [bp-0x78]
    unsigned long long v57;  // [bp-0x70]
    unsigned long long v58;  // [bp-0x68]
    unsigned long long v59;  // [bp-0x60]
    unsigned long long v60;  // [bp-0x58]
    unsigned long long v61;  // [bp-0x50]
    unsigned long long v62;  // [bp-0x48]
    unsigned long long v63;  // [bp-0x40]
    char v64;  // [bp-0x31]

    v33 = 0;
    v25 = (int)a8[24];
    v19 = 1;
    v20 = a4->field_31;
    v21 = a4->field_35;
    v27 = sub_411785(a0, (a4->field_4 == 2 ? 0x20000 : 0));
    if (v27 < 0)
    {
        sub_418a0f(4, a0);
        v65 = gettext("cannot open %s for reading");
        err = __errno_location();
        error(0, *(err), v65);
        return 0;
    }
    if (fstat(v27, &v54))
    {
        sub_418a0f(4, a0);
        v68 = gettext("cannot fstat %s");
        v69 = __errno_location();
        error(0, *(v69), v68);
        v19 = 0;
    }
    else if ((long long)a8[8] != *((unsigned long long *)(&v54 + 8)) || *((long long *)a8) != *((unsigned long long *)&v54))
    {
        sub_418a0f(4, a0);
        error(0, 0, gettext("skipping file %s, as it was replaced while being copied"));
        v19 = 0;
    }
    else if (*((char *)a7) != 1)
    {
        v28 = (!v20 ? 1 : 513);
        v23 = sub_416b35(a2, a3, v28, a3);
        v24 = *(__errno_location());
        if (v23 >= 0 && (a4->field_28 || a4->field_33) && (char)sub_408750(a1, 0, a4) != 1 && a4->field_34)
        {
            v19 = 0;
            goto LABEL_409a22;
        }
        if (v23 < 0 && v24 != 2 && a4->field_16)
        {
            if (!unlinkat(a2, a3, 0, a3))
            {
                if (a4->field_3c)
                {
                    sub_418a0f(4, a1);
                    printf(gettext("removed %s\n"));
                }
LABEL_408d56:
                v24 = 2;
            }
            else
            {
                if (*(__errno_location()) == 2)
                    goto LABEL_408d56;
                sub_418a0f(4, a1);
                v71 = gettext("cannot remove %s");
                v72 = __errno_location();
                error(0, *(v72), v71);
                v19 = 0;
                goto LABEL_409a84;
            }
        }
        if (v23 >= 0 || v24 != 2)
            goto LABEL_408dca;
        if (a4->field_28 && (char)sub_4084c6(a0, a1, a5, 1, a4) != 1)
        {
            v19 = 0;
        }
        else
        {
            *((char *)a7) = 1;
            goto LABEL_408dca;
        }
    }
    else
    {
LABEL_408dca:
        if (!*((char *)a7))
        {
            v26 = 0;
            a6 = v26;
LABEL_408fc4:
            if (v23 < 0)
            {
                sub_418a0f(4, a1);
                error(0, v24, gettext("cannot create regular file %s"));
                v19 = 0;
                goto LABEL_409a84;
            }
            else
            {
                if (v20 && a4->field_44)
                {
                    if (!sub_40750c(v23, v27))
                    {
                        v20 = 0;
                    }
                    else if (a4->field_44 == 2)
                    {
                        sub_418941(1, 4, a0);
                        sub_418941(0, 4, a1);
                        v76 = gettext("failed to clone %s from %s");
                        err1 = __errno_location();
                        error(0, *(err1), v76);
                        v19 = 0;
                        goto LABEL_409a22;
                    }
                }
                if (!a4->field_1d && !v20 && !v26)
                {
                    v44.field_18 = 0;
LABEL_40917e:
                    v31 = v44.field_18 | v26;
                    if (v31 != v44.field_18 && sub_408868(v23, a2, a3, v31))
                        v26 = 0;
                    if (v20)
                    {
                        v17 = v53;
                        v16 = v52;
                        v15 = v51;
                        v14 = v50;
                        v13 = v49;
                        v12 = v48;
                        v11 = v47;
                        v10 = v46;
                        v9 = v45;
                        v8 = *((unsigned long long *)((void*)&v44 + 64));
                        v7 = *((unsigned long long *)((void*)&v44 + 56));
                        v6 = *((unsigned long long *)((void*)&v44 + 48));
                        v5 = *((unsigned long long *)((void*)&v44 + 40));
                        v4 = *((unsigned long long *)((void*)&v44 + 32));
                        v3 = *((unsigned long long *)((void*)&v44 + 24));
                        v2 = *((unsigned long long *)((void*)&v44 + 16));
                        v1 = *((unsigned long long *)((void*)&v44 + 8));
                        v0 = *((unsigned long long *)&v44);
                        v36 = sub_406bc9();
                        if (*((unsigned long long *)((void*)&v44 + 56)) > 0 && *((unsigned long long *)((void*)&v44 + 56)) <= 0x2000000000000000)
                            v81 = *((unsigned long long *)((void*)&v44 + 56));
                        else
                            v81 = 0x200;
                        v37 = v81;
                        v32 = sub_4088ae(v27, &v54, &v34);
                        if (!v32)
                        {
                            sub_418a0f(4, a0);
                            v82 = gettext("cannot lseek %s");
                            v83 = __errno_location();
                            error(0, *(v83), v82);
                            v19 = 0;
                        }
                        else
                        {
                            if (((unsigned short)v44.field_18 & 0xf000) == 0x8000 && (a4->field_c == 3 || a4->field_c == 2 && v32 != 1))
                                v84 = 1;
                            else
                                v84 = 0;
                            v22 = v84;
                            v22 &= 1;
                            sub_411711(v27, 0, 0, 2);
                            if (v22 != 1)
                            {
                                v38 = 0x7fffffffffffffff;
                                v17 = v63;
                                v16 = v62;
                                v15 = v61;
                                v14 = v60;
                                v13 = v59;
                                v12 = v58;
                                v11 = v57;
                                v10 = v56;
                                v9 = v55;
                                v8 = *((unsigned long long *)(&v54 + 64));
                                v7 = *((unsigned long long *)(&v54 + 56));
                                v6 = *((unsigned long long *)(&v54 + 48));
                                v5 = *((unsigned long long *)(&v54 + 40));
                                v4 = *((unsigned long long *)(&v54 + 32));
                                v3 = *((unsigned long long *)(&v54 + 24));
                                v2 = *((unsigned long long *)(&v54 + 16));
                                v1 = *((unsigned long long *)(&v54 + 8));
                                v0 = *((unsigned long long *)&v54);
                                v85 = sub_406bc9();
                                v39 = sub_410429(v85, v36, v38, v85);
                                if (((unsigned short)*((unsigned int *)(&v54 + 24)) & 0xf000) == 0x8000 && v36 > *((unsigned long long *)(&v54 + 48)))
                                    v36 = *((unsigned long long *)(&v54 + 48)) + 1;
                                v36 = v36 + v39 - 1;
                                v36 -= v36 % v39;
                                if (!v36 || v38 < v36)
                                    v36 = v39;
                            }
                            v18 = 0;
                            if (v32 == 3)
                                v86 = (char)sub_4075ed(v27, v23, &v33, v36, v37, v34, *((unsigned long long *)(&v54 + 48)), (!v22 ? 1 : a4->field_c), a4->field_44, a0, a1) ^ 1;
                            else
                                v86 = (char)sub_406ea8(v27, v23, &v33, v36, (!v22 ? 0 : v37), a4->field_c == 3, a4->field_44, a0, a1, 0xffffffffffffffff, &v35, &v18) ^ 1;
                            if (v86)
                            {
                                v19 = 0;
                            }
                            else
                            {
                                if (!v18 || ftruncate(v23, v35) >= 0)
                                    goto LABEL_409666;
                                sub_418a0f(4, a1);
                                v87 = gettext("failed to extend %s");
                                v88 = __errno_location();
                                error(0, *(v88), v87);
                                v19 = 0;
                            }
                        }
                    }
                    else
                    {
LABEL_409666:
                        if (a4->field_1f)
                        {
                            v40 = sub_419e57(a8);
                            v41 = v89;
                            v42 = sub_419e8b(a8);
                            v43 = v89;
                            if (sub_4118c1(v23, a2, a3, &v40, 0))
                            {
                                sub_418a0f(4, a1);
                                v90 = gettext("preserving times for %s");
                                v91 = __errno_location();
                                error(0, *(v91), v90);
                                if (a4->field_32)
                                {
                                    v19 = 0;
                                    goto LABEL_409a22;
                                }
                            }
                        }
                        if (a4->field_1d && ((int)a8[28] != *((unsigned int *)((void*)&v44 + 28)) || (int)a8[32] != *((unsigned int *)((void*)&v44 + 32))))
                        {
                            v92 = sub_40820f(a4, a1, a2, a3, v23, a8, *((char *)a7), &v44);
                            if (v92 == 0xffffffff)
                            {
                                v19 = 0;
                                goto LABEL_409a22;
                            }
                            else if (!v92)
                            {
                                v25 &= 0xfffff1ff;
                            }
                        }
                        if (v21 && sub_407d7e(a0, v27, a1, v23, a4) != 1 && a4->field_36)
                            v19 = 0;
                        sub_4084b0(a1, v23, a8);
                        if (a4->field_1e || a4->field_18)
                        {
                            if (sub_40ec57(a0, v27, a1, v23, v25) && a4->field_32)
                                v19 = 0;
                        }
                        else if (a4->field_39)
                        {
                            if (sub_40ed24(a1, v23, a4->field_10, v23))
                                v19 = 0;
                        }
                        else
                        {
                            if (a4->field_20 && *((char *)a7))
                            {
                                if (sub_40ed24(a1, v23, ~((unsigned int)sub_40dcaf()) & 438, v23))
                                    v19 = 0;
                            }
                            else
                            {
                                if ((a6 || v26) && !(a6 &= ~((unsigned int)(unsigned long long)sub_40dcaf()), !a6 && !v26 || !(unsigned int)(unsigned long long)sub_408868(v23, a2, a3, ~((unsigned int)(unsigned long long)sub_40dcaf()) & a5)))
                                {
                                    sub_418a0f(4, a1);
                                    v93 = gettext("preserving permissions for %s");
                                    v94 = __errno_location();
                                    error(0, *(v94), v93);
                                    if (!a4->field_32)
                                        goto LABEL_409a21;
                                    v19 = 0;
                                }
                                else
                                {
LABEL_409a21:
                                }
                            }
                        }
                    }
                }
                else
                {
                    if (!fstat(v23, &v44))
                        goto LABEL_40917e;
                    sub_418a0f(4, a1);
                    v79 = gettext("cannot fstat %s");
                    err2 = __errno_location();
                    error(0, *(err2), v79);
                    v19 = 0;
                }
LABEL_409a22:
                if (close(v23) < 0)
                {
                    sub_418a0f(4, a1);
                    v95 = gettext("failed to close %s");
                    v96 = __errno_location();
                    error(0, *(v96), v95);
                    v19 = 0;
                    goto LABEL_409a84;
                }
            }
        }
        if (v21 && a4->field_1b != 1)
            v73 = 128;
        else
            v73 = 0;
        v29 = v73 | ~(a6) & a5;
        v26 = ~(a5) & v29;
        v30 = 65;
        v23 = sub_416b35(a2, a3, (v30 | 128) & 0xffffffff, v29);
        v24 = *(__errno_location());
        if (!(v23 < 0 && v24 == 0x11 && a4->field_18 != 1 && readlinkat(a2, a3, &v64, 1) >= 0))
        {
LABEL_408f61:
            if (v23 < 0 && v24 == 21 && *(a1) && *(&a1[strlen(a1)] - 1) == 47)
            {
                v24 = 20;
                goto LABEL_408fc4;
            }
        }
        else if (a4->field_3e)
        {
            v23 = sub_416b35(a2, a3, v30, v29);
            v24 = *(__errno_location());
            goto LABEL_408f61;
        }
        else
        {
            sub_418a0f(4, a1);
            error(0, 0, gettext("not writing through dangling symlink %s"));
            v19 = 0;
        }
    }
LABEL_409a84:
    if (close(v27) < 0)
    {
        sub_418a0f(4, a0);
        v98 = gettext("failed to close %s");
        v99 = __errno_location();
        error(0, *(v99), v98);
        v19 = 0;
    }
    sub_40eda5(v33);
    return v19;
}



// Function: same_file_ok @ 0x9b18
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

unsigned long long same_file_ok(unsigned long a0, void* a1, unsigned int a2, long long a3, void* a4, unsigned long a5, unsigned long a6)
{
    char v11;  // al
    unsigned int v12;  // eax
    char v13;  // al
    void* v15;  // rax
    void* v17;  // rax
    char v0;  // [bp-0x164]
    char v1;  // [bp-0x163]
    char v2;  // [bp-0x162]
    char v3;  // [bp-0x161]
    void* v4;  // [bp-0x160]
    void* v5;  // [bp-0x158]
    unsigned long v6;  // [bp-0x150]
    int v7;  // [bp-0x148], Other Possible Types: char
    unsigned long long v8;  // [bp-0x140]
    unsigned int v9;  // [bp-0x130]
    stat v10;  // [bp-0xb8], Other Possible Types: int

    if ((long long)a1[8] == (long long)a4[8] && *((long long *)a1) == *((long long *)a4))
        v11 = 1;
    else
        v11 = 0;
    v1 = v11;
    v1 &= 1;
    *((char *)a6) = 0;
    if (v1 && *((char *)(a5 + 23)))
    {
        *((char *)a6) = 1;
        return 1;
    }
    if (*((int *)(a5 + 4)) == 2)
    {
        v0 = v1;
        if (((short)(int)a1[24] & 0xf000) == 0xa000 && ((short)(int)a4[24] & 0xf000) == 0xa000)
        {
            v2 = sub_419370(0xffffff9c, a0, a2, a3);
            if (v2 == 1)
                return !v2;
            if (*((int *)a5))
                return 1;
            if (v0)
            {
                *((char *)a6) = 1;
                return !*((char *)(a5 + 24));
            }
            return !v2;
        }
        v4 = a1;
        v5 = a4;
    }
    else if (v1 != 1)
    {
        return 1;
    }
    else if (fstatat(a2, a3, &v7, 0x100) || !(v12 = (unsigned int)(int)(long long)lstat(a0, &v10), !(int)(long long)lstat(a0, &v10)))
    {
        return 1;
    }
    else
    {
        v4 = &v10;
        v5 = &v7;
        if (v10.st_ino == v8 && *((long long *)&v10.st_mode) == *((long long *)&v7))
            v13 = 1;
        else
            v13 = 0;
        v0 = v13;
        v0 &= 1;
        if (((unsigned short)v10.st_nlink & 0xf000) == 0xa000 && ((unsigned short)v9 & 0xf000) == 0xa000 && *((char *)(a5 + 21)))
            return 1;
    }
    if (!*((int *)a5))
    {
        if (*((char *)(a5 + 24)) || *((char *)(a5 + 21)))
        {
            if (((short)(int)v5[24] & 0xf000) == 0xa000)
                return 1;
            if (v0 && (long long)v5[16] > 1 && (char)sub_419370(0xffffff9c, a0, a2, a3) != 1)
                return !*((char *)(a5 + 24));
        }
        if (((short)(int)v4[24] & 0xf000) != 0xa000 && ((short)(int)v5[24] & 0xf000) != 0xa000)
        {
            if ((long long)v4[8] != (long long)v5[8] || *((long long *)v4) != *((long long *)v5))
                return 1;
            if (*((char *)(a5 + 23)))
            {
                *((char *)a6) = 1;
                return 1;
            }
        }
        if (*((char *)(a5 + 24)) && ((short)(int)a1[24] & 0xf000) == 0xa000 && (long long)v5[16] > 1)
        {
            v6 = canonicalize_file_name(a0);
            if (v6)
            {
                v3 = !(char)sub_419370(0xffffff9c, v6, a2, a3);
                v3 &= 1;
                free(v6);
                return v3;
            }
        }
        if (*((char *)(a5 + 58)) && ((short)(int)v5[24] & 0xf000) == 0xa000)
            return 1;
        if (*((int *)(a5 + 4)) != 2)
            return 0;
        if (((short)(int)v4[24] & 0xf000) != 0xa000)
        {
            v15 = v4;
            *((long long *)&v10) = *((long long *)v15);
            *((long long *)&(&v10)[8]) = (long long)v15[8];
            *((long long *)&(&v10)[16]) = (long long)v15[16];
            *((long long *)&(&v10)[24]) = (long long)v15[24];
            *((long long *)&(&v10)[32]) = (long long)v15[32];
            *((long long *)&(&v10)[40]) = (long long)v15[40];
            *((long long *)&(&v10)[48]) = (long long)v15[48];
            *((long long *)&(&v10)[56]) = (long long)v15[56];
            *((long long *)&(&v10)[64]) = (long long)v15[64];
            *((long long *)&(&v10)[72]) = (long long)v15[72];
            *((long long *)&(&v10)[80]) = (long long)v15[80];
            *((long long *)&(&v10)[88]) = (long long)v15[88];
            *((long long *)&(&v10)[96]) = (long long)v15[96];
            *((long long *)&(&v10)[104]) = (long long)v15[104];
            *((long long *)&(&v10)[112]) = (long long)v15[112];
            *((long long *)&(&v10)[120]) = (long long)v15[120];
            *((long long *)&(&v10)[128]) = (long long)v15[128];
            *((long long *)&(&v10)[0x88]) = (long long)v15[0x88];
        }
        else
        {
            if (stat(a0, &v10))
                return 1;
        }
        if (((short)(int)v5[24] & 0xf000) != 0xa000)
        {
            v17 = v5;
            *((long long *)&v7) = *((long long *)v17);
            *((long long *)&(&v7)[8]) = (long long)v17[8];
            *((long long *)&(&v7)[16]) = (long long)v17[16];
            *((long long *)&(&v7)[24]) = (long long)v17[24];
            *((long long *)&(&v7)[32]) = (long long)v17[32];
            *((long long *)&(&v7)[40]) = (long long)v17[40];
            *((long long *)&(&v7)[48]) = (long long)v17[48];
            *((long long *)&(&v7)[56]) = (long long)v17[56];
            *((long long *)&(&v7)[64]) = (long long)v17[64];
            *((long long *)&(&v7)[72]) = (long long)v17[72];
            *((long long *)&(&v7)[80]) = (long long)v17[80];
            *((long long *)&(&v7)[88]) = (long long)v17[88];
            *((long long *)&(&v7)[96]) = (long long)v17[96];
            *((long long *)&(&v7)[104]) = (long long)v17[104];
            *((long long *)&(&v7)[112]) = (long long)v17[112];
            *((long long *)&(&v7)[120]) = (long long)v17[120];
            *((long long *)&(&v7)[128]) = (long long)v17[128];
            *((long long *)&(&v7)[0x88]) = (long long)v17[0x88];
        }
        else
        {
            if (fstatat(a2, a3, &v7, 0))
                return 1;
        }
        if (*((unsigned long long *)(&v10 + 8)) != *((unsigned long long *)(&v7 + 8)) || *((unsigned long long *)&v10) != *((unsigned long long *)&v7))
            return 1;
        if (*((char *)(a5 + 23)))
        {
            *((char *)a6) = ((int)v5[24] & 0xf000) != 0xa000;
            return 1;
        }
        return 0;
    }
    else if (v0 != 1)
    {
        if (*((char *)(a5 + 24)) == 1 || *((int *)(a5 + 4)) == 2 || ((short)(int)v4[24] & 0xf000) != 0xa000 || ((short)(int)v5[24] & 0xf000) == 0xa000)
            return 1;
        return 0;
        return 1;
    }
    else
    {
        return !(char)sub_419370(0xffffff9c, a0, a2, a3);
    }
}



// Function: writable_destination @ 0xa37a
unsigned int writable_destination(unsigned int a0, long long a1, unsigned int a2)
{
    unsigned int v1;  // eax

    if (((unsigned short)a2 & 0xf000) == 0xa000 || (char)sub_41b86c() || !faccessat(a0, a1, 2, 0x200))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: overwrite_ok @ 0xa3d5
typedef struct struct_0 {
    char padding_0[24];
    unsigned int field_18;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stderr;

int overwrite_ok(char *a0, long long a1, unsigned int a2, long long a3, struct_0 *a4)
{
    char *v3;  // rax
    char *v4;  // rax
    long long v5;  // rdi
    long long v6;  // rsi
    long long v7;  // rdx
    long long v8;  // rcx
    long long v9;  // r8
    long long v10;  // r9
    char v0;  // [bp-0x3c]
    char v1;  // [bp-0x32]

    if ((char)sub_40a37a(a2, a3, a4->field_18) != 1)
    {
        sub_411bad(a4->field_18, &v0, &v0);
        v1 = 0;
        sub_418a0f(4, a1);
        if (a0[24] || a0[21] || a0[22])
            v3 = gettext("%s: replace %s, overriding mode %04lo (%s)? ");
        else
            v3 = gettext("%s: unwritable %s (mode %04lo, %s); try anyway? ");
        fprintf(stderr, v3);
    }
    else
    {
        sub_418a0f(4, a1);
        v4 = gettext("%s: overwrite %s? ");
        fprintf(stderr, v4);
    }
    return sub_41c30f(v5, v6, v7, v8, v9, v10);
}



// Function: dest_info_init @ 0xa549
typedef struct struct_0 {
    char padding_0[72];
    unsigned long long field_48;
} struct_0;

unsigned long long dest_info_init(struct_0 *a0)
{
    a0->field_48 = sub_41563d(61, 0, sub_4167c7, sub_41674b, sub_416893);
    if (a0->field_48)
        return a0->field_48;
    sub_41bf79(); /* do not return */
}



// Function: src_info_init @ 0xa5a6
typedef struct struct_0 {
    char padding_0[80];
    unsigned long long field_50;
} struct_0;

unsigned long long src_info_init(struct_0 *a0)
{
    a0->field_50 = sub_41563d(61, 0, sub_41671d, sub_41674b, sub_416893);
    if (a0->field_50)
        return a0->field_50;
    sub_41bf79(); /* do not return */
}



// Function: abandon_move @ 0xa603
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[12];
    char field_18;
    char padding_19[36];
    char field_3d;
} struct_0;

unsigned int abandon_move(struct_0 *a0, long long a1, unsigned int a2, long long a3, struct_0 *a4)
{
    unsigned int v1;  // eax

    if (!a0->field_18)
        __assert_fail(); /* do not return */
    if (a0->field_8 == 2 || (a0->field_8 == 3 || a0->field_8 == 4 && a0->field_3d && (char)sub_40a37a(a2, a3, *((int *)&a4->field_18)) != 1) && (char)sub_40a3d5(a0, a1, a2, a3, a4) != 1)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: emit_verbose @ 0xa6db
int emit_verbose(long long a0, long long a1, long long a2)
{
    char *v1;  // rax

    v1 = sub_418941(1, 4, a1);
    printf("%s -> %s", sub_418941(0, 4, a0), v1);
    if (a2)
    {
        sub_418a0f(4, a2);
        printf(gettext(" (backup: %s)"));
    }
    return putchar_unlocked(10);
}



// Function: restore_default_fscreatecon_or_die @ 0xa788
void restore_default_fscreatecon_or_die(void)
{
    char *v1;  // rax
    int *err;  // rax

    if (!setfscreatecon(0))
        return;
    v1 = gettext("failed to restore the default file creation context");
    err = __errno_location();
    error(1, *(err), v1);
}



// Function: subst_suffix @ 0xa7d7
void* subst_suffix(void* a0, unsigned long a1, void* a2)
{
    unsigned long v0;  // [bp-0x20]
    unsigned long n;  // [bp-0x18]
    void* v2;  // [bp-0x10]

    v0 = a1 - a0;
    n = strlen(a2) + 1;
    v2 = sub_41b967(n + v0);
    memcpy(v0 + v2, a2, n);
    return memcpy(v2, a0, v0);
}



// Function: create_hard_link @ 0xa85d
unsigned int create_hard_link(unsigned long a0, unsigned int a1, unsigned long a2, unsigned long a3, unsigned int a4, long long a5, char a6, char a7, char a8)
{
    unsigned long long v0;  // [bp-0x78]
    char v1;  // [bp-0x54]
    unsigned long v2;  // [bp-0x30]
    unsigned int v3;  // [bp-0x24]
    unsigned long v4;  // [bp-0x20]

    v2 = a0;
    v1 = a6;
    v0 = 0xffffffffffffffff;
    v3 = sub_40e069(a1, a2, a4, a5);
    if (v3 > 0)
    {
        v4 = 0;
        if (!v2)
        {
            v4 = sub_40a7d7(a3, a5, a2);
            v2 = v4;
        }
        sub_418941(1, 4, v2);
        sub_418941(0, 4, a3);
        error(0, v3, gettext("cannot create hard link %s to %s"));
        free(v4);
        return 0;
    }
    else
    {
        if (v3 >= 0 || !a7)
            return 1;
        sub_418a0f(4, a3);
        printf(gettext("removed %s\n"));
        return 1;
    }
}



// Function: should_dereference @ 0xa9cc
typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
} struct_0;

unsigned int should_dereference(struct_0 *a0, char a1)
{
    unsigned int v1;  // eax

    if (a0->field_4 == 4 || a0->field_4 == 3 && a1)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: source_is_dst_backup @ 0xaa08
extern void* g_4275e8;

unsigned int source_is_dst_backup(void* a0, unsigned long long *a1, unsigned int a2, unsigned long a3)
{
    unsigned long len;  // rax
    unsigned int v10;  // eax
    unsigned int v0;  // [bp-0xe4]
    unsigned long len1;  // [bp-0xe0]
    void* v2;  // [bp-0xd8]
    unsigned long len2;  // [bp-0xd0]
    unsigned long v4;  // [bp-0xc8]
    unsigned long v5;  // [bp-0xc0]
    char v6;  // [bp-0xb8]
    unsigned long v7;  // [bp-0xb0]

    len1 = strlen(a0);
    v2 = sub_410369(a3);
    len2 = strlen(v2);
    v4 = strlen(g_4275e8);
    if (len1 == v4 + len2 && !memcmp(a0, v2, len2) && !strcmp(len2 + a0, g_4275e8))
    {
        len = strlen(a3);
        v5 = sub_40a7d7(a3, len + a3, g_4275e8);
        v0 = fstatat(a2, v5, &v6, 0);
        free(v5);
        if (!v0 && a1[1] == v7 && *(a1) == v6)
            v10 = 1;
        else
            v10 = 0;
        return v10 & 1;
    }
    return 0;
}



// Function: copy_internal @ 0xabda
typedef struct struct_1 {
    char field_0;
    char padding_1[23];
    unsigned int field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[4];
    unsigned int field_4;
    char padding_8[8];
    unsigned int field_10;
    char padding_14[5];
    char field_19;
    char padding_1a[22];
    char field_30;
    char field_31;
    char padding_32[6];
    char field_38;
    char padding_39[3];
    char field_3c;
    char padding_3d[2];
    char field_3f;
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

extern unsigned long long g_8;
extern unsigned long long g_10;
extern long long g_4271a0;
extern long long g_4271a8;

unsigned int copy_internal(unsigned long a0, unsigned long a1, unsigned int a2, unsigned long a3, int a4, void* a5, struct_1 *a6, struct_0 *idx, char *a8, char *a9, char *a10, char *a11)
{
    char v66;  // al
    unsigned int v76;  // eax
    unsigned long long v82;  // rax
    char *v84;  // rax
    char v67;  // al
    int *err;  // rax
    char *v86;  // rax
    int *err1;  // rax
    char v93;  // al
    int v94;  // eax
    char *v68;  // rax
    char *v98;  // rax
    int *err2;  // rax
    unsigned long long v102;  // rax
    char *v103;  // rax
    int *v104;  // rax
    int *v69;  // rax
    char *v105;  // rax
    int *v106;  // rax
    char *v107;  // rax
    int *v108;  // rax
    unsigned int v110;  // eax
    char v111;  // al
    char v114;  // al
    char *v115;  // rax
    int *v116;  // rax
    char *v117;  // rax
    int *v118;  // rax
    char *v120;  // rax
    int *v121;  // rax
    char *v123;  // rax
    int *v124;  // rax
    unsigned long long v125;  // rdx
    char *v126;  // rax
    int *v127;  // rax
    unsigned int v128;  // eax
    unsigned int v129;  // eax
    char *v130;  // rax
    int *v131;  // rax
    char *v132;  // rax
    int *v133;  // rax
    char *v134;  // rax
    char v72;  // al
    int *v135;  // rax
    char *v73;  // rax
    int *v74;  // rax
    int v0;  // [bp-0x1430]
    char v1;  // [bp-0x384]
    int v2;  // [bp-0x360]
    char v3;  // [bp-0x344]
    char v4;  // [bp-0x343]
    char v5;  // [bp-0x342]
    char v6;  // [bp-0x341]
    char v7;  // [bp-0x340]
    char v8;  // [bp-0x33f]
    char v9;  // [bp-0x33e]
    char v10;  // [bp-0x33d]
    char v11;  // [bp-0x33c]
    char v12;  // [bp-0x33b]
    char v13;  // [bp-0x33a]
    char v14;  // [bp-0x339]
    unsigned int v15;  // [bp-0x338]
    unsigned int v16;  // [bp-0x334]
    unsigned int v17;  // [bp-0x330]
    int v18;  // [bp-0x32c]
    unsigned int v19;  // [bp-0x328]
    unsigned int v20;  // [bp-0x324]
    unsigned int v21;  // [bp-0x320]
    unsigned int v22;  // [bp-0x31c]
    unsigned int v23;  // [bp-0x318]
    unsigned int v24;  // [bp-0x314]
    unsigned int v25;  // [bp-0x310]
    unsigned int v26;  // [bp-0x30c]
    unsigned int v27;  // [bp-0x308]
    unsigned int v28;  // [bp-0x304]
    unsigned int v29;  // [bp-0x300]
    unsigned int v30;  // [bp-0x2fc]
    unsigned long v31;  // [bp-0x2f8]
    unsigned long long v32;  // [bp-0x2f0]
    struct_1 *v33;  // [bp-0x2e8]
    unsigned long v34;  // [bp-0x2e0]
    unsigned long v35;  // [bp-0x2d8]
    unsigned long v36;  // [bp-0x2d0]
    char *v37;  // [bp-0x2c8]
    void* ptr;  // [bp-0x2c0]
    unsigned long v39;  // [bp-0x2b8]
    unsigned long n;  // [bp-0x2b0]
    unsigned long v41;  // [bp-0x2a8]
    unsigned long v42;  // [bp-0x2a0]
    void* v43;  // [bp-0x298]
    unsigned long v44;  // [bp-0x290]
    struct_1 **index;  // [bp-0x288]
    long long v46;  // [bp-0x280]
    char v47;  // [bp-0x278]
    unsigned long v48;  // [bp-0x270]
    unsigned long long v49;  // [bp-0x268]
    unsigned int v50;  // [bp-0x260]
    unsigned int v51;  // [bp-0x25c]
    unsigned int v52;  // [bp-0x258]
    long long v53;  // [bp-0x250]
    long long v54;  // [bp-0x248]
    char v55;  // [bp-0x1e8]
    unsigned int v56;  // [bp-0x1cc]
    unsigned int v57;  // [bp-0x1c8]
    unsigned long v58;  // [bp-0x1b8]
    stat v59;  // [bp-0x158]
    char v60;  // [bp-0xc8], Other Possible Types: unsigned long
    unsigned long long v61;  // [bp-0xc0]
    unsigned long long v62;  // [bp-0xb8]
    unsigned long long v63;  // [bp-0xb0]
    long long v64;  // [bp+0x0]

    v2 = a4;
    v1 = *((unsigned int *)&a8);
    v5 = 0;
    v31 = 0;
    v32 = 0;
    v7 = 0;
    v8 = 0;
    v9 = 0;
    v3 = 0 < v2;
    *(a10) = 0;
    v18 = (int)idx[1].padding_0;
    if (idx->padding_14[4])
    {
        if (v18 < 0)
            v18 = (!sub_418ece(0xffffff9c, a0, a2, a3, 1) ? 0 : *(__errno_location()));
        v3 = !v18;
        *(a11) = v3;
        v2 = *(a11);
    }
    switch (v18)
    {
    case 0:
        v66 = idx->field_3f ^ 1;
        goto LABEL_40ad8e;
    case 17:
        if (*((int *)&idx->padding_8[0]) != 2)
            goto LABEL_40ad7f;
        v67 = 0;
        break;
    default:
LABEL_40ad7f:
        v67 = 1;
        break;
    }
    v66 = v67 & 1;
LABEL_40ad8e:
    if (v66)
    {
        v34 = (!v18 ? a1 : a0);
        v20 = (!v18 ? a2 : 0xffffff9c);
        v35 = (!v18 ? a3 : a0);
        v21 = (idx->field_4 == 2 ? 0x100 : 0);
        if ((int)sub_406cbb(v20, v35, &v47, v21))
        {
            sub_418a0f(4, v34);
            v68 = gettext("cannot stat %s");
            v69 = __errno_location();
            error(0, *(v69), v68);
            return 0;
        }
        v15 = v50;
        if (((unsigned short)v15 & 0xf000) == 0x4000 && idx->field_38 != 1)
        {
            sub_418a0f(4, a0);
            error(0, 0, (!(idx->field_19 ^ 1) ? gettext("omitting directory %s") : gettext("-r not specified; omitting directory %s")));
            return 0;
        }
    }
    if (v1 && *((long long *)&idx[1].field_10))
    {
        if (((unsigned short)v15 & 0xf000) != 0x4000 && !idx->padding_0 && (char)sub_411a7b(*((long long *)&idx[1].field_10), a0, &v47, a0))
        {
            sub_418a0f(4, a0);
            error(0, 0, gettext("warning: source file %s specified more than once"));
            return 1;
        }
        sub_4119db(*((long long *)&idx[1].field_10), a0, &v47, a0);
    }
    /* unsupported instruction */ = (int)(&v0 - 4224);
    v11 = sub_40a9cc(idx, v1);
    if (v2 <= 0)
    {
        if (v18 != 0x11 || *((int *)&idx->padding_8[0]) != 2)
        {
            if (((unsigned short)v15 & 0xf000) != 0x8000 && (idx->padding_14[0] != 1 || ((unsigned short)v15 & 0xf000) == 0x4000 || ((unsigned short)v15 & 0xf000) == 0xa000) || idx->padding_14[4] || idx->padding_39[1] || idx->padding_14[3] || idx->padding_0 || idx->padding_14[1])
                v72 = 1;
            else
                v72 = 0;
            v12 = v72;
            v12 &= 1;
            v22 = (!v12 ? 0 : 0x100);
            if (v12 != 1 && v2 < 0)
            {
                v3 = 1;
            }
            else
            {
                /* unsupported instruction */ = (int)(&v0 - 4224);
                if (!(int)sub_406cbb(a2, a3, &v55, v22))
                {
                    v9 = v12;
                    v18 = 0x11;
                }
                else
                {
                    /* unsupported instruction */ = (int)(&v0 - 4224);
                    if (*(__errno_location()) != 40 || !idx->padding_14[2])
                    {
                        /* unsupported instruction */ = (int)(&v0 - 4224);
                        if (*(__errno_location()) != 2)
                        {
                            sub_418a0f(4, a1);
                            v73 = gettext("cannot stat %s");
                            v74 = __errno_location();
                            error(0, *(v74), v73);
                            return 0;
                        }
                        v3 = 1;
                    }
                }
            }
        }
        if (v18 != 0x11)
            goto LABEL_40bbf1;
        v4 = 0;
        if (*((int *)&idx->padding_8[0]) != 2)
        {
            *((char **)&(&v0)[4216]) = &v4;
            /* unsupported instruction */ = (int)(&v0 - 4232);
            if ((char)sub_409b18(a0, &v47, a2, a3, &v55, idx, *((unsigned long long *)(&v0 + 4216))) != 1)
            {
                sub_418941(1, 4, a1);
                sub_418941(0, 4, a0);
                error(0, 0, gettext("%s and %s are the same file"));
                return 0;
            }
        }
        if (idx->padding_39[2] && ((unsigned short)v15 & 0xf000) != 0x4000)
        {
            if (idx->padding_1a[5] && (idx->padding_14[4] != 1 || *((unsigned long long *)&v55) != v47))
                v76 = 1;
            else
                v76 = 0;
            v23 = v76;
            /* unsupported instruction */ = (int)(&v0 - 4224);
            if (sub_41a52e(a2, a3, &v55, &v47, v23) < 0)
                goto LABEL_40b410;
            if (a11)
                *(a11) = 1;
            v31 = sub_40de9c(a3, v48, v47, v48);
            if (!v31 || !(*((unsigned long long *)&(&v0)[4216]) = (unsigned long long)v11, *((unsigned long long *)&(&v0)[4208]) = (unsigned long long)idx->field_3c, *((unsigned long long *)&(&v0)[4200]) = 1, (char)(unsigned long long)sub_40a85d(0, a2, v31, a1, a2, a3, *((char *)((void*)&v0 + 4200)), *((char *)((void*)&v0 + 4208)), *((char *)((void*)&v0 + 4216))) != 1))
                return 1;
        }
        else
        {
LABEL_40b410:
            if (idx->padding_14[4])
            {
                /* unsupported instruction */ = (int)(&v0 - 4224);
                if ((char)sub_40a603(idx, a1, a2, a3, &v55))
                {
                    if (!a11)
                        return 1;
                    *(a11) = 1;
                    return 1;
                }
            }
            else if (((unsigned short)v15 & 0xf000) != 0x4000 && (*((int *)&idx->padding_8[0]) == 2 || *((int *)&idx->padding_8[0]) == 3 && !(/* unsupported instruction */ = (int)(&v0 - 4224), (char)(long long)sub_40a3d5(idx, a1, a2, a3, &v55) == 1)))
            {
                return 1;
            }
            if (v4)
                return 1;
            if (((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) != 0x4000)
            {
                if (((unsigned short)v15 & 0xf000) == 0x4000 && (!idx->padding_14[4] || !idx->padding_0))
                {
                    sub_418941(1, 4, a0);
                    sub_418941(0, 4, a1);
                    error(0, 0, gettext("cannot overwrite non-directory %s with directory %s"));
                    return 0;
                }
                if (v1 && idx->padding_0 != 3)
                {
                    /* unsupported instruction */ = (int)(&v0 - 4224);
                    if ((char)sub_411a7b(idx[1].padding_8, a3, &v55, a3))
                    {
                        sub_418941(1, 4, a0);
                        sub_418941(0, 4, a1);
                        error(0, 0, gettext("will not overwrite just-created %s with %s"));
                        return 0;
                    }
                }
            }
            if (((unsigned short)v15 & 0xf000) != 0x4000 && ((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) == 0x4000 && (!idx->padding_14[4] || !idx->padding_0))
            {
                sub_418a0f(4, a1);
                error(0, 0, gettext("cannot overwrite directory %s with non-directory"));
                return 0;
            }
            if (idx->padding_14[4] && ((unsigned short)v50 & 0xf000) == 0x4000 && ((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) != 0x4000 && !idx->padding_0)
            {
                sub_418ba1(0, 3, a1);
                sub_418ba1(0, 3, a0);
                error(0, 0, gettext("cannot move directory onto non-directory: %s -> %s"));
                return 0;
            }
            if (idx->padding_0 && !(v36 = (unsigned long)(unsigned long long)sub_410369(a0), /* unsupported instruction */ = (int)(&v0 - 4224), (char)(unsigned long long)sub_406aa4(v36) == 1 || !idx->padding_14[4] && ((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) == 0x4000))
            {
                if (idx->padding_0 != 3 && (char)sub_40aa08(v36, &v47, a2, a3))
                {
                    v37 = (!idx->padding_14[4] ? gettext("backing up %s might destroy source;  %s not copied") : gettext("backing up %s might destroy source;  %s not moved"));
                    sub_418941(1, 4, a0);
                    sub_418941(0, 4, a1);
                    error(0, 0, v37);
                    return 0;
                }
                ptr = sub_41021f(a2, a3, (unsigned long long)idx->padding_0, a3);
                if (ptr)
                {
                    v39 = a3 - a1;
                    n = strlen(ptr) + 1;
                    v82 = (unsigned long long)((15 + n + v39 + 8) / 16) * 16;
                    while (&v0 - 0x88 != &v0 - 4232 - (v82 & 0xfffffffffffff000))
                    ;
                    /* unsupported instruction */ = (int)(&v0 - 0x88 - ((unsigned int)v82 & 0xfff));
                    if ((unsigned short)v82 & 0xfff)
                        *((long long *)(((unsigned int)v82 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v82 & 0xfff) - 8 + /* unsupported instruction */));
                    v32 = (/* unsupported instruction */ + 15 >> 4) * 16;
                    memcpy(mempcpy(v32, a1, v39), ptr, n);
                    /* unsupported instruction */ = /* unsupported instruction */ - 24;
                    free(ptr);
                }
                else
                {
                    /* unsupported instruction */ = (int)(&v0 - 4224);
                    if (*(__errno_location()) != 2)
                    {
                        sub_418a0f(4, a1);
                        v84 = gettext("cannot backup %s");
                        err = __errno_location();
                        error(0, *(err), v84);
                        return 0;
                    }
                }
                v3 = 1;
                goto LABEL_40bbf1;
            }
            else
            {
                if (((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) != 0x4000 && idx->padding_14[4] != 1 && (idx->padding_14[1] || idx->field_31 && (idx->field_30 && *((unsigned long long *)((void*)&v55 + 16)) > 1 || idx->field_4 == 2 && ((unsigned short)v50 & 0xf000) != 0x8000)))
                {
                    /* unsupported instruction */ = (int)(&v0 - 4224);
                    if (unlinkat(a2, a3, 0, a3))
                    {
                        /* unsupported instruction */ = (int)(&v0 - 4224);
                        if (*(__errno_location()) != 2)
                        {
                            sub_418a0f(4, a1);
                            v86 = gettext("cannot remove %s");
                            err1 = __errno_location();
                            error(0, *(err1), v86);
                            return 0;
                        }
                    }
                    v3 = 1;
                    if (idx->field_3c)
                    {
                        sub_418a0f(4, a1);
                        /* unsupported instruction */ = (int)(&v0 - 4224);
                        printf(gettext("removed %s\n"));
                    }
                }
            }
            goto LABEL_40bbf1;
        }
    }
    else
    {
LABEL_40bbf1:
        if (v1 && idx[1].padding_8 && idx->padding_14[4] != 1 && !idx->padding_0)
        {
            v10 = 1;
            if (v9)
            {
                v33 = &v55;
            }
            else
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if (!fstatat(a2, a3, &v60, 0x100))
                    v33 = &v60;
                else
                    v10 = 0;
            }
            if (v10 && ((unsigned short)v33->field_18 & 0xf000) == 0xa000)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if ((char)sub_411a7b(idx[1].padding_8, a3, v33, a3))
                {
                    sub_418941(1, 4, a1);
                    sub_418941(0, 4, a0);
                    error(0, 0, gettext("will not copy %s through just-created symlink %s"));
                    return 0;
                }
            }
        }
        if (idx->field_3c && idx->padding_14[4] != 1 && ((unsigned short)v15 & 0xf000) != 0x4000)
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            sub_40a6db(a0, a1, v32);
        }
        if (!v18)
        {
            v31 = 0;
        }
        else if (idx->field_38 && ((unsigned short)v15 & 0xf000) == 0x4000)
        {
            if (v1)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v31 = sub_40de9c(a3, v48, v47, v48);
            }
            else
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v31 = sub_40de23(v48, v47, v47);
            }
        }
        else
        {
            if (idx->padding_14[4] && v49 == 1)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v31 = sub_40de23(v48, v47, v47);
            }
            else if (idx->field_30 && idx->padding_14[3] != 1 && (v49 > 1 || v1 && idx->field_4 == 3 || idx->field_4 == 4))
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                v31 = sub_40de9c(a3, v48, v47, v48);
            }
        }
        if (!v31)
        {
LABEL_40c172:
            if (idx->padding_14[4])
            {
                if (v18 == 0x11)
                {
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    if (renameat(0xffffff9c, a0, a2, a3))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v94 = *(__errno_location());
                    }
                    else
                    {
                        v94 = 0;
                    }
                    v18 = v94;
                }
                switch (v18)
                {
                case 0:
                    if (idx->field_3c)
                    {
                        printf(gettext("renamed "));
                        sub_40a6db(a0, a1, v32);
                    }
                    if (*((long long *)&idx->padding_1a[14]))
                        sub_408750(a1, 1, idx);
                    if (a11)
                        *(a11) = 1;
                    if (!v1 || idx->field_3f == 1)
                        return 1;
                    sub_4119db(idx[1].padding_8, a3, &v47, a3);
                    return 1;
                case 22:
                    sub_418941(1, 4, g_4271a8);
                    sub_418941(0, 4, g_4271a0);
                    error(0, 0, gettext("cannot move %s to a subdirectory of itself, %s"));
                    *(a10) = 1;
                    return 1;
                case 18:
                    if (((unsigned short)v15 & 0xf000) == 0x4000)
                        break;
                    else
                        break;
                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                    if (unlinkat(a2, a3))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        if (*(__errno_location()) != 2)
                        {
                            sub_418941(1, 4, a1);
                            sub_418941(0, 4, a0);
                            v98 = gettext("inter-device move failed: %s to %s; unable to remove target");
                            err2 = __errno_location();
                            error(0, *(err2), v98);
                            sub_40dda4(v48, v47, v47);
                            return 0;
                        }
                    }
                    if (idx->field_3c && ((unsigned short)v15 & 0xf000) != 0x4000)
                    {
                        printf(gettext("copied "));
                        /* unsupported instruction */ = /* unsupported instruction */ - 24;
                        sub_40a6db(a0, a1, v32);
                    }
                    v3 = 1;
                    goto LABEL_40c4fe;
                default:
                    sub_418941(1, 4, a1);
                    sub_418941(0, 4, a0);
                    error(0, v18, gettext("cannot move %s to %s"));
                    sub_40dda4(v48, v47, v47);
                    return 0;
                }
            }
LABEL_40c4fe:
            v24 = (!idx->padding_39[0] ? v15 & 0xfff : idx->field_10 & 0xfff);
            v17 = (!(idx->padding_1a[3] ^ 1) ? 63 : ((v15 & 0xf000) == 0x4000 ? 18 : 0)) & v24;
            v6 = 1;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            if ((char)sub_4084c6(a0, a1, v15, v3, idx) != 1)
                return 0;
            if (((unsigned short)v15 & 0xf000) == 0x4000)
            {
                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                if ((char)sub_407a8e(&v47, a6))
                {
                    sub_418a0f(4, a0);
                    error(0, 0, gettext("cannot copy cyclic symbolic link %s"));
                }
                else
                {
                    v102 = (unsigned long long)(47 / 16) * 16;
                    for (/* unsupported instruction */ = /* unsupported instruction */; /* unsupported instruction */ != /* unsupported instruction */ - (v102 & 0xfffffffffffff000); *((long long *)(/* unsupported instruction */ + 0xff8)) = *((long long *)(/* unsupported instruction */ + 0xff8)))
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 0x1000;
                    }
                    /* unsupported instruction */ = /* unsupported instruction */ - ((unsigned int)v102 & 0xfff);
                    if ((unsigned short)v102 & 0xfff)
                        *((long long *)(((unsigned int)v102 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v102 & 0xfff) - 8 + /* unsupported instruction */));
                    index = (/* unsupported instruction */ + 15 >> 4) * 16;
                    *(index) = a6;
                    index[1] = v48;
                    index[2] = v47;
                    if (v3 || ((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) != 0x4000)
                    {
                        v28 = ~(v17) & v24;
                        if (mkdirat(a2, a3, v28, a3))
                        {
                            sub_418a0f(4, a1);
                            v103 = gettext("cannot create directory %s");
                            v104 = __errno_location();
                            error(0, *(v104), v103);
                        }
                        else
                        {
                            /* unsupported instruction */ = /* unsupported instruction */ - 16;
                            if (fstatat(a2, a3, &v55, 0x100))
                            {
                                sub_418a0f(4, a1);
                                v105 = gettext("cannot stat %s");
                                v106 = __errno_location();
                                error(0, *(v106), v105);
                            }
                            else
                            {
                                if (((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 448) != 448 && !(v16 = *((unsigned int *)((void*)&v55 + 24)), v5 = 1, /* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), !(unsigned int)(unsigned long long)sub_411260((unsigned long long)a2, a3, (unsigned long long)(v16 | 448), a3)))
                                {
                                    sub_418a0f(4, a1);
                                    v107 = gettext("setting permissions for %s");
                                    v108 = __errno_location();
                                    error(0, *(v108), v107);
                                }
                                else
                                {
                                    if (*(a9) != 1)
                                    {
                                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                        sub_40de9c(a3, *((unsigned long long *)((void*)&v55 + 8)), *((unsigned long long *)&v55), *((unsigned long long *)((void*)&v55 + 8)));
                                        *(a9) = 1;
                                    }
                                    if (idx->field_3c)
                                    {
                                        if (idx->padding_14[4])
                                        {
                                            sub_418a0f(4, a1);
                                            /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                            printf(gettext("created directory %s\n"));
                                        }
                                        else
                                        {
                                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                            sub_40a6db(a0, a1, 0);
                                        }
                                    }
                                    goto LABEL_40c9b4;
                                }
                            }
                        }
                    }
                    else
                    {
                        v17 = 0;
                        if (!*((long long *)&idx->padding_1a[14]) && !idx->padding_32[1] || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (char)(unsigned long long)sub_408750(a1, 0, idx) == 1 || !idx->padding_32[2]))
                        {
LABEL_40c9b4:
                            if (!idx->padding_1a[2] || !a5 || *((long long *)a5) == v47)
                            {
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                *((char **)(/* unsupported instruction */ - 8)) = a10;
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                *((char **)(/* unsupported instruction */ - 8)) = a9;
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                *((struct_0 **)(/* unsupported instruction */ - 8)) = idx;
                                *((struct_1 ***)(/* unsupported instruction */ - 8)) = index;
                                /* unsupported instruction */ = /* unsupported instruction */ + 16;
                                v6 = sub_407ee6(a0, a1, a2, a3, v3, &v47, v64, a6, idx, a8);
                                goto LABEL_40d20a;
                            }
                        }
                    }
                }
            }
            else
            {
                if (idx->padding_39[1])
                {
                    v8 = 1;
                    if (*((char *)a0) != 47)
                    {
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v44 = sub_411559(a3);
                        if (a2 == 0xffffff9c && !(/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (int)(long long)strcmp(".", v44)) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), v110 = (unsigned int)(int)(long long)stat(".", &v59), (int)(long long)stat(".", &v59) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (unsigned int)(unsigned long long)fstatat((unsigned long long)a2, v44, &v60, 0) || v59.st_ino == *((unsigned long long *)((void*)&v60 + 8)) && *((unsigned long long *)(void*)&v59) == *((unsigned long long *)(void*)&v60))))
                            v111 = 1;
                        else
                            v111 = 0;
                        v14 = v111;
                        v14 &= 1;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        free(v44);
                        if (!(v14 ^ 1))
                            goto LABEL_40cb97;
                        sub_418ba1(0, 3, a1);
                        error(0, 0, gettext("%s: can make relative symbolic links only in current directory"));
                    }
                    else
                    {
LABEL_40cb97:
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        v27 = sub_40e27f(a0, a2, a3, idx->padding_14[2], 0xffffffff);
                        if (v27 <= 0)
                            goto LABEL_40d20a;
                        sub_418941(1, 4, a0);
                        sub_418941(0, 4, a1);
                        error(0, v27, gettext("cannot create symbolic link %s to %s"));
                    }
                }
                else
                {
                    if (idx->padding_14[3])
                    {
                        if (idx->padding_14[2] || *((int *)&idx->padding_8[0]) == 3)
                            v114 = 1;
                        else
                            v114 = 0;
                        v13 = v114;
                        v13 &= 1;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((unsigned long long *)(/* unsupported instruction */ - 8)) = v11;
                        /* unsupported instruction */ = /* unsupported instruction */ - 8;
                        *((unsigned long *)(/* unsupported instruction */ - 8)) = 0;
                        *((unsigned long long *)(/* unsupported instruction */ - 8)) = v13;
                        /* unsupported instruction */ = /* unsupported instruction */ + 16;
                        if (!((char)sub_40a85d(a0, 0xffffff9c, a0, a1, a2, a3, v64, *((char *)&a6), *((char *)&idx)) ^ 1))
                            goto LABEL_40d20a;
                    }
                    else
                    {
                        if (((unsigned short)v15 & 0xf000) == 0x8000 || idx->padding_14[0] && ((unsigned short)v15 & 0xf000) != 0xa000)
                        {
                            v7 = 1;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((char **)(/* unsupported instruction */ - 8)) = &v47;
                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                            *((char **)(/* unsupported instruction */ - 8)) = &v3;
                            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v17;
                            /* unsupported instruction */ = /* unsupported instruction */ + 16;
                            if (!((char)sub_40896f(a0, a1, a2, a3, idx, v24 & 0x1ff, v64, a6, idx) ^ 1))
                                goto LABEL_40d20a;
                        }
                        else
                        {
                            if (((unsigned short)v15 & 0xf000) == 0x1000)
                            {
                                v26 = ~(v17) & v15;
                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                if (!mknodat(a2, a3, v26, 0) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), !(unsigned int)(unsigned long long)mkfifoat((unsigned long long)a2, a3, _INSERT((unsigned long long)v26, 1, (char)v26 & 239) & 0xffffffff, a3)))
                                    goto LABEL_40d20a;
                                sub_418a0f(4, a1);
                                v115 = gettext("cannot create fifo %s");
                                v116 = __errno_location();
                                error(0, *(v116), v115);
                            }
                            else
                            {
                                if (((unsigned short)v15 & 0xf000) == 0x6000 || ((unsigned short)v15 & 0xf000) == 0x2000 || ((unsigned short)v15 & 0xf000) == 0xc000)
                                {
                                    v25 = ~(v17) & v15;
                                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                    if (!mknodat(a2, a3, v25, v53))
                                        goto LABEL_40d20a;
                                    sub_418a0f(4, a1);
                                    v123 = gettext("cannot create special file %s");
                                    v124 = __errno_location();
                                    error(0, *(v124), v123);
                                }
                                else
                                {
                                    if (((unsigned short)v15 & 0xf000) == 0xa000)
                                    {
                                        v42 = sub_40eded(a0, v54, v54);
                                        v8 = 1;
                                        if (!v42)
                                        {
                                            sub_418a0f(4, a0);
                                            v117 = gettext("cannot read symbolic link %s");
                                            v118 = __errno_location();
                                            error(0, *(v118), v117);
                                        }
                                        else
                                        {
                                            /* unsupported instruction */ = /* unsupported instruction */ - 16;
                                            v19 = sub_40e27f(v42, a2, a3, idx->padding_14[2], 0xffffffff);
                                            if (v19 > 0 && idx->padding_39[2] && v3 != 1 && ((unsigned short)*((unsigned int *)((void*)&v55 + 24)) & 0xf000) == 0xa000)
                                            {
                                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                                if (v58 == strlen(v42))
                                                {
                                                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                                    v43 = sub_40f0b6(a2, a3, v58, a3);
                                                    if (v43)
                                                    {
                                                        if (!strcmp(v43, v42))
                                                            v19 = 0;
                                                        /* unsupported instruction */ = /* unsupported instruction */ - 16;
                                                        free(v43);
                                                    }
                                                }
                                            }
                                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                            free(v42);
                                            if (v19 > 0)
                                            {
                                                sub_418a0f(4, a1);
                                                error(0, v19, gettext("cannot create symbolic link %s"));
                                            }
                                            else
                                            {
                                                if (idx->padding_32[1])
                                                {
                                                    /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                                    sub_40a788();
                                                }
                                                if (idx->padding_1a[3] && !(/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), !(unsigned int)(unsigned long long)sub_4112c3((unsigned long long)a2, a3, (unsigned long long)v51, (unsigned long long)v52) || (/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (char)(unsigned long long)sub_40dc1f(idx) == 1)))
                                                {
                                                    v120 = gettext("failed to preserve ownership for %s");
                                                    v121 = __errno_location();
                                                    /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                                    error(0, *(v121), v120);
                                                    if (!idx->padding_32[0])
                                                        goto LABEL_40d20a;
                                                }
                                                else
                                                {
LABEL_40d20a:
                                                    if (v3 == 1 || idx->padding_14[0] == 1 || ((unsigned short)v15 & 0xf000) == 0x4000 || !*((long long *)&idx->padding_1a[14]) && !idx->padding_32[1] || !(/* unsupported instruction */ = (int)(/* unsupported instruction */ - 8), (char)(unsigned long long)sub_408750(a1, 0, idx) != 1 && idx->padding_32[2]))
                                                    {
                                                        if (v1 && idx[1].padding_8)
                                                        {
                                                            /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                                            if (!fstatat(a2, a3, &v60, 0x100))
                                                            {
                                                                /* unsupported instruction */ = /* unsupported instruction */ - 8;
                                                                sub_4119db(idx[1].padding_8, a3, &v60, a3);
                                                            }
                                                        }
                                                        if (idx->padding_14[3] && ((unsigned short)v15 & 0xf000) != 0x4000)
                                                            return v6;
                                                        if (v7)
                                                            return v6;
                                                        if (idx->padding_1a[5])
                                                        {
                                                            v60 = sub_419e57(&v47);
                                                            v61 = v125;
                                                            v62 = sub_419e8b(&v47);
                                                            v63 = v125;
                                                            v29 = (!v8 ? 0 : 0x100);
                                                            /* unsupported instruction */ = /* unsupported instruction */ - 24;
                                                            if (utimensat(a2, a3, &v60, v29))
                                                            {
                                                                sub_418a0f(4, a1);
                                                                v126 = gettext("preserving times for %s");
                                                                v127 = __errno_location();
                                                                /* unsupported instruction */ = /* unsupported instruction */ - 32;
                                                                error(0, *(v127), v126);
                                                                if (idx->padding_32[0])
                                                                    return 0;
                                                            }
                                                        }
                                                        if (v8 != 1 && idx->padding_1a[3] && (v3 || v51 != v56 || v52 != v57))
                                                        {
                                                            *((char **)(/* unsupported instruction */ - 8)) = &v55;
                                                            *((unsigned long long *)(/* unsupported instruction */ - 16)) = v3;
                                                            v128 = sub_40820f(idx, a1, a2, a3, 0xffffffff, &v47, v64, a6);
                                                            if (v128 == 0xffffffff)
                                                                return 0;
                                                            if (!v128)
                                                                v15 &= 0xfffff1ff;
                                                        }
                                                        if (idx->padding_32[3] && sub_407d7e(a0, 0xffffffff, a1, 0xffffffff, idx) != 1 && idx->padding_32[4])
                                                            return 0;
                                                        if (v8)
                                                            return v6;
                                                        sub_4084b0(a1, 0xffffffff, &v47);
                                                        if (idx->padding_1a[4] || idx->padding_14[4])
                                                        {
                                                            if (sub_40ec57(a0, 0xffffffff, a1, 0xffffffff, v15) && idx->padding_32[0])
                                                                return 0;
                                                        }
                                                        else
                                                        {
                                                            if (idx->padding_39[0])
                                                            {
                                                                if (sub_40ed24(a1, 0xffffffff, idx->field_10))
                                                                    return 0;
                                                            }
                                                            else
                                                            {
                                                                if (idx->padding_1a[6] && v3)
                                                                {
                                                                    if (((unsigned short)v15 & 0xf000) == 0x4000 || ((unsigned short)v15 & 0xf000) == 0xc000)
                                                                        v129 = 0x1ff;
                                                                    else
                                                                        v129 = 438;
                                                                    v30 = v129;
                                                                    if (sub_40ed24(a1, 0xffffffff, ~((unsigned int)sub_40dcaf()) & v30))
                                                                        return 0;
                                                                }
                                                                else
                                                                {
                                                                    if (v17)
                                                                    {
                                                                        v17 &= ~((unsigned int)sub_40dcaf());
                                                                        if (v17 && v5 != 1)
                                                                        {
                                                                            if (v3 && fstatat(a2, a3, &v55, 0x100))
                                                                            {
                                                                                sub_418a0f(4, a1);
                                                                                v130 = gettext("cannot stat %s");
                                                                                v131 = __errno_location();
                                                                                error(0, *(v131), v130);
                                                                                return 0;
                                                                            }
                                                                            v16 = *((unsigned int *)((void*)&v55 + 24));
                                                                            if (~(v16) & v17)
                                                                                v5 = 1;
                                                                        }
                                                                    }
                                                                    if (v5 && sub_411260(a2, a3, v16 | v17, a3))
                                                                    {
                                                                        sub_418a0f(4, a1);
                                                                        v132 = gettext("preserving permissions for %s");
                                                                        v133 = __errno_location();
                                                                        error(0, *(v133), v132);
                                                                        if (idx->padding_32[0])
                                                                            return 0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        return v6;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    else
                                    {
                                        sub_418a0f(4, a0);
                                        error(0, 0, gettext("%s has unknown file type"));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        else if (((unsigned short)v15 & 0xf000) != 0x4000)
        {
            v93 = idx->field_3c;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            /* unsupported instruction */ = /* unsupported instruction */ - 8;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v11;
            *((unsigned long long *)(/* unsupported instruction */ - 8)) = v93;
            *((unsigned long *)(/* unsupported instruction */ - 16)) = 1;
            if ((char)sub_40a85d(0, a2, v31, a1, a2, a3, v64, *((char *)&a6), *((char *)&idx)) == 1)
                return 1;
        }
        else if ((char)sub_419370(0xffffff9c, a0, a2, v31))
        {
            sub_418941(1, 4, g_4271a8);
            sub_418941(0, 4, g_4271a0);
            error(0, 0, gettext("cannot copy a directory, %s, into itself, %s"));
            *(a10) = 1;
        }
        else
        {
            /* unsupported instruction */ = /* unsupported instruction */ - 16;
            if ((char)sub_419370(a2, a3, a2, v31))
            {
                sub_418a0f(4, g_4271a0);
                error(0, 0, gettext("warning: source directory %s specified more than once"));
                if (!idx->padding_14[4] || !a11)
                    return 1;
                *(a11) = 1;
                return 1;
            }
            if (idx->field_4 == 4 || v1 && idx->field_4 == 3)
                goto LABEL_40c172;
            v41 = sub_40a7d7(a1, a3, v31);
            sub_418941(1, 4, v41);
            sub_418941(0, 4, a1);
            error(0, 0, gettext("will not create hard link %s to directory %s"));
            free(v41);
        }
    }
    if (idx->padding_32[1])
        sub_40a788();
    if (!v31)
        sub_40dda4(v48, v47, v47);
    if (v32)
    {
        v46 = v32 + a3 - a1;
        if (renameat(a2, v46, a2, a3))
        {
            sub_418a0f(4, a1);
            v134 = gettext("cannot un-backup %s");
            v135 = __errno_location();
            error(0, *(v135), v134);
        }
        else if (idx->field_3c)
        {
            sub_418941(1, 4, a1);
            sub_418941(0, 4, v32);
            printf(gettext("%s -> %s (unbackup)\n"));
        }
    }
    return 0;
}



// Function: valid_options @ 0xd983
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

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned int valid_options(struct_0 *a0)
{
    if (a0->field_0 > 3)
        __assert_fail(); /* do not return */
    if (a0->field_c != 1 && a0->field_c != 2 && a0->field_c != 3)
        __assert_fail(); /* do not return */
    if (a0->field_44 && a0->field_44 != 1 && a0->field_44 != 2)
        __assert_fail(); /* do not return */
    if (a0->field_17 == 1 && a0->field_3a == 1)
        __assert_fail(); /* do not return */
    if (a0->field_44 != 2)
    {
        return 1;
    }
    else if (a0->field_c != 2)
    {
        __assert_fail(); /* do not return */
    }
    else
    {
        return 1;
    }
}



// Function: copy @ 0xdaea
extern unsigned long long g_4271a0;
extern unsigned long long g_4271a8;

unsigned int copy(unsigned long a0, unsigned long a1, unsigned int a2, unsigned int a3, int a4, void* a5, char *a6, char *a7)
{
    char v0;  // [bp-0x11]

    if ((char)sub_40d983(a5))
    {
        g_4271a0 = a0;
        g_4271a8 = a1;
        v0 = 0;
        return sub_40abda(a0, a1, a2, a3, a4, NULL, NULL, a5, 0x1, &v0, a6, a7);
    }
    __assert_fail(); /* do not return */
}



// Function: cp_options_default @ 0xdbcb
typedef struct struct_0 {
    char padding_0[26];
    char field_1a;
    char field_1b;
    char padding_1c[36];
    unsigned int field_40;
} struct_0;

struct_0 * cp_options_default(struct_0 *ptr)
{
    memset(ptr, 0, 88);
    ptr->field_1b = !geteuid();
    ptr->field_1a = ptr->field_1b;
    ptr->field_40 = 0xffffffff;
    return ptr;
}



// Function: chown_failure_ok @ 0xdc1f
typedef struct struct_0 {
    char padding_0[26];
    char field_1a;
} struct_0;

extern unsigned long long g_10;

unsigned int chown_failure_ok(struct_0 *a0)
{
    unsigned int v1;  // eax

    if ((*(__errno_location()) == 1 || *(__errno_location()) == 22) && a0->field_1a != 1)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: owner_failure_ok @ 0xdc67
typedef struct struct_0 {
    char padding_0[27];
    char field_1b;
} struct_0;

extern unsigned long long g_10;

unsigned int owner_failure_ok(struct_0 *a0)
{
    unsigned int v1;  // eax

    if ((*(__errno_location()) == 1 || *(__errno_location()) == 22) && a0->field_1b != 1)
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: cached_umask @ 0xdcaf
extern unsigned int g_427018;

unsigned int cached_umask(void)
{
    if (g_427018 == 0xffffffff)
    {
        g_427018 = umask(0);
        umask(g_427018);
    }
    return g_427018;
}



// Function: src_to_dest_hash @ 0xdce7
long long src_to_dest_hash(unsigned long long *a0, unsigned long long a1)
{
    return *(a0) % a1;
}



// Function: src_to_dest_compare @ 0xdd14
extern unsigned long long g_8;

unsigned int src_to_dest_compare(unsigned long long *a0, unsigned long a1)
{
    unsigned int v1;  // eax

    if (*(a0) == *((long long *)a1) && a0[1] == *((long long *)(a1 + 8)))
        v1 = 1;
    else
        v1 = 0;
    return v1 & 1;
}



// Function: src_to_dest_free @ 0xdd6d
typedef struct struct_0 {
    char padding_0[16];
    void* field_10;
} struct_0;

extern unsigned long long g_10;

void src_to_dest_free(struct_0 *a0)
{
    free(a0->field_10);
    free(a0);
    return;
}



// Function: forget_created @ 0xdda4
typedef struct struct_0 {
    char padding_0[16];
    void* field_10;
} struct_0;

extern long long g_4275e0;

void forget_created(unsigned long a0, unsigned long long a1)
{
    struct_0 *v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]
    unsigned long long v3;  // [bp-0x18]

    v1 = a0;
    v2 = a1;
    v3 = 0;
    v0 = sub_4163ee(g_4275e0, &v1, &v1);
    if (!v0)
        return;
    sub_40dd6d(v0);
    return;
}



// Function: src_to_dest_lookup @ 0xde23
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

extern unsigned long long g_10;
extern long long g_4275e0;

unsigned long long src_to_dest_lookup(unsigned long a0, unsigned long long a1)
{
    struct_0 *v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]
    unsigned long long v2;  // [bp-0x20]

    v1 = a0;
    v2 = a1;
    v0 = sub_414f88(g_4275e0, &v1, &v1);
    return (!v0 ? 0 : v0->field_10);
}



// Function: remember_copied @ 0xde9c
extern unsigned long long g_8;
extern unsigned long long g_10;
extern long long g_4275e0;

unsigned long long remember_copied(long long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long *idx;  // [bp-0x18]
    unsigned long long *v1;  // [bp-0x10]

    idx = sub_41b941(24);
    idx[2] = sub_41bf48(a0);
    *(idx) = a1;
    idx[1] = a2;
    v1 = sub_41637e(g_4275e0, idx, idx);
    if (!v1)
    {
        sub_41bf79(); /* do not return */
    }
    else if (v1 != idx)
    {
        sub_40dd6d(idx);
        return v1[2];
    }
    else
    {
        return 0;
    }
}



// Function: hash_init @ 0xdf3d
extern unsigned long long g_4275e0;

unsigned long long hash_init(void)
{
    g_4275e0 = sub_41563d(103, 0, sub_40dce7, sub_40dd14, sub_40dd6d);
    if (g_4275e0)
        return g_4275e0;
    sub_41bf79(); /* do not return */
}



// Function: samedir_template @ 0xdf8a
extern unsigned long long g_8;

void* samedir_template(unsigned long a0, void* a1)
{
    void* ptr;  // [bp-0x20]
    unsigned long v1;  // [bp-0x18]
    unsigned long v2;  // [bp-0x10]

    v1 = sub_410369(a0) - a0;
    v2 = v1 + 9;
    if (v2 <= 0x100)
    {
        ptr = a1;
    }
    else
    {
        ptr = malloc(v2);
        if (!ptr)
            return ptr;
    }
    strcpy(mempcpy(ptr, a0, v1), "CuXXXXXX");
    return ptr;
}



// Function: try_link @ 0xe021
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    unsigned int field_10;
    unsigned int field_14;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

int try_link(long long a0, struct_0 *a1)
{
    return linkat(a1->field_0, a1->field_8, a1->field_10, a0, a1->field_14);
}



// Function: force_linkat @ 0xe069
extern unsigned long long g_10;

int force_linkat(unsigned int a0, long long a1, unsigned int a2, long long a3, unsigned int a4, char a5, int a6)
{
    unsigned int v8;  // eax
    unsigned int v9;  // eax
    int v0;  // [bp-0x144]
    char *v1;  // [bp-0x140]
    unsigned int v2;  // [bp-0x138]
    long long v3;  // [bp-0x130]
    unsigned int v4;  // [bp-0x128]
    unsigned int v5;  // [bp-0x124]
    char v6;  // [bp-0x118]

    if (a6 < 0)
        a6 = (!linkat(a0, a1, a2, a3, a4) ? 0 : *(__errno_location()));
    if (a5 != 1 || a6 != 0x11)
        return a6;
    v1 = sub_40df8a(a3, &v6);
    if (!v1)
    {
        v8 = *(__errno_location());
        v9 = *(__errno_location());
        return *(__errno_location());
    }
    v2 = a0;
    v3 = a1;
    v4 = a2;
    v5 = a4;
    if (sub_41a191(v1, 0, &v2, sub_40e021, 6))
    {
        v0 = *(__errno_location());
    }
    else
    {
        v0 = (!renameat(a2, v1, a2, a3) ? -0x1 : *(__errno_location()));
        unlinkat(a2, v1, 0, v1);
    }
    if (v1 == &v6)
        return v0;
    free(v1);
    return v0;
    return a6;
}



// Function: try_symlink @ 0xe245
typedef struct struct_0 {
    long long field_0;
    unsigned int field_8;
} struct_0;

extern unsigned long long g_8;

long long try_symlink(long long a0, struct_0 *a1)
{
    unsigned long v1;  // rcx

    v1 = a1->field_8;
    return symlinkat(a1->field_0, v1 & 0xffffffff, a0, v1);
}



// Function: force_symlinkat @ 0xe27f
extern unsigned long long g_10;

int force_symlinkat(unsigned long a0, unsigned int a1, long long a2, char a3, int a4)
{
    unsigned int v8;  // eax
    unsigned int v9;  // eax
    int v0;  // [bp-0x154]
    int v1;  // [bp-0x134]
    char *v2;  // [bp-0x130]
    unsigned long v3;  // [bp-0x128]
    unsigned int v4;  // [bp-0x120]
    char v5;  // [bp-0x118]

    v0 = a4;
    if (v0 < 0)
        v0 = (!symlinkat(a0, a1, a2, a1) ? 0 : *(__errno_location()));
    if (a3 != 1 || v0 != 0x11)
        return v0;
    v2 = sub_40df8a(a2, &v5);
    if (!v2)
    {
        v8 = *(__errno_location());
        v9 = *(__errno_location());
        return *(__errno_location());
    }
    v3 = a0;
    v4 = a1;
    if (sub_41a191(v2, 0, &v3, sub_40e245, 6))
    {
        v1 = *(__errno_location());
    }
    else if (renameat(a1, v2, a1, a2))
    {
        v1 = *(__errno_location());
        unlinkat(a1, v2, 0, v2);
    }
    else
    {
        v1 = -0x1;
    }
    if (v2 == &v5)
        return v1;
    free(v2);
    return v1;
    return v0;
}



// Function: computecon @ 0xe441
long long computecon(long long a0, unsigned int a1, long long a2)
{
    unsigned short v0;  // [bp-0x32]
    unsigned int v1;  // [bp-0x30]
    unsigned int v2;  // [bp-0x2c]
    unsigned long v3;  // [bp-0x28]
    unsigned long v4;  // [bp-0x20]
    unsigned long v5;  // [bp-0x18]

    v3 = 0;
    v4 = 0;
    v1 = 0xffffffff;
    v5 = sub_411559(a0);
    if (v5 && getcon(&v3) >= 0 && sub_419d4e(v5, &v4, &v4) >= 0)
    {
        v0 = mode_to_security_class(a1);
        if (v0)
            v1 = security_compute_create(v3, v4, v0, a2);
    }
    v2 = *(__errno_location());
    free(v5);
    freecon(v3);
    freecon(v4);
    *(__errno_location()) = v2;
    return v1;
}



// Function: defaultcon @ 0xe549
long long defaultcon(long long a0, char *a1, unsigned int a2)
{
    char *v0;  // [bp-0x68]
    unsigned int v1;  // [bp-0x50]
    unsigned int v2;  // [bp-0x4c]
    unsigned long v3;  // [bp-0x48]
    unsigned long v4;  // [bp-0x40]
    long long ptr;  // [bp-0x38]
    long long ptr1;  // [bp-0x30]
    char *v7;  // [bp-0x28]
    long long v8;  // [bp-0x20]
    long long v9;  // [bp-0x18]

    v0 = a1;
    v1 = 0xffffffff;
    v3 = 0;
    v4 = 0;
    ptr = 0;
    ptr1 = 0;
    v7 = NULL;
    if (*(v0) != 47)
    {
        v7 = sub_4111d3(v0, 2);
        if (!v7)
            goto LABEL_40e6c3;
        v0 = v7;
    }
    if (selabel_lookup(a0, &v3, v0, a2) < 0)
    {
        if (*(__errno_location()) == 2)
            *(__errno_location()) = 61;
    }
    else
    {
        if ((int)sub_40e441(v0, a2, &v4) >= 0)
        {
            ptr = context_new(v3);
            if (ptr)
            {
                ptr1 = context_new(v4);
                if (ptr1)
                {
                    v8 = context_type_get(ptr);
                    if (v8 && !context_type_set(ptr1, v8, v8))
                    {
                        v9 = context_str(ptr1);
                        if (v9)
                            v1 = setfscreatecon(v9);
                    }
                }
            }
        }
    }
LABEL_40e6c3:
    v2 = *(__errno_location());
    context_free(ptr);
    context_free(ptr1);
    freecon(v3);
    freecon(v4);
    free(v7);
    *(__errno_location()) = v2;
    return v1;
}



// Function: restorecon_private @ 0xe72c
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
    unsigned int v0;  // [bp-0xe8]
    unsigned int v1;  // [bp-0xe4]
    unsigned int v2;  // [bp-0xe0]
    unsigned int v3;  // [bp-0xdc]
    unsigned long v4;  // [bp-0xd8]
    unsigned long v5;  // [bp-0xd0]
    long long ptr;  // [bp-0xc8]
    long long ptr1;  // [bp-0xc0]
    long long v8;  // [bp-0xb8]
    long long v9;  // [bp-0xb0]
    int v10;  // [bp-0xa8], Other Possible Types: stat

    v0 = 0xffffffff;
    v4 = 0;
    v5 = 0;
    ptr = 0;
    ptr1 = 0;
    if (a0)
    {
        v1 = open(a1, 0x20000, a2);
        if (v1 != 0xffffffff || *(__errno_location()) == 40)
        {
            if (v1 != 0xffffffff)
            {
                if (fstat(v1, &v10) >= 0)
                    goto LABEL_40e8b1;
            }
            else
            {
                if (lstat(a1, &v10) >= 0)
                {
LABEL_40e8b1:
                    if (selabel_lookup(a0, &v4, a1, *((unsigned int *)(&v10 + 24))) < 0)
                    {
                        if (*(__errno_location()) == 2)
                            *(__errno_location()) = 61;
                    }
                    else
                    {
                        ptr = context_new(v4);
                        if (ptr)
                        {
                            if (v1 != 0xffffffff)
                            {
                                if (sub_419dc8(v1, &v5, &v5) >= 0)
                                    goto LABEL_40e968;
                            }
                            else
                            {
                                if (sub_419d8b(a1, &v5, &v5) >= 0)
                                {
LABEL_40e968:
                                    ptr1 = context_new(v5);
                                    if (ptr1)
                                    {
                                        v8 = context_type_get(ptr);
                                        if (v8 && !context_type_set(ptr1, v8, v8))
                                        {
                                            v9 = context_str(ptr1);
                                            if (v9)
                                                v0 = (v1 == 0xffffffff ? (unsigned int)lsetfilecon(a1, v9, v9) : (unsigned int)fsetfilecon(v1, v9, v9));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        v2 = *(__errno_location());
        if (v1 != 0xffffffff)
            close(v1);
        context_free(ptr);
        context_free(ptr1);
        freecon(v4);
        freecon(v5);
        *(__errno_location()) = v2;
        return v0;
    }
    else if (getfscreatecon(&v5) < 0)
    {
        return v0;
    }
    else if (!v5)
    {
        *(__errno_location()) = 61;
        return v0;
    }
    else
    {
        v0 = lsetfilecon(a1, v5, v5);
        v3 = *(__errno_location());
        freecon(v5);
        *(__errno_location()) = v3;
        return v0;
    }
}



// Function: restorecon @ 0xead7
typedef struct struct_0 {
    char padding_0[32];
    unsigned int field_20;
} struct_0;

extern unsigned long long g_10;

unsigned int restorecon(long long a0, char *a1, char a2)
{
    char *v0;  // [bp-0x68]
    char v1;  // [bp-0x49]
    unsigned int v2;  // [bp-0x48]
    unsigned int v3;  // [bp-0x44]
    char *v4;  // [bp-0x40]
    struct_0 *v5;  // [bp-0x38]
    unsigned long long v6;  // [bp-0x30]
    char *v7;  // [bp-0x28]
    unsigned long long v8;  // [bp-0x20]

    v0 = a1;
    v4 = NULL;
    if (*(v0) != 47)
    {
        v4 = sub_4111d3(v0, 2);
        if (!v4)
            return 0;
        v0 = v4;
    }
    if (a2 != 1)
    {
        v1 = sub_40e72c(a0, v0, *((unsigned int *)&v0)) != 0xffffffff;
        v3 = *(__errno_location());
        free(v4);
        *(__errno_location()) = v3;
        return v1;
    }
    v7 = v0;
    v8 = 0;
    v5 = sub_41bfb9(&v7, 16, 0);
    v2 = 0;
    while (1)
    {
        v6 = sub_412fac(v5);
        if (!v6)
            break;
        if (sub_40e72c(a0, *((long long *)&v5->field_20), *((long long *)&v5->field_20)) >= 0)
            continue;
        v2 = *(__errno_location());
    }
    if (*(__errno_location()))
        v2 = *(__errno_location());
    if (sub_412af2(v5))
        v2 = *(__errno_location());
    return _INSERT((unsigned long long)free(v4), 0, !v2);
}



// Function: dir_name @ 0x11559
unsigned long long dir_name(long long a0)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_411610(a0);
    if (v0)
        return v0;
    sub_41bf79(); /* do not return */
}



// Function: hash_get_n_buckets @ 0x14c84
typedef struct struct_0 {
    char padding_0[16];
    unsigned long long field_10;
} struct_0;

extern unsigned long long g_10;

long long hash_get_n_buckets(struct_0 *a0)
{
    return a0->field_10;
}



// Function: hash_get_n_buckets_used @ 0x14c9a
typedef struct struct_0 {
    char padding_0[24];
    unsigned long long field_18;
} struct_0;

long long hash_get_n_buckets_used(struct_0 *a0)
{
    return a0->field_18;
}



// Function: hash_get_n_entries @ 0x14cb0
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

long long hash_get_n_entries(struct_0 *a0)
{
    return a0->field_20;
}



// Function: hash_get_max_bucket_length @ 0x14cc6
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long hash_get_max_bucket_length(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *iter;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v1 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v3 = 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                v3 += 1;
            }
            if (v3 > v1)
                v1 = v3;
        }
    }
    return v1;
}



// Function: hash_table_ok @ 0x14d48
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned int hash_table_ok(struct_0 **a0)
{
    struct_0 *i;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long node;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v1 = 0;
    node = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            iter = i;
            v1 += 1;
            node += 1;
            while (1)
            {
                iter = iter->field_8;
                if (!iter)
                    break;
                node += 1;
            }
        }
    }
    if (v1 == a0[3] && node == a0[4])
        return 1;
    return 0;
}



// Function: hash_print_statistics @ 0x14de6
typedef struct FILE {
} FILE;

int hash_print_statistics(void* a0, FILE *ptr)
{
    uint128_t v5;  // xmm0
    long long v0;  // [bp-0x28]
    unsigned long v1;  // [bp-0x20]
    unsigned long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]

    v0 = sub_414cb0(a0);
    v1 = sub_414c84(a0);
    v2 = sub_414c9a(a0);
    v3 = sub_414cc6(a0);
    fprintf(ptr, "# entries:         %lu\n", v0);
    fprintf(ptr, "# buckets:         %lu\n", v1);
    v5 = (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1);
    fprintf(ptr, "# buckets used:    %lu (%.2f%%)\n", v2, (unsigned long long)(DivV(MulV(0x4059000000000000, (v2 < 0 ? AddV(v2 >> 1 | (unsigned int)v2 & 1, v2 >> 1 | (unsigned int)v2 & 1) : v2)), (v1 < 0 ? AddV(v1 >> 1 | (unsigned int)v1 & 1, v1 >> 1 | (unsigned int)v1 & 1) : v1))));
    return fprintf(ptr, "max bucket length: %lu\n", v3);
}



// Function: safe_hasher @ 0x14f2d
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

extern unsigned long long g_10;

long long safe_hasher(struct_0 *a0, unsigned long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = a0->field_30(a1, a0->field_10, a0->field_10, a0->field_30);
    if (v0 < a0->field_10)
        return v0 * 16 + a0->field_0;
    abort(); /* do not return */
}



// Function: hash_lookup @ 0x14f88
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

extern unsigned long long g_8;

unsigned long long hash_lookup(struct_0 *a0, unsigned long long a1)
{
    struct_0 *iter;  // [bp-0x18]
    struct_0 *v1;  // [bp-0x10]

    v1 = sub_414f2d(a0, a1);
    if (!v1->field_0)
        return 0;
    iter = v1;
    while (1)
    {
        if (!iter)
            return 0;
        if (a1 == iter->field_0 || (char)a0[1].field_0(a1, iter->field_0, iter->field_0, a0[1].field_0))
            break;
        iter = (struct_0 *)iter->padding_8;
    }
    return iter->field_0;
}



// Function: hash_get_first @ 0x1501f
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[24];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long hash_get_first(struct_0 *a0)
{
    unsigned long long *v0;  // [bp-0x10]

    if (!a0->field_20)
        return 0;
    v0 = &a0->field_0->field_0;
    while (1)
    {
        if (v0 >= *((long long *)&a0->padding_8[0]))
            abort(); /* do not return */
        if (*(v0))
            return *(v0);
        v0 += 2;
    }
}



// Function: hash_get_next @ 0x1507f
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

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long hash_get_next(struct_0 *a0, unsigned int a1)
{
    struct_0 *v0;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = sub_414f2d(a0, a1);
    iter = v0;
    while (1)
    {
        if (a1 == iter->field_0 && iter->padding_8)
            return *((long long *)iter->padding_8);
        iter = (struct_0 *)iter->padding_8;
        if (!iter)
        {
            while (1)
            {
                v0 = &v0->field_10;
                if (v0 >= a0->padding_8)
                {
                    return 0;
                }
                else if (v0->field_0)
                {
                    return v0->field_0;
                }
            }
        }
    }
}



// Function: hash_get_entries @ 0x1511d
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long hash_get_entries(struct_0 **a0, unsigned long a1, unsigned long long a2)
{
    unsigned long long idx;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    idx = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; idx = v1)
            {
                if (idx >= a2)
                    return idx;
                v1 = idx + 1;
                *((unsigned long long *)(idx * 8 + a1)) = iter->field_0;
                iter = iter->field_8;
            }
        }
    }
    return idx;
}



// Function: hash_do_for_each @ 0x151bd
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long hash_do_for_each(struct_0 **a0, unsigned long long *a1, unsigned long a2)
{
    unsigned long long v0;  // [bp-0x20]
    struct_0 *i;  // [bp-0x18]
    struct_0 *iter;  // [bp-0x10]

    v0 = 0;
    for (i = *(a0); i < a0[1]; i = &i->field_10)
    {
        if (i->field_0)
        {
            for (iter = i; iter; iter = iter->field_8)
            {
                if ((char)a1(iter->field_0, a2, a2, a1) != 1)
                    return v0;
                v0 += 1;
            }
        }
    }
    return v0;
}



// Function: hash_string @ 0x15255
unsigned long long hash_string(char *a0, unsigned long long a1)
{
    char *iter;  // [bp-0x20]
    char v1;  // [bp-0x11]
    unsigned long long v2;  // [bp-0x10]

    iter = a0;
    v2 = 0;
    while (1)
    {
        v1 = *(iter);
        if (!v1)
            break;
        v2 = (v1 + v2 * 31) % a1;
        iter += 1;
    }
    return v2;
}



// Function: is_prime @ 0x152af
long long is_prime(unsigned long long a0)
{
    unsigned long long iter;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x10]

    iter = 3;
    for (v1 = iter * iter; v1 < a0 && a0 % iter; iter += 1)
    {
        iter += 1;
        v1 += iter * 4;
    }
    return _INSERT(a0 % iter, 0, a0 % iter);
}



// Function: next_prime @ 0x1531e
extern unsigned long long g_8;

unsigned long long next_prime(unsigned long long a0)
{
    unsigned long long i;  // [bp-0x10]

    i = a0;
    if (i <= 9)
        i = 10;
    for (i |= 1; i != 0xffffffffffffffff; i += 2)
    {
        if ((char)sub_4152af(i) == 1)
            return i;
    }
    return i;
}



// Function: hash_reset_tuning @ 0x15369
typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    unsigned int field_c;
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

struct_0 * hash_reset_tuning(struct_0 *idx)
{
    idx->field_0 = 0;
    idx->field_4 = 0x3f800000;
    idx->field_8 = 1061997773;
    idx->field_c = 1068826100;
    idx->field_10 = 0;
    return idx;
}



// Function: raw_hasher @ 0x153bf
long long raw_hasher(long long a0, unsigned long long a1)
{
    unsigned long long v0;  // [bp-0x10]

    v0 = sub_41c5bc(a0, 3);
    return v0 % a1;
}



// Function: raw_comparator @ 0x153fa
long long raw_comparator(unsigned long a0, unsigned long a1)
{
    return _INSERT(a0, 0, a0 == a1);
}



// Function: check_tuning @ 0x15417
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

extern unsigned long long g_8;
extern unsigned int g_420780[4];

unsigned int check_tuning(struct_0 *idx)
{
    unsigned int v3;  // ymm1
    unsigned int v4;  // ymm1
    unsigned int v0;  // [bp-0x14]
    unsigned int *v1;  // [bp-0x10]

    v1 = &idx->field_28->field_0;
    if (v1 == &g_420780[0])
        return 1;
    v0 = 1036831949;
    if (((CmpF(v1[2], v0) & 69 | (char)((CmpF(v1[2], v0) & 69) >> 6)) & 1) != 1 && ((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69 | (char)((CmpF((unsigned int)(SubV(0x3f800000, v0)), v1[2]) & 69) >> 6)) & 1) != 1 && ((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69 | (char)((CmpF(v1[3], (unsigned int)(AddV(v0, 0x3f800000))) & 69) >> 6)) & 1) != 1 && !(CmpF(*(v1), 0) & 1))
    {
        v3 = AddV(*(v1), v0);
        if (((CmpF(v1[1], v3) & 69 | (char)((CmpF(v1[1], v3) & 69) >> 6)) & 1) != 1 && !(CmpF(0x3f800000, v1[1]) & 1))
        {
            v4 = AddV(*(v1), v0);
            if (((CmpF(v1[2], v4) & 69 | (char)((CmpF(v1[2], v4) & 69) >> 6)) & 1) != 1)
                return 1;
        }
    }
    idx->field_28 = &g_420780[0];
    return 0;
}



// Function: compute_bucket_size @ 0x15524
typedef struct struct_0 {
    char padding_0[8];
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long compute_bucket_size(unsigned long a0, struct_0 *a1)
{
    char v3;  // al
    unsigned long v0;  // [bp-0x20], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0xc]

    v0 = a0;
    if (a1->field_10 != 1)
    {
        v1 = DivV((v0 < 0 ? AddV((unsigned int)(v0 >> 1 | (unsigned int)v0 & 1), (unsigned int)(v0 >> 1 | (unsigned int)v0 & 1)) : (unsigned int)v0), a1->field_8);
        if (!(CmpF(v1, 0x5f800000) & 1))
            return 0;
        if (CmpF(v1, 0x5f000000) & 1)
        {
            v0 = v1;
        }
        else
        {
            v0 = (unsigned int)(SubV(v1, 0x5f000000));
            v0 ^= 0x8000000000000000;
        }
    }
    v0 = sub_41531e(v0);
    v3 = 0;
    if (v0 & 0xe000000000000000)
        v3 = 1;
    if (v0 * 8 < 0)
        v3 = 1;
    if (!(v3 & 1))
        return v0;
    return 0;
}



// Function: hash_initialize @ 0x1563d
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

extern unsigned long long g_8;
extern unsigned long long g_10;
extern struct_0 g_420780;

struct_0 * hash_initialize(unsigned int a0, struct_0 *a1, unsigned long long a2, unsigned long long a3, unsigned long a4)
{
    unsigned long long v0;  // [bp-0x38]
    unsigned long long v1;  // [bp-0x30]
    struct_0 *v2;  // [bp-0x28]
    struct_0 *ptr;  // [bp-0x10]

    v2 = a1;
    v1 = a2;
    v0 = a3;
    if (!v1)
        v1 = sub_4153bf;
    if (!v0)
        v0 = sub_4153fa;
    ptr = malloc(80);
    if (!ptr)
        return NULL;
    if (!v2)
        v2 = &g_420780.padding_0[0];
    ptr->field_28 = v2;
    if ((char)sub_415417(ptr) == 1)
    {
        *((unsigned long long *)&ptr->field_10) = sub_415524(a0, v2);
        if (*((long long *)&ptr->field_10))
        {
            ptr->field_0 = calloc(*((long long *)&ptr->field_10), 16);
            if (ptr->field_0)
            {
                ptr->field_8 = ptr->field_0 + *((long long *)&ptr->field_10) * 16;
                ptr->field_18 = 0;
                ptr->field_20 = 0;
                ptr->field_30 = v1;
                ptr->field_38 = v0;
                ptr->field_40 = a4;
                ptr->field_48 = 0;
                return ptr;
            }
        }
    }
    free(ptr);
    return NULL;
}



// Function: hash_clear @ 0x157b2
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
    unsigned long long field_20;
    char padding_28[24];
    struct struct_3 *field_40;
    struct struct_0 *field_48;
} struct_2;

typedef struct struct_3 {
    unsigned long long field_0;
} struct_3;

extern unsigned long long g_8;
extern unsigned long long g_10;

struct_2 * hash_clear(struct_2 *idx)
{
    struct_1 *iter;  // [bp-0x20]
    struct_0 *idx1;  // [bp-0x18]
    struct_0 *v2;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx1 = iter->field_8; idx1; idx1 = v2)
            {
                if (idx->field_40)
                    idx->field_40(idx1->field_0);
                idx1->field_0 = 0;
                v2 = idx1->field_8;
                idx1->field_8 = idx->field_48;
                idx->field_48 = idx1;
            }
            if (idx->field_40)
                idx->field_40(iter->field_0);
            iter->field_0 = 0;
            iter->field_8 = NULL;
        }
    }
    idx->field_18 = 0;
    idx->field_20 = 0;
    return idx;
}



// Function: hash_free @ 0x158bd
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

extern unsigned long long g_8;
extern unsigned long long g_10;

void hash_free(struct_2 *ptr)
{
    struct_0 *node;  // [bp-0x20]
    struct_1 *iter;  // [bp-0x18], Other Possible Types: struct_0 *
    struct_0 *v2;  // [bp-0x10], Other Possible Types: struct_1 *

    if (ptr->field_40 && ptr->field_20)
    {
        for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
        {
            if (node->field_0)
            {
                for (iter = node; iter; iter = iter->field_8)
                {
                    ptr->field_40(iter->field_0);
                }
            }
        }
    }
    for (node = ptr->field_0; node < *((long long *)&ptr->padding_8[0]); node = &node->field_10)
    {
        for (iter = node->field_8; iter; iter = v2)
        {
            v2 = iter->field_8;
            free(iter);
        }
    }
    for (iter = ptr->field_48; iter; iter = v2)
    {
        v2 = iter->field_8;
        free(iter);
    }
    free(ptr->field_0);
    free(ptr);
    return;
}



// Function: allocate_entry @ 0x159ec
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
} struct_0;

typedef struct struct_1 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_1;

extern unsigned long long g_8;

struct_0 * allocate_entry(struct_1 *idx)
{
    struct_0 *ptr;  // [bp-0x10]

    if (!idx->field_48)
    {
        ptr = malloc(16);
        return ptr;
    }
    ptr = idx->field_48;
    idx->field_48 = ptr->field_8;
    return ptr;
}



// Function: free_entry @ 0x15a3b
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    char padding_0[72];
    struct struct_0 *field_48;
} struct_2;

extern unsigned long long g_8;

struct_2 * free_entry(struct_2 *idx, struct_0 *a1)
{
    a1->field_0 = 0;
    a1->field_8 = idx->field_48;
    idx->field_48 = a1;
    return idx;
}



// Function: hash_find_entry @ 0x15a75
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    char padding_0[56];
    struct struct_1 *field_38;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

extern unsigned long long g_8;

unsigned long long hash_find_entry(struct_0 *a0, unsigned long long a1, struct_2 **a2, char a3)
{
    struct_2 *v7;  // rdx
    struct_2 *iter;  // [bp-0x38]
    struct_2 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    unsigned long long v4;  // [bp-0x18]
    struct_2 *v5;  // [bp-0x10]

    index = sub_414f2d(a0, a1);
    *(a2) = index;
    if (!index->field_0)
        return 0;
    if (a1 != index->field_0 && !(char)a0->field_38(a1, index->field_0, index->field_0, a0->field_38))
    {
        iter = index;
        while (1)
        {
            if (!iter->field_8)
                return 0;
            if (a1 == iter->field_8->field_0 || (char)a0->field_38(a1, iter->field_8->field_0, iter->field_8->field_0, a0->field_38))
                break;
            iter = iter->field_8;
        }
        v2 = iter->field_8->field_0;
        if (!a3)
            return v2;
        v3 = iter->field_8;
        iter->field_8 = v3->field_8;
        sub_415a3b(a0, v3);
        return v2;
    }
    v4 = index->field_0;
    if (!a3)
        return v4;
    if (index->field_8)
    {
        v5 = index->field_8;
        v7 = v5->field_8;
        index->field_0 = v5->field_0;
        index->field_8 = v7;
        sub_415a3b(a0, v5);
    }
    else
    {
        index->field_0 = 0;
    }
    return v4;
}



// Function: transfer_entries @ 0x15c0e
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

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned int transfer_entries(struct_2 *index, struct_2 *idx, char a2)
{
    struct_1 *iter;  // [bp-0x38]
    struct_0 *idx2;  // [bp-0x30]
    unsigned long v2;  // [bp-0x28], Other Possible Types: unsigned long long
    struct_0 *idx1;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]
    struct_0 *v5;  // [bp-0x10]

    for (iter = idx->field_0; iter < *((long long *)&idx->padding_8[0]); iter = &iter->field_10)
    {
        if (iter->field_0)
        {
            for (idx2 = iter->field_8; idx2; idx2 = v5)
            {
                v2 = idx2->field_0;
                idx1 = sub_414f2d(index, v2);
                v5 = idx2->field_8;
                if (idx1->field_0)
                {
                    idx2->field_8 = idx1->field_8;
                    idx1->field_8 = idx2;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                    sub_415a3b(index, idx2);
                }
            }
            v2 = iter->field_0;
            iter->field_8 = NULL;
            if (!a2)
            {
                idx1 = sub_414f2d(index, v2);
                if (idx1->field_0)
                {
                    v4 = sub_4159ec(index);
                    if (!v4)
                        return 0;
                    v4->field_0 = v2;
                    v4->field_8 = idx1->field_8;
                    idx1->field_8 = v4;
                }
                else
                {
                    idx1->field_0 = v2;
                    index->field_18 = index->field_18 + 1;
                }
                iter->field_0 = 0;
                idx->field_18 = idx->field_18 - 1;
            }
        }
    }
    return 1;
}



// Function: hash_rehash @ 0x15de2
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

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned int hash_rehash(struct_2 *index, unsigned int a1)
{
    unsigned long v0;  // [bp-0x78]
    struct_2 *idx;  // [bp-0x70]
    char v2;  // [bp-0x68], Other Possible Types: unsigned long
    unsigned long v3;  // [bp-0x60]
    unsigned long v4;  // [bp-0x58]
    unsigned long long v5;  // [bp-0x50]
    unsigned long long v6;  // [bp-0x48]
    unsigned long v7;  // [bp-0x40]
    unsigned long v8;  // [bp-0x38]
    unsigned long v9;  // [bp-0x30]
    unsigned long v10;  // [bp-0x28]
    unsigned long v11;  // [bp-0x20]

    v0 = sub_415524(a1, *((long long *)&index[1].padding_8[0]));
    if (!v0)
    {
        return 0;
    }
    else if (v0 == *((long long *)&index->padding_8[8]))
    {
        return 1;
    }
    else
    {
        idx = &v2;
        v2 = calloc(v0, 16);
        if (!v2)
            return 0;
        v4 = v0;
        v3 = v2 + v0 * 16;
        v5 = 0;
        v6 = 0;
        v7 = *((long long *)&index[1].padding_8[0]);
        v8 = *((long long *)&index[1].padding_8[8]);
        v9 = index[1].field_18;
        v10 = index[2].field_0;
        v11 = *((long long *)&index[2].padding_8[0]);
        if ((char)sub_415c0e(idx, index, 0))
        {
            free(index->field_0);
            index->field_0 = v2;
            *((unsigned long *)&index->padding_8[0]) = v3;
            *((unsigned long *)&index->padding_8[8]) = v4;
            index->field_18 = v5;
            *((unsigned long *)&index[2].padding_8[0]) = v11;
            return 1;
        }
        *((unsigned long *)&index[2].padding_8[0]) = v11;
        if ((char)sub_415c0e(index, idx, 1) == 1 && (char)sub_415c0e(index, idx, 0) == 1)
        {
            free(v2);
            return 0;
        }
        abort(); /* do not return */
    }
}



// Function: hash_insert_if_absent @ 0x1600d
typedef struct struct_0 {
    unsigned long long field_0;
    struct struct_0 *field_8;
} struct_0;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned int hash_insert_if_absent(void* idx, unsigned long long a1, unsigned long long *a2)
{
    unsigned long v6;  // rax
    unsigned int v7;  // ymm0
    unsigned long v16;  // rax
    unsigned int v17;  // ymm0
    unsigned long v18;  // rax
    int v8;  // xmm2
    unsigned long v9;  // rax
    unsigned long v11;  // rax
    unsigned int v12;  // ymm0
    int v13;  // xmm2
    unsigned long v14;  // rax
    unsigned int v0;  // [bp-0x34]
    struct_0 *index;  // [bp-0x30]
    unsigned long long v2;  // [bp-0x28]
    struct_2 *v3;  // [bp-0x20]
    struct_0 *v4;  // [bp-0x18]

    if (!a1)
        abort(); /* do not return */
    v2 = sub_415a75(idx, a1, &index, 0);
    if (v2)
    {
        if (!a2)
            return 0;
        *(a2) = v2;
        return 0;
    }
    else
    {
        v6 = (long long)idx[24];
        v7 = (v6 < 0 ? (unsigned int)(AddV((unsigned int)(v6 >> 1 | (unsigned int)v6 & 1), (unsigned int)(v6 >> 1 | (unsigned int)v6 & 1))) : (int)v6);
        v8 = (int)*((int *)((long long)idx[40] + 8));
        v9 = (long long)idx[16];
        if (...)
        {
            sub_415417(idx);
            v11 = (long long)idx[24];
            v12 = (v11 < 0 ? (unsigned int)(AddV((unsigned int)(v11 >> 1 | (unsigned int)v11 & 1), (unsigned int)(v11 >> 1 | (unsigned int)v11 & 1))) : (int)v11);
            v13 = (int)*((int *)((long long)idx[40] + 8));
            v14 = (long long)idx[16];
            if (...)
            {
                v3 = (long long)idx[40];
                if ((char)v3[1].field_0)
                {
                    v16 = (long long)idx[16];
                    v17 = MulV((v16 < 0 ? AddV((unsigned int)(v16 >> 1 | (unsigned int)v16 & 1), (unsigned int)(v16 >> 1 | (unsigned int)v16 & 1)) : (unsigned int)v16), *((int *)((char *)&v3->field_8 + 4)));
                }
                else
                {
                    v18 = (long long)idx[16];
                    v17 = MulV((int)v3->field_8, MulV(*((int *)((char *)&v3->field_8 + 4)), (v18 < 0 ? AddV((unsigned int)(v18 >> 1 | (unsigned int)v18 & 1), (unsigned int)(v18 >> 1 | (unsigned int)v18 & 1)) : (unsigned int)v18)));
                }
                v0 = v17;
                if (!(CmpF(v0, 0x5f800000) & 1))
                {
                    return 0xffffffff;
                }
                else if ((char)sub_415de2(idx, (CmpF(v0, 0x5f000000) & 69 & 1 ? (unsigned int)v0 : (unsigned int)(SubV(v0, 0x5f000000)) ^ 0)) != 1)
                {
                    return 0xffffffff;
                }
                else if (sub_415a75(idx, a1, &index, 0))
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
        v4 = sub_4159ec(idx);
        if (v4)
        {
            v4->field_0 = a1;
            v4->field_8 = index->field_8;
            index->field_8 = v4;
            *((unsigned long long *)&idx[32]) = (long long)idx[32] + 1;
            return 1;
        }
        return 0xffffffff;
    }
}



// Function: hash_insert @ 0x1637e
unsigned long long hash_insert(void* a0, unsigned long long a1)
{
    unsigned int v0;  // [bp-0x1c]
    unsigned long long v1;  // [bp-0x18]

    v0 = sub_41600d(a0, a1, &v1);
    if (v0 == 0xffffffff)
    {
        return 0;
    }
    else if (!v0)
    {
        return v1;
    }
    else
    {
        return a1;
    }
}



// Function: hash_remove @ 0x163ee
typedef struct struct_1 {
    char padding_0[8];
    struct struct_1 *field_8;
} struct_1;

typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

typedef struct struct_0 {
    unsigned int field_0;
    unsigned int field_4;
    unsigned int field_8;
    char padding_c[4];
    char field_10;
} struct_0;

extern unsigned long long g_8;
extern unsigned long long g_10;

unsigned long long hash_remove(struct_2 *idx, unsigned long long a1)
{
    unsigned long v7;  // rax
    unsigned int v8;  // ymm0
    unsigned long v17;  // rax
    int v18;  // xmm0
    unsigned long long v19;  // rax
    unsigned long v20;  // rax
    int v21;  // xmm0
    int v9;  // xmm2
    unsigned long v10;  // rax
    unsigned long v12;  // rax
    unsigned int v13;  // ymm0
    int v14;  // xmm2
    unsigned long v15;  // rax
    struct_2 *v0;  // [bp-0x40]
    struct_1 *ptr;  // [bp-0x38]
    unsigned long long v2;  // [bp-0x30]
    struct_0 *v3;  // [bp-0x28]
    unsigned long long v4;  // [bp-0x20]
    struct_1 *v5;  // [bp-0x18]

    v2 = sub_415a75(idx, a1, &v0, 1);
    if (!v2)
        return 0;
    idx[2].field_0 = idx[2].field_0 - 1;
    if (v0->field_0)
        return v2;
    idx[1].field_8 = (char *)idx[1].field_8 - 1;
    v7 = idx[1].field_8;
    v8 = (v7 < 0 ? (unsigned int)(AddV((unsigned int)(v7 >> 1 | (unsigned int)v7 & 1), (unsigned int)(v7 >> 1 | (unsigned int)v7 & 1))) : (int)v7);
    v9 = (int)(unsigned int)idx[2].field_8->field_0;
    v10 = idx[1].field_0;
    if (...)
    {
        sub_415417(idx);
        v12 = idx[1].field_8;
        v13 = (v12 < 0 ? (unsigned int)(AddV((unsigned int)(v12 >> 1 | (unsigned int)v12 & 1), (unsigned int)(v12 >> 1 | (unsigned int)v12 & 1))) : (int)v12);
        v14 = (int)(unsigned int)idx[2].field_8->field_0;
        v15 = idx[1].field_0;
        if (...)
        {
            v3 = idx[2].field_8;
            if (v3->field_10)
            {
                v17 = idx[1].field_0;
                v18 = (int)(MulV((v17 < 0 ? AddV((unsigned int)(v17 >> 1 | (unsigned int)v17 & 1), (unsigned int)(v17 >> 1 | (unsigned int)v17 & 1)) : (unsigned int)v17), v3->field_4));
                v19 = (CmpF(*((unsigned int *)&v18), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v18) : *((unsigned int *)&SubV(v18, 0x5f000000)) ^ 0x8000000000000000);
            }
            else
            {
                v20 = idx[1].field_0;
                v21 = (int)(MulV(v3->field_8, MulV(v3->field_4, (v20 < 0 ? AddV((unsigned int)(v20 >> 1 | (unsigned int)v20 & 1), (unsigned int)(v20 >> 1 | (unsigned int)v20 & 1)) : (unsigned int)v20))));
                v19 = (CmpF(*((unsigned int *)&v21), 0x5f000000) & 69 & 1 ? *((unsigned int *)&v21) : *((unsigned int *)&SubV(v21, 0x5f000000)) ^ 0x8000000000000000);
            }
            v4 = v19;
            if ((char)sub_415de2(idx, v4) != 1)
            {
                for (ptr = idx[4].field_8; ptr; ptr = v5)
                {
                    v5 = ptr->field_8;
                    free(ptr);
                }
                idx[4].field_8 = NULL;
            }
        }
    }
    return v2;
}



// Function: hash_delete @ 0x166f4
typedef struct struct_2 {
    unsigned long long field_0;
    struct struct_2 *field_8;
} struct_2;

unsigned long long hash_delete(struct_2 *a0, unsigned long long a1)
{
    return sub_4163ee(a0, a1);
}



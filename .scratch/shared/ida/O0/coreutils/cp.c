// Function: usage @ 0x44b3
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v3; // r13
  long long v4; // r12
  long long v5; // rbx
  char *v6; // rax
  FILE *v7; // rbx
  char *v8; // rax
  FILE *v9; // rbx
  char *v10; // rax
  FILE *v11; // rbx
  char *v12; // rax
  FILE *v13; // rbx
  char *v14; // rax
  FILE *v15; // rbx
  char *v16; // rax
  FILE *v17; // rbx
  char *v18; // rax
  FILE *v19; // rbx
  char *v20; // rax
  FILE *v21; // rbx
  char *v22; // rax
  FILE *v23; // rbx
  char *v24; // rax
  FILE *v25; // rbx
  char *v26; // rax
  FILE *v27; // rbx
  char *v28; // rax
  FILE *v29; // rbx
  char *v30; // rax
  FILE *v31; // rbx
  char *v32; // rax
  FILE *v33; // rbx
  char *v34; // rax
  FILE *v35; // rbx
  char *v36; // rax
  FILE *v37; // rbx
  char *v38; // rdi
  FILE *v39; // rbx
  char *v40; // rax

  if ( a1 )
  {
    v1 = qword_27608;
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = qword_27608;
    v4 = qword_27608;
    v5 = qword_27608;
    v6 = gettext(
           "Usage: %s [OPTION]... [-T] SOURCE DEST\n"
           "  or:  %s [OPTION]... SOURCE... DIRECTORY\n"
           "  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n");
    printf(v6, v5, v4, v3);
    v7 = stdout;
    v8 = gettext("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n");
    fputs_unlocked(v8, v7);
    sub_4199();
    v9 = stdout;
    v10 = gettext(
            "  -a, --archive                same as -dR --preserve=all\n"
            "      --attributes-only        don't copy the file data, just the attributes\n"
            "      --backup[=CONTROL]       make a backup of each existing destination file\n"
            "  -b                           like --backup but does not accept an argument\n"
            "      --copy-contents          copy contents of special files when recursive\n"
            "  -d                           same as --no-dereference --preserve=links\n");
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = gettext(
            "  -f, --force                  if an existing destination file cannot be\n"
            "                                 opened, remove it and try again (this option\n"
            "                                 is ignored when the -n option is also used)\n"
            "  -i, --interactive            prompt before overwrite (overrides a previous -n\n"
            "                                  option)\n"
            "  -H                           follow command-line symbolic links in SOURCE\n");
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = gettext(
            "  -l, --link                   hard link files instead of copying\n"
            "  -L, --dereference            always follow symbolic links in SOURCE\n");
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = gettext(
            "  -n, --no-clobber             do not overwrite an existing file (overrides\n"
            "                                 a previous -i option)\n"
            "  -P, --no-dereference         never follow symbolic links in SOURCE\n");
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = gettext(
            "  -p                           same as --preserve=mode,ownership,timestamps\n"
            "      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n"
            "                                 mode,ownership,timestamps), if possible\n"
            "                                 additional attributes: context, links, xattr,\n"
            "                                 all\n");
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = gettext(
            "      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n"
            "      --parents                use full source file name under DIRECTORY\n");
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = gettext(
            "  -R, -r, --recursive          copy directories recursively\n"
            "      --reflink[=WHEN]         control clone/CoW copies. See below\n"
            "      --remove-destination     remove each existing destination file before\n"
            "                                 attempting to open it (contrast with --force)\n");
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = gettext(
            "      --sparse=WHEN            control creation of sparse files. See below\n"
            "      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n"
            "                                 argument\n");
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = gettext(
            "  -s, --symbolic-link          make symbolic links instead of copying\n"
            "  -S, --suffix=SUFFIX          override the usual backup suffix\n"
            "  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n"
            "  -T, --no-target-directory    treat DEST as a normal file\n");
    fputs_unlocked(v26, v25);
    v27 = stdout;
    v28 = gettext(
            "  -u, --update                 copy only when the SOURCE file is newer\n"
            "                                 than the destination file or when the\n"
            "                                 destination file is missing\n"
            "  -v, --verbose                explain what is being done\n"
            "  -x, --one-file-system        stay on this file system\n");
    fputs_unlocked(v28, v27);
    v29 = stdout;
    v30 = gettext(
            "  -Z                           set SELinux security context of destination\n"
            "                                 file to default type\n"
            "      --context[=CTX]          like -Z, or if CTX is specified then set the\n"
            "                                 SELinux or SMACK security context to CTX\n");
    fputs_unlocked(v30, v29);
    v31 = stdout;
    v32 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v32, v31);
    v33 = stdout;
    v34 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v34, v33);
    v35 = stdout;
    v36 = gettext(
            "\n"
            "By default, sparse SOURCE files are detected by a crude heuristic and the\n"
            "corresponding DEST file is made sparse as well.  That is the behavior\n"
            "selected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\n"
            "file whenever the SOURCE file contains a long enough sequence of zero bytes.\n"
            "Use --sparse=never to inhibit creation of sparse files.\n");
    fputs_unlocked(v36, v35);
    v37 = stdout;
    v38 = gettext(
            "\n"
            "When --reflink[=always] is specified, perform a lightweight copy, where the\n"
            "data blocks are copied only when modified.  If this is not possible the copy\n"
            "fails, or if --reflink=auto is specified, fall back to a standard copy.\n"
            "Use --reflink=never to ensure a standard copy is performed.\n");
    fputs_unlocked(v38, v37);
    sub_41CA();
    v39 = stdout;
    v40 = gettext(
            "\n"
            "As a special case, cp makes a backup of SOURCE when the force and backup\n"
            "options are given and SOURCE and DEST are the same name for an existing,\n"
            "regular file.\n");
    fputs_unlocked(v40, v39);
    sub_421C("cp");
  }
  exit(a1);
}



// Function: re_protect @ 0x4793
long long re_protect(char *a1, int a2, long long a3, long long a4, char *a5)
{
  size_t v5; // rax
  unsigned long long v6; // rax
  void *v7; // rsp
  __syscall_slong_t v8; // rdx
  long long v9; // rdx
  long long v10; // r12
  char *v11; // rbx
  int *v12; // rax
  long long v14; // r12
  char *v15; // rbx
  int *v16; // rax
  long long v17; // r12
  char *v18; // rbx
  int *v19; // rax
  char v20[8]; // [rsp+8h] [rbp-B0h] BYREF
  char *v21; // [rsp+10h] [rbp-A8h]
  long long v22; // [rsp+18h] [rbp-A0h]
  long long v23; // [rsp+20h] [rbp-98h]
  int fd; // [rsp+2Ch] [rbp-8Ch]
  char *v25; // [rsp+30h] [rbp-88h]
  int v26; // [rsp+44h] [rbp-74h]
  long long i; // [rsp+48h] [rbp-70h]
  char *s; // [rsp+50h] [rbp-68h]
  size_t n; // [rsp+58h] [rbp-60h]
  void *dest; // [rsp+60h] [rbp-58h]
  char *v31; // [rsp+68h] [rbp-50h]
  char *path; // [rsp+70h] [rbp-48h]
  struct timespec times; // [rsp+78h] [rbp-40h] BYREF
  long long v34; // [rsp+88h] [rbp-30h]
  long long v35; // [rsp+90h] [rbp-28h]
  unsigned long long v36; // [rsp+A0h] [rbp-18h]

  v25 = a1;
  fd = a2;
  v23 = a3;
  v22 = a4;
  v21 = a5;
  v36 = __readfsqword(0x28u);
  s = a1;
  v5 = strlen(a1);
  n = v5 + 1;
  v6 = 16 * ((v5 + 24) / 0x10);
  while ( v20 != &v20[-(v6 & 0xFFFFFFFFFFFFF000LL)] )
    ;
  v7 = alloca(v6 & 0xFFF);
  if ( (v6 & 0xFFF) != 0 )
    *(long long *)&v20[(v6 & 0xFFF) - 8] = *(long long *)&v20[(v6 & 0xFFF) - 8];
  dest = v20;
  v31 = memcpy(v20, s, n);
  path = &v31[v23 - (long long)v25];
  for ( i = v22; i; i = *(long long *)(i + 160) )
  {
    v31[*(long long *)(i + 152)] = 0;
    if ( v21[31] )
    {
      times.tv_sec = sub_19E57(i);
      times.tv_nsec = v8;
      v34 = sub_19E8B(i);
      v35 = v9;
      if ( utimensat(fd, path, &times, 0) )
      {
        v10 = sub_18A0F(4, v31);
        v11 = gettext("failed to preserve times for %s");
        v12 = __errno_location();
        error(0, *v12, v11, v10);
        return 0;
      }
    }
    if ( v21[29]
      && (unsigned int)sub_112C3((unsigned int)fd, path, *(unsigned int *)(i + 28), *(unsigned int *)(i + 32)) )
    {
      if ( (unsigned char)sub_DC1F(v21) != 1 )
      {
        v14 = sub_18A0F(4, v31);
        v15 = gettext("failed to preserve ownership for %s");
        v16 = __errno_location();
        error(0, *v16, v15, v14);
        return 0;
      }
      v26 = sub_112C3((unsigned int)fd, path, 0xFFFFFFFFLL, *(unsigned int *)(i + 32));
    }
    if ( v21[30] )
    {
      if ( (unsigned int)sub_EC57(path, 0xFFFFFFFFLL, v31, 0xFFFFFFFFLL, *(unsigned int *)(i + 24)) )
        return 0;
    }
    else if ( *(char *)(i + 144) && (unsigned int)sub_11260((unsigned int)fd, path, *(unsigned int *)(i + 24)) )
    {
      v17 = sub_18A0F(4, v31);
      v18 = gettext("failed to preserve permissions for %s");
      v19 = __errno_location();
      error(0, *v19, v18, v17);
      return 0;
    }
    v31[*(long long *)(i + 152)] = 47;
  }
  return 1;
}



// Function: make_dir_parents_private @ 0x4b50
long long make_dir_parents_private(const char *a1, size_t a2, int a3, char *a4, __dev_t **a5, char *a6, long long a7)
{
  size_t v8; // rax
  unsigned long long v9; // rax
  void *v10; // rsp
  size_t v11; // rax
  void *v12; // rsp
  int v13; // eax
  int v14; // eax
  long long v15; // rbx
  char *v16; // rax
  __dev_t *v17; // rax
  __ino_t st_ino; // rbx
  __dev_t v19; // rbx
  __dev_t st_rdev; // rbx
  __blksize_t st_blksize; // rbx
  __time_t tv_sec; // rbx
  __time_t v23; // rbx
  __time_t v24; // rbx
  __syscall_slong_t v25; // rbx
  __syscall_slong_t v26; // rbx
  unsigned int v27; // eax
  int v28; // eax
  short v29; // ax
  long long v30; // r12
  char *v31; // rbx
  int *v32; // rax
  long long v33; // r12
  char *v34; // rbx
  int *v35; // rax
  long long v36; // r12
  char *v37; // rbx
  int *v38; // rax
  long long v39; // rbx
  char *v40; // rax
  char *v41; // rax
  long long v42; // rbx
  char *v43; // rax
  char v44[8]; // [rsp+8h] [rbp-1F0h] BYREF
  long long v45; // [rsp+10h] [rbp-1E8h]
  char *v46; // [rsp+18h] [rbp-1E0h]
  __dev_t **v47; // [rsp+20h] [rbp-1D8h]
  char *format; // [rsp+28h] [rbp-1D0h]
  int fd; // [rsp+34h] [rbp-1C4h]
  size_t v50; // [rsp+38h] [rbp-1C0h]
  char *v51; // [rsp+40h] [rbp-1B8h]
  bool v52; // [rsp+53h] [rbp-1A5h]
  int v53; // [rsp+54h] [rbp-1A4h]
  int errnum; // [rsp+58h] [rbp-1A0h]
  int v55; // [rsp+5Ch] [rbp-19Ch]
  __mode_t mode; // [rsp+60h] [rbp-198h]
  __mode_t v57; // [rsp+64h] [rbp-194h]
  char *file; // [rsp+68h] [rbp-190h]
  char *i; // [rsp+70h] [rbp-188h]
  __dev_t *v60; // [rsp+78h] [rbp-180h]
  size_t v61; // [rsp+80h] [rbp-178h]
  char *s; // [rsp+88h] [rbp-170h]
  size_t n; // [rsp+90h] [rbp-168h]
  void *dest; // [rsp+98h] [rbp-160h]
  void *src; // [rsp+A0h] [rbp-158h]
  char *v66; // [rsp+A8h] [rbp-150h]
  void *v67; // [rsp+B0h] [rbp-148h]
  struct stat buf; // [rsp+B8h] [rbp-140h] BYREF
  struct stat v69; // [rsp+148h] [rbp-B0h] BYREF
  unsigned long long v70; // [rsp+1E0h] [rbp-18h]

  v51 = (char *)a1;
  v50 = a2;
  fd = a3;
  format = a4;
  v47 = a5;
  v46 = a6;
  v45 = a7;
  v70 = __readfsqword(0x28u);
  v61 = sub_1158B(a1);
  *v47 = 0;
  if ( v50 >= v61 )
    return 1;
  s = v51;
  v8 = strlen(v51);
  n = v8 + 1;
  v9 = 16 * ((v8 + 24) / 0x10);
  while ( v44 != &v44[-(v9 & 0xFFFFFFFFFFFFF000LL)] )
    ;
  v10 = alloca(v9 & 0xFFF);
  if ( (v9 & 0xFFF) != 0 )
    *(long long *)&v44[(v9 & 0xFFF) - 8] = *(long long *)&v44[(v9 & 0xFFF) - 8];
  dest = v44;
  src = memcpy(v44, s, n);
  v66 = (char *)src + v50;
  v11 = 16 * ((v61 + 24) / 0x10);
  while ( v44 != &v44[-(v11 & 0xFFFFFFFFFFFFF000LL)] )
    ;
  v12 = alloca(v11 & 0xFFF);
  if ( (v11 & 0xFFF) != 0 )
    *(long long *)&v44[(v11 & 0xFFF) - 8] = *(long long *)&v44[(v11 & 0xFFF) - 8];
  v67 = v44;
  memcpy(v44, src, v61);
  *((char *)v67 + v61) = 0;
  for ( file = (char *)v67 + v50; *file == 47; ++file )
    ;
  if ( fstatat(fd, file, &buf, 0) )
  {
    for ( i = v66; *i == 47; ++i )
      ;
    file = i;
    while ( 1 )
    {
      i = strchr(i, 47);
      if ( !i )
        break;
      *i = 0;
      v13 = fstatat(fd, file, &buf, 0);
      v52 = v13 != 0;
      if ( v13 || *(char *)(v45 + 29) || *(char *)(v45 + 30) || *(char *)(v45 + 31) )
      {
        if ( stat(v66, &v69) )
        {
          v14 = *__errno_location();
        }
        else if ( (v69.st_mode & 0xF000) == 0x4000 )
        {
          v14 = 0;
        }
        else
        {
          v14 = 20;
        }
        errnum = v14;
        if ( v14 )
        {
          v15 = sub_18A0F(4, v66);
          v16 = gettext("failed to get attributes of %s");
          error(0, errnum, v16, v15);
          return 0;
        }
        v17 = (__dev_t *)sub_1B941(168);
        v60 = v17;
        st_ino = v69.st_ino;
        *v17 = v69.st_dev;
        v17[1] = st_ino;
        v19 = *(long long *)&v69.st_mode;
        v17[2] = v69.st_nlink;
        v17[3] = v19;
        st_rdev = v69.st_rdev;
        v17[4] = *(long long *)&v69.st_gid;
        v17[5] = st_rdev;
        st_blksize = v69.st_blksize;
        v17[6] = v69.st_size;
        v17[7] = st_blksize;
        tv_sec = v69.st_atim.tv_sec;
        v17[8] = v69.st_blocks;
        v17[9] = tv_sec;
        v23 = v69.st_mtim.tv_sec;
        v17[10] = v69.st_atim.tv_nsec;
        v17[11] = v23;
        v24 = v69.st_ctim.tv_sec;
        v17[12] = v69.st_mtim.tv_nsec;
        v17[13] = v24;
        v25 = v69.__unused[0];
        v17[14] = v69.st_ctim.tv_nsec;
        v17[15] = v25;
        v26 = v69.__unused[2];
        v17[16] = v69.__unused[1];
        v17[17] = v26;
        v60[19] = i - (char *)src;
        *((char *)v60 + 144) = 0;
        v60[20] = (__dev_t)*v47;
        *v47 = v60;
      }
      if ( v52 )
        v27 = *((int *)v60 + 6);
      else
        v27 = 0;
      if ( (unsigned char)sub_84C6(v66, src, v27, v52, v45) != 1 )
        return 0;
      if ( v52 )
      {
        *v46 = 1;
        v55 = *((int *)v60 + 6);
        if ( *(char *)(v45 + 29) != 1 )
        {
          if ( *(char *)(v45 + 30) )
            v28 = 18;
          else
            v28 = 0;
        }
        else
        {
          v28 = 63;
        }
        v53 = v55 & v28;
        if ( *(char *)(v45 + 32) )
          v29 = 511;
        else
          v29 = v55;
        mode = (unsigned short)v29 & (unsigned short)~(short)v53 & 0xFFF;
        if ( mkdirat(fd, file, mode) )
        {
          v30 = sub_18A0F(4, src);
          v31 = gettext("cannot make directory %s");
          v32 = __errno_location();
          error(0, *v32, v31, v30);
          return 0;
        }
        if ( format )
          printf(format, v66, src);
        if ( fstatat(fd, file, &buf, 256) )
        {
          v33 = sub_18A0F(4, src);
          v34 = gettext("failed to get attributes of %s");
          v35 = __errno_location();
          error(0, *v35, v34, v33);
          return 0;
        }
        if ( *(char *)(v45 + 30) != 1 )
        {
          if ( (v53 & ~buf.st_mode) != 0 )
            v53 &= ~(unsigned int)sub_DCAF();
          if ( (v53 & ~buf.st_mode) != 0 || (buf.st_mode & 0x1C0) != 0x1C0 )
          {
            *((int *)v60 + 6) = v53 | buf.st_mode;
            *((char *)v60 + 144) = 1;
          }
        }
        v57 = buf.st_mode | 0x1C0;
        if ( (buf.st_mode | 0x1C0) != buf.st_mode && (unsigned int)sub_11260((unsigned int)fd, file, v57) )
        {
          v36 = sub_18A0F(4, src);
          v37 = gettext("setting permissions for %s");
          v38 = __errno_location();
          error(0, *v38, v37, v36);
          return 0;
        }
      }
      else
      {
        if ( (buf.st_mode & 0xF000) != 0x4000 )
        {
          v39 = sub_18A0F(4, src);
          v40 = gettext("%s exists but is not a directory");
          error(0, 0, v40, v39);
          return 0;
        }
        *v46 = 0;
      }
      if ( *v46 != 1
        && (*(long long *)(v45 + 40) || *(char *)(v45 + 51))
        && (unsigned char)sub_8750(src, 0, v45) != 1
        && *(char *)(v45 + 52) )
      {
        return 0;
      }
      v41 = i++;
      *v41 = 47;
      while ( *i == 47 )
        ++i;
    }
  }
  else
  {
    if ( (buf.st_mode & 0xF000) != 0x4000 )
    {
      v42 = sub_18A0F(4, v67);
      v43 = gettext("%s exists but is not a directory");
      error(0, 0, v43, v42);
      return 0;
    }
    *v46 = 0;
  }
  return 1;
}



// Function: do_copy @ 0x5522
long long do_copy(int a1, char **a2, char *a3, char a4, long long *a5)
{
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  long long v9; // rbx
  char *v10; // rax
  long long v11; // r12
  char *v12; // rbx
  int *v13; // rax
  long long v14; // rbx
  char *v15; // rax
  size_t v16; // rax
  unsigned long long v17; // rax
  void *v18; // rsp
  char *v19; // rcx
  size_t v20; // rax
  unsigned long long v21; // rax
  void *v22; // rsp
  int v23; // eax
  unsigned char v24; // al
  unsigned char v25; // al
  char *v26; // rax
  long long v27; // rbx
  long long v28; // rbx
  long long v29; // rbx
  long long v30; // rbx
  long long v31; // rbx
  long long *v33; // [rsp+8h] [rbp-170h] BYREF
  char *v34; // [rsp+10h] [rbp-168h]
  char **v35; // [rsp+18h] [rbp-160h]
  char v36; // [rsp+20h] [rbp-158h]
  int v37; // [rsp+24h] [rbp-154h]
  unsigned char v38; // [rsp+34h] [rbp-144h] BYREF
  char v39; // [rsp+35h] [rbp-143h] BYREF
  bool v40; // [rsp+36h] [rbp-142h]
  char v41; // [rsp+37h] [rbp-141h]
  unsigned int v42; // [rsp+38h] [rbp-140h]
  int i; // [rsp+3Ch] [rbp-13Ch]
  unsigned int v44; // [rsp+40h] [rbp-138h]
  int errnum; // [rsp+44h] [rbp-134h]
  __dev_t *v46; // [rsp+48h] [rbp-130h] BYREF
  char *v47; // [rsp+50h] [rbp-128h] BYREF
  void *v48; // [rsp+58h] [rbp-120h]
  char *s2; // [rsp+60h] [rbp-118h]
  char *v50; // [rsp+68h] [rbp-110h]
  char *v51; // [rsp+70h] [rbp-108h]
  char *v52; // [rsp+78h] [rbp-100h]
  char *v53; // [rsp+80h] [rbp-F8h]
  size_t v54; // [rsp+88h] [rbp-F0h]
  void *v55; // [rsp+90h] [rbp-E8h]
  char *s1; // [rsp+98h] [rbp-E0h]
  char *s; // [rsp+A0h] [rbp-D8h]
  size_t n; // [rsp+A8h] [rbp-D0h]
  void *dest; // [rsp+B0h] [rbp-C8h]
  void *v60; // [rsp+B8h] [rbp-C0h]
  void *ptr; // [rsp+C0h] [rbp-B8h]
  struct stat buf; // [rsp+C8h] [rbp-B0h] BYREF
  unsigned long long v63; // [rsp+160h] [rbp-18h]

  v37 = a1;
  v35 = a2;
  v34 = a3;
  v33 = a5;
  v36 = a4;
  v63 = __readfsqword(0x28u);
  v38 = 0;
  v40 = 1;
  if ( a1 <= (a3 == 0) )
  {
    if ( v37 > 0 )
    {
      v6 = sub_18A0F(4, *v35);
      v7 = gettext("missing destination file operand after %s");
      error(0, 0, v7, v6);
    }
    else
    {
      v5 = gettext("missing file operand");
      error(0, 0, v5);
    }
    sub_44B3(1);
  }
  buf.st_mode = 0;
  v42 = -100;
  if ( v36 )
  {
    if ( v34 )
    {
      v8 = gettext("cannot combine --target-directory (-t) and --no-target-directory (-T)");
      error(1, 0, v8);
    }
    if ( v37 > 2 )
    {
      v9 = sub_18A0F(4, v35[2]);
      v10 = gettext("extra operand %s");
      error(0, 0, v10, v9);
      sub_44B3(1);
    }
  }
  else
  {
    if ( v34 )
    {
      v42 = sub_19F57(v34, &buf);
      if ( (unsigned char)sub_19EDF(v42) == 1 )
        goto LABEL_20;
      v11 = sub_18A0F(4, v34);
      v12 = gettext("target directory %s");
      v13 = __errno_location();
      error(1, *v13, v12, v11);
    }
    v50 = v35[v37 - 1];
    v44 = sub_19F57(v50, &buf);
    if ( (unsigned char)sub_19EDF(v44) )
    {
      v42 = v44;
      v34 = v50;
      --v37;
    }
    else
    {
      errnum = *__errno_location();
      if ( errnum == 2 )
        v38 = 1;
      if ( v37 > 2 )
      {
        v14 = sub_18A0F(4, v50);
        v15 = gettext("target %s");
        error(1, errnum, v15, v14);
      }
    }
  }
LABEL_20:
  if ( v34 )
  {
    if ( v37 > 1 )
    {
      sub_A549(v33);
      sub_A5A6(v33);
    }
    for ( i = 0; i < v37; ++i )
    {
      v41 = 1;
      v47 = 0;
      v52 = v35[i];
      if ( byte_27122 )
        sub_116AB(v52);
      if ( byte_27121 )
      {
        s = v52;
        v16 = strlen(v52);
        n = v16 + 1;
        v17 = 16 * ((v16 + 24) / 0x10);
        while ( &v33 != (long long **)((char *)&v33 - (v17 & 0xFFFFFFFFFFFFF000LL)) )
          ;
        v18 = alloca(v17 & 0xFFF);
        if ( (v17 & 0xFFF) != 0 )
          *(long long **)((char *)&v33 + (v17 & 0xFFF) - 8) = *(long long **)((char *)&v33 + (v17 & 0xFFF) - 8);
        dest = &v33;
        v60 = memcpy(&v33, s, n);
        sub_116AB(v60);
        v48 = (void *)sub_11DA5(v34, v60, &v47);
        if ( *((char *)v33 + 60) )
          v19 = "%s -> %s\n";
        else
          v19 = 0;
        v41 = sub_4B50((const char *)v48, v47 - (char *)v48, v42, v19, &v46, &v38, (long long)v33);
        while ( *v47 == 47 )
          ++v47;
      }
      else
      {
        v53 = (char *)sub_10369(v52);
        v20 = strlen(v53);
        v54 = v20 + 1;
        v21 = 16 * ((v20 + 24) / 0x10);
        while ( &v33 != (long long **)((char *)&v33 - (v21 & 0xFFFFFFFFFFFFF000LL)) )
          ;
        v22 = alloca(v21 & 0xFFF);
        if ( (v21 & 0xFFF) != 0 )
          *(long long **)((char *)&v33 + (v21 & 0xFFF) - 8) = *(long long **)((char *)&v33 + (v21 & 0xFFF) - 8);
        v55 = &v33;
        s1 = (char *)memcpy(&v33, v53, v54);
        sub_116AB(s1);
        v23 = strcmp(s1, "..");
        s1 += v23 == 0;
        v48 = (void *)sub_11DA5(v34, s1, &v47);
      }
      if ( v41 != 1 )
      {
        v40 = 0;
      }
      else
      {
        v24 = sub_DAEA((int)v52, (int)v48, v42, (int)v47, v38, (int)v33, (long long)&v39, 0);
        v40 = (v40 & v24) != 0;
        if ( byte_27121 )
        {
          v25 = sub_4793((char *)v48, v42, (long long)v47, (long long)v46, v33);
          v40 = (v40 & v25) != 0;
        }
      }
      if ( byte_27121 )
      {
        while ( v46 )
        {
          ptr = v46;
          v46 = (__dev_t *)v46[20];
          free(ptr);
        }
      }
      free(v48);
    }
  }
  else
  {
    v51 = *v35;
    s2 = v35[1];
    if ( byte_27121 )
    {
      v26 = gettext("with --parents, the destination must be a directory");
      error(0, 0, v26);
      sub_44B3(1);
    }
    if ( *((char *)v33 + 22)
      && *(int *)v33
      && !strcmp(v51, s2)
      && v38 != 1
      && (buf.st_mode || !stat(s2, &buf))
      && (buf.st_mode & 0xF000) == 0x8000 )
    {
      s2 = (char *)sub_1024D(4294967196LL, s2, *(unsigned int *)v33);
      v27 = v33[1];
      qword_27140 = *v33;
      qword_27148 = v27;
      v28 = v33[3];
      qword_27150 = v33[2];
      qword_27158 = v28;
      v29 = v33[5];
      qword_27160 = v33[4];
      qword_27168 = v29;
      v30 = v33[7];
      qword_27170 = v33[6];
      qword_27178 = v30;
      v31 = v33[9];
      qword_27180 = v33[8];
      qword_27188 = v31;
      qword_27190 = v33[10];
      LODWORD(qword_27140) = 0;
      v33 = &qword_27140;
    }
    return (bool)sub_DAEA((int)v51, (int)s2, -100, (int)s2, -v38, (int)v33, (long long)&v47, 0);
  }
  return v40;
}



// Function: cp_option_init @ 0x5f11
long long cp_option_init(long long a1)
{
  sub_DBCB(a1);
  *(char *)(a1 + 20) = 1;
  *(int *)(a1 + 4) = 1;
  *(char *)(a1 + 21) = 0;
  *(char *)(a1 + 22) = 0;
  *(char *)(a1 + 23) = 0;
  *(int *)(a1 + 8) = 4;
  *(char *)(a1 + 24) = 0;
  *(char *)(a1 + 25) = 0;
  *(char *)(a1 + 28) = 0;
  *(int *)(a1 + 68) = 1;
  *(char *)(a1 + 29) = 0;
  *(char *)(a1 + 48) = 0;
  *(char *)(a1 + 30) = 0;
  *(char *)(a1 + 31) = 0;
  *(char *)(a1 + 32) = 0;
  *(char *)(a1 + 51) = 0;
  *(char *)(a1 + 52) = 0;
  *(long long *)(a1 + 40) = 0;
  *(char *)(a1 + 53) = 0;
  *(char *)(a1 + 55) = 0;
  *(char *)(a1 + 54) = 0;
  *(char *)(a1 + 49) = 1;
  *(char *)(a1 + 50) = 0;
  *(char *)(a1 + 56) = 0;
  *(int *)(a1 + 12) = 2;
  *(char *)(a1 + 58) = 0;
  *(char *)(a1 + 57) = 0;
  *(int *)(a1 + 16) = 0;
  *(char *)(a1 + 61) = 0;
  *(char *)(a1 + 59) = 0;
  *(char *)(a1 + 60) = 0;
  *(char *)(a1 + 62) = getenv("POSIXLY_CORRECT") != 0;
  *(long long *)(a1 + 72) = 0;
  *(long long *)(a1 + 80) = 0;
  return a1;
}



// Function: decode_preserve_arg @ 0x606f
void decode_preserve_arg(long long a1, char *a2, char a3)
{
  char *v3; // rax
  const char *v4; // rax
  char *s; // [rsp+28h] [rbp-18h]
  char *v7; // [rsp+30h] [rbp-10h]
  char *ptr; // [rsp+38h] [rbp-8h]

  ptr = (char *)sub_1BF48(a1);
  s = ptr;
  do
  {
    v7 = strchr(s, 44);
    if ( v7 )
    {
      v3 = v7++;
      *v3 = 0;
    }
    if ( a3 )
      v4 = "--preserve";
    else
      v4 = "--no-preserve";
    switch ( sub_F72A((int)v4, (int)s, (unsigned int)off_26840, (unsigned int)&unk_1FB60, 4, (int)off_27028, 1) )
    {
      case 0LL:
      case 7LL:
        a2[30] = a3;
        a2[32] = a3 == 0;
        break;
      case 1LL:
        a2[31] = a3;
        break;
      case 2LL:
        a2[29] = a3;
        break;
      case 3LL:
        a2[48] = a3;
        break;
      case 4LL:
        a2[52] = a3;
        a2[51] = a3;
        break;
      case 5LL:
        a2[53] = a3;
        a2[54] = a3;
        break;
      case 6LL:
        a2[30] = a3;
        a2[31] = a3;
        a2[29] = a3;
        a2[48] = a3;
        a2[32] = a3 == 0;
        if ( byte_27120 )
          a2[51] = a3;
        a2[53] = a3;
        break;
      default:
        abort();
    }
    s = v7;
  }
  while ( v7 );
  free(ptr);
}



// Function: main @ 0x6276
long long main(int a1, char **a2, char **a3)
{
  char *v3; // rax
  char *v4; // rbx
  int *v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  int v11; // eax
  char *v12; // rax
  char *v13; // rax
  long long v14; // r12
  char *v15; // rbx
  int *v16; // rax
  char v18; // [rsp+18h] [rbp-98h]
  char v19; // [rsp+19h] [rbp-97h]
  char v20; // [rsp+1Ah] [rbp-96h]
  int v21; // [rsp+1Ch] [rbp-94h]
  long long v22; // [rsp+20h] [rbp-90h]
  long long v23; // [rsp+28h] [rbp-88h]
  char *v24; // [rsp+30h] [rbp-80h]
  long long v25; // [rsp+38h] [rbp-78h]
  long long v26; // [rsp+40h] [rbp-70h] BYREF
  int v27; // [rsp+48h] [rbp-68h]
  int v28; // [rsp+4Ch] [rbp-64h]
  char v29; // [rsp+54h] [rbp-5Ch]
  char v30; // [rsp+55h] [rbp-5Bh]
  char v31; // [rsp+56h] [rbp-5Ah]
  char v32; // [rsp+57h] [rbp-59h]
  char v33; // [rsp+5Ch] [rbp-54h]
  char v34; // [rsp+5Dh] [rbp-53h]
  char v35; // [rsp+5Eh] [rbp-52h]
  char v36; // [rsp+5Fh] [rbp-51h]
  long long v37; // [rsp+68h] [rbp-48h]
  char v38; // [rsp+70h] [rbp-40h]
  char v39; // [rsp+71h] [rbp-3Fh]
  char v40; // [rsp+72h] [rbp-3Eh]
  char v41; // [rsp+73h] [rbp-3Dh]
  char v42; // [rsp+74h] [rbp-3Ch]
  char v43; // [rsp+75h] [rbp-3Bh]
  char v44; // [rsp+77h] [rbp-39h]
  char v45; // [rsp+78h] [rbp-38h]
  char v46; // [rsp+7Ah] [rbp-36h]
  char v47; // [rsp+7Bh] [rbp-35h]
  char v48; // [rsp+7Ch] [rbp-34h]
  int v49; // [rsp+84h] [rbp-2Ch]
  unsigned long long v50; // [rsp+98h] [rbp-18h]

  v50 = __readfsqword(0x28u);
  v18 = 0;
  v22 = 0;
  v23 = 0;
  v19 = 0;
  v24 = 0;
  v20 = 0;
  v25 = 0;
  sub_16D05(*a2, a2, a3);
  setlocale(6, locale);
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_1D610(sub_11312);
  byte_27120 = (int)is_selinux_enabled() > 0;
  sub_5F11((long long)&v26);
  while ( 2 )
  {
    v21 = getopt_long(a1, a2, "abdfHilLnprst:uvxPRS:TZ", &longopts, 0);
    if ( v21 != -1 )
    {
      if ( v21 <= 136 )
      {
        if ( v21 >= 72 )
        {
          switch ( v21 )
          {
            case 72:
              HIDWORD(v26) = 3;
              continue;
            case 76:
              HIDWORD(v26) = 4;
              continue;
            case 80:
              HIDWORD(v26) = 2;
              continue;
            case 82:
            case 114:
              v45 = 1;
              continue;
            case 83:
              v18 = 1;
              v22 = optarg;
              continue;
            case 84:
              v20 = 1;
              continue;
            case 90:
              if ( byte_27120 )
              {
                if ( optarg )
                {
                  v25 = optarg;
                }
                else
                {
                  v37 = selabel_open(0, 0, 0);
                  if ( !v37 )
                  {
                    v4 = gettext("warning: ignoring --context");
                    v5 = __errno_location();
                    error(0, *v5, v4);
                  }
                }
              }
              else if ( optarg )
              {
                v6 = gettext("warning: ignoring --context; it requires an SELinux-enabled kernel");
                error(0, 0, v6);
              }
              continue;
            case 97:
              HIDWORD(v26) = 2;
              v38 = 1;
              v34 = 1;
              v35 = 1;
              v36 = 1;
              v40 = 1;
              if ( byte_27120 )
                v41 = 1;
              v43 = 1;
              v44 = 1;
              v45 = 1;
              continue;
            case 98:
              v18 = 1;
              if ( optarg )
                v23 = optarg;
              continue;
            case 100:
              v38 = 1;
              HIDWORD(v26) = 2;
              continue;
            case 102:
              v31 = 1;
              continue;
            case 105:
              v27 = 3;
              continue;
            case 108:
              v32 = 1;
              continue;
            case 110:
              v27 = 2;
              continue;
            case 112:
              goto LABEL_30;
            case 115:
              v46 = 1;
              continue;
            case 116:
              if ( v24 )
              {
                v3 = gettext("multiple target directories specified");
                error(1, 0, v3);
              }
              v24 = (char *)optarg;
              continue;
            case 117:
              v47 = 1;
              continue;
            case 118:
              v48 = 1;
              continue;
            case 120:
              v33 = 1;
              continue;
            case 128:
              v39 = 0;
              continue;
            case 129:
              v19 = 1;
              continue;
            case 130:
              sub_606F(optarg, &v26, 0);
              continue;
            case 131:
              byte_27121 = 1;
              continue;
            case 132:
              if ( optarg )
              {
                sub_606F(optarg, &v26, 1);
                v40 = 1;
              }
              else
              {
LABEL_30:
                v34 = 1;
                v35 = 1;
                v36 = 1;
                v40 = 1;
              }
              continue;
            case 133:
              if ( optarg )
                v49 = dword_1E3B0[sub_F72A(
                                    (unsigned int)"--reflink",
                                    optarg,
                                    (unsigned int)off_26460,
                                    (unsigned int)dword_1E3B0,
                                    4,
                                    (int)off_27028,
                                    1)];
              else
                v49 = 2;
              continue;
            case 134:
              v28 = dword_1E3A0[sub_F72A(
                                  (unsigned int)"--sparse",
                                  optarg,
                                  (unsigned int)off_26440,
                                  (unsigned int)dword_1E3A0,
                                  4,
                                  (int)off_27028,
                                  1)];
              continue;
            case 135:
              byte_27122 = 1;
              continue;
            case 136:
              v30 = 1;
              continue;
            default:
              goto LABEL_53;
          }
        }
        if ( v21 == -131 )
        {
          sub_1B6DF(
            (int)stdout,
            (unsigned int)"cp",
            (unsigned int)"GNU coreutils",
            (int)off_27020,
            (unsigned int)"Torbjorn Granlund",
            (unsigned int)"David MacKenzie",
            "Jim Meyering",
            0);
          exit(0);
        }
        if ( v21 == -130 )
          sub_44B3(0);
      }
LABEL_53:
      sub_44B3(1);
    }
    break;
  }
  if ( v32 && v46 )
  {
    v7 = gettext("cannot make both hard and symbolic links");
    error(0, 0, v7);
    sub_44B3(1);
  }
  if ( v27 == 2 )
    v47 = 0;
  if ( v18 && v27 == 2 )
  {
    v8 = gettext("options --backup and --no-clobber are mutually exclusive");
    error(0, 0, v8);
    sub_44B3(1);
  }
  if ( v49 == 2 && v28 != 2 )
  {
    v9 = gettext("--reflink can be used only with --sparse=auto");
    error(0, 0, v9);
    sub_44B3(1);
  }
  if ( v18 )
  {
    v10 = gettext("backup type");
    v11 = sub_1030B(v10, v23);
  }
  else
  {
    v11 = 0;
  }
  LODWORD(v26) = v11;
  sub_F869(v22);
  if ( HIDWORD(v26) == 1 )
  {
    if ( v45 && v32 != 1 )
      HIDWORD(v26) = 2;
    else
      HIDWORD(v26) = 4;
  }
  if ( v45 )
    v29 = v19;
  if ( (v37 || v25) && v42 != 1 )
    v41 = 0;
  if ( v41 && (v37 || v25) )
  {
    v12 = gettext("cannot set target context and preserve it");
    error(1, 0, v12);
  }
  if ( v42 && byte_27120 != 1 )
  {
    v13 = gettext("cannot preserve security context without an SELinux-enabled kernel");
    error(1, 0, v13);
  }
  if ( v25 && (int)setfscreatecon(v25) < 0 )
  {
    v14 = sub_18E56(v25);
    v15 = gettext("failed to set default file creation context to %s");
    v16 = __errno_location();
    error(1, *v16, v15, v14);
  }
  sub_DF3D();
  return (unsigned char)sub_5522(a1 - optind, &a2[optind], v24, v20, &v26) ^ 1u;
}



// Function: follow_fstatat @ 0x6cbb
long long follow_fstatat(int a1, const char *a2, struct stat *a3, int a4)
{
  return (unsigned int)fstatat(a1, a2, a3, a4);
}



// Function: punch_hole @ 0x6d36
long long punch_hole(int a1, __off_t a2, __off_t a3)
{
  int *v3; // rax
  int v5; // [rsp+2Ch] [rbp-4h]

  v5 = fallocate(a1, 3, a2, a3);
  if ( v5 < 0 )
  {
    v3 = __errno_location();
    if ( (unsigned char)sub_6BB9((unsigned int)*v3) || *__errno_location() == 38 )
      return 0;
  }
  return (unsigned int)v5;
}



// Function: create_hole @ 0x6d9e
long long create_hole(int a1, long long a2, char a3, __off_t a4)
{
  long long v4; // r12
  char *v5; // rbx
  int *v6; // rax
  long long v8; // r12
  char *v9; // rbx
  int *v10; // rax
  __off_t v13; // [rsp+28h] [rbp-18h]

  v13 = lseek(a1, a4, 1);
  if ( v13 >= 0 )
  {
    if ( a3 && (int)sub_6D36(a1, v13 - a4, a4) < 0 )
    {
      v8 = sub_18A0F(4, a2);
      v9 = gettext("error deallocating %s");
      v10 = __errno_location();
      error(0, *v10, v9, v8);
      return 0;
    }
    else
    {
      return 1;
    }
  }
  else
  {
    v4 = sub_18A0F(4, a2);
    v5 = gettext("cannot lseek %s");
    v6 = __errno_location();
    error(0, *v6, v5, v4);
    return 0;
  }
}



// Function: sparse_copy @ 0x6ea8
long long sparse_copy(
        unsigned int a1,
        unsigned int a2,
        void **a3,
        size_t a4,
        size_t a5,
        char a6,
        char a7,
        long long a8,
        long long a9,
        size_t nbytes,
        long long *a11,
        char *a12)
{
  size_t v12; // rax
  int *v14; // rax
  long long v15; // r13
  long long v16; // r12
  char *v17; // rbx
  int *v18; // rax
  int v19; // eax
  size_t v20; // rdx
  long long v21; // r12
  char *v22; // rbx
  int *v23; // rax
  size_t v24; // rax
  unsigned long long v25; // rax
  bool v26; // al
  bool v27; // al
  long long v28; // r12
  char *v29; // rbx
  int *v30; // rax
  bool v31; // cl
  long long v32; // rbx
  char *v33; // rax
  char v38; // [rsp+54h] [rbp-6Ch]
  char v39; // [rsp+55h] [rbp-6Bh]
  bool v40; // [rsp+56h] [rbp-6Ah]
  bool v41; // [rsp+57h] [rbp-69h]
  __off_t v42; // [rsp+58h] [rbp-68h]
  long long v43; // [rsp+60h] [rbp-60h]
  ssize_t v44; // [rsp+68h] [rbp-58h]
  unsigned long long v45; // [rsp+70h] [rbp-50h]
  char *v46; // [rsp+78h] [rbp-48h]
  void *v47; // [rsp+80h] [rbp-40h]
  char *buf; // [rsp+90h] [rbp-30h]

  *a12 = 0;
  *a11 = 0;
  if ( !a5 && a7 )
  {
    while ( nbytes )
    {
      v12 = nbytes;
      if ( nbytes >= 0x7FFFFFFFC0000000LL )
        v12 = 0x7FFFFFFFC0000000LL;
      v43 = copy_file_range(a1, 0, a2, 0, v12, 0);
      if ( !v43 )
      {
        if ( !*a11 )
          break;
        return 1;
      }
      if ( v43 < 0 )
      {
        if ( *__errno_location() == 38 )
          break;
        v14 = __errno_location();
        if ( (unsigned char)sub_6BB9((unsigned int)*v14)
          || *__errno_location() == 22
          || *__errno_location() == 9
          || *__errno_location() == 18
          || *__errno_location() == 26
          || *__errno_location() == 1 && !*a11 )
        {
          break;
        }
        if ( *__errno_location() != 4 )
        {
          v15 = sub_18941(1, 4, a9);
          v16 = sub_18941(0, 4, a8);
          v17 = gettext("error copying %s to %s");
          v18 = __errno_location();
          error(0, *v18, v17, v16, v15);
          return 0;
        }
        v43 = 0;
      }
      nbytes -= v43;
      *a11 += v43;
    }
  }
  v38 = 0;
  v42 = 0;
  while ( nbytes )
  {
    if ( !*a3 )
    {
      v19 = getpagesize();
      *a3 = (void *)sub_1B8B1(v19, a4);
    }
    buf = (char *)*a3;
    v20 = a4;
    if ( a4 > nbytes )
      v20 = nbytes;
    v44 = read(a1, buf, v20);
    if ( v44 >= 0 )
    {
      if ( !v44 )
        return !v38 || (unsigned char)sub_6D9E(a2, a9, a6, v42) == 1;
      nbytes -= v44;
      *a11 += v44;
      if ( a5 )
        v24 = a5;
      else
        v24 = a4;
      v45 = v24;
      v46 = buf;
      v47 = buf;
      while ( v44 )
      {
        v39 = v38;
        v25 = v44;
        if ( v45 <= v44 )
          v25 = v45;
        v45 = v25;
        if ( a5 && v25 )
          v38 = sub_6B03(v46, v25);
        v26 = v38 != v39 && v42;
        v40 = v26;
        v27 = v44 == v45 && v38 != 1 || !v45;
        v41 = v27;
        if ( v40 || v27 )
        {
          if ( !v40 )
            v42 += v45;
          if ( v39 != 1 )
          {
            if ( sub_14BFF(a2, v47, v42) != v42 )
            {
              v28 = sub_18A0F(4, a9);
              v29 = gettext("error writing %s");
              v30 = __errno_location();
              error(0, *v30, v29, v28);
              return 0;
            }
          }
          else if ( (unsigned char)sub_6D9E(a2, a9, a6, v42) != 1 )
          {
            return 0;
          }
          v47 = v46;
          v42 = v45;
          if ( v41 )
          {
            if ( !v45 )
              v44 = 0;
            if ( v40 )
              v45 = 0;
            else
              v42 = 0;
          }
        }
        else
        {
          v31 = v42 + v45 + 0x8000000000000000LL < v45;
          v42 += v45;
          if ( v31 )
          {
            v32 = sub_18A0F(4, a8);
            v33 = gettext("overflow reading %s");
            error(0, 0, v33, v32);
            return 0;
          }
        }
        v44 -= v45;
        v46 += v45;
      }
      *a12 = v38;
    }
    else if ( *__errno_location() != 4 )
    {
      v21 = sub_18A0F(4, a8);
      v22 = gettext("error reading %s");
      v23 = __errno_location();
      error(0, *v23, v22, v21);
      return 0;
    }
  }
  return !v38 || (unsigned char)sub_6D9E(a2, a9, a6, v42) == 1;
}



// Function: clone_file @ 0x750c
int clone_file(int a1, unsigned int a2)
{
  return ioctl(a1, 0x40049409u, a2);
}



// Function: write_zeros @ 0x7533
long long write_zeros(unsigned int a1, size_t a2)
{
  size_t v2; // rax
  size_t v5; // [rsp+18h] [rbp-8h]

  if ( !qword_271C0 )
  {
    qword_271C0 = (long long)calloc(nmemb, 1u);
    if ( !qword_271C0 )
    {
      qword_271C0 = (long long)&unk_271E0;
      nmemb = 1024;
    }
  }
  while ( a2 )
  {
    v2 = nmemb;
    if ( a2 <= nmemb )
      v2 = a2;
    v5 = v2;
    if ( v2 != sub_14BFF(a1, qword_271C0, v2) )
      return 0;
    a2 -= v5;
  }
  return 1;
}



// Function: lseek_copy @ 0x75ed
long long lseek_copy(
        unsigned int a1,
        unsigned int a2,
        void **a3,
        size_t a4,
        size_t a5,
        long long a6,
        __off_t length,
        int a8,
        char a9,
        long long a10,
        long long a11)
{
  long long v12; // r12
  char *v13; // rbx
  int *v14; // rax
  size_t v15; // r10
  char v16; // al
  long long v17; // r12
  char *v18; // rbx
  int *v19; // rax
  long long v20; // r12
  char *v21; // rbx
  int *v22; // rax
  long long v23; // r12
  char *v24; // rbx
  int *v25; // rax
  short v30[5]; // [rsp+4Eh] [rbp-52h] BYREF
  long long v31; // [rsp+58h] [rbp-48h]
  __off_t v32; // [rsp+60h] [rbp-40h]
  __off_t v33; // [rsp+68h] [rbp-38h]
  __off_t v34; // [rsp+70h] [rbp-30h]
  long long v35; // [rsp+78h] [rbp-28h]
  size_t nbytes; // [rsp+80h] [rbp-20h]
  unsigned long long v37; // [rsp+88h] [rbp-18h]

  v37 = __readfsqword(0x28u);
  v31 = 0;
  v32 = 0;
  v33 = 0;
  HIBYTE(v30[0]) = 1;
  while ( a6 >= 0 )
  {
    v34 = lseek(a1, a6, 4);
    if ( v34 < 0 )
    {
      if ( *__errno_location() != 6 )
        goto LABEL_42;
      v34 = length;
      if ( length <= a6 )
      {
        length = lseek(a1, 0, 2);
        if ( length < 0 )
          goto LABEL_42;
        if ( length <= a6 )
          break;
        v34 = length;
      }
    }
    if ( length < v34 )
      length = v34;
    if ( lseek(a1, a6, 0) < 0 )
    {
LABEL_42:
      v23 = sub_18A0F(4, a10);
      v24 = gettext("cannot lseek %s");
      v25 = __errno_location();
      error(0, *v25, v24, v23);
      return 0;
    }
    HIBYTE(v30[0]) = 0;
    v35 = a6 - v31 - v32;
    if ( a6 - v31 != v32 )
    {
      if ( a8 == 1 )
      {
        if ( (unsigned char)sub_7533(a2, v35) != 1 )
        {
          v12 = sub_18BA1(0, 3, a11);
          v13 = gettext("%s: write failed");
          v14 = __errno_location();
          error(0, *v14, v13, v12);
          return 0;
        }
      }
      else
      {
        if ( (unsigned char)sub_6D9E(a2, a11, a8 == 3, v35) != 1 )
          return 0;
        HIBYTE(v30[0]) = 1;
      }
    }
    nbytes = v34 - a6;
    v31 = a6;
    v32 = v34 - a6;
    if ( a8 == 1 )
      v15 = 0;
    else
      v15 = a5;
    if ( !sub_6EA8(a1, a2, a3, a4, v15, 1, a9, a10, a11, v34 - a6, &v30[1], v30) )
      return 0;
    v33 = *(long long *)&v30[1] + a6;
    if ( *(long long *)&v30[1] )
      HIBYTE(v30[0]) = v30[0];
    if ( (long long)nbytes > *(long long *)&v30[1] )
    {
      length = v33;
      break;
    }
    a6 = lseek(a1, v33, 3);
    if ( a6 < 0 && *__errno_location() != 6 )
      goto LABEL_42;
  }
  if ( (v33 < length || HIBYTE(v30[0]))
    && (a8 != 1 ? (v16 = ftruncate(a2, length) != 0) : (v16 = sub_7533(a2, length - v33) ^ 1), v16) )
  {
    v17 = sub_18A0F(4, a11);
    v18 = gettext("failed to extend %s");
    v19 = __errno_location();
    error(0, *v19, v18, v17);
    return 0;
  }
  else if ( a8 == 3 && v33 < length && (int)sub_6D36(a2, v33, length - v33) < 0 )
  {
    v20 = sub_18A0F(4, a11);
    v21 = gettext("error deallocating %s");
    v22 = __errno_location();
    error(0, *v22, v21, v20);
    return 0;
  }
  else
  {
    return 1;
  }
}



// Function: is_ancestor @ 0x7a8e
long long is_ancestor(long long *a1, long long *a2)
{
  while ( a2 )
  {
    if ( a2[1] == a1[1] && a2[2] == *a1 )
      return 1;
    a2 = (long long *)*a2;
  }
  return 0;
}



// Function: errno_unsupported @ 0x7ae9
long long errno_unsupported(int a1)
{
  return a1 == 95 || a1 == 61;
}



// Function: copy_attr_error @ 0x7b11
unsigned long long copy_attr_error(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, ...)
{
  int *v6; // rax
  unsigned int v8; // [rsp+1Ch] [rbp-D4h]
  gcc_va_list va; // [rsp+20h] [rbp-D0h] BYREF
  unsigned long long v10; // [rsp+38h] [rbp-B8h]
  long long v11; // [rsp+50h] [rbp-A0h]
  long long v12; // [rsp+58h] [rbp-98h]
  long long v13; // [rsp+60h] [rbp-90h]
  long long v14; // [rsp+68h] [rbp-88h]

  v11 = a3;
  v12 = a4;
  v13 = a5;
  v14 = a6;
  v10 = __readfsqword(0x28u);
  v6 = __errno_location();
  if ( !sub_7AE9(*v6) )
  {
    v8 = *__errno_location();
    va_start(va, a6);
    sub_1AE51(0, v8, a2, va);
  }
  return v10 - __readfsqword(0x28u);
}



// Function: copy_attr_allerror @ 0x7c09
unsigned long long copy_attr_allerror(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, ...)
{
  unsigned int v7; // [rsp+1Ch] [rbp-D4h]
  gcc_va_list va; // [rsp+20h] [rbp-D0h] BYREF
  unsigned long long v9; // [rsp+38h] [rbp-B8h]
  long long v10; // [rsp+50h] [rbp-A0h]
  long long v11; // [rsp+58h] [rbp-98h]
  long long v12; // [rsp+60h] [rbp-90h]
  long long v13; // [rsp+68h] [rbp-88h]

  va_start(va, a6);
  v10 = a3;
  v11 = a4;
  v12 = a5;
  v13 = a6;
  v9 = __readfsqword(0x28u);
  v7 = *__errno_location();
  va[0].gp_offset = 16;
  sub_1AE51(0, v7, a2, va);
  return v9 - __readfsqword(0x28u);
}



// Function: copy_attr_quote @ 0x7cec
long long copy_attr_quote(long long a1, long long a2)
{
  return sub_18A0F(4, a2);
}



// Function: copy_attr_free @ 0x7d13
void copy_attr_free()
{
  ;
}



// Function: check_selinux_attr @ 0x7d26
long long check_selinux_attr(const char *a1, long long a2)
{
  return strncmp(a1, "security.selinux", 0x10u) && (unsigned int)attr_copy_check_permissions(a1, a2);
}



// Function: copy_attr @ 0x7d7e
bool copy_attr(long long a1, int a2, long long a3, int a4, long long a5)
{
  bool v5; // al
  bool v6; // al
  long long (*v7)(const char *, long long); // rax
  unsigned long long (*v8)(long long, long long, long long, long long, long long, long long, ...); // rax
  long long *v9; // rax
  bool v11; // [rsp+2Eh] [rbp-32h]
  bool v12; // [rsp+2Fh] [rbp-31h]
  long long (*v13)(const char *, long long); // [rsp+30h] [rbp-30h]
  long long v14[4]; // [rsp+40h] [rbp-20h] BYREF

  v14[3] = __readfsqword(0x28u);
  v5 = *(char *)(a5 + 49) != 1 || *(char *)(a5 + 54);
  v11 = v5;
  v6 = !v5 && *(char *)(a5 + 55) != 1;
  v12 = v6;
  if ( *(char *)(a5 + 51) || *(long long *)(a5 + 40) )
    v7 = sub_7D26;
  else
    v7 = 0;
  v13 = v7;
  if ( v11 || v12 )
  {
    if ( v11 )
      v8 = sub_7C09;
    else
      v8 = sub_7B11;
    v14[0] = v8;
    v14[1] = sub_7CEC;
    v14[2] = sub_7D13;
    v9 = v14;
  }
  else
  {
    v9 = 0;
  }
  if ( a2 < 0 || a4 < 0 )
    return (unsigned int)attr_copy_file(a1, a3, v13, v9) == 0;
  else
    return (unsigned int)attr_copy_fd(a1, (unsigned int)a2, a3, (unsigned int)a4, v13, v9) == 0;
}



// Function: copy_dir @ 0x7ee6
long long copy_dir(
        long long a1,
        long long a2,
        int a3,
        int a4,
        unsigned char a5,
        int a6,
        long long a7,
        long long a8,
        bool *a9,
        bool *a10)
{
  long long v10; // rbx
  long long v11; // rbx
  long long v12; // rbx
  long long v13; // rbx
  long long v14; // rbx
  long long v15; // r12
  char *v16; // rbx
  int *v17; // rax
  unsigned char v19; // al
  size_t v20; // rax
  char v25; // [rsp+5Bh] [rbp-95h] BYREF
  bool v26; // [rsp+5Ch] [rbp-94h] BYREF
  char v27; // [rsp+5Dh] [rbp-93h] BYREF
  bool v28; // [rsp+5Eh] [rbp-92h]
  bool v29; // [rsp+5Fh] [rbp-91h]
  char *s; // [rsp+60h] [rbp-90h]
  void *v31; // [rsp+68h] [rbp-88h]
  void *v32; // [rsp+70h] [rbp-80h]
  void *ptr; // [rsp+78h] [rbp-78h]
  long long v34[14]; // [rsp+80h] [rbp-70h] BYREF

  v34[11] = __readfsqword(0x28u);
  v10 = *(long long *)(a8 + 8);
  v34[0] = *(long long *)a8;
  v34[1] = v10;
  v11 = *(long long *)(a8 + 24);
  v34[2] = *(long long *)(a8 + 16);
  v34[3] = v11;
  v12 = *(long long *)(a8 + 40);
  v34[4] = *(long long *)(a8 + 32);
  v34[5] = v12;
  v13 = *(long long *)(a8 + 56);
  v34[6] = *(long long *)(a8 + 48);
  v34[7] = v13;
  v14 = *(long long *)(a8 + 72);
  v34[8] = *(long long *)(a8 + 64);
  v34[9] = v14;
  v34[10] = *(long long *)(a8 + 80);
  v28 = 1;
  v31 = (void *)sub_199A1(a1, 2);
  if ( v31 )
  {
    if ( *(int *)(a8 + 4) == 3 )
      HIDWORD(v34[0]) = 2;
    v29 = 0;
    for ( s = (char *)v31; *s; s += v20 + 1 )
    {
      v32 = (void *)sub_11DA5(a1, s, 0);
      ptr = (void *)sub_11DA5(a2, s, 0);
      v26 = *a9;
      v19 = sub_ABDA(
              (int)v32,
              (int)ptr,
              a3,
              a4 - (int)a2 + (int)ptr,
              a5,
              a6,
              a7,
              (long long)v34,
              0,
              (long long)&v26,
              (long long)&v25,
              (long long)&v27);
      v28 = (v28 & v19) != 0;
      *a10 = (unsigned char)(*a10 | v25) != 0;
      free(ptr);
      free(v32);
      if ( v25 )
        break;
      v29 = v29 || v26;
      v20 = strlen(s);
    }
    free(v31);
    *a9 = v29;
    return v28;
  }
  else
  {
    v15 = sub_18A0F(4, a1);
    v16 = gettext("cannot access %s");
    v17 = __errno_location();
    error(0, *v17, v16, v15);
    return 0;
  }
}



// Function: set_owner @ 0x820f
long long set_owner(
        long long a1,
        long long a2,
        unsigned int a3,
        long long a4,
        unsigned int a5,
        int *a6,
        char a7,
        long long a8)
{
  int v8; // eax
  unsigned short v9; // ax
  long long v10; // r12
  char *v11; // rbx
  int *v12; // rax
  long long v14; // r12
  char *v15; // rbx
  int *v16; // rax
  __uid_t owner; // [rsp+3Ch] [rbp-34h]
  __gid_t group; // [rsp+40h] [rbp-30h]
  int v22; // [rsp+44h] [rbp-2Ch]
  unsigned int v23; // [rsp+4Ch] [rbp-24h]
  int v24; // [rsp+50h] [rbp-20h]
  int v25; // [rsp+58h] [rbp-18h]

  owner = a6[7];
  group = a6[8];
  if ( a7 != 1
    && (*(char *)(a1 + 30) || *(char *)(a1 + 24) || *(char *)(a1 + 57))
    && ((v22 = *(int *)(a8 + 24), !*(char *)(a1 + 30)) && !*(char *)(a1 + 24)
      ? (v8 = *(int *)(a1 + 16))
      : (v8 = a6[6]),
        (v23 = (unsigned short)v8 & (unsigned short)v22 & 0x1C0,
         v9 = ~(short)v8,
         HIBYTE(v9) |= 0xEu,
         ((unsigned short)v22 & v9 & 0xFFF) != 0)
     && (unsigned int)sub_16E81(a2, a5, v23)) )
  {
    if ( (unsigned char)sub_DC67(a1) != 1 )
    {
      v10 = sub_18A0F(4, a2);
      v11 = gettext("clearing permissions for %s");
      v12 = __errno_location();
      error(0, *v12, v11, v10);
    }
    return (unsigned int)-*(unsigned char *)(a1 + 50);
  }
  else
  {
    if ( a5 == -1 )
    {
      if ( !(unsigned int)sub_112C3(a3, a4, owner, group) )
        return 1;
      if ( *__errno_location() == 1 || *__errno_location() == 22 )
      {
        v24 = *__errno_location();
        sub_112C3(a3, a4, 0xFFFFFFFFLL, group);
        *__errno_location() = v24;
      }
    }
    else
    {
      if ( !fchown(a5, owner, group) )
        return 1;
      if ( *__errno_location() == 1 || *__errno_location() == 22 )
      {
        v25 = *__errno_location();
        fchown(a5, 0xFFFFFFFF, group);
        *__errno_location() = v25;
      }
    }
    if ( (unsigned char)sub_DC1F(a1) != 1
      && (v14 = sub_18A0F(4, a2),
          v15 = gettext("failed to preserve ownership for %s"),
          v16 = __errno_location(),
          error(0, *v16, v15, v14),
          *(char *)(a1 + 50)) )
    {
      return 0xFFFFFFFFLL;
    }
    else
    {
      return 0;
    }
  }
}



// Function: set_author @ 0x84b0
void set_author()
{
  ;
}



// Function: set_process_security_ctx @ 0x84c6
long long set_process_security_ctx(long long a1, long long a2, unsigned int a3, char a4, long long a5)
{
  bool v5; // al
  bool v6; // al
  int *v7; // rax
  long long v8; // r12
  char *v9; // rbx
  int *v10; // rax
  int *v12; // rax
  long long v13; // r12
  char *v14; // rbx
  int *v15; // rax
  int *v16; // rax
  long long v17; // r12
  char *v18; // rbx
  int *v19; // rax
  bool v21; // [rsp+2Eh] [rbp-22h]
  bool v22; // [rsp+2Fh] [rbp-21h]
  long long v23[4]; // [rsp+30h] [rbp-20h] BYREF

  v23[1] = __readfsqword(0x28u);
  if ( *(char *)(a5 + 51) )
  {
    v5 = *(char *)(a5 + 49) != 1 || *(char *)(a5 + 52);
    v21 = v5;
    v6 = !v5 && *(char *)(a5 + 55) != 1;
    v22 = v6;
    if ( (int)sub_19D8B(a1, v23) < 0 )
    {
      if ( v21 || v22 && (v12 = __errno_location(), !sub_7AE9(*v12)) )
      {
        v13 = sub_18A0F(4, a1);
        v14 = gettext("failed to get security context of %s");
        v15 = __errno_location();
        error(0, *v15, v14, v13);
      }
      if ( *(char *)(a5 + 52) )
        return 0;
    }
    else
    {
      if ( (int)setfscreatecon(v23[0]) < 0 )
      {
        if ( v21 || v22 && (v7 = __errno_location(), !sub_7AE9(*v7)) )
        {
          v8 = sub_18E56(v23[0]);
          v9 = gettext("failed to set default file creation context to %s");
          v10 = __errno_location();
          error(0, *v10, v9, v8);
        }
        if ( *(char *)(a5 + 52) )
        {
          freecon(v23[0]);
          return 0;
        }
      }
      freecon(v23[0]);
    }
  }
  else if ( *(long long *)(a5 + 40) )
  {
    if ( a4 )
    {
      if ( (int)sub_E549(*(long long *)(a5 + 40), a2, a3) < 0 )
      {
        v16 = __errno_location();
        if ( (unsigned char)sub_6C97((unsigned int)*v16) != 1 )
        {
          v17 = sub_18A0F(4, a2);
          v18 = gettext("failed to set default file creation context for %s");
          v19 = __errno_location();
          error(0, *v19, v18, v17);
        }
      }
    }
  }
  return 1;
}



// Function: set_file_security_ctx @ 0x8750
long long set_file_security_ctx(long long a1, unsigned char a2, long long a3)
{
  bool v3; // al
  bool v4; // al
  int *v5; // rax
  long long v6; // r12
  char *v7; // rbx
  int *v8; // rax
  bool v10; // [rsp+2Eh] [rbp-12h]
  bool v11; // [rsp+2Fh] [rbp-11h]

  v3 = *(char *)(a3 + 49) != 1 || *(char *)(a3 + 52);
  v10 = v3;
  v4 = !v3 && *(char *)(a3 + 55) != 1;
  v11 = v4;
  if ( (unsigned char)sub_EAD7(*(long long *)(a3 + 40), a1, a2) == 1 )
    return 1;
  if ( v10 || v11 && (v5 = __errno_location(), !sub_7AE9(*v5)) )
  {
    v6 = sub_18941(0, 4, a1);
    v7 = gettext("failed to set the security context of %s");
    v8 = __errno_location();
    error(0, *v8, v7, v6);
  }
  return 0;
}



// Function: fchmod_or_lchmod @ 0x8868
int fchmod_or_lchmod(int a1, unsigned int a2, long long a3, __mode_t a4)
{
  if ( a1 < 0 )
    return sub_11260(a2, a3, a4);
  else
    return fchmod(a1, a4);
}



// Function: infer_scantype @ 0x88ae
long long infer_scantype(int a1, long long a2, __off_t *a3)
{
  int *v4; // rax

  if ( (*(int *)(a2 + 24) & 0xF000) != 0x8000 || *(long long *)(a2 + 64) >= *(long long *)(a2 + 48) / 512LL )
    return 1;
  *a3 = lseek(a1, 0, 3);
  if ( *a3 >= 0 || *__errno_location() == 6 )
    return 3;
  if ( *__errno_location() == 22 )
    return 2;
  v4 = __errno_location();
  if ( (unsigned char)sub_6BB9((unsigned int)*v4) == 1 )
    return 2;
  return 0;
}



// Function: copy_reg @ 0x896f
long long copy_reg(
        long long a1,
        const char *a2,
        unsigned int a3,
        const char *a4,
        long long a5,
        unsigned int a6,
        int a7,
        char *a8,
        long long *a9)
{
  int v9; // edx
  long long v10; // r12
  char *v11; // rbx
  int *v12; // rax
  struct stat *p_buf; // rsi
  char *v15; // rdi
  int v16; // ecx
  int v17; // r8d
  int v18; // r9d
  long long v19; // r12
  char *v20; // rbx
  int *v21; // rax
  long long v22; // rdx
  long long v23; // rbx
  char *v24; // rax
  int v25; // eax
  long long v26; // r12
  char *v27; // rbx
  int *v28; // rax
  int v29; // edx
  int v30; // eax
  long long v31; // rbx
  char *v32; // rax
  size_t v33; // rax
  long long v34; // rbx
  char *v35; // rax
  long long v36; // r13
  long long v37; // r12
  char *v38; // rbx
  int *v39; // rax
  long long v40; // r12
  char *v41; // rbx
  int *v42; // rax
  __blksize_t st_blksize; // rax
  long long v44; // r12
  char *v45; // rbx
  int *v46; // rax
  bool v47; // al
  int v48; // edx
  int v49; // ecx
  int v50; // r8d
  int v51; // r9d
  long long v52; // rax
  int v53; // edi
  char v54; // al
  size_t v55; // r10
  long long v56; // r12
  char *v57; // rbx
  int *v58; // rax
  long long v59; // rdx
  long long v60; // rdx
  long long v61; // r12
  char *v62; // rbx
  int *v63; // rax
  int v64; // eax
  long long v65; // rsi
  short v66; // ax
  int v67; // eax
  long long v68; // r12
  char *v69; // rbx
  int *v70; // rax
  long long v71; // r12
  char *v72; // rbx
  int *v73; // rax
  long long v74; // r12
  char *v75; // rbx
  int *v76; // rax
  char v83[9]; // [rsp+4Bh] [rbp-1D5h] BYREF
  int errnum; // [rsp+54h] [rbp-1CCh]
  unsigned int v85; // [rsp+58h] [rbp-1C8h]
  int v86; // [rsp+5Ch] [rbp-1C4h]
  int fd; // [rsp+60h] [rbp-1C0h]
  int v88; // [rsp+64h] [rbp-1BCh]
  int v89; // [rsp+68h] [rbp-1B8h]
  int v90; // [rsp+6Ch] [rbp-1B4h]
  __mode_t v91; // [rsp+70h] [rbp-1B0h]
  int v92; // [rsp+74h] [rbp-1ACh]
  int v93[2]; // [rsp+78h] [rbp-1A8h] BYREF
  int v94[2]; // [rsp+80h] [rbp-1A0h] BYREF
  __off_t length; // [rsp+88h] [rbp-198h] BYREF
  int v96[2]; // [rsp+90h] [rbp-190h]
  int v97[2]; // [rsp+98h] [rbp-188h]
  unsigned long long v98; // [rsp+A0h] [rbp-180h]
  unsigned long long v99; // [rsp+A8h] [rbp-178h]
  long long v100[4]; // [rsp+B0h] [rbp-170h] BYREF
  struct stat v101; // [rsp+D0h] [rbp-150h] BYREF
  struct stat buf; // [rsp+160h] [rbp-C0h] BYREF
  char v103; // [rsp+1F7h] [rbp-29h] BYREF
  unsigned long long v104; // [rsp+1F8h] [rbp-28h]
  unsigned int v105; // [rsp+230h] [rbp+10h]

  v104 = __readfsqword(0x28u);
  *(long long *)v93 = 0;
  v85 = *((int *)a9 + 6);
  v83[1] = 1;
  v83[2] = *(char *)(a5 + 49);
  v83[3] = *(char *)(a5 + 53) != 0;
  if ( *(int *)(a5 + 4) == 2 )
    v9 = 0x20000;
  else
    v9 = 0;
  fd = sub_11785(a1, v9, v9, (int)a4, a5, a6);
  if ( fd < 0 )
  {
    v10 = sub_18A0F(4, a1);
    v11 = gettext("cannot open %s for reading");
    v12 = __errno_location();
    error(0, *v12, v11, v10);
    return 0;
  }
  p_buf = &buf;
  LODWORD(v15) = fd;
  if ( !fstat(fd, &buf) )
  {
    if ( a9[1] != buf.st_ino || (v22 = *a9, *a9 != buf.st_dev) )
    {
      v23 = sub_18A0F(4, a1);
      v24 = gettext("skipping file %s, as it was replaced while being copied");
      error(0, 0, v24, v23);
      v83[1] = 0;
      goto LABEL_138;
    }
    if ( *a8 != 1 )
    {
      if ( v83[2] )
        v25 = 513;
      else
        v25 = 1;
      v88 = v25;
      LODWORD(p_buf) = (int)a4;
      LODWORD(v15) = a3;
      *(int *)&v83[5] = sub_16B35(a3, (int)a4, v25, (int)a4, v17, v18);
      errnum = *__errno_location();
      if ( *(int *)&v83[5] >= 0 && (*(long long *)(a5 + 40) || *(char *)(a5 + 51)) )
      {
        LODWORD(p_buf) = 0;
        LODWORD(v15) = (int)a2;
        if ( (unsigned char)sub_8750((long long)a2, 0, a5) != 1 )
        {
          if ( *(char *)(a5 + 52) )
          {
            v83[1] = 0;
            goto LABEL_136;
          }
        }
      }
      if ( *(int *)&v83[5] < 0 && errnum != 2 && *(char *)(a5 + 22) )
      {
        LODWORD(p_buf) = (int)a4;
        LODWORD(v15) = a3;
        if ( unlinkat(a3, a4, 0) )
        {
          if ( *__errno_location() != 2 )
          {
            v26 = sub_18A0F(4, a2);
            v27 = gettext("cannot remove %s");
            v28 = __errno_location();
            error(0, *v28, v27, v26);
            v83[1] = 0;
            goto LABEL_138;
          }
        }
        else if ( *(char *)(a5 + 60) )
        {
          p_buf = (struct stat *)sub_18A0F(4, a2);
          v15 = gettext("removed %s\n");
          printf(v15, p_buf);
        }
        errnum = 2;
      }
      if ( *(int *)&v83[5] < 0 && errnum == 2 )
      {
        if ( *(long long *)(a5 + 40) )
        {
          LODWORD(p_buf) = (int)a2;
          LODWORD(v15) = a1;
          if ( (unsigned char)sub_84C6(a1, (long long)a2, a6, 1, a5) != 1 )
          {
            v83[1] = 0;
            goto LABEL_138;
          }
        }
        *a8 = 1;
      }
    }
    if ( *a8 )
    {
      v29 = a6 & ~a7;
      if ( v83[3] && *(char *)(a5 + 27) != 1 )
        v30 = 128;
      else
        v30 = 0;
      v89 = v29 | v30;
      v86 = (v29 | v30) & ~a6;
      v90 = 65;
      LODWORD(p_buf) = (int)a4;
      LODWORD(v15) = a3;
      *(int *)&v83[5] = sub_16B35(a3, (int)a4, 193, v29 | (unsigned int)v30, v17, v18);
      errnum = *__errno_location();
      if ( *(int *)&v83[5] < 0 && errnum == 17 && *(char *)(a5 + 24) != 1 )
      {
        LODWORD(p_buf) = (int)a4;
        LODWORD(v15) = a3;
        if ( readlinkat(a3, a4, &v103, 1u) >= 0 )
        {
          if ( !*(char *)(a5 + 62) )
          {
            v31 = sub_18A0F(4, a2);
            v32 = gettext("not writing through dangling symlink %s");
            error(0, 0, v32, v31);
            v83[1] = 0;
            goto LABEL_138;
          }
          LODWORD(p_buf) = (int)a4;
          LODWORD(v15) = a3;
          *(int *)&v83[5] = sub_16B35(a3, (int)a4, v90, v89, v17, v18);
          errnum = *__errno_location();
        }
      }
      if ( *(int *)&v83[5] < 0 && errnum == 21 )
      {
        if ( *a2 )
        {
          LODWORD(v15) = (int)a2;
          v33 = strlen(a2);
          LODWORD(v22) = v33 - 1;
          if ( a2[v33 - 1] == 47 )
            errnum = 20;
        }
      }
    }
    else
    {
      v86 = 0;
      a7 = 0;
    }
    if ( *(int *)&v83[5] < 0 )
    {
      v34 = sub_18A0F(4, a2);
      v35 = gettext("cannot create regular file %s");
      error(0, errnum, v35, v34);
      v83[1] = 0;
      goto LABEL_138;
    }
    if ( v83[2] && *(int *)(a5 + 68) )
    {
      LODWORD(p_buf) = fd;
      LODWORD(v15) = *(int *)&v83[5];
      if ( sub_750C(*(int *)&v83[5], fd) )
      {
        if ( *(int *)(a5 + 68) == 2 )
        {
          v36 = sub_18941(1, 4, a1);
          v37 = sub_18941(0, 4, a2);
          v38 = gettext("failed to clone %s from %s");
          v39 = __errno_location();
          error(0, *v39, v38, v37, v36);
          v83[1] = 0;
          goto LABEL_136;
        }
      }
      else
      {
        v83[2] = 0;
      }
    }
    if ( v86 | (unsigned char)(v83[2] | *(char *)(a5 + 29)) )
    {
      p_buf = &v101;
      LODWORD(v15) = *(int *)&v83[5];
      if ( fstat(*(int *)&v83[5], &v101) )
      {
        v40 = sub_18A0F(4, a2);
        v41 = gettext("cannot fstat %s");
        v42 = __errno_location();
        error(0, *v42, v41, v40);
        v83[1] = 0;
        goto LABEL_136;
      }
    }
    else
    {
      v101.st_mode = 0;
    }
    v91 = v86 | v101.st_mode;
    if ( (v86 | v101.st_mode) != v101.st_mode )
    {
      LODWORD(p_buf) = a3;
      LODWORD(v15) = *(int *)&v83[5];
      if ( sub_8868(*(int *)&v83[5], a3, (long long)a4, v91) )
        v86 = 0;
    }
    if ( v83[2] )
    {
      *(long long *)v96 = sub_6BC9(
                         (int)v15,
                         (int)p_buf,
                         v22,
                         v16,
                         v17,
                         v18,
                         v101.st_dev,
                         v101.st_ino,
                         v101.st_nlink,
                         v101.st_mode,
                         v101.st_gid,
                         v101.st_rdev,
                         v101.st_size,
                         v101.st_blksize);
      if ( v101.st_blksize <= 0 || v101.st_blksize > 0x2000000000000000uLL )
        st_blksize = 512;
      else
        st_blksize = v101.st_blksize;
      *(long long *)v97 = st_blksize;
      v92 = sub_88AE(fd, (long long)&buf, (__off_t *)v94);
      if ( !v92 )
      {
        v44 = sub_18A0F(4, a1);
        v45 = gettext("cannot lseek %s");
        v46 = __errno_location();
        error(0, *v46, v45, v44);
        v83[1] = 0;
        goto LABEL_136;
      }
      v47 = (v101.st_mode & 0xF000) == 0x8000 && (*(int *)(a5 + 12) == 3 || *(int *)(a5 + 12) == 2 && v92 != 1);
      v83[4] = v47;
      sub_11711((unsigned int)fd, 0, 0, 2);
      if ( v83[4] != 1 )
      {
        v98 = 0x7FFFFFFFFFFFFFFFLL;
        v52 = sub_6BC9(
                fd,
                0,
                v48,
                v49,
                v50,
                v51,
                buf.st_dev,
                buf.st_ino,
                buf.st_nlink,
                buf.st_mode,
                buf.st_gid,
                buf.st_rdev,
                buf.st_size,
                buf.st_blksize);
        v99 = sub_10429(v52, *(long long *)v96, v98);
        if ( (buf.st_mode & 0xF000) == 0x8000 && *(long long *)v96 > buf.st_size )
          *(long long *)v96 = buf.st_size + 1;
        *(long long *)v96 = v99 + *(long long *)v96 - 1;
        *(long long *)v96 -= *(long long *)v96 % v99;
        if ( !*(long long *)v96 || v98 < *(long long *)v96 )
          *(long long *)v96 = v99;
      }
      v83[0] = 0;
      if ( v92 == 3 )
      {
        if ( v83[4] )
          v53 = *(int *)(a5 + 12);
        else
          v53 = 1;
        v54 = sub_75ED(
                fd,
                *(unsigned int *)&v83[5],
                (void **)v93,
                *(size_t *)v96,
                *(size_t *)v97,
                *(long long *)v94,
                buf.st_size,
                v53,
                *(int *)(a5 + 68) != 0,
                a1,
                (long long)a2)
            ^ 1;
      }
      else
      {
        if ( v83[4] )
          v55 = *(long long *)v97;
        else
          v55 = 0;
        v54 = !sub_6EA8(
                 fd,
                 *(unsigned int *)&v83[5],
                 (void **)v93,
                 *(size_t *)v96,
                 v55,
                 *(int *)(a5 + 12) == 3,
                 *(int *)(a5 + 68) != 0,
                 a1,
                 (long long)a2,
                 0xFFFFFFFFFFFFFFFFLL,
                 &length,
                 v83);
      }
      if ( v54 )
      {
        v83[1] = 0;
        goto LABEL_136;
      }
      if ( v83[0] && ftruncate(*(int *)&v83[5], length) < 0 )
      {
        v56 = sub_18A0F(4, a2);
        v57 = gettext("failed to extend %s");
        v58 = __errno_location();
        error(0, *v58, v57, v56);
        v83[1] = 0;
        goto LABEL_136;
      }
    }
    if ( *(char *)(a5 + 31) )
    {
      v100[0] = sub_19E57(a9);
      v100[1] = v59;
      v100[2] = sub_19E8B(a9);
      v100[3] = v60;
      if ( (unsigned int)sub_118C1(*(unsigned int *)&v83[5], a3, a4, v100, 0) )
      {
        v61 = sub_18A0F(4, a2);
        v62 = gettext("preserving times for %s");
        v63 = __errno_location();
        error(0, *v63, v62, v61);
        if ( *(char *)(a5 + 50) )
        {
          v83[1] = 0;
          goto LABEL_136;
        }
      }
    }
    if ( *(char *)(a5 + 29) && (*((int *)a9 + 7) != v101.st_uid || *((int *)a9 + 8) != v101.st_gid) )
    {
      v64 = sub_820F(a5, (long long)a2, a3, (long long)a4, *(unsigned int *)&v83[5], a9, *a8, (long long)&v101);
      if ( v64 == -1 )
      {
        v83[1] = 0;
LABEL_136:
        if ( close(*(int *)&v83[5]) < 0 )
        {
          v71 = sub_18A0F(4, a2);
          v72 = gettext("failed to close %s");
          v73 = __errno_location();
          error(0, *v73, v72, v71);
          v83[1] = 0;
        }
        goto LABEL_138;
      }
      if ( !v64 )
        v85 &= 0xFFFFF1FF;
    }
    if ( v83[3] && !sub_7D7E(a1, fd, (long long)a2, *(int *)&v83[5], a5) && *(char *)(a5 + 54) )
      v83[1] = 0;
    v65 = *(unsigned int *)&v83[5];
    sub_84B0();
    if ( *(char *)(a5 + 30) || *(char *)(a5 + 24) )
    {
      if ( (unsigned int)sub_EC57(a1, (unsigned int)fd, a2, *(unsigned int *)&v83[5], v85) && *(char *)(a5 + 50) )
        v83[1] = 0;
    }
    else if ( *(char *)(a5 + 57) )
    {
      if ( (unsigned int)sub_ED24(a2, *(unsigned int *)&v83[5], *(unsigned int *)(a5 + 16)) )
        v83[1] = 0;
    }
    else if ( *(char *)(a5 + 32) && *a8 )
    {
      v66 = sub_DCAF(a2, v65);
      if ( (unsigned int)sub_ED24(a2, *(unsigned int *)&v83[5], ~v66 & 0x1B6) )
        v83[1] = 0;
    }
    else if ( v86 | a7 )
    {
      v105 = ~(unsigned int)sub_DCAF(a2, v65) & a7;
      if ( v86 | v105 )
      {
        v67 = sub_DCAF(a2, v65);
        if ( sub_8868(*(int *)&v83[5], a3, (long long)a4, a6 & ~v67) )
        {
          v68 = sub_18A0F(4, a2);
          v69 = gettext("preserving permissions for %s");
          v70 = __errno_location();
          error(0, *v70, v69, v68);
          if ( *(char *)(a5 + 50) )
            v83[1] = 0;
        }
      }
    }
    goto LABEL_136;
  }
  v19 = sub_18A0F(4, a1);
  v20 = gettext("cannot fstat %s");
  v21 = __errno_location();
  error(0, *v21, v20, v19);
  v83[1] = 0;
LABEL_138:
  if ( close(fd) < 0 )
  {
    v74 = sub_18A0F(4, a1);
    v75 = gettext("failed to close %s");
    v76 = __errno_location();
    error(0, *v76, v75, v74);
    v83[1] = 0;
  }
  sub_EDA5(*(long long *)v93);
  return v83[1];
}



// Function: same_file_ok @ 0x9b18
long long same_file_ok(
        const char *a1,
        struct stat *a2,
        unsigned int a3,
        const char *a4,
        struct stat *a5,
        long long a6,
        bool *a7)
{
  bool v7; // al
  bool v9; // al
  __ino_t st_ino; // rbx
  long long v11; // rbx
  __dev_t st_rdev; // rbx
  __blksize_t st_blksize; // rbx
  __time_t tv_sec; // rbx
  __time_t v15; // rbx
  __time_t v16; // rbx
  __syscall_slong_t v17; // rbx
  __syscall_slong_t v18; // rdx
  __ino_t v19; // rbx
  long long v20; // rbx
  __dev_t v21; // rbx
  __blksize_t v22; // rbx
  __time_t v23; // rbx
  __time_t v24; // rbx
  __time_t v25; // rbx
  __syscall_slong_t v26; // rbx
  __syscall_slong_t v27; // rdx
  bool v31; // [rsp+44h] [rbp-15Ch]
  bool v32; // [rsp+45h] [rbp-15Bh]
  char v33; // [rsp+46h] [rbp-15Ah]
  bool v34; // [rsp+47h] [rbp-159h]
  struct stat *v35; // [rsp+48h] [rbp-158h]
  struct stat *p_buf; // [rsp+50h] [rbp-150h]
  char *ptr; // [rsp+58h] [rbp-148h]
  struct stat buf; // [rsp+60h] [rbp-140h] BYREF
  struct stat v39; // [rsp+F0h] [rbp-B0h] BYREF
  unsigned long long v40; // [rsp+188h] [rbp-18h]

  v40 = __readfsqword(0x28u);
  v7 = a2->st_ino == a5->st_ino && a2->st_dev == a5->st_dev;
  v32 = v7;
  *a7 = 0;
  if ( v7 && *(char *)(a6 + 23) )
  {
    *a7 = 1;
    return 1;
  }
  if ( *(int *)(a6 + 4) == 2 )
  {
    v31 = v7;
    if ( (a2->st_mode & 0xF000) == 0xA000 && (a5->st_mode & 0xF000) == 0xA000 )
    {
      v33 = sub_19370(4294967196LL, a1, a3, a4);
      if ( v33 == 1 )
        return v33 == 0;
      if ( *(int *)a6 )
        return 1;
      if ( !v32 )
        return v33 == 0;
      *a7 = 1;
      return *(unsigned char *)(a6 + 24) == 0;
    }
    v35 = a2;
    p_buf = a5;
  }
  else
  {
    if ( !v7 )
      return 1;
    if ( fstatat(a3, a4, &buf, 256) || lstat(a1, &v39) )
      return 1;
    v35 = &v39;
    p_buf = &buf;
    v9 = v39.st_ino == buf.st_ino && v39.st_dev == buf.st_dev;
    v31 = v9;
    if ( (v39.st_mode & 0xF000) == 0xA000 && (buf.st_mode & 0xF000) == 0xA000 && *(char *)(a6 + 21) )
      return 1;
  }
  if ( *(int *)a6 )
  {
    if ( !v31 )
      return *(char *)(a6 + 24) == 1
          || *(int *)(a6 + 4) == 2
          || (v35->st_mode & 0xF000) != 0xA000
          || (p_buf->st_mode & 0xF000) == 0xA000;
    else
      return (unsigned char)sub_19370(4294967196LL, a1, a3, a4) == 0;
  }
  else
  {
    if ( *(char *)(a6 + 24) || *(char *)(a6 + 21) )
    {
      if ( (p_buf->st_mode & 0xF000) == 0xA000 )
        return 1;
      if ( v31 && p_buf->st_nlink > 1 && (unsigned char)sub_19370(4294967196LL, a1, a3, a4) != 1 )
        return *(unsigned char *)(a6 + 24) == 0;
    }
    if ( (v35->st_mode & 0xF000) != 0xA000 && (p_buf->st_mode & 0xF000) != 0xA000 )
    {
      if ( v35->st_ino != p_buf->st_ino || v35->st_dev != p_buf->st_dev )
        return 1;
      if ( *(char *)(a6 + 23) )
      {
        *a7 = 1;
        return 1;
      }
    }
    if ( *(char *)(a6 + 24) )
    {
      if ( (a2->st_mode & 0xF000) == 0xA000 && p_buf->st_nlink > 1 )
      {
        ptr = canonicalize_file_name(a1);
        if ( ptr )
        {
          v34 = (unsigned char)sub_19370(4294967196LL, ptr, a3, a4) == 0;
          free(ptr);
          return v34;
        }
      }
    }
    if ( *(char *)(a6 + 58) && (p_buf->st_mode & 0xF000) == 0xA000 )
      return 1;
    if ( *(int *)(a6 + 4) != 2 )
      return 0;
    if ( (v35->st_mode & 0xF000) == 0xA000 )
    {
      if ( stat(a1, &v39) )
        return 1;
    }
    else
    {
      st_ino = v35->st_ino;
      v39.st_dev = v35->st_dev;
      v39.st_ino = st_ino;
      v11 = *(long long *)&v35->st_mode;
      v39.st_nlink = v35->st_nlink;
      *(long long *)&v39.st_mode = v11;
      st_rdev = v35->st_rdev;
      *(long long *)&v39.st_gid = *(long long *)&v35->st_gid;
      v39.st_rdev = st_rdev;
      st_blksize = v35->st_blksize;
      v39.st_size = v35->st_size;
      v39.st_blksize = st_blksize;
      tv_sec = v35->st_atim.tv_sec;
      v39.st_blocks = v35->st_blocks;
      v39.st_atim.tv_sec = tv_sec;
      v15 = v35->st_mtim.tv_sec;
      v39.st_atim.tv_nsec = v35->st_atim.tv_nsec;
      v39.st_mtim.tv_sec = v15;
      v16 = v35->st_ctim.tv_sec;
      v39.st_mtim.tv_nsec = v35->st_mtim.tv_nsec;
      v39.st_ctim.tv_sec = v16;
      v17 = v35->__unused[0];
      v39.st_ctim.tv_nsec = v35->st_ctim.tv_nsec;
      v39.__unused[0] = v17;
      v18 = v35->__unused[2];
      v39.__unused[1] = v35->__unused[1];
      v39.__unused[2] = v18;
    }
    if ( (p_buf->st_mode & 0xF000) == 0xA000 )
    {
      if ( fstatat(a3, a4, &buf, 0) )
        return 1;
    }
    else
    {
      v19 = p_buf->st_ino;
      buf.st_dev = p_buf->st_dev;
      buf.st_ino = v19;
      v20 = *(long long *)&p_buf->st_mode;
      buf.st_nlink = p_buf->st_nlink;
      *(long long *)&buf.st_mode = v20;
      v21 = p_buf->st_rdev;
      *(long long *)&buf.st_gid = *(long long *)&p_buf->st_gid;
      buf.st_rdev = v21;
      v22 = p_buf->st_blksize;
      buf.st_size = p_buf->st_size;
      buf.st_blksize = v22;
      v23 = p_buf->st_atim.tv_sec;
      buf.st_blocks = p_buf->st_blocks;
      buf.st_atim.tv_sec = v23;
      v24 = p_buf->st_mtim.tv_sec;
      buf.st_atim.tv_nsec = p_buf->st_atim.tv_nsec;
      buf.st_mtim.tv_sec = v24;
      v25 = p_buf->st_ctim.tv_sec;
      buf.st_mtim.tv_nsec = p_buf->st_mtim.tv_nsec;
      buf.st_ctim.tv_sec = v25;
      v26 = p_buf->__unused[0];
      buf.st_ctim.tv_nsec = p_buf->st_ctim.tv_nsec;
      buf.__unused[0] = v26;
      v27 = p_buf->__unused[2];
      buf.__unused[1] = p_buf->__unused[1];
      buf.__unused[2] = v27;
    }
    if ( v39.st_ino != buf.st_ino || v39.st_dev != buf.st_dev )
      return 1;
    if ( *(char *)(a6 + 23) )
    {
      *a7 = (p_buf->st_mode & 0xF000) != 40960;
      return 1;
    }
    else
    {
      return 0;
    }
  }
}



// Function: writable_destination @ 0xa37a
long long writable_destination(int a1, const char *a2, short a3)
{
  return (a3 & 0xF000) == 0xA000 || (unsigned char)sub_1B86C() || !faccessat(a1, a2, 2, 512);
}



// Function: overwrite_ok @ 0xa3d5
long long overwrite_ok(char *a1, long long a2, int a3, const char *a4, long long a5)
{
  long long v5; // r13
  long long v6; // r12
  long long v7; // rbx
  char *v8; // rax
  long long v9; // r12
  long long v10; // rbx
  char *v11; // rax
  char v14; // [rsp+3Ch] [rbp-34h] BYREF
  char v15[11]; // [rsp+3Dh] [rbp-33h] BYREF
  unsigned long long v16; // [rsp+48h] [rbp-28h]

  v16 = __readfsqword(0x28u);
  if ( !sub_A37A(a3, a4, *(int *)(a5 + 24)) )
  {
    sub_11BAD(*(unsigned int *)(a5 + 24), &v14);
    v15[9] = 0;
    v5 = *(int *)(a5 + 24) & 0xFFF;
    v6 = sub_18A0F(4, a2);
    v7 = qword_27608;
    if ( a1[24] || a1[21] || a1[22] )
      v8 = gettext("%s: replace %s, overriding mode %04lo (%s)? ");
    else
      v8 = gettext("%s: unwritable %s (mode %04lo, %s); try anyway? ");
    fprintf(stderr, v8, v7, v6, v5, v15);
  }
  else
  {
    v9 = sub_18A0F(4, a2);
    v10 = qword_27608;
    v11 = gettext("%s: overwrite %s? ");
    fprintf(stderr, v11, v10, v9);
  }
  return sub_1C30F();
}



// Function: dest_info_init @ 0xa549
long long dest_info_init(long long a1)
{
  long long result; // rax

  *(long long *)(a1 + 72) = sub_1563D(61, 0, sub_167C7, sub_1674B, sub_16893);
  result = *(long long *)(a1 + 72);
  if ( !result )
    sub_1BF79();
  return result;
}



// Function: src_info_init @ 0xa5a6
long long src_info_init(long long a1)
{
  long long result; // rax

  *(long long *)(a1 + 80) = sub_1563D(61, 0, sub_1671D, sub_1674B, sub_16893);
  result = *(long long *)(a1 + 80);
  if ( !result )
    sub_1BF79();
  return result;
}



// Function: abandon_move @ 0xa603
long long abandon_move(long long a1, long long a2, int a3, const char *a4, long long a5)
{
  if ( !*(char *)(a1 + 24) )
    __assert_fail("x->move_mode", "src/copy.c", 0x710u, "abandon_move");
  return *(int *)(a1 + 8) == 2
      || (*(int *)(a1 + 8) == 3
       || *(int *)(a1 + 8) == 4 && *(char *)(a1 + 61) && !sub_A37A(a3, a4, *(int *)(a5 + 24)))
      && (unsigned char)sub_A3D5((char *)a1, a2, a3, a4, a5) != 1;
}



// Function: emit_verbose @ 0xa6db
int emit_verbose(long long a1, long long a2, long long a3)
{
  const char *v3; // rbx
  const char *v4; // rax
  long long v5; // rbx
  char *v6; // rax

  v3 = (const char *)sub_18941(1, 4, a2);
  v4 = (const char *)sub_18941(0, 4, a1);
  printf("%s -> %s", v4, v3);
  if ( a3 )
  {
    v5 = sub_18A0F(4, a3);
    v6 = gettext(" (backup: %s)");
    printf(v6, v5);
  }
  return putchar_unlocked(10);
}



// Function: restore_default_fscreatecon_or_die @ 0xa788
void restore_default_fscreatecon_or_die()
{
  char *v0; // rbx
  int *v1; // rax

  if ( (unsigned int)setfscreatecon(0) )
  {
    v0 = gettext("failed to restore the default file creation context");
    v1 = __errno_location();
    error(1, *v1, v0);
  }
}



// Function: subst_suffix @ 0xa7d7
void *subst_suffix(const void *a1, long long a2, const char *a3)
{
  size_t n; // [rsp+30h] [rbp-10h]
  char *dest; // [rsp+38h] [rbp-8h]

  n = strlen(a3) + 1;
  dest = (char *)sub_1B967(a2 - (long long)a1 + n);
  memcpy(&dest[a2 - (long long)a1], a3, n);
  return memcpy(dest, a1, a2 - (long long)a1);
}



// Function: create_hard_link @ 0xa85d
long long create_hard_link(
        void *a1,
        int a2,
        const char *a3,
        const void *a4,
        int a5,
        long long a6,
        unsigned char a7,
        char a8,
        char a9)
{
  int v9; // r8d
  long long v10; // r12
  long long v11; // rbx
  char *v12; // rax
  long long v14; // rbx
  char *v15; // rax
  void *v20; // [rsp+38h] [rbp-28h]
  int errnum; // [rsp+44h] [rbp-1Ch]
  void *ptr; // [rsp+48h] [rbp-18h]

  v20 = a1;
  if ( a9 )
    v9 = 1024;
  else
    v9 = 0;
  errnum = sub_E069(a2, (int)a3, a5, a6, v9, a7, -1);
  if ( errnum <= 0 )
  {
    if ( errnum < 0 && a8 )
    {
      v14 = sub_18A0F(4, a4);
      v15 = gettext("removed %s\n");
      printf(v15, v14);
    }
    return 1;
  }
  else
  {
    ptr = 0;
    if ( !a1 )
    {
      ptr = sub_A7D7(a4, a6, a3);
      v20 = ptr;
    }
    v10 = sub_18941(1, 4, v20);
    v11 = sub_18941(0, 4, a4);
    v12 = gettext("cannot create hard link %s to %s");
    error(0, errnum, v12, v11, v10);
    free(ptr);
    return 0;
  }
}



// Function: should_dereference @ 0xa9cc
long long should_dereference(long long a1, char a2)
{
  return *(int *)(a1 + 4) == 4 || *(int *)(a1 + 4) == 3 && a2;
}



// Function: source_is_dst_backup @ 0xaa08
long long source_is_dst_backup(const char *a1, long long *a2, int a3, const char *a4)
{
  const char *v5; // rbx
  size_t v6; // rax
  int v10; // [rsp+24h] [rbp-DCh]
  size_t v11; // [rsp+28h] [rbp-D8h]
  char *v12; // [rsp+30h] [rbp-D0h]
  size_t n; // [rsp+38h] [rbp-C8h]
  char *file; // [rsp+48h] [rbp-B8h]
  struct stat buf; // [rsp+50h] [rbp-B0h] BYREF
  unsigned long long v16; // [rsp+E8h] [rbp-18h]

  v16 = __readfsqword(0x28u);
  v11 = strlen(a1);
  v12 = (char *)sub_10369(a4);
  n = strlen(v12);
  if ( v11 != n + strlen(s) || memcmp(a1, v12, n) || strcmp(&a1[n], s) )
    return 0;
  v5 = s;
  v6 = strlen(a4);
  file = (char *)sub_A7D7(a4, (long long)&a4[v6], v5);
  v10 = fstatat(a3, file, &buf, 0);
  free(file);
  return !v10 && a2[1] == buf.st_ino && *a2 == buf.st_dev;
}



// Function: copy_internal @ 0xabda
long long copy_internal(
        char *a1,
        void *a2,
        int a3,
        char *a4,
        int a5,
        long long *a6,
        long long *a7,
        char *a8,
        char a9,
        bool *a10,
        bool *a11,
        char *a12)
{
  int v12; // eax
  char v13; // al
  void *v14; // rax
  int v15; // eax
  char *v16; // rax
  int v17; // eax
  long long v18; // r12
  char *v19; // rbx
  int *v20; // rax
  long long v22; // rbx
  char *v23; // rax
  long long v24; // rbx
  char *v25; // rax
  bool v26; // al
  int v27; // eax
  long long v28; // r12
  char *v29; // rbx
  int *v30; // rax
  long long v31; // r12
  long long v32; // rbx
  char *v33; // rax
  int v34; // eax
  long long v35; // r12
  long long v36; // rbx
  char *v37; // rax
  long long v38; // r12
  long long v39; // rbx
  char *v40; // rax
  long long v41; // rbx
  char *v42; // rax
  long long v43; // r12
  long long v44; // rbx
  char *v45; // rax
  char *v46; // rax
  long long v47; // rbx
  long long v48; // rax
  size_t v49; // rax
  void *v50; // rsp
  size_t v51; // rbx
  void *v52; // rax
  long long v53; // r12
  char *v54; // rbx
  int *v55; // rax
  long long v56; // r12
  char *v57; // rbx
  int *v58; // rax
  long long v59; // rbx
  char *v60; // rax
  long long v61; // r12
  long long v62; // rbx
  char *v63; // rax
  long long v64; // r12
  long long v65; // rbx
  char *v66; // rax
  long long v67; // rbx
  char *v68; // rax
  long long v69; // r12
  long long v70; // rbx
  char *v71; // rax
  int v72; // eax
  char *v73; // rax
  long long v74; // r12
  long long v75; // rbx
  char *v76; // rax
  long long v77; // r12
  long long v78; // rbx
  char *v79; // rax
  int v80; // edx
  long long v81; // r13
  long long v82; // r12
  char *v83; // rbx
  int *v84; // rax
  char *v85; // rax
  unsigned int v86; // eax
  int v87; // eax
  long long v88; // rbx
  char *v89; // rax
  void *v90; // rsp
  long long v91; // r12
  char *v92; // rbx
  int *v93; // rax
  long long v94; // r12
  char *v95; // rbx
  int *v96; // rax
  long long v97; // r12
  char *v98; // rbx
  int *v99; // rax
  long long v100; // rbx
  char *v101; // rax
  bool v102; // al
  long long v103; // rbx
  char *v104; // rax
  long long v105; // r12
  long long v106; // rbx
  char *v107; // rax
  bool v108; // al
  __mode_t v109; // eax
  long long v110; // r12
  char *v111; // rbx
  int *v112; // rax
  long long v113; // r12
  char *v114; // rbx
  int *v115; // rax
  long long v116; // r12
  char *v117; // rbx
  int *v118; // rax
  __off_t st_size; // rbx
  long long v120; // rbx
  char *v121; // rax
  char *v122; // rbx
  int *v123; // rax
  long long v124; // rbx
  char *v125; // rax
  __ino_t v126; // rdx
  long long v127; // rdx
  int v128; // eax
  long long v129; // r12
  char *v130; // rbx
  int *v131; // rax
  int v132; // eax
  void *v133; // rdi
  int v134; // eax
  int v135; // eax
  int v136; // eax
  long long v137; // r12
  char *v138; // rbx
  int *v139; // rax
  long long v140; // r12
  char *v141; // rbx
  int *v142; // rax
  long long v143; // r12
  char *v144; // rbx
  int *v145; // rax
  long long v146; // r12
  long long v147; // rbx
  char *v148; // rax
  long long *v149; // [rsp+8h] [rbp-3A0h] BYREF
  char *st_ino; // [rsp+10h] [rbp-398h]
  bool *st_dev; // [rsp+18h] [rbp-390h]
  bool *v152; // [rsp+20h] [rbp-388h]
  char v153; // [rsp+2Ch] [rbp-37Ch]
  char *v154; // [rsp+30h] [rbp-378h]
  long long *v155; // [rsp+38h] [rbp-370h]
  long long *v156; // [rsp+40h] [rbp-368h]
  char *name; // [rsp+48h] [rbp-360h]
  int v158; // [rsp+50h] [rbp-358h]
  int fd; // [rsp+54h] [rbp-354h]
  void *src; // [rsp+58h] [rbp-350h]
  char *old; // [rsp+60h] [rbp-348h]
  bool v162; // [rsp+6Ch] [rbp-33Ch] BYREF
  bool v163; // [rsp+6Dh] [rbp-33Bh] BYREF
  char v164; // [rsp+6Eh] [rbp-33Ah]
  bool v165; // [rsp+6Fh] [rbp-339h]
  char v166; // [rsp+70h] [rbp-338h]
  char v167; // [rsp+71h] [rbp-337h]
  bool v168; // [rsp+72h] [rbp-336h]
  char v169; // [rsp+73h] [rbp-335h]
  char v170; // [rsp+74h] [rbp-334h]
  bool v171; // [rsp+75h] [rbp-333h]
  bool v172; // [rsp+76h] [rbp-332h]
  bool v173; // [rsp+77h] [rbp-331h]
  __mode_t st_mode; // [rsp+78h] [rbp-330h]
  __mode_t v175; // [rsp+7Ch] [rbp-32Ch]
  unsigned int v176; // [rsp+80h] [rbp-328h]
  int errnum; // [rsp+84h] [rbp-324h]
  int v178; // [rsp+88h] [rbp-320h]
  int v179; // [rsp+8Ch] [rbp-31Ch]
  int v180; // [rsp+90h] [rbp-318h]
  int v181; // [rsp+94h] [rbp-314h]
  int v182; // [rsp+98h] [rbp-310h]
  unsigned int v183; // [rsp+9Ch] [rbp-30Ch]
  __mode_t v184; // [rsp+A0h] [rbp-308h]
  __mode_t v185; // [rsp+A4h] [rbp-304h]
  int v186; // [rsp+A8h] [rbp-300h]
  __mode_t mode; // [rsp+ACh] [rbp-2FCh]
  int flags; // [rsp+B0h] [rbp-2F8h]
  int v189; // [rsp+B4h] [rbp-2F4h]
  const char *v190; // [rsp+B8h] [rbp-2F0h]
  void *dest; // [rsp+C0h] [rbp-2E8h]
  struct stat *p_buf; // [rsp+C8h] [rbp-2E0h]
  void *v193; // [rsp+D0h] [rbp-2D8h]
  char *v194; // [rsp+D8h] [rbp-2D0h]
  const char *v195; // [rsp+E0h] [rbp-2C8h]
  char *format; // [rsp+E8h] [rbp-2C0h]
  char *s; // [rsp+F0h] [rbp-2B8h]
  size_t n; // [rsp+F8h] [rbp-2B0h]
  size_t v199; // [rsp+100h] [rbp-2A8h]
  void *ptr; // [rsp+108h] [rbp-2A0h]
  char *v201; // [rsp+110h] [rbp-298h]
  char *s1; // [rsp+118h] [rbp-290h]
  char *s2; // [rsp+120h] [rbp-288h]
  long long **v204; // [rsp+128h] [rbp-280h]
  char *v205; // [rsp+130h] [rbp-278h]
  struct stat v206; // [rsp+138h] [rbp-270h] BYREF
  struct stat v207; // [rsp+1C8h] [rbp-1E0h] BYREF
  struct stat v208; // [rsp+258h] [rbp-150h] BYREF
  struct stat buf; // [rsp+2E8h] [rbp-C0h] BYREF
  unsigned long long v210; // [rsp+380h] [rbp-28h]

  old = a1;
  src = a2;
  fd = a3;
  name = a4;
  v158 = a5;
  v156 = a6;
  v155 = a7;
  v154 = a8;
  v153 = a9;
  v152 = a10;
  st_dev = a11;
  st_ino = a12;
  v210 = __readfsqword(0x28u);
  v164 = 0;
  v190 = 0;
  dest = 0;
  v166 = 0;
  v167 = 0;
  v168 = 0;
  v162 = a5 > 0;
  *a11 = 0;
  errnum = *((int *)v154 + 16);
  if ( v154[24] )
  {
    if ( errnum < 0 )
    {
      if ( (unsigned int)sub_18ECE(4294967196LL, old, (unsigned int)fd, name, 1) )
        v12 = *__errno_location();
      else
        v12 = 0;
      errnum = v12;
    }
    v162 = errnum == 0;
    *st_ino = errnum == 0;
    v158 = (unsigned char)*st_ino;
  }
  if ( errnum )
    v13 = errnum != 17 || *((int *)v154 + 2) != 2;
  else
    v13 = v154[63] ^ 1;
  if ( v13 )
  {
    if ( errnum )
      v14 = old;
    else
      v14 = src;
    v193 = v14;
    if ( errnum )
      v15 = -100;
    else
      v15 = fd;
    v179 = v15;
    if ( errnum )
      v16 = old;
    else
      v16 = name;
    v194 = v16;
    if ( *((int *)v154 + 1) == 2 )
      v17 = 256;
    else
      v17 = 0;
    v180 = v17;
    if ( (unsigned int)sub_6CBB(v179, v194, &v206, v17) )
    {
      v18 = sub_18A0F(4, v193);
      v19 = gettext("cannot stat %s");
      v20 = __errno_location();
      error(0, *v20, v19, v18);
      return 0;
    }
    st_mode = v206.st_mode;
    if ( (v206.st_mode & 0xF000) == 0x4000 && v154[56] != 1 )
    {
      v22 = sub_18A0F(4, old);
      if ( v154[25] != 1 )
        v23 = gettext("-r not specified; omitting directory %s");
      else
        v23 = gettext("omitting directory %s");
      error(0, 0, v23, v22);
      return 0;
    }
  }
  if ( v153 && *((long long *)v154 + 10) )
  {
    if ( (st_mode & 0xF000) != 0x4000
      && !*(int *)v154
      && (unsigned char)sub_11A7B(*((long long *)v154 + 10), old, &v206) )
    {
      v24 = sub_18A0F(4, old);
      v25 = gettext("warning: source file %s specified more than once");
      error(0, 0, v25, v24);
      return 1;
    }
    sub_119DB(*((long long *)v154 + 10), old, &v206);
  }
  v170 = sub_A9CC((long long)v154, v153);
  if ( v158 <= 0 )
  {
    if ( errnum != 17 || *((int *)v154 + 2) != 2 )
    {
      v26 = (st_mode & 0xF000) != 0x8000
         && (v154[20] != 1 || (st_mode & 0xF000) == 0x4000 || (st_mode & 0xF000) == 0xA000)
         || v154[24]
         || v154[58]
         || v154[23]
         || *(int *)v154
         || v154[21];
      v171 = v26;
      if ( v26 )
        v27 = 256;
      else
        v27 = 0;
      v181 = v27;
      if ( !v171 && v158 < 0 )
      {
        v162 = 1;
      }
      else if ( (unsigned int)sub_6CBB(fd, name, &v207, v181) )
      {
        if ( *__errno_location() != 40 || !v154[22] )
        {
          if ( *__errno_location() != 2 )
          {
            v28 = sub_18A0F(4, src);
            v29 = gettext("cannot stat %s");
            v30 = __errno_location();
            error(0, *v30, v29, v28);
            return 0;
          }
          v162 = 1;
        }
      }
      else
      {
        v168 = v171;
        errnum = 17;
      }
    }
    if ( errnum == 17 )
    {
      v163 = 0;
      if ( *((int *)v154 + 2) != 2
        && (unsigned char)sub_9B18(old, &v206, fd, name, &v207, (long long)v154, &v163) != 1 )
      {
        v31 = sub_18941(1, 4, src);
        v32 = sub_18941(0, 4, old);
        v33 = gettext("%s and %s are the same file");
        error(0, 0, v33, v32, v31);
        return 0;
      }
      if ( v154[59] && (st_mode & 0xF000) != 0x4000 )
      {
        v34 = v154[31] && (v154[24] != 1 || v207.st_dev != v206.st_dev);
        v182 = v34;
        if ( (int)sub_1A52E((unsigned int)fd, name, &v207, &v206, v34) >= 0 )
        {
          if ( st_ino )
            *st_ino = 1;
          v190 = (const char *)sub_DE9C(name, v206.st_ino, v206.st_dev);
          if ( !v190 || (unsigned char)sub_A85D(0, fd, v190, src, fd, (long long)name, 1u, v154[60], v170) == 1 )
            return 1;
          goto LABEL_388;
        }
      }
      if ( v154[24] )
      {
        if ( sub_A603((long long)v154, (long long)src, fd, name, (long long)&v207) )
        {
          if ( st_ino )
            *st_ino = 1;
          return 1;
        }
      }
      else if ( (st_mode & 0xF000) != 0x4000
             && (*((int *)v154 + 2) == 2
              || *((int *)v154 + 2) == 3
              && (unsigned char)sub_A3D5(v154, (long long)src, fd, name, (long long)&v207) != 1) )
      {
        return 1;
      }
      if ( v163 )
        return 1;
      if ( (v207.st_mode & 0xF000) != 0x4000 )
      {
        if ( (st_mode & 0xF000) == 0x4000 && (!v154[24] || !*(int *)v154) )
        {
          v35 = sub_18941(1, 4, old);
          v36 = sub_18941(0, 4, src);
          v37 = gettext("cannot overwrite non-directory %s with directory %s");
          error(0, 0, v37, v36, v35);
          return 0;
        }
        if ( v153 && *(int *)v154 != 3 && (unsigned char)sub_11A7B(*((long long *)v154 + 9), name, &v207) )
        {
          v38 = sub_18941(1, 4, old);
          v39 = sub_18941(0, 4, src);
          v40 = gettext("will not overwrite just-created %s with %s");
          error(0, 0, v40, v39, v38);
          return 0;
        }
      }
      if ( (st_mode & 0xF000) != 0x4000 && (v207.st_mode & 0xF000) == 0x4000 && (!v154[24] || !*(int *)v154) )
      {
        v41 = sub_18A0F(4, src);
        v42 = gettext("cannot overwrite directory %s with non-directory");
        error(0, 0, v42, v41);
        return 0;
      }
      if ( v154[24] && (v206.st_mode & 0xF000) == 0x4000 && (v207.st_mode & 0xF000) != 0x4000 && !*(int *)v154 )
      {
        v43 = sub_18BA1(0, 3, src);
        v44 = sub_18BA1(0, 3, old);
        v45 = gettext("cannot move directory onto non-directory: %s -> %s");
        error(0, 0, v45, v44, v43);
        return 0;
      }
      if ( *(int *)v154
        && (v195 = (const char *)sub_10369(old), (unsigned char)sub_6AA4(v195) ^ 1)
        && (v154[24] || (v207.st_mode & 0xF000) != 0x4000) )
      {
        if ( *(int *)v154 != 3 && sub_AA08(v195, &v206, fd, name) )
        {
          if ( v154[24] )
            v46 = gettext("backing up %s might destroy source;  %s not moved");
          else
            v46 = gettext("backing up %s might destroy source;  %s not copied");
          format = v46;
          v47 = sub_18941(1, 4, old);
          v48 = sub_18941(0, 4, src);
          error(0, 0, format, v48, v47);
          return 0;
        }
        s = (char *)sub_1021F((unsigned int)fd, name, *(unsigned int *)v154);
        if ( s )
        {
          n = name - (char *)src;
          v199 = strlen(s) + 1;
          v49 = 16 * ((n + v199 + 23) / 0x10);
          while ( &v149 != (long long **)((char *)&v149 - (v49 & 0xFFFFFFFFFFFFF000LL)) )
            ;
          v50 = alloca(v49 & 0xFFF);
          if ( (v49 & 0xFFF) != 0 )
            *(long long **)((char *)&v149 + (v49 & 0xFFF) - 8) = *(long long **)((char *)&v149 + (v49 & 0xFFF) - 8);
          dest = &v149;
          v51 = v199;
          v52 = mempcpy(&v149, src, n);
          memcpy(v52, s, v51);
          free(s);
        }
        else if ( *__errno_location() != 2 )
        {
          v53 = sub_18A0F(4, src);
          v54 = gettext("cannot backup %s");
          v55 = __errno_location();
          error(0, *v55, v54, v53);
          return 0;
        }
        v162 = 1;
      }
      else if ( (v207.st_mode & 0xF000) != 0x4000
             && v154[24] != 1
             && (v154[21]
              || v154[49]
              && (v154[48] && v207.st_nlink > 1 || *((int *)v154 + 1) == 2 && (v206.st_mode & 0xF000) != 0x8000)) )
      {
        if ( unlinkat(fd, name, 0) && *__errno_location() != 2 )
        {
          v56 = sub_18A0F(4, src);
          v57 = gettext("cannot remove %s");
          v58 = __errno_location();
          error(0, *v58, v57, v56);
          return 0;
        }
        v162 = 1;
        if ( v154[60] )
        {
          v59 = sub_18A0F(4, src);
          v60 = gettext("removed %s\n");
          printf(v60, v59);
        }
      }
    }
  }
  if ( v153 && *((long long *)v154 + 9) && v154[24] != 1 && !*(int *)v154 )
  {
    v169 = 1;
    if ( v168 )
    {
      p_buf = &v207;
    }
    else if ( fstatat(fd, name, &buf, 256) )
    {
      v169 = 0;
    }
    else
    {
      p_buf = &buf;
    }
    if ( v169 && (p_buf->st_mode & 0xF000) == 0xA000 && (unsigned char)sub_11A7B(*((long long *)v154 + 9), name, p_buf) )
    {
      v61 = sub_18941(1, 4, src);
      v62 = sub_18941(0, 4, old);
      v63 = gettext("will not copy %s through just-created symlink %s");
      error(0, 0, v63, v62, v61);
      return 0;
    }
  }
  if ( v154[60] && v154[24] != 1 && (st_mode & 0xF000) != 0x4000 )
    sub_A6DB((long long)old, (long long)src, (long long)dest);
  if ( !errnum )
  {
    v190 = 0;
    goto LABEL_181;
  }
  if ( v154[56] && (st_mode & 0xF000) == 0x4000 )
  {
    if ( !v153 )
    {
LABEL_171:
      v190 = (const char *)sub_DE23(v206.st_ino, v206.st_dev);
      goto LABEL_181;
    }
LABEL_180:
    v190 = (const char *)sub_DE9C(name, v206.st_ino, v206.st_dev);
    goto LABEL_181;
  }
  if ( v154[24] && v206.st_nlink == 1 )
    goto LABEL_171;
  if ( v154[48]
    && v154[23] != 1
    && (v206.st_nlink > 1 || v153 && *((int *)v154 + 1) == 3 || *((int *)v154 + 1) == 4) )
  {
    goto LABEL_180;
  }
LABEL_181:
  if ( v190 )
  {
    if ( (st_mode & 0xF000) != 0x4000 )
    {
      if ( (unsigned char)sub_A85D(0, fd, v190, src, fd, (long long)name, 1u, v154[60], v170) == 1 )
        return 1;
      goto LABEL_388;
    }
    if ( (unsigned char)sub_19370(4294967196LL, old, (unsigned int)fd, v190) )
    {
      v64 = sub_18941(1, 4, qword_271A8);
      v65 = sub_18941(0, 4, qword_271A0);
      v66 = gettext("cannot copy a directory, %s, into itself, %s");
      error(0, 0, v66, v65, v64);
      *st_dev = 1;
LABEL_388:
      if ( v154[51] )
        sub_A788();
      if ( !v190 )
        sub_DDA4(v206.st_ino, v206.st_dev);
      if ( dest )
      {
        v205 = (char *)dest + name - (char *)src;
        if ( renameat(fd, v205, fd, name) )
        {
          v143 = sub_18A0F(4, src);
          v144 = gettext("cannot un-backup %s");
          v145 = __errno_location();
          error(0, *v145, v144, v143);
        }
        else if ( v154[60] )
        {
          v146 = sub_18941(1, 4, src);
          v147 = sub_18941(0, 4, dest);
          v148 = gettext("%s -> %s (unbackup)\n");
          printf(v148, v147, v146);
        }
      }
      return 0;
    }
    if ( (unsigned char)sub_19370((unsigned int)fd, name, (unsigned int)fd, v190) )
    {
      v67 = sub_18A0F(4, qword_271A0);
      v68 = gettext("warning: source directory %s specified more than once");
      error(0, 0, v68, v67);
      if ( v154[24] && st_ino )
        *st_ino = 1;
      return 1;
    }
    if ( *((int *)v154 + 1) != 4 && (!v153 || *((int *)v154 + 1) != 3) )
    {
      ptr = sub_A7D7(src, (long long)name, v190);
      v69 = sub_18941(1, 4, ptr);
      v70 = sub_18941(0, 4, src);
      v71 = gettext("will not create hard link %s to directory %s");
      error(0, 0, v71, v70, v69);
      free(ptr);
      goto LABEL_388;
    }
  }
  if ( v154[24] )
  {
    if ( errnum == 17 )
    {
      if ( renameat(-100, old, fd, name) )
        v72 = *__errno_location();
      else
        v72 = 0;
      errnum = v72;
    }
    if ( !errnum )
    {
      if ( v154[60] )
      {
        v73 = gettext("renamed ");
        printf(v73);
        sub_A6DB((long long)old, (long long)src, (long long)dest);
      }
      if ( *((long long *)v154 + 5) )
        sub_8750((long long)src, 1u, (long long)v154);
      if ( st_ino )
        *st_ino = 1;
      if ( v153 && v154[63] != 1 )
        sub_119DB(*((long long *)v154 + 9), name, &v206);
      return 1;
    }
    if ( errnum == 22 )
    {
      v74 = sub_18941(1, 4, qword_271A8);
      v75 = sub_18941(0, 4, qword_271A0);
      v76 = gettext("cannot move %s to a subdirectory of itself, %s");
      error(0, 0, v76, v75, v74);
      *st_dev = 1;
      return 1;
    }
    if ( errnum != 18 )
    {
      v77 = sub_18941(1, 4, src);
      v78 = sub_18941(0, 4, old);
      v79 = gettext("cannot move %s to %s");
      error(0, errnum, v79, v78, v77);
      sub_DDA4(v206.st_ino, v206.st_dev);
      return 0;
    }
    if ( (st_mode & 0xF000) == 0x4000 )
      v80 = 512;
    else
      v80 = 0;
    if ( unlinkat(fd, name, v80) && *__errno_location() != 2 )
    {
      v81 = sub_18941(1, 4, src);
      v82 = sub_18941(0, 4, old);
      v83 = gettext("inter-device move failed: %s to %s; unable to remove target");
      v84 = __errno_location();
      error(0, *v84, v83, v82, v81);
      sub_DDA4(v206.st_ino, v206.st_dev);
      return 0;
    }
    if ( v154[60] && (st_mode & 0xF000) != 0x4000 )
    {
      v85 = gettext("copied ");
      printf(v85);
      sub_A6DB((long long)old, (long long)src, (long long)dest);
    }
    v162 = 1;
  }
  if ( v154[57] )
    v86 = *((int *)v154 + 4) & 0xFFF;
  else
    v86 = st_mode & 0xFFF;
  v183 = v86;
  if ( v154[29] != 1 )
  {
    if ( (st_mode & 0xF000) == 0x4000 )
      v87 = 18;
    else
      v87 = 0;
  }
  else
  {
    v87 = 63;
  }
  v176 = v183 & v87;
  v165 = 1;
  if ( (unsigned char)sub_84C6((long long)old, (long long)src, st_mode, v162, (long long)v154) != 1 )
    return 0;
  if ( (st_mode & 0xF000) == 0x4000 )
  {
    if ( (unsigned char)sub_7A8E(&v206, v155) )
    {
      v88 = sub_18A0F(4, old);
      v89 = gettext("cannot copy cyclic symbolic link %s");
      error(0, 0, v89, v88);
      goto LABEL_388;
    }
    v90 = alloca(32);
    v204 = &v149;
    v149 = v155;
    st_ino = (char *)v206.st_ino;
    st_dev = (bool *)v206.st_dev;
    if ( !v162 && (v207.st_mode & 0xF000) == 0x4000 )
    {
      v176 = 0;
      if ( (*((long long *)v154 + 5) || v154[51])
        && (unsigned char)sub_8750((long long)src, 0, (long long)v154) != 1
        && v154[52] )
      {
        goto LABEL_388;
      }
    }
    else
    {
      mode = v183 & ~v176;
      if ( mkdirat(fd, name, mode) )
      {
        v91 = sub_18A0F(4, src);
        v92 = gettext("cannot create directory %s");
        v93 = __errno_location();
        error(0, *v93, v92, v91);
        goto LABEL_388;
      }
      if ( fstatat(fd, name, &v207, 256) )
      {
        v94 = sub_18A0F(4, src);
        v95 = gettext("cannot stat %s");
        v96 = __errno_location();
        error(0, *v96, v95, v94);
        goto LABEL_388;
      }
      if ( (v207.st_mode & 0x1C0) != 0x1C0 )
      {
        v175 = v207.st_mode;
        v164 = 1;
        if ( (unsigned int)sub_11260((unsigned int)fd, name, v207.st_mode | 0x1C0) )
        {
          v97 = sub_18A0F(4, src);
          v98 = gettext("setting permissions for %s");
          v99 = __errno_location();
          error(0, *v99, v98, v97);
          goto LABEL_388;
        }
      }
      if ( !*v152 )
      {
        sub_DE9C(name, v207.st_ino, v207.st_dev);
        *v152 = 1;
      }
      if ( v154[60] )
      {
        if ( v154[24] )
        {
          v100 = sub_18A0F(4, src);
          v101 = gettext("created directory %s\n");
          printf(v101, v100);
        }
        else
        {
          sub_A6DB((long long)old, (long long)src, 0);
        }
      }
    }
    if ( !v154[28] || !v156 || *v156 == v206.st_dev )
      v165 = sub_7EE6(
               (long long)old,
               (long long)src,
               fd,
               (int)name,
               v162,
               (int)&v206,
               (long long)v204,
               (long long)v154,
               v152,
               st_dev);
  }
  else if ( v154[58] )
  {
    v167 = 1;
    if ( *old != 47 )
    {
      v102 = (s2 = (char *)sub_11559(name), fd == -100) && !strcmp(".", s2)
          || stat(".", &v208)
          || fstatat(fd, s2, &buf, 0)
          || v208.st_ino == buf.st_ino && v208.st_dev == buf.st_dev;
      v173 = v102;
      free(s2);
      if ( !v173 )
      {
        v103 = sub_18BA1(0, 3, src);
        v104 = gettext("%s: can make relative symbolic links only in current directory");
        error(0, 0, v104, v103);
        goto LABEL_388;
      }
    }
    v186 = sub_E27F(old, (unsigned int)fd, name, (unsigned char)v154[22], 0xFFFFFFFFLL);
    if ( v186 > 0 )
    {
      v105 = sub_18941(1, 4, old);
      v106 = sub_18941(0, 4, src);
      v107 = gettext("cannot create symbolic link %s to %s");
      error(0, v186, v107, v106, v105);
      goto LABEL_388;
    }
  }
  else if ( v154[23] )
  {
    v108 = v154[22] || *((int *)v154 + 2) == 3;
    v172 = v108;
    if ( (unsigned char)sub_A85D(old, -100, old, src, fd, (long long)name, v108, 0, v170) != 1 )
      goto LABEL_388;
  }
  else if ( (st_mode & 0xF000) == 0x8000 || v154[20] && (st_mode & 0xF000) != 0xA000 )
  {
    v166 = 1;
    if ( (unsigned char)sub_896F(
                            (long long)old,
                            (const char *)src,
                            fd,
                            name,
                            (long long)v154,
                            v183 & 0x1FF,
                            v176,
                            (char *)&v162,
                            (long long *)&v206) != 1 )
      goto LABEL_388;
  }
  else
  {
    switch ( st_mode & 0xF000 )
    {
      case 0x1000u:
        v185 = st_mode & ~v176;
        if ( mknodat(fd, name, v185, 0) )
        {
          v109 = v185;
          BYTE1(v109) = BYTE1(v185) & 0xEF;
          if ( mkfifoat(fd, name, v109) )
          {
            v110 = sub_18A0F(4, src);
            v111 = gettext("cannot create fifo %s");
            v112 = __errno_location();
            error(0, *v112, v111, v110);
            goto LABEL_388;
          }
        }
        break;
      case 0x6000u:
      case 0x2000u:
      case 0xC000u:
        v184 = st_mode & ~v176;
        if ( mknodat(fd, name, v184, v206.st_rdev) )
        {
          v113 = sub_18A0F(4, src);
          v114 = gettext("cannot create special file %s");
          v115 = __errno_location();
          error(0, *v115, v114, v113);
          goto LABEL_388;
        }
        break;
      case 0xA000u:
        v201 = (char *)sub_EDED(old, v206.st_size);
        v167 = 1;
        if ( !v201 )
        {
          v116 = sub_18A0F(4, old);
          v117 = gettext("cannot read symbolic link %s");
          v118 = __errno_location();
          error(0, *v118, v117, v116);
          goto LABEL_388;
        }
        v178 = sub_E27F(v201, (unsigned int)fd, name, (unsigned char)v154[22], 0xFFFFFFFFLL);
        if ( v178 > 0 )
        {
          if ( v154[59] )
          {
            if ( !v162 && (v207.st_mode & 0xF000) == 0xA000 )
            {
              st_size = v207.st_size;
              if ( st_size == strlen(v201) )
              {
                s1 = (char *)sub_F0B6((unsigned int)fd, name, v207.st_size);
                if ( s1 )
                {
                  if ( !strcmp(s1, v201) )
                    v178 = 0;
                  free(s1);
                }
              }
            }
          }
        }
        free(v201);
        if ( v178 > 0 )
        {
          v120 = sub_18A0F(4, src);
          v121 = gettext("cannot create symbolic link %s");
          error(0, v178, v121, v120);
          goto LABEL_388;
        }
        if ( v154[51] )
          sub_A788();
        if ( v154[29] )
        {
          if ( (unsigned int)sub_112C3((unsigned int)fd, name, v206.st_uid, v206.st_gid) )
          {
            if ( (unsigned char)sub_DC1F(v154) != 1 )
            {
              v122 = gettext("failed to preserve ownership for %s");
              v123 = __errno_location();
              error(0, *v123, v122, src);
              if ( v154[50] )
                goto LABEL_388;
            }
          }
        }
        break;
      default:
        v124 = sub_18A0F(4, old);
        v125 = gettext("%s has unknown file type");
        error(0, 0, v125, v124);
        goto LABEL_388;
    }
  }
  if ( !v162
    && v154[20] != 1
    && (st_mode & 0xF000) != 0x4000
    && (*((long long *)v154 + 5) || v154[51])
    && (unsigned char)sub_8750((long long)src, 0, (long long)v154) != 1
    && v154[52] )
  {
    goto LABEL_388;
  }
  if ( v153 && *((long long *)v154 + 9) && !fstatat(fd, name, &buf, 256) )
    sub_119DB(*((long long *)v154 + 9), name, &buf);
  if ( v154[23] && (st_mode & 0xF000) != 0x4000 )
    return v165;
  if ( v166 )
    return v165;
  if ( v154[31] )
  {
    buf.st_dev = sub_19E57(&v206);
    buf.st_ino = v126;
    buf.st_nlink = sub_19E8B(&v206);
    *(long long *)&buf.st_mode = v127;
    v128 = v167 ? 256 : 0;
    flags = v128;
    if ( utimensat(fd, name, (const struct timespec *)&buf, v128) )
    {
      v129 = sub_18A0F(4, src);
      v130 = gettext("preserving times for %s");
      v131 = __errno_location();
      error(0, *v131, v130, v129);
      if ( v154[50] )
        return 0;
    }
  }
  if ( v167 != 1 && v154[29] && (v162 || v206.st_uid != v207.st_uid || v206.st_gid != v207.st_gid) )
  {
    v132 = sub_820F((long long)v154, (long long)src, fd, (long long)name, 0xFFFFFFFF, &v206, v162, (long long)&v207);
    if ( v132 == -1 )
      return 0;
    if ( !v132 )
      st_mode &= 0xFFFFF1FF;
  }
  if ( v154[53] && !sub_7D7E((long long)old, -1, (long long)src, -1, (long long)v154) && v154[54] )
    return 0;
  if ( v167 )
    return v165;
  v133 = src;
  sub_84B0();
  if ( v154[30] || v154[24] )
  {
    if ( (unsigned int)sub_EC57(old, 0xFFFFFFFFLL, src, 0xFFFFFFFFLL, st_mode) && v154[50] )
      return 0;
  }
  else if ( v154[57] )
  {
    if ( (unsigned int)sub_ED24(src, 0xFFFFFFFFLL, *((unsigned int *)v154 + 4)) )
      return 0;
  }
  else if ( v154[32] && v162 )
  {
    if ( (st_mode & 0xF000) == 0x4000 || (st_mode & 0xF000) == 0xC000 )
      v134 = 511;
    else
      v134 = 438;
    v189 = v134;
    v135 = sub_DCAF(v133, 0xFFFFFFFFLL);
    if ( (unsigned int)sub_ED24(src, 0xFFFFFFFFLL, v189 & (unsigned int)~v135) )
      return 0;
  }
  else
  {
    if ( v176 )
    {
      v136 = sub_DCAF(v133, 0xFFFFFFFFLL);
      v176 &= ~v136;
      if ( v176 )
      {
        if ( v164 != 1 )
        {
          if ( v162 && fstatat(fd, name, &v207, 256) )
          {
            v137 = sub_18A0F(4, src);
            v138 = gettext("cannot stat %s");
            v139 = __errno_location();
            error(0, *v139, v138, v137);
            return 0;
          }
          v175 = v207.st_mode;
          if ( (v176 & ~v207.st_mode) != 0 )
            v164 = 1;
        }
      }
    }
    if ( v164 )
    {
      if ( (unsigned int)sub_11260((unsigned int)fd, name, v176 | v175) )
      {
        v140 = sub_18A0F(4, src);
        v141 = gettext("preserving permissions for %s");
        v142 = __errno_location();
        error(0, *v142, v141, v140);
        if ( v154[50] )
          return 0;
      }
    }
  }
  return v165;
}



// Function: valid_options @ 0xd983
long long valid_options(long long a1)
{
  if ( *(int *)a1 > 3u )
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)", "src/copy.c", 0xC0Fu, "valid_options");
  if ( *(int *)(a1 + 12) != 1 && *(int *)(a1 + 12) != 2 && *(int *)(a1 + 12) != 3 )
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)", "src/copy.c", 0xC10u, "valid_options");
  if ( *(int *)(a1 + 68) > 2u )
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)", "src/copy.c", 0xC11u, "valid_options");
  if ( *(char *)(a1 + 23) == 1 && *(char *)(a1 + 58) == 1 )
    __assert_fail("!(co->hard_link && co->symbolic_link)", "src/copy.c", 0xC12u, "valid_options");
  if ( *(int *)(a1 + 68) == 2 && *(int *)(a1 + 12) != 2 )
    __assert_fail(
      "! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)",
      "src/copy.c",
      0xC13u,
      "valid_options");
  return 1;
}



// Function: copy @ 0xdaea
long long copy(char *a1, void *a2, int a3, char *a4, int a5, char *a6, bool *a7, char *a8)
{
  bool v13; // [rsp+47h] [rbp-9h] BYREF
  unsigned long long v14; // [rsp+48h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_D983((long long)a6) )
    __assert_fail("valid_options (options)", "src/copy.c", 0xC2Au, "copy");
  qword_271A0 = (long long)a1;
  qword_271A8 = (long long)a2;
  v13 = 0;
  return sub_ABDA(a1, a2, a3, a4, a5, 0, 0, a6, 1, &v13, a7, a8);
}



// Function: cp_options_default @ 0xdbcb
long long cp_options_default(long long a1)
{
  memset((void *)a1, 0, 0x58u);
  *(char *)(a1 + 27) = geteuid() == 0;
  *(char *)(a1 + 26) = *(char *)(a1 + 27);
  *(int *)(a1 + 64) = -1;
  return a1;
}



// Function: chown_failure_ok @ 0xdc1f
long long chown_failure_ok(long long a1)
{
  return (*__errno_location() == 1 || *__errno_location() == 22) && *(char *)(a1 + 26) != 1;
}



// Function: owner_failure_ok @ 0xdc67
long long owner_failure_ok(long long a1)
{
  return (*__errno_location() == 1 || *__errno_location() == 22) && *(char *)(a1 + 27) != 1;
}



// Function: cached_umask @ 0xdcaf
long long cached_umask()
{
  if ( mask == -1 )
  {
    mask = umask(0);
    umask(mask);
  }
  return mask;
}



// Function: src_to_dest_hash @ 0xdce7
unsigned long long src_to_dest_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: src_to_dest_compare @ 0xdd14
long long src_to_dest_compare(long long *a1, long long *a2)
{
  return *a1 == *a2 && a1[1] == a2[1];
}



// Function: src_to_dest_free @ 0xdd6d
void src_to_dest_free(void **a1)
{
  free(a1[2]);
  free(a1);
}



// Function: forget_created @ 0xdda4
unsigned long long forget_created(long long a1, long long a2)
{
  void **v3; // [rsp+18h] [rbp-28h]
  long long v4[3]; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v5; // [rsp+38h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  v4[0] = a1;
  v4[1] = a2;
  v4[2] = 0;
  v3 = (void **)sub_163EE(qword_275E0, v4);
  if ( v3 )
    sub_DD6D(v3);
  return v5 - __readfsqword(0x28u);
}



// Function: src_to_dest_lookup @ 0xde23
long long src_to_dest_lookup(long long a1, long long a2)
{
  long long v3; // [rsp+18h] [rbp-28h]
  long long v4[4]; // [rsp+20h] [rbp-20h] BYREF

  v4[3] = __readfsqword(0x28u);
  v4[0] = a1;
  v4[1] = a2;
  v3 = sub_14F88(qword_275E0, v4);
  if ( v3 )
    return *(long long *)(v3 + 16);
  else
    return 0;
}



// Function: remember_copied @ 0xde9c
long long remember_copied(long long a1, long long a2, long long a3)
{
  long long v5; // [rsp+20h] [rbp-10h]
  long long v6; // [rsp+28h] [rbp-8h]

  v5 = sub_1B941(24);
  *(long long *)(v5 + 16) = sub_1BF48(a1);
  *(long long *)v5 = a2;
  *(long long *)(v5 + 8) = a3;
  v6 = sub_1637E(qword_275E0, v5);
  if ( !v6 )
    sub_1BF79();
  if ( v6 == v5 )
    return 0;
  sub_DD6D((void **)v5);
  return *(long long *)(v6 + 16);
}



// Function: hash_init @ 0xdf3d
long long hash_init()
{
  long long result; // rax

  qword_275E0 = sub_1563D(103, 0, sub_DCE7, sub_DD14, sub_DD6D);
  result = qword_275E0;
  if ( !qword_275E0 )
    sub_1BF79();
  return result;
}



// Function: samedir_template @ 0xdf8a
void *samedir_template(const void *a1, void *a2)
{
  char *v2; // rax
  void *dest; // [rsp+18h] [rbp-18h]
  size_t n; // [rsp+20h] [rbp-10h]

  n = sub_10369(a1) - (long long)a1;
  if ( n + 9 > 0x100 )
  {
    dest = malloc(n + 9);
    if ( !dest )
      return 0;
    v2 = (char *)mempcpy(dest, a1, n);
  }
  else
  {
    dest = a2;
    v2 = (char *)mempcpy(a2, a1, n);
  }
  strcpy(v2, "CuXXXXXX");
  return dest;
}



// Function: try_link @ 0xe021
int try_link(const char *a1, long long a2)
{
  return linkat(*(int *)a2, *(const char **)(a2 + 8), *(int *)(a2 + 16), a1, *(int *)(a2 + 20));
}



// Function: force_linkat @ 0xe069
long long force_linkat(int a1, const char *a2, int a3, const char *a4, int a5, char a6, int a7)
{
  int v7; // eax
  int v9; // eax
  unsigned int v14; // [rsp+24h] [rbp-13Ch]
  char *old; // [rsp+28h] [rbp-138h]
  int v16; // [rsp+30h] [rbp-130h] BYREF
  const char *v17; // [rsp+38h] [rbp-128h]
  int v18; // [rsp+40h] [rbp-120h]
  int v19; // [rsp+44h] [rbp-11Ch]
  char v20[264]; // [rsp+50h] [rbp-110h] BYREF
  unsigned long long v21; // [rsp+158h] [rbp-8h]

  v21 = __readfsqword(0x28u);
  if ( a7 < 0 )
  {
    if ( linkat(a1, a2, a3, a4, a5) )
      v7 = *__errno_location();
    else
      v7 = 0;
    a7 = v7;
  }
  if ( a6 != 1 || a7 != 17 )
    return (unsigned int)a7;
  old = (char *)sub_DF8A(a4, v20);
  if ( !old )
    return (unsigned int)*__errno_location();
  v16 = a1;
  v17 = a2;
  v18 = a3;
  v19 = a5;
  if ( (unsigned int)sub_1A191(old, 0, &v16, sub_E021, 6) )
  {
    v14 = *__errno_location();
  }
  else
  {
    if ( renameat(a3, old, a3, a4) )
      v9 = *__errno_location();
    else
      v9 = -1;
    v14 = v9;
    unlinkat(a3, old, 0);
  }
  if ( old != v20 )
    free(old);
  return v14;
}



// Function: try_symlink @ 0xe245
int try_symlink(const char *a1, long long a2)
{
  return symlinkat(*(const char **)a2, *(int *)(a2 + 8), a1);
}



// Function: force_symlinkat @ 0xe27f
long long force_symlinkat(const char *a1, int a2, const char *a3, char a4, int a5)
{
  int v5; // eax
  unsigned int v7; // [rsp+4h] [rbp-14Ch]
  unsigned int v10; // [rsp+24h] [rbp-12Ch]
  char *old; // [rsp+28h] [rbp-128h]
  const char *v12; // [rsp+30h] [rbp-120h] BYREF
  int v13; // [rsp+38h] [rbp-118h]
  char v14[264]; // [rsp+40h] [rbp-110h] BYREF
  unsigned long long v15; // [rsp+148h] [rbp-8h]

  v7 = a5;
  v15 = __readfsqword(0x28u);
  if ( a5 < 0 )
  {
    if ( symlinkat(a1, a2, a3) )
      v5 = *__errno_location();
    else
      v5 = 0;
    v7 = v5;
  }
  if ( a4 != 1 || v7 != 17 )
    return v7;
  old = (char *)sub_DF8A(a3, v14);
  if ( !old )
    return (unsigned int)*__errno_location();
  v12 = a1;
  v13 = a2;
  if ( (unsigned int)sub_1A191(old, 0, &v12, sub_E245, 6) )
  {
    v10 = *__errno_location();
  }
  else if ( renameat(a2, old, a2, a3) )
  {
    v10 = *__errno_location();
    unlinkat(a2, old, 0);
  }
  else
  {
    v10 = -1;
  }
  if ( old != v14 )
    free(old);
  return v10;
}



// Function: computecon @ 0xe441
long long computecon(long long a1, unsigned int a2, long long a3)
{
  unsigned short v5; // [rsp+26h] [rbp-2Ah]
  unsigned int v6; // [rsp+28h] [rbp-28h]
  int v7; // [rsp+2Ch] [rbp-24h]
  long long v8; // [rsp+30h] [rbp-20h] BYREF
  long long v9; // [rsp+38h] [rbp-18h] BYREF
  void *ptr; // [rsp+40h] [rbp-10h]
  unsigned long long v11; // [rsp+48h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  v8 = 0;
  v9 = 0;
  v6 = -1;
  ptr = (void *)sub_11559(a1);
  if ( ptr )
  {
    if ( (int)getcon(&v8) >= 0 && (int)sub_19D4E(ptr, &v9) >= 0 )
    {
      v5 = mode_to_security_class(a2);
      if ( v5 )
        v6 = security_compute_create(v8, v9, v5, a3);
    }
  }
  v7 = *__errno_location();
  free(ptr);
  freecon(v8);
  freecon(v9);
  *__errno_location() = v7;
  return v6;
}



// Function: defaultcon @ 0xe549
long long defaultcon(long long a1, char *a2, unsigned int a3)
{
  void *v5; // [rsp+10h] [rbp-60h]
  unsigned int v6; // [rsp+28h] [rbp-48h]
  int v7; // [rsp+2Ch] [rbp-44h]
  long long v8; // [rsp+30h] [rbp-40h] BYREF
  long long v9; // [rsp+38h] [rbp-38h] BYREF
  long long v10; // [rsp+40h] [rbp-30h]
  long long v11; // [rsp+48h] [rbp-28h]
  void *ptr; // [rsp+50h] [rbp-20h]
  long long v13; // [rsp+58h] [rbp-18h]
  long long v14; // [rsp+60h] [rbp-10h]
  unsigned long long v15; // [rsp+68h] [rbp-8h]

  v5 = a2;
  v15 = __readfsqword(0x28u);
  v6 = -1;
  v8 = 0;
  v9 = 0;
  v10 = 0;
  v11 = 0;
  ptr = 0;
  if ( *a2 != 47 )
  {
    ptr = (void *)sub_111D3(a2, 2);
    if ( !ptr )
      goto LABEL_14;
    v5 = ptr;
  }
  if ( (int)selabel_lookup(a1, &v8, v5, a3) >= 0 )
  {
    if ( (int)sub_E441((long long)v5, a3, (long long)&v9) >= 0 )
    {
      v10 = context_new(v8);
      if ( v10 )
      {
        v11 = context_new(v9);
        if ( v11 )
        {
          v13 = context_type_get(v10);
          if ( v13 )
          {
            if ( !(unsigned int)context_type_set(v11, v13) )
            {
              v14 = context_str(v11);
              if ( v14 )
                v6 = setfscreatecon(v14);
            }
          }
        }
      }
    }
  }
  else if ( *__errno_location() == 2 )
  {
    *__errno_location() = 61;
  }
LABEL_14:
  v7 = *__errno_location();
  context_free(v10);
  context_free(v11);
  freecon(v8);
  freecon(v9);
  free(ptr);
  *__errno_location() = v7;
  return v6;
}



// Function: restorecon_private @ 0xe72c
long long restorecon_private(long long a1, const char *a2)
{
  unsigned int v3; // [rsp+10h] [rbp-E0h]
  unsigned int fd; // [rsp+14h] [rbp-DCh]
  int v5; // [rsp+18h] [rbp-D8h]
  int v6; // [rsp+1Ch] [rbp-D4h]
  long long v7; // [rsp+20h] [rbp-D0h] BYREF
  long long v8; // [rsp+28h] [rbp-C8h] BYREF
  long long v9; // [rsp+30h] [rbp-C0h]
  long long v10; // [rsp+38h] [rbp-B8h]
  long long v11; // [rsp+40h] [rbp-B0h]
  long long v12; // [rsp+48h] [rbp-A8h]
  struct stat buf; // [rsp+50h] [rbp-A0h] BYREF
  unsigned long long v14; // [rsp+E8h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  v3 = -1;
  v7 = 0;
  v8 = 0;
  v9 = 0;
  v10 = 0;
  if ( !a1 )
  {
    if ( (int)getfscreatecon(&v8) < 0 )
      return 0xFFFFFFFFLL;
    if ( v8 )
    {
      v3 = lsetfilecon(a2, v8);
      v6 = *__errno_location();
      freecon(v8);
      *__errno_location() = v6;
    }
    else
    {
      *__errno_location() = 61;
    }
    return v3;
  }
  fd = open(a2, 0x20000);
  if ( fd != -1 || *__errno_location() == 40 )
  {
    if ( fd == -1 )
    {
      if ( lstat(a2, &buf) < 0 )
        goto LABEL_29;
    }
    else if ( fstat(fd, &buf) < 0 )
    {
      goto LABEL_29;
    }
    if ( (int)selabel_lookup(a1, &v7, a2, buf.st_mode) < 0 )
    {
      if ( *__errno_location() == 2 )
        *__errno_location() = 61;
      goto LABEL_29;
    }
    v9 = context_new(v7);
    if ( v9 )
    {
      if ( fd != -1 )
      {
        if ( (int)sub_19DC8(fd, &v8) < 0 )
          goto LABEL_29;
LABEL_22:
        v10 = context_new(v8);
        if ( v10 )
        {
          v11 = context_type_get(v9);
          if ( v11 )
          {
            if ( !(unsigned int)context_type_set(v10, v11) )
            {
              v12 = context_str(v10);
              if ( v12 )
              {
                if ( fd == -1 )
                  v3 = lsetfilecon(a2, v12);
                else
                  v3 = fsetfilecon(fd, v12);
              }
            }
          }
        }
        goto LABEL_29;
      }
      if ( (int)sub_19D8B(a2, &v8) >= 0 )
        goto LABEL_22;
    }
  }
LABEL_29:
  v5 = *__errno_location();
  if ( fd != -1 )
    close(fd);
  context_free(v9);
  context_free(v10);
  freecon(v7);
  freecon(v8);
  *__errno_location() = v5;
  return v3;
}



// Function: restorecon @ 0xead7
bool restorecon(long long a1, const char *a2, char a3)
{
  long long i; // rax
  const char *v6; // [rsp+10h] [rbp-60h]
  bool v7; // [rsp+2Fh] [rbp-41h]
  int v8; // [rsp+30h] [rbp-40h]
  int v9; // [rsp+34h] [rbp-3Ch]
  char *ptr; // [rsp+38h] [rbp-38h]
  long long v11; // [rsp+40h] [rbp-30h]
  long long v12[4]; // [rsp+50h] [rbp-20h] BYREF

  v6 = a2;
  v12[3] = __readfsqword(0x28u);
  ptr = 0;
  if ( *a2 != 47 )
  {
    ptr = (char *)sub_111D3(a2, 2);
    if ( !ptr )
      return 0;
    v6 = ptr;
  }
  if ( a3 != 1 )
  {
    v7 = (unsigned int)sub_E72C(a1, v6) != -1;
    v9 = *__errno_location();
    free(ptr);
    *__errno_location() = v9;
    return v7;
  }
  else
  {
    v12[0] = v6;
    v12[1] = 0;
    v11 = sub_1BFB9(v12, 16, 0);
    v8 = 0;
    for ( i = sub_12FAC(v11); i; i = sub_12FAC(v11) )
    {
      if ( (int)sub_E72C(a1, *(const char **)(v11 + 32)) < 0 )
        v8 = *__errno_location();
    }
    if ( *__errno_location() )
      v8 = *__errno_location();
    if ( (unsigned int)sub_12AF2(v11) )
      v8 = *__errno_location();
    free(ptr);
    return v8 == 0;
  }
}



// Function: dir_name @ 0x11559
long long dir_name(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = sub_11610(a1);
  if ( !v2 )
    sub_1BF79();
  return v2;
}



// Function: hash_get_n_buckets @ 0x14c84
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0x14c9a
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0x14cb0
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0x14cc6
unsigned long long hash_get_max_bucket_length(long long a1)
{
  long long *i; // [rsp+8h] [rbp-20h]
  unsigned long long v3; // [rsp+10h] [rbp-18h]
  long long *v4; // [rsp+18h] [rbp-10h]
  unsigned long long j; // [rsp+20h] [rbp-8h]

  v3 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      v4 = i;
      for ( j = 1; ; ++j )
      {
        v4 = (long long *)v4[1];
        if ( !v4 )
          break;
      }
      if ( j > v3 )
        v3 = j;
    }
  }
  return v3;
}



// Function: hash_table_ok @ 0x14d48
long long hash_table_ok(long long a1)
{
  long long *i; // [rsp+8h] [rbp-20h]
  long long v3; // [rsp+10h] [rbp-18h]
  long long v4; // [rsp+18h] [rbp-10h]
  long long *v5; // [rsp+20h] [rbp-8h]

  v3 = 0;
  v4 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      v5 = i;
      ++v3;
      ++v4;
      while ( 1 )
      {
        v5 = (long long *)v5[1];
        if ( !v5 )
          break;
        ++v4;
      }
    }
  }
  return v3 == *(long long *)(a1 + 24) && v4 == *(long long *)(a1 + 32);
}



// Function: hash_print_statistics @ 0x14de6
int hash_print_statistics(long long a1, FILE *a2)
{
  double v2; // xmm0_8
  double v3; // xmm1_8
  double v4; // xmm0_8
  long long v6; // [rsp+10h] [rbp-20h]
  long long v7; // [rsp+18h] [rbp-18h]
  long long v8; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v6 = sub_14CB0(a1);
  v7 = sub_14C84(a1);
  v8 = sub_14C9A(a1);
  v9 = sub_14CC6(a1);
  fprintf(a2, "# entries:         %lu\n", v6);
  fprintf(a2, "# buckets:         %lu\n", v7);
  if ( v8 < 0 )
    v2 = (double)(int)(v8 & 1 | ((unsigned long long)v8 >> 1)) + (double)(int)(v8 & 1 | ((unsigned long long)v8 >> 1));
  else
    v2 = (double)(int)v8;
  v3 = 100.0 * v2;
  if ( v7 < 0 )
    v4 = (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1)) + (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1));
  else
    v4 = (double)(int)v7;
  fprintf(a2, "# buckets used:    %lu (%.2f%%)\n", v8, v3 / v4);
  return fprintf(a2, "max bucket length: %lu\n", v9);
}



// Function: safe_hasher @ 0x14f2d
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v3 >= *(long long *)(a1 + 16) )
    abort();
  return *(long long *)a1 + 16 * v3;
}



// Function: hash_lookup @ 0x14f88
long long hash_lookup(long long a1, long long a2)
{
  long long *i; // [rsp+10h] [rbp-10h]
  long long *v4; // [rsp+18h] [rbp-8h]

  v4 = (long long *)sub_14F2D(a1, a2);
  if ( !*v4 )
    return 0;
  for ( i = v4; i; i = (long long *)i[1] )
  {
    if ( a2 == *i || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *i) )
      return *i;
  }
  return 0;
}



// Function: hash_get_first @ 0x1501f
long long hash_get_first(unsigned long long *a1)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  if ( !a1[4] )
    return 0;
  for ( i = *a1; ; i += 16LL )
  {
    if ( i >= a1[1] )
      abort();
    if ( *(long long *)i )
      break;
  }
  return *(long long *)i;
}



// Function: hash_get_next @ 0x1507f
long long hash_get_next(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-10h]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v3 = sub_14F2D(a1, a2);
  v4 = v3;
  do
  {
    if ( a2 == *(long long *)v4 && *(long long *)(v4 + 8) )
      return **(long long **)(v4 + 8);
    v4 = *(long long *)(v4 + 8);
  }
  while ( v4 );
  while ( 1 )
  {
    v3 += 16LL;
    if ( v3 >= *(long long *)(a1 + 8) )
      break;
    if ( *(long long *)v3 )
      return *(long long *)v3;
  }
  return 0;
}



// Function: hash_get_entries @ 0x1511d
unsigned long long hash_get_entries(long long a1, long long a2, unsigned long long a3)
{
  long long v4; // rax
  unsigned long long v5; // [rsp+20h] [rbp-18h]
  long long *i; // [rsp+28h] [rbp-10h]
  long long *j; // [rsp+30h] [rbp-8h]

  v5 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = i; j; j = (long long *)j[1] )
      {
        if ( v5 >= a3 )
          return v5;
        v4 = v5++;
        *(long long *)(a2 + 8 * v4) = *j;
      }
    }
  }
  return v5;
}



// Function: hash_do_for_each @ 0x151bd
long long hash_do_for_each(long long a1, unsigned char (*a2)(long long, long long), long long a3)
{
  long long v5; // [rsp+28h] [rbp-18h]
  long long *i; // [rsp+30h] [rbp-10h]
  long long *j; // [rsp+38h] [rbp-8h]

  v5 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = i; j; j = (long long *)j[1] )
      {
        if ( a2(*j, a3) != 1 )
          return v5;
        ++v5;
      }
    }
  }
  return v5;
}



// Function: hash_string @ 0x15255
unsigned long long hash_string(char *a1, unsigned long long a2)
{
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = 0;
  while ( *a1 )
    v4 = (31 * v4 + (unsigned char)*a1++) % a2;
  return v4;
}



// Function: is_prime @ 0x152af
bool is_prime(unsigned long long a1)
{
  unsigned long long v2; // [rsp+8h] [rbp-10h]
  unsigned long long v3; // [rsp+8h] [rbp-10h]
  unsigned long long v4; // [rsp+10h] [rbp-8h]

  v2 = 3;
  v4 = 9;
  while ( v4 < a1 && a1 % v2 )
  {
    v3 = v2 + 1;
    v4 += 4 * v3;
    v2 = v3 + 1;
  }
  return a1 % v2 != 0;
}



// Function: next_prime @ 0x1531e
unsigned long long next_prime(unsigned long long a1)
{
  long long v2; // [rsp+0h] [rbp-8h]
  unsigned long long i; // [rsp+0h] [rbp-8h]

  v2 = a1;
  if ( a1 <= 9 )
    v2 = 10;
  for ( i = v2 | 1; i != -1 && !sub_152AF(i); i += 2LL )
    ;
  return i;
}



// Function: hash_reset_tuning @ 0x15369
long long hash_reset_tuning(long long a1)
{
  *(int *)a1 = 0;
  *(int *)(a1 + 4) = 1065353216;
  *(int *)(a1 + 8) = 1061997773;
  *(int *)(a1 + 12) = 1068826100;
  *(char *)(a1 + 16) = 0;
  return a1;
}



// Function: raw_hasher @ 0x153bf
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_1C5BC(a1, 3) % a2;
}



// Function: raw_comparator @ 0x153fa
bool raw_comparator(long long a1, long long a2)
{
  return a1 == a2;
}



// Function: check_tuning @ 0x15417
long long check_tuning(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v2 == &unk_20780 )
    return 1;
  if ( *(float *)(v2 + 8) > 0.1
    && (float)(1.0 - 0.1) > *(float *)(v2 + 8)
    && *(float *)(v2 + 12) > (float)(0.1 + 1.0)
    && *(float *)v2 >= 0.0
    && *(float *)(v2 + 4) > (float)(*(float *)v2 + 0.1)
    && *(float *)(v2 + 4) <= 1.0
    && *(float *)(v2 + 8) > (float)(*(float *)v2 + 0.1) )
  {
    return 1;
  }
  *(long long *)(a1 + 40) = &unk_20780;
  return 0;
}



// Function: compute_bucket_size @ 0x15524
unsigned long long compute_bucket_size(long long a1, long long a2)
{
  float v2; // xmm0_4
  bool v4; // al
  unsigned long long v5; // [rsp+8h] [rbp-18h]
  unsigned long long v6; // [rsp+8h] [rbp-18h]
  float v7; // [rsp+1Ch] [rbp-4h]

  v5 = a1;
  if ( *(char *)(a2 + 16) != 1 )
  {
    if ( a1 < 0 )
      v2 = (float)(a1 & 1 | (unsigned int)((unsigned long long)a1 >> 1))
         + (float)(a1 & 1 | (unsigned int)((unsigned long long)a1 >> 1));
    else
      v2 = (float)(int)a1;
    v7 = v2 / *(float *)(a2 + 8);
    if ( v7 >= 1.8446744e19 )
      return 0;
    if ( v7 >= 9.223372e18 )
      v5 = (unsigned int)(int)(float)(v7 - 9.223372e18) ^ 0x8000000000000000LL;
    else
      v5 = (unsigned int)(int)v7;
  }
  v6 = sub_1531E(v5);
  v4 = v6 >> 61 != 0;
  if ( (v6 & 0x1000000000000000LL) != 0 )
    v4 = 1;
  if ( v4 )
    return 0;
  else
    return v6;
}



// Function: hash_initialize @ 0x1563d
size_t *hash_initialize(
        long long a1,
        void *a2,
        unsigned long long (*a3)(long long a1, unsigned long long a2),
        bool (*a4)(long long a1, long long a2),
        size_t a5)
{
  bool (*v7)(long long, long long); // [rsp+10h] [rbp-30h]
  unsigned long long (*v8)(long long, unsigned long long); // [rsp+18h] [rbp-28h]
  void *v9; // [rsp+20h] [rbp-20h]
  size_t *ptr; // [rsp+38h] [rbp-8h]

  v9 = a2;
  v8 = a3;
  v7 = a4;
  if ( !a3 )
    v8 = sub_153BF;
  if ( !a4 )
    v7 = sub_153FA;
  ptr = (size_t *)malloc(0x50u);
  if ( !ptr )
    return 0;
  if ( !a2 )
    v9 = &unk_20780;
  ptr[5] = (size_t)v9;
  if ( (unsigned char)sub_15417((long long)ptr) == 1
    && (ptr[2] = sub_15524(a1, (long long)v9)) != 0
    && (*ptr = (size_t)calloc(ptr[2], 0x10u)) != 0 )
  {
    ptr[1] = 16 * ptr[2] + *ptr;
    ptr[3] = 0;
    ptr[4] = 0;
    ptr[6] = (size_t)v8;
    ptr[7] = (size_t)v7;
    ptr[8] = a5;
    ptr[9] = 0;
    return ptr;
  }
  else
  {
    free(ptr);
    return 0;
  }
}



// Function: hash_clear @ 0x157b2
long long hash_clear(long long a1)
{
  long long *i; // [rsp+18h] [rbp-18h]
  long long *j; // [rsp+20h] [rbp-10h]
  long long *v4; // [rsp+28h] [rbp-8h]

  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = (long long *)i[1]; j; j = v4 )
      {
        if ( *(long long *)(a1 + 64) )
          (*(void (**)(long long))(a1 + 64))(*j);
        *j = 0;
        v4 = (long long *)j[1];
        j[1] = *(long long *)(a1 + 72);
        *(long long *)(a1 + 72) = j;
      }
      if ( *(long long *)(a1 + 64) )
        (*(void (**)(long long))(a1 + 64))(*i);
      *i = 0;
      i[1] = 0;
    }
  }
  *(long long *)(a1 + 24) = 0;
  *(long long *)(a1 + 32) = 0;
  return a1;
}



// Function: hash_free @ 0x158bd
void hash_free(long long a1)
{
  long long *i; // [rsp+18h] [rbp-18h]
  long long *j; // [rsp+18h] [rbp-18h]
  long long *ptr; // [rsp+20h] [rbp-10h]
  long long *ptra; // [rsp+20h] [rbp-10h]
  long long *ptrb; // [rsp+20h] [rbp-10h]
  long long *v6; // [rsp+28h] [rbp-8h]
  long long *v7; // [rsp+28h] [rbp-8h]

  if ( *(long long *)(a1 + 64) && *(long long *)(a1 + 32) )
  {
    for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
    {
      if ( *i )
      {
        for ( ptr = i; ptr; ptr = (long long *)ptr[1] )
          (*(void (**)(long long))(a1 + 64))(*ptr);
      }
    }
  }
  for ( j = *(long long **)a1; (unsigned long long)j < *(long long *)(a1 + 8); j += 2 )
  {
    for ( ptra = (long long *)j[1]; ptra; ptra = v6 )
    {
      v6 = (long long *)ptra[1];
      free(ptra);
    }
  }
  for ( ptrb = *(long long **)(a1 + 72); ptrb; ptrb = v7 )
  {
    v7 = (long long *)ptrb[1];
    free(ptrb);
  }
  free(*(void **)a1);
  free((void *)a1);
}



// Function: allocate_entry @ 0x159ec
void *allocate_entry(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  if ( !*(long long *)(a1 + 72) )
    return malloc(0x10u);
  v2 = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = *(long long *)(v2 + 8);
  return (void *)v2;
}



// Function: free_entry @ 0x15a3b
long long free_entry(long long a1, long long *a2)
{
  *a2 = 0;
  a2[1] = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = a2;
  return a1;
}



// Function: hash_find_entry @ 0x15a75
long long hash_find_entry(long long a1, long long a2, long long *a3, char a4)
{
  long long v5; // rdx
  long long *i; // [rsp+20h] [rbp-30h]
  long long *v9; // [rsp+28h] [rbp-28h]
  long long v10; // [rsp+30h] [rbp-20h]
  long long *v11; // [rsp+38h] [rbp-18h]
  long long v12; // [rsp+40h] [rbp-10h]
  long long *v13; // [rsp+48h] [rbp-8h]

  v9 = (long long *)sub_14F2D(a1, a2);
  *a3 = v9;
  if ( !*v9 )
    return 0;
  if ( a2 == *v9 || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *v9) )
  {
    v12 = *v9;
    if ( a4 )
    {
      if ( v9[1] )
      {
        v13 = (long long *)v9[1];
        v5 = v13[1];
        *v9 = *v13;
        v9[1] = v5;
        sub_15A3B(a1, v13);
      }
      else
      {
        *v9 = 0;
      }
    }
    return v12;
  }
  else
  {
    for ( i = v9; ; i = (long long *)i[1] )
    {
      if ( !i[1] )
        return 0;
      if ( a2 == *(long long *)i[1] || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *(long long *)i[1]) )
        break;
    }
    v10 = *(long long *)i[1];
    if ( a4 )
    {
      v11 = (long long *)i[1];
      i[1] = v11[1];
      sub_15A3B(a1, v11);
    }
    return v10;
  }
}



// Function: transfer_entries @ 0x15c0e
long long transfer_entries(long long a1, long long a2, char a3)
{
  long long *i; // [rsp+20h] [rbp-30h]
  long long *j; // [rsp+28h] [rbp-28h]
  long long v7; // [rsp+30h] [rbp-20h]
  long long v8; // [rsp+30h] [rbp-20h]
  long long *v9; // [rsp+38h] [rbp-18h]
  long long *v10; // [rsp+38h] [rbp-18h]
  long long *v11; // [rsp+40h] [rbp-10h]
  long long *v12; // [rsp+48h] [rbp-8h]

  for ( i = *(long long **)a2; (unsigned long long)i < *(long long *)(a2 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = (long long *)i[1]; j; j = v12 )
      {
        v7 = *j;
        v9 = (long long *)sub_14F2D(a1, *j);
        v12 = (long long *)j[1];
        if ( *v9 )
        {
          j[1] = v9[1];
          v9[1] = j;
        }
        else
        {
          *v9 = v7;
          ++*(long long *)(a1 + 24);
          sub_15A3B(a1, j);
        }
      }
      v8 = *i;
      i[1] = 0;
      if ( !a3 )
      {
        v10 = (long long *)sub_14F2D(a1, v8);
        if ( *v10 )
        {
          v11 = sub_159EC(a1);
          if ( !v11 )
            return 0;
          *v11 = v8;
          v11[1] = v10[1];
          v10[1] = v11;
        }
        else
        {
          *v10 = v8;
          ++*(long long *)(a1 + 24);
        }
        *i = 0;
        --*(long long *)(a2 + 24);
      }
    }
  }
  return 1;
}



// Function: hash_rehash @ 0x15de2
long long hash_rehash(long long a1, long long a2)
{
  unsigned long long nmemb; // [rsp+10h] [rbp-70h]
  char *v4; // [rsp+20h] [rbp-60h] BYREF
  char *v5; // [rsp+28h] [rbp-58h]
  unsigned long long v6; // [rsp+30h] [rbp-50h]
  long long v7; // [rsp+38h] [rbp-48h]
  long long v8; // [rsp+40h] [rbp-40h]
  long long v9; // [rsp+48h] [rbp-38h]
  long long v10; // [rsp+50h] [rbp-30h]
  long long v11; // [rsp+58h] [rbp-28h]
  long long v12; // [rsp+60h] [rbp-20h]
  long long v13; // [rsp+68h] [rbp-18h]
  unsigned long long v14; // [rsp+78h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  nmemb = sub_15524(a2, *(long long *)(a1 + 40));
  if ( !nmemb )
    return 0;
  if ( nmemb == *(long long *)(a1 + 16) )
    return 1;
  v4 = (char *)calloc(nmemb, 0x10u);
  if ( !v4 )
    return 0;
  v6 = nmemb;
  v5 = &v4[16 * nmemb];
  v7 = 0;
  v8 = 0;
  v9 = *(long long *)(a1 + 40);
  v10 = *(long long *)(a1 + 48);
  v11 = *(long long *)(a1 + 56);
  v12 = *(long long *)(a1 + 64);
  v13 = *(long long *)(a1 + 72);
  if ( (unsigned char)sub_15C0E((long long)&v4, a1, 0) )
  {
    free(*(void **)a1);
    *(long long *)a1 = v4;
    *(long long *)(a1 + 8) = v5;
    *(long long *)(a1 + 16) = v6;
    *(long long *)(a1 + 24) = v7;
    *(long long *)(a1 + 72) = v13;
    return 1;
  }
  else
  {
    *(long long *)(a1 + 72) = v13;
    if ( (unsigned char)sub_15C0E(a1, (long long)&v4, 1) != 1 || (unsigned char)sub_15C0E(a1, (long long)&v4, 0) != 1 )
      abort();
    free(v4);
    return 0;
  }
}



// Function: hash_insert_if_absent @ 0x1600d
long long hash_insert_if_absent(long long *a1, long long a2, long long *a3)
{
  long long v4; // rax
  float v5; // xmm0_4
  long long v6; // rdx
  long long v7; // rax
  float v8; // xmm1_4
  long long v9; // rdx
  long long v10; // rax
  float v11; // xmm0_4
  long long v12; // rdx
  long long v13; // rax
  float v14; // xmm1_4
  long long v15; // rdx
  long long v16; // rax
  float v17; // xmm0_4
  long long v18; // rdx
  float v19; // xmm0_4
  long long v20; // rax
  float v21; // xmm0_4
  long long v22; // rdx
  unsigned long long v23; // rax
  long long *v25; // [rsp+28h] [rbp-28h] BYREF
  long long v26; // [rsp+30h] [rbp-20h]
  long long v27; // [rsp+38h] [rbp-18h]
  long long *v28; // [rsp+40h] [rbp-10h]
  unsigned long long v29; // [rsp+48h] [rbp-8h]

  v29 = __readfsqword(0x28u);
  if ( !a2 )
    abort();
  v26 = sub_15A75((long long)a1, a2, &v25, 0);
  if ( v26 )
  {
    if ( a3 )
      *a3 = v26;
    return 0;
  }
  else
  {
    v4 = a1[3];
    if ( v4 < 0 )
    {
      v6 = a1[3] & 1LL | ((unsigned long long)v4 >> 1);
      v5 = (float)(int)v6 + (float)(int)v6;
    }
    else
    {
      v5 = (float)(int)v4;
    }
    v7 = a1[2];
    if ( v7 < 0 )
    {
      v9 = a1[2] & 1LL | ((unsigned long long)v7 >> 1);
      v8 = (float)(int)v9 + (float)(int)v9;
    }
    else
    {
      v8 = (float)(int)v7;
    }
    if ( v5 > (float)(v8 * *(float *)(a1[5] + 8LL)) )
    {
      sub_15417((long long)a1);
      v10 = a1[3];
      if ( v10 < 0 )
      {
        v12 = a1[3] & 1LL | ((unsigned long long)v10 >> 1);
        v11 = (float)(int)v12 + (float)(int)v12;
      }
      else
      {
        v11 = (float)(int)v10;
      }
      v13 = a1[2];
      if ( v13 < 0 )
      {
        v15 = a1[2] & 1LL | ((unsigned long long)v13 >> 1);
        v14 = (float)(int)v15 + (float)(int)v15;
      }
      else
      {
        v14 = (float)(int)v13;
      }
      if ( v11 > (float)(v14 * *(float *)(a1[5] + 8LL)) )
      {
        v27 = a1[5];
        if ( *(char *)(v27 + 16) )
        {
          v16 = a1[2];
          if ( v16 < 0 )
          {
            v18 = a1[2] & 1LL | ((unsigned long long)v16 >> 1);
            v17 = (float)(int)v18 + (float)(int)v18;
          }
          else
          {
            v17 = (float)(int)v16;
          }
          v19 = v17 * *(float *)(v27 + 12);
        }
        else
        {
          v20 = a1[2];
          if ( v20 < 0 )
          {
            v22 = a1[2] & 1LL | ((unsigned long long)v20 >> 1);
            v21 = (float)(int)v22 + (float)(int)v22;
          }
          else
          {
            v21 = (float)(int)v20;
          }
          v19 = *(float *)(v27 + 8) * (float)(*(float *)(v27 + 12) * v21);
        }
        if ( v19 >= 1.8446744e19 )
          return 0xFFFFFFFFLL;
        if ( v19 >= 9.223372e18 )
          v23 = (unsigned int)(int)(float)(v19 - 9.223372e18) ^ 0x8000000000000000LL;
        else
          v23 = (unsigned int)(int)v19;
        if ( (unsigned char)sub_15DE2((long long)a1, v23) != 1 )
          return 0xFFFFFFFFLL;
        if ( sub_15A75((long long)a1, a2, &v25, 0) )
          abort();
      }
    }
    if ( *v25 )
    {
      v28 = sub_159EC((long long)a1);
      if ( v28 )
      {
        *v28 = a2;
        v28[1] = v25[1];
        v25[1] = (long long)v28;
        ++a1[4];
        return 1;
      }
      else
      {
        return 0xFFFFFFFFLL;
      }
    }
    else
    {
      *v25 = a2;
      ++a1[4];
      ++a1[3];
      return 1;
    }
  }
}



// Function: hash_insert @ 0x1637e
long long hash_insert(long long *a1, long long a2)
{
  int v3; // [rsp+1Ch] [rbp-14h]
  long long v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = __readfsqword(0x28u);
  v3 = sub_1600D(a1, a2, v4);
  if ( v3 == -1 )
    return 0;
  if ( v3 )
    return a2;
  return v4[0];
}



// Function: hash_remove @ 0x163ee
long long hash_remove(long long a1, long long a2)
{
  long long v3; // rax
  float v4; // xmm0_4
  long long v5; // rdx
  long long v6; // rax
  float v7; // xmm1_4
  long long v8; // rdx
  long long v9; // rax
  float v10; // xmm0_4
  long long v11; // rdx
  long long v12; // rax
  float v13; // xmm1_4
  long long v14; // rdx
  long long v15; // rax
  float v16; // xmm0_4
  long long v17; // rdx
  float v18; // xmm0_4
  unsigned long long v19; // rax
  long long v20; // rax
  float v21; // xmm0_4
  long long v22; // rdx
  float v23; // xmm0_4
  long long *v24; // [rsp+18h] [rbp-38h] BYREF
  void *ptr; // [rsp+20h] [rbp-30h]
  long long v26; // [rsp+28h] [rbp-28h]
  long long v27; // [rsp+30h] [rbp-20h]
  unsigned long long v28; // [rsp+38h] [rbp-18h]
  void *v29; // [rsp+40h] [rbp-10h]
  unsigned long long v30; // [rsp+48h] [rbp-8h]

  v30 = __readfsqword(0x28u);
  v26 = sub_15A75(a1, a2, &v24, 1);
  if ( !v26 )
    return 0;
  --*(long long *)(a1 + 32);
  if ( !*v24 )
  {
    v3 = --*(long long *)(a1 + 24);
    if ( v3 < 0 )
    {
      v5 = *(long long *)(a1 + 24) & 1LL | ((unsigned long long)v3 >> 1);
      v4 = (float)(int)v5 + (float)(int)v5;
    }
    else
    {
      v4 = (float)(int)v3;
    }
    v6 = *(long long *)(a1 + 16);
    if ( v6 < 0 )
    {
      v8 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v6 >> 1);
      v7 = (float)(int)v8 + (float)(int)v8;
    }
    else
    {
      v7 = (float)(int)v6;
    }
    if ( (float)(v7 * **(float **)(a1 + 40)) > v4 )
    {
      sub_15417(a1);
      v9 = *(long long *)(a1 + 24);
      if ( v9 < 0 )
      {
        v11 = *(long long *)(a1 + 24) & 1LL | ((unsigned long long)v9 >> 1);
        v10 = (float)(int)v11 + (float)(int)v11;
      }
      else
      {
        v10 = (float)(int)v9;
      }
      v12 = *(long long *)(a1 + 16);
      if ( v12 < 0 )
      {
        v14 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v12 >> 1);
        v13 = (float)(int)v14 + (float)(int)v14;
      }
      else
      {
        v13 = (float)(int)v12;
      }
      if ( (float)(v13 * **(float **)(a1 + 40)) > v10 )
      {
        v27 = *(long long *)(a1 + 40);
        if ( *(char *)(v27 + 16) )
        {
          v15 = *(long long *)(a1 + 16);
          if ( v15 < 0 )
          {
            v17 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v15 >> 1);
            v16 = (float)(int)v17 + (float)(int)v17;
          }
          else
          {
            v16 = (float)(int)v15;
          }
          v18 = v16 * *(float *)(v27 + 4);
          if ( v18 >= 9.223372e18 )
            v19 = (unsigned int)(int)(float)(v18 - 9.223372e18) ^ 0x8000000000000000LL;
          else
            v19 = (unsigned int)(int)v18;
        }
        else
        {
          v20 = *(long long *)(a1 + 16);
          if ( v20 < 0 )
          {
            v22 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v20 >> 1);
            v21 = (float)(int)v22 + (float)(int)v22;
          }
          else
          {
            v21 = (float)(int)v20;
          }
          v23 = *(float *)(v27 + 8) * (float)(*(float *)(v27 + 4) * v21);
          if ( v23 >= 9.223372e18 )
            v19 = (unsigned int)(int)(float)(v23 - 9.223372e18) ^ 0x8000000000000000LL;
          else
            v19 = (unsigned int)(int)v23;
        }
        v28 = v19;
        if ( (unsigned char)sub_15DE2(a1, v19) != 1 )
        {
          for ( ptr = *(void **)(a1 + 72); ptr; ptr = v29 )
          {
            v29 = (void *)*((long long *)ptr + 1);
            free(ptr);
          }
          *(long long *)(a1 + 72) = 0;
        }
      }
    }
  }
  return v26;
}



// Function: hash_delete @ 0x166f4
long long hash_delete(long long a1, long long a2)
{
  return sub_163EE(a1, a2);
}



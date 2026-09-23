// Function: sub_44b3 @ 0x44b3
void sub_44b3(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_27608;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n"),v2,v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n"),v1);
    sub_4199();
    v1 = stdout;
    fputs_unlocked(gettext("  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don\'t copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --no-preserve=ATTR_LIST  don\'t preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n"),v1);
    sub_41ca();
    v1 = stdout;
    fputs_unlocked(gettext("\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n"),v1);
    sub_421c("cp");
  }
  exit(a0); // no-return
}


// Function: sub_4793 @ 0x4793
unsigned long sub_4793(char *a0,unsigned int a1,long a2,long a3,long a4)
{
  unsigned int v1;
  char *v10; // rax
  char v11 [8];
  unsigned long v12; // stack - 0x48
  char *v13; // rsp
  char v14 [16];
  long v15; // stack - 0xb0
  long v16; // stack - 0xa8
  long v17; // stack - 0xa0
  unsigned int v18; // stack - 0x94
  char *v19; // stack - 0x90
  unsigned int v2;
  unsigned int v20; // stack - 0x7c
  long v21; // stack - 0x78
  char *v22; // stack - 0x70
  unsigned long v23; // stack - 0x68
  void *v24; // stack - 0x60
  void *v25; // stack - 0x58
  long v26; // stack - 0x50
  unsigned long v27; // stack - 0x40
  unsigned long v28; // stack - 0x38
  unsigned long v29; // stack - 0x30
  char v3;
  long v4;
  char v5; // al
  int v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v15 = a4;
  v16 = a3;
  v17 = a2;
  v18 = a1;
  v19 = a0;
  v22 = a0;
  v7 = strlen(a0);
  v23 = v7 + 1;
  v8 = ((v7 + 0x18) / 0x10) * 0x10;
  for (v13 = v11; v13 != &v11[-(v8 & 0xfffffffffffff000)]; v13 = &v13[-0x1000]) {
    *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
  }
  v4 = -(unsigned long)((unsigned int)v8 & 0xfff);
  if (v8 & 0xfff)
    *(unsigned long *)&v13[((unsigned long)((unsigned int)v8 & 0xfff) - 8) + v4] = *(unsigned long *)&v13[((unsigned long)((unsigned int)v8 & 0xfff) - 8) + v4];
  v24 = (void *)((unsigned long)&v13[v4 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v13[v4 + -8] = 0x4895;
  v25 = memcpy((void *)((unsigned long)&v13[v4 + 0xf] & 0xfffffffffffffff0),v22,v7 + 1,v13[v4 + -8]);
  v26 = (long)v25 + (v17 - (long)v19);
  v21 = v16;
  do {
    if (!v21)
      return 1;
    *(char *)((long)v25 + *(long *)(v21 + 0x98)) = 0;
    if (*(char *)(v15 + 0x1f)) {
      *(unsigned long *)&v13[v4 + -8] = 0x48f9;
      v14 = sub_19e57(v21);
      v12 = SUB168(v14,0);
      v27 = SUB168(v14,8);
      *(unsigned long *)&v13[v4 + -8] = 0x490d;
      v14 = sub_19e8b(v21);
      v28 = SUB168(v14,0);
      v29 = SUB168(v14,8);
      *(unsigned long *)&v13[v4 + -8] = 0x492f;
      v6 = utimensat(v18,v26,&v12,0);
      if (v6) {
        *(unsigned long *)&v13[v4 + -8] = 0x4944;
        v9 = sub_18a0f(4,v25);
        *(unsigned long *)&v13[v4 + -8] = 0x4956;
        v10 = gettext("failed to preserve times for %s",v13[v4 + -8]);
        *(unsigned long *)&v13[v4 + -8] = 0x495e;
        v3 = v13[v4 + -8];
        v6 = *__errno_location(v3);
        *(unsigned long *)&v13[v4 + -8] = 0x4977;
        error(0,v6,v10,v9);
        return 0;
      }
    }
    if (*(char *)(v15 + 0x1d)) {
      v1 = *(unsigned int *)(v21 + 0x20);
      v2 = *(unsigned int *)(v21 + 0x1c);
      *(unsigned long *)&v13[v4 + -8] = 0x49b3;
      v6 = sub_112c3(v18,v26,v2,v1);
      if (v6) {
        *(unsigned long *)&v13[v4 + -8] = 0x49ca;
        v5 = sub_dc1f(v15);
        if (v5 != '\x01') {
          *(unsigned long *)&v13[v4 + -8] = 0x49e2;
          v9 = sub_18a0f(4,v25);
          *(unsigned long *)&v13[v4 + -8] = 0x49f4;
          v10 = gettext("failed to preserve ownership for %s",v13[v4 + -8]);
          *(unsigned long *)&v13[v4 + -8] = 0x49fc;
          v3 = v13[v4 + -8];
          v6 = *__errno_location(v3);
          *(unsigned long *)&v13[v4 + -8] = 0x4a15;
          error(0,v6,v10,v9);
          return 0;
        }
        v1 = *(unsigned int *)(v21 + 0x20);
        *(unsigned long *)&v13[v4 + -8] = 0x4a3e;
        v20 = sub_112c3(v18,v26,0xffffffff,v1);
      }
    }
    if (*(char *)(v15 + 0x1e)) { // branch-flip
      v1 = *(unsigned int *)(v21 + 0x18);
      *(unsigned long *)&v13[v4 + -8] = 0x4a74;
      v6 = sub_ec57(v26,0xffffffff,v25,0xffffffff,v1);
      if (v6)
        return 0;
    }
    else if (*(char *)(v21 + 0x90)) {
      v1 = *(unsigned int *)(v21 + 0x18);
      *(unsigned long *)&v13[v4 + -8] = 0x4ab0;
      v6 = sub_11260(v18,v26,v1);
      if (v6) {
        *(unsigned long *)&v13[v4 + -8] = 0x4ac5;
        v9 = sub_18a0f(4,v25);
        *(unsigned long *)&v13[v4 + -8] = 0x4ad7;
        v10 = gettext("failed to preserve permissions for %s",v13[v4 + -8]);
        *(unsigned long *)&v13[v4 + -8] = 0x4adf;
        v3 = v13[v4 + -8];
        v6 = *__errno_location(v3);
        *(unsigned long *)&v13[v4 + -8] = 0x4af8;
        error(0,v6,v10,v9);
        return 0;
      }
    }
    *(char *)((long)v25 + *(long *)(v21 + 0x98)) = 0x2f;
    v21 = *(long *)(v21 + 0xa0);
  } while( true );
}


// Function: sub_4b50 @ 0x4b50
unsigned long sub_4b50(char *a0,unsigned long a1,int a2,char *a3,long *a4,char *a5,long a6) // return-dupe, ternary x2
{
  char v1;
  void *v10; // rax
  unsigned long v11; // rax
  char *v12;
  char v13 [8];
  char v14 [24];
  unsigned long v15; // stack - 0xb8
  char *v16; // rsp
  char *v17; // rsp
  long v18; // stack - 0x1f0
  char *v19; // stack - 0x1e8
  long v2;
  long *v20; // stack - 0x1e0
  char *v21; // stack - 0x1d8
  int v22; // stack - 0x1cc
  unsigned long v23; // stack - 0x1c8
  char *v24; // stack - 0x1c0
  bool v25; // stack - 0x1ad
  unsigned int v26; // stack - 0x1ac
  int v27; // stack - 0x1a8
  unsigned int v28; // stack - 0x1a4
  unsigned int v29; // stack - 0x1a0
  char v3; // al
  unsigned int v30; // stack - 0x19c
  char *v31; // stack - 0x198
  char *v32; // stack - 0x190
  unsigned long *v33; // stack - 0x188
  unsigned long v34; // stack - 0x180
  char *v35; // stack - 0x178
  unsigned long v36; // stack - 0x170
  void *v37; // stack - 0x168
  void *v38; // stack - 0x160
  char *v39; // stack - 0x158
  int v4;
  void *v40; // stack - 0x150
  unsigned int v41; // stack - 0x130
  unsigned long v42; // stack - 0xb0
  unsigned long v43; // stack - 0xa8
  unsigned int v44; // stack - 0xa0
  unsigned int v45; // stack - 0x9c
  unsigned long v46; // stack - 0x98
  unsigned long v47; // stack - 0x90
  unsigned long v48; // stack - 0x88
  unsigned long v49; // stack - 0x80
  unsigned int v5;
  unsigned long v50; // stack - 0x78
  unsigned long v51; // stack - 0x70
  unsigned long v52; // stack - 0x68
  unsigned long v53; // stack - 0x60
  unsigned long v54; // stack - 0x58
  unsigned long v55; // stack - 0x50
  unsigned long v56; // stack - 0x48
  unsigned long v57; // stack - 0x40
  unsigned long v58; // stack - 0x38
  unsigned long v59; // stack - 0x30
  unsigned int v6; // eax
  unsigned int v7;
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v16 = v13;
  v18 = a6;
  v19 = a5;
  v20 = a4;
  v21 = a3;
  v22 = a2;
  v23 = a1;
  v24 = a0;
  v34 = sub_1158b(a0);
  *v20 = 0;
  if (v34 <= v23)
    return 1;
  v35 = v24;
  v8 = strlen(v24);
  v36 = v8 + 1;
  v9 = ((v8 + 0x18) / 0x10) * 0x10;
  for (; v16 != &v13[-(v9 & 0xfffffffffffff000)]; v16 = &v16[-0x1000]) {
    *(unsigned long *)&v16[-8] = *(unsigned long *)&v16[-8];
  }
  v2 = -(unsigned long)((unsigned int)v9 & 0xfff);
  v17 = &v16[v2];
  if (v9 & 0xfff)
    *(unsigned long *)&v16[((unsigned long)((unsigned int)v9 & 0xfff) - 8) + v2] = *(unsigned long *)&v16[((unsigned long)((unsigned int)v9 & 0xfff) - 8) + v2];
  v37 = (void *)((unsigned long)&v16[v2 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v16[v2 + -8] = 0x4cba;
  v10 = memcpy((void *)((unsigned long)&v16[v2 + 0xf] & 0xfffffffffffffff0),v35,v8 + 1,v16[v2 + -8]);
  v38 = v10;
  v39 = (char *)(v23 + (long)v10);
  v8 = ((v34 + 0x18) / 0x10) * 0x10;
  for (; v17 != &v16[v2 - (v8 & 0xfffffffffffff000)]; v17 = &v17[-0x1000]) {
    *(unsigned long *)&v17[-8] = *(unsigned long *)&v17[-8];
  }
  v2 = -(unsigned long)((unsigned int)v8 & 0xfff);
  if (v8 & 0xfff)
    *(unsigned long *)&v17[((unsigned long)((unsigned int)v8 & 0xfff) - 8) + v2] = *(unsigned long *)&v17[((unsigned long)((unsigned int)v8 & 0xfff) - 8) + v2];
  v40 = (void *)((unsigned long)&v17[v2 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v17[v2 + -8] = 0x4d8c;
  memcpy((void *)((unsigned long)&v17[v2 + 0xf] & 0xfffffffffffffff0),v10,v34,v17[v2 + -8]);
  *(char *)((long)v40 + v34) = 0;
  for (v31 = (char *)(v23 + (long)v40); *v31 == '/'; v31 = &v31[1]) {
  }
  *(unsigned long *)&v17[v2 + -8] = 0x4df0;
  v4 = fstatat(v22,v31,v14,0,v17[v2 + -8]);
  if (!v4) {
    if ((v41 & 0xf000) != 0x4000) {
      *(unsigned long *)&v17[v2 + -8] = 0x54c3;
      v11 = sub_18a0f(4,v40);
      *(unsigned long *)&v17[v2 + -8] = 0x54d5;
      v12 = gettext("%s exists but is not a directory",v17[v2 + -8]);
      *(unsigned long *)&v17[v2 + -8] = 0x54ef;
      error(0,0,v12,v11);
      return 0;
    }
    *v19 = '\0';
    return 1;
  }
  for (v32 = v39; *v32 == '/'; v32 = &v32[1]) {
  }
  v31 = v32;
  do {
    *(unsigned long *)&v17[v2 + -8] = 0x5486;
    v32 = strchr(v32,0x2f,v17[v2 + -8]);
    if (!v32)
      return 1;
    *v32 = '\0';
    *(unsigned long *)&v17[v2 + -8] = 0x4e5b;
    v4 = fstatat(v22,v31,v14,0,v17[v2 + -8]);
    v25 = v4 != 0;
    if ((((v25) || (*(char *)(v18 + 0x1d))) || (*(char *)(v18 + 0x1e))) || (*(char *)(v18 + 0x1f))) {
      *(unsigned long *)&v17[v2 + -8] = 0x4eb9;
      v4 = stat(v39,&v15,v17[v2 + -8]);
      if (v4) { // branch-flip
        *(unsigned long *)&v17[v2 + -8] = 0x4ec2;
        v1 = v17[v2 + -8];
        v27 = *__errno_location(v1);
      }
      else {
        v27 = ((v44 & 0xf000) != 0x4000) ? 0x14 : 0; // branch-flip
      }
      if (v27) {
        *(unsigned long *)&v17[v2 + -8] = 0x4f07;
        v11 = sub_18a0f(4,v39);
        *(unsigned long *)&v17[v2 + -8] = 0x4f19;
        v12 = gettext("failed to get attributes of %s",v17[v2 + -8]);
        *(unsigned long *)&v17[v2 + -8] = 0x4f36;
        error(0,v27,v12,v11);
        return 0;
      }
      *(unsigned long *)&v17[v2 + -8] = 0x4f4a;
      v33 = (unsigned long *)sub_1b941(0xa8);
      *v33 = v15;
      v33[1] = v42;
      v33[2] = v43;
      v33[3] = CONCAT44(v45,v44);
      v33[4] = v46;
      v33[5] = v47;
      v33[6] = v48;
      v33[7] = v49;
      v33[8] = v50;
      v33[9] = v51;
      v33[10] = v52;
      v33[0xb] = v53;
      v33[0xc] = v54;
      v33[0xd] = v55;
      v33[0xe] = v56;
      v33[0xf] = v57;
      v33[0x10] = v58;
      v33[0x11] = v59;
      v33[0x13] = (long)v32 - (long)v38;
      *(char *)&v33[0x12] = 0;
      v33[0x14] = *v20;
      *v20 = (long)v33;
    }
    if (v25) // branch-flip
      v5 = *(unsigned int *)&v33[3];
    else {
      v5 = 0;
    }
    *(unsigned long *)&v17[v2 + -8] = 0x5097;
    v3 = sub_84c6(v39,v38,v5,v25,v18);
    if (v3 != '\x01')
      return 0;
    if (v25) { // branch-flip
      *v19 = '\x01';
      v7 = *(unsigned int *)&v33[3];
      v28 = v7;
      if (*(char *)(v18 + 0x1d) != '\x01') { // branch-flip
        v6 = (*(char *)(v18 + 0x1e)) ? 0x12 : 0; // branch-flip
      }
      else {
        v6 = 0x3f;
      }
      v26 = v6 & v7;
      if (*(char *)(v18 + 0x20))
        v7 = 0x1ff;
      v7 = ~v26 & v7 & 0xfff;
      v29 = v7;
      *(unsigned long *)&v17[v2 + -8] = 0x5167;
      v4 = mkdirat(v22,v31,v7);
      if (v4) {
        *(unsigned long *)&v17[v2 + -8] = 0x517f;
        v11 = sub_18a0f(4,v38);
        *(unsigned long *)&v17[v2 + -8] = 0x5191;
        v12 = gettext("cannot make directory %s",v17[v2 + -8]);
        *(unsigned long *)&v17[v2 + -8] = 0x5199;
        v1 = v17[v2 + -8];
        v4 = *__errno_location(v1);
        *(unsigned long *)&v17[v2 + -8] = 0x51b2;
        error(0,v4,v12,v11);
        return 0;
      }
      if (v21) {
        *(unsigned long *)&v17[v2 + -8] = 0x51eb;
        printf(v21,v39,v38);
      }
      *(unsigned long *)&v17[v2 + -8] = 0x520b;
      v4 = fstatat(v22,v31,v14,0x100,v17[v2 + -8]);
      if (v4) {
        *(unsigned long *)&v17[v2 + -8] = 0x5223;
        v11 = sub_18a0f(4,v38);
        *(unsigned long *)&v17[v2 + -8] = 0x5235;
        v12 = gettext("failed to get attributes of %s",v17[v2 + -8]);
        *(unsigned long *)&v17[v2 + -8] = 0x523d;
        v1 = v17[v2 + -8];
        v4 = *__errno_location(v1);
        *(unsigned long *)&v17[v2 + -8] = 0x5256;
        error(0,v4,v12,v11);
        return 0;
      }
      if (*(char *)(v18 + 0x1e) != '\x01') {
        if (~v41 & v26) {
          *(unsigned long *)&v17[v2 + -8] = 0x5289;
          v26 &= ~sub_dcaf();
        }
        if ((~v41 & v26) || ((v41 & 0x1c0) != 0x1c0)) {
          *(unsigned int *)&v33[3] = v41 | v26;
          *(char *)&v33[0x12] = 1;
        }
      }
      v7 = v41 | 0x1c0;
      v30 = v7;
      if (v7 != v41) {
        *(unsigned long *)&v17[v2 + -8] = 0x531b;
        v4 = sub_11260(v22,v31,v7);
        if (v4) {
          *(unsigned long *)&v17[v2 + -8] = 0x5337;
          v11 = sub_18a0f(4,v38);
          *(unsigned long *)&v17[v2 + -8] = 0x5349;
          v12 = gettext("setting permissions for %s",v17[v2 + -8]);
          *(unsigned long *)&v17[v2 + -8] = 0x5351;
          v1 = v17[v2 + -8];
          v4 = *__errno_location(v1);
          *(unsigned long *)&v17[v2 + -8] = 0x536a;
          error(0,v4,v12,v11);
          return 0;
        }
      }
    }
    else {
      if ((v41 & 0xf000) != 0x4000) {
        *(unsigned long *)&v17[v2 + -8] = 0x539a;
        v11 = sub_18a0f(4,v38);
        *(unsigned long *)&v17[v2 + -8] = 0x53ac;
        v12 = gettext("%s exists but is not a directory",v17[v2 + -8]);
        *(unsigned long *)&v17[v2 + -8] = 0x53c6;
        error(0,0,v12,v11);
        return 0;
      }
      *v19 = '\0';
    }
    if ((*v19 != '\x01') && ((*(long *)(v18 + 0x28) || (*(char *)(v18 + 0x33))))) {
      *(unsigned long *)&v17[v2 + -8] = 0x5425;
      v3 = sub_8750(v38,0,v18);
      if ((v3 != '\x01') && (*(char *)(v18 + 0x34)))
        return 0;
    }
    v12 = v32;
    v32 = &v32[1];
    *v12 = '/';
    for (; *v32 == '/'; v32 = &v32[1]) {
    }
  } while( true );
}


// Function: sub_5522 @ 0x5522
unsigned long sub_5522(int a0,unsigned long *a1,long a2,char a3,unsigned long *a4) // ternary
{
  long v1;
  unsigned long v10;
  unsigned long v11; // rax
  void *v12;
  char *v13;
  unsigned long *v14; // stack - 0x178
  char v15 [24];
  char *v16; // stack - 0x130
  unsigned char v17; // stack - 0x14c
  void *v18; // stack - 0x138
  long v19;
  unsigned char v2; // al
  char *v20;
  unsigned long **v21; // rsp
  long v22; // stack - 0x170
  unsigned long *v23; // stack - 0x168
  char v24; // stack - 0x160
  int v25; // stack - 0x15c
  bool v26; // stack - 0x14a
  char v27; // stack - 0x149
  unsigned int v28; // stack - 0x148
  int v29; // stack - 0x144
  unsigned int v3; // eax
  unsigned int v30; // stack - 0x140
  int v31; // stack - 0x13c
  void *v32; // stack - 0x128
  char *v33; // stack - 0x120
  long v34; // stack - 0x118
  char *v35; // stack - 0x110
  char *v36; // stack - 0x108
  char *v37; // stack - 0x100
  unsigned long v38; // stack - 0xf8
  void *v39; // stack - 0xf0
  int v4; // eax
  char *v40; // stack - 0xe8
  char *v41; // stack - 0xe0
  unsigned long v42; // stack - 0xd8
  void *v43; // stack - 0xd0
  void *v44; // stack - 0xc8
  void *v45; // stack - 0xc0
  unsigned int v46; // stack - 0xa0
  unsigned int v5; // eax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v21 = &v14;
  v25 = a0;
  v23 = a1;
  v22 = a2;
  v14 = a4;
  v24 = a3;
  v17 = 0;
  v26 = 1;
  if (a0 <= (int)(unsigned int)(a2 == 0)) {
    if (1 <= a0) { // branch-flip
      v6 = sub_18a0f(4,*a1);
      error(0,0,gettext("missing destination file operand after %s"),v6);
    }
    else {
      error(0,0,gettext("missing file operand"));
    }
    sub_44b3(1); // no-return
  }
  v46 = 0;
  v28 = 0xffffff9c;
  if (a3) { // branch-flip
    if (a2) {
      error(1,0,gettext("cannot combine --target-directory (-t) and --no-target-directory (-T)"));
      return v7;
    }
    if (3 <= a0) {
      v6 = sub_18a0f(4,a1[2]);
      error(0,0,gettext("extra operand %s"),v6);
      sub_44b3(1); // no-return
    }
  }
  else if (a2) { // branch-flip
    v3 = sub_19f57(a2,v15);
    v28 = v3;
    if (sub_19edf(v3) != '\x01') {
      v6 = sub_18a0f(4,v22);
      v13 = gettext("target directory %s");
      error(1,*__errno_location(),v13,v6);
      return v8;
    }
  }
  else {
    v34 = a1[(long)a0 + -1];
    v3 = sub_19f57(v34,v15);
    v30 = v3;
    if (sub_19edf(v3)) { // branch-flip
      v28 = v30;
      v22 = v34;
      v25 -= 1;
    }
    else {
      v31 = *__errno_location();
      if (v31 == 2)
        v17 = 1;
      if (3 <= v25) {
        v6 = sub_18a0f(4,v34);
        error(1,v31,gettext("target %s"),v6);
        return v9;
      }
    }
  }
  if (v22) { // branch-flip
    if (2 <= v25) {
      sub_a549(v14);
      sub_a5a6(v14);
    }
    for (v29 = 0; v29 < v25; v29 = v29 + 1) {
      v27 = '\x01';
      v16 = NULL;
      v13 = (char *)v23[v29];
      v36 = v13;
      if (dat_27122) {
        *(unsigned long *)((long)v21 + -8) = 0x58b3;
        sub_116ab(v13);
      }
      if (dat_27121) { // branch-flip
        v41 = v36;
        *(unsigned long *)((long)v21 + -8) = 0x58df;
        v10 = strlen(v36,*(char *)((long)v21 + -8));
        v42 = v10 + 1;
        v11 = ((v10 + 0x18) / 0x10) * 0x10;
        for (v20 = (char *)v21; v20 != (char *)((long)v21 + -(v11 & 0xfffffffffffff000)); v20 = &v20[-0x1000]) {
          *(unsigned long *)&v20[-8] = *(unsigned long *)&v20[-8];
        }
        v1 = -(unsigned long)((unsigned int)v11 & 0xfff);
        v21 = (char *)&v20[v1];
        if (v11 & 0xfff)
          *(unsigned long *)&v20[((unsigned long)((unsigned int)v11 & 0xfff) - 8) + v1] = *(unsigned long *)&v20[((unsigned long)((unsigned int)v11 & 0xfff) - 8) + v1];
        v43 = (void *)((unsigned long)&v20[v1 + 0xf] & 0xfffffffffffffff0);
        *(unsigned long *)&v20[v1 + -8] = 0x5999;
        v12 = memcpy((void *)((unsigned long)&v20[v1 + 0xf] & 0xfffffffffffffff0),v41,v10 + 1,v20[v1 + -8]);
        v44 = v12;
        *(unsigned long *)&v20[v1 + -8] = 0x59af;
        sub_116ab(v12);
        *(unsigned long *)&v20[v1 + -8] = 0x59cf;
        v32 = (void *)sub_11da5(v22,v44,&v16);
        v13 = (*(char *)((long)v14 + 0x3c)) ? "%s -> %s\n" : NULL; // branch-flip
        *(unsigned long **)&v20[v1 + -0x10] = v14;
        *(unsigned long *)&v20[v1 + -0x18] = 0x5a3a;
        v27 = sub_4b50(v32,(long)v16 - (long)v32,v28,v13,&v18,&v17);
        for (; *v16 == '/'; v16 = &v16[1]) {
        }
      }
      else {
        *(unsigned long *)((long)v21 + -8) = 0x5a7a;
        v13 = (char *)sub_10369(v36);
        v37 = v13;
        *(unsigned long *)((long)v21 + -8) = 0x5a90;
        v10 = strlen(v13,*(char *)((long)v21 + -8));
        v38 = v10 + 1;
        v11 = ((v10 + 0x18) / 0x10) * 0x10;
        for (v20 = (char *)v21; v20 != (char *)((long)v21 + -(v11 & 0xfffffffffffff000)); v20 = &v20[-0x1000]) {
          *(unsigned long *)&v20[-8] = *(unsigned long *)&v20[-8];
        }
        v1 = -(unsigned long)((unsigned int)v11 & 0xfff);
        v21 = (char *)&v20[v1];
        if (v11 & 0xfff)
          *(unsigned long *)&v20[((unsigned long)((unsigned int)v11 & 0xfff) - 8) + v1] = *(unsigned long *)&v20[((unsigned long)((unsigned int)v11 & 0xfff) - 8) + v1];
        v39 = (void *)((unsigned long)&v20[v1 + 0xf] & 0xfffffffffffffff0);
        *(unsigned long *)&v20[v1 + -8] = 0x5b4a;
        v13 = memcpy((void *)((unsigned long)&v20[v1 + 0xf] & 0xfffffffffffffff0),v37,v10 + 1,v20[v1 + -8]);
        v40 = v13;
        *(unsigned long *)&v20[v1 + -8] = 0x5b60;
        sub_116ab(v13);
        *(unsigned long *)&v20[v1 + -8] = 0x5b79;
        v4 = strcmp(v40,"..",v20[v1 + -8]);
        v13 = &v40[v4 == 0];
        v40 = v13;
        *(unsigned long *)&v20[v1 + -8] = 0x5ba8;
        v32 = (void *)sub_11da5(v22,v13,&v16);
      }
      if (v27 != '\x01') // branch-flip
        v26 = 0;
      else {
        *(unsigned long *)((long)v21 + -8) = 0;
        *(char **)((long)v21 + -0x10) = &v19;
        *(unsigned long *)((long)v21 + -0x18) = 0x5c08;
        v2 = sub_daea(v36,v32,v28,v16,v17,v14);
        v26 = (v2 & v26) != 0;
        if (dat_27121) {
          *(unsigned long *)((long)v21 + -8) = 0x5c5b;
          v2 = sub_4793(v32,v28,v16,v18,v14);
          v26 = (v2 & v26) != 0;
        }
      }
      if (dat_27121) {
        while (v12 = v18, v18) {
          v45 = v18;
          v18 = *(void **)((long)v18 + 0xa0);
          *(unsigned long *)((long)v21 + -8) = 0x5cb1;
          free(v12,*(char *)((long)v21 + -8));
        }
      }
      *(unsigned long *)((long)v21 + -8) = 0x5ccc;
      free(v32,*(char *)((long)v21 + -8));
    }
  }
  else {
    v35 = (char *)*v23;
    v33 = (char *)v23[1];
    if (dat_27121) {
      error(0,0,gettext("with --parents, the destination must be a directory"));
      sub_44b3(1); // no-return
    }
    if (((((*(char *)((long)v14 + 0x16)) && ((int)*v14)) && (v4 = strcmp(v35,v33), !v4)) && ((v17 != 1 && ((v46 || (v4 = stat(v33,v15), !v4)))))) && ((v46 & 0xf000) == 0x8000)) {
      v10 = *v14;
      v33 = (char *)sub_1024d(0xffffff9c,v33,(int)v10);
      dat_27148 = v14[1];
      dat_27150 = v14[2];
      dat_27158 = v14[3];
      dat_27160 = v14[4];
      dat_27168 = v14[5];
      dat_27170 = v14[6];
      dat_27178 = v14[7];
      dat_27180 = v14[8];
      dat_27188 = v14[9];
      dat_27190 = v14[10];
      dat_27140 = *v14 & 0xffffffff00000000;
      v14 = (unsigned long *)0x27140;
    }
    v5 = (unsigned int)v17;
    v26 = (bool)sub_daea(v35,v33,0xffffff9c,v33,-v5,v14,&v16,0);
  }
  return (unsigned long)v26;
}


// Function: sub_5f11 @ 0x5f11
void sub_5f11(long a0)
{
  sub_dbcb(a0);
  *(char *)(a0 + 0x14) = 1;
  *(unsigned int *)(a0 + 4) = 1;
  *(char *)(a0 + 0x15) = 0;
  *(char *)(a0 + 0x16) = 0;
  *(char *)(a0 + 0x17) = 0;
  *(unsigned int *)(a0 + 8) = 4;
  *(char *)(a0 + 0x18) = 0;
  *(char *)(a0 + 0x19) = 0;
  *(char *)(a0 + 0x1c) = 0;
  *(unsigned int *)(a0 + 0x44) = 1;
  *(char *)(a0 + 0x1d) = 0;
  *(char *)(a0 + 0x30) = 0;
  *(char *)(a0 + 0x1e) = 0;
  *(char *)(a0 + 0x1f) = 0;
  *(char *)(a0 + 0x20) = 0;
  *(char *)(a0 + 0x33) = 0;
  *(char *)(a0 + 0x34) = 0;
  *(unsigned long *)(a0 + 0x28) = 0;
  *(char *)(a0 + 0x35) = 0;
  *(char *)(a0 + 0x37) = 0;
  *(char *)(a0 + 0x36) = 0;
  *(char *)(a0 + 0x31) = 1;
  *(char *)(a0 + 0x32) = 0;
  *(char *)(a0 + 0x38) = 0;
  *(unsigned int *)(a0 + 0xc) = 2;
  *(char *)(a0 + 0x3a) = 0;
  *(char *)(a0 + 0x39) = 0;
  *(unsigned int *)(a0 + 0x10) = 0;
  *(char *)(a0 + 0x3d) = 0;
  *(char *)(a0 + 0x3b) = 0;
  *(char *)(a0 + 0x3c) = 0;
  *(bool *)(a0 + 0x3e) = getenv("POSIXLY_CORRECT") != NULL;
  *(unsigned long *)(a0 + 0x48) = 0;
  *(unsigned long *)(a0 + 0x50) = 0;
}


// Function: sub_606f @ 0x606f
void sub_606f(unsigned long a0,long a1,char a2) // ternary
{
  char *v1; // rax
  char *v2; // rax
  long v3; // rax
  char *v4; // stack - 0x20
  char *v5; // stack - 0x18
  
  v1 = (char *)sub_1bf48(a0);
  v4 = v1;
  do {
    v2 = strchr(v4,0x2c);
    v5 = v2;
    if (v2) {
      v5 = &v2[1];
      *v2 = '\0';
    }
    v2 = (a2) ? "--preserve" : "--no-preserve"; // branch-flip
    v3 = sub_f72a(v2,v4,0x26840,0x1fb60,4,dat_27028,1);
    switch(*(unsigned int *)(v3 * 4 + 0x1fb60)) {
      case 0:
        *(char *)(a1 + 0x1e) = a2;
        *(bool *)(a1 + 0x20) = a2 == '\0';
        break;
      case 1:
        *(char *)(a1 + 0x1f) = a2;
        break;
      case 2:
        *(char *)(a1 + 0x1d) = a2;
        break;
      case 3:
        *(char *)(a1 + 0x30) = a2;
        break;
      case 4:
        *(char *)(a1 + 0x34) = a2;
        *(char *)(a1 + 0x33) = a2;
        break;
      case 5:
        *(char *)(a1 + 0x35) = a2;
        *(char *)(a1 + 0x36) = a2;
        break;
      case 6:
        *(char *)(a1 + 0x1e) = a2;
        *(char *)(a1 + 0x1f) = a2;
        *(char *)(a1 + 0x1d) = a2;
        *(char *)(a1 + 0x30) = a2;
        *(bool *)(a1 + 0x20) = a2 == '\0';
        if (dat_27120)
          *(char *)(a1 + 0x33) = a2;
        *(char *)(a1 + 0x35) = a2;
        break;
      default:
        abort(); // no-return
      
    }
    v4 = v5;
  } while (v5);
  free(v1);
}


// Function: sub_6276 @ 0x6276
unsigned long sub_6276(int a0,char **a1)
{
  bool v1;
  char v10; // stack - 0x9f
  char v11; // stack - 0x9e
  long v12; // stack - 0x98
  long v13; // stack - 0x90
  long v14; // stack - 0x88
  long v15; // stack - 0x80
  int v16; // stack - 0x74
  int v17; // stack - 0x70
  int v18; // stack - 0x6c
  char v19; // stack - 0x64
  int v2; // eax
  char v20; // stack - 0x63
  char v21; // stack - 0x62
  char v22; // stack - 0x61
  char v23; // stack - 0x5c
  char v24; // stack - 0x5b
  char v25; // stack - 0x5a
  char v26; // stack - 0x59
  long v27; // stack - 0x50
  char v28; // stack - 0x48
  char v29; // stack - 0x47
  unsigned long v3; // rax
  char v30; // stack - 0x46
  char v31; // stack - 0x45
  char v32; // stack - 0x44
  char v33; // stack - 0x43
  char v34; // stack - 0x41
  char v35; // stack - 0x40
  char v36; // stack - 0x3e
  char v37; // stack - 0x3d
  char v38; // stack - 0x3c
  int v39; // stack - 0x34
  unsigned long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7; // rax
  long v8; // rax
  unsigned int v9; // stack - 0x78
  
  v1 = 0;
  v12 = 0;
  v13 = 0;
  v10 = 0;
  v14 = 0;
  v11 = 0;
  v15 = 0;
  sub_16d05(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_1d610(sub_11312);
  dat_27120 = 0 < (int)is_selinux_enabled();
  sub_5f11(&v9);
label_67ab:
  do {
    v2 = getopt_long(a0,a1,"abdfHilLnprst:uvxPRS:TZ",(void *)0x26480,NULL);
    if (v2 == -1) {
      if ((v22) && (v36)) {
        error(0,0,gettext("cannot make both hard and symbolic links"));
        sub_44b3(1); // no-return
      }
      if (v17 == 2)
        v37 = 0;
      if ((v1) && (v17 == 2)) {
        error(0,0,gettext("options --backup and --no-clobber are mutually exclusive"));
        sub_44b3(1); // no-return
      }
      if ((v39 == 2) && (v18 != 2)) {
        error(0,0,gettext("--reflink can be used only with --sparse=auto"));
        sub_44b3(1); // no-return
      }
      if (v1)
        v9 = sub_1030b(gettext("backup type"),v13);
      else {
        v9 = 0;
      }
      sub_f869(v12);
      if (v16 == 1) {
        if ((v35) && (v22 != '\x01'))
          v16 = 2;
        else {
          v16 = 4;
        }
      }
      if (v35)
        v19 = v10;
      if (((v27) || (v15)) && (v32 != '\x01'))
        v31 = '\0';
      if ((v31) && ((v27 || (v15)))) {
        error(1,0,gettext("cannot set target context and preserve it"));
        return v3;
      }
      if ((v32) && (dat_27120 != '\x01')) {
        error(1,0,gettext("cannot preserve security context without an SELinux-enabled kernel"));
        return v4;
      }
      if ((v15) && ((int)setfscreatecon(v15) <= -1)) {
        v5 = sub_18e56(v15);
        v6 = gettext("failed to set default file creation context to %s");
        error(1,*__errno_location(),v6,v5);
        return v7;
      }
      sub_df3d();
      v8 = (long)optind;
      a0 -= optind;
      return (unsigned long)(sub_5522(a0,&a1[v8],v14,v11,&v9) ^ 1);
    }
    if (0x88 < v2)
      sub_44b3(1); // no-return, return-dupe
    if (v2 <= 0x47) {
      if (v2 == -0x83) {
        sub_1b6df(stdout,"cp","GNU coreutils",dat_27020,"Torbjorn Granlund","David MacKenzie","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_44b3(0); // no-return
      sub_44b3(1);
    }
    switch(v2) {
      case 0x48:
        v16 = 3;
        goto label_67ab;
      default:
        goto label_67a1;
      case 0x4c:
        v16 = 4;
        goto label_67ab;
      case 0x50:
        v16 = 2;
        goto label_67ab;
      case 0x52:
      case 0x72:
        v35 = '\x01';
        goto label_67ab;
      case 0x53:
        v1 = 1;
        v12 = optarg;
        goto label_67ab;
      case 0x54:
        v11 = 1;
        goto label_67ab;
      case 0x5a:
        if (dat_27120) { // branch-flip
          if (optarg) // branch-flip
            v15 = optarg;
          else {
            v27 = selabel_open(0,0,0);
            if (!v27) {
              v6 = gettext("warning: ignoring --context");
              error(0,*__errno_location(),v6);
            }
          }
        }
        else if (optarg)
          error(0,0,gettext("warning: ignoring --context; it requires an SELinux-enabled kernel"));
        goto label_67ab;
      case 0x61:
        v16 = 2;
        v28 = 1;
        v24 = 1;
        v25 = 1;
        v26 = 1;
        v30 = 1;
        if (dat_27120)
          v31 = '\x01';
        v33 = 1;
        v34 = 1;
        v35 = '\x01';
        goto label_67ab;
      case 0x62:
        v1 = 1;
        if (optarg)
          v13 = optarg;
        goto label_67ab;
      case 100:
        v28 = 1;
        v16 = 2;
        goto label_67ab;
      case 0x66:
        v21 = 1;
        goto label_67ab;
      case 0x69:
        v17 = 3;
        goto label_67ab;
      case 0x6c:
        v22 = '\x01';
        goto label_67ab;
      case 0x6e:
        v17 = 2;
        goto label_67ab;
      case 0x70:
label_65cc:
        v24 = 1;
        v25 = 1;
        v26 = 1;
        v30 = 1;
        goto label_67ab;
      case 0x73:
        v36 = '\x01';
        goto label_67ab;
      case 0x74:
        if (v14)
          error(1,0,gettext("multiple target directories specified"));
        v14 = optarg;
        goto label_67ab;
      case 0x75:
        v37 = 1;
        goto label_67ab;
      case 0x76:
        v38 = 1;
        goto label_67ab;
      case 0x78:
        v23 = 1;
        goto label_67ab;
      case 0x80:
        v29 = 0;
        goto label_67ab;
      case 0x81:
        v10 = 1;
        goto label_67ab;
      case 0x82:
        sub_606f(optarg,&v9,0);
        goto label_67ab;
      case 0x83:
        dat_27121 = 1;
        goto label_67ab;
      case 0x84:
        if (optarg) {
          sub_606f(optarg,&v9,1);
          v30 = 1;
          goto label_67ab;
        }
        goto label_65cc;
      case 0x85:
        if (optarg) { // branch-flip
          v8 = sub_f72a("--reflink",optarg,0x26460,0x1e3b0,4,dat_27028,1);
          v39 = *(int *)(v8 * 4 + 0x1e3b0);
        }
        else {
          v39 = 2;
        }
        break;
      case 0x86:
        v8 = sub_f72a("--sparse",optarg,0x26440,0x1e3a0,4,dat_27028,1);
        v18 = *(int *)(v8 * 4 + 0x1e3a0);
        break;
      case 0x87:
        dat_27122 = 1;
        break;
      case 0x88:
        goto label_65f6;
      
    }
  } while( true );
label_65f6:
  v20 = 1;
  goto label_67ab;
}


// Function: sub_6cbb @ 0x6cbb
int sub_6cbb(int a0,char *a1,void *a2,int a3)
{
  return fstatat(a0,a1,a2,a3);
}


// Function: sub_6d36 @ 0x6d36
int sub_6d36(unsigned int a0,unsigned long a1,unsigned long a2)
{
  int v1;
  int v2; // stack - 0xc
  
  v2 = fallocate(a0,3,a1,a2);
  if (v2 <= -1) {
    v1 = *__errno_location();
    if ((!sub_6bb9(v1)) && (*__errno_location() != 0x26))
      return v2;
    v2 = 0;
  }
  return v2;
}


// Function: sub_6d9e @ 0x6d9e
unsigned long sub_6d9e(unsigned int a0,unsigned long a1,char a2,long a3) // return-dupe x2
{
  long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = lseek(a0,a3,1);
  if (v1 <= -1) {
    v2 = sub_18a0f(4,a1);
    v3 = gettext("cannot lseek %s");
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  if ((a2) && ((int)sub_6d36(a0,v1 - a3,a3) <= -1)) {
    v2 = sub_18a0f(4,a1);
    v3 = gettext("error deallocating %s");
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  return 1;
}


// Function: sub_6ea8 @ 0x6ea8
unsigned long sub_6ea8(int a0,unsigned int a1,long *a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,unsigned long a7,unsigned long a8,unsigned long a9,long *a10,char *a11) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_6ea8
{
  long *v1;
  char v10; // stack - 0x74
  unsigned long v11; // stack - 0x70
  long v12; // stack - 0x68
  unsigned long v13; // stack - 0x60
  unsigned long v14; // stack - 0x58
  void *v15; // stack - 0x50
  void *v16; // stack - 0x48
  char v2;
  bool v3; // al
  bool v4; // al
  int v5;
  unsigned long v6;
  unsigned long v7;
  char *v8;
  unsigned long v9;
  
  v8 = a11;
  v1 = a10;
  v6 = a8;
  v7 = a7;
  *a11 = '\0';
  *a10 = 0;
  if ((!a4) && ((char)a6)) {
    while (a9) {
      v9 = a9;
      if (0x7fffffffc0000000 <= a9)
        v9 = 0x7fffffffc0000000;
      v12 = copy_file_range(a0,0,a1,0,v9,0);
      if (!v12) {
        if (*v1)
          return 1;
        break;
      }
      if (v12 <= -1) {
        if (*__errno_location() == 0x26) break;
        v5 = *__errno_location();
        if ((((sub_6bb9(v5)) || (*__errno_location() == 0x16)) || (*__errno_location() == 9)) || (((*__errno_location() == 0x12 || (*__errno_location() == 0x1a)) || ((*__errno_location() == 1 && (!*v1)))))) break;
        if (*__errno_location() != 4) {
          v6 = sub_18941(1,4,v6);
          v7 = sub_18941(0,4,v7);
          v8 = gettext("error copying %s to %s");
          error(0,*__errno_location(),v8,v7,v6);
          return 0;
        }
        v12 = 0;
      }
      a9 -= v12;
      *v1 = *v1 + v12;
    }
  }
  v10 = '\0';
  v11 = 0;
  while (a9) {
    if (!*a2) {
      v5 = getpagesize();
      *a2 = sub_1b8b1((long)v5,a3);
    }
    v15 = (void *)*a2;
    v9 = a3;
    if (a9 < a3)
      v9 = a9;
    v13 = read(a0,v15,v9);
    if (0 <= (long)v13) { // branch-flip
      if (!v13) break;
      a9 -= v13;
      *v1 = *v1 + v13;
      if (a4) // branch-flip
        v14 = a4;
      else {
        v14 = a3;
      }
      v16 = v15;
      for (; v2 = v10, v13; v13 = v13 - v14) {
        v9 = v13;
        if (v14 <= v13)
          v9 = v14;
        if ((a4) && (v9))
          v10 = sub_6b03(v15,v9);
        v3 = ((v10 != v2) && (v11));
        v4 = !(((v13 != v9) || (v10 == '\x01')) && (v9));
        v14 = v9;
        if ((v3) || (v4)) {
          if (!v3)
            v11 = v9 + v11;
          if (v2 != '\x01') { // branch-flip
            if (sub_14bff(a1,v16,v11) != v11) {
              v7 = sub_18a0f(4,v6);
              v8 = gettext("error writing %s");
              error(0,*__errno_location(),v8,v7);
              return 0;
            }
          }
          else if (sub_6d9e(a1,v6,a5,v11) != '\x01')
            return 0;
          v16 = v15;
          v11 = v9;
          if (v4) {
            if (!v9)
              v13 = 0;
            if (v3)
              v14 = 0;
            else {
              v11 = 0;
            }
          }
        }
        else {
          v11 = v9 + v11;
          if (v11 + 0x8000000000000000 < v9) {
            v7 = sub_18a0f(4,v7);
            error(0,0,gettext("overflow reading %s"),v7);
            return 0;
          }
        }
        v15 = (void *)((long)v15 + v14);
      }
      *v8 = v10;
    }
    else if (*__errno_location() != 4) {
      v7 = sub_18a0f(4,v7);
      v8 = gettext("error reading %s");
      error(0,*__errno_location(),v8,v7);
      return 0;
    }
  }
  if ((v10) && (sub_6d9e(a1,v6,a5,v11) != '\x01'))
    return 0;
  return 1;
}


// Function: sub_750c @ 0x750c
void sub_750c(int a0,unsigned int a1)
{
  ioctl(a0,0x40049409,(unsigned long)a1);
}


// Function: sub_7533 @ 0x7533
unsigned long sub_7533(unsigned int a0,unsigned long a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x28
  
  v3 = a1;
  if ((!dat_271c0) && (dat_271c0 = calloc(dat_27010,1), !dat_271c0)) {
    dat_271c0 = (void *)0x271e0;
    dat_27010 = 0x400;
  }
  while( true ) {
    if (!v3)
      return 1;
    v1 = dat_27010;
    if (v3 <= dat_27010)
      v1 = v3;
    v2 = sub_14bff(a0,dat_271c0,v1);
    if (v1 != v2) break;
    v3 -= v1;
  }
  return 0;
}


// Function: sub_75ed @ 0x75ed
unsigned long sub_75ed(unsigned int a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,long a5,long a6,int a7,unsigned int a8,unsigned long a9,unsigned long a10)
{
  unsigned long v1;
  long v10; // stack - 0x58
  unsigned long v11;
  long v12; // stack - 0x90
  char v13; // stack - 0x59
  long v14; // stack - 0x50
  long v15; // stack - 0x48
  long v16; // stack - 0x40
  long v17; // stack - 0x38
  long v18; // stack - 0x30
  long v19; // stack - 0x28
  char v2; // al
  char v3; // al
  unsigned char v4; // al
  int v5; // eax
  long v6; // rax
  unsigned long v7;
  char *v8; // rax
  char v9; // stack - 0x5a
  
  v1 = a10;
  v7 = a9;
  v2 = (char)a8;
  v14 = 0;
  v15 = 0;
  v16 = 0;
  v13 = '\x01';
  v12 = a5;
  do {
    if (v12 <= -1) {
label_78f7:
      if ((v16 < a6) || (v13)) {
        if (a7 != 1) { // branch-flip
          v5 = ftruncate(a1,a6);
          v4 = v5 != 0;
        }
        else {
          v6 = a6 - v16;
          v4 = sub_7533(a1,v6) ^ 1;
        }
        if (v4) {
          v7 = sub_18a0f(4,v1);
          v8 = gettext("failed to extend %s");
          error(0,*__errno_location(),v8,v7);
          return 0;
        }
      }
      if (((a7 == 3) && (v16 < a6)) && (v5 = sub_6d36(a1,v16,a6 - v16), v5 <= -1)) {
        v7 = sub_18a0f(4,v1);
        v8 = gettext("error deallocating %s");
        error(0,*__errno_location(),v8,v7);
        return 0;
      }
      return 1;
    }
    v17 = lseek(a0,v12,4);
    if (v17 <= -1) {
      if (*__errno_location() != 6) break;
      v17 = a6;
      if (a6 <= v12) {
        a6 = lseek(a0,0,2);
        if (a6 < 0) break;
        if (a6 <= v12) goto label_78f7;
        v17 = a6;
      }
    }
    if (a6 < v17)
      a6 = v17;
    if ((long)lseek(a0,v12,0) < 0) break;
    v13 = '\0';
    v18 = (v12 - v14) - v15;
    if (v18) {
      if (a7 != 1) { // branch-flip
        v3 = sub_6d9e(a1,v1,a7 == 3,v18);
        if (v3 != '\x01')
          return 0;
        v13 = '\x01';
      }
      else {
        v3 = sub_7533(a1,v18);
        if (v3 != '\x01') {
          v7 = sub_18ba1(0,3,v1);
          v8 = gettext("%s: write failed");
          error(0,*__errno_location(),v8,v7);
          return 0;
        }
      }
    }
    v15 = v17 - v12;
    v14 = v12;
    if (a7 != 1) // branch-flip
      v11 = a4;
    else {
      v11 = 0;
    }
    v19 = v15;
    v3 = sub_6ea8(a0,a1,a2,a3,v11,1,v2,v7,v1,v15,&v10,&v9);
    if (v3 != '\x01')
      return 0;
    v16 = v12 + v10;
    if (v10)
      v13 = v9;
    if (v10 < v19) {
      a6 = v16;
      goto label_78f7;
    }
    v12 = lseek(a0,v16,3);
  } while ((0 <= v12) || (*__errno_location() == 6));
  v7 = sub_18a0f(4,v7);
  v8 = gettext("cannot lseek %s");
  error(0,*__errno_location(),v8,v7);
  return 0;
}


// Function: sub_7a8e @ 0x7a8e
unsigned long sub_7a8e(long *a0,unsigned long *a1)
{
  unsigned long *v1; // stack - 0x18
  
  v1 = a1;
  while( true ) {
    if (!v1)
      return 0;
    if ((v1[1] == a0[1]) && (v1[2] == *a0)) break;
    v1 = (unsigned long *)*v1;
  }
  return 1;
}


// Function: sub_7ae9 @ 0x7ae9
unsigned int sub_7ae9(int a0) // return-dupe
{
  if ((a0 != 0x5f) && (a0 != 0x3d))
    return 0;
  return 1;
}


// Function: sub_7b11 @ 0x7b11
void sub_7b11(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  int v1;
  unsigned long v10; // stack - 0x98
  unsigned long v11; // stack - 0x90
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long v15; // stack - 0x58
  unsigned long v16; // stack - 0x48
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x18
  char v2; // al
  char v3 [16];
  unsigned int v4; // stack - 0xd8
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v2) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  v1 = *__errno_location();
  if (sub_7ae9(v1) == '\x01')
    return;
  v4 = 0x10;
  v5 = 0x30;
  v6 = &Stack0000000000000008;
  v7 = v3;
  sub_1ae51(0,*__errno_location(),a9,&v4);
}


// Function: sub_7c09 @ 0x7c09
void sub_7c09(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x90
  unsigned long v11; // stack - 0x88
  unsigned long v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x38
  unsigned long v17; // stack - 0x28
  unsigned long v18; // stack - 0x18
  char v2 [16];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xa8
  unsigned long v8; // stack - 0xa0
  unsigned long v9; // stack - 0x98
  
  if (v1) {
    v11 = a0;
    v12 = a1;
    v13 = a2;
    v14 = a3;
    v15 = a4;
    v16 = a5;
    v17 = a6;
    v18 = a7;
  }
  v7 = a10;
  v8 = a11;
  v9 = a12;
  v10 = a13;
  v3 = 0x10;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  sub_1ae51(0,*__errno_location(),a9,&v3);
}


// Function: sub_7cec @ 0x7cec
void sub_7cec(unsigned long a0,unsigned long a1)
{
  sub_18a0f(4,a1);
}


// Function: sub_7d13 @ 0x7d13
void sub_7d13(void)
{
  return;
}


// Function: sub_7d26 @ 0x7d26
unsigned long sub_7d26(char *a0,unsigned long a1) // return-dupe
{
  if ((strncmp(a0,"security.selinux",0x10)) && (attr_copy_check_permissions(a0,a1)))
    return 1;
  return 0;
}


// Function: sub_7d7e @ 0x7d7e
unsigned long sub_7d7e(unsigned long a0,int a1,unsigned long a2,int a3,long a4) // ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_7d7e
{
  bool v1; // al
  bool v2; // al
  void *v3; // rax
  void **v4;
  unsigned long v5; // rax
  void *v6; // stack - 0x28
  void *v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v1 = ((*(char *)(a4 + 0x31) != '\x01') || (*(char *)(a4 + 0x36)));
  v2 = !((v1) || (*(char *)(a4 + 0x37) == '\x01'));
  if ((*(char *)(a4 + 0x33)) || (*(long *)(a4 + 0x28)))
    v3 = sub_7d26;
  else {
    v3 = NULL;
  }
  if ((v1) || (v2)) {
    v6 = (v1) ? sub_7c09 : sub_7b11;
    v7 = sub_7cec;
    v8 = sub_7d13;
    v4 = &v6;
  }
  else {
    v4 = NULL;
  }
  if ((0 <= a1) && (0 <= a3)) {
    v5 = attr_copy_fd(a0,a1,a2,a3,v3,v4);
    v5 = CONCAT71((undefined7)((unsigned long)v5 >> 8),(int)v5 == 0);
  }
  else {
    v5 = attr_copy_file(a0,a2,v3,v4);
    v5 = CONCAT71((undefined7)((unsigned long)v5 >> 8),(int)v5 == 0);
  }
  return v5;
}


// Function: sub_7ee6 @ 0x7ee6
unsigned char sub_7ee6(unsigned long a0,long a1,unsigned int a2,long a3,char a4,unsigned long a5,unsigned long a6,unsigned long *a7,char *a8,char *a9) // early-return
{
  unsigned long *v1;
  unsigned long v10; // stack - 0x78
  unsigned char v11; // stack - 0x9a
  char v12; // stack - 0x99
  char *v13; // stack - 0x98
  char *v14; // stack - 0x90
  void *v15; // stack - 0x88
  void *v16; // stack - 0x80
  unsigned long v17; // stack - 0x70
  unsigned long v18; // stack - 0x68
  unsigned long v19; // stack - 0x60
  char *v2;
  unsigned long v20; // stack - 0x58
  unsigned long v21; // stack - 0x50
  unsigned long v22; // stack - 0x48
  unsigned long v23; // stack - 0x40
  unsigned long v24; // stack - 0x38
  unsigned long v25; // stack - 0x30
  unsigned long v26; // stack - 0x28
  unsigned char v3; // al
  unsigned long v4;
  char *v5;
  unsigned long v6; // rax
  long v7;
  char v8; // stack - 0x9d
  char v9; // stack - 0x9c
  
  v2 = a9;
  v5 = a8;
  v1 = a7;
  v4 = a6;
  v10 = *a7;
  v17 = a7[1];
  v18 = a7[2];
  v19 = a7[3];
  v20 = a7[4];
  v21 = a7[5];
  v22 = a7[6];
  v23 = a7[7];
  v24 = a7[8];
  v25 = a7[9];
  v26 = a7[10];
  v11 = 1;
  v14 = (char *)sub_199a1(a0,2);
  if (!v14) {
    v4 = sub_18a0f(4,a0);
    v5 = gettext("cannot access %s");
    error(0,*__errno_location(),v5,v4);
    return 0;
  }
  if (*(int *)((long)v1 + 4) == 3)
    v10 = CONCAT44(2,(unsigned int)v10);
  v12 = '\0';
  for (v13 = v14; *v13; v13 = &v13[v6 + 1]) {
    v15 = (void *)sub_11da5(a0,v13,0);
    v16 = (void *)sub_11da5(a1,v13,0);
    v9 = *v5;
    v3 = sub_abda(v15,v16,a2,(a3 - a1) + (long)v16,a4,a5,v4,&v10,0,&v9,&v8,&v7);
    v11 = (v3 & v11) != 0;
    *v2 = v8 || *v2;
    free(v16);
    free(v15);
    if (v8) break;
    v12 = v9 || v12;
    v6 = strlen(v13);
  }
  free(v14);
  *v5 = v12;
  return v11;
}


// Function: sub_820f @ 0x820f
int sub_820f(long a0,unsigned long a1,unsigned int a2,unsigned long a3,int a4,long a5,unsigned int a6,long a7) // return-dupe
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  int v4;
  unsigned int v5;
  unsigned long v6; // rax
  char *v7; // rax
  
  v1 = *(unsigned int *)(a5 + 0x1c);
  v2 = *(unsigned int *)(a5 + 0x20);
  if (((char)a6 != '\x01') && (((*(char *)(a0 + 0x1e) || (*(char *)(a0 + 0x18))) || (*(char *)(a0 + 0x39))))) {
    v3 = *(unsigned int *)(a7 + 0x18);
    if ((*(char *)(a0 + 0x1e)) || (*(char *)(a0 + 0x18)))
      v5 = *(unsigned int *)(a5 + 0x18);
    else {
      v5 = *(unsigned int *)(a0 + 0x10);
    }
    if (((unsigned short)(~(unsigned short)v5 | 0xe00) & v3 & 0xfff) && (sub_16e81(a1,a4,v3 & v5 & 0x1c0))) {
      if (sub_dc67(a0) != '\x01') {
        v6 = sub_18a0f(4,a1);
        v7 = gettext("clearing permissions for %s");
        error(0,*__errno_location(),v7,v6);
      }
      return -(unsigned int)*(unsigned char *)(a0 + 0x32);
    }
  }
  if (a4 != -1) { // branch-flip
    if (!fchown(a4,v1,v2))
      return 1;
    if ((*__errno_location() == 1) || (*__errno_location() == 0x16)) {
      v4 = *__errno_location();
      fchown(a4,0xffffffff,v2);
      *__errno_location() = v4;
    }
  }
  else {
    if (!sub_112c3(a2,a3,v1,v2))
      return 1;
    if ((*__errno_location() == 1) || (*__errno_location() == 0x16)) {
      v4 = *__errno_location();
      sub_112c3(a2,a3,0xffffffff,v2);
      *__errno_location() = v4;
    }
  }
  if (sub_dc1f(a0) == '\x01')
    return 0;
  v6 = sub_18a0f(4,a1);
  v7 = gettext("failed to preserve ownership for %s");
  error(0,*__errno_location(),v7,v6);
  if (!*(char *)(a0 + 0x32))
    return 0;
  return -1;
}


// Function: sub_84b0 @ 0x84b0
void sub_84b0(void)
{
  return;
}


// Function: sub_84c6 @ 0x84c6
unsigned long sub_84c6(unsigned long a0,unsigned long a1,unsigned int a2,char a3,long a4) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_84c6
{
  bool v1; // al
  bool v2; // al
  int v3;
  unsigned long v4;
  char *v5; // rax
  unsigned long v6; // stack - 0x28
  
  if (!*(char *)(a4 + 0x33)) {
    if (!*(long *)(a4 + 0x28))
      return 1;
    if (!a3)
      return 1;
    v4 = *(unsigned long *)(a4 + 0x28);
    if (0 <= (int)sub_e549(v4,a1,a2))
      return 1;
    v3 = *__errno_location();
    if (sub_6c97(v3) == '\x01')
      return 1;
    v4 = sub_18a0f(4,a1);
    v5 = gettext("failed to set default file creation context for %s");
    error(0,*__errno_location(),v5,v4);
    return 1;
  }
  v1 = ((*(char *)(a4 + 0x31) != '\x01') || (*(char *)(a4 + 0x34)));
  v2 = !((v1) || (*(char *)(a4 + 0x37) == '\x01'));
  if ((int)sub_19d8b(a0,&v6) <= -1) {
    if (!v1) {
      if (!v2) goto label_868a;
      v3 = *__errno_location();
      if (sub_7ae9(v3) == '\x01') goto label_868a;
    }
    v4 = sub_18a0f(4,a0);
    v5 = gettext("failed to get security context of %s");
    error(0,*__errno_location(),v5,v4);
label_868a:
    if (!*(char *)(a4 + 0x34))
      return 1;
    return 0;
  }
  v3 = setfscreatecon(v6);
  if (0 <= v3) {
    freecon(v6); // return-dupe
    return 1;
  }
  if (v1) {
label_85b3:
    v4 = sub_18e56(v6);
    v5 = gettext("failed to set default file creation context to %s");
    error(0,*__errno_location(),v5,v4);
  }
  else if (v2) {
    v3 = *__errno_location();
    if (sub_7ae9(v3) != '\x01') goto label_85b3;
  }
  if (*(char *)(a4 + 0x34)) {
    freecon(v6);
    return 0;
  }
  freecon(v6);
  return 1;
}


// Function: sub_8750 @ 0x8750
unsigned long sub_8750(unsigned long a0,char a1,long a2) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_8750
{
  int v1;
  bool v2; // al
  bool v3; // al
  unsigned long v4;
  char *v5; // rax
  
  v2 = ((*(char *)(a2 + 0x31) != '\x01') || (*(char *)(a2 + 0x34)));
  v3 = !((v2) || (*(char *)(a2 + 0x37) == '\x01'));
  v4 = *(unsigned long *)(a2 + 0x28);
  if (sub_ead7(v4,a0,a1) == '\x01')
    return 1;
  if (!v2) {
    if (!v3)
      return 0;
    v1 = *__errno_location();
    if (sub_7ae9(v1) == '\x01')
      return 0;
  }
  v4 = sub_18941(0,4,a0);
  v5 = gettext("failed to set the security context of %s");
  error(0,*__errno_location(),v5,v4);
  return 0;
}


// Function: sub_8868 @ 0x8868
void sub_8868(int a0,unsigned int a1,unsigned long a2,unsigned int a3) // return-dupe
{
  if (0 <= a0) {
    fchmod(a0,a3);
    return;
  }
  sub_11260(a1,a2,a3);
}


// Function: sub_88ae @ 0x88ae
unsigned long sub_88ae(unsigned int a0,long a1,long *a2)
{
  int v1;
  long v2;
  
  if ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000) {
    v2 = *(long *)(a1 + 0x30);
    if (v2 <= -1)
      v2 += 0x1ff;
    if (*(long *)(a1 + 0x40) < v2 >> 9) {
      *a2 = lseek(a0,0,3);
      if ((*a2 <= -1) && (*__errno_location() != 6)) {
        if (*__errno_location() != 0x16) {
          v1 = *__errno_location();
          if (sub_6bb9(v1) != '\x01')
            return 0;
        }
        return 2;
      }
      return 3;
    }
  }
  return 1;
}


// Function: sub_896f @ 0x896f
char sub_896f(unsigned long a0,char *a1,int a2,char *a3,long a4,unsigned int a5,unsigned int a6,char *a7,long *a8) // ternary x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_896f
{
  unsigned int v1;
  unsigned long v10;
  long v11; // stack - 0xc8
  long v12;
  char v13 [24];
  unsigned long v14; // stack - 0x1a8
  unsigned long v15; // stack - 0x1b0
  char v16; // stack - 0x1dd
  unsigned long v17; // stack - 0x1a0
  unsigned long v18; // stack - 0x178
  unsigned int v19;
  int v2;
  unsigned long v20;
  char v21 [16];
  char v22; // stack - 0x1dc
  char v23; // stack - 0x1db
  char v24; // stack - 0x1da
  char v25; // stack - 0x1d9
  int v26; // stack - 0x1d8
  int v27; // stack - 0x1d4
  unsigned int v28; // stack - 0x1d0
  unsigned int v29; // stack - 0x1cc
  char *v3;
  int v30; // stack - 0x1c8
  unsigned int v31; // stack - 0x1c4
  unsigned int v32; // stack - 0x1c0
  unsigned int v33; // stack - 0x1bc
  unsigned int v34; // stack - 0x1b8
  int v35; // stack - 0x1b4
  unsigned long v36; // stack - 0x198
  unsigned long v37; // stack - 0x190
  unsigned long v38; // stack - 0x188
  unsigned long v39; // stack - 0x180
  unsigned long v4;
  unsigned long v40; // stack - 0x170
  unsigned long v41; // stack - 0x168
  unsigned long v42; // stack - 0x160
  unsigned int v43; // stack - 0x140
  int v44; // stack - 0x13c
  int v45; // stack - 0x138
  unsigned long v46; // stack - 0x120
  long v47; // stack - 0xc0
  unsigned int v48; // stack - 0xb0
  unsigned long v49; // stack - 0x98
  char *v5;
  long *v6;
  char v7; // al
  unsigned int v8; // eax
  int *v9;
  
  v6 = a8;
  v3 = a7;
  v15 = 0;
  v28 = *(unsigned int *)&a8[3];
  v22 = 1;
  v23 = *(char *)(a4 + 0x31);
  v24 = *(char *)(a4 + 0x35) != '\0';
  v10 = (*(int *)(a4 + 4) != 2) ? 0 : 0x20000; // branch-flip
  v8 = sub_11785(a0,v10);
  v30 = v8;
  if ((int)v8 <= -1) {
    v9 = (unsigned long)sub_18a0f(4,a0);
    v10 = v9;
    v9 = (char *)gettext("cannot open %s for reading");
    v3 = (char *)v9;
    v9 = __errno_location();
    error(0,*v9,v3,v10);
    v7 = 0;
    return v7;
  }
  v8 = fstat(v8,&v11);
  if (v8) {
    v9 = (unsigned long)sub_18a0f(4,a0);
    v10 = v9;
    v9 = (char *)gettext("cannot fstat %s");
    v3 = (char *)v9;
    v9 = __errno_location();
    error(0,*v9,v3,v10);
    v22 = 0;
    goto label_9a84;
  }
  if ((v6[1] != v47) || (*v6 != v11)) {
    v9 = (unsigned long)sub_18a0f(4,a0);
    v10 = v9;
    v9 = (char *)gettext("skipping file %s, as it was replaced while being copied");
    error(0,0,(char *)v9,v10);
    v22 = 0;
    goto label_9a84;
  }
  if (*v3 != '\x01') { // branch-flip
    v8 = (v23) ? 0x201 : 1; // branch-flip
    v31 = v8;
    v8 = sub_16b35(a2,a3,v8);
    v26 = v8;
    v9 = __errno_location();
    v27 = *v9;
    if ((v26 < 0) || ((((!*(long *)(a4 + 0x28) && (!*(char *)(a4 + 0x33))) || (v7 = sub_8750(a1,0,a4), v7 == '\x01')) || (!*(char *)(a4 + 0x34))))) {
      if (((v26 <= -1) && (v27 != 2)) && (*(char *)(a4 + 0x16))) {
        v8 = unlinkat(a2,a3,0);
        if (v8) { // branch-flip
          v9 = __errno_location();
          if (*v9 != 2) {
            v9 = (unsigned long)sub_18a0f(4,a1);
            v10 = v9;
            v9 = (char *)gettext("cannot remove %s");
            v3 = (char *)v9;
            v9 = __errno_location();
            error(0,*v9,v3,v10);
            v22 = 0;
            goto label_9a84;
          }
        }
        else if (*(char *)(a4 + 0x3c)) {
          v9 = (unsigned long)sub_18a0f(4,a1);
          v10 = v9;
          v9 = (char *)gettext("removed %s\n");
          printf((char *)v9,v10);
        }
        v27 = 2;
      }
      if ((v26 <= -1) && (v27 == 2)) {
        if ((*(long *)(a4 + 0x28)) && (v7 = sub_84c6(a0,a1,a5,1,a4), v7 != '\x01')) {
          v22 = 0;
          goto label_9a84;
        }
        *v3 = '\x01';
      }
      goto label_8dca;
    }
    v22 = 0;
  }
  else {
label_8dca:
    if (*v3) { // branch-flip
      v8 = ~a6;
      v8 &= a5;
      v1 = v8;
      if ((v24) && (*(char *)(a4 + 0x1b) != '\x01'))
        v8 = 0x80;
      else {
        v8 = 0;
      }
      v8 |= v1;
      v32 = v8;
      v8 = ~a5;
      v8 &= v32;
      v33 = 0x41;
      v29 = v8;
      v8 = sub_16b35(a2,a3,0xc1,v32);
      v26 = v8;
      v9 = __errno_location();
      v27 = *v9;
      if ((((v26 <= -1) && (v27 == 0x11)) && (*(char *)(a4 + 0x18) != '\x01')) && (v9 = (long)readlinkat(a2,a3,&v12,1), 0 <= (long)v9)) {
        if (!*(char *)(a4 + 0x3e)) {
          v9 = (unsigned long)sub_18a0f(4,a1);
          v10 = v9;
          v9 = (char *)gettext("not writing through dangling symlink %s");
          error(0,0,(char *)v9,v10);
          v22 = 0;
          goto label_9a84;
        }
        v8 = sub_16b35(a2,a3,v33,v32);
        v26 = v8;
        v9 = __errno_location();
        v27 = *v9;
      }
      if (((v26 <= -1) && (v27 == 0x15)) && ((*a1 && (v9 = (unsigned long)strlen(a1), v9 = (char *)&a1[(long)v9 - 1], *(char *)v9 == '/'))))
        v27 = 0x14;
    }
    else {
      v29 = 0;
      a6 = 0;
    }
    if (v26 <= -1) {
      v9 = (unsigned long)sub_18a0f(4,a1);
      v10 = v9;
      v9 = (char *)gettext("cannot create regular file %s");
      error(0,v27,(char *)v9,v10);
      v22 = 0;
      goto label_9a84;
    }
    if ((v23) && (*(int *)(a4 + 0x44))) {
      v8 = sub_750c(v26,v30);
      if (v8) { // branch-flip
        if (*(int *)(a4 + 0x44) == 2) {
          v9 = (unsigned long)sub_18941(1,4,a0);
          v10 = v9;
          v9 = (unsigned long)sub_18941(0,4,a1);
          v4 = v9;
          v9 = (char *)gettext("failed to clone %s from %s");
          v3 = (char *)v9;
          v9 = __errno_location();
          error(0,*v9,v3,v4,v10);
          v22 = 0;
          goto label_9a22;
        }
      }
      else {
        v23 = '\0';
      }
    }
    if ((*(char *)(a4 + 0x1d) || v23) || v29) { // branch-flip
      v8 = fstat(v26,v13);
      if (v8) {
        v9 = (unsigned long)sub_18a0f(4,a1);
        v10 = v9;
        v9 = (char *)gettext("cannot fstat %s");
        v3 = (char *)v9;
        v9 = __errno_location();
        error(0,*v9,v3,v10);
        v22 = 0;
        goto label_9a22;
      }
    }
    else {
      v43 = 0;
    }
    v8 = v43 | v29;
    v34 = v8;
    if ((v8 != v43) && (v8 = sub_8868(v26,a2,a3,v8), v8))
      v29 = 0;
    if (v23) { // branch-flip
      v9 = (unsigned long)sub_6bc9();
      v36 = (unsigned long)v9;
      if ((1 <= (long)v46) && (v46 <= 0x2000000000000000))
        v9 = (int *)v46;
      else {
        v9 = (int *)0x200;
      }
      v37 = (unsigned long)v9;
      v8 = sub_88ae(v30,&v11,&v14);
      v2 = v8;
      v35 = v8;
      if (v8) { // branch-flip
        v8 = v43 & 0xf000;
        v7 = !((v8 != 0x8000) || ((*(int *)(a4 + 0xc) != 3 && ((*(int *)(a4 + 0xc) != 2 || (v2 == 1))))));
        v25 = v7;
        sub_11711(v30,0,0,2);
        if (v25 != '\x01') {
          v38 = 0x7fffffffffffffff;
          v9 = (unsigned long)sub_6bc9();
          v9 = (unsigned long)sub_10429(v9,v36,v38);
          v39 = (unsigned long)v9;
          v8 = v48 & 0xf000;
          if ((v8 == 0x8000) && (v49 < v36)) {
            v9 = (unsigned long)(v49 + 1);
            v36 = (unsigned long)v9;
          }
          v9 = (long)(v36 + v39);
          v9 = (unsigned long)((long)v9 - 1);
          v36 = (long)v9 - (unsigned long)v9 % v39;
          if ((!v36) || (v38 < v36))
            v36 = v39;
        }
        v16 = '\0';
        if (v35 != 3) { // branch-flip
          if (v25) // branch-flip
            v20 = v37;
          else {
            v20 = 0;
          }
          v7 = sub_6ea8(v30,v26,&v15,v36,v20,*(int *)(a4 + 0xc) == 3,*(int *)(a4 + 0x44) != 0,a0,a1,0xffffffffffffffff,&v17,&v16);
        }
        else {
          if (v25) // branch-flip
            v19 = *(unsigned int *)(a4 + 0xc);
          else {
            v19 = 1;
          }
          v7 = sub_75ed(v30,v26,&v15,v36,v37,v14,v49,v19,*(int *)(a4 + 0x44) != 0,a0,a1);
        }
        if (v7 != '\x01') // branch-flip
          v22 = 0;
        else {
          if ((!v16) || (v8 = ftruncate(v26,v17), 0 <= (int)v8)) goto label_9666;
          v9 = (unsigned long)sub_18a0f(4,a1);
          v10 = v9;
          v9 = (char *)gettext("failed to extend %s");
          v3 = (char *)v9;
          v9 = __errno_location();
          error(0,*v9,v3,v10);
          v22 = 0;
        }
      }
      else {
        v9 = (unsigned long)sub_18a0f(4,a0);
        v10 = v9;
        v9 = (char *)gettext("cannot lseek %s");
        v3 = (char *)v9;
        v9 = __errno_location();
        error(0,*v9,v3,v10);
        v22 = 0;
      }
    }
    else {
label_9666:
      if (*(char *)(a4 + 0x1f)) {
        v21 = sub_19e57(v6);
        v40 = SUB168(v21,8);
        v9 = SUB168(v21,0);
        v18 = v9;
        v21 = sub_19e8b(v6);
        v42 = SUB168(v21,8);
        v9 = SUB168(v21,0);
        v41 = v9;
        v8 = sub_118c1(v26,a2,a3,&v18,0);
        if (v8) {
          v9 = (unsigned long)sub_18a0f(4,a1);
          v10 = v9;
          v9 = (char *)gettext("preserving times for %s");
          v5 = (char *)v9;
          v9 = __errno_location();
          error(0,*v9,v5,v10);
          if (*(char *)(a4 + 0x32)) {
            v22 = 0;
            goto label_9a22;
          }
        }
      }
      if ((*(char *)(a4 + 0x1d)) && ((*(int *)((long)v6 + 0x1c) != v44 || ((int)v6[4] != v45)))) {
        v8 = sub_820f(a4,a1,a2,a3,v26,v6,*v3,v13);
        if (v8 == -1) {
          v22 = 0;
          goto label_9a22;
        }
        if (!v8)
          v28 &= 0xfffff1ff;
      }
      if (((v24) && (v7 = sub_7d7e(a0,v30,a1,v26,a4), v7 != '\x01')) && (*(char *)(a4 + 0x36)))
        v22 = 0;
      sub_84b0(a1,v26,v6);
      if ((*(char *)(a4 + 0x1e)) || (*(char *)(a4 + 0x18))) {
        v8 = sub_ec57(a0,v30,a1,v26,v28);
        if ((v8) && (*(char *)(a4 + 0x32)))
          v22 = 0;
      }
      else if (*(char *)(a4 + 0x39)) { // branch-flip
        v8 = sub_ed24(a1,v26,*(unsigned int *)(a4 + 0x10));
        if (v8)
          v22 = 0;
      }
      else if ((*(char *)(a4 + 0x20)) && (*v3)) {
        v8 = sub_dcaf();
        v8 = ~v8;
        v8 &= 0x1b6;
        v8 = sub_ed24(a1,v26,v8);
        if (v8)
          v22 = 0;
      }
      else if (a6 || v29) {
        v8 = sub_dcaf();
        v8 = ~v8;
        a6 &= v8;
        if (a6 || v29) {
          v8 = sub_dcaf();
          v8 = ~v8;
          v8 &= a5;
          v8 = sub_8868(v26,a2,a3,v8);
          if (v8) {
            v9 = (unsigned long)sub_18a0f(4,a1);
            v10 = v9;
            v9 = (char *)gettext("preserving permissions for %s");
            v3 = (char *)v9;
            v9 = __errno_location();
            error(0,*v9,v3,v10);
            if (*(char *)(a4 + 0x32))
              v22 = 0;
          }
        }
      }
    }
  }
label_9a22:
  v8 = close(v26);
  if ((int)v8 <= -1) {
    v9 = (unsigned long)sub_18a0f(4,a1);
    v10 = v9;
    v9 = (char *)gettext("failed to close %s");
    v3 = (char *)v9;
    v9 = __errno_location();
    error(0,*v9,v3,v10);
    v22 = 0;
  }
label_9a84:
  v8 = close(v30);
  if ((int)v8 <= -1) {
    v9 = (unsigned long)sub_18a0f(4,a0);
    v10 = v9;
    v9 = (char *)gettext("failed to close %s");
    v3 = (char *)v9;
    v9 = __errno_location();
    error(0,*v9,v3,v10);
    v22 = 0;
  }
  sub_eda5(v15);
  v7 = v22;
  return v7;
}


// Function: sub_9b18 @ 0x9b18
char sub_9b18(char *a0,long *a1,int a2,char *a3,long *a4,int *a5,char *a6) // early-return x2, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_9b18
{
  bool v1;
  long v10; // stack - 0x140
  unsigned int v11; // stack - 0x130
  long v12; // stack - 0xb0
  long v13; // stack - 0xa8
  long v14; // stack - 0xa0
  long v15; // stack - 0x98
  long v16; // stack - 0x90
  long v17; // stack - 0x88
  long v18; // stack - 0x80
  long v19; // stack - 0x78
  char *v2;
  long v20; // stack - 0x70
  long v21; // stack - 0x68
  long v22; // stack - 0x60
  long v23; // stack - 0x58
  long v24; // stack - 0x50
  long v25; // stack - 0x48
  long v26; // stack - 0x40
  long v27; // stack - 0x38
  long v28; // stack - 0x30
  char v3; // al
  unsigned int v4; // eax
  char *v5; // rax
  long v6; // stack - 0x148
  long v7; // stack - 0xb8
  long *v8; // stack - 0x160
  long *v9; // stack - 0x158
  
  v3 = !((a1[1] != a4[1]) || (*a1 != *a4));
  v1 = (bool)v3;
  *a6 = 0;
  if (((bool)v3) && (*(char *)((long)a5 + 0x17))) {
    *a6 = 1;
    v3 = 1;
    return v3;
  }
  if (a5[1] != 2) { // branch-flip
    if (!(bool)v3) {
      v3 = 1;
      return v3;
    }
    v4 = fstatat(a2,a3,&v6,0x100);
    if ((v4) || (v4 = lstat(a0,&v7), v4)) {
      v3 = 1;
      return v3;
    }
    v8 = &v7;
    v9 = &v6;
    v3 = !((v12 != v10) || (v7 != v6));
    v1 = (bool)v3;
    v4 = (unsigned int)v14 & 0xf000;
    if (((v4 == 0xa000) && (v4 = v11 & 0xf000, v4 == 0xa000)) && (*(char *)((long)a5 + 0x15))) {
      v3 = 1;
      return v3;
    }
  }
  else {
    v4 = *(unsigned int *)&a1[3] & 0xf000;
    if ((v4 == 0xa000) && (v4 = *(unsigned int *)&a4[3] & 0xf000, v4 == 0xa000)) {
      v3 = sub_19370(0xffffff9c,a0,a2,a3);
      if (v3 != '\x01') {
        if (*a5) {
          v3 = 1;
          return v3;
        }
        if (v1) {
          *a6 = 1;
          v3 = (char)a5[6] == '\0';
          return v3;
        }
      }
      v3 = v3 == '\0';
      return v3;
    }
    v8 = a1;
    v9 = a4;
  }
  if (*a5) {
    if (v1) {
      v3 = sub_19370(0xffffff9c,a0,a2,a3);
      v3 = v3 == '\0';
      return v3;
    }
    if ((((char)a5[6] != '\x01') && (a5[1] != 2)) && ((v4 = *(unsigned int *)&v8[3] & 0xf000, v4 == 0xa000 && (v4 = *(unsigned int *)&v9[3] & 0xf000, v4 != 0xa000)))) {
      v3 = 0;
      return v3;
    }
    v3 = 1;
    return v3;
  }
  if (((char)a5[6]) || (*(char *)((long)a5 + 0x15))) {
    v4 = *(unsigned int *)&v9[3] & 0xf000;
    if (v4 == 0xa000) {
      v3 = 1;
      return v3;
    }
    if (((v1) && (2 <= (unsigned long)v9[2])) && (v3 = sub_19370(0xffffff9c,a0,a2,a3), v3 != '\x01')) {
      v3 = (char)a5[6] == '\0';
      return v3;
    }
  }
  v4 = *(unsigned int *)&v8[3] & 0xf000;
  if ((v4 != 0xa000) && (v4 = *(unsigned int *)&v9[3] & 0xf000, v4 != 0xa000)) {
    if ((v8[1] != v9[1]) || (*v8 != *v9)) {
      v3 = 1;
      return v3;
    }
    if (*(char *)((long)a5 + 0x17)) {
      *a6 = 1;
      v3 = 1;
      return v3;
    }
  }
  if ((((char)a5[6]) && (v4 = *(unsigned int *)&a1[3] & 0xf000, v4 == 0xa000)) && ((2 <= (unsigned long)v9[2] && (v5 = canonicalize_file_name(a0), v2 = v5, v5)))) {
    v3 = sub_19370(0xffffff9c,v5,a2,a3);
    free(v2);
    v3 = v3 == '\0';
    return v3;
  }
  if ((*(char *)((long)a5 + 0x3a)) && (v4 = *(unsigned int *)&v9[3] & 0xf000, v4 == 0xa000)) {
    v3 = 1;
    return v3;
  }
  if (a5[1] == 2) {
    v4 = *(unsigned int *)&v8[3] & 0xf000;
    if (v4 != 0xa000) { // branch-flip
      v7 = *v8;
      v12 = v8[1];
      v13 = v8[2];
      v14 = v8[3];
      v15 = v8[4];
      v16 = v8[5];
      v17 = v8[6];
      v18 = v8[7];
      v19 = v8[8];
      v20 = v8[9];
      v21 = v8[10];
      v22 = v8[0xb];
      v23 = v8[0xc];
      v24 = v8[0xd];
      v25 = v8[0xe];
      v26 = v8[0xf];
      v28 = v8[0x11];
      v27 = v8[0x10];
    }
    else {
      v4 = stat(a0,&v7);
      if (v4) {
        v3 = 1;
        return v3;
      }
    }
    v4 = *(unsigned int *)&v9[3] & 0xf000;
    if (v4 != 0xa000) { // branch-flip
      v6 = *v9;
      v10 = v9[1];
    }
    else {
      v4 = fstatat(a2,a3,&v6,0);
      if (v4) {
        v3 = 1;
        return v3;
      }
    }
    if ((v12 != v10) || (v7 != v6)) {
      v3 = 1;
      return v3;
    }
    if (*(char *)((long)a5 + 0x17)) {
      v4 = *(unsigned int *)&v9[3] & 0xf000;
      *a6 = v4 != 0xa000;
      v3 = 1;
      return v3;
    }
  }
  v3 = 0;
  return v3;
}


// Function: sub_a37a @ 0xa37a
unsigned int sub_a37a(unsigned int a0,unsigned long a1,unsigned int a2)
{
  if ((((a2 & 0xf000) != 0xa000) && (!sub_1b86c())) && (faccessat(a0,a1,2,0x200)))
    return 0;
  return 1;
}


// Function: sub_a3d5 @ 0xa3d5
void sub_a3d5(long a0,unsigned long a1,unsigned int a2,unsigned long a3,long a4)
{
  unsigned int v1;
  unsigned int v2;
  unsigned long v3;
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  char v7 [9];
  char v8; // stack - 0x32
  
  v1 = *(unsigned int *)(a4 + 0x18);
  if (sub_a37a(a2,a3,v1) != '\x01') { // branch-flip
    sub_11bad(*(unsigned int *)(a4 + 0x18),&v6);
    v8 = 0;
    v2 = *(unsigned int *)(a4 + 0x18);
    v4 = sub_18a0f(4,a1);
    v3 = dat_27608;
    if (((*(char *)(a0 + 0x18)) || (*(char *)(a0 + 0x15))) || (*(char *)(a0 + 0x16)))
      v5 = gettext("%s: replace %s, overriding mode %04lo (%s)? ");
    else {
      v5 = gettext("%s: unwritable %s (mode %04lo, %s); try anyway? ");
    }
    fprintf(stderr,v5,v3,v4,(unsigned long)(v2 & 0xfff),v7);
  }
  else {
    v4 = sub_18a0f(4,a1);
    v3 = dat_27608;
    fprintf(stderr,gettext("%s: overwrite %s? "),v3,v4);
  }
  sub_1c30f();
}


// Function: sub_a549 @ 0xa549
void sub_a549(long a0)
{
  *(unsigned long *)(a0 + 0x48) = sub_1563d(0x3d,0,sub_167c7,sub_1674b,sub_16893);
  if (!*(long *)(a0 + 0x48))
    sub_1bf79(); // no-return
}


// Function: sub_a5a6 @ 0xa5a6
void sub_a5a6(long a0)
{
  *(unsigned long *)(a0 + 0x50) = sub_1563d(0x3d,0,sub_1671d,sub_1674b,sub_16893);
  if (!*(long *)(a0 + 0x50))
    sub_1bf79(); // no-return
}


// Function: sub_a603 @ 0xa603
unsigned int sub_a603(long a0,unsigned long a1,unsigned int a2,unsigned long a3,long a4)
{
  unsigned int v1;
  
  if (!*(char *)(a0 + 0x18))
    __assert_fail("x->move_mode","src/copy.c",0x710,"abandon_move"); // no-return
  if (*(int *)(a0 + 8) != 2) { // branch-flip
    if (*(int *)(a0 + 8) != 3) { // branch-flip
      if ((*(int *)(a0 + 8) == 4) && (*(char *)(a0 + 0x3d))) {
        v1 = *(unsigned int *)(a4 + 0x18);
        if (sub_a37a(a2,a3,v1) != '\x01') goto label_a6a5;
      }
    }
    else {
label_a6a5:
      if (sub_a3d5(a0,a1,a2,a3,a4) != '\x01') {
        v1 = 1;
        return v1;
      }
    }
    v1 = 0;
  }
  else {
    v1 = 1;
  }
  return v1;
}


// Function: sub_a6db @ 0xa6db
void sub_a6db(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  
  v1 = sub_18941(1,4,a1);
  printf("%s -> %s",sub_18941(0,4,a0),v1);
  if (a2) {
    v1 = sub_18a0f(4,a2);
    printf(gettext(" (backup: %s)"),v1);
  }
  putchar_unlocked(10);
}


// Function: sub_a788 @ 0xa788
void sub_a788(void)
{
  char *v1; // rax
  
  if (setfscreatecon(0)) {
    v1 = gettext("failed to restore the default file creation context");
    error(1,*__errno_location(),v1);
    return;
  }
}


// Function: sub_a7d7 @ 0xa7d7
void sub_a7d7(void *a0,long a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  v1 = a1 - (long)a0;
  v2 = strlen(a2) + 1;
  v3 = (void *)sub_1b967(v2 + v1);
  memcpy((void *)(v1 + (long)v3),a2,v2);
  memcpy(v3,a0,v1);
}


// Function: sub_a85d @ 0xa85d
unsigned long sub_a85d(void *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned int a4,unsigned long a5,unsigned int a6,unsigned int a7,unsigned int a8) // return-dupe, ternary
{
  int v1; // eax
  unsigned long v2; // rax
  char v3; // dl
  unsigned long v4; // rax
  void *v5; // stack - 0x30
  void *v6; // stack - 0x20
  
  v3 = (char)a7;
  v4 = ((char)a8) ? 0x400 : 0; // branch-flip
  v1 = sub_e069(a1,a2,a4,a5,v4,(char)a6,0xffffffffffffffff);
  if (1 <= v1) {
    v6 = NULL;
    v5 = a0;
    if (!a0) {
      v6 = (void *)sub_a7d7(a3,a5,a2);
      v5 = v6;
    }
    v4 = sub_18941(1,4,v5);
    v2 = sub_18941(0,4,a3);
    error(0,v1,gettext("cannot create hard link %s to %s"),v2,v4);
    free(v6);
    return 0;
  }
  if ((v1 <= -1) && (v3)) {
    v4 = sub_18a0f(4,a3);
    printf(gettext("removed %s\n"),v4);
  }
  return 1;
}


// Function: sub_a9cc @ 0xa9cc
unsigned int sub_a9cc(long a0,char a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_a9cc
{
  unsigned int v1; // eax
  
  v1 = !((*(int *)(a0 + 4) != 4) && ((*(int *)(a0 + 4) != 3 || (!a1))));
  return v1;
}


// Function: sub_aa08 @ 0xaa08
unsigned int sub_aa08(char *a0,long *a1,int a2,char *a3) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_aa08
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  long v7; // stack - 0xb8
  long v8; // stack - 0xb0
  
  v3 = strlen(a0);
  v4 = (char *)sub_10369(a3);
  v5 = strlen(v4);
  v6 = strlen(dat_275e8);
  if (((v3 == v6 + v5) && (!memcmp(a0,v4,v5))) && (v1 = strcmp(&a0[v5],dat_275e8), v4 = dat_275e8, !v1)) {
    v4 = (char *)sub_a7d7(a3,&a3[strlen(a3)],v4);
    v1 = fstatat(a2,v4,&v7,0);
    free(v4);
    v2 = !(((v1) || (a1[1] != v8)) || (*a1 != v7));
    return v2;
  }
  return 0;
}


// Function: sub_abda @ 0xabda
unsigned char sub_abda(char *a0,char *a1,int a2,char *a3,unsigned int a4,long *a5,unsigned long a6,int *a7,unsigned int a8,char *a9,char *a10,unsigned char *a11) // ternary x7, warn: iteboolean: re-rolled 4 0/1 select diamond(s) to a boolean assignment in sub_abda
{
  unsigned char v1;
  unsigned char v10; // al
  unsigned int v11;
  int *v12;
  int v13;
  char v14 [8];
  long v15; // stack - 0x278
  long v16; // stack - 0x1e8
  char v17; // stack - 0x343
  long v18; // stack - 0xc8
  long v19; // stack - 0x158
  char v2;
  bool v20; // stack - 0x344
  char *v21; // rsp
  char *v22; // rsp
  char *v23;
  char *v24; // rsp
  char *v25;
  char v26 [16];
  unsigned char *v27; // stack - 0x3a0
  char *v28; // stack - 0x398
  char *v29; // stack - 0x390
  unsigned long v3;
  char v30; // stack - 0x384
  int *v31; // stack - 0x380
  unsigned long v32; // stack - 0x378
  long *v33; // stack - 0x370
  char *v34; // stack - 0x368
  unsigned int v35; // stack - 0x360
  int v36; // stack - 0x35c
  char *v37; // stack - 0x358
  char *v38; // stack - 0x350
  char v39; // stack - 0x342
  long v4;
  char v40; // stack - 0x341
  char v41; // stack - 0x340
  char v42; // stack - 0x33f
  char v43; // stack - 0x33e
  char v44; // stack - 0x33d
  unsigned char v45; // stack - 0x33c
  char v46; // stack - 0x33b
  unsigned char v47; // stack - 0x33a
  char v48; // stack - 0x339
  unsigned int v49; // stack - 0x338
  char v5;
  unsigned int v50; // stack - 0x334
  unsigned int v51; // stack - 0x330
  int v52; // stack - 0x32c
  int v53; // stack - 0x328
  int v54; // stack - 0x324
  unsigned int v55; // stack - 0x320
  unsigned int v56; // stack - 0x31c
  unsigned int v57; // stack - 0x318
  unsigned int v58; // stack - 0x314
  unsigned int v59; // stack - 0x310
  unsigned int v6;
  unsigned int v60; // stack - 0x30c
  int v61; // stack - 0x308
  unsigned int v62; // stack - 0x304
  unsigned int v63; // stack - 0x300
  unsigned int v64; // stack - 0x2fc
  long v65; // stack - 0x2f8
  void *v66; // stack - 0x2f0
  long *v67; // stack - 0x2e8
  char *v68; // stack - 0x2e0
  char *v69; // stack - 0x2d8
  char *v7;
  unsigned long v70; // stack - 0x2d0
  char *v71; // stack - 0x2c8
  char *v72; // stack - 0x2c0
  unsigned long v73; // stack - 0x2b8
  unsigned long v74; // stack - 0x2b0
  void *v75; // stack - 0x2a8
  char *v76; // stack - 0x2a0
  char *v77; // stack - 0x298
  char *v78; // stack - 0x290
  unsigned long *v79; // stack - 0x288
  unsigned long v8;
  long v80; // stack - 0x280
  unsigned long v81; // stack - 0x270
  unsigned long v82; // stack - 0x268
  unsigned int v83; // stack - 0x260
  int v84; // stack - 0x25c
  int v85; // stack - 0x258
  unsigned long v86; // stack - 0x250
  unsigned long v87; // stack - 0x248
  unsigned long v88; // stack - 0x1e0
  unsigned long v89; // stack - 0x1d8
  unsigned long v9;
  unsigned int v90; // stack - 0x1d0
  int v91; // stack - 0x1cc
  int v92; // stack - 0x1c8
  unsigned long v93; // stack - 0x1b8
  long v94; // stack - 0x150
  long v95; // stack - 0xc0
  unsigned long v96; // stack - 0xb8
  unsigned long v97; // stack - 0xb0
  
  v25 = v14;
  v21 = v14;
  v22 = v14;
  v32 = a6;
  v31 = a7;
  v10 = (char)a8;
  v29 = a9;
  v28 = a10;
  v27 = a11;
  v39 = '\0';
  v65 = 0;
  v66 = NULL;
  v41 = '\0';
  v42 = '\0';
  v43 = '\0';
  v20 = 0 < (int)a4;
  *a10 = 0;
  v52 = a7[0x10];
  v30 = v10;
  v33 = a5;
  v34 = a3;
  v35 = a4;
  v36 = a2;
  v37 = a1;
  v38 = a0;
  if ((char)a7[6]) {
    if (v52 <= -1) {
      v35 = a4;
      v11 = sub_18ece(0xffffff9c,a0,a2,a3,1);
      if (v11) { // branch-flip
        v12 = __errno_location();
        v11 = *v12;
      }
      else {
        v11 = 0;
      }
      v52 = v11;
    }
    v20 = v52 == 0;
    *v27 = v20;
    v11 = (unsigned int)*v27;
    v35 = v11;
  }
  if (v52) { // branch-flip
    v10 = ((v52 != 0x11) || (v31[2] != 2));
  }
  else {
    v10 = *(unsigned char *)((long)v31 + 0x3f) ^ 1;
  }
  if (v10) {
    if (v52) { // branch-flip
      v11 = -100;
      v12 = (int *)v38;
      v54 = v11;
      v68 = v38;
    }
    else {
      v12 = (int *)v34;
      v54 = v36;
      v68 = v37;
    }
    v69 = (char *)v12;
    v11 = (v31[1] != 2) ? 0 : 0x100; // branch-flip
    v55 = v11;
    v11 = sub_6cbb(v54,v69,&v15,v11);
    if (v11) {
      v12 = (unsigned long)sub_18a0f(4,v68);
      v3 = v12;
      v12 = (char *)gettext("cannot stat %s");
      v7 = (char *)v12;
      v12 = __errno_location();
      error(0,*v12,v7,v3);
      v10 = 0;
      return v10;
    }
    v49 = v83;
    v11 = v83 & 0xf000;
    if ((v11 == 0x4000) && ((char)v31[0xe] != '\x01')) {
      v12 = (unsigned long)sub_18a0f(4,v38);
      v3 = v12;
      v12 = (*(char *)((long)v31 + 0x19) != '\x01') ? (char *)gettext("-r not specified; omitting directory %s") : (char *)gettext("omitting directory %s"); // branch-flip
      error(0,0,(char *)v12,v3);
      v10 = 0;
      return v10;
    }
  }
  if ((v30) && (*(long *)&v31[0x14])) {
    v11 = v49 & 0xf000;
    if ((v11 != 0x4000) && ((!*v31 && (v10 = sub_11a7b(*(unsigned long *)&v31[0x14],v38,&v15), v10)))) {
      v12 = (unsigned long)sub_18a0f(4,v38);
      v3 = v12;
      v12 = (char *)gettext("warning: source file %s specified more than once");
      error(0,0,(char *)v12,v3);
      v10 = 1;
      return v10;
    }
    sub_119db(*(unsigned long *)&v31[0x14],v38,&v15);
  }
  v10 = sub_a9cc(v31,v30);
  v45 = v10;
  v23 = v14;
  if ((int)v35 <= 0) {
    if ((v52 != 0x11) || (v31[2] != 2)) {
      v11 = v49 & 0xf000;
      v10 = ((((v11 != 0x8000) && ((((char)v31[5] != '\x01' || (v11 = v49 & 0xf000, v11 == 0x4000)) || (v11 = v49 & 0xf000, v11 == 0xa000)))) || ((((char)v31[6] || (*(char *)((long)v31 + 0x3a))) || (*(char *)((long)v31 + 0x17))))) || ((*v31 || (*(char *)((long)v31 + 0x15)))));
      v5 = v10;
      v46 = v10;
      v11 = (v10) ? 0x100 : 0; // branch-flip
      v56 = v11;
      if ((v5 != '\x01') && ((int)v35 <= -1))
        v20 = 1;
      else {
        v11 = sub_6cbb(v36,v34,&v16,v11);
        if (v11) { // branch-flip
          v12 = __errno_location();
          if ((*v12 != 0x28) || (!*(char *)((long)v31 + 0x16))) {
            v12 = __errno_location();
            if (*v12 != 2) {
              v12 = (unsigned long)sub_18a0f(4,v37);
              v3 = v12;
              v12 = (char *)gettext("cannot stat %s");
              v7 = (char *)v12;
              v12 = __errno_location();
              error(0,*v12,v7,v3);
              v10 = 0;
              return v10;
            }
            v20 = 1;
          }
        }
        else {
          v43 = v46;
          v52 = 0x11;
        }
      }
    }
    v23 = v14;
    if (v52 == 0x11) {
      v17 = '\0';
      if ((v31[2] != 2) && (v10 = sub_9b18(v38,&v15,v36,v34,&v16,v31,&v17), v10 != '\x01')) {
        v12 = (unsigned long)sub_18941(1,4,v37);
        v3 = v12;
        v12 = (unsigned long)sub_18941(0,4,v38);
        v8 = v12;
        v12 = (char *)gettext("%s and %s are the same file");
        error(0,0,(char *)v12,v8,v3);
        v10 = 0;
        return v10;
      }
      if ((*(char *)((long)v31 + 0x3b)) && (v11 = v49 & 0xf000, v11 != 0x4000)) {
        v11 = ((*(char *)((long)v31 + 0x1f)) && (((char)v31[6] != '\x01' || (v16 != v15))));
        v57 = v11;
        v11 = sub_1a52e(v36,v34,&v16,&v15,v11);
        if (0 <= (int)v11) {
          if (v27)
            *v27 = 1;
          v12 = (long)sub_de9c(v34,v81,v15);
          v65 = (long)v12;
          if ((!v12) || (v10 = sub_a85d(0,v36,v12,v37,v36,v34,1,(char)v31[0xf],v45), v10 == '\x01')) {
            v10 = 1;
            return v10;
          }
          goto label_d821;
        }
      }
      if ((char)v31[6]) { // branch-flip
        v10 = sub_a603(v31,v37,v36,v34,&v16);
        if (v10) {
          if (v27)
            *v27 = 1;
          v10 = 1;
          return v10;
        }
      }
      else {
        v11 = v49 & 0xf000;
        if ((v11 != 0x4000) && ((v31[2] == 2 || ((v31[2] == 3 && (v10 = sub_a3d5(v31,v37,v36,v34,&v16), v10 != '\x01')))))) {
          v10 = 1;
          return v10;
        }
      }
      if (v17) {
        v10 = 1;
        return v10;
      }
      v11 = v90 & 0xf000;
      if (v11 != 0x4000) {
        v11 = v49 & 0xf000;
        if ((v11 == 0x4000) && ((!(char)v31[6] || (!*v31)))) {
          v12 = (unsigned long)sub_18941(1,4,v38);
          v3 = v12;
          v12 = (unsigned long)sub_18941(0,4,v37);
          v8 = v12;
          v12 = (char *)gettext("cannot overwrite non-directory %s with directory %s");
          error(0,0,(char *)v12,v8,v3);
          v10 = 0;
          return v10;
        }
        if (((v30) && (*v31 != 3)) && (v10 = sub_11a7b(*(unsigned long *)&v31[0x12],v34,&v16), v10)) {
          v12 = (unsigned long)sub_18941(1,4,v38);
          v3 = v12;
          v12 = (unsigned long)sub_18941(0,4,v37);
          v8 = v12;
          v12 = (char *)gettext("will not overwrite just-created %s with %s");
          error(0,0,(char *)v12,v8,v3);
          v10 = 0;
          return v10;
        }
      }
      v11 = v49 & 0xf000;
      if (((v11 != 0x4000) && (v11 = v90 & 0xf000, v11 == 0x4000)) && ((!(char)v31[6] || (!*v31)))) {
        v12 = (unsigned long)sub_18a0f(4,v37);
        v3 = v12;
        v12 = (char *)gettext("cannot overwrite directory %s with non-directory");
        error(0,0,(char *)v12,v3);
        v10 = 0;
        return v10;
      }
      if ((((char)v31[6]) && (v11 = v83 & 0xf000, v11 == 0x4000)) && ((v11 = v90 & 0xf000, v11 != 0x4000 && (!*v31)))) {
        v12 = (unsigned long)sub_18ba1(0,3,v37);
        v3 = v12;
        v12 = (unsigned long)sub_18ba1(0,3,v38);
        v8 = v12;
        v12 = (char *)gettext("cannot move directory onto non-directory: %s -> %s");
        error(0,0,(char *)v12,v8,v3);
        v10 = 0;
        return v10;
      }
      if (*v31) {
        v12 = (unsigned long)sub_10369(v38);
        v70 = v12;
        v10 = sub_6aa4(v12);
        if ((v10 != '\x01') && (((char)v31[6] || (v11 = v90 & 0xf000, v11 != 0x4000)))) {
          if ((*v31 != 3) && (v10 = sub_aa08(v70,&v15,v36,v34), v10)) {
            v12 = ((char)v31[6]) ? (char *)gettext("backing up %s might destroy source;  %s not moved") : (char *)gettext("backing up %s might destroy source;  %s not copied"); // branch-flip
            v71 = (char *)v12;
            v12 = (unsigned long)sub_18941(1,4,v38);
            v3 = v12;
            v12 = (unsigned long)sub_18941(0,4,v37);
            error(0,0,v71,v12,v3);
            v10 = 0;
            return v10;
          }
          v12 = (char *)sub_1021f(v36,v34,*v31);
          v72 = (char *)v12;
          if (v12) { // branch-flip
            v12 = (unsigned long)((long)v34 - (long)v37);
            v73 = (unsigned long)v12;
            v12 = (unsigned long)strlen(v72);
            v12 = (unsigned long)((long)v12 + 1);
            v9 = (unsigned long)v12;
            v74 = (unsigned long)v12;
            v12 = (long)((long)v12 + v73);
            v12 = (unsigned long)(((long)v12 + 0x17U) / 0x10);
            v12 = (unsigned long)((long)v12 * 0x10);
            for (; v21 != &v14[-((unsigned long)v12 & 0xfffffffffffff000)]; v21 = &v21[-0x1000]) {
              *(unsigned long *)&v21[-8] = *(unsigned long *)&v21[-8];
            }
            v4 = -(unsigned long)((unsigned int)v12 & 0xfff);
            v22 = &v21[v4];
            if ((unsigned long)v12 & 0xfff) {
              v11 = (unsigned int)v12 & 0xfff;
              v12 = (unsigned long)(unsigned long)v11;
              v12 = (long)((long)v12 - 8);
              v12 = (unsigned long *)&v21[(long)v12 + v4];
              *(unsigned long *)v12 = *(unsigned long *)v12;
            }
            v12 = (char *)&v21[v4 + 0xf];
            v12 = (void *)((unsigned long)v12 & 0xfffffffffffffff0);
            v66 = v12;
            *(unsigned long *)&v21[v4 + -8] = 0xba03;
            v12 = mempcpy(v12,v37,v73,v21[v4 + -8]);
            *(unsigned long *)&v21[v4 + -8] = 0xba1b;
            memcpy(v12,v72,v9,v21[v4 + -8]);
            *(unsigned long *)&v21[v4 + -8] = 0xba2a;
            free(v72,v21[v4 + -8]);
          }
          else {
            v12 = __errno_location();
            if (*v12 != 2) {
              v12 = (unsigned long)sub_18a0f(4,v37);
              v3 = v12;
              v12 = (char *)gettext("cannot backup %s");
              v7 = (char *)v12;
              v12 = __errno_location();
              error(0,*v12,v7,v3);
              v10 = 0;
              return v10;
            }
          }
          v20 = 1;
          v23 = v22;
          goto label_bbf1;
        }
      }
      v11 = v90 & 0xf000;
      v23 = v14;
      if (((v11 != 0x4000) && (v23 = v14, (char)v31[6] != '\x01')) && ((*(char *)((long)v31 + 0x15) || ((v23 = v14, *(char *)((long)v31 + 0x31) && ((((char)v31[0xc] && (2 <= v89)) || ((v23 = v14, v31[1] == 2 && (v11 = v83 & 0xf000, v23 = v14, v11 != 0x8000)))))))))) {
        v11 = unlinkat(v36,v34,0);
        if ((v11) && (v12 = __errno_location(), *v12 != 2)) {
          v12 = (unsigned long)sub_18a0f(4,v37);
          v3 = v12;
          v12 = (char *)gettext("cannot remove %s");
          v7 = (char *)v12;
          v12 = __errno_location();
          error(0,*v12,v7,v3);
          v10 = 0;
          return v10;
        }
        v20 = 1;
        v23 = v14;
        if ((char)v31[0xf]) {
          v12 = (unsigned long)sub_18a0f(4,v37);
          v3 = v12;
          v12 = (char *)gettext("removed %s\n");
          printf((char *)v12,v3);
          v23 = v14;
        }
      }
    }
  }
label_bbf1:
  v25 = v23;
  if ((((v30) && (*(long *)&v31[0x12])) && ((char)v31[6] != '\x01')) && (!*v31)) {
    v44 = '\x01';
    if (v43) // branch-flip
      v67 = &v16;
    else {
      *(unsigned long *)&v25[-8] = 0xbc79;
      v11 = fstatat(v36,v34,&v18,0x100,v25[-8]);
      if (v11) // branch-flip
        v44 = '\0';
      else {
        v67 = &v18;
      }
    }
    if ((v44) && (v11 = *(unsigned int *)&v67[3] & 0xf000, v11 == 0xa000)) {
      v3 = *(unsigned long *)&v31[0x12];
      *(unsigned long *)&v25[-8] = 0xbcdf;
      v10 = sub_11a7b(v3,v34,v67);
      if (v10) {
        *(unsigned long *)&v25[-8] = 0xbcfc;
        v12 = (unsigned long)sub_18941(1,4,v37);
        v3 = v12;
        *(unsigned long *)&v25[-8] = 0xbd18;
        v12 = (unsigned long)sub_18941(0,4,v38);
        v8 = v12;
        *(unsigned long *)&v25[-8] = 0xbd2a;
        v12 = (char *)gettext("will not copy %s through just-created symlink %s",v25[-8]);
        *(unsigned long *)&v25[-8] = 0xbd47;
        error(0,0,(char *)v12,v8,v3);
        v10 = 0;
        return v10;
      }
    }
  }
  if ((((char)v31[0xf]) && ((char)v31[6] != '\x01')) && (v11 = v49 & 0xf000, v11 != 0x4000)) {
    *(unsigned long *)&v25[-8] = 0xbda4;
    sub_a6db(v38,v37,v66);
  }
  if (v52) { // branch-flip
    if (((char)v31[0xe]) && (v11 = v49 & 0xf000, v11 == 0x4000)) {
      if (v30) { // branch-flip
        *(unsigned long *)&v25[-8] = 0xbe07;
        v12 = (long)sub_de9c(v34,v81,v15);
        v65 = (long)v12;
      }
      else {
        *(unsigned long *)&v25[-8] = 0xbe2c;
        v12 = (long)sub_de23(v81,v15);
        v65 = (long)v12;
      }
    }
    else if (((char)v31[6]) && (v82 == 1)) {
      *(unsigned long *)&v25[-8] = 0xbe6d;
      v12 = (long)sub_de23(v81,v15);
      v65 = (long)v12;
    }
    else if ((((char)v31[0xc]) && (*(char *)((long)v31 + 0x17) != '\x01')) && ((2 <= v82 || (((v30 && (v31[1] == 3)) || (v31[1] == 4)))))) {
      *(unsigned long *)&v25[-8] = 0xbeeb;
      v12 = (long)sub_de9c(v34,v81,v15);
      v65 = (long)v12;
    }
  }
  else {
    v65 = 0;
  }
  if (v65) {
    v11 = v49 & 0xf000;
    if (v11 != 0x4000) {
      v1 = *(unsigned char *)&v31[0xf];
      *(unsigned long *)&v25[-0x10] = (unsigned long)v45;
      *(unsigned long *)&v25[-0x18] = (unsigned long)v1;
      *(unsigned long *)&v25[-0x20] = 1;
      *(unsigned long *)&v25[-0x28] = 0xc159;
      v10 = sub_a85d(0,v36,v65,v37,v36,v34);
      if (v10 == '\x01') {
        v10 = 1;
        return v10;
      }
      goto label_d821;
    }
    *(unsigned long *)&v25[-8] = 0xbf37;
    v10 = sub_19370(0xffffff9c,v38,v36,v65);
    if (v10) {
      *(unsigned long *)&v25[-8] = 0xbf54;
      v12 = (unsigned long)sub_18941(1,4,dat_271a8);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xbf70;
      v12 = (unsigned long)sub_18941(0,4,dat_271a0);
      v8 = v12;
      *(unsigned long *)&v25[-8] = 0xbf82;
      v12 = (char *)gettext("cannot copy a directory, %s, into itself, %s",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xbf9f;
      error(0,0,(char *)v12,v8,v3);
      *v28 = 1;
      goto label_d821;
    }
    *(unsigned long *)&v25[-8] = 0xbfcf;
    v10 = sub_19370(v36,v34,v36,v65);
    if (v10) {
      *(unsigned long *)&v25[-8] = 0xbfe7;
      v12 = (unsigned long)sub_18a0f(4,dat_271a0);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xbff9;
      v12 = (char *)gettext("warning: source directory %s specified more than once",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xc013;
      error(0,0,(char *)v12,v3);
      if (((char)v31[6]) && (v27))
        *v27 = 1;
      v10 = 1;
      return v10;
    }
    if ((v31[1] != 4) && ((!v30 || (v31[1] != 3)))) {
      *(unsigned long *)&v25[-8] = 0xc08f;
      v12 = (void *)sub_a7d7(v37,v34,v65);
      v75 = v12;
      *(unsigned long *)&v25[-8] = 0xc0af;
      v12 = (unsigned long)sub_18941(1,4,v12);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xc0cb;
      v12 = (unsigned long)sub_18941(0,4,v37);
      v8 = v12;
      *(unsigned long *)&v25[-8] = 0xc0dd;
      v12 = (char *)gettext("will not create hard link %s to directory %s",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xc0fa;
      error(0,0,(char *)v12,v8,v3);
      *(unsigned long *)&v25[-8] = 0xc109;
      free(v75,v25[-8]);
      goto label_d821;
    }
  }
  if ((char)v31[6]) {
    if (v52 == 0x11) {
      *(unsigned long *)&v25[-8] = 0xc1af;
      v11 = renameat(0xffffff9c,v38,v36,v34);
      if (v11) { // branch-flip
        *(unsigned long *)&v25[-8] = 0xc1b8;
        v12 = __errno_location(v25[-8]);
        v11 = *v12;
      }
      else {
        v11 = 0;
      }
      v52 = v11;
    }
    if (!v52) {
      if ((char)v31[0xf]) {
        *(unsigned long *)&v25[-8] = 0xc1f2;
        v12 = (char *)gettext("renamed ",v25[-8]);
        *(unsigned long *)&v25[-8] = 0xc1ff;
        printf((char *)v12);
        *(unsigned long *)&v25[-8] = 0xc21f;
        sub_a6db(v38,v37,v66);
      }
      if (*(long *)&v31[10]) {
        *(unsigned long *)&v25[-8] = 0xc24a;
        sub_8750(v37,1,v31);
      }
      if (v27)
        *v27 = 1;
      if ((v30) && (*(char *)((long)v31 + 0x3f) != '\x01')) {
        v3 = *(unsigned long *)&v31[0x12];
        *(unsigned long *)&v25[-8] = 0xc29d;
        sub_119db(v3,v34,&v15);
      }
      v10 = 1;
      return v10;
    }
    if (v52 == 0x16) {
      *(unsigned long *)&v25[-8] = 0xc2c9;
      v12 = (unsigned long)sub_18941(1,4,dat_271a8);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xc2e5;
      v12 = (unsigned long)sub_18941(0,4,dat_271a0);
      v8 = v12;
      *(unsigned long *)&v25[-8] = 0xc2f7;
      v12 = (char *)gettext("cannot move %s to a subdirectory of itself, %s",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xc314;
      error(0,0,(char *)v12,v8,v3);
      *v28 = 1;
      v10 = 1;
      return v10;
    }
    if (v52 != 0x12) {
      *(unsigned long *)&v25[-8] = 0xc34e;
      v12 = (unsigned long)sub_18941(1,4,v37);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xc36a;
      v12 = (unsigned long)sub_18941(0,4,v38);
      v8 = v12;
      *(unsigned long *)&v25[-8] = 0xc37c;
      v12 = (char *)gettext("cannot move %s to %s",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xc39c;
      error(0,v52,(char *)v12,v8,v3);
      *(unsigned long *)&v25[-8] = 0xc3b5;
      sub_dda4(v81,v15);
      v10 = 0;
      return v10;
    }
    v11 = v49 & 0xf000;
    v13 = (v11 != 0x4000) ? 0 : 0x200; // branch-flip
    *(unsigned long *)&v25[-8] = 0xc3f4;
    v11 = unlinkat(v36,v34,v13,v25[-8]);
    if (v11) {
      *(unsigned long *)&v25[-8] = 0xc401;
      v12 = __errno_location(v25[-8]);
      if (*v12 != 2) {
        *(unsigned long *)&v25[-8] = 0xc425;
        v12 = (unsigned long)sub_18941(1,4,v37);
        v3 = v12;
        *(unsigned long *)&v25[-8] = 0xc441;
        v12 = (unsigned long)sub_18941(0,4,v38);
        v8 = v12;
        *(unsigned long *)&v25[-8] = 0xc453;
        v12 = (char *)gettext("inter-device move failed: %s to %s; unable to remove target",v25[-8]);
        v7 = (char *)v12;
        *(unsigned long *)&v25[-8] = 0xc45b;
        v12 = __errno_location(v25[-8]);
        v13 = *v12;
        *(unsigned long *)&v25[-8] = 0xc477;
        error(0,v13,v7,v8,v3);
        *(unsigned long *)&v25[-8] = 0xc490;
        sub_dda4(v81,v15);
        v10 = 0;
        return v10;
      }
    }
    if (((char)v31[0xf]) && (v11 = v49 & 0xf000, v11 != 0x4000)) {
      *(unsigned long *)&v25[-8] = 0xc4ca;
      v12 = (char *)gettext("copied ",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xc4d7;
      printf((char *)v12);
      *(unsigned long *)&v25[-8] = 0xc4f7;
      sub_a6db(v38,v37,v66);
    }
    v20 = 1;
  }
  if (*(char *)((long)v31 + 0x39)) // branch-flip
    v11 = v31[4];
  else {
    v11 = v49;
  }
  v11 &= 0xfff;
  v6 = v11;
  v58 = v11;
  if (*(char *)((long)v31 + 0x1d) != '\x01') { // branch-flip
    v11 = v49 & 0xf000;
    v11 = (v11 != 0x4000) ? 0 : 0x12; // branch-flip
  }
  else {
    v11 = 0x3f;
  }
  v11 &= v6;
  v51 = v11;
  v40 = 1;
  *(unsigned long *)&v25[-8] = 0xc5a9;
  v10 = sub_84c6(v38,v37,v49,v20,v31);
  if (v10 != '\x01') {
    v10 = 0;
    return v10;
  }
  v11 = v49 & 0xf000;
  if (v11 != 0x4000) { // branch-flip
    if (*(char *)((long)v31 + 0x3a)) { // branch-flip
      v42 = '\x01';
      if (*v38 != '/') {
        *(unsigned long *)&v25[-8] = 0xca83;
        v12 = (char *)sub_11559(v34);
        v78 = (char *)v12;
        if (v36 != -100) { // branch-flip
label_cab0:
          *(unsigned long *)&v25[-8] = 0xcac9;
          v11 = stat(".",&v19,v25[-8]);
          if (v11) goto label_cb17;
          *(unsigned long *)&v25[-8] = 0xcaed;
          v11 = fstatat(v36,v78,&v18,0,v25[-8]);
          if ((v11) || ((v94 == v95 && (v19 == v18)))) goto label_cb17;
          v10 = '\0';
        }
        else {
          *(unsigned long *)&v25[-8] = 0xcaac;
          v11 = strcmp(".",(char *)v12,v25[-8]);
          if (v11) goto label_cab0;
label_cb17:
          v10 = '\x01';
        }
        v48 = v10;
        *(unsigned long *)&v25[-8] = 0xcb3f;
        free(v78,v25[-8]);
        if (v48 != '\x01') {
          *(unsigned long *)&v25[-8] = 0xcb66;
          v12 = (unsigned long)sub_18ba1(0,3,v37);
          v3 = v12;
          *(unsigned long *)&v25[-8] = 0xcb78;
          v12 = (char *)gettext("%s: can make relative symbolic links only in current directory",v25[-8]);
          *(unsigned long *)&v25[-8] = 0xcb92;
          error(0,0,(char *)v12,v3);
          goto label_d821;
        }
      }
      v2 = *(char *)((long)v31 + 0x16);
      *(unsigned long *)&v25[-8] = 0xcbc7;
      v11 = sub_e27f(v38,v36,v34,v2,0xffffffff);
      v61 = v11;
      if (1 <= (int)v11) {
        *(unsigned long *)&v25[-8] = 0xcbf3;
        v12 = (unsigned long)sub_18941(1,4,v38);
        v3 = v12;
        *(unsigned long *)&v25[-8] = 0xcc0f;
        v12 = (unsigned long)sub_18941(0,4,v37);
        v8 = v12;
        *(unsigned long *)&v25[-8] = 0xcc21;
        v12 = (char *)gettext("cannot create symbolic link %s to %s",v25[-8]);
        *(unsigned long *)&v25[-8] = 0xcc41;
        error(0,v61,(char *)v12,v8,v3);
        goto label_d821;
      }
    }
    else {
      if (*(char *)((long)v31 + 0x17)) { // branch-flip
        v10 = ((*(char *)((long)v31 + 0x16)) || (v31[2] == 3));
        v47 = v10;
        *(unsigned long *)&v25[-0x10] = (unsigned long)v45;
        *(unsigned long *)&v25[-0x18] = 0;
        *(unsigned long *)&v25[-0x20] = (unsigned long)v10;
        *(unsigned long *)&v25[-0x28] = 0xccd6;
        v10 = sub_a85d(v38,0xffffff9c,v38,v37,v36,v34);
      }
      else {
        v11 = v49 & 0xf000;
        if ((v11 != 0x8000) && ((!(char)v31[5] || (v11 = v49 & 0xf000, v11 == 0xa000)))) {
          v11 = v49 & 0xf000;
          if (v11 != 0x1000) { // branch-flip
            v11 = v49 & 0xf000;
            if (((v11 != 0x6000) && (v11 = v49 & 0xf000, v11 != 0x2000)) && (v11 = v49 & 0xf000, v11 != 0xc000)) {
              v11 = v49 & 0xf000;
              if (v11 != 0xa000) {
                *(unsigned long *)&v25[-8] = 0xd1d8;
                v12 = (unsigned long)sub_18a0f(4,v38);
                v3 = v12;
                *(unsigned long *)&v25[-8] = 0xd1ea;
                v12 = (char *)gettext("%s has unknown file type",v25[-8]);
                *(unsigned long *)&v25[-8] = 0xd204;
                error(0,0,(char *)v12,v3);
                goto label_d821;
              }
              *(unsigned long *)&v25[-8] = 0xcf4b;
              v12 = (char *)sub_eded(v38,v87);
              v76 = (char *)v12;
              v42 = '\x01';
              if (!v12) {
                *(unsigned long *)&v25[-8] = 0xcf77;
                v12 = (unsigned long)sub_18a0f(4,v38);
                v3 = v12;
                *(unsigned long *)&v25[-8] = 0xcf89;
                v12 = (char *)gettext("cannot read symbolic link %s",v25[-8]);
                v7 = (char *)v12;
                *(unsigned long *)&v25[-8] = 0xcf91;
                v12 = __errno_location(v25[-8]);
                v13 = *v12;
                *(unsigned long *)&v25[-8] = 0xcfaa;
                error(0,v13,v7,v3);
                goto label_d821;
              }
              v2 = *(char *)((long)v31 + 0x16);
              *(unsigned long *)&v25[-8] = 0xcfdf;
              v11 = sub_e27f(v12,v36,v34,v2,0xffffffff);
              v9 = v93;
              v53 = v11;
              if ((((1 <= (int)v11) && (*(char *)((long)v31 + 0x3b))) && ((bool)v20 != 1)) && (v11 = v90 & 0xf000, v11 == 0xa000)) {
                *(unsigned long *)&v25[-8] = 0xd043;
                v12 = (unsigned long)strlen(v76,v25[-8]);
                if ((int *)v9 == v12) {
                  *(unsigned long *)&v25[-8] = 0xd069;
                  v12 = (char *)sub_f0b6(v36,v34,v93);
                  v77 = (char *)v12;
                  if (v12) {
                    *(unsigned long *)&v25[-8] = 0xd093;
                    v11 = strcmp((char *)v12,v76,v25[-8]);
                    if (!v11)
                      v53 = 0;
                    *(unsigned long *)&v25[-8] = 0xd0b0;
                    free(v77,v25[-8]);
                  }
                }
              }
              *(unsigned long *)&v25[-8] = 0xd0bf;
              free(v76,v25[-8]);
              if (1 <= v53) {
                *(unsigned long *)&v25[-8] = 0xd0dc;
                v12 = (unsigned long)sub_18a0f(4,v37);
                v3 = v12;
                *(unsigned long *)&v25[-8] = 0xd0ee;
                v12 = (char *)gettext("cannot create symbolic link %s",v25[-8]);
                *(unsigned long *)&v25[-8] = 0xd10b;
                error(0,v53,(char *)v12,v3);
                goto label_d821;
              }
              if (*(char *)((long)v31 + 0x33)) {
                *(unsigned long *)&v25[-8] = 0xd124;
                sub_a788();
              }
              if (*(char *)((long)v31 + 0x1d)) {
                *(unsigned long *)&v25[-8] = 0xd157;
                v11 = sub_112c3(v36,v34,v84,v85);
                if (v11) {
                  *(unsigned long *)&v25[-8] = 0xd16e;
                  v10 = sub_dc1f(v31);
                  if (v10 != '\x01') {
                    *(unsigned long *)&v25[-8] = 0xd188;
                    v12 = (char *)gettext("failed to preserve ownership for %s",v25[-8]);
                    v7 = (char *)v12;
                    *(unsigned long *)&v25[-8] = 0xd190;
                    v12 = __errno_location(v25[-8]);
                    v13 = *v12;
                    *(unsigned long *)&v25[-8] = 0xd1b0;
                    error(0,v13,v7,v37);
                    if (*(char *)((long)v31 + 0x32)) goto label_d821;
                  }
                }
              }
            }
            else {
              v11 = ~v51;
              v11 &= v49;
              v59 = v11;
              *(unsigned long *)&v25[-8] = 0xcec5;
              v11 = mknodat(v36,v34,v11,v86);
              if (v11) {
                *(unsigned long *)&v25[-8] = 0xcee1;
                v12 = (unsigned long)sub_18a0f(4,v37);
                v3 = v12;
                *(unsigned long *)&v25[-8] = 0xcef3;
                v12 = (char *)gettext("cannot create special file %s",v25[-8]);
                v7 = (char *)v12;
                *(unsigned long *)&v25[-8] = 0xcefb;
                v12 = __errno_location(v25[-8]);
                v13 = *v12;
                *(unsigned long *)&v25[-8] = 0xcf14;
                error(0,v13,v7,v3);
                goto label_d821;
              }
            }
          }
          else {
            v11 = ~v51;
            v11 &= v49;
            v60 = v11;
            *(unsigned long *)&v25[-8] = 0xcdd8;
            v11 = mknodat(v36,v34,v11,0);
            if (v11) {
              v11 = v60 & 0xffffefff;
              *(unsigned long *)&v25[-8] = 0xce02;
              v11 = mkfifoat(v36,v34,v11);
              if (v11) {
                *(unsigned long *)&v25[-8] = 0xce1e;
                v12 = (unsigned long)sub_18a0f(4,v37);
                v3 = v12;
                *(unsigned long *)&v25[-8] = 0xce30;
                v12 = (char *)gettext("cannot create fifo %s",v25[-8]);
                v7 = (char *)v12;
                *(unsigned long *)&v25[-8] = 0xce38;
                v12 = __errno_location(v25[-8]);
                v13 = *v12;
                *(unsigned long *)&v25[-8] = 0xce51;
                error(0,v13,v7,v3);
                goto label_d821;
              }
            }
          }
          goto label_d20a;
        }
        v41 = '\x01';
        v11 = v58 & 0x1ff;
        *(long **)&v25[-0x10] = &v15;
        *(bool **)&v25[-0x18] = &v20;
        *(unsigned long *)&v25[-0x20] = (unsigned long)v51;
        *(unsigned long *)&v25[-0x28] = 0xcd7b;
        v10 = sub_896f(v38,v37,v36,v34,v31,v11);
      }
      if (v10 != '\x01') goto label_d821;
    }
  }
  else {
    *(unsigned long *)&v25[-8] = 0xc5e9;
    v10 = sub_7a8e(&v15,v32);
    v23 = v25;
    if (v10) {
      *(unsigned long *)&v25[-8] = 0xc601;
      v12 = (unsigned long)sub_18a0f(4,v38);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xc613;
      v12 = (char *)gettext("cannot copy cyclic symbolic link %s",v25[-8]);
      *(unsigned long *)&v25[-8] = 0xc62d;
      error(0,0,(char *)v12,v3);
      goto label_d821;
    }
    while (v23 != v25) {
      *(unsigned long *)&v23[-8] = *(unsigned long *)&v23[-8];
      v24 = &v23[-0x1000];
      v23 = v24;
    }
    v25 = &v23[-0x20];
    v12 = (unsigned long *)&v23[-8];
    *(unsigned long *)v12 = *(unsigned long *)v12;
    v12 = (char *)&v23[-0x11];
    v12 = (unsigned long *)((unsigned long)v12 & 0xfffffffffffffff0);
    v79 = (unsigned long *)v12;
    *(unsigned long *)v12 = v32;
    *(unsigned long *)((long)v12 + 8) = v81;
    *(long *)((long)v12 + 0x10) = v15;
    if (((bool)v20) || (v11 = v90 & 0xf000, v11 != 0x4000)) {
      v11 = ~v51;
      v11 &= v58;
      v62 = v11;
      *(unsigned long *)&v23[-0x28] = 0xc73e;
      v11 = mkdirat(v36,v34,v11);
      if (v11) {
        *(unsigned long *)&v23[-0x28] = 0xc756;
        v12 = (unsigned long)sub_18a0f(4,v37);
        v3 = v12;
        *(unsigned long *)&v23[-0x28] = 0xc768;
        v12 = (char *)gettext("cannot create directory %s",v23[-0x28]);
        v7 = (char *)v12;
        *(unsigned long *)&v23[-0x28] = 0xc770;
        v12 = __errno_location(v23[-0x28]);
        v13 = *v12;
        *(unsigned long *)&v23[-0x28] = 0xc789;
        error(0,v13,v7,v3);
        v25 = &v23[-0x20];
        goto label_d821;
      }
      *(unsigned long *)&v23[-0x28] = 0xc7ae;
      v11 = fstatat(v36,v34,&v16,0x100,v23[-0x28]);
      if (v11) {
        *(unsigned long *)&v23[-0x28] = 0xc7c6;
        v12 = (unsigned long)sub_18a0f(4,v37);
        v3 = v12;
        *(unsigned long *)&v23[-0x28] = 0xc7d8;
        v12 = (char *)gettext("cannot stat %s",v23[-0x28]);
        v7 = (char *)v12;
        *(unsigned long *)&v23[-0x28] = 0xc7e0;
        v12 = __errno_location(v23[-0x28]);
        v13 = *v12;
        *(unsigned long *)&v23[-0x28] = 0xc7f9;
        error(0,v13,v7,v3);
        v25 = &v23[-0x20];
        goto label_d821;
      }
      v11 = v90 & 0x1c0;
      if (v11 != 0x1c0) {
        v50 = v90;
        v39 = '\x01';
        v11 = v90 | 0x1c0;
        *(unsigned long *)&v23[-0x28] = 0xc84b;
        v11 = sub_11260(v36,v34,v11);
        if (v11) {
          *(unsigned long *)&v23[-0x28] = 0xc863;
          v12 = (unsigned long)sub_18a0f(4,v37);
          v3 = v12;
          *(unsigned long *)&v23[-0x28] = 0xc875;
          v12 = (char *)gettext("setting permissions for %s",v23[-0x28]);
          v7 = (char *)v12;
          *(unsigned long *)&v23[-0x28] = 0xc87d;
          v12 = __errno_location(v23[-0x28]);
          v13 = *v12;
          *(unsigned long *)&v23[-0x28] = 0xc896;
          error(0,v13,v7,v3);
          v25 = &v23[-0x20];
          goto label_d821;
        }
      }
      if (*v29 != '\x01') {
        *(unsigned long *)&v23[-0x28] = 0xc8cc;
        sub_de9c(v34,v88,v16);
        *v29 = '\x01';
      }
      if ((char)v31[0xf]) {
        if ((char)v31[6]) { // branch-flip
          *(unsigned long *)&v23[-0x28] = 0xc90c;
          v12 = (unsigned long)sub_18a0f(4,v37);
          v3 = v12;
          *(unsigned long *)&v23[-0x28] = 0xc91e;
          v12 = (char *)gettext("created directory %s\n",v23[-0x28]);
          *(unsigned long *)&v23[-0x28] = 0xc92e;
          printf((char *)v12,v3);
        }
        else {
          *(unsigned long *)&v23[-0x28] = 0xc951;
          sub_a6db(v38,v37,0);
        }
      }
    }
    else {
      v51 = 0;
      if ((*(long *)&v31[10]) || (*(char *)((long)v31 + 0x33))) {
        *(unsigned long *)&v23[-0x28] = 0xc997;
        v10 = sub_8750(v37,0,v31);
        if ((v10 != '\x01') && ((char)v31[0xd])) goto label_d821;
      }
    }
    if (((!(char)v31[7]) || (!v33)) || (v25 = &v23[-0x20], *v33 == v15)) {
      *(char **)&v23[-0x28] = v28;
      *(char **)&v23[-0x30] = v29;
      *(int **)&v23[-0x38] = v31;
      *(unsigned long **)&v23[-0x40] = v79;
      *(unsigned long *)&v23[-0x48] = 0xca39;
      v10 = sub_7ee6(v38,v37,v36,v34,v20,&v15);
      v40 = v10;
      v25 = &v23[-0x20];
    }
  }
label_d20a:
  if (((((bool)v20 != 1) && ((char)v31[5] != '\x01')) && (v11 = v49 & 0xf000, v11 != 0x4000)) && ((*(long *)&v31[10] || (*(char *)((long)v31 + 0x33))))) {
    *(unsigned long *)&v25[-8] = 0xd276;
    v10 = sub_8750(v37,0,v31);
    if ((v10 != '\x01') && ((char)v31[0xd])) {
label_d821:
      if (*(char *)((long)v31 + 0x33)) {
        *(unsigned long *)&v25[-8] = 0xd835;
        sub_a788();
      }
      if (!v65) {
        *(unsigned long *)&v25[-8] = 0xd858;
        sub_dda4(v81,v15);
      }
      if (v66) {
        v12 = (long)((long)v34 - (long)v37);
        v12 = (long)((long)v66 + (long)v12);
        v80 = (long)v12;
        *(unsigned long *)&v25[-8] = 0xd8a9;
        v11 = renameat(v36,v12,v36,v34);
        if (v11) { // branch-flip
          *(unsigned long *)&v25[-8] = 0xd8c1;
          v12 = (unsigned long)sub_18a0f(4,v37);
          v3 = v12;
          *(unsigned long *)&v25[-8] = 0xd8d3;
          v12 = (char *)gettext("cannot un-backup %s",v25[-8]);
          v7 = (char *)v12;
          *(unsigned long *)&v25[-8] = 0xd8db;
          v12 = __errno_location(v25[-8]);
          v13 = *v12;
          *(unsigned long *)&v25[-8] = 0xd8f4;
          error(0,v13,v7,v3);
        }
        else if ((char)v31[0xf]) {
          *(unsigned long *)&v25[-8] = 0xd91e;
          v12 = (unsigned long)sub_18941(1,4,v37);
          v3 = v12;
          *(unsigned long *)&v25[-8] = 0xd93a;
          v12 = (unsigned long)sub_18941(0,4,v66);
          v8 = v12;
          *(unsigned long *)&v25[-8] = 0xd94c;
          v12 = (char *)gettext("%s -> %s (unbackup)\n",v25[-8]);
          *(unsigned long *)&v25[-8] = 0xd95f;
          printf((char *)v12,v8,v3);
        }
      }
      v10 = 0;
      return v10;
    }
  }
  if ((v30) && (*(long *)&v31[0x12])) {
    *(unsigned long *)&v25[-8] = 0xd2c9;
    v11 = fstatat(v36,v34,&v18,0x100,v25[-8]);
    if (!v11) {
      v3 = *(unsigned long *)&v31[0x12];
      *(unsigned long *)&v25[-8] = 0xd2f1;
      sub_119db(v3,v34,&v18);
    }
  }
  if ((*(char *)((long)v31 + 0x17)) && (v11 = v49 & 0xf000, v11 != 0x4000)) {
    v10 = v40;
    return v10;
  }
  if (v41) {
    v10 = v40;
    return v10;
  }
  if (*(char *)((long)v31 + 0x1f)) {
    *(unsigned long *)&v25[-8] = 0xd355;
    v26 = sub_19e57(&v15);
    v12 = SUB168(v26,0);
    v18 = (long)v12;
    v95 = SUB168(v26,8);
    *(unsigned long *)&v25[-8] = 0xd372;
    v26 = sub_19e8b(&v15);
    v12 = SUB168(v26,0);
    v96 = v12;
    v97 = SUB168(v26,8);
    v11 = (v42) ? 0x100 : 0; // branch-flip
    v63 = v11;
    *(unsigned long *)&v25[-8] = 0xd3bc;
    v11 = utimensat(v36,v34,&v18,v11);
    if (v11) {
      *(unsigned long *)&v25[-8] = 0xd3d4;
      v12 = (unsigned long)sub_18a0f(4,v37);
      v3 = v12;
      *(unsigned long *)&v25[-8] = 0xd3e6;
      v12 = (char *)gettext("preserving times for %s",v25[-8]);
      v7 = (char *)v12;
      *(unsigned long *)&v25[-8] = 0xd3ee;
      v12 = __errno_location(v25[-8]);
      v13 = *v12;
      *(unsigned long *)&v25[-8] = 0xd407;
      error(0,v13,v7,v3);
      if (*(char *)((long)v31 + 0x32)) {
        v10 = 0;
        return v10;
      }
    }
  }
  if (((v42 != '\x01') && (*(char *)((long)v31 + 0x1d))) && (((bool)v20 || ((v84 != v91 || (v85 != v92)))))) {
    *(long **)&v25[-8] = &v16;
    *(unsigned long *)&v25[-0x10] = (unsigned long)v20;
    *(unsigned long *)&v25[-0x18] = 0xd4b4;
    v11 = sub_820f(v31,v37,v36,v34,0xffffffff,&v15);
    if (v11 == -1) {
      v10 = 0;
      return v10;
    }
    if (!v11)
      v49 &= 0xfffff1ff;
  }
  if (*(char *)((long)v31 + 0x35)) {
    *(unsigned long *)&v25[-8] = 0xd511;
    v10 = sub_7d7e(v38,0xffffffff,v37,0xffffffff,v31);
    if ((v10 != '\x01') && (*(char *)((long)v31 + 0x36))) {
      v10 = 0;
      return v10;
    }
  }
  if (v42) {
    v10 = v40;
    return v10;
  }
  *(unsigned long *)&v25[-8] = 0xd561;
  sub_84b0(v37,0xffffffff,&v15);
  if ((*(char *)((long)v31 + 0x1e)) || ((char)v31[6])) {
    *(unsigned long *)&v25[-8] = 0xd5a8;
    v11 = sub_ec57(v38,0xffffffff,v37,0xffffffff,v49);
    if ((v11) && (*(char *)((long)v31 + 0x32))) {
      v10 = 0;
      return v10;
    }
  }
  else if (*(char *)((long)v31 + 0x39)) { // branch-flip
    v13 = v31[4];
    *(unsigned long *)&v25[-8] = 0xd5fa;
    v11 = sub_ed24(v37,0xffffffff,v13);
    if (v11) {
      v10 = 0;
      return v10;
    }
  }
  else if (((char)v31[8]) && ((bool)v20)) {
    v11 = v49 & 0xf000;
    if ((v11 != 0x4000) && (v11 = v49 & 0xf000, v11 != 0xc000))
      v11 = 0x1b6;
    else {
      v11 = 0x1ff;
    }
    v64 = v11;
    *(unsigned long *)&v25[-8] = 0xd661;
    v11 = sub_dcaf();
    v11 = ~v11;
    *(unsigned long *)&v25[-8] = 0xd681;
    v11 = sub_ed24(v37,0xffffffff,v11 & v64);
    if (v11) {
      v10 = 0;
      return v10;
    }
  }
  else {
    if (v51) {
      *(unsigned long *)&v25[-8] = 0xd6a5;
      v11 = sub_dcaf();
      v11 = ~v11;
      v51 &= v11;
      if ((v51) && (v39 != '\x01')) {
        if ((bool)v20) {
          *(unsigned long *)&v25[-8] = 0xd6f7;
          v11 = fstatat(v36,v34,&v16,0x100,v25[-8]);
          if (v11) {
            *(unsigned long *)&v25[-8] = 0xd70f;
            v12 = (unsigned long)sub_18a0f(4,v37);
            v3 = v12;
            *(unsigned long *)&v25[-8] = 0xd721;
            v12 = (char *)gettext("cannot stat %s",v25[-8]);
            v7 = (char *)v12;
            *(unsigned long *)&v25[-8] = 0xd729;
            v12 = __errno_location(v25[-8]);
            v13 = *v12;
            *(unsigned long *)&v25[-8] = 0xd742;
            error(0,v13,v7,v3);
            v10 = 0;
            return v10;
          }
        }
        v50 = v90;
        v11 = ~v90;
        v11 &= v51;
        if (v11)
          v39 = '\x01';
      }
    }
    if (v39) {
      v11 = v50 | v51;
      *(unsigned long *)&v25[-8] = 0xd7a3;
      v11 = sub_11260(v36,v34,v11);
      if (v11) {
        *(unsigned long *)&v25[-8] = 0xd7bb;
        v12 = (unsigned long)sub_18a0f(4,v37);
        v3 = v12;
        *(unsigned long *)&v25[-8] = 0xd7cd;
        v12 = (char *)gettext("preserving permissions for %s",v25[-8]);
        v7 = (char *)v12;
        *(unsigned long *)&v25[-8] = 0xd7d5;
        v12 = __errno_location(v25[-8]);
        v13 = *v12;
        *(unsigned long *)&v25[-8] = 0xd7ee;
        error(0,v13,v7,v3);
        if (*(char *)((long)v31 + 0x32)) {
          v10 = 0;
          return v10;
        }
      }
    }
  }
  v10 = v40;
  return v10;
}


// Function: sub_d983 @ 0xd983
unsigned long sub_d983(unsigned int *a0)
{
  if (4 <= *a0)
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options"); // no-return
  if (((a0[3] != 1) && (a0[3] != 2)) && (a0[3] != 3))
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options"); // no-return
  if (((a0[0x11]) && (a0[0x11] != 1)) && (a0[0x11] != 2))
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options"); // no-return
  if ((*(char *)((long)a0 + 0x17) == '\x01') && (*(char *)((long)a0 + 0x3a) == '\x01'))
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options"); // no-return
  if ((a0[0x11] == 2) && (a0[3] != 2))
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)","src/copy.c",0xc13,"valid_options"); // no-return
  return 1;
}


// Function: sub_daea @ 0xdaea
void sub_daea(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long a3,unsigned int a4,unsigned long a5,unsigned long a6,unsigned long a7)
{
  unsigned long v1;
  unsigned long v2;
  char v3; // stack - 0x11
  
  v2 = a7;
  v1 = a6;
  if (sub_d983(a5)) {
    v3 = 0;
    dat_271a0 = a0;
    dat_271a8 = a1;
    sub_abda(a0,a1,a2,a3,a4,0,0,a5,1,&v3,v1,v2);
    return;
  }
  __assert_fail("valid_options (options)","src/copy.c",0xc2a,"copy"); // no-return
}


// Function: sub_dbcb @ 0xdbcb
void sub_dbcb(void *a0)
{
  memset(a0,0,0x58);
  *(bool *)((long)a0 + 0x1b) = geteuid() == 0;
  *(char *)((long)a0 + 0x1a) = *(char *)((long)a0 + 0x1b);
  *(unsigned int *)((long)a0 + 0x40) = 0xffffffff;
}


// Function: sub_dc1f @ 0xdc1f
unsigned int sub_dc1f(long a0)
{
  if (((*__errno_location() == 1) || (*__errno_location() == 0x16)) && (*(char *)(a0 + 0x1a) != '\x01'))
    return 1;
  return 0;
}


// Function: sub_dc67 @ 0xdc67
unsigned int sub_dc67(long a0)
{
  if (((*__errno_location() == 1) || (*__errno_location() == 0x16)) && (*(char *)(a0 + 0x1b) != '\x01'))
    return 1;
  return 0;
}


// Function: sub_dcaf @ 0xdcaf
unsigned int sub_dcaf(void)
{
  if (dat_27018 == 0xffffffff) {
    dat_27018 = umask(0);
    umask(dat_27018);
  }
  return dat_27018;
}


// Function: sub_dce7 @ 0xdce7
unsigned long sub_dce7(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_dd14 @ 0xdd14
unsigned int sub_dd14(long *a0,long *a1) // return-dupe
{
  if ((*a0 == *a1) && (a0[1] == a1[1]))
    return 1;
  return 0;
}


// Function: sub_dd6d @ 0xdd6d
void sub_dd6d(void *a0)
{
  free(*(void **)((long)a0 + 0x10));
  free(a0);
}


// Function: sub_dda4 @ 0xdda4
void sub_dda4(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v4 = 0;
  v2 = a0;
  v3 = a1;
  v1 = sub_163ee(dat_275e0,&v2);
  if (!v1)
    return;
  sub_dd6d(v1);
}


// Function: sub_de23 @ 0xde23
unsigned long sub_de23(unsigned long a0,unsigned long a1) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  
  v2 = a0;
  v3 = a1;
  v1 = sub_14f88(dat_275e0,&v2);
  if (v1)
    return *(unsigned long *)(v1 + 0x10);
  return 0;
}


// Function: sub_de9c @ 0xde9c
unsigned long sub_de9c(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  v1 = (unsigned long *)sub_1b941(0x18);
  v1[2] = sub_1bf48(a0);
  *v1 = a1;
  v1[1] = a2;
  v2 = (unsigned long *)sub_1637e(dat_275e0,v1);
  if (!v2)
    sub_1bf79(); // no-return
  if (v2 != v1) {
    sub_dd6d(v1);
    return v2[2];
  }
  return 0;
}


// Function: sub_df3d @ 0xdf3d
void sub_df3d(void)
{
  dat_275e0 = sub_1563d(0x67,0,sub_dce7,sub_dd14,sub_dd6d);
  if (!dat_275e0)
    sub_1bf79(); // no-return
}


// Function: sub_df8a @ 0xdf8a
void * sub_df8a(void *a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  void *v3; // stack - 0x20
  
  v1 = sub_10369(a0) - (long)a0;
  v2 = v1 + 9;
  if (0x101 <= v2) { // branch-flip
    v3 = malloc(v2);
    if (!v3) {
      return v3;
    }
  }
  else {
    v3 = a1;
  }
  strcpy(mempcpy(v3,a0,v1),"CuXXXXXX");
  return v3;
}


// Function: sub_e021 @ 0xe021
void sub_e021(unsigned long a0,unsigned int *a1)
{
  linkat(*a1,*(unsigned long *)&a1[2],a1[4],a0,a1[5]);
}


// Function: sub_e069 @ 0xe069
int sub_e069(unsigned int a0,unsigned long a1,int a2,unsigned long a3,unsigned int a4,char a5,int a6)
{
  char *v1;
  int v2; // eax
  int *v3; // rax
  char v4 [264];
  unsigned int v5 [2]; // stack - 0x138
  int v6; // stack - 0x144
  unsigned long v7; // stack - 0x130
  int v8; // stack - 0x128
  unsigned int v9; // stack - 0x124
  
  if (a6 <= -1) {
    v2 = linkat(a0,a1,a2,a3,a4);
    if (v2) { // branch-flip
      v3 = __errno_location();
      v2 = *v3;
    }
    else {
      v2 = 0;
    }
    a6 = v2;
  }
  if ((a5 != '\x01') || (a6 != 0x11))
    v2 = a6;
  else {
    v3 = (char *)sub_df8a(a3,v4);
    v1 = (char *)v3;
    if (v3) { // branch-flip
      v5[0] = a0;
      v7 = a1;
      v8 = a2;
      v9 = a4;
      v2 = sub_1a191(v3,0,v5,sub_e021,6);
      if (v2) { // branch-flip
        v3 = __errno_location();
        v6 = *v3;
      }
      else {
        v2 = renameat(a2,v1,a2,a3);
        if (v2) { // branch-flip
          v3 = __errno_location();
          v2 = *v3;
        }
        else {
          v2 = -1;
        }
        unlinkat(a2,v1,0);
        v6 = v2;
      }
      if (v1 != v4)
        free(v1);
      v2 = v6;
    }
    else {
      v3 = __errno_location();
      v2 = *v3;
    }
  }
  return v2;
}


// Function: sub_e245 @ 0xe245
void sub_e245(unsigned long a0,unsigned long *a1)
{
  symlinkat(*a1,*(unsigned int *)&a1[1],a0);
}


// Function: sub_e27f @ 0xe27f
int sub_e27f(unsigned long a0,int a1,unsigned long a2,char a3,int a4)
{
  char *v1;
  int v2; // eax
  int *v3; // rax
  char v4 [264];
  unsigned long v5; // stack - 0x128
  int v6; // stack - 0x154
  int v7; // stack - 0x134
  int v8; // stack - 0x120
  
  v6 = a4;
  if (a4 <= -1) {
    v2 = symlinkat(a0,a1,a2);
    if (v2) { // branch-flip
      v3 = __errno_location();
      v2 = *v3;
    }
    else {
      v2 = 0;
    }
    v6 = v2;
  }
  if ((a3 != '\x01') || (v6 != 0x11))
    v2 = v6;
  else {
    v3 = (char *)sub_df8a(a2,v4);
    v1 = (char *)v3;
    if (v3) { // branch-flip
      v5 = a0;
      v8 = a1;
      v2 = sub_1a191(v3,0,&v5,sub_e245,6);
      if (v2) { // branch-flip
        v3 = __errno_location();
        v7 = *v3;
      }
      else {
        v2 = renameat(a1,v1,a1,a2);
        if (v2) { // branch-flip
          v3 = __errno_location();
          v7 = *v3;
          unlinkat(a1,v1,0);
        }
        else {
          v7 = -1;
        }
      }
      if (v1 != v4)
        free(v1);
      v2 = v7;
    }
    else {
      v3 = __errno_location();
      v2 = *v3;
    }
  }
  return v2;
}


// Function: sub_e441 @ 0xe441
unsigned int sub_e441(unsigned long a0,unsigned int a1,unsigned long a2)
{
  short v1; // ax
  int v2;
  unsigned long v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  unsigned int v5; // stack - 0x30
  void *v6; // stack - 0x18
  
  v3 = 0;
  v4 = 0;
  v5 = 0xffffffff;
  v6 = (void *)sub_11559(a0);
  if ((((v6) && (0 <= (int)getcon(&v3))) && (v2 = sub_19d4e(v6,&v4), 0 <= v2)) && (v1 = mode_to_security_class(a1), v1))
    v5 = security_compute_create(v3,v4,v1,a2);
  v2 = *__errno_location();
  free(v6);
  freecon(v3);
  freecon(v4);
  *__errno_location() = v2;
  return v5;
}


// Function: sub_e549 @ 0xe549
unsigned int sub_e549(unsigned long a0,char *a1,unsigned int a2)
{
  int v1;
  long v10; // stack - 0x18
  unsigned long v2; // stack - 0x48
  unsigned long v3; // stack - 0x40
  char *v4; // stack - 0x68
  unsigned int v5; // stack - 0x50
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  char *v8; // stack - 0x28
  long v9; // stack - 0x20
  
  v5 = 0xffffffff;
  v2 = 0;
  v3 = 0;
  v6 = 0;
  v7 = 0;
  v8 = NULL;
  v4 = a1;
  if (*a1 != '/') {
    v8 = (char *)sub_111d3(a1,2);
    if (!v8) goto label_e6c3;
    v4 = v8;
  }
  if (0 <= (int)selabel_lookup(a0,&v2,v4,a2)) { // branch-flip
    if (0 <= (int)sub_e441(v4,a2,&v3)) {
      v6 = context_new(v2);
      if (v6) {
        v7 = context_new(v3);
        if (v7) {
          v9 = context_type_get(v6);
          if ((v9) && (v1 = context_type_set(v7,v9), !v1)) {
            v10 = context_str(v7);
            if (v10)
              v5 = setfscreatecon(v10);
          }
        }
      }
    }
  }
  else if (*__errno_location() == 2)
    *__errno_location() = 0x3d;
label_e6c3:
  v1 = *__errno_location();
  context_free(v6);
  context_free(v7);
  freecon(v2);
  freecon(v3);
  free(v8);
  *__errno_location() = v1;
  return v5;
}


// Function: sub_e72c @ 0xe72c
unsigned int sub_e72c(long a0,char *a1) // early-return x2
{
  int v1;
  long v10; // stack - 0xb0
  unsigned int v11; // stack - 0x90
  int v2;
  long v3; // stack - 0xd0
  char v4 [24];
  unsigned long v5; // stack - 0xd8
  unsigned int v6; // stack - 0xe8
  long v7; // stack - 0xc8
  long v8; // stack - 0xc0
  long v9; // stack - 0xb8
  
  v6 = 0xffffffff;
  v5 = 0;
  v3 = 0;
  v7 = 0;
  v8 = 0;
  if (a0) { // branch-flip
    v1 = open(a1,0x20000);
    if ((v1 != -1) || (*__errno_location() == 0x28)) {
      if (v1 != -1) // branch-flip
        v2 = fstat(v1,v4);
      else {
        v2 = lstat(a1,v4);
      }
      if (0 <= v2) {
        if (0 <= (int)selabel_lookup(a0,&v5,a1,v11)) { // branch-flip
          v7 = context_new(v5);
          if (v7) {
            if (v1 != -1) // branch-flip
              v2 = sub_19dc8(v1,&v3);
            else {
              v2 = sub_19d8b(a1,&v3);
            }
            if (0 <= v2) {
              v8 = context_new(v3);
              if (v8) {
                v9 = context_type_get(v7);
                if ((v9) && (v2 = context_type_set(v8,v9), !v2)) {
                  v10 = context_str(v8);
                  if (v10) {
                    if (v1 != -1) // branch-flip
                      v6 = fsetfilecon(v1,v10);
                    else {
                      v6 = lsetfilecon(a1,v10);
                    }
                  }
                }
              }
            }
          }
        }
        else if (*__errno_location() == 2)
          *__errno_location() = 0x3d;
      }
    }
    v2 = *__errno_location();
    if (v1 != -1)
      close(v1);
    context_free(v7);
    context_free(v8);
    freecon(v5);
    freecon(v3);
    *__errno_location() = v2;
  }
  else {
    if ((int)getfscreatecon(&v3) <= -1)
      return 0xffffffff;
    if (!v3) {
      *__errno_location() = 0x3d;
      return 0xffffffff;
    }
    v6 = lsetfilecon(a1,v3);
    v1 = *__errno_location();
    freecon(v3);
    *__errno_location() = v1;
  }
  return v6;
}


// Function: sub_ead7 @ 0xead7
unsigned long sub_ead7(unsigned long a0,char *a1,char a2)
{
  int v1;
  char *v10; // stack - 0x40
  unsigned long v11; // stack - 0x20
  unsigned long v2;
  int v3; // eax
  long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7; // stack - 0x28
  char *v8; // stack - 0x68
  int v9; // stack - 0x48
  
  v10 = NULL;
  v8 = a1;
  if (*a1 != '/') {
    v10 = (char *)sub_111d3(a1,2);
    if (!v10)
      return 0;
    v8 = v10;
  }
  if (a2 != '\x01') { // branch-flip
    v3 = sub_e72c(a0,v8);
    v1 = *__errno_location();
    free(v10);
    *__errno_location() = v1;
    v6 = (unsigned long)(v3 != -1);
  }
  else {
    v7 = v8;
    v11 = 0;
    v4 = sub_1bfb9(&v7,0x10,0);
    v9 = 0;
    while (sub_12fac(v4)) {
      v2 = *(unsigned long *)(v4 + 0x20);
      if ((int)sub_e72c(a0,v2) <= -1)
        v9 = *__errno_location();
    }
    if (*__errno_location())
      v9 = *__errno_location();
    if (sub_12af2(v4))
      v9 = *__errno_location();
    free(v10);
    v6 = CONCAT71((undefined7)((unsigned long)v5 >> 8),v9 == 0);
  }
  return v6;
}


// Function: sub_11559 @ 0x11559
long sub_11559(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_11610(a0);
  if (!v1)
    sub_1bf79(); // no-return
  return v1;
}


// Function: sub_14c84 @ 0x14c84
unsigned long sub_14c84(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_14c9a @ 0x14c9a
unsigned long sub_14c9a(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_14cb0 @ 0x14cb0
unsigned long sub_14cb0(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_14cc6 @ 0x14cc6
unsigned long sub_14cc6(unsigned long *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v3 = v1;
      v4 = 1;
      while (v3 = (long *)v3[1], v3) {
        v4 += 1;
      }
      if (v2 < v4)
        v2 = v4;
    }
  }
  return v2;
}


// Function: sub_14d48 @ 0x14d48
unsigned long sub_14d48(unsigned long *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0[3]) && (v3 == a0[4]))
    return 1;
  return 0;
}


// Function: sub_14de6 @ 0x14de6
void sub_14de6(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_14cb0(a0);
  v2 = sub_14c84(a0);
  v3 = sub_14c9a(a0);
  v4 = sub_14cc6(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",(double)v2,(dat_20808 * (double)v3) / (double)v2,v3);
  fprintf(a1,"max bucket length: %lu\n",v4);
}


// Function: sub_14f2d @ 0x14f2d
long sub_14f2d(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v1)
    abort(); // no-return
  return v1 * 0x10 + *a0;
}


// Function: sub_14f88 @ 0x14f88
long sub_14f88(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_14f2d(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = *(void **)(a0 + 0x38), v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}


// Function: sub_1501f @ 0x1501f
long sub_1501f(unsigned long *a0) // early-return
{
  long *v1; // stack - 0x10
  
  if (!a0[4])
    return 0;
  v1 = (long *)*a0;
  while( true ) {
    if ((long *)a0[1] <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}


// Function: sub_1507f @ 0x1507f
long sub_1507f(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_14f2d(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)(a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}


// Function: sub_1511d @ 0x1511d
unsigned long sub_1511d(unsigned long *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      v4 = v3;
      while (v4) {
        if (a2 <= v2)
          return v2;
        v1 = v2 + 1;
        *(long *)(v2 * 8 + a1) = *v4;
        v4 = (long *)v4[1];
        v2 = v1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_151bd @ 0x151bd
long sub_151bd(unsigned long *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      for (v4 = v3; v4; v4 = (long *)v4[1]) {
        v1 = *v4;
        if ((*a1)(v1,a2) != '\x01')
          return v2;
        v2 += 1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_15255 @ 0x15255
unsigned long sub_15255(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}


// Function: sub_152af @ 0x152af
unsigned long sub_152af(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}


// Function: sub_1531e @ 0x1531e
unsigned long sub_1531e(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_152af(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}


// Function: sub_15369 @ 0x15369
void sub_15369(unsigned int *a0)
{
  *a0 = 0;
  a0[1] = dat_20810;
  a0[2] = dat_20814;
  a0[3] = dat_20818;
  *(char *)&a0[4] = 0;
}


// Function: sub_153bf @ 0x153bf
unsigned long sub_153bf(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_1c5bc(a0,3) % a1;
}


// Function: sub_153fa @ 0x153fa
unsigned long sub_153fa(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}


// Function: sub_15417 @ 0x15417
unsigned long sub_15417(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x20780)
    return 1;
  if ((((dat_2081c < v1[2]) && (v1[2] < dat_20810 - dat_2081c)) && (dat_2081c + dat_20810 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_2081c < v1[1])) && ((v1[1] <= dat_20810 && (*v1 + dat_2081c < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x20780;
  return 0;
}


// Function: sub_15524 @ 0x15524
unsigned long sub_15524(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_20820 <= v2)
      return 0;
    if (dat_20824 <= v2)
      v3 = (long)(v2 - dat_20824) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_1531e(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}


// Function: sub_1563d @ 0x1563d
long * sub_1563d(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_153bf;
  v3 = a3;
  if (!a3)
    v3 = sub_153fa;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x20780;
  v2[5] = v5;
  if (sub_15417(v2) == '\x01') {
    v2[2] = sub_15524(a0,v5);
    if (v2[2]) {
      v1 = v2[2];
      *v2 = (long)calloc(v1,0x10);
      if (*v2) {
        v2[1] = *v2 + v2[2] * 0x10;
        v2[3] = 0;
        v2[4] = 0;
        v2[6] = (long)v4;
        v2[7] = (long)v3;
        v2[8] = a4;
        v2[9] = 0;
        return v2;
      }
    }
  }
  free(v2);
  return NULL;
}


// Function: sub_157b2 @ 0x157b2
void sub_157b2(unsigned long *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0[8])
          (*(void *)a0[8])(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0[9];
        a0[9] = v3;
        v3 = v1;
      }
      if (a0[8])
        (*(void *)a0[8])(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0[3] = 0;
  a0[4] = 0;
}


// Function: sub_158bd @ 0x158bd
void sub_158bd(unsigned long *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0[8]) && (a0[4])) {
    for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0[8])(*v3);
        }
      }
    }
  }
  for (v2 = (unsigned long)*a0; v2 < a0[1]; v2 = (unsigned long)((long)v2 + 0x10)) {
    v3 = *(void **)((long)v2 + 8);
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = (void *)a0[9];
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free((void *)*a0);
  free(a0);
}


// Function: sub_159ec @ 0x159ec
void * sub_159ec(long a0)
{
  void *v1; // stack - 0x10
  
  if (*(long *)(a0 + 0x48)) { // branch-flip
    v1 = *(void **)(a0 + 0x48);
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}


// Function: sub_15a3b @ 0x15a3b
void sub_15a3b(long a0,unsigned long *a1)
{
  *a1 = 0;
  a1[1] = *(unsigned long *)(a0 + 0x48);
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_15a75 @ 0x15a75
long sub_15a75(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long v2;
  long *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_14f2d(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = *(void **)(a0 + 0x38), v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = *(void **)(a0 + 0x38), v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v4 = v6[1];
      v6[1] = *(long *)(v4 + 8);
      sub_15a3b(a0,v4);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (long *)v6[1];
      v4 = v3[1];
      *v6 = *v3;
      v6[1] = v4;
      sub_15a3b(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}


// Function: sub_15c0e @ 0x15c0e
unsigned long sub_15c0e(long a0,unsigned long *a1,char a2)
{
  long v1;
  long *v2;
  long *v3; // rax
  long *v4; // stack - 0x38
  long *v5; // stack - 0x30
  
  v4 = (long *)*a1;
  do {
    if ((long *)a1[1] <= v4)
      return 1;
    if (*v4) {
      v5 = (long *)v4[1];
      while (v5) {
        v1 = *v5;
        v3 = (long *)sub_14f2d(a0,v1);
        v2 = (long *)v5[1];
        if (*v3) { // branch-flip
          v5[1] = v3[1];
          v3[1] = (long)v5;
        }
        else {
          *v3 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_15a3b(a0,v5);
        }
        v5 = v2;
      }
      v1 = *v4;
      v4[1] = 0;
      if (!a2) {
        v2 = (long *)sub_14f2d(a0,v1);
        if (*v2) { // branch-flip
          v3 = (long *)sub_159ec(a0);
          if (!v3)
            return 0;
          *v3 = v1;
          v3[1] = v2[1];
          v2[1] = (long)v3;
        }
        else {
          *v2 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v4 = 0;
        a1[3] = a1[3] + -1;
      }
    }
    v4 = &v4[2];
  } while( true );
}


// Function: sub_15de2 @ 0x15de2
unsigned long sub_15de2(unsigned long *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_15524(a1,a0[5]);
  if (!v3)
    return 0;
  if (v3 == a0[2])
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0[5];
  v7 = a0[6];
  v8 = a0[7];
  v9 = a0[8];
  v10 = a0[9];
  if (sub_15c0e(&v1,a0,0)) {
    free((void *)*a0);
    *a0 = v1;
    a0[1] = v2;
    a0[2] = v3;
    a0[3] = v4;
    a0[9] = v10;
    return 1;
  }
  a0[9] = v10;
  if ((sub_15c0e(a0,&v1,1) == '\x01') && (sub_15c0e(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}


// Function: sub_1600d @ 0x1600d
unsigned long sub_1600d(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_15a75(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_15417(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_20820 <= v4)
      return 0xffffffff;
    v1 = (dat_20824 <= v4) ? (long)(v4 - dat_20824) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_15de2(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_15a75(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_159ec(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}


// Function: sub_1637e @ 0x1637e
unsigned long sub_1637e(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_1600d(a0,a1,&v3);
  if (v1 != -1) {
    if (v1) // branch-flip
      v2 = a1;
    else {
      v2 = v3;
    }
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_163ee @ 0x163ee
long sub_163ee(long a0,unsigned long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_15a75(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_15417(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_20824 <= v3) ? (long)(v3 - dat_20824) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_20824 <= v3) ? (long)(v3 - dat_20824) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_15de2(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)(a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)(a0 + 0x48) = 0;
    }
  }
  return v5;
}


// Function: sub_166f4 @ 0x166f4
void sub_166f4(unsigned long a0,unsigned long a1)
{
  sub_163ee(a0,a1);
}

